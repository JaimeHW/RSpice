#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13690_e19432: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19433: f64 = (1.0 + assign13690_e19432);
        let assign13690_e19435: f64 = (assign13690_e19433 - 1e-6);
        let assign13690_e19437: f64 = (-10000.0);
        let assign13690_e19439: f64 = (assign13690_e19437 * 0.001);
        let (assign13690_e19500, assign13690_e19500_d_n4,) = {
    if (!(assign13690_e19435 < assign13690_e19439)) {
        let assign13690_e19446: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19447: f64 = (1.0 + assign13690_e19446);
        let assign13690_e19449: f64 = (assign13690_e19447 - 1e-6);
        let assign13690_e19453: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19454: f64 = (1.0 + assign13690_e19453);
        let assign13690_e19456: f64 = (assign13690_e19454 - 1e-6);
        let assign13690_e19460: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19461: f64 = (1.0 + assign13690_e19460);
        let assign13690_e19463: f64 = (assign13690_e19461 - 1e-6);
        let assign13690_e19464: f64 = (assign13690_e19456 * assign13690_e19463);
        let assign13690_e19467: f64 = (4.0 * 0.001);
        let assign13690_e19469: f64 = (assign13690_e19467 * 0.001);
        let assign13690_e19470: f64 = (assign13690_e19464 + assign13690_e19469);
        let assign13690_e19471: f64 = (assign13690_e19470).sqrt();
        let assign13690_e19472: f64 = (assign13690_e19449 + assign13690_e19471);
        let assign13690_e19473: f64 = (0.5 * assign13690_e19472);
        (assign13690_e19473, (0.5 * ((locals.var_c0si1_i * locals.var_deltemp_dn4) + ((((locals.var_c0si1_i * locals.var_deltemp_dn4) * assign13690_e19463) + (assign13690_e19456 * (locals.var_c0si1_i * locals.var_deltemp_dn4))) / (2.0 * assign13690_e19471)))),)
    } else {
        let assign13690_e19477: f64 = (locals.var_c0si1_i * locals.var_deltemp);
        let assign13690_e19478: f64 = (1.0 + assign13690_e19477);
        let assign13690_e19480: f64 = (assign13690_e19478 - 1e-6);
        let assign13690_e19482: f64 = (-10000.0);
        let assign13690_e19484: f64 = (assign13690_e19482 * 0.001);
        let (assign13690_e19499, assign13690_e19499_d_n4,) = {
            if (assign13690_e19480 < assign13690_e19484) {
                let assign13690_e19487: f64 = (-0.001);
                let assign13690_e19489: f64 = (assign13690_e19487 * 0.001);
                let assign13690_e19493: f64 = (locals.var_c0si1_i * locals.var_deltemp);
                let assign13690_e19494: f64 = (1.0 + assign13690_e19493);
                let assign13690_e19496: f64 = (assign13690_e19494 - 1e-6);
                let assign13690_e19497: f64 = (assign13690_e19489 / assign13690_e19496);
                (assign13690_e19497, (-((assign13690_e19489 * (locals.var_c0si1_i * locals.var_deltemp_dn4)) / (assign13690_e19496 * assign13690_e19496))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13690_e19499, assign13690_e19499_d_n4,)
    }
};
        let assign13690_e19501: f64 = (locals.var_c0si_i * assign13690_e19500);
        locals.var_c0si_t = assign13690_e19501;
        locals.var_c0si_t_dn4 = (locals.var_c0si_i * assign13690_e19500_d_n4);
        locals.var_c0si_t_rv = 0.0;

        let assign13700_e19506: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19507: f64 = (1.0 + assign13700_e19506);
        let assign13700_e19509: f64 = (assign13700_e19507 - 1e-6);
        let assign13700_e19511: f64 = (-10000.0);
        let assign13700_e19513: f64 = (assign13700_e19511 * 0.001);
        let (assign13700_e19574, assign13700_e19574_d_n4,) = {
    if (!(assign13700_e19509 < assign13700_e19513)) {
        let assign13700_e19520: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19521: f64 = (1.0 + assign13700_e19520);
        let assign13700_e19523: f64 = (assign13700_e19521 - 1e-6);
        let assign13700_e19527: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19528: f64 = (1.0 + assign13700_e19527);
        let assign13700_e19530: f64 = (assign13700_e19528 - 1e-6);
        let assign13700_e19534: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19535: f64 = (1.0 + assign13700_e19534);
        let assign13700_e19537: f64 = (assign13700_e19535 - 1e-6);
        let assign13700_e19538: f64 = (assign13700_e19530 * assign13700_e19537);
        let assign13700_e19541: f64 = (4.0 * 0.001);
        let assign13700_e19543: f64 = (assign13700_e19541 * 0.001);
        let assign13700_e19544: f64 = (assign13700_e19538 + assign13700_e19543);
        let assign13700_e19545: f64 = (assign13700_e19544).sqrt();
        let assign13700_e19546: f64 = (assign13700_e19523 + assign13700_e19545);
        let assign13700_e19547: f64 = (0.5 * assign13700_e19546);
        (assign13700_e19547, (0.5 * ((locals.var_c0sisat1_i * locals.var_deltemp_dn4) + ((((locals.var_c0sisat1_i * locals.var_deltemp_dn4) * assign13700_e19537) + (assign13700_e19530 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4))) / (2.0 * assign13700_e19545)))),)
    } else {
        let assign13700_e19551: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
        let assign13700_e19552: f64 = (1.0 + assign13700_e19551);
        let assign13700_e19554: f64 = (assign13700_e19552 - 1e-6);
        let assign13700_e19556: f64 = (-10000.0);
        let assign13700_e19558: f64 = (assign13700_e19556 * 0.001);
        let (assign13700_e19573, assign13700_e19573_d_n4,) = {
            if (assign13700_e19554 < assign13700_e19558) {
                let assign13700_e19561: f64 = (-0.001);
                let assign13700_e19563: f64 = (assign13700_e19561 * 0.001);
                let assign13700_e19567: f64 = (locals.var_c0sisat1_i * locals.var_deltemp);
                let assign13700_e19568: f64 = (1.0 + assign13700_e19567);
                let assign13700_e19570: f64 = (assign13700_e19568 - 1e-6);
                let assign13700_e19571: f64 = (assign13700_e19563 / assign13700_e19570);
                (assign13700_e19571, (-((assign13700_e19563 * (locals.var_c0sisat1_i * locals.var_deltemp_dn4)) / (assign13700_e19570 * assign13700_e19570))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13700_e19573, assign13700_e19573_d_n4,)
    }
};
        let assign13700_e19575: f64 = (locals.var_c0sisat_i * assign13700_e19574);
        locals.var_c0sisat_t = assign13700_e19575;
        locals.var_c0sisat_t_dn4 = (locals.var_c0sisat_i * assign13700_e19574_d_n4);
        locals.var_c0sisat_t_rv = 0.0;

        let assign13710_e19580: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19581: f64 = (1.0 + assign13710_e19580);
        let assign13710_e19583: f64 = (assign13710_e19581 - 1e-6);
        let assign13710_e19585: f64 = (-10000.0);
        let assign13710_e19587: f64 = (assign13710_e19585 * 0.001);
        let (assign13710_e19648, assign13710_e19648_d_n4,) = {
    if (!(assign13710_e19583 < assign13710_e19587)) {
        let assign13710_e19594: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19595: f64 = (1.0 + assign13710_e19594);
        let assign13710_e19597: f64 = (assign13710_e19595 - 1e-6);
        let assign13710_e19601: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19602: f64 = (1.0 + assign13710_e19601);
        let assign13710_e19604: f64 = (assign13710_e19602 - 1e-6);
        let assign13710_e19608: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19609: f64 = (1.0 + assign13710_e19608);
        let assign13710_e19611: f64 = (assign13710_e19609 - 1e-6);
        let assign13710_e19612: f64 = (assign13710_e19604 * assign13710_e19611);
        let assign13710_e19615: f64 = (4.0 * 0.001);
        let assign13710_e19617: f64 = (assign13710_e19615 * 0.001);
        let assign13710_e19618: f64 = (assign13710_e19612 + assign13710_e19617);
        let assign13710_e19619: f64 = (assign13710_e19618).sqrt();
        let assign13710_e19620: f64 = (assign13710_e19597 + assign13710_e19619);
        let assign13710_e19621: f64 = (0.5 * assign13710_e19620);
        (assign13710_e19621, (0.5 * ((p.p889 * locals.var_deltemp_dn4) + ((((p.p889 * locals.var_deltemp_dn4) * assign13710_e19611) + (assign13710_e19604 * (p.p889 * locals.var_deltemp_dn4))) / (2.0 * assign13710_e19619)))),)
    } else {
        let assign13710_e19625: f64 = (p.p889 * locals.var_deltemp);
        let assign13710_e19626: f64 = (1.0 + assign13710_e19625);
        let assign13710_e19628: f64 = (assign13710_e19626 - 1e-6);
        let assign13710_e19630: f64 = (-10000.0);
        let assign13710_e19632: f64 = (assign13710_e19630 * 0.001);
        let (assign13710_e19647, assign13710_e19647_d_n4,) = {
            if (assign13710_e19628 < assign13710_e19632) {
                let assign13710_e19635: f64 = (-0.001);
                let assign13710_e19637: f64 = (assign13710_e19635 * 0.001);
                let assign13710_e19641: f64 = (p.p889 * locals.var_deltemp);
                let assign13710_e19642: f64 = (1.0 + assign13710_e19641);
                let assign13710_e19644: f64 = (assign13710_e19642 - 1e-6);
                let assign13710_e19645: f64 = (assign13710_e19637 / assign13710_e19644);
                (assign13710_e19645, (-((assign13710_e19637 * (p.p889 * locals.var_deltemp_dn4)) / (assign13710_e19644 * assign13710_e19644))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13710_e19647, assign13710_e19647_d_n4,)
    }
};
        let assign13710_e19649: f64 = (p.p701 * assign13710_e19648);
        locals.var_cjs_t = assign13710_e19649;
        locals.var_cjs_t_dn4 = (p.p701 * assign13710_e19648_d_n4);
        locals.var_cjs_t_rv = 0.0;

        let assign13720_e19654: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19655: f64 = (1.0 + assign13720_e19654);
        let assign13720_e19657: f64 = (assign13720_e19655 - 1e-6);
        let assign13720_e19659: f64 = (-10000.0);
        let assign13720_e19661: f64 = (assign13720_e19659 * 0.001);
        let (assign13720_e19722, assign13720_e19722_d_n4,) = {
    if (!(assign13720_e19657 < assign13720_e19661)) {
        let assign13720_e19668: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19669: f64 = (1.0 + assign13720_e19668);
        let assign13720_e19671: f64 = (assign13720_e19669 - 1e-6);
        let assign13720_e19675: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19676: f64 = (1.0 + assign13720_e19675);
        let assign13720_e19678: f64 = (assign13720_e19676 - 1e-6);
        let assign13720_e19682: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19683: f64 = (1.0 + assign13720_e19682);
        let assign13720_e19685: f64 = (assign13720_e19683 - 1e-6);
        let assign13720_e19686: f64 = (assign13720_e19678 * assign13720_e19685);
        let assign13720_e19689: f64 = (4.0 * 0.001);
        let assign13720_e19691: f64 = (assign13720_e19689 * 0.001);
        let assign13720_e19692: f64 = (assign13720_e19686 + assign13720_e19691);
        let assign13720_e19693: f64 = (assign13720_e19692).sqrt();
        let assign13720_e19694: f64 = (assign13720_e19671 + assign13720_e19693);
        let assign13720_e19695: f64 = (0.5 * assign13720_e19694);
        (assign13720_e19695, (0.5 * ((p.p889 * locals.var_deltemp_dn4) + ((((p.p889 * locals.var_deltemp_dn4) * assign13720_e19685) + (assign13720_e19678 * (p.p889 * locals.var_deltemp_dn4))) / (2.0 * assign13720_e19693)))),)
    } else {
        let assign13720_e19699: f64 = (p.p889 * locals.var_deltemp);
        let assign13720_e19700: f64 = (1.0 + assign13720_e19699);
        let assign13720_e19702: f64 = (assign13720_e19700 - 1e-6);
        let assign13720_e19704: f64 = (-10000.0);
        let assign13720_e19706: f64 = (assign13720_e19704 * 0.001);
        let (assign13720_e19721, assign13720_e19721_d_n4,) = {
            if (assign13720_e19702 < assign13720_e19706) {
                let assign13720_e19709: f64 = (-0.001);
                let assign13720_e19711: f64 = (assign13720_e19709 * 0.001);
                let assign13720_e19715: f64 = (p.p889 * locals.var_deltemp);
                let assign13720_e19716: f64 = (1.0 + assign13720_e19715);
                let assign13720_e19718: f64 = (assign13720_e19716 - 1e-6);
                let assign13720_e19719: f64 = (assign13720_e19711 / assign13720_e19718);
                (assign13720_e19719, (-((assign13720_e19711 * (p.p889 * locals.var_deltemp_dn4)) / (assign13720_e19718 * assign13720_e19718))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13720_e19721, assign13720_e19721_d_n4,)
    }
};
        let assign13720_e19723: f64 = (p.p702 * assign13720_e19722);
        locals.var_cjd_t = assign13720_e19723;
        locals.var_cjd_t_dn4 = (p.p702 * assign13720_e19722_d_n4);
        locals.var_cjd_t_rv = 0.0;

        let assign13730_e19728: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19729: f64 = (1.0 + assign13730_e19728);
        let assign13730_e19731: f64 = (assign13730_e19729 - 1e-6);
        let assign13730_e19733: f64 = (-10000.0);
        let assign13730_e19735: f64 = (assign13730_e19733 * 0.001);
        let (assign13730_e19796, assign13730_e19796_d_n4,) = {
    if (!(assign13730_e19731 < assign13730_e19735)) {
        let assign13730_e19742: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19743: f64 = (1.0 + assign13730_e19742);
        let assign13730_e19745: f64 = (assign13730_e19743 - 1e-6);
        let assign13730_e19749: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19750: f64 = (1.0 + assign13730_e19749);
        let assign13730_e19752: f64 = (assign13730_e19750 - 1e-6);
        let assign13730_e19756: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19757: f64 = (1.0 + assign13730_e19756);
        let assign13730_e19759: f64 = (assign13730_e19757 - 1e-6);
        let assign13730_e19760: f64 = (assign13730_e19752 * assign13730_e19759);
        let assign13730_e19763: f64 = (4.0 * 0.001);
        let assign13730_e19765: f64 = (assign13730_e19763 * 0.001);
        let assign13730_e19766: f64 = (assign13730_e19760 + assign13730_e19765);
        let assign13730_e19767: f64 = (assign13730_e19766).sqrt();
        let assign13730_e19768: f64 = (assign13730_e19745 + assign13730_e19767);
        let assign13730_e19769: f64 = (0.5 * assign13730_e19768);
        (assign13730_e19769, (0.5 * ((p.p890 * locals.var_deltemp_dn4) + ((((p.p890 * locals.var_deltemp_dn4) * assign13730_e19759) + (assign13730_e19752 * (p.p890 * locals.var_deltemp_dn4))) / (2.0 * assign13730_e19767)))),)
    } else {
        let assign13730_e19773: f64 = (p.p890 * locals.var_deltemp);
        let assign13730_e19774: f64 = (1.0 + assign13730_e19773);
        let assign13730_e19776: f64 = (assign13730_e19774 - 1e-6);
        let assign13730_e19778: f64 = (-10000.0);
        let assign13730_e19780: f64 = (assign13730_e19778 * 0.001);
        let (assign13730_e19795, assign13730_e19795_d_n4,) = {
            if (assign13730_e19776 < assign13730_e19780) {
                let assign13730_e19783: f64 = (-0.001);
                let assign13730_e19785: f64 = (assign13730_e19783 * 0.001);
                let assign13730_e19789: f64 = (p.p890 * locals.var_deltemp);
                let assign13730_e19790: f64 = (1.0 + assign13730_e19789);
                let assign13730_e19792: f64 = (assign13730_e19790 - 1e-6);
                let assign13730_e19793: f64 = (assign13730_e19785 / assign13730_e19792);
                (assign13730_e19793, (-((assign13730_e19785 * (p.p890 * locals.var_deltemp_dn4)) / (assign13730_e19792 * assign13730_e19792))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13730_e19795, assign13730_e19795_d_n4,)
    }
};
        let assign13730_e19797: f64 = (p.p703 * assign13730_e19796);
        locals.var_cjsws_t = assign13730_e19797;
        locals.var_cjsws_t_dn4 = (p.p703 * assign13730_e19796_d_n4);
        locals.var_cjsws_t_rv = 0.0;

        let assign13740_e19802: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19803: f64 = (1.0 + assign13740_e19802);
        let assign13740_e19805: f64 = (assign13740_e19803 - 1e-6);
        let assign13740_e19807: f64 = (-10000.0);
        let assign13740_e19809: f64 = (assign13740_e19807 * 0.001);
        let (assign13740_e19870, assign13740_e19870_d_n4,) = {
    if (!(assign13740_e19805 < assign13740_e19809)) {
        let assign13740_e19816: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19817: f64 = (1.0 + assign13740_e19816);
        let assign13740_e19819: f64 = (assign13740_e19817 - 1e-6);
        let assign13740_e19823: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19824: f64 = (1.0 + assign13740_e19823);
        let assign13740_e19826: f64 = (assign13740_e19824 - 1e-6);
        let assign13740_e19830: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19831: f64 = (1.0 + assign13740_e19830);
        let assign13740_e19833: f64 = (assign13740_e19831 - 1e-6);
        let assign13740_e19834: f64 = (assign13740_e19826 * assign13740_e19833);
        let assign13740_e19837: f64 = (4.0 * 0.001);
        let assign13740_e19839: f64 = (assign13740_e19837 * 0.001);
        let assign13740_e19840: f64 = (assign13740_e19834 + assign13740_e19839);
        let assign13740_e19841: f64 = (assign13740_e19840).sqrt();
        let assign13740_e19842: f64 = (assign13740_e19819 + assign13740_e19841);
        let assign13740_e19843: f64 = (0.5 * assign13740_e19842);
        (assign13740_e19843, (0.5 * ((p.p890 * locals.var_deltemp_dn4) + ((((p.p890 * locals.var_deltemp_dn4) * assign13740_e19833) + (assign13740_e19826 * (p.p890 * locals.var_deltemp_dn4))) / (2.0 * assign13740_e19841)))),)
    } else {
        let assign13740_e19847: f64 = (p.p890 * locals.var_deltemp);
        let assign13740_e19848: f64 = (1.0 + assign13740_e19847);
        let assign13740_e19850: f64 = (assign13740_e19848 - 1e-6);
        let assign13740_e19852: f64 = (-10000.0);
        let assign13740_e19854: f64 = (assign13740_e19852 * 0.001);
        let (assign13740_e19869, assign13740_e19869_d_n4,) = {
            if (assign13740_e19850 < assign13740_e19854) {
                let assign13740_e19857: f64 = (-0.001);
                let assign13740_e19859: f64 = (assign13740_e19857 * 0.001);
                let assign13740_e19863: f64 = (p.p890 * locals.var_deltemp);
                let assign13740_e19864: f64 = (1.0 + assign13740_e19863);
                let assign13740_e19866: f64 = (assign13740_e19864 - 1e-6);
                let assign13740_e19867: f64 = (assign13740_e19859 / assign13740_e19866);
                (assign13740_e19867, (-((assign13740_e19859 * (p.p890 * locals.var_deltemp_dn4)) / (assign13740_e19866 * assign13740_e19866))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13740_e19869, assign13740_e19869_d_n4,)
    }
};
        let assign13740_e19871: f64 = (p.p704 * assign13740_e19870);
        locals.var_cjswd_t = assign13740_e19871;
        locals.var_cjswd_t_dn4 = (p.p704 * assign13740_e19870_d_n4);
        locals.var_cjswd_t_rv = 0.0;

        let assign13750_e19876: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19877: f64 = (1.0 + assign13750_e19876);
        let assign13750_e19879: f64 = (assign13750_e19877 - 1e-6);
        let assign13750_e19881: f64 = (-10000.0);
        let assign13750_e19883: f64 = (assign13750_e19881 * 0.001);
        let (assign13750_e19944, assign13750_e19944_d_n4,) = {
    if (!(assign13750_e19879 < assign13750_e19883)) {
        let assign13750_e19890: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19891: f64 = (1.0 + assign13750_e19890);
        let assign13750_e19893: f64 = (assign13750_e19891 - 1e-6);
        let assign13750_e19897: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19898: f64 = (1.0 + assign13750_e19897);
        let assign13750_e19900: f64 = (assign13750_e19898 - 1e-6);
        let assign13750_e19904: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19905: f64 = (1.0 + assign13750_e19904);
        let assign13750_e19907: f64 = (assign13750_e19905 - 1e-6);
        let assign13750_e19908: f64 = (assign13750_e19900 * assign13750_e19907);
        let assign13750_e19911: f64 = (4.0 * 0.001);
        let assign13750_e19913: f64 = (assign13750_e19911 * 0.001);
        let assign13750_e19914: f64 = (assign13750_e19908 + assign13750_e19913);
        let assign13750_e19915: f64 = (assign13750_e19914).sqrt();
        let assign13750_e19916: f64 = (assign13750_e19893 + assign13750_e19915);
        let assign13750_e19917: f64 = (0.5 * assign13750_e19916);
        (assign13750_e19917, (0.5 * ((p.p891 * locals.var_deltemp_dn4) + ((((p.p891 * locals.var_deltemp_dn4) * assign13750_e19907) + (assign13750_e19900 * (p.p891 * locals.var_deltemp_dn4))) / (2.0 * assign13750_e19915)))),)
    } else {
        let assign13750_e19921: f64 = (p.p891 * locals.var_deltemp);
        let assign13750_e19922: f64 = (1.0 + assign13750_e19921);
        let assign13750_e19924: f64 = (assign13750_e19922 - 1e-6);
        let assign13750_e19926: f64 = (-10000.0);
        let assign13750_e19928: f64 = (assign13750_e19926 * 0.001);
        let (assign13750_e19943, assign13750_e19943_d_n4,) = {
            if (assign13750_e19924 < assign13750_e19928) {
                let assign13750_e19931: f64 = (-0.001);
                let assign13750_e19933: f64 = (assign13750_e19931 * 0.001);
                let assign13750_e19937: f64 = (p.p891 * locals.var_deltemp);
                let assign13750_e19938: f64 = (1.0 + assign13750_e19937);
                let assign13750_e19940: f64 = (assign13750_e19938 - 1e-6);
                let assign13750_e19941: f64 = (assign13750_e19933 / assign13750_e19940);
                (assign13750_e19941, (-((assign13750_e19933 * (p.p891 * locals.var_deltemp_dn4)) / (assign13750_e19940 * assign13750_e19940))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13750_e19943, assign13750_e19943_d_n4,)
    }
};
        let assign13750_e19945: f64 = (p.p705 * assign13750_e19944);
        locals.var_cjswgs_t = assign13750_e19945;
        locals.var_cjswgs_t_dn4 = (p.p705 * assign13750_e19944_d_n4);
        locals.var_cjswgs_t_rv = 0.0;

        let assign13760_e19950: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19951: f64 = (1.0 + assign13760_e19950);
        let assign13760_e19953: f64 = (assign13760_e19951 - 1e-6);
        let assign13760_e19955: f64 = (-10000.0);
        let assign13760_e19957: f64 = (assign13760_e19955 * 0.001);
        let (assign13760_e20018, assign13760_e20018_d_n4,) = {
    if (!(assign13760_e19953 < assign13760_e19957)) {
        let assign13760_e19964: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19965: f64 = (1.0 + assign13760_e19964);
        let assign13760_e19967: f64 = (assign13760_e19965 - 1e-6);
        let assign13760_e19971: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19972: f64 = (1.0 + assign13760_e19971);
        let assign13760_e19974: f64 = (assign13760_e19972 - 1e-6);
        let assign13760_e19978: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19979: f64 = (1.0 + assign13760_e19978);
        let assign13760_e19981: f64 = (assign13760_e19979 - 1e-6);
        let assign13760_e19982: f64 = (assign13760_e19974 * assign13760_e19981);
        let assign13760_e19985: f64 = (4.0 * 0.001);
        let assign13760_e19987: f64 = (assign13760_e19985 * 0.001);
        let assign13760_e19988: f64 = (assign13760_e19982 + assign13760_e19987);
        let assign13760_e19989: f64 = (assign13760_e19988).sqrt();
        let assign13760_e19990: f64 = (assign13760_e19967 + assign13760_e19989);
        let assign13760_e19991: f64 = (0.5 * assign13760_e19990);
        (assign13760_e19991, (0.5 * ((p.p891 * locals.var_deltemp_dn4) + ((((p.p891 * locals.var_deltemp_dn4) * assign13760_e19981) + (assign13760_e19974 * (p.p891 * locals.var_deltemp_dn4))) / (2.0 * assign13760_e19989)))),)
    } else {
        let assign13760_e19995: f64 = (p.p891 * locals.var_deltemp);
        let assign13760_e19996: f64 = (1.0 + assign13760_e19995);
        let assign13760_e19998: f64 = (assign13760_e19996 - 1e-6);
        let assign13760_e20000: f64 = (-10000.0);
        let assign13760_e20002: f64 = (assign13760_e20000 * 0.001);
        let (assign13760_e20017, assign13760_e20017_d_n4,) = {
            if (assign13760_e19998 < assign13760_e20002) {
                let assign13760_e20005: f64 = (-0.001);
                let assign13760_e20007: f64 = (assign13760_e20005 * 0.001);
                let assign13760_e20011: f64 = (p.p891 * locals.var_deltemp);
                let assign13760_e20012: f64 = (1.0 + assign13760_e20011);
                let assign13760_e20014: f64 = (assign13760_e20012 - 1e-6);
                let assign13760_e20015: f64 = (assign13760_e20007 / assign13760_e20014);
                (assign13760_e20015, (-((assign13760_e20007 * (p.p891 * locals.var_deltemp_dn4)) / (assign13760_e20014 * assign13760_e20014))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13760_e20017, assign13760_e20017_d_n4,)
    }
};
        let assign13760_e20019: f64 = (p.p706 * assign13760_e20018);
        locals.var_cjswgd_t = assign13760_e20019;
        locals.var_cjswgd_t_dn4 = (p.p706 * assign13760_e20018_d_n4);
        locals.var_cjswgd_t_rv = 0.0;

        let assign13770_e20023: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20024: f64 = (p.p707 - assign13770_e20023);
        let assign13770_e20026: f64 = (assign13770_e20024 - 0.01);
        let assign13770_e20028: f64 = (-10000.0);
        let assign13770_e20030: f64 = (assign13770_e20028 * 0.001);
        let (assign13770_e20091, assign13770_e20091_d_n4,) = {
    if (!(assign13770_e20026 < assign13770_e20030)) {
        let assign13770_e20037: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20038: f64 = (p.p707 - assign13770_e20037);
        let assign13770_e20040: f64 = (assign13770_e20038 - 0.01);
        let assign13770_e20044: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20045: f64 = (p.p707 - assign13770_e20044);
        let assign13770_e20047: f64 = (assign13770_e20045 - 0.01);
        let assign13770_e20051: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20052: f64 = (p.p707 - assign13770_e20051);
        let assign13770_e20054: f64 = (assign13770_e20052 - 0.01);
        let assign13770_e20055: f64 = (assign13770_e20047 * assign13770_e20054);
        let assign13770_e20058: f64 = (4.0 * 0.001);
        let assign13770_e20060: f64 = (assign13770_e20058 * 0.001);
        let assign13770_e20061: f64 = (assign13770_e20055 + assign13770_e20060);
        let assign13770_e20062: f64 = (assign13770_e20061).sqrt();
        let assign13770_e20063: f64 = (assign13770_e20040 + assign13770_e20062);
        let assign13770_e20064: f64 = (0.5 * assign13770_e20063);
        (assign13770_e20064, (0.5 * ((-(p.p892 * locals.var_deltemp_dn4)) + ((((-(p.p892 * locals.var_deltemp_dn4)) * assign13770_e20054) + (assign13770_e20047 * (-(p.p892 * locals.var_deltemp_dn4)))) / (2.0 * assign13770_e20062)))),)
    } else {
        let assign13770_e20068: f64 = (p.p892 * locals.var_deltemp);
        let assign13770_e20069: f64 = (p.p707 - assign13770_e20068);
        let assign13770_e20071: f64 = (assign13770_e20069 - 0.01);
        let assign13770_e20073: f64 = (-10000.0);
        let assign13770_e20075: f64 = (assign13770_e20073 * 0.001);
        let (assign13770_e20090, assign13770_e20090_d_n4,) = {
            if (assign13770_e20071 < assign13770_e20075) {
                let assign13770_e20078: f64 = (-0.001);
                let assign13770_e20080: f64 = (assign13770_e20078 * 0.001);
                let assign13770_e20084: f64 = (p.p892 * locals.var_deltemp);
                let assign13770_e20085: f64 = (p.p707 - assign13770_e20084);
                let assign13770_e20087: f64 = (assign13770_e20085 - 0.01);
                let assign13770_e20088: f64 = (assign13770_e20080 / assign13770_e20087);
                (assign13770_e20088, (-((assign13770_e20080 * (-(p.p892 * locals.var_deltemp_dn4))) / (assign13770_e20087 * assign13770_e20087))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13770_e20090, assign13770_e20090_d_n4,)
    }
};
        let assign13770_e20093: f64 = (assign13770_e20091 + 0.01);
        locals.var_pbs_t = assign13770_e20093;
        locals.var_pbs_t_dn4 = assign13770_e20091_d_n4;
        locals.var_pbs_t_rv = 0.0;

        let assign13780_e20097: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20098: f64 = (p.p708 - assign13780_e20097);
        let assign13780_e20100: f64 = (assign13780_e20098 - 0.01);
        let assign13780_e20102: f64 = (-10000.0);
        let assign13780_e20104: f64 = (assign13780_e20102 * 0.001);
        let (assign13780_e20165, assign13780_e20165_d_n4,) = {
    if (!(assign13780_e20100 < assign13780_e20104)) {
        let assign13780_e20111: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20112: f64 = (p.p708 - assign13780_e20111);
        let assign13780_e20114: f64 = (assign13780_e20112 - 0.01);
        let assign13780_e20118: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20119: f64 = (p.p708 - assign13780_e20118);
        let assign13780_e20121: f64 = (assign13780_e20119 - 0.01);
        let assign13780_e20125: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20126: f64 = (p.p708 - assign13780_e20125);
        let assign13780_e20128: f64 = (assign13780_e20126 - 0.01);
        let assign13780_e20129: f64 = (assign13780_e20121 * assign13780_e20128);
        let assign13780_e20132: f64 = (4.0 * 0.001);
        let assign13780_e20134: f64 = (assign13780_e20132 * 0.001);
        let assign13780_e20135: f64 = (assign13780_e20129 + assign13780_e20134);
        let assign13780_e20136: f64 = (assign13780_e20135).sqrt();
        let assign13780_e20137: f64 = (assign13780_e20114 + assign13780_e20136);
        let assign13780_e20138: f64 = (0.5 * assign13780_e20137);
        (assign13780_e20138, (0.5 * ((-(p.p892 * locals.var_deltemp_dn4)) + ((((-(p.p892 * locals.var_deltemp_dn4)) * assign13780_e20128) + (assign13780_e20121 * (-(p.p892 * locals.var_deltemp_dn4)))) / (2.0 * assign13780_e20136)))),)
    } else {
        let assign13780_e20142: f64 = (p.p892 * locals.var_deltemp);
        let assign13780_e20143: f64 = (p.p708 - assign13780_e20142);
        let assign13780_e20145: f64 = (assign13780_e20143 - 0.01);
        let assign13780_e20147: f64 = (-10000.0);
        let assign13780_e20149: f64 = (assign13780_e20147 * 0.001);
        let (assign13780_e20164, assign13780_e20164_d_n4,) = {
            if (assign13780_e20145 < assign13780_e20149) {
                let assign13780_e20152: f64 = (-0.001);
                let assign13780_e20154: f64 = (assign13780_e20152 * 0.001);
                let assign13780_e20158: f64 = (p.p892 * locals.var_deltemp);
                let assign13780_e20159: f64 = (p.p708 - assign13780_e20158);
                let assign13780_e20161: f64 = (assign13780_e20159 - 0.01);
                let assign13780_e20162: f64 = (assign13780_e20154 / assign13780_e20161);
                (assign13780_e20162, (-((assign13780_e20154 * (-(p.p892 * locals.var_deltemp_dn4))) / (assign13780_e20161 * assign13780_e20161))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13780_e20164, assign13780_e20164_d_n4,)
    }
};
        let assign13780_e20167: f64 = (assign13780_e20165 + 0.01);
        locals.var_pbd_t = assign13780_e20167;
        locals.var_pbd_t_dn4 = assign13780_e20165_d_n4;
        locals.var_pbd_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13790_e20171: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20172: f64 = (p.p709 - assign13790_e20171);
        let assign13790_e20174: f64 = (assign13790_e20172 - 0.01);
        let assign13790_e20176: f64 = (-10000.0);
        let assign13790_e20178: f64 = (assign13790_e20176 * 0.001);
        let (assign13790_e20239, assign13790_e20239_d_n4,) = {
    if (!(assign13790_e20174 < assign13790_e20178)) {
        let assign13790_e20185: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20186: f64 = (p.p709 - assign13790_e20185);
        let assign13790_e20188: f64 = (assign13790_e20186 - 0.01);
        let assign13790_e20192: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20193: f64 = (p.p709 - assign13790_e20192);
        let assign13790_e20195: f64 = (assign13790_e20193 - 0.01);
        let assign13790_e20199: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20200: f64 = (p.p709 - assign13790_e20199);
        let assign13790_e20202: f64 = (assign13790_e20200 - 0.01);
        let assign13790_e20203: f64 = (assign13790_e20195 * assign13790_e20202);
        let assign13790_e20206: f64 = (4.0 * 0.001);
        let assign13790_e20208: f64 = (assign13790_e20206 * 0.001);
        let assign13790_e20209: f64 = (assign13790_e20203 + assign13790_e20208);
        let assign13790_e20210: f64 = (assign13790_e20209).sqrt();
        let assign13790_e20211: f64 = (assign13790_e20188 + assign13790_e20210);
        let assign13790_e20212: f64 = (0.5 * assign13790_e20211);
        (assign13790_e20212, (0.5 * ((-(p.p893 * locals.var_deltemp_dn4)) + ((((-(p.p893 * locals.var_deltemp_dn4)) * assign13790_e20202) + (assign13790_e20195 * (-(p.p893 * locals.var_deltemp_dn4)))) / (2.0 * assign13790_e20210)))),)
    } else {
        let assign13790_e20216: f64 = (p.p893 * locals.var_deltemp);
        let assign13790_e20217: f64 = (p.p709 - assign13790_e20216);
        let assign13790_e20219: f64 = (assign13790_e20217 - 0.01);
        let assign13790_e20221: f64 = (-10000.0);
        let assign13790_e20223: f64 = (assign13790_e20221 * 0.001);
        let (assign13790_e20238, assign13790_e20238_d_n4,) = {
            if (assign13790_e20219 < assign13790_e20223) {
                let assign13790_e20226: f64 = (-0.001);
                let assign13790_e20228: f64 = (assign13790_e20226 * 0.001);
                let assign13790_e20232: f64 = (p.p893 * locals.var_deltemp);
                let assign13790_e20233: f64 = (p.p709 - assign13790_e20232);
                let assign13790_e20235: f64 = (assign13790_e20233 - 0.01);
                let assign13790_e20236: f64 = (assign13790_e20228 / assign13790_e20235);
                (assign13790_e20236, (-((assign13790_e20228 * (-(p.p893 * locals.var_deltemp_dn4))) / (assign13790_e20235 * assign13790_e20235))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13790_e20238, assign13790_e20238_d_n4,)
    }
};
        let assign13790_e20241: f64 = (assign13790_e20239 + 0.01);
        locals.var_pbsws_t = assign13790_e20241;
        locals.var_pbsws_t_dn4 = assign13790_e20239_d_n4;
        locals.var_pbsws_t_rv = 0.0;

        let assign13800_e20245: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20246: f64 = (p.p710 - assign13800_e20245);
        let assign13800_e20248: f64 = (assign13800_e20246 - 0.01);
        let assign13800_e20250: f64 = (-10000.0);
        let assign13800_e20252: f64 = (assign13800_e20250 * 0.001);
        let (assign13800_e20313, assign13800_e20313_d_n4,) = {
    if (!(assign13800_e20248 < assign13800_e20252)) {
        let assign13800_e20259: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20260: f64 = (p.p710 - assign13800_e20259);
        let assign13800_e20262: f64 = (assign13800_e20260 - 0.01);
        let assign13800_e20266: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20267: f64 = (p.p710 - assign13800_e20266);
        let assign13800_e20269: f64 = (assign13800_e20267 - 0.01);
        let assign13800_e20273: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20274: f64 = (p.p710 - assign13800_e20273);
        let assign13800_e20276: f64 = (assign13800_e20274 - 0.01);
        let assign13800_e20277: f64 = (assign13800_e20269 * assign13800_e20276);
        let assign13800_e20280: f64 = (4.0 * 0.001);
        let assign13800_e20282: f64 = (assign13800_e20280 * 0.001);
        let assign13800_e20283: f64 = (assign13800_e20277 + assign13800_e20282);
        let assign13800_e20284: f64 = (assign13800_e20283).sqrt();
        let assign13800_e20285: f64 = (assign13800_e20262 + assign13800_e20284);
        let assign13800_e20286: f64 = (0.5 * assign13800_e20285);
        (assign13800_e20286, (0.5 * ((-(p.p893 * locals.var_deltemp_dn4)) + ((((-(p.p893 * locals.var_deltemp_dn4)) * assign13800_e20276) + (assign13800_e20269 * (-(p.p893 * locals.var_deltemp_dn4)))) / (2.0 * assign13800_e20284)))),)
    } else {
        let assign13800_e20290: f64 = (p.p893 * locals.var_deltemp);
        let assign13800_e20291: f64 = (p.p710 - assign13800_e20290);
        let assign13800_e20293: f64 = (assign13800_e20291 - 0.01);
        let assign13800_e20295: f64 = (-10000.0);
        let assign13800_e20297: f64 = (assign13800_e20295 * 0.001);
        let (assign13800_e20312, assign13800_e20312_d_n4,) = {
            if (assign13800_e20293 < assign13800_e20297) {
                let assign13800_e20300: f64 = (-0.001);
                let assign13800_e20302: f64 = (assign13800_e20300 * 0.001);
                let assign13800_e20306: f64 = (p.p893 * locals.var_deltemp);
                let assign13800_e20307: f64 = (p.p710 - assign13800_e20306);
                let assign13800_e20309: f64 = (assign13800_e20307 - 0.01);
                let assign13800_e20310: f64 = (assign13800_e20302 / assign13800_e20309);
                (assign13800_e20310, (-((assign13800_e20302 * (-(p.p893 * locals.var_deltemp_dn4))) / (assign13800_e20309 * assign13800_e20309))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13800_e20312, assign13800_e20312_d_n4,)
    }
};
        let assign13800_e20315: f64 = (assign13800_e20313 + 0.01);
        locals.var_pbswd_t = assign13800_e20315;
        locals.var_pbswd_t_dn4 = assign13800_e20313_d_n4;
        locals.var_pbswd_t_rv = 0.0;

        let assign13810_e20319: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20320: f64 = (p.p711 - assign13810_e20319);
        let assign13810_e20322: f64 = (assign13810_e20320 - 0.01);
        let assign13810_e20324: f64 = (-10000.0);
        let assign13810_e20326: f64 = (assign13810_e20324 * 0.001);
        let (assign13810_e20387, assign13810_e20387_d_n4,) = {
    if (!(assign13810_e20322 < assign13810_e20326)) {
        let assign13810_e20333: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20334: f64 = (p.p711 - assign13810_e20333);
        let assign13810_e20336: f64 = (assign13810_e20334 - 0.01);
        let assign13810_e20340: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20341: f64 = (p.p711 - assign13810_e20340);
        let assign13810_e20343: f64 = (assign13810_e20341 - 0.01);
        let assign13810_e20347: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20348: f64 = (p.p711 - assign13810_e20347);
        let assign13810_e20350: f64 = (assign13810_e20348 - 0.01);
        let assign13810_e20351: f64 = (assign13810_e20343 * assign13810_e20350);
        let assign13810_e20354: f64 = (4.0 * 0.001);
        let assign13810_e20356: f64 = (assign13810_e20354 * 0.001);
        let assign13810_e20357: f64 = (assign13810_e20351 + assign13810_e20356);
        let assign13810_e20358: f64 = (assign13810_e20357).sqrt();
        let assign13810_e20359: f64 = (assign13810_e20336 + assign13810_e20358);
        let assign13810_e20360: f64 = (0.5 * assign13810_e20359);
        (assign13810_e20360, (0.5 * ((-(p.p894 * locals.var_deltemp_dn4)) + ((((-(p.p894 * locals.var_deltemp_dn4)) * assign13810_e20350) + (assign13810_e20343 * (-(p.p894 * locals.var_deltemp_dn4)))) / (2.0 * assign13810_e20358)))),)
    } else {
        let assign13810_e20364: f64 = (p.p894 * locals.var_deltemp);
        let assign13810_e20365: f64 = (p.p711 - assign13810_e20364);
        let assign13810_e20367: f64 = (assign13810_e20365 - 0.01);
        let assign13810_e20369: f64 = (-10000.0);
        let assign13810_e20371: f64 = (assign13810_e20369 * 0.001);
        let (assign13810_e20386, assign13810_e20386_d_n4,) = {
            if (assign13810_e20367 < assign13810_e20371) {
                let assign13810_e20374: f64 = (-0.001);
                let assign13810_e20376: f64 = (assign13810_e20374 * 0.001);
                let assign13810_e20380: f64 = (p.p894 * locals.var_deltemp);
                let assign13810_e20381: f64 = (p.p711 - assign13810_e20380);
                let assign13810_e20383: f64 = (assign13810_e20381 - 0.01);
                let assign13810_e20384: f64 = (assign13810_e20376 / assign13810_e20383);
                (assign13810_e20384, (-((assign13810_e20376 * (-(p.p894 * locals.var_deltemp_dn4))) / (assign13810_e20383 * assign13810_e20383))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13810_e20386, assign13810_e20386_d_n4,)
    }
};
        let assign13810_e20389: f64 = (assign13810_e20387 + 0.01);
        locals.var_pbswgs_t = assign13810_e20389;
        locals.var_pbswgs_t_dn4 = assign13810_e20387_d_n4;
        locals.var_pbswgs_t_rv = 0.0;

        let assign13820_e20393: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20394: f64 = (p.p712 - assign13820_e20393);
        let assign13820_e20396: f64 = (assign13820_e20394 - 0.01);
        let assign13820_e20398: f64 = (-10000.0);
        let assign13820_e20400: f64 = (assign13820_e20398 * 0.001);
        let (assign13820_e20461, assign13820_e20461_d_n4,) = {
    if (!(assign13820_e20396 < assign13820_e20400)) {
        let assign13820_e20407: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20408: f64 = (p.p712 - assign13820_e20407);
        let assign13820_e20410: f64 = (assign13820_e20408 - 0.01);
        let assign13820_e20414: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20415: f64 = (p.p712 - assign13820_e20414);
        let assign13820_e20417: f64 = (assign13820_e20415 - 0.01);
        let assign13820_e20421: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20422: f64 = (p.p712 - assign13820_e20421);
        let assign13820_e20424: f64 = (assign13820_e20422 - 0.01);
        let assign13820_e20425: f64 = (assign13820_e20417 * assign13820_e20424);
        let assign13820_e20428: f64 = (4.0 * 0.001);
        let assign13820_e20430: f64 = (assign13820_e20428 * 0.001);
        let assign13820_e20431: f64 = (assign13820_e20425 + assign13820_e20430);
        let assign13820_e20432: f64 = (assign13820_e20431).sqrt();
        let assign13820_e20433: f64 = (assign13820_e20410 + assign13820_e20432);
        let assign13820_e20434: f64 = (0.5 * assign13820_e20433);
        (assign13820_e20434, (0.5 * ((-(p.p894 * locals.var_deltemp_dn4)) + ((((-(p.p894 * locals.var_deltemp_dn4)) * assign13820_e20424) + (assign13820_e20417 * (-(p.p894 * locals.var_deltemp_dn4)))) / (2.0 * assign13820_e20432)))),)
    } else {
        let assign13820_e20438: f64 = (p.p894 * locals.var_deltemp);
        let assign13820_e20439: f64 = (p.p712 - assign13820_e20438);
        let assign13820_e20441: f64 = (assign13820_e20439 - 0.01);
        let assign13820_e20443: f64 = (-10000.0);
        let assign13820_e20445: f64 = (assign13820_e20443 * 0.001);
        let (assign13820_e20460, assign13820_e20460_d_n4,) = {
            if (assign13820_e20441 < assign13820_e20445) {
                let assign13820_e20448: f64 = (-0.001);
                let assign13820_e20450: f64 = (assign13820_e20448 * 0.001);
                let assign13820_e20454: f64 = (p.p894 * locals.var_deltemp);
                let assign13820_e20455: f64 = (p.p712 - assign13820_e20454);
                let assign13820_e20457: f64 = (assign13820_e20455 - 0.01);
                let assign13820_e20458: f64 = (assign13820_e20450 / assign13820_e20457);
                (assign13820_e20458, (-((assign13820_e20450 * (-(p.p894 * locals.var_deltemp_dn4))) / (assign13820_e20457 * assign13820_e20457))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13820_e20460, assign13820_e20460_d_n4,)
    }
};
        let assign13820_e20463: f64 = (assign13820_e20461 + 0.01);
        locals.var_pbswgd_t = assign13820_e20463;
        locals.var_pbswgd_t_dn4 = assign13820_e20461_d_n4;
        locals.var_pbswgd_t_rv = 0.0;

        let assign13830_e20466: f64 = (locals.var_eg0 / locals.var_vtm0);
        let assign13830_e20469: f64 = (locals.var_eg / locals.var_vtm);
        let assign13830_e20470: f64 = (assign13830_e20466 - assign13830_e20469);
        locals.var_t0 = assign13830_e20470;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = (-(((locals.var_eg_dn4 * locals.var_vtm) - (locals.var_eg * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm)));
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign13840_e20473: f64 = (locals.var_tratio).max(1e-38);
        let assign13840_e20474: f64 = (assign13840_e20473).ln();
        locals.var_t1 = assign13840_e20474;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (if locals.var_tratio >= 1e-38 { locals.var_tratio_dn4 } else { 0.0 } / assign13840_e20473);
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign13850_e20478: f64 = (p.p895 * locals.var_t1);
        let assign13850_e20479: f64 = (locals.var_t0 + assign13850_e20478);
        let assign13850_e20481: f64 = (assign13850_e20479 / p.p725);
        let assign13850_e20482: f64 = { let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_t3 = assign13850_e20482;
        locals.var_t3_dn0 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 + (p.p895 * locals.var_t1_dn0)) / p.p725));
        locals.var_t3_dn2 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 + (p.p895 * locals.var_t1_dn2)) / p.p725));
        locals.var_t3_dn3 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 + (p.p895 * locals.var_t1_dn3)) / p.p725));
        locals.var_t3_dn4 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p895 * locals.var_t1_dn4)) / p.p725));
        locals.var_t3_dn5 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 + (p.p895 * locals.var_t1_dn5)) / p.p725));
        locals.var_t3_dn6 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 + (p.p895 * locals.var_t1_dn6)) / p.p725));
        locals.var_t3_dn7 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 + (p.p895 * locals.var_t1_dn7)) / p.p725));
        locals.var_t3_dn8 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 + (p.p895 * locals.var_t1_dn8)) / p.p725));
        locals.var_t3_dn9 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 + (p.p895 * locals.var_t1_dn9)) / p.p725));
        locals.var_t3_dn10 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 + (p.p895 * locals.var_t1_dn10)) / p.p725));
        locals.var_t3_dn11 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 + (p.p895 * locals.var_t1_dn11)) / p.p725));
        locals.var_t3_dn12 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 + (p.p895 * locals.var_t1_dn12)) / p.p725));
        locals.var_t3_dn13 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 + (p.p895 * locals.var_t1_dn13)) / p.p725));
        locals.var_t3_dn14 = ({ let limited_exp_arg = assign13850_e20481; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 + (p.p895 * locals.var_t1_dn14)) / p.p725));
        locals.var_t3_rv = 0.0;

        let assign13860_e20485: f64 = (p.p719 * locals.var_t3);
        locals.var_jss_t = assign13860_e20485;
        locals.var_jss_t_dn0 = (p.p719 * locals.var_t3_dn0);
        locals.var_jss_t_dn2 = (p.p719 * locals.var_t3_dn2);
        locals.var_jss_t_dn3 = (p.p719 * locals.var_t3_dn3);
        locals.var_jss_t_dn4 = (p.p719 * locals.var_t3_dn4);
        locals.var_jss_t_dn5 = (p.p719 * locals.var_t3_dn5);
        locals.var_jss_t_dn6 = (p.p719 * locals.var_t3_dn6);
        locals.var_jss_t_dn7 = (p.p719 * locals.var_t3_dn7);
        locals.var_jss_t_dn8 = (p.p719 * locals.var_t3_dn8);
        locals.var_jss_t_dn9 = (p.p719 * locals.var_t3_dn9);
        locals.var_jss_t_dn10 = (p.p719 * locals.var_t3_dn10);
        locals.var_jss_t_dn11 = (p.p719 * locals.var_t3_dn11);
        locals.var_jss_t_dn12 = (p.p719 * locals.var_t3_dn12);
        locals.var_jss_t_dn13 = (p.p719 * locals.var_t3_dn13);
        locals.var_jss_t_dn14 = (p.p719 * locals.var_t3_dn14);
        locals.var_jss_t_rv = 0.0;

        let assign13870_e20488: f64 = (p.p721 * locals.var_t3);
        locals.var_jsws_t = assign13870_e20488;
        locals.var_jsws_t_dn0 = (p.p721 * locals.var_t3_dn0);
        locals.var_jsws_t_dn2 = (p.p721 * locals.var_t3_dn2);
        locals.var_jsws_t_dn3 = (p.p721 * locals.var_t3_dn3);
        locals.var_jsws_t_dn4 = (p.p721 * locals.var_t3_dn4);
        locals.var_jsws_t_dn5 = (p.p721 * locals.var_t3_dn5);
        locals.var_jsws_t_dn6 = (p.p721 * locals.var_t3_dn6);
        locals.var_jsws_t_dn7 = (p.p721 * locals.var_t3_dn7);
        locals.var_jsws_t_dn8 = (p.p721 * locals.var_t3_dn8);
        locals.var_jsws_t_dn9 = (p.p721 * locals.var_t3_dn9);
        locals.var_jsws_t_dn10 = (p.p721 * locals.var_t3_dn10);
        locals.var_jsws_t_dn11 = (p.p721 * locals.var_t3_dn11);
        locals.var_jsws_t_dn12 = (p.p721 * locals.var_t3_dn12);
        locals.var_jsws_t_dn13 = (p.p721 * locals.var_t3_dn13);
        locals.var_jsws_t_dn14 = (p.p721 * locals.var_t3_dn14);
        locals.var_jsws_t_rv = 0.0;

        let assign13880_e20491: f64 = (p.p723 * locals.var_t3);
        locals.var_jswgs_t = assign13880_e20491;
        locals.var_jswgs_t_dn0 = (p.p723 * locals.var_t3_dn0);
        locals.var_jswgs_t_dn2 = (p.p723 * locals.var_t3_dn2);
        locals.var_jswgs_t_dn3 = (p.p723 * locals.var_t3_dn3);
        locals.var_jswgs_t_dn4 = (p.p723 * locals.var_t3_dn4);
        locals.var_jswgs_t_dn5 = (p.p723 * locals.var_t3_dn5);
        locals.var_jswgs_t_dn6 = (p.p723 * locals.var_t3_dn6);
        locals.var_jswgs_t_dn7 = (p.p723 * locals.var_t3_dn7);
        locals.var_jswgs_t_dn8 = (p.p723 * locals.var_t3_dn8);
        locals.var_jswgs_t_dn9 = (p.p723 * locals.var_t3_dn9);
        locals.var_jswgs_t_dn10 = (p.p723 * locals.var_t3_dn10);
        locals.var_jswgs_t_dn11 = (p.p723 * locals.var_t3_dn11);
        locals.var_jswgs_t_dn12 = (p.p723 * locals.var_t3_dn12);
        locals.var_jswgs_t_dn13 = (p.p723 * locals.var_t3_dn13);
        locals.var_jswgs_t_dn14 = (p.p723 * locals.var_t3_dn14);
        locals.var_jswgs_t_rv = 0.0;

        let assign13890_e20495: f64 = (p.p896 * locals.var_t1);
        let assign13890_e20496: f64 = (locals.var_t0 + assign13890_e20495);
        let assign13890_e20498: f64 = (assign13890_e20496 / p.p726);
        let assign13890_e20499: f64 = { let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_t3 = assign13890_e20499;
        locals.var_t3_dn0 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 + (p.p896 * locals.var_t1_dn0)) / p.p726));
        locals.var_t3_dn2 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 + (p.p896 * locals.var_t1_dn2)) / p.p726));
        locals.var_t3_dn3 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 + (p.p896 * locals.var_t1_dn3)) / p.p726));
        locals.var_t3_dn4 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 + (p.p896 * locals.var_t1_dn4)) / p.p726));
        locals.var_t3_dn5 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 + (p.p896 * locals.var_t1_dn5)) / p.p726));
        locals.var_t3_dn6 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 + (p.p896 * locals.var_t1_dn6)) / p.p726));
        locals.var_t3_dn7 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 + (p.p896 * locals.var_t1_dn7)) / p.p726));
        locals.var_t3_dn8 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 + (p.p896 * locals.var_t1_dn8)) / p.p726));
        locals.var_t3_dn9 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 + (p.p896 * locals.var_t1_dn9)) / p.p726));
        locals.var_t3_dn10 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 + (p.p896 * locals.var_t1_dn10)) / p.p726));
        locals.var_t3_dn11 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 + (p.p896 * locals.var_t1_dn11)) / p.p726));
        locals.var_t3_dn12 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 + (p.p896 * locals.var_t1_dn12)) / p.p726));
        locals.var_t3_dn13 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 + (p.p896 * locals.var_t1_dn13)) / p.p726));
        locals.var_t3_dn14 = ({ let limited_exp_arg = assign13890_e20498; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 + (p.p896 * locals.var_t1_dn14)) / p.p726));
        locals.var_t3_rv = 0.0;

        let assign13900_e20502: f64 = (p.p720 * locals.var_t3);
        locals.var_jsd_t = assign13900_e20502;
        locals.var_jsd_t_dn0 = (p.p720 * locals.var_t3_dn0);
        locals.var_jsd_t_dn2 = (p.p720 * locals.var_t3_dn2);
        locals.var_jsd_t_dn3 = (p.p720 * locals.var_t3_dn3);
        locals.var_jsd_t_dn4 = (p.p720 * locals.var_t3_dn4);
        locals.var_jsd_t_dn5 = (p.p720 * locals.var_t3_dn5);
        locals.var_jsd_t_dn6 = (p.p720 * locals.var_t3_dn6);
        locals.var_jsd_t_dn7 = (p.p720 * locals.var_t3_dn7);
        locals.var_jsd_t_dn8 = (p.p720 * locals.var_t3_dn8);
        locals.var_jsd_t_dn9 = (p.p720 * locals.var_t3_dn9);
        locals.var_jsd_t_dn10 = (p.p720 * locals.var_t3_dn10);
        locals.var_jsd_t_dn11 = (p.p720 * locals.var_t3_dn11);
        locals.var_jsd_t_dn12 = (p.p720 * locals.var_t3_dn12);
        locals.var_jsd_t_dn13 = (p.p720 * locals.var_t3_dn13);
        locals.var_jsd_t_dn14 = (p.p720 * locals.var_t3_dn14);
        locals.var_jsd_t_rv = 0.0;

        let assign13910_e20505: f64 = (p.p722 * locals.var_t3);
        locals.var_jswd_t = assign13910_e20505;
        locals.var_jswd_t_dn0 = (p.p722 * locals.var_t3_dn0);
        locals.var_jswd_t_dn2 = (p.p722 * locals.var_t3_dn2);
        locals.var_jswd_t_dn3 = (p.p722 * locals.var_t3_dn3);
        locals.var_jswd_t_dn4 = (p.p722 * locals.var_t3_dn4);
        locals.var_jswd_t_dn5 = (p.p722 * locals.var_t3_dn5);
        locals.var_jswd_t_dn6 = (p.p722 * locals.var_t3_dn6);
        locals.var_jswd_t_dn7 = (p.p722 * locals.var_t3_dn7);
        locals.var_jswd_t_dn8 = (p.p722 * locals.var_t3_dn8);
        locals.var_jswd_t_dn9 = (p.p722 * locals.var_t3_dn9);
        locals.var_jswd_t_dn10 = (p.p722 * locals.var_t3_dn10);
        locals.var_jswd_t_dn11 = (p.p722 * locals.var_t3_dn11);
        locals.var_jswd_t_dn12 = (p.p722 * locals.var_t3_dn12);
        locals.var_jswd_t_dn13 = (p.p722 * locals.var_t3_dn13);
        locals.var_jswd_t_dn14 = (p.p722 * locals.var_t3_dn14);
        locals.var_jswd_t_rv = 0.0;

        let assign13920_e20508: f64 = (p.p724 * locals.var_t3);
        locals.var_jswgd_t = assign13920_e20508;
        locals.var_jswgd_t_dn0 = (p.p724 * locals.var_t3_dn0);
        locals.var_jswgd_t_dn2 = (p.p724 * locals.var_t3_dn2);
        locals.var_jswgd_t_dn3 = (p.p724 * locals.var_t3_dn3);
        locals.var_jswgd_t_dn4 = (p.p724 * locals.var_t3_dn4);
        locals.var_jswgd_t_dn5 = (p.p724 * locals.var_t3_dn5);
        locals.var_jswgd_t_dn6 = (p.p724 * locals.var_t3_dn6);
        locals.var_jswgd_t_dn7 = (p.p724 * locals.var_t3_dn7);
        locals.var_jswgd_t_dn8 = (p.p724 * locals.var_t3_dn8);
        locals.var_jswgd_t_dn9 = (p.p724 * locals.var_t3_dn9);
        locals.var_jswgd_t_dn10 = (p.p724 * locals.var_t3_dn10);
        locals.var_jswgd_t_dn11 = (p.p724 * locals.var_t3_dn11);
        locals.var_jswgd_t_dn12 = (p.p724 * locals.var_t3_dn12);
        locals.var_jswgd_t_dn13 = (p.p724 * locals.var_t3_dn13);
        locals.var_jswgd_t_dn14 = (p.p724 * locals.var_t3_dn14);
        locals.var_jswgd_t_rv = 0.0;

        let assign13930_e20512: f64 = (locals.var_eg0 * p.p897);
        let assign13930_e20515: f64 = (locals.var_tratio - 1.0);
        let assign13930_e20516: f64 = (assign13930_e20512 * assign13930_e20515);
        let assign13930_e20518: f64 = (assign13930_e20516 / locals.var_vtm);
        let assign13930_e20519: f64 = { let limited_exp_arg = assign13930_e20518; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13930_e20520: f64 = (p.p735 * assign13930_e20519);
        locals.var_jtss_t = assign13930_e20520;
        locals.var_jtss_t_dn4 = (p.p735 * ({ let limited_exp_arg = assign13930_e20518; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13930_e20512 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13930_e20516 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtss_t_rv = 0.0;

        let assign13940_e20524: f64 = (locals.var_eg0 * p.p899);
        let assign13940_e20527: f64 = (locals.var_tratio - 1.0);
        let assign13940_e20528: f64 = (assign13940_e20524 * assign13940_e20527);
        let assign13940_e20530: f64 = (assign13940_e20528 / locals.var_vtm);
        let assign13940_e20531: f64 = { let limited_exp_arg = assign13940_e20530; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13940_e20532: f64 = (p.p737 * assign13940_e20531);
        locals.var_jtssws_t = assign13940_e20532;
        locals.var_jtssws_t_dn4 = (p.p737 * ({ let limited_exp_arg = assign13940_e20530; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13940_e20524 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13940_e20528 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtssws_t_rv = 0.0;

        let assign13950_e20536: f64 = (p.p741 / locals.var_weffcj);
        let assign13950_e20537: f64 = (assign13950_e20536).sqrt();
        let assign13950_e20539: f64 = (assign13950_e20537 + 1.0);
        let assign13950_e20540: f64 = (p.p739 * assign13950_e20539);
        let assign13950_e20543: f64 = (locals.var_eg0 * p.p901);
        let assign13950_e20546: f64 = (locals.var_tratio - 1.0);
        let assign13950_e20547: f64 = (assign13950_e20543 * assign13950_e20546);
        let assign13950_e20549: f64 = (assign13950_e20547 / locals.var_vtm);
        let assign13950_e20550: f64 = { let limited_exp_arg = assign13950_e20549; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13950_e20551: f64 = (assign13950_e20540 * assign13950_e20550);
        locals.var_jtsswgs_t = assign13950_e20551;
        locals.var_jtsswgs_t_dn4 = (assign13950_e20540 * ({ let limited_exp_arg = assign13950_e20549; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13950_e20543 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13950_e20547 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtsswgs_t_rv = 0.0;

        let assign13960_e20555: f64 = (locals.var_eg0 * p.p898);
        let assign13960_e20558: f64 = (locals.var_tratio - 1.0);
        let assign13960_e20559: f64 = (assign13960_e20555 * assign13960_e20558);
        let assign13960_e20561: f64 = (assign13960_e20559 / locals.var_vtm);
        let assign13960_e20562: f64 = { let limited_exp_arg = assign13960_e20561; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13960_e20563: f64 = (p.p736 * assign13960_e20562);
        locals.var_jtsd_t = assign13960_e20563;
        locals.var_jtsd_t_dn4 = (p.p736 * ({ let limited_exp_arg = assign13960_e20561; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13960_e20555 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13960_e20559 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtsd_t_rv = 0.0;

        let assign13970_e20567: f64 = (locals.var_eg0 * p.p900);
        let assign13970_e20570: f64 = (locals.var_tratio - 1.0);
        let assign13970_e20571: f64 = (assign13970_e20567 * assign13970_e20570);
        let assign13970_e20573: f64 = (assign13970_e20571 / locals.var_vtm);
        let assign13970_e20574: f64 = { let limited_exp_arg = assign13970_e20573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13970_e20575: f64 = (p.p738 * assign13970_e20574);
        locals.var_jtsswd_t = assign13970_e20575;
        locals.var_jtsswd_t_dn4 = (p.p738 * ({ let limited_exp_arg = assign13970_e20573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13970_e20567 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13970_e20571 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtsswd_t_rv = 0.0;

        let assign13980_e20579: f64 = (p.p741 / locals.var_weffcj);
        let assign13980_e20580: f64 = (assign13980_e20579).sqrt();
        let assign13980_e20582: f64 = (assign13980_e20580 + 1.0);
        let assign13980_e20583: f64 = (p.p740 * assign13980_e20582);
        let assign13980_e20586: f64 = (locals.var_eg0 * p.p902);
        let assign13980_e20589: f64 = (locals.var_tratio - 1.0);
        let assign13980_e20590: f64 = (assign13980_e20586 * assign13980_e20589);
        let assign13980_e20592: f64 = (assign13980_e20590 / locals.var_vtm);
        let assign13980_e20593: f64 = { let limited_exp_arg = assign13980_e20592; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13980_e20594: f64 = (assign13980_e20583 * assign13980_e20593);
        locals.var_jtsswgd_t = assign13980_e20594;
        locals.var_jtsswgd_t_dn4 = (assign13980_e20583 * ({ let limited_exp_arg = assign13980_e20592; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign13980_e20586 * locals.var_tratio_dn4) * locals.var_vtm) - (assign13980_e20590 * locals.var_vtm_dn4)) / (locals.var_vtm * locals.var_vtm))));
        locals.var_jtsswgd_t_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign13990_e20600: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20601: f64 = (p.p903 * assign13990_e20600);
        let assign13990_e20602: f64 = (1.0 + assign13990_e20601);
        let assign13990_e20603: f64 = (p.p742 * assign13990_e20602);
        let assign13990_e20605: f64 = (assign13990_e20603 - 0.01);
        let assign13990_e20607: f64 = (-10000.0);
        let assign13990_e20609: f64 = (assign13990_e20607 * 0.001);
        let (assign13990_e20690, assign13990_e20690_d_n4,) = {
    if (!(assign13990_e20605 < assign13990_e20609)) {
        let assign13990_e20618: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20619: f64 = (p.p903 * assign13990_e20618);
        let assign13990_e20620: f64 = (1.0 + assign13990_e20619);
        let assign13990_e20621: f64 = (p.p742 * assign13990_e20620);
        let assign13990_e20623: f64 = (assign13990_e20621 - 0.01);
        let assign13990_e20629: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20630: f64 = (p.p903 * assign13990_e20629);
        let assign13990_e20631: f64 = (1.0 + assign13990_e20630);
        let assign13990_e20632: f64 = (p.p742 * assign13990_e20631);
        let assign13990_e20634: f64 = (assign13990_e20632 - 0.01);
        let assign13990_e20640: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20641: f64 = (p.p903 * assign13990_e20640);
        let assign13990_e20642: f64 = (1.0 + assign13990_e20641);
        let assign13990_e20643: f64 = (p.p742 * assign13990_e20642);
        let assign13990_e20645: f64 = (assign13990_e20643 - 0.01);
        let assign13990_e20646: f64 = (assign13990_e20634 * assign13990_e20645);
        let assign13990_e20649: f64 = (4.0 * 0.001);
        let assign13990_e20651: f64 = (assign13990_e20649 * 0.001);
        let assign13990_e20652: f64 = (assign13990_e20646 + assign13990_e20651);
        let assign13990_e20653: f64 = (assign13990_e20652).sqrt();
        let assign13990_e20654: f64 = (assign13990_e20623 + assign13990_e20653);
        let assign13990_e20655: f64 = (0.5 * assign13990_e20654);
        (assign13990_e20655, (0.5 * ((p.p742 * (p.p903 * locals.var_tratio_dn4)) + ((((p.p742 * (p.p903 * locals.var_tratio_dn4)) * assign13990_e20645) + (assign13990_e20634 * (p.p742 * (p.p903 * locals.var_tratio_dn4)))) / (2.0 * assign13990_e20653)))),)
    } else {
        let assign13990_e20661: f64 = (locals.var_tratio - 1.0);
        let assign13990_e20662: f64 = (p.p903 * assign13990_e20661);
        let assign13990_e20663: f64 = (1.0 + assign13990_e20662);
        let assign13990_e20664: f64 = (p.p742 * assign13990_e20663);
        let assign13990_e20666: f64 = (assign13990_e20664 - 0.01);
        let assign13990_e20668: f64 = (-10000.0);
        let assign13990_e20670: f64 = (assign13990_e20668 * 0.001);
        let (assign13990_e20689, assign13990_e20689_d_n4,) = {
            if (assign13990_e20666 < assign13990_e20670) {
                let assign13990_e20673: f64 = (-0.001);
                let assign13990_e20675: f64 = (assign13990_e20673 * 0.001);
                let assign13990_e20681: f64 = (locals.var_tratio - 1.0);
                let assign13990_e20682: f64 = (p.p903 * assign13990_e20681);
                let assign13990_e20683: f64 = (1.0 + assign13990_e20682);
                let assign13990_e20684: f64 = (p.p742 * assign13990_e20683);
                let assign13990_e20686: f64 = (assign13990_e20684 - 0.01);
                let assign13990_e20687: f64 = (assign13990_e20675 / assign13990_e20686);
                (assign13990_e20687, (-((assign13990_e20675 * (p.p742 * (p.p903 * locals.var_tratio_dn4))) / (assign13990_e20686 * assign13990_e20686))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign13990_e20689, assign13990_e20689_d_n4,)
    }
};
        let assign13990_e20692: f64 = (assign13990_e20690 + 0.01);
        locals.var_njts_t = assign13990_e20692;
        locals.var_njts_t_dn4 = assign13990_e20690_d_n4;
        locals.var_njts_t_rv = 0.0;

        let assign14000_e20698: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20699: f64 = (p.p905 * assign14000_e20698);
        let assign14000_e20700: f64 = (1.0 + assign14000_e20699);
        let assign14000_e20701: f64 = (p.p744 * assign14000_e20700);
        let assign14000_e20703: f64 = (assign14000_e20701 - 0.01);
        let assign14000_e20705: f64 = (-10000.0);
        let assign14000_e20707: f64 = (assign14000_e20705 * 0.001);
        let (assign14000_e20788, assign14000_e20788_d_n4,) = {
    if (!(assign14000_e20703 < assign14000_e20707)) {
        let assign14000_e20716: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20717: f64 = (p.p905 * assign14000_e20716);
        let assign14000_e20718: f64 = (1.0 + assign14000_e20717);
        let assign14000_e20719: f64 = (p.p744 * assign14000_e20718);
        let assign14000_e20721: f64 = (assign14000_e20719 - 0.01);
        let assign14000_e20727: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20728: f64 = (p.p905 * assign14000_e20727);
        let assign14000_e20729: f64 = (1.0 + assign14000_e20728);
        let assign14000_e20730: f64 = (p.p744 * assign14000_e20729);
        let assign14000_e20732: f64 = (assign14000_e20730 - 0.01);
        let assign14000_e20738: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20739: f64 = (p.p905 * assign14000_e20738);
        let assign14000_e20740: f64 = (1.0 + assign14000_e20739);
        let assign14000_e20741: f64 = (p.p744 * assign14000_e20740);
        let assign14000_e20743: f64 = (assign14000_e20741 - 0.01);
        let assign14000_e20744: f64 = (assign14000_e20732 * assign14000_e20743);
        let assign14000_e20747: f64 = (4.0 * 0.001);
        let assign14000_e20749: f64 = (assign14000_e20747 * 0.001);
        let assign14000_e20750: f64 = (assign14000_e20744 + assign14000_e20749);
        let assign14000_e20751: f64 = (assign14000_e20750).sqrt();
        let assign14000_e20752: f64 = (assign14000_e20721 + assign14000_e20751);
        let assign14000_e20753: f64 = (0.5 * assign14000_e20752);
        (assign14000_e20753, (0.5 * ((p.p744 * (p.p905 * locals.var_tratio_dn4)) + ((((p.p744 * (p.p905 * locals.var_tratio_dn4)) * assign14000_e20743) + (assign14000_e20732 * (p.p744 * (p.p905 * locals.var_tratio_dn4)))) / (2.0 * assign14000_e20751)))),)
    } else {
        let assign14000_e20759: f64 = (locals.var_tratio - 1.0);
        let assign14000_e20760: f64 = (p.p905 * assign14000_e20759);
        let assign14000_e20761: f64 = (1.0 + assign14000_e20760);
        let assign14000_e20762: f64 = (p.p744 * assign14000_e20761);
        let assign14000_e20764: f64 = (assign14000_e20762 - 0.01);
        let assign14000_e20766: f64 = (-10000.0);
        let assign14000_e20768: f64 = (assign14000_e20766 * 0.001);
        let (assign14000_e20787, assign14000_e20787_d_n4,) = {
            if (assign14000_e20764 < assign14000_e20768) {
                let assign14000_e20771: f64 = (-0.001);
                let assign14000_e20773: f64 = (assign14000_e20771 * 0.001);
                let assign14000_e20779: f64 = (locals.var_tratio - 1.0);
                let assign14000_e20780: f64 = (p.p905 * assign14000_e20779);
                let assign14000_e20781: f64 = (1.0 + assign14000_e20780);
                let assign14000_e20782: f64 = (p.p744 * assign14000_e20781);
                let assign14000_e20784: f64 = (assign14000_e20782 - 0.01);
                let assign14000_e20785: f64 = (assign14000_e20773 / assign14000_e20784);
                (assign14000_e20785, (-((assign14000_e20773 * (p.p744 * (p.p905 * locals.var_tratio_dn4))) / (assign14000_e20784 * assign14000_e20784))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14000_e20787, assign14000_e20787_d_n4,)
    }
};
        let assign14000_e20790: f64 = (assign14000_e20788 + 0.01);
        locals.var_njtssw_t = assign14000_e20790;
        locals.var_njtssw_t_dn4 = assign14000_e20788_d_n4;
        locals.var_njtssw_t_rv = 0.0;

        let assign14010_e20796: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20797: f64 = (p.p907 * assign14010_e20796);
        let assign14010_e20798: f64 = (1.0 + assign14010_e20797);
        let assign14010_e20799: f64 = (p.p746 * assign14010_e20798);
        let assign14010_e20801: f64 = (assign14010_e20799 - 0.01);
        let assign14010_e20803: f64 = (-10000.0);
        let assign14010_e20805: f64 = (assign14010_e20803 * 0.001);
        let (assign14010_e20886, assign14010_e20886_d_n4,) = {
    if (!(assign14010_e20801 < assign14010_e20805)) {
        let assign14010_e20814: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20815: f64 = (p.p907 * assign14010_e20814);
        let assign14010_e20816: f64 = (1.0 + assign14010_e20815);
        let assign14010_e20817: f64 = (p.p746 * assign14010_e20816);
        let assign14010_e20819: f64 = (assign14010_e20817 - 0.01);
        let assign14010_e20825: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20826: f64 = (p.p907 * assign14010_e20825);
        let assign14010_e20827: f64 = (1.0 + assign14010_e20826);
        let assign14010_e20828: f64 = (p.p746 * assign14010_e20827);
        let assign14010_e20830: f64 = (assign14010_e20828 - 0.01);
        let assign14010_e20836: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20837: f64 = (p.p907 * assign14010_e20836);
        let assign14010_e20838: f64 = (1.0 + assign14010_e20837);
        let assign14010_e20839: f64 = (p.p746 * assign14010_e20838);
        let assign14010_e20841: f64 = (assign14010_e20839 - 0.01);
        let assign14010_e20842: f64 = (assign14010_e20830 * assign14010_e20841);
        let assign14010_e20845: f64 = (4.0 * 0.001);
        let assign14010_e20847: f64 = (assign14010_e20845 * 0.001);
        let assign14010_e20848: f64 = (assign14010_e20842 + assign14010_e20847);
        let assign14010_e20849: f64 = (assign14010_e20848).sqrt();
        let assign14010_e20850: f64 = (assign14010_e20819 + assign14010_e20849);
        let assign14010_e20851: f64 = (0.5 * assign14010_e20850);
        (assign14010_e20851, (0.5 * ((p.p746 * (p.p907 * locals.var_tratio_dn4)) + ((((p.p746 * (p.p907 * locals.var_tratio_dn4)) * assign14010_e20841) + (assign14010_e20830 * (p.p746 * (p.p907 * locals.var_tratio_dn4)))) / (2.0 * assign14010_e20849)))),)
    } else {
        let assign14010_e20857: f64 = (locals.var_tratio - 1.0);
        let assign14010_e20858: f64 = (p.p907 * assign14010_e20857);
        let assign14010_e20859: f64 = (1.0 + assign14010_e20858);
        let assign14010_e20860: f64 = (p.p746 * assign14010_e20859);
        let assign14010_e20862: f64 = (assign14010_e20860 - 0.01);
        let assign14010_e20864: f64 = (-10000.0);
        let assign14010_e20866: f64 = (assign14010_e20864 * 0.001);
        let (assign14010_e20885, assign14010_e20885_d_n4,) = {
            if (assign14010_e20862 < assign14010_e20866) {
                let assign14010_e20869: f64 = (-0.001);
                let assign14010_e20871: f64 = (assign14010_e20869 * 0.001);
                let assign14010_e20877: f64 = (locals.var_tratio - 1.0);
                let assign14010_e20878: f64 = (p.p907 * assign14010_e20877);
                let assign14010_e20879: f64 = (1.0 + assign14010_e20878);
                let assign14010_e20880: f64 = (p.p746 * assign14010_e20879);
                let assign14010_e20882: f64 = (assign14010_e20880 - 0.01);
                let assign14010_e20883: f64 = (assign14010_e20871 / assign14010_e20882);
                (assign14010_e20883, (-((assign14010_e20871 * (p.p746 * (p.p907 * locals.var_tratio_dn4))) / (assign14010_e20882 * assign14010_e20882))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14010_e20885, assign14010_e20885_d_n4,)
    }
};
        let assign14010_e20888: f64 = (assign14010_e20886 + 0.01);
        locals.var_njtsswg_t = assign14010_e20888;
        locals.var_njtsswg_t_dn4 = assign14010_e20886_d_n4;
        locals.var_njtsswg_t_rv = 0.0;

        let assign14020_e20894: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20895: f64 = (p.p904 * assign14020_e20894);
        let assign14020_e20896: f64 = (1.0 + assign14020_e20895);
        let assign14020_e20897: f64 = (p.p743 * assign14020_e20896);
        let assign14020_e20899: f64 = (assign14020_e20897 - 0.01);
        let assign14020_e20901: f64 = (-10000.0);
        let assign14020_e20903: f64 = (assign14020_e20901 * 0.001);
        let (assign14020_e20984, assign14020_e20984_d_n4,) = {
    if (!(assign14020_e20899 < assign14020_e20903)) {
        let assign14020_e20912: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20913: f64 = (p.p904 * assign14020_e20912);
        let assign14020_e20914: f64 = (1.0 + assign14020_e20913);
        let assign14020_e20915: f64 = (p.p743 * assign14020_e20914);
        let assign14020_e20917: f64 = (assign14020_e20915 - 0.01);
        let assign14020_e20923: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20924: f64 = (p.p904 * assign14020_e20923);
        let assign14020_e20925: f64 = (1.0 + assign14020_e20924);
        let assign14020_e20926: f64 = (p.p743 * assign14020_e20925);
        let assign14020_e20928: f64 = (assign14020_e20926 - 0.01);
        let assign14020_e20934: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20935: f64 = (p.p904 * assign14020_e20934);
        let assign14020_e20936: f64 = (1.0 + assign14020_e20935);
        let assign14020_e20937: f64 = (p.p743 * assign14020_e20936);
        let assign14020_e20939: f64 = (assign14020_e20937 - 0.01);
        let assign14020_e20940: f64 = (assign14020_e20928 * assign14020_e20939);
        let assign14020_e20943: f64 = (4.0 * 0.001);
        let assign14020_e20945: f64 = (assign14020_e20943 * 0.001);
        let assign14020_e20946: f64 = (assign14020_e20940 + assign14020_e20945);
        let assign14020_e20947: f64 = (assign14020_e20946).sqrt();
        let assign14020_e20948: f64 = (assign14020_e20917 + assign14020_e20947);
        let assign14020_e20949: f64 = (0.5 * assign14020_e20948);
        (assign14020_e20949, (0.5 * ((p.p743 * (p.p904 * locals.var_tratio_dn4)) + ((((p.p743 * (p.p904 * locals.var_tratio_dn4)) * assign14020_e20939) + (assign14020_e20928 * (p.p743 * (p.p904 * locals.var_tratio_dn4)))) / (2.0 * assign14020_e20947)))),)
    } else {
        let assign14020_e20955: f64 = (locals.var_tratio - 1.0);
        let assign14020_e20956: f64 = (p.p904 * assign14020_e20955);
        let assign14020_e20957: f64 = (1.0 + assign14020_e20956);
        let assign14020_e20958: f64 = (p.p743 * assign14020_e20957);
        let assign14020_e20960: f64 = (assign14020_e20958 - 0.01);
        let assign14020_e20962: f64 = (-10000.0);
        let assign14020_e20964: f64 = (assign14020_e20962 * 0.001);
        let (assign14020_e20983, assign14020_e20983_d_n4,) = {
            if (assign14020_e20960 < assign14020_e20964) {
                let assign14020_e20967: f64 = (-0.001);
                let assign14020_e20969: f64 = (assign14020_e20967 * 0.001);
                let assign14020_e20975: f64 = (locals.var_tratio - 1.0);
                let assign14020_e20976: f64 = (p.p904 * assign14020_e20975);
                let assign14020_e20977: f64 = (1.0 + assign14020_e20976);
                let assign14020_e20978: f64 = (p.p743 * assign14020_e20977);
                let assign14020_e20980: f64 = (assign14020_e20978 - 0.01);
                let assign14020_e20981: f64 = (assign14020_e20969 / assign14020_e20980);
                (assign14020_e20981, (-((assign14020_e20969 * (p.p743 * (p.p904 * locals.var_tratio_dn4))) / (assign14020_e20980 * assign14020_e20980))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14020_e20983, assign14020_e20983_d_n4,)
    }
};
        let assign14020_e20986: f64 = (assign14020_e20984 + 0.01);
        locals.var_njtsd_t = assign14020_e20986;
        locals.var_njtsd_t_dn4 = assign14020_e20984_d_n4;
        locals.var_njtsd_t_rv = 0.0;

        let assign14030_e20992: f64 = (locals.var_tratio - 1.0);
        let assign14030_e20993: f64 = (p.p906 * assign14030_e20992);
        let assign14030_e20994: f64 = (1.0 + assign14030_e20993);
        let assign14030_e20995: f64 = (p.p745 * assign14030_e20994);
        let assign14030_e20997: f64 = (assign14030_e20995 - 0.01);
        let assign14030_e20999: f64 = (-10000.0);
        let assign14030_e21001: f64 = (assign14030_e20999 * 0.001);
        let (assign14030_e21082, assign14030_e21082_d_n4,) = {
    if (!(assign14030_e20997 < assign14030_e21001)) {
        let assign14030_e21010: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21011: f64 = (p.p906 * assign14030_e21010);
        let assign14030_e21012: f64 = (1.0 + assign14030_e21011);
        let assign14030_e21013: f64 = (p.p745 * assign14030_e21012);
        let assign14030_e21015: f64 = (assign14030_e21013 - 0.01);
        let assign14030_e21021: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21022: f64 = (p.p906 * assign14030_e21021);
        let assign14030_e21023: f64 = (1.0 + assign14030_e21022);
        let assign14030_e21024: f64 = (p.p745 * assign14030_e21023);
        let assign14030_e21026: f64 = (assign14030_e21024 - 0.01);
        let assign14030_e21032: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21033: f64 = (p.p906 * assign14030_e21032);
        let assign14030_e21034: f64 = (1.0 + assign14030_e21033);
        let assign14030_e21035: f64 = (p.p745 * assign14030_e21034);
        let assign14030_e21037: f64 = (assign14030_e21035 - 0.01);
        let assign14030_e21038: f64 = (assign14030_e21026 * assign14030_e21037);
        let assign14030_e21041: f64 = (4.0 * 0.001);
        let assign14030_e21043: f64 = (assign14030_e21041 * 0.001);
        let assign14030_e21044: f64 = (assign14030_e21038 + assign14030_e21043);
        let assign14030_e21045: f64 = (assign14030_e21044).sqrt();
        let assign14030_e21046: f64 = (assign14030_e21015 + assign14030_e21045);
        let assign14030_e21047: f64 = (0.5 * assign14030_e21046);
        (assign14030_e21047, (0.5 * ((p.p745 * (p.p906 * locals.var_tratio_dn4)) + ((((p.p745 * (p.p906 * locals.var_tratio_dn4)) * assign14030_e21037) + (assign14030_e21026 * (p.p745 * (p.p906 * locals.var_tratio_dn4)))) / (2.0 * assign14030_e21045)))),)
    } else {
        let assign14030_e21053: f64 = (locals.var_tratio - 1.0);
        let assign14030_e21054: f64 = (p.p906 * assign14030_e21053);
        let assign14030_e21055: f64 = (1.0 + assign14030_e21054);
        let assign14030_e21056: f64 = (p.p745 * assign14030_e21055);
        let assign14030_e21058: f64 = (assign14030_e21056 - 0.01);
        let assign14030_e21060: f64 = (-10000.0);
        let assign14030_e21062: f64 = (assign14030_e21060 * 0.001);
        let (assign14030_e21081, assign14030_e21081_d_n4,) = {
            if (assign14030_e21058 < assign14030_e21062) {
                let assign14030_e21065: f64 = (-0.001);
                let assign14030_e21067: f64 = (assign14030_e21065 * 0.001);
                let assign14030_e21073: f64 = (locals.var_tratio - 1.0);
                let assign14030_e21074: f64 = (p.p906 * assign14030_e21073);
                let assign14030_e21075: f64 = (1.0 + assign14030_e21074);
                let assign14030_e21076: f64 = (p.p745 * assign14030_e21075);
                let assign14030_e21078: f64 = (assign14030_e21076 - 0.01);
                let assign14030_e21079: f64 = (assign14030_e21067 / assign14030_e21078);
                (assign14030_e21079, (-((assign14030_e21067 * (p.p745 * (p.p906 * locals.var_tratio_dn4))) / (assign14030_e21078 * assign14030_e21078))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14030_e21081, assign14030_e21081_d_n4,)
    }
};
        let assign14030_e21084: f64 = (assign14030_e21082 + 0.01);
        locals.var_njtsswd_t = assign14030_e21084;
        locals.var_njtsswd_t_dn4 = assign14030_e21082_d_n4;
        locals.var_njtsswd_t_rv = 0.0;

        let assign14040_e21090: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21091: f64 = (p.p908 * assign14040_e21090);
        let assign14040_e21092: f64 = (1.0 + assign14040_e21091);
        let assign14040_e21093: f64 = (p.p747 * assign14040_e21092);
        let assign14040_e21095: f64 = (assign14040_e21093 - 0.01);
        let assign14040_e21097: f64 = (-10000.0);
        let assign14040_e21099: f64 = (assign14040_e21097 * 0.001);
        let (assign14040_e21180, assign14040_e21180_d_n4,) = {
    if (!(assign14040_e21095 < assign14040_e21099)) {
        let assign14040_e21108: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21109: f64 = (p.p908 * assign14040_e21108);
        let assign14040_e21110: f64 = (1.0 + assign14040_e21109);
        let assign14040_e21111: f64 = (p.p747 * assign14040_e21110);
        let assign14040_e21113: f64 = (assign14040_e21111 - 0.01);
        let assign14040_e21119: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21120: f64 = (p.p908 * assign14040_e21119);
        let assign14040_e21121: f64 = (1.0 + assign14040_e21120);
        let assign14040_e21122: f64 = (p.p747 * assign14040_e21121);
        let assign14040_e21124: f64 = (assign14040_e21122 - 0.01);
        let assign14040_e21130: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21131: f64 = (p.p908 * assign14040_e21130);
        let assign14040_e21132: f64 = (1.0 + assign14040_e21131);
        let assign14040_e21133: f64 = (p.p747 * assign14040_e21132);
        let assign14040_e21135: f64 = (assign14040_e21133 - 0.01);
        let assign14040_e21136: f64 = (assign14040_e21124 * assign14040_e21135);
        let assign14040_e21139: f64 = (4.0 * 0.001);
        let assign14040_e21141: f64 = (assign14040_e21139 * 0.001);
        let assign14040_e21142: f64 = (assign14040_e21136 + assign14040_e21141);
        let assign14040_e21143: f64 = (assign14040_e21142).sqrt();
        let assign14040_e21144: f64 = (assign14040_e21113 + assign14040_e21143);
        let assign14040_e21145: f64 = (0.5 * assign14040_e21144);
        (assign14040_e21145, (0.5 * ((p.p747 * (p.p908 * locals.var_tratio_dn4)) + ((((p.p747 * (p.p908 * locals.var_tratio_dn4)) * assign14040_e21135) + (assign14040_e21124 * (p.p747 * (p.p908 * locals.var_tratio_dn4)))) / (2.0 * assign14040_e21143)))),)
    } else {
        let assign14040_e21151: f64 = (locals.var_tratio - 1.0);
        let assign14040_e21152: f64 = (p.p908 * assign14040_e21151);
        let assign14040_e21153: f64 = (1.0 + assign14040_e21152);
        let assign14040_e21154: f64 = (p.p747 * assign14040_e21153);
        let assign14040_e21156: f64 = (assign14040_e21154 - 0.01);
        let assign14040_e21158: f64 = (-10000.0);
        let assign14040_e21160: f64 = (assign14040_e21158 * 0.001);
        let (assign14040_e21179, assign14040_e21179_d_n4,) = {
            if (assign14040_e21156 < assign14040_e21160) {
                let assign14040_e21163: f64 = (-0.001);
                let assign14040_e21165: f64 = (assign14040_e21163 * 0.001);
                let assign14040_e21171: f64 = (locals.var_tratio - 1.0);
                let assign14040_e21172: f64 = (p.p908 * assign14040_e21171);
                let assign14040_e21173: f64 = (1.0 + assign14040_e21172);
                let assign14040_e21174: f64 = (p.p747 * assign14040_e21173);
                let assign14040_e21176: f64 = (assign14040_e21174 - 0.01);
                let assign14040_e21177: f64 = (assign14040_e21165 / assign14040_e21176);
                (assign14040_e21177, (-((assign14040_e21165 * (p.p747 * (p.p908 * locals.var_tratio_dn4))) / (assign14040_e21176 * assign14040_e21176))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14040_e21179, assign14040_e21179_d_n4,)
    }
};
        let assign14040_e21182: f64 = (assign14040_e21180 + 0.01);
        locals.var_njtsswgd_t = assign14040_e21182;
        locals.var_njtsswgd_t_dn4 = assign14040_e21180_d_n4;
        locals.var_njtsswgd_t_rv = 0.0;

        let assign14050_e21185: f64 = if p.p9 < 9.0 { 1.0 } else { 0.0 };
        locals.var_guard462 = assign14050_e21185;
        locals.var_guard462_rv = 0.0;

        let assign14060_e21188: f64 = (p.p2 % 2.0);
        let assign14060_e21190: f64 = if assign14060_e21188 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard463 = assign14060_e21190;
        locals.var_guard463_rv = 0.0;

        let (assign14070_e21196,) = {
    if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14070_e21196;
        locals.var_nuendd_rv = 0.0;

        let (assign14080_e21202,) = {
    if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14080_e21202;
        locals.var_nuends_rv = 0.0;

        let (assign14090_e21216,) = {
    if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
        let assign14090_e21209: f64 = (p.p2 - 1.0);
        let assign14090_e21211: f64 = (assign14090_e21209 / 2.0);
        let assign14090_e21213: f64 = (assign14090_e21211).max(0.0);
        let assign14090_e21214: f64 = (2.0 * assign14090_e21213);
        (assign14090_e21214,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14090_e21216;
        locals.var_nuintd_rv = 0.0;

        let (assign14100_e21222,) = {
    if ((locals.var_guard462 != 0.0) && (locals.var_guard463 != 0.0)) {
        (locals.var_nuintd,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14100_e21222;
        locals.var_nuints_rv = 0.0;

        let assign14110_e21225: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard464 = assign14110_e21225;
        locals.var_guard464_rv = 0.0;

        let (assign14120_e21234,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14120_e21234;
        locals.var_nuendd_rv = 0.0;

        let (assign14130_e21251,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
        let assign14130_e21244: f64 = (p.p2 / 2.0);
        let assign14130_e21246: f64 = (assign14130_e21244 - 1.0);
        let assign14130_e21248: f64 = (assign14130_e21246).max(0.0);
        let assign14130_e21249: f64 = (2.0 * assign14130_e21248);
        (assign14130_e21249,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14130_e21251;
        locals.var_nuintd_rv = 0.0;

        let (assign14140_e21260,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14140_e21260;
        locals.var_nuends_rv = 0.0;

        let (assign14150_e21269,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 != 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14150_e21269;
        locals.var_nuints_rv = 0.0;

        let (assign14160_e21279,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_nuendd,)
    }
};
        locals.var_nuendd = assign14160_e21279;
        locals.var_nuendd_rv = 0.0;

        let (assign14170_e21289,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
        (p.p2,)
    } else {
        (locals.var_nuintd,)
    }
};
        locals.var_nuintd = assign14170_e21289;
        locals.var_nuintd_rv = 0.0;

        let (assign14180_e21299,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
        (2.0,)
    } else {
        (locals.var_nuends,)
    }
};
        locals.var_nuends = assign14180_e21299;
        locals.var_nuends_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14190_e21317,) = {
    if (((locals.var_guard462 != 0.0) && (locals.var_guard463 == 0.0)) && (locals.var_guard464 == 0.0)) {
        let assign14190_e21310: f64 = (p.p2 / 2.0);
        let assign14190_e21312: f64 = (assign14190_e21310 - 1.0);
        let assign14190_e21314: f64 = (assign14190_e21312).max(0.0);
        let assign14190_e21315: f64 = (2.0 * assign14190_e21314);
        (assign14190_e21315,)
    } else {
        (locals.var_nuints,)
    }
};
        locals.var_nuints = assign14190_e21317;
        locals.var_nuints_rv = 0.0;

        let assign14200_e21320: f64 = (locals.var_dmcgeff + locals.var_dmcieff);
        locals.var_t0 = assign14200_e21320;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign14210_e21323: f64 = (locals.var_dmcgeff + locals.var_dmcgeff);
        locals.var_t1 = assign14210_e21323;
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
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign14220_e21326: f64 = (locals.var_dmdgeff + locals.var_dmdgeff);
        locals.var_t2 = assign14220_e21326;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn3 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn12 = 0.0;
        locals.var_t2_dn13 = 0.0;
        locals.var_t2_dn14 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign14230_e21329: f64 = (locals.var_t0 + locals.var_t0);
        let assign14230_e21331: f64 = (assign14230_e21329 + locals.var_weffcj);
        locals.var_psiso = assign14230_e21331;
        locals.var_psiso_dn0 = (locals.var_t0_dn0 + locals.var_t0_dn0);
        locals.var_psiso_dn2 = (locals.var_t0_dn2 + locals.var_t0_dn2);
        locals.var_psiso_dn3 = (locals.var_t0_dn3 + locals.var_t0_dn3);
        locals.var_psiso_dn4 = (locals.var_t0_dn4 + locals.var_t0_dn4);
        locals.var_psiso_dn5 = (locals.var_t0_dn5 + locals.var_t0_dn5);
        locals.var_psiso_dn6 = (locals.var_t0_dn6 + locals.var_t0_dn6);
        locals.var_psiso_dn7 = (locals.var_t0_dn7 + locals.var_t0_dn7);
        locals.var_psiso_dn8 = (locals.var_t0_dn8 + locals.var_t0_dn8);
        locals.var_psiso_dn9 = (locals.var_t0_dn9 + locals.var_t0_dn9);
        locals.var_psiso_dn10 = (locals.var_t0_dn10 + locals.var_t0_dn10);
        locals.var_psiso_dn11 = (locals.var_t0_dn11 + locals.var_t0_dn11);
        locals.var_psiso_dn12 = (locals.var_t0_dn12 + locals.var_t0_dn12);
        locals.var_psiso_dn13 = (locals.var_t0_dn13 + locals.var_t0_dn13);
        locals.var_psiso_dn14 = (locals.var_t0_dn14 + locals.var_t0_dn14);
        locals.var_psiso_rv = 0.0;

        let assign14240_e21334: f64 = (locals.var_t0 + locals.var_t0);
        let assign14240_e21336: f64 = (assign14240_e21334 + locals.var_weffcj);
        locals.var_pdiso = assign14240_e21336;
        locals.var_pdiso_dn0 = (locals.var_t0_dn0 + locals.var_t0_dn0);
        locals.var_pdiso_dn2 = (locals.var_t0_dn2 + locals.var_t0_dn2);
        locals.var_pdiso_dn3 = (locals.var_t0_dn3 + locals.var_t0_dn3);
        locals.var_pdiso_dn4 = (locals.var_t0_dn4 + locals.var_t0_dn4);
        locals.var_pdiso_dn5 = (locals.var_t0_dn5 + locals.var_t0_dn5);
        locals.var_pdiso_dn6 = (locals.var_t0_dn6 + locals.var_t0_dn6);
        locals.var_pdiso_dn7 = (locals.var_t0_dn7 + locals.var_t0_dn7);
        locals.var_pdiso_dn8 = (locals.var_t0_dn8 + locals.var_t0_dn8);
        locals.var_pdiso_dn9 = (locals.var_t0_dn9 + locals.var_t0_dn9);
        locals.var_pdiso_dn10 = (locals.var_t0_dn10 + locals.var_t0_dn10);
        locals.var_pdiso_dn11 = (locals.var_t0_dn11 + locals.var_t0_dn11);
        locals.var_pdiso_dn12 = (locals.var_t0_dn12 + locals.var_t0_dn12);
        locals.var_pdiso_dn13 = (locals.var_t0_dn13 + locals.var_t0_dn13);
        locals.var_pdiso_dn14 = (locals.var_t0_dn14 + locals.var_t0_dn14);
        locals.var_pdiso_rv = 0.0;

        locals.var_pssha = locals.var_t1;
        locals.var_pssha_dn0 = locals.var_t1_dn0;
        locals.var_pssha_dn2 = locals.var_t1_dn2;
        locals.var_pssha_dn3 = locals.var_t1_dn3;
        locals.var_pssha_dn4 = locals.var_t1_dn4;
        locals.var_pssha_dn5 = locals.var_t1_dn5;
        locals.var_pssha_dn6 = locals.var_t1_dn6;
        locals.var_pssha_dn7 = locals.var_t1_dn7;
        locals.var_pssha_dn8 = locals.var_t1_dn8;
        locals.var_pssha_dn9 = locals.var_t1_dn9;
        locals.var_pssha_dn10 = locals.var_t1_dn10;
        locals.var_pssha_dn11 = locals.var_t1_dn11;
        locals.var_pssha_dn12 = locals.var_t1_dn12;
        locals.var_pssha_dn13 = locals.var_t1_dn13;
        locals.var_pssha_dn14 = locals.var_t1_dn14;
        locals.var_pssha_rv = 0.0;

        locals.var_pdsha = locals.var_t1;
        locals.var_pdsha_dn0 = locals.var_t1_dn0;
        locals.var_pdsha_dn2 = locals.var_t1_dn2;
        locals.var_pdsha_dn3 = locals.var_t1_dn3;
        locals.var_pdsha_dn4 = locals.var_t1_dn4;
        locals.var_pdsha_dn5 = locals.var_t1_dn5;
        locals.var_pdsha_dn6 = locals.var_t1_dn6;
        locals.var_pdsha_dn7 = locals.var_t1_dn7;
        locals.var_pdsha_dn8 = locals.var_t1_dn8;
        locals.var_pdsha_dn9 = locals.var_t1_dn9;
        locals.var_pdsha_dn10 = locals.var_t1_dn10;
        locals.var_pdsha_dn11 = locals.var_t1_dn11;
        locals.var_pdsha_dn12 = locals.var_t1_dn12;
        locals.var_pdsha_dn13 = locals.var_t1_dn13;
        locals.var_pdsha_dn14 = locals.var_t1_dn14;
        locals.var_pdsha_rv = 0.0;

        locals.var_psmer = locals.var_t2;
        locals.var_psmer_dn0 = locals.var_t2_dn0;
        locals.var_psmer_dn2 = locals.var_t2_dn2;
        locals.var_psmer_dn3 = locals.var_t2_dn3;
        locals.var_psmer_dn4 = locals.var_t2_dn4;
        locals.var_psmer_dn5 = locals.var_t2_dn5;
        locals.var_psmer_dn6 = locals.var_t2_dn6;
        locals.var_psmer_dn7 = locals.var_t2_dn7;
        locals.var_psmer_dn8 = locals.var_t2_dn8;
        locals.var_psmer_dn9 = locals.var_t2_dn9;
        locals.var_psmer_dn10 = locals.var_t2_dn10;
        locals.var_psmer_dn11 = locals.var_t2_dn11;
        locals.var_psmer_dn12 = locals.var_t2_dn12;
        locals.var_psmer_dn13 = locals.var_t2_dn13;
        locals.var_psmer_dn14 = locals.var_t2_dn14;
        locals.var_psmer_rv = 0.0;

        locals.var_pdmer = locals.var_t2;
        locals.var_pdmer_dn0 = locals.var_t2_dn0;
        locals.var_pdmer_dn2 = locals.var_t2_dn2;
        locals.var_pdmer_dn3 = locals.var_t2_dn3;
        locals.var_pdmer_dn4 = locals.var_t2_dn4;
        locals.var_pdmer_dn5 = locals.var_t2_dn5;
        locals.var_pdmer_dn6 = locals.var_t2_dn6;
        locals.var_pdmer_dn7 = locals.var_t2_dn7;
        locals.var_pdmer_dn8 = locals.var_t2_dn8;
        locals.var_pdmer_dn9 = locals.var_t2_dn9;
        locals.var_pdmer_dn10 = locals.var_t2_dn10;
        locals.var_pdmer_dn11 = locals.var_t2_dn11;
        locals.var_pdmer_dn12 = locals.var_t2_dn12;
        locals.var_pdmer_dn13 = locals.var_t2_dn13;
        locals.var_pdmer_dn14 = locals.var_t2_dn14;
        locals.var_pdmer_rv = 0.0;

        let assign14290_e21343: f64 = (locals.var_t0 * locals.var_weffcj);
        locals.var_asiso = assign14290_e21343;
        locals.var_asiso_dn0 = (locals.var_t0_dn0 * locals.var_weffcj);
        locals.var_asiso_dn2 = (locals.var_t0_dn2 * locals.var_weffcj);
        locals.var_asiso_dn3 = (locals.var_t0_dn3 * locals.var_weffcj);
        locals.var_asiso_dn4 = (locals.var_t0_dn4 * locals.var_weffcj);
        locals.var_asiso_dn5 = (locals.var_t0_dn5 * locals.var_weffcj);
        locals.var_asiso_dn6 = (locals.var_t0_dn6 * locals.var_weffcj);
        locals.var_asiso_dn7 = (locals.var_t0_dn7 * locals.var_weffcj);
        locals.var_asiso_dn8 = (locals.var_t0_dn8 * locals.var_weffcj);
        locals.var_asiso_dn9 = (locals.var_t0_dn9 * locals.var_weffcj);
        locals.var_asiso_dn10 = (locals.var_t0_dn10 * locals.var_weffcj);
        locals.var_asiso_dn11 = (locals.var_t0_dn11 * locals.var_weffcj);
        locals.var_asiso_dn12 = (locals.var_t0_dn12 * locals.var_weffcj);
        locals.var_asiso_dn13 = (locals.var_t0_dn13 * locals.var_weffcj);
        locals.var_asiso_dn14 = (locals.var_t0_dn14 * locals.var_weffcj);
        locals.var_asiso_rv = 0.0;

        let assign14300_e21346: f64 = (locals.var_t0 * locals.var_weffcj);
        locals.var_adiso = assign14300_e21346;
        locals.var_adiso_dn0 = (locals.var_t0_dn0 * locals.var_weffcj);
        locals.var_adiso_dn2 = (locals.var_t0_dn2 * locals.var_weffcj);
        locals.var_adiso_dn3 = (locals.var_t0_dn3 * locals.var_weffcj);
        locals.var_adiso_dn4 = (locals.var_t0_dn4 * locals.var_weffcj);
        locals.var_adiso_dn5 = (locals.var_t0_dn5 * locals.var_weffcj);
        locals.var_adiso_dn6 = (locals.var_t0_dn6 * locals.var_weffcj);
        locals.var_adiso_dn7 = (locals.var_t0_dn7 * locals.var_weffcj);
        locals.var_adiso_dn8 = (locals.var_t0_dn8 * locals.var_weffcj);
        locals.var_adiso_dn9 = (locals.var_t0_dn9 * locals.var_weffcj);
        locals.var_adiso_dn10 = (locals.var_t0_dn10 * locals.var_weffcj);
        locals.var_adiso_dn11 = (locals.var_t0_dn11 * locals.var_weffcj);
        locals.var_adiso_dn12 = (locals.var_t0_dn12 * locals.var_weffcj);
        locals.var_adiso_dn13 = (locals.var_t0_dn13 * locals.var_weffcj);
        locals.var_adiso_dn14 = (locals.var_t0_dn14 * locals.var_weffcj);
        locals.var_adiso_rv = 0.0;

        let assign14310_e21349: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_assha = assign14310_e21349;
        locals.var_assha_rv = 0.0;

        let assign14320_e21352: f64 = (locals.var_dmcgeff * locals.var_weffcj);
        locals.var_adsha = assign14320_e21352;
        locals.var_adsha_rv = 0.0;

        let assign14330_e21355: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_asmer = assign14330_e21355;
        locals.var_asmer_rv = 0.0;

        let assign14340_e21358: f64 = (locals.var_dmdgeff * locals.var_weffcj);
        locals.var_admer = assign14340_e21358;
        locals.var_admer_rv = 0.0;

        let assign14350_e21361: f64 = if p.p9 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard465 = assign14350_e21361;
        locals.var_guard465_rv = 0.0;

        let assign14360_e21364: f64 = if p.p9 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard466 = assign14360_e21364;
        locals.var_guard466_rv = 0.0;

        let assign14370_e21367: f64 = if p.p9 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard467 = assign14370_e21367;
        locals.var_guard467_rv = 0.0;

        let assign14380_e21370: f64 = if p.p9 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard468 = assign14380_e21370;
        locals.var_guard468_rv = 0.0;

        let assign14390_e21373: f64 = if p.p9 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard469 = assign14390_e21373;
        locals.var_guard469_rv = 0.0;

        let assign14400_e21376: f64 = if p.p9 == 5.0 { 1.0 } else { 0.0 };
        locals.var_guard470 = assign14400_e21376;
        locals.var_guard470_rv = 0.0;

        let assign14410_e21379: f64 = if p.p9 == 6.0 { 1.0 } else { 0.0 };
        locals.var_guard471 = assign14410_e21379;
        locals.var_guard471_rv = 0.0;

        let assign14420_e21382: f64 = if p.p9 == 7.0 { 1.0 } else { 0.0 };
        locals.var_guard472 = assign14420_e21382;
        locals.var_guard472_rv = 0.0;

        let assign14430_e21385: f64 = if p.p9 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign14430_e21385;
        locals.var_guard473_rv = 0.0;

        let assign14440_e21388: f64 = if p.p9 == 9.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign14440_e21388;
        locals.var_guard474_rv = 0.0;

        let assign14450_e21391: f64 = if p.p9 == 10.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign14450_e21391;
        locals.var_guard475_rv = 0.0;

        let (assign14460_e21401, assign14460_e21401_d_n0, assign14460_e21401_d_n2, assign14460_e21401_d_n3, assign14460_e21401_d_n4, assign14460_e21401_d_n5, assign14460_e21401_d_n6, assign14460_e21401_d_n7, assign14460_e21401_d_n8, assign14460_e21401_d_n9, assign14460_e21401_d_n10, assign14460_e21401_d_n11, assign14460_e21401_d_n12, assign14460_e21401_d_n13, assign14460_e21401_d_n14,) = {
    if (locals.var_guard465 != 0.0) {
        let assign14460_e21395: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14460_e21398: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14460_e21399: f64 = (assign14460_e21395 + assign14460_e21398);
        (assign14460_e21399, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14460_e21401;
        locals.var_temp_pseff_dn0 = assign14460_e21401_d_n0;
        locals.var_temp_pseff_dn2 = assign14460_e21401_d_n2;
        locals.var_temp_pseff_dn3 = assign14460_e21401_d_n3;
        locals.var_temp_pseff_dn4 = assign14460_e21401_d_n4;
        locals.var_temp_pseff_dn5 = assign14460_e21401_d_n5;
        locals.var_temp_pseff_dn6 = assign14460_e21401_d_n6;
        locals.var_temp_pseff_dn7 = assign14460_e21401_d_n7;
        locals.var_temp_pseff_dn8 = assign14460_e21401_d_n8;
        locals.var_temp_pseff_dn9 = assign14460_e21401_d_n9;
        locals.var_temp_pseff_dn10 = assign14460_e21401_d_n10;
        locals.var_temp_pseff_dn11 = assign14460_e21401_d_n11;
        locals.var_temp_pseff_dn12 = assign14460_e21401_d_n12;
        locals.var_temp_pseff_dn13 = assign14460_e21401_d_n13;
        locals.var_temp_pseff_dn14 = assign14460_e21401_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14470_e21411, assign14470_e21411_d_n0, assign14470_e21411_d_n2, assign14470_e21411_d_n3, assign14470_e21411_d_n4, assign14470_e21411_d_n5, assign14470_e21411_d_n6, assign14470_e21411_d_n7, assign14470_e21411_d_n8, assign14470_e21411_d_n9, assign14470_e21411_d_n10, assign14470_e21411_d_n11, assign14470_e21411_d_n12, assign14470_e21411_d_n13, assign14470_e21411_d_n14,) = {
    if (locals.var_guard465 != 0.0) {
        let assign14470_e21405: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14470_e21408: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14470_e21409: f64 = (assign14470_e21405 + assign14470_e21408);
        (assign14470_e21409, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14470_e21411;
        locals.var_temp_pdeff_dn0 = assign14470_e21411_d_n0;
        locals.var_temp_pdeff_dn2 = assign14470_e21411_d_n2;
        locals.var_temp_pdeff_dn3 = assign14470_e21411_d_n3;
        locals.var_temp_pdeff_dn4 = assign14470_e21411_d_n4;
        locals.var_temp_pdeff_dn5 = assign14470_e21411_d_n5;
        locals.var_temp_pdeff_dn6 = assign14470_e21411_d_n6;
        locals.var_temp_pdeff_dn7 = assign14470_e21411_d_n7;
        locals.var_temp_pdeff_dn8 = assign14470_e21411_d_n8;
        locals.var_temp_pdeff_dn9 = assign14470_e21411_d_n9;
        locals.var_temp_pdeff_dn10 = assign14470_e21411_d_n10;
        locals.var_temp_pdeff_dn11 = assign14470_e21411_d_n11;
        locals.var_temp_pdeff_dn12 = assign14470_e21411_d_n12;
        locals.var_temp_pdeff_dn13 = assign14470_e21411_d_n13;
        locals.var_temp_pdeff_dn14 = assign14470_e21411_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14480_e21421, assign14480_e21421_d_n0, assign14480_e21421_d_n2, assign14480_e21421_d_n3, assign14480_e21421_d_n4, assign14480_e21421_d_n5, assign14480_e21421_d_n6, assign14480_e21421_d_n7, assign14480_e21421_d_n8, assign14480_e21421_d_n9, assign14480_e21421_d_n10, assign14480_e21421_d_n11, assign14480_e21421_d_n12, assign14480_e21421_d_n13, assign14480_e21421_d_n14,) = {
    if (locals.var_guard465 != 0.0) {
        let assign14480_e21415: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14480_e21418: f64 = (locals.var_nuints * locals.var_assha);
        let assign14480_e21419: f64 = (assign14480_e21415 + assign14480_e21418);
        (assign14480_e21419, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14480_e21421;
        locals.var_temp_aseff_dn0 = assign14480_e21421_d_n0;
        locals.var_temp_aseff_dn2 = assign14480_e21421_d_n2;
        locals.var_temp_aseff_dn3 = assign14480_e21421_d_n3;
        locals.var_temp_aseff_dn4 = assign14480_e21421_d_n4;
        locals.var_temp_aseff_dn5 = assign14480_e21421_d_n5;
        locals.var_temp_aseff_dn6 = assign14480_e21421_d_n6;
        locals.var_temp_aseff_dn7 = assign14480_e21421_d_n7;
        locals.var_temp_aseff_dn8 = assign14480_e21421_d_n8;
        locals.var_temp_aseff_dn9 = assign14480_e21421_d_n9;
        locals.var_temp_aseff_dn10 = assign14480_e21421_d_n10;
        locals.var_temp_aseff_dn11 = assign14480_e21421_d_n11;
        locals.var_temp_aseff_dn12 = assign14480_e21421_d_n12;
        locals.var_temp_aseff_dn13 = assign14480_e21421_d_n13;
        locals.var_temp_aseff_dn14 = assign14480_e21421_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14490_e21431, assign14490_e21431_d_n0, assign14490_e21431_d_n2, assign14490_e21431_d_n3, assign14490_e21431_d_n4, assign14490_e21431_d_n5, assign14490_e21431_d_n6, assign14490_e21431_d_n7, assign14490_e21431_d_n8, assign14490_e21431_d_n9, assign14490_e21431_d_n10, assign14490_e21431_d_n11, assign14490_e21431_d_n12, assign14490_e21431_d_n13, assign14490_e21431_d_n14,) = {
    if (locals.var_guard465 != 0.0) {
        let assign14490_e21425: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14490_e21428: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14490_e21429: f64 = (assign14490_e21425 + assign14490_e21428);
        (assign14490_e21429, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14490_e21431;
        locals.var_temp_adeff_dn0 = assign14490_e21431_d_n0;
        locals.var_temp_adeff_dn2 = assign14490_e21431_d_n2;
        locals.var_temp_adeff_dn3 = assign14490_e21431_d_n3;
        locals.var_temp_adeff_dn4 = assign14490_e21431_d_n4;
        locals.var_temp_adeff_dn5 = assign14490_e21431_d_n5;
        locals.var_temp_adeff_dn6 = assign14490_e21431_d_n6;
        locals.var_temp_adeff_dn7 = assign14490_e21431_d_n7;
        locals.var_temp_adeff_dn8 = assign14490_e21431_d_n8;
        locals.var_temp_adeff_dn9 = assign14490_e21431_d_n9;
        locals.var_temp_adeff_dn10 = assign14490_e21431_d_n10;
        locals.var_temp_adeff_dn11 = assign14490_e21431_d_n11;
        locals.var_temp_adeff_dn12 = assign14490_e21431_d_n12;
        locals.var_temp_adeff_dn13 = assign14490_e21431_d_n13;
        locals.var_temp_adeff_dn14 = assign14490_e21431_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14500_e21444, assign14500_e21444_d_n0, assign14500_e21444_d_n2, assign14500_e21444_d_n3, assign14500_e21444_d_n4, assign14500_e21444_d_n5, assign14500_e21444_d_n6, assign14500_e21444_d_n7, assign14500_e21444_d_n8, assign14500_e21444_d_n9, assign14500_e21444_d_n10, assign14500_e21444_d_n11, assign14500_e21444_d_n12, assign14500_e21444_d_n13, assign14500_e21444_d_n14,) = {
    if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
        let assign14500_e21438: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14500_e21441: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14500_e21442: f64 = (assign14500_e21438 + assign14500_e21441);
        (assign14500_e21442, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14500_e21444;
        locals.var_temp_pseff_dn0 = assign14500_e21444_d_n0;
        locals.var_temp_pseff_dn2 = assign14500_e21444_d_n2;
        locals.var_temp_pseff_dn3 = assign14500_e21444_d_n3;
        locals.var_temp_pseff_dn4 = assign14500_e21444_d_n4;
        locals.var_temp_pseff_dn5 = assign14500_e21444_d_n5;
        locals.var_temp_pseff_dn6 = assign14500_e21444_d_n6;
        locals.var_temp_pseff_dn7 = assign14500_e21444_d_n7;
        locals.var_temp_pseff_dn8 = assign14500_e21444_d_n8;
        locals.var_temp_pseff_dn9 = assign14500_e21444_d_n9;
        locals.var_temp_pseff_dn10 = assign14500_e21444_d_n10;
        locals.var_temp_pseff_dn11 = assign14500_e21444_d_n11;
        locals.var_temp_pseff_dn12 = assign14500_e21444_d_n12;
        locals.var_temp_pseff_dn13 = assign14500_e21444_d_n13;
        locals.var_temp_pseff_dn14 = assign14500_e21444_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14510_e21455, assign14510_e21455_d_n0, assign14510_e21455_d_n2, assign14510_e21455_d_n3, assign14510_e21455_d_n4, assign14510_e21455_d_n5, assign14510_e21455_d_n6, assign14510_e21455_d_n7, assign14510_e21455_d_n8, assign14510_e21455_d_n9, assign14510_e21455_d_n10, assign14510_e21455_d_n11, assign14510_e21455_d_n12, assign14510_e21455_d_n13, assign14510_e21455_d_n14,) = {
    if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
        let assign14510_e21451: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14510_e21453: f64 = (assign14510_e21451 * locals.var_pdsha);
        (assign14510_e21453, (assign14510_e21451 * locals.var_pdsha_dn0), (assign14510_e21451 * locals.var_pdsha_dn2), (assign14510_e21451 * locals.var_pdsha_dn3), (assign14510_e21451 * locals.var_pdsha_dn4), (assign14510_e21451 * locals.var_pdsha_dn5), (assign14510_e21451 * locals.var_pdsha_dn6), (assign14510_e21451 * locals.var_pdsha_dn7), (assign14510_e21451 * locals.var_pdsha_dn8), (assign14510_e21451 * locals.var_pdsha_dn9), (assign14510_e21451 * locals.var_pdsha_dn10), (assign14510_e21451 * locals.var_pdsha_dn11), (assign14510_e21451 * locals.var_pdsha_dn12), (assign14510_e21451 * locals.var_pdsha_dn13), (assign14510_e21451 * locals.var_pdsha_dn14),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14510_e21455;
        locals.var_temp_pdeff_dn0 = assign14510_e21455_d_n0;
        locals.var_temp_pdeff_dn2 = assign14510_e21455_d_n2;
        locals.var_temp_pdeff_dn3 = assign14510_e21455_d_n3;
        locals.var_temp_pdeff_dn4 = assign14510_e21455_d_n4;
        locals.var_temp_pdeff_dn5 = assign14510_e21455_d_n5;
        locals.var_temp_pdeff_dn6 = assign14510_e21455_d_n6;
        locals.var_temp_pdeff_dn7 = assign14510_e21455_d_n7;
        locals.var_temp_pdeff_dn8 = assign14510_e21455_d_n8;
        locals.var_temp_pdeff_dn9 = assign14510_e21455_d_n9;
        locals.var_temp_pdeff_dn10 = assign14510_e21455_d_n10;
        locals.var_temp_pdeff_dn11 = assign14510_e21455_d_n11;
        locals.var_temp_pdeff_dn12 = assign14510_e21455_d_n12;
        locals.var_temp_pdeff_dn13 = assign14510_e21455_d_n13;
        locals.var_temp_pdeff_dn14 = assign14510_e21455_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14520_e21468, assign14520_e21468_d_n0, assign14520_e21468_d_n2, assign14520_e21468_d_n3, assign14520_e21468_d_n4, assign14520_e21468_d_n5, assign14520_e21468_d_n6, assign14520_e21468_d_n7, assign14520_e21468_d_n8, assign14520_e21468_d_n9, assign14520_e21468_d_n10, assign14520_e21468_d_n11, assign14520_e21468_d_n12, assign14520_e21468_d_n13, assign14520_e21468_d_n14,) = {
    if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
        let assign14520_e21462: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14520_e21465: f64 = (locals.var_nuints * locals.var_assha);
        let assign14520_e21466: f64 = (assign14520_e21462 + assign14520_e21465);
        (assign14520_e21466, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14520_e21468;
        locals.var_temp_aseff_dn0 = assign14520_e21468_d_n0;
        locals.var_temp_aseff_dn2 = assign14520_e21468_d_n2;
        locals.var_temp_aseff_dn3 = assign14520_e21468_d_n3;
        locals.var_temp_aseff_dn4 = assign14520_e21468_d_n4;
        locals.var_temp_aseff_dn5 = assign14520_e21468_d_n5;
        locals.var_temp_aseff_dn6 = assign14520_e21468_d_n6;
        locals.var_temp_aseff_dn7 = assign14520_e21468_d_n7;
        locals.var_temp_aseff_dn8 = assign14520_e21468_d_n8;
        locals.var_temp_aseff_dn9 = assign14520_e21468_d_n9;
        locals.var_temp_aseff_dn10 = assign14520_e21468_d_n10;
        locals.var_temp_aseff_dn11 = assign14520_e21468_d_n11;
        locals.var_temp_aseff_dn12 = assign14520_e21468_d_n12;
        locals.var_temp_aseff_dn13 = assign14520_e21468_d_n13;
        locals.var_temp_aseff_dn14 = assign14520_e21468_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14530_e21479, assign14530_e21479_d_n0, assign14530_e21479_d_n2, assign14530_e21479_d_n3, assign14530_e21479_d_n4, assign14530_e21479_d_n5, assign14530_e21479_d_n6, assign14530_e21479_d_n7, assign14530_e21479_d_n8, assign14530_e21479_d_n9, assign14530_e21479_d_n10, assign14530_e21479_d_n11, assign14530_e21479_d_n12, assign14530_e21479_d_n13, assign14530_e21479_d_n14,) = {
    if ((locals.var_guard466 != 0.0) && (locals.var_guard465 == 0.0)) {
        let assign14530_e21475: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14530_e21477: f64 = (assign14530_e21475 * locals.var_adsha);
        (assign14530_e21477, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14530_e21479;
        locals.var_temp_adeff_dn0 = assign14530_e21479_d_n0;
        locals.var_temp_adeff_dn2 = assign14530_e21479_d_n2;
        locals.var_temp_adeff_dn3 = assign14530_e21479_d_n3;
        locals.var_temp_adeff_dn4 = assign14530_e21479_d_n4;
        locals.var_temp_adeff_dn5 = assign14530_e21479_d_n5;
        locals.var_temp_adeff_dn6 = assign14530_e21479_d_n6;
        locals.var_temp_adeff_dn7 = assign14530_e21479_d_n7;
        locals.var_temp_adeff_dn8 = assign14530_e21479_d_n8;
        locals.var_temp_adeff_dn9 = assign14530_e21479_d_n9;
        locals.var_temp_adeff_dn10 = assign14530_e21479_d_n10;
        locals.var_temp_adeff_dn11 = assign14530_e21479_d_n11;
        locals.var_temp_adeff_dn12 = assign14530_e21479_d_n12;
        locals.var_temp_adeff_dn13 = assign14530_e21479_d_n13;
        locals.var_temp_adeff_dn14 = assign14530_e21479_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14540_e21492, assign14540_e21492_d_n0, assign14540_e21492_d_n2, assign14540_e21492_d_n3, assign14540_e21492_d_n4, assign14540_e21492_d_n5, assign14540_e21492_d_n6, assign14540_e21492_d_n7, assign14540_e21492_d_n8, assign14540_e21492_d_n9, assign14540_e21492_d_n10, assign14540_e21492_d_n11, assign14540_e21492_d_n12, assign14540_e21492_d_n13, assign14540_e21492_d_n14,) = {
    if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
        let assign14540_e21488: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14540_e21490: f64 = (assign14540_e21488 * locals.var_pssha);
        (assign14540_e21490, (assign14540_e21488 * locals.var_pssha_dn0), (assign14540_e21488 * locals.var_pssha_dn2), (assign14540_e21488 * locals.var_pssha_dn3), (assign14540_e21488 * locals.var_pssha_dn4), (assign14540_e21488 * locals.var_pssha_dn5), (assign14540_e21488 * locals.var_pssha_dn6), (assign14540_e21488 * locals.var_pssha_dn7), (assign14540_e21488 * locals.var_pssha_dn8), (assign14540_e21488 * locals.var_pssha_dn9), (assign14540_e21488 * locals.var_pssha_dn10), (assign14540_e21488 * locals.var_pssha_dn11), (assign14540_e21488 * locals.var_pssha_dn12), (assign14540_e21488 * locals.var_pssha_dn13), (assign14540_e21488 * locals.var_pssha_dn14),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14540_e21492;
        locals.var_temp_pseff_dn0 = assign14540_e21492_d_n0;
        locals.var_temp_pseff_dn2 = assign14540_e21492_d_n2;
        locals.var_temp_pseff_dn3 = assign14540_e21492_d_n3;
        locals.var_temp_pseff_dn4 = assign14540_e21492_d_n4;
        locals.var_temp_pseff_dn5 = assign14540_e21492_d_n5;
        locals.var_temp_pseff_dn6 = assign14540_e21492_d_n6;
        locals.var_temp_pseff_dn7 = assign14540_e21492_d_n7;
        locals.var_temp_pseff_dn8 = assign14540_e21492_d_n8;
        locals.var_temp_pseff_dn9 = assign14540_e21492_d_n9;
        locals.var_temp_pseff_dn10 = assign14540_e21492_d_n10;
        locals.var_temp_pseff_dn11 = assign14540_e21492_d_n11;
        locals.var_temp_pseff_dn12 = assign14540_e21492_d_n12;
        locals.var_temp_pseff_dn13 = assign14540_e21492_d_n13;
        locals.var_temp_pseff_dn14 = assign14540_e21492_d_n14;
        locals.var_temp_pseff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign14550_e21507, assign14550_e21507_d_n0, assign14550_e21507_d_n2, assign14550_e21507_d_n3, assign14550_e21507_d_n4, assign14550_e21507_d_n5, assign14550_e21507_d_n6, assign14550_e21507_d_n7, assign14550_e21507_d_n8, assign14550_e21507_d_n9, assign14550_e21507_d_n10, assign14550_e21507_d_n11, assign14550_e21507_d_n12, assign14550_e21507_d_n13, assign14550_e21507_d_n14,) = {
    if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
        let assign14550_e21501: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14550_e21504: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14550_e21505: f64 = (assign14550_e21501 + assign14550_e21504);
        (assign14550_e21505, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14550_e21507;
        locals.var_temp_pdeff_dn0 = assign14550_e21507_d_n0;
        locals.var_temp_pdeff_dn2 = assign14550_e21507_d_n2;
        locals.var_temp_pdeff_dn3 = assign14550_e21507_d_n3;
        locals.var_temp_pdeff_dn4 = assign14550_e21507_d_n4;
        locals.var_temp_pdeff_dn5 = assign14550_e21507_d_n5;
        locals.var_temp_pdeff_dn6 = assign14550_e21507_d_n6;
        locals.var_temp_pdeff_dn7 = assign14550_e21507_d_n7;
        locals.var_temp_pdeff_dn8 = assign14550_e21507_d_n8;
        locals.var_temp_pdeff_dn9 = assign14550_e21507_d_n9;
        locals.var_temp_pdeff_dn10 = assign14550_e21507_d_n10;
        locals.var_temp_pdeff_dn11 = assign14550_e21507_d_n11;
        locals.var_temp_pdeff_dn12 = assign14550_e21507_d_n12;
        locals.var_temp_pdeff_dn13 = assign14550_e21507_d_n13;
        locals.var_temp_pdeff_dn14 = assign14550_e21507_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14560_e21520, assign14560_e21520_d_n0, assign14560_e21520_d_n2, assign14560_e21520_d_n3, assign14560_e21520_d_n4, assign14560_e21520_d_n5, assign14560_e21520_d_n6, assign14560_e21520_d_n7, assign14560_e21520_d_n8, assign14560_e21520_d_n9, assign14560_e21520_d_n10, assign14560_e21520_d_n11, assign14560_e21520_d_n12, assign14560_e21520_d_n13, assign14560_e21520_d_n14,) = {
    if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
        let assign14560_e21516: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14560_e21518: f64 = (assign14560_e21516 * locals.var_assha);
        (assign14560_e21518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14560_e21520;
        locals.var_temp_aseff_dn0 = assign14560_e21520_d_n0;
        locals.var_temp_aseff_dn2 = assign14560_e21520_d_n2;
        locals.var_temp_aseff_dn3 = assign14560_e21520_d_n3;
        locals.var_temp_aseff_dn4 = assign14560_e21520_d_n4;
        locals.var_temp_aseff_dn5 = assign14560_e21520_d_n5;
        locals.var_temp_aseff_dn6 = assign14560_e21520_d_n6;
        locals.var_temp_aseff_dn7 = assign14560_e21520_d_n7;
        locals.var_temp_aseff_dn8 = assign14560_e21520_d_n8;
        locals.var_temp_aseff_dn9 = assign14560_e21520_d_n9;
        locals.var_temp_aseff_dn10 = assign14560_e21520_d_n10;
        locals.var_temp_aseff_dn11 = assign14560_e21520_d_n11;
        locals.var_temp_aseff_dn12 = assign14560_e21520_d_n12;
        locals.var_temp_aseff_dn13 = assign14560_e21520_d_n13;
        locals.var_temp_aseff_dn14 = assign14560_e21520_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14570_e21535, assign14570_e21535_d_n0, assign14570_e21535_d_n2, assign14570_e21535_d_n3, assign14570_e21535_d_n4, assign14570_e21535_d_n5, assign14570_e21535_d_n6, assign14570_e21535_d_n7, assign14570_e21535_d_n8, assign14570_e21535_d_n9, assign14570_e21535_d_n10, assign14570_e21535_d_n11, assign14570_e21535_d_n12, assign14570_e21535_d_n13, assign14570_e21535_d_n14,) = {
    if ((locals.var_guard467 != 0.0) && (!((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)))) {
        let assign14570_e21529: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14570_e21532: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14570_e21533: f64 = (assign14570_e21529 + assign14570_e21532);
        (assign14570_e21533, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14570_e21535;
        locals.var_temp_adeff_dn0 = assign14570_e21535_d_n0;
        locals.var_temp_adeff_dn2 = assign14570_e21535_d_n2;
        locals.var_temp_adeff_dn3 = assign14570_e21535_d_n3;
        locals.var_temp_adeff_dn4 = assign14570_e21535_d_n4;
        locals.var_temp_adeff_dn5 = assign14570_e21535_d_n5;
        locals.var_temp_adeff_dn6 = assign14570_e21535_d_n6;
        locals.var_temp_adeff_dn7 = assign14570_e21535_d_n7;
        locals.var_temp_adeff_dn8 = assign14570_e21535_d_n8;
        locals.var_temp_adeff_dn9 = assign14570_e21535_d_n9;
        locals.var_temp_adeff_dn10 = assign14570_e21535_d_n10;
        locals.var_temp_adeff_dn11 = assign14570_e21535_d_n11;
        locals.var_temp_adeff_dn12 = assign14570_e21535_d_n12;
        locals.var_temp_adeff_dn13 = assign14570_e21535_d_n13;
        locals.var_temp_adeff_dn14 = assign14570_e21535_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14580_e21550, assign14580_e21550_d_n0, assign14580_e21550_d_n2, assign14580_e21550_d_n3, assign14580_e21550_d_n4, assign14580_e21550_d_n5, assign14580_e21550_d_n6, assign14580_e21550_d_n7, assign14580_e21550_d_n8, assign14580_e21550_d_n9, assign14580_e21550_d_n10, assign14580_e21550_d_n11, assign14580_e21550_d_n12, assign14580_e21550_d_n13, assign14580_e21550_d_n14,) = {
    if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign14580_e21546: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14580_e21548: f64 = (assign14580_e21546 * locals.var_pssha);
        (assign14580_e21548, (assign14580_e21546 * locals.var_pssha_dn0), (assign14580_e21546 * locals.var_pssha_dn2), (assign14580_e21546 * locals.var_pssha_dn3), (assign14580_e21546 * locals.var_pssha_dn4), (assign14580_e21546 * locals.var_pssha_dn5), (assign14580_e21546 * locals.var_pssha_dn6), (assign14580_e21546 * locals.var_pssha_dn7), (assign14580_e21546 * locals.var_pssha_dn8), (assign14580_e21546 * locals.var_pssha_dn9), (assign14580_e21546 * locals.var_pssha_dn10), (assign14580_e21546 * locals.var_pssha_dn11), (assign14580_e21546 * locals.var_pssha_dn12), (assign14580_e21546 * locals.var_pssha_dn13), (assign14580_e21546 * locals.var_pssha_dn14),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14580_e21550;
        locals.var_temp_pseff_dn0 = assign14580_e21550_d_n0;
        locals.var_temp_pseff_dn2 = assign14580_e21550_d_n2;
        locals.var_temp_pseff_dn3 = assign14580_e21550_d_n3;
        locals.var_temp_pseff_dn4 = assign14580_e21550_d_n4;
        locals.var_temp_pseff_dn5 = assign14580_e21550_d_n5;
        locals.var_temp_pseff_dn6 = assign14580_e21550_d_n6;
        locals.var_temp_pseff_dn7 = assign14580_e21550_d_n7;
        locals.var_temp_pseff_dn8 = assign14580_e21550_d_n8;
        locals.var_temp_pseff_dn9 = assign14580_e21550_d_n9;
        locals.var_temp_pseff_dn10 = assign14580_e21550_d_n10;
        locals.var_temp_pseff_dn11 = assign14580_e21550_d_n11;
        locals.var_temp_pseff_dn12 = assign14580_e21550_d_n12;
        locals.var_temp_pseff_dn13 = assign14580_e21550_d_n13;
        locals.var_temp_pseff_dn14 = assign14580_e21550_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14590_e21565, assign14590_e21565_d_n0, assign14590_e21565_d_n2, assign14590_e21565_d_n3, assign14590_e21565_d_n4, assign14590_e21565_d_n5, assign14590_e21565_d_n6, assign14590_e21565_d_n7, assign14590_e21565_d_n8, assign14590_e21565_d_n9, assign14590_e21565_d_n10, assign14590_e21565_d_n11, assign14590_e21565_d_n12, assign14590_e21565_d_n13, assign14590_e21565_d_n14,) = {
    if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign14590_e21561: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14590_e21563: f64 = (assign14590_e21561 * locals.var_pdsha);
        (assign14590_e21563, (assign14590_e21561 * locals.var_pdsha_dn0), (assign14590_e21561 * locals.var_pdsha_dn2), (assign14590_e21561 * locals.var_pdsha_dn3), (assign14590_e21561 * locals.var_pdsha_dn4), (assign14590_e21561 * locals.var_pdsha_dn5), (assign14590_e21561 * locals.var_pdsha_dn6), (assign14590_e21561 * locals.var_pdsha_dn7), (assign14590_e21561 * locals.var_pdsha_dn8), (assign14590_e21561 * locals.var_pdsha_dn9), (assign14590_e21561 * locals.var_pdsha_dn10), (assign14590_e21561 * locals.var_pdsha_dn11), (assign14590_e21561 * locals.var_pdsha_dn12), (assign14590_e21561 * locals.var_pdsha_dn13), (assign14590_e21561 * locals.var_pdsha_dn14),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14590_e21565;
        locals.var_temp_pdeff_dn0 = assign14590_e21565_d_n0;
        locals.var_temp_pdeff_dn2 = assign14590_e21565_d_n2;
        locals.var_temp_pdeff_dn3 = assign14590_e21565_d_n3;
        locals.var_temp_pdeff_dn4 = assign14590_e21565_d_n4;
        locals.var_temp_pdeff_dn5 = assign14590_e21565_d_n5;
        locals.var_temp_pdeff_dn6 = assign14590_e21565_d_n6;
        locals.var_temp_pdeff_dn7 = assign14590_e21565_d_n7;
        locals.var_temp_pdeff_dn8 = assign14590_e21565_d_n8;
        locals.var_temp_pdeff_dn9 = assign14590_e21565_d_n9;
        locals.var_temp_pdeff_dn10 = assign14590_e21565_d_n10;
        locals.var_temp_pdeff_dn11 = assign14590_e21565_d_n11;
        locals.var_temp_pdeff_dn12 = assign14590_e21565_d_n12;
        locals.var_temp_pdeff_dn13 = assign14590_e21565_d_n13;
        locals.var_temp_pdeff_dn14 = assign14590_e21565_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14600_e21580, assign14600_e21580_d_n0, assign14600_e21580_d_n2, assign14600_e21580_d_n3, assign14600_e21580_d_n4, assign14600_e21580_d_n5, assign14600_e21580_d_n6, assign14600_e21580_d_n7, assign14600_e21580_d_n8, assign14600_e21580_d_n9, assign14600_e21580_d_n10, assign14600_e21580_d_n11, assign14600_e21580_d_n12, assign14600_e21580_d_n13, assign14600_e21580_d_n14,) = {
    if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign14600_e21576: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14600_e21578: f64 = (assign14600_e21576 * locals.var_assha);
        (assign14600_e21578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14600_e21580;
        locals.var_temp_aseff_dn0 = assign14600_e21580_d_n0;
        locals.var_temp_aseff_dn2 = assign14600_e21580_d_n2;
        locals.var_temp_aseff_dn3 = assign14600_e21580_d_n3;
        locals.var_temp_aseff_dn4 = assign14600_e21580_d_n4;
        locals.var_temp_aseff_dn5 = assign14600_e21580_d_n5;
        locals.var_temp_aseff_dn6 = assign14600_e21580_d_n6;
        locals.var_temp_aseff_dn7 = assign14600_e21580_d_n7;
        locals.var_temp_aseff_dn8 = assign14600_e21580_d_n8;
        locals.var_temp_aseff_dn9 = assign14600_e21580_d_n9;
        locals.var_temp_aseff_dn10 = assign14600_e21580_d_n10;
        locals.var_temp_aseff_dn11 = assign14600_e21580_d_n11;
        locals.var_temp_aseff_dn12 = assign14600_e21580_d_n12;
        locals.var_temp_aseff_dn13 = assign14600_e21580_d_n13;
        locals.var_temp_aseff_dn14 = assign14600_e21580_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14610_e21595, assign14610_e21595_d_n0, assign14610_e21595_d_n2, assign14610_e21595_d_n3, assign14610_e21595_d_n4, assign14610_e21595_d_n5, assign14610_e21595_d_n6, assign14610_e21595_d_n7, assign14610_e21595_d_n8, assign14610_e21595_d_n9, assign14610_e21595_d_n10, assign14610_e21595_d_n11, assign14610_e21595_d_n12, assign14610_e21595_d_n13, assign14610_e21595_d_n14,) = {
    if ((locals.var_guard468 != 0.0) && (!(((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)))) {
        let assign14610_e21591: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14610_e21593: f64 = (assign14610_e21591 * locals.var_adsha);
        (assign14610_e21593, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14610_e21595;
        locals.var_temp_adeff_dn0 = assign14610_e21595_d_n0;
        locals.var_temp_adeff_dn2 = assign14610_e21595_d_n2;
        locals.var_temp_adeff_dn3 = assign14610_e21595_d_n3;
        locals.var_temp_adeff_dn4 = assign14610_e21595_d_n4;
        locals.var_temp_adeff_dn5 = assign14610_e21595_d_n5;
        locals.var_temp_adeff_dn6 = assign14610_e21595_d_n6;
        locals.var_temp_adeff_dn7 = assign14610_e21595_d_n7;
        locals.var_temp_adeff_dn8 = assign14610_e21595_d_n8;
        locals.var_temp_adeff_dn9 = assign14610_e21595_d_n9;
        locals.var_temp_adeff_dn10 = assign14610_e21595_d_n10;
        locals.var_temp_adeff_dn11 = assign14610_e21595_d_n11;
        locals.var_temp_adeff_dn12 = assign14610_e21595_d_n12;
        locals.var_temp_adeff_dn13 = assign14610_e21595_d_n13;
        locals.var_temp_adeff_dn14 = assign14610_e21595_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14620_e21614, assign14620_e21614_d_n0, assign14620_e21614_d_n2, assign14620_e21614_d_n3, assign14620_e21614_d_n4, assign14620_e21614_d_n5, assign14620_e21614_d_n6, assign14620_e21614_d_n7, assign14620_e21614_d_n8, assign14620_e21614_d_n9, assign14620_e21614_d_n10, assign14620_e21614_d_n11, assign14620_e21614_d_n12, assign14620_e21614_d_n13, assign14620_e21614_d_n14,) = {
    if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign14620_e21608: f64 = (locals.var_nuends * locals.var_psiso);
        let assign14620_e21611: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14620_e21612: f64 = (assign14620_e21608 + assign14620_e21611);
        (assign14620_e21612, ((locals.var_nuends * locals.var_psiso_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psiso_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psiso_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psiso_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psiso_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psiso_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psiso_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psiso_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psiso_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psiso_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psiso_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psiso_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psiso_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psiso_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14620_e21614;
        locals.var_temp_pseff_dn0 = assign14620_e21614_d_n0;
        locals.var_temp_pseff_dn2 = assign14620_e21614_d_n2;
        locals.var_temp_pseff_dn3 = assign14620_e21614_d_n3;
        locals.var_temp_pseff_dn4 = assign14620_e21614_d_n4;
        locals.var_temp_pseff_dn5 = assign14620_e21614_d_n5;
        locals.var_temp_pseff_dn6 = assign14620_e21614_d_n6;
        locals.var_temp_pseff_dn7 = assign14620_e21614_d_n7;
        locals.var_temp_pseff_dn8 = assign14620_e21614_d_n8;
        locals.var_temp_pseff_dn9 = assign14620_e21614_d_n9;
        locals.var_temp_pseff_dn10 = assign14620_e21614_d_n10;
        locals.var_temp_pseff_dn11 = assign14620_e21614_d_n11;
        locals.var_temp_pseff_dn12 = assign14620_e21614_d_n12;
        locals.var_temp_pseff_dn13 = assign14620_e21614_d_n13;
        locals.var_temp_pseff_dn14 = assign14620_e21614_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14630_e21633, assign14630_e21633_d_n0, assign14630_e21633_d_n2, assign14630_e21633_d_n3, assign14630_e21633_d_n4, assign14630_e21633_d_n5, assign14630_e21633_d_n6, assign14630_e21633_d_n7, assign14630_e21633_d_n8, assign14630_e21633_d_n9, assign14630_e21633_d_n10, assign14630_e21633_d_n11, assign14630_e21633_d_n12, assign14630_e21633_d_n13, assign14630_e21633_d_n14,) = {
    if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign14630_e21627: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14630_e21630: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14630_e21631: f64 = (assign14630_e21627 + assign14630_e21630);
        (assign14630_e21631, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14630_e21633;
        locals.var_temp_pdeff_dn0 = assign14630_e21633_d_n0;
        locals.var_temp_pdeff_dn2 = assign14630_e21633_d_n2;
        locals.var_temp_pdeff_dn3 = assign14630_e21633_d_n3;
        locals.var_temp_pdeff_dn4 = assign14630_e21633_d_n4;
        locals.var_temp_pdeff_dn5 = assign14630_e21633_d_n5;
        locals.var_temp_pdeff_dn6 = assign14630_e21633_d_n6;
        locals.var_temp_pdeff_dn7 = assign14630_e21633_d_n7;
        locals.var_temp_pdeff_dn8 = assign14630_e21633_d_n8;
        locals.var_temp_pdeff_dn9 = assign14630_e21633_d_n9;
        locals.var_temp_pdeff_dn10 = assign14630_e21633_d_n10;
        locals.var_temp_pdeff_dn11 = assign14630_e21633_d_n11;
        locals.var_temp_pdeff_dn12 = assign14630_e21633_d_n12;
        locals.var_temp_pdeff_dn13 = assign14630_e21633_d_n13;
        locals.var_temp_pdeff_dn14 = assign14630_e21633_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14640_e21652, assign14640_e21652_d_n0, assign14640_e21652_d_n2, assign14640_e21652_d_n3, assign14640_e21652_d_n4, assign14640_e21652_d_n5, assign14640_e21652_d_n6, assign14640_e21652_d_n7, assign14640_e21652_d_n8, assign14640_e21652_d_n9, assign14640_e21652_d_n10, assign14640_e21652_d_n11, assign14640_e21652_d_n12, assign14640_e21652_d_n13, assign14640_e21652_d_n14,) = {
    if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign14640_e21646: f64 = (locals.var_nuends * locals.var_asiso);
        let assign14640_e21649: f64 = (locals.var_nuints * locals.var_assha);
        let assign14640_e21650: f64 = (assign14640_e21646 + assign14640_e21649);
        (assign14640_e21650, (locals.var_nuends * locals.var_asiso_dn0), (locals.var_nuends * locals.var_asiso_dn2), (locals.var_nuends * locals.var_asiso_dn3), (locals.var_nuends * locals.var_asiso_dn4), (locals.var_nuends * locals.var_asiso_dn5), (locals.var_nuends * locals.var_asiso_dn6), (locals.var_nuends * locals.var_asiso_dn7), (locals.var_nuends * locals.var_asiso_dn8), (locals.var_nuends * locals.var_asiso_dn9), (locals.var_nuends * locals.var_asiso_dn10), (locals.var_nuends * locals.var_asiso_dn11), (locals.var_nuends * locals.var_asiso_dn12), (locals.var_nuends * locals.var_asiso_dn13), (locals.var_nuends * locals.var_asiso_dn14),)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14640_e21652;
        locals.var_temp_aseff_dn0 = assign14640_e21652_d_n0;
        locals.var_temp_aseff_dn2 = assign14640_e21652_d_n2;
        locals.var_temp_aseff_dn3 = assign14640_e21652_d_n3;
        locals.var_temp_aseff_dn4 = assign14640_e21652_d_n4;
        locals.var_temp_aseff_dn5 = assign14640_e21652_d_n5;
        locals.var_temp_aseff_dn6 = assign14640_e21652_d_n6;
        locals.var_temp_aseff_dn7 = assign14640_e21652_d_n7;
        locals.var_temp_aseff_dn8 = assign14640_e21652_d_n8;
        locals.var_temp_aseff_dn9 = assign14640_e21652_d_n9;
        locals.var_temp_aseff_dn10 = assign14640_e21652_d_n10;
        locals.var_temp_aseff_dn11 = assign14640_e21652_d_n11;
        locals.var_temp_aseff_dn12 = assign14640_e21652_d_n12;
        locals.var_temp_aseff_dn13 = assign14640_e21652_d_n13;
        locals.var_temp_aseff_dn14 = assign14640_e21652_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14650_e21671, assign14650_e21671_d_n0, assign14650_e21671_d_n2, assign14650_e21671_d_n3, assign14650_e21671_d_n4, assign14650_e21671_d_n5, assign14650_e21671_d_n6, assign14650_e21671_d_n7, assign14650_e21671_d_n8, assign14650_e21671_d_n9, assign14650_e21671_d_n10, assign14650_e21671_d_n11, assign14650_e21671_d_n12, assign14650_e21671_d_n13, assign14650_e21671_d_n14,) = {
    if ((locals.var_guard469 != 0.0) && (!((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)))) {
        let assign14650_e21665: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14650_e21668: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14650_e21669: f64 = (assign14650_e21665 + assign14650_e21668);
        (assign14650_e21669, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14650_e21671;
        locals.var_temp_adeff_dn0 = assign14650_e21671_d_n0;
        locals.var_temp_adeff_dn2 = assign14650_e21671_d_n2;
        locals.var_temp_adeff_dn3 = assign14650_e21671_d_n3;
        locals.var_temp_adeff_dn4 = assign14650_e21671_d_n4;
        locals.var_temp_adeff_dn5 = assign14650_e21671_d_n5;
        locals.var_temp_adeff_dn6 = assign14650_e21671_d_n6;
        locals.var_temp_adeff_dn7 = assign14650_e21671_d_n7;
        locals.var_temp_adeff_dn8 = assign14650_e21671_d_n8;
        locals.var_temp_adeff_dn9 = assign14650_e21671_d_n9;
        locals.var_temp_adeff_dn10 = assign14650_e21671_d_n10;
        locals.var_temp_adeff_dn11 = assign14650_e21671_d_n11;
        locals.var_temp_adeff_dn12 = assign14650_e21671_d_n12;
        locals.var_temp_adeff_dn13 = assign14650_e21671_d_n13;
        locals.var_temp_adeff_dn14 = assign14650_e21671_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14660_e21690, assign14660_e21690_d_n0, assign14660_e21690_d_n2, assign14660_e21690_d_n3, assign14660_e21690_d_n4, assign14660_e21690_d_n5, assign14660_e21690_d_n6, assign14660_e21690_d_n7, assign14660_e21690_d_n8, assign14660_e21690_d_n9, assign14660_e21690_d_n10, assign14660_e21690_d_n11, assign14660_e21690_d_n12, assign14660_e21690_d_n13, assign14660_e21690_d_n14,) = {
    if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
        let assign14660_e21686: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14660_e21688: f64 = (assign14660_e21686 * locals.var_pssha);
        (assign14660_e21688, (assign14660_e21686 * locals.var_pssha_dn0), (assign14660_e21686 * locals.var_pssha_dn2), (assign14660_e21686 * locals.var_pssha_dn3), (assign14660_e21686 * locals.var_pssha_dn4), (assign14660_e21686 * locals.var_pssha_dn5), (assign14660_e21686 * locals.var_pssha_dn6), (assign14660_e21686 * locals.var_pssha_dn7), (assign14660_e21686 * locals.var_pssha_dn8), (assign14660_e21686 * locals.var_pssha_dn9), (assign14660_e21686 * locals.var_pssha_dn10), (assign14660_e21686 * locals.var_pssha_dn11), (assign14660_e21686 * locals.var_pssha_dn12), (assign14660_e21686 * locals.var_pssha_dn13), (assign14660_e21686 * locals.var_pssha_dn14),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14660_e21690;
        locals.var_temp_pseff_dn0 = assign14660_e21690_d_n0;
        locals.var_temp_pseff_dn2 = assign14660_e21690_d_n2;
        locals.var_temp_pseff_dn3 = assign14660_e21690_d_n3;
        locals.var_temp_pseff_dn4 = assign14660_e21690_d_n4;
        locals.var_temp_pseff_dn5 = assign14660_e21690_d_n5;
        locals.var_temp_pseff_dn6 = assign14660_e21690_d_n6;
        locals.var_temp_pseff_dn7 = assign14660_e21690_d_n7;
        locals.var_temp_pseff_dn8 = assign14660_e21690_d_n8;
        locals.var_temp_pseff_dn9 = assign14660_e21690_d_n9;
        locals.var_temp_pseff_dn10 = assign14660_e21690_d_n10;
        locals.var_temp_pseff_dn11 = assign14660_e21690_d_n11;
        locals.var_temp_pseff_dn12 = assign14660_e21690_d_n12;
        locals.var_temp_pseff_dn13 = assign14660_e21690_d_n13;
        locals.var_temp_pseff_dn14 = assign14660_e21690_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14670_e21711, assign14670_e21711_d_n0, assign14670_e21711_d_n2, assign14670_e21711_d_n3, assign14670_e21711_d_n4, assign14670_e21711_d_n5, assign14670_e21711_d_n6, assign14670_e21711_d_n7, assign14670_e21711_d_n8, assign14670_e21711_d_n9, assign14670_e21711_d_n10, assign14670_e21711_d_n11, assign14670_e21711_d_n12, assign14670_e21711_d_n13, assign14670_e21711_d_n14,) = {
    if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
        let assign14670_e21705: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14670_e21708: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14670_e21709: f64 = (assign14670_e21705 + assign14670_e21708);
        (assign14670_e21709, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14670_e21711;
        locals.var_temp_pdeff_dn0 = assign14670_e21711_d_n0;
        locals.var_temp_pdeff_dn2 = assign14670_e21711_d_n2;
        locals.var_temp_pdeff_dn3 = assign14670_e21711_d_n3;
        locals.var_temp_pdeff_dn4 = assign14670_e21711_d_n4;
        locals.var_temp_pdeff_dn5 = assign14670_e21711_d_n5;
        locals.var_temp_pdeff_dn6 = assign14670_e21711_d_n6;
        locals.var_temp_pdeff_dn7 = assign14670_e21711_d_n7;
        locals.var_temp_pdeff_dn8 = assign14670_e21711_d_n8;
        locals.var_temp_pdeff_dn9 = assign14670_e21711_d_n9;
        locals.var_temp_pdeff_dn10 = assign14670_e21711_d_n10;
        locals.var_temp_pdeff_dn11 = assign14670_e21711_d_n11;
        locals.var_temp_pdeff_dn12 = assign14670_e21711_d_n12;
        locals.var_temp_pdeff_dn13 = assign14670_e21711_d_n13;
        locals.var_temp_pdeff_dn14 = assign14670_e21711_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14680_e21730, assign14680_e21730_d_n0, assign14680_e21730_d_n2, assign14680_e21730_d_n3, assign14680_e21730_d_n4, assign14680_e21730_d_n5, assign14680_e21730_d_n6, assign14680_e21730_d_n7, assign14680_e21730_d_n8, assign14680_e21730_d_n9, assign14680_e21730_d_n10, assign14680_e21730_d_n11, assign14680_e21730_d_n12, assign14680_e21730_d_n13, assign14680_e21730_d_n14,) = {
    if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
        let assign14680_e21726: f64 = (locals.var_nuends + locals.var_nuints);
        let assign14680_e21728: f64 = (assign14680_e21726 * locals.var_assha);
        (assign14680_e21728, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14680_e21730;
        locals.var_temp_aseff_dn0 = assign14680_e21730_d_n0;
        locals.var_temp_aseff_dn2 = assign14680_e21730_d_n2;
        locals.var_temp_aseff_dn3 = assign14680_e21730_d_n3;
        locals.var_temp_aseff_dn4 = assign14680_e21730_d_n4;
        locals.var_temp_aseff_dn5 = assign14680_e21730_d_n5;
        locals.var_temp_aseff_dn6 = assign14680_e21730_d_n6;
        locals.var_temp_aseff_dn7 = assign14680_e21730_d_n7;
        locals.var_temp_aseff_dn8 = assign14680_e21730_d_n8;
        locals.var_temp_aseff_dn9 = assign14680_e21730_d_n9;
        locals.var_temp_aseff_dn10 = assign14680_e21730_d_n10;
        locals.var_temp_aseff_dn11 = assign14680_e21730_d_n11;
        locals.var_temp_aseff_dn12 = assign14680_e21730_d_n12;
        locals.var_temp_aseff_dn13 = assign14680_e21730_d_n13;
        locals.var_temp_aseff_dn14 = assign14680_e21730_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14690_e21751, assign14690_e21751_d_n0, assign14690_e21751_d_n2, assign14690_e21751_d_n3, assign14690_e21751_d_n4, assign14690_e21751_d_n5, assign14690_e21751_d_n6, assign14690_e21751_d_n7, assign14690_e21751_d_n8, assign14690_e21751_d_n9, assign14690_e21751_d_n10, assign14690_e21751_d_n11, assign14690_e21751_d_n12, assign14690_e21751_d_n13, assign14690_e21751_d_n14,) = {
    if ((locals.var_guard470 != 0.0) && (!(((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)))) {
        let assign14690_e21745: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14690_e21748: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14690_e21749: f64 = (assign14690_e21745 + assign14690_e21748);
        (assign14690_e21749, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14690_e21751;
        locals.var_temp_adeff_dn0 = assign14690_e21751_d_n0;
        locals.var_temp_adeff_dn2 = assign14690_e21751_d_n2;
        locals.var_temp_adeff_dn3 = assign14690_e21751_d_n3;
        locals.var_temp_adeff_dn4 = assign14690_e21751_d_n4;
        locals.var_temp_adeff_dn5 = assign14690_e21751_d_n5;
        locals.var_temp_adeff_dn6 = assign14690_e21751_d_n6;
        locals.var_temp_adeff_dn7 = assign14690_e21751_d_n7;
        locals.var_temp_adeff_dn8 = assign14690_e21751_d_n8;
        locals.var_temp_adeff_dn9 = assign14690_e21751_d_n9;
        locals.var_temp_adeff_dn10 = assign14690_e21751_d_n10;
        locals.var_temp_adeff_dn11 = assign14690_e21751_d_n11;
        locals.var_temp_adeff_dn12 = assign14690_e21751_d_n12;
        locals.var_temp_adeff_dn13 = assign14690_e21751_d_n13;
        locals.var_temp_adeff_dn14 = assign14690_e21751_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14700_e21774, assign14700_e21774_d_n0, assign14700_e21774_d_n2, assign14700_e21774_d_n3, assign14700_e21774_d_n4, assign14700_e21774_d_n5, assign14700_e21774_d_n6, assign14700_e21774_d_n7, assign14700_e21774_d_n8, assign14700_e21774_d_n9, assign14700_e21774_d_n10, assign14700_e21774_d_n11, assign14700_e21774_d_n12, assign14700_e21774_d_n13, assign14700_e21774_d_n14,) = {
    if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
        let assign14700_e21768: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14700_e21771: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14700_e21772: f64 = (assign14700_e21768 + assign14700_e21771);
        (assign14700_e21772, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14700_e21774;
        locals.var_temp_pseff_dn0 = assign14700_e21774_d_n0;
        locals.var_temp_pseff_dn2 = assign14700_e21774_d_n2;
        locals.var_temp_pseff_dn3 = assign14700_e21774_d_n3;
        locals.var_temp_pseff_dn4 = assign14700_e21774_d_n4;
        locals.var_temp_pseff_dn5 = assign14700_e21774_d_n5;
        locals.var_temp_pseff_dn6 = assign14700_e21774_d_n6;
        locals.var_temp_pseff_dn7 = assign14700_e21774_d_n7;
        locals.var_temp_pseff_dn8 = assign14700_e21774_d_n8;
        locals.var_temp_pseff_dn9 = assign14700_e21774_d_n9;
        locals.var_temp_pseff_dn10 = assign14700_e21774_d_n10;
        locals.var_temp_pseff_dn11 = assign14700_e21774_d_n11;
        locals.var_temp_pseff_dn12 = assign14700_e21774_d_n12;
        locals.var_temp_pseff_dn13 = assign14700_e21774_d_n13;
        locals.var_temp_pseff_dn14 = assign14700_e21774_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14710_e21797, assign14710_e21797_d_n0, assign14710_e21797_d_n2, assign14710_e21797_d_n3, assign14710_e21797_d_n4, assign14710_e21797_d_n5, assign14710_e21797_d_n6, assign14710_e21797_d_n7, assign14710_e21797_d_n8, assign14710_e21797_d_n9, assign14710_e21797_d_n10, assign14710_e21797_d_n11, assign14710_e21797_d_n12, assign14710_e21797_d_n13, assign14710_e21797_d_n14,) = {
    if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
        let assign14710_e21791: f64 = (locals.var_nuendd * locals.var_pdiso);
        let assign14710_e21794: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14710_e21795: f64 = (assign14710_e21791 + assign14710_e21794);
        (assign14710_e21795, ((locals.var_nuendd * locals.var_pdiso_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdiso_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdiso_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdiso_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdiso_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdiso_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdiso_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdiso_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdiso_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdiso_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdiso_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdiso_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdiso_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdiso_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14710_e21797;
        locals.var_temp_pdeff_dn0 = assign14710_e21797_d_n0;
        locals.var_temp_pdeff_dn2 = assign14710_e21797_d_n2;
        locals.var_temp_pdeff_dn3 = assign14710_e21797_d_n3;
        locals.var_temp_pdeff_dn4 = assign14710_e21797_d_n4;
        locals.var_temp_pdeff_dn5 = assign14710_e21797_d_n5;
        locals.var_temp_pdeff_dn6 = assign14710_e21797_d_n6;
        locals.var_temp_pdeff_dn7 = assign14710_e21797_d_n7;
        locals.var_temp_pdeff_dn8 = assign14710_e21797_d_n8;
        locals.var_temp_pdeff_dn9 = assign14710_e21797_d_n9;
        locals.var_temp_pdeff_dn10 = assign14710_e21797_d_n10;
        locals.var_temp_pdeff_dn11 = assign14710_e21797_d_n11;
        locals.var_temp_pdeff_dn12 = assign14710_e21797_d_n12;
        locals.var_temp_pdeff_dn13 = assign14710_e21797_d_n13;
        locals.var_temp_pdeff_dn14 = assign14710_e21797_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14720_e21820, assign14720_e21820_d_n0, assign14720_e21820_d_n2, assign14720_e21820_d_n3, assign14720_e21820_d_n4, assign14720_e21820_d_n5, assign14720_e21820_d_n6, assign14720_e21820_d_n7, assign14720_e21820_d_n8, assign14720_e21820_d_n9, assign14720_e21820_d_n10, assign14720_e21820_d_n11, assign14720_e21820_d_n12, assign14720_e21820_d_n13, assign14720_e21820_d_n14,) = {
    if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
        let assign14720_e21814: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14720_e21817: f64 = (locals.var_nuints * locals.var_assha);
        let assign14720_e21818: f64 = (assign14720_e21814 + assign14720_e21817);
        (assign14720_e21818, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14720_e21820;
        locals.var_temp_aseff_dn0 = assign14720_e21820_d_n0;
        locals.var_temp_aseff_dn2 = assign14720_e21820_d_n2;
        locals.var_temp_aseff_dn3 = assign14720_e21820_d_n3;
        locals.var_temp_aseff_dn4 = assign14720_e21820_d_n4;
        locals.var_temp_aseff_dn5 = assign14720_e21820_d_n5;
        locals.var_temp_aseff_dn6 = assign14720_e21820_d_n6;
        locals.var_temp_aseff_dn7 = assign14720_e21820_d_n7;
        locals.var_temp_aseff_dn8 = assign14720_e21820_d_n8;
        locals.var_temp_aseff_dn9 = assign14720_e21820_d_n9;
        locals.var_temp_aseff_dn10 = assign14720_e21820_d_n10;
        locals.var_temp_aseff_dn11 = assign14720_e21820_d_n11;
        locals.var_temp_aseff_dn12 = assign14720_e21820_d_n12;
        locals.var_temp_aseff_dn13 = assign14720_e21820_d_n13;
        locals.var_temp_aseff_dn14 = assign14720_e21820_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14730_e21843, assign14730_e21843_d_n0, assign14730_e21843_d_n2, assign14730_e21843_d_n3, assign14730_e21843_d_n4, assign14730_e21843_d_n5, assign14730_e21843_d_n6, assign14730_e21843_d_n7, assign14730_e21843_d_n8, assign14730_e21843_d_n9, assign14730_e21843_d_n10, assign14730_e21843_d_n11, assign14730_e21843_d_n12, assign14730_e21843_d_n13, assign14730_e21843_d_n14,) = {
    if ((locals.var_guard471 != 0.0) && (!((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)))) {
        let assign14730_e21837: f64 = (locals.var_nuendd * locals.var_adiso);
        let assign14730_e21840: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14730_e21841: f64 = (assign14730_e21837 + assign14730_e21840);
        (assign14730_e21841, (locals.var_nuendd * locals.var_adiso_dn0), (locals.var_nuendd * locals.var_adiso_dn2), (locals.var_nuendd * locals.var_adiso_dn3), (locals.var_nuendd * locals.var_adiso_dn4), (locals.var_nuendd * locals.var_adiso_dn5), (locals.var_nuendd * locals.var_adiso_dn6), (locals.var_nuendd * locals.var_adiso_dn7), (locals.var_nuendd * locals.var_adiso_dn8), (locals.var_nuendd * locals.var_adiso_dn9), (locals.var_nuendd * locals.var_adiso_dn10), (locals.var_nuendd * locals.var_adiso_dn11), (locals.var_nuendd * locals.var_adiso_dn12), (locals.var_nuendd * locals.var_adiso_dn13), (locals.var_nuendd * locals.var_adiso_dn14),)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14730_e21843;
        locals.var_temp_adeff_dn0 = assign14730_e21843_d_n0;
        locals.var_temp_adeff_dn2 = assign14730_e21843_d_n2;
        locals.var_temp_adeff_dn3 = assign14730_e21843_d_n3;
        locals.var_temp_adeff_dn4 = assign14730_e21843_d_n4;
        locals.var_temp_adeff_dn5 = assign14730_e21843_d_n5;
        locals.var_temp_adeff_dn6 = assign14730_e21843_d_n6;
        locals.var_temp_adeff_dn7 = assign14730_e21843_d_n7;
        locals.var_temp_adeff_dn8 = assign14730_e21843_d_n8;
        locals.var_temp_adeff_dn9 = assign14730_e21843_d_n9;
        locals.var_temp_adeff_dn10 = assign14730_e21843_d_n10;
        locals.var_temp_adeff_dn11 = assign14730_e21843_d_n11;
        locals.var_temp_adeff_dn12 = assign14730_e21843_d_n12;
        locals.var_temp_adeff_dn13 = assign14730_e21843_d_n13;
        locals.var_temp_adeff_dn14 = assign14730_e21843_d_n14;
        locals.var_temp_adeff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14740_e21868, assign14740_e21868_d_n0, assign14740_e21868_d_n2, assign14740_e21868_d_n3, assign14740_e21868_d_n4, assign14740_e21868_d_n5, assign14740_e21868_d_n6, assign14740_e21868_d_n7, assign14740_e21868_d_n8, assign14740_e21868_d_n9, assign14740_e21868_d_n10, assign14740_e21868_d_n11, assign14740_e21868_d_n12, assign14740_e21868_d_n13, assign14740_e21868_d_n14,) = {
    if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
        let assign14740_e21862: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14740_e21865: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14740_e21866: f64 = (assign14740_e21862 + assign14740_e21865);
        (assign14740_e21866, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14740_e21868;
        locals.var_temp_pseff_dn0 = assign14740_e21868_d_n0;
        locals.var_temp_pseff_dn2 = assign14740_e21868_d_n2;
        locals.var_temp_pseff_dn3 = assign14740_e21868_d_n3;
        locals.var_temp_pseff_dn4 = assign14740_e21868_d_n4;
        locals.var_temp_pseff_dn5 = assign14740_e21868_d_n5;
        locals.var_temp_pseff_dn6 = assign14740_e21868_d_n6;
        locals.var_temp_pseff_dn7 = assign14740_e21868_d_n7;
        locals.var_temp_pseff_dn8 = assign14740_e21868_d_n8;
        locals.var_temp_pseff_dn9 = assign14740_e21868_d_n9;
        locals.var_temp_pseff_dn10 = assign14740_e21868_d_n10;
        locals.var_temp_pseff_dn11 = assign14740_e21868_d_n11;
        locals.var_temp_pseff_dn12 = assign14740_e21868_d_n12;
        locals.var_temp_pseff_dn13 = assign14740_e21868_d_n13;
        locals.var_temp_pseff_dn14 = assign14740_e21868_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14750_e21891, assign14750_e21891_d_n0, assign14750_e21891_d_n2, assign14750_e21891_d_n3, assign14750_e21891_d_n4, assign14750_e21891_d_n5, assign14750_e21891_d_n6, assign14750_e21891_d_n7, assign14750_e21891_d_n8, assign14750_e21891_d_n9, assign14750_e21891_d_n10, assign14750_e21891_d_n11, assign14750_e21891_d_n12, assign14750_e21891_d_n13, assign14750_e21891_d_n14,) = {
    if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
        let assign14750_e21887: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14750_e21889: f64 = (assign14750_e21887 * locals.var_pdsha);
        (assign14750_e21889, (assign14750_e21887 * locals.var_pdsha_dn0), (assign14750_e21887 * locals.var_pdsha_dn2), (assign14750_e21887 * locals.var_pdsha_dn3), (assign14750_e21887 * locals.var_pdsha_dn4), (assign14750_e21887 * locals.var_pdsha_dn5), (assign14750_e21887 * locals.var_pdsha_dn6), (assign14750_e21887 * locals.var_pdsha_dn7), (assign14750_e21887 * locals.var_pdsha_dn8), (assign14750_e21887 * locals.var_pdsha_dn9), (assign14750_e21887 * locals.var_pdsha_dn10), (assign14750_e21887 * locals.var_pdsha_dn11), (assign14750_e21887 * locals.var_pdsha_dn12), (assign14750_e21887 * locals.var_pdsha_dn13), (assign14750_e21887 * locals.var_pdsha_dn14),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14750_e21891;
        locals.var_temp_pdeff_dn0 = assign14750_e21891_d_n0;
        locals.var_temp_pdeff_dn2 = assign14750_e21891_d_n2;
        locals.var_temp_pdeff_dn3 = assign14750_e21891_d_n3;
        locals.var_temp_pdeff_dn4 = assign14750_e21891_d_n4;
        locals.var_temp_pdeff_dn5 = assign14750_e21891_d_n5;
        locals.var_temp_pdeff_dn6 = assign14750_e21891_d_n6;
        locals.var_temp_pdeff_dn7 = assign14750_e21891_d_n7;
        locals.var_temp_pdeff_dn8 = assign14750_e21891_d_n8;
        locals.var_temp_pdeff_dn9 = assign14750_e21891_d_n9;
        locals.var_temp_pdeff_dn10 = assign14750_e21891_d_n10;
        locals.var_temp_pdeff_dn11 = assign14750_e21891_d_n11;
        locals.var_temp_pdeff_dn12 = assign14750_e21891_d_n12;
        locals.var_temp_pdeff_dn13 = assign14750_e21891_d_n13;
        locals.var_temp_pdeff_dn14 = assign14750_e21891_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14760_e21916, assign14760_e21916_d_n0, assign14760_e21916_d_n2, assign14760_e21916_d_n3, assign14760_e21916_d_n4, assign14760_e21916_d_n5, assign14760_e21916_d_n6, assign14760_e21916_d_n7, assign14760_e21916_d_n8, assign14760_e21916_d_n9, assign14760_e21916_d_n10, assign14760_e21916_d_n11, assign14760_e21916_d_n12, assign14760_e21916_d_n13, assign14760_e21916_d_n14,) = {
    if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
        let assign14760_e21910: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14760_e21913: f64 = (locals.var_nuints * locals.var_assha);
        let assign14760_e21914: f64 = (assign14760_e21910 + assign14760_e21913);
        (assign14760_e21914, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14760_e21916;
        locals.var_temp_aseff_dn0 = assign14760_e21916_d_n0;
        locals.var_temp_aseff_dn2 = assign14760_e21916_d_n2;
        locals.var_temp_aseff_dn3 = assign14760_e21916_d_n3;
        locals.var_temp_aseff_dn4 = assign14760_e21916_d_n4;
        locals.var_temp_aseff_dn5 = assign14760_e21916_d_n5;
        locals.var_temp_aseff_dn6 = assign14760_e21916_d_n6;
        locals.var_temp_aseff_dn7 = assign14760_e21916_d_n7;
        locals.var_temp_aseff_dn8 = assign14760_e21916_d_n8;
        locals.var_temp_aseff_dn9 = assign14760_e21916_d_n9;
        locals.var_temp_aseff_dn10 = assign14760_e21916_d_n10;
        locals.var_temp_aseff_dn11 = assign14760_e21916_d_n11;
        locals.var_temp_aseff_dn12 = assign14760_e21916_d_n12;
        locals.var_temp_aseff_dn13 = assign14760_e21916_d_n13;
        locals.var_temp_aseff_dn14 = assign14760_e21916_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14770_e21939, assign14770_e21939_d_n0, assign14770_e21939_d_n2, assign14770_e21939_d_n3, assign14770_e21939_d_n4, assign14770_e21939_d_n5, assign14770_e21939_d_n6, assign14770_e21939_d_n7, assign14770_e21939_d_n8, assign14770_e21939_d_n9, assign14770_e21939_d_n10, assign14770_e21939_d_n11, assign14770_e21939_d_n12, assign14770_e21939_d_n13, assign14770_e21939_d_n14,) = {
    if ((locals.var_guard472 != 0.0) && (!(((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)))) {
        let assign14770_e21935: f64 = (locals.var_nuendd + locals.var_nuintd);
        let assign14770_e21937: f64 = (assign14770_e21935 * locals.var_adsha);
        (assign14770_e21937, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14770_e21939;
        locals.var_temp_adeff_dn0 = assign14770_e21939_d_n0;
        locals.var_temp_adeff_dn2 = assign14770_e21939_d_n2;
        locals.var_temp_adeff_dn3 = assign14770_e21939_d_n3;
        locals.var_temp_adeff_dn4 = assign14770_e21939_d_n4;
        locals.var_temp_adeff_dn5 = assign14770_e21939_d_n5;
        locals.var_temp_adeff_dn6 = assign14770_e21939_d_n6;
        locals.var_temp_adeff_dn7 = assign14770_e21939_d_n7;
        locals.var_temp_adeff_dn8 = assign14770_e21939_d_n8;
        locals.var_temp_adeff_dn9 = assign14770_e21939_d_n9;
        locals.var_temp_adeff_dn10 = assign14770_e21939_d_n10;
        locals.var_temp_adeff_dn11 = assign14770_e21939_d_n11;
        locals.var_temp_adeff_dn12 = assign14770_e21939_d_n12;
        locals.var_temp_adeff_dn13 = assign14770_e21939_d_n13;
        locals.var_temp_adeff_dn14 = assign14770_e21939_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14780_e21966, assign14780_e21966_d_n0, assign14780_e21966_d_n2, assign14780_e21966_d_n3, assign14780_e21966_d_n4, assign14780_e21966_d_n5, assign14780_e21966_d_n6, assign14780_e21966_d_n7, assign14780_e21966_d_n8, assign14780_e21966_d_n9, assign14780_e21966_d_n10, assign14780_e21966_d_n11, assign14780_e21966_d_n12, assign14780_e21966_d_n13, assign14780_e21966_d_n14,) = {
    if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
        let assign14780_e21960: f64 = (locals.var_nuends * locals.var_psmer);
        let assign14780_e21963: f64 = (locals.var_nuints * locals.var_pssha);
        let assign14780_e21964: f64 = (assign14780_e21960 + assign14780_e21963);
        (assign14780_e21964, ((locals.var_nuends * locals.var_psmer_dn0) + (locals.var_nuints * locals.var_pssha_dn0)), ((locals.var_nuends * locals.var_psmer_dn2) + (locals.var_nuints * locals.var_pssha_dn2)), ((locals.var_nuends * locals.var_psmer_dn3) + (locals.var_nuints * locals.var_pssha_dn3)), ((locals.var_nuends * locals.var_psmer_dn4) + (locals.var_nuints * locals.var_pssha_dn4)), ((locals.var_nuends * locals.var_psmer_dn5) + (locals.var_nuints * locals.var_pssha_dn5)), ((locals.var_nuends * locals.var_psmer_dn6) + (locals.var_nuints * locals.var_pssha_dn6)), ((locals.var_nuends * locals.var_psmer_dn7) + (locals.var_nuints * locals.var_pssha_dn7)), ((locals.var_nuends * locals.var_psmer_dn8) + (locals.var_nuints * locals.var_pssha_dn8)), ((locals.var_nuends * locals.var_psmer_dn9) + (locals.var_nuints * locals.var_pssha_dn9)), ((locals.var_nuends * locals.var_psmer_dn10) + (locals.var_nuints * locals.var_pssha_dn10)), ((locals.var_nuends * locals.var_psmer_dn11) + (locals.var_nuints * locals.var_pssha_dn11)), ((locals.var_nuends * locals.var_psmer_dn12) + (locals.var_nuints * locals.var_pssha_dn12)), ((locals.var_nuends * locals.var_psmer_dn13) + (locals.var_nuints * locals.var_pssha_dn13)), ((locals.var_nuends * locals.var_psmer_dn14) + (locals.var_nuints * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14780_e21966;
        locals.var_temp_pseff_dn0 = assign14780_e21966_d_n0;
        locals.var_temp_pseff_dn2 = assign14780_e21966_d_n2;
        locals.var_temp_pseff_dn3 = assign14780_e21966_d_n3;
        locals.var_temp_pseff_dn4 = assign14780_e21966_d_n4;
        locals.var_temp_pseff_dn5 = assign14780_e21966_d_n5;
        locals.var_temp_pseff_dn6 = assign14780_e21966_d_n6;
        locals.var_temp_pseff_dn7 = assign14780_e21966_d_n7;
        locals.var_temp_pseff_dn8 = assign14780_e21966_d_n8;
        locals.var_temp_pseff_dn9 = assign14780_e21966_d_n9;
        locals.var_temp_pseff_dn10 = assign14780_e21966_d_n10;
        locals.var_temp_pseff_dn11 = assign14780_e21966_d_n11;
        locals.var_temp_pseff_dn12 = assign14780_e21966_d_n12;
        locals.var_temp_pseff_dn13 = assign14780_e21966_d_n13;
        locals.var_temp_pseff_dn14 = assign14780_e21966_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14790_e21993, assign14790_e21993_d_n0, assign14790_e21993_d_n2, assign14790_e21993_d_n3, assign14790_e21993_d_n4, assign14790_e21993_d_n5, assign14790_e21993_d_n6, assign14790_e21993_d_n7, assign14790_e21993_d_n8, assign14790_e21993_d_n9, assign14790_e21993_d_n10, assign14790_e21993_d_n11, assign14790_e21993_d_n12, assign14790_e21993_d_n13, assign14790_e21993_d_n14,) = {
    if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
        let assign14790_e21987: f64 = (locals.var_nuendd * locals.var_pdmer);
        let assign14790_e21990: f64 = (locals.var_nuintd * locals.var_pdsha);
        let assign14790_e21991: f64 = (assign14790_e21987 + assign14790_e21990);
        (assign14790_e21991, ((locals.var_nuendd * locals.var_pdmer_dn0) + (locals.var_nuintd * locals.var_pdsha_dn0)), ((locals.var_nuendd * locals.var_pdmer_dn2) + (locals.var_nuintd * locals.var_pdsha_dn2)), ((locals.var_nuendd * locals.var_pdmer_dn3) + (locals.var_nuintd * locals.var_pdsha_dn3)), ((locals.var_nuendd * locals.var_pdmer_dn4) + (locals.var_nuintd * locals.var_pdsha_dn4)), ((locals.var_nuendd * locals.var_pdmer_dn5) + (locals.var_nuintd * locals.var_pdsha_dn5)), ((locals.var_nuendd * locals.var_pdmer_dn6) + (locals.var_nuintd * locals.var_pdsha_dn6)), ((locals.var_nuendd * locals.var_pdmer_dn7) + (locals.var_nuintd * locals.var_pdsha_dn7)), ((locals.var_nuendd * locals.var_pdmer_dn8) + (locals.var_nuintd * locals.var_pdsha_dn8)), ((locals.var_nuendd * locals.var_pdmer_dn9) + (locals.var_nuintd * locals.var_pdsha_dn9)), ((locals.var_nuendd * locals.var_pdmer_dn10) + (locals.var_nuintd * locals.var_pdsha_dn10)), ((locals.var_nuendd * locals.var_pdmer_dn11) + (locals.var_nuintd * locals.var_pdsha_dn11)), ((locals.var_nuendd * locals.var_pdmer_dn12) + (locals.var_nuintd * locals.var_pdsha_dn12)), ((locals.var_nuendd * locals.var_pdmer_dn13) + (locals.var_nuintd * locals.var_pdsha_dn13)), ((locals.var_nuendd * locals.var_pdmer_dn14) + (locals.var_nuintd * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14790_e21993;
        locals.var_temp_pdeff_dn0 = assign14790_e21993_d_n0;
        locals.var_temp_pdeff_dn2 = assign14790_e21993_d_n2;
        locals.var_temp_pdeff_dn3 = assign14790_e21993_d_n3;
        locals.var_temp_pdeff_dn4 = assign14790_e21993_d_n4;
        locals.var_temp_pdeff_dn5 = assign14790_e21993_d_n5;
        locals.var_temp_pdeff_dn6 = assign14790_e21993_d_n6;
        locals.var_temp_pdeff_dn7 = assign14790_e21993_d_n7;
        locals.var_temp_pdeff_dn8 = assign14790_e21993_d_n8;
        locals.var_temp_pdeff_dn9 = assign14790_e21993_d_n9;
        locals.var_temp_pdeff_dn10 = assign14790_e21993_d_n10;
        locals.var_temp_pdeff_dn11 = assign14790_e21993_d_n11;
        locals.var_temp_pdeff_dn12 = assign14790_e21993_d_n12;
        locals.var_temp_pdeff_dn13 = assign14790_e21993_d_n13;
        locals.var_temp_pdeff_dn14 = assign14790_e21993_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14800_e22020, assign14800_e22020_d_n0, assign14800_e22020_d_n2, assign14800_e22020_d_n3, assign14800_e22020_d_n4, assign14800_e22020_d_n5, assign14800_e22020_d_n6, assign14800_e22020_d_n7, assign14800_e22020_d_n8, assign14800_e22020_d_n9, assign14800_e22020_d_n10, assign14800_e22020_d_n11, assign14800_e22020_d_n12, assign14800_e22020_d_n13, assign14800_e22020_d_n14,) = {
    if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
        let assign14800_e22014: f64 = (locals.var_nuends * locals.var_asmer);
        let assign14800_e22017: f64 = (locals.var_nuints * locals.var_assha);
        let assign14800_e22018: f64 = (assign14800_e22014 + assign14800_e22017);
        (assign14800_e22018, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14800_e22020;
        locals.var_temp_aseff_dn0 = assign14800_e22020_d_n0;
        locals.var_temp_aseff_dn2 = assign14800_e22020_d_n2;
        locals.var_temp_aseff_dn3 = assign14800_e22020_d_n3;
        locals.var_temp_aseff_dn4 = assign14800_e22020_d_n4;
        locals.var_temp_aseff_dn5 = assign14800_e22020_d_n5;
        locals.var_temp_aseff_dn6 = assign14800_e22020_d_n6;
        locals.var_temp_aseff_dn7 = assign14800_e22020_d_n7;
        locals.var_temp_aseff_dn8 = assign14800_e22020_d_n8;
        locals.var_temp_aseff_dn9 = assign14800_e22020_d_n9;
        locals.var_temp_aseff_dn10 = assign14800_e22020_d_n10;
        locals.var_temp_aseff_dn11 = assign14800_e22020_d_n11;
        locals.var_temp_aseff_dn12 = assign14800_e22020_d_n12;
        locals.var_temp_aseff_dn13 = assign14800_e22020_d_n13;
        locals.var_temp_aseff_dn14 = assign14800_e22020_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14810_e22047, assign14810_e22047_d_n0, assign14810_e22047_d_n2, assign14810_e22047_d_n3, assign14810_e22047_d_n4, assign14810_e22047_d_n5, assign14810_e22047_d_n6, assign14810_e22047_d_n7, assign14810_e22047_d_n8, assign14810_e22047_d_n9, assign14810_e22047_d_n10, assign14810_e22047_d_n11, assign14810_e22047_d_n12, assign14810_e22047_d_n13, assign14810_e22047_d_n14,) = {
    if ((locals.var_guard473 != 0.0) && (!((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)))) {
        let assign14810_e22041: f64 = (locals.var_nuendd * locals.var_admer);
        let assign14810_e22044: f64 = (locals.var_nuintd * locals.var_adsha);
        let assign14810_e22045: f64 = (assign14810_e22041 + assign14810_e22044);
        (assign14810_e22045, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14810_e22047;
        locals.var_temp_adeff_dn0 = assign14810_e22047_d_n0;
        locals.var_temp_adeff_dn2 = assign14810_e22047_d_n2;
        locals.var_temp_adeff_dn3 = assign14810_e22047_d_n3;
        locals.var_temp_adeff_dn4 = assign14810_e22047_d_n4;
        locals.var_temp_adeff_dn5 = assign14810_e22047_d_n5;
        locals.var_temp_adeff_dn6 = assign14810_e22047_d_n6;
        locals.var_temp_adeff_dn7 = assign14810_e22047_d_n7;
        locals.var_temp_adeff_dn8 = assign14810_e22047_d_n8;
        locals.var_temp_adeff_dn9 = assign14810_e22047_d_n9;
        locals.var_temp_adeff_dn10 = assign14810_e22047_d_n10;
        locals.var_temp_adeff_dn11 = assign14810_e22047_d_n11;
        locals.var_temp_adeff_dn12 = assign14810_e22047_d_n12;
        locals.var_temp_adeff_dn13 = assign14810_e22047_d_n13;
        locals.var_temp_adeff_dn14 = assign14810_e22047_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14820_e22076, assign14820_e22076_d_n0, assign14820_e22076_d_n2, assign14820_e22076_d_n3, assign14820_e22076_d_n4, assign14820_e22076_d_n5, assign14820_e22076_d_n6, assign14820_e22076_d_n7, assign14820_e22076_d_n8, assign14820_e22076_d_n9, assign14820_e22076_d_n10, assign14820_e22076_d_n11, assign14820_e22076_d_n12, assign14820_e22076_d_n13, assign14820_e22076_d_n14,) = {
    if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
        let assign14820_e22071: f64 = (p.p2 - 1.0);
        let assign14820_e22073: f64 = (assign14820_e22071 * locals.var_pssha);
        let assign14820_e22074: f64 = (locals.var_psiso + assign14820_e22073);
        (assign14820_e22074, (locals.var_psiso_dn0 + (assign14820_e22071 * locals.var_pssha_dn0)), (locals.var_psiso_dn2 + (assign14820_e22071 * locals.var_pssha_dn2)), (locals.var_psiso_dn3 + (assign14820_e22071 * locals.var_pssha_dn3)), (locals.var_psiso_dn4 + (assign14820_e22071 * locals.var_pssha_dn4)), (locals.var_psiso_dn5 + (assign14820_e22071 * locals.var_pssha_dn5)), (locals.var_psiso_dn6 + (assign14820_e22071 * locals.var_pssha_dn6)), (locals.var_psiso_dn7 + (assign14820_e22071 * locals.var_pssha_dn7)), (locals.var_psiso_dn8 + (assign14820_e22071 * locals.var_pssha_dn8)), (locals.var_psiso_dn9 + (assign14820_e22071 * locals.var_pssha_dn9)), (locals.var_psiso_dn10 + (assign14820_e22071 * locals.var_pssha_dn10)), (locals.var_psiso_dn11 + (assign14820_e22071 * locals.var_pssha_dn11)), (locals.var_psiso_dn12 + (assign14820_e22071 * locals.var_pssha_dn12)), (locals.var_psiso_dn13 + (assign14820_e22071 * locals.var_pssha_dn13)), (locals.var_psiso_dn14 + (assign14820_e22071 * locals.var_pssha_dn14)),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14820_e22076;
        locals.var_temp_pseff_dn0 = assign14820_e22076_d_n0;
        locals.var_temp_pseff_dn2 = assign14820_e22076_d_n2;
        locals.var_temp_pseff_dn3 = assign14820_e22076_d_n3;
        locals.var_temp_pseff_dn4 = assign14820_e22076_d_n4;
        locals.var_temp_pseff_dn5 = assign14820_e22076_d_n5;
        locals.var_temp_pseff_dn6 = assign14820_e22076_d_n6;
        locals.var_temp_pseff_dn7 = assign14820_e22076_d_n7;
        locals.var_temp_pseff_dn8 = assign14820_e22076_d_n8;
        locals.var_temp_pseff_dn9 = assign14820_e22076_d_n9;
        locals.var_temp_pseff_dn10 = assign14820_e22076_d_n10;
        locals.var_temp_pseff_dn11 = assign14820_e22076_d_n11;
        locals.var_temp_pseff_dn12 = assign14820_e22076_d_n12;
        locals.var_temp_pseff_dn13 = assign14820_e22076_d_n13;
        locals.var_temp_pseff_dn14 = assign14820_e22076_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14830_e22101, assign14830_e22101_d_n0, assign14830_e22101_d_n2, assign14830_e22101_d_n3, assign14830_e22101_d_n4, assign14830_e22101_d_n5, assign14830_e22101_d_n6, assign14830_e22101_d_n7, assign14830_e22101_d_n8, assign14830_e22101_d_n9, assign14830_e22101_d_n10, assign14830_e22101_d_n11, assign14830_e22101_d_n12, assign14830_e22101_d_n13, assign14830_e22101_d_n14,) = {
    if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
        let assign14830_e22099: f64 = (p.p2 * locals.var_pdsha);
        (assign14830_e22099, (p.p2 * locals.var_pdsha_dn0), (p.p2 * locals.var_pdsha_dn2), (p.p2 * locals.var_pdsha_dn3), (p.p2 * locals.var_pdsha_dn4), (p.p2 * locals.var_pdsha_dn5), (p.p2 * locals.var_pdsha_dn6), (p.p2 * locals.var_pdsha_dn7), (p.p2 * locals.var_pdsha_dn8), (p.p2 * locals.var_pdsha_dn9), (p.p2 * locals.var_pdsha_dn10), (p.p2 * locals.var_pdsha_dn11), (p.p2 * locals.var_pdsha_dn12), (p.p2 * locals.var_pdsha_dn13), (p.p2 * locals.var_pdsha_dn14),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14830_e22101;
        locals.var_temp_pdeff_dn0 = assign14830_e22101_d_n0;
        locals.var_temp_pdeff_dn2 = assign14830_e22101_d_n2;
        locals.var_temp_pdeff_dn3 = assign14830_e22101_d_n3;
        locals.var_temp_pdeff_dn4 = assign14830_e22101_d_n4;
        locals.var_temp_pdeff_dn5 = assign14830_e22101_d_n5;
        locals.var_temp_pdeff_dn6 = assign14830_e22101_d_n6;
        locals.var_temp_pdeff_dn7 = assign14830_e22101_d_n7;
        locals.var_temp_pdeff_dn8 = assign14830_e22101_d_n8;
        locals.var_temp_pdeff_dn9 = assign14830_e22101_d_n9;
        locals.var_temp_pdeff_dn10 = assign14830_e22101_d_n10;
        locals.var_temp_pdeff_dn11 = assign14830_e22101_d_n11;
        locals.var_temp_pdeff_dn12 = assign14830_e22101_d_n12;
        locals.var_temp_pdeff_dn13 = assign14830_e22101_d_n13;
        locals.var_temp_pdeff_dn14 = assign14830_e22101_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14840_e22130, assign14840_e22130_d_n0, assign14840_e22130_d_n2, assign14840_e22130_d_n3, assign14840_e22130_d_n4, assign14840_e22130_d_n5, assign14840_e22130_d_n6, assign14840_e22130_d_n7, assign14840_e22130_d_n8, assign14840_e22130_d_n9, assign14840_e22130_d_n10, assign14840_e22130_d_n11, assign14840_e22130_d_n12, assign14840_e22130_d_n13, assign14840_e22130_d_n14,) = {
    if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
        let assign14840_e22125: f64 = (p.p2 - 1.0);
        let assign14840_e22127: f64 = (assign14840_e22125 * locals.var_assha);
        let assign14840_e22128: f64 = (locals.var_asiso + assign14840_e22127);
        (assign14840_e22128, locals.var_asiso_dn0, locals.var_asiso_dn2, locals.var_asiso_dn3, locals.var_asiso_dn4, locals.var_asiso_dn5, locals.var_asiso_dn6, locals.var_asiso_dn7, locals.var_asiso_dn8, locals.var_asiso_dn9, locals.var_asiso_dn10, locals.var_asiso_dn11, locals.var_asiso_dn12, locals.var_asiso_dn13, locals.var_asiso_dn14,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14840_e22130;
        locals.var_temp_aseff_dn0 = assign14840_e22130_d_n0;
        locals.var_temp_aseff_dn2 = assign14840_e22130_d_n2;
        locals.var_temp_aseff_dn3 = assign14840_e22130_d_n3;
        locals.var_temp_aseff_dn4 = assign14840_e22130_d_n4;
        locals.var_temp_aseff_dn5 = assign14840_e22130_d_n5;
        locals.var_temp_aseff_dn6 = assign14840_e22130_d_n6;
        locals.var_temp_aseff_dn7 = assign14840_e22130_d_n7;
        locals.var_temp_aseff_dn8 = assign14840_e22130_d_n8;
        locals.var_temp_aseff_dn9 = assign14840_e22130_d_n9;
        locals.var_temp_aseff_dn10 = assign14840_e22130_d_n10;
        locals.var_temp_aseff_dn11 = assign14840_e22130_d_n11;
        locals.var_temp_aseff_dn12 = assign14840_e22130_d_n12;
        locals.var_temp_aseff_dn13 = assign14840_e22130_d_n13;
        locals.var_temp_aseff_dn14 = assign14840_e22130_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14850_e22155, assign14850_e22155_d_n0, assign14850_e22155_d_n2, assign14850_e22155_d_n3, assign14850_e22155_d_n4, assign14850_e22155_d_n5, assign14850_e22155_d_n6, assign14850_e22155_d_n7, assign14850_e22155_d_n8, assign14850_e22155_d_n9, assign14850_e22155_d_n10, assign14850_e22155_d_n11, assign14850_e22155_d_n12, assign14850_e22155_d_n13, assign14850_e22155_d_n14,) = {
    if ((locals.var_guard474 != 0.0) && (!(((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)))) {
        let assign14850_e22153: f64 = (p.p2 * locals.var_adsha);
        (assign14850_e22153, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14850_e22155;
        locals.var_temp_adeff_dn0 = assign14850_e22155_d_n0;
        locals.var_temp_adeff_dn2 = assign14850_e22155_d_n2;
        locals.var_temp_adeff_dn3 = assign14850_e22155_d_n3;
        locals.var_temp_adeff_dn4 = assign14850_e22155_d_n4;
        locals.var_temp_adeff_dn5 = assign14850_e22155_d_n5;
        locals.var_temp_adeff_dn6 = assign14850_e22155_d_n6;
        locals.var_temp_adeff_dn7 = assign14850_e22155_d_n7;
        locals.var_temp_adeff_dn8 = assign14850_e22155_d_n8;
        locals.var_temp_adeff_dn9 = assign14850_e22155_d_n9;
        locals.var_temp_adeff_dn10 = assign14850_e22155_d_n10;
        locals.var_temp_adeff_dn11 = assign14850_e22155_d_n11;
        locals.var_temp_adeff_dn12 = assign14850_e22155_d_n12;
        locals.var_temp_adeff_dn13 = assign14850_e22155_d_n13;
        locals.var_temp_adeff_dn14 = assign14850_e22155_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14860_e22182, assign14860_e22182_d_n0, assign14860_e22182_d_n2, assign14860_e22182_d_n3, assign14860_e22182_d_n4, assign14860_e22182_d_n5, assign14860_e22182_d_n6, assign14860_e22182_d_n7, assign14860_e22182_d_n8, assign14860_e22182_d_n9, assign14860_e22182_d_n10, assign14860_e22182_d_n11, assign14860_e22182_d_n12, assign14860_e22182_d_n13, assign14860_e22182_d_n14,) = {
    if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
        let assign14860_e22180: f64 = (p.p2 * locals.var_pssha);
        (assign14860_e22180, (p.p2 * locals.var_pssha_dn0), (p.p2 * locals.var_pssha_dn2), (p.p2 * locals.var_pssha_dn3), (p.p2 * locals.var_pssha_dn4), (p.p2 * locals.var_pssha_dn5), (p.p2 * locals.var_pssha_dn6), (p.p2 * locals.var_pssha_dn7), (p.p2 * locals.var_pssha_dn8), (p.p2 * locals.var_pssha_dn9), (p.p2 * locals.var_pssha_dn10), (p.p2 * locals.var_pssha_dn11), (p.p2 * locals.var_pssha_dn12), (p.p2 * locals.var_pssha_dn13), (p.p2 * locals.var_pssha_dn14),)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14860_e22182;
        locals.var_temp_pseff_dn0 = assign14860_e22182_d_n0;
        locals.var_temp_pseff_dn2 = assign14860_e22182_d_n2;
        locals.var_temp_pseff_dn3 = assign14860_e22182_d_n3;
        locals.var_temp_pseff_dn4 = assign14860_e22182_d_n4;
        locals.var_temp_pseff_dn5 = assign14860_e22182_d_n5;
        locals.var_temp_pseff_dn6 = assign14860_e22182_d_n6;
        locals.var_temp_pseff_dn7 = assign14860_e22182_d_n7;
        locals.var_temp_pseff_dn8 = assign14860_e22182_d_n8;
        locals.var_temp_pseff_dn9 = assign14860_e22182_d_n9;
        locals.var_temp_pseff_dn10 = assign14860_e22182_d_n10;
        locals.var_temp_pseff_dn11 = assign14860_e22182_d_n11;
        locals.var_temp_pseff_dn12 = assign14860_e22182_d_n12;
        locals.var_temp_pseff_dn13 = assign14860_e22182_d_n13;
        locals.var_temp_pseff_dn14 = assign14860_e22182_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14870_e22213, assign14870_e22213_d_n0, assign14870_e22213_d_n2, assign14870_e22213_d_n3, assign14870_e22213_d_n4, assign14870_e22213_d_n5, assign14870_e22213_d_n6, assign14870_e22213_d_n7, assign14870_e22213_d_n8, assign14870_e22213_d_n9, assign14870_e22213_d_n10, assign14870_e22213_d_n11, assign14870_e22213_d_n12, assign14870_e22213_d_n13, assign14870_e22213_d_n14,) = {
    if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
        let assign14870_e22208: f64 = (p.p2 - 1.0);
        let assign14870_e22210: f64 = (assign14870_e22208 * locals.var_pdsha);
        let assign14870_e22211: f64 = (locals.var_pdiso + assign14870_e22210);
        (assign14870_e22211, (locals.var_pdiso_dn0 + (assign14870_e22208 * locals.var_pdsha_dn0)), (locals.var_pdiso_dn2 + (assign14870_e22208 * locals.var_pdsha_dn2)), (locals.var_pdiso_dn3 + (assign14870_e22208 * locals.var_pdsha_dn3)), (locals.var_pdiso_dn4 + (assign14870_e22208 * locals.var_pdsha_dn4)), (locals.var_pdiso_dn5 + (assign14870_e22208 * locals.var_pdsha_dn5)), (locals.var_pdiso_dn6 + (assign14870_e22208 * locals.var_pdsha_dn6)), (locals.var_pdiso_dn7 + (assign14870_e22208 * locals.var_pdsha_dn7)), (locals.var_pdiso_dn8 + (assign14870_e22208 * locals.var_pdsha_dn8)), (locals.var_pdiso_dn9 + (assign14870_e22208 * locals.var_pdsha_dn9)), (locals.var_pdiso_dn10 + (assign14870_e22208 * locals.var_pdsha_dn10)), (locals.var_pdiso_dn11 + (assign14870_e22208 * locals.var_pdsha_dn11)), (locals.var_pdiso_dn12 + (assign14870_e22208 * locals.var_pdsha_dn12)), (locals.var_pdiso_dn13 + (assign14870_e22208 * locals.var_pdsha_dn13)), (locals.var_pdiso_dn14 + (assign14870_e22208 * locals.var_pdsha_dn14)),)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14870_e22213;
        locals.var_temp_pdeff_dn0 = assign14870_e22213_d_n0;
        locals.var_temp_pdeff_dn2 = assign14870_e22213_d_n2;
        locals.var_temp_pdeff_dn3 = assign14870_e22213_d_n3;
        locals.var_temp_pdeff_dn4 = assign14870_e22213_d_n4;
        locals.var_temp_pdeff_dn5 = assign14870_e22213_d_n5;
        locals.var_temp_pdeff_dn6 = assign14870_e22213_d_n6;
        locals.var_temp_pdeff_dn7 = assign14870_e22213_d_n7;
        locals.var_temp_pdeff_dn8 = assign14870_e22213_d_n8;
        locals.var_temp_pdeff_dn9 = assign14870_e22213_d_n9;
        locals.var_temp_pdeff_dn10 = assign14870_e22213_d_n10;
        locals.var_temp_pdeff_dn11 = assign14870_e22213_d_n11;
        locals.var_temp_pdeff_dn12 = assign14870_e22213_d_n12;
        locals.var_temp_pdeff_dn13 = assign14870_e22213_d_n13;
        locals.var_temp_pdeff_dn14 = assign14870_e22213_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14880_e22240, assign14880_e22240_d_n0, assign14880_e22240_d_n2, assign14880_e22240_d_n3, assign14880_e22240_d_n4, assign14880_e22240_d_n5, assign14880_e22240_d_n6, assign14880_e22240_d_n7, assign14880_e22240_d_n8, assign14880_e22240_d_n9, assign14880_e22240_d_n10, assign14880_e22240_d_n11, assign14880_e22240_d_n12, assign14880_e22240_d_n13, assign14880_e22240_d_n14,) = {
    if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
        let assign14880_e22238: f64 = (p.p2 * locals.var_assha);
        (assign14880_e22238, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14880_e22240;
        locals.var_temp_aseff_dn0 = assign14880_e22240_d_n0;
        locals.var_temp_aseff_dn2 = assign14880_e22240_d_n2;
        locals.var_temp_aseff_dn3 = assign14880_e22240_d_n3;
        locals.var_temp_aseff_dn4 = assign14880_e22240_d_n4;
        locals.var_temp_aseff_dn5 = assign14880_e22240_d_n5;
        locals.var_temp_aseff_dn6 = assign14880_e22240_d_n6;
        locals.var_temp_aseff_dn7 = assign14880_e22240_d_n7;
        locals.var_temp_aseff_dn8 = assign14880_e22240_d_n8;
        locals.var_temp_aseff_dn9 = assign14880_e22240_d_n9;
        locals.var_temp_aseff_dn10 = assign14880_e22240_d_n10;
        locals.var_temp_aseff_dn11 = assign14880_e22240_d_n11;
        locals.var_temp_aseff_dn12 = assign14880_e22240_d_n12;
        locals.var_temp_aseff_dn13 = assign14880_e22240_d_n13;
        locals.var_temp_aseff_dn14 = assign14880_e22240_d_n14;
        locals.var_temp_aseff_rv = 0.0;

        let (assign14890_e22271, assign14890_e22271_d_n0, assign14890_e22271_d_n2, assign14890_e22271_d_n3, assign14890_e22271_d_n4, assign14890_e22271_d_n5, assign14890_e22271_d_n6, assign14890_e22271_d_n7, assign14890_e22271_d_n8, assign14890_e22271_d_n9, assign14890_e22271_d_n10, assign14890_e22271_d_n11, assign14890_e22271_d_n12, assign14890_e22271_d_n13, assign14890_e22271_d_n14,) = {
    if ((locals.var_guard475 != 0.0) && (!((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)))) {
        let assign14890_e22266: f64 = (p.p2 - 1.0);
        let assign14890_e22268: f64 = (assign14890_e22266 * locals.var_adsha);
        let assign14890_e22269: f64 = (locals.var_adiso + assign14890_e22268);
        (assign14890_e22269, locals.var_adiso_dn0, locals.var_adiso_dn2, locals.var_adiso_dn3, locals.var_adiso_dn4, locals.var_adiso_dn5, locals.var_adiso_dn6, locals.var_adiso_dn7, locals.var_adiso_dn8, locals.var_adiso_dn9, locals.var_adiso_dn10, locals.var_adiso_dn11, locals.var_adiso_dn12, locals.var_adiso_dn13, locals.var_adiso_dn14,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14890_e22271;
        locals.var_temp_adeff_dn0 = assign14890_e22271_d_n0;
        locals.var_temp_adeff_dn2 = assign14890_e22271_d_n2;
        locals.var_temp_adeff_dn3 = assign14890_e22271_d_n3;
        locals.var_temp_adeff_dn4 = assign14890_e22271_d_n4;
        locals.var_temp_adeff_dn5 = assign14890_e22271_d_n5;
        locals.var_temp_adeff_dn6 = assign14890_e22271_d_n6;
        locals.var_temp_adeff_dn7 = assign14890_e22271_d_n7;
        locals.var_temp_adeff_dn8 = assign14890_e22271_d_n8;
        locals.var_temp_adeff_dn9 = assign14890_e22271_d_n9;
        locals.var_temp_adeff_dn10 = assign14890_e22271_d_n10;
        locals.var_temp_adeff_dn11 = assign14890_e22271_d_n11;
        locals.var_temp_adeff_dn12 = assign14890_e22271_d_n12;
        locals.var_temp_adeff_dn13 = assign14890_e22271_d_n13;
        locals.var_temp_adeff_dn14 = assign14890_e22271_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let (assign14900_e22296, assign14900_e22296_d_n0, assign14900_e22296_d_n2, assign14900_e22296_d_n3, assign14900_e22296_d_n4, assign14900_e22296_d_n5, assign14900_e22296_d_n6, assign14900_e22296_d_n7, assign14900_e22296_d_n8, assign14900_e22296_d_n9, assign14900_e22296_d_n10, assign14900_e22296_d_n11, assign14900_e22296_d_n12, assign14900_e22296_d_n13, assign14900_e22296_d_n14,) = {
    if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    }
};
        locals.var_temp_pseff = assign14900_e22296;
        locals.var_temp_pseff_dn0 = assign14900_e22296_d_n0;
        locals.var_temp_pseff_dn2 = assign14900_e22296_d_n2;
        locals.var_temp_pseff_dn3 = assign14900_e22296_d_n3;
        locals.var_temp_pseff_dn4 = assign14900_e22296_d_n4;
        locals.var_temp_pseff_dn5 = assign14900_e22296_d_n5;
        locals.var_temp_pseff_dn6 = assign14900_e22296_d_n6;
        locals.var_temp_pseff_dn7 = assign14900_e22296_d_n7;
        locals.var_temp_pseff_dn8 = assign14900_e22296_d_n8;
        locals.var_temp_pseff_dn9 = assign14900_e22296_d_n9;
        locals.var_temp_pseff_dn10 = assign14900_e22296_d_n10;
        locals.var_temp_pseff_dn11 = assign14900_e22296_d_n11;
        locals.var_temp_pseff_dn12 = assign14900_e22296_d_n12;
        locals.var_temp_pseff_dn13 = assign14900_e22296_d_n13;
        locals.var_temp_pseff_dn14 = assign14900_e22296_d_n14;
        locals.var_temp_pseff_rv = 0.0;

        let (assign14910_e22321, assign14910_e22321_d_n0, assign14910_e22321_d_n2, assign14910_e22321_d_n3, assign14910_e22321_d_n4, assign14910_e22321_d_n5, assign14910_e22321_d_n6, assign14910_e22321_d_n7, assign14910_e22321_d_n8, assign14910_e22321_d_n9, assign14910_e22321_d_n10, assign14910_e22321_d_n11, assign14910_e22321_d_n12, assign14910_e22321_d_n13, assign14910_e22321_d_n14,) = {
    if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    }
};
        locals.var_temp_pdeff = assign14910_e22321;
        locals.var_temp_pdeff_dn0 = assign14910_e22321_d_n0;
        locals.var_temp_pdeff_dn2 = assign14910_e22321_d_n2;
        locals.var_temp_pdeff_dn3 = assign14910_e22321_d_n3;
        locals.var_temp_pdeff_dn4 = assign14910_e22321_d_n4;
        locals.var_temp_pdeff_dn5 = assign14910_e22321_d_n5;
        locals.var_temp_pdeff_dn6 = assign14910_e22321_d_n6;
        locals.var_temp_pdeff_dn7 = assign14910_e22321_d_n7;
        locals.var_temp_pdeff_dn8 = assign14910_e22321_d_n8;
        locals.var_temp_pdeff_dn9 = assign14910_e22321_d_n9;
        locals.var_temp_pdeff_dn10 = assign14910_e22321_d_n10;
        locals.var_temp_pdeff_dn11 = assign14910_e22321_d_n11;
        locals.var_temp_pdeff_dn12 = assign14910_e22321_d_n12;
        locals.var_temp_pdeff_dn13 = assign14910_e22321_d_n13;
        locals.var_temp_pdeff_dn14 = assign14910_e22321_d_n14;
        locals.var_temp_pdeff_rv = 0.0;

        let (assign14920_e22346, assign14920_e22346_d_n0, assign14920_e22346_d_n2, assign14920_e22346_d_n3, assign14920_e22346_d_n4, assign14920_e22346_d_n5, assign14920_e22346_d_n6, assign14920_e22346_d_n7, assign14920_e22346_d_n8, assign14920_e22346_d_n9, assign14920_e22346_d_n10, assign14920_e22346_d_n11, assign14920_e22346_d_n12, assign14920_e22346_d_n13, assign14920_e22346_d_n14,) = {
    if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    }
};
        locals.var_temp_aseff = assign14920_e22346;
        locals.var_temp_aseff_dn0 = assign14920_e22346_d_n0;
        locals.var_temp_aseff_dn2 = assign14920_e22346_d_n2;
        locals.var_temp_aseff_dn3 = assign14920_e22346_d_n3;
        locals.var_temp_aseff_dn4 = assign14920_e22346_d_n4;
        locals.var_temp_aseff_dn5 = assign14920_e22346_d_n5;
        locals.var_temp_aseff_dn6 = assign14920_e22346_d_n6;
        locals.var_temp_aseff_dn7 = assign14920_e22346_d_n7;
        locals.var_temp_aseff_dn8 = assign14920_e22346_d_n8;
        locals.var_temp_aseff_dn9 = assign14920_e22346_d_n9;
        locals.var_temp_aseff_dn10 = assign14920_e22346_d_n10;
        locals.var_temp_aseff_dn11 = assign14920_e22346_d_n11;
        locals.var_temp_aseff_dn12 = assign14920_e22346_d_n12;
        locals.var_temp_aseff_dn13 = assign14920_e22346_d_n13;
        locals.var_temp_aseff_dn14 = assign14920_e22346_d_n14;
        locals.var_temp_aseff_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_31(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign14930_e22371, assign14930_e22371_d_n0, assign14930_e22371_d_n2, assign14930_e22371_d_n3, assign14930_e22371_d_n4, assign14930_e22371_d_n5, assign14930_e22371_d_n6, assign14930_e22371_d_n7, assign14930_e22371_d_n8, assign14930_e22371_d_n9, assign14930_e22371_d_n10, assign14930_e22371_d_n11, assign14930_e22371_d_n12, assign14930_e22371_d_n13, assign14930_e22371_d_n14,) = {
    if (!(((((((((((locals.var_guard465 != 0.0) || (locals.var_guard466 != 0.0)) || (locals.var_guard467 != 0.0)) || (locals.var_guard468 != 0.0)) || (locals.var_guard469 != 0.0)) || (locals.var_guard470 != 0.0)) || (locals.var_guard471 != 0.0)) || (locals.var_guard472 != 0.0)) || (locals.var_guard473 != 0.0)) || (locals.var_guard474 != 0.0)) || (locals.var_guard475 != 0.0))) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    }
};
        locals.var_temp_adeff = assign14930_e22371;
        locals.var_temp_adeff_dn0 = assign14930_e22371_d_n0;
        locals.var_temp_adeff_dn2 = assign14930_e22371_d_n2;
        locals.var_temp_adeff_dn3 = assign14930_e22371_d_n3;
        locals.var_temp_adeff_dn4 = assign14930_e22371_d_n4;
        locals.var_temp_adeff_dn5 = assign14930_e22371_d_n5;
        locals.var_temp_adeff_dn6 = assign14930_e22371_d_n6;
        locals.var_temp_adeff_dn7 = assign14930_e22371_d_n7;
        locals.var_temp_adeff_dn8 = assign14930_e22371_d_n8;
        locals.var_temp_adeff_dn9 = assign14930_e22371_d_n9;
        locals.var_temp_adeff_dn10 = assign14930_e22371_d_n10;
        locals.var_temp_adeff_dn11 = assign14930_e22371_d_n11;
        locals.var_temp_adeff_dn12 = assign14930_e22371_d_n12;
        locals.var_temp_adeff_dn13 = assign14930_e22371_d_n13;
        locals.var_temp_adeff_dn14 = assign14930_e22371_d_n14;
        locals.var_temp_adeff_rv = 0.0;

        let assign14940_e22373: f64 = if param_given[24] { 1.0 } else { 0.0 };
        locals.var_guard476 = assign14940_e22373;
        locals.var_guard476_rv = 0.0;

        let (assign14950_e22381, assign14950_e22381_d_n0, assign14950_e22381_d_n2, assign14950_e22381_d_n3, assign14950_e22381_d_n4, assign14950_e22381_d_n5, assign14950_e22381_d_n6, assign14950_e22381_d_n7, assign14950_e22381_d_n8, assign14950_e22381_d_n9, assign14950_e22381_d_n10, assign14950_e22381_d_n11, assign14950_e22381_d_n12, assign14950_e22381_d_n13, assign14950_e22381_d_n14,) = {
    if (locals.var_guard476 != 0.0) {
        let assign14950_e22377: f64 = (p.p24 * p.p53);
        let assign14950_e22379: f64 = (assign14950_e22377 * p.p52);
        (assign14950_e22379, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14,)
    }
};
        locals.var_aseff = assign14950_e22381;
        locals.var_aseff_dn0 = assign14950_e22381_d_n0;
        locals.var_aseff_dn2 = assign14950_e22381_d_n2;
        locals.var_aseff_dn3 = assign14950_e22381_d_n3;
        locals.var_aseff_dn4 = assign14950_e22381_d_n4;
        locals.var_aseff_dn5 = assign14950_e22381_d_n5;
        locals.var_aseff_dn6 = assign14950_e22381_d_n6;
        locals.var_aseff_dn7 = assign14950_e22381_d_n7;
        locals.var_aseff_dn8 = assign14950_e22381_d_n8;
        locals.var_aseff_dn9 = assign14950_e22381_d_n9;
        locals.var_aseff_dn10 = assign14950_e22381_d_n10;
        locals.var_aseff_dn11 = assign14950_e22381_d_n11;
        locals.var_aseff_dn12 = assign14950_e22381_d_n12;
        locals.var_aseff_dn13 = assign14950_e22381_d_n13;
        locals.var_aseff_dn14 = assign14950_e22381_d_n14;
        locals.var_aseff_rv = 0.0;

        let (assign14960_e22386, assign14960_e22386_d_n0, assign14960_e22386_d_n2, assign14960_e22386_d_n3, assign14960_e22386_d_n4, assign14960_e22386_d_n5, assign14960_e22386_d_n6, assign14960_e22386_d_n7, assign14960_e22386_d_n8, assign14960_e22386_d_n9, assign14960_e22386_d_n10, assign14960_e22386_d_n11, assign14960_e22386_d_n12, assign14960_e22386_d_n13, assign14960_e22386_d_n14,) = {
    if (locals.var_guard476 == 0.0) {
        (locals.var_temp_aseff, locals.var_temp_aseff_dn0, locals.var_temp_aseff_dn2, locals.var_temp_aseff_dn3, locals.var_temp_aseff_dn4, locals.var_temp_aseff_dn5, locals.var_temp_aseff_dn6, locals.var_temp_aseff_dn7, locals.var_temp_aseff_dn8, locals.var_temp_aseff_dn9, locals.var_temp_aseff_dn10, locals.var_temp_aseff_dn11, locals.var_temp_aseff_dn12, locals.var_temp_aseff_dn13, locals.var_temp_aseff_dn14,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14,)
    }
};
        locals.var_aseff = assign14960_e22386;
        locals.var_aseff_dn0 = assign14960_e22386_d_n0;
        locals.var_aseff_dn2 = assign14960_e22386_d_n2;
        locals.var_aseff_dn3 = assign14960_e22386_d_n3;
        locals.var_aseff_dn4 = assign14960_e22386_d_n4;
        locals.var_aseff_dn5 = assign14960_e22386_d_n5;
        locals.var_aseff_dn6 = assign14960_e22386_d_n6;
        locals.var_aseff_dn7 = assign14960_e22386_d_n7;
        locals.var_aseff_dn8 = assign14960_e22386_d_n8;
        locals.var_aseff_dn9 = assign14960_e22386_d_n9;
        locals.var_aseff_dn10 = assign14960_e22386_d_n10;
        locals.var_aseff_dn11 = assign14960_e22386_d_n11;
        locals.var_aseff_dn12 = assign14960_e22386_d_n12;
        locals.var_aseff_dn13 = assign14960_e22386_d_n13;
        locals.var_aseff_dn14 = assign14960_e22386_d_n14;
        locals.var_aseff_rv = 0.0;

        let assign14970_e22389: f64 = if locals.var_aseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign14970_e22389;
        locals.var_guard477_rv = 0.0;

        let (assign14980_e22393, assign14980_e22393_d_n0, assign14980_e22393_d_n2, assign14980_e22393_d_n3, assign14980_e22393_d_n4, assign14980_e22393_d_n5, assign14980_e22393_d_n6, assign14980_e22393_d_n7, assign14980_e22393_d_n8, assign14980_e22393_d_n9, assign14980_e22393_d_n10, assign14980_e22393_d_n11, assign14980_e22393_d_n12, assign14980_e22393_d_n13, assign14980_e22393_d_n14,) = {
    if (locals.var_guard477 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_aseff, locals.var_aseff_dn0, locals.var_aseff_dn2, locals.var_aseff_dn3, locals.var_aseff_dn4, locals.var_aseff_dn5, locals.var_aseff_dn6, locals.var_aseff_dn7, locals.var_aseff_dn8, locals.var_aseff_dn9, locals.var_aseff_dn10, locals.var_aseff_dn11, locals.var_aseff_dn12, locals.var_aseff_dn13, locals.var_aseff_dn14,)
    }
};
        locals.var_aseff = assign14980_e22393;
        locals.var_aseff_dn0 = assign14980_e22393_d_n0;
        locals.var_aseff_dn2 = assign14980_e22393_d_n2;
        locals.var_aseff_dn3 = assign14980_e22393_d_n3;
        locals.var_aseff_dn4 = assign14980_e22393_d_n4;
        locals.var_aseff_dn5 = assign14980_e22393_d_n5;
        locals.var_aseff_dn6 = assign14980_e22393_d_n6;
        locals.var_aseff_dn7 = assign14980_e22393_d_n7;
        locals.var_aseff_dn8 = assign14980_e22393_d_n8;
        locals.var_aseff_dn9 = assign14980_e22393_d_n9;
        locals.var_aseff_dn10 = assign14980_e22393_d_n10;
        locals.var_aseff_dn11 = assign14980_e22393_d_n11;
        locals.var_aseff_dn12 = assign14980_e22393_d_n12;
        locals.var_aseff_dn13 = assign14980_e22393_d_n13;
        locals.var_aseff_dn14 = assign14980_e22393_d_n14;
        locals.var_aseff_rv = 0.0;

        let assign14990_e22395: f64 = if param_given[25] { 1.0 } else { 0.0 };
        locals.var_guard478 = assign14990_e22395;
        locals.var_guard478_rv = 0.0;

        let (assign15000_e22403, assign15000_e22403_d_n0, assign15000_e22403_d_n2, assign15000_e22403_d_n3, assign15000_e22403_d_n4, assign15000_e22403_d_n5, assign15000_e22403_d_n6, assign15000_e22403_d_n7, assign15000_e22403_d_n8, assign15000_e22403_d_n9, assign15000_e22403_d_n10, assign15000_e22403_d_n11, assign15000_e22403_d_n12, assign15000_e22403_d_n13, assign15000_e22403_d_n14,) = {
    if (locals.var_guard478 != 0.0) {
        let assign15000_e22399: f64 = (p.p25 * p.p53);
        let assign15000_e22401: f64 = (assign15000_e22399 * p.p52);
        (assign15000_e22401, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14,)
    }
};
        locals.var_adeff = assign15000_e22403;
        locals.var_adeff_dn0 = assign15000_e22403_d_n0;
        locals.var_adeff_dn2 = assign15000_e22403_d_n2;
        locals.var_adeff_dn3 = assign15000_e22403_d_n3;
        locals.var_adeff_dn4 = assign15000_e22403_d_n4;
        locals.var_adeff_dn5 = assign15000_e22403_d_n5;
        locals.var_adeff_dn6 = assign15000_e22403_d_n6;
        locals.var_adeff_dn7 = assign15000_e22403_d_n7;
        locals.var_adeff_dn8 = assign15000_e22403_d_n8;
        locals.var_adeff_dn9 = assign15000_e22403_d_n9;
        locals.var_adeff_dn10 = assign15000_e22403_d_n10;
        locals.var_adeff_dn11 = assign15000_e22403_d_n11;
        locals.var_adeff_dn12 = assign15000_e22403_d_n12;
        locals.var_adeff_dn13 = assign15000_e22403_d_n13;
        locals.var_adeff_dn14 = assign15000_e22403_d_n14;
        locals.var_adeff_rv = 0.0;

        let (assign15010_e22408, assign15010_e22408_d_n0, assign15010_e22408_d_n2, assign15010_e22408_d_n3, assign15010_e22408_d_n4, assign15010_e22408_d_n5, assign15010_e22408_d_n6, assign15010_e22408_d_n7, assign15010_e22408_d_n8, assign15010_e22408_d_n9, assign15010_e22408_d_n10, assign15010_e22408_d_n11, assign15010_e22408_d_n12, assign15010_e22408_d_n13, assign15010_e22408_d_n14,) = {
    if (locals.var_guard478 == 0.0) {
        (locals.var_temp_adeff, locals.var_temp_adeff_dn0, locals.var_temp_adeff_dn2, locals.var_temp_adeff_dn3, locals.var_temp_adeff_dn4, locals.var_temp_adeff_dn5, locals.var_temp_adeff_dn6, locals.var_temp_adeff_dn7, locals.var_temp_adeff_dn8, locals.var_temp_adeff_dn9, locals.var_temp_adeff_dn10, locals.var_temp_adeff_dn11, locals.var_temp_adeff_dn12, locals.var_temp_adeff_dn13, locals.var_temp_adeff_dn14,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14,)
    }
};
        locals.var_adeff = assign15010_e22408;
        locals.var_adeff_dn0 = assign15010_e22408_d_n0;
        locals.var_adeff_dn2 = assign15010_e22408_d_n2;
        locals.var_adeff_dn3 = assign15010_e22408_d_n3;
        locals.var_adeff_dn4 = assign15010_e22408_d_n4;
        locals.var_adeff_dn5 = assign15010_e22408_d_n5;
        locals.var_adeff_dn6 = assign15010_e22408_d_n6;
        locals.var_adeff_dn7 = assign15010_e22408_d_n7;
        locals.var_adeff_dn8 = assign15010_e22408_d_n8;
        locals.var_adeff_dn9 = assign15010_e22408_d_n9;
        locals.var_adeff_dn10 = assign15010_e22408_d_n10;
        locals.var_adeff_dn11 = assign15010_e22408_d_n11;
        locals.var_adeff_dn12 = assign15010_e22408_d_n12;
        locals.var_adeff_dn13 = assign15010_e22408_d_n13;
        locals.var_adeff_dn14 = assign15010_e22408_d_n14;
        locals.var_adeff_rv = 0.0;

        let assign15020_e22411: f64 = if locals.var_adeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard479 = assign15020_e22411;
        locals.var_guard479_rv = 0.0;

        let (assign15030_e22415, assign15030_e22415_d_n0, assign15030_e22415_d_n2, assign15030_e22415_d_n3, assign15030_e22415_d_n4, assign15030_e22415_d_n5, assign15030_e22415_d_n6, assign15030_e22415_d_n7, assign15030_e22415_d_n8, assign15030_e22415_d_n9, assign15030_e22415_d_n10, assign15030_e22415_d_n11, assign15030_e22415_d_n12, assign15030_e22415_d_n13, assign15030_e22415_d_n14,) = {
    if (locals.var_guard479 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_adeff, locals.var_adeff_dn0, locals.var_adeff_dn2, locals.var_adeff_dn3, locals.var_adeff_dn4, locals.var_adeff_dn5, locals.var_adeff_dn6, locals.var_adeff_dn7, locals.var_adeff_dn8, locals.var_adeff_dn9, locals.var_adeff_dn10, locals.var_adeff_dn11, locals.var_adeff_dn12, locals.var_adeff_dn13, locals.var_adeff_dn14,)
    }
};
        locals.var_adeff = assign15030_e22415;
        locals.var_adeff_dn0 = assign15030_e22415_d_n0;
        locals.var_adeff_dn2 = assign15030_e22415_d_n2;
        locals.var_adeff_dn3 = assign15030_e22415_d_n3;
        locals.var_adeff_dn4 = assign15030_e22415_d_n4;
        locals.var_adeff_dn5 = assign15030_e22415_d_n5;
        locals.var_adeff_dn6 = assign15030_e22415_d_n6;
        locals.var_adeff_dn7 = assign15030_e22415_d_n7;
        locals.var_adeff_dn8 = assign15030_e22415_d_n8;
        locals.var_adeff_dn9 = assign15030_e22415_d_n9;
        locals.var_adeff_dn10 = assign15030_e22415_d_n10;
        locals.var_adeff_dn11 = assign15030_e22415_d_n11;
        locals.var_adeff_dn12 = assign15030_e22415_d_n12;
        locals.var_adeff_dn13 = assign15030_e22415_d_n13;
        locals.var_adeff_dn14 = assign15030_e22415_d_n14;
        locals.var_adeff_rv = 0.0;

        let assign15040_e22417: f64 = if param_given[26] { 1.0 } else { 0.0 };
        locals.var_guard480 = assign15040_e22417;
        locals.var_guard480_rv = 0.0;

        let assign15050_e22420: f64 = if p.p137 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard481 = assign15050_e22420;
        locals.var_guard481_rv = 0.0;

        let (assign15060_e22428, assign15060_e22428_d_n0, assign15060_e22428_d_n2, assign15060_e22428_d_n3, assign15060_e22428_d_n4, assign15060_e22428_d_n5, assign15060_e22428_d_n6, assign15060_e22428_d_n7, assign15060_e22428_d_n8, assign15060_e22428_d_n9, assign15060_e22428_d_n10, assign15060_e22428_d_n11, assign15060_e22428_d_n12, assign15060_e22428_d_n13, assign15060_e22428_d_n14,) = {
    if ((locals.var_guard480 != 0.0) && (locals.var_guard481 != 0.0)) {
        let assign15060_e22426: f64 = (p.p26 * p.p53);
        (assign15060_e22426, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14,)
    }
};
        locals.var_pseff = assign15060_e22428;
        locals.var_pseff_dn0 = assign15060_e22428_d_n0;
        locals.var_pseff_dn2 = assign15060_e22428_d_n2;
        locals.var_pseff_dn3 = assign15060_e22428_d_n3;
        locals.var_pseff_dn4 = assign15060_e22428_d_n4;
        locals.var_pseff_dn5 = assign15060_e22428_d_n5;
        locals.var_pseff_dn6 = assign15060_e22428_d_n6;
        locals.var_pseff_dn7 = assign15060_e22428_d_n7;
        locals.var_pseff_dn8 = assign15060_e22428_d_n8;
        locals.var_pseff_dn9 = assign15060_e22428_d_n9;
        locals.var_pseff_dn10 = assign15060_e22428_d_n10;
        locals.var_pseff_dn11 = assign15060_e22428_d_n11;
        locals.var_pseff_dn12 = assign15060_e22428_d_n12;
        locals.var_pseff_dn13 = assign15060_e22428_d_n13;
        locals.var_pseff_dn14 = assign15060_e22428_d_n14;
        locals.var_pseff_rv = 0.0;

        let (assign15070_e22443, assign15070_e22443_d_n0, assign15070_e22443_d_n2, assign15070_e22443_d_n3, assign15070_e22443_d_n4, assign15070_e22443_d_n5, assign15070_e22443_d_n6, assign15070_e22443_d_n7, assign15070_e22443_d_n8, assign15070_e22443_d_n9, assign15070_e22443_d_n10, assign15070_e22443_d_n11, assign15070_e22443_d_n12, assign15070_e22443_d_n13, assign15070_e22443_d_n14,) = {
    if ((locals.var_guard480 != 0.0) && (locals.var_guard481 == 0.0)) {
        let assign15070_e22435: f64 = (p.p26 * p.p53);
        let assign15070_e22438: f64 = (locals.var_weffcj * p.p2);
        let assign15070_e22439: f64 = (assign15070_e22435 - assign15070_e22438);
        let assign15070_e22441: f64 = (assign15070_e22439).max(0.0);
        (assign15070_e22441, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14,)
    }
};
        locals.var_pseff = assign15070_e22443;
        locals.var_pseff_dn0 = assign15070_e22443_d_n0;
        locals.var_pseff_dn2 = assign15070_e22443_d_n2;
        locals.var_pseff_dn3 = assign15070_e22443_d_n3;
        locals.var_pseff_dn4 = assign15070_e22443_d_n4;
        locals.var_pseff_dn5 = assign15070_e22443_d_n5;
        locals.var_pseff_dn6 = assign15070_e22443_d_n6;
        locals.var_pseff_dn7 = assign15070_e22443_d_n7;
        locals.var_pseff_dn8 = assign15070_e22443_d_n8;
        locals.var_pseff_dn9 = assign15070_e22443_d_n9;
        locals.var_pseff_dn10 = assign15070_e22443_d_n10;
        locals.var_pseff_dn11 = assign15070_e22443_d_n11;
        locals.var_pseff_dn12 = assign15070_e22443_d_n12;
        locals.var_pseff_dn13 = assign15070_e22443_d_n13;
        locals.var_pseff_dn14 = assign15070_e22443_d_n14;
        locals.var_pseff_rv = 0.0;

        let (assign15080_e22448, assign15080_e22448_d_n0, assign15080_e22448_d_n2, assign15080_e22448_d_n3, assign15080_e22448_d_n4, assign15080_e22448_d_n5, assign15080_e22448_d_n6, assign15080_e22448_d_n7, assign15080_e22448_d_n8, assign15080_e22448_d_n9, assign15080_e22448_d_n10, assign15080_e22448_d_n11, assign15080_e22448_d_n12, assign15080_e22448_d_n13, assign15080_e22448_d_n14,) = {
    if (locals.var_guard480 == 0.0) {
        (locals.var_temp_pseff, locals.var_temp_pseff_dn0, locals.var_temp_pseff_dn2, locals.var_temp_pseff_dn3, locals.var_temp_pseff_dn4, locals.var_temp_pseff_dn5, locals.var_temp_pseff_dn6, locals.var_temp_pseff_dn7, locals.var_temp_pseff_dn8, locals.var_temp_pseff_dn9, locals.var_temp_pseff_dn10, locals.var_temp_pseff_dn11, locals.var_temp_pseff_dn12, locals.var_temp_pseff_dn13, locals.var_temp_pseff_dn14,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14,)
    }
};
        locals.var_pseff = assign15080_e22448;
        locals.var_pseff_dn0 = assign15080_e22448_d_n0;
        locals.var_pseff_dn2 = assign15080_e22448_d_n2;
        locals.var_pseff_dn3 = assign15080_e22448_d_n3;
        locals.var_pseff_dn4 = assign15080_e22448_d_n4;
        locals.var_pseff_dn5 = assign15080_e22448_d_n5;
        locals.var_pseff_dn6 = assign15080_e22448_d_n6;
        locals.var_pseff_dn7 = assign15080_e22448_d_n7;
        locals.var_pseff_dn8 = assign15080_e22448_d_n8;
        locals.var_pseff_dn9 = assign15080_e22448_d_n9;
        locals.var_pseff_dn10 = assign15080_e22448_d_n10;
        locals.var_pseff_dn11 = assign15080_e22448_d_n11;
        locals.var_pseff_dn12 = assign15080_e22448_d_n12;
        locals.var_pseff_dn13 = assign15080_e22448_d_n13;
        locals.var_pseff_dn14 = assign15080_e22448_d_n14;
        locals.var_pseff_rv = 0.0;

        let assign15090_e22451: f64 = if locals.var_pseff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard482 = assign15090_e22451;
        locals.var_guard482_rv = 0.0;

        let (assign15100_e22458, assign15100_e22458_d_n0, assign15100_e22458_d_n2, assign15100_e22458_d_n3, assign15100_e22458_d_n4, assign15100_e22458_d_n5, assign15100_e22458_d_n6, assign15100_e22458_d_n7, assign15100_e22458_d_n8, assign15100_e22458_d_n9, assign15100_e22458_d_n10, assign15100_e22458_d_n11, assign15100_e22458_d_n12, assign15100_e22458_d_n13, assign15100_e22458_d_n14,) = {
    if ((locals.var_guard480 == 0.0) && (locals.var_guard482 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pseff, locals.var_pseff_dn0, locals.var_pseff_dn2, locals.var_pseff_dn3, locals.var_pseff_dn4, locals.var_pseff_dn5, locals.var_pseff_dn6, locals.var_pseff_dn7, locals.var_pseff_dn8, locals.var_pseff_dn9, locals.var_pseff_dn10, locals.var_pseff_dn11, locals.var_pseff_dn12, locals.var_pseff_dn13, locals.var_pseff_dn14,)
    }
};
        locals.var_pseff = assign15100_e22458;
        locals.var_pseff_dn0 = assign15100_e22458_d_n0;
        locals.var_pseff_dn2 = assign15100_e22458_d_n2;
        locals.var_pseff_dn3 = assign15100_e22458_d_n3;
        locals.var_pseff_dn4 = assign15100_e22458_d_n4;
        locals.var_pseff_dn5 = assign15100_e22458_d_n5;
        locals.var_pseff_dn6 = assign15100_e22458_d_n6;
        locals.var_pseff_dn7 = assign15100_e22458_d_n7;
        locals.var_pseff_dn8 = assign15100_e22458_d_n8;
        locals.var_pseff_dn9 = assign15100_e22458_d_n9;
        locals.var_pseff_dn10 = assign15100_e22458_d_n10;
        locals.var_pseff_dn11 = assign15100_e22458_d_n11;
        locals.var_pseff_dn12 = assign15100_e22458_d_n12;
        locals.var_pseff_dn13 = assign15100_e22458_d_n13;
        locals.var_pseff_dn14 = assign15100_e22458_d_n14;
        locals.var_pseff_rv = 0.0;

        let assign15110_e22460: f64 = if param_given[27] { 1.0 } else { 0.0 };
        locals.var_guard483 = assign15110_e22460;
        locals.var_guard483_rv = 0.0;

        let assign15120_e22463: f64 = if p.p137 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard484 = assign15120_e22463;
        locals.var_guard484_rv = 0.0;

        let (assign15130_e22471, assign15130_e22471_d_n0, assign15130_e22471_d_n2, assign15130_e22471_d_n3, assign15130_e22471_d_n4, assign15130_e22471_d_n5, assign15130_e22471_d_n6, assign15130_e22471_d_n7, assign15130_e22471_d_n8, assign15130_e22471_d_n9, assign15130_e22471_d_n10, assign15130_e22471_d_n11, assign15130_e22471_d_n12, assign15130_e22471_d_n13, assign15130_e22471_d_n14,) = {
    if ((locals.var_guard483 != 0.0) && (locals.var_guard484 != 0.0)) {
        let assign15130_e22469: f64 = (p.p27 * p.p53);
        (assign15130_e22469, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14,)
    }
};
        locals.var_pdeff = assign15130_e22471;
        locals.var_pdeff_dn0 = assign15130_e22471_d_n0;
        locals.var_pdeff_dn2 = assign15130_e22471_d_n2;
        locals.var_pdeff_dn3 = assign15130_e22471_d_n3;
        locals.var_pdeff_dn4 = assign15130_e22471_d_n4;
        locals.var_pdeff_dn5 = assign15130_e22471_d_n5;
        locals.var_pdeff_dn6 = assign15130_e22471_d_n6;
        locals.var_pdeff_dn7 = assign15130_e22471_d_n7;
        locals.var_pdeff_dn8 = assign15130_e22471_d_n8;
        locals.var_pdeff_dn9 = assign15130_e22471_d_n9;
        locals.var_pdeff_dn10 = assign15130_e22471_d_n10;
        locals.var_pdeff_dn11 = assign15130_e22471_d_n11;
        locals.var_pdeff_dn12 = assign15130_e22471_d_n12;
        locals.var_pdeff_dn13 = assign15130_e22471_d_n13;
        locals.var_pdeff_dn14 = assign15130_e22471_d_n14;
        locals.var_pdeff_rv = 0.0;

        let (assign15140_e22486, assign15140_e22486_d_n0, assign15140_e22486_d_n2, assign15140_e22486_d_n3, assign15140_e22486_d_n4, assign15140_e22486_d_n5, assign15140_e22486_d_n6, assign15140_e22486_d_n7, assign15140_e22486_d_n8, assign15140_e22486_d_n9, assign15140_e22486_d_n10, assign15140_e22486_d_n11, assign15140_e22486_d_n12, assign15140_e22486_d_n13, assign15140_e22486_d_n14,) = {
    if ((locals.var_guard483 != 0.0) && (locals.var_guard484 == 0.0)) {
        let assign15140_e22478: f64 = (p.p27 * p.p53);
        let assign15140_e22481: f64 = (locals.var_weffcj * p.p2);
        let assign15140_e22482: f64 = (assign15140_e22478 - assign15140_e22481);
        let assign15140_e22484: f64 = (assign15140_e22482).max(0.0);
        (assign15140_e22484, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14,)
    }
};
        locals.var_pdeff = assign15140_e22486;
        locals.var_pdeff_dn0 = assign15140_e22486_d_n0;
        locals.var_pdeff_dn2 = assign15140_e22486_d_n2;
        locals.var_pdeff_dn3 = assign15140_e22486_d_n3;
        locals.var_pdeff_dn4 = assign15140_e22486_d_n4;
        locals.var_pdeff_dn5 = assign15140_e22486_d_n5;
        locals.var_pdeff_dn6 = assign15140_e22486_d_n6;
        locals.var_pdeff_dn7 = assign15140_e22486_d_n7;
        locals.var_pdeff_dn8 = assign15140_e22486_d_n8;
        locals.var_pdeff_dn9 = assign15140_e22486_d_n9;
        locals.var_pdeff_dn10 = assign15140_e22486_d_n10;
        locals.var_pdeff_dn11 = assign15140_e22486_d_n11;
        locals.var_pdeff_dn12 = assign15140_e22486_d_n12;
        locals.var_pdeff_dn13 = assign15140_e22486_d_n13;
        locals.var_pdeff_dn14 = assign15140_e22486_d_n14;
        locals.var_pdeff_rv = 0.0;

        let (assign15150_e22491, assign15150_e22491_d_n0, assign15150_e22491_d_n2, assign15150_e22491_d_n3, assign15150_e22491_d_n4, assign15150_e22491_d_n5, assign15150_e22491_d_n6, assign15150_e22491_d_n7, assign15150_e22491_d_n8, assign15150_e22491_d_n9, assign15150_e22491_d_n10, assign15150_e22491_d_n11, assign15150_e22491_d_n12, assign15150_e22491_d_n13, assign15150_e22491_d_n14,) = {
    if (locals.var_guard483 == 0.0) {
        (locals.var_temp_pdeff, locals.var_temp_pdeff_dn0, locals.var_temp_pdeff_dn2, locals.var_temp_pdeff_dn3, locals.var_temp_pdeff_dn4, locals.var_temp_pdeff_dn5, locals.var_temp_pdeff_dn6, locals.var_temp_pdeff_dn7, locals.var_temp_pdeff_dn8, locals.var_temp_pdeff_dn9, locals.var_temp_pdeff_dn10, locals.var_temp_pdeff_dn11, locals.var_temp_pdeff_dn12, locals.var_temp_pdeff_dn13, locals.var_temp_pdeff_dn14,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14,)
    }
};
        locals.var_pdeff = assign15150_e22491;
        locals.var_pdeff_dn0 = assign15150_e22491_d_n0;
        locals.var_pdeff_dn2 = assign15150_e22491_d_n2;
        locals.var_pdeff_dn3 = assign15150_e22491_d_n3;
        locals.var_pdeff_dn4 = assign15150_e22491_d_n4;
        locals.var_pdeff_dn5 = assign15150_e22491_d_n5;
        locals.var_pdeff_dn6 = assign15150_e22491_d_n6;
        locals.var_pdeff_dn7 = assign15150_e22491_d_n7;
        locals.var_pdeff_dn8 = assign15150_e22491_d_n8;
        locals.var_pdeff_dn9 = assign15150_e22491_d_n9;
        locals.var_pdeff_dn10 = assign15150_e22491_d_n10;
        locals.var_pdeff_dn11 = assign15150_e22491_d_n11;
        locals.var_pdeff_dn12 = assign15150_e22491_d_n12;
        locals.var_pdeff_dn13 = assign15150_e22491_d_n13;
        locals.var_pdeff_dn14 = assign15150_e22491_d_n14;
        locals.var_pdeff_rv = 0.0;

        let assign15160_e22494: f64 = if locals.var_pdeff < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard485 = assign15160_e22494;
        locals.var_guard485_rv = 0.0;

        let (assign15170_e22501, assign15170_e22501_d_n0, assign15170_e22501_d_n2, assign15170_e22501_d_n3, assign15170_e22501_d_n4, assign15170_e22501_d_n5, assign15170_e22501_d_n6, assign15170_e22501_d_n7, assign15170_e22501_d_n8, assign15170_e22501_d_n9, assign15170_e22501_d_n10, assign15170_e22501_d_n11, assign15170_e22501_d_n12, assign15170_e22501_d_n13, assign15170_e22501_d_n14,) = {
    if ((locals.var_guard483 == 0.0) && (locals.var_guard485 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pdeff, locals.var_pdeff_dn0, locals.var_pdeff_dn2, locals.var_pdeff_dn3, locals.var_pdeff_dn4, locals.var_pdeff_dn5, locals.var_pdeff_dn6, locals.var_pdeff_dn7, locals.var_pdeff_dn8, locals.var_pdeff_dn9, locals.var_pdeff_dn10, locals.var_pdeff_dn11, locals.var_pdeff_dn12, locals.var_pdeff_dn13, locals.var_pdeff_dn14,)
    }
};
        locals.var_pdeff = assign15170_e22501;
        locals.var_pdeff_dn0 = assign15170_e22501_d_n0;
        locals.var_pdeff_dn2 = assign15170_e22501_d_n2;
        locals.var_pdeff_dn3 = assign15170_e22501_d_n3;
        locals.var_pdeff_dn4 = assign15170_e22501_d_n4;
        locals.var_pdeff_dn5 = assign15170_e22501_d_n5;
        locals.var_pdeff_dn6 = assign15170_e22501_d_n6;
        locals.var_pdeff_dn7 = assign15170_e22501_d_n7;
        locals.var_pdeff_dn8 = assign15170_e22501_d_n8;
        locals.var_pdeff_dn9 = assign15170_e22501_d_n9;
        locals.var_pdeff_dn10 = assign15170_e22501_d_n10;
        locals.var_pdeff_dn11 = assign15170_e22501_d_n11;
        locals.var_pdeff_dn12 = assign15170_e22501_d_n12;
        locals.var_pdeff_dn13 = assign15170_e22501_d_n13;
        locals.var_pdeff_dn14 = assign15170_e22501_d_n14;
        locals.var_pdeff_rv = 0.0;

        let assign15180_e22504: f64 = (locals.var_aseff * locals.var_jss_t);
        let assign15180_e22507: f64 = (locals.var_pseff * locals.var_jsws_t);
        let assign15180_e22508: f64 = (assign15180_e22504 + assign15180_e22507);
        let assign15180_e22511: f64 = (locals.var_weffcj * p.p2);
        let assign15180_e22513: f64 = (assign15180_e22511 * locals.var_jswgs_t);
        let assign15180_e22514: f64 = (assign15180_e22508 + assign15180_e22513);
        locals.var_isbs = assign15180_e22514;
        locals.var_isbs_dn0 = ((((locals.var_aseff_dn0 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn0)) + ((locals.var_pseff_dn0 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn0))) + (assign15180_e22511 * locals.var_jswgs_t_dn0));
        locals.var_isbs_dn2 = ((((locals.var_aseff_dn2 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn2)) + ((locals.var_pseff_dn2 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn2))) + (assign15180_e22511 * locals.var_jswgs_t_dn2));
        locals.var_isbs_dn3 = ((((locals.var_aseff_dn3 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn3)) + ((locals.var_pseff_dn3 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn3))) + (assign15180_e22511 * locals.var_jswgs_t_dn3));
        locals.var_isbs_dn4 = ((((locals.var_aseff_dn4 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn4)) + ((locals.var_pseff_dn4 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn4))) + (assign15180_e22511 * locals.var_jswgs_t_dn4));
        locals.var_isbs_dn5 = ((((locals.var_aseff_dn5 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn5)) + ((locals.var_pseff_dn5 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn5))) + (assign15180_e22511 * locals.var_jswgs_t_dn5));
        locals.var_isbs_dn6 = ((((locals.var_aseff_dn6 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn6)) + ((locals.var_pseff_dn6 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn6))) + (assign15180_e22511 * locals.var_jswgs_t_dn6));
        locals.var_isbs_dn7 = ((((locals.var_aseff_dn7 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn7)) + ((locals.var_pseff_dn7 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn7))) + (assign15180_e22511 * locals.var_jswgs_t_dn7));
        locals.var_isbs_dn8 = ((((locals.var_aseff_dn8 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn8)) + ((locals.var_pseff_dn8 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn8))) + (assign15180_e22511 * locals.var_jswgs_t_dn8));
        locals.var_isbs_dn9 = ((((locals.var_aseff_dn9 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn9)) + ((locals.var_pseff_dn9 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn9))) + (assign15180_e22511 * locals.var_jswgs_t_dn9));
        locals.var_isbs_dn10 = ((((locals.var_aseff_dn10 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn10)) + ((locals.var_pseff_dn10 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn10))) + (assign15180_e22511 * locals.var_jswgs_t_dn10));
        locals.var_isbs_dn11 = ((((locals.var_aseff_dn11 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn11)) + ((locals.var_pseff_dn11 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn11))) + (assign15180_e22511 * locals.var_jswgs_t_dn11));
        locals.var_isbs_dn12 = ((((locals.var_aseff_dn12 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn12)) + ((locals.var_pseff_dn12 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn12))) + (assign15180_e22511 * locals.var_jswgs_t_dn12));
        locals.var_isbs_dn13 = ((((locals.var_aseff_dn13 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn13)) + ((locals.var_pseff_dn13 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn13))) + (assign15180_e22511 * locals.var_jswgs_t_dn13));
        locals.var_isbs_dn14 = ((((locals.var_aseff_dn14 * locals.var_jss_t) + (locals.var_aseff * locals.var_jss_t_dn14)) + ((locals.var_pseff_dn14 * locals.var_jsws_t) + (locals.var_pseff * locals.var_jsws_t_dn14))) + (assign15180_e22511 * locals.var_jswgs_t_dn14));
        locals.var_isbs_rv = 0.0;

        let assign15190_e22517: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard486 = assign15190_e22517;
        locals.var_guard486_rv = 0.0;

        let (assign15200_e22523, assign15200_e22523_d_n4,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15200_e22521: f64 = (locals.var_vtm * p.p725);
        (assign15200_e22521, (locals.var_vtm_dn4 * p.p725),)
    } else {
        (locals.var_nvtms, locals.var_nvtms_dn4,)
    }
};
        locals.var_nvtms = assign15200_e22523;
        locals.var_nvtms_dn4 = assign15200_e22523_d_n4;
        locals.var_nvtms_rv = 0.0;

        let (assign15210_e22533, assign15210_e22533_d_n4,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15210_e22526: f64 = (-p.p731);
        let assign15210_e22528: f64 = (assign15210_e22526 / locals.var_nvtms);
        let assign15210_e22529: f64 = { let limited_exp_arg = assign15210_e22528; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15210_e22531: f64 = (assign15210_e22529 * p.p733);
        (assign15210_e22531, (({ let limited_exp_arg = assign15210_e22528; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign15210_e22526 * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms)))) * p.p733),)
    } else {
        (locals.var_xexpbvs, locals.var_xexpbvs_dn4,)
    }
};
        locals.var_xexpbvs = assign15210_e22533;
        locals.var_xexpbvs_dn4 = assign15210_e22533_d_n4;
        locals.var_xexpbvs_rv = 0.0;

        let (assign15220_e22541, assign15220_e22541_d_n0, assign15220_e22541_d_n2, assign15220_e22541_d_n3, assign15220_e22541_d_n4, assign15220_e22541_d_n5, assign15220_e22541_d_n6, assign15220_e22541_d_n7, assign15220_e22541_d_n8, assign15220_e22541_d_n9, assign15220_e22541_d_n10, assign15220_e22541_d_n11, assign15220_e22541_d_n12, assign15220_e22541_d_n13, assign15220_e22541_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15220_e22537: f64 = (p.p727 / locals.var_isbs);
        let assign15220_e22539: f64 = (assign15220_e22537).max(10.0);
        (assign15220_e22539, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 }, if assign15220_e22537 >= 10.0 { (-((p.p727 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) } else { 0.0 },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15220_e22541;
        locals.var_t2_dn0 = assign15220_e22541_d_n0;
        locals.var_t2_dn2 = assign15220_e22541_d_n2;
        locals.var_t2_dn3 = assign15220_e22541_d_n3;
        locals.var_t2_dn4 = assign15220_e22541_d_n4;
        locals.var_t2_dn5 = assign15220_e22541_d_n5;
        locals.var_t2_dn6 = assign15220_e22541_d_n6;
        locals.var_t2_dn7 = assign15220_e22541_d_n7;
        locals.var_t2_dn8 = assign15220_e22541_d_n8;
        locals.var_t2_dn9 = assign15220_e22541_d_n9;
        locals.var_t2_dn10 = assign15220_e22541_d_n10;
        locals.var_t2_dn11 = assign15220_e22541_d_n11;
        locals.var_t2_dn12 = assign15220_e22541_d_n12;
        locals.var_t2_dn13 = assign15220_e22541_d_n13;
        locals.var_t2_dn14 = assign15220_e22541_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15230_e22549, assign15230_e22549_d_n0, assign15230_e22549_d_n2, assign15230_e22549_d_n3, assign15230_e22549_d_n4, assign15230_e22549_d_n5, assign15230_e22549_d_n6, assign15230_e22549_d_n7, assign15230_e22549_d_n8, assign15230_e22549_d_n9, assign15230_e22549_d_n10, assign15230_e22549_d_n11, assign15230_e22549_d_n12, assign15230_e22549_d_n13, assign15230_e22549_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15230_e22545: f64 = (1.0 + locals.var_t2);
        let assign15230_e22547: f64 = (assign15230_e22545 - locals.var_xexpbvs);
        (assign15230_e22547, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, (locals.var_t2_dn4 - locals.var_xexpbvs_dn4), locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    } else {
        (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn12, locals.var_tb_dn13, locals.var_tb_dn14,)
    }
};
        locals.var_tb = assign15230_e22549;
        locals.var_tb_dn0 = assign15230_e22549_d_n0;
        locals.var_tb_dn2 = assign15230_e22549_d_n2;
        locals.var_tb_dn3 = assign15230_e22549_d_n3;
        locals.var_tb_dn4 = assign15230_e22549_d_n4;
        locals.var_tb_dn5 = assign15230_e22549_d_n5;
        locals.var_tb_dn6 = assign15230_e22549_d_n6;
        locals.var_tb_dn7 = assign15230_e22549_d_n7;
        locals.var_tb_dn8 = assign15230_e22549_d_n8;
        locals.var_tb_dn9 = assign15230_e22549_d_n9;
        locals.var_tb_dn10 = assign15230_e22549_d_n10;
        locals.var_tb_dn11 = assign15230_e22549_d_n11;
        locals.var_tb_dn12 = assign15230_e22549_d_n12;
        locals.var_tb_dn13 = assign15230_e22549_d_n13;
        locals.var_tb_dn14 = assign15230_e22549_d_n14;
        locals.var_tb_rv = 0.0;

        let (assign15240_e22569, assign15240_e22569_d_n0, assign15240_e22569_d_n2, assign15240_e22569_d_n3, assign15240_e22569_d_n4, assign15240_e22569_d_n5, assign15240_e22569_d_n6, assign15240_e22569_d_n7, assign15240_e22569_d_n8, assign15240_e22569_d_n9, assign15240_e22569_d_n10, assign15240_e22569_d_n11, assign15240_e22569_d_n12, assign15240_e22569_d_n13, assign15240_e22569_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15240_e22556: f64 = (locals.var_tb * locals.var_tb);
        let assign15240_e22559: f64 = (4.0 * locals.var_xexpbvs);
        let assign15240_e22560: f64 = (assign15240_e22556 + assign15240_e22559);
        let assign15240_e22561: f64 = (assign15240_e22560).sqrt();
        let assign15240_e22562: f64 = (locals.var_tb + assign15240_e22561);
        let assign15240_e22563: f64 = (0.5 * assign15240_e22562);
        let assign15240_e22565: f64 = (assign15240_e22563).max(1e-38);
        let assign15240_e22566: f64 = (assign15240_e22565).ln();
        let assign15240_e22567: f64 = (locals.var_nvtms * assign15240_e22566);
        (assign15240_e22567, (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn0 + (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn2 + (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn3 + (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), ((locals.var_nvtms_dn4 * assign15240_e22566) + (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) + (4.0 * locals.var_xexpbvs_dn4)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565))), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn5 + (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn6 + (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn7 + (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn8 + (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn9 + (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn10 + (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn11 + (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn12 + (((locals.var_tb_dn12 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn12)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn13 + (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)), (locals.var_nvtms * (if assign15240_e22563 >= 1e-38 { (0.5 * (locals.var_tb_dn14 + (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) / (2.0 * assign15240_e22561)))) } else { 0.0 } / assign15240_e22565)),)
    } else {
        (locals.var_vjsmfwd, locals.var_vjsmfwd_dn0, locals.var_vjsmfwd_dn2, locals.var_vjsmfwd_dn3, locals.var_vjsmfwd_dn4, locals.var_vjsmfwd_dn5, locals.var_vjsmfwd_dn6, locals.var_vjsmfwd_dn7, locals.var_vjsmfwd_dn8, locals.var_vjsmfwd_dn9, locals.var_vjsmfwd_dn10, locals.var_vjsmfwd_dn11, locals.var_vjsmfwd_dn12, locals.var_vjsmfwd_dn13, locals.var_vjsmfwd_dn14,)
    }
};
        locals.var_vjsmfwd = assign15240_e22569;
        locals.var_vjsmfwd_dn0 = assign15240_e22569_d_n0;
        locals.var_vjsmfwd_dn2 = assign15240_e22569_d_n2;
        locals.var_vjsmfwd_dn3 = assign15240_e22569_d_n3;
        locals.var_vjsmfwd_dn4 = assign15240_e22569_d_n4;
        locals.var_vjsmfwd_dn5 = assign15240_e22569_d_n5;
        locals.var_vjsmfwd_dn6 = assign15240_e22569_d_n6;
        locals.var_vjsmfwd_dn7 = assign15240_e22569_d_n7;
        locals.var_vjsmfwd_dn8 = assign15240_e22569_d_n8;
        locals.var_vjsmfwd_dn9 = assign15240_e22569_d_n9;
        locals.var_vjsmfwd_dn10 = assign15240_e22569_d_n10;
        locals.var_vjsmfwd_dn11 = assign15240_e22569_d_n11;
        locals.var_vjsmfwd_dn12 = assign15240_e22569_d_n12;
        locals.var_vjsmfwd_dn13 = assign15240_e22569_d_n13;
        locals.var_vjsmfwd_dn14 = assign15240_e22569_d_n14;
        locals.var_vjsmfwd_rv = 0.0;

        let (assign15250_e22576, assign15250_e22576_d_n0, assign15250_e22576_d_n2, assign15250_e22576_d_n3, assign15250_e22576_d_n4, assign15250_e22576_d_n5, assign15250_e22576_d_n6, assign15250_e22576_d_n7, assign15250_e22576_d_n8, assign15250_e22576_d_n9, assign15250_e22576_d_n10, assign15250_e22576_d_n11, assign15250_e22576_d_n12, assign15250_e22576_d_n13, assign15250_e22576_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15250_e22573: f64 = (locals.var_vjsmfwd / locals.var_nvtms);
        let assign15250_e22574: f64 = { let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign15250_e22574, ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn0 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn2 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn3 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vjsmfwd_dn4 * locals.var_nvtms) - (locals.var_vjsmfwd * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms))), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn5 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn6 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn7 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn8 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn9 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn10 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn11 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn12 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn13 / locals.var_nvtms)), ({ let limited_exp_arg = assign15250_e22573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjsmfwd_dn14 / locals.var_nvtms)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15250_e22576;
        locals.var_t0_dn0 = assign15250_e22576_d_n0;
        locals.var_t0_dn2 = assign15250_e22576_d_n2;
        locals.var_t0_dn3 = assign15250_e22576_d_n3;
        locals.var_t0_dn4 = assign15250_e22576_d_n4;
        locals.var_t0_dn5 = assign15250_e22576_d_n5;
        locals.var_t0_dn6 = assign15250_e22576_d_n6;
        locals.var_t0_dn7 = assign15250_e22576_d_n7;
        locals.var_t0_dn8 = assign15250_e22576_d_n8;
        locals.var_t0_dn9 = assign15250_e22576_d_n9;
        locals.var_t0_dn10 = assign15250_e22576_d_n10;
        locals.var_t0_dn11 = assign15250_e22576_d_n11;
        locals.var_t0_dn12 = assign15250_e22576_d_n12;
        locals.var_t0_dn13 = assign15250_e22576_d_n13;
        locals.var_t0_dn14 = assign15250_e22576_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15260_e22590, assign15260_e22590_d_n0, assign15260_e22590_d_n2, assign15260_e22590_d_n3, assign15260_e22590_d_n4, assign15260_e22590_d_n5, assign15260_e22590_d_n6, assign15260_e22590_d_n7, assign15260_e22590_d_n8, assign15260_e22590_d_n9, assign15260_e22590_d_n10, assign15260_e22590_d_n11, assign15260_e22590_d_n12, assign15260_e22590_d_n13, assign15260_e22590_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15260_e22582: f64 = (locals.var_xexpbvs / locals.var_t0);
        let assign15260_e22583: f64 = (locals.var_t0 - assign15260_e22582);
        let assign15260_e22585: f64 = (assign15260_e22583 + locals.var_xexpbvs);
        let assign15260_e22587: f64 = (assign15260_e22585 - 1.0);
        let assign15260_e22588: f64 = (locals.var_isbs * assign15260_e22587);
        (assign15260_e22588, ((locals.var_isbs_dn0 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn0 - (-((locals.var_xexpbvs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn2 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn2 - (-((locals.var_xexpbvs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn3 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn3 - (-((locals.var_xexpbvs * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn4 * assign15260_e22587) + (locals.var_isbs * ((locals.var_t0_dn4 - (((locals.var_xexpbvs_dn4 * locals.var_t0) - (locals.var_xexpbvs * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))) + locals.var_xexpbvs_dn4))), ((locals.var_isbs_dn5 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn5 - (-((locals.var_xexpbvs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn6 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn6 - (-((locals.var_xexpbvs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn7 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn7 - (-((locals.var_xexpbvs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn8 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn8 - (-((locals.var_xexpbvs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn9 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn9 - (-((locals.var_xexpbvs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn10 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn10 - (-((locals.var_xexpbvs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn11 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn11 - (-((locals.var_xexpbvs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn12 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn12 - (-((locals.var_xexpbvs * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn13 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn13 - (-((locals.var_xexpbvs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbs_dn14 * assign15260_e22587) + (locals.var_isbs * (locals.var_t0_dn14 - (-((locals.var_xexpbvs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))),)
    } else {
        (locals.var_ivjsmfwd, locals.var_ivjsmfwd_dn0, locals.var_ivjsmfwd_dn2, locals.var_ivjsmfwd_dn3, locals.var_ivjsmfwd_dn4, locals.var_ivjsmfwd_dn5, locals.var_ivjsmfwd_dn6, locals.var_ivjsmfwd_dn7, locals.var_ivjsmfwd_dn8, locals.var_ivjsmfwd_dn9, locals.var_ivjsmfwd_dn10, locals.var_ivjsmfwd_dn11, locals.var_ivjsmfwd_dn12, locals.var_ivjsmfwd_dn13, locals.var_ivjsmfwd_dn14,)
    }
};
        locals.var_ivjsmfwd = assign15260_e22590;
        locals.var_ivjsmfwd_dn0 = assign15260_e22590_d_n0;
        locals.var_ivjsmfwd_dn2 = assign15260_e22590_d_n2;
        locals.var_ivjsmfwd_dn3 = assign15260_e22590_d_n3;
        locals.var_ivjsmfwd_dn4 = assign15260_e22590_d_n4;
        locals.var_ivjsmfwd_dn5 = assign15260_e22590_d_n5;
        locals.var_ivjsmfwd_dn6 = assign15260_e22590_d_n6;
        locals.var_ivjsmfwd_dn7 = assign15260_e22590_d_n7;
        locals.var_ivjsmfwd_dn8 = assign15260_e22590_d_n8;
        locals.var_ivjsmfwd_dn9 = assign15260_e22590_d_n9;
        locals.var_ivjsmfwd_dn10 = assign15260_e22590_d_n10;
        locals.var_ivjsmfwd_dn11 = assign15260_e22590_d_n11;
        locals.var_ivjsmfwd_dn12 = assign15260_e22590_d_n12;
        locals.var_ivjsmfwd_dn13 = assign15260_e22590_d_n13;
        locals.var_ivjsmfwd_dn14 = assign15260_e22590_d_n14;
        locals.var_ivjsmfwd_rv = 0.0;

        let (assign15270_e22602, assign15270_e22602_d_n0, assign15270_e22602_d_n2, assign15270_e22602_d_n3, assign15270_e22602_d_n4, assign15270_e22602_d_n5, assign15270_e22602_d_n6, assign15270_e22602_d_n7, assign15270_e22602_d_n8, assign15270_e22602_d_n9, assign15270_e22602_d_n10, assign15270_e22602_d_n11, assign15270_e22602_d_n12, assign15270_e22602_d_n13, assign15270_e22602_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15270_e22596: f64 = (locals.var_xexpbvs / locals.var_t0);
        let assign15270_e22597: f64 = (locals.var_t0 + assign15270_e22596);
        let assign15270_e22598: f64 = (locals.var_isbs * assign15270_e22597);
        let assign15270_e22600: f64 = (assign15270_e22598 / locals.var_nvtms);
        (assign15270_e22600, (((locals.var_isbs_dn0 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn0 + (-((locals.var_xexpbvs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn2 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn2 + (-((locals.var_xexpbvs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn3 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn3 + (-((locals.var_xexpbvs * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((((locals.var_isbs_dn4 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn4 + (((locals.var_xexpbvs_dn4 * locals.var_t0) - (locals.var_xexpbvs * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))))) * locals.var_nvtms) - (assign15270_e22598 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)), (((locals.var_isbs_dn5 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn5 + (-((locals.var_xexpbvs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn6 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn6 + (-((locals.var_xexpbvs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn7 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn7 + (-((locals.var_xexpbvs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn8 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn8 + (-((locals.var_xexpbvs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn9 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn9 + (-((locals.var_xexpbvs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn10 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn10 + (-((locals.var_xexpbvs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn11 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn11 + (-((locals.var_xexpbvs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn12 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn12 + (-((locals.var_xexpbvs * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn13 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn13 + (-((locals.var_xexpbvs * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms), (((locals.var_isbs_dn14 * assign15270_e22597) + (locals.var_isbs * (locals.var_t0_dn14 + (-((locals.var_xexpbvs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtms),)
    } else {
        (locals.var_sslpfwd, locals.var_sslpfwd_dn0, locals.var_sslpfwd_dn2, locals.var_sslpfwd_dn3, locals.var_sslpfwd_dn4, locals.var_sslpfwd_dn5, locals.var_sslpfwd_dn6, locals.var_sslpfwd_dn7, locals.var_sslpfwd_dn8, locals.var_sslpfwd_dn9, locals.var_sslpfwd_dn10, locals.var_sslpfwd_dn11, locals.var_sslpfwd_dn12, locals.var_sslpfwd_dn13, locals.var_sslpfwd_dn14,)
    }
};
        locals.var_sslpfwd = assign15270_e22602;
        locals.var_sslpfwd_dn0 = assign15270_e22602_d_n0;
        locals.var_sslpfwd_dn2 = assign15270_e22602_d_n2;
        locals.var_sslpfwd_dn3 = assign15270_e22602_d_n3;
        locals.var_sslpfwd_dn4 = assign15270_e22602_d_n4;
        locals.var_sslpfwd_dn5 = assign15270_e22602_d_n5;
        locals.var_sslpfwd_dn6 = assign15270_e22602_d_n6;
        locals.var_sslpfwd_dn7 = assign15270_e22602_d_n7;
        locals.var_sslpfwd_dn8 = assign15270_e22602_d_n8;
        locals.var_sslpfwd_dn9 = assign15270_e22602_d_n9;
        locals.var_sslpfwd_dn10 = assign15270_e22602_d_n10;
        locals.var_sslpfwd_dn11 = assign15270_e22602_d_n11;
        locals.var_sslpfwd_dn12 = assign15270_e22602_d_n12;
        locals.var_sslpfwd_dn13 = assign15270_e22602_d_n13;
        locals.var_sslpfwd_dn14 = assign15270_e22602_d_n14;
        locals.var_sslpfwd_rv = 0.0;

        let (assign15280_e22667, assign15280_e22667_d_n0, assign15280_e22667_d_n2, assign15280_e22667_d_n3, assign15280_e22667_d_n4, assign15280_e22667_d_n5, assign15280_e22667_d_n6, assign15280_e22667_d_n7, assign15280_e22667_d_n8, assign15280_e22667_d_n9, assign15280_e22667_d_n10, assign15280_e22667_d_n11, assign15280_e22667_d_n12, assign15280_e22667_d_n13, assign15280_e22667_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15280_e22606: f64 = (p.p729 / locals.var_isbs);
        let assign15280_e22608: f64 = (assign15280_e22606 - 10.0);
        let assign15280_e22610: f64 = (-10000.0);
        let assign15280_e22612: f64 = (assign15280_e22610 * 0.001);
        let (assign15280_e22663, assign15280_e22663_d_n0, assign15280_e22663_d_n2, assign15280_e22663_d_n3, assign15280_e22663_d_n4, assign15280_e22663_d_n5, assign15280_e22663_d_n6, assign15280_e22663_d_n7, assign15280_e22663_d_n8, assign15280_e22663_d_n9, assign15280_e22663_d_n10, assign15280_e22663_d_n11, assign15280_e22663_d_n12, assign15280_e22663_d_n13, assign15280_e22663_d_n14,) = {
            if (!(assign15280_e22608 < assign15280_e22612)) {
                let assign15280_e22618: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22620: f64 = (assign15280_e22618 - 10.0);
                let assign15280_e22623: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22625: f64 = (assign15280_e22623 - 10.0);
                let assign15280_e22628: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22630: f64 = (assign15280_e22628 - 10.0);
                let assign15280_e22631: f64 = (assign15280_e22625 * assign15280_e22630);
                let assign15280_e22634: f64 = (4.0 * 0.001);
                let assign15280_e22636: f64 = (assign15280_e22634 * 0.001);
                let assign15280_e22637: f64 = (assign15280_e22631 + assign15280_e22636);
                let assign15280_e22638: f64 = (assign15280_e22637).sqrt();
                let assign15280_e22639: f64 = (assign15280_e22620 + assign15280_e22638);
                let assign15280_e22640: f64 = (0.5 * assign15280_e22639);
                (assign15280_e22640, (0.5 * ((-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))), (0.5 * ((-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) + ((((-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))) * assign15280_e22630) + (assign15280_e22625 * (-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs))))) / (2.0 * assign15280_e22638)))),)
            } else {
                let assign15280_e22643: f64 = (p.p729 / locals.var_isbs);
                let assign15280_e22645: f64 = (assign15280_e22643 - 10.0);
                let assign15280_e22647: f64 = (-10000.0);
                let assign15280_e22649: f64 = (assign15280_e22647 * 0.001);
                let (assign15280_e22662, assign15280_e22662_d_n0, assign15280_e22662_d_n2, assign15280_e22662_d_n3, assign15280_e22662_d_n4, assign15280_e22662_d_n5, assign15280_e22662_d_n6, assign15280_e22662_d_n7, assign15280_e22662_d_n8, assign15280_e22662_d_n9, assign15280_e22662_d_n10, assign15280_e22662_d_n11, assign15280_e22662_d_n12, assign15280_e22662_d_n13, assign15280_e22662_d_n14,) = {
                    if (assign15280_e22645 < assign15280_e22649) {
                        let assign15280_e22652: f64 = (-0.001);
                        let assign15280_e22654: f64 = (assign15280_e22652 * 0.001);
                        let assign15280_e22657: f64 = (p.p729 / locals.var_isbs);
                        let assign15280_e22659: f64 = (assign15280_e22657 - 10.0);
                        let assign15280_e22660: f64 = (assign15280_e22654 / assign15280_e22659);
                        (assign15280_e22660, (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn0) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn2) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn3) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn4) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn5) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn6) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn7) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn8) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn9) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn10) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn11) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn12) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn13) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))), (-((assign15280_e22654 * (-((p.p729 * locals.var_isbs_dn14) / (locals.var_isbs * locals.var_isbs)))) / (assign15280_e22659 * assign15280_e22659))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15280_e22662, assign15280_e22662_d_n0, assign15280_e22662_d_n2, assign15280_e22662_d_n3, assign15280_e22662_d_n4, assign15280_e22662_d_n5, assign15280_e22662_d_n6, assign15280_e22662_d_n7, assign15280_e22662_d_n8, assign15280_e22662_d_n9, assign15280_e22662_d_n10, assign15280_e22662_d_n11, assign15280_e22662_d_n12, assign15280_e22662_d_n13, assign15280_e22662_d_n14,)
            }
        };
        let assign15280_e22665: f64 = (assign15280_e22663 + 10.0);
        (assign15280_e22665, assign15280_e22663_d_n0, assign15280_e22663_d_n2, assign15280_e22663_d_n3, assign15280_e22663_d_n4, assign15280_e22663_d_n5, assign15280_e22663_d_n6, assign15280_e22663_d_n7, assign15280_e22663_d_n8, assign15280_e22663_d_n9, assign15280_e22663_d_n10, assign15280_e22663_d_n11, assign15280_e22663_d_n12, assign15280_e22663_d_n13, assign15280_e22663_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15280_e22667;
        locals.var_t2_dn0 = assign15280_e22667_d_n0;
        locals.var_t2_dn2 = assign15280_e22667_d_n2;
        locals.var_t2_dn3 = assign15280_e22667_d_n3;
        locals.var_t2_dn4 = assign15280_e22667_d_n4;
        locals.var_t2_dn5 = assign15280_e22667_d_n5;
        locals.var_t2_dn6 = assign15280_e22667_d_n6;
        locals.var_t2_dn7 = assign15280_e22667_d_n7;
        locals.var_t2_dn8 = assign15280_e22667_d_n8;
        locals.var_t2_dn9 = assign15280_e22667_d_n9;
        locals.var_t2_dn10 = assign15280_e22667_d_n10;
        locals.var_t2_dn11 = assign15280_e22667_d_n11;
        locals.var_t2_dn12 = assign15280_e22667_d_n12;
        locals.var_t2_dn13 = assign15280_e22667_d_n13;
        locals.var_t2_dn14 = assign15280_e22667_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15290_e22683, assign15290_e22683_d_n0, assign15290_e22683_d_n2, assign15290_e22683_d_n3, assign15290_e22683_d_n4, assign15290_e22683_d_n5, assign15290_e22683_d_n6, assign15290_e22683_d_n7, assign15290_e22683_d_n8, assign15290_e22683_d_n9, assign15290_e22683_d_n10, assign15290_e22683_d_n11, assign15290_e22683_d_n12, assign15290_e22683_d_n13, assign15290_e22683_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15290_e22670: f64 = (-p.p731);
        let assign15290_e22674: f64 = (locals.var_t2 - 1.0);
        let assign15290_e22676: f64 = (assign15290_e22674 / p.p733);
        let assign15290_e22678: f64 = (assign15290_e22676).max(1e-38);
        let assign15290_e22679: f64 = (assign15290_e22678).ln();
        let assign15290_e22680: f64 = (locals.var_nvtms * assign15290_e22679);
        let assign15290_e22681: f64 = (assign15290_e22670 - assign15290_e22680);
        (assign15290_e22681, (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn0 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn2 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn3 / p.p733) } else { 0.0 } / assign15290_e22678))), (-((locals.var_nvtms_dn4 * assign15290_e22679) + (locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn4 / p.p733) } else { 0.0 } / assign15290_e22678)))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn5 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn6 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn7 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn8 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn9 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn10 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn11 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn12 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn13 / p.p733) } else { 0.0 } / assign15290_e22678))), (-(locals.var_nvtms * (if assign15290_e22676 >= 1e-38 { (locals.var_t2_dn14 / p.p733) } else { 0.0 } / assign15290_e22678))),)
    } else {
        (locals.var_vjsmrev, locals.var_vjsmrev_dn0, locals.var_vjsmrev_dn2, locals.var_vjsmrev_dn3, locals.var_vjsmrev_dn4, locals.var_vjsmrev_dn5, locals.var_vjsmrev_dn6, locals.var_vjsmrev_dn7, locals.var_vjsmrev_dn8, locals.var_vjsmrev_dn9, locals.var_vjsmrev_dn10, locals.var_vjsmrev_dn11, locals.var_vjsmrev_dn12, locals.var_vjsmrev_dn13, locals.var_vjsmrev_dn14,)
    }
};
        locals.var_vjsmrev = assign15290_e22683;
        locals.var_vjsmrev_dn0 = assign15290_e22683_d_n0;
        locals.var_vjsmrev_dn2 = assign15290_e22683_d_n2;
        locals.var_vjsmrev_dn3 = assign15290_e22683_d_n3;
        locals.var_vjsmrev_dn4 = assign15290_e22683_d_n4;
        locals.var_vjsmrev_dn5 = assign15290_e22683_d_n5;
        locals.var_vjsmrev_dn6 = assign15290_e22683_d_n6;
        locals.var_vjsmrev_dn7 = assign15290_e22683_d_n7;
        locals.var_vjsmrev_dn8 = assign15290_e22683_d_n8;
        locals.var_vjsmrev_dn9 = assign15290_e22683_d_n9;
        locals.var_vjsmrev_dn10 = assign15290_e22683_d_n10;
        locals.var_vjsmrev_dn11 = assign15290_e22683_d_n11;
        locals.var_vjsmrev_dn12 = assign15290_e22683_d_n12;
        locals.var_vjsmrev_dn13 = assign15290_e22683_d_n13;
        locals.var_vjsmrev_dn14 = assign15290_e22683_d_n14;
        locals.var_vjsmrev_rv = 0.0;

        let (assign15300_e22695, assign15300_e22695_d_n0, assign15300_e22695_d_n2, assign15300_e22695_d_n3, assign15300_e22695_d_n4, assign15300_e22695_d_n5, assign15300_e22695_d_n6, assign15300_e22695_d_n7, assign15300_e22695_d_n8, assign15300_e22695_d_n9, assign15300_e22695_d_n10, assign15300_e22695_d_n11, assign15300_e22695_d_n12, assign15300_e22695_d_n13, assign15300_e22695_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15300_e22688: f64 = (p.p731 + locals.var_vjsmrev);
        let assign15300_e22689: f64 = (-assign15300_e22688);
        let assign15300_e22691: f64 = (assign15300_e22689 / locals.var_nvtms);
        let assign15300_e22692: f64 = { let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15300_e22693: f64 = (p.p733 * assign15300_e22692);
        (assign15300_e22693, (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn0) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn2) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn3) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-locals.var_vjsmrev_dn4) * locals.var_nvtms) - (assign15300_e22689 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn5) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn6) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn7) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn8) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn9) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn10) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn11) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn12) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn13) / locals.var_nvtms))), (p.p733 * ({ let limited_exp_arg = assign15300_e22691; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjsmrev_dn14) / locals.var_nvtms))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15300_e22695;
        locals.var_t1_dn0 = assign15300_e22695_d_n0;
        locals.var_t1_dn2 = assign15300_e22695_d_n2;
        locals.var_t1_dn3 = assign15300_e22695_d_n3;
        locals.var_t1_dn4 = assign15300_e22695_d_n4;
        locals.var_t1_dn5 = assign15300_e22695_d_n5;
        locals.var_t1_dn6 = assign15300_e22695_d_n6;
        locals.var_t1_dn7 = assign15300_e22695_d_n7;
        locals.var_t1_dn8 = assign15300_e22695_d_n8;
        locals.var_t1_dn9 = assign15300_e22695_d_n9;
        locals.var_t1_dn10 = assign15300_e22695_d_n10;
        locals.var_t1_dn11 = assign15300_e22695_d_n11;
        locals.var_t1_dn12 = assign15300_e22695_d_n12;
        locals.var_t1_dn13 = assign15300_e22695_d_n13;
        locals.var_t1_dn14 = assign15300_e22695_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15310_e22703, assign15310_e22703_d_n0, assign15310_e22703_d_n2, assign15310_e22703_d_n3, assign15310_e22703_d_n4, assign15310_e22703_d_n5, assign15310_e22703_d_n6, assign15310_e22703_d_n7, assign15310_e22703_d_n8, assign15310_e22703_d_n9, assign15310_e22703_d_n10, assign15310_e22703_d_n11, assign15310_e22703_d_n12, assign15310_e22703_d_n13, assign15310_e22703_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15310_e22700: f64 = (1.0 + locals.var_t1);
        let assign15310_e22701: f64 = (locals.var_isbs * assign15310_e22700);
        (assign15310_e22701, ((locals.var_isbs_dn0 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn0)), ((locals.var_isbs_dn2 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn2)), ((locals.var_isbs_dn3 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn3)), ((locals.var_isbs_dn4 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn4)), ((locals.var_isbs_dn5 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn5)), ((locals.var_isbs_dn6 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn6)), ((locals.var_isbs_dn7 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn7)), ((locals.var_isbs_dn8 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn8)), ((locals.var_isbs_dn9 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn9)), ((locals.var_isbs_dn10 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn10)), ((locals.var_isbs_dn11 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn11)), ((locals.var_isbs_dn12 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn12)), ((locals.var_isbs_dn13 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn13)), ((locals.var_isbs_dn14 * assign15310_e22700) + (locals.var_isbs * locals.var_t1_dn14)),)
    } else {
        (locals.var_ivjsmrev, locals.var_ivjsmrev_dn0, locals.var_ivjsmrev_dn2, locals.var_ivjsmrev_dn3, locals.var_ivjsmrev_dn4, locals.var_ivjsmrev_dn5, locals.var_ivjsmrev_dn6, locals.var_ivjsmrev_dn7, locals.var_ivjsmrev_dn8, locals.var_ivjsmrev_dn9, locals.var_ivjsmrev_dn10, locals.var_ivjsmrev_dn11, locals.var_ivjsmrev_dn12, locals.var_ivjsmrev_dn13, locals.var_ivjsmrev_dn14,)
    }
};
        locals.var_ivjsmrev = assign15310_e22703;
        locals.var_ivjsmrev_dn0 = assign15310_e22703_d_n0;
        locals.var_ivjsmrev_dn2 = assign15310_e22703_d_n2;
        locals.var_ivjsmrev_dn3 = assign15310_e22703_d_n3;
        locals.var_ivjsmrev_dn4 = assign15310_e22703_d_n4;
        locals.var_ivjsmrev_dn5 = assign15310_e22703_d_n5;
        locals.var_ivjsmrev_dn6 = assign15310_e22703_d_n6;
        locals.var_ivjsmrev_dn7 = assign15310_e22703_d_n7;
        locals.var_ivjsmrev_dn8 = assign15310_e22703_d_n8;
        locals.var_ivjsmrev_dn9 = assign15310_e22703_d_n9;
        locals.var_ivjsmrev_dn10 = assign15310_e22703_d_n10;
        locals.var_ivjsmrev_dn11 = assign15310_e22703_d_n11;
        locals.var_ivjsmrev_dn12 = assign15310_e22703_d_n12;
        locals.var_ivjsmrev_dn13 = assign15310_e22703_d_n13;
        locals.var_ivjsmrev_dn14 = assign15310_e22703_d_n14;
        locals.var_ivjsmrev_rv = 0.0;

        let (assign15320_e22712, assign15320_e22712_d_n0, assign15320_e22712_d_n2, assign15320_e22712_d_n3, assign15320_e22712_d_n4, assign15320_e22712_d_n5, assign15320_e22712_d_n6, assign15320_e22712_d_n7, assign15320_e22712_d_n8, assign15320_e22712_d_n9, assign15320_e22712_d_n10, assign15320_e22712_d_n11, assign15320_e22712_d_n12, assign15320_e22712_d_n13, assign15320_e22712_d_n14,) = {
    if (locals.var_guard486 != 0.0) {
        let assign15320_e22706: f64 = (-locals.var_isbs);
        let assign15320_e22708: f64 = (assign15320_e22706 * locals.var_t1);
        let assign15320_e22710: f64 = (assign15320_e22708 / locals.var_nvtms);
        (assign15320_e22710, ((((-locals.var_isbs_dn0) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn0)) / locals.var_nvtms), ((((-locals.var_isbs_dn2) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn2)) / locals.var_nvtms), ((((-locals.var_isbs_dn3) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn3)) / locals.var_nvtms), ((((((-locals.var_isbs_dn4) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn4)) * locals.var_nvtms) - (assign15320_e22708 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)), ((((-locals.var_isbs_dn5) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn5)) / locals.var_nvtms), ((((-locals.var_isbs_dn6) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn6)) / locals.var_nvtms), ((((-locals.var_isbs_dn7) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn7)) / locals.var_nvtms), ((((-locals.var_isbs_dn8) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn8)) / locals.var_nvtms), ((((-locals.var_isbs_dn9) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn9)) / locals.var_nvtms), ((((-locals.var_isbs_dn10) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn10)) / locals.var_nvtms), ((((-locals.var_isbs_dn11) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn11)) / locals.var_nvtms), ((((-locals.var_isbs_dn12) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn12)) / locals.var_nvtms), ((((-locals.var_isbs_dn13) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn13)) / locals.var_nvtms), ((((-locals.var_isbs_dn14) * locals.var_t1) + (assign15320_e22706 * locals.var_t1_dn14)) / locals.var_nvtms),)
    } else {
        (locals.var_sslprev, locals.var_sslprev_dn0, locals.var_sslprev_dn2, locals.var_sslprev_dn3, locals.var_sslprev_dn4, locals.var_sslprev_dn5, locals.var_sslprev_dn6, locals.var_sslprev_dn7, locals.var_sslprev_dn8, locals.var_sslprev_dn9, locals.var_sslprev_dn10, locals.var_sslprev_dn11, locals.var_sslprev_dn12, locals.var_sslprev_dn13, locals.var_sslprev_dn14,)
    }
};
        locals.var_sslprev = assign15320_e22712;
        locals.var_sslprev_dn0 = assign15320_e22712_d_n0;
        locals.var_sslprev_dn2 = assign15320_e22712_d_n2;
        locals.var_sslprev_dn3 = assign15320_e22712_d_n3;
        locals.var_sslprev_dn4 = assign15320_e22712_d_n4;
        locals.var_sslprev_dn5 = assign15320_e22712_d_n5;
        locals.var_sslprev_dn6 = assign15320_e22712_d_n6;
        locals.var_sslprev_dn7 = assign15320_e22712_d_n7;
        locals.var_sslprev_dn8 = assign15320_e22712_d_n8;
        locals.var_sslprev_dn9 = assign15320_e22712_d_n9;
        locals.var_sslprev_dn10 = assign15320_e22712_d_n10;
        locals.var_sslprev_dn11 = assign15320_e22712_d_n11;
        locals.var_sslprev_dn12 = assign15320_e22712_d_n12;
        locals.var_sslprev_dn13 = assign15320_e22712_d_n13;
        locals.var_sslprev_dn14 = assign15320_e22712_d_n14;
        locals.var_sslprev_rv = 0.0;

        let (assign15330_e22717, assign15330_e22717_d_n4,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_nvtms, locals.var_nvtms_dn4,)
    }
};
        locals.var_nvtms = assign15330_e22717;
        locals.var_nvtms_dn4 = assign15330_e22717_d_n4;
        locals.var_nvtms_rv = 0.0;

        let (assign15340_e22722, assign15340_e22722_d_n4,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_xexpbvs, locals.var_xexpbvs_dn4,)
    }
};
        locals.var_xexpbvs = assign15340_e22722;
        locals.var_xexpbvs_dn4 = assign15340_e22722_d_n4;
        locals.var_xexpbvs_rv = 0.0;

        let (assign15350_e22727, assign15350_e22727_d_n0, assign15350_e22727_d_n2, assign15350_e22727_d_n3, assign15350_e22727_d_n4, assign15350_e22727_d_n5, assign15350_e22727_d_n6, assign15350_e22727_d_n7, assign15350_e22727_d_n8, assign15350_e22727_d_n9, assign15350_e22727_d_n10, assign15350_e22727_d_n11, assign15350_e22727_d_n12, assign15350_e22727_d_n13, assign15350_e22727_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjsmfwd, locals.var_vjsmfwd_dn0, locals.var_vjsmfwd_dn2, locals.var_vjsmfwd_dn3, locals.var_vjsmfwd_dn4, locals.var_vjsmfwd_dn5, locals.var_vjsmfwd_dn6, locals.var_vjsmfwd_dn7, locals.var_vjsmfwd_dn8, locals.var_vjsmfwd_dn9, locals.var_vjsmfwd_dn10, locals.var_vjsmfwd_dn11, locals.var_vjsmfwd_dn12, locals.var_vjsmfwd_dn13, locals.var_vjsmfwd_dn14,)
    }
};
        locals.var_vjsmfwd = assign15350_e22727;
        locals.var_vjsmfwd_dn0 = assign15350_e22727_d_n0;
        locals.var_vjsmfwd_dn2 = assign15350_e22727_d_n2;
        locals.var_vjsmfwd_dn3 = assign15350_e22727_d_n3;
        locals.var_vjsmfwd_dn4 = assign15350_e22727_d_n4;
        locals.var_vjsmfwd_dn5 = assign15350_e22727_d_n5;
        locals.var_vjsmfwd_dn6 = assign15350_e22727_d_n6;
        locals.var_vjsmfwd_dn7 = assign15350_e22727_d_n7;
        locals.var_vjsmfwd_dn8 = assign15350_e22727_d_n8;
        locals.var_vjsmfwd_dn9 = assign15350_e22727_d_n9;
        locals.var_vjsmfwd_dn10 = assign15350_e22727_d_n10;
        locals.var_vjsmfwd_dn11 = assign15350_e22727_d_n11;
        locals.var_vjsmfwd_dn12 = assign15350_e22727_d_n12;
        locals.var_vjsmfwd_dn13 = assign15350_e22727_d_n13;
        locals.var_vjsmfwd_dn14 = assign15350_e22727_d_n14;
        locals.var_vjsmfwd_rv = 0.0;

        let (assign15360_e22732, assign15360_e22732_d_n0, assign15360_e22732_d_n2, assign15360_e22732_d_n3, assign15360_e22732_d_n4, assign15360_e22732_d_n5, assign15360_e22732_d_n6, assign15360_e22732_d_n7, assign15360_e22732_d_n8, assign15360_e22732_d_n9, assign15360_e22732_d_n10, assign15360_e22732_d_n11, assign15360_e22732_d_n12, assign15360_e22732_d_n13, assign15360_e22732_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ivjsmfwd, locals.var_ivjsmfwd_dn0, locals.var_ivjsmfwd_dn2, locals.var_ivjsmfwd_dn3, locals.var_ivjsmfwd_dn4, locals.var_ivjsmfwd_dn5, locals.var_ivjsmfwd_dn6, locals.var_ivjsmfwd_dn7, locals.var_ivjsmfwd_dn8, locals.var_ivjsmfwd_dn9, locals.var_ivjsmfwd_dn10, locals.var_ivjsmfwd_dn11, locals.var_ivjsmfwd_dn12, locals.var_ivjsmfwd_dn13, locals.var_ivjsmfwd_dn14,)
    }
};
        locals.var_ivjsmfwd = assign15360_e22732;
        locals.var_ivjsmfwd_dn0 = assign15360_e22732_d_n0;
        locals.var_ivjsmfwd_dn2 = assign15360_e22732_d_n2;
        locals.var_ivjsmfwd_dn3 = assign15360_e22732_d_n3;
        locals.var_ivjsmfwd_dn4 = assign15360_e22732_d_n4;
        locals.var_ivjsmfwd_dn5 = assign15360_e22732_d_n5;
        locals.var_ivjsmfwd_dn6 = assign15360_e22732_d_n6;
        locals.var_ivjsmfwd_dn7 = assign15360_e22732_d_n7;
        locals.var_ivjsmfwd_dn8 = assign15360_e22732_d_n8;
        locals.var_ivjsmfwd_dn9 = assign15360_e22732_d_n9;
        locals.var_ivjsmfwd_dn10 = assign15360_e22732_d_n10;
        locals.var_ivjsmfwd_dn11 = assign15360_e22732_d_n11;
        locals.var_ivjsmfwd_dn12 = assign15360_e22732_d_n12;
        locals.var_ivjsmfwd_dn13 = assign15360_e22732_d_n13;
        locals.var_ivjsmfwd_dn14 = assign15360_e22732_d_n14;
        locals.var_ivjsmfwd_rv = 0.0;

        let (assign15370_e22737, assign15370_e22737_d_n0, assign15370_e22737_d_n2, assign15370_e22737_d_n3, assign15370_e22737_d_n4, assign15370_e22737_d_n5, assign15370_e22737_d_n6, assign15370_e22737_d_n7, assign15370_e22737_d_n8, assign15370_e22737_d_n9, assign15370_e22737_d_n10, assign15370_e22737_d_n11, assign15370_e22737_d_n12, assign15370_e22737_d_n13, assign15370_e22737_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sslpfwd, locals.var_sslpfwd_dn0, locals.var_sslpfwd_dn2, locals.var_sslpfwd_dn3, locals.var_sslpfwd_dn4, locals.var_sslpfwd_dn5, locals.var_sslpfwd_dn6, locals.var_sslpfwd_dn7, locals.var_sslpfwd_dn8, locals.var_sslpfwd_dn9, locals.var_sslpfwd_dn10, locals.var_sslpfwd_dn11, locals.var_sslpfwd_dn12, locals.var_sslpfwd_dn13, locals.var_sslpfwd_dn14,)
    }
};
        locals.var_sslpfwd = assign15370_e22737;
        locals.var_sslpfwd_dn0 = assign15370_e22737_d_n0;
        locals.var_sslpfwd_dn2 = assign15370_e22737_d_n2;
        locals.var_sslpfwd_dn3 = assign15370_e22737_d_n3;
        locals.var_sslpfwd_dn4 = assign15370_e22737_d_n4;
        locals.var_sslpfwd_dn5 = assign15370_e22737_d_n5;
        locals.var_sslpfwd_dn6 = assign15370_e22737_d_n6;
        locals.var_sslpfwd_dn7 = assign15370_e22737_d_n7;
        locals.var_sslpfwd_dn8 = assign15370_e22737_d_n8;
        locals.var_sslpfwd_dn9 = assign15370_e22737_d_n9;
        locals.var_sslpfwd_dn10 = assign15370_e22737_d_n10;
        locals.var_sslpfwd_dn11 = assign15370_e22737_d_n11;
        locals.var_sslpfwd_dn12 = assign15370_e22737_d_n12;
        locals.var_sslpfwd_dn13 = assign15370_e22737_d_n13;
        locals.var_sslpfwd_dn14 = assign15370_e22737_d_n14;
        locals.var_sslpfwd_rv = 0.0;

        let (assign15380_e22742, assign15380_e22742_d_n0, assign15380_e22742_d_n2, assign15380_e22742_d_n3, assign15380_e22742_d_n4, assign15380_e22742_d_n5, assign15380_e22742_d_n6, assign15380_e22742_d_n7, assign15380_e22742_d_n8, assign15380_e22742_d_n9, assign15380_e22742_d_n10, assign15380_e22742_d_n11, assign15380_e22742_d_n12, assign15380_e22742_d_n13, assign15380_e22742_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjsmrev, locals.var_vjsmrev_dn0, locals.var_vjsmrev_dn2, locals.var_vjsmrev_dn3, locals.var_vjsmrev_dn4, locals.var_vjsmrev_dn5, locals.var_vjsmrev_dn6, locals.var_vjsmrev_dn7, locals.var_vjsmrev_dn8, locals.var_vjsmrev_dn9, locals.var_vjsmrev_dn10, locals.var_vjsmrev_dn11, locals.var_vjsmrev_dn12, locals.var_vjsmrev_dn13, locals.var_vjsmrev_dn14,)
    }
};
        locals.var_vjsmrev = assign15380_e22742;
        locals.var_vjsmrev_dn0 = assign15380_e22742_d_n0;
        locals.var_vjsmrev_dn2 = assign15380_e22742_d_n2;
        locals.var_vjsmrev_dn3 = assign15380_e22742_d_n3;
        locals.var_vjsmrev_dn4 = assign15380_e22742_d_n4;
        locals.var_vjsmrev_dn5 = assign15380_e22742_d_n5;
        locals.var_vjsmrev_dn6 = assign15380_e22742_d_n6;
        locals.var_vjsmrev_dn7 = assign15380_e22742_d_n7;
        locals.var_vjsmrev_dn8 = assign15380_e22742_d_n8;
        locals.var_vjsmrev_dn9 = assign15380_e22742_d_n9;
        locals.var_vjsmrev_dn10 = assign15380_e22742_d_n10;
        locals.var_vjsmrev_dn11 = assign15380_e22742_d_n11;
        locals.var_vjsmrev_dn12 = assign15380_e22742_d_n12;
        locals.var_vjsmrev_dn13 = assign15380_e22742_d_n13;
        locals.var_vjsmrev_dn14 = assign15380_e22742_d_n14;
        locals.var_vjsmrev_rv = 0.0;

        let (assign15390_e22747, assign15390_e22747_d_n0, assign15390_e22747_d_n2, assign15390_e22747_d_n3, assign15390_e22747_d_n4, assign15390_e22747_d_n5, assign15390_e22747_d_n6, assign15390_e22747_d_n7, assign15390_e22747_d_n8, assign15390_e22747_d_n9, assign15390_e22747_d_n10, assign15390_e22747_d_n11, assign15390_e22747_d_n12, assign15390_e22747_d_n13, assign15390_e22747_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ivjsmrev, locals.var_ivjsmrev_dn0, locals.var_ivjsmrev_dn2, locals.var_ivjsmrev_dn3, locals.var_ivjsmrev_dn4, locals.var_ivjsmrev_dn5, locals.var_ivjsmrev_dn6, locals.var_ivjsmrev_dn7, locals.var_ivjsmrev_dn8, locals.var_ivjsmrev_dn9, locals.var_ivjsmrev_dn10, locals.var_ivjsmrev_dn11, locals.var_ivjsmrev_dn12, locals.var_ivjsmrev_dn13, locals.var_ivjsmrev_dn14,)
    }
};
        locals.var_ivjsmrev = assign15390_e22747;
        locals.var_ivjsmrev_dn0 = assign15390_e22747_d_n0;
        locals.var_ivjsmrev_dn2 = assign15390_e22747_d_n2;
        locals.var_ivjsmrev_dn3 = assign15390_e22747_d_n3;
        locals.var_ivjsmrev_dn4 = assign15390_e22747_d_n4;
        locals.var_ivjsmrev_dn5 = assign15390_e22747_d_n5;
        locals.var_ivjsmrev_dn6 = assign15390_e22747_d_n6;
        locals.var_ivjsmrev_dn7 = assign15390_e22747_d_n7;
        locals.var_ivjsmrev_dn8 = assign15390_e22747_d_n8;
        locals.var_ivjsmrev_dn9 = assign15390_e22747_d_n9;
        locals.var_ivjsmrev_dn10 = assign15390_e22747_d_n10;
        locals.var_ivjsmrev_dn11 = assign15390_e22747_d_n11;
        locals.var_ivjsmrev_dn12 = assign15390_e22747_d_n12;
        locals.var_ivjsmrev_dn13 = assign15390_e22747_d_n13;
        locals.var_ivjsmrev_dn14 = assign15390_e22747_d_n14;
        locals.var_ivjsmrev_rv = 0.0;

        let (assign15400_e22752, assign15400_e22752_d_n0, assign15400_e22752_d_n2, assign15400_e22752_d_n3, assign15400_e22752_d_n4, assign15400_e22752_d_n5, assign15400_e22752_d_n6, assign15400_e22752_d_n7, assign15400_e22752_d_n8, assign15400_e22752_d_n9, assign15400_e22752_d_n10, assign15400_e22752_d_n11, assign15400_e22752_d_n12, assign15400_e22752_d_n13, assign15400_e22752_d_n14,) = {
    if (locals.var_guard486 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_sslprev, locals.var_sslprev_dn0, locals.var_sslprev_dn2, locals.var_sslprev_dn3, locals.var_sslprev_dn4, locals.var_sslprev_dn5, locals.var_sslprev_dn6, locals.var_sslprev_dn7, locals.var_sslprev_dn8, locals.var_sslprev_dn9, locals.var_sslprev_dn10, locals.var_sslprev_dn11, locals.var_sslprev_dn12, locals.var_sslprev_dn13, locals.var_sslprev_dn14,)
    }
};
        locals.var_sslprev = assign15400_e22752;
        locals.var_sslprev_dn0 = assign15400_e22752_d_n0;
        locals.var_sslprev_dn2 = assign15400_e22752_d_n2;
        locals.var_sslprev_dn3 = assign15400_e22752_d_n3;
        locals.var_sslprev_dn4 = assign15400_e22752_d_n4;
        locals.var_sslprev_dn5 = assign15400_e22752_d_n5;
        locals.var_sslprev_dn6 = assign15400_e22752_d_n6;
        locals.var_sslprev_dn7 = assign15400_e22752_d_n7;
        locals.var_sslprev_dn8 = assign15400_e22752_d_n8;
        locals.var_sslprev_dn9 = assign15400_e22752_d_n9;
        locals.var_sslprev_dn10 = assign15400_e22752_d_n10;
        locals.var_sslprev_dn11 = assign15400_e22752_d_n11;
        locals.var_sslprev_dn12 = assign15400_e22752_d_n12;
        locals.var_sslprev_dn13 = assign15400_e22752_d_n13;
        locals.var_sslprev_dn14 = assign15400_e22752_d_n14;
        locals.var_sslprev_rv = 0.0;

        let assign15410_e22755: f64 = (locals.var_adeff * locals.var_jsd_t);
        let assign15410_e22758: f64 = (locals.var_pdeff * locals.var_jswd_t);
        let assign15410_e22759: f64 = (assign15410_e22755 + assign15410_e22758);
        let assign15410_e22762: f64 = (locals.var_weffcj * p.p2);
        let assign15410_e22764: f64 = (assign15410_e22762 * locals.var_jswgd_t);
        let assign15410_e22765: f64 = (assign15410_e22759 + assign15410_e22764);
        locals.var_isbd = assign15410_e22765;
        locals.var_isbd_dn0 = ((((locals.var_adeff_dn0 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn0)) + ((locals.var_pdeff_dn0 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn0))) + (assign15410_e22762 * locals.var_jswgd_t_dn0));
        locals.var_isbd_dn2 = ((((locals.var_adeff_dn2 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn2)) + ((locals.var_pdeff_dn2 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn2))) + (assign15410_e22762 * locals.var_jswgd_t_dn2));
        locals.var_isbd_dn3 = ((((locals.var_adeff_dn3 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn3)) + ((locals.var_pdeff_dn3 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn3))) + (assign15410_e22762 * locals.var_jswgd_t_dn3));
        locals.var_isbd_dn4 = ((((locals.var_adeff_dn4 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn4)) + ((locals.var_pdeff_dn4 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn4))) + (assign15410_e22762 * locals.var_jswgd_t_dn4));
        locals.var_isbd_dn5 = ((((locals.var_adeff_dn5 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn5)) + ((locals.var_pdeff_dn5 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn5))) + (assign15410_e22762 * locals.var_jswgd_t_dn5));
        locals.var_isbd_dn6 = ((((locals.var_adeff_dn6 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn6)) + ((locals.var_pdeff_dn6 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn6))) + (assign15410_e22762 * locals.var_jswgd_t_dn6));
        locals.var_isbd_dn7 = ((((locals.var_adeff_dn7 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn7)) + ((locals.var_pdeff_dn7 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn7))) + (assign15410_e22762 * locals.var_jswgd_t_dn7));
        locals.var_isbd_dn8 = ((((locals.var_adeff_dn8 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn8)) + ((locals.var_pdeff_dn8 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn8))) + (assign15410_e22762 * locals.var_jswgd_t_dn8));
        locals.var_isbd_dn9 = ((((locals.var_adeff_dn9 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn9)) + ((locals.var_pdeff_dn9 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn9))) + (assign15410_e22762 * locals.var_jswgd_t_dn9));
        locals.var_isbd_dn10 = ((((locals.var_adeff_dn10 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn10)) + ((locals.var_pdeff_dn10 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn10))) + (assign15410_e22762 * locals.var_jswgd_t_dn10));
        locals.var_isbd_dn11 = ((((locals.var_adeff_dn11 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn11)) + ((locals.var_pdeff_dn11 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn11))) + (assign15410_e22762 * locals.var_jswgd_t_dn11));
        locals.var_isbd_dn12 = ((((locals.var_adeff_dn12 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn12)) + ((locals.var_pdeff_dn12 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn12))) + (assign15410_e22762 * locals.var_jswgd_t_dn12));
        locals.var_isbd_dn13 = ((((locals.var_adeff_dn13 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn13)) + ((locals.var_pdeff_dn13 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn13))) + (assign15410_e22762 * locals.var_jswgd_t_dn13));
        locals.var_isbd_dn14 = ((((locals.var_adeff_dn14 * locals.var_jsd_t) + (locals.var_adeff * locals.var_jsd_t_dn14)) + ((locals.var_pdeff_dn14 * locals.var_jswd_t) + (locals.var_pdeff * locals.var_jswd_t_dn14))) + (assign15410_e22762 * locals.var_jswgd_t_dn14));
        locals.var_isbd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign15420_e22768: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard487 = assign15420_e22768;
        locals.var_guard487_rv = 0.0;

        let (assign15430_e22774, assign15430_e22774_d_n4,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15430_e22772: f64 = (locals.var_vtm * p.p726);
        (assign15430_e22772, (locals.var_vtm_dn4 * p.p726),)
    } else {
        (locals.var_nvtmd, locals.var_nvtmd_dn4,)
    }
};
        locals.var_nvtmd = assign15430_e22774;
        locals.var_nvtmd_dn4 = assign15430_e22774_d_n4;
        locals.var_nvtmd_rv = 0.0;

        let (assign15440_e22784, assign15440_e22784_d_n4,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15440_e22777: f64 = (-p.p732);
        let assign15440_e22779: f64 = (assign15440_e22777 / locals.var_nvtmd);
        let assign15440_e22780: f64 = { let limited_exp_arg = assign15440_e22779; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15440_e22782: f64 = (assign15440_e22780 * p.p734);
        (assign15440_e22782, (({ let limited_exp_arg = assign15440_e22779; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-((assign15440_e22777 * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd)))) * p.p734),)
    } else {
        (locals.var_xexpbvd, locals.var_xexpbvd_dn4,)
    }
};
        locals.var_xexpbvd = assign15440_e22784;
        locals.var_xexpbvd_dn4 = assign15440_e22784_d_n4;
        locals.var_xexpbvd_rv = 0.0;

        let (assign15450_e22792, assign15450_e22792_d_n0, assign15450_e22792_d_n2, assign15450_e22792_d_n3, assign15450_e22792_d_n4, assign15450_e22792_d_n5, assign15450_e22792_d_n6, assign15450_e22792_d_n7, assign15450_e22792_d_n8, assign15450_e22792_d_n9, assign15450_e22792_d_n10, assign15450_e22792_d_n11, assign15450_e22792_d_n12, assign15450_e22792_d_n13, assign15450_e22792_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15450_e22788: f64 = (p.p728 / locals.var_isbd);
        let assign15450_e22790: f64 = (assign15450_e22788).max(10.0);
        (assign15450_e22790, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 }, if assign15450_e22788 >= 10.0 { (-((p.p728 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) } else { 0.0 },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15450_e22792;
        locals.var_t2_dn0 = assign15450_e22792_d_n0;
        locals.var_t2_dn2 = assign15450_e22792_d_n2;
        locals.var_t2_dn3 = assign15450_e22792_d_n3;
        locals.var_t2_dn4 = assign15450_e22792_d_n4;
        locals.var_t2_dn5 = assign15450_e22792_d_n5;
        locals.var_t2_dn6 = assign15450_e22792_d_n6;
        locals.var_t2_dn7 = assign15450_e22792_d_n7;
        locals.var_t2_dn8 = assign15450_e22792_d_n8;
        locals.var_t2_dn9 = assign15450_e22792_d_n9;
        locals.var_t2_dn10 = assign15450_e22792_d_n10;
        locals.var_t2_dn11 = assign15450_e22792_d_n11;
        locals.var_t2_dn12 = assign15450_e22792_d_n12;
        locals.var_t2_dn13 = assign15450_e22792_d_n13;
        locals.var_t2_dn14 = assign15450_e22792_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15460_e22800, assign15460_e22800_d_n0, assign15460_e22800_d_n2, assign15460_e22800_d_n3, assign15460_e22800_d_n4, assign15460_e22800_d_n5, assign15460_e22800_d_n6, assign15460_e22800_d_n7, assign15460_e22800_d_n8, assign15460_e22800_d_n9, assign15460_e22800_d_n10, assign15460_e22800_d_n11, assign15460_e22800_d_n12, assign15460_e22800_d_n13, assign15460_e22800_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15460_e22796: f64 = (1.0 + locals.var_t2);
        let assign15460_e22798: f64 = (assign15460_e22796 - locals.var_xexpbvd);
        (assign15460_e22798, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, (locals.var_t2_dn4 - locals.var_xexpbvd_dn4), locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    } else {
        (locals.var_tb, locals.var_tb_dn0, locals.var_tb_dn2, locals.var_tb_dn3, locals.var_tb_dn4, locals.var_tb_dn5, locals.var_tb_dn6, locals.var_tb_dn7, locals.var_tb_dn8, locals.var_tb_dn9, locals.var_tb_dn10, locals.var_tb_dn11, locals.var_tb_dn12, locals.var_tb_dn13, locals.var_tb_dn14,)
    }
};
        locals.var_tb = assign15460_e22800;
        locals.var_tb_dn0 = assign15460_e22800_d_n0;
        locals.var_tb_dn2 = assign15460_e22800_d_n2;
        locals.var_tb_dn3 = assign15460_e22800_d_n3;
        locals.var_tb_dn4 = assign15460_e22800_d_n4;
        locals.var_tb_dn5 = assign15460_e22800_d_n5;
        locals.var_tb_dn6 = assign15460_e22800_d_n6;
        locals.var_tb_dn7 = assign15460_e22800_d_n7;
        locals.var_tb_dn8 = assign15460_e22800_d_n8;
        locals.var_tb_dn9 = assign15460_e22800_d_n9;
        locals.var_tb_dn10 = assign15460_e22800_d_n10;
        locals.var_tb_dn11 = assign15460_e22800_d_n11;
        locals.var_tb_dn12 = assign15460_e22800_d_n12;
        locals.var_tb_dn13 = assign15460_e22800_d_n13;
        locals.var_tb_dn14 = assign15460_e22800_d_n14;
        locals.var_tb_rv = 0.0;

        let (assign15470_e22820, assign15470_e22820_d_n0, assign15470_e22820_d_n2, assign15470_e22820_d_n3, assign15470_e22820_d_n4, assign15470_e22820_d_n5, assign15470_e22820_d_n6, assign15470_e22820_d_n7, assign15470_e22820_d_n8, assign15470_e22820_d_n9, assign15470_e22820_d_n10, assign15470_e22820_d_n11, assign15470_e22820_d_n12, assign15470_e22820_d_n13, assign15470_e22820_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15470_e22807: f64 = (locals.var_tb * locals.var_tb);
        let assign15470_e22810: f64 = (4.0 * locals.var_xexpbvd);
        let assign15470_e22811: f64 = (assign15470_e22807 + assign15470_e22810);
        let assign15470_e22812: f64 = (assign15470_e22811).sqrt();
        let assign15470_e22813: f64 = (locals.var_tb + assign15470_e22812);
        let assign15470_e22814: f64 = (0.5 * assign15470_e22813);
        let assign15470_e22816: f64 = (assign15470_e22814).max(1e-38);
        let assign15470_e22817: f64 = (assign15470_e22816).ln();
        let assign15470_e22818: f64 = (locals.var_nvtmd * assign15470_e22817);
        (assign15470_e22818, (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn0 + (((locals.var_tb_dn0 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn0)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn2 + (((locals.var_tb_dn2 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn2)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn3 + (((locals.var_tb_dn3 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn3)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), ((locals.var_nvtmd_dn4 * assign15470_e22817) + (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn4 + ((((locals.var_tb_dn4 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn4)) + (4.0 * locals.var_xexpbvd_dn4)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816))), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn5 + (((locals.var_tb_dn5 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn5)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn6 + (((locals.var_tb_dn6 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn6)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn7 + (((locals.var_tb_dn7 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn7)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn8 + (((locals.var_tb_dn8 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn8)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn9 + (((locals.var_tb_dn9 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn9)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn10 + (((locals.var_tb_dn10 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn10)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn11 + (((locals.var_tb_dn11 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn11)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn12 + (((locals.var_tb_dn12 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn12)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn13 + (((locals.var_tb_dn13 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn13)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)), (locals.var_nvtmd * (if assign15470_e22814 >= 1e-38 { (0.5 * (locals.var_tb_dn14 + (((locals.var_tb_dn14 * locals.var_tb) + (locals.var_tb * locals.var_tb_dn14)) / (2.0 * assign15470_e22812)))) } else { 0.0 } / assign15470_e22816)),)
    } else {
        (locals.var_vjdmfwd, locals.var_vjdmfwd_dn0, locals.var_vjdmfwd_dn2, locals.var_vjdmfwd_dn3, locals.var_vjdmfwd_dn4, locals.var_vjdmfwd_dn5, locals.var_vjdmfwd_dn6, locals.var_vjdmfwd_dn7, locals.var_vjdmfwd_dn8, locals.var_vjdmfwd_dn9, locals.var_vjdmfwd_dn10, locals.var_vjdmfwd_dn11, locals.var_vjdmfwd_dn12, locals.var_vjdmfwd_dn13, locals.var_vjdmfwd_dn14,)
    }
};
        locals.var_vjdmfwd = assign15470_e22820;
        locals.var_vjdmfwd_dn0 = assign15470_e22820_d_n0;
        locals.var_vjdmfwd_dn2 = assign15470_e22820_d_n2;
        locals.var_vjdmfwd_dn3 = assign15470_e22820_d_n3;
        locals.var_vjdmfwd_dn4 = assign15470_e22820_d_n4;
        locals.var_vjdmfwd_dn5 = assign15470_e22820_d_n5;
        locals.var_vjdmfwd_dn6 = assign15470_e22820_d_n6;
        locals.var_vjdmfwd_dn7 = assign15470_e22820_d_n7;
        locals.var_vjdmfwd_dn8 = assign15470_e22820_d_n8;
        locals.var_vjdmfwd_dn9 = assign15470_e22820_d_n9;
        locals.var_vjdmfwd_dn10 = assign15470_e22820_d_n10;
        locals.var_vjdmfwd_dn11 = assign15470_e22820_d_n11;
        locals.var_vjdmfwd_dn12 = assign15470_e22820_d_n12;
        locals.var_vjdmfwd_dn13 = assign15470_e22820_d_n13;
        locals.var_vjdmfwd_dn14 = assign15470_e22820_d_n14;
        locals.var_vjdmfwd_rv = 0.0;

        let (assign15480_e22827, assign15480_e22827_d_n0, assign15480_e22827_d_n2, assign15480_e22827_d_n3, assign15480_e22827_d_n4, assign15480_e22827_d_n5, assign15480_e22827_d_n6, assign15480_e22827_d_n7, assign15480_e22827_d_n8, assign15480_e22827_d_n9, assign15480_e22827_d_n10, assign15480_e22827_d_n11, assign15480_e22827_d_n12, assign15480_e22827_d_n13, assign15480_e22827_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15480_e22824: f64 = (locals.var_vjdmfwd / locals.var_nvtmd);
        let assign15480_e22825: f64 = { let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign15480_e22825, ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn0 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn2 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn3 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((locals.var_vjdmfwd_dn4 * locals.var_nvtmd) - (locals.var_vjdmfwd * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd))), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn5 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn6 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn7 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn8 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn9 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn10 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn11 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn12 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn13 / locals.var_nvtmd)), ({ let limited_exp_arg = assign15480_e22824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_vjdmfwd_dn14 / locals.var_nvtmd)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15480_e22827;
        locals.var_t0_dn0 = assign15480_e22827_d_n0;
        locals.var_t0_dn2 = assign15480_e22827_d_n2;
        locals.var_t0_dn3 = assign15480_e22827_d_n3;
        locals.var_t0_dn4 = assign15480_e22827_d_n4;
        locals.var_t0_dn5 = assign15480_e22827_d_n5;
        locals.var_t0_dn6 = assign15480_e22827_d_n6;
        locals.var_t0_dn7 = assign15480_e22827_d_n7;
        locals.var_t0_dn8 = assign15480_e22827_d_n8;
        locals.var_t0_dn9 = assign15480_e22827_d_n9;
        locals.var_t0_dn10 = assign15480_e22827_d_n10;
        locals.var_t0_dn11 = assign15480_e22827_d_n11;
        locals.var_t0_dn12 = assign15480_e22827_d_n12;
        locals.var_t0_dn13 = assign15480_e22827_d_n13;
        locals.var_t0_dn14 = assign15480_e22827_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15490_e22841, assign15490_e22841_d_n0, assign15490_e22841_d_n2, assign15490_e22841_d_n3, assign15490_e22841_d_n4, assign15490_e22841_d_n5, assign15490_e22841_d_n6, assign15490_e22841_d_n7, assign15490_e22841_d_n8, assign15490_e22841_d_n9, assign15490_e22841_d_n10, assign15490_e22841_d_n11, assign15490_e22841_d_n12, assign15490_e22841_d_n13, assign15490_e22841_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15490_e22833: f64 = (locals.var_xexpbvd / locals.var_t0);
        let assign15490_e22834: f64 = (locals.var_t0 - assign15490_e22833);
        let assign15490_e22836: f64 = (assign15490_e22834 + locals.var_xexpbvd);
        let assign15490_e22838: f64 = (assign15490_e22836 - 1.0);
        let assign15490_e22839: f64 = (locals.var_isbd * assign15490_e22838);
        (assign15490_e22839, ((locals.var_isbd_dn0 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn0 - (-((locals.var_xexpbvd * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn2 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn2 - (-((locals.var_xexpbvd * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn3 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn3 - (-((locals.var_xexpbvd * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn4 * assign15490_e22838) + (locals.var_isbd * ((locals.var_t0_dn4 - (((locals.var_xexpbvd_dn4 * locals.var_t0) - (locals.var_xexpbvd * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))) + locals.var_xexpbvd_dn4))), ((locals.var_isbd_dn5 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn5 - (-((locals.var_xexpbvd * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn6 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn6 - (-((locals.var_xexpbvd * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn7 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn7 - (-((locals.var_xexpbvd * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn8 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn8 - (-((locals.var_xexpbvd * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn9 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn9 - (-((locals.var_xexpbvd * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn10 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn10 - (-((locals.var_xexpbvd * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn11 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn11 - (-((locals.var_xexpbvd * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn12 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn12 - (-((locals.var_xexpbvd * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn13 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn13 - (-((locals.var_xexpbvd * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))), ((locals.var_isbd_dn14 * assign15490_e22838) + (locals.var_isbd * (locals.var_t0_dn14 - (-((locals.var_xexpbvd * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))),)
    } else {
        (locals.var_ivjdmfwd, locals.var_ivjdmfwd_dn0, locals.var_ivjdmfwd_dn2, locals.var_ivjdmfwd_dn3, locals.var_ivjdmfwd_dn4, locals.var_ivjdmfwd_dn5, locals.var_ivjdmfwd_dn6, locals.var_ivjdmfwd_dn7, locals.var_ivjdmfwd_dn8, locals.var_ivjdmfwd_dn9, locals.var_ivjdmfwd_dn10, locals.var_ivjdmfwd_dn11, locals.var_ivjdmfwd_dn12, locals.var_ivjdmfwd_dn13, locals.var_ivjdmfwd_dn14,)
    }
};
        locals.var_ivjdmfwd = assign15490_e22841;
        locals.var_ivjdmfwd_dn0 = assign15490_e22841_d_n0;
        locals.var_ivjdmfwd_dn2 = assign15490_e22841_d_n2;
        locals.var_ivjdmfwd_dn3 = assign15490_e22841_d_n3;
        locals.var_ivjdmfwd_dn4 = assign15490_e22841_d_n4;
        locals.var_ivjdmfwd_dn5 = assign15490_e22841_d_n5;
        locals.var_ivjdmfwd_dn6 = assign15490_e22841_d_n6;
        locals.var_ivjdmfwd_dn7 = assign15490_e22841_d_n7;
        locals.var_ivjdmfwd_dn8 = assign15490_e22841_d_n8;
        locals.var_ivjdmfwd_dn9 = assign15490_e22841_d_n9;
        locals.var_ivjdmfwd_dn10 = assign15490_e22841_d_n10;
        locals.var_ivjdmfwd_dn11 = assign15490_e22841_d_n11;
        locals.var_ivjdmfwd_dn12 = assign15490_e22841_d_n12;
        locals.var_ivjdmfwd_dn13 = assign15490_e22841_d_n13;
        locals.var_ivjdmfwd_dn14 = assign15490_e22841_d_n14;
        locals.var_ivjdmfwd_rv = 0.0;

        let (assign15500_e22853, assign15500_e22853_d_n0, assign15500_e22853_d_n2, assign15500_e22853_d_n3, assign15500_e22853_d_n4, assign15500_e22853_d_n5, assign15500_e22853_d_n6, assign15500_e22853_d_n7, assign15500_e22853_d_n8, assign15500_e22853_d_n9, assign15500_e22853_d_n10, assign15500_e22853_d_n11, assign15500_e22853_d_n12, assign15500_e22853_d_n13, assign15500_e22853_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15500_e22847: f64 = (locals.var_xexpbvd / locals.var_t0);
        let assign15500_e22848: f64 = (locals.var_t0 + assign15500_e22847);
        let assign15500_e22849: f64 = (locals.var_isbd * assign15500_e22848);
        let assign15500_e22851: f64 = (assign15500_e22849 / locals.var_nvtmd);
        (assign15500_e22851, (((locals.var_isbd_dn0 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn0 + (-((locals.var_xexpbvd * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn2 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn2 + (-((locals.var_xexpbvd * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn3 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn3 + (-((locals.var_xexpbvd * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((((locals.var_isbd_dn4 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn4 + (((locals.var_xexpbvd_dn4 * locals.var_t0) - (locals.var_xexpbvd * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0))))) * locals.var_nvtmd) - (assign15500_e22849 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)), (((locals.var_isbd_dn5 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn5 + (-((locals.var_xexpbvd * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn6 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn6 + (-((locals.var_xexpbvd * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn7 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn7 + (-((locals.var_xexpbvd * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn8 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn8 + (-((locals.var_xexpbvd * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn9 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn9 + (-((locals.var_xexpbvd * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn10 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn10 + (-((locals.var_xexpbvd * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn11 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn11 + (-((locals.var_xexpbvd * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn12 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn12 + (-((locals.var_xexpbvd * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn13 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn13 + (-((locals.var_xexpbvd * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd), (((locals.var_isbd_dn14 * assign15500_e22848) + (locals.var_isbd * (locals.var_t0_dn14 + (-((locals.var_xexpbvd * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)))))) / locals.var_nvtmd),)
    } else {
        (locals.var_dslpfwd, locals.var_dslpfwd_dn0, locals.var_dslpfwd_dn2, locals.var_dslpfwd_dn3, locals.var_dslpfwd_dn4, locals.var_dslpfwd_dn5, locals.var_dslpfwd_dn6, locals.var_dslpfwd_dn7, locals.var_dslpfwd_dn8, locals.var_dslpfwd_dn9, locals.var_dslpfwd_dn10, locals.var_dslpfwd_dn11, locals.var_dslpfwd_dn12, locals.var_dslpfwd_dn13, locals.var_dslpfwd_dn14,)
    }
};
        locals.var_dslpfwd = assign15500_e22853;
        locals.var_dslpfwd_dn0 = assign15500_e22853_d_n0;
        locals.var_dslpfwd_dn2 = assign15500_e22853_d_n2;
        locals.var_dslpfwd_dn3 = assign15500_e22853_d_n3;
        locals.var_dslpfwd_dn4 = assign15500_e22853_d_n4;
        locals.var_dslpfwd_dn5 = assign15500_e22853_d_n5;
        locals.var_dslpfwd_dn6 = assign15500_e22853_d_n6;
        locals.var_dslpfwd_dn7 = assign15500_e22853_d_n7;
        locals.var_dslpfwd_dn8 = assign15500_e22853_d_n8;
        locals.var_dslpfwd_dn9 = assign15500_e22853_d_n9;
        locals.var_dslpfwd_dn10 = assign15500_e22853_d_n10;
        locals.var_dslpfwd_dn11 = assign15500_e22853_d_n11;
        locals.var_dslpfwd_dn12 = assign15500_e22853_d_n12;
        locals.var_dslpfwd_dn13 = assign15500_e22853_d_n13;
        locals.var_dslpfwd_dn14 = assign15500_e22853_d_n14;
        locals.var_dslpfwd_rv = 0.0;

        let (assign15510_e22918, assign15510_e22918_d_n0, assign15510_e22918_d_n2, assign15510_e22918_d_n3, assign15510_e22918_d_n4, assign15510_e22918_d_n5, assign15510_e22918_d_n6, assign15510_e22918_d_n7, assign15510_e22918_d_n8, assign15510_e22918_d_n9, assign15510_e22918_d_n10, assign15510_e22918_d_n11, assign15510_e22918_d_n12, assign15510_e22918_d_n13, assign15510_e22918_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15510_e22857: f64 = (p.p730 / locals.var_isbd);
        let assign15510_e22859: f64 = (assign15510_e22857 - 10.0);
        let assign15510_e22861: f64 = (-10000.0);
        let assign15510_e22863: f64 = (assign15510_e22861 * 0.001);
        let (assign15510_e22914, assign15510_e22914_d_n0, assign15510_e22914_d_n2, assign15510_e22914_d_n3, assign15510_e22914_d_n4, assign15510_e22914_d_n5, assign15510_e22914_d_n6, assign15510_e22914_d_n7, assign15510_e22914_d_n8, assign15510_e22914_d_n9, assign15510_e22914_d_n10, assign15510_e22914_d_n11, assign15510_e22914_d_n12, assign15510_e22914_d_n13, assign15510_e22914_d_n14,) = {
            if (!(assign15510_e22859 < assign15510_e22863)) {
                let assign15510_e22869: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22871: f64 = (assign15510_e22869 - 10.0);
                let assign15510_e22874: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22876: f64 = (assign15510_e22874 - 10.0);
                let assign15510_e22879: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22881: f64 = (assign15510_e22879 - 10.0);
                let assign15510_e22882: f64 = (assign15510_e22876 * assign15510_e22881);
                let assign15510_e22885: f64 = (4.0 * 0.001);
                let assign15510_e22887: f64 = (assign15510_e22885 * 0.001);
                let assign15510_e22888: f64 = (assign15510_e22882 + assign15510_e22887);
                let assign15510_e22889: f64 = (assign15510_e22888).sqrt();
                let assign15510_e22890: f64 = (assign15510_e22871 + assign15510_e22889);
                let assign15510_e22891: f64 = (0.5 * assign15510_e22890);
                (assign15510_e22891, (0.5 * ((-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))), (0.5 * ((-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) + ((((-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))) * assign15510_e22881) + (assign15510_e22876 * (-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd))))) / (2.0 * assign15510_e22889)))),)
            } else {
                let assign15510_e22894: f64 = (p.p730 / locals.var_isbd);
                let assign15510_e22896: f64 = (assign15510_e22894 - 10.0);
                let assign15510_e22898: f64 = (-10000.0);
                let assign15510_e22900: f64 = (assign15510_e22898 * 0.001);
                let (assign15510_e22913, assign15510_e22913_d_n0, assign15510_e22913_d_n2, assign15510_e22913_d_n3, assign15510_e22913_d_n4, assign15510_e22913_d_n5, assign15510_e22913_d_n6, assign15510_e22913_d_n7, assign15510_e22913_d_n8, assign15510_e22913_d_n9, assign15510_e22913_d_n10, assign15510_e22913_d_n11, assign15510_e22913_d_n12, assign15510_e22913_d_n13, assign15510_e22913_d_n14,) = {
                    if (assign15510_e22896 < assign15510_e22900) {
                        let assign15510_e22903: f64 = (-0.001);
                        let assign15510_e22905: f64 = (assign15510_e22903 * 0.001);
                        let assign15510_e22908: f64 = (p.p730 / locals.var_isbd);
                        let assign15510_e22910: f64 = (assign15510_e22908 - 10.0);
                        let assign15510_e22911: f64 = (assign15510_e22905 / assign15510_e22910);
                        (assign15510_e22911, (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn0) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn2) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn3) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn4) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn5) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn6) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn7) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn8) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn9) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn10) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn11) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn12) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn13) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))), (-((assign15510_e22905 * (-((p.p730 * locals.var_isbd_dn14) / (locals.var_isbd * locals.var_isbd)))) / (assign15510_e22910 * assign15510_e22910))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15510_e22913, assign15510_e22913_d_n0, assign15510_e22913_d_n2, assign15510_e22913_d_n3, assign15510_e22913_d_n4, assign15510_e22913_d_n5, assign15510_e22913_d_n6, assign15510_e22913_d_n7, assign15510_e22913_d_n8, assign15510_e22913_d_n9, assign15510_e22913_d_n10, assign15510_e22913_d_n11, assign15510_e22913_d_n12, assign15510_e22913_d_n13, assign15510_e22913_d_n14,)
            }
        };
        let assign15510_e22916: f64 = (assign15510_e22914 + 10.0);
        (assign15510_e22916, assign15510_e22914_d_n0, assign15510_e22914_d_n2, assign15510_e22914_d_n3, assign15510_e22914_d_n4, assign15510_e22914_d_n5, assign15510_e22914_d_n6, assign15510_e22914_d_n7, assign15510_e22914_d_n8, assign15510_e22914_d_n9, assign15510_e22914_d_n10, assign15510_e22914_d_n11, assign15510_e22914_d_n12, assign15510_e22914_d_n13, assign15510_e22914_d_n14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15510_e22918;
        locals.var_t2_dn0 = assign15510_e22918_d_n0;
        locals.var_t2_dn2 = assign15510_e22918_d_n2;
        locals.var_t2_dn3 = assign15510_e22918_d_n3;
        locals.var_t2_dn4 = assign15510_e22918_d_n4;
        locals.var_t2_dn5 = assign15510_e22918_d_n5;
        locals.var_t2_dn6 = assign15510_e22918_d_n6;
        locals.var_t2_dn7 = assign15510_e22918_d_n7;
        locals.var_t2_dn8 = assign15510_e22918_d_n8;
        locals.var_t2_dn9 = assign15510_e22918_d_n9;
        locals.var_t2_dn10 = assign15510_e22918_d_n10;
        locals.var_t2_dn11 = assign15510_e22918_d_n11;
        locals.var_t2_dn12 = assign15510_e22918_d_n12;
        locals.var_t2_dn13 = assign15510_e22918_d_n13;
        locals.var_t2_dn14 = assign15510_e22918_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign15520_e22934, assign15520_e22934_d_n0, assign15520_e22934_d_n2, assign15520_e22934_d_n3, assign15520_e22934_d_n4, assign15520_e22934_d_n5, assign15520_e22934_d_n6, assign15520_e22934_d_n7, assign15520_e22934_d_n8, assign15520_e22934_d_n9, assign15520_e22934_d_n10, assign15520_e22934_d_n11, assign15520_e22934_d_n12, assign15520_e22934_d_n13, assign15520_e22934_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15520_e22921: f64 = (-p.p732);
        let assign15520_e22925: f64 = (locals.var_t2 - 1.0);
        let assign15520_e22927: f64 = (assign15520_e22925 / p.p734);
        let assign15520_e22929: f64 = (assign15520_e22927).max(1e-38);
        let assign15520_e22930: f64 = (assign15520_e22929).ln();
        let assign15520_e22931: f64 = (locals.var_nvtmd * assign15520_e22930);
        let assign15520_e22932: f64 = (assign15520_e22921 - assign15520_e22931);
        (assign15520_e22932, (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn0 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn2 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn3 / p.p734) } else { 0.0 } / assign15520_e22929))), (-((locals.var_nvtmd_dn4 * assign15520_e22930) + (locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn4 / p.p734) } else { 0.0 } / assign15520_e22929)))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn5 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn6 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn7 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn8 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn9 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn10 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn11 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn12 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn13 / p.p734) } else { 0.0 } / assign15520_e22929))), (-(locals.var_nvtmd * (if assign15520_e22927 >= 1e-38 { (locals.var_t2_dn14 / p.p734) } else { 0.0 } / assign15520_e22929))),)
    } else {
        (locals.var_vjdmrev, locals.var_vjdmrev_dn0, locals.var_vjdmrev_dn2, locals.var_vjdmrev_dn3, locals.var_vjdmrev_dn4, locals.var_vjdmrev_dn5, locals.var_vjdmrev_dn6, locals.var_vjdmrev_dn7, locals.var_vjdmrev_dn8, locals.var_vjdmrev_dn9, locals.var_vjdmrev_dn10, locals.var_vjdmrev_dn11, locals.var_vjdmrev_dn12, locals.var_vjdmrev_dn13, locals.var_vjdmrev_dn14,)
    }
};
        locals.var_vjdmrev = assign15520_e22934;
        locals.var_vjdmrev_dn0 = assign15520_e22934_d_n0;
        locals.var_vjdmrev_dn2 = assign15520_e22934_d_n2;
        locals.var_vjdmrev_dn3 = assign15520_e22934_d_n3;
        locals.var_vjdmrev_dn4 = assign15520_e22934_d_n4;
        locals.var_vjdmrev_dn5 = assign15520_e22934_d_n5;
        locals.var_vjdmrev_dn6 = assign15520_e22934_d_n6;
        locals.var_vjdmrev_dn7 = assign15520_e22934_d_n7;
        locals.var_vjdmrev_dn8 = assign15520_e22934_d_n8;
        locals.var_vjdmrev_dn9 = assign15520_e22934_d_n9;
        locals.var_vjdmrev_dn10 = assign15520_e22934_d_n10;
        locals.var_vjdmrev_dn11 = assign15520_e22934_d_n11;
        locals.var_vjdmrev_dn12 = assign15520_e22934_d_n12;
        locals.var_vjdmrev_dn13 = assign15520_e22934_d_n13;
        locals.var_vjdmrev_dn14 = assign15520_e22934_d_n14;
        locals.var_vjdmrev_rv = 0.0;

        let (assign15530_e22946, assign15530_e22946_d_n0, assign15530_e22946_d_n2, assign15530_e22946_d_n3, assign15530_e22946_d_n4, assign15530_e22946_d_n5, assign15530_e22946_d_n6, assign15530_e22946_d_n7, assign15530_e22946_d_n8, assign15530_e22946_d_n9, assign15530_e22946_d_n10, assign15530_e22946_d_n11, assign15530_e22946_d_n12, assign15530_e22946_d_n13, assign15530_e22946_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15530_e22939: f64 = (p.p732 + locals.var_vjdmrev);
        let assign15530_e22940: f64 = (-assign15530_e22939);
        let assign15530_e22942: f64 = (assign15530_e22940 / locals.var_nvtmd);
        let assign15530_e22943: f64 = { let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15530_e22944: f64 = (p.p734 * assign15530_e22943);
        (assign15530_e22944, (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn0) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn2) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn3) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((-locals.var_vjdmrev_dn4) * locals.var_nvtmd) - (assign15530_e22940 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn5) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn6) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn7) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn8) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn9) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn10) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn11) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn12) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn13) / locals.var_nvtmd))), (p.p734 * ({ let limited_exp_arg = assign15530_e22942; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((-locals.var_vjdmrev_dn14) / locals.var_nvtmd))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15530_e22946;
        locals.var_t1_dn0 = assign15530_e22946_d_n0;
        locals.var_t1_dn2 = assign15530_e22946_d_n2;
        locals.var_t1_dn3 = assign15530_e22946_d_n3;
        locals.var_t1_dn4 = assign15530_e22946_d_n4;
        locals.var_t1_dn5 = assign15530_e22946_d_n5;
        locals.var_t1_dn6 = assign15530_e22946_d_n6;
        locals.var_t1_dn7 = assign15530_e22946_d_n7;
        locals.var_t1_dn8 = assign15530_e22946_d_n8;
        locals.var_t1_dn9 = assign15530_e22946_d_n9;
        locals.var_t1_dn10 = assign15530_e22946_d_n10;
        locals.var_t1_dn11 = assign15530_e22946_d_n11;
        locals.var_t1_dn12 = assign15530_e22946_d_n12;
        locals.var_t1_dn13 = assign15530_e22946_d_n13;
        locals.var_t1_dn14 = assign15530_e22946_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15540_e22954, assign15540_e22954_d_n0, assign15540_e22954_d_n2, assign15540_e22954_d_n3, assign15540_e22954_d_n4, assign15540_e22954_d_n5, assign15540_e22954_d_n6, assign15540_e22954_d_n7, assign15540_e22954_d_n8, assign15540_e22954_d_n9, assign15540_e22954_d_n10, assign15540_e22954_d_n11, assign15540_e22954_d_n12, assign15540_e22954_d_n13, assign15540_e22954_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15540_e22951: f64 = (1.0 + locals.var_t1);
        let assign15540_e22952: f64 = (locals.var_isbd * assign15540_e22951);
        (assign15540_e22952, ((locals.var_isbd_dn0 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn0)), ((locals.var_isbd_dn2 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn2)), ((locals.var_isbd_dn3 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn3)), ((locals.var_isbd_dn4 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn4)), ((locals.var_isbd_dn5 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn5)), ((locals.var_isbd_dn6 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn6)), ((locals.var_isbd_dn7 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn7)), ((locals.var_isbd_dn8 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn8)), ((locals.var_isbd_dn9 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn9)), ((locals.var_isbd_dn10 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn10)), ((locals.var_isbd_dn11 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn11)), ((locals.var_isbd_dn12 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn12)), ((locals.var_isbd_dn13 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn13)), ((locals.var_isbd_dn14 * assign15540_e22951) + (locals.var_isbd * locals.var_t1_dn14)),)
    } else {
        (locals.var_ivjdmrev, locals.var_ivjdmrev_dn0, locals.var_ivjdmrev_dn2, locals.var_ivjdmrev_dn3, locals.var_ivjdmrev_dn4, locals.var_ivjdmrev_dn5, locals.var_ivjdmrev_dn6, locals.var_ivjdmrev_dn7, locals.var_ivjdmrev_dn8, locals.var_ivjdmrev_dn9, locals.var_ivjdmrev_dn10, locals.var_ivjdmrev_dn11, locals.var_ivjdmrev_dn12, locals.var_ivjdmrev_dn13, locals.var_ivjdmrev_dn14,)
    }
};
        locals.var_ivjdmrev = assign15540_e22954;
        locals.var_ivjdmrev_dn0 = assign15540_e22954_d_n0;
        locals.var_ivjdmrev_dn2 = assign15540_e22954_d_n2;
        locals.var_ivjdmrev_dn3 = assign15540_e22954_d_n3;
        locals.var_ivjdmrev_dn4 = assign15540_e22954_d_n4;
        locals.var_ivjdmrev_dn5 = assign15540_e22954_d_n5;
        locals.var_ivjdmrev_dn6 = assign15540_e22954_d_n6;
        locals.var_ivjdmrev_dn7 = assign15540_e22954_d_n7;
        locals.var_ivjdmrev_dn8 = assign15540_e22954_d_n8;
        locals.var_ivjdmrev_dn9 = assign15540_e22954_d_n9;
        locals.var_ivjdmrev_dn10 = assign15540_e22954_d_n10;
        locals.var_ivjdmrev_dn11 = assign15540_e22954_d_n11;
        locals.var_ivjdmrev_dn12 = assign15540_e22954_d_n12;
        locals.var_ivjdmrev_dn13 = assign15540_e22954_d_n13;
        locals.var_ivjdmrev_dn14 = assign15540_e22954_d_n14;
        locals.var_ivjdmrev_rv = 0.0;

        let (assign15550_e22963, assign15550_e22963_d_n0, assign15550_e22963_d_n2, assign15550_e22963_d_n3, assign15550_e22963_d_n4, assign15550_e22963_d_n5, assign15550_e22963_d_n6, assign15550_e22963_d_n7, assign15550_e22963_d_n8, assign15550_e22963_d_n9, assign15550_e22963_d_n10, assign15550_e22963_d_n11, assign15550_e22963_d_n12, assign15550_e22963_d_n13, assign15550_e22963_d_n14,) = {
    if (locals.var_guard487 != 0.0) {
        let assign15550_e22957: f64 = (-locals.var_isbd);
        let assign15550_e22959: f64 = (assign15550_e22957 * locals.var_t1);
        let assign15550_e22961: f64 = (assign15550_e22959 / locals.var_nvtmd);
        (assign15550_e22961, ((((-locals.var_isbd_dn0) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn0)) / locals.var_nvtmd), ((((-locals.var_isbd_dn2) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn2)) / locals.var_nvtmd), ((((-locals.var_isbd_dn3) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn3)) / locals.var_nvtmd), ((((((-locals.var_isbd_dn4) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn4)) * locals.var_nvtmd) - (assign15550_e22959 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)), ((((-locals.var_isbd_dn5) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn5)) / locals.var_nvtmd), ((((-locals.var_isbd_dn6) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn6)) / locals.var_nvtmd), ((((-locals.var_isbd_dn7) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn7)) / locals.var_nvtmd), ((((-locals.var_isbd_dn8) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn8)) / locals.var_nvtmd), ((((-locals.var_isbd_dn9) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn9)) / locals.var_nvtmd), ((((-locals.var_isbd_dn10) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn10)) / locals.var_nvtmd), ((((-locals.var_isbd_dn11) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn11)) / locals.var_nvtmd), ((((-locals.var_isbd_dn12) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn12)) / locals.var_nvtmd), ((((-locals.var_isbd_dn13) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn13)) / locals.var_nvtmd), ((((-locals.var_isbd_dn14) * locals.var_t1) + (assign15550_e22957 * locals.var_t1_dn14)) / locals.var_nvtmd),)
    } else {
        (locals.var_dslprev, locals.var_dslprev_dn0, locals.var_dslprev_dn2, locals.var_dslprev_dn3, locals.var_dslprev_dn4, locals.var_dslprev_dn5, locals.var_dslprev_dn6, locals.var_dslprev_dn7, locals.var_dslprev_dn8, locals.var_dslprev_dn9, locals.var_dslprev_dn10, locals.var_dslprev_dn11, locals.var_dslprev_dn12, locals.var_dslprev_dn13, locals.var_dslprev_dn14,)
    }
};
        locals.var_dslprev = assign15550_e22963;
        locals.var_dslprev_dn0 = assign15550_e22963_d_n0;
        locals.var_dslprev_dn2 = assign15550_e22963_d_n2;
        locals.var_dslprev_dn3 = assign15550_e22963_d_n3;
        locals.var_dslprev_dn4 = assign15550_e22963_d_n4;
        locals.var_dslprev_dn5 = assign15550_e22963_d_n5;
        locals.var_dslprev_dn6 = assign15550_e22963_d_n6;
        locals.var_dslprev_dn7 = assign15550_e22963_d_n7;
        locals.var_dslprev_dn8 = assign15550_e22963_d_n8;
        locals.var_dslprev_dn9 = assign15550_e22963_d_n9;
        locals.var_dslprev_dn10 = assign15550_e22963_d_n10;
        locals.var_dslprev_dn11 = assign15550_e22963_d_n11;
        locals.var_dslprev_dn12 = assign15550_e22963_d_n12;
        locals.var_dslprev_dn13 = assign15550_e22963_d_n13;
        locals.var_dslprev_dn14 = assign15550_e22963_d_n14;
        locals.var_dslprev_rv = 0.0;

        let (assign15560_e22968, assign15560_e22968_d_n4,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_nvtmd, locals.var_nvtmd_dn4,)
    }
};
        locals.var_nvtmd = assign15560_e22968;
        locals.var_nvtmd_dn4 = assign15560_e22968_d_n4;
        locals.var_nvtmd_rv = 0.0;

        let (assign15570_e22973, assign15570_e22973_d_n4,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_xexpbvd, locals.var_xexpbvd_dn4,)
    }
};
        locals.var_xexpbvd = assign15570_e22973;
        locals.var_xexpbvd_dn4 = assign15570_e22973_d_n4;
        locals.var_xexpbvd_rv = 0.0;

        let (assign15580_e22978, assign15580_e22978_d_n0, assign15580_e22978_d_n2, assign15580_e22978_d_n3, assign15580_e22978_d_n4, assign15580_e22978_d_n5, assign15580_e22978_d_n6, assign15580_e22978_d_n7, assign15580_e22978_d_n8, assign15580_e22978_d_n9, assign15580_e22978_d_n10, assign15580_e22978_d_n11, assign15580_e22978_d_n12, assign15580_e22978_d_n13, assign15580_e22978_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjdmfwd, locals.var_vjdmfwd_dn0, locals.var_vjdmfwd_dn2, locals.var_vjdmfwd_dn3, locals.var_vjdmfwd_dn4, locals.var_vjdmfwd_dn5, locals.var_vjdmfwd_dn6, locals.var_vjdmfwd_dn7, locals.var_vjdmfwd_dn8, locals.var_vjdmfwd_dn9, locals.var_vjdmfwd_dn10, locals.var_vjdmfwd_dn11, locals.var_vjdmfwd_dn12, locals.var_vjdmfwd_dn13, locals.var_vjdmfwd_dn14,)
    }
};
        locals.var_vjdmfwd = assign15580_e22978;
        locals.var_vjdmfwd_dn0 = assign15580_e22978_d_n0;
        locals.var_vjdmfwd_dn2 = assign15580_e22978_d_n2;
        locals.var_vjdmfwd_dn3 = assign15580_e22978_d_n3;
        locals.var_vjdmfwd_dn4 = assign15580_e22978_d_n4;
        locals.var_vjdmfwd_dn5 = assign15580_e22978_d_n5;
        locals.var_vjdmfwd_dn6 = assign15580_e22978_d_n6;
        locals.var_vjdmfwd_dn7 = assign15580_e22978_d_n7;
        locals.var_vjdmfwd_dn8 = assign15580_e22978_d_n8;
        locals.var_vjdmfwd_dn9 = assign15580_e22978_d_n9;
        locals.var_vjdmfwd_dn10 = assign15580_e22978_d_n10;
        locals.var_vjdmfwd_dn11 = assign15580_e22978_d_n11;
        locals.var_vjdmfwd_dn12 = assign15580_e22978_d_n12;
        locals.var_vjdmfwd_dn13 = assign15580_e22978_d_n13;
        locals.var_vjdmfwd_dn14 = assign15580_e22978_d_n14;
        locals.var_vjdmfwd_rv = 0.0;

        let (assign15590_e22983, assign15590_e22983_d_n0, assign15590_e22983_d_n2, assign15590_e22983_d_n3, assign15590_e22983_d_n4, assign15590_e22983_d_n5, assign15590_e22983_d_n6, assign15590_e22983_d_n7, assign15590_e22983_d_n8, assign15590_e22983_d_n9, assign15590_e22983_d_n10, assign15590_e22983_d_n11, assign15590_e22983_d_n12, assign15590_e22983_d_n13, assign15590_e22983_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ivjdmfwd, locals.var_ivjdmfwd_dn0, locals.var_ivjdmfwd_dn2, locals.var_ivjdmfwd_dn3, locals.var_ivjdmfwd_dn4, locals.var_ivjdmfwd_dn5, locals.var_ivjdmfwd_dn6, locals.var_ivjdmfwd_dn7, locals.var_ivjdmfwd_dn8, locals.var_ivjdmfwd_dn9, locals.var_ivjdmfwd_dn10, locals.var_ivjdmfwd_dn11, locals.var_ivjdmfwd_dn12, locals.var_ivjdmfwd_dn13, locals.var_ivjdmfwd_dn14,)
    }
};
        locals.var_ivjdmfwd = assign15590_e22983;
        locals.var_ivjdmfwd_dn0 = assign15590_e22983_d_n0;
        locals.var_ivjdmfwd_dn2 = assign15590_e22983_d_n2;
        locals.var_ivjdmfwd_dn3 = assign15590_e22983_d_n3;
        locals.var_ivjdmfwd_dn4 = assign15590_e22983_d_n4;
        locals.var_ivjdmfwd_dn5 = assign15590_e22983_d_n5;
        locals.var_ivjdmfwd_dn6 = assign15590_e22983_d_n6;
        locals.var_ivjdmfwd_dn7 = assign15590_e22983_d_n7;
        locals.var_ivjdmfwd_dn8 = assign15590_e22983_d_n8;
        locals.var_ivjdmfwd_dn9 = assign15590_e22983_d_n9;
        locals.var_ivjdmfwd_dn10 = assign15590_e22983_d_n10;
        locals.var_ivjdmfwd_dn11 = assign15590_e22983_d_n11;
        locals.var_ivjdmfwd_dn12 = assign15590_e22983_d_n12;
        locals.var_ivjdmfwd_dn13 = assign15590_e22983_d_n13;
        locals.var_ivjdmfwd_dn14 = assign15590_e22983_d_n14;
        locals.var_ivjdmfwd_rv = 0.0;

        let (assign15600_e22988, assign15600_e22988_d_n0, assign15600_e22988_d_n2, assign15600_e22988_d_n3, assign15600_e22988_d_n4, assign15600_e22988_d_n5, assign15600_e22988_d_n6, assign15600_e22988_d_n7, assign15600_e22988_d_n8, assign15600_e22988_d_n9, assign15600_e22988_d_n10, assign15600_e22988_d_n11, assign15600_e22988_d_n12, assign15600_e22988_d_n13, assign15600_e22988_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dslpfwd, locals.var_dslpfwd_dn0, locals.var_dslpfwd_dn2, locals.var_dslpfwd_dn3, locals.var_dslpfwd_dn4, locals.var_dslpfwd_dn5, locals.var_dslpfwd_dn6, locals.var_dslpfwd_dn7, locals.var_dslpfwd_dn8, locals.var_dslpfwd_dn9, locals.var_dslpfwd_dn10, locals.var_dslpfwd_dn11, locals.var_dslpfwd_dn12, locals.var_dslpfwd_dn13, locals.var_dslpfwd_dn14,)
    }
};
        locals.var_dslpfwd = assign15600_e22988;
        locals.var_dslpfwd_dn0 = assign15600_e22988_d_n0;
        locals.var_dslpfwd_dn2 = assign15600_e22988_d_n2;
        locals.var_dslpfwd_dn3 = assign15600_e22988_d_n3;
        locals.var_dslpfwd_dn4 = assign15600_e22988_d_n4;
        locals.var_dslpfwd_dn5 = assign15600_e22988_d_n5;
        locals.var_dslpfwd_dn6 = assign15600_e22988_d_n6;
        locals.var_dslpfwd_dn7 = assign15600_e22988_d_n7;
        locals.var_dslpfwd_dn8 = assign15600_e22988_d_n8;
        locals.var_dslpfwd_dn9 = assign15600_e22988_d_n9;
        locals.var_dslpfwd_dn10 = assign15600_e22988_d_n10;
        locals.var_dslpfwd_dn11 = assign15600_e22988_d_n11;
        locals.var_dslpfwd_dn12 = assign15600_e22988_d_n12;
        locals.var_dslpfwd_dn13 = assign15600_e22988_d_n13;
        locals.var_dslpfwd_dn14 = assign15600_e22988_d_n14;
        locals.var_dslpfwd_rv = 0.0;

        let (assign15610_e22993, assign15610_e22993_d_n0, assign15610_e22993_d_n2, assign15610_e22993_d_n3, assign15610_e22993_d_n4, assign15610_e22993_d_n5, assign15610_e22993_d_n6, assign15610_e22993_d_n7, assign15610_e22993_d_n8, assign15610_e22993_d_n9, assign15610_e22993_d_n10, assign15610_e22993_d_n11, assign15610_e22993_d_n12, assign15610_e22993_d_n13, assign15610_e22993_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vjdmrev, locals.var_vjdmrev_dn0, locals.var_vjdmrev_dn2, locals.var_vjdmrev_dn3, locals.var_vjdmrev_dn4, locals.var_vjdmrev_dn5, locals.var_vjdmrev_dn6, locals.var_vjdmrev_dn7, locals.var_vjdmrev_dn8, locals.var_vjdmrev_dn9, locals.var_vjdmrev_dn10, locals.var_vjdmrev_dn11, locals.var_vjdmrev_dn12, locals.var_vjdmrev_dn13, locals.var_vjdmrev_dn14,)
    }
};
        locals.var_vjdmrev = assign15610_e22993;
        locals.var_vjdmrev_dn0 = assign15610_e22993_d_n0;
        locals.var_vjdmrev_dn2 = assign15610_e22993_d_n2;
        locals.var_vjdmrev_dn3 = assign15610_e22993_d_n3;
        locals.var_vjdmrev_dn4 = assign15610_e22993_d_n4;
        locals.var_vjdmrev_dn5 = assign15610_e22993_d_n5;
        locals.var_vjdmrev_dn6 = assign15610_e22993_d_n6;
        locals.var_vjdmrev_dn7 = assign15610_e22993_d_n7;
        locals.var_vjdmrev_dn8 = assign15610_e22993_d_n8;
        locals.var_vjdmrev_dn9 = assign15610_e22993_d_n9;
        locals.var_vjdmrev_dn10 = assign15610_e22993_d_n10;
        locals.var_vjdmrev_dn11 = assign15610_e22993_d_n11;
        locals.var_vjdmrev_dn12 = assign15610_e22993_d_n12;
        locals.var_vjdmrev_dn13 = assign15610_e22993_d_n13;
        locals.var_vjdmrev_dn14 = assign15610_e22993_d_n14;
        locals.var_vjdmrev_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15620_e22998, assign15620_e22998_d_n0, assign15620_e22998_d_n2, assign15620_e22998_d_n3, assign15620_e22998_d_n4, assign15620_e22998_d_n5, assign15620_e22998_d_n6, assign15620_e22998_d_n7, assign15620_e22998_d_n8, assign15620_e22998_d_n9, assign15620_e22998_d_n10, assign15620_e22998_d_n11, assign15620_e22998_d_n12, assign15620_e22998_d_n13, assign15620_e22998_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ivjdmrev, locals.var_ivjdmrev_dn0, locals.var_ivjdmrev_dn2, locals.var_ivjdmrev_dn3, locals.var_ivjdmrev_dn4, locals.var_ivjdmrev_dn5, locals.var_ivjdmrev_dn6, locals.var_ivjdmrev_dn7, locals.var_ivjdmrev_dn8, locals.var_ivjdmrev_dn9, locals.var_ivjdmrev_dn10, locals.var_ivjdmrev_dn11, locals.var_ivjdmrev_dn12, locals.var_ivjdmrev_dn13, locals.var_ivjdmrev_dn14,)
    }
};
        locals.var_ivjdmrev = assign15620_e22998;
        locals.var_ivjdmrev_dn0 = assign15620_e22998_d_n0;
        locals.var_ivjdmrev_dn2 = assign15620_e22998_d_n2;
        locals.var_ivjdmrev_dn3 = assign15620_e22998_d_n3;
        locals.var_ivjdmrev_dn4 = assign15620_e22998_d_n4;
        locals.var_ivjdmrev_dn5 = assign15620_e22998_d_n5;
        locals.var_ivjdmrev_dn6 = assign15620_e22998_d_n6;
        locals.var_ivjdmrev_dn7 = assign15620_e22998_d_n7;
        locals.var_ivjdmrev_dn8 = assign15620_e22998_d_n8;
        locals.var_ivjdmrev_dn9 = assign15620_e22998_d_n9;
        locals.var_ivjdmrev_dn10 = assign15620_e22998_d_n10;
        locals.var_ivjdmrev_dn11 = assign15620_e22998_d_n11;
        locals.var_ivjdmrev_dn12 = assign15620_e22998_d_n12;
        locals.var_ivjdmrev_dn13 = assign15620_e22998_d_n13;
        locals.var_ivjdmrev_dn14 = assign15620_e22998_d_n14;
        locals.var_ivjdmrev_rv = 0.0;

        let (assign15630_e23003, assign15630_e23003_d_n0, assign15630_e23003_d_n2, assign15630_e23003_d_n3, assign15630_e23003_d_n4, assign15630_e23003_d_n5, assign15630_e23003_d_n6, assign15630_e23003_d_n7, assign15630_e23003_d_n8, assign15630_e23003_d_n9, assign15630_e23003_d_n10, assign15630_e23003_d_n11, assign15630_e23003_d_n12, assign15630_e23003_d_n13, assign15630_e23003_d_n14,) = {
    if (locals.var_guard487 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dslprev, locals.var_dslprev_dn0, locals.var_dslprev_dn2, locals.var_dslprev_dn3, locals.var_dslprev_dn4, locals.var_dslprev_dn5, locals.var_dslprev_dn6, locals.var_dslprev_dn7, locals.var_dslprev_dn8, locals.var_dslprev_dn9, locals.var_dslprev_dn10, locals.var_dslprev_dn11, locals.var_dslprev_dn12, locals.var_dslprev_dn13, locals.var_dslprev_dn14,)
    }
};
        locals.var_dslprev = assign15630_e23003;
        locals.var_dslprev_dn0 = assign15630_e23003_d_n0;
        locals.var_dslprev_dn2 = assign15630_e23003_d_n2;
        locals.var_dslprev_dn3 = assign15630_e23003_d_n3;
        locals.var_dslprev_dn4 = assign15630_e23003_d_n4;
        locals.var_dslprev_dn5 = assign15630_e23003_d_n5;
        locals.var_dslprev_dn6 = assign15630_e23003_d_n6;
        locals.var_dslprev_dn7 = assign15630_e23003_d_n7;
        locals.var_dslprev_dn8 = assign15630_e23003_d_n8;
        locals.var_dslprev_dn9 = assign15630_e23003_d_n9;
        locals.var_dslprev_dn10 = assign15630_e23003_d_n10;
        locals.var_dslprev_dn11 = assign15630_e23003_d_n11;
        locals.var_dslprev_dn12 = assign15630_e23003_d_n12;
        locals.var_dslprev_dn13 = assign15630_e23003_d_n13;
        locals.var_dslprev_dn14 = assign15630_e23003_d_n14;
        locals.var_dslprev_rv = 0.0;

        let assign15640_e23022: f64 = if (((p.p17 > 0.0) && (p.p18 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p19 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard488 = assign15640_e23022;
        locals.var_guard488_rv = 0.0;

        let (assign15650_e23028, assign15650_e23028_d_n0, assign15650_e23028_d_n2, assign15650_e23028_d_n3, assign15650_e23028_d_n4, assign15650_e23028_d_n5, assign15650_e23028_d_n6, assign15650_e23028_d_n7, assign15650_e23028_d_n8, assign15650_e23028_d_n9, assign15650_e23028_d_n10, assign15650_e23028_d_n11, assign15650_e23028_d_n12, assign15650_e23028_d_n13, assign15650_e23028_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15650_e23026: f64 = (locals.var_lnew).powf(p.p921);
        (assign15650_e23026, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15650_e23028;
        locals.var_t0_dn0 = assign15650_e23028_d_n0;
        locals.var_t0_dn2 = assign15650_e23028_d_n2;
        locals.var_t0_dn3 = assign15650_e23028_d_n3;
        locals.var_t0_dn4 = assign15650_e23028_d_n4;
        locals.var_t0_dn5 = assign15650_e23028_d_n5;
        locals.var_t0_dn6 = assign15650_e23028_d_n6;
        locals.var_t0_dn7 = assign15650_e23028_d_n7;
        locals.var_t0_dn8 = assign15650_e23028_d_n8;
        locals.var_t0_dn9 = assign15650_e23028_d_n9;
        locals.var_t0_dn10 = assign15650_e23028_d_n10;
        locals.var_t0_dn11 = assign15650_e23028_d_n11;
        locals.var_t0_dn12 = assign15650_e23028_d_n12;
        locals.var_t0_dn13 = assign15650_e23028_d_n13;
        locals.var_t0_dn14 = assign15650_e23028_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15660_e23034,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15660_e23032: f64 = (locals.var_wnew + p.p914);
        (assign15660_e23032,)
    } else {
        (locals.var_w_tmp_stress,)
    }
};
        locals.var_w_tmp_stress = assign15660_e23034;
        locals.var_w_tmp_stress_rv = 0.0;

        let (assign15670_e23040, assign15670_e23040_d_n0, assign15670_e23040_d_n2, assign15670_e23040_d_n3, assign15670_e23040_d_n4, assign15670_e23040_d_n5, assign15670_e23040_d_n6, assign15670_e23040_d_n7, assign15670_e23040_d_n8, assign15670_e23040_d_n9, assign15670_e23040_d_n10, assign15670_e23040_d_n11, assign15670_e23040_d_n12, assign15670_e23040_d_n13, assign15670_e23040_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15670_e23038: f64 = (locals.var_w_tmp_stress).powf(p.p922);
        (assign15670_e23038, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15670_e23040;
        locals.var_t1_dn0 = assign15670_e23040_d_n0;
        locals.var_t1_dn2 = assign15670_e23040_d_n2;
        locals.var_t1_dn3 = assign15670_e23040_d_n3;
        locals.var_t1_dn4 = assign15670_e23040_d_n4;
        locals.var_t1_dn5 = assign15670_e23040_d_n5;
        locals.var_t1_dn6 = assign15670_e23040_d_n6;
        locals.var_t1_dn7 = assign15670_e23040_d_n7;
        locals.var_t1_dn8 = assign15670_e23040_d_n8;
        locals.var_t1_dn9 = assign15670_e23040_d_n9;
        locals.var_t1_dn10 = assign15670_e23040_d_n10;
        locals.var_t1_dn11 = assign15670_e23040_d_n11;
        locals.var_t1_dn12 = assign15670_e23040_d_n12;
        locals.var_t1_dn13 = assign15670_e23040_d_n13;
        locals.var_t1_dn14 = assign15670_e23040_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15680_e23056, assign15680_e23056_d_n0, assign15680_e23056_d_n2, assign15680_e23056_d_n3, assign15680_e23056_d_n4, assign15680_e23056_d_n5, assign15680_e23056_d_n6, assign15680_e23056_d_n7, assign15680_e23056_d_n8, assign15680_e23056_d_n9, assign15680_e23056_d_n10, assign15680_e23056_d_n11, assign15680_e23056_d_n12, assign15680_e23056_d_n13, assign15680_e23056_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15680_e23044: f64 = (p.p918 / locals.var_t0);
        let assign15680_e23047: f64 = (p.p919 / locals.var_t1);
        let assign15680_e23048: f64 = (assign15680_e23044 + assign15680_e23047);
        let assign15680_e23052: f64 = (locals.var_t0 * locals.var_t1);
        let assign15680_e23053: f64 = (p.p920 / assign15680_e23052);
        let assign15680_e23054: f64 = (assign15680_e23048 + assign15680_e23053);
        (assign15680_e23054, (((-((p.p918 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn13 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn13))) / (assign15680_e23052 * assign15680_e23052)))), (((-((p.p918 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) + (-((p.p919 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)))) + (-((p.p920 * ((locals.var_t0_dn14 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn14))) / (assign15680_e23052 * assign15680_e23052)))),)
    } else {
        (locals.var_tmp1_stress, locals.var_tmp1_stress_dn0, locals.var_tmp1_stress_dn2, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11, locals.var_tmp1_stress_dn12, locals.var_tmp1_stress_dn13, locals.var_tmp1_stress_dn14,)
    }
};
        locals.var_tmp1_stress = assign15680_e23056;
        locals.var_tmp1_stress_dn0 = assign15680_e23056_d_n0;
        locals.var_tmp1_stress_dn2 = assign15680_e23056_d_n2;
        locals.var_tmp1_stress_dn3 = assign15680_e23056_d_n3;
        locals.var_tmp1_stress_dn4 = assign15680_e23056_d_n4;
        locals.var_tmp1_stress_dn5 = assign15680_e23056_d_n5;
        locals.var_tmp1_stress_dn6 = assign15680_e23056_d_n6;
        locals.var_tmp1_stress_dn7 = assign15680_e23056_d_n7;
        locals.var_tmp1_stress_dn8 = assign15680_e23056_d_n8;
        locals.var_tmp1_stress_dn9 = assign15680_e23056_d_n9;
        locals.var_tmp1_stress_dn10 = assign15680_e23056_d_n10;
        locals.var_tmp1_stress_dn11 = assign15680_e23056_d_n11;
        locals.var_tmp1_stress_dn12 = assign15680_e23056_d_n12;
        locals.var_tmp1_stress_dn13 = assign15680_e23056_d_n13;
        locals.var_tmp1_stress_dn14 = assign15680_e23056_d_n14;
        locals.var_tmp1_stress_rv = 0.0;

        let (assign15690_e23062, assign15690_e23062_d_n0, assign15690_e23062_d_n2, assign15690_e23062_d_n3, assign15690_e23062_d_n4, assign15690_e23062_d_n5, assign15690_e23062_d_n6, assign15690_e23062_d_n7, assign15690_e23062_d_n8, assign15690_e23062_d_n9, assign15690_e23062_d_n10, assign15690_e23062_d_n11, assign15690_e23062_d_n12, assign15690_e23062_d_n13, assign15690_e23062_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15690_e23060: f64 = (1.0 + locals.var_tmp1_stress);
        (assign15690_e23060, locals.var_tmp1_stress_dn0, locals.var_tmp1_stress_dn2, locals.var_tmp1_stress_dn3, locals.var_tmp1_stress_dn4, locals.var_tmp1_stress_dn5, locals.var_tmp1_stress_dn6, locals.var_tmp1_stress_dn7, locals.var_tmp1_stress_dn8, locals.var_tmp1_stress_dn9, locals.var_tmp1_stress_dn10, locals.var_tmp1_stress_dn11, locals.var_tmp1_stress_dn12, locals.var_tmp1_stress_dn13, locals.var_tmp1_stress_dn14,)
    } else {
        (locals.var_kstress_u0, locals.var_kstress_u0_dn0, locals.var_kstress_u0_dn2, locals.var_kstress_u0_dn3, locals.var_kstress_u0_dn4, locals.var_kstress_u0_dn5, locals.var_kstress_u0_dn6, locals.var_kstress_u0_dn7, locals.var_kstress_u0_dn8, locals.var_kstress_u0_dn9, locals.var_kstress_u0_dn10, locals.var_kstress_u0_dn11, locals.var_kstress_u0_dn12, locals.var_kstress_u0_dn13, locals.var_kstress_u0_dn14,)
    }
};
        locals.var_kstress_u0 = assign15690_e23062;
        locals.var_kstress_u0_dn0 = assign15690_e23062_d_n0;
        locals.var_kstress_u0_dn2 = assign15690_e23062_d_n2;
        locals.var_kstress_u0_dn3 = assign15690_e23062_d_n3;
        locals.var_kstress_u0_dn4 = assign15690_e23062_d_n4;
        locals.var_kstress_u0_dn5 = assign15690_e23062_d_n5;
        locals.var_kstress_u0_dn6 = assign15690_e23062_d_n6;
        locals.var_kstress_u0_dn7 = assign15690_e23062_d_n7;
        locals.var_kstress_u0_dn8 = assign15690_e23062_d_n8;
        locals.var_kstress_u0_dn9 = assign15690_e23062_d_n9;
        locals.var_kstress_u0_dn10 = assign15690_e23062_d_n10;
        locals.var_kstress_u0_dn11 = assign15690_e23062_d_n11;
        locals.var_kstress_u0_dn12 = assign15690_e23062_d_n12;
        locals.var_kstress_u0_dn13 = assign15690_e23062_d_n13;
        locals.var_kstress_u0_dn14 = assign15690_e23062_d_n14;
        locals.var_kstress_u0_rv = 0.0;

        let (assign15700_e23068, assign15700_e23068_d_n0, assign15700_e23068_d_n2, assign15700_e23068_d_n3, assign15700_e23068_d_n4, assign15700_e23068_d_n5, assign15700_e23068_d_n6, assign15700_e23068_d_n7, assign15700_e23068_d_n8, assign15700_e23068_d_n9, assign15700_e23068_d_n10, assign15700_e23068_d_n11, assign15700_e23068_d_n12, assign15700_e23068_d_n13, assign15700_e23068_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15700_e23066: f64 = (locals.var_lnew).powf(p.p927);
        (assign15700_e23066, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15700_e23068;
        locals.var_t0_dn0 = assign15700_e23068_d_n0;
        locals.var_t0_dn2 = assign15700_e23068_d_n2;
        locals.var_t0_dn3 = assign15700_e23068_d_n3;
        locals.var_t0_dn4 = assign15700_e23068_d_n4;
        locals.var_t0_dn5 = assign15700_e23068_d_n5;
        locals.var_t0_dn6 = assign15700_e23068_d_n6;
        locals.var_t0_dn7 = assign15700_e23068_d_n7;
        locals.var_t0_dn8 = assign15700_e23068_d_n8;
        locals.var_t0_dn9 = assign15700_e23068_d_n9;
        locals.var_t0_dn10 = assign15700_e23068_d_n10;
        locals.var_t0_dn11 = assign15700_e23068_d_n11;
        locals.var_t0_dn12 = assign15700_e23068_d_n12;
        locals.var_t0_dn13 = assign15700_e23068_d_n13;
        locals.var_t0_dn14 = assign15700_e23068_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15710_e23074, assign15710_e23074_d_n0, assign15710_e23074_d_n2, assign15710_e23074_d_n3, assign15710_e23074_d_n4, assign15710_e23074_d_n5, assign15710_e23074_d_n6, assign15710_e23074_d_n7, assign15710_e23074_d_n8, assign15710_e23074_d_n9, assign15710_e23074_d_n10, assign15710_e23074_d_n11, assign15710_e23074_d_n12, assign15710_e23074_d_n13, assign15710_e23074_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15710_e23072: f64 = (locals.var_w_tmp_stress).powf(p.p928);
        (assign15710_e23072, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15710_e23074;
        locals.var_t1_dn0 = assign15710_e23074_d_n0;
        locals.var_t1_dn2 = assign15710_e23074_d_n2;
        locals.var_t1_dn3 = assign15710_e23074_d_n3;
        locals.var_t1_dn4 = assign15710_e23074_d_n4;
        locals.var_t1_dn5 = assign15710_e23074_d_n5;
        locals.var_t1_dn6 = assign15710_e23074_d_n6;
        locals.var_t1_dn7 = assign15710_e23074_d_n7;
        locals.var_t1_dn8 = assign15710_e23074_d_n8;
        locals.var_t1_dn9 = assign15710_e23074_d_n9;
        locals.var_t1_dn10 = assign15710_e23074_d_n10;
        locals.var_t1_dn11 = assign15710_e23074_d_n11;
        locals.var_t1_dn12 = assign15710_e23074_d_n12;
        locals.var_t1_dn13 = assign15710_e23074_d_n13;
        locals.var_t1_dn14 = assign15710_e23074_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign15720_e23090, assign15720_e23090_d_n0, assign15720_e23090_d_n2, assign15720_e23090_d_n3, assign15720_e23090_d_n4, assign15720_e23090_d_n5, assign15720_e23090_d_n6, assign15720_e23090_d_n7, assign15720_e23090_d_n8, assign15720_e23090_d_n9, assign15720_e23090_d_n10, assign15720_e23090_d_n11, assign15720_e23090_d_n12, assign15720_e23090_d_n13, assign15720_e23090_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15720_e23078: f64 = (p.p924 / locals.var_t0);
        let assign15720_e23081: f64 = (p.p925 / locals.var_t1);
        let assign15720_e23082: f64 = (assign15720_e23078 + assign15720_e23081);
        let assign15720_e23086: f64 = (locals.var_t0 * locals.var_t1);
        let assign15720_e23087: f64 = (p.p926 / assign15720_e23086);
        let assign15720_e23088: f64 = (assign15720_e23082 + assign15720_e23087);
        (assign15720_e23088, (((-((p.p924 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn0 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn0))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn2 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn2))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn13 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn13))) / (assign15720_e23086 * assign15720_e23086)))), (((-((p.p924 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) + (-((p.p925 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)))) + (-((p.p926 * ((locals.var_t0_dn14 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn14))) / (assign15720_e23086 * assign15720_e23086)))),)
    } else {
        (locals.var_tmp1_stress_vth, locals.var_tmp1_stress_vth_dn0, locals.var_tmp1_stress_vth_dn2, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11, locals.var_tmp1_stress_vth_dn12, locals.var_tmp1_stress_vth_dn13, locals.var_tmp1_stress_vth_dn14,)
    }
};
        locals.var_tmp1_stress_vth = assign15720_e23090;
        locals.var_tmp1_stress_vth_dn0 = assign15720_e23090_d_n0;
        locals.var_tmp1_stress_vth_dn2 = assign15720_e23090_d_n2;
        locals.var_tmp1_stress_vth_dn3 = assign15720_e23090_d_n3;
        locals.var_tmp1_stress_vth_dn4 = assign15720_e23090_d_n4;
        locals.var_tmp1_stress_vth_dn5 = assign15720_e23090_d_n5;
        locals.var_tmp1_stress_vth_dn6 = assign15720_e23090_d_n6;
        locals.var_tmp1_stress_vth_dn7 = assign15720_e23090_d_n7;
        locals.var_tmp1_stress_vth_dn8 = assign15720_e23090_d_n8;
        locals.var_tmp1_stress_vth_dn9 = assign15720_e23090_d_n9;
        locals.var_tmp1_stress_vth_dn10 = assign15720_e23090_d_n10;
        locals.var_tmp1_stress_vth_dn11 = assign15720_e23090_d_n11;
        locals.var_tmp1_stress_vth_dn12 = assign15720_e23090_d_n12;
        locals.var_tmp1_stress_vth_dn13 = assign15720_e23090_d_n13;
        locals.var_tmp1_stress_vth_dn14 = assign15720_e23090_d_n14;
        locals.var_tmp1_stress_vth_rv = 0.0;

        let (assign15730_e23096, assign15730_e23096_d_n0, assign15730_e23096_d_n2, assign15730_e23096_d_n3, assign15730_e23096_d_n4, assign15730_e23096_d_n5, assign15730_e23096_d_n6, assign15730_e23096_d_n7, assign15730_e23096_d_n8, assign15730_e23096_d_n9, assign15730_e23096_d_n10, assign15730_e23096_d_n11, assign15730_e23096_d_n12, assign15730_e23096_d_n13, assign15730_e23096_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15730_e23094: f64 = (1.0 + locals.var_tmp1_stress_vth);
        (assign15730_e23094, locals.var_tmp1_stress_vth_dn0, locals.var_tmp1_stress_vth_dn2, locals.var_tmp1_stress_vth_dn3, locals.var_tmp1_stress_vth_dn4, locals.var_tmp1_stress_vth_dn5, locals.var_tmp1_stress_vth_dn6, locals.var_tmp1_stress_vth_dn7, locals.var_tmp1_stress_vth_dn8, locals.var_tmp1_stress_vth_dn9, locals.var_tmp1_stress_vth_dn10, locals.var_tmp1_stress_vth_dn11, locals.var_tmp1_stress_vth_dn12, locals.var_tmp1_stress_vth_dn13, locals.var_tmp1_stress_vth_dn14,)
    } else {
        (locals.var_kstress_vth0, locals.var_kstress_vth0_dn0, locals.var_kstress_vth0_dn2, locals.var_kstress_vth0_dn3, locals.var_kstress_vth0_dn4, locals.var_kstress_vth0_dn5, locals.var_kstress_vth0_dn6, locals.var_kstress_vth0_dn7, locals.var_kstress_vth0_dn8, locals.var_kstress_vth0_dn9, locals.var_kstress_vth0_dn10, locals.var_kstress_vth0_dn11, locals.var_kstress_vth0_dn12, locals.var_kstress_vth0_dn13, locals.var_kstress_vth0_dn14,)
    }
};
        locals.var_kstress_vth0 = assign15730_e23096;
        locals.var_kstress_vth0_dn0 = assign15730_e23096_d_n0;
        locals.var_kstress_vth0_dn2 = assign15730_e23096_d_n2;
        locals.var_kstress_vth0_dn3 = assign15730_e23096_d_n3;
        locals.var_kstress_vth0_dn4 = assign15730_e23096_d_n4;
        locals.var_kstress_vth0_dn5 = assign15730_e23096_d_n5;
        locals.var_kstress_vth0_dn6 = assign15730_e23096_d_n6;
        locals.var_kstress_vth0_dn7 = assign15730_e23096_d_n7;
        locals.var_kstress_vth0_dn8 = assign15730_e23096_d_n8;
        locals.var_kstress_vth0_dn9 = assign15730_e23096_d_n9;
        locals.var_kstress_vth0_dn10 = assign15730_e23096_d_n10;
        locals.var_kstress_vth0_dn11 = assign15730_e23096_d_n11;
        locals.var_kstress_vth0_dn12 = assign15730_e23096_d_n12;
        locals.var_kstress_vth0_dn13 = assign15730_e23096_d_n13;
        locals.var_kstress_vth0_dn14 = assign15730_e23096_d_n14;
        locals.var_kstress_vth0_rv = 0.0;

        let (assign15740_e23102, assign15740_e23102_d_n0, assign15740_e23102_d_n2, assign15740_e23102_d_n3, assign15740_e23102_d_n4, assign15740_e23102_d_n5, assign15740_e23102_d_n6, assign15740_e23102_d_n7, assign15740_e23102_d_n8, assign15740_e23102_d_n9, assign15740_e23102_d_n10, assign15740_e23102_d_n11, assign15740_e23102_d_n12, assign15740_e23102_d_n13, assign15740_e23102_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15740_e23100: f64 = (locals.var_tratio - 1.0);
        (assign15740_e23100, 0.0, 0.0, 0.0, locals.var_tratio_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15740_e23102;
        locals.var_t0_dn0 = assign15740_e23102_d_n0;
        locals.var_t0_dn2 = assign15740_e23102_d_n2;
        locals.var_t0_dn3 = assign15740_e23102_d_n3;
        locals.var_t0_dn4 = assign15740_e23102_d_n4;
        locals.var_t0_dn5 = assign15740_e23102_d_n5;
        locals.var_t0_dn6 = assign15740_e23102_d_n6;
        locals.var_t0_dn7 = assign15740_e23102_d_n7;
        locals.var_t0_dn8 = assign15740_e23102_d_n8;
        locals.var_t0_dn9 = assign15740_e23102_d_n9;
        locals.var_t0_dn10 = assign15740_e23102_d_n10;
        locals.var_t0_dn11 = assign15740_e23102_d_n11;
        locals.var_t0_dn12 = assign15740_e23102_d_n12;
        locals.var_t0_dn13 = assign15740_e23102_d_n13;
        locals.var_t0_dn14 = assign15740_e23102_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign15750_e23114, assign15750_e23114_d_n0, assign15750_e23114_d_n2, assign15750_e23114_d_n3, assign15750_e23114_d_n4, assign15750_e23114_d_n5, assign15750_e23114_d_n6, assign15750_e23114_d_n7, assign15750_e23114_d_n8, assign15750_e23114_d_n9, assign15750_e23114_d_n10, assign15750_e23114_d_n11, assign15750_e23114_d_n12, assign15750_e23114_d_n13, assign15750_e23114_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15750_e23108: f64 = (p.p917 * locals.var_t0);
        let assign15750_e23109: f64 = (1.0 + assign15750_e23108);
        let assign15750_e23110: f64 = (locals.var_kstress_u0 * assign15750_e23109);
        let assign15750_e23112: f64 = (assign15750_e23110 + 1e-9);
        (assign15750_e23112, ((locals.var_kstress_u0_dn0 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn0))), ((locals.var_kstress_u0_dn2 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn2))), ((locals.var_kstress_u0_dn3 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn3))), ((locals.var_kstress_u0_dn4 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn4))), ((locals.var_kstress_u0_dn5 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn5))), ((locals.var_kstress_u0_dn6 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn6))), ((locals.var_kstress_u0_dn7 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn7))), ((locals.var_kstress_u0_dn8 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn8))), ((locals.var_kstress_u0_dn9 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn9))), ((locals.var_kstress_u0_dn10 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn10))), ((locals.var_kstress_u0_dn11 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn11))), ((locals.var_kstress_u0_dn12 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn12))), ((locals.var_kstress_u0_dn13 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn13))), ((locals.var_kstress_u0_dn14 * assign15750_e23109) + (locals.var_kstress_u0 * (p.p917 * locals.var_t0_dn14))),)
    } else {
        (locals.var_ku0_temp, locals.var_ku0_temp_dn0, locals.var_ku0_temp_dn2, locals.var_ku0_temp_dn3, locals.var_ku0_temp_dn4, locals.var_ku0_temp_dn5, locals.var_ku0_temp_dn6, locals.var_ku0_temp_dn7, locals.var_ku0_temp_dn8, locals.var_ku0_temp_dn9, locals.var_ku0_temp_dn10, locals.var_ku0_temp_dn11, locals.var_ku0_temp_dn12, locals.var_ku0_temp_dn13, locals.var_ku0_temp_dn14,)
    }
};
        locals.var_ku0_temp = assign15750_e23114;
        locals.var_ku0_temp_dn0 = assign15750_e23114_d_n0;
        locals.var_ku0_temp_dn2 = assign15750_e23114_d_n2;
        locals.var_ku0_temp_dn3 = assign15750_e23114_d_n3;
        locals.var_ku0_temp_dn4 = assign15750_e23114_d_n4;
        locals.var_ku0_temp_dn5 = assign15750_e23114_d_n5;
        locals.var_ku0_temp_dn6 = assign15750_e23114_d_n6;
        locals.var_ku0_temp_dn7 = assign15750_e23114_d_n7;
        locals.var_ku0_temp_dn8 = assign15750_e23114_d_n8;
        locals.var_ku0_temp_dn9 = assign15750_e23114_d_n9;
        locals.var_ku0_temp_dn10 = assign15750_e23114_d_n10;
        locals.var_ku0_temp_dn11 = assign15750_e23114_d_n11;
        locals.var_ku0_temp_dn12 = assign15750_e23114_d_n12;
        locals.var_ku0_temp_dn13 = assign15750_e23114_d_n13;
        locals.var_ku0_temp_dn14 = assign15750_e23114_d_n14;
        locals.var_ku0_temp_rv = 0.0;

        let (assign15760_e23118,) = {
    if (locals.var_guard488 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign15760_e23118;
        locals.var_i_rv = 0.0;

        let mut assign15770_loop_guard: usize = 0;
        while {
            let assign15770_cond_e23123: f64 = if ((locals.var_guard488 != 0.0) && (locals.var_i < p.p2)) { 1.0 } else { 0.0 };
            assign15770_cond_e23123 != 0.0
        } {
            assign15770_loop_guard += 1;
            assert!(assign15770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign15770_body0_e23141, assign15770_body0_e23141_d_n0, assign15770_body0_e23141_d_n2, assign15770_body0_e23141_d_n3, assign15770_body0_e23141_d_n4, assign15770_body0_e23141_d_n5, assign15770_body0_e23141_d_n6, assign15770_body0_e23141_d_n7, assign15770_body0_e23141_d_n8, assign15770_body0_e23141_d_n9, assign15770_body0_e23141_d_n10, assign15770_body0_e23141_d_n11, assign15770_body0_e23141_d_n12, assign15770_body0_e23141_d_n13, assign15770_body0_e23141_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body0_e23127: f64 = (1.0 / p.p2);
        let assign15770_body0_e23131: f64 = (0.5 * locals.var_l_mult);
        let assign15770_body0_e23132: f64 = (p.p17 + assign15770_body0_e23131);
        let assign15770_body0_e23136: f64 = (p.p19 + locals.var_l_mult);
        let assign15770_body0_e23137: f64 = (locals.var_i * assign15770_body0_e23136);
        let assign15770_body0_e23138: f64 = (assign15770_body0_e23132 + assign15770_body0_e23137);
        let assign15770_body0_e23139: f64 = (assign15770_body0_e23127 / assign15770_body0_e23138);
        (assign15770_body0_e23139, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
            locals.var_t0 = assign15770_body0_e23141;
            locals.var_t0_dn0 = assign15770_body0_e23141_d_n0;
            locals.var_t0_dn2 = assign15770_body0_e23141_d_n2;
            locals.var_t0_dn3 = assign15770_body0_e23141_d_n3;
            locals.var_t0_dn4 = assign15770_body0_e23141_d_n4;
            locals.var_t0_dn5 = assign15770_body0_e23141_d_n5;
            locals.var_t0_dn6 = assign15770_body0_e23141_d_n6;
            locals.var_t0_dn7 = assign15770_body0_e23141_d_n7;
            locals.var_t0_dn8 = assign15770_body0_e23141_d_n8;
            locals.var_t0_dn9 = assign15770_body0_e23141_d_n9;
            locals.var_t0_dn10 = assign15770_body0_e23141_d_n10;
            locals.var_t0_dn11 = assign15770_body0_e23141_d_n11;
            locals.var_t0_dn12 = assign15770_body0_e23141_d_n12;
            locals.var_t0_dn13 = assign15770_body0_e23141_d_n13;
            locals.var_t0_dn14 = assign15770_body0_e23141_d_n14;
            locals.var_t0_rv = 0.0;
            let (assign15770_body1_e23159, assign15770_body1_e23159_d_n0, assign15770_body1_e23159_d_n2, assign15770_body1_e23159_d_n3, assign15770_body1_e23159_d_n4, assign15770_body1_e23159_d_n5, assign15770_body1_e23159_d_n6, assign15770_body1_e23159_d_n7, assign15770_body1_e23159_d_n8, assign15770_body1_e23159_d_n9, assign15770_body1_e23159_d_n10, assign15770_body1_e23159_d_n11, assign15770_body1_e23159_d_n12, assign15770_body1_e23159_d_n13, assign15770_body1_e23159_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body1_e23145: f64 = (1.0 / p.p2);
        let assign15770_body1_e23149: f64 = (0.5 * locals.var_l_mult);
        let assign15770_body1_e23150: f64 = (p.p18 + assign15770_body1_e23149);
        let assign15770_body1_e23154: f64 = (p.p19 + locals.var_l_mult);
        let assign15770_body1_e23155: f64 = (locals.var_i * assign15770_body1_e23154);
        let assign15770_body1_e23156: f64 = (assign15770_body1_e23150 + assign15770_body1_e23155);
        let assign15770_body1_e23157: f64 = (assign15770_body1_e23145 / assign15770_body1_e23156);
        (assign15770_body1_e23157, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign15770_body1_e23159;
            locals.var_t1_dn0 = assign15770_body1_e23159_d_n0;
            locals.var_t1_dn2 = assign15770_body1_e23159_d_n2;
            locals.var_t1_dn3 = assign15770_body1_e23159_d_n3;
            locals.var_t1_dn4 = assign15770_body1_e23159_d_n4;
            locals.var_t1_dn5 = assign15770_body1_e23159_d_n5;
            locals.var_t1_dn6 = assign15770_body1_e23159_d_n6;
            locals.var_t1_dn7 = assign15770_body1_e23159_d_n7;
            locals.var_t1_dn8 = assign15770_body1_e23159_d_n8;
            locals.var_t1_dn9 = assign15770_body1_e23159_d_n9;
            locals.var_t1_dn10 = assign15770_body1_e23159_d_n10;
            locals.var_t1_dn11 = assign15770_body1_e23159_d_n11;
            locals.var_t1_dn12 = assign15770_body1_e23159_d_n12;
            locals.var_t1_dn13 = assign15770_body1_e23159_d_n13;
            locals.var_t1_dn14 = assign15770_body1_e23159_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign15770_body2_e23165, assign15770_body2_e23165_d_n0, assign15770_body2_e23165_d_n2, assign15770_body2_e23165_d_n3, assign15770_body2_e23165_d_n4, assign15770_body2_e23165_d_n5, assign15770_body2_e23165_d_n6, assign15770_body2_e23165_d_n7, assign15770_body2_e23165_d_n8, assign15770_body2_e23165_d_n9, assign15770_body2_e23165_d_n10, assign15770_body2_e23165_d_n11, assign15770_body2_e23165_d_n12, assign15770_body2_e23165_d_n13, assign15770_body2_e23165_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body2_e23163: f64 = (locals.var_inv_sa + locals.var_t0);
        (assign15770_body2_e23163, (locals.var_inv_sa_dn0 + locals.var_t0_dn0), (locals.var_inv_sa_dn2 + locals.var_t0_dn2), (locals.var_inv_sa_dn3 + locals.var_t0_dn3), (locals.var_inv_sa_dn4 + locals.var_t0_dn4), (locals.var_inv_sa_dn5 + locals.var_t0_dn5), (locals.var_inv_sa_dn6 + locals.var_t0_dn6), (locals.var_inv_sa_dn7 + locals.var_t0_dn7), (locals.var_inv_sa_dn8 + locals.var_t0_dn8), (locals.var_inv_sa_dn9 + locals.var_t0_dn9), (locals.var_inv_sa_dn10 + locals.var_t0_dn10), (locals.var_inv_sa_dn11 + locals.var_t0_dn11), (locals.var_inv_sa_dn12 + locals.var_t0_dn12), (locals.var_inv_sa_dn13 + locals.var_t0_dn13), (locals.var_inv_sa_dn14 + locals.var_t0_dn14),)
    } else {
        (locals.var_inv_sa, locals.var_inv_sa_dn0, locals.var_inv_sa_dn2, locals.var_inv_sa_dn3, locals.var_inv_sa_dn4, locals.var_inv_sa_dn5, locals.var_inv_sa_dn6, locals.var_inv_sa_dn7, locals.var_inv_sa_dn8, locals.var_inv_sa_dn9, locals.var_inv_sa_dn10, locals.var_inv_sa_dn11, locals.var_inv_sa_dn12, locals.var_inv_sa_dn13, locals.var_inv_sa_dn14,)
    }
};
            locals.var_inv_sa = assign15770_body2_e23165;
            locals.var_inv_sa_dn0 = assign15770_body2_e23165_d_n0;
            locals.var_inv_sa_dn2 = assign15770_body2_e23165_d_n2;
            locals.var_inv_sa_dn3 = assign15770_body2_e23165_d_n3;
            locals.var_inv_sa_dn4 = assign15770_body2_e23165_d_n4;
            locals.var_inv_sa_dn5 = assign15770_body2_e23165_d_n5;
            locals.var_inv_sa_dn6 = assign15770_body2_e23165_d_n6;
            locals.var_inv_sa_dn7 = assign15770_body2_e23165_d_n7;
            locals.var_inv_sa_dn8 = assign15770_body2_e23165_d_n8;
            locals.var_inv_sa_dn9 = assign15770_body2_e23165_d_n9;
            locals.var_inv_sa_dn10 = assign15770_body2_e23165_d_n10;
            locals.var_inv_sa_dn11 = assign15770_body2_e23165_d_n11;
            locals.var_inv_sa_dn12 = assign15770_body2_e23165_d_n12;
            locals.var_inv_sa_dn13 = assign15770_body2_e23165_d_n13;
            locals.var_inv_sa_dn14 = assign15770_body2_e23165_d_n14;
            locals.var_inv_sa_rv = 0.0;
            let (assign15770_body3_e23171, assign15770_body3_e23171_d_n0, assign15770_body3_e23171_d_n2, assign15770_body3_e23171_d_n3, assign15770_body3_e23171_d_n4, assign15770_body3_e23171_d_n5, assign15770_body3_e23171_d_n6, assign15770_body3_e23171_d_n7, assign15770_body3_e23171_d_n8, assign15770_body3_e23171_d_n9, assign15770_body3_e23171_d_n10, assign15770_body3_e23171_d_n11, assign15770_body3_e23171_d_n12, assign15770_body3_e23171_d_n13, assign15770_body3_e23171_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body3_e23169: f64 = (locals.var_inv_sb + locals.var_t1);
        (assign15770_body3_e23169, (locals.var_inv_sb_dn0 + locals.var_t1_dn0), (locals.var_inv_sb_dn2 + locals.var_t1_dn2), (locals.var_inv_sb_dn3 + locals.var_t1_dn3), (locals.var_inv_sb_dn4 + locals.var_t1_dn4), (locals.var_inv_sb_dn5 + locals.var_t1_dn5), (locals.var_inv_sb_dn6 + locals.var_t1_dn6), (locals.var_inv_sb_dn7 + locals.var_t1_dn7), (locals.var_inv_sb_dn8 + locals.var_t1_dn8), (locals.var_inv_sb_dn9 + locals.var_t1_dn9), (locals.var_inv_sb_dn10 + locals.var_t1_dn10), (locals.var_inv_sb_dn11 + locals.var_t1_dn11), (locals.var_inv_sb_dn12 + locals.var_t1_dn12), (locals.var_inv_sb_dn13 + locals.var_t1_dn13), (locals.var_inv_sb_dn14 + locals.var_t1_dn14),)
    } else {
        (locals.var_inv_sb, locals.var_inv_sb_dn0, locals.var_inv_sb_dn2, locals.var_inv_sb_dn3, locals.var_inv_sb_dn4, locals.var_inv_sb_dn5, locals.var_inv_sb_dn6, locals.var_inv_sb_dn7, locals.var_inv_sb_dn8, locals.var_inv_sb_dn9, locals.var_inv_sb_dn10, locals.var_inv_sb_dn11, locals.var_inv_sb_dn12, locals.var_inv_sb_dn13, locals.var_inv_sb_dn14,)
    }
};
            locals.var_inv_sb = assign15770_body3_e23171;
            locals.var_inv_sb_dn0 = assign15770_body3_e23171_d_n0;
            locals.var_inv_sb_dn2 = assign15770_body3_e23171_d_n2;
            locals.var_inv_sb_dn3 = assign15770_body3_e23171_d_n3;
            locals.var_inv_sb_dn4 = assign15770_body3_e23171_d_n4;
            locals.var_inv_sb_dn5 = assign15770_body3_e23171_d_n5;
            locals.var_inv_sb_dn6 = assign15770_body3_e23171_d_n6;
            locals.var_inv_sb_dn7 = assign15770_body3_e23171_d_n7;
            locals.var_inv_sb_dn8 = assign15770_body3_e23171_d_n8;
            locals.var_inv_sb_dn9 = assign15770_body3_e23171_d_n9;
            locals.var_inv_sb_dn10 = assign15770_body3_e23171_d_n10;
            locals.var_inv_sb_dn11 = assign15770_body3_e23171_d_n11;
            locals.var_inv_sb_dn12 = assign15770_body3_e23171_d_n12;
            locals.var_inv_sb_dn13 = assign15770_body3_e23171_d_n13;
            locals.var_inv_sb_dn14 = assign15770_body3_e23171_d_n14;
            locals.var_inv_sb_rv = 0.0;
            let (assign15770_body4_e23177,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15770_body4_e23175: f64 = (locals.var_i + 1.0);
        (assign15770_body4_e23175,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign15770_body4_e23177;
            locals.var_i_rv = 0.0;
        }

        let (assign15780_e23187,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15780_e23183: f64 = (0.5 * locals.var_l_mult);
        let assign15780_e23184: f64 = (p.p912 + assign15780_e23183);
        let assign15780_e23185: f64 = (1.0 / assign15780_e23184);
        (assign15780_e23185,)
    } else {
        (locals.var_inv_saref,)
    }
};
        locals.var_inv_saref = assign15780_e23187;
        locals.var_inv_saref_rv = 0.0;

        let (assign15790_e23197,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15790_e23193: f64 = (0.5 * locals.var_l_mult);
        let assign15790_e23194: f64 = (p.p913 + assign15790_e23193);
        let assign15790_e23195: f64 = (1.0 / assign15790_e23194);
        (assign15790_e23195,)
    } else {
        (locals.var_inv_sbref,)
    }
};
        locals.var_inv_sbref = assign15790_e23197;
        locals.var_inv_sbref_rv = 0.0;

        let (assign15800_e23203,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15800_e23201: f64 = (locals.var_inv_saref + locals.var_inv_sbref);
        (assign15800_e23201,)
    } else {
        (locals.var_inv_odref,)
    }
};
        locals.var_inv_odref = assign15800_e23203;
        locals.var_inv_odref_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15810_e23211, assign15810_e23211_d_n0, assign15810_e23211_d_n2, assign15810_e23211_d_n3, assign15810_e23211_d_n4, assign15810_e23211_d_n5, assign15810_e23211_d_n6, assign15810_e23211_d_n7, assign15810_e23211_d_n8, assign15810_e23211_d_n9, assign15810_e23211_d_n10, assign15810_e23211_d_n11, assign15810_e23211_d_n12, assign15810_e23211_d_n13, assign15810_e23211_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15810_e23207: f64 = (p.p915 / locals.var_ku0_temp);
        let assign15810_e23209: f64 = (assign15810_e23207 * locals.var_inv_odref);
        (assign15810_e23209, ((-((p.p915 * locals.var_ku0_temp_dn0) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn2) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn3) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn4) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn5) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn6) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn7) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn8) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn9) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn10) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn11) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn12) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn13) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref), ((-((p.p915 * locals.var_ku0_temp_dn14) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_odref),)
    } else {
        (locals.var_rho_ref, locals.var_rho_ref_dn0, locals.var_rho_ref_dn2, locals.var_rho_ref_dn3, locals.var_rho_ref_dn4, locals.var_rho_ref_dn5, locals.var_rho_ref_dn6, locals.var_rho_ref_dn7, locals.var_rho_ref_dn8, locals.var_rho_ref_dn9, locals.var_rho_ref_dn10, locals.var_rho_ref_dn11, locals.var_rho_ref_dn12, locals.var_rho_ref_dn13, locals.var_rho_ref_dn14,)
    }
};
        locals.var_rho_ref = assign15810_e23211;
        locals.var_rho_ref_dn0 = assign15810_e23211_d_n0;
        locals.var_rho_ref_dn2 = assign15810_e23211_d_n2;
        locals.var_rho_ref_dn3 = assign15810_e23211_d_n3;
        locals.var_rho_ref_dn4 = assign15810_e23211_d_n4;
        locals.var_rho_ref_dn5 = assign15810_e23211_d_n5;
        locals.var_rho_ref_dn6 = assign15810_e23211_d_n6;
        locals.var_rho_ref_dn7 = assign15810_e23211_d_n7;
        locals.var_rho_ref_dn8 = assign15810_e23211_d_n8;
        locals.var_rho_ref_dn9 = assign15810_e23211_d_n9;
        locals.var_rho_ref_dn10 = assign15810_e23211_d_n10;
        locals.var_rho_ref_dn11 = assign15810_e23211_d_n11;
        locals.var_rho_ref_dn12 = assign15810_e23211_d_n12;
        locals.var_rho_ref_dn13 = assign15810_e23211_d_n13;
        locals.var_rho_ref_dn14 = assign15810_e23211_d_n14;
        locals.var_rho_ref_rv = 0.0;

        let (assign15820_e23217, assign15820_e23217_d_n0, assign15820_e23217_d_n2, assign15820_e23217_d_n3, assign15820_e23217_d_n4, assign15820_e23217_d_n5, assign15820_e23217_d_n6, assign15820_e23217_d_n7, assign15820_e23217_d_n8, assign15820_e23217_d_n9, assign15820_e23217_d_n10, assign15820_e23217_d_n11, assign15820_e23217_d_n12, assign15820_e23217_d_n13, assign15820_e23217_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15820_e23215: f64 = (locals.var_inv_sa + locals.var_inv_sb);
        (assign15820_e23215, (locals.var_inv_sa_dn0 + locals.var_inv_sb_dn0), (locals.var_inv_sa_dn2 + locals.var_inv_sb_dn2), (locals.var_inv_sa_dn3 + locals.var_inv_sb_dn3), (locals.var_inv_sa_dn4 + locals.var_inv_sb_dn4), (locals.var_inv_sa_dn5 + locals.var_inv_sb_dn5), (locals.var_inv_sa_dn6 + locals.var_inv_sb_dn6), (locals.var_inv_sa_dn7 + locals.var_inv_sb_dn7), (locals.var_inv_sa_dn8 + locals.var_inv_sb_dn8), (locals.var_inv_sa_dn9 + locals.var_inv_sb_dn9), (locals.var_inv_sa_dn10 + locals.var_inv_sb_dn10), (locals.var_inv_sa_dn11 + locals.var_inv_sb_dn11), (locals.var_inv_sa_dn12 + locals.var_inv_sb_dn12), (locals.var_inv_sa_dn13 + locals.var_inv_sb_dn13), (locals.var_inv_sa_dn14 + locals.var_inv_sb_dn14),)
    } else {
        (locals.var_inv_od, locals.var_inv_od_dn0, locals.var_inv_od_dn2, locals.var_inv_od_dn3, locals.var_inv_od_dn4, locals.var_inv_od_dn5, locals.var_inv_od_dn6, locals.var_inv_od_dn7, locals.var_inv_od_dn8, locals.var_inv_od_dn9, locals.var_inv_od_dn10, locals.var_inv_od_dn11, locals.var_inv_od_dn12, locals.var_inv_od_dn13, locals.var_inv_od_dn14,)
    }
};
        locals.var_inv_od = assign15820_e23217;
        locals.var_inv_od_dn0 = assign15820_e23217_d_n0;
        locals.var_inv_od_dn2 = assign15820_e23217_d_n2;
        locals.var_inv_od_dn3 = assign15820_e23217_d_n3;
        locals.var_inv_od_dn4 = assign15820_e23217_d_n4;
        locals.var_inv_od_dn5 = assign15820_e23217_d_n5;
        locals.var_inv_od_dn6 = assign15820_e23217_d_n6;
        locals.var_inv_od_dn7 = assign15820_e23217_d_n7;
        locals.var_inv_od_dn8 = assign15820_e23217_d_n8;
        locals.var_inv_od_dn9 = assign15820_e23217_d_n9;
        locals.var_inv_od_dn10 = assign15820_e23217_d_n10;
        locals.var_inv_od_dn11 = assign15820_e23217_d_n11;
        locals.var_inv_od_dn12 = assign15820_e23217_d_n12;
        locals.var_inv_od_dn13 = assign15820_e23217_d_n13;
        locals.var_inv_od_dn14 = assign15820_e23217_d_n14;
        locals.var_inv_od_rv = 0.0;

        let (assign15830_e23225, assign15830_e23225_d_n0, assign15830_e23225_d_n2, assign15830_e23225_d_n3, assign15830_e23225_d_n4, assign15830_e23225_d_n5, assign15830_e23225_d_n6, assign15830_e23225_d_n7, assign15830_e23225_d_n8, assign15830_e23225_d_n9, assign15830_e23225_d_n10, assign15830_e23225_d_n11, assign15830_e23225_d_n12, assign15830_e23225_d_n13, assign15830_e23225_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15830_e23221: f64 = (p.p915 / locals.var_ku0_temp);
        let assign15830_e23223: f64 = (assign15830_e23221 * locals.var_inv_od);
        (assign15830_e23223, (((-((p.p915 * locals.var_ku0_temp_dn0) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn0)), (((-((p.p915 * locals.var_ku0_temp_dn2) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn2)), (((-((p.p915 * locals.var_ku0_temp_dn3) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn3)), (((-((p.p915 * locals.var_ku0_temp_dn4) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn4)), (((-((p.p915 * locals.var_ku0_temp_dn5) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn5)), (((-((p.p915 * locals.var_ku0_temp_dn6) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn6)), (((-((p.p915 * locals.var_ku0_temp_dn7) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn7)), (((-((p.p915 * locals.var_ku0_temp_dn8) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn8)), (((-((p.p915 * locals.var_ku0_temp_dn9) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn9)), (((-((p.p915 * locals.var_ku0_temp_dn10) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn10)), (((-((p.p915 * locals.var_ku0_temp_dn11) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn11)), (((-((p.p915 * locals.var_ku0_temp_dn12) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn12)), (((-((p.p915 * locals.var_ku0_temp_dn13) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn13)), (((-((p.p915 * locals.var_ku0_temp_dn14) / (locals.var_ku0_temp * locals.var_ku0_temp))) * locals.var_inv_od) + (assign15830_e23221 * locals.var_inv_od_dn14)),)
    } else {
        (locals.var_rho, locals.var_rho_dn0, locals.var_rho_dn2, locals.var_rho_dn3, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12, locals.var_rho_dn13, locals.var_rho_dn14,)
    }
};
        locals.var_rho = assign15830_e23225;
        locals.var_rho_dn0 = assign15830_e23225_d_n0;
        locals.var_rho_dn2 = assign15830_e23225_d_n2;
        locals.var_rho_dn3 = assign15830_e23225_d_n3;
        locals.var_rho_dn4 = assign15830_e23225_d_n4;
        locals.var_rho_dn5 = assign15830_e23225_d_n5;
        locals.var_rho_dn6 = assign15830_e23225_d_n6;
        locals.var_rho_dn7 = assign15830_e23225_d_n7;
        locals.var_rho_dn8 = assign15830_e23225_d_n8;
        locals.var_rho_dn9 = assign15830_e23225_d_n9;
        locals.var_rho_dn10 = assign15830_e23225_d_n10;
        locals.var_rho_dn11 = assign15830_e23225_d_n11;
        locals.var_rho_dn12 = assign15830_e23225_d_n12;
        locals.var_rho_dn13 = assign15830_e23225_d_n13;
        locals.var_rho_dn14 = assign15830_e23225_d_n14;
        locals.var_rho_rv = 0.0;

        let (assign15840_e23235, assign15840_e23235_d_n0, assign15840_e23235_d_n2, assign15840_e23235_d_n3, assign15840_e23235_d_n4, assign15840_e23235_d_n5, assign15840_e23235_d_n6, assign15840_e23235_d_n7, assign15840_e23235_d_n8, assign15840_e23235_d_n9, assign15840_e23235_d_n10, assign15840_e23235_d_n11, assign15840_e23235_d_n12, assign15840_e23235_d_n13, assign15840_e23235_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15840_e23229: f64 = (1.0 + locals.var_rho);
        let assign15840_e23232: f64 = (1.0 + locals.var_rho_ref);
        let assign15840_e23233: f64 = (assign15840_e23229 / assign15840_e23232);
        (assign15840_e23233, (((locals.var_rho_dn0 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn0)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn2 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn2)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn3 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn3)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn4 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn4)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn5 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn5)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn6 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn6)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn7 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn7)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn8 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn8)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn9 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn9)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn10 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn10)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn11 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn11)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn12 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn12)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn13 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn13)) / (assign15840_e23232 * assign15840_e23232)), (((locals.var_rho_dn14 * assign15840_e23232) - (assign15840_e23229 * locals.var_rho_ref_dn14)) / (assign15840_e23232 * assign15840_e23232)),)
    } else {
        (locals.var_mu0_mult, locals.var_mu0_mult_dn0, locals.var_mu0_mult_dn2, locals.var_mu0_mult_dn3, locals.var_mu0_mult_dn4, locals.var_mu0_mult_dn5, locals.var_mu0_mult_dn6, locals.var_mu0_mult_dn7, locals.var_mu0_mult_dn8, locals.var_mu0_mult_dn9, locals.var_mu0_mult_dn10, locals.var_mu0_mult_dn11, locals.var_mu0_mult_dn12, locals.var_mu0_mult_dn13, locals.var_mu0_mult_dn14,)
    }
};
        locals.var_mu0_mult = assign15840_e23235;
        locals.var_mu0_mult_dn0 = assign15840_e23235_d_n0;
        locals.var_mu0_mult_dn2 = assign15840_e23235_d_n2;
        locals.var_mu0_mult_dn3 = assign15840_e23235_d_n3;
        locals.var_mu0_mult_dn4 = assign15840_e23235_d_n4;
        locals.var_mu0_mult_dn5 = assign15840_e23235_d_n5;
        locals.var_mu0_mult_dn6 = assign15840_e23235_d_n6;
        locals.var_mu0_mult_dn7 = assign15840_e23235_d_n7;
        locals.var_mu0_mult_dn8 = assign15840_e23235_d_n8;
        locals.var_mu0_mult_dn9 = assign15840_e23235_d_n9;
        locals.var_mu0_mult_dn10 = assign15840_e23235_d_n10;
        locals.var_mu0_mult_dn11 = assign15840_e23235_d_n11;
        locals.var_mu0_mult_dn12 = assign15840_e23235_d_n12;
        locals.var_mu0_mult_dn13 = assign15840_e23235_d_n13;
        locals.var_mu0_mult_dn14 = assign15840_e23235_d_n14;
        locals.var_mu0_mult_rv = 0.0;

        let (assign15850_e23249, assign15850_e23249_d_n0, assign15850_e23249_d_n2, assign15850_e23249_d_n3, assign15850_e23249_d_n4, assign15850_e23249_d_n5, assign15850_e23249_d_n6, assign15850_e23249_d_n7, assign15850_e23249_d_n8, assign15850_e23249_d_n9, assign15850_e23249_d_n10, assign15850_e23249_d_n11, assign15850_e23249_d_n12, assign15850_e23249_d_n13, assign15850_e23249_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15850_e23240: f64 = (locals.var_rho * p.p916);
        let assign15850_e23241: f64 = (1.0 + assign15850_e23240);
        let assign15850_e23245: f64 = (locals.var_rho_ref * p.p916);
        let assign15850_e23246: f64 = (1.0 + assign15850_e23245);
        let assign15850_e23247: f64 = (assign15850_e23241 / assign15850_e23246);
        (assign15850_e23247, ((((locals.var_rho_dn0 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn0 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn2 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn2 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn3 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn3 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn4 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn4 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn5 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn5 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn6 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn6 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn7 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn7 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn8 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn8 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn9 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn9 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn10 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn10 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn11 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn11 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn12 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn12 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn13 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn13 * p.p916))) / (assign15850_e23246 * assign15850_e23246)), ((((locals.var_rho_dn14 * p.p916) * assign15850_e23246) - (assign15850_e23241 * (locals.var_rho_ref_dn14 * p.p916))) / (assign15850_e23246 * assign15850_e23246)),)
    } else {
        (locals.var_vsat_mult, locals.var_vsat_mult_dn0, locals.var_vsat_mult_dn2, locals.var_vsat_mult_dn3, locals.var_vsat_mult_dn4, locals.var_vsat_mult_dn5, locals.var_vsat_mult_dn6, locals.var_vsat_mult_dn7, locals.var_vsat_mult_dn8, locals.var_vsat_mult_dn9, locals.var_vsat_mult_dn10, locals.var_vsat_mult_dn11, locals.var_vsat_mult_dn12, locals.var_vsat_mult_dn13, locals.var_vsat_mult_dn14,)
    }
};
        locals.var_vsat_mult = assign15850_e23249;
        locals.var_vsat_mult_dn0 = assign15850_e23249_d_n0;
        locals.var_vsat_mult_dn2 = assign15850_e23249_d_n2;
        locals.var_vsat_mult_dn3 = assign15850_e23249_d_n3;
        locals.var_vsat_mult_dn4 = assign15850_e23249_d_n4;
        locals.var_vsat_mult_dn5 = assign15850_e23249_d_n5;
        locals.var_vsat_mult_dn6 = assign15850_e23249_d_n6;
        locals.var_vsat_mult_dn7 = assign15850_e23249_d_n7;
        locals.var_vsat_mult_dn8 = assign15850_e23249_d_n8;
        locals.var_vsat_mult_dn9 = assign15850_e23249_d_n9;
        locals.var_vsat_mult_dn10 = assign15850_e23249_d_n10;
        locals.var_vsat_mult_dn11 = assign15850_e23249_d_n11;
        locals.var_vsat_mult_dn12 = assign15850_e23249_d_n12;
        locals.var_vsat_mult_dn13 = assign15850_e23249_d_n13;
        locals.var_vsat_mult_dn14 = assign15850_e23249_d_n14;
        locals.var_vsat_mult_rv = 0.0;

        let (assign15860_e23259, assign15860_e23259_d_n0, assign15860_e23259_d_n2, assign15860_e23259_d_n3, assign15860_e23259_d_n4, assign15860_e23259_d_n5, assign15860_e23259_d_n6, assign15860_e23259_d_n7, assign15860_e23259_d_n8, assign15860_e23259_d_n9, assign15860_e23259_d_n10, assign15860_e23259_d_n11, assign15860_e23259_d_n12, assign15860_e23259_d_n13, assign15860_e23259_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15860_e23253: f64 = (p.p923 / locals.var_kstress_vth0);
        let assign15860_e23256: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15860_e23257: f64 = (assign15860_e23253 * assign15860_e23256);
        (assign15860_e23257, (((-((p.p923 * locals.var_kstress_vth0_dn0) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn0)), (((-((p.p923 * locals.var_kstress_vth0_dn2) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn2)), (((-((p.p923 * locals.var_kstress_vth0_dn3) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn3)), (((-((p.p923 * locals.var_kstress_vth0_dn4) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn4)), (((-((p.p923 * locals.var_kstress_vth0_dn5) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn5)), (((-((p.p923 * locals.var_kstress_vth0_dn6) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn6)), (((-((p.p923 * locals.var_kstress_vth0_dn7) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn7)), (((-((p.p923 * locals.var_kstress_vth0_dn8) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn8)), (((-((p.p923 * locals.var_kstress_vth0_dn9) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn9)), (((-((p.p923 * locals.var_kstress_vth0_dn10) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn10)), (((-((p.p923 * locals.var_kstress_vth0_dn11) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn11)), (((-((p.p923 * locals.var_kstress_vth0_dn12) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn12)), (((-((p.p923 * locals.var_kstress_vth0_dn13) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn13)), (((-((p.p923 * locals.var_kstress_vth0_dn14) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15860_e23256) + (assign15860_e23253 * locals.var_inv_od_dn14)),)
    } else {
        (locals.var_vth0_stress, locals.var_vth0_stress_dn0, locals.var_vth0_stress_dn2, locals.var_vth0_stress_dn3, locals.var_vth0_stress_dn4, locals.var_vth0_stress_dn5, locals.var_vth0_stress_dn6, locals.var_vth0_stress_dn7, locals.var_vth0_stress_dn8, locals.var_vth0_stress_dn9, locals.var_vth0_stress_dn10, locals.var_vth0_stress_dn11, locals.var_vth0_stress_dn12, locals.var_vth0_stress_dn13, locals.var_vth0_stress_dn14,)
    }
};
        locals.var_vth0_stress = assign15860_e23259;
        locals.var_vth0_stress_dn0 = assign15860_e23259_d_n0;
        locals.var_vth0_stress_dn2 = assign15860_e23259_d_n2;
        locals.var_vth0_stress_dn3 = assign15860_e23259_d_n3;
        locals.var_vth0_stress_dn4 = assign15860_e23259_d_n4;
        locals.var_vth0_stress_dn5 = assign15860_e23259_d_n5;
        locals.var_vth0_stress_dn6 = assign15860_e23259_d_n6;
        locals.var_vth0_stress_dn7 = assign15860_e23259_d_n7;
        locals.var_vth0_stress_dn8 = assign15860_e23259_d_n8;
        locals.var_vth0_stress_dn9 = assign15860_e23259_d_n9;
        locals.var_vth0_stress_dn10 = assign15860_e23259_d_n10;
        locals.var_vth0_stress_dn11 = assign15860_e23259_d_n11;
        locals.var_vth0_stress_dn12 = assign15860_e23259_d_n12;
        locals.var_vth0_stress_dn13 = assign15860_e23259_d_n13;
        locals.var_vth0_stress_dn14 = assign15860_e23259_d_n14;
        locals.var_vth0_stress_rv = 0.0;

        let (assign15870_e23271, assign15870_e23271_d_n0, assign15870_e23271_d_n2, assign15870_e23271_d_n3, assign15870_e23271_d_n4, assign15870_e23271_d_n5, assign15870_e23271_d_n6, assign15870_e23271_d_n7, assign15870_e23271_d_n8, assign15870_e23271_d_n9, assign15870_e23271_d_n10, assign15870_e23271_d_n11, assign15870_e23271_d_n12, assign15870_e23271_d_n13, assign15870_e23271_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15870_e23264: f64 = (locals.var_kstress_vth0).powf(p.p930);
        let assign15870_e23265: f64 = (p.p929 / assign15870_e23264);
        let assign15870_e23268: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15870_e23269: f64 = (assign15870_e23265 * assign15870_e23268);
        (assign15870_e23269, (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn0)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn0 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn0)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn2)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn2 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn2)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn3)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn4)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn5)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn6)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn7)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn8)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn9)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn10)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn11)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn12)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn12 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn12)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn13)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn13 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn13)), (((-((p.p929 * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn14)) } } else { (assign15870_e23264 * (p.p930 * (locals.var_kstress_vth0_dn14 / locals.var_kstress_vth0))) }) / (assign15870_e23264 * assign15870_e23264))) * assign15870_e23268) + (assign15870_e23265 * locals.var_inv_od_dn14)),)
    } else {
        (locals.var_k2_stress, locals.var_k2_stress_dn0, locals.var_k2_stress_dn2, locals.var_k2_stress_dn3, locals.var_k2_stress_dn4, locals.var_k2_stress_dn5, locals.var_k2_stress_dn6, locals.var_k2_stress_dn7, locals.var_k2_stress_dn8, locals.var_k2_stress_dn9, locals.var_k2_stress_dn10, locals.var_k2_stress_dn11, locals.var_k2_stress_dn12, locals.var_k2_stress_dn13, locals.var_k2_stress_dn14,)
    }
};
        locals.var_k2_stress = assign15870_e23271;
        locals.var_k2_stress_dn0 = assign15870_e23271_d_n0;
        locals.var_k2_stress_dn2 = assign15870_e23271_d_n2;
        locals.var_k2_stress_dn3 = assign15870_e23271_d_n3;
        locals.var_k2_stress_dn4 = assign15870_e23271_d_n4;
        locals.var_k2_stress_dn5 = assign15870_e23271_d_n5;
        locals.var_k2_stress_dn6 = assign15870_e23271_d_n6;
        locals.var_k2_stress_dn7 = assign15870_e23271_d_n7;
        locals.var_k2_stress_dn8 = assign15870_e23271_d_n8;
        locals.var_k2_stress_dn9 = assign15870_e23271_d_n9;
        locals.var_k2_stress_dn10 = assign15870_e23271_d_n10;
        locals.var_k2_stress_dn11 = assign15870_e23271_d_n11;
        locals.var_k2_stress_dn12 = assign15870_e23271_d_n12;
        locals.var_k2_stress_dn13 = assign15870_e23271_d_n13;
        locals.var_k2_stress_dn14 = assign15870_e23271_d_n14;
        locals.var_k2_stress_rv = 0.0;

        let (assign15880_e23283, assign15880_e23283_d_n0, assign15880_e23283_d_n2, assign15880_e23283_d_n3, assign15880_e23283_d_n4, assign15880_e23283_d_n5, assign15880_e23283_d_n6, assign15880_e23283_d_n7, assign15880_e23283_d_n8, assign15880_e23283_d_n9, assign15880_e23283_d_n10, assign15880_e23283_d_n11, assign15880_e23283_d_n12, assign15880_e23283_d_n13, assign15880_e23283_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15880_e23276: f64 = (locals.var_kstress_vth0).powf(p.p932);
        let assign15880_e23277: f64 = (p.p931 / assign15880_e23276);
        let assign15880_e23280: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15880_e23281: f64 = (assign15880_e23277 * assign15880_e23280);
        (assign15880_e23281, (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn0)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn0 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn0)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn2)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn2 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn2)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn3)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn4)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn5)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn6)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn7)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn8)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn9)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn10)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn11)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn12)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn12 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn12)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn13)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn13 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn13)), (((-((p.p931 * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn14)) } } else { (assign15880_e23276 * (p.p932 * (locals.var_kstress_vth0_dn14 / locals.var_kstress_vth0))) }) / (assign15880_e23276 * assign15880_e23276))) * assign15880_e23280) + (assign15880_e23277 * locals.var_inv_od_dn14)),)
    } else {
        (locals.var_eta_stress, locals.var_eta_stress_dn0, locals.var_eta_stress_dn2, locals.var_eta_stress_dn3, locals.var_eta_stress_dn4, locals.var_eta_stress_dn5, locals.var_eta_stress_dn6, locals.var_eta_stress_dn7, locals.var_eta_stress_dn8, locals.var_eta_stress_dn9, locals.var_eta_stress_dn10, locals.var_eta_stress_dn11, locals.var_eta_stress_dn12, locals.var_eta_stress_dn13, locals.var_eta_stress_dn14,)
    }
};
        locals.var_eta_stress = assign15880_e23283;
        locals.var_eta_stress_dn0 = assign15880_e23283_d_n0;
        locals.var_eta_stress_dn2 = assign15880_e23283_d_n2;
        locals.var_eta_stress_dn3 = assign15880_e23283_d_n3;
        locals.var_eta_stress_dn4 = assign15880_e23283_d_n4;
        locals.var_eta_stress_dn5 = assign15880_e23283_d_n5;
        locals.var_eta_stress_dn6 = assign15880_e23283_d_n6;
        locals.var_eta_stress_dn7 = assign15880_e23283_d_n7;
        locals.var_eta_stress_dn8 = assign15880_e23283_d_n8;
        locals.var_eta_stress_dn9 = assign15880_e23283_d_n9;
        locals.var_eta_stress_dn10 = assign15880_e23283_d_n10;
        locals.var_eta_stress_dn11 = assign15880_e23283_d_n11;
        locals.var_eta_stress_dn12 = assign15880_e23283_d_n12;
        locals.var_eta_stress_dn13 = assign15880_e23283_d_n13;
        locals.var_eta_stress_dn14 = assign15880_e23283_d_n14;
        locals.var_eta_stress_rv = 0.0;

        let (assign15890_e23289, assign15890_e23289_d_n0, assign15890_e23289_d_n2, assign15890_e23289_d_n3, assign15890_e23289_d_n4, assign15890_e23289_d_n5, assign15890_e23289_d_n6, assign15890_e23289_d_n7, assign15890_e23289_d_n8, assign15890_e23289_d_n9, assign15890_e23289_d_n10, assign15890_e23289_d_n11, assign15890_e23289_d_n12, assign15890_e23289_d_n13, assign15890_e23289_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15890_e23287: f64 = (locals.var_u0_t * locals.var_mu0_mult);
        (assign15890_e23287, ((locals.var_u0_t_dn0 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn0)), ((locals.var_u0_t_dn2 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn2)), ((locals.var_u0_t_dn3 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn3)), ((locals.var_u0_t_dn4 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn4)), ((locals.var_u0_t_dn5 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn5)), ((locals.var_u0_t_dn6 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn6)), ((locals.var_u0_t_dn7 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn7)), ((locals.var_u0_t_dn8 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn8)), ((locals.var_u0_t_dn9 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn9)), ((locals.var_u0_t_dn10 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn10)), ((locals.var_u0_t_dn11 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn11)), ((locals.var_u0_t_dn12 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn12)), ((locals.var_u0_t_dn13 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn13)), ((locals.var_u0_t_dn14 * locals.var_mu0_mult) + (locals.var_u0_t * locals.var_mu0_mult_dn14)),)
    } else {
        (locals.var_u0_t, locals.var_u0_t_dn0, locals.var_u0_t_dn2, locals.var_u0_t_dn3, locals.var_u0_t_dn4, locals.var_u0_t_dn5, locals.var_u0_t_dn6, locals.var_u0_t_dn7, locals.var_u0_t_dn8, locals.var_u0_t_dn9, locals.var_u0_t_dn10, locals.var_u0_t_dn11, locals.var_u0_t_dn12, locals.var_u0_t_dn13, locals.var_u0_t_dn14,)
    }
};
        locals.var_u0_t = assign15890_e23289;
        locals.var_u0_t_dn0 = assign15890_e23289_d_n0;
        locals.var_u0_t_dn2 = assign15890_e23289_d_n2;
        locals.var_u0_t_dn3 = assign15890_e23289_d_n3;
        locals.var_u0_t_dn4 = assign15890_e23289_d_n4;
        locals.var_u0_t_dn5 = assign15890_e23289_d_n5;
        locals.var_u0_t_dn6 = assign15890_e23289_d_n6;
        locals.var_u0_t_dn7 = assign15890_e23289_d_n7;
        locals.var_u0_t_dn8 = assign15890_e23289_d_n8;
        locals.var_u0_t_dn9 = assign15890_e23289_d_n9;
        locals.var_u0_t_dn10 = assign15890_e23289_d_n10;
        locals.var_u0_t_dn11 = assign15890_e23289_d_n11;
        locals.var_u0_t_dn12 = assign15890_e23289_d_n12;
        locals.var_u0_t_dn13 = assign15890_e23289_d_n13;
        locals.var_u0_t_dn14 = assign15890_e23289_d_n14;
        locals.var_u0_t_rv = 0.0;

        let (assign15900_e23295, assign15900_e23295_d_n0, assign15900_e23295_d_n2, assign15900_e23295_d_n3, assign15900_e23295_d_n4, assign15900_e23295_d_n5, assign15900_e23295_d_n6, assign15900_e23295_d_n7, assign15900_e23295_d_n8, assign15900_e23295_d_n9, assign15900_e23295_d_n10, assign15900_e23295_d_n11, assign15900_e23295_d_n12, assign15900_e23295_d_n13, assign15900_e23295_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15900_e23293: f64 = (locals.var_vsat_t * locals.var_vsat_mult);
        (assign15900_e23293, ((locals.var_vsat_t_dn0 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn0)), ((locals.var_vsat_t_dn2 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn2)), ((locals.var_vsat_t_dn3 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn3)), ((locals.var_vsat_t_dn4 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn4)), ((locals.var_vsat_t_dn5 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn5)), ((locals.var_vsat_t_dn6 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn6)), ((locals.var_vsat_t_dn7 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn7)), ((locals.var_vsat_t_dn8 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn8)), ((locals.var_vsat_t_dn9 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn9)), ((locals.var_vsat_t_dn10 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn10)), ((locals.var_vsat_t_dn11 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn11)), ((locals.var_vsat_t_dn12 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn12)), ((locals.var_vsat_t_dn13 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn13)), ((locals.var_vsat_t_dn14 * locals.var_vsat_mult) + (locals.var_vsat_t * locals.var_vsat_mult_dn14)),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign15900_e23295;
        locals.var_vsat_t_dn0 = assign15900_e23295_d_n0;
        locals.var_vsat_t_dn2 = assign15900_e23295_d_n2;
        locals.var_vsat_t_dn3 = assign15900_e23295_d_n3;
        locals.var_vsat_t_dn4 = assign15900_e23295_d_n4;
        locals.var_vsat_t_dn5 = assign15900_e23295_d_n5;
        locals.var_vsat_t_dn6 = assign15900_e23295_d_n6;
        locals.var_vsat_t_dn7 = assign15900_e23295_d_n7;
        locals.var_vsat_t_dn8 = assign15900_e23295_d_n8;
        locals.var_vsat_t_dn9 = assign15900_e23295_d_n9;
        locals.var_vsat_t_dn10 = assign15900_e23295_d_n10;
        locals.var_vsat_t_dn11 = assign15900_e23295_d_n11;
        locals.var_vsat_t_dn12 = assign15900_e23295_d_n12;
        locals.var_vsat_t_dn13 = assign15900_e23295_d_n13;
        locals.var_vsat_t_dn14 = assign15900_e23295_d_n14;
        locals.var_vsat_t_rv = 0.0;

        let (assign15910_e23301, assign15910_e23301_d_n0, assign15910_e23301_d_n2, assign15910_e23301_d_n3, assign15910_e23301_d_n4, assign15910_e23301_d_n5, assign15910_e23301_d_n6, assign15910_e23301_d_n7, assign15910_e23301_d_n8, assign15910_e23301_d_n9, assign15910_e23301_d_n10, assign15910_e23301_d_n11, assign15910_e23301_d_n12, assign15910_e23301_d_n13, assign15910_e23301_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15910_e23299: f64 = (locals.var_k2_i + locals.var_k2_stress);
        (assign15910_e23299, (locals.var_k2_i_dn0 + locals.var_k2_stress_dn0), (locals.var_k2_i_dn2 + locals.var_k2_stress_dn2), (locals.var_k2_i_dn3 + locals.var_k2_stress_dn3), (locals.var_k2_i_dn4 + locals.var_k2_stress_dn4), (locals.var_k2_i_dn5 + locals.var_k2_stress_dn5), (locals.var_k2_i_dn6 + locals.var_k2_stress_dn6), (locals.var_k2_i_dn7 + locals.var_k2_stress_dn7), (locals.var_k2_i_dn8 + locals.var_k2_stress_dn8), (locals.var_k2_i_dn9 + locals.var_k2_stress_dn9), (locals.var_k2_i_dn10 + locals.var_k2_stress_dn10), (locals.var_k2_i_dn11 + locals.var_k2_stress_dn11), (locals.var_k2_i_dn12 + locals.var_k2_stress_dn12), (locals.var_k2_i_dn13 + locals.var_k2_stress_dn13), (locals.var_k2_i_dn14 + locals.var_k2_stress_dn14),)
    } else {
        (locals.var_k2_i, locals.var_k2_i_dn0, locals.var_k2_i_dn2, locals.var_k2_i_dn3, locals.var_k2_i_dn4, locals.var_k2_i_dn5, locals.var_k2_i_dn6, locals.var_k2_i_dn7, locals.var_k2_i_dn8, locals.var_k2_i_dn9, locals.var_k2_i_dn10, locals.var_k2_i_dn11, locals.var_k2_i_dn12, locals.var_k2_i_dn13, locals.var_k2_i_dn14,)
    }
};
        locals.var_k2_i = assign15910_e23301;
        locals.var_k2_i_dn0 = assign15910_e23301_d_n0;
        locals.var_k2_i_dn2 = assign15910_e23301_d_n2;
        locals.var_k2_i_dn3 = assign15910_e23301_d_n3;
        locals.var_k2_i_dn4 = assign15910_e23301_d_n4;
        locals.var_k2_i_dn5 = assign15910_e23301_d_n5;
        locals.var_k2_i_dn6 = assign15910_e23301_d_n6;
        locals.var_k2_i_dn7 = assign15910_e23301_d_n7;
        locals.var_k2_i_dn8 = assign15910_e23301_d_n8;
        locals.var_k2_i_dn9 = assign15910_e23301_d_n9;
        locals.var_k2_i_dn10 = assign15910_e23301_d_n10;
        locals.var_k2_i_dn11 = assign15910_e23301_d_n11;
        locals.var_k2_i_dn12 = assign15910_e23301_d_n12;
        locals.var_k2_i_dn13 = assign15910_e23301_d_n13;
        locals.var_k2_i_dn14 = assign15910_e23301_d_n14;
        locals.var_k2_i_rv = 0.0;

        let (assign15920_e23307, assign15920_e23307_d_n0, assign15920_e23307_d_n2, assign15920_e23307_d_n3, assign15920_e23307_d_n4, assign15920_e23307_d_n5, assign15920_e23307_d_n6, assign15920_e23307_d_n7, assign15920_e23307_d_n8, assign15920_e23307_d_n9, assign15920_e23307_d_n10, assign15920_e23307_d_n11, assign15920_e23307_d_n12, assign15920_e23307_d_n13, assign15920_e23307_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15920_e23305: f64 = (locals.var_eta0_t + locals.var_eta_stress);
        (assign15920_e23305, (locals.var_eta0_t_dn0 + locals.var_eta_stress_dn0), (locals.var_eta0_t_dn2 + locals.var_eta_stress_dn2), (locals.var_eta0_t_dn3 + locals.var_eta_stress_dn3), (locals.var_eta0_t_dn4 + locals.var_eta_stress_dn4), (locals.var_eta0_t_dn5 + locals.var_eta_stress_dn5), (locals.var_eta0_t_dn6 + locals.var_eta_stress_dn6), (locals.var_eta0_t_dn7 + locals.var_eta_stress_dn7), (locals.var_eta0_t_dn8 + locals.var_eta_stress_dn8), (locals.var_eta0_t_dn9 + locals.var_eta_stress_dn9), (locals.var_eta0_t_dn10 + locals.var_eta_stress_dn10), (locals.var_eta0_t_dn11 + locals.var_eta_stress_dn11), (locals.var_eta0_t_dn12 + locals.var_eta_stress_dn12), (locals.var_eta0_t_dn13 + locals.var_eta_stress_dn13), (locals.var_eta0_t_dn14 + locals.var_eta_stress_dn14),)
    } else {
        (locals.var_eta0_t, locals.var_eta0_t_dn0, locals.var_eta0_t_dn2, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11, locals.var_eta0_t_dn12, locals.var_eta0_t_dn13, locals.var_eta0_t_dn14,)
    }
};
        locals.var_eta0_t = assign15920_e23307;
        locals.var_eta0_t_dn0 = assign15920_e23307_d_n0;
        locals.var_eta0_t_dn2 = assign15920_e23307_d_n2;
        locals.var_eta0_t_dn3 = assign15920_e23307_d_n3;
        locals.var_eta0_t_dn4 = assign15920_e23307_d_n4;
        locals.var_eta0_t_dn5 = assign15920_e23307_d_n5;
        locals.var_eta0_t_dn6 = assign15920_e23307_d_n6;
        locals.var_eta0_t_dn7 = assign15920_e23307_d_n7;
        locals.var_eta0_t_dn8 = assign15920_e23307_d_n8;
        locals.var_eta0_t_dn9 = assign15920_e23307_d_n9;
        locals.var_eta0_t_dn10 = assign15920_e23307_d_n10;
        locals.var_eta0_t_dn11 = assign15920_e23307_d_n11;
        locals.var_eta0_t_dn12 = assign15920_e23307_d_n12;
        locals.var_eta0_t_dn13 = assign15920_e23307_d_n13;
        locals.var_eta0_t_dn14 = assign15920_e23307_d_n14;
        locals.var_eta0_t_rv = 0.0;

        let assign15930_e23310: f64 = if p.p37 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard489 = assign15930_e23310;
        locals.var_guard489_rv = 0.0;

        let (assign15940_e23322, assign15940_e23322_d_n0, assign15940_e23322_d_n2, assign15940_e23322_d_n3, assign15940_e23322_d_n4, assign15940_e23322_d_n5, assign15940_e23322_d_n6, assign15940_e23322_d_n7, assign15940_e23322_d_n8, assign15940_e23322_d_n9, assign15940_e23322_d_n10, assign15940_e23322_d_n11, assign15940_e23322_d_n12, assign15940_e23322_d_n13, assign15940_e23322_d_n14,) = {
    if ((locals.var_guard488 != 0.0) && (locals.var_guard489 != 0.0)) {
        let assign15940_e23316: f64 = (locals.var_kvth0edge_i / locals.var_kstress_vth0);
        let assign15940_e23319: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15940_e23320: f64 = (assign15940_e23316 * assign15940_e23319);
        (assign15940_e23320, (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn0) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn0)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn2) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn2)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn3) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn3)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn4) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn4)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn5) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn5)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn6) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn6)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn7) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn7)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn8) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn8)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn9) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn9)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn10) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn10)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn11) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn11)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn12) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn12)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn13) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn13)), (((-((locals.var_kvth0edge_i * locals.var_kstress_vth0_dn14) / (locals.var_kstress_vth0 * locals.var_kstress_vth0))) * assign15940_e23319) + (assign15940_e23316 * locals.var_inv_od_dn14)),)
    } else {
        (locals.var_vth0_stress_edge, locals.var_vth0_stress_edge_dn0, locals.var_vth0_stress_edge_dn2, locals.var_vth0_stress_edge_dn3, locals.var_vth0_stress_edge_dn4, locals.var_vth0_stress_edge_dn5, locals.var_vth0_stress_edge_dn6, locals.var_vth0_stress_edge_dn7, locals.var_vth0_stress_edge_dn8, locals.var_vth0_stress_edge_dn9, locals.var_vth0_stress_edge_dn10, locals.var_vth0_stress_edge_dn11, locals.var_vth0_stress_edge_dn12, locals.var_vth0_stress_edge_dn13, locals.var_vth0_stress_edge_dn14,)
    }
};
        locals.var_vth0_stress_edge = assign15940_e23322;
        locals.var_vth0_stress_edge_dn0 = assign15940_e23322_d_n0;
        locals.var_vth0_stress_edge_dn2 = assign15940_e23322_d_n2;
        locals.var_vth0_stress_edge_dn3 = assign15940_e23322_d_n3;
        locals.var_vth0_stress_edge_dn4 = assign15940_e23322_d_n4;
        locals.var_vth0_stress_edge_dn5 = assign15940_e23322_d_n5;
        locals.var_vth0_stress_edge_dn6 = assign15940_e23322_d_n6;
        locals.var_vth0_stress_edge_dn7 = assign15940_e23322_d_n7;
        locals.var_vth0_stress_edge_dn8 = assign15940_e23322_d_n8;
        locals.var_vth0_stress_edge_dn9 = assign15940_e23322_d_n9;
        locals.var_vth0_stress_edge_dn10 = assign15940_e23322_d_n10;
        locals.var_vth0_stress_edge_dn11 = assign15940_e23322_d_n11;
        locals.var_vth0_stress_edge_dn12 = assign15940_e23322_d_n12;
        locals.var_vth0_stress_edge_dn13 = assign15940_e23322_d_n13;
        locals.var_vth0_stress_edge_dn14 = assign15940_e23322_d_n14;
        locals.var_vth0_stress_edge_rv = 0.0;

        let (assign15950_e23336, assign15950_e23336_d_n0, assign15950_e23336_d_n2, assign15950_e23336_d_n3, assign15950_e23336_d_n4, assign15950_e23336_d_n5, assign15950_e23336_d_n6, assign15950_e23336_d_n7, assign15950_e23336_d_n8, assign15950_e23336_d_n9, assign15950_e23336_d_n10, assign15950_e23336_d_n11, assign15950_e23336_d_n12, assign15950_e23336_d_n13, assign15950_e23336_d_n14,) = {
    if ((locals.var_guard488 != 0.0) && (locals.var_guard489 != 0.0)) {
        let assign15950_e23329: f64 = (locals.var_kstress_vth0).powf(p.p930);
        let assign15950_e23330: f64 = (locals.var_stk2edge_i / assign15950_e23329);
        let assign15950_e23333: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15950_e23334: f64 = (assign15950_e23330 * assign15950_e23333);
        (assign15950_e23334, (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn0)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn0 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn0)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn2)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn2 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn2)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn3)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn4)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn5)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn6)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn7)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn8)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn9)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn10)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn11)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn12)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn12 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn12)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn13)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn13 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn13)), (((-((locals.var_stk2edge_i * if 0.0 == 0.0 && ((p.p930) as f64).is_finite() && ((p.p930) as f64).fract() == 0.0 { if p.p930 == 0.0 { 0.0 } else { (p.p930 * ((locals.var_kstress_vth0).powf(p.p930 - 1.0) * locals.var_kstress_vth0_dn14)) } } else { (assign15950_e23329 * (p.p930 * (locals.var_kstress_vth0_dn14 / locals.var_kstress_vth0))) }) / (assign15950_e23329 * assign15950_e23329))) * assign15950_e23333) + (assign15950_e23330 * locals.var_inv_od_dn14)),)
    } else {
        (locals.var_k2_stress_edge, locals.var_k2_stress_edge_dn0, locals.var_k2_stress_edge_dn2, locals.var_k2_stress_edge_dn3, locals.var_k2_stress_edge_dn4, locals.var_k2_stress_edge_dn5, locals.var_k2_stress_edge_dn6, locals.var_k2_stress_edge_dn7, locals.var_k2_stress_edge_dn8, locals.var_k2_stress_edge_dn9, locals.var_k2_stress_edge_dn10, locals.var_k2_stress_edge_dn11, locals.var_k2_stress_edge_dn12, locals.var_k2_stress_edge_dn13, locals.var_k2_stress_edge_dn14,)
    }
};
        locals.var_k2_stress_edge = assign15950_e23336;
        locals.var_k2_stress_edge_dn0 = assign15950_e23336_d_n0;
        locals.var_k2_stress_edge_dn2 = assign15950_e23336_d_n2;
        locals.var_k2_stress_edge_dn3 = assign15950_e23336_d_n3;
        locals.var_k2_stress_edge_dn4 = assign15950_e23336_d_n4;
        locals.var_k2_stress_edge_dn5 = assign15950_e23336_d_n5;
        locals.var_k2_stress_edge_dn6 = assign15950_e23336_d_n6;
        locals.var_k2_stress_edge_dn7 = assign15950_e23336_d_n7;
        locals.var_k2_stress_edge_dn8 = assign15950_e23336_d_n8;
        locals.var_k2_stress_edge_dn9 = assign15950_e23336_d_n9;
        locals.var_k2_stress_edge_dn10 = assign15950_e23336_d_n10;
        locals.var_k2_stress_edge_dn11 = assign15950_e23336_d_n11;
        locals.var_k2_stress_edge_dn12 = assign15950_e23336_d_n12;
        locals.var_k2_stress_edge_dn13 = assign15950_e23336_d_n13;
        locals.var_k2_stress_edge_dn14 = assign15950_e23336_d_n14;
        locals.var_k2_stress_edge_rv = 0.0;

        let (assign15960_e23350, assign15960_e23350_d_n0, assign15960_e23350_d_n2, assign15960_e23350_d_n3, assign15960_e23350_d_n4, assign15960_e23350_d_n5, assign15960_e23350_d_n6, assign15960_e23350_d_n7, assign15960_e23350_d_n8, assign15960_e23350_d_n9, assign15960_e23350_d_n10, assign15960_e23350_d_n11, assign15960_e23350_d_n12, assign15960_e23350_d_n13, assign15960_e23350_d_n14,) = {
    if ((locals.var_guard488 != 0.0) && (locals.var_guard489 != 0.0)) {
        let assign15960_e23343: f64 = (locals.var_kstress_vth0).powf(p.p932);
        let assign15960_e23344: f64 = (locals.var_steta0edge_i / assign15960_e23343);
        let assign15960_e23347: f64 = (locals.var_inv_od - locals.var_inv_odref);
        let assign15960_e23348: f64 = (assign15960_e23344 * assign15960_e23347);
        (assign15960_e23348, (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn0)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn0 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn0)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn2)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn2 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn2)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn3)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn3 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn3)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn4)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn4 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn4)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn5)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn5 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn5)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn6)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn6 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn6)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn7)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn7 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn7)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn8)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn8 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn8)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn9)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn9 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn9)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn10)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn10 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn10)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn11)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn11 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn11)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn12)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn12 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn12)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn13)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn13 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn13)), (((-((locals.var_steta0edge_i * if 0.0 == 0.0 && ((p.p932) as f64).is_finite() && ((p.p932) as f64).fract() == 0.0 { if p.p932 == 0.0 { 0.0 } else { (p.p932 * ((locals.var_kstress_vth0).powf(p.p932 - 1.0) * locals.var_kstress_vth0_dn14)) } } else { (assign15960_e23343 * (p.p932 * (locals.var_kstress_vth0_dn14 / locals.var_kstress_vth0))) }) / (assign15960_e23343 * assign15960_e23343))) * assign15960_e23347) + (assign15960_e23344 * locals.var_inv_od_dn14)),)
    } else {
        (locals.var_eta_stress_edge, locals.var_eta_stress_edge_dn0, locals.var_eta_stress_edge_dn2, locals.var_eta_stress_edge_dn3, locals.var_eta_stress_edge_dn4, locals.var_eta_stress_edge_dn5, locals.var_eta_stress_edge_dn6, locals.var_eta_stress_edge_dn7, locals.var_eta_stress_edge_dn8, locals.var_eta_stress_edge_dn9, locals.var_eta_stress_edge_dn10, locals.var_eta_stress_edge_dn11, locals.var_eta_stress_edge_dn12, locals.var_eta_stress_edge_dn13, locals.var_eta_stress_edge_dn14,)
    }
};
        locals.var_eta_stress_edge = assign15960_e23350;
        locals.var_eta_stress_edge_dn0 = assign15960_e23350_d_n0;
        locals.var_eta_stress_edge_dn2 = assign15960_e23350_d_n2;
        locals.var_eta_stress_edge_dn3 = assign15960_e23350_d_n3;
        locals.var_eta_stress_edge_dn4 = assign15960_e23350_d_n4;
        locals.var_eta_stress_edge_dn5 = assign15960_e23350_d_n5;
        locals.var_eta_stress_edge_dn6 = assign15960_e23350_d_n6;
        locals.var_eta_stress_edge_dn7 = assign15960_e23350_d_n7;
        locals.var_eta_stress_edge_dn8 = assign15960_e23350_d_n8;
        locals.var_eta_stress_edge_dn9 = assign15960_e23350_d_n9;
        locals.var_eta_stress_edge_dn10 = assign15960_e23350_d_n10;
        locals.var_eta_stress_edge_dn11 = assign15960_e23350_d_n11;
        locals.var_eta_stress_edge_dn12 = assign15960_e23350_d_n12;
        locals.var_eta_stress_edge_dn13 = assign15960_e23350_d_n13;
        locals.var_eta_stress_edge_dn14 = assign15960_e23350_d_n14;
        locals.var_eta_stress_edge_rv = 0.0;

        let (assign15970_e23356, assign15970_e23356_d_n0, assign15970_e23356_d_n2, assign15970_e23356_d_n3, assign15970_e23356_d_n4, assign15970_e23356_d_n5, assign15970_e23356_d_n6, assign15970_e23356_d_n7, assign15970_e23356_d_n8, assign15970_e23356_d_n9, assign15970_e23356_d_n10, assign15970_e23356_d_n11, assign15970_e23356_d_n12, assign15970_e23356_d_n13, assign15970_e23356_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15970_e23354: f64 = (locals.var_k2edge_i + locals.var_k2_stress_edge);
        (assign15970_e23354, (locals.var_k2edge_i_dn0 + locals.var_k2_stress_edge_dn0), (locals.var_k2edge_i_dn2 + locals.var_k2_stress_edge_dn2), (locals.var_k2edge_i_dn3 + locals.var_k2_stress_edge_dn3), (locals.var_k2edge_i_dn4 + locals.var_k2_stress_edge_dn4), (locals.var_k2edge_i_dn5 + locals.var_k2_stress_edge_dn5), (locals.var_k2edge_i_dn6 + locals.var_k2_stress_edge_dn6), (locals.var_k2edge_i_dn7 + locals.var_k2_stress_edge_dn7), (locals.var_k2edge_i_dn8 + locals.var_k2_stress_edge_dn8), (locals.var_k2edge_i_dn9 + locals.var_k2_stress_edge_dn9), (locals.var_k2edge_i_dn10 + locals.var_k2_stress_edge_dn10), (locals.var_k2edge_i_dn11 + locals.var_k2_stress_edge_dn11), (locals.var_k2edge_i_dn12 + locals.var_k2_stress_edge_dn12), (locals.var_k2edge_i_dn13 + locals.var_k2_stress_edge_dn13), (locals.var_k2edge_i_dn14 + locals.var_k2_stress_edge_dn14),)
    } else {
        (locals.var_k2edge_i, locals.var_k2edge_i_dn0, locals.var_k2edge_i_dn2, locals.var_k2edge_i_dn3, locals.var_k2edge_i_dn4, locals.var_k2edge_i_dn5, locals.var_k2edge_i_dn6, locals.var_k2edge_i_dn7, locals.var_k2edge_i_dn8, locals.var_k2edge_i_dn9, locals.var_k2edge_i_dn10, locals.var_k2edge_i_dn11, locals.var_k2edge_i_dn12, locals.var_k2edge_i_dn13, locals.var_k2edge_i_dn14,)
    }
};
        locals.var_k2edge_i = assign15970_e23356;
        locals.var_k2edge_i_dn0 = assign15970_e23356_d_n0;
        locals.var_k2edge_i_dn2 = assign15970_e23356_d_n2;
        locals.var_k2edge_i_dn3 = assign15970_e23356_d_n3;
        locals.var_k2edge_i_dn4 = assign15970_e23356_d_n4;
        locals.var_k2edge_i_dn5 = assign15970_e23356_d_n5;
        locals.var_k2edge_i_dn6 = assign15970_e23356_d_n6;
        locals.var_k2edge_i_dn7 = assign15970_e23356_d_n7;
        locals.var_k2edge_i_dn8 = assign15970_e23356_d_n8;
        locals.var_k2edge_i_dn9 = assign15970_e23356_d_n9;
        locals.var_k2edge_i_dn10 = assign15970_e23356_d_n10;
        locals.var_k2edge_i_dn11 = assign15970_e23356_d_n11;
        locals.var_k2edge_i_dn12 = assign15970_e23356_d_n12;
        locals.var_k2edge_i_dn13 = assign15970_e23356_d_n13;
        locals.var_k2edge_i_dn14 = assign15970_e23356_d_n14;
        locals.var_k2edge_i_rv = 0.0;

        let (assign15980_e23362, assign15980_e23362_d_n0, assign15980_e23362_d_n2, assign15980_e23362_d_n3, assign15980_e23362_d_n4, assign15980_e23362_d_n5, assign15980_e23362_d_n6, assign15980_e23362_d_n7, assign15980_e23362_d_n8, assign15980_e23362_d_n9, assign15980_e23362_d_n10, assign15980_e23362_d_n11, assign15980_e23362_d_n12, assign15980_e23362_d_n13, assign15980_e23362_d_n14,) = {
    if (locals.var_guard488 != 0.0) {
        let assign15980_e23360: f64 = (locals.var_eta0edge_i + locals.var_eta_stress_edge);
        (assign15980_e23360, (locals.var_eta0edge_i_dn0 + locals.var_eta_stress_edge_dn0), (locals.var_eta0edge_i_dn2 + locals.var_eta_stress_edge_dn2), (locals.var_eta0edge_i_dn3 + locals.var_eta_stress_edge_dn3), (locals.var_eta0edge_i_dn4 + locals.var_eta_stress_edge_dn4), (locals.var_eta0edge_i_dn5 + locals.var_eta_stress_edge_dn5), (locals.var_eta0edge_i_dn6 + locals.var_eta_stress_edge_dn6), (locals.var_eta0edge_i_dn7 + locals.var_eta_stress_edge_dn7), (locals.var_eta0edge_i_dn8 + locals.var_eta_stress_edge_dn8), (locals.var_eta0edge_i_dn9 + locals.var_eta_stress_edge_dn9), (locals.var_eta0edge_i_dn10 + locals.var_eta_stress_edge_dn10), (locals.var_eta0edge_i_dn11 + locals.var_eta_stress_edge_dn11), (locals.var_eta0edge_i_dn12 + locals.var_eta_stress_edge_dn12), (locals.var_eta0edge_i_dn13 + locals.var_eta_stress_edge_dn13), (locals.var_eta0edge_i_dn14 + locals.var_eta_stress_edge_dn14),)
    } else {
        (locals.var_eta0edge_i, locals.var_eta0edge_i_dn0, locals.var_eta0edge_i_dn2, locals.var_eta0edge_i_dn3, locals.var_eta0edge_i_dn4, locals.var_eta0edge_i_dn5, locals.var_eta0edge_i_dn6, locals.var_eta0edge_i_dn7, locals.var_eta0edge_i_dn8, locals.var_eta0edge_i_dn9, locals.var_eta0edge_i_dn10, locals.var_eta0edge_i_dn11, locals.var_eta0edge_i_dn12, locals.var_eta0edge_i_dn13, locals.var_eta0edge_i_dn14,)
    }
};
        locals.var_eta0edge_i = assign15980_e23362;
        locals.var_eta0edge_i_dn0 = assign15980_e23362_d_n0;
        locals.var_eta0edge_i_dn2 = assign15980_e23362_d_n2;
        locals.var_eta0edge_i_dn3 = assign15980_e23362_d_n3;
        locals.var_eta0edge_i_dn4 = assign15980_e23362_d_n4;
        locals.var_eta0edge_i_dn5 = assign15980_e23362_d_n5;
        locals.var_eta0edge_i_dn6 = assign15980_e23362_d_n6;
        locals.var_eta0edge_i_dn7 = assign15980_e23362_d_n7;
        locals.var_eta0edge_i_dn8 = assign15980_e23362_d_n8;
        locals.var_eta0edge_i_dn9 = assign15980_e23362_d_n9;
        locals.var_eta0edge_i_dn10 = assign15980_e23362_d_n10;
        locals.var_eta0edge_i_dn11 = assign15980_e23362_d_n11;
        locals.var_eta0edge_i_dn12 = assign15980_e23362_d_n12;
        locals.var_eta0edge_i_dn13 = assign15980_e23362_d_n13;
        locals.var_eta0edge_i_dn14 = assign15980_e23362_d_n14;
        locals.var_eta0edge_i_rv = 0.0;

        let (assign15990_e23367, assign15990_e23367_d_n0, assign15990_e23367_d_n2, assign15990_e23367_d_n3, assign15990_e23367_d_n4, assign15990_e23367_d_n5, assign15990_e23367_d_n6, assign15990_e23367_d_n7, assign15990_e23367_d_n8, assign15990_e23367_d_n9, assign15990_e23367_d_n10, assign15990_e23367_d_n11, assign15990_e23367_d_n12, assign15990_e23367_d_n13, assign15990_e23367_d_n14,) = {
    if (locals.var_guard488 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vth0_stress, locals.var_vth0_stress_dn0, locals.var_vth0_stress_dn2, locals.var_vth0_stress_dn3, locals.var_vth0_stress_dn4, locals.var_vth0_stress_dn5, locals.var_vth0_stress_dn6, locals.var_vth0_stress_dn7, locals.var_vth0_stress_dn8, locals.var_vth0_stress_dn9, locals.var_vth0_stress_dn10, locals.var_vth0_stress_dn11, locals.var_vth0_stress_dn12, locals.var_vth0_stress_dn13, locals.var_vth0_stress_dn14,)
    }
};
        locals.var_vth0_stress = assign15990_e23367;
        locals.var_vth0_stress_dn0 = assign15990_e23367_d_n0;
        locals.var_vth0_stress_dn2 = assign15990_e23367_d_n2;
        locals.var_vth0_stress_dn3 = assign15990_e23367_d_n3;
        locals.var_vth0_stress_dn4 = assign15990_e23367_d_n4;
        locals.var_vth0_stress_dn5 = assign15990_e23367_d_n5;
        locals.var_vth0_stress_dn6 = assign15990_e23367_d_n6;
        locals.var_vth0_stress_dn7 = assign15990_e23367_d_n7;
        locals.var_vth0_stress_dn8 = assign15990_e23367_d_n8;
        locals.var_vth0_stress_dn9 = assign15990_e23367_d_n9;
        locals.var_vth0_stress_dn10 = assign15990_e23367_d_n10;
        locals.var_vth0_stress_dn11 = assign15990_e23367_d_n11;
        locals.var_vth0_stress_dn12 = assign15990_e23367_d_n12;
        locals.var_vth0_stress_dn13 = assign15990_e23367_d_n13;
        locals.var_vth0_stress_dn14 = assign15990_e23367_d_n14;
        locals.var_vth0_stress_rv = 0.0;

        let (assign16000_e23372, assign16000_e23372_d_n0, assign16000_e23372_d_n2, assign16000_e23372_d_n3, assign16000_e23372_d_n4, assign16000_e23372_d_n5, assign16000_e23372_d_n6, assign16000_e23372_d_n7, assign16000_e23372_d_n8, assign16000_e23372_d_n9, assign16000_e23372_d_n10, assign16000_e23372_d_n11, assign16000_e23372_d_n12, assign16000_e23372_d_n13, assign16000_e23372_d_n14,) = {
    if (locals.var_guard488 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vth0_stress_edge, locals.var_vth0_stress_edge_dn0, locals.var_vth0_stress_edge_dn2, locals.var_vth0_stress_edge_dn3, locals.var_vth0_stress_edge_dn4, locals.var_vth0_stress_edge_dn5, locals.var_vth0_stress_edge_dn6, locals.var_vth0_stress_edge_dn7, locals.var_vth0_stress_edge_dn8, locals.var_vth0_stress_edge_dn9, locals.var_vth0_stress_edge_dn10, locals.var_vth0_stress_edge_dn11, locals.var_vth0_stress_edge_dn12, locals.var_vth0_stress_edge_dn13, locals.var_vth0_stress_edge_dn14,)
    }
};
        locals.var_vth0_stress_edge = assign16000_e23372;
        locals.var_vth0_stress_edge_dn0 = assign16000_e23372_d_n0;
        locals.var_vth0_stress_edge_dn2 = assign16000_e23372_d_n2;
        locals.var_vth0_stress_edge_dn3 = assign16000_e23372_d_n3;
        locals.var_vth0_stress_edge_dn4 = assign16000_e23372_d_n4;
        locals.var_vth0_stress_edge_dn5 = assign16000_e23372_d_n5;
        locals.var_vth0_stress_edge_dn6 = assign16000_e23372_d_n6;
        locals.var_vth0_stress_edge_dn7 = assign16000_e23372_d_n7;
        locals.var_vth0_stress_edge_dn8 = assign16000_e23372_d_n8;
        locals.var_vth0_stress_edge_dn9 = assign16000_e23372_d_n9;
        locals.var_vth0_stress_edge_dn10 = assign16000_e23372_d_n10;
        locals.var_vth0_stress_edge_dn11 = assign16000_e23372_d_n11;
        locals.var_vth0_stress_edge_dn12 = assign16000_e23372_d_n12;
        locals.var_vth0_stress_edge_dn13 = assign16000_e23372_d_n13;
        locals.var_vth0_stress_edge_dn14 = assign16000_e23372_d_n14;
        locals.var_vth0_stress_edge_rv = 0.0;

        let assign16010_e23375: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard490 = assign16010_e23375;
        locals.var_guard490_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_36(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (assign16020_e23381,) = {
    if (locals.var_guard490 != 0.0) {
        let assign16020_e23379: f64 = (p.p1 / p.p2);
        (assign16020_e23379,)
    } else {
        (locals.var_wdrn,)
    }
};
        locals.var_wdrn = assign16020_e23381;
        locals.var_wdrn_rv = 0.0;

        let (assign16030_e23385, assign16030_e23385_d_n0, assign16030_e23385_d_n2, assign16030_e23385_d_n3, assign16030_e23385_d_n4, assign16030_e23385_d_n5, assign16030_e23385_d_n6, assign16030_e23385_d_n7, assign16030_e23385_d_n8, assign16030_e23385_d_n9, assign16030_e23385_d_n10, assign16030_e23385_d_n11, assign16030_e23385_d_n12, assign16030_e23385_d_n13, assign16030_e23385_d_n14,) = {
    if (locals.var_guard490 != 0.0) {
        (p.p20, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_local_sca, locals.var_local_sca_dn0, locals.var_local_sca_dn2, locals.var_local_sca_dn3, locals.var_local_sca_dn4, locals.var_local_sca_dn5, locals.var_local_sca_dn6, locals.var_local_sca_dn7, locals.var_local_sca_dn8, locals.var_local_sca_dn9, locals.var_local_sca_dn10, locals.var_local_sca_dn11, locals.var_local_sca_dn12, locals.var_local_sca_dn13, locals.var_local_sca_dn14,)
    }
};
        locals.var_local_sca = assign16030_e23385;
        locals.var_local_sca_dn0 = assign16030_e23385_d_n0;
        locals.var_local_sca_dn2 = assign16030_e23385_d_n2;
        locals.var_local_sca_dn3 = assign16030_e23385_d_n3;
        locals.var_local_sca_dn4 = assign16030_e23385_d_n4;
        locals.var_local_sca_dn5 = assign16030_e23385_d_n5;
        locals.var_local_sca_dn6 = assign16030_e23385_d_n6;
        locals.var_local_sca_dn7 = assign16030_e23385_d_n7;
        locals.var_local_sca_dn8 = assign16030_e23385_d_n8;
        locals.var_local_sca_dn9 = assign16030_e23385_d_n9;
        locals.var_local_sca_dn10 = assign16030_e23385_d_n10;
        locals.var_local_sca_dn11 = assign16030_e23385_d_n11;
        locals.var_local_sca_dn12 = assign16030_e23385_d_n12;
        locals.var_local_sca_dn13 = assign16030_e23385_d_n13;
        locals.var_local_sca_dn14 = assign16030_e23385_d_n14;
        locals.var_local_sca_rv = 0.0;

        let (assign16040_e23389, assign16040_e23389_d_n0, assign16040_e23389_d_n2, assign16040_e23389_d_n3, assign16040_e23389_d_n4, assign16040_e23389_d_n5, assign16040_e23389_d_n6, assign16040_e23389_d_n7, assign16040_e23389_d_n8, assign16040_e23389_d_n9, assign16040_e23389_d_n10, assign16040_e23389_d_n11, assign16040_e23389_d_n12, assign16040_e23389_d_n13, assign16040_e23389_d_n14,) = {
    if (locals.var_guard490 != 0.0) {
        (p.p21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_local_scb, locals.var_local_scb_dn0, locals.var_local_scb_dn2, locals.var_local_scb_dn3, locals.var_local_scb_dn4, locals.var_local_scb_dn5, locals.var_local_scb_dn6, locals.var_local_scb_dn7, locals.var_local_scb_dn8, locals.var_local_scb_dn9, locals.var_local_scb_dn10, locals.var_local_scb_dn11, locals.var_local_scb_dn12, locals.var_local_scb_dn13, locals.var_local_scb_dn14,)
    }
};
        locals.var_local_scb = assign16040_e23389;
        locals.var_local_scb_dn0 = assign16040_e23389_d_n0;
        locals.var_local_scb_dn2 = assign16040_e23389_d_n2;
        locals.var_local_scb_dn3 = assign16040_e23389_d_n3;
        locals.var_local_scb_dn4 = assign16040_e23389_d_n4;
        locals.var_local_scb_dn5 = assign16040_e23389_d_n5;
        locals.var_local_scb_dn6 = assign16040_e23389_d_n6;
        locals.var_local_scb_dn7 = assign16040_e23389_d_n7;
        locals.var_local_scb_dn8 = assign16040_e23389_d_n8;
        locals.var_local_scb_dn9 = assign16040_e23389_d_n9;
        locals.var_local_scb_dn10 = assign16040_e23389_d_n10;
        locals.var_local_scb_dn11 = assign16040_e23389_d_n11;
        locals.var_local_scb_dn12 = assign16040_e23389_d_n12;
        locals.var_local_scb_dn13 = assign16040_e23389_d_n13;
        locals.var_local_scb_dn14 = assign16040_e23389_d_n14;
        locals.var_local_scb_rv = 0.0;

        let (assign16050_e23393, assign16050_e23393_d_n0, assign16050_e23393_d_n2, assign16050_e23393_d_n3, assign16050_e23393_d_n4, assign16050_e23393_d_n5, assign16050_e23393_d_n6, assign16050_e23393_d_n7, assign16050_e23393_d_n8, assign16050_e23393_d_n9, assign16050_e23393_d_n10, assign16050_e23393_d_n11, assign16050_e23393_d_n12, assign16050_e23393_d_n13, assign16050_e23393_d_n14,) = {
    if (locals.var_guard490 != 0.0) {
        (p.p22, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_local_scc, locals.var_local_scc_dn0, locals.var_local_scc_dn2, locals.var_local_scc_dn3, locals.var_local_scc_dn4, locals.var_local_scc_dn5, locals.var_local_scc_dn6, locals.var_local_scc_dn7, locals.var_local_scc_dn8, locals.var_local_scc_dn9, locals.var_local_scc_dn10, locals.var_local_scc_dn11, locals.var_local_scc_dn12, locals.var_local_scc_dn13, locals.var_local_scc_dn14,)
    }
};
        locals.var_local_scc = assign16050_e23393;
        locals.var_local_scc_dn0 = assign16050_e23393_d_n0;
        locals.var_local_scc_dn2 = assign16050_e23393_d_n2;
        locals.var_local_scc_dn3 = assign16050_e23393_d_n3;
        locals.var_local_scc_dn4 = assign16050_e23393_d_n4;
        locals.var_local_scc_dn5 = assign16050_e23393_d_n5;
        locals.var_local_scc_dn6 = assign16050_e23393_d_n6;
        locals.var_local_scc_dn7 = assign16050_e23393_d_n7;
        locals.var_local_scc_dn8 = assign16050_e23393_d_n8;
        locals.var_local_scc_dn9 = assign16050_e23393_d_n9;
        locals.var_local_scc_dn10 = assign16050_e23393_d_n10;
        locals.var_local_scc_dn11 = assign16050_e23393_d_n11;
        locals.var_local_scc_dn12 = assign16050_e23393_d_n12;
        locals.var_local_scc_dn13 = assign16050_e23393_d_n13;
        locals.var_local_scc_dn14 = assign16050_e23393_d_n14;
        locals.var_local_scc_rv = 0.0;

        let assign16060_e23404: f64 = if (((!param_given[20]) && (!param_given[21])) && (!param_given[22])) { 1.0 } else { 0.0 };
        locals.var_guard491 = assign16060_e23404;
        locals.var_guard491_rv = 0.0;

        let assign16070_e23410: f64 = if (param_given[23] && (p.p23 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard492 = assign16070_e23410;
        locals.var_guard492_rv = 0.0;

        let (assign16080_e23420, assign16080_e23420_d_n0, assign16080_e23420_d_n2, assign16080_e23420_d_n3, assign16080_e23420_d_n4, assign16080_e23420_d_n5, assign16080_e23420_d_n6, assign16080_e23420_d_n7, assign16080_e23420_d_n8, assign16080_e23420_d_n9, assign16080_e23420_d_n10, assign16080_e23420_d_n11, assign16080_e23420_d_n12, assign16080_e23420_d_n13, assign16080_e23420_d_n14,) = {
    if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign16080_e23418: f64 = (p.p23 + locals.var_wdrn);
        (assign16080_e23418, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign16080_e23420;
        locals.var_t1_dn0 = assign16080_e23420_d_n0;
        locals.var_t1_dn2 = assign16080_e23420_d_n2;
        locals.var_t1_dn3 = assign16080_e23420_d_n3;
        locals.var_t1_dn4 = assign16080_e23420_d_n4;
        locals.var_t1_dn5 = assign16080_e23420_d_n5;
        locals.var_t1_dn6 = assign16080_e23420_d_n6;
        locals.var_t1_dn7 = assign16080_e23420_d_n7;
        locals.var_t1_dn8 = assign16080_e23420_d_n8;
        locals.var_t1_dn9 = assign16080_e23420_d_n9;
        locals.var_t1_dn10 = assign16080_e23420_d_n10;
        locals.var_t1_dn11 = assign16080_e23420_d_n11;
        locals.var_t1_dn12 = assign16080_e23420_d_n12;
        locals.var_t1_dn13 = assign16080_e23420_d_n13;
        locals.var_t1_dn14 = assign16080_e23420_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign16090_e23430, assign16090_e23430_d_n0, assign16090_e23430_d_n2, assign16090_e23430_d_n3, assign16090_e23430_d_n4, assign16090_e23430_d_n5, assign16090_e23430_d_n6, assign16090_e23430_d_n7, assign16090_e23430_d_n8, assign16090_e23430_d_n9, assign16090_e23430_d_n10, assign16090_e23430_d_n11, assign16090_e23430_d_n12, assign16090_e23430_d_n13, assign16090_e23430_d_n14,) = {
    if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign16090_e23428: f64 = (1.0 / p.p947);
        (assign16090_e23428, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign16090_e23430;
        locals.var_t2_dn0 = assign16090_e23430_d_n0;
        locals.var_t2_dn2 = assign16090_e23430_d_n2;
        locals.var_t2_dn3 = assign16090_e23430_d_n3;
        locals.var_t2_dn4 = assign16090_e23430_d_n4;
        locals.var_t2_dn5 = assign16090_e23430_d_n5;
        locals.var_t2_dn6 = assign16090_e23430_d_n6;
        locals.var_t2_dn7 = assign16090_e23430_d_n7;
        locals.var_t2_dn8 = assign16090_e23430_d_n8;
        locals.var_t2_dn9 = assign16090_e23430_d_n9;
        locals.var_t2_dn10 = assign16090_e23430_d_n10;
        locals.var_t2_dn11 = assign16090_e23430_d_n11;
        locals.var_t2_dn12 = assign16090_e23430_d_n12;
        locals.var_t2_dn13 = assign16090_e23430_d_n13;
        locals.var_t2_dn14 = assign16090_e23430_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign16100_e23444, assign16100_e23444_d_n0, assign16100_e23444_d_n2, assign16100_e23444_d_n3, assign16100_e23444_d_n4, assign16100_e23444_d_n5, assign16100_e23444_d_n6, assign16100_e23444_d_n7, assign16100_e23444_d_n8, assign16100_e23444_d_n9, assign16100_e23444_d_n10, assign16100_e23444_d_n11, assign16100_e23444_d_n12, assign16100_e23444_d_n13, assign16100_e23444_d_n14,) = {
    if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign16100_e23438: f64 = (p.p947 * p.p947);
        let assign16100_e23441: f64 = (p.p23 * locals.var_t1);
        let assign16100_e23442: f64 = (assign16100_e23438 / assign16100_e23441);
        (assign16100_e23442, (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn0)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn2)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn3)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn4)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn5)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn6)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn7)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn8)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn9)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn10)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn11)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn12)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn13)) / (assign16100_e23441 * assign16100_e23441))), (-((assign16100_e23438 * (p.p23 * locals.var_t1_dn14)) / (assign16100_e23441 * assign16100_e23441))),)
    } else {
        (locals.var_local_sca, locals.var_local_sca_dn0, locals.var_local_sca_dn2, locals.var_local_sca_dn3, locals.var_local_sca_dn4, locals.var_local_sca_dn5, locals.var_local_sca_dn6, locals.var_local_sca_dn7, locals.var_local_sca_dn8, locals.var_local_sca_dn9, locals.var_local_sca_dn10, locals.var_local_sca_dn11, locals.var_local_sca_dn12, locals.var_local_sca_dn13, locals.var_local_sca_dn14,)
    }
};
        locals.var_local_sca = assign16100_e23444;
        locals.var_local_sca_dn0 = assign16100_e23444_d_n0;
        locals.var_local_sca_dn2 = assign16100_e23444_d_n2;
        locals.var_local_sca_dn3 = assign16100_e23444_d_n3;
        locals.var_local_sca_dn4 = assign16100_e23444_d_n4;
        locals.var_local_sca_dn5 = assign16100_e23444_d_n5;
        locals.var_local_sca_dn6 = assign16100_e23444_d_n6;
        locals.var_local_sca_dn7 = assign16100_e23444_d_n7;
        locals.var_local_sca_dn8 = assign16100_e23444_d_n8;
        locals.var_local_sca_dn9 = assign16100_e23444_d_n9;
        locals.var_local_sca_dn10 = assign16100_e23444_d_n10;
        locals.var_local_sca_dn11 = assign16100_e23444_d_n11;
        locals.var_local_sca_dn12 = assign16100_e23444_d_n12;
        locals.var_local_sca_dn13 = assign16100_e23444_d_n13;
        locals.var_local_sca_dn14 = assign16100_e23444_d_n14;
        locals.var_local_sca_rv = 0.0;

        let (assign16110_e23484, assign16110_e23484_d_n0, assign16110_e23484_d_n2, assign16110_e23484_d_n3, assign16110_e23484_d_n4, assign16110_e23484_d_n5, assign16110_e23484_d_n6, assign16110_e23484_d_n7, assign16110_e23484_d_n8, assign16110_e23484_d_n9, assign16110_e23484_d_n10, assign16110_e23484_d_n11, assign16110_e23484_d_n12, assign16110_e23484_d_n13, assign16110_e23484_d_n14,) = {
    if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign16110_e23452: f64 = (0.1 * p.p23);
        let assign16110_e23455: f64 = (0.01 * p.p947);
        let assign16110_e23456: f64 = (assign16110_e23452 + assign16110_e23455);
        let assign16110_e23458: f64 = (-10.0);
        let assign16110_e23460: f64 = (assign16110_e23458 * p.p23);
        let assign16110_e23462: f64 = (assign16110_e23460 * locals.var_t2);
        let assign16110_e23463: f64 = { let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16110_e23464: f64 = (assign16110_e23456 * assign16110_e23463);
        let assign16110_e23467: f64 = (0.1 * locals.var_t1);
        let assign16110_e23470: f64 = (0.01 * p.p947);
        let assign16110_e23471: f64 = (assign16110_e23467 + assign16110_e23470);
        let assign16110_e23473: f64 = (-10.0);
        let assign16110_e23475: f64 = (assign16110_e23473 * locals.var_t1);
        let assign16110_e23477: f64 = (assign16110_e23475 * locals.var_t2);
        let assign16110_e23478: f64 = { let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16110_e23479: f64 = (assign16110_e23471 * assign16110_e23478);
        let assign16110_e23480: f64 = (assign16110_e23464 - assign16110_e23479);
        let assign16110_e23482: f64 = (assign16110_e23480 / locals.var_wdrn);
        (assign16110_e23482, (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn0))) - (((0.1 * locals.var_t1_dn0) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn0) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn0)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn2))) - (((0.1 * locals.var_t1_dn2) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn2) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn2)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn3))) - (((0.1 * locals.var_t1_dn3) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn3) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn3)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn4))) - (((0.1 * locals.var_t1_dn4) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn4) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn4)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn5))) - (((0.1 * locals.var_t1_dn5) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn5) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn5)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn6))) - (((0.1 * locals.var_t1_dn6) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn6) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn6)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn7))) - (((0.1 * locals.var_t1_dn7) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn7) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn7)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn8))) - (((0.1 * locals.var_t1_dn8) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn8) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn8)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn9))) - (((0.1 * locals.var_t1_dn9) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn9) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn9)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn10))) - (((0.1 * locals.var_t1_dn10) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn10) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn10)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn11))) - (((0.1 * locals.var_t1_dn11) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn11) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn11)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn12))) - (((0.1 * locals.var_t1_dn12) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn12) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn12)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn13))) - (((0.1 * locals.var_t1_dn13) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn13) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn13)))))) / locals.var_wdrn), (((assign16110_e23456 * ({ let limited_exp_arg = assign16110_e23462; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16110_e23460 * locals.var_t2_dn14))) - (((0.1 * locals.var_t1_dn14) * assign16110_e23478) + (assign16110_e23471 * ({ let limited_exp_arg = assign16110_e23477; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16110_e23473 * locals.var_t1_dn14) * locals.var_t2) + (assign16110_e23475 * locals.var_t2_dn14)))))) / locals.var_wdrn),)
    } else {
        (locals.var_local_scb, locals.var_local_scb_dn0, locals.var_local_scb_dn2, locals.var_local_scb_dn3, locals.var_local_scb_dn4, locals.var_local_scb_dn5, locals.var_local_scb_dn6, locals.var_local_scb_dn7, locals.var_local_scb_dn8, locals.var_local_scb_dn9, locals.var_local_scb_dn10, locals.var_local_scb_dn11, locals.var_local_scb_dn12, locals.var_local_scb_dn13, locals.var_local_scb_dn14,)
    }
};
        locals.var_local_scb = assign16110_e23484;
        locals.var_local_scb_dn0 = assign16110_e23484_d_n0;
        locals.var_local_scb_dn2 = assign16110_e23484_d_n2;
        locals.var_local_scb_dn3 = assign16110_e23484_d_n3;
        locals.var_local_scb_dn4 = assign16110_e23484_d_n4;
        locals.var_local_scb_dn5 = assign16110_e23484_d_n5;
        locals.var_local_scb_dn6 = assign16110_e23484_d_n6;
        locals.var_local_scb_dn7 = assign16110_e23484_d_n7;
        locals.var_local_scb_dn8 = assign16110_e23484_d_n8;
        locals.var_local_scb_dn9 = assign16110_e23484_d_n9;
        locals.var_local_scb_dn10 = assign16110_e23484_d_n10;
        locals.var_local_scb_dn11 = assign16110_e23484_d_n11;
        locals.var_local_scb_dn12 = assign16110_e23484_d_n12;
        locals.var_local_scb_dn13 = assign16110_e23484_d_n13;
        locals.var_local_scb_dn14 = assign16110_e23484_d_n14;
        locals.var_local_scb_rv = 0.0;

        let (assign16120_e23524, assign16120_e23524_d_n0, assign16120_e23524_d_n2, assign16120_e23524_d_n3, assign16120_e23524_d_n4, assign16120_e23524_d_n5, assign16120_e23524_d_n6, assign16120_e23524_d_n7, assign16120_e23524_d_n8, assign16120_e23524_d_n9, assign16120_e23524_d_n10, assign16120_e23524_d_n11, assign16120_e23524_d_n12, assign16120_e23524_d_n13, assign16120_e23524_d_n14,) = {
    if (((locals.var_guard490 != 0.0) && (locals.var_guard491 != 0.0)) && (locals.var_guard492 != 0.0)) {
        let assign16120_e23492: f64 = (0.05 * p.p23);
        let assign16120_e23495: f64 = (0.0025 * p.p947);
        let assign16120_e23496: f64 = (assign16120_e23492 + assign16120_e23495);
        let assign16120_e23498: f64 = (-20.0);
        let assign16120_e23500: f64 = (assign16120_e23498 * p.p23);
        let assign16120_e23502: f64 = (assign16120_e23500 * locals.var_t2);
        let assign16120_e23503: f64 = { let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16120_e23504: f64 = (assign16120_e23496 * assign16120_e23503);
        let assign16120_e23507: f64 = (0.05 * locals.var_t1);
        let assign16120_e23510: f64 = (0.0025 * p.p947);
        let assign16120_e23511: f64 = (assign16120_e23507 + assign16120_e23510);
        let assign16120_e23513: f64 = (-20.0);
        let assign16120_e23515: f64 = (assign16120_e23513 * locals.var_t1);
        let assign16120_e23517: f64 = (assign16120_e23515 * locals.var_t2);
        let assign16120_e23518: f64 = { let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign16120_e23519: f64 = (assign16120_e23511 * assign16120_e23518);
        let assign16120_e23520: f64 = (assign16120_e23504 - assign16120_e23519);
        let assign16120_e23522: f64 = (assign16120_e23520 / locals.var_wdrn);
        (assign16120_e23522, (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn0))) - (((0.05 * locals.var_t1_dn0) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn0) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn0)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn2))) - (((0.05 * locals.var_t1_dn2) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn2) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn2)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn3))) - (((0.05 * locals.var_t1_dn3) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn3) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn3)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn4))) - (((0.05 * locals.var_t1_dn4) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn4) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn4)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn5))) - (((0.05 * locals.var_t1_dn5) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn5) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn5)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn6))) - (((0.05 * locals.var_t1_dn6) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn6) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn6)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn7))) - (((0.05 * locals.var_t1_dn7) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn7) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn7)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn8))) - (((0.05 * locals.var_t1_dn8) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn8) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn8)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn9))) - (((0.05 * locals.var_t1_dn9) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn9) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn9)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn10))) - (((0.05 * locals.var_t1_dn10) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn10) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn10)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn11))) - (((0.05 * locals.var_t1_dn11) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn11) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn11)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn12))) - (((0.05 * locals.var_t1_dn12) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn12) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn12)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn13))) - (((0.05 * locals.var_t1_dn13) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn13) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn13)))))) / locals.var_wdrn), (((assign16120_e23496 * ({ let limited_exp_arg = assign16120_e23502; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign16120_e23500 * locals.var_t2_dn14))) - (((0.05 * locals.var_t1_dn14) * assign16120_e23518) + (assign16120_e23511 * ({ let limited_exp_arg = assign16120_e23517; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (((assign16120_e23513 * locals.var_t1_dn14) * locals.var_t2) + (assign16120_e23515 * locals.var_t2_dn14)))))) / locals.var_wdrn),)
    } else {
        (locals.var_local_scc, locals.var_local_scc_dn0, locals.var_local_scc_dn2, locals.var_local_scc_dn3, locals.var_local_scc_dn4, locals.var_local_scc_dn5, locals.var_local_scc_dn6, locals.var_local_scc_dn7, locals.var_local_scc_dn8, locals.var_local_scc_dn9, locals.var_local_scc_dn10, locals.var_local_scc_dn11, locals.var_local_scc_dn12, locals.var_local_scc_dn13, locals.var_local_scc_dn14,)
    }
};
        locals.var_local_scc = assign16120_e23524;
        locals.var_local_scc_dn0 = assign16120_e23524_d_n0;
        locals.var_local_scc_dn2 = assign16120_e23524_d_n2;
        locals.var_local_scc_dn3 = assign16120_e23524_d_n3;
        locals.var_local_scc_dn4 = assign16120_e23524_d_n4;
        locals.var_local_scc_dn5 = assign16120_e23524_d_n5;
        locals.var_local_scc_dn6 = assign16120_e23524_d_n6;
        locals.var_local_scc_dn7 = assign16120_e23524_d_n7;
        locals.var_local_scc_dn8 = assign16120_e23524_d_n8;
        locals.var_local_scc_dn9 = assign16120_e23524_d_n9;
        locals.var_local_scc_dn10 = assign16120_e23524_d_n10;
        locals.var_local_scc_dn11 = assign16120_e23524_d_n11;
        locals.var_local_scc_dn12 = assign16120_e23524_d_n12;
        locals.var_local_scc_dn13 = assign16120_e23524_d_n13;
        locals.var_local_scc_dn14 = assign16120_e23524_d_n14;
        locals.var_local_scc_rv = 0.0;

        let assign16130_e23529: f64 = (p.p933 * locals.var_local_scb);
        let assign16130_e23530: f64 = (locals.var_local_sca + assign16130_e23529);
        let assign16130_e23533: f64 = (p.p934 * locals.var_local_scc);
        let assign16130_e23534: f64 = (assign16130_e23530 + assign16130_e23533);
        let assign16130_e23535: f64 = (locals.var_kvth0we_i * assign16130_e23534);
        locals.var_vth0_well = assign16130_e23535;
        locals.var_vth0_well_dn0 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0)));
        locals.var_vth0_well_dn2 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2)));
        locals.var_vth0_well_dn3 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3)));
        locals.var_vth0_well_dn4 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4)));
        locals.var_vth0_well_dn5 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5)));
        locals.var_vth0_well_dn6 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6)));
        locals.var_vth0_well_dn7 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7)));
        locals.var_vth0_well_dn8 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8)));
        locals.var_vth0_well_dn9 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9)));
        locals.var_vth0_well_dn10 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10)));
        locals.var_vth0_well_dn11 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11)));
        locals.var_vth0_well_dn12 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12)));
        locals.var_vth0_well_dn13 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13)));
        locals.var_vth0_well_dn14 = (locals.var_kvth0we_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14)));
        locals.var_vth0_well_rv = 0.0;

        let assign16140_e23540: f64 = (p.p933 * locals.var_local_scb);
        let assign16140_e23541: f64 = (locals.var_local_sca + assign16140_e23540);
        let assign16140_e23544: f64 = (p.p934 * locals.var_local_scc);
        let assign16140_e23545: f64 = (assign16140_e23541 + assign16140_e23544);
        let assign16140_e23546: f64 = (locals.var_k2we_i * assign16140_e23545);
        locals.var_k2_well = assign16140_e23546;
        locals.var_k2_well_dn0 = (locals.var_k2we_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0)));
        locals.var_k2_well_dn2 = (locals.var_k2we_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2)));
        locals.var_k2_well_dn3 = (locals.var_k2we_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3)));
        locals.var_k2_well_dn4 = (locals.var_k2we_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4)));
        locals.var_k2_well_dn5 = (locals.var_k2we_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5)));
        locals.var_k2_well_dn6 = (locals.var_k2we_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6)));
        locals.var_k2_well_dn7 = (locals.var_k2we_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7)));
        locals.var_k2_well_dn8 = (locals.var_k2we_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8)));
        locals.var_k2_well_dn9 = (locals.var_k2we_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9)));
        locals.var_k2_well_dn10 = (locals.var_k2we_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10)));
        locals.var_k2_well_dn11 = (locals.var_k2we_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11)));
        locals.var_k2_well_dn12 = (locals.var_k2we_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12)));
        locals.var_k2_well_dn13 = (locals.var_k2we_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13)));
        locals.var_k2_well_dn14 = (locals.var_k2we_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14)));
        locals.var_k2_well_rv = 0.0;

        let assign16150_e23551: f64 = (p.p933 * locals.var_local_scb);
        let assign16150_e23552: f64 = (locals.var_local_sca + assign16150_e23551);
        let assign16150_e23555: f64 = (p.p934 * locals.var_local_scc);
        let assign16150_e23556: f64 = (assign16150_e23552 + assign16150_e23555);
        let assign16150_e23557: f64 = (locals.var_kvth0edgewe_i * assign16150_e23556);
        locals.var_vth0_well_edge = assign16150_e23557;
        locals.var_vth0_well_edge_dn0 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0)));
        locals.var_vth0_well_edge_dn2 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2)));
        locals.var_vth0_well_edge_dn3 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3)));
        locals.var_vth0_well_edge_dn4 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4)));
        locals.var_vth0_well_edge_dn5 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5)));
        locals.var_vth0_well_edge_dn6 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6)));
        locals.var_vth0_well_edge_dn7 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7)));
        locals.var_vth0_well_edge_dn8 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8)));
        locals.var_vth0_well_edge_dn9 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9)));
        locals.var_vth0_well_edge_dn10 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10)));
        locals.var_vth0_well_edge_dn11 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11)));
        locals.var_vth0_well_edge_dn12 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12)));
        locals.var_vth0_well_edge_dn13 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13)));
        locals.var_vth0_well_edge_dn14 = (locals.var_kvth0edgewe_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14)));
        locals.var_vth0_well_edge_rv = 0.0;

        let assign16160_e23562: f64 = (p.p933 * locals.var_local_scb);
        let assign16160_e23563: f64 = (locals.var_local_sca + assign16160_e23562);
        let assign16160_e23566: f64 = (p.p934 * locals.var_local_scc);
        let assign16160_e23567: f64 = (assign16160_e23563 + assign16160_e23566);
        let assign16160_e23568: f64 = (locals.var_k2edgewe_i * assign16160_e23567);
        locals.var_k2_well_edge = assign16160_e23568;
        locals.var_k2_well_edge_dn0 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0)));
        locals.var_k2_well_edge_dn2 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2)));
        locals.var_k2_well_edge_dn3 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3)));
        locals.var_k2_well_edge_dn4 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4)));
        locals.var_k2_well_edge_dn5 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5)));
        locals.var_k2_well_edge_dn6 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6)));
        locals.var_k2_well_edge_dn7 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7)));
        locals.var_k2_well_edge_dn8 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8)));
        locals.var_k2_well_edge_dn9 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9)));
        locals.var_k2_well_edge_dn10 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10)));
        locals.var_k2_well_edge_dn11 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11)));
        locals.var_k2_well_edge_dn12 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12)));
        locals.var_k2_well_edge_dn13 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13)));
        locals.var_k2_well_edge_dn14 = (locals.var_k2edgewe_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14)));
        locals.var_k2_well_edge_rv = 0.0;

        let assign16170_e23574: f64 = (p.p933 * locals.var_local_scb);
        let assign16170_e23575: f64 = (locals.var_local_sca + assign16170_e23574);
        let assign16170_e23578: f64 = (p.p934 * locals.var_local_scc);
        let assign16170_e23579: f64 = (assign16170_e23575 + assign16170_e23578);
        let assign16170_e23580: f64 = (locals.var_ku0we_i * assign16170_e23579);
        let assign16170_e23581: f64 = (1.0 + assign16170_e23580);
        locals.var_mu_well = assign16170_e23581;
        locals.var_mu_well_dn0 = (locals.var_ku0we_i * ((locals.var_local_sca_dn0 + (p.p933 * locals.var_local_scb_dn0)) + (p.p934 * locals.var_local_scc_dn0)));
        locals.var_mu_well_dn2 = (locals.var_ku0we_i * ((locals.var_local_sca_dn2 + (p.p933 * locals.var_local_scb_dn2)) + (p.p934 * locals.var_local_scc_dn2)));
        locals.var_mu_well_dn3 = (locals.var_ku0we_i * ((locals.var_local_sca_dn3 + (p.p933 * locals.var_local_scb_dn3)) + (p.p934 * locals.var_local_scc_dn3)));
        locals.var_mu_well_dn4 = (locals.var_ku0we_i * ((locals.var_local_sca_dn4 + (p.p933 * locals.var_local_scb_dn4)) + (p.p934 * locals.var_local_scc_dn4)));
        locals.var_mu_well_dn5 = (locals.var_ku0we_i * ((locals.var_local_sca_dn5 + (p.p933 * locals.var_local_scb_dn5)) + (p.p934 * locals.var_local_scc_dn5)));
        locals.var_mu_well_dn6 = (locals.var_ku0we_i * ((locals.var_local_sca_dn6 + (p.p933 * locals.var_local_scb_dn6)) + (p.p934 * locals.var_local_scc_dn6)));
        locals.var_mu_well_dn7 = (locals.var_ku0we_i * ((locals.var_local_sca_dn7 + (p.p933 * locals.var_local_scb_dn7)) + (p.p934 * locals.var_local_scc_dn7)));
        locals.var_mu_well_dn8 = (locals.var_ku0we_i * ((locals.var_local_sca_dn8 + (p.p933 * locals.var_local_scb_dn8)) + (p.p934 * locals.var_local_scc_dn8)));
        locals.var_mu_well_dn9 = (locals.var_ku0we_i * ((locals.var_local_sca_dn9 + (p.p933 * locals.var_local_scb_dn9)) + (p.p934 * locals.var_local_scc_dn9)));
        locals.var_mu_well_dn10 = (locals.var_ku0we_i * ((locals.var_local_sca_dn10 + (p.p933 * locals.var_local_scb_dn10)) + (p.p934 * locals.var_local_scc_dn10)));
        locals.var_mu_well_dn11 = (locals.var_ku0we_i * ((locals.var_local_sca_dn11 + (p.p933 * locals.var_local_scb_dn11)) + (p.p934 * locals.var_local_scc_dn11)));
        locals.var_mu_well_dn12 = (locals.var_ku0we_i * ((locals.var_local_sca_dn12 + (p.p933 * locals.var_local_scb_dn12)) + (p.p934 * locals.var_local_scc_dn12)));
        locals.var_mu_well_dn13 = (locals.var_ku0we_i * ((locals.var_local_sca_dn13 + (p.p933 * locals.var_local_scb_dn13)) + (p.p934 * locals.var_local_scc_dn13)));
        locals.var_mu_well_dn14 = (locals.var_ku0we_i * ((locals.var_local_sca_dn14 + (p.p933 * locals.var_local_scb_dn14)) + (p.p934 * locals.var_local_scc_dn14)));
        locals.var_mu_well_rv = 0.0;

        let assign16180_e23584: f64 = (locals.var_u0_t * locals.var_mu_well);
        locals.var_u0_t = assign16180_e23584;
        locals.var_u0_t_dn0 = ((locals.var_u0_t_dn0 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn0));
        locals.var_u0_t_dn2 = ((locals.var_u0_t_dn2 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn2));
        locals.var_u0_t_dn3 = ((locals.var_u0_t_dn3 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn3));
        locals.var_u0_t_dn4 = ((locals.var_u0_t_dn4 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn4));
        locals.var_u0_t_dn5 = ((locals.var_u0_t_dn5 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn5));
        locals.var_u0_t_dn6 = ((locals.var_u0_t_dn6 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn6));
        locals.var_u0_t_dn7 = ((locals.var_u0_t_dn7 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn7));
        locals.var_u0_t_dn8 = ((locals.var_u0_t_dn8 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn8));
        locals.var_u0_t_dn9 = ((locals.var_u0_t_dn9 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn9));
        locals.var_u0_t_dn10 = ((locals.var_u0_t_dn10 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn10));
        locals.var_u0_t_dn11 = ((locals.var_u0_t_dn11 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn11));
        locals.var_u0_t_dn12 = ((locals.var_u0_t_dn12 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn12));
        locals.var_u0_t_dn13 = ((locals.var_u0_t_dn13 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn13));
        locals.var_u0_t_dn14 = ((locals.var_u0_t_dn14 * locals.var_mu_well) + (locals.var_u0_t * locals.var_mu_well_dn14));
        locals.var_u0_t_rv = 0.0;

        let assign16190_e23587: f64 = (locals.var_k2_i + locals.var_k2_well);
        locals.var_k2_i = assign16190_e23587;
        locals.var_k2_i_dn0 = (locals.var_k2_i_dn0 + locals.var_k2_well_dn0);
        locals.var_k2_i_dn2 = (locals.var_k2_i_dn2 + locals.var_k2_well_dn2);
        locals.var_k2_i_dn3 = (locals.var_k2_i_dn3 + locals.var_k2_well_dn3);
        locals.var_k2_i_dn4 = (locals.var_k2_i_dn4 + locals.var_k2_well_dn4);
        locals.var_k2_i_dn5 = (locals.var_k2_i_dn5 + locals.var_k2_well_dn5);
        locals.var_k2_i_dn6 = (locals.var_k2_i_dn6 + locals.var_k2_well_dn6);
        locals.var_k2_i_dn7 = (locals.var_k2_i_dn7 + locals.var_k2_well_dn7);
        locals.var_k2_i_dn8 = (locals.var_k2_i_dn8 + locals.var_k2_well_dn8);
        locals.var_k2_i_dn9 = (locals.var_k2_i_dn9 + locals.var_k2_well_dn9);
        locals.var_k2_i_dn10 = (locals.var_k2_i_dn10 + locals.var_k2_well_dn10);
        locals.var_k2_i_dn11 = (locals.var_k2_i_dn11 + locals.var_k2_well_dn11);
        locals.var_k2_i_dn12 = (locals.var_k2_i_dn12 + locals.var_k2_well_dn12);
        locals.var_k2_i_dn13 = (locals.var_k2_i_dn13 + locals.var_k2_well_dn13);
        locals.var_k2_i_dn14 = (locals.var_k2_i_dn14 + locals.var_k2_well_dn14);
        locals.var_k2_i_rv = 0.0;

        let assign16200_e23590: f64 = (locals.var_devsign * (nv9 - nv11));
        locals.var_vg = assign16200_e23590;
        locals.var_vg_dn9 = locals.var_devsign;
        locals.var_vg_dn11 = (-locals.var_devsign);
        locals.var_vg_rv = 0.0;

        let assign16210_e23593: f64 = (locals.var_devsign * (nv5 - nv11));
        locals.var_vd = assign16210_e23593;
        locals.var_vd_dn5 = locals.var_devsign;
        locals.var_vd_dn7 = 0.0;
        locals.var_vd_dn11 = (-locals.var_devsign);
        locals.var_vd_rv = 0.0;

        let assign16220_e23596: f64 = (locals.var_devsign * (nv7 - nv11));
        locals.var_vs = assign16220_e23596;
        locals.var_vs_dn5 = 0.0;
        locals.var_vs_dn7 = locals.var_devsign;
        locals.var_vs_dn11 = (-locals.var_devsign);
        locals.var_vs_rv = 0.0;

        let assign16230_e23599: f64 = (locals.var_vd - locals.var_vs);
        locals.var_vds = assign16230_e23599;
        locals.var_vds_dn5 = (locals.var_vd_dn5 - locals.var_vs_dn5);
        locals.var_vds_dn7 = (locals.var_vd_dn7 - locals.var_vs_dn7);
        locals.var_vds_dn11 = (locals.var_vd_dn11 - locals.var_vs_dn11);
        locals.var_vds_rv = 0.0;

        locals.var_vdcv = locals.var_vd;
        locals.var_vdcv_dn5 = locals.var_vd_dn5;
        locals.var_vdcv_dn6 = 0.0;
        locals.var_vdcv_dn7 = locals.var_vd_dn7;
        locals.var_vdcv_dn11 = locals.var_vd_dn11;
        locals.var_vdcv_rv = 0.0;

        locals.var_vds_noswap = locals.var_vds;
        locals.var_vds_noswap_dn5 = locals.var_vds_dn5;
        locals.var_vds_noswap_dn7 = locals.var_vds_dn7;
        locals.var_vds_noswap_dn11 = locals.var_vds_dn11;
        locals.var_vds_noswap_rv = 0.0;

        locals.var_vsb_noswap = locals.var_vs;
        locals.var_vsb_noswap_dn5 = locals.var_vs_dn5;
        locals.var_vsb_noswap_dn7 = locals.var_vs_dn7;
        locals.var_vsb_noswap_dn11 = locals.var_vs_dn11;
        locals.var_vsb_noswap_rv = 0.0;

        locals.var_vdb_noswap = locals.var_vd;
        locals.var_vdb_noswap_dn5 = locals.var_vd_dn5;
        locals.var_vdb_noswap_dn7 = locals.var_vd_dn7;
        locals.var_vdb_noswap_dn11 = locals.var_vd_dn11;
        locals.var_vdb_noswap_rv = 0.0;

        let assign16280_e23606: f64 = (locals.var_devsign * (nv12 - nv7));
        locals.var_vbs_jct = assign16280_e23606;
        locals.var_vbs_jct_dn7 = (-locals.var_devsign);
        locals.var_vbs_jct_dn12 = locals.var_devsign;
        locals.var_vbs_jct_rv = 0.0;

        let assign16290_e23609: f64 = (locals.var_devsign * (nv13 - nv5));
        locals.var_vbd_jct = assign16290_e23609;
        locals.var_vbd_jct_dn5 = (-locals.var_devsign);
        locals.var_vbd_jct_dn13 = locals.var_devsign;
        locals.var_vbd_jct_rv = 0.0;

        let assign16300_e23612: f64 = (locals.var_devsign * (nv13 - nv5));
        locals.var_vbd_jctcv = assign16300_e23612;
        locals.var_vbd_jctcv_dn5 = (-locals.var_devsign);
        locals.var_vbd_jctcv_dn6 = 0.0;
        locals.var_vbd_jctcv_dn7 = 0.0;
        locals.var_vbd_jctcv_dn11 = 0.0;
        locals.var_vbd_jctcv_dn13 = locals.var_devsign;
        locals.var_vbd_jctcv_rv = 0.0;

        let assign16310_e23615: f64 = (locals.var_devsign * (nv13 - nv14));
        locals.var_vbd_ext = assign16310_e23615;
        locals.var_vbd_ext_dn13 = locals.var_devsign;
        locals.var_vbd_ext_dn14 = (-locals.var_devsign);
        locals.var_vbd_ext_rv = 0.0;

        let assign16320_e23618: f64 = (locals.var_vg - locals.var_vd);
        locals.var_vgd_noswap = assign16320_e23618;
        locals.var_vgd_noswap_dn5 = (-locals.var_vd_dn5);
        locals.var_vgd_noswap_dn7 = (-locals.var_vd_dn7);
        locals.var_vgd_noswap_dn9 = locals.var_vg_dn9;
        locals.var_vgd_noswap_dn11 = (locals.var_vg_dn11 - locals.var_vd_dn11);
        locals.var_vgd_noswap_rv = 0.0;

        let assign16330_e23621: f64 = (locals.var_vg - locals.var_vs);
        locals.var_vgs_noswap = assign16330_e23621;
        locals.var_vgs_noswap_dn5 = (-locals.var_vs_dn5);
        locals.var_vgs_noswap_dn7 = (-locals.var_vs_dn7);
        locals.var_vgs_noswap_dn9 = locals.var_vg_dn9;
        locals.var_vgs_noswap_dn11 = (locals.var_vg_dn11 - locals.var_vs_dn11);
        locals.var_vgs_noswap_rv = 0.0;

        let assign16340_e23624: f64 = (locals.var_devsign * (nv10 - nv5));
        locals.var_vgd_ov_noswap = assign16340_e23624;
        locals.var_vgd_ov_noswap_dn5 = (-locals.var_devsign);
        locals.var_vgd_ov_noswap_dn10 = locals.var_devsign;
        locals.var_vgd_ov_noswap_rv = 0.0;

        let assign16350_e23627: f64 = (locals.var_devsign * (nv10 - nv7));
        locals.var_vgs_ov_noswap = assign16350_e23627;
        locals.var_vgs_ov_noswap_dn7 = (-locals.var_devsign);
        locals.var_vgs_ov_noswap_dn10 = locals.var_devsign;
        locals.var_vgs_ov_noswap_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_37(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        locals.var_vgd_ov_noswapcv = locals.var_vgd_ov_noswap;
        locals.var_vgd_ov_noswapcv_dn5 = locals.var_vgd_ov_noswap_dn5;
        locals.var_vgd_ov_noswapcv_dn6 = 0.0;
        locals.var_vgd_ov_noswapcv_dn7 = 0.0;
        locals.var_vgd_ov_noswapcv_dn10 = locals.var_vgd_ov_noswap_dn10;
        locals.var_vgd_ov_noswapcv_dn11 = 0.0;
        locals.var_vgd_ov_noswapcv_rv = 0.0;

        let assign16370_e23643: f64 = if ((((p.p1110 != 0.0) && (p.p42 == 1.0)) && (p.p1095 == 1.0)) && (p.p1094 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard493 = assign16370_e23643;
        locals.var_guard493_rv = 0.0;

        let (assign16380_e23657, assign16380_e23657_d_n5, assign16380_e23657_d_n6, assign16380_e23657_d_n7, assign16380_e23657_d_n11,) = {
    if (locals.var_guard493 != 0.0) {
        let assign16380_e23650: f64 = (p.p1111 / p.p1110);
        let assign16380_e23651: f64 = (1.0 - assign16380_e23650);
        let assign16380_e23652: f64 = (locals.var_devsign * assign16380_e23651);
        let assign16380_e23654: f64 = (assign16380_e23652 * (nv6 - nv5));
        let assign16380_e23655: f64 = (locals.var_vd + assign16380_e23654);
        (assign16380_e23655, (locals.var_vd_dn5 + (-assign16380_e23652)), assign16380_e23652, locals.var_vd_dn7, locals.var_vd_dn11,)
    } else {
        (locals.var_vdcv, locals.var_vdcv_dn5, locals.var_vdcv_dn6, locals.var_vdcv_dn7, locals.var_vdcv_dn11,)
    }
};
        locals.var_vdcv = assign16380_e23657;
        locals.var_vdcv_dn5 = assign16380_e23657_d_n5;
        locals.var_vdcv_dn6 = assign16380_e23657_d_n6;
        locals.var_vdcv_dn7 = assign16380_e23657_d_n7;
        locals.var_vdcv_dn11 = assign16380_e23657_d_n11;
        locals.var_vdcv_rv = 0.0;

        let (assign16390_e23665, assign16390_e23665_d_n5, assign16390_e23665_d_n6, assign16390_e23665_d_n7, assign16390_e23665_d_n11, assign16390_e23665_d_n13,) = {
    if (locals.var_guard493 != 0.0) {
        let assign16390_e23661: f64 = (locals.var_vbd_jct + locals.var_vd);
        let assign16390_e23663: f64 = (assign16390_e23661 - locals.var_vdcv);
        (assign16390_e23663, ((locals.var_vbd_jct_dn5 + locals.var_vd_dn5) - locals.var_vdcv_dn5), (-locals.var_vdcv_dn6), (locals.var_vd_dn7 - locals.var_vdcv_dn7), (locals.var_vd_dn11 - locals.var_vdcv_dn11), locals.var_vbd_jct_dn13,)
    } else {
        (locals.var_vbd_jctcv, locals.var_vbd_jctcv_dn5, locals.var_vbd_jctcv_dn6, locals.var_vbd_jctcv_dn7, locals.var_vbd_jctcv_dn11, locals.var_vbd_jctcv_dn13,)
    }
};
        locals.var_vbd_jctcv = assign16390_e23665;
        locals.var_vbd_jctcv_dn5 = assign16390_e23665_d_n5;
        locals.var_vbd_jctcv_dn6 = assign16390_e23665_d_n6;
        locals.var_vbd_jctcv_dn7 = assign16390_e23665_d_n7;
        locals.var_vbd_jctcv_dn11 = assign16390_e23665_d_n11;
        locals.var_vbd_jctcv_dn13 = assign16390_e23665_d_n13;
        locals.var_vbd_jctcv_rv = 0.0;

        let (assign16400_e23673, assign16400_e23673_d_n5, assign16400_e23673_d_n6, assign16400_e23673_d_n7, assign16400_e23673_d_n10, assign16400_e23673_d_n11,) = {
    if (locals.var_guard493 != 0.0) {
        let assign16400_e23669: f64 = (locals.var_vgd_ov_noswap + locals.var_vd);
        let assign16400_e23671: f64 = (assign16400_e23669 - locals.var_vdcv);
        (assign16400_e23671, ((locals.var_vgd_ov_noswap_dn5 + locals.var_vd_dn5) - locals.var_vdcv_dn5), (-locals.var_vdcv_dn6), (locals.var_vd_dn7 - locals.var_vdcv_dn7), locals.var_vgd_ov_noswap_dn10, (locals.var_vd_dn11 - locals.var_vdcv_dn11),)
    } else {
        (locals.var_vgd_ov_noswapcv, locals.var_vgd_ov_noswapcv_dn5, locals.var_vgd_ov_noswapcv_dn6, locals.var_vgd_ov_noswapcv_dn7, locals.var_vgd_ov_noswapcv_dn10, locals.var_vgd_ov_noswapcv_dn11,)
    }
};
        locals.var_vgd_ov_noswapcv = assign16400_e23673;
        locals.var_vgd_ov_noswapcv_dn5 = assign16400_e23673_d_n5;
        locals.var_vgd_ov_noswapcv_dn6 = assign16400_e23673_d_n6;
        locals.var_vgd_ov_noswapcv_dn7 = assign16400_e23673_d_n7;
        locals.var_vgd_ov_noswapcv_dn10 = assign16400_e23673_d_n10;
        locals.var_vgd_ov_noswapcv_dn11 = assign16400_e23673_d_n11;
        locals.var_vgd_ov_noswapcv_rv = 0.0;

        locals.var_vdcv_noswap = locals.var_vdcv;
        locals.var_vdcv_noswap_dn5 = locals.var_vdcv_dn5;
        locals.var_vdcv_noswap_dn6 = locals.var_vdcv_dn6;
        locals.var_vdcv_noswap_dn7 = locals.var_vdcv_dn7;
        locals.var_vdcv_noswap_dn11 = locals.var_vdcv_dn11;
        locals.var_vdcv_noswap_rv = 0.0;

        let assign16420_e23677: f64 = (locals.var_devsign * (nv7 - nv11));
        locals.var_vscv = assign16420_e23677;
        locals.var_vscv_dn5 = 0.0;
        locals.var_vscv_dn6 = 0.0;
        locals.var_vscv_dn7 = locals.var_devsign;
        locals.var_vscv_dn11 = (-locals.var_devsign);
        locals.var_vscv_rv = 0.0;

        locals.var_sigvds = 1.0;
        locals.var_sigvds_rv = 0.0;

        let assign16440_e23681: f64 = if locals.var_vds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard494 = assign16440_e23681;
        locals.var_guard494_rv = 0.0;

        let (assign16450_e23686,) = {
    if (locals.var_guard494 != 0.0) {
        let assign16450_e23684: f64 = (-1.0);
        (assign16450_e23684,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign16450_e23686;
        locals.var_sigvds_rv = 0.0;

        let (assign16460_e23692, assign16460_e23692_d_n5, assign16460_e23692_d_n7, assign16460_e23692_d_n11,) = {
    if (locals.var_guard494 != 0.0) {
        let assign16460_e23690: f64 = (locals.var_devsign * (nv7 - nv11));
        (assign16460_e23690, 0.0, locals.var_devsign, (-locals.var_devsign),)
    } else {
        (locals.var_vd, locals.var_vd_dn5, locals.var_vd_dn7, locals.var_vd_dn11,)
    }
};
        locals.var_vd = assign16460_e23692;
        locals.var_vd_dn5 = assign16460_e23692_d_n5;
        locals.var_vd_dn7 = assign16460_e23692_d_n7;
        locals.var_vd_dn11 = assign16460_e23692_d_n11;
        locals.var_vd_rv = 0.0;

        let (assign16470_e23698, assign16470_e23698_d_n5, assign16470_e23698_d_n7, assign16470_e23698_d_n11,) = {
    if (locals.var_guard494 != 0.0) {
        let assign16470_e23696: f64 = (locals.var_devsign * (nv5 - nv11));
        (assign16470_e23696, locals.var_devsign, 0.0, (-locals.var_devsign),)
    } else {
        (locals.var_vs, locals.var_vs_dn5, locals.var_vs_dn7, locals.var_vs_dn11,)
    }
};
        locals.var_vs = assign16470_e23698;
        locals.var_vs_dn5 = assign16470_e23698_d_n5;
        locals.var_vs_dn7 = assign16470_e23698_d_n7;
        locals.var_vs_dn11 = assign16470_e23698_d_n11;
        locals.var_vs_rv = 0.0;

        let (assign16480_e23702, assign16480_e23702_d_n5, assign16480_e23702_d_n6, assign16480_e23702_d_n7, assign16480_e23702_d_n11,) = {
    if (locals.var_guard494 != 0.0) {
        (locals.var_vdcv_noswap, locals.var_vdcv_noswap_dn5, locals.var_vdcv_noswap_dn6, locals.var_vdcv_noswap_dn7, locals.var_vdcv_noswap_dn11,)
    } else {
        (locals.var_vscv, locals.var_vscv_dn5, locals.var_vscv_dn6, locals.var_vscv_dn7, locals.var_vscv_dn11,)
    }
};
        locals.var_vscv = assign16480_e23702;
        locals.var_vscv_dn5 = assign16480_e23702_d_n5;
        locals.var_vscv_dn6 = assign16480_e23702_d_n6;
        locals.var_vscv_dn7 = assign16480_e23702_d_n7;
        locals.var_vscv_dn11 = assign16480_e23702_d_n11;
        locals.var_vscv_rv = 0.0;

        let (assign16490_e23708, assign16490_e23708_d_n5, assign16490_e23708_d_n6, assign16490_e23708_d_n7, assign16490_e23708_d_n11,) = {
    if (locals.var_guard494 != 0.0) {
        let assign16490_e23706: f64 = (locals.var_devsign * (nv7 - nv11));
        (assign16490_e23706, 0.0, 0.0, locals.var_devsign, (-locals.var_devsign),)
    } else {
        (locals.var_vdcv, locals.var_vdcv_dn5, locals.var_vdcv_dn6, locals.var_vdcv_dn7, locals.var_vdcv_dn11,)
    }
};
        locals.var_vdcv = assign16490_e23708;
        locals.var_vdcv_dn5 = assign16490_e23708_d_n5;
        locals.var_vdcv_dn6 = assign16490_e23708_d_n6;
        locals.var_vdcv_dn7 = assign16490_e23708_d_n7;
        locals.var_vdcv_dn11 = assign16490_e23708_d_n11;
        locals.var_vdcv_rv = 0.0;

        let assign16500_e23711: f64 = (locals.var_vd - locals.var_vs);
        locals.var_vds = assign16500_e23711;
        locals.var_vds_dn5 = (locals.var_vd_dn5 - locals.var_vs_dn5);
        locals.var_vds_dn7 = (locals.var_vd_dn7 - locals.var_vs_dn7);
        locals.var_vds_dn11 = (locals.var_vd_dn11 - locals.var_vs_dn11);
        locals.var_vds_rv = 0.0;

        let assign16510_e23714: f64 = (locals.var_vdcv - locals.var_vscv);
        locals.var_vdscv = assign16510_e23714;
        locals.var_vdscv_dn5 = (locals.var_vdcv_dn5 - locals.var_vscv_dn5);
        locals.var_vdscv_dn6 = (locals.var_vdcv_dn6 - locals.var_vscv_dn6);
        locals.var_vdscv_dn7 = (locals.var_vdcv_dn7 - locals.var_vscv_dn7);
        locals.var_vdscv_dn11 = (locals.var_vdcv_dn11 - locals.var_vscv_dn11);
        locals.var_vdscv_rv = 0.0;

        let assign16520_e23717: f64 = (p.p956 * locals.var_vdscv);
        locals.var_t0 = assign16520_e23717;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = (p.p956 * locals.var_vdscv_dn5);
        locals.var_t0_dn6 = (p.p956 * locals.var_vdscv_dn6);
        locals.var_t0_dn7 = (p.p956 * locals.var_vdscv_dn7);
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = (p.p956 * locals.var_vdscv_dn11);
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign16530_e23724: f64 = (-37.0);
        let (assign16530_e23751, assign16530_e23751_d_n0, assign16530_e23751_d_n2, assign16530_e23751_d_n3, assign16530_e23751_d_n4, assign16530_e23751_d_n5, assign16530_e23751_d_n6, assign16530_e23751_d_n7, assign16530_e23751_d_n8, assign16530_e23751_d_n9, assign16530_e23751_d_n10, assign16530_e23751_d_n11, assign16530_e23751_d_n12, assign16530_e23751_d_n13, assign16530_e23751_d_n14,) = {
    if ((!(locals.var_t0 > 37.0)) && (!(locals.var_t0 < assign16530_e23724))) {
        let assign16530_e23730: f64 = (locals.var_t0).exp();
        let assign16530_e23731: f64 = (1.0 + assign16530_e23730);
        let assign16530_e23732: f64 = (assign16530_e23731).ln();
        (assign16530_e23732, ((assign16530_e23730 * locals.var_t0_dn0) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn2) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn3) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn4) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn5) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn6) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn7) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn8) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn9) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn10) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn11) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn12) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn13) / assign16530_e23731), ((assign16530_e23730 * locals.var_t0_dn14) / assign16530_e23731),)
    } else {
        let assign16530_e23739: f64 = (-37.0);
        let (assign16530_e23750, assign16530_e23750_d_n0, assign16530_e23750_d_n2, assign16530_e23750_d_n3, assign16530_e23750_d_n4, assign16530_e23750_d_n5, assign16530_e23750_d_n6, assign16530_e23750_d_n7, assign16530_e23750_d_n8, assign16530_e23750_d_n9, assign16530_e23750_d_n10, assign16530_e23750_d_n11, assign16530_e23750_d_n12, assign16530_e23750_d_n13, assign16530_e23750_d_n14,) = {
            if ((!(locals.var_t0 > 37.0)) && (locals.var_t0 < assign16530_e23739)) {
                let assign16530_e23743: f64 = (locals.var_t0).exp();
                (assign16530_e23743, (assign16530_e23743 * locals.var_t0_dn0), (assign16530_e23743 * locals.var_t0_dn2), (assign16530_e23743 * locals.var_t0_dn3), (assign16530_e23743 * locals.var_t0_dn4), (assign16530_e23743 * locals.var_t0_dn5), (assign16530_e23743 * locals.var_t0_dn6), (assign16530_e23743 * locals.var_t0_dn7), (assign16530_e23743 * locals.var_t0_dn8), (assign16530_e23743 * locals.var_t0_dn9), (assign16530_e23743 * locals.var_t0_dn10), (assign16530_e23743 * locals.var_t0_dn11), (assign16530_e23743 * locals.var_t0_dn12), (assign16530_e23743 * locals.var_t0_dn13), (assign16530_e23743 * locals.var_t0_dn14),)
            } else {
                let (assign16530_e23749, assign16530_e23749_d_n0, assign16530_e23749_d_n2, assign16530_e23749_d_n3, assign16530_e23749_d_n4, assign16530_e23749_d_n5, assign16530_e23749_d_n6, assign16530_e23749_d_n7, assign16530_e23749_d_n8, assign16530_e23749_d_n9, assign16530_e23749_d_n10, assign16530_e23749_d_n11, assign16530_e23749_d_n12, assign16530_e23749_d_n13, assign16530_e23749_d_n14,) = {
                    if (locals.var_t0 > 37.0) {
                        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign16530_e23749, assign16530_e23749_d_n0, assign16530_e23749_d_n2, assign16530_e23749_d_n3, assign16530_e23749_d_n4, assign16530_e23749_d_n5, assign16530_e23749_d_n6, assign16530_e23749_d_n7, assign16530_e23749_d_n8, assign16530_e23749_d_n9, assign16530_e23749_d_n10, assign16530_e23749_d_n11, assign16530_e23749_d_n12, assign16530_e23749_d_n13, assign16530_e23749_d_n14,)
            }
        };
        (assign16530_e23750, assign16530_e23750_d_n0, assign16530_e23750_d_n2, assign16530_e23750_d_n3, assign16530_e23750_d_n4, assign16530_e23750_d_n5, assign16530_e23750_d_n6, assign16530_e23750_d_n7, assign16530_e23750_d_n8, assign16530_e23750_d_n9, assign16530_e23750_d_n10, assign16530_e23750_d_n11, assign16530_e23750_d_n12, assign16530_e23750_d_n13, assign16530_e23750_d_n14,)
    }
};
        locals.var_t1 = assign16530_e23751;
        locals.var_t1_dn0 = assign16530_e23751_d_n0;
        locals.var_t1_dn2 = assign16530_e23751_d_n2;
        locals.var_t1_dn3 = assign16530_e23751_d_n3;
        locals.var_t1_dn4 = assign16530_e23751_d_n4;
        locals.var_t1_dn5 = assign16530_e23751_d_n5;
        locals.var_t1_dn6 = assign16530_e23751_d_n6;
        locals.var_t1_dn7 = assign16530_e23751_d_n7;
        locals.var_t1_dn8 = assign16530_e23751_d_n8;
        locals.var_t1_dn9 = assign16530_e23751_d_n9;
        locals.var_t1_dn10 = assign16530_e23751_d_n10;
        locals.var_t1_dn11 = assign16530_e23751_d_n11;
        locals.var_t1_dn12 = assign16530_e23751_d_n12;
        locals.var_t1_dn13 = assign16530_e23751_d_n13;
        locals.var_t1_dn14 = assign16530_e23751_d_n14;
        locals.var_t1_rv = 0.0;

        let assign16540_e23754: f64 = (2.0 / p.p956);
        let assign16540_e23756: f64 = (assign16540_e23754 * locals.var_t1);
        let assign16540_e23758: f64 = (assign16540_e23756 - locals.var_vdscv);
        let assign16540_e23761: f64 = (2.0 / p.p956);
        let assign16540_e23763: f64 = (2.0_f64).ln();
        let assign16540_e23764: f64 = (assign16540_e23761 * assign16540_e23763);
        let assign16540_e23765: f64 = (assign16540_e23758 - assign16540_e23764);
        locals.var_vdsx = assign16540_e23765;
        locals.var_vdsx_dn0 = (assign16540_e23754 * locals.var_t1_dn0);
        locals.var_vdsx_dn2 = (assign16540_e23754 * locals.var_t1_dn2);
        locals.var_vdsx_dn3 = (assign16540_e23754 * locals.var_t1_dn3);
        locals.var_vdsx_dn4 = (assign16540_e23754 * locals.var_t1_dn4);
        locals.var_vdsx_dn5 = ((assign16540_e23754 * locals.var_t1_dn5) - locals.var_vdscv_dn5);
        locals.var_vdsx_dn6 = ((assign16540_e23754 * locals.var_t1_dn6) - locals.var_vdscv_dn6);
        locals.var_vdsx_dn7 = ((assign16540_e23754 * locals.var_t1_dn7) - locals.var_vdscv_dn7);
        locals.var_vdsx_dn8 = (assign16540_e23754 * locals.var_t1_dn8);
        locals.var_vdsx_dn9 = (assign16540_e23754 * locals.var_t1_dn9);
        locals.var_vdsx_dn10 = (assign16540_e23754 * locals.var_t1_dn10);
        locals.var_vdsx_dn11 = ((assign16540_e23754 * locals.var_t1_dn11) - locals.var_vdscv_dn11);
        locals.var_vdsx_dn12 = (assign16540_e23754 * locals.var_t1_dn12);
        locals.var_vdsx_dn13 = (assign16540_e23754 * locals.var_t1_dn13);
        locals.var_vdsx_dn14 = (assign16540_e23754 * locals.var_t1_dn14);
        locals.var_vdsx_rv = 0.0;

        let assign16550_e23770: f64 = (locals.var_vdscv - locals.var_vdsx);
        let assign16550_e23771: f64 = (0.5 * assign16550_e23770);
        let assign16550_e23772: f64 = (locals.var_vscv + assign16550_e23771);
        let assign16550_e23773: f64 = (-assign16550_e23772);
        locals.var_vbsxcv = assign16550_e23773;
        locals.var_vbsxcv_dn0 = (-(0.5 * (-locals.var_vdsx_dn0)));
        locals.var_vbsxcv_dn2 = (-(0.5 * (-locals.var_vdsx_dn2)));
        locals.var_vbsxcv_dn3 = (-(0.5 * (-locals.var_vdsx_dn3)));
        locals.var_vbsxcv_dn4 = (-(0.5 * (-locals.var_vdsx_dn4)));
        locals.var_vbsxcv_dn5 = (-(locals.var_vscv_dn5 + (0.5 * (locals.var_vdscv_dn5 - locals.var_vdsx_dn5))));
        locals.var_vbsxcv_dn6 = (-(locals.var_vscv_dn6 + (0.5 * (locals.var_vdscv_dn6 - locals.var_vdsx_dn6))));
        locals.var_vbsxcv_dn7 = (-(locals.var_vscv_dn7 + (0.5 * (locals.var_vdscv_dn7 - locals.var_vdsx_dn7))));
        locals.var_vbsxcv_dn8 = (-(0.5 * (-locals.var_vdsx_dn8)));
        locals.var_vbsxcv_dn9 = (-(0.5 * (-locals.var_vdsx_dn9)));
        locals.var_vbsxcv_dn10 = (-(0.5 * (-locals.var_vdsx_dn10)));
        locals.var_vbsxcv_dn11 = (-(locals.var_vscv_dn11 + (0.5 * (locals.var_vdscv_dn11 - locals.var_vdsx_dn11))));
        locals.var_vbsxcv_dn12 = (-(0.5 * (-locals.var_vdsx_dn12)));
        locals.var_vbsxcv_dn13 = (-(0.5 * (-locals.var_vdsx_dn13)));
        locals.var_vbsxcv_dn14 = (-(0.5 * (-locals.var_vdsx_dn14)));
        locals.var_vbsxcv_rv = 0.0;

        let assign16560_e23776: f64 = (p.p956 * locals.var_vds);
        locals.var_t0 = assign16560_e23776;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = (p.p956 * locals.var_vds_dn5);
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = (p.p956 * locals.var_vds_dn7);
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = (p.p956 * locals.var_vds_dn11);
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign16570_e23783: f64 = (-37.0);
        let (assign16570_e23810, assign16570_e23810_d_n0, assign16570_e23810_d_n2, assign16570_e23810_d_n3, assign16570_e23810_d_n4, assign16570_e23810_d_n5, assign16570_e23810_d_n6, assign16570_e23810_d_n7, assign16570_e23810_d_n8, assign16570_e23810_d_n9, assign16570_e23810_d_n10, assign16570_e23810_d_n11, assign16570_e23810_d_n12, assign16570_e23810_d_n13, assign16570_e23810_d_n14,) = {
    if ((!(locals.var_t0 > 37.0)) && (!(locals.var_t0 < assign16570_e23783))) {
        let assign16570_e23789: f64 = (locals.var_t0).exp();
        let assign16570_e23790: f64 = (1.0 + assign16570_e23789);
        let assign16570_e23791: f64 = (assign16570_e23790).ln();
        (assign16570_e23791, ((assign16570_e23789 * locals.var_t0_dn0) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn2) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn3) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn4) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn5) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn6) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn7) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn8) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn9) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn10) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn11) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn12) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn13) / assign16570_e23790), ((assign16570_e23789 * locals.var_t0_dn14) / assign16570_e23790),)
    } else {
        let assign16570_e23798: f64 = (-37.0);
        let (assign16570_e23809, assign16570_e23809_d_n0, assign16570_e23809_d_n2, assign16570_e23809_d_n3, assign16570_e23809_d_n4, assign16570_e23809_d_n5, assign16570_e23809_d_n6, assign16570_e23809_d_n7, assign16570_e23809_d_n8, assign16570_e23809_d_n9, assign16570_e23809_d_n10, assign16570_e23809_d_n11, assign16570_e23809_d_n12, assign16570_e23809_d_n13, assign16570_e23809_d_n14,) = {
            if ((!(locals.var_t0 > 37.0)) && (locals.var_t0 < assign16570_e23798)) {
                let assign16570_e23802: f64 = (locals.var_t0).exp();
                (assign16570_e23802, (assign16570_e23802 * locals.var_t0_dn0), (assign16570_e23802 * locals.var_t0_dn2), (assign16570_e23802 * locals.var_t0_dn3), (assign16570_e23802 * locals.var_t0_dn4), (assign16570_e23802 * locals.var_t0_dn5), (assign16570_e23802 * locals.var_t0_dn6), (assign16570_e23802 * locals.var_t0_dn7), (assign16570_e23802 * locals.var_t0_dn8), (assign16570_e23802 * locals.var_t0_dn9), (assign16570_e23802 * locals.var_t0_dn10), (assign16570_e23802 * locals.var_t0_dn11), (assign16570_e23802 * locals.var_t0_dn12), (assign16570_e23802 * locals.var_t0_dn13), (assign16570_e23802 * locals.var_t0_dn14),)
            } else {
                let (assign16570_e23808, assign16570_e23808_d_n0, assign16570_e23808_d_n2, assign16570_e23808_d_n3, assign16570_e23808_d_n4, assign16570_e23808_d_n5, assign16570_e23808_d_n6, assign16570_e23808_d_n7, assign16570_e23808_d_n8, assign16570_e23808_d_n9, assign16570_e23808_d_n10, assign16570_e23808_d_n11, assign16570_e23808_d_n12, assign16570_e23808_d_n13, assign16570_e23808_d_n14,) = {
                    if (locals.var_t0 > 37.0) {
                        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign16570_e23808, assign16570_e23808_d_n0, assign16570_e23808_d_n2, assign16570_e23808_d_n3, assign16570_e23808_d_n4, assign16570_e23808_d_n5, assign16570_e23808_d_n6, assign16570_e23808_d_n7, assign16570_e23808_d_n8, assign16570_e23808_d_n9, assign16570_e23808_d_n10, assign16570_e23808_d_n11, assign16570_e23808_d_n12, assign16570_e23808_d_n13, assign16570_e23808_d_n14,)
            }
        };
        (assign16570_e23809, assign16570_e23809_d_n0, assign16570_e23809_d_n2, assign16570_e23809_d_n3, assign16570_e23809_d_n4, assign16570_e23809_d_n5, assign16570_e23809_d_n6, assign16570_e23809_d_n7, assign16570_e23809_d_n8, assign16570_e23809_d_n9, assign16570_e23809_d_n10, assign16570_e23809_d_n11, assign16570_e23809_d_n12, assign16570_e23809_d_n13, assign16570_e23809_d_n14,)
    }
};
        locals.var_t1 = assign16570_e23810;
        locals.var_t1_dn0 = assign16570_e23810_d_n0;
        locals.var_t1_dn2 = assign16570_e23810_d_n2;
        locals.var_t1_dn3 = assign16570_e23810_d_n3;
        locals.var_t1_dn4 = assign16570_e23810_d_n4;
        locals.var_t1_dn5 = assign16570_e23810_d_n5;
        locals.var_t1_dn6 = assign16570_e23810_d_n6;
        locals.var_t1_dn7 = assign16570_e23810_d_n7;
        locals.var_t1_dn8 = assign16570_e23810_d_n8;
        locals.var_t1_dn9 = assign16570_e23810_d_n9;
        locals.var_t1_dn10 = assign16570_e23810_d_n10;
        locals.var_t1_dn11 = assign16570_e23810_d_n11;
        locals.var_t1_dn12 = assign16570_e23810_d_n12;
        locals.var_t1_dn13 = assign16570_e23810_d_n13;
        locals.var_t1_dn14 = assign16570_e23810_d_n14;
        locals.var_t1_rv = 0.0;

        let assign16580_e23813: f64 = (2.0 / p.p956);
        let assign16580_e23815: f64 = (assign16580_e23813 * locals.var_t1);
        let assign16580_e23817: f64 = (assign16580_e23815 - locals.var_vds);
        let assign16580_e23820: f64 = (2.0 / p.p956);
        let assign16580_e23822: f64 = (2.0_f64).ln();
        let assign16580_e23823: f64 = (assign16580_e23820 * assign16580_e23822);
        let assign16580_e23824: f64 = (assign16580_e23817 - assign16580_e23823);
        locals.var_vdsx = assign16580_e23824;
        locals.var_vdsx_dn0 = (assign16580_e23813 * locals.var_t1_dn0);
        locals.var_vdsx_dn2 = (assign16580_e23813 * locals.var_t1_dn2);
        locals.var_vdsx_dn3 = (assign16580_e23813 * locals.var_t1_dn3);
        locals.var_vdsx_dn4 = (assign16580_e23813 * locals.var_t1_dn4);
        locals.var_vdsx_dn5 = ((assign16580_e23813 * locals.var_t1_dn5) - locals.var_vds_dn5);
        locals.var_vdsx_dn6 = (assign16580_e23813 * locals.var_t1_dn6);
        locals.var_vdsx_dn7 = ((assign16580_e23813 * locals.var_t1_dn7) - locals.var_vds_dn7);
        locals.var_vdsx_dn8 = (assign16580_e23813 * locals.var_t1_dn8);
        locals.var_vdsx_dn9 = (assign16580_e23813 * locals.var_t1_dn9);
        locals.var_vdsx_dn10 = (assign16580_e23813 * locals.var_t1_dn10);
        locals.var_vdsx_dn11 = ((assign16580_e23813 * locals.var_t1_dn11) - locals.var_vds_dn11);
        locals.var_vdsx_dn12 = (assign16580_e23813 * locals.var_t1_dn12);
        locals.var_vdsx_dn13 = (assign16580_e23813 * locals.var_t1_dn13);
        locals.var_vdsx_dn14 = (assign16580_e23813 * locals.var_t1_dn14);
        locals.var_vdsx_rv = 0.0;

        let assign16590_e23829: f64 = (locals.var_vds - locals.var_vdsx);
        let assign16590_e23830: f64 = (0.5 * assign16590_e23829);
        let assign16590_e23831: f64 = (locals.var_vs + assign16590_e23830);
        let assign16590_e23832: f64 = (-assign16590_e23831);
        locals.var_vbsx = assign16590_e23832;
        locals.var_vbsx_dn0 = (-(0.5 * (-locals.var_vdsx_dn0)));
        locals.var_vbsx_dn2 = (-(0.5 * (-locals.var_vdsx_dn2)));
        locals.var_vbsx_dn3 = (-(0.5 * (-locals.var_vdsx_dn3)));
        locals.var_vbsx_dn4 = (-(0.5 * (-locals.var_vdsx_dn4)));
        locals.var_vbsx_dn5 = (-(locals.var_vs_dn5 + (0.5 * (locals.var_vds_dn5 - locals.var_vdsx_dn5))));
        locals.var_vbsx_dn6 = (-(0.5 * (-locals.var_vdsx_dn6)));
        locals.var_vbsx_dn7 = (-(locals.var_vs_dn7 + (0.5 * (locals.var_vds_dn7 - locals.var_vdsx_dn7))));
        locals.var_vbsx_dn8 = (-(0.5 * (-locals.var_vdsx_dn8)));
        locals.var_vbsx_dn9 = (-(0.5 * (-locals.var_vdsx_dn9)));
        locals.var_vbsx_dn10 = (-(0.5 * (-locals.var_vdsx_dn10)));
        locals.var_vbsx_dn11 = (-(locals.var_vs_dn11 + (0.5 * (locals.var_vds_dn11 - locals.var_vdsx_dn11))));
        locals.var_vbsx_dn12 = (-(0.5 * (-locals.var_vdsx_dn12)));
        locals.var_vbsx_dn13 = (-(0.5 * (-locals.var_vdsx_dn13)));
        locals.var_vbsx_dn14 = (-(0.5 * (-locals.var_vdsx_dn14)));
        locals.var_vbsx_rv = 0.0;

        let assign16600_e23835: f64 = (p.p1123 * locals.var_vds_noswap);
        let assign16600_e23837: f64 = (assign16600_e23835 / locals.var_vtm);
        let assign16600_e23838: f64 = (assign16600_e23837).tanh();
        locals.var_t0 = assign16600_e23838;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = ((-((assign16600_e23835 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) / ((assign16600_e23837).cosh() * (assign16600_e23837).cosh()));
        locals.var_t0_dn5 = (((p.p1123 * locals.var_vds_noswap_dn5) / locals.var_vtm) / ((assign16600_e23837).cosh() * (assign16600_e23837).cosh()));
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = (((p.p1123 * locals.var_vds_noswap_dn7) / locals.var_vtm) / ((assign16600_e23837).cosh() * (assign16600_e23837).cosh()));
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = (((p.p1123 * locals.var_vds_noswap_dn11) / locals.var_vtm) / ((assign16600_e23837).cosh() * (assign16600_e23837).cosh()));
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign16610_e23842: f64 = (0.5 * locals.var_t0);
        let assign16610_e23843: f64 = (0.5 + assign16610_e23842);
        locals.var_wf = assign16610_e23843;
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
        locals.var_wf_dn12 = (0.5 * locals.var_t0_dn12);
        locals.var_wf_dn13 = (0.5 * locals.var_t0_dn13);
        locals.var_wf_dn14 = (0.5 * locals.var_t0_dn14);
        locals.var_wf_rv = 0.0;

        let assign16620_e23846: f64 = (1.0 - locals.var_wf);
        locals.var_wr = assign16620_e23846;
        locals.var_wr_dn0 = (-locals.var_wf_dn0);
        locals.var_wr_dn2 = (-locals.var_wf_dn2);
        locals.var_wr_dn3 = (-locals.var_wf_dn3);
        locals.var_wr_dn4 = (-locals.var_wf_dn4);
        locals.var_wr_dn5 = (-locals.var_wf_dn5);
        locals.var_wr_dn6 = (-locals.var_wf_dn6);
        locals.var_wr_dn7 = (-locals.var_wf_dn7);
        locals.var_wr_dn8 = (-locals.var_wf_dn8);
        locals.var_wr_dn9 = (-locals.var_wf_dn9);
        locals.var_wr_dn10 = (-locals.var_wf_dn10);
        locals.var_wr_dn11 = (-locals.var_wf_dn11);
        locals.var_wr_dn12 = (-locals.var_wf_dn12);
        locals.var_wr_dn13 = (-locals.var_wf_dn13);
        locals.var_wr_dn14 = (-locals.var_wf_dn14);
        locals.var_wr_rv = 0.0;

        let assign16630_e23849: f64 = if p.p44 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard495 = assign16630_e23849;
        locals.var_guard495_rv = 0.0;

        let (assign16640_e23859, assign16640_e23859_d_n0, assign16640_e23859_d_n2, assign16640_e23859_d_n3, assign16640_e23859_d_n4, assign16640_e23859_d_n5, assign16640_e23859_d_n6, assign16640_e23859_d_n7, assign16640_e23859_d_n8, assign16640_e23859_d_n9, assign16640_e23859_d_n10, assign16640_e23859_d_n11, assign16640_e23859_d_n12, assign16640_e23859_d_n13, assign16640_e23859_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16640_e23853: f64 = (locals.var_cdscdr_i * locals.var_wr);
        let assign16640_e23856: f64 = (locals.var_cdscd_i * locals.var_wf);
        let assign16640_e23857: f64 = (assign16640_e23853 + assign16640_e23856);
        (assign16640_e23857, (((locals.var_cdscdr_i_dn0 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn0)) + ((locals.var_cdscd_i_dn0 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn0))), (((locals.var_cdscdr_i_dn2 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn2)) + ((locals.var_cdscd_i_dn2 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn2))), (((locals.var_cdscdr_i_dn3 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn3)) + ((locals.var_cdscd_i_dn3 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn3))), (((locals.var_cdscdr_i_dn4 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn4)) + ((locals.var_cdscd_i_dn4 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn4))), (((locals.var_cdscdr_i_dn5 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn5)) + ((locals.var_cdscd_i_dn5 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn5))), (((locals.var_cdscdr_i_dn6 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn6)) + ((locals.var_cdscd_i_dn6 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn6))), (((locals.var_cdscdr_i_dn7 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn7)) + ((locals.var_cdscd_i_dn7 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn7))), (((locals.var_cdscdr_i_dn8 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn8)) + ((locals.var_cdscd_i_dn8 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn8))), (((locals.var_cdscdr_i_dn9 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn9)) + ((locals.var_cdscd_i_dn9 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn9))), (((locals.var_cdscdr_i_dn10 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn10)) + ((locals.var_cdscd_i_dn10 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn10))), (((locals.var_cdscdr_i_dn11 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn11)) + ((locals.var_cdscd_i_dn11 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn11))), (((locals.var_cdscdr_i_dn12 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn12)) + ((locals.var_cdscd_i_dn12 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn12))), (((locals.var_cdscdr_i_dn13 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn13)) + ((locals.var_cdscd_i_dn13 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn13))), (((locals.var_cdscdr_i_dn14 * locals.var_wr) + (locals.var_cdscdr_i * locals.var_wr_dn14)) + ((locals.var_cdscd_i_dn14 * locals.var_wf) + (locals.var_cdscd_i * locals.var_wf_dn14))),)
    } else {
        (locals.var_cdscd_a, locals.var_cdscd_a_dn0, locals.var_cdscd_a_dn2, locals.var_cdscd_a_dn3, locals.var_cdscd_a_dn4, locals.var_cdscd_a_dn5, locals.var_cdscd_a_dn6, locals.var_cdscd_a_dn7, locals.var_cdscd_a_dn8, locals.var_cdscd_a_dn9, locals.var_cdscd_a_dn10, locals.var_cdscd_a_dn11, locals.var_cdscd_a_dn12, locals.var_cdscd_a_dn13, locals.var_cdscd_a_dn14,)
    }
};
        locals.var_cdscd_a = assign16640_e23859;
        locals.var_cdscd_a_dn0 = assign16640_e23859_d_n0;
        locals.var_cdscd_a_dn2 = assign16640_e23859_d_n2;
        locals.var_cdscd_a_dn3 = assign16640_e23859_d_n3;
        locals.var_cdscd_a_dn4 = assign16640_e23859_d_n4;
        locals.var_cdscd_a_dn5 = assign16640_e23859_d_n5;
        locals.var_cdscd_a_dn6 = assign16640_e23859_d_n6;
        locals.var_cdscd_a_dn7 = assign16640_e23859_d_n7;
        locals.var_cdscd_a_dn8 = assign16640_e23859_d_n8;
        locals.var_cdscd_a_dn9 = assign16640_e23859_d_n9;
        locals.var_cdscd_a_dn10 = assign16640_e23859_d_n10;
        locals.var_cdscd_a_dn11 = assign16640_e23859_d_n11;
        locals.var_cdscd_a_dn12 = assign16640_e23859_d_n12;
        locals.var_cdscd_a_dn13 = assign16640_e23859_d_n13;
        locals.var_cdscd_a_dn14 = assign16640_e23859_d_n14;
        locals.var_cdscd_a_rv = 0.0;

        let (assign16650_e23869, assign16650_e23869_d_n0, assign16650_e23869_d_n2, assign16650_e23869_d_n3, assign16650_e23869_d_n4, assign16650_e23869_d_n5, assign16650_e23869_d_n6, assign16650_e23869_d_n7, assign16650_e23869_d_n8, assign16650_e23869_d_n9, assign16650_e23869_d_n10, assign16650_e23869_d_n11, assign16650_e23869_d_n12, assign16650_e23869_d_n13, assign16650_e23869_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16650_e23863: f64 = (locals.var_eta0r_t * locals.var_wr);
        let assign16650_e23866: f64 = (locals.var_eta0_t * locals.var_wf);
        let assign16650_e23867: f64 = (assign16650_e23863 + assign16650_e23866);
        (assign16650_e23867, (((locals.var_eta0r_t_dn0 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn0)) + ((locals.var_eta0_t_dn0 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn0))), (((locals.var_eta0r_t_dn2 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn2)) + ((locals.var_eta0_t_dn2 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn2))), (((locals.var_eta0r_t_dn3 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn3)) + ((locals.var_eta0_t_dn3 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn3))), (((locals.var_eta0r_t_dn4 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn4)) + ((locals.var_eta0_t_dn4 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn4))), (((locals.var_eta0r_t_dn5 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn5)) + ((locals.var_eta0_t_dn5 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn5))), (((locals.var_eta0r_t_dn6 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn6)) + ((locals.var_eta0_t_dn6 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn6))), (((locals.var_eta0r_t_dn7 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn7)) + ((locals.var_eta0_t_dn7 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn7))), (((locals.var_eta0r_t_dn8 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn8)) + ((locals.var_eta0_t_dn8 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn8))), (((locals.var_eta0r_t_dn9 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn9)) + ((locals.var_eta0_t_dn9 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn9))), (((locals.var_eta0r_t_dn10 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn10)) + ((locals.var_eta0_t_dn10 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn10))), (((locals.var_eta0r_t_dn11 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn11)) + ((locals.var_eta0_t_dn11 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn11))), (((locals.var_eta0r_t_dn12 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn12)) + ((locals.var_eta0_t_dn12 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn12))), (((locals.var_eta0r_t_dn13 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn13)) + ((locals.var_eta0_t_dn13 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn13))), (((locals.var_eta0r_t_dn14 * locals.var_wr) + (locals.var_eta0r_t * locals.var_wr_dn14)) + ((locals.var_eta0_t_dn14 * locals.var_wf) + (locals.var_eta0_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_eta0_a, locals.var_eta0_a_dn0, locals.var_eta0_a_dn2, locals.var_eta0_a_dn3, locals.var_eta0_a_dn4, locals.var_eta0_a_dn5, locals.var_eta0_a_dn6, locals.var_eta0_a_dn7, locals.var_eta0_a_dn8, locals.var_eta0_a_dn9, locals.var_eta0_a_dn10, locals.var_eta0_a_dn11, locals.var_eta0_a_dn12, locals.var_eta0_a_dn13, locals.var_eta0_a_dn14,)
    }
};
        locals.var_eta0_a = assign16650_e23869;
        locals.var_eta0_a_dn0 = assign16650_e23869_d_n0;
        locals.var_eta0_a_dn2 = assign16650_e23869_d_n2;
        locals.var_eta0_a_dn3 = assign16650_e23869_d_n3;
        locals.var_eta0_a_dn4 = assign16650_e23869_d_n4;
        locals.var_eta0_a_dn5 = assign16650_e23869_d_n5;
        locals.var_eta0_a_dn6 = assign16650_e23869_d_n6;
        locals.var_eta0_a_dn7 = assign16650_e23869_d_n7;
        locals.var_eta0_a_dn8 = assign16650_e23869_d_n8;
        locals.var_eta0_a_dn9 = assign16650_e23869_d_n9;
        locals.var_eta0_a_dn10 = assign16650_e23869_d_n10;
        locals.var_eta0_a_dn11 = assign16650_e23869_d_n11;
        locals.var_eta0_a_dn12 = assign16650_e23869_d_n12;
        locals.var_eta0_a_dn13 = assign16650_e23869_d_n13;
        locals.var_eta0_a_dn14 = assign16650_e23869_d_n14;
        locals.var_eta0_a_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_38(
        locals: &mut StampLocals,
    ) {
        let (assign16660_e23879, assign16660_e23879_d_n0, assign16660_e23879_d_n2, assign16660_e23879_d_n3, assign16660_e23879_d_n4, assign16660_e23879_d_n5, assign16660_e23879_d_n6, assign16660_e23879_d_n7, assign16660_e23879_d_n8, assign16660_e23879_d_n9, assign16660_e23879_d_n10, assign16660_e23879_d_n11, assign16660_e23879_d_n12, assign16660_e23879_d_n13, assign16660_e23879_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16660_e23873: f64 = (locals.var_pdiblcr_i * locals.var_wr);
        let assign16660_e23876: f64 = (locals.var_pdiblc_i * locals.var_wf);
        let assign16660_e23877: f64 = (assign16660_e23873 + assign16660_e23876);
        (assign16660_e23877, (((locals.var_pdiblcr_i_dn0 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn0)) + ((locals.var_pdiblc_i_dn0 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn0))), (((locals.var_pdiblcr_i_dn2 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn2)) + ((locals.var_pdiblc_i_dn2 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn2))), (((locals.var_pdiblcr_i_dn3 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn3)) + ((locals.var_pdiblc_i_dn3 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn3))), (((locals.var_pdiblcr_i_dn4 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn4)) + ((locals.var_pdiblc_i_dn4 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn4))), (((locals.var_pdiblcr_i_dn5 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn5)) + ((locals.var_pdiblc_i_dn5 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn5))), (((locals.var_pdiblcr_i_dn6 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn6)) + ((locals.var_pdiblc_i_dn6 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn6))), (((locals.var_pdiblcr_i_dn7 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn7)) + ((locals.var_pdiblc_i_dn7 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn7))), (((locals.var_pdiblcr_i_dn8 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn8)) + ((locals.var_pdiblc_i_dn8 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn8))), (((locals.var_pdiblcr_i_dn9 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn9)) + ((locals.var_pdiblc_i_dn9 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn9))), (((locals.var_pdiblcr_i_dn10 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn10)) + ((locals.var_pdiblc_i_dn10 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn10))), (((locals.var_pdiblcr_i_dn11 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn11)) + ((locals.var_pdiblc_i_dn11 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn11))), (((locals.var_pdiblcr_i_dn12 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn12)) + ((locals.var_pdiblc_i_dn12 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn12))), (((locals.var_pdiblcr_i_dn13 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn13)) + ((locals.var_pdiblc_i_dn13 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn13))), (((locals.var_pdiblcr_i_dn14 * locals.var_wr) + (locals.var_pdiblcr_i * locals.var_wr_dn14)) + ((locals.var_pdiblc_i_dn14 * locals.var_wf) + (locals.var_pdiblc_i * locals.var_wf_dn14))),)
    } else {
        (locals.var_pdiblc_a, locals.var_pdiblc_a_dn0, locals.var_pdiblc_a_dn2, locals.var_pdiblc_a_dn3, locals.var_pdiblc_a_dn4, locals.var_pdiblc_a_dn5, locals.var_pdiblc_a_dn6, locals.var_pdiblc_a_dn7, locals.var_pdiblc_a_dn8, locals.var_pdiblc_a_dn9, locals.var_pdiblc_a_dn10, locals.var_pdiblc_a_dn11, locals.var_pdiblc_a_dn12, locals.var_pdiblc_a_dn13, locals.var_pdiblc_a_dn14,)
    }
};
        locals.var_pdiblc_a = assign16660_e23879;
        locals.var_pdiblc_a_dn0 = assign16660_e23879_d_n0;
        locals.var_pdiblc_a_dn2 = assign16660_e23879_d_n2;
        locals.var_pdiblc_a_dn3 = assign16660_e23879_d_n3;
        locals.var_pdiblc_a_dn4 = assign16660_e23879_d_n4;
        locals.var_pdiblc_a_dn5 = assign16660_e23879_d_n5;
        locals.var_pdiblc_a_dn6 = assign16660_e23879_d_n6;
        locals.var_pdiblc_a_dn7 = assign16660_e23879_d_n7;
        locals.var_pdiblc_a_dn8 = assign16660_e23879_d_n8;
        locals.var_pdiblc_a_dn9 = assign16660_e23879_d_n9;
        locals.var_pdiblc_a_dn10 = assign16660_e23879_d_n10;
        locals.var_pdiblc_a_dn11 = assign16660_e23879_d_n11;
        locals.var_pdiblc_a_dn12 = assign16660_e23879_d_n12;
        locals.var_pdiblc_a_dn13 = assign16660_e23879_d_n13;
        locals.var_pdiblc_a_dn14 = assign16660_e23879_d_n14;
        locals.var_pdiblc_a_rv = 0.0;

        let (assign16670_e23889, assign16670_e23889_d_n0, assign16670_e23889_d_n2, assign16670_e23889_d_n3, assign16670_e23889_d_n4, assign16670_e23889_d_n5, assign16670_e23889_d_n6, assign16670_e23889_d_n7, assign16670_e23889_d_n8, assign16670_e23889_d_n9, assign16670_e23889_d_n10, assign16670_e23889_d_n11, assign16670_e23889_d_n12, assign16670_e23889_d_n13, assign16670_e23889_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16670_e23883: f64 = (locals.var_pclmr_i * locals.var_wr);
        let assign16670_e23886: f64 = (locals.var_pclm_i * locals.var_wf);
        let assign16670_e23887: f64 = (assign16670_e23883 + assign16670_e23886);
        (assign16670_e23887, (((locals.var_pclmr_i_dn0 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn0)) + ((locals.var_pclm_i_dn0 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn0))), (((locals.var_pclmr_i_dn2 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn2)) + ((locals.var_pclm_i_dn2 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn2))), (((locals.var_pclmr_i_dn3 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn3)) + ((locals.var_pclm_i_dn3 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn3))), (((locals.var_pclmr_i_dn4 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn4)) + ((locals.var_pclm_i_dn4 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn4))), (((locals.var_pclmr_i_dn5 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn5)) + ((locals.var_pclm_i_dn5 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn5))), (((locals.var_pclmr_i_dn6 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn6)) + ((locals.var_pclm_i_dn6 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn6))), (((locals.var_pclmr_i_dn7 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn7)) + ((locals.var_pclm_i_dn7 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn7))), (((locals.var_pclmr_i_dn8 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn8)) + ((locals.var_pclm_i_dn8 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn8))), (((locals.var_pclmr_i_dn9 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn9)) + ((locals.var_pclm_i_dn9 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn9))), (((locals.var_pclmr_i_dn10 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn10)) + ((locals.var_pclm_i_dn10 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn10))), (((locals.var_pclmr_i_dn11 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn11)) + ((locals.var_pclm_i_dn11 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn11))), (((locals.var_pclmr_i_dn12 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn12)) + ((locals.var_pclm_i_dn12 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn12))), (((locals.var_pclmr_i_dn13 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn13)) + ((locals.var_pclm_i_dn13 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn13))), (((locals.var_pclmr_i_dn14 * locals.var_wr) + (locals.var_pclmr_i * locals.var_wr_dn14)) + ((locals.var_pclm_i_dn14 * locals.var_wf) + (locals.var_pclm_i * locals.var_wf_dn14))),)
    } else {
        (locals.var_pclm_a, locals.var_pclm_a_dn0, locals.var_pclm_a_dn2, locals.var_pclm_a_dn3, locals.var_pclm_a_dn4, locals.var_pclm_a_dn5, locals.var_pclm_a_dn6, locals.var_pclm_a_dn7, locals.var_pclm_a_dn8, locals.var_pclm_a_dn9, locals.var_pclm_a_dn10, locals.var_pclm_a_dn11, locals.var_pclm_a_dn12, locals.var_pclm_a_dn13, locals.var_pclm_a_dn14,)
    }
};
        locals.var_pclm_a = assign16670_e23889;
        locals.var_pclm_a_dn0 = assign16670_e23889_d_n0;
        locals.var_pclm_a_dn2 = assign16670_e23889_d_n2;
        locals.var_pclm_a_dn3 = assign16670_e23889_d_n3;
        locals.var_pclm_a_dn4 = assign16670_e23889_d_n4;
        locals.var_pclm_a_dn5 = assign16670_e23889_d_n5;
        locals.var_pclm_a_dn6 = assign16670_e23889_d_n6;
        locals.var_pclm_a_dn7 = assign16670_e23889_d_n7;
        locals.var_pclm_a_dn8 = assign16670_e23889_d_n8;
        locals.var_pclm_a_dn9 = assign16670_e23889_d_n9;
        locals.var_pclm_a_dn10 = assign16670_e23889_d_n10;
        locals.var_pclm_a_dn11 = assign16670_e23889_d_n11;
        locals.var_pclm_a_dn12 = assign16670_e23889_d_n12;
        locals.var_pclm_a_dn13 = assign16670_e23889_d_n13;
        locals.var_pclm_a_dn14 = assign16670_e23889_d_n14;
        locals.var_pclm_a_rv = 0.0;

        let (assign16680_e23899, assign16680_e23899_d_n0, assign16680_e23899_d_n2, assign16680_e23899_d_n3, assign16680_e23899_d_n4, assign16680_e23899_d_n5, assign16680_e23899_d_n6, assign16680_e23899_d_n7, assign16680_e23899_d_n8, assign16680_e23899_d_n9, assign16680_e23899_d_n10, assign16680_e23899_d_n11, assign16680_e23899_d_n12, assign16680_e23899_d_n13, assign16680_e23899_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16680_e23893: f64 = (locals.var_psatr_i * locals.var_wr);
        let assign16680_e23896: f64 = (locals.var_psat_i * locals.var_wf);
        let assign16680_e23897: f64 = (assign16680_e23893 + assign16680_e23896);
        (assign16680_e23897, ((locals.var_psatr_i * locals.var_wr_dn0) + (locals.var_psat_i * locals.var_wf_dn0)), ((locals.var_psatr_i * locals.var_wr_dn2) + (locals.var_psat_i * locals.var_wf_dn2)), ((locals.var_psatr_i * locals.var_wr_dn3) + (locals.var_psat_i * locals.var_wf_dn3)), ((locals.var_psatr_i * locals.var_wr_dn4) + (locals.var_psat_i * locals.var_wf_dn4)), ((locals.var_psatr_i * locals.var_wr_dn5) + (locals.var_psat_i * locals.var_wf_dn5)), ((locals.var_psatr_i * locals.var_wr_dn6) + (locals.var_psat_i * locals.var_wf_dn6)), ((locals.var_psatr_i * locals.var_wr_dn7) + (locals.var_psat_i * locals.var_wf_dn7)), ((locals.var_psatr_i * locals.var_wr_dn8) + (locals.var_psat_i * locals.var_wf_dn8)), ((locals.var_psatr_i * locals.var_wr_dn9) + (locals.var_psat_i * locals.var_wf_dn9)), ((locals.var_psatr_i * locals.var_wr_dn10) + (locals.var_psat_i * locals.var_wf_dn10)), ((locals.var_psatr_i * locals.var_wr_dn11) + (locals.var_psat_i * locals.var_wf_dn11)), ((locals.var_psatr_i * locals.var_wr_dn12) + (locals.var_psat_i * locals.var_wf_dn12)), ((locals.var_psatr_i * locals.var_wr_dn13) + (locals.var_psat_i * locals.var_wf_dn13)), ((locals.var_psatr_i * locals.var_wr_dn14) + (locals.var_psat_i * locals.var_wf_dn14)),)
    } else {
        (locals.var_psat_a, locals.var_psat_a_dn0, locals.var_psat_a_dn2, locals.var_psat_a_dn3, locals.var_psat_a_dn4, locals.var_psat_a_dn5, locals.var_psat_a_dn6, locals.var_psat_a_dn7, locals.var_psat_a_dn8, locals.var_psat_a_dn9, locals.var_psat_a_dn10, locals.var_psat_a_dn11, locals.var_psat_a_dn12, locals.var_psat_a_dn13, locals.var_psat_a_dn14,)
    }
};
        locals.var_psat_a = assign16680_e23899;
        locals.var_psat_a_dn0 = assign16680_e23899_d_n0;
        locals.var_psat_a_dn2 = assign16680_e23899_d_n2;
        locals.var_psat_a_dn3 = assign16680_e23899_d_n3;
        locals.var_psat_a_dn4 = assign16680_e23899_d_n4;
        locals.var_psat_a_dn5 = assign16680_e23899_d_n5;
        locals.var_psat_a_dn6 = assign16680_e23899_d_n6;
        locals.var_psat_a_dn7 = assign16680_e23899_d_n7;
        locals.var_psat_a_dn8 = assign16680_e23899_d_n8;
        locals.var_psat_a_dn9 = assign16680_e23899_d_n9;
        locals.var_psat_a_dn10 = assign16680_e23899_d_n10;
        locals.var_psat_a_dn11 = assign16680_e23899_d_n11;
        locals.var_psat_a_dn12 = assign16680_e23899_d_n12;
        locals.var_psat_a_dn13 = assign16680_e23899_d_n13;
        locals.var_psat_a_dn14 = assign16680_e23899_d_n14;
        locals.var_psat_a_rv = 0.0;

        let (assign16690_e23909, assign16690_e23909_d_n0, assign16690_e23909_d_n2, assign16690_e23909_d_n3, assign16690_e23909_d_n4, assign16690_e23909_d_n5, assign16690_e23909_d_n6, assign16690_e23909_d_n7, assign16690_e23909_d_n8, assign16690_e23909_d_n9, assign16690_e23909_d_n10, assign16690_e23909_d_n11, assign16690_e23909_d_n12, assign16690_e23909_d_n13, assign16690_e23909_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16690_e23903: f64 = (locals.var_vsatr_t * locals.var_wr);
        let assign16690_e23906: f64 = (locals.var_vsat_t * locals.var_wf);
        let assign16690_e23907: f64 = (assign16690_e23903 + assign16690_e23906);
        (assign16690_e23907, (((locals.var_vsatr_t_dn0 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn0)) + ((locals.var_vsat_t_dn0 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn0))), (((locals.var_vsatr_t_dn2 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn2)) + ((locals.var_vsat_t_dn2 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn2))), (((locals.var_vsatr_t_dn3 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn3)) + ((locals.var_vsat_t_dn3 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn3))), (((locals.var_vsatr_t_dn4 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn4)) + ((locals.var_vsat_t_dn4 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn4))), (((locals.var_vsatr_t_dn5 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn5)) + ((locals.var_vsat_t_dn5 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn5))), (((locals.var_vsatr_t_dn6 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn6)) + ((locals.var_vsat_t_dn6 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn6))), (((locals.var_vsatr_t_dn7 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn7)) + ((locals.var_vsat_t_dn7 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn7))), (((locals.var_vsatr_t_dn8 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn8)) + ((locals.var_vsat_t_dn8 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn8))), (((locals.var_vsatr_t_dn9 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn9)) + ((locals.var_vsat_t_dn9 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn9))), (((locals.var_vsatr_t_dn10 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn10)) + ((locals.var_vsat_t_dn10 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn10))), (((locals.var_vsatr_t_dn11 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn11)) + ((locals.var_vsat_t_dn11 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn11))), (((locals.var_vsatr_t_dn12 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn12)) + ((locals.var_vsat_t_dn12 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn12))), (((locals.var_vsatr_t_dn13 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn13)) + ((locals.var_vsat_t_dn13 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn13))), (((locals.var_vsatr_t_dn14 * locals.var_wr) + (locals.var_vsatr_t * locals.var_wr_dn14)) + ((locals.var_vsat_t_dn14 * locals.var_wf) + (locals.var_vsat_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_vsat_a, locals.var_vsat_a_dn0, locals.var_vsat_a_dn2, locals.var_vsat_a_dn3, locals.var_vsat_a_dn4, locals.var_vsat_a_dn5, locals.var_vsat_a_dn6, locals.var_vsat_a_dn7, locals.var_vsat_a_dn8, locals.var_vsat_a_dn9, locals.var_vsat_a_dn10, locals.var_vsat_a_dn11, locals.var_vsat_a_dn12, locals.var_vsat_a_dn13, locals.var_vsat_a_dn14,)
    }
};
        locals.var_vsat_a = assign16690_e23909;
        locals.var_vsat_a_dn0 = assign16690_e23909_d_n0;
        locals.var_vsat_a_dn2 = assign16690_e23909_d_n2;
        locals.var_vsat_a_dn3 = assign16690_e23909_d_n3;
        locals.var_vsat_a_dn4 = assign16690_e23909_d_n4;
        locals.var_vsat_a_dn5 = assign16690_e23909_d_n5;
        locals.var_vsat_a_dn6 = assign16690_e23909_d_n6;
        locals.var_vsat_a_dn7 = assign16690_e23909_d_n7;
        locals.var_vsat_a_dn8 = assign16690_e23909_d_n8;
        locals.var_vsat_a_dn9 = assign16690_e23909_d_n9;
        locals.var_vsat_a_dn10 = assign16690_e23909_d_n10;
        locals.var_vsat_a_dn11 = assign16690_e23909_d_n11;
        locals.var_vsat_a_dn12 = assign16690_e23909_d_n12;
        locals.var_vsat_a_dn13 = assign16690_e23909_d_n13;
        locals.var_vsat_a_dn14 = assign16690_e23909_d_n14;
        locals.var_vsat_a_rv = 0.0;

        let (assign16700_e23919, assign16700_e23919_d_n0, assign16700_e23919_d_n2, assign16700_e23919_d_n3, assign16700_e23919_d_n4, assign16700_e23919_d_n5, assign16700_e23919_d_n6, assign16700_e23919_d_n7, assign16700_e23919_d_n8, assign16700_e23919_d_n9, assign16700_e23919_d_n10, assign16700_e23919_d_n11, assign16700_e23919_d_n12, assign16700_e23919_d_n13, assign16700_e23919_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16700_e23913: f64 = (locals.var_ptwgr_t * locals.var_wr);
        let assign16700_e23916: f64 = (locals.var_ptwg_t * locals.var_wf);
        let assign16700_e23917: f64 = (assign16700_e23913 + assign16700_e23916);
        (assign16700_e23917, (((locals.var_ptwgr_t_dn0 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn0)) + ((locals.var_ptwg_t_dn0 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn0))), (((locals.var_ptwgr_t_dn2 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn2)) + ((locals.var_ptwg_t_dn2 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn2))), (((locals.var_ptwgr_t_dn3 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn3)) + ((locals.var_ptwg_t_dn3 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn3))), (((locals.var_ptwgr_t_dn4 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn4)) + ((locals.var_ptwg_t_dn4 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn4))), (((locals.var_ptwgr_t_dn5 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn5)) + ((locals.var_ptwg_t_dn5 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn5))), (((locals.var_ptwgr_t_dn6 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn6)) + ((locals.var_ptwg_t_dn6 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn6))), (((locals.var_ptwgr_t_dn7 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn7)) + ((locals.var_ptwg_t_dn7 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn7))), (((locals.var_ptwgr_t_dn8 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn8)) + ((locals.var_ptwg_t_dn8 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn8))), (((locals.var_ptwgr_t_dn9 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn9)) + ((locals.var_ptwg_t_dn9 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn9))), (((locals.var_ptwgr_t_dn10 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn10)) + ((locals.var_ptwg_t_dn10 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn10))), (((locals.var_ptwgr_t_dn11 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn11)) + ((locals.var_ptwg_t_dn11 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn11))), (((locals.var_ptwgr_t_dn12 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn12)) + ((locals.var_ptwg_t_dn12 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn12))), (((locals.var_ptwgr_t_dn13 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn13)) + ((locals.var_ptwg_t_dn13 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn13))), (((locals.var_ptwgr_t_dn14 * locals.var_wr) + (locals.var_ptwgr_t * locals.var_wr_dn14)) + ((locals.var_ptwg_t_dn14 * locals.var_wf) + (locals.var_ptwg_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_ptwg_a, locals.var_ptwg_a_dn0, locals.var_ptwg_a_dn2, locals.var_ptwg_a_dn3, locals.var_ptwg_a_dn4, locals.var_ptwg_a_dn5, locals.var_ptwg_a_dn6, locals.var_ptwg_a_dn7, locals.var_ptwg_a_dn8, locals.var_ptwg_a_dn9, locals.var_ptwg_a_dn10, locals.var_ptwg_a_dn11, locals.var_ptwg_a_dn12, locals.var_ptwg_a_dn13, locals.var_ptwg_a_dn14,)
    }
};
        locals.var_ptwg_a = assign16700_e23919;
        locals.var_ptwg_a_dn0 = assign16700_e23919_d_n0;
        locals.var_ptwg_a_dn2 = assign16700_e23919_d_n2;
        locals.var_ptwg_a_dn3 = assign16700_e23919_d_n3;
        locals.var_ptwg_a_dn4 = assign16700_e23919_d_n4;
        locals.var_ptwg_a_dn5 = assign16700_e23919_d_n5;
        locals.var_ptwg_a_dn6 = assign16700_e23919_d_n6;
        locals.var_ptwg_a_dn7 = assign16700_e23919_d_n7;
        locals.var_ptwg_a_dn8 = assign16700_e23919_d_n8;
        locals.var_ptwg_a_dn9 = assign16700_e23919_d_n9;
        locals.var_ptwg_a_dn10 = assign16700_e23919_d_n10;
        locals.var_ptwg_a_dn11 = assign16700_e23919_d_n11;
        locals.var_ptwg_a_dn12 = assign16700_e23919_d_n12;
        locals.var_ptwg_a_dn13 = assign16700_e23919_d_n13;
        locals.var_ptwg_a_dn14 = assign16700_e23919_d_n14;
        locals.var_ptwg_a_rv = 0.0;

        let (assign16710_e23929, assign16710_e23929_d_n0, assign16710_e23929_d_n2, assign16710_e23929_d_n3, assign16710_e23929_d_n4, assign16710_e23929_d_n5, assign16710_e23929_d_n6, assign16710_e23929_d_n7, assign16710_e23929_d_n8, assign16710_e23929_d_n9, assign16710_e23929_d_n10, assign16710_e23929_d_n11, assign16710_e23929_d_n12, assign16710_e23929_d_n13, assign16710_e23929_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16710_e23923: f64 = (locals.var_u0r_t * locals.var_wr);
        let assign16710_e23926: f64 = (locals.var_u0_t * locals.var_wf);
        let assign16710_e23927: f64 = (assign16710_e23923 + assign16710_e23926);
        (assign16710_e23927, ((locals.var_u0r_t * locals.var_wr_dn0) + ((locals.var_u0_t_dn0 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn0))), ((locals.var_u0r_t * locals.var_wr_dn2) + ((locals.var_u0_t_dn2 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn2))), ((locals.var_u0r_t * locals.var_wr_dn3) + ((locals.var_u0_t_dn3 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn3))), (((locals.var_u0r_t_dn4 * locals.var_wr) + (locals.var_u0r_t * locals.var_wr_dn4)) + ((locals.var_u0_t_dn4 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn4))), ((locals.var_u0r_t * locals.var_wr_dn5) + ((locals.var_u0_t_dn5 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn5))), ((locals.var_u0r_t * locals.var_wr_dn6) + ((locals.var_u0_t_dn6 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn6))), ((locals.var_u0r_t * locals.var_wr_dn7) + ((locals.var_u0_t_dn7 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn7))), ((locals.var_u0r_t * locals.var_wr_dn8) + ((locals.var_u0_t_dn8 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn8))), ((locals.var_u0r_t * locals.var_wr_dn9) + ((locals.var_u0_t_dn9 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn9))), ((locals.var_u0r_t * locals.var_wr_dn10) + ((locals.var_u0_t_dn10 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn10))), ((locals.var_u0r_t * locals.var_wr_dn11) + ((locals.var_u0_t_dn11 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn11))), ((locals.var_u0r_t * locals.var_wr_dn12) + ((locals.var_u0_t_dn12 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn12))), ((locals.var_u0r_t * locals.var_wr_dn13) + ((locals.var_u0_t_dn13 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn13))), ((locals.var_u0r_t * locals.var_wr_dn14) + ((locals.var_u0_t_dn14 * locals.var_wf) + (locals.var_u0_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_u0_a, locals.var_u0_a_dn0, locals.var_u0_a_dn2, locals.var_u0_a_dn3, locals.var_u0_a_dn4, locals.var_u0_a_dn5, locals.var_u0_a_dn6, locals.var_u0_a_dn7, locals.var_u0_a_dn8, locals.var_u0_a_dn9, locals.var_u0_a_dn10, locals.var_u0_a_dn11, locals.var_u0_a_dn12, locals.var_u0_a_dn13, locals.var_u0_a_dn14,)
    }
};
        locals.var_u0_a = assign16710_e23929;
        locals.var_u0_a_dn0 = assign16710_e23929_d_n0;
        locals.var_u0_a_dn2 = assign16710_e23929_d_n2;
        locals.var_u0_a_dn3 = assign16710_e23929_d_n3;
        locals.var_u0_a_dn4 = assign16710_e23929_d_n4;
        locals.var_u0_a_dn5 = assign16710_e23929_d_n5;
        locals.var_u0_a_dn6 = assign16710_e23929_d_n6;
        locals.var_u0_a_dn7 = assign16710_e23929_d_n7;
        locals.var_u0_a_dn8 = assign16710_e23929_d_n8;
        locals.var_u0_a_dn9 = assign16710_e23929_d_n9;
        locals.var_u0_a_dn10 = assign16710_e23929_d_n10;
        locals.var_u0_a_dn11 = assign16710_e23929_d_n11;
        locals.var_u0_a_dn12 = assign16710_e23929_d_n12;
        locals.var_u0_a_dn13 = assign16710_e23929_d_n13;
        locals.var_u0_a_dn14 = assign16710_e23929_d_n14;
        locals.var_u0_a_rv = 0.0;

        let (assign16720_e23939, assign16720_e23939_d_n0, assign16720_e23939_d_n2, assign16720_e23939_d_n3, assign16720_e23939_d_n4, assign16720_e23939_d_n5, assign16720_e23939_d_n6, assign16720_e23939_d_n7, assign16720_e23939_d_n8, assign16720_e23939_d_n9, assign16720_e23939_d_n10, assign16720_e23939_d_n11, assign16720_e23939_d_n12, assign16720_e23939_d_n13, assign16720_e23939_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16720_e23933: f64 = (locals.var_uar_t * locals.var_wr);
        let assign16720_e23936: f64 = (locals.var_ua_t * locals.var_wf);
        let assign16720_e23937: f64 = (assign16720_e23933 + assign16720_e23936);
        (assign16720_e23937, (((locals.var_uar_t_dn0 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn0)) + ((locals.var_ua_t_dn0 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn0))), (((locals.var_uar_t_dn2 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn2)) + ((locals.var_ua_t_dn2 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn2))), (((locals.var_uar_t_dn3 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn3)) + ((locals.var_ua_t_dn3 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn3))), (((locals.var_uar_t_dn4 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn4)) + ((locals.var_ua_t_dn4 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn4))), (((locals.var_uar_t_dn5 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn5)) + ((locals.var_ua_t_dn5 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn5))), (((locals.var_uar_t_dn6 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn6)) + ((locals.var_ua_t_dn6 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn6))), (((locals.var_uar_t_dn7 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn7)) + ((locals.var_ua_t_dn7 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn7))), (((locals.var_uar_t_dn8 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn8)) + ((locals.var_ua_t_dn8 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn8))), (((locals.var_uar_t_dn9 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn9)) + ((locals.var_ua_t_dn9 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn9))), (((locals.var_uar_t_dn10 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn10)) + ((locals.var_ua_t_dn10 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn10))), (((locals.var_uar_t_dn11 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn11)) + ((locals.var_ua_t_dn11 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn11))), (((locals.var_uar_t_dn12 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn12)) + ((locals.var_ua_t_dn12 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn12))), (((locals.var_uar_t_dn13 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn13)) + ((locals.var_ua_t_dn13 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn13))), (((locals.var_uar_t_dn14 * locals.var_wr) + (locals.var_uar_t * locals.var_wr_dn14)) + ((locals.var_ua_t_dn14 * locals.var_wf) + (locals.var_ua_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_ua_a, locals.var_ua_a_dn0, locals.var_ua_a_dn2, locals.var_ua_a_dn3, locals.var_ua_a_dn4, locals.var_ua_a_dn5, locals.var_ua_a_dn6, locals.var_ua_a_dn7, locals.var_ua_a_dn8, locals.var_ua_a_dn9, locals.var_ua_a_dn10, locals.var_ua_a_dn11, locals.var_ua_a_dn12, locals.var_ua_a_dn13, locals.var_ua_a_dn14,)
    }
};
        locals.var_ua_a = assign16720_e23939;
        locals.var_ua_a_dn0 = assign16720_e23939_d_n0;
        locals.var_ua_a_dn2 = assign16720_e23939_d_n2;
        locals.var_ua_a_dn3 = assign16720_e23939_d_n3;
        locals.var_ua_a_dn4 = assign16720_e23939_d_n4;
        locals.var_ua_a_dn5 = assign16720_e23939_d_n5;
        locals.var_ua_a_dn6 = assign16720_e23939_d_n6;
        locals.var_ua_a_dn7 = assign16720_e23939_d_n7;
        locals.var_ua_a_dn8 = assign16720_e23939_d_n8;
        locals.var_ua_a_dn9 = assign16720_e23939_d_n9;
        locals.var_ua_a_dn10 = assign16720_e23939_d_n10;
        locals.var_ua_a_dn11 = assign16720_e23939_d_n11;
        locals.var_ua_a_dn12 = assign16720_e23939_d_n12;
        locals.var_ua_a_dn13 = assign16720_e23939_d_n13;
        locals.var_ua_a_dn14 = assign16720_e23939_d_n14;
        locals.var_ua_a_rv = 0.0;

        let (assign16730_e23949, assign16730_e23949_d_n0, assign16730_e23949_d_n2, assign16730_e23949_d_n3, assign16730_e23949_d_n4, assign16730_e23949_d_n5, assign16730_e23949_d_n6, assign16730_e23949_d_n7, assign16730_e23949_d_n8, assign16730_e23949_d_n9, assign16730_e23949_d_n10, assign16730_e23949_d_n11, assign16730_e23949_d_n12, assign16730_e23949_d_n13, assign16730_e23949_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16730_e23943: f64 = (locals.var_ucr_t * locals.var_wr);
        let assign16730_e23946: f64 = (locals.var_uc_t * locals.var_wf);
        let assign16730_e23947: f64 = (assign16730_e23943 + assign16730_e23946);
        (assign16730_e23947, (((locals.var_ucr_t_dn0 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn0)) + ((locals.var_uc_t_dn0 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn0))), (((locals.var_ucr_t_dn2 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn2)) + ((locals.var_uc_t_dn2 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn2))), (((locals.var_ucr_t_dn3 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn3)) + ((locals.var_uc_t_dn3 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn3))), (((locals.var_ucr_t_dn4 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn4)) + ((locals.var_uc_t_dn4 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn4))), (((locals.var_ucr_t_dn5 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn5)) + ((locals.var_uc_t_dn5 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn5))), (((locals.var_ucr_t_dn6 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn6)) + ((locals.var_uc_t_dn6 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn6))), (((locals.var_ucr_t_dn7 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn7)) + ((locals.var_uc_t_dn7 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn7))), (((locals.var_ucr_t_dn8 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn8)) + ((locals.var_uc_t_dn8 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn8))), (((locals.var_ucr_t_dn9 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn9)) + ((locals.var_uc_t_dn9 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn9))), (((locals.var_ucr_t_dn10 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn10)) + ((locals.var_uc_t_dn10 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn10))), (((locals.var_ucr_t_dn11 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn11)) + ((locals.var_uc_t_dn11 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn11))), (((locals.var_ucr_t_dn12 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn12)) + ((locals.var_uc_t_dn12 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn12))), (((locals.var_ucr_t_dn13 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn13)) + ((locals.var_uc_t_dn13 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn13))), (((locals.var_ucr_t_dn14 * locals.var_wr) + (locals.var_ucr_t * locals.var_wr_dn14)) + ((locals.var_uc_t_dn14 * locals.var_wf) + (locals.var_uc_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_uc_a, locals.var_uc_a_dn0, locals.var_uc_a_dn2, locals.var_uc_a_dn3, locals.var_uc_a_dn4, locals.var_uc_a_dn5, locals.var_uc_a_dn6, locals.var_uc_a_dn7, locals.var_uc_a_dn8, locals.var_uc_a_dn9, locals.var_uc_a_dn10, locals.var_uc_a_dn11, locals.var_uc_a_dn12, locals.var_uc_a_dn13, locals.var_uc_a_dn14,)
    }
};
        locals.var_uc_a = assign16730_e23949;
        locals.var_uc_a_dn0 = assign16730_e23949_d_n0;
        locals.var_uc_a_dn2 = assign16730_e23949_d_n2;
        locals.var_uc_a_dn3 = assign16730_e23949_d_n3;
        locals.var_uc_a_dn4 = assign16730_e23949_d_n4;
        locals.var_uc_a_dn5 = assign16730_e23949_d_n5;
        locals.var_uc_a_dn6 = assign16730_e23949_d_n6;
        locals.var_uc_a_dn7 = assign16730_e23949_d_n7;
        locals.var_uc_a_dn8 = assign16730_e23949_d_n8;
        locals.var_uc_a_dn9 = assign16730_e23949_d_n9;
        locals.var_uc_a_dn10 = assign16730_e23949_d_n10;
        locals.var_uc_a_dn11 = assign16730_e23949_d_n11;
        locals.var_uc_a_dn12 = assign16730_e23949_d_n12;
        locals.var_uc_a_dn13 = assign16730_e23949_d_n13;
        locals.var_uc_a_dn14 = assign16730_e23949_d_n14;
        locals.var_uc_a_rv = 0.0;

        let (assign16740_e23959, assign16740_e23959_d_n0, assign16740_e23959_d_n2, assign16740_e23959_d_n3, assign16740_e23959_d_n4, assign16740_e23959_d_n5, assign16740_e23959_d_n6, assign16740_e23959_d_n7, assign16740_e23959_d_n8, assign16740_e23959_d_n9, assign16740_e23959_d_n10, assign16740_e23959_d_n11, assign16740_e23959_d_n12, assign16740_e23959_d_n13, assign16740_e23959_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16740_e23953: f64 = (locals.var_udr_t * locals.var_wr);
        let assign16740_e23956: f64 = (locals.var_ud_t * locals.var_wf);
        let assign16740_e23957: f64 = (assign16740_e23953 + assign16740_e23956);
        (assign16740_e23957, (((locals.var_udr_t_dn0 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn0)) + ((locals.var_ud_t_dn0 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn0))), (((locals.var_udr_t_dn2 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn2)) + ((locals.var_ud_t_dn2 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn2))), (((locals.var_udr_t_dn3 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn3)) + ((locals.var_ud_t_dn3 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn3))), (((locals.var_udr_t_dn4 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn4)) + ((locals.var_ud_t_dn4 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn4))), (((locals.var_udr_t_dn5 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn5)) + ((locals.var_ud_t_dn5 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn5))), (((locals.var_udr_t_dn6 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn6)) + ((locals.var_ud_t_dn6 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn6))), (((locals.var_udr_t_dn7 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn7)) + ((locals.var_ud_t_dn7 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn7))), (((locals.var_udr_t_dn8 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn8)) + ((locals.var_ud_t_dn8 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn8))), (((locals.var_udr_t_dn9 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn9)) + ((locals.var_ud_t_dn9 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn9))), (((locals.var_udr_t_dn10 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn10)) + ((locals.var_ud_t_dn10 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn10))), (((locals.var_udr_t_dn11 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn11)) + ((locals.var_ud_t_dn11 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn11))), (((locals.var_udr_t_dn12 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn12)) + ((locals.var_ud_t_dn12 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn12))), (((locals.var_udr_t_dn13 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn13)) + ((locals.var_ud_t_dn13 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn13))), (((locals.var_udr_t_dn14 * locals.var_wr) + (locals.var_udr_t * locals.var_wr_dn14)) + ((locals.var_ud_t_dn14 * locals.var_wf) + (locals.var_ud_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_ud_a, locals.var_ud_a_dn0, locals.var_ud_a_dn2, locals.var_ud_a_dn3, locals.var_ud_a_dn4, locals.var_ud_a_dn5, locals.var_ud_a_dn6, locals.var_ud_a_dn7, locals.var_ud_a_dn8, locals.var_ud_a_dn9, locals.var_ud_a_dn10, locals.var_ud_a_dn11, locals.var_ud_a_dn12, locals.var_ud_a_dn13, locals.var_ud_a_dn14,)
    }
};
        locals.var_ud_a = assign16740_e23959;
        locals.var_ud_a_dn0 = assign16740_e23959_d_n0;
        locals.var_ud_a_dn2 = assign16740_e23959_d_n2;
        locals.var_ud_a_dn3 = assign16740_e23959_d_n3;
        locals.var_ud_a_dn4 = assign16740_e23959_d_n4;
        locals.var_ud_a_dn5 = assign16740_e23959_d_n5;
        locals.var_ud_a_dn6 = assign16740_e23959_d_n6;
        locals.var_ud_a_dn7 = assign16740_e23959_d_n7;
        locals.var_ud_a_dn8 = assign16740_e23959_d_n8;
        locals.var_ud_a_dn9 = assign16740_e23959_d_n9;
        locals.var_ud_a_dn10 = assign16740_e23959_d_n10;
        locals.var_ud_a_dn11 = assign16740_e23959_d_n11;
        locals.var_ud_a_dn12 = assign16740_e23959_d_n12;
        locals.var_ud_a_dn13 = assign16740_e23959_d_n13;
        locals.var_ud_a_dn14 = assign16740_e23959_d_n14;
        locals.var_ud_a_rv = 0.0;

        let (assign16750_e23969, assign16750_e23969_d_n0, assign16750_e23969_d_n2, assign16750_e23969_d_n3, assign16750_e23969_d_n4, assign16750_e23969_d_n5, assign16750_e23969_d_n6, assign16750_e23969_d_n7, assign16750_e23969_d_n8, assign16750_e23969_d_n9, assign16750_e23969_d_n10, assign16750_e23969_d_n11, assign16750_e23969_d_n12, assign16750_e23969_d_n13, assign16750_e23969_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16750_e23963: f64 = (locals.var_ucsr_t * locals.var_wr);
        let assign16750_e23966: f64 = (locals.var_ucs_t * locals.var_wf);
        let assign16750_e23967: f64 = (assign16750_e23963 + assign16750_e23966);
        (assign16750_e23967, ((locals.var_ucsr_t * locals.var_wr_dn0) + (locals.var_ucs_t * locals.var_wf_dn0)), ((locals.var_ucsr_t * locals.var_wr_dn2) + (locals.var_ucs_t * locals.var_wf_dn2)), ((locals.var_ucsr_t * locals.var_wr_dn3) + (locals.var_ucs_t * locals.var_wf_dn3)), (((locals.var_ucsr_t_dn4 * locals.var_wr) + (locals.var_ucsr_t * locals.var_wr_dn4)) + ((locals.var_ucs_t_dn4 * locals.var_wf) + (locals.var_ucs_t * locals.var_wf_dn4))), ((locals.var_ucsr_t * locals.var_wr_dn5) + (locals.var_ucs_t * locals.var_wf_dn5)), ((locals.var_ucsr_t * locals.var_wr_dn6) + (locals.var_ucs_t * locals.var_wf_dn6)), ((locals.var_ucsr_t * locals.var_wr_dn7) + (locals.var_ucs_t * locals.var_wf_dn7)), ((locals.var_ucsr_t * locals.var_wr_dn8) + (locals.var_ucs_t * locals.var_wf_dn8)), ((locals.var_ucsr_t * locals.var_wr_dn9) + (locals.var_ucs_t * locals.var_wf_dn9)), ((locals.var_ucsr_t * locals.var_wr_dn10) + (locals.var_ucs_t * locals.var_wf_dn10)), ((locals.var_ucsr_t * locals.var_wr_dn11) + (locals.var_ucs_t * locals.var_wf_dn11)), ((locals.var_ucsr_t * locals.var_wr_dn12) + (locals.var_ucs_t * locals.var_wf_dn12)), ((locals.var_ucsr_t * locals.var_wr_dn13) + (locals.var_ucs_t * locals.var_wf_dn13)), ((locals.var_ucsr_t * locals.var_wr_dn14) + (locals.var_ucs_t * locals.var_wf_dn14)),)
    } else {
        (locals.var_ucs_a, locals.var_ucs_a_dn0, locals.var_ucs_a_dn2, locals.var_ucs_a_dn3, locals.var_ucs_a_dn4, locals.var_ucs_a_dn5, locals.var_ucs_a_dn6, locals.var_ucs_a_dn7, locals.var_ucs_a_dn8, locals.var_ucs_a_dn9, locals.var_ucs_a_dn10, locals.var_ucs_a_dn11, locals.var_ucs_a_dn12, locals.var_ucs_a_dn13, locals.var_ucs_a_dn14,)
    }
};
        locals.var_ucs_a = assign16750_e23969;
        locals.var_ucs_a_dn0 = assign16750_e23969_d_n0;
        locals.var_ucs_a_dn2 = assign16750_e23969_d_n2;
        locals.var_ucs_a_dn3 = assign16750_e23969_d_n3;
        locals.var_ucs_a_dn4 = assign16750_e23969_d_n4;
        locals.var_ucs_a_dn5 = assign16750_e23969_d_n5;
        locals.var_ucs_a_dn6 = assign16750_e23969_d_n6;
        locals.var_ucs_a_dn7 = assign16750_e23969_d_n7;
        locals.var_ucs_a_dn8 = assign16750_e23969_d_n8;
        locals.var_ucs_a_dn9 = assign16750_e23969_d_n9;
        locals.var_ucs_a_dn10 = assign16750_e23969_d_n10;
        locals.var_ucs_a_dn11 = assign16750_e23969_d_n11;
        locals.var_ucs_a_dn12 = assign16750_e23969_d_n12;
        locals.var_ucs_a_dn13 = assign16750_e23969_d_n13;
        locals.var_ucs_a_dn14 = assign16750_e23969_d_n14;
        locals.var_ucs_a_rv = 0.0;

        let (assign16760_e23979, assign16760_e23979_d_n0, assign16760_e23979_d_n2, assign16760_e23979_d_n3, assign16760_e23979_d_n4, assign16760_e23979_d_n5, assign16760_e23979_d_n6, assign16760_e23979_d_n7, assign16760_e23979_d_n8, assign16760_e23979_d_n9, assign16760_e23979_d_n10, assign16760_e23979_d_n11, assign16760_e23979_d_n12, assign16760_e23979_d_n13, assign16760_e23979_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16760_e23973: f64 = (locals.var_alpha0r_i * locals.var_wr);
        let assign16760_e23976: f64 = (locals.var_alpha0_i * locals.var_wf);
        let assign16760_e23977: f64 = (assign16760_e23973 + assign16760_e23976);
        (assign16760_e23977, (((locals.var_alpha0r_i_dn0 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn0)) + ((locals.var_alpha0_i_dn0 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn0))), (((locals.var_alpha0r_i_dn2 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn2)) + ((locals.var_alpha0_i_dn2 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn2))), (((locals.var_alpha0r_i_dn3 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn3)) + ((locals.var_alpha0_i_dn3 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn3))), (((locals.var_alpha0r_i_dn4 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn4)) + ((locals.var_alpha0_i_dn4 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn4))), (((locals.var_alpha0r_i_dn5 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn5)) + ((locals.var_alpha0_i_dn5 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn5))), (((locals.var_alpha0r_i_dn6 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn6)) + ((locals.var_alpha0_i_dn6 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn6))), (((locals.var_alpha0r_i_dn7 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn7)) + ((locals.var_alpha0_i_dn7 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn7))), (((locals.var_alpha0r_i_dn8 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn8)) + ((locals.var_alpha0_i_dn8 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn8))), (((locals.var_alpha0r_i_dn9 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn9)) + ((locals.var_alpha0_i_dn9 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn9))), (((locals.var_alpha0r_i_dn10 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn10)) + ((locals.var_alpha0_i_dn10 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn10))), (((locals.var_alpha0r_i_dn11 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn11)) + ((locals.var_alpha0_i_dn11 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn11))), (((locals.var_alpha0r_i_dn12 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn12)) + ((locals.var_alpha0_i_dn12 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn12))), (((locals.var_alpha0r_i_dn13 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn13)) + ((locals.var_alpha0_i_dn13 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn13))), (((locals.var_alpha0r_i_dn14 * locals.var_wr) + (locals.var_alpha0r_i * locals.var_wr_dn14)) + ((locals.var_alpha0_i_dn14 * locals.var_wf) + (locals.var_alpha0_i * locals.var_wf_dn14))),)
    } else {
        (locals.var_alpha0_a, locals.var_alpha0_a_dn0, locals.var_alpha0_a_dn2, locals.var_alpha0_a_dn3, locals.var_alpha0_a_dn4, locals.var_alpha0_a_dn5, locals.var_alpha0_a_dn6, locals.var_alpha0_a_dn7, locals.var_alpha0_a_dn8, locals.var_alpha0_a_dn9, locals.var_alpha0_a_dn10, locals.var_alpha0_a_dn11, locals.var_alpha0_a_dn12, locals.var_alpha0_a_dn13, locals.var_alpha0_a_dn14,)
    }
};
        locals.var_alpha0_a = assign16760_e23979;
        locals.var_alpha0_a_dn0 = assign16760_e23979_d_n0;
        locals.var_alpha0_a_dn2 = assign16760_e23979_d_n2;
        locals.var_alpha0_a_dn3 = assign16760_e23979_d_n3;
        locals.var_alpha0_a_dn4 = assign16760_e23979_d_n4;
        locals.var_alpha0_a_dn5 = assign16760_e23979_d_n5;
        locals.var_alpha0_a_dn6 = assign16760_e23979_d_n6;
        locals.var_alpha0_a_dn7 = assign16760_e23979_d_n7;
        locals.var_alpha0_a_dn8 = assign16760_e23979_d_n8;
        locals.var_alpha0_a_dn9 = assign16760_e23979_d_n9;
        locals.var_alpha0_a_dn10 = assign16760_e23979_d_n10;
        locals.var_alpha0_a_dn11 = assign16760_e23979_d_n11;
        locals.var_alpha0_a_dn12 = assign16760_e23979_d_n12;
        locals.var_alpha0_a_dn13 = assign16760_e23979_d_n13;
        locals.var_alpha0_a_dn14 = assign16760_e23979_d_n14;
        locals.var_alpha0_a_rv = 0.0;

        let (assign16770_e23989, assign16770_e23989_d_n0, assign16770_e23989_d_n2, assign16770_e23989_d_n3, assign16770_e23989_d_n4, assign16770_e23989_d_n5, assign16770_e23989_d_n6, assign16770_e23989_d_n7, assign16770_e23989_d_n8, assign16770_e23989_d_n9, assign16770_e23989_d_n10, assign16770_e23989_d_n11, assign16770_e23989_d_n12, assign16770_e23989_d_n13, assign16770_e23989_d_n14,) = {
    if (locals.var_guard495 != 0.0) {
        let assign16770_e23983: f64 = (locals.var_beta0r_t * locals.var_wr);
        let assign16770_e23986: f64 = (locals.var_beta0_t * locals.var_wf);
        let assign16770_e23987: f64 = (assign16770_e23983 + assign16770_e23986);
        (assign16770_e23987, ((locals.var_beta0r_t * locals.var_wr_dn0) + ((locals.var_beta0_t_dn0 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn0))), ((locals.var_beta0r_t * locals.var_wr_dn2) + ((locals.var_beta0_t_dn2 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn2))), ((locals.var_beta0r_t * locals.var_wr_dn3) + ((locals.var_beta0_t_dn3 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn3))), (((locals.var_beta0r_t_dn4 * locals.var_wr) + (locals.var_beta0r_t * locals.var_wr_dn4)) + ((locals.var_beta0_t_dn4 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn4))), ((locals.var_beta0r_t * locals.var_wr_dn5) + ((locals.var_beta0_t_dn5 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn5))), ((locals.var_beta0r_t * locals.var_wr_dn6) + ((locals.var_beta0_t_dn6 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn6))), ((locals.var_beta0r_t * locals.var_wr_dn7) + ((locals.var_beta0_t_dn7 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn7))), ((locals.var_beta0r_t * locals.var_wr_dn8) + ((locals.var_beta0_t_dn8 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn8))), ((locals.var_beta0r_t * locals.var_wr_dn9) + ((locals.var_beta0_t_dn9 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn9))), ((locals.var_beta0r_t * locals.var_wr_dn10) + ((locals.var_beta0_t_dn10 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn10))), ((locals.var_beta0r_t * locals.var_wr_dn11) + ((locals.var_beta0_t_dn11 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn11))), ((locals.var_beta0r_t * locals.var_wr_dn12) + ((locals.var_beta0_t_dn12 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn12))), ((locals.var_beta0r_t * locals.var_wr_dn13) + ((locals.var_beta0_t_dn13 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn13))), ((locals.var_beta0r_t * locals.var_wr_dn14) + ((locals.var_beta0_t_dn14 * locals.var_wf) + (locals.var_beta0_t * locals.var_wf_dn14))),)
    } else {
        (locals.var_beta0_a, locals.var_beta0_a_dn0, locals.var_beta0_a_dn2, locals.var_beta0_a_dn3, locals.var_beta0_a_dn4, locals.var_beta0_a_dn5, locals.var_beta0_a_dn6, locals.var_beta0_a_dn7, locals.var_beta0_a_dn8, locals.var_beta0_a_dn9, locals.var_beta0_a_dn10, locals.var_beta0_a_dn11, locals.var_beta0_a_dn12, locals.var_beta0_a_dn13, locals.var_beta0_a_dn14,)
    }
};
        locals.var_beta0_a = assign16770_e23989;
        locals.var_beta0_a_dn0 = assign16770_e23989_d_n0;
        locals.var_beta0_a_dn2 = assign16770_e23989_d_n2;
        locals.var_beta0_a_dn3 = assign16770_e23989_d_n3;
        locals.var_beta0_a_dn4 = assign16770_e23989_d_n4;
        locals.var_beta0_a_dn5 = assign16770_e23989_d_n5;
        locals.var_beta0_a_dn6 = assign16770_e23989_d_n6;
        locals.var_beta0_a_dn7 = assign16770_e23989_d_n7;
        locals.var_beta0_a_dn8 = assign16770_e23989_d_n8;
        locals.var_beta0_a_dn9 = assign16770_e23989_d_n9;
        locals.var_beta0_a_dn10 = assign16770_e23989_d_n10;
        locals.var_beta0_a_dn11 = assign16770_e23989_d_n11;
        locals.var_beta0_a_dn12 = assign16770_e23989_d_n12;
        locals.var_beta0_a_dn13 = assign16770_e23989_d_n13;
        locals.var_beta0_a_dn14 = assign16770_e23989_d_n14;
        locals.var_beta0_a_rv = 0.0;

        let (assign16780_e23994, assign16780_e23994_d_n0, assign16780_e23994_d_n2, assign16780_e23994_d_n3, assign16780_e23994_d_n4, assign16780_e23994_d_n5, assign16780_e23994_d_n6, assign16780_e23994_d_n7, assign16780_e23994_d_n8, assign16780_e23994_d_n9, assign16780_e23994_d_n10, assign16780_e23994_d_n11, assign16780_e23994_d_n12, assign16780_e23994_d_n13, assign16780_e23994_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_cdscd_i, locals.var_cdscd_i_dn0, locals.var_cdscd_i_dn2, locals.var_cdscd_i_dn3, locals.var_cdscd_i_dn4, locals.var_cdscd_i_dn5, locals.var_cdscd_i_dn6, locals.var_cdscd_i_dn7, locals.var_cdscd_i_dn8, locals.var_cdscd_i_dn9, locals.var_cdscd_i_dn10, locals.var_cdscd_i_dn11, locals.var_cdscd_i_dn12, locals.var_cdscd_i_dn13, locals.var_cdscd_i_dn14,)
    } else {
        (locals.var_cdscd_a, locals.var_cdscd_a_dn0, locals.var_cdscd_a_dn2, locals.var_cdscd_a_dn3, locals.var_cdscd_a_dn4, locals.var_cdscd_a_dn5, locals.var_cdscd_a_dn6, locals.var_cdscd_a_dn7, locals.var_cdscd_a_dn8, locals.var_cdscd_a_dn9, locals.var_cdscd_a_dn10, locals.var_cdscd_a_dn11, locals.var_cdscd_a_dn12, locals.var_cdscd_a_dn13, locals.var_cdscd_a_dn14,)
    }
};
        locals.var_cdscd_a = assign16780_e23994;
        locals.var_cdscd_a_dn0 = assign16780_e23994_d_n0;
        locals.var_cdscd_a_dn2 = assign16780_e23994_d_n2;
        locals.var_cdscd_a_dn3 = assign16780_e23994_d_n3;
        locals.var_cdscd_a_dn4 = assign16780_e23994_d_n4;
        locals.var_cdscd_a_dn5 = assign16780_e23994_d_n5;
        locals.var_cdscd_a_dn6 = assign16780_e23994_d_n6;
        locals.var_cdscd_a_dn7 = assign16780_e23994_d_n7;
        locals.var_cdscd_a_dn8 = assign16780_e23994_d_n8;
        locals.var_cdscd_a_dn9 = assign16780_e23994_d_n9;
        locals.var_cdscd_a_dn10 = assign16780_e23994_d_n10;
        locals.var_cdscd_a_dn11 = assign16780_e23994_d_n11;
        locals.var_cdscd_a_dn12 = assign16780_e23994_d_n12;
        locals.var_cdscd_a_dn13 = assign16780_e23994_d_n13;
        locals.var_cdscd_a_dn14 = assign16780_e23994_d_n14;
        locals.var_cdscd_a_rv = 0.0;

        let (assign16790_e23999, assign16790_e23999_d_n0, assign16790_e23999_d_n2, assign16790_e23999_d_n3, assign16790_e23999_d_n4, assign16790_e23999_d_n5, assign16790_e23999_d_n6, assign16790_e23999_d_n7, assign16790_e23999_d_n8, assign16790_e23999_d_n9, assign16790_e23999_d_n10, assign16790_e23999_d_n11, assign16790_e23999_d_n12, assign16790_e23999_d_n13, assign16790_e23999_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_eta0_t, locals.var_eta0_t_dn0, locals.var_eta0_t_dn2, locals.var_eta0_t_dn3, locals.var_eta0_t_dn4, locals.var_eta0_t_dn5, locals.var_eta0_t_dn6, locals.var_eta0_t_dn7, locals.var_eta0_t_dn8, locals.var_eta0_t_dn9, locals.var_eta0_t_dn10, locals.var_eta0_t_dn11, locals.var_eta0_t_dn12, locals.var_eta0_t_dn13, locals.var_eta0_t_dn14,)
    } else {
        (locals.var_eta0_a, locals.var_eta0_a_dn0, locals.var_eta0_a_dn2, locals.var_eta0_a_dn3, locals.var_eta0_a_dn4, locals.var_eta0_a_dn5, locals.var_eta0_a_dn6, locals.var_eta0_a_dn7, locals.var_eta0_a_dn8, locals.var_eta0_a_dn9, locals.var_eta0_a_dn10, locals.var_eta0_a_dn11, locals.var_eta0_a_dn12, locals.var_eta0_a_dn13, locals.var_eta0_a_dn14,)
    }
};
        locals.var_eta0_a = assign16790_e23999;
        locals.var_eta0_a_dn0 = assign16790_e23999_d_n0;
        locals.var_eta0_a_dn2 = assign16790_e23999_d_n2;
        locals.var_eta0_a_dn3 = assign16790_e23999_d_n3;
        locals.var_eta0_a_dn4 = assign16790_e23999_d_n4;
        locals.var_eta0_a_dn5 = assign16790_e23999_d_n5;
        locals.var_eta0_a_dn6 = assign16790_e23999_d_n6;
        locals.var_eta0_a_dn7 = assign16790_e23999_d_n7;
        locals.var_eta0_a_dn8 = assign16790_e23999_d_n8;
        locals.var_eta0_a_dn9 = assign16790_e23999_d_n9;
        locals.var_eta0_a_dn10 = assign16790_e23999_d_n10;
        locals.var_eta0_a_dn11 = assign16790_e23999_d_n11;
        locals.var_eta0_a_dn12 = assign16790_e23999_d_n12;
        locals.var_eta0_a_dn13 = assign16790_e23999_d_n13;
        locals.var_eta0_a_dn14 = assign16790_e23999_d_n14;
        locals.var_eta0_a_rv = 0.0;

        let (assign16800_e24004, assign16800_e24004_d_n0, assign16800_e24004_d_n2, assign16800_e24004_d_n3, assign16800_e24004_d_n4, assign16800_e24004_d_n5, assign16800_e24004_d_n6, assign16800_e24004_d_n7, assign16800_e24004_d_n8, assign16800_e24004_d_n9, assign16800_e24004_d_n10, assign16800_e24004_d_n11, assign16800_e24004_d_n12, assign16800_e24004_d_n13, assign16800_e24004_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_pdiblc_i, locals.var_pdiblc_i_dn0, locals.var_pdiblc_i_dn2, locals.var_pdiblc_i_dn3, locals.var_pdiblc_i_dn4, locals.var_pdiblc_i_dn5, locals.var_pdiblc_i_dn6, locals.var_pdiblc_i_dn7, locals.var_pdiblc_i_dn8, locals.var_pdiblc_i_dn9, locals.var_pdiblc_i_dn10, locals.var_pdiblc_i_dn11, locals.var_pdiblc_i_dn12, locals.var_pdiblc_i_dn13, locals.var_pdiblc_i_dn14,)
    } else {
        (locals.var_pdiblc_a, locals.var_pdiblc_a_dn0, locals.var_pdiblc_a_dn2, locals.var_pdiblc_a_dn3, locals.var_pdiblc_a_dn4, locals.var_pdiblc_a_dn5, locals.var_pdiblc_a_dn6, locals.var_pdiblc_a_dn7, locals.var_pdiblc_a_dn8, locals.var_pdiblc_a_dn9, locals.var_pdiblc_a_dn10, locals.var_pdiblc_a_dn11, locals.var_pdiblc_a_dn12, locals.var_pdiblc_a_dn13, locals.var_pdiblc_a_dn14,)
    }
};
        locals.var_pdiblc_a = assign16800_e24004;
        locals.var_pdiblc_a_dn0 = assign16800_e24004_d_n0;
        locals.var_pdiblc_a_dn2 = assign16800_e24004_d_n2;
        locals.var_pdiblc_a_dn3 = assign16800_e24004_d_n3;
        locals.var_pdiblc_a_dn4 = assign16800_e24004_d_n4;
        locals.var_pdiblc_a_dn5 = assign16800_e24004_d_n5;
        locals.var_pdiblc_a_dn6 = assign16800_e24004_d_n6;
        locals.var_pdiblc_a_dn7 = assign16800_e24004_d_n7;
        locals.var_pdiblc_a_dn8 = assign16800_e24004_d_n8;
        locals.var_pdiblc_a_dn9 = assign16800_e24004_d_n9;
        locals.var_pdiblc_a_dn10 = assign16800_e24004_d_n10;
        locals.var_pdiblc_a_dn11 = assign16800_e24004_d_n11;
        locals.var_pdiblc_a_dn12 = assign16800_e24004_d_n12;
        locals.var_pdiblc_a_dn13 = assign16800_e24004_d_n13;
        locals.var_pdiblc_a_dn14 = assign16800_e24004_d_n14;
        locals.var_pdiblc_a_rv = 0.0;

        let (assign16810_e24009, assign16810_e24009_d_n0, assign16810_e24009_d_n2, assign16810_e24009_d_n3, assign16810_e24009_d_n4, assign16810_e24009_d_n5, assign16810_e24009_d_n6, assign16810_e24009_d_n7, assign16810_e24009_d_n8, assign16810_e24009_d_n9, assign16810_e24009_d_n10, assign16810_e24009_d_n11, assign16810_e24009_d_n12, assign16810_e24009_d_n13, assign16810_e24009_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_pclm_i, locals.var_pclm_i_dn0, locals.var_pclm_i_dn2, locals.var_pclm_i_dn3, locals.var_pclm_i_dn4, locals.var_pclm_i_dn5, locals.var_pclm_i_dn6, locals.var_pclm_i_dn7, locals.var_pclm_i_dn8, locals.var_pclm_i_dn9, locals.var_pclm_i_dn10, locals.var_pclm_i_dn11, locals.var_pclm_i_dn12, locals.var_pclm_i_dn13, locals.var_pclm_i_dn14,)
    } else {
        (locals.var_pclm_a, locals.var_pclm_a_dn0, locals.var_pclm_a_dn2, locals.var_pclm_a_dn3, locals.var_pclm_a_dn4, locals.var_pclm_a_dn5, locals.var_pclm_a_dn6, locals.var_pclm_a_dn7, locals.var_pclm_a_dn8, locals.var_pclm_a_dn9, locals.var_pclm_a_dn10, locals.var_pclm_a_dn11, locals.var_pclm_a_dn12, locals.var_pclm_a_dn13, locals.var_pclm_a_dn14,)
    }
};
        locals.var_pclm_a = assign16810_e24009;
        locals.var_pclm_a_dn0 = assign16810_e24009_d_n0;
        locals.var_pclm_a_dn2 = assign16810_e24009_d_n2;
        locals.var_pclm_a_dn3 = assign16810_e24009_d_n3;
        locals.var_pclm_a_dn4 = assign16810_e24009_d_n4;
        locals.var_pclm_a_dn5 = assign16810_e24009_d_n5;
        locals.var_pclm_a_dn6 = assign16810_e24009_d_n6;
        locals.var_pclm_a_dn7 = assign16810_e24009_d_n7;
        locals.var_pclm_a_dn8 = assign16810_e24009_d_n8;
        locals.var_pclm_a_dn9 = assign16810_e24009_d_n9;
        locals.var_pclm_a_dn10 = assign16810_e24009_d_n10;
        locals.var_pclm_a_dn11 = assign16810_e24009_d_n11;
        locals.var_pclm_a_dn12 = assign16810_e24009_d_n12;
        locals.var_pclm_a_dn13 = assign16810_e24009_d_n13;
        locals.var_pclm_a_dn14 = assign16810_e24009_d_n14;
        locals.var_pclm_a_rv = 0.0;

        let (assign16820_e24014, assign16820_e24014_d_n0, assign16820_e24014_d_n2, assign16820_e24014_d_n3, assign16820_e24014_d_n4, assign16820_e24014_d_n5, assign16820_e24014_d_n6, assign16820_e24014_d_n7, assign16820_e24014_d_n8, assign16820_e24014_d_n9, assign16820_e24014_d_n10, assign16820_e24014_d_n11, assign16820_e24014_d_n12, assign16820_e24014_d_n13, assign16820_e24014_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_psat_i, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psat_a, locals.var_psat_a_dn0, locals.var_psat_a_dn2, locals.var_psat_a_dn3, locals.var_psat_a_dn4, locals.var_psat_a_dn5, locals.var_psat_a_dn6, locals.var_psat_a_dn7, locals.var_psat_a_dn8, locals.var_psat_a_dn9, locals.var_psat_a_dn10, locals.var_psat_a_dn11, locals.var_psat_a_dn12, locals.var_psat_a_dn13, locals.var_psat_a_dn14,)
    }
};
        locals.var_psat_a = assign16820_e24014;
        locals.var_psat_a_dn0 = assign16820_e24014_d_n0;
        locals.var_psat_a_dn2 = assign16820_e24014_d_n2;
        locals.var_psat_a_dn3 = assign16820_e24014_d_n3;
        locals.var_psat_a_dn4 = assign16820_e24014_d_n4;
        locals.var_psat_a_dn5 = assign16820_e24014_d_n5;
        locals.var_psat_a_dn6 = assign16820_e24014_d_n6;
        locals.var_psat_a_dn7 = assign16820_e24014_d_n7;
        locals.var_psat_a_dn8 = assign16820_e24014_d_n8;
        locals.var_psat_a_dn9 = assign16820_e24014_d_n9;
        locals.var_psat_a_dn10 = assign16820_e24014_d_n10;
        locals.var_psat_a_dn11 = assign16820_e24014_d_n11;
        locals.var_psat_a_dn12 = assign16820_e24014_d_n12;
        locals.var_psat_a_dn13 = assign16820_e24014_d_n13;
        locals.var_psat_a_dn14 = assign16820_e24014_d_n14;
        locals.var_psat_a_rv = 0.0;

        let (assign16830_e24019, assign16830_e24019_d_n0, assign16830_e24019_d_n2, assign16830_e24019_d_n3, assign16830_e24019_d_n4, assign16830_e24019_d_n5, assign16830_e24019_d_n6, assign16830_e24019_d_n7, assign16830_e24019_d_n8, assign16830_e24019_d_n9, assign16830_e24019_d_n10, assign16830_e24019_d_n11, assign16830_e24019_d_n12, assign16830_e24019_d_n13, assign16830_e24019_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn12, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    } else {
        (locals.var_vsat_a, locals.var_vsat_a_dn0, locals.var_vsat_a_dn2, locals.var_vsat_a_dn3, locals.var_vsat_a_dn4, locals.var_vsat_a_dn5, locals.var_vsat_a_dn6, locals.var_vsat_a_dn7, locals.var_vsat_a_dn8, locals.var_vsat_a_dn9, locals.var_vsat_a_dn10, locals.var_vsat_a_dn11, locals.var_vsat_a_dn12, locals.var_vsat_a_dn13, locals.var_vsat_a_dn14,)
    }
};
        locals.var_vsat_a = assign16830_e24019;
        locals.var_vsat_a_dn0 = assign16830_e24019_d_n0;
        locals.var_vsat_a_dn2 = assign16830_e24019_d_n2;
        locals.var_vsat_a_dn3 = assign16830_e24019_d_n3;
        locals.var_vsat_a_dn4 = assign16830_e24019_d_n4;
        locals.var_vsat_a_dn5 = assign16830_e24019_d_n5;
        locals.var_vsat_a_dn6 = assign16830_e24019_d_n6;
        locals.var_vsat_a_dn7 = assign16830_e24019_d_n7;
        locals.var_vsat_a_dn8 = assign16830_e24019_d_n8;
        locals.var_vsat_a_dn9 = assign16830_e24019_d_n9;
        locals.var_vsat_a_dn10 = assign16830_e24019_d_n10;
        locals.var_vsat_a_dn11 = assign16830_e24019_d_n11;
        locals.var_vsat_a_dn12 = assign16830_e24019_d_n12;
        locals.var_vsat_a_dn13 = assign16830_e24019_d_n13;
        locals.var_vsat_a_dn14 = assign16830_e24019_d_n14;
        locals.var_vsat_a_rv = 0.0;

        let (assign16840_e24024, assign16840_e24024_d_n0, assign16840_e24024_d_n2, assign16840_e24024_d_n3, assign16840_e24024_d_n4, assign16840_e24024_d_n5, assign16840_e24024_d_n6, assign16840_e24024_d_n7, assign16840_e24024_d_n8, assign16840_e24024_d_n9, assign16840_e24024_d_n10, assign16840_e24024_d_n11, assign16840_e24024_d_n12, assign16840_e24024_d_n13, assign16840_e24024_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_ptwg_t, locals.var_ptwg_t_dn0, locals.var_ptwg_t_dn2, locals.var_ptwg_t_dn3, locals.var_ptwg_t_dn4, locals.var_ptwg_t_dn5, locals.var_ptwg_t_dn6, locals.var_ptwg_t_dn7, locals.var_ptwg_t_dn8, locals.var_ptwg_t_dn9, locals.var_ptwg_t_dn10, locals.var_ptwg_t_dn11, locals.var_ptwg_t_dn12, locals.var_ptwg_t_dn13, locals.var_ptwg_t_dn14,)
    } else {
        (locals.var_ptwg_a, locals.var_ptwg_a_dn0, locals.var_ptwg_a_dn2, locals.var_ptwg_a_dn3, locals.var_ptwg_a_dn4, locals.var_ptwg_a_dn5, locals.var_ptwg_a_dn6, locals.var_ptwg_a_dn7, locals.var_ptwg_a_dn8, locals.var_ptwg_a_dn9, locals.var_ptwg_a_dn10, locals.var_ptwg_a_dn11, locals.var_ptwg_a_dn12, locals.var_ptwg_a_dn13, locals.var_ptwg_a_dn14,)
    }
};
        locals.var_ptwg_a = assign16840_e24024;
        locals.var_ptwg_a_dn0 = assign16840_e24024_d_n0;
        locals.var_ptwg_a_dn2 = assign16840_e24024_d_n2;
        locals.var_ptwg_a_dn3 = assign16840_e24024_d_n3;
        locals.var_ptwg_a_dn4 = assign16840_e24024_d_n4;
        locals.var_ptwg_a_dn5 = assign16840_e24024_d_n5;
        locals.var_ptwg_a_dn6 = assign16840_e24024_d_n6;
        locals.var_ptwg_a_dn7 = assign16840_e24024_d_n7;
        locals.var_ptwg_a_dn8 = assign16840_e24024_d_n8;
        locals.var_ptwg_a_dn9 = assign16840_e24024_d_n9;
        locals.var_ptwg_a_dn10 = assign16840_e24024_d_n10;
        locals.var_ptwg_a_dn11 = assign16840_e24024_d_n11;
        locals.var_ptwg_a_dn12 = assign16840_e24024_d_n12;
        locals.var_ptwg_a_dn13 = assign16840_e24024_d_n13;
        locals.var_ptwg_a_dn14 = assign16840_e24024_d_n14;
        locals.var_ptwg_a_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_39(
        locals: &mut StampLocals,
    ) {
        let (assign16850_e24029, assign16850_e24029_d_n0, assign16850_e24029_d_n2, assign16850_e24029_d_n3, assign16850_e24029_d_n4, assign16850_e24029_d_n5, assign16850_e24029_d_n6, assign16850_e24029_d_n7, assign16850_e24029_d_n8, assign16850_e24029_d_n9, assign16850_e24029_d_n10, assign16850_e24029_d_n11, assign16850_e24029_d_n12, assign16850_e24029_d_n13, assign16850_e24029_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_u0_t, locals.var_u0_t_dn0, locals.var_u0_t_dn2, locals.var_u0_t_dn3, locals.var_u0_t_dn4, locals.var_u0_t_dn5, locals.var_u0_t_dn6, locals.var_u0_t_dn7, locals.var_u0_t_dn8, locals.var_u0_t_dn9, locals.var_u0_t_dn10, locals.var_u0_t_dn11, locals.var_u0_t_dn12, locals.var_u0_t_dn13, locals.var_u0_t_dn14,)
    } else {
        (locals.var_u0_a, locals.var_u0_a_dn0, locals.var_u0_a_dn2, locals.var_u0_a_dn3, locals.var_u0_a_dn4, locals.var_u0_a_dn5, locals.var_u0_a_dn6, locals.var_u0_a_dn7, locals.var_u0_a_dn8, locals.var_u0_a_dn9, locals.var_u0_a_dn10, locals.var_u0_a_dn11, locals.var_u0_a_dn12, locals.var_u0_a_dn13, locals.var_u0_a_dn14,)
    }
};
        locals.var_u0_a = assign16850_e24029;
        locals.var_u0_a_dn0 = assign16850_e24029_d_n0;
        locals.var_u0_a_dn2 = assign16850_e24029_d_n2;
        locals.var_u0_a_dn3 = assign16850_e24029_d_n3;
        locals.var_u0_a_dn4 = assign16850_e24029_d_n4;
        locals.var_u0_a_dn5 = assign16850_e24029_d_n5;
        locals.var_u0_a_dn6 = assign16850_e24029_d_n6;
        locals.var_u0_a_dn7 = assign16850_e24029_d_n7;
        locals.var_u0_a_dn8 = assign16850_e24029_d_n8;
        locals.var_u0_a_dn9 = assign16850_e24029_d_n9;
        locals.var_u0_a_dn10 = assign16850_e24029_d_n10;
        locals.var_u0_a_dn11 = assign16850_e24029_d_n11;
        locals.var_u0_a_dn12 = assign16850_e24029_d_n12;
        locals.var_u0_a_dn13 = assign16850_e24029_d_n13;
        locals.var_u0_a_dn14 = assign16850_e24029_d_n14;
        locals.var_u0_a_rv = 0.0;

        let (assign16860_e24034, assign16860_e24034_d_n0, assign16860_e24034_d_n2, assign16860_e24034_d_n3, assign16860_e24034_d_n4, assign16860_e24034_d_n5, assign16860_e24034_d_n6, assign16860_e24034_d_n7, assign16860_e24034_d_n8, assign16860_e24034_d_n9, assign16860_e24034_d_n10, assign16860_e24034_d_n11, assign16860_e24034_d_n12, assign16860_e24034_d_n13, assign16860_e24034_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn12, locals.var_ua_t_dn13, locals.var_ua_t_dn14,)
    } else {
        (locals.var_ua_a, locals.var_ua_a_dn0, locals.var_ua_a_dn2, locals.var_ua_a_dn3, locals.var_ua_a_dn4, locals.var_ua_a_dn5, locals.var_ua_a_dn6, locals.var_ua_a_dn7, locals.var_ua_a_dn8, locals.var_ua_a_dn9, locals.var_ua_a_dn10, locals.var_ua_a_dn11, locals.var_ua_a_dn12, locals.var_ua_a_dn13, locals.var_ua_a_dn14,)
    }
};
        locals.var_ua_a = assign16860_e24034;
        locals.var_ua_a_dn0 = assign16860_e24034_d_n0;
        locals.var_ua_a_dn2 = assign16860_e24034_d_n2;
        locals.var_ua_a_dn3 = assign16860_e24034_d_n3;
        locals.var_ua_a_dn4 = assign16860_e24034_d_n4;
        locals.var_ua_a_dn5 = assign16860_e24034_d_n5;
        locals.var_ua_a_dn6 = assign16860_e24034_d_n6;
        locals.var_ua_a_dn7 = assign16860_e24034_d_n7;
        locals.var_ua_a_dn8 = assign16860_e24034_d_n8;
        locals.var_ua_a_dn9 = assign16860_e24034_d_n9;
        locals.var_ua_a_dn10 = assign16860_e24034_d_n10;
        locals.var_ua_a_dn11 = assign16860_e24034_d_n11;
        locals.var_ua_a_dn12 = assign16860_e24034_d_n12;
        locals.var_ua_a_dn13 = assign16860_e24034_d_n13;
        locals.var_ua_a_dn14 = assign16860_e24034_d_n14;
        locals.var_ua_a_rv = 0.0;

        let (assign16870_e24039, assign16870_e24039_d_n0, assign16870_e24039_d_n2, assign16870_e24039_d_n3, assign16870_e24039_d_n4, assign16870_e24039_d_n5, assign16870_e24039_d_n6, assign16870_e24039_d_n7, assign16870_e24039_d_n8, assign16870_e24039_d_n9, assign16870_e24039_d_n10, assign16870_e24039_d_n11, assign16870_e24039_d_n12, assign16870_e24039_d_n13, assign16870_e24039_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_uc_t, locals.var_uc_t_dn0, locals.var_uc_t_dn2, locals.var_uc_t_dn3, locals.var_uc_t_dn4, locals.var_uc_t_dn5, locals.var_uc_t_dn6, locals.var_uc_t_dn7, locals.var_uc_t_dn8, locals.var_uc_t_dn9, locals.var_uc_t_dn10, locals.var_uc_t_dn11, locals.var_uc_t_dn12, locals.var_uc_t_dn13, locals.var_uc_t_dn14,)
    } else {
        (locals.var_uc_a, locals.var_uc_a_dn0, locals.var_uc_a_dn2, locals.var_uc_a_dn3, locals.var_uc_a_dn4, locals.var_uc_a_dn5, locals.var_uc_a_dn6, locals.var_uc_a_dn7, locals.var_uc_a_dn8, locals.var_uc_a_dn9, locals.var_uc_a_dn10, locals.var_uc_a_dn11, locals.var_uc_a_dn12, locals.var_uc_a_dn13, locals.var_uc_a_dn14,)
    }
};
        locals.var_uc_a = assign16870_e24039;
        locals.var_uc_a_dn0 = assign16870_e24039_d_n0;
        locals.var_uc_a_dn2 = assign16870_e24039_d_n2;
        locals.var_uc_a_dn3 = assign16870_e24039_d_n3;
        locals.var_uc_a_dn4 = assign16870_e24039_d_n4;
        locals.var_uc_a_dn5 = assign16870_e24039_d_n5;
        locals.var_uc_a_dn6 = assign16870_e24039_d_n6;
        locals.var_uc_a_dn7 = assign16870_e24039_d_n7;
        locals.var_uc_a_dn8 = assign16870_e24039_d_n8;
        locals.var_uc_a_dn9 = assign16870_e24039_d_n9;
        locals.var_uc_a_dn10 = assign16870_e24039_d_n10;
        locals.var_uc_a_dn11 = assign16870_e24039_d_n11;
        locals.var_uc_a_dn12 = assign16870_e24039_d_n12;
        locals.var_uc_a_dn13 = assign16870_e24039_d_n13;
        locals.var_uc_a_dn14 = assign16870_e24039_d_n14;
        locals.var_uc_a_rv = 0.0;

        let (assign16880_e24044, assign16880_e24044_d_n0, assign16880_e24044_d_n2, assign16880_e24044_d_n3, assign16880_e24044_d_n4, assign16880_e24044_d_n5, assign16880_e24044_d_n6, assign16880_e24044_d_n7, assign16880_e24044_d_n8, assign16880_e24044_d_n9, assign16880_e24044_d_n10, assign16880_e24044_d_n11, assign16880_e24044_d_n12, assign16880_e24044_d_n13, assign16880_e24044_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn12, locals.var_ud_t_dn13, locals.var_ud_t_dn14,)
    } else {
        (locals.var_ud_a, locals.var_ud_a_dn0, locals.var_ud_a_dn2, locals.var_ud_a_dn3, locals.var_ud_a_dn4, locals.var_ud_a_dn5, locals.var_ud_a_dn6, locals.var_ud_a_dn7, locals.var_ud_a_dn8, locals.var_ud_a_dn9, locals.var_ud_a_dn10, locals.var_ud_a_dn11, locals.var_ud_a_dn12, locals.var_ud_a_dn13, locals.var_ud_a_dn14,)
    }
};
        locals.var_ud_a = assign16880_e24044;
        locals.var_ud_a_dn0 = assign16880_e24044_d_n0;
        locals.var_ud_a_dn2 = assign16880_e24044_d_n2;
        locals.var_ud_a_dn3 = assign16880_e24044_d_n3;
        locals.var_ud_a_dn4 = assign16880_e24044_d_n4;
        locals.var_ud_a_dn5 = assign16880_e24044_d_n5;
        locals.var_ud_a_dn6 = assign16880_e24044_d_n6;
        locals.var_ud_a_dn7 = assign16880_e24044_d_n7;
        locals.var_ud_a_dn8 = assign16880_e24044_d_n8;
        locals.var_ud_a_dn9 = assign16880_e24044_d_n9;
        locals.var_ud_a_dn10 = assign16880_e24044_d_n10;
        locals.var_ud_a_dn11 = assign16880_e24044_d_n11;
        locals.var_ud_a_dn12 = assign16880_e24044_d_n12;
        locals.var_ud_a_dn13 = assign16880_e24044_d_n13;
        locals.var_ud_a_dn14 = assign16880_e24044_d_n14;
        locals.var_ud_a_rv = 0.0;

        let (assign16890_e24049, assign16890_e24049_d_n0, assign16890_e24049_d_n2, assign16890_e24049_d_n3, assign16890_e24049_d_n4, assign16890_e24049_d_n5, assign16890_e24049_d_n6, assign16890_e24049_d_n7, assign16890_e24049_d_n8, assign16890_e24049_d_n9, assign16890_e24049_d_n10, assign16890_e24049_d_n11, assign16890_e24049_d_n12, assign16890_e24049_d_n13, assign16890_e24049_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_ucs_t, 0.0, 0.0, 0.0, locals.var_ucs_t_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ucs_a, locals.var_ucs_a_dn0, locals.var_ucs_a_dn2, locals.var_ucs_a_dn3, locals.var_ucs_a_dn4, locals.var_ucs_a_dn5, locals.var_ucs_a_dn6, locals.var_ucs_a_dn7, locals.var_ucs_a_dn8, locals.var_ucs_a_dn9, locals.var_ucs_a_dn10, locals.var_ucs_a_dn11, locals.var_ucs_a_dn12, locals.var_ucs_a_dn13, locals.var_ucs_a_dn14,)
    }
};
        locals.var_ucs_a = assign16890_e24049;
        locals.var_ucs_a_dn0 = assign16890_e24049_d_n0;
        locals.var_ucs_a_dn2 = assign16890_e24049_d_n2;
        locals.var_ucs_a_dn3 = assign16890_e24049_d_n3;
        locals.var_ucs_a_dn4 = assign16890_e24049_d_n4;
        locals.var_ucs_a_dn5 = assign16890_e24049_d_n5;
        locals.var_ucs_a_dn6 = assign16890_e24049_d_n6;
        locals.var_ucs_a_dn7 = assign16890_e24049_d_n7;
        locals.var_ucs_a_dn8 = assign16890_e24049_d_n8;
        locals.var_ucs_a_dn9 = assign16890_e24049_d_n9;
        locals.var_ucs_a_dn10 = assign16890_e24049_d_n10;
        locals.var_ucs_a_dn11 = assign16890_e24049_d_n11;
        locals.var_ucs_a_dn12 = assign16890_e24049_d_n12;
        locals.var_ucs_a_dn13 = assign16890_e24049_d_n13;
        locals.var_ucs_a_dn14 = assign16890_e24049_d_n14;
        locals.var_ucs_a_rv = 0.0;

        let (assign16900_e24054, assign16900_e24054_d_n0, assign16900_e24054_d_n2, assign16900_e24054_d_n3, assign16900_e24054_d_n4, assign16900_e24054_d_n5, assign16900_e24054_d_n6, assign16900_e24054_d_n7, assign16900_e24054_d_n8, assign16900_e24054_d_n9, assign16900_e24054_d_n10, assign16900_e24054_d_n11, assign16900_e24054_d_n12, assign16900_e24054_d_n13, assign16900_e24054_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_alpha0_i, locals.var_alpha0_i_dn0, locals.var_alpha0_i_dn2, locals.var_alpha0_i_dn3, locals.var_alpha0_i_dn4, locals.var_alpha0_i_dn5, locals.var_alpha0_i_dn6, locals.var_alpha0_i_dn7, locals.var_alpha0_i_dn8, locals.var_alpha0_i_dn9, locals.var_alpha0_i_dn10, locals.var_alpha0_i_dn11, locals.var_alpha0_i_dn12, locals.var_alpha0_i_dn13, locals.var_alpha0_i_dn14,)
    } else {
        (locals.var_alpha0_a, locals.var_alpha0_a_dn0, locals.var_alpha0_a_dn2, locals.var_alpha0_a_dn3, locals.var_alpha0_a_dn4, locals.var_alpha0_a_dn5, locals.var_alpha0_a_dn6, locals.var_alpha0_a_dn7, locals.var_alpha0_a_dn8, locals.var_alpha0_a_dn9, locals.var_alpha0_a_dn10, locals.var_alpha0_a_dn11, locals.var_alpha0_a_dn12, locals.var_alpha0_a_dn13, locals.var_alpha0_a_dn14,)
    }
};
        locals.var_alpha0_a = assign16900_e24054;
        locals.var_alpha0_a_dn0 = assign16900_e24054_d_n0;
        locals.var_alpha0_a_dn2 = assign16900_e24054_d_n2;
        locals.var_alpha0_a_dn3 = assign16900_e24054_d_n3;
        locals.var_alpha0_a_dn4 = assign16900_e24054_d_n4;
        locals.var_alpha0_a_dn5 = assign16900_e24054_d_n5;
        locals.var_alpha0_a_dn6 = assign16900_e24054_d_n6;
        locals.var_alpha0_a_dn7 = assign16900_e24054_d_n7;
        locals.var_alpha0_a_dn8 = assign16900_e24054_d_n8;
        locals.var_alpha0_a_dn9 = assign16900_e24054_d_n9;
        locals.var_alpha0_a_dn10 = assign16900_e24054_d_n10;
        locals.var_alpha0_a_dn11 = assign16900_e24054_d_n11;
        locals.var_alpha0_a_dn12 = assign16900_e24054_d_n12;
        locals.var_alpha0_a_dn13 = assign16900_e24054_d_n13;
        locals.var_alpha0_a_dn14 = assign16900_e24054_d_n14;
        locals.var_alpha0_a_rv = 0.0;

        let (assign16910_e24059, assign16910_e24059_d_n0, assign16910_e24059_d_n2, assign16910_e24059_d_n3, assign16910_e24059_d_n4, assign16910_e24059_d_n5, assign16910_e24059_d_n6, assign16910_e24059_d_n7, assign16910_e24059_d_n8, assign16910_e24059_d_n9, assign16910_e24059_d_n10, assign16910_e24059_d_n11, assign16910_e24059_d_n12, assign16910_e24059_d_n13, assign16910_e24059_d_n14,) = {
    if (locals.var_guard495 == 0.0) {
        (locals.var_beta0_t, locals.var_beta0_t_dn0, locals.var_beta0_t_dn2, locals.var_beta0_t_dn3, locals.var_beta0_t_dn4, locals.var_beta0_t_dn5, locals.var_beta0_t_dn6, locals.var_beta0_t_dn7, locals.var_beta0_t_dn8, locals.var_beta0_t_dn9, locals.var_beta0_t_dn10, locals.var_beta0_t_dn11, locals.var_beta0_t_dn12, locals.var_beta0_t_dn13, locals.var_beta0_t_dn14,)
    } else {
        (locals.var_beta0_a, locals.var_beta0_a_dn0, locals.var_beta0_a_dn2, locals.var_beta0_a_dn3, locals.var_beta0_a_dn4, locals.var_beta0_a_dn5, locals.var_beta0_a_dn6, locals.var_beta0_a_dn7, locals.var_beta0_a_dn8, locals.var_beta0_a_dn9, locals.var_beta0_a_dn10, locals.var_beta0_a_dn11, locals.var_beta0_a_dn12, locals.var_beta0_a_dn13, locals.var_beta0_a_dn14,)
    }
};
        locals.var_beta0_a = assign16910_e24059;
        locals.var_beta0_a_dn0 = assign16910_e24059_d_n0;
        locals.var_beta0_a_dn2 = assign16910_e24059_d_n2;
        locals.var_beta0_a_dn3 = assign16910_e24059_d_n3;
        locals.var_beta0_a_dn4 = assign16910_e24059_d_n4;
        locals.var_beta0_a_dn5 = assign16910_e24059_d_n5;
        locals.var_beta0_a_dn6 = assign16910_e24059_d_n6;
        locals.var_beta0_a_dn7 = assign16910_e24059_d_n7;
        locals.var_beta0_a_dn8 = assign16910_e24059_d_n8;
        locals.var_beta0_a_dn9 = assign16910_e24059_d_n9;
        locals.var_beta0_a_dn10 = assign16910_e24059_d_n10;
        locals.var_beta0_a_dn11 = assign16910_e24059_d_n11;
        locals.var_beta0_a_dn12 = assign16910_e24059_d_n12;
        locals.var_beta0_a_dn13 = assign16910_e24059_d_n13;
        locals.var_beta0_a_dn14 = assign16910_e24059_d_n14;
        locals.var_beta0_a_rv = 0.0;

        let assign16920_e24065: f64 = (locals.var_phist - locals.var_vbsx);
        let assign16920_e24067: f64 = (-2500.0);
        let assign16920_e24069: f64 = (assign16920_e24067 * 0.1);
        let assign16920_e24071: f64 = if ((0.05 == 0.0) && (assign16920_e24065 < assign16920_e24069)) { 1.0 } else { 0.0 };
        locals.var_guard496 = assign16920_e24071;
        locals.var_guard496_rv = 0.0;

        let (assign16930_e24084, assign16930_e24084_d_n0, assign16930_e24084_d_n2, assign16930_e24084_d_n3, assign16930_e24084_d_n4, assign16930_e24084_d_n5, assign16930_e24084_d_n6, assign16930_e24084_d_n7, assign16930_e24084_d_n8, assign16930_e24084_d_n9, assign16930_e24084_d_n10, assign16930_e24084_d_n11, assign16930_e24084_d_n12, assign16930_e24084_d_n13, assign16930_e24084_d_n14,) = {
    if (locals.var_guard496 != 0.0) {
        let assign16930_e24074: f64 = (-0.1);
        let assign16930_e24076: f64 = (assign16930_e24074 * 0.1);
        let assign16930_e24080: f64 = (locals.var_phist - locals.var_vbsx);
        let assign16930_e24081: f64 = (16.0 * assign16930_e24080);
        let assign16930_e24082: f64 = (assign16930_e24076 / assign16930_e24081);
        (assign16930_e24082, (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn0 - locals.var_vbsx_dn0))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn2 - locals.var_vbsx_dn2))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn12 - locals.var_vbsx_dn12))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn13 - locals.var_vbsx_dn13))) / (assign16930_e24081 * assign16930_e24081))), (-((assign16930_e24076 * (16.0 * (locals.var_phist_dn14 - locals.var_vbsx_dn14))) / (assign16930_e24081 * assign16930_e24081))),)
    } else {
        (locals.var_phistvbs, locals.var_phistvbs_dn0, locals.var_phistvbs_dn2, locals.var_phistvbs_dn3, locals.var_phistvbs_dn4, locals.var_phistvbs_dn5, locals.var_phistvbs_dn6, locals.var_phistvbs_dn7, locals.var_phistvbs_dn8, locals.var_phistvbs_dn9, locals.var_phistvbs_dn10, locals.var_phistvbs_dn11, locals.var_phistvbs_dn12, locals.var_phistvbs_dn13, locals.var_phistvbs_dn14,)
    }
};
        locals.var_phistvbs = assign16930_e24084;
        locals.var_phistvbs_dn0 = assign16930_e24084_d_n0;
        locals.var_phistvbs_dn2 = assign16930_e24084_d_n2;
        locals.var_phistvbs_dn3 = assign16930_e24084_d_n3;
        locals.var_phistvbs_dn4 = assign16930_e24084_d_n4;
        locals.var_phistvbs_dn5 = assign16930_e24084_d_n5;
        locals.var_phistvbs_dn6 = assign16930_e24084_d_n6;
        locals.var_phistvbs_dn7 = assign16930_e24084_d_n7;
        locals.var_phistvbs_dn8 = assign16930_e24084_d_n8;
        locals.var_phistvbs_dn9 = assign16930_e24084_d_n9;
        locals.var_phistvbs_dn10 = assign16930_e24084_d_n10;
        locals.var_phistvbs_dn11 = assign16930_e24084_d_n11;
        locals.var_phistvbs_dn12 = assign16930_e24084_d_n12;
        locals.var_phistvbs_dn13 = assign16930_e24084_d_n13;
        locals.var_phistvbs_dn14 = assign16930_e24084_d_n14;
        locals.var_phistvbs_rv = 0.0;

        let (assign16940_e24114, assign16940_e24114_d_n0, assign16940_e24114_d_n2, assign16940_e24114_d_n3, assign16940_e24114_d_n4, assign16940_e24114_d_n5, assign16940_e24114_d_n6, assign16940_e24114_d_n7, assign16940_e24114_d_n8, assign16940_e24114_d_n9, assign16940_e24114_d_n10, assign16940_e24114_d_n11, assign16940_e24114_d_n12, assign16940_e24114_d_n13, assign16940_e24114_d_n14,) = {
    if (locals.var_guard496 == 0.0) {
        let assign16940_e24090: f64 = (locals.var_phist - locals.var_vbsx);
        let assign16940_e24092: f64 = (assign16940_e24090 + 0.05);
        let assign16940_e24095: f64 = (locals.var_phist - locals.var_vbsx);
        let assign16940_e24097: f64 = (assign16940_e24095 - 0.05);
        let assign16940_e24100: f64 = (locals.var_phist - locals.var_vbsx);
        let assign16940_e24102: f64 = (assign16940_e24100 - 0.05);
        let assign16940_e24103: f64 = (assign16940_e24097 * assign16940_e24102);
        let assign16940_e24106: f64 = (0.25 * 0.1);
        let assign16940_e24108: f64 = (assign16940_e24106 * 0.1);
        let assign16940_e24109: f64 = (assign16940_e24103 + assign16940_e24108);
        let assign16940_e24110: f64 = (assign16940_e24109).sqrt();
        let assign16940_e24111: f64 = (assign16940_e24092 + assign16940_e24110);
        let assign16940_e24112: f64 = (0.5 * assign16940_e24111);
        (assign16940_e24112, (0.5 * ((locals.var_phist_dn0 - locals.var_vbsx_dn0) + ((((locals.var_phist_dn0 - locals.var_vbsx_dn0) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn0 - locals.var_vbsx_dn0))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn2 - locals.var_vbsx_dn2) + ((((locals.var_phist_dn2 - locals.var_vbsx_dn2) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn2 - locals.var_vbsx_dn2))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn3 - locals.var_vbsx_dn3) + ((((locals.var_phist_dn3 - locals.var_vbsx_dn3) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn4 - locals.var_vbsx_dn4) + ((((locals.var_phist_dn4 - locals.var_vbsx_dn4) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn5 - locals.var_vbsx_dn5) + ((((locals.var_phist_dn5 - locals.var_vbsx_dn5) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn6 - locals.var_vbsx_dn6) + ((((locals.var_phist_dn6 - locals.var_vbsx_dn6) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn7 - locals.var_vbsx_dn7) + ((((locals.var_phist_dn7 - locals.var_vbsx_dn7) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn8 - locals.var_vbsx_dn8) + ((((locals.var_phist_dn8 - locals.var_vbsx_dn8) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn9 - locals.var_vbsx_dn9) + ((((locals.var_phist_dn9 - locals.var_vbsx_dn9) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn10 - locals.var_vbsx_dn10) + ((((locals.var_phist_dn10 - locals.var_vbsx_dn10) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn11 - locals.var_vbsx_dn11) + ((((locals.var_phist_dn11 - locals.var_vbsx_dn11) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn12 - locals.var_vbsx_dn12) + ((((locals.var_phist_dn12 - locals.var_vbsx_dn12) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn12 - locals.var_vbsx_dn12))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn13 - locals.var_vbsx_dn13) + ((((locals.var_phist_dn13 - locals.var_vbsx_dn13) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn13 - locals.var_vbsx_dn13))) / (2.0 * assign16940_e24110)))), (0.5 * ((locals.var_phist_dn14 - locals.var_vbsx_dn14) + ((((locals.var_phist_dn14 - locals.var_vbsx_dn14) * assign16940_e24102) + (assign16940_e24097 * (locals.var_phist_dn14 - locals.var_vbsx_dn14))) / (2.0 * assign16940_e24110)))),)
    } else {
        (locals.var_phistvbs, locals.var_phistvbs_dn0, locals.var_phistvbs_dn2, locals.var_phistvbs_dn3, locals.var_phistvbs_dn4, locals.var_phistvbs_dn5, locals.var_phistvbs_dn6, locals.var_phistvbs_dn7, locals.var_phistvbs_dn8, locals.var_phistvbs_dn9, locals.var_phistvbs_dn10, locals.var_phistvbs_dn11, locals.var_phistvbs_dn12, locals.var_phistvbs_dn13, locals.var_phistvbs_dn14,)
    }
};
        locals.var_phistvbs = assign16940_e24114;
        locals.var_phistvbs_dn0 = assign16940_e24114_d_n0;
        locals.var_phistvbs_dn2 = assign16940_e24114_d_n2;
        locals.var_phistvbs_dn3 = assign16940_e24114_d_n3;
        locals.var_phistvbs_dn4 = assign16940_e24114_d_n4;
        locals.var_phistvbs_dn5 = assign16940_e24114_d_n5;
        locals.var_phistvbs_dn6 = assign16940_e24114_d_n6;
        locals.var_phistvbs_dn7 = assign16940_e24114_d_n7;
        locals.var_phistvbs_dn8 = assign16940_e24114_d_n8;
        locals.var_phistvbs_dn9 = assign16940_e24114_d_n9;
        locals.var_phistvbs_dn10 = assign16940_e24114_d_n10;
        locals.var_phistvbs_dn11 = assign16940_e24114_d_n11;
        locals.var_phistvbs_dn12 = assign16940_e24114_d_n12;
        locals.var_phistvbs_dn13 = assign16940_e24114_d_n13;
        locals.var_phistvbs_dn14 = assign16940_e24114_d_n14;
        locals.var_phistvbs_rv = 0.0;

        let assign16950_e24116: f64 = (locals.var_phistvbs).sqrt();
        locals.var_sqrtphistvbs = assign16950_e24116;
        locals.var_sqrtphistvbs_dn0 = (locals.var_phistvbs_dn0 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn2 = (locals.var_phistvbs_dn2 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn3 = (locals.var_phistvbs_dn3 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn4 = (locals.var_phistvbs_dn4 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn5 = (locals.var_phistvbs_dn5 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn6 = (locals.var_phistvbs_dn6 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn7 = (locals.var_phistvbs_dn7 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn8 = (locals.var_phistvbs_dn8 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn9 = (locals.var_phistvbs_dn9 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn10 = (locals.var_phistvbs_dn10 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn11 = (locals.var_phistvbs_dn11 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn12 = (locals.var_phistvbs_dn12 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn13 = (locals.var_phistvbs_dn13 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_dn14 = (locals.var_phistvbs_dn14 / (2.0 * assign16950_e24116));
        locals.var_sqrtphistvbs_rv = 0.0;

        let assign16960_e24119: f64 = (locals.var_t1dep * locals.var_sqrtphistvbs);
        locals.var_xdep = assign16960_e24119;
        locals.var_xdep_dn0 = ((locals.var_t1dep_dn0 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn0));
        locals.var_xdep_dn2 = ((locals.var_t1dep_dn2 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn2));
        locals.var_xdep_dn3 = ((locals.var_t1dep_dn3 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn3));
        locals.var_xdep_dn4 = ((locals.var_t1dep_dn4 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn4));
        locals.var_xdep_dn5 = ((locals.var_t1dep_dn5 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn5));
        locals.var_xdep_dn6 = ((locals.var_t1dep_dn6 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn6));
        locals.var_xdep_dn7 = ((locals.var_t1dep_dn7 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn7));
        locals.var_xdep_dn8 = ((locals.var_t1dep_dn8 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn8));
        locals.var_xdep_dn9 = ((locals.var_t1dep_dn9 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn9));
        locals.var_xdep_dn10 = ((locals.var_t1dep_dn10 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn10));
        locals.var_xdep_dn11 = ((locals.var_t1dep_dn11 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn11));
        locals.var_xdep_dn12 = ((locals.var_t1dep_dn12 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn12));
        locals.var_xdep_dn13 = ((locals.var_t1dep_dn13 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn13));
        locals.var_xdep_dn14 = ((locals.var_t1dep_dn14 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn14));
        locals.var_xdep_rv = 0.0;

        let assign16970_e24122: f64 = (locals.var_epssi / locals.var_xdep);
        locals.var_cdep = assign16970_e24122;
        locals.var_cdep_dn0 = (-((locals.var_epssi * locals.var_xdep_dn0) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn2 = (-((locals.var_epssi * locals.var_xdep_dn2) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn3 = (-((locals.var_epssi * locals.var_xdep_dn3) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn4 = (-((locals.var_epssi * locals.var_xdep_dn4) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn5 = (-((locals.var_epssi * locals.var_xdep_dn5) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn6 = (-((locals.var_epssi * locals.var_xdep_dn6) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn7 = (-((locals.var_epssi * locals.var_xdep_dn7) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn8 = (-((locals.var_epssi * locals.var_xdep_dn8) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn9 = (-((locals.var_epssi * locals.var_xdep_dn9) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn10 = (-((locals.var_epssi * locals.var_xdep_dn10) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn11 = (-((locals.var_epssi * locals.var_xdep_dn11) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn12 = (-((locals.var_epssi * locals.var_xdep_dn12) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn13 = (-((locals.var_epssi * locals.var_xdep_dn13) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_dn14 = (-((locals.var_epssi * locals.var_xdep_dn14) / (locals.var_xdep * locals.var_xdep)));
        locals.var_cdep_rv = 0.0;

        let assign16980_e24125: f64 = (locals.var_cit_i + locals.var_nfactor_t);
        let assign16980_e24128: f64 = (locals.var_cdscd_a * locals.var_vdsx);
        let assign16980_e24129: f64 = (assign16980_e24125 + assign16980_e24128);
        let assign16980_e24132: f64 = (locals.var_cdscb_i * locals.var_vbsx);
        let assign16980_e24133: f64 = (assign16980_e24129 - assign16980_e24132);
        locals.var_cdsc = assign16980_e24133;
        locals.var_cdsc_dn0 = ((locals.var_nfactor_t_dn0 + ((locals.var_cdscd_a_dn0 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn0))) - (locals.var_cdscb_i * locals.var_vbsx_dn0));
        locals.var_cdsc_dn2 = ((locals.var_nfactor_t_dn2 + ((locals.var_cdscd_a_dn2 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn2))) - (locals.var_cdscb_i * locals.var_vbsx_dn2));
        locals.var_cdsc_dn3 = ((locals.var_nfactor_t_dn3 + ((locals.var_cdscd_a_dn3 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn3))) - (locals.var_cdscb_i * locals.var_vbsx_dn3));
        locals.var_cdsc_dn4 = ((locals.var_nfactor_t_dn4 + ((locals.var_cdscd_a_dn4 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn4))) - (locals.var_cdscb_i * locals.var_vbsx_dn4));
        locals.var_cdsc_dn5 = ((locals.var_nfactor_t_dn5 + ((locals.var_cdscd_a_dn5 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn5))) - (locals.var_cdscb_i * locals.var_vbsx_dn5));
        locals.var_cdsc_dn6 = ((locals.var_nfactor_t_dn6 + ((locals.var_cdscd_a_dn6 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn6))) - (locals.var_cdscb_i * locals.var_vbsx_dn6));
        locals.var_cdsc_dn7 = ((locals.var_nfactor_t_dn7 + ((locals.var_cdscd_a_dn7 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn7))) - (locals.var_cdscb_i * locals.var_vbsx_dn7));
        locals.var_cdsc_dn8 = ((locals.var_nfactor_t_dn8 + ((locals.var_cdscd_a_dn8 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn8))) - (locals.var_cdscb_i * locals.var_vbsx_dn8));
        locals.var_cdsc_dn9 = ((locals.var_nfactor_t_dn9 + ((locals.var_cdscd_a_dn9 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn9))) - (locals.var_cdscb_i * locals.var_vbsx_dn9));
        locals.var_cdsc_dn10 = ((locals.var_nfactor_t_dn10 + ((locals.var_cdscd_a_dn10 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn10))) - (locals.var_cdscb_i * locals.var_vbsx_dn10));
        locals.var_cdsc_dn11 = ((locals.var_nfactor_t_dn11 + ((locals.var_cdscd_a_dn11 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn11))) - (locals.var_cdscb_i * locals.var_vbsx_dn11));
        locals.var_cdsc_dn12 = ((locals.var_nfactor_t_dn12 + ((locals.var_cdscd_a_dn12 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn12))) - (locals.var_cdscb_i * locals.var_vbsx_dn12));
        locals.var_cdsc_dn13 = ((locals.var_nfactor_t_dn13 + ((locals.var_cdscd_a_dn13 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn13))) - (locals.var_cdscb_i * locals.var_vbsx_dn13));
        locals.var_cdsc_dn14 = ((locals.var_nfactor_t_dn14 + ((locals.var_cdscd_a_dn14 * locals.var_vdsx) + (locals.var_cdscd_a * locals.var_vdsx_dn14))) - (locals.var_cdscb_i * locals.var_vbsx_dn14));
        locals.var_cdsc_rv = 0.0;

        let assign16990_e24137: f64 = (locals.var_cdsc / locals.var_cox);
        let assign16990_e24138: f64 = (1.0 + assign16990_e24137);
        locals.var_t1 = assign16990_e24138;
        locals.var_t1_dn0 = (locals.var_cdsc_dn0 / locals.var_cox);
        locals.var_t1_dn2 = (locals.var_cdsc_dn2 / locals.var_cox);
        locals.var_t1_dn3 = (locals.var_cdsc_dn3 / locals.var_cox);
        locals.var_t1_dn4 = (locals.var_cdsc_dn4 / locals.var_cox);
        locals.var_t1_dn5 = (locals.var_cdsc_dn5 / locals.var_cox);
        locals.var_t1_dn6 = (locals.var_cdsc_dn6 / locals.var_cox);
        locals.var_t1_dn7 = (locals.var_cdsc_dn7 / locals.var_cox);
        locals.var_t1_dn8 = (locals.var_cdsc_dn8 / locals.var_cox);
        locals.var_t1_dn9 = (locals.var_cdsc_dn9 / locals.var_cox);
        locals.var_t1_dn10 = (locals.var_cdsc_dn10 / locals.var_cox);
        locals.var_t1_dn11 = (locals.var_cdsc_dn11 / locals.var_cox);
        locals.var_t1_dn12 = (locals.var_cdsc_dn12 / locals.var_cox);
        locals.var_t1_dn13 = (locals.var_cdsc_dn13 / locals.var_cox);
        locals.var_t1_dn14 = (locals.var_cdsc_dn14 / locals.var_cox);
        locals.var_t1_rv = 0.0;

        let assign17000_e24144: f64 = (-2500.0);
        let assign17000_e24146: f64 = (assign17000_e24144 * 0.05);
        let assign17000_e24148: f64 = if ((1.0 == 0.0) && (locals.var_t1 < assign17000_e24146)) { 1.0 } else { 0.0 };
        locals.var_guard497 = assign17000_e24148;
        locals.var_guard497_rv = 0.0;

        let (assign17010_e24159, assign17010_e24159_d_n0, assign17010_e24159_d_n2, assign17010_e24159_d_n3, assign17010_e24159_d_n4, assign17010_e24159_d_n5, assign17010_e24159_d_n6, assign17010_e24159_d_n7, assign17010_e24159_d_n8, assign17010_e24159_d_n9, assign17010_e24159_d_n10, assign17010_e24159_d_n11, assign17010_e24159_d_n12, assign17010_e24159_d_n13, assign17010_e24159_d_n14,) = {
    if (locals.var_guard497 != 0.0) {
        let assign17010_e24151: f64 = (-0.05);
        let assign17010_e24153: f64 = (assign17010_e24151 * 0.05);
        let assign17010_e24156: f64 = (16.0 * locals.var_t1);
        let assign17010_e24157: f64 = (assign17010_e24153 / assign17010_e24156);
        (assign17010_e24157, (-((assign17010_e24153 * (16.0 * locals.var_t1_dn0)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn2)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn3)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn4)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn5)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn6)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn7)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn8)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn9)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn10)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn11)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn12)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn13)) / (assign17010_e24156 * assign17010_e24156))), (-((assign17010_e24153 * (16.0 * locals.var_t1_dn14)) / (assign17010_e24156 * assign17010_e24156))),)
    } else {
        (locals.var_n, locals.var_n_dn0, locals.var_n_dn2, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12, locals.var_n_dn13, locals.var_n_dn14,)
    }
};
        locals.var_n = assign17010_e24159;
        locals.var_n_dn0 = assign17010_e24159_d_n0;
        locals.var_n_dn2 = assign17010_e24159_d_n2;
        locals.var_n_dn3 = assign17010_e24159_d_n3;
        locals.var_n_dn4 = assign17010_e24159_d_n4;
        locals.var_n_dn5 = assign17010_e24159_d_n5;
        locals.var_n_dn6 = assign17010_e24159_d_n6;
        locals.var_n_dn7 = assign17010_e24159_d_n7;
        locals.var_n_dn8 = assign17010_e24159_d_n8;
        locals.var_n_dn9 = assign17010_e24159_d_n9;
        locals.var_n_dn10 = assign17010_e24159_d_n10;
        locals.var_n_dn11 = assign17010_e24159_d_n11;
        locals.var_n_dn12 = assign17010_e24159_d_n12;
        locals.var_n_dn13 = assign17010_e24159_d_n13;
        locals.var_n_dn14 = assign17010_e24159_d_n14;
        locals.var_n_rv = 0.0;

        let (assign17020_e24183, assign17020_e24183_d_n0, assign17020_e24183_d_n2, assign17020_e24183_d_n3, assign17020_e24183_d_n4, assign17020_e24183_d_n5, assign17020_e24183_d_n6, assign17020_e24183_d_n7, assign17020_e24183_d_n8, assign17020_e24183_d_n9, assign17020_e24183_d_n10, assign17020_e24183_d_n11, assign17020_e24183_d_n12, assign17020_e24183_d_n13, assign17020_e24183_d_n14,) = {
    if (locals.var_guard497 == 0.0) {
        let assign17020_e24165: f64 = (locals.var_t1 + 1.0);
        let assign17020_e24168: f64 = (locals.var_t1 - 1.0);
        let assign17020_e24171: f64 = (locals.var_t1 - 1.0);
        let assign17020_e24172: f64 = (assign17020_e24168 * assign17020_e24171);
        let assign17020_e24175: f64 = (0.25 * 0.05);
        let assign17020_e24177: f64 = (assign17020_e24175 * 0.05);
        let assign17020_e24178: f64 = (assign17020_e24172 + assign17020_e24177);
        let assign17020_e24179: f64 = (assign17020_e24178).sqrt();
        let assign17020_e24180: f64 = (assign17020_e24165 + assign17020_e24179);
        let assign17020_e24181: f64 = (0.5 * assign17020_e24180);
        (assign17020_e24181, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn0)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn2)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn3)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn4)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn5)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn6)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn7)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn8)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn9)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn10)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn11)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn12 + (((locals.var_t1_dn12 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn12)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn13)) / (2.0 * assign17020_e24179)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * assign17020_e24171) + (assign17020_e24168 * locals.var_t1_dn14)) / (2.0 * assign17020_e24179)))),)
    } else {
        (locals.var_n, locals.var_n_dn0, locals.var_n_dn2, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12, locals.var_n_dn13, locals.var_n_dn14,)
    }
};
        locals.var_n = assign17020_e24183;
        locals.var_n_dn0 = assign17020_e24183_d_n0;
        locals.var_n_dn2 = assign17020_e24183_d_n2;
        locals.var_n_dn3 = assign17020_e24183_d_n3;
        locals.var_n_dn4 = assign17020_e24183_d_n4;
        locals.var_n_dn5 = assign17020_e24183_d_n5;
        locals.var_n_dn6 = assign17020_e24183_d_n6;
        locals.var_n_dn7 = assign17020_e24183_d_n7;
        locals.var_n_dn8 = assign17020_e24183_d_n8;
        locals.var_n_dn9 = assign17020_e24183_d_n9;
        locals.var_n_dn10 = assign17020_e24183_d_n10;
        locals.var_n_dn11 = assign17020_e24183_d_n11;
        locals.var_n_dn12 = assign17020_e24183_d_n12;
        locals.var_n_dn13 = assign17020_e24183_d_n13;
        locals.var_n_dn14 = assign17020_e24183_d_n14;
        locals.var_n_rv = 0.0;

        let assign17030_e24186: f64 = (locals.var_n * locals.var_vt);
        locals.var_nvt = assign17030_e24186;
        locals.var_nvt_dn0 = (locals.var_n_dn0 * locals.var_vt);
        locals.var_nvt_dn2 = (locals.var_n_dn2 * locals.var_vt);
        locals.var_nvt_dn3 = (locals.var_n_dn3 * locals.var_vt);
        locals.var_nvt_dn4 = ((locals.var_n_dn4 * locals.var_vt) + (locals.var_n * locals.var_vt_dn4));
        locals.var_nvt_dn5 = (locals.var_n_dn5 * locals.var_vt);
        locals.var_nvt_dn6 = (locals.var_n_dn6 * locals.var_vt);
        locals.var_nvt_dn7 = (locals.var_n_dn7 * locals.var_vt);
        locals.var_nvt_dn8 = (locals.var_n_dn8 * locals.var_vt);
        locals.var_nvt_dn9 = (locals.var_n_dn9 * locals.var_vt);
        locals.var_nvt_dn10 = (locals.var_n_dn10 * locals.var_vt);
        locals.var_nvt_dn11 = (locals.var_n_dn11 * locals.var_vt);
        locals.var_nvt_dn12 = (locals.var_n_dn12 * locals.var_vt);
        locals.var_nvt_dn13 = (locals.var_n_dn13 * locals.var_vt);
        locals.var_nvt_dn14 = (locals.var_n_dn14 * locals.var_vt);
        locals.var_nvt_rv = 0.0;

        let assign17040_e24189: f64 = (1.0 / locals.var_nvt);
        locals.var_inv_nvt = assign17040_e24189;
        locals.var_inv_nvt_dn0 = (-(locals.var_nvt_dn0 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn2 = (-(locals.var_nvt_dn2 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn3 = (-(locals.var_nvt_dn3 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn4 = (-(locals.var_nvt_dn4 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn5 = (-(locals.var_nvt_dn5 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn6 = (-(locals.var_nvt_dn6 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn7 = (-(locals.var_nvt_dn7 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn8 = (-(locals.var_nvt_dn8 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn9 = (-(locals.var_nvt_dn9 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn10 = (-(locals.var_nvt_dn10 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn11 = (-(locals.var_nvt_dn11 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn12 = (-(locals.var_nvt_dn12 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn13 = (-(locals.var_nvt_dn13 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_dn14 = (-(locals.var_nvt_dn14 / (locals.var_nvt * locals.var_nvt)));
        locals.var_inv_nvt_rv = 0.0;

        let assign17050_e24193: f64 = (locals.var_etab_i * locals.var_vbsx);
        let assign17050_e24194: f64 = (locals.var_eta0_a + assign17050_e24193);
        let assign17050_e24195: f64 = (-assign17050_e24194);
        let assign17050_e24197: f64 = (assign17050_e24195 * locals.var_vdsx);
        locals.var_dvth_dibl = assign17050_e24197;
        locals.var_dvth_dibl_dn0 = (((-(locals.var_eta0_a_dn0 + (locals.var_etab_i * locals.var_vbsx_dn0))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn0));
        locals.var_dvth_dibl_dn2 = (((-(locals.var_eta0_a_dn2 + (locals.var_etab_i * locals.var_vbsx_dn2))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn2));
        locals.var_dvth_dibl_dn3 = (((-(locals.var_eta0_a_dn3 + (locals.var_etab_i * locals.var_vbsx_dn3))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn3));
        locals.var_dvth_dibl_dn4 = (((-(locals.var_eta0_a_dn4 + (locals.var_etab_i * locals.var_vbsx_dn4))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn4));
        locals.var_dvth_dibl_dn5 = (((-(locals.var_eta0_a_dn5 + (locals.var_etab_i * locals.var_vbsx_dn5))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn5));
        locals.var_dvth_dibl_dn6 = (((-(locals.var_eta0_a_dn6 + (locals.var_etab_i * locals.var_vbsx_dn6))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn6));
        locals.var_dvth_dibl_dn7 = (((-(locals.var_eta0_a_dn7 + (locals.var_etab_i * locals.var_vbsx_dn7))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn7));
        locals.var_dvth_dibl_dn8 = (((-(locals.var_eta0_a_dn8 + (locals.var_etab_i * locals.var_vbsx_dn8))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn8));
        locals.var_dvth_dibl_dn9 = (((-(locals.var_eta0_a_dn9 + (locals.var_etab_i * locals.var_vbsx_dn9))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn9));
        locals.var_dvth_dibl_dn10 = (((-(locals.var_eta0_a_dn10 + (locals.var_etab_i * locals.var_vbsx_dn10))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn10));
        locals.var_dvth_dibl_dn11 = (((-(locals.var_eta0_a_dn11 + (locals.var_etab_i * locals.var_vbsx_dn11))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn11));
        locals.var_dvth_dibl_dn12 = (((-(locals.var_eta0_a_dn12 + (locals.var_etab_i * locals.var_vbsx_dn12))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn12));
        locals.var_dvth_dibl_dn13 = (((-(locals.var_eta0_a_dn13 + (locals.var_etab_i * locals.var_vbsx_dn13))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn13));
        locals.var_dvth_dibl_dn14 = (((-(locals.var_eta0_a_dn14 + (locals.var_etab_i * locals.var_vbsx_dn14))) * locals.var_vdsx) + (assign17050_e24195 * locals.var_vdsx_dn14));
        locals.var_dvth_dibl_rv = 0.0;

        let assign17060_e24201: f64 = locals.var_dvth_dibl;
        let assign17060_e24204: f64 = locals.var_dvth_dibl;
        let assign17060_e24207: f64 = locals.var_dvth_dibl;
        let assign17060_e24208: f64 = (assign17060_e24204 * assign17060_e24207);
        let assign17060_e24211: f64 = (0.25 * 0.005);
        let assign17060_e24213: f64 = (assign17060_e24211 * 0.005);
        let assign17060_e24214: f64 = (assign17060_e24208 + assign17060_e24213);
        let assign17060_e24215: f64 = (assign17060_e24214).sqrt();
        let assign17060_e24216: f64 = (assign17060_e24201 - assign17060_e24215);
        let assign17060_e24217: f64 = (0.5 * assign17060_e24216);
        let assign17060_e24220: f64 = (0.25 * 0.005);
        let assign17060_e24221: f64 = (assign17060_e24217 + assign17060_e24220);
        locals.var_dvth_dibl = assign17060_e24221;
        locals.var_dvth_dibl_dn0 = (0.5 * (locals.var_dvth_dibl_dn0 - (((locals.var_dvth_dibl_dn0 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn0)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn2 = (0.5 * (locals.var_dvth_dibl_dn2 - (((locals.var_dvth_dibl_dn2 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn2)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn3 = (0.5 * (locals.var_dvth_dibl_dn3 - (((locals.var_dvth_dibl_dn3 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn3)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn4 = (0.5 * (locals.var_dvth_dibl_dn4 - (((locals.var_dvth_dibl_dn4 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn4)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn5 = (0.5 * (locals.var_dvth_dibl_dn5 - (((locals.var_dvth_dibl_dn5 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn5)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn6 = (0.5 * (locals.var_dvth_dibl_dn6 - (((locals.var_dvth_dibl_dn6 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn6)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn7 = (0.5 * (locals.var_dvth_dibl_dn7 - (((locals.var_dvth_dibl_dn7 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn7)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn8 = (0.5 * (locals.var_dvth_dibl_dn8 - (((locals.var_dvth_dibl_dn8 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn8)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn9 = (0.5 * (locals.var_dvth_dibl_dn9 - (((locals.var_dvth_dibl_dn9 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn9)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn10 = (0.5 * (locals.var_dvth_dibl_dn10 - (((locals.var_dvth_dibl_dn10 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn10)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn11 = (0.5 * (locals.var_dvth_dibl_dn11 - (((locals.var_dvth_dibl_dn11 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn11)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn12 = (0.5 * (locals.var_dvth_dibl_dn12 - (((locals.var_dvth_dibl_dn12 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn12)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn13 = (0.5 * (locals.var_dvth_dibl_dn13 - (((locals.var_dvth_dibl_dn13 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn13)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_dn14 = (0.5 * (locals.var_dvth_dibl_dn14 - (((locals.var_dvth_dibl_dn14 * assign17060_e24207) + (assign17060_e24204 * locals.var_dvth_dibl_dn14)) / (2.0 * assign17060_e24215))));
        locals.var_dvth_dibl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign17070_e24225: f64 = (p.p869 / locals.var_leff);
        let assign17070_e24226: f64 = (locals.var_kt1_i + assign17070_e24225);
        let assign17070_e24229: f64 = (locals.var_kt2_i * locals.var_vbsx);
        let assign17070_e24230: f64 = (assign17070_e24226 + assign17070_e24229);
        let assign17070_e24233: f64 = (locals.var_tratio).powf(p.p868);
        let assign17070_e24235: f64 = (assign17070_e24233 - 1.0);
        let assign17070_e24236: f64 = (assign17070_e24230 * assign17070_e24235);
        locals.var_dvth_temp = assign17070_e24236;
        locals.var_dvth_temp_dn0 = ((locals.var_kt2_i * locals.var_vbsx_dn0) * assign17070_e24235);
        locals.var_dvth_temp_dn2 = ((locals.var_kt2_i * locals.var_vbsx_dn2) * assign17070_e24235);
        locals.var_dvth_temp_dn3 = ((locals.var_kt2_i * locals.var_vbsx_dn3) * assign17070_e24235);
        locals.var_dvth_temp_dn4 = (((locals.var_kt2_i * locals.var_vbsx_dn4) * assign17070_e24235) + (assign17070_e24230 * if 0.0 == 0.0 && ((p.p868) as f64).is_finite() && ((p.p868) as f64).fract() == 0.0 { if p.p868 == 0.0 { 0.0 } else { (p.p868 * ((locals.var_tratio).powf(p.p868 - 1.0) * locals.var_tratio_dn4)) } } else { (assign17070_e24233 * (p.p868 * (locals.var_tratio_dn4 / locals.var_tratio))) }));
        locals.var_dvth_temp_dn5 = ((locals.var_kt2_i * locals.var_vbsx_dn5) * assign17070_e24235);
        locals.var_dvth_temp_dn6 = ((locals.var_kt2_i * locals.var_vbsx_dn6) * assign17070_e24235);
        locals.var_dvth_temp_dn7 = ((locals.var_kt2_i * locals.var_vbsx_dn7) * assign17070_e24235);
        locals.var_dvth_temp_dn8 = ((locals.var_kt2_i * locals.var_vbsx_dn8) * assign17070_e24235);
        locals.var_dvth_temp_dn9 = ((locals.var_kt2_i * locals.var_vbsx_dn9) * assign17070_e24235);
        locals.var_dvth_temp_dn10 = ((locals.var_kt2_i * locals.var_vbsx_dn10) * assign17070_e24235);
        locals.var_dvth_temp_dn11 = ((locals.var_kt2_i * locals.var_vbsx_dn11) * assign17070_e24235);
        locals.var_dvth_temp_dn12 = ((locals.var_kt2_i * locals.var_vbsx_dn12) * assign17070_e24235);
        locals.var_dvth_temp_dn13 = ((locals.var_kt2_i * locals.var_vbsx_dn13) * assign17070_e24235);
        locals.var_dvth_temp_dn14 = ((locals.var_kt2_i * locals.var_vbsx_dn14) * assign17070_e24235);
        locals.var_dvth_temp_rv = 0.0;

        let assign17080_e24239: f64 = if locals.var_dvtp0_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign17080_e24239;
        locals.var_guard498_rv = 0.0;

        let (assign17090_e24246, assign17090_e24246_d_n0, assign17090_e24246_d_n2, assign17090_e24246_d_n3, assign17090_e24246_d_n4, assign17090_e24246_d_n5, assign17090_e24246_d_n6, assign17090_e24246_d_n7, assign17090_e24246_d_n8, assign17090_e24246_d_n9, assign17090_e24246_d_n10, assign17090_e24246_d_n11, assign17090_e24246_d_n12, assign17090_e24246_d_n13, assign17090_e24246_d_n14,) = {
    if (locals.var_guard498 != 0.0) {
        let assign17090_e24242: f64 = (-locals.var_dvtp1_i);
        let assign17090_e24244: f64 = (assign17090_e24242 * locals.var_vdsx);
        (assign17090_e24244, (assign17090_e24242 * locals.var_vdsx_dn0), (assign17090_e24242 * locals.var_vdsx_dn2), (assign17090_e24242 * locals.var_vdsx_dn3), (assign17090_e24242 * locals.var_vdsx_dn4), (assign17090_e24242 * locals.var_vdsx_dn5), (assign17090_e24242 * locals.var_vdsx_dn6), (assign17090_e24242 * locals.var_vdsx_dn7), (assign17090_e24242 * locals.var_vdsx_dn8), (assign17090_e24242 * locals.var_vdsx_dn9), (assign17090_e24242 * locals.var_vdsx_dn10), (assign17090_e24242 * locals.var_vdsx_dn11), (assign17090_e24242 * locals.var_vdsx_dn12), (assign17090_e24242 * locals.var_vdsx_dn13), (assign17090_e24242 * locals.var_vdsx_dn14),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign17090_e24246;
        locals.var_t0_dn0 = assign17090_e24246_d_n0;
        locals.var_t0_dn2 = assign17090_e24246_d_n2;
        locals.var_t0_dn3 = assign17090_e24246_d_n3;
        locals.var_t0_dn4 = assign17090_e24246_d_n4;
        locals.var_t0_dn5 = assign17090_e24246_d_n5;
        locals.var_t0_dn6 = assign17090_e24246_d_n6;
        locals.var_t0_dn7 = assign17090_e24246_d_n7;
        locals.var_t0_dn8 = assign17090_e24246_d_n8;
        locals.var_t0_dn9 = assign17090_e24246_d_n9;
        locals.var_t0_dn10 = assign17090_e24246_d_n10;
        locals.var_t0_dn11 = assign17090_e24246_d_n11;
        locals.var_t0_dn12 = assign17090_e24246_d_n12;
        locals.var_t0_dn13 = assign17090_e24246_d_n13;
        locals.var_t0_dn14 = assign17090_e24246_d_n14;
        locals.var_t0_rv = 0.0;

        let assign17100_e24249: f64 = (-80.0);
        let assign17100_e24250: f64 = if locals.var_t0 < assign17100_e24249 { 1.0 } else { 0.0 };
        locals.var_guard499 = assign17100_e24250;
        locals.var_guard499_rv = 0.0;

        let (assign17110_e24256, assign17110_e24256_d_n0, assign17110_e24256_d_n2, assign17110_e24256_d_n3, assign17110_e24256_d_n4, assign17110_e24256_d_n5, assign17110_e24256_d_n6, assign17110_e24256_d_n7, assign17110_e24256_d_n8, assign17110_e24256_d_n9, assign17110_e24256_d_n10, assign17110_e24256_d_n11, assign17110_e24256_d_n12, assign17110_e24256_d_n13, assign17110_e24256_d_n14,) = {
    if ((locals.var_guard498 != 0.0) && (locals.var_guard499 != 0.0)) {
        (1.804851387e-35, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17110_e24256;
        locals.var_t2_dn0 = assign17110_e24256_d_n0;
        locals.var_t2_dn2 = assign17110_e24256_d_n2;
        locals.var_t2_dn3 = assign17110_e24256_d_n3;
        locals.var_t2_dn4 = assign17110_e24256_d_n4;
        locals.var_t2_dn5 = assign17110_e24256_d_n5;
        locals.var_t2_dn6 = assign17110_e24256_d_n6;
        locals.var_t2_dn7 = assign17110_e24256_d_n7;
        locals.var_t2_dn8 = assign17110_e24256_d_n8;
        locals.var_t2_dn9 = assign17110_e24256_d_n9;
        locals.var_t2_dn10 = assign17110_e24256_d_n10;
        locals.var_t2_dn11 = assign17110_e24256_d_n11;
        locals.var_t2_dn12 = assign17110_e24256_d_n12;
        locals.var_t2_dn13 = assign17110_e24256_d_n13;
        locals.var_t2_dn14 = assign17110_e24256_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign17120_e24264, assign17120_e24264_d_n0, assign17120_e24264_d_n2, assign17120_e24264_d_n3, assign17120_e24264_d_n4, assign17120_e24264_d_n5, assign17120_e24264_d_n6, assign17120_e24264_d_n7, assign17120_e24264_d_n8, assign17120_e24264_d_n9, assign17120_e24264_d_n10, assign17120_e24264_d_n11, assign17120_e24264_d_n12, assign17120_e24264_d_n13, assign17120_e24264_d_n14,) = {
    if ((locals.var_guard498 != 0.0) && (locals.var_guard499 == 0.0)) {
        let assign17120_e24262: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign17120_e24262, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn12), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign17120_e24264;
        locals.var_t2_dn0 = assign17120_e24264_d_n0;
        locals.var_t2_dn2 = assign17120_e24264_d_n2;
        locals.var_t2_dn3 = assign17120_e24264_d_n3;
        locals.var_t2_dn4 = assign17120_e24264_d_n4;
        locals.var_t2_dn5 = assign17120_e24264_d_n5;
        locals.var_t2_dn6 = assign17120_e24264_d_n6;
        locals.var_t2_dn7 = assign17120_e24264_d_n7;
        locals.var_t2_dn8 = assign17120_e24264_d_n8;
        locals.var_t2_dn9 = assign17120_e24264_d_n9;
        locals.var_t2_dn10 = assign17120_e24264_d_n10;
        locals.var_t2_dn11 = assign17120_e24264_d_n11;
        locals.var_t2_dn12 = assign17120_e24264_d_n12;
        locals.var_t2_dn13 = assign17120_e24264_d_n13;
        locals.var_t2_dn14 = assign17120_e24264_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign17130_e24274, assign17130_e24274_d_n0, assign17130_e24274_d_n2, assign17130_e24274_d_n3, assign17130_e24274_d_n4, assign17130_e24274_d_n5, assign17130_e24274_d_n6, assign17130_e24274_d_n7, assign17130_e24274_d_n8, assign17130_e24274_d_n9, assign17130_e24274_d_n10, assign17130_e24274_d_n11, assign17130_e24274_d_n12, assign17130_e24274_d_n13, assign17130_e24274_d_n14,) = {
    if (locals.var_guard498 != 0.0) {
        let assign17130_e24270: f64 = (1.0 + locals.var_t2);
        let assign17130_e24271: f64 = (locals.var_dvtp0_i * assign17130_e24270);
        let assign17130_e24272: f64 = (locals.var_leff + assign17130_e24271);
        (assign17130_e24272, (locals.var_dvtp0_i * locals.var_t2_dn0), (locals.var_dvtp0_i * locals.var_t2_dn2), (locals.var_dvtp0_i * locals.var_t2_dn3), (locals.var_dvtp0_i * locals.var_t2_dn4), (locals.var_dvtp0_i * locals.var_t2_dn5), (locals.var_dvtp0_i * locals.var_t2_dn6), (locals.var_dvtp0_i * locals.var_t2_dn7), (locals.var_dvtp0_i * locals.var_t2_dn8), (locals.var_dvtp0_i * locals.var_t2_dn9), (locals.var_dvtp0_i * locals.var_t2_dn10), (locals.var_dvtp0_i * locals.var_t2_dn11), (locals.var_dvtp0_i * locals.var_t2_dn12), (locals.var_dvtp0_i * locals.var_t2_dn13), (locals.var_dvtp0_i * locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign17130_e24274;
        locals.var_t3_dn0 = assign17130_e24274_d_n0;
        locals.var_t3_dn2 = assign17130_e24274_d_n2;
        locals.var_t3_dn3 = assign17130_e24274_d_n3;
        locals.var_t3_dn4 = assign17130_e24274_d_n4;
        locals.var_t3_dn5 = assign17130_e24274_d_n5;
        locals.var_t3_dn6 = assign17130_e24274_d_n6;
        locals.var_t3_dn7 = assign17130_e24274_d_n7;
        locals.var_t3_dn8 = assign17130_e24274_d_n8;
        locals.var_t3_dn9 = assign17130_e24274_d_n9;
        locals.var_t3_dn10 = assign17130_e24274_d_n10;
        locals.var_t3_dn11 = assign17130_e24274_d_n11;
        locals.var_t3_dn12 = assign17130_e24274_d_n12;
        locals.var_t3_dn13 = assign17130_e24274_d_n13;
        locals.var_t3_dn14 = assign17130_e24274_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign17140_e24286, assign17140_e24286_d_n0, assign17140_e24286_d_n2, assign17140_e24286_d_n3, assign17140_e24286_d_n4, assign17140_e24286_d_n5, assign17140_e24286_d_n6, assign17140_e24286_d_n7, assign17140_e24286_d_n8, assign17140_e24286_d_n9, assign17140_e24286_d_n10, assign17140_e24286_d_n11, assign17140_e24286_d_n12, assign17140_e24286_d_n13, assign17140_e24286_d_n14,) = {
    if (locals.var_guard498 != 0.0) {
        let assign17140_e24277: f64 = (-locals.var_nvt);
        let assign17140_e24280: f64 = (locals.var_leff / locals.var_t3);
        let assign17140_e24282: f64 = (assign17140_e24280).max(1e-38);
        let assign17140_e24283: f64 = (assign17140_e24282).ln();
        let assign17140_e24284: f64 = (assign17140_e24277 * assign17140_e24283);
        (assign17140_e24284, (((-locals.var_nvt_dn0) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn2) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn3) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn4) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn5) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn6) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn7) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn8) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn9) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn10) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn11) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn12) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn13) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))), (((-locals.var_nvt_dn14) * assign17140_e24283) + (assign17140_e24277 * (if assign17140_e24280 >= 1e-38 { (-((locals.var_leff * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) } else { 0.0 } / assign17140_e24282))),)
    } else {
        (locals.var_dvth_ldop, locals.var_dvth_ldop_dn0, locals.var_dvth_ldop_dn2, locals.var_dvth_ldop_dn3, locals.var_dvth_ldop_dn4, locals.var_dvth_ldop_dn5, locals.var_dvth_ldop_dn6, locals.var_dvth_ldop_dn7, locals.var_dvth_ldop_dn8, locals.var_dvth_ldop_dn9, locals.var_dvth_ldop_dn10, locals.var_dvth_ldop_dn11, locals.var_dvth_ldop_dn12, locals.var_dvth_ldop_dn13, locals.var_dvth_ldop_dn14,)
    }
};
        locals.var_dvth_ldop = assign17140_e24286;
        locals.var_dvth_ldop_dn0 = assign17140_e24286_d_n0;
        locals.var_dvth_ldop_dn2 = assign17140_e24286_d_n2;
        locals.var_dvth_ldop_dn3 = assign17140_e24286_d_n3;
        locals.var_dvth_ldop_dn4 = assign17140_e24286_d_n4;
        locals.var_dvth_ldop_dn5 = assign17140_e24286_d_n5;
        locals.var_dvth_ldop_dn6 = assign17140_e24286_d_n6;
        locals.var_dvth_ldop_dn7 = assign17140_e24286_d_n7;
        locals.var_dvth_ldop_dn8 = assign17140_e24286_d_n8;
        locals.var_dvth_ldop_dn9 = assign17140_e24286_d_n9;
        locals.var_dvth_ldop_dn10 = assign17140_e24286_d_n10;
        locals.var_dvth_ldop_dn11 = assign17140_e24286_d_n11;
        locals.var_dvth_ldop_dn12 = assign17140_e24286_d_n12;
        locals.var_dvth_ldop_dn13 = assign17140_e24286_d_n13;
        locals.var_dvth_ldop_dn14 = assign17140_e24286_d_n14;
        locals.var_dvth_ldop_rv = 0.0;

        let (assign17150_e24291, assign17150_e24291_d_n0, assign17150_e24291_d_n2, assign17150_e24291_d_n3, assign17150_e24291_d_n4, assign17150_e24291_d_n5, assign17150_e24291_d_n6, assign17150_e24291_d_n7, assign17150_e24291_d_n8, assign17150_e24291_d_n9, assign17150_e24291_d_n10, assign17150_e24291_d_n11, assign17150_e24291_d_n12, assign17150_e24291_d_n13, assign17150_e24291_d_n14,) = {
    if (locals.var_guard498 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvth_ldop, locals.var_dvth_ldop_dn0, locals.var_dvth_ldop_dn2, locals.var_dvth_ldop_dn3, locals.var_dvth_ldop_dn4, locals.var_dvth_ldop_dn5, locals.var_dvth_ldop_dn6, locals.var_dvth_ldop_dn7, locals.var_dvth_ldop_dn8, locals.var_dvth_ldop_dn9, locals.var_dvth_ldop_dn10, locals.var_dvth_ldop_dn11, locals.var_dvth_ldop_dn12, locals.var_dvth_ldop_dn13, locals.var_dvth_ldop_dn14,)
    }
};
        locals.var_dvth_ldop = assign17150_e24291;
        locals.var_dvth_ldop_dn0 = assign17150_e24291_d_n0;
        locals.var_dvth_ldop_dn2 = assign17150_e24291_d_n2;
        locals.var_dvth_ldop_dn3 = assign17150_e24291_d_n3;
        locals.var_dvth_ldop_dn4 = assign17150_e24291_d_n4;
        locals.var_dvth_ldop_dn5 = assign17150_e24291_d_n5;
        locals.var_dvth_ldop_dn6 = assign17150_e24291_d_n6;
        locals.var_dvth_ldop_dn7 = assign17150_e24291_d_n7;
        locals.var_dvth_ldop_dn8 = assign17150_e24291_d_n8;
        locals.var_dvth_ldop_dn9 = assign17150_e24291_d_n9;
        locals.var_dvth_ldop_dn10 = assign17150_e24291_d_n10;
        locals.var_dvth_ldop_dn11 = assign17150_e24291_d_n11;
        locals.var_dvth_ldop_dn12 = assign17150_e24291_d_n12;
        locals.var_dvth_ldop_dn13 = assign17150_e24291_d_n13;
        locals.var_dvth_ldop_dn14 = assign17150_e24291_d_n14;
        locals.var_dvth_ldop_rv = 0.0;

        let assign17160_e24296: f64 = (locals.var_leff).powf(locals.var_dvtp3_i);
        let assign17160_e24297: f64 = (locals.var_dvtp2_i / assign17160_e24296);
        let assign17160_e24298: f64 = (locals.var_dvtp5_i + assign17160_e24297);
        locals.var_t4 = assign17160_e24298;
        locals.var_t4_dn0 = 0.0;
        locals.var_t4_dn2 = 0.0;
        locals.var_t4_dn3 = 0.0;
        locals.var_t4_dn4 = 0.0;
        locals.var_t4_dn5 = 0.0;
        locals.var_t4_dn6 = 0.0;
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn11 = 0.0;
        locals.var_t4_dn12 = 0.0;
        locals.var_t4_dn13 = 0.0;
        locals.var_t4_dn14 = 0.0;
        locals.var_t4_rv = 0.0;

        let assign17170_e24303: f64 = (locals.var_dvtp4_i * locals.var_vdsx);
        let assign17170_e24304: f64 = (assign17170_e24303).tanh();
        let assign17170_e24305: f64 = (locals.var_t4 * assign17170_e24304);
        let assign17170_e24306: f64 = (locals.var_dvth_ldop - assign17170_e24305);
        locals.var_dvth_ldop = assign17170_e24306;
        locals.var_dvth_ldop_dn0 = (locals.var_dvth_ldop_dn0 - ((locals.var_t4_dn0 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn0) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn2 = (locals.var_dvth_ldop_dn2 - ((locals.var_t4_dn2 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn2) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn3 = (locals.var_dvth_ldop_dn3 - ((locals.var_t4_dn3 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn3) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn4 = (locals.var_dvth_ldop_dn4 - ((locals.var_t4_dn4 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn4) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn5 = (locals.var_dvth_ldop_dn5 - ((locals.var_t4_dn5 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn5) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn6 = (locals.var_dvth_ldop_dn6 - ((locals.var_t4_dn6 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn6) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn7 = (locals.var_dvth_ldop_dn7 - ((locals.var_t4_dn7 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn7) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn8 = (locals.var_dvth_ldop_dn8 - ((locals.var_t4_dn8 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn8) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn9 = (locals.var_dvth_ldop_dn9 - ((locals.var_t4_dn9 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn9) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn10 = (locals.var_dvth_ldop_dn10 - ((locals.var_t4_dn10 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn10) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn11 = (locals.var_dvth_ldop_dn11 - ((locals.var_t4_dn11 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn11) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn12 = (locals.var_dvth_ldop_dn12 - ((locals.var_t4_dn12 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn12) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn13 = (locals.var_dvth_ldop_dn13 - ((locals.var_t4_dn13 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn13) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_dn14 = (locals.var_dvth_ldop_dn14 - ((locals.var_t4_dn14 * assign17170_e24304) + (locals.var_t4 * ((locals.var_dvtp4_i * locals.var_vdsx_dn14) / ((assign17170_e24303).cosh() * (assign17170_e24303).cosh())))));
        locals.var_dvth_ldop_rv = 0.0;

        let assign17180_e24309: f64 = (locals.var_vfb_i + p.p35);
        locals.var_vfb_i = assign17180_e24309;
        locals.var_vfb_i_dn0 = locals.var_vfb_i_dn0;
        locals.var_vfb_i_dn2 = locals.var_vfb_i_dn2;
        locals.var_vfb_i_dn3 = locals.var_vfb_i_dn3;
        locals.var_vfb_i_dn4 = locals.var_vfb_i_dn4;
        locals.var_vfb_i_dn5 = locals.var_vfb_i_dn5;
        locals.var_vfb_i_dn6 = locals.var_vfb_i_dn6;
        locals.var_vfb_i_dn7 = locals.var_vfb_i_dn7;
        locals.var_vfb_i_dn8 = locals.var_vfb_i_dn8;
        locals.var_vfb_i_dn9 = locals.var_vfb_i_dn9;
        locals.var_vfb_i_dn10 = locals.var_vfb_i_dn10;
        locals.var_vfb_i_dn11 = locals.var_vfb_i_dn11;
        locals.var_vfb_i_dn12 = locals.var_vfb_i_dn12;
        locals.var_vfb_i_dn13 = locals.var_vfb_i_dn13;
        locals.var_vfb_i_dn14 = locals.var_vfb_i_dn14;
        locals.var_vfb_i_rv = 0.0;

        let assign17190_e24312: f64 = (locals.var_vg * locals.var_inv_nvt);
        locals.var_vg_1 = assign17190_e24312;
        locals.var_vg_1_dn0 = (locals.var_vg * locals.var_inv_nvt_dn0);
        locals.var_vg_1_dn2 = (locals.var_vg * locals.var_inv_nvt_dn2);
        locals.var_vg_1_dn3 = (locals.var_vg * locals.var_inv_nvt_dn3);
        locals.var_vg_1_dn4 = (locals.var_vg * locals.var_inv_nvt_dn4);
        locals.var_vg_1_dn5 = (locals.var_vg * locals.var_inv_nvt_dn5);
        locals.var_vg_1_dn6 = (locals.var_vg * locals.var_inv_nvt_dn6);
        locals.var_vg_1_dn7 = (locals.var_vg * locals.var_inv_nvt_dn7);
        locals.var_vg_1_dn8 = (locals.var_vg * locals.var_inv_nvt_dn8);
        locals.var_vg_1_dn9 = ((locals.var_vg_dn9 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn9));
        locals.var_vg_1_dn10 = (locals.var_vg * locals.var_inv_nvt_dn10);
        locals.var_vg_1_dn11 = ((locals.var_vg_dn11 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn11));
        locals.var_vg_1_dn12 = (locals.var_vg * locals.var_inv_nvt_dn12);
        locals.var_vg_1_dn13 = (locals.var_vg * locals.var_inv_nvt_dn13);
        locals.var_vg_1_dn14 = (locals.var_vg * locals.var_inv_nvt_dn14);
        locals.var_vg_1_rv = 0.0;

        let assign17200_e24315: f64 = (locals.var_vs * locals.var_inv_nvt);
        locals.var_vs_1 = assign17200_e24315;
        locals.var_vs_1_dn0 = (locals.var_vs * locals.var_inv_nvt_dn0);
        locals.var_vs_1_dn2 = (locals.var_vs * locals.var_inv_nvt_dn2);
        locals.var_vs_1_dn3 = (locals.var_vs * locals.var_inv_nvt_dn3);
        locals.var_vs_1_dn4 = (locals.var_vs * locals.var_inv_nvt_dn4);
        locals.var_vs_1_dn5 = ((locals.var_vs_dn5 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn5));
        locals.var_vs_1_dn6 = (locals.var_vs * locals.var_inv_nvt_dn6);
        locals.var_vs_1_dn7 = ((locals.var_vs_dn7 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn7));
        locals.var_vs_1_dn8 = (locals.var_vs * locals.var_inv_nvt_dn8);
        locals.var_vs_1_dn9 = (locals.var_vs * locals.var_inv_nvt_dn9);
        locals.var_vs_1_dn10 = (locals.var_vs * locals.var_inv_nvt_dn10);
        locals.var_vs_1_dn11 = ((locals.var_vs_dn11 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn11));
        locals.var_vs_1_dn12 = (locals.var_vs * locals.var_inv_nvt_dn12);
        locals.var_vs_1_dn13 = (locals.var_vs * locals.var_inv_nvt_dn13);
        locals.var_vs_1_dn14 = (locals.var_vs * locals.var_inv_nvt_dn14);
        locals.var_vs_1_rv = 0.0;

        let assign17210_e24318: f64 = (locals.var_vfb_i * locals.var_inv_nvt);
        locals.var_vfb = assign17210_e24318;
        locals.var_vfb_dn0 = ((locals.var_vfb_i_dn0 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn0));
        locals.var_vfb_dn2 = ((locals.var_vfb_i_dn2 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn2));
        locals.var_vfb_dn3 = ((locals.var_vfb_i_dn3 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn3));
        locals.var_vfb_dn4 = ((locals.var_vfb_i_dn4 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn4));
        locals.var_vfb_dn5 = ((locals.var_vfb_i_dn5 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn5));
        locals.var_vfb_dn6 = ((locals.var_vfb_i_dn6 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn6));
        locals.var_vfb_dn7 = ((locals.var_vfb_i_dn7 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn7));
        locals.var_vfb_dn8 = ((locals.var_vfb_i_dn8 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn8));
        locals.var_vfb_dn9 = ((locals.var_vfb_i_dn9 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn9));
        locals.var_vfb_dn10 = ((locals.var_vfb_i_dn10 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn10));
        locals.var_vfb_dn11 = ((locals.var_vfb_i_dn11 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn11));
        locals.var_vfb_dn12 = ((locals.var_vfb_i_dn12 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn12));
        locals.var_vfb_dn13 = ((locals.var_vfb_i_dn13 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn13));
        locals.var_vfb_dn14 = ((locals.var_vfb_i_dn14 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn14));
        locals.var_vfb_rv = 0.0;

        let assign17220_e24322: f64 = (locals.var_sqrtphistvbs - locals.var_sqrtphist);
        let assign17220_e24323: f64 = (locals.var_k1_i * assign17220_e24322);
        let assign17220_e24326: f64 = (locals.var_k2_i * locals.var_vbsx);
        let assign17220_e24327: f64 = (assign17220_e24323 - assign17220_e24326);
        locals.var_dvth_vnud = assign17220_e24327;
        locals.var_dvth_vnud_dn0 = (((locals.var_k1_i_dn0 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn0 - locals.var_sqrtphist_dn0))) - ((locals.var_k2_i_dn0 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn0)));
        locals.var_dvth_vnud_dn2 = (((locals.var_k1_i_dn2 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn2 - locals.var_sqrtphist_dn2))) - ((locals.var_k2_i_dn2 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn2)));
        locals.var_dvth_vnud_dn3 = (((locals.var_k1_i_dn3 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn3 - locals.var_sqrtphist_dn3))) - ((locals.var_k2_i_dn3 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn3)));
        locals.var_dvth_vnud_dn4 = (((locals.var_k1_i_dn4 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn4 - locals.var_sqrtphist_dn4))) - ((locals.var_k2_i_dn4 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn4)));
        locals.var_dvth_vnud_dn5 = (((locals.var_k1_i_dn5 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn5 - locals.var_sqrtphist_dn5))) - ((locals.var_k2_i_dn5 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn5)));
        locals.var_dvth_vnud_dn6 = (((locals.var_k1_i_dn6 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn6 - locals.var_sqrtphist_dn6))) - ((locals.var_k2_i_dn6 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn6)));
        locals.var_dvth_vnud_dn7 = (((locals.var_k1_i_dn7 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn7 - locals.var_sqrtphist_dn7))) - ((locals.var_k2_i_dn7 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn7)));
        locals.var_dvth_vnud_dn8 = (((locals.var_k1_i_dn8 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn8 - locals.var_sqrtphist_dn8))) - ((locals.var_k2_i_dn8 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn8)));
        locals.var_dvth_vnud_dn9 = (((locals.var_k1_i_dn9 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn9 - locals.var_sqrtphist_dn9))) - ((locals.var_k2_i_dn9 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn9)));
        locals.var_dvth_vnud_dn10 = (((locals.var_k1_i_dn10 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn10 - locals.var_sqrtphist_dn10))) - ((locals.var_k2_i_dn10 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn10)));
        locals.var_dvth_vnud_dn11 = (((locals.var_k1_i_dn11 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn11 - locals.var_sqrtphist_dn11))) - ((locals.var_k2_i_dn11 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn11)));
        locals.var_dvth_vnud_dn12 = (((locals.var_k1_i_dn12 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn12 - locals.var_sqrtphist_dn12))) - ((locals.var_k2_i_dn12 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn12)));
        locals.var_dvth_vnud_dn13 = (((locals.var_k1_i_dn13 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn13 - locals.var_sqrtphist_dn13))) - ((locals.var_k2_i_dn13 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn13)));
        locals.var_dvth_vnud_dn14 = (((locals.var_k1_i_dn14 * assign17220_e24322) + (locals.var_k1_i * (locals.var_sqrtphistvbs_dn14 - locals.var_sqrtphist_dn14))) - ((locals.var_k2_i_dn14 * locals.var_vbsx) + (locals.var_k2_i * locals.var_vbsx_dn14)));
        locals.var_dvth_vnud_rv = 0.0;

        let assign17230_e24330: f64 = (locals.var_dvth_dibl + locals.var_dvth_ldop);
        let assign17230_e24332: f64 = (assign17230_e24330 + locals.var_dvth_vnud);
        let assign17230_e24334: f64 = (assign17230_e24332 - locals.var_dvth_temp);
        let assign17230_e24336: f64 = (assign17230_e24334 + locals.var_vth0_stress);
        let assign17230_e24338: f64 = (assign17230_e24336 + locals.var_vth0_well);
        locals.var_vth_shift = assign17230_e24338;
        locals.var_vth_shift_dn0 = (((((locals.var_dvth_dibl_dn0 + locals.var_dvth_ldop_dn0) + locals.var_dvth_vnud_dn0) - locals.var_dvth_temp_dn0) + locals.var_vth0_stress_dn0) + locals.var_vth0_well_dn0);
        locals.var_vth_shift_dn2 = (((((locals.var_dvth_dibl_dn2 + locals.var_dvth_ldop_dn2) + locals.var_dvth_vnud_dn2) - locals.var_dvth_temp_dn2) + locals.var_vth0_stress_dn2) + locals.var_vth0_well_dn2);
        locals.var_vth_shift_dn3 = (((((locals.var_dvth_dibl_dn3 + locals.var_dvth_ldop_dn3) + locals.var_dvth_vnud_dn3) - locals.var_dvth_temp_dn3) + locals.var_vth0_stress_dn3) + locals.var_vth0_well_dn3);
        locals.var_vth_shift_dn4 = (((((locals.var_dvth_dibl_dn4 + locals.var_dvth_ldop_dn4) + locals.var_dvth_vnud_dn4) - locals.var_dvth_temp_dn4) + locals.var_vth0_stress_dn4) + locals.var_vth0_well_dn4);
        locals.var_vth_shift_dn5 = (((((locals.var_dvth_dibl_dn5 + locals.var_dvth_ldop_dn5) + locals.var_dvth_vnud_dn5) - locals.var_dvth_temp_dn5) + locals.var_vth0_stress_dn5) + locals.var_vth0_well_dn5);
        locals.var_vth_shift_dn6 = (((((locals.var_dvth_dibl_dn6 + locals.var_dvth_ldop_dn6) + locals.var_dvth_vnud_dn6) - locals.var_dvth_temp_dn6) + locals.var_vth0_stress_dn6) + locals.var_vth0_well_dn6);
        locals.var_vth_shift_dn7 = (((((locals.var_dvth_dibl_dn7 + locals.var_dvth_ldop_dn7) + locals.var_dvth_vnud_dn7) - locals.var_dvth_temp_dn7) + locals.var_vth0_stress_dn7) + locals.var_vth0_well_dn7);
        locals.var_vth_shift_dn8 = (((((locals.var_dvth_dibl_dn8 + locals.var_dvth_ldop_dn8) + locals.var_dvth_vnud_dn8) - locals.var_dvth_temp_dn8) + locals.var_vth0_stress_dn8) + locals.var_vth0_well_dn8);
        locals.var_vth_shift_dn9 = (((((locals.var_dvth_dibl_dn9 + locals.var_dvth_ldop_dn9) + locals.var_dvth_vnud_dn9) - locals.var_dvth_temp_dn9) + locals.var_vth0_stress_dn9) + locals.var_vth0_well_dn9);
        locals.var_vth_shift_dn10 = (((((locals.var_dvth_dibl_dn10 + locals.var_dvth_ldop_dn10) + locals.var_dvth_vnud_dn10) - locals.var_dvth_temp_dn10) + locals.var_vth0_stress_dn10) + locals.var_vth0_well_dn10);
        locals.var_vth_shift_dn11 = (((((locals.var_dvth_dibl_dn11 + locals.var_dvth_ldop_dn11) + locals.var_dvth_vnud_dn11) - locals.var_dvth_temp_dn11) + locals.var_vth0_stress_dn11) + locals.var_vth0_well_dn11);
        locals.var_vth_shift_dn12 = (((((locals.var_dvth_dibl_dn12 + locals.var_dvth_ldop_dn12) + locals.var_dvth_vnud_dn12) - locals.var_dvth_temp_dn12) + locals.var_vth0_stress_dn12) + locals.var_vth0_well_dn12);
        locals.var_vth_shift_dn13 = (((((locals.var_dvth_dibl_dn13 + locals.var_dvth_ldop_dn13) + locals.var_dvth_vnud_dn13) - locals.var_dvth_temp_dn13) + locals.var_vth0_stress_dn13) + locals.var_vth0_well_dn13);
        locals.var_vth_shift_dn14 = (((((locals.var_dvth_dibl_dn14 + locals.var_dvth_ldop_dn14) + locals.var_dvth_vnud_dn14) - locals.var_dvth_temp_dn14) + locals.var_vth0_stress_dn14) + locals.var_vth0_well_dn14);
        locals.var_vth_shift_rv = 0.0;

        let assign17240_e24341: f64 = (locals.var_vg_1 - locals.var_vfb);
        let assign17240_e24344: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign17240_e24345: f64 = (assign17240_e24341 - assign17240_e24344);
        locals.var_vgfb = assign17240_e24345;
        locals.var_vgfb_dn0 = ((locals.var_vg_1_dn0 - locals.var_vfb_dn0) - ((locals.var_vth_shift_dn0 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn0)));
        locals.var_vgfb_dn2 = ((locals.var_vg_1_dn2 - locals.var_vfb_dn2) - ((locals.var_vth_shift_dn2 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn2)));
        locals.var_vgfb_dn3 = ((locals.var_vg_1_dn3 - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3)));
        locals.var_vgfb_dn4 = ((locals.var_vg_1_dn4 - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4)));
        locals.var_vgfb_dn5 = ((locals.var_vg_1_dn5 - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5)));
        locals.var_vgfb_dn6 = ((locals.var_vg_1_dn6 - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6)));
        locals.var_vgfb_dn7 = ((locals.var_vg_1_dn7 - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7)));
        locals.var_vgfb_dn8 = ((locals.var_vg_1_dn8 - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8)));
        locals.var_vgfb_dn9 = ((locals.var_vg_1_dn9 - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9)));
        locals.var_vgfb_dn10 = ((locals.var_vg_1_dn10 - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10)));
        locals.var_vgfb_dn11 = ((locals.var_vg_1_dn11 - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11)));
        locals.var_vgfb_dn12 = ((locals.var_vg_1_dn12 - locals.var_vfb_dn12) - ((locals.var_vth_shift_dn12 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn12)));
        locals.var_vgfb_dn13 = ((locals.var_vg_1_dn13 - locals.var_vfb_dn13) - ((locals.var_vth_shift_dn13 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn13)));
        locals.var_vgfb_dn14 = ((locals.var_vg_1_dn14 - locals.var_vfb_dn14) - ((locals.var_vth_shift_dn14 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn14)));
        locals.var_vgfb_rv = 0.0;

        let assign17250_e24348: f64 = (2.0 * 1.60219e-19);
        let assign17250_e24350: f64 = (assign17250_e24348 * locals.var_epssi);
        let assign17250_e24352: f64 = (assign17250_e24350 * locals.var_ndep_i);
        let assign17250_e24354: f64 = (assign17250_e24352 * locals.var_inv_vt);
        let assign17250_e24355: f64 = (assign17250_e24354).sqrt();
        let assign17250_e24357: f64 = (assign17250_e24355 / locals.var_cox);
        locals.var_gam = assign17250_e24357;
        locals.var_gam_dn0 = ((((assign17250_e24350 * locals.var_ndep_i_dn0) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn2 = ((((assign17250_e24350 * locals.var_ndep_i_dn2) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn3 = ((((assign17250_e24350 * locals.var_ndep_i_dn3) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn4 = (((((assign17250_e24350 * locals.var_ndep_i_dn4) * locals.var_inv_vt) + (assign17250_e24352 * locals.var_inv_vt_dn4)) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn5 = ((((assign17250_e24350 * locals.var_ndep_i_dn5) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn6 = ((((assign17250_e24350 * locals.var_ndep_i_dn6) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn7 = ((((assign17250_e24350 * locals.var_ndep_i_dn7) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn8 = ((((assign17250_e24350 * locals.var_ndep_i_dn8) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn9 = ((((assign17250_e24350 * locals.var_ndep_i_dn9) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn10 = ((((assign17250_e24350 * locals.var_ndep_i_dn10) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn11 = ((((assign17250_e24350 * locals.var_ndep_i_dn11) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn12 = ((((assign17250_e24350 * locals.var_ndep_i_dn12) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn13 = ((((assign17250_e24350 * locals.var_ndep_i_dn13) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_dn14 = ((((assign17250_e24350 * locals.var_ndep_i_dn14) * locals.var_inv_vt) / (2.0 * assign17250_e24355)) / locals.var_cox);
        locals.var_gam_rv = 0.0;

        let assign17270_e24361: f64 = (2.0 * locals.var_phib);
        let assign17270_e24364: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17270_e24365: f64 = (assign17270_e24361 + assign17270_e24364);
        let assign17270_e24367: f64 = (-10000.0);
        let assign17270_e24369: f64 = (assign17270_e24367 * 0.001);
        let (assign17270_e24430, assign17270_e24430_d_n0, assign17270_e24430_d_n2, assign17270_e24430_d_n3, assign17270_e24430_d_n4, assign17270_e24430_d_n5, assign17270_e24430_d_n6, assign17270_e24430_d_n7, assign17270_e24430_d_n8, assign17270_e24430_d_n9, assign17270_e24430_d_n10, assign17270_e24430_d_n11, assign17270_e24430_d_n12, assign17270_e24430_d_n13, assign17270_e24430_d_n14,) = {
    if (!(assign17270_e24365 < assign17270_e24369)) {
        let assign17270_e24375: f64 = (2.0 * locals.var_phib);
        let assign17270_e24378: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17270_e24379: f64 = (assign17270_e24375 + assign17270_e24378);
        let assign17270_e24382: f64 = (2.0 * locals.var_phib);
        let assign17270_e24385: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17270_e24386: f64 = (assign17270_e24382 + assign17270_e24385);
        let assign17270_e24389: f64 = (2.0 * locals.var_phib);
        let assign17270_e24392: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17270_e24393: f64 = (assign17270_e24389 + assign17270_e24392);
        let assign17270_e24394: f64 = (assign17270_e24386 * assign17270_e24393);
        let assign17270_e24397: f64 = (4.0 * 0.001);
        let assign17270_e24399: f64 = (assign17270_e24397 * 0.001);
        let assign17270_e24400: f64 = (assign17270_e24394 + assign17270_e24399);
        let assign17270_e24401: f64 = (assign17270_e24400).sqrt();
        let assign17270_e24402: f64 = (assign17270_e24379 + assign17270_e24401);
        let assign17270_e24403: f64 = (0.5 * assign17270_e24402);
        (assign17270_e24403, (0.5 * ((2.0 * locals.var_phib_dn0) + ((((2.0 * locals.var_phib_dn0) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn0))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn2) + ((((2.0 * locals.var_phib_dn2) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn2))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn3) + ((((2.0 * locals.var_phib_dn3) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn3))) / (2.0 * assign17270_e24401)))), (0.5 * (((2.0 * locals.var_phib_dn4) + (locals.var_vs * locals.var_inv_vt_dn4)) + (((((2.0 * locals.var_phib_dn4) + (locals.var_vs * locals.var_inv_vt_dn4)) * assign17270_e24393) + (assign17270_e24386 * ((2.0 * locals.var_phib_dn4) + (locals.var_vs * locals.var_inv_vt_dn4)))) / (2.0 * assign17270_e24401)))), (0.5 * (((2.0 * locals.var_phib_dn5) + (locals.var_vs_dn5 * locals.var_inv_vt)) + (((((2.0 * locals.var_phib_dn5) + (locals.var_vs_dn5 * locals.var_inv_vt)) * assign17270_e24393) + (assign17270_e24386 * ((2.0 * locals.var_phib_dn5) + (locals.var_vs_dn5 * locals.var_inv_vt)))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn6) + ((((2.0 * locals.var_phib_dn6) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn6))) / (2.0 * assign17270_e24401)))), (0.5 * (((2.0 * locals.var_phib_dn7) + (locals.var_vs_dn7 * locals.var_inv_vt)) + (((((2.0 * locals.var_phib_dn7) + (locals.var_vs_dn7 * locals.var_inv_vt)) * assign17270_e24393) + (assign17270_e24386 * ((2.0 * locals.var_phib_dn7) + (locals.var_vs_dn7 * locals.var_inv_vt)))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn8) + ((((2.0 * locals.var_phib_dn8) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn8))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn9) + ((((2.0 * locals.var_phib_dn9) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn9))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn10) + ((((2.0 * locals.var_phib_dn10) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn10))) / (2.0 * assign17270_e24401)))), (0.5 * (((2.0 * locals.var_phib_dn11) + (locals.var_vs_dn11 * locals.var_inv_vt)) + (((((2.0 * locals.var_phib_dn11) + (locals.var_vs_dn11 * locals.var_inv_vt)) * assign17270_e24393) + (assign17270_e24386 * ((2.0 * locals.var_phib_dn11) + (locals.var_vs_dn11 * locals.var_inv_vt)))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn12) + ((((2.0 * locals.var_phib_dn12) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn12))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn13) + ((((2.0 * locals.var_phib_dn13) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn13))) / (2.0 * assign17270_e24401)))), (0.5 * ((2.0 * locals.var_phib_dn14) + ((((2.0 * locals.var_phib_dn14) * assign17270_e24393) + (assign17270_e24386 * (2.0 * locals.var_phib_dn14))) / (2.0 * assign17270_e24401)))),)
    } else {
        let assign17270_e24406: f64 = (2.0 * locals.var_phib);
        let assign17270_e24409: f64 = (locals.var_vs * locals.var_inv_vt);
        let assign17270_e24410: f64 = (assign17270_e24406 + assign17270_e24409);
        let assign17270_e24412: f64 = (-10000.0);
        let assign17270_e24414: f64 = (assign17270_e24412 * 0.001);
        let (assign17270_e24429, assign17270_e24429_d_n0, assign17270_e24429_d_n2, assign17270_e24429_d_n3, assign17270_e24429_d_n4, assign17270_e24429_d_n5, assign17270_e24429_d_n6, assign17270_e24429_d_n7, assign17270_e24429_d_n8, assign17270_e24429_d_n9, assign17270_e24429_d_n10, assign17270_e24429_d_n11, assign17270_e24429_d_n12, assign17270_e24429_d_n13, assign17270_e24429_d_n14,) = {
            if (assign17270_e24410 < assign17270_e24414) {
                let assign17270_e24417: f64 = (-0.001);
                let assign17270_e24419: f64 = (assign17270_e24417 * 0.001);
                let assign17270_e24422: f64 = (2.0 * locals.var_phib);
                let assign17270_e24425: f64 = (locals.var_vs * locals.var_inv_vt);
                let assign17270_e24426: f64 = (assign17270_e24422 + assign17270_e24425);
                let assign17270_e24427: f64 = (assign17270_e24419 / assign17270_e24426);
                (assign17270_e24427, (-((assign17270_e24419 * (2.0 * locals.var_phib_dn0)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn2)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn3)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * ((2.0 * locals.var_phib_dn4) + (locals.var_vs * locals.var_inv_vt_dn4))) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * ((2.0 * locals.var_phib_dn5) + (locals.var_vs_dn5 * locals.var_inv_vt))) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn6)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * ((2.0 * locals.var_phib_dn7) + (locals.var_vs_dn7 * locals.var_inv_vt))) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn8)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn9)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn10)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * ((2.0 * locals.var_phib_dn11) + (locals.var_vs_dn11 * locals.var_inv_vt))) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn12)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn13)) / (assign17270_e24426 * assign17270_e24426))), (-((assign17270_e24419 * (2.0 * locals.var_phib_dn14)) / (assign17270_e24426 * assign17270_e24426))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign17270_e24429, assign17270_e24429_d_n0, assign17270_e24429_d_n2, assign17270_e24429_d_n3, assign17270_e24429_d_n4, assign17270_e24429_d_n5, assign17270_e24429_d_n6, assign17270_e24429_d_n7, assign17270_e24429_d_n8, assign17270_e24429_d_n9, assign17270_e24429_d_n10, assign17270_e24429_d_n11, assign17270_e24429_d_n12, assign17270_e24429_d_n13, assign17270_e24429_d_n14,)
    }
};
        locals.var_t0 = assign17270_e24430;
        locals.var_t0_dn0 = assign17270_e24430_d_n0;
        locals.var_t0_dn2 = assign17270_e24430_d_n2;
        locals.var_t0_dn3 = assign17270_e24430_d_n3;
        locals.var_t0_dn4 = assign17270_e24430_d_n4;
        locals.var_t0_dn5 = assign17270_e24430_d_n5;
        locals.var_t0_dn6 = assign17270_e24430_d_n6;
        locals.var_t0_dn7 = assign17270_e24430_d_n7;
        locals.var_t0_dn8 = assign17270_e24430_d_n8;
        locals.var_t0_dn9 = assign17270_e24430_d_n9;
        locals.var_t0_dn10 = assign17270_e24430_d_n10;
        locals.var_t0_dn11 = assign17270_e24430_d_n11;
        locals.var_t0_dn12 = assign17270_e24430_d_n12;
        locals.var_t0_dn13 = assign17270_e24430_d_n13;
        locals.var_t0_dn14 = assign17270_e24430_d_n14;
        locals.var_t0_rv = 0.0;

        let assign17280_e24435: f64 = (locals.var_t0).sqrt();
        let assign17280_e24436: f64 = (2.0 * assign17280_e24435);
        let assign17280_e24437: f64 = (locals.var_gam / assign17280_e24436);
        let assign17280_e24438: f64 = (1.0 + assign17280_e24437);
        locals.var_nq = assign17280_e24438;
        locals.var_nq_dn0 = (((locals.var_gam_dn0 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn0 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn2 = (((locals.var_gam_dn2 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn2 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn3 = (((locals.var_gam_dn3 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn3 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn4 = (((locals.var_gam_dn4 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn4 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn5 = (((locals.var_gam_dn5 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn5 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn6 = (((locals.var_gam_dn6 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn6 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn7 = (((locals.var_gam_dn7 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn7 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn8 = (((locals.var_gam_dn8 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn8 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn9 = (((locals.var_gam_dn9 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn9 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn10 = (((locals.var_gam_dn10 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn10 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn11 = (((locals.var_gam_dn11 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn11 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn12 = (((locals.var_gam_dn12 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn12 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn13 = (((locals.var_gam_dn13 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn13 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_dn14 = (((locals.var_gam_dn14 * assign17280_e24436) - (locals.var_gam * (2.0 * (locals.var_t0_dn14 / (2.0 * assign17280_e24435))))) / (assign17280_e24436 * assign17280_e24436));
        locals.var_nq_rv = 0.0;

        let assign17310_e24719: f64 = (2.0 * 1.60219e-19);
        let assign17310_e24721: f64 = (assign17310_e24719 * locals.var_epssi);
        let assign17310_e24723: f64 = (assign17310_e24721 * locals.var_ndep_i);
        let assign17310_e24725: f64 = (assign17310_e24723 * locals.var_inv_nvt);
        let assign17310_e24726: f64 = (assign17310_e24725).sqrt();
        let assign17310_e24728: f64 = (assign17310_e24726 / locals.var_cox);
        locals.var_gam = assign17310_e24728;
        locals.var_gam_dn0 = (((((assign17310_e24721 * locals.var_ndep_i_dn0) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn0)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn2 = (((((assign17310_e24721 * locals.var_ndep_i_dn2) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn2)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn3 = (((((assign17310_e24721 * locals.var_ndep_i_dn3) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn3)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn4 = (((((assign17310_e24721 * locals.var_ndep_i_dn4) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn4)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn5 = (((((assign17310_e24721 * locals.var_ndep_i_dn5) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn5)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn6 = (((((assign17310_e24721 * locals.var_ndep_i_dn6) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn6)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn7 = (((((assign17310_e24721 * locals.var_ndep_i_dn7) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn7)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn8 = (((((assign17310_e24721 * locals.var_ndep_i_dn8) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn8)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn9 = (((((assign17310_e24721 * locals.var_ndep_i_dn9) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn9)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn10 = (((((assign17310_e24721 * locals.var_ndep_i_dn10) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn10)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn11 = (((((assign17310_e24721 * locals.var_ndep_i_dn11) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn11)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn12 = (((((assign17310_e24721 * locals.var_ndep_i_dn12) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn12)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn13 = (((((assign17310_e24721 * locals.var_ndep_i_dn13) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn13)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_dn14 = (((((assign17310_e24721 * locals.var_ndep_i_dn14) * locals.var_inv_nvt) + (assign17310_e24723 * locals.var_inv_nvt_dn14)) / (2.0 * assign17310_e24726)) / locals.var_cox);
        locals.var_gam_rv = 0.0;

        let assign17320_e24731: f64 = (1.0 / locals.var_gam);
        locals.var_inv_gam = assign17320_e24731;
        locals.var_inv_gam_dn0 = (-(locals.var_gam_dn0 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn2 = (-(locals.var_gam_dn2 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn3 = (-(locals.var_gam_dn3 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn4 = (-(locals.var_gam_dn4 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn5 = (-(locals.var_gam_dn5 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn6 = (-(locals.var_gam_dn6 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn7 = (-(locals.var_gam_dn7 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn8 = (-(locals.var_gam_dn8 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn9 = (-(locals.var_gam_dn9 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn10 = (-(locals.var_gam_dn10 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn11 = (-(locals.var_gam_dn11 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn12 = (-(locals.var_gam_dn12 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn13 = (-(locals.var_gam_dn13 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_dn14 = (-(locals.var_gam_dn14 / (locals.var_gam * locals.var_gam)));
        locals.var_inv_gam_rv = 0.0;

    }
}
