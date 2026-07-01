#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign18570_e34676, assign18570_e34676_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18570_e34668: f64 = (locals.var_eg0 * p.p1729);
        let assign18570_e34670: f64 = (assign18570_e34668 * locals.var_tratio_m1);
        let assign18570_e34672: f64 = (assign18570_e34670 / locals.var_vtm);
        let assign18570_e34673: f64 = { let limited_exp_arg = assign18570_e34672; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18570_e34674: f64 = (p.p1630 * assign18570_e34673);
        (assign18570_e34674, (p.p1630 * ({ let limited_exp_arg = assign18570_e34672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18570_e34668 * locals.var_tratio_m1_dn4) * locals.var_vtm) - (assign18570_e34670 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))),)
    } else {
        (locals.var_jtss_t, locals.var_jtss_t_dn4,)
    }
};
        locals.var_jtss_t = assign18570_e34676;
        locals.var_jtss_t_dn4 = assign18570_e34676_d_n4;

        let (assign18580_e34689, assign18580_e34689_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18580_e34681: f64 = (locals.var_eg0 * p.p1730);
        let assign18580_e34683: f64 = (assign18580_e34681 * locals.var_tratio_m1);
        let assign18580_e34685: f64 = (assign18580_e34683 / locals.var_vtm);
        let assign18580_e34686: f64 = { let limited_exp_arg = assign18580_e34685; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18580_e34687: f64 = (p.p1631 * assign18580_e34686);
        (assign18580_e34687, (p.p1631 * ({ let limited_exp_arg = assign18580_e34685; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18580_e34681 * locals.var_tratio_m1_dn4) * locals.var_vtm) - (assign18580_e34683 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))),)
    } else {
        (locals.var_jtsd_t, locals.var_jtsd_t_dn4,)
    }
};
        locals.var_jtsd_t = assign18580_e34689;
        locals.var_jtsd_t_dn4 = assign18580_e34689_d_n4;

        let (assign18590_e34702, assign18590_e34702_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18590_e34694: f64 = (locals.var_eg0 * p.p1731);
        let assign18590_e34696: f64 = (assign18590_e34694 * locals.var_tratio_m1);
        let assign18590_e34698: f64 = (assign18590_e34696 / locals.var_vtm);
        let assign18590_e34699: f64 = { let limited_exp_arg = assign18590_e34698; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18590_e34700: f64 = (p.p1632 * assign18590_e34699);
        (assign18590_e34700, (p.p1632 * ({ let limited_exp_arg = assign18590_e34698; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18590_e34694 * locals.var_tratio_m1_dn4) * locals.var_vtm) - (assign18590_e34696 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))),)
    } else {
        (locals.var_jtssws_t, locals.var_jtssws_t_dn4,)
    }
};
        locals.var_jtssws_t = assign18590_e34702;
        locals.var_jtssws_t_dn4 = assign18590_e34702_d_n4;

        let (assign18600_e34715, assign18600_e34715_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18600_e34707: f64 = (locals.var_eg0 * p.p1732);
        let assign18600_e34709: f64 = (assign18600_e34707 * locals.var_tratio_m1);
        let assign18600_e34711: f64 = (assign18600_e34709 / locals.var_vtm);
        let assign18600_e34712: f64 = { let limited_exp_arg = assign18600_e34711; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18600_e34713: f64 = (p.p1633 * assign18600_e34712);
        (assign18600_e34713, (p.p1633 * ({ let limited_exp_arg = assign18600_e34711; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18600_e34707 * locals.var_tratio_m1_dn4) * locals.var_vtm) - (assign18600_e34709 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))),)
    } else {
        (locals.var_jtsswd_t, locals.var_jtsswd_t_dn4,)
    }
};
        locals.var_jtsswd_t = assign18600_e34715;
        locals.var_jtsswd_t_dn4 = assign18600_e34715_d_n4;

        let (assign18610_e34735, assign18610_e34735_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18610_e34720: f64 = (p.p1636 / locals.var_weff0);
        let assign18610_e34721: f64 = (assign18610_e34720).sqrt();
        let assign18610_e34723: f64 = (assign18610_e34721 + 1.0);
        let assign18610_e34724: f64 = (p.p1634 * assign18610_e34723);
        let assign18610_e34727: f64 = (locals.var_eg0 * p.p1733);
        let assign18610_e34729: f64 = (assign18610_e34727 * locals.var_tratio_m1);
        let assign18610_e34731: f64 = (assign18610_e34729 / locals.var_vtm);
        let assign18610_e34732: f64 = { let limited_exp_arg = assign18610_e34731; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18610_e34733: f64 = (assign18610_e34724 * assign18610_e34732);
        (assign18610_e34733, (assign18610_e34724 * ({ let limited_exp_arg = assign18610_e34731; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18610_e34727 * locals.var_tratio_m1_dn4) * locals.var_vtm) - (assign18610_e34729 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))),)
    } else {
        (locals.var_jtsswgs_t, locals.var_jtsswgs_t_dn4,)
    }
};
        locals.var_jtsswgs_t = assign18610_e34735;
        locals.var_jtsswgs_t_dn4 = assign18610_e34735_d_n4;

        let (assign18620_e34755, assign18620_e34755_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18620_e34740: f64 = (p.p1636 / locals.var_weff0);
        let assign18620_e34741: f64 = (assign18620_e34740).sqrt();
        let assign18620_e34743: f64 = (assign18620_e34741 + 1.0);
        let assign18620_e34744: f64 = (p.p1635 * assign18620_e34743);
        let assign18620_e34747: f64 = (locals.var_eg0 * p.p1734);
        let assign18620_e34749: f64 = (assign18620_e34747 * locals.var_tratio_m1);
        let assign18620_e34751: f64 = (assign18620_e34749 / locals.var_vtm);
        let assign18620_e34752: f64 = { let limited_exp_arg = assign18620_e34751; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18620_e34753: f64 = (assign18620_e34744 * assign18620_e34752);
        (assign18620_e34753, (assign18620_e34744 * ({ let limited_exp_arg = assign18620_e34751; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign18620_e34747 * locals.var_tratio_m1_dn4) * locals.var_vtm) - (assign18620_e34749 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)))),)
    } else {
        (locals.var_jtsswgd_t, locals.var_jtsswgd_t_dn4,)
    }
};
        locals.var_jtsswgd_t = assign18620_e34755;
        locals.var_jtsswgd_t_dn4 = assign18620_e34755_d_n4;

        let (assign18630_e34844, assign18630_e34844_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18630_e34761: f64 = (p.p1735 * locals.var_tratio_m1);
        let assign18630_e34762: f64 = (1.0 + assign18630_e34761);
        let assign18630_e34763: f64 = (p.p1637 * assign18630_e34762);
        let assign18630_e34765: f64 = (assign18630_e34763 - 0.01);
        let assign18630_e34767: f64 = (-10000.0);
        let assign18630_e34769: f64 = (assign18630_e34767 * 0.001);
        let (assign18630_e34840, assign18630_e34840_d_n4,) = {
            if (!(assign18630_e34765 < assign18630_e34769)) {
                let assign18630_e34777: f64 = (p.p1735 * locals.var_tratio_m1);
                let assign18630_e34778: f64 = (1.0 + assign18630_e34777);
                let assign18630_e34779: f64 = (p.p1637 * assign18630_e34778);
                let assign18630_e34781: f64 = (assign18630_e34779 - 0.01);
                let assign18630_e34786: f64 = (p.p1735 * locals.var_tratio_m1);
                let assign18630_e34787: f64 = (1.0 + assign18630_e34786);
                let assign18630_e34788: f64 = (p.p1637 * assign18630_e34787);
                let assign18630_e34790: f64 = (assign18630_e34788 - 0.01);
                let assign18630_e34795: f64 = (p.p1735 * locals.var_tratio_m1);
                let assign18630_e34796: f64 = (1.0 + assign18630_e34795);
                let assign18630_e34797: f64 = (p.p1637 * assign18630_e34796);
                let assign18630_e34799: f64 = (assign18630_e34797 - 0.01);
                let assign18630_e34800: f64 = (assign18630_e34790 * assign18630_e34799);
                let assign18630_e34803: f64 = (4.0 * 0.001);
                let assign18630_e34805: f64 = (assign18630_e34803 * 0.001);
                let assign18630_e34806: f64 = (assign18630_e34800 + assign18630_e34805);
                let assign18630_e34807: f64 = (assign18630_e34806).sqrt();
                let assign18630_e34808: f64 = (assign18630_e34781 + assign18630_e34807);
                let assign18630_e34809: f64 = (0.5 * assign18630_e34808);
                (assign18630_e34809, (0.5 * ((p.p1637 * (p.p1735 * locals.var_tratio_m1_dn4)) + ((((p.p1637 * (p.p1735 * locals.var_tratio_m1_dn4)) * assign18630_e34799) + (assign18630_e34790 * (p.p1637 * (p.p1735 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18630_e34807)))),)
            } else {
                let assign18630_e34814: f64 = (p.p1735 * locals.var_tratio_m1);
                let assign18630_e34815: f64 = (1.0 + assign18630_e34814);
                let assign18630_e34816: f64 = (p.p1637 * assign18630_e34815);
                let assign18630_e34818: f64 = (assign18630_e34816 - 0.01);
                let assign18630_e34820: f64 = (-10000.0);
                let assign18630_e34822: f64 = (assign18630_e34820 * 0.001);
                let (assign18630_e34839, assign18630_e34839_d_n4,) = {
                    if (assign18630_e34818 < assign18630_e34822) {
                        let assign18630_e34825: f64 = (-0.001);
                        let assign18630_e34827: f64 = (assign18630_e34825 * 0.001);
                        let assign18630_e34832: f64 = (p.p1735 * locals.var_tratio_m1);
                        let assign18630_e34833: f64 = (1.0 + assign18630_e34832);
                        let assign18630_e34834: f64 = (p.p1637 * assign18630_e34833);
                        let assign18630_e34836: f64 = (assign18630_e34834 - 0.01);
                        let assign18630_e34837: f64 = (assign18630_e34827 / assign18630_e34836);
                        (assign18630_e34837, (-((assign18630_e34827 * (p.p1637 * (p.p1735 * locals.var_tratio_m1_dn4))) / (assign18630_e34836 * assign18630_e34836))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18630_e34839, assign18630_e34839_d_n4,)
            }
        };
        let assign18630_e34842: f64 = (assign18630_e34840 + 0.01);
        (assign18630_e34842, assign18630_e34840_d_n4,)
    } else {
        (locals.var_njts_t, locals.var_njts_t_dn4,)
    }
};
        locals.var_njts_t = assign18630_e34844;
        locals.var_njts_t_dn4 = assign18630_e34844_d_n4;

        let (assign18640_e34933, assign18640_e34933_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18640_e34850: f64 = (p.p1736 * locals.var_tratio_m1);
        let assign18640_e34851: f64 = (1.0 + assign18640_e34850);
        let assign18640_e34852: f64 = (p.p1638 * assign18640_e34851);
        let assign18640_e34854: f64 = (assign18640_e34852 - 0.01);
        let assign18640_e34856: f64 = (-10000.0);
        let assign18640_e34858: f64 = (assign18640_e34856 * 0.001);
        let (assign18640_e34929, assign18640_e34929_d_n4,) = {
            if (!(assign18640_e34854 < assign18640_e34858)) {
                let assign18640_e34866: f64 = (p.p1736 * locals.var_tratio_m1);
                let assign18640_e34867: f64 = (1.0 + assign18640_e34866);
                let assign18640_e34868: f64 = (p.p1638 * assign18640_e34867);
                let assign18640_e34870: f64 = (assign18640_e34868 - 0.01);
                let assign18640_e34875: f64 = (p.p1736 * locals.var_tratio_m1);
                let assign18640_e34876: f64 = (1.0 + assign18640_e34875);
                let assign18640_e34877: f64 = (p.p1638 * assign18640_e34876);
                let assign18640_e34879: f64 = (assign18640_e34877 - 0.01);
                let assign18640_e34884: f64 = (p.p1736 * locals.var_tratio_m1);
                let assign18640_e34885: f64 = (1.0 + assign18640_e34884);
                let assign18640_e34886: f64 = (p.p1638 * assign18640_e34885);
                let assign18640_e34888: f64 = (assign18640_e34886 - 0.01);
                let assign18640_e34889: f64 = (assign18640_e34879 * assign18640_e34888);
                let assign18640_e34892: f64 = (4.0 * 0.001);
                let assign18640_e34894: f64 = (assign18640_e34892 * 0.001);
                let assign18640_e34895: f64 = (assign18640_e34889 + assign18640_e34894);
                let assign18640_e34896: f64 = (assign18640_e34895).sqrt();
                let assign18640_e34897: f64 = (assign18640_e34870 + assign18640_e34896);
                let assign18640_e34898: f64 = (0.5 * assign18640_e34897);
                (assign18640_e34898, (0.5 * ((p.p1638 * (p.p1736 * locals.var_tratio_m1_dn4)) + ((((p.p1638 * (p.p1736 * locals.var_tratio_m1_dn4)) * assign18640_e34888) + (assign18640_e34879 * (p.p1638 * (p.p1736 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18640_e34896)))),)
            } else {
                let assign18640_e34903: f64 = (p.p1736 * locals.var_tratio_m1);
                let assign18640_e34904: f64 = (1.0 + assign18640_e34903);
                let assign18640_e34905: f64 = (p.p1638 * assign18640_e34904);
                let assign18640_e34907: f64 = (assign18640_e34905 - 0.01);
                let assign18640_e34909: f64 = (-10000.0);
                let assign18640_e34911: f64 = (assign18640_e34909 * 0.001);
                let (assign18640_e34928, assign18640_e34928_d_n4,) = {
                    if (assign18640_e34907 < assign18640_e34911) {
                        let assign18640_e34914: f64 = (-0.001);
                        let assign18640_e34916: f64 = (assign18640_e34914 * 0.001);
                        let assign18640_e34921: f64 = (p.p1736 * locals.var_tratio_m1);
                        let assign18640_e34922: f64 = (1.0 + assign18640_e34921);
                        let assign18640_e34923: f64 = (p.p1638 * assign18640_e34922);
                        let assign18640_e34925: f64 = (assign18640_e34923 - 0.01);
                        let assign18640_e34926: f64 = (assign18640_e34916 / assign18640_e34925);
                        (assign18640_e34926, (-((assign18640_e34916 * (p.p1638 * (p.p1736 * locals.var_tratio_m1_dn4))) / (assign18640_e34925 * assign18640_e34925))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18640_e34928, assign18640_e34928_d_n4,)
            }
        };
        let assign18640_e34931: f64 = (assign18640_e34929 + 0.01);
        (assign18640_e34931, assign18640_e34929_d_n4,)
    } else {
        (locals.var_njtsd_t, locals.var_njtsd_t_dn4,)
    }
};
        locals.var_njtsd_t = assign18640_e34933;
        locals.var_njtsd_t_dn4 = assign18640_e34933_d_n4;

        let (assign18650_e35022, assign18650_e35022_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18650_e34939: f64 = (p.p1737 * locals.var_tratio_m1);
        let assign18650_e34940: f64 = (1.0 + assign18650_e34939);
        let assign18650_e34941: f64 = (p.p1639 * assign18650_e34940);
        let assign18650_e34943: f64 = (assign18650_e34941 - 0.01);
        let assign18650_e34945: f64 = (-10000.0);
        let assign18650_e34947: f64 = (assign18650_e34945 * 0.001);
        let (assign18650_e35018, assign18650_e35018_d_n4,) = {
            if (!(assign18650_e34943 < assign18650_e34947)) {
                let assign18650_e34955: f64 = (p.p1737 * locals.var_tratio_m1);
                let assign18650_e34956: f64 = (1.0 + assign18650_e34955);
                let assign18650_e34957: f64 = (p.p1639 * assign18650_e34956);
                let assign18650_e34959: f64 = (assign18650_e34957 - 0.01);
                let assign18650_e34964: f64 = (p.p1737 * locals.var_tratio_m1);
                let assign18650_e34965: f64 = (1.0 + assign18650_e34964);
                let assign18650_e34966: f64 = (p.p1639 * assign18650_e34965);
                let assign18650_e34968: f64 = (assign18650_e34966 - 0.01);
                let assign18650_e34973: f64 = (p.p1737 * locals.var_tratio_m1);
                let assign18650_e34974: f64 = (1.0 + assign18650_e34973);
                let assign18650_e34975: f64 = (p.p1639 * assign18650_e34974);
                let assign18650_e34977: f64 = (assign18650_e34975 - 0.01);
                let assign18650_e34978: f64 = (assign18650_e34968 * assign18650_e34977);
                let assign18650_e34981: f64 = (4.0 * 0.001);
                let assign18650_e34983: f64 = (assign18650_e34981 * 0.001);
                let assign18650_e34984: f64 = (assign18650_e34978 + assign18650_e34983);
                let assign18650_e34985: f64 = (assign18650_e34984).sqrt();
                let assign18650_e34986: f64 = (assign18650_e34959 + assign18650_e34985);
                let assign18650_e34987: f64 = (0.5 * assign18650_e34986);
                (assign18650_e34987, (0.5 * ((p.p1639 * (p.p1737 * locals.var_tratio_m1_dn4)) + ((((p.p1639 * (p.p1737 * locals.var_tratio_m1_dn4)) * assign18650_e34977) + (assign18650_e34968 * (p.p1639 * (p.p1737 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18650_e34985)))),)
            } else {
                let assign18650_e34992: f64 = (p.p1737 * locals.var_tratio_m1);
                let assign18650_e34993: f64 = (1.0 + assign18650_e34992);
                let assign18650_e34994: f64 = (p.p1639 * assign18650_e34993);
                let assign18650_e34996: f64 = (assign18650_e34994 - 0.01);
                let assign18650_e34998: f64 = (-10000.0);
                let assign18650_e35000: f64 = (assign18650_e34998 * 0.001);
                let (assign18650_e35017, assign18650_e35017_d_n4,) = {
                    if (assign18650_e34996 < assign18650_e35000) {
                        let assign18650_e35003: f64 = (-0.001);
                        let assign18650_e35005: f64 = (assign18650_e35003 * 0.001);
                        let assign18650_e35010: f64 = (p.p1737 * locals.var_tratio_m1);
                        let assign18650_e35011: f64 = (1.0 + assign18650_e35010);
                        let assign18650_e35012: f64 = (p.p1639 * assign18650_e35011);
                        let assign18650_e35014: f64 = (assign18650_e35012 - 0.01);
                        let assign18650_e35015: f64 = (assign18650_e35005 / assign18650_e35014);
                        (assign18650_e35015, (-((assign18650_e35005 * (p.p1639 * (p.p1737 * locals.var_tratio_m1_dn4))) / (assign18650_e35014 * assign18650_e35014))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18650_e35017, assign18650_e35017_d_n4,)
            }
        };
        let assign18650_e35020: f64 = (assign18650_e35018 + 0.01);
        (assign18650_e35020, assign18650_e35018_d_n4,)
    } else {
        (locals.var_njtssw_t, locals.var_njtssw_t_dn4,)
    }
};
        locals.var_njtssw_t = assign18650_e35022;
        locals.var_njtssw_t_dn4 = assign18650_e35022_d_n4;

        let (assign18660_e35111, assign18660_e35111_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18660_e35028: f64 = (p.p1738 * locals.var_tratio_m1);
        let assign18660_e35029: f64 = (1.0 + assign18660_e35028);
        let assign18660_e35030: f64 = (p.p1640 * assign18660_e35029);
        let assign18660_e35032: f64 = (assign18660_e35030 - 0.01);
        let assign18660_e35034: f64 = (-10000.0);
        let assign18660_e35036: f64 = (assign18660_e35034 * 0.001);
        let (assign18660_e35107, assign18660_e35107_d_n4,) = {
            if (!(assign18660_e35032 < assign18660_e35036)) {
                let assign18660_e35044: f64 = (p.p1738 * locals.var_tratio_m1);
                let assign18660_e35045: f64 = (1.0 + assign18660_e35044);
                let assign18660_e35046: f64 = (p.p1640 * assign18660_e35045);
                let assign18660_e35048: f64 = (assign18660_e35046 - 0.01);
                let assign18660_e35053: f64 = (p.p1738 * locals.var_tratio_m1);
                let assign18660_e35054: f64 = (1.0 + assign18660_e35053);
                let assign18660_e35055: f64 = (p.p1640 * assign18660_e35054);
                let assign18660_e35057: f64 = (assign18660_e35055 - 0.01);
                let assign18660_e35062: f64 = (p.p1738 * locals.var_tratio_m1);
                let assign18660_e35063: f64 = (1.0 + assign18660_e35062);
                let assign18660_e35064: f64 = (p.p1640 * assign18660_e35063);
                let assign18660_e35066: f64 = (assign18660_e35064 - 0.01);
                let assign18660_e35067: f64 = (assign18660_e35057 * assign18660_e35066);
                let assign18660_e35070: f64 = (4.0 * 0.001);
                let assign18660_e35072: f64 = (assign18660_e35070 * 0.001);
                let assign18660_e35073: f64 = (assign18660_e35067 + assign18660_e35072);
                let assign18660_e35074: f64 = (assign18660_e35073).sqrt();
                let assign18660_e35075: f64 = (assign18660_e35048 + assign18660_e35074);
                let assign18660_e35076: f64 = (0.5 * assign18660_e35075);
                (assign18660_e35076, (0.5 * ((p.p1640 * (p.p1738 * locals.var_tratio_m1_dn4)) + ((((p.p1640 * (p.p1738 * locals.var_tratio_m1_dn4)) * assign18660_e35066) + (assign18660_e35057 * (p.p1640 * (p.p1738 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18660_e35074)))),)
            } else {
                let assign18660_e35081: f64 = (p.p1738 * locals.var_tratio_m1);
                let assign18660_e35082: f64 = (1.0 + assign18660_e35081);
                let assign18660_e35083: f64 = (p.p1640 * assign18660_e35082);
                let assign18660_e35085: f64 = (assign18660_e35083 - 0.01);
                let assign18660_e35087: f64 = (-10000.0);
                let assign18660_e35089: f64 = (assign18660_e35087 * 0.001);
                let (assign18660_e35106, assign18660_e35106_d_n4,) = {
                    if (assign18660_e35085 < assign18660_e35089) {
                        let assign18660_e35092: f64 = (-0.001);
                        let assign18660_e35094: f64 = (assign18660_e35092 * 0.001);
                        let assign18660_e35099: f64 = (p.p1738 * locals.var_tratio_m1);
                        let assign18660_e35100: f64 = (1.0 + assign18660_e35099);
                        let assign18660_e35101: f64 = (p.p1640 * assign18660_e35100);
                        let assign18660_e35103: f64 = (assign18660_e35101 - 0.01);
                        let assign18660_e35104: f64 = (assign18660_e35094 / assign18660_e35103);
                        (assign18660_e35104, (-((assign18660_e35094 * (p.p1640 * (p.p1738 * locals.var_tratio_m1_dn4))) / (assign18660_e35103 * assign18660_e35103))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18660_e35106, assign18660_e35106_d_n4,)
            }
        };
        let assign18660_e35109: f64 = (assign18660_e35107 + 0.01);
        (assign18660_e35109, assign18660_e35107_d_n4,)
    } else {
        (locals.var_njtsswd_t, locals.var_njtsswd_t_dn4,)
    }
};
        locals.var_njtsswd_t = assign18660_e35111;
        locals.var_njtsswd_t_dn4 = assign18660_e35111_d_n4;

        let (assign18670_e35200, assign18670_e35200_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18670_e35117: f64 = (p.p1739 * locals.var_tratio_m1);
        let assign18670_e35118: f64 = (1.0 + assign18670_e35117);
        let assign18670_e35119: f64 = (p.p1641 * assign18670_e35118);
        let assign18670_e35121: f64 = (assign18670_e35119 - 0.01);
        let assign18670_e35123: f64 = (-10000.0);
        let assign18670_e35125: f64 = (assign18670_e35123 * 0.001);
        let (assign18670_e35196, assign18670_e35196_d_n4,) = {
            if (!(assign18670_e35121 < assign18670_e35125)) {
                let assign18670_e35133: f64 = (p.p1739 * locals.var_tratio_m1);
                let assign18670_e35134: f64 = (1.0 + assign18670_e35133);
                let assign18670_e35135: f64 = (p.p1641 * assign18670_e35134);
                let assign18670_e35137: f64 = (assign18670_e35135 - 0.01);
                let assign18670_e35142: f64 = (p.p1739 * locals.var_tratio_m1);
                let assign18670_e35143: f64 = (1.0 + assign18670_e35142);
                let assign18670_e35144: f64 = (p.p1641 * assign18670_e35143);
                let assign18670_e35146: f64 = (assign18670_e35144 - 0.01);
                let assign18670_e35151: f64 = (p.p1739 * locals.var_tratio_m1);
                let assign18670_e35152: f64 = (1.0 + assign18670_e35151);
                let assign18670_e35153: f64 = (p.p1641 * assign18670_e35152);
                let assign18670_e35155: f64 = (assign18670_e35153 - 0.01);
                let assign18670_e35156: f64 = (assign18670_e35146 * assign18670_e35155);
                let assign18670_e35159: f64 = (4.0 * 0.001);
                let assign18670_e35161: f64 = (assign18670_e35159 * 0.001);
                let assign18670_e35162: f64 = (assign18670_e35156 + assign18670_e35161);
                let assign18670_e35163: f64 = (assign18670_e35162).sqrt();
                let assign18670_e35164: f64 = (assign18670_e35137 + assign18670_e35163);
                let assign18670_e35165: f64 = (0.5 * assign18670_e35164);
                (assign18670_e35165, (0.5 * ((p.p1641 * (p.p1739 * locals.var_tratio_m1_dn4)) + ((((p.p1641 * (p.p1739 * locals.var_tratio_m1_dn4)) * assign18670_e35155) + (assign18670_e35146 * (p.p1641 * (p.p1739 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18670_e35163)))),)
            } else {
                let assign18670_e35170: f64 = (p.p1739 * locals.var_tratio_m1);
                let assign18670_e35171: f64 = (1.0 + assign18670_e35170);
                let assign18670_e35172: f64 = (p.p1641 * assign18670_e35171);
                let assign18670_e35174: f64 = (assign18670_e35172 - 0.01);
                let assign18670_e35176: f64 = (-10000.0);
                let assign18670_e35178: f64 = (assign18670_e35176 * 0.001);
                let (assign18670_e35195, assign18670_e35195_d_n4,) = {
                    if (assign18670_e35174 < assign18670_e35178) {
                        let assign18670_e35181: f64 = (-0.001);
                        let assign18670_e35183: f64 = (assign18670_e35181 * 0.001);
                        let assign18670_e35188: f64 = (p.p1739 * locals.var_tratio_m1);
                        let assign18670_e35189: f64 = (1.0 + assign18670_e35188);
                        let assign18670_e35190: f64 = (p.p1641 * assign18670_e35189);
                        let assign18670_e35192: f64 = (assign18670_e35190 - 0.01);
                        let assign18670_e35193: f64 = (assign18670_e35183 / assign18670_e35192);
                        (assign18670_e35193, (-((assign18670_e35183 * (p.p1641 * (p.p1739 * locals.var_tratio_m1_dn4))) / (assign18670_e35192 * assign18670_e35192))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18670_e35195, assign18670_e35195_d_n4,)
            }
        };
        let assign18670_e35198: f64 = (assign18670_e35196 + 0.01);
        (assign18670_e35198, assign18670_e35196_d_n4,)
    } else {
        (locals.var_njtsswg_t, locals.var_njtsswg_t_dn4,)
    }
};
        locals.var_njtsswg_t = assign18670_e35200;
        locals.var_njtsswg_t_dn4 = assign18670_e35200_d_n4;

        let (assign18680_e35289, assign18680_e35289_d_n4,) = {
    if (locals.var_guard336 != 0.0) {
        let assign18680_e35206: f64 = (p.p1740 * locals.var_tratio_m1);
        let assign18680_e35207: f64 = (1.0 + assign18680_e35206);
        let assign18680_e35208: f64 = (p.p1642 * assign18680_e35207);
        let assign18680_e35210: f64 = (assign18680_e35208 - 0.01);
        let assign18680_e35212: f64 = (-10000.0);
        let assign18680_e35214: f64 = (assign18680_e35212 * 0.001);
        let (assign18680_e35285, assign18680_e35285_d_n4,) = {
            if (!(assign18680_e35210 < assign18680_e35214)) {
                let assign18680_e35222: f64 = (p.p1740 * locals.var_tratio_m1);
                let assign18680_e35223: f64 = (1.0 + assign18680_e35222);
                let assign18680_e35224: f64 = (p.p1642 * assign18680_e35223);
                let assign18680_e35226: f64 = (assign18680_e35224 - 0.01);
                let assign18680_e35231: f64 = (p.p1740 * locals.var_tratio_m1);
                let assign18680_e35232: f64 = (1.0 + assign18680_e35231);
                let assign18680_e35233: f64 = (p.p1642 * assign18680_e35232);
                let assign18680_e35235: f64 = (assign18680_e35233 - 0.01);
                let assign18680_e35240: f64 = (p.p1740 * locals.var_tratio_m1);
                let assign18680_e35241: f64 = (1.0 + assign18680_e35240);
                let assign18680_e35242: f64 = (p.p1642 * assign18680_e35241);
                let assign18680_e35244: f64 = (assign18680_e35242 - 0.01);
                let assign18680_e35245: f64 = (assign18680_e35235 * assign18680_e35244);
                let assign18680_e35248: f64 = (4.0 * 0.001);
                let assign18680_e35250: f64 = (assign18680_e35248 * 0.001);
                let assign18680_e35251: f64 = (assign18680_e35245 + assign18680_e35250);
                let assign18680_e35252: f64 = (assign18680_e35251).sqrt();
                let assign18680_e35253: f64 = (assign18680_e35226 + assign18680_e35252);
                let assign18680_e35254: f64 = (0.5 * assign18680_e35253);
                (assign18680_e35254, (0.5 * ((p.p1642 * (p.p1740 * locals.var_tratio_m1_dn4)) + ((((p.p1642 * (p.p1740 * locals.var_tratio_m1_dn4)) * assign18680_e35244) + (assign18680_e35235 * (p.p1642 * (p.p1740 * locals.var_tratio_m1_dn4)))) / (2.0 * assign18680_e35252)))),)
            } else {
                let assign18680_e35259: f64 = (p.p1740 * locals.var_tratio_m1);
                let assign18680_e35260: f64 = (1.0 + assign18680_e35259);
                let assign18680_e35261: f64 = (p.p1642 * assign18680_e35260);
                let assign18680_e35263: f64 = (assign18680_e35261 - 0.01);
                let assign18680_e35265: f64 = (-10000.0);
                let assign18680_e35267: f64 = (assign18680_e35265 * 0.001);
                let (assign18680_e35284, assign18680_e35284_d_n4,) = {
                    if (assign18680_e35263 < assign18680_e35267) {
                        let assign18680_e35270: f64 = (-0.001);
                        let assign18680_e35272: f64 = (assign18680_e35270 * 0.001);
                        let assign18680_e35277: f64 = (p.p1740 * locals.var_tratio_m1);
                        let assign18680_e35278: f64 = (1.0 + assign18680_e35277);
                        let assign18680_e35279: f64 = (p.p1642 * assign18680_e35278);
                        let assign18680_e35281: f64 = (assign18680_e35279 - 0.01);
                        let assign18680_e35282: f64 = (assign18680_e35272 / assign18680_e35281);
                        (assign18680_e35282, (-((assign18680_e35272 * (p.p1642 * (p.p1740 * locals.var_tratio_m1_dn4))) / (assign18680_e35281 * assign18680_e35281))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign18680_e35284, assign18680_e35284_d_n4,)
            }
        };
        let assign18680_e35287: f64 = (assign18680_e35285 + 0.01);
        (assign18680_e35287, assign18680_e35285_d_n4,)
    } else {
        (locals.var_njtsswgd_t, locals.var_njtsswgd_t_dn4,)
    }
};
        locals.var_njtsswgd_t = assign18680_e35289;
        locals.var_njtsswgd_t_dn4 = assign18680_e35289_d_n4;

        let assign18690_e35292: f64 = if (!param_given[1106]) { 1.0 } else { 0.0 };
        locals.var_guard343 = assign18690_e35292;

        let assign18700_e35295: f64 = if p.p145 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard344 = assign18700_e35295;

        let assign18710_e35298: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard345 = assign18710_e35298;

    }

    pub(super) fn stamp_transient_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18720_e35690, assign18720_e35690_d_n0, assign18720_e35690_d_n2, assign18720_e35690_d_n3, assign18720_e35690_d_n4, assign18720_e35690_d_n5, assign18720_e35690_d_n6, assign18720_e35690_d_n7, assign18720_e35690_d_n8, assign18720_e35690_d_n9, assign18720_e35690_d_n10, assign18720_e35690_d_n11, assign18720_e35690_d_n13, assign18720_e35690_d_n14,) = {
    if (((locals.var_guard343 != 0.0) && (locals.var_guard344 != 0.0)) && (locals.var_guard345 != 0.0)) {
        let assign18720_e35307: f64 = (0.5 * locals.var_eg);
        let assign18720_e35311: f64 = (p.p145 / locals.var_ni);
        let (assign18720_e35328, assign18720_e35328_d_n0, assign18720_e35328_d_n2, assign18720_e35328_d_n3, assign18720_e35328_d_n4, assign18720_e35328_d_n5, assign18720_e35328_d_n6, assign18720_e35328_d_n7, assign18720_e35328_d_n8, assign18720_e35328_d_n9, assign18720_e35328_d_n10, assign18720_e35328_d_n11, assign18720_e35328_d_n13, assign18720_e35328_d_n14,) = {
            if (!(assign18720_e35311 > 1e-38)) {
                let assign18720_e35316: f64 = (-87.498233534);
                (assign18720_e35316, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18720_e35319: f64 = (p.p145 / locals.var_ni);
                let (assign18720_e35327, assign18720_e35327_d_n0, assign18720_e35327_d_n2, assign18720_e35327_d_n3, assign18720_e35327_d_n4, assign18720_e35327_d_n5, assign18720_e35327_d_n6, assign18720_e35327_d_n7, assign18720_e35327_d_n8, assign18720_e35327_d_n9, assign18720_e35327_d_n10, assign18720_e35327_d_n11, assign18720_e35327_d_n13, assign18720_e35327_d_n14,) = {
                    if (assign18720_e35319 > 1e-38) {
                        let assign18720_e35324: f64 = (p.p145 / locals.var_ni);
                        let assign18720_e35325: f64 = (assign18720_e35324).ln();
                        (assign18720_e35325, ((-((p.p145 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35324), ((-((p.p145 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35324),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18720_e35327, assign18720_e35327_d_n0, assign18720_e35327_d_n2, assign18720_e35327_d_n3, assign18720_e35327_d_n4, assign18720_e35327_d_n5, assign18720_e35327_d_n6, assign18720_e35327_d_n7, assign18720_e35327_d_n8, assign18720_e35327_d_n9, assign18720_e35327_d_n10, assign18720_e35327_d_n11, assign18720_e35327_d_n13, assign18720_e35327_d_n14,)
            }
        };
        let assign18720_e35329: f64 = (locals.var_vtm * assign18720_e35328);
        let assign18720_e35330: f64 = (assign18720_e35307 - assign18720_e35329);
        let assign18720_e35332: f64 = (-10000.0);
        let assign18720_e35334: f64 = (assign18720_e35332 * 0.0001);
        let (assign18720_e35490, assign18720_e35490_d_n0, assign18720_e35490_d_n2, assign18720_e35490_d_n3, assign18720_e35490_d_n4, assign18720_e35490_d_n5, assign18720_e35490_d_n6, assign18720_e35490_d_n7, assign18720_e35490_d_n8, assign18720_e35490_d_n9, assign18720_e35490_d_n10, assign18720_e35490_d_n11, assign18720_e35490_d_n13, assign18720_e35490_d_n14,) = {
            if (!(assign18720_e35330 < assign18720_e35334)) {
                let assign18720_e35340: f64 = (0.5 * locals.var_eg);
                let assign18720_e35344: f64 = (p.p145 / locals.var_ni);
                let (assign18720_e35361, assign18720_e35361_d_n0, assign18720_e35361_d_n2, assign18720_e35361_d_n3, assign18720_e35361_d_n4, assign18720_e35361_d_n5, assign18720_e35361_d_n6, assign18720_e35361_d_n7, assign18720_e35361_d_n8, assign18720_e35361_d_n9, assign18720_e35361_d_n10, assign18720_e35361_d_n11, assign18720_e35361_d_n13, assign18720_e35361_d_n14,) = {
                    if (!(assign18720_e35344 > 1e-38)) {
                        let assign18720_e35349: f64 = (-87.498233534);
                        (assign18720_e35349, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35352: f64 = (p.p145 / locals.var_ni);
                        let (assign18720_e35360, assign18720_e35360_d_n0, assign18720_e35360_d_n2, assign18720_e35360_d_n3, assign18720_e35360_d_n4, assign18720_e35360_d_n5, assign18720_e35360_d_n6, assign18720_e35360_d_n7, assign18720_e35360_d_n8, assign18720_e35360_d_n9, assign18720_e35360_d_n10, assign18720_e35360_d_n11, assign18720_e35360_d_n13, assign18720_e35360_d_n14,) = {
                            if (assign18720_e35352 > 1e-38) {
                                let assign18720_e35357: f64 = (p.p145 / locals.var_ni);
                                let assign18720_e35358: f64 = (assign18720_e35357).ln();
                                (assign18720_e35358, ((-((p.p145 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35357), ((-((p.p145 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35357),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35360, assign18720_e35360_d_n0, assign18720_e35360_d_n2, assign18720_e35360_d_n3, assign18720_e35360_d_n4, assign18720_e35360_d_n5, assign18720_e35360_d_n6, assign18720_e35360_d_n7, assign18720_e35360_d_n8, assign18720_e35360_d_n9, assign18720_e35360_d_n10, assign18720_e35360_d_n11, assign18720_e35360_d_n13, assign18720_e35360_d_n14,)
                    }
                };
                let assign18720_e35362: f64 = (locals.var_vtm * assign18720_e35361);
                let assign18720_e35363: f64 = (assign18720_e35340 - assign18720_e35362);
                let assign18720_e35366: f64 = (0.5 * locals.var_eg);
                let assign18720_e35370: f64 = (p.p145 / locals.var_ni);
                let (assign18720_e35387, assign18720_e35387_d_n0, assign18720_e35387_d_n2, assign18720_e35387_d_n3, assign18720_e35387_d_n4, assign18720_e35387_d_n5, assign18720_e35387_d_n6, assign18720_e35387_d_n7, assign18720_e35387_d_n8, assign18720_e35387_d_n9, assign18720_e35387_d_n10, assign18720_e35387_d_n11, assign18720_e35387_d_n13, assign18720_e35387_d_n14,) = {
                    if (!(assign18720_e35370 > 1e-38)) {
                        let assign18720_e35375: f64 = (-87.498233534);
                        (assign18720_e35375, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35378: f64 = (p.p145 / locals.var_ni);
                        let (assign18720_e35386, assign18720_e35386_d_n0, assign18720_e35386_d_n2, assign18720_e35386_d_n3, assign18720_e35386_d_n4, assign18720_e35386_d_n5, assign18720_e35386_d_n6, assign18720_e35386_d_n7, assign18720_e35386_d_n8, assign18720_e35386_d_n9, assign18720_e35386_d_n10, assign18720_e35386_d_n11, assign18720_e35386_d_n13, assign18720_e35386_d_n14,) = {
                            if (assign18720_e35378 > 1e-38) {
                                let assign18720_e35383: f64 = (p.p145 / locals.var_ni);
                                let assign18720_e35384: f64 = (assign18720_e35383).ln();
                                (assign18720_e35384, ((-((p.p145 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35383), ((-((p.p145 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35383),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35386, assign18720_e35386_d_n0, assign18720_e35386_d_n2, assign18720_e35386_d_n3, assign18720_e35386_d_n4, assign18720_e35386_d_n5, assign18720_e35386_d_n6, assign18720_e35386_d_n7, assign18720_e35386_d_n8, assign18720_e35386_d_n9, assign18720_e35386_d_n10, assign18720_e35386_d_n11, assign18720_e35386_d_n13, assign18720_e35386_d_n14,)
                    }
                };
                let assign18720_e35388: f64 = (locals.var_vtm * assign18720_e35387);
                let assign18720_e35389: f64 = (assign18720_e35366 - assign18720_e35388);
                let assign18720_e35392: f64 = (0.5 * locals.var_eg);
                let assign18720_e35396: f64 = (p.p145 / locals.var_ni);
                let (assign18720_e35413, assign18720_e35413_d_n0, assign18720_e35413_d_n2, assign18720_e35413_d_n3, assign18720_e35413_d_n4, assign18720_e35413_d_n5, assign18720_e35413_d_n6, assign18720_e35413_d_n7, assign18720_e35413_d_n8, assign18720_e35413_d_n9, assign18720_e35413_d_n10, assign18720_e35413_d_n11, assign18720_e35413_d_n13, assign18720_e35413_d_n14,) = {
                    if (!(assign18720_e35396 > 1e-38)) {
                        let assign18720_e35401: f64 = (-87.498233534);
                        (assign18720_e35401, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35404: f64 = (p.p145 / locals.var_ni);
                        let (assign18720_e35412, assign18720_e35412_d_n0, assign18720_e35412_d_n2, assign18720_e35412_d_n3, assign18720_e35412_d_n4, assign18720_e35412_d_n5, assign18720_e35412_d_n6, assign18720_e35412_d_n7, assign18720_e35412_d_n8, assign18720_e35412_d_n9, assign18720_e35412_d_n10, assign18720_e35412_d_n11, assign18720_e35412_d_n13, assign18720_e35412_d_n14,) = {
                            if (assign18720_e35404 > 1e-38) {
                                let assign18720_e35409: f64 = (p.p145 / locals.var_ni);
                                let assign18720_e35410: f64 = (assign18720_e35409).ln();
                                (assign18720_e35410, ((-((p.p145 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35409), ((-((p.p145 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35409),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35412, assign18720_e35412_d_n0, assign18720_e35412_d_n2, assign18720_e35412_d_n3, assign18720_e35412_d_n4, assign18720_e35412_d_n5, assign18720_e35412_d_n6, assign18720_e35412_d_n7, assign18720_e35412_d_n8, assign18720_e35412_d_n9, assign18720_e35412_d_n10, assign18720_e35412_d_n11, assign18720_e35412_d_n13, assign18720_e35412_d_n14,)
                    }
                };
                let assign18720_e35414: f64 = (locals.var_vtm * assign18720_e35413);
                let assign18720_e35415: f64 = (assign18720_e35392 - assign18720_e35414);
                let assign18720_e35416: f64 = (assign18720_e35389 * assign18720_e35415);
                let assign18720_e35419: f64 = (4.0 * 0.0001);
                let assign18720_e35421: f64 = (assign18720_e35419 * 0.0001);
                let assign18720_e35422: f64 = (assign18720_e35416 + assign18720_e35421);
                let assign18720_e35423: f64 = (assign18720_e35422).sqrt();
                let assign18720_e35424: f64 = (assign18720_e35363 + assign18720_e35423);
                let assign18720_e35425: f64 = (0.5 * assign18720_e35424);
                (assign18720_e35425, (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n0)) + ((((-(locals.var_vtm * assign18720_e35387_d_n0)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n0)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n2)) + ((((-(locals.var_vtm * assign18720_e35387_d_n2)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n2)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n3)) + ((((-(locals.var_vtm * assign18720_e35387_d_n3)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n3)))) / (2.0 * assign18720_e35423)))), (0.5 * (((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18720_e35361) + (locals.var_vtm * assign18720_e35361_d_n4))) + (((((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18720_e35387) + (locals.var_vtm * assign18720_e35387_d_n4))) * assign18720_e35415) + (assign18720_e35389 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18720_e35413) + (locals.var_vtm * assign18720_e35413_d_n4))))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n5)) + ((((-(locals.var_vtm * assign18720_e35387_d_n5)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n5)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n6)) + ((((-(locals.var_vtm * assign18720_e35387_d_n6)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n6)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n7)) + ((((-(locals.var_vtm * assign18720_e35387_d_n7)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n7)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n8)) + ((((-(locals.var_vtm * assign18720_e35387_d_n8)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n8)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n9)) + ((((-(locals.var_vtm * assign18720_e35387_d_n9)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n9)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n10)) + ((((-(locals.var_vtm * assign18720_e35387_d_n10)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n10)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n11)) + ((((-(locals.var_vtm * assign18720_e35387_d_n11)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n11)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n13)) + ((((-(locals.var_vtm * assign18720_e35387_d_n13)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n13)))) / (2.0 * assign18720_e35423)))), (0.5 * ((-(locals.var_vtm * assign18720_e35361_d_n14)) + ((((-(locals.var_vtm * assign18720_e35387_d_n14)) * assign18720_e35415) + (assign18720_e35389 * (-(locals.var_vtm * assign18720_e35413_d_n14)))) / (2.0 * assign18720_e35423)))),)
            } else {
                let assign18720_e35428: f64 = (0.5 * locals.var_eg);
                let assign18720_e35432: f64 = (p.p145 / locals.var_ni);
                let (assign18720_e35449, assign18720_e35449_d_n0, assign18720_e35449_d_n2, assign18720_e35449_d_n3, assign18720_e35449_d_n4, assign18720_e35449_d_n5, assign18720_e35449_d_n6, assign18720_e35449_d_n7, assign18720_e35449_d_n8, assign18720_e35449_d_n9, assign18720_e35449_d_n10, assign18720_e35449_d_n11, assign18720_e35449_d_n13, assign18720_e35449_d_n14,) = {
                    if (!(assign18720_e35432 > 1e-38)) {
                        let assign18720_e35437: f64 = (-87.498233534);
                        (assign18720_e35437, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35440: f64 = (p.p145 / locals.var_ni);
                        let (assign18720_e35448, assign18720_e35448_d_n0, assign18720_e35448_d_n2, assign18720_e35448_d_n3, assign18720_e35448_d_n4, assign18720_e35448_d_n5, assign18720_e35448_d_n6, assign18720_e35448_d_n7, assign18720_e35448_d_n8, assign18720_e35448_d_n9, assign18720_e35448_d_n10, assign18720_e35448_d_n11, assign18720_e35448_d_n13, assign18720_e35448_d_n14,) = {
                            if (assign18720_e35440 > 1e-38) {
                                let assign18720_e35445: f64 = (p.p145 / locals.var_ni);
                                let assign18720_e35446: f64 = (assign18720_e35445).ln();
                                (assign18720_e35446, ((-((p.p145 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35445), ((-((p.p145 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35445),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35448, assign18720_e35448_d_n0, assign18720_e35448_d_n2, assign18720_e35448_d_n3, assign18720_e35448_d_n4, assign18720_e35448_d_n5, assign18720_e35448_d_n6, assign18720_e35448_d_n7, assign18720_e35448_d_n8, assign18720_e35448_d_n9, assign18720_e35448_d_n10, assign18720_e35448_d_n11, assign18720_e35448_d_n13, assign18720_e35448_d_n14,)
                    }
                };
                let assign18720_e35450: f64 = (locals.var_vtm * assign18720_e35449);
                let assign18720_e35451: f64 = (assign18720_e35428 - assign18720_e35450);
                let assign18720_e35453: f64 = (-10000.0);
                let assign18720_e35455: f64 = (assign18720_e35453 * 0.0001);
                let (assign18720_e35489, assign18720_e35489_d_n0, assign18720_e35489_d_n2, assign18720_e35489_d_n3, assign18720_e35489_d_n4, assign18720_e35489_d_n5, assign18720_e35489_d_n6, assign18720_e35489_d_n7, assign18720_e35489_d_n8, assign18720_e35489_d_n9, assign18720_e35489_d_n10, assign18720_e35489_d_n11, assign18720_e35489_d_n13, assign18720_e35489_d_n14,) = {
                    if (assign18720_e35451 < assign18720_e35455) {
                        let assign18720_e35458: f64 = (-0.0001);
                        let assign18720_e35460: f64 = (assign18720_e35458 * 0.0001);
                        let assign18720_e35463: f64 = (0.5 * locals.var_eg);
                        let assign18720_e35467: f64 = (p.p145 / locals.var_ni);
                        let (assign18720_e35484, assign18720_e35484_d_n0, assign18720_e35484_d_n2, assign18720_e35484_d_n3, assign18720_e35484_d_n4, assign18720_e35484_d_n5, assign18720_e35484_d_n6, assign18720_e35484_d_n7, assign18720_e35484_d_n8, assign18720_e35484_d_n9, assign18720_e35484_d_n10, assign18720_e35484_d_n11, assign18720_e35484_d_n13, assign18720_e35484_d_n14,) = {
                            if (!(assign18720_e35467 > 1e-38)) {
                                let assign18720_e35472: f64 = (-87.498233534);
                                (assign18720_e35472, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            } else {
                                let assign18720_e35475: f64 = (p.p145 / locals.var_ni);
                                let (assign18720_e35483, assign18720_e35483_d_n0, assign18720_e35483_d_n2, assign18720_e35483_d_n3, assign18720_e35483_d_n4, assign18720_e35483_d_n5, assign18720_e35483_d_n6, assign18720_e35483_d_n7, assign18720_e35483_d_n8, assign18720_e35483_d_n9, assign18720_e35483_d_n10, assign18720_e35483_d_n11, assign18720_e35483_d_n13, assign18720_e35483_d_n14,) = {
                                    if (assign18720_e35475 > 1e-38) {
                                        let assign18720_e35480: f64 = (p.p145 / locals.var_ni);
                                        let assign18720_e35481: f64 = (assign18720_e35480).ln();
                                        (assign18720_e35481, ((-((p.p145 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35480), ((-((p.p145 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35480),)
                                    } else {
                                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                                    }
                                };
                                (assign18720_e35483, assign18720_e35483_d_n0, assign18720_e35483_d_n2, assign18720_e35483_d_n3, assign18720_e35483_d_n4, assign18720_e35483_d_n5, assign18720_e35483_d_n6, assign18720_e35483_d_n7, assign18720_e35483_d_n8, assign18720_e35483_d_n9, assign18720_e35483_d_n10, assign18720_e35483_d_n11, assign18720_e35483_d_n13, assign18720_e35483_d_n14,)
                            }
                        };
                        let assign18720_e35485: f64 = (locals.var_vtm * assign18720_e35484);
                        let assign18720_e35486: f64 = (assign18720_e35463 - assign18720_e35485);
                        let assign18720_e35487: f64 = (assign18720_e35460 / assign18720_e35486);
                        (assign18720_e35487, (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n0))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n2))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n3))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18720_e35484) + (locals.var_vtm * assign18720_e35484_d_n4)))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n5))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n6))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n7))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n8))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n9))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n10))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n11))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n13))) / (assign18720_e35486 * assign18720_e35486))), (-((assign18720_e35460 * (-(locals.var_vtm * assign18720_e35484_d_n14))) / (assign18720_e35486 * assign18720_e35486))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18720_e35489, assign18720_e35489_d_n0, assign18720_e35489_d_n2, assign18720_e35489_d_n3, assign18720_e35489_d_n4, assign18720_e35489_d_n5, assign18720_e35489_d_n6, assign18720_e35489_d_n7, assign18720_e35489_d_n8, assign18720_e35489_d_n9, assign18720_e35489_d_n10, assign18720_e35489_d_n11, assign18720_e35489_d_n13, assign18720_e35489_d_n14,)
            }
        };
        let assign18720_e35493: f64 = (0.5 * locals.var_eg);
        let assign18720_e35497: f64 = (0.5 * locals.var_eg);
        let assign18720_e35500: f64 = (0.5 * locals.var_eg);
        let assign18720_e35504: f64 = (p.p97 / locals.var_ni);
        let (assign18720_e35521, assign18720_e35521_d_n0, assign18720_e35521_d_n2, assign18720_e35521_d_n3, assign18720_e35521_d_n4, assign18720_e35521_d_n5, assign18720_e35521_d_n6, assign18720_e35521_d_n7, assign18720_e35521_d_n8, assign18720_e35521_d_n9, assign18720_e35521_d_n10, assign18720_e35521_d_n11, assign18720_e35521_d_n13, assign18720_e35521_d_n14,) = {
            if (!(assign18720_e35504 > 1e-38)) {
                let assign18720_e35509: f64 = (-87.498233534);
                (assign18720_e35509, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18720_e35512: f64 = (p.p97 / locals.var_ni);
                let (assign18720_e35520, assign18720_e35520_d_n0, assign18720_e35520_d_n2, assign18720_e35520_d_n3, assign18720_e35520_d_n4, assign18720_e35520_d_n5, assign18720_e35520_d_n6, assign18720_e35520_d_n7, assign18720_e35520_d_n8, assign18720_e35520_d_n9, assign18720_e35520_d_n10, assign18720_e35520_d_n11, assign18720_e35520_d_n13, assign18720_e35520_d_n14,) = {
                    if (assign18720_e35512 > 1e-38) {
                        let assign18720_e35517: f64 = (p.p97 / locals.var_ni);
                        let assign18720_e35518: f64 = (assign18720_e35517).ln();
                        (assign18720_e35518, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35517), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35517),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18720_e35520, assign18720_e35520_d_n0, assign18720_e35520_d_n2, assign18720_e35520_d_n3, assign18720_e35520_d_n4, assign18720_e35520_d_n5, assign18720_e35520_d_n6, assign18720_e35520_d_n7, assign18720_e35520_d_n8, assign18720_e35520_d_n9, assign18720_e35520_d_n10, assign18720_e35520_d_n11, assign18720_e35520_d_n13, assign18720_e35520_d_n14,)
            }
        };
        let assign18720_e35522: f64 = (locals.var_vtm * assign18720_e35521);
        let assign18720_e35523: f64 = (assign18720_e35500 - assign18720_e35522);
        let assign18720_e35525: f64 = (-10000.0);
        let assign18720_e35527: f64 = (assign18720_e35525 * 0.0001);
        let (assign18720_e35683, assign18720_e35683_d_n0, assign18720_e35683_d_n2, assign18720_e35683_d_n3, assign18720_e35683_d_n4, assign18720_e35683_d_n5, assign18720_e35683_d_n6, assign18720_e35683_d_n7, assign18720_e35683_d_n8, assign18720_e35683_d_n9, assign18720_e35683_d_n10, assign18720_e35683_d_n11, assign18720_e35683_d_n13, assign18720_e35683_d_n14,) = {
            if (!(assign18720_e35523 < assign18720_e35527)) {
                let assign18720_e35533: f64 = (0.5 * locals.var_eg);
                let assign18720_e35537: f64 = (p.p97 / locals.var_ni);
                let (assign18720_e35554, assign18720_e35554_d_n0, assign18720_e35554_d_n2, assign18720_e35554_d_n3, assign18720_e35554_d_n4, assign18720_e35554_d_n5, assign18720_e35554_d_n6, assign18720_e35554_d_n7, assign18720_e35554_d_n8, assign18720_e35554_d_n9, assign18720_e35554_d_n10, assign18720_e35554_d_n11, assign18720_e35554_d_n13, assign18720_e35554_d_n14,) = {
                    if (!(assign18720_e35537 > 1e-38)) {
                        let assign18720_e35542: f64 = (-87.498233534);
                        (assign18720_e35542, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35545: f64 = (p.p97 / locals.var_ni);
                        let (assign18720_e35553, assign18720_e35553_d_n0, assign18720_e35553_d_n2, assign18720_e35553_d_n3, assign18720_e35553_d_n4, assign18720_e35553_d_n5, assign18720_e35553_d_n6, assign18720_e35553_d_n7, assign18720_e35553_d_n8, assign18720_e35553_d_n9, assign18720_e35553_d_n10, assign18720_e35553_d_n11, assign18720_e35553_d_n13, assign18720_e35553_d_n14,) = {
                            if (assign18720_e35545 > 1e-38) {
                                let assign18720_e35550: f64 = (p.p97 / locals.var_ni);
                                let assign18720_e35551: f64 = (assign18720_e35550).ln();
                                (assign18720_e35551, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35550), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35550),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35553, assign18720_e35553_d_n0, assign18720_e35553_d_n2, assign18720_e35553_d_n3, assign18720_e35553_d_n4, assign18720_e35553_d_n5, assign18720_e35553_d_n6, assign18720_e35553_d_n7, assign18720_e35553_d_n8, assign18720_e35553_d_n9, assign18720_e35553_d_n10, assign18720_e35553_d_n11, assign18720_e35553_d_n13, assign18720_e35553_d_n14,)
                    }
                };
                let assign18720_e35555: f64 = (locals.var_vtm * assign18720_e35554);
                let assign18720_e35556: f64 = (assign18720_e35533 - assign18720_e35555);
                let assign18720_e35559: f64 = (0.5 * locals.var_eg);
                let assign18720_e35563: f64 = (p.p97 / locals.var_ni);
                let (assign18720_e35580, assign18720_e35580_d_n0, assign18720_e35580_d_n2, assign18720_e35580_d_n3, assign18720_e35580_d_n4, assign18720_e35580_d_n5, assign18720_e35580_d_n6, assign18720_e35580_d_n7, assign18720_e35580_d_n8, assign18720_e35580_d_n9, assign18720_e35580_d_n10, assign18720_e35580_d_n11, assign18720_e35580_d_n13, assign18720_e35580_d_n14,) = {
                    if (!(assign18720_e35563 > 1e-38)) {
                        let assign18720_e35568: f64 = (-87.498233534);
                        (assign18720_e35568, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35571: f64 = (p.p97 / locals.var_ni);
                        let (assign18720_e35579, assign18720_e35579_d_n0, assign18720_e35579_d_n2, assign18720_e35579_d_n3, assign18720_e35579_d_n4, assign18720_e35579_d_n5, assign18720_e35579_d_n6, assign18720_e35579_d_n7, assign18720_e35579_d_n8, assign18720_e35579_d_n9, assign18720_e35579_d_n10, assign18720_e35579_d_n11, assign18720_e35579_d_n13, assign18720_e35579_d_n14,) = {
                            if (assign18720_e35571 > 1e-38) {
                                let assign18720_e35576: f64 = (p.p97 / locals.var_ni);
                                let assign18720_e35577: f64 = (assign18720_e35576).ln();
                                (assign18720_e35577, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35576), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35576),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35579, assign18720_e35579_d_n0, assign18720_e35579_d_n2, assign18720_e35579_d_n3, assign18720_e35579_d_n4, assign18720_e35579_d_n5, assign18720_e35579_d_n6, assign18720_e35579_d_n7, assign18720_e35579_d_n8, assign18720_e35579_d_n9, assign18720_e35579_d_n10, assign18720_e35579_d_n11, assign18720_e35579_d_n13, assign18720_e35579_d_n14,)
                    }
                };
                let assign18720_e35581: f64 = (locals.var_vtm * assign18720_e35580);
                let assign18720_e35582: f64 = (assign18720_e35559 - assign18720_e35581);
                let assign18720_e35585: f64 = (0.5 * locals.var_eg);
                let assign18720_e35589: f64 = (p.p97 / locals.var_ni);
                let (assign18720_e35606, assign18720_e35606_d_n0, assign18720_e35606_d_n2, assign18720_e35606_d_n3, assign18720_e35606_d_n4, assign18720_e35606_d_n5, assign18720_e35606_d_n6, assign18720_e35606_d_n7, assign18720_e35606_d_n8, assign18720_e35606_d_n9, assign18720_e35606_d_n10, assign18720_e35606_d_n11, assign18720_e35606_d_n13, assign18720_e35606_d_n14,) = {
                    if (!(assign18720_e35589 > 1e-38)) {
                        let assign18720_e35594: f64 = (-87.498233534);
                        (assign18720_e35594, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35597: f64 = (p.p97 / locals.var_ni);
                        let (assign18720_e35605, assign18720_e35605_d_n0, assign18720_e35605_d_n2, assign18720_e35605_d_n3, assign18720_e35605_d_n4, assign18720_e35605_d_n5, assign18720_e35605_d_n6, assign18720_e35605_d_n7, assign18720_e35605_d_n8, assign18720_e35605_d_n9, assign18720_e35605_d_n10, assign18720_e35605_d_n11, assign18720_e35605_d_n13, assign18720_e35605_d_n14,) = {
                            if (assign18720_e35597 > 1e-38) {
                                let assign18720_e35602: f64 = (p.p97 / locals.var_ni);
                                let assign18720_e35603: f64 = (assign18720_e35602).ln();
                                (assign18720_e35603, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35602), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35602),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35605, assign18720_e35605_d_n0, assign18720_e35605_d_n2, assign18720_e35605_d_n3, assign18720_e35605_d_n4, assign18720_e35605_d_n5, assign18720_e35605_d_n6, assign18720_e35605_d_n7, assign18720_e35605_d_n8, assign18720_e35605_d_n9, assign18720_e35605_d_n10, assign18720_e35605_d_n11, assign18720_e35605_d_n13, assign18720_e35605_d_n14,)
                    }
                };
                let assign18720_e35607: f64 = (locals.var_vtm * assign18720_e35606);
                let assign18720_e35608: f64 = (assign18720_e35585 - assign18720_e35607);
                let assign18720_e35609: f64 = (assign18720_e35582 * assign18720_e35608);
                let assign18720_e35612: f64 = (4.0 * 0.0001);
                let assign18720_e35614: f64 = (assign18720_e35612 * 0.0001);
                let assign18720_e35615: f64 = (assign18720_e35609 + assign18720_e35614);
                let assign18720_e35616: f64 = (assign18720_e35615).sqrt();
                let assign18720_e35617: f64 = (assign18720_e35556 + assign18720_e35616);
                let assign18720_e35618: f64 = (0.5 * assign18720_e35617);
                (assign18720_e35618, (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n0)) + ((((-(locals.var_vtm * assign18720_e35580_d_n0)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n0)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n2)) + ((((-(locals.var_vtm * assign18720_e35580_d_n2)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n2)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n3)) + ((((-(locals.var_vtm * assign18720_e35580_d_n3)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n3)))) / (2.0 * assign18720_e35616)))), (0.5 * (((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18720_e35554) + (locals.var_vtm * assign18720_e35554_d_n4))) + (((((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18720_e35580) + (locals.var_vtm * assign18720_e35580_d_n4))) * assign18720_e35608) + (assign18720_e35582 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18720_e35606) + (locals.var_vtm * assign18720_e35606_d_n4))))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n5)) + ((((-(locals.var_vtm * assign18720_e35580_d_n5)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n5)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n6)) + ((((-(locals.var_vtm * assign18720_e35580_d_n6)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n6)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n7)) + ((((-(locals.var_vtm * assign18720_e35580_d_n7)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n7)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n8)) + ((((-(locals.var_vtm * assign18720_e35580_d_n8)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n8)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n9)) + ((((-(locals.var_vtm * assign18720_e35580_d_n9)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n9)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n10)) + ((((-(locals.var_vtm * assign18720_e35580_d_n10)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n10)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n11)) + ((((-(locals.var_vtm * assign18720_e35580_d_n11)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n11)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n13)) + ((((-(locals.var_vtm * assign18720_e35580_d_n13)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n13)))) / (2.0 * assign18720_e35616)))), (0.5 * ((-(locals.var_vtm * assign18720_e35554_d_n14)) + ((((-(locals.var_vtm * assign18720_e35580_d_n14)) * assign18720_e35608) + (assign18720_e35582 * (-(locals.var_vtm * assign18720_e35606_d_n14)))) / (2.0 * assign18720_e35616)))),)
            } else {
                let assign18720_e35621: f64 = (0.5 * locals.var_eg);
                let assign18720_e35625: f64 = (p.p97 / locals.var_ni);
                let (assign18720_e35642, assign18720_e35642_d_n0, assign18720_e35642_d_n2, assign18720_e35642_d_n3, assign18720_e35642_d_n4, assign18720_e35642_d_n5, assign18720_e35642_d_n6, assign18720_e35642_d_n7, assign18720_e35642_d_n8, assign18720_e35642_d_n9, assign18720_e35642_d_n10, assign18720_e35642_d_n11, assign18720_e35642_d_n13, assign18720_e35642_d_n14,) = {
                    if (!(assign18720_e35625 > 1e-38)) {
                        let assign18720_e35630: f64 = (-87.498233534);
                        (assign18720_e35630, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18720_e35633: f64 = (p.p97 / locals.var_ni);
                        let (assign18720_e35641, assign18720_e35641_d_n0, assign18720_e35641_d_n2, assign18720_e35641_d_n3, assign18720_e35641_d_n4, assign18720_e35641_d_n5, assign18720_e35641_d_n6, assign18720_e35641_d_n7, assign18720_e35641_d_n8, assign18720_e35641_d_n9, assign18720_e35641_d_n10, assign18720_e35641_d_n11, assign18720_e35641_d_n13, assign18720_e35641_d_n14,) = {
                            if (assign18720_e35633 > 1e-38) {
                                let assign18720_e35638: f64 = (p.p97 / locals.var_ni);
                                let assign18720_e35639: f64 = (assign18720_e35638).ln();
                                (assign18720_e35639, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35638), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35638),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18720_e35641, assign18720_e35641_d_n0, assign18720_e35641_d_n2, assign18720_e35641_d_n3, assign18720_e35641_d_n4, assign18720_e35641_d_n5, assign18720_e35641_d_n6, assign18720_e35641_d_n7, assign18720_e35641_d_n8, assign18720_e35641_d_n9, assign18720_e35641_d_n10, assign18720_e35641_d_n11, assign18720_e35641_d_n13, assign18720_e35641_d_n14,)
                    }
                };
                let assign18720_e35643: f64 = (locals.var_vtm * assign18720_e35642);
                let assign18720_e35644: f64 = (assign18720_e35621 - assign18720_e35643);
                let assign18720_e35646: f64 = (-10000.0);
                let assign18720_e35648: f64 = (assign18720_e35646 * 0.0001);
                let (assign18720_e35682, assign18720_e35682_d_n0, assign18720_e35682_d_n2, assign18720_e35682_d_n3, assign18720_e35682_d_n4, assign18720_e35682_d_n5, assign18720_e35682_d_n6, assign18720_e35682_d_n7, assign18720_e35682_d_n8, assign18720_e35682_d_n9, assign18720_e35682_d_n10, assign18720_e35682_d_n11, assign18720_e35682_d_n13, assign18720_e35682_d_n14,) = {
                    if (assign18720_e35644 < assign18720_e35648) {
                        let assign18720_e35651: f64 = (-0.0001);
                        let assign18720_e35653: f64 = (assign18720_e35651 * 0.0001);
                        let assign18720_e35656: f64 = (0.5 * locals.var_eg);
                        let assign18720_e35660: f64 = (p.p97 / locals.var_ni);
                        let (assign18720_e35677, assign18720_e35677_d_n0, assign18720_e35677_d_n2, assign18720_e35677_d_n3, assign18720_e35677_d_n4, assign18720_e35677_d_n5, assign18720_e35677_d_n6, assign18720_e35677_d_n7, assign18720_e35677_d_n8, assign18720_e35677_d_n9, assign18720_e35677_d_n10, assign18720_e35677_d_n11, assign18720_e35677_d_n13, assign18720_e35677_d_n14,) = {
                            if (!(assign18720_e35660 > 1e-38)) {
                                let assign18720_e35665: f64 = (-87.498233534);
                                (assign18720_e35665, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            } else {
                                let assign18720_e35668: f64 = (p.p97 / locals.var_ni);
                                let (assign18720_e35676, assign18720_e35676_d_n0, assign18720_e35676_d_n2, assign18720_e35676_d_n3, assign18720_e35676_d_n4, assign18720_e35676_d_n5, assign18720_e35676_d_n6, assign18720_e35676_d_n7, assign18720_e35676_d_n8, assign18720_e35676_d_n9, assign18720_e35676_d_n10, assign18720_e35676_d_n11, assign18720_e35676_d_n13, assign18720_e35676_d_n14,) = {
                                    if (assign18720_e35668 > 1e-38) {
                                        let assign18720_e35673: f64 = (p.p97 / locals.var_ni);
                                        let assign18720_e35674: f64 = (assign18720_e35673).ln();
                                        (assign18720_e35674, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18720_e35673), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18720_e35673),)
                                    } else {
                                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                                    }
                                };
                                (assign18720_e35676, assign18720_e35676_d_n0, assign18720_e35676_d_n2, assign18720_e35676_d_n3, assign18720_e35676_d_n4, assign18720_e35676_d_n5, assign18720_e35676_d_n6, assign18720_e35676_d_n7, assign18720_e35676_d_n8, assign18720_e35676_d_n9, assign18720_e35676_d_n10, assign18720_e35676_d_n11, assign18720_e35676_d_n13, assign18720_e35676_d_n14,)
                            }
                        };
                        let assign18720_e35678: f64 = (locals.var_vtm * assign18720_e35677);
                        let assign18720_e35679: f64 = (assign18720_e35656 - assign18720_e35678);
                        let assign18720_e35680: f64 = (assign18720_e35653 / assign18720_e35679);
                        (assign18720_e35680, (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n0))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n2))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n3))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18720_e35677) + (locals.var_vtm * assign18720_e35677_d_n4)))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n5))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n6))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n7))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n8))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n9))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n10))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n11))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n13))) / (assign18720_e35679 * assign18720_e35679))), (-((assign18720_e35653 * (-(locals.var_vtm * assign18720_e35677_d_n14))) / (assign18720_e35679 * assign18720_e35679))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18720_e35682, assign18720_e35682_d_n0, assign18720_e35682_d_n2, assign18720_e35682_d_n3, assign18720_e35682_d_n4, assign18720_e35682_d_n5, assign18720_e35682_d_n6, assign18720_e35682_d_n7, assign18720_e35682_d_n8, assign18720_e35682_d_n9, assign18720_e35682_d_n10, assign18720_e35682_d_n11, assign18720_e35682_d_n13, assign18720_e35682_d_n14,)
            }
        };
        let assign18720_e35684: f64 = (assign18720_e35497 - assign18720_e35683);
        let assign18720_e35685: f64 = (locals.var_devsign * assign18720_e35684);
        let assign18720_e35686: f64 = (assign18720_e35493 - assign18720_e35685);
        let assign18720_e35687: f64 = (assign18720_e35490 - assign18720_e35686);
        let assign18720_e35688: f64 = (locals.var_devsign * assign18720_e35687);
        (assign18720_e35688, (locals.var_devsign * (assign18720_e35490_d_n0 - (-(locals.var_devsign * (-assign18720_e35683_d_n0))))), (locals.var_devsign * (assign18720_e35490_d_n2 - (-(locals.var_devsign * (-assign18720_e35683_d_n2))))), (locals.var_devsign * (assign18720_e35490_d_n3 - (-(locals.var_devsign * (-assign18720_e35683_d_n3))))), (locals.var_devsign * (assign18720_e35490_d_n4 - ((0.5 * locals.var_eg_dn4) - (locals.var_devsign * ((0.5 * locals.var_eg_dn4) - assign18720_e35683_d_n4))))), (locals.var_devsign * (assign18720_e35490_d_n5 - (-(locals.var_devsign * (-assign18720_e35683_d_n5))))), (locals.var_devsign * (assign18720_e35490_d_n6 - (-(locals.var_devsign * (-assign18720_e35683_d_n6))))), (locals.var_devsign * (assign18720_e35490_d_n7 - (-(locals.var_devsign * (-assign18720_e35683_d_n7))))), (locals.var_devsign * (assign18720_e35490_d_n8 - (-(locals.var_devsign * (-assign18720_e35683_d_n8))))), (locals.var_devsign * (assign18720_e35490_d_n9 - (-(locals.var_devsign * (-assign18720_e35683_d_n9))))), (locals.var_devsign * (assign18720_e35490_d_n10 - (-(locals.var_devsign * (-assign18720_e35683_d_n10))))), (locals.var_devsign * (assign18720_e35490_d_n11 - (-(locals.var_devsign * (-assign18720_e35683_d_n11))))), (locals.var_devsign * (assign18720_e35490_d_n13 - (-(locals.var_devsign * (-assign18720_e35683_d_n13))))), (locals.var_devsign * (assign18720_e35490_d_n14 - (-(locals.var_devsign * (-assign18720_e35683_d_n14))))),)
    } else {
        (locals.var_vfbsd_v, locals.var_vfbsd_v_dn0, locals.var_vfbsd_v_dn2, locals.var_vfbsd_v_dn3, locals.var_vfbsd_v_dn4, locals.var_vfbsd_v_dn5, locals.var_vfbsd_v_dn6, locals.var_vfbsd_v_dn7, locals.var_vfbsd_v_dn8, locals.var_vfbsd_v_dn9, locals.var_vfbsd_v_dn10, locals.var_vfbsd_v_dn11, locals.var_vfbsd_v_dn13, locals.var_vfbsd_v_dn14,)
    }
};
        locals.var_vfbsd_v = assign18720_e35690;
        locals.var_vfbsd_v_dn0 = assign18720_e35690_d_n0;
        locals.var_vfbsd_v_dn2 = assign18720_e35690_d_n2;
        locals.var_vfbsd_v_dn3 = assign18720_e35690_d_n3;
        locals.var_vfbsd_v_dn4 = assign18720_e35690_d_n4;
        locals.var_vfbsd_v_dn5 = assign18720_e35690_d_n5;
        locals.var_vfbsd_v_dn6 = assign18720_e35690_d_n6;
        locals.var_vfbsd_v_dn7 = assign18720_e35690_d_n7;
        locals.var_vfbsd_v_dn8 = assign18720_e35690_d_n8;
        locals.var_vfbsd_v_dn9 = assign18720_e35690_d_n9;
        locals.var_vfbsd_v_dn10 = assign18720_e35690_d_n10;
        locals.var_vfbsd_v_dn11 = assign18720_e35690_d_n11;
        locals.var_vfbsd_v_dn13 = assign18720_e35690_d_n13;
        locals.var_vfbsd_v_dn14 = assign18720_e35690_d_n14;

    }

    pub(super) fn stamp_transient_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18730_e36035, assign18730_e36035_d_n0, assign18730_e36035_d_n2, assign18730_e36035_d_n3, assign18730_e36035_d_n4, assign18730_e36035_d_n5, assign18730_e36035_d_n6, assign18730_e36035_d_n7, assign18730_e36035_d_n8, assign18730_e36035_d_n9, assign18730_e36035_d_n10, assign18730_e36035_d_n11, assign18730_e36035_d_n13, assign18730_e36035_d_n14,) = {
    if (((locals.var_guard343 != 0.0) && (locals.var_guard344 != 0.0)) && (locals.var_guard345 == 0.0)) {
        let assign18730_e35700: f64 = (0.5 * locals.var_eg);
        let (assign18730_e35715,) = {
            if (!(p.p145 > 1e-38)) {
                let assign18730_e35707: f64 = (-87.498233534);
                (assign18730_e35707,)
            } else {
                let (assign18730_e35714,) = {
                    if (p.p145 > 1e-38) {
                        let assign18730_e35712: f64 = (p.p145).ln();
                        (assign18730_e35712,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18730_e35714,)
            }
        };
        let assign18730_e35717: f64 = (assign18730_e35715 - locals.var_niln);
        let assign18730_e35718: f64 = (locals.var_vtm * assign18730_e35717);
        let assign18730_e35719: f64 = (assign18730_e35700 - assign18730_e35718);
        let assign18730_e35721: f64 = (-10000.0);
        let assign18730_e35723: f64 = (assign18730_e35721 * 0.0001);
        let (assign18730_e35859, assign18730_e35859_d_n0, assign18730_e35859_d_n2, assign18730_e35859_d_n3, assign18730_e35859_d_n4, assign18730_e35859_d_n5, assign18730_e35859_d_n6, assign18730_e35859_d_n7, assign18730_e35859_d_n8, assign18730_e35859_d_n9, assign18730_e35859_d_n10, assign18730_e35859_d_n11, assign18730_e35859_d_n13, assign18730_e35859_d_n14,) = {
            if (!(assign18730_e35719 < assign18730_e35723)) {
                let assign18730_e35729: f64 = (0.5 * locals.var_eg);
                let (assign18730_e35744,) = {
                    if (!(p.p145 > 1e-38)) {
                        let assign18730_e35736: f64 = (-87.498233534);
                        (assign18730_e35736,)
                    } else {
                        let (assign18730_e35743,) = {
                            if (p.p145 > 1e-38) {
                                let assign18730_e35741: f64 = (p.p145).ln();
                                (assign18730_e35741,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35743,)
                    }
                };
                let assign18730_e35746: f64 = (assign18730_e35744 - locals.var_niln);
                let assign18730_e35747: f64 = (locals.var_vtm * assign18730_e35746);
                let assign18730_e35748: f64 = (assign18730_e35729 - assign18730_e35747);
                let assign18730_e35751: f64 = (0.5 * locals.var_eg);
                let (assign18730_e35766,) = {
                    if (!(p.p145 > 1e-38)) {
                        let assign18730_e35758: f64 = (-87.498233534);
                        (assign18730_e35758,)
                    } else {
                        let (assign18730_e35765,) = {
                            if (p.p145 > 1e-38) {
                                let assign18730_e35763: f64 = (p.p145).ln();
                                (assign18730_e35763,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35765,)
                    }
                };
                let assign18730_e35768: f64 = (assign18730_e35766 - locals.var_niln);
                let assign18730_e35769: f64 = (locals.var_vtm * assign18730_e35768);
                let assign18730_e35770: f64 = (assign18730_e35751 - assign18730_e35769);
                let assign18730_e35773: f64 = (0.5 * locals.var_eg);
                let (assign18730_e35788,) = {
                    if (!(p.p145 > 1e-38)) {
                        let assign18730_e35780: f64 = (-87.498233534);
                        (assign18730_e35780,)
                    } else {
                        let (assign18730_e35787,) = {
                            if (p.p145 > 1e-38) {
                                let assign18730_e35785: f64 = (p.p145).ln();
                                (assign18730_e35785,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35787,)
                    }
                };
                let assign18730_e35790: f64 = (assign18730_e35788 - locals.var_niln);
                let assign18730_e35791: f64 = (locals.var_vtm * assign18730_e35790);
                let assign18730_e35792: f64 = (assign18730_e35773 - assign18730_e35791);
                let assign18730_e35793: f64 = (assign18730_e35770 * assign18730_e35792);
                let assign18730_e35796: f64 = (4.0 * 0.0001);
                let assign18730_e35798: f64 = (assign18730_e35796 * 0.0001);
                let assign18730_e35799: f64 = (assign18730_e35793 + assign18730_e35798);
                let assign18730_e35800: f64 = (assign18730_e35799).sqrt();
                let assign18730_e35801: f64 = (assign18730_e35748 + assign18730_e35800);
                let assign18730_e35802: f64 = (0.5 * assign18730_e35801);
                (assign18730_e35802, (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn0))) + ((((-(locals.var_vtm * (-locals.var_niln_dn0))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn0))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn2))) + ((((-(locals.var_vtm * (-locals.var_niln_dn2))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn2))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn3))) + ((((-(locals.var_vtm * (-locals.var_niln_dn3))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn3))))) / (2.0 * assign18730_e35800)))), (0.5 * (((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18730_e35746) + (locals.var_vtm * (-locals.var_niln_dn4)))) + (((((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18730_e35768) + (locals.var_vtm * (-locals.var_niln_dn4)))) * assign18730_e35792) + (assign18730_e35770 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18730_e35790) + (locals.var_vtm * (-locals.var_niln_dn4)))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn5))) + ((((-(locals.var_vtm * (-locals.var_niln_dn5))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn5))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn6))) + ((((-(locals.var_vtm * (-locals.var_niln_dn6))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn6))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn7))) + ((((-(locals.var_vtm * (-locals.var_niln_dn7))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn7))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn8))) + ((((-(locals.var_vtm * (-locals.var_niln_dn8))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn8))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn9))) + ((((-(locals.var_vtm * (-locals.var_niln_dn9))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn9))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn10))) + ((((-(locals.var_vtm * (-locals.var_niln_dn10))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn10))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn11))) + ((((-(locals.var_vtm * (-locals.var_niln_dn11))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn11))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn13))) + ((((-(locals.var_vtm * (-locals.var_niln_dn13))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn13))))) / (2.0 * assign18730_e35800)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn14))) + ((((-(locals.var_vtm * (-locals.var_niln_dn14))) * assign18730_e35792) + (assign18730_e35770 * (-(locals.var_vtm * (-locals.var_niln_dn14))))) / (2.0 * assign18730_e35800)))),)
            } else {
                let assign18730_e35805: f64 = (0.5 * locals.var_eg);
                let (assign18730_e35820,) = {
                    if (!(p.p145 > 1e-38)) {
                        let assign18730_e35812: f64 = (-87.498233534);
                        (assign18730_e35812,)
                    } else {
                        let (assign18730_e35819,) = {
                            if (p.p145 > 1e-38) {
                                let assign18730_e35817: f64 = (p.p145).ln();
                                (assign18730_e35817,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35819,)
                    }
                };
                let assign18730_e35822: f64 = (assign18730_e35820 - locals.var_niln);
                let assign18730_e35823: f64 = (locals.var_vtm * assign18730_e35822);
                let assign18730_e35824: f64 = (assign18730_e35805 - assign18730_e35823);
                let assign18730_e35826: f64 = (-10000.0);
                let assign18730_e35828: f64 = (assign18730_e35826 * 0.0001);
                let (assign18730_e35858, assign18730_e35858_d_n0, assign18730_e35858_d_n2, assign18730_e35858_d_n3, assign18730_e35858_d_n4, assign18730_e35858_d_n5, assign18730_e35858_d_n6, assign18730_e35858_d_n7, assign18730_e35858_d_n8, assign18730_e35858_d_n9, assign18730_e35858_d_n10, assign18730_e35858_d_n11, assign18730_e35858_d_n13, assign18730_e35858_d_n14,) = {
                    if (assign18730_e35824 < assign18730_e35828) {
                        let assign18730_e35831: f64 = (-0.0001);
                        let assign18730_e35833: f64 = (assign18730_e35831 * 0.0001);
                        let assign18730_e35836: f64 = (0.5 * locals.var_eg);
                        let (assign18730_e35851,) = {
                            if (!(p.p145 > 1e-38)) {
                                let assign18730_e35843: f64 = (-87.498233534);
                                (assign18730_e35843,)
                            } else {
                                let (assign18730_e35850,) = {
                                    if (p.p145 > 1e-38) {
                                        let assign18730_e35848: f64 = (p.p145).ln();
                                        (assign18730_e35848,)
                                    } else {
                                        (0.0,)
                                    }
                                };
                                (assign18730_e35850,)
                            }
                        };
                        let assign18730_e35853: f64 = (assign18730_e35851 - locals.var_niln);
                        let assign18730_e35854: f64 = (locals.var_vtm * assign18730_e35853);
                        let assign18730_e35855: f64 = (assign18730_e35836 - assign18730_e35854);
                        let assign18730_e35856: f64 = (assign18730_e35833 / assign18730_e35855);
                        (assign18730_e35856, (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn0)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn2)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn3)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18730_e35853) + (locals.var_vtm * (-locals.var_niln_dn4))))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn5)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn6)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn7)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn8)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn9)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn10)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn11)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn13)))) / (assign18730_e35855 * assign18730_e35855))), (-((assign18730_e35833 * (-(locals.var_vtm * (-locals.var_niln_dn14)))) / (assign18730_e35855 * assign18730_e35855))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18730_e35858, assign18730_e35858_d_n0, assign18730_e35858_d_n2, assign18730_e35858_d_n3, assign18730_e35858_d_n4, assign18730_e35858_d_n5, assign18730_e35858_d_n6, assign18730_e35858_d_n7, assign18730_e35858_d_n8, assign18730_e35858_d_n9, assign18730_e35858_d_n10, assign18730_e35858_d_n11, assign18730_e35858_d_n13, assign18730_e35858_d_n14,)
            }
        };
        let assign18730_e35862: f64 = (0.5 * locals.var_eg);
        let assign18730_e35866: f64 = (0.5 * locals.var_eg);
        let assign18730_e35869: f64 = (0.5 * locals.var_eg);
        let (assign18730_e35884,) = {
            if (!(p.p97 > 1e-38)) {
                let assign18730_e35876: f64 = (-87.498233534);
                (assign18730_e35876,)
            } else {
                let (assign18730_e35883,) = {
                    if (p.p97 > 1e-38) {
                        let assign18730_e35881: f64 = (p.p97).ln();
                        (assign18730_e35881,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18730_e35883,)
            }
        };
        let assign18730_e35886: f64 = (assign18730_e35884 - locals.var_niln);
        let assign18730_e35887: f64 = (locals.var_vtm * assign18730_e35886);
        let assign18730_e35888: f64 = (assign18730_e35869 - assign18730_e35887);
        let assign18730_e35890: f64 = (-10000.0);
        let assign18730_e35892: f64 = (assign18730_e35890 * 0.0001);
        let (assign18730_e36028, assign18730_e36028_d_n0, assign18730_e36028_d_n2, assign18730_e36028_d_n3, assign18730_e36028_d_n4, assign18730_e36028_d_n5, assign18730_e36028_d_n6, assign18730_e36028_d_n7, assign18730_e36028_d_n8, assign18730_e36028_d_n9, assign18730_e36028_d_n10, assign18730_e36028_d_n11, assign18730_e36028_d_n13, assign18730_e36028_d_n14,) = {
            if (!(assign18730_e35888 < assign18730_e35892)) {
                let assign18730_e35898: f64 = (0.5 * locals.var_eg);
                let (assign18730_e35913,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18730_e35905: f64 = (-87.498233534);
                        (assign18730_e35905,)
                    } else {
                        let (assign18730_e35912,) = {
                            if (p.p97 > 1e-38) {
                                let assign18730_e35910: f64 = (p.p97).ln();
                                (assign18730_e35910,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35912,)
                    }
                };
                let assign18730_e35915: f64 = (assign18730_e35913 - locals.var_niln);
                let assign18730_e35916: f64 = (locals.var_vtm * assign18730_e35915);
                let assign18730_e35917: f64 = (assign18730_e35898 - assign18730_e35916);
                let assign18730_e35920: f64 = (0.5 * locals.var_eg);
                let (assign18730_e35935,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18730_e35927: f64 = (-87.498233534);
                        (assign18730_e35927,)
                    } else {
                        let (assign18730_e35934,) = {
                            if (p.p97 > 1e-38) {
                                let assign18730_e35932: f64 = (p.p97).ln();
                                (assign18730_e35932,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35934,)
                    }
                };
                let assign18730_e35937: f64 = (assign18730_e35935 - locals.var_niln);
                let assign18730_e35938: f64 = (locals.var_vtm * assign18730_e35937);
                let assign18730_e35939: f64 = (assign18730_e35920 - assign18730_e35938);
                let assign18730_e35942: f64 = (0.5 * locals.var_eg);
                let (assign18730_e35957,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18730_e35949: f64 = (-87.498233534);
                        (assign18730_e35949,)
                    } else {
                        let (assign18730_e35956,) = {
                            if (p.p97 > 1e-38) {
                                let assign18730_e35954: f64 = (p.p97).ln();
                                (assign18730_e35954,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35956,)
                    }
                };
                let assign18730_e35959: f64 = (assign18730_e35957 - locals.var_niln);
                let assign18730_e35960: f64 = (locals.var_vtm * assign18730_e35959);
                let assign18730_e35961: f64 = (assign18730_e35942 - assign18730_e35960);
                let assign18730_e35962: f64 = (assign18730_e35939 * assign18730_e35961);
                let assign18730_e35965: f64 = (4.0 * 0.0001);
                let assign18730_e35967: f64 = (assign18730_e35965 * 0.0001);
                let assign18730_e35968: f64 = (assign18730_e35962 + assign18730_e35967);
                let assign18730_e35969: f64 = (assign18730_e35968).sqrt();
                let assign18730_e35970: f64 = (assign18730_e35917 + assign18730_e35969);
                let assign18730_e35971: f64 = (0.5 * assign18730_e35970);
                (assign18730_e35971, (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn0))) + ((((-(locals.var_vtm * (-locals.var_niln_dn0))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn0))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn2))) + ((((-(locals.var_vtm * (-locals.var_niln_dn2))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn2))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn3))) + ((((-(locals.var_vtm * (-locals.var_niln_dn3))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn3))))) / (2.0 * assign18730_e35969)))), (0.5 * (((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18730_e35915) + (locals.var_vtm * (-locals.var_niln_dn4)))) + (((((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18730_e35937) + (locals.var_vtm * (-locals.var_niln_dn4)))) * assign18730_e35961) + (assign18730_e35939 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18730_e35959) + (locals.var_vtm * (-locals.var_niln_dn4)))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn5))) + ((((-(locals.var_vtm * (-locals.var_niln_dn5))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn5))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn6))) + ((((-(locals.var_vtm * (-locals.var_niln_dn6))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn6))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn7))) + ((((-(locals.var_vtm * (-locals.var_niln_dn7))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn7))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn8))) + ((((-(locals.var_vtm * (-locals.var_niln_dn8))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn8))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn9))) + ((((-(locals.var_vtm * (-locals.var_niln_dn9))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn9))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn10))) + ((((-(locals.var_vtm * (-locals.var_niln_dn10))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn10))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn11))) + ((((-(locals.var_vtm * (-locals.var_niln_dn11))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn11))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn13))) + ((((-(locals.var_vtm * (-locals.var_niln_dn13))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn13))))) / (2.0 * assign18730_e35969)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn14))) + ((((-(locals.var_vtm * (-locals.var_niln_dn14))) * assign18730_e35961) + (assign18730_e35939 * (-(locals.var_vtm * (-locals.var_niln_dn14))))) / (2.0 * assign18730_e35969)))),)
            } else {
                let assign18730_e35974: f64 = (0.5 * locals.var_eg);
                let (assign18730_e35989,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18730_e35981: f64 = (-87.498233534);
                        (assign18730_e35981,)
                    } else {
                        let (assign18730_e35988,) = {
                            if (p.p97 > 1e-38) {
                                let assign18730_e35986: f64 = (p.p97).ln();
                                (assign18730_e35986,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18730_e35988,)
                    }
                };
                let assign18730_e35991: f64 = (assign18730_e35989 - locals.var_niln);
                let assign18730_e35992: f64 = (locals.var_vtm * assign18730_e35991);
                let assign18730_e35993: f64 = (assign18730_e35974 - assign18730_e35992);
                let assign18730_e35995: f64 = (-10000.0);
                let assign18730_e35997: f64 = (assign18730_e35995 * 0.0001);
                let (assign18730_e36027, assign18730_e36027_d_n0, assign18730_e36027_d_n2, assign18730_e36027_d_n3, assign18730_e36027_d_n4, assign18730_e36027_d_n5, assign18730_e36027_d_n6, assign18730_e36027_d_n7, assign18730_e36027_d_n8, assign18730_e36027_d_n9, assign18730_e36027_d_n10, assign18730_e36027_d_n11, assign18730_e36027_d_n13, assign18730_e36027_d_n14,) = {
                    if (assign18730_e35993 < assign18730_e35997) {
                        let assign18730_e36000: f64 = (-0.0001);
                        let assign18730_e36002: f64 = (assign18730_e36000 * 0.0001);
                        let assign18730_e36005: f64 = (0.5 * locals.var_eg);
                        let (assign18730_e36020,) = {
                            if (!(p.p97 > 1e-38)) {
                                let assign18730_e36012: f64 = (-87.498233534);
                                (assign18730_e36012,)
                            } else {
                                let (assign18730_e36019,) = {
                                    if (p.p97 > 1e-38) {
                                        let assign18730_e36017: f64 = (p.p97).ln();
                                        (assign18730_e36017,)
                                    } else {
                                        (0.0,)
                                    }
                                };
                                (assign18730_e36019,)
                            }
                        };
                        let assign18730_e36022: f64 = (assign18730_e36020 - locals.var_niln);
                        let assign18730_e36023: f64 = (locals.var_vtm * assign18730_e36022);
                        let assign18730_e36024: f64 = (assign18730_e36005 - assign18730_e36023);
                        let assign18730_e36025: f64 = (assign18730_e36002 / assign18730_e36024);
                        (assign18730_e36025, (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn0)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn2)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn3)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18730_e36022) + (locals.var_vtm * (-locals.var_niln_dn4))))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn5)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn6)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn7)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn8)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn9)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn10)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn11)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn13)))) / (assign18730_e36024 * assign18730_e36024))), (-((assign18730_e36002 * (-(locals.var_vtm * (-locals.var_niln_dn14)))) / (assign18730_e36024 * assign18730_e36024))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18730_e36027, assign18730_e36027_d_n0, assign18730_e36027_d_n2, assign18730_e36027_d_n3, assign18730_e36027_d_n4, assign18730_e36027_d_n5, assign18730_e36027_d_n6, assign18730_e36027_d_n7, assign18730_e36027_d_n8, assign18730_e36027_d_n9, assign18730_e36027_d_n10, assign18730_e36027_d_n11, assign18730_e36027_d_n13, assign18730_e36027_d_n14,)
            }
        };
        let assign18730_e36029: f64 = (assign18730_e35866 - assign18730_e36028);
        let assign18730_e36030: f64 = (locals.var_devsign * assign18730_e36029);
        let assign18730_e36031: f64 = (assign18730_e35862 - assign18730_e36030);
        let assign18730_e36032: f64 = (assign18730_e35859 - assign18730_e36031);
        let assign18730_e36033: f64 = (locals.var_devsign * assign18730_e36032);
        (assign18730_e36033, (locals.var_devsign * (assign18730_e35859_d_n0 - (-(locals.var_devsign * (-assign18730_e36028_d_n0))))), (locals.var_devsign * (assign18730_e35859_d_n2 - (-(locals.var_devsign * (-assign18730_e36028_d_n2))))), (locals.var_devsign * (assign18730_e35859_d_n3 - (-(locals.var_devsign * (-assign18730_e36028_d_n3))))), (locals.var_devsign * (assign18730_e35859_d_n4 - ((0.5 * locals.var_eg_dn4) - (locals.var_devsign * ((0.5 * locals.var_eg_dn4) - assign18730_e36028_d_n4))))), (locals.var_devsign * (assign18730_e35859_d_n5 - (-(locals.var_devsign * (-assign18730_e36028_d_n5))))), (locals.var_devsign * (assign18730_e35859_d_n6 - (-(locals.var_devsign * (-assign18730_e36028_d_n6))))), (locals.var_devsign * (assign18730_e35859_d_n7 - (-(locals.var_devsign * (-assign18730_e36028_d_n7))))), (locals.var_devsign * (assign18730_e35859_d_n8 - (-(locals.var_devsign * (-assign18730_e36028_d_n8))))), (locals.var_devsign * (assign18730_e35859_d_n9 - (-(locals.var_devsign * (-assign18730_e36028_d_n9))))), (locals.var_devsign * (assign18730_e35859_d_n10 - (-(locals.var_devsign * (-assign18730_e36028_d_n10))))), (locals.var_devsign * (assign18730_e35859_d_n11 - (-(locals.var_devsign * (-assign18730_e36028_d_n11))))), (locals.var_devsign * (assign18730_e35859_d_n13 - (-(locals.var_devsign * (-assign18730_e36028_d_n13))))), (locals.var_devsign * (assign18730_e35859_d_n14 - (-(locals.var_devsign * (-assign18730_e36028_d_n14))))),)
    } else {
        (locals.var_vfbsd_v, locals.var_vfbsd_v_dn0, locals.var_vfbsd_v_dn2, locals.var_vfbsd_v_dn3, locals.var_vfbsd_v_dn4, locals.var_vfbsd_v_dn5, locals.var_vfbsd_v_dn6, locals.var_vfbsd_v_dn7, locals.var_vfbsd_v_dn8, locals.var_vfbsd_v_dn9, locals.var_vfbsd_v_dn10, locals.var_vfbsd_v_dn11, locals.var_vfbsd_v_dn13, locals.var_vfbsd_v_dn14,)
    }
};
        locals.var_vfbsd_v = assign18730_e36035;
        locals.var_vfbsd_v_dn0 = assign18730_e36035_d_n0;
        locals.var_vfbsd_v_dn2 = assign18730_e36035_d_n2;
        locals.var_vfbsd_v_dn3 = assign18730_e36035_d_n3;
        locals.var_vfbsd_v_dn4 = assign18730_e36035_d_n4;
        locals.var_vfbsd_v_dn5 = assign18730_e36035_d_n5;
        locals.var_vfbsd_v_dn6 = assign18730_e36035_d_n6;
        locals.var_vfbsd_v_dn7 = assign18730_e36035_d_n7;
        locals.var_vfbsd_v_dn8 = assign18730_e36035_d_n8;
        locals.var_vfbsd_v_dn9 = assign18730_e36035_d_n9;
        locals.var_vfbsd_v_dn10 = assign18730_e36035_d_n10;
        locals.var_vfbsd_v_dn11 = assign18730_e36035_d_n11;
        locals.var_vfbsd_v_dn13 = assign18730_e36035_d_n13;
        locals.var_vfbsd_v_dn14 = assign18730_e36035_d_n14;

        let assign18740_e36038: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard346 = assign18740_e36038;

    }

    pub(super) fn stamp_transient_block_67(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign18750_e36248, assign18750_e36248_d_n0, assign18750_e36248_d_n2, assign18750_e36248_d_n3, assign18750_e36248_d_n4, assign18750_e36248_d_n5, assign18750_e36248_d_n6, assign18750_e36248_d_n7, assign18750_e36248_d_n8, assign18750_e36248_d_n9, assign18750_e36248_d_n10, assign18750_e36248_d_n11, assign18750_e36248_d_n13, assign18750_e36248_d_n14,) = {
    if (((locals.var_guard343 != 0.0) && (locals.var_guard344 == 0.0)) && (locals.var_guard346 != 0.0)) {
        let assign18750_e36050: f64 = (0.5 * locals.var_eg);
        let assign18750_e36051: f64 = (p.p104 + assign18750_e36050);
        let assign18750_e36055: f64 = (0.5 * locals.var_eg);
        let assign18750_e36058: f64 = (0.5 * locals.var_eg);
        let assign18750_e36062: f64 = (p.p97 / locals.var_ni);
        let (assign18750_e36079, assign18750_e36079_d_n0, assign18750_e36079_d_n2, assign18750_e36079_d_n3, assign18750_e36079_d_n4, assign18750_e36079_d_n5, assign18750_e36079_d_n6, assign18750_e36079_d_n7, assign18750_e36079_d_n8, assign18750_e36079_d_n9, assign18750_e36079_d_n10, assign18750_e36079_d_n11, assign18750_e36079_d_n13, assign18750_e36079_d_n14,) = {
            if (!(assign18750_e36062 > 1e-38)) {
                let assign18750_e36067: f64 = (-87.498233534);
                (assign18750_e36067, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18750_e36070: f64 = (p.p97 / locals.var_ni);
                let (assign18750_e36078, assign18750_e36078_d_n0, assign18750_e36078_d_n2, assign18750_e36078_d_n3, assign18750_e36078_d_n4, assign18750_e36078_d_n5, assign18750_e36078_d_n6, assign18750_e36078_d_n7, assign18750_e36078_d_n8, assign18750_e36078_d_n9, assign18750_e36078_d_n10, assign18750_e36078_d_n11, assign18750_e36078_d_n13, assign18750_e36078_d_n14,) = {
                    if (assign18750_e36070 > 1e-38) {
                        let assign18750_e36075: f64 = (p.p97 / locals.var_ni);
                        let assign18750_e36076: f64 = (assign18750_e36075).ln();
                        (assign18750_e36076, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18750_e36075), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18750_e36075),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18750_e36078, assign18750_e36078_d_n0, assign18750_e36078_d_n2, assign18750_e36078_d_n3, assign18750_e36078_d_n4, assign18750_e36078_d_n5, assign18750_e36078_d_n6, assign18750_e36078_d_n7, assign18750_e36078_d_n8, assign18750_e36078_d_n9, assign18750_e36078_d_n10, assign18750_e36078_d_n11, assign18750_e36078_d_n13, assign18750_e36078_d_n14,)
            }
        };
        let assign18750_e36080: f64 = (locals.var_vtm * assign18750_e36079);
        let assign18750_e36081: f64 = (assign18750_e36058 - assign18750_e36080);
        let assign18750_e36083: f64 = (-10000.0);
        let assign18750_e36085: f64 = (assign18750_e36083 * 0.0001);
        let (assign18750_e36241, assign18750_e36241_d_n0, assign18750_e36241_d_n2, assign18750_e36241_d_n3, assign18750_e36241_d_n4, assign18750_e36241_d_n5, assign18750_e36241_d_n6, assign18750_e36241_d_n7, assign18750_e36241_d_n8, assign18750_e36241_d_n9, assign18750_e36241_d_n10, assign18750_e36241_d_n11, assign18750_e36241_d_n13, assign18750_e36241_d_n14,) = {
            if (!(assign18750_e36081 < assign18750_e36085)) {
                let assign18750_e36091: f64 = (0.5 * locals.var_eg);
                let assign18750_e36095: f64 = (p.p97 / locals.var_ni);
                let (assign18750_e36112, assign18750_e36112_d_n0, assign18750_e36112_d_n2, assign18750_e36112_d_n3, assign18750_e36112_d_n4, assign18750_e36112_d_n5, assign18750_e36112_d_n6, assign18750_e36112_d_n7, assign18750_e36112_d_n8, assign18750_e36112_d_n9, assign18750_e36112_d_n10, assign18750_e36112_d_n11, assign18750_e36112_d_n13, assign18750_e36112_d_n14,) = {
                    if (!(assign18750_e36095 > 1e-38)) {
                        let assign18750_e36100: f64 = (-87.498233534);
                        (assign18750_e36100, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18750_e36103: f64 = (p.p97 / locals.var_ni);
                        let (assign18750_e36111, assign18750_e36111_d_n0, assign18750_e36111_d_n2, assign18750_e36111_d_n3, assign18750_e36111_d_n4, assign18750_e36111_d_n5, assign18750_e36111_d_n6, assign18750_e36111_d_n7, assign18750_e36111_d_n8, assign18750_e36111_d_n9, assign18750_e36111_d_n10, assign18750_e36111_d_n11, assign18750_e36111_d_n13, assign18750_e36111_d_n14,) = {
                            if (assign18750_e36103 > 1e-38) {
                                let assign18750_e36108: f64 = (p.p97 / locals.var_ni);
                                let assign18750_e36109: f64 = (assign18750_e36108).ln();
                                (assign18750_e36109, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18750_e36108), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18750_e36108),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18750_e36111, assign18750_e36111_d_n0, assign18750_e36111_d_n2, assign18750_e36111_d_n3, assign18750_e36111_d_n4, assign18750_e36111_d_n5, assign18750_e36111_d_n6, assign18750_e36111_d_n7, assign18750_e36111_d_n8, assign18750_e36111_d_n9, assign18750_e36111_d_n10, assign18750_e36111_d_n11, assign18750_e36111_d_n13, assign18750_e36111_d_n14,)
                    }
                };
                let assign18750_e36113: f64 = (locals.var_vtm * assign18750_e36112);
                let assign18750_e36114: f64 = (assign18750_e36091 - assign18750_e36113);
                let assign18750_e36117: f64 = (0.5 * locals.var_eg);
                let assign18750_e36121: f64 = (p.p97 / locals.var_ni);
                let (assign18750_e36138, assign18750_e36138_d_n0, assign18750_e36138_d_n2, assign18750_e36138_d_n3, assign18750_e36138_d_n4, assign18750_e36138_d_n5, assign18750_e36138_d_n6, assign18750_e36138_d_n7, assign18750_e36138_d_n8, assign18750_e36138_d_n9, assign18750_e36138_d_n10, assign18750_e36138_d_n11, assign18750_e36138_d_n13, assign18750_e36138_d_n14,) = {
                    if (!(assign18750_e36121 > 1e-38)) {
                        let assign18750_e36126: f64 = (-87.498233534);
                        (assign18750_e36126, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18750_e36129: f64 = (p.p97 / locals.var_ni);
                        let (assign18750_e36137, assign18750_e36137_d_n0, assign18750_e36137_d_n2, assign18750_e36137_d_n3, assign18750_e36137_d_n4, assign18750_e36137_d_n5, assign18750_e36137_d_n6, assign18750_e36137_d_n7, assign18750_e36137_d_n8, assign18750_e36137_d_n9, assign18750_e36137_d_n10, assign18750_e36137_d_n11, assign18750_e36137_d_n13, assign18750_e36137_d_n14,) = {
                            if (assign18750_e36129 > 1e-38) {
                                let assign18750_e36134: f64 = (p.p97 / locals.var_ni);
                                let assign18750_e36135: f64 = (assign18750_e36134).ln();
                                (assign18750_e36135, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18750_e36134), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18750_e36134),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18750_e36137, assign18750_e36137_d_n0, assign18750_e36137_d_n2, assign18750_e36137_d_n3, assign18750_e36137_d_n4, assign18750_e36137_d_n5, assign18750_e36137_d_n6, assign18750_e36137_d_n7, assign18750_e36137_d_n8, assign18750_e36137_d_n9, assign18750_e36137_d_n10, assign18750_e36137_d_n11, assign18750_e36137_d_n13, assign18750_e36137_d_n14,)
                    }
                };
                let assign18750_e36139: f64 = (locals.var_vtm * assign18750_e36138);
                let assign18750_e36140: f64 = (assign18750_e36117 - assign18750_e36139);
                let assign18750_e36143: f64 = (0.5 * locals.var_eg);
                let assign18750_e36147: f64 = (p.p97 / locals.var_ni);
                let (assign18750_e36164, assign18750_e36164_d_n0, assign18750_e36164_d_n2, assign18750_e36164_d_n3, assign18750_e36164_d_n4, assign18750_e36164_d_n5, assign18750_e36164_d_n6, assign18750_e36164_d_n7, assign18750_e36164_d_n8, assign18750_e36164_d_n9, assign18750_e36164_d_n10, assign18750_e36164_d_n11, assign18750_e36164_d_n13, assign18750_e36164_d_n14,) = {
                    if (!(assign18750_e36147 > 1e-38)) {
                        let assign18750_e36152: f64 = (-87.498233534);
                        (assign18750_e36152, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18750_e36155: f64 = (p.p97 / locals.var_ni);
                        let (assign18750_e36163, assign18750_e36163_d_n0, assign18750_e36163_d_n2, assign18750_e36163_d_n3, assign18750_e36163_d_n4, assign18750_e36163_d_n5, assign18750_e36163_d_n6, assign18750_e36163_d_n7, assign18750_e36163_d_n8, assign18750_e36163_d_n9, assign18750_e36163_d_n10, assign18750_e36163_d_n11, assign18750_e36163_d_n13, assign18750_e36163_d_n14,) = {
                            if (assign18750_e36155 > 1e-38) {
                                let assign18750_e36160: f64 = (p.p97 / locals.var_ni);
                                let assign18750_e36161: f64 = (assign18750_e36160).ln();
                                (assign18750_e36161, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18750_e36160), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18750_e36160),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18750_e36163, assign18750_e36163_d_n0, assign18750_e36163_d_n2, assign18750_e36163_d_n3, assign18750_e36163_d_n4, assign18750_e36163_d_n5, assign18750_e36163_d_n6, assign18750_e36163_d_n7, assign18750_e36163_d_n8, assign18750_e36163_d_n9, assign18750_e36163_d_n10, assign18750_e36163_d_n11, assign18750_e36163_d_n13, assign18750_e36163_d_n14,)
                    }
                };
                let assign18750_e36165: f64 = (locals.var_vtm * assign18750_e36164);
                let assign18750_e36166: f64 = (assign18750_e36143 - assign18750_e36165);
                let assign18750_e36167: f64 = (assign18750_e36140 * assign18750_e36166);
                let assign18750_e36170: f64 = (4.0 * 0.0001);
                let assign18750_e36172: f64 = (assign18750_e36170 * 0.0001);
                let assign18750_e36173: f64 = (assign18750_e36167 + assign18750_e36172);
                let assign18750_e36174: f64 = (assign18750_e36173).sqrt();
                let assign18750_e36175: f64 = (assign18750_e36114 + assign18750_e36174);
                let assign18750_e36176: f64 = (0.5 * assign18750_e36175);
                (assign18750_e36176, (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n0)) + ((((-(locals.var_vtm * assign18750_e36138_d_n0)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n0)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n2)) + ((((-(locals.var_vtm * assign18750_e36138_d_n2)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n2)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n3)) + ((((-(locals.var_vtm * assign18750_e36138_d_n3)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n3)))) / (2.0 * assign18750_e36174)))), (0.5 * (((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18750_e36112) + (locals.var_vtm * assign18750_e36112_d_n4))) + (((((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18750_e36138) + (locals.var_vtm * assign18750_e36138_d_n4))) * assign18750_e36166) + (assign18750_e36140 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18750_e36164) + (locals.var_vtm * assign18750_e36164_d_n4))))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n5)) + ((((-(locals.var_vtm * assign18750_e36138_d_n5)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n5)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n6)) + ((((-(locals.var_vtm * assign18750_e36138_d_n6)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n6)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n7)) + ((((-(locals.var_vtm * assign18750_e36138_d_n7)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n7)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n8)) + ((((-(locals.var_vtm * assign18750_e36138_d_n8)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n8)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n9)) + ((((-(locals.var_vtm * assign18750_e36138_d_n9)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n9)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n10)) + ((((-(locals.var_vtm * assign18750_e36138_d_n10)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n10)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n11)) + ((((-(locals.var_vtm * assign18750_e36138_d_n11)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n11)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n13)) + ((((-(locals.var_vtm * assign18750_e36138_d_n13)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n13)))) / (2.0 * assign18750_e36174)))), (0.5 * ((-(locals.var_vtm * assign18750_e36112_d_n14)) + ((((-(locals.var_vtm * assign18750_e36138_d_n14)) * assign18750_e36166) + (assign18750_e36140 * (-(locals.var_vtm * assign18750_e36164_d_n14)))) / (2.0 * assign18750_e36174)))),)
            } else {
                let assign18750_e36179: f64 = (0.5 * locals.var_eg);
                let assign18750_e36183: f64 = (p.p97 / locals.var_ni);
                let (assign18750_e36200, assign18750_e36200_d_n0, assign18750_e36200_d_n2, assign18750_e36200_d_n3, assign18750_e36200_d_n4, assign18750_e36200_d_n5, assign18750_e36200_d_n6, assign18750_e36200_d_n7, assign18750_e36200_d_n8, assign18750_e36200_d_n9, assign18750_e36200_d_n10, assign18750_e36200_d_n11, assign18750_e36200_d_n13, assign18750_e36200_d_n14,) = {
                    if (!(assign18750_e36183 > 1e-38)) {
                        let assign18750_e36188: f64 = (-87.498233534);
                        (assign18750_e36188, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let assign18750_e36191: f64 = (p.p97 / locals.var_ni);
                        let (assign18750_e36199, assign18750_e36199_d_n0, assign18750_e36199_d_n2, assign18750_e36199_d_n3, assign18750_e36199_d_n4, assign18750_e36199_d_n5, assign18750_e36199_d_n6, assign18750_e36199_d_n7, assign18750_e36199_d_n8, assign18750_e36199_d_n9, assign18750_e36199_d_n10, assign18750_e36199_d_n11, assign18750_e36199_d_n13, assign18750_e36199_d_n14,) = {
                            if (assign18750_e36191 > 1e-38) {
                                let assign18750_e36196: f64 = (p.p97 / locals.var_ni);
                                let assign18750_e36197: f64 = (assign18750_e36196).ln();
                                (assign18750_e36197, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18750_e36196), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18750_e36196),)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign18750_e36199, assign18750_e36199_d_n0, assign18750_e36199_d_n2, assign18750_e36199_d_n3, assign18750_e36199_d_n4, assign18750_e36199_d_n5, assign18750_e36199_d_n6, assign18750_e36199_d_n7, assign18750_e36199_d_n8, assign18750_e36199_d_n9, assign18750_e36199_d_n10, assign18750_e36199_d_n11, assign18750_e36199_d_n13, assign18750_e36199_d_n14,)
                    }
                };
                let assign18750_e36201: f64 = (locals.var_vtm * assign18750_e36200);
                let assign18750_e36202: f64 = (assign18750_e36179 - assign18750_e36201);
                let assign18750_e36204: f64 = (-10000.0);
                let assign18750_e36206: f64 = (assign18750_e36204 * 0.0001);
                let (assign18750_e36240, assign18750_e36240_d_n0, assign18750_e36240_d_n2, assign18750_e36240_d_n3, assign18750_e36240_d_n4, assign18750_e36240_d_n5, assign18750_e36240_d_n6, assign18750_e36240_d_n7, assign18750_e36240_d_n8, assign18750_e36240_d_n9, assign18750_e36240_d_n10, assign18750_e36240_d_n11, assign18750_e36240_d_n13, assign18750_e36240_d_n14,) = {
                    if (assign18750_e36202 < assign18750_e36206) {
                        let assign18750_e36209: f64 = (-0.0001);
                        let assign18750_e36211: f64 = (assign18750_e36209 * 0.0001);
                        let assign18750_e36214: f64 = (0.5 * locals.var_eg);
                        let assign18750_e36218: f64 = (p.p97 / locals.var_ni);
                        let (assign18750_e36235, assign18750_e36235_d_n0, assign18750_e36235_d_n2, assign18750_e36235_d_n3, assign18750_e36235_d_n4, assign18750_e36235_d_n5, assign18750_e36235_d_n6, assign18750_e36235_d_n7, assign18750_e36235_d_n8, assign18750_e36235_d_n9, assign18750_e36235_d_n10, assign18750_e36235_d_n11, assign18750_e36235_d_n13, assign18750_e36235_d_n14,) = {
                            if (!(assign18750_e36218 > 1e-38)) {
                                let assign18750_e36223: f64 = (-87.498233534);
                                (assign18750_e36223, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            } else {
                                let assign18750_e36226: f64 = (p.p97 / locals.var_ni);
                                let (assign18750_e36234, assign18750_e36234_d_n0, assign18750_e36234_d_n2, assign18750_e36234_d_n3, assign18750_e36234_d_n4, assign18750_e36234_d_n5, assign18750_e36234_d_n6, assign18750_e36234_d_n7, assign18750_e36234_d_n8, assign18750_e36234_d_n9, assign18750_e36234_d_n10, assign18750_e36234_d_n11, assign18750_e36234_d_n13, assign18750_e36234_d_n14,) = {
                                    if (assign18750_e36226 > 1e-38) {
                                        let assign18750_e36231: f64 = (p.p97 / locals.var_ni);
                                        let assign18750_e36232: f64 = (assign18750_e36231).ln();
                                        (assign18750_e36232, ((-((p.p97 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18750_e36231), ((-((p.p97 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18750_e36231),)
                                    } else {
                                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                                    }
                                };
                                (assign18750_e36234, assign18750_e36234_d_n0, assign18750_e36234_d_n2, assign18750_e36234_d_n3, assign18750_e36234_d_n4, assign18750_e36234_d_n5, assign18750_e36234_d_n6, assign18750_e36234_d_n7, assign18750_e36234_d_n8, assign18750_e36234_d_n9, assign18750_e36234_d_n10, assign18750_e36234_d_n11, assign18750_e36234_d_n13, assign18750_e36234_d_n14,)
                            }
                        };
                        let assign18750_e36236: f64 = (locals.var_vtm * assign18750_e36235);
                        let assign18750_e36237: f64 = (assign18750_e36214 - assign18750_e36236);
                        let assign18750_e36238: f64 = (assign18750_e36211 / assign18750_e36237);
                        (assign18750_e36238, (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n0))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n2))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n3))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18750_e36235) + (locals.var_vtm * assign18750_e36235_d_n4)))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n5))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n6))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n7))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n8))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n9))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n10))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n11))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n13))) / (assign18750_e36237 * assign18750_e36237))), (-((assign18750_e36211 * (-(locals.var_vtm * assign18750_e36235_d_n14))) / (assign18750_e36237 * assign18750_e36237))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18750_e36240, assign18750_e36240_d_n0, assign18750_e36240_d_n2, assign18750_e36240_d_n3, assign18750_e36240_d_n4, assign18750_e36240_d_n5, assign18750_e36240_d_n6, assign18750_e36240_d_n7, assign18750_e36240_d_n8, assign18750_e36240_d_n9, assign18750_e36240_d_n10, assign18750_e36240_d_n11, assign18750_e36240_d_n13, assign18750_e36240_d_n14,)
            }
        };
        let assign18750_e36242: f64 = (assign18750_e36055 - assign18750_e36241);
        let assign18750_e36243: f64 = (locals.var_devsign * assign18750_e36242);
        let assign18750_e36244: f64 = (assign18750_e36051 - assign18750_e36243);
        let assign18750_e36245: f64 = (locals.var_phig_i - assign18750_e36244);
        let assign18750_e36246: f64 = (locals.var_devsign * assign18750_e36245);
        (assign18750_e36246, (locals.var_devsign * (locals.var_phig_i_dn0 - (-(locals.var_devsign * (-assign18750_e36241_d_n0))))), (locals.var_devsign * (locals.var_phig_i_dn2 - (-(locals.var_devsign * (-assign18750_e36241_d_n2))))), (locals.var_devsign * (locals.var_phig_i_dn3 - (-(locals.var_devsign * (-assign18750_e36241_d_n3))))), (locals.var_devsign * (locals.var_phig_i_dn4 - ((0.5 * locals.var_eg_dn4) - (locals.var_devsign * ((0.5 * locals.var_eg_dn4) - assign18750_e36241_d_n4))))), (locals.var_devsign * (locals.var_phig_i_dn5 - (-(locals.var_devsign * (-assign18750_e36241_d_n5))))), (locals.var_devsign * (locals.var_phig_i_dn6 - (-(locals.var_devsign * (-assign18750_e36241_d_n6))))), (locals.var_devsign * (locals.var_phig_i_dn7 - (-(locals.var_devsign * (-assign18750_e36241_d_n7))))), (locals.var_devsign * (locals.var_phig_i_dn8 - (-(locals.var_devsign * (-assign18750_e36241_d_n8))))), (locals.var_devsign * (locals.var_phig_i_dn9 - (-(locals.var_devsign * (-assign18750_e36241_d_n9))))), (locals.var_devsign * (locals.var_phig_i_dn10 - (-(locals.var_devsign * (-assign18750_e36241_d_n10))))), (locals.var_devsign * (locals.var_phig_i_dn11 - (-(locals.var_devsign * (-assign18750_e36241_d_n11))))), (locals.var_devsign * (locals.var_phig_i_dn13 - (-(locals.var_devsign * (-assign18750_e36241_d_n13))))), (locals.var_devsign * (locals.var_phig_i_dn14 - (-(locals.var_devsign * (-assign18750_e36241_d_n14))))),)
    } else {
        (locals.var_vfbsd_v, locals.var_vfbsd_v_dn0, locals.var_vfbsd_v_dn2, locals.var_vfbsd_v_dn3, locals.var_vfbsd_v_dn4, locals.var_vfbsd_v_dn5, locals.var_vfbsd_v_dn6, locals.var_vfbsd_v_dn7, locals.var_vfbsd_v_dn8, locals.var_vfbsd_v_dn9, locals.var_vfbsd_v_dn10, locals.var_vfbsd_v_dn11, locals.var_vfbsd_v_dn13, locals.var_vfbsd_v_dn14,)
    }
};
        locals.var_vfbsd_v = assign18750_e36248;
        locals.var_vfbsd_v_dn0 = assign18750_e36248_d_n0;
        locals.var_vfbsd_v_dn2 = assign18750_e36248_d_n2;
        locals.var_vfbsd_v_dn3 = assign18750_e36248_d_n3;
        locals.var_vfbsd_v_dn4 = assign18750_e36248_d_n4;
        locals.var_vfbsd_v_dn5 = assign18750_e36248_d_n5;
        locals.var_vfbsd_v_dn6 = assign18750_e36248_d_n6;
        locals.var_vfbsd_v_dn7 = assign18750_e36248_d_n7;
        locals.var_vfbsd_v_dn8 = assign18750_e36248_d_n8;
        locals.var_vfbsd_v_dn9 = assign18750_e36248_d_n9;
        locals.var_vfbsd_v_dn10 = assign18750_e36248_d_n10;
        locals.var_vfbsd_v_dn11 = assign18750_e36248_d_n11;
        locals.var_vfbsd_v_dn13 = assign18750_e36248_d_n13;
        locals.var_vfbsd_v_dn14 = assign18750_e36248_d_n14;

        let (assign18760_e36435, assign18760_e36435_d_n0, assign18760_e36435_d_n2, assign18760_e36435_d_n3, assign18760_e36435_d_n4, assign18760_e36435_d_n5, assign18760_e36435_d_n6, assign18760_e36435_d_n7, assign18760_e36435_d_n8, assign18760_e36435_d_n9, assign18760_e36435_d_n10, assign18760_e36435_d_n11, assign18760_e36435_d_n13, assign18760_e36435_d_n14,) = {
    if (((locals.var_guard343 != 0.0) && (locals.var_guard344 == 0.0)) && (locals.var_guard346 == 0.0)) {
        let assign18760_e36261: f64 = (0.5 * locals.var_eg);
        let assign18760_e36262: f64 = (p.p104 + assign18760_e36261);
        let assign18760_e36266: f64 = (0.5 * locals.var_eg);
        let assign18760_e36269: f64 = (0.5 * locals.var_eg);
        let (assign18760_e36284,) = {
            if (!(p.p97 > 1e-38)) {
                let assign18760_e36276: f64 = (-87.498233534);
                (assign18760_e36276,)
            } else {
                let (assign18760_e36283,) = {
                    if (p.p97 > 1e-38) {
                        let assign18760_e36281: f64 = (p.p97).ln();
                        (assign18760_e36281,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18760_e36283,)
            }
        };
        let assign18760_e36286: f64 = (assign18760_e36284 - locals.var_niln);
        let assign18760_e36287: f64 = (locals.var_vtm * assign18760_e36286);
        let assign18760_e36288: f64 = (assign18760_e36269 - assign18760_e36287);
        let assign18760_e36290: f64 = (-10000.0);
        let assign18760_e36292: f64 = (assign18760_e36290 * 0.0001);
        let (assign18760_e36428, assign18760_e36428_d_n0, assign18760_e36428_d_n2, assign18760_e36428_d_n3, assign18760_e36428_d_n4, assign18760_e36428_d_n5, assign18760_e36428_d_n6, assign18760_e36428_d_n7, assign18760_e36428_d_n8, assign18760_e36428_d_n9, assign18760_e36428_d_n10, assign18760_e36428_d_n11, assign18760_e36428_d_n13, assign18760_e36428_d_n14,) = {
            if (!(assign18760_e36288 < assign18760_e36292)) {
                let assign18760_e36298: f64 = (0.5 * locals.var_eg);
                let (assign18760_e36313,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18760_e36305: f64 = (-87.498233534);
                        (assign18760_e36305,)
                    } else {
                        let (assign18760_e36312,) = {
                            if (p.p97 > 1e-38) {
                                let assign18760_e36310: f64 = (p.p97).ln();
                                (assign18760_e36310,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18760_e36312,)
                    }
                };
                let assign18760_e36315: f64 = (assign18760_e36313 - locals.var_niln);
                let assign18760_e36316: f64 = (locals.var_vtm * assign18760_e36315);
                let assign18760_e36317: f64 = (assign18760_e36298 - assign18760_e36316);
                let assign18760_e36320: f64 = (0.5 * locals.var_eg);
                let (assign18760_e36335,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18760_e36327: f64 = (-87.498233534);
                        (assign18760_e36327,)
                    } else {
                        let (assign18760_e36334,) = {
                            if (p.p97 > 1e-38) {
                                let assign18760_e36332: f64 = (p.p97).ln();
                                (assign18760_e36332,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18760_e36334,)
                    }
                };
                let assign18760_e36337: f64 = (assign18760_e36335 - locals.var_niln);
                let assign18760_e36338: f64 = (locals.var_vtm * assign18760_e36337);
                let assign18760_e36339: f64 = (assign18760_e36320 - assign18760_e36338);
                let assign18760_e36342: f64 = (0.5 * locals.var_eg);
                let (assign18760_e36357,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18760_e36349: f64 = (-87.498233534);
                        (assign18760_e36349,)
                    } else {
                        let (assign18760_e36356,) = {
                            if (p.p97 > 1e-38) {
                                let assign18760_e36354: f64 = (p.p97).ln();
                                (assign18760_e36354,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18760_e36356,)
                    }
                };
                let assign18760_e36359: f64 = (assign18760_e36357 - locals.var_niln);
                let assign18760_e36360: f64 = (locals.var_vtm * assign18760_e36359);
                let assign18760_e36361: f64 = (assign18760_e36342 - assign18760_e36360);
                let assign18760_e36362: f64 = (assign18760_e36339 * assign18760_e36361);
                let assign18760_e36365: f64 = (4.0 * 0.0001);
                let assign18760_e36367: f64 = (assign18760_e36365 * 0.0001);
                let assign18760_e36368: f64 = (assign18760_e36362 + assign18760_e36367);
                let assign18760_e36369: f64 = (assign18760_e36368).sqrt();
                let assign18760_e36370: f64 = (assign18760_e36317 + assign18760_e36369);
                let assign18760_e36371: f64 = (0.5 * assign18760_e36370);
                (assign18760_e36371, (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn0))) + ((((-(locals.var_vtm * (-locals.var_niln_dn0))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn0))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn2))) + ((((-(locals.var_vtm * (-locals.var_niln_dn2))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn2))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn3))) + ((((-(locals.var_vtm * (-locals.var_niln_dn3))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn3))))) / (2.0 * assign18760_e36369)))), (0.5 * (((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18760_e36315) + (locals.var_vtm * (-locals.var_niln_dn4)))) + (((((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18760_e36337) + (locals.var_vtm * (-locals.var_niln_dn4)))) * assign18760_e36361) + (assign18760_e36339 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18760_e36359) + (locals.var_vtm * (-locals.var_niln_dn4)))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn5))) + ((((-(locals.var_vtm * (-locals.var_niln_dn5))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn5))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn6))) + ((((-(locals.var_vtm * (-locals.var_niln_dn6))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn6))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn7))) + ((((-(locals.var_vtm * (-locals.var_niln_dn7))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn7))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn8))) + ((((-(locals.var_vtm * (-locals.var_niln_dn8))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn8))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn9))) + ((((-(locals.var_vtm * (-locals.var_niln_dn9))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn9))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn10))) + ((((-(locals.var_vtm * (-locals.var_niln_dn10))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn10))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn11))) + ((((-(locals.var_vtm * (-locals.var_niln_dn11))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn11))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn13))) + ((((-(locals.var_vtm * (-locals.var_niln_dn13))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn13))))) / (2.0 * assign18760_e36369)))), (0.5 * ((-(locals.var_vtm * (-locals.var_niln_dn14))) + ((((-(locals.var_vtm * (-locals.var_niln_dn14))) * assign18760_e36361) + (assign18760_e36339 * (-(locals.var_vtm * (-locals.var_niln_dn14))))) / (2.0 * assign18760_e36369)))),)
            } else {
                let assign18760_e36374: f64 = (0.5 * locals.var_eg);
                let (assign18760_e36389,) = {
                    if (!(p.p97 > 1e-38)) {
                        let assign18760_e36381: f64 = (-87.498233534);
                        (assign18760_e36381,)
                    } else {
                        let (assign18760_e36388,) = {
                            if (p.p97 > 1e-38) {
                                let assign18760_e36386: f64 = (p.p97).ln();
                                (assign18760_e36386,)
                            } else {
                                (0.0,)
                            }
                        };
                        (assign18760_e36388,)
                    }
                };
                let assign18760_e36391: f64 = (assign18760_e36389 - locals.var_niln);
                let assign18760_e36392: f64 = (locals.var_vtm * assign18760_e36391);
                let assign18760_e36393: f64 = (assign18760_e36374 - assign18760_e36392);
                let assign18760_e36395: f64 = (-10000.0);
                let assign18760_e36397: f64 = (assign18760_e36395 * 0.0001);
                let (assign18760_e36427, assign18760_e36427_d_n0, assign18760_e36427_d_n2, assign18760_e36427_d_n3, assign18760_e36427_d_n4, assign18760_e36427_d_n5, assign18760_e36427_d_n6, assign18760_e36427_d_n7, assign18760_e36427_d_n8, assign18760_e36427_d_n9, assign18760_e36427_d_n10, assign18760_e36427_d_n11, assign18760_e36427_d_n13, assign18760_e36427_d_n14,) = {
                    if (assign18760_e36393 < assign18760_e36397) {
                        let assign18760_e36400: f64 = (-0.0001);
                        let assign18760_e36402: f64 = (assign18760_e36400 * 0.0001);
                        let assign18760_e36405: f64 = (0.5 * locals.var_eg);
                        let (assign18760_e36420,) = {
                            if (!(p.p97 > 1e-38)) {
                                let assign18760_e36412: f64 = (-87.498233534);
                                (assign18760_e36412,)
                            } else {
                                let (assign18760_e36419,) = {
                                    if (p.p97 > 1e-38) {
                                        let assign18760_e36417: f64 = (p.p97).ln();
                                        (assign18760_e36417,)
                                    } else {
                                        (0.0,)
                                    }
                                };
                                (assign18760_e36419,)
                            }
                        };
                        let assign18760_e36422: f64 = (assign18760_e36420 - locals.var_niln);
                        let assign18760_e36423: f64 = (locals.var_vtm * assign18760_e36422);
                        let assign18760_e36424: f64 = (assign18760_e36405 - assign18760_e36423);
                        let assign18760_e36425: f64 = (assign18760_e36402 / assign18760_e36424);
                        (assign18760_e36425, (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn0)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn2)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn3)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * ((0.5 * locals.var_eg_dn4) - ((locals.var_vtm_dn4 * assign18760_e36422) + (locals.var_vtm * (-locals.var_niln_dn4))))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn5)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn6)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn7)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn8)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn9)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn10)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn11)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn13)))) / (assign18760_e36424 * assign18760_e36424))), (-((assign18760_e36402 * (-(locals.var_vtm * (-locals.var_niln_dn14)))) / (assign18760_e36424 * assign18760_e36424))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18760_e36427, assign18760_e36427_d_n0, assign18760_e36427_d_n2, assign18760_e36427_d_n3, assign18760_e36427_d_n4, assign18760_e36427_d_n5, assign18760_e36427_d_n6, assign18760_e36427_d_n7, assign18760_e36427_d_n8, assign18760_e36427_d_n9, assign18760_e36427_d_n10, assign18760_e36427_d_n11, assign18760_e36427_d_n13, assign18760_e36427_d_n14,)
            }
        };
        let assign18760_e36429: f64 = (assign18760_e36266 - assign18760_e36428);
        let assign18760_e36430: f64 = (locals.var_devsign * assign18760_e36429);
        let assign18760_e36431: f64 = (assign18760_e36262 - assign18760_e36430);
        let assign18760_e36432: f64 = (locals.var_phig_i - assign18760_e36431);
        let assign18760_e36433: f64 = (locals.var_devsign * assign18760_e36432);
        (assign18760_e36433, (locals.var_devsign * (locals.var_phig_i_dn0 - (-(locals.var_devsign * (-assign18760_e36428_d_n0))))), (locals.var_devsign * (locals.var_phig_i_dn2 - (-(locals.var_devsign * (-assign18760_e36428_d_n2))))), (locals.var_devsign * (locals.var_phig_i_dn3 - (-(locals.var_devsign * (-assign18760_e36428_d_n3))))), (locals.var_devsign * (locals.var_phig_i_dn4 - ((0.5 * locals.var_eg_dn4) - (locals.var_devsign * ((0.5 * locals.var_eg_dn4) - assign18760_e36428_d_n4))))), (locals.var_devsign * (locals.var_phig_i_dn5 - (-(locals.var_devsign * (-assign18760_e36428_d_n5))))), (locals.var_devsign * (locals.var_phig_i_dn6 - (-(locals.var_devsign * (-assign18760_e36428_d_n6))))), (locals.var_devsign * (locals.var_phig_i_dn7 - (-(locals.var_devsign * (-assign18760_e36428_d_n7))))), (locals.var_devsign * (locals.var_phig_i_dn8 - (-(locals.var_devsign * (-assign18760_e36428_d_n8))))), (locals.var_devsign * (locals.var_phig_i_dn9 - (-(locals.var_devsign * (-assign18760_e36428_d_n9))))), (locals.var_devsign * (locals.var_phig_i_dn10 - (-(locals.var_devsign * (-assign18760_e36428_d_n10))))), (locals.var_devsign * (locals.var_phig_i_dn11 - (-(locals.var_devsign * (-assign18760_e36428_d_n11))))), (locals.var_devsign * (locals.var_phig_i_dn13 - (-(locals.var_devsign * (-assign18760_e36428_d_n13))))), (locals.var_devsign * (locals.var_phig_i_dn14 - (-(locals.var_devsign * (-assign18760_e36428_d_n14))))),)
    } else {
        (locals.var_vfbsd_v, locals.var_vfbsd_v_dn0, locals.var_vfbsd_v_dn2, locals.var_vfbsd_v_dn3, locals.var_vfbsd_v_dn4, locals.var_vfbsd_v_dn5, locals.var_vfbsd_v_dn6, locals.var_vfbsd_v_dn7, locals.var_vfbsd_v_dn8, locals.var_vfbsd_v_dn9, locals.var_vfbsd_v_dn10, locals.var_vfbsd_v_dn11, locals.var_vfbsd_v_dn13, locals.var_vfbsd_v_dn14,)
    }
};
        locals.var_vfbsd_v = assign18760_e36435;
        locals.var_vfbsd_v_dn0 = assign18760_e36435_d_n0;
        locals.var_vfbsd_v_dn2 = assign18760_e36435_d_n2;
        locals.var_vfbsd_v_dn3 = assign18760_e36435_d_n3;
        locals.var_vfbsd_v_dn4 = assign18760_e36435_d_n4;
        locals.var_vfbsd_v_dn5 = assign18760_e36435_d_n5;
        locals.var_vfbsd_v_dn6 = assign18760_e36435_d_n6;
        locals.var_vfbsd_v_dn7 = assign18760_e36435_d_n7;
        locals.var_vfbsd_v_dn8 = assign18760_e36435_d_n8;
        locals.var_vfbsd_v_dn9 = assign18760_e36435_d_n9;
        locals.var_vfbsd_v_dn10 = assign18760_e36435_d_n10;
        locals.var_vfbsd_v_dn11 = assign18760_e36435_d_n11;
        locals.var_vfbsd_v_dn13 = assign18760_e36435_d_n13;
        locals.var_vfbsd_v_dn14 = assign18760_e36435_d_n14;

        let (assign18770_e36440, assign18770_e36440_d_n0, assign18770_e36440_d_n2, assign18770_e36440_d_n3, assign18770_e36440_d_n4, assign18770_e36440_d_n5, assign18770_e36440_d_n6, assign18770_e36440_d_n7, assign18770_e36440_d_n8, assign18770_e36440_d_n9, assign18770_e36440_d_n10, assign18770_e36440_d_n11, assign18770_e36440_d_n13, assign18770_e36440_d_n14,) = {
    if (locals.var_guard343 == 0.0) {
        (p.p1106, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbsd_v, locals.var_vfbsd_v_dn0, locals.var_vfbsd_v_dn2, locals.var_vfbsd_v_dn3, locals.var_vfbsd_v_dn4, locals.var_vfbsd_v_dn5, locals.var_vfbsd_v_dn6, locals.var_vfbsd_v_dn7, locals.var_vfbsd_v_dn8, locals.var_vfbsd_v_dn9, locals.var_vfbsd_v_dn10, locals.var_vfbsd_v_dn11, locals.var_vfbsd_v_dn13, locals.var_vfbsd_v_dn14,)
    }
};
        locals.var_vfbsd_v = assign18770_e36440;
        locals.var_vfbsd_v_dn0 = assign18770_e36440_d_n0;
        locals.var_vfbsd_v_dn2 = assign18770_e36440_d_n2;
        locals.var_vfbsd_v_dn3 = assign18770_e36440_d_n3;
        locals.var_vfbsd_v_dn4 = assign18770_e36440_d_n4;
        locals.var_vfbsd_v_dn5 = assign18770_e36440_d_n5;
        locals.var_vfbsd_v_dn6 = assign18770_e36440_d_n6;
        locals.var_vfbsd_v_dn7 = assign18770_e36440_d_n7;
        locals.var_vfbsd_v_dn8 = assign18770_e36440_d_n8;
        locals.var_vfbsd_v_dn9 = assign18770_e36440_d_n9;
        locals.var_vfbsd_v_dn10 = assign18770_e36440_d_n10;
        locals.var_vfbsd_v_dn11 = assign18770_e36440_d_n11;
        locals.var_vfbsd_v_dn13 = assign18770_e36440_d_n13;
        locals.var_vfbsd_v_dn14 = assign18770_e36440_d_n14;

        let assign18780_e36443: f64 = if (!param_given[1107]) { 1.0 } else { 0.0 };
        locals.var_guard347 = assign18780_e36443;

        let (assign18790_e36447, assign18790_e36447_d_n0, assign18790_e36447_d_n2, assign18790_e36447_d_n3, assign18790_e36447_d_n4, assign18790_e36447_d_n5, assign18790_e36447_d_n6, assign18790_e36447_d_n7, assign18790_e36447_d_n8, assign18790_e36447_d_n9, assign18790_e36447_d_n10, assign18790_e36447_d_n11, assign18790_e36447_d_n13, assign18790_e36447_d_n14,) = {
    if (locals.var_guard347 != 0.0) {
        (locals.var_vfbsd_v, locals.var_vfbsd_v_dn0, locals.var_vfbsd_v_dn2, locals.var_vfbsd_v_dn3, locals.var_vfbsd_v_dn4, locals.var_vfbsd_v_dn5, locals.var_vfbsd_v_dn6, locals.var_vfbsd_v_dn7, locals.var_vfbsd_v_dn8, locals.var_vfbsd_v_dn9, locals.var_vfbsd_v_dn10, locals.var_vfbsd_v_dn11, locals.var_vfbsd_v_dn13, locals.var_vfbsd_v_dn14,)
    } else {
        (locals.var_vfbsdcv_v, locals.var_vfbsdcv_v_dn0, locals.var_vfbsdcv_v_dn2, locals.var_vfbsdcv_v_dn3, locals.var_vfbsdcv_v_dn4, locals.var_vfbsdcv_v_dn5, locals.var_vfbsdcv_v_dn6, locals.var_vfbsdcv_v_dn7, locals.var_vfbsdcv_v_dn8, locals.var_vfbsdcv_v_dn9, locals.var_vfbsdcv_v_dn10, locals.var_vfbsdcv_v_dn11, locals.var_vfbsdcv_v_dn13, locals.var_vfbsdcv_v_dn14,)
    }
};
        locals.var_vfbsdcv_v = assign18790_e36447;
        locals.var_vfbsdcv_v_dn0 = assign18790_e36447_d_n0;
        locals.var_vfbsdcv_v_dn2 = assign18790_e36447_d_n2;
        locals.var_vfbsdcv_v_dn3 = assign18790_e36447_d_n3;
        locals.var_vfbsdcv_v_dn4 = assign18790_e36447_d_n4;
        locals.var_vfbsdcv_v_dn5 = assign18790_e36447_d_n5;
        locals.var_vfbsdcv_v_dn6 = assign18790_e36447_d_n6;
        locals.var_vfbsdcv_v_dn7 = assign18790_e36447_d_n7;
        locals.var_vfbsdcv_v_dn8 = assign18790_e36447_d_n8;
        locals.var_vfbsdcv_v_dn9 = assign18790_e36447_d_n9;
        locals.var_vfbsdcv_v_dn10 = assign18790_e36447_d_n10;
        locals.var_vfbsdcv_v_dn11 = assign18790_e36447_d_n11;
        locals.var_vfbsdcv_v_dn13 = assign18790_e36447_d_n13;
        locals.var_vfbsdcv_v_dn14 = assign18790_e36447_d_n14;

        let (assign18800_e36452, assign18800_e36452_d_n0, assign18800_e36452_d_n2, assign18800_e36452_d_n3, assign18800_e36452_d_n4, assign18800_e36452_d_n5, assign18800_e36452_d_n6, assign18800_e36452_d_n7, assign18800_e36452_d_n8, assign18800_e36452_d_n9, assign18800_e36452_d_n10, assign18800_e36452_d_n11, assign18800_e36452_d_n13, assign18800_e36452_d_n14,) = {
    if (locals.var_guard347 == 0.0) {
        (p.p1107, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbsdcv_v, locals.var_vfbsdcv_v_dn0, locals.var_vfbsdcv_v_dn2, locals.var_vfbsdcv_v_dn3, locals.var_vfbsdcv_v_dn4, locals.var_vfbsdcv_v_dn5, locals.var_vfbsdcv_v_dn6, locals.var_vfbsdcv_v_dn7, locals.var_vfbsdcv_v_dn8, locals.var_vfbsdcv_v_dn9, locals.var_vfbsdcv_v_dn10, locals.var_vfbsdcv_v_dn11, locals.var_vfbsdcv_v_dn13, locals.var_vfbsdcv_v_dn14,)
    }
};
        locals.var_vfbsdcv_v = assign18800_e36452;
        locals.var_vfbsdcv_v_dn0 = assign18800_e36452_d_n0;
        locals.var_vfbsdcv_v_dn2 = assign18800_e36452_d_n2;
        locals.var_vfbsdcv_v_dn3 = assign18800_e36452_d_n3;
        locals.var_vfbsdcv_v_dn4 = assign18800_e36452_d_n4;
        locals.var_vfbsdcv_v_dn5 = assign18800_e36452_d_n5;
        locals.var_vfbsdcv_v_dn6 = assign18800_e36452_d_n6;
        locals.var_vfbsdcv_v_dn7 = assign18800_e36452_d_n7;
        locals.var_vfbsdcv_v_dn8 = assign18800_e36452_d_n8;
        locals.var_vfbsdcv_v_dn9 = assign18800_e36452_d_n9;
        locals.var_vfbsdcv_v_dn10 = assign18800_e36452_d_n10;
        locals.var_vfbsdcv_v_dn11 = assign18800_e36452_d_n11;
        locals.var_vfbsdcv_v_dn13 = assign18800_e36452_d_n13;
        locals.var_vfbsdcv_v_dn14 = assign18800_e36452_d_n14;

        let assign18810_e36455: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard348 = assign18810_e36455;

        let (assign18820_e36480, assign18820_e36480_d_n0, assign18820_e36480_d_n2, assign18820_e36480_d_n3, assign18820_e36480_d_n4, assign18820_e36480_d_n5, assign18820_e36480_d_n6, assign18820_e36480_d_n7, assign18820_e36480_d_n8, assign18820_e36480_d_n9, assign18820_e36480_d_n10, assign18820_e36480_d_n11, assign18820_e36480_d_n13, assign18820_e36480_d_n14,) = {
    if (locals.var_guard348 != 0.0) {
        let assign18820_e36460: f64 = (locals.var_nbody_i / locals.var_ni);
        let (assign18820_e36477, assign18820_e36477_d_n0, assign18820_e36477_d_n2, assign18820_e36477_d_n3, assign18820_e36477_d_n4, assign18820_e36477_d_n5, assign18820_e36477_d_n6, assign18820_e36477_d_n7, assign18820_e36477_d_n8, assign18820_e36477_d_n9, assign18820_e36477_d_n10, assign18820_e36477_d_n11, assign18820_e36477_d_n13, assign18820_e36477_d_n14,) = {
            if (!(assign18820_e36460 > 1e-38)) {
                let assign18820_e36465: f64 = (-87.498233534);
                (assign18820_e36465, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18820_e36468: f64 = (locals.var_nbody_i / locals.var_ni);
                let (assign18820_e36476, assign18820_e36476_d_n0, assign18820_e36476_d_n2, assign18820_e36476_d_n3, assign18820_e36476_d_n4, assign18820_e36476_d_n5, assign18820_e36476_d_n6, assign18820_e36476_d_n7, assign18820_e36476_d_n8, assign18820_e36476_d_n9, assign18820_e36476_d_n10, assign18820_e36476_d_n11, assign18820_e36476_d_n13, assign18820_e36476_d_n14,) = {
                    if (assign18820_e36468 > 1e-38) {
                        let assign18820_e36473: f64 = (locals.var_nbody_i / locals.var_ni);
                        let assign18820_e36474: f64 = (assign18820_e36473).ln();
                        (assign18820_e36474, ((-((locals.var_nbody_i * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign18820_e36473), ((-((locals.var_nbody_i * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign18820_e36473),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18820_e36476, assign18820_e36476_d_n0, assign18820_e36476_d_n2, assign18820_e36476_d_n3, assign18820_e36476_d_n4, assign18820_e36476_d_n5, assign18820_e36476_d_n6, assign18820_e36476_d_n7, assign18820_e36476_d_n8, assign18820_e36476_d_n9, assign18820_e36476_d_n10, assign18820_e36476_d_n11, assign18820_e36476_d_n13, assign18820_e36476_d_n14,)
            }
        };
        let assign18820_e36478: f64 = (locals.var_vtm * assign18820_e36477);
        (assign18820_e36478, (locals.var_vtm * assign18820_e36477_d_n0), (locals.var_vtm * assign18820_e36477_d_n2), (locals.var_vtm * assign18820_e36477_d_n3), ((locals.var_vtm_dn4 * assign18820_e36477) + (locals.var_vtm * assign18820_e36477_d_n4)), (locals.var_vtm * assign18820_e36477_d_n5), (locals.var_vtm * assign18820_e36477_d_n6), (locals.var_vtm * assign18820_e36477_d_n7), (locals.var_vtm * assign18820_e36477_d_n8), (locals.var_vtm * assign18820_e36477_d_n9), (locals.var_vtm * assign18820_e36477_d_n10), (locals.var_vtm * assign18820_e36477_d_n11), (locals.var_vtm * assign18820_e36477_d_n13), (locals.var_vtm * assign18820_e36477_d_n14),)
    } else {
        (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn13, locals.var_phib_dn14,)
    }
};
        locals.var_phib = assign18820_e36480;
        locals.var_phib_dn0 = assign18820_e36480_d_n0;
        locals.var_phib_dn2 = assign18820_e36480_d_n2;
        locals.var_phib_dn3 = assign18820_e36480_d_n3;
        locals.var_phib_dn4 = assign18820_e36480_d_n4;
        locals.var_phib_dn5 = assign18820_e36480_d_n5;
        locals.var_phib_dn6 = assign18820_e36480_d_n6;
        locals.var_phib_dn7 = assign18820_e36480_d_n7;
        locals.var_phib_dn8 = assign18820_e36480_d_n8;
        locals.var_phib_dn9 = assign18820_e36480_d_n9;
        locals.var_phib_dn10 = assign18820_e36480_d_n10;
        locals.var_phib_dn11 = assign18820_e36480_d_n11;
        locals.var_phib_dn13 = assign18820_e36480_d_n13;
        locals.var_phib_dn14 = assign18820_e36480_d_n14;

    }

    pub(super) fn stamp_transient_block_68(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18830_e36503, assign18830_e36503_d_n0, assign18830_e36503_d_n2, assign18830_e36503_d_n3, assign18830_e36503_d_n4, assign18830_e36503_d_n5, assign18830_e36503_d_n6, assign18830_e36503_d_n7, assign18830_e36503_d_n8, assign18830_e36503_d_n9, assign18830_e36503_d_n10, assign18830_e36503_d_n11, assign18830_e36503_d_n13, assign18830_e36503_d_n14,) = {
    if (locals.var_guard348 != 0.0) {
        let assign18830_e36485: f64 = locals.var_phib;
        let assign18830_e36488: f64 = locals.var_phib;
        let assign18830_e36491: f64 = locals.var_phib;
        let assign18830_e36492: f64 = (assign18830_e36488 * assign18830_e36491);
        let assign18830_e36495: f64 = (0.25 * 1e-10);
        let assign18830_e36497: f64 = (assign18830_e36495 * 1e-10);
        let assign18830_e36498: f64 = (assign18830_e36492 + assign18830_e36497);
        let assign18830_e36499: f64 = (assign18830_e36498).sqrt();
        let assign18830_e36500: f64 = (assign18830_e36485 + assign18830_e36499);
        let assign18830_e36501: f64 = (0.5 * assign18830_e36500);
        (assign18830_e36501, (0.5 * (locals.var_phib_dn0 + (((locals.var_phib_dn0 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn0)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn2 + (((locals.var_phib_dn2 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn2)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn3 + (((locals.var_phib_dn3 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn3)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn4 + (((locals.var_phib_dn4 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn4)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn5 + (((locals.var_phib_dn5 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn5)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn6 + (((locals.var_phib_dn6 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn6)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn7 + (((locals.var_phib_dn7 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn7)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn8 + (((locals.var_phib_dn8 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn8)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn9 + (((locals.var_phib_dn9 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn9)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn10 + (((locals.var_phib_dn10 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn10)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn11 + (((locals.var_phib_dn11 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn11)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn13 + (((locals.var_phib_dn13 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn13)) / (2.0 * assign18830_e36499)))), (0.5 * (locals.var_phib_dn14 + (((locals.var_phib_dn14 * assign18830_e36491) + (assign18830_e36488 * locals.var_phib_dn14)) / (2.0 * assign18830_e36499)))),)
    } else {
        (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn13, locals.var_phib_dn14,)
    }
};
        locals.var_phib = assign18830_e36503;
        locals.var_phib_dn0 = assign18830_e36503_d_n0;
        locals.var_phib_dn2 = assign18830_e36503_d_n2;
        locals.var_phib_dn3 = assign18830_e36503_d_n3;
        locals.var_phib_dn4 = assign18830_e36503_d_n4;
        locals.var_phib_dn5 = assign18830_e36503_d_n5;
        locals.var_phib_dn6 = assign18830_e36503_d_n6;
        locals.var_phib_dn7 = assign18830_e36503_d_n7;
        locals.var_phib_dn8 = assign18830_e36503_d_n8;
        locals.var_phib_dn9 = assign18830_e36503_d_n9;
        locals.var_phib_dn10 = assign18830_e36503_d_n10;
        locals.var_phib_dn11 = assign18830_e36503_d_n11;
        locals.var_phib_dn13 = assign18830_e36503_d_n13;
        locals.var_phib_dn14 = assign18830_e36503_d_n14;

        let (assign18840_e36540, assign18840_e36540_d_n0, assign18840_e36540_d_n2, assign18840_e36540_d_n3, assign18840_e36540_d_n4, assign18840_e36540_d_n5, assign18840_e36540_d_n6, assign18840_e36540_d_n7, assign18840_e36540_d_n8, assign18840_e36540_d_n9, assign18840_e36540_d_n10, assign18840_e36540_d_n11, assign18840_e36540_d_n13, assign18840_e36540_d_n14,) = {
    if (locals.var_guard348 != 0.0) {
        let assign18840_e36508: f64 = (locals.var_nbody_i * p.p97);
        let assign18840_e36511: f64 = (locals.var_ni * locals.var_ni);
        let assign18840_e36512: f64 = (assign18840_e36508 / assign18840_e36511);
        let (assign18840_e36537, assign18840_e36537_d_n0, assign18840_e36537_d_n2, assign18840_e36537_d_n3, assign18840_e36537_d_n4, assign18840_e36537_d_n5, assign18840_e36537_d_n6, assign18840_e36537_d_n7, assign18840_e36537_d_n8, assign18840_e36537_d_n9, assign18840_e36537_d_n10, assign18840_e36537_d_n11, assign18840_e36537_d_n13, assign18840_e36537_d_n14,) = {
            if (!(assign18840_e36512 > 1e-38)) {
                let assign18840_e36517: f64 = (-87.498233534);
                (assign18840_e36517, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign18840_e36520: f64 = (locals.var_nbody_i * p.p97);
                let assign18840_e36523: f64 = (locals.var_ni * locals.var_ni);
                let assign18840_e36524: f64 = (assign18840_e36520 / assign18840_e36523);
                let (assign18840_e36536, assign18840_e36536_d_n0, assign18840_e36536_d_n2, assign18840_e36536_d_n3, assign18840_e36536_d_n4, assign18840_e36536_d_n5, assign18840_e36536_d_n6, assign18840_e36536_d_n7, assign18840_e36536_d_n8, assign18840_e36536_d_n9, assign18840_e36536_d_n10, assign18840_e36536_d_n11, assign18840_e36536_d_n13, assign18840_e36536_d_n14,) = {
                    if (assign18840_e36524 > 1e-38) {
                        let assign18840_e36529: f64 = (locals.var_nbody_i * p.p97);
                        let assign18840_e36532: f64 = (locals.var_ni * locals.var_ni);
                        let assign18840_e36533: f64 = (assign18840_e36529 / assign18840_e36532);
                        let assign18840_e36534: f64 = (assign18840_e36533).ln();
                        (assign18840_e36534, ((-((assign18840_e36529 * ((locals.var_ni_dn0 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn0))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn2 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn2))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn13 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn13))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533), ((-((assign18840_e36529 * ((locals.var_ni_dn14 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn14))) / (assign18840_e36532 * assign18840_e36532))) / assign18840_e36533),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign18840_e36536, assign18840_e36536_d_n0, assign18840_e36536_d_n2, assign18840_e36536_d_n3, assign18840_e36536_d_n4, assign18840_e36536_d_n5, assign18840_e36536_d_n6, assign18840_e36536_d_n7, assign18840_e36536_d_n8, assign18840_e36536_d_n9, assign18840_e36536_d_n10, assign18840_e36536_d_n11, assign18840_e36536_d_n13, assign18840_e36536_d_n14,)
            }
        };
        let assign18840_e36538: f64 = (locals.var_vtm * assign18840_e36537);
        (assign18840_e36538, (locals.var_vtm * assign18840_e36537_d_n0), (locals.var_vtm * assign18840_e36537_d_n2), (locals.var_vtm * assign18840_e36537_d_n3), ((locals.var_vtm_dn4 * assign18840_e36537) + (locals.var_vtm * assign18840_e36537_d_n4)), (locals.var_vtm * assign18840_e36537_d_n5), (locals.var_vtm * assign18840_e36537_d_n6), (locals.var_vtm * assign18840_e36537_d_n7), (locals.var_vtm * assign18840_e36537_d_n8), (locals.var_vtm * assign18840_e36537_d_n9), (locals.var_vtm * assign18840_e36537_d_n10), (locals.var_vtm * assign18840_e36537_d_n11), (locals.var_vtm * assign18840_e36537_d_n13), (locals.var_vtm * assign18840_e36537_d_n14),)
    } else {
        (locals.var_vbi, locals.var_vbi_dn0, locals.var_vbi_dn2, locals.var_vbi_dn3, locals.var_vbi_dn4, locals.var_vbi_dn5, locals.var_vbi_dn6, locals.var_vbi_dn7, locals.var_vbi_dn8, locals.var_vbi_dn9, locals.var_vbi_dn10, locals.var_vbi_dn11, locals.var_vbi_dn13, locals.var_vbi_dn14,)
    }
};
        locals.var_vbi = assign18840_e36540;
        locals.var_vbi_dn0 = assign18840_e36540_d_n0;
        locals.var_vbi_dn2 = assign18840_e36540_d_n2;
        locals.var_vbi_dn3 = assign18840_e36540_d_n3;
        locals.var_vbi_dn4 = assign18840_e36540_d_n4;
        locals.var_vbi_dn5 = assign18840_e36540_d_n5;
        locals.var_vbi_dn6 = assign18840_e36540_d_n6;
        locals.var_vbi_dn7 = assign18840_e36540_d_n7;
        locals.var_vbi_dn8 = assign18840_e36540_d_n8;
        locals.var_vbi_dn9 = assign18840_e36540_d_n9;
        locals.var_vbi_dn10 = assign18840_e36540_d_n10;
        locals.var_vbi_dn11 = assign18840_e36540_d_n11;
        locals.var_vbi_dn13 = assign18840_e36540_d_n13;
        locals.var_vbi_dn14 = assign18840_e36540_d_n14;

        let (assign18850_e36562, assign18850_e36562_d_n0, assign18850_e36562_d_n2, assign18850_e36562_d_n3, assign18850_e36562_d_n4, assign18850_e36562_d_n5, assign18850_e36562_d_n6, assign18850_e36562_d_n7, assign18850_e36562_d_n8, assign18850_e36562_d_n9, assign18850_e36562_d_n10, assign18850_e36562_d_n11, assign18850_e36562_d_n13, assign18850_e36562_d_n14,) = {
    if (locals.var_guard348 == 0.0) {
        let (assign18850_e36557,) = {
            if (!(locals.var_nbody_i > 1e-38)) {
                let assign18850_e36549: f64 = (-87.498233534);
                (assign18850_e36549,)
            } else {
                let (assign18850_e36556,) = {
                    if (locals.var_nbody_i > 1e-38) {
                        let assign18850_e36554: f64 = (locals.var_nbody_i).ln();
                        (assign18850_e36554,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18850_e36556,)
            }
        };
        let assign18850_e36559: f64 = (assign18850_e36557 - locals.var_niln);
        let assign18850_e36560: f64 = (locals.var_vtm * assign18850_e36559);
        (assign18850_e36560, (locals.var_vtm * (-locals.var_niln_dn0)), (locals.var_vtm * (-locals.var_niln_dn2)), (locals.var_vtm * (-locals.var_niln_dn3)), ((locals.var_vtm_dn4 * assign18850_e36559) + (locals.var_vtm * (-locals.var_niln_dn4))), (locals.var_vtm * (-locals.var_niln_dn5)), (locals.var_vtm * (-locals.var_niln_dn6)), (locals.var_vtm * (-locals.var_niln_dn7)), (locals.var_vtm * (-locals.var_niln_dn8)), (locals.var_vtm * (-locals.var_niln_dn9)), (locals.var_vtm * (-locals.var_niln_dn10)), (locals.var_vtm * (-locals.var_niln_dn11)), (locals.var_vtm * (-locals.var_niln_dn13)), (locals.var_vtm * (-locals.var_niln_dn14)),)
    } else {
        (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn13, locals.var_phib_dn14,)
    }
};
        locals.var_phib = assign18850_e36562;
        locals.var_phib_dn0 = assign18850_e36562_d_n0;
        locals.var_phib_dn2 = assign18850_e36562_d_n2;
        locals.var_phib_dn3 = assign18850_e36562_d_n3;
        locals.var_phib_dn4 = assign18850_e36562_d_n4;
        locals.var_phib_dn5 = assign18850_e36562_d_n5;
        locals.var_phib_dn6 = assign18850_e36562_d_n6;
        locals.var_phib_dn7 = assign18850_e36562_d_n7;
        locals.var_phib_dn8 = assign18850_e36562_d_n8;
        locals.var_phib_dn9 = assign18850_e36562_d_n9;
        locals.var_phib_dn10 = assign18850_e36562_d_n10;
        locals.var_phib_dn11 = assign18850_e36562_d_n11;
        locals.var_phib_dn13 = assign18850_e36562_d_n13;
        locals.var_phib_dn14 = assign18850_e36562_d_n14;

        let (assign18860_e36586, assign18860_e36586_d_n0, assign18860_e36586_d_n2, assign18860_e36586_d_n3, assign18860_e36586_d_n4, assign18860_e36586_d_n5, assign18860_e36586_d_n6, assign18860_e36586_d_n7, assign18860_e36586_d_n8, assign18860_e36586_d_n9, assign18860_e36586_d_n10, assign18860_e36586_d_n11, assign18860_e36586_d_n13, assign18860_e36586_d_n14,) = {
    if (locals.var_guard348 == 0.0) {
        let assign18860_e36568: f64 = locals.var_phib;
        let assign18860_e36571: f64 = locals.var_phib;
        let assign18860_e36574: f64 = locals.var_phib;
        let assign18860_e36575: f64 = (assign18860_e36571 * assign18860_e36574);
        let assign18860_e36578: f64 = (0.25 * 1e-10);
        let assign18860_e36580: f64 = (assign18860_e36578 * 1e-10);
        let assign18860_e36581: f64 = (assign18860_e36575 + assign18860_e36580);
        let assign18860_e36582: f64 = (assign18860_e36581).sqrt();
        let assign18860_e36583: f64 = (assign18860_e36568 + assign18860_e36582);
        let assign18860_e36584: f64 = (0.5 * assign18860_e36583);
        (assign18860_e36584, (0.5 * (locals.var_phib_dn0 + (((locals.var_phib_dn0 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn0)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn2 + (((locals.var_phib_dn2 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn2)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn3 + (((locals.var_phib_dn3 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn3)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn4 + (((locals.var_phib_dn4 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn4)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn5 + (((locals.var_phib_dn5 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn5)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn6 + (((locals.var_phib_dn6 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn6)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn7 + (((locals.var_phib_dn7 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn7)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn8 + (((locals.var_phib_dn8 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn8)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn9 + (((locals.var_phib_dn9 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn9)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn10 + (((locals.var_phib_dn10 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn10)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn11 + (((locals.var_phib_dn11 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn11)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn13 + (((locals.var_phib_dn13 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn13)) / (2.0 * assign18860_e36582)))), (0.5 * (locals.var_phib_dn14 + (((locals.var_phib_dn14 * assign18860_e36574) + (assign18860_e36571 * locals.var_phib_dn14)) / (2.0 * assign18860_e36582)))),)
    } else {
        (locals.var_phib, locals.var_phib_dn0, locals.var_phib_dn2, locals.var_phib_dn3, locals.var_phib_dn4, locals.var_phib_dn5, locals.var_phib_dn6, locals.var_phib_dn7, locals.var_phib_dn8, locals.var_phib_dn9, locals.var_phib_dn10, locals.var_phib_dn11, locals.var_phib_dn13, locals.var_phib_dn14,)
    }
};
        locals.var_phib = assign18860_e36586;
        locals.var_phib_dn0 = assign18860_e36586_d_n0;
        locals.var_phib_dn2 = assign18860_e36586_d_n2;
        locals.var_phib_dn3 = assign18860_e36586_d_n3;
        locals.var_phib_dn4 = assign18860_e36586_d_n4;
        locals.var_phib_dn5 = assign18860_e36586_d_n5;
        locals.var_phib_dn6 = assign18860_e36586_d_n6;
        locals.var_phib_dn7 = assign18860_e36586_d_n7;
        locals.var_phib_dn8 = assign18860_e36586_d_n8;
        locals.var_phib_dn9 = assign18860_e36586_d_n9;
        locals.var_phib_dn10 = assign18860_e36586_d_n10;
        locals.var_phib_dn11 = assign18860_e36586_d_n11;
        locals.var_phib_dn13 = assign18860_e36586_d_n13;
        locals.var_phib_dn14 = assign18860_e36586_d_n14;

        let (assign18870_e36616, assign18870_e36616_d_n0, assign18870_e36616_d_n2, assign18870_e36616_d_n3, assign18870_e36616_d_n4, assign18870_e36616_d_n5, assign18870_e36616_d_n6, assign18870_e36616_d_n7, assign18870_e36616_d_n8, assign18870_e36616_d_n9, assign18870_e36616_d_n10, assign18870_e36616_d_n11, assign18870_e36616_d_n13, assign18870_e36616_d_n14,) = {
    if (locals.var_guard348 == 0.0) {
        let assign18870_e36592: f64 = (locals.var_nbody_i * p.p97);
        let (assign18870_e36609,) = {
            if (!(assign18870_e36592 > 1e-38)) {
                let assign18870_e36597: f64 = (-87.498233534);
                (assign18870_e36597,)
            } else {
                let assign18870_e36600: f64 = (locals.var_nbody_i * p.p97);
                let (assign18870_e36608,) = {
                    if (assign18870_e36600 > 1e-38) {
                        let assign18870_e36605: f64 = (locals.var_nbody_i * p.p97);
                        let assign18870_e36606: f64 = (assign18870_e36605).ln();
                        (assign18870_e36606,)
                    } else {
                        (0.0,)
                    }
                };
                (assign18870_e36608,)
            }
        };
        let assign18870_e36612: f64 = (2.0 * locals.var_niln);
        let assign18870_e36613: f64 = (assign18870_e36609 - assign18870_e36612);
        let assign18870_e36614: f64 = (locals.var_vtm * assign18870_e36613);
        (assign18870_e36614, (locals.var_vtm * (-(2.0 * locals.var_niln_dn0))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn2))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn3))), ((locals.var_vtm_dn4 * assign18870_e36613) + (locals.var_vtm * (-(2.0 * locals.var_niln_dn4)))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn5))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn6))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn7))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn8))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn9))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn10))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn11))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn13))), (locals.var_vtm * (-(2.0 * locals.var_niln_dn14))),)
    } else {
        (locals.var_vbi, locals.var_vbi_dn0, locals.var_vbi_dn2, locals.var_vbi_dn3, locals.var_vbi_dn4, locals.var_vbi_dn5, locals.var_vbi_dn6, locals.var_vbi_dn7, locals.var_vbi_dn8, locals.var_vbi_dn9, locals.var_vbi_dn10, locals.var_vbi_dn11, locals.var_vbi_dn13, locals.var_vbi_dn14,)
    }
};
        locals.var_vbi = assign18870_e36616;
        locals.var_vbi_dn0 = assign18870_e36616_d_n0;
        locals.var_vbi_dn2 = assign18870_e36616_d_n2;
        locals.var_vbi_dn3 = assign18870_e36616_d_n3;
        locals.var_vbi_dn4 = assign18870_e36616_d_n4;
        locals.var_vbi_dn5 = assign18870_e36616_d_n5;
        locals.var_vbi_dn6 = assign18870_e36616_d_n6;
        locals.var_vbi_dn7 = assign18870_e36616_d_n7;
        locals.var_vbi_dn8 = assign18870_e36616_d_n8;
        locals.var_vbi_dn9 = assign18870_e36616_d_n9;
        locals.var_vbi_dn10 = assign18870_e36616_d_n10;
        locals.var_vbi_dn11 = assign18870_e36616_d_n11;
        locals.var_vbi_dn13 = assign18870_e36616_d_n13;
        locals.var_vbi_dn14 = assign18870_e36616_d_n14;

        let (assign18880_e36625, assign18880_e36625_d_n4,) = {
    if (p.p60 == 1.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_eg, locals.var_eg_dn4,)
    }
};
        let assign18880_e36626: f64 = (p.p104 + assign18880_e36625);
        let assign18880_e36627: f64 = (locals.var_phig_i - assign18880_e36626);
        let assign18880_e36628: f64 = (locals.var_devsign * assign18880_e36627);
        locals.var_deltaphi = assign18880_e36628;
        locals.var_deltaphi_dn0 = (locals.var_devsign * locals.var_phig_i_dn0);
        locals.var_deltaphi_dn2 = (locals.var_devsign * locals.var_phig_i_dn2);
        locals.var_deltaphi_dn3 = (locals.var_devsign * locals.var_phig_i_dn3);
        locals.var_deltaphi_dn4 = (locals.var_devsign * (locals.var_phig_i_dn4 - assign18880_e36625_d_n4));
        locals.var_deltaphi_dn5 = (locals.var_devsign * locals.var_phig_i_dn5);
        locals.var_deltaphi_dn6 = (locals.var_devsign * locals.var_phig_i_dn6);
        locals.var_deltaphi_dn7 = (locals.var_devsign * locals.var_phig_i_dn7);
        locals.var_deltaphi_dn8 = (locals.var_devsign * locals.var_phig_i_dn8);
        locals.var_deltaphi_dn9 = (locals.var_devsign * locals.var_phig_i_dn9);
        locals.var_deltaphi_dn10 = (locals.var_devsign * locals.var_phig_i_dn10);
        locals.var_deltaphi_dn11 = (locals.var_devsign * locals.var_phig_i_dn11);
        locals.var_deltaphi_dn13 = (locals.var_devsign * locals.var_phig_i_dn13);
        locals.var_deltaphi_dn14 = (locals.var_devsign * locals.var_phig_i_dn14);

        let assign18890_e36631: f64 = (0.5 * locals.var_etamob_t);
        locals.var_eta_mu = assign18890_e36631;
        locals.var_eta_mu_dn4 = (0.5 * locals.var_etamob_t_dn4);

        locals.var_eta_mu_cv = 0.5;

        let assign18910_e36635: f64 = if p.p60 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard349 = assign18910_e36635;

        let (assign18920_e36641, assign18920_e36641_d_n4,) = {
    if (locals.var_guard349 != 0.0) {
        let assign18920_e36639: f64 = (0.333333333 * locals.var_etamob_t);
        (assign18920_e36639, (0.333333333 * locals.var_etamob_t_dn4),)
    } else {
        (locals.var_eta_mu, locals.var_eta_mu_dn4,)
    }
};
        locals.var_eta_mu = assign18920_e36641;
        locals.var_eta_mu_dn4 = assign18920_e36641_d_n4;

        let (assign18930_e36645,) = {
    if (locals.var_guard349 != 0.0) {
        (0.333333333,)
    } else {
        (locals.var_eta_mu_cv,)
    }
};
        locals.var_eta_mu_cv = assign18930_e36645;

        let assign18940_e36648: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard350 = assign18940_e36648;

        let (assign18950_e36664, assign18950_e36664_d_n0, assign18950_e36664_d_n2, assign18950_e36664_d_n3, assign18950_e36664_d_n4, assign18950_e36664_d_n5, assign18950_e36664_d_n6, assign18950_e36664_d_n7, assign18950_e36664_d_n8, assign18950_e36664_d_n9, assign18950_e36664_d_n10, assign18950_e36664_d_n11, assign18950_e36664_d_n13, assign18950_e36664_d_n14,) = {
    if (locals.var_guard350 != 0.0) {
        let assign18950_e36652: f64 = (p.p11 * locals.var_jss_t);
        let assign18950_e36655: f64 = (p.p13 * locals.var_jsws_t);
        let assign18950_e36656: f64 = (assign18950_e36652 + assign18950_e36655);
        let assign18950_e36659: f64 = (p.p3 * locals.var_nfintotal);
        let assign18950_e36661: f64 = (assign18950_e36659 * locals.var_jswgs_t);
        let assign18950_e36662: f64 = (assign18950_e36656 + assign18950_e36661);
        (assign18950_e36662, (((p.p11 * locals.var_jss_t_dn0) + (p.p13 * locals.var_jsws_t_dn0)) + (assign18950_e36659 * locals.var_jswgs_t_dn0)), (((p.p11 * locals.var_jss_t_dn2) + (p.p13 * locals.var_jsws_t_dn2)) + (assign18950_e36659 * locals.var_jswgs_t_dn2)), (((p.p11 * locals.var_jss_t_dn3) + (p.p13 * locals.var_jsws_t_dn3)) + (assign18950_e36659 * locals.var_jswgs_t_dn3)), (((p.p11 * locals.var_jss_t_dn4) + (p.p13 * locals.var_jsws_t_dn4)) + (assign18950_e36659 * locals.var_jswgs_t_dn4)), (((p.p11 * locals.var_jss_t_dn5) + (p.p13 * locals.var_jsws_t_dn5)) + (assign18950_e36659 * locals.var_jswgs_t_dn5)), (((p.p11 * locals.var_jss_t_dn6) + (p.p13 * locals.var_jsws_t_dn6)) + (assign18950_e36659 * locals.var_jswgs_t_dn6)), (((p.p11 * locals.var_jss_t_dn7) + (p.p13 * locals.var_jsws_t_dn7)) + (assign18950_e36659 * locals.var_jswgs_t_dn7)), (((p.p11 * locals.var_jss_t_dn8) + (p.p13 * locals.var_jsws_t_dn8)) + (assign18950_e36659 * locals.var_jswgs_t_dn8)), (((p.p11 * locals.var_jss_t_dn9) + (p.p13 * locals.var_jsws_t_dn9)) + (assign18950_e36659 * locals.var_jswgs_t_dn9)), (((p.p11 * locals.var_jss_t_dn10) + (p.p13 * locals.var_jsws_t_dn10)) + (assign18950_e36659 * locals.var_jswgs_t_dn10)), (((p.p11 * locals.var_jss_t_dn11) + (p.p13 * locals.var_jsws_t_dn11)) + (assign18950_e36659 * locals.var_jswgs_t_dn11)), (((p.p11 * locals.var_jss_t_dn13) + (p.p13 * locals.var_jsws_t_dn13)) + (assign18950_e36659 * locals.var_jswgs_t_dn13)), (((p.p11 * locals.var_jss_t_dn14) + (p.p13 * locals.var_jsws_t_dn14)) + (assign18950_e36659 * locals.var_jswgs_t_dn14)),)
    } else {
        (locals.var_isbs, locals.var_isbs_dn0, locals.var_isbs_dn2, locals.var_isbs_dn3, locals.var_isbs_dn4, locals.var_isbs_dn5, locals.var_isbs_dn6, locals.var_isbs_dn7, locals.var_isbs_dn8, locals.var_isbs_dn9, locals.var_isbs_dn10, locals.var_isbs_dn11, locals.var_isbs_dn13, locals.var_isbs_dn14,)
    }
};
        locals.var_isbs = assign18950_e36664;
        locals.var_isbs_dn0 = assign18950_e36664_d_n0;
        locals.var_isbs_dn2 = assign18950_e36664_d_n2;
        locals.var_isbs_dn3 = assign18950_e36664_d_n3;
        locals.var_isbs_dn4 = assign18950_e36664_d_n4;
        locals.var_isbs_dn5 = assign18950_e36664_d_n5;
        locals.var_isbs_dn6 = assign18950_e36664_d_n6;
        locals.var_isbs_dn7 = assign18950_e36664_d_n7;
        locals.var_isbs_dn8 = assign18950_e36664_d_n8;
        locals.var_isbs_dn9 = assign18950_e36664_d_n9;
        locals.var_isbs_dn10 = assign18950_e36664_d_n10;
        locals.var_isbs_dn11 = assign18950_e36664_d_n11;
        locals.var_isbs_dn13 = assign18950_e36664_d_n13;
        locals.var_isbs_dn14 = assign18950_e36664_d_n14;

        let assign18960_e36667: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard351 = assign18960_e36667;

        let (assign18970_e36675, assign18970_e36675_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign18970_e36673: f64 = (locals.var_vtm * p.p1620);
        (assign18970_e36673, (locals.var_vtm_dn4 * p.p1620),)
    } else {
        (locals.var_nvtms, locals.var_nvtms_dn4,)
    }
};
        locals.var_nvtms = assign18970_e36675;
        locals.var_nvtms_dn4 = assign18970_e36675_d_n4;

        let (assign18980_e36687, assign18980_e36687_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign18980_e36680: f64 = (-p.p1626);
        let assign18980_e36682: f64 = (assign18980_e36680 / locals.var_nvtms);
        let assign18980_e36683: f64 = { let limited_exp_arg = assign18980_e36682; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign18980_e36685: f64 = (assign18980_e36683 * p.p1628);
        (assign18980_e36685, (({ let limited_exp_arg = assign18980_e36682; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign18980_e36680 * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms)))) * p.p1628),)
    } else {
        (locals.var_xexpbvs, locals.var_xexpbvs_dn4,)
    }
};
        locals.var_xexpbvs = assign18980_e36687;
        locals.var_xexpbvs_dn4 = assign18980_e36687_d_n4;

        let (assign18990_e36697, assign18990_e36697_d_n0, assign18990_e36697_d_n2, assign18990_e36697_d_n3, assign18990_e36697_d_n4, assign18990_e36697_d_n5, assign18990_e36697_d_n6, assign18990_e36697_d_n7, assign18990_e36697_d_n8, assign18990_e36697_d_n9, assign18990_e36697_d_n10, assign18990_e36697_d_n11, assign18990_e36697_d_n13, assign18990_e36697_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign18990_e36693: f64 = (p.p1622 / locals.var_isbs);
        let assign18990_e36695: f64 = (assign18990_e36693).max(10.0);
        (assign18990_e36695, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign18990_e36693 >= 10.0 { (-((p.p1622 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18990_e36697;
        locals.var_t2_dn0 = assign18990_e36697_d_n0;
        locals.var_t2_dn2 = assign18990_e36697_d_n2;
        locals.var_t2_dn3 = assign18990_e36697_d_n3;
        locals.var_t2_dn4 = assign18990_e36697_d_n4;
        locals.var_t2_dn5 = assign18990_e36697_d_n5;
        locals.var_t2_dn6 = assign18990_e36697_d_n6;
        locals.var_t2_dn7 = assign18990_e36697_d_n7;
        locals.var_t2_dn8 = assign18990_e36697_d_n8;
        locals.var_t2_dn9 = assign18990_e36697_d_n9;
        locals.var_t2_dn10 = assign18990_e36697_d_n10;
        locals.var_t2_dn11 = assign18990_e36697_d_n11;
        locals.var_t2_dn13 = assign18990_e36697_d_n13;
        locals.var_t2_dn14 = assign18990_e36697_d_n14;

        let (assign19000_e36707, assign19000_e36707_d_n0, assign19000_e36707_d_n2, assign19000_e36707_d_n3, assign19000_e36707_d_n4, assign19000_e36707_d_n5, assign19000_e36707_d_n6, assign19000_e36707_d_n7, assign19000_e36707_d_n8, assign19000_e36707_d_n9, assign19000_e36707_d_n10, assign19000_e36707_d_n11, assign19000_e36707_d_n13, assign19000_e36707_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19000_e36703: f64 = (1.0 + locals.var_t2);
        let assign19000_e36705: f64 = (assign19000_e36703 - locals.var_xexpbvs);
        (assign19000_e36705, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, (locals.var_t2_dn4 - locals.var_xexpbvs_dn4), locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    } else {
        (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn13, locals.var_tb_dn14,)
    }
};
        locals.var_tb = assign19000_e36707;
        locals.var_tb_dn0 = assign19000_e36707_d_n0;
        locals.var_tb_dn2 = assign19000_e36707_d_n2;
        locals.var_tb_dn3 = assign19000_e36707_d_n3;
        locals.var_tb_dn4 = assign19000_e36707_d_n4;
        locals.var_tb_dn5 = assign19000_e36707_d_n5;
        locals.var_tb_dn6 = assign19000_e36707_d_n6;
        locals.var_tb_dn7 = assign19000_e36707_d_n7;
        locals.var_tb_dn8 = assign19000_e36707_d_n8;
        locals.var_tb_dn9 = assign19000_e36707_d_n9;
        locals.var_tb_dn10 = assign19000_e36707_d_n10;
        locals.var_tb_dn11 = assign19000_e36707_d_n11;
        locals.var_tb_dn13 = assign19000_e36707_d_n13;
        locals.var_tb_dn14 = assign19000_e36707_d_n14;

        let (assign19010_e36761, assign19010_e36761_d_n0, assign19010_e36761_d_n2, assign19010_e36761_d_n3, assign19010_e36761_d_n4, assign19010_e36761_d_n5, assign19010_e36761_d_n6, assign19010_e36761_d_n7, assign19010_e36761_d_n8, assign19010_e36761_d_n9, assign19010_e36761_d_n10, assign19010_e36761_d_n11, assign19010_e36761_d_n13, assign19010_e36761_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19010_e36716: f64 = (locals.var_tb * locals.var_tb);
        let assign19010_e36719: f64 = (4.0 * locals.var_xexpbvs);
        let assign19010_e36720: f64 = (assign19010_e36716 + assign19010_e36719);
        let assign19010_e36721: f64 = (assign19010_e36720).sqrt();
        let assign19010_e36722: f64 = (locals.var_tb + assign19010_e36721);
        let assign19010_e36723: f64 = (0.5 * assign19010_e36722);
        let (assign19010_e36758, assign19010_e36758_d_n0, assign19010_e36758_d_n2, assign19010_e36758_d_n3, assign19010_e36758_d_n4, assign19010_e36758_d_n5, assign19010_e36758_d_n6, assign19010_e36758_d_n7, assign19010_e36758_d_n8, assign19010_e36758_d_n9, assign19010_e36758_d_n10, assign19010_e36758_d_n11, assign19010_e36758_d_n13, assign19010_e36758_d_n14,) = {
            if (!(assign19010_e36723 > 1e-38)) {
                let assign19010_e36728: f64 = (-87.498233534);
                (assign19010_e36728, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19010_e36733: f64 = (locals.var_tb * locals.var_tb);
                let assign19010_e36736: f64 = (4.0 * locals.var_xexpbvs);
                let assign19010_e36737: f64 = (assign19010_e36733 + assign19010_e36736);
                let assign19010_e36738: f64 = (assign19010_e36737).sqrt();
                let assign19010_e36739: f64 = (locals.var_tb + assign19010_e36738);
                let assign19010_e36740: f64 = (0.5 * assign19010_e36739);
                let (assign19010_e36757, assign19010_e36757_d_n0, assign19010_e36757_d_n2, assign19010_e36757_d_n3, assign19010_e36757_d_n4, assign19010_e36757_d_n5, assign19010_e36757_d_n6, assign19010_e36757_d_n7, assign19010_e36757_d_n8, assign19010_e36757_d_n9, assign19010_e36757_d_n10, assign19010_e36757_d_n11, assign19010_e36757_d_n13, assign19010_e36757_d_n14,) = {
                    if (assign19010_e36740 > 1e-38) {
                        let assign19010_e36747: f64 = (locals.var_tb * locals.var_tb);
                        let assign19010_e36750: f64 = (4.0 * locals.var_xexpbvs);
                        let assign19010_e36751: f64 = (assign19010_e36747 + assign19010_e36750);
                        let assign19010_e36752: f64 = (assign19010_e36751).sqrt();
                        let assign19010_e36753: f64 = (locals.var_tb + assign19010_e36752);
                        let assign19010_e36754: f64 = (0.5 * assign19010_e36753);
                        let assign19010_e36755: f64 = (assign19010_e36754).ln();
                        (assign19010_e36755, ((0.5 * (locals.var_tb_dn0 + (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn2 + (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn3 + (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) + (4.0 * locals.var_xexpbvs_dn4)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn5 + (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn6 + (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn7 + (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn8 + (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn9 + (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn10 + (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn11 + (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn13 + (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) / (2.0 * assign19010_e36752)))) / assign19010_e36754), ((0.5 * (locals.var_tb_dn14 + (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) / (2.0 * assign19010_e36752)))) / assign19010_e36754),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19010_e36757, assign19010_e36757_d_n0, assign19010_e36757_d_n2, assign19010_e36757_d_n3, assign19010_e36757_d_n4, assign19010_e36757_d_n5, assign19010_e36757_d_n6, assign19010_e36757_d_n7, assign19010_e36757_d_n8, assign19010_e36757_d_n9, assign19010_e36757_d_n10, assign19010_e36757_d_n11, assign19010_e36757_d_n13, assign19010_e36757_d_n14,)
            }
        };
        let assign19010_e36759: f64 = (locals.var_nvtms * assign19010_e36758);
        (assign19010_e36759, (locals.var_nvtms * assign19010_e36758_d_n0), (locals.var_nvtms * assign19010_e36758_d_n2), (locals.var_nvtms * assign19010_e36758_d_n3), ((locals.var_nvtms_dn4 * assign19010_e36758) + (locals.var_nvtms * assign19010_e36758_d_n4)), (locals.var_nvtms * assign19010_e36758_d_n5), (locals.var_nvtms * assign19010_e36758_d_n6), (locals.var_nvtms * assign19010_e36758_d_n7), (locals.var_nvtms * assign19010_e36758_d_n8), (locals.var_nvtms * assign19010_e36758_d_n9), (locals.var_nvtms * assign19010_e36758_d_n10), (locals.var_nvtms * assign19010_e36758_d_n11), (locals.var_nvtms * assign19010_e36758_d_n13), (locals.var_nvtms * assign19010_e36758_d_n14),)
    } else {
        (locals.var_vjsmfwd, locals.var_vjsmfwd_dn0, locals.var_vjsmfwd_dn2, locals.var_vjsmfwd_dn3, locals.var_vjsmfwd_dn4, locals.var_vjsmfwd_dn5, locals.var_vjsmfwd_dn6, locals.var_vjsmfwd_dn7, locals.var_vjsmfwd_dn8, locals.var_vjsmfwd_dn9, locals.var_vjsmfwd_dn10, locals.var_vjsmfwd_dn11, locals.var_vjsmfwd_dn13, locals.var_vjsmfwd_dn14,)
    }
};
        locals.var_vjsmfwd = assign19010_e36761;
        locals.var_vjsmfwd_dn0 = assign19010_e36761_d_n0;
        locals.var_vjsmfwd_dn2 = assign19010_e36761_d_n2;
        locals.var_vjsmfwd_dn3 = assign19010_e36761_d_n3;
        locals.var_vjsmfwd_dn4 = assign19010_e36761_d_n4;
        locals.var_vjsmfwd_dn5 = assign19010_e36761_d_n5;
        locals.var_vjsmfwd_dn6 = assign19010_e36761_d_n6;
        locals.var_vjsmfwd_dn7 = assign19010_e36761_d_n7;
        locals.var_vjsmfwd_dn8 = assign19010_e36761_d_n8;
        locals.var_vjsmfwd_dn9 = assign19010_e36761_d_n9;
        locals.var_vjsmfwd_dn10 = assign19010_e36761_d_n10;
        locals.var_vjsmfwd_dn11 = assign19010_e36761_d_n11;
        locals.var_vjsmfwd_dn13 = assign19010_e36761_d_n13;
        locals.var_vjsmfwd_dn14 = assign19010_e36761_d_n14;

        let (assign19020_e36770, assign19020_e36770_d_n0, assign19020_e36770_d_n2, assign19020_e36770_d_n3, assign19020_e36770_d_n4, assign19020_e36770_d_n5, assign19020_e36770_d_n6, assign19020_e36770_d_n7, assign19020_e36770_d_n8, assign19020_e36770_d_n9, assign19020_e36770_d_n10, assign19020_e36770_d_n11, assign19020_e36770_d_n13, assign19020_e36770_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19020_e36767: f64 = (locals.var_vjsmfwd / locals.var_nvtms);
        let assign19020_e36768: f64 = { let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign19020_e36768, ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn0 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn2 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn3 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vjsmfwd_dn4 * locals.var_nvtms) - (locals.var_vjsmfwd * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms))), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn5 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn6 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn7 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn8 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn9 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn10 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn11 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn13 / locals.var_nvtms)), ({ let limited_exp_arg = assign19020_e36767; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn14 / locals.var_nvtms)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19020_e36770;
        locals.var_t0_dn0 = assign19020_e36770_d_n0;
        locals.var_t0_dn2 = assign19020_e36770_d_n2;
        locals.var_t0_dn3 = assign19020_e36770_d_n3;
        locals.var_t0_dn4 = assign19020_e36770_d_n4;
        locals.var_t0_dn5 = assign19020_e36770_d_n5;
        locals.var_t0_dn6 = assign19020_e36770_d_n6;
        locals.var_t0_dn7 = assign19020_e36770_d_n7;
        locals.var_t0_dn8 = assign19020_e36770_d_n8;
        locals.var_t0_dn9 = assign19020_e36770_d_n9;
        locals.var_t0_dn10 = assign19020_e36770_d_n10;
        locals.var_t0_dn11 = assign19020_e36770_d_n11;
        locals.var_t0_dn13 = assign19020_e36770_d_n13;
        locals.var_t0_dn14 = assign19020_e36770_d_n14;

        let (assign19030_e36786, assign19030_e36786_d_n0, assign19030_e36786_d_n2, assign19030_e36786_d_n3, assign19030_e36786_d_n4, assign19030_e36786_d_n5, assign19030_e36786_d_n6, assign19030_e36786_d_n7, assign19030_e36786_d_n8, assign19030_e36786_d_n9, assign19030_e36786_d_n10, assign19030_e36786_d_n11, assign19030_e36786_d_n13, assign19030_e36786_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19030_e36778: f64 = (locals.var_xexpbvs / locals.var_t0);
        let assign19030_e36779: f64 = (locals.var_t0 - assign19030_e36778);
        let assign19030_e36781: f64 = (assign19030_e36779 + locals.var_xexpbvs);
        let assign19030_e36783: f64 = (assign19030_e36781 - 1.0);
        let assign19030_e36784: f64 = (locals.var_isbs * assign19030_e36783);
        (assign19030_e36784, ((locals.var_isbs_dn0 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn0 - (-((locals.var_xexpbvs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn2 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn2 - (-((locals.var_xexpbvs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn3 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn3 - (-((locals.var_xexpbvs * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn4 * assign19030_e36783) + (locals.var_isbs * ((locals.var_t0_dn4 - (((locals.var_xexpbvs_dn4 * locals.var_t0) - (locals.var_xexpbvs * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))) + locals.var_xexpbvs_dn4))), ((locals.var_isbs_dn5 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn5 - (-((locals.var_xexpbvs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn6 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn6 - (-((locals.var_xexpbvs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn7 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn7 - (-((locals.var_xexpbvs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn8 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn8 - (-((locals.var_xexpbvs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn9 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn9 - (-((locals.var_xexpbvs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn10 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn10 - (-((locals.var_xexpbvs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn11 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn11 - (-((locals.var_xexpbvs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn13 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn13 - (-((locals.var_xexpbvs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn14 * assign19030_e36783) + (locals.var_isbs * (locals.var_t0_dn14 - (-((locals.var_xexpbvs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))),)
    } else {
        (locals.var_ivjsmfwd, locals.var_ivjsmfwd_dn0, locals.var_ivjsmfwd_dn2, locals.var_ivjsmfwd_dn3, locals.var_ivjsmfwd_dn4, locals.var_ivjsmfwd_dn5, locals.var_ivjsmfwd_dn6, locals.var_ivjsmfwd_dn7, locals.var_ivjsmfwd_dn8, locals.var_ivjsmfwd_dn9, locals.var_ivjsmfwd_dn10, locals.var_ivjsmfwd_dn11, locals.var_ivjsmfwd_dn13, locals.var_ivjsmfwd_dn14,)
    }
};
        locals.var_ivjsmfwd = assign19030_e36786;
        locals.var_ivjsmfwd_dn0 = assign19030_e36786_d_n0;
        locals.var_ivjsmfwd_dn2 = assign19030_e36786_d_n2;
        locals.var_ivjsmfwd_dn3 = assign19030_e36786_d_n3;
        locals.var_ivjsmfwd_dn4 = assign19030_e36786_d_n4;
        locals.var_ivjsmfwd_dn5 = assign19030_e36786_d_n5;
        locals.var_ivjsmfwd_dn6 = assign19030_e36786_d_n6;
        locals.var_ivjsmfwd_dn7 = assign19030_e36786_d_n7;
        locals.var_ivjsmfwd_dn8 = assign19030_e36786_d_n8;
        locals.var_ivjsmfwd_dn9 = assign19030_e36786_d_n9;
        locals.var_ivjsmfwd_dn10 = assign19030_e36786_d_n10;
        locals.var_ivjsmfwd_dn11 = assign19030_e36786_d_n11;
        locals.var_ivjsmfwd_dn13 = assign19030_e36786_d_n13;
        locals.var_ivjsmfwd_dn14 = assign19030_e36786_d_n14;

        let (assign19040_e36800, assign19040_e36800_d_n0, assign19040_e36800_d_n2, assign19040_e36800_d_n3, assign19040_e36800_d_n4, assign19040_e36800_d_n5, assign19040_e36800_d_n6, assign19040_e36800_d_n7, assign19040_e36800_d_n8, assign19040_e36800_d_n9, assign19040_e36800_d_n10, assign19040_e36800_d_n11, assign19040_e36800_d_n13, assign19040_e36800_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19040_e36794: f64 = (locals.var_xexpbvs / locals.var_t0);
        let assign19040_e36795: f64 = (locals.var_t0 + assign19040_e36794);
        let assign19040_e36796: f64 = (locals.var_isbs * assign19040_e36795);
        let assign19040_e36798: f64 = (assign19040_e36796 / locals.var_nvtms);
        (assign19040_e36798, (((locals.var_isbs_dn0 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn0 + (-((locals.var_xexpbvs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn2 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn2 + (-((locals.var_xexpbvs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn3 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn3 + (-((locals.var_xexpbvs * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((((locals.var_isbs_dn4 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn4 + (((locals.var_xexpbvs_dn4 * locals.var_t0) - (locals.var_xexpbvs * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))))) * locals.var_nvtms) - (assign19040_e36796 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)), (((locals.var_isbs_dn5 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn5 + (-((locals.var_xexpbvs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn6 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn6 + (-((locals.var_xexpbvs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn7 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn7 + (-((locals.var_xexpbvs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn8 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn8 + (-((locals.var_xexpbvs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn9 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn9 + (-((locals.var_xexpbvs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn10 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn10 + (-((locals.var_xexpbvs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn11 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn11 + (-((locals.var_xexpbvs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn13 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn13 + (-((locals.var_xexpbvs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn14 * assign19040_e36795) + (locals.var_isbs * (locals.var_t0_dn14 + (-((locals.var_xexpbvs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms),)
    } else {
        (locals.var_sslpfwd, locals.var_sslpfwd_dn0, locals.var_sslpfwd_dn2, locals.var_sslpfwd_dn3, locals.var_sslpfwd_dn4, locals.var_sslpfwd_dn5, locals.var_sslpfwd_dn6, locals.var_sslpfwd_dn7, locals.var_sslpfwd_dn8, locals.var_sslpfwd_dn9, locals.var_sslpfwd_dn10, locals.var_sslpfwd_dn11, locals.var_sslpfwd_dn13, locals.var_sslpfwd_dn14,)
    }
};
        locals.var_sslpfwd = assign19040_e36800;
        locals.var_sslpfwd_dn0 = assign19040_e36800_d_n0;
        locals.var_sslpfwd_dn2 = assign19040_e36800_d_n2;
        locals.var_sslpfwd_dn3 = assign19040_e36800_d_n3;
        locals.var_sslpfwd_dn4 = assign19040_e36800_d_n4;
        locals.var_sslpfwd_dn5 = assign19040_e36800_d_n5;
        locals.var_sslpfwd_dn6 = assign19040_e36800_d_n6;
        locals.var_sslpfwd_dn7 = assign19040_e36800_d_n7;
        locals.var_sslpfwd_dn8 = assign19040_e36800_d_n8;
        locals.var_sslpfwd_dn9 = assign19040_e36800_d_n9;
        locals.var_sslpfwd_dn10 = assign19040_e36800_d_n10;
        locals.var_sslpfwd_dn11 = assign19040_e36800_d_n11;
        locals.var_sslpfwd_dn13 = assign19040_e36800_d_n13;
        locals.var_sslpfwd_dn14 = assign19040_e36800_d_n14;

    }

    pub(super) fn stamp_transient_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19050_e36867, assign19050_e36867_d_n0, assign19050_e36867_d_n2, assign19050_e36867_d_n3, assign19050_e36867_d_n4, assign19050_e36867_d_n5, assign19050_e36867_d_n6, assign19050_e36867_d_n7, assign19050_e36867_d_n8, assign19050_e36867_d_n9, assign19050_e36867_d_n10, assign19050_e36867_d_n11, assign19050_e36867_d_n13, assign19050_e36867_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19050_e36806: f64 = (p.p1624 / locals.var_isbs);
        let assign19050_e36808: f64 = (assign19050_e36806 - 10.0);
        let assign19050_e36810: f64 = (-10000.0);
        let assign19050_e36812: f64 = (assign19050_e36810 * 0.001);
        let (assign19050_e36863, assign19050_e36863_d_n0, assign19050_e36863_d_n2, assign19050_e36863_d_n3, assign19050_e36863_d_n4, assign19050_e36863_d_n5, assign19050_e36863_d_n6, assign19050_e36863_d_n7, assign19050_e36863_d_n8, assign19050_e36863_d_n9, assign19050_e36863_d_n10, assign19050_e36863_d_n11, assign19050_e36863_d_n13, assign19050_e36863_d_n14,) = {
            if (!(assign19050_e36808 < assign19050_e36812)) {
                let assign19050_e36818: f64 = (p.p1624 / locals.var_isbs);
                let assign19050_e36820: f64 = (assign19050_e36818 - 10.0);
                let assign19050_e36823: f64 = (p.p1624 / locals.var_isbs);
                let assign19050_e36825: f64 = (assign19050_e36823 - 10.0);
                let assign19050_e36828: f64 = (p.p1624 / locals.var_isbs);
                let assign19050_e36830: f64 = (assign19050_e36828 - 10.0);
                let assign19050_e36831: f64 = (assign19050_e36825 * assign19050_e36830);
                let assign19050_e36834: f64 = (4.0 * 0.001);
                let assign19050_e36836: f64 = (assign19050_e36834 * 0.001);
                let assign19050_e36837: f64 = (assign19050_e36831 + assign19050_e36836);
                let assign19050_e36838: f64 = (assign19050_e36837).sqrt();
                let assign19050_e36839: f64 = (assign19050_e36820 + assign19050_e36838);
                let assign19050_e36840: f64 = (0.5 * assign19050_e36839);
                (assign19050_e36840, (0.5 * ((-((p.p1624 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))), (0.5 * ((-((p.p1624 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p1624 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) * assign19050_e36830) + (assign19050_e36825 * (-((p.p1624 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign19050_e36838)))),)
            } else {
                let assign19050_e36843: f64 = (p.p1624 / locals.var_isbs);
                let assign19050_e36845: f64 = (assign19050_e36843 - 10.0);
                let assign19050_e36847: f64 = (-10000.0);
                let assign19050_e36849: f64 = (assign19050_e36847 * 0.001);
                let (assign19050_e36862, assign19050_e36862_d_n0, assign19050_e36862_d_n2, assign19050_e36862_d_n3, assign19050_e36862_d_n4, assign19050_e36862_d_n5, assign19050_e36862_d_n6, assign19050_e36862_d_n7, assign19050_e36862_d_n8, assign19050_e36862_d_n9, assign19050_e36862_d_n10, assign19050_e36862_d_n11, assign19050_e36862_d_n13, assign19050_e36862_d_n14,) = {
                    if (assign19050_e36845 < assign19050_e36849) {
                        let assign19050_e36852: f64 = (-0.001);
                        let assign19050_e36854: f64 = (assign19050_e36852 * 0.001);
                        let assign19050_e36857: f64 = (p.p1624 / locals.var_isbs);
                        let assign19050_e36859: f64 = (assign19050_e36857 - 10.0);
                        let assign19050_e36860: f64 = (assign19050_e36854 / assign19050_e36859);
                        (assign19050_e36860, (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))), (-((assign19050_e36854 * (-((p.p1624 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs)))) / (assign19050_e36859 * assign19050_e36859))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19050_e36862, assign19050_e36862_d_n0, assign19050_e36862_d_n2, assign19050_e36862_d_n3, assign19050_e36862_d_n4, assign19050_e36862_d_n5, assign19050_e36862_d_n6, assign19050_e36862_d_n7, assign19050_e36862_d_n8, assign19050_e36862_d_n9, assign19050_e36862_d_n10, assign19050_e36862_d_n11, assign19050_e36862_d_n13, assign19050_e36862_d_n14,)
            }
        };
        let assign19050_e36865: f64 = (assign19050_e36863 + 10.0);
        (assign19050_e36865, assign19050_e36863_d_n0, assign19050_e36863_d_n2, assign19050_e36863_d_n3, assign19050_e36863_d_n4, assign19050_e36863_d_n5, assign19050_e36863_d_n6, assign19050_e36863_d_n7, assign19050_e36863_d_n8, assign19050_e36863_d_n9, assign19050_e36863_d_n10, assign19050_e36863_d_n11, assign19050_e36863_d_n13, assign19050_e36863_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign19050_e36867;
        locals.var_t2_dn0 = assign19050_e36867_d_n0;
        locals.var_t2_dn2 = assign19050_e36867_d_n2;
        locals.var_t2_dn3 = assign19050_e36867_d_n3;
        locals.var_t2_dn4 = assign19050_e36867_d_n4;
        locals.var_t2_dn5 = assign19050_e36867_d_n5;
        locals.var_t2_dn6 = assign19050_e36867_d_n6;
        locals.var_t2_dn7 = assign19050_e36867_d_n7;
        locals.var_t2_dn8 = assign19050_e36867_d_n8;
        locals.var_t2_dn9 = assign19050_e36867_d_n9;
        locals.var_t2_dn10 = assign19050_e36867_d_n10;
        locals.var_t2_dn11 = assign19050_e36867_d_n11;
        locals.var_t2_dn13 = assign19050_e36867_d_n13;
        locals.var_t2_dn14 = assign19050_e36867_d_n14;

        let (assign19060_e36903, assign19060_e36903_d_n0, assign19060_e36903_d_n2, assign19060_e36903_d_n3, assign19060_e36903_d_n4, assign19060_e36903_d_n5, assign19060_e36903_d_n6, assign19060_e36903_d_n7, assign19060_e36903_d_n8, assign19060_e36903_d_n9, assign19060_e36903_d_n10, assign19060_e36903_d_n11, assign19060_e36903_d_n13, assign19060_e36903_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19060_e36872: f64 = (-p.p1626);
        let assign19060_e36876: f64 = (locals.var_t2 - 1.0);
        let assign19060_e36878: f64 = (assign19060_e36876 / p.p1628);
        let (assign19060_e36899, assign19060_e36899_d_n0, assign19060_e36899_d_n2, assign19060_e36899_d_n3, assign19060_e36899_d_n4, assign19060_e36899_d_n5, assign19060_e36899_d_n6, assign19060_e36899_d_n7, assign19060_e36899_d_n8, assign19060_e36899_d_n9, assign19060_e36899_d_n10, assign19060_e36899_d_n11, assign19060_e36899_d_n13, assign19060_e36899_d_n14,) = {
            if (!(assign19060_e36878 > 1e-38)) {
                let assign19060_e36883: f64 = (-87.498233534);
                (assign19060_e36883, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19060_e36886: f64 = (locals.var_t2 - 1.0);
                let assign19060_e36888: f64 = (assign19060_e36886 / p.p1628);
                let (assign19060_e36898, assign19060_e36898_d_n0, assign19060_e36898_d_n2, assign19060_e36898_d_n3, assign19060_e36898_d_n4, assign19060_e36898_d_n5, assign19060_e36898_d_n6, assign19060_e36898_d_n7, assign19060_e36898_d_n8, assign19060_e36898_d_n9, assign19060_e36898_d_n10, assign19060_e36898_d_n11, assign19060_e36898_d_n13, assign19060_e36898_d_n14,) = {
                    if (assign19060_e36888 > 1e-38) {
                        let assign19060_e36893: f64 = (locals.var_t2 - 1.0);
                        let assign19060_e36895: f64 = (assign19060_e36893 / p.p1628);
                        let assign19060_e36896: f64 = (assign19060_e36895).ln();
                        (assign19060_e36896, ((locals.var_t2_dn0 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn2 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn3 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn4 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn5 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn6 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn7 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn8 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn9 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn10 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn11 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn13 / p.p1628) / assign19060_e36895), ((locals.var_t2_dn14 / p.p1628) / assign19060_e36895),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19060_e36898, assign19060_e36898_d_n0, assign19060_e36898_d_n2, assign19060_e36898_d_n3, assign19060_e36898_d_n4, assign19060_e36898_d_n5, assign19060_e36898_d_n6, assign19060_e36898_d_n7, assign19060_e36898_d_n8, assign19060_e36898_d_n9, assign19060_e36898_d_n10, assign19060_e36898_d_n11, assign19060_e36898_d_n13, assign19060_e36898_d_n14,)
            }
        };
        let assign19060_e36900: f64 = (locals.var_nvtms * assign19060_e36899);
        let assign19060_e36901: f64 = (assign19060_e36872 - assign19060_e36900);
        (assign19060_e36901, (-(locals.var_nvtms * assign19060_e36899_d_n0)), (-(locals.var_nvtms * assign19060_e36899_d_n2)), (-(locals.var_nvtms * assign19060_e36899_d_n3)), (-((locals.var_nvtms_dn4 * assign19060_e36899) + (locals.var_nvtms * assign19060_e36899_d_n4))), (-(locals.var_nvtms * assign19060_e36899_d_n5)), (-(locals.var_nvtms * assign19060_e36899_d_n6)), (-(locals.var_nvtms * assign19060_e36899_d_n7)), (-(locals.var_nvtms * assign19060_e36899_d_n8)), (-(locals.var_nvtms * assign19060_e36899_d_n9)), (-(locals.var_nvtms * assign19060_e36899_d_n10)), (-(locals.var_nvtms * assign19060_e36899_d_n11)), (-(locals.var_nvtms * assign19060_e36899_d_n13)), (-(locals.var_nvtms * assign19060_e36899_d_n14)),)
    } else {
        (locals.var_vjsmrev, locals.var_vjsmrev_dn0, locals.var_vjsmrev_dn2, locals.var_vjsmrev_dn3, locals.var_vjsmrev_dn4, locals.var_vjsmrev_dn5, locals.var_vjsmrev_dn6, locals.var_vjsmrev_dn7, locals.var_vjsmrev_dn8, locals.var_vjsmrev_dn9, locals.var_vjsmrev_dn10, locals.var_vjsmrev_dn11, locals.var_vjsmrev_dn13, locals.var_vjsmrev_dn14,)
    }
};
        locals.var_vjsmrev = assign19060_e36903;
        locals.var_vjsmrev_dn0 = assign19060_e36903_d_n0;
        locals.var_vjsmrev_dn2 = assign19060_e36903_d_n2;
        locals.var_vjsmrev_dn3 = assign19060_e36903_d_n3;
        locals.var_vjsmrev_dn4 = assign19060_e36903_d_n4;
        locals.var_vjsmrev_dn5 = assign19060_e36903_d_n5;
        locals.var_vjsmrev_dn6 = assign19060_e36903_d_n6;
        locals.var_vjsmrev_dn7 = assign19060_e36903_d_n7;
        locals.var_vjsmrev_dn8 = assign19060_e36903_d_n8;
        locals.var_vjsmrev_dn9 = assign19060_e36903_d_n9;
        locals.var_vjsmrev_dn10 = assign19060_e36903_d_n10;
        locals.var_vjsmrev_dn11 = assign19060_e36903_d_n11;
        locals.var_vjsmrev_dn13 = assign19060_e36903_d_n13;
        locals.var_vjsmrev_dn14 = assign19060_e36903_d_n14;

        let (assign19070_e36917, assign19070_e36917_d_n0, assign19070_e36917_d_n2, assign19070_e36917_d_n3, assign19070_e36917_d_n4, assign19070_e36917_d_n5, assign19070_e36917_d_n6, assign19070_e36917_d_n7, assign19070_e36917_d_n8, assign19070_e36917_d_n9, assign19070_e36917_d_n10, assign19070_e36917_d_n11, assign19070_e36917_d_n13, assign19070_e36917_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19070_e36910: f64 = (p.p1626 + locals.var_vjsmrev);
        let assign19070_e36911: f64 = (-assign19070_e36910);
        let assign19070_e36913: f64 = (assign19070_e36911 / locals.var_nvtms);
        let assign19070_e36914: f64 = { let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign19070_e36915: f64 = (p.p1628 * assign19070_e36914);
        (assign19070_e36915, (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn0) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn2) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn3) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-locals.var_vjsmrev_dn4) * locals.var_nvtms) - (assign19070_e36911 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn5) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn6) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn7) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn8) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn9) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn10) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn11) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn13) / locals.var_nvtms))), (p.p1628 * ({ let limited_exp_arg = assign19070_e36913; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn14) / locals.var_nvtms))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign19070_e36917;
        locals.var_t1_dn0 = assign19070_e36917_d_n0;
        locals.var_t1_dn2 = assign19070_e36917_d_n2;
        locals.var_t1_dn3 = assign19070_e36917_d_n3;
        locals.var_t1_dn4 = assign19070_e36917_d_n4;
        locals.var_t1_dn5 = assign19070_e36917_d_n5;
        locals.var_t1_dn6 = assign19070_e36917_d_n6;
        locals.var_t1_dn7 = assign19070_e36917_d_n7;
        locals.var_t1_dn8 = assign19070_e36917_d_n8;
        locals.var_t1_dn9 = assign19070_e36917_d_n9;
        locals.var_t1_dn10 = assign19070_e36917_d_n10;
        locals.var_t1_dn11 = assign19070_e36917_d_n11;
        locals.var_t1_dn13 = assign19070_e36917_d_n13;
        locals.var_t1_dn14 = assign19070_e36917_d_n14;

        let (assign19080_e36927, assign19080_e36927_d_n0, assign19080_e36927_d_n2, assign19080_e36927_d_n3, assign19080_e36927_d_n4, assign19080_e36927_d_n5, assign19080_e36927_d_n6, assign19080_e36927_d_n7, assign19080_e36927_d_n8, assign19080_e36927_d_n9, assign19080_e36927_d_n10, assign19080_e36927_d_n11, assign19080_e36927_d_n13, assign19080_e36927_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19080_e36924: f64 = (1.0 + locals.var_t1);
        let assign19080_e36925: f64 = (locals.var_isbs * assign19080_e36924);
        (assign19080_e36925, ((locals.var_isbs_dn0 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn0)), ((locals.var_isbs_dn2 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn2)), ((locals.var_isbs_dn3 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn3)), ((locals.var_isbs_dn4 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn4)), ((locals.var_isbs_dn5 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn5)), ((locals.var_isbs_dn6 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn6)), ((locals.var_isbs_dn7 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn7)), ((locals.var_isbs_dn8 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn8)), ((locals.var_isbs_dn9 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn9)), ((locals.var_isbs_dn10 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn10)), ((locals.var_isbs_dn11 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn11)), ((locals.var_isbs_dn13 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn13)), ((locals.var_isbs_dn14 * assign19080_e36924) + (locals.var_isbs * locals.var_t1_dn14)),)
    } else {
        (locals.var_ivjsmrev, locals.var_ivjsmrev_dn0, locals.var_ivjsmrev_dn2, locals.var_ivjsmrev_dn3, locals.var_ivjsmrev_dn4, locals.var_ivjsmrev_dn5, locals.var_ivjsmrev_dn6, locals.var_ivjsmrev_dn7, locals.var_ivjsmrev_dn8, locals.var_ivjsmrev_dn9, locals.var_ivjsmrev_dn10, locals.var_ivjsmrev_dn11, locals.var_ivjsmrev_dn13, locals.var_ivjsmrev_dn14,)
    }
};
        locals.var_ivjsmrev = assign19080_e36927;
        locals.var_ivjsmrev_dn0 = assign19080_e36927_d_n0;
        locals.var_ivjsmrev_dn2 = assign19080_e36927_d_n2;
        locals.var_ivjsmrev_dn3 = assign19080_e36927_d_n3;
        locals.var_ivjsmrev_dn4 = assign19080_e36927_d_n4;
        locals.var_ivjsmrev_dn5 = assign19080_e36927_d_n5;
        locals.var_ivjsmrev_dn6 = assign19080_e36927_d_n6;
        locals.var_ivjsmrev_dn7 = assign19080_e36927_d_n7;
        locals.var_ivjsmrev_dn8 = assign19080_e36927_d_n8;
        locals.var_ivjsmrev_dn9 = assign19080_e36927_d_n9;
        locals.var_ivjsmrev_dn10 = assign19080_e36927_d_n10;
        locals.var_ivjsmrev_dn11 = assign19080_e36927_d_n11;
        locals.var_ivjsmrev_dn13 = assign19080_e36927_d_n13;
        locals.var_ivjsmrev_dn14 = assign19080_e36927_d_n14;

        let (assign19090_e36938, assign19090_e36938_d_n0, assign19090_e36938_d_n2, assign19090_e36938_d_n3, assign19090_e36938_d_n4, assign19090_e36938_d_n5, assign19090_e36938_d_n6, assign19090_e36938_d_n7, assign19090_e36938_d_n8, assign19090_e36938_d_n9, assign19090_e36938_d_n10, assign19090_e36938_d_n11, assign19090_e36938_d_n13, assign19090_e36938_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard351 != 0.0)) {
        let assign19090_e36932: f64 = (-locals.var_isbs);
        let assign19090_e36934: f64 = (assign19090_e36932 * locals.var_t1);
        let assign19090_e36936: f64 = (assign19090_e36934 / locals.var_nvtms);
        (assign19090_e36936, ((((-locals.var_isbs_dn0) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn0)) / locals.var_nvtms), ((((-locals.var_isbs_dn2) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn2)) / locals.var_nvtms), ((((-locals.var_isbs_dn3) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn3)) / locals.var_nvtms), ((((((-locals.var_isbs_dn4) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn4)) * locals.var_nvtms) - (assign19090_e36934 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)), ((((-locals.var_isbs_dn5) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn5)) / locals.var_nvtms), ((((-locals.var_isbs_dn6) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn6)) / locals.var_nvtms), ((((-locals.var_isbs_dn7) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn7)) / locals.var_nvtms), ((((-locals.var_isbs_dn8) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn8)) / locals.var_nvtms), ((((-locals.var_isbs_dn9) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn9)) / locals.var_nvtms), ((((-locals.var_isbs_dn10) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn10)) / locals.var_nvtms), ((((-locals.var_isbs_dn11) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn11)) / locals.var_nvtms), ((((-locals.var_isbs_dn13) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn13)) / locals.var_nvtms), ((((-locals.var_isbs_dn14) * locals.var_t1) + (assign19090_e36932 * locals.var_t1_dn14)) / locals.var_nvtms),)
    } else {
        (locals.var_sslprev, locals.var_sslprev_dn0, locals.var_sslprev_dn2, locals.var_sslprev_dn3, locals.var_sslprev_dn4, locals.var_sslprev_dn5, locals.var_sslprev_dn6, locals.var_sslprev_dn7, locals.var_sslprev_dn8, locals.var_sslprev_dn9, locals.var_sslprev_dn10, locals.var_sslprev_dn11, locals.var_sslprev_dn13, locals.var_sslprev_dn14,)
    }
};
        locals.var_sslprev = assign19090_e36938;
        locals.var_sslprev_dn0 = assign19090_e36938_d_n0;
        locals.var_sslprev_dn2 = assign19090_e36938_d_n2;
        locals.var_sslprev_dn3 = assign19090_e36938_d_n3;
        locals.var_sslprev_dn4 = assign19090_e36938_d_n4;
        locals.var_sslprev_dn5 = assign19090_e36938_d_n5;
        locals.var_sslprev_dn6 = assign19090_e36938_d_n6;
        locals.var_sslprev_dn7 = assign19090_e36938_d_n7;
        locals.var_sslprev_dn8 = assign19090_e36938_d_n8;
        locals.var_sslprev_dn9 = assign19090_e36938_d_n9;
        locals.var_sslprev_dn10 = assign19090_e36938_d_n10;
        locals.var_sslprev_dn11 = assign19090_e36938_d_n11;
        locals.var_sslprev_dn13 = assign19090_e36938_d_n13;
        locals.var_sslprev_dn14 = assign19090_e36938_d_n14;

        let (assign19100_e36954, assign19100_e36954_d_n0, assign19100_e36954_d_n2, assign19100_e36954_d_n3, assign19100_e36954_d_n4, assign19100_e36954_d_n5, assign19100_e36954_d_n6, assign19100_e36954_d_n7, assign19100_e36954_d_n8, assign19100_e36954_d_n9, assign19100_e36954_d_n10, assign19100_e36954_d_n11, assign19100_e36954_d_n13, assign19100_e36954_d_n14,) = {
    if (locals.var_guard350 != 0.0) {
        let assign19100_e36942: f64 = (p.p12 * locals.var_jsd_t);
        let assign19100_e36945: f64 = (p.p14 * locals.var_jswd_t);
        let assign19100_e36946: f64 = (assign19100_e36942 + assign19100_e36945);
        let assign19100_e36949: f64 = (p.p3 * locals.var_nfintotal);
        let assign19100_e36951: f64 = (assign19100_e36949 * locals.var_jswgd_t);
        let assign19100_e36952: f64 = (assign19100_e36946 + assign19100_e36951);
        (assign19100_e36952, (((p.p12 * locals.var_jsd_t_dn0) + (p.p14 * locals.var_jswd_t_dn0)) + (assign19100_e36949 * locals.var_jswgd_t_dn0)), (((p.p12 * locals.var_jsd_t_dn2) + (p.p14 * locals.var_jswd_t_dn2)) + (assign19100_e36949 * locals.var_jswgd_t_dn2)), (((p.p12 * locals.var_jsd_t_dn3) + (p.p14 * locals.var_jswd_t_dn3)) + (assign19100_e36949 * locals.var_jswgd_t_dn3)), (((p.p12 * locals.var_jsd_t_dn4) + (p.p14 * locals.var_jswd_t_dn4)) + (assign19100_e36949 * locals.var_jswgd_t_dn4)), (((p.p12 * locals.var_jsd_t_dn5) + (p.p14 * locals.var_jswd_t_dn5)) + (assign19100_e36949 * locals.var_jswgd_t_dn5)), (((p.p12 * locals.var_jsd_t_dn6) + (p.p14 * locals.var_jswd_t_dn6)) + (assign19100_e36949 * locals.var_jswgd_t_dn6)), (((p.p12 * locals.var_jsd_t_dn7) + (p.p14 * locals.var_jswd_t_dn7)) + (assign19100_e36949 * locals.var_jswgd_t_dn7)), (((p.p12 * locals.var_jsd_t_dn8) + (p.p14 * locals.var_jswd_t_dn8)) + (assign19100_e36949 * locals.var_jswgd_t_dn8)), (((p.p12 * locals.var_jsd_t_dn9) + (p.p14 * locals.var_jswd_t_dn9)) + (assign19100_e36949 * locals.var_jswgd_t_dn9)), (((p.p12 * locals.var_jsd_t_dn10) + (p.p14 * locals.var_jswd_t_dn10)) + (assign19100_e36949 * locals.var_jswgd_t_dn10)), (((p.p12 * locals.var_jsd_t_dn11) + (p.p14 * locals.var_jswd_t_dn11)) + (assign19100_e36949 * locals.var_jswgd_t_dn11)), (((p.p12 * locals.var_jsd_t_dn13) + (p.p14 * locals.var_jswd_t_dn13)) + (assign19100_e36949 * locals.var_jswgd_t_dn13)), (((p.p12 * locals.var_jsd_t_dn14) + (p.p14 * locals.var_jswd_t_dn14)) + (assign19100_e36949 * locals.var_jswgd_t_dn14)),)
    } else {
        (locals.var_isbd, locals.var_isbd_dn0, locals.var_isbd_dn2, locals.var_isbd_dn3, locals.var_isbd_dn4, locals.var_isbd_dn5, locals.var_isbd_dn6, locals.var_isbd_dn7, locals.var_isbd_dn8, locals.var_isbd_dn9, locals.var_isbd_dn10, locals.var_isbd_dn11, locals.var_isbd_dn13, locals.var_isbd_dn14,)
    }
};
        locals.var_isbd = assign19100_e36954;
        locals.var_isbd_dn0 = assign19100_e36954_d_n0;
        locals.var_isbd_dn2 = assign19100_e36954_d_n2;
        locals.var_isbd_dn3 = assign19100_e36954_d_n3;
        locals.var_isbd_dn4 = assign19100_e36954_d_n4;
        locals.var_isbd_dn5 = assign19100_e36954_d_n5;
        locals.var_isbd_dn6 = assign19100_e36954_d_n6;
        locals.var_isbd_dn7 = assign19100_e36954_d_n7;
        locals.var_isbd_dn8 = assign19100_e36954_d_n8;
        locals.var_isbd_dn9 = assign19100_e36954_d_n9;
        locals.var_isbd_dn10 = assign19100_e36954_d_n10;
        locals.var_isbd_dn11 = assign19100_e36954_d_n11;
        locals.var_isbd_dn13 = assign19100_e36954_d_n13;
        locals.var_isbd_dn14 = assign19100_e36954_d_n14;

        let assign19110_e36957: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard352 = assign19110_e36957;

        let (assign19120_e36965, assign19120_e36965_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19120_e36963: f64 = (locals.var_vtm * p.p1621);
        (assign19120_e36963, (locals.var_vtm_dn4 * p.p1621),)
    } else {
        (locals.var_nvtmd, locals.var_nvtmd_dn4,)
    }
};
        locals.var_nvtmd = assign19120_e36965;
        locals.var_nvtmd_dn4 = assign19120_e36965_d_n4;

        let (assign19130_e36977, assign19130_e36977_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19130_e36970: f64 = (-p.p1627);
        let assign19130_e36972: f64 = (assign19130_e36970 / locals.var_nvtmd);
        let assign19130_e36973: f64 = { let limited_exp_arg = assign19130_e36972; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign19130_e36975: f64 = (assign19130_e36973 * p.p1629);
        (assign19130_e36975, (({ let limited_exp_arg = assign19130_e36972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign19130_e36970 * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd)))) * p.p1629),)
    } else {
        (locals.var_xexpbvd, locals.var_xexpbvd_dn4,)
    }
};
        locals.var_xexpbvd = assign19130_e36977;
        locals.var_xexpbvd_dn4 = assign19130_e36977_d_n4;

        let (assign19140_e36987, assign19140_e36987_d_n0, assign19140_e36987_d_n2, assign19140_e36987_d_n3, assign19140_e36987_d_n4, assign19140_e36987_d_n5, assign19140_e36987_d_n6, assign19140_e36987_d_n7, assign19140_e36987_d_n8, assign19140_e36987_d_n9, assign19140_e36987_d_n10, assign19140_e36987_d_n11, assign19140_e36987_d_n13, assign19140_e36987_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19140_e36983: f64 = (p.p1623 / locals.var_isbd);
        let assign19140_e36985: f64 = (assign19140_e36983).max(10.0);
        (assign19140_e36985, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign19140_e36983 >= 10.0 { (-((p.p1623 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign19140_e36987;
        locals.var_t2_dn0 = assign19140_e36987_d_n0;
        locals.var_t2_dn2 = assign19140_e36987_d_n2;
        locals.var_t2_dn3 = assign19140_e36987_d_n3;
        locals.var_t2_dn4 = assign19140_e36987_d_n4;
        locals.var_t2_dn5 = assign19140_e36987_d_n5;
        locals.var_t2_dn6 = assign19140_e36987_d_n6;
        locals.var_t2_dn7 = assign19140_e36987_d_n7;
        locals.var_t2_dn8 = assign19140_e36987_d_n8;
        locals.var_t2_dn9 = assign19140_e36987_d_n9;
        locals.var_t2_dn10 = assign19140_e36987_d_n10;
        locals.var_t2_dn11 = assign19140_e36987_d_n11;
        locals.var_t2_dn13 = assign19140_e36987_d_n13;
        locals.var_t2_dn14 = assign19140_e36987_d_n14;

        let (assign19150_e36997, assign19150_e36997_d_n0, assign19150_e36997_d_n2, assign19150_e36997_d_n3, assign19150_e36997_d_n4, assign19150_e36997_d_n5, assign19150_e36997_d_n6, assign19150_e36997_d_n7, assign19150_e36997_d_n8, assign19150_e36997_d_n9, assign19150_e36997_d_n10, assign19150_e36997_d_n11, assign19150_e36997_d_n13, assign19150_e36997_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19150_e36993: f64 = (1.0 + locals.var_t2);
        let assign19150_e36995: f64 = (assign19150_e36993 - locals.var_xexpbvd);
        (assign19150_e36995, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, (locals.var_t2_dn4 - locals.var_xexpbvd_dn4), locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    } else {
        (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn13, locals.var_tb_dn14,)
    }
};
        locals.var_tb = assign19150_e36997;
        locals.var_tb_dn0 = assign19150_e36997_d_n0;
        locals.var_tb_dn2 = assign19150_e36997_d_n2;
        locals.var_tb_dn3 = assign19150_e36997_d_n3;
        locals.var_tb_dn4 = assign19150_e36997_d_n4;
        locals.var_tb_dn5 = assign19150_e36997_d_n5;
        locals.var_tb_dn6 = assign19150_e36997_d_n6;
        locals.var_tb_dn7 = assign19150_e36997_d_n7;
        locals.var_tb_dn8 = assign19150_e36997_d_n8;
        locals.var_tb_dn9 = assign19150_e36997_d_n9;
        locals.var_tb_dn10 = assign19150_e36997_d_n10;
        locals.var_tb_dn11 = assign19150_e36997_d_n11;
        locals.var_tb_dn13 = assign19150_e36997_d_n13;
        locals.var_tb_dn14 = assign19150_e36997_d_n14;

        let (assign19160_e37051, assign19160_e37051_d_n0, assign19160_e37051_d_n2, assign19160_e37051_d_n3, assign19160_e37051_d_n4, assign19160_e37051_d_n5, assign19160_e37051_d_n6, assign19160_e37051_d_n7, assign19160_e37051_d_n8, assign19160_e37051_d_n9, assign19160_e37051_d_n10, assign19160_e37051_d_n11, assign19160_e37051_d_n13, assign19160_e37051_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19160_e37006: f64 = (locals.var_tb * locals.var_tb);
        let assign19160_e37009: f64 = (4.0 * locals.var_xexpbvd);
        let assign19160_e37010: f64 = (assign19160_e37006 + assign19160_e37009);
        let assign19160_e37011: f64 = (assign19160_e37010).sqrt();
        let assign19160_e37012: f64 = (locals.var_tb + assign19160_e37011);
        let assign19160_e37013: f64 = (0.5 * assign19160_e37012);
        let (assign19160_e37048, assign19160_e37048_d_n0, assign19160_e37048_d_n2, assign19160_e37048_d_n3, assign19160_e37048_d_n4, assign19160_e37048_d_n5, assign19160_e37048_d_n6, assign19160_e37048_d_n7, assign19160_e37048_d_n8, assign19160_e37048_d_n9, assign19160_e37048_d_n10, assign19160_e37048_d_n11, assign19160_e37048_d_n13, assign19160_e37048_d_n14,) = {
            if (!(assign19160_e37013 > 1e-38)) {
                let assign19160_e37018: f64 = (-87.498233534);
                (assign19160_e37018, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19160_e37023: f64 = (locals.var_tb * locals.var_tb);
                let assign19160_e37026: f64 = (4.0 * locals.var_xexpbvd);
                let assign19160_e37027: f64 = (assign19160_e37023 + assign19160_e37026);
                let assign19160_e37028: f64 = (assign19160_e37027).sqrt();
                let assign19160_e37029: f64 = (locals.var_tb + assign19160_e37028);
                let assign19160_e37030: f64 = (0.5 * assign19160_e37029);
                let (assign19160_e37047, assign19160_e37047_d_n0, assign19160_e37047_d_n2, assign19160_e37047_d_n3, assign19160_e37047_d_n4, assign19160_e37047_d_n5, assign19160_e37047_d_n6, assign19160_e37047_d_n7, assign19160_e37047_d_n8, assign19160_e37047_d_n9, assign19160_e37047_d_n10, assign19160_e37047_d_n11, assign19160_e37047_d_n13, assign19160_e37047_d_n14,) = {
                    if (assign19160_e37030 > 1e-38) {
                        let assign19160_e37037: f64 = (locals.var_tb * locals.var_tb);
                        let assign19160_e37040: f64 = (4.0 * locals.var_xexpbvd);
                        let assign19160_e37041: f64 = (assign19160_e37037 + assign19160_e37040);
                        let assign19160_e37042: f64 = (assign19160_e37041).sqrt();
                        let assign19160_e37043: f64 = (locals.var_tb + assign19160_e37042);
                        let assign19160_e37044: f64 = (0.5 * assign19160_e37043);
                        let assign19160_e37045: f64 = (assign19160_e37044).ln();
                        (assign19160_e37045, ((0.5 * (locals.var_tb_dn0 + (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn2 + (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn3 + (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) + (4.0 * locals.var_xexpbvd_dn4)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn5 + (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn6 + (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn7 + (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn8 + (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn9 + (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn10 + (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn11 + (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn13 + (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) / (2.0 * assign19160_e37042)))) / assign19160_e37044), ((0.5 * (locals.var_tb_dn14 + (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) / (2.0 * assign19160_e37042)))) / assign19160_e37044),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19160_e37047, assign19160_e37047_d_n0, assign19160_e37047_d_n2, assign19160_e37047_d_n3, assign19160_e37047_d_n4, assign19160_e37047_d_n5, assign19160_e37047_d_n6, assign19160_e37047_d_n7, assign19160_e37047_d_n8, assign19160_e37047_d_n9, assign19160_e37047_d_n10, assign19160_e37047_d_n11, assign19160_e37047_d_n13, assign19160_e37047_d_n14,)
            }
        };
        let assign19160_e37049: f64 = (locals.var_nvtmd * assign19160_e37048);
        (assign19160_e37049, (locals.var_nvtmd * assign19160_e37048_d_n0), (locals.var_nvtmd * assign19160_e37048_d_n2), (locals.var_nvtmd * assign19160_e37048_d_n3), ((locals.var_nvtmd_dn4 * assign19160_e37048) + (locals.var_nvtmd * assign19160_e37048_d_n4)), (locals.var_nvtmd * assign19160_e37048_d_n5), (locals.var_nvtmd * assign19160_e37048_d_n6), (locals.var_nvtmd * assign19160_e37048_d_n7), (locals.var_nvtmd * assign19160_e37048_d_n8), (locals.var_nvtmd * assign19160_e37048_d_n9), (locals.var_nvtmd * assign19160_e37048_d_n10), (locals.var_nvtmd * assign19160_e37048_d_n11), (locals.var_nvtmd * assign19160_e37048_d_n13), (locals.var_nvtmd * assign19160_e37048_d_n14),)
    } else {
        (locals.var_vjdmfwd, locals.var_vjdmfwd_dn0, locals.var_vjdmfwd_dn2, locals.var_vjdmfwd_dn3, locals.var_vjdmfwd_dn4, locals.var_vjdmfwd_dn5, locals.var_vjdmfwd_dn6, locals.var_vjdmfwd_dn7, locals.var_vjdmfwd_dn8, locals.var_vjdmfwd_dn9, locals.var_vjdmfwd_dn10, locals.var_vjdmfwd_dn11, locals.var_vjdmfwd_dn13, locals.var_vjdmfwd_dn14,)
    }
};
        locals.var_vjdmfwd = assign19160_e37051;
        locals.var_vjdmfwd_dn0 = assign19160_e37051_d_n0;
        locals.var_vjdmfwd_dn2 = assign19160_e37051_d_n2;
        locals.var_vjdmfwd_dn3 = assign19160_e37051_d_n3;
        locals.var_vjdmfwd_dn4 = assign19160_e37051_d_n4;
        locals.var_vjdmfwd_dn5 = assign19160_e37051_d_n5;
        locals.var_vjdmfwd_dn6 = assign19160_e37051_d_n6;
        locals.var_vjdmfwd_dn7 = assign19160_e37051_d_n7;
        locals.var_vjdmfwd_dn8 = assign19160_e37051_d_n8;
        locals.var_vjdmfwd_dn9 = assign19160_e37051_d_n9;
        locals.var_vjdmfwd_dn10 = assign19160_e37051_d_n10;
        locals.var_vjdmfwd_dn11 = assign19160_e37051_d_n11;
        locals.var_vjdmfwd_dn13 = assign19160_e37051_d_n13;
        locals.var_vjdmfwd_dn14 = assign19160_e37051_d_n14;

        let (assign19170_e37060, assign19170_e37060_d_n0, assign19170_e37060_d_n2, assign19170_e37060_d_n3, assign19170_e37060_d_n4, assign19170_e37060_d_n5, assign19170_e37060_d_n6, assign19170_e37060_d_n7, assign19170_e37060_d_n8, assign19170_e37060_d_n9, assign19170_e37060_d_n10, assign19170_e37060_d_n11, assign19170_e37060_d_n13, assign19170_e37060_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19170_e37057: f64 = (locals.var_vjdmfwd / locals.var_nvtmd);
        let assign19170_e37058: f64 = { let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign19170_e37058, ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn0 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn2 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn3 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vjdmfwd_dn4 * locals.var_nvtmd) - (locals.var_vjdmfwd * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd))), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn5 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn6 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn7 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn8 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn9 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn10 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn11 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn13 / locals.var_nvtmd)), ({ let limited_exp_arg = assign19170_e37057; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn14 / locals.var_nvtmd)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19170_e37060;
        locals.var_t0_dn0 = assign19170_e37060_d_n0;
        locals.var_t0_dn2 = assign19170_e37060_d_n2;
        locals.var_t0_dn3 = assign19170_e37060_d_n3;
        locals.var_t0_dn4 = assign19170_e37060_d_n4;
        locals.var_t0_dn5 = assign19170_e37060_d_n5;
        locals.var_t0_dn6 = assign19170_e37060_d_n6;
        locals.var_t0_dn7 = assign19170_e37060_d_n7;
        locals.var_t0_dn8 = assign19170_e37060_d_n8;
        locals.var_t0_dn9 = assign19170_e37060_d_n9;
        locals.var_t0_dn10 = assign19170_e37060_d_n10;
        locals.var_t0_dn11 = assign19170_e37060_d_n11;
        locals.var_t0_dn13 = assign19170_e37060_d_n13;
        locals.var_t0_dn14 = assign19170_e37060_d_n14;

        let (assign19180_e37076, assign19180_e37076_d_n0, assign19180_e37076_d_n2, assign19180_e37076_d_n3, assign19180_e37076_d_n4, assign19180_e37076_d_n5, assign19180_e37076_d_n6, assign19180_e37076_d_n7, assign19180_e37076_d_n8, assign19180_e37076_d_n9, assign19180_e37076_d_n10, assign19180_e37076_d_n11, assign19180_e37076_d_n13, assign19180_e37076_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19180_e37068: f64 = (locals.var_xexpbvd / locals.var_t0);
        let assign19180_e37069: f64 = (locals.var_t0 - assign19180_e37068);
        let assign19180_e37071: f64 = (assign19180_e37069 + locals.var_xexpbvd);
        let assign19180_e37073: f64 = (assign19180_e37071 - 1.0);
        let assign19180_e37074: f64 = (locals.var_isbd * assign19180_e37073);
        (assign19180_e37074, ((locals.var_isbd_dn0 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn0 - (-((locals.var_xexpbvd * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn2 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn2 - (-((locals.var_xexpbvd * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn3 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn3 - (-((locals.var_xexpbvd * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn4 * assign19180_e37073) + (locals.var_isbd * ((locals.var_t0_dn4 - (((locals.var_xexpbvd_dn4 * locals.var_t0) - (locals.var_xexpbvd * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))) + locals.var_xexpbvd_dn4))), ((locals.var_isbd_dn5 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn5 - (-((locals.var_xexpbvd * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn6 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn6 - (-((locals.var_xexpbvd * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn7 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn7 - (-((locals.var_xexpbvd * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn8 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn8 - (-((locals.var_xexpbvd * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn9 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn9 - (-((locals.var_xexpbvd * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn10 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn10 - (-((locals.var_xexpbvd * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn11 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn11 - (-((locals.var_xexpbvd * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn13 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn13 - (-((locals.var_xexpbvd * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn14 * assign19180_e37073) + (locals.var_isbd * (locals.var_t0_dn14 - (-((locals.var_xexpbvd * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))),)
    } else {
        (locals.var_ivjdmfwd, locals.var_ivjdmfwd_dn0, locals.var_ivjdmfwd_dn2, locals.var_ivjdmfwd_dn3, locals.var_ivjdmfwd_dn4, locals.var_ivjdmfwd_dn5, locals.var_ivjdmfwd_dn6, locals.var_ivjdmfwd_dn7, locals.var_ivjdmfwd_dn8, locals.var_ivjdmfwd_dn9, locals.var_ivjdmfwd_dn10, locals.var_ivjdmfwd_dn11, locals.var_ivjdmfwd_dn13, locals.var_ivjdmfwd_dn14,)
    }
};
        locals.var_ivjdmfwd = assign19180_e37076;
        locals.var_ivjdmfwd_dn0 = assign19180_e37076_d_n0;
        locals.var_ivjdmfwd_dn2 = assign19180_e37076_d_n2;
        locals.var_ivjdmfwd_dn3 = assign19180_e37076_d_n3;
        locals.var_ivjdmfwd_dn4 = assign19180_e37076_d_n4;
        locals.var_ivjdmfwd_dn5 = assign19180_e37076_d_n5;
        locals.var_ivjdmfwd_dn6 = assign19180_e37076_d_n6;
        locals.var_ivjdmfwd_dn7 = assign19180_e37076_d_n7;
        locals.var_ivjdmfwd_dn8 = assign19180_e37076_d_n8;
        locals.var_ivjdmfwd_dn9 = assign19180_e37076_d_n9;
        locals.var_ivjdmfwd_dn10 = assign19180_e37076_d_n10;
        locals.var_ivjdmfwd_dn11 = assign19180_e37076_d_n11;
        locals.var_ivjdmfwd_dn13 = assign19180_e37076_d_n13;
        locals.var_ivjdmfwd_dn14 = assign19180_e37076_d_n14;

        let (assign19190_e37090, assign19190_e37090_d_n0, assign19190_e37090_d_n2, assign19190_e37090_d_n3, assign19190_e37090_d_n4, assign19190_e37090_d_n5, assign19190_e37090_d_n6, assign19190_e37090_d_n7, assign19190_e37090_d_n8, assign19190_e37090_d_n9, assign19190_e37090_d_n10, assign19190_e37090_d_n11, assign19190_e37090_d_n13, assign19190_e37090_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19190_e37084: f64 = (locals.var_xexpbvd / locals.var_t0);
        let assign19190_e37085: f64 = (locals.var_t0 + assign19190_e37084);
        let assign19190_e37086: f64 = (locals.var_isbd * assign19190_e37085);
        let assign19190_e37088: f64 = (assign19190_e37086 / locals.var_nvtmd);
        (assign19190_e37088, (((locals.var_isbd_dn0 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn0 + (-((locals.var_xexpbvd * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn2 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn2 + (-((locals.var_xexpbvd * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn3 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn3 + (-((locals.var_xexpbvd * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((((locals.var_isbd_dn4 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn4 + (((locals.var_xexpbvd_dn4 * locals.var_t0) - (locals.var_xexpbvd * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))))) * locals.var_nvtmd) - (assign19190_e37086 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)), (((locals.var_isbd_dn5 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn5 + (-((locals.var_xexpbvd * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn6 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn6 + (-((locals.var_xexpbvd * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn7 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn7 + (-((locals.var_xexpbvd * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn8 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn8 + (-((locals.var_xexpbvd * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn9 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn9 + (-((locals.var_xexpbvd * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn10 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn10 + (-((locals.var_xexpbvd * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn11 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn11 + (-((locals.var_xexpbvd * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn13 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn13 + (-((locals.var_xexpbvd * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn14 * assign19190_e37085) + (locals.var_isbd * (locals.var_t0_dn14 + (-((locals.var_xexpbvd * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd),)
    } else {
        (locals.var_dslpfwd, locals.var_dslpfwd_dn0, locals.var_dslpfwd_dn2, locals.var_dslpfwd_dn3, locals.var_dslpfwd_dn4, locals.var_dslpfwd_dn5, locals.var_dslpfwd_dn6, locals.var_dslpfwd_dn7, locals.var_dslpfwd_dn8, locals.var_dslpfwd_dn9, locals.var_dslpfwd_dn10, locals.var_dslpfwd_dn11, locals.var_dslpfwd_dn13, locals.var_dslpfwd_dn14,)
    }
};
        locals.var_dslpfwd = assign19190_e37090;
        locals.var_dslpfwd_dn0 = assign19190_e37090_d_n0;
        locals.var_dslpfwd_dn2 = assign19190_e37090_d_n2;
        locals.var_dslpfwd_dn3 = assign19190_e37090_d_n3;
        locals.var_dslpfwd_dn4 = assign19190_e37090_d_n4;
        locals.var_dslpfwd_dn5 = assign19190_e37090_d_n5;
        locals.var_dslpfwd_dn6 = assign19190_e37090_d_n6;
        locals.var_dslpfwd_dn7 = assign19190_e37090_d_n7;
        locals.var_dslpfwd_dn8 = assign19190_e37090_d_n8;
        locals.var_dslpfwd_dn9 = assign19190_e37090_d_n9;
        locals.var_dslpfwd_dn10 = assign19190_e37090_d_n10;
        locals.var_dslpfwd_dn11 = assign19190_e37090_d_n11;
        locals.var_dslpfwd_dn13 = assign19190_e37090_d_n13;
        locals.var_dslpfwd_dn14 = assign19190_e37090_d_n14;

        let (assign19200_e37157, assign19200_e37157_d_n0, assign19200_e37157_d_n2, assign19200_e37157_d_n3, assign19200_e37157_d_n4, assign19200_e37157_d_n5, assign19200_e37157_d_n6, assign19200_e37157_d_n7, assign19200_e37157_d_n8, assign19200_e37157_d_n9, assign19200_e37157_d_n10, assign19200_e37157_d_n11, assign19200_e37157_d_n13, assign19200_e37157_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19200_e37096: f64 = (p.p1625 / locals.var_isbd);
        let assign19200_e37098: f64 = (assign19200_e37096 - 10.0);
        let assign19200_e37100: f64 = (-10000.0);
        let assign19200_e37102: f64 = (assign19200_e37100 * 0.001);
        let (assign19200_e37153, assign19200_e37153_d_n0, assign19200_e37153_d_n2, assign19200_e37153_d_n3, assign19200_e37153_d_n4, assign19200_e37153_d_n5, assign19200_e37153_d_n6, assign19200_e37153_d_n7, assign19200_e37153_d_n8, assign19200_e37153_d_n9, assign19200_e37153_d_n10, assign19200_e37153_d_n11, assign19200_e37153_d_n13, assign19200_e37153_d_n14,) = {
            if (!(assign19200_e37098 < assign19200_e37102)) {
                let assign19200_e37108: f64 = (p.p1625 / locals.var_isbd);
                let assign19200_e37110: f64 = (assign19200_e37108 - 10.0);
                let assign19200_e37113: f64 = (p.p1625 / locals.var_isbd);
                let assign19200_e37115: f64 = (assign19200_e37113 - 10.0);
                let assign19200_e37118: f64 = (p.p1625 / locals.var_isbd);
                let assign19200_e37120: f64 = (assign19200_e37118 - 10.0);
                let assign19200_e37121: f64 = (assign19200_e37115 * assign19200_e37120);
                let assign19200_e37124: f64 = (4.0 * 0.001);
                let assign19200_e37126: f64 = (assign19200_e37124 * 0.001);
                let assign19200_e37127: f64 = (assign19200_e37121 + assign19200_e37126);
                let assign19200_e37128: f64 = (assign19200_e37127).sqrt();
                let assign19200_e37129: f64 = (assign19200_e37110 + assign19200_e37128);
                let assign19200_e37130: f64 = (0.5 * assign19200_e37129);
                (assign19200_e37130, (0.5 * ((-((p.p1625 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))), (0.5 * ((-((p.p1625 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p1625 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) * assign19200_e37120) + (assign19200_e37115 * (-((p.p1625 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign19200_e37128)))),)
            } else {
                let assign19200_e37133: f64 = (p.p1625 / locals.var_isbd);
                let assign19200_e37135: f64 = (assign19200_e37133 - 10.0);
                let assign19200_e37137: f64 = (-10000.0);
                let assign19200_e37139: f64 = (assign19200_e37137 * 0.001);
                let (assign19200_e37152, assign19200_e37152_d_n0, assign19200_e37152_d_n2, assign19200_e37152_d_n3, assign19200_e37152_d_n4, assign19200_e37152_d_n5, assign19200_e37152_d_n6, assign19200_e37152_d_n7, assign19200_e37152_d_n8, assign19200_e37152_d_n9, assign19200_e37152_d_n10, assign19200_e37152_d_n11, assign19200_e37152_d_n13, assign19200_e37152_d_n14,) = {
                    if (assign19200_e37135 < assign19200_e37139) {
                        let assign19200_e37142: f64 = (-0.001);
                        let assign19200_e37144: f64 = (assign19200_e37142 * 0.001);
                        let assign19200_e37147: f64 = (p.p1625 / locals.var_isbd);
                        let assign19200_e37149: f64 = (assign19200_e37147 - 10.0);
                        let assign19200_e37150: f64 = (assign19200_e37144 / assign19200_e37149);
                        (assign19200_e37150, (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))), (-((assign19200_e37144 * (-((p.p1625 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd)))) / (assign19200_e37149 * assign19200_e37149))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19200_e37152, assign19200_e37152_d_n0, assign19200_e37152_d_n2, assign19200_e37152_d_n3, assign19200_e37152_d_n4, assign19200_e37152_d_n5, assign19200_e37152_d_n6, assign19200_e37152_d_n7, assign19200_e37152_d_n8, assign19200_e37152_d_n9, assign19200_e37152_d_n10, assign19200_e37152_d_n11, assign19200_e37152_d_n13, assign19200_e37152_d_n14,)
            }
        };
        let assign19200_e37155: f64 = (assign19200_e37153 + 10.0);
        (assign19200_e37155, assign19200_e37153_d_n0, assign19200_e37153_d_n2, assign19200_e37153_d_n3, assign19200_e37153_d_n4, assign19200_e37153_d_n5, assign19200_e37153_d_n6, assign19200_e37153_d_n7, assign19200_e37153_d_n8, assign19200_e37153_d_n9, assign19200_e37153_d_n10, assign19200_e37153_d_n11, assign19200_e37153_d_n13, assign19200_e37153_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign19200_e37157;
        locals.var_t2_dn0 = assign19200_e37157_d_n0;
        locals.var_t2_dn2 = assign19200_e37157_d_n2;
        locals.var_t2_dn3 = assign19200_e37157_d_n3;
        locals.var_t2_dn4 = assign19200_e37157_d_n4;
        locals.var_t2_dn5 = assign19200_e37157_d_n5;
        locals.var_t2_dn6 = assign19200_e37157_d_n6;
        locals.var_t2_dn7 = assign19200_e37157_d_n7;
        locals.var_t2_dn8 = assign19200_e37157_d_n8;
        locals.var_t2_dn9 = assign19200_e37157_d_n9;
        locals.var_t2_dn10 = assign19200_e37157_d_n10;
        locals.var_t2_dn11 = assign19200_e37157_d_n11;
        locals.var_t2_dn13 = assign19200_e37157_d_n13;
        locals.var_t2_dn14 = assign19200_e37157_d_n14;

    }

    pub(super) fn stamp_transient_block_70(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (assign19210_e37193, assign19210_e37193_d_n0, assign19210_e37193_d_n2, assign19210_e37193_d_n3, assign19210_e37193_d_n4, assign19210_e37193_d_n5, assign19210_e37193_d_n6, assign19210_e37193_d_n7, assign19210_e37193_d_n8, assign19210_e37193_d_n9, assign19210_e37193_d_n10, assign19210_e37193_d_n11, assign19210_e37193_d_n13, assign19210_e37193_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19210_e37162: f64 = (-p.p1627);
        let assign19210_e37166: f64 = (locals.var_t2 - 1.0);
        let assign19210_e37168: f64 = (assign19210_e37166 / p.p1629);
        let (assign19210_e37189, assign19210_e37189_d_n0, assign19210_e37189_d_n2, assign19210_e37189_d_n3, assign19210_e37189_d_n4, assign19210_e37189_d_n5, assign19210_e37189_d_n6, assign19210_e37189_d_n7, assign19210_e37189_d_n8, assign19210_e37189_d_n9, assign19210_e37189_d_n10, assign19210_e37189_d_n11, assign19210_e37189_d_n13, assign19210_e37189_d_n14,) = {
            if (!(assign19210_e37168 > 1e-38)) {
                let assign19210_e37173: f64 = (-87.498233534);
                (assign19210_e37173, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign19210_e37176: f64 = (locals.var_t2 - 1.0);
                let assign19210_e37178: f64 = (assign19210_e37176 / p.p1629);
                let (assign19210_e37188, assign19210_e37188_d_n0, assign19210_e37188_d_n2, assign19210_e37188_d_n3, assign19210_e37188_d_n4, assign19210_e37188_d_n5, assign19210_e37188_d_n6, assign19210_e37188_d_n7, assign19210_e37188_d_n8, assign19210_e37188_d_n9, assign19210_e37188_d_n10, assign19210_e37188_d_n11, assign19210_e37188_d_n13, assign19210_e37188_d_n14,) = {
                    if (assign19210_e37178 > 1e-38) {
                        let assign19210_e37183: f64 = (locals.var_t2 - 1.0);
                        let assign19210_e37185: f64 = (assign19210_e37183 / p.p1629);
                        let assign19210_e37186: f64 = (assign19210_e37185).ln();
                        (assign19210_e37186, ((locals.var_t2_dn0 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn2 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn3 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn4 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn5 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn6 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn7 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn8 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn9 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn10 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn11 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn13 / p.p1629) / assign19210_e37185), ((locals.var_t2_dn14 / p.p1629) / assign19210_e37185),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign19210_e37188, assign19210_e37188_d_n0, assign19210_e37188_d_n2, assign19210_e37188_d_n3, assign19210_e37188_d_n4, assign19210_e37188_d_n5, assign19210_e37188_d_n6, assign19210_e37188_d_n7, assign19210_e37188_d_n8, assign19210_e37188_d_n9, assign19210_e37188_d_n10, assign19210_e37188_d_n11, assign19210_e37188_d_n13, assign19210_e37188_d_n14,)
            }
        };
        let assign19210_e37190: f64 = (locals.var_nvtmd * assign19210_e37189);
        let assign19210_e37191: f64 = (assign19210_e37162 - assign19210_e37190);
        (assign19210_e37191, (-(locals.var_nvtmd * assign19210_e37189_d_n0)), (-(locals.var_nvtmd * assign19210_e37189_d_n2)), (-(locals.var_nvtmd * assign19210_e37189_d_n3)), (-((locals.var_nvtmd_dn4 * assign19210_e37189) + (locals.var_nvtmd * assign19210_e37189_d_n4))), (-(locals.var_nvtmd * assign19210_e37189_d_n5)), (-(locals.var_nvtmd * assign19210_e37189_d_n6)), (-(locals.var_nvtmd * assign19210_e37189_d_n7)), (-(locals.var_nvtmd * assign19210_e37189_d_n8)), (-(locals.var_nvtmd * assign19210_e37189_d_n9)), (-(locals.var_nvtmd * assign19210_e37189_d_n10)), (-(locals.var_nvtmd * assign19210_e37189_d_n11)), (-(locals.var_nvtmd * assign19210_e37189_d_n13)), (-(locals.var_nvtmd * assign19210_e37189_d_n14)),)
    } else {
        (locals.var_vjdmrev, locals.var_vjdmrev_dn0, locals.var_vjdmrev_dn2, locals.var_vjdmrev_dn3, locals.var_vjdmrev_dn4, locals.var_vjdmrev_dn5, locals.var_vjdmrev_dn6, locals.var_vjdmrev_dn7, locals.var_vjdmrev_dn8, locals.var_vjdmrev_dn9, locals.var_vjdmrev_dn10, locals.var_vjdmrev_dn11, locals.var_vjdmrev_dn13, locals.var_vjdmrev_dn14,)
    }
};
        locals.var_vjdmrev = assign19210_e37193;
        locals.var_vjdmrev_dn0 = assign19210_e37193_d_n0;
        locals.var_vjdmrev_dn2 = assign19210_e37193_d_n2;
        locals.var_vjdmrev_dn3 = assign19210_e37193_d_n3;
        locals.var_vjdmrev_dn4 = assign19210_e37193_d_n4;
        locals.var_vjdmrev_dn5 = assign19210_e37193_d_n5;
        locals.var_vjdmrev_dn6 = assign19210_e37193_d_n6;
        locals.var_vjdmrev_dn7 = assign19210_e37193_d_n7;
        locals.var_vjdmrev_dn8 = assign19210_e37193_d_n8;
        locals.var_vjdmrev_dn9 = assign19210_e37193_d_n9;
        locals.var_vjdmrev_dn10 = assign19210_e37193_d_n10;
        locals.var_vjdmrev_dn11 = assign19210_e37193_d_n11;
        locals.var_vjdmrev_dn13 = assign19210_e37193_d_n13;
        locals.var_vjdmrev_dn14 = assign19210_e37193_d_n14;

        let (assign19220_e37207, assign19220_e37207_d_n0, assign19220_e37207_d_n2, assign19220_e37207_d_n3, assign19220_e37207_d_n4, assign19220_e37207_d_n5, assign19220_e37207_d_n6, assign19220_e37207_d_n7, assign19220_e37207_d_n8, assign19220_e37207_d_n9, assign19220_e37207_d_n10, assign19220_e37207_d_n11, assign19220_e37207_d_n13, assign19220_e37207_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19220_e37200: f64 = (p.p1627 + locals.var_vjdmrev);
        let assign19220_e37201: f64 = (-assign19220_e37200);
        let assign19220_e37203: f64 = (assign19220_e37201 / locals.var_nvtmd);
        let assign19220_e37204: f64 = { let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign19220_e37205: f64 = (p.p1629 * assign19220_e37204);
        (assign19220_e37205, (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn0) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn2) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn3) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-locals.var_vjdmrev_dn4) * locals.var_nvtmd) - (assign19220_e37201 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn5) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn6) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn7) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn8) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn9) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn10) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn11) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn13) / locals.var_nvtmd))), (p.p1629 * ({ let limited_exp_arg = assign19220_e37203; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn14) / locals.var_nvtmd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign19220_e37207;
        locals.var_t1_dn0 = assign19220_e37207_d_n0;
        locals.var_t1_dn2 = assign19220_e37207_d_n2;
        locals.var_t1_dn3 = assign19220_e37207_d_n3;
        locals.var_t1_dn4 = assign19220_e37207_d_n4;
        locals.var_t1_dn5 = assign19220_e37207_d_n5;
        locals.var_t1_dn6 = assign19220_e37207_d_n6;
        locals.var_t1_dn7 = assign19220_e37207_d_n7;
        locals.var_t1_dn8 = assign19220_e37207_d_n8;
        locals.var_t1_dn9 = assign19220_e37207_d_n9;
        locals.var_t1_dn10 = assign19220_e37207_d_n10;
        locals.var_t1_dn11 = assign19220_e37207_d_n11;
        locals.var_t1_dn13 = assign19220_e37207_d_n13;
        locals.var_t1_dn14 = assign19220_e37207_d_n14;

        let (assign19230_e37217, assign19230_e37217_d_n0, assign19230_e37217_d_n2, assign19230_e37217_d_n3, assign19230_e37217_d_n4, assign19230_e37217_d_n5, assign19230_e37217_d_n6, assign19230_e37217_d_n7, assign19230_e37217_d_n8, assign19230_e37217_d_n9, assign19230_e37217_d_n10, assign19230_e37217_d_n11, assign19230_e37217_d_n13, assign19230_e37217_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19230_e37214: f64 = (1.0 + locals.var_t1);
        let assign19230_e37215: f64 = (locals.var_isbd * assign19230_e37214);
        (assign19230_e37215, ((locals.var_isbd_dn0 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn0)), ((locals.var_isbd_dn2 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn2)), ((locals.var_isbd_dn3 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn3)), ((locals.var_isbd_dn4 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn4)), ((locals.var_isbd_dn5 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn5)), ((locals.var_isbd_dn6 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn6)), ((locals.var_isbd_dn7 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn7)), ((locals.var_isbd_dn8 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn8)), ((locals.var_isbd_dn9 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn9)), ((locals.var_isbd_dn10 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn10)), ((locals.var_isbd_dn11 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn11)), ((locals.var_isbd_dn13 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn13)), ((locals.var_isbd_dn14 * assign19230_e37214) + (locals.var_isbd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ivjdmrev, locals.var_ivjdmrev_dn0, locals.var_ivjdmrev_dn2, locals.var_ivjdmrev_dn3, locals.var_ivjdmrev_dn4, locals.var_ivjdmrev_dn5, locals.var_ivjdmrev_dn6, locals.var_ivjdmrev_dn7, locals.var_ivjdmrev_dn8, locals.var_ivjdmrev_dn9, locals.var_ivjdmrev_dn10, locals.var_ivjdmrev_dn11, locals.var_ivjdmrev_dn13, locals.var_ivjdmrev_dn14,)
    }
};
        locals.var_ivjdmrev = assign19230_e37217;
        locals.var_ivjdmrev_dn0 = assign19230_e37217_d_n0;
        locals.var_ivjdmrev_dn2 = assign19230_e37217_d_n2;
        locals.var_ivjdmrev_dn3 = assign19230_e37217_d_n3;
        locals.var_ivjdmrev_dn4 = assign19230_e37217_d_n4;
        locals.var_ivjdmrev_dn5 = assign19230_e37217_d_n5;
        locals.var_ivjdmrev_dn6 = assign19230_e37217_d_n6;
        locals.var_ivjdmrev_dn7 = assign19230_e37217_d_n7;
        locals.var_ivjdmrev_dn8 = assign19230_e37217_d_n8;
        locals.var_ivjdmrev_dn9 = assign19230_e37217_d_n9;
        locals.var_ivjdmrev_dn10 = assign19230_e37217_d_n10;
        locals.var_ivjdmrev_dn11 = assign19230_e37217_d_n11;
        locals.var_ivjdmrev_dn13 = assign19230_e37217_d_n13;
        locals.var_ivjdmrev_dn14 = assign19230_e37217_d_n14;

        let (assign19240_e37228, assign19240_e37228_d_n0, assign19240_e37228_d_n2, assign19240_e37228_d_n3, assign19240_e37228_d_n4, assign19240_e37228_d_n5, assign19240_e37228_d_n6, assign19240_e37228_d_n7, assign19240_e37228_d_n8, assign19240_e37228_d_n9, assign19240_e37228_d_n10, assign19240_e37228_d_n11, assign19240_e37228_d_n13, assign19240_e37228_d_n14,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard352 != 0.0)) {
        let assign19240_e37222: f64 = (-locals.var_isbd);
        let assign19240_e37224: f64 = (assign19240_e37222 * locals.var_t1);
        let assign19240_e37226: f64 = (assign19240_e37224 / locals.var_nvtmd);
        (assign19240_e37226, ((((-locals.var_isbd_dn0) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn0)) / locals.var_nvtmd), ((((-locals.var_isbd_dn2) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn2)) / locals.var_nvtmd), ((((-locals.var_isbd_dn3) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn3)) / locals.var_nvtmd), ((((((-locals.var_isbd_dn4) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn4)) * locals.var_nvtmd) - (assign19240_e37224 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)), ((((-locals.var_isbd_dn5) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn5)) / locals.var_nvtmd), ((((-locals.var_isbd_dn6) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn6)) / locals.var_nvtmd), ((((-locals.var_isbd_dn7) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn7)) / locals.var_nvtmd), ((((-locals.var_isbd_dn8) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn8)) / locals.var_nvtmd), ((((-locals.var_isbd_dn9) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn9)) / locals.var_nvtmd), ((((-locals.var_isbd_dn10) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn10)) / locals.var_nvtmd), ((((-locals.var_isbd_dn11) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn11)) / locals.var_nvtmd), ((((-locals.var_isbd_dn13) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn13)) / locals.var_nvtmd), ((((-locals.var_isbd_dn14) * locals.var_t1) + (assign19240_e37222 * locals.var_t1_dn14)) / locals.var_nvtmd),)
    } else {
        (locals.var_dslprev, locals.var_dslprev_dn0, locals.var_dslprev_dn2, locals.var_dslprev_dn3, locals.var_dslprev_dn4, locals.var_dslprev_dn5, locals.var_dslprev_dn6, locals.var_dslprev_dn7, locals.var_dslprev_dn8, locals.var_dslprev_dn9, locals.var_dslprev_dn10, locals.var_dslprev_dn11, locals.var_dslprev_dn13, locals.var_dslprev_dn14,)
    }
};
        locals.var_dslprev = assign19240_e37228;
        locals.var_dslprev_dn0 = assign19240_e37228_d_n0;
        locals.var_dslprev_dn2 = assign19240_e37228_d_n2;
        locals.var_dslprev_dn3 = assign19240_e37228_d_n3;
        locals.var_dslprev_dn4 = assign19240_e37228_d_n4;
        locals.var_dslprev_dn5 = assign19240_e37228_d_n5;
        locals.var_dslprev_dn6 = assign19240_e37228_d_n6;
        locals.var_dslprev_dn7 = assign19240_e37228_d_n7;
        locals.var_dslprev_dn8 = assign19240_e37228_d_n8;
        locals.var_dslprev_dn9 = assign19240_e37228_d_n9;
        locals.var_dslprev_dn10 = assign19240_e37228_d_n10;
        locals.var_dslprev_dn11 = assign19240_e37228_d_n11;
        locals.var_dslprev_dn13 = assign19240_e37228_d_n13;
        locals.var_dslprev_dn14 = assign19240_e37228_d_n14;

        let (assign19250_e37234, assign19250_e37234_d_n4,) = {
    if (locals.var_guard350 != 0.0) {
        let assign19250_e37232: f64 = (locals.var_cjs_t * p.p11);
        (assign19250_e37232, (locals.var_cjs_t_dn4 * p.p11),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn4,)
    }
};
        locals.var_czbs = assign19250_e37234;
        locals.var_czbs_dn4 = assign19250_e37234_d_n4;

        let (assign19260_e37240, assign19260_e37240_d_n4,) = {
    if (locals.var_guard350 != 0.0) {
        let assign19260_e37238: f64 = (locals.var_cjsws_t * p.p13);
        (assign19260_e37238, (locals.var_cjsws_t_dn4 * p.p13),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn4,)
    }
};
        locals.var_czbssw = assign19260_e37240;
        locals.var_czbssw_dn4 = assign19260_e37240_d_n4;

        let (assign19270_e37248, assign19270_e37248_d_n4,) = {
    if (locals.var_guard350 != 0.0) {
        let assign19270_e37244: f64 = (locals.var_cjswgs_t * locals.var_weff0);
        let assign19270_e37246: f64 = (assign19270_e37244 * locals.var_nfintotal);
        (assign19270_e37246, ((locals.var_cjswgs_t_dn4 * locals.var_weff0) * locals.var_nfintotal),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn4,)
    }
};
        locals.var_czbsswg = assign19270_e37248;
        locals.var_czbsswg_dn4 = assign19270_e37248_d_n4;

        let (assign19280_e37254, assign19280_e37254_d_n4,) = {
    if (locals.var_guard350 != 0.0) {
        let assign19280_e37252: f64 = (locals.var_cjd_t * p.p12);
        (assign19280_e37252, (locals.var_cjd_t_dn4 * p.p12),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn4,)
    }
};
        locals.var_czbd = assign19280_e37254;
        locals.var_czbd_dn4 = assign19280_e37254_d_n4;

        let (assign19290_e37260, assign19290_e37260_d_n4,) = {
    if (locals.var_guard350 != 0.0) {
        let assign19290_e37258: f64 = (locals.var_cjswd_t * p.p14);
        (assign19290_e37258, (locals.var_cjswd_t_dn4 * p.p14),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn4,)
    }
};
        locals.var_czbdsw = assign19290_e37260;
        locals.var_czbdsw_dn4 = assign19290_e37260_d_n4;

        let (assign19300_e37268, assign19300_e37268_d_n4,) = {
    if (locals.var_guard350 != 0.0) {
        let assign19300_e37264: f64 = (locals.var_cjswgd_t * locals.var_weff0);
        let assign19300_e37266: f64 = (assign19300_e37264 * locals.var_nfintotal);
        (assign19300_e37266, ((locals.var_cjswgd_t_dn4 * locals.var_weff0) * locals.var_nfintotal),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn4,)
    }
};
        locals.var_czbdswg = assign19300_e37268;
        locals.var_czbdswg_dn4 = assign19300_e37268_d_n4;

        let assign19310_e37271: f64 = if p.p1602 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard353 = assign19310_e37271;

        let (assign19320_e37287, assign19320_e37287_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign19320_e37279: f64 = (1.0 / p.p1602);
        let assign19320_e37282: f64 = (1.0 / p.p1596);
        let assign19320_e37283: f64 = (assign19320_e37279).powf(assign19320_e37282);
        let assign19320_e37284: f64 = (1.0 - assign19320_e37283);
        let assign19320_e37285: f64 = (locals.var_pbs_t * assign19320_e37284);
        (assign19320_e37285, (locals.var_pbs_t_dn4 * assign19320_e37284),)
    } else {
        (locals.var_vec1s, locals.var_vec1s_dn4,)
    }
};
        locals.var_vec1s = assign19320_e37287;
        locals.var_vec1s_dn4 = assign19320_e37287_d_n4;

        let (assign19330_e37310, assign19330_e37310_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard353 != 0.0)) {
        let assign19330_e37293: f64 = (locals.var_pbs_t * p.p1602);
        let assign19330_e37295: f64 = (assign19330_e37293 * p.p1608);
        let assign19330_e37297: f64 = (assign19330_e37295 / p.p1596);
        let assign19330_e37301: f64 = (locals.var_vec1s / locals.var_pbs_t);
        let assign19330_e37302: f64 = (1.0 - assign19330_e37301);
        let assign19330_e37305: f64 = (1.0 + p.p1596);
        let assign19330_e37306: f64 = (-assign19330_e37305);
        let assign19330_e37307: f64 = (assign19330_e37302).powf(assign19330_e37306);
        let assign19330_e37308: f64 = (assign19330_e37297 / assign19330_e37307);
        (assign19330_e37308, ((((((locals.var_pbs_t_dn4 * p.p1602) * p.p1608) / p.p1596) * assign19330_e37307) - (assign19330_e37297 * if 0.0 == 0.0 && ((assign19330_e37306) as f64).is_finite() && ((assign19330_e37306) as f64).fract() == 0.0 { if assign19330_e37306 == 0.0 { 0.0 } else { (assign19330_e37306 * ((assign19330_e37302).powf(assign19330_e37306 - 1.0) * (-(((locals.var_vec1s_dn4 * locals.var_pbs_t) - (locals.var_vec1s * locals.var_pbs_t_dn4)) / (locals.var_pbs_t * locals.var_pbs_t))))) } } else { (assign19330_e37307 * (assign19330_e37306 * ((-(((locals.var_vec1s_dn4 * locals.var_pbs_t) - (locals.var_vec1s * locals.var_pbs_t_dn4)) / (locals.var_pbs_t * locals.var_pbs_t))) / assign19330_e37302))) })) / (assign19330_e37307 * assign19330_e37307)),)
    } else {
        (locals.var_pb21s, locals.var_pb21s_dn4,)
    }
};
        locals.var_pb21s = assign19330_e37310;
        locals.var_pb21s_dn4 = assign19330_e37310_d_n4;

        let assign19340_e37313: f64 = if p.p1604 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard354 = assign19340_e37313;

        let (assign19350_e37329, assign19350_e37329_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard354 != 0.0)) {
        let assign19350_e37321: f64 = (1.0 / p.p1604);
        let assign19350_e37324: f64 = (1.0 / p.p1598);
        let assign19350_e37325: f64 = (assign19350_e37321).powf(assign19350_e37324);
        let assign19350_e37326: f64 = (1.0 - assign19350_e37325);
        let assign19350_e37327: f64 = (locals.var_pbsws_t * assign19350_e37326);
        (assign19350_e37327, (locals.var_pbsws_t_dn4 * assign19350_e37326),)
    } else {
        (locals.var_vec2s, locals.var_vec2s_dn4,)
    }
};
        locals.var_vec2s = assign19350_e37329;
        locals.var_vec2s_dn4 = assign19350_e37329_d_n4;

        let (assign19360_e37352, assign19360_e37352_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard354 != 0.0)) {
        let assign19360_e37335: f64 = (locals.var_pbsws_t * p.p1604);
        let assign19360_e37337: f64 = (assign19360_e37335 * p.p1610);
        let assign19360_e37339: f64 = (assign19360_e37337 / p.p1598);
        let assign19360_e37343: f64 = (locals.var_vec2s / locals.var_pbsws_t);
        let assign19360_e37344: f64 = (1.0 - assign19360_e37343);
        let assign19360_e37347: f64 = (1.0 + p.p1598);
        let assign19360_e37348: f64 = (-assign19360_e37347);
        let assign19360_e37349: f64 = (assign19360_e37344).powf(assign19360_e37348);
        let assign19360_e37350: f64 = (assign19360_e37339 / assign19360_e37349);
        (assign19360_e37350, ((((((locals.var_pbsws_t_dn4 * p.p1604) * p.p1610) / p.p1598) * assign19360_e37349) - (assign19360_e37339 * if 0.0 == 0.0 && ((assign19360_e37348) as f64).is_finite() && ((assign19360_e37348) as f64).fract() == 0.0 { if assign19360_e37348 == 0.0 { 0.0 } else { (assign19360_e37348 * ((assign19360_e37344).powf(assign19360_e37348 - 1.0) * (-(((locals.var_vec2s_dn4 * locals.var_pbsws_t) - (locals.var_vec2s * locals.var_pbsws_t_dn4)) / (locals.var_pbsws_t * locals.var_pbsws_t))))) } } else { (assign19360_e37349 * (assign19360_e37348 * ((-(((locals.var_vec2s_dn4 * locals.var_pbsws_t) - (locals.var_vec2s * locals.var_pbsws_t_dn4)) / (locals.var_pbsws_t * locals.var_pbsws_t))) / assign19360_e37344))) })) / (assign19360_e37349 * assign19360_e37349)),)
    } else {
        (locals.var_pb22s, locals.var_pb22s_dn4,)
    }
};
        locals.var_pb22s = assign19360_e37352;
        locals.var_pb22s_dn4 = assign19360_e37352_d_n4;

        let assign19370_e37355: f64 = if p.p1606 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard355 = assign19370_e37355;

        let (assign19380_e37371, assign19380_e37371_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign19380_e37363: f64 = (1.0 / p.p1606);
        let assign19380_e37366: f64 = (1.0 / p.p1600);
        let assign19380_e37367: f64 = (assign19380_e37363).powf(assign19380_e37366);
        let assign19380_e37368: f64 = (1.0 - assign19380_e37367);
        let assign19380_e37369: f64 = (locals.var_pbswgs_t * assign19380_e37368);
        (assign19380_e37369, (locals.var_pbswgs_t_dn4 * assign19380_e37368),)
    } else {
        (locals.var_vec3s, locals.var_vec3s_dn4,)
    }
};
        locals.var_vec3s = assign19380_e37371;
        locals.var_vec3s_dn4 = assign19380_e37371_d_n4;

        let (assign19390_e37394, assign19390_e37394_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard355 != 0.0)) {
        let assign19390_e37377: f64 = (locals.var_pbswgs_t * p.p1606);
        let assign19390_e37379: f64 = (assign19390_e37377 * p.p1612);
        let assign19390_e37381: f64 = (assign19390_e37379 / p.p1600);
        let assign19390_e37385: f64 = (locals.var_vec3s / locals.var_pbswgs_t);
        let assign19390_e37386: f64 = (1.0 - assign19390_e37385);
        let assign19390_e37389: f64 = (1.0 + p.p1600);
        let assign19390_e37390: f64 = (-assign19390_e37389);
        let assign19390_e37391: f64 = (assign19390_e37386).powf(assign19390_e37390);
        let assign19390_e37392: f64 = (assign19390_e37381 / assign19390_e37391);
        (assign19390_e37392, ((((((locals.var_pbswgs_t_dn4 * p.p1606) * p.p1612) / p.p1600) * assign19390_e37391) - (assign19390_e37381 * if 0.0 == 0.0 && ((assign19390_e37390) as f64).is_finite() && ((assign19390_e37390) as f64).fract() == 0.0 { if assign19390_e37390 == 0.0 { 0.0 } else { (assign19390_e37390 * ((assign19390_e37386).powf(assign19390_e37390 - 1.0) * (-(((locals.var_vec3s_dn4 * locals.var_pbswgs_t) - (locals.var_vec3s * locals.var_pbswgs_t_dn4)) / (locals.var_pbswgs_t * locals.var_pbswgs_t))))) } } else { (assign19390_e37391 * (assign19390_e37390 * ((-(((locals.var_vec3s_dn4 * locals.var_pbswgs_t) - (locals.var_vec3s * locals.var_pbswgs_t_dn4)) / (locals.var_pbswgs_t * locals.var_pbswgs_t))) / assign19390_e37386))) })) / (assign19390_e37391 * assign19390_e37391)),)
    } else {
        (locals.var_pb23s, locals.var_pb23s_dn4,)
    }
};
        locals.var_pb23s = assign19390_e37394;
        locals.var_pb23s_dn4 = assign19390_e37394_d_n4;

        let assign19400_e37397: f64 = if p.p1603 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard356 = assign19400_e37397;

        let (assign19410_e37413, assign19410_e37413_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard356 != 0.0)) {
        let assign19410_e37405: f64 = (1.0 / p.p1603);
        let assign19410_e37408: f64 = (1.0 / p.p1597);
        let assign19410_e37409: f64 = (assign19410_e37405).powf(assign19410_e37408);
        let assign19410_e37410: f64 = (1.0 - assign19410_e37409);
        let assign19410_e37411: f64 = (locals.var_pbd_t * assign19410_e37410);
        (assign19410_e37411, (locals.var_pbd_t_dn4 * assign19410_e37410),)
    } else {
        (locals.var_vec1d, locals.var_vec1d_dn4,)
    }
};
        locals.var_vec1d = assign19410_e37413;
        locals.var_vec1d_dn4 = assign19410_e37413_d_n4;

        let (assign19420_e37436, assign19420_e37436_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard356 != 0.0)) {
        let assign19420_e37419: f64 = (locals.var_pbd_t * p.p1603);
        let assign19420_e37421: f64 = (assign19420_e37419 * p.p1609);
        let assign19420_e37423: f64 = (assign19420_e37421 / p.p1597);
        let assign19420_e37427: f64 = (locals.var_vec1d / locals.var_pbd_t);
        let assign19420_e37428: f64 = (1.0 - assign19420_e37427);
        let assign19420_e37431: f64 = (1.0 + p.p1597);
        let assign19420_e37432: f64 = (-assign19420_e37431);
        let assign19420_e37433: f64 = (assign19420_e37428).powf(assign19420_e37432);
        let assign19420_e37434: f64 = (assign19420_e37423 / assign19420_e37433);
        (assign19420_e37434, ((((((locals.var_pbd_t_dn4 * p.p1603) * p.p1609) / p.p1597) * assign19420_e37433) - (assign19420_e37423 * if 0.0 == 0.0 && ((assign19420_e37432) as f64).is_finite() && ((assign19420_e37432) as f64).fract() == 0.0 { if assign19420_e37432 == 0.0 { 0.0 } else { (assign19420_e37432 * ((assign19420_e37428).powf(assign19420_e37432 - 1.0) * (-(((locals.var_vec1d_dn4 * locals.var_pbd_t) - (locals.var_vec1d * locals.var_pbd_t_dn4)) / (locals.var_pbd_t * locals.var_pbd_t))))) } } else { (assign19420_e37433 * (assign19420_e37432 * ((-(((locals.var_vec1d_dn4 * locals.var_pbd_t) - (locals.var_vec1d * locals.var_pbd_t_dn4)) / (locals.var_pbd_t * locals.var_pbd_t))) / assign19420_e37428))) })) / (assign19420_e37433 * assign19420_e37433)),)
    } else {
        (locals.var_pb21d, locals.var_pb21d_dn4,)
    }
};
        locals.var_pb21d = assign19420_e37436;
        locals.var_pb21d_dn4 = assign19420_e37436_d_n4;

        let assign19430_e37439: f64 = if p.p1605 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard357 = assign19430_e37439;

        let (assign19440_e37455, assign19440_e37455_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign19440_e37447: f64 = (1.0 / p.p1605);
        let assign19440_e37450: f64 = (1.0 / p.p1599);
        let assign19440_e37451: f64 = (assign19440_e37447).powf(assign19440_e37450);
        let assign19440_e37452: f64 = (1.0 - assign19440_e37451);
        let assign19440_e37453: f64 = (locals.var_pbswd_t * assign19440_e37452);
        (assign19440_e37453, (locals.var_pbswd_t_dn4 * assign19440_e37452),)
    } else {
        (locals.var_vec2d, locals.var_vec2d_dn4,)
    }
};
        locals.var_vec2d = assign19440_e37455;
        locals.var_vec2d_dn4 = assign19440_e37455_d_n4;

        let (assign19450_e37478, assign19450_e37478_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard357 != 0.0)) {
        let assign19450_e37461: f64 = (locals.var_pbswd_t * p.p1605);
        let assign19450_e37463: f64 = (assign19450_e37461 * p.p1611);
        let assign19450_e37465: f64 = (assign19450_e37463 / p.p1599);
        let assign19450_e37469: f64 = (locals.var_vec2d / locals.var_pbswd_t);
        let assign19450_e37470: f64 = (1.0 - assign19450_e37469);
        let assign19450_e37473: f64 = (1.0 + p.p1599);
        let assign19450_e37474: f64 = (-assign19450_e37473);
        let assign19450_e37475: f64 = (assign19450_e37470).powf(assign19450_e37474);
        let assign19450_e37476: f64 = (assign19450_e37465 / assign19450_e37475);
        (assign19450_e37476, ((((((locals.var_pbswd_t_dn4 * p.p1605) * p.p1611) / p.p1599) * assign19450_e37475) - (assign19450_e37465 * if 0.0 == 0.0 && ((assign19450_e37474) as f64).is_finite() && ((assign19450_e37474) as f64).fract() == 0.0 { if assign19450_e37474 == 0.0 { 0.0 } else { (assign19450_e37474 * ((assign19450_e37470).powf(assign19450_e37474 - 1.0) * (-(((locals.var_vec2d_dn4 * locals.var_pbswd_t) - (locals.var_vec2d * locals.var_pbswd_t_dn4)) / (locals.var_pbswd_t * locals.var_pbswd_t))))) } } else { (assign19450_e37475 * (assign19450_e37474 * ((-(((locals.var_vec2d_dn4 * locals.var_pbswd_t) - (locals.var_vec2d * locals.var_pbswd_t_dn4)) / (locals.var_pbswd_t * locals.var_pbswd_t))) / assign19450_e37470))) })) / (assign19450_e37475 * assign19450_e37475)),)
    } else {
        (locals.var_pb22d, locals.var_pb22d_dn4,)
    }
};
        locals.var_pb22d = assign19450_e37478;
        locals.var_pb22d_dn4 = assign19450_e37478_d_n4;

        let assign19460_e37481: f64 = if p.p1607 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard358 = assign19460_e37481;

        let (assign19470_e37497, assign19470_e37497_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard358 != 0.0)) {
        let assign19470_e37489: f64 = (1.0 / p.p1607);
        let assign19470_e37492: f64 = (1.0 / p.p1601);
        let assign19470_e37493: f64 = (assign19470_e37489).powf(assign19470_e37492);
        let assign19470_e37494: f64 = (1.0 - assign19470_e37493);
        let assign19470_e37495: f64 = (locals.var_pbswgd_t * assign19470_e37494);
        (assign19470_e37495, (locals.var_pbswgd_t_dn4 * assign19470_e37494),)
    } else {
        (locals.var_vec3d, locals.var_vec3d_dn4,)
    }
};
        locals.var_vec3d = assign19470_e37497;
        locals.var_vec3d_dn4 = assign19470_e37497_d_n4;

        let (assign19480_e37520, assign19480_e37520_d_n4,) = {
    if ((locals.var_guard350 != 0.0) && (locals.var_guard358 != 0.0)) {
        let assign19480_e37503: f64 = (locals.var_pbswgd_t * p.p1607);
        let assign19480_e37505: f64 = (assign19480_e37503 * p.p1613);
        let assign19480_e37507: f64 = (assign19480_e37505 / p.p1601);
        let assign19480_e37511: f64 = (locals.var_vec3d / locals.var_pbswgd_t);
        let assign19480_e37512: f64 = (1.0 - assign19480_e37511);
        let assign19480_e37515: f64 = (1.0 + p.p1601);
        let assign19480_e37516: f64 = (-assign19480_e37515);
        let assign19480_e37517: f64 = (assign19480_e37512).powf(assign19480_e37516);
        let assign19480_e37518: f64 = (assign19480_e37507 / assign19480_e37517);
        (assign19480_e37518, ((((((locals.var_pbswgd_t_dn4 * p.p1607) * p.p1613) / p.p1601) * assign19480_e37517) - (assign19480_e37507 * if 0.0 == 0.0 && ((assign19480_e37516) as f64).is_finite() && ((assign19480_e37516) as f64).fract() == 0.0 { if assign19480_e37516 == 0.0 { 0.0 } else { (assign19480_e37516 * ((assign19480_e37512).powf(assign19480_e37516 - 1.0) * (-(((locals.var_vec3d_dn4 * locals.var_pbswgd_t) - (locals.var_vec3d * locals.var_pbswgd_t_dn4)) / (locals.var_pbswgd_t * locals.var_pbswgd_t))))) } } else { (assign19480_e37517 * (assign19480_e37516 * ((-(((locals.var_vec3d_dn4 * locals.var_pbswgd_t) - (locals.var_vec3d * locals.var_pbswgd_t_dn4)) / (locals.var_pbswgd_t * locals.var_pbswgd_t))) / assign19480_e37512))) })) / (assign19480_e37517 * assign19480_e37517)),)
    } else {
        (locals.var_pb23d, locals.var_pb23d_dn4,)
    }
};
        locals.var_pb23d = assign19480_e37520;
        locals.var_pb23d_dn4 = assign19480_e37520_d_n4;

        let assign19490_e37523: f64 = (locals.var_eg * locals.var_tratio_m1);
        let assign19490_e37525: f64 = (assign19490_e37523 / locals.var_vtm);
        let assign19490_e37527: f64 = (assign19490_e37525 / locals.var_ntgen_i);
        let assign19490_e37528: f64 = { let limited_exp_arg = assign19490_e37527; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_igentemp = assign19490_e37528;
        locals.var_igentemp_dn4 = ({ let limited_exp_arg = assign19490_e37527; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((((locals.var_eg_dn4 * locals.var_tratio_m1) + (locals.var_eg * locals.var_tratio_m1_dn4)) * locals.var_vtm) - (assign19490_e37523 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)) / locals.var_ntgen_i));

        let assign19500_e37531: f64 = (locals.var_devsign * (nv11 - nv6));
        locals.var_vgs_noswap = assign19500_e37531;
        locals.var_vgs_noswap_dn6 = (-locals.var_devsign);
        locals.var_vgs_noswap_dn11 = locals.var_devsign;

        let assign19510_e37534: f64 = (locals.var_devsign * (nv5 - nv6));
        locals.var_vds_noswap = assign19510_e37534;
        locals.var_vds_noswap_dn5 = locals.var_devsign;
        locals.var_vds_noswap_dn6 = (-locals.var_devsign);

        let assign19520_e37537: f64 = (locals.var_devsign * (nv11 - nv5));
        locals.var_vgd_noswap = assign19520_e37537;
        locals.var_vgd_noswap_dn5 = (-locals.var_devsign);
        locals.var_vgd_noswap_dn11 = locals.var_devsign;

        let assign19530_e37540: f64 = (locals.var_devsign * (nv3 - nv6));
        locals.var_ves_jct = assign19530_e37540;
        locals.var_ves_jct_dn3 = locals.var_devsign;
        locals.var_ves_jct_dn6 = (-locals.var_devsign);

        let assign19540_e37543: f64 = (locals.var_devsign * (nv3 - nv5));
        locals.var_ved_jct = assign19540_e37543;
        locals.var_ved_jct_dn3 = locals.var_devsign;
        locals.var_ved_jct_dn5 = (-locals.var_devsign);

        let assign19550_e37546: f64 = (locals.var_devsign * (nv11 - nv3));
        locals.var_vge = assign19550_e37546;
        locals.var_vge_dn3 = (-locals.var_devsign);
        locals.var_vge_dn11 = locals.var_devsign;

        let assign19560_e37549: f64 = if p.p76 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard359 = assign19560_e37549;

        let (assign19570_e37555, assign19570_e37555_d_n5, assign19570_e37555_d_n10, assign19570_e37555_d_n14,) = {
    if (locals.var_guard359 != 0.0) {
        let assign19570_e37553: f64 = (locals.var_devsign * (nv10 - nv5));
        (assign19570_e37553, (-locals.var_devsign), locals.var_devsign, 0.0,)
    } else {
        (locals.var_vgdrift, locals.var_vgdrift_dn5, locals.var_vgdrift_dn10, locals.var_vgdrift_dn14,)
    }
};
        locals.var_vgdrift = assign19570_e37555;
        locals.var_vgdrift_dn5 = assign19570_e37555_d_n5;
        locals.var_vgdrift_dn10 = assign19570_e37555_d_n10;
        locals.var_vgdrift_dn14 = assign19570_e37555_d_n14;

        let (assign19580_e37561, assign19580_e37561_d_n6, assign19580_e37561_d_n10, assign19580_e37561_d_n13,) = {
    if (locals.var_guard359 != 0.0) {
        let assign19580_e37559: f64 = (locals.var_devsign * (nv10 - nv6));
        (assign19580_e37559, (-locals.var_devsign), locals.var_devsign, 0.0,)
    } else {
        (locals.var_vgdrift_s, locals.var_vgdrift_s_dn6, locals.var_vgdrift_s_dn10, locals.var_vgdrift_s_dn13,)
    }
};
        locals.var_vgdrift_s = assign19580_e37561;
        locals.var_vgdrift_s_dn6 = assign19580_e37561_d_n6;
        locals.var_vgdrift_s_dn10 = assign19580_e37561_d_n10;
        locals.var_vgdrift_s_dn13 = assign19580_e37561_d_n13;

        let (assign19590_e37568, assign19590_e37568_d_n5, assign19590_e37568_d_n10, assign19590_e37568_d_n14,) = {
    if (locals.var_guard359 == 0.0) {
        let assign19590_e37566: f64 = (locals.var_devsign * (nv14 - nv5));
        (assign19590_e37566, (-locals.var_devsign), 0.0, locals.var_devsign,)
    } else {
        (locals.var_vgdrift, locals.var_vgdrift_dn5, locals.var_vgdrift_dn10, locals.var_vgdrift_dn14,)
    }
};
        locals.var_vgdrift = assign19590_e37568;
        locals.var_vgdrift_dn5 = assign19590_e37568_d_n5;
        locals.var_vgdrift_dn10 = assign19590_e37568_d_n10;
        locals.var_vgdrift_dn14 = assign19590_e37568_d_n14;

        let (assign19600_e37575, assign19600_e37575_d_n6, assign19600_e37575_d_n10, assign19600_e37575_d_n13,) = {
    if (locals.var_guard359 == 0.0) {
        let assign19600_e37573: f64 = (locals.var_devsign * (nv13 - nv6));
        (assign19600_e37573, (-locals.var_devsign), 0.0, locals.var_devsign,)
    } else {
        (locals.var_vgdrift_s, locals.var_vgdrift_s_dn6, locals.var_vgdrift_s_dn10, locals.var_vgdrift_s_dn13,)
    }
};
        locals.var_vgdrift_s = assign19600_e37575;
        locals.var_vgdrift_s_dn6 = assign19600_e37575_d_n6;
        locals.var_vgdrift_s_dn10 = assign19600_e37575_d_n10;
        locals.var_vgdrift_s_dn13 = assign19600_e37575_d_n13;

        locals.var_sigvds = 1.0;

        let assign19620_e37579: f64 = if locals.var_vds_noswap < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard360 = assign19620_e37579;

    }

    pub(super) fn stamp_transient_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19630_e37584,) = {
    if (locals.var_guard360 != 0.0) {
        let assign19630_e37582: f64 = (-1.0);
        (assign19630_e37582,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign19630_e37584;

        let (assign19640_e37590, assign19640_e37590_d_n5, assign19640_e37590_d_n6, assign19640_e37590_d_n11,) = {
    if (locals.var_guard360 != 0.0) {
        let assign19640_e37588: f64 = (locals.var_vgs_noswap - locals.var_vds_noswap);
        (assign19640_e37588, (-locals.var_vds_noswap_dn5), (locals.var_vgs_noswap_dn6 - locals.var_vds_noswap_dn6), locals.var_vgs_noswap_dn11,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn5, locals.var_vgs_dn6, locals.var_vgs_dn11,)
    }
};
        locals.var_vgs = assign19640_e37590;
        locals.var_vgs_dn5 = assign19640_e37590_d_n5;
        locals.var_vgs_dn6 = assign19640_e37590_d_n6;
        locals.var_vgs_dn11 = assign19640_e37590_d_n11;

        let (assign19650_e37597, assign19650_e37597_d_n5, assign19650_e37597_d_n6,) = {
    if (locals.var_guard360 != 0.0) {
        let assign19650_e37593: f64 = (-1.0);
        let assign19650_e37595: f64 = (assign19650_e37593 * locals.var_vds_noswap);
        (assign19650_e37595, (assign19650_e37593 * locals.var_vds_noswap_dn5), (assign19650_e37593 * locals.var_vds_noswap_dn6),)
    } else {
        (locals.var_vds, locals.var_vds_dn5, locals.var_vds_dn6,)
    }
};
        locals.var_vds = assign19650_e37597;
        locals.var_vds_dn5 = assign19650_e37597_d_n5;
        locals.var_vds_dn6 = assign19650_e37597_d_n6;

        let (assign19660_e37601, assign19660_e37601_d_n3, assign19660_e37601_d_n5, assign19660_e37601_d_n6,) = {
    if (locals.var_guard360 != 0.0) {
        (locals.var_ved_jct, locals.var_ved_jct_dn3, locals.var_ved_jct_dn5, 0.0,)
    } else {
        (locals.var_ves, locals.var_ves_dn3, locals.var_ves_dn5, locals.var_ves_dn6,)
    }
};
        locals.var_ves = assign19660_e37601;
        locals.var_ves_dn3 = assign19660_e37601_d_n3;
        locals.var_ves_dn5 = assign19660_e37601_d_n5;
        locals.var_ves_dn6 = assign19660_e37601_d_n6;

        let (assign19670_e37606, assign19670_e37606_d_n5, assign19670_e37606_d_n6, assign19670_e37606_d_n11,) = {
    if (locals.var_guard360 == 0.0) {
        (locals.var_vgs_noswap, 0.0, locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn11,)
    } else {
        (locals.var_vgs, locals.var_vgs_dn5, locals.var_vgs_dn6, locals.var_vgs_dn11,)
    }
};
        locals.var_vgs = assign19670_e37606;
        locals.var_vgs_dn5 = assign19670_e37606_d_n5;
        locals.var_vgs_dn6 = assign19670_e37606_d_n6;
        locals.var_vgs_dn11 = assign19670_e37606_d_n11;

        let (assign19680_e37611, assign19680_e37611_d_n5, assign19680_e37611_d_n6,) = {
    if (locals.var_guard360 == 0.0) {
        (locals.var_vds_noswap, locals.var_vds_noswap_dn5, locals.var_vds_noswap_dn6,)
    } else {
        (locals.var_vds, locals.var_vds_dn5, locals.var_vds_dn6,)
    }
};
        locals.var_vds = assign19680_e37611;
        locals.var_vds_dn5 = assign19680_e37611_d_n5;
        locals.var_vds_dn6 = assign19680_e37611_d_n6;

        let (assign19690_e37616, assign19690_e37616_d_n3, assign19690_e37616_d_n5, assign19690_e37616_d_n6,) = {
    if (locals.var_guard360 == 0.0) {
        (locals.var_ves_jct, locals.var_ves_jct_dn3, 0.0, locals.var_ves_jct_dn6,)
    } else {
        (locals.var_ves, locals.var_ves_dn3, locals.var_ves_dn5, locals.var_ves_dn6,)
    }
};
        locals.var_ves = assign19690_e37616;
        locals.var_ves_dn3 = assign19690_e37616_d_n3;
        locals.var_ves_dn5 = assign19690_e37616_d_n5;
        locals.var_ves_dn6 = assign19690_e37616_d_n6;

        let assign19700_e37619: f64 = (locals.var_vgs - locals.var_deltaphi);
        locals.var_vgsfb = assign19700_e37619;
        locals.var_vgsfb_dn0 = (-locals.var_deltaphi_dn0);
        locals.var_vgsfb_dn2 = (-locals.var_deltaphi_dn2);
        locals.var_vgsfb_dn3 = (-locals.var_deltaphi_dn3);
        locals.var_vgsfb_dn4 = (-locals.var_deltaphi_dn4);
        locals.var_vgsfb_dn5 = (locals.var_vgs_dn5 - locals.var_deltaphi_dn5);
        locals.var_vgsfb_dn6 = (locals.var_vgs_dn6 - locals.var_deltaphi_dn6);
        locals.var_vgsfb_dn7 = (-locals.var_deltaphi_dn7);
        locals.var_vgsfb_dn8 = (-locals.var_deltaphi_dn8);
        locals.var_vgsfb_dn9 = (-locals.var_deltaphi_dn9);
        locals.var_vgsfb_dn10 = (-locals.var_deltaphi_dn10);
        locals.var_vgsfb_dn11 = (locals.var_vgs_dn11 - locals.var_deltaphi_dn11);
        locals.var_vgsfb_dn13 = (-locals.var_deltaphi_dn13);
        locals.var_vgsfb_dn14 = (-locals.var_deltaphi_dn14);

        let assign19710_e37622: f64 = (locals.var_vds * locals.var_vds);
        let assign19710_e37624: f64 = (assign19710_e37622 + 0.01);
        let assign19710_e37625: f64 = (assign19710_e37624).sqrt();
        let assign19710_e37627: f64 = (assign19710_e37625 - 0.1);
        locals.var_vdsx = assign19710_e37627;
        locals.var_vdsx_dn5 = (((locals.var_vds_dn5 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn5)) / (2.0 * assign19710_e37625));
        locals.var_vdsx_dn6 = (((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign19710_e37625));

        let assign19720_e37630: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard361 = assign19720_e37630;

        let (assign19730_e37640, assign19730_e37640_d_n3, assign19730_e37640_d_n5, assign19730_e37640_d_n6,) = {
    if (locals.var_guard361 != 0.0) {
        let assign19730_e37636: f64 = (locals.var_vds - locals.var_vdsx);
        let assign19730_e37637: f64 = (0.5 * assign19730_e37636);
        let assign19730_e37638: f64 = (locals.var_ves - assign19730_e37637);
        (assign19730_e37638, locals.var_ves_dn3, (locals.var_ves_dn5 - (0.5 * (locals.var_vds_dn5 - locals.var_vdsx_dn5))), (locals.var_ves_dn6 - (0.5 * (locals.var_vds_dn6 - locals.var_vdsx_dn6))),)
    } else {
        (locals.var_vesx, locals.var_vesx_dn3, locals.var_vesx_dn5, locals.var_vesx_dn6,)
    }
};
        locals.var_vesx = assign19730_e37640;
        locals.var_vesx_dn3 = assign19730_e37640_d_n3;
        locals.var_vesx_dn5 = assign19730_e37640_d_n5;
        locals.var_vesx_dn6 = assign19730_e37640_d_n6;

        let (assign19740_e37646,) = {
    if (locals.var_guard361 != 0.0) {
        let assign19740_e37644: f64 = (0.95 * locals.var_phibe_i);
        (assign19740_e37644,)
    } else {
        (locals.var_vesmax,)
    }
};
        locals.var_vesmax = assign19740_e37646;

        let (assign19750_e37654, assign19750_e37654_d_n0, assign19750_e37654_d_n2, assign19750_e37654_d_n3, assign19750_e37654_d_n4, assign19750_e37654_d_n5, assign19750_e37654_d_n6, assign19750_e37654_d_n7, assign19750_e37654_d_n8, assign19750_e37654_d_n9, assign19750_e37654_d_n10, assign19750_e37654_d_n11, assign19750_e37654_d_n13, assign19750_e37654_d_n14,) = {
    if (locals.var_guard361 != 0.0) {
        let assign19750_e37650: f64 = (locals.var_vesmax - locals.var_vesx);
        let assign19750_e37652: f64 = (assign19750_e37650 - 0.001);
        (assign19750_e37652, 0.0, 0.0, (-locals.var_vesx_dn3), 0.0, (-locals.var_vesx_dn5), (-locals.var_vesx_dn6), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign19750_e37654;
        locals.var_t2_dn0 = assign19750_e37654_d_n0;
        locals.var_t2_dn2 = assign19750_e37654_d_n2;
        locals.var_t2_dn3 = assign19750_e37654_d_n3;
        locals.var_t2_dn4 = assign19750_e37654_d_n4;
        locals.var_t2_dn5 = assign19750_e37654_d_n5;
        locals.var_t2_dn6 = assign19750_e37654_d_n6;
        locals.var_t2_dn7 = assign19750_e37654_d_n7;
        locals.var_t2_dn8 = assign19750_e37654_d_n8;
        locals.var_t2_dn9 = assign19750_e37654_d_n9;
        locals.var_t2_dn10 = assign19750_e37654_d_n10;
        locals.var_t2_dn11 = assign19750_e37654_d_n11;
        locals.var_t2_dn13 = assign19750_e37654_d_n13;
        locals.var_t2_dn14 = assign19750_e37654_d_n14;

        let (assign19760_e37671, assign19760_e37671_d_n0, assign19760_e37671_d_n2, assign19760_e37671_d_n3, assign19760_e37671_d_n4, assign19760_e37671_d_n5, assign19760_e37671_d_n6, assign19760_e37671_d_n7, assign19760_e37671_d_n8, assign19760_e37671_d_n9, assign19760_e37671_d_n10, assign19760_e37671_d_n11, assign19760_e37671_d_n13, assign19760_e37671_d_n14,) = {
    if (locals.var_guard361 != 0.0) {
        let assign19760_e37661: f64 = (locals.var_t2 * locals.var_t2);
        let assign19760_e37664: f64 = (0.004 * locals.var_vesmax);
        let assign19760_e37665: f64 = (assign19760_e37661 + assign19760_e37664);
        let assign19760_e37666: f64 = (assign19760_e37665).sqrt();
        let assign19760_e37667: f64 = (locals.var_t2 + assign19760_e37666);
        let assign19760_e37668: f64 = (0.5 * assign19760_e37667);
        let assign19760_e37669: f64 = (locals.var_vesmax - assign19760_e37668);
        (assign19760_e37669, (-(0.5 * (locals.var_t2_dn0 + (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn2 + (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn13 + (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign19760_e37666))))), (-(0.5 * (locals.var_t2_dn14 + (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign19760_e37666))))),)
    } else {
        (locals.var_veseff, locals.var_veseff_dn0, locals.var_veseff_dn2, locals.var_veseff_dn3, locals.var_veseff_dn4, locals.var_veseff_dn5, locals.var_veseff_dn6, locals.var_veseff_dn7, locals.var_veseff_dn8, locals.var_veseff_dn9, locals.var_veseff_dn10, locals.var_veseff_dn11, locals.var_veseff_dn13, locals.var_veseff_dn14,)
    }
};
        locals.var_veseff = assign19760_e37671;
        locals.var_veseff_dn0 = assign19760_e37671_d_n0;
        locals.var_veseff_dn2 = assign19760_e37671_d_n2;
        locals.var_veseff_dn3 = assign19760_e37671_d_n3;
        locals.var_veseff_dn4 = assign19760_e37671_d_n4;
        locals.var_veseff_dn5 = assign19760_e37671_d_n5;
        locals.var_veseff_dn6 = assign19760_e37671_d_n6;
        locals.var_veseff_dn7 = assign19760_e37671_d_n7;
        locals.var_veseff_dn8 = assign19760_e37671_d_n8;
        locals.var_veseff_dn9 = assign19760_e37671_d_n9;
        locals.var_veseff_dn10 = assign19760_e37671_d_n10;
        locals.var_veseff_dn11 = assign19760_e37671_d_n11;
        locals.var_veseff_dn13 = assign19760_e37671_d_n13;
        locals.var_veseff_dn14 = assign19760_e37671_d_n14;

        let assign19770_e37674: f64 = (0.6 * locals.var_vds_noswap);
        let assign19770_e37676: f64 = (assign19770_e37674 / locals.var_vtm);
        let assign19770_e37677: f64 = (assign19770_e37676).tanh();
        locals.var_t0 = assign19770_e37677;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = ((-((assign19770_e37674 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) / ((assign19770_e37676).cosh() * (assign19770_e37676).cosh()));
        locals.var_t0_dn5 = (((0.6 * locals.var_vds_noswap_dn5) / locals.var_vtm) / ((assign19770_e37676).cosh() * (assign19770_e37676).cosh()));
        locals.var_t0_dn6 = (((0.6 * locals.var_vds_noswap_dn6) / locals.var_vtm) / ((assign19770_e37676).cosh() * (assign19770_e37676).cosh()));
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign19780_e37681: f64 = (0.5 * locals.var_t0);
        let assign19780_e37682: f64 = (0.5 + assign19780_e37681);
        locals.var_wf = assign19780_e37682;
        locals.var_wf_dn0 = (0.5 * locals.var_t0_dn0);
        locals.var_wf_dn2 = (0.5 * locals.var_t0_dn2);
        locals.var_wf_dn3 = (0.5 * locals.var_t0_dn3);
        locals.var_wf_dn4 = (0.5 * locals.var_t0_dn4);
        locals.var_wf_dn5 = (0.5 * locals.var_t0_dn5);
        locals.var_wf_dn6 = (0.5 * locals.var_t0_dn6);
        locals.var_wf_dn7 = (0.5 * locals.var_t0_dn7);
        locals.var_wf_dn8 = (0.5 * locals.var_t0_dn8);
        locals.var_wf_dn9 = (0.5 * locals.var_t0_dn9);
        locals.var_wf_dn10 = (0.5 * locals.var_t0_dn10);
        locals.var_wf_dn11 = (0.5 * locals.var_t0_dn11);
        locals.var_wf_dn13 = (0.5 * locals.var_t0_dn13);
        locals.var_wf_dn14 = (0.5 * locals.var_t0_dn14);

        let assign19790_e37685: f64 = (1.0 - locals.var_wf);
        locals.var_wr_v = assign19790_e37685;
        locals.var_wr_v_dn0 = (-locals.var_wf_dn0);
        locals.var_wr_v_dn2 = (-locals.var_wf_dn2);
        locals.var_wr_v_dn3 = (-locals.var_wf_dn3);
        locals.var_wr_v_dn4 = (-locals.var_wf_dn4);
        locals.var_wr_v_dn5 = (-locals.var_wf_dn5);
        locals.var_wr_v_dn6 = (-locals.var_wf_dn6);
        locals.var_wr_v_dn7 = (-locals.var_wf_dn7);
        locals.var_wr_v_dn8 = (-locals.var_wf_dn8);
        locals.var_wr_v_dn9 = (-locals.var_wf_dn9);
        locals.var_wr_v_dn10 = (-locals.var_wf_dn10);
        locals.var_wr_v_dn11 = (-locals.var_wf_dn11);
        locals.var_wr_v_dn13 = (-locals.var_wf_dn13);
        locals.var_wr_v_dn14 = (-locals.var_wf_dn14);

        let assign19800_e37688: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard362 = assign19800_e37688;

        let (assign19810_e37698, assign19810_e37698_d_n0, assign19810_e37698_d_n2, assign19810_e37698_d_n3, assign19810_e37698_d_n4, assign19810_e37698_d_n5, assign19810_e37698_d_n6, assign19810_e37698_d_n7, assign19810_e37698_d_n8, assign19810_e37698_d_n9, assign19810_e37698_d_n10, assign19810_e37698_d_n11, assign19810_e37698_d_n13, assign19810_e37698_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19810_e37692: f64 = (locals.var_cdscdr_i * locals.var_wr_v);
        let assign19810_e37695: f64 = (locals.var_cdscd_i * locals.var_wf);
        let assign19810_e37696: f64 = (assign19810_e37692 + assign19810_e37695);
        (assign19810_e37696, ((locals.var_cdscdr_i * locals.var_wr_v_dn0) + (locals.var_cdscd_i * locals.var_wf_dn0)), ((locals.var_cdscdr_i * locals.var_wr_v_dn2) + (locals.var_cdscd_i * locals.var_wf_dn2)), ((locals.var_cdscdr_i * locals.var_wr_v_dn3) + (locals.var_cdscd_i * locals.var_wf_dn3)), ((locals.var_cdscdr_i * locals.var_wr_v_dn4) + (locals.var_cdscd_i * locals.var_wf_dn4)), ((locals.var_cdscdr_i * locals.var_wr_v_dn5) + (locals.var_cdscd_i * locals.var_wf_dn5)), ((locals.var_cdscdr_i * locals.var_wr_v_dn6) + (locals.var_cdscd_i * locals.var_wf_dn6)), ((locals.var_cdscdr_i * locals.var_wr_v_dn7) + (locals.var_cdscd_i * locals.var_wf_dn7)), ((locals.var_cdscdr_i * locals.var_wr_v_dn8) + (locals.var_cdscd_i * locals.var_wf_dn8)), ((locals.var_cdscdr_i * locals.var_wr_v_dn9) + (locals.var_cdscd_i * locals.var_wf_dn9)), ((locals.var_cdscdr_i * locals.var_wr_v_dn10) + (locals.var_cdscd_i * locals.var_wf_dn10)), ((locals.var_cdscdr_i * locals.var_wr_v_dn11) + (locals.var_cdscd_i * locals.var_wf_dn11)), ((locals.var_cdscdr_i * locals.var_wr_v_dn13) + (locals.var_cdscd_i * locals.var_wf_dn13)), ((locals.var_cdscdr_i * locals.var_wr_v_dn14) + (locals.var_cdscd_i * locals.var_wf_dn14)),)
    } else {
        (locals.var_cdscd_a, locals.var_cdscd_a_dn0, locals.var_cdscd_a_dn2, locals.var_cdscd_a_dn3, locals.var_cdscd_a_dn4, locals.var_cdscd_a_dn5, locals.var_cdscd_a_dn6, locals.var_cdscd_a_dn7, locals.var_cdscd_a_dn8, locals.var_cdscd_a_dn9, locals.var_cdscd_a_dn10, locals.var_cdscd_a_dn11, locals.var_cdscd_a_dn13, locals.var_cdscd_a_dn14,)
    }
};
        locals.var_cdscd_a = assign19810_e37698;
        locals.var_cdscd_a_dn0 = assign19810_e37698_d_n0;
        locals.var_cdscd_a_dn2 = assign19810_e37698_d_n2;
        locals.var_cdscd_a_dn3 = assign19810_e37698_d_n3;
        locals.var_cdscd_a_dn4 = assign19810_e37698_d_n4;
        locals.var_cdscd_a_dn5 = assign19810_e37698_d_n5;
        locals.var_cdscd_a_dn6 = assign19810_e37698_d_n6;
        locals.var_cdscd_a_dn7 = assign19810_e37698_d_n7;
        locals.var_cdscd_a_dn8 = assign19810_e37698_d_n8;
        locals.var_cdscd_a_dn9 = assign19810_e37698_d_n9;
        locals.var_cdscd_a_dn10 = assign19810_e37698_d_n10;
        locals.var_cdscd_a_dn11 = assign19810_e37698_d_n11;
        locals.var_cdscd_a_dn13 = assign19810_e37698_d_n13;
        locals.var_cdscd_a_dn14 = assign19810_e37698_d_n14;

        let (assign19820_e37708, assign19820_e37708_d_n0, assign19820_e37708_d_n2, assign19820_e37708_d_n3, assign19820_e37708_d_n4, assign19820_e37708_d_n5, assign19820_e37708_d_n6, assign19820_e37708_d_n7, assign19820_e37708_d_n8, assign19820_e37708_d_n9, assign19820_e37708_d_n10, assign19820_e37708_d_n11, assign19820_e37708_d_n13, assign19820_e37708_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19820_e37702: f64 = (locals.var_eta0r_t * locals.var_wr_v);
        let assign19820_e37705: f64 = (locals.var_eta0_t * locals.var_wf);
        let assign19820_e37706: f64 = (assign19820_e37702 + assign19820_e37705);
        (assign19820_e37706, ((locals.var_eta0r_t * locals.var_wr_v_dn0) + ((locals.var_eta0_t_dn0 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn0))), ((locals.var_eta0r_t * locals.var_wr_v_dn2) + ((locals.var_eta0_t_dn2 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn2))), ((locals.var_eta0r_t * locals.var_wr_v_dn3) + ((locals.var_eta0_t_dn3 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn3))), (((locals.var_eta0r_t_dn4 * locals.var_wr_v) + (locals.var_eta0r_t * locals.var_wr_v_dn4)) + ((locals.var_eta0_t_dn4 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn4))), ((locals.var_eta0r_t * locals.var_wr_v_dn5) + ((locals.var_eta0_t_dn5 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn5))), ((locals.var_eta0r_t * locals.var_wr_v_dn6) + ((locals.var_eta0_t_dn6 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn6))), ((locals.var_eta0r_t * locals.var_wr_v_dn7) + ((locals.var_eta0_t_dn7 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn7))), ((locals.var_eta0r_t * locals.var_wr_v_dn8) + ((locals.var_eta0_t_dn8 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn8))), ((locals.var_eta0r_t * locals.var_wr_v_dn9) + ((locals.var_eta0_t_dn9 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn9))), ((locals.var_eta0r_t * locals.var_wr_v_dn10) + ((locals.var_eta0_t_dn10 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn10))), ((locals.var_eta0r_t * locals.var_wr_v_dn11) + ((locals.var_eta0_t_dn11 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn11))), ((locals.var_eta0r_t * locals.var_wr_v_dn13) + ((locals.var_eta0_t_dn13 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn13))), ((locals.var_eta0r_t * locals.var_wr_v_dn14) + ((locals.var_eta0_t_dn14 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_eta0_a, locals.var_eta0_a_dn0, locals.var_eta0_a_dn2, locals.var_eta0_a_dn3, locals.var_eta0_a_dn4, locals.var_eta0_a_dn5, locals.var_eta0_a_dn6, locals.var_eta0_a_dn7, locals.var_eta0_a_dn8, locals.var_eta0_a_dn9, locals.var_eta0_a_dn10, locals.var_eta0_a_dn11, locals.var_eta0_a_dn13, locals.var_eta0_a_dn14,)
    }
};
        locals.var_eta0_a = assign19820_e37708;
        locals.var_eta0_a_dn0 = assign19820_e37708_d_n0;
        locals.var_eta0_a_dn2 = assign19820_e37708_d_n2;
        locals.var_eta0_a_dn3 = assign19820_e37708_d_n3;
        locals.var_eta0_a_dn4 = assign19820_e37708_d_n4;
        locals.var_eta0_a_dn5 = assign19820_e37708_d_n5;
        locals.var_eta0_a_dn6 = assign19820_e37708_d_n6;
        locals.var_eta0_a_dn7 = assign19820_e37708_d_n7;
        locals.var_eta0_a_dn8 = assign19820_e37708_d_n8;
        locals.var_eta0_a_dn9 = assign19820_e37708_d_n9;
        locals.var_eta0_a_dn10 = assign19820_e37708_d_n10;
        locals.var_eta0_a_dn11 = assign19820_e37708_d_n11;
        locals.var_eta0_a_dn13 = assign19820_e37708_d_n13;
        locals.var_eta0_a_dn14 = assign19820_e37708_d_n14;

        let (assign19830_e37718, assign19830_e37718_d_n0, assign19830_e37718_d_n2, assign19830_e37718_d_n3, assign19830_e37718_d_n4, assign19830_e37718_d_n5, assign19830_e37718_d_n6, assign19830_e37718_d_n7, assign19830_e37718_d_n8, assign19830_e37718_d_n9, assign19830_e37718_d_n10, assign19830_e37718_d_n11, assign19830_e37718_d_n13, assign19830_e37718_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19830_e37712: f64 = (locals.var_pdibl1r_i * locals.var_wr_v);
        let assign19830_e37715: f64 = (locals.var_pdibl1_i * locals.var_wf);
        let assign19830_e37716: f64 = (assign19830_e37712 + assign19830_e37715);
        (assign19830_e37716, ((locals.var_pdibl1r_i * locals.var_wr_v_dn0) + (locals.var_pdibl1_i * locals.var_wf_dn0)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn2) + (locals.var_pdibl1_i * locals.var_wf_dn2)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn3) + (locals.var_pdibl1_i * locals.var_wf_dn3)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn4) + (locals.var_pdibl1_i * locals.var_wf_dn4)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn5) + (locals.var_pdibl1_i * locals.var_wf_dn5)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn6) + (locals.var_pdibl1_i * locals.var_wf_dn6)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn7) + (locals.var_pdibl1_i * locals.var_wf_dn7)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn8) + (locals.var_pdibl1_i * locals.var_wf_dn8)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn9) + (locals.var_pdibl1_i * locals.var_wf_dn9)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn10) + (locals.var_pdibl1_i * locals.var_wf_dn10)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn11) + (locals.var_pdibl1_i * locals.var_wf_dn11)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn13) + (locals.var_pdibl1_i * locals.var_wf_dn13)), ((locals.var_pdibl1r_i * locals.var_wr_v_dn14) + (locals.var_pdibl1_i * locals.var_wf_dn14)),)
    } else {
        (locals.var_pdibl1_a, locals.var_pdibl1_a_dn0, locals.var_pdibl1_a_dn2, locals.var_pdibl1_a_dn3, locals.var_pdibl1_a_dn4, locals.var_pdibl1_a_dn5, locals.var_pdibl1_a_dn6, locals.var_pdibl1_a_dn7, locals.var_pdibl1_a_dn8, locals.var_pdibl1_a_dn9, locals.var_pdibl1_a_dn10, locals.var_pdibl1_a_dn11, locals.var_pdibl1_a_dn13, locals.var_pdibl1_a_dn14,)
    }
};
        locals.var_pdibl1_a = assign19830_e37718;
        locals.var_pdibl1_a_dn0 = assign19830_e37718_d_n0;
        locals.var_pdibl1_a_dn2 = assign19830_e37718_d_n2;
        locals.var_pdibl1_a_dn3 = assign19830_e37718_d_n3;
        locals.var_pdibl1_a_dn4 = assign19830_e37718_d_n4;
        locals.var_pdibl1_a_dn5 = assign19830_e37718_d_n5;
        locals.var_pdibl1_a_dn6 = assign19830_e37718_d_n6;
        locals.var_pdibl1_a_dn7 = assign19830_e37718_d_n7;
        locals.var_pdibl1_a_dn8 = assign19830_e37718_d_n8;
        locals.var_pdibl1_a_dn9 = assign19830_e37718_d_n9;
        locals.var_pdibl1_a_dn10 = assign19830_e37718_d_n10;
        locals.var_pdibl1_a_dn11 = assign19830_e37718_d_n11;
        locals.var_pdibl1_a_dn13 = assign19830_e37718_d_n13;
        locals.var_pdibl1_a_dn14 = assign19830_e37718_d_n14;

        let (assign19840_e37728, assign19840_e37728_d_n0, assign19840_e37728_d_n2, assign19840_e37728_d_n3, assign19840_e37728_d_n4, assign19840_e37728_d_n5, assign19840_e37728_d_n6, assign19840_e37728_d_n7, assign19840_e37728_d_n8, assign19840_e37728_d_n9, assign19840_e37728_d_n10, assign19840_e37728_d_n11, assign19840_e37728_d_n13, assign19840_e37728_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19840_e37722: f64 = (locals.var_pdibl2r_i * locals.var_wr_v);
        let assign19840_e37725: f64 = (locals.var_pdibl2_i * locals.var_wf);
        let assign19840_e37726: f64 = (assign19840_e37722 + assign19840_e37725);
        (assign19840_e37726, ((locals.var_pdibl2r_i * locals.var_wr_v_dn0) + (locals.var_pdibl2_i * locals.var_wf_dn0)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn2) + (locals.var_pdibl2_i * locals.var_wf_dn2)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn3) + (locals.var_pdibl2_i * locals.var_wf_dn3)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn4) + (locals.var_pdibl2_i * locals.var_wf_dn4)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn5) + (locals.var_pdibl2_i * locals.var_wf_dn5)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn6) + (locals.var_pdibl2_i * locals.var_wf_dn6)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn7) + (locals.var_pdibl2_i * locals.var_wf_dn7)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn8) + (locals.var_pdibl2_i * locals.var_wf_dn8)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn9) + (locals.var_pdibl2_i * locals.var_wf_dn9)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn10) + (locals.var_pdibl2_i * locals.var_wf_dn10)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn11) + (locals.var_pdibl2_i * locals.var_wf_dn11)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn13) + (locals.var_pdibl2_i * locals.var_wf_dn13)), ((locals.var_pdibl2r_i * locals.var_wr_v_dn14) + (locals.var_pdibl2_i * locals.var_wf_dn14)),)
    } else {
        (locals.var_pdibl2_a, locals.var_pdibl2_a_dn0, locals.var_pdibl2_a_dn2, locals.var_pdibl2_a_dn3, locals.var_pdibl2_a_dn4, locals.var_pdibl2_a_dn5, locals.var_pdibl2_a_dn6, locals.var_pdibl2_a_dn7, locals.var_pdibl2_a_dn8, locals.var_pdibl2_a_dn9, locals.var_pdibl2_a_dn10, locals.var_pdibl2_a_dn11, locals.var_pdibl2_a_dn13, locals.var_pdibl2_a_dn14,)
    }
};
        locals.var_pdibl2_a = assign19840_e37728;
        locals.var_pdibl2_a_dn0 = assign19840_e37728_d_n0;
        locals.var_pdibl2_a_dn2 = assign19840_e37728_d_n2;
        locals.var_pdibl2_a_dn3 = assign19840_e37728_d_n3;
        locals.var_pdibl2_a_dn4 = assign19840_e37728_d_n4;
        locals.var_pdibl2_a_dn5 = assign19840_e37728_d_n5;
        locals.var_pdibl2_a_dn6 = assign19840_e37728_d_n6;
        locals.var_pdibl2_a_dn7 = assign19840_e37728_d_n7;
        locals.var_pdibl2_a_dn8 = assign19840_e37728_d_n8;
        locals.var_pdibl2_a_dn9 = assign19840_e37728_d_n9;
        locals.var_pdibl2_a_dn10 = assign19840_e37728_d_n10;
        locals.var_pdibl2_a_dn11 = assign19840_e37728_d_n11;
        locals.var_pdibl2_a_dn13 = assign19840_e37728_d_n13;
        locals.var_pdibl2_a_dn14 = assign19840_e37728_d_n14;

        let (assign19850_e37738, assign19850_e37738_d_n0, assign19850_e37738_d_n2, assign19850_e37738_d_n3, assign19850_e37738_d_n4, assign19850_e37738_d_n5, assign19850_e37738_d_n6, assign19850_e37738_d_n7, assign19850_e37738_d_n8, assign19850_e37738_d_n9, assign19850_e37738_d_n10, assign19850_e37738_d_n11, assign19850_e37738_d_n13, assign19850_e37738_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19850_e37732: f64 = (locals.var_mexpr_t * locals.var_wr_v);
        let assign19850_e37735: f64 = (locals.var_mexp_t * locals.var_wf);
        let assign19850_e37736: f64 = (assign19850_e37732 + assign19850_e37735);
        (assign19850_e37736, (((locals.var_mexpr_t_dn0 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn0)) + ((locals.var_mexp_t_dn0 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn0))), (((locals.var_mexpr_t_dn2 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn2)) + ((locals.var_mexp_t_dn2 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn2))), (((locals.var_mexpr_t_dn3 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn3)) + ((locals.var_mexp_t_dn3 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn3))), (((locals.var_mexpr_t_dn4 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn4)) + ((locals.var_mexp_t_dn4 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn4))), (((locals.var_mexpr_t_dn5 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn5)) + ((locals.var_mexp_t_dn5 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn5))), (((locals.var_mexpr_t_dn6 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn6)) + ((locals.var_mexp_t_dn6 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn6))), (((locals.var_mexpr_t_dn7 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn7)) + ((locals.var_mexp_t_dn7 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn7))), (((locals.var_mexpr_t_dn8 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn8)) + ((locals.var_mexp_t_dn8 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn8))), (((locals.var_mexpr_t_dn9 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn9)) + ((locals.var_mexp_t_dn9 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn9))), (((locals.var_mexpr_t_dn10 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn10)) + ((locals.var_mexp_t_dn10 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn10))), (((locals.var_mexpr_t_dn11 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn11)) + ((locals.var_mexp_t_dn11 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn11))), (((locals.var_mexpr_t_dn13 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn13)) + ((locals.var_mexp_t_dn13 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn13))), (((locals.var_mexpr_t_dn14 * locals.var_wr_v) + (locals.var_mexpr_t * locals.var_wr_v_dn14)) + ((locals.var_mexp_t_dn14 * locals.var_wf) + (locals.var_mexp_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_mexp_a, locals.var_mexp_a_dn0, locals.var_mexp_a_dn2, locals.var_mexp_a_dn3, locals.var_mexp_a_dn4, locals.var_mexp_a_dn5, locals.var_mexp_a_dn6, locals.var_mexp_a_dn7, locals.var_mexp_a_dn8, locals.var_mexp_a_dn9, locals.var_mexp_a_dn10, locals.var_mexp_a_dn11, locals.var_mexp_a_dn13, locals.var_mexp_a_dn14,)
    }
};
        locals.var_mexp_a = assign19850_e37738;
        locals.var_mexp_a_dn0 = assign19850_e37738_d_n0;
        locals.var_mexp_a_dn2 = assign19850_e37738_d_n2;
        locals.var_mexp_a_dn3 = assign19850_e37738_d_n3;
        locals.var_mexp_a_dn4 = assign19850_e37738_d_n4;
        locals.var_mexp_a_dn5 = assign19850_e37738_d_n5;
        locals.var_mexp_a_dn6 = assign19850_e37738_d_n6;
        locals.var_mexp_a_dn7 = assign19850_e37738_d_n7;
        locals.var_mexp_a_dn8 = assign19850_e37738_d_n8;
        locals.var_mexp_a_dn9 = assign19850_e37738_d_n9;
        locals.var_mexp_a_dn10 = assign19850_e37738_d_n10;
        locals.var_mexp_a_dn11 = assign19850_e37738_d_n11;
        locals.var_mexp_a_dn13 = assign19850_e37738_d_n13;
        locals.var_mexp_a_dn14 = assign19850_e37738_d_n14;

        let (assign19860_e37748, assign19860_e37748_d_n0, assign19860_e37748_d_n2, assign19860_e37748_d_n3, assign19860_e37748_d_n4, assign19860_e37748_d_n5, assign19860_e37748_d_n6, assign19860_e37748_d_n7, assign19860_e37748_d_n8, assign19860_e37748_d_n9, assign19860_e37748_d_n10, assign19860_e37748_d_n11, assign19860_e37748_d_n13, assign19860_e37748_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19860_e37742: f64 = (locals.var_ptwgr_t * locals.var_wr_v);
        let assign19860_e37745: f64 = (locals.var_ptwg_t * locals.var_wf);
        let assign19860_e37746: f64 = (assign19860_e37742 + assign19860_e37745);
        (assign19860_e37746, (((locals.var_ptwgr_t_dn0 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn0)) + ((locals.var_ptwg_t_dn0 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn0))), (((locals.var_ptwgr_t_dn2 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn2)) + ((locals.var_ptwg_t_dn2 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn2))), (((locals.var_ptwgr_t_dn3 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn3)) + ((locals.var_ptwg_t_dn3 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn3))), (((locals.var_ptwgr_t_dn4 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn4)) + ((locals.var_ptwg_t_dn4 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn4))), (((locals.var_ptwgr_t_dn5 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn5)) + ((locals.var_ptwg_t_dn5 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn5))), (((locals.var_ptwgr_t_dn6 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn6)) + ((locals.var_ptwg_t_dn6 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn6))), (((locals.var_ptwgr_t_dn7 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn7)) + ((locals.var_ptwg_t_dn7 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn7))), (((locals.var_ptwgr_t_dn8 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn8)) + ((locals.var_ptwg_t_dn8 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn8))), (((locals.var_ptwgr_t_dn9 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn9)) + ((locals.var_ptwg_t_dn9 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn9))), (((locals.var_ptwgr_t_dn10 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn10)) + ((locals.var_ptwg_t_dn10 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn10))), (((locals.var_ptwgr_t_dn11 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn11)) + ((locals.var_ptwg_t_dn11 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn11))), (((locals.var_ptwgr_t_dn13 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn13)) + ((locals.var_ptwg_t_dn13 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn13))), (((locals.var_ptwgr_t_dn14 * locals.var_wr_v) + (locals.var_ptwgr_t * locals.var_wr_v_dn14)) + ((locals.var_ptwg_t_dn14 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_ptwg_a, locals.var_ptwg_a_dn0, locals.var_ptwg_a_dn2, locals.var_ptwg_a_dn3, locals.var_ptwg_a_dn4, locals.var_ptwg_a_dn5, locals.var_ptwg_a_dn6, locals.var_ptwg_a_dn7, locals.var_ptwg_a_dn8, locals.var_ptwg_a_dn9, locals.var_ptwg_a_dn10, locals.var_ptwg_a_dn11, locals.var_ptwg_a_dn13, locals.var_ptwg_a_dn14,)
    }
};
        locals.var_ptwg_a = assign19860_e37748;
        locals.var_ptwg_a_dn0 = assign19860_e37748_d_n0;
        locals.var_ptwg_a_dn2 = assign19860_e37748_d_n2;
        locals.var_ptwg_a_dn3 = assign19860_e37748_d_n3;
        locals.var_ptwg_a_dn4 = assign19860_e37748_d_n4;
        locals.var_ptwg_a_dn5 = assign19860_e37748_d_n5;
        locals.var_ptwg_a_dn6 = assign19860_e37748_d_n6;
        locals.var_ptwg_a_dn7 = assign19860_e37748_d_n7;
        locals.var_ptwg_a_dn8 = assign19860_e37748_d_n8;
        locals.var_ptwg_a_dn9 = assign19860_e37748_d_n9;
        locals.var_ptwg_a_dn10 = assign19860_e37748_d_n10;
        locals.var_ptwg_a_dn11 = assign19860_e37748_d_n11;
        locals.var_ptwg_a_dn13 = assign19860_e37748_d_n13;
        locals.var_ptwg_a_dn14 = assign19860_e37748_d_n14;

        let (assign19870_e37758, assign19870_e37758_d_n0, assign19870_e37758_d_n2, assign19870_e37758_d_n3, assign19870_e37758_d_n4, assign19870_e37758_d_n5, assign19870_e37758_d_n6, assign19870_e37758_d_n7, assign19870_e37758_d_n8, assign19870_e37758_d_n9, assign19870_e37758_d_n10, assign19870_e37758_d_n11, assign19870_e37758_d_n13, assign19870_e37758_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19870_e37752: f64 = (locals.var_vsat1r_t * locals.var_wr_v);
        let assign19870_e37755: f64 = (locals.var_vsat1_t * locals.var_wf);
        let assign19870_e37756: f64 = (assign19870_e37752 + assign19870_e37755);
        (assign19870_e37756, (((locals.var_vsat1r_t_dn0 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn0)) + ((locals.var_vsat1_t_dn0 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn0))), (((locals.var_vsat1r_t_dn2 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn2)) + ((locals.var_vsat1_t_dn2 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn2))), (((locals.var_vsat1r_t_dn3 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn3)) + ((locals.var_vsat1_t_dn3 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn3))), (((locals.var_vsat1r_t_dn4 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn4)) + ((locals.var_vsat1_t_dn4 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn4))), (((locals.var_vsat1r_t_dn5 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn5)) + ((locals.var_vsat1_t_dn5 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn5))), (((locals.var_vsat1r_t_dn6 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn6)) + ((locals.var_vsat1_t_dn6 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn6))), (((locals.var_vsat1r_t_dn7 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn7)) + ((locals.var_vsat1_t_dn7 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn7))), (((locals.var_vsat1r_t_dn8 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn8)) + ((locals.var_vsat1_t_dn8 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn8))), (((locals.var_vsat1r_t_dn9 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn9)) + ((locals.var_vsat1_t_dn9 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn9))), (((locals.var_vsat1r_t_dn10 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn10)) + ((locals.var_vsat1_t_dn10 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn10))), (((locals.var_vsat1r_t_dn11 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn11)) + ((locals.var_vsat1_t_dn11 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn11))), (((locals.var_vsat1r_t_dn13 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn13)) + ((locals.var_vsat1_t_dn13 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn13))), (((locals.var_vsat1r_t_dn14 * locals.var_wr_v) + (locals.var_vsat1r_t * locals.var_wr_v_dn14)) + ((locals.var_vsat1_t_dn14 * locals.var_wf) + (locals.var_vsat1_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_vsat1_a, locals.var_vsat1_a_dn0, locals.var_vsat1_a_dn2, locals.var_vsat1_a_dn3, locals.var_vsat1_a_dn4, locals.var_vsat1_a_dn5, locals.var_vsat1_a_dn6, locals.var_vsat1_a_dn7, locals.var_vsat1_a_dn8, locals.var_vsat1_a_dn9, locals.var_vsat1_a_dn10, locals.var_vsat1_a_dn11, locals.var_vsat1_a_dn13, locals.var_vsat1_a_dn14,)
    }
};
        locals.var_vsat1_a = assign19870_e37758;
        locals.var_vsat1_a_dn0 = assign19870_e37758_d_n0;
        locals.var_vsat1_a_dn2 = assign19870_e37758_d_n2;
        locals.var_vsat1_a_dn3 = assign19870_e37758_d_n3;
        locals.var_vsat1_a_dn4 = assign19870_e37758_d_n4;
        locals.var_vsat1_a_dn5 = assign19870_e37758_d_n5;
        locals.var_vsat1_a_dn6 = assign19870_e37758_d_n6;
        locals.var_vsat1_a_dn7 = assign19870_e37758_d_n7;
        locals.var_vsat1_a_dn8 = assign19870_e37758_d_n8;
        locals.var_vsat1_a_dn9 = assign19870_e37758_d_n9;
        locals.var_vsat1_a_dn10 = assign19870_e37758_d_n10;
        locals.var_vsat1_a_dn11 = assign19870_e37758_d_n11;
        locals.var_vsat1_a_dn13 = assign19870_e37758_d_n13;
        locals.var_vsat1_a_dn14 = assign19870_e37758_d_n14;

        let (assign19880_e37768, assign19880_e37768_d_n0, assign19880_e37768_d_n2, assign19880_e37768_d_n3, assign19880_e37768_d_n4, assign19880_e37768_d_n5, assign19880_e37768_d_n6, assign19880_e37768_d_n7, assign19880_e37768_d_n8, assign19880_e37768_d_n9, assign19880_e37768_d_n10, assign19880_e37768_d_n11, assign19880_e37768_d_n13, assign19880_e37768_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19880_e37762: f64 = (locals.var_rsdrr_t * locals.var_wr_v);
        let assign19880_e37765: f64 = (locals.var_rsdr_t * locals.var_wf);
        let assign19880_e37766: f64 = (assign19880_e37762 + assign19880_e37765);
        (assign19880_e37766, ((locals.var_rsdrr_t * locals.var_wr_v_dn0) + (locals.var_rsdr_t * locals.var_wf_dn0)), ((locals.var_rsdrr_t * locals.var_wr_v_dn2) + (locals.var_rsdr_t * locals.var_wf_dn2)), ((locals.var_rsdrr_t * locals.var_wr_v_dn3) + (locals.var_rsdr_t * locals.var_wf_dn3)), (((locals.var_rsdrr_t_dn4 * locals.var_wr_v) + (locals.var_rsdrr_t * locals.var_wr_v_dn4)) + ((locals.var_rsdr_t_dn4 * locals.var_wf) + (locals.var_rsdr_t * locals.var_wf_dn4))), ((locals.var_rsdrr_t * locals.var_wr_v_dn5) + (locals.var_rsdr_t * locals.var_wf_dn5)), ((locals.var_rsdrr_t * locals.var_wr_v_dn6) + (locals.var_rsdr_t * locals.var_wf_dn6)), ((locals.var_rsdrr_t * locals.var_wr_v_dn7) + (locals.var_rsdr_t * locals.var_wf_dn7)), ((locals.var_rsdrr_t * locals.var_wr_v_dn8) + (locals.var_rsdr_t * locals.var_wf_dn8)), ((locals.var_rsdrr_t * locals.var_wr_v_dn9) + (locals.var_rsdr_t * locals.var_wf_dn9)), ((locals.var_rsdrr_t * locals.var_wr_v_dn10) + (locals.var_rsdr_t * locals.var_wf_dn10)), ((locals.var_rsdrr_t * locals.var_wr_v_dn11) + (locals.var_rsdr_t * locals.var_wf_dn11)), ((locals.var_rsdrr_t * locals.var_wr_v_dn13) + (locals.var_rsdr_t * locals.var_wf_dn13)), ((locals.var_rsdrr_t * locals.var_wr_v_dn14) + (locals.var_rsdr_t * locals.var_wf_dn14)),)
    } else {
        (locals.var_rsdr_a, locals.var_rsdr_a_dn0, locals.var_rsdr_a_dn2, locals.var_rsdr_a_dn3, locals.var_rsdr_a_dn4, locals.var_rsdr_a_dn5, locals.var_rsdr_a_dn6, locals.var_rsdr_a_dn7, locals.var_rsdr_a_dn8, locals.var_rsdr_a_dn9, locals.var_rsdr_a_dn10, locals.var_rsdr_a_dn11, locals.var_rsdr_a_dn13, locals.var_rsdr_a_dn14,)
    }
};
        locals.var_rsdr_a = assign19880_e37768;
        locals.var_rsdr_a_dn0 = assign19880_e37768_d_n0;
        locals.var_rsdr_a_dn2 = assign19880_e37768_d_n2;
        locals.var_rsdr_a_dn3 = assign19880_e37768_d_n3;
        locals.var_rsdr_a_dn4 = assign19880_e37768_d_n4;
        locals.var_rsdr_a_dn5 = assign19880_e37768_d_n5;
        locals.var_rsdr_a_dn6 = assign19880_e37768_d_n6;
        locals.var_rsdr_a_dn7 = assign19880_e37768_d_n7;
        locals.var_rsdr_a_dn8 = assign19880_e37768_d_n8;
        locals.var_rsdr_a_dn9 = assign19880_e37768_d_n9;
        locals.var_rsdr_a_dn10 = assign19880_e37768_d_n10;
        locals.var_rsdr_a_dn11 = assign19880_e37768_d_n11;
        locals.var_rsdr_a_dn13 = assign19880_e37768_d_n13;
        locals.var_rsdr_a_dn14 = assign19880_e37768_d_n14;

        let (assign19890_e37778, assign19890_e37778_d_n0, assign19890_e37778_d_n2, assign19890_e37778_d_n3, assign19890_e37778_d_n4, assign19890_e37778_d_n5, assign19890_e37778_d_n6, assign19890_e37778_d_n7, assign19890_e37778_d_n8, assign19890_e37778_d_n9, assign19890_e37778_d_n10, assign19890_e37778_d_n11, assign19890_e37778_d_n13, assign19890_e37778_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19890_e37772: f64 = (locals.var_rddrr_t * locals.var_wr_v);
        let assign19890_e37775: f64 = (locals.var_rddr_t * locals.var_wf);
        let assign19890_e37776: f64 = (assign19890_e37772 + assign19890_e37775);
        (assign19890_e37776, ((locals.var_rddrr_t * locals.var_wr_v_dn0) + (locals.var_rddr_t * locals.var_wf_dn0)), ((locals.var_rddrr_t * locals.var_wr_v_dn2) + (locals.var_rddr_t * locals.var_wf_dn2)), ((locals.var_rddrr_t * locals.var_wr_v_dn3) + (locals.var_rddr_t * locals.var_wf_dn3)), (((locals.var_rddrr_t_dn4 * locals.var_wr_v) + (locals.var_rddrr_t * locals.var_wr_v_dn4)) + ((locals.var_rddr_t_dn4 * locals.var_wf) + (locals.var_rddr_t * locals.var_wf_dn4))), ((locals.var_rddrr_t * locals.var_wr_v_dn5) + (locals.var_rddr_t * locals.var_wf_dn5)), ((locals.var_rddrr_t * locals.var_wr_v_dn6) + (locals.var_rddr_t * locals.var_wf_dn6)), ((locals.var_rddrr_t * locals.var_wr_v_dn7) + (locals.var_rddr_t * locals.var_wf_dn7)), ((locals.var_rddrr_t * locals.var_wr_v_dn8) + (locals.var_rddr_t * locals.var_wf_dn8)), ((locals.var_rddrr_t * locals.var_wr_v_dn9) + (locals.var_rddr_t * locals.var_wf_dn9)), ((locals.var_rddrr_t * locals.var_wr_v_dn10) + (locals.var_rddr_t * locals.var_wf_dn10)), ((locals.var_rddrr_t * locals.var_wr_v_dn11) + (locals.var_rddr_t * locals.var_wf_dn11)), ((locals.var_rddrr_t * locals.var_wr_v_dn13) + (locals.var_rddr_t * locals.var_wf_dn13)), ((locals.var_rddrr_t * locals.var_wr_v_dn14) + (locals.var_rddr_t * locals.var_wf_dn14)),)
    } else {
        (locals.var_rddr_a, locals.var_rddr_a_dn0, locals.var_rddr_a_dn2, locals.var_rddr_a_dn3, locals.var_rddr_a_dn4, locals.var_rddr_a_dn5, locals.var_rddr_a_dn6, locals.var_rddr_a_dn7, locals.var_rddr_a_dn8, locals.var_rddr_a_dn9, locals.var_rddr_a_dn10, locals.var_rddr_a_dn11, locals.var_rddr_a_dn13, locals.var_rddr_a_dn14,)
    }
};
        locals.var_rddr_a = assign19890_e37778;
        locals.var_rddr_a_dn0 = assign19890_e37778_d_n0;
        locals.var_rddr_a_dn2 = assign19890_e37778_d_n2;
        locals.var_rddr_a_dn3 = assign19890_e37778_d_n3;
        locals.var_rddr_a_dn4 = assign19890_e37778_d_n4;
        locals.var_rddr_a_dn5 = assign19890_e37778_d_n5;
        locals.var_rddr_a_dn6 = assign19890_e37778_d_n6;
        locals.var_rddr_a_dn7 = assign19890_e37778_d_n7;
        locals.var_rddr_a_dn8 = assign19890_e37778_d_n8;
        locals.var_rddr_a_dn9 = assign19890_e37778_d_n9;
        locals.var_rddr_a_dn10 = assign19890_e37778_d_n10;
        locals.var_rddr_a_dn11 = assign19890_e37778_d_n11;
        locals.var_rddr_a_dn13 = assign19890_e37778_d_n13;
        locals.var_rddr_a_dn14 = assign19890_e37778_d_n14;

        let (assign19900_e37788, assign19900_e37788_d_n0, assign19900_e37788_d_n2, assign19900_e37788_d_n3, assign19900_e37788_d_n4, assign19900_e37788_d_n5, assign19900_e37788_d_n6, assign19900_e37788_d_n7, assign19900_e37788_d_n8, assign19900_e37788_d_n9, assign19900_e37788_d_n10, assign19900_e37788_d_n11, assign19900_e37788_d_n13, assign19900_e37788_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19900_e37782: f64 = (locals.var_pclmr_i * locals.var_wr_v);
        let assign19900_e37785: f64 = (locals.var_pclm_t * locals.var_wf);
        let assign19900_e37786: f64 = (assign19900_e37782 + assign19900_e37785);
        (assign19900_e37786, (((locals.var_pclmr_i_dn0 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn0)) + ((locals.var_pclm_t_dn0 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn0))), (((locals.var_pclmr_i_dn2 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn2)) + ((locals.var_pclm_t_dn2 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn2))), (((locals.var_pclmr_i_dn3 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn3)) + ((locals.var_pclm_t_dn3 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn3))), (((locals.var_pclmr_i_dn4 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn4)) + ((locals.var_pclm_t_dn4 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn4))), (((locals.var_pclmr_i_dn5 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn5)) + ((locals.var_pclm_t_dn5 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn5))), (((locals.var_pclmr_i_dn6 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn6)) + ((locals.var_pclm_t_dn6 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn6))), (((locals.var_pclmr_i_dn7 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn7)) + ((locals.var_pclm_t_dn7 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn7))), (((locals.var_pclmr_i_dn8 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn8)) + ((locals.var_pclm_t_dn8 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn8))), (((locals.var_pclmr_i_dn9 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn9)) + ((locals.var_pclm_t_dn9 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn9))), (((locals.var_pclmr_i_dn10 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn10)) + ((locals.var_pclm_t_dn10 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn10))), (((locals.var_pclmr_i_dn11 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn11)) + ((locals.var_pclm_t_dn11 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn11))), (((locals.var_pclmr_i_dn13 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn13)) + ((locals.var_pclm_t_dn13 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn13))), (((locals.var_pclmr_i_dn14 * locals.var_wr_v) + (locals.var_pclmr_i * locals.var_wr_v_dn14)) + ((locals.var_pclm_t_dn14 * locals.var_wf) + (locals.var_pclm_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_pclm_a, locals.var_pclm_a_dn0, locals.var_pclm_a_dn2, locals.var_pclm_a_dn3, locals.var_pclm_a_dn4, locals.var_pclm_a_dn5, locals.var_pclm_a_dn6, locals.var_pclm_a_dn7, locals.var_pclm_a_dn8, locals.var_pclm_a_dn9, locals.var_pclm_a_dn10, locals.var_pclm_a_dn11, locals.var_pclm_a_dn13, locals.var_pclm_a_dn14,)
    }
};
        locals.var_pclm_a = assign19900_e37788;
        locals.var_pclm_a_dn0 = assign19900_e37788_d_n0;
        locals.var_pclm_a_dn2 = assign19900_e37788_d_n2;
        locals.var_pclm_a_dn3 = assign19900_e37788_d_n3;
        locals.var_pclm_a_dn4 = assign19900_e37788_d_n4;
        locals.var_pclm_a_dn5 = assign19900_e37788_d_n5;
        locals.var_pclm_a_dn6 = assign19900_e37788_d_n6;
        locals.var_pclm_a_dn7 = assign19900_e37788_d_n7;
        locals.var_pclm_a_dn8 = assign19900_e37788_d_n8;
        locals.var_pclm_a_dn9 = assign19900_e37788_d_n9;
        locals.var_pclm_a_dn10 = assign19900_e37788_d_n10;
        locals.var_pclm_a_dn11 = assign19900_e37788_d_n11;
        locals.var_pclm_a_dn13 = assign19900_e37788_d_n13;
        locals.var_pclm_a_dn14 = assign19900_e37788_d_n14;

    }

    pub(super) fn stamp_transient_block_72(
        locals: &mut StampLocals,
    ) {
        let (assign19910_e37798, assign19910_e37798_d_n0, assign19910_e37798_d_n2, assign19910_e37798_d_n3, assign19910_e37798_d_n4, assign19910_e37798_d_n5, assign19910_e37798_d_n6, assign19910_e37798_d_n7, assign19910_e37798_d_n8, assign19910_e37798_d_n9, assign19910_e37798_d_n10, assign19910_e37798_d_n11, assign19910_e37798_d_n13, assign19910_e37798_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19910_e37792: f64 = (locals.var_vsatr_t * locals.var_wr_v);
        let assign19910_e37795: f64 = (locals.var_vsat_t * locals.var_wf);
        let assign19910_e37796: f64 = (assign19910_e37792 + assign19910_e37795);
        (assign19910_e37796, ((locals.var_vsatr_t * locals.var_wr_v_dn0) + ((locals.var_vsat_t_dn0 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn0))), ((locals.var_vsatr_t * locals.var_wr_v_dn2) + ((locals.var_vsat_t_dn2 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn2))), ((locals.var_vsatr_t * locals.var_wr_v_dn3) + ((locals.var_vsat_t_dn3 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn3))), (((locals.var_vsatr_t_dn4 * locals.var_wr_v) + (locals.var_vsatr_t * locals.var_wr_v_dn4)) + ((locals.var_vsat_t_dn4 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn4))), ((locals.var_vsatr_t * locals.var_wr_v_dn5) + ((locals.var_vsat_t_dn5 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn5))), ((locals.var_vsatr_t * locals.var_wr_v_dn6) + ((locals.var_vsat_t_dn6 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn6))), ((locals.var_vsatr_t * locals.var_wr_v_dn7) + ((locals.var_vsat_t_dn7 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn7))), ((locals.var_vsatr_t * locals.var_wr_v_dn8) + ((locals.var_vsat_t_dn8 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn8))), ((locals.var_vsatr_t * locals.var_wr_v_dn9) + ((locals.var_vsat_t_dn9 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn9))), ((locals.var_vsatr_t * locals.var_wr_v_dn10) + ((locals.var_vsat_t_dn10 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn10))), ((locals.var_vsatr_t * locals.var_wr_v_dn11) + ((locals.var_vsat_t_dn11 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn11))), ((locals.var_vsatr_t * locals.var_wr_v_dn13) + ((locals.var_vsat_t_dn13 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn13))), ((locals.var_vsatr_t * locals.var_wr_v_dn14) + ((locals.var_vsat_t_dn14 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_vsat_a, locals.var_vsat_a_dn0, locals.var_vsat_a_dn2, locals.var_vsat_a_dn3, locals.var_vsat_a_dn4, locals.var_vsat_a_dn5, locals.var_vsat_a_dn6, locals.var_vsat_a_dn7, locals.var_vsat_a_dn8, locals.var_vsat_a_dn9, locals.var_vsat_a_dn10, locals.var_vsat_a_dn11, locals.var_vsat_a_dn13, locals.var_vsat_a_dn14,)
    }
};
        locals.var_vsat_a = assign19910_e37798;
        locals.var_vsat_a_dn0 = assign19910_e37798_d_n0;
        locals.var_vsat_a_dn2 = assign19910_e37798_d_n2;
        locals.var_vsat_a_dn3 = assign19910_e37798_d_n3;
        locals.var_vsat_a_dn4 = assign19910_e37798_d_n4;
        locals.var_vsat_a_dn5 = assign19910_e37798_d_n5;
        locals.var_vsat_a_dn6 = assign19910_e37798_d_n6;
        locals.var_vsat_a_dn7 = assign19910_e37798_d_n7;
        locals.var_vsat_a_dn8 = assign19910_e37798_d_n8;
        locals.var_vsat_a_dn9 = assign19910_e37798_d_n9;
        locals.var_vsat_a_dn10 = assign19910_e37798_d_n10;
        locals.var_vsat_a_dn11 = assign19910_e37798_d_n11;
        locals.var_vsat_a_dn13 = assign19910_e37798_d_n13;
        locals.var_vsat_a_dn14 = assign19910_e37798_d_n14;

        let (assign19920_e37808, assign19920_e37808_d_n0, assign19920_e37808_d_n2, assign19920_e37808_d_n3, assign19920_e37808_d_n4, assign19920_e37808_d_n5, assign19920_e37808_d_n6, assign19920_e37808_d_n7, assign19920_e37808_d_n8, assign19920_e37808_d_n9, assign19920_e37808_d_n10, assign19920_e37808_d_n11, assign19920_e37808_d_n13, assign19920_e37808_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19920_e37802: f64 = (locals.var_ksativr_i * locals.var_wr_v);
        let assign19920_e37805: f64 = (locals.var_ksativ_t * locals.var_wf);
        let assign19920_e37806: f64 = (assign19920_e37802 + assign19920_e37805);
        (assign19920_e37806, ((locals.var_ksativr_i * locals.var_wr_v_dn0) + (locals.var_ksativ_t * locals.var_wf_dn0)), ((locals.var_ksativr_i * locals.var_wr_v_dn2) + (locals.var_ksativ_t * locals.var_wf_dn2)), ((locals.var_ksativr_i * locals.var_wr_v_dn3) + (locals.var_ksativ_t * locals.var_wf_dn3)), ((locals.var_ksativr_i * locals.var_wr_v_dn4) + ((locals.var_ksativ_t_dn4 * locals.var_wf) + (locals.var_ksativ_t * locals.var_wf_dn4))), ((locals.var_ksativr_i * locals.var_wr_v_dn5) + (locals.var_ksativ_t * locals.var_wf_dn5)), ((locals.var_ksativr_i * locals.var_wr_v_dn6) + (locals.var_ksativ_t * locals.var_wf_dn6)), ((locals.var_ksativr_i * locals.var_wr_v_dn7) + (locals.var_ksativ_t * locals.var_wf_dn7)), ((locals.var_ksativr_i * locals.var_wr_v_dn8) + (locals.var_ksativ_t * locals.var_wf_dn8)), ((locals.var_ksativr_i * locals.var_wr_v_dn9) + (locals.var_ksativ_t * locals.var_wf_dn9)), ((locals.var_ksativr_i * locals.var_wr_v_dn10) + (locals.var_ksativ_t * locals.var_wf_dn10)), ((locals.var_ksativr_i * locals.var_wr_v_dn11) + (locals.var_ksativ_t * locals.var_wf_dn11)), ((locals.var_ksativr_i * locals.var_wr_v_dn13) + (locals.var_ksativ_t * locals.var_wf_dn13)), ((locals.var_ksativr_i * locals.var_wr_v_dn14) + (locals.var_ksativ_t * locals.var_wf_dn14)),)
    } else {
        (locals.var_ksativ_a, locals.var_ksativ_a_dn0, locals.var_ksativ_a_dn2, locals.var_ksativ_a_dn3, locals.var_ksativ_a_dn4, locals.var_ksativ_a_dn5, locals.var_ksativ_a_dn6, locals.var_ksativ_a_dn7, locals.var_ksativ_a_dn8, locals.var_ksativ_a_dn9, locals.var_ksativ_a_dn10, locals.var_ksativ_a_dn11, locals.var_ksativ_a_dn13, locals.var_ksativ_a_dn14,)
    }
};
        locals.var_ksativ_a = assign19920_e37808;
        locals.var_ksativ_a_dn0 = assign19920_e37808_d_n0;
        locals.var_ksativ_a_dn2 = assign19920_e37808_d_n2;
        locals.var_ksativ_a_dn3 = assign19920_e37808_d_n3;
        locals.var_ksativ_a_dn4 = assign19920_e37808_d_n4;
        locals.var_ksativ_a_dn5 = assign19920_e37808_d_n5;
        locals.var_ksativ_a_dn6 = assign19920_e37808_d_n6;
        locals.var_ksativ_a_dn7 = assign19920_e37808_d_n7;
        locals.var_ksativ_a_dn8 = assign19920_e37808_d_n8;
        locals.var_ksativ_a_dn9 = assign19920_e37808_d_n9;
        locals.var_ksativ_a_dn10 = assign19920_e37808_d_n10;
        locals.var_ksativ_a_dn11 = assign19920_e37808_d_n11;
        locals.var_ksativ_a_dn13 = assign19920_e37808_d_n13;
        locals.var_ksativ_a_dn14 = assign19920_e37808_d_n14;

        let (assign19930_e37818, assign19930_e37818_d_n0, assign19930_e37818_d_n2, assign19930_e37818_d_n3, assign19930_e37818_d_n4, assign19930_e37818_d_n5, assign19930_e37818_d_n6, assign19930_e37818_d_n7, assign19930_e37818_d_n8, assign19930_e37818_d_n9, assign19930_e37818_d_n10, assign19930_e37818_d_n11, assign19930_e37818_d_n13, assign19930_e37818_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19930_e37812: f64 = (locals.var_dvtshiftr_i * locals.var_wr_v);
        let assign19930_e37815: f64 = (locals.var_dvtshift_i * locals.var_wf);
        let assign19930_e37816: f64 = (assign19930_e37812 + assign19930_e37815);
        (assign19930_e37816, ((locals.var_dvtshiftr_i * locals.var_wr_v_dn0) + (locals.var_dvtshift_i * locals.var_wf_dn0)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn2) + (locals.var_dvtshift_i * locals.var_wf_dn2)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn3) + (locals.var_dvtshift_i * locals.var_wf_dn3)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn4) + (locals.var_dvtshift_i * locals.var_wf_dn4)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn5) + (locals.var_dvtshift_i * locals.var_wf_dn5)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn6) + (locals.var_dvtshift_i * locals.var_wf_dn6)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn7) + (locals.var_dvtshift_i * locals.var_wf_dn7)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn8) + (locals.var_dvtshift_i * locals.var_wf_dn8)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn9) + (locals.var_dvtshift_i * locals.var_wf_dn9)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn10) + (locals.var_dvtshift_i * locals.var_wf_dn10)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn11) + (locals.var_dvtshift_i * locals.var_wf_dn11)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn13) + (locals.var_dvtshift_i * locals.var_wf_dn13)), ((locals.var_dvtshiftr_i * locals.var_wr_v_dn14) + (locals.var_dvtshift_i * locals.var_wf_dn14)),)
    } else {
        (locals.var_dvtshift_a, locals.var_dvtshift_a_dn0, locals.var_dvtshift_a_dn2, locals.var_dvtshift_a_dn3, locals.var_dvtshift_a_dn4, locals.var_dvtshift_a_dn5, locals.var_dvtshift_a_dn6, locals.var_dvtshift_a_dn7, locals.var_dvtshift_a_dn8, locals.var_dvtshift_a_dn9, locals.var_dvtshift_a_dn10, locals.var_dvtshift_a_dn11, locals.var_dvtshift_a_dn13, locals.var_dvtshift_a_dn14,)
    }
};
        locals.var_dvtshift_a = assign19930_e37818;
        locals.var_dvtshift_a_dn0 = assign19930_e37818_d_n0;
        locals.var_dvtshift_a_dn2 = assign19930_e37818_d_n2;
        locals.var_dvtshift_a_dn3 = assign19930_e37818_d_n3;
        locals.var_dvtshift_a_dn4 = assign19930_e37818_d_n4;
        locals.var_dvtshift_a_dn5 = assign19930_e37818_d_n5;
        locals.var_dvtshift_a_dn6 = assign19930_e37818_d_n6;
        locals.var_dvtshift_a_dn7 = assign19930_e37818_d_n7;
        locals.var_dvtshift_a_dn8 = assign19930_e37818_d_n8;
        locals.var_dvtshift_a_dn9 = assign19930_e37818_d_n9;
        locals.var_dvtshift_a_dn10 = assign19930_e37818_d_n10;
        locals.var_dvtshift_a_dn11 = assign19930_e37818_d_n11;
        locals.var_dvtshift_a_dn13 = assign19930_e37818_d_n13;
        locals.var_dvtshift_a_dn14 = assign19930_e37818_d_n14;

        let (assign19940_e37828, assign19940_e37828_d_n0, assign19940_e37828_d_n2, assign19940_e37828_d_n3, assign19940_e37828_d_n4, assign19940_e37828_d_n5, assign19940_e37828_d_n6, assign19940_e37828_d_n7, assign19940_e37828_d_n8, assign19940_e37828_d_n9, assign19940_e37828_d_n10, assign19940_e37828_d_n11, assign19940_e37828_d_n13, assign19940_e37828_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19940_e37822: f64 = (locals.var_citr_i * locals.var_wr_v);
        let assign19940_e37825: f64 = (locals.var_cit_i * locals.var_wf);
        let assign19940_e37826: f64 = (assign19940_e37822 + assign19940_e37825);
        (assign19940_e37826, ((locals.var_citr_i * locals.var_wr_v_dn0) + (locals.var_cit_i * locals.var_wf_dn0)), ((locals.var_citr_i * locals.var_wr_v_dn2) + (locals.var_cit_i * locals.var_wf_dn2)), ((locals.var_citr_i * locals.var_wr_v_dn3) + (locals.var_cit_i * locals.var_wf_dn3)), ((locals.var_citr_i * locals.var_wr_v_dn4) + (locals.var_cit_i * locals.var_wf_dn4)), ((locals.var_citr_i * locals.var_wr_v_dn5) + (locals.var_cit_i * locals.var_wf_dn5)), ((locals.var_citr_i * locals.var_wr_v_dn6) + (locals.var_cit_i * locals.var_wf_dn6)), ((locals.var_citr_i * locals.var_wr_v_dn7) + (locals.var_cit_i * locals.var_wf_dn7)), ((locals.var_citr_i * locals.var_wr_v_dn8) + (locals.var_cit_i * locals.var_wf_dn8)), ((locals.var_citr_i * locals.var_wr_v_dn9) + (locals.var_cit_i * locals.var_wf_dn9)), ((locals.var_citr_i * locals.var_wr_v_dn10) + (locals.var_cit_i * locals.var_wf_dn10)), ((locals.var_citr_i * locals.var_wr_v_dn11) + (locals.var_cit_i * locals.var_wf_dn11)), ((locals.var_citr_i * locals.var_wr_v_dn13) + (locals.var_cit_i * locals.var_wf_dn13)), ((locals.var_citr_i * locals.var_wr_v_dn14) + (locals.var_cit_i * locals.var_wf_dn14)),)
    } else {
        (locals.var_cit_a, locals.var_cit_a_dn0, locals.var_cit_a_dn2, locals.var_cit_a_dn3, locals.var_cit_a_dn4, locals.var_cit_a_dn5, locals.var_cit_a_dn6, locals.var_cit_a_dn7, locals.var_cit_a_dn8, locals.var_cit_a_dn9, locals.var_cit_a_dn10, locals.var_cit_a_dn11, locals.var_cit_a_dn13, locals.var_cit_a_dn14,)
    }
};
        locals.var_cit_a = assign19940_e37828;
        locals.var_cit_a_dn0 = assign19940_e37828_d_n0;
        locals.var_cit_a_dn2 = assign19940_e37828_d_n2;
        locals.var_cit_a_dn3 = assign19940_e37828_d_n3;
        locals.var_cit_a_dn4 = assign19940_e37828_d_n4;
        locals.var_cit_a_dn5 = assign19940_e37828_d_n5;
        locals.var_cit_a_dn6 = assign19940_e37828_d_n6;
        locals.var_cit_a_dn7 = assign19940_e37828_d_n7;
        locals.var_cit_a_dn8 = assign19940_e37828_d_n8;
        locals.var_cit_a_dn9 = assign19940_e37828_d_n9;
        locals.var_cit_a_dn10 = assign19940_e37828_d_n10;
        locals.var_cit_a_dn11 = assign19940_e37828_d_n11;
        locals.var_cit_a_dn13 = assign19940_e37828_d_n13;
        locals.var_cit_a_dn14 = assign19940_e37828_d_n14;

        let (assign19950_e37838, assign19950_e37838_d_n0, assign19950_e37838_d_n2, assign19950_e37838_d_n3, assign19950_e37838_d_n4, assign19950_e37838_d_n5, assign19950_e37838_d_n6, assign19950_e37838_d_n7, assign19950_e37838_d_n8, assign19950_e37838_d_n9, assign19950_e37838_d_n10, assign19950_e37838_d_n11, assign19950_e37838_d_n13, assign19950_e37838_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19950_e37832: f64 = (locals.var_u0r_v * locals.var_wr_v);
        let assign19950_e37835: f64 = (locals.var_u0_v * locals.var_wf);
        let assign19950_e37836: f64 = (assign19950_e37832 + assign19950_e37835);
        (assign19950_e37836, (((locals.var_u0r_v_dn0 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn0)) + ((locals.var_u0_v_dn0 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn0))), (((locals.var_u0r_v_dn2 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn2)) + ((locals.var_u0_v_dn2 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn2))), (((locals.var_u0r_v_dn3 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn3)) + ((locals.var_u0_v_dn3 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn3))), (((locals.var_u0r_v_dn4 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn4)) + ((locals.var_u0_v_dn4 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn4))), (((locals.var_u0r_v_dn5 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn5)) + ((locals.var_u0_v_dn5 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn5))), (((locals.var_u0r_v_dn6 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn6)) + ((locals.var_u0_v_dn6 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn6))), (((locals.var_u0r_v_dn7 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn7)) + ((locals.var_u0_v_dn7 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn7))), (((locals.var_u0r_v_dn8 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn8)) + ((locals.var_u0_v_dn8 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn8))), (((locals.var_u0r_v_dn9 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn9)) + ((locals.var_u0_v_dn9 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn9))), (((locals.var_u0r_v_dn10 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn10)) + ((locals.var_u0_v_dn10 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn10))), (((locals.var_u0r_v_dn11 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn11)) + ((locals.var_u0_v_dn11 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn11))), (((locals.var_u0r_v_dn13 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn13)) + ((locals.var_u0_v_dn13 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn13))), (((locals.var_u0r_v_dn14 * locals.var_wr_v) + (locals.var_u0r_v * locals.var_wr_v_dn14)) + ((locals.var_u0_v_dn14 * locals.var_wf) + (locals.var_u0_v * locals.var_wf_dn14))),)
    } else {
        (locals.var_u0_a, locals.var_u0_a_dn0, locals.var_u0_a_dn2, locals.var_u0_a_dn3, locals.var_u0_a_dn4, locals.var_u0_a_dn5, locals.var_u0_a_dn6, locals.var_u0_a_dn7, locals.var_u0_a_dn8, locals.var_u0_a_dn9, locals.var_u0_a_dn10, locals.var_u0_a_dn11, locals.var_u0_a_dn13, locals.var_u0_a_dn14,)
    }
};
        locals.var_u0_a = assign19950_e37838;
        locals.var_u0_a_dn0 = assign19950_e37838_d_n0;
        locals.var_u0_a_dn2 = assign19950_e37838_d_n2;
        locals.var_u0_a_dn3 = assign19950_e37838_d_n3;
        locals.var_u0_a_dn4 = assign19950_e37838_d_n4;
        locals.var_u0_a_dn5 = assign19950_e37838_d_n5;
        locals.var_u0_a_dn6 = assign19950_e37838_d_n6;
        locals.var_u0_a_dn7 = assign19950_e37838_d_n7;
        locals.var_u0_a_dn8 = assign19950_e37838_d_n8;
        locals.var_u0_a_dn9 = assign19950_e37838_d_n9;
        locals.var_u0_a_dn10 = assign19950_e37838_d_n10;
        locals.var_u0_a_dn11 = assign19950_e37838_d_n11;
        locals.var_u0_a_dn13 = assign19950_e37838_d_n13;
        locals.var_u0_a_dn14 = assign19950_e37838_d_n14;

        let (assign19960_e37848, assign19960_e37848_d_n0, assign19960_e37848_d_n2, assign19960_e37848_d_n3, assign19960_e37848_d_n4, assign19960_e37848_d_n5, assign19960_e37848_d_n6, assign19960_e37848_d_n7, assign19960_e37848_d_n8, assign19960_e37848_d_n9, assign19960_e37848_d_n10, assign19960_e37848_d_n11, assign19960_e37848_d_n13, assign19960_e37848_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19960_e37842: f64 = (locals.var_uar_t * locals.var_wr_v);
        let assign19960_e37845: f64 = (locals.var_ua_t * locals.var_wf);
        let assign19960_e37846: f64 = (assign19960_e37842 + assign19960_e37845);
        (assign19960_e37846, (((locals.var_uar_t_dn0 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn0)) + ((locals.var_ua_t_dn0 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn0))), (((locals.var_uar_t_dn2 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn2)) + ((locals.var_ua_t_dn2 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn2))), (((locals.var_uar_t_dn3 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn3)) + ((locals.var_ua_t_dn3 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn3))), (((locals.var_uar_t_dn4 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn4)) + ((locals.var_ua_t_dn4 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn4))), (((locals.var_uar_t_dn5 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn5)) + ((locals.var_ua_t_dn5 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn5))), (((locals.var_uar_t_dn6 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn6)) + ((locals.var_ua_t_dn6 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn6))), (((locals.var_uar_t_dn7 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn7)) + ((locals.var_ua_t_dn7 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn7))), (((locals.var_uar_t_dn8 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn8)) + ((locals.var_ua_t_dn8 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn8))), (((locals.var_uar_t_dn9 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn9)) + ((locals.var_ua_t_dn9 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn9))), (((locals.var_uar_t_dn10 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn10)) + ((locals.var_ua_t_dn10 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn10))), (((locals.var_uar_t_dn11 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn11)) + ((locals.var_ua_t_dn11 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn11))), (((locals.var_uar_t_dn13 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn13)) + ((locals.var_ua_t_dn13 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn13))), (((locals.var_uar_t_dn14 * locals.var_wr_v) + (locals.var_uar_t * locals.var_wr_v_dn14)) + ((locals.var_ua_t_dn14 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_ua_a, locals.var_ua_a_dn0, locals.var_ua_a_dn2, locals.var_ua_a_dn3, locals.var_ua_a_dn4, locals.var_ua_a_dn5, locals.var_ua_a_dn6, locals.var_ua_a_dn7, locals.var_ua_a_dn8, locals.var_ua_a_dn9, locals.var_ua_a_dn10, locals.var_ua_a_dn11, locals.var_ua_a_dn13, locals.var_ua_a_dn14,)
    }
};
        locals.var_ua_a = assign19960_e37848;
        locals.var_ua_a_dn0 = assign19960_e37848_d_n0;
        locals.var_ua_a_dn2 = assign19960_e37848_d_n2;
        locals.var_ua_a_dn3 = assign19960_e37848_d_n3;
        locals.var_ua_a_dn4 = assign19960_e37848_d_n4;
        locals.var_ua_a_dn5 = assign19960_e37848_d_n5;
        locals.var_ua_a_dn6 = assign19960_e37848_d_n6;
        locals.var_ua_a_dn7 = assign19960_e37848_d_n7;
        locals.var_ua_a_dn8 = assign19960_e37848_d_n8;
        locals.var_ua_a_dn9 = assign19960_e37848_d_n9;
        locals.var_ua_a_dn10 = assign19960_e37848_d_n10;
        locals.var_ua_a_dn11 = assign19960_e37848_d_n11;
        locals.var_ua_a_dn13 = assign19960_e37848_d_n13;
        locals.var_ua_a_dn14 = assign19960_e37848_d_n14;

        let (assign19970_e37858, assign19970_e37858_d_n0, assign19970_e37858_d_n2, assign19970_e37858_d_n3, assign19970_e37858_d_n4, assign19970_e37858_d_n5, assign19970_e37858_d_n6, assign19970_e37858_d_n7, assign19970_e37858_d_n8, assign19970_e37858_d_n9, assign19970_e37858_d_n10, assign19970_e37858_d_n11, assign19970_e37858_d_n13, assign19970_e37858_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19970_e37852: f64 = (locals.var_udr_t * locals.var_wr_v);
        let assign19970_e37855: f64 = (locals.var_ud_t * locals.var_wf);
        let assign19970_e37856: f64 = (assign19970_e37852 + assign19970_e37855);
        (assign19970_e37856, (((locals.var_udr_t_dn0 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn0)) + ((locals.var_ud_t_dn0 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn0))), (((locals.var_udr_t_dn2 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn2)) + ((locals.var_ud_t_dn2 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn2))), (((locals.var_udr_t_dn3 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn3)) + ((locals.var_ud_t_dn3 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn3))), (((locals.var_udr_t_dn4 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn4)) + ((locals.var_ud_t_dn4 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn4))), (((locals.var_udr_t_dn5 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn5)) + ((locals.var_ud_t_dn5 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn5))), (((locals.var_udr_t_dn6 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn6)) + ((locals.var_ud_t_dn6 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn6))), (((locals.var_udr_t_dn7 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn7)) + ((locals.var_ud_t_dn7 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn7))), (((locals.var_udr_t_dn8 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn8)) + ((locals.var_ud_t_dn8 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn8))), (((locals.var_udr_t_dn9 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn9)) + ((locals.var_ud_t_dn9 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn9))), (((locals.var_udr_t_dn10 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn10)) + ((locals.var_ud_t_dn10 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn10))), (((locals.var_udr_t_dn11 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn11)) + ((locals.var_ud_t_dn11 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn11))), (((locals.var_udr_t_dn13 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn13)) + ((locals.var_ud_t_dn13 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn13))), (((locals.var_udr_t_dn14 * locals.var_wr_v) + (locals.var_udr_t * locals.var_wr_v_dn14)) + ((locals.var_ud_t_dn14 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_ud_a, locals.var_ud_a_dn0, locals.var_ud_a_dn2, locals.var_ud_a_dn3, locals.var_ud_a_dn4, locals.var_ud_a_dn5, locals.var_ud_a_dn6, locals.var_ud_a_dn7, locals.var_ud_a_dn8, locals.var_ud_a_dn9, locals.var_ud_a_dn10, locals.var_ud_a_dn11, locals.var_ud_a_dn13, locals.var_ud_a_dn14,)
    }
};
        locals.var_ud_a = assign19970_e37858;
        locals.var_ud_a_dn0 = assign19970_e37858_d_n0;
        locals.var_ud_a_dn2 = assign19970_e37858_d_n2;
        locals.var_ud_a_dn3 = assign19970_e37858_d_n3;
        locals.var_ud_a_dn4 = assign19970_e37858_d_n4;
        locals.var_ud_a_dn5 = assign19970_e37858_d_n5;
        locals.var_ud_a_dn6 = assign19970_e37858_d_n6;
        locals.var_ud_a_dn7 = assign19970_e37858_d_n7;
        locals.var_ud_a_dn8 = assign19970_e37858_d_n8;
        locals.var_ud_a_dn9 = assign19970_e37858_d_n9;
        locals.var_ud_a_dn10 = assign19970_e37858_d_n10;
        locals.var_ud_a_dn11 = assign19970_e37858_d_n11;
        locals.var_ud_a_dn13 = assign19970_e37858_d_n13;
        locals.var_ud_a_dn14 = assign19970_e37858_d_n14;

        let (assign19980_e37868, assign19980_e37868_d_n0, assign19980_e37868_d_n2, assign19980_e37868_d_n3, assign19980_e37868_d_n4, assign19980_e37868_d_n5, assign19980_e37868_d_n6, assign19980_e37868_d_n7, assign19980_e37868_d_n8, assign19980_e37868_d_n9, assign19980_e37868_d_n10, assign19980_e37868_d_n11, assign19980_e37868_d_n13, assign19980_e37868_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19980_e37862: f64 = (locals.var_ucr_t * locals.var_wr_v);
        let assign19980_e37865: f64 = (locals.var_uc_t * locals.var_wf);
        let assign19980_e37866: f64 = (assign19980_e37862 + assign19980_e37865);
        (assign19980_e37866, ((locals.var_ucr_t * locals.var_wr_v_dn0) + (locals.var_uc_t * locals.var_wf_dn0)), ((locals.var_ucr_t * locals.var_wr_v_dn2) + (locals.var_uc_t * locals.var_wf_dn2)), ((locals.var_ucr_t * locals.var_wr_v_dn3) + (locals.var_uc_t * locals.var_wf_dn3)), (((locals.var_ucr_t_dn4 * locals.var_wr_v) + (locals.var_ucr_t * locals.var_wr_v_dn4)) + ((locals.var_uc_t_dn4 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn4))), ((locals.var_ucr_t * locals.var_wr_v_dn5) + (locals.var_uc_t * locals.var_wf_dn5)), ((locals.var_ucr_t * locals.var_wr_v_dn6) + (locals.var_uc_t * locals.var_wf_dn6)), ((locals.var_ucr_t * locals.var_wr_v_dn7) + (locals.var_uc_t * locals.var_wf_dn7)), ((locals.var_ucr_t * locals.var_wr_v_dn8) + (locals.var_uc_t * locals.var_wf_dn8)), ((locals.var_ucr_t * locals.var_wr_v_dn9) + (locals.var_uc_t * locals.var_wf_dn9)), ((locals.var_ucr_t * locals.var_wr_v_dn10) + (locals.var_uc_t * locals.var_wf_dn10)), ((locals.var_ucr_t * locals.var_wr_v_dn11) + (locals.var_uc_t * locals.var_wf_dn11)), ((locals.var_ucr_t * locals.var_wr_v_dn13) + (locals.var_uc_t * locals.var_wf_dn13)), ((locals.var_ucr_t * locals.var_wr_v_dn14) + (locals.var_uc_t * locals.var_wf_dn14)),)
    } else {
        (locals.var_uc_a, locals.var_uc_a_dn0, locals.var_uc_a_dn2, locals.var_uc_a_dn3, locals.var_uc_a_dn4, locals.var_uc_a_dn5, locals.var_uc_a_dn6, locals.var_uc_a_dn7, locals.var_uc_a_dn8, locals.var_uc_a_dn9, locals.var_uc_a_dn10, locals.var_uc_a_dn11, locals.var_uc_a_dn13, locals.var_uc_a_dn14,)
    }
};
        locals.var_uc_a = assign19980_e37868;
        locals.var_uc_a_dn0 = assign19980_e37868_d_n0;
        locals.var_uc_a_dn2 = assign19980_e37868_d_n2;
        locals.var_uc_a_dn3 = assign19980_e37868_d_n3;
        locals.var_uc_a_dn4 = assign19980_e37868_d_n4;
        locals.var_uc_a_dn5 = assign19980_e37868_d_n5;
        locals.var_uc_a_dn6 = assign19980_e37868_d_n6;
        locals.var_uc_a_dn7 = assign19980_e37868_d_n7;
        locals.var_uc_a_dn8 = assign19980_e37868_d_n8;
        locals.var_uc_a_dn9 = assign19980_e37868_d_n9;
        locals.var_uc_a_dn10 = assign19980_e37868_d_n10;
        locals.var_uc_a_dn11 = assign19980_e37868_d_n11;
        locals.var_uc_a_dn13 = assign19980_e37868_d_n13;
        locals.var_uc_a_dn14 = assign19980_e37868_d_n14;

        let (assign19990_e37878, assign19990_e37878_d_n0, assign19990_e37878_d_n2, assign19990_e37878_d_n3, assign19990_e37878_d_n4, assign19990_e37878_d_n5, assign19990_e37878_d_n6, assign19990_e37878_d_n7, assign19990_e37878_d_n8, assign19990_e37878_d_n9, assign19990_e37878_d_n10, assign19990_e37878_d_n11, assign19990_e37878_d_n13, assign19990_e37878_d_n14,) = {
    if (locals.var_guard362 != 0.0) {
        let assign19990_e37872: f64 = (locals.var_eur_i * locals.var_wr_v);
        let assign19990_e37875: f64 = (locals.var_eu_t * locals.var_wf);
        let assign19990_e37876: f64 = (assign19990_e37872 + assign19990_e37875);
        (assign19990_e37876, (((locals.var_eur_i_dn0 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn0)) + ((locals.var_eu_t_dn0 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn0))), (((locals.var_eur_i_dn2 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn2)) + ((locals.var_eu_t_dn2 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn2))), (((locals.var_eur_i_dn3 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn3)) + ((locals.var_eu_t_dn3 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn3))), (((locals.var_eur_i_dn4 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn4)) + ((locals.var_eu_t_dn4 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn4))), (((locals.var_eur_i_dn5 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn5)) + ((locals.var_eu_t_dn5 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn5))), (((locals.var_eur_i_dn6 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn6)) + ((locals.var_eu_t_dn6 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn6))), (((locals.var_eur_i_dn7 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn7)) + ((locals.var_eu_t_dn7 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn7))), (((locals.var_eur_i_dn8 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn8)) + ((locals.var_eu_t_dn8 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn8))), (((locals.var_eur_i_dn9 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn9)) + ((locals.var_eu_t_dn9 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn9))), (((locals.var_eur_i_dn10 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn10)) + ((locals.var_eu_t_dn10 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn10))), (((locals.var_eur_i_dn11 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn11)) + ((locals.var_eu_t_dn11 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn11))), (((locals.var_eur_i_dn13 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn13)) + ((locals.var_eu_t_dn13 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn13))), (((locals.var_eur_i_dn14 * locals.var_wr_v) + (locals.var_eur_i * locals.var_wr_v_dn14)) + ((locals.var_eu_t_dn14 * locals.var_wf) + (locals.var_eu_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_eu_a, locals.var_eu_a_dn0, locals.var_eu_a_dn2, locals.var_eu_a_dn3, locals.var_eu_a_dn4, locals.var_eu_a_dn5, locals.var_eu_a_dn6, locals.var_eu_a_dn7, locals.var_eu_a_dn8, locals.var_eu_a_dn9, locals.var_eu_a_dn10, locals.var_eu_a_dn11, locals.var_eu_a_dn13, locals.var_eu_a_dn14,)
    }
};
        locals.var_eu_a = assign19990_e37878;
        locals.var_eu_a_dn0 = assign19990_e37878_d_n0;
        locals.var_eu_a_dn2 = assign19990_e37878_d_n2;
        locals.var_eu_a_dn3 = assign19990_e37878_d_n3;
        locals.var_eu_a_dn4 = assign19990_e37878_d_n4;
        locals.var_eu_a_dn5 = assign19990_e37878_d_n5;
        locals.var_eu_a_dn6 = assign19990_e37878_d_n6;
        locals.var_eu_a_dn7 = assign19990_e37878_d_n7;
        locals.var_eu_a_dn8 = assign19990_e37878_d_n8;
        locals.var_eu_a_dn9 = assign19990_e37878_d_n9;
        locals.var_eu_a_dn10 = assign19990_e37878_d_n10;
        locals.var_eu_a_dn11 = assign19990_e37878_d_n11;
        locals.var_eu_a_dn13 = assign19990_e37878_d_n13;
        locals.var_eu_a_dn14 = assign19990_e37878_d_n14;

        let (assign20000_e37883, assign20000_e37883_d_n0, assign20000_e37883_d_n2, assign20000_e37883_d_n3, assign20000_e37883_d_n4, assign20000_e37883_d_n5, assign20000_e37883_d_n6, assign20000_e37883_d_n7, assign20000_e37883_d_n8, assign20000_e37883_d_n9, assign20000_e37883_d_n10, assign20000_e37883_d_n11, assign20000_e37883_d_n13, assign20000_e37883_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_cdscd_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cdscd_a, locals.var_cdscd_a_dn0, locals.var_cdscd_a_dn2, locals.var_cdscd_a_dn3, locals.var_cdscd_a_dn4, locals.var_cdscd_a_dn5, locals.var_cdscd_a_dn6, locals.var_cdscd_a_dn7, locals.var_cdscd_a_dn8, locals.var_cdscd_a_dn9, locals.var_cdscd_a_dn10, locals.var_cdscd_a_dn11, locals.var_cdscd_a_dn13, locals.var_cdscd_a_dn14,)
    }
};
        locals.var_cdscd_a = assign20000_e37883;
        locals.var_cdscd_a_dn0 = assign20000_e37883_d_n0;
        locals.var_cdscd_a_dn2 = assign20000_e37883_d_n2;
        locals.var_cdscd_a_dn3 = assign20000_e37883_d_n3;
        locals.var_cdscd_a_dn4 = assign20000_e37883_d_n4;
        locals.var_cdscd_a_dn5 = assign20000_e37883_d_n5;
        locals.var_cdscd_a_dn6 = assign20000_e37883_d_n6;
        locals.var_cdscd_a_dn7 = assign20000_e37883_d_n7;
        locals.var_cdscd_a_dn8 = assign20000_e37883_d_n8;
        locals.var_cdscd_a_dn9 = assign20000_e37883_d_n9;
        locals.var_cdscd_a_dn10 = assign20000_e37883_d_n10;
        locals.var_cdscd_a_dn11 = assign20000_e37883_d_n11;
        locals.var_cdscd_a_dn13 = assign20000_e37883_d_n13;
        locals.var_cdscd_a_dn14 = assign20000_e37883_d_n14;

        let (assign20010_e37888, assign20010_e37888_d_n0, assign20010_e37888_d_n2, assign20010_e37888_d_n3, assign20010_e37888_d_n4, assign20010_e37888_d_n5, assign20010_e37888_d_n6, assign20010_e37888_d_n7, assign20010_e37888_d_n8, assign20010_e37888_d_n9, assign20010_e37888_d_n10, assign20010_e37888_d_n11, assign20010_e37888_d_n13, assign20010_e37888_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_eta0_t, locals.var_eta0_t_dn0, locals.var_eta0_t_dn2, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11, locals.var_eta0_t_dn13, locals.var_eta0_t_dn14,)
    } else {
        (locals.var_eta0_a, locals.var_eta0_a_dn0, locals.var_eta0_a_dn2, locals.var_eta0_a_dn3, locals.var_eta0_a_dn4, locals.var_eta0_a_dn5, locals.var_eta0_a_dn6, locals.var_eta0_a_dn7, locals.var_eta0_a_dn8, locals.var_eta0_a_dn9, locals.var_eta0_a_dn10, locals.var_eta0_a_dn11, locals.var_eta0_a_dn13, locals.var_eta0_a_dn14,)
    }
};
        locals.var_eta0_a = assign20010_e37888;
        locals.var_eta0_a_dn0 = assign20010_e37888_d_n0;
        locals.var_eta0_a_dn2 = assign20010_e37888_d_n2;
        locals.var_eta0_a_dn3 = assign20010_e37888_d_n3;
        locals.var_eta0_a_dn4 = assign20010_e37888_d_n4;
        locals.var_eta0_a_dn5 = assign20010_e37888_d_n5;
        locals.var_eta0_a_dn6 = assign20010_e37888_d_n6;
        locals.var_eta0_a_dn7 = assign20010_e37888_d_n7;
        locals.var_eta0_a_dn8 = assign20010_e37888_d_n8;
        locals.var_eta0_a_dn9 = assign20010_e37888_d_n9;
        locals.var_eta0_a_dn10 = assign20010_e37888_d_n10;
        locals.var_eta0_a_dn11 = assign20010_e37888_d_n11;
        locals.var_eta0_a_dn13 = assign20010_e37888_d_n13;
        locals.var_eta0_a_dn14 = assign20010_e37888_d_n14;

        let (assign20020_e37893, assign20020_e37893_d_n0, assign20020_e37893_d_n2, assign20020_e37893_d_n3, assign20020_e37893_d_n4, assign20020_e37893_d_n5, assign20020_e37893_d_n6, assign20020_e37893_d_n7, assign20020_e37893_d_n8, assign20020_e37893_d_n9, assign20020_e37893_d_n10, assign20020_e37893_d_n11, assign20020_e37893_d_n13, assign20020_e37893_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_pdibl1_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdibl1_a, locals.var_pdibl1_a_dn0, locals.var_pdibl1_a_dn2, locals.var_pdibl1_a_dn3, locals.var_pdibl1_a_dn4, locals.var_pdibl1_a_dn5, locals.var_pdibl1_a_dn6, locals.var_pdibl1_a_dn7, locals.var_pdibl1_a_dn8, locals.var_pdibl1_a_dn9, locals.var_pdibl1_a_dn10, locals.var_pdibl1_a_dn11, locals.var_pdibl1_a_dn13, locals.var_pdibl1_a_dn14,)
    }
};
        locals.var_pdibl1_a = assign20020_e37893;
        locals.var_pdibl1_a_dn0 = assign20020_e37893_d_n0;
        locals.var_pdibl1_a_dn2 = assign20020_e37893_d_n2;
        locals.var_pdibl1_a_dn3 = assign20020_e37893_d_n3;
        locals.var_pdibl1_a_dn4 = assign20020_e37893_d_n4;
        locals.var_pdibl1_a_dn5 = assign20020_e37893_d_n5;
        locals.var_pdibl1_a_dn6 = assign20020_e37893_d_n6;
        locals.var_pdibl1_a_dn7 = assign20020_e37893_d_n7;
        locals.var_pdibl1_a_dn8 = assign20020_e37893_d_n8;
        locals.var_pdibl1_a_dn9 = assign20020_e37893_d_n9;
        locals.var_pdibl1_a_dn10 = assign20020_e37893_d_n10;
        locals.var_pdibl1_a_dn11 = assign20020_e37893_d_n11;
        locals.var_pdibl1_a_dn13 = assign20020_e37893_d_n13;
        locals.var_pdibl1_a_dn14 = assign20020_e37893_d_n14;

        let (assign20030_e37898, assign20030_e37898_d_n0, assign20030_e37898_d_n2, assign20030_e37898_d_n3, assign20030_e37898_d_n4, assign20030_e37898_d_n5, assign20030_e37898_d_n6, assign20030_e37898_d_n7, assign20030_e37898_d_n8, assign20030_e37898_d_n9, assign20030_e37898_d_n10, assign20030_e37898_d_n11, assign20030_e37898_d_n13, assign20030_e37898_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_pdibl2_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdibl2_a, locals.var_pdibl2_a_dn0, locals.var_pdibl2_a_dn2, locals.var_pdibl2_a_dn3, locals.var_pdibl2_a_dn4, locals.var_pdibl2_a_dn5, locals.var_pdibl2_a_dn6, locals.var_pdibl2_a_dn7, locals.var_pdibl2_a_dn8, locals.var_pdibl2_a_dn9, locals.var_pdibl2_a_dn10, locals.var_pdibl2_a_dn11, locals.var_pdibl2_a_dn13, locals.var_pdibl2_a_dn14,)
    }
};
        locals.var_pdibl2_a = assign20030_e37898;
        locals.var_pdibl2_a_dn0 = assign20030_e37898_d_n0;
        locals.var_pdibl2_a_dn2 = assign20030_e37898_d_n2;
        locals.var_pdibl2_a_dn3 = assign20030_e37898_d_n3;
        locals.var_pdibl2_a_dn4 = assign20030_e37898_d_n4;
        locals.var_pdibl2_a_dn5 = assign20030_e37898_d_n5;
        locals.var_pdibl2_a_dn6 = assign20030_e37898_d_n6;
        locals.var_pdibl2_a_dn7 = assign20030_e37898_d_n7;
        locals.var_pdibl2_a_dn8 = assign20030_e37898_d_n8;
        locals.var_pdibl2_a_dn9 = assign20030_e37898_d_n9;
        locals.var_pdibl2_a_dn10 = assign20030_e37898_d_n10;
        locals.var_pdibl2_a_dn11 = assign20030_e37898_d_n11;
        locals.var_pdibl2_a_dn13 = assign20030_e37898_d_n13;
        locals.var_pdibl2_a_dn14 = assign20030_e37898_d_n14;

        let (assign20040_e37903, assign20040_e37903_d_n0, assign20040_e37903_d_n2, assign20040_e37903_d_n3, assign20040_e37903_d_n4, assign20040_e37903_d_n5, assign20040_e37903_d_n6, assign20040_e37903_d_n7, assign20040_e37903_d_n8, assign20040_e37903_d_n9, assign20040_e37903_d_n10, assign20040_e37903_d_n11, assign20040_e37903_d_n13, assign20040_e37903_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_mexp_t, locals.var_mexp_t_dn0, locals.var_mexp_t_dn2, locals.var_mexp_t_dn3, locals.var_mexp_t_dn4, locals.var_mexp_t_dn5, locals.var_mexp_t_dn6, locals.var_mexp_t_dn7, locals.var_mexp_t_dn8, locals.var_mexp_t_dn9, locals.var_mexp_t_dn10, locals.var_mexp_t_dn11, locals.var_mexp_t_dn13, locals.var_mexp_t_dn14,)
    } else {
        (locals.var_mexp_a, locals.var_mexp_a_dn0, locals.var_mexp_a_dn2, locals.var_mexp_a_dn3, locals.var_mexp_a_dn4, locals.var_mexp_a_dn5, locals.var_mexp_a_dn6, locals.var_mexp_a_dn7, locals.var_mexp_a_dn8, locals.var_mexp_a_dn9, locals.var_mexp_a_dn10, locals.var_mexp_a_dn11, locals.var_mexp_a_dn13, locals.var_mexp_a_dn14,)
    }
};
        locals.var_mexp_a = assign20040_e37903;
        locals.var_mexp_a_dn0 = assign20040_e37903_d_n0;
        locals.var_mexp_a_dn2 = assign20040_e37903_d_n2;
        locals.var_mexp_a_dn3 = assign20040_e37903_d_n3;
        locals.var_mexp_a_dn4 = assign20040_e37903_d_n4;
        locals.var_mexp_a_dn5 = assign20040_e37903_d_n5;
        locals.var_mexp_a_dn6 = assign20040_e37903_d_n6;
        locals.var_mexp_a_dn7 = assign20040_e37903_d_n7;
        locals.var_mexp_a_dn8 = assign20040_e37903_d_n8;
        locals.var_mexp_a_dn9 = assign20040_e37903_d_n9;
        locals.var_mexp_a_dn10 = assign20040_e37903_d_n10;
        locals.var_mexp_a_dn11 = assign20040_e37903_d_n11;
        locals.var_mexp_a_dn13 = assign20040_e37903_d_n13;
        locals.var_mexp_a_dn14 = assign20040_e37903_d_n14;

        let (assign20050_e37908, assign20050_e37908_d_n0, assign20050_e37908_d_n2, assign20050_e37908_d_n3, assign20050_e37908_d_n4, assign20050_e37908_d_n5, assign20050_e37908_d_n6, assign20050_e37908_d_n7, assign20050_e37908_d_n8, assign20050_e37908_d_n9, assign20050_e37908_d_n10, assign20050_e37908_d_n11, assign20050_e37908_d_n13, assign20050_e37908_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_ptwg_t, locals.var_ptwg_t_dn0, locals.var_ptwg_t_dn2, locals.var_ptwg_t_dn3, locals.var_ptwg_t_dn4, locals.var_ptwg_t_dn5, locals.var_ptwg_t_dn6, locals.var_ptwg_t_dn7, locals.var_ptwg_t_dn8, locals.var_ptwg_t_dn9, locals.var_ptwg_t_dn10, locals.var_ptwg_t_dn11, locals.var_ptwg_t_dn13, locals.var_ptwg_t_dn14,)
    } else {
        (locals.var_ptwg_a, locals.var_ptwg_a_dn0, locals.var_ptwg_a_dn2, locals.var_ptwg_a_dn3, locals.var_ptwg_a_dn4, locals.var_ptwg_a_dn5, locals.var_ptwg_a_dn6, locals.var_ptwg_a_dn7, locals.var_ptwg_a_dn8, locals.var_ptwg_a_dn9, locals.var_ptwg_a_dn10, locals.var_ptwg_a_dn11, locals.var_ptwg_a_dn13, locals.var_ptwg_a_dn14,)
    }
};
        locals.var_ptwg_a = assign20050_e37908;
        locals.var_ptwg_a_dn0 = assign20050_e37908_d_n0;
        locals.var_ptwg_a_dn2 = assign20050_e37908_d_n2;
        locals.var_ptwg_a_dn3 = assign20050_e37908_d_n3;
        locals.var_ptwg_a_dn4 = assign20050_e37908_d_n4;
        locals.var_ptwg_a_dn5 = assign20050_e37908_d_n5;
        locals.var_ptwg_a_dn6 = assign20050_e37908_d_n6;
        locals.var_ptwg_a_dn7 = assign20050_e37908_d_n7;
        locals.var_ptwg_a_dn8 = assign20050_e37908_d_n8;
        locals.var_ptwg_a_dn9 = assign20050_e37908_d_n9;
        locals.var_ptwg_a_dn10 = assign20050_e37908_d_n10;
        locals.var_ptwg_a_dn11 = assign20050_e37908_d_n11;
        locals.var_ptwg_a_dn13 = assign20050_e37908_d_n13;
        locals.var_ptwg_a_dn14 = assign20050_e37908_d_n14;

        let (assign20060_e37913, assign20060_e37913_d_n0, assign20060_e37913_d_n2, assign20060_e37913_d_n3, assign20060_e37913_d_n4, assign20060_e37913_d_n5, assign20060_e37913_d_n6, assign20060_e37913_d_n7, assign20060_e37913_d_n8, assign20060_e37913_d_n9, assign20060_e37913_d_n10, assign20060_e37913_d_n11, assign20060_e37913_d_n13, assign20060_e37913_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    } else {
        (locals.var_vsat1_a, locals.var_vsat1_a_dn0, locals.var_vsat1_a_dn2, locals.var_vsat1_a_dn3, locals.var_vsat1_a_dn4, locals.var_vsat1_a_dn5, locals.var_vsat1_a_dn6, locals.var_vsat1_a_dn7, locals.var_vsat1_a_dn8, locals.var_vsat1_a_dn9, locals.var_vsat1_a_dn10, locals.var_vsat1_a_dn11, locals.var_vsat1_a_dn13, locals.var_vsat1_a_dn14,)
    }
};
        locals.var_vsat1_a = assign20060_e37913;
        locals.var_vsat1_a_dn0 = assign20060_e37913_d_n0;
        locals.var_vsat1_a_dn2 = assign20060_e37913_d_n2;
        locals.var_vsat1_a_dn3 = assign20060_e37913_d_n3;
        locals.var_vsat1_a_dn4 = assign20060_e37913_d_n4;
        locals.var_vsat1_a_dn5 = assign20060_e37913_d_n5;
        locals.var_vsat1_a_dn6 = assign20060_e37913_d_n6;
        locals.var_vsat1_a_dn7 = assign20060_e37913_d_n7;
        locals.var_vsat1_a_dn8 = assign20060_e37913_d_n8;
        locals.var_vsat1_a_dn9 = assign20060_e37913_d_n9;
        locals.var_vsat1_a_dn10 = assign20060_e37913_d_n10;
        locals.var_vsat1_a_dn11 = assign20060_e37913_d_n11;
        locals.var_vsat1_a_dn13 = assign20060_e37913_d_n13;
        locals.var_vsat1_a_dn14 = assign20060_e37913_d_n14;

        let (assign20070_e37918, assign20070_e37918_d_n0, assign20070_e37918_d_n2, assign20070_e37918_d_n3, assign20070_e37918_d_n4, assign20070_e37918_d_n5, assign20070_e37918_d_n6, assign20070_e37918_d_n7, assign20070_e37918_d_n8, assign20070_e37918_d_n9, assign20070_e37918_d_n10, assign20070_e37918_d_n11, assign20070_e37918_d_n13, assign20070_e37918_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_rsdr_t, 0.0, 0.0, 0.0, locals.var_rsdr_t_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsdr_a, locals.var_rsdr_a_dn0, locals.var_rsdr_a_dn2, locals.var_rsdr_a_dn3, locals.var_rsdr_a_dn4, locals.var_rsdr_a_dn5, locals.var_rsdr_a_dn6, locals.var_rsdr_a_dn7, locals.var_rsdr_a_dn8, locals.var_rsdr_a_dn9, locals.var_rsdr_a_dn10, locals.var_rsdr_a_dn11, locals.var_rsdr_a_dn13, locals.var_rsdr_a_dn14,)
    }
};
        locals.var_rsdr_a = assign20070_e37918;
        locals.var_rsdr_a_dn0 = assign20070_e37918_d_n0;
        locals.var_rsdr_a_dn2 = assign20070_e37918_d_n2;
        locals.var_rsdr_a_dn3 = assign20070_e37918_d_n3;
        locals.var_rsdr_a_dn4 = assign20070_e37918_d_n4;
        locals.var_rsdr_a_dn5 = assign20070_e37918_d_n5;
        locals.var_rsdr_a_dn6 = assign20070_e37918_d_n6;
        locals.var_rsdr_a_dn7 = assign20070_e37918_d_n7;
        locals.var_rsdr_a_dn8 = assign20070_e37918_d_n8;
        locals.var_rsdr_a_dn9 = assign20070_e37918_d_n9;
        locals.var_rsdr_a_dn10 = assign20070_e37918_d_n10;
        locals.var_rsdr_a_dn11 = assign20070_e37918_d_n11;
        locals.var_rsdr_a_dn13 = assign20070_e37918_d_n13;
        locals.var_rsdr_a_dn14 = assign20070_e37918_d_n14;

        let (assign20080_e37923, assign20080_e37923_d_n0, assign20080_e37923_d_n2, assign20080_e37923_d_n3, assign20080_e37923_d_n4, assign20080_e37923_d_n5, assign20080_e37923_d_n6, assign20080_e37923_d_n7, assign20080_e37923_d_n8, assign20080_e37923_d_n9, assign20080_e37923_d_n10, assign20080_e37923_d_n11, assign20080_e37923_d_n13, assign20080_e37923_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_rddr_t, 0.0, 0.0, 0.0, locals.var_rddr_t_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rddr_a, locals.var_rddr_a_dn0, locals.var_rddr_a_dn2, locals.var_rddr_a_dn3, locals.var_rddr_a_dn4, locals.var_rddr_a_dn5, locals.var_rddr_a_dn6, locals.var_rddr_a_dn7, locals.var_rddr_a_dn8, locals.var_rddr_a_dn9, locals.var_rddr_a_dn10, locals.var_rddr_a_dn11, locals.var_rddr_a_dn13, locals.var_rddr_a_dn14,)
    }
};
        locals.var_rddr_a = assign20080_e37923;
        locals.var_rddr_a_dn0 = assign20080_e37923_d_n0;
        locals.var_rddr_a_dn2 = assign20080_e37923_d_n2;
        locals.var_rddr_a_dn3 = assign20080_e37923_d_n3;
        locals.var_rddr_a_dn4 = assign20080_e37923_d_n4;
        locals.var_rddr_a_dn5 = assign20080_e37923_d_n5;
        locals.var_rddr_a_dn6 = assign20080_e37923_d_n6;
        locals.var_rddr_a_dn7 = assign20080_e37923_d_n7;
        locals.var_rddr_a_dn8 = assign20080_e37923_d_n8;
        locals.var_rddr_a_dn9 = assign20080_e37923_d_n9;
        locals.var_rddr_a_dn10 = assign20080_e37923_d_n10;
        locals.var_rddr_a_dn11 = assign20080_e37923_d_n11;
        locals.var_rddr_a_dn13 = assign20080_e37923_d_n13;
        locals.var_rddr_a_dn14 = assign20080_e37923_d_n14;

        let (assign20090_e37928, assign20090_e37928_d_n0, assign20090_e37928_d_n2, assign20090_e37928_d_n3, assign20090_e37928_d_n4, assign20090_e37928_d_n5, assign20090_e37928_d_n6, assign20090_e37928_d_n7, assign20090_e37928_d_n8, assign20090_e37928_d_n9, assign20090_e37928_d_n10, assign20090_e37928_d_n11, assign20090_e37928_d_n13, assign20090_e37928_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_pclm_t, locals.var_pclm_t_dn0, locals.var_pclm_t_dn2, locals.var_pclm_t_dn3, locals.var_pclm_t_dn4, locals.var_pclm_t_dn5, locals.var_pclm_t_dn6, locals.var_pclm_t_dn7, locals.var_pclm_t_dn8, locals.var_pclm_t_dn9, locals.var_pclm_t_dn10, locals.var_pclm_t_dn11, locals.var_pclm_t_dn13, locals.var_pclm_t_dn14,)
    } else {
        (locals.var_pclm_a, locals.var_pclm_a_dn0, locals.var_pclm_a_dn2, locals.var_pclm_a_dn3, locals.var_pclm_a_dn4, locals.var_pclm_a_dn5, locals.var_pclm_a_dn6, locals.var_pclm_a_dn7, locals.var_pclm_a_dn8, locals.var_pclm_a_dn9, locals.var_pclm_a_dn10, locals.var_pclm_a_dn11, locals.var_pclm_a_dn13, locals.var_pclm_a_dn14,)
    }
};
        locals.var_pclm_a = assign20090_e37928;
        locals.var_pclm_a_dn0 = assign20090_e37928_d_n0;
        locals.var_pclm_a_dn2 = assign20090_e37928_d_n2;
        locals.var_pclm_a_dn3 = assign20090_e37928_d_n3;
        locals.var_pclm_a_dn4 = assign20090_e37928_d_n4;
        locals.var_pclm_a_dn5 = assign20090_e37928_d_n5;
        locals.var_pclm_a_dn6 = assign20090_e37928_d_n6;
        locals.var_pclm_a_dn7 = assign20090_e37928_d_n7;
        locals.var_pclm_a_dn8 = assign20090_e37928_d_n8;
        locals.var_pclm_a_dn9 = assign20090_e37928_d_n9;
        locals.var_pclm_a_dn10 = assign20090_e37928_d_n10;
        locals.var_pclm_a_dn11 = assign20090_e37928_d_n11;
        locals.var_pclm_a_dn13 = assign20090_e37928_d_n13;
        locals.var_pclm_a_dn14 = assign20090_e37928_d_n14;

        let (assign20100_e37933, assign20100_e37933_d_n0, assign20100_e37933_d_n2, assign20100_e37933_d_n3, assign20100_e37933_d_n4, assign20100_e37933_d_n5, assign20100_e37933_d_n6, assign20100_e37933_d_n7, assign20100_e37933_d_n8, assign20100_e37933_d_n9, assign20100_e37933_d_n10, assign20100_e37933_d_n11, assign20100_e37933_d_n13, assign20100_e37933_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    } else {
        (locals.var_vsat_a, locals.var_vsat_a_dn0, locals.var_vsat_a_dn2, locals.var_vsat_a_dn3, locals.var_vsat_a_dn4, locals.var_vsat_a_dn5, locals.var_vsat_a_dn6, locals.var_vsat_a_dn7, locals.var_vsat_a_dn8, locals.var_vsat_a_dn9, locals.var_vsat_a_dn10, locals.var_vsat_a_dn11, locals.var_vsat_a_dn13, locals.var_vsat_a_dn14,)
    }
};
        locals.var_vsat_a = assign20100_e37933;
        locals.var_vsat_a_dn0 = assign20100_e37933_d_n0;
        locals.var_vsat_a_dn2 = assign20100_e37933_d_n2;
        locals.var_vsat_a_dn3 = assign20100_e37933_d_n3;
        locals.var_vsat_a_dn4 = assign20100_e37933_d_n4;
        locals.var_vsat_a_dn5 = assign20100_e37933_d_n5;
        locals.var_vsat_a_dn6 = assign20100_e37933_d_n6;
        locals.var_vsat_a_dn7 = assign20100_e37933_d_n7;
        locals.var_vsat_a_dn8 = assign20100_e37933_d_n8;
        locals.var_vsat_a_dn9 = assign20100_e37933_d_n9;
        locals.var_vsat_a_dn10 = assign20100_e37933_d_n10;
        locals.var_vsat_a_dn11 = assign20100_e37933_d_n11;
        locals.var_vsat_a_dn13 = assign20100_e37933_d_n13;
        locals.var_vsat_a_dn14 = assign20100_e37933_d_n14;

        let (assign20110_e37938, assign20110_e37938_d_n0, assign20110_e37938_d_n2, assign20110_e37938_d_n3, assign20110_e37938_d_n4, assign20110_e37938_d_n5, assign20110_e37938_d_n6, assign20110_e37938_d_n7, assign20110_e37938_d_n8, assign20110_e37938_d_n9, assign20110_e37938_d_n10, assign20110_e37938_d_n11, assign20110_e37938_d_n13, assign20110_e37938_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_ksativ_t, 0.0, 0.0, 0.0, locals.var_ksativ_t_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ksativ_a, locals.var_ksativ_a_dn0, locals.var_ksativ_a_dn2, locals.var_ksativ_a_dn3, locals.var_ksativ_a_dn4, locals.var_ksativ_a_dn5, locals.var_ksativ_a_dn6, locals.var_ksativ_a_dn7, locals.var_ksativ_a_dn8, locals.var_ksativ_a_dn9, locals.var_ksativ_a_dn10, locals.var_ksativ_a_dn11, locals.var_ksativ_a_dn13, locals.var_ksativ_a_dn14,)
    }
};
        locals.var_ksativ_a = assign20110_e37938;
        locals.var_ksativ_a_dn0 = assign20110_e37938_d_n0;
        locals.var_ksativ_a_dn2 = assign20110_e37938_d_n2;
        locals.var_ksativ_a_dn3 = assign20110_e37938_d_n3;
        locals.var_ksativ_a_dn4 = assign20110_e37938_d_n4;
        locals.var_ksativ_a_dn5 = assign20110_e37938_d_n5;
        locals.var_ksativ_a_dn6 = assign20110_e37938_d_n6;
        locals.var_ksativ_a_dn7 = assign20110_e37938_d_n7;
        locals.var_ksativ_a_dn8 = assign20110_e37938_d_n8;
        locals.var_ksativ_a_dn9 = assign20110_e37938_d_n9;
        locals.var_ksativ_a_dn10 = assign20110_e37938_d_n10;
        locals.var_ksativ_a_dn11 = assign20110_e37938_d_n11;
        locals.var_ksativ_a_dn13 = assign20110_e37938_d_n13;
        locals.var_ksativ_a_dn14 = assign20110_e37938_d_n14;

        let (assign20120_e37943, assign20120_e37943_d_n0, assign20120_e37943_d_n2, assign20120_e37943_d_n3, assign20120_e37943_d_n4, assign20120_e37943_d_n5, assign20120_e37943_d_n6, assign20120_e37943_d_n7, assign20120_e37943_d_n8, assign20120_e37943_d_n9, assign20120_e37943_d_n10, assign20120_e37943_d_n11, assign20120_e37943_d_n13, assign20120_e37943_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_dvtshift_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvtshift_a, locals.var_dvtshift_a_dn0, locals.var_dvtshift_a_dn2, locals.var_dvtshift_a_dn3, locals.var_dvtshift_a_dn4, locals.var_dvtshift_a_dn5, locals.var_dvtshift_a_dn6, locals.var_dvtshift_a_dn7, locals.var_dvtshift_a_dn8, locals.var_dvtshift_a_dn9, locals.var_dvtshift_a_dn10, locals.var_dvtshift_a_dn11, locals.var_dvtshift_a_dn13, locals.var_dvtshift_a_dn14,)
    }
};
        locals.var_dvtshift_a = assign20120_e37943;
        locals.var_dvtshift_a_dn0 = assign20120_e37943_d_n0;
        locals.var_dvtshift_a_dn2 = assign20120_e37943_d_n2;
        locals.var_dvtshift_a_dn3 = assign20120_e37943_d_n3;
        locals.var_dvtshift_a_dn4 = assign20120_e37943_d_n4;
        locals.var_dvtshift_a_dn5 = assign20120_e37943_d_n5;
        locals.var_dvtshift_a_dn6 = assign20120_e37943_d_n6;
        locals.var_dvtshift_a_dn7 = assign20120_e37943_d_n7;
        locals.var_dvtshift_a_dn8 = assign20120_e37943_d_n8;
        locals.var_dvtshift_a_dn9 = assign20120_e37943_d_n9;
        locals.var_dvtshift_a_dn10 = assign20120_e37943_d_n10;
        locals.var_dvtshift_a_dn11 = assign20120_e37943_d_n11;
        locals.var_dvtshift_a_dn13 = assign20120_e37943_d_n13;
        locals.var_dvtshift_a_dn14 = assign20120_e37943_d_n14;

    }

    pub(super) fn stamp_transient_block_73(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20130_e37948, assign20130_e37948_d_n0, assign20130_e37948_d_n2, assign20130_e37948_d_n3, assign20130_e37948_d_n4, assign20130_e37948_d_n5, assign20130_e37948_d_n6, assign20130_e37948_d_n7, assign20130_e37948_d_n8, assign20130_e37948_d_n9, assign20130_e37948_d_n10, assign20130_e37948_d_n11, assign20130_e37948_d_n13, assign20130_e37948_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_cit_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cit_a, locals.var_cit_a_dn0, locals.var_cit_a_dn2, locals.var_cit_a_dn3, locals.var_cit_a_dn4, locals.var_cit_a_dn5, locals.var_cit_a_dn6, locals.var_cit_a_dn7, locals.var_cit_a_dn8, locals.var_cit_a_dn9, locals.var_cit_a_dn10, locals.var_cit_a_dn11, locals.var_cit_a_dn13, locals.var_cit_a_dn14,)
    }
};
        locals.var_cit_a = assign20130_e37948;
        locals.var_cit_a_dn0 = assign20130_e37948_d_n0;
        locals.var_cit_a_dn2 = assign20130_e37948_d_n2;
        locals.var_cit_a_dn3 = assign20130_e37948_d_n3;
        locals.var_cit_a_dn4 = assign20130_e37948_d_n4;
        locals.var_cit_a_dn5 = assign20130_e37948_d_n5;
        locals.var_cit_a_dn6 = assign20130_e37948_d_n6;
        locals.var_cit_a_dn7 = assign20130_e37948_d_n7;
        locals.var_cit_a_dn8 = assign20130_e37948_d_n8;
        locals.var_cit_a_dn9 = assign20130_e37948_d_n9;
        locals.var_cit_a_dn10 = assign20130_e37948_d_n10;
        locals.var_cit_a_dn11 = assign20130_e37948_d_n11;
        locals.var_cit_a_dn13 = assign20130_e37948_d_n13;
        locals.var_cit_a_dn14 = assign20130_e37948_d_n14;

        let (assign20140_e37953, assign20140_e37953_d_n0, assign20140_e37953_d_n2, assign20140_e37953_d_n3, assign20140_e37953_d_n4, assign20140_e37953_d_n5, assign20140_e37953_d_n6, assign20140_e37953_d_n7, assign20140_e37953_d_n8, assign20140_e37953_d_n9, assign20140_e37953_d_n10, assign20140_e37953_d_n11, assign20140_e37953_d_n13, assign20140_e37953_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_u0_v, locals.var_u0_v_dn0, locals.var_u0_v_dn2, locals.var_u0_v_dn3, locals.var_u0_v_dn4, locals.var_u0_v_dn5, locals.var_u0_v_dn6, locals.var_u0_v_dn7, locals.var_u0_v_dn8, locals.var_u0_v_dn9, locals.var_u0_v_dn10, locals.var_u0_v_dn11, locals.var_u0_v_dn13, locals.var_u0_v_dn14,)
    } else {
        (locals.var_u0_a, locals.var_u0_a_dn0, locals.var_u0_a_dn2, locals.var_u0_a_dn3, locals.var_u0_a_dn4, locals.var_u0_a_dn5, locals.var_u0_a_dn6, locals.var_u0_a_dn7, locals.var_u0_a_dn8, locals.var_u0_a_dn9, locals.var_u0_a_dn10, locals.var_u0_a_dn11, locals.var_u0_a_dn13, locals.var_u0_a_dn14,)
    }
};
        locals.var_u0_a = assign20140_e37953;
        locals.var_u0_a_dn0 = assign20140_e37953_d_n0;
        locals.var_u0_a_dn2 = assign20140_e37953_d_n2;
        locals.var_u0_a_dn3 = assign20140_e37953_d_n3;
        locals.var_u0_a_dn4 = assign20140_e37953_d_n4;
        locals.var_u0_a_dn5 = assign20140_e37953_d_n5;
        locals.var_u0_a_dn6 = assign20140_e37953_d_n6;
        locals.var_u0_a_dn7 = assign20140_e37953_d_n7;
        locals.var_u0_a_dn8 = assign20140_e37953_d_n8;
        locals.var_u0_a_dn9 = assign20140_e37953_d_n9;
        locals.var_u0_a_dn10 = assign20140_e37953_d_n10;
        locals.var_u0_a_dn11 = assign20140_e37953_d_n11;
        locals.var_u0_a_dn13 = assign20140_e37953_d_n13;
        locals.var_u0_a_dn14 = assign20140_e37953_d_n14;

        let (assign20150_e37958, assign20150_e37958_d_n0, assign20150_e37958_d_n2, assign20150_e37958_d_n3, assign20150_e37958_d_n4, assign20150_e37958_d_n5, assign20150_e37958_d_n6, assign20150_e37958_d_n7, assign20150_e37958_d_n8, assign20150_e37958_d_n9, assign20150_e37958_d_n10, assign20150_e37958_d_n11, assign20150_e37958_d_n13, assign20150_e37958_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn13, locals.var_ua_t_dn14,)
    } else {
        (locals.var_ua_a, locals.var_ua_a_dn0, locals.var_ua_a_dn2, locals.var_ua_a_dn3, locals.var_ua_a_dn4, locals.var_ua_a_dn5, locals.var_ua_a_dn6, locals.var_ua_a_dn7, locals.var_ua_a_dn8, locals.var_ua_a_dn9, locals.var_ua_a_dn10, locals.var_ua_a_dn11, locals.var_ua_a_dn13, locals.var_ua_a_dn14,)
    }
};
        locals.var_ua_a = assign20150_e37958;
        locals.var_ua_a_dn0 = assign20150_e37958_d_n0;
        locals.var_ua_a_dn2 = assign20150_e37958_d_n2;
        locals.var_ua_a_dn3 = assign20150_e37958_d_n3;
        locals.var_ua_a_dn4 = assign20150_e37958_d_n4;
        locals.var_ua_a_dn5 = assign20150_e37958_d_n5;
        locals.var_ua_a_dn6 = assign20150_e37958_d_n6;
        locals.var_ua_a_dn7 = assign20150_e37958_d_n7;
        locals.var_ua_a_dn8 = assign20150_e37958_d_n8;
        locals.var_ua_a_dn9 = assign20150_e37958_d_n9;
        locals.var_ua_a_dn10 = assign20150_e37958_d_n10;
        locals.var_ua_a_dn11 = assign20150_e37958_d_n11;
        locals.var_ua_a_dn13 = assign20150_e37958_d_n13;
        locals.var_ua_a_dn14 = assign20150_e37958_d_n14;

        let (assign20160_e37963, assign20160_e37963_d_n0, assign20160_e37963_d_n2, assign20160_e37963_d_n3, assign20160_e37963_d_n4, assign20160_e37963_d_n5, assign20160_e37963_d_n6, assign20160_e37963_d_n7, assign20160_e37963_d_n8, assign20160_e37963_d_n9, assign20160_e37963_d_n10, assign20160_e37963_d_n11, assign20160_e37963_d_n13, assign20160_e37963_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn13, locals.var_ud_t_dn14,)
    } else {
        (locals.var_ud_a, locals.var_ud_a_dn0, locals.var_ud_a_dn2, locals.var_ud_a_dn3, locals.var_ud_a_dn4, locals.var_ud_a_dn5, locals.var_ud_a_dn6, locals.var_ud_a_dn7, locals.var_ud_a_dn8, locals.var_ud_a_dn9, locals.var_ud_a_dn10, locals.var_ud_a_dn11, locals.var_ud_a_dn13, locals.var_ud_a_dn14,)
    }
};
        locals.var_ud_a = assign20160_e37963;
        locals.var_ud_a_dn0 = assign20160_e37963_d_n0;
        locals.var_ud_a_dn2 = assign20160_e37963_d_n2;
        locals.var_ud_a_dn3 = assign20160_e37963_d_n3;
        locals.var_ud_a_dn4 = assign20160_e37963_d_n4;
        locals.var_ud_a_dn5 = assign20160_e37963_d_n5;
        locals.var_ud_a_dn6 = assign20160_e37963_d_n6;
        locals.var_ud_a_dn7 = assign20160_e37963_d_n7;
        locals.var_ud_a_dn8 = assign20160_e37963_d_n8;
        locals.var_ud_a_dn9 = assign20160_e37963_d_n9;
        locals.var_ud_a_dn10 = assign20160_e37963_d_n10;
        locals.var_ud_a_dn11 = assign20160_e37963_d_n11;
        locals.var_ud_a_dn13 = assign20160_e37963_d_n13;
        locals.var_ud_a_dn14 = assign20160_e37963_d_n14;

        let (assign20170_e37968, assign20170_e37968_d_n0, assign20170_e37968_d_n2, assign20170_e37968_d_n3, assign20170_e37968_d_n4, assign20170_e37968_d_n5, assign20170_e37968_d_n6, assign20170_e37968_d_n7, assign20170_e37968_d_n8, assign20170_e37968_d_n9, assign20170_e37968_d_n10, assign20170_e37968_d_n11, assign20170_e37968_d_n13, assign20170_e37968_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_uc_t, 0.0, 0.0, 0.0, locals.var_uc_t_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_a, locals.var_uc_a_dn0, locals.var_uc_a_dn2, locals.var_uc_a_dn3, locals.var_uc_a_dn4, locals.var_uc_a_dn5, locals.var_uc_a_dn6, locals.var_uc_a_dn7, locals.var_uc_a_dn8, locals.var_uc_a_dn9, locals.var_uc_a_dn10, locals.var_uc_a_dn11, locals.var_uc_a_dn13, locals.var_uc_a_dn14,)
    }
};
        locals.var_uc_a = assign20170_e37968;
        locals.var_uc_a_dn0 = assign20170_e37968_d_n0;
        locals.var_uc_a_dn2 = assign20170_e37968_d_n2;
        locals.var_uc_a_dn3 = assign20170_e37968_d_n3;
        locals.var_uc_a_dn4 = assign20170_e37968_d_n4;
        locals.var_uc_a_dn5 = assign20170_e37968_d_n5;
        locals.var_uc_a_dn6 = assign20170_e37968_d_n6;
        locals.var_uc_a_dn7 = assign20170_e37968_d_n7;
        locals.var_uc_a_dn8 = assign20170_e37968_d_n8;
        locals.var_uc_a_dn9 = assign20170_e37968_d_n9;
        locals.var_uc_a_dn10 = assign20170_e37968_d_n10;
        locals.var_uc_a_dn11 = assign20170_e37968_d_n11;
        locals.var_uc_a_dn13 = assign20170_e37968_d_n13;
        locals.var_uc_a_dn14 = assign20170_e37968_d_n14;

        let (assign20180_e37973, assign20180_e37973_d_n0, assign20180_e37973_d_n2, assign20180_e37973_d_n3, assign20180_e37973_d_n4, assign20180_e37973_d_n5, assign20180_e37973_d_n6, assign20180_e37973_d_n7, assign20180_e37973_d_n8, assign20180_e37973_d_n9, assign20180_e37973_d_n10, assign20180_e37973_d_n11, assign20180_e37973_d_n13, assign20180_e37973_d_n14,) = {
    if (locals.var_guard362 == 0.0) {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    } else {
        (locals.var_eu_a, locals.var_eu_a_dn0, locals.var_eu_a_dn2, locals.var_eu_a_dn3, locals.var_eu_a_dn4, locals.var_eu_a_dn5, locals.var_eu_a_dn6, locals.var_eu_a_dn7, locals.var_eu_a_dn8, locals.var_eu_a_dn9, locals.var_eu_a_dn10, locals.var_eu_a_dn11, locals.var_eu_a_dn13, locals.var_eu_a_dn14,)
    }
};
        locals.var_eu_a = assign20180_e37973;
        locals.var_eu_a_dn0 = assign20180_e37973_d_n0;
        locals.var_eu_a_dn2 = assign20180_e37973_d_n2;
        locals.var_eu_a_dn3 = assign20180_e37973_d_n3;
        locals.var_eu_a_dn4 = assign20180_e37973_d_n4;
        locals.var_eu_a_dn5 = assign20180_e37973_d_n5;
        locals.var_eu_a_dn6 = assign20180_e37973_d_n6;
        locals.var_eu_a_dn7 = assign20180_e37973_d_n7;
        locals.var_eu_a_dn8 = assign20180_e37973_d_n8;
        locals.var_eu_a_dn9 = assign20180_e37973_d_n9;
        locals.var_eu_a_dn10 = assign20180_e37973_d_n10;
        locals.var_eu_a_dn11 = assign20180_e37973_d_n11;
        locals.var_eu_a_dn13 = assign20180_e37973_d_n13;
        locals.var_eu_a_dn14 = assign20180_e37973_d_n14;

        let assign20190_e37976: f64 = (1.0 / locals.var_mexp_a);
        locals.var_inv_mexp = assign20190_e37976;
        locals.var_inv_mexp_dn0 = (-(locals.var_mexp_a_dn0 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn2 = (-(locals.var_mexp_a_dn2 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn3 = (-(locals.var_mexp_a_dn3 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn4 = (-(locals.var_mexp_a_dn4 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn5 = (-(locals.var_mexp_a_dn5 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn6 = (-(locals.var_mexp_a_dn6 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn7 = (-(locals.var_mexp_a_dn7 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn8 = (-(locals.var_mexp_a_dn8 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn9 = (-(locals.var_mexp_a_dn9 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn10 = (-(locals.var_mexp_a_dn10 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn11 = (-(locals.var_mexp_a_dn11 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn13 = (-(locals.var_mexp_a_dn13 / (locals.var_mexp_a * locals.var_mexp_a)));
        locals.var_inv_mexp_dn14 = (-(locals.var_mexp_a_dn14 / (locals.var_mexp_a * locals.var_mexp_a)));

        let assign20200_e37979: f64 = (0.4 + locals.var_phib);
        let assign20200_e37981: f64 = (assign20200_e37979 + locals.var_phin_i);
        locals.var_phist = assign20200_e37981;
        locals.var_phist_dn0 = locals.var_phib_dn0;
        locals.var_phist_dn2 = locals.var_phib_dn2;
        locals.var_phist_dn3 = locals.var_phib_dn3;
        locals.var_phist_dn4 = locals.var_phib_dn4;
        locals.var_phist_dn5 = locals.var_phib_dn5;
        locals.var_phist_dn6 = locals.var_phib_dn6;
        locals.var_phist_dn7 = locals.var_phib_dn7;
        locals.var_phist_dn8 = locals.var_phib_dn8;
        locals.var_phist_dn9 = locals.var_phib_dn9;
        locals.var_phist_dn10 = locals.var_phib_dn10;
        locals.var_phist_dn11 = locals.var_phib_dn11;
        locals.var_phist_dn13 = locals.var_phib_dn13;
        locals.var_phist_dn14 = locals.var_phib_dn14;

        let assign20210_e37985: f64 = (locals.var_cins / locals.var_weff_ufcm);
        let assign20210_e37986: f64 = (2.0 * assign20210_e37985);
        let assign20210_e37989: f64 = (locals.var_rc + 2.0);
        let assign20210_e37990: f64 = (assign20210_e37986 / assign20210_e37989);
        locals.var_t1 = assign20210_e37990;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign20220_e37995: f64 = (locals.var_cdscd_a * locals.var_vdsx);
        let assign20220_e37996: f64 = (locals.var_cdsc_i + assign20220_e37995);
        let assign20220_e37997: f64 = (locals.var_theta_sw * assign20220_e37996);
        locals.var_cdsc_v = assign20220_e37997;
        locals.var_cdsc_v_dn0 = ((locals.var_theta_sw_dn0 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn0 * locals.var_vdsx)));
        locals.var_cdsc_v_dn2 = ((locals.var_theta_sw_dn2 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn2 * locals.var_vdsx)));
        locals.var_cdsc_v_dn3 = ((locals.var_theta_sw_dn3 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn3 * locals.var_vdsx)));
        locals.var_cdsc_v_dn4 = ((locals.var_theta_sw_dn4 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn4 * locals.var_vdsx)));
        locals.var_cdsc_v_dn5 = ((locals.var_theta_sw_dn5 * assign20220_e37996) + (locals.var_theta_sw * ((locals.var_cdscd_a_dn5 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn5))));
        locals.var_cdsc_v_dn6 = ((locals.var_theta_sw_dn6 * assign20220_e37996) + (locals.var_theta_sw * ((locals.var_cdscd_a_dn6 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn6))));
        locals.var_cdsc_v_dn7 = ((locals.var_theta_sw_dn7 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn7 * locals.var_vdsx)));
        locals.var_cdsc_v_dn8 = ((locals.var_theta_sw_dn8 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn8 * locals.var_vdsx)));
        locals.var_cdsc_v_dn9 = ((locals.var_theta_sw_dn9 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn9 * locals.var_vdsx)));
        locals.var_cdsc_v_dn10 = ((locals.var_theta_sw_dn10 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn10 * locals.var_vdsx)));
        locals.var_cdsc_v_dn11 = ((locals.var_theta_sw_dn11 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn11 * locals.var_vdsx)));
        locals.var_cdsc_v_dn13 = ((locals.var_theta_sw_dn13 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn13 * locals.var_vdsx)));
        locals.var_cdsc_v_dn14 = ((locals.var_theta_sw_dn14 * assign20220_e37996) + (locals.var_theta_sw * (locals.var_cdscd_a_dn14 * locals.var_vdsx)));

        let assign20230_e38000: f64 = if p.p175 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard363 = assign20230_e38000;

        let assign20240_e38003: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard364 = assign20240_e38003;

        let (assign20250_e38019, assign20250_e38019_d_n0, assign20250_e38019_d_n2, assign20250_e38019_d_n3, assign20250_e38019_d_n4, assign20250_e38019_d_n5, assign20250_e38019_d_n6, assign20250_e38019_d_n7, assign20250_e38019_d_n8, assign20250_e38019_d_n9, assign20250_e38019_d_n10, assign20250_e38019_d_n11, assign20250_e38019_d_n13, assign20250_e38019_d_n14,) = {
    if ((locals.var_guard363 != 0.0) && (locals.var_guard364 != 0.0)) {
        let assign20250_e38009: f64 = (locals.var_vtm * locals.var_thetass);
        let assign20250_e38013: f64 = (locals.var_cit_a + locals.var_cdsc_v);
        let assign20250_e38015: f64 = (assign20250_e38013 / locals.var_t1);
        let assign20250_e38016: f64 = (1.0 + assign20250_e38015);
        let assign20250_e38017: f64 = (assign20250_e38009 * assign20250_e38016);
        (assign20250_e38017, (assign20250_e38009 * ((((locals.var_cit_a_dn0 + locals.var_cdsc_v_dn0) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn2 + locals.var_cdsc_v_dn2) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn3 + locals.var_cdsc_v_dn3) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1))), ((((locals.var_vtm_dn4 * locals.var_thetass) + (locals.var_vtm * locals.var_thetass_dn4)) * assign20250_e38016) + (assign20250_e38009 * ((((locals.var_cit_a_dn4 + locals.var_cdsc_v_dn4) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)))), (assign20250_e38009 * ((((locals.var_cit_a_dn5 + locals.var_cdsc_v_dn5) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn6 + locals.var_cdsc_v_dn6) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn7 + locals.var_cdsc_v_dn7) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn8 + locals.var_cdsc_v_dn8) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn9 + locals.var_cdsc_v_dn9) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn10 + locals.var_cdsc_v_dn10) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn11 + locals.var_cdsc_v_dn11) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn13 + locals.var_cdsc_v_dn13) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1))), (assign20250_e38009 * ((((locals.var_cit_a_dn14 + locals.var_cdsc_v_dn14) * locals.var_t1) - (assign20250_e38013 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_nvtm, locals.var_nvtm_dn0, locals.var_nvtm_dn2, locals.var_nvtm_dn3, locals.var_nvtm_dn4, locals.var_nvtm_dn5, locals.var_nvtm_dn6, locals.var_nvtm_dn7, locals.var_nvtm_dn8, locals.var_nvtm_dn9, locals.var_nvtm_dn10, locals.var_nvtm_dn11, locals.var_nvtm_dn13, locals.var_nvtm_dn14,)
    }
};
        locals.var_nvtm = assign20250_e38019;
        locals.var_nvtm_dn0 = assign20250_e38019_d_n0;
        locals.var_nvtm_dn2 = assign20250_e38019_d_n2;
        locals.var_nvtm_dn3 = assign20250_e38019_d_n3;
        locals.var_nvtm_dn4 = assign20250_e38019_d_n4;
        locals.var_nvtm_dn5 = assign20250_e38019_d_n5;
        locals.var_nvtm_dn6 = assign20250_e38019_d_n6;
        locals.var_nvtm_dn7 = assign20250_e38019_d_n7;
        locals.var_nvtm_dn8 = assign20250_e38019_d_n8;
        locals.var_nvtm_dn9 = assign20250_e38019_d_n9;
        locals.var_nvtm_dn10 = assign20250_e38019_d_n10;
        locals.var_nvtm_dn11 = assign20250_e38019_d_n11;
        locals.var_nvtm_dn13 = assign20250_e38019_d_n13;
        locals.var_nvtm_dn14 = assign20250_e38019_d_n14;

        let (assign20260_e38036, assign20260_e38036_d_n0, assign20260_e38036_d_n2, assign20260_e38036_d_n3, assign20260_e38036_d_n4, assign20260_e38036_d_n5, assign20260_e38036_d_n6, assign20260_e38036_d_n7, assign20260_e38036_d_n8, assign20260_e38036_d_n9, assign20260_e38036_d_n10, assign20260_e38036_d_n11, assign20260_e38036_d_n13, assign20260_e38036_d_n14,) = {
    if ((locals.var_guard363 != 0.0) && (locals.var_guard364 == 0.0)) {
        let assign20260_e38026: f64 = (locals.var_vtmeff * locals.var_thetass);
        let assign20260_e38030: f64 = (locals.var_cit_a + locals.var_cdsc_v);
        let assign20260_e38032: f64 = (assign20260_e38030 / locals.var_t1);
        let assign20260_e38033: f64 = (1.0 + assign20260_e38032);
        let assign20260_e38034: f64 = (assign20260_e38026 * assign20260_e38033);
        (assign20260_e38034, (((locals.var_vtmeff_dn0 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn0 + locals.var_cdsc_v_dn0) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn2 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn2 + locals.var_cdsc_v_dn2) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn3 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn3 + locals.var_cdsc_v_dn3) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)))), ((((locals.var_vtmeff_dn4 * locals.var_thetass) + (locals.var_vtmeff * locals.var_thetass_dn4)) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn4 + locals.var_cdsc_v_dn4) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn5 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn5 + locals.var_cdsc_v_dn5) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn6 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn6 + locals.var_cdsc_v_dn6) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn7 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn7 + locals.var_cdsc_v_dn7) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn8 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn8 + locals.var_cdsc_v_dn8) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn9 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn9 + locals.var_cdsc_v_dn9) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn10 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn10 + locals.var_cdsc_v_dn10) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn11 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn11 + locals.var_cdsc_v_dn11) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn13 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn13 + locals.var_cdsc_v_dn13) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)))), (((locals.var_vtmeff_dn14 * locals.var_thetass) * assign20260_e38033) + (assign20260_e38026 * ((((locals.var_cit_a_dn14 + locals.var_cdsc_v_dn14) * locals.var_t1) - (assign20260_e38030 * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)))),)
    } else {
        (locals.var_nvtm, locals.var_nvtm_dn0, locals.var_nvtm_dn2, locals.var_nvtm_dn3, locals.var_nvtm_dn4, locals.var_nvtm_dn5, locals.var_nvtm_dn6, locals.var_nvtm_dn7, locals.var_nvtm_dn8, locals.var_nvtm_dn9, locals.var_nvtm_dn10, locals.var_nvtm_dn11, locals.var_nvtm_dn13, locals.var_nvtm_dn14,)
    }
};
        locals.var_nvtm = assign20260_e38036;
        locals.var_nvtm_dn0 = assign20260_e38036_d_n0;
        locals.var_nvtm_dn2 = assign20260_e38036_d_n2;
        locals.var_nvtm_dn3 = assign20260_e38036_d_n3;
        locals.var_nvtm_dn4 = assign20260_e38036_d_n4;
        locals.var_nvtm_dn5 = assign20260_e38036_d_n5;
        locals.var_nvtm_dn6 = assign20260_e38036_d_n6;
        locals.var_nvtm_dn7 = assign20260_e38036_d_n7;
        locals.var_nvtm_dn8 = assign20260_e38036_d_n8;
        locals.var_nvtm_dn9 = assign20260_e38036_d_n9;
        locals.var_nvtm_dn10 = assign20260_e38036_d_n10;
        locals.var_nvtm_dn11 = assign20260_e38036_d_n11;
        locals.var_nvtm_dn13 = assign20260_e38036_d_n13;
        locals.var_nvtm_dn14 = assign20260_e38036_d_n14;

        let (assign20270_e38041, assign20270_e38041_d_n0, assign20270_e38041_d_n2, assign20270_e38041_d_n3, assign20270_e38041_d_n4, assign20270_e38041_d_n5, assign20270_e38041_d_n6, assign20270_e38041_d_n7, assign20270_e38041_d_n8, assign20270_e38041_d_n9, assign20270_e38041_d_n10, assign20270_e38041_d_n11, assign20270_e38041_d_n13, assign20270_e38041_d_n14,) = {
    if (locals.var_guard363 == 0.0) {
        (p.p175, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_nvtm, locals.var_nvtm_dn0, locals.var_nvtm_dn2, locals.var_nvtm_dn3, locals.var_nvtm_dn4, locals.var_nvtm_dn5, locals.var_nvtm_dn6, locals.var_nvtm_dn7, locals.var_nvtm_dn8, locals.var_nvtm_dn9, locals.var_nvtm_dn10, locals.var_nvtm_dn11, locals.var_nvtm_dn13, locals.var_nvtm_dn14,)
    }
};
        locals.var_nvtm = assign20270_e38041;
        locals.var_nvtm_dn0 = assign20270_e38041_d_n0;
        locals.var_nvtm_dn2 = assign20270_e38041_d_n2;
        locals.var_nvtm_dn3 = assign20270_e38041_d_n3;
        locals.var_nvtm_dn4 = assign20270_e38041_d_n4;
        locals.var_nvtm_dn5 = assign20270_e38041_d_n5;
        locals.var_nvtm_dn6 = assign20270_e38041_d_n6;
        locals.var_nvtm_dn7 = assign20270_e38041_d_n7;
        locals.var_nvtm_dn8 = assign20270_e38041_d_n8;
        locals.var_nvtm_dn9 = assign20270_e38041_d_n9;
        locals.var_nvtm_dn10 = assign20270_e38041_d_n10;
        locals.var_nvtm_dn11 = assign20270_e38041_d_n11;
        locals.var_nvtm_dn13 = assign20270_e38041_d_n13;
        locals.var_nvtm_dn14 = assign20270_e38041_d_n14;

        let assign20280_e38044: f64 = (locals.var_qdep_ov_cins / locals.var_nvtm);
        locals.var_qdep = assign20280_e38044;
        locals.var_qdep_dn0 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn0) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn2 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn2) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn3 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn3) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn4 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn4) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn5 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn5) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn6 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn6) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn7 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn7) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn8 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn8) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn9 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn9) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn10 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn10) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn11 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn11) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn13 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn13) / (locals.var_nvtm * locals.var_nvtm)));
        locals.var_qdep_dn14 = (-((locals.var_qdep_ov_cins * locals.var_nvtm_dn14) / (locals.var_nvtm * locals.var_nvtm)));

        let assign20290_e38047: f64 = (locals.var_cins * locals.var_nvtm);
        let assign20290_e38050: f64 = (1.60219e-19 * locals.var_nc);
        let assign20290_e38052: f64 = (assign20290_e38050 * 2.0);
        let assign20290_e38054: f64 = (assign20290_e38052 * locals.var_ach);
        let assign20290_e38055: f64 = (assign20290_e38047 / assign20290_e38054);
        let (assign20290_e38088, assign20290_e38088_d_n0, assign20290_e38088_d_n2, assign20290_e38088_d_n3, assign20290_e38088_d_n4, assign20290_e38088_d_n5, assign20290_e38088_d_n6, assign20290_e38088_d_n7, assign20290_e38088_d_n8, assign20290_e38088_d_n9, assign20290_e38088_d_n10, assign20290_e38088_d_n11, assign20290_e38088_d_n13, assign20290_e38088_d_n14,) = {
    if (!(assign20290_e38055 > 1e-38)) {
        let assign20290_e38060: f64 = (-87.498233534);
        (assign20290_e38060, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign20290_e38063: f64 = (locals.var_cins * locals.var_nvtm);
        let assign20290_e38066: f64 = (1.60219e-19 * locals.var_nc);
        let assign20290_e38068: f64 = (assign20290_e38066 * 2.0);
        let assign20290_e38070: f64 = (assign20290_e38068 * locals.var_ach);
        let assign20290_e38071: f64 = (assign20290_e38063 / assign20290_e38070);
        let (assign20290_e38087, assign20290_e38087_d_n0, assign20290_e38087_d_n2, assign20290_e38087_d_n3, assign20290_e38087_d_n4, assign20290_e38087_d_n5, assign20290_e38087_d_n6, assign20290_e38087_d_n7, assign20290_e38087_d_n8, assign20290_e38087_d_n9, assign20290_e38087_d_n10, assign20290_e38087_d_n11, assign20290_e38087_d_n13, assign20290_e38087_d_n14,) = {
            if (assign20290_e38071 > 1e-38) {
                let assign20290_e38076: f64 = (locals.var_cins * locals.var_nvtm);
                let assign20290_e38079: f64 = (1.60219e-19 * locals.var_nc);
                let assign20290_e38081: f64 = (assign20290_e38079 * 2.0);
                let assign20290_e38083: f64 = (assign20290_e38081 * locals.var_ach);
                let assign20290_e38084: f64 = (assign20290_e38076 / assign20290_e38083);
                let assign20290_e38085: f64 = (assign20290_e38084).ln();
                (assign20290_e38085, (((((locals.var_cins * locals.var_nvtm_dn0) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn0) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn2) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn2) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn3) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn3) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn4) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn4) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn5) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn5) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn6) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn6) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn7) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn7) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn8) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn8) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn9) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn9) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn10) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn10) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn11) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn11) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn13) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn13) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084), (((((locals.var_cins * locals.var_nvtm_dn14) * assign20290_e38083) - (assign20290_e38076 * (((1.60219e-19 * locals.var_nc_dn14) * 2.0) * locals.var_ach))) / (assign20290_e38083 * assign20290_e38083)) / assign20290_e38084),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign20290_e38087, assign20290_e38087_d_n0, assign20290_e38087_d_n2, assign20290_e38087_d_n3, assign20290_e38087_d_n4, assign20290_e38087_d_n5, assign20290_e38087_d_n6, assign20290_e38087_d_n7, assign20290_e38087_d_n8, assign20290_e38087_d_n9, assign20290_e38087_d_n10, assign20290_e38087_d_n11, assign20290_e38087_d_n13, assign20290_e38087_d_n14,)
    }
};
        locals.var_vth_fixed_factor_si = assign20290_e38088;
        locals.var_vth_fixed_factor_si_dn0 = assign20290_e38088_d_n0;
        locals.var_vth_fixed_factor_si_dn2 = assign20290_e38088_d_n2;
        locals.var_vth_fixed_factor_si_dn3 = assign20290_e38088_d_n3;
        locals.var_vth_fixed_factor_si_dn4 = assign20290_e38088_d_n4;
        locals.var_vth_fixed_factor_si_dn5 = assign20290_e38088_d_n5;
        locals.var_vth_fixed_factor_si_dn6 = assign20290_e38088_d_n6;
        locals.var_vth_fixed_factor_si_dn7 = assign20290_e38088_d_n7;
        locals.var_vth_fixed_factor_si_dn8 = assign20290_e38088_d_n8;
        locals.var_vth_fixed_factor_si_dn9 = assign20290_e38088_d_n9;
        locals.var_vth_fixed_factor_si_dn10 = assign20290_e38088_d_n10;
        locals.var_vth_fixed_factor_si_dn11 = assign20290_e38088_d_n11;
        locals.var_vth_fixed_factor_si_dn13 = assign20290_e38088_d_n13;
        locals.var_vth_fixed_factor_si_dn14 = assign20290_e38088_d_n14;

        let assign20300_e38091: f64 = (locals.var_qdep * locals.var_rc);
        let assign20300_e38094: f64 = (locals.var_qdep * locals.var_rc);
        let assign20300_e38095: f64 = (assign20300_e38091 * assign20300_e38094);
        let assign20300_e38098: f64 = (locals.var_qdep * locals.var_rc);
        let assign20300_e38099: f64 = { let limited_exp_arg = assign20300_e38098; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign20300_e38102: f64 = (locals.var_qdep * locals.var_rc);
        let assign20300_e38103: f64 = (assign20300_e38099 - assign20300_e38102);
        let assign20300_e38105: f64 = (assign20300_e38103 - 1.0);
        let assign20300_e38106: f64 = (assign20300_e38095 / assign20300_e38105);
        let (assign20300_e38153, assign20300_e38153_d_n0, assign20300_e38153_d_n2, assign20300_e38153_d_n3, assign20300_e38153_d_n4, assign20300_e38153_d_n5, assign20300_e38153_d_n6, assign20300_e38153_d_n7, assign20300_e38153_d_n8, assign20300_e38153_d_n9, assign20300_e38153_d_n10, assign20300_e38153_d_n11, assign20300_e38153_d_n13, assign20300_e38153_d_n14,) = {
    if (!(assign20300_e38106 > 1e-38)) {
        let assign20300_e38111: f64 = (-87.498233534);
        (assign20300_e38111, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign20300_e38114: f64 = (locals.var_qdep * locals.var_rc);
        let assign20300_e38117: f64 = (locals.var_qdep * locals.var_rc);
        let assign20300_e38118: f64 = (assign20300_e38114 * assign20300_e38117);
        let assign20300_e38121: f64 = (locals.var_qdep * locals.var_rc);
        let assign20300_e38122: f64 = { let limited_exp_arg = assign20300_e38121; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign20300_e38125: f64 = (locals.var_qdep * locals.var_rc);
        let assign20300_e38126: f64 = (assign20300_e38122 - assign20300_e38125);
        let assign20300_e38128: f64 = (assign20300_e38126 - 1.0);
        let assign20300_e38129: f64 = (assign20300_e38118 / assign20300_e38128);
        let (assign20300_e38152, assign20300_e38152_d_n0, assign20300_e38152_d_n2, assign20300_e38152_d_n3, assign20300_e38152_d_n4, assign20300_e38152_d_n5, assign20300_e38152_d_n6, assign20300_e38152_d_n7, assign20300_e38152_d_n8, assign20300_e38152_d_n9, assign20300_e38152_d_n10, assign20300_e38152_d_n11, assign20300_e38152_d_n13, assign20300_e38152_d_n14,) = {
            if (assign20300_e38129 > 1e-38) {
                let assign20300_e38134: f64 = (locals.var_qdep * locals.var_rc);
                let assign20300_e38137: f64 = (locals.var_qdep * locals.var_rc);
                let assign20300_e38138: f64 = (assign20300_e38134 * assign20300_e38137);
                let assign20300_e38141: f64 = (locals.var_qdep * locals.var_rc);
                let assign20300_e38142: f64 = { let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
                let assign20300_e38145: f64 = (locals.var_qdep * locals.var_rc);
                let assign20300_e38146: f64 = (assign20300_e38142 - assign20300_e38145);
                let assign20300_e38148: f64 = (assign20300_e38146 - 1.0);
                let assign20300_e38149: f64 = (assign20300_e38138 / assign20300_e38148);
                let assign20300_e38150: f64 = (assign20300_e38149).ln();
                (assign20300_e38150, (((((((locals.var_qdep_dn0 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn0 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn0 * locals.var_rc)) - (locals.var_qdep_dn0 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn2 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn2 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn2 * locals.var_rc)) - (locals.var_qdep_dn2 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn3 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn3 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn3 * locals.var_rc)) - (locals.var_qdep_dn3 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn4 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn4 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn4 * locals.var_rc)) - (locals.var_qdep_dn4 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn5 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn5 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn5 * locals.var_rc)) - (locals.var_qdep_dn5 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn6 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn6 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn6 * locals.var_rc)) - (locals.var_qdep_dn6 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn7 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn7 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn7 * locals.var_rc)) - (locals.var_qdep_dn7 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn8 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn8 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn8 * locals.var_rc)) - (locals.var_qdep_dn8 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn9 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn9 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn9 * locals.var_rc)) - (locals.var_qdep_dn9 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn10 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn10 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn10 * locals.var_rc)) - (locals.var_qdep_dn10 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn11 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn11 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn11 * locals.var_rc)) - (locals.var_qdep_dn11 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn13 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn13 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn13 * locals.var_rc)) - (locals.var_qdep_dn13 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149), (((((((locals.var_qdep_dn14 * locals.var_rc) * assign20300_e38137) + (assign20300_e38134 * (locals.var_qdep_dn14 * locals.var_rc))) * assign20300_e38148) - (assign20300_e38138 * (({ let limited_exp_arg = assign20300_e38141; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qdep_dn14 * locals.var_rc)) - (locals.var_qdep_dn14 * locals.var_rc)))) / (assign20300_e38148 * assign20300_e38148)) / assign20300_e38149),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign20300_e38152, assign20300_e38152_d_n0, assign20300_e38152_d_n2, assign20300_e38152_d_n3, assign20300_e38152_d_n4, assign20300_e38152_d_n5, assign20300_e38152_d_n6, assign20300_e38152_d_n7, assign20300_e38152_d_n8, assign20300_e38152_d_n9, assign20300_e38152_d_n10, assign20300_e38152_d_n11, assign20300_e38152_d_n13, assign20300_e38152_d_n14,)
    }
};
        let assign20300_e38155: f64 = (assign20300_e38153 + locals.var_vth_fixed_factor_si);
        locals.var_vth_fixed_factor_sub = assign20300_e38155;
        locals.var_vth_fixed_factor_sub_dn0 = (assign20300_e38153_d_n0 + locals.var_vth_fixed_factor_si_dn0);
        locals.var_vth_fixed_factor_sub_dn2 = (assign20300_e38153_d_n2 + locals.var_vth_fixed_factor_si_dn2);
        locals.var_vth_fixed_factor_sub_dn3 = (assign20300_e38153_d_n3 + locals.var_vth_fixed_factor_si_dn3);
        locals.var_vth_fixed_factor_sub_dn4 = (assign20300_e38153_d_n4 + locals.var_vth_fixed_factor_si_dn4);
        locals.var_vth_fixed_factor_sub_dn5 = (assign20300_e38153_d_n5 + locals.var_vth_fixed_factor_si_dn5);
        locals.var_vth_fixed_factor_sub_dn6 = (assign20300_e38153_d_n6 + locals.var_vth_fixed_factor_si_dn6);
        locals.var_vth_fixed_factor_sub_dn7 = (assign20300_e38153_d_n7 + locals.var_vth_fixed_factor_si_dn7);
        locals.var_vth_fixed_factor_sub_dn8 = (assign20300_e38153_d_n8 + locals.var_vth_fixed_factor_si_dn8);
        locals.var_vth_fixed_factor_sub_dn9 = (assign20300_e38153_d_n9 + locals.var_vth_fixed_factor_si_dn9);
        locals.var_vth_fixed_factor_sub_dn10 = (assign20300_e38153_d_n10 + locals.var_vth_fixed_factor_si_dn10);
        locals.var_vth_fixed_factor_sub_dn11 = (assign20300_e38153_d_n11 + locals.var_vth_fixed_factor_si_dn11);
        locals.var_vth_fixed_factor_sub_dn13 = (assign20300_e38153_d_n13 + locals.var_vth_fixed_factor_si_dn13);
        locals.var_vth_fixed_factor_sub_dn14 = (assign20300_e38153_d_n14 + locals.var_vth_fixed_factor_si_dn14);

        let assign20310_e38158: f64 = (10.0 * locals.var_nvtm);
        let assign20310_e38160: f64 = (assign20310_e38158 / locals.var_rc);
        let assign20310_e38163: f64 = (2.0 * locals.var_qbs);
        let assign20310_e38164: f64 = (assign20310_e38160 + assign20310_e38163);
        locals.var_q0 = assign20310_e38164;
        locals.var_q0_dn0 = ((10.0 * locals.var_nvtm_dn0) / locals.var_rc);
        locals.var_q0_dn2 = ((10.0 * locals.var_nvtm_dn2) / locals.var_rc);
        locals.var_q0_dn3 = ((10.0 * locals.var_nvtm_dn3) / locals.var_rc);
        locals.var_q0_dn4 = ((10.0 * locals.var_nvtm_dn4) / locals.var_rc);
        locals.var_q0_dn5 = ((10.0 * locals.var_nvtm_dn5) / locals.var_rc);
        locals.var_q0_dn6 = ((10.0 * locals.var_nvtm_dn6) / locals.var_rc);
        locals.var_q0_dn7 = ((10.0 * locals.var_nvtm_dn7) / locals.var_rc);
        locals.var_q0_dn8 = ((10.0 * locals.var_nvtm_dn8) / locals.var_rc);
        locals.var_q0_dn9 = ((10.0 * locals.var_nvtm_dn9) / locals.var_rc);
        locals.var_q0_dn10 = ((10.0 * locals.var_nvtm_dn10) / locals.var_rc);
        locals.var_q0_dn11 = ((10.0 * locals.var_nvtm_dn11) / locals.var_rc);
        locals.var_q0_dn13 = ((10.0 * locals.var_nvtm_dn13) / locals.var_rc);
        locals.var_q0_dn14 = ((10.0 * locals.var_nvtm_dn14) / locals.var_rc);

        let assign20320_e38167: f64 = (locals.var_vtm * locals.var_cins);
        let assign20320_e38170: f64 = (locals.var_weff_ufcm * locals.var_epssub);
        let assign20320_e38171: f64 = (assign20320_e38167 / assign20320_e38170);
        locals.var_fieldnormalizationfactor = assign20320_e38171;
        locals.var_fieldnormalizationfactor_dn4 = ((locals.var_vtm_dn4 * locals.var_cins) / assign20320_e38170);

        let assign20330_e38174: f64 = (4.5 * 1.05457e-34);
        let assign20330_e38176: f64 = (assign20330_e38174 * 3.141592653589793);
        let assign20330_e38178: f64 = (assign20330_e38176 * 1.60219e-19);
        let assign20330_e38182: f64 = (2.0 * locals.var_mx);
        let assign20330_e38183: f64 = (assign20330_e38182).sqrt();
        let assign20330_e38184: f64 = (4.0 * assign20330_e38183);
        let assign20330_e38185: f64 = (assign20330_e38178 / assign20330_e38184);
        let assign20330_e38187: f64 = (assign20330_e38185).powf(0.666666667);
        locals.var_auxqmfact = assign20330_e38187;

        let assign20340_e38190: f64 = (p.p1804 * locals.var_auxqmfact);
        let assign20340_e38193: f64 = (locals.var_fieldnormalizationfactor).powf(0.666666667);
        let assign20340_e38194: f64 = (assign20340_e38190 * assign20340_e38193);
        let assign20340_e38197: f64 = (1.60219e-19 * locals.var_vtm);
        let assign20340_e38198: f64 = (assign20340_e38194 / assign20340_e38197);
        locals.var_qmfactorcvfinal = assign20340_e38198;
        locals.var_qmfactorcvfinal_dn4 = ((((assign20340_e38190 * if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((locals.var_fieldnormalizationfactor).powf(0.666666667 - 1.0) * locals.var_fieldnormalizationfactor_dn4)) } } else { (assign20340_e38193 * (0.666666667 * (locals.var_fieldnormalizationfactor_dn4 / locals.var_fieldnormalizationfactor))) }) * assign20340_e38197) - (assign20340_e38194 * (1.60219e-19 * locals.var_vtm_dn4))) / (assign20340_e38197 * assign20340_e38197));

        let assign20350_e38200: f64 = (-locals.var_dvt0_i);
        let assign20350_e38202: f64 = (assign20350_e38200 * locals.var_theta_sce);
        let assign20350_e38205: f64 = (locals.var_vbi - locals.var_phist);
        let assign20350_e38206: f64 = (assign20350_e38202 * assign20350_e38205);
        locals.var_dvth_vtroll = assign20350_e38206;
        locals.var_dvth_vtroll_dn0 = (((assign20350_e38200 * locals.var_theta_sce_dn0) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn0 - locals.var_phist_dn0)));
        locals.var_dvth_vtroll_dn2 = (((assign20350_e38200 * locals.var_theta_sce_dn2) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn2 - locals.var_phist_dn2)));
        locals.var_dvth_vtroll_dn3 = (((assign20350_e38200 * locals.var_theta_sce_dn3) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn3 - locals.var_phist_dn3)));
        locals.var_dvth_vtroll_dn4 = (((assign20350_e38200 * locals.var_theta_sce_dn4) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn4 - locals.var_phist_dn4)));
        locals.var_dvth_vtroll_dn5 = (((assign20350_e38200 * locals.var_theta_sce_dn5) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn5 - locals.var_phist_dn5)));
        locals.var_dvth_vtroll_dn6 = (((assign20350_e38200 * locals.var_theta_sce_dn6) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn6 - locals.var_phist_dn6)));
        locals.var_dvth_vtroll_dn7 = (((assign20350_e38200 * locals.var_theta_sce_dn7) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn7 - locals.var_phist_dn7)));
        locals.var_dvth_vtroll_dn8 = (((assign20350_e38200 * locals.var_theta_sce_dn8) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn8 - locals.var_phist_dn8)));
        locals.var_dvth_vtroll_dn9 = (((assign20350_e38200 * locals.var_theta_sce_dn9) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn9 - locals.var_phist_dn9)));
        locals.var_dvth_vtroll_dn10 = (((assign20350_e38200 * locals.var_theta_sce_dn10) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn10 - locals.var_phist_dn10)));
        locals.var_dvth_vtroll_dn11 = (((assign20350_e38200 * locals.var_theta_sce_dn11) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn11 - locals.var_phist_dn11)));
        locals.var_dvth_vtroll_dn13 = (((assign20350_e38200 * locals.var_theta_sce_dn13) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn13 - locals.var_phist_dn13)));
        locals.var_dvth_vtroll_dn14 = (((assign20350_e38200 * locals.var_theta_sce_dn14) * assign20350_e38205) + (assign20350_e38202 * (locals.var_vbi_dn14 - locals.var_phist_dn14)));

        let assign20360_e38208: f64 = (-locals.var_eta0_a);
        let assign20360_e38210: f64 = (assign20360_e38208 * locals.var_theta_dibl);
        let assign20360_e38215: f64 = (locals.var_vdsx + 0.01);
        let assign20360_e38216: f64 = (assign20360_e38215).sqrt();
        let assign20360_e38217: f64 = (locals.var_eta1_i * assign20360_e38216);
        let assign20360_e38218: f64 = (locals.var_vdsx + assign20360_e38217);
        let assign20360_e38219: f64 = (assign20360_e38210 * assign20360_e38218);
        let assign20360_e38222: f64 = (locals.var_dvtp0_i * locals.var_theta_dits);
        let assign20360_e38225: f64 = (locals.var_vdsx + 0.01);
        let assign20360_e38227: f64 = (assign20360_e38225).powf(locals.var_dvtp1_i);
        let assign20360_e38228: f64 = (assign20360_e38222 * assign20360_e38227);
        let assign20360_e38229: f64 = (assign20360_e38219 + assign20360_e38228);
        locals.var_dvth_dibl = assign20360_e38229;
        locals.var_dvth_dibl_dn0 = (((((-locals.var_eta0_a_dn0) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn0)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn0 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn0)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn0 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn0 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn2 = (((((-locals.var_eta0_a_dn2) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn2)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn2 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn2)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn2 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn2 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn3 = (((((-locals.var_eta0_a_dn3) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn3)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn3 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn3)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn3 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn3 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn4 = (((((-locals.var_eta0_a_dn4) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn4)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn4 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn4)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn4 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn4 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn5 = ((((((-locals.var_eta0_a_dn5) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn5)) * assign20360_e38218) + (assign20360_e38210 * (locals.var_vdsx_dn5 + (locals.var_eta1_i * (locals.var_vdsx_dn5 / (2.0 * assign20360_e38216)))))) + ((((locals.var_dvtp0_i_dn5 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn5)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn5 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { if locals.var_dvtp1_i == 0.0 { 0.0 } else { (locals.var_dvtp1_i * ((assign20360_e38225).powf(locals.var_dvtp1_i - 1.0) * locals.var_vdsx_dn5)) } } else { (assign20360_e38227 * ((locals.var_dvtp1_i_dn5 * (assign20360_e38225).ln()) + (locals.var_dvtp1_i * (locals.var_vdsx_dn5 / assign20360_e38225)))) })));
        locals.var_dvth_dibl_dn6 = ((((((-locals.var_eta0_a_dn6) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn6)) * assign20360_e38218) + (assign20360_e38210 * (locals.var_vdsx_dn6 + (locals.var_eta1_i * (locals.var_vdsx_dn6 / (2.0 * assign20360_e38216)))))) + ((((locals.var_dvtp0_i_dn6 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn6)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn6 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { if locals.var_dvtp1_i == 0.0 { 0.0 } else { (locals.var_dvtp1_i * ((assign20360_e38225).powf(locals.var_dvtp1_i - 1.0) * locals.var_vdsx_dn6)) } } else { (assign20360_e38227 * ((locals.var_dvtp1_i_dn6 * (assign20360_e38225).ln()) + (locals.var_dvtp1_i * (locals.var_vdsx_dn6 / assign20360_e38225)))) })));
        locals.var_dvth_dibl_dn7 = (((((-locals.var_eta0_a_dn7) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn7)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn7 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn7)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn7 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn7 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn8 = (((((-locals.var_eta0_a_dn8) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn8)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn8 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn8)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn8 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn8 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn9 = (((((-locals.var_eta0_a_dn9) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn9)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn9 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn9)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn9 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn9 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn10 = (((((-locals.var_eta0_a_dn10) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn10)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn10 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn10)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn10 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn10 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn11 = (((((-locals.var_eta0_a_dn11) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn11)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn11 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn11)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn11 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn11 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn13 = (((((-locals.var_eta0_a_dn13) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn13)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn13 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn13)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn13 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn13 * (assign20360_e38225).ln())) })));
        locals.var_dvth_dibl_dn14 = (((((-locals.var_eta0_a_dn14) * locals.var_theta_dibl) + (assign20360_e38208 * locals.var_theta_dibl_dn14)) * assign20360_e38218) + ((((locals.var_dvtp0_i_dn14 * locals.var_theta_dits) + (locals.var_dvtp0_i * locals.var_theta_dits_dn14)) * assign20360_e38227) + (assign20360_e38222 * if locals.var_dvtp1_i_dn14 == 0.0 && ((locals.var_dvtp1_i) as f64).is_finite() && ((locals.var_dvtp1_i) as f64).fract() == 0.0 { 0.0 } else { (assign20360_e38227 * (locals.var_dvtp1_i_dn14 * (assign20360_e38225).ln())) })));

        let assign20370_e38232: f64 = (locals.var_k1rsce_i * locals.var_theta_rsce);
        let assign20370_e38234: f64 = (locals.var_phist).sqrt();
        let assign20370_e38235: f64 = (assign20370_e38232 * assign20370_e38234);
        locals.var_dvth_rsce = assign20370_e38235;
        locals.var_dvth_rsce_dn0 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn0) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn0 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn2 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn2) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn2 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn3 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn3) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn3 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn4 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn4) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn4 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn5 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn5) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn5 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn6 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn6) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn6 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn7 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn7) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn7 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn8 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn8) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn8 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn9 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn9) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn9 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn10 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn10) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn10 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn11 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn11) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn11 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn13 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn13) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn13 / (2.0 * assign20370_e38234))));
        locals.var_dvth_rsce_dn14 = (((locals.var_k1rsce_i * locals.var_theta_rsce_dn14) * assign20370_e38234) + (assign20370_e38232 * (locals.var_phist_dn14 / (2.0 * assign20370_e38234))));

    }

    pub(super) fn stamp_transient_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign20380_e38238: f64 = (locals.var_dvth_vtroll + locals.var_dvth_dibl);
        let assign20380_e38240: f64 = (assign20380_e38238 + locals.var_dvth_rsce);
        let assign20380_e38242: f64 = (assign20380_e38240 + locals.var_dvth_temp);
        let assign20380_e38244: f64 = (assign20380_e38242 + locals.var_dvtshift_a);
        locals.var_dvth_all = assign20380_e38244;
        locals.var_dvth_all_dn0 = ((((locals.var_dvth_vtroll_dn0 + locals.var_dvth_dibl_dn0) + locals.var_dvth_rsce_dn0) + locals.var_dvth_temp_dn0) + locals.var_dvtshift_a_dn0);
        locals.var_dvth_all_dn2 = ((((locals.var_dvth_vtroll_dn2 + locals.var_dvth_dibl_dn2) + locals.var_dvth_rsce_dn2) + locals.var_dvth_temp_dn2) + locals.var_dvtshift_a_dn2);
        locals.var_dvth_all_dn3 = ((((locals.var_dvth_vtroll_dn3 + locals.var_dvth_dibl_dn3) + locals.var_dvth_rsce_dn3) + locals.var_dvth_temp_dn3) + locals.var_dvtshift_a_dn3);
        locals.var_dvth_all_dn4 = ((((locals.var_dvth_vtroll_dn4 + locals.var_dvth_dibl_dn4) + locals.var_dvth_rsce_dn4) + locals.var_dvth_temp_dn4) + locals.var_dvtshift_a_dn4);
        locals.var_dvth_all_dn5 = ((((locals.var_dvth_vtroll_dn5 + locals.var_dvth_dibl_dn5) + locals.var_dvth_rsce_dn5) + locals.var_dvth_temp_dn5) + locals.var_dvtshift_a_dn5);
        locals.var_dvth_all_dn6 = ((((locals.var_dvth_vtroll_dn6 + locals.var_dvth_dibl_dn6) + locals.var_dvth_rsce_dn6) + locals.var_dvth_temp_dn6) + locals.var_dvtshift_a_dn6);
        locals.var_dvth_all_dn7 = ((((locals.var_dvth_vtroll_dn7 + locals.var_dvth_dibl_dn7) + locals.var_dvth_rsce_dn7) + locals.var_dvth_temp_dn7) + locals.var_dvtshift_a_dn7);
        locals.var_dvth_all_dn8 = ((((locals.var_dvth_vtroll_dn8 + locals.var_dvth_dibl_dn8) + locals.var_dvth_rsce_dn8) + locals.var_dvth_temp_dn8) + locals.var_dvtshift_a_dn8);
        locals.var_dvth_all_dn9 = ((((locals.var_dvth_vtroll_dn9 + locals.var_dvth_dibl_dn9) + locals.var_dvth_rsce_dn9) + locals.var_dvth_temp_dn9) + locals.var_dvtshift_a_dn9);
        locals.var_dvth_all_dn10 = ((((locals.var_dvth_vtroll_dn10 + locals.var_dvth_dibl_dn10) + locals.var_dvth_rsce_dn10) + locals.var_dvth_temp_dn10) + locals.var_dvtshift_a_dn10);
        locals.var_dvth_all_dn11 = ((((locals.var_dvth_vtroll_dn11 + locals.var_dvth_dibl_dn11) + locals.var_dvth_rsce_dn11) + locals.var_dvth_temp_dn11) + locals.var_dvtshift_a_dn11);
        locals.var_dvth_all_dn13 = ((((locals.var_dvth_vtroll_dn13 + locals.var_dvth_dibl_dn13) + locals.var_dvth_rsce_dn13) + locals.var_dvth_temp_dn13) + locals.var_dvtshift_a_dn13);
        locals.var_dvth_all_dn14 = ((((locals.var_dvth_vtroll_dn14 + locals.var_dvth_dibl_dn14) + locals.var_dvth_rsce_dn14) + locals.var_dvth_temp_dn14) + locals.var_dvtshift_a_dn14);

        let assign20390_e38247: f64 = (locals.var_vgsfb - locals.var_dvth_all);
        locals.var_vgsfb = assign20390_e38247;
        locals.var_vgsfb_dn0 = (locals.var_vgsfb_dn0 - locals.var_dvth_all_dn0);
        locals.var_vgsfb_dn2 = (locals.var_vgsfb_dn2 - locals.var_dvth_all_dn2);
        locals.var_vgsfb_dn3 = (locals.var_vgsfb_dn3 - locals.var_dvth_all_dn3);
        locals.var_vgsfb_dn4 = (locals.var_vgsfb_dn4 - locals.var_dvth_all_dn4);
        locals.var_vgsfb_dn5 = (locals.var_vgsfb_dn5 - locals.var_dvth_all_dn5);
        locals.var_vgsfb_dn6 = (locals.var_vgsfb_dn6 - locals.var_dvth_all_dn6);
        locals.var_vgsfb_dn7 = (locals.var_vgsfb_dn7 - locals.var_dvth_all_dn7);
        locals.var_vgsfb_dn8 = (locals.var_vgsfb_dn8 - locals.var_dvth_all_dn8);
        locals.var_vgsfb_dn9 = (locals.var_vgsfb_dn9 - locals.var_dvth_all_dn9);
        locals.var_vgsfb_dn10 = (locals.var_vgsfb_dn10 - locals.var_dvth_all_dn10);
        locals.var_vgsfb_dn11 = (locals.var_vgsfb_dn11 - locals.var_dvth_all_dn11);
        locals.var_vgsfb_dn13 = (locals.var_vgsfb_dn13 - locals.var_dvth_all_dn13);
        locals.var_vgsfb_dn14 = (locals.var_vgsfb_dn14 - locals.var_dvth_all_dn14);

        let assign20400_e38250: f64 = (locals.var_u0_a * locals.var_cox);
        let assign20400_e38252: f64 = (assign20400_e38250 * locals.var_weff0);
        let assign20400_e38254: f64 = (assign20400_e38252 / locals.var_leff_1);
        locals.var_beta0_v = assign20400_e38254;
        locals.var_beta0_v_dn0 = (((((locals.var_u0_a_dn0 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn0)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn2 = (((((locals.var_u0_a_dn2 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn2)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn3 = (((((locals.var_u0_a_dn3 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn3)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn4 = (((((locals.var_u0_a_dn4 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn4)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn5 = (((((locals.var_u0_a_dn5 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn5)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn6 = (((((locals.var_u0_a_dn6 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn6)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn7 = (((((locals.var_u0_a_dn7 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn7)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn8 = (((((locals.var_u0_a_dn8 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn8)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn9 = (((((locals.var_u0_a_dn9 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn9)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn10 = (((((locals.var_u0_a_dn10 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn10)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn11 = (((((locals.var_u0_a_dn11 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn11)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn13 = (((((locals.var_u0_a_dn13 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn13)) / (locals.var_leff_1 * locals.var_leff_1));
        locals.var_beta0_v_dn14 = (((((locals.var_u0_a_dn14 * locals.var_cox) * locals.var_weff0) * locals.var_leff_1) - (assign20400_e38252 * locals.var_leff_1_dn14)) / (locals.var_leff_1 * locals.var_leff_1));

        let assign20410_e38257: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard365 = assign20410_e38257;

        let (assign20420_e38277, assign20420_e38277_d_n0, assign20420_e38277_d_n2, assign20420_e38277_d_n3, assign20420_e38277_d_n4, assign20420_e38277_d_n5, assign20420_e38277_d_n6, assign20420_e38277_d_n7, assign20420_e38277_d_n8, assign20420_e38277_d_n9, assign20420_e38277_d_n10, assign20420_e38277_d_n11, assign20420_e38277_d_n13, assign20420_e38277_d_n14,) = {
    if (locals.var_guard365 != 0.0) {
        let assign20420_e38261: f64 = (2.0 * locals.var_cox);
        let assign20420_e38263: f64 = (assign20420_e38261 * p.p108);
        let assign20420_e38266: f64 = (locals.var_beta0_v * locals.var_nvtm);
        let assign20420_e38268: f64 = (assign20420_e38266 * 1.60219e-19);
        let assign20420_e38270: f64 = (assign20420_e38268 * locals.var_nc);
        let assign20420_e38272: f64 = (assign20420_e38270 * p.p3);
        let assign20420_e38273: f64 = (assign20420_e38263 / assign20420_e38272);
        let assign20420_e38275: f64 = (assign20420_e38273).powf(locals.var_nvtm);
        (assign20420_e38275, if locals.var_nvtm_dn0 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn0 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn0)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn0)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn0 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn0 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn0)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn0)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn2 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn2 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn2)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn2)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn2 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn2 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn2)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn2)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn3 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn3 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn3)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn3)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn3 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn3 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn3)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn3)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn4 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn4 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn4)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn4)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn4 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn4 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn4)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn4)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn5 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn5 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn5)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn5)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn5 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn5 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn5)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn5)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn6 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn6 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn6)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn6)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn6 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn6 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn6)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn6)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn7 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn7 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn7)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn7)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn7 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn7 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn7)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn7)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn8 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn8 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn8)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn8)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn8 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn8 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn8)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn8)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn9 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn9 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn9)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn9)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn9 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn9 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn9)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn9)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn10 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn10 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn10)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn10)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn10 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn10 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn10)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn10)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn11 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn11 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn11)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn11)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn11 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn11 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn11)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn11)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn13 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn13 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn13)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn13)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn13 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn13 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn13)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn13)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) }, if locals.var_nvtm_dn14 == 0.0 && ((locals.var_nvtm) as f64).is_finite() && ((locals.var_nvtm) as f64).fract() == 0.0 { if locals.var_nvtm == 0.0 { 0.0 } else { (locals.var_nvtm * ((assign20420_e38273).powf(locals.var_nvtm - 1.0) * (-((assign20420_e38263 * ((((((locals.var_beta0_v_dn14 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn14)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn14)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))))) } } else { (assign20420_e38275 * ((locals.var_nvtm_dn14 * (assign20420_e38273).ln()) + (locals.var_nvtm * ((-((assign20420_e38263 * ((((((locals.var_beta0_v_dn14 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn14)) * 1.60219e-19) * locals.var_nc) + (assign20420_e38268 * locals.var_nc_dn14)) * p.p3)) / (assign20420_e38272 * assign20420_e38272))) / assign20420_e38273)))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20420_e38277;
        locals.var_t3_dn0 = assign20420_e38277_d_n0;
        locals.var_t3_dn2 = assign20420_e38277_d_n2;
        locals.var_t3_dn3 = assign20420_e38277_d_n3;
        locals.var_t3_dn4 = assign20420_e38277_d_n4;
        locals.var_t3_dn5 = assign20420_e38277_d_n5;
        locals.var_t3_dn6 = assign20420_e38277_d_n6;
        locals.var_t3_dn7 = assign20420_e38277_d_n7;
        locals.var_t3_dn8 = assign20420_e38277_d_n8;
        locals.var_t3_dn9 = assign20420_e38277_d_n9;
        locals.var_t3_dn10 = assign20420_e38277_d_n10;
        locals.var_t3_dn11 = assign20420_e38277_d_n11;
        locals.var_t3_dn13 = assign20420_e38277_d_n13;
        locals.var_t3_dn14 = assign20420_e38277_d_n14;

        let (assign20430_e38297, assign20430_e38297_d_n0, assign20430_e38297_d_n2, assign20430_e38297_d_n3, assign20430_e38297_d_n4, assign20430_e38297_d_n5, assign20430_e38297_d_n6, assign20430_e38297_d_n7, assign20430_e38297_d_n8, assign20430_e38297_d_n9, assign20430_e38297_d_n10, assign20430_e38297_d_n11, assign20430_e38297_d_n13, assign20430_e38297_d_n14,) = {
    if (locals.var_guard365 != 0.0) {
        let (assign20430_e38293, assign20430_e38293_d_n0, assign20430_e38293_d_n2, assign20430_e38293_d_n3, assign20430_e38293_d_n4, assign20430_e38293_d_n5, assign20430_e38293_d_n6, assign20430_e38293_d_n7, assign20430_e38293_d_n8, assign20430_e38293_d_n9, assign20430_e38293_d_n10, assign20430_e38293_d_n11, assign20430_e38293_d_n13, assign20430_e38293_d_n14,) = {
            if (!(locals.var_t3 > 1e-38)) {
                let assign20430_e38285: f64 = (-87.498233534);
                (assign20430_e38285, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (assign20430_e38292, assign20430_e38292_d_n0, assign20430_e38292_d_n2, assign20430_e38292_d_n3, assign20430_e38292_d_n4, assign20430_e38292_d_n5, assign20430_e38292_d_n6, assign20430_e38292_d_n7, assign20430_e38292_d_n8, assign20430_e38292_d_n9, assign20430_e38292_d_n10, assign20430_e38292_d_n11, assign20430_e38292_d_n13, assign20430_e38292_d_n14,) = {
                    if (locals.var_t3 > 1e-38) {
                        let assign20430_e38290: f64 = (locals.var_t3).ln();
                        (assign20430_e38290, (locals.var_t3_dn0 / locals.var_t3), (locals.var_t3_dn2 / locals.var_t3), (locals.var_t3_dn3 / locals.var_t3), (locals.var_t3_dn4 / locals.var_t3), (locals.var_t3_dn5 / locals.var_t3), (locals.var_t3_dn6 / locals.var_t3), (locals.var_t3_dn7 / locals.var_t3), (locals.var_t3_dn8 / locals.var_t3), (locals.var_t3_dn9 / locals.var_t3), (locals.var_t3_dn10 / locals.var_t3), (locals.var_t3_dn11 / locals.var_t3), (locals.var_t3_dn13 / locals.var_t3), (locals.var_t3_dn14 / locals.var_t3),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20430_e38292, assign20430_e38292_d_n0, assign20430_e38292_d_n2, assign20430_e38292_d_n3, assign20430_e38292_d_n4, assign20430_e38292_d_n5, assign20430_e38292_d_n6, assign20430_e38292_d_n7, assign20430_e38292_d_n8, assign20430_e38292_d_n9, assign20430_e38292_d_n10, assign20430_e38292_d_n11, assign20430_e38292_d_n13, assign20430_e38292_d_n14,)
            }
        };
        let assign20430_e38294: f64 = (locals.var_dvch_qm + assign20430_e38293);
        let assign20430_e38295: f64 = (-assign20430_e38294);
        (assign20430_e38295, (-(locals.var_dvch_qm_dn0 + assign20430_e38293_d_n0)), (-(locals.var_dvch_qm_dn2 + assign20430_e38293_d_n2)), (-(locals.var_dvch_qm_dn3 + assign20430_e38293_d_n3)), (-(locals.var_dvch_qm_dn4 + assign20430_e38293_d_n4)), (-(locals.var_dvch_qm_dn5 + assign20430_e38293_d_n5)), (-(locals.var_dvch_qm_dn6 + assign20430_e38293_d_n6)), (-(locals.var_dvch_qm_dn7 + assign20430_e38293_d_n7)), (-(locals.var_dvch_qm_dn8 + assign20430_e38293_d_n8)), (-(locals.var_dvch_qm_dn9 + assign20430_e38293_d_n9)), (-(locals.var_dvch_qm_dn10 + assign20430_e38293_d_n10)), (-(locals.var_dvch_qm_dn11 + assign20430_e38293_d_n11)), (-(locals.var_dvch_qm_dn13 + assign20430_e38293_d_n13)), (-(locals.var_dvch_qm_dn14 + assign20430_e38293_d_n14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20430_e38297;
        locals.var_t0_dn0 = assign20430_e38297_d_n0;
        locals.var_t0_dn2 = assign20430_e38297_d_n2;
        locals.var_t0_dn3 = assign20430_e38297_d_n3;
        locals.var_t0_dn4 = assign20430_e38297_d_n4;
        locals.var_t0_dn5 = assign20430_e38297_d_n5;
        locals.var_t0_dn6 = assign20430_e38297_d_n6;
        locals.var_t0_dn7 = assign20430_e38297_d_n7;
        locals.var_t0_dn8 = assign20430_e38297_d_n8;
        locals.var_t0_dn9 = assign20430_e38297_d_n9;
        locals.var_t0_dn10 = assign20430_e38297_d_n10;
        locals.var_t0_dn11 = assign20430_e38297_d_n11;
        locals.var_t0_dn13 = assign20430_e38297_d_n13;
        locals.var_t0_dn14 = assign20430_e38297_d_n14;

        let (assign20440_e38305, assign20440_e38305_d_n0, assign20440_e38305_d_n2, assign20440_e38305_d_n3, assign20440_e38305_d_n4, assign20440_e38305_d_n5, assign20440_e38305_d_n6, assign20440_e38305_d_n7, assign20440_e38305_d_n8, assign20440_e38305_d_n9, assign20440_e38305_d_n10, assign20440_e38305_d_n11, assign20440_e38305_d_n13, assign20440_e38305_d_n14,) = {
    if (locals.var_guard365 != 0.0) {
        let assign20440_e38301: f64 = (locals.var_vgsfb + locals.var_t0);
        let assign20440_e38303: f64 = (assign20440_e38301 + p.p23);
        (assign20440_e38303, (locals.var_vgsfb_dn0 + locals.var_t0_dn0), (locals.var_vgsfb_dn2 + locals.var_t0_dn2), (locals.var_vgsfb_dn3 + locals.var_t0_dn3), (locals.var_vgsfb_dn4 + locals.var_t0_dn4), (locals.var_vgsfb_dn5 + locals.var_t0_dn5), (locals.var_vgsfb_dn6 + locals.var_t0_dn6), (locals.var_vgsfb_dn7 + locals.var_t0_dn7), (locals.var_vgsfb_dn8 + locals.var_t0_dn8), (locals.var_vgsfb_dn9 + locals.var_t0_dn9), (locals.var_vgsfb_dn10 + locals.var_t0_dn10), (locals.var_vgsfb_dn11 + locals.var_t0_dn11), (locals.var_vgsfb_dn13 + locals.var_t0_dn13), (locals.var_vgsfb_dn14 + locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20440_e38305;
        locals.var_t1_dn0 = assign20440_e38305_d_n0;
        locals.var_t1_dn2 = assign20440_e38305_d_n2;
        locals.var_t1_dn3 = assign20440_e38305_d_n3;
        locals.var_t1_dn4 = assign20440_e38305_d_n4;
        locals.var_t1_dn5 = assign20440_e38305_d_n5;
        locals.var_t1_dn6 = assign20440_e38305_d_n6;
        locals.var_t1_dn7 = assign20440_e38305_d_n7;
        locals.var_t1_dn8 = assign20440_e38305_d_n8;
        locals.var_t1_dn9 = assign20440_e38305_d_n9;
        locals.var_t1_dn10 = assign20440_e38305_d_n10;
        locals.var_t1_dn11 = assign20440_e38305_d_n11;
        locals.var_t1_dn13 = assign20440_e38305_d_n13;
        locals.var_t1_dn14 = assign20440_e38305_d_n14;

        let (assign20450_e38346, assign20450_e38346_d_n0, assign20450_e38346_d_n2, assign20450_e38346_d_n3, assign20450_e38346_d_n4, assign20450_e38346_d_n5, assign20450_e38346_d_n6, assign20450_e38346_d_n7, assign20450_e38346_d_n8, assign20450_e38346_d_n9, assign20450_e38346_d_n10, assign20450_e38346_d_n11, assign20450_e38346_d_n13, assign20450_e38346_d_n14,) = {
    if (locals.var_guard365 != 0.0) {
        let assign20450_e38309: f64 = (-10000.0);
        let assign20450_e38311: f64 = (assign20450_e38309 * 0.0001);
        let (assign20450_e38342, assign20450_e38342_d_n0, assign20450_e38342_d_n2, assign20450_e38342_d_n3, assign20450_e38342_d_n4, assign20450_e38342_d_n5, assign20450_e38342_d_n6, assign20450_e38342_d_n7, assign20450_e38342_d_n8, assign20450_e38342_d_n9, assign20450_e38342_d_n10, assign20450_e38342_d_n11, assign20450_e38342_d_n13, assign20450_e38342_d_n14,) = {
            if (!(locals.var_t1 < assign20450_e38311)) {
                let assign20450_e38318: f64 = (locals.var_t1 * locals.var_t1);
                let assign20450_e38321: f64 = (4.0 * 0.0001);
                let assign20450_e38323: f64 = (assign20450_e38321 * 0.0001);
                let assign20450_e38324: f64 = (assign20450_e38318 + assign20450_e38323);
                let assign20450_e38325: f64 = (assign20450_e38324).sqrt();
                let assign20450_e38326: f64 = (locals.var_t1 + assign20450_e38325);
                let assign20450_e38327: f64 = (0.5 * assign20450_e38326);
                (assign20450_e38327, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign20450_e38325)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign20450_e38325)))),)
            } else {
                let assign20450_e38330: f64 = (-10000.0);
                let assign20450_e38332: f64 = (assign20450_e38330 * 0.0001);
                let (assign20450_e38341, assign20450_e38341_d_n0, assign20450_e38341_d_n2, assign20450_e38341_d_n3, assign20450_e38341_d_n4, assign20450_e38341_d_n5, assign20450_e38341_d_n6, assign20450_e38341_d_n7, assign20450_e38341_d_n8, assign20450_e38341_d_n9, assign20450_e38341_d_n10, assign20450_e38341_d_n11, assign20450_e38341_d_n13, assign20450_e38341_d_n14,) = {
                    if (locals.var_t1 < assign20450_e38332) {
                        let assign20450_e38335: f64 = (-0.0001);
                        let assign20450_e38337: f64 = (assign20450_e38335 * 0.0001);
                        let assign20450_e38339: f64 = (assign20450_e38337 / locals.var_t1);
                        (assign20450_e38339, (-((assign20450_e38337 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((assign20450_e38337 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20450_e38341, assign20450_e38341_d_n0, assign20450_e38341_d_n2, assign20450_e38341_d_n3, assign20450_e38341_d_n4, assign20450_e38341_d_n5, assign20450_e38341_d_n6, assign20450_e38341_d_n7, assign20450_e38341_d_n8, assign20450_e38341_d_n9, assign20450_e38341_d_n10, assign20450_e38341_d_n11, assign20450_e38341_d_n13, assign20450_e38341_d_n14,)
            }
        };
        let assign20450_e38344: f64 = (assign20450_e38342 - locals.var_t0);
        (assign20450_e38344, (assign20450_e38342_d_n0 - locals.var_t0_dn0), (assign20450_e38342_d_n2 - locals.var_t0_dn2), (assign20450_e38342_d_n3 - locals.var_t0_dn3), (assign20450_e38342_d_n4 - locals.var_t0_dn4), (assign20450_e38342_d_n5 - locals.var_t0_dn5), (assign20450_e38342_d_n6 - locals.var_t0_dn6), (assign20450_e38342_d_n7 - locals.var_t0_dn7), (assign20450_e38342_d_n8 - locals.var_t0_dn8), (assign20450_e38342_d_n9 - locals.var_t0_dn9), (assign20450_e38342_d_n10 - locals.var_t0_dn10), (assign20450_e38342_d_n11 - locals.var_t0_dn11), (assign20450_e38342_d_n13 - locals.var_t0_dn13), (assign20450_e38342_d_n14 - locals.var_t0_dn14),)
    } else {
        (locals.var_vgsfbeff, locals.var_vgsfbeff_dn0, locals.var_vgsfbeff_dn2, locals.var_vgsfbeff_dn3, locals.var_vgsfbeff_dn4, locals.var_vgsfbeff_dn5, locals.var_vgsfbeff_dn6, locals.var_vgsfbeff_dn7, locals.var_vgsfbeff_dn8, locals.var_vgsfbeff_dn9, locals.var_vgsfbeff_dn10, locals.var_vgsfbeff_dn11, locals.var_vgsfbeff_dn13, locals.var_vgsfbeff_dn14,)
    }
};
        locals.var_vgsfbeff = assign20450_e38346;
        locals.var_vgsfbeff_dn0 = assign20450_e38346_d_n0;
        locals.var_vgsfbeff_dn2 = assign20450_e38346_d_n2;
        locals.var_vgsfbeff_dn3 = assign20450_e38346_d_n3;
        locals.var_vgsfbeff_dn4 = assign20450_e38346_d_n4;
        locals.var_vgsfbeff_dn5 = assign20450_e38346_d_n5;
        locals.var_vgsfbeff_dn6 = assign20450_e38346_d_n6;
        locals.var_vgsfbeff_dn7 = assign20450_e38346_d_n7;
        locals.var_vgsfbeff_dn8 = assign20450_e38346_d_n8;
        locals.var_vgsfbeff_dn9 = assign20450_e38346_d_n9;
        locals.var_vgsfbeff_dn10 = assign20450_e38346_d_n10;
        locals.var_vgsfbeff_dn11 = assign20450_e38346_d_n11;
        locals.var_vgsfbeff_dn13 = assign20450_e38346_d_n13;
        locals.var_vgsfbeff_dn14 = assign20450_e38346_d_n14;

        let (assign20460_e38409, assign20460_e38409_d_n0, assign20460_e38409_d_n2, assign20460_e38409_d_n3, assign20460_e38409_d_n4, assign20460_e38409_d_n5, assign20460_e38409_d_n6, assign20460_e38409_d_n7, assign20460_e38409_d_n8, assign20460_e38409_d_n9, assign20460_e38409_d_n10, assign20460_e38409_d_n11, assign20460_e38409_d_n13, assign20460_e38409_d_n14,) = {
    if (locals.var_guard365 == 0.0) {
        let assign20460_e38350: f64 = (-locals.var_nvtm);
        let assign20460_e38353: f64 = (2.0 * locals.var_cox);
        let assign20460_e38355: f64 = (assign20460_e38353 * p.p108);
        let assign20460_e38358: f64 = (locals.var_beta0_v * locals.var_nvtm);
        let assign20460_e38360: f64 = (assign20460_e38358 * 1.60219e-19);
        let assign20460_e38362: f64 = (assign20460_e38360 * locals.var_nc);
        let assign20460_e38364: f64 = (assign20460_e38362 * p.p3);
        let assign20460_e38365: f64 = (assign20460_e38355 / assign20460_e38364);
        let (assign20460_e38406, assign20460_e38406_d_n0, assign20460_e38406_d_n2, assign20460_e38406_d_n3, assign20460_e38406_d_n4, assign20460_e38406_d_n5, assign20460_e38406_d_n6, assign20460_e38406_d_n7, assign20460_e38406_d_n8, assign20460_e38406_d_n9, assign20460_e38406_d_n10, assign20460_e38406_d_n11, assign20460_e38406_d_n13, assign20460_e38406_d_n14,) = {
            if (!(assign20460_e38365 > 1e-38)) {
                let assign20460_e38370: f64 = (-87.498233534);
                (assign20460_e38370, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign20460_e38373: f64 = (2.0 * locals.var_cox);
                let assign20460_e38375: f64 = (assign20460_e38373 * p.p108);
                let assign20460_e38378: f64 = (locals.var_beta0_v * locals.var_nvtm);
                let assign20460_e38380: f64 = (assign20460_e38378 * 1.60219e-19);
                let assign20460_e38382: f64 = (assign20460_e38380 * locals.var_nc);
                let assign20460_e38384: f64 = (assign20460_e38382 * p.p3);
                let assign20460_e38385: f64 = (assign20460_e38375 / assign20460_e38384);
                let (assign20460_e38405, assign20460_e38405_d_n0, assign20460_e38405_d_n2, assign20460_e38405_d_n3, assign20460_e38405_d_n4, assign20460_e38405_d_n5, assign20460_e38405_d_n6, assign20460_e38405_d_n7, assign20460_e38405_d_n8, assign20460_e38405_d_n9, assign20460_e38405_d_n10, assign20460_e38405_d_n11, assign20460_e38405_d_n13, assign20460_e38405_d_n14,) = {
                    if (assign20460_e38385 > 1e-38) {
                        let assign20460_e38390: f64 = (2.0 * locals.var_cox);
                        let assign20460_e38392: f64 = (assign20460_e38390 * p.p108);
                        let assign20460_e38395: f64 = (locals.var_beta0_v * locals.var_nvtm);
                        let assign20460_e38397: f64 = (assign20460_e38395 * 1.60219e-19);
                        let assign20460_e38399: f64 = (assign20460_e38397 * locals.var_nc);
                        let assign20460_e38401: f64 = (assign20460_e38399 * p.p3);
                        let assign20460_e38402: f64 = (assign20460_e38392 / assign20460_e38401);
                        let assign20460_e38403: f64 = (assign20460_e38402).ln();
                        (assign20460_e38403, ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn0 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn0)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn0)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn2 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn2)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn2)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn3 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn3)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn3)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn4 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn4)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn4)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn5 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn5)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn5)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn6 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn6)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn6)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn7 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn7)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn7)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn8 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn8)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn8)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn9 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn9)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn9)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn10 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn10)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn10)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn11 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn11)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn11)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn13 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn13)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn13)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402), ((-((assign20460_e38392 * ((((((locals.var_beta0_v_dn14 * locals.var_nvtm) + (locals.var_beta0_v * locals.var_nvtm_dn14)) * 1.60219e-19) * locals.var_nc) + (assign20460_e38397 * locals.var_nc_dn14)) * p.p3)) / (assign20460_e38401 * assign20460_e38401))) / assign20460_e38402),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20460_e38405, assign20460_e38405_d_n0, assign20460_e38405_d_n2, assign20460_e38405_d_n3, assign20460_e38405_d_n4, assign20460_e38405_d_n5, assign20460_e38405_d_n6, assign20460_e38405_d_n7, assign20460_e38405_d_n8, assign20460_e38405_d_n9, assign20460_e38405_d_n10, assign20460_e38405_d_n11, assign20460_e38405_d_n13, assign20460_e38405_d_n14,)
            }
        };
        let assign20460_e38407: f64 = (assign20460_e38350 * assign20460_e38406);
        (assign20460_e38407, (((-locals.var_nvtm_dn0) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n0)), (((-locals.var_nvtm_dn2) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n2)), (((-locals.var_nvtm_dn3) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n3)), (((-locals.var_nvtm_dn4) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n4)), (((-locals.var_nvtm_dn5) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n5)), (((-locals.var_nvtm_dn6) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n6)), (((-locals.var_nvtm_dn7) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n7)), (((-locals.var_nvtm_dn8) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n8)), (((-locals.var_nvtm_dn9) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n9)), (((-locals.var_nvtm_dn10) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n10)), (((-locals.var_nvtm_dn11) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n11)), (((-locals.var_nvtm_dn13) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n13)), (((-locals.var_nvtm_dn14) * assign20460_e38406) + (assign20460_e38350 * assign20460_e38406_d_n14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20460_e38409;
        locals.var_t0_dn0 = assign20460_e38409_d_n0;
        locals.var_t0_dn2 = assign20460_e38409_d_n2;
        locals.var_t0_dn3 = assign20460_e38409_d_n3;
        locals.var_t0_dn4 = assign20460_e38409_d_n4;
        locals.var_t0_dn5 = assign20460_e38409_d_n5;
        locals.var_t0_dn6 = assign20460_e38409_d_n6;
        locals.var_t0_dn7 = assign20460_e38409_d_n7;
        locals.var_t0_dn8 = assign20460_e38409_d_n8;
        locals.var_t0_dn9 = assign20460_e38409_d_n9;
        locals.var_t0_dn10 = assign20460_e38409_d_n10;
        locals.var_t0_dn11 = assign20460_e38409_d_n11;
        locals.var_t0_dn13 = assign20460_e38409_d_n13;
        locals.var_t0_dn14 = assign20460_e38409_d_n14;

        let (assign20470_e38436, assign20470_e38436_d_n0, assign20470_e38436_d_n2, assign20470_e38436_d_n3, assign20470_e38436_d_n4, assign20470_e38436_d_n5, assign20470_e38436_d_n6, assign20470_e38436_d_n7, assign20470_e38436_d_n8, assign20470_e38436_d_n9, assign20470_e38436_d_n10, assign20470_e38436_d_n11, assign20470_e38436_d_n13, assign20470_e38436_d_n14,) = {
    if (locals.var_guard365 == 0.0) {
        let assign20470_e38413: f64 = (-locals.var_dvch_qm);
        let assign20470_e38417: f64 = (locals.var_t0 + 0.01);
        let assign20470_e38420: f64 = (locals.var_t0 - 0.01);
        let assign20470_e38423: f64 = (locals.var_t0 - 0.01);
        let assign20470_e38424: f64 = (assign20470_e38420 * assign20470_e38423);
        let assign20470_e38427: f64 = (0.25 * 0.0001);
        let assign20470_e38429: f64 = (assign20470_e38427 * 0.0001);
        let assign20470_e38430: f64 = (assign20470_e38424 + assign20470_e38429);
        let assign20470_e38431: f64 = (assign20470_e38430).sqrt();
        let assign20470_e38432: f64 = (assign20470_e38417 + assign20470_e38431);
        let assign20470_e38433: f64 = (0.5 * assign20470_e38432);
        let assign20470_e38434: f64 = (assign20470_e38413 + assign20470_e38433);
        (assign20470_e38434, ((-locals.var_dvch_qm_dn0) + (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn0)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn2) + (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn2)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn3) + (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn3)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn4) + (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn4)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn5) + (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn5)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn6) + (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn6)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn7) + (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn7)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn8) + (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn8)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn9) + (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn9)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn10) + (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn10)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn11) + (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn11)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn13) + (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn13)) / (2.0 * assign20470_e38431))))), ((-locals.var_dvch_qm_dn14) + (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign20470_e38423) + (assign20470_e38420 * locals.var_t0_dn14)) / (2.0 * assign20470_e38431))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20470_e38436;
        locals.var_t1_dn0 = assign20470_e38436_d_n0;
        locals.var_t1_dn2 = assign20470_e38436_d_n2;
        locals.var_t1_dn3 = assign20470_e38436_d_n3;
        locals.var_t1_dn4 = assign20470_e38436_d_n4;
        locals.var_t1_dn5 = assign20470_e38436_d_n5;
        locals.var_t1_dn6 = assign20470_e38436_d_n6;
        locals.var_t1_dn7 = assign20470_e38436_d_n7;
        locals.var_t1_dn8 = assign20470_e38436_d_n8;
        locals.var_t1_dn9 = assign20470_e38436_d_n9;
        locals.var_t1_dn10 = assign20470_e38436_d_n10;
        locals.var_t1_dn11 = assign20470_e38436_d_n11;
        locals.var_t1_dn13 = assign20470_e38436_d_n13;
        locals.var_t1_dn14 = assign20470_e38436_d_n14;

        let (assign20480_e38445, assign20480_e38445_d_n0, assign20480_e38445_d_n2, assign20480_e38445_d_n3, assign20480_e38445_d_n4, assign20480_e38445_d_n5, assign20480_e38445_d_n6, assign20480_e38445_d_n7, assign20480_e38445_d_n8, assign20480_e38445_d_n9, assign20480_e38445_d_n10, assign20480_e38445_d_n11, assign20480_e38445_d_n13, assign20480_e38445_d_n14,) = {
    if (locals.var_guard365 == 0.0) {
        let assign20480_e38441: f64 = (locals.var_vgsfb + locals.var_t1);
        let assign20480_e38443: f64 = (assign20480_e38441 + p.p23);
        (assign20480_e38443, (locals.var_vgsfb_dn0 + locals.var_t1_dn0), (locals.var_vgsfb_dn2 + locals.var_t1_dn2), (locals.var_vgsfb_dn3 + locals.var_t1_dn3), (locals.var_vgsfb_dn4 + locals.var_t1_dn4), (locals.var_vgsfb_dn5 + locals.var_t1_dn5), (locals.var_vgsfb_dn6 + locals.var_t1_dn6), (locals.var_vgsfb_dn7 + locals.var_t1_dn7), (locals.var_vgsfb_dn8 + locals.var_t1_dn8), (locals.var_vgsfb_dn9 + locals.var_t1_dn9), (locals.var_vgsfb_dn10 + locals.var_t1_dn10), (locals.var_vgsfb_dn11 + locals.var_t1_dn11), (locals.var_vgsfb_dn13 + locals.var_t1_dn13), (locals.var_vgsfb_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20480_e38445;
        locals.var_t2_dn0 = assign20480_e38445_d_n0;
        locals.var_t2_dn2 = assign20480_e38445_d_n2;
        locals.var_t2_dn3 = assign20480_e38445_d_n3;
        locals.var_t2_dn4 = assign20480_e38445_d_n4;
        locals.var_t2_dn5 = assign20480_e38445_d_n5;
        locals.var_t2_dn6 = assign20480_e38445_d_n6;
        locals.var_t2_dn7 = assign20480_e38445_d_n7;
        locals.var_t2_dn8 = assign20480_e38445_d_n8;
        locals.var_t2_dn9 = assign20480_e38445_d_n9;
        locals.var_t2_dn10 = assign20480_e38445_d_n10;
        locals.var_t2_dn11 = assign20480_e38445_d_n11;
        locals.var_t2_dn13 = assign20480_e38445_d_n13;
        locals.var_t2_dn14 = assign20480_e38445_d_n14;

        let (assign20490_e38487, assign20490_e38487_d_n0, assign20490_e38487_d_n2, assign20490_e38487_d_n3, assign20490_e38487_d_n4, assign20490_e38487_d_n5, assign20490_e38487_d_n6, assign20490_e38487_d_n7, assign20490_e38487_d_n8, assign20490_e38487_d_n9, assign20490_e38487_d_n10, assign20490_e38487_d_n11, assign20490_e38487_d_n13, assign20490_e38487_d_n14,) = {
    if (locals.var_guard365 == 0.0) {
        let assign20490_e38450: f64 = (-10000.0);
        let assign20490_e38452: f64 = (assign20490_e38450 * 0.0001);
        let (assign20490_e38483, assign20490_e38483_d_n0, assign20490_e38483_d_n2, assign20490_e38483_d_n3, assign20490_e38483_d_n4, assign20490_e38483_d_n5, assign20490_e38483_d_n6, assign20490_e38483_d_n7, assign20490_e38483_d_n8, assign20490_e38483_d_n9, assign20490_e38483_d_n10, assign20490_e38483_d_n11, assign20490_e38483_d_n13, assign20490_e38483_d_n14,) = {
            if (!(locals.var_t2 < assign20490_e38452)) {
                let assign20490_e38459: f64 = (locals.var_t2 * locals.var_t2);
                let assign20490_e38462: f64 = (4.0 * 0.0001);
                let assign20490_e38464: f64 = (assign20490_e38462 * 0.0001);
                let assign20490_e38465: f64 = (assign20490_e38459 + assign20490_e38464);
                let assign20490_e38466: f64 = (assign20490_e38465).sqrt();
                let assign20490_e38467: f64 = (locals.var_t2 + assign20490_e38466);
                let assign20490_e38468: f64 = (0.5 * assign20490_e38467);
                (assign20490_e38468, (0.5 * (locals.var_t2_dn0 + (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn2 + (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn13 + (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign20490_e38466)))), (0.5 * (locals.var_t2_dn14 + (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign20490_e38466)))),)
            } else {
                let assign20490_e38471: f64 = (-10000.0);
                let assign20490_e38473: f64 = (assign20490_e38471 * 0.0001);
                let (assign20490_e38482, assign20490_e38482_d_n0, assign20490_e38482_d_n2, assign20490_e38482_d_n3, assign20490_e38482_d_n4, assign20490_e38482_d_n5, assign20490_e38482_d_n6, assign20490_e38482_d_n7, assign20490_e38482_d_n8, assign20490_e38482_d_n9, assign20490_e38482_d_n10, assign20490_e38482_d_n11, assign20490_e38482_d_n13, assign20490_e38482_d_n14,) = {
                    if (locals.var_t2 < assign20490_e38473) {
                        let assign20490_e38476: f64 = (-0.0001);
                        let assign20490_e38478: f64 = (assign20490_e38476 * 0.0001);
                        let assign20490_e38480: f64 = (assign20490_e38478 / locals.var_t2);
                        (assign20490_e38480, (-((assign20490_e38478 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn3) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn13) / (locals.var_t2 * locals.var_t2))), (-((assign20490_e38478 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20490_e38482, assign20490_e38482_d_n0, assign20490_e38482_d_n2, assign20490_e38482_d_n3, assign20490_e38482_d_n4, assign20490_e38482_d_n5, assign20490_e38482_d_n6, assign20490_e38482_d_n7, assign20490_e38482_d_n8, assign20490_e38482_d_n9, assign20490_e38482_d_n10, assign20490_e38482_d_n11, assign20490_e38482_d_n13, assign20490_e38482_d_n14,)
            }
        };
        let assign20490_e38485: f64 = (assign20490_e38483 - locals.var_t1);
        (assign20490_e38485, (assign20490_e38483_d_n0 - locals.var_t1_dn0), (assign20490_e38483_d_n2 - locals.var_t1_dn2), (assign20490_e38483_d_n3 - locals.var_t1_dn3), (assign20490_e38483_d_n4 - locals.var_t1_dn4), (assign20490_e38483_d_n5 - locals.var_t1_dn5), (assign20490_e38483_d_n6 - locals.var_t1_dn6), (assign20490_e38483_d_n7 - locals.var_t1_dn7), (assign20490_e38483_d_n8 - locals.var_t1_dn8), (assign20490_e38483_d_n9 - locals.var_t1_dn9), (assign20490_e38483_d_n10 - locals.var_t1_dn10), (assign20490_e38483_d_n11 - locals.var_t1_dn11), (assign20490_e38483_d_n13 - locals.var_t1_dn13), (assign20490_e38483_d_n14 - locals.var_t1_dn14),)
    } else {
        (locals.var_vgsfbeff, locals.var_vgsfbeff_dn0, locals.var_vgsfbeff_dn2, locals.var_vgsfbeff_dn3, locals.var_vgsfbeff_dn4, locals.var_vgsfbeff_dn5, locals.var_vgsfbeff_dn6, locals.var_vgsfbeff_dn7, locals.var_vgsfbeff_dn8, locals.var_vgsfbeff_dn9, locals.var_vgsfbeff_dn10, locals.var_vgsfbeff_dn11, locals.var_vgsfbeff_dn13, locals.var_vgsfbeff_dn14,)
    }
};
        locals.var_vgsfbeff = assign20490_e38487;
        locals.var_vgsfbeff_dn0 = assign20490_e38487_d_n0;
        locals.var_vgsfbeff_dn2 = assign20490_e38487_d_n2;
        locals.var_vgsfbeff_dn3 = assign20490_e38487_d_n3;
        locals.var_vgsfbeff_dn4 = assign20490_e38487_d_n4;
        locals.var_vgsfbeff_dn5 = assign20490_e38487_d_n5;
        locals.var_vgsfbeff_dn6 = assign20490_e38487_d_n6;
        locals.var_vgsfbeff_dn7 = assign20490_e38487_d_n7;
        locals.var_vgsfbeff_dn8 = assign20490_e38487_d_n8;
        locals.var_vgsfbeff_dn9 = assign20490_e38487_d_n9;
        locals.var_vgsfbeff_dn10 = assign20490_e38487_d_n10;
        locals.var_vgsfbeff_dn11 = assign20490_e38487_d_n11;
        locals.var_vgsfbeff_dn13 = assign20490_e38487_d_n13;
        locals.var_vgsfbeff_dn14 = assign20490_e38487_d_n14;

        locals.var_vch = locals.var_dvch_qm;
        locals.var_vch_dn0 = locals.var_dvch_qm_dn0;
        locals.var_vch_dn2 = locals.var_dvch_qm_dn2;
        locals.var_vch_dn3 = locals.var_dvch_qm_dn3;
        locals.var_vch_dn4 = locals.var_dvch_qm_dn4;
        locals.var_vch_dn5 = locals.var_dvch_qm_dn5;
        locals.var_vch_dn6 = locals.var_dvch_qm_dn6;
        locals.var_vch_dn7 = locals.var_dvch_qm_dn7;
        locals.var_vch_dn8 = locals.var_dvch_qm_dn8;
        locals.var_vch_dn9 = locals.var_dvch_qm_dn9;
        locals.var_vch_dn10 = locals.var_dvch_qm_dn10;
        locals.var_vch_dn11 = locals.var_dvch_qm_dn11;
        locals.var_vch_dn13 = locals.var_dvch_qm_dn13;
        locals.var_vch_dn14 = locals.var_dvch_qm_dn14;

        let assign20510_e38490: f64 = (-locals.var_qdep);
        let assign20510_e38492: f64 = (assign20510_e38490).powf(0.666666667);
        locals.var_t4 = assign20510_e38492;
        locals.var_t4_dn0 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn0))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn0) / assign20510_e38490))) };
        locals.var_t4_dn2 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn2))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn2) / assign20510_e38490))) };
        locals.var_t4_dn3 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn3))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn3) / assign20510_e38490))) };
        locals.var_t4_dn4 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn4))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn4) / assign20510_e38490))) };
        locals.var_t4_dn5 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn5))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn5) / assign20510_e38490))) };
        locals.var_t4_dn6 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn6))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn6) / assign20510_e38490))) };
        locals.var_t4_dn7 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn7))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn7) / assign20510_e38490))) };
        locals.var_t4_dn8 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn8))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn8) / assign20510_e38490))) };
        locals.var_t4_dn9 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn9))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn9) / assign20510_e38490))) };
        locals.var_t4_dn10 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn10))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn10) / assign20510_e38490))) };
        locals.var_t4_dn11 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn11))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn11) / assign20510_e38490))) };
        locals.var_t4_dn13 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn13))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn13) / assign20510_e38490))) };
        locals.var_t4_dn14 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign20510_e38490).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn14))) } } else { (assign20510_e38492 * (0.666666667 * ((-locals.var_qdep_dn14) / assign20510_e38490))) };

        let assign20520_e38495: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard366 = assign20520_e38495;

        let (assign20530_e38570, assign20530_e38570_d_n0, assign20530_e38570_d_n2, assign20530_e38570_d_n3, assign20530_e38570_d_n4, assign20530_e38570_d_n5, assign20530_e38570_d_n6, assign20530_e38570_d_n7, assign20530_e38570_d_n8, assign20530_e38570_d_n9, assign20530_e38570_d_n10, assign20530_e38570_d_n11, assign20530_e38570_d_n13, assign20530_e38570_d_n14,) = {
    if (locals.var_guard366 != 0.0) {
        let assign20530_e38499: f64 = (2.0 * locals.var_phib);
        let assign20530_e38501: f64 = (assign20530_e38499 + locals.var_vch);
        let assign20530_e38503: f64 = (assign20530_e38501 - locals.var_ves);
        let assign20530_e38505: f64 = (-10000.0);
        let assign20530_e38507: f64 = (assign20530_e38505 * 0.1);
        let (assign20530_e38568, assign20530_e38568_d_n0, assign20530_e38568_d_n2, assign20530_e38568_d_n3, assign20530_e38568_d_n4, assign20530_e38568_d_n5, assign20530_e38568_d_n6, assign20530_e38568_d_n7, assign20530_e38568_d_n8, assign20530_e38568_d_n9, assign20530_e38568_d_n10, assign20530_e38568_d_n11, assign20530_e38568_d_n13, assign20530_e38568_d_n14,) = {
            if (!(assign20530_e38503 < assign20530_e38507)) {
                let assign20530_e38513: f64 = (2.0 * locals.var_phib);
                let assign20530_e38515: f64 = (assign20530_e38513 + locals.var_vch);
                let assign20530_e38517: f64 = (assign20530_e38515 - locals.var_ves);
                let assign20530_e38520: f64 = (2.0 * locals.var_phib);
                let assign20530_e38522: f64 = (assign20530_e38520 + locals.var_vch);
                let assign20530_e38524: f64 = (assign20530_e38522 - locals.var_ves);
                let assign20530_e38527: f64 = (2.0 * locals.var_phib);
                let assign20530_e38529: f64 = (assign20530_e38527 + locals.var_vch);
                let assign20530_e38531: f64 = (assign20530_e38529 - locals.var_ves);
                let assign20530_e38532: f64 = (assign20530_e38524 * assign20530_e38531);
                let assign20530_e38535: f64 = (4.0 * 0.1);
                let assign20530_e38537: f64 = (assign20530_e38535 * 0.1);
                let assign20530_e38538: f64 = (assign20530_e38532 + assign20530_e38537);
                let assign20530_e38539: f64 = (assign20530_e38538).sqrt();
                let assign20530_e38540: f64 = (assign20530_e38517 + assign20530_e38539);
                let assign20530_e38541: f64 = (0.5 * assign20530_e38540);
                (assign20530_e38541, (0.5 * (((2.0 * locals.var_phib_dn0) + locals.var_vch_dn0) + (((((2.0 * locals.var_phib_dn0) + locals.var_vch_dn0) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn0) + locals.var_vch_dn0))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn2) + locals.var_vch_dn2) + (((((2.0 * locals.var_phib_dn2) + locals.var_vch_dn2) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn2) + locals.var_vch_dn2))) / (2.0 * assign20530_e38539)))), (0.5 * ((((2.0 * locals.var_phib_dn3) + locals.var_vch_dn3) - locals.var_ves_dn3) + ((((((2.0 * locals.var_phib_dn3) + locals.var_vch_dn3) - locals.var_ves_dn3) * assign20530_e38531) + (assign20530_e38524 * (((2.0 * locals.var_phib_dn3) + locals.var_vch_dn3) - locals.var_ves_dn3))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn4) + locals.var_vch_dn4) + (((((2.0 * locals.var_phib_dn4) + locals.var_vch_dn4) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn4) + locals.var_vch_dn4))) / (2.0 * assign20530_e38539)))), (0.5 * ((((2.0 * locals.var_phib_dn5) + locals.var_vch_dn5) - locals.var_ves_dn5) + ((((((2.0 * locals.var_phib_dn5) + locals.var_vch_dn5) - locals.var_ves_dn5) * assign20530_e38531) + (assign20530_e38524 * (((2.0 * locals.var_phib_dn5) + locals.var_vch_dn5) - locals.var_ves_dn5))) / (2.0 * assign20530_e38539)))), (0.5 * ((((2.0 * locals.var_phib_dn6) + locals.var_vch_dn6) - locals.var_ves_dn6) + ((((((2.0 * locals.var_phib_dn6) + locals.var_vch_dn6) - locals.var_ves_dn6) * assign20530_e38531) + (assign20530_e38524 * (((2.0 * locals.var_phib_dn6) + locals.var_vch_dn6) - locals.var_ves_dn6))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn7) + locals.var_vch_dn7) + (((((2.0 * locals.var_phib_dn7) + locals.var_vch_dn7) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn7) + locals.var_vch_dn7))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn8) + locals.var_vch_dn8) + (((((2.0 * locals.var_phib_dn8) + locals.var_vch_dn8) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn8) + locals.var_vch_dn8))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn9) + locals.var_vch_dn9) + (((((2.0 * locals.var_phib_dn9) + locals.var_vch_dn9) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn9) + locals.var_vch_dn9))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn10) + locals.var_vch_dn10) + (((((2.0 * locals.var_phib_dn10) + locals.var_vch_dn10) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn10) + locals.var_vch_dn10))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn11) + locals.var_vch_dn11) + (((((2.0 * locals.var_phib_dn11) + locals.var_vch_dn11) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn11) + locals.var_vch_dn11))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn13) + locals.var_vch_dn13) + (((((2.0 * locals.var_phib_dn13) + locals.var_vch_dn13) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn13) + locals.var_vch_dn13))) / (2.0 * assign20530_e38539)))), (0.5 * (((2.0 * locals.var_phib_dn14) + locals.var_vch_dn14) + (((((2.0 * locals.var_phib_dn14) + locals.var_vch_dn14) * assign20530_e38531) + (assign20530_e38524 * ((2.0 * locals.var_phib_dn14) + locals.var_vch_dn14))) / (2.0 * assign20530_e38539)))),)
            } else {
                let assign20530_e38544: f64 = (2.0 * locals.var_phib);
                let assign20530_e38546: f64 = (assign20530_e38544 + locals.var_vch);
                let assign20530_e38548: f64 = (assign20530_e38546 - locals.var_ves);
                let assign20530_e38550: f64 = (-10000.0);
                let assign20530_e38552: f64 = (assign20530_e38550 * 0.1);
                let (assign20530_e38567, assign20530_e38567_d_n0, assign20530_e38567_d_n2, assign20530_e38567_d_n3, assign20530_e38567_d_n4, assign20530_e38567_d_n5, assign20530_e38567_d_n6, assign20530_e38567_d_n7, assign20530_e38567_d_n8, assign20530_e38567_d_n9, assign20530_e38567_d_n10, assign20530_e38567_d_n11, assign20530_e38567_d_n13, assign20530_e38567_d_n14,) = {
                    if (assign20530_e38548 < assign20530_e38552) {
                        let assign20530_e38555: f64 = (-0.1);
                        let assign20530_e38557: f64 = (assign20530_e38555 * 0.1);
                        let assign20530_e38560: f64 = (2.0 * locals.var_phib);
                        let assign20530_e38562: f64 = (assign20530_e38560 + locals.var_vch);
                        let assign20530_e38564: f64 = (assign20530_e38562 - locals.var_ves);
                        let assign20530_e38565: f64 = (assign20530_e38557 / assign20530_e38564);
                        (assign20530_e38565, (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn0) + locals.var_vch_dn0)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn2) + locals.var_vch_dn2)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * (((2.0 * locals.var_phib_dn3) + locals.var_vch_dn3) - locals.var_ves_dn3)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn4) + locals.var_vch_dn4)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * (((2.0 * locals.var_phib_dn5) + locals.var_vch_dn5) - locals.var_ves_dn5)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * (((2.0 * locals.var_phib_dn6) + locals.var_vch_dn6) - locals.var_ves_dn6)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn7) + locals.var_vch_dn7)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn8) + locals.var_vch_dn8)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn9) + locals.var_vch_dn9)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn10) + locals.var_vch_dn10)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn11) + locals.var_vch_dn11)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn13) + locals.var_vch_dn13)) / (assign20530_e38564 * assign20530_e38564))), (-((assign20530_e38557 * ((2.0 * locals.var_phib_dn14) + locals.var_vch_dn14)) / (assign20530_e38564 * assign20530_e38564))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20530_e38567, assign20530_e38567_d_n0, assign20530_e38567_d_n2, assign20530_e38567_d_n3, assign20530_e38567_d_n4, assign20530_e38567_d_n5, assign20530_e38567_d_n6, assign20530_e38567_d_n7, assign20530_e38567_d_n8, assign20530_e38567_d_n9, assign20530_e38567_d_n10, assign20530_e38567_d_n11, assign20530_e38567_d_n13, assign20530_e38567_d_n14,)
            }
        };
        (assign20530_e38568, assign20530_e38568_d_n0, assign20530_e38568_d_n2, assign20530_e38568_d_n3, assign20530_e38568_d_n4, assign20530_e38568_d_n5, assign20530_e38568_d_n6, assign20530_e38568_d_n7, assign20530_e38568_d_n8, assign20530_e38568_d_n9, assign20530_e38568_d_n10, assign20530_e38568_d_n11, assign20530_e38568_d_n13, assign20530_e38568_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20530_e38570;
        locals.var_t1_dn0 = assign20530_e38570_d_n0;
        locals.var_t1_dn2 = assign20530_e38570_d_n2;
        locals.var_t1_dn3 = assign20530_e38570_d_n3;
        locals.var_t1_dn4 = assign20530_e38570_d_n4;
        locals.var_t1_dn5 = assign20530_e38570_d_n5;
        locals.var_t1_dn6 = assign20530_e38570_d_n6;
        locals.var_t1_dn7 = assign20530_e38570_d_n7;
        locals.var_t1_dn8 = assign20530_e38570_d_n8;
        locals.var_t1_dn9 = assign20530_e38570_d_n9;
        locals.var_t1_dn10 = assign20530_e38570_d_n10;
        locals.var_t1_dn11 = assign20530_e38570_d_n11;
        locals.var_t1_dn13 = assign20530_e38570_d_n13;
        locals.var_t1_dn14 = assign20530_e38570_d_n14;

        let (assign20540_e38587, assign20540_e38587_d_n0, assign20540_e38587_d_n2, assign20540_e38587_d_n3, assign20540_e38587_d_n4, assign20540_e38587_d_n5, assign20540_e38587_d_n6, assign20540_e38587_d_n7, assign20540_e38587_d_n8, assign20540_e38587_d_n9, assign20540_e38587_d_n10, assign20540_e38587_d_n11, assign20540_e38587_d_n13, assign20540_e38587_d_n14,) = {
    if (locals.var_guard366 != 0.0) {
        let assign20540_e38573: f64 = (-locals.var_k1_t);
        let assign20540_e38576: f64 = (2.0 * locals.var_nvtm);
        let assign20540_e38577: f64 = (assign20540_e38573 / assign20540_e38576);
        let assign20540_e38579: f64 = (locals.var_t1).sqrt();
        let assign20540_e38582: f64 = (2.0 * locals.var_phib);
        let assign20540_e38583: f64 = (assign20540_e38582).sqrt();
        let assign20540_e38584: f64 = (assign20540_e38579 - assign20540_e38583);
        let assign20540_e38585: f64 = (assign20540_e38577 * assign20540_e38584);
        (assign20540_e38585, (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn0)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn0 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn0) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn2)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn2 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn2) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn3)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn3 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn3) / (2.0 * assign20540_e38583))))), ((((((-locals.var_k1_t_dn4) * assign20540_e38576) - (assign20540_e38573 * (2.0 * locals.var_nvtm_dn4))) / (assign20540_e38576 * assign20540_e38576)) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn4 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn4) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn5)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn5 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn5) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn6)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn6 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn6) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn7)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn7 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn7) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn8)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn8 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn8) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn9)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn9 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn9) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn10)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn10 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn10) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn11)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn11 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn11) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn13)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn13 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn13) / (2.0 * assign20540_e38583))))), (((-((assign20540_e38573 * (2.0 * locals.var_nvtm_dn14)) / (assign20540_e38576 * assign20540_e38576))) * assign20540_e38584) + (assign20540_e38577 * ((locals.var_t1_dn14 / (2.0 * assign20540_e38579)) - ((2.0 * locals.var_phib_dn14) / (2.0 * assign20540_e38583))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20540_e38587;
        locals.var_t3_dn0 = assign20540_e38587_d_n0;
        locals.var_t3_dn2 = assign20540_e38587_d_n2;
        locals.var_t3_dn3 = assign20540_e38587_d_n3;
        locals.var_t3_dn4 = assign20540_e38587_d_n4;
        locals.var_t3_dn5 = assign20540_e38587_d_n5;
        locals.var_t3_dn6 = assign20540_e38587_d_n6;
        locals.var_t3_dn7 = assign20540_e38587_d_n7;
        locals.var_t3_dn8 = assign20540_e38587_d_n8;
        locals.var_t3_dn9 = assign20540_e38587_d_n9;
        locals.var_t3_dn10 = assign20540_e38587_d_n10;
        locals.var_t3_dn11 = assign20540_e38587_d_n11;
        locals.var_t3_dn13 = assign20540_e38587_d_n13;
        locals.var_t3_dn14 = assign20540_e38587_d_n14;

    }

    pub(super) fn stamp_transient_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20550_e38600, assign20550_e38600_d_n0, assign20550_e38600_d_n2, assign20550_e38600_d_n3, assign20550_e38600_d_n4, assign20550_e38600_d_n5, assign20550_e38600_d_n6, assign20550_e38600_d_n7, assign20550_e38600_d_n8, assign20550_e38600_d_n9, assign20550_e38600_d_n10, assign20550_e38600_d_n11, assign20550_e38600_d_n13, assign20550_e38600_d_n14,) = {
    if (locals.var_guard366 != 0.0) {
        let assign20550_e38590: f64 = (-locals.var_qdep);
        let assign20550_e38592: f64 = (assign20550_e38590 - locals.var_t3);
        let assign20550_e38594: f64 = (assign20550_e38592 + locals.var_vth_fixed_factor_sub);
        let assign20550_e38597: f64 = (locals.var_qmfactorcvfinal * locals.var_t4);
        let assign20550_e38598: f64 = (assign20550_e38594 + assign20550_e38597);
        (assign20550_e38598, ((((-locals.var_qdep_dn0) - locals.var_t3_dn0) + locals.var_vth_fixed_factor_sub_dn0) + (locals.var_qmfactorcvfinal * locals.var_t4_dn0)), ((((-locals.var_qdep_dn2) - locals.var_t3_dn2) + locals.var_vth_fixed_factor_sub_dn2) + (locals.var_qmfactorcvfinal * locals.var_t4_dn2)), ((((-locals.var_qdep_dn3) - locals.var_t3_dn3) + locals.var_vth_fixed_factor_sub_dn3) + (locals.var_qmfactorcvfinal * locals.var_t4_dn3)), ((((-locals.var_qdep_dn4) - locals.var_t3_dn4) + locals.var_vth_fixed_factor_sub_dn4) + ((locals.var_qmfactorcvfinal_dn4 * locals.var_t4) + (locals.var_qmfactorcvfinal * locals.var_t4_dn4))), ((((-locals.var_qdep_dn5) - locals.var_t3_dn5) + locals.var_vth_fixed_factor_sub_dn5) + (locals.var_qmfactorcvfinal * locals.var_t4_dn5)), ((((-locals.var_qdep_dn6) - locals.var_t3_dn6) + locals.var_vth_fixed_factor_sub_dn6) + (locals.var_qmfactorcvfinal * locals.var_t4_dn6)), ((((-locals.var_qdep_dn7) - locals.var_t3_dn7) + locals.var_vth_fixed_factor_sub_dn7) + (locals.var_qmfactorcvfinal * locals.var_t4_dn7)), ((((-locals.var_qdep_dn8) - locals.var_t3_dn8) + locals.var_vth_fixed_factor_sub_dn8) + (locals.var_qmfactorcvfinal * locals.var_t4_dn8)), ((((-locals.var_qdep_dn9) - locals.var_t3_dn9) + locals.var_vth_fixed_factor_sub_dn9) + (locals.var_qmfactorcvfinal * locals.var_t4_dn9)), ((((-locals.var_qdep_dn10) - locals.var_t3_dn10) + locals.var_vth_fixed_factor_sub_dn10) + (locals.var_qmfactorcvfinal * locals.var_t4_dn10)), ((((-locals.var_qdep_dn11) - locals.var_t3_dn11) + locals.var_vth_fixed_factor_sub_dn11) + (locals.var_qmfactorcvfinal * locals.var_t4_dn11)), ((((-locals.var_qdep_dn13) - locals.var_t3_dn13) + locals.var_vth_fixed_factor_sub_dn13) + (locals.var_qmfactorcvfinal * locals.var_t4_dn13)), ((((-locals.var_qdep_dn14) - locals.var_t3_dn14) + locals.var_vth_fixed_factor_sub_dn14) + (locals.var_qmfactorcvfinal * locals.var_t4_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20550_e38600;
        locals.var_t0_dn0 = assign20550_e38600_d_n0;
        locals.var_t0_dn2 = assign20550_e38600_d_n2;
        locals.var_t0_dn3 = assign20550_e38600_d_n3;
        locals.var_t0_dn4 = assign20550_e38600_d_n4;
        locals.var_t0_dn5 = assign20550_e38600_d_n5;
        locals.var_t0_dn6 = assign20550_e38600_d_n6;
        locals.var_t0_dn7 = assign20550_e38600_d_n7;
        locals.var_t0_dn8 = assign20550_e38600_d_n8;
        locals.var_t0_dn9 = assign20550_e38600_d_n9;
        locals.var_t0_dn10 = assign20550_e38600_d_n10;
        locals.var_t0_dn11 = assign20550_e38600_d_n11;
        locals.var_t0_dn13 = assign20550_e38600_d_n13;
        locals.var_t0_dn14 = assign20550_e38600_d_n14;

        let (assign20560_e38609, assign20560_e38609_d_n0, assign20560_e38609_d_n2, assign20560_e38609_d_n3, assign20560_e38609_d_n4, assign20560_e38609_d_n5, assign20560_e38609_d_n6, assign20560_e38609_d_n7, assign20560_e38609_d_n8, assign20560_e38609_d_n9, assign20560_e38609_d_n10, assign20560_e38609_d_n11, assign20560_e38609_d_n13, assign20560_e38609_d_n14,) = {
    if (locals.var_guard366 != 0.0) {
        let assign20560_e38603: f64 = (-locals.var_qdep);
        let assign20560_e38605: f64 = (assign20560_e38603 - locals.var_t3);
        let assign20560_e38607: f64 = (assign20560_e38605 + locals.var_vth_fixed_factor_si);
        (assign20560_e38607, (((-locals.var_qdep_dn0) - locals.var_t3_dn0) + locals.var_vth_fixed_factor_si_dn0), (((-locals.var_qdep_dn2) - locals.var_t3_dn2) + locals.var_vth_fixed_factor_si_dn2), (((-locals.var_qdep_dn3) - locals.var_t3_dn3) + locals.var_vth_fixed_factor_si_dn3), (((-locals.var_qdep_dn4) - locals.var_t3_dn4) + locals.var_vth_fixed_factor_si_dn4), (((-locals.var_qdep_dn5) - locals.var_t3_dn5) + locals.var_vth_fixed_factor_si_dn5), (((-locals.var_qdep_dn6) - locals.var_t3_dn6) + locals.var_vth_fixed_factor_si_dn6), (((-locals.var_qdep_dn7) - locals.var_t3_dn7) + locals.var_vth_fixed_factor_si_dn7), (((-locals.var_qdep_dn8) - locals.var_t3_dn8) + locals.var_vth_fixed_factor_si_dn8), (((-locals.var_qdep_dn9) - locals.var_t3_dn9) + locals.var_vth_fixed_factor_si_dn9), (((-locals.var_qdep_dn10) - locals.var_t3_dn10) + locals.var_vth_fixed_factor_si_dn10), (((-locals.var_qdep_dn11) - locals.var_t3_dn11) + locals.var_vth_fixed_factor_si_dn11), (((-locals.var_qdep_dn13) - locals.var_t3_dn13) + locals.var_vth_fixed_factor_si_dn13), (((-locals.var_qdep_dn14) - locals.var_t3_dn14) + locals.var_vth_fixed_factor_si_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20560_e38609;
        locals.var_t1_dn0 = assign20560_e38609_d_n0;
        locals.var_t1_dn2 = assign20560_e38609_d_n2;
        locals.var_t1_dn3 = assign20560_e38609_d_n3;
        locals.var_t1_dn4 = assign20560_e38609_d_n4;
        locals.var_t1_dn5 = assign20560_e38609_d_n5;
        locals.var_t1_dn6 = assign20560_e38609_d_n6;
        locals.var_t1_dn7 = assign20560_e38609_d_n7;
        locals.var_t1_dn8 = assign20560_e38609_d_n8;
        locals.var_t1_dn9 = assign20560_e38609_d_n9;
        locals.var_t1_dn10 = assign20560_e38609_d_n10;
        locals.var_t1_dn11 = assign20560_e38609_d_n11;
        locals.var_t1_dn13 = assign20560_e38609_d_n13;
        locals.var_t1_dn14 = assign20560_e38609_d_n14;

        let (assign20570_e38621, assign20570_e38621_d_n0, assign20570_e38621_d_n2, assign20570_e38621_d_n3, assign20570_e38621_d_n4, assign20570_e38621_d_n5, assign20570_e38621_d_n6, assign20570_e38621_d_n7, assign20570_e38621_d_n8, assign20570_e38621_d_n9, assign20570_e38621_d_n10, assign20570_e38621_d_n11, assign20570_e38621_d_n13, assign20570_e38621_d_n14,) = {
    if (locals.var_guard366 == 0.0) {
        let assign20570_e38613: f64 = (-locals.var_qdep);
        let assign20570_e38615: f64 = (assign20570_e38613 + locals.var_vth_fixed_factor_sub);
        let assign20570_e38618: f64 = (locals.var_qmfactorcvfinal * locals.var_t4);
        let assign20570_e38619: f64 = (assign20570_e38615 + assign20570_e38618);
        (assign20570_e38619, (((-locals.var_qdep_dn0) + locals.var_vth_fixed_factor_sub_dn0) + (locals.var_qmfactorcvfinal * locals.var_t4_dn0)), (((-locals.var_qdep_dn2) + locals.var_vth_fixed_factor_sub_dn2) + (locals.var_qmfactorcvfinal * locals.var_t4_dn2)), (((-locals.var_qdep_dn3) + locals.var_vth_fixed_factor_sub_dn3) + (locals.var_qmfactorcvfinal * locals.var_t4_dn3)), (((-locals.var_qdep_dn4) + locals.var_vth_fixed_factor_sub_dn4) + ((locals.var_qmfactorcvfinal_dn4 * locals.var_t4) + (locals.var_qmfactorcvfinal * locals.var_t4_dn4))), (((-locals.var_qdep_dn5) + locals.var_vth_fixed_factor_sub_dn5) + (locals.var_qmfactorcvfinal * locals.var_t4_dn5)), (((-locals.var_qdep_dn6) + locals.var_vth_fixed_factor_sub_dn6) + (locals.var_qmfactorcvfinal * locals.var_t4_dn6)), (((-locals.var_qdep_dn7) + locals.var_vth_fixed_factor_sub_dn7) + (locals.var_qmfactorcvfinal * locals.var_t4_dn7)), (((-locals.var_qdep_dn8) + locals.var_vth_fixed_factor_sub_dn8) + (locals.var_qmfactorcvfinal * locals.var_t4_dn8)), (((-locals.var_qdep_dn9) + locals.var_vth_fixed_factor_sub_dn9) + (locals.var_qmfactorcvfinal * locals.var_t4_dn9)), (((-locals.var_qdep_dn10) + locals.var_vth_fixed_factor_sub_dn10) + (locals.var_qmfactorcvfinal * locals.var_t4_dn10)), (((-locals.var_qdep_dn11) + locals.var_vth_fixed_factor_sub_dn11) + (locals.var_qmfactorcvfinal * locals.var_t4_dn11)), (((-locals.var_qdep_dn13) + locals.var_vth_fixed_factor_sub_dn13) + (locals.var_qmfactorcvfinal * locals.var_t4_dn13)), (((-locals.var_qdep_dn14) + locals.var_vth_fixed_factor_sub_dn14) + (locals.var_qmfactorcvfinal * locals.var_t4_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20570_e38621;
        locals.var_t0_dn0 = assign20570_e38621_d_n0;
        locals.var_t0_dn2 = assign20570_e38621_d_n2;
        locals.var_t0_dn3 = assign20570_e38621_d_n3;
        locals.var_t0_dn4 = assign20570_e38621_d_n4;
        locals.var_t0_dn5 = assign20570_e38621_d_n5;
        locals.var_t0_dn6 = assign20570_e38621_d_n6;
        locals.var_t0_dn7 = assign20570_e38621_d_n7;
        locals.var_t0_dn8 = assign20570_e38621_d_n8;
        locals.var_t0_dn9 = assign20570_e38621_d_n9;
        locals.var_t0_dn10 = assign20570_e38621_d_n10;
        locals.var_t0_dn11 = assign20570_e38621_d_n11;
        locals.var_t0_dn13 = assign20570_e38621_d_n13;
        locals.var_t0_dn14 = assign20570_e38621_d_n14;

        let (assign20580_e38629, assign20580_e38629_d_n0, assign20580_e38629_d_n2, assign20580_e38629_d_n3, assign20580_e38629_d_n4, assign20580_e38629_d_n5, assign20580_e38629_d_n6, assign20580_e38629_d_n7, assign20580_e38629_d_n8, assign20580_e38629_d_n9, assign20580_e38629_d_n10, assign20580_e38629_d_n11, assign20580_e38629_d_n13, assign20580_e38629_d_n14,) = {
    if (locals.var_guard366 == 0.0) {
        let assign20580_e38625: f64 = (-locals.var_qdep);
        let assign20580_e38627: f64 = (assign20580_e38625 + locals.var_vth_fixed_factor_si);
        (assign20580_e38627, ((-locals.var_qdep_dn0) + locals.var_vth_fixed_factor_si_dn0), ((-locals.var_qdep_dn2) + locals.var_vth_fixed_factor_si_dn2), ((-locals.var_qdep_dn3) + locals.var_vth_fixed_factor_si_dn3), ((-locals.var_qdep_dn4) + locals.var_vth_fixed_factor_si_dn4), ((-locals.var_qdep_dn5) + locals.var_vth_fixed_factor_si_dn5), ((-locals.var_qdep_dn6) + locals.var_vth_fixed_factor_si_dn6), ((-locals.var_qdep_dn7) + locals.var_vth_fixed_factor_si_dn7), ((-locals.var_qdep_dn8) + locals.var_vth_fixed_factor_si_dn8), ((-locals.var_qdep_dn9) + locals.var_vth_fixed_factor_si_dn9), ((-locals.var_qdep_dn10) + locals.var_vth_fixed_factor_si_dn10), ((-locals.var_qdep_dn11) + locals.var_vth_fixed_factor_si_dn11), ((-locals.var_qdep_dn13) + locals.var_vth_fixed_factor_si_dn13), ((-locals.var_qdep_dn14) + locals.var_vth_fixed_factor_si_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20580_e38629;
        locals.var_t1_dn0 = assign20580_e38629_d_n0;
        locals.var_t1_dn2 = assign20580_e38629_d_n2;
        locals.var_t1_dn3 = assign20580_e38629_d_n3;
        locals.var_t1_dn4 = assign20580_e38629_d_n4;
        locals.var_t1_dn5 = assign20580_e38629_d_n5;
        locals.var_t1_dn6 = assign20580_e38629_d_n6;
        locals.var_t1_dn7 = assign20580_e38629_d_n7;
        locals.var_t1_dn8 = assign20580_e38629_d_n8;
        locals.var_t1_dn9 = assign20580_e38629_d_n9;
        locals.var_t1_dn10 = assign20580_e38629_d_n10;
        locals.var_t1_dn11 = assign20580_e38629_d_n11;
        locals.var_t1_dn13 = assign20580_e38629_d_n13;
        locals.var_t1_dn14 = assign20580_e38629_d_n14;

        let assign20590_e38632: f64 = (locals.var_vgsfbeff - locals.var_vch);
        let assign20590_e38634: f64 = (assign20590_e38632 / locals.var_nvtm);
        locals.var_t2 = assign20590_e38634;
        locals.var_t2_dn0 = ((((locals.var_vgsfbeff_dn0 - locals.var_vch_dn0) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn0)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn2 = ((((locals.var_vgsfbeff_dn2 - locals.var_vch_dn2) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn2)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn3 = ((((locals.var_vgsfbeff_dn3 - locals.var_vch_dn3) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn4 = ((((locals.var_vgsfbeff_dn4 - locals.var_vch_dn4) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn5 = ((((locals.var_vgsfbeff_dn5 - locals.var_vch_dn5) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn6 = ((((locals.var_vgsfbeff_dn6 - locals.var_vch_dn6) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn7 = ((((locals.var_vgsfbeff_dn7 - locals.var_vch_dn7) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn8 = ((((locals.var_vgsfbeff_dn8 - locals.var_vch_dn8) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn9 = ((((locals.var_vgsfbeff_dn9 - locals.var_vch_dn9) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn9)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn10 = ((((locals.var_vgsfbeff_dn10 - locals.var_vch_dn10) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn11 = ((((locals.var_vgsfbeff_dn11 - locals.var_vch_dn11) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn11)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn13 = ((((locals.var_vgsfbeff_dn13 - locals.var_vch_dn13) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn13)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn14 = ((((locals.var_vgsfbeff_dn14 - locals.var_vch_dn14) * locals.var_nvtm) - (assign20590_e38632 * locals.var_nvtm_dn14)) / (locals.var_nvtm * locals.var_nvtm));

        let assign20600_e38636: f64 = (-locals.var_t2);
        let assign20600_e38638: f64 = (assign20600_e38636 + locals.var_t1);
        locals.var_f0 = assign20600_e38638;
        locals.var_f0_dn0 = ((-locals.var_t2_dn0) + locals.var_t1_dn0);
        locals.var_f0_dn2 = ((-locals.var_t2_dn2) + locals.var_t1_dn2);
        locals.var_f0_dn3 = ((-locals.var_t2_dn3) + locals.var_t1_dn3);
        locals.var_f0_dn4 = ((-locals.var_t2_dn4) + locals.var_t1_dn4);
        locals.var_f0_dn5 = ((-locals.var_t2_dn5) + locals.var_t1_dn5);
        locals.var_f0_dn6 = ((-locals.var_t2_dn6) + locals.var_t1_dn6);
        locals.var_f0_dn7 = ((-locals.var_t2_dn7) + locals.var_t1_dn7);
        locals.var_f0_dn8 = ((-locals.var_t2_dn8) + locals.var_t1_dn8);
        locals.var_f0_dn9 = ((-locals.var_t2_dn9) + locals.var_t1_dn9);
        locals.var_f0_dn10 = ((-locals.var_t2_dn10) + locals.var_t1_dn10);
        locals.var_f0_dn11 = ((-locals.var_t2_dn11) + locals.var_t1_dn11);
        locals.var_f0_dn13 = ((-locals.var_t2_dn13) + locals.var_t1_dn13);
        locals.var_f0_dn14 = ((-locals.var_t2_dn14) + locals.var_t1_dn14);

        let assign20610_e38642: f64 = (locals.var_t2 - locals.var_t0);
        let assign20610_e38643: f64 = (0.5 * assign20610_e38642);
        locals.var_t3 = assign20610_e38643;
        locals.var_t3_dn0 = (0.5 * (locals.var_t2_dn0 - locals.var_t0_dn0));
        locals.var_t3_dn2 = (0.5 * (locals.var_t2_dn2 - locals.var_t0_dn2));
        locals.var_t3_dn3 = (0.5 * (locals.var_t2_dn3 - locals.var_t0_dn3));
        locals.var_t3_dn4 = (0.5 * (locals.var_t2_dn4 - locals.var_t0_dn4));
        locals.var_t3_dn5 = (0.5 * (locals.var_t2_dn5 - locals.var_t0_dn5));
        locals.var_t3_dn6 = (0.5 * (locals.var_t2_dn6 - locals.var_t0_dn6));
        locals.var_t3_dn7 = (0.5 * (locals.var_t2_dn7 - locals.var_t0_dn7));
        locals.var_t3_dn8 = (0.5 * (locals.var_t2_dn8 - locals.var_t0_dn8));
        locals.var_t3_dn9 = (0.5 * (locals.var_t2_dn9 - locals.var_t0_dn9));
        locals.var_t3_dn10 = (0.5 * (locals.var_t2_dn10 - locals.var_t0_dn10));
        locals.var_t3_dn11 = (0.5 * (locals.var_t2_dn11 - locals.var_t0_dn11));
        locals.var_t3_dn13 = (0.5 * (locals.var_t2_dn13 - locals.var_t0_dn13));
        locals.var_t3_dn14 = (0.5 * (locals.var_t2_dn14 - locals.var_t0_dn14));

        let assign20620_e38645: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_qm = assign20620_e38645;
        locals.var_qm_dn0 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn0);
        locals.var_qm_dn2 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn2);
        locals.var_qm_dn3 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3);
        locals.var_qm_dn4 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4);
        locals.var_qm_dn5 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5);
        locals.var_qm_dn6 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6);
        locals.var_qm_dn7 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7);
        locals.var_qm_dn8 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8);
        locals.var_qm_dn9 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9);
        locals.var_qm_dn10 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10);
        locals.var_qm_dn11 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11);
        locals.var_qm_dn13 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn13);
        locals.var_qm_dn14 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn14);

        let assign20630_e38648: f64 = if locals.var_qm > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard367 = assign20630_e38648;

        let (assign20640_e38655, assign20640_e38655_d_n0, assign20640_e38655_d_n2, assign20640_e38655_d_n3, assign20640_e38655_d_n4, assign20640_e38655_d_n5, assign20640_e38655_d_n6, assign20640_e38655_d_n7, assign20640_e38655_d_n8, assign20640_e38655_d_n9, assign20640_e38655_d_n10, assign20640_e38655_d_n11, assign20640_e38655_d_n13, assign20640_e38655_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20640_e38652: f64 = (1.0 + locals.var_qm);
        let assign20640_e38653: f64 = (assign20640_e38652).ln();
        (assign20640_e38653, (locals.var_qm_dn0 / assign20640_e38652), (locals.var_qm_dn2 / assign20640_e38652), (locals.var_qm_dn3 / assign20640_e38652), (locals.var_qm_dn4 / assign20640_e38652), (locals.var_qm_dn5 / assign20640_e38652), (locals.var_qm_dn6 / assign20640_e38652), (locals.var_qm_dn7 / assign20640_e38652), (locals.var_qm_dn8 / assign20640_e38652), (locals.var_qm_dn9 / assign20640_e38652), (locals.var_qm_dn10 / assign20640_e38652), (locals.var_qm_dn11 / assign20640_e38652), (locals.var_qm_dn13 / assign20640_e38652), (locals.var_qm_dn14 / assign20640_e38652),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign20640_e38655;
        locals.var_t7_dn0 = assign20640_e38655_d_n0;
        locals.var_t7_dn2 = assign20640_e38655_d_n2;
        locals.var_t7_dn3 = assign20640_e38655_d_n3;
        locals.var_t7_dn4 = assign20640_e38655_d_n4;
        locals.var_t7_dn5 = assign20640_e38655_d_n5;
        locals.var_t7_dn6 = assign20640_e38655_d_n6;
        locals.var_t7_dn7 = assign20640_e38655_d_n7;
        locals.var_t7_dn8 = assign20640_e38655_d_n8;
        locals.var_t7_dn9 = assign20640_e38655_d_n9;
        locals.var_t7_dn10 = assign20640_e38655_d_n10;
        locals.var_t7_dn11 = assign20640_e38655_d_n11;
        locals.var_t7_dn13 = assign20640_e38655_d_n13;
        locals.var_t7_dn14 = assign20640_e38655_d_n14;

        let (assign20650_e38668, assign20650_e38668_d_n0, assign20650_e38668_d_n2, assign20650_e38668_d_n3, assign20650_e38668_d_n4, assign20650_e38668_d_n5, assign20650_e38668_d_n6, assign20650_e38668_d_n7, assign20650_e38668_d_n8, assign20650_e38668_d_n9, assign20650_e38668_d_n10, assign20650_e38668_d_n11, assign20650_e38668_d_n13, assign20650_e38668_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20650_e38662: f64 = (locals.var_t7 * locals.var_t7);
        let assign20650_e38663: f64 = (1.0 + assign20650_e38662);
        let assign20650_e38664: f64 = (assign20650_e38663).sqrt();
        let assign20650_e38665: f64 = (1.0 - assign20650_e38664);
        let assign20650_e38666: f64 = (2.0 * assign20650_e38665);
        (assign20650_e38666, (2.0 * (-(((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)) / (2.0 * assign20650_e38664)))), (2.0 * (-(((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)) / (2.0 * assign20650_e38664)))),)
    } else {
        (locals.var_qm, locals.var_qm_dn0, locals.var_qm_dn2, locals.var_qm_dn3, locals.var_qm_dn4, locals.var_qm_dn5, locals.var_qm_dn6, locals.var_qm_dn7, locals.var_qm_dn8, locals.var_qm_dn9, locals.var_qm_dn10, locals.var_qm_dn11, locals.var_qm_dn13, locals.var_qm_dn14,)
    }
};
        locals.var_qm = assign20650_e38668;
        locals.var_qm_dn0 = assign20650_e38668_d_n0;
        locals.var_qm_dn2 = assign20650_e38668_d_n2;
        locals.var_qm_dn3 = assign20650_e38668_d_n3;
        locals.var_qm_dn4 = assign20650_e38668_d_n4;
        locals.var_qm_dn5 = assign20650_e38668_d_n5;
        locals.var_qm_dn6 = assign20650_e38668_d_n6;
        locals.var_qm_dn7 = assign20650_e38668_d_n7;
        locals.var_qm_dn8 = assign20650_e38668_d_n8;
        locals.var_qm_dn9 = assign20650_e38668_d_n9;
        locals.var_qm_dn10 = assign20650_e38668_d_n10;
        locals.var_qm_dn11 = assign20650_e38668_d_n11;
        locals.var_qm_dn13 = assign20650_e38668_d_n13;
        locals.var_qm_dn14 = assign20650_e38668_d_n14;

        let (assign20660_e38678, assign20660_e38678_d_n0, assign20660_e38678_d_n2, assign20660_e38678_d_n3, assign20660_e38678_d_n4, assign20660_e38678_d_n5, assign20660_e38678_d_n6, assign20660_e38678_d_n7, assign20660_e38678_d_n8, assign20660_e38678_d_n9, assign20660_e38678_d_n10, assign20660_e38678_d_n11, assign20660_e38678_d_n13, assign20660_e38678_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20660_e38672: f64 = (locals.var_qm * p.p1805);
        let assign20660_e38674: f64 = (assign20660_e38672 + locals.var_qdep);
        let assign20660_e38676: f64 = (assign20660_e38674 * locals.var_rc);
        (assign20660_e38676, (((locals.var_qm_dn0 * p.p1805) + locals.var_qdep_dn0) * locals.var_rc), (((locals.var_qm_dn2 * p.p1805) + locals.var_qdep_dn2) * locals.var_rc), (((locals.var_qm_dn3 * p.p1805) + locals.var_qdep_dn3) * locals.var_rc), (((locals.var_qm_dn4 * p.p1805) + locals.var_qdep_dn4) * locals.var_rc), (((locals.var_qm_dn5 * p.p1805) + locals.var_qdep_dn5) * locals.var_rc), (((locals.var_qm_dn6 * p.p1805) + locals.var_qdep_dn6) * locals.var_rc), (((locals.var_qm_dn7 * p.p1805) + locals.var_qdep_dn7) * locals.var_rc), (((locals.var_qm_dn8 * p.p1805) + locals.var_qdep_dn8) * locals.var_rc), (((locals.var_qm_dn9 * p.p1805) + locals.var_qdep_dn9) * locals.var_rc), (((locals.var_qm_dn10 * p.p1805) + locals.var_qdep_dn10) * locals.var_rc), (((locals.var_qm_dn11 * p.p1805) + locals.var_qdep_dn11) * locals.var_rc), (((locals.var_qm_dn13 * p.p1805) + locals.var_qdep_dn13) * locals.var_rc), (((locals.var_qm_dn14 * p.p1805) + locals.var_qdep_dn14) * locals.var_rc),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign20660_e38678;
        locals.var_t8_dn0 = assign20660_e38678_d_n0;
        locals.var_t8_dn2 = assign20660_e38678_d_n2;
        locals.var_t8_dn3 = assign20660_e38678_d_n3;
        locals.var_t8_dn4 = assign20660_e38678_d_n4;
        locals.var_t8_dn5 = assign20660_e38678_d_n5;
        locals.var_t8_dn6 = assign20660_e38678_d_n6;
        locals.var_t8_dn7 = assign20660_e38678_d_n7;
        locals.var_t8_dn8 = assign20660_e38678_d_n8;
        locals.var_t8_dn9 = assign20660_e38678_d_n9;
        locals.var_t8_dn10 = assign20660_e38678_d_n10;
        locals.var_t8_dn11 = assign20660_e38678_d_n11;
        locals.var_t8_dn13 = assign20660_e38678_d_n13;
        locals.var_t8_dn14 = assign20660_e38678_d_n14;

        let (assign20670_e38689, assign20670_e38689_d_n0, assign20670_e38689_d_n2, assign20670_e38689_d_n3, assign20670_e38689_d_n4, assign20670_e38689_d_n5, assign20670_e38689_d_n6, assign20670_e38689_d_n7, assign20670_e38689_d_n8, assign20670_e38689_d_n9, assign20670_e38689_d_n10, assign20670_e38689_d_n11, assign20670_e38689_d_n13, assign20670_e38689_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20670_e38682: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign20670_e38684: f64 = (assign20670_e38682 - locals.var_t8);
        let assign20670_e38686: f64 = (assign20670_e38684 - 1.0);
        let assign20670_e38687: f64 = (locals.var_t8 / assign20670_e38686);
        (assign20670_e38687, (((locals.var_t8_dn0 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0) - locals.var_t8_dn0))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn2 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2) - locals.var_t8_dn2))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn3 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3) - locals.var_t8_dn3))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn4 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4) - locals.var_t8_dn4))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn5 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5) - locals.var_t8_dn5))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn6 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6) - locals.var_t8_dn6))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn7 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7) - locals.var_t8_dn7))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn8 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8) - locals.var_t8_dn8))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn9 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9) - locals.var_t8_dn9))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn10 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10) - locals.var_t8_dn10))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn11 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11) - locals.var_t8_dn11))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn13 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13) - locals.var_t8_dn13))) / (assign20670_e38686 * assign20670_e38686)), (((locals.var_t8_dn14 * assign20670_e38686) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14) - locals.var_t8_dn14))) / (assign20670_e38686 * assign20670_e38686)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20670_e38689;
        locals.var_t4_dn0 = assign20670_e38689_d_n0;
        locals.var_t4_dn2 = assign20670_e38689_d_n2;
        locals.var_t4_dn3 = assign20670_e38689_d_n3;
        locals.var_t4_dn4 = assign20670_e38689_d_n4;
        locals.var_t4_dn5 = assign20670_e38689_d_n5;
        locals.var_t4_dn6 = assign20670_e38689_d_n6;
        locals.var_t4_dn7 = assign20670_e38689_d_n7;
        locals.var_t4_dn8 = assign20670_e38689_d_n8;
        locals.var_t4_dn9 = assign20670_e38689_d_n9;
        locals.var_t4_dn10 = assign20670_e38689_d_n10;
        locals.var_t4_dn11 = assign20670_e38689_d_n11;
        locals.var_t4_dn13 = assign20670_e38689_d_n13;
        locals.var_t4_dn14 = assign20670_e38689_d_n14;

        let (assign20680_e38695, assign20680_e38695_d_n0, assign20680_e38695_d_n2, assign20680_e38695_d_n3, assign20680_e38695_d_n4, assign20680_e38695_d_n5, assign20680_e38695_d_n6, assign20680_e38695_d_n7, assign20680_e38695_d_n8, assign20680_e38695_d_n9, assign20680_e38695_d_n10, assign20680_e38695_d_n11, assign20680_e38695_d_n13, assign20680_e38695_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20680_e38693: f64 = (locals.var_t8 * locals.var_t4);
        (assign20680_e38693, ((locals.var_t8_dn0 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn0)), ((locals.var_t8_dn2 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn2)), ((locals.var_t8_dn3 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn3)), ((locals.var_t8_dn4 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn4)), ((locals.var_t8_dn5 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn5)), ((locals.var_t8_dn6 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn6)), ((locals.var_t8_dn7 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn7)), ((locals.var_t8_dn8 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn8)), ((locals.var_t8_dn9 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn9)), ((locals.var_t8_dn10 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn10)), ((locals.var_t8_dn11 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn11)), ((locals.var_t8_dn13 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn13)), ((locals.var_t8_dn14 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign20680_e38695;
        locals.var_t5_dn0 = assign20680_e38695_d_n0;
        locals.var_t5_dn2 = assign20680_e38695_d_n2;
        locals.var_t5_dn3 = assign20680_e38695_d_n3;
        locals.var_t5_dn4 = assign20680_e38695_d_n4;
        locals.var_t5_dn5 = assign20680_e38695_d_n5;
        locals.var_t5_dn6 = assign20680_e38695_d_n6;
        locals.var_t5_dn7 = assign20680_e38695_d_n7;
        locals.var_t5_dn8 = assign20680_e38695_d_n8;
        locals.var_t5_dn9 = assign20680_e38695_d_n9;
        locals.var_t5_dn10 = assign20680_e38695_d_n10;
        locals.var_t5_dn11 = assign20680_e38695_d_n11;
        locals.var_t5_dn13 = assign20680_e38695_d_n13;
        locals.var_t5_dn14 = assign20680_e38695_d_n14;

        let (assign20690_e38703, assign20690_e38703_d_n0, assign20690_e38703_d_n2, assign20690_e38703_d_n3, assign20690_e38703_d_n4, assign20690_e38703_d_n5, assign20690_e38703_d_n6, assign20690_e38703_d_n7, assign20690_e38703_d_n8, assign20690_e38703_d_n9, assign20690_e38703_d_n10, assign20690_e38703_d_n11, assign20690_e38703_d_n13, assign20690_e38703_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20690_e38699: f64 = (locals.var_qm + locals.var_qdep);
        let assign20690_e38700: f64 = (-assign20690_e38699);
        let assign20690_e38701: f64 = (assign20690_e38700).ln();
        (assign20690_e38701, ((-(locals.var_qm_dn0 + locals.var_qdep_dn0)) / assign20690_e38700), ((-(locals.var_qm_dn2 + locals.var_qdep_dn2)) / assign20690_e38700), ((-(locals.var_qm_dn3 + locals.var_qdep_dn3)) / assign20690_e38700), ((-(locals.var_qm_dn4 + locals.var_qdep_dn4)) / assign20690_e38700), ((-(locals.var_qm_dn5 + locals.var_qdep_dn5)) / assign20690_e38700), ((-(locals.var_qm_dn6 + locals.var_qdep_dn6)) / assign20690_e38700), ((-(locals.var_qm_dn7 + locals.var_qdep_dn7)) / assign20690_e38700), ((-(locals.var_qm_dn8 + locals.var_qdep_dn8)) / assign20690_e38700), ((-(locals.var_qm_dn9 + locals.var_qdep_dn9)) / assign20690_e38700), ((-(locals.var_qm_dn10 + locals.var_qdep_dn10)) / assign20690_e38700), ((-(locals.var_qm_dn11 + locals.var_qdep_dn11)) / assign20690_e38700), ((-(locals.var_qm_dn13 + locals.var_qdep_dn13)) / assign20690_e38700), ((-(locals.var_qm_dn14 + locals.var_qdep_dn14)) / assign20690_e38700),)
    } else {
        (locals.var_qm_ln, locals.var_qm_ln_dn0, locals.var_qm_ln_dn2, locals.var_qm_ln_dn3, locals.var_qm_ln_dn4, locals.var_qm_ln_dn5, locals.var_qm_ln_dn6, locals.var_qm_ln_dn7, locals.var_qm_ln_dn8, locals.var_qm_ln_dn9, locals.var_qm_ln_dn10, locals.var_qm_ln_dn11, locals.var_qm_ln_dn13, locals.var_qm_ln_dn14,)
    }
};
        locals.var_qm_ln = assign20690_e38703;
        locals.var_qm_ln_dn0 = assign20690_e38703_d_n0;
        locals.var_qm_ln_dn2 = assign20690_e38703_d_n2;
        locals.var_qm_ln_dn3 = assign20690_e38703_d_n3;
        locals.var_qm_ln_dn4 = assign20690_e38703_d_n4;
        locals.var_qm_ln_dn5 = assign20690_e38703_d_n5;
        locals.var_qm_ln_dn6 = assign20690_e38703_d_n6;
        locals.var_qm_ln_dn7 = assign20690_e38703_d_n7;
        locals.var_qm_ln_dn8 = assign20690_e38703_d_n8;
        locals.var_qm_ln_dn9 = assign20690_e38703_d_n9;
        locals.var_qm_ln_dn10 = assign20690_e38703_d_n10;
        locals.var_qm_ln_dn11 = assign20690_e38703_d_n11;
        locals.var_qm_ln_dn13 = assign20690_e38703_d_n13;
        locals.var_qm_ln_dn14 = assign20690_e38703_d_n14;

        let (assign20700_e38749, assign20700_e38749_d_n0, assign20700_e38749_d_n2, assign20700_e38749_d_n3, assign20700_e38749_d_n4, assign20700_e38749_d_n5, assign20700_e38749_d_n6, assign20700_e38749_d_n7, assign20700_e38749_d_n8, assign20700_e38749_d_n9, assign20700_e38749_d_n10, assign20700_e38749_d_n11, assign20700_e38749_d_n13, assign20700_e38749_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20700_e38707: f64 = (locals.var_f0 - locals.var_qm);
        let assign20700_e38709: f64 = (-locals.var_qm);
        let (assign20700_e38724, assign20700_e38724_d_n0, assign20700_e38724_d_n2, assign20700_e38724_d_n3, assign20700_e38724_d_n4, assign20700_e38724_d_n5, assign20700_e38724_d_n6, assign20700_e38724_d_n7, assign20700_e38724_d_n8, assign20700_e38724_d_n9, assign20700_e38724_d_n10, assign20700_e38724_d_n11, assign20700_e38724_d_n13, assign20700_e38724_d_n14,) = {
            if (!(assign20700_e38709 > 1e-38)) {
                let assign20700_e38714: f64 = (-87.498233534);
                (assign20700_e38714, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign20700_e38716: f64 = (-locals.var_qm);
                let (assign20700_e38723, assign20700_e38723_d_n0, assign20700_e38723_d_n2, assign20700_e38723_d_n3, assign20700_e38723_d_n4, assign20700_e38723_d_n5, assign20700_e38723_d_n6, assign20700_e38723_d_n7, assign20700_e38723_d_n8, assign20700_e38723_d_n9, assign20700_e38723_d_n10, assign20700_e38723_d_n11, assign20700_e38723_d_n13, assign20700_e38723_d_n14,) = {
                    if (assign20700_e38716 > 1e-38) {
                        let assign20700_e38720: f64 = (-locals.var_qm);
                        let assign20700_e38721: f64 = (assign20700_e38720).ln();
                        (assign20700_e38721, ((-locals.var_qm_dn0) / assign20700_e38720), ((-locals.var_qm_dn2) / assign20700_e38720), ((-locals.var_qm_dn3) / assign20700_e38720), ((-locals.var_qm_dn4) / assign20700_e38720), ((-locals.var_qm_dn5) / assign20700_e38720), ((-locals.var_qm_dn6) / assign20700_e38720), ((-locals.var_qm_dn7) / assign20700_e38720), ((-locals.var_qm_dn8) / assign20700_e38720), ((-locals.var_qm_dn9) / assign20700_e38720), ((-locals.var_qm_dn10) / assign20700_e38720), ((-locals.var_qm_dn11) / assign20700_e38720), ((-locals.var_qm_dn13) / assign20700_e38720), ((-locals.var_qm_dn14) / assign20700_e38720),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20700_e38723, assign20700_e38723_d_n0, assign20700_e38723_d_n2, assign20700_e38723_d_n3, assign20700_e38723_d_n4, assign20700_e38723_d_n5, assign20700_e38723_d_n6, assign20700_e38723_d_n7, assign20700_e38723_d_n8, assign20700_e38723_d_n9, assign20700_e38723_d_n10, assign20700_e38723_d_n11, assign20700_e38723_d_n13, assign20700_e38723_d_n14,)
            }
        };
        let assign20700_e38725: f64 = (assign20700_e38707 + assign20700_e38724);
        let (assign20700_e38739, assign20700_e38739_d_n0, assign20700_e38739_d_n2, assign20700_e38739_d_n3, assign20700_e38739_d_n4, assign20700_e38739_d_n5, assign20700_e38739_d_n6, assign20700_e38739_d_n7, assign20700_e38739_d_n8, assign20700_e38739_d_n9, assign20700_e38739_d_n10, assign20700_e38739_d_n11, assign20700_e38739_d_n13, assign20700_e38739_d_n14,) = {
            if (!(locals.var_t5 > 1e-38)) {
                let assign20700_e38731: f64 = (-87.498233534);
                (assign20700_e38731, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (assign20700_e38738, assign20700_e38738_d_n0, assign20700_e38738_d_n2, assign20700_e38738_d_n3, assign20700_e38738_d_n4, assign20700_e38738_d_n5, assign20700_e38738_d_n6, assign20700_e38738_d_n7, assign20700_e38738_d_n8, assign20700_e38738_d_n9, assign20700_e38738_d_n10, assign20700_e38738_d_n11, assign20700_e38738_d_n13, assign20700_e38738_d_n14,) = {
                    if (locals.var_t5 > 1e-38) {
                        let assign20700_e38736: f64 = (locals.var_t5).ln();
                        (assign20700_e38736, (locals.var_t5_dn0 / locals.var_t5), (locals.var_t5_dn2 / locals.var_t5), (locals.var_t5_dn3 / locals.var_t5), (locals.var_t5_dn4 / locals.var_t5), (locals.var_t5_dn5 / locals.var_t5), (locals.var_t5_dn6 / locals.var_t5), (locals.var_t5_dn7 / locals.var_t5), (locals.var_t5_dn8 / locals.var_t5), (locals.var_t5_dn9 / locals.var_t5), (locals.var_t5_dn10 / locals.var_t5), (locals.var_t5_dn11 / locals.var_t5), (locals.var_t5_dn13 / locals.var_t5), (locals.var_t5_dn14 / locals.var_t5),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20700_e38738, assign20700_e38738_d_n0, assign20700_e38738_d_n2, assign20700_e38738_d_n3, assign20700_e38738_d_n4, assign20700_e38738_d_n5, assign20700_e38738_d_n6, assign20700_e38738_d_n7, assign20700_e38738_d_n8, assign20700_e38738_d_n9, assign20700_e38738_d_n10, assign20700_e38738_d_n11, assign20700_e38738_d_n13, assign20700_e38738_d_n14,)
            }
        };
        let assign20700_e38740: f64 = (assign20700_e38725 + assign20700_e38739);
        let assign20700_e38744: f64 = (0.666666667 * locals.var_qm_ln);
        let assign20700_e38745: f64 = (assign20700_e38744).exp();
        let assign20700_e38746: f64 = (locals.var_qmfactorcvfinal * assign20700_e38745);
        let assign20700_e38747: f64 = (assign20700_e38740 + assign20700_e38746);
        (assign20700_e38747, ((((locals.var_f0_dn0 - locals.var_qm_dn0) + assign20700_e38724_d_n0) + assign20700_e38739_d_n0) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn0)))), ((((locals.var_f0_dn2 - locals.var_qm_dn2) + assign20700_e38724_d_n2) + assign20700_e38739_d_n2) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn2)))), ((((locals.var_f0_dn3 - locals.var_qm_dn3) + assign20700_e38724_d_n3) + assign20700_e38739_d_n3) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn3)))), ((((locals.var_f0_dn4 - locals.var_qm_dn4) + assign20700_e38724_d_n4) + assign20700_e38739_d_n4) + ((locals.var_qmfactorcvfinal_dn4 * assign20700_e38745) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn4))))), ((((locals.var_f0_dn5 - locals.var_qm_dn5) + assign20700_e38724_d_n5) + assign20700_e38739_d_n5) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn5)))), ((((locals.var_f0_dn6 - locals.var_qm_dn6) + assign20700_e38724_d_n6) + assign20700_e38739_d_n6) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn6)))), ((((locals.var_f0_dn7 - locals.var_qm_dn7) + assign20700_e38724_d_n7) + assign20700_e38739_d_n7) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn7)))), ((((locals.var_f0_dn8 - locals.var_qm_dn8) + assign20700_e38724_d_n8) + assign20700_e38739_d_n8) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn8)))), ((((locals.var_f0_dn9 - locals.var_qm_dn9) + assign20700_e38724_d_n9) + assign20700_e38739_d_n9) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn9)))), ((((locals.var_f0_dn10 - locals.var_qm_dn10) + assign20700_e38724_d_n10) + assign20700_e38739_d_n10) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn10)))), ((((locals.var_f0_dn11 - locals.var_qm_dn11) + assign20700_e38724_d_n11) + assign20700_e38739_d_n11) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn11)))), ((((locals.var_f0_dn13 - locals.var_qm_dn13) + assign20700_e38724_d_n13) + assign20700_e38739_d_n13) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn13)))), ((((locals.var_f0_dn14 - locals.var_qm_dn14) + assign20700_e38724_d_n14) + assign20700_e38739_d_n14) + (locals.var_qmfactorcvfinal * (assign20700_e38745 * (0.666666667 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn2, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10, locals.var_e0_dn11, locals.var_e0_dn13, locals.var_e0_dn14,)
    }
};
        locals.var_e0 = assign20700_e38749;
        locals.var_e0_dn0 = assign20700_e38749_d_n0;
        locals.var_e0_dn2 = assign20700_e38749_d_n2;
        locals.var_e0_dn3 = assign20700_e38749_d_n3;
        locals.var_e0_dn4 = assign20700_e38749_d_n4;
        locals.var_e0_dn5 = assign20700_e38749_d_n5;
        locals.var_e0_dn6 = assign20700_e38749_d_n6;
        locals.var_e0_dn7 = assign20700_e38749_d_n7;
        locals.var_e0_dn8 = assign20700_e38749_d_n8;
        locals.var_e0_dn9 = assign20700_e38749_d_n9;
        locals.var_e0_dn10 = assign20700_e38749_d_n10;
        locals.var_e0_dn11 = assign20700_e38749_d_n11;
        locals.var_e0_dn13 = assign20700_e38749_d_n13;
        locals.var_e0_dn14 = assign20700_e38749_d_n14;

        let (assign20710_e38778, assign20710_e38778_d_n0, assign20710_e38778_d_n2, assign20710_e38778_d_n3, assign20710_e38778_d_n4, assign20710_e38778_d_n5, assign20710_e38778_d_n6, assign20710_e38778_d_n7, assign20710_e38778_d_n8, assign20710_e38778_d_n9, assign20710_e38778_d_n10, assign20710_e38778_d_n11, assign20710_e38778_d_n13, assign20710_e38778_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20710_e38752: f64 = (-1.0);
        let assign20710_e38755: f64 = (1.0 / locals.var_qm);
        let assign20710_e38756: f64 = (assign20710_e38752 + assign20710_e38755);
        let assign20710_e38759: f64 = (2.0 / locals.var_t8);
        let assign20710_e38761: f64 = (assign20710_e38759 - locals.var_t4);
        let assign20710_e38763: f64 = (assign20710_e38761 - 1.0);
        let assign20710_e38765: f64 = (assign20710_e38763 * locals.var_rc);
        let assign20710_e38766: f64 = (assign20710_e38756 + assign20710_e38765);
        let assign20710_e38769: f64 = (0.666666667 * locals.var_qmfactorcvfinal);
        let assign20710_e38771: f64 = (-0.333333333);
        let assign20710_e38773: f64 = (assign20710_e38771 * locals.var_qm_ln);
        let assign20710_e38774: f64 = (assign20710_e38773).exp();
        let assign20710_e38775: f64 = (assign20710_e38769 * assign20710_e38774);
        let assign20710_e38776: f64 = (assign20710_e38766 - assign20710_e38775);
        (assign20710_e38776, (((-(locals.var_qm_dn0 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn0) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn0) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn0)))), (((-(locals.var_qm_dn2 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn2) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn2) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn2)))), (((-(locals.var_qm_dn3 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn3) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn3) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn3)))), (((-(locals.var_qm_dn4 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn4) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn4) * locals.var_rc)) - (((0.666666667 * locals.var_qmfactorcvfinal_dn4) * assign20710_e38774) + (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn4))))), (((-(locals.var_qm_dn5 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn5) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn5) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn5)))), (((-(locals.var_qm_dn6 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn6) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn6) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn6)))), (((-(locals.var_qm_dn7 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn7) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn7) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn7)))), (((-(locals.var_qm_dn8 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn8) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn8) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn8)))), (((-(locals.var_qm_dn9 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn9) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn9) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn9)))), (((-(locals.var_qm_dn10 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn10) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn10) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn10)))), (((-(locals.var_qm_dn11 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn11) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn11) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn11)))), (((-(locals.var_qm_dn13 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn13) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn13) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn13)))), (((-(locals.var_qm_dn14 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn14) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn14) * locals.var_rc)) - (assign20710_e38769 * (assign20710_e38774 * (assign20710_e38771 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn3, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn13, locals.var_e1_dn14,)
    }
};
        locals.var_e1 = assign20710_e38778;
        locals.var_e1_dn0 = assign20710_e38778_d_n0;
        locals.var_e1_dn2 = assign20710_e38778_d_n2;
        locals.var_e1_dn3 = assign20710_e38778_d_n3;
        locals.var_e1_dn4 = assign20710_e38778_d_n4;
        locals.var_e1_dn5 = assign20710_e38778_d_n5;
        locals.var_e1_dn6 = assign20710_e38778_d_n6;
        locals.var_e1_dn7 = assign20710_e38778_d_n7;
        locals.var_e1_dn8 = assign20710_e38778_d_n8;
        locals.var_e1_dn9 = assign20710_e38778_d_n9;
        locals.var_e1_dn10 = assign20710_e38778_d_n10;
        locals.var_e1_dn11 = assign20710_e38778_d_n11;
        locals.var_e1_dn13 = assign20710_e38778_d_n13;
        locals.var_e1_dn14 = assign20710_e38778_d_n14;

        let (assign20720_e38799, assign20720_e38799_d_n0, assign20720_e38799_d_n2, assign20720_e38799_d_n3, assign20720_e38799_d_n4, assign20720_e38799_d_n5, assign20720_e38799_d_n6, assign20720_e38799_d_n7, assign20720_e38799_d_n8, assign20720_e38799_d_n9, assign20720_e38799_d_n10, assign20720_e38799_d_n11, assign20720_e38799_d_n13, assign20720_e38799_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20720_e38781: f64 = (-1.0);
        let assign20720_e38784: f64 = (locals.var_qm * locals.var_qm);
        let assign20720_e38785: f64 = (assign20720_e38781 / assign20720_e38784);
        let assign20720_e38788: f64 = (2.0 / 9.0);
        let assign20720_e38790: f64 = (assign20720_e38788 * locals.var_qmfactorcvfinal);
        let assign20720_e38792: f64 = (-1.333333333);
        let assign20720_e38794: f64 = (assign20720_e38792 * locals.var_qm_ln);
        let assign20720_e38795: f64 = (assign20720_e38794).exp();
        let assign20720_e38796: f64 = (assign20720_e38790 * assign20720_e38795);
        let assign20720_e38797: f64 = (assign20720_e38785 - assign20720_e38796);
        (assign20720_e38797, ((-((assign20720_e38781 * ((locals.var_qm_dn0 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn0))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn0)))), ((-((assign20720_e38781 * ((locals.var_qm_dn2 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn2))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn2)))), ((-((assign20720_e38781 * ((locals.var_qm_dn3 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn3))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn3)))), ((-((assign20720_e38781 * ((locals.var_qm_dn4 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn4))) / (assign20720_e38784 * assign20720_e38784))) - (((assign20720_e38788 * locals.var_qmfactorcvfinal_dn4) * assign20720_e38795) + (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn4))))), ((-((assign20720_e38781 * ((locals.var_qm_dn5 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn5))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn5)))), ((-((assign20720_e38781 * ((locals.var_qm_dn6 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn6))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn6)))), ((-((assign20720_e38781 * ((locals.var_qm_dn7 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn7))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn7)))), ((-((assign20720_e38781 * ((locals.var_qm_dn8 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn8))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn8)))), ((-((assign20720_e38781 * ((locals.var_qm_dn9 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn9))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn9)))), ((-((assign20720_e38781 * ((locals.var_qm_dn10 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn10))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn10)))), ((-((assign20720_e38781 * ((locals.var_qm_dn11 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn11))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn11)))), ((-((assign20720_e38781 * ((locals.var_qm_dn13 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn13))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn13)))), ((-((assign20720_e38781 * ((locals.var_qm_dn14 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn14))) / (assign20720_e38784 * assign20720_e38784))) - (assign20720_e38790 * (assign20720_e38795 * (assign20720_e38792 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e2, locals.var_e2_dn0, locals.var_e2_dn2, locals.var_e2_dn3, locals.var_e2_dn4, locals.var_e2_dn5, locals.var_e2_dn6, locals.var_e2_dn7, locals.var_e2_dn8, locals.var_e2_dn9, locals.var_e2_dn10, locals.var_e2_dn11, locals.var_e2_dn13, locals.var_e2_dn14,)
    }
};
        locals.var_e2 = assign20720_e38799;
        locals.var_e2_dn0 = assign20720_e38799_d_n0;
        locals.var_e2_dn2 = assign20720_e38799_d_n2;
        locals.var_e2_dn3 = assign20720_e38799_d_n3;
        locals.var_e2_dn4 = assign20720_e38799_d_n4;
        locals.var_e2_dn5 = assign20720_e38799_d_n5;
        locals.var_e2_dn6 = assign20720_e38799_d_n6;
        locals.var_e2_dn7 = assign20720_e38799_d_n7;
        locals.var_e2_dn8 = assign20720_e38799_d_n8;
        locals.var_e2_dn9 = assign20720_e38799_d_n9;
        locals.var_e2_dn10 = assign20720_e38799_d_n10;
        locals.var_e2_dn11 = assign20720_e38799_d_n11;
        locals.var_e2_dn13 = assign20720_e38799_d_n13;
        locals.var_e2_dn14 = assign20720_e38799_d_n14;

        let (assign20730_e38819, assign20730_e38819_d_n0, assign20730_e38819_d_n2, assign20730_e38819_d_n3, assign20730_e38819_d_n4, assign20730_e38819_d_n5, assign20730_e38819_d_n6, assign20730_e38819_d_n7, assign20730_e38819_d_n8, assign20730_e38819_d_n9, assign20730_e38819_d_n10, assign20730_e38819_d_n11, assign20730_e38819_d_n13, assign20730_e38819_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20730_e38804: f64 = (locals.var_e0 / locals.var_e1);
        let assign20730_e38808: f64 = (locals.var_e0 * locals.var_e2);
        let assign20730_e38811: f64 = (2.0 * locals.var_e1);
        let assign20730_e38813: f64 = (assign20730_e38811 * locals.var_e1);
        let assign20730_e38814: f64 = (assign20730_e38808 / assign20730_e38813);
        let assign20730_e38815: f64 = (1.0 + assign20730_e38814);
        let assign20730_e38816: f64 = (assign20730_e38804 * assign20730_e38815);
        let assign20730_e38817: f64 = (locals.var_qm - assign20730_e38816);
        (assign20730_e38817, (locals.var_qm_dn0 - (((((locals.var_e0_dn0 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn0)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn0 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn0)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn0) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn0)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn2 - (((((locals.var_e0_dn2 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn2)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn2 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn2)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn2) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn2)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn3 - (((((locals.var_e0_dn3 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn3)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn3 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn3)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn3) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn3)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn4 - (((((locals.var_e0_dn4 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn4)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn4 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn4)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn4) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn4)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn5 - (((((locals.var_e0_dn5 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn5)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn5 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn5)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn5) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn5)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn6 - (((((locals.var_e0_dn6 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn6)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn6 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn6)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn6) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn6)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn7 - (((((locals.var_e0_dn7 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn7)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn7 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn7)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn7) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn7)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn8 - (((((locals.var_e0_dn8 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn8)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn8 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn8)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn8) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn8)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn9 - (((((locals.var_e0_dn9 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn9)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn9 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn9)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn9) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn9)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn10 - (((((locals.var_e0_dn10 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn10)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn10 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn10)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn10) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn10)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn11 - (((((locals.var_e0_dn11 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn11)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn11 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn11)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn11) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn11)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn13 - (((((locals.var_e0_dn13 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn13)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn13 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn13)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn13) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn13)))) / (assign20730_e38813 * assign20730_e38813))))), (locals.var_qm_dn14 - (((((locals.var_e0_dn14 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn14)) / (locals.var_e1 * locals.var_e1)) * assign20730_e38815) + (assign20730_e38804 * (((((locals.var_e0_dn14 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn14)) * assign20730_e38813) - (assign20730_e38808 * (((2.0 * locals.var_e1_dn14) * locals.var_e1) + (assign20730_e38811 * locals.var_e1_dn14)))) / (assign20730_e38813 * assign20730_e38813))))),)
    } else {
        (locals.var_qm, locals.var_qm_dn0, locals.var_qm_dn2, locals.var_qm_dn3, locals.var_qm_dn4, locals.var_qm_dn5, locals.var_qm_dn6, locals.var_qm_dn7, locals.var_qm_dn8, locals.var_qm_dn9, locals.var_qm_dn10, locals.var_qm_dn11, locals.var_qm_dn13, locals.var_qm_dn14,)
    }
};
        locals.var_qm = assign20730_e38819;
        locals.var_qm_dn0 = assign20730_e38819_d_n0;
        locals.var_qm_dn2 = assign20730_e38819_d_n2;
        locals.var_qm_dn3 = assign20730_e38819_d_n3;
        locals.var_qm_dn4 = assign20730_e38819_d_n4;
        locals.var_qm_dn5 = assign20730_e38819_d_n5;
        locals.var_qm_dn6 = assign20730_e38819_d_n6;
        locals.var_qm_dn7 = assign20730_e38819_d_n7;
        locals.var_qm_dn8 = assign20730_e38819_d_n8;
        locals.var_qm_dn9 = assign20730_e38819_d_n9;
        locals.var_qm_dn10 = assign20730_e38819_d_n10;
        locals.var_qm_dn11 = assign20730_e38819_d_n11;
        locals.var_qm_dn13 = assign20730_e38819_d_n13;
        locals.var_qm_dn14 = assign20730_e38819_d_n14;

        let (assign20740_e38829, assign20740_e38829_d_n0, assign20740_e38829_d_n2, assign20740_e38829_d_n3, assign20740_e38829_d_n4, assign20740_e38829_d_n5, assign20740_e38829_d_n6, assign20740_e38829_d_n7, assign20740_e38829_d_n8, assign20740_e38829_d_n9, assign20740_e38829_d_n10, assign20740_e38829_d_n11, assign20740_e38829_d_n13, assign20740_e38829_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20740_e38823: f64 = (locals.var_qm * p.p1805);
        let assign20740_e38825: f64 = (assign20740_e38823 + locals.var_qdep);
        let assign20740_e38827: f64 = (assign20740_e38825 * locals.var_rc);
        (assign20740_e38827, (((locals.var_qm_dn0 * p.p1805) + locals.var_qdep_dn0) * locals.var_rc), (((locals.var_qm_dn2 * p.p1805) + locals.var_qdep_dn2) * locals.var_rc), (((locals.var_qm_dn3 * p.p1805) + locals.var_qdep_dn3) * locals.var_rc), (((locals.var_qm_dn4 * p.p1805) + locals.var_qdep_dn4) * locals.var_rc), (((locals.var_qm_dn5 * p.p1805) + locals.var_qdep_dn5) * locals.var_rc), (((locals.var_qm_dn6 * p.p1805) + locals.var_qdep_dn6) * locals.var_rc), (((locals.var_qm_dn7 * p.p1805) + locals.var_qdep_dn7) * locals.var_rc), (((locals.var_qm_dn8 * p.p1805) + locals.var_qdep_dn8) * locals.var_rc), (((locals.var_qm_dn9 * p.p1805) + locals.var_qdep_dn9) * locals.var_rc), (((locals.var_qm_dn10 * p.p1805) + locals.var_qdep_dn10) * locals.var_rc), (((locals.var_qm_dn11 * p.p1805) + locals.var_qdep_dn11) * locals.var_rc), (((locals.var_qm_dn13 * p.p1805) + locals.var_qdep_dn13) * locals.var_rc), (((locals.var_qm_dn14 * p.p1805) + locals.var_qdep_dn14) * locals.var_rc),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign20740_e38829;
        locals.var_t8_dn0 = assign20740_e38829_d_n0;
        locals.var_t8_dn2 = assign20740_e38829_d_n2;
        locals.var_t8_dn3 = assign20740_e38829_d_n3;
        locals.var_t8_dn4 = assign20740_e38829_d_n4;
        locals.var_t8_dn5 = assign20740_e38829_d_n5;
        locals.var_t8_dn6 = assign20740_e38829_d_n6;
        locals.var_t8_dn7 = assign20740_e38829_d_n7;
        locals.var_t8_dn8 = assign20740_e38829_d_n8;
        locals.var_t8_dn9 = assign20740_e38829_d_n9;
        locals.var_t8_dn10 = assign20740_e38829_d_n10;
        locals.var_t8_dn11 = assign20740_e38829_d_n11;
        locals.var_t8_dn13 = assign20740_e38829_d_n13;
        locals.var_t8_dn14 = assign20740_e38829_d_n14;

    }

    pub(super) fn stamp_transient_block_76(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20750_e38840, assign20750_e38840_d_n0, assign20750_e38840_d_n2, assign20750_e38840_d_n3, assign20750_e38840_d_n4, assign20750_e38840_d_n5, assign20750_e38840_d_n6, assign20750_e38840_d_n7, assign20750_e38840_d_n8, assign20750_e38840_d_n9, assign20750_e38840_d_n10, assign20750_e38840_d_n11, assign20750_e38840_d_n13, assign20750_e38840_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20750_e38833: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign20750_e38835: f64 = (assign20750_e38833 - locals.var_t8);
        let assign20750_e38837: f64 = (assign20750_e38835 - 1.0);
        let assign20750_e38838: f64 = (locals.var_t8 / assign20750_e38837);
        (assign20750_e38838, (((locals.var_t8_dn0 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0) - locals.var_t8_dn0))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn2 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2) - locals.var_t8_dn2))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn3 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3) - locals.var_t8_dn3))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn4 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4) - locals.var_t8_dn4))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn5 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5) - locals.var_t8_dn5))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn6 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6) - locals.var_t8_dn6))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn7 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7) - locals.var_t8_dn7))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn8 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8) - locals.var_t8_dn8))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn9 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9) - locals.var_t8_dn9))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn10 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10) - locals.var_t8_dn10))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn11 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11) - locals.var_t8_dn11))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn13 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13) - locals.var_t8_dn13))) / (assign20750_e38837 * assign20750_e38837)), (((locals.var_t8_dn14 * assign20750_e38837) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14) - locals.var_t8_dn14))) / (assign20750_e38837 * assign20750_e38837)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20750_e38840;
        locals.var_t4_dn0 = assign20750_e38840_d_n0;
        locals.var_t4_dn2 = assign20750_e38840_d_n2;
        locals.var_t4_dn3 = assign20750_e38840_d_n3;
        locals.var_t4_dn4 = assign20750_e38840_d_n4;
        locals.var_t4_dn5 = assign20750_e38840_d_n5;
        locals.var_t4_dn6 = assign20750_e38840_d_n6;
        locals.var_t4_dn7 = assign20750_e38840_d_n7;
        locals.var_t4_dn8 = assign20750_e38840_d_n8;
        locals.var_t4_dn9 = assign20750_e38840_d_n9;
        locals.var_t4_dn10 = assign20750_e38840_d_n10;
        locals.var_t4_dn11 = assign20750_e38840_d_n11;
        locals.var_t4_dn13 = assign20750_e38840_d_n13;
        locals.var_t4_dn14 = assign20750_e38840_d_n14;

        let (assign20760_e38846, assign20760_e38846_d_n0, assign20760_e38846_d_n2, assign20760_e38846_d_n3, assign20760_e38846_d_n4, assign20760_e38846_d_n5, assign20760_e38846_d_n6, assign20760_e38846_d_n7, assign20760_e38846_d_n8, assign20760_e38846_d_n9, assign20760_e38846_d_n10, assign20760_e38846_d_n11, assign20760_e38846_d_n13, assign20760_e38846_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20760_e38844: f64 = (locals.var_t8 * locals.var_t4);
        (assign20760_e38844, ((locals.var_t8_dn0 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn0)), ((locals.var_t8_dn2 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn2)), ((locals.var_t8_dn3 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn3)), ((locals.var_t8_dn4 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn4)), ((locals.var_t8_dn5 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn5)), ((locals.var_t8_dn6 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn6)), ((locals.var_t8_dn7 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn7)), ((locals.var_t8_dn8 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn8)), ((locals.var_t8_dn9 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn9)), ((locals.var_t8_dn10 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn10)), ((locals.var_t8_dn11 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn11)), ((locals.var_t8_dn13 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn13)), ((locals.var_t8_dn14 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign20760_e38846;
        locals.var_t5_dn0 = assign20760_e38846_d_n0;
        locals.var_t5_dn2 = assign20760_e38846_d_n2;
        locals.var_t5_dn3 = assign20760_e38846_d_n3;
        locals.var_t5_dn4 = assign20760_e38846_d_n4;
        locals.var_t5_dn5 = assign20760_e38846_d_n5;
        locals.var_t5_dn6 = assign20760_e38846_d_n6;
        locals.var_t5_dn7 = assign20760_e38846_d_n7;
        locals.var_t5_dn8 = assign20760_e38846_d_n8;
        locals.var_t5_dn9 = assign20760_e38846_d_n9;
        locals.var_t5_dn10 = assign20760_e38846_d_n10;
        locals.var_t5_dn11 = assign20760_e38846_d_n11;
        locals.var_t5_dn13 = assign20760_e38846_d_n13;
        locals.var_t5_dn14 = assign20760_e38846_d_n14;

        let (assign20770_e38854, assign20770_e38854_d_n0, assign20770_e38854_d_n2, assign20770_e38854_d_n3, assign20770_e38854_d_n4, assign20770_e38854_d_n5, assign20770_e38854_d_n6, assign20770_e38854_d_n7, assign20770_e38854_d_n8, assign20770_e38854_d_n9, assign20770_e38854_d_n10, assign20770_e38854_d_n11, assign20770_e38854_d_n13, assign20770_e38854_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20770_e38850: f64 = (locals.var_qm + locals.var_qdep);
        let assign20770_e38851: f64 = (-assign20770_e38850);
        let assign20770_e38852: f64 = (assign20770_e38851).ln();
        (assign20770_e38852, ((-(locals.var_qm_dn0 + locals.var_qdep_dn0)) / assign20770_e38851), ((-(locals.var_qm_dn2 + locals.var_qdep_dn2)) / assign20770_e38851), ((-(locals.var_qm_dn3 + locals.var_qdep_dn3)) / assign20770_e38851), ((-(locals.var_qm_dn4 + locals.var_qdep_dn4)) / assign20770_e38851), ((-(locals.var_qm_dn5 + locals.var_qdep_dn5)) / assign20770_e38851), ((-(locals.var_qm_dn6 + locals.var_qdep_dn6)) / assign20770_e38851), ((-(locals.var_qm_dn7 + locals.var_qdep_dn7)) / assign20770_e38851), ((-(locals.var_qm_dn8 + locals.var_qdep_dn8)) / assign20770_e38851), ((-(locals.var_qm_dn9 + locals.var_qdep_dn9)) / assign20770_e38851), ((-(locals.var_qm_dn10 + locals.var_qdep_dn10)) / assign20770_e38851), ((-(locals.var_qm_dn11 + locals.var_qdep_dn11)) / assign20770_e38851), ((-(locals.var_qm_dn13 + locals.var_qdep_dn13)) / assign20770_e38851), ((-(locals.var_qm_dn14 + locals.var_qdep_dn14)) / assign20770_e38851),)
    } else {
        (locals.var_qm_ln, locals.var_qm_ln_dn0, locals.var_qm_ln_dn2, locals.var_qm_ln_dn3, locals.var_qm_ln_dn4, locals.var_qm_ln_dn5, locals.var_qm_ln_dn6, locals.var_qm_ln_dn7, locals.var_qm_ln_dn8, locals.var_qm_ln_dn9, locals.var_qm_ln_dn10, locals.var_qm_ln_dn11, locals.var_qm_ln_dn13, locals.var_qm_ln_dn14,)
    }
};
        locals.var_qm_ln = assign20770_e38854;
        locals.var_qm_ln_dn0 = assign20770_e38854_d_n0;
        locals.var_qm_ln_dn2 = assign20770_e38854_d_n2;
        locals.var_qm_ln_dn3 = assign20770_e38854_d_n3;
        locals.var_qm_ln_dn4 = assign20770_e38854_d_n4;
        locals.var_qm_ln_dn5 = assign20770_e38854_d_n5;
        locals.var_qm_ln_dn6 = assign20770_e38854_d_n6;
        locals.var_qm_ln_dn7 = assign20770_e38854_d_n7;
        locals.var_qm_ln_dn8 = assign20770_e38854_d_n8;
        locals.var_qm_ln_dn9 = assign20770_e38854_d_n9;
        locals.var_qm_ln_dn10 = assign20770_e38854_d_n10;
        locals.var_qm_ln_dn11 = assign20770_e38854_d_n11;
        locals.var_qm_ln_dn13 = assign20770_e38854_d_n13;
        locals.var_qm_ln_dn14 = assign20770_e38854_d_n14;

        let (assign20780_e38900, assign20780_e38900_d_n0, assign20780_e38900_d_n2, assign20780_e38900_d_n3, assign20780_e38900_d_n4, assign20780_e38900_d_n5, assign20780_e38900_d_n6, assign20780_e38900_d_n7, assign20780_e38900_d_n8, assign20780_e38900_d_n9, assign20780_e38900_d_n10, assign20780_e38900_d_n11, assign20780_e38900_d_n13, assign20780_e38900_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20780_e38858: f64 = (locals.var_f0 - locals.var_qm);
        let assign20780_e38860: f64 = (-locals.var_qm);
        let (assign20780_e38875, assign20780_e38875_d_n0, assign20780_e38875_d_n2, assign20780_e38875_d_n3, assign20780_e38875_d_n4, assign20780_e38875_d_n5, assign20780_e38875_d_n6, assign20780_e38875_d_n7, assign20780_e38875_d_n8, assign20780_e38875_d_n9, assign20780_e38875_d_n10, assign20780_e38875_d_n11, assign20780_e38875_d_n13, assign20780_e38875_d_n14,) = {
            if (!(assign20780_e38860 > 1e-38)) {
                let assign20780_e38865: f64 = (-87.498233534);
                (assign20780_e38865, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign20780_e38867: f64 = (-locals.var_qm);
                let (assign20780_e38874, assign20780_e38874_d_n0, assign20780_e38874_d_n2, assign20780_e38874_d_n3, assign20780_e38874_d_n4, assign20780_e38874_d_n5, assign20780_e38874_d_n6, assign20780_e38874_d_n7, assign20780_e38874_d_n8, assign20780_e38874_d_n9, assign20780_e38874_d_n10, assign20780_e38874_d_n11, assign20780_e38874_d_n13, assign20780_e38874_d_n14,) = {
                    if (assign20780_e38867 > 1e-38) {
                        let assign20780_e38871: f64 = (-locals.var_qm);
                        let assign20780_e38872: f64 = (assign20780_e38871).ln();
                        (assign20780_e38872, ((-locals.var_qm_dn0) / assign20780_e38871), ((-locals.var_qm_dn2) / assign20780_e38871), ((-locals.var_qm_dn3) / assign20780_e38871), ((-locals.var_qm_dn4) / assign20780_e38871), ((-locals.var_qm_dn5) / assign20780_e38871), ((-locals.var_qm_dn6) / assign20780_e38871), ((-locals.var_qm_dn7) / assign20780_e38871), ((-locals.var_qm_dn8) / assign20780_e38871), ((-locals.var_qm_dn9) / assign20780_e38871), ((-locals.var_qm_dn10) / assign20780_e38871), ((-locals.var_qm_dn11) / assign20780_e38871), ((-locals.var_qm_dn13) / assign20780_e38871), ((-locals.var_qm_dn14) / assign20780_e38871),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20780_e38874, assign20780_e38874_d_n0, assign20780_e38874_d_n2, assign20780_e38874_d_n3, assign20780_e38874_d_n4, assign20780_e38874_d_n5, assign20780_e38874_d_n6, assign20780_e38874_d_n7, assign20780_e38874_d_n8, assign20780_e38874_d_n9, assign20780_e38874_d_n10, assign20780_e38874_d_n11, assign20780_e38874_d_n13, assign20780_e38874_d_n14,)
            }
        };
        let assign20780_e38876: f64 = (assign20780_e38858 + assign20780_e38875);
        let (assign20780_e38890, assign20780_e38890_d_n0, assign20780_e38890_d_n2, assign20780_e38890_d_n3, assign20780_e38890_d_n4, assign20780_e38890_d_n5, assign20780_e38890_d_n6, assign20780_e38890_d_n7, assign20780_e38890_d_n8, assign20780_e38890_d_n9, assign20780_e38890_d_n10, assign20780_e38890_d_n11, assign20780_e38890_d_n13, assign20780_e38890_d_n14,) = {
            if (!(locals.var_t5 > 1e-38)) {
                let assign20780_e38882: f64 = (-87.498233534);
                (assign20780_e38882, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (assign20780_e38889, assign20780_e38889_d_n0, assign20780_e38889_d_n2, assign20780_e38889_d_n3, assign20780_e38889_d_n4, assign20780_e38889_d_n5, assign20780_e38889_d_n6, assign20780_e38889_d_n7, assign20780_e38889_d_n8, assign20780_e38889_d_n9, assign20780_e38889_d_n10, assign20780_e38889_d_n11, assign20780_e38889_d_n13, assign20780_e38889_d_n14,) = {
                    if (locals.var_t5 > 1e-38) {
                        let assign20780_e38887: f64 = (locals.var_t5).ln();
                        (assign20780_e38887, (locals.var_t5_dn0 / locals.var_t5), (locals.var_t5_dn2 / locals.var_t5), (locals.var_t5_dn3 / locals.var_t5), (locals.var_t5_dn4 / locals.var_t5), (locals.var_t5_dn5 / locals.var_t5), (locals.var_t5_dn6 / locals.var_t5), (locals.var_t5_dn7 / locals.var_t5), (locals.var_t5_dn8 / locals.var_t5), (locals.var_t5_dn9 / locals.var_t5), (locals.var_t5_dn10 / locals.var_t5), (locals.var_t5_dn11 / locals.var_t5), (locals.var_t5_dn13 / locals.var_t5), (locals.var_t5_dn14 / locals.var_t5),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign20780_e38889, assign20780_e38889_d_n0, assign20780_e38889_d_n2, assign20780_e38889_d_n3, assign20780_e38889_d_n4, assign20780_e38889_d_n5, assign20780_e38889_d_n6, assign20780_e38889_d_n7, assign20780_e38889_d_n8, assign20780_e38889_d_n9, assign20780_e38889_d_n10, assign20780_e38889_d_n11, assign20780_e38889_d_n13, assign20780_e38889_d_n14,)
            }
        };
        let assign20780_e38891: f64 = (assign20780_e38876 + assign20780_e38890);
        let assign20780_e38895: f64 = (0.666666667 * locals.var_qm_ln);
        let assign20780_e38896: f64 = (assign20780_e38895).exp();
        let assign20780_e38897: f64 = (locals.var_qmfactorcvfinal * assign20780_e38896);
        let assign20780_e38898: f64 = (assign20780_e38891 + assign20780_e38897);
        (assign20780_e38898, ((((locals.var_f0_dn0 - locals.var_qm_dn0) + assign20780_e38875_d_n0) + assign20780_e38890_d_n0) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn0)))), ((((locals.var_f0_dn2 - locals.var_qm_dn2) + assign20780_e38875_d_n2) + assign20780_e38890_d_n2) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn2)))), ((((locals.var_f0_dn3 - locals.var_qm_dn3) + assign20780_e38875_d_n3) + assign20780_e38890_d_n3) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn3)))), ((((locals.var_f0_dn4 - locals.var_qm_dn4) + assign20780_e38875_d_n4) + assign20780_e38890_d_n4) + ((locals.var_qmfactorcvfinal_dn4 * assign20780_e38896) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn4))))), ((((locals.var_f0_dn5 - locals.var_qm_dn5) + assign20780_e38875_d_n5) + assign20780_e38890_d_n5) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn5)))), ((((locals.var_f0_dn6 - locals.var_qm_dn6) + assign20780_e38875_d_n6) + assign20780_e38890_d_n6) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn6)))), ((((locals.var_f0_dn7 - locals.var_qm_dn7) + assign20780_e38875_d_n7) + assign20780_e38890_d_n7) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn7)))), ((((locals.var_f0_dn8 - locals.var_qm_dn8) + assign20780_e38875_d_n8) + assign20780_e38890_d_n8) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn8)))), ((((locals.var_f0_dn9 - locals.var_qm_dn9) + assign20780_e38875_d_n9) + assign20780_e38890_d_n9) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn9)))), ((((locals.var_f0_dn10 - locals.var_qm_dn10) + assign20780_e38875_d_n10) + assign20780_e38890_d_n10) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn10)))), ((((locals.var_f0_dn11 - locals.var_qm_dn11) + assign20780_e38875_d_n11) + assign20780_e38890_d_n11) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn11)))), ((((locals.var_f0_dn13 - locals.var_qm_dn13) + assign20780_e38875_d_n13) + assign20780_e38890_d_n13) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn13)))), ((((locals.var_f0_dn14 - locals.var_qm_dn14) + assign20780_e38875_d_n14) + assign20780_e38890_d_n14) + (locals.var_qmfactorcvfinal * (assign20780_e38896 * (0.666666667 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn2, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10, locals.var_e0_dn11, locals.var_e0_dn13, locals.var_e0_dn14,)
    }
};
        locals.var_e0 = assign20780_e38900;
        locals.var_e0_dn0 = assign20780_e38900_d_n0;
        locals.var_e0_dn2 = assign20780_e38900_d_n2;
        locals.var_e0_dn3 = assign20780_e38900_d_n3;
        locals.var_e0_dn4 = assign20780_e38900_d_n4;
        locals.var_e0_dn5 = assign20780_e38900_d_n5;
        locals.var_e0_dn6 = assign20780_e38900_d_n6;
        locals.var_e0_dn7 = assign20780_e38900_d_n7;
        locals.var_e0_dn8 = assign20780_e38900_d_n8;
        locals.var_e0_dn9 = assign20780_e38900_d_n9;
        locals.var_e0_dn10 = assign20780_e38900_d_n10;
        locals.var_e0_dn11 = assign20780_e38900_d_n11;
        locals.var_e0_dn13 = assign20780_e38900_d_n13;
        locals.var_e0_dn14 = assign20780_e38900_d_n14;

        let (assign20790_e38929, assign20790_e38929_d_n0, assign20790_e38929_d_n2, assign20790_e38929_d_n3, assign20790_e38929_d_n4, assign20790_e38929_d_n5, assign20790_e38929_d_n6, assign20790_e38929_d_n7, assign20790_e38929_d_n8, assign20790_e38929_d_n9, assign20790_e38929_d_n10, assign20790_e38929_d_n11, assign20790_e38929_d_n13, assign20790_e38929_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20790_e38903: f64 = (-1.0);
        let assign20790_e38906: f64 = (1.0 / locals.var_qm);
        let assign20790_e38907: f64 = (assign20790_e38903 + assign20790_e38906);
        let assign20790_e38910: f64 = (2.0 / locals.var_t8);
        let assign20790_e38912: f64 = (assign20790_e38910 - locals.var_t4);
        let assign20790_e38914: f64 = (assign20790_e38912 - 1.0);
        let assign20790_e38916: f64 = (assign20790_e38914 * locals.var_rc);
        let assign20790_e38917: f64 = (assign20790_e38907 + assign20790_e38916);
        let assign20790_e38920: f64 = (0.666666667 * locals.var_qmfactorcvfinal);
        let assign20790_e38922: f64 = (-0.333333333);
        let assign20790_e38924: f64 = (assign20790_e38922 * locals.var_qm_ln);
        let assign20790_e38925: f64 = (assign20790_e38924).exp();
        let assign20790_e38926: f64 = (assign20790_e38920 * assign20790_e38925);
        let assign20790_e38927: f64 = (assign20790_e38917 - assign20790_e38926);
        (assign20790_e38927, (((-(locals.var_qm_dn0 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn0) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn0) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn0)))), (((-(locals.var_qm_dn2 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn2) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn2) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn2)))), (((-(locals.var_qm_dn3 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn3) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn3) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn3)))), (((-(locals.var_qm_dn4 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn4) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn4) * locals.var_rc)) - (((0.666666667 * locals.var_qmfactorcvfinal_dn4) * assign20790_e38925) + (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn4))))), (((-(locals.var_qm_dn5 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn5) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn5) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn5)))), (((-(locals.var_qm_dn6 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn6) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn6) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn6)))), (((-(locals.var_qm_dn7 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn7) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn7) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn7)))), (((-(locals.var_qm_dn8 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn8) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn8) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn8)))), (((-(locals.var_qm_dn9 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn9) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn9) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn9)))), (((-(locals.var_qm_dn10 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn10) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn10) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn10)))), (((-(locals.var_qm_dn11 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn11) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn11) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn11)))), (((-(locals.var_qm_dn13 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn13) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn13) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn13)))), (((-(locals.var_qm_dn14 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn14) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn14) * locals.var_rc)) - (assign20790_e38920 * (assign20790_e38925 * (assign20790_e38922 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn3, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn13, locals.var_e1_dn14,)
    }
};
        locals.var_e1 = assign20790_e38929;
        locals.var_e1_dn0 = assign20790_e38929_d_n0;
        locals.var_e1_dn2 = assign20790_e38929_d_n2;
        locals.var_e1_dn3 = assign20790_e38929_d_n3;
        locals.var_e1_dn4 = assign20790_e38929_d_n4;
        locals.var_e1_dn5 = assign20790_e38929_d_n5;
        locals.var_e1_dn6 = assign20790_e38929_d_n6;
        locals.var_e1_dn7 = assign20790_e38929_d_n7;
        locals.var_e1_dn8 = assign20790_e38929_d_n8;
        locals.var_e1_dn9 = assign20790_e38929_d_n9;
        locals.var_e1_dn10 = assign20790_e38929_d_n10;
        locals.var_e1_dn11 = assign20790_e38929_d_n11;
        locals.var_e1_dn13 = assign20790_e38929_d_n13;
        locals.var_e1_dn14 = assign20790_e38929_d_n14;

        let (assign20800_e38950, assign20800_e38950_d_n0, assign20800_e38950_d_n2, assign20800_e38950_d_n3, assign20800_e38950_d_n4, assign20800_e38950_d_n5, assign20800_e38950_d_n6, assign20800_e38950_d_n7, assign20800_e38950_d_n8, assign20800_e38950_d_n9, assign20800_e38950_d_n10, assign20800_e38950_d_n11, assign20800_e38950_d_n13, assign20800_e38950_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20800_e38932: f64 = (-1.0);
        let assign20800_e38935: f64 = (locals.var_qm * locals.var_qm);
        let assign20800_e38936: f64 = (assign20800_e38932 / assign20800_e38935);
        let assign20800_e38939: f64 = (2.0 / 9.0);
        let assign20800_e38941: f64 = (assign20800_e38939 * locals.var_qmfactorcvfinal);
        let assign20800_e38943: f64 = (-1.333333333);
        let assign20800_e38945: f64 = (assign20800_e38943 * locals.var_qm_ln);
        let assign20800_e38946: f64 = (assign20800_e38945).exp();
        let assign20800_e38947: f64 = (assign20800_e38941 * assign20800_e38946);
        let assign20800_e38948: f64 = (assign20800_e38936 - assign20800_e38947);
        (assign20800_e38948, ((-((assign20800_e38932 * ((locals.var_qm_dn0 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn0))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn0)))), ((-((assign20800_e38932 * ((locals.var_qm_dn2 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn2))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn2)))), ((-((assign20800_e38932 * ((locals.var_qm_dn3 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn3))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn3)))), ((-((assign20800_e38932 * ((locals.var_qm_dn4 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn4))) / (assign20800_e38935 * assign20800_e38935))) - (((assign20800_e38939 * locals.var_qmfactorcvfinal_dn4) * assign20800_e38946) + (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn4))))), ((-((assign20800_e38932 * ((locals.var_qm_dn5 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn5))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn5)))), ((-((assign20800_e38932 * ((locals.var_qm_dn6 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn6))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn6)))), ((-((assign20800_e38932 * ((locals.var_qm_dn7 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn7))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn7)))), ((-((assign20800_e38932 * ((locals.var_qm_dn8 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn8))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn8)))), ((-((assign20800_e38932 * ((locals.var_qm_dn9 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn9))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn9)))), ((-((assign20800_e38932 * ((locals.var_qm_dn10 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn10))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn10)))), ((-((assign20800_e38932 * ((locals.var_qm_dn11 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn11))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn11)))), ((-((assign20800_e38932 * ((locals.var_qm_dn13 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn13))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn13)))), ((-((assign20800_e38932 * ((locals.var_qm_dn14 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn14))) / (assign20800_e38935 * assign20800_e38935))) - (assign20800_e38941 * (assign20800_e38946 * (assign20800_e38943 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e2, locals.var_e2_dn0, locals.var_e2_dn2, locals.var_e2_dn3, locals.var_e2_dn4, locals.var_e2_dn5, locals.var_e2_dn6, locals.var_e2_dn7, locals.var_e2_dn8, locals.var_e2_dn9, locals.var_e2_dn10, locals.var_e2_dn11, locals.var_e2_dn13, locals.var_e2_dn14,)
    }
};
        locals.var_e2 = assign20800_e38950;
        locals.var_e2_dn0 = assign20800_e38950_d_n0;
        locals.var_e2_dn2 = assign20800_e38950_d_n2;
        locals.var_e2_dn3 = assign20800_e38950_d_n3;
        locals.var_e2_dn4 = assign20800_e38950_d_n4;
        locals.var_e2_dn5 = assign20800_e38950_d_n5;
        locals.var_e2_dn6 = assign20800_e38950_d_n6;
        locals.var_e2_dn7 = assign20800_e38950_d_n7;
        locals.var_e2_dn8 = assign20800_e38950_d_n8;
        locals.var_e2_dn9 = assign20800_e38950_d_n9;
        locals.var_e2_dn10 = assign20800_e38950_d_n10;
        locals.var_e2_dn11 = assign20800_e38950_d_n11;
        locals.var_e2_dn13 = assign20800_e38950_d_n13;
        locals.var_e2_dn14 = assign20800_e38950_d_n14;

        let (assign20810_e38970, assign20810_e38970_d_n0, assign20810_e38970_d_n2, assign20810_e38970_d_n3, assign20810_e38970_d_n4, assign20810_e38970_d_n5, assign20810_e38970_d_n6, assign20810_e38970_d_n7, assign20810_e38970_d_n8, assign20810_e38970_d_n9, assign20810_e38970_d_n10, assign20810_e38970_d_n11, assign20810_e38970_d_n13, assign20810_e38970_d_n14,) = {
    if (locals.var_guard367 != 0.0) {
        let assign20810_e38955: f64 = (locals.var_e0 / locals.var_e1);
        let assign20810_e38959: f64 = (locals.var_e0 * locals.var_e2);
        let assign20810_e38962: f64 = (2.0 * locals.var_e1);
        let assign20810_e38964: f64 = (assign20810_e38962 * locals.var_e1);
        let assign20810_e38965: f64 = (assign20810_e38959 / assign20810_e38964);
        let assign20810_e38966: f64 = (1.0 + assign20810_e38965);
        let assign20810_e38967: f64 = (assign20810_e38955 * assign20810_e38966);
        let assign20810_e38968: f64 = (locals.var_qm - assign20810_e38967);
        (assign20810_e38968, (locals.var_qm_dn0 - (((((locals.var_e0_dn0 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn0)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn0 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn0)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn0) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn0)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn2 - (((((locals.var_e0_dn2 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn2)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn2 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn2)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn2) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn2)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn3 - (((((locals.var_e0_dn3 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn3)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn3 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn3)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn3) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn3)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn4 - (((((locals.var_e0_dn4 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn4)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn4 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn4)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn4) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn4)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn5 - (((((locals.var_e0_dn5 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn5)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn5 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn5)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn5) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn5)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn6 - (((((locals.var_e0_dn6 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn6)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn6 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn6)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn6) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn6)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn7 - (((((locals.var_e0_dn7 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn7)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn7 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn7)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn7) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn7)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn8 - (((((locals.var_e0_dn8 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn8)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn8 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn8)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn8) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn8)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn9 - (((((locals.var_e0_dn9 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn9)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn9 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn9)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn9) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn9)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn10 - (((((locals.var_e0_dn10 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn10)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn10 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn10)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn10) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn10)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn11 - (((((locals.var_e0_dn11 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn11)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn11 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn11)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn11) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn11)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn13 - (((((locals.var_e0_dn13 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn13)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn13 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn13)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn13) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn13)))) / (assign20810_e38964 * assign20810_e38964))))), (locals.var_qm_dn14 - (((((locals.var_e0_dn14 * locals.var_e1) - (locals.var_e0 * locals.var_e1_dn14)) / (locals.var_e1 * locals.var_e1)) * assign20810_e38966) + (assign20810_e38955 * (((((locals.var_e0_dn14 * locals.var_e2) + (locals.var_e0 * locals.var_e2_dn14)) * assign20810_e38964) - (assign20810_e38959 * (((2.0 * locals.var_e1_dn14) * locals.var_e1) + (assign20810_e38962 * locals.var_e1_dn14)))) / (assign20810_e38964 * assign20810_e38964))))),)
    } else {
        (locals.var_qm, locals.var_qm_dn0, locals.var_qm_dn2, locals.var_qm_dn3, locals.var_qm_dn4, locals.var_qm_dn5, locals.var_qm_dn6, locals.var_qm_dn7, locals.var_qm_dn8, locals.var_qm_dn9, locals.var_qm_dn10, locals.var_qm_dn11, locals.var_qm_dn13, locals.var_qm_dn14,)
    }
};
        locals.var_qm = assign20810_e38970;
        locals.var_qm_dn0 = assign20810_e38970_d_n0;
        locals.var_qm_dn2 = assign20810_e38970_d_n2;
        locals.var_qm_dn3 = assign20810_e38970_d_n3;
        locals.var_qm_dn4 = assign20810_e38970_d_n4;
        locals.var_qm_dn5 = assign20810_e38970_d_n5;
        locals.var_qm_dn6 = assign20810_e38970_d_n6;
        locals.var_qm_dn7 = assign20810_e38970_d_n7;
        locals.var_qm_dn8 = assign20810_e38970_d_n8;
        locals.var_qm_dn9 = assign20810_e38970_d_n9;
        locals.var_qm_dn10 = assign20810_e38970_d_n10;
        locals.var_qm_dn11 = assign20810_e38970_d_n11;
        locals.var_qm_dn13 = assign20810_e38970_d_n13;
        locals.var_qm_dn14 = assign20810_e38970_d_n14;

        let (assign20820_e38978, assign20820_e38978_d_n0, assign20820_e38978_d_n2, assign20820_e38978_d_n3, assign20820_e38978_d_n4, assign20820_e38978_d_n5, assign20820_e38978_d_n6, assign20820_e38978_d_n7, assign20820_e38978_d_n8, assign20820_e38978_d_n9, assign20820_e38978_d_n10, assign20820_e38978_d_n11, assign20820_e38978_d_n13, assign20820_e38978_d_n14,) = {
    if (locals.var_guard367 == 0.0) {
        let assign20820_e38974: f64 = (-locals.var_qm);
        let assign20820_e38976: f64 = (assign20820_e38974 * locals.var_qm);
        (assign20820_e38976, (((-locals.var_qm_dn0) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn0)), (((-locals.var_qm_dn2) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn2)), (((-locals.var_qm_dn3) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn3)), (((-locals.var_qm_dn4) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn4)), (((-locals.var_qm_dn5) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn5)), (((-locals.var_qm_dn6) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn6)), (((-locals.var_qm_dn7) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn7)), (((-locals.var_qm_dn8) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn8)), (((-locals.var_qm_dn9) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn9)), (((-locals.var_qm_dn10) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn10)), (((-locals.var_qm_dn11) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn11)), (((-locals.var_qm_dn13) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn13)), (((-locals.var_qm_dn14) * locals.var_qm) + (assign20820_e38974 * locals.var_qm_dn14)),)
    } else {
        (locals.var_qm, locals.var_qm_dn0, locals.var_qm_dn2, locals.var_qm_dn3, locals.var_qm_dn4, locals.var_qm_dn5, locals.var_qm_dn6, locals.var_qm_dn7, locals.var_qm_dn8, locals.var_qm_dn9, locals.var_qm_dn10, locals.var_qm_dn11, locals.var_qm_dn13, locals.var_qm_dn14,)
    }
};
        locals.var_qm = assign20820_e38978;
        locals.var_qm_dn0 = assign20820_e38978_d_n0;
        locals.var_qm_dn2 = assign20820_e38978_d_n2;
        locals.var_qm_dn3 = assign20820_e38978_d_n3;
        locals.var_qm_dn4 = assign20820_e38978_d_n4;
        locals.var_qm_dn5 = assign20820_e38978_d_n5;
        locals.var_qm_dn6 = assign20820_e38978_d_n6;
        locals.var_qm_dn7 = assign20820_e38978_d_n7;
        locals.var_qm_dn8 = assign20820_e38978_d_n8;
        locals.var_qm_dn9 = assign20820_e38978_d_n9;
        locals.var_qm_dn10 = assign20820_e38978_d_n10;
        locals.var_qm_dn11 = assign20820_e38978_d_n11;
        locals.var_qm_dn13 = assign20820_e38978_d_n13;
        locals.var_qm_dn14 = assign20820_e38978_d_n14;

        let assign20830_e38980: f64 = (-locals.var_qm);
        let assign20830_e38982: f64 = (assign20830_e38980 * locals.var_nvtm);
        locals.var_qis = assign20830_e38982;
        locals.var_qis_dn0 = (((-locals.var_qm_dn0) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn0));
        locals.var_qis_dn2 = (((-locals.var_qm_dn2) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn2));
        locals.var_qis_dn3 = (((-locals.var_qm_dn3) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn3));
        locals.var_qis_dn4 = (((-locals.var_qm_dn4) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn4));
        locals.var_qis_dn5 = (((-locals.var_qm_dn5) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn5));
        locals.var_qis_dn6 = (((-locals.var_qm_dn6) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn6));
        locals.var_qis_dn7 = (((-locals.var_qm_dn7) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn7));
        locals.var_qis_dn8 = (((-locals.var_qm_dn8) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn8));
        locals.var_qis_dn9 = (((-locals.var_qm_dn9) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn9));
        locals.var_qis_dn10 = (((-locals.var_qm_dn10) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn10));
        locals.var_qis_dn11 = (((-locals.var_qm_dn11) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn11));
        locals.var_qis_dn13 = (((-locals.var_qm_dn13) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn13));
        locals.var_qis_dn14 = (((-locals.var_qm_dn14) * locals.var_nvtm) + (assign20830_e38980 * locals.var_nvtm_dn14));

        let assign20840_e38985: f64 = if p.p57 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard368 = assign20840_e38985;

        let (assign20850_e38993, assign20850_e38993_d_n0, assign20850_e38993_d_n2, assign20850_e38993_d_n3, assign20850_e38993_d_n4, assign20850_e38993_d_n5, assign20850_e38993_d_n6, assign20850_e38993_d_n7, assign20850_e38993_d_n8, assign20850_e38993_d_n9, assign20850_e38993_d_n10, assign20850_e38993_d_n11, assign20850_e38993_d_n13, assign20850_e38993_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20850_e38989: f64 = (locals.var_vgsfb - locals.var_vch);
        let assign20850_e38991: f64 = (assign20850_e38989 / locals.var_nvtm);
        (assign20850_e38991, ((((locals.var_vgsfb_dn0 - locals.var_vch_dn0) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn0)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn2 - locals.var_vch_dn2) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn2)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn3 - locals.var_vch_dn3) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn4 - locals.var_vch_dn4) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn5 - locals.var_vch_dn5) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn6 - locals.var_vch_dn6) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn7 - locals.var_vch_dn7) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn8 - locals.var_vch_dn8) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn9 - locals.var_vch_dn9) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn9)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn10 - locals.var_vch_dn10) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn11 - locals.var_vch_dn11) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn11)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn13 - locals.var_vch_dn13) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn13)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn14 - locals.var_vch_dn14) * locals.var_nvtm) - (assign20850_e38989 * locals.var_nvtm_dn14)) / (locals.var_nvtm * locals.var_nvtm)),)
    } else {
        (locals.var_qt2, locals.var_qt2_dn0, locals.var_qt2_dn2, locals.var_qt2_dn3, locals.var_qt2_dn4, locals.var_qt2_dn5, locals.var_qt2_dn6, locals.var_qt2_dn7, locals.var_qt2_dn8, locals.var_qt2_dn9, locals.var_qt2_dn10, locals.var_qt2_dn11, locals.var_qt2_dn13, locals.var_qt2_dn14,)
    }
};
        locals.var_qt2 = assign20850_e38993;
        locals.var_qt2_dn0 = assign20850_e38993_d_n0;
        locals.var_qt2_dn2 = assign20850_e38993_d_n2;
        locals.var_qt2_dn3 = assign20850_e38993_d_n3;
        locals.var_qt2_dn4 = assign20850_e38993_d_n4;
        locals.var_qt2_dn5 = assign20850_e38993_d_n5;
        locals.var_qt2_dn6 = assign20850_e38993_d_n6;
        locals.var_qt2_dn7 = assign20850_e38993_d_n7;
        locals.var_qt2_dn8 = assign20850_e38993_d_n8;
        locals.var_qt2_dn9 = assign20850_e38993_d_n9;
        locals.var_qt2_dn10 = assign20850_e38993_d_n10;
        locals.var_qt2_dn11 = assign20850_e38993_d_n11;
        locals.var_qt2_dn13 = assign20850_e38993_d_n13;
        locals.var_qt2_dn14 = assign20850_e38993_d_n14;

        let (assign20860_e39016, assign20860_e39016_d_n0, assign20860_e39016_d_n2, assign20860_e39016_d_n3, assign20860_e39016_d_n4, assign20860_e39016_d_n5, assign20860_e39016_d_n6, assign20860_e39016_d_n7, assign20860_e39016_d_n8, assign20860_e39016_d_n9, assign20860_e39016_d_n10, assign20860_e39016_d_n11, assign20860_e39016_d_n13, assign20860_e39016_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20860_e38998: f64 = locals.var_qt2;
        let assign20860_e39001: f64 = locals.var_qt2;
        let assign20860_e39004: f64 = locals.var_qt2;
        let assign20860_e39005: f64 = (assign20860_e39001 * assign20860_e39004);
        let assign20860_e39008: f64 = (0.25 * locals.var_p1);
        let assign20860_e39010: f64 = (assign20860_e39008 * locals.var_p1);
        let assign20860_e39011: f64 = (assign20860_e39005 + assign20860_e39010);
        let assign20860_e39012: f64 = (assign20860_e39011).sqrt();
        let assign20860_e39013: f64 = (assign20860_e38998 + assign20860_e39012);
        let assign20860_e39014: f64 = (0.5 * assign20860_e39013);
        (assign20860_e39014, (0.5 * (locals.var_qt2_dn0 + (((locals.var_qt2_dn0 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn0)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn2 + (((locals.var_qt2_dn2 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn2)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn3 + (((locals.var_qt2_dn3 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn3)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn4 + (((locals.var_qt2_dn4 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn4)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn5 + (((locals.var_qt2_dn5 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn5)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn6 + (((locals.var_qt2_dn6 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn6)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn7 + (((locals.var_qt2_dn7 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn7)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn8 + (((locals.var_qt2_dn8 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn8)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn9 + (((locals.var_qt2_dn9 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn9)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn10 + (((locals.var_qt2_dn10 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn10)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn11 + (((locals.var_qt2_dn11 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn11)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn13 + (((locals.var_qt2_dn13 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn13)) / (2.0 * assign20860_e39012)))), (0.5 * (locals.var_qt2_dn14 + (((locals.var_qt2_dn14 * assign20860_e39004) + (assign20860_e39001 * locals.var_qt2_dn14)) / (2.0 * assign20860_e39012)))),)
    } else {
        (locals.var_qt3, locals.var_qt3_dn0, locals.var_qt3_dn2, locals.var_qt3_dn3, locals.var_qt3_dn4, locals.var_qt3_dn5, locals.var_qt3_dn6, locals.var_qt3_dn7, locals.var_qt3_dn8, locals.var_qt3_dn9, locals.var_qt3_dn10, locals.var_qt3_dn11, locals.var_qt3_dn13, locals.var_qt3_dn14,)
    }
};
        locals.var_qt3 = assign20860_e39016;
        locals.var_qt3_dn0 = assign20860_e39016_d_n0;
        locals.var_qt3_dn2 = assign20860_e39016_d_n2;
        locals.var_qt3_dn3 = assign20860_e39016_d_n3;
        locals.var_qt3_dn4 = assign20860_e39016_d_n4;
        locals.var_qt3_dn5 = assign20860_e39016_d_n5;
        locals.var_qt3_dn6 = assign20860_e39016_d_n6;
        locals.var_qt3_dn7 = assign20860_e39016_d_n7;
        locals.var_qt3_dn8 = assign20860_e39016_d_n8;
        locals.var_qt3_dn9 = assign20860_e39016_d_n9;
        locals.var_qt3_dn10 = assign20860_e39016_d_n10;
        locals.var_qt3_dn11 = assign20860_e39016_d_n11;
        locals.var_qt3_dn13 = assign20860_e39016_d_n13;
        locals.var_qt3_dn14 = assign20860_e39016_d_n14;

        let (assign20870_e39024, assign20870_e39024_d_n0, assign20870_e39024_d_n2, assign20870_e39024_d_n3, assign20870_e39024_d_n4, assign20870_e39024_d_n5, assign20870_e39024_d_n6, assign20870_e39024_d_n7, assign20870_e39024_d_n8, assign20870_e39024_d_n9, assign20870_e39024_d_n10, assign20870_e39024_d_n11, assign20870_e39024_d_n13, assign20870_e39024_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20870_e39021: f64 = (locals.var_d1 / 2.0);
        let assign20870_e39022: f64 = (locals.var_qt3).powf(assign20870_e39021);
        (assign20870_e39022, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn0)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn0 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn2)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn2 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn3)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn3 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn4)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn4 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn5)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn5 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn6)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn6 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn7)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn7 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn8)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn8 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn9)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn9 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn10)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn10 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn11)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn11 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn13)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn13 / locals.var_qt3))) }, if 0.0 == 0.0 && ((assign20870_e39021) as f64).is_finite() && ((assign20870_e39021) as f64).fract() == 0.0 { if assign20870_e39021 == 0.0 { 0.0 } else { (assign20870_e39021 * ((locals.var_qt3).powf(assign20870_e39021 - 1.0) * locals.var_qt3_dn14)) } } else { (assign20870_e39022 * (assign20870_e39021 * (locals.var_qt3_dn14 / locals.var_qt3))) },)
    } else {
        (locals.var_qt4, locals.var_qt4_dn0, locals.var_qt4_dn2, locals.var_qt4_dn3, locals.var_qt4_dn4, locals.var_qt4_dn5, locals.var_qt4_dn6, locals.var_qt4_dn7, locals.var_qt4_dn8, locals.var_qt4_dn9, locals.var_qt4_dn10, locals.var_qt4_dn11, locals.var_qt4_dn13, locals.var_qt4_dn14,)
    }
};
        locals.var_qt4 = assign20870_e39024;
        locals.var_qt4_dn0 = assign20870_e39024_d_n0;
        locals.var_qt4_dn2 = assign20870_e39024_d_n2;
        locals.var_qt4_dn3 = assign20870_e39024_d_n3;
        locals.var_qt4_dn4 = assign20870_e39024_d_n4;
        locals.var_qt4_dn5 = assign20870_e39024_d_n5;
        locals.var_qt4_dn6 = assign20870_e39024_d_n6;
        locals.var_qt4_dn7 = assign20870_e39024_d_n7;
        locals.var_qt4_dn8 = assign20870_e39024_d_n8;
        locals.var_qt4_dn9 = assign20870_e39024_d_n9;
        locals.var_qt4_dn10 = assign20870_e39024_d_n10;
        locals.var_qt4_dn11 = assign20870_e39024_d_n11;
        locals.var_qt4_dn13 = assign20870_e39024_d_n13;
        locals.var_qt4_dn14 = assign20870_e39024_d_n14;

        let (assign20880_e39035, assign20880_e39035_d_n0, assign20880_e39035_d_n2, assign20880_e39035_d_n3, assign20880_e39035_d_n4, assign20880_e39035_d_n5, assign20880_e39035_d_n6, assign20880_e39035_d_n7, assign20880_e39035_d_n8, assign20880_e39035_d_n9, assign20880_e39035_d_n10, assign20880_e39035_d_n11, assign20880_e39035_d_n13, assign20880_e39035_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20880_e39028: f64 = (locals.var_qnd10 * locals.var_qt4);
        let assign20880_e39031: f64 = (locals.var_qt2 - locals.var_qt3);
        let assign20880_e39032: f64 = { let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign20880_e39033: f64 = (assign20880_e39028 * assign20880_e39032);
        (assign20880_e39033, (((locals.var_qnd10 * locals.var_qt4_dn0) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn0 - locals.var_qt3_dn0)))), (((locals.var_qnd10 * locals.var_qt4_dn2) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn2 - locals.var_qt3_dn2)))), (((locals.var_qnd10 * locals.var_qt4_dn3) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn3 - locals.var_qt3_dn3)))), (((locals.var_qnd10 * locals.var_qt4_dn4) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn4 - locals.var_qt3_dn4)))), (((locals.var_qnd10 * locals.var_qt4_dn5) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn5 - locals.var_qt3_dn5)))), (((locals.var_qnd10 * locals.var_qt4_dn6) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn6 - locals.var_qt3_dn6)))), (((locals.var_qnd10 * locals.var_qt4_dn7) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn7 - locals.var_qt3_dn7)))), (((locals.var_qnd10 * locals.var_qt4_dn8) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn8 - locals.var_qt3_dn8)))), (((locals.var_qnd10 * locals.var_qt4_dn9) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn9 - locals.var_qt3_dn9)))), (((locals.var_qnd10 * locals.var_qt4_dn10) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn10 - locals.var_qt3_dn10)))), (((locals.var_qnd10 * locals.var_qt4_dn11) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn11 - locals.var_qt3_dn11)))), (((locals.var_qnd10 * locals.var_qt4_dn13) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn13 - locals.var_qt3_dn13)))), (((locals.var_qnd10 * locals.var_qt4_dn14) * assign20880_e39032) + (assign20880_e39028 * ({ let limited_exp_arg = assign20880_e39031; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt2_dn14 - locals.var_qt3_dn14)))),)
    } else {
        (locals.var_qnds1, locals.var_qnds1_dn0, locals.var_qnds1_dn2, locals.var_qnds1_dn3, locals.var_qnds1_dn4, locals.var_qnds1_dn5, locals.var_qnds1_dn6, locals.var_qnds1_dn7, locals.var_qnds1_dn8, locals.var_qnds1_dn9, locals.var_qnds1_dn10, locals.var_qnds1_dn11, locals.var_qnds1_dn13, locals.var_qnds1_dn14,)
    }
};
        locals.var_qnds1 = assign20880_e39035;
        locals.var_qnds1_dn0 = assign20880_e39035_d_n0;
        locals.var_qnds1_dn2 = assign20880_e39035_d_n2;
        locals.var_qnds1_dn3 = assign20880_e39035_d_n3;
        locals.var_qnds1_dn4 = assign20880_e39035_d_n4;
        locals.var_qnds1_dn5 = assign20880_e39035_d_n5;
        locals.var_qnds1_dn6 = assign20880_e39035_d_n6;
        locals.var_qnds1_dn7 = assign20880_e39035_d_n7;
        locals.var_qnds1_dn8 = assign20880_e39035_d_n8;
        locals.var_qnds1_dn9 = assign20880_e39035_d_n9;
        locals.var_qnds1_dn10 = assign20880_e39035_d_n10;
        locals.var_qnds1_dn11 = assign20880_e39035_d_n11;
        locals.var_qnds1_dn13 = assign20880_e39035_d_n13;
        locals.var_qnds1_dn14 = assign20880_e39035_d_n14;

        let (assign20890_e39045, assign20890_e39045_d_n0, assign20890_e39045_d_n2, assign20890_e39045_d_n3, assign20890_e39045_d_n4, assign20890_e39045_d_n5, assign20890_e39045_d_n6, assign20890_e39045_d_n7, assign20890_e39045_d_n8, assign20890_e39045_d_n9, assign20890_e39045_d_n10, assign20890_e39045_d_n11, assign20890_e39045_d_n13, assign20890_e39045_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20890_e39039: f64 = (locals.var_vgsfb - locals.var_vch);
        let assign20890_e39041: f64 = (assign20890_e39039 - locals.var_qe2);
        let assign20890_e39043: f64 = (assign20890_e39041 / locals.var_nvtm);
        (assign20890_e39043, (((((locals.var_vgsfb_dn0 - locals.var_vch_dn0) - locals.var_qe2_dn0) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn0)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn2 - locals.var_vch_dn2) - locals.var_qe2_dn2) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn2)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn3 - locals.var_vch_dn3) - locals.var_qe2_dn3) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn4 - locals.var_vch_dn4) - locals.var_qe2_dn4) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn5 - locals.var_vch_dn5) - locals.var_qe2_dn5) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn6 - locals.var_vch_dn6) - locals.var_qe2_dn6) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn7 - locals.var_vch_dn7) - locals.var_qe2_dn7) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn8 - locals.var_vch_dn8) - locals.var_qe2_dn8) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn9 - locals.var_vch_dn9) - locals.var_qe2_dn9) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn9)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn10 - locals.var_vch_dn10) - locals.var_qe2_dn10) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn11 - locals.var_vch_dn11) - locals.var_qe2_dn11) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn11)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn13 - locals.var_vch_dn13) - locals.var_qe2_dn13) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn13)) / (locals.var_nvtm * locals.var_nvtm)), (((((locals.var_vgsfb_dn14 - locals.var_vch_dn14) - locals.var_qe2_dn14) * locals.var_nvtm) - (assign20890_e39041 * locals.var_nvtm_dn14)) / (locals.var_nvtm * locals.var_nvtm)),)
    } else {
        (locals.var_qt5, locals.var_qt5_dn0, locals.var_qt5_dn2, locals.var_qt5_dn3, locals.var_qt5_dn4, locals.var_qt5_dn5, locals.var_qt5_dn6, locals.var_qt5_dn7, locals.var_qt5_dn8, locals.var_qt5_dn9, locals.var_qt5_dn10, locals.var_qt5_dn11, locals.var_qt5_dn13, locals.var_qt5_dn14,)
    }
};
        locals.var_qt5 = assign20890_e39045;
        locals.var_qt5_dn0 = assign20890_e39045_d_n0;
        locals.var_qt5_dn2 = assign20890_e39045_d_n2;
        locals.var_qt5_dn3 = assign20890_e39045_d_n3;
        locals.var_qt5_dn4 = assign20890_e39045_d_n4;
        locals.var_qt5_dn5 = assign20890_e39045_d_n5;
        locals.var_qt5_dn6 = assign20890_e39045_d_n6;
        locals.var_qt5_dn7 = assign20890_e39045_d_n7;
        locals.var_qt5_dn8 = assign20890_e39045_d_n8;
        locals.var_qt5_dn9 = assign20890_e39045_d_n9;
        locals.var_qt5_dn10 = assign20890_e39045_d_n10;
        locals.var_qt5_dn11 = assign20890_e39045_d_n11;
        locals.var_qt5_dn13 = assign20890_e39045_d_n13;
        locals.var_qt5_dn14 = assign20890_e39045_d_n14;

        let (assign20900_e39068, assign20900_e39068_d_n0, assign20900_e39068_d_n2, assign20900_e39068_d_n3, assign20900_e39068_d_n4, assign20900_e39068_d_n5, assign20900_e39068_d_n6, assign20900_e39068_d_n7, assign20900_e39068_d_n8, assign20900_e39068_d_n9, assign20900_e39068_d_n10, assign20900_e39068_d_n11, assign20900_e39068_d_n13, assign20900_e39068_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20900_e39050: f64 = locals.var_qt5;
        let assign20900_e39053: f64 = locals.var_qt5;
        let assign20900_e39056: f64 = locals.var_qt5;
        let assign20900_e39057: f64 = (assign20900_e39053 * assign20900_e39056);
        let assign20900_e39060: f64 = (0.25 * locals.var_p2);
        let assign20900_e39062: f64 = (assign20900_e39060 * locals.var_p2);
        let assign20900_e39063: f64 = (assign20900_e39057 + assign20900_e39062);
        let assign20900_e39064: f64 = (assign20900_e39063).sqrt();
        let assign20900_e39065: f64 = (assign20900_e39050 + assign20900_e39064);
        let assign20900_e39066: f64 = (0.5 * assign20900_e39065);
        (assign20900_e39066, (0.5 * (locals.var_qt5_dn0 + (((locals.var_qt5_dn0 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn0)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn2 + (((locals.var_qt5_dn2 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn2)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn3 + (((locals.var_qt5_dn3 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn3)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn4 + (((locals.var_qt5_dn4 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn4)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn5 + (((locals.var_qt5_dn5 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn5)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn6 + (((locals.var_qt5_dn6 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn6)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn7 + (((locals.var_qt5_dn7 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn7)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn8 + (((locals.var_qt5_dn8 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn8)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn9 + (((locals.var_qt5_dn9 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn9)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn10 + (((locals.var_qt5_dn10 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn10)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn11 + (((locals.var_qt5_dn11 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn11)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn13 + (((locals.var_qt5_dn13 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn13)) / (2.0 * assign20900_e39064)))), (0.5 * (locals.var_qt5_dn14 + (((locals.var_qt5_dn14 * assign20900_e39056) + (assign20900_e39053 * locals.var_qt5_dn14)) / (2.0 * assign20900_e39064)))),)
    } else {
        (locals.var_qt6, locals.var_qt6_dn0, locals.var_qt6_dn2, locals.var_qt6_dn3, locals.var_qt6_dn4, locals.var_qt6_dn5, locals.var_qt6_dn6, locals.var_qt6_dn7, locals.var_qt6_dn8, locals.var_qt6_dn9, locals.var_qt6_dn10, locals.var_qt6_dn11, locals.var_qt6_dn13, locals.var_qt6_dn14,)
    }
};
        locals.var_qt6 = assign20900_e39068;
        locals.var_qt6_dn0 = assign20900_e39068_d_n0;
        locals.var_qt6_dn2 = assign20900_e39068_d_n2;
        locals.var_qt6_dn3 = assign20900_e39068_d_n3;
        locals.var_qt6_dn4 = assign20900_e39068_d_n4;
        locals.var_qt6_dn5 = assign20900_e39068_d_n5;
        locals.var_qt6_dn6 = assign20900_e39068_d_n6;
        locals.var_qt6_dn7 = assign20900_e39068_d_n7;
        locals.var_qt6_dn8 = assign20900_e39068_d_n8;
        locals.var_qt6_dn9 = assign20900_e39068_d_n9;
        locals.var_qt6_dn10 = assign20900_e39068_d_n10;
        locals.var_qt6_dn11 = assign20900_e39068_d_n11;
        locals.var_qt6_dn13 = assign20900_e39068_d_n13;
        locals.var_qt6_dn14 = assign20900_e39068_d_n14;

        let (assign20910_e39076, assign20910_e39076_d_n0, assign20910_e39076_d_n2, assign20910_e39076_d_n3, assign20910_e39076_d_n4, assign20910_e39076_d_n5, assign20910_e39076_d_n6, assign20910_e39076_d_n7, assign20910_e39076_d_n8, assign20910_e39076_d_n9, assign20910_e39076_d_n10, assign20910_e39076_d_n11, assign20910_e39076_d_n13, assign20910_e39076_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20910_e39073: f64 = (locals.var_d2 / 2.0);
        let assign20910_e39074: f64 = (locals.var_qt6).powf(assign20910_e39073);
        (assign20910_e39074, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn0)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn0 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn2)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn2 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn3)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn3 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn4)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn4 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn5)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn5 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn6)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn6 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn7)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn7 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn8)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn8 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn9)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn9 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn10)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn10 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn11)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn11 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn13)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn13 / locals.var_qt6))) }, if 0.0 == 0.0 && ((assign20910_e39073) as f64).is_finite() && ((assign20910_e39073) as f64).fract() == 0.0 { if assign20910_e39073 == 0.0 { 0.0 } else { (assign20910_e39073 * ((locals.var_qt6).powf(assign20910_e39073 - 1.0) * locals.var_qt6_dn14)) } } else { (assign20910_e39074 * (assign20910_e39073 * (locals.var_qt6_dn14 / locals.var_qt6))) },)
    } else {
        (locals.var_qt7, locals.var_qt7_dn0, locals.var_qt7_dn2, locals.var_qt7_dn3, locals.var_qt7_dn4, locals.var_qt7_dn5, locals.var_qt7_dn6, locals.var_qt7_dn7, locals.var_qt7_dn8, locals.var_qt7_dn9, locals.var_qt7_dn10, locals.var_qt7_dn11, locals.var_qt7_dn13, locals.var_qt7_dn14,)
    }
};
        locals.var_qt7 = assign20910_e39076;
        locals.var_qt7_dn0 = assign20910_e39076_d_n0;
        locals.var_qt7_dn2 = assign20910_e39076_d_n2;
        locals.var_qt7_dn3 = assign20910_e39076_d_n3;
        locals.var_qt7_dn4 = assign20910_e39076_d_n4;
        locals.var_qt7_dn5 = assign20910_e39076_d_n5;
        locals.var_qt7_dn6 = assign20910_e39076_d_n6;
        locals.var_qt7_dn7 = assign20910_e39076_d_n7;
        locals.var_qt7_dn8 = assign20910_e39076_d_n8;
        locals.var_qt7_dn9 = assign20910_e39076_d_n9;
        locals.var_qt7_dn10 = assign20910_e39076_d_n10;
        locals.var_qt7_dn11 = assign20910_e39076_d_n11;
        locals.var_qt7_dn13 = assign20910_e39076_d_n13;
        locals.var_qt7_dn14 = assign20910_e39076_d_n14;

        let (assign20920_e39087, assign20920_e39087_d_n0, assign20920_e39087_d_n2, assign20920_e39087_d_n3, assign20920_e39087_d_n4, assign20920_e39087_d_n5, assign20920_e39087_d_n6, assign20920_e39087_d_n7, assign20920_e39087_d_n8, assign20920_e39087_d_n9, assign20920_e39087_d_n10, assign20920_e39087_d_n11, assign20920_e39087_d_n13, assign20920_e39087_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20920_e39080: f64 = (locals.var_qnd20 * locals.var_qt7);
        let assign20920_e39083: f64 = (locals.var_qt5 - locals.var_qt6);
        let assign20920_e39084: f64 = { let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign20920_e39085: f64 = (assign20920_e39080 * assign20920_e39084);
        (assign20920_e39085, (((locals.var_qnd20 * locals.var_qt7_dn0) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn0 - locals.var_qt6_dn0)))), (((locals.var_qnd20 * locals.var_qt7_dn2) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn2 - locals.var_qt6_dn2)))), (((locals.var_qnd20 * locals.var_qt7_dn3) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn3 - locals.var_qt6_dn3)))), (((locals.var_qnd20 * locals.var_qt7_dn4) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn4 - locals.var_qt6_dn4)))), (((locals.var_qnd20 * locals.var_qt7_dn5) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn5 - locals.var_qt6_dn5)))), (((locals.var_qnd20 * locals.var_qt7_dn6) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn6 - locals.var_qt6_dn6)))), (((locals.var_qnd20 * locals.var_qt7_dn7) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn7 - locals.var_qt6_dn7)))), (((locals.var_qnd20 * locals.var_qt7_dn8) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn8 - locals.var_qt6_dn8)))), (((locals.var_qnd20 * locals.var_qt7_dn9) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn9 - locals.var_qt6_dn9)))), (((locals.var_qnd20 * locals.var_qt7_dn10) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn10 - locals.var_qt6_dn10)))), (((locals.var_qnd20 * locals.var_qt7_dn11) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn11 - locals.var_qt6_dn11)))), (((locals.var_qnd20 * locals.var_qt7_dn13) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn13 - locals.var_qt6_dn13)))), (((locals.var_qnd20 * locals.var_qt7_dn14) * assign20920_e39084) + (assign20920_e39080 * ({ let limited_exp_arg = assign20920_e39083; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt5_dn14 - locals.var_qt6_dn14)))),)
    } else {
        (locals.var_qnds2, locals.var_qnds2_dn0, locals.var_qnds2_dn2, locals.var_qnds2_dn3, locals.var_qnds2_dn4, locals.var_qnds2_dn5, locals.var_qnds2_dn6, locals.var_qnds2_dn7, locals.var_qnds2_dn8, locals.var_qnds2_dn9, locals.var_qnds2_dn10, locals.var_qnds2_dn11, locals.var_qnds2_dn13, locals.var_qnds2_dn14,)
    }
};
        locals.var_qnds2 = assign20920_e39087;
        locals.var_qnds2_dn0 = assign20920_e39087_d_n0;
        locals.var_qnds2_dn2 = assign20920_e39087_d_n2;
        locals.var_qnds2_dn3 = assign20920_e39087_d_n3;
        locals.var_qnds2_dn4 = assign20920_e39087_d_n4;
        locals.var_qnds2_dn5 = assign20920_e39087_d_n5;
        locals.var_qnds2_dn6 = assign20920_e39087_d_n6;
        locals.var_qnds2_dn7 = assign20920_e39087_d_n7;
        locals.var_qnds2_dn8 = assign20920_e39087_d_n8;
        locals.var_qnds2_dn9 = assign20920_e39087_d_n9;
        locals.var_qnds2_dn10 = assign20920_e39087_d_n10;
        locals.var_qnds2_dn11 = assign20920_e39087_d_n11;
        locals.var_qnds2_dn13 = assign20920_e39087_d_n13;
        locals.var_qnds2_dn14 = assign20920_e39087_d_n14;

    }

    pub(super) fn stamp_transient_block_77(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20930_e39097, assign20930_e39097_d_n0, assign20930_e39097_d_n2, assign20930_e39097_d_n3, assign20930_e39097_d_n4, assign20930_e39097_d_n5, assign20930_e39097_d_n6, assign20930_e39097_d_n7, assign20930_e39097_d_n8, assign20930_e39097_d_n9, assign20930_e39097_d_n10, assign20930_e39097_d_n11, assign20930_e39097_d_n13, assign20930_e39097_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20930_e39091: f64 = (locals.var_vgsfb - locals.var_vch);
        let assign20930_e39093: f64 = (assign20930_e39091 - locals.var_qe3);
        let assign20930_e39095: f64 = (assign20930_e39093 / locals.var_nvtm);
        (assign20930_e39095, ((((locals.var_vgsfb_dn0 - locals.var_vch_dn0) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn0)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn2 - locals.var_vch_dn2) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn2)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn3 - locals.var_vch_dn3) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn4 - locals.var_vch_dn4) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn5 - locals.var_vch_dn5) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn6 - locals.var_vch_dn6) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn7 - locals.var_vch_dn7) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn8 - locals.var_vch_dn8) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn9 - locals.var_vch_dn9) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn9)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn10 - locals.var_vch_dn10) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn11 - locals.var_vch_dn11) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn11)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn13 - locals.var_vch_dn13) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn13)) / (locals.var_nvtm * locals.var_nvtm)), ((((locals.var_vgsfb_dn14 - locals.var_vch_dn14) * locals.var_nvtm) - (assign20930_e39093 * locals.var_nvtm_dn14)) / (locals.var_nvtm * locals.var_nvtm)),)
    } else {
        (locals.var_qt8, locals.var_qt8_dn0, locals.var_qt8_dn2, locals.var_qt8_dn3, locals.var_qt8_dn4, locals.var_qt8_dn5, locals.var_qt8_dn6, locals.var_qt8_dn7, locals.var_qt8_dn8, locals.var_qt8_dn9, locals.var_qt8_dn10, locals.var_qt8_dn11, locals.var_qt8_dn13, locals.var_qt8_dn14,)
    }
};
        locals.var_qt8 = assign20930_e39097;
        locals.var_qt8_dn0 = assign20930_e39097_d_n0;
        locals.var_qt8_dn2 = assign20930_e39097_d_n2;
        locals.var_qt8_dn3 = assign20930_e39097_d_n3;
        locals.var_qt8_dn4 = assign20930_e39097_d_n4;
        locals.var_qt8_dn5 = assign20930_e39097_d_n5;
        locals.var_qt8_dn6 = assign20930_e39097_d_n6;
        locals.var_qt8_dn7 = assign20930_e39097_d_n7;
        locals.var_qt8_dn8 = assign20930_e39097_d_n8;
        locals.var_qt8_dn9 = assign20930_e39097_d_n9;
        locals.var_qt8_dn10 = assign20930_e39097_d_n10;
        locals.var_qt8_dn11 = assign20930_e39097_d_n11;
        locals.var_qt8_dn13 = assign20930_e39097_d_n13;
        locals.var_qt8_dn14 = assign20930_e39097_d_n14;

        let (assign20940_e39120, assign20940_e39120_d_n0, assign20940_e39120_d_n2, assign20940_e39120_d_n3, assign20940_e39120_d_n4, assign20940_e39120_d_n5, assign20940_e39120_d_n6, assign20940_e39120_d_n7, assign20940_e39120_d_n8, assign20940_e39120_d_n9, assign20940_e39120_d_n10, assign20940_e39120_d_n11, assign20940_e39120_d_n13, assign20940_e39120_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20940_e39102: f64 = locals.var_qt8;
        let assign20940_e39105: f64 = locals.var_qt8;
        let assign20940_e39108: f64 = locals.var_qt8;
        let assign20940_e39109: f64 = (assign20940_e39105 * assign20940_e39108);
        let assign20940_e39112: f64 = (0.25 * locals.var_p3);
        let assign20940_e39114: f64 = (assign20940_e39112 * locals.var_p3);
        let assign20940_e39115: f64 = (assign20940_e39109 + assign20940_e39114);
        let assign20940_e39116: f64 = (assign20940_e39115).sqrt();
        let assign20940_e39117: f64 = (assign20940_e39102 + assign20940_e39116);
        let assign20940_e39118: f64 = (0.5 * assign20940_e39117);
        (assign20940_e39118, (0.5 * (locals.var_qt8_dn0 + (((locals.var_qt8_dn0 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn0)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn2 + (((locals.var_qt8_dn2 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn2)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn3 + (((locals.var_qt8_dn3 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn3)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn4 + (((locals.var_qt8_dn4 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn4)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn5 + (((locals.var_qt8_dn5 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn5)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn6 + (((locals.var_qt8_dn6 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn6)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn7 + (((locals.var_qt8_dn7 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn7)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn8 + (((locals.var_qt8_dn8 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn8)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn9 + (((locals.var_qt8_dn9 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn9)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn10 + (((locals.var_qt8_dn10 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn10)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn11 + (((locals.var_qt8_dn11 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn11)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn13 + (((locals.var_qt8_dn13 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn13)) / (2.0 * assign20940_e39116)))), (0.5 * (locals.var_qt8_dn14 + (((locals.var_qt8_dn14 * assign20940_e39108) + (assign20940_e39105 * locals.var_qt8_dn14)) / (2.0 * assign20940_e39116)))),)
    } else {
        (locals.var_qt9, locals.var_qt9_dn0, locals.var_qt9_dn2, locals.var_qt9_dn3, locals.var_qt9_dn4, locals.var_qt9_dn5, locals.var_qt9_dn6, locals.var_qt9_dn7, locals.var_qt9_dn8, locals.var_qt9_dn9, locals.var_qt9_dn10, locals.var_qt9_dn11, locals.var_qt9_dn13, locals.var_qt9_dn14,)
    }
};
        locals.var_qt9 = assign20940_e39120;
        locals.var_qt9_dn0 = assign20940_e39120_d_n0;
        locals.var_qt9_dn2 = assign20940_e39120_d_n2;
        locals.var_qt9_dn3 = assign20940_e39120_d_n3;
        locals.var_qt9_dn4 = assign20940_e39120_d_n4;
        locals.var_qt9_dn5 = assign20940_e39120_d_n5;
        locals.var_qt9_dn6 = assign20940_e39120_d_n6;
        locals.var_qt9_dn7 = assign20940_e39120_d_n7;
        locals.var_qt9_dn8 = assign20940_e39120_d_n8;
        locals.var_qt9_dn9 = assign20940_e39120_d_n9;
        locals.var_qt9_dn10 = assign20940_e39120_d_n10;
        locals.var_qt9_dn11 = assign20940_e39120_d_n11;
        locals.var_qt9_dn13 = assign20940_e39120_d_n13;
        locals.var_qt9_dn14 = assign20940_e39120_d_n14;

        let (assign20950_e39128, assign20950_e39128_d_n0, assign20950_e39128_d_n2, assign20950_e39128_d_n3, assign20950_e39128_d_n4, assign20950_e39128_d_n5, assign20950_e39128_d_n6, assign20950_e39128_d_n7, assign20950_e39128_d_n8, assign20950_e39128_d_n9, assign20950_e39128_d_n10, assign20950_e39128_d_n11, assign20950_e39128_d_n13, assign20950_e39128_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20950_e39125: f64 = (locals.var_d3 / 2.0);
        let assign20950_e39126: f64 = (locals.var_qt9).powf(assign20950_e39125);
        (assign20950_e39126, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn0)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn0 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn2)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn2 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn3)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn3 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn4)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn4 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn5)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn5 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn6)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn6 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn7)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn7 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn8)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn8 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn9)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn9 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn10)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn10 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn11)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn11 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn13)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn13 / locals.var_qt9))) }, if 0.0 == 0.0 && ((assign20950_e39125) as f64).is_finite() && ((assign20950_e39125) as f64).fract() == 0.0 { if assign20950_e39125 == 0.0 { 0.0 } else { (assign20950_e39125 * ((locals.var_qt9).powf(assign20950_e39125 - 1.0) * locals.var_qt9_dn14)) } } else { (assign20950_e39126 * (assign20950_e39125 * (locals.var_qt9_dn14 / locals.var_qt9))) },)
    } else {
        (locals.var_qt10, locals.var_qt10_dn0, locals.var_qt10_dn2, locals.var_qt10_dn3, locals.var_qt10_dn4, locals.var_qt10_dn5, locals.var_qt10_dn6, locals.var_qt10_dn7, locals.var_qt10_dn8, locals.var_qt10_dn9, locals.var_qt10_dn10, locals.var_qt10_dn11, locals.var_qt10_dn13, locals.var_qt10_dn14,)
    }
};
        locals.var_qt10 = assign20950_e39128;
        locals.var_qt10_dn0 = assign20950_e39128_d_n0;
        locals.var_qt10_dn2 = assign20950_e39128_d_n2;
        locals.var_qt10_dn3 = assign20950_e39128_d_n3;
        locals.var_qt10_dn4 = assign20950_e39128_d_n4;
        locals.var_qt10_dn5 = assign20950_e39128_d_n5;
        locals.var_qt10_dn6 = assign20950_e39128_d_n6;
        locals.var_qt10_dn7 = assign20950_e39128_d_n7;
        locals.var_qt10_dn8 = assign20950_e39128_d_n8;
        locals.var_qt10_dn9 = assign20950_e39128_d_n9;
        locals.var_qt10_dn10 = assign20950_e39128_d_n10;
        locals.var_qt10_dn11 = assign20950_e39128_d_n11;
        locals.var_qt10_dn13 = assign20950_e39128_d_n13;
        locals.var_qt10_dn14 = assign20950_e39128_d_n14;

        let (assign20960_e39139, assign20960_e39139_d_n0, assign20960_e39139_d_n2, assign20960_e39139_d_n3, assign20960_e39139_d_n4, assign20960_e39139_d_n5, assign20960_e39139_d_n6, assign20960_e39139_d_n7, assign20960_e39139_d_n8, assign20960_e39139_d_n9, assign20960_e39139_d_n10, assign20960_e39139_d_n11, assign20960_e39139_d_n13, assign20960_e39139_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20960_e39132: f64 = (locals.var_qnd30 * locals.var_qt10);
        let assign20960_e39135: f64 = (locals.var_qt8 - locals.var_qt9);
        let assign20960_e39136: f64 = { let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign20960_e39137: f64 = (assign20960_e39132 * assign20960_e39136);
        (assign20960_e39137, (((locals.var_qnd30 * locals.var_qt10_dn0) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn0 - locals.var_qt9_dn0)))), (((locals.var_qnd30 * locals.var_qt10_dn2) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn2 - locals.var_qt9_dn2)))), (((locals.var_qnd30 * locals.var_qt10_dn3) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn3 - locals.var_qt9_dn3)))), (((locals.var_qnd30 * locals.var_qt10_dn4) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn4 - locals.var_qt9_dn4)))), (((locals.var_qnd30 * locals.var_qt10_dn5) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn5 - locals.var_qt9_dn5)))), (((locals.var_qnd30 * locals.var_qt10_dn6) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn6 - locals.var_qt9_dn6)))), (((locals.var_qnd30 * locals.var_qt10_dn7) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn7 - locals.var_qt9_dn7)))), (((locals.var_qnd30 * locals.var_qt10_dn8) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn8 - locals.var_qt9_dn8)))), (((locals.var_qnd30 * locals.var_qt10_dn9) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn9 - locals.var_qt9_dn9)))), (((locals.var_qnd30 * locals.var_qt10_dn10) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn10 - locals.var_qt9_dn10)))), (((locals.var_qnd30 * locals.var_qt10_dn11) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn11 - locals.var_qt9_dn11)))), (((locals.var_qnd30 * locals.var_qt10_dn13) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn13 - locals.var_qt9_dn13)))), (((locals.var_qnd30 * locals.var_qt10_dn14) * assign20960_e39136) + (assign20960_e39132 * ({ let limited_exp_arg = assign20960_e39135; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_qt8_dn14 - locals.var_qt9_dn14)))),)
    } else {
        (locals.var_qnds3, locals.var_qnds3_dn0, locals.var_qnds3_dn2, locals.var_qnds3_dn3, locals.var_qnds3_dn4, locals.var_qnds3_dn5, locals.var_qnds3_dn6, locals.var_qnds3_dn7, locals.var_qnds3_dn8, locals.var_qnds3_dn9, locals.var_qnds3_dn10, locals.var_qnds3_dn11, locals.var_qnds3_dn13, locals.var_qnds3_dn14,)
    }
};
        locals.var_qnds3 = assign20960_e39139;
        locals.var_qnds3_dn0 = assign20960_e39139_d_n0;
        locals.var_qnds3_dn2 = assign20960_e39139_d_n2;
        locals.var_qnds3_dn3 = assign20960_e39139_d_n3;
        locals.var_qnds3_dn4 = assign20960_e39139_d_n4;
        locals.var_qnds3_dn5 = assign20960_e39139_d_n5;
        locals.var_qnds3_dn6 = assign20960_e39139_d_n6;
        locals.var_qnds3_dn7 = assign20960_e39139_d_n7;
        locals.var_qnds3_dn8 = assign20960_e39139_d_n8;
        locals.var_qnds3_dn9 = assign20960_e39139_d_n9;
        locals.var_qnds3_dn10 = assign20960_e39139_d_n10;
        locals.var_qnds3_dn11 = assign20960_e39139_d_n11;
        locals.var_qnds3_dn13 = assign20960_e39139_d_n13;
        locals.var_qnds3_dn14 = assign20960_e39139_d_n14;

        let (assign20970_e39153, assign20970_e39153_d_n0, assign20970_e39153_d_n2, assign20970_e39153_d_n3, assign20970_e39153_d_n4, assign20970_e39153_d_n5, assign20970_e39153_d_n6, assign20970_e39153_d_n7, assign20970_e39153_d_n8, assign20970_e39153_d_n9, assign20970_e39153_d_n10, assign20970_e39153_d_n11, assign20970_e39153_d_n13, assign20970_e39153_d_n14,) = {
    if (locals.var_guard368 != 0.0) {
        let assign20970_e39143: f64 = (locals.var_nc3d * locals.var_qis);
        let assign20970_e39147: f64 = (locals.var_qnds1 + locals.var_qnds2);
        let assign20970_e39149: f64 = (assign20970_e39147 + locals.var_qnds3);
        let assign20970_e39150: f64 = (locals.var_ncq * assign20970_e39149);
        let assign20970_e39151: f64 = (assign20970_e39143 + assign20970_e39150);
        (assign20970_e39151, ((locals.var_nc3d * locals.var_qis_dn0) + (locals.var_ncq * ((locals.var_qnds1_dn0 + locals.var_qnds2_dn0) + locals.var_qnds3_dn0))), ((locals.var_nc3d * locals.var_qis_dn2) + (locals.var_ncq * ((locals.var_qnds1_dn2 + locals.var_qnds2_dn2) + locals.var_qnds3_dn2))), ((locals.var_nc3d * locals.var_qis_dn3) + (locals.var_ncq * ((locals.var_qnds1_dn3 + locals.var_qnds2_dn3) + locals.var_qnds3_dn3))), ((locals.var_nc3d * locals.var_qis_dn4) + (locals.var_ncq * ((locals.var_qnds1_dn4 + locals.var_qnds2_dn4) + locals.var_qnds3_dn4))), ((locals.var_nc3d * locals.var_qis_dn5) + (locals.var_ncq * ((locals.var_qnds1_dn5 + locals.var_qnds2_dn5) + locals.var_qnds3_dn5))), ((locals.var_nc3d * locals.var_qis_dn6) + (locals.var_ncq * ((locals.var_qnds1_dn6 + locals.var_qnds2_dn6) + locals.var_qnds3_dn6))), ((locals.var_nc3d * locals.var_qis_dn7) + (locals.var_ncq * ((locals.var_qnds1_dn7 + locals.var_qnds2_dn7) + locals.var_qnds3_dn7))), ((locals.var_nc3d * locals.var_qis_dn8) + (locals.var_ncq * ((locals.var_qnds1_dn8 + locals.var_qnds2_dn8) + locals.var_qnds3_dn8))), ((locals.var_nc3d * locals.var_qis_dn9) + (locals.var_ncq * ((locals.var_qnds1_dn9 + locals.var_qnds2_dn9) + locals.var_qnds3_dn9))), ((locals.var_nc3d * locals.var_qis_dn10) + (locals.var_ncq * ((locals.var_qnds1_dn10 + locals.var_qnds2_dn10) + locals.var_qnds3_dn10))), ((locals.var_nc3d * locals.var_qis_dn11) + (locals.var_ncq * ((locals.var_qnds1_dn11 + locals.var_qnds2_dn11) + locals.var_qnds3_dn11))), ((locals.var_nc3d * locals.var_qis_dn13) + (locals.var_ncq * ((locals.var_qnds1_dn13 + locals.var_qnds2_dn13) + locals.var_qnds3_dn13))), ((locals.var_nc3d * locals.var_qis_dn14) + (locals.var_ncq * ((locals.var_qnds1_dn14 + locals.var_qnds2_dn14) + locals.var_qnds3_dn14))),)
    } else {
        (locals.var_qis, locals.var_qis_dn0, locals.var_qis_dn2, locals.var_qis_dn3, locals.var_qis_dn4, locals.var_qis_dn5, locals.var_qis_dn6, locals.var_qis_dn7, locals.var_qis_dn8, locals.var_qis_dn9, locals.var_qis_dn10, locals.var_qis_dn11, locals.var_qis_dn13, locals.var_qis_dn14,)
    }
};
        locals.var_qis = assign20970_e39153;
        locals.var_qis_dn0 = assign20970_e39153_d_n0;
        locals.var_qis_dn2 = assign20970_e39153_d_n2;
        locals.var_qis_dn3 = assign20970_e39153_d_n3;
        locals.var_qis_dn4 = assign20970_e39153_d_n4;
        locals.var_qis_dn5 = assign20970_e39153_d_n5;
        locals.var_qis_dn6 = assign20970_e39153_d_n6;
        locals.var_qis_dn7 = assign20970_e39153_d_n7;
        locals.var_qis_dn8 = assign20970_e39153_d_n8;
        locals.var_qis_dn9 = assign20970_e39153_d_n9;
        locals.var_qis_dn10 = assign20970_e39153_d_n10;
        locals.var_qis_dn11 = assign20970_e39153_d_n11;
        locals.var_qis_dn13 = assign20970_e39153_d_n13;
        locals.var_qis_dn14 = assign20970_e39153_d_n14;

        let assign20980_e39156: f64 = (0.01 / locals.var_cox);
        locals.var_qb0 = assign20980_e39156;

        let assign20990_e39161: f64 = (locals.var_eta_mu * locals.var_qis);
        let assign20990_e39162: f64 = (locals.var_qbs + assign20990_e39161);
        let assign20990_e39163: f64 = (locals.var_eefffactor * assign20990_e39162);
        locals.var_eeffs = assign20990_e39163;
        locals.var_eeffs_dn0 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn0));
        locals.var_eeffs_dn2 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn2));
        locals.var_eeffs_dn3 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn3));
        locals.var_eeffs_dn4 = (locals.var_eefffactor * ((locals.var_eta_mu_dn4 * locals.var_qis) + (locals.var_eta_mu * locals.var_qis_dn4)));
        locals.var_eeffs_dn5 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn5));
        locals.var_eeffs_dn6 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn6));
        locals.var_eeffs_dn7 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn7));
        locals.var_eeffs_dn8 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn8));
        locals.var_eeffs_dn9 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn9));
        locals.var_eeffs_dn10 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn10));
        locals.var_eeffs_dn11 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn11));
        locals.var_eeffs_dn13 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn13));
        locals.var_eeffs_dn14 = (locals.var_eefffactor * (locals.var_eta_mu * locals.var_qis_dn14));

        let assign21000_e39168: f64 = (locals.var_qis / locals.var_qb0);
        let assign21000_e39169: f64 = (1.0 + assign21000_e39168);
        let assign21000_e39170: f64 = (0.5 * assign21000_e39169);
        let assign21000_e39172: f64 = (assign21000_e39170).powf(locals.var_ucs_t);
        locals.var_t2 = assign21000_e39172;
        locals.var_t2_dn0 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn0 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn0 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn2 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn2 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn2 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn3 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn3 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn3 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn4 = if locals.var_ucs_t_dn4 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn4 / locals.var_qb0)))) } } else { (assign21000_e39172 * ((locals.var_ucs_t_dn4 * (assign21000_e39170).ln()) + (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn4 / locals.var_qb0)) / assign21000_e39170)))) };
        locals.var_t2_dn5 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn5 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn5 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn6 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn6 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn6 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn7 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn7 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn7 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn8 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn8 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn8 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn9 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn9 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn9 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn10 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn10 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn10 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn11 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn11 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn11 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn13 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn13 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn13 / locals.var_qb0)) / assign21000_e39170))) };
        locals.var_t2_dn14 = if 0.0 == 0.0 && ((locals.var_ucs_t) as f64).is_finite() && ((locals.var_ucs_t) as f64).fract() == 0.0 { if locals.var_ucs_t == 0.0 { 0.0 } else { (locals.var_ucs_t * ((assign21000_e39170).powf(locals.var_ucs_t - 1.0) * (0.5 * (locals.var_qis_dn14 / locals.var_qb0)))) } } else { (assign21000_e39172 * (locals.var_ucs_t * ((0.5 * (locals.var_qis_dn14 / locals.var_qb0)) / assign21000_e39170))) };

        let assign21010_e39175: f64 = (locals.var_eeffs).powf(locals.var_eu_a);
        locals.var_t3 = assign21010_e39175;
        locals.var_t3_dn0 = if locals.var_eu_a_dn0 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn0)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn0 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn0 / locals.var_eeffs)))) };
        locals.var_t3_dn2 = if locals.var_eu_a_dn2 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn2)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn2 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn2 / locals.var_eeffs)))) };
        locals.var_t3_dn3 = if locals.var_eu_a_dn3 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn3)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn3 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn3 / locals.var_eeffs)))) };
        locals.var_t3_dn4 = if locals.var_eu_a_dn4 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn4)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn4 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn4 / locals.var_eeffs)))) };
        locals.var_t3_dn5 = if locals.var_eu_a_dn5 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn5)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn5 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn5 / locals.var_eeffs)))) };
        locals.var_t3_dn6 = if locals.var_eu_a_dn6 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn6)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn6 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn6 / locals.var_eeffs)))) };
        locals.var_t3_dn7 = if locals.var_eu_a_dn7 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn7)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn7 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn7 / locals.var_eeffs)))) };
        locals.var_t3_dn8 = if locals.var_eu_a_dn8 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn8)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn8 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn8 / locals.var_eeffs)))) };
        locals.var_t3_dn9 = if locals.var_eu_a_dn9 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn9)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn9 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn9 / locals.var_eeffs)))) };
        locals.var_t3_dn10 = if locals.var_eu_a_dn10 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn10)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn10 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn10 / locals.var_eeffs)))) };
        locals.var_t3_dn11 = if locals.var_eu_a_dn11 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn11)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn11 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn11 / locals.var_eeffs)))) };
        locals.var_t3_dn13 = if locals.var_eu_a_dn13 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn13)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn13 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn13 / locals.var_eeffs)))) };
        locals.var_t3_dn14 = if locals.var_eu_a_dn14 == 0.0 && ((locals.var_eu_a) as f64).is_finite() && ((locals.var_eu_a) as f64).fract() == 0.0 { if locals.var_eu_a == 0.0 { 0.0 } else { (locals.var_eu_a * ((locals.var_eeffs).powf(locals.var_eu_a - 1.0) * locals.var_eeffs_dn14)) } } else { (assign21010_e39175 * ((locals.var_eu_a_dn14 * (locals.var_eeffs).ln()) + (locals.var_eu_a * (locals.var_eeffs_dn14 / locals.var_eeffs)))) };

        let assign21020_e39178: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard369 = assign21020_e39178;

        let (assign21030_e39192, assign21030_e39192_d_n0, assign21030_e39192_d_n2, assign21030_e39192_d_n3, assign21030_e39192_d_n4, assign21030_e39192_d_n5, assign21030_e39192_d_n6, assign21030_e39192_d_n7, assign21030_e39192_d_n8, assign21030_e39192_d_n9, assign21030_e39192_d_n10, assign21030_e39192_d_n11, assign21030_e39192_d_n13, assign21030_e39192_d_n14,) = {
    if (locals.var_guard369 != 0.0) {
        let assign21030_e39183: f64 = (locals.var_uc_a * locals.var_veseff);
        let assign21030_e39184: f64 = (locals.var_ua_a + assign21030_e39183);
        let assign21030_e39186: f64 = (assign21030_e39184 * locals.var_t3);
        let assign21030_e39189: f64 = (locals.var_ud_a / locals.var_t2);
        let assign21030_e39190: f64 = (assign21030_e39186 + assign21030_e39189);
        (assign21030_e39190, ((((locals.var_ua_a_dn0 + ((locals.var_uc_a_dn0 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn0))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn0)) + (((locals.var_ud_a_dn0 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn2 + ((locals.var_uc_a_dn2 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn2))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn2)) + (((locals.var_ud_a_dn2 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn3 + ((locals.var_uc_a_dn3 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn3))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn3)) + (((locals.var_ud_a_dn3 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn4 + ((locals.var_uc_a_dn4 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn4))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn4)) + (((locals.var_ud_a_dn4 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn5 + ((locals.var_uc_a_dn5 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn5))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn5)) + (((locals.var_ud_a_dn5 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn6 + ((locals.var_uc_a_dn6 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn6))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn6)) + (((locals.var_ud_a_dn6 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn7 + ((locals.var_uc_a_dn7 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn7))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn7)) + (((locals.var_ud_a_dn7 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn8 + ((locals.var_uc_a_dn8 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn8))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn8)) + (((locals.var_ud_a_dn8 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn9 + ((locals.var_uc_a_dn9 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn9))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn9)) + (((locals.var_ud_a_dn9 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn10 + ((locals.var_uc_a_dn10 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn10))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn10)) + (((locals.var_ud_a_dn10 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn11 + ((locals.var_uc_a_dn11 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn11))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn11)) + (((locals.var_ud_a_dn11 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn13 + ((locals.var_uc_a_dn13 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn13))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn13)) + (((locals.var_ud_a_dn13 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2))), ((((locals.var_ua_a_dn14 + ((locals.var_uc_a_dn14 * locals.var_veseff) + (locals.var_uc_a * locals.var_veseff_dn14))) * locals.var_t3) + (assign21030_e39184 * locals.var_t3_dn14)) + (((locals.var_ud_a_dn14 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21030_e39192;
        locals.var_t3_dn0 = assign21030_e39192_d_n0;
        locals.var_t3_dn2 = assign21030_e39192_d_n2;
        locals.var_t3_dn3 = assign21030_e39192_d_n3;
        locals.var_t3_dn4 = assign21030_e39192_d_n4;
        locals.var_t3_dn5 = assign21030_e39192_d_n5;
        locals.var_t3_dn6 = assign21030_e39192_d_n6;
        locals.var_t3_dn7 = assign21030_e39192_d_n7;
        locals.var_t3_dn8 = assign21030_e39192_d_n8;
        locals.var_t3_dn9 = assign21030_e39192_d_n9;
        locals.var_t3_dn10 = assign21030_e39192_d_n10;
        locals.var_t3_dn11 = assign21030_e39192_d_n11;
        locals.var_t3_dn13 = assign21030_e39192_d_n13;
        locals.var_t3_dn14 = assign21030_e39192_d_n14;

        let (assign21040_e39203, assign21040_e39203_d_n0, assign21040_e39203_d_n2, assign21040_e39203_d_n3, assign21040_e39203_d_n4, assign21040_e39203_d_n5, assign21040_e39203_d_n6, assign21040_e39203_d_n7, assign21040_e39203_d_n8, assign21040_e39203_d_n9, assign21040_e39203_d_n10, assign21040_e39203_d_n11, assign21040_e39203_d_n13, assign21040_e39203_d_n14,) = {
    if (locals.var_guard369 == 0.0) {
        let assign21040_e39197: f64 = (locals.var_ua_a * locals.var_t3);
        let assign21040_e39200: f64 = (locals.var_ud_a / locals.var_t2);
        let assign21040_e39201: f64 = (assign21040_e39197 + assign21040_e39200);
        (assign21040_e39201, (((locals.var_ua_a_dn0 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn0)) + (((locals.var_ud_a_dn0 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn2 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn2)) + (((locals.var_ud_a_dn2 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn3 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn3)) + (((locals.var_ud_a_dn3 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn4 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn4)) + (((locals.var_ud_a_dn4 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn5 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn5)) + (((locals.var_ud_a_dn5 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn6 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn6)) + (((locals.var_ud_a_dn6 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn7 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn7)) + (((locals.var_ud_a_dn7 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn8 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn8)) + (((locals.var_ud_a_dn8 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn9 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn9)) + (((locals.var_ud_a_dn9 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn10 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn10)) + (((locals.var_ud_a_dn10 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn11 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn11)) + (((locals.var_ud_a_dn11 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn13 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn13)) + (((locals.var_ud_a_dn13 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2))), (((locals.var_ua_a_dn14 * locals.var_t3) + (locals.var_ua_a * locals.var_t3_dn14)) + (((locals.var_ud_a_dn14 * locals.var_t2) - (locals.var_ud_a * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21040_e39203;
        locals.var_t3_dn0 = assign21040_e39203_d_n0;
        locals.var_t3_dn2 = assign21040_e39203_d_n2;
        locals.var_t3_dn3 = assign21040_e39203_d_n3;
        locals.var_t3_dn4 = assign21040_e39203_d_n4;
        locals.var_t3_dn5 = assign21040_e39203_d_n5;
        locals.var_t3_dn6 = assign21040_e39203_d_n6;
        locals.var_t3_dn7 = assign21040_e39203_d_n7;
        locals.var_t3_dn8 = assign21040_e39203_d_n8;
        locals.var_t3_dn9 = assign21040_e39203_d_n9;
        locals.var_t3_dn10 = assign21040_e39203_d_n10;
        locals.var_t3_dn11 = assign21040_e39203_d_n11;
        locals.var_t3_dn13 = assign21040_e39203_d_n13;
        locals.var_t3_dn14 = assign21040_e39203_d_n14;

        let assign21050_e39206: f64 = (1.0 + locals.var_t3);
        locals.var_dmobs = assign21050_e39206;
        locals.var_dmobs_dn0 = locals.var_t3_dn0;
        locals.var_dmobs_dn2 = locals.var_t3_dn2;
        locals.var_dmobs_dn3 = locals.var_t3_dn3;
        locals.var_dmobs_dn4 = locals.var_t3_dn4;
        locals.var_dmobs_dn5 = locals.var_t3_dn5;
        locals.var_dmobs_dn6 = locals.var_t3_dn6;
        locals.var_dmobs_dn7 = locals.var_t3_dn7;
        locals.var_dmobs_dn8 = locals.var_t3_dn8;
        locals.var_dmobs_dn9 = locals.var_t3_dn9;
        locals.var_dmobs_dn10 = locals.var_t3_dn10;
        locals.var_dmobs_dn11 = locals.var_t3_dn11;
        locals.var_dmobs_dn13 = locals.var_t3_dn13;
        locals.var_dmobs_dn14 = locals.var_t3_dn14;

        let assign21060_e39210: f64 = (locals.var_dmobs + 1.0);
        let assign21060_e39213: f64 = (locals.var_dmobs - 1.0);
        let assign21060_e39216: f64 = (locals.var_dmobs - 1.0);
        let assign21060_e39217: f64 = (assign21060_e39213 * assign21060_e39216);
        let assign21060_e39220: f64 = (0.25 * p.p604);
        let assign21060_e39222: f64 = (assign21060_e39220 * p.p604);
        let assign21060_e39223: f64 = (assign21060_e39217 + assign21060_e39222);
        let assign21060_e39224: f64 = (assign21060_e39223).sqrt();
        let assign21060_e39225: f64 = (assign21060_e39210 + assign21060_e39224);
        let assign21060_e39226: f64 = (0.5 * assign21060_e39225);
        locals.var_dmobs = assign21060_e39226;
        locals.var_dmobs_dn0 = (0.5 * (locals.var_dmobs_dn0 + (((locals.var_dmobs_dn0 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn0)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn2 = (0.5 * (locals.var_dmobs_dn2 + (((locals.var_dmobs_dn2 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn2)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn3 = (0.5 * (locals.var_dmobs_dn3 + (((locals.var_dmobs_dn3 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn3)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn4 = (0.5 * (locals.var_dmobs_dn4 + (((locals.var_dmobs_dn4 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn4)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn5 = (0.5 * (locals.var_dmobs_dn5 + (((locals.var_dmobs_dn5 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn5)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn6 = (0.5 * (locals.var_dmobs_dn6 + (((locals.var_dmobs_dn6 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn6)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn7 = (0.5 * (locals.var_dmobs_dn7 + (((locals.var_dmobs_dn7 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn7)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn8 = (0.5 * (locals.var_dmobs_dn8 + (((locals.var_dmobs_dn8 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn8)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn9 = (0.5 * (locals.var_dmobs_dn9 + (((locals.var_dmobs_dn9 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn9)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn10 = (0.5 * (locals.var_dmobs_dn10 + (((locals.var_dmobs_dn10 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn10)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn11 = (0.5 * (locals.var_dmobs_dn11 + (((locals.var_dmobs_dn11 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn11)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn13 = (0.5 * (locals.var_dmobs_dn13 + (((locals.var_dmobs_dn13 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn13)) / (2.0 * assign21060_e39224))));
        locals.var_dmobs_dn14 = (0.5 * (locals.var_dmobs_dn14 + (((locals.var_dmobs_dn14 * assign21060_e39216) + (assign21060_e39213 * locals.var_dmobs_dn14)) / (2.0 * assign21060_e39224))));

        let assign21070_e39229: f64 = (locals.var_dmobs / p.p24);
        locals.var_dmobs = assign21070_e39229;
        locals.var_dmobs_dn0 = (locals.var_dmobs_dn0 / p.p24);
        locals.var_dmobs_dn2 = (locals.var_dmobs_dn2 / p.p24);
        locals.var_dmobs_dn3 = (locals.var_dmobs_dn3 / p.p24);
        locals.var_dmobs_dn4 = (locals.var_dmobs_dn4 / p.p24);
        locals.var_dmobs_dn5 = (locals.var_dmobs_dn5 / p.p24);
        locals.var_dmobs_dn6 = (locals.var_dmobs_dn6 / p.p24);
        locals.var_dmobs_dn7 = (locals.var_dmobs_dn7 / p.p24);
        locals.var_dmobs_dn8 = (locals.var_dmobs_dn8 / p.p24);
        locals.var_dmobs_dn9 = (locals.var_dmobs_dn9 / p.p24);
        locals.var_dmobs_dn10 = (locals.var_dmobs_dn10 / p.p24);
        locals.var_dmobs_dn11 = (locals.var_dmobs_dn11 / p.p24);
        locals.var_dmobs_dn13 = (locals.var_dmobs_dn13 / p.p24);
        locals.var_dmobs_dn14 = (locals.var_dmobs_dn14 / p.p24);

        let assign21080_e39232: f64 = if p.p64 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard370 = assign21080_e39232;

        let (assign21090_e39236, assign21090_e39236_d_n0, assign21090_e39236_d_n2, assign21090_e39236_d_n3, assign21090_e39236_d_n4, assign21090_e39236_d_n5, assign21090_e39236_d_n6, assign21090_e39236_d_n7, assign21090_e39236_d_n8, assign21090_e39236_d_n9, assign21090_e39236_d_n10, assign21090_e39236_d_n11, assign21090_e39236_d_n13, assign21090_e39236_d_n14,) = {
    if (locals.var_guard370 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdss, locals.var_rdss_dn0, locals.var_rdss_dn2, locals.var_rdss_dn3, locals.var_rdss_dn4, locals.var_rdss_dn5, locals.var_rdss_dn6, locals.var_rdss_dn7, locals.var_rdss_dn8, locals.var_rdss_dn9, locals.var_rdss_dn10, locals.var_rdss_dn11, locals.var_rdss_dn13, locals.var_rdss_dn14,)
    }
};
        locals.var_rdss = assign21090_e39236;
        locals.var_rdss_dn0 = assign21090_e39236_d_n0;
        locals.var_rdss_dn2 = assign21090_e39236_d_n2;
        locals.var_rdss_dn3 = assign21090_e39236_d_n3;
        locals.var_rdss_dn4 = assign21090_e39236_d_n4;
        locals.var_rdss_dn5 = assign21090_e39236_d_n5;
        locals.var_rdss_dn6 = assign21090_e39236_d_n6;
        locals.var_rdss_dn7 = assign21090_e39236_d_n7;
        locals.var_rdss_dn8 = assign21090_e39236_d_n8;
        locals.var_rdss_dn9 = assign21090_e39236_d_n9;
        locals.var_rdss_dn10 = assign21090_e39236_d_n10;
        locals.var_rdss_dn11 = assign21090_e39236_d_n11;
        locals.var_rdss_dn13 = assign21090_e39236_d_n13;
        locals.var_rdss_dn14 = assign21090_e39236_d_n14;

        let assign21100_e39239: f64 = if p.p64 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard371 = assign21100_e39239;

        let (assign21110_e39250, assign21110_e39250_d_n0, assign21110_e39250_d_n2, assign21110_e39250_d_n3, assign21110_e39250_d_n4, assign21110_e39250_d_n5, assign21110_e39250_d_n6, assign21110_e39250_d_n7, assign21110_e39250_d_n8, assign21110_e39250_d_n9, assign21110_e39250_d_n10, assign21110_e39250_d_n11, assign21110_e39250_d_n13, assign21110_e39250_d_n14,) = {
    if ((locals.var_guard370 == 0.0) && (locals.var_guard371 != 0.0)) {
        let assign21110_e39247: f64 = (locals.var_prwgs_i * locals.var_qis);
        let assign21110_e39248: f64 = (1.0 + assign21110_e39247);
        (assign21110_e39248, (locals.var_prwgs_i * locals.var_qis_dn0), (locals.var_prwgs_i * locals.var_qis_dn2), (locals.var_prwgs_i * locals.var_qis_dn3), (locals.var_prwgs_i * locals.var_qis_dn4), (locals.var_prwgs_i * locals.var_qis_dn5), (locals.var_prwgs_i * locals.var_qis_dn6), (locals.var_prwgs_i * locals.var_qis_dn7), (locals.var_prwgs_i * locals.var_qis_dn8), (locals.var_prwgs_i * locals.var_qis_dn9), (locals.var_prwgs_i * locals.var_qis_dn10), (locals.var_prwgs_i * locals.var_qis_dn11), (locals.var_prwgs_i * locals.var_qis_dn13), (locals.var_prwgs_i * locals.var_qis_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21110_e39250;
        locals.var_t4_dn0 = assign21110_e39250_d_n0;
        locals.var_t4_dn2 = assign21110_e39250_d_n2;
        locals.var_t4_dn3 = assign21110_e39250_d_n3;
        locals.var_t4_dn4 = assign21110_e39250_d_n4;
        locals.var_t4_dn5 = assign21110_e39250_d_n5;
        locals.var_t4_dn6 = assign21110_e39250_d_n6;
        locals.var_t4_dn7 = assign21110_e39250_d_n7;
        locals.var_t4_dn8 = assign21110_e39250_d_n8;
        locals.var_t4_dn9 = assign21110_e39250_d_n9;
        locals.var_t4_dn10 = assign21110_e39250_d_n10;
        locals.var_t4_dn11 = assign21110_e39250_d_n11;
        locals.var_t4_dn13 = assign21110_e39250_d_n13;
        locals.var_t4_dn14 = assign21110_e39250_d_n14;

        let (assign21120_e39259, assign21120_e39259_d_n0, assign21120_e39259_d_n2, assign21120_e39259_d_n3, assign21120_e39259_d_n4, assign21120_e39259_d_n5, assign21120_e39259_d_n6, assign21120_e39259_d_n7, assign21120_e39259_d_n8, assign21120_e39259_d_n9, assign21120_e39259_d_n10, assign21120_e39259_d_n11, assign21120_e39259_d_n13, assign21120_e39259_d_n14,) = {
    if ((locals.var_guard370 == 0.0) && (locals.var_guard371 != 0.0)) {
        let assign21120_e39257: f64 = (1.0 / locals.var_t4);
        (assign21120_e39257, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21120_e39259;
        locals.var_t1_dn0 = assign21120_e39259_d_n0;
        locals.var_t1_dn2 = assign21120_e39259_d_n2;
        locals.var_t1_dn3 = assign21120_e39259_d_n3;
        locals.var_t1_dn4 = assign21120_e39259_d_n4;
        locals.var_t1_dn5 = assign21120_e39259_d_n5;
        locals.var_t1_dn6 = assign21120_e39259_d_n6;
        locals.var_t1_dn7 = assign21120_e39259_d_n7;
        locals.var_t1_dn8 = assign21120_e39259_d_n8;
        locals.var_t1_dn9 = assign21120_e39259_d_n9;
        locals.var_t1_dn10 = assign21120_e39259_d_n10;
        locals.var_t1_dn11 = assign21120_e39259_d_n11;
        locals.var_t1_dn13 = assign21120_e39259_d_n13;
        locals.var_t1_dn14 = assign21120_e39259_d_n14;

        let (assign21130_e39275, assign21130_e39275_d_n0, assign21130_e39275_d_n2, assign21130_e39275_d_n3, assign21130_e39275_d_n4, assign21130_e39275_d_n5, assign21130_e39275_d_n6, assign21130_e39275_d_n7, assign21130_e39275_d_n8, assign21130_e39275_d_n9, assign21130_e39275_d_n10, assign21130_e39275_d_n11, assign21130_e39275_d_n13, assign21130_e39275_d_n14,) = {
    if ((locals.var_guard370 == 0.0) && (locals.var_guard371 != 0.0)) {
        let assign21130_e39268: f64 = (locals.var_t1 * locals.var_t1);
        let assign21130_e39270: f64 = (assign21130_e39268 + 0.01);
        let assign21130_e39271: f64 = (assign21130_e39270).sqrt();
        let assign21130_e39272: f64 = (locals.var_t1 + assign21130_e39271);
        let assign21130_e39273: f64 = (0.5 * assign21130_e39272);
        (assign21130_e39273, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign21130_e39271)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign21130_e39271)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21130_e39275;
        locals.var_t0_dn0 = assign21130_e39275_d_n0;
        locals.var_t0_dn2 = assign21130_e39275_d_n2;
        locals.var_t0_dn3 = assign21130_e39275_d_n3;
        locals.var_t0_dn4 = assign21130_e39275_d_n4;
        locals.var_t0_dn5 = assign21130_e39275_d_n5;
        locals.var_t0_dn6 = assign21130_e39275_d_n6;
        locals.var_t0_dn7 = assign21130_e39275_d_n7;
        locals.var_t0_dn8 = assign21130_e39275_d_n8;
        locals.var_t0_dn9 = assign21130_e39275_d_n9;
        locals.var_t0_dn10 = assign21130_e39275_d_n10;
        locals.var_t0_dn11 = assign21130_e39275_d_n11;
        locals.var_t0_dn13 = assign21130_e39275_d_n13;
        locals.var_t0_dn14 = assign21130_e39275_d_n14;

        let (assign21140_e39292, assign21140_e39292_d_n0, assign21140_e39292_d_n2, assign21140_e39292_d_n3, assign21140_e39292_d_n4, assign21140_e39292_d_n5, assign21140_e39292_d_n6, assign21140_e39292_d_n7, assign21140_e39292_d_n8, assign21140_e39292_d_n9, assign21140_e39292_d_n10, assign21140_e39292_d_n11, assign21140_e39292_d_n13, assign21140_e39292_d_n14,) = {
    if ((locals.var_guard370 == 0.0) && (locals.var_guard371 != 0.0)) {
        let assign21140_e39283: f64 = (locals.var_rdsw_i * locals.var_t0);
        let assign21140_e39284: f64 = (p.p908 + assign21140_e39283);
        let assign21140_e39286: f64 = (assign21140_e39284 * locals.var_weffwrfactor);
        let assign21140_e39288: f64 = (assign21140_e39286 * locals.var_nfintotal);
        let assign21140_e39290: f64 = (assign21140_e39288 * locals.var_rdstemp);
        (assign21140_e39290, ((((((locals.var_rdsw_i_dn0 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn0)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn0)), ((((((locals.var_rdsw_i_dn2 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn2)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn2)), ((((((locals.var_rdsw_i_dn3 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn3)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn3)), ((((((locals.var_rdsw_i_dn4 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn4)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn4)), ((((((locals.var_rdsw_i_dn5 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn5)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn5)), ((((((locals.var_rdsw_i_dn6 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn6)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn6)), ((((((locals.var_rdsw_i_dn7 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn7)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn7)), ((((((locals.var_rdsw_i_dn8 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn8)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn8)), ((((((locals.var_rdsw_i_dn9 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn9)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn9)), ((((((locals.var_rdsw_i_dn10 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn10)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn10)), ((((((locals.var_rdsw_i_dn11 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn11)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn11)), ((((((locals.var_rdsw_i_dn13 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn13)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn13)), ((((((locals.var_rdsw_i_dn14 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn14)) * locals.var_weffwrfactor) * locals.var_nfintotal) * locals.var_rdstemp) + (assign21140_e39288 * locals.var_rdstemp_dn14)),)
    } else {
        (locals.var_rdss, locals.var_rdss_dn0, locals.var_rdss_dn2, locals.var_rdss_dn3, locals.var_rdss_dn4, locals.var_rdss_dn5, locals.var_rdss_dn6, locals.var_rdss_dn7, locals.var_rdss_dn8, locals.var_rdss_dn9, locals.var_rdss_dn10, locals.var_rdss_dn11, locals.var_rdss_dn13, locals.var_rdss_dn14,)
    }
};
        locals.var_rdss = assign21140_e39292;
        locals.var_rdss_dn0 = assign21140_e39292_d_n0;
        locals.var_rdss_dn2 = assign21140_e39292_d_n2;
        locals.var_rdss_dn3 = assign21140_e39292_d_n3;
        locals.var_rdss_dn4 = assign21140_e39292_d_n4;
        locals.var_rdss_dn5 = assign21140_e39292_d_n5;
        locals.var_rdss_dn6 = assign21140_e39292_d_n6;
        locals.var_rdss_dn7 = assign21140_e39292_d_n7;
        locals.var_rdss_dn8 = assign21140_e39292_d_n8;
        locals.var_rdss_dn9 = assign21140_e39292_d_n9;
        locals.var_rdss_dn10 = assign21140_e39292_d_n10;
        locals.var_rdss_dn11 = assign21140_e39292_d_n11;
        locals.var_rdss_dn13 = assign21140_e39292_d_n13;
        locals.var_rdss_dn14 = assign21140_e39292_d_n14;

        let (assign21150_e39304, assign21150_e39304_d_n0, assign21150_e39304_d_n2, assign21150_e39304_d_n3, assign21150_e39304_d_n4, assign21150_e39304_d_n5, assign21150_e39304_d_n6, assign21150_e39304_d_n7, assign21150_e39304_d_n8, assign21150_e39304_d_n9, assign21150_e39304_d_n10, assign21150_e39304_d_n11, assign21150_e39304_d_n13, assign21150_e39304_d_n14,) = {
    if ((locals.var_guard370 == 0.0) && (locals.var_guard371 == 0.0)) {
        let assign21150_e39301: f64 = (locals.var_prwgs_i * locals.var_qis);
        let assign21150_e39302: f64 = (1.0 + assign21150_e39301);
        (assign21150_e39302, (locals.var_prwgs_i * locals.var_qis_dn0), (locals.var_prwgs_i * locals.var_qis_dn2), (locals.var_prwgs_i * locals.var_qis_dn3), (locals.var_prwgs_i * locals.var_qis_dn4), (locals.var_prwgs_i * locals.var_qis_dn5), (locals.var_prwgs_i * locals.var_qis_dn6), (locals.var_prwgs_i * locals.var_qis_dn7), (locals.var_prwgs_i * locals.var_qis_dn8), (locals.var_prwgs_i * locals.var_qis_dn9), (locals.var_prwgs_i * locals.var_qis_dn10), (locals.var_prwgs_i * locals.var_qis_dn11), (locals.var_prwgs_i * locals.var_qis_dn13), (locals.var_prwgs_i * locals.var_qis_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21150_e39304;
        locals.var_t4_dn0 = assign21150_e39304_d_n0;
        locals.var_t4_dn2 = assign21150_e39304_d_n2;
        locals.var_t4_dn3 = assign21150_e39304_d_n3;
        locals.var_t4_dn4 = assign21150_e39304_d_n4;
        locals.var_t4_dn5 = assign21150_e39304_d_n5;
        locals.var_t4_dn6 = assign21150_e39304_d_n6;
        locals.var_t4_dn7 = assign21150_e39304_d_n7;
        locals.var_t4_dn8 = assign21150_e39304_d_n8;
        locals.var_t4_dn9 = assign21150_e39304_d_n9;
        locals.var_t4_dn10 = assign21150_e39304_d_n10;
        locals.var_t4_dn11 = assign21150_e39304_d_n11;
        locals.var_t4_dn13 = assign21150_e39304_d_n13;
        locals.var_t4_dn14 = assign21150_e39304_d_n14;

        let (assign21160_e39314, assign21160_e39314_d_n0, assign21160_e39314_d_n2, assign21160_e39314_d_n3, assign21160_e39314_d_n4, assign21160_e39314_d_n5, assign21160_e39314_d_n6, assign21160_e39314_d_n7, assign21160_e39314_d_n8, assign21160_e39314_d_n9, assign21160_e39314_d_n10, assign21160_e39314_d_n11, assign21160_e39314_d_n13, assign21160_e39314_d_n14,) = {
    if ((locals.var_guard370 == 0.0) && (locals.var_guard371 == 0.0)) {
        let assign21160_e39312: f64 = (1.0 / locals.var_t4);
        (assign21160_e39312, (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn3 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn13 / (locals.var_t4 * locals.var_t4))), (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21160_e39314;
        locals.var_t1_dn0 = assign21160_e39314_d_n0;
        locals.var_t1_dn2 = assign21160_e39314_d_n2;
        locals.var_t1_dn3 = assign21160_e39314_d_n3;
        locals.var_t1_dn4 = assign21160_e39314_d_n4;
        locals.var_t1_dn5 = assign21160_e39314_d_n5;
        locals.var_t1_dn6 = assign21160_e39314_d_n6;
        locals.var_t1_dn7 = assign21160_e39314_d_n7;
        locals.var_t1_dn8 = assign21160_e39314_d_n8;
        locals.var_t1_dn9 = assign21160_e39314_d_n9;
        locals.var_t1_dn10 = assign21160_e39314_d_n10;
        locals.var_t1_dn11 = assign21160_e39314_d_n11;
        locals.var_t1_dn13 = assign21160_e39314_d_n13;
        locals.var_t1_dn14 = assign21160_e39314_d_n14;

        let (assign21170_e39331, assign21170_e39331_d_n0, assign21170_e39331_d_n2, assign21170_e39331_d_n3, assign21170_e39331_d_n4, assign21170_e39331_d_n5, assign21170_e39331_d_n6, assign21170_e39331_d_n7, assign21170_e39331_d_n8, assign21170_e39331_d_n9, assign21170_e39331_d_n10, assign21170_e39331_d_n11, assign21170_e39331_d_n13, assign21170_e39331_d_n14,) = {
    if ((locals.var_guard370 == 0.0) && (locals.var_guard371 == 0.0)) {
        let assign21170_e39324: f64 = (locals.var_t1 * locals.var_t1);
        let assign21170_e39326: f64 = (assign21170_e39324 + 0.01);
        let assign21170_e39327: f64 = (assign21170_e39326).sqrt();
        let assign21170_e39328: f64 = (locals.var_t1 + assign21170_e39327);
        let assign21170_e39329: f64 = (0.5 * assign21170_e39328);
        (assign21170_e39329, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign21170_e39327)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign21170_e39327)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21170_e39331;
        locals.var_t0_dn0 = assign21170_e39331_d_n0;
        locals.var_t0_dn2 = assign21170_e39331_d_n2;
        locals.var_t0_dn3 = assign21170_e39331_d_n3;
        locals.var_t0_dn4 = assign21170_e39331_d_n4;
        locals.var_t0_dn5 = assign21170_e39331_d_n5;
        locals.var_t0_dn6 = assign21170_e39331_d_n6;
        locals.var_t0_dn7 = assign21170_e39331_d_n7;
        locals.var_t0_dn8 = assign21170_e39331_d_n8;
        locals.var_t0_dn9 = assign21170_e39331_d_n9;
        locals.var_t0_dn10 = assign21170_e39331_d_n10;
        locals.var_t0_dn11 = assign21170_e39331_d_n11;
        locals.var_t0_dn13 = assign21170_e39331_d_n13;
        locals.var_t0_dn14 = assign21170_e39331_d_n14;

    }

    pub(super) fn stamp_transient_block_78(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21180_e39353, assign21180_e39353_d_n0, assign21180_e39353_d_n2, assign21180_e39353_d_n3, assign21180_e39353_d_n4, assign21180_e39353_d_n5, assign21180_e39353_d_n6, assign21180_e39353_d_n7, assign21180_e39353_d_n8, assign21180_e39353_d_n9, assign21180_e39353_d_n10, assign21180_e39353_d_n11, assign21180_e39353_d_n13, assign21180_e39353_d_n14,) = {
    if ((locals.var_guard370 == 0.0) && (locals.var_guard371 == 0.0)) {
        let assign21180_e39339: f64 = (locals.var_rsourcegeo + locals.var_rdraingeo);
        let assign21180_e39343: f64 = (locals.var_rdsw_i * locals.var_t0);
        let assign21180_e39344: f64 = (p.p908 + assign21180_e39343);
        let assign21180_e39346: f64 = (assign21180_e39344 * locals.var_weffwrfactor);
        let assign21180_e39348: f64 = (assign21180_e39346 * locals.var_nfintotal);
        let assign21180_e39349: f64 = (assign21180_e39339 + assign21180_e39348);
        let assign21180_e39351: f64 = (assign21180_e39349 * locals.var_rdstemp);
        (assign21180_e39351, ((((locals.var_rsourcegeo_dn0 + locals.var_rdraingeo_dn0) + ((((locals.var_rdsw_i_dn0 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn0)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn0)), ((((locals.var_rsourcegeo_dn2 + locals.var_rdraingeo_dn2) + ((((locals.var_rdsw_i_dn2 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn2)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn2)), ((((locals.var_rsourcegeo_dn3 + locals.var_rdraingeo_dn3) + ((((locals.var_rdsw_i_dn3 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn3)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn3)), ((((locals.var_rsourcegeo_dn4 + locals.var_rdraingeo_dn4) + ((((locals.var_rdsw_i_dn4 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn4)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn4)), ((((locals.var_rsourcegeo_dn5 + locals.var_rdraingeo_dn5) + ((((locals.var_rdsw_i_dn5 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn5)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn5)), ((((locals.var_rsourcegeo_dn6 + locals.var_rdraingeo_dn6) + ((((locals.var_rdsw_i_dn6 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn6)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn6)), ((((locals.var_rsourcegeo_dn7 + locals.var_rdraingeo_dn7) + ((((locals.var_rdsw_i_dn7 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn7)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn7)), ((((locals.var_rsourcegeo_dn8 + locals.var_rdraingeo_dn8) + ((((locals.var_rdsw_i_dn8 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn8)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn8)), ((((locals.var_rsourcegeo_dn9 + locals.var_rdraingeo_dn9) + ((((locals.var_rdsw_i_dn9 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn9)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn9)), ((((locals.var_rsourcegeo_dn10 + locals.var_rdraingeo_dn10) + ((((locals.var_rdsw_i_dn10 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn10)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn10)), ((((locals.var_rsourcegeo_dn11 + locals.var_rdraingeo_dn11) + ((((locals.var_rdsw_i_dn11 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn11)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn11)), ((((locals.var_rsourcegeo_dn13 + locals.var_rdraingeo_dn13) + ((((locals.var_rdsw_i_dn13 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn13)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn13)), ((((locals.var_rsourcegeo_dn14 + locals.var_rdraingeo_dn14) + ((((locals.var_rdsw_i_dn14 * locals.var_t0) + (locals.var_rdsw_i * locals.var_t0_dn14)) * locals.var_weffwrfactor) * locals.var_nfintotal)) * locals.var_rdstemp) + (assign21180_e39349 * locals.var_rdstemp_dn14)),)
    } else {
        (locals.var_rdss, locals.var_rdss_dn0, locals.var_rdss_dn2, locals.var_rdss_dn3, locals.var_rdss_dn4, locals.var_rdss_dn5, locals.var_rdss_dn6, locals.var_rdss_dn7, locals.var_rdss_dn8, locals.var_rdss_dn9, locals.var_rdss_dn10, locals.var_rdss_dn11, locals.var_rdss_dn13, locals.var_rdss_dn14,)
    }
};
        locals.var_rdss = assign21180_e39353;
        locals.var_rdss_dn0 = assign21180_e39353_d_n0;
        locals.var_rdss_dn2 = assign21180_e39353_d_n2;
        locals.var_rdss_dn3 = assign21180_e39353_d_n3;
        locals.var_rdss_dn4 = assign21180_e39353_d_n4;
        locals.var_rdss_dn5 = assign21180_e39353_d_n5;
        locals.var_rdss_dn6 = assign21180_e39353_d_n6;
        locals.var_rdss_dn7 = assign21180_e39353_d_n7;
        locals.var_rdss_dn8 = assign21180_e39353_d_n8;
        locals.var_rdss_dn9 = assign21180_e39353_d_n9;
        locals.var_rdss_dn10 = assign21180_e39353_d_n10;
        locals.var_rdss_dn11 = assign21180_e39353_d_n11;
        locals.var_rdss_dn13 = assign21180_e39353_d_n13;
        locals.var_rdss_dn14 = assign21180_e39353_d_n14;

        let assign21190_e39356: f64 = (2.0 * locals.var_vsat_a);
        let assign21190_e39358: f64 = (assign21190_e39356 / locals.var_u0_a);
        let assign21190_e39360: f64 = (assign21190_e39358 * locals.var_dmobs);
        locals.var_esat = assign21190_e39360;
        locals.var_esat_dn0 = ((((((2.0 * locals.var_vsat_a_dn0) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn0)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn0));
        locals.var_esat_dn2 = ((((((2.0 * locals.var_vsat_a_dn2) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn2)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn2));
        locals.var_esat_dn3 = ((((((2.0 * locals.var_vsat_a_dn3) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn3)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn3));
        locals.var_esat_dn4 = ((((((2.0 * locals.var_vsat_a_dn4) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn4)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn4));
        locals.var_esat_dn5 = ((((((2.0 * locals.var_vsat_a_dn5) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn5)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn5));
        locals.var_esat_dn6 = ((((((2.0 * locals.var_vsat_a_dn6) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn6)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn6));
        locals.var_esat_dn7 = ((((((2.0 * locals.var_vsat_a_dn7) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn7)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn7));
        locals.var_esat_dn8 = ((((((2.0 * locals.var_vsat_a_dn8) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn8)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn8));
        locals.var_esat_dn9 = ((((((2.0 * locals.var_vsat_a_dn9) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn9)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn9));
        locals.var_esat_dn10 = ((((((2.0 * locals.var_vsat_a_dn10) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn10)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn10));
        locals.var_esat_dn11 = ((((((2.0 * locals.var_vsat_a_dn11) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn11)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn11));
        locals.var_esat_dn13 = ((((((2.0 * locals.var_vsat_a_dn13) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn13)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn13));
        locals.var_esat_dn14 = ((((((2.0 * locals.var_vsat_a_dn14) * locals.var_u0_a) - (assign21190_e39356 * locals.var_u0_a_dn14)) / (locals.var_u0_a * locals.var_u0_a)) * locals.var_dmobs) + (assign21190_e39358 * locals.var_dmobs_dn14));

        let assign21200_e39363: f64 = (locals.var_esat * locals.var_leff_1);
        locals.var_esatl = assign21200_e39363;
        locals.var_esatl_dn0 = ((locals.var_esat_dn0 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn0));
        locals.var_esatl_dn2 = ((locals.var_esat_dn2 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn2));
        locals.var_esatl_dn3 = ((locals.var_esat_dn3 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn3));
        locals.var_esatl_dn4 = ((locals.var_esat_dn4 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn4));
        locals.var_esatl_dn5 = ((locals.var_esat_dn5 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn5));
        locals.var_esatl_dn6 = ((locals.var_esat_dn6 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn6));
        locals.var_esatl_dn7 = ((locals.var_esat_dn7 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn7));
        locals.var_esatl_dn8 = ((locals.var_esat_dn8 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn8));
        locals.var_esatl_dn9 = ((locals.var_esat_dn9 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn9));
        locals.var_esatl_dn10 = ((locals.var_esat_dn10 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn10));
        locals.var_esatl_dn11 = ((locals.var_esat_dn11 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn11));
        locals.var_esatl_dn13 = ((locals.var_esat_dn13 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn13));
        locals.var_esatl_dn14 = ((locals.var_esat_dn14 * locals.var_leff_1) + (locals.var_esat * locals.var_leff_1_dn14));

        let assign21210_e39366: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard372 = assign21210_e39366;

        let (assign21220_e39376, assign21220_e39376_d_n0, assign21220_e39376_d_n2, assign21220_e39376_d_n3, assign21220_e39376_d_n4, assign21220_e39376_d_n5, assign21220_e39376_d_n6, assign21220_e39376_d_n7, assign21220_e39376_d_n8, assign21220_e39376_d_n9, assign21220_e39376_d_n10, assign21220_e39376_d_n11, assign21220_e39376_d_n13, assign21220_e39376_d_n14,) = {
    if (locals.var_guard372 != 0.0) {
        let assign21220_e39372: f64 = (2.0 * locals.var_vtm);
        let assign21220_e39373: f64 = (locals.var_qis + assign21220_e39372);
        let assign21220_e39374: f64 = (locals.var_ksativ_a * assign21220_e39373);
        (assign21220_e39374, ((locals.var_ksativ_a_dn0 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn0)), ((locals.var_ksativ_a_dn2 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn2)), ((locals.var_ksativ_a_dn3 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn3)), ((locals.var_ksativ_a_dn4 * assign21220_e39373) + (locals.var_ksativ_a * (locals.var_qis_dn4 + (2.0 * locals.var_vtm_dn4)))), ((locals.var_ksativ_a_dn5 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn5)), ((locals.var_ksativ_a_dn6 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn6)), ((locals.var_ksativ_a_dn7 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn7)), ((locals.var_ksativ_a_dn8 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn8)), ((locals.var_ksativ_a_dn9 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn9)), ((locals.var_ksativ_a_dn10 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn10)), ((locals.var_ksativ_a_dn11 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn11)), ((locals.var_ksativ_a_dn13 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn13)), ((locals.var_ksativ_a_dn14 * assign21220_e39373) + (locals.var_ksativ_a * locals.var_qis_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign21220_e39376;
        locals.var_t6_dn0 = assign21220_e39376_d_n0;
        locals.var_t6_dn2 = assign21220_e39376_d_n2;
        locals.var_t6_dn3 = assign21220_e39376_d_n3;
        locals.var_t6_dn4 = assign21220_e39376_d_n4;
        locals.var_t6_dn5 = assign21220_e39376_d_n5;
        locals.var_t6_dn6 = assign21220_e39376_d_n6;
        locals.var_t6_dn7 = assign21220_e39376_d_n7;
        locals.var_t6_dn8 = assign21220_e39376_d_n8;
        locals.var_t6_dn9 = assign21220_e39376_d_n9;
        locals.var_t6_dn10 = assign21220_e39376_d_n10;
        locals.var_t6_dn11 = assign21220_e39376_d_n11;
        locals.var_t6_dn13 = assign21220_e39376_d_n13;
        locals.var_t6_dn14 = assign21220_e39376_d_n14;

        let (assign21230_e39387, assign21230_e39387_d_n0, assign21230_e39387_d_n2, assign21230_e39387_d_n3, assign21230_e39387_d_n4, assign21230_e39387_d_n5, assign21230_e39387_d_n6, assign21230_e39387_d_n7, assign21230_e39387_d_n8, assign21230_e39387_d_n9, assign21230_e39387_d_n10, assign21230_e39387_d_n11, assign21230_e39387_d_n13, assign21230_e39387_d_n14,) = {
    if (locals.var_guard372 == 0.0) {
        let assign21230_e39383: f64 = (2.0 * locals.var_vtmeff);
        let assign21230_e39384: f64 = (locals.var_qis + assign21230_e39383);
        let assign21230_e39385: f64 = (locals.var_ksativ_a * assign21230_e39384);
        (assign21230_e39385, ((locals.var_ksativ_a_dn0 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn0 + (2.0 * locals.var_vtmeff_dn0)))), ((locals.var_ksativ_a_dn2 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn2 + (2.0 * locals.var_vtmeff_dn2)))), ((locals.var_ksativ_a_dn3 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn3 + (2.0 * locals.var_vtmeff_dn3)))), ((locals.var_ksativ_a_dn4 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn4 + (2.0 * locals.var_vtmeff_dn4)))), ((locals.var_ksativ_a_dn5 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn5 + (2.0 * locals.var_vtmeff_dn5)))), ((locals.var_ksativ_a_dn6 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn6 + (2.0 * locals.var_vtmeff_dn6)))), ((locals.var_ksativ_a_dn7 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn7 + (2.0 * locals.var_vtmeff_dn7)))), ((locals.var_ksativ_a_dn8 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn8 + (2.0 * locals.var_vtmeff_dn8)))), ((locals.var_ksativ_a_dn9 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn9 + (2.0 * locals.var_vtmeff_dn9)))), ((locals.var_ksativ_a_dn10 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn10 + (2.0 * locals.var_vtmeff_dn10)))), ((locals.var_ksativ_a_dn11 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn11 + (2.0 * locals.var_vtmeff_dn11)))), ((locals.var_ksativ_a_dn13 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn13 + (2.0 * locals.var_vtmeff_dn13)))), ((locals.var_ksativ_a_dn14 * assign21230_e39384) + (locals.var_ksativ_a * (locals.var_qis_dn14 + (2.0 * locals.var_vtmeff_dn14)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign21230_e39387;
        locals.var_t6_dn0 = assign21230_e39387_d_n0;
        locals.var_t6_dn2 = assign21230_e39387_d_n2;
        locals.var_t6_dn3 = assign21230_e39387_d_n3;
        locals.var_t6_dn4 = assign21230_e39387_d_n4;
        locals.var_t6_dn5 = assign21230_e39387_d_n5;
        locals.var_t6_dn6 = assign21230_e39387_d_n6;
        locals.var_t6_dn7 = assign21230_e39387_d_n7;
        locals.var_t6_dn8 = assign21230_e39387_d_n8;
        locals.var_t6_dn9 = assign21230_e39387_d_n9;
        locals.var_t6_dn10 = assign21230_e39387_d_n10;
        locals.var_t6_dn11 = assign21230_e39387_d_n11;
        locals.var_t6_dn13 = assign21230_e39387_d_n13;
        locals.var_t6_dn14 = assign21230_e39387_d_n14;

        let assign21240_e39390: f64 = if locals.var_rdss > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard373 = assign21240_e39390;

        let (assign21250_e39398, assign21250_e39398_d_n0, assign21250_e39398_d_n2, assign21250_e39398_d_n3, assign21250_e39398_d_n4, assign21250_e39398_d_n5, assign21250_e39398_d_n6, assign21250_e39398_d_n7, assign21250_e39398_d_n8, assign21250_e39398_d_n9, assign21250_e39398_d_n10, assign21250_e39398_d_n11, assign21250_e39398_d_n13, assign21250_e39398_d_n14,) = {
    if (locals.var_guard373 != 0.0) {
        let assign21250_e39394: f64 = (locals.var_weff0 * locals.var_vsat_a);
        let assign21250_e39396: f64 = (assign21250_e39394 * locals.var_cox);
        (assign21250_e39396, ((locals.var_weff0 * locals.var_vsat_a_dn0) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn2) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn3) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn4) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn5) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn6) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn7) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn8) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn9) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn10) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn11) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn13) * locals.var_cox), ((locals.var_weff0 * locals.var_vsat_a_dn14) * locals.var_cox),)
    } else {
        (locals.var_wvcox, locals.var_wvcox_dn0, locals.var_wvcox_dn2, locals.var_wvcox_dn3, locals.var_wvcox_dn4, locals.var_wvcox_dn5, locals.var_wvcox_dn6, locals.var_wvcox_dn7, locals.var_wvcox_dn8, locals.var_wvcox_dn9, locals.var_wvcox_dn10, locals.var_wvcox_dn11, locals.var_wvcox_dn13, locals.var_wvcox_dn14,)
    }
};
        locals.var_wvcox = assign21250_e39398;
        locals.var_wvcox_dn0 = assign21250_e39398_d_n0;
        locals.var_wvcox_dn2 = assign21250_e39398_d_n2;
        locals.var_wvcox_dn3 = assign21250_e39398_d_n3;
        locals.var_wvcox_dn4 = assign21250_e39398_d_n4;
        locals.var_wvcox_dn5 = assign21250_e39398_d_n5;
        locals.var_wvcox_dn6 = assign21250_e39398_d_n6;
        locals.var_wvcox_dn7 = assign21250_e39398_d_n7;
        locals.var_wvcox_dn8 = assign21250_e39398_d_n8;
        locals.var_wvcox_dn9 = assign21250_e39398_d_n9;
        locals.var_wvcox_dn10 = assign21250_e39398_d_n10;
        locals.var_wvcox_dn11 = assign21250_e39398_d_n11;
        locals.var_wvcox_dn13 = assign21250_e39398_d_n13;
        locals.var_wvcox_dn14 = assign21250_e39398_d_n14;

        let (assign21260_e39404, assign21260_e39404_d_n0, assign21260_e39404_d_n2, assign21260_e39404_d_n3, assign21260_e39404_d_n4, assign21260_e39404_d_n5, assign21260_e39404_d_n6, assign21260_e39404_d_n7, assign21260_e39404_d_n8, assign21260_e39404_d_n9, assign21260_e39404_d_n10, assign21260_e39404_d_n11, assign21260_e39404_d_n13, assign21260_e39404_d_n14,) = {
    if (locals.var_guard373 != 0.0) {
        let assign21260_e39402: f64 = (locals.var_wvcox * locals.var_rdss);
        (assign21260_e39402, ((locals.var_wvcox_dn0 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn0)), ((locals.var_wvcox_dn2 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn2)), ((locals.var_wvcox_dn3 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn3)), ((locals.var_wvcox_dn4 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn4)), ((locals.var_wvcox_dn5 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn5)), ((locals.var_wvcox_dn6 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn6)), ((locals.var_wvcox_dn7 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn7)), ((locals.var_wvcox_dn8 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn8)), ((locals.var_wvcox_dn9 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn9)), ((locals.var_wvcox_dn10 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn10)), ((locals.var_wvcox_dn11 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn11)), ((locals.var_wvcox_dn13 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn13)), ((locals.var_wvcox_dn14 * locals.var_rdss) + (locals.var_wvcox * locals.var_rdss_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21260_e39404;
        locals.var_t0_dn0 = assign21260_e39404_d_n0;
        locals.var_t0_dn2 = assign21260_e39404_d_n2;
        locals.var_t0_dn3 = assign21260_e39404_d_n3;
        locals.var_t0_dn4 = assign21260_e39404_d_n4;
        locals.var_t0_dn5 = assign21260_e39404_d_n5;
        locals.var_t0_dn6 = assign21260_e39404_d_n6;
        locals.var_t0_dn7 = assign21260_e39404_d_n7;
        locals.var_t0_dn8 = assign21260_e39404_d_n8;
        locals.var_t0_dn9 = assign21260_e39404_d_n9;
        locals.var_t0_dn10 = assign21260_e39404_d_n10;
        locals.var_t0_dn11 = assign21260_e39404_d_n11;
        locals.var_t0_dn13 = assign21260_e39404_d_n13;
        locals.var_t0_dn14 = assign21260_e39404_d_n14;

        let (assign21270_e39410, assign21270_e39410_d_n0, assign21270_e39410_d_n2, assign21270_e39410_d_n3, assign21270_e39410_d_n4, assign21270_e39410_d_n5, assign21270_e39410_d_n6, assign21270_e39410_d_n7, assign21270_e39410_d_n8, assign21270_e39410_d_n9, assign21270_e39410_d_n10, assign21270_e39410_d_n11, assign21270_e39410_d_n13, assign21270_e39410_d_n14,) = {
    if (locals.var_guard373 != 0.0) {
        let assign21270_e39408: f64 = (2.0 * locals.var_t0);
        (assign21270_e39408, (2.0 * locals.var_t0_dn0), (2.0 * locals.var_t0_dn2), (2.0 * locals.var_t0_dn3), (2.0 * locals.var_t0_dn4), (2.0 * locals.var_t0_dn5), (2.0 * locals.var_t0_dn6), (2.0 * locals.var_t0_dn7), (2.0 * locals.var_t0_dn8), (2.0 * locals.var_t0_dn9), (2.0 * locals.var_t0_dn10), (2.0 * locals.var_t0_dn11), (2.0 * locals.var_t0_dn13), (2.0 * locals.var_t0_dn14),)
    } else {
        (locals.var_ta, locals.var_ta_dn0, locals.var_ta_dn2, locals.var_ta_dn3, locals.var_ta_dn4, locals.var_ta_dn5, locals.var_ta_dn6, locals.var_ta_dn7, locals.var_ta_dn8, locals.var_ta_dn9, locals.var_ta_dn10, locals.var_ta_dn11, locals.var_ta_dn13, locals.var_ta_dn14,)
    }
};
        locals.var_ta = assign21270_e39410;
        locals.var_ta_dn0 = assign21270_e39410_d_n0;
        locals.var_ta_dn2 = assign21270_e39410_d_n2;
        locals.var_ta_dn3 = assign21270_e39410_d_n3;
        locals.var_ta_dn4 = assign21270_e39410_d_n4;
        locals.var_ta_dn5 = assign21270_e39410_d_n5;
        locals.var_ta_dn6 = assign21270_e39410_d_n6;
        locals.var_ta_dn7 = assign21270_e39410_d_n7;
        locals.var_ta_dn8 = assign21270_e39410_d_n8;
        locals.var_ta_dn9 = assign21270_e39410_d_n9;
        locals.var_ta_dn10 = assign21270_e39410_d_n10;
        locals.var_ta_dn11 = assign21270_e39410_d_n11;
        locals.var_ta_dn13 = assign21270_e39410_d_n13;
        locals.var_ta_dn14 = assign21270_e39410_d_n14;

        let (assign21280_e39422, assign21280_e39422_d_n0, assign21280_e39422_d_n2, assign21280_e39422_d_n3, assign21280_e39422_d_n4, assign21280_e39422_d_n5, assign21280_e39422_d_n6, assign21280_e39422_d_n7, assign21280_e39422_d_n8, assign21280_e39422_d_n9, assign21280_e39422_d_n10, assign21280_e39422_d_n11, assign21280_e39422_d_n13, assign21280_e39422_d_n14,) = {
    if (locals.var_guard373 != 0.0) {
        let assign21280_e39414: f64 = (locals.var_t6 + locals.var_esatl);
        let assign21280_e39417: f64 = (3.0 * locals.var_t6);
        let assign21280_e39419: f64 = (assign21280_e39417 * locals.var_t0);
        let assign21280_e39420: f64 = (assign21280_e39414 + assign21280_e39419);
        (assign21280_e39420, ((locals.var_t6_dn0 + locals.var_esatl_dn0) + (((3.0 * locals.var_t6_dn0) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn0))), ((locals.var_t6_dn2 + locals.var_esatl_dn2) + (((3.0 * locals.var_t6_dn2) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn2))), ((locals.var_t6_dn3 + locals.var_esatl_dn3) + (((3.0 * locals.var_t6_dn3) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn3))), ((locals.var_t6_dn4 + locals.var_esatl_dn4) + (((3.0 * locals.var_t6_dn4) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn4))), ((locals.var_t6_dn5 + locals.var_esatl_dn5) + (((3.0 * locals.var_t6_dn5) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn5))), ((locals.var_t6_dn6 + locals.var_esatl_dn6) + (((3.0 * locals.var_t6_dn6) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn6))), ((locals.var_t6_dn7 + locals.var_esatl_dn7) + (((3.0 * locals.var_t6_dn7) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn7))), ((locals.var_t6_dn8 + locals.var_esatl_dn8) + (((3.0 * locals.var_t6_dn8) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn8))), ((locals.var_t6_dn9 + locals.var_esatl_dn9) + (((3.0 * locals.var_t6_dn9) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn9))), ((locals.var_t6_dn10 + locals.var_esatl_dn10) + (((3.0 * locals.var_t6_dn10) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn10))), ((locals.var_t6_dn11 + locals.var_esatl_dn11) + (((3.0 * locals.var_t6_dn11) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn11))), ((locals.var_t6_dn13 + locals.var_esatl_dn13) + (((3.0 * locals.var_t6_dn13) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn13))), ((locals.var_t6_dn14 + locals.var_esatl_dn14) + (((3.0 * locals.var_t6_dn14) * locals.var_t0) + (assign21280_e39417 * locals.var_t0_dn14))),)
    } else {
        (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn13, locals.var_tb_dn14,)
    }
};
        locals.var_tb = assign21280_e39422;
        locals.var_tb_dn0 = assign21280_e39422_d_n0;
        locals.var_tb_dn2 = assign21280_e39422_d_n2;
        locals.var_tb_dn3 = assign21280_e39422_d_n3;
        locals.var_tb_dn4 = assign21280_e39422_d_n4;
        locals.var_tb_dn5 = assign21280_e39422_d_n5;
        locals.var_tb_dn6 = assign21280_e39422_d_n6;
        locals.var_tb_dn7 = assign21280_e39422_d_n7;
        locals.var_tb_dn8 = assign21280_e39422_d_n8;
        locals.var_tb_dn9 = assign21280_e39422_d_n9;
        locals.var_tb_dn10 = assign21280_e39422_d_n10;
        locals.var_tb_dn11 = assign21280_e39422_d_n11;
        locals.var_tb_dn13 = assign21280_e39422_d_n13;
        locals.var_tb_dn14 = assign21280_e39422_d_n14;

        let (assign21290_e39434, assign21290_e39434_d_n0, assign21290_e39434_d_n2, assign21290_e39434_d_n3, assign21290_e39434_d_n4, assign21290_e39434_d_n5, assign21290_e39434_d_n6, assign21290_e39434_d_n7, assign21290_e39434_d_n8, assign21290_e39434_d_n9, assign21290_e39434_d_n10, assign21290_e39434_d_n11, assign21290_e39434_d_n13, assign21290_e39434_d_n14,) = {
    if (locals.var_guard373 != 0.0) {
        let assign21290_e39428: f64 = (2.0 * locals.var_t6);
        let assign21290_e39430: f64 = (assign21290_e39428 * locals.var_t0);
        let assign21290_e39431: f64 = (locals.var_esatl + assign21290_e39430);
        let assign21290_e39432: f64 = (locals.var_t6 * assign21290_e39431);
        (assign21290_e39432, ((locals.var_t6_dn0 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn0 + (((2.0 * locals.var_t6_dn0) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn0))))), ((locals.var_t6_dn2 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn2 + (((2.0 * locals.var_t6_dn2) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn2))))), ((locals.var_t6_dn3 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn3 + (((2.0 * locals.var_t6_dn3) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn3))))), ((locals.var_t6_dn4 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn4 + (((2.0 * locals.var_t6_dn4) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn4))))), ((locals.var_t6_dn5 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn5 + (((2.0 * locals.var_t6_dn5) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn5))))), ((locals.var_t6_dn6 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn6 + (((2.0 * locals.var_t6_dn6) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn6))))), ((locals.var_t6_dn7 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn7 + (((2.0 * locals.var_t6_dn7) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn7))))), ((locals.var_t6_dn8 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn8 + (((2.0 * locals.var_t6_dn8) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn8))))), ((locals.var_t6_dn9 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn9 + (((2.0 * locals.var_t6_dn9) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn9))))), ((locals.var_t6_dn10 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn10 + (((2.0 * locals.var_t6_dn10) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn10))))), ((locals.var_t6_dn11 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn11 + (((2.0 * locals.var_t6_dn11) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn11))))), ((locals.var_t6_dn13 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn13 + (((2.0 * locals.var_t6_dn13) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn13))))), ((locals.var_t6_dn14 * assign21290_e39431) + (locals.var_t6 * (locals.var_esatl_dn14 + (((2.0 * locals.var_t6_dn14) * locals.var_t0) + (assign21290_e39428 * locals.var_t0_dn14))))),)
    } else {
        (locals.var_tc, locals.var_tc_dn0, locals.var_tc_dn2, locals.var_tc_dn3, locals.var_tc_dn4, locals.var_tc_dn5, locals.var_tc_dn6, locals.var_tc_dn7, locals.var_tc_dn8, locals.var_tc_dn9, locals.var_tc_dn10, locals.var_tc_dn11, locals.var_tc_dn13, locals.var_tc_dn14,)
    }
};
        locals.var_tc = assign21290_e39434;
        locals.var_tc_dn0 = assign21290_e39434_d_n0;
        locals.var_tc_dn2 = assign21290_e39434_d_n2;
        locals.var_tc_dn3 = assign21290_e39434_d_n3;
        locals.var_tc_dn4 = assign21290_e39434_d_n4;
        locals.var_tc_dn5 = assign21290_e39434_d_n5;
        locals.var_tc_dn6 = assign21290_e39434_d_n6;
        locals.var_tc_dn7 = assign21290_e39434_d_n7;
        locals.var_tc_dn8 = assign21290_e39434_d_n8;
        locals.var_tc_dn9 = assign21290_e39434_d_n9;
        locals.var_tc_dn10 = assign21290_e39434_d_n10;
        locals.var_tc_dn11 = assign21290_e39434_d_n11;
        locals.var_tc_dn13 = assign21290_e39434_d_n13;
        locals.var_tc_dn14 = assign21290_e39434_d_n14;

        let (assign21300_e39465, assign21300_e39465_d_n0, assign21300_e39465_d_n2, assign21300_e39465_d_n3, assign21300_e39465_d_n4, assign21300_e39465_d_n5, assign21300_e39465_d_n6, assign21300_e39465_d_n7, assign21300_e39465_d_n8, assign21300_e39465_d_n9, assign21300_e39465_d_n10, assign21300_e39465_d_n11, assign21300_e39465_d_n13, assign21300_e39465_d_n14,) = {
    if (locals.var_guard373 != 0.0) {
        let assign21300_e39438: f64 = (locals.var_tb * locals.var_tb);
        let assign21300_e39441: f64 = (locals.var_tb * locals.var_tb);
        let assign21300_e39444: f64 = (2.0 * locals.var_ta);
        let assign21300_e39446: f64 = (assign21300_e39444 * locals.var_tc);
        let assign21300_e39447: f64 = (assign21300_e39441 - assign21300_e39446);
        let assign21300_e39448: f64 = (assign21300_e39438 - assign21300_e39447);
        let assign21300_e39452: f64 = (locals.var_tb * locals.var_tb);
        let assign21300_e39455: f64 = (2.0 * locals.var_ta);
        let assign21300_e39457: f64 = (assign21300_e39455 * locals.var_tc);
        let assign21300_e39458: f64 = (assign21300_e39452 - assign21300_e39457);
        let assign21300_e39459: f64 = (assign21300_e39458).sqrt();
        let assign21300_e39460: f64 = (locals.var_tb + assign21300_e39459);
        let assign21300_e39462: f64 = (assign21300_e39460 * locals.var_ta);
        let assign21300_e39463: f64 = (assign21300_e39448 / assign21300_e39462);
        (assign21300_e39463, ((((((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) - (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) - (((2.0 * locals.var_ta_dn0) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn0)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn0 + ((((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) - (((2.0 * locals.var_ta_dn0) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn0))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn0)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) - (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) - (((2.0 * locals.var_ta_dn2) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn2)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn2 + ((((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) - (((2.0 * locals.var_ta_dn2) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn2))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn2)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) - (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) - (((2.0 * locals.var_ta_dn3) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn3)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn3 + ((((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) - (((2.0 * locals.var_ta_dn3) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn3))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn3)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) - (((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) - (((2.0 * locals.var_ta_dn4) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn4)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) - (((2.0 * locals.var_ta_dn4) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn4))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn4)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) - (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) - (((2.0 * locals.var_ta_dn5) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn5)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn5 + ((((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) - (((2.0 * locals.var_ta_dn5) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn5))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn5)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) - (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) - (((2.0 * locals.var_ta_dn6) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn6)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn6 + ((((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) - (((2.0 * locals.var_ta_dn6) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn6))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn6)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) - (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) - (((2.0 * locals.var_ta_dn7) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn7)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn7 + ((((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) - (((2.0 * locals.var_ta_dn7) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn7))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn7)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) - (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) - (((2.0 * locals.var_ta_dn8) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn8)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn8 + ((((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) - (((2.0 * locals.var_ta_dn8) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn8))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn8)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) - (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) - (((2.0 * locals.var_ta_dn9) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn9)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn9 + ((((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) - (((2.0 * locals.var_ta_dn9) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn9))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn9)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) - (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) - (((2.0 * locals.var_ta_dn10) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn10)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn10 + ((((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) - (((2.0 * locals.var_ta_dn10) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn10))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn10)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) - (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) - (((2.0 * locals.var_ta_dn11) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn11)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn11 + ((((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) - (((2.0 * locals.var_ta_dn11) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn11))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn11)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) - (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) - (((2.0 * locals.var_ta_dn13) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn13)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn13 + ((((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) - (((2.0 * locals.var_ta_dn13) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn13))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn13)))) / (assign21300_e39462 * assign21300_e39462)), ((((((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) - (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) - (((2.0 * locals.var_ta_dn14) * locals.var_tc) + (assign21300_e39444 * locals.var_tc_dn14)))) * assign21300_e39462) - (assign21300_e39448 * (((locals.var_tb_dn14 + ((((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) - (((2.0 * locals.var_ta_dn14) * locals.var_tc) + (assign21300_e39455 * locals.var_tc_dn14))) / (2.0 * assign21300_e39459))) * locals.var_ta) + (assign21300_e39460 * locals.var_ta_dn14)))) / (assign21300_e39462 * assign21300_e39462)),)
    } else {
        (locals.var_vdsat, locals.var_vdsat_dn0, locals.var_vdsat_dn2, locals.var_vdsat_dn3, locals.var_vdsat_dn4, locals.var_vdsat_dn5, locals.var_vdsat_dn6, locals.var_vdsat_dn7, locals.var_vdsat_dn8, locals.var_vdsat_dn9, locals.var_vdsat_dn10, locals.var_vdsat_dn11, locals.var_vdsat_dn13, locals.var_vdsat_dn14,)
    }
};
        locals.var_vdsat = assign21300_e39465;
        locals.var_vdsat_dn0 = assign21300_e39465_d_n0;
        locals.var_vdsat_dn2 = assign21300_e39465_d_n2;
        locals.var_vdsat_dn3 = assign21300_e39465_d_n3;
        locals.var_vdsat_dn4 = assign21300_e39465_d_n4;
        locals.var_vdsat_dn5 = assign21300_e39465_d_n5;
        locals.var_vdsat_dn6 = assign21300_e39465_d_n6;
        locals.var_vdsat_dn7 = assign21300_e39465_d_n7;
        locals.var_vdsat_dn8 = assign21300_e39465_d_n8;
        locals.var_vdsat_dn9 = assign21300_e39465_d_n9;
        locals.var_vdsat_dn10 = assign21300_e39465_d_n10;
        locals.var_vdsat_dn11 = assign21300_e39465_d_n11;
        locals.var_vdsat_dn13 = assign21300_e39465_d_n13;
        locals.var_vdsat_dn14 = assign21300_e39465_d_n14;

        let (assign21310_e39476, assign21310_e39476_d_n0, assign21310_e39476_d_n2, assign21310_e39476_d_n3, assign21310_e39476_d_n4, assign21310_e39476_d_n5, assign21310_e39476_d_n6, assign21310_e39476_d_n7, assign21310_e39476_d_n8, assign21310_e39476_d_n9, assign21310_e39476_d_n10, assign21310_e39476_d_n11, assign21310_e39476_d_n13, assign21310_e39476_d_n14,) = {
    if (locals.var_guard373 == 0.0) {
        let assign21310_e39470: f64 = (locals.var_esatl * locals.var_t6);
        let assign21310_e39473: f64 = (locals.var_esatl + locals.var_t6);
        let assign21310_e39474: f64 = (assign21310_e39470 / assign21310_e39473);
        (assign21310_e39474, (((((locals.var_esatl_dn0 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn0)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn0 + locals.var_t6_dn0))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn2 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn2)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn2 + locals.var_t6_dn2))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn3 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn3)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn3 + locals.var_t6_dn3))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn4 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn4)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn4 + locals.var_t6_dn4))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn5 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn5)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn5 + locals.var_t6_dn5))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn6 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn6)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn6 + locals.var_t6_dn6))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn7 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn7)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn7 + locals.var_t6_dn7))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn8 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn8)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn8 + locals.var_t6_dn8))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn9 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn9)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn9 + locals.var_t6_dn9))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn10 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn10)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn10 + locals.var_t6_dn10))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn11 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn11)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn11 + locals.var_t6_dn11))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn13 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn13)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn13 + locals.var_t6_dn13))) / (assign21310_e39473 * assign21310_e39473)), (((((locals.var_esatl_dn14 * locals.var_t6) + (locals.var_esatl * locals.var_t6_dn14)) * assign21310_e39473) - (assign21310_e39470 * (locals.var_esatl_dn14 + locals.var_t6_dn14))) / (assign21310_e39473 * assign21310_e39473)),)
    } else {
        (locals.var_vdsat, locals.var_vdsat_dn0, locals.var_vdsat_dn2, locals.var_vdsat_dn3, locals.var_vdsat_dn4, locals.var_vdsat_dn5, locals.var_vdsat_dn6, locals.var_vdsat_dn7, locals.var_vdsat_dn8, locals.var_vdsat_dn9, locals.var_vdsat_dn10, locals.var_vdsat_dn11, locals.var_vdsat_dn13, locals.var_vdsat_dn14,)
    }
};
        locals.var_vdsat = assign21310_e39476;
        locals.var_vdsat_dn0 = assign21310_e39476_d_n0;
        locals.var_vdsat_dn2 = assign21310_e39476_d_n2;
        locals.var_vdsat_dn3 = assign21310_e39476_d_n3;
        locals.var_vdsat_dn4 = assign21310_e39476_d_n4;
        locals.var_vdsat_dn5 = assign21310_e39476_d_n5;
        locals.var_vdsat_dn6 = assign21310_e39476_d_n6;
        locals.var_vdsat_dn7 = assign21310_e39476_d_n7;
        locals.var_vdsat_dn8 = assign21310_e39476_d_n8;
        locals.var_vdsat_dn9 = assign21310_e39476_d_n9;
        locals.var_vdsat_dn10 = assign21310_e39476_d_n10;
        locals.var_vdsat_dn11 = assign21310_e39476_d_n11;
        locals.var_vdsat_dn13 = assign21310_e39476_d_n13;
        locals.var_vdsat_dn14 = assign21310_e39476_d_n14;

        let assign21320_e39479: f64 = (locals.var_vdsat - 0.001);
        let assign21320_e39481: f64 = (-10000.0);
        let assign21320_e39483: f64 = (assign21320_e39481 * 1e-5);
        let (assign21320_e39524, assign21320_e39524_d_n0, assign21320_e39524_d_n2, assign21320_e39524_d_n3, assign21320_e39524_d_n4, assign21320_e39524_d_n5, assign21320_e39524_d_n6, assign21320_e39524_d_n7, assign21320_e39524_d_n8, assign21320_e39524_d_n9, assign21320_e39524_d_n10, assign21320_e39524_d_n11, assign21320_e39524_d_n13, assign21320_e39524_d_n14,) = {
    if (!(assign21320_e39479 < assign21320_e39483)) {
        let assign21320_e39489: f64 = (locals.var_vdsat - 0.001);
        let assign21320_e39492: f64 = (locals.var_vdsat - 0.001);
        let assign21320_e39495: f64 = (locals.var_vdsat - 0.001);
        let assign21320_e39496: f64 = (assign21320_e39492 * assign21320_e39495);
        let assign21320_e39499: f64 = (4.0 * 1e-5);
        let assign21320_e39501: f64 = (assign21320_e39499 * 1e-5);
        let assign21320_e39502: f64 = (assign21320_e39496 + assign21320_e39501);
        let assign21320_e39503: f64 = (assign21320_e39502).sqrt();
        let assign21320_e39504: f64 = (assign21320_e39489 + assign21320_e39503);
        let assign21320_e39505: f64 = (0.5 * assign21320_e39504);
        (assign21320_e39505, (0.5 * (locals.var_vdsat_dn0 + (((locals.var_vdsat_dn0 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn0)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn2 + (((locals.var_vdsat_dn2 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn2)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn3 + (((locals.var_vdsat_dn3 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn3)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn4 + (((locals.var_vdsat_dn4 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn4)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn5 + (((locals.var_vdsat_dn5 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn5)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn6 + (((locals.var_vdsat_dn6 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn6)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn7 + (((locals.var_vdsat_dn7 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn7)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn8 + (((locals.var_vdsat_dn8 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn8)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn9 + (((locals.var_vdsat_dn9 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn9)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn10 + (((locals.var_vdsat_dn10 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn10)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn11 + (((locals.var_vdsat_dn11 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn11)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn13 + (((locals.var_vdsat_dn13 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn13)) / (2.0 * assign21320_e39503)))), (0.5 * (locals.var_vdsat_dn14 + (((locals.var_vdsat_dn14 * assign21320_e39495) + (assign21320_e39492 * locals.var_vdsat_dn14)) / (2.0 * assign21320_e39503)))),)
    } else {
        let assign21320_e39508: f64 = (locals.var_vdsat - 0.001);
        let assign21320_e39510: f64 = (-10000.0);
        let assign21320_e39512: f64 = (assign21320_e39510 * 1e-5);
        let (assign21320_e39523, assign21320_e39523_d_n0, assign21320_e39523_d_n2, assign21320_e39523_d_n3, assign21320_e39523_d_n4, assign21320_e39523_d_n5, assign21320_e39523_d_n6, assign21320_e39523_d_n7, assign21320_e39523_d_n8, assign21320_e39523_d_n9, assign21320_e39523_d_n10, assign21320_e39523_d_n11, assign21320_e39523_d_n13, assign21320_e39523_d_n14,) = {
            if (assign21320_e39508 < assign21320_e39512) {
                let assign21320_e39515: f64 = (-1e-5);
                let assign21320_e39517: f64 = (assign21320_e39515 * 1e-5);
                let assign21320_e39520: f64 = (locals.var_vdsat - 0.001);
                let assign21320_e39521: f64 = (assign21320_e39517 / assign21320_e39520);
                (assign21320_e39521, (-((assign21320_e39517 * locals.var_vdsat_dn0) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn2) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn3) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn4) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn5) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn6) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn7) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn8) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn9) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn10) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn11) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn13) / (assign21320_e39520 * assign21320_e39520))), (-((assign21320_e39517 * locals.var_vdsat_dn14) / (assign21320_e39520 * assign21320_e39520))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign21320_e39523, assign21320_e39523_d_n0, assign21320_e39523_d_n2, assign21320_e39523_d_n3, assign21320_e39523_d_n4, assign21320_e39523_d_n5, assign21320_e39523_d_n6, assign21320_e39523_d_n7, assign21320_e39523_d_n8, assign21320_e39523_d_n9, assign21320_e39523_d_n10, assign21320_e39523_d_n11, assign21320_e39523_d_n13, assign21320_e39523_d_n14,)
    }
};
        let assign21320_e39526: f64 = (assign21320_e39524 + 0.001);
        locals.var_vdsat = assign21320_e39526;
        locals.var_vdsat_dn0 = assign21320_e39524_d_n0;
        locals.var_vdsat_dn2 = assign21320_e39524_d_n2;
        locals.var_vdsat_dn3 = assign21320_e39524_d_n3;
        locals.var_vdsat_dn4 = assign21320_e39524_d_n4;
        locals.var_vdsat_dn5 = assign21320_e39524_d_n5;
        locals.var_vdsat_dn6 = assign21320_e39524_d_n6;
        locals.var_vdsat_dn7 = assign21320_e39524_d_n7;
        locals.var_vdsat_dn8 = assign21320_e39524_d_n8;
        locals.var_vdsat_dn9 = assign21320_e39524_d_n9;
        locals.var_vdsat_dn10 = assign21320_e39524_d_n10;
        locals.var_vdsat_dn11 = assign21320_e39524_d_n11;
        locals.var_vdsat_dn13 = assign21320_e39524_d_n13;
        locals.var_vdsat_dn14 = assign21320_e39524_d_n14;

        let assign21330_e39529: f64 = (locals.var_vds / locals.var_vdsat);
        let assign21330_e39531: f64 = (assign21330_e39529 + 1e-6);
        let assign21330_e39533: f64 = (assign21330_e39531).powf(locals.var_mexp_a);
        locals.var_t7 = assign21330_e39533;
        locals.var_t7_dn0 = if locals.var_mexp_a_dn0 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn0) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn0 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn0) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn2 = if locals.var_mexp_a_dn2 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn2) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn2 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn2) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn3 = if locals.var_mexp_a_dn3 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn3) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn3 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn3) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn4 = if locals.var_mexp_a_dn4 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn4) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn4 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn4) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn5 = if locals.var_mexp_a_dn5 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (((locals.var_vds_dn5 * locals.var_vdsat) - (locals.var_vds * locals.var_vdsat_dn5)) / (locals.var_vdsat * locals.var_vdsat)))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn5 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((((locals.var_vds_dn5 * locals.var_vdsat) - (locals.var_vds * locals.var_vdsat_dn5)) / (locals.var_vdsat * locals.var_vdsat)) / assign21330_e39531)))) };
        locals.var_t7_dn6 = if locals.var_mexp_a_dn6 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (((locals.var_vds_dn6 * locals.var_vdsat) - (locals.var_vds * locals.var_vdsat_dn6)) / (locals.var_vdsat * locals.var_vdsat)))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn6 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((((locals.var_vds_dn6 * locals.var_vdsat) - (locals.var_vds * locals.var_vdsat_dn6)) / (locals.var_vdsat * locals.var_vdsat)) / assign21330_e39531)))) };
        locals.var_t7_dn7 = if locals.var_mexp_a_dn7 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn7) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn7 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn7) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn8 = if locals.var_mexp_a_dn8 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn8) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn8 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn8) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn9 = if locals.var_mexp_a_dn9 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn9) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn9 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn9) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn10 = if locals.var_mexp_a_dn10 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn10) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn10 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn10) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn11 = if locals.var_mexp_a_dn11 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn11) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn11 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn11) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn13 = if locals.var_mexp_a_dn13 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn13) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn13 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn13) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };
        locals.var_t7_dn14 = if locals.var_mexp_a_dn14 == 0.0 && ((locals.var_mexp_a) as f64).is_finite() && ((locals.var_mexp_a) as f64).fract() == 0.0 { if locals.var_mexp_a == 0.0 { 0.0 } else { (locals.var_mexp_a * ((assign21330_e39531).powf(locals.var_mexp_a - 1.0) * (-((locals.var_vds * locals.var_vdsat_dn14) / (locals.var_vdsat * locals.var_vdsat))))) } } else { (assign21330_e39533 * ((locals.var_mexp_a_dn14 * (assign21330_e39531).ln()) + (locals.var_mexp_a * ((-((locals.var_vds * locals.var_vdsat_dn14) / (locals.var_vdsat * locals.var_vdsat))) / assign21330_e39531)))) };

        let assign21340_e39536: f64 = (1.0 + locals.var_t7);
        let assign21340_e39538: f64 = (assign21340_e39536).powf(locals.var_inv_mexp);
        locals.var_t8 = assign21340_e39538;
        locals.var_t8_dn0 = if locals.var_inv_mexp_dn0 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn0)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn0 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn0 / assign21340_e39536)))) };
        locals.var_t8_dn2 = if locals.var_inv_mexp_dn2 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn2)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn2 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn2 / assign21340_e39536)))) };
        locals.var_t8_dn3 = if locals.var_inv_mexp_dn3 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn3)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn3 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn3 / assign21340_e39536)))) };
        locals.var_t8_dn4 = if locals.var_inv_mexp_dn4 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn4)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn4 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn4 / assign21340_e39536)))) };
        locals.var_t8_dn5 = if locals.var_inv_mexp_dn5 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn5)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn5 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn5 / assign21340_e39536)))) };
        locals.var_t8_dn6 = if locals.var_inv_mexp_dn6 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn6)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn6 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn6 / assign21340_e39536)))) };
        locals.var_t8_dn7 = if locals.var_inv_mexp_dn7 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn7)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn7 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn7 / assign21340_e39536)))) };
        locals.var_t8_dn8 = if locals.var_inv_mexp_dn8 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn8)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn8 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn8 / assign21340_e39536)))) };
        locals.var_t8_dn9 = if locals.var_inv_mexp_dn9 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn9)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn9 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn9 / assign21340_e39536)))) };
        locals.var_t8_dn10 = if locals.var_inv_mexp_dn10 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn10)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn10 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn10 / assign21340_e39536)))) };
        locals.var_t8_dn11 = if locals.var_inv_mexp_dn11 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn11)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn11 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn11 / assign21340_e39536)))) };
        locals.var_t8_dn13 = if locals.var_inv_mexp_dn13 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn13)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn13 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn13 / assign21340_e39536)))) };
        locals.var_t8_dn14 = if locals.var_inv_mexp_dn14 == 0.0 && ((locals.var_inv_mexp) as f64).is_finite() && ((locals.var_inv_mexp) as f64).fract() == 0.0 { if locals.var_inv_mexp == 0.0 { 0.0 } else { (locals.var_inv_mexp * ((assign21340_e39536).powf(locals.var_inv_mexp - 1.0) * locals.var_t7_dn14)) } } else { (assign21340_e39538 * ((locals.var_inv_mexp_dn14 * (assign21340_e39536).ln()) + (locals.var_inv_mexp * (locals.var_t7_dn14 / assign21340_e39536)))) };

        let assign21350_e39541: f64 = (locals.var_vds / locals.var_t8);
        let assign21350_e39543: f64 = (assign21350_e39541).min(locals.var_vds);
        locals.var_vdseff_1 = assign21350_e39543;
        locals.var_vdseff_1_dn0 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn0) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn2 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn2) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn3 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn3) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn4 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn4) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn5 = if assign21350_e39541 <= locals.var_vds { (((locals.var_vds_dn5 * locals.var_t8) - (locals.var_vds * locals.var_t8_dn5)) / (locals.var_t8 * locals.var_t8)) } else { locals.var_vds_dn5 };
        locals.var_vdseff_1_dn6 = if assign21350_e39541 <= locals.var_vds { (((locals.var_vds_dn6 * locals.var_t8) - (locals.var_vds * locals.var_t8_dn6)) / (locals.var_t8 * locals.var_t8)) } else { locals.var_vds_dn6 };
        locals.var_vdseff_1_dn7 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn7) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn8 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn8) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn9 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn9) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn10 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn10) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn11 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn11) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn13 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn13) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };
        locals.var_vdseff_1_dn14 = if assign21350_e39541 <= locals.var_vds { (-((locals.var_vds * locals.var_t8_dn14) / (locals.var_t8 * locals.var_t8))) } else { 0.0 };

        let assign21360_e39546: f64 = (locals.var_vdseff_1 + locals.var_dvch_qm);
        locals.var_vch = assign21360_e39546;
        locals.var_vch_dn0 = (locals.var_vdseff_1_dn0 + locals.var_dvch_qm_dn0);
        locals.var_vch_dn2 = (locals.var_vdseff_1_dn2 + locals.var_dvch_qm_dn2);
        locals.var_vch_dn3 = (locals.var_vdseff_1_dn3 + locals.var_dvch_qm_dn3);
        locals.var_vch_dn4 = (locals.var_vdseff_1_dn4 + locals.var_dvch_qm_dn4);
        locals.var_vch_dn5 = (locals.var_vdseff_1_dn5 + locals.var_dvch_qm_dn5);
        locals.var_vch_dn6 = (locals.var_vdseff_1_dn6 + locals.var_dvch_qm_dn6);
        locals.var_vch_dn7 = (locals.var_vdseff_1_dn7 + locals.var_dvch_qm_dn7);
        locals.var_vch_dn8 = (locals.var_vdseff_1_dn8 + locals.var_dvch_qm_dn8);
        locals.var_vch_dn9 = (locals.var_vdseff_1_dn9 + locals.var_dvch_qm_dn9);
        locals.var_vch_dn10 = (locals.var_vdseff_1_dn10 + locals.var_dvch_qm_dn10);
        locals.var_vch_dn11 = (locals.var_vdseff_1_dn11 + locals.var_dvch_qm_dn11);
        locals.var_vch_dn13 = (locals.var_vdseff_1_dn13 + locals.var_dvch_qm_dn13);
        locals.var_vch_dn14 = (locals.var_vdseff_1_dn14 + locals.var_dvch_qm_dn14);

        let assign21370_e39548: f64 = (-locals.var_qdep);
        let assign21370_e39550: f64 = (assign21370_e39548).powf(0.666666667);
        locals.var_t2 = assign21370_e39550;
        locals.var_t2_dn0 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn0))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn0) / assign21370_e39548))) };
        locals.var_t2_dn2 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn2))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn2) / assign21370_e39548))) };
        locals.var_t2_dn3 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn3))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn3) / assign21370_e39548))) };
        locals.var_t2_dn4 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn4))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn4) / assign21370_e39548))) };
        locals.var_t2_dn5 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn5))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn5) / assign21370_e39548))) };
        locals.var_t2_dn6 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn6))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn6) / assign21370_e39548))) };
        locals.var_t2_dn7 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn7))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn7) / assign21370_e39548))) };
        locals.var_t2_dn8 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn8))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn8) / assign21370_e39548))) };
        locals.var_t2_dn9 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn9))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn9) / assign21370_e39548))) };
        locals.var_t2_dn10 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn10))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn10) / assign21370_e39548))) };
        locals.var_t2_dn11 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn11))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn11) / assign21370_e39548))) };
        locals.var_t2_dn13 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn13))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn13) / assign21370_e39548))) };
        locals.var_t2_dn14 = if 0.0 == 0.0 && ((0.666666667) as f64).is_finite() && ((0.666666667) as f64).fract() == 0.0 { if 0.666666667 == 0.0 { 0.0 } else { (0.666666667 * ((assign21370_e39548).powf(0.666666667 - 1.0) * (-locals.var_qdep_dn14))) } } else { (assign21370_e39550 * (0.666666667 * ((-locals.var_qdep_dn14) / assign21370_e39548))) };

        let assign21380_e39553: f64 = if p.p61 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard374 = assign21380_e39553;

        let (assign21390_e39628, assign21390_e39628_d_n0, assign21390_e39628_d_n2, assign21390_e39628_d_n3, assign21390_e39628_d_n4, assign21390_e39628_d_n5, assign21390_e39628_d_n6, assign21390_e39628_d_n7, assign21390_e39628_d_n8, assign21390_e39628_d_n9, assign21390_e39628_d_n10, assign21390_e39628_d_n11, assign21390_e39628_d_n13, assign21390_e39628_d_n14,) = {
    if (locals.var_guard374 != 0.0) {
        let assign21390_e39557: f64 = (2.0 * locals.var_phib);
        let assign21390_e39559: f64 = (assign21390_e39557 + locals.var_vch);
        let assign21390_e39561: f64 = (assign21390_e39559 - locals.var_ves);
        let assign21390_e39563: f64 = (-10000.0);
        let assign21390_e39565: f64 = (assign21390_e39563 * 0.1);
        let (assign21390_e39626, assign21390_e39626_d_n0, assign21390_e39626_d_n2, assign21390_e39626_d_n3, assign21390_e39626_d_n4, assign21390_e39626_d_n5, assign21390_e39626_d_n6, assign21390_e39626_d_n7, assign21390_e39626_d_n8, assign21390_e39626_d_n9, assign21390_e39626_d_n10, assign21390_e39626_d_n11, assign21390_e39626_d_n13, assign21390_e39626_d_n14,) = {
            if (!(assign21390_e39561 < assign21390_e39565)) {
                let assign21390_e39571: f64 = (2.0 * locals.var_phib);
                let assign21390_e39573: f64 = (assign21390_e39571 + locals.var_vch);
                let assign21390_e39575: f64 = (assign21390_e39573 - locals.var_ves);
                let assign21390_e39578: f64 = (2.0 * locals.var_phib);
                let assign21390_e39580: f64 = (assign21390_e39578 + locals.var_vch);
                let assign21390_e39582: f64 = (assign21390_e39580 - locals.var_ves);
                let assign21390_e39585: f64 = (2.0 * locals.var_phib);
                let assign21390_e39587: f64 = (assign21390_e39585 + locals.var_vch);
                let assign21390_e39589: f64 = (assign21390_e39587 - locals.var_ves);
                let assign21390_e39590: f64 = (assign21390_e39582 * assign21390_e39589);
                let assign21390_e39593: f64 = (4.0 * 0.1);
                let assign21390_e39595: f64 = (assign21390_e39593 * 0.1);
                let assign21390_e39596: f64 = (assign21390_e39590 + assign21390_e39595);
                let assign21390_e39597: f64 = (assign21390_e39596).sqrt();
                let assign21390_e39598: f64 = (assign21390_e39575 + assign21390_e39597);
                let assign21390_e39599: f64 = (0.5 * assign21390_e39598);
                (assign21390_e39599, (0.5 * (((2.0 * locals.var_phib_dn0) + locals.var_vch_dn0) + (((((2.0 * locals.var_phib_dn0) + locals.var_vch_dn0) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn0) + locals.var_vch_dn0))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn2) + locals.var_vch_dn2) + (((((2.0 * locals.var_phib_dn2) + locals.var_vch_dn2) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn2) + locals.var_vch_dn2))) / (2.0 * assign21390_e39597)))), (0.5 * ((((2.0 * locals.var_phib_dn3) + locals.var_vch_dn3) - locals.var_ves_dn3) + ((((((2.0 * locals.var_phib_dn3) + locals.var_vch_dn3) - locals.var_ves_dn3) * assign21390_e39589) + (assign21390_e39582 * (((2.0 * locals.var_phib_dn3) + locals.var_vch_dn3) - locals.var_ves_dn3))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn4) + locals.var_vch_dn4) + (((((2.0 * locals.var_phib_dn4) + locals.var_vch_dn4) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn4) + locals.var_vch_dn4))) / (2.0 * assign21390_e39597)))), (0.5 * ((((2.0 * locals.var_phib_dn5) + locals.var_vch_dn5) - locals.var_ves_dn5) + ((((((2.0 * locals.var_phib_dn5) + locals.var_vch_dn5) - locals.var_ves_dn5) * assign21390_e39589) + (assign21390_e39582 * (((2.0 * locals.var_phib_dn5) + locals.var_vch_dn5) - locals.var_ves_dn5))) / (2.0 * assign21390_e39597)))), (0.5 * ((((2.0 * locals.var_phib_dn6) + locals.var_vch_dn6) - locals.var_ves_dn6) + ((((((2.0 * locals.var_phib_dn6) + locals.var_vch_dn6) - locals.var_ves_dn6) * assign21390_e39589) + (assign21390_e39582 * (((2.0 * locals.var_phib_dn6) + locals.var_vch_dn6) - locals.var_ves_dn6))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn7) + locals.var_vch_dn7) + (((((2.0 * locals.var_phib_dn7) + locals.var_vch_dn7) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn7) + locals.var_vch_dn7))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn8) + locals.var_vch_dn8) + (((((2.0 * locals.var_phib_dn8) + locals.var_vch_dn8) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn8) + locals.var_vch_dn8))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn9) + locals.var_vch_dn9) + (((((2.0 * locals.var_phib_dn9) + locals.var_vch_dn9) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn9) + locals.var_vch_dn9))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn10) + locals.var_vch_dn10) + (((((2.0 * locals.var_phib_dn10) + locals.var_vch_dn10) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn10) + locals.var_vch_dn10))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn11) + locals.var_vch_dn11) + (((((2.0 * locals.var_phib_dn11) + locals.var_vch_dn11) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn11) + locals.var_vch_dn11))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn13) + locals.var_vch_dn13) + (((((2.0 * locals.var_phib_dn13) + locals.var_vch_dn13) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn13) + locals.var_vch_dn13))) / (2.0 * assign21390_e39597)))), (0.5 * (((2.0 * locals.var_phib_dn14) + locals.var_vch_dn14) + (((((2.0 * locals.var_phib_dn14) + locals.var_vch_dn14) * assign21390_e39589) + (assign21390_e39582 * ((2.0 * locals.var_phib_dn14) + locals.var_vch_dn14))) / (2.0 * assign21390_e39597)))),)
            } else {
                let assign21390_e39602: f64 = (2.0 * locals.var_phib);
                let assign21390_e39604: f64 = (assign21390_e39602 + locals.var_vch);
                let assign21390_e39606: f64 = (assign21390_e39604 - locals.var_ves);
                let assign21390_e39608: f64 = (-10000.0);
                let assign21390_e39610: f64 = (assign21390_e39608 * 0.1);
                let (assign21390_e39625, assign21390_e39625_d_n0, assign21390_e39625_d_n2, assign21390_e39625_d_n3, assign21390_e39625_d_n4, assign21390_e39625_d_n5, assign21390_e39625_d_n6, assign21390_e39625_d_n7, assign21390_e39625_d_n8, assign21390_e39625_d_n9, assign21390_e39625_d_n10, assign21390_e39625_d_n11, assign21390_e39625_d_n13, assign21390_e39625_d_n14,) = {
                    if (assign21390_e39606 < assign21390_e39610) {
                        let assign21390_e39613: f64 = (-0.1);
                        let assign21390_e39615: f64 = (assign21390_e39613 * 0.1);
                        let assign21390_e39618: f64 = (2.0 * locals.var_phib);
                        let assign21390_e39620: f64 = (assign21390_e39618 + locals.var_vch);
                        let assign21390_e39622: f64 = (assign21390_e39620 - locals.var_ves);
                        let assign21390_e39623: f64 = (assign21390_e39615 / assign21390_e39622);
                        (assign21390_e39623, (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn0) + locals.var_vch_dn0)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn2) + locals.var_vch_dn2)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * (((2.0 * locals.var_phib_dn3) + locals.var_vch_dn3) - locals.var_ves_dn3)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn4) + locals.var_vch_dn4)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * (((2.0 * locals.var_phib_dn5) + locals.var_vch_dn5) - locals.var_ves_dn5)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * (((2.0 * locals.var_phib_dn6) + locals.var_vch_dn6) - locals.var_ves_dn6)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn7) + locals.var_vch_dn7)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn8) + locals.var_vch_dn8)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn9) + locals.var_vch_dn9)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn10) + locals.var_vch_dn10)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn11) + locals.var_vch_dn11)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn13) + locals.var_vch_dn13)) / (assign21390_e39622 * assign21390_e39622))), (-((assign21390_e39615 * ((2.0 * locals.var_phib_dn14) + locals.var_vch_dn14)) / (assign21390_e39622 * assign21390_e39622))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign21390_e39625, assign21390_e39625_d_n0, assign21390_e39625_d_n2, assign21390_e39625_d_n3, assign21390_e39625_d_n4, assign21390_e39625_d_n5, assign21390_e39625_d_n6, assign21390_e39625_d_n7, assign21390_e39625_d_n8, assign21390_e39625_d_n9, assign21390_e39625_d_n10, assign21390_e39625_d_n11, assign21390_e39625_d_n13, assign21390_e39625_d_n14,)
            }
        };
        (assign21390_e39626, assign21390_e39626_d_n0, assign21390_e39626_d_n2, assign21390_e39626_d_n3, assign21390_e39626_d_n4, assign21390_e39626_d_n5, assign21390_e39626_d_n6, assign21390_e39626_d_n7, assign21390_e39626_d_n8, assign21390_e39626_d_n9, assign21390_e39626_d_n10, assign21390_e39626_d_n11, assign21390_e39626_d_n13, assign21390_e39626_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21390_e39628;
        locals.var_t1_dn0 = assign21390_e39628_d_n0;
        locals.var_t1_dn2 = assign21390_e39628_d_n2;
        locals.var_t1_dn3 = assign21390_e39628_d_n3;
        locals.var_t1_dn4 = assign21390_e39628_d_n4;
        locals.var_t1_dn5 = assign21390_e39628_d_n5;
        locals.var_t1_dn6 = assign21390_e39628_d_n6;
        locals.var_t1_dn7 = assign21390_e39628_d_n7;
        locals.var_t1_dn8 = assign21390_e39628_d_n8;
        locals.var_t1_dn9 = assign21390_e39628_d_n9;
        locals.var_t1_dn10 = assign21390_e39628_d_n10;
        locals.var_t1_dn11 = assign21390_e39628_d_n11;
        locals.var_t1_dn13 = assign21390_e39628_d_n13;
        locals.var_t1_dn14 = assign21390_e39628_d_n14;

    }

    pub(super) fn stamp_transient_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21400_e39645, assign21400_e39645_d_n0, assign21400_e39645_d_n2, assign21400_e39645_d_n3, assign21400_e39645_d_n4, assign21400_e39645_d_n5, assign21400_e39645_d_n6, assign21400_e39645_d_n7, assign21400_e39645_d_n8, assign21400_e39645_d_n9, assign21400_e39645_d_n10, assign21400_e39645_d_n11, assign21400_e39645_d_n13, assign21400_e39645_d_n14,) = {
    if (locals.var_guard374 != 0.0) {
        let assign21400_e39631: f64 = (-locals.var_k1_t);
        let assign21400_e39634: f64 = (2.0 * locals.var_nvtm);
        let assign21400_e39635: f64 = (assign21400_e39631 / assign21400_e39634);
        let assign21400_e39637: f64 = (locals.var_t1).sqrt();
        let assign21400_e39640: f64 = (2.0 * locals.var_phib);
        let assign21400_e39641: f64 = (assign21400_e39640).sqrt();
        let assign21400_e39642: f64 = (assign21400_e39637 - assign21400_e39641);
        let assign21400_e39643: f64 = (assign21400_e39635 * assign21400_e39642);
        (assign21400_e39643, (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn0)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn0 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn0) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn2)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn2 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn2) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn3)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn3 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn3) / (2.0 * assign21400_e39641))))), ((((((-locals.var_k1_t_dn4) * assign21400_e39634) - (assign21400_e39631 * (2.0 * locals.var_nvtm_dn4))) / (assign21400_e39634 * assign21400_e39634)) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn4 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn4) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn5)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn5 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn5) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn6)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn6 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn6) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn7)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn7 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn7) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn8)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn8 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn8) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn9)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn9 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn9) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn10)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn10 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn10) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn11)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn11 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn11) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn13)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn13 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn13) / (2.0 * assign21400_e39641))))), (((-((assign21400_e39631 * (2.0 * locals.var_nvtm_dn14)) / (assign21400_e39634 * assign21400_e39634))) * assign21400_e39642) + (assign21400_e39635 * ((locals.var_t1_dn14 / (2.0 * assign21400_e39637)) - ((2.0 * locals.var_phib_dn14) / (2.0 * assign21400_e39641))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21400_e39645;
        locals.var_t3_dn0 = assign21400_e39645_d_n0;
        locals.var_t3_dn2 = assign21400_e39645_d_n2;
        locals.var_t3_dn3 = assign21400_e39645_d_n3;
        locals.var_t3_dn4 = assign21400_e39645_d_n4;
        locals.var_t3_dn5 = assign21400_e39645_d_n5;
        locals.var_t3_dn6 = assign21400_e39645_d_n6;
        locals.var_t3_dn7 = assign21400_e39645_d_n7;
        locals.var_t3_dn8 = assign21400_e39645_d_n8;
        locals.var_t3_dn9 = assign21400_e39645_d_n9;
        locals.var_t3_dn10 = assign21400_e39645_d_n10;
        locals.var_t3_dn11 = assign21400_e39645_d_n11;
        locals.var_t3_dn13 = assign21400_e39645_d_n13;
        locals.var_t3_dn14 = assign21400_e39645_d_n14;

        let (assign21410_e39658, assign21410_e39658_d_n0, assign21410_e39658_d_n2, assign21410_e39658_d_n3, assign21410_e39658_d_n4, assign21410_e39658_d_n5, assign21410_e39658_d_n6, assign21410_e39658_d_n7, assign21410_e39658_d_n8, assign21410_e39658_d_n9, assign21410_e39658_d_n10, assign21410_e39658_d_n11, assign21410_e39658_d_n13, assign21410_e39658_d_n14,) = {
    if (locals.var_guard374 != 0.0) {
        let assign21410_e39648: f64 = (-locals.var_qdep);
        let assign21410_e39650: f64 = (assign21410_e39648 - locals.var_t3);
        let assign21410_e39652: f64 = (assign21410_e39650 + locals.var_vth_fixed_factor_sub);
        let assign21410_e39655: f64 = (locals.var_qmfactorcvfinal * locals.var_t2);
        let assign21410_e39656: f64 = (assign21410_e39652 + assign21410_e39655);
        (assign21410_e39656, ((((-locals.var_qdep_dn0) - locals.var_t3_dn0) + locals.var_vth_fixed_factor_sub_dn0) + (locals.var_qmfactorcvfinal * locals.var_t2_dn0)), ((((-locals.var_qdep_dn2) - locals.var_t3_dn2) + locals.var_vth_fixed_factor_sub_dn2) + (locals.var_qmfactorcvfinal * locals.var_t2_dn2)), ((((-locals.var_qdep_dn3) - locals.var_t3_dn3) + locals.var_vth_fixed_factor_sub_dn3) + (locals.var_qmfactorcvfinal * locals.var_t2_dn3)), ((((-locals.var_qdep_dn4) - locals.var_t3_dn4) + locals.var_vth_fixed_factor_sub_dn4) + ((locals.var_qmfactorcvfinal_dn4 * locals.var_t2) + (locals.var_qmfactorcvfinal * locals.var_t2_dn4))), ((((-locals.var_qdep_dn5) - locals.var_t3_dn5) + locals.var_vth_fixed_factor_sub_dn5) + (locals.var_qmfactorcvfinal * locals.var_t2_dn5)), ((((-locals.var_qdep_dn6) - locals.var_t3_dn6) + locals.var_vth_fixed_factor_sub_dn6) + (locals.var_qmfactorcvfinal * locals.var_t2_dn6)), ((((-locals.var_qdep_dn7) - locals.var_t3_dn7) + locals.var_vth_fixed_factor_sub_dn7) + (locals.var_qmfactorcvfinal * locals.var_t2_dn7)), ((((-locals.var_qdep_dn8) - locals.var_t3_dn8) + locals.var_vth_fixed_factor_sub_dn8) + (locals.var_qmfactorcvfinal * locals.var_t2_dn8)), ((((-locals.var_qdep_dn9) - locals.var_t3_dn9) + locals.var_vth_fixed_factor_sub_dn9) + (locals.var_qmfactorcvfinal * locals.var_t2_dn9)), ((((-locals.var_qdep_dn10) - locals.var_t3_dn10) + locals.var_vth_fixed_factor_sub_dn10) + (locals.var_qmfactorcvfinal * locals.var_t2_dn10)), ((((-locals.var_qdep_dn11) - locals.var_t3_dn11) + locals.var_vth_fixed_factor_sub_dn11) + (locals.var_qmfactorcvfinal * locals.var_t2_dn11)), ((((-locals.var_qdep_dn13) - locals.var_t3_dn13) + locals.var_vth_fixed_factor_sub_dn13) + (locals.var_qmfactorcvfinal * locals.var_t2_dn13)), ((((-locals.var_qdep_dn14) - locals.var_t3_dn14) + locals.var_vth_fixed_factor_sub_dn14) + (locals.var_qmfactorcvfinal * locals.var_t2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21410_e39658;
        locals.var_t0_dn0 = assign21410_e39658_d_n0;
        locals.var_t0_dn2 = assign21410_e39658_d_n2;
        locals.var_t0_dn3 = assign21410_e39658_d_n3;
        locals.var_t0_dn4 = assign21410_e39658_d_n4;
        locals.var_t0_dn5 = assign21410_e39658_d_n5;
        locals.var_t0_dn6 = assign21410_e39658_d_n6;
        locals.var_t0_dn7 = assign21410_e39658_d_n7;
        locals.var_t0_dn8 = assign21410_e39658_d_n8;
        locals.var_t0_dn9 = assign21410_e39658_d_n9;
        locals.var_t0_dn10 = assign21410_e39658_d_n10;
        locals.var_t0_dn11 = assign21410_e39658_d_n11;
        locals.var_t0_dn13 = assign21410_e39658_d_n13;
        locals.var_t0_dn14 = assign21410_e39658_d_n14;

        let (assign21420_e39667, assign21420_e39667_d_n0, assign21420_e39667_d_n2, assign21420_e39667_d_n3, assign21420_e39667_d_n4, assign21420_e39667_d_n5, assign21420_e39667_d_n6, assign21420_e39667_d_n7, assign21420_e39667_d_n8, assign21420_e39667_d_n9, assign21420_e39667_d_n10, assign21420_e39667_d_n11, assign21420_e39667_d_n13, assign21420_e39667_d_n14,) = {
    if (locals.var_guard374 != 0.0) {
        let assign21420_e39661: f64 = (-locals.var_qdep);
        let assign21420_e39663: f64 = (assign21420_e39661 - locals.var_t3);
        let assign21420_e39665: f64 = (assign21420_e39663 + locals.var_vth_fixed_factor_si);
        (assign21420_e39665, (((-locals.var_qdep_dn0) - locals.var_t3_dn0) + locals.var_vth_fixed_factor_si_dn0), (((-locals.var_qdep_dn2) - locals.var_t3_dn2) + locals.var_vth_fixed_factor_si_dn2), (((-locals.var_qdep_dn3) - locals.var_t3_dn3) + locals.var_vth_fixed_factor_si_dn3), (((-locals.var_qdep_dn4) - locals.var_t3_dn4) + locals.var_vth_fixed_factor_si_dn4), (((-locals.var_qdep_dn5) - locals.var_t3_dn5) + locals.var_vth_fixed_factor_si_dn5), (((-locals.var_qdep_dn6) - locals.var_t3_dn6) + locals.var_vth_fixed_factor_si_dn6), (((-locals.var_qdep_dn7) - locals.var_t3_dn7) + locals.var_vth_fixed_factor_si_dn7), (((-locals.var_qdep_dn8) - locals.var_t3_dn8) + locals.var_vth_fixed_factor_si_dn8), (((-locals.var_qdep_dn9) - locals.var_t3_dn9) + locals.var_vth_fixed_factor_si_dn9), (((-locals.var_qdep_dn10) - locals.var_t3_dn10) + locals.var_vth_fixed_factor_si_dn10), (((-locals.var_qdep_dn11) - locals.var_t3_dn11) + locals.var_vth_fixed_factor_si_dn11), (((-locals.var_qdep_dn13) - locals.var_t3_dn13) + locals.var_vth_fixed_factor_si_dn13), (((-locals.var_qdep_dn14) - locals.var_t3_dn14) + locals.var_vth_fixed_factor_si_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21420_e39667;
        locals.var_t1_dn0 = assign21420_e39667_d_n0;
        locals.var_t1_dn2 = assign21420_e39667_d_n2;
        locals.var_t1_dn3 = assign21420_e39667_d_n3;
        locals.var_t1_dn4 = assign21420_e39667_d_n4;
        locals.var_t1_dn5 = assign21420_e39667_d_n5;
        locals.var_t1_dn6 = assign21420_e39667_d_n6;
        locals.var_t1_dn7 = assign21420_e39667_d_n7;
        locals.var_t1_dn8 = assign21420_e39667_d_n8;
        locals.var_t1_dn9 = assign21420_e39667_d_n9;
        locals.var_t1_dn10 = assign21420_e39667_d_n10;
        locals.var_t1_dn11 = assign21420_e39667_d_n11;
        locals.var_t1_dn13 = assign21420_e39667_d_n13;
        locals.var_t1_dn14 = assign21420_e39667_d_n14;

        let (assign21430_e39679, assign21430_e39679_d_n0, assign21430_e39679_d_n2, assign21430_e39679_d_n3, assign21430_e39679_d_n4, assign21430_e39679_d_n5, assign21430_e39679_d_n6, assign21430_e39679_d_n7, assign21430_e39679_d_n8, assign21430_e39679_d_n9, assign21430_e39679_d_n10, assign21430_e39679_d_n11, assign21430_e39679_d_n13, assign21430_e39679_d_n14,) = {
    if (locals.var_guard374 == 0.0) {
        let assign21430_e39671: f64 = (-locals.var_qdep);
        let assign21430_e39673: f64 = (assign21430_e39671 + locals.var_vth_fixed_factor_sub);
        let assign21430_e39676: f64 = (locals.var_qmfactorcvfinal * locals.var_t2);
        let assign21430_e39677: f64 = (assign21430_e39673 + assign21430_e39676);
        (assign21430_e39677, (((-locals.var_qdep_dn0) + locals.var_vth_fixed_factor_sub_dn0) + (locals.var_qmfactorcvfinal * locals.var_t2_dn0)), (((-locals.var_qdep_dn2) + locals.var_vth_fixed_factor_sub_dn2) + (locals.var_qmfactorcvfinal * locals.var_t2_dn2)), (((-locals.var_qdep_dn3) + locals.var_vth_fixed_factor_sub_dn3) + (locals.var_qmfactorcvfinal * locals.var_t2_dn3)), (((-locals.var_qdep_dn4) + locals.var_vth_fixed_factor_sub_dn4) + ((locals.var_qmfactorcvfinal_dn4 * locals.var_t2) + (locals.var_qmfactorcvfinal * locals.var_t2_dn4))), (((-locals.var_qdep_dn5) + locals.var_vth_fixed_factor_sub_dn5) + (locals.var_qmfactorcvfinal * locals.var_t2_dn5)), (((-locals.var_qdep_dn6) + locals.var_vth_fixed_factor_sub_dn6) + (locals.var_qmfactorcvfinal * locals.var_t2_dn6)), (((-locals.var_qdep_dn7) + locals.var_vth_fixed_factor_sub_dn7) + (locals.var_qmfactorcvfinal * locals.var_t2_dn7)), (((-locals.var_qdep_dn8) + locals.var_vth_fixed_factor_sub_dn8) + (locals.var_qmfactorcvfinal * locals.var_t2_dn8)), (((-locals.var_qdep_dn9) + locals.var_vth_fixed_factor_sub_dn9) + (locals.var_qmfactorcvfinal * locals.var_t2_dn9)), (((-locals.var_qdep_dn10) + locals.var_vth_fixed_factor_sub_dn10) + (locals.var_qmfactorcvfinal * locals.var_t2_dn10)), (((-locals.var_qdep_dn11) + locals.var_vth_fixed_factor_sub_dn11) + (locals.var_qmfactorcvfinal * locals.var_t2_dn11)), (((-locals.var_qdep_dn13) + locals.var_vth_fixed_factor_sub_dn13) + (locals.var_qmfactorcvfinal * locals.var_t2_dn13)), (((-locals.var_qdep_dn14) + locals.var_vth_fixed_factor_sub_dn14) + (locals.var_qmfactorcvfinal * locals.var_t2_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21430_e39679;
        locals.var_t0_dn0 = assign21430_e39679_d_n0;
        locals.var_t0_dn2 = assign21430_e39679_d_n2;
        locals.var_t0_dn3 = assign21430_e39679_d_n3;
        locals.var_t0_dn4 = assign21430_e39679_d_n4;
        locals.var_t0_dn5 = assign21430_e39679_d_n5;
        locals.var_t0_dn6 = assign21430_e39679_d_n6;
        locals.var_t0_dn7 = assign21430_e39679_d_n7;
        locals.var_t0_dn8 = assign21430_e39679_d_n8;
        locals.var_t0_dn9 = assign21430_e39679_d_n9;
        locals.var_t0_dn10 = assign21430_e39679_d_n10;
        locals.var_t0_dn11 = assign21430_e39679_d_n11;
        locals.var_t0_dn13 = assign21430_e39679_d_n13;
        locals.var_t0_dn14 = assign21430_e39679_d_n14;

        let (assign21440_e39687, assign21440_e39687_d_n0, assign21440_e39687_d_n2, assign21440_e39687_d_n3, assign21440_e39687_d_n4, assign21440_e39687_d_n5, assign21440_e39687_d_n6, assign21440_e39687_d_n7, assign21440_e39687_d_n8, assign21440_e39687_d_n9, assign21440_e39687_d_n10, assign21440_e39687_d_n11, assign21440_e39687_d_n13, assign21440_e39687_d_n14,) = {
    if (locals.var_guard374 == 0.0) {
        let assign21440_e39683: f64 = (-locals.var_qdep);
        let assign21440_e39685: f64 = (assign21440_e39683 + locals.var_vth_fixed_factor_si);
        (assign21440_e39685, ((-locals.var_qdep_dn0) + locals.var_vth_fixed_factor_si_dn0), ((-locals.var_qdep_dn2) + locals.var_vth_fixed_factor_si_dn2), ((-locals.var_qdep_dn3) + locals.var_vth_fixed_factor_si_dn3), ((-locals.var_qdep_dn4) + locals.var_vth_fixed_factor_si_dn4), ((-locals.var_qdep_dn5) + locals.var_vth_fixed_factor_si_dn5), ((-locals.var_qdep_dn6) + locals.var_vth_fixed_factor_si_dn6), ((-locals.var_qdep_dn7) + locals.var_vth_fixed_factor_si_dn7), ((-locals.var_qdep_dn8) + locals.var_vth_fixed_factor_si_dn8), ((-locals.var_qdep_dn9) + locals.var_vth_fixed_factor_si_dn9), ((-locals.var_qdep_dn10) + locals.var_vth_fixed_factor_si_dn10), ((-locals.var_qdep_dn11) + locals.var_vth_fixed_factor_si_dn11), ((-locals.var_qdep_dn13) + locals.var_vth_fixed_factor_si_dn13), ((-locals.var_qdep_dn14) + locals.var_vth_fixed_factor_si_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21440_e39687;
        locals.var_t1_dn0 = assign21440_e39687_d_n0;
        locals.var_t1_dn2 = assign21440_e39687_d_n2;
        locals.var_t1_dn3 = assign21440_e39687_d_n3;
        locals.var_t1_dn4 = assign21440_e39687_d_n4;
        locals.var_t1_dn5 = assign21440_e39687_d_n5;
        locals.var_t1_dn6 = assign21440_e39687_d_n6;
        locals.var_t1_dn7 = assign21440_e39687_d_n7;
        locals.var_t1_dn8 = assign21440_e39687_d_n8;
        locals.var_t1_dn9 = assign21440_e39687_d_n9;
        locals.var_t1_dn10 = assign21440_e39687_d_n10;
        locals.var_t1_dn11 = assign21440_e39687_d_n11;
        locals.var_t1_dn13 = assign21440_e39687_d_n13;
        locals.var_t1_dn14 = assign21440_e39687_d_n14;

        let assign21450_e39690: f64 = (locals.var_vgsfbeff - locals.var_vch);
        let assign21450_e39692: f64 = (assign21450_e39690 / locals.var_nvtm);
        locals.var_t2 = assign21450_e39692;
        locals.var_t2_dn0 = ((((locals.var_vgsfbeff_dn0 - locals.var_vch_dn0) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn0)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn2 = ((((locals.var_vgsfbeff_dn2 - locals.var_vch_dn2) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn2)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn3 = ((((locals.var_vgsfbeff_dn3 - locals.var_vch_dn3) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn3)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn4 = ((((locals.var_vgsfbeff_dn4 - locals.var_vch_dn4) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn4)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn5 = ((((locals.var_vgsfbeff_dn5 - locals.var_vch_dn5) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn5)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn6 = ((((locals.var_vgsfbeff_dn6 - locals.var_vch_dn6) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn6)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn7 = ((((locals.var_vgsfbeff_dn7 - locals.var_vch_dn7) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn7)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn8 = ((((locals.var_vgsfbeff_dn8 - locals.var_vch_dn8) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn8)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn9 = ((((locals.var_vgsfbeff_dn9 - locals.var_vch_dn9) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn9)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn10 = ((((locals.var_vgsfbeff_dn10 - locals.var_vch_dn10) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn10)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn11 = ((((locals.var_vgsfbeff_dn11 - locals.var_vch_dn11) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn11)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn13 = ((((locals.var_vgsfbeff_dn13 - locals.var_vch_dn13) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn13)) / (locals.var_nvtm * locals.var_nvtm));
        locals.var_t2_dn14 = ((((locals.var_vgsfbeff_dn14 - locals.var_vch_dn14) * locals.var_nvtm) - (assign21450_e39690 * locals.var_nvtm_dn14)) / (locals.var_nvtm * locals.var_nvtm));

        let assign21460_e39694: f64 = (-locals.var_t2);
        let assign21460_e39696: f64 = (assign21460_e39694 + locals.var_t1);
        locals.var_f0 = assign21460_e39696;
        locals.var_f0_dn0 = ((-locals.var_t2_dn0) + locals.var_t1_dn0);
        locals.var_f0_dn2 = ((-locals.var_t2_dn2) + locals.var_t1_dn2);
        locals.var_f0_dn3 = ((-locals.var_t2_dn3) + locals.var_t1_dn3);
        locals.var_f0_dn4 = ((-locals.var_t2_dn4) + locals.var_t1_dn4);
        locals.var_f0_dn5 = ((-locals.var_t2_dn5) + locals.var_t1_dn5);
        locals.var_f0_dn6 = ((-locals.var_t2_dn6) + locals.var_t1_dn6);
        locals.var_f0_dn7 = ((-locals.var_t2_dn7) + locals.var_t1_dn7);
        locals.var_f0_dn8 = ((-locals.var_t2_dn8) + locals.var_t1_dn8);
        locals.var_f0_dn9 = ((-locals.var_t2_dn9) + locals.var_t1_dn9);
        locals.var_f0_dn10 = ((-locals.var_t2_dn10) + locals.var_t1_dn10);
        locals.var_f0_dn11 = ((-locals.var_t2_dn11) + locals.var_t1_dn11);
        locals.var_f0_dn13 = ((-locals.var_t2_dn13) + locals.var_t1_dn13);
        locals.var_f0_dn14 = ((-locals.var_t2_dn14) + locals.var_t1_dn14);

        let assign21470_e39699: f64 = (locals.var_t2 - locals.var_t0);
        let assign21470_e39701: f64 = (assign21470_e39699 * 0.5);
        locals.var_t3 = assign21470_e39701;
        locals.var_t3_dn0 = ((locals.var_t2_dn0 - locals.var_t0_dn0) * 0.5);
        locals.var_t3_dn2 = ((locals.var_t2_dn2 - locals.var_t0_dn2) * 0.5);
        locals.var_t3_dn3 = ((locals.var_t2_dn3 - locals.var_t0_dn3) * 0.5);
        locals.var_t3_dn4 = ((locals.var_t2_dn4 - locals.var_t0_dn4) * 0.5);
        locals.var_t3_dn5 = ((locals.var_t2_dn5 - locals.var_t0_dn5) * 0.5);
        locals.var_t3_dn6 = ((locals.var_t2_dn6 - locals.var_t0_dn6) * 0.5);
        locals.var_t3_dn7 = ((locals.var_t2_dn7 - locals.var_t0_dn7) * 0.5);
        locals.var_t3_dn8 = ((locals.var_t2_dn8 - locals.var_t0_dn8) * 0.5);
        locals.var_t3_dn9 = ((locals.var_t2_dn9 - locals.var_t0_dn9) * 0.5);
        locals.var_t3_dn10 = ((locals.var_t2_dn10 - locals.var_t0_dn10) * 0.5);
        locals.var_t3_dn11 = ((locals.var_t2_dn11 - locals.var_t0_dn11) * 0.5);
        locals.var_t3_dn13 = ((locals.var_t2_dn13 - locals.var_t0_dn13) * 0.5);
        locals.var_t3_dn14 = ((locals.var_t2_dn14 - locals.var_t0_dn14) * 0.5);

        let assign21480_e39703: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_qm = assign21480_e39703;
        locals.var_qm_dn0 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn0);
        locals.var_qm_dn2 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn2);
        locals.var_qm_dn3 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3);
        locals.var_qm_dn4 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4);
        locals.var_qm_dn5 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5);
        locals.var_qm_dn6 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6);
        locals.var_qm_dn7 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7);
        locals.var_qm_dn8 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8);
        locals.var_qm_dn9 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9);
        locals.var_qm_dn10 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10);
        locals.var_qm_dn11 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11);
        locals.var_qm_dn13 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn13);
        locals.var_qm_dn14 = ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn14);

        let assign21490_e39706: f64 = if locals.var_qm > 1e-7 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign21490_e39706;

        let (assign21500_e39713, assign21500_e39713_d_n0, assign21500_e39713_d_n2, assign21500_e39713_d_n3, assign21500_e39713_d_n4, assign21500_e39713_d_n5, assign21500_e39713_d_n6, assign21500_e39713_d_n7, assign21500_e39713_d_n8, assign21500_e39713_d_n9, assign21500_e39713_d_n10, assign21500_e39713_d_n11, assign21500_e39713_d_n13, assign21500_e39713_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21500_e39710: f64 = (1.0 + locals.var_qm);
        let assign21500_e39711: f64 = (assign21500_e39710).ln();
        (assign21500_e39711, (locals.var_qm_dn0 / assign21500_e39710), (locals.var_qm_dn2 / assign21500_e39710), (locals.var_qm_dn3 / assign21500_e39710), (locals.var_qm_dn4 / assign21500_e39710), (locals.var_qm_dn5 / assign21500_e39710), (locals.var_qm_dn6 / assign21500_e39710), (locals.var_qm_dn7 / assign21500_e39710), (locals.var_qm_dn8 / assign21500_e39710), (locals.var_qm_dn9 / assign21500_e39710), (locals.var_qm_dn10 / assign21500_e39710), (locals.var_qm_dn11 / assign21500_e39710), (locals.var_qm_dn13 / assign21500_e39710), (locals.var_qm_dn14 / assign21500_e39710),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign21500_e39713;
        locals.var_t7_dn0 = assign21500_e39713_d_n0;
        locals.var_t7_dn2 = assign21500_e39713_d_n2;
        locals.var_t7_dn3 = assign21500_e39713_d_n3;
        locals.var_t7_dn4 = assign21500_e39713_d_n4;
        locals.var_t7_dn5 = assign21500_e39713_d_n5;
        locals.var_t7_dn6 = assign21500_e39713_d_n6;
        locals.var_t7_dn7 = assign21500_e39713_d_n7;
        locals.var_t7_dn8 = assign21500_e39713_d_n8;
        locals.var_t7_dn9 = assign21500_e39713_d_n9;
        locals.var_t7_dn10 = assign21500_e39713_d_n10;
        locals.var_t7_dn11 = assign21500_e39713_d_n11;
        locals.var_t7_dn13 = assign21500_e39713_d_n13;
        locals.var_t7_dn14 = assign21500_e39713_d_n14;

        let (assign21510_e39726, assign21510_e39726_d_n0, assign21510_e39726_d_n2, assign21510_e39726_d_n3, assign21510_e39726_d_n4, assign21510_e39726_d_n5, assign21510_e39726_d_n6, assign21510_e39726_d_n7, assign21510_e39726_d_n8, assign21510_e39726_d_n9, assign21510_e39726_d_n10, assign21510_e39726_d_n11, assign21510_e39726_d_n13, assign21510_e39726_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21510_e39720: f64 = (locals.var_t7 * locals.var_t7);
        let assign21510_e39721: f64 = (1.0 + assign21510_e39720);
        let assign21510_e39722: f64 = (assign21510_e39721).sqrt();
        let assign21510_e39723: f64 = (1.0 - assign21510_e39722);
        let assign21510_e39724: f64 = (2.0 * assign21510_e39723);
        (assign21510_e39724, (2.0 * (-(((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn13 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn13)) / (2.0 * assign21510_e39722)))), (2.0 * (-(((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)) / (2.0 * assign21510_e39722)))),)
    } else {
        (locals.var_qm, locals.var_qm_dn0, locals.var_qm_dn2, locals.var_qm_dn3, locals.var_qm_dn4, locals.var_qm_dn5, locals.var_qm_dn6, locals.var_qm_dn7, locals.var_qm_dn8, locals.var_qm_dn9, locals.var_qm_dn10, locals.var_qm_dn11, locals.var_qm_dn13, locals.var_qm_dn14,)
    }
};
        locals.var_qm = assign21510_e39726;
        locals.var_qm_dn0 = assign21510_e39726_d_n0;
        locals.var_qm_dn2 = assign21510_e39726_d_n2;
        locals.var_qm_dn3 = assign21510_e39726_d_n3;
        locals.var_qm_dn4 = assign21510_e39726_d_n4;
        locals.var_qm_dn5 = assign21510_e39726_d_n5;
        locals.var_qm_dn6 = assign21510_e39726_d_n6;
        locals.var_qm_dn7 = assign21510_e39726_d_n7;
        locals.var_qm_dn8 = assign21510_e39726_d_n8;
        locals.var_qm_dn9 = assign21510_e39726_d_n9;
        locals.var_qm_dn10 = assign21510_e39726_d_n10;
        locals.var_qm_dn11 = assign21510_e39726_d_n11;
        locals.var_qm_dn13 = assign21510_e39726_d_n13;
        locals.var_qm_dn14 = assign21510_e39726_d_n14;

        let (assign21520_e39736, assign21520_e39736_d_n0, assign21520_e39736_d_n2, assign21520_e39736_d_n3, assign21520_e39736_d_n4, assign21520_e39736_d_n5, assign21520_e39736_d_n6, assign21520_e39736_d_n7, assign21520_e39736_d_n8, assign21520_e39736_d_n9, assign21520_e39736_d_n10, assign21520_e39736_d_n11, assign21520_e39736_d_n13, assign21520_e39736_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21520_e39730: f64 = (locals.var_qm * p.p1805);
        let assign21520_e39732: f64 = (assign21520_e39730 + locals.var_qdep);
        let assign21520_e39734: f64 = (assign21520_e39732 * locals.var_rc);
        (assign21520_e39734, (((locals.var_qm_dn0 * p.p1805) + locals.var_qdep_dn0) * locals.var_rc), (((locals.var_qm_dn2 * p.p1805) + locals.var_qdep_dn2) * locals.var_rc), (((locals.var_qm_dn3 * p.p1805) + locals.var_qdep_dn3) * locals.var_rc), (((locals.var_qm_dn4 * p.p1805) + locals.var_qdep_dn4) * locals.var_rc), (((locals.var_qm_dn5 * p.p1805) + locals.var_qdep_dn5) * locals.var_rc), (((locals.var_qm_dn6 * p.p1805) + locals.var_qdep_dn6) * locals.var_rc), (((locals.var_qm_dn7 * p.p1805) + locals.var_qdep_dn7) * locals.var_rc), (((locals.var_qm_dn8 * p.p1805) + locals.var_qdep_dn8) * locals.var_rc), (((locals.var_qm_dn9 * p.p1805) + locals.var_qdep_dn9) * locals.var_rc), (((locals.var_qm_dn10 * p.p1805) + locals.var_qdep_dn10) * locals.var_rc), (((locals.var_qm_dn11 * p.p1805) + locals.var_qdep_dn11) * locals.var_rc), (((locals.var_qm_dn13 * p.p1805) + locals.var_qdep_dn13) * locals.var_rc), (((locals.var_qm_dn14 * p.p1805) + locals.var_qdep_dn14) * locals.var_rc),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign21520_e39736;
        locals.var_t8_dn0 = assign21520_e39736_d_n0;
        locals.var_t8_dn2 = assign21520_e39736_d_n2;
        locals.var_t8_dn3 = assign21520_e39736_d_n3;
        locals.var_t8_dn4 = assign21520_e39736_d_n4;
        locals.var_t8_dn5 = assign21520_e39736_d_n5;
        locals.var_t8_dn6 = assign21520_e39736_d_n6;
        locals.var_t8_dn7 = assign21520_e39736_d_n7;
        locals.var_t8_dn8 = assign21520_e39736_d_n8;
        locals.var_t8_dn9 = assign21520_e39736_d_n9;
        locals.var_t8_dn10 = assign21520_e39736_d_n10;
        locals.var_t8_dn11 = assign21520_e39736_d_n11;
        locals.var_t8_dn13 = assign21520_e39736_d_n13;
        locals.var_t8_dn14 = assign21520_e39736_d_n14;

        let (assign21530_e39747, assign21530_e39747_d_n0, assign21530_e39747_d_n2, assign21530_e39747_d_n3, assign21530_e39747_d_n4, assign21530_e39747_d_n5, assign21530_e39747_d_n6, assign21530_e39747_d_n7, assign21530_e39747_d_n8, assign21530_e39747_d_n9, assign21530_e39747_d_n10, assign21530_e39747_d_n11, assign21530_e39747_d_n13, assign21530_e39747_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21530_e39740: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign21530_e39742: f64 = (assign21530_e39740 - locals.var_t8);
        let assign21530_e39744: f64 = (assign21530_e39742 - 1.0);
        let assign21530_e39745: f64 = (locals.var_t8 / assign21530_e39744);
        (assign21530_e39745, (((locals.var_t8_dn0 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0) - locals.var_t8_dn0))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn2 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2) - locals.var_t8_dn2))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn3 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3) - locals.var_t8_dn3))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn4 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4) - locals.var_t8_dn4))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn5 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5) - locals.var_t8_dn5))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn6 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6) - locals.var_t8_dn6))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn7 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7) - locals.var_t8_dn7))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn8 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8) - locals.var_t8_dn8))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn9 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9) - locals.var_t8_dn9))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn10 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10) - locals.var_t8_dn10))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn11 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11) - locals.var_t8_dn11))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn13 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13) - locals.var_t8_dn13))) / (assign21530_e39744 * assign21530_e39744)), (((locals.var_t8_dn14 * assign21530_e39744) - (locals.var_t8 * (({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14) - locals.var_t8_dn14))) / (assign21530_e39744 * assign21530_e39744)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21530_e39747;
        locals.var_t4_dn0 = assign21530_e39747_d_n0;
        locals.var_t4_dn2 = assign21530_e39747_d_n2;
        locals.var_t4_dn3 = assign21530_e39747_d_n3;
        locals.var_t4_dn4 = assign21530_e39747_d_n4;
        locals.var_t4_dn5 = assign21530_e39747_d_n5;
        locals.var_t4_dn6 = assign21530_e39747_d_n6;
        locals.var_t4_dn7 = assign21530_e39747_d_n7;
        locals.var_t4_dn8 = assign21530_e39747_d_n8;
        locals.var_t4_dn9 = assign21530_e39747_d_n9;
        locals.var_t4_dn10 = assign21530_e39747_d_n10;
        locals.var_t4_dn11 = assign21530_e39747_d_n11;
        locals.var_t4_dn13 = assign21530_e39747_d_n13;
        locals.var_t4_dn14 = assign21530_e39747_d_n14;

        let (assign21540_e39753, assign21540_e39753_d_n0, assign21540_e39753_d_n2, assign21540_e39753_d_n3, assign21540_e39753_d_n4, assign21540_e39753_d_n5, assign21540_e39753_d_n6, assign21540_e39753_d_n7, assign21540_e39753_d_n8, assign21540_e39753_d_n9, assign21540_e39753_d_n10, assign21540_e39753_d_n11, assign21540_e39753_d_n13, assign21540_e39753_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21540_e39751: f64 = (locals.var_t8 * locals.var_t4);
        (assign21540_e39751, ((locals.var_t8_dn0 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn0)), ((locals.var_t8_dn2 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn2)), ((locals.var_t8_dn3 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn3)), ((locals.var_t8_dn4 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn4)), ((locals.var_t8_dn5 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn5)), ((locals.var_t8_dn6 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn6)), ((locals.var_t8_dn7 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn7)), ((locals.var_t8_dn8 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn8)), ((locals.var_t8_dn9 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn9)), ((locals.var_t8_dn10 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn10)), ((locals.var_t8_dn11 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn11)), ((locals.var_t8_dn13 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn13)), ((locals.var_t8_dn14 * locals.var_t4) + (locals.var_t8 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21540_e39753;
        locals.var_t5_dn0 = assign21540_e39753_d_n0;
        locals.var_t5_dn2 = assign21540_e39753_d_n2;
        locals.var_t5_dn3 = assign21540_e39753_d_n3;
        locals.var_t5_dn4 = assign21540_e39753_d_n4;
        locals.var_t5_dn5 = assign21540_e39753_d_n5;
        locals.var_t5_dn6 = assign21540_e39753_d_n6;
        locals.var_t5_dn7 = assign21540_e39753_d_n7;
        locals.var_t5_dn8 = assign21540_e39753_d_n8;
        locals.var_t5_dn9 = assign21540_e39753_d_n9;
        locals.var_t5_dn10 = assign21540_e39753_d_n10;
        locals.var_t5_dn11 = assign21540_e39753_d_n11;
        locals.var_t5_dn13 = assign21540_e39753_d_n13;
        locals.var_t5_dn14 = assign21540_e39753_d_n14;

        let (assign21550_e39761, assign21550_e39761_d_n0, assign21550_e39761_d_n2, assign21550_e39761_d_n3, assign21550_e39761_d_n4, assign21550_e39761_d_n5, assign21550_e39761_d_n6, assign21550_e39761_d_n7, assign21550_e39761_d_n8, assign21550_e39761_d_n9, assign21550_e39761_d_n10, assign21550_e39761_d_n11, assign21550_e39761_d_n13, assign21550_e39761_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21550_e39757: f64 = (locals.var_qm + locals.var_qdep);
        let assign21550_e39758: f64 = (-assign21550_e39757);
        let assign21550_e39759: f64 = (assign21550_e39758).ln();
        (assign21550_e39759, ((-(locals.var_qm_dn0 + locals.var_qdep_dn0)) / assign21550_e39758), ((-(locals.var_qm_dn2 + locals.var_qdep_dn2)) / assign21550_e39758), ((-(locals.var_qm_dn3 + locals.var_qdep_dn3)) / assign21550_e39758), ((-(locals.var_qm_dn4 + locals.var_qdep_dn4)) / assign21550_e39758), ((-(locals.var_qm_dn5 + locals.var_qdep_dn5)) / assign21550_e39758), ((-(locals.var_qm_dn6 + locals.var_qdep_dn6)) / assign21550_e39758), ((-(locals.var_qm_dn7 + locals.var_qdep_dn7)) / assign21550_e39758), ((-(locals.var_qm_dn8 + locals.var_qdep_dn8)) / assign21550_e39758), ((-(locals.var_qm_dn9 + locals.var_qdep_dn9)) / assign21550_e39758), ((-(locals.var_qm_dn10 + locals.var_qdep_dn10)) / assign21550_e39758), ((-(locals.var_qm_dn11 + locals.var_qdep_dn11)) / assign21550_e39758), ((-(locals.var_qm_dn13 + locals.var_qdep_dn13)) / assign21550_e39758), ((-(locals.var_qm_dn14 + locals.var_qdep_dn14)) / assign21550_e39758),)
    } else {
        (locals.var_qm_ln, locals.var_qm_ln_dn0, locals.var_qm_ln_dn2, locals.var_qm_ln_dn3, locals.var_qm_ln_dn4, locals.var_qm_ln_dn5, locals.var_qm_ln_dn6, locals.var_qm_ln_dn7, locals.var_qm_ln_dn8, locals.var_qm_ln_dn9, locals.var_qm_ln_dn10, locals.var_qm_ln_dn11, locals.var_qm_ln_dn13, locals.var_qm_ln_dn14,)
    }
};
        locals.var_qm_ln = assign21550_e39761;
        locals.var_qm_ln_dn0 = assign21550_e39761_d_n0;
        locals.var_qm_ln_dn2 = assign21550_e39761_d_n2;
        locals.var_qm_ln_dn3 = assign21550_e39761_d_n3;
        locals.var_qm_ln_dn4 = assign21550_e39761_d_n4;
        locals.var_qm_ln_dn5 = assign21550_e39761_d_n5;
        locals.var_qm_ln_dn6 = assign21550_e39761_d_n6;
        locals.var_qm_ln_dn7 = assign21550_e39761_d_n7;
        locals.var_qm_ln_dn8 = assign21550_e39761_d_n8;
        locals.var_qm_ln_dn9 = assign21550_e39761_d_n9;
        locals.var_qm_ln_dn10 = assign21550_e39761_d_n10;
        locals.var_qm_ln_dn11 = assign21550_e39761_d_n11;
        locals.var_qm_ln_dn13 = assign21550_e39761_d_n13;
        locals.var_qm_ln_dn14 = assign21550_e39761_d_n14;

        let (assign21560_e39807, assign21560_e39807_d_n0, assign21560_e39807_d_n2, assign21560_e39807_d_n3, assign21560_e39807_d_n4, assign21560_e39807_d_n5, assign21560_e39807_d_n6, assign21560_e39807_d_n7, assign21560_e39807_d_n8, assign21560_e39807_d_n9, assign21560_e39807_d_n10, assign21560_e39807_d_n11, assign21560_e39807_d_n13, assign21560_e39807_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21560_e39765: f64 = (locals.var_f0 - locals.var_qm);
        let assign21560_e39767: f64 = (-locals.var_qm);
        let (assign21560_e39782, assign21560_e39782_d_n0, assign21560_e39782_d_n2, assign21560_e39782_d_n3, assign21560_e39782_d_n4, assign21560_e39782_d_n5, assign21560_e39782_d_n6, assign21560_e39782_d_n7, assign21560_e39782_d_n8, assign21560_e39782_d_n9, assign21560_e39782_d_n10, assign21560_e39782_d_n11, assign21560_e39782_d_n13, assign21560_e39782_d_n14,) = {
            if (!(assign21560_e39767 > 1e-38)) {
                let assign21560_e39772: f64 = (-87.498233534);
                (assign21560_e39772, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign21560_e39774: f64 = (-locals.var_qm);
                let (assign21560_e39781, assign21560_e39781_d_n0, assign21560_e39781_d_n2, assign21560_e39781_d_n3, assign21560_e39781_d_n4, assign21560_e39781_d_n5, assign21560_e39781_d_n6, assign21560_e39781_d_n7, assign21560_e39781_d_n8, assign21560_e39781_d_n9, assign21560_e39781_d_n10, assign21560_e39781_d_n11, assign21560_e39781_d_n13, assign21560_e39781_d_n14,) = {
                    if (assign21560_e39774 > 1e-38) {
                        let assign21560_e39778: f64 = (-locals.var_qm);
                        let assign21560_e39779: f64 = (assign21560_e39778).ln();
                        (assign21560_e39779, ((-locals.var_qm_dn0) / assign21560_e39778), ((-locals.var_qm_dn2) / assign21560_e39778), ((-locals.var_qm_dn3) / assign21560_e39778), ((-locals.var_qm_dn4) / assign21560_e39778), ((-locals.var_qm_dn5) / assign21560_e39778), ((-locals.var_qm_dn6) / assign21560_e39778), ((-locals.var_qm_dn7) / assign21560_e39778), ((-locals.var_qm_dn8) / assign21560_e39778), ((-locals.var_qm_dn9) / assign21560_e39778), ((-locals.var_qm_dn10) / assign21560_e39778), ((-locals.var_qm_dn11) / assign21560_e39778), ((-locals.var_qm_dn13) / assign21560_e39778), ((-locals.var_qm_dn14) / assign21560_e39778),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign21560_e39781, assign21560_e39781_d_n0, assign21560_e39781_d_n2, assign21560_e39781_d_n3, assign21560_e39781_d_n4, assign21560_e39781_d_n5, assign21560_e39781_d_n6, assign21560_e39781_d_n7, assign21560_e39781_d_n8, assign21560_e39781_d_n9, assign21560_e39781_d_n10, assign21560_e39781_d_n11, assign21560_e39781_d_n13, assign21560_e39781_d_n14,)
            }
        };
        let assign21560_e39783: f64 = (assign21560_e39765 + assign21560_e39782);
        let (assign21560_e39797, assign21560_e39797_d_n0, assign21560_e39797_d_n2, assign21560_e39797_d_n3, assign21560_e39797_d_n4, assign21560_e39797_d_n5, assign21560_e39797_d_n6, assign21560_e39797_d_n7, assign21560_e39797_d_n8, assign21560_e39797_d_n9, assign21560_e39797_d_n10, assign21560_e39797_d_n11, assign21560_e39797_d_n13, assign21560_e39797_d_n14,) = {
            if (!(locals.var_t5 > 1e-38)) {
                let assign21560_e39789: f64 = (-87.498233534);
                (assign21560_e39789, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (assign21560_e39796, assign21560_e39796_d_n0, assign21560_e39796_d_n2, assign21560_e39796_d_n3, assign21560_e39796_d_n4, assign21560_e39796_d_n5, assign21560_e39796_d_n6, assign21560_e39796_d_n7, assign21560_e39796_d_n8, assign21560_e39796_d_n9, assign21560_e39796_d_n10, assign21560_e39796_d_n11, assign21560_e39796_d_n13, assign21560_e39796_d_n14,) = {
                    if (locals.var_t5 > 1e-38) {
                        let assign21560_e39794: f64 = (locals.var_t5).ln();
                        (assign21560_e39794, (locals.var_t5_dn0 / locals.var_t5), (locals.var_t5_dn2 / locals.var_t5), (locals.var_t5_dn3 / locals.var_t5), (locals.var_t5_dn4 / locals.var_t5), (locals.var_t5_dn5 / locals.var_t5), (locals.var_t5_dn6 / locals.var_t5), (locals.var_t5_dn7 / locals.var_t5), (locals.var_t5_dn8 / locals.var_t5), (locals.var_t5_dn9 / locals.var_t5), (locals.var_t5_dn10 / locals.var_t5), (locals.var_t5_dn11 / locals.var_t5), (locals.var_t5_dn13 / locals.var_t5), (locals.var_t5_dn14 / locals.var_t5),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign21560_e39796, assign21560_e39796_d_n0, assign21560_e39796_d_n2, assign21560_e39796_d_n3, assign21560_e39796_d_n4, assign21560_e39796_d_n5, assign21560_e39796_d_n6, assign21560_e39796_d_n7, assign21560_e39796_d_n8, assign21560_e39796_d_n9, assign21560_e39796_d_n10, assign21560_e39796_d_n11, assign21560_e39796_d_n13, assign21560_e39796_d_n14,)
            }
        };
        let assign21560_e39798: f64 = (assign21560_e39783 + assign21560_e39797);
        let assign21560_e39802: f64 = (0.666666667 * locals.var_qm_ln);
        let assign21560_e39803: f64 = (assign21560_e39802).exp();
        let assign21560_e39804: f64 = (locals.var_qmfactorcvfinal * assign21560_e39803);
        let assign21560_e39805: f64 = (assign21560_e39798 + assign21560_e39804);
        (assign21560_e39805, ((((locals.var_f0_dn0 - locals.var_qm_dn0) + assign21560_e39782_d_n0) + assign21560_e39797_d_n0) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn0)))), ((((locals.var_f0_dn2 - locals.var_qm_dn2) + assign21560_e39782_d_n2) + assign21560_e39797_d_n2) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn2)))), ((((locals.var_f0_dn3 - locals.var_qm_dn3) + assign21560_e39782_d_n3) + assign21560_e39797_d_n3) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn3)))), ((((locals.var_f0_dn4 - locals.var_qm_dn4) + assign21560_e39782_d_n4) + assign21560_e39797_d_n4) + ((locals.var_qmfactorcvfinal_dn4 * assign21560_e39803) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn4))))), ((((locals.var_f0_dn5 - locals.var_qm_dn5) + assign21560_e39782_d_n5) + assign21560_e39797_d_n5) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn5)))), ((((locals.var_f0_dn6 - locals.var_qm_dn6) + assign21560_e39782_d_n6) + assign21560_e39797_d_n6) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn6)))), ((((locals.var_f0_dn7 - locals.var_qm_dn7) + assign21560_e39782_d_n7) + assign21560_e39797_d_n7) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn7)))), ((((locals.var_f0_dn8 - locals.var_qm_dn8) + assign21560_e39782_d_n8) + assign21560_e39797_d_n8) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn8)))), ((((locals.var_f0_dn9 - locals.var_qm_dn9) + assign21560_e39782_d_n9) + assign21560_e39797_d_n9) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn9)))), ((((locals.var_f0_dn10 - locals.var_qm_dn10) + assign21560_e39782_d_n10) + assign21560_e39797_d_n10) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn10)))), ((((locals.var_f0_dn11 - locals.var_qm_dn11) + assign21560_e39782_d_n11) + assign21560_e39797_d_n11) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn11)))), ((((locals.var_f0_dn13 - locals.var_qm_dn13) + assign21560_e39782_d_n13) + assign21560_e39797_d_n13) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn13)))), ((((locals.var_f0_dn14 - locals.var_qm_dn14) + assign21560_e39782_d_n14) + assign21560_e39797_d_n14) + (locals.var_qmfactorcvfinal * (assign21560_e39803 * (0.666666667 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e0, locals.var_e0_dn0, locals.var_e0_dn2, locals.var_e0_dn3, locals.var_e0_dn4, locals.var_e0_dn5, locals.var_e0_dn6, locals.var_e0_dn7, locals.var_e0_dn8, locals.var_e0_dn9, locals.var_e0_dn10, locals.var_e0_dn11, locals.var_e0_dn13, locals.var_e0_dn14,)
    }
};
        locals.var_e0 = assign21560_e39807;
        locals.var_e0_dn0 = assign21560_e39807_d_n0;
        locals.var_e0_dn2 = assign21560_e39807_d_n2;
        locals.var_e0_dn3 = assign21560_e39807_d_n3;
        locals.var_e0_dn4 = assign21560_e39807_d_n4;
        locals.var_e0_dn5 = assign21560_e39807_d_n5;
        locals.var_e0_dn6 = assign21560_e39807_d_n6;
        locals.var_e0_dn7 = assign21560_e39807_d_n7;
        locals.var_e0_dn8 = assign21560_e39807_d_n8;
        locals.var_e0_dn9 = assign21560_e39807_d_n9;
        locals.var_e0_dn10 = assign21560_e39807_d_n10;
        locals.var_e0_dn11 = assign21560_e39807_d_n11;
        locals.var_e0_dn13 = assign21560_e39807_d_n13;
        locals.var_e0_dn14 = assign21560_e39807_d_n14;

        let (assign21570_e39836, assign21570_e39836_d_n0, assign21570_e39836_d_n2, assign21570_e39836_d_n3, assign21570_e39836_d_n4, assign21570_e39836_d_n5, assign21570_e39836_d_n6, assign21570_e39836_d_n7, assign21570_e39836_d_n8, assign21570_e39836_d_n9, assign21570_e39836_d_n10, assign21570_e39836_d_n11, assign21570_e39836_d_n13, assign21570_e39836_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21570_e39810: f64 = (-1.0);
        let assign21570_e39813: f64 = (1.0 / locals.var_qm);
        let assign21570_e39814: f64 = (assign21570_e39810 + assign21570_e39813);
        let assign21570_e39817: f64 = (2.0 / locals.var_t8);
        let assign21570_e39819: f64 = (assign21570_e39817 - locals.var_t4);
        let assign21570_e39821: f64 = (assign21570_e39819 - 1.0);
        let assign21570_e39823: f64 = (assign21570_e39821 * locals.var_rc);
        let assign21570_e39824: f64 = (assign21570_e39814 + assign21570_e39823);
        let assign21570_e39827: f64 = (0.666666667 * locals.var_qmfactorcvfinal);
        let assign21570_e39829: f64 = (-0.333333333);
        let assign21570_e39831: f64 = (assign21570_e39829 * locals.var_qm_ln);
        let assign21570_e39832: f64 = (assign21570_e39831).exp();
        let assign21570_e39833: f64 = (assign21570_e39827 * assign21570_e39832);
        let assign21570_e39834: f64 = (assign21570_e39824 - assign21570_e39833);
        (assign21570_e39834, (((-(locals.var_qm_dn0 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn0) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn0) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn0)))), (((-(locals.var_qm_dn2 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn2) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn2) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn2)))), (((-(locals.var_qm_dn3 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn3) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn3) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn3)))), (((-(locals.var_qm_dn4 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn4) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn4) * locals.var_rc)) - (((0.666666667 * locals.var_qmfactorcvfinal_dn4) * assign21570_e39832) + (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn4))))), (((-(locals.var_qm_dn5 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn5) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn5) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn5)))), (((-(locals.var_qm_dn6 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn6) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn6) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn6)))), (((-(locals.var_qm_dn7 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn7) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn7) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn7)))), (((-(locals.var_qm_dn8 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn8) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn8) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn8)))), (((-(locals.var_qm_dn9 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn9) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn9) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn9)))), (((-(locals.var_qm_dn10 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn10) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn10) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn10)))), (((-(locals.var_qm_dn11 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn11) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn11) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn11)))), (((-(locals.var_qm_dn13 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn13) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn13) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn13)))), (((-(locals.var_qm_dn14 / (locals.var_qm * locals.var_qm))) + (((-((2.0 * locals.var_t8_dn14) / (locals.var_t8 * locals.var_t8))) - locals.var_t4_dn14) * locals.var_rc)) - (assign21570_e39827 * (assign21570_e39832 * (assign21570_e39829 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e1, locals.var_e1_dn0, locals.var_e1_dn2, locals.var_e1_dn3, locals.var_e1_dn4, locals.var_e1_dn5, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, locals.var_e1_dn10, locals.var_e1_dn11, locals.var_e1_dn13, locals.var_e1_dn14,)
    }
};
        locals.var_e1 = assign21570_e39836;
        locals.var_e1_dn0 = assign21570_e39836_d_n0;
        locals.var_e1_dn2 = assign21570_e39836_d_n2;
        locals.var_e1_dn3 = assign21570_e39836_d_n3;
        locals.var_e1_dn4 = assign21570_e39836_d_n4;
        locals.var_e1_dn5 = assign21570_e39836_d_n5;
        locals.var_e1_dn6 = assign21570_e39836_d_n6;
        locals.var_e1_dn7 = assign21570_e39836_d_n7;
        locals.var_e1_dn8 = assign21570_e39836_d_n8;
        locals.var_e1_dn9 = assign21570_e39836_d_n9;
        locals.var_e1_dn10 = assign21570_e39836_d_n10;
        locals.var_e1_dn11 = assign21570_e39836_d_n11;
        locals.var_e1_dn13 = assign21570_e39836_d_n13;
        locals.var_e1_dn14 = assign21570_e39836_d_n14;

        let (assign21580_e39857, assign21580_e39857_d_n0, assign21580_e39857_d_n2, assign21580_e39857_d_n3, assign21580_e39857_d_n4, assign21580_e39857_d_n5, assign21580_e39857_d_n6, assign21580_e39857_d_n7, assign21580_e39857_d_n8, assign21580_e39857_d_n9, assign21580_e39857_d_n10, assign21580_e39857_d_n11, assign21580_e39857_d_n13, assign21580_e39857_d_n14,) = {
    if (locals.var_guard375 != 0.0) {
        let assign21580_e39839: f64 = (-1.0);
        let assign21580_e39842: f64 = (locals.var_qm * locals.var_qm);
        let assign21580_e39843: f64 = (assign21580_e39839 / assign21580_e39842);
        let assign21580_e39846: f64 = (2.0 / 9.0);
        let assign21580_e39848: f64 = (assign21580_e39846 * locals.var_qmfactorcvfinal);
        let assign21580_e39850: f64 = (-1.333333333);
        let assign21580_e39852: f64 = (assign21580_e39850 * locals.var_qm_ln);
        let assign21580_e39853: f64 = (assign21580_e39852).exp();
        let assign21580_e39854: f64 = (assign21580_e39848 * assign21580_e39853);
        let assign21580_e39855: f64 = (assign21580_e39843 - assign21580_e39854);
        (assign21580_e39855, ((-((assign21580_e39839 * ((locals.var_qm_dn0 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn0))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn0)))), ((-((assign21580_e39839 * ((locals.var_qm_dn2 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn2))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn2)))), ((-((assign21580_e39839 * ((locals.var_qm_dn3 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn3))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn3)))), ((-((assign21580_e39839 * ((locals.var_qm_dn4 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn4))) / (assign21580_e39842 * assign21580_e39842))) - (((assign21580_e39846 * locals.var_qmfactorcvfinal_dn4) * assign21580_e39853) + (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn4))))), ((-((assign21580_e39839 * ((locals.var_qm_dn5 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn5))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn5)))), ((-((assign21580_e39839 * ((locals.var_qm_dn6 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn6))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn6)))), ((-((assign21580_e39839 * ((locals.var_qm_dn7 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn7))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn7)))), ((-((assign21580_e39839 * ((locals.var_qm_dn8 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn8))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn8)))), ((-((assign21580_e39839 * ((locals.var_qm_dn9 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn9))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn9)))), ((-((assign21580_e39839 * ((locals.var_qm_dn10 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn10))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn10)))), ((-((assign21580_e39839 * ((locals.var_qm_dn11 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn11))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn11)))), ((-((assign21580_e39839 * ((locals.var_qm_dn13 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn13))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn13)))), ((-((assign21580_e39839 * ((locals.var_qm_dn14 * locals.var_qm) + (locals.var_qm * locals.var_qm_dn14))) / (assign21580_e39842 * assign21580_e39842))) - (assign21580_e39848 * (assign21580_e39853 * (assign21580_e39850 * locals.var_qm_ln_dn14)))),)
    } else {
        (locals.var_e2, locals.var_e2_dn0, locals.var_e2_dn2, locals.var_e2_dn3, locals.var_e2_dn4, locals.var_e2_dn5, locals.var_e2_dn6, locals.var_e2_dn7, locals.var_e2_dn8, locals.var_e2_dn9, locals.var_e2_dn10, locals.var_e2_dn11, locals.var_e2_dn13, locals.var_e2_dn14,)
    }
};
        locals.var_e2 = assign21580_e39857;
        locals.var_e2_dn0 = assign21580_e39857_d_n0;
        locals.var_e2_dn2 = assign21580_e39857_d_n2;
        locals.var_e2_dn3 = assign21580_e39857_d_n3;
        locals.var_e2_dn4 = assign21580_e39857_d_n4;
        locals.var_e2_dn5 = assign21580_e39857_d_n5;
        locals.var_e2_dn6 = assign21580_e39857_d_n6;
        locals.var_e2_dn7 = assign21580_e39857_d_n7;
        locals.var_e2_dn8 = assign21580_e39857_d_n8;
        locals.var_e2_dn9 = assign21580_e39857_d_n9;
        locals.var_e2_dn10 = assign21580_e39857_d_n10;
        locals.var_e2_dn11 = assign21580_e39857_d_n11;
        locals.var_e2_dn13 = assign21580_e39857_d_n13;
        locals.var_e2_dn14 = assign21580_e39857_d_n14;

    }
}
