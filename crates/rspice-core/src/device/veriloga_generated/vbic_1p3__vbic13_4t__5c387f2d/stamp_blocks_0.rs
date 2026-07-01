#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e408: f64 = if ctx.analysis_static() { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e408;

        let assign70_e452: f64 = if param_given[10] { 1.0 } else { 0.0 };
        locals.var_guard4 = assign70_e452;

        let (assign80_e458,) = {
    if ((locals.var_guard1 != 0.0) && (locals.var_guard4 != 0.0)) {
        (p.p10,)
    } else {
        (locals.var_gminmod,)
    }
};
        locals.var_gminmod = assign80_e458;

        let (assign90_e467,) = {
    if ((locals.var_guard1 != 0.0) && (locals.var_guard4 == 0.0)) {
        let assign90_e465: f64 = 1e-12;
        (assign90_e465,)
    } else {
        (locals.var_gminmod,)
    }
};
        locals.var_gminmod = assign90_e467;

        let assign100_e469: f64 = if param_given[11] { 1.0 } else { 0.0 };
        locals.var_guard5 = assign100_e469;

        let (assign110_e475,) = {
    if ((locals.var_guard1 != 0.0) && (locals.var_guard5 != 0.0)) {
        (p.p11,)
    } else {
        (locals.var_imaxmod,)
    }
};
        locals.var_imaxmod = assign110_e475;

        let (assign120_e484,) = {
    if ((locals.var_guard1 != 0.0) && (locals.var_guard5 == 0.0)) {
        let assign120_e482: f64 = 1.0;
        (assign120_e482,)
    } else {
        (locals.var_imaxmod,)
    }
};
        locals.var_imaxmod = assign120_e484;

        let assign130_e486: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard6 = assign130_e486;

        let (assign140_e493,) = {
    if ((locals.var_guard1 != 0.0) && (locals.var_guard6 != 0.0)) {
        let assign140_e491: f64 = 1.0;
        (assign140_e491,)
    } else {
        (locals.var_vbictype,)
    }
};
        locals.var_vbictype = assign140_e493;

        let assign150_e495: f64 = if param_given[4] { 1.0 } else { 0.0 };
        locals.var_guard7 = assign150_e495;

        let (assign160_e505,) = {
    if (((locals.var_guard1 != 0.0) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 != 0.0)) {
        let assign160_e503: f64 = (-1.0);
        (assign160_e503,)
    } else {
        (locals.var_vbictype,)
    }
};
        locals.var_vbictype = assign160_e505;

        let assign170_e507: f64 = if param_given[5] { 1.0 } else { 0.0 };
        locals.var_guard8 = assign170_e507;

        let (assign180_e519,) = {
    if ((((locals.var_guard1 != 0.0) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 == 0.0)) && (locals.var_guard8 != 0.0)) {
        (p.p5,)
    } else {
        (locals.var_vbictype,)
    }
};
        locals.var_vbictype = assign180_e519;

        let (assign190_e533,) = {
    if ((((locals.var_guard1 != 0.0) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 == 0.0)) && (locals.var_guard8 == 0.0)) {
        let assign190_e531: f64 = 1.0;
        (assign190_e531,)
    } else {
        (locals.var_vbictype,)
    }
};
        locals.var_vbictype = assign190_e533;

        let (assign200_e538,) = {
    if (locals.var_guard1 != 0.0) {
        let assign200_e536: f64 = (p.p12).ln();
        (assign200_e536,)
    } else {
        (locals.var_vmaxexp,)
    }
};
        locals.var_vmaxexp = assign200_e538;

        let (assign210_e549,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign210_e547,) = {
            if (p.p74 > 0.0) {
                let assign210_e545: f64 = (1.0 / p.p74);
                (assign210_e545,)
            } else {
                (0.0,)
            }
        };
        (assign210_e547,)
    } else {
        (locals.var_iikr,)
    }
};
        locals.var_iikr = assign210_e549;

        let (assign220_e560,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign220_e558,) = {
            if (p.p75 > 0.0) {
                let assign220_e556: f64 = (1.0 / p.p75);
                (assign220_e556,)
            } else {
                (0.0,)
            }
        };
        (assign220_e558,)
    } else {
        (locals.var_iikp,)
    }
};
        locals.var_iikp = assign220_e560;

        let (assign230_e571,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign230_e569,) = {
            if (p.p20 > 0.0) {
                let assign230_e567: f64 = (1.0 / p.p20);
                (assign230_e567,)
            } else {
                (0.0,)
            }
        };
        (assign230_e569,)
    } else {
        (locals.var_ihrcf,)
    }
};
        locals.var_ihrcf = assign230_e571;

        let (assign240_e582,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign240_e580,) = {
            if (p.p79 > 0.0) {
                let assign240_e578: f64 = (1.0 / p.p79);
                (assign240_e578,)
            } else {
                (0.0,)
            }
        };
        (assign240_e580,)
    } else {
        (locals.var_ivtf,)
    }
};
        locals.var_ivtf = assign240_e582;

        let (assign250_e593,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign250_e591,) = {
            if (p.p80 > 0.0) {
                let assign250_e589: f64 = (1.0 / p.p80);
                (assign250_e589,)
            } else {
                (0.0,)
            }
        };
        (assign250_e591,)
    } else {
        (locals.var_iitf,)
    }
};
        locals.var_iitf = assign250_e593;

        let (assign260_e602,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign260_e600,) = {
            if (p.p80 > 0.0) {
                (0.0,)
            } else {
                (1.0,)
            }
        };
        (assign260_e600,)
    } else {
        (locals.var_sltf,)
    }
};
        locals.var_sltf = assign260_e602;

        let (assign270_e608,) = {
    if (locals.var_guard1 != 0.0) {
        let assign270_e606: f64 = (273.15 + p.p13);
        (assign270_e606,)
    } else {
        (locals.var_tinik,)
    }
};
        locals.var_tinik = assign270_e608;

        let assign290_e610: f64 = ctx_temp;
        let assign290_e612: f64 = (assign290_e610 + p.p0);
        let assign290_e614: f64 = (assign290_e612 - 273.15);
        locals.var_tdevc = assign290_e614;
        locals.var_tdevc_dn4 = 0.0;

        let assign320_e624: f64 = (p.p14 + 1.0);
        let assign320_e625: f64 = if locals.var_tdevc < assign320_e624 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign320_e625;

        let (assign330_e636, assign330_e636_d_n4,) = {
    if (locals.var_guard11 != 0.0) {
        let assign330_e630: f64 = (locals.var_tdevc - p.p14);
        let assign330_e632: f64 = (assign330_e630 - 1.0);
        let assign330_e633: f64 = (assign330_e632).exp();
        let assign330_e634: f64 = (p.p14 + assign330_e633);
        (assign330_e634, (assign330_e633 * locals.var_tdevc_dn4),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign330_e636;
        locals.var_tdevc_dn4 = assign330_e636_d_n4;

        let assign340_e640: f64 = (p.p15 - 1.0);
        let assign340_e641: f64 = if locals.var_tdevc > assign340_e640 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign340_e641;

        let (assign350_e655, assign350_e655_d_n4,) = {
    if ((locals.var_guard11 == 0.0) && (locals.var_guard12 != 0.0)) {
        let assign350_e649: f64 = (p.p15 - locals.var_tdevc);
        let assign350_e651: f64 = (assign350_e649 - 1.0);
        let assign350_e652: f64 = (assign350_e651).exp();
        let assign350_e653: f64 = (p.p15 - assign350_e652);
        (assign350_e653, (-(assign350_e652 * (-locals.var_tdevc_dn4))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign350_e655;
        locals.var_tdevc_dn4 = assign350_e655_d_n4;

        let (assign360_e663, assign360_e663_d_n4,) = {
    if ((locals.var_guard11 == 0.0) && (locals.var_guard12 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign360_e663;
        locals.var_tdevc_dn4 = assign360_e663_d_n4;

        let assign370_e666: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign370_e666;
        locals.var_tdevk_dn4 = locals.var_tdevc_dn4;

        let assign380_e669: f64 = (1.380662e-23 * locals.var_tdevk);
        let assign380_e671: f64 = (assign380_e669 / 1.602189e-19);
        locals.var_vtv = assign380_e671;
        locals.var_vtv_dn4 = ((1.380662e-23 * locals.var_tdevk_dn4) / 1.602189e-19);

        let assign390_e674: f64 = (locals.var_tdevk / locals.var_tinik);
        locals.var_rt = assign390_e674;
        locals.var_rt_dn4 = (locals.var_tdevk_dn4 / locals.var_tinik);

        let assign410_e687: f64 = if p.p90 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign410_e687;

        let (assign420_e706, assign420_e706_d_n4,) = {
    if (locals.var_guard13 != 0.0) {
        let assign420_e691: f64 = (p.p89 * locals.var_vtv);
        let assign420_e693: f64 = (-p.p88);
        let assign420_e696: f64 = (p.p89 * locals.var_vtv);
        let assign420_e697: f64 = (assign420_e693 / assign420_e696);
        let assign420_e698: f64 = (assign420_e697).exp();
        let assign420_e701: f64 = (locals.var_imaxmod / p.p90);
        let assign420_e702: f64 = (assign420_e698 + assign420_e701);
        let assign420_e703: f64 = (assign420_e702).ln();
        let assign420_e704: f64 = (assign420_e691 * assign420_e703);
        (assign420_e704, (((p.p89 * locals.var_vtv_dn4) * assign420_e703) + (assign420_e691 * ((assign420_e698 * (-((assign420_e693 * (p.p89 * locals.var_vtv_dn4)) / (assign420_e696 * assign420_e696)))) / assign420_e702))),)
    } else {
        (locals.var_maxvibbe, locals.var_maxvibbe_dn4,)
    }
};
        locals.var_maxvibbe = assign420_e706;
        locals.var_maxvibbe_dn4 = assign420_e706_d_n4;

        let (assign430_e711, assign430_e711_d_n4,) = {
    if (locals.var_guard13 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibbe, locals.var_maxvibbe_dn4,)
    }
};
        locals.var_maxvibbe = assign430_e711;
        locals.var_maxvibbe_dn4 = assign430_e711_d_n4;

        let assign440_e716: f64 = (p.p122 / p.p28);
        let assign440_e717: f64 = (locals.var_rt).powf(assign440_e716);
        let assign440_e718: f64 = (p.p26 * assign440_e717);
        let assign440_e720: f64 = (-p.p113);
        let assign440_e723: f64 = (1.0 - locals.var_rt);
        let assign440_e724: f64 = (assign440_e720 * assign440_e723);
        let assign440_e727: f64 = (locals.var_vtv * p.p28);
        let assign440_e728: f64 = (assign440_e724 / assign440_e727);
        let assign440_e729: f64 = (assign440_e728).exp();
        let assign440_e730: f64 = (assign440_e718 * assign440_e729);
        locals.var_is_t = assign440_e730;
        locals.var_is_t_dn4 = (((p.p26 * if 0.0 == 0.0 && ((assign440_e716) as f64).is_finite() && ((assign440_e716) as f64).fract() == 0.0 { if assign440_e716 == 0.0 { 0.0 } else { (assign440_e716 * ((locals.var_rt).powf(assign440_e716 - 1.0) * locals.var_rt_dn4)) } } else { (assign440_e717 * (assign440_e716 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign440_e729) + (assign440_e718 * (assign440_e729 * ((((assign440_e720 * (-locals.var_rt_dn4)) * assign440_e727) - (assign440_e724 * (locals.var_vtv_dn4 * p.p28))) / (assign440_e727 * assign440_e727)))));

        let assign450_e733: f64 = if locals.var_is_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign450_e733;

        let assign460_e740: f64 = if ((p.p72 > 0.0) && (locals.var_imaxmod > p.p72)) { 1.0 } else { 0.0 };
        locals.var_guard15 = assign460_e740;

        let (assign470_e769, assign470_e769_d_n4,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 != 0.0)) {
        let assign470_e746: f64 = (p.p28 * locals.var_vtv);
        let assign470_e750: f64 = (0.5 * locals.var_imaxmod);
        let assign470_e753: f64 = (4.0 / p.p72);
        let assign470_e755: f64 = (assign470_e753).powf(p.p73);
        let assign470_e756: f64 = (assign470_e750 * assign470_e755);
        let assign470_e760: f64 = (1.0 - p.p73);
        let assign470_e761: f64 = (1.0 / assign470_e760);
        let assign470_e762: f64 = (assign470_e756).powf(assign470_e761);
        let assign470_e764: f64 = (assign470_e762 / locals.var_is_t);
        let assign470_e765: f64 = (1.0 + assign470_e764);
        let assign470_e766: f64 = (assign470_e765).ln();
        let assign470_e767: f64 = (assign470_e746 * assign470_e766);
        (assign470_e767, (((p.p28 * locals.var_vtv_dn4) * assign470_e766) + (assign470_e746 * ((-((assign470_e762 * locals.var_is_t_dn4) / (locals.var_is_t * locals.var_is_t))) / assign470_e765))),)
    } else {
        (locals.var_maxvifi, locals.var_maxvifi_dn4,)
    }
};
        locals.var_maxvifi = assign470_e769;
        locals.var_maxvifi_dn4 = assign470_e769_d_n4;

        let (assign480_e785, assign480_e785_d_n4,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign480_e776: f64 = (p.p28 * locals.var_vtv);
        let assign480_e780: f64 = (locals.var_imaxmod / locals.var_is_t);
        let assign480_e781: f64 = (1.0 + assign480_e780);
        let assign480_e782: f64 = (assign480_e781).ln();
        let assign480_e783: f64 = (assign480_e776 * assign480_e782);
        (assign480_e783, (((p.p28 * locals.var_vtv_dn4) * assign480_e782) + (assign480_e776 * ((-((locals.var_imaxmod * locals.var_is_t_dn4) / (locals.var_is_t * locals.var_is_t))) / assign480_e781))),)
    } else {
        (locals.var_maxvifi, locals.var_maxvifi_dn4,)
    }
};
        locals.var_maxvifi = assign480_e785;
        locals.var_maxvifi_dn4 = assign480_e785_d_n4;

        let (assign490_e790, assign490_e790_d_n4,) = {
    if (locals.var_guard14 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvifi, locals.var_maxvifi_dn4,)
    }
};
        locals.var_maxvifi = assign490_e790;
        locals.var_maxvifi_dn4 = assign490_e790_d_n4;

        let assign500_e795: f64 = (p.p125 / p.p29);
        let assign500_e796: f64 = (locals.var_rt).powf(assign500_e795);
        let assign500_e797: f64 = (p.p27 * assign500_e796);
        let assign500_e799: f64 = (-p.p121);
        let assign500_e802: f64 = (1.0 - locals.var_rt);
        let assign500_e803: f64 = (assign500_e799 * assign500_e802);
        let assign500_e806: f64 = (locals.var_vtv * p.p29);
        let assign500_e807: f64 = (assign500_e803 / assign500_e806);
        let assign500_e808: f64 = (assign500_e807).exp();
        let assign500_e809: f64 = (assign500_e797 * assign500_e808);
        locals.var_isrr_t = assign500_e809;
        locals.var_isrr_t_dn4 = (((p.p27 * if 0.0 == 0.0 && ((assign500_e795) as f64).is_finite() && ((assign500_e795) as f64).fract() == 0.0 { if assign500_e795 == 0.0 { 0.0 } else { (assign500_e795 * ((locals.var_rt).powf(assign500_e795 - 1.0) * locals.var_rt_dn4)) } } else { (assign500_e796 * (assign500_e795 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign500_e808) + (assign500_e797 * (assign500_e808 * ((((assign500_e799 * (-locals.var_rt_dn4)) * assign500_e806) - (assign500_e803 * (locals.var_vtv_dn4 * p.p29))) / (assign500_e806 * assign500_e806)))));

        let assign510_e816: f64 = if ((locals.var_is_t > 0.0) && (locals.var_isrr_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard16 = assign510_e816;

        let assign520_e823: f64 = if ((p.p74 > 0.0) && (locals.var_imaxmod > p.p74)) { 1.0 } else { 0.0 };
        locals.var_guard17 = assign520_e823;

        let (assign530_e854, assign530_e854_d_n4,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 != 0.0)) {
        let assign530_e829: f64 = (p.p29 * locals.var_vtv);
        let assign530_e833: f64 = (0.5 * locals.var_imaxmod);
        let assign530_e836: f64 = (4.0 / p.p74);
        let assign530_e838: f64 = (assign530_e836).powf(p.p73);
        let assign530_e839: f64 = (assign530_e833 * assign530_e838);
        let assign530_e843: f64 = (1.0 - p.p73);
        let assign530_e844: f64 = (1.0 / assign530_e843);
        let assign530_e845: f64 = (assign530_e839).powf(assign530_e844);
        let assign530_e848: f64 = (locals.var_is_t * locals.var_isrr_t);
        let assign530_e849: f64 = (assign530_e845 / assign530_e848);
        let assign530_e850: f64 = (1.0 + assign530_e849);
        let assign530_e851: f64 = (assign530_e850).ln();
        let assign530_e852: f64 = (assign530_e829 * assign530_e851);
        (assign530_e852, (((p.p29 * locals.var_vtv_dn4) * assign530_e851) + (assign530_e829 * ((-((assign530_e845 * ((locals.var_is_t_dn4 * locals.var_isrr_t) + (locals.var_is_t * locals.var_isrr_t_dn4))) / (assign530_e848 * assign530_e848))) / assign530_e850))),)
    } else {
        (locals.var_maxviri, locals.var_maxviri_dn4,)
    }
};
        locals.var_maxviri = assign530_e854;
        locals.var_maxviri_dn4 = assign530_e854_d_n4;

        let (assign540_e872, assign540_e872_d_n4,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign540_e861: f64 = (p.p29 * locals.var_vtv);
        let assign540_e866: f64 = (locals.var_is_t * locals.var_isrr_t);
        let assign540_e867: f64 = (locals.var_imaxmod / assign540_e866);
        let assign540_e868: f64 = (1.0 + assign540_e867);
        let assign540_e869: f64 = (assign540_e868).ln();
        let assign540_e870: f64 = (assign540_e861 * assign540_e869);
        (assign540_e870, (((p.p29 * locals.var_vtv_dn4) * assign540_e869) + (assign540_e861 * ((-((locals.var_imaxmod * ((locals.var_is_t_dn4 * locals.var_isrr_t) + (locals.var_is_t * locals.var_isrr_t_dn4))) / (assign540_e866 * assign540_e866))) / assign540_e868))),)
    } else {
        (locals.var_maxviri, locals.var_maxviri_dn4,)
    }
};
        locals.var_maxviri = assign540_e872;
        locals.var_maxviri_dn4 = assign540_e872_d_n4;

        let (assign550_e877, assign550_e877_d_n4,) = {
    if (locals.var_guard16 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxviri, locals.var_maxviri_dn4,)
    }
};
        locals.var_maxviri = assign550_e877;
        locals.var_maxviri_dn4 = assign550_e877_d_n4;

        let assign560_e882: f64 = (p.p122 / p.p33);
        let assign560_e883: f64 = (locals.var_rt).powf(assign560_e882);
        let assign560_e884: f64 = (p.p31 * assign560_e883);
        let assign560_e886: f64 = (-p.p120);
        let assign560_e889: f64 = (1.0 - locals.var_rt);
        let assign560_e890: f64 = (assign560_e886 * assign560_e889);
        let assign560_e893: f64 = (locals.var_vtv * p.p33);
        let assign560_e894: f64 = (assign560_e890 / assign560_e893);
        let assign560_e895: f64 = (assign560_e894).exp();
        let assign560_e896: f64 = (assign560_e884 * assign560_e895);
        locals.var_isp_t = assign560_e896;
        locals.var_isp_t_dn4 = (((p.p31 * if 0.0 == 0.0 && ((assign560_e882) as f64).is_finite() && ((assign560_e882) as f64).fract() == 0.0 { if assign560_e882 == 0.0 { 0.0 } else { (assign560_e882 * ((locals.var_rt).powf(assign560_e882 - 1.0) * locals.var_rt_dn4)) } } else { (assign560_e883 * (assign560_e882 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign560_e895) + (assign560_e884 * (assign560_e895 * ((((assign560_e886 * (-locals.var_rt_dn4)) * assign560_e893) - (assign560_e890 * (locals.var_vtv_dn4 * p.p33))) / (assign560_e893 * assign560_e893)))));

        let assign570_e899: f64 = if locals.var_isp_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign570_e899;

        let assign580_e906: f64 = if ((p.p75 > 0.0) && (locals.var_imaxmod > p.p75)) { 1.0 } else { 0.0 };
        locals.var_guard19 = assign580_e906;

        let (assign590_e925, assign590_e925_d_n4,) = {
    if ((locals.var_guard18 != 0.0) && (locals.var_guard19 != 0.0)) {
        let assign590_e912: f64 = (p.p33 * locals.var_vtv);
        let assign590_e916: f64 = (locals.var_imaxmod * locals.var_imaxmod);
        let assign590_e918: f64 = (assign590_e916 * locals.var_iikp);
        let assign590_e920: f64 = (assign590_e918 / locals.var_isp_t);
        let assign590_e921: f64 = (1.0 + assign590_e920);
        let assign590_e922: f64 = (assign590_e921).ln();
        let assign590_e923: f64 = (assign590_e912 * assign590_e922);
        (assign590_e923, (((p.p33 * locals.var_vtv_dn4) * assign590_e922) + (assign590_e912 * ((-((assign590_e918 * locals.var_isp_t_dn4) / (locals.var_isp_t * locals.var_isp_t))) / assign590_e921))),)
    } else {
        (locals.var_maxvip, locals.var_maxvip_dn4,)
    }
};
        locals.var_maxvip = assign590_e925;
        locals.var_maxvip_dn4 = assign590_e925_d_n4;

        let (assign600_e941, assign600_e941_d_n4,) = {
    if ((locals.var_guard18 != 0.0) && (locals.var_guard19 == 0.0)) {
        let assign600_e932: f64 = (p.p33 * locals.var_vtv);
        let assign600_e936: f64 = (locals.var_imaxmod / locals.var_isp_t);
        let assign600_e937: f64 = (1.0 + assign600_e936);
        let assign600_e938: f64 = (assign600_e937).ln();
        let assign600_e939: f64 = (assign600_e932 * assign600_e938);
        (assign600_e939, (((p.p33 * locals.var_vtv_dn4) * assign600_e938) + (assign600_e932 * ((-((locals.var_imaxmod * locals.var_isp_t_dn4) / (locals.var_isp_t * locals.var_isp_t))) / assign600_e937))),)
    } else {
        (locals.var_maxvip, locals.var_maxvip_dn4,)
    }
};
        locals.var_maxvip = assign600_e941;
        locals.var_maxvip_dn4 = assign600_e941_d_n4;

        let (assign610_e946, assign610_e946_d_n4,) = {
    if (locals.var_guard18 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvip, locals.var_maxvip_dn4,)
    }
};
        locals.var_maxvip = assign610_e946;
        locals.var_maxvip_dn4 = assign610_e946_d_n4;

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign620_e951: f64 = (p.p123 / p.p56);
        let assign620_e952: f64 = (locals.var_rt).powf(assign620_e951);
        let assign620_e953: f64 = (p.p54 * assign620_e952);
        let assign620_e955: f64 = (-p.p114);
        let assign620_e958: f64 = (1.0 - locals.var_rt);
        let assign620_e959: f64 = (assign620_e955 * assign620_e958);
        let assign620_e962: f64 = (locals.var_vtv * p.p56);
        let assign620_e963: f64 = (assign620_e959 / assign620_e962);
        let assign620_e964: f64 = (assign620_e963).exp();
        let assign620_e965: f64 = (assign620_e953 * assign620_e964);
        locals.var_ibei_t = assign620_e965;
        locals.var_ibei_t_dn4 = (((p.p54 * if 0.0 == 0.0 && ((assign620_e951) as f64).is_finite() && ((assign620_e951) as f64).fract() == 0.0 { if assign620_e951 == 0.0 { 0.0 } else { (assign620_e951 * ((locals.var_rt).powf(assign620_e951 - 1.0) * locals.var_rt_dn4)) } } else { (assign620_e952 * (assign620_e951 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign620_e964) + (assign620_e953 * (assign620_e964 * ((((assign620_e955 * (-locals.var_rt_dn4)) * assign620_e962) - (assign620_e959 * (locals.var_vtv_dn4 * p.p56))) / (assign620_e962 * assign620_e962)))));

        let assign630_e968: f64 = if locals.var_ibei_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign630_e968;

        let (assign640_e981, assign640_e981_d_n4,) = {
    if (locals.var_guard20 != 0.0) {
        let assign640_e972: f64 = (p.p56 * locals.var_vtv);
        let assign640_e976: f64 = (locals.var_imaxmod / locals.var_ibei_t);
        let assign640_e977: f64 = (1.0 + assign640_e976);
        let assign640_e978: f64 = (assign640_e977).ln();
        let assign640_e979: f64 = (assign640_e972 * assign640_e978);
        (assign640_e979, (((p.p56 * locals.var_vtv_dn4) * assign640_e978) + (assign640_e972 * ((-((locals.var_imaxmod * locals.var_ibei_t_dn4) / (locals.var_ibei_t * locals.var_ibei_t))) / assign640_e977))),)
    } else {
        (locals.var_maxvibei, locals.var_maxvibei_dn4,)
    }
};
        locals.var_maxvibei = assign640_e981;
        locals.var_maxvibei_dn4 = assign640_e981_d_n4;

        let (assign650_e986, assign650_e986_d_n4,) = {
    if (locals.var_guard20 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibei, locals.var_maxvibei_dn4,)
    }
};
        locals.var_maxvibei = assign650_e986;
        locals.var_maxvibei_dn4 = assign650_e986_d_n4;

        let assign660_e991: f64 = (p.p124 / p.p59);
        let assign660_e992: f64 = (locals.var_rt).powf(assign660_e991);
        let assign660_e993: f64 = (p.p58 * assign660_e992);
        let assign660_e995: f64 = (-p.p117);
        let assign660_e998: f64 = (1.0 - locals.var_rt);
        let assign660_e999: f64 = (assign660_e995 * assign660_e998);
        let assign660_e1002: f64 = (locals.var_vtv * p.p59);
        let assign660_e1003: f64 = (assign660_e999 / assign660_e1002);
        let assign660_e1004: f64 = (assign660_e1003).exp();
        let assign660_e1005: f64 = (assign660_e993 * assign660_e1004);
        locals.var_iben_t = assign660_e1005;
        locals.var_iben_t_dn4 = (((p.p58 * if 0.0 == 0.0 && ((assign660_e991) as f64).is_finite() && ((assign660_e991) as f64).fract() == 0.0 { if assign660_e991 == 0.0 { 0.0 } else { (assign660_e991 * ((locals.var_rt).powf(assign660_e991 - 1.0) * locals.var_rt_dn4)) } } else { (assign660_e992 * (assign660_e991 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign660_e1004) + (assign660_e993 * (assign660_e1004 * ((((assign660_e995 * (-locals.var_rt_dn4)) * assign660_e1002) - (assign660_e999 * (locals.var_vtv_dn4 * p.p59))) / (assign660_e1002 * assign660_e1002)))));

        let assign670_e1008: f64 = if locals.var_iben_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign670_e1008;

        let (assign680_e1021, assign680_e1021_d_n4,) = {
    if (locals.var_guard21 != 0.0) {
        let assign680_e1012: f64 = (p.p59 * locals.var_vtv);
        let assign680_e1016: f64 = (locals.var_imaxmod / locals.var_iben_t);
        let assign680_e1017: f64 = (1.0 + assign680_e1016);
        let assign680_e1018: f64 = (assign680_e1017).ln();
        let assign680_e1019: f64 = (assign680_e1012 * assign680_e1018);
        (assign680_e1019, (((p.p59 * locals.var_vtv_dn4) * assign680_e1018) + (assign680_e1012 * ((-((locals.var_imaxmod * locals.var_iben_t_dn4) / (locals.var_iben_t * locals.var_iben_t))) / assign680_e1017))),)
    } else {
        (locals.var_maxviben, locals.var_maxviben_dn4,)
    }
};
        locals.var_maxviben = assign680_e1021;
        locals.var_maxviben_dn4 = assign680_e1021_d_n4;

        let (assign690_e1026, assign690_e1026_d_n4,) = {
    if (locals.var_guard21 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxviben, locals.var_maxviben_dn4,)
    }
};
        locals.var_maxviben = assign690_e1026;
        locals.var_maxviben_dn4 = assign690_e1026_d_n4;

        let assign700_e1031: f64 = (p.p123 / p.p61);
        let assign700_e1032: f64 = (locals.var_rt).powf(assign700_e1031);
        let assign700_e1033: f64 = (p.p60 * assign700_e1032);
        let assign700_e1035: f64 = (-p.p115);
        let assign700_e1038: f64 = (1.0 - locals.var_rt);
        let assign700_e1039: f64 = (assign700_e1035 * assign700_e1038);
        let assign700_e1042: f64 = (locals.var_vtv * p.p61);
        let assign700_e1043: f64 = (assign700_e1039 / assign700_e1042);
        let assign700_e1044: f64 = (assign700_e1043).exp();
        let assign700_e1045: f64 = (assign700_e1033 * assign700_e1044);
        locals.var_ibci_t = assign700_e1045;
        locals.var_ibci_t_dn4 = (((p.p60 * if 0.0 == 0.0 && ((assign700_e1031) as f64).is_finite() && ((assign700_e1031) as f64).fract() == 0.0 { if assign700_e1031 == 0.0 { 0.0 } else { (assign700_e1031 * ((locals.var_rt).powf(assign700_e1031 - 1.0) * locals.var_rt_dn4)) } } else { (assign700_e1032 * (assign700_e1031 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign700_e1044) + (assign700_e1033 * (assign700_e1044 * ((((assign700_e1035 * (-locals.var_rt_dn4)) * assign700_e1042) - (assign700_e1039 * (locals.var_vtv_dn4 * p.p61))) / (assign700_e1042 * assign700_e1042)))));

        let assign710_e1048: f64 = if locals.var_ibci_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign710_e1048;

        let (assign720_e1061, assign720_e1061_d_n4,) = {
    if (locals.var_guard22 != 0.0) {
        let assign720_e1052: f64 = (p.p61 * locals.var_vtv);
        let assign720_e1056: f64 = (locals.var_imaxmod / locals.var_ibci_t);
        let assign720_e1057: f64 = (1.0 + assign720_e1056);
        let assign720_e1058: f64 = (assign720_e1057).ln();
        let assign720_e1059: f64 = (assign720_e1052 * assign720_e1058);
        (assign720_e1059, (((p.p61 * locals.var_vtv_dn4) * assign720_e1058) + (assign720_e1052 * ((-((locals.var_imaxmod * locals.var_ibci_t_dn4) / (locals.var_ibci_t * locals.var_ibci_t))) / assign720_e1057))),)
    } else {
        (locals.var_maxvibci, locals.var_maxvibci_dn4,)
    }
};
        locals.var_maxvibci = assign720_e1061;
        locals.var_maxvibci_dn4 = assign720_e1061_d_n4;

        let (assign730_e1066, assign730_e1066_d_n4,) = {
    if (locals.var_guard22 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibci, locals.var_maxvibci_dn4,)
    }
};
        locals.var_maxvibci = assign730_e1066;
        locals.var_maxvibci_dn4 = assign730_e1066_d_n4;

        let assign740_e1071: f64 = (p.p124 / p.p63);
        let assign740_e1072: f64 = (locals.var_rt).powf(assign740_e1071);
        let assign740_e1073: f64 = (p.p62 * assign740_e1072);
        let assign740_e1075: f64 = (-p.p118);
        let assign740_e1078: f64 = (1.0 - locals.var_rt);
        let assign740_e1079: f64 = (assign740_e1075 * assign740_e1078);
        let assign740_e1082: f64 = (locals.var_vtv * p.p63);
        let assign740_e1083: f64 = (assign740_e1079 / assign740_e1082);
        let assign740_e1084: f64 = (assign740_e1083).exp();
        let assign740_e1085: f64 = (assign740_e1073 * assign740_e1084);
        locals.var_ibcn_t = assign740_e1085;
        locals.var_ibcn_t_dn4 = (((p.p62 * if 0.0 == 0.0 && ((assign740_e1071) as f64).is_finite() && ((assign740_e1071) as f64).fract() == 0.0 { if assign740_e1071 == 0.0 { 0.0 } else { (assign740_e1071 * ((locals.var_rt).powf(assign740_e1071 - 1.0) * locals.var_rt_dn4)) } } else { (assign740_e1072 * (assign740_e1071 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign740_e1084) + (assign740_e1073 * (assign740_e1084 * ((((assign740_e1075 * (-locals.var_rt_dn4)) * assign740_e1082) - (assign740_e1079 * (locals.var_vtv_dn4 * p.p63))) / (assign740_e1082 * assign740_e1082)))));

        let assign750_e1088: f64 = if locals.var_ibcn_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign750_e1088;

        let (assign760_e1101, assign760_e1101_d_n4,) = {
    if (locals.var_guard23 != 0.0) {
        let assign760_e1092: f64 = (p.p63 * locals.var_vtv);
        let assign760_e1096: f64 = (locals.var_imaxmod / locals.var_ibcn_t);
        let assign760_e1097: f64 = (1.0 + assign760_e1096);
        let assign760_e1098: f64 = (assign760_e1097).ln();
        let assign760_e1099: f64 = (assign760_e1092 * assign760_e1098);
        (assign760_e1099, (((p.p63 * locals.var_vtv_dn4) * assign760_e1098) + (assign760_e1092 * ((-((locals.var_imaxmod * locals.var_ibcn_t_dn4) / (locals.var_ibcn_t * locals.var_ibcn_t))) / assign760_e1097))),)
    } else {
        (locals.var_maxvibcn, locals.var_maxvibcn_dn4,)
    }
};
        locals.var_maxvibcn = assign760_e1101;
        locals.var_maxvibcn_dn4 = assign760_e1101_d_n4;

        let (assign770_e1106, assign770_e1106_d_n4,) = {
    if (locals.var_guard23 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibcn, locals.var_maxvibcn_dn4,)
    }
};
        locals.var_maxvibcn = assign770_e1106;
        locals.var_maxvibcn_dn4 = assign770_e1106_d_n4;

        let assign780_e1111: f64 = (p.p123 / p.p61);
        let assign780_e1112: f64 = (locals.var_rt).powf(assign780_e1111);
        let assign780_e1113: f64 = (p.p64 * assign780_e1112);
        let assign780_e1115: f64 = (-p.p115);
        let assign780_e1118: f64 = (1.0 - locals.var_rt);
        let assign780_e1119: f64 = (assign780_e1115 * assign780_e1118);
        let assign780_e1122: f64 = (locals.var_vtv * p.p61);
        let assign780_e1123: f64 = (assign780_e1119 / assign780_e1122);
        let assign780_e1124: f64 = (assign780_e1123).exp();
        let assign780_e1125: f64 = (assign780_e1113 * assign780_e1124);
        locals.var_ibeip_t = assign780_e1125;
        locals.var_ibeip_t_dn4 = (((p.p64 * if 0.0 == 0.0 && ((assign780_e1111) as f64).is_finite() && ((assign780_e1111) as f64).fract() == 0.0 { if assign780_e1111 == 0.0 { 0.0 } else { (assign780_e1111 * ((locals.var_rt).powf(assign780_e1111 - 1.0) * locals.var_rt_dn4)) } } else { (assign780_e1112 * (assign780_e1111 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign780_e1124) + (assign780_e1113 * (assign780_e1124 * ((((assign780_e1115 * (-locals.var_rt_dn4)) * assign780_e1122) - (assign780_e1119 * (locals.var_vtv_dn4 * p.p61))) / (assign780_e1122 * assign780_e1122)))));

        let assign790_e1128: f64 = if locals.var_ibeip_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign790_e1128;

        let (assign800_e1141, assign800_e1141_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign800_e1132: f64 = (p.p61 * locals.var_vtv);
        let assign800_e1136: f64 = (locals.var_imaxmod / locals.var_ibeip_t);
        let assign800_e1137: f64 = (1.0 + assign800_e1136);
        let assign800_e1138: f64 = (assign800_e1137).ln();
        let assign800_e1139: f64 = (assign800_e1132 * assign800_e1138);
        (assign800_e1139, (((p.p61 * locals.var_vtv_dn4) * assign800_e1138) + (assign800_e1132 * ((-((locals.var_imaxmod * locals.var_ibeip_t_dn4) / (locals.var_ibeip_t * locals.var_ibeip_t))) / assign800_e1137))),)
    } else {
        (locals.var_maxvibeip, locals.var_maxvibeip_dn4,)
    }
};
        locals.var_maxvibeip = assign800_e1141;
        locals.var_maxvibeip_dn4 = assign800_e1141_d_n4;

        let (assign810_e1146, assign810_e1146_d_n4,) = {
    if (locals.var_guard24 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibeip, locals.var_maxvibeip_dn4,)
    }
};
        locals.var_maxvibeip = assign810_e1146;
        locals.var_maxvibeip_dn4 = assign810_e1146_d_n4;

        let assign820_e1151: f64 = (p.p124 / p.p63);
        let assign820_e1152: f64 = (locals.var_rt).powf(assign820_e1151);
        let assign820_e1153: f64 = (p.p65 * assign820_e1152);
        let assign820_e1155: f64 = (-p.p118);
        let assign820_e1158: f64 = (1.0 - locals.var_rt);
        let assign820_e1159: f64 = (assign820_e1155 * assign820_e1158);
        let assign820_e1162: f64 = (locals.var_vtv * p.p63);
        let assign820_e1163: f64 = (assign820_e1159 / assign820_e1162);
        let assign820_e1164: f64 = (assign820_e1163).exp();
        let assign820_e1165: f64 = (assign820_e1153 * assign820_e1164);
        locals.var_ibenp_t = assign820_e1165;
        locals.var_ibenp_t_dn4 = (((p.p65 * if 0.0 == 0.0 && ((assign820_e1151) as f64).is_finite() && ((assign820_e1151) as f64).fract() == 0.0 { if assign820_e1151 == 0.0 { 0.0 } else { (assign820_e1151 * ((locals.var_rt).powf(assign820_e1151 - 1.0) * locals.var_rt_dn4)) } } else { (assign820_e1152 * (assign820_e1151 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign820_e1164) + (assign820_e1153 * (assign820_e1164 * ((((assign820_e1155 * (-locals.var_rt_dn4)) * assign820_e1162) - (assign820_e1159 * (locals.var_vtv_dn4 * p.p63))) / (assign820_e1162 * assign820_e1162)))));

        let assign830_e1168: f64 = if locals.var_ibenp_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign830_e1168;

        let (assign840_e1181, assign840_e1181_d_n4,) = {
    if (locals.var_guard25 != 0.0) {
        let assign840_e1172: f64 = (p.p63 * locals.var_vtv);
        let assign840_e1176: f64 = (locals.var_imaxmod / locals.var_ibenp_t);
        let assign840_e1177: f64 = (1.0 + assign840_e1176);
        let assign840_e1178: f64 = (assign840_e1177).ln();
        let assign840_e1179: f64 = (assign840_e1172 * assign840_e1178);
        (assign840_e1179, (((p.p63 * locals.var_vtv_dn4) * assign840_e1178) + (assign840_e1172 * ((-((locals.var_imaxmod * locals.var_ibenp_t_dn4) / (locals.var_ibenp_t * locals.var_ibenp_t))) / assign840_e1177))),)
    } else {
        (locals.var_maxvibenp, locals.var_maxvibenp_dn4,)
    }
};
        locals.var_maxvibenp = assign840_e1181;
        locals.var_maxvibenp_dn4 = assign840_e1181_d_n4;

        let (assign850_e1186, assign850_e1186_d_n4,) = {
    if (locals.var_guard25 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibenp, locals.var_maxvibenp_dn4,)
    }
};
        locals.var_maxvibenp = assign850_e1186;
        locals.var_maxvibenp_dn4 = assign850_e1186_d_n4;

        let assign860_e1191: f64 = (p.p123 / p.p67);
        let assign860_e1192: f64 = (locals.var_rt).powf(assign860_e1191);
        let assign860_e1193: f64 = (p.p66 * assign860_e1192);
        let assign860_e1195: f64 = (-p.p116);
        let assign860_e1198: f64 = (1.0 - locals.var_rt);
        let assign860_e1199: f64 = (assign860_e1195 * assign860_e1198);
        let assign860_e1202: f64 = (locals.var_vtv * p.p67);
        let assign860_e1203: f64 = (assign860_e1199 / assign860_e1202);
        let assign860_e1204: f64 = (assign860_e1203).exp();
        let assign860_e1205: f64 = (assign860_e1193 * assign860_e1204);
        locals.var_ibcip_t = assign860_e1205;
        locals.var_ibcip_t_dn4 = (((p.p66 * if 0.0 == 0.0 && ((assign860_e1191) as f64).is_finite() && ((assign860_e1191) as f64).fract() == 0.0 { if assign860_e1191 == 0.0 { 0.0 } else { (assign860_e1191 * ((locals.var_rt).powf(assign860_e1191 - 1.0) * locals.var_rt_dn4)) } } else { (assign860_e1192 * (assign860_e1191 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign860_e1204) + (assign860_e1193 * (assign860_e1204 * ((((assign860_e1195 * (-locals.var_rt_dn4)) * assign860_e1202) - (assign860_e1199 * (locals.var_vtv_dn4 * p.p67))) / (assign860_e1202 * assign860_e1202)))));

        let assign870_e1208: f64 = if locals.var_ibcip_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign870_e1208;

        let (assign880_e1221, assign880_e1221_d_n4,) = {
    if (locals.var_guard26 != 0.0) {
        let assign880_e1212: f64 = (p.p67 * locals.var_vtv);
        let assign880_e1216: f64 = (locals.var_imaxmod / locals.var_ibcip_t);
        let assign880_e1217: f64 = (1.0 + assign880_e1216);
        let assign880_e1218: f64 = (assign880_e1217).ln();
        let assign880_e1219: f64 = (assign880_e1212 * assign880_e1218);
        (assign880_e1219, (((p.p67 * locals.var_vtv_dn4) * assign880_e1218) + (assign880_e1212 * ((-((locals.var_imaxmod * locals.var_ibcip_t_dn4) / (locals.var_ibcip_t * locals.var_ibcip_t))) / assign880_e1217))),)
    } else {
        (locals.var_maxvibcip, locals.var_maxvibcip_dn4,)
    }
};
        locals.var_maxvibcip = assign880_e1221;
        locals.var_maxvibcip_dn4 = assign880_e1221_d_n4;

        let (assign890_e1226, assign890_e1226_d_n4,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibcip, locals.var_maxvibcip_dn4,)
    }
};
        locals.var_maxvibcip = assign890_e1226;
        locals.var_maxvibcip_dn4 = assign890_e1226_d_n4;

        let assign900_e1231: f64 = (p.p124 / p.p69);
        let assign900_e1232: f64 = (locals.var_rt).powf(assign900_e1231);
        let assign900_e1233: f64 = (p.p68 * assign900_e1232);
        let assign900_e1235: f64 = (-p.p119);
        let assign900_e1238: f64 = (1.0 - locals.var_rt);
        let assign900_e1239: f64 = (assign900_e1235 * assign900_e1238);
        let assign900_e1242: f64 = (locals.var_vtv * p.p69);
        let assign900_e1243: f64 = (assign900_e1239 / assign900_e1242);
        let assign900_e1244: f64 = (assign900_e1243).exp();
        let assign900_e1245: f64 = (assign900_e1233 * assign900_e1244);
        locals.var_ibcnp_t = assign900_e1245;
        locals.var_ibcnp_t_dn4 = (((p.p68 * if 0.0 == 0.0 && ((assign900_e1231) as f64).is_finite() && ((assign900_e1231) as f64).fract() == 0.0 { if assign900_e1231 == 0.0 { 0.0 } else { (assign900_e1231 * ((locals.var_rt).powf(assign900_e1231 - 1.0) * locals.var_rt_dn4)) } } else { (assign900_e1232 * (assign900_e1231 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign900_e1244) + (assign900_e1233 * (assign900_e1244 * ((((assign900_e1235 * (-locals.var_rt_dn4)) * assign900_e1242) - (assign900_e1239 * (locals.var_vtv_dn4 * p.p69))) / (assign900_e1242 * assign900_e1242)))));

        let assign910_e1248: f64 = if locals.var_ibcnp_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign910_e1248;

        let (assign920_e1261, assign920_e1261_d_n4,) = {
    if (locals.var_guard27 != 0.0) {
        let assign920_e1252: f64 = (p.p69 * locals.var_vtv);
        let assign920_e1256: f64 = (locals.var_imaxmod / locals.var_ibcnp_t);
        let assign920_e1257: f64 = (1.0 + assign920_e1256);
        let assign920_e1258: f64 = (assign920_e1257).ln();
        let assign920_e1259: f64 = (assign920_e1252 * assign920_e1258);
        (assign920_e1259, (((p.p69 * locals.var_vtv_dn4) * assign920_e1258) + (assign920_e1252 * ((-((locals.var_imaxmod * locals.var_ibcnp_t_dn4) / (locals.var_ibcnp_t * locals.var_ibcnp_t))) / assign920_e1257))),)
    } else {
        (locals.var_maxvibcnp, locals.var_maxvibcnp_dn4,)
    }
};
        locals.var_maxvibcnp = assign920_e1261;
        locals.var_maxvibcnp_dn4 = assign920_e1261_d_n4;

        let (assign930_e1266, assign930_e1266_d_n4,) = {
    if (locals.var_guard27 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibcnp, locals.var_maxvibcnp_dn4,)
    }
};
        locals.var_maxvibcnp = assign930_e1266;
        locals.var_maxvibcnp_dn4 = assign930_e1266_d_n4;

        locals.var_dt_et = (nv4 - 0.0);
        locals.var_dt_et_dn4 = 1.0;

        let assign950_e1268: f64 = ctx_temp;
        let assign950_e1270: f64 = (assign950_e1268 + p.p0);
        let assign950_e1272: f64 = (assign950_e1270 + locals.var_dt_et);
        let assign950_e1274: f64 = (assign950_e1272 - 273.15);
        locals.var_tdevc = assign950_e1274;
        locals.var_tdevc_dn4 = locals.var_dt_et_dn4;

        let assign960_e1278: f64 = (p.p14 + 1.0);
        let assign960_e1279: f64 = if locals.var_tdevc < assign960_e1278 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign960_e1279;

        let (assign970_e1290, assign970_e1290_d_n4,) = {
    if (locals.var_guard28 != 0.0) {
        let assign970_e1284: f64 = (locals.var_tdevc - p.p14);
        let assign970_e1286: f64 = (assign970_e1284 - 1.0);
        let assign970_e1287: f64 = (assign970_e1286).exp();
        let assign970_e1288: f64 = (p.p14 + assign970_e1287);
        (assign970_e1288, (assign970_e1287 * locals.var_tdevc_dn4),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign970_e1290;
        locals.var_tdevc_dn4 = assign970_e1290_d_n4;

        let assign980_e1294: f64 = (p.p15 - 1.0);
        let assign980_e1295: f64 = if locals.var_tdevc > assign980_e1294 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign980_e1295;

        let (assign990_e1309, assign990_e1309_d_n4,) = {
    if ((locals.var_guard28 == 0.0) && (locals.var_guard29 != 0.0)) {
        let assign990_e1303: f64 = (p.p15 - locals.var_tdevc);
        let assign990_e1305: f64 = (assign990_e1303 - 1.0);
        let assign990_e1306: f64 = (assign990_e1305).exp();
        let assign990_e1307: f64 = (p.p15 - assign990_e1306);
        (assign990_e1307, (-(assign990_e1306 * (-locals.var_tdevc_dn4))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign990_e1309;
        locals.var_tdevc_dn4 = assign990_e1309_d_n4;

        let (assign1000_e1317, assign1000_e1317_d_n4,) = {
    if ((locals.var_guard28 == 0.0) && (locals.var_guard29 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign1000_e1317;
        locals.var_tdevc_dn4 = assign1000_e1317_d_n4;

        let assign1010_e1320: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign1010_e1320;
        locals.var_tdevk_dn4 = locals.var_tdevc_dn4;

        let assign1020_e1323: f64 = (1.380662e-23 * locals.var_tdevk);
        let assign1020_e1325: f64 = (assign1020_e1323 / 1.602189e-19);
        locals.var_vtv = assign1020_e1325;
        locals.var_vtv_dn4 = ((1.380662e-23 * locals.var_tdevk_dn4) / 1.602189e-19);

        let assign1030_e1328: f64 = (locals.var_tdevk / locals.var_tinik);
        locals.var_rt = assign1030_e1328;
        locals.var_rt_dn4 = (locals.var_tdevk_dn4 / locals.var_tinik);

        let assign1040_e1331: f64 = (locals.var_tdevk - locals.var_tinik);
        locals.var_dt = assign1040_e1331;
        locals.var_dt_dn4 = locals.var_tdevk_dn4;

        let assign1050_e1335: f64 = (locals.var_rt).powf(p.p126);
        let assign1050_e1336: f64 = (p.p72 * assign1050_e1335);
        locals.var_ikf_t = assign1050_e1336;
        locals.var_ikf_t_dn4 = (p.p72 * if 0.0 == 0.0 && ((p.p126) as f64).is_finite() && ((p.p126) as f64).fract() == 0.0 { if p.p126 == 0.0 { 0.0 } else { (p.p126 * ((locals.var_rt).powf(p.p126 - 1.0) * locals.var_rt_dn4)) } } else { (assign1050_e1335 * (p.p126 * (locals.var_rt_dn4 / locals.var_rt))) });

        let assign1060_e1338: f64 = if param_given[109] { 1.0 } else { 0.0 };
        locals.var_guard30 = assign1060_e1338;

        let (assign1070_e1346, assign1070_e1346_d_n4,) = {
    if (locals.var_guard30 != 0.0) {
        let assign1070_e1343: f64 = (locals.var_rt).powf(p.p109);
        let assign1070_e1344: f64 = (p.p16 * assign1070_e1343);
        (assign1070_e1344, (p.p16 * if 0.0 == 0.0 && ((p.p109) as f64).is_finite() && ((p.p109) as f64).fract() == 0.0 { if p.p109 == 0.0 { 0.0 } else { (p.p109 * ((locals.var_rt).powf(p.p109 - 1.0) * locals.var_rt_dn4)) } } else { (assign1070_e1343 * (p.p109 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rcx_t, locals.var_rcx_t_dn4,)
    }
};
        locals.var_rcx_t = assign1070_e1346;
        locals.var_rcx_t_dn4 = assign1070_e1346_d_n4;

        let (assign1080_e1355, assign1080_e1355_d_n4,) = {
    if (locals.var_guard30 == 0.0) {
        let assign1080_e1352: f64 = (locals.var_rt).powf(p.p107);
        let assign1080_e1353: f64 = (p.p16 * assign1080_e1352);
        (assign1080_e1353, (p.p16 * if 0.0 == 0.0 && ((p.p107) as f64).is_finite() && ((p.p107) as f64).fract() == 0.0 { if p.p107 == 0.0 { 0.0 } else { (p.p107 * ((locals.var_rt).powf(p.p107 - 1.0) * locals.var_rt_dn4)) } } else { (assign1080_e1352 * (p.p107 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rcx_t, locals.var_rcx_t_dn4,)
    }
};
        locals.var_rcx_t = assign1080_e1355;
        locals.var_rcx_t_dn4 = assign1080_e1355_d_n4;

        let assign1090_e1357: f64 = if param_given[108] { 1.0 } else { 0.0 };
        locals.var_guard31 = assign1090_e1357;

        let (assign1100_e1365, assign1100_e1365_d_n4,) = {
    if (locals.var_guard31 != 0.0) {
        let assign1100_e1362: f64 = (locals.var_rt).powf(p.p108);
        let assign1100_e1363: f64 = (p.p17 * assign1100_e1362);
        (assign1100_e1363, (p.p17 * if 0.0 == 0.0 && ((p.p108) as f64).is_finite() && ((p.p108) as f64).fract() == 0.0 { if p.p108 == 0.0 { 0.0 } else { (p.p108 * ((locals.var_rt).powf(p.p108 - 1.0) * locals.var_rt_dn4)) } } else { (assign1100_e1362 * (p.p108 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rci_t, locals.var_rci_t_dn4,)
    }
};
        locals.var_rci_t = assign1100_e1365;
        locals.var_rci_t_dn4 = assign1100_e1365_d_n4;

        let (assign1110_e1374, assign1110_e1374_d_n4,) = {
    if (locals.var_guard31 == 0.0) {
        let assign1110_e1371: f64 = (locals.var_rt).powf(p.p107);
        let assign1110_e1372: f64 = (p.p17 * assign1110_e1371);
        (assign1110_e1372, (p.p17 * if 0.0 == 0.0 && ((p.p107) as f64).is_finite() && ((p.p107) as f64).fract() == 0.0 { if p.p107 == 0.0 { 0.0 } else { (p.p107 * ((locals.var_rt).powf(p.p107 - 1.0) * locals.var_rt_dn4)) } } else { (assign1110_e1371 * (p.p107 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rci_t, locals.var_rci_t_dn4,)
    }
};
        locals.var_rci_t = assign1110_e1374;
        locals.var_rci_t_dn4 = assign1110_e1374_d_n4;

        let assign1120_e1376: f64 = if param_given[106] { 1.0 } else { 0.0 };
        locals.var_guard32 = assign1120_e1376;

        let (assign1130_e1384, assign1130_e1384_d_n4,) = {
    if (locals.var_guard32 != 0.0) {
        let assign1130_e1381: f64 = (locals.var_rt).powf(p.p106);
        let assign1130_e1382: f64 = (p.p21 * assign1130_e1381);
        (assign1130_e1382, (p.p21 * if 0.0 == 0.0 && ((p.p106) as f64).is_finite() && ((p.p106) as f64).fract() == 0.0 { if p.p106 == 0.0 { 0.0 } else { (p.p106 * ((locals.var_rt).powf(p.p106 - 1.0) * locals.var_rt_dn4)) } } else { (assign1130_e1381 * (p.p106 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rbx_t, locals.var_rbx_t_dn4,)
    }
};
        locals.var_rbx_t = assign1130_e1384;
        locals.var_rbx_t_dn4 = assign1130_e1384_d_n4;

        let (assign1140_e1393, assign1140_e1393_d_n4,) = {
    if (locals.var_guard32 == 0.0) {
        let assign1140_e1390: f64 = (locals.var_rt).powf(p.p104);
        let assign1140_e1391: f64 = (p.p21 * assign1140_e1390);
        (assign1140_e1391, (p.p21 * if 0.0 == 0.0 && ((p.p104) as f64).is_finite() && ((p.p104) as f64).fract() == 0.0 { if p.p104 == 0.0 { 0.0 } else { (p.p104 * ((locals.var_rt).powf(p.p104 - 1.0) * locals.var_rt_dn4)) } } else { (assign1140_e1390 * (p.p104 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rbx_t, locals.var_rbx_t_dn4,)
    }
};
        locals.var_rbx_t = assign1140_e1393;
        locals.var_rbx_t_dn4 = assign1140_e1393_d_n4;

        let assign1150_e1395: f64 = if param_given[105] { 1.0 } else { 0.0 };
        locals.var_guard33 = assign1150_e1395;

        let (assign1160_e1403, assign1160_e1403_d_n4,) = {
    if (locals.var_guard33 != 0.0) {
        let assign1160_e1400: f64 = (locals.var_rt).powf(p.p105);
        let assign1160_e1401: f64 = (p.p22 * assign1160_e1400);
        (assign1160_e1401, (p.p22 * if 0.0 == 0.0 && ((p.p105) as f64).is_finite() && ((p.p105) as f64).fract() == 0.0 { if p.p105 == 0.0 { 0.0 } else { (p.p105 * ((locals.var_rt).powf(p.p105 - 1.0) * locals.var_rt_dn4)) } } else { (assign1160_e1400 * (p.p105 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rbi_t, locals.var_rbi_t_dn4,)
    }
};
        locals.var_rbi_t = assign1160_e1403;
        locals.var_rbi_t_dn4 = assign1160_e1403_d_n4;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign1170_e1412, assign1170_e1412_d_n4,) = {
    if (locals.var_guard33 == 0.0) {
        let assign1170_e1409: f64 = (locals.var_rt).powf(p.p104);
        let assign1170_e1410: f64 = (p.p22 * assign1170_e1409);
        (assign1170_e1410, (p.p22 * if 0.0 == 0.0 && ((p.p104) as f64).is_finite() && ((p.p104) as f64).fract() == 0.0 { if p.p104 == 0.0 { 0.0 } else { (p.p104 * ((locals.var_rt).powf(p.p104 - 1.0) * locals.var_rt_dn4)) } } else { (assign1170_e1409 * (p.p104 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rbi_t, locals.var_rbi_t_dn4,)
    }
};
        locals.var_rbi_t = assign1170_e1412;
        locals.var_rbi_t_dn4 = assign1170_e1412_d_n4;

        let assign1180_e1416: f64 = (locals.var_rt).powf(p.p103);
        let assign1180_e1417: f64 = (p.p23 * assign1180_e1416);
        locals.var_re_t = assign1180_e1417;
        locals.var_re_t_dn4 = (p.p23 * if 0.0 == 0.0 && ((p.p103) as f64).is_finite() && ((p.p103) as f64).fract() == 0.0 { if p.p103 == 0.0 { 0.0 } else { (p.p103 * ((locals.var_rt).powf(p.p103 - 1.0) * locals.var_rt_dn4)) } } else { (assign1180_e1416 * (p.p103 * (locals.var_rt_dn4 / locals.var_rt))) });

        let assign1190_e1421: f64 = (locals.var_rt).powf(p.p111);
        let assign1190_e1422: f64 = (p.p24 * assign1190_e1421);
        locals.var_rs_t = assign1190_e1422;
        locals.var_rs_t_dn4 = (p.p24 * if 0.0 == 0.0 && ((p.p111) as f64).is_finite() && ((p.p111) as f64).fract() == 0.0 { if p.p111 == 0.0 { 0.0 } else { (p.p111 * ((locals.var_rt).powf(p.p111 - 1.0) * locals.var_rt_dn4)) } } else { (assign1190_e1421 * (p.p111 * (locals.var_rt_dn4 / locals.var_rt))) });

        let assign1200_e1424: f64 = if param_given[110] { 1.0 } else { 0.0 };
        locals.var_guard34 = assign1200_e1424;

        let (assign1210_e1432, assign1210_e1432_d_n4,) = {
    if (locals.var_guard34 != 0.0) {
        let assign1210_e1429: f64 = (locals.var_rt).powf(p.p110);
        let assign1210_e1430: f64 = (p.p25 * assign1210_e1429);
        (assign1210_e1430, (p.p25 * if 0.0 == 0.0 && ((p.p110) as f64).is_finite() && ((p.p110) as f64).fract() == 0.0 { if p.p110 == 0.0 { 0.0 } else { (p.p110 * ((locals.var_rt).powf(p.p110 - 1.0) * locals.var_rt_dn4)) } } else { (assign1210_e1429 * (p.p110 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rbp_t, locals.var_rbp_t_dn4,)
    }
};
        locals.var_rbp_t = assign1210_e1432;
        locals.var_rbp_t_dn4 = assign1210_e1432_d_n4;

        let (assign1220_e1441, assign1220_e1441_d_n4,) = {
    if (locals.var_guard34 == 0.0) {
        let assign1220_e1438: f64 = (locals.var_rt).powf(p.p107);
        let assign1220_e1439: f64 = (p.p25 * assign1220_e1438);
        (assign1220_e1439, (p.p25 * if 0.0 == 0.0 && ((p.p107) as f64).is_finite() && ((p.p107) as f64).fract() == 0.0 { if p.p107 == 0.0 { 0.0 } else { (p.p107 * ((locals.var_rt).powf(p.p107 - 1.0) * locals.var_rt_dn4)) } } else { (assign1220_e1438 * (p.p107 * (locals.var_rt_dn4 / locals.var_rt))) }),)
    } else {
        (locals.var_rbp_t, locals.var_rbp_t_dn4,)
    }
};
        locals.var_rbp_t = assign1220_e1441;
        locals.var_rbp_t_dn4 = assign1220_e1441_d_n4;

        let assign1230_e1446: f64 = (locals.var_dt * p.p132);
        let assign1230_e1447: f64 = (1.0 + assign1230_e1446);
        let assign1230_e1448: f64 = (p.p101 * assign1230_e1447);
        locals.var_rth_t = assign1230_e1448;
        locals.var_rth_t_dn4 = (p.p101 * (locals.var_dt_dn4 * p.p132));

        let assign1240_e1453: f64 = (p.p122 / p.p28);
        let assign1240_e1454: f64 = (locals.var_rt).powf(assign1240_e1453);
        let assign1240_e1455: f64 = (p.p26 * assign1240_e1454);
        let assign1240_e1457: f64 = (-p.p113);
        let assign1240_e1460: f64 = (1.0 - locals.var_rt);
        let assign1240_e1461: f64 = (assign1240_e1457 * assign1240_e1460);
        let assign1240_e1464: f64 = (locals.var_vtv * p.p28);
        let assign1240_e1465: f64 = (assign1240_e1461 / assign1240_e1464);
        let assign1240_e1466: f64 = (assign1240_e1465).exp();
        let assign1240_e1467: f64 = (assign1240_e1455 * assign1240_e1466);
        locals.var_is_t = assign1240_e1467;
        locals.var_is_t_dn4 = (((p.p26 * if 0.0 == 0.0 && ((assign1240_e1453) as f64).is_finite() && ((assign1240_e1453) as f64).fract() == 0.0 { if assign1240_e1453 == 0.0 { 0.0 } else { (assign1240_e1453 * ((locals.var_rt).powf(assign1240_e1453 - 1.0) * locals.var_rt_dn4)) } } else { (assign1240_e1454 * (assign1240_e1453 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1240_e1466) + (assign1240_e1455 * (assign1240_e1466 * ((((assign1240_e1457 * (-locals.var_rt_dn4)) * assign1240_e1464) - (assign1240_e1461 * (locals.var_vtv_dn4 * p.p28))) / (assign1240_e1464 * assign1240_e1464)))));

        let assign1250_e1472: f64 = (p.p125 / p.p29);
        let assign1250_e1473: f64 = (locals.var_rt).powf(assign1250_e1472);
        let assign1250_e1474: f64 = (p.p27 * assign1250_e1473);
        let assign1250_e1476: f64 = (-p.p121);
        let assign1250_e1479: f64 = (1.0 - locals.var_rt);
        let assign1250_e1480: f64 = (assign1250_e1476 * assign1250_e1479);
        let assign1250_e1483: f64 = (locals.var_vtv * p.p29);
        let assign1250_e1484: f64 = (assign1250_e1480 / assign1250_e1483);
        let assign1250_e1485: f64 = (assign1250_e1484).exp();
        let assign1250_e1486: f64 = (assign1250_e1474 * assign1250_e1485);
        locals.var_isrr_t = assign1250_e1486;
        locals.var_isrr_t_dn4 = (((p.p27 * if 0.0 == 0.0 && ((assign1250_e1472) as f64).is_finite() && ((assign1250_e1472) as f64).fract() == 0.0 { if assign1250_e1472 == 0.0 { 0.0 } else { (assign1250_e1472 * ((locals.var_rt).powf(assign1250_e1472 - 1.0) * locals.var_rt_dn4)) } } else { (assign1250_e1473 * (assign1250_e1472 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1250_e1485) + (assign1250_e1474 * (assign1250_e1485 * ((((assign1250_e1476 * (-locals.var_rt_dn4)) * assign1250_e1483) - (assign1250_e1480 * (locals.var_vtv_dn4 * p.p29))) / (assign1250_e1483 * assign1250_e1483)))));

        let assign1260_e1491: f64 = (p.p122 / p.p33);
        let assign1260_e1492: f64 = (locals.var_rt).powf(assign1260_e1491);
        let assign1260_e1493: f64 = (p.p31 * assign1260_e1492);
        let assign1260_e1495: f64 = (-p.p120);
        let assign1260_e1498: f64 = (1.0 - locals.var_rt);
        let assign1260_e1499: f64 = (assign1260_e1495 * assign1260_e1498);
        let assign1260_e1502: f64 = (locals.var_vtv * p.p33);
        let assign1260_e1503: f64 = (assign1260_e1499 / assign1260_e1502);
        let assign1260_e1504: f64 = (assign1260_e1503).exp();
        let assign1260_e1505: f64 = (assign1260_e1493 * assign1260_e1504);
        locals.var_isp_t = assign1260_e1505;
        locals.var_isp_t_dn4 = (((p.p31 * if 0.0 == 0.0 && ((assign1260_e1491) as f64).is_finite() && ((assign1260_e1491) as f64).fract() == 0.0 { if assign1260_e1491 == 0.0 { 0.0 } else { (assign1260_e1491 * ((locals.var_rt).powf(assign1260_e1491 - 1.0) * locals.var_rt_dn4)) } } else { (assign1260_e1492 * (assign1260_e1491 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1260_e1504) + (assign1260_e1493 * (assign1260_e1504 * ((((assign1260_e1495 * (-locals.var_rt_dn4)) * assign1260_e1502) - (assign1260_e1499 * (locals.var_vtv_dn4 * p.p33))) / (assign1260_e1502 * assign1260_e1502)))));

        let assign1270_e1510: f64 = (p.p123 / p.p56);
        let assign1270_e1511: f64 = (locals.var_rt).powf(assign1270_e1510);
        let assign1270_e1512: f64 = (p.p54 * assign1270_e1511);
        let assign1270_e1514: f64 = (-p.p114);
        let assign1270_e1517: f64 = (1.0 - locals.var_rt);
        let assign1270_e1518: f64 = (assign1270_e1514 * assign1270_e1517);
        let assign1270_e1521: f64 = (locals.var_vtv * p.p56);
        let assign1270_e1522: f64 = (assign1270_e1518 / assign1270_e1521);
        let assign1270_e1523: f64 = (assign1270_e1522).exp();
        let assign1270_e1524: f64 = (assign1270_e1512 * assign1270_e1523);
        locals.var_ibei_t = assign1270_e1524;
        locals.var_ibei_t_dn4 = (((p.p54 * if 0.0 == 0.0 && ((assign1270_e1510) as f64).is_finite() && ((assign1270_e1510) as f64).fract() == 0.0 { if assign1270_e1510 == 0.0 { 0.0 } else { (assign1270_e1510 * ((locals.var_rt).powf(assign1270_e1510 - 1.0) * locals.var_rt_dn4)) } } else { (assign1270_e1511 * (assign1270_e1510 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1270_e1523) + (assign1270_e1512 * (assign1270_e1523 * ((((assign1270_e1514 * (-locals.var_rt_dn4)) * assign1270_e1521) - (assign1270_e1518 * (locals.var_vtv_dn4 * p.p56))) / (assign1270_e1521 * assign1270_e1521)))));

        let assign1280_e1529: f64 = (p.p124 / p.p59);
        let assign1280_e1530: f64 = (locals.var_rt).powf(assign1280_e1529);
        let assign1280_e1531: f64 = (p.p58 * assign1280_e1530);
        let assign1280_e1533: f64 = (-p.p117);
        let assign1280_e1536: f64 = (1.0 - locals.var_rt);
        let assign1280_e1537: f64 = (assign1280_e1533 * assign1280_e1536);
        let assign1280_e1540: f64 = (locals.var_vtv * p.p59);
        let assign1280_e1541: f64 = (assign1280_e1537 / assign1280_e1540);
        let assign1280_e1542: f64 = (assign1280_e1541).exp();
        let assign1280_e1543: f64 = (assign1280_e1531 * assign1280_e1542);
        locals.var_iben_t = assign1280_e1543;
        locals.var_iben_t_dn4 = (((p.p58 * if 0.0 == 0.0 && ((assign1280_e1529) as f64).is_finite() && ((assign1280_e1529) as f64).fract() == 0.0 { if assign1280_e1529 == 0.0 { 0.0 } else { (assign1280_e1529 * ((locals.var_rt).powf(assign1280_e1529 - 1.0) * locals.var_rt_dn4)) } } else { (assign1280_e1530 * (assign1280_e1529 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1280_e1542) + (assign1280_e1531 * (assign1280_e1542 * ((((assign1280_e1533 * (-locals.var_rt_dn4)) * assign1280_e1540) - (assign1280_e1537 * (locals.var_vtv_dn4 * p.p59))) / (assign1280_e1540 * assign1280_e1540)))));

        let assign1290_e1548: f64 = (p.p123 / p.p61);
        let assign1290_e1549: f64 = (locals.var_rt).powf(assign1290_e1548);
        let assign1290_e1550: f64 = (p.p60 * assign1290_e1549);
        let assign1290_e1552: f64 = (-p.p115);
        let assign1290_e1555: f64 = (1.0 - locals.var_rt);
        let assign1290_e1556: f64 = (assign1290_e1552 * assign1290_e1555);
        let assign1290_e1559: f64 = (locals.var_vtv * p.p61);
        let assign1290_e1560: f64 = (assign1290_e1556 / assign1290_e1559);
        let assign1290_e1561: f64 = (assign1290_e1560).exp();
        let assign1290_e1562: f64 = (assign1290_e1550 * assign1290_e1561);
        locals.var_ibci_t = assign1290_e1562;
        locals.var_ibci_t_dn4 = (((p.p60 * if 0.0 == 0.0 && ((assign1290_e1548) as f64).is_finite() && ((assign1290_e1548) as f64).fract() == 0.0 { if assign1290_e1548 == 0.0 { 0.0 } else { (assign1290_e1548 * ((locals.var_rt).powf(assign1290_e1548 - 1.0) * locals.var_rt_dn4)) } } else { (assign1290_e1549 * (assign1290_e1548 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1290_e1561) + (assign1290_e1550 * (assign1290_e1561 * ((((assign1290_e1552 * (-locals.var_rt_dn4)) * assign1290_e1559) - (assign1290_e1556 * (locals.var_vtv_dn4 * p.p61))) / (assign1290_e1559 * assign1290_e1559)))));

        let assign1300_e1567: f64 = (p.p124 / p.p63);
        let assign1300_e1568: f64 = (locals.var_rt).powf(assign1300_e1567);
        let assign1300_e1569: f64 = (p.p62 * assign1300_e1568);
        let assign1300_e1571: f64 = (-p.p118);
        let assign1300_e1574: f64 = (1.0 - locals.var_rt);
        let assign1300_e1575: f64 = (assign1300_e1571 * assign1300_e1574);
        let assign1300_e1578: f64 = (locals.var_vtv * p.p63);
        let assign1300_e1579: f64 = (assign1300_e1575 / assign1300_e1578);
        let assign1300_e1580: f64 = (assign1300_e1579).exp();
        let assign1300_e1581: f64 = (assign1300_e1569 * assign1300_e1580);
        locals.var_ibcn_t = assign1300_e1581;
        locals.var_ibcn_t_dn4 = (((p.p62 * if 0.0 == 0.0 && ((assign1300_e1567) as f64).is_finite() && ((assign1300_e1567) as f64).fract() == 0.0 { if assign1300_e1567 == 0.0 { 0.0 } else { (assign1300_e1567 * ((locals.var_rt).powf(assign1300_e1567 - 1.0) * locals.var_rt_dn4)) } } else { (assign1300_e1568 * (assign1300_e1567 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1300_e1580) + (assign1300_e1569 * (assign1300_e1580 * ((((assign1300_e1571 * (-locals.var_rt_dn4)) * assign1300_e1578) - (assign1300_e1575 * (locals.var_vtv_dn4 * p.p63))) / (assign1300_e1578 * assign1300_e1578)))));

        let assign1310_e1586: f64 = (p.p123 / p.p61);
        let assign1310_e1587: f64 = (locals.var_rt).powf(assign1310_e1586);
        let assign1310_e1588: f64 = (p.p64 * assign1310_e1587);
        let assign1310_e1590: f64 = (-p.p115);
        let assign1310_e1593: f64 = (1.0 - locals.var_rt);
        let assign1310_e1594: f64 = (assign1310_e1590 * assign1310_e1593);
        let assign1310_e1597: f64 = (locals.var_vtv * p.p61);
        let assign1310_e1598: f64 = (assign1310_e1594 / assign1310_e1597);
        let assign1310_e1599: f64 = (assign1310_e1598).exp();
        let assign1310_e1600: f64 = (assign1310_e1588 * assign1310_e1599);
        locals.var_ibeip_t = assign1310_e1600;
        locals.var_ibeip_t_dn4 = (((p.p64 * if 0.0 == 0.0 && ((assign1310_e1586) as f64).is_finite() && ((assign1310_e1586) as f64).fract() == 0.0 { if assign1310_e1586 == 0.0 { 0.0 } else { (assign1310_e1586 * ((locals.var_rt).powf(assign1310_e1586 - 1.0) * locals.var_rt_dn4)) } } else { (assign1310_e1587 * (assign1310_e1586 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1310_e1599) + (assign1310_e1588 * (assign1310_e1599 * ((((assign1310_e1590 * (-locals.var_rt_dn4)) * assign1310_e1597) - (assign1310_e1594 * (locals.var_vtv_dn4 * p.p61))) / (assign1310_e1597 * assign1310_e1597)))));

        let assign1320_e1605: f64 = (p.p124 / p.p63);
        let assign1320_e1606: f64 = (locals.var_rt).powf(assign1320_e1605);
        let assign1320_e1607: f64 = (p.p65 * assign1320_e1606);
        let assign1320_e1609: f64 = (-p.p118);
        let assign1320_e1612: f64 = (1.0 - locals.var_rt);
        let assign1320_e1613: f64 = (assign1320_e1609 * assign1320_e1612);
        let assign1320_e1616: f64 = (locals.var_vtv * p.p63);
        let assign1320_e1617: f64 = (assign1320_e1613 / assign1320_e1616);
        let assign1320_e1618: f64 = (assign1320_e1617).exp();
        let assign1320_e1619: f64 = (assign1320_e1607 * assign1320_e1618);
        locals.var_ibenp_t = assign1320_e1619;
        locals.var_ibenp_t_dn4 = (((p.p65 * if 0.0 == 0.0 && ((assign1320_e1605) as f64).is_finite() && ((assign1320_e1605) as f64).fract() == 0.0 { if assign1320_e1605 == 0.0 { 0.0 } else { (assign1320_e1605 * ((locals.var_rt).powf(assign1320_e1605 - 1.0) * locals.var_rt_dn4)) } } else { (assign1320_e1606 * (assign1320_e1605 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1320_e1618) + (assign1320_e1607 * (assign1320_e1618 * ((((assign1320_e1609 * (-locals.var_rt_dn4)) * assign1320_e1616) - (assign1320_e1613 * (locals.var_vtv_dn4 * p.p63))) / (assign1320_e1616 * assign1320_e1616)))));

        let assign1330_e1624: f64 = (p.p123 / p.p67);
        let assign1330_e1625: f64 = (locals.var_rt).powf(assign1330_e1624);
        let assign1330_e1626: f64 = (p.p66 * assign1330_e1625);
        let assign1330_e1628: f64 = (-p.p116);
        let assign1330_e1631: f64 = (1.0 - locals.var_rt);
        let assign1330_e1632: f64 = (assign1330_e1628 * assign1330_e1631);
        let assign1330_e1635: f64 = (locals.var_vtv * p.p67);
        let assign1330_e1636: f64 = (assign1330_e1632 / assign1330_e1635);
        let assign1330_e1637: f64 = (assign1330_e1636).exp();
        let assign1330_e1638: f64 = (assign1330_e1626 * assign1330_e1637);
        locals.var_ibcip_t = assign1330_e1638;
        locals.var_ibcip_t_dn4 = (((p.p66 * if 0.0 == 0.0 && ((assign1330_e1624) as f64).is_finite() && ((assign1330_e1624) as f64).fract() == 0.0 { if assign1330_e1624 == 0.0 { 0.0 } else { (assign1330_e1624 * ((locals.var_rt).powf(assign1330_e1624 - 1.0) * locals.var_rt_dn4)) } } else { (assign1330_e1625 * (assign1330_e1624 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1330_e1637) + (assign1330_e1626 * (assign1330_e1637 * ((((assign1330_e1628 * (-locals.var_rt_dn4)) * assign1330_e1635) - (assign1330_e1632 * (locals.var_vtv_dn4 * p.p67))) / (assign1330_e1635 * assign1330_e1635)))));

        let assign1340_e1643: f64 = (p.p124 / p.p69);
        let assign1340_e1644: f64 = (locals.var_rt).powf(assign1340_e1643);
        let assign1340_e1645: f64 = (p.p68 * assign1340_e1644);
        let assign1340_e1647: f64 = (-p.p119);
        let assign1340_e1650: f64 = (1.0 - locals.var_rt);
        let assign1340_e1651: f64 = (assign1340_e1647 * assign1340_e1650);
        let assign1340_e1654: f64 = (locals.var_vtv * p.p69);
        let assign1340_e1655: f64 = (assign1340_e1651 / assign1340_e1654);
        let assign1340_e1656: f64 = (assign1340_e1655).exp();
        let assign1340_e1657: f64 = (assign1340_e1645 * assign1340_e1656);
        locals.var_ibcnp_t = assign1340_e1657;
        locals.var_ibcnp_t_dn4 = (((p.p68 * if 0.0 == 0.0 && ((assign1340_e1643) as f64).is_finite() && ((assign1340_e1643) as f64).fract() == 0.0 { if assign1340_e1643 == 0.0 { 0.0 } else { (assign1340_e1643 * ((locals.var_rt).powf(assign1340_e1643 - 1.0) * locals.var_rt_dn4)) } } else { (assign1340_e1644 * (assign1340_e1643 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1340_e1656) + (assign1340_e1645 * (assign1340_e1656 * ((((assign1340_e1647 * (-locals.var_rt_dn4)) * assign1340_e1654) - (assign1340_e1651 * (locals.var_vtv_dn4 * p.p69))) / (assign1340_e1654 * assign1340_e1654)))));

        let assign1350_e1662: f64 = (locals.var_dt * p.p129);
        let assign1350_e1663: f64 = (1.0 + assign1350_e1662);
        let assign1350_e1664: f64 = (p.p28 * assign1350_e1663);
        locals.var_nf_t = assign1350_e1664;
        locals.var_nf_t_dn4 = (p.p28 * (locals.var_dt_dn4 * p.p129));

        let assign1360_e1669: f64 = (locals.var_dt * p.p129);
        let assign1360_e1670: f64 = (1.0 + assign1360_e1669);
        let assign1360_e1671: f64 = (p.p29 * assign1360_e1670);
        locals.var_nr_t = assign1360_e1671;
        locals.var_nr_t_dn4 = (p.p29 * (locals.var_dt_dn4 * p.p129));

        let assign1370_e1676: f64 = (locals.var_dt * p.p127);
        let assign1370_e1677: f64 = (1.0 + assign1370_e1676);
        let assign1370_e1678: f64 = (p.p84 * assign1370_e1677);
        locals.var_avc2_t = assign1370_e1678;
        locals.var_avc2_t_dn4 = (p.p84 * (locals.var_dt_dn4 * p.p127));

        let assign1380_e1683: f64 = (locals.var_dt * p.p128);
        let assign1380_e1684: f64 = (1.0 + assign1380_e1683);
        let assign1380_e1685: f64 = (p.p86 * assign1380_e1684);
        locals.var_avcx2_t = assign1380_e1685;
        locals.var_avcx2_t_dn4 = (p.p86 * (locals.var_dt_dn4 * p.p128));

        let assign1390_e1692: f64 = (locals.var_dt * p.p92);
        let assign1390_e1693: f64 = (p.p91 + assign1390_e1692);
        let assign1390_e1694: f64 = (locals.var_dt * assign1390_e1693);
        let assign1390_e1695: f64 = (1.0 + assign1390_e1694);
        let assign1390_e1696: f64 = (p.p88 * assign1390_e1695);
        locals.var_vbbe_t = assign1390_e1696;
        locals.var_vbbe_t_dn4 = (p.p88 * ((locals.var_dt_dn4 * assign1390_e1693) + (locals.var_dt * (locals.var_dt_dn4 * p.p92))));

        let assign1400_e1701: f64 = (locals.var_dt * p.p93);
        let assign1400_e1702: f64 = (1.0 + assign1400_e1701);
        let assign1400_e1703: f64 = (p.p89 * assign1400_e1702);
        locals.var_nbbe_t = assign1400_e1703;
        locals.var_nbbe_t_dn4 = (p.p89 * (locals.var_dt_dn4 * p.p93));

        let assign1410_e1707: f64 = (locals.var_vtv / locals.var_rt);
        let assign1410_e1708: f64 = (2.0 * assign1410_e1707);
        let assign1410_e1711: f64 = (0.5 * p.p37);
        let assign1410_e1713: f64 = (assign1410_e1711 * locals.var_rt);
        let assign1410_e1715: f64 = (assign1410_e1713 / locals.var_vtv);
        let assign1410_e1716: f64 = (assign1410_e1715).exp();
        let assign1410_e1718: f64 = (-0.5);
        let assign1410_e1720: f64 = (assign1410_e1718 * p.p37);
        let assign1410_e1722: f64 = (assign1410_e1720 * locals.var_rt);
        let assign1410_e1724: f64 = (assign1410_e1722 / locals.var_vtv);
        let assign1410_e1725: f64 = (assign1410_e1724).exp();
        let assign1410_e1726: f64 = (assign1410_e1716 - assign1410_e1725);
        let assign1410_e1727: f64 = (assign1410_e1726).ln();
        let assign1410_e1728: f64 = (assign1410_e1708 * assign1410_e1727);
        locals.var_psiio = assign1410_e1728;
        locals.var_psiio_dn4 = (((2.0 * (((locals.var_vtv_dn4 * locals.var_rt) - (locals.var_vtv * locals.var_rt_dn4)) / (locals.var_rt * locals.var_rt))) * assign1410_e1727) + (assign1410_e1708 * (((assign1410_e1716 * ((((assign1410_e1711 * locals.var_rt_dn4) * locals.var_vtv) - (assign1410_e1713 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))) - (assign1410_e1725 * ((((assign1410_e1720 * locals.var_rt_dn4) * locals.var_vtv) - (assign1410_e1722 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / assign1410_e1726)));

        let assign1420_e1731: f64 = (locals.var_psiio * locals.var_rt);
        let assign1420_e1734: f64 = (3.0 * locals.var_vtv);
        let assign1420_e1736: f64 = (locals.var_rt).ln();
        let assign1420_e1737: f64 = (assign1420_e1734 * assign1420_e1736);
        let assign1420_e1738: f64 = (assign1420_e1731 - assign1420_e1737);
        let assign1420_e1742: f64 = (locals.var_rt - 1.0);
        let assign1420_e1743: f64 = (p.p114 * assign1420_e1742);
        let assign1420_e1744: f64 = (assign1420_e1738 - assign1420_e1743);
        locals.var_psiin = assign1420_e1744;
        locals.var_psiin_dn4 = ((((locals.var_psiio_dn4 * locals.var_rt) + (locals.var_psiio * locals.var_rt_dn4)) - (((3.0 * locals.var_vtv_dn4) * assign1420_e1736) + (assign1420_e1734 * (locals.var_rt_dn4 / locals.var_rt)))) - (p.p114 * locals.var_rt_dn4));

        let assign1430_e1748: f64 = (2.0 * locals.var_vtv);
        let assign1430_e1754: f64 = (-locals.var_psiin);
        let assign1430_e1756: f64 = (assign1430_e1754 / locals.var_vtv);
        let assign1430_e1757: f64 = (assign1430_e1756).exp();
        let assign1430_e1758: f64 = (4.0 * assign1430_e1757);
        let assign1430_e1759: f64 = (1.0 + assign1430_e1758);
        let assign1430_e1760: f64 = (assign1430_e1759).sqrt();
        let assign1430_e1761: f64 = (1.0 + assign1430_e1760);
        let assign1430_e1762: f64 = (0.5 * assign1430_e1761);
        let assign1430_e1763: f64 = (assign1430_e1762).ln();
        let assign1430_e1764: f64 = (assign1430_e1748 * assign1430_e1763);
        let assign1430_e1765: f64 = (locals.var_psiin + assign1430_e1764);
        locals.var_pe_t = assign1430_e1765;
        locals.var_pe_t_dn4 = (locals.var_psiin_dn4 + (((2.0 * locals.var_vtv_dn4) * assign1430_e1763) + (assign1430_e1748 * ((0.5 * ((4.0 * (assign1430_e1757 * ((((-locals.var_psiin_dn4) * locals.var_vtv) - (assign1430_e1754 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / (2.0 * assign1430_e1760))) / assign1430_e1762))));

        let assign1440_e1769: f64 = (locals.var_vtv / locals.var_rt);
        let assign1440_e1770: f64 = (2.0 * assign1440_e1769);
        let assign1440_e1773: f64 = (0.5 * p.p42);
        let assign1440_e1775: f64 = (assign1440_e1773 * locals.var_rt);
        let assign1440_e1777: f64 = (assign1440_e1775 / locals.var_vtv);
        let assign1440_e1778: f64 = (assign1440_e1777).exp();
        let assign1440_e1780: f64 = (-0.5);
        let assign1440_e1782: f64 = (assign1440_e1780 * p.p42);
        let assign1440_e1784: f64 = (assign1440_e1782 * locals.var_rt);
        let assign1440_e1786: f64 = (assign1440_e1784 / locals.var_vtv);
        let assign1440_e1787: f64 = (assign1440_e1786).exp();
        let assign1440_e1788: f64 = (assign1440_e1778 - assign1440_e1787);
        let assign1440_e1789: f64 = (assign1440_e1788).ln();
        let assign1440_e1790: f64 = (assign1440_e1770 * assign1440_e1789);
        locals.var_psiio__blk37 = assign1440_e1790;
        locals.var_psiio__blk37_dn4 = (((2.0 * (((locals.var_vtv_dn4 * locals.var_rt) - (locals.var_vtv * locals.var_rt_dn4)) / (locals.var_rt * locals.var_rt))) * assign1440_e1789) + (assign1440_e1770 * (((assign1440_e1778 * ((((assign1440_e1773 * locals.var_rt_dn4) * locals.var_vtv) - (assign1440_e1775 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))) - (assign1440_e1787 * ((((assign1440_e1782 * locals.var_rt_dn4) * locals.var_vtv) - (assign1440_e1784 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / assign1440_e1788)));

        let assign1450_e1793: f64 = (locals.var_psiio__blk37 * locals.var_rt);
        let assign1450_e1796: f64 = (3.0 * locals.var_vtv);
        let assign1450_e1798: f64 = (locals.var_rt).ln();
        let assign1450_e1799: f64 = (assign1450_e1796 * assign1450_e1798);
        let assign1450_e1800: f64 = (assign1450_e1793 - assign1450_e1799);
        let assign1450_e1804: f64 = (locals.var_rt - 1.0);
        let assign1450_e1805: f64 = (p.p115 * assign1450_e1804);
        let assign1450_e1806: f64 = (assign1450_e1800 - assign1450_e1805);
        locals.var_psiin__blk38 = assign1450_e1806;
        locals.var_psiin__blk38_dn4 = ((((locals.var_psiio__blk37_dn4 * locals.var_rt) + (locals.var_psiio__blk37 * locals.var_rt_dn4)) - (((3.0 * locals.var_vtv_dn4) * assign1450_e1798) + (assign1450_e1796 * (locals.var_rt_dn4 / locals.var_rt)))) - (p.p115 * locals.var_rt_dn4));

        let assign1460_e1810: f64 = (2.0 * locals.var_vtv);
        let assign1460_e1816: f64 = (-locals.var_psiin__blk38);
        let assign1460_e1818: f64 = (assign1460_e1816 / locals.var_vtv);
        let assign1460_e1819: f64 = (assign1460_e1818).exp();
        let assign1460_e1820: f64 = (4.0 * assign1460_e1819);
        let assign1460_e1821: f64 = (1.0 + assign1460_e1820);
        let assign1460_e1822: f64 = (assign1460_e1821).sqrt();
        let assign1460_e1823: f64 = (1.0 + assign1460_e1822);
        let assign1460_e1824: f64 = (0.5 * assign1460_e1823);
        let assign1460_e1825: f64 = (assign1460_e1824).ln();
        let assign1460_e1826: f64 = (assign1460_e1810 * assign1460_e1825);
        let assign1460_e1827: f64 = (locals.var_psiin__blk38 + assign1460_e1826);
        locals.var_pc_t = assign1460_e1827;
        locals.var_pc_t_dn4 = (locals.var_psiin__blk38_dn4 + (((2.0 * locals.var_vtv_dn4) * assign1460_e1825) + (assign1460_e1810 * ((0.5 * ((4.0 * (assign1460_e1819 * ((((-locals.var_psiin__blk38_dn4) * locals.var_vtv) - (assign1460_e1816 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / (2.0 * assign1460_e1822))) / assign1460_e1824))));

        let assign1470_e1831: f64 = (locals.var_vtv / locals.var_rt);
        let assign1470_e1832: f64 = (2.0 * assign1470_e1831);
        let assign1470_e1835: f64 = (0.5 * p.p50);
        let assign1470_e1837: f64 = (assign1470_e1835 * locals.var_rt);
        let assign1470_e1839: f64 = (assign1470_e1837 / locals.var_vtv);
        let assign1470_e1840: f64 = (assign1470_e1839).exp();
        let assign1470_e1842: f64 = (-0.5);
        let assign1470_e1844: f64 = (assign1470_e1842 * p.p50);
        let assign1470_e1846: f64 = (assign1470_e1844 * locals.var_rt);
        let assign1470_e1848: f64 = (assign1470_e1846 / locals.var_vtv);
        let assign1470_e1849: f64 = (assign1470_e1848).exp();
        let assign1470_e1850: f64 = (assign1470_e1840 - assign1470_e1849);
        let assign1470_e1851: f64 = (assign1470_e1850).ln();
        let assign1470_e1852: f64 = (assign1470_e1832 * assign1470_e1851);
        locals.var_psiio__blk39 = assign1470_e1852;
        locals.var_psiio__blk39_dn4 = (((2.0 * (((locals.var_vtv_dn4 * locals.var_rt) - (locals.var_vtv * locals.var_rt_dn4)) / (locals.var_rt * locals.var_rt))) * assign1470_e1851) + (assign1470_e1832 * (((assign1470_e1840 * ((((assign1470_e1835 * locals.var_rt_dn4) * locals.var_vtv) - (assign1470_e1837 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))) - (assign1470_e1849 * ((((assign1470_e1844 * locals.var_rt_dn4) * locals.var_vtv) - (assign1470_e1846 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / assign1470_e1850)));

        let assign1480_e1855: f64 = (locals.var_psiio__blk39 * locals.var_rt);
        let assign1480_e1858: f64 = (3.0 * locals.var_vtv);
        let assign1480_e1860: f64 = (locals.var_rt).ln();
        let assign1480_e1861: f64 = (assign1480_e1858 * assign1480_e1860);
        let assign1480_e1862: f64 = (assign1480_e1855 - assign1480_e1861);
        let assign1480_e1866: f64 = (locals.var_rt - 1.0);
        let assign1480_e1867: f64 = (p.p116 * assign1480_e1866);
        let assign1480_e1868: f64 = (assign1480_e1862 - assign1480_e1867);
        locals.var_psiin__blk40 = assign1480_e1868;
        locals.var_psiin__blk40_dn4 = ((((locals.var_psiio__blk39_dn4 * locals.var_rt) + (locals.var_psiio__blk39 * locals.var_rt_dn4)) - (((3.0 * locals.var_vtv_dn4) * assign1480_e1860) + (assign1480_e1858 * (locals.var_rt_dn4 / locals.var_rt)))) - (p.p116 * locals.var_rt_dn4));

        let assign1490_e1872: f64 = (2.0 * locals.var_vtv);
        let assign1490_e1878: f64 = (-locals.var_psiin__blk40);
        let assign1490_e1880: f64 = (assign1490_e1878 / locals.var_vtv);
        let assign1490_e1881: f64 = (assign1490_e1880).exp();
        let assign1490_e1882: f64 = (4.0 * assign1490_e1881);
        let assign1490_e1883: f64 = (1.0 + assign1490_e1882);
        let assign1490_e1884: f64 = (assign1490_e1883).sqrt();
        let assign1490_e1885: f64 = (1.0 + assign1490_e1884);
        let assign1490_e1886: f64 = (0.5 * assign1490_e1885);
        let assign1490_e1887: f64 = (assign1490_e1886).ln();
        let assign1490_e1888: f64 = (assign1490_e1872 * assign1490_e1887);
        let assign1490_e1889: f64 = (locals.var_psiin__blk40 + assign1490_e1888);
        locals.var_ps_t = assign1490_e1889;
        locals.var_ps_t_dn4 = (locals.var_psiin__blk40_dn4 + (((2.0 * locals.var_vtv_dn4) * assign1490_e1887) + (assign1490_e1872 * ((0.5 * ((4.0 * (assign1490_e1881 * ((((-locals.var_psiin__blk40_dn4) * locals.var_vtv) - (assign1490_e1878 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / (2.0 * assign1490_e1884))) / assign1490_e1886))));

        let assign1500_e1893: f64 = (p.p37 / locals.var_pe_t);
        let assign1500_e1895: f64 = (assign1500_e1893).powf(p.p38);
        let assign1500_e1896: f64 = (p.p36 * assign1500_e1895);
        locals.var_cje_t = assign1500_e1896;
        locals.var_cje_t_dn4 = (p.p36 * if 0.0 == 0.0 && ((p.p38) as f64).is_finite() && ((p.p38) as f64).fract() == 0.0 { if p.p38 == 0.0 { 0.0 } else { (p.p38 * ((assign1500_e1893).powf(p.p38 - 1.0) * (-((p.p37 * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign1500_e1895 * (p.p38 * ((-((p.p37 * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t))) / assign1500_e1893))) });

        let assign1510_e1900: f64 = (p.p42 / locals.var_pc_t);
        let assign1510_e1902: f64 = (assign1510_e1900).powf(p.p43);
        let assign1510_e1903: f64 = (p.p41 * assign1510_e1902);
        locals.var_cjc_t = assign1510_e1903;
        locals.var_cjc_t_dn4 = (p.p41 * if 0.0 == 0.0 && ((p.p43) as f64).is_finite() && ((p.p43) as f64).fract() == 0.0 { if p.p43 == 0.0 { 0.0 } else { (p.p43 * ((assign1510_e1900).powf(p.p43 - 1.0) * (-((p.p42 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign1510_e1902 * (p.p43 * ((-((p.p42 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign1510_e1900))) });

        let assign1520_e1907: f64 = (p.p42 / locals.var_pc_t);
        let assign1520_e1909: f64 = (assign1520_e1907).powf(p.p43);
        let assign1520_e1910: f64 = (p.p48 * assign1520_e1909);
        locals.var_cjep_t = assign1520_e1910;
        locals.var_cjep_t_dn4 = (p.p48 * if 0.0 == 0.0 && ((p.p43) as f64).is_finite() && ((p.p43) as f64).fract() == 0.0 { if p.p43 == 0.0 { 0.0 } else { (p.p43 * ((assign1520_e1907).powf(p.p43 - 1.0) * (-((p.p42 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign1520_e1909 * (p.p43 * ((-((p.p42 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign1520_e1907))) });

        let assign1530_e1914: f64 = (p.p50 / locals.var_ps_t);
        let assign1530_e1916: f64 = (assign1530_e1914).powf(p.p51);
        let assign1530_e1917: f64 = (p.p49 * assign1530_e1916);
        locals.var_cjcp_t = assign1530_e1917;
        locals.var_cjcp_t_dn4 = (p.p49 * if 0.0 == 0.0 && ((p.p51) as f64).is_finite() && ((p.p51) as f64).fract() == 0.0 { if p.p51 == 0.0 { 0.0 } else { (p.p51 * ((assign1530_e1914).powf(p.p51 - 1.0) * (-((p.p50 * locals.var_ps_t_dn4) / (locals.var_ps_t * locals.var_ps_t))))) } } else { (assign1530_e1916 * (p.p51 * ((-((p.p50 * locals.var_ps_t_dn4) / (locals.var_ps_t * locals.var_ps_t))) / assign1530_e1914))) });

        let assign1540_e1921: f64 = (locals.var_rt).powf(p.p122);
        let assign1540_e1922: f64 = (p.p19 * assign1540_e1921);
        let assign1540_e1924: f64 = (-p.p113);
        let assign1540_e1927: f64 = (1.0 - locals.var_rt);
        let assign1540_e1928: f64 = (assign1540_e1924 * assign1540_e1927);
        let assign1540_e1930: f64 = (assign1540_e1928 / locals.var_vtv);
        let assign1540_e1931: f64 = (assign1540_e1930).exp();
        let assign1540_e1932: f64 = (assign1540_e1922 * assign1540_e1931);
        locals.var_gamm_t = assign1540_e1932;
        locals.var_gamm_t_dn4 = (((p.p19 * if 0.0 == 0.0 && ((p.p122) as f64).is_finite() && ((p.p122) as f64).fract() == 0.0 { if p.p122 == 0.0 { 0.0 } else { (p.p122 * ((locals.var_rt).powf(p.p122 - 1.0) * locals.var_rt_dn4)) } } else { (assign1540_e1921 * (p.p122 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1540_e1931) + (assign1540_e1922 * (assign1540_e1931 * ((((assign1540_e1924 * (-locals.var_rt_dn4)) * locals.var_vtv) - (assign1540_e1928 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))));

        let assign1550_e1936: f64 = (locals.var_rt).powf(p.p112);
        let assign1550_e1937: f64 = (p.p18 * assign1550_e1936);
        locals.var_vo_t = assign1550_e1937;
        locals.var_vo_t_dn4 = (p.p18 * if 0.0 == 0.0 && ((p.p112) as f64).is_finite() && ((p.p112) as f64).fract() == 0.0 { if p.p112 == 0.0 { 0.0 } else { (p.p112 * ((locals.var_rt).powf(p.p112 - 1.0) * locals.var_rt_dn4)) } } else { (assign1550_e1936 * (p.p112 * (locals.var_rt_dn4 / locals.var_rt))) });

        let assign1560_e1939: f64 = (-locals.var_vbbe_t);
        let assign1560_e1942: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign1560_e1943: f64 = (assign1560_e1939 / assign1560_e1942);
        let assign1560_e1944: f64 = (assign1560_e1943).exp();
        locals.var_ebbe_t = assign1560_e1944;
        locals.var_ebbe_t_dn4 = (assign1560_e1944 * ((((-locals.var_vbbe_t_dn4) * assign1560_e1942) - (assign1560_e1939 * ((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)))) / (assign1560_e1942 * assign1560_e1942)));

        let assign1570_e1949: f64 = (locals.var_dt * p.p130);
        let assign1570_e1950: f64 = (1.0 + assign1570_e1949);
        let assign1570_e1951: f64 = (p.p70 * assign1570_e1950);
        locals.var_vef_t = assign1570_e1951;
        locals.var_vef_t_dn4 = (p.p70 * (locals.var_dt_dn4 * p.p130));

        let assign1580_e1956: f64 = (locals.var_dt * p.p131);
        let assign1580_e1957: f64 = (1.0 + assign1580_e1956);
        let assign1580_e1958: f64 = (p.p71 * assign1580_e1957);
        locals.var_ver_t = assign1580_e1958;
        locals.var_ver_t_dn4 = (p.p71 * (locals.var_dt_dn4 * p.p131));

        let (assign1590_e1966, assign1590_e1966_d_n4,) = {
    if (locals.var_rcx_t > 0.001) {
        let assign1590_e1964: f64 = (1.0 / locals.var_rcx_t);
        (assign1590_e1964, (-(locals.var_rcx_t_dn4 / (locals.var_rcx_t * locals.var_rcx_t))),)
    } else {
        (1000.0, 0.0,)
    }
};
        locals.var_gcx = assign1590_e1966;
        locals.var_gcx_dn4 = assign1590_e1966_d_n4;

        let (assign1600_e1974, assign1600_e1974_d_n4,) = {
    if (locals.var_rci_t > 0.001) {
        let assign1600_e1972: f64 = (1.0 / locals.var_rci_t);
        (assign1600_e1972, (-(locals.var_rci_t_dn4 / (locals.var_rci_t * locals.var_rci_t))),)
    } else {
        (1000.0, 0.0,)
    }
};
        locals.var_gci = assign1600_e1974;
        locals.var_gci_dn4 = assign1600_e1974_d_n4;

        let (assign1610_e1982, assign1610_e1982_d_n4,) = {
    if (locals.var_rbx_t > 0.001) {
        let assign1610_e1980: f64 = (1.0 / locals.var_rbx_t);
        (assign1610_e1980, (-(locals.var_rbx_t_dn4 / (locals.var_rbx_t * locals.var_rbx_t))),)
    } else {
        (1000.0, 0.0,)
    }
};
        locals.var_gbx = assign1610_e1982;
        locals.var_gbx_dn4 = assign1610_e1982_d_n4;

        let (assign1620_e1990, assign1620_e1990_d_n4,) = {
    if (locals.var_rbi_t > 0.001) {
        let assign1620_e1988: f64 = (1.0 / locals.var_rbi_t);
        (assign1620_e1988, (-(locals.var_rbi_t_dn4 / (locals.var_rbi_t * locals.var_rbi_t))),)
    } else {
        (1000.0, 0.0,)
    }
};
        locals.var_gbi = assign1620_e1990;
        locals.var_gbi_dn4 = assign1620_e1990_d_n4;

        let (assign1630_e1998, assign1630_e1998_d_n4,) = {
    if (locals.var_re_t > 0.001) {
        let assign1630_e1996: f64 = (1.0 / locals.var_re_t);
        (assign1630_e1996, (-(locals.var_re_t_dn4 / (locals.var_re_t * locals.var_re_t))),)
    } else {
        (1000.0, 0.0,)
    }
};
        locals.var_ge = assign1630_e1998;
        locals.var_ge_dn4 = assign1630_e1998_d_n4;

        let (assign1640_e2006, assign1640_e2006_d_n4,) = {
    if (locals.var_rbp_t > 0.001) {
        let assign1640_e2004: f64 = (1.0 / locals.var_rbp_t);
        (assign1640_e2004, (-(locals.var_rbp_t_dn4 / (locals.var_rbp_t * locals.var_rbp_t))),)
    } else {
        (1000.0, 0.0,)
    }
};
        locals.var_gbp = assign1640_e2006;
        locals.var_gbp_dn4 = assign1640_e2006_d_n4;

        let (assign1650_e2014, assign1650_e2014_d_n4,) = {
    if (locals.var_rs_t > 0.001) {
        let assign1650_e2012: f64 = (1.0 / locals.var_rs_t);
        (assign1650_e2012, (-(locals.var_rs_t_dn4 / (locals.var_rs_t * locals.var_rs_t))),)
    } else {
        (1000.0, 0.0,)
    }
};
        locals.var_gs = assign1650_e2014;
        locals.var_gs_dn4 = assign1650_e2014_d_n4;

        let (assign1660_e2022, assign1660_e2022_d_n4,) = {
    if (locals.var_rth_t > 0.001) {
        let assign1660_e2020: f64 = (1.0 / locals.var_rth_t);
        (assign1660_e2020, (-(locals.var_rth_t_dn4 / (locals.var_rth_t * locals.var_rth_t))),)
    } else {
        (1000.0, 0.0,)
    }
};
        locals.var_gth = assign1660_e2022;
        locals.var_gth_dn4 = assign1660_e2022_d_n4;

    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (assign1670_e2030, assign1670_e2030_d_n4,) = {
    if (locals.var_vef_t > 0.0) {
        let assign1670_e2028: f64 = (1.0 / locals.var_vef_t);
        (assign1670_e2028, (-(locals.var_vef_t_dn4 / (locals.var_vef_t * locals.var_vef_t))),)
    } else {
        (0.0, 0.0,)
    }
};
        locals.var_ivef = assign1670_e2030;
        locals.var_ivef_dn4 = assign1670_e2030_d_n4;

        let (assign1680_e2038, assign1680_e2038_d_n4,) = {
    if (locals.var_ver_t > 0.0) {
        let assign1680_e2036: f64 = (1.0 / locals.var_ver_t);
        (assign1680_e2036, (-(locals.var_ver_t_dn4 / (locals.var_ver_t * locals.var_ver_t))),)
    } else {
        (0.0, 0.0,)
    }
};
        locals.var_iver = assign1680_e2038;
        locals.var_iver_dn4 = assign1680_e2038_d_n4;

        let (assign1690_e2046, assign1690_e2046_d_n4,) = {
    if (locals.var_ikf_t > 0.0) {
        let assign1690_e2044: f64 = (1.0 / locals.var_ikf_t);
        (assign1690_e2044, (-(locals.var_ikf_t_dn4 / (locals.var_ikf_t * locals.var_ikf_t))),)
    } else {
        (0.0, 0.0,)
    }
};
        locals.var_iikf = assign1690_e2046;
        locals.var_iikf_dn4 = assign1690_e2046_d_n4;

        let (assign1700_e2054, assign1700_e2054_d_n4,) = {
    if (locals.var_vo_t > 0.0) {
        let assign1700_e2052: f64 = (1.0 / locals.var_vo_t);
        (assign1700_e2052, (-(locals.var_vo_t_dn4 / (locals.var_vo_t * locals.var_vo_t))),)
    } else {
        (0.0, 0.0,)
    }
};
        locals.var_ivo = assign1700_e2054;
        locals.var_ivo_dn4 = assign1700_e2054_d_n4;

        let assign1710_e2057: f64 = (locals.var_vbictype * (nv8 - nv9));
        locals.var_vbei = assign1710_e2057;
        locals.var_vbei_dn8 = locals.var_vbictype;
        locals.var_vbei_dn9 = (-locals.var_vbictype);

        let assign1720_e2060: f64 = (locals.var_vbictype * (nv7 - nv9));
        locals.var_vbex = assign1720_e2060;
        locals.var_vbex_dn7 = locals.var_vbictype;
        locals.var_vbex_dn9 = (-locals.var_vbictype);

        let assign1730_e2063: f64 = (locals.var_vbictype * (nv8 - nv6));
        locals.var_vbci = assign1730_e2063;
        locals.var_vbci_dn6 = (-locals.var_vbictype);
        locals.var_vbci_dn8 = locals.var_vbictype;

        let assign1740_e2066: f64 = (locals.var_vbictype * (nv8 - nv5));
        locals.var_vbcx = assign1740_e2066;
        locals.var_vbcx_dn5 = (-locals.var_vbictype);
        locals.var_vbcx_dn8 = locals.var_vbictype;

        let assign1750_e2069: f64 = (locals.var_vbictype * (nv7 - nv5));
        locals.var_vbxcx = assign1750_e2069;
        locals.var_vbxcx_dn5 = (-locals.var_vbictype);
        locals.var_vbxcx_dn7 = locals.var_vbictype;

        let assign1760_e2072: f64 = (locals.var_vbictype * (nv7 - nv10));
        locals.var_vbep = assign1760_e2072;
        locals.var_vbep_dn7 = locals.var_vbictype;
        locals.var_vbep_dn10 = (-locals.var_vbictype);

        let assign1780_e2076: f64 = (locals.var_vbictype * (nv6 - nv9));
        locals.var_vcei = assign1780_e2076;
        locals.var_vcei_dn6 = locals.var_vbictype;
        locals.var_vcei_dn9 = (-locals.var_vbictype);

        locals.var_vrcx = (nv0 - nv5);
        locals.var_vrcx_dn0 = 1.0;
        locals.var_vrcx_dn5 = -1.0;

        let assign1810_e2081: f64 = (locals.var_vbictype * (nv5 - nv6));
        locals.var_vrci = assign1810_e2081;
        locals.var_vrci_dn5 = locals.var_vbictype;
        locals.var_vrci_dn6 = (-locals.var_vbictype);

        locals.var_vrbx = (nv1 - nv7);
        locals.var_vrbx_dn1 = 1.0;
        locals.var_vrbx_dn7 = -1.0;

        locals.var_vrbi = (nv7 - nv8);
        locals.var_vrbi_dn7 = 1.0;
        locals.var_vrbi_dn8 = -1.0;

        locals.var_vre = (nv2 - nv9);
        locals.var_vre_dn2 = 1.0;
        locals.var_vre_dn9 = -1.0;

        locals.var_vrbp = (nv10 - nv5);
        locals.var_vrbp_dn5 = -1.0;
        locals.var_vrbp_dn10 = 1.0;

        let assign1860_e2088: f64 = (locals.var_vbictype * (nv11 - nv10));
        locals.var_vbcp = assign1860_e2088;
        locals.var_vbcp_dn10 = (-locals.var_vbictype);
        locals.var_vbcp_dn11 = locals.var_vbictype;

        let assign1870_e2091: f64 = (locals.var_vbictype * (nv7 - nv11));
        locals.var_vcep = assign1870_e2091;
        locals.var_vcep_dn7 = locals.var_vbictype;
        locals.var_vcep_dn11 = (-locals.var_vbictype);

        locals.var_vrs = (nv3 - nv11);
        locals.var_vrs_dn3 = 1.0;
        locals.var_vrs_dn11 = -1.0;

        locals.var_vxf2 = (nv13 - 0.0);
        locals.var_vxf2_dn13 = 1.0;

        let assign1910_e2096: f64 = (-locals.var_pe_t);
        let assign1910_e2098: f64 = (assign1910_e2096 * p.p34);
        locals.var_dv0 = assign1910_e2098;
        locals.var_dv0_dn4 = ((-locals.var_pe_t_dn4) * p.p34);

        let assign1920_e2101: f64 = if p.p39 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign1920_e2101;

        let (assign1930_e2107, assign1930_e2107_d_n4, assign1930_e2107_d_n8, assign1930_e2107_d_n9,) = {
    if (locals.var_guard52 != 0.0) {
        let assign1930_e2105: f64 = (locals.var_vbei + locals.var_dv0);
        (assign1930_e2105, locals.var_dv0_dn4, locals.var_vbei_dn8, locals.var_vbei_dn9,)
    } else {
        (locals.var_dvh, locals.var_dvh_dn4, locals.var_dvh_dn8, locals.var_dvh_dn9,)
    }
};
        locals.var_dvh = assign1930_e2107;
        locals.var_dvh_dn4 = assign1930_e2107_d_n4;
        locals.var_dvh_dn8 = assign1930_e2107_d_n8;
        locals.var_dvh_dn9 = assign1930_e2107_d_n9;

        let assign1940_e2110: f64 = if locals.var_dvh > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign1940_e2110;

        let (assign1950_e2121,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign1950_e2116: f64 = (1.0 - p.p34);
        let assign1950_e2118: f64 = (-p.p38);
        let assign1950_e2119: f64 = (assign1950_e2116).powf(assign1950_e2118);
        (assign1950_e2119,)
    } else {
        (locals.var_pwq,)
    }
};
        locals.var_pwq = assign1950_e2121;

        let (assign1960_e2139, assign1960_e2139_d_n4, assign1960_e2139_d_n8, assign1960_e2139_d_n9,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign1960_e2130: f64 = (1.0 - p.p34);
        let assign1960_e2131: f64 = (locals.var_pwq * assign1960_e2130);
        let assign1960_e2132: f64 = (1.0 - assign1960_e2131);
        let assign1960_e2133: f64 = (locals.var_pe_t * assign1960_e2132);
        let assign1960_e2136: f64 = (1.0 - p.p38);
        let assign1960_e2137: f64 = (assign1960_e2133 / assign1960_e2136);
        (assign1960_e2137, ((locals.var_pe_t_dn4 * assign1960_e2132) / assign1960_e2136), 0.0, 0.0,)
    } else {
        (locals.var_qlo, locals.var_qlo_dn4, locals.var_qlo_dn8, locals.var_qlo_dn9,)
    }
};
        locals.var_qlo = assign1960_e2139;
        locals.var_qlo_dn4 = assign1960_e2139_d_n4;
        locals.var_qlo_dn8 = assign1960_e2139_d_n8;
        locals.var_qlo_dn9 = assign1960_e2139_d_n9;

        let (assign1970_e2161, assign1970_e2161_d_n4, assign1970_e2161_d_n8, assign1970_e2161_d_n9,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign1970_e2147: f64 = (0.5 * p.p38);
        let assign1970_e2149: f64 = (assign1970_e2147 * locals.var_dvh);
        let assign1970_e2153: f64 = (1.0 - p.p34);
        let assign1970_e2154: f64 = (locals.var_pe_t * assign1970_e2153);
        let assign1970_e2155: f64 = (assign1970_e2149 / assign1970_e2154);
        let assign1970_e2156: f64 = (1.0 + assign1970_e2155);
        let assign1970_e2157: f64 = (locals.var_dvh * assign1970_e2156);
        let assign1970_e2159: f64 = (assign1970_e2157 * locals.var_pwq);
        (assign1970_e2159, (((locals.var_dvh_dn4 * assign1970_e2156) + (locals.var_dvh * ((((assign1970_e2147 * locals.var_dvh_dn4) * assign1970_e2154) - (assign1970_e2149 * (locals.var_pe_t_dn4 * assign1970_e2153))) / (assign1970_e2154 * assign1970_e2154)))) * locals.var_pwq), (((locals.var_dvh_dn8 * assign1970_e2156) + (locals.var_dvh * ((assign1970_e2147 * locals.var_dvh_dn8) / assign1970_e2154))) * locals.var_pwq), (((locals.var_dvh_dn9 * assign1970_e2156) + (locals.var_dvh * ((assign1970_e2147 * locals.var_dvh_dn9) / assign1970_e2154))) * locals.var_pwq),)
    } else {
        (locals.var_qhi, locals.var_qhi_dn4, locals.var_qhi_dn8, locals.var_qhi_dn9,)
    }
};
        locals.var_qhi = assign1970_e2161;
        locals.var_qhi_dn4 = assign1970_e2161_d_n4;
        locals.var_qhi_dn8 = assign1970_e2161_d_n8;
        locals.var_qhi_dn9 = assign1970_e2161_d_n9;

        let (assign1980_e2184, assign1980_e2184_d_n4, assign1980_e2184_d_n8, assign1980_e2184_d_n9,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 == 0.0)) {
        let assign1980_e2171: f64 = (locals.var_vbei / locals.var_pe_t);
        let assign1980_e2172: f64 = (1.0 - assign1980_e2171);
        let assign1980_e2175: f64 = (1.0 - p.p38);
        let assign1980_e2176: f64 = (assign1980_e2172).powf(assign1980_e2175);
        let assign1980_e2177: f64 = (1.0 - assign1980_e2176);
        let assign1980_e2178: f64 = (locals.var_pe_t * assign1980_e2177);
        let assign1980_e2181: f64 = (1.0 - p.p38);
        let assign1980_e2182: f64 = (assign1980_e2178 / assign1980_e2181);
        (assign1980_e2182, (((locals.var_pe_t_dn4 * assign1980_e2177) + (locals.var_pe_t * (-if 0.0 == 0.0 && ((assign1980_e2175) as f64).is_finite() && ((assign1980_e2175) as f64).fract() == 0.0 { if assign1980_e2175 == 0.0 { 0.0 } else { (assign1980_e2175 * ((assign1980_e2172).powf(assign1980_e2175 - 1.0) * (-(-((locals.var_vbei * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t)))))) } } else { (assign1980_e2176 * (assign1980_e2175 * ((-(-((locals.var_vbei * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t)))) / assign1980_e2172))) }))) / assign1980_e2181), ((locals.var_pe_t * (-if 0.0 == 0.0 && ((assign1980_e2175) as f64).is_finite() && ((assign1980_e2175) as f64).fract() == 0.0 { if assign1980_e2175 == 0.0 { 0.0 } else { (assign1980_e2175 * ((assign1980_e2172).powf(assign1980_e2175 - 1.0) * (-(locals.var_vbei_dn8 / locals.var_pe_t)))) } } else { (assign1980_e2176 * (assign1980_e2175 * ((-(locals.var_vbei_dn8 / locals.var_pe_t)) / assign1980_e2172))) })) / assign1980_e2181), ((locals.var_pe_t * (-if 0.0 == 0.0 && ((assign1980_e2175) as f64).is_finite() && ((assign1980_e2175) as f64).fract() == 0.0 { if assign1980_e2175 == 0.0 { 0.0 } else { (assign1980_e2175 * ((assign1980_e2172).powf(assign1980_e2175 - 1.0) * (-(locals.var_vbei_dn9 / locals.var_pe_t)))) } } else { (assign1980_e2176 * (assign1980_e2175 * ((-(locals.var_vbei_dn9 / locals.var_pe_t)) / assign1980_e2172))) })) / assign1980_e2181),)
    } else {
        (locals.var_qlo, locals.var_qlo_dn4, locals.var_qlo_dn8, locals.var_qlo_dn9,)
    }
};
        locals.var_qlo = assign1980_e2184;
        locals.var_qlo_dn4 = assign1980_e2184_d_n4;
        locals.var_qlo_dn8 = assign1980_e2184_d_n8;
        locals.var_qlo_dn9 = assign1980_e2184_d_n9;

        let (assign1990_e2191, assign1990_e2191_d_n4, assign1990_e2191_d_n8, assign1990_e2191_d_n9,) = {
    if ((locals.var_guard52 != 0.0) && (locals.var_guard53 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi, locals.var_qhi_dn4, locals.var_qhi_dn8, locals.var_qhi_dn9,)
    }
};
        locals.var_qhi = assign1990_e2191;
        locals.var_qhi_dn4 = assign1990_e2191_d_n4;
        locals.var_qhi_dn8 = assign1990_e2191_d_n8;
        locals.var_qhi_dn9 = assign1990_e2191_d_n9;

        let (assign2000_e2197, assign2000_e2197_d_n4, assign2000_e2197_d_n8, assign2000_e2197_d_n9,) = {
    if (locals.var_guard52 != 0.0) {
        let assign2000_e2195: f64 = (locals.var_qlo + locals.var_qhi);
        (assign2000_e2195, (locals.var_qlo_dn4 + locals.var_qhi_dn4), (locals.var_qlo_dn8 + locals.var_qhi_dn8), (locals.var_qlo_dn9 + locals.var_qhi_dn9),)
    } else {
        (locals.var_qdbe, locals.var_qdbe_dn4, locals.var_qdbe_dn8, locals.var_qdbe_dn9,)
    }
};
        locals.var_qdbe = assign2000_e2197;
        locals.var_qdbe_dn4 = assign2000_e2197_d_n4;
        locals.var_qdbe_dn8 = assign2000_e2197_d_n8;
        locals.var_qdbe_dn9 = assign2000_e2197_d_n9;

        let (assign2010_e2211, assign2010_e2211_d_n4,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2010_e2202: f64 = (locals.var_dv0 * locals.var_dv0);
        let assign2010_e2205: f64 = (4.0 * p.p39);
        let assign2010_e2207: f64 = (assign2010_e2205 * p.p39);
        let assign2010_e2208: f64 = (assign2010_e2202 + assign2010_e2207);
        let assign2010_e2209: f64 = (assign2010_e2208).sqrt();
        (assign2010_e2209, (((locals.var_dv0_dn4 * locals.var_dv0) + (locals.var_dv0 * locals.var_dv0_dn4)) / (2.0 * assign2010_e2209)),)
    } else {
        (locals.var_mv0, locals.var_mv0_dn4,)
    }
};
        locals.var_mv0 = assign2010_e2211;
        locals.var_mv0_dn4 = assign2010_e2211_d_n4;

        let (assign2020_e2221, assign2020_e2221_d_n4,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2020_e2215: f64 = (-0.5);
        let assign2020_e2218: f64 = (locals.var_dv0 + locals.var_mv0);
        let assign2020_e2219: f64 = (assign2020_e2215 * assign2020_e2218);
        (assign2020_e2219, (assign2020_e2215 * (locals.var_dv0_dn4 + locals.var_mv0_dn4)),)
    } else {
        (locals.var_vl0, locals.var_vl0_dn4,)
    }
};
        locals.var_vl0 = assign2020_e2221;
        locals.var_vl0_dn4 = assign2020_e2221_d_n4;

        let (assign2030_e2241, assign2030_e2241_d_n4,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2030_e2225: f64 = (-locals.var_pe_t);
        let assign2030_e2229: f64 = (locals.var_vl0 / locals.var_pe_t);
        let assign2030_e2230: f64 = (1.0 - assign2030_e2229);
        let assign2030_e2233: f64 = (1.0 - p.p38);
        let assign2030_e2234: f64 = (assign2030_e2230).powf(assign2030_e2233);
        let assign2030_e2235: f64 = (assign2030_e2225 * assign2030_e2234);
        let assign2030_e2238: f64 = (1.0 - p.p38);
        let assign2030_e2239: f64 = (assign2030_e2235 / assign2030_e2238);
        (assign2030_e2239, ((((-locals.var_pe_t_dn4) * assign2030_e2234) + (assign2030_e2225 * if 0.0 == 0.0 && ((assign2030_e2233) as f64).is_finite() && ((assign2030_e2233) as f64).fract() == 0.0 { if assign2030_e2233 == 0.0 { 0.0 } else { (assign2030_e2233 * ((assign2030_e2230).powf(assign2030_e2233 - 1.0) * (-(((locals.var_vl0_dn4 * locals.var_pe_t) - (locals.var_vl0 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign2030_e2234 * (assign2030_e2233 * ((-(((locals.var_vl0_dn4 * locals.var_pe_t) - (locals.var_vl0 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))) / assign2030_e2230))) })) / assign2030_e2238),)
    } else {
        (locals.var_q0, locals.var_q0_dn4,)
    }
};
        locals.var_q0 = assign2030_e2241;
        locals.var_q0_dn4 = assign2030_e2241_d_n4;

        let (assign2040_e2248, assign2040_e2248_d_n4, assign2040_e2248_d_n8, assign2040_e2248_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2040_e2246: f64 = (locals.var_vbei + locals.var_dv0);
        (assign2040_e2246, locals.var_dv0_dn4, locals.var_vbei_dn8, locals.var_vbei_dn9,)
    } else {
        (locals.var_dv, locals.var_dv_dn4, locals.var_dv_dn8, locals.var_dv_dn9,)
    }
};
        locals.var_dv = assign2040_e2248;
        locals.var_dv_dn4 = assign2040_e2248_d_n4;
        locals.var_dv_dn8 = assign2040_e2248_d_n8;
        locals.var_dv_dn9 = assign2040_e2248_d_n9;

        let (assign2050_e2262, assign2050_e2262_d_n4, assign2050_e2262_d_n8, assign2050_e2262_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2050_e2253: f64 = (locals.var_dv * locals.var_dv);
        let assign2050_e2256: f64 = (4.0 * p.p39);
        let assign2050_e2258: f64 = (assign2050_e2256 * p.p39);
        let assign2050_e2259: f64 = (assign2050_e2253 + assign2050_e2258);
        let assign2050_e2260: f64 = (assign2050_e2259).sqrt();
        (assign2050_e2260, (((locals.var_dv_dn4 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn4)) / (2.0 * assign2050_e2260)), (((locals.var_dv_dn8 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn8)) / (2.0 * assign2050_e2260)), (((locals.var_dv_dn9 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn9)) / (2.0 * assign2050_e2260)),)
    } else {
        (locals.var_mv, locals.var_mv_dn4, locals.var_mv_dn8, locals.var_mv_dn9,)
    }
};
        locals.var_mv = assign2050_e2262;
        locals.var_mv_dn4 = assign2050_e2262_d_n4;
        locals.var_mv_dn8 = assign2050_e2262_d_n8;
        locals.var_mv_dn9 = assign2050_e2262_d_n9;

        let (assign2060_e2273, assign2060_e2273_d_n4, assign2060_e2273_d_n8, assign2060_e2273_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2060_e2268: f64 = (locals.var_dv - locals.var_mv);
        let assign2060_e2269: f64 = (0.5 * assign2060_e2268);
        let assign2060_e2271: f64 = (assign2060_e2269 - locals.var_dv0);
        (assign2060_e2271, ((0.5 * (locals.var_dv_dn4 - locals.var_mv_dn4)) - locals.var_dv0_dn4), (0.5 * (locals.var_dv_dn8 - locals.var_mv_dn8)), (0.5 * (locals.var_dv_dn9 - locals.var_mv_dn9)),)
    } else {
        (locals.var_vl, locals.var_vl_dn4, locals.var_vl_dn8, locals.var_vl_dn9,)
    }
};
        locals.var_vl = assign2060_e2273;
        locals.var_vl_dn4 = assign2060_e2273_d_n4;
        locals.var_vl_dn8 = assign2060_e2273_d_n8;
        locals.var_vl_dn9 = assign2060_e2273_d_n9;

        let (assign2070_e2293, assign2070_e2293_d_n4, assign2070_e2293_d_n8, assign2070_e2293_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2070_e2277: f64 = (-locals.var_pe_t);
        let assign2070_e2281: f64 = (locals.var_vl / locals.var_pe_t);
        let assign2070_e2282: f64 = (1.0 - assign2070_e2281);
        let assign2070_e2285: f64 = (1.0 - p.p38);
        let assign2070_e2286: f64 = (assign2070_e2282).powf(assign2070_e2285);
        let assign2070_e2287: f64 = (assign2070_e2277 * assign2070_e2286);
        let assign2070_e2290: f64 = (1.0 - p.p38);
        let assign2070_e2291: f64 = (assign2070_e2287 / assign2070_e2290);
        (assign2070_e2291, ((((-locals.var_pe_t_dn4) * assign2070_e2286) + (assign2070_e2277 * if 0.0 == 0.0 && ((assign2070_e2285) as f64).is_finite() && ((assign2070_e2285) as f64).fract() == 0.0 { if assign2070_e2285 == 0.0 { 0.0 } else { (assign2070_e2285 * ((assign2070_e2282).powf(assign2070_e2285 - 1.0) * (-(((locals.var_vl_dn4 * locals.var_pe_t) - (locals.var_vl * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign2070_e2286 * (assign2070_e2285 * ((-(((locals.var_vl_dn4 * locals.var_pe_t) - (locals.var_vl * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))) / assign2070_e2282))) })) / assign2070_e2290), ((assign2070_e2277 * if 0.0 == 0.0 && ((assign2070_e2285) as f64).is_finite() && ((assign2070_e2285) as f64).fract() == 0.0 { if assign2070_e2285 == 0.0 { 0.0 } else { (assign2070_e2285 * ((assign2070_e2282).powf(assign2070_e2285 - 1.0) * (-(locals.var_vl_dn8 / locals.var_pe_t)))) } } else { (assign2070_e2286 * (assign2070_e2285 * ((-(locals.var_vl_dn8 / locals.var_pe_t)) / assign2070_e2282))) }) / assign2070_e2290), ((assign2070_e2277 * if 0.0 == 0.0 && ((assign2070_e2285) as f64).is_finite() && ((assign2070_e2285) as f64).fract() == 0.0 { if assign2070_e2285 == 0.0 { 0.0 } else { (assign2070_e2285 * ((assign2070_e2282).powf(assign2070_e2285 - 1.0) * (-(locals.var_vl_dn9 / locals.var_pe_t)))) } } else { (assign2070_e2286 * (assign2070_e2285 * ((-(locals.var_vl_dn9 / locals.var_pe_t)) / assign2070_e2282))) }) / assign2070_e2290),)
    } else {
        (locals.var_qlo, locals.var_qlo_dn4, locals.var_qlo_dn8, locals.var_qlo_dn9,)
    }
};
        locals.var_qlo = assign2070_e2293;
        locals.var_qlo_dn4 = assign2070_e2293_d_n4;
        locals.var_qlo_dn8 = assign2070_e2293_d_n8;
        locals.var_qlo_dn9 = assign2070_e2293_d_n9;

        let (assign2080_e2331, assign2080_e2331_d_n4, assign2080_e2331_d_n8, assign2080_e2331_d_n9,) = {
    if (locals.var_guard52 == 0.0) {
        let assign2080_e2299: f64 = (1.0 - p.p34);
        let assign2080_e2301: f64 = (-p.p38);
        let assign2080_e2302: f64 = (assign2080_e2299).powf(assign2080_e2301);
        let assign2080_e2305: f64 = (locals.var_vbei - locals.var_vl);
        let assign2080_e2307: f64 = (assign2080_e2305 + locals.var_vl0);
        let assign2080_e2308: f64 = (assign2080_e2302 * assign2080_e2307);
        let assign2080_e2312: f64 = (0.5 * p.p38);
        let assign2080_e2315: f64 = (locals.var_vbei - locals.var_vl);
        let assign2080_e2317: f64 = (assign2080_e2315 + locals.var_vl0);
        let assign2080_e2318: f64 = (assign2080_e2312 * assign2080_e2317);
        let assign2080_e2322: f64 = (1.0 - p.p34);
        let assign2080_e2323: f64 = (locals.var_pe_t * assign2080_e2322);
        let assign2080_e2324: f64 = (assign2080_e2318 / assign2080_e2323);
        let assign2080_e2325: f64 = (1.0 + assign2080_e2324);
        let assign2080_e2326: f64 = (assign2080_e2308 * assign2080_e2325);
        let assign2080_e2327: f64 = (locals.var_qlo + assign2080_e2326);
        let assign2080_e2329: f64 = (assign2080_e2327 - locals.var_q0);
        (assign2080_e2329, ((locals.var_qlo_dn4 + (((assign2080_e2302 * ((-locals.var_vl_dn4) + locals.var_vl0_dn4)) * assign2080_e2325) + (assign2080_e2308 * ((((assign2080_e2312 * ((-locals.var_vl_dn4) + locals.var_vl0_dn4)) * assign2080_e2323) - (assign2080_e2318 * (locals.var_pe_t_dn4 * assign2080_e2322))) / (assign2080_e2323 * assign2080_e2323))))) - locals.var_q0_dn4), (locals.var_qlo_dn8 + (((assign2080_e2302 * (locals.var_vbei_dn8 - locals.var_vl_dn8)) * assign2080_e2325) + (assign2080_e2308 * ((assign2080_e2312 * (locals.var_vbei_dn8 - locals.var_vl_dn8)) / assign2080_e2323)))), (locals.var_qlo_dn9 + (((assign2080_e2302 * (locals.var_vbei_dn9 - locals.var_vl_dn9)) * assign2080_e2325) + (assign2080_e2308 * ((assign2080_e2312 * (locals.var_vbei_dn9 - locals.var_vl_dn9)) / assign2080_e2323)))),)
    } else {
        (locals.var_qdbe, locals.var_qdbe_dn4, locals.var_qdbe_dn8, locals.var_qdbe_dn9,)
    }
};
        locals.var_qdbe = assign2080_e2331;
        locals.var_qdbe_dn4 = assign2080_e2331_d_n4;
        locals.var_qdbe_dn8 = assign2080_e2331_d_n8;
        locals.var_qdbe_dn9 = assign2080_e2331_d_n9;

        let assign2090_e2333: f64 = (-locals.var_pc_t);
        let assign2090_e2335: f64 = (assign2090_e2333 * p.p34);
        locals.var_dv0__blk54 = assign2090_e2335;
        locals.var_dv0__blk54_dn4 = ((-locals.var_pc_t_dn4) * p.p34);

        let assign2100_e2338: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign2100_e2338;

        let (assign2110_e2344, assign2110_e2344_d_n4, assign2110_e2344_d_n6, assign2110_e2344_d_n8,) = {
    if (locals.var_guard75 != 0.0) {
        let assign2110_e2342: f64 = (locals.var_vbci + locals.var_dv0__blk54);
        (assign2110_e2342, locals.var_dv0__blk54_dn4, locals.var_vbci_dn6, locals.var_vbci_dn8,)
    } else {
        (locals.var_dvh__blk55, locals.var_dvh__blk55_dn4, locals.var_dvh__blk55_dn6, locals.var_dvh__blk55_dn8,)
    }
};
        locals.var_dvh__blk55 = assign2110_e2344;
        locals.var_dvh__blk55_dn4 = assign2110_e2344_d_n4;
        locals.var_dvh__blk55_dn6 = assign2110_e2344_d_n6;
        locals.var_dvh__blk55_dn8 = assign2110_e2344_d_n8;

        let assign2120_e2347: f64 = if locals.var_dvh__blk55 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard76 = assign2120_e2347;

        let (assign2130_e2360,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign2130_e2353: f64 = (1.0 - p.p34);
        let assign2130_e2355: f64 = (-1.0);
        let assign2130_e2357: f64 = (assign2130_e2355 - p.p43);
        let assign2130_e2358: f64 = (assign2130_e2353).powf(assign2130_e2357);
        (assign2130_e2358,)
    } else {
        (locals.var_pwq__blk56,)
    }
};
        locals.var_pwq__blk56 = assign2130_e2360;

        let (assign2140_e2382, assign2140_e2382_d_n4, assign2140_e2382_d_n6, assign2140_e2382_d_n8,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign2140_e2369: f64 = (1.0 - p.p34);
        let assign2140_e2370: f64 = (locals.var_pwq__blk56 * assign2140_e2369);
        let assign2140_e2373: f64 = (1.0 - p.p34);
        let assign2140_e2374: f64 = (assign2140_e2370 * assign2140_e2373);
        let assign2140_e2375: f64 = (1.0 - assign2140_e2374);
        let assign2140_e2376: f64 = (locals.var_pc_t * assign2140_e2375);
        let assign2140_e2379: f64 = (1.0 - p.p43);
        let assign2140_e2380: f64 = (assign2140_e2376 / assign2140_e2379);
        (assign2140_e2380, ((locals.var_pc_t_dn4 * assign2140_e2375) / assign2140_e2379), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2140_e2382;
        locals.var_qlo__blk57_dn4 = assign2140_e2382_d_n4;
        locals.var_qlo__blk57_dn6 = assign2140_e2382_d_n6;
        locals.var_qlo__blk57_dn8 = assign2140_e2382_d_n8;

        let (assign2150_e2402, assign2150_e2402_d_n4, assign2150_e2402_d_n6, assign2150_e2402_d_n8,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign2150_e2389: f64 = (1.0 - p.p34);
        let assign2150_e2392: f64 = (0.5 * p.p43);
        let assign2150_e2394: f64 = (assign2150_e2392 * locals.var_dvh__blk55);
        let assign2150_e2396: f64 = (assign2150_e2394 / locals.var_pc_t);
        let assign2150_e2397: f64 = (assign2150_e2389 + assign2150_e2396);
        let assign2150_e2398: f64 = (locals.var_dvh__blk55 * assign2150_e2397);
        let assign2150_e2400: f64 = (assign2150_e2398 * locals.var_pwq__blk56);
        (assign2150_e2400, (((locals.var_dvh__blk55_dn4 * assign2150_e2397) + (locals.var_dvh__blk55 * ((((assign2150_e2392 * locals.var_dvh__blk55_dn4) * locals.var_pc_t) - (assign2150_e2394 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)))) * locals.var_pwq__blk56), (((locals.var_dvh__blk55_dn6 * assign2150_e2397) + (locals.var_dvh__blk55 * ((assign2150_e2392 * locals.var_dvh__blk55_dn6) / locals.var_pc_t))) * locals.var_pwq__blk56), (((locals.var_dvh__blk55_dn8 * assign2150_e2397) + (locals.var_dvh__blk55 * ((assign2150_e2392 * locals.var_dvh__blk55_dn8) / locals.var_pc_t))) * locals.var_pwq__blk56),)
    } else {
        (locals.var_qhi__blk58, locals.var_qhi__blk58_dn4, locals.var_qhi__blk58_dn6, locals.var_qhi__blk58_dn8,)
    }
};
        locals.var_qhi__blk58 = assign2150_e2402;
        locals.var_qhi__blk58_dn4 = assign2150_e2402_d_n4;
        locals.var_qhi__blk58_dn6 = assign2150_e2402_d_n6;
        locals.var_qhi__blk58_dn8 = assign2150_e2402_d_n8;

        let assign2160_e2408: f64 = (-p.p45);
        let assign2160_e2410: f64 = if ((p.p45 > 0.0) && (locals.var_vbci < assign2160_e2408)) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign2160_e2410;

        let (assign2170_e2449, assign2170_e2449_d_n4, assign2170_e2449_d_n6, assign2170_e2449_d_n8,) = {
    if (((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) && (locals.var_guard77 != 0.0)) {
        let assign2170_e2422: f64 = (p.p45 / locals.var_pc_t);
        let assign2170_e2423: f64 = (1.0 + assign2170_e2422);
        let assign2170_e2426: f64 = (1.0 - p.p43);
        let assign2170_e2427: f64 = (assign2170_e2423).powf(assign2170_e2426);
        let assign2170_e2431: f64 = (1.0 - p.p43);
        let assign2170_e2434: f64 = (locals.var_vbci + p.p45);
        let assign2170_e2435: f64 = (assign2170_e2431 * assign2170_e2434);
        let assign2170_e2438: f64 = (locals.var_pc_t + p.p45);
        let assign2170_e2439: f64 = (assign2170_e2435 / assign2170_e2438);
        let assign2170_e2440: f64 = (1.0 - assign2170_e2439);
        let assign2170_e2441: f64 = (assign2170_e2427 * assign2170_e2440);
        let assign2170_e2442: f64 = (1.0 - assign2170_e2441);
        let assign2170_e2443: f64 = (locals.var_pc_t * assign2170_e2442);
        let assign2170_e2446: f64 = (1.0 - p.p43);
        let assign2170_e2447: f64 = (assign2170_e2443 / assign2170_e2446);
        (assign2170_e2447, (((locals.var_pc_t_dn4 * assign2170_e2442) + (locals.var_pc_t * (-((if 0.0 == 0.0 && ((assign2170_e2426) as f64).is_finite() && ((assign2170_e2426) as f64).fract() == 0.0 { if assign2170_e2426 == 0.0 { 0.0 } else { (assign2170_e2426 * ((assign2170_e2423).powf(assign2170_e2426 - 1.0) * (-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2170_e2427 * (assign2170_e2426 * ((-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign2170_e2423))) } * assign2170_e2440) + (assign2170_e2427 * (-(-((assign2170_e2435 * locals.var_pc_t_dn4) / (assign2170_e2438 * assign2170_e2438))))))))) / assign2170_e2446), ((locals.var_pc_t * (-(assign2170_e2427 * (-((assign2170_e2431 * locals.var_vbci_dn6) / assign2170_e2438))))) / assign2170_e2446), ((locals.var_pc_t * (-(assign2170_e2427 * (-((assign2170_e2431 * locals.var_vbci_dn8) / assign2170_e2438))))) / assign2170_e2446),)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2170_e2449;
        locals.var_qlo__blk57_dn4 = assign2170_e2449_d_n4;
        locals.var_qlo__blk57_dn6 = assign2170_e2449_d_n6;
        locals.var_qlo__blk57_dn8 = assign2170_e2449_d_n8;

        let (assign2180_e2475, assign2180_e2475_d_n4, assign2180_e2475_d_n6, assign2180_e2475_d_n8,) = {
    if (((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) && (locals.var_guard77 == 0.0)) {
        let assign2180_e2462: f64 = (locals.var_vbci / locals.var_pc_t);
        let assign2180_e2463: f64 = (1.0 - assign2180_e2462);
        let assign2180_e2466: f64 = (1.0 - p.p43);
        let assign2180_e2467: f64 = (assign2180_e2463).powf(assign2180_e2466);
        let assign2180_e2468: f64 = (1.0 - assign2180_e2467);
        let assign2180_e2469: f64 = (locals.var_pc_t * assign2180_e2468);
        let assign2180_e2472: f64 = (1.0 - p.p43);
        let assign2180_e2473: f64 = (assign2180_e2469 / assign2180_e2472);
        (assign2180_e2473, (((locals.var_pc_t_dn4 * assign2180_e2468) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2180_e2466) as f64).is_finite() && ((assign2180_e2466) as f64).fract() == 0.0 { if assign2180_e2466 == 0.0 { 0.0 } else { (assign2180_e2466 * ((assign2180_e2463).powf(assign2180_e2466 - 1.0) * (-(-((locals.var_vbci * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t)))))) } } else { (assign2180_e2467 * (assign2180_e2466 * ((-(-((locals.var_vbci * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t)))) / assign2180_e2463))) }))) / assign2180_e2472), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2180_e2466) as f64).is_finite() && ((assign2180_e2466) as f64).fract() == 0.0 { if assign2180_e2466 == 0.0 { 0.0 } else { (assign2180_e2466 * ((assign2180_e2463).powf(assign2180_e2466 - 1.0) * (-(locals.var_vbci_dn6 / locals.var_pc_t)))) } } else { (assign2180_e2467 * (assign2180_e2466 * ((-(locals.var_vbci_dn6 / locals.var_pc_t)) / assign2180_e2463))) })) / assign2180_e2472), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2180_e2466) as f64).is_finite() && ((assign2180_e2466) as f64).fract() == 0.0 { if assign2180_e2466 == 0.0 { 0.0 } else { (assign2180_e2466 * ((assign2180_e2463).powf(assign2180_e2466 - 1.0) * (-(locals.var_vbci_dn8 / locals.var_pc_t)))) } } else { (assign2180_e2467 * (assign2180_e2466 * ((-(locals.var_vbci_dn8 / locals.var_pc_t)) / assign2180_e2463))) })) / assign2180_e2472),)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2180_e2475;
        locals.var_qlo__blk57_dn4 = assign2180_e2475_d_n4;
        locals.var_qlo__blk57_dn6 = assign2180_e2475_d_n6;
        locals.var_qlo__blk57_dn8 = assign2180_e2475_d_n8;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2190_e2482, assign2190_e2482_d_n4, assign2190_e2482_d_n6, assign2190_e2482_d_n8,) = {
    if ((locals.var_guard75 != 0.0) && (locals.var_guard76 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk58, locals.var_qhi__blk58_dn4, locals.var_qhi__blk58_dn6, locals.var_qhi__blk58_dn8,)
    }
};
        locals.var_qhi__blk58 = assign2190_e2482;
        locals.var_qhi__blk58_dn4 = assign2190_e2482_d_n4;
        locals.var_qhi__blk58_dn6 = assign2190_e2482_d_n6;
        locals.var_qhi__blk58_dn8 = assign2190_e2482_d_n8;

        let (assign2200_e2488, assign2200_e2488_d_n4, assign2200_e2488_d_n6, assign2200_e2488_d_n8,) = {
    if (locals.var_guard75 != 0.0) {
        let assign2200_e2486: f64 = (locals.var_qlo__blk57 + locals.var_qhi__blk58);
        (assign2200_e2486, (locals.var_qlo__blk57_dn4 + locals.var_qhi__blk58_dn4), (locals.var_qlo__blk57_dn6 + locals.var_qhi__blk58_dn6), (locals.var_qlo__blk57_dn8 + locals.var_qhi__blk58_dn8),)
    } else {
        (locals.var_qdbc, locals.var_qdbc_dn4, locals.var_qdbc_dn6, locals.var_qdbc_dn8,)
    }
};
        locals.var_qdbc = assign2200_e2488;
        locals.var_qdbc_dn4 = assign2200_e2488_d_n4;
        locals.var_qdbc_dn6 = assign2200_e2488_d_n6;
        locals.var_qdbc_dn8 = assign2200_e2488_d_n8;

        let assign2210_e2495: f64 = if ((p.p45 > 0.0) && (p.p46 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign2210_e2495;

        let (assign2220_e2508, assign2220_e2508_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2220_e2502: f64 = (p.p45 + locals.var_dv0__blk54);
        let assign2220_e2505: f64 = (p.p45 - locals.var_dv0__blk54);
        let assign2220_e2506: f64 = (assign2220_e2502 / assign2220_e2505);
        (assign2220_e2506, (((locals.var_dv0__blk54_dn4 * assign2220_e2505) - (assign2220_e2502 * (-locals.var_dv0__blk54_dn4))) / (assign2220_e2505 * assign2220_e2505)),)
    } else {
        (locals.var_vn0, locals.var_vn0_dn4,)
    }
};
        locals.var_vn0 = assign2220_e2508;
        locals.var_vn0_dn4 = assign2220_e2508_d_n4;

        let (assign2230_e2547, assign2230_e2547_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2230_e2515: f64 = (2.0 * locals.var_vn0);
        let assign2230_e2518: f64 = (locals.var_vn0 - 1.0);
        let assign2230_e2521: f64 = (locals.var_vn0 - 1.0);
        let assign2230_e2522: f64 = (assign2230_e2518 * assign2230_e2521);
        let assign2230_e2525: f64 = (4.0 * p.p44);
        let assign2230_e2527: f64 = (assign2230_e2525 * p.p44);
        let assign2230_e2528: f64 = (assign2230_e2522 + assign2230_e2527);
        let assign2230_e2529: f64 = (assign2230_e2528).sqrt();
        let assign2230_e2532: f64 = (locals.var_vn0 + 1.0);
        let assign2230_e2535: f64 = (locals.var_vn0 + 1.0);
        let assign2230_e2536: f64 = (assign2230_e2532 * assign2230_e2535);
        let assign2230_e2539: f64 = (4.0 * p.p46);
        let assign2230_e2541: f64 = (assign2230_e2539 * p.p46);
        let assign2230_e2542: f64 = (assign2230_e2536 + assign2230_e2541);
        let assign2230_e2543: f64 = (assign2230_e2542).sqrt();
        let assign2230_e2544: f64 = (assign2230_e2529 + assign2230_e2543);
        let assign2230_e2545: f64 = (assign2230_e2515 / assign2230_e2544);
        (assign2230_e2545, ((((2.0 * locals.var_vn0_dn4) * assign2230_e2544) - (assign2230_e2515 * ((((locals.var_vn0_dn4 * assign2230_e2521) + (assign2230_e2518 * locals.var_vn0_dn4)) / (2.0 * assign2230_e2529)) + (((locals.var_vn0_dn4 * assign2230_e2535) + (assign2230_e2532 * locals.var_vn0_dn4)) / (2.0 * assign2230_e2543))))) / (assign2230_e2544 * assign2230_e2544)),)
    } else {
        (locals.var_vnl0, locals.var_vnl0_dn4,)
    }
};
        locals.var_vnl0 = assign2230_e2547;
        locals.var_vnl0_dn4 = assign2230_e2547_d_n4;

        let (assign2240_e2564, assign2240_e2564_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2240_e2556: f64 = (p.p45 - locals.var_dv0__blk54);
        let assign2240_e2557: f64 = (locals.var_vnl0 * assign2240_e2556);
        let assign2240_e2559: f64 = (assign2240_e2557 - p.p45);
        let assign2240_e2561: f64 = (assign2240_e2559 - locals.var_dv0__blk54);
        let assign2240_e2562: f64 = (0.5 * assign2240_e2561);
        (assign2240_e2562, (0.5 * (((locals.var_vnl0_dn4 * assign2240_e2556) + (locals.var_vnl0 * (-locals.var_dv0__blk54_dn4))) - locals.var_dv0__blk54_dn4)),)
    } else {
        (locals.var_vl0__blk61, locals.var_vl0__blk61_dn4,)
    }
};
        locals.var_vl0__blk61 = assign2240_e2564;
        locals.var_vl0__blk61_dn4 = assign2240_e2564_d_n4;

        let (assign2250_e2587, assign2250_e2587_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2250_e2574: f64 = (locals.var_vl0__blk61 / locals.var_pc_t);
        let assign2250_e2575: f64 = (1.0 - assign2250_e2574);
        let assign2250_e2578: f64 = (1.0 - p.p43);
        let assign2250_e2579: f64 = (assign2250_e2575).powf(assign2250_e2578);
        let assign2250_e2580: f64 = (1.0 - assign2250_e2579);
        let assign2250_e2581: f64 = (locals.var_pc_t * assign2250_e2580);
        let assign2250_e2584: f64 = (1.0 - p.p43);
        let assign2250_e2585: f64 = (assign2250_e2581 / assign2250_e2584);
        (assign2250_e2585, (((locals.var_pc_t_dn4 * assign2250_e2580) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2250_e2578) as f64).is_finite() && ((assign2250_e2578) as f64).fract() == 0.0 { if assign2250_e2578 == 0.0 { 0.0 } else { (assign2250_e2578 * ((assign2250_e2575).powf(assign2250_e2578 - 1.0) * (-(((locals.var_vl0__blk61_dn4 * locals.var_pc_t) - (locals.var_vl0__blk61 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2250_e2579 * (assign2250_e2578 * ((-(((locals.var_vl0__blk61_dn4 * locals.var_pc_t) - (locals.var_vl0__blk61 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign2250_e2575))) }))) / assign2250_e2584),)
    } else {
        (locals.var_qlo0, locals.var_qlo0_dn4,)
    }
};
        locals.var_qlo0 = assign2250_e2587;
        locals.var_qlo0_dn4 = assign2250_e2587_d_n4;

        let (assign2260_e2604, assign2260_e2604_d_n4, assign2260_e2604_d_n6, assign2260_e2604_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2260_e2594: f64 = (2.0 * locals.var_vbci);
        let assign2260_e2596: f64 = (assign2260_e2594 + p.p45);
        let assign2260_e2598: f64 = (assign2260_e2596 + locals.var_dv0__blk54);
        let assign2260_e2601: f64 = (p.p45 - locals.var_dv0__blk54);
        let assign2260_e2602: f64 = (assign2260_e2598 / assign2260_e2601);
        (assign2260_e2602, (((locals.var_dv0__blk54_dn4 * assign2260_e2601) - (assign2260_e2598 * (-locals.var_dv0__blk54_dn4))) / (assign2260_e2601 * assign2260_e2601)), ((2.0 * locals.var_vbci_dn6) / assign2260_e2601), ((2.0 * locals.var_vbci_dn8) / assign2260_e2601),)
    } else {
        (locals.var_vn, locals.var_vn_dn4, locals.var_vn_dn6, locals.var_vn_dn8,)
    }
};
        locals.var_vn = assign2260_e2604;
        locals.var_vn_dn4 = assign2260_e2604_d_n4;
        locals.var_vn_dn6 = assign2260_e2604_d_n6;
        locals.var_vn_dn8 = assign2260_e2604_d_n8;

        let (assign2270_e2643, assign2270_e2643_d_n4, assign2270_e2643_d_n6, assign2270_e2643_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2270_e2611: f64 = (2.0 * locals.var_vn);
        let assign2270_e2614: f64 = (locals.var_vn - 1.0);
        let assign2270_e2617: f64 = (locals.var_vn - 1.0);
        let assign2270_e2618: f64 = (assign2270_e2614 * assign2270_e2617);
        let assign2270_e2621: f64 = (4.0 * p.p44);
        let assign2270_e2623: f64 = (assign2270_e2621 * p.p44);
        let assign2270_e2624: f64 = (assign2270_e2618 + assign2270_e2623);
        let assign2270_e2625: f64 = (assign2270_e2624).sqrt();
        let assign2270_e2628: f64 = (locals.var_vn + 1.0);
        let assign2270_e2631: f64 = (locals.var_vn + 1.0);
        let assign2270_e2632: f64 = (assign2270_e2628 * assign2270_e2631);
        let assign2270_e2635: f64 = (4.0 * p.p46);
        let assign2270_e2637: f64 = (assign2270_e2635 * p.p46);
        let assign2270_e2638: f64 = (assign2270_e2632 + assign2270_e2637);
        let assign2270_e2639: f64 = (assign2270_e2638).sqrt();
        let assign2270_e2640: f64 = (assign2270_e2625 + assign2270_e2639);
        let assign2270_e2641: f64 = (assign2270_e2611 / assign2270_e2640);
        (assign2270_e2641, ((((2.0 * locals.var_vn_dn4) * assign2270_e2640) - (assign2270_e2611 * ((((locals.var_vn_dn4 * assign2270_e2617) + (assign2270_e2614 * locals.var_vn_dn4)) / (2.0 * assign2270_e2625)) + (((locals.var_vn_dn4 * assign2270_e2631) + (assign2270_e2628 * locals.var_vn_dn4)) / (2.0 * assign2270_e2639))))) / (assign2270_e2640 * assign2270_e2640)), ((((2.0 * locals.var_vn_dn6) * assign2270_e2640) - (assign2270_e2611 * ((((locals.var_vn_dn6 * assign2270_e2617) + (assign2270_e2614 * locals.var_vn_dn6)) / (2.0 * assign2270_e2625)) + (((locals.var_vn_dn6 * assign2270_e2631) + (assign2270_e2628 * locals.var_vn_dn6)) / (2.0 * assign2270_e2639))))) / (assign2270_e2640 * assign2270_e2640)), ((((2.0 * locals.var_vn_dn8) * assign2270_e2640) - (assign2270_e2611 * ((((locals.var_vn_dn8 * assign2270_e2617) + (assign2270_e2614 * locals.var_vn_dn8)) / (2.0 * assign2270_e2625)) + (((locals.var_vn_dn8 * assign2270_e2631) + (assign2270_e2628 * locals.var_vn_dn8)) / (2.0 * assign2270_e2639))))) / (assign2270_e2640 * assign2270_e2640)),)
    } else {
        (locals.var_vnl, locals.var_vnl_dn4, locals.var_vnl_dn6, locals.var_vnl_dn8,)
    }
};
        locals.var_vnl = assign2270_e2643;
        locals.var_vnl_dn4 = assign2270_e2643_d_n4;
        locals.var_vnl_dn6 = assign2270_e2643_d_n6;
        locals.var_vnl_dn8 = assign2270_e2643_d_n8;

        let (assign2280_e2660, assign2280_e2660_d_n4, assign2280_e2660_d_n6, assign2280_e2660_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2280_e2652: f64 = (p.p45 - locals.var_dv0__blk54);
        let assign2280_e2653: f64 = (locals.var_vnl * assign2280_e2652);
        let assign2280_e2655: f64 = (assign2280_e2653 - p.p45);
        let assign2280_e2657: f64 = (assign2280_e2655 - locals.var_dv0__blk54);
        let assign2280_e2658: f64 = (0.5 * assign2280_e2657);
        (assign2280_e2658, (0.5 * (((locals.var_vnl_dn4 * assign2280_e2652) + (locals.var_vnl * (-locals.var_dv0__blk54_dn4))) - locals.var_dv0__blk54_dn4)), (0.5 * (locals.var_vnl_dn6 * assign2280_e2652)), (0.5 * (locals.var_vnl_dn8 * assign2280_e2652)),)
    } else {
        (locals.var_vl__blk65, locals.var_vl__blk65_dn4, locals.var_vl__blk65_dn6, locals.var_vl__blk65_dn8,)
    }
};
        locals.var_vl__blk65 = assign2280_e2660;
        locals.var_vl__blk65_dn4 = assign2280_e2660_d_n4;
        locals.var_vl__blk65_dn6 = assign2280_e2660_d_n6;
        locals.var_vl__blk65_dn8 = assign2280_e2660_d_n8;

        let (assign2290_e2683, assign2290_e2683_d_n4, assign2290_e2683_d_n6, assign2290_e2683_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2290_e2670: f64 = (locals.var_vl__blk65 / locals.var_pc_t);
        let assign2290_e2671: f64 = (1.0 - assign2290_e2670);
        let assign2290_e2674: f64 = (1.0 - p.p43);
        let assign2290_e2675: f64 = (assign2290_e2671).powf(assign2290_e2674);
        let assign2290_e2676: f64 = (1.0 - assign2290_e2675);
        let assign2290_e2677: f64 = (locals.var_pc_t * assign2290_e2676);
        let assign2290_e2680: f64 = (1.0 - p.p43);
        let assign2290_e2681: f64 = (assign2290_e2677 / assign2290_e2680);
        (assign2290_e2681, (((locals.var_pc_t_dn4 * assign2290_e2676) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2290_e2674) as f64).is_finite() && ((assign2290_e2674) as f64).fract() == 0.0 { if assign2290_e2674 == 0.0 { 0.0 } else { (assign2290_e2674 * ((assign2290_e2671).powf(assign2290_e2674 - 1.0) * (-(((locals.var_vl__blk65_dn4 * locals.var_pc_t) - (locals.var_vl__blk65 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2290_e2675 * (assign2290_e2674 * ((-(((locals.var_vl__blk65_dn4 * locals.var_pc_t) - (locals.var_vl__blk65 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign2290_e2671))) }))) / assign2290_e2680), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2290_e2674) as f64).is_finite() && ((assign2290_e2674) as f64).fract() == 0.0 { if assign2290_e2674 == 0.0 { 0.0 } else { (assign2290_e2674 * ((assign2290_e2671).powf(assign2290_e2674 - 1.0) * (-(locals.var_vl__blk65_dn6 / locals.var_pc_t)))) } } else { (assign2290_e2675 * (assign2290_e2674 * ((-(locals.var_vl__blk65_dn6 / locals.var_pc_t)) / assign2290_e2671))) })) / assign2290_e2680), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign2290_e2674) as f64).is_finite() && ((assign2290_e2674) as f64).fract() == 0.0 { if assign2290_e2674 == 0.0 { 0.0 } else { (assign2290_e2674 * ((assign2290_e2671).powf(assign2290_e2674 - 1.0) * (-(locals.var_vl__blk65_dn8 / locals.var_pc_t)))) } } else { (assign2290_e2675 * (assign2290_e2674 * ((-(locals.var_vl__blk65_dn8 / locals.var_pc_t)) / assign2290_e2671))) })) / assign2290_e2680),)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2290_e2683;
        locals.var_qlo__blk57_dn4 = assign2290_e2683_d_n4;
        locals.var_qlo__blk57_dn6 = assign2290_e2683_d_n6;
        locals.var_qlo__blk57_dn8 = assign2290_e2683_d_n8;

        let (assign2300_e2694, assign2300_e2694_d_n4, assign2300_e2694_d_n6, assign2300_e2694_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2300_e2691: f64 = (locals.var_vnl + 1.0);
        let assign2300_e2692: f64 = (0.5 * assign2300_e2691);
        (assign2300_e2692, (0.5 * locals.var_vnl_dn4), (0.5 * locals.var_vnl_dn6), (0.5 * locals.var_vnl_dn8),)
    } else {
        (locals.var_sel, locals.var_sel_dn4, locals.var_sel_dn6, locals.var_sel_dn8,)
    }
};
        locals.var_sel = assign2300_e2694;
        locals.var_sel_dn4 = assign2300_e2694_d_n4;
        locals.var_sel_dn6 = assign2300_e2694_d_n6;
        locals.var_sel_dn8 = assign2300_e2694_d_n8;

        let (assign2310_e2708, assign2310_e2708_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2310_e2702: f64 = (p.p45 / locals.var_pc_t);
        let assign2310_e2703: f64 = (1.0 + assign2310_e2702);
        let assign2310_e2705: f64 = (-p.p43);
        let assign2310_e2706: f64 = (assign2310_e2703).powf(assign2310_e2705);
        (assign2310_e2706, if 0.0 == 0.0 && ((assign2310_e2705) as f64).is_finite() && ((assign2310_e2705) as f64).fract() == 0.0 { if assign2310_e2705 == 0.0 { 0.0 } else { (assign2310_e2705 * ((assign2310_e2703).powf(assign2310_e2705 - 1.0) * (-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2310_e2706 * (assign2310_e2705 * ((-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign2310_e2703))) },)
    } else {
        (locals.var_crt, locals.var_crt_dn4,)
    }
};
        locals.var_crt = assign2310_e2708;
        locals.var_crt_dn4 = assign2310_e2708_d_n4;

        let (assign2320_e2722, assign2320_e2722_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2320_e2716: f64 = (locals.var_dv0__blk54 / locals.var_pc_t);
        let assign2320_e2717: f64 = (1.0 + assign2320_e2716);
        let assign2320_e2719: f64 = (-p.p43);
        let assign2320_e2720: f64 = (assign2320_e2717).powf(assign2320_e2719);
        (assign2320_e2720, if 0.0 == 0.0 && ((assign2320_e2719) as f64).is_finite() && ((assign2320_e2719) as f64).fract() == 0.0 { if assign2320_e2719 == 0.0 { 0.0 } else { (assign2320_e2719 * ((assign2320_e2717).powf(assign2320_e2719 - 1.0) * (((locals.var_dv0__blk54_dn4 * locals.var_pc_t) - (locals.var_dv0__blk54 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)))) } } else { (assign2320_e2720 * (assign2320_e2719 * ((((locals.var_dv0__blk54_dn4 * locals.var_pc_t) - (locals.var_dv0__blk54 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)) / assign2320_e2717))) },)
    } else {
        (locals.var_cmx, locals.var_cmx_dn4,)
    }
};
        locals.var_cmx = assign2320_e2722;
        locals.var_cmx_dn4 = assign2320_e2722_d_n4;

        let (assign2330_e2737, assign2330_e2737_d_n4, assign2330_e2737_d_n6, assign2330_e2737_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2330_e2729: f64 = (1.0 - locals.var_sel);
        let assign2330_e2731: f64 = (assign2330_e2729 * locals.var_crt);
        let assign2330_e2734: f64 = (locals.var_sel * locals.var_cmx);
        let assign2330_e2735: f64 = (assign2330_e2731 + assign2330_e2734);
        (assign2330_e2735, ((((-locals.var_sel_dn4) * locals.var_crt) + (assign2330_e2729 * locals.var_crt_dn4)) + ((locals.var_sel_dn4 * locals.var_cmx) + (locals.var_sel * locals.var_cmx_dn4))), (((-locals.var_sel_dn6) * locals.var_crt) + (locals.var_sel_dn6 * locals.var_cmx)), (((-locals.var_sel_dn8) * locals.var_crt) + (locals.var_sel_dn8 * locals.var_cmx)),)
    } else {
        (locals.var_cl, locals.var_cl_dn4, locals.var_cl_dn6, locals.var_cl_dn8,)
    }
};
        locals.var_cl = assign2330_e2737;
        locals.var_cl_dn4 = assign2330_e2737_d_n4;
        locals.var_cl_dn6 = assign2330_e2737_d_n6;
        locals.var_cl_dn8 = assign2330_e2737_d_n8;

        let (assign2340_e2750, assign2340_e2750_d_n4, assign2340_e2750_d_n6, assign2340_e2750_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2340_e2744: f64 = (locals.var_vbci - locals.var_vl__blk65);
        let assign2340_e2746: f64 = (assign2340_e2744 + locals.var_vl0__blk61);
        let assign2340_e2748: f64 = (assign2340_e2746 * locals.var_cl);
        (assign2340_e2748, ((((-locals.var_vl__blk65_dn4) + locals.var_vl0__blk61_dn4) * locals.var_cl) + (assign2340_e2746 * locals.var_cl_dn4)), (((locals.var_vbci_dn6 - locals.var_vl__blk65_dn6) * locals.var_cl) + (assign2340_e2746 * locals.var_cl_dn6)), (((locals.var_vbci_dn8 - locals.var_vl__blk65_dn8) * locals.var_cl) + (assign2340_e2746 * locals.var_cl_dn8)),)
    } else {
        (locals.var_ql, locals.var_ql_dn4, locals.var_ql_dn6, locals.var_ql_dn8,)
    }
};
        locals.var_ql = assign2340_e2750;
        locals.var_ql_dn4 = assign2340_e2750_d_n4;
        locals.var_ql_dn6 = assign2340_e2750_d_n6;
        locals.var_ql_dn8 = assign2340_e2750_d_n8;

        let (assign2350_e2761, assign2350_e2761_d_n4, assign2350_e2761_d_n6, assign2350_e2761_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 != 0.0)) {
        let assign2350_e2757: f64 = (locals.var_ql + locals.var_qlo__blk57);
        let assign2350_e2759: f64 = (assign2350_e2757 - locals.var_qlo0);
        (assign2350_e2759, ((locals.var_ql_dn4 + locals.var_qlo__blk57_dn4) - locals.var_qlo0_dn4), (locals.var_ql_dn6 + locals.var_qlo__blk57_dn6), (locals.var_ql_dn8 + locals.var_qlo__blk57_dn8),)
    } else {
        (locals.var_qdbc, locals.var_qdbc_dn4, locals.var_qdbc_dn6, locals.var_qdbc_dn8,)
    }
};
        locals.var_qdbc = assign2350_e2761;
        locals.var_qdbc_dn4 = assign2350_e2761_d_n4;
        locals.var_qdbc_dn6 = assign2350_e2761_d_n6;
        locals.var_qdbc_dn8 = assign2350_e2761_d_n8;

        let (assign2360_e2778, assign2360_e2778_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2360_e2769: f64 = (locals.var_dv0__blk54 * locals.var_dv0__blk54);
        let assign2360_e2772: f64 = (4.0 * p.p44);
        let assign2360_e2774: f64 = (assign2360_e2772 * p.p44);
        let assign2360_e2775: f64 = (assign2360_e2769 + assign2360_e2774);
        let assign2360_e2776: f64 = (assign2360_e2775).sqrt();
        (assign2360_e2776, (((locals.var_dv0__blk54_dn4 * locals.var_dv0__blk54) + (locals.var_dv0__blk54 * locals.var_dv0__blk54_dn4)) / (2.0 * assign2360_e2776)),)
    } else {
        (locals.var_mv0__blk71, locals.var_mv0__blk71_dn4,)
    }
};
        locals.var_mv0__blk71 = assign2360_e2778;
        locals.var_mv0__blk71_dn4 = assign2360_e2778_d_n4;

        let (assign2370_e2791, assign2370_e2791_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2370_e2785: f64 = (-0.5);
        let assign2370_e2788: f64 = (locals.var_dv0__blk54 + locals.var_mv0__blk71);
        let assign2370_e2789: f64 = (assign2370_e2785 * assign2370_e2788);
        (assign2370_e2789, (assign2370_e2785 * (locals.var_dv0__blk54_dn4 + locals.var_mv0__blk71_dn4)),)
    } else {
        (locals.var_vl0__blk61, locals.var_vl0__blk61_dn4,)
    }
};
        locals.var_vl0__blk61 = assign2370_e2791;
        locals.var_vl0__blk61_dn4 = assign2370_e2791_d_n4;

        let (assign2380_e2814, assign2380_e2814_d_n4,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2380_e2798: f64 = (-locals.var_pc_t);
        let assign2380_e2802: f64 = (locals.var_vl0__blk61 / locals.var_pc_t);
        let assign2380_e2803: f64 = (1.0 - assign2380_e2802);
        let assign2380_e2806: f64 = (1.0 - p.p43);
        let assign2380_e2807: f64 = (assign2380_e2803).powf(assign2380_e2806);
        let assign2380_e2808: f64 = (assign2380_e2798 * assign2380_e2807);
        let assign2380_e2811: f64 = (1.0 - p.p43);
        let assign2380_e2812: f64 = (assign2380_e2808 / assign2380_e2811);
        (assign2380_e2812, ((((-locals.var_pc_t_dn4) * assign2380_e2807) + (assign2380_e2798 * if 0.0 == 0.0 && ((assign2380_e2806) as f64).is_finite() && ((assign2380_e2806) as f64).fract() == 0.0 { if assign2380_e2806 == 0.0 { 0.0 } else { (assign2380_e2806 * ((assign2380_e2803).powf(assign2380_e2806 - 1.0) * (-(((locals.var_vl0__blk61_dn4 * locals.var_pc_t) - (locals.var_vl0__blk61 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2380_e2807 * (assign2380_e2806 * ((-(((locals.var_vl0__blk61_dn4 * locals.var_pc_t) - (locals.var_vl0__blk61 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign2380_e2803))) })) / assign2380_e2811),)
    } else {
        (locals.var_q0__blk72, locals.var_q0__blk72_dn4,)
    }
};
        locals.var_q0__blk72 = assign2380_e2814;
        locals.var_q0__blk72_dn4 = assign2380_e2814_d_n4;

        let (assign2390_e2824, assign2390_e2824_d_n4, assign2390_e2824_d_n6, assign2390_e2824_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2390_e2822: f64 = (locals.var_vbci + locals.var_dv0__blk54);
        (assign2390_e2822, locals.var_dv0__blk54_dn4, locals.var_vbci_dn6, locals.var_vbci_dn8,)
    } else {
        (locals.var_dv__blk73, locals.var_dv__blk73_dn4, locals.var_dv__blk73_dn6, locals.var_dv__blk73_dn8,)
    }
};
        locals.var_dv__blk73 = assign2390_e2824;
        locals.var_dv__blk73_dn4 = assign2390_e2824_d_n4;
        locals.var_dv__blk73_dn6 = assign2390_e2824_d_n6;
        locals.var_dv__blk73_dn8 = assign2390_e2824_d_n8;

        let (assign2400_e2841, assign2400_e2841_d_n4, assign2400_e2841_d_n6, assign2400_e2841_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2400_e2832: f64 = (locals.var_dv__blk73 * locals.var_dv__blk73);
        let assign2400_e2835: f64 = (4.0 * p.p44);
        let assign2400_e2837: f64 = (assign2400_e2835 * p.p44);
        let assign2400_e2838: f64 = (assign2400_e2832 + assign2400_e2837);
        let assign2400_e2839: f64 = (assign2400_e2838).sqrt();
        (assign2400_e2839, (((locals.var_dv__blk73_dn4 * locals.var_dv__blk73) + (locals.var_dv__blk73 * locals.var_dv__blk73_dn4)) / (2.0 * assign2400_e2839)), (((locals.var_dv__blk73_dn6 * locals.var_dv__blk73) + (locals.var_dv__blk73 * locals.var_dv__blk73_dn6)) / (2.0 * assign2400_e2839)), (((locals.var_dv__blk73_dn8 * locals.var_dv__blk73) + (locals.var_dv__blk73 * locals.var_dv__blk73_dn8)) / (2.0 * assign2400_e2839)),)
    } else {
        (locals.var_mv__blk74, locals.var_mv__blk74_dn4, locals.var_mv__blk74_dn6, locals.var_mv__blk74_dn8,)
    }
};
        locals.var_mv__blk74 = assign2400_e2841;
        locals.var_mv__blk74_dn4 = assign2400_e2841_d_n4;
        locals.var_mv__blk74_dn6 = assign2400_e2841_d_n6;
        locals.var_mv__blk74_dn8 = assign2400_e2841_d_n8;

        let (assign2410_e2855, assign2410_e2855_d_n4, assign2410_e2855_d_n6, assign2410_e2855_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2410_e2850: f64 = (locals.var_dv__blk73 - locals.var_mv__blk74);
        let assign2410_e2851: f64 = (0.5 * assign2410_e2850);
        let assign2410_e2853: f64 = (assign2410_e2851 - locals.var_dv0__blk54);
        (assign2410_e2853, ((0.5 * (locals.var_dv__blk73_dn4 - locals.var_mv__blk74_dn4)) - locals.var_dv0__blk54_dn4), (0.5 * (locals.var_dv__blk73_dn6 - locals.var_mv__blk74_dn6)), (0.5 * (locals.var_dv__blk73_dn8 - locals.var_mv__blk74_dn8)),)
    } else {
        (locals.var_vl__blk65, locals.var_vl__blk65_dn4, locals.var_vl__blk65_dn6, locals.var_vl__blk65_dn8,)
    }
};
        locals.var_vl__blk65 = assign2410_e2855;
        locals.var_vl__blk65_dn4 = assign2410_e2855_d_n4;
        locals.var_vl__blk65_dn6 = assign2410_e2855_d_n6;
        locals.var_vl__blk65_dn8 = assign2410_e2855_d_n8;

        let (assign2420_e2878, assign2420_e2878_d_n4, assign2420_e2878_d_n6, assign2420_e2878_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2420_e2862: f64 = (-locals.var_pc_t);
        let assign2420_e2866: f64 = (locals.var_vl__blk65 / locals.var_pc_t);
        let assign2420_e2867: f64 = (1.0 - assign2420_e2866);
        let assign2420_e2870: f64 = (1.0 - p.p43);
        let assign2420_e2871: f64 = (assign2420_e2867).powf(assign2420_e2870);
        let assign2420_e2872: f64 = (assign2420_e2862 * assign2420_e2871);
        let assign2420_e2875: f64 = (1.0 - p.p43);
        let assign2420_e2876: f64 = (assign2420_e2872 / assign2420_e2875);
        (assign2420_e2876, ((((-locals.var_pc_t_dn4) * assign2420_e2871) + (assign2420_e2862 * if 0.0 == 0.0 && ((assign2420_e2870) as f64).is_finite() && ((assign2420_e2870) as f64).fract() == 0.0 { if assign2420_e2870 == 0.0 { 0.0 } else { (assign2420_e2870 * ((assign2420_e2867).powf(assign2420_e2870 - 1.0) * (-(((locals.var_vl__blk65_dn4 * locals.var_pc_t) - (locals.var_vl__blk65 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign2420_e2871 * (assign2420_e2870 * ((-(((locals.var_vl__blk65_dn4 * locals.var_pc_t) - (locals.var_vl__blk65 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign2420_e2867))) })) / assign2420_e2875), ((assign2420_e2862 * if 0.0 == 0.0 && ((assign2420_e2870) as f64).is_finite() && ((assign2420_e2870) as f64).fract() == 0.0 { if assign2420_e2870 == 0.0 { 0.0 } else { (assign2420_e2870 * ((assign2420_e2867).powf(assign2420_e2870 - 1.0) * (-(locals.var_vl__blk65_dn6 / locals.var_pc_t)))) } } else { (assign2420_e2871 * (assign2420_e2870 * ((-(locals.var_vl__blk65_dn6 / locals.var_pc_t)) / assign2420_e2867))) }) / assign2420_e2875), ((assign2420_e2862 * if 0.0 == 0.0 && ((assign2420_e2870) as f64).is_finite() && ((assign2420_e2870) as f64).fract() == 0.0 { if assign2420_e2870 == 0.0 { 0.0 } else { (assign2420_e2870 * ((assign2420_e2867).powf(assign2420_e2870 - 1.0) * (-(locals.var_vl__blk65_dn8 / locals.var_pc_t)))) } } else { (assign2420_e2871 * (assign2420_e2870 * ((-(locals.var_vl__blk65_dn8 / locals.var_pc_t)) / assign2420_e2867))) }) / assign2420_e2875),)
    } else {
        (locals.var_qlo__blk57, locals.var_qlo__blk57_dn4, locals.var_qlo__blk57_dn6, locals.var_qlo__blk57_dn8,)
    }
};
        locals.var_qlo__blk57 = assign2420_e2878;
        locals.var_qlo__blk57_dn4 = assign2420_e2878_d_n4;
        locals.var_qlo__blk57_dn6 = assign2420_e2878_d_n6;
        locals.var_qlo__blk57_dn8 = assign2420_e2878_d_n8;

        let (assign2430_e2901, assign2430_e2901_d_n4, assign2430_e2901_d_n6, assign2430_e2901_d_n8,) = {
    if ((locals.var_guard75 == 0.0) && (locals.var_guard78 == 0.0)) {
        let assign2430_e2887: f64 = (1.0 - p.p34);
        let assign2430_e2889: f64 = (-p.p43);
        let assign2430_e2890: f64 = (assign2430_e2887).powf(assign2430_e2889);
        let assign2430_e2893: f64 = (locals.var_vbci - locals.var_vl__blk65);
        let assign2430_e2895: f64 = (assign2430_e2893 + locals.var_vl0__blk61);
        let assign2430_e2896: f64 = (assign2430_e2890 * assign2430_e2895);
        let assign2430_e2897: f64 = (locals.var_qlo__blk57 + assign2430_e2896);
        let assign2430_e2899: f64 = (assign2430_e2897 - locals.var_q0__blk72);
        (assign2430_e2899, ((locals.var_qlo__blk57_dn4 + (assign2430_e2890 * ((-locals.var_vl__blk65_dn4) + locals.var_vl0__blk61_dn4))) - locals.var_q0__blk72_dn4), (locals.var_qlo__blk57_dn6 + (assign2430_e2890 * (locals.var_vbci_dn6 - locals.var_vl__blk65_dn6))), (locals.var_qlo__blk57_dn8 + (assign2430_e2890 * (locals.var_vbci_dn8 - locals.var_vl__blk65_dn8))),)
    } else {
        (locals.var_qdbc, locals.var_qdbc_dn4, locals.var_qdbc_dn6, locals.var_qdbc_dn8,)
    }
};
        locals.var_qdbc = assign2430_e2901;
        locals.var_qdbc_dn4 = assign2430_e2901_d_n4;
        locals.var_qdbc_dn6 = assign2430_e2901_d_n6;
        locals.var_qdbc_dn8 = assign2430_e2901_d_n8;

        let assign2440_e2905: f64 = (locals.var_nf_t * locals.var_vtv);
        let assign2440_e2906: f64 = (1.0 / assign2440_e2905);
        locals.var_afac = assign2440_e2906;
        locals.var_afac_dn4 = (-(((locals.var_nf_t_dn4 * locals.var_vtv) + (locals.var_nf_t * locals.var_vtv_dn4)) / (assign2440_e2905 * assign2440_e2905)));

        let assign2450_e2909: f64 = if locals.var_vbei < locals.var_maxvifi { 1.0 } else { 0.0 };
        locals.var_guard79 = assign2450_e2909;

        let (assign2460_e2916, assign2460_e2916_d_n4, assign2460_e2916_d_n5, assign2460_e2916_d_n6, assign2460_e2916_d_n7, assign2460_e2916_d_n8, assign2460_e2916_d_n9, assign2460_e2916_d_n10, assign2460_e2916_d_n11,) = {
    if (locals.var_guard79 != 0.0) {
        let assign2460_e2913: f64 = (locals.var_vbei * locals.var_afac);
        let assign2460_e2914: f64 = (assign2460_e2913).exp();
        (assign2460_e2914, (assign2460_e2914 * (locals.var_vbei * locals.var_afac_dn4)), 0.0, 0.0, 0.0, (assign2460_e2914 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2460_e2914 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2460_e2916;
        locals.var_expi_dn4 = assign2460_e2916_d_n4;
        locals.var_expi_dn5 = assign2460_e2916_d_n5;
        locals.var_expi_dn6 = assign2460_e2916_d_n6;
        locals.var_expi_dn7 = assign2460_e2916_d_n7;
        locals.var_expi_dn8 = assign2460_e2916_d_n8;
        locals.var_expi_dn9 = assign2460_e2916_d_n9;
        locals.var_expi_dn10 = assign2460_e2916_d_n10;
        locals.var_expi_dn11 = assign2460_e2916_d_n11;

        let (assign2470_e2932, assign2470_e2932_d_n4, assign2470_e2932_d_n5, assign2470_e2932_d_n6, assign2470_e2932_d_n7, assign2470_e2932_d_n8, assign2470_e2932_d_n9, assign2470_e2932_d_n10, assign2470_e2932_d_n11,) = {
    if (locals.var_guard79 == 0.0) {
        let assign2470_e2921: f64 = (locals.var_maxvifi * locals.var_afac);
        let assign2470_e2922: f64 = (assign2470_e2921).exp();
        let assign2470_e2926: f64 = (locals.var_vbei - locals.var_maxvifi);
        let assign2470_e2928: f64 = (assign2470_e2926 * locals.var_afac);
        let assign2470_e2929: f64 = (1.0 + assign2470_e2928);
        let assign2470_e2930: f64 = (assign2470_e2922 * assign2470_e2929);
        (assign2470_e2930, (((assign2470_e2922 * ((locals.var_maxvifi_dn4 * locals.var_afac) + (locals.var_maxvifi * locals.var_afac_dn4))) * assign2470_e2929) + (assign2470_e2922 * (((-locals.var_maxvifi_dn4) * locals.var_afac) + (assign2470_e2926 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign2470_e2922 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2470_e2922 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2470_e2932;
        locals.var_expi_dn4 = assign2470_e2932_d_n4;
        locals.var_expi_dn5 = assign2470_e2932_d_n5;
        locals.var_expi_dn6 = assign2470_e2932_d_n6;
        locals.var_expi_dn7 = assign2470_e2932_d_n7;
        locals.var_expi_dn8 = assign2470_e2932_d_n8;
        locals.var_expi_dn9 = assign2470_e2932_d_n9;
        locals.var_expi_dn10 = assign2470_e2932_d_n10;
        locals.var_expi_dn11 = assign2470_e2932_d_n11;

        let assign2480_e2936: f64 = (locals.var_expi - 1.0);
        let assign2480_e2937: f64 = (locals.var_is_t * assign2480_e2936);
        locals.var_ifi = assign2480_e2937;
        locals.var_ifi_dn4 = ((locals.var_is_t_dn4 * assign2480_e2936) + (locals.var_is_t * locals.var_expi_dn4));
        locals.var_ifi_dn5 = (locals.var_is_t * locals.var_expi_dn5);
        locals.var_ifi_dn6 = (locals.var_is_t * locals.var_expi_dn6);
        locals.var_ifi_dn7 = (locals.var_is_t * locals.var_expi_dn7);
        locals.var_ifi_dn8 = (locals.var_is_t * locals.var_expi_dn8);
        locals.var_ifi_dn9 = (locals.var_is_t * locals.var_expi_dn9);
        locals.var_ifi_dn10 = (locals.var_is_t * locals.var_expi_dn10);
        locals.var_ifi_dn11 = (locals.var_is_t * locals.var_expi_dn11);

        let assign2490_e2941: f64 = (locals.var_nr_t * locals.var_vtv);
        let assign2490_e2942: f64 = (1.0 / assign2490_e2941);
        locals.var_afac = assign2490_e2942;
        locals.var_afac_dn4 = (-(((locals.var_nr_t_dn4 * locals.var_vtv) + (locals.var_nr_t * locals.var_vtv_dn4)) / (assign2490_e2941 * assign2490_e2941)));

        let assign2500_e2945: f64 = if locals.var_vbci < locals.var_maxviri { 1.0 } else { 0.0 };
        locals.var_guard80 = assign2500_e2945;

        let (assign2510_e2952, assign2510_e2952_d_n4, assign2510_e2952_d_n5, assign2510_e2952_d_n6, assign2510_e2952_d_n7, assign2510_e2952_d_n8, assign2510_e2952_d_n9, assign2510_e2952_d_n10, assign2510_e2952_d_n11,) = {
    if (locals.var_guard80 != 0.0) {
        let assign2510_e2949: f64 = (locals.var_vbci * locals.var_afac);
        let assign2510_e2950: f64 = (assign2510_e2949).exp();
        (assign2510_e2950, (assign2510_e2950 * (locals.var_vbci * locals.var_afac_dn4)), 0.0, (assign2510_e2950 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign2510_e2950 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2510_e2952;
        locals.var_expi_dn4 = assign2510_e2952_d_n4;
        locals.var_expi_dn5 = assign2510_e2952_d_n5;
        locals.var_expi_dn6 = assign2510_e2952_d_n6;
        locals.var_expi_dn7 = assign2510_e2952_d_n7;
        locals.var_expi_dn8 = assign2510_e2952_d_n8;
        locals.var_expi_dn9 = assign2510_e2952_d_n9;
        locals.var_expi_dn10 = assign2510_e2952_d_n10;
        locals.var_expi_dn11 = assign2510_e2952_d_n11;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2520_e2968, assign2520_e2968_d_n4, assign2520_e2968_d_n5, assign2520_e2968_d_n6, assign2520_e2968_d_n7, assign2520_e2968_d_n8, assign2520_e2968_d_n9, assign2520_e2968_d_n10, assign2520_e2968_d_n11,) = {
    if (locals.var_guard80 == 0.0) {
        let assign2520_e2957: f64 = (locals.var_maxviri * locals.var_afac);
        let assign2520_e2958: f64 = (assign2520_e2957).exp();
        let assign2520_e2962: f64 = (locals.var_vbci - locals.var_maxviri);
        let assign2520_e2964: f64 = (assign2520_e2962 * locals.var_afac);
        let assign2520_e2965: f64 = (1.0 + assign2520_e2964);
        let assign2520_e2966: f64 = (assign2520_e2958 * assign2520_e2965);
        (assign2520_e2966, (((assign2520_e2958 * ((locals.var_maxviri_dn4 * locals.var_afac) + (locals.var_maxviri * locals.var_afac_dn4))) * assign2520_e2965) + (assign2520_e2958 * (((-locals.var_maxviri_dn4) * locals.var_afac) + (assign2520_e2962 * locals.var_afac_dn4)))), 0.0, (assign2520_e2958 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign2520_e2958 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2520_e2968;
        locals.var_expi_dn4 = assign2520_e2968_d_n4;
        locals.var_expi_dn5 = assign2520_e2968_d_n5;
        locals.var_expi_dn6 = assign2520_e2968_d_n6;
        locals.var_expi_dn7 = assign2520_e2968_d_n7;
        locals.var_expi_dn8 = assign2520_e2968_d_n8;
        locals.var_expi_dn9 = assign2520_e2968_d_n9;
        locals.var_expi_dn10 = assign2520_e2968_d_n10;
        locals.var_expi_dn11 = assign2520_e2968_d_n11;

        let assign2530_e2971: f64 = (locals.var_is_t * locals.var_isrr_t);
        let assign2530_e2974: f64 = (locals.var_expi - 1.0);
        let assign2530_e2975: f64 = (assign2530_e2971 * assign2530_e2974);
        locals.var_iri = assign2530_e2975;
        locals.var_iri_dn4 = ((((locals.var_is_t_dn4 * locals.var_isrr_t) + (locals.var_is_t * locals.var_isrr_t_dn4)) * assign2530_e2974) + (assign2530_e2971 * locals.var_expi_dn4));
        locals.var_iri_dn5 = (assign2530_e2971 * locals.var_expi_dn5);
        locals.var_iri_dn6 = (assign2530_e2971 * locals.var_expi_dn6);
        locals.var_iri_dn7 = (assign2530_e2971 * locals.var_expi_dn7);
        locals.var_iri_dn8 = (assign2530_e2971 * locals.var_expi_dn8);
        locals.var_iri_dn9 = (assign2530_e2971 * locals.var_expi_dn9);
        locals.var_iri_dn10 = (assign2530_e2971 * locals.var_expi_dn10);
        locals.var_iri_dn11 = (assign2530_e2971 * locals.var_expi_dn11);

        let assign2540_e2979: f64 = (locals.var_qdbe * locals.var_iver);
        let assign2540_e2980: f64 = (1.0 + assign2540_e2979);
        let assign2540_e2983: f64 = (locals.var_qdbc * locals.var_ivef);
        let assign2540_e2984: f64 = (assign2540_e2980 + assign2540_e2983);
        let assign2540_e2986: f64 = (assign2540_e2984 - 0.0001);
        locals.var_q1z = assign2540_e2986;
        locals.var_q1z_dn4 = (((locals.var_qdbe_dn4 * locals.var_iver) + (locals.var_qdbe * locals.var_iver_dn4)) + ((locals.var_qdbc_dn4 * locals.var_ivef) + (locals.var_qdbc * locals.var_ivef_dn4)));
        locals.var_q1z_dn6 = (locals.var_qdbc_dn6 * locals.var_ivef);
        locals.var_q1z_dn8 = ((locals.var_qdbe_dn8 * locals.var_iver) + (locals.var_qdbc_dn8 * locals.var_ivef));
        locals.var_q1z_dn9 = (locals.var_qdbe_dn9 * locals.var_iver);

        let assign2550_e2990: f64 = (locals.var_q1z * locals.var_q1z);
        let assign2550_e2992: f64 = (assign2550_e2990 + 1e-8);
        let assign2550_e2993: f64 = (assign2550_e2992).sqrt();
        let assign2550_e2995: f64 = (assign2550_e2993 + locals.var_q1z);
        let assign2550_e2996: f64 = (0.5 * assign2550_e2995);
        let assign2550_e2998: f64 = (assign2550_e2996 + 0.0001);
        locals.var_q1 = assign2550_e2998;
        locals.var_q1_dn4 = (0.5 * ((((locals.var_q1z_dn4 * locals.var_q1z) + (locals.var_q1z * locals.var_q1z_dn4)) / (2.0 * assign2550_e2993)) + locals.var_q1z_dn4));
        locals.var_q1_dn6 = (0.5 * ((((locals.var_q1z_dn6 * locals.var_q1z) + (locals.var_q1z * locals.var_q1z_dn6)) / (2.0 * assign2550_e2993)) + locals.var_q1z_dn6));
        locals.var_q1_dn8 = (0.5 * ((((locals.var_q1z_dn8 * locals.var_q1z) + (locals.var_q1z * locals.var_q1z_dn8)) / (2.0 * assign2550_e2993)) + locals.var_q1z_dn8));
        locals.var_q1_dn9 = (0.5 * ((((locals.var_q1z_dn9 * locals.var_q1z) + (locals.var_q1z * locals.var_q1z_dn9)) / (2.0 * assign2550_e2993)) + locals.var_q1z_dn9));

        let assign2560_e3001: f64 = (locals.var_ifi * locals.var_iikf);
        let assign2560_e3004: f64 = (locals.var_iri * locals.var_iikr);
        let assign2560_e3005: f64 = (assign2560_e3001 + assign2560_e3004);
        locals.var_q2 = assign2560_e3005;
        locals.var_q2_dn4 = (((locals.var_ifi_dn4 * locals.var_iikf) + (locals.var_ifi * locals.var_iikf_dn4)) + (locals.var_iri_dn4 * locals.var_iikr));
        locals.var_q2_dn5 = ((locals.var_ifi_dn5 * locals.var_iikf) + (locals.var_iri_dn5 * locals.var_iikr));
        locals.var_q2_dn6 = ((locals.var_ifi_dn6 * locals.var_iikf) + (locals.var_iri_dn6 * locals.var_iikr));
        locals.var_q2_dn7 = ((locals.var_ifi_dn7 * locals.var_iikf) + (locals.var_iri_dn7 * locals.var_iikr));
        locals.var_q2_dn8 = ((locals.var_ifi_dn8 * locals.var_iikf) + (locals.var_iri_dn8 * locals.var_iikr));
        locals.var_q2_dn9 = ((locals.var_ifi_dn9 * locals.var_iikf) + (locals.var_iri_dn9 * locals.var_iikr));
        locals.var_q2_dn10 = ((locals.var_ifi_dn10 * locals.var_iikf) + (locals.var_iri_dn10 * locals.var_iikr));
        locals.var_q2_dn11 = ((locals.var_ifi_dn11 * locals.var_iikf) + (locals.var_iri_dn11 * locals.var_iikr));

        let assign2570_e3008: f64 = if p.p30 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard81 = assign2570_e3008;

        let (assign2580_e3020, assign2580_e3020_d_n4, assign2580_e3020_d_n5, assign2580_e3020_d_n6, assign2580_e3020_d_n7, assign2580_e3020_d_n8, assign2580_e3020_d_n9, assign2580_e3020_d_n10, assign2580_e3020_d_n11,) = {
    if (locals.var_guard81 != 0.0) {
        let assign2580_e3013: f64 = (1.0 / p.p73);
        let assign2580_e3014: f64 = (locals.var_q1).powf(assign2580_e3013);
        let assign2580_e3017: f64 = (4.0 * locals.var_q2);
        let assign2580_e3018: f64 = (assign2580_e3014 + assign2580_e3017);
        (assign2580_e3018, (if 0.0 == 0.0 && ((assign2580_e3013) as f64).is_finite() && ((assign2580_e3013) as f64).fract() == 0.0 { if assign2580_e3013 == 0.0 { 0.0 } else { (assign2580_e3013 * ((locals.var_q1).powf(assign2580_e3013 - 1.0) * locals.var_q1_dn4)) } } else { (assign2580_e3014 * (assign2580_e3013 * (locals.var_q1_dn4 / locals.var_q1))) } + (4.0 * locals.var_q2_dn4)), (4.0 * locals.var_q2_dn5), (if 0.0 == 0.0 && ((assign2580_e3013) as f64).is_finite() && ((assign2580_e3013) as f64).fract() == 0.0 { if assign2580_e3013 == 0.0 { 0.0 } else { (assign2580_e3013 * ((locals.var_q1).powf(assign2580_e3013 - 1.0) * locals.var_q1_dn6)) } } else { (assign2580_e3014 * (assign2580_e3013 * (locals.var_q1_dn6 / locals.var_q1))) } + (4.0 * locals.var_q2_dn6)), (4.0 * locals.var_q2_dn7), (if 0.0 == 0.0 && ((assign2580_e3013) as f64).is_finite() && ((assign2580_e3013) as f64).fract() == 0.0 { if assign2580_e3013 == 0.0 { 0.0 } else { (assign2580_e3013 * ((locals.var_q1).powf(assign2580_e3013 - 1.0) * locals.var_q1_dn8)) } } else { (assign2580_e3014 * (assign2580_e3013 * (locals.var_q1_dn8 / locals.var_q1))) } + (4.0 * locals.var_q2_dn8)), (if 0.0 == 0.0 && ((assign2580_e3013) as f64).is_finite() && ((assign2580_e3013) as f64).fract() == 0.0 { if assign2580_e3013 == 0.0 { 0.0 } else { (assign2580_e3013 * ((locals.var_q1).powf(assign2580_e3013 - 1.0) * locals.var_q1_dn9)) } } else { (assign2580_e3014 * (assign2580_e3013 * (locals.var_q1_dn9 / locals.var_q1))) } + (4.0 * locals.var_q2_dn9)), (4.0 * locals.var_q2_dn10), (4.0 * locals.var_q2_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign2580_e3020;
        locals.var_arg_dn4 = assign2580_e3020_d_n4;
        locals.var_arg_dn5 = assign2580_e3020_d_n5;
        locals.var_arg_dn6 = assign2580_e3020_d_n6;
        locals.var_arg_dn7 = assign2580_e3020_d_n7;
        locals.var_arg_dn8 = assign2580_e3020_d_n8;
        locals.var_arg_dn9 = assign2580_e3020_d_n9;
        locals.var_arg_dn10 = assign2580_e3020_d_n10;
        locals.var_arg_dn11 = assign2580_e3020_d_n11;

        let assign2590_e3023: f64 = if locals.var_arg > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard82 = assign2590_e3023;

        let (assign2600_e3035, assign2600_e3035_d_n4, assign2600_e3035_d_n5, assign2600_e3035_d_n6, assign2600_e3035_d_n7, assign2600_e3035_d_n8, assign2600_e3035_d_n9, assign2600_e3035_d_n10, assign2600_e3035_d_n11,) = {
    if ((locals.var_guard81 != 0.0) && (locals.var_guard82 != 0.0)) {
        let assign2600_e3031: f64 = (locals.var_arg).powf(p.p73);
        let assign2600_e3032: f64 = (locals.var_q1 + assign2600_e3031);
        let assign2600_e3033: f64 = (0.5 * assign2600_e3032);
        (assign2600_e3033, (0.5 * (locals.var_q1_dn4 + if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn4)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn4 / locals.var_arg))) })), (0.5 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn5)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn5 / locals.var_arg))) }), (0.5 * (locals.var_q1_dn6 + if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn6)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn6 / locals.var_arg))) })), (0.5 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn7)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn7 / locals.var_arg))) }), (0.5 * (locals.var_q1_dn8 + if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn8)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn8 / locals.var_arg))) })), (0.5 * (locals.var_q1_dn9 + if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn9)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn9 / locals.var_arg))) })), (0.5 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn10)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn10 / locals.var_arg))) }), (0.5 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn11)) } } else { (assign2600_e3031 * (p.p73 * (locals.var_arg_dn11 / locals.var_arg))) }),)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign2600_e3035;
        locals.var_qb_dn4 = assign2600_e3035_d_n4;
        locals.var_qb_dn5 = assign2600_e3035_d_n5;
        locals.var_qb_dn6 = assign2600_e3035_d_n6;
        locals.var_qb_dn7 = assign2600_e3035_d_n7;
        locals.var_qb_dn8 = assign2600_e3035_d_n8;
        locals.var_qb_dn9 = assign2600_e3035_d_n9;
        locals.var_qb_dn10 = assign2600_e3035_d_n10;
        locals.var_qb_dn11 = assign2600_e3035_d_n11;

        let (assign2610_e3048, assign2610_e3048_d_n4, assign2610_e3048_d_n5, assign2610_e3048_d_n6, assign2610_e3048_d_n7, assign2610_e3048_d_n8, assign2610_e3048_d_n9, assign2610_e3048_d_n10, assign2610_e3048_d_n11,) = {
    if ((locals.var_guard81 != 0.0) && (locals.var_guard82 == 0.0)) {
        let assign2610_e3044: f64 = (1e-8_f64).powf(p.p73);
        let assign2610_e3045: f64 = (locals.var_q1 + assign2610_e3044);
        let assign2610_e3046: f64 = (0.5 * assign2610_e3045);
        (assign2610_e3046, (0.5 * locals.var_q1_dn4), 0.0, (0.5 * locals.var_q1_dn6), 0.0, (0.5 * locals.var_q1_dn8), (0.5 * locals.var_q1_dn9), 0.0, 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign2610_e3048;
        locals.var_qb_dn4 = assign2610_e3048_d_n4;
        locals.var_qb_dn5 = assign2610_e3048_d_n5;
        locals.var_qb_dn6 = assign2610_e3048_d_n6;
        locals.var_qb_dn7 = assign2610_e3048_d_n7;
        locals.var_qb_dn8 = assign2610_e3048_d_n8;
        locals.var_qb_dn9 = assign2610_e3048_d_n9;
        locals.var_qb_dn10 = assign2610_e3048_d_n10;
        locals.var_qb_dn11 = assign2610_e3048_d_n11;

        let (assign2620_e3057, assign2620_e3057_d_n4, assign2620_e3057_d_n5, assign2620_e3057_d_n6, assign2620_e3057_d_n7, assign2620_e3057_d_n8, assign2620_e3057_d_n9, assign2620_e3057_d_n10, assign2620_e3057_d_n11,) = {
    if (locals.var_guard81 == 0.0) {
        let assign2620_e3054: f64 = (4.0 * locals.var_q2);
        let assign2620_e3055: f64 = (1.0 + assign2620_e3054);
        (assign2620_e3055, (4.0 * locals.var_q2_dn4), (4.0 * locals.var_q2_dn5), (4.0 * locals.var_q2_dn6), (4.0 * locals.var_q2_dn7), (4.0 * locals.var_q2_dn8), (4.0 * locals.var_q2_dn9), (4.0 * locals.var_q2_dn10), (4.0 * locals.var_q2_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign2620_e3057;
        locals.var_arg_dn4 = assign2620_e3057_d_n4;
        locals.var_arg_dn5 = assign2620_e3057_d_n5;
        locals.var_arg_dn6 = assign2620_e3057_d_n6;
        locals.var_arg_dn7 = assign2620_e3057_d_n7;
        locals.var_arg_dn8 = assign2620_e3057_d_n8;
        locals.var_arg_dn9 = assign2620_e3057_d_n9;
        locals.var_arg_dn10 = assign2620_e3057_d_n10;
        locals.var_arg_dn11 = assign2620_e3057_d_n11;

        let assign2630_e3060: f64 = if locals.var_arg > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign2630_e3060;

        let (assign2640_e3075, assign2640_e3075_d_n4, assign2640_e3075_d_n5, assign2640_e3075_d_n6, assign2640_e3075_d_n7, assign2640_e3075_d_n8, assign2640_e3075_d_n9, assign2640_e3075_d_n10, assign2640_e3075_d_n11,) = {
    if ((locals.var_guard81 == 0.0) && (locals.var_guard83 != 0.0)) {
        let assign2640_e3067: f64 = (0.5 * locals.var_q1);
        let assign2640_e3071: f64 = (locals.var_arg).powf(p.p73);
        let assign2640_e3072: f64 = (1.0 + assign2640_e3071);
        let assign2640_e3073: f64 = (assign2640_e3067 * assign2640_e3072);
        (assign2640_e3073, (((0.5 * locals.var_q1_dn4) * assign2640_e3072) + (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn4)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn4 / locals.var_arg))) })), (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn5)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn5 / locals.var_arg))) }), (((0.5 * locals.var_q1_dn6) * assign2640_e3072) + (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn6)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn6 / locals.var_arg))) })), (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn7)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn7 / locals.var_arg))) }), (((0.5 * locals.var_q1_dn8) * assign2640_e3072) + (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn8)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn8 / locals.var_arg))) })), (((0.5 * locals.var_q1_dn9) * assign2640_e3072) + (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn9)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn9 / locals.var_arg))) })), (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn10)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn10 / locals.var_arg))) }), (assign2640_e3067 * if 0.0 == 0.0 && ((p.p73) as f64).is_finite() && ((p.p73) as f64).fract() == 0.0 { if p.p73 == 0.0 { 0.0 } else { (p.p73 * ((locals.var_arg).powf(p.p73 - 1.0) * locals.var_arg_dn11)) } } else { (assign2640_e3071 * (p.p73 * (locals.var_arg_dn11 / locals.var_arg))) }),)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign2640_e3075;
        locals.var_qb_dn4 = assign2640_e3075_d_n4;
        locals.var_qb_dn5 = assign2640_e3075_d_n5;
        locals.var_qb_dn6 = assign2640_e3075_d_n6;
        locals.var_qb_dn7 = assign2640_e3075_d_n7;
        locals.var_qb_dn8 = assign2640_e3075_d_n8;
        locals.var_qb_dn9 = assign2640_e3075_d_n9;
        locals.var_qb_dn10 = assign2640_e3075_d_n10;
        locals.var_qb_dn11 = assign2640_e3075_d_n11;

        let (assign2650_e3091, assign2650_e3091_d_n4, assign2650_e3091_d_n5, assign2650_e3091_d_n6, assign2650_e3091_d_n7, assign2650_e3091_d_n8, assign2650_e3091_d_n9, assign2650_e3091_d_n10, assign2650_e3091_d_n11,) = {
    if ((locals.var_guard81 == 0.0) && (locals.var_guard83 == 0.0)) {
        let assign2650_e3083: f64 = (0.5 * locals.var_q1);
        let assign2650_e3087: f64 = (1e-8_f64).powf(p.p73);
        let assign2650_e3088: f64 = (1.0 + assign2650_e3087);
        let assign2650_e3089: f64 = (assign2650_e3083 * assign2650_e3088);
        (assign2650_e3089, ((0.5 * locals.var_q1_dn4) * assign2650_e3088), 0.0, ((0.5 * locals.var_q1_dn6) * assign2650_e3088), 0.0, ((0.5 * locals.var_q1_dn8) * assign2650_e3088), ((0.5 * locals.var_q1_dn9) * assign2650_e3088), 0.0, 0.0,)
    } else {
        (locals.var_qb, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11,)
    }
};
        locals.var_qb = assign2650_e3091;
        locals.var_qb_dn4 = assign2650_e3091_d_n4;
        locals.var_qb_dn5 = assign2650_e3091_d_n5;
        locals.var_qb_dn6 = assign2650_e3091_d_n6;
        locals.var_qb_dn7 = assign2650_e3091_d_n7;
        locals.var_qb_dn8 = assign2650_e3091_d_n8;
        locals.var_qb_dn9 = assign2650_e3091_d_n9;
        locals.var_qb_dn10 = assign2650_e3091_d_n10;
        locals.var_qb_dn11 = assign2650_e3091_d_n11;

        let assign2660_e3094: f64 = (locals.var_iri / locals.var_qb);
        locals.var_itzr = assign2660_e3094;
        locals.var_itzr_dn4 = (((locals.var_iri_dn4 * locals.var_qb) - (locals.var_iri * locals.var_qb_dn4)) / (locals.var_qb * locals.var_qb));
        locals.var_itzr_dn5 = (((locals.var_iri_dn5 * locals.var_qb) - (locals.var_iri * locals.var_qb_dn5)) / (locals.var_qb * locals.var_qb));
        locals.var_itzr_dn6 = (((locals.var_iri_dn6 * locals.var_qb) - (locals.var_iri * locals.var_qb_dn6)) / (locals.var_qb * locals.var_qb));
        locals.var_itzr_dn7 = (((locals.var_iri_dn7 * locals.var_qb) - (locals.var_iri * locals.var_qb_dn7)) / (locals.var_qb * locals.var_qb));
        locals.var_itzr_dn8 = (((locals.var_iri_dn8 * locals.var_qb) - (locals.var_iri * locals.var_qb_dn8)) / (locals.var_qb * locals.var_qb));
        locals.var_itzr_dn9 = (((locals.var_iri_dn9 * locals.var_qb) - (locals.var_iri * locals.var_qb_dn9)) / (locals.var_qb * locals.var_qb));
        locals.var_itzr_dn10 = (((locals.var_iri_dn10 * locals.var_qb) - (locals.var_iri * locals.var_qb_dn10)) / (locals.var_qb * locals.var_qb));
        locals.var_itzr_dn11 = (((locals.var_iri_dn11 * locals.var_qb) - (locals.var_iri * locals.var_qb_dn11)) / (locals.var_qb * locals.var_qb));

        let assign2670_e3097: f64 = (locals.var_ifi / locals.var_qb);
        locals.var_itzf = assign2670_e3097;
        locals.var_itzf_dn4 = (((locals.var_ifi_dn4 * locals.var_qb) - (locals.var_ifi * locals.var_qb_dn4)) / (locals.var_qb * locals.var_qb));
        locals.var_itzf_dn5 = (((locals.var_ifi_dn5 * locals.var_qb) - (locals.var_ifi * locals.var_qb_dn5)) / (locals.var_qb * locals.var_qb));
        locals.var_itzf_dn6 = (((locals.var_ifi_dn6 * locals.var_qb) - (locals.var_ifi * locals.var_qb_dn6)) / (locals.var_qb * locals.var_qb));
        locals.var_itzf_dn7 = (((locals.var_ifi_dn7 * locals.var_qb) - (locals.var_ifi * locals.var_qb_dn7)) / (locals.var_qb * locals.var_qb));
        locals.var_itzf_dn8 = (((locals.var_ifi_dn8 * locals.var_qb) - (locals.var_ifi * locals.var_qb_dn8)) / (locals.var_qb * locals.var_qb));
        locals.var_itzf_dn9 = (((locals.var_ifi_dn9 * locals.var_qb) - (locals.var_ifi * locals.var_qb_dn9)) / (locals.var_qb * locals.var_qb));
        locals.var_itzf_dn10 = (((locals.var_ifi_dn10 * locals.var_qb) - (locals.var_ifi * locals.var_qb_dn10)) / (locals.var_qb * locals.var_qb));
        locals.var_itzf_dn11 = (((locals.var_ifi_dn11 * locals.var_qb) - (locals.var_ifi * locals.var_qb_dn11)) / (locals.var_qb * locals.var_qb));

        locals.var_itxf = locals.var_vxf2;
        locals.var_itxf_dn13 = locals.var_vxf2_dn13;

        let assign2690_e3101: f64 = if p.p31 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign2690_e3101;

        let (assign2700_e3109, assign2700_e3109_d_n4,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2700_e3106: f64 = (p.p33 * locals.var_vtv);
        let assign2700_e3107: f64 = (1.0 / assign2700_e3106);
        (assign2700_e3107, (-((p.p33 * locals.var_vtv_dn4) / (assign2700_e3106 * assign2700_e3106))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign2700_e3109;
        locals.var_afac_dn4 = assign2700_e3109_d_n4;

        let assign2710_e3112: f64 = if locals.var_vbep < locals.var_maxvip { 1.0 } else { 0.0 };
        locals.var_guard85 = assign2710_e3112;

        let (assign2720_e3121, assign2720_e3121_d_n4, assign2720_e3121_d_n5, assign2720_e3121_d_n6, assign2720_e3121_d_n7, assign2720_e3121_d_n8, assign2720_e3121_d_n9, assign2720_e3121_d_n10, assign2720_e3121_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign2720_e3118: f64 = (locals.var_vbep * locals.var_afac);
        let assign2720_e3119: f64 = (assign2720_e3118).exp();
        (assign2720_e3119, (assign2720_e3119 * (locals.var_vbep * locals.var_afac_dn4)), 0.0, 0.0, (assign2720_e3119 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign2720_e3119 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2720_e3121;
        locals.var_expi_dn4 = assign2720_e3121_d_n4;
        locals.var_expi_dn5 = assign2720_e3121_d_n5;
        locals.var_expi_dn6 = assign2720_e3121_d_n6;
        locals.var_expi_dn7 = assign2720_e3121_d_n7;
        locals.var_expi_dn8 = assign2720_e3121_d_n8;
        locals.var_expi_dn9 = assign2720_e3121_d_n9;
        locals.var_expi_dn10 = assign2720_e3121_d_n10;
        locals.var_expi_dn11 = assign2720_e3121_d_n11;

        let (assign2730_e3139, assign2730_e3139_d_n4, assign2730_e3139_d_n5, assign2730_e3139_d_n6, assign2730_e3139_d_n7, assign2730_e3139_d_n8, assign2730_e3139_d_n9, assign2730_e3139_d_n10, assign2730_e3139_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard85 == 0.0)) {
        let assign2730_e3128: f64 = (locals.var_maxvip * locals.var_afac);
        let assign2730_e3129: f64 = (assign2730_e3128).exp();
        let assign2730_e3133: f64 = (locals.var_vbep - locals.var_maxvip);
        let assign2730_e3135: f64 = (assign2730_e3133 * locals.var_afac);
        let assign2730_e3136: f64 = (1.0 + assign2730_e3135);
        let assign2730_e3137: f64 = (assign2730_e3129 * assign2730_e3136);
        (assign2730_e3137, (((assign2730_e3129 * ((locals.var_maxvip_dn4 * locals.var_afac) + (locals.var_maxvip * locals.var_afac_dn4))) * assign2730_e3136) + (assign2730_e3129 * (((-locals.var_maxvip_dn4) * locals.var_afac) + (assign2730_e3133 * locals.var_afac_dn4)))), 0.0, 0.0, (assign2730_e3129 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign2730_e3129 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2730_e3139;
        locals.var_expi_dn4 = assign2730_e3139_d_n4;
        locals.var_expi_dn5 = assign2730_e3139_d_n5;
        locals.var_expi_dn6 = assign2730_e3139_d_n6;
        locals.var_expi_dn7 = assign2730_e3139_d_n7;
        locals.var_expi_dn8 = assign2730_e3139_d_n8;
        locals.var_expi_dn9 = assign2730_e3139_d_n9;
        locals.var_expi_dn10 = assign2730_e3139_d_n10;
        locals.var_expi_dn11 = assign2730_e3139_d_n11;

        let assign2740_e3142: f64 = if locals.var_vbci < locals.var_maxvip { 1.0 } else { 0.0 };
        locals.var_guard86 = assign2740_e3142;

        let (assign2750_e3151, assign2750_e3151_d_n4, assign2750_e3151_d_n5, assign2750_e3151_d_n6, assign2750_e3151_d_n7, assign2750_e3151_d_n8, assign2750_e3151_d_n9, assign2750_e3151_d_n10, assign2750_e3151_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign2750_e3148: f64 = (locals.var_vbci * locals.var_afac);
        let assign2750_e3149: f64 = (assign2750_e3148).exp();
        (assign2750_e3149, (assign2750_e3149 * (locals.var_vbci * locals.var_afac_dn4)), 0.0, (assign2750_e3149 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign2750_e3149 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign2750_e3151;
        locals.var_expx_dn4 = assign2750_e3151_d_n4;
        locals.var_expx_dn5 = assign2750_e3151_d_n5;
        locals.var_expx_dn6 = assign2750_e3151_d_n6;
        locals.var_expx_dn7 = assign2750_e3151_d_n7;
        locals.var_expx_dn8 = assign2750_e3151_d_n8;
        locals.var_expx_dn9 = assign2750_e3151_d_n9;
        locals.var_expx_dn10 = assign2750_e3151_d_n10;
        locals.var_expx_dn11 = assign2750_e3151_d_n11;

        let (assign2760_e3169, assign2760_e3169_d_n4, assign2760_e3169_d_n5, assign2760_e3169_d_n6, assign2760_e3169_d_n7, assign2760_e3169_d_n8, assign2760_e3169_d_n9, assign2760_e3169_d_n10, assign2760_e3169_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard86 == 0.0)) {
        let assign2760_e3158: f64 = (locals.var_maxvip * locals.var_afac);
        let assign2760_e3159: f64 = (assign2760_e3158).exp();
        let assign2760_e3163: f64 = (locals.var_vbci - locals.var_maxvip);
        let assign2760_e3165: f64 = (assign2760_e3163 * locals.var_afac);
        let assign2760_e3166: f64 = (1.0 + assign2760_e3165);
        let assign2760_e3167: f64 = (assign2760_e3159 * assign2760_e3166);
        (assign2760_e3167, (((assign2760_e3159 * ((locals.var_maxvip_dn4 * locals.var_afac) + (locals.var_maxvip * locals.var_afac_dn4))) * assign2760_e3166) + (assign2760_e3159 * (((-locals.var_maxvip_dn4) * locals.var_afac) + (assign2760_e3163 * locals.var_afac_dn4)))), 0.0, (assign2760_e3159 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign2760_e3159 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign2760_e3169;
        locals.var_expx_dn4 = assign2760_e3169_d_n4;
        locals.var_expx_dn5 = assign2760_e3169_d_n5;
        locals.var_expx_dn6 = assign2760_e3169_d_n6;
        locals.var_expx_dn7 = assign2760_e3169_d_n7;
        locals.var_expx_dn8 = assign2760_e3169_d_n8;
        locals.var_expx_dn9 = assign2760_e3169_d_n9;
        locals.var_expx_dn10 = assign2760_e3169_d_n10;
        locals.var_expx_dn11 = assign2760_e3169_d_n11;

        let (assign2770_e3185, assign2770_e3185_d_n4, assign2770_e3185_d_n5, assign2770_e3185_d_n6, assign2770_e3185_d_n7, assign2770_e3185_d_n8, assign2770_e3185_d_n9, assign2770_e3185_d_n10, assign2770_e3185_d_n11,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2770_e3174: f64 = (p.p32 * locals.var_expi);
        let assign2770_e3177: f64 = (1.0 - p.p32);
        let assign2770_e3179: f64 = (assign2770_e3177 * locals.var_expx);
        let assign2770_e3180: f64 = (assign2770_e3174 + assign2770_e3179);
        let assign2770_e3182: f64 = (assign2770_e3180 - 1.0);
        let assign2770_e3183: f64 = (locals.var_isp_t * assign2770_e3182);
        (assign2770_e3183, ((locals.var_isp_t_dn4 * assign2770_e3182) + (locals.var_isp_t * ((p.p32 * locals.var_expi_dn4) + (assign2770_e3177 * locals.var_expx_dn4)))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn5) + (assign2770_e3177 * locals.var_expx_dn5))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn6) + (assign2770_e3177 * locals.var_expx_dn6))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn7) + (assign2770_e3177 * locals.var_expx_dn7))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn8) + (assign2770_e3177 * locals.var_expx_dn8))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn9) + (assign2770_e3177 * locals.var_expx_dn9))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn10) + (assign2770_e3177 * locals.var_expx_dn10))), (locals.var_isp_t * ((p.p32 * locals.var_expi_dn11) + (assign2770_e3177 * locals.var_expx_dn11))),)
    } else {
        (locals.var_ifp, locals.var_ifp_dn4, locals.var_ifp_dn5, locals.var_ifp_dn6, locals.var_ifp_dn7, locals.var_ifp_dn8, locals.var_ifp_dn9, locals.var_ifp_dn10, locals.var_ifp_dn11,)
    }
};
        locals.var_ifp = assign2770_e3185;
        locals.var_ifp_dn4 = assign2770_e3185_d_n4;
        locals.var_ifp_dn5 = assign2770_e3185_d_n5;
        locals.var_ifp_dn6 = assign2770_e3185_d_n6;
        locals.var_ifp_dn7 = assign2770_e3185_d_n7;
        locals.var_ifp_dn8 = assign2770_e3185_d_n8;
        locals.var_ifp_dn9 = assign2770_e3185_d_n9;
        locals.var_ifp_dn10 = assign2770_e3185_d_n10;
        locals.var_ifp_dn11 = assign2770_e3185_d_n11;

        let (assign2780_e3191, assign2780_e3191_d_n4, assign2780_e3191_d_n5, assign2780_e3191_d_n6, assign2780_e3191_d_n7, assign2780_e3191_d_n8, assign2780_e3191_d_n9, assign2780_e3191_d_n10, assign2780_e3191_d_n11,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2780_e3189: f64 = (locals.var_ifp * locals.var_iikp);
        (assign2780_e3189, (locals.var_ifp_dn4 * locals.var_iikp), (locals.var_ifp_dn5 * locals.var_iikp), (locals.var_ifp_dn6 * locals.var_iikp), (locals.var_ifp_dn7 * locals.var_iikp), (locals.var_ifp_dn8 * locals.var_iikp), (locals.var_ifp_dn9 * locals.var_iikp), (locals.var_ifp_dn10 * locals.var_iikp), (locals.var_ifp_dn11 * locals.var_iikp),)
    } else {
        (locals.var_q2p, locals.var_q2p_dn4, locals.var_q2p_dn5, locals.var_q2p_dn6, locals.var_q2p_dn7, locals.var_q2p_dn8, locals.var_q2p_dn9, locals.var_q2p_dn10, locals.var_q2p_dn11,)
    }
};
        locals.var_q2p = assign2780_e3191;
        locals.var_q2p_dn4 = assign2780_e3191_d_n4;
        locals.var_q2p_dn5 = assign2780_e3191_d_n5;
        locals.var_q2p_dn6 = assign2780_e3191_d_n6;
        locals.var_q2p_dn7 = assign2780_e3191_d_n7;
        locals.var_q2p_dn8 = assign2780_e3191_d_n8;
        locals.var_q2p_dn9 = assign2780_e3191_d_n9;
        locals.var_q2p_dn10 = assign2780_e3191_d_n10;
        locals.var_q2p_dn11 = assign2780_e3191_d_n11;

        let (assign2790_e3199, assign2790_e3199_d_n4, assign2790_e3199_d_n5, assign2790_e3199_d_n6, assign2790_e3199_d_n7, assign2790_e3199_d_n8, assign2790_e3199_d_n9, assign2790_e3199_d_n10, assign2790_e3199_d_n11,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2790_e3196: f64 = (4.0 * locals.var_q2p);
        let assign2790_e3197: f64 = (1.0 + assign2790_e3196);
        (assign2790_e3197, (4.0 * locals.var_q2p_dn4), (4.0 * locals.var_q2p_dn5), (4.0 * locals.var_q2p_dn6), (4.0 * locals.var_q2p_dn7), (4.0 * locals.var_q2p_dn8), (4.0 * locals.var_q2p_dn9), (4.0 * locals.var_q2p_dn10), (4.0 * locals.var_q2p_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign2790_e3199;
        locals.var_arg_dn4 = assign2790_e3199_d_n4;
        locals.var_arg_dn5 = assign2790_e3199_d_n5;
        locals.var_arg_dn6 = assign2790_e3199_d_n6;
        locals.var_arg_dn7 = assign2790_e3199_d_n7;
        locals.var_arg_dn8 = assign2790_e3199_d_n8;
        locals.var_arg_dn9 = assign2790_e3199_d_n9;
        locals.var_arg_dn10 = assign2790_e3199_d_n10;
        locals.var_arg_dn11 = assign2790_e3199_d_n11;

        let assign2800_e3202: f64 = if locals.var_arg > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign2800_e3202;

        let (assign2810_e3213, assign2810_e3213_d_n4, assign2810_e3213_d_n5, assign2810_e3213_d_n6, assign2810_e3213_d_n7, assign2810_e3213_d_n8, assign2810_e3213_d_n9, assign2810_e3213_d_n10, assign2810_e3213_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign2810_e3209: f64 = (locals.var_arg).sqrt();
        let assign2810_e3210: f64 = (1.0 + assign2810_e3209);
        let assign2810_e3211: f64 = (0.5 * assign2810_e3210);
        (assign2810_e3211, (0.5 * (locals.var_arg_dn4 / (2.0 * assign2810_e3209))), (0.5 * (locals.var_arg_dn5 / (2.0 * assign2810_e3209))), (0.5 * (locals.var_arg_dn6 / (2.0 * assign2810_e3209))), (0.5 * (locals.var_arg_dn7 / (2.0 * assign2810_e3209))), (0.5 * (locals.var_arg_dn8 / (2.0 * assign2810_e3209))), (0.5 * (locals.var_arg_dn9 / (2.0 * assign2810_e3209))), (0.5 * (locals.var_arg_dn10 / (2.0 * assign2810_e3209))), (0.5 * (locals.var_arg_dn11 / (2.0 * assign2810_e3209))),)
    } else {
        (locals.var_qbp, locals.var_qbp_dn4, locals.var_qbp_dn5, locals.var_qbp_dn6, locals.var_qbp_dn7, locals.var_qbp_dn8, locals.var_qbp_dn9, locals.var_qbp_dn10, locals.var_qbp_dn11,)
    }
};
        locals.var_qbp = assign2810_e3213;
        locals.var_qbp_dn4 = assign2810_e3213_d_n4;
        locals.var_qbp_dn5 = assign2810_e3213_d_n5;
        locals.var_qbp_dn6 = assign2810_e3213_d_n6;
        locals.var_qbp_dn7 = assign2810_e3213_d_n7;
        locals.var_qbp_dn8 = assign2810_e3213_d_n8;
        locals.var_qbp_dn9 = assign2810_e3213_d_n9;
        locals.var_qbp_dn10 = assign2810_e3213_d_n10;
        locals.var_qbp_dn11 = assign2810_e3213_d_n11;

        let (assign2820_e3225, assign2820_e3225_d_n4, assign2820_e3225_d_n5, assign2820_e3225_d_n6, assign2820_e3225_d_n7, assign2820_e3225_d_n8, assign2820_e3225_d_n9, assign2820_e3225_d_n10, assign2820_e3225_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard87 == 0.0)) {
        let assign2820_e3221: f64 = (1e-8_f64).sqrt();
        let assign2820_e3222: f64 = (1.0 + assign2820_e3221);
        let assign2820_e3223: f64 = (0.5 * assign2820_e3222);
        (assign2820_e3223, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbp, locals.var_qbp_dn4, locals.var_qbp_dn5, locals.var_qbp_dn6, locals.var_qbp_dn7, locals.var_qbp_dn8, locals.var_qbp_dn9, locals.var_qbp_dn10, locals.var_qbp_dn11,)
    }
};
        locals.var_qbp = assign2820_e3225;
        locals.var_qbp_dn4 = assign2820_e3225_d_n4;
        locals.var_qbp_dn5 = assign2820_e3225_d_n5;
        locals.var_qbp_dn6 = assign2820_e3225_d_n6;
        locals.var_qbp_dn7 = assign2820_e3225_d_n7;
        locals.var_qbp_dn8 = assign2820_e3225_d_n8;
        locals.var_qbp_dn9 = assign2820_e3225_d_n9;
        locals.var_qbp_dn10 = assign2820_e3225_d_n10;
        locals.var_qbp_dn11 = assign2820_e3225_d_n11;

        let assign2830_e3228: f64 = if locals.var_vbcp < locals.var_maxvip { 1.0 } else { 0.0 };
        locals.var_guard88 = assign2830_e3228;

        let (assign2840_e3237, assign2840_e3237_d_n4, assign2840_e3237_d_n5, assign2840_e3237_d_n6, assign2840_e3237_d_n7, assign2840_e3237_d_n8, assign2840_e3237_d_n9, assign2840_e3237_d_n10, assign2840_e3237_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard88 != 0.0)) {
        let assign2840_e3234: f64 = (locals.var_vbcp * locals.var_afac);
        let assign2840_e3235: f64 = (assign2840_e3234).exp();
        (assign2840_e3235, (assign2840_e3235 * (locals.var_vbcp * locals.var_afac_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0, (assign2840_e3235 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign2840_e3235 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2840_e3237;
        locals.var_expi_dn4 = assign2840_e3237_d_n4;
        locals.var_expi_dn5 = assign2840_e3237_d_n5;
        locals.var_expi_dn6 = assign2840_e3237_d_n6;
        locals.var_expi_dn7 = assign2840_e3237_d_n7;
        locals.var_expi_dn8 = assign2840_e3237_d_n8;
        locals.var_expi_dn9 = assign2840_e3237_d_n9;
        locals.var_expi_dn10 = assign2840_e3237_d_n10;
        locals.var_expi_dn11 = assign2840_e3237_d_n11;

        let (assign2850_e3255, assign2850_e3255_d_n4, assign2850_e3255_d_n5, assign2850_e3255_d_n6, assign2850_e3255_d_n7, assign2850_e3255_d_n8, assign2850_e3255_d_n9, assign2850_e3255_d_n10, assign2850_e3255_d_n11,) = {
    if ((locals.var_guard84 != 0.0) && (locals.var_guard88 == 0.0)) {
        let assign2850_e3244: f64 = (locals.var_maxvip * locals.var_afac);
        let assign2850_e3245: f64 = (assign2850_e3244).exp();
        let assign2850_e3249: f64 = (locals.var_vbcp - locals.var_maxvip);
        let assign2850_e3251: f64 = (assign2850_e3249 * locals.var_afac);
        let assign2850_e3252: f64 = (1.0 + assign2850_e3251);
        let assign2850_e3253: f64 = (assign2850_e3245 * assign2850_e3252);
        (assign2850_e3253, (((assign2850_e3245 * ((locals.var_maxvip_dn4 * locals.var_afac) + (locals.var_maxvip * locals.var_afac_dn4))) * assign2850_e3252) + (assign2850_e3245 * (((-locals.var_maxvip_dn4) * locals.var_afac) + (assign2850_e3249 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, 0.0, 0.0, (assign2850_e3245 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign2850_e3245 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2850_e3255;
        locals.var_expi_dn4 = assign2850_e3255_d_n4;
        locals.var_expi_dn5 = assign2850_e3255_d_n5;
        locals.var_expi_dn6 = assign2850_e3255_d_n6;
        locals.var_expi_dn7 = assign2850_e3255_d_n7;
        locals.var_expi_dn8 = assign2850_e3255_d_n8;
        locals.var_expi_dn9 = assign2850_e3255_d_n9;
        locals.var_expi_dn10 = assign2850_e3255_d_n10;
        locals.var_expi_dn11 = assign2850_e3255_d_n11;

        let (assign2860_e3263, assign2860_e3263_d_n4, assign2860_e3263_d_n5, assign2860_e3263_d_n6, assign2860_e3263_d_n7, assign2860_e3263_d_n8, assign2860_e3263_d_n9, assign2860_e3263_d_n10, assign2860_e3263_d_n11,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2860_e3260: f64 = (locals.var_expi - 1.0);
        let assign2860_e3261: f64 = (locals.var_isp_t * assign2860_e3260);
        (assign2860_e3261, ((locals.var_isp_t_dn4 * assign2860_e3260) + (locals.var_isp_t * locals.var_expi_dn4)), (locals.var_isp_t * locals.var_expi_dn5), (locals.var_isp_t * locals.var_expi_dn6), (locals.var_isp_t * locals.var_expi_dn7), (locals.var_isp_t * locals.var_expi_dn8), (locals.var_isp_t * locals.var_expi_dn9), (locals.var_isp_t * locals.var_expi_dn10), (locals.var_isp_t * locals.var_expi_dn11),)
    } else {
        (locals.var_irp, locals.var_irp_dn4, locals.var_irp_dn5, locals.var_irp_dn6, locals.var_irp_dn7, locals.var_irp_dn8, locals.var_irp_dn9, locals.var_irp_dn10, locals.var_irp_dn11,)
    }
};
        locals.var_irp = assign2860_e3263;
        locals.var_irp_dn4 = assign2860_e3263_d_n4;
        locals.var_irp_dn5 = assign2860_e3263_d_n5;
        locals.var_irp_dn6 = assign2860_e3263_d_n6;
        locals.var_irp_dn7 = assign2860_e3263_d_n7;
        locals.var_irp_dn8 = assign2860_e3263_d_n8;
        locals.var_irp_dn9 = assign2860_e3263_d_n9;
        locals.var_irp_dn10 = assign2860_e3263_d_n10;
        locals.var_irp_dn11 = assign2860_e3263_d_n11;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2870_e3271, assign2870_e3271_d_n4, assign2870_e3271_d_n5, assign2870_e3271_d_n6, assign2870_e3271_d_n7, assign2870_e3271_d_n8, assign2870_e3271_d_n9, assign2870_e3271_d_n10, assign2870_e3271_d_n11,) = {
    if (locals.var_guard84 != 0.0) {
        let assign2870_e3267: f64 = (locals.var_ifp - locals.var_irp);
        let assign2870_e3269: f64 = (assign2870_e3267 / locals.var_qbp);
        (assign2870_e3269, ((((locals.var_ifp_dn4 - locals.var_irp_dn4) * locals.var_qbp) - (assign2870_e3267 * locals.var_qbp_dn4)) / (locals.var_qbp * locals.var_qbp)), ((((locals.var_ifp_dn5 - locals.var_irp_dn5) * locals.var_qbp) - (assign2870_e3267 * locals.var_qbp_dn5)) / (locals.var_qbp * locals.var_qbp)), ((((locals.var_ifp_dn6 - locals.var_irp_dn6) * locals.var_qbp) - (assign2870_e3267 * locals.var_qbp_dn6)) / (locals.var_qbp * locals.var_qbp)), ((((locals.var_ifp_dn7 - locals.var_irp_dn7) * locals.var_qbp) - (assign2870_e3267 * locals.var_qbp_dn7)) / (locals.var_qbp * locals.var_qbp)), ((((locals.var_ifp_dn8 - locals.var_irp_dn8) * locals.var_qbp) - (assign2870_e3267 * locals.var_qbp_dn8)) / (locals.var_qbp * locals.var_qbp)), ((((locals.var_ifp_dn9 - locals.var_irp_dn9) * locals.var_qbp) - (assign2870_e3267 * locals.var_qbp_dn9)) / (locals.var_qbp * locals.var_qbp)), ((((locals.var_ifp_dn10 - locals.var_irp_dn10) * locals.var_qbp) - (assign2870_e3267 * locals.var_qbp_dn10)) / (locals.var_qbp * locals.var_qbp)), ((((locals.var_ifp_dn11 - locals.var_irp_dn11) * locals.var_qbp) - (assign2870_e3267 * locals.var_qbp_dn11)) / (locals.var_qbp * locals.var_qbp)),)
    } else {
        (locals.var_iccp, locals.var_iccp_dn4, locals.var_iccp_dn5, locals.var_iccp_dn6, locals.var_iccp_dn7, locals.var_iccp_dn8, locals.var_iccp_dn9, locals.var_iccp_dn10, locals.var_iccp_dn11,)
    }
};
        locals.var_iccp = assign2870_e3271;
        locals.var_iccp_dn4 = assign2870_e3271_d_n4;
        locals.var_iccp_dn5 = assign2870_e3271_d_n5;
        locals.var_iccp_dn6 = assign2870_e3271_d_n6;
        locals.var_iccp_dn7 = assign2870_e3271_d_n7;
        locals.var_iccp_dn8 = assign2870_e3271_d_n8;
        locals.var_iccp_dn9 = assign2870_e3271_d_n9;
        locals.var_iccp_dn10 = assign2870_e3271_d_n10;
        locals.var_iccp_dn11 = assign2870_e3271_d_n11;

        let (assign2880_e3276, assign2880_e3276_d_n4, assign2880_e3276_d_n5, assign2880_e3276_d_n6, assign2880_e3276_d_n7, assign2880_e3276_d_n8, assign2880_e3276_d_n9, assign2880_e3276_d_n10, assign2880_e3276_d_n11,) = {
    if (locals.var_guard84 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ifp, locals.var_ifp_dn4, locals.var_ifp_dn5, locals.var_ifp_dn6, locals.var_ifp_dn7, locals.var_ifp_dn8, locals.var_ifp_dn9, locals.var_ifp_dn10, locals.var_ifp_dn11,)
    }
};
        locals.var_ifp = assign2880_e3276;
        locals.var_ifp_dn4 = assign2880_e3276_d_n4;
        locals.var_ifp_dn5 = assign2880_e3276_d_n5;
        locals.var_ifp_dn6 = assign2880_e3276_d_n6;
        locals.var_ifp_dn7 = assign2880_e3276_d_n7;
        locals.var_ifp_dn8 = assign2880_e3276_d_n8;
        locals.var_ifp_dn9 = assign2880_e3276_d_n9;
        locals.var_ifp_dn10 = assign2880_e3276_d_n10;
        locals.var_ifp_dn11 = assign2880_e3276_d_n11;

        let (assign2890_e3281, assign2890_e3281_d_n4, assign2890_e3281_d_n5, assign2890_e3281_d_n6, assign2890_e3281_d_n7, assign2890_e3281_d_n8, assign2890_e3281_d_n9, assign2890_e3281_d_n10, assign2890_e3281_d_n11,) = {
    if (locals.var_guard84 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbp, locals.var_qbp_dn4, locals.var_qbp_dn5, locals.var_qbp_dn6, locals.var_qbp_dn7, locals.var_qbp_dn8, locals.var_qbp_dn9, locals.var_qbp_dn10, locals.var_qbp_dn11,)
    }
};
        locals.var_qbp = assign2890_e3281;
        locals.var_qbp_dn4 = assign2890_e3281_d_n4;
        locals.var_qbp_dn5 = assign2890_e3281_d_n5;
        locals.var_qbp_dn6 = assign2890_e3281_d_n6;
        locals.var_qbp_dn7 = assign2890_e3281_d_n7;
        locals.var_qbp_dn8 = assign2890_e3281_d_n8;
        locals.var_qbp_dn9 = assign2890_e3281_d_n9;
        locals.var_qbp_dn10 = assign2890_e3281_d_n10;
        locals.var_qbp_dn11 = assign2890_e3281_d_n11;

        let (assign2900_e3286, assign2900_e3286_d_n4, assign2900_e3286_d_n5, assign2900_e3286_d_n6, assign2900_e3286_d_n7, assign2900_e3286_d_n8, assign2900_e3286_d_n9, assign2900_e3286_d_n10, assign2900_e3286_d_n11,) = {
    if (locals.var_guard84 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iccp, locals.var_iccp_dn4, locals.var_iccp_dn5, locals.var_iccp_dn6, locals.var_iccp_dn7, locals.var_iccp_dn8, locals.var_iccp_dn9, locals.var_iccp_dn10, locals.var_iccp_dn11,)
    }
};
        locals.var_iccp = assign2900_e3286;
        locals.var_iccp_dn4 = assign2900_e3286_d_n4;
        locals.var_iccp_dn5 = assign2900_e3286_d_n5;
        locals.var_iccp_dn6 = assign2900_e3286_d_n6;
        locals.var_iccp_dn7 = assign2900_e3286_d_n7;
        locals.var_iccp_dn8 = assign2900_e3286_d_n8;
        locals.var_iccp_dn9 = assign2900_e3286_d_n9;
        locals.var_iccp_dn10 = assign2900_e3286_d_n10;
        locals.var_iccp_dn11 = assign2900_e3286_d_n11;

        let assign2910_e3289: f64 = if p.p55 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign2910_e3289;

        let (assign2920_e3297, assign2920_e3297_d_n4,) = {
    if (locals.var_guard89 != 0.0) {
        let assign2920_e3294: f64 = (p.p56 * locals.var_vtv);
        let assign2920_e3295: f64 = (1.0 / assign2920_e3294);
        (assign2920_e3295, (-((p.p56 * locals.var_vtv_dn4) / (assign2920_e3294 * assign2920_e3294))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign2920_e3297;
        locals.var_afac_dn4 = assign2920_e3297_d_n4;

        let assign2930_e3300: f64 = if locals.var_vbei < locals.var_maxvibei { 1.0 } else { 0.0 };
        locals.var_guard90 = assign2930_e3300;

        let (assign2940_e3309, assign2940_e3309_d_n4, assign2940_e3309_d_n5, assign2940_e3309_d_n6, assign2940_e3309_d_n7, assign2940_e3309_d_n8, assign2940_e3309_d_n9, assign2940_e3309_d_n10, assign2940_e3309_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign2940_e3306: f64 = (locals.var_vbei * locals.var_afac);
        let assign2940_e3307: f64 = (assign2940_e3306).exp();
        (assign2940_e3307, (assign2940_e3307 * (locals.var_vbei * locals.var_afac_dn4)), 0.0, 0.0, 0.0, (assign2940_e3307 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2940_e3307 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2940_e3309;
        locals.var_expi_dn4 = assign2940_e3309_d_n4;
        locals.var_expi_dn5 = assign2940_e3309_d_n5;
        locals.var_expi_dn6 = assign2940_e3309_d_n6;
        locals.var_expi_dn7 = assign2940_e3309_d_n7;
        locals.var_expi_dn8 = assign2940_e3309_d_n8;
        locals.var_expi_dn9 = assign2940_e3309_d_n9;
        locals.var_expi_dn10 = assign2940_e3309_d_n10;
        locals.var_expi_dn11 = assign2940_e3309_d_n11;

        let (assign2950_e3327, assign2950_e3327_d_n4, assign2950_e3327_d_n5, assign2950_e3327_d_n6, assign2950_e3327_d_n7, assign2950_e3327_d_n8, assign2950_e3327_d_n9, assign2950_e3327_d_n10, assign2950_e3327_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard90 == 0.0)) {
        let assign2950_e3316: f64 = (locals.var_maxvibei * locals.var_afac);
        let assign2950_e3317: f64 = (assign2950_e3316).exp();
        let assign2950_e3321: f64 = (locals.var_vbei - locals.var_maxvibei);
        let assign2950_e3323: f64 = (assign2950_e3321 * locals.var_afac);
        let assign2950_e3324: f64 = (1.0 + assign2950_e3323);
        let assign2950_e3325: f64 = (assign2950_e3317 * assign2950_e3324);
        (assign2950_e3325, (((assign2950_e3317 * ((locals.var_maxvibei_dn4 * locals.var_afac) + (locals.var_maxvibei * locals.var_afac_dn4))) * assign2950_e3324) + (assign2950_e3317 * (((-locals.var_maxvibei_dn4) * locals.var_afac) + (assign2950_e3321 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign2950_e3317 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2950_e3317 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign2950_e3327;
        locals.var_expi_dn4 = assign2950_e3327_d_n4;
        locals.var_expi_dn5 = assign2950_e3327_d_n5;
        locals.var_expi_dn6 = assign2950_e3327_d_n6;
        locals.var_expi_dn7 = assign2950_e3327_d_n7;
        locals.var_expi_dn8 = assign2950_e3327_d_n8;
        locals.var_expi_dn9 = assign2950_e3327_d_n9;
        locals.var_expi_dn10 = assign2950_e3327_d_n10;
        locals.var_expi_dn11 = assign2950_e3327_d_n11;

        let (assign2960_e3335, assign2960_e3335_d_n4,) = {
    if (locals.var_guard89 != 0.0) {
        let assign2960_e3332: f64 = (p.p59 * locals.var_vtv);
        let assign2960_e3333: f64 = (1.0 / assign2960_e3332);
        (assign2960_e3333, (-((p.p59 * locals.var_vtv_dn4) / (assign2960_e3332 * assign2960_e3332))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign2960_e3335;
        locals.var_afac_dn4 = assign2960_e3335_d_n4;

        let assign2970_e3338: f64 = if locals.var_vbei < locals.var_maxviben { 1.0 } else { 0.0 };
        locals.var_guard91 = assign2970_e3338;

        let (assign2980_e3347, assign2980_e3347_d_n4, assign2980_e3347_d_n6, assign2980_e3347_d_n7, assign2980_e3347_d_n8, assign2980_e3347_d_n9, assign2980_e3347_d_n10, assign2980_e3347_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard91 != 0.0)) {
        let assign2980_e3344: f64 = (locals.var_vbei * locals.var_afac);
        let assign2980_e3345: f64 = (assign2980_e3344).exp();
        (assign2980_e3345, (assign2980_e3345 * (locals.var_vbei * locals.var_afac_dn4)), 0.0, 0.0, (assign2980_e3345 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2980_e3345 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign2980_e3347;
        locals.var_expn_dn4 = assign2980_e3347_d_n4;
        locals.var_expn_dn6 = assign2980_e3347_d_n6;
        locals.var_expn_dn7 = assign2980_e3347_d_n7;
        locals.var_expn_dn8 = assign2980_e3347_d_n8;
        locals.var_expn_dn9 = assign2980_e3347_d_n9;
        locals.var_expn_dn10 = assign2980_e3347_d_n10;
        locals.var_expn_dn11 = assign2980_e3347_d_n11;

        let (assign2990_e3365, assign2990_e3365_d_n4, assign2990_e3365_d_n6, assign2990_e3365_d_n7, assign2990_e3365_d_n8, assign2990_e3365_d_n9, assign2990_e3365_d_n10, assign2990_e3365_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard91 == 0.0)) {
        let assign2990_e3354: f64 = (locals.var_maxviben * locals.var_afac);
        let assign2990_e3355: f64 = (assign2990_e3354).exp();
        let assign2990_e3359: f64 = (locals.var_vbei - locals.var_maxviben);
        let assign2990_e3361: f64 = (assign2990_e3359 * locals.var_afac);
        let assign2990_e3362: f64 = (1.0 + assign2990_e3361);
        let assign2990_e3363: f64 = (assign2990_e3355 * assign2990_e3362);
        (assign2990_e3363, (((assign2990_e3355 * ((locals.var_maxviben_dn4 * locals.var_afac) + (locals.var_maxviben * locals.var_afac_dn4))) * assign2990_e3362) + (assign2990_e3355 * (((-locals.var_maxviben_dn4) * locals.var_afac) + (assign2990_e3359 * locals.var_afac_dn4)))), 0.0, 0.0, (assign2990_e3355 * (locals.var_vbei_dn8 * locals.var_afac)), (assign2990_e3355 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign2990_e3365;
        locals.var_expn_dn4 = assign2990_e3365_d_n4;
        locals.var_expn_dn6 = assign2990_e3365_d_n6;
        locals.var_expn_dn7 = assign2990_e3365_d_n7;
        locals.var_expn_dn8 = assign2990_e3365_d_n8;
        locals.var_expn_dn9 = assign2990_e3365_d_n9;
        locals.var_expn_dn10 = assign2990_e3365_d_n10;
        locals.var_expn_dn11 = assign2990_e3365_d_n11;

        let assign3000_e3368: f64 = if p.p57 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign3000_e3368;

        let (assign3010_e3392, assign3010_e3392_d_n4, assign3010_e3392_d_n5, assign3010_e3392_d_n6, assign3010_e3392_d_n7, assign3010_e3392_d_n8, assign3010_e3392_d_n9, assign3010_e3392_d_n10, assign3010_e3392_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign3010_e3377: f64 = (locals.var_q1 - 1.0);
        let assign3010_e3378: f64 = (p.p57 * assign3010_e3377);
        let assign3010_e3379: f64 = (1.0 + assign3010_e3378);
        let assign3010_e3380: f64 = (locals.var_ibei_t * assign3010_e3379);
        let assign3010_e3383: f64 = (locals.var_expi - 1.0);
        let assign3010_e3384: f64 = (assign3010_e3380 * assign3010_e3383);
        let assign3010_e3388: f64 = (locals.var_expn - 1.0);
        let assign3010_e3389: f64 = (locals.var_iben_t * assign3010_e3388);
        let assign3010_e3390: f64 = (assign3010_e3384 + assign3010_e3389);
        (assign3010_e3390, (((((locals.var_ibei_t_dn4 * assign3010_e3379) + (locals.var_ibei_t * (p.p57 * locals.var_q1_dn4))) * assign3010_e3383) + (assign3010_e3380 * locals.var_expi_dn4)) + ((locals.var_iben_t_dn4 * assign3010_e3388) + (locals.var_iben_t * locals.var_expn_dn4))), (assign3010_e3380 * locals.var_expi_dn5), ((((locals.var_ibei_t * (p.p57 * locals.var_q1_dn6)) * assign3010_e3383) + (assign3010_e3380 * locals.var_expi_dn6)) + (locals.var_iben_t * locals.var_expn_dn6)), ((assign3010_e3380 * locals.var_expi_dn7) + (locals.var_iben_t * locals.var_expn_dn7)), ((((locals.var_ibei_t * (p.p57 * locals.var_q1_dn8)) * assign3010_e3383) + (assign3010_e3380 * locals.var_expi_dn8)) + (locals.var_iben_t * locals.var_expn_dn8)), ((((locals.var_ibei_t * (p.p57 * locals.var_q1_dn9)) * assign3010_e3383) + (assign3010_e3380 * locals.var_expi_dn9)) + (locals.var_iben_t * locals.var_expn_dn9)), ((assign3010_e3380 * locals.var_expi_dn10) + (locals.var_iben_t * locals.var_expn_dn10)), ((assign3010_e3380 * locals.var_expi_dn11) + (locals.var_iben_t * locals.var_expn_dn11)),)
    } else {
        (locals.var_ibe, locals.var_ibe_dn4, locals.var_ibe_dn5, locals.var_ibe_dn6, locals.var_ibe_dn7, locals.var_ibe_dn8, locals.var_ibe_dn9, locals.var_ibe_dn10, locals.var_ibe_dn11,)
    }
};
        locals.var_ibe = assign3010_e3392;
        locals.var_ibe_dn4 = assign3010_e3392_d_n4;
        locals.var_ibe_dn5 = assign3010_e3392_d_n5;
        locals.var_ibe_dn6 = assign3010_e3392_d_n6;
        locals.var_ibe_dn7 = assign3010_e3392_d_n7;
        locals.var_ibe_dn8 = assign3010_e3392_d_n8;
        locals.var_ibe_dn9 = assign3010_e3392_d_n9;
        locals.var_ibe_dn10 = assign3010_e3392_d_n10;
        locals.var_ibe_dn11 = assign3010_e3392_d_n11;

        let (assign3020_e3409, assign3020_e3409_d_n4, assign3020_e3409_d_n5, assign3020_e3409_d_n6, assign3020_e3409_d_n7, assign3020_e3409_d_n8, assign3020_e3409_d_n9, assign3020_e3409_d_n10, assign3020_e3409_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard92 == 0.0)) {
        let assign3020_e3400: f64 = (locals.var_expi - 1.0);
        let assign3020_e3401: f64 = (locals.var_ibei_t * assign3020_e3400);
        let assign3020_e3405: f64 = (locals.var_expn - 1.0);
        let assign3020_e3406: f64 = (locals.var_iben_t * assign3020_e3405);
        let assign3020_e3407: f64 = (assign3020_e3401 + assign3020_e3406);
        (assign3020_e3407, (((locals.var_ibei_t_dn4 * assign3020_e3400) + (locals.var_ibei_t * locals.var_expi_dn4)) + ((locals.var_iben_t_dn4 * assign3020_e3405) + (locals.var_iben_t * locals.var_expn_dn4))), (locals.var_ibei_t * locals.var_expi_dn5), ((locals.var_ibei_t * locals.var_expi_dn6) + (locals.var_iben_t * locals.var_expn_dn6)), ((locals.var_ibei_t * locals.var_expi_dn7) + (locals.var_iben_t * locals.var_expn_dn7)), ((locals.var_ibei_t * locals.var_expi_dn8) + (locals.var_iben_t * locals.var_expn_dn8)), ((locals.var_ibei_t * locals.var_expi_dn9) + (locals.var_iben_t * locals.var_expn_dn9)), ((locals.var_ibei_t * locals.var_expi_dn10) + (locals.var_iben_t * locals.var_expn_dn10)), ((locals.var_ibei_t * locals.var_expi_dn11) + (locals.var_iben_t * locals.var_expn_dn11)),)
    } else {
        (locals.var_ibe, locals.var_ibe_dn4, locals.var_ibe_dn5, locals.var_ibe_dn6, locals.var_ibe_dn7, locals.var_ibe_dn8, locals.var_ibe_dn9, locals.var_ibe_dn10, locals.var_ibe_dn11,)
    }
};
        locals.var_ibe = assign3020_e3409;
        locals.var_ibe_dn4 = assign3020_e3409_d_n4;
        locals.var_ibe_dn5 = assign3020_e3409_d_n5;
        locals.var_ibe_dn6 = assign3020_e3409_d_n6;
        locals.var_ibe_dn7 = assign3020_e3409_d_n7;
        locals.var_ibe_dn8 = assign3020_e3409_d_n8;
        locals.var_ibe_dn9 = assign3020_e3409_d_n9;
        locals.var_ibe_dn10 = assign3020_e3409_d_n10;
        locals.var_ibe_dn11 = assign3020_e3409_d_n11;

        let assign3030_e3412: f64 = if p.p88 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign3030_e3412;

        let (assign3040_e3421, assign3040_e3421_d_n4, assign3040_e3421_d_n8, assign3040_e3421_d_n9,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign3040_e3417: f64 = (-locals.var_vbbe_t);
        let assign3040_e3419: f64 = (assign3040_e3417 - locals.var_vbei);
        (assign3040_e3419, (-locals.var_vbbe_t_dn4), (-locals.var_vbei_dn8), (-locals.var_vbei_dn9),)
    } else {
        (locals.var_bvbe, locals.var_bvbe_dn4, locals.var_bvbe_dn8, locals.var_bvbe_dn9,)
    }
};
        locals.var_bvbe = assign3040_e3421;
        locals.var_bvbe_dn4 = assign3040_e3421_d_n4;
        locals.var_bvbe_dn8 = assign3040_e3421_d_n8;
        locals.var_bvbe_dn9 = assign3040_e3421_d_n9;

        let (assign3050_e3431, assign3050_e3431_d_n4,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign3050_e3428: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign3050_e3429: f64 = (1.0 / assign3050_e3428);
        (assign3050_e3429, (-(((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)) / (assign3050_e3428 * assign3050_e3428))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3050_e3431;
        locals.var_afac_dn4 = assign3050_e3431_d_n4;

        let assign3060_e3434: f64 = if locals.var_bvbe < locals.var_maxvibbe { 1.0 } else { 0.0 };
        locals.var_guard94 = assign3060_e3434;

        let (assign3070_e3445, assign3070_e3445_d_n4, assign3070_e3445_d_n5, assign3070_e3445_d_n6, assign3070_e3445_d_n7, assign3070_e3445_d_n8, assign3070_e3445_d_n9, assign3070_e3445_d_n10, assign3070_e3445_d_n11,) = {
    if (((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 != 0.0)) {
        let assign3070_e3442: f64 = (locals.var_bvbe * locals.var_afac);
        let assign3070_e3443: f64 = (assign3070_e3442).exp();
        (assign3070_e3443, (assign3070_e3443 * ((locals.var_bvbe_dn4 * locals.var_afac) + (locals.var_bvbe * locals.var_afac_dn4))), 0.0, 0.0, 0.0, (assign3070_e3443 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3070_e3443 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3070_e3445;
        locals.var_expx_dn4 = assign3070_e3445_d_n4;
        locals.var_expx_dn5 = assign3070_e3445_d_n5;
        locals.var_expx_dn6 = assign3070_e3445_d_n6;
        locals.var_expx_dn7 = assign3070_e3445_d_n7;
        locals.var_expx_dn8 = assign3070_e3445_d_n8;
        locals.var_expx_dn9 = assign3070_e3445_d_n9;
        locals.var_expx_dn10 = assign3070_e3445_d_n10;
        locals.var_expx_dn11 = assign3070_e3445_d_n11;

        let (assign3080_e3465, assign3080_e3465_d_n4, assign3080_e3465_d_n5, assign3080_e3465_d_n6, assign3080_e3465_d_n7, assign3080_e3465_d_n8, assign3080_e3465_d_n9, assign3080_e3465_d_n10, assign3080_e3465_d_n11,) = {
    if (((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) && (locals.var_guard94 == 0.0)) {
        let assign3080_e3454: f64 = (locals.var_maxvibbe * locals.var_afac);
        let assign3080_e3455: f64 = (assign3080_e3454).exp();
        let assign3080_e3459: f64 = (locals.var_bvbe - locals.var_maxvibbe);
        let assign3080_e3461: f64 = (assign3080_e3459 * locals.var_afac);
        let assign3080_e3462: f64 = (1.0 + assign3080_e3461);
        let assign3080_e3463: f64 = (assign3080_e3455 * assign3080_e3462);
        (assign3080_e3463, (((assign3080_e3455 * ((locals.var_maxvibbe_dn4 * locals.var_afac) + (locals.var_maxvibbe * locals.var_afac_dn4))) * assign3080_e3462) + (assign3080_e3455 * (((locals.var_bvbe_dn4 - locals.var_maxvibbe_dn4) * locals.var_afac) + (assign3080_e3459 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3080_e3455 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3080_e3455 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3080_e3465;
        locals.var_expx_dn4 = assign3080_e3465_d_n4;
        locals.var_expx_dn5 = assign3080_e3465_d_n5;
        locals.var_expx_dn6 = assign3080_e3465_d_n6;
        locals.var_expx_dn7 = assign3080_e3465_d_n7;
        locals.var_expx_dn8 = assign3080_e3465_d_n8;
        locals.var_expx_dn9 = assign3080_e3465_d_n9;
        locals.var_expx_dn10 = assign3080_e3465_d_n10;
        locals.var_expx_dn11 = assign3080_e3465_d_n11;

        let (assign3090_e3477, assign3090_e3477_d_n4, assign3090_e3477_d_n5, assign3090_e3477_d_n6, assign3090_e3477_d_n7, assign3090_e3477_d_n8, assign3090_e3477_d_n9, assign3090_e3477_d_n10, assign3090_e3477_d_n11,) = {
    if ((locals.var_guard89 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign3090_e3473: f64 = (locals.var_expx - locals.var_ebbe_t);
        let assign3090_e3474: f64 = (p.p90 * assign3090_e3473);
        let assign3090_e3475: f64 = (locals.var_ibe - assign3090_e3474);
        (assign3090_e3475, (locals.var_ibe_dn4 - (p.p90 * (locals.var_expx_dn4 - locals.var_ebbe_t_dn4))), (locals.var_ibe_dn5 - (p.p90 * locals.var_expx_dn5)), (locals.var_ibe_dn6 - (p.p90 * locals.var_expx_dn6)), (locals.var_ibe_dn7 - (p.p90 * locals.var_expx_dn7)), (locals.var_ibe_dn8 - (p.p90 * locals.var_expx_dn8)), (locals.var_ibe_dn9 - (p.p90 * locals.var_expx_dn9)), (locals.var_ibe_dn10 - (p.p90 * locals.var_expx_dn10)), (locals.var_ibe_dn11 - (p.p90 * locals.var_expx_dn11)),)
    } else {
        (locals.var_ibe, locals.var_ibe_dn4, locals.var_ibe_dn5, locals.var_ibe_dn6, locals.var_ibe_dn7, locals.var_ibe_dn8, locals.var_ibe_dn9, locals.var_ibe_dn10, locals.var_ibe_dn11,)
    }
};
        locals.var_ibe = assign3090_e3477;
        locals.var_ibe_dn4 = assign3090_e3477_d_n4;
        locals.var_ibe_dn5 = assign3090_e3477_d_n5;
        locals.var_ibe_dn6 = assign3090_e3477_d_n6;
        locals.var_ibe_dn7 = assign3090_e3477_d_n7;
        locals.var_ibe_dn8 = assign3090_e3477_d_n8;
        locals.var_ibe_dn9 = assign3090_e3477_d_n9;
        locals.var_ibe_dn10 = assign3090_e3477_d_n10;
        locals.var_ibe_dn11 = assign3090_e3477_d_n11;

        let (assign3100_e3481, assign3100_e3481_d_n4, assign3100_e3481_d_n5, assign3100_e3481_d_n6, assign3100_e3481_d_n7, assign3100_e3481_d_n8, assign3100_e3481_d_n9, assign3100_e3481_d_n10, assign3100_e3481_d_n11,) = {
    if (locals.var_guard89 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibex, locals.var_ibex_dn4, locals.var_ibex_dn5, locals.var_ibex_dn6, locals.var_ibex_dn7, locals.var_ibex_dn8, locals.var_ibex_dn9, locals.var_ibex_dn10, locals.var_ibex_dn11,)
    }
};
        locals.var_ibex = assign3100_e3481;
        locals.var_ibex_dn4 = assign3100_e3481_d_n4;
        locals.var_ibex_dn5 = assign3100_e3481_d_n5;
        locals.var_ibex_dn6 = assign3100_e3481_d_n6;
        locals.var_ibex_dn7 = assign3100_e3481_d_n7;
        locals.var_ibex_dn8 = assign3100_e3481_d_n8;
        locals.var_ibex_dn9 = assign3100_e3481_d_n9;
        locals.var_ibex_dn10 = assign3100_e3481_d_n10;
        locals.var_ibex_dn11 = assign3100_e3481_d_n11;

        let assign3110_e3484: f64 = if p.p55 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign3110_e3484;

        let (assign3120_e3491, assign3120_e3491_d_n4, assign3120_e3491_d_n5, assign3120_e3491_d_n6, assign3120_e3491_d_n7, assign3120_e3491_d_n8, assign3120_e3491_d_n9, assign3120_e3491_d_n10, assign3120_e3491_d_n11,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibe, locals.var_ibe_dn4, locals.var_ibe_dn5, locals.var_ibe_dn6, locals.var_ibe_dn7, locals.var_ibe_dn8, locals.var_ibe_dn9, locals.var_ibe_dn10, locals.var_ibe_dn11,)
    }
};
        locals.var_ibe = assign3120_e3491;
        locals.var_ibe_dn4 = assign3120_e3491_d_n4;
        locals.var_ibe_dn5 = assign3120_e3491_d_n5;
        locals.var_ibe_dn6 = assign3120_e3491_d_n6;
        locals.var_ibe_dn7 = assign3120_e3491_d_n7;
        locals.var_ibe_dn8 = assign3120_e3491_d_n8;
        locals.var_ibe_dn9 = assign3120_e3491_d_n9;
        locals.var_ibe_dn10 = assign3120_e3491_d_n10;
        locals.var_ibe_dn11 = assign3120_e3491_d_n11;

        let (assign3130_e3502, assign3130_e3502_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) {
        let assign3130_e3499: f64 = (p.p56 * locals.var_vtv);
        let assign3130_e3500: f64 = (1.0 / assign3130_e3499);
        (assign3130_e3500, (-((p.p56 * locals.var_vtv_dn4) / (assign3130_e3499 * assign3130_e3499))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3130_e3502;
        locals.var_afac_dn4 = assign3130_e3502_d_n4;

        let assign3140_e3505: f64 = if locals.var_vbex < locals.var_maxvibei { 1.0 } else { 0.0 };
        locals.var_guard96 = assign3140_e3505;

        let (assign3150_e3517, assign3150_e3517_d_n4, assign3150_e3517_d_n5, assign3150_e3517_d_n6, assign3150_e3517_d_n7, assign3150_e3517_d_n8, assign3150_e3517_d_n9, assign3150_e3517_d_n10, assign3150_e3517_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 != 0.0)) {
        let assign3150_e3514: f64 = (locals.var_vbex * locals.var_afac);
        let assign3150_e3515: f64 = (assign3150_e3514).exp();
        (assign3150_e3515, (assign3150_e3515 * (locals.var_vbex * locals.var_afac_dn4)), 0.0, 0.0, (assign3150_e3515 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3150_e3515 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3150_e3517;
        locals.var_expi_dn4 = assign3150_e3517_d_n4;
        locals.var_expi_dn5 = assign3150_e3517_d_n5;
        locals.var_expi_dn6 = assign3150_e3517_d_n6;
        locals.var_expi_dn7 = assign3150_e3517_d_n7;
        locals.var_expi_dn8 = assign3150_e3517_d_n8;
        locals.var_expi_dn9 = assign3150_e3517_d_n9;
        locals.var_expi_dn10 = assign3150_e3517_d_n10;
        locals.var_expi_dn11 = assign3150_e3517_d_n11;

        let (assign3160_e3538, assign3160_e3538_d_n4, assign3160_e3538_d_n5, assign3160_e3538_d_n6, assign3160_e3538_d_n7, assign3160_e3538_d_n8, assign3160_e3538_d_n9, assign3160_e3538_d_n10, assign3160_e3538_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard96 == 0.0)) {
        let assign3160_e3527: f64 = (locals.var_maxvibei * locals.var_afac);
        let assign3160_e3528: f64 = (assign3160_e3527).exp();
        let assign3160_e3532: f64 = (locals.var_vbex - locals.var_maxvibei);
        let assign3160_e3534: f64 = (assign3160_e3532 * locals.var_afac);
        let assign3160_e3535: f64 = (1.0 + assign3160_e3534);
        let assign3160_e3536: f64 = (assign3160_e3528 * assign3160_e3535);
        (assign3160_e3536, (((assign3160_e3528 * ((locals.var_maxvibei_dn4 * locals.var_afac) + (locals.var_maxvibei * locals.var_afac_dn4))) * assign3160_e3535) + (assign3160_e3528 * (((-locals.var_maxvibei_dn4) * locals.var_afac) + (assign3160_e3532 * locals.var_afac_dn4)))), 0.0, 0.0, (assign3160_e3528 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3160_e3528 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3160_e3538;
        locals.var_expi_dn4 = assign3160_e3538_d_n4;
        locals.var_expi_dn5 = assign3160_e3538_d_n5;
        locals.var_expi_dn6 = assign3160_e3538_d_n6;
        locals.var_expi_dn7 = assign3160_e3538_d_n7;
        locals.var_expi_dn8 = assign3160_e3538_d_n8;
        locals.var_expi_dn9 = assign3160_e3538_d_n9;
        locals.var_expi_dn10 = assign3160_e3538_d_n10;
        locals.var_expi_dn11 = assign3160_e3538_d_n11;

        let (assign3170_e3549, assign3170_e3549_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) {
        let assign3170_e3546: f64 = (p.p59 * locals.var_vtv);
        let assign3170_e3547: f64 = (1.0 / assign3170_e3546);
        (assign3170_e3547, (-((p.p59 * locals.var_vtv_dn4) / (assign3170_e3546 * assign3170_e3546))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3170_e3549;
        locals.var_afac_dn4 = assign3170_e3549_d_n4;

        let assign3180_e3552: f64 = if locals.var_vbex < locals.var_maxviben { 1.0 } else { 0.0 };
        locals.var_guard97 = assign3180_e3552;

        let (assign3190_e3564, assign3190_e3564_d_n4, assign3190_e3564_d_n6, assign3190_e3564_d_n7, assign3190_e3564_d_n8, assign3190_e3564_d_n9, assign3190_e3564_d_n10, assign3190_e3564_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard97 != 0.0)) {
        let assign3190_e3561: f64 = (locals.var_vbex * locals.var_afac);
        let assign3190_e3562: f64 = (assign3190_e3561).exp();
        (assign3190_e3562, (assign3190_e3562 * (locals.var_vbex * locals.var_afac_dn4)), 0.0, (assign3190_e3562 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3190_e3562 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3190_e3564;
        locals.var_expn_dn4 = assign3190_e3564_d_n4;
        locals.var_expn_dn6 = assign3190_e3564_d_n6;
        locals.var_expn_dn7 = assign3190_e3564_d_n7;
        locals.var_expn_dn8 = assign3190_e3564_d_n8;
        locals.var_expn_dn9 = assign3190_e3564_d_n9;
        locals.var_expn_dn10 = assign3190_e3564_d_n10;
        locals.var_expn_dn11 = assign3190_e3564_d_n11;

        let (assign3200_e3585, assign3200_e3585_d_n4, assign3200_e3585_d_n6, assign3200_e3585_d_n7, assign3200_e3585_d_n8, assign3200_e3585_d_n9, assign3200_e3585_d_n10, assign3200_e3585_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard97 == 0.0)) {
        let assign3200_e3574: f64 = (locals.var_maxviben * locals.var_afac);
        let assign3200_e3575: f64 = (assign3200_e3574).exp();
        let assign3200_e3579: f64 = (locals.var_vbex - locals.var_maxviben);
        let assign3200_e3581: f64 = (assign3200_e3579 * locals.var_afac);
        let assign3200_e3582: f64 = (1.0 + assign3200_e3581);
        let assign3200_e3583: f64 = (assign3200_e3575 * assign3200_e3582);
        (assign3200_e3583, (((assign3200_e3575 * ((locals.var_maxviben_dn4 * locals.var_afac) + (locals.var_maxviben * locals.var_afac_dn4))) * assign3200_e3582) + (assign3200_e3575 * (((-locals.var_maxviben_dn4) * locals.var_afac) + (assign3200_e3579 * locals.var_afac_dn4)))), 0.0, (assign3200_e3575 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3200_e3575 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3200_e3585;
        locals.var_expn_dn4 = assign3200_e3585_d_n4;
        locals.var_expn_dn6 = assign3200_e3585_d_n6;
        locals.var_expn_dn7 = assign3200_e3585_d_n7;
        locals.var_expn_dn8 = assign3200_e3585_d_n8;
        locals.var_expn_dn9 = assign3200_e3585_d_n9;
        locals.var_expn_dn10 = assign3200_e3585_d_n10;
        locals.var_expn_dn11 = assign3200_e3585_d_n11;

        let (assign3210_e3602, assign3210_e3602_d_n4, assign3210_e3602_d_n5, assign3210_e3602_d_n6, assign3210_e3602_d_n7, assign3210_e3602_d_n8, assign3210_e3602_d_n9, assign3210_e3602_d_n10, assign3210_e3602_d_n11,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) {
        let assign3210_e3593: f64 = (locals.var_expi - 1.0);
        let assign3210_e3594: f64 = (locals.var_ibei_t * assign3210_e3593);
        let assign3210_e3598: f64 = (locals.var_expn - 1.0);
        let assign3210_e3599: f64 = (locals.var_iben_t * assign3210_e3598);
        let assign3210_e3600: f64 = (assign3210_e3594 + assign3210_e3599);
        (assign3210_e3600, (((locals.var_ibei_t_dn4 * assign3210_e3593) + (locals.var_ibei_t * locals.var_expi_dn4)) + ((locals.var_iben_t_dn4 * assign3210_e3598) + (locals.var_iben_t * locals.var_expn_dn4))), (locals.var_ibei_t * locals.var_expi_dn5), ((locals.var_ibei_t * locals.var_expi_dn6) + (locals.var_iben_t * locals.var_expn_dn6)), ((locals.var_ibei_t * locals.var_expi_dn7) + (locals.var_iben_t * locals.var_expn_dn7)), ((locals.var_ibei_t * locals.var_expi_dn8) + (locals.var_iben_t * locals.var_expn_dn8)), ((locals.var_ibei_t * locals.var_expi_dn9) + (locals.var_iben_t * locals.var_expn_dn9)), ((locals.var_ibei_t * locals.var_expi_dn10) + (locals.var_iben_t * locals.var_expn_dn10)), ((locals.var_ibei_t * locals.var_expi_dn11) + (locals.var_iben_t * locals.var_expn_dn11)),)
    } else {
        (locals.var_ibex, locals.var_ibex_dn4, locals.var_ibex_dn5, locals.var_ibex_dn6, locals.var_ibex_dn7, locals.var_ibex_dn8, locals.var_ibex_dn9, locals.var_ibex_dn10, locals.var_ibex_dn11,)
    }
};
        locals.var_ibex = assign3210_e3602;
        locals.var_ibex_dn4 = assign3210_e3602_d_n4;
        locals.var_ibex_dn5 = assign3210_e3602_d_n5;
        locals.var_ibex_dn6 = assign3210_e3602_d_n6;
        locals.var_ibex_dn7 = assign3210_e3602_d_n7;
        locals.var_ibex_dn8 = assign3210_e3602_d_n8;
        locals.var_ibex_dn9 = assign3210_e3602_d_n9;
        locals.var_ibex_dn10 = assign3210_e3602_d_n10;
        locals.var_ibex_dn11 = assign3210_e3602_d_n11;

        let assign3220_e3605: f64 = if p.p88 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign3220_e3605;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3230_e3617, assign3230_e3617_d_n4, assign3230_e3617_d_n8, assign3230_e3617_d_n9,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign3230_e3613: f64 = (-locals.var_vbbe_t);
        let assign3230_e3615: f64 = (assign3230_e3613 - locals.var_vbei);
        (assign3230_e3615, (-locals.var_vbbe_t_dn4), (-locals.var_vbei_dn8), (-locals.var_vbei_dn9),)
    } else {
        (locals.var_bvbe, locals.var_bvbe_dn4, locals.var_bvbe_dn8, locals.var_bvbe_dn9,)
    }
};
        locals.var_bvbe = assign3230_e3617;
        locals.var_bvbe_dn4 = assign3230_e3617_d_n4;
        locals.var_bvbe_dn8 = assign3230_e3617_d_n8;
        locals.var_bvbe_dn9 = assign3230_e3617_d_n9;

        let (assign3240_e3630, assign3240_e3630_d_n4,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign3240_e3627: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign3240_e3628: f64 = (1.0 / assign3240_e3627);
        (assign3240_e3628, (-(((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)) / (assign3240_e3627 * assign3240_e3627))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3240_e3630;
        locals.var_afac_dn4 = assign3240_e3630_d_n4;

        let assign3250_e3633: f64 = if locals.var_bvbe < locals.var_maxvibbe { 1.0 } else { 0.0 };
        locals.var_guard99 = assign3250_e3633;

        let (assign3260_e3647, assign3260_e3647_d_n4, assign3260_e3647_d_n5, assign3260_e3647_d_n6, assign3260_e3647_d_n7, assign3260_e3647_d_n8, assign3260_e3647_d_n9, assign3260_e3647_d_n10, assign3260_e3647_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
        let assign3260_e3644: f64 = (locals.var_bvbe * locals.var_afac);
        let assign3260_e3645: f64 = (assign3260_e3644).exp();
        (assign3260_e3645, (assign3260_e3645 * ((locals.var_bvbe_dn4 * locals.var_afac) + (locals.var_bvbe * locals.var_afac_dn4))), 0.0, 0.0, 0.0, (assign3260_e3645 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3260_e3645 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3260_e3647;
        locals.var_expx_dn4 = assign3260_e3647_d_n4;
        locals.var_expx_dn5 = assign3260_e3647_d_n5;
        locals.var_expx_dn6 = assign3260_e3647_d_n6;
        locals.var_expx_dn7 = assign3260_e3647_d_n7;
        locals.var_expx_dn8 = assign3260_e3647_d_n8;
        locals.var_expx_dn9 = assign3260_e3647_d_n9;
        locals.var_expx_dn10 = assign3260_e3647_d_n10;
        locals.var_expx_dn11 = assign3260_e3647_d_n11;

        let (assign3270_e3670, assign3270_e3670_d_n4, assign3270_e3670_d_n5, assign3270_e3670_d_n6, assign3270_e3670_d_n7, assign3270_e3670_d_n8, assign3270_e3670_d_n9, assign3270_e3670_d_n10, assign3270_e3670_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 == 0.0)) {
        let assign3270_e3659: f64 = (locals.var_maxvibbe * locals.var_afac);
        let assign3270_e3660: f64 = (assign3270_e3659).exp();
        let assign3270_e3664: f64 = (locals.var_bvbe - locals.var_maxvibbe);
        let assign3270_e3666: f64 = (assign3270_e3664 * locals.var_afac);
        let assign3270_e3667: f64 = (1.0 + assign3270_e3666);
        let assign3270_e3668: f64 = (assign3270_e3660 * assign3270_e3667);
        (assign3270_e3668, (((assign3270_e3660 * ((locals.var_maxvibbe_dn4 * locals.var_afac) + (locals.var_maxvibbe * locals.var_afac_dn4))) * assign3270_e3667) + (assign3270_e3660 * (((locals.var_bvbe_dn4 - locals.var_maxvibbe_dn4) * locals.var_afac) + (assign3270_e3664 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3270_e3660 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3270_e3660 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3270_e3670;
        locals.var_expx_dn4 = assign3270_e3670_d_n4;
        locals.var_expx_dn5 = assign3270_e3670_d_n5;
        locals.var_expx_dn6 = assign3270_e3670_d_n6;
        locals.var_expx_dn7 = assign3270_e3670_d_n7;
        locals.var_expx_dn8 = assign3270_e3670_d_n8;
        locals.var_expx_dn9 = assign3270_e3670_d_n9;
        locals.var_expx_dn10 = assign3270_e3670_d_n10;
        locals.var_expx_dn11 = assign3270_e3670_d_n11;

        let (assign3280_e3685, assign3280_e3685_d_n4, assign3280_e3685_d_n5, assign3280_e3685_d_n6, assign3280_e3685_d_n7, assign3280_e3685_d_n8, assign3280_e3685_d_n9, assign3280_e3685_d_n10, assign3280_e3685_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 != 0.0)) && (locals.var_guard98 != 0.0)) {
        let assign3280_e3681: f64 = (locals.var_expx - locals.var_ebbe_t);
        let assign3280_e3682: f64 = (p.p90 * assign3280_e3681);
        let assign3280_e3683: f64 = (locals.var_ibex - assign3280_e3682);
        (assign3280_e3683, (locals.var_ibex_dn4 - (p.p90 * (locals.var_expx_dn4 - locals.var_ebbe_t_dn4))), (locals.var_ibex_dn5 - (p.p90 * locals.var_expx_dn5)), (locals.var_ibex_dn6 - (p.p90 * locals.var_expx_dn6)), (locals.var_ibex_dn7 - (p.p90 * locals.var_expx_dn7)), (locals.var_ibex_dn8 - (p.p90 * locals.var_expx_dn8)), (locals.var_ibex_dn9 - (p.p90 * locals.var_expx_dn9)), (locals.var_ibex_dn10 - (p.p90 * locals.var_expx_dn10)), (locals.var_ibex_dn11 - (p.p90 * locals.var_expx_dn11)),)
    } else {
        (locals.var_ibex, locals.var_ibex_dn4, locals.var_ibex_dn5, locals.var_ibex_dn6, locals.var_ibex_dn7, locals.var_ibex_dn8, locals.var_ibex_dn9, locals.var_ibex_dn10, locals.var_ibex_dn11,)
    }
};
        locals.var_ibex = assign3280_e3685;
        locals.var_ibex_dn4 = assign3280_e3685_d_n4;
        locals.var_ibex_dn5 = assign3280_e3685_d_n5;
        locals.var_ibex_dn6 = assign3280_e3685_d_n6;
        locals.var_ibex_dn7 = assign3280_e3685_d_n7;
        locals.var_ibex_dn8 = assign3280_e3685_d_n8;
        locals.var_ibex_dn9 = assign3280_e3685_d_n9;
        locals.var_ibex_dn10 = assign3280_e3685_d_n10;
        locals.var_ibex_dn11 = assign3280_e3685_d_n11;

        let (assign3290_e3697, assign3290_e3697_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3290_e3694: f64 = (p.p56 * locals.var_vtv);
        let assign3290_e3695: f64 = (1.0 / assign3290_e3694);
        (assign3290_e3695, (-((p.p56 * locals.var_vtv_dn4) / (assign3290_e3694 * assign3290_e3694))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3290_e3697;
        locals.var_afac_dn4 = assign3290_e3697_d_n4;

        let assign3300_e3700: f64 = if locals.var_vbei < locals.var_maxvibei { 1.0 } else { 0.0 };
        locals.var_guard100 = assign3300_e3700;

        let (assign3310_e3713, assign3310_e3713_d_n4, assign3310_e3713_d_n5, assign3310_e3713_d_n6, assign3310_e3713_d_n7, assign3310_e3713_d_n8, assign3310_e3713_d_n9, assign3310_e3713_d_n10, assign3310_e3713_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard100 != 0.0)) {
        let assign3310_e3710: f64 = (locals.var_vbei * locals.var_afac);
        let assign3310_e3711: f64 = (assign3310_e3710).exp();
        (assign3310_e3711, (assign3310_e3711 * (locals.var_vbei * locals.var_afac_dn4)), 0.0, 0.0, 0.0, (assign3310_e3711 * (locals.var_vbei_dn8 * locals.var_afac)), (assign3310_e3711 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3310_e3713;
        locals.var_expi_dn4 = assign3310_e3713_d_n4;
        locals.var_expi_dn5 = assign3310_e3713_d_n5;
        locals.var_expi_dn6 = assign3310_e3713_d_n6;
        locals.var_expi_dn7 = assign3310_e3713_d_n7;
        locals.var_expi_dn8 = assign3310_e3713_d_n8;
        locals.var_expi_dn9 = assign3310_e3713_d_n9;
        locals.var_expi_dn10 = assign3310_e3713_d_n10;
        locals.var_expi_dn11 = assign3310_e3713_d_n11;

        let (assign3320_e3735, assign3320_e3735_d_n4, assign3320_e3735_d_n5, assign3320_e3735_d_n6, assign3320_e3735_d_n7, assign3320_e3735_d_n8, assign3320_e3735_d_n9, assign3320_e3735_d_n10, assign3320_e3735_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard100 == 0.0)) {
        let assign3320_e3724: f64 = (locals.var_maxvibei * locals.var_afac);
        let assign3320_e3725: f64 = (assign3320_e3724).exp();
        let assign3320_e3729: f64 = (locals.var_vbei - locals.var_maxvibei);
        let assign3320_e3731: f64 = (assign3320_e3729 * locals.var_afac);
        let assign3320_e3732: f64 = (1.0 + assign3320_e3731);
        let assign3320_e3733: f64 = (assign3320_e3725 * assign3320_e3732);
        (assign3320_e3733, (((assign3320_e3725 * ((locals.var_maxvibei_dn4 * locals.var_afac) + (locals.var_maxvibei * locals.var_afac_dn4))) * assign3320_e3732) + (assign3320_e3725 * (((-locals.var_maxvibei_dn4) * locals.var_afac) + (assign3320_e3729 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3320_e3725 * (locals.var_vbei_dn8 * locals.var_afac)), (assign3320_e3725 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3320_e3735;
        locals.var_expi_dn4 = assign3320_e3735_d_n4;
        locals.var_expi_dn5 = assign3320_e3735_d_n5;
        locals.var_expi_dn6 = assign3320_e3735_d_n6;
        locals.var_expi_dn7 = assign3320_e3735_d_n7;
        locals.var_expi_dn8 = assign3320_e3735_d_n8;
        locals.var_expi_dn9 = assign3320_e3735_d_n9;
        locals.var_expi_dn10 = assign3320_e3735_d_n10;
        locals.var_expi_dn11 = assign3320_e3735_d_n11;

        let (assign3330_e3747, assign3330_e3747_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3330_e3744: f64 = (p.p59 * locals.var_vtv);
        let assign3330_e3745: f64 = (1.0 / assign3330_e3744);
        (assign3330_e3745, (-((p.p59 * locals.var_vtv_dn4) / (assign3330_e3744 * assign3330_e3744))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3330_e3747;
        locals.var_afac_dn4 = assign3330_e3747_d_n4;

        let assign3340_e3750: f64 = if locals.var_vbei < locals.var_maxviben { 1.0 } else { 0.0 };
        locals.var_guard101 = assign3340_e3750;

        let (assign3350_e3763, assign3350_e3763_d_n4, assign3350_e3763_d_n6, assign3350_e3763_d_n7, assign3350_e3763_d_n8, assign3350_e3763_d_n9, assign3350_e3763_d_n10, assign3350_e3763_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard101 != 0.0)) {
        let assign3350_e3760: f64 = (locals.var_vbei * locals.var_afac);
        let assign3350_e3761: f64 = (assign3350_e3760).exp();
        (assign3350_e3761, (assign3350_e3761 * (locals.var_vbei * locals.var_afac_dn4)), 0.0, 0.0, (assign3350_e3761 * (locals.var_vbei_dn8 * locals.var_afac)), (assign3350_e3761 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3350_e3763;
        locals.var_expn_dn4 = assign3350_e3763_d_n4;
        locals.var_expn_dn6 = assign3350_e3763_d_n6;
        locals.var_expn_dn7 = assign3350_e3763_d_n7;
        locals.var_expn_dn8 = assign3350_e3763_d_n8;
        locals.var_expn_dn9 = assign3350_e3763_d_n9;
        locals.var_expn_dn10 = assign3350_e3763_d_n10;
        locals.var_expn_dn11 = assign3350_e3763_d_n11;

        let (assign3360_e3785, assign3360_e3785_d_n4, assign3360_e3785_d_n6, assign3360_e3785_d_n7, assign3360_e3785_d_n8, assign3360_e3785_d_n9, assign3360_e3785_d_n10, assign3360_e3785_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard101 == 0.0)) {
        let assign3360_e3774: f64 = (locals.var_maxviben * locals.var_afac);
        let assign3360_e3775: f64 = (assign3360_e3774).exp();
        let assign3360_e3779: f64 = (locals.var_vbei - locals.var_maxviben);
        let assign3360_e3781: f64 = (assign3360_e3779 * locals.var_afac);
        let assign3360_e3782: f64 = (1.0 + assign3360_e3781);
        let assign3360_e3783: f64 = (assign3360_e3775 * assign3360_e3782);
        (assign3360_e3783, (((assign3360_e3775 * ((locals.var_maxviben_dn4 * locals.var_afac) + (locals.var_maxviben * locals.var_afac_dn4))) * assign3360_e3782) + (assign3360_e3775 * (((-locals.var_maxviben_dn4) * locals.var_afac) + (assign3360_e3779 * locals.var_afac_dn4)))), 0.0, 0.0, (assign3360_e3775 * (locals.var_vbei_dn8 * locals.var_afac)), (assign3360_e3775 * (locals.var_vbei_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3360_e3785;
        locals.var_expn_dn4 = assign3360_e3785_d_n4;
        locals.var_expn_dn6 = assign3360_e3785_d_n6;
        locals.var_expn_dn7 = assign3360_e3785_d_n7;
        locals.var_expn_dn8 = assign3360_e3785_d_n8;
        locals.var_expn_dn9 = assign3360_e3785_d_n9;
        locals.var_expn_dn10 = assign3360_e3785_d_n10;
        locals.var_expn_dn11 = assign3360_e3785_d_n11;

        let assign3370_e3788: f64 = if p.p57 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign3370_e3788;

        let (assign3380_e3818, assign3380_e3818_d_n4, assign3380_e3818_d_n5, assign3380_e3818_d_n6, assign3380_e3818_d_n7, assign3380_e3818_d_n8, assign3380_e3818_d_n9, assign3380_e3818_d_n10, assign3380_e3818_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard102 != 0.0)) {
        let assign3380_e3802: f64 = (locals.var_q1 - 1.0);
        let assign3380_e3803: f64 = (p.p57 * assign3380_e3802);
        let assign3380_e3804: f64 = (1.0 + assign3380_e3803);
        let assign3380_e3805: f64 = (locals.var_ibei_t * assign3380_e3804);
        let assign3380_e3808: f64 = (locals.var_expi - 1.0);
        let assign3380_e3809: f64 = (assign3380_e3805 * assign3380_e3808);
        let assign3380_e3813: f64 = (locals.var_expn - 1.0);
        let assign3380_e3814: f64 = (locals.var_iben_t * assign3380_e3813);
        let assign3380_e3815: f64 = (assign3380_e3809 + assign3380_e3814);
        let assign3380_e3816: f64 = (p.p55 * assign3380_e3815);
        (assign3380_e3816, (p.p55 * (((((locals.var_ibei_t_dn4 * assign3380_e3804) + (locals.var_ibei_t * (p.p57 * locals.var_q1_dn4))) * assign3380_e3808) + (assign3380_e3805 * locals.var_expi_dn4)) + ((locals.var_iben_t_dn4 * assign3380_e3813) + (locals.var_iben_t * locals.var_expn_dn4)))), (p.p55 * (assign3380_e3805 * locals.var_expi_dn5)), (p.p55 * ((((locals.var_ibei_t * (p.p57 * locals.var_q1_dn6)) * assign3380_e3808) + (assign3380_e3805 * locals.var_expi_dn6)) + (locals.var_iben_t * locals.var_expn_dn6))), (p.p55 * ((assign3380_e3805 * locals.var_expi_dn7) + (locals.var_iben_t * locals.var_expn_dn7))), (p.p55 * ((((locals.var_ibei_t * (p.p57 * locals.var_q1_dn8)) * assign3380_e3808) + (assign3380_e3805 * locals.var_expi_dn8)) + (locals.var_iben_t * locals.var_expn_dn8))), (p.p55 * ((((locals.var_ibei_t * (p.p57 * locals.var_q1_dn9)) * assign3380_e3808) + (assign3380_e3805 * locals.var_expi_dn9)) + (locals.var_iben_t * locals.var_expn_dn9))), (p.p55 * ((assign3380_e3805 * locals.var_expi_dn10) + (locals.var_iben_t * locals.var_expn_dn10))), (p.p55 * ((assign3380_e3805 * locals.var_expi_dn11) + (locals.var_iben_t * locals.var_expn_dn11))),)
    } else {
        (locals.var_ibe, locals.var_ibe_dn4, locals.var_ibe_dn5, locals.var_ibe_dn6, locals.var_ibe_dn7, locals.var_ibe_dn8, locals.var_ibe_dn9, locals.var_ibe_dn10, locals.var_ibe_dn11,)
    }
};
        locals.var_ibe = assign3380_e3818;
        locals.var_ibe_dn4 = assign3380_e3818_d_n4;
        locals.var_ibe_dn5 = assign3380_e3818_d_n5;
        locals.var_ibe_dn6 = assign3380_e3818_d_n6;
        locals.var_ibe_dn7 = assign3380_e3818_d_n7;
        locals.var_ibe_dn8 = assign3380_e3818_d_n8;
        locals.var_ibe_dn9 = assign3380_e3818_d_n9;
        locals.var_ibe_dn10 = assign3380_e3818_d_n10;
        locals.var_ibe_dn11 = assign3380_e3818_d_n11;

        let (assign3390_e3841, assign3390_e3841_d_n4, assign3390_e3841_d_n5, assign3390_e3841_d_n6, assign3390_e3841_d_n7, assign3390_e3841_d_n8, assign3390_e3841_d_n9, assign3390_e3841_d_n10, assign3390_e3841_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard102 == 0.0)) {
        let assign3390_e3831: f64 = (locals.var_expi - 1.0);
        let assign3390_e3832: f64 = (locals.var_ibei_t * assign3390_e3831);
        let assign3390_e3836: f64 = (locals.var_expn - 1.0);
        let assign3390_e3837: f64 = (locals.var_iben_t * assign3390_e3836);
        let assign3390_e3838: f64 = (assign3390_e3832 + assign3390_e3837);
        let assign3390_e3839: f64 = (p.p55 * assign3390_e3838);
        (assign3390_e3839, (p.p55 * (((locals.var_ibei_t_dn4 * assign3390_e3831) + (locals.var_ibei_t * locals.var_expi_dn4)) + ((locals.var_iben_t_dn4 * assign3390_e3836) + (locals.var_iben_t * locals.var_expn_dn4)))), (p.p55 * (locals.var_ibei_t * locals.var_expi_dn5)), (p.p55 * ((locals.var_ibei_t * locals.var_expi_dn6) + (locals.var_iben_t * locals.var_expn_dn6))), (p.p55 * ((locals.var_ibei_t * locals.var_expi_dn7) + (locals.var_iben_t * locals.var_expn_dn7))), (p.p55 * ((locals.var_ibei_t * locals.var_expi_dn8) + (locals.var_iben_t * locals.var_expn_dn8))), (p.p55 * ((locals.var_ibei_t * locals.var_expi_dn9) + (locals.var_iben_t * locals.var_expn_dn9))), (p.p55 * ((locals.var_ibei_t * locals.var_expi_dn10) + (locals.var_iben_t * locals.var_expn_dn10))), (p.p55 * ((locals.var_ibei_t * locals.var_expi_dn11) + (locals.var_iben_t * locals.var_expn_dn11))),)
    } else {
        (locals.var_ibe, locals.var_ibe_dn4, locals.var_ibe_dn5, locals.var_ibe_dn6, locals.var_ibe_dn7, locals.var_ibe_dn8, locals.var_ibe_dn9, locals.var_ibe_dn10, locals.var_ibe_dn11,)
    }
};
        locals.var_ibe = assign3390_e3841;
        locals.var_ibe_dn4 = assign3390_e3841_d_n4;
        locals.var_ibe_dn5 = assign3390_e3841_d_n5;
        locals.var_ibe_dn6 = assign3390_e3841_d_n6;
        locals.var_ibe_dn7 = assign3390_e3841_d_n7;
        locals.var_ibe_dn8 = assign3390_e3841_d_n8;
        locals.var_ibe_dn9 = assign3390_e3841_d_n9;
        locals.var_ibe_dn10 = assign3390_e3841_d_n10;
        locals.var_ibe_dn11 = assign3390_e3841_d_n11;

        let assign3400_e3844: f64 = if p.p88 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign3400_e3844;

        let (assign3410_e3857, assign3410_e3857_d_n4, assign3410_e3857_d_n8, assign3410_e3857_d_n9,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign3410_e3853: f64 = (-locals.var_vbbe_t);
        let assign3410_e3855: f64 = (assign3410_e3853 - locals.var_vbei);
        (assign3410_e3855, (-locals.var_vbbe_t_dn4), (-locals.var_vbei_dn8), (-locals.var_vbei_dn9),)
    } else {
        (locals.var_bvbe, locals.var_bvbe_dn4, locals.var_bvbe_dn8, locals.var_bvbe_dn9,)
    }
};
        locals.var_bvbe = assign3410_e3857;
        locals.var_bvbe_dn4 = assign3410_e3857_d_n4;
        locals.var_bvbe_dn8 = assign3410_e3857_d_n8;
        locals.var_bvbe_dn9 = assign3410_e3857_d_n9;

        let (assign3420_e3871, assign3420_e3871_d_n4,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign3420_e3868: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign3420_e3869: f64 = (1.0 / assign3420_e3868);
        (assign3420_e3869, (-(((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)) / (assign3420_e3868 * assign3420_e3868))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3420_e3871;
        locals.var_afac_dn4 = assign3420_e3871_d_n4;

        let assign3430_e3874: f64 = if locals.var_bvbe < locals.var_maxvibbe { 1.0 } else { 0.0 };
        locals.var_guard104 = assign3430_e3874;

        let (assign3440_e3889, assign3440_e3889_d_n4, assign3440_e3889_d_n5, assign3440_e3889_d_n6, assign3440_e3889_d_n7, assign3440_e3889_d_n8, assign3440_e3889_d_n9, assign3440_e3889_d_n10, assign3440_e3889_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 != 0.0)) {
        let assign3440_e3886: f64 = (locals.var_bvbe * locals.var_afac);
        let assign3440_e3887: f64 = (assign3440_e3886).exp();
        (assign3440_e3887, (assign3440_e3887 * ((locals.var_bvbe_dn4 * locals.var_afac) + (locals.var_bvbe * locals.var_afac_dn4))), 0.0, 0.0, 0.0, (assign3440_e3887 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3440_e3887 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3440_e3889;
        locals.var_expx_dn4 = assign3440_e3889_d_n4;
        locals.var_expx_dn5 = assign3440_e3889_d_n5;
        locals.var_expx_dn6 = assign3440_e3889_d_n6;
        locals.var_expx_dn7 = assign3440_e3889_d_n7;
        locals.var_expx_dn8 = assign3440_e3889_d_n8;
        locals.var_expx_dn9 = assign3440_e3889_d_n9;
        locals.var_expx_dn10 = assign3440_e3889_d_n10;
        locals.var_expx_dn11 = assign3440_e3889_d_n11;

        let (assign3450_e3913, assign3450_e3913_d_n4, assign3450_e3913_d_n5, assign3450_e3913_d_n6, assign3450_e3913_d_n7, assign3450_e3913_d_n8, assign3450_e3913_d_n9, assign3450_e3913_d_n10, assign3450_e3913_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) && (locals.var_guard104 == 0.0)) {
        let assign3450_e3902: f64 = (locals.var_maxvibbe * locals.var_afac);
        let assign3450_e3903: f64 = (assign3450_e3902).exp();
        let assign3450_e3907: f64 = (locals.var_bvbe - locals.var_maxvibbe);
        let assign3450_e3909: f64 = (assign3450_e3907 * locals.var_afac);
        let assign3450_e3910: f64 = (1.0 + assign3450_e3909);
        let assign3450_e3911: f64 = (assign3450_e3903 * assign3450_e3910);
        (assign3450_e3911, (((assign3450_e3903 * ((locals.var_maxvibbe_dn4 * locals.var_afac) + (locals.var_maxvibbe * locals.var_afac_dn4))) * assign3450_e3910) + (assign3450_e3903 * (((locals.var_bvbe_dn4 - locals.var_maxvibbe_dn4) * locals.var_afac) + (assign3450_e3907 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3450_e3903 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3450_e3903 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3450_e3913;
        locals.var_expx_dn4 = assign3450_e3913_d_n4;
        locals.var_expx_dn5 = assign3450_e3913_d_n5;
        locals.var_expx_dn6 = assign3450_e3913_d_n6;
        locals.var_expx_dn7 = assign3450_e3913_d_n7;
        locals.var_expx_dn8 = assign3450_e3913_d_n8;
        locals.var_expx_dn9 = assign3450_e3913_d_n9;
        locals.var_expx_dn10 = assign3450_e3913_d_n10;
        locals.var_expx_dn11 = assign3450_e3913_d_n11;

        let (assign3460_e3931, assign3460_e3931_d_n4, assign3460_e3931_d_n5, assign3460_e3931_d_n6, assign3460_e3931_d_n7, assign3460_e3931_d_n8, assign3460_e3931_d_n9, assign3460_e3931_d_n10, assign3460_e3931_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard103 != 0.0)) {
        let assign3460_e3924: f64 = (p.p55 * p.p90);
        let assign3460_e3927: f64 = (locals.var_expx - locals.var_ebbe_t);
        let assign3460_e3928: f64 = (assign3460_e3924 * assign3460_e3927);
        let assign3460_e3929: f64 = (locals.var_ibe - assign3460_e3928);
        (assign3460_e3929, (locals.var_ibe_dn4 - (assign3460_e3924 * (locals.var_expx_dn4 - locals.var_ebbe_t_dn4))), (locals.var_ibe_dn5 - (assign3460_e3924 * locals.var_expx_dn5)), (locals.var_ibe_dn6 - (assign3460_e3924 * locals.var_expx_dn6)), (locals.var_ibe_dn7 - (assign3460_e3924 * locals.var_expx_dn7)), (locals.var_ibe_dn8 - (assign3460_e3924 * locals.var_expx_dn8)), (locals.var_ibe_dn9 - (assign3460_e3924 * locals.var_expx_dn9)), (locals.var_ibe_dn10 - (assign3460_e3924 * locals.var_expx_dn10)), (locals.var_ibe_dn11 - (assign3460_e3924 * locals.var_expx_dn11)),)
    } else {
        (locals.var_ibe, locals.var_ibe_dn4, locals.var_ibe_dn5, locals.var_ibe_dn6, locals.var_ibe_dn7, locals.var_ibe_dn8, locals.var_ibe_dn9, locals.var_ibe_dn10, locals.var_ibe_dn11,)
    }
};
        locals.var_ibe = assign3460_e3931;
        locals.var_ibe_dn4 = assign3460_e3931_d_n4;
        locals.var_ibe_dn5 = assign3460_e3931_d_n5;
        locals.var_ibe_dn6 = assign3460_e3931_d_n6;
        locals.var_ibe_dn7 = assign3460_e3931_d_n7;
        locals.var_ibe_dn8 = assign3460_e3931_d_n8;
        locals.var_ibe_dn9 = assign3460_e3931_d_n9;
        locals.var_ibe_dn10 = assign3460_e3931_d_n10;
        locals.var_ibe_dn11 = assign3460_e3931_d_n11;

        let (assign3470_e3943, assign3470_e3943_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3470_e3940: f64 = (p.p56 * locals.var_vtv);
        let assign3470_e3941: f64 = (1.0 / assign3470_e3940);
        (assign3470_e3941, (-((p.p56 * locals.var_vtv_dn4) / (assign3470_e3940 * assign3470_e3940))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3470_e3943;
        locals.var_afac_dn4 = assign3470_e3943_d_n4;

        let assign3480_e3946: f64 = if locals.var_vbex < locals.var_maxvibei { 1.0 } else { 0.0 };
        locals.var_guard105 = assign3480_e3946;

        let (assign3490_e3959, assign3490_e3959_d_n4, assign3490_e3959_d_n5, assign3490_e3959_d_n6, assign3490_e3959_d_n7, assign3490_e3959_d_n8, assign3490_e3959_d_n9, assign3490_e3959_d_n10, assign3490_e3959_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard105 != 0.0)) {
        let assign3490_e3956: f64 = (locals.var_vbex * locals.var_afac);
        let assign3490_e3957: f64 = (assign3490_e3956).exp();
        (assign3490_e3957, (assign3490_e3957 * (locals.var_vbex * locals.var_afac_dn4)), 0.0, 0.0, (assign3490_e3957 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3490_e3957 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3490_e3959;
        locals.var_expi_dn4 = assign3490_e3959_d_n4;
        locals.var_expi_dn5 = assign3490_e3959_d_n5;
        locals.var_expi_dn6 = assign3490_e3959_d_n6;
        locals.var_expi_dn7 = assign3490_e3959_d_n7;
        locals.var_expi_dn8 = assign3490_e3959_d_n8;
        locals.var_expi_dn9 = assign3490_e3959_d_n9;
        locals.var_expi_dn10 = assign3490_e3959_d_n10;
        locals.var_expi_dn11 = assign3490_e3959_d_n11;

        let (assign3500_e3981, assign3500_e3981_d_n4, assign3500_e3981_d_n5, assign3500_e3981_d_n6, assign3500_e3981_d_n7, assign3500_e3981_d_n8, assign3500_e3981_d_n9, assign3500_e3981_d_n10, assign3500_e3981_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard105 == 0.0)) {
        let assign3500_e3970: f64 = (locals.var_maxvibei * locals.var_afac);
        let assign3500_e3971: f64 = (assign3500_e3970).exp();
        let assign3500_e3975: f64 = (locals.var_vbex - locals.var_maxvibei);
        let assign3500_e3977: f64 = (assign3500_e3975 * locals.var_afac);
        let assign3500_e3978: f64 = (1.0 + assign3500_e3977);
        let assign3500_e3979: f64 = (assign3500_e3971 * assign3500_e3978);
        (assign3500_e3979, (((assign3500_e3971 * ((locals.var_maxvibei_dn4 * locals.var_afac) + (locals.var_maxvibei * locals.var_afac_dn4))) * assign3500_e3978) + (assign3500_e3971 * (((-locals.var_maxvibei_dn4) * locals.var_afac) + (assign3500_e3975 * locals.var_afac_dn4)))), 0.0, 0.0, (assign3500_e3971 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3500_e3971 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3500_e3981;
        locals.var_expi_dn4 = assign3500_e3981_d_n4;
        locals.var_expi_dn5 = assign3500_e3981_d_n5;
        locals.var_expi_dn6 = assign3500_e3981_d_n6;
        locals.var_expi_dn7 = assign3500_e3981_d_n7;
        locals.var_expi_dn8 = assign3500_e3981_d_n8;
        locals.var_expi_dn9 = assign3500_e3981_d_n9;
        locals.var_expi_dn10 = assign3500_e3981_d_n10;
        locals.var_expi_dn11 = assign3500_e3981_d_n11;

        let (assign3510_e3993, assign3510_e3993_d_n4,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3510_e3990: f64 = (p.p59 * locals.var_vtv);
        let assign3510_e3991: f64 = (1.0 / assign3510_e3990);
        (assign3510_e3991, (-((p.p59 * locals.var_vtv_dn4) / (assign3510_e3990 * assign3510_e3990))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3510_e3993;
        locals.var_afac_dn4 = assign3510_e3993_d_n4;

        let assign3520_e3996: f64 = if locals.var_vbex < locals.var_maxviben { 1.0 } else { 0.0 };
        locals.var_guard106 = assign3520_e3996;

        let (assign3530_e4009, assign3530_e4009_d_n4, assign3530_e4009_d_n6, assign3530_e4009_d_n7, assign3530_e4009_d_n8, assign3530_e4009_d_n9, assign3530_e4009_d_n10, assign3530_e4009_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard106 != 0.0)) {
        let assign3530_e4006: f64 = (locals.var_vbex * locals.var_afac);
        let assign3530_e4007: f64 = (assign3530_e4006).exp();
        (assign3530_e4007, (assign3530_e4007 * (locals.var_vbex * locals.var_afac_dn4)), 0.0, (assign3530_e4007 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3530_e4007 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3530_e4009;
        locals.var_expn_dn4 = assign3530_e4009_d_n4;
        locals.var_expn_dn6 = assign3530_e4009_d_n6;
        locals.var_expn_dn7 = assign3530_e4009_d_n7;
        locals.var_expn_dn8 = assign3530_e4009_d_n8;
        locals.var_expn_dn9 = assign3530_e4009_d_n9;
        locals.var_expn_dn10 = assign3530_e4009_d_n10;
        locals.var_expn_dn11 = assign3530_e4009_d_n11;

        let (assign3540_e4031, assign3540_e4031_d_n4, assign3540_e4031_d_n6, assign3540_e4031_d_n7, assign3540_e4031_d_n8, assign3540_e4031_d_n9, assign3540_e4031_d_n10, assign3540_e4031_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard106 == 0.0)) {
        let assign3540_e4020: f64 = (locals.var_maxviben * locals.var_afac);
        let assign3540_e4021: f64 = (assign3540_e4020).exp();
        let assign3540_e4025: f64 = (locals.var_vbex - locals.var_maxviben);
        let assign3540_e4027: f64 = (assign3540_e4025 * locals.var_afac);
        let assign3540_e4028: f64 = (1.0 + assign3540_e4027);
        let assign3540_e4029: f64 = (assign3540_e4021 * assign3540_e4028);
        (assign3540_e4029, (((assign3540_e4021 * ((locals.var_maxviben_dn4 * locals.var_afac) + (locals.var_maxviben * locals.var_afac_dn4))) * assign3540_e4028) + (assign3540_e4021 * (((-locals.var_maxviben_dn4) * locals.var_afac) + (assign3540_e4025 * locals.var_afac_dn4)))), 0.0, (assign3540_e4021 * (locals.var_vbex_dn7 * locals.var_afac)), 0.0, (assign3540_e4021 * (locals.var_vbex_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3540_e4031;
        locals.var_expn_dn4 = assign3540_e4031_d_n4;
        locals.var_expn_dn6 = assign3540_e4031_d_n6;
        locals.var_expn_dn7 = assign3540_e4031_d_n7;
        locals.var_expn_dn8 = assign3540_e4031_d_n8;
        locals.var_expn_dn9 = assign3540_e4031_d_n9;
        locals.var_expn_dn10 = assign3540_e4031_d_n10;
        locals.var_expn_dn11 = assign3540_e4031_d_n11;

        let (assign3550_e4053, assign3550_e4053_d_n4, assign3550_e4053_d_n5, assign3550_e4053_d_n6, assign3550_e4053_d_n7, assign3550_e4053_d_n8, assign3550_e4053_d_n9, assign3550_e4053_d_n10, assign3550_e4053_d_n11,) = {
    if ((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) {
        let assign3550_e4039: f64 = (1.0 - p.p55);
        let assign3550_e4043: f64 = (locals.var_expi - 1.0);
        let assign3550_e4044: f64 = (locals.var_ibei_t * assign3550_e4043);
        let assign3550_e4048: f64 = (locals.var_expn - 1.0);
        let assign3550_e4049: f64 = (locals.var_iben_t * assign3550_e4048);
        let assign3550_e4050: f64 = (assign3550_e4044 + assign3550_e4049);
        let assign3550_e4051: f64 = (assign3550_e4039 * assign3550_e4050);
        (assign3550_e4051, (assign3550_e4039 * (((locals.var_ibei_t_dn4 * assign3550_e4043) + (locals.var_ibei_t * locals.var_expi_dn4)) + ((locals.var_iben_t_dn4 * assign3550_e4048) + (locals.var_iben_t * locals.var_expn_dn4)))), (assign3550_e4039 * (locals.var_ibei_t * locals.var_expi_dn5)), (assign3550_e4039 * ((locals.var_ibei_t * locals.var_expi_dn6) + (locals.var_iben_t * locals.var_expn_dn6))), (assign3550_e4039 * ((locals.var_ibei_t * locals.var_expi_dn7) + (locals.var_iben_t * locals.var_expn_dn7))), (assign3550_e4039 * ((locals.var_ibei_t * locals.var_expi_dn8) + (locals.var_iben_t * locals.var_expn_dn8))), (assign3550_e4039 * ((locals.var_ibei_t * locals.var_expi_dn9) + (locals.var_iben_t * locals.var_expn_dn9))), (assign3550_e4039 * ((locals.var_ibei_t * locals.var_expi_dn10) + (locals.var_iben_t * locals.var_expn_dn10))), (assign3550_e4039 * ((locals.var_ibei_t * locals.var_expi_dn11) + (locals.var_iben_t * locals.var_expn_dn11))),)
    } else {
        (locals.var_ibex, locals.var_ibex_dn4, locals.var_ibex_dn5, locals.var_ibex_dn6, locals.var_ibex_dn7, locals.var_ibex_dn8, locals.var_ibex_dn9, locals.var_ibex_dn10, locals.var_ibex_dn11,)
    }
};
        locals.var_ibex = assign3550_e4053;
        locals.var_ibex_dn4 = assign3550_e4053_d_n4;
        locals.var_ibex_dn5 = assign3550_e4053_d_n5;
        locals.var_ibex_dn6 = assign3550_e4053_d_n6;
        locals.var_ibex_dn7 = assign3550_e4053_d_n7;
        locals.var_ibex_dn8 = assign3550_e4053_d_n8;
        locals.var_ibex_dn9 = assign3550_e4053_d_n9;
        locals.var_ibex_dn10 = assign3550_e4053_d_n10;
        locals.var_ibex_dn11 = assign3550_e4053_d_n11;

        let assign3560_e4056: f64 = if p.p88 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign3560_e4056;

        let (assign3570_e4069, assign3570_e4069_d_n4, assign3570_e4069_d_n8, assign3570_e4069_d_n9,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) {
        let assign3570_e4065: f64 = (-locals.var_vbbe_t);
        let assign3570_e4067: f64 = (assign3570_e4065 - locals.var_vbei);
        (assign3570_e4067, (-locals.var_vbbe_t_dn4), (-locals.var_vbei_dn8), (-locals.var_vbei_dn9),)
    } else {
        (locals.var_bvbe, locals.var_bvbe_dn4, locals.var_bvbe_dn8, locals.var_bvbe_dn9,)
    }
};
        locals.var_bvbe = assign3570_e4069;
        locals.var_bvbe_dn4 = assign3570_e4069_d_n4;
        locals.var_bvbe_dn8 = assign3570_e4069_d_n8;
        locals.var_bvbe_dn9 = assign3570_e4069_d_n9;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3580_e4083, assign3580_e4083_d_n4,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) {
        let assign3580_e4080: f64 = (locals.var_nbbe_t * locals.var_vtv);
        let assign3580_e4081: f64 = (1.0 / assign3580_e4080);
        (assign3580_e4081, (-(((locals.var_nbbe_t_dn4 * locals.var_vtv) + (locals.var_nbbe_t * locals.var_vtv_dn4)) / (assign3580_e4080 * assign3580_e4080))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3580_e4083;
        locals.var_afac_dn4 = assign3580_e4083_d_n4;

        let assign3590_e4086: f64 = if locals.var_bvbe < locals.var_maxvibbe { 1.0 } else { 0.0 };
        locals.var_guard108 = assign3590_e4086;

        let (assign3600_e4101, assign3600_e4101_d_n4, assign3600_e4101_d_n5, assign3600_e4101_d_n6, assign3600_e4101_d_n7, assign3600_e4101_d_n8, assign3600_e4101_d_n9, assign3600_e4101_d_n10, assign3600_e4101_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 != 0.0)) {
        let assign3600_e4098: f64 = (locals.var_bvbe * locals.var_afac);
        let assign3600_e4099: f64 = (assign3600_e4098).exp();
        (assign3600_e4099, (assign3600_e4099 * ((locals.var_bvbe_dn4 * locals.var_afac) + (locals.var_bvbe * locals.var_afac_dn4))), 0.0, 0.0, 0.0, (assign3600_e4099 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3600_e4099 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3600_e4101;
        locals.var_expx_dn4 = assign3600_e4101_d_n4;
        locals.var_expx_dn5 = assign3600_e4101_d_n5;
        locals.var_expx_dn6 = assign3600_e4101_d_n6;
        locals.var_expx_dn7 = assign3600_e4101_d_n7;
        locals.var_expx_dn8 = assign3600_e4101_d_n8;
        locals.var_expx_dn9 = assign3600_e4101_d_n9;
        locals.var_expx_dn10 = assign3600_e4101_d_n10;
        locals.var_expx_dn11 = assign3600_e4101_d_n11;

        let (assign3610_e4125, assign3610_e4125_d_n4, assign3610_e4125_d_n5, assign3610_e4125_d_n6, assign3610_e4125_d_n7, assign3610_e4125_d_n8, assign3610_e4125_d_n9, assign3610_e4125_d_n10, assign3610_e4125_d_n11,) = {
    if ((((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) && (locals.var_guard108 == 0.0)) {
        let assign3610_e4114: f64 = (locals.var_maxvibbe * locals.var_afac);
        let assign3610_e4115: f64 = (assign3610_e4114).exp();
        let assign3610_e4119: f64 = (locals.var_bvbe - locals.var_maxvibbe);
        let assign3610_e4121: f64 = (assign3610_e4119 * locals.var_afac);
        let assign3610_e4122: f64 = (1.0 + assign3610_e4121);
        let assign3610_e4123: f64 = (assign3610_e4115 * assign3610_e4122);
        (assign3610_e4123, (((assign3610_e4115 * ((locals.var_maxvibbe_dn4 * locals.var_afac) + (locals.var_maxvibbe * locals.var_afac_dn4))) * assign3610_e4122) + (assign3610_e4115 * (((locals.var_bvbe_dn4 - locals.var_maxvibbe_dn4) * locals.var_afac) + (assign3610_e4119 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, (assign3610_e4115 * (locals.var_bvbe_dn8 * locals.var_afac)), (assign3610_e4115 * (locals.var_bvbe_dn9 * locals.var_afac)), 0.0, 0.0,)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3610_e4125;
        locals.var_expx_dn4 = assign3610_e4125_d_n4;
        locals.var_expx_dn5 = assign3610_e4125_d_n5;
        locals.var_expx_dn6 = assign3610_e4125_d_n6;
        locals.var_expx_dn7 = assign3610_e4125_d_n7;
        locals.var_expx_dn8 = assign3610_e4125_d_n8;
        locals.var_expx_dn9 = assign3610_e4125_d_n9;
        locals.var_expx_dn10 = assign3610_e4125_d_n10;
        locals.var_expx_dn11 = assign3610_e4125_d_n11;

        let (assign3620_e4145, assign3620_e4145_d_n4, assign3620_e4145_d_n5, assign3620_e4145_d_n6, assign3620_e4145_d_n7, assign3620_e4145_d_n8, assign3620_e4145_d_n9, assign3620_e4145_d_n10, assign3620_e4145_d_n11,) = {
    if (((locals.var_guard89 == 0.0) && (locals.var_guard95 == 0.0)) && (locals.var_guard107 != 0.0)) {
        let assign3620_e4136: f64 = (1.0 - p.p55);
        let assign3620_e4138: f64 = (assign3620_e4136 * p.p90);
        let assign3620_e4141: f64 = (locals.var_expx - locals.var_ebbe_t);
        let assign3620_e4142: f64 = (assign3620_e4138 * assign3620_e4141);
        let assign3620_e4143: f64 = (locals.var_ibex - assign3620_e4142);
        (assign3620_e4143, (locals.var_ibex_dn4 - (assign3620_e4138 * (locals.var_expx_dn4 - locals.var_ebbe_t_dn4))), (locals.var_ibex_dn5 - (assign3620_e4138 * locals.var_expx_dn5)), (locals.var_ibex_dn6 - (assign3620_e4138 * locals.var_expx_dn6)), (locals.var_ibex_dn7 - (assign3620_e4138 * locals.var_expx_dn7)), (locals.var_ibex_dn8 - (assign3620_e4138 * locals.var_expx_dn8)), (locals.var_ibex_dn9 - (assign3620_e4138 * locals.var_expx_dn9)), (locals.var_ibex_dn10 - (assign3620_e4138 * locals.var_expx_dn10)), (locals.var_ibex_dn11 - (assign3620_e4138 * locals.var_expx_dn11)),)
    } else {
        (locals.var_ibex, locals.var_ibex_dn4, locals.var_ibex_dn5, locals.var_ibex_dn6, locals.var_ibex_dn7, locals.var_ibex_dn8, locals.var_ibex_dn9, locals.var_ibex_dn10, locals.var_ibex_dn11,)
    }
};
        locals.var_ibex = assign3620_e4145;
        locals.var_ibex_dn4 = assign3620_e4145_d_n4;
        locals.var_ibex_dn5 = assign3620_e4145_d_n5;
        locals.var_ibex_dn6 = assign3620_e4145_d_n6;
        locals.var_ibex_dn7 = assign3620_e4145_d_n7;
        locals.var_ibex_dn8 = assign3620_e4145_d_n8;
        locals.var_ibex_dn9 = assign3620_e4145_d_n9;
        locals.var_ibex_dn10 = assign3620_e4145_d_n10;
        locals.var_ibex_dn11 = assign3620_e4145_d_n11;

        let assign3630_e4149: f64 = (p.p61 * locals.var_vtv);
        let assign3630_e4150: f64 = (1.0 / assign3630_e4149);
        locals.var_afac = assign3630_e4150;
        locals.var_afac_dn4 = (-((p.p61 * locals.var_vtv_dn4) / (assign3630_e4149 * assign3630_e4149)));

        let assign3640_e4153: f64 = if locals.var_vbci < locals.var_maxvibci { 1.0 } else { 0.0 };
        locals.var_guard109 = assign3640_e4153;

        let (assign3650_e4160, assign3650_e4160_d_n4, assign3650_e4160_d_n5, assign3650_e4160_d_n6, assign3650_e4160_d_n7, assign3650_e4160_d_n8, assign3650_e4160_d_n9, assign3650_e4160_d_n10, assign3650_e4160_d_n11,) = {
    if (locals.var_guard109 != 0.0) {
        let assign3650_e4157: f64 = (locals.var_vbci * locals.var_afac);
        let assign3650_e4158: f64 = (assign3650_e4157).exp();
        (assign3650_e4158, (assign3650_e4158 * (locals.var_vbci * locals.var_afac_dn4)), 0.0, (assign3650_e4158 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign3650_e4158 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3650_e4160;
        locals.var_expi_dn4 = assign3650_e4160_d_n4;
        locals.var_expi_dn5 = assign3650_e4160_d_n5;
        locals.var_expi_dn6 = assign3650_e4160_d_n6;
        locals.var_expi_dn7 = assign3650_e4160_d_n7;
        locals.var_expi_dn8 = assign3650_e4160_d_n8;
        locals.var_expi_dn9 = assign3650_e4160_d_n9;
        locals.var_expi_dn10 = assign3650_e4160_d_n10;
        locals.var_expi_dn11 = assign3650_e4160_d_n11;

        let (assign3660_e4176, assign3660_e4176_d_n4, assign3660_e4176_d_n5, assign3660_e4176_d_n6, assign3660_e4176_d_n7, assign3660_e4176_d_n8, assign3660_e4176_d_n9, assign3660_e4176_d_n10, assign3660_e4176_d_n11,) = {
    if (locals.var_guard109 == 0.0) {
        let assign3660_e4165: f64 = (locals.var_maxvibci * locals.var_afac);
        let assign3660_e4166: f64 = (assign3660_e4165).exp();
        let assign3660_e4170: f64 = (locals.var_vbci - locals.var_maxvibci);
        let assign3660_e4172: f64 = (assign3660_e4170 * locals.var_afac);
        let assign3660_e4173: f64 = (1.0 + assign3660_e4172);
        let assign3660_e4174: f64 = (assign3660_e4166 * assign3660_e4173);
        (assign3660_e4174, (((assign3660_e4166 * ((locals.var_maxvibci_dn4 * locals.var_afac) + (locals.var_maxvibci * locals.var_afac_dn4))) * assign3660_e4173) + (assign3660_e4166 * (((-locals.var_maxvibci_dn4) * locals.var_afac) + (assign3660_e4170 * locals.var_afac_dn4)))), 0.0, (assign3660_e4166 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign3660_e4166 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3660_e4176;
        locals.var_expi_dn4 = assign3660_e4176_d_n4;
        locals.var_expi_dn5 = assign3660_e4176_d_n5;
        locals.var_expi_dn6 = assign3660_e4176_d_n6;
        locals.var_expi_dn7 = assign3660_e4176_d_n7;
        locals.var_expi_dn8 = assign3660_e4176_d_n8;
        locals.var_expi_dn9 = assign3660_e4176_d_n9;
        locals.var_expi_dn10 = assign3660_e4176_d_n10;
        locals.var_expi_dn11 = assign3660_e4176_d_n11;

        let assign3670_e4180: f64 = (p.p63 * locals.var_vtv);
        let assign3670_e4181: f64 = (1.0 / assign3670_e4180);
        locals.var_afac = assign3670_e4181;
        locals.var_afac_dn4 = (-((p.p63 * locals.var_vtv_dn4) / (assign3670_e4180 * assign3670_e4180)));

        let assign3680_e4184: f64 = if locals.var_vbci < locals.var_maxvibcn { 1.0 } else { 0.0 };
        locals.var_guard110 = assign3680_e4184;

        let (assign3690_e4191, assign3690_e4191_d_n4, assign3690_e4191_d_n6, assign3690_e4191_d_n7, assign3690_e4191_d_n8, assign3690_e4191_d_n9, assign3690_e4191_d_n10, assign3690_e4191_d_n11,) = {
    if (locals.var_guard110 != 0.0) {
        let assign3690_e4188: f64 = (locals.var_vbci * locals.var_afac);
        let assign3690_e4189: f64 = (assign3690_e4188).exp();
        (assign3690_e4189, (assign3690_e4189 * (locals.var_vbci * locals.var_afac_dn4)), (assign3690_e4189 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign3690_e4189 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3690_e4191;
        locals.var_expn_dn4 = assign3690_e4191_d_n4;
        locals.var_expn_dn6 = assign3690_e4191_d_n6;
        locals.var_expn_dn7 = assign3690_e4191_d_n7;
        locals.var_expn_dn8 = assign3690_e4191_d_n8;
        locals.var_expn_dn9 = assign3690_e4191_d_n9;
        locals.var_expn_dn10 = assign3690_e4191_d_n10;
        locals.var_expn_dn11 = assign3690_e4191_d_n11;

        let (assign3700_e4207, assign3700_e4207_d_n4, assign3700_e4207_d_n6, assign3700_e4207_d_n7, assign3700_e4207_d_n8, assign3700_e4207_d_n9, assign3700_e4207_d_n10, assign3700_e4207_d_n11,) = {
    if (locals.var_guard110 == 0.0) {
        let assign3700_e4196: f64 = (locals.var_maxvibcn * locals.var_afac);
        let assign3700_e4197: f64 = (assign3700_e4196).exp();
        let assign3700_e4201: f64 = (locals.var_vbci - locals.var_maxvibcn);
        let assign3700_e4203: f64 = (assign3700_e4201 * locals.var_afac);
        let assign3700_e4204: f64 = (1.0 + assign3700_e4203);
        let assign3700_e4205: f64 = (assign3700_e4197 * assign3700_e4204);
        (assign3700_e4205, (((assign3700_e4197 * ((locals.var_maxvibcn_dn4 * locals.var_afac) + (locals.var_maxvibcn * locals.var_afac_dn4))) * assign3700_e4204) + (assign3700_e4197 * (((-locals.var_maxvibcn_dn4) * locals.var_afac) + (assign3700_e4201 * locals.var_afac_dn4)))), (assign3700_e4197 * (locals.var_vbci_dn6 * locals.var_afac)), 0.0, (assign3700_e4197 * (locals.var_vbci_dn8 * locals.var_afac)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3700_e4207;
        locals.var_expn_dn4 = assign3700_e4207_d_n4;
        locals.var_expn_dn6 = assign3700_e4207_d_n6;
        locals.var_expn_dn7 = assign3700_e4207_d_n7;
        locals.var_expn_dn8 = assign3700_e4207_d_n8;
        locals.var_expn_dn9 = assign3700_e4207_d_n9;
        locals.var_expn_dn10 = assign3700_e4207_d_n10;
        locals.var_expn_dn11 = assign3700_e4207_d_n11;

        let assign3710_e4211: f64 = (locals.var_expi - 1.0);
        let assign3710_e4212: f64 = (locals.var_ibci_t * assign3710_e4211);
        let assign3710_e4216: f64 = (locals.var_expn - 1.0);
        let assign3710_e4217: f64 = (locals.var_ibcn_t * assign3710_e4216);
        let assign3710_e4218: f64 = (assign3710_e4212 + assign3710_e4217);
        locals.var_ibcj = assign3710_e4218;
        locals.var_ibcj_dn4 = (((locals.var_ibci_t_dn4 * assign3710_e4211) + (locals.var_ibci_t * locals.var_expi_dn4)) + ((locals.var_ibcn_t_dn4 * assign3710_e4216) + (locals.var_ibcn_t * locals.var_expn_dn4)));
        locals.var_ibcj_dn5 = (locals.var_ibci_t * locals.var_expi_dn5);
        locals.var_ibcj_dn6 = ((locals.var_ibci_t * locals.var_expi_dn6) + (locals.var_ibcn_t * locals.var_expn_dn6));
        locals.var_ibcj_dn7 = ((locals.var_ibci_t * locals.var_expi_dn7) + (locals.var_ibcn_t * locals.var_expn_dn7));
        locals.var_ibcj_dn8 = ((locals.var_ibci_t * locals.var_expi_dn8) + (locals.var_ibcn_t * locals.var_expn_dn8));
        locals.var_ibcj_dn9 = ((locals.var_ibci_t * locals.var_expi_dn9) + (locals.var_ibcn_t * locals.var_expn_dn9));
        locals.var_ibcj_dn10 = ((locals.var_ibci_t * locals.var_expi_dn10) + (locals.var_ibcn_t * locals.var_expn_dn10));
        locals.var_ibcj_dn11 = ((locals.var_ibci_t * locals.var_expi_dn11) + (locals.var_ibcn_t * locals.var_expn_dn11));

        let assign3720_e4225: f64 = if ((p.p64 > 0.0) || (p.p65 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard111 = assign3720_e4225;

        let (assign3730_e4233, assign3730_e4233_d_n4,) = {
    if (locals.var_guard111 != 0.0) {
        let assign3730_e4230: f64 = (p.p61 * locals.var_vtv);
        let assign3730_e4231: f64 = (1.0 / assign3730_e4230);
        (assign3730_e4231, (-((p.p61 * locals.var_vtv_dn4) / (assign3730_e4230 * assign3730_e4230))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3730_e4233;
        locals.var_afac_dn4 = assign3730_e4233_d_n4;

        let assign3740_e4236: f64 = if locals.var_vbep < locals.var_maxvibeip { 1.0 } else { 0.0 };
        locals.var_guard112 = assign3740_e4236;

        let (assign3750_e4245, assign3750_e4245_d_n4, assign3750_e4245_d_n5, assign3750_e4245_d_n6, assign3750_e4245_d_n7, assign3750_e4245_d_n8, assign3750_e4245_d_n9, assign3750_e4245_d_n10, assign3750_e4245_d_n11,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard112 != 0.0)) {
        let assign3750_e4242: f64 = (locals.var_vbep * locals.var_afac);
        let assign3750_e4243: f64 = (assign3750_e4242).exp();
        (assign3750_e4243, (assign3750_e4243 * (locals.var_vbep * locals.var_afac_dn4)), 0.0, 0.0, (assign3750_e4243 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign3750_e4243 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3750_e4245;
        locals.var_expi_dn4 = assign3750_e4245_d_n4;
        locals.var_expi_dn5 = assign3750_e4245_d_n5;
        locals.var_expi_dn6 = assign3750_e4245_d_n6;
        locals.var_expi_dn7 = assign3750_e4245_d_n7;
        locals.var_expi_dn8 = assign3750_e4245_d_n8;
        locals.var_expi_dn9 = assign3750_e4245_d_n9;
        locals.var_expi_dn10 = assign3750_e4245_d_n10;
        locals.var_expi_dn11 = assign3750_e4245_d_n11;

        let (assign3760_e4263, assign3760_e4263_d_n4, assign3760_e4263_d_n5, assign3760_e4263_d_n6, assign3760_e4263_d_n7, assign3760_e4263_d_n8, assign3760_e4263_d_n9, assign3760_e4263_d_n10, assign3760_e4263_d_n11,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard112 == 0.0)) {
        let assign3760_e4252: f64 = (locals.var_maxvibeip * locals.var_afac);
        let assign3760_e4253: f64 = (assign3760_e4252).exp();
        let assign3760_e4257: f64 = (locals.var_vbep - locals.var_maxvibeip);
        let assign3760_e4259: f64 = (assign3760_e4257 * locals.var_afac);
        let assign3760_e4260: f64 = (1.0 + assign3760_e4259);
        let assign3760_e4261: f64 = (assign3760_e4253 * assign3760_e4260);
        (assign3760_e4261, (((assign3760_e4253 * ((locals.var_maxvibeip_dn4 * locals.var_afac) + (locals.var_maxvibeip * locals.var_afac_dn4))) * assign3760_e4260) + (assign3760_e4253 * (((-locals.var_maxvibeip_dn4) * locals.var_afac) + (assign3760_e4257 * locals.var_afac_dn4)))), 0.0, 0.0, (assign3760_e4253 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign3760_e4253 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3760_e4263;
        locals.var_expi_dn4 = assign3760_e4263_d_n4;
        locals.var_expi_dn5 = assign3760_e4263_d_n5;
        locals.var_expi_dn6 = assign3760_e4263_d_n6;
        locals.var_expi_dn7 = assign3760_e4263_d_n7;
        locals.var_expi_dn8 = assign3760_e4263_d_n8;
        locals.var_expi_dn9 = assign3760_e4263_d_n9;
        locals.var_expi_dn10 = assign3760_e4263_d_n10;
        locals.var_expi_dn11 = assign3760_e4263_d_n11;

        let (assign3770_e4271, assign3770_e4271_d_n4,) = {
    if (locals.var_guard111 != 0.0) {
        let assign3770_e4268: f64 = (p.p63 * locals.var_vtv);
        let assign3770_e4269: f64 = (1.0 / assign3770_e4268);
        (assign3770_e4269, (-((p.p63 * locals.var_vtv_dn4) / (assign3770_e4268 * assign3770_e4268))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign3770_e4271;
        locals.var_afac_dn4 = assign3770_e4271_d_n4;

        let assign3780_e4274: f64 = if locals.var_vbep < locals.var_maxvibenp { 1.0 } else { 0.0 };
        locals.var_guard113 = assign3780_e4274;

        let (assign3790_e4283, assign3790_e4283_d_n4, assign3790_e4283_d_n6, assign3790_e4283_d_n7, assign3790_e4283_d_n8, assign3790_e4283_d_n9, assign3790_e4283_d_n10, assign3790_e4283_d_n11,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard113 != 0.0)) {
        let assign3790_e4280: f64 = (locals.var_vbep * locals.var_afac);
        let assign3790_e4281: f64 = (assign3790_e4280).exp();
        (assign3790_e4281, (assign3790_e4281 * (locals.var_vbep * locals.var_afac_dn4)), 0.0, (assign3790_e4281 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign3790_e4281 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3790_e4283;
        locals.var_expn_dn4 = assign3790_e4283_d_n4;
        locals.var_expn_dn6 = assign3790_e4283_d_n6;
        locals.var_expn_dn7 = assign3790_e4283_d_n7;
        locals.var_expn_dn8 = assign3790_e4283_d_n8;
        locals.var_expn_dn9 = assign3790_e4283_d_n9;
        locals.var_expn_dn10 = assign3790_e4283_d_n10;
        locals.var_expn_dn11 = assign3790_e4283_d_n11;

        let (assign3800_e4301, assign3800_e4301_d_n4, assign3800_e4301_d_n6, assign3800_e4301_d_n7, assign3800_e4301_d_n8, assign3800_e4301_d_n9, assign3800_e4301_d_n10, assign3800_e4301_d_n11,) = {
    if ((locals.var_guard111 != 0.0) && (locals.var_guard113 == 0.0)) {
        let assign3800_e4290: f64 = (locals.var_maxvibenp * locals.var_afac);
        let assign3800_e4291: f64 = (assign3800_e4290).exp();
        let assign3800_e4295: f64 = (locals.var_vbep - locals.var_maxvibenp);
        let assign3800_e4297: f64 = (assign3800_e4295 * locals.var_afac);
        let assign3800_e4298: f64 = (1.0 + assign3800_e4297);
        let assign3800_e4299: f64 = (assign3800_e4291 * assign3800_e4298);
        (assign3800_e4299, (((assign3800_e4291 * ((locals.var_maxvibenp_dn4 * locals.var_afac) + (locals.var_maxvibenp * locals.var_afac_dn4))) * assign3800_e4298) + (assign3800_e4291 * (((-locals.var_maxvibenp_dn4) * locals.var_afac) + (assign3800_e4295 * locals.var_afac_dn4)))), 0.0, (assign3800_e4291 * (locals.var_vbep_dn7 * locals.var_afac)), 0.0, 0.0, (assign3800_e4291 * (locals.var_vbep_dn10 * locals.var_afac)), 0.0,)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign3800_e4301;
        locals.var_expn_dn4 = assign3800_e4301_d_n4;
        locals.var_expn_dn6 = assign3800_e4301_d_n6;
        locals.var_expn_dn7 = assign3800_e4301_d_n7;
        locals.var_expn_dn8 = assign3800_e4301_d_n8;
        locals.var_expn_dn9 = assign3800_e4301_d_n9;
        locals.var_expn_dn10 = assign3800_e4301_d_n10;
        locals.var_expn_dn11 = assign3800_e4301_d_n11;

        let (assign3810_e4315, assign3810_e4315_d_n4, assign3810_e4315_d_n5, assign3810_e4315_d_n6, assign3810_e4315_d_n7, assign3810_e4315_d_n8, assign3810_e4315_d_n9, assign3810_e4315_d_n10, assign3810_e4315_d_n11,) = {
    if (locals.var_guard111 != 0.0) {
        let assign3810_e4306: f64 = (locals.var_expi - 1.0);
        let assign3810_e4307: f64 = (locals.var_ibeip_t * assign3810_e4306);
        let assign3810_e4311: f64 = (locals.var_expn - 1.0);
        let assign3810_e4312: f64 = (locals.var_ibenp_t * assign3810_e4311);
        let assign3810_e4313: f64 = (assign3810_e4307 + assign3810_e4312);
        (assign3810_e4313, (((locals.var_ibeip_t_dn4 * assign3810_e4306) + (locals.var_ibeip_t * locals.var_expi_dn4)) + ((locals.var_ibenp_t_dn4 * assign3810_e4311) + (locals.var_ibenp_t * locals.var_expn_dn4))), (locals.var_ibeip_t * locals.var_expi_dn5), ((locals.var_ibeip_t * locals.var_expi_dn6) + (locals.var_ibenp_t * locals.var_expn_dn6)), ((locals.var_ibeip_t * locals.var_expi_dn7) + (locals.var_ibenp_t * locals.var_expn_dn7)), ((locals.var_ibeip_t * locals.var_expi_dn8) + (locals.var_ibenp_t * locals.var_expn_dn8)), ((locals.var_ibeip_t * locals.var_expi_dn9) + (locals.var_ibenp_t * locals.var_expn_dn9)), ((locals.var_ibeip_t * locals.var_expi_dn10) + (locals.var_ibenp_t * locals.var_expn_dn10)), ((locals.var_ibeip_t * locals.var_expi_dn11) + (locals.var_ibenp_t * locals.var_expn_dn11)),)
    } else {
        (locals.var_ibep, locals.var_ibep_dn4, locals.var_ibep_dn5, locals.var_ibep_dn6, locals.var_ibep_dn7, locals.var_ibep_dn8, locals.var_ibep_dn9, locals.var_ibep_dn10, locals.var_ibep_dn11,)
    }
};
        locals.var_ibep = assign3810_e4315;
        locals.var_ibep_dn4 = assign3810_e4315_d_n4;
        locals.var_ibep_dn5 = assign3810_e4315_d_n5;
        locals.var_ibep_dn6 = assign3810_e4315_d_n6;
        locals.var_ibep_dn7 = assign3810_e4315_d_n7;
        locals.var_ibep_dn8 = assign3810_e4315_d_n8;
        locals.var_ibep_dn9 = assign3810_e4315_d_n9;
        locals.var_ibep_dn10 = assign3810_e4315_d_n10;
        locals.var_ibep_dn11 = assign3810_e4315_d_n11;

        let (assign3820_e4320, assign3820_e4320_d_n4, assign3820_e4320_d_n5, assign3820_e4320_d_n6, assign3820_e4320_d_n7, assign3820_e4320_d_n8, assign3820_e4320_d_n9, assign3820_e4320_d_n10, assign3820_e4320_d_n11,) = {
    if (locals.var_guard111 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibep, locals.var_ibep_dn4, locals.var_ibep_dn5, locals.var_ibep_dn6, locals.var_ibep_dn7, locals.var_ibep_dn8, locals.var_ibep_dn9, locals.var_ibep_dn10, locals.var_ibep_dn11,)
    }
};
        locals.var_ibep = assign3820_e4320;
        locals.var_ibep_dn4 = assign3820_e4320_d_n4;
        locals.var_ibep_dn5 = assign3820_e4320_d_n5;
        locals.var_ibep_dn6 = assign3820_e4320_d_n6;
        locals.var_ibep_dn7 = assign3820_e4320_d_n7;
        locals.var_ibep_dn8 = assign3820_e4320_d_n8;
        locals.var_ibep_dn9 = assign3820_e4320_d_n9;
        locals.var_ibep_dn10 = assign3820_e4320_d_n10;
        locals.var_ibep_dn11 = assign3820_e4320_d_n11;

        let assign3830_e4323: f64 = (locals.var_vbci / locals.var_vtv);
        locals.var_arg = assign3830_e4323;
        locals.var_arg_dn4 = (-((locals.var_vbci * locals.var_vtv_dn4) / (locals.var_vtv * locals.var_vtv)));
        locals.var_arg_dn5 = 0.0;
        locals.var_arg_dn6 = (locals.var_vbci_dn6 / locals.var_vtv);
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = (locals.var_vbci_dn8 / locals.var_vtv);
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;

        let assign3840_e4326: f64 = if locals.var_arg < locals.var_vmaxexp { 1.0 } else { 0.0 };
        locals.var_guard114 = assign3840_e4326;

        let (assign3850_e4331, assign3850_e4331_d_n4, assign3850_e4331_d_n5, assign3850_e4331_d_n6, assign3850_e4331_d_n7, assign3850_e4331_d_n8, assign3850_e4331_d_n9, assign3850_e4331_d_n10, assign3850_e4331_d_n11,) = {
    if (locals.var_guard114 != 0.0) {
        let assign3850_e4329: f64 = (locals.var_arg).exp();
        (assign3850_e4329, (assign3850_e4329 * locals.var_arg_dn4), (assign3850_e4329 * locals.var_arg_dn5), (assign3850_e4329 * locals.var_arg_dn6), (assign3850_e4329 * locals.var_arg_dn7), (assign3850_e4329 * locals.var_arg_dn8), (assign3850_e4329 * locals.var_arg_dn9), (assign3850_e4329 * locals.var_arg_dn10), (assign3850_e4329 * locals.var_arg_dn11),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3850_e4331;
        locals.var_expi_dn4 = assign3850_e4331_d_n4;
        locals.var_expi_dn5 = assign3850_e4331_d_n5;
        locals.var_expi_dn6 = assign3850_e4331_d_n6;
        locals.var_expi_dn7 = assign3850_e4331_d_n7;
        locals.var_expi_dn8 = assign3850_e4331_d_n8;
        locals.var_expi_dn9 = assign3850_e4331_d_n9;
        locals.var_expi_dn10 = assign3850_e4331_d_n10;
        locals.var_expi_dn11 = assign3850_e4331_d_n11;

        let (assign3860_e4343, assign3860_e4343_d_n4, assign3860_e4343_d_n5, assign3860_e4343_d_n6, assign3860_e4343_d_n7, assign3860_e4343_d_n8, assign3860_e4343_d_n9, assign3860_e4343_d_n10, assign3860_e4343_d_n11,) = {
    if (locals.var_guard114 == 0.0) {
        let assign3860_e4335: f64 = (locals.var_vmaxexp).exp();
        let assign3860_e4339: f64 = (locals.var_arg - locals.var_vmaxexp);
        let assign3860_e4340: f64 = (1.0 + assign3860_e4339);
        let assign3860_e4341: f64 = (assign3860_e4335 * assign3860_e4340);
        (assign3860_e4341, (assign3860_e4335 * locals.var_arg_dn4), (assign3860_e4335 * locals.var_arg_dn5), (assign3860_e4335 * locals.var_arg_dn6), (assign3860_e4335 * locals.var_arg_dn7), (assign3860_e4335 * locals.var_arg_dn8), (assign3860_e4335 * locals.var_arg_dn9), (assign3860_e4335 * locals.var_arg_dn10), (assign3860_e4335 * locals.var_arg_dn11),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign3860_e4343;
        locals.var_expi_dn4 = assign3860_e4343_d_n4;
        locals.var_expi_dn5 = assign3860_e4343_d_n5;
        locals.var_expi_dn6 = assign3860_e4343_d_n6;
        locals.var_expi_dn7 = assign3860_e4343_d_n7;
        locals.var_expi_dn8 = assign3860_e4343_d_n8;
        locals.var_expi_dn9 = assign3860_e4343_d_n9;
        locals.var_expi_dn10 = assign3860_e4343_d_n10;
        locals.var_expi_dn11 = assign3860_e4343_d_n11;

        let assign3870_e4346: f64 = (locals.var_vbcx / locals.var_vtv);
        locals.var_arg = assign3870_e4346;
        locals.var_arg_dn4 = (-((locals.var_vbcx * locals.var_vtv_dn4) / (locals.var_vtv * locals.var_vtv)));
        locals.var_arg_dn5 = (locals.var_vbcx_dn5 / locals.var_vtv);
        locals.var_arg_dn6 = 0.0;
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = (locals.var_vbcx_dn8 / locals.var_vtv);
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;

        let assign3880_e4349: f64 = if locals.var_arg < locals.var_vmaxexp { 1.0 } else { 0.0 };
        locals.var_guard115 = assign3880_e4349;

        let (assign3890_e4354, assign3890_e4354_d_n4, assign3890_e4354_d_n5, assign3890_e4354_d_n6, assign3890_e4354_d_n7, assign3890_e4354_d_n8, assign3890_e4354_d_n9, assign3890_e4354_d_n10, assign3890_e4354_d_n11,) = {
    if (locals.var_guard115 != 0.0) {
        let assign3890_e4352: f64 = (locals.var_arg).exp();
        (assign3890_e4352, (assign3890_e4352 * locals.var_arg_dn4), (assign3890_e4352 * locals.var_arg_dn5), (assign3890_e4352 * locals.var_arg_dn6), (assign3890_e4352 * locals.var_arg_dn7), (assign3890_e4352 * locals.var_arg_dn8), (assign3890_e4352 * locals.var_arg_dn9), (assign3890_e4352 * locals.var_arg_dn10), (assign3890_e4352 * locals.var_arg_dn11),)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3890_e4354;
        locals.var_expx_dn4 = assign3890_e4354_d_n4;
        locals.var_expx_dn5 = assign3890_e4354_d_n5;
        locals.var_expx_dn6 = assign3890_e4354_d_n6;
        locals.var_expx_dn7 = assign3890_e4354_d_n7;
        locals.var_expx_dn8 = assign3890_e4354_d_n8;
        locals.var_expx_dn9 = assign3890_e4354_d_n9;
        locals.var_expx_dn10 = assign3890_e4354_d_n10;
        locals.var_expx_dn11 = assign3890_e4354_d_n11;

        let (assign3900_e4366, assign3900_e4366_d_n4, assign3900_e4366_d_n5, assign3900_e4366_d_n6, assign3900_e4366_d_n7, assign3900_e4366_d_n8, assign3900_e4366_d_n9, assign3900_e4366_d_n10, assign3900_e4366_d_n11,) = {
    if (locals.var_guard115 == 0.0) {
        let assign3900_e4358: f64 = (locals.var_vmaxexp).exp();
        let assign3900_e4362: f64 = (locals.var_arg - locals.var_vmaxexp);
        let assign3900_e4363: f64 = (1.0 + assign3900_e4362);
        let assign3900_e4364: f64 = (assign3900_e4358 * assign3900_e4363);
        (assign3900_e4364, (assign3900_e4358 * locals.var_arg_dn4), (assign3900_e4358 * locals.var_arg_dn5), (assign3900_e4358 * locals.var_arg_dn6), (assign3900_e4358 * locals.var_arg_dn7), (assign3900_e4358 * locals.var_arg_dn8), (assign3900_e4358 * locals.var_arg_dn9), (assign3900_e4358 * locals.var_arg_dn10), (assign3900_e4358 * locals.var_arg_dn11),)
    } else {
        (locals.var_expx, locals.var_expx_dn4, locals.var_expx_dn5, locals.var_expx_dn6, locals.var_expx_dn7, locals.var_expx_dn8, locals.var_expx_dn9, locals.var_expx_dn10, locals.var_expx_dn11,)
    }
};
        locals.var_expx = assign3900_e4366;
        locals.var_expx_dn4 = assign3900_e4366_d_n4;
        locals.var_expx_dn5 = assign3900_e4366_d_n5;
        locals.var_expx_dn6 = assign3900_e4366_d_n6;
        locals.var_expx_dn7 = assign3900_e4366_d_n7;
        locals.var_expx_dn8 = assign3900_e4366_d_n8;
        locals.var_expx_dn9 = assign3900_e4366_d_n9;
        locals.var_expx_dn10 = assign3900_e4366_d_n10;
        locals.var_expx_dn11 = assign3900_e4366_d_n11;

        let assign3910_e4370: f64 = (locals.var_gamm_t * locals.var_expi);
        let assign3910_e4371: f64 = (1.0 + assign3910_e4370);
        let assign3910_e4372: f64 = (assign3910_e4371).sqrt();
        locals.var_kbci = assign3910_e4372;
        locals.var_kbci_dn4 = (((locals.var_gamm_t_dn4 * locals.var_expi) + (locals.var_gamm_t * locals.var_expi_dn4)) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn5 = ((locals.var_gamm_t * locals.var_expi_dn5) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn6 = ((locals.var_gamm_t * locals.var_expi_dn6) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn7 = ((locals.var_gamm_t * locals.var_expi_dn7) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn8 = ((locals.var_gamm_t * locals.var_expi_dn8) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn9 = ((locals.var_gamm_t * locals.var_expi_dn9) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn10 = ((locals.var_gamm_t * locals.var_expi_dn10) / (2.0 * assign3910_e4372));
        locals.var_kbci_dn11 = ((locals.var_gamm_t * locals.var_expi_dn11) / (2.0 * assign3910_e4372));

        let assign3920_e4376: f64 = (locals.var_gamm_t * locals.var_expx);
        let assign3920_e4377: f64 = (1.0 + assign3920_e4376);
        let assign3920_e4378: f64 = (assign3920_e4377).sqrt();
        locals.var_kbcx = assign3920_e4378;
        locals.var_kbcx_dn4 = (((locals.var_gamm_t_dn4 * locals.var_expx) + (locals.var_gamm_t * locals.var_expx_dn4)) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn5 = ((locals.var_gamm_t * locals.var_expx_dn5) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn6 = ((locals.var_gamm_t * locals.var_expx_dn6) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn7 = ((locals.var_gamm_t * locals.var_expx_dn7) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn8 = ((locals.var_gamm_t * locals.var_expx_dn8) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn9 = ((locals.var_gamm_t * locals.var_expx_dn9) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn10 = ((locals.var_gamm_t * locals.var_expx_dn10) / (2.0 * assign3920_e4378));
        locals.var_kbcx_dn11 = ((locals.var_gamm_t * locals.var_expx_dn11) / (2.0 * assign3920_e4378));

        let assign3930_e4381: f64 = (locals.var_vrcx * locals.var_gcx);
        locals.var_ircx = assign3930_e4381;
        locals.var_ircx_dn0 = (locals.var_vrcx_dn0 * locals.var_gcx);
        locals.var_ircx_dn4 = (locals.var_vrcx * locals.var_gcx_dn4);
        locals.var_ircx_dn5 = (locals.var_vrcx_dn5 * locals.var_gcx);

        let assign3940_e4384: f64 = (locals.var_kbci + 1.0);
        let assign3940_e4387: f64 = (locals.var_kbcx + 1.0);
        let assign3940_e4388: f64 = (assign3940_e4384 / assign3940_e4387);
        locals.var_rkp1 = assign3940_e4388;
        locals.var_rkp1_dn4 = (((locals.var_kbci_dn4 * assign3940_e4387) - (assign3940_e4384 * locals.var_kbcx_dn4)) / (assign3940_e4387 * assign3940_e4387));
        locals.var_rkp1_dn5 = (((locals.var_kbci_dn5 * assign3940_e4387) - (assign3940_e4384 * locals.var_kbcx_dn5)) / (assign3940_e4387 * assign3940_e4387));
        locals.var_rkp1_dn6 = (((locals.var_kbci_dn6 * assign3940_e4387) - (assign3940_e4384 * locals.var_kbcx_dn6)) / (assign3940_e4387 * assign3940_e4387));
        locals.var_rkp1_dn7 = (((locals.var_kbci_dn7 * assign3940_e4387) - (assign3940_e4384 * locals.var_kbcx_dn7)) / (assign3940_e4387 * assign3940_e4387));
        locals.var_rkp1_dn8 = (((locals.var_kbci_dn8 * assign3940_e4387) - (assign3940_e4384 * locals.var_kbcx_dn8)) / (assign3940_e4387 * assign3940_e4387));
        locals.var_rkp1_dn9 = (((locals.var_kbci_dn9 * assign3940_e4387) - (assign3940_e4384 * locals.var_kbcx_dn9)) / (assign3940_e4387 * assign3940_e4387));
        locals.var_rkp1_dn10 = (((locals.var_kbci_dn10 * assign3940_e4387) - (assign3940_e4384 * locals.var_kbcx_dn10)) / (assign3940_e4387 * assign3940_e4387));
        locals.var_rkp1_dn11 = (((locals.var_kbci_dn11 * assign3940_e4387) - (assign3940_e4384 * locals.var_kbcx_dn11)) / (assign3940_e4387 * assign3940_e4387));

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3950_e4393: f64 = (locals.var_kbci - locals.var_kbcx);
        let assign3950_e4395: f64 = (locals.var_rkp1).ln();
        let assign3950_e4396: f64 = (assign3950_e4393 - assign3950_e4395);
        let assign3950_e4397: f64 = (locals.var_vtv * assign3950_e4396);
        let assign3950_e4398: f64 = (locals.var_vrci + assign3950_e4397);
        let assign3950_e4400: f64 = (assign3950_e4398 * locals.var_gci);
        locals.var_iohm = assign3950_e4400;
        locals.var_iohm_dn4 = ((((locals.var_vtv_dn4 * assign3950_e4396) + (locals.var_vtv * ((locals.var_kbci_dn4 - locals.var_kbcx_dn4) - (locals.var_rkp1_dn4 / locals.var_rkp1)))) * locals.var_gci) + (assign3950_e4398 * locals.var_gci_dn4));
        locals.var_iohm_dn5 = ((locals.var_vrci_dn5 + (locals.var_vtv * ((locals.var_kbci_dn5 - locals.var_kbcx_dn5) - (locals.var_rkp1_dn5 / locals.var_rkp1)))) * locals.var_gci);
        locals.var_iohm_dn6 = ((locals.var_vrci_dn6 + (locals.var_vtv * ((locals.var_kbci_dn6 - locals.var_kbcx_dn6) - (locals.var_rkp1_dn6 / locals.var_rkp1)))) * locals.var_gci);
        locals.var_iohm_dn7 = ((locals.var_vtv * ((locals.var_kbci_dn7 - locals.var_kbcx_dn7) - (locals.var_rkp1_dn7 / locals.var_rkp1))) * locals.var_gci);
        locals.var_iohm_dn8 = ((locals.var_vtv * ((locals.var_kbci_dn8 - locals.var_kbcx_dn8) - (locals.var_rkp1_dn8 / locals.var_rkp1))) * locals.var_gci);
        locals.var_iohm_dn9 = ((locals.var_vtv * ((locals.var_kbci_dn9 - locals.var_kbcx_dn9) - (locals.var_rkp1_dn9 / locals.var_rkp1))) * locals.var_gci);
        locals.var_iohm_dn10 = ((locals.var_vtv * ((locals.var_kbci_dn10 - locals.var_kbcx_dn10) - (locals.var_rkp1_dn10 / locals.var_rkp1))) * locals.var_gci);
        locals.var_iohm_dn11 = ((locals.var_vtv * ((locals.var_kbci_dn11 - locals.var_kbcx_dn11) - (locals.var_rkp1_dn11 / locals.var_rkp1))) * locals.var_gci);

        let assign3960_e4403: f64 = (locals.var_ivo * locals.var_iohm);
        let assign3960_e4408: f64 = (0.5 * locals.var_ivo);
        let assign3960_e4410: f64 = (assign3960_e4408 * locals.var_ihrcf);
        let assign3960_e4413: f64 = (locals.var_vrci * locals.var_vrci);
        let assign3960_e4415: f64 = (assign3960_e4413 + 0.01);
        let assign3960_e4416: f64 = (assign3960_e4415).sqrt();
        let assign3960_e4417: f64 = (assign3960_e4410 * assign3960_e4416);
        let assign3960_e4418: f64 = (1.0 + assign3960_e4417);
        let assign3960_e4419: f64 = (locals.var_gci * assign3960_e4418);
        let assign3960_e4420: f64 = (assign3960_e4403 / assign3960_e4419);
        locals.var_derf = assign3960_e4420;
        locals.var_derf_dn4 = (((((locals.var_ivo_dn4 * locals.var_iohm) + (locals.var_ivo * locals.var_iohm_dn4)) * assign3960_e4419) - (assign3960_e4403 * ((locals.var_gci_dn4 * assign3960_e4418) + (locals.var_gci * (((0.5 * locals.var_ivo_dn4) * locals.var_ihrcf) * assign3960_e4416))))) / (assign3960_e4419 * assign3960_e4419));
        locals.var_derf_dn5 = ((((locals.var_ivo * locals.var_iohm_dn5) * assign3960_e4419) - (assign3960_e4403 * (locals.var_gci * (assign3960_e4410 * (((locals.var_vrci_dn5 * locals.var_vrci) + (locals.var_vrci * locals.var_vrci_dn5)) / (2.0 * assign3960_e4416)))))) / (assign3960_e4419 * assign3960_e4419));
        locals.var_derf_dn6 = ((((locals.var_ivo * locals.var_iohm_dn6) * assign3960_e4419) - (assign3960_e4403 * (locals.var_gci * (assign3960_e4410 * (((locals.var_vrci_dn6 * locals.var_vrci) + (locals.var_vrci * locals.var_vrci_dn6)) / (2.0 * assign3960_e4416)))))) / (assign3960_e4419 * assign3960_e4419));
        locals.var_derf_dn7 = ((locals.var_ivo * locals.var_iohm_dn7) / assign3960_e4419);
        locals.var_derf_dn8 = ((locals.var_ivo * locals.var_iohm_dn8) / assign3960_e4419);
        locals.var_derf_dn9 = ((locals.var_ivo * locals.var_iohm_dn9) / assign3960_e4419);
        locals.var_derf_dn10 = ((locals.var_ivo * locals.var_iohm_dn10) / assign3960_e4419);
        locals.var_derf_dn11 = ((locals.var_ivo * locals.var_iohm_dn11) / assign3960_e4419);

        let assign3970_e4425: f64 = (locals.var_derf * locals.var_derf);
        let assign3970_e4426: f64 = (1.0 + assign3970_e4425);
        let assign3970_e4427: f64 = (assign3970_e4426).sqrt();
        let assign3970_e4428: f64 = (locals.var_iohm / assign3970_e4427);
        locals.var_irci = assign3970_e4428;
        locals.var_irci_dn4 = (((locals.var_iohm_dn4 * assign3970_e4427) - (locals.var_iohm * (((locals.var_derf_dn4 * locals.var_derf) + (locals.var_derf * locals.var_derf_dn4)) / (2.0 * assign3970_e4427)))) / (assign3970_e4427 * assign3970_e4427));
        locals.var_irci_dn5 = (((locals.var_iohm_dn5 * assign3970_e4427) - (locals.var_iohm * (((locals.var_derf_dn5 * locals.var_derf) + (locals.var_derf * locals.var_derf_dn5)) / (2.0 * assign3970_e4427)))) / (assign3970_e4427 * assign3970_e4427));
        locals.var_irci_dn6 = (((locals.var_iohm_dn6 * assign3970_e4427) - (locals.var_iohm * (((locals.var_derf_dn6 * locals.var_derf) + (locals.var_derf * locals.var_derf_dn6)) / (2.0 * assign3970_e4427)))) / (assign3970_e4427 * assign3970_e4427));
        locals.var_irci_dn7 = (((locals.var_iohm_dn7 * assign3970_e4427) - (locals.var_iohm * (((locals.var_derf_dn7 * locals.var_derf) + (locals.var_derf * locals.var_derf_dn7)) / (2.0 * assign3970_e4427)))) / (assign3970_e4427 * assign3970_e4427));
        locals.var_irci_dn8 = (((locals.var_iohm_dn8 * assign3970_e4427) - (locals.var_iohm * (((locals.var_derf_dn8 * locals.var_derf) + (locals.var_derf * locals.var_derf_dn8)) / (2.0 * assign3970_e4427)))) / (assign3970_e4427 * assign3970_e4427));
        locals.var_irci_dn9 = (((locals.var_iohm_dn9 * assign3970_e4427) - (locals.var_iohm * (((locals.var_derf_dn9 * locals.var_derf) + (locals.var_derf * locals.var_derf_dn9)) / (2.0 * assign3970_e4427)))) / (assign3970_e4427 * assign3970_e4427));
        locals.var_irci_dn10 = (((locals.var_iohm_dn10 * assign3970_e4427) - (locals.var_iohm * (((locals.var_derf_dn10 * locals.var_derf) + (locals.var_derf * locals.var_derf_dn10)) / (2.0 * assign3970_e4427)))) / (assign3970_e4427 * assign3970_e4427));
        locals.var_irci_dn11 = (((locals.var_iohm_dn11 * assign3970_e4427) - (locals.var_iohm * (((locals.var_derf_dn11 * locals.var_derf) + (locals.var_derf * locals.var_derf_dn11)) / (2.0 * assign3970_e4427)))) / (assign3970_e4427 * assign3970_e4427));

        let assign3980_e4431: f64 = (locals.var_vrbx * locals.var_gbx);
        locals.var_irbx = assign3980_e4431;
        locals.var_irbx_dn1 = (locals.var_vrbx_dn1 * locals.var_gbx);
        locals.var_irbx_dn4 = (locals.var_vrbx * locals.var_gbx_dn4);
        locals.var_irbx_dn7 = (locals.var_vrbx_dn7 * locals.var_gbx);

        let assign3990_e4434: f64 = (locals.var_vrbi * locals.var_qb);
        let assign3990_e4436: f64 = (assign3990_e4434 * locals.var_gbi);
        locals.var_irbi = assign3990_e4436;
        locals.var_irbi_dn4 = (((locals.var_vrbi * locals.var_qb_dn4) * locals.var_gbi) + (assign3990_e4434 * locals.var_gbi_dn4));
        locals.var_irbi_dn5 = ((locals.var_vrbi * locals.var_qb_dn5) * locals.var_gbi);
        locals.var_irbi_dn6 = ((locals.var_vrbi * locals.var_qb_dn6) * locals.var_gbi);
        locals.var_irbi_dn7 = (((locals.var_vrbi_dn7 * locals.var_qb) + (locals.var_vrbi * locals.var_qb_dn7)) * locals.var_gbi);
        locals.var_irbi_dn8 = (((locals.var_vrbi_dn8 * locals.var_qb) + (locals.var_vrbi * locals.var_qb_dn8)) * locals.var_gbi);
        locals.var_irbi_dn9 = ((locals.var_vrbi * locals.var_qb_dn9) * locals.var_gbi);
        locals.var_irbi_dn10 = ((locals.var_vrbi * locals.var_qb_dn10) * locals.var_gbi);
        locals.var_irbi_dn11 = ((locals.var_vrbi * locals.var_qb_dn11) * locals.var_gbi);

        let assign4000_e4439: f64 = (locals.var_vre * locals.var_ge);
        locals.var_ire = assign4000_e4439;
        locals.var_ire_dn2 = (locals.var_vre_dn2 * locals.var_ge);
        locals.var_ire_dn4 = (locals.var_vre * locals.var_ge_dn4);
        locals.var_ire_dn9 = (locals.var_vre_dn9 * locals.var_ge);

        let assign4010_e4442: f64 = (locals.var_vrbp * locals.var_qbp);
        let assign4010_e4444: f64 = (assign4010_e4442 * locals.var_gbp);
        locals.var_irbp = assign4010_e4444;
        locals.var_irbp_dn4 = (((locals.var_vrbp * locals.var_qbp_dn4) * locals.var_gbp) + (assign4010_e4442 * locals.var_gbp_dn4));
        locals.var_irbp_dn5 = (((locals.var_vrbp_dn5 * locals.var_qbp) + (locals.var_vrbp * locals.var_qbp_dn5)) * locals.var_gbp);
        locals.var_irbp_dn6 = ((locals.var_vrbp * locals.var_qbp_dn6) * locals.var_gbp);
        locals.var_irbp_dn7 = ((locals.var_vrbp * locals.var_qbp_dn7) * locals.var_gbp);
        locals.var_irbp_dn8 = ((locals.var_vrbp * locals.var_qbp_dn8) * locals.var_gbp);
        locals.var_irbp_dn9 = ((locals.var_vrbp * locals.var_qbp_dn9) * locals.var_gbp);
        locals.var_irbp_dn10 = (((locals.var_vrbp_dn10 * locals.var_qbp) + (locals.var_vrbp * locals.var_qbp_dn10)) * locals.var_gbp);
        locals.var_irbp_dn11 = ((locals.var_vrbp * locals.var_qbp_dn11) * locals.var_gbp);

        let assign4020_e4447: f64 = (locals.var_vrs * locals.var_gs);
        locals.var_irs = assign4020_e4447;
        locals.var_irs_dn3 = (locals.var_vrs_dn3 * locals.var_gs);
        locals.var_irs_dn4 = (locals.var_vrs * locals.var_gs_dn4);
        locals.var_irs_dn11 = (locals.var_vrs_dn11 * locals.var_gs);

        let assign4030_e4450: f64 = if p.p83 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign4030_e4450;

        let (assign4040_e4464, assign4040_e4464_d_n4,) = {
    if (locals.var_guard116 != 0.0) {
        let assign4040_e4455: f64 = (locals.var_avc2_t + 1.0);
        let assign4040_e4456: f64 = (0.02 * assign4040_e4455);
        let assign4040_e4460: f64 = (1.01 - p.p43);
        let assign4040_e4461: f64 = (1.0 / assign4040_e4460);
        let assign4040_e4462: f64 = (assign4040_e4456).powf(assign4040_e4461);
        (assign4040_e4462, if 0.0 == 0.0 && ((assign4040_e4461) as f64).is_finite() && ((assign4040_e4461) as f64).fract() == 0.0 { if assign4040_e4461 == 0.0 { 0.0 } else { (assign4040_e4461 * ((assign4040_e4456).powf(assign4040_e4461 - 1.0) * (0.02 * locals.var_avc2_t_dn4))) } } else { (assign4040_e4462 * (assign4040_e4461 * ((0.02 * locals.var_avc2_t_dn4) / assign4040_e4456))) },)
    } else {
        (locals.var_vminm, locals.var_vminm_dn4,)
    }
};
        locals.var_vminm = assign4040_e4464;
        locals.var_vminm_dn4 = assign4040_e4464_d_n4;

        let (assign4050_e4491, assign4050_e4491_d_n4, assign4050_e4491_d_n6, assign4050_e4491_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign4050_e4469: f64 = (locals.var_pc_t - locals.var_vbci);
        let assign4050_e4471: f64 = (assign4050_e4469 - locals.var_vminm);
        let assign4050_e4474: f64 = (locals.var_pc_t - locals.var_vbci);
        let assign4050_e4476: f64 = (assign4050_e4474 - locals.var_vminm);
        let assign4050_e4477: f64 = (assign4050_e4471 * assign4050_e4476);
        let assign4050_e4479: f64 = (assign4050_e4477 + 0.01);
        let assign4050_e4480: f64 = (assign4050_e4479).sqrt();
        let assign4050_e4483: f64 = (locals.var_pc_t - locals.var_vbci);
        let assign4050_e4485: f64 = (assign4050_e4483 - locals.var_vminm);
        let assign4050_e4486: f64 = (assign4050_e4480 + assign4050_e4485);
        let assign4050_e4487: f64 = (0.5 * assign4050_e4486);
        let assign4050_e4489: f64 = (assign4050_e4487 + locals.var_vminm);
        (assign4050_e4489, ((0.5 * (((((locals.var_pc_t_dn4 - locals.var_vminm_dn4) * assign4050_e4476) + (assign4050_e4471 * (locals.var_pc_t_dn4 - locals.var_vminm_dn4))) / (2.0 * assign4050_e4480)) + (locals.var_pc_t_dn4 - locals.var_vminm_dn4))) + locals.var_vminm_dn4), (0.5 * (((((-locals.var_vbci_dn6) * assign4050_e4476) + (assign4050_e4471 * (-locals.var_vbci_dn6))) / (2.0 * assign4050_e4480)) + (-locals.var_vbci_dn6))), (0.5 * (((((-locals.var_vbci_dn8) * assign4050_e4476) + (assign4050_e4471 * (-locals.var_vbci_dn8))) / (2.0 * assign4050_e4480)) + (-locals.var_vbci_dn8))),)
    } else {
        (locals.var_vl__blk118, locals.var_vl__blk118_dn4, locals.var_vl__blk118_dn6, locals.var_vl__blk118_dn8,)
    }
};
        locals.var_vl__blk118 = assign4050_e4491;
        locals.var_vl__blk118_dn4 = assign4050_e4491_d_n4;
        locals.var_vl__blk118_dn6 = assign4050_e4491_d_n6;
        locals.var_vl__blk118_dn8 = assign4050_e4491_d_n8;

        let (assign4060_e4502, assign4060_e4502_d_n4, assign4060_e4502_d_n6, assign4060_e4502_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign4060_e4494: f64 = (-locals.var_avc2_t);
        let assign4060_e4498: f64 = (p.p43 - 1.0);
        let assign4060_e4499: f64 = (locals.var_vl__blk118).powf(assign4060_e4498);
        let assign4060_e4500: f64 = (assign4060_e4494 * assign4060_e4499);
        (assign4060_e4500, (((-locals.var_avc2_t_dn4) * assign4060_e4499) + (assign4060_e4494 * if 0.0 == 0.0 && ((assign4060_e4498) as f64).is_finite() && ((assign4060_e4498) as f64).fract() == 0.0 { if assign4060_e4498 == 0.0 { 0.0 } else { (assign4060_e4498 * ((locals.var_vl__blk118).powf(assign4060_e4498 - 1.0) * locals.var_vl__blk118_dn4)) } } else { (assign4060_e4499 * (assign4060_e4498 * (locals.var_vl__blk118_dn4 / locals.var_vl__blk118))) })), (assign4060_e4494 * if 0.0 == 0.0 && ((assign4060_e4498) as f64).is_finite() && ((assign4060_e4498) as f64).fract() == 0.0 { if assign4060_e4498 == 0.0 { 0.0 } else { (assign4060_e4498 * ((locals.var_vl__blk118).powf(assign4060_e4498 - 1.0) * locals.var_vl__blk118_dn6)) } } else { (assign4060_e4499 * (assign4060_e4498 * (locals.var_vl__blk118_dn6 / locals.var_vl__blk118))) }), (assign4060_e4494 * if 0.0 == 0.0 && ((assign4060_e4498) as f64).is_finite() && ((assign4060_e4498) as f64).fract() == 0.0 { if assign4060_e4498 == 0.0 { 0.0 } else { (assign4060_e4498 * ((locals.var_vl__blk118).powf(assign4060_e4498 - 1.0) * locals.var_vl__blk118_dn8)) } } else { (assign4060_e4499 * (assign4060_e4498 * (locals.var_vl__blk118_dn8 / locals.var_vl__blk118))) }),)
    } else {
        (locals.var_mac1, locals.var_mac1_dn4, locals.var_mac1_dn6, locals.var_mac1_dn8,)
    }
};
        locals.var_mac1 = assign4060_e4502;
        locals.var_mac1_dn4 = assign4060_e4502_d_n4;
        locals.var_mac1_dn6 = assign4060_e4502_d_n6;
        locals.var_mac1_dn8 = assign4060_e4502_d_n8;

        let assign4070_e4505: f64 = if locals.var_mac1 < locals.var_vmaxexp { 1.0 } else { 0.0 };
        locals.var_guard122 = assign4070_e4505;

        let (assign4080_e4512, assign4080_e4512_d_n4, assign4080_e4512_d_n6, assign4080_e4512_d_n8,) = {
    if ((locals.var_guard116 != 0.0) && (locals.var_guard122 != 0.0)) {
        let assign4080_e4510: f64 = (locals.var_mac1).exp();
        (assign4080_e4510, (assign4080_e4510 * locals.var_mac1_dn4), (assign4080_e4510 * locals.var_mac1_dn6), (assign4080_e4510 * locals.var_mac1_dn8),)
    } else {
        (locals.var_expi__blk120, locals.var_expi__blk120_dn4, locals.var_expi__blk120_dn6, locals.var_expi__blk120_dn8,)
    }
};
        locals.var_expi__blk120 = assign4080_e4512;
        locals.var_expi__blk120_dn4 = assign4080_e4512_d_n4;
        locals.var_expi__blk120_dn6 = assign4080_e4512_d_n6;
        locals.var_expi__blk120_dn8 = assign4080_e4512_d_n8;

        let (assign4090_e4520,) = {
    if ((locals.var_guard116 != 0.0) && (locals.var_guard122 == 0.0)) {
        let assign4090_e4518: f64 = (locals.var_vmaxexp).exp();
        (assign4090_e4518,)
    } else {
        (locals.var_expl,)
    }
};
        locals.var_expl = assign4090_e4520;

        let (assign4100_e4533, assign4100_e4533_d_n4, assign4100_e4533_d_n6, assign4100_e4533_d_n8,) = {
    if ((locals.var_guard116 != 0.0) && (locals.var_guard122 == 0.0)) {
        let assign4100_e4529: f64 = (locals.var_mac1 - locals.var_vmaxexp);
        let assign4100_e4530: f64 = (1.0 + assign4100_e4529);
        let assign4100_e4531: f64 = (locals.var_expl * assign4100_e4530);
        (assign4100_e4531, (locals.var_expl * locals.var_mac1_dn4), (locals.var_expl * locals.var_mac1_dn6), (locals.var_expl * locals.var_mac1_dn8),)
    } else {
        (locals.var_expi__blk120, locals.var_expi__blk120_dn4, locals.var_expi__blk120_dn6, locals.var_expi__blk120_dn8,)
    }
};
        locals.var_expi__blk120 = assign4100_e4533;
        locals.var_expi__blk120_dn4 = assign4100_e4533_d_n4;
        locals.var_expi__blk120_dn6 = assign4100_e4533_d_n6;
        locals.var_expi__blk120_dn8 = assign4100_e4533_d_n8;

        let (assign4110_e4541, assign4110_e4541_d_n4, assign4110_e4541_d_n5, assign4110_e4541_d_n6, assign4110_e4541_d_n7, assign4110_e4541_d_n8,) = {
    if (locals.var_guard116 != 0.0) {
        let assign4110_e4537: f64 = (p.p83 * locals.var_vl__blk118);
        let assign4110_e4539: f64 = (assign4110_e4537 * locals.var_expi__blk120);
        (assign4110_e4539, (((p.p83 * locals.var_vl__blk118_dn4) * locals.var_expi__blk120) + (assign4110_e4537 * locals.var_expi__blk120_dn4)), 0.0, (((p.p83 * locals.var_vl__blk118_dn6) * locals.var_expi__blk120) + (assign4110_e4537 * locals.var_expi__blk120_dn6)), 0.0, (((p.p83 * locals.var_vl__blk118_dn8) * locals.var_expi__blk120) + (assign4110_e4537 * locals.var_expi__blk120_dn8)),)
    } else {
        (locals.var_avalf, locals.var_avalf_dn4, locals.var_avalf_dn5, locals.var_avalf_dn6, locals.var_avalf_dn7, locals.var_avalf_dn8,)
    }
};
        locals.var_avalf = assign4110_e4541;
        locals.var_avalf_dn4 = assign4110_e4541_d_n4;
        locals.var_avalf_dn5 = assign4110_e4541_d_n5;
        locals.var_avalf_dn6 = assign4110_e4541_d_n6;
        locals.var_avalf_dn7 = assign4110_e4541_d_n7;
        locals.var_avalf_dn8 = assign4110_e4541_d_n8;

        let (assign4120_e4551, assign4120_e4551_d_n4, assign4120_e4551_d_n5, assign4120_e4551_d_n6, assign4120_e4551_d_n7, assign4120_e4551_d_n8, assign4120_e4551_d_n9, assign4120_e4551_d_n10, assign4120_e4551_d_n11, assign4120_e4551_d_n13,) = {
    if (locals.var_guard116 != 0.0) {
        let assign4120_e4545: f64 = (locals.var_itxf - locals.var_itzr);
        let assign4120_e4547: f64 = (assign4120_e4545 - locals.var_ibcj);
        let assign4120_e4549: f64 = (assign4120_e4547 * locals.var_avalf);
        (assign4120_e4549, ((((-locals.var_itzr_dn4) - locals.var_ibcj_dn4) * locals.var_avalf) + (assign4120_e4547 * locals.var_avalf_dn4)), ((((-locals.var_itzr_dn5) - locals.var_ibcj_dn5) * locals.var_avalf) + (assign4120_e4547 * locals.var_avalf_dn5)), ((((-locals.var_itzr_dn6) - locals.var_ibcj_dn6) * locals.var_avalf) + (assign4120_e4547 * locals.var_avalf_dn6)), ((((-locals.var_itzr_dn7) - locals.var_ibcj_dn7) * locals.var_avalf) + (assign4120_e4547 * locals.var_avalf_dn7)), ((((-locals.var_itzr_dn8) - locals.var_ibcj_dn8) * locals.var_avalf) + (assign4120_e4547 * locals.var_avalf_dn8)), (((-locals.var_itzr_dn9) - locals.var_ibcj_dn9) * locals.var_avalf), (((-locals.var_itzr_dn10) - locals.var_ibcj_dn10) * locals.var_avalf), (((-locals.var_itzr_dn11) - locals.var_ibcj_dn11) * locals.var_avalf), (locals.var_itxf_dn13 * locals.var_avalf),)
    } else {
        (locals.var_igc, locals.var_igc_dn4, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, locals.var_igc_dn9, locals.var_igc_dn10, locals.var_igc_dn11, locals.var_igc_dn13,)
    }
};
        locals.var_igc = assign4120_e4551;
        locals.var_igc_dn4 = assign4120_e4551_d_n4;
        locals.var_igc_dn5 = assign4120_e4551_d_n5;
        locals.var_igc_dn6 = assign4120_e4551_d_n6;
        locals.var_igc_dn7 = assign4120_e4551_d_n7;
        locals.var_igc_dn8 = assign4120_e4551_d_n8;
        locals.var_igc_dn9 = assign4120_e4551_d_n9;
        locals.var_igc_dn10 = assign4120_e4551_d_n10;
        locals.var_igc_dn11 = assign4120_e4551_d_n11;
        locals.var_igc_dn13 = assign4120_e4551_d_n13;

        let (assign4130_e4556, assign4130_e4556_d_n4, assign4130_e4556_d_n5, assign4130_e4556_d_n6, assign4130_e4556_d_n7, assign4130_e4556_d_n8, assign4130_e4556_d_n9, assign4130_e4556_d_n10, assign4130_e4556_d_n11, assign4130_e4556_d_n13,) = {
    if (locals.var_guard116 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igc, locals.var_igc_dn4, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, locals.var_igc_dn9, locals.var_igc_dn10, locals.var_igc_dn11, locals.var_igc_dn13,)
    }
};
        locals.var_igc = assign4130_e4556;
        locals.var_igc_dn4 = assign4130_e4556_d_n4;
        locals.var_igc_dn5 = assign4130_e4556_d_n5;
        locals.var_igc_dn6 = assign4130_e4556_d_n6;
        locals.var_igc_dn7 = assign4130_e4556_d_n7;
        locals.var_igc_dn8 = assign4130_e4556_d_n8;
        locals.var_igc_dn9 = assign4130_e4556_d_n9;
        locals.var_igc_dn10 = assign4130_e4556_d_n10;
        locals.var_igc_dn11 = assign4130_e4556_d_n11;
        locals.var_igc_dn13 = assign4130_e4556_d_n13;

        let assign4140_e4559: f64 = if p.p85 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign4140_e4559;

        let (assign4150_e4573, assign4150_e4573_d_n4,) = {
    if (locals.var_guard123 != 0.0) {
        let assign4150_e4564: f64 = (locals.var_avcx2_t + 1.0);
        let assign4150_e4565: f64 = (0.02 * assign4150_e4564);
        let assign4150_e4569: f64 = (1.01 - p.p87);
        let assign4150_e4570: f64 = (1.0 / assign4150_e4569);
        let assign4150_e4571: f64 = (assign4150_e4565).powf(assign4150_e4570);
        (assign4150_e4571, if 0.0 == 0.0 && ((assign4150_e4570) as f64).is_finite() && ((assign4150_e4570) as f64).fract() == 0.0 { if assign4150_e4570 == 0.0 { 0.0 } else { (assign4150_e4570 * ((assign4150_e4565).powf(assign4150_e4570 - 1.0) * (0.02 * locals.var_avcx2_t_dn4))) } } else { (assign4150_e4571 * (assign4150_e4570 * ((0.02 * locals.var_avcx2_t_dn4) / assign4150_e4565))) },)
    } else {
        (locals.var_vminm__blk124, locals.var_vminm__blk124_dn4,)
    }
};
        locals.var_vminm__blk124 = assign4150_e4573;
        locals.var_vminm__blk124_dn4 = assign4150_e4573_d_n4;

        let (assign4160_e4600, assign4160_e4600_d_n4, assign4160_e4600_d_n5, assign4160_e4600_d_n7,) = {
    if (locals.var_guard123 != 0.0) {
        let assign4160_e4578: f64 = (-locals.var_vbxcx);
        let assign4160_e4580: f64 = (assign4160_e4578 - locals.var_vminm__blk124);
        let assign4160_e4583: f64 = (-locals.var_vbxcx);
        let assign4160_e4585: f64 = (assign4160_e4583 - locals.var_vminm__blk124);
        let assign4160_e4586: f64 = (assign4160_e4580 * assign4160_e4585);
        let assign4160_e4588: f64 = (assign4160_e4586 + 0.01);
        let assign4160_e4589: f64 = (assign4160_e4588).sqrt();
        let assign4160_e4592: f64 = (-locals.var_vbxcx);
        let assign4160_e4594: f64 = (assign4160_e4592 - locals.var_vminm__blk124);
        let assign4160_e4595: f64 = (assign4160_e4589 + assign4160_e4594);
        let assign4160_e4596: f64 = (0.5 * assign4160_e4595);
        let assign4160_e4598: f64 = (assign4160_e4596 + locals.var_vminm__blk124);
        (assign4160_e4598, ((0.5 * (((((-locals.var_vminm__blk124_dn4) * assign4160_e4585) + (assign4160_e4580 * (-locals.var_vminm__blk124_dn4))) / (2.0 * assign4160_e4589)) + (-locals.var_vminm__blk124_dn4))) + locals.var_vminm__blk124_dn4), (0.5 * (((((-locals.var_vbxcx_dn5) * assign4160_e4585) + (assign4160_e4580 * (-locals.var_vbxcx_dn5))) / (2.0 * assign4160_e4589)) + (-locals.var_vbxcx_dn5))), (0.5 * (((((-locals.var_vbxcx_dn7) * assign4160_e4585) + (assign4160_e4580 * (-locals.var_vbxcx_dn7))) / (2.0 * assign4160_e4589)) + (-locals.var_vbxcx_dn7))),)
    } else {
        (locals.var_vl__blk125, locals.var_vl__blk125_dn4, locals.var_vl__blk125_dn5, locals.var_vl__blk125_dn7,)
    }
};
        locals.var_vl__blk125 = assign4160_e4600;
        locals.var_vl__blk125_dn4 = assign4160_e4600_d_n4;
        locals.var_vl__blk125_dn5 = assign4160_e4600_d_n5;
        locals.var_vl__blk125_dn7 = assign4160_e4600_d_n7;

        let (assign4170_e4611, assign4170_e4611_d_n4, assign4170_e4611_d_n5, assign4170_e4611_d_n7,) = {
    if (locals.var_guard123 != 0.0) {
        let assign4170_e4603: f64 = (-locals.var_avcx2_t);
        let assign4170_e4607: f64 = (p.p87 - 1.0);
        let assign4170_e4608: f64 = (locals.var_vl__blk125).powf(assign4170_e4607);
        let assign4170_e4609: f64 = (assign4170_e4603 * assign4170_e4608);
        (assign4170_e4609, (((-locals.var_avcx2_t_dn4) * assign4170_e4608) + (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((locals.var_vl__blk125).powf(assign4170_e4607 - 1.0) * locals.var_vl__blk125_dn4)) } } else { (assign4170_e4608 * (assign4170_e4607 * (locals.var_vl__blk125_dn4 / locals.var_vl__blk125))) })), (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((locals.var_vl__blk125).powf(assign4170_e4607 - 1.0) * locals.var_vl__blk125_dn5)) } } else { (assign4170_e4608 * (assign4170_e4607 * (locals.var_vl__blk125_dn5 / locals.var_vl__blk125))) }), (assign4170_e4603 * if 0.0 == 0.0 && ((assign4170_e4607) as f64).is_finite() && ((assign4170_e4607) as f64).fract() == 0.0 { if assign4170_e4607 == 0.0 { 0.0 } else { (assign4170_e4607 * ((locals.var_vl__blk125).powf(assign4170_e4607 - 1.0) * locals.var_vl__blk125_dn7)) } } else { (assign4170_e4608 * (assign4170_e4607 * (locals.var_vl__blk125_dn7 / locals.var_vl__blk125))) }),)
    } else {
        (locals.var_mac1__blk126, locals.var_mac1__blk126_dn4, locals.var_mac1__blk126_dn5, locals.var_mac1__blk126_dn7,)
    }
};
        locals.var_mac1__blk126 = assign4170_e4611;
        locals.var_mac1__blk126_dn4 = assign4170_e4611_d_n4;
        locals.var_mac1__blk126_dn5 = assign4170_e4611_d_n5;
        locals.var_mac1__blk126_dn7 = assign4170_e4611_d_n7;

        let assign4180_e4614: f64 = if locals.var_mac1__blk126 < locals.var_vmaxexp { 1.0 } else { 0.0 };
        locals.var_guard129 = assign4180_e4614;

        let (assign4190_e4621, assign4190_e4621_d_n4, assign4190_e4621_d_n5, assign4190_e4621_d_n7,) = {
    if ((locals.var_guard123 != 0.0) && (locals.var_guard129 != 0.0)) {
        let assign4190_e4619: f64 = (locals.var_mac1__blk126).exp();
        (assign4190_e4619, (assign4190_e4619 * locals.var_mac1__blk126_dn4), (assign4190_e4619 * locals.var_mac1__blk126_dn5), (assign4190_e4619 * locals.var_mac1__blk126_dn7),)
    } else {
        (locals.var_expi__blk127, locals.var_expi__blk127_dn4, locals.var_expi__blk127_dn5, locals.var_expi__blk127_dn7,)
    }
};
        locals.var_expi__blk127 = assign4190_e4621;
        locals.var_expi__blk127_dn4 = assign4190_e4621_d_n4;
        locals.var_expi__blk127_dn5 = assign4190_e4621_d_n5;
        locals.var_expi__blk127_dn7 = assign4190_e4621_d_n7;

        let (assign4200_e4629,) = {
    if ((locals.var_guard123 != 0.0) && (locals.var_guard129 == 0.0)) {
        let assign4200_e4627: f64 = (locals.var_vmaxexp).exp();
        (assign4200_e4627,)
    } else {
        (locals.var_expl__blk128,)
    }
};
        locals.var_expl__blk128 = assign4200_e4629;

        let (assign4210_e4642, assign4210_e4642_d_n4, assign4210_e4642_d_n5, assign4210_e4642_d_n7,) = {
    if ((locals.var_guard123 != 0.0) && (locals.var_guard129 == 0.0)) {
        let assign4210_e4638: f64 = (locals.var_mac1__blk126 - locals.var_vmaxexp);
        let assign4210_e4639: f64 = (1.0 + assign4210_e4638);
        let assign4210_e4640: f64 = (locals.var_expl__blk128 * assign4210_e4639);
        (assign4210_e4640, (locals.var_expl__blk128 * locals.var_mac1__blk126_dn4), (locals.var_expl__blk128 * locals.var_mac1__blk126_dn5), (locals.var_expl__blk128 * locals.var_mac1__blk126_dn7),)
    } else {
        (locals.var_expi__blk127, locals.var_expi__blk127_dn4, locals.var_expi__blk127_dn5, locals.var_expi__blk127_dn7,)
    }
};
        locals.var_expi__blk127 = assign4210_e4642;
        locals.var_expi__blk127_dn4 = assign4210_e4642_d_n4;
        locals.var_expi__blk127_dn5 = assign4210_e4642_d_n5;
        locals.var_expi__blk127_dn7 = assign4210_e4642_d_n7;

        let (assign4220_e4650, assign4220_e4650_d_n4, assign4220_e4650_d_n5, assign4220_e4650_d_n6, assign4220_e4650_d_n7, assign4220_e4650_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign4220_e4646: f64 = (p.p85 * locals.var_vl__blk125);
        let assign4220_e4648: f64 = (assign4220_e4646 * locals.var_expi__blk127);
        (assign4220_e4648, (((p.p85 * locals.var_vl__blk125_dn4) * locals.var_expi__blk127) + (assign4220_e4646 * locals.var_expi__blk127_dn4)), (((p.p85 * locals.var_vl__blk125_dn5) * locals.var_expi__blk127) + (assign4220_e4646 * locals.var_expi__blk127_dn5)), 0.0, (((p.p85 * locals.var_vl__blk125_dn7) * locals.var_expi__blk127) + (assign4220_e4646 * locals.var_expi__blk127_dn7)), 0.0,)
    } else {
        (locals.var_avalf, locals.var_avalf_dn4, locals.var_avalf_dn5, locals.var_avalf_dn6, locals.var_avalf_dn7, locals.var_avalf_dn8,)
    }
};
        locals.var_avalf = assign4220_e4650;
        locals.var_avalf_dn4 = assign4220_e4650_d_n4;
        locals.var_avalf_dn5 = assign4220_e4650_d_n5;
        locals.var_avalf_dn6 = assign4220_e4650_d_n6;
        locals.var_avalf_dn7 = assign4220_e4650_d_n7;
        locals.var_avalf_dn8 = assign4220_e4650_d_n8;

        let (assign4230_e4657, assign4230_e4657_d_n0, assign4230_e4657_d_n4, assign4230_e4657_d_n5, assign4230_e4657_d_n6, assign4230_e4657_d_n7, assign4230_e4657_d_n8,) = {
    if (locals.var_guard123 != 0.0) {
        let assign4230_e4653: f64 = (-locals.var_ircx);
        let assign4230_e4655: f64 = (assign4230_e4653 * locals.var_avalf);
        (assign4230_e4655, ((-locals.var_ircx_dn0) * locals.var_avalf), (((-locals.var_ircx_dn4) * locals.var_avalf) + (assign4230_e4653 * locals.var_avalf_dn4)), (((-locals.var_ircx_dn5) * locals.var_avalf) + (assign4230_e4653 * locals.var_avalf_dn5)), (assign4230_e4653 * locals.var_avalf_dn6), (assign4230_e4653 * locals.var_avalf_dn7), (assign4230_e4653 * locals.var_avalf_dn8),)
    } else {
        (locals.var_igcx, locals.var_igcx_dn0, locals.var_igcx_dn4, locals.var_igcx_dn5, locals.var_igcx_dn6, locals.var_igcx_dn7, locals.var_igcx_dn8,)
    }
};
        locals.var_igcx = assign4230_e4657;
        locals.var_igcx_dn0 = assign4230_e4657_d_n0;
        locals.var_igcx_dn4 = assign4230_e4657_d_n4;
        locals.var_igcx_dn5 = assign4230_e4657_d_n5;
        locals.var_igcx_dn6 = assign4230_e4657_d_n6;
        locals.var_igcx_dn7 = assign4230_e4657_d_n7;
        locals.var_igcx_dn8 = assign4230_e4657_d_n8;

        let (assign4240_e4662, assign4240_e4662_d_n0, assign4240_e4662_d_n4, assign4240_e4662_d_n5, assign4240_e4662_d_n6, assign4240_e4662_d_n7, assign4240_e4662_d_n8,) = {
    if (locals.var_guard123 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcx, locals.var_igcx_dn0, locals.var_igcx_dn4, locals.var_igcx_dn5, locals.var_igcx_dn6, locals.var_igcx_dn7, locals.var_igcx_dn8,)
    }
};
        locals.var_igcx = assign4240_e4662;
        locals.var_igcx_dn0 = assign4240_e4662_d_n0;
        locals.var_igcx_dn4 = assign4240_e4662_d_n4;
        locals.var_igcx_dn5 = assign4240_e4662_d_n5;
        locals.var_igcx_dn6 = assign4240_e4662_d_n6;
        locals.var_igcx_dn7 = assign4240_e4662_d_n7;
        locals.var_igcx_dn8 = assign4240_e4662_d_n8;

        let assign4250_e4669: f64 = if ((p.p97 > 0.0) && (p.p95 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign4250_e4669;

        let assign4260_e4672: f64 = if p.p94 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign4260_e4672;

        let (assign4270_e4684, assign4270_e4684_d_n6, assign4270_e4684_d_n8,) = {
    if ((locals.var_guard130 != 0.0) && (locals.var_guard131 != 0.0)) {
        let assign4270_e4679: f64 = (locals.var_vbci / p.p94);
        let assign4270_e4680: f64 = (1.0 - assign4270_e4679);
        let assign4270_e4682: f64 = (assign4270_e4680 - 0.1);
        (assign4270_e4682, (-(locals.var_vbci_dn6 / p.p94)), (-(locals.var_vbci_dn8 / p.p94)),)
    } else {
        (locals.var_vcbfac, locals.var_vcbfac_dn6, locals.var_vcbfac_dn8,)
    }
};
        locals.var_vcbfac = assign4270_e4684;
        locals.var_vcbfac_dn6 = assign4270_e4684_d_n6;
        locals.var_vcbfac_dn8 = assign4270_e4684_d_n8;

        let (assign4280_e4701, assign4280_e4701_d_n6, assign4280_e4701_d_n8,) = {
    if ((locals.var_guard130 != 0.0) && (locals.var_guard131 != 0.0)) {
        let assign4280_e4693: f64 = (locals.var_vcbfac * locals.var_vcbfac);
        let assign4280_e4695: f64 = (assign4280_e4693 + 0.0001);
        let assign4280_e4696: f64 = (assign4280_e4695).sqrt();
        let assign4280_e4697: f64 = (locals.var_vcbfac + assign4280_e4696);
        let assign4280_e4698: f64 = (0.5 * assign4280_e4697);
        let assign4280_e4699: f64 = (0.1 + assign4280_e4698);
        (assign4280_e4699, (0.5 * (locals.var_vcbfac_dn6 + (((locals.var_vcbfac_dn6 * locals.var_vcbfac) + (locals.var_vcbfac * locals.var_vcbfac_dn6)) / (2.0 * assign4280_e4696)))), (0.5 * (locals.var_vcbfac_dn8 + (((locals.var_vcbfac_dn8 * locals.var_vcbfac) + (locals.var_vcbfac * locals.var_vcbfac_dn8)) / (2.0 * assign4280_e4696)))),)
    } else {
        (locals.var_vcbfac, locals.var_vcbfac_dn6, locals.var_vcbfac_dn8,)
    }
};
        locals.var_vcbfac = assign4280_e4701;
        locals.var_vcbfac_dn6 = assign4280_e4701_d_n6;
        locals.var_vcbfac_dn8 = assign4280_e4701_d_n8;

        let (assign4290_e4709, assign4290_e4709_d_n6, assign4290_e4709_d_n8,) = {
    if ((locals.var_guard130 != 0.0) && (locals.var_guard131 != 0.0)) {
        let assign4290_e4707: f64 = (p.p95 * locals.var_vcbfac);
        (assign4290_e4707, (p.p95 * locals.var_vcbfac_dn6), (p.p95 * locals.var_vcbfac_dn8),)
    } else {
        (locals.var_iibk, locals.var_iibk_dn6, locals.var_iibk_dn8,)
    }
};
        locals.var_iibk = assign4290_e4709;
        locals.var_iibk_dn6 = assign4290_e4709_d_n6;
        locals.var_iibk_dn8 = assign4290_e4709_d_n8;

        let (assign4300_e4716, assign4300_e4716_d_n6, assign4300_e4716_d_n8,) = {
    if ((locals.var_guard130 != 0.0) && (locals.var_guard131 == 0.0)) {
        (p.p95, 0.0, 0.0,)
    } else {
        (locals.var_iibk, locals.var_iibk_dn6, locals.var_iibk_dn8,)
    }
};
        locals.var_iibk = assign4300_e4716;
        locals.var_iibk_dn6 = assign4300_e4716_d_n6;
        locals.var_iibk_dn8 = assign4300_e4716_d_n8;

        let (assign4310_e4728, assign4310_e4728_d_n4, assign4310_e4728_d_n5, assign4310_e4728_d_n6, assign4310_e4728_d_n7, assign4310_e4728_d_n8, assign4310_e4728_d_n9, assign4310_e4728_d_n10, assign4310_e4728_d_n11,) = {
    if (locals.var_guard130 != 0.0) {
        let assign4310_e4721: f64 = (locals.var_itzf / locals.var_iibk);
        let assign4310_e4723: f64 = (assign4310_e4721 - 1.0);
        let assign4310_e4725: f64 = (assign4310_e4723).powf(p.p96);
        let assign4310_e4726: f64 = (p.p97 * assign4310_e4725);
        (assign4310_e4726, (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (locals.var_itzf_dn4 / locals.var_iibk))) } } else { (assign4310_e4725 * (p.p96 * ((locals.var_itzf_dn4 / locals.var_iibk) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (locals.var_itzf_dn5 / locals.var_iibk))) } } else { (assign4310_e4725 * (p.p96 * ((locals.var_itzf_dn5 / locals.var_iibk) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((locals.var_itzf_dn6 * locals.var_iibk) - (locals.var_itzf * locals.var_iibk_dn6)) / (locals.var_iibk * locals.var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((locals.var_itzf_dn6 * locals.var_iibk) - (locals.var_itzf * locals.var_iibk_dn6)) / (locals.var_iibk * locals.var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (locals.var_itzf_dn7 / locals.var_iibk))) } } else { (assign4310_e4725 * (p.p96 * ((locals.var_itzf_dn7 / locals.var_iibk) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (((locals.var_itzf_dn8 * locals.var_iibk) - (locals.var_itzf * locals.var_iibk_dn8)) / (locals.var_iibk * locals.var_iibk)))) } } else { (assign4310_e4725 * (p.p96 * ((((locals.var_itzf_dn8 * locals.var_iibk) - (locals.var_itzf * locals.var_iibk_dn8)) / (locals.var_iibk * locals.var_iibk)) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (locals.var_itzf_dn9 / locals.var_iibk))) } } else { (assign4310_e4725 * (p.p96 * ((locals.var_itzf_dn9 / locals.var_iibk) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (locals.var_itzf_dn10 / locals.var_iibk))) } } else { (assign4310_e4725 * (p.p96 * ((locals.var_itzf_dn10 / locals.var_iibk) / assign4310_e4723))) }), (p.p97 * if 0.0 == 0.0 && ((p.p96) as f64).is_finite() && ((p.p96) as f64).fract() == 0.0 { if p.p96 == 0.0 { 0.0 } else { (p.p96 * ((assign4310_e4723).powf(p.p96 - 1.0) * (locals.var_itzf_dn11 / locals.var_iibk))) } } else { (assign4310_e4725 * (p.p96 * ((locals.var_itzf_dn11 / locals.var_iibk) / assign4310_e4723))) }),)
    } else {
        (locals.var_ibk, locals.var_ibk_dn4, locals.var_ibk_dn5, locals.var_ibk_dn6, locals.var_ibk_dn7, locals.var_ibk_dn8, locals.var_ibk_dn9, locals.var_ibk_dn10, locals.var_ibk_dn11,)
    }
};
        locals.var_ibk = assign4310_e4728;
        locals.var_ibk_dn4 = assign4310_e4728_d_n4;
        locals.var_ibk_dn5 = assign4310_e4728_d_n5;
        locals.var_ibk_dn6 = assign4310_e4728_d_n6;
        locals.var_ibk_dn7 = assign4310_e4728_d_n7;
        locals.var_ibk_dn8 = assign4310_e4728_d_n8;
        locals.var_ibk_dn9 = assign4310_e4728_d_n9;
        locals.var_ibk_dn10 = assign4310_e4728_d_n10;
        locals.var_ibk_dn11 = assign4310_e4728_d_n11;

        let (assign4320_e4733, assign4320_e4733_d_n4, assign4320_e4733_d_n5, assign4320_e4733_d_n6, assign4320_e4733_d_n7, assign4320_e4733_d_n8, assign4320_e4733_d_n9, assign4320_e4733_d_n10, assign4320_e4733_d_n11,) = {
    if (locals.var_guard130 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibk, locals.var_ibk_dn4, locals.var_ibk_dn5, locals.var_ibk_dn6, locals.var_ibk_dn7, locals.var_ibk_dn8, locals.var_ibk_dn9, locals.var_ibk_dn10, locals.var_ibk_dn11,)
    }
};
        locals.var_ibk = assign4320_e4733;
        locals.var_ibk_dn4 = assign4320_e4733_d_n4;
        locals.var_ibk_dn5 = assign4320_e4733_d_n5;
        locals.var_ibk_dn6 = assign4320_e4733_d_n6;
        locals.var_ibk_dn7 = assign4320_e4733_d_n7;
        locals.var_ibk_dn8 = assign4320_e4733_d_n8;
        locals.var_ibk_dn9 = assign4320_e4733_d_n9;
        locals.var_ibk_dn10 = assign4320_e4733_d_n10;
        locals.var_ibk_dn11 = assign4320_e4733_d_n11;

        let assign4330_e4736: f64 = (locals.var_ibcj - locals.var_igc);
        let assign4330_e4738: f64 = (assign4330_e4736 - locals.var_ibk);
        locals.var_ibc = assign4330_e4738;
        locals.var_ibc_dn4 = ((locals.var_ibcj_dn4 - locals.var_igc_dn4) - locals.var_ibk_dn4);
        locals.var_ibc_dn5 = ((locals.var_ibcj_dn5 - locals.var_igc_dn5) - locals.var_ibk_dn5);
        locals.var_ibc_dn6 = ((locals.var_ibcj_dn6 - locals.var_igc_dn6) - locals.var_ibk_dn6);
        locals.var_ibc_dn7 = ((locals.var_ibcj_dn7 - locals.var_igc_dn7) - locals.var_ibk_dn7);
        locals.var_ibc_dn8 = ((locals.var_ibcj_dn8 - locals.var_igc_dn8) - locals.var_ibk_dn8);
        locals.var_ibc_dn9 = ((locals.var_ibcj_dn9 - locals.var_igc_dn9) - locals.var_ibk_dn9);
        locals.var_ibc_dn10 = ((locals.var_ibcj_dn10 - locals.var_igc_dn10) - locals.var_ibk_dn10);
        locals.var_ibc_dn11 = ((locals.var_ibcj_dn11 - locals.var_igc_dn11) - locals.var_ibk_dn11);
        locals.var_ibc_dn13 = (-locals.var_igc_dn13);

        let assign4340_e4745: f64 = if ((p.p66 > 0.0) || (p.p68 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign4340_e4745;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4350_e4753, assign4350_e4753_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign4350_e4750: f64 = (p.p67 * locals.var_vtv);
        let assign4350_e4751: f64 = (1.0 / assign4350_e4750);
        (assign4350_e4751, (-((p.p67 * locals.var_vtv_dn4) / (assign4350_e4750 * assign4350_e4750))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign4350_e4753;
        locals.var_afac_dn4 = assign4350_e4753_d_n4;

        let assign4360_e4756: f64 = if locals.var_vbcp < locals.var_maxvibcip { 1.0 } else { 0.0 };
        locals.var_guard133 = assign4360_e4756;

        let (assign4370_e4765, assign4370_e4765_d_n4, assign4370_e4765_d_n5, assign4370_e4765_d_n6, assign4370_e4765_d_n7, assign4370_e4765_d_n8, assign4370_e4765_d_n9, assign4370_e4765_d_n10, assign4370_e4765_d_n11,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard133 != 0.0)) {
        let assign4370_e4762: f64 = (locals.var_vbcp * locals.var_afac);
        let assign4370_e4763: f64 = (assign4370_e4762).exp();
        (assign4370_e4763, (assign4370_e4763 * (locals.var_vbcp * locals.var_afac_dn4)), 0.0, 0.0, 0.0, 0.0, 0.0, (assign4370_e4763 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign4370_e4763 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign4370_e4765;
        locals.var_expi_dn4 = assign4370_e4765_d_n4;
        locals.var_expi_dn5 = assign4370_e4765_d_n5;
        locals.var_expi_dn6 = assign4370_e4765_d_n6;
        locals.var_expi_dn7 = assign4370_e4765_d_n7;
        locals.var_expi_dn8 = assign4370_e4765_d_n8;
        locals.var_expi_dn9 = assign4370_e4765_d_n9;
        locals.var_expi_dn10 = assign4370_e4765_d_n10;
        locals.var_expi_dn11 = assign4370_e4765_d_n11;

        let (assign4380_e4783, assign4380_e4783_d_n4, assign4380_e4783_d_n5, assign4380_e4783_d_n6, assign4380_e4783_d_n7, assign4380_e4783_d_n8, assign4380_e4783_d_n9, assign4380_e4783_d_n10, assign4380_e4783_d_n11,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard133 == 0.0)) {
        let assign4380_e4772: f64 = (locals.var_maxvibcip * locals.var_afac);
        let assign4380_e4773: f64 = (assign4380_e4772).exp();
        let assign4380_e4777: f64 = (locals.var_vbcp - locals.var_maxvibcip);
        let assign4380_e4779: f64 = (assign4380_e4777 * locals.var_afac);
        let assign4380_e4780: f64 = (1.0 + assign4380_e4779);
        let assign4380_e4781: f64 = (assign4380_e4773 * assign4380_e4780);
        (assign4380_e4781, (((assign4380_e4773 * ((locals.var_maxvibcip_dn4 * locals.var_afac) + (locals.var_maxvibcip * locals.var_afac_dn4))) * assign4380_e4780) + (assign4380_e4773 * (((-locals.var_maxvibcip_dn4) * locals.var_afac) + (assign4380_e4777 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, 0.0, 0.0, (assign4380_e4773 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign4380_e4773 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign4380_e4783;
        locals.var_expi_dn4 = assign4380_e4783_d_n4;
        locals.var_expi_dn5 = assign4380_e4783_d_n5;
        locals.var_expi_dn6 = assign4380_e4783_d_n6;
        locals.var_expi_dn7 = assign4380_e4783_d_n7;
        locals.var_expi_dn8 = assign4380_e4783_d_n8;
        locals.var_expi_dn9 = assign4380_e4783_d_n9;
        locals.var_expi_dn10 = assign4380_e4783_d_n10;
        locals.var_expi_dn11 = assign4380_e4783_d_n11;

        let (assign4390_e4791, assign4390_e4791_d_n4,) = {
    if (locals.var_guard132 != 0.0) {
        let assign4390_e4788: f64 = (p.p69 * locals.var_vtv);
        let assign4390_e4789: f64 = (1.0 / assign4390_e4788);
        (assign4390_e4789, (-((p.p69 * locals.var_vtv_dn4) / (assign4390_e4788 * assign4390_e4788))),)
    } else {
        (locals.var_afac, locals.var_afac_dn4,)
    }
};
        locals.var_afac = assign4390_e4791;
        locals.var_afac_dn4 = assign4390_e4791_d_n4;

        let assign4400_e4794: f64 = if locals.var_vbcp < locals.var_maxvibcnp { 1.0 } else { 0.0 };
        locals.var_guard134 = assign4400_e4794;

        let (assign4410_e4803, assign4410_e4803_d_n4, assign4410_e4803_d_n6, assign4410_e4803_d_n7, assign4410_e4803_d_n8, assign4410_e4803_d_n9, assign4410_e4803_d_n10, assign4410_e4803_d_n11,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign4410_e4800: f64 = (locals.var_vbcp * locals.var_afac);
        let assign4410_e4801: f64 = (assign4410_e4800).exp();
        (assign4410_e4801, (assign4410_e4801 * (locals.var_vbcp * locals.var_afac_dn4)), 0.0, 0.0, 0.0, 0.0, (assign4410_e4801 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign4410_e4801 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign4410_e4803;
        locals.var_expn_dn4 = assign4410_e4803_d_n4;
        locals.var_expn_dn6 = assign4410_e4803_d_n6;
        locals.var_expn_dn7 = assign4410_e4803_d_n7;
        locals.var_expn_dn8 = assign4410_e4803_d_n8;
        locals.var_expn_dn9 = assign4410_e4803_d_n9;
        locals.var_expn_dn10 = assign4410_e4803_d_n10;
        locals.var_expn_dn11 = assign4410_e4803_d_n11;

        let (assign4420_e4821, assign4420_e4821_d_n4, assign4420_e4821_d_n6, assign4420_e4821_d_n7, assign4420_e4821_d_n8, assign4420_e4821_d_n9, assign4420_e4821_d_n10, assign4420_e4821_d_n11,) = {
    if ((locals.var_guard132 != 0.0) && (locals.var_guard134 == 0.0)) {
        let assign4420_e4810: f64 = (locals.var_maxvibcnp * locals.var_afac);
        let assign4420_e4811: f64 = (assign4420_e4810).exp();
        let assign4420_e4815: f64 = (locals.var_vbcp - locals.var_maxvibcnp);
        let assign4420_e4817: f64 = (assign4420_e4815 * locals.var_afac);
        let assign4420_e4818: f64 = (1.0 + assign4420_e4817);
        let assign4420_e4819: f64 = (assign4420_e4811 * assign4420_e4818);
        (assign4420_e4819, (((assign4420_e4811 * ((locals.var_maxvibcnp_dn4 * locals.var_afac) + (locals.var_maxvibcnp * locals.var_afac_dn4))) * assign4420_e4818) + (assign4420_e4811 * (((-locals.var_maxvibcnp_dn4) * locals.var_afac) + (assign4420_e4815 * locals.var_afac_dn4)))), 0.0, 0.0, 0.0, 0.0, (assign4420_e4811 * (locals.var_vbcp_dn10 * locals.var_afac)), (assign4420_e4811 * (locals.var_vbcp_dn11 * locals.var_afac)),)
    } else {
        (locals.var_expn, locals.var_expn_dn4, locals.var_expn_dn6, locals.var_expn_dn7, locals.var_expn_dn8, locals.var_expn_dn9, locals.var_expn_dn10, locals.var_expn_dn11,)
    }
};
        locals.var_expn = assign4420_e4821;
        locals.var_expn_dn4 = assign4420_e4821_d_n4;
        locals.var_expn_dn6 = assign4420_e4821_d_n6;
        locals.var_expn_dn7 = assign4420_e4821_d_n7;
        locals.var_expn_dn8 = assign4420_e4821_d_n8;
        locals.var_expn_dn9 = assign4420_e4821_d_n9;
        locals.var_expn_dn10 = assign4420_e4821_d_n10;
        locals.var_expn_dn11 = assign4420_e4821_d_n11;

        let (assign4430_e4835, assign4430_e4835_d_n4, assign4430_e4835_d_n5, assign4430_e4835_d_n6, assign4430_e4835_d_n7, assign4430_e4835_d_n8, assign4430_e4835_d_n9, assign4430_e4835_d_n10, assign4430_e4835_d_n11,) = {
    if (locals.var_guard132 != 0.0) {
        let assign4430_e4826: f64 = (locals.var_expi - 1.0);
        let assign4430_e4827: f64 = (locals.var_ibcip_t * assign4430_e4826);
        let assign4430_e4831: f64 = (locals.var_expn - 1.0);
        let assign4430_e4832: f64 = (locals.var_ibcnp_t * assign4430_e4831);
        let assign4430_e4833: f64 = (assign4430_e4827 + assign4430_e4832);
        (assign4430_e4833, (((locals.var_ibcip_t_dn4 * assign4430_e4826) + (locals.var_ibcip_t * locals.var_expi_dn4)) + ((locals.var_ibcnp_t_dn4 * assign4430_e4831) + (locals.var_ibcnp_t * locals.var_expn_dn4))), (locals.var_ibcip_t * locals.var_expi_dn5), ((locals.var_ibcip_t * locals.var_expi_dn6) + (locals.var_ibcnp_t * locals.var_expn_dn6)), ((locals.var_ibcip_t * locals.var_expi_dn7) + (locals.var_ibcnp_t * locals.var_expn_dn7)), ((locals.var_ibcip_t * locals.var_expi_dn8) + (locals.var_ibcnp_t * locals.var_expn_dn8)), ((locals.var_ibcip_t * locals.var_expi_dn9) + (locals.var_ibcnp_t * locals.var_expn_dn9)), ((locals.var_ibcip_t * locals.var_expi_dn10) + (locals.var_ibcnp_t * locals.var_expn_dn10)), ((locals.var_ibcip_t * locals.var_expi_dn11) + (locals.var_ibcnp_t * locals.var_expn_dn11)),)
    } else {
        (locals.var_ibcp, locals.var_ibcp_dn4, locals.var_ibcp_dn5, locals.var_ibcp_dn6, locals.var_ibcp_dn7, locals.var_ibcp_dn8, locals.var_ibcp_dn9, locals.var_ibcp_dn10, locals.var_ibcp_dn11,)
    }
};
        locals.var_ibcp = assign4430_e4835;
        locals.var_ibcp_dn4 = assign4430_e4835_d_n4;
        locals.var_ibcp_dn5 = assign4430_e4835_d_n5;
        locals.var_ibcp_dn6 = assign4430_e4835_d_n6;
        locals.var_ibcp_dn7 = assign4430_e4835_d_n7;
        locals.var_ibcp_dn8 = assign4430_e4835_d_n8;
        locals.var_ibcp_dn9 = assign4430_e4835_d_n9;
        locals.var_ibcp_dn10 = assign4430_e4835_d_n10;
        locals.var_ibcp_dn11 = assign4430_e4835_d_n11;

        let (assign4440_e4840, assign4440_e4840_d_n4, assign4440_e4840_d_n5, assign4440_e4840_d_n6, assign4440_e4840_d_n7, assign4440_e4840_d_n8, assign4440_e4840_d_n9, assign4440_e4840_d_n10, assign4440_e4840_d_n11,) = {
    if (locals.var_guard132 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibcp, locals.var_ibcp_dn4, locals.var_ibcp_dn5, locals.var_ibcp_dn6, locals.var_ibcp_dn7, locals.var_ibcp_dn8, locals.var_ibcp_dn9, locals.var_ibcp_dn10, locals.var_ibcp_dn11,)
    }
};
        locals.var_ibcp = assign4440_e4840;
        locals.var_ibcp_dn4 = assign4440_e4840_d_n4;
        locals.var_ibcp_dn5 = assign4440_e4840_d_n5;
        locals.var_ibcp_dn6 = assign4440_e4840_d_n6;
        locals.var_ibcp_dn7 = assign4440_e4840_d_n7;
        locals.var_ibcp_dn8 = assign4440_e4840_d_n8;
        locals.var_ibcp_dn9 = assign4440_e4840_d_n9;
        locals.var_ibcp_dn10 = assign4440_e4840_d_n10;
        locals.var_ibcp_dn11 = assign4440_e4840_d_n11;

        let assign4450_e4843: f64 = (locals.var_ibe * locals.var_vbei);
        let assign4450_e4846: f64 = (locals.var_ibc * locals.var_vbci);
        let assign4450_e4847: f64 = (assign4450_e4843 + assign4450_e4846);
        let assign4450_e4850: f64 = (locals.var_itxf - locals.var_itzr);
        let assign4450_e4852: f64 = (assign4450_e4850 * locals.var_vcei);
        let assign4450_e4853: f64 = (assign4450_e4847 + assign4450_e4852);
        let assign4450_e4856: f64 = (locals.var_ibex * locals.var_vbex);
        let assign4450_e4857: f64 = (assign4450_e4853 + assign4450_e4856);
        let assign4450_e4860: f64 = (locals.var_ibep * locals.var_vbep);
        let assign4450_e4861: f64 = (assign4450_e4857 + assign4450_e4860);
        let assign4450_e4864: f64 = (locals.var_irs * locals.var_vrs);
        let assign4450_e4865: f64 = (assign4450_e4861 + assign4450_e4864);
        let assign4450_e4868: f64 = (locals.var_ibcp * locals.var_vbcp);
        let assign4450_e4869: f64 = (assign4450_e4865 + assign4450_e4868);
        let assign4450_e4872: f64 = (locals.var_iccp * locals.var_vcep);
        let assign4450_e4873: f64 = (assign4450_e4869 + assign4450_e4872);
        let assign4450_e4876: f64 = (locals.var_ircx * locals.var_vrcx);
        let assign4450_e4877: f64 = (assign4450_e4873 + assign4450_e4876);
        let assign4450_e4880: f64 = (locals.var_irci * locals.var_vrci);
        let assign4450_e4881: f64 = (assign4450_e4877 + assign4450_e4880);
        let assign4450_e4884: f64 = (locals.var_irbx * locals.var_vrbx);
        let assign4450_e4885: f64 = (assign4450_e4881 + assign4450_e4884);
        let assign4450_e4888: f64 = (locals.var_irbi * locals.var_vrbi);
        let assign4450_e4889: f64 = (assign4450_e4885 + assign4450_e4888);
        let assign4450_e4892: f64 = (locals.var_ire * locals.var_vre);
        let assign4450_e4893: f64 = (assign4450_e4889 + assign4450_e4892);
        let assign4450_e4896: f64 = (locals.var_irbp * locals.var_vrbp);
        let assign4450_e4897: f64 = (assign4450_e4893 + assign4450_e4896);
        locals.var_power = assign4450_e4897;
        locals.var_power_dn0 = ((locals.var_ircx_dn0 * locals.var_vrcx) + (locals.var_ircx * locals.var_vrcx_dn0));
        locals.var_power_dn1 = ((locals.var_irbx_dn1 * locals.var_vrbx) + (locals.var_irbx * locals.var_vrbx_dn1));
        locals.var_power_dn2 = ((locals.var_ire_dn2 * locals.var_vre) + (locals.var_ire * locals.var_vre_dn2));
        locals.var_power_dn3 = ((locals.var_irs_dn3 * locals.var_vrs) + (locals.var_irs * locals.var_vrs_dn3));
        locals.var_power_dn4 = ((((((((((((((locals.var_ibe_dn4 * locals.var_vbei) + (locals.var_ibc_dn4 * locals.var_vbci)) + ((-locals.var_itzr_dn4) * locals.var_vcei)) + (locals.var_ibex_dn4 * locals.var_vbex)) + (locals.var_ibep_dn4 * locals.var_vbep)) + (locals.var_irs_dn4 * locals.var_vrs)) + (locals.var_ibcp_dn4 * locals.var_vbcp)) + (locals.var_iccp_dn4 * locals.var_vcep)) + (locals.var_ircx_dn4 * locals.var_vrcx)) + (locals.var_irci_dn4 * locals.var_vrci)) + (locals.var_irbx_dn4 * locals.var_vrbx)) + (locals.var_irbi_dn4 * locals.var_vrbi)) + (locals.var_ire_dn4 * locals.var_vre)) + (locals.var_irbp_dn4 * locals.var_vrbp));
        locals.var_power_dn5 = (((((((((((locals.var_ibe_dn5 * locals.var_vbei) + (locals.var_ibc_dn5 * locals.var_vbci)) + ((-locals.var_itzr_dn5) * locals.var_vcei)) + (locals.var_ibex_dn5 * locals.var_vbex)) + (locals.var_ibep_dn5 * locals.var_vbep)) + (locals.var_ibcp_dn5 * locals.var_vbcp)) + (locals.var_iccp_dn5 * locals.var_vcep)) + ((locals.var_ircx_dn5 * locals.var_vrcx) + (locals.var_ircx * locals.var_vrcx_dn5))) + ((locals.var_irci_dn5 * locals.var_vrci) + (locals.var_irci * locals.var_vrci_dn5))) + (locals.var_irbi_dn5 * locals.var_vrbi)) + ((locals.var_irbp_dn5 * locals.var_vrbp) + (locals.var_irbp * locals.var_vrbp_dn5)));
        locals.var_power_dn6 = ((((((((((locals.var_ibe_dn6 * locals.var_vbei) + ((locals.var_ibc_dn6 * locals.var_vbci) + (locals.var_ibc * locals.var_vbci_dn6))) + (((-locals.var_itzr_dn6) * locals.var_vcei) + (assign4450_e4850 * locals.var_vcei_dn6))) + (locals.var_ibex_dn6 * locals.var_vbex)) + (locals.var_ibep_dn6 * locals.var_vbep)) + (locals.var_ibcp_dn6 * locals.var_vbcp)) + (locals.var_iccp_dn6 * locals.var_vcep)) + ((locals.var_irci_dn6 * locals.var_vrci) + (locals.var_irci * locals.var_vrci_dn6))) + (locals.var_irbi_dn6 * locals.var_vrbi)) + (locals.var_irbp_dn6 * locals.var_vrbp));
        locals.var_power_dn7 = (((((((((((locals.var_ibe_dn7 * locals.var_vbei) + (locals.var_ibc_dn7 * locals.var_vbci)) + ((-locals.var_itzr_dn7) * locals.var_vcei)) + ((locals.var_ibex_dn7 * locals.var_vbex) + (locals.var_ibex * locals.var_vbex_dn7))) + ((locals.var_ibep_dn7 * locals.var_vbep) + (locals.var_ibep * locals.var_vbep_dn7))) + (locals.var_ibcp_dn7 * locals.var_vbcp)) + ((locals.var_iccp_dn7 * locals.var_vcep) + (locals.var_iccp * locals.var_vcep_dn7))) + (locals.var_irci_dn7 * locals.var_vrci)) + ((locals.var_irbx_dn7 * locals.var_vrbx) + (locals.var_irbx * locals.var_vrbx_dn7))) + ((locals.var_irbi_dn7 * locals.var_vrbi) + (locals.var_irbi * locals.var_vrbi_dn7))) + (locals.var_irbp_dn7 * locals.var_vrbp));
        locals.var_power_dn8 = (((((((((((locals.var_ibe_dn8 * locals.var_vbei) + (locals.var_ibe * locals.var_vbei_dn8)) + ((locals.var_ibc_dn8 * locals.var_vbci) + (locals.var_ibc * locals.var_vbci_dn8))) + ((-locals.var_itzr_dn8) * locals.var_vcei)) + (locals.var_ibex_dn8 * locals.var_vbex)) + (locals.var_ibep_dn8 * locals.var_vbep)) + (locals.var_ibcp_dn8 * locals.var_vbcp)) + (locals.var_iccp_dn8 * locals.var_vcep)) + (locals.var_irci_dn8 * locals.var_vrci)) + ((locals.var_irbi_dn8 * locals.var_vrbi) + (locals.var_irbi * locals.var_vrbi_dn8))) + (locals.var_irbp_dn8 * locals.var_vrbp));
        locals.var_power_dn9 = ((((((((((((locals.var_ibe_dn9 * locals.var_vbei) + (locals.var_ibe * locals.var_vbei_dn9)) + (locals.var_ibc_dn9 * locals.var_vbci)) + (((-locals.var_itzr_dn9) * locals.var_vcei) + (assign4450_e4850 * locals.var_vcei_dn9))) + ((locals.var_ibex_dn9 * locals.var_vbex) + (locals.var_ibex * locals.var_vbex_dn9))) + (locals.var_ibep_dn9 * locals.var_vbep)) + (locals.var_ibcp_dn9 * locals.var_vbcp)) + (locals.var_iccp_dn9 * locals.var_vcep)) + (locals.var_irci_dn9 * locals.var_vrci)) + (locals.var_irbi_dn9 * locals.var_vrbi)) + ((locals.var_ire_dn9 * locals.var_vre) + (locals.var_ire * locals.var_vre_dn9))) + (locals.var_irbp_dn9 * locals.var_vrbp));
        locals.var_power_dn10 = ((((((((((locals.var_ibe_dn10 * locals.var_vbei) + (locals.var_ibc_dn10 * locals.var_vbci)) + ((-locals.var_itzr_dn10) * locals.var_vcei)) + (locals.var_ibex_dn10 * locals.var_vbex)) + ((locals.var_ibep_dn10 * locals.var_vbep) + (locals.var_ibep * locals.var_vbep_dn10))) + ((locals.var_ibcp_dn10 * locals.var_vbcp) + (locals.var_ibcp * locals.var_vbcp_dn10))) + (locals.var_iccp_dn10 * locals.var_vcep)) + (locals.var_irci_dn10 * locals.var_vrci)) + (locals.var_irbi_dn10 * locals.var_vrbi)) + ((locals.var_irbp_dn10 * locals.var_vrbp) + (locals.var_irbp * locals.var_vrbp_dn10)));
        locals.var_power_dn11 = (((((((((((locals.var_ibe_dn11 * locals.var_vbei) + (locals.var_ibc_dn11 * locals.var_vbci)) + ((-locals.var_itzr_dn11) * locals.var_vcei)) + (locals.var_ibex_dn11 * locals.var_vbex)) + (locals.var_ibep_dn11 * locals.var_vbep)) + ((locals.var_irs_dn11 * locals.var_vrs) + (locals.var_irs * locals.var_vrs_dn11))) + ((locals.var_ibcp_dn11 * locals.var_vbcp) + (locals.var_ibcp * locals.var_vbcp_dn11))) + ((locals.var_iccp_dn11 * locals.var_vcep) + (locals.var_iccp * locals.var_vcep_dn11))) + (locals.var_irci_dn11 * locals.var_vrci)) + (locals.var_irbi_dn11 * locals.var_vrbi)) + (locals.var_irbp_dn11 * locals.var_vrbp));
        locals.var_power_dn13 = ((locals.var_ibc_dn13 * locals.var_vbci) + (locals.var_itxf_dn13 * locals.var_vcei));

        let assign4460_e4899: f64 = (-p.p2);
        let assign4460_e4901: f64 = (assign4460_e4899 * locals.var_power);
        locals.var_ith = assign4460_e4901;
        locals.var_ith_dn0 = (assign4460_e4899 * locals.var_power_dn0);
        locals.var_ith_dn1 = (assign4460_e4899 * locals.var_power_dn1);
        locals.var_ith_dn2 = (assign4460_e4899 * locals.var_power_dn2);
        locals.var_ith_dn3 = (assign4460_e4899 * locals.var_power_dn3);
        locals.var_ith_dn4 = (assign4460_e4899 * locals.var_power_dn4);
        locals.var_ith_dn5 = (assign4460_e4899 * locals.var_power_dn5);
        locals.var_ith_dn6 = (assign4460_e4899 * locals.var_power_dn6);
        locals.var_ith_dn7 = (assign4460_e4899 * locals.var_power_dn7);
        locals.var_ith_dn8 = (assign4460_e4899 * locals.var_power_dn8);
        locals.var_ith_dn9 = (assign4460_e4899 * locals.var_power_dn9);
        locals.var_ith_dn10 = (assign4460_e4899 * locals.var_power_dn10);
        locals.var_ith_dn11 = (assign4460_e4899 * locals.var_power_dn11);
        locals.var_ith_dn13 = (assign4460_e4899 * locals.var_power_dn13);

        let assign4470_e4904: f64 = (locals.var_dt_et * locals.var_gth);
        locals.var_irth = assign4470_e4904;
        locals.var_irth_dn4 = ((locals.var_dt_et_dn4 * locals.var_gth) + (locals.var_dt_et * locals.var_gth_dn4));

        let assign4480_e4907: f64 = (locals.var_vxf2 - locals.var_itzf);
        locals.var_ixf1 = assign4480_e4907;
        locals.var_ixf1_dn4 = (-locals.var_itzf_dn4);
        locals.var_ixf1_dn5 = (-locals.var_itzf_dn5);
        locals.var_ixf1_dn6 = (-locals.var_itzf_dn6);
        locals.var_ixf1_dn7 = (-locals.var_itzf_dn7);
        locals.var_ixf1_dn8 = (-locals.var_itzf_dn8);
        locals.var_ixf1_dn9 = (-locals.var_itzf_dn9);
        locals.var_ixf1_dn10 = (-locals.var_itzf_dn10);
        locals.var_ixf1_dn11 = (-locals.var_itzf_dn11);
        locals.var_ixf1_dn13 = locals.var_vxf2_dn13;

        let assign4500_e4914: f64 = (locals.var_gminmod * locals.var_vbei);
        let assign4500_e4915: f64 = (locals.var_ibe + assign4500_e4914);
        locals.var_ibe = assign4500_e4915;
        locals.var_ibe_dn4 = locals.var_ibe_dn4;
        locals.var_ibe_dn5 = locals.var_ibe_dn5;
        locals.var_ibe_dn6 = locals.var_ibe_dn6;
        locals.var_ibe_dn7 = locals.var_ibe_dn7;
        locals.var_ibe_dn8 = (locals.var_ibe_dn8 + (locals.var_gminmod * locals.var_vbei_dn8));
        locals.var_ibe_dn9 = (locals.var_ibe_dn9 + (locals.var_gminmod * locals.var_vbei_dn9));
        locals.var_ibe_dn10 = locals.var_ibe_dn10;
        locals.var_ibe_dn11 = locals.var_ibe_dn11;

        let assign4510_e4919: f64 = (locals.var_gminmod * locals.var_vbex);
        let assign4510_e4920: f64 = (locals.var_ibex + assign4510_e4919);
        locals.var_ibex = assign4510_e4920;
        locals.var_ibex_dn4 = locals.var_ibex_dn4;
        locals.var_ibex_dn5 = locals.var_ibex_dn5;
        locals.var_ibex_dn6 = locals.var_ibex_dn6;
        locals.var_ibex_dn7 = (locals.var_ibex_dn7 + (locals.var_gminmod * locals.var_vbex_dn7));
        locals.var_ibex_dn8 = locals.var_ibex_dn8;
        locals.var_ibex_dn9 = (locals.var_ibex_dn9 + (locals.var_gminmod * locals.var_vbex_dn9));
        locals.var_ibex_dn10 = locals.var_ibex_dn10;
        locals.var_ibex_dn11 = locals.var_ibex_dn11;

        let assign4520_e4924: f64 = (locals.var_gminmod * locals.var_vbep);
        let assign4520_e4925: f64 = (locals.var_ibep + assign4520_e4924);
        locals.var_ibep = assign4520_e4925;
        locals.var_ibep_dn4 = locals.var_ibep_dn4;
        locals.var_ibep_dn5 = locals.var_ibep_dn5;
        locals.var_ibep_dn6 = locals.var_ibep_dn6;
        locals.var_ibep_dn7 = (locals.var_ibep_dn7 + (locals.var_gminmod * locals.var_vbep_dn7));
        locals.var_ibep_dn8 = locals.var_ibep_dn8;
        locals.var_ibep_dn9 = locals.var_ibep_dn9;
        locals.var_ibep_dn10 = (locals.var_ibep_dn10 + (locals.var_gminmod * locals.var_vbep_dn10));
        locals.var_ibep_dn11 = locals.var_ibep_dn11;

        let assign4530_e4929: f64 = (locals.var_gminmod * locals.var_vbci);
        let assign4530_e4930: f64 = (locals.var_ibc + assign4530_e4929);
        locals.var_ibc = assign4530_e4930;
        locals.var_ibc_dn4 = locals.var_ibc_dn4;
        locals.var_ibc_dn5 = locals.var_ibc_dn5;
        locals.var_ibc_dn6 = (locals.var_ibc_dn6 + (locals.var_gminmod * locals.var_vbci_dn6));
        locals.var_ibc_dn7 = locals.var_ibc_dn7;
        locals.var_ibc_dn8 = (locals.var_ibc_dn8 + (locals.var_gminmod * locals.var_vbci_dn8));
        locals.var_ibc_dn9 = locals.var_ibc_dn9;
        locals.var_ibc_dn10 = locals.var_ibc_dn10;
        locals.var_ibc_dn11 = locals.var_ibc_dn11;
        locals.var_ibc_dn13 = locals.var_ibc_dn13;

        let assign4540_e4934: f64 = (locals.var_gminmod * locals.var_vbxcx);
        let assign4540_e4935: f64 = (locals.var_igcx + assign4540_e4934);
        locals.var_igcx = assign4540_e4935;
        locals.var_igcx_dn0 = locals.var_igcx_dn0;
        locals.var_igcx_dn4 = locals.var_igcx_dn4;
        locals.var_igcx_dn5 = (locals.var_igcx_dn5 + (locals.var_gminmod * locals.var_vbxcx_dn5));
        locals.var_igcx_dn6 = locals.var_igcx_dn6;
        locals.var_igcx_dn7 = (locals.var_igcx_dn7 + (locals.var_gminmod * locals.var_vbxcx_dn7));
        locals.var_igcx_dn8 = locals.var_igcx_dn8;

        let assign4550_e4939: f64 = (locals.var_gminmod * locals.var_vbcp);
        let assign4550_e4940: f64 = (locals.var_ibcp + assign4550_e4939);
        locals.var_ibcp = assign4550_e4940;
        locals.var_ibcp_dn4 = locals.var_ibcp_dn4;
        locals.var_ibcp_dn5 = locals.var_ibcp_dn5;
        locals.var_ibcp_dn6 = locals.var_ibcp_dn6;
        locals.var_ibcp_dn7 = locals.var_ibcp_dn7;
        locals.var_ibcp_dn8 = locals.var_ibcp_dn8;
        locals.var_ibcp_dn9 = locals.var_ibcp_dn9;
        locals.var_ibcp_dn10 = (locals.var_ibcp_dn10 + (locals.var_gminmod * locals.var_vbcp_dn10));
        locals.var_ibcp_dn11 = (locals.var_ibcp_dn11 + (locals.var_gminmod * locals.var_vbcp_dn11));

        let assign4560_e4943: f64 = locals.var_vbictype;
        let assign4560_e4945: f64 = (assign4560_e4943 * locals.var_ibe);
        locals.var_ibe = assign4560_e4945;
        locals.var_ibe_dn4 = (assign4560_e4943 * locals.var_ibe_dn4);
        locals.var_ibe_dn5 = (assign4560_e4943 * locals.var_ibe_dn5);
        locals.var_ibe_dn6 = (assign4560_e4943 * locals.var_ibe_dn6);
        locals.var_ibe_dn7 = (assign4560_e4943 * locals.var_ibe_dn7);
        locals.var_ibe_dn8 = (assign4560_e4943 * locals.var_ibe_dn8);
        locals.var_ibe_dn9 = (assign4560_e4943 * locals.var_ibe_dn9);
        locals.var_ibe_dn10 = (assign4560_e4943 * locals.var_ibe_dn10);
        locals.var_ibe_dn11 = (assign4560_e4943 * locals.var_ibe_dn11);

        let assign4570_e4948: f64 = locals.var_vbictype;
        let assign4570_e4950: f64 = (assign4570_e4948 * locals.var_ibex);
        locals.var_ibex = assign4570_e4950;
        locals.var_ibex_dn4 = (assign4570_e4948 * locals.var_ibex_dn4);
        locals.var_ibex_dn5 = (assign4570_e4948 * locals.var_ibex_dn5);
        locals.var_ibex_dn6 = (assign4570_e4948 * locals.var_ibex_dn6);
        locals.var_ibex_dn7 = (assign4570_e4948 * locals.var_ibex_dn7);
        locals.var_ibex_dn8 = (assign4570_e4948 * locals.var_ibex_dn8);
        locals.var_ibex_dn9 = (assign4570_e4948 * locals.var_ibex_dn9);
        locals.var_ibex_dn10 = (assign4570_e4948 * locals.var_ibex_dn10);
        locals.var_ibex_dn11 = (assign4570_e4948 * locals.var_ibex_dn11);

        let assign4580_e4953: f64 = locals.var_vbictype;
        let assign4580_e4955: f64 = (assign4580_e4953 * locals.var_itzf);
        locals.var_itzf = assign4580_e4955;
        locals.var_itzf_dn4 = (assign4580_e4953 * locals.var_itzf_dn4);
        locals.var_itzf_dn5 = (assign4580_e4953 * locals.var_itzf_dn5);
        locals.var_itzf_dn6 = (assign4580_e4953 * locals.var_itzf_dn6);
        locals.var_itzf_dn7 = (assign4580_e4953 * locals.var_itzf_dn7);
        locals.var_itzf_dn8 = (assign4580_e4953 * locals.var_itzf_dn8);
        locals.var_itzf_dn9 = (assign4580_e4953 * locals.var_itzf_dn9);
        locals.var_itzf_dn10 = (assign4580_e4953 * locals.var_itzf_dn10);
        locals.var_itzf_dn11 = (assign4580_e4953 * locals.var_itzf_dn11);

        let assign4590_e4958: f64 = locals.var_vbictype;
        let assign4590_e4960: f64 = (assign4590_e4958 * locals.var_itxf);
        locals.var_itxf = assign4590_e4960;
        locals.var_itxf_dn13 = (assign4590_e4958 * locals.var_itxf_dn13);

        let assign4600_e4963: f64 = locals.var_vbictype;
        let assign4600_e4965: f64 = (assign4600_e4963 * locals.var_itzr);
        locals.var_itzr = assign4600_e4965;
        locals.var_itzr_dn4 = (assign4600_e4963 * locals.var_itzr_dn4);
        locals.var_itzr_dn5 = (assign4600_e4963 * locals.var_itzr_dn5);
        locals.var_itzr_dn6 = (assign4600_e4963 * locals.var_itzr_dn6);
        locals.var_itzr_dn7 = (assign4600_e4963 * locals.var_itzr_dn7);
        locals.var_itzr_dn8 = (assign4600_e4963 * locals.var_itzr_dn8);
        locals.var_itzr_dn9 = (assign4600_e4963 * locals.var_itzr_dn9);
        locals.var_itzr_dn10 = (assign4600_e4963 * locals.var_itzr_dn10);
        locals.var_itzr_dn11 = (assign4600_e4963 * locals.var_itzr_dn11);

        let assign4610_e4968: f64 = locals.var_vbictype;
        let assign4610_e4970: f64 = (assign4610_e4968 * locals.var_ibc);
        locals.var_ibc = assign4610_e4970;
        locals.var_ibc_dn4 = (assign4610_e4968 * locals.var_ibc_dn4);
        locals.var_ibc_dn5 = (assign4610_e4968 * locals.var_ibc_dn5);
        locals.var_ibc_dn6 = (assign4610_e4968 * locals.var_ibc_dn6);
        locals.var_ibc_dn7 = (assign4610_e4968 * locals.var_ibc_dn7);
        locals.var_ibc_dn8 = (assign4610_e4968 * locals.var_ibc_dn8);
        locals.var_ibc_dn9 = (assign4610_e4968 * locals.var_ibc_dn9);
        locals.var_ibc_dn10 = (assign4610_e4968 * locals.var_ibc_dn10);
        locals.var_ibc_dn11 = (assign4610_e4968 * locals.var_ibc_dn11);
        locals.var_ibc_dn13 = (assign4610_e4968 * locals.var_ibc_dn13);

        let assign4620_e4973: f64 = locals.var_vbictype;
        let assign4620_e4975: f64 = (assign4620_e4973 * locals.var_igcx);
        locals.var_igcx = assign4620_e4975;
        locals.var_igcx_dn0 = (assign4620_e4973 * locals.var_igcx_dn0);
        locals.var_igcx_dn4 = (assign4620_e4973 * locals.var_igcx_dn4);
        locals.var_igcx_dn5 = (assign4620_e4973 * locals.var_igcx_dn5);
        locals.var_igcx_dn6 = (assign4620_e4973 * locals.var_igcx_dn6);
        locals.var_igcx_dn7 = (assign4620_e4973 * locals.var_igcx_dn7);
        locals.var_igcx_dn8 = (assign4620_e4973 * locals.var_igcx_dn8);

        let assign4630_e4978: f64 = locals.var_vbictype;
        let assign4630_e4980: f64 = (assign4630_e4978 * locals.var_ibep);
        locals.var_ibep = assign4630_e4980;
        locals.var_ibep_dn4 = (assign4630_e4978 * locals.var_ibep_dn4);
        locals.var_ibep_dn5 = (assign4630_e4978 * locals.var_ibep_dn5);
        locals.var_ibep_dn6 = (assign4630_e4978 * locals.var_ibep_dn6);
        locals.var_ibep_dn7 = (assign4630_e4978 * locals.var_ibep_dn7);
        locals.var_ibep_dn8 = (assign4630_e4978 * locals.var_ibep_dn8);
        locals.var_ibep_dn9 = (assign4630_e4978 * locals.var_ibep_dn9);
        locals.var_ibep_dn10 = (assign4630_e4978 * locals.var_ibep_dn10);
        locals.var_ibep_dn11 = (assign4630_e4978 * locals.var_ibep_dn11);

        let assign4640_e4983: f64 = locals.var_ircx;
        locals.var_ircx = assign4640_e4983;
        locals.var_ircx_dn0 = locals.var_ircx_dn0;
        locals.var_ircx_dn4 = locals.var_ircx_dn4;
        locals.var_ircx_dn5 = locals.var_ircx_dn5;

        let assign4650_e4986: f64 = locals.var_vbictype;
        let assign4650_e4988: f64 = (assign4650_e4986 * locals.var_irci);
        locals.var_irci = assign4650_e4988;
        locals.var_irci_dn4 = (assign4650_e4986 * locals.var_irci_dn4);
        locals.var_irci_dn5 = (assign4650_e4986 * locals.var_irci_dn5);
        locals.var_irci_dn6 = (assign4650_e4986 * locals.var_irci_dn6);
        locals.var_irci_dn7 = (assign4650_e4986 * locals.var_irci_dn7);
        locals.var_irci_dn8 = (assign4650_e4986 * locals.var_irci_dn8);
        locals.var_irci_dn9 = (assign4650_e4986 * locals.var_irci_dn9);
        locals.var_irci_dn10 = (assign4650_e4986 * locals.var_irci_dn10);
        locals.var_irci_dn11 = (assign4650_e4986 * locals.var_irci_dn11);

        let assign4660_e4991: f64 = locals.var_irbx;
        locals.var_irbx = assign4660_e4991;
        locals.var_irbx_dn1 = locals.var_irbx_dn1;
        locals.var_irbx_dn4 = locals.var_irbx_dn4;
        locals.var_irbx_dn7 = locals.var_irbx_dn7;

        let assign4670_e4994: f64 = locals.var_irbi;
        locals.var_irbi = assign4670_e4994;
        locals.var_irbi_dn4 = locals.var_irbi_dn4;
        locals.var_irbi_dn5 = locals.var_irbi_dn5;
        locals.var_irbi_dn6 = locals.var_irbi_dn6;
        locals.var_irbi_dn7 = locals.var_irbi_dn7;
        locals.var_irbi_dn8 = locals.var_irbi_dn8;
        locals.var_irbi_dn9 = locals.var_irbi_dn9;
        locals.var_irbi_dn10 = locals.var_irbi_dn10;
        locals.var_irbi_dn11 = locals.var_irbi_dn11;

        let assign4680_e4997: f64 = locals.var_ire;
        locals.var_ire = assign4680_e4997;
        locals.var_ire_dn2 = locals.var_ire_dn2;
        locals.var_ire_dn4 = locals.var_ire_dn4;
        locals.var_ire_dn9 = locals.var_ire_dn9;

        let assign4690_e5000: f64 = locals.var_irbp;
        locals.var_irbp = assign4690_e5000;
        locals.var_irbp_dn4 = locals.var_irbp_dn4;
        locals.var_irbp_dn5 = locals.var_irbp_dn5;
        locals.var_irbp_dn6 = locals.var_irbp_dn6;
        locals.var_irbp_dn7 = locals.var_irbp_dn7;
        locals.var_irbp_dn8 = locals.var_irbp_dn8;
        locals.var_irbp_dn9 = locals.var_irbp_dn9;
        locals.var_irbp_dn10 = locals.var_irbp_dn10;
        locals.var_irbp_dn11 = locals.var_irbp_dn11;

        let assign4700_e5003: f64 = locals.var_vbictype;
        let assign4700_e5005: f64 = (assign4700_e5003 * locals.var_ibcp);
        locals.var_ibcp = assign4700_e5005;
        locals.var_ibcp_dn4 = (assign4700_e5003 * locals.var_ibcp_dn4);
        locals.var_ibcp_dn5 = (assign4700_e5003 * locals.var_ibcp_dn5);
        locals.var_ibcp_dn6 = (assign4700_e5003 * locals.var_ibcp_dn6);
        locals.var_ibcp_dn7 = (assign4700_e5003 * locals.var_ibcp_dn7);
        locals.var_ibcp_dn8 = (assign4700_e5003 * locals.var_ibcp_dn8);
        locals.var_ibcp_dn9 = (assign4700_e5003 * locals.var_ibcp_dn9);
        locals.var_ibcp_dn10 = (assign4700_e5003 * locals.var_ibcp_dn10);
        locals.var_ibcp_dn11 = (assign4700_e5003 * locals.var_ibcp_dn11);

        let assign4710_e5008: f64 = locals.var_vbictype;
        let assign4710_e5010: f64 = (assign4710_e5008 * locals.var_iccp);
        locals.var_iccp = assign4710_e5010;
        locals.var_iccp_dn4 = (assign4710_e5008 * locals.var_iccp_dn4);
        locals.var_iccp_dn5 = (assign4710_e5008 * locals.var_iccp_dn5);
        locals.var_iccp_dn6 = (assign4710_e5008 * locals.var_iccp_dn6);
        locals.var_iccp_dn7 = (assign4710_e5008 * locals.var_iccp_dn7);
        locals.var_iccp_dn8 = (assign4710_e5008 * locals.var_iccp_dn8);
        locals.var_iccp_dn9 = (assign4710_e5008 * locals.var_iccp_dn9);
        locals.var_iccp_dn10 = (assign4710_e5008 * locals.var_iccp_dn10);
        locals.var_iccp_dn11 = (assign4710_e5008 * locals.var_iccp_dn11);

        let assign4720_e5013: f64 = locals.var_irs;
        locals.var_irs = assign4720_e5013;
        locals.var_irs_dn3 = locals.var_irs_dn3;
        locals.var_irs_dn4 = locals.var_irs_dn4;
        locals.var_irs_dn11 = locals.var_irs_dn11;

        let assign4730_e5016: f64 = locals.var_ith;
        locals.var_ith = assign4730_e5016;
        locals.var_ith_dn0 = locals.var_ith_dn0;
        locals.var_ith_dn1 = locals.var_ith_dn1;
        locals.var_ith_dn2 = locals.var_ith_dn2;
        locals.var_ith_dn3 = locals.var_ith_dn3;
        locals.var_ith_dn4 = locals.var_ith_dn4;
        locals.var_ith_dn5 = locals.var_ith_dn5;
        locals.var_ith_dn6 = locals.var_ith_dn6;
        locals.var_ith_dn7 = locals.var_ith_dn7;
        locals.var_ith_dn8 = locals.var_ith_dn8;
        locals.var_ith_dn9 = locals.var_ith_dn9;
        locals.var_ith_dn10 = locals.var_ith_dn10;
        locals.var_ith_dn11 = locals.var_ith_dn11;
        locals.var_ith_dn13 = locals.var_ith_dn13;

        let assign4740_e5019: f64 = locals.var_irth;
        locals.var_irth = assign4740_e5019;
        locals.var_irth_dn4 = locals.var_irth_dn4;

        let assign4750_e5022: f64 = if p.p49 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard135 = assign4750_e5022;

        let (assign4760_e5029, assign4760_e5029_d_n4,) = {
    if (locals.var_guard135 != 0.0) {
        let assign4760_e5025: f64 = (-locals.var_ps_t);
        let assign4760_e5027: f64 = (assign4760_e5025 * p.p34);
        (assign4760_e5027, ((-locals.var_ps_t_dn4) * p.p34),)
    } else {
        (locals.var_dv0__blk136, locals.var_dv0__blk136_dn4,)
    }
};
        locals.var_dv0__blk136 = assign4760_e5029;
        locals.var_dv0__blk136_dn4 = assign4760_e5029_d_n4;

        let assign4770_e5032: f64 = if p.p52 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign4770_e5032;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4780_e5040, assign4780_e5040_d_n4, assign4780_e5040_d_n10, assign4780_e5040_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign4780_e5038: f64 = (locals.var_vbcp + locals.var_dv0__blk136);
        (assign4780_e5038, locals.var_dv0__blk136_dn4, locals.var_vbcp_dn10, locals.var_vbcp_dn11,)
    } else {
        (locals.var_dvh__blk137, locals.var_dvh__blk137_dn4, locals.var_dvh__blk137_dn10, locals.var_dvh__blk137_dn11,)
    }
};
        locals.var_dvh__blk137 = assign4780_e5040;
        locals.var_dvh__blk137_dn4 = assign4780_e5040_d_n4;
        locals.var_dvh__blk137_dn10 = assign4780_e5040_d_n10;
        locals.var_dvh__blk137_dn11 = assign4780_e5040_d_n11;

        let assign4790_e5043: f64 = if locals.var_dvh__blk137 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign4790_e5043;

        let (assign4800_e5056,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 != 0.0)) {
        let assign4800_e5051: f64 = (1.0 - p.p34);
        let assign4800_e5053: f64 = (-p.p51);
        let assign4800_e5054: f64 = (assign4800_e5051).powf(assign4800_e5053);
        (assign4800_e5054,)
    } else {
        (locals.var_pwq__blk138,)
    }
};
        locals.var_pwq__blk138 = assign4800_e5056;

        let (assign4810_e5076, assign4810_e5076_d_n4, assign4810_e5076_d_n10, assign4810_e5076_d_n11,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 != 0.0)) {
        let assign4810_e5067: f64 = (1.0 - p.p34);
        let assign4810_e5068: f64 = (locals.var_pwq__blk138 * assign4810_e5067);
        let assign4810_e5069: f64 = (1.0 - assign4810_e5068);
        let assign4810_e5070: f64 = (locals.var_ps_t * assign4810_e5069);
        let assign4810_e5073: f64 = (1.0 - p.p51);
        let assign4810_e5074: f64 = (assign4810_e5070 / assign4810_e5073);
        (assign4810_e5074, ((locals.var_ps_t_dn4 * assign4810_e5069) / assign4810_e5073), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk139, locals.var_qlo__blk139_dn4, locals.var_qlo__blk139_dn10, locals.var_qlo__blk139_dn11,)
    }
};
        locals.var_qlo__blk139 = assign4810_e5076;
        locals.var_qlo__blk139_dn4 = assign4810_e5076_d_n4;
        locals.var_qlo__blk139_dn10 = assign4810_e5076_d_n10;
        locals.var_qlo__blk139_dn11 = assign4810_e5076_d_n11;

        let (assign4820_e5100, assign4820_e5100_d_n4, assign4820_e5100_d_n10, assign4820_e5100_d_n11,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 != 0.0)) {
        let assign4820_e5086: f64 = (0.5 * p.p51);
        let assign4820_e5088: f64 = (assign4820_e5086 * locals.var_dvh__blk137);
        let assign4820_e5092: f64 = (1.0 - p.p34);
        let assign4820_e5093: f64 = (locals.var_ps_t * assign4820_e5092);
        let assign4820_e5094: f64 = (assign4820_e5088 / assign4820_e5093);
        let assign4820_e5095: f64 = (1.0 + assign4820_e5094);
        let assign4820_e5096: f64 = (locals.var_dvh__blk137 * assign4820_e5095);
        let assign4820_e5098: f64 = (assign4820_e5096 * locals.var_pwq__blk138);
        (assign4820_e5098, (((locals.var_dvh__blk137_dn4 * assign4820_e5095) + (locals.var_dvh__blk137 * ((((assign4820_e5086 * locals.var_dvh__blk137_dn4) * assign4820_e5093) - (assign4820_e5088 * (locals.var_ps_t_dn4 * assign4820_e5092))) / (assign4820_e5093 * assign4820_e5093)))) * locals.var_pwq__blk138), (((locals.var_dvh__blk137_dn10 * assign4820_e5095) + (locals.var_dvh__blk137 * ((assign4820_e5086 * locals.var_dvh__blk137_dn10) / assign4820_e5093))) * locals.var_pwq__blk138), (((locals.var_dvh__blk137_dn11 * assign4820_e5095) + (locals.var_dvh__blk137 * ((assign4820_e5086 * locals.var_dvh__blk137_dn11) / assign4820_e5093))) * locals.var_pwq__blk138),)
    } else {
        (locals.var_qhi__blk140, locals.var_qhi__blk140_dn4, locals.var_qhi__blk140_dn10, locals.var_qhi__blk140_dn11,)
    }
};
        locals.var_qhi__blk140 = assign4820_e5100;
        locals.var_qhi__blk140_dn4 = assign4820_e5100_d_n4;
        locals.var_qhi__blk140_dn10 = assign4820_e5100_d_n10;
        locals.var_qhi__blk140_dn11 = assign4820_e5100_d_n11;

        let (assign4830_e5125, assign4830_e5125_d_n4, assign4830_e5125_d_n10, assign4830_e5125_d_n11,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 == 0.0)) {
        let assign4830_e5112: f64 = (locals.var_vbcp / locals.var_ps_t);
        let assign4830_e5113: f64 = (1.0 - assign4830_e5112);
        let assign4830_e5116: f64 = (1.0 - p.p51);
        let assign4830_e5117: f64 = (assign4830_e5113).powf(assign4830_e5116);
        let assign4830_e5118: f64 = (1.0 - assign4830_e5117);
        let assign4830_e5119: f64 = (locals.var_ps_t * assign4830_e5118);
        let assign4830_e5122: f64 = (1.0 - p.p51);
        let assign4830_e5123: f64 = (assign4830_e5119 / assign4830_e5122);
        (assign4830_e5123, (((locals.var_ps_t_dn4 * assign4830_e5118) + (locals.var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(-((locals.var_vbcp * locals.var_ps_t_dn4) / (locals.var_ps_t * locals.var_ps_t)))))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(-((locals.var_vbcp * locals.var_ps_t_dn4) / (locals.var_ps_t * locals.var_ps_t)))) / assign4830_e5113))) }))) / assign4830_e5122), ((locals.var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(locals.var_vbcp_dn10 / locals.var_ps_t)))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(locals.var_vbcp_dn10 / locals.var_ps_t)) / assign4830_e5113))) })) / assign4830_e5122), ((locals.var_ps_t * (-if 0.0 == 0.0 && ((assign4830_e5116) as f64).is_finite() && ((assign4830_e5116) as f64).fract() == 0.0 { if assign4830_e5116 == 0.0 { 0.0 } else { (assign4830_e5116 * ((assign4830_e5113).powf(assign4830_e5116 - 1.0) * (-(locals.var_vbcp_dn11 / locals.var_ps_t)))) } } else { (assign4830_e5117 * (assign4830_e5116 * ((-(locals.var_vbcp_dn11 / locals.var_ps_t)) / assign4830_e5113))) })) / assign4830_e5122),)
    } else {
        (locals.var_qlo__blk139, locals.var_qlo__blk139_dn4, locals.var_qlo__blk139_dn10, locals.var_qlo__blk139_dn11,)
    }
};
        locals.var_qlo__blk139 = assign4830_e5125;
        locals.var_qlo__blk139_dn4 = assign4830_e5125_d_n4;
        locals.var_qlo__blk139_dn10 = assign4830_e5125_d_n10;
        locals.var_qlo__blk139_dn11 = assign4830_e5125_d_n11;

        let (assign4840_e5134, assign4840_e5134_d_n4, assign4840_e5134_d_n10, assign4840_e5134_d_n11,) = {
    if (((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) && (locals.var_guard148 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk140, locals.var_qhi__blk140_dn4, locals.var_qhi__blk140_dn10, locals.var_qhi__blk140_dn11,)
    }
};
        locals.var_qhi__blk140 = assign4840_e5134;
        locals.var_qhi__blk140_dn4 = assign4840_e5134_d_n4;
        locals.var_qhi__blk140_dn10 = assign4840_e5134_d_n10;
        locals.var_qhi__blk140_dn11 = assign4840_e5134_d_n11;

        let (assign4850_e5142, assign4850_e5142_d_n4, assign4850_e5142_d_n10, assign4850_e5142_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign4850_e5140: f64 = (locals.var_qlo__blk139 + locals.var_qhi__blk140);
        (assign4850_e5140, (locals.var_qlo__blk139_dn4 + locals.var_qhi__blk140_dn4), (locals.var_qlo__blk139_dn10 + locals.var_qhi__blk140_dn10), (locals.var_qlo__blk139_dn11 + locals.var_qhi__blk140_dn11),)
    } else {
        (locals.var_qdbcp, locals.var_qdbcp_dn4, locals.var_qdbcp_dn10, locals.var_qdbcp_dn11,)
    }
};
        locals.var_qdbcp = assign4850_e5142;
        locals.var_qdbcp_dn4 = assign4850_e5142_d_n4;
        locals.var_qdbcp_dn10 = assign4850_e5142_d_n10;
        locals.var_qdbcp_dn11 = assign4850_e5142_d_n11;

        let (assign4860_e5158, assign4860_e5158_d_n4,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4860_e5149: f64 = (locals.var_dv0__blk136 * locals.var_dv0__blk136);
        let assign4860_e5152: f64 = (4.0 * p.p52);
        let assign4860_e5154: f64 = (assign4860_e5152 * p.p52);
        let assign4860_e5155: f64 = (assign4860_e5149 + assign4860_e5154);
        let assign4860_e5156: f64 = (assign4860_e5155).sqrt();
        (assign4860_e5156, (((locals.var_dv0__blk136_dn4 * locals.var_dv0__blk136) + (locals.var_dv0__blk136 * locals.var_dv0__blk136_dn4)) / (2.0 * assign4860_e5156)),)
    } else {
        (locals.var_mv0__blk141, locals.var_mv0__blk141_dn4,)
    }
};
        locals.var_mv0__blk141 = assign4860_e5158;
        locals.var_mv0__blk141_dn4 = assign4860_e5158_d_n4;

        let (assign4870_e5170, assign4870_e5170_d_n4,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4870_e5164: f64 = (-0.5);
        let assign4870_e5167: f64 = (locals.var_dv0__blk136 + locals.var_mv0__blk141);
        let assign4870_e5168: f64 = (assign4870_e5164 * assign4870_e5167);
        (assign4870_e5168, (assign4870_e5164 * (locals.var_dv0__blk136_dn4 + locals.var_mv0__blk141_dn4)),)
    } else {
        (locals.var_vl0__blk142, locals.var_vl0__blk142_dn4,)
    }
};
        locals.var_vl0__blk142 = assign4870_e5170;
        locals.var_vl0__blk142_dn4 = assign4870_e5170_d_n4;

        let (assign4880_e5192, assign4880_e5192_d_n4,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4880_e5176: f64 = (-locals.var_ps_t);
        let assign4880_e5180: f64 = (locals.var_vl0__blk142 / locals.var_ps_t);
        let assign4880_e5181: f64 = (1.0 - assign4880_e5180);
        let assign4880_e5184: f64 = (1.0 - p.p51);
        let assign4880_e5185: f64 = (assign4880_e5181).powf(assign4880_e5184);
        let assign4880_e5186: f64 = (assign4880_e5176 * assign4880_e5185);
        let assign4880_e5189: f64 = (1.0 - p.p51);
        let assign4880_e5190: f64 = (assign4880_e5186 / assign4880_e5189);
        (assign4880_e5190, ((((-locals.var_ps_t_dn4) * assign4880_e5185) + (assign4880_e5176 * if 0.0 == 0.0 && ((assign4880_e5184) as f64).is_finite() && ((assign4880_e5184) as f64).fract() == 0.0 { if assign4880_e5184 == 0.0 { 0.0 } else { (assign4880_e5184 * ((assign4880_e5181).powf(assign4880_e5184 - 1.0) * (-(((locals.var_vl0__blk142_dn4 * locals.var_ps_t) - (locals.var_vl0__blk142 * locals.var_ps_t_dn4)) / (locals.var_ps_t * locals.var_ps_t))))) } } else { (assign4880_e5185 * (assign4880_e5184 * ((-(((locals.var_vl0__blk142_dn4 * locals.var_ps_t) - (locals.var_vl0__blk142 * locals.var_ps_t_dn4)) / (locals.var_ps_t * locals.var_ps_t))) / assign4880_e5181))) })) / assign4880_e5189),)
    } else {
        (locals.var_q0__blk143, locals.var_q0__blk143_dn4,)
    }
};
        locals.var_q0__blk143 = assign4880_e5192;
        locals.var_q0__blk143_dn4 = assign4880_e5192_d_n4;

        let (assign4890_e5201, assign4890_e5201_d_n4, assign4890_e5201_d_n10, assign4890_e5201_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4890_e5199: f64 = (locals.var_vbcp + locals.var_dv0__blk136);
        (assign4890_e5199, locals.var_dv0__blk136_dn4, locals.var_vbcp_dn10, locals.var_vbcp_dn11,)
    } else {
        (locals.var_dv__blk144, locals.var_dv__blk144_dn4, locals.var_dv__blk144_dn10, locals.var_dv__blk144_dn11,)
    }
};
        locals.var_dv__blk144 = assign4890_e5201;
        locals.var_dv__blk144_dn4 = assign4890_e5201_d_n4;
        locals.var_dv__blk144_dn10 = assign4890_e5201_d_n10;
        locals.var_dv__blk144_dn11 = assign4890_e5201_d_n11;

        let (assign4900_e5217, assign4900_e5217_d_n4, assign4900_e5217_d_n10, assign4900_e5217_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4900_e5208: f64 = (locals.var_dv__blk144 * locals.var_dv__blk144);
        let assign4900_e5211: f64 = (4.0 * p.p52);
        let assign4900_e5213: f64 = (assign4900_e5211 * p.p52);
        let assign4900_e5214: f64 = (assign4900_e5208 + assign4900_e5213);
        let assign4900_e5215: f64 = (assign4900_e5214).sqrt();
        (assign4900_e5215, (((locals.var_dv__blk144_dn4 * locals.var_dv__blk144) + (locals.var_dv__blk144 * locals.var_dv__blk144_dn4)) / (2.0 * assign4900_e5215)), (((locals.var_dv__blk144_dn10 * locals.var_dv__blk144) + (locals.var_dv__blk144 * locals.var_dv__blk144_dn10)) / (2.0 * assign4900_e5215)), (((locals.var_dv__blk144_dn11 * locals.var_dv__blk144) + (locals.var_dv__blk144 * locals.var_dv__blk144_dn11)) / (2.0 * assign4900_e5215)),)
    } else {
        (locals.var_mv__blk145, locals.var_mv__blk145_dn4, locals.var_mv__blk145_dn10, locals.var_mv__blk145_dn11,)
    }
};
        locals.var_mv__blk145 = assign4900_e5217;
        locals.var_mv__blk145_dn4 = assign4900_e5217_d_n4;
        locals.var_mv__blk145_dn10 = assign4900_e5217_d_n10;
        locals.var_mv__blk145_dn11 = assign4900_e5217_d_n11;

        let (assign4910_e5230, assign4910_e5230_d_n4, assign4910_e5230_d_n10, assign4910_e5230_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4910_e5225: f64 = (locals.var_dv__blk144 - locals.var_mv__blk145);
        let assign4910_e5226: f64 = (0.5 * assign4910_e5225);
        let assign4910_e5228: f64 = (assign4910_e5226 - locals.var_dv0__blk136);
        (assign4910_e5228, ((0.5 * (locals.var_dv__blk144_dn4 - locals.var_mv__blk145_dn4)) - locals.var_dv0__blk136_dn4), (0.5 * (locals.var_dv__blk144_dn10 - locals.var_mv__blk145_dn10)), (0.5 * (locals.var_dv__blk144_dn11 - locals.var_mv__blk145_dn11)),)
    } else {
        (locals.var_vl__blk146, locals.var_vl__blk146_dn4, locals.var_vl__blk146_dn10, locals.var_vl__blk146_dn11,)
    }
};
        locals.var_vl__blk146 = assign4910_e5230;
        locals.var_vl__blk146_dn4 = assign4910_e5230_d_n4;
        locals.var_vl__blk146_dn10 = assign4910_e5230_d_n10;
        locals.var_vl__blk146_dn11 = assign4910_e5230_d_n11;

        let (assign4920_e5252, assign4920_e5252_d_n4, assign4920_e5252_d_n10, assign4920_e5252_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4920_e5236: f64 = (-locals.var_ps_t);
        let assign4920_e5240: f64 = (locals.var_vl__blk146 / locals.var_ps_t);
        let assign4920_e5241: f64 = (1.0 - assign4920_e5240);
        let assign4920_e5244: f64 = (1.0 - p.p51);
        let assign4920_e5245: f64 = (assign4920_e5241).powf(assign4920_e5244);
        let assign4920_e5246: f64 = (assign4920_e5236 * assign4920_e5245);
        let assign4920_e5249: f64 = (1.0 - p.p51);
        let assign4920_e5250: f64 = (assign4920_e5246 / assign4920_e5249);
        (assign4920_e5250, ((((-locals.var_ps_t_dn4) * assign4920_e5245) + (assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(((locals.var_vl__blk146_dn4 * locals.var_ps_t) - (locals.var_vl__blk146 * locals.var_ps_t_dn4)) / (locals.var_ps_t * locals.var_ps_t))))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(((locals.var_vl__blk146_dn4 * locals.var_ps_t) - (locals.var_vl__blk146 * locals.var_ps_t_dn4)) / (locals.var_ps_t * locals.var_ps_t))) / assign4920_e5241))) })) / assign4920_e5249), ((assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(locals.var_vl__blk146_dn10 / locals.var_ps_t)))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(locals.var_vl__blk146_dn10 / locals.var_ps_t)) / assign4920_e5241))) }) / assign4920_e5249), ((assign4920_e5236 * if 0.0 == 0.0 && ((assign4920_e5244) as f64).is_finite() && ((assign4920_e5244) as f64).fract() == 0.0 { if assign4920_e5244 == 0.0 { 0.0 } else { (assign4920_e5244 * ((assign4920_e5241).powf(assign4920_e5244 - 1.0) * (-(locals.var_vl__blk146_dn11 / locals.var_ps_t)))) } } else { (assign4920_e5245 * (assign4920_e5244 * ((-(locals.var_vl__blk146_dn11 / locals.var_ps_t)) / assign4920_e5241))) }) / assign4920_e5249),)
    } else {
        (locals.var_qlo__blk139, locals.var_qlo__blk139_dn4, locals.var_qlo__blk139_dn10, locals.var_qlo__blk139_dn11,)
    }
};
        locals.var_qlo__blk139 = assign4920_e5252;
        locals.var_qlo__blk139_dn4 = assign4920_e5252_d_n4;
        locals.var_qlo__blk139_dn10 = assign4920_e5252_d_n10;
        locals.var_qlo__blk139_dn11 = assign4920_e5252_d_n11;

        let (assign4930_e5292, assign4930_e5292_d_n4, assign4930_e5292_d_n10, assign4930_e5292_d_n11,) = {
    if ((locals.var_guard135 != 0.0) && (locals.var_guard147 == 0.0)) {
        let assign4930_e5260: f64 = (1.0 - p.p34);
        let assign4930_e5262: f64 = (-p.p51);
        let assign4930_e5263: f64 = (assign4930_e5260).powf(assign4930_e5262);
        let assign4930_e5266: f64 = (locals.var_vbcp - locals.var_vl__blk146);
        let assign4930_e5268: f64 = (assign4930_e5266 + locals.var_vl0__blk142);
        let assign4930_e5269: f64 = (assign4930_e5263 * assign4930_e5268);
        let assign4930_e5273: f64 = (0.5 * p.p51);
        let assign4930_e5276: f64 = (locals.var_vbcp - locals.var_vl__blk146);
        let assign4930_e5278: f64 = (assign4930_e5276 + locals.var_vl0__blk142);
        let assign4930_e5279: f64 = (assign4930_e5273 * assign4930_e5278);
        let assign4930_e5283: f64 = (1.0 - p.p34);
        let assign4930_e5284: f64 = (locals.var_ps_t * assign4930_e5283);
        let assign4930_e5285: f64 = (assign4930_e5279 / assign4930_e5284);
        let assign4930_e5286: f64 = (1.0 + assign4930_e5285);
        let assign4930_e5287: f64 = (assign4930_e5269 * assign4930_e5286);
        let assign4930_e5288: f64 = (locals.var_qlo__blk139 + assign4930_e5287);
        let assign4930_e5290: f64 = (assign4930_e5288 - locals.var_q0__blk143);
        (assign4930_e5290, ((locals.var_qlo__blk139_dn4 + (((assign4930_e5263 * ((-locals.var_vl__blk146_dn4) + locals.var_vl0__blk142_dn4)) * assign4930_e5286) + (assign4930_e5269 * ((((assign4930_e5273 * ((-locals.var_vl__blk146_dn4) + locals.var_vl0__blk142_dn4)) * assign4930_e5284) - (assign4930_e5279 * (locals.var_ps_t_dn4 * assign4930_e5283))) / (assign4930_e5284 * assign4930_e5284))))) - locals.var_q0__blk143_dn4), (locals.var_qlo__blk139_dn10 + (((assign4930_e5263 * (locals.var_vbcp_dn10 - locals.var_vl__blk146_dn10)) * assign4930_e5286) + (assign4930_e5269 * ((assign4930_e5273 * (locals.var_vbcp_dn10 - locals.var_vl__blk146_dn10)) / assign4930_e5284)))), (locals.var_qlo__blk139_dn11 + (((assign4930_e5263 * (locals.var_vbcp_dn11 - locals.var_vl__blk146_dn11)) * assign4930_e5286) + (assign4930_e5269 * ((assign4930_e5273 * (locals.var_vbcp_dn11 - locals.var_vl__blk146_dn11)) / assign4930_e5284)))),)
    } else {
        (locals.var_qdbcp, locals.var_qdbcp_dn4, locals.var_qdbcp_dn10, locals.var_qdbcp_dn11,)
    }
};
        locals.var_qdbcp = assign4930_e5292;
        locals.var_qdbcp_dn4 = assign4930_e5292_d_n4;
        locals.var_qdbcp_dn10 = assign4930_e5292_d_n10;
        locals.var_qdbcp_dn11 = assign4930_e5292_d_n11;

        let (assign4940_e5297, assign4940_e5297_d_n4, assign4940_e5297_d_n10, assign4940_e5297_d_n11,) = {
    if (locals.var_guard135 == 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qdbcp, locals.var_qdbcp_dn4, locals.var_qdbcp_dn10, locals.var_qdbcp_dn11,)
    }
};
        locals.var_qdbcp = assign4940_e5297;
        locals.var_qdbcp_dn4 = assign4940_e5297_d_n4;
        locals.var_qdbcp_dn10 = assign4940_e5297_d_n10;
        locals.var_qdbcp_dn11 = assign4940_e5297_d_n11;

        let assign4950_e5299: f64 = (-locals.var_pe_t);
        let assign4950_e5301: f64 = (assign4950_e5299 * p.p34);
        locals.var_dv0__blk149 = assign4950_e5301;
        locals.var_dv0__blk149_dn4 = ((-locals.var_pe_t_dn4) * p.p34);

        let assign4960_e5304: f64 = if p.p39 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign4960_e5304;

        let (assign4970_e5310, assign4970_e5310_d_n4, assign4970_e5310_d_n7, assign4970_e5310_d_n9,) = {
    if (locals.var_guard160 != 0.0) {
        let assign4970_e5308: f64 = (locals.var_vbex + locals.var_dv0__blk149);
        (assign4970_e5308, locals.var_dv0__blk149_dn4, locals.var_vbex_dn7, locals.var_vbex_dn9,)
    } else {
        (locals.var_dvh__blk150, locals.var_dvh__blk150_dn4, locals.var_dvh__blk150_dn7, locals.var_dvh__blk150_dn9,)
    }
};
        locals.var_dvh__blk150 = assign4970_e5310;
        locals.var_dvh__blk150_dn4 = assign4970_e5310_d_n4;
        locals.var_dvh__blk150_dn7 = assign4970_e5310_d_n7;
        locals.var_dvh__blk150_dn9 = assign4970_e5310_d_n9;

        let assign4980_e5313: f64 = if locals.var_dvh__blk150 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign4980_e5313;

        let (assign4990_e5324,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 != 0.0)) {
        let assign4990_e5319: f64 = (1.0 - p.p34);
        let assign4990_e5321: f64 = (-p.p38);
        let assign4990_e5322: f64 = (assign4990_e5319).powf(assign4990_e5321);
        (assign4990_e5322,)
    } else {
        (locals.var_pwq__blk151,)
    }
};
        locals.var_pwq__blk151 = assign4990_e5324;

        let (assign5000_e5342, assign5000_e5342_d_n4, assign5000_e5342_d_n7, assign5000_e5342_d_n9,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 != 0.0)) {
        let assign5000_e5333: f64 = (1.0 - p.p34);
        let assign5000_e5334: f64 = (locals.var_pwq__blk151 * assign5000_e5333);
        let assign5000_e5335: f64 = (1.0 - assign5000_e5334);
        let assign5000_e5336: f64 = (locals.var_pe_t * assign5000_e5335);
        let assign5000_e5339: f64 = (1.0 - p.p38);
        let assign5000_e5340: f64 = (assign5000_e5336 / assign5000_e5339);
        (assign5000_e5340, ((locals.var_pe_t_dn4 * assign5000_e5335) / assign5000_e5339), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk152, locals.var_qlo__blk152_dn4, locals.var_qlo__blk152_dn7, locals.var_qlo__blk152_dn9,)
    }
};
        locals.var_qlo__blk152 = assign5000_e5342;
        locals.var_qlo__blk152_dn4 = assign5000_e5342_d_n4;
        locals.var_qlo__blk152_dn7 = assign5000_e5342_d_n7;
        locals.var_qlo__blk152_dn9 = assign5000_e5342_d_n9;

        let (assign5010_e5364, assign5010_e5364_d_n4, assign5010_e5364_d_n7, assign5010_e5364_d_n9,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 != 0.0)) {
        let assign5010_e5350: f64 = (0.5 * p.p38);
        let assign5010_e5352: f64 = (assign5010_e5350 * locals.var_dvh__blk150);
        let assign5010_e5356: f64 = (1.0 - p.p34);
        let assign5010_e5357: f64 = (locals.var_pe_t * assign5010_e5356);
        let assign5010_e5358: f64 = (assign5010_e5352 / assign5010_e5357);
        let assign5010_e5359: f64 = (1.0 + assign5010_e5358);
        let assign5010_e5360: f64 = (locals.var_dvh__blk150 * assign5010_e5359);
        let assign5010_e5362: f64 = (assign5010_e5360 * locals.var_pwq__blk151);
        (assign5010_e5362, (((locals.var_dvh__blk150_dn4 * assign5010_e5359) + (locals.var_dvh__blk150 * ((((assign5010_e5350 * locals.var_dvh__blk150_dn4) * assign5010_e5357) - (assign5010_e5352 * (locals.var_pe_t_dn4 * assign5010_e5356))) / (assign5010_e5357 * assign5010_e5357)))) * locals.var_pwq__blk151), (((locals.var_dvh__blk150_dn7 * assign5010_e5359) + (locals.var_dvh__blk150 * ((assign5010_e5350 * locals.var_dvh__blk150_dn7) / assign5010_e5357))) * locals.var_pwq__blk151), (((locals.var_dvh__blk150_dn9 * assign5010_e5359) + (locals.var_dvh__blk150 * ((assign5010_e5350 * locals.var_dvh__blk150_dn9) / assign5010_e5357))) * locals.var_pwq__blk151),)
    } else {
        (locals.var_qhi__blk153, locals.var_qhi__blk153_dn4, locals.var_qhi__blk153_dn7, locals.var_qhi__blk153_dn9,)
    }
};
        locals.var_qhi__blk153 = assign5010_e5364;
        locals.var_qhi__blk153_dn4 = assign5010_e5364_d_n4;
        locals.var_qhi__blk153_dn7 = assign5010_e5364_d_n7;
        locals.var_qhi__blk153_dn9 = assign5010_e5364_d_n9;

        let (assign5020_e5387, assign5020_e5387_d_n4, assign5020_e5387_d_n7, assign5020_e5387_d_n9,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 == 0.0)) {
        let assign5020_e5374: f64 = (locals.var_vbex / locals.var_pe_t);
        let assign5020_e5375: f64 = (1.0 - assign5020_e5374);
        let assign5020_e5378: f64 = (1.0 - p.p38);
        let assign5020_e5379: f64 = (assign5020_e5375).powf(assign5020_e5378);
        let assign5020_e5380: f64 = (1.0 - assign5020_e5379);
        let assign5020_e5381: f64 = (locals.var_pe_t * assign5020_e5380);
        let assign5020_e5384: f64 = (1.0 - p.p38);
        let assign5020_e5385: f64 = (assign5020_e5381 / assign5020_e5384);
        (assign5020_e5385, (((locals.var_pe_t_dn4 * assign5020_e5380) + (locals.var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(-((locals.var_vbex * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t)))))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(-((locals.var_vbex * locals.var_pe_t_dn4) / (locals.var_pe_t * locals.var_pe_t)))) / assign5020_e5375))) }))) / assign5020_e5384), ((locals.var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(locals.var_vbex_dn7 / locals.var_pe_t)))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(locals.var_vbex_dn7 / locals.var_pe_t)) / assign5020_e5375))) })) / assign5020_e5384), ((locals.var_pe_t * (-if 0.0 == 0.0 && ((assign5020_e5378) as f64).is_finite() && ((assign5020_e5378) as f64).fract() == 0.0 { if assign5020_e5378 == 0.0 { 0.0 } else { (assign5020_e5378 * ((assign5020_e5375).powf(assign5020_e5378 - 1.0) * (-(locals.var_vbex_dn9 / locals.var_pe_t)))) } } else { (assign5020_e5379 * (assign5020_e5378 * ((-(locals.var_vbex_dn9 / locals.var_pe_t)) / assign5020_e5375))) })) / assign5020_e5384),)
    } else {
        (locals.var_qlo__blk152, locals.var_qlo__blk152_dn4, locals.var_qlo__blk152_dn7, locals.var_qlo__blk152_dn9,)
    }
};
        locals.var_qlo__blk152 = assign5020_e5387;
        locals.var_qlo__blk152_dn4 = assign5020_e5387_d_n4;
        locals.var_qlo__blk152_dn7 = assign5020_e5387_d_n7;
        locals.var_qlo__blk152_dn9 = assign5020_e5387_d_n9;

        let (assign5030_e5394, assign5030_e5394_d_n4, assign5030_e5394_d_n7, assign5030_e5394_d_n9,) = {
    if ((locals.var_guard160 != 0.0) && (locals.var_guard161 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk153, locals.var_qhi__blk153_dn4, locals.var_qhi__blk153_dn7, locals.var_qhi__blk153_dn9,)
    }
};
        locals.var_qhi__blk153 = assign5030_e5394;
        locals.var_qhi__blk153_dn4 = assign5030_e5394_d_n4;
        locals.var_qhi__blk153_dn7 = assign5030_e5394_d_n7;
        locals.var_qhi__blk153_dn9 = assign5030_e5394_d_n9;

        let (assign5040_e5400, assign5040_e5400_d_n4, assign5040_e5400_d_n7, assign5040_e5400_d_n9,) = {
    if (locals.var_guard160 != 0.0) {
        let assign5040_e5398: f64 = (locals.var_qlo__blk152 + locals.var_qhi__blk153);
        (assign5040_e5398, (locals.var_qlo__blk152_dn4 + locals.var_qhi__blk153_dn4), (locals.var_qlo__blk152_dn7 + locals.var_qhi__blk153_dn7), (locals.var_qlo__blk152_dn9 + locals.var_qhi__blk153_dn9),)
    } else {
        (locals.var_qdbex, locals.var_qdbex_dn4, locals.var_qdbex_dn7, locals.var_qdbex_dn9,)
    }
};
        locals.var_qdbex = assign5040_e5400;
        locals.var_qdbex_dn4 = assign5040_e5400_d_n4;
        locals.var_qdbex_dn7 = assign5040_e5400_d_n7;
        locals.var_qdbex_dn9 = assign5040_e5400_d_n9;

        let (assign5050_e5414, assign5050_e5414_d_n4,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5050_e5405: f64 = (locals.var_dv0__blk149 * locals.var_dv0__blk149);
        let assign5050_e5408: f64 = (4.0 * p.p39);
        let assign5050_e5410: f64 = (assign5050_e5408 * p.p39);
        let assign5050_e5411: f64 = (assign5050_e5405 + assign5050_e5410);
        let assign5050_e5412: f64 = (assign5050_e5411).sqrt();
        (assign5050_e5412, (((locals.var_dv0__blk149_dn4 * locals.var_dv0__blk149) + (locals.var_dv0__blk149 * locals.var_dv0__blk149_dn4)) / (2.0 * assign5050_e5412)),)
    } else {
        (locals.var_mv0__blk154, locals.var_mv0__blk154_dn4,)
    }
};
        locals.var_mv0__blk154 = assign5050_e5414;
        locals.var_mv0__blk154_dn4 = assign5050_e5414_d_n4;

        let (assign5060_e5424, assign5060_e5424_d_n4,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5060_e5418: f64 = (-0.5);
        let assign5060_e5421: f64 = (locals.var_dv0__blk149 + locals.var_mv0__blk154);
        let assign5060_e5422: f64 = (assign5060_e5418 * assign5060_e5421);
        (assign5060_e5422, (assign5060_e5418 * (locals.var_dv0__blk149_dn4 + locals.var_mv0__blk154_dn4)),)
    } else {
        (locals.var_vl0__blk155, locals.var_vl0__blk155_dn4,)
    }
};
        locals.var_vl0__blk155 = assign5060_e5424;
        locals.var_vl0__blk155_dn4 = assign5060_e5424_d_n4;

        let (assign5070_e5444, assign5070_e5444_d_n4,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5070_e5428: f64 = (-locals.var_pe_t);
        let assign5070_e5432: f64 = (locals.var_vl0__blk155 / locals.var_pe_t);
        let assign5070_e5433: f64 = (1.0 - assign5070_e5432);
        let assign5070_e5436: f64 = (1.0 - p.p38);
        let assign5070_e5437: f64 = (assign5070_e5433).powf(assign5070_e5436);
        let assign5070_e5438: f64 = (assign5070_e5428 * assign5070_e5437);
        let assign5070_e5441: f64 = (1.0 - p.p38);
        let assign5070_e5442: f64 = (assign5070_e5438 / assign5070_e5441);
        (assign5070_e5442, ((((-locals.var_pe_t_dn4) * assign5070_e5437) + (assign5070_e5428 * if 0.0 == 0.0 && ((assign5070_e5436) as f64).is_finite() && ((assign5070_e5436) as f64).fract() == 0.0 { if assign5070_e5436 == 0.0 { 0.0 } else { (assign5070_e5436 * ((assign5070_e5433).powf(assign5070_e5436 - 1.0) * (-(((locals.var_vl0__blk155_dn4 * locals.var_pe_t) - (locals.var_vl0__blk155 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign5070_e5437 * (assign5070_e5436 * ((-(((locals.var_vl0__blk155_dn4 * locals.var_pe_t) - (locals.var_vl0__blk155 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))) / assign5070_e5433))) })) / assign5070_e5441),)
    } else {
        (locals.var_q0__blk156, locals.var_q0__blk156_dn4,)
    }
};
        locals.var_q0__blk156 = assign5070_e5444;
        locals.var_q0__blk156_dn4 = assign5070_e5444_d_n4;

        let (assign5080_e5451, assign5080_e5451_d_n4, assign5080_e5451_d_n7, assign5080_e5451_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5080_e5449: f64 = (locals.var_vbex + locals.var_dv0__blk149);
        (assign5080_e5449, locals.var_dv0__blk149_dn4, locals.var_vbex_dn7, locals.var_vbex_dn9,)
    } else {
        (locals.var_dv__blk157, locals.var_dv__blk157_dn4, locals.var_dv__blk157_dn7, locals.var_dv__blk157_dn9,)
    }
};
        locals.var_dv__blk157 = assign5080_e5451;
        locals.var_dv__blk157_dn4 = assign5080_e5451_d_n4;
        locals.var_dv__blk157_dn7 = assign5080_e5451_d_n7;
        locals.var_dv__blk157_dn9 = assign5080_e5451_d_n9;

        let (assign5090_e5465, assign5090_e5465_d_n4, assign5090_e5465_d_n7, assign5090_e5465_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5090_e5456: f64 = (locals.var_dv__blk157 * locals.var_dv__blk157);
        let assign5090_e5459: f64 = (4.0 * p.p39);
        let assign5090_e5461: f64 = (assign5090_e5459 * p.p39);
        let assign5090_e5462: f64 = (assign5090_e5456 + assign5090_e5461);
        let assign5090_e5463: f64 = (assign5090_e5462).sqrt();
        (assign5090_e5463, (((locals.var_dv__blk157_dn4 * locals.var_dv__blk157) + (locals.var_dv__blk157 * locals.var_dv__blk157_dn4)) / (2.0 * assign5090_e5463)), (((locals.var_dv__blk157_dn7 * locals.var_dv__blk157) + (locals.var_dv__blk157 * locals.var_dv__blk157_dn7)) / (2.0 * assign5090_e5463)), (((locals.var_dv__blk157_dn9 * locals.var_dv__blk157) + (locals.var_dv__blk157 * locals.var_dv__blk157_dn9)) / (2.0 * assign5090_e5463)),)
    } else {
        (locals.var_mv__blk158, locals.var_mv__blk158_dn4, locals.var_mv__blk158_dn7, locals.var_mv__blk158_dn9,)
    }
};
        locals.var_mv__blk158 = assign5090_e5465;
        locals.var_mv__blk158_dn4 = assign5090_e5465_d_n4;
        locals.var_mv__blk158_dn7 = assign5090_e5465_d_n7;
        locals.var_mv__blk158_dn9 = assign5090_e5465_d_n9;

        let (assign5100_e5476, assign5100_e5476_d_n4, assign5100_e5476_d_n7, assign5100_e5476_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5100_e5471: f64 = (locals.var_dv__blk157 - locals.var_mv__blk158);
        let assign5100_e5472: f64 = (0.5 * assign5100_e5471);
        let assign5100_e5474: f64 = (assign5100_e5472 - locals.var_dv0__blk149);
        (assign5100_e5474, ((0.5 * (locals.var_dv__blk157_dn4 - locals.var_mv__blk158_dn4)) - locals.var_dv0__blk149_dn4), (0.5 * (locals.var_dv__blk157_dn7 - locals.var_mv__blk158_dn7)), (0.5 * (locals.var_dv__blk157_dn9 - locals.var_mv__blk158_dn9)),)
    } else {
        (locals.var_vl__blk159, locals.var_vl__blk159_dn4, locals.var_vl__blk159_dn7, locals.var_vl__blk159_dn9,)
    }
};
        locals.var_vl__blk159 = assign5100_e5476;
        locals.var_vl__blk159_dn4 = assign5100_e5476_d_n4;
        locals.var_vl__blk159_dn7 = assign5100_e5476_d_n7;
        locals.var_vl__blk159_dn9 = assign5100_e5476_d_n9;

        let (assign5110_e5496, assign5110_e5496_d_n4, assign5110_e5496_d_n7, assign5110_e5496_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5110_e5480: f64 = (-locals.var_pe_t);
        let assign5110_e5484: f64 = (locals.var_vl__blk159 / locals.var_pe_t);
        let assign5110_e5485: f64 = (1.0 - assign5110_e5484);
        let assign5110_e5488: f64 = (1.0 - p.p38);
        let assign5110_e5489: f64 = (assign5110_e5485).powf(assign5110_e5488);
        let assign5110_e5490: f64 = (assign5110_e5480 * assign5110_e5489);
        let assign5110_e5493: f64 = (1.0 - p.p38);
        let assign5110_e5494: f64 = (assign5110_e5490 / assign5110_e5493);
        (assign5110_e5494, ((((-locals.var_pe_t_dn4) * assign5110_e5489) + (assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(((locals.var_vl__blk159_dn4 * locals.var_pe_t) - (locals.var_vl__blk159 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(((locals.var_vl__blk159_dn4 * locals.var_pe_t) - (locals.var_vl__blk159 * locals.var_pe_t_dn4)) / (locals.var_pe_t * locals.var_pe_t))) / assign5110_e5485))) })) / assign5110_e5493), ((assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(locals.var_vl__blk159_dn7 / locals.var_pe_t)))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(locals.var_vl__blk159_dn7 / locals.var_pe_t)) / assign5110_e5485))) }) / assign5110_e5493), ((assign5110_e5480 * if 0.0 == 0.0 && ((assign5110_e5488) as f64).is_finite() && ((assign5110_e5488) as f64).fract() == 0.0 { if assign5110_e5488 == 0.0 { 0.0 } else { (assign5110_e5488 * ((assign5110_e5485).powf(assign5110_e5488 - 1.0) * (-(locals.var_vl__blk159_dn9 / locals.var_pe_t)))) } } else { (assign5110_e5489 * (assign5110_e5488 * ((-(locals.var_vl__blk159_dn9 / locals.var_pe_t)) / assign5110_e5485))) }) / assign5110_e5493),)
    } else {
        (locals.var_qlo__blk152, locals.var_qlo__blk152_dn4, locals.var_qlo__blk152_dn7, locals.var_qlo__blk152_dn9,)
    }
};
        locals.var_qlo__blk152 = assign5110_e5496;
        locals.var_qlo__blk152_dn4 = assign5110_e5496_d_n4;
        locals.var_qlo__blk152_dn7 = assign5110_e5496_d_n7;
        locals.var_qlo__blk152_dn9 = assign5110_e5496_d_n9;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5120_e5534, assign5120_e5534_d_n4, assign5120_e5534_d_n7, assign5120_e5534_d_n9,) = {
    if (locals.var_guard160 == 0.0) {
        let assign5120_e5502: f64 = (1.0 - p.p34);
        let assign5120_e5504: f64 = (-p.p38);
        let assign5120_e5505: f64 = (assign5120_e5502).powf(assign5120_e5504);
        let assign5120_e5508: f64 = (locals.var_vbex - locals.var_vl__blk159);
        let assign5120_e5510: f64 = (assign5120_e5508 + locals.var_vl0__blk155);
        let assign5120_e5511: f64 = (assign5120_e5505 * assign5120_e5510);
        let assign5120_e5515: f64 = (0.5 * p.p38);
        let assign5120_e5518: f64 = (locals.var_vbex - locals.var_vl__blk159);
        let assign5120_e5520: f64 = (assign5120_e5518 + locals.var_vl0__blk155);
        let assign5120_e5521: f64 = (assign5120_e5515 * assign5120_e5520);
        let assign5120_e5525: f64 = (1.0 - p.p34);
        let assign5120_e5526: f64 = (locals.var_pe_t * assign5120_e5525);
        let assign5120_e5527: f64 = (assign5120_e5521 / assign5120_e5526);
        let assign5120_e5528: f64 = (1.0 + assign5120_e5527);
        let assign5120_e5529: f64 = (assign5120_e5511 * assign5120_e5528);
        let assign5120_e5530: f64 = (locals.var_qlo__blk152 + assign5120_e5529);
        let assign5120_e5532: f64 = (assign5120_e5530 - locals.var_q0__blk156);
        (assign5120_e5532, ((locals.var_qlo__blk152_dn4 + (((assign5120_e5505 * ((-locals.var_vl__blk159_dn4) + locals.var_vl0__blk155_dn4)) * assign5120_e5528) + (assign5120_e5511 * ((((assign5120_e5515 * ((-locals.var_vl__blk159_dn4) + locals.var_vl0__blk155_dn4)) * assign5120_e5526) - (assign5120_e5521 * (locals.var_pe_t_dn4 * assign5120_e5525))) / (assign5120_e5526 * assign5120_e5526))))) - locals.var_q0__blk156_dn4), (locals.var_qlo__blk152_dn7 + (((assign5120_e5505 * (locals.var_vbex_dn7 - locals.var_vl__blk159_dn7)) * assign5120_e5528) + (assign5120_e5511 * ((assign5120_e5515 * (locals.var_vbex_dn7 - locals.var_vl__blk159_dn7)) / assign5120_e5526)))), (locals.var_qlo__blk152_dn9 + (((assign5120_e5505 * (locals.var_vbex_dn9 - locals.var_vl__blk159_dn9)) * assign5120_e5528) + (assign5120_e5511 * ((assign5120_e5515 * (locals.var_vbex_dn9 - locals.var_vl__blk159_dn9)) / assign5120_e5526)))),)
    } else {
        (locals.var_qdbex, locals.var_qdbex_dn4, locals.var_qdbex_dn7, locals.var_qdbex_dn9,)
    }
};
        locals.var_qdbex = assign5120_e5534;
        locals.var_qdbex_dn4 = assign5120_e5534_d_n4;
        locals.var_qdbex_dn7 = assign5120_e5534_d_n7;
        locals.var_qdbex_dn9 = assign5120_e5534_d_n9;

        let assign5130_e5536: f64 = (-locals.var_pc_t);
        let assign5130_e5538: f64 = (assign5130_e5536 * p.p34);
        locals.var_dv0__blk162 = assign5130_e5538;
        locals.var_dv0__blk162_dn4 = ((-locals.var_pc_t_dn4) * p.p34);

        let assign5140_e5541: f64 = if p.p44 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard183 = assign5140_e5541;

        let (assign5150_e5547, assign5150_e5547_d_n4, assign5150_e5547_d_n7, assign5150_e5547_d_n10,) = {
    if (locals.var_guard183 != 0.0) {
        let assign5150_e5545: f64 = (locals.var_vbep + locals.var_dv0__blk162);
        (assign5150_e5545, locals.var_dv0__blk162_dn4, locals.var_vbep_dn7, locals.var_vbep_dn10,)
    } else {
        (locals.var_dvh__blk163, locals.var_dvh__blk163_dn4, locals.var_dvh__blk163_dn7, locals.var_dvh__blk163_dn10,)
    }
};
        locals.var_dvh__blk163 = assign5150_e5547;
        locals.var_dvh__blk163_dn4 = assign5150_e5547_d_n4;
        locals.var_dvh__blk163_dn7 = assign5150_e5547_d_n7;
        locals.var_dvh__blk163_dn10 = assign5150_e5547_d_n10;

        let assign5160_e5550: f64 = if locals.var_dvh__blk163 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard184 = assign5160_e5550;

        let (assign5170_e5563,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 != 0.0)) {
        let assign5170_e5556: f64 = (1.0 - p.p34);
        let assign5170_e5558: f64 = (-1.0);
        let assign5170_e5560: f64 = (assign5170_e5558 - p.p43);
        let assign5170_e5561: f64 = (assign5170_e5556).powf(assign5170_e5560);
        (assign5170_e5561,)
    } else {
        (locals.var_pwq__blk164,)
    }
};
        locals.var_pwq__blk164 = assign5170_e5563;

        let (assign5180_e5585, assign5180_e5585_d_n4, assign5180_e5585_d_n7, assign5180_e5585_d_n10,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 != 0.0)) {
        let assign5180_e5572: f64 = (1.0 - p.p34);
        let assign5180_e5573: f64 = (locals.var_pwq__blk164 * assign5180_e5572);
        let assign5180_e5576: f64 = (1.0 - p.p34);
        let assign5180_e5577: f64 = (assign5180_e5573 * assign5180_e5576);
        let assign5180_e5578: f64 = (1.0 - assign5180_e5577);
        let assign5180_e5579: f64 = (locals.var_pc_t * assign5180_e5578);
        let assign5180_e5582: f64 = (1.0 - p.p43);
        let assign5180_e5583: f64 = (assign5180_e5579 / assign5180_e5582);
        (assign5180_e5583, ((locals.var_pc_t_dn4 * assign5180_e5578) / assign5180_e5582), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5180_e5585;
        locals.var_qlo__blk165_dn4 = assign5180_e5585_d_n4;
        locals.var_qlo__blk165_dn7 = assign5180_e5585_d_n7;
        locals.var_qlo__blk165_dn10 = assign5180_e5585_d_n10;

        let (assign5190_e5605, assign5190_e5605_d_n4, assign5190_e5605_d_n7, assign5190_e5605_d_n10,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 != 0.0)) {
        let assign5190_e5592: f64 = (1.0 - p.p34);
        let assign5190_e5595: f64 = (0.5 * p.p43);
        let assign5190_e5597: f64 = (assign5190_e5595 * locals.var_dvh__blk163);
        let assign5190_e5599: f64 = (assign5190_e5597 / locals.var_pc_t);
        let assign5190_e5600: f64 = (assign5190_e5592 + assign5190_e5599);
        let assign5190_e5601: f64 = (locals.var_dvh__blk163 * assign5190_e5600);
        let assign5190_e5603: f64 = (assign5190_e5601 * locals.var_pwq__blk164);
        (assign5190_e5603, (((locals.var_dvh__blk163_dn4 * assign5190_e5600) + (locals.var_dvh__blk163 * ((((assign5190_e5595 * locals.var_dvh__blk163_dn4) * locals.var_pc_t) - (assign5190_e5597 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)))) * locals.var_pwq__blk164), (((locals.var_dvh__blk163_dn7 * assign5190_e5600) + (locals.var_dvh__blk163 * ((assign5190_e5595 * locals.var_dvh__blk163_dn7) / locals.var_pc_t))) * locals.var_pwq__blk164), (((locals.var_dvh__blk163_dn10 * assign5190_e5600) + (locals.var_dvh__blk163 * ((assign5190_e5595 * locals.var_dvh__blk163_dn10) / locals.var_pc_t))) * locals.var_pwq__blk164),)
    } else {
        (locals.var_qhi__blk166, locals.var_qhi__blk166_dn4, locals.var_qhi__blk166_dn7, locals.var_qhi__blk166_dn10,)
    }
};
        locals.var_qhi__blk166 = assign5190_e5605;
        locals.var_qhi__blk166_dn4 = assign5190_e5605_d_n4;
        locals.var_qhi__blk166_dn7 = assign5190_e5605_d_n7;
        locals.var_qhi__blk166_dn10 = assign5190_e5605_d_n10;

        let assign5200_e5611: f64 = (-p.p45);
        let assign5200_e5613: f64 = if ((p.p45 > 0.0) && (locals.var_vbep < assign5200_e5611)) { 1.0 } else { 0.0 };
        locals.var_guard185 = assign5200_e5613;

        let (assign5210_e5652, assign5210_e5652_d_n4, assign5210_e5652_d_n7, assign5210_e5652_d_n10,) = {
    if (((locals.var_guard183 != 0.0) && (locals.var_guard184 == 0.0)) && (locals.var_guard185 != 0.0)) {
        let assign5210_e5625: f64 = (p.p45 / locals.var_pc_t);
        let assign5210_e5626: f64 = (1.0 + assign5210_e5625);
        let assign5210_e5629: f64 = (1.0 - p.p43);
        let assign5210_e5630: f64 = (assign5210_e5626).powf(assign5210_e5629);
        let assign5210_e5634: f64 = (1.0 - p.p43);
        let assign5210_e5637: f64 = (locals.var_vbep + p.p45);
        let assign5210_e5638: f64 = (assign5210_e5634 * assign5210_e5637);
        let assign5210_e5641: f64 = (locals.var_pc_t + p.p45);
        let assign5210_e5642: f64 = (assign5210_e5638 / assign5210_e5641);
        let assign5210_e5643: f64 = (1.0 - assign5210_e5642);
        let assign5210_e5644: f64 = (assign5210_e5630 * assign5210_e5643);
        let assign5210_e5645: f64 = (1.0 - assign5210_e5644);
        let assign5210_e5646: f64 = (locals.var_pc_t * assign5210_e5645);
        let assign5210_e5649: f64 = (1.0 - p.p43);
        let assign5210_e5650: f64 = (assign5210_e5646 / assign5210_e5649);
        (assign5210_e5650, (((locals.var_pc_t_dn4 * assign5210_e5645) + (locals.var_pc_t * (-((if 0.0 == 0.0 && ((assign5210_e5629) as f64).is_finite() && ((assign5210_e5629) as f64).fract() == 0.0 { if assign5210_e5629 == 0.0 { 0.0 } else { (assign5210_e5629 * ((assign5210_e5626).powf(assign5210_e5629 - 1.0) * (-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5210_e5630 * (assign5210_e5629 * ((-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign5210_e5626))) } * assign5210_e5643) + (assign5210_e5630 * (-(-((assign5210_e5638 * locals.var_pc_t_dn4) / (assign5210_e5641 * assign5210_e5641))))))))) / assign5210_e5649), ((locals.var_pc_t * (-(assign5210_e5630 * (-((assign5210_e5634 * locals.var_vbep_dn7) / assign5210_e5641))))) / assign5210_e5649), ((locals.var_pc_t * (-(assign5210_e5630 * (-((assign5210_e5634 * locals.var_vbep_dn10) / assign5210_e5641))))) / assign5210_e5649),)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5210_e5652;
        locals.var_qlo__blk165_dn4 = assign5210_e5652_d_n4;
        locals.var_qlo__blk165_dn7 = assign5210_e5652_d_n7;
        locals.var_qlo__blk165_dn10 = assign5210_e5652_d_n10;

        let (assign5220_e5678, assign5220_e5678_d_n4, assign5220_e5678_d_n7, assign5220_e5678_d_n10,) = {
    if (((locals.var_guard183 != 0.0) && (locals.var_guard184 == 0.0)) && (locals.var_guard185 == 0.0)) {
        let assign5220_e5665: f64 = (locals.var_vbep / locals.var_pc_t);
        let assign5220_e5666: f64 = (1.0 - assign5220_e5665);
        let assign5220_e5669: f64 = (1.0 - p.p43);
        let assign5220_e5670: f64 = (assign5220_e5666).powf(assign5220_e5669);
        let assign5220_e5671: f64 = (1.0 - assign5220_e5670);
        let assign5220_e5672: f64 = (locals.var_pc_t * assign5220_e5671);
        let assign5220_e5675: f64 = (1.0 - p.p43);
        let assign5220_e5676: f64 = (assign5220_e5672 / assign5220_e5675);
        (assign5220_e5676, (((locals.var_pc_t_dn4 * assign5220_e5671) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(-((locals.var_vbep * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t)))))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(-((locals.var_vbep * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t)))) / assign5220_e5666))) }))) / assign5220_e5675), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(locals.var_vbep_dn7 / locals.var_pc_t)))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(locals.var_vbep_dn7 / locals.var_pc_t)) / assign5220_e5666))) })) / assign5220_e5675), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5220_e5669) as f64).is_finite() && ((assign5220_e5669) as f64).fract() == 0.0 { if assign5220_e5669 == 0.0 { 0.0 } else { (assign5220_e5669 * ((assign5220_e5666).powf(assign5220_e5669 - 1.0) * (-(locals.var_vbep_dn10 / locals.var_pc_t)))) } } else { (assign5220_e5670 * (assign5220_e5669 * ((-(locals.var_vbep_dn10 / locals.var_pc_t)) / assign5220_e5666))) })) / assign5220_e5675),)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5220_e5678;
        locals.var_qlo__blk165_dn4 = assign5220_e5678_d_n4;
        locals.var_qlo__blk165_dn7 = assign5220_e5678_d_n7;
        locals.var_qlo__blk165_dn10 = assign5220_e5678_d_n10;

        let (assign5230_e5685, assign5230_e5685_d_n4, assign5230_e5685_d_n7, assign5230_e5685_d_n10,) = {
    if ((locals.var_guard183 != 0.0) && (locals.var_guard184 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk166, locals.var_qhi__blk166_dn4, locals.var_qhi__blk166_dn7, locals.var_qhi__blk166_dn10,)
    }
};
        locals.var_qhi__blk166 = assign5230_e5685;
        locals.var_qhi__blk166_dn4 = assign5230_e5685_d_n4;
        locals.var_qhi__blk166_dn7 = assign5230_e5685_d_n7;
        locals.var_qhi__blk166_dn10 = assign5230_e5685_d_n10;

        let (assign5240_e5691, assign5240_e5691_d_n4, assign5240_e5691_d_n7, assign5240_e5691_d_n10,) = {
    if (locals.var_guard183 != 0.0) {
        let assign5240_e5689: f64 = (locals.var_qlo__blk165 + locals.var_qhi__blk166);
        (assign5240_e5689, (locals.var_qlo__blk165_dn4 + locals.var_qhi__blk166_dn4), (locals.var_qlo__blk165_dn7 + locals.var_qhi__blk166_dn7), (locals.var_qlo__blk165_dn10 + locals.var_qhi__blk166_dn10),)
    } else {
        (locals.var_qdbep, locals.var_qdbep_dn4, locals.var_qdbep_dn7, locals.var_qdbep_dn10,)
    }
};
        locals.var_qdbep = assign5240_e5691;
        locals.var_qdbep_dn4 = assign5240_e5691_d_n4;
        locals.var_qdbep_dn7 = assign5240_e5691_d_n7;
        locals.var_qdbep_dn10 = assign5240_e5691_d_n10;

        let assign5250_e5698: f64 = if ((p.p45 > 0.0) && (p.p46 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard186 = assign5250_e5698;

        let (assign5260_e5711, assign5260_e5711_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5260_e5705: f64 = (p.p45 + locals.var_dv0__blk162);
        let assign5260_e5708: f64 = (p.p45 - locals.var_dv0__blk162);
        let assign5260_e5709: f64 = (assign5260_e5705 / assign5260_e5708);
        (assign5260_e5709, (((locals.var_dv0__blk162_dn4 * assign5260_e5708) - (assign5260_e5705 * (-locals.var_dv0__blk162_dn4))) / (assign5260_e5708 * assign5260_e5708)),)
    } else {
        (locals.var_vn0__blk167, locals.var_vn0__blk167_dn4,)
    }
};
        locals.var_vn0__blk167 = assign5260_e5711;
        locals.var_vn0__blk167_dn4 = assign5260_e5711_d_n4;

        let (assign5270_e5750, assign5270_e5750_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5270_e5718: f64 = (2.0 * locals.var_vn0__blk167);
        let assign5270_e5721: f64 = (locals.var_vn0__blk167 - 1.0);
        let assign5270_e5724: f64 = (locals.var_vn0__blk167 - 1.0);
        let assign5270_e5725: f64 = (assign5270_e5721 * assign5270_e5724);
        let assign5270_e5728: f64 = (4.0 * p.p44);
        let assign5270_e5730: f64 = (assign5270_e5728 * p.p44);
        let assign5270_e5731: f64 = (assign5270_e5725 + assign5270_e5730);
        let assign5270_e5732: f64 = (assign5270_e5731).sqrt();
        let assign5270_e5735: f64 = (locals.var_vn0__blk167 + 1.0);
        let assign5270_e5738: f64 = (locals.var_vn0__blk167 + 1.0);
        let assign5270_e5739: f64 = (assign5270_e5735 * assign5270_e5738);
        let assign5270_e5742: f64 = (4.0 * p.p46);
        let assign5270_e5744: f64 = (assign5270_e5742 * p.p46);
        let assign5270_e5745: f64 = (assign5270_e5739 + assign5270_e5744);
        let assign5270_e5746: f64 = (assign5270_e5745).sqrt();
        let assign5270_e5747: f64 = (assign5270_e5732 + assign5270_e5746);
        let assign5270_e5748: f64 = (assign5270_e5718 / assign5270_e5747);
        (assign5270_e5748, ((((2.0 * locals.var_vn0__blk167_dn4) * assign5270_e5747) - (assign5270_e5718 * ((((locals.var_vn0__blk167_dn4 * assign5270_e5724) + (assign5270_e5721 * locals.var_vn0__blk167_dn4)) / (2.0 * assign5270_e5732)) + (((locals.var_vn0__blk167_dn4 * assign5270_e5738) + (assign5270_e5735 * locals.var_vn0__blk167_dn4)) / (2.0 * assign5270_e5746))))) / (assign5270_e5747 * assign5270_e5747)),)
    } else {
        (locals.var_vnl0__blk168, locals.var_vnl0__blk168_dn4,)
    }
};
        locals.var_vnl0__blk168 = assign5270_e5750;
        locals.var_vnl0__blk168_dn4 = assign5270_e5750_d_n4;

        let (assign5280_e5767, assign5280_e5767_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5280_e5759: f64 = (p.p45 - locals.var_dv0__blk162);
        let assign5280_e5760: f64 = (locals.var_vnl0__blk168 * assign5280_e5759);
        let assign5280_e5762: f64 = (assign5280_e5760 - p.p45);
        let assign5280_e5764: f64 = (assign5280_e5762 - locals.var_dv0__blk162);
        let assign5280_e5765: f64 = (0.5 * assign5280_e5764);
        (assign5280_e5765, (0.5 * (((locals.var_vnl0__blk168_dn4 * assign5280_e5759) + (locals.var_vnl0__blk168 * (-locals.var_dv0__blk162_dn4))) - locals.var_dv0__blk162_dn4)),)
    } else {
        (locals.var_vl0__blk169, locals.var_vl0__blk169_dn4,)
    }
};
        locals.var_vl0__blk169 = assign5280_e5767;
        locals.var_vl0__blk169_dn4 = assign5280_e5767_d_n4;

        let (assign5290_e5790, assign5290_e5790_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5290_e5777: f64 = (locals.var_vl0__blk169 / locals.var_pc_t);
        let assign5290_e5778: f64 = (1.0 - assign5290_e5777);
        let assign5290_e5781: f64 = (1.0 - p.p43);
        let assign5290_e5782: f64 = (assign5290_e5778).powf(assign5290_e5781);
        let assign5290_e5783: f64 = (1.0 - assign5290_e5782);
        let assign5290_e5784: f64 = (locals.var_pc_t * assign5290_e5783);
        let assign5290_e5787: f64 = (1.0 - p.p43);
        let assign5290_e5788: f64 = (assign5290_e5784 / assign5290_e5787);
        (assign5290_e5788, (((locals.var_pc_t_dn4 * assign5290_e5783) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5290_e5781) as f64).is_finite() && ((assign5290_e5781) as f64).fract() == 0.0 { if assign5290_e5781 == 0.0 { 0.0 } else { (assign5290_e5781 * ((assign5290_e5778).powf(assign5290_e5781 - 1.0) * (-(((locals.var_vl0__blk169_dn4 * locals.var_pc_t) - (locals.var_vl0__blk169 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5290_e5782 * (assign5290_e5781 * ((-(((locals.var_vl0__blk169_dn4 * locals.var_pc_t) - (locals.var_vl0__blk169 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign5290_e5778))) }))) / assign5290_e5787),)
    } else {
        (locals.var_qlo0__blk170, locals.var_qlo0__blk170_dn4,)
    }
};
        locals.var_qlo0__blk170 = assign5290_e5790;
        locals.var_qlo0__blk170_dn4 = assign5290_e5790_d_n4;

        let (assign5300_e5807, assign5300_e5807_d_n4, assign5300_e5807_d_n7, assign5300_e5807_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5300_e5797: f64 = (2.0 * locals.var_vbep);
        let assign5300_e5799: f64 = (assign5300_e5797 + p.p45);
        let assign5300_e5801: f64 = (assign5300_e5799 + locals.var_dv0__blk162);
        let assign5300_e5804: f64 = (p.p45 - locals.var_dv0__blk162);
        let assign5300_e5805: f64 = (assign5300_e5801 / assign5300_e5804);
        (assign5300_e5805, (((locals.var_dv0__blk162_dn4 * assign5300_e5804) - (assign5300_e5801 * (-locals.var_dv0__blk162_dn4))) / (assign5300_e5804 * assign5300_e5804)), ((2.0 * locals.var_vbep_dn7) / assign5300_e5804), ((2.0 * locals.var_vbep_dn10) / assign5300_e5804),)
    } else {
        (locals.var_vn__blk171, locals.var_vn__blk171_dn4, locals.var_vn__blk171_dn7, locals.var_vn__blk171_dn10,)
    }
};
        locals.var_vn__blk171 = assign5300_e5807;
        locals.var_vn__blk171_dn4 = assign5300_e5807_d_n4;
        locals.var_vn__blk171_dn7 = assign5300_e5807_d_n7;
        locals.var_vn__blk171_dn10 = assign5300_e5807_d_n10;

        let (assign5310_e5846, assign5310_e5846_d_n4, assign5310_e5846_d_n7, assign5310_e5846_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5310_e5814: f64 = (2.0 * locals.var_vn__blk171);
        let assign5310_e5817: f64 = (locals.var_vn__blk171 - 1.0);
        let assign5310_e5820: f64 = (locals.var_vn__blk171 - 1.0);
        let assign5310_e5821: f64 = (assign5310_e5817 * assign5310_e5820);
        let assign5310_e5824: f64 = (4.0 * p.p44);
        let assign5310_e5826: f64 = (assign5310_e5824 * p.p44);
        let assign5310_e5827: f64 = (assign5310_e5821 + assign5310_e5826);
        let assign5310_e5828: f64 = (assign5310_e5827).sqrt();
        let assign5310_e5831: f64 = (locals.var_vn__blk171 + 1.0);
        let assign5310_e5834: f64 = (locals.var_vn__blk171 + 1.0);
        let assign5310_e5835: f64 = (assign5310_e5831 * assign5310_e5834);
        let assign5310_e5838: f64 = (4.0 * p.p46);
        let assign5310_e5840: f64 = (assign5310_e5838 * p.p46);
        let assign5310_e5841: f64 = (assign5310_e5835 + assign5310_e5840);
        let assign5310_e5842: f64 = (assign5310_e5841).sqrt();
        let assign5310_e5843: f64 = (assign5310_e5828 + assign5310_e5842);
        let assign5310_e5844: f64 = (assign5310_e5814 / assign5310_e5843);
        (assign5310_e5844, ((((2.0 * locals.var_vn__blk171_dn4) * assign5310_e5843) - (assign5310_e5814 * ((((locals.var_vn__blk171_dn4 * assign5310_e5820) + (assign5310_e5817 * locals.var_vn__blk171_dn4)) / (2.0 * assign5310_e5828)) + (((locals.var_vn__blk171_dn4 * assign5310_e5834) + (assign5310_e5831 * locals.var_vn__blk171_dn4)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * locals.var_vn__blk171_dn7) * assign5310_e5843) - (assign5310_e5814 * ((((locals.var_vn__blk171_dn7 * assign5310_e5820) + (assign5310_e5817 * locals.var_vn__blk171_dn7)) / (2.0 * assign5310_e5828)) + (((locals.var_vn__blk171_dn7 * assign5310_e5834) + (assign5310_e5831 * locals.var_vn__blk171_dn7)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)), ((((2.0 * locals.var_vn__blk171_dn10) * assign5310_e5843) - (assign5310_e5814 * ((((locals.var_vn__blk171_dn10 * assign5310_e5820) + (assign5310_e5817 * locals.var_vn__blk171_dn10)) / (2.0 * assign5310_e5828)) + (((locals.var_vn__blk171_dn10 * assign5310_e5834) + (assign5310_e5831 * locals.var_vn__blk171_dn10)) / (2.0 * assign5310_e5842))))) / (assign5310_e5843 * assign5310_e5843)),)
    } else {
        (locals.var_vnl__blk172, locals.var_vnl__blk172_dn4, locals.var_vnl__blk172_dn7, locals.var_vnl__blk172_dn10,)
    }
};
        locals.var_vnl__blk172 = assign5310_e5846;
        locals.var_vnl__blk172_dn4 = assign5310_e5846_d_n4;
        locals.var_vnl__blk172_dn7 = assign5310_e5846_d_n7;
        locals.var_vnl__blk172_dn10 = assign5310_e5846_d_n10;

        let (assign5320_e5863, assign5320_e5863_d_n4, assign5320_e5863_d_n7, assign5320_e5863_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5320_e5855: f64 = (p.p45 - locals.var_dv0__blk162);
        let assign5320_e5856: f64 = (locals.var_vnl__blk172 * assign5320_e5855);
        let assign5320_e5858: f64 = (assign5320_e5856 - p.p45);
        let assign5320_e5860: f64 = (assign5320_e5858 - locals.var_dv0__blk162);
        let assign5320_e5861: f64 = (0.5 * assign5320_e5860);
        (assign5320_e5861, (0.5 * (((locals.var_vnl__blk172_dn4 * assign5320_e5855) + (locals.var_vnl__blk172 * (-locals.var_dv0__blk162_dn4))) - locals.var_dv0__blk162_dn4)), (0.5 * (locals.var_vnl__blk172_dn7 * assign5320_e5855)), (0.5 * (locals.var_vnl__blk172_dn10 * assign5320_e5855)),)
    } else {
        (locals.var_vl__blk173, locals.var_vl__blk173_dn4, locals.var_vl__blk173_dn7, locals.var_vl__blk173_dn10,)
    }
};
        locals.var_vl__blk173 = assign5320_e5863;
        locals.var_vl__blk173_dn4 = assign5320_e5863_d_n4;
        locals.var_vl__blk173_dn7 = assign5320_e5863_d_n7;
        locals.var_vl__blk173_dn10 = assign5320_e5863_d_n10;

        let (assign5330_e5886, assign5330_e5886_d_n4, assign5330_e5886_d_n7, assign5330_e5886_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5330_e5873: f64 = (locals.var_vl__blk173 / locals.var_pc_t);
        let assign5330_e5874: f64 = (1.0 - assign5330_e5873);
        let assign5330_e5877: f64 = (1.0 - p.p43);
        let assign5330_e5878: f64 = (assign5330_e5874).powf(assign5330_e5877);
        let assign5330_e5879: f64 = (1.0 - assign5330_e5878);
        let assign5330_e5880: f64 = (locals.var_pc_t * assign5330_e5879);
        let assign5330_e5883: f64 = (1.0 - p.p43);
        let assign5330_e5884: f64 = (assign5330_e5880 / assign5330_e5883);
        (assign5330_e5884, (((locals.var_pc_t_dn4 * assign5330_e5879) + (locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(((locals.var_vl__blk173_dn4 * locals.var_pc_t) - (locals.var_vl__blk173 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(((locals.var_vl__blk173_dn4 * locals.var_pc_t) - (locals.var_vl__blk173 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign5330_e5874))) }))) / assign5330_e5883), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(locals.var_vl__blk173_dn7 / locals.var_pc_t)))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(locals.var_vl__blk173_dn7 / locals.var_pc_t)) / assign5330_e5874))) })) / assign5330_e5883), ((locals.var_pc_t * (-if 0.0 == 0.0 && ((assign5330_e5877) as f64).is_finite() && ((assign5330_e5877) as f64).fract() == 0.0 { if assign5330_e5877 == 0.0 { 0.0 } else { (assign5330_e5877 * ((assign5330_e5874).powf(assign5330_e5877 - 1.0) * (-(locals.var_vl__blk173_dn10 / locals.var_pc_t)))) } } else { (assign5330_e5878 * (assign5330_e5877 * ((-(locals.var_vl__blk173_dn10 / locals.var_pc_t)) / assign5330_e5874))) })) / assign5330_e5883),)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5330_e5886;
        locals.var_qlo__blk165_dn4 = assign5330_e5886_d_n4;
        locals.var_qlo__blk165_dn7 = assign5330_e5886_d_n7;
        locals.var_qlo__blk165_dn10 = assign5330_e5886_d_n10;

        let (assign5340_e5897, assign5340_e5897_d_n4, assign5340_e5897_d_n7, assign5340_e5897_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5340_e5894: f64 = (locals.var_vnl__blk172 + 1.0);
        let assign5340_e5895: f64 = (0.5 * assign5340_e5894);
        (assign5340_e5895, (0.5 * locals.var_vnl__blk172_dn4), (0.5 * locals.var_vnl__blk172_dn7), (0.5 * locals.var_vnl__blk172_dn10),)
    } else {
        (locals.var_sel__blk174, locals.var_sel__blk174_dn4, locals.var_sel__blk174_dn7, locals.var_sel__blk174_dn10,)
    }
};
        locals.var_sel__blk174 = assign5340_e5897;
        locals.var_sel__blk174_dn4 = assign5340_e5897_d_n4;
        locals.var_sel__blk174_dn7 = assign5340_e5897_d_n7;
        locals.var_sel__blk174_dn10 = assign5340_e5897_d_n10;

        let (assign5350_e5911, assign5350_e5911_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5350_e5905: f64 = (p.p45 / locals.var_pc_t);
        let assign5350_e5906: f64 = (1.0 + assign5350_e5905);
        let assign5350_e5908: f64 = (-p.p43);
        let assign5350_e5909: f64 = (assign5350_e5906).powf(assign5350_e5908);
        (assign5350_e5909, if 0.0 == 0.0 && ((assign5350_e5908) as f64).is_finite() && ((assign5350_e5908) as f64).fract() == 0.0 { if assign5350_e5908 == 0.0 { 0.0 } else { (assign5350_e5908 * ((assign5350_e5906).powf(assign5350_e5908 - 1.0) * (-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5350_e5909 * (assign5350_e5908 * ((-((p.p45 * locals.var_pc_t_dn4) / (locals.var_pc_t * locals.var_pc_t))) / assign5350_e5906))) },)
    } else {
        (locals.var_crt__blk175, locals.var_crt__blk175_dn4,)
    }
};
        locals.var_crt__blk175 = assign5350_e5911;
        locals.var_crt__blk175_dn4 = assign5350_e5911_d_n4;

        let (assign5360_e5925, assign5360_e5925_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5360_e5919: f64 = (locals.var_dv0__blk162 / locals.var_pc_t);
        let assign5360_e5920: f64 = (1.0 + assign5360_e5919);
        let assign5360_e5922: f64 = (-p.p43);
        let assign5360_e5923: f64 = (assign5360_e5920).powf(assign5360_e5922);
        (assign5360_e5923, if 0.0 == 0.0 && ((assign5360_e5922) as f64).is_finite() && ((assign5360_e5922) as f64).fract() == 0.0 { if assign5360_e5922 == 0.0 { 0.0 } else { (assign5360_e5922 * ((assign5360_e5920).powf(assign5360_e5922 - 1.0) * (((locals.var_dv0__blk162_dn4 * locals.var_pc_t) - (locals.var_dv0__blk162 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)))) } } else { (assign5360_e5923 * (assign5360_e5922 * ((((locals.var_dv0__blk162_dn4 * locals.var_pc_t) - (locals.var_dv0__blk162 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t)) / assign5360_e5920))) },)
    } else {
        (locals.var_cmx__blk176, locals.var_cmx__blk176_dn4,)
    }
};
        locals.var_cmx__blk176 = assign5360_e5925;
        locals.var_cmx__blk176_dn4 = assign5360_e5925_d_n4;

        let (assign5370_e5940, assign5370_e5940_d_n4, assign5370_e5940_d_n7, assign5370_e5940_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5370_e5932: f64 = (1.0 - locals.var_sel__blk174);
        let assign5370_e5934: f64 = (assign5370_e5932 * locals.var_crt__blk175);
        let assign5370_e5937: f64 = (locals.var_sel__blk174 * locals.var_cmx__blk176);
        let assign5370_e5938: f64 = (assign5370_e5934 + assign5370_e5937);
        (assign5370_e5938, ((((-locals.var_sel__blk174_dn4) * locals.var_crt__blk175) + (assign5370_e5932 * locals.var_crt__blk175_dn4)) + ((locals.var_sel__blk174_dn4 * locals.var_cmx__blk176) + (locals.var_sel__blk174 * locals.var_cmx__blk176_dn4))), (((-locals.var_sel__blk174_dn7) * locals.var_crt__blk175) + (locals.var_sel__blk174_dn7 * locals.var_cmx__blk176)), (((-locals.var_sel__blk174_dn10) * locals.var_crt__blk175) + (locals.var_sel__blk174_dn10 * locals.var_cmx__blk176)),)
    } else {
        (locals.var_cl__blk177, locals.var_cl__blk177_dn4, locals.var_cl__blk177_dn7, locals.var_cl__blk177_dn10,)
    }
};
        locals.var_cl__blk177 = assign5370_e5940;
        locals.var_cl__blk177_dn4 = assign5370_e5940_d_n4;
        locals.var_cl__blk177_dn7 = assign5370_e5940_d_n7;
        locals.var_cl__blk177_dn10 = assign5370_e5940_d_n10;

        let (assign5380_e5953, assign5380_e5953_d_n4, assign5380_e5953_d_n7, assign5380_e5953_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5380_e5947: f64 = (locals.var_vbep - locals.var_vl__blk173);
        let assign5380_e5949: f64 = (assign5380_e5947 + locals.var_vl0__blk169);
        let assign5380_e5951: f64 = (assign5380_e5949 * locals.var_cl__blk177);
        (assign5380_e5951, ((((-locals.var_vl__blk173_dn4) + locals.var_vl0__blk169_dn4) * locals.var_cl__blk177) + (assign5380_e5949 * locals.var_cl__blk177_dn4)), (((locals.var_vbep_dn7 - locals.var_vl__blk173_dn7) * locals.var_cl__blk177) + (assign5380_e5949 * locals.var_cl__blk177_dn7)), (((locals.var_vbep_dn10 - locals.var_vl__blk173_dn10) * locals.var_cl__blk177) + (assign5380_e5949 * locals.var_cl__blk177_dn10)),)
    } else {
        (locals.var_ql__blk178, locals.var_ql__blk178_dn4, locals.var_ql__blk178_dn7, locals.var_ql__blk178_dn10,)
    }
};
        locals.var_ql__blk178 = assign5380_e5953;
        locals.var_ql__blk178_dn4 = assign5380_e5953_d_n4;
        locals.var_ql__blk178_dn7 = assign5380_e5953_d_n7;
        locals.var_ql__blk178_dn10 = assign5380_e5953_d_n10;

        let (assign5390_e5964, assign5390_e5964_d_n4, assign5390_e5964_d_n7, assign5390_e5964_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 != 0.0)) {
        let assign5390_e5960: f64 = (locals.var_ql__blk178 + locals.var_qlo__blk165);
        let assign5390_e5962: f64 = (assign5390_e5960 - locals.var_qlo0__blk170);
        (assign5390_e5962, ((locals.var_ql__blk178_dn4 + locals.var_qlo__blk165_dn4) - locals.var_qlo0__blk170_dn4), (locals.var_ql__blk178_dn7 + locals.var_qlo__blk165_dn7), (locals.var_ql__blk178_dn10 + locals.var_qlo__blk165_dn10),)
    } else {
        (locals.var_qdbep, locals.var_qdbep_dn4, locals.var_qdbep_dn7, locals.var_qdbep_dn10,)
    }
};
        locals.var_qdbep = assign5390_e5964;
        locals.var_qdbep_dn4 = assign5390_e5964_d_n4;
        locals.var_qdbep_dn7 = assign5390_e5964_d_n7;
        locals.var_qdbep_dn10 = assign5390_e5964_d_n10;

        let (assign5400_e5981, assign5400_e5981_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5400_e5972: f64 = (locals.var_dv0__blk162 * locals.var_dv0__blk162);
        let assign5400_e5975: f64 = (4.0 * p.p44);
        let assign5400_e5977: f64 = (assign5400_e5975 * p.p44);
        let assign5400_e5978: f64 = (assign5400_e5972 + assign5400_e5977);
        let assign5400_e5979: f64 = (assign5400_e5978).sqrt();
        (assign5400_e5979, (((locals.var_dv0__blk162_dn4 * locals.var_dv0__blk162) + (locals.var_dv0__blk162 * locals.var_dv0__blk162_dn4)) / (2.0 * assign5400_e5979)),)
    } else {
        (locals.var_mv0__blk179, locals.var_mv0__blk179_dn4,)
    }
};
        locals.var_mv0__blk179 = assign5400_e5981;
        locals.var_mv0__blk179_dn4 = assign5400_e5981_d_n4;

        let (assign5410_e5994, assign5410_e5994_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5410_e5988: f64 = (-0.5);
        let assign5410_e5991: f64 = (locals.var_dv0__blk162 + locals.var_mv0__blk179);
        let assign5410_e5992: f64 = (assign5410_e5988 * assign5410_e5991);
        (assign5410_e5992, (assign5410_e5988 * (locals.var_dv0__blk162_dn4 + locals.var_mv0__blk179_dn4)),)
    } else {
        (locals.var_vl0__blk169, locals.var_vl0__blk169_dn4,)
    }
};
        locals.var_vl0__blk169 = assign5410_e5994;
        locals.var_vl0__blk169_dn4 = assign5410_e5994_d_n4;

        let (assign5420_e6017, assign5420_e6017_d_n4,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5420_e6001: f64 = (-locals.var_pc_t);
        let assign5420_e6005: f64 = (locals.var_vl0__blk169 / locals.var_pc_t);
        let assign5420_e6006: f64 = (1.0 - assign5420_e6005);
        let assign5420_e6009: f64 = (1.0 - p.p43);
        let assign5420_e6010: f64 = (assign5420_e6006).powf(assign5420_e6009);
        let assign5420_e6011: f64 = (assign5420_e6001 * assign5420_e6010);
        let assign5420_e6014: f64 = (1.0 - p.p43);
        let assign5420_e6015: f64 = (assign5420_e6011 / assign5420_e6014);
        (assign5420_e6015, ((((-locals.var_pc_t_dn4) * assign5420_e6010) + (assign5420_e6001 * if 0.0 == 0.0 && ((assign5420_e6009) as f64).is_finite() && ((assign5420_e6009) as f64).fract() == 0.0 { if assign5420_e6009 == 0.0 { 0.0 } else { (assign5420_e6009 * ((assign5420_e6006).powf(assign5420_e6009 - 1.0) * (-(((locals.var_vl0__blk169_dn4 * locals.var_pc_t) - (locals.var_vl0__blk169 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5420_e6010 * (assign5420_e6009 * ((-(((locals.var_vl0__blk169_dn4 * locals.var_pc_t) - (locals.var_vl0__blk169 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign5420_e6006))) })) / assign5420_e6014),)
    } else {
        (locals.var_q0__blk180, locals.var_q0__blk180_dn4,)
    }
};
        locals.var_q0__blk180 = assign5420_e6017;
        locals.var_q0__blk180_dn4 = assign5420_e6017_d_n4;

        let (assign5430_e6027, assign5430_e6027_d_n4, assign5430_e6027_d_n7, assign5430_e6027_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5430_e6025: f64 = (locals.var_vbep + locals.var_dv0__blk162);
        (assign5430_e6025, locals.var_dv0__blk162_dn4, locals.var_vbep_dn7, locals.var_vbep_dn10,)
    } else {
        (locals.var_dv__blk181, locals.var_dv__blk181_dn4, locals.var_dv__blk181_dn7, locals.var_dv__blk181_dn10,)
    }
};
        locals.var_dv__blk181 = assign5430_e6027;
        locals.var_dv__blk181_dn4 = assign5430_e6027_d_n4;
        locals.var_dv__blk181_dn7 = assign5430_e6027_d_n7;
        locals.var_dv__blk181_dn10 = assign5430_e6027_d_n10;

        let (assign5440_e6044, assign5440_e6044_d_n4, assign5440_e6044_d_n7, assign5440_e6044_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5440_e6035: f64 = (locals.var_dv__blk181 * locals.var_dv__blk181);
        let assign5440_e6038: f64 = (4.0 * p.p44);
        let assign5440_e6040: f64 = (assign5440_e6038 * p.p44);
        let assign5440_e6041: f64 = (assign5440_e6035 + assign5440_e6040);
        let assign5440_e6042: f64 = (assign5440_e6041).sqrt();
        (assign5440_e6042, (((locals.var_dv__blk181_dn4 * locals.var_dv__blk181) + (locals.var_dv__blk181 * locals.var_dv__blk181_dn4)) / (2.0 * assign5440_e6042)), (((locals.var_dv__blk181_dn7 * locals.var_dv__blk181) + (locals.var_dv__blk181 * locals.var_dv__blk181_dn7)) / (2.0 * assign5440_e6042)), (((locals.var_dv__blk181_dn10 * locals.var_dv__blk181) + (locals.var_dv__blk181 * locals.var_dv__blk181_dn10)) / (2.0 * assign5440_e6042)),)
    } else {
        (locals.var_mv__blk182, locals.var_mv__blk182_dn4, locals.var_mv__blk182_dn7, locals.var_mv__blk182_dn10,)
    }
};
        locals.var_mv__blk182 = assign5440_e6044;
        locals.var_mv__blk182_dn4 = assign5440_e6044_d_n4;
        locals.var_mv__blk182_dn7 = assign5440_e6044_d_n7;
        locals.var_mv__blk182_dn10 = assign5440_e6044_d_n10;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5450_e6058, assign5450_e6058_d_n4, assign5450_e6058_d_n7, assign5450_e6058_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5450_e6053: f64 = (locals.var_dv__blk181 - locals.var_mv__blk182);
        let assign5450_e6054: f64 = (0.5 * assign5450_e6053);
        let assign5450_e6056: f64 = (assign5450_e6054 - locals.var_dv0__blk162);
        (assign5450_e6056, ((0.5 * (locals.var_dv__blk181_dn4 - locals.var_mv__blk182_dn4)) - locals.var_dv0__blk162_dn4), (0.5 * (locals.var_dv__blk181_dn7 - locals.var_mv__blk182_dn7)), (0.5 * (locals.var_dv__blk181_dn10 - locals.var_mv__blk182_dn10)),)
    } else {
        (locals.var_vl__blk173, locals.var_vl__blk173_dn4, locals.var_vl__blk173_dn7, locals.var_vl__blk173_dn10,)
    }
};
        locals.var_vl__blk173 = assign5450_e6058;
        locals.var_vl__blk173_dn4 = assign5450_e6058_d_n4;
        locals.var_vl__blk173_dn7 = assign5450_e6058_d_n7;
        locals.var_vl__blk173_dn10 = assign5450_e6058_d_n10;

        let (assign5460_e6081, assign5460_e6081_d_n4, assign5460_e6081_d_n7, assign5460_e6081_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5460_e6065: f64 = (-locals.var_pc_t);
        let assign5460_e6069: f64 = (locals.var_vl__blk173 / locals.var_pc_t);
        let assign5460_e6070: f64 = (1.0 - assign5460_e6069);
        let assign5460_e6073: f64 = (1.0 - p.p43);
        let assign5460_e6074: f64 = (assign5460_e6070).powf(assign5460_e6073);
        let assign5460_e6075: f64 = (assign5460_e6065 * assign5460_e6074);
        let assign5460_e6078: f64 = (1.0 - p.p43);
        let assign5460_e6079: f64 = (assign5460_e6075 / assign5460_e6078);
        (assign5460_e6079, ((((-locals.var_pc_t_dn4) * assign5460_e6074) + (assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(((locals.var_vl__blk173_dn4 * locals.var_pc_t) - (locals.var_vl__blk173 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(((locals.var_vl__blk173_dn4 * locals.var_pc_t) - (locals.var_vl__blk173 * locals.var_pc_t_dn4)) / (locals.var_pc_t * locals.var_pc_t))) / assign5460_e6070))) })) / assign5460_e6078), ((assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(locals.var_vl__blk173_dn7 / locals.var_pc_t)))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(locals.var_vl__blk173_dn7 / locals.var_pc_t)) / assign5460_e6070))) }) / assign5460_e6078), ((assign5460_e6065 * if 0.0 == 0.0 && ((assign5460_e6073) as f64).is_finite() && ((assign5460_e6073) as f64).fract() == 0.0 { if assign5460_e6073 == 0.0 { 0.0 } else { (assign5460_e6073 * ((assign5460_e6070).powf(assign5460_e6073 - 1.0) * (-(locals.var_vl__blk173_dn10 / locals.var_pc_t)))) } } else { (assign5460_e6074 * (assign5460_e6073 * ((-(locals.var_vl__blk173_dn10 / locals.var_pc_t)) / assign5460_e6070))) }) / assign5460_e6078),)
    } else {
        (locals.var_qlo__blk165, locals.var_qlo__blk165_dn4, locals.var_qlo__blk165_dn7, locals.var_qlo__blk165_dn10,)
    }
};
        locals.var_qlo__blk165 = assign5460_e6081;
        locals.var_qlo__blk165_dn4 = assign5460_e6081_d_n4;
        locals.var_qlo__blk165_dn7 = assign5460_e6081_d_n7;
        locals.var_qlo__blk165_dn10 = assign5460_e6081_d_n10;

        let (assign5470_e6104, assign5470_e6104_d_n4, assign5470_e6104_d_n7, assign5470_e6104_d_n10,) = {
    if ((locals.var_guard183 == 0.0) && (locals.var_guard186 == 0.0)) {
        let assign5470_e6090: f64 = (1.0 - p.p34);
        let assign5470_e6092: f64 = (-p.p43);
        let assign5470_e6093: f64 = (assign5470_e6090).powf(assign5470_e6092);
        let assign5470_e6096: f64 = (locals.var_vbep - locals.var_vl__blk173);
        let assign5470_e6098: f64 = (assign5470_e6096 + locals.var_vl0__blk169);
        let assign5470_e6099: f64 = (assign5470_e6093 * assign5470_e6098);
        let assign5470_e6100: f64 = (locals.var_qlo__blk165 + assign5470_e6099);
        let assign5470_e6102: f64 = (assign5470_e6100 - locals.var_q0__blk180);
        (assign5470_e6102, ((locals.var_qlo__blk165_dn4 + (assign5470_e6093 * ((-locals.var_vl__blk173_dn4) + locals.var_vl0__blk169_dn4))) - locals.var_q0__blk180_dn4), (locals.var_qlo__blk165_dn7 + (assign5470_e6093 * (locals.var_vbep_dn7 - locals.var_vl__blk173_dn7))), (locals.var_qlo__blk165_dn10 + (assign5470_e6093 * (locals.var_vbep_dn10 - locals.var_vl__blk173_dn10))),)
    } else {
        (locals.var_qdbep, locals.var_qdbep_dn4, locals.var_qdbep_dn7, locals.var_qdbep_dn10,)
    }
};
        locals.var_qdbep = assign5470_e6104;
        locals.var_qdbep_dn4 = assign5470_e6104_d_n4;
        locals.var_qdbep_dn7 = assign5470_e6104_d_n7;
        locals.var_qdbep_dn10 = assign5470_e6104_d_n10;

        let (assign5480_e6110,) = {
    if (locals.var_ifi > 0.0) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        locals.var_sgif = assign5480_e6110;

        let assign5490_e6113: f64 = (locals.var_ifi * locals.var_sgif);
        let assign5490_e6115: f64 = (assign5490_e6113 * locals.var_iitf);
        locals.var_rif = assign5490_e6115;
        locals.var_rif_dn4 = ((locals.var_ifi_dn4 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn5 = ((locals.var_ifi_dn5 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn6 = ((locals.var_ifi_dn6 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn7 = ((locals.var_ifi_dn7 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn8 = ((locals.var_ifi_dn8 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn9 = ((locals.var_ifi_dn9 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn10 = ((locals.var_ifi_dn10 * locals.var_sgif) * locals.var_iitf);
        locals.var_rif_dn11 = ((locals.var_ifi_dn11 * locals.var_sgif) * locals.var_iitf);

        let assign5500_e6119: f64 = (locals.var_rif + 1.0);
        let assign5500_e6120: f64 = (locals.var_rif / assign5500_e6119);
        locals.var_mif = assign5500_e6120;
        locals.var_mif_dn4 = (((locals.var_rif_dn4 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn4)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn5 = (((locals.var_rif_dn5 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn5)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn6 = (((locals.var_rif_dn6 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn6)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn7 = (((locals.var_rif_dn7 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn7)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn8 = (((locals.var_rif_dn8 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn8)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn9 = (((locals.var_rif_dn9 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn9)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn10 = (((locals.var_rif_dn10 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn10)) / (assign5500_e6119 * assign5500_e6119));
        locals.var_mif_dn11 = (((locals.var_rif_dn11 * assign5500_e6119) - (locals.var_rif * locals.var_rif_dn11)) / (assign5500_e6119 * assign5500_e6119));

        let assign5510_e6123: f64 = (locals.var_vbci * locals.var_ivtf);
        let assign5510_e6125: f64 = (assign5510_e6123 / 1.44);
        locals.var_arg = assign5510_e6125;
        locals.var_arg_dn4 = 0.0;
        locals.var_arg_dn5 = 0.0;
        locals.var_arg_dn6 = ((locals.var_vbci_dn6 * locals.var_ivtf) / 1.44);
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = ((locals.var_vbci_dn8 * locals.var_ivtf) / 1.44);
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;

        let assign5520_e6128: f64 = if locals.var_arg < locals.var_vmaxexp { 1.0 } else { 0.0 };
        locals.var_guard187 = assign5520_e6128;

        let (assign5530_e6133, assign5530_e6133_d_n4, assign5530_e6133_d_n5, assign5530_e6133_d_n6, assign5530_e6133_d_n7, assign5530_e6133_d_n8, assign5530_e6133_d_n9, assign5530_e6133_d_n10, assign5530_e6133_d_n11,) = {
    if (locals.var_guard187 != 0.0) {
        let assign5530_e6131: f64 = (locals.var_arg).exp();
        (assign5530_e6131, (assign5530_e6131 * locals.var_arg_dn4), (assign5530_e6131 * locals.var_arg_dn5), (assign5530_e6131 * locals.var_arg_dn6), (assign5530_e6131 * locals.var_arg_dn7), (assign5530_e6131 * locals.var_arg_dn8), (assign5530_e6131 * locals.var_arg_dn9), (assign5530_e6131 * locals.var_arg_dn10), (assign5530_e6131 * locals.var_arg_dn11),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign5530_e6133;
        locals.var_expi_dn4 = assign5530_e6133_d_n4;
        locals.var_expi_dn5 = assign5530_e6133_d_n5;
        locals.var_expi_dn6 = assign5530_e6133_d_n6;
        locals.var_expi_dn7 = assign5530_e6133_d_n7;
        locals.var_expi_dn8 = assign5530_e6133_d_n8;
        locals.var_expi_dn9 = assign5530_e6133_d_n9;
        locals.var_expi_dn10 = assign5530_e6133_d_n10;
        locals.var_expi_dn11 = assign5530_e6133_d_n11;

        let (assign5540_e6145, assign5540_e6145_d_n4, assign5540_e6145_d_n5, assign5540_e6145_d_n6, assign5540_e6145_d_n7, assign5540_e6145_d_n8, assign5540_e6145_d_n9, assign5540_e6145_d_n10, assign5540_e6145_d_n11,) = {
    if (locals.var_guard187 == 0.0) {
        let assign5540_e6137: f64 = (locals.var_vmaxexp).exp();
        let assign5540_e6141: f64 = (locals.var_arg - locals.var_vmaxexp);
        let assign5540_e6142: f64 = (1.0 + assign5540_e6141);
        let assign5540_e6143: f64 = (assign5540_e6137 * assign5540_e6142);
        (assign5540_e6143, (assign5540_e6137 * locals.var_arg_dn4), (assign5540_e6137 * locals.var_arg_dn5), (assign5540_e6137 * locals.var_arg_dn6), (assign5540_e6137 * locals.var_arg_dn7), (assign5540_e6137 * locals.var_arg_dn8), (assign5540_e6137 * locals.var_arg_dn9), (assign5540_e6137 * locals.var_arg_dn10), (assign5540_e6137 * locals.var_arg_dn11),)
    } else {
        (locals.var_expi, locals.var_expi_dn4, locals.var_expi_dn5, locals.var_expi_dn6, locals.var_expi_dn7, locals.var_expi_dn8, locals.var_expi_dn9, locals.var_expi_dn10, locals.var_expi_dn11,)
    }
};
        locals.var_expi = assign5540_e6145;
        locals.var_expi_dn4 = assign5540_e6145_d_n4;
        locals.var_expi_dn5 = assign5540_e6145_d_n5;
        locals.var_expi_dn6 = assign5540_e6145_d_n6;
        locals.var_expi_dn7 = assign5540_e6145_d_n7;
        locals.var_expi_dn8 = assign5540_e6145_d_n8;
        locals.var_expi_dn9 = assign5540_e6145_d_n9;
        locals.var_expi_dn10 = assign5540_e6145_d_n10;
        locals.var_expi_dn11 = assign5540_e6145_d_n11;

        let assign5550_e6150: f64 = (p.p77 * locals.var_q1);
        let assign5550_e6151: f64 = (1.0 + assign5550_e6150);
        let assign5550_e6152: f64 = (p.p76 * assign5550_e6151);
        let assign5550_e6156: f64 = (p.p78 * locals.var_expi);
        let assign5550_e6160: f64 = (locals.var_mif * locals.var_mif);
        let assign5550_e6161: f64 = (locals.var_sltf + assign5550_e6160);
        let assign5550_e6162: f64 = (assign5550_e6156 * assign5550_e6161);
        let assign5550_e6164: f64 = (assign5550_e6162 * locals.var_sgif);
        let assign5550_e6165: f64 = (1.0 + assign5550_e6164);
        let assign5550_e6166: f64 = (assign5550_e6152 * assign5550_e6165);
        locals.var_tff = assign5550_e6166;
        locals.var_tff_dn4 = (((p.p76 * (p.p77 * locals.var_q1_dn4)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn4) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn4 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn4)))) * locals.var_sgif)));
        locals.var_tff_dn5 = (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn5) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn5 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn5)))) * locals.var_sgif));
        locals.var_tff_dn6 = (((p.p76 * (p.p77 * locals.var_q1_dn6)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn6) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn6 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn6)))) * locals.var_sgif)));
        locals.var_tff_dn7 = (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn7) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn7 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn7)))) * locals.var_sgif));
        locals.var_tff_dn8 = (((p.p76 * (p.p77 * locals.var_q1_dn8)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn8) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn8 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn8)))) * locals.var_sgif)));
        locals.var_tff_dn9 = (((p.p76 * (p.p77 * locals.var_q1_dn9)) * assign5550_e6165) + (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn9) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn9 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn9)))) * locals.var_sgif)));
        locals.var_tff_dn10 = (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn10) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn10 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn10)))) * locals.var_sgif));
        locals.var_tff_dn11 = (assign5550_e6152 * ((((p.p78 * locals.var_expi_dn11) * assign5550_e6161) + (assign5550_e6156 * ((locals.var_mif_dn11 * locals.var_mif) + (locals.var_mif * locals.var_mif_dn11)))) * locals.var_sgif));

        let assign5560_e6169: f64 = (locals.var_cje_t * locals.var_qdbe);
        let assign5560_e6171: f64 = (assign5560_e6169 * p.p55);
        let assign5560_e6174: f64 = (locals.var_tff * locals.var_ifi);
        let assign5560_e6176: f64 = (assign5560_e6174 / locals.var_qb);
        let assign5560_e6177: f64 = (assign5560_e6171 + assign5560_e6176);
        locals.var_qbe = assign5560_e6177;
        locals.var_qbe_dn4 = ((((locals.var_cje_t_dn4 * locals.var_qdbe) + (locals.var_cje_t * locals.var_qdbe_dn4)) * p.p55) + (((((locals.var_tff_dn4 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn4)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn4)) / (locals.var_qb * locals.var_qb)));
        locals.var_qbe_dn5 = (((((locals.var_tff_dn5 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn5)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn5)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_dn6 = (((((locals.var_tff_dn6 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn6)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn6)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_dn7 = (((((locals.var_tff_dn7 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn7)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn7)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_dn8 = (((locals.var_cje_t * locals.var_qdbe_dn8) * p.p55) + (((((locals.var_tff_dn8 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn8)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn8)) / (locals.var_qb * locals.var_qb)));
        locals.var_qbe_dn9 = (((locals.var_cje_t * locals.var_qdbe_dn9) * p.p55) + (((((locals.var_tff_dn9 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn9)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn9)) / (locals.var_qb * locals.var_qb)));
        locals.var_qbe_dn10 = (((((locals.var_tff_dn10 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn10)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn10)) / (locals.var_qb * locals.var_qb));
        locals.var_qbe_dn11 = (((((locals.var_tff_dn11 * locals.var_ifi) + (locals.var_tff * locals.var_ifi_dn11)) * locals.var_qb) - (assign5560_e6174 * locals.var_qb_dn11)) / (locals.var_qb * locals.var_qb));

        let assign5570_e6180: f64 = (locals.var_cje_t * locals.var_qdbex);
        let assign5570_e6183: f64 = (1.0 - p.p55);
        let assign5570_e6184: f64 = (assign5570_e6180 * assign5570_e6183);
        locals.var_qbex = assign5570_e6184;
        locals.var_qbex_dn4 = (((locals.var_cje_t_dn4 * locals.var_qdbex) + (locals.var_cje_t * locals.var_qdbex_dn4)) * assign5570_e6183);
        locals.var_qbex_dn7 = ((locals.var_cje_t * locals.var_qdbex_dn7) * assign5570_e6183);
        locals.var_qbex_dn9 = ((locals.var_cje_t * locals.var_qdbex_dn9) * assign5570_e6183);

        let assign5580_e6187: f64 = (locals.var_cjc_t * locals.var_qdbc);
        let assign5580_e6190: f64 = (p.p81 * locals.var_iri);
        let assign5580_e6191: f64 = (assign5580_e6187 + assign5580_e6190);
        let assign5580_e6194: f64 = (p.p47 * locals.var_kbci);
        let assign5580_e6195: f64 = (assign5580_e6191 + assign5580_e6194);
        locals.var_qbc = assign5580_e6195;
        locals.var_qbc_dn4 = ((((locals.var_cjc_t_dn4 * locals.var_qdbc) + (locals.var_cjc_t * locals.var_qdbc_dn4)) + (p.p81 * locals.var_iri_dn4)) + (p.p47 * locals.var_kbci_dn4));
        locals.var_qbc_dn5 = ((p.p81 * locals.var_iri_dn5) + (p.p47 * locals.var_kbci_dn5));
        locals.var_qbc_dn6 = (((locals.var_cjc_t * locals.var_qdbc_dn6) + (p.p81 * locals.var_iri_dn6)) + (p.p47 * locals.var_kbci_dn6));
        locals.var_qbc_dn7 = ((p.p81 * locals.var_iri_dn7) + (p.p47 * locals.var_kbci_dn7));
        locals.var_qbc_dn8 = (((locals.var_cjc_t * locals.var_qdbc_dn8) + (p.p81 * locals.var_iri_dn8)) + (p.p47 * locals.var_kbci_dn8));
        locals.var_qbc_dn9 = ((p.p81 * locals.var_iri_dn9) + (p.p47 * locals.var_kbci_dn9));
        locals.var_qbc_dn10 = ((p.p81 * locals.var_iri_dn10) + (p.p47 * locals.var_kbci_dn10));
        locals.var_qbc_dn11 = ((p.p81 * locals.var_iri_dn11) + (p.p47 * locals.var_kbci_dn11));

        let assign5590_e6198: f64 = (p.p47 * locals.var_kbcx);
        locals.var_qbcx = assign5590_e6198;
        locals.var_qbcx_dn4 = (p.p47 * locals.var_kbcx_dn4);
        locals.var_qbcx_dn5 = (p.p47 * locals.var_kbcx_dn5);
        locals.var_qbcx_dn6 = (p.p47 * locals.var_kbcx_dn6);
        locals.var_qbcx_dn7 = (p.p47 * locals.var_kbcx_dn7);
        locals.var_qbcx_dn8 = (p.p47 * locals.var_kbcx_dn8);
        locals.var_qbcx_dn9 = (p.p47 * locals.var_kbcx_dn9);
        locals.var_qbcx_dn10 = (p.p47 * locals.var_kbcx_dn10);
        locals.var_qbcx_dn11 = (p.p47 * locals.var_kbcx_dn11);

        let assign5600_e6201: f64 = (locals.var_cjep_t * locals.var_qdbep);
        let assign5600_e6204: f64 = (p.p81 * locals.var_ifp);
        let assign5600_e6205: f64 = (assign5600_e6201 + assign5600_e6204);
        locals.var_qbep = assign5600_e6205;
        locals.var_qbep_dn4 = (((locals.var_cjep_t_dn4 * locals.var_qdbep) + (locals.var_cjep_t * locals.var_qdbep_dn4)) + (p.p81 * locals.var_ifp_dn4));
        locals.var_qbep_dn5 = (p.p81 * locals.var_ifp_dn5);
        locals.var_qbep_dn6 = (p.p81 * locals.var_ifp_dn6);
        locals.var_qbep_dn7 = ((locals.var_cjep_t * locals.var_qdbep_dn7) + (p.p81 * locals.var_ifp_dn7));
        locals.var_qbep_dn8 = (p.p81 * locals.var_ifp_dn8);
        locals.var_qbep_dn9 = (p.p81 * locals.var_ifp_dn9);
        locals.var_qbep_dn10 = ((locals.var_cjep_t * locals.var_qdbep_dn10) + (p.p81 * locals.var_ifp_dn10));
        locals.var_qbep_dn11 = (p.p81 * locals.var_ifp_dn11);

        let assign5610_e6208: f64 = (locals.var_cjcp_t * locals.var_qdbcp);
        let assign5610_e6211: f64 = (p.p53 * locals.var_vbcp);
        let assign5610_e6212: f64 = (assign5610_e6208 + assign5610_e6211);
        locals.var_qbcp = assign5610_e6212;
        locals.var_qbcp_dn4 = ((locals.var_cjcp_t_dn4 * locals.var_qdbcp) + (locals.var_cjcp_t * locals.var_qdbcp_dn4));
        locals.var_qbcp_dn10 = ((locals.var_cjcp_t * locals.var_qdbcp_dn10) + (p.p53 * locals.var_vbcp_dn10));
        locals.var_qbcp_dn11 = ((locals.var_cjcp_t * locals.var_qdbcp_dn11) + (p.p53 * locals.var_vbcp_dn11));

        let assign5640_e6221: f64 = (locals.var_dt_et * p.p102);
        locals.var_qcth = assign5640_e6221;
        locals.var_qcth_dn4 = (locals.var_dt_et_dn4 * p.p102);

        let assign5670_e6232: f64 = locals.var_vbictype;
        let assign5670_e6234: f64 = (assign5670_e6232 * locals.var_qbe);
        locals.var_qbe = assign5670_e6234;
        locals.var_qbe_dn4 = (assign5670_e6232 * locals.var_qbe_dn4);
        locals.var_qbe_dn5 = (assign5670_e6232 * locals.var_qbe_dn5);
        locals.var_qbe_dn6 = (assign5670_e6232 * locals.var_qbe_dn6);
        locals.var_qbe_dn7 = (assign5670_e6232 * locals.var_qbe_dn7);
        locals.var_qbe_dn8 = (assign5670_e6232 * locals.var_qbe_dn8);
        locals.var_qbe_dn9 = (assign5670_e6232 * locals.var_qbe_dn9);
        locals.var_qbe_dn10 = (assign5670_e6232 * locals.var_qbe_dn10);
        locals.var_qbe_dn11 = (assign5670_e6232 * locals.var_qbe_dn11);

        let assign5680_e6237: f64 = locals.var_vbictype;
        let assign5680_e6239: f64 = (assign5680_e6237 * locals.var_qbex);
        locals.var_qbex = assign5680_e6239;
        locals.var_qbex_dn4 = (assign5680_e6237 * locals.var_qbex_dn4);
        locals.var_qbex_dn7 = (assign5680_e6237 * locals.var_qbex_dn7);
        locals.var_qbex_dn9 = (assign5680_e6237 * locals.var_qbex_dn9);

        let assign5690_e6242: f64 = locals.var_vbictype;
        let assign5690_e6244: f64 = (assign5690_e6242 * locals.var_qbc);
        locals.var_qbc = assign5690_e6244;
        locals.var_qbc_dn4 = (assign5690_e6242 * locals.var_qbc_dn4);
        locals.var_qbc_dn5 = (assign5690_e6242 * locals.var_qbc_dn5);
        locals.var_qbc_dn6 = (assign5690_e6242 * locals.var_qbc_dn6);
        locals.var_qbc_dn7 = (assign5690_e6242 * locals.var_qbc_dn7);
        locals.var_qbc_dn8 = (assign5690_e6242 * locals.var_qbc_dn8);
        locals.var_qbc_dn9 = (assign5690_e6242 * locals.var_qbc_dn9);
        locals.var_qbc_dn10 = (assign5690_e6242 * locals.var_qbc_dn10);
        locals.var_qbc_dn11 = (assign5690_e6242 * locals.var_qbc_dn11);

        let assign5700_e6247: f64 = locals.var_vbictype;
        let assign5700_e6249: f64 = (assign5700_e6247 * locals.var_qbcx);
        locals.var_qbcx = assign5700_e6249;
        locals.var_qbcx_dn4 = (assign5700_e6247 * locals.var_qbcx_dn4);
        locals.var_qbcx_dn5 = (assign5700_e6247 * locals.var_qbcx_dn5);
        locals.var_qbcx_dn6 = (assign5700_e6247 * locals.var_qbcx_dn6);
        locals.var_qbcx_dn7 = (assign5700_e6247 * locals.var_qbcx_dn7);
        locals.var_qbcx_dn8 = (assign5700_e6247 * locals.var_qbcx_dn8);
        locals.var_qbcx_dn9 = (assign5700_e6247 * locals.var_qbcx_dn9);
        locals.var_qbcx_dn10 = (assign5700_e6247 * locals.var_qbcx_dn10);
        locals.var_qbcx_dn11 = (assign5700_e6247 * locals.var_qbcx_dn11);

        let assign5710_e6252: f64 = locals.var_vbictype;
        let assign5710_e6254: f64 = (assign5710_e6252 * locals.var_qbep);
        locals.var_qbep = assign5710_e6254;
        locals.var_qbep_dn4 = (assign5710_e6252 * locals.var_qbep_dn4);
        locals.var_qbep_dn5 = (assign5710_e6252 * locals.var_qbep_dn5);
        locals.var_qbep_dn6 = (assign5710_e6252 * locals.var_qbep_dn6);
        locals.var_qbep_dn7 = (assign5710_e6252 * locals.var_qbep_dn7);
        locals.var_qbep_dn8 = (assign5710_e6252 * locals.var_qbep_dn8);
        locals.var_qbep_dn9 = (assign5710_e6252 * locals.var_qbep_dn9);
        locals.var_qbep_dn10 = (assign5710_e6252 * locals.var_qbep_dn10);
        locals.var_qbep_dn11 = (assign5710_e6252 * locals.var_qbep_dn11);

        let assign5740_e6263: f64 = locals.var_vbictype;
        let assign5740_e6265: f64 = (assign5740_e6263 * locals.var_qbcp);
        locals.var_qbcp = assign5740_e6265;
        locals.var_qbcp_dn4 = (assign5740_e6263 * locals.var_qbcp_dn4);
        locals.var_qbcp_dn10 = (assign5740_e6263 * locals.var_qbcp_dn10);
        locals.var_qbcp_dn11 = (assign5740_e6263 * locals.var_qbcp_dn11);

        let assign5750_e6268: f64 = locals.var_qcth;
        locals.var_qcth = assign5750_e6268;
        locals.var_qcth_dn4 = locals.var_qcth_dn4;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e408: f64 = if ctx.analysis_static() { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e408;
        locals.var_guard1_rv = 0.0;

        let assign100_e469: f64 = if param_given[11] { 1.0 } else { 0.0 };
        locals.var_guard5 = assign100_e469;
        locals.var_guard5_rv = 0.0;

        let (assign110_e475,) = {
    if ((locals.var_guard1 != 0.0) && (locals.var_guard5 != 0.0)) {
        (p.p11,)
    } else {
        (locals.var_imaxmod,)
    }
};
        locals.var_imaxmod = assign110_e475;
        locals.var_imaxmod_rv = 0.0;

        let (assign120_e484,) = {
    if ((locals.var_guard1 != 0.0) && (locals.var_guard5 == 0.0)) {
        let assign120_e482: f64 = 1.0;
        (assign120_e482,)
    } else {
        (locals.var_imaxmod,)
    }
};
        locals.var_imaxmod = assign120_e484;
        locals.var_imaxmod_rv = 0.0;

        let assign130_e486: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard6 = assign130_e486;
        locals.var_guard6_rv = 0.0;

        let (assign140_e493,) = {
    if ((locals.var_guard1 != 0.0) && (locals.var_guard6 != 0.0)) {
        let assign140_e491: f64 = 1.0;
        (assign140_e491,)
    } else {
        (locals.var_vbictype,)
    }
};
        locals.var_vbictype = assign140_e493;
        locals.var_vbictype_rv = 0.0;

        let assign150_e495: f64 = if param_given[4] { 1.0 } else { 0.0 };
        locals.var_guard7 = assign150_e495;
        locals.var_guard7_rv = 0.0;

        let (assign160_e505,) = {
    if (((locals.var_guard1 != 0.0) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 != 0.0)) {
        let assign160_e503: f64 = (-1.0);
        (assign160_e503,)
    } else {
        (locals.var_vbictype,)
    }
};
        locals.var_vbictype = assign160_e505;
        locals.var_vbictype_rv = 0.0;

        let assign170_e507: f64 = if param_given[5] { 1.0 } else { 0.0 };
        locals.var_guard8 = assign170_e507;
        locals.var_guard8_rv = 0.0;

        let (assign180_e519,) = {
    if ((((locals.var_guard1 != 0.0) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 == 0.0)) && (locals.var_guard8 != 0.0)) {
        (p.p5,)
    } else {
        (locals.var_vbictype,)
    }
};
        locals.var_vbictype = assign180_e519;
        locals.var_vbictype_rv = 0.0;

        let (assign190_e533,) = {
    if ((((locals.var_guard1 != 0.0) && (locals.var_guard6 == 0.0)) && (locals.var_guard7 == 0.0)) && (locals.var_guard8 == 0.0)) {
        let assign190_e531: f64 = 1.0;
        (assign190_e531,)
    } else {
        (locals.var_vbictype,)
    }
};
        locals.var_vbictype = assign190_e533;
        locals.var_vbictype_rv = 0.0;

        let (assign200_e538,) = {
    if (locals.var_guard1 != 0.0) {
        let assign200_e536: f64 = (p.p12).ln();
        (assign200_e536,)
    } else {
        (locals.var_vmaxexp,)
    }
};
        locals.var_vmaxexp = assign200_e538;
        locals.var_vmaxexp_rv = 0.0;

        let (assign210_e549,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign210_e547,) = {
            if (p.p74 > 0.0) {
                let assign210_e545: f64 = (1.0 / p.p74);
                (assign210_e545,)
            } else {
                (0.0,)
            }
        };
        (assign210_e547,)
    } else {
        (locals.var_iikr,)
    }
};
        locals.var_iikr = assign210_e549;
        locals.var_iikr_rv = 0.0;

        let (assign220_e560,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign220_e558,) = {
            if (p.p75 > 0.0) {
                let assign220_e556: f64 = (1.0 / p.p75);
                (assign220_e556,)
            } else {
                (0.0,)
            }
        };
        (assign220_e558,)
    } else {
        (locals.var_iikp,)
    }
};
        locals.var_iikp = assign220_e560;
        locals.var_iikp_rv = 0.0;

        let (assign240_e582,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign240_e580,) = {
            if (p.p79 > 0.0) {
                let assign240_e578: f64 = (1.0 / p.p79);
                (assign240_e578,)
            } else {
                (0.0,)
            }
        };
        (assign240_e580,)
    } else {
        (locals.var_ivtf,)
    }
};
        locals.var_ivtf = assign240_e582;
        locals.var_ivtf_rv = 0.0;

        let (assign250_e593,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign250_e591,) = {
            if (p.p80 > 0.0) {
                let assign250_e589: f64 = (1.0 / p.p80);
                (assign250_e589,)
            } else {
                (0.0,)
            }
        };
        (assign250_e591,)
    } else {
        (locals.var_iitf,)
    }
};
        locals.var_iitf = assign250_e593;
        locals.var_iitf_rv = 0.0;

        let (assign260_e602,) = {
    if (locals.var_guard1 != 0.0) {
        let (assign260_e600,) = {
            if (p.p80 > 0.0) {
                (0.0,)
            } else {
                (1.0,)
            }
        };
        (assign260_e600,)
    } else {
        (locals.var_sltf,)
    }
};
        locals.var_sltf = assign260_e602;
        locals.var_sltf_rv = 0.0;

        let (assign270_e608,) = {
    if (locals.var_guard1 != 0.0) {
        let assign270_e606: f64 = (273.15 + p.p13);
        (assign270_e606,)
    } else {
        (locals.var_tinik,)
    }
};
        locals.var_tinik = assign270_e608;
        locals.var_tinik_rv = 0.0;

        let assign290_e610: f64 = ctx_temp;
        let assign290_e612: f64 = (assign290_e610 + p.p0);
        let assign290_e614: f64 = (assign290_e612 - 273.15);
        locals.var_tdevc = assign290_e614;
        locals.var_tdevc_dn4 = 0.0;
        locals.var_tdevc_rv = 0.0;

        let assign320_e624: f64 = (p.p14 + 1.0);
        let assign320_e625: f64 = if locals.var_tdevc < assign320_e624 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign320_e625;
        locals.var_guard11_rv = 0.0;

        let (assign330_e636, assign330_e636_d_n4,) = {
    if (locals.var_guard11 != 0.0) {
        let assign330_e630: f64 = (locals.var_tdevc - p.p14);
        let assign330_e632: f64 = (assign330_e630 - 1.0);
        let assign330_e633: f64 = (assign330_e632).exp();
        let assign330_e634: f64 = (p.p14 + assign330_e633);
        (assign330_e634, (assign330_e633 * locals.var_tdevc_dn4),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign330_e636;
        locals.var_tdevc_dn4 = assign330_e636_d_n4;
        locals.var_tdevc_rv = 0.0;

        let assign340_e640: f64 = (p.p15 - 1.0);
        let assign340_e641: f64 = if locals.var_tdevc > assign340_e640 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign340_e641;
        locals.var_guard12_rv = 0.0;

        let (assign350_e655, assign350_e655_d_n4,) = {
    if ((locals.var_guard11 == 0.0) && (locals.var_guard12 != 0.0)) {
        let assign350_e649: f64 = (p.p15 - locals.var_tdevc);
        let assign350_e651: f64 = (assign350_e649 - 1.0);
        let assign350_e652: f64 = (assign350_e651).exp();
        let assign350_e653: f64 = (p.p15 - assign350_e652);
        (assign350_e653, (-(assign350_e652 * (-locals.var_tdevc_dn4))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign350_e655;
        locals.var_tdevc_dn4 = assign350_e655_d_n4;
        locals.var_tdevc_rv = 0.0;

        let (assign360_e663, assign360_e663_d_n4,) = {
    if ((locals.var_guard11 == 0.0) && (locals.var_guard12 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign360_e663;
        locals.var_tdevc_dn4 = assign360_e663_d_n4;
        locals.var_tdevc_rv = 0.0;

        let assign370_e666: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign370_e666;
        locals.var_tdevk_dn4 = locals.var_tdevc_dn4;
        locals.var_tdevk_rv = 0.0;

        let assign380_e669: f64 = (1.380662e-23 * locals.var_tdevk);
        let assign380_e671: f64 = (assign380_e669 / 1.602189e-19);
        locals.var_vtv = assign380_e671;
        locals.var_vtv_dn4 = ((1.380662e-23 * locals.var_tdevk_dn4) / 1.602189e-19);
        locals.var_vtv_rv = 0.0;

        let assign390_e674: f64 = (locals.var_tdevk / locals.var_tinik);
        locals.var_rt = assign390_e674;
        locals.var_rt_dn4 = (locals.var_tdevk_dn4 / locals.var_tinik);
        locals.var_rt_rv = 0.0;

        let assign410_e687: f64 = if p.p90 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign410_e687;
        locals.var_guard13_rv = 0.0;

        let (assign420_e706, assign420_e706_d_n4,) = {
    if (locals.var_guard13 != 0.0) {
        let assign420_e691: f64 = (p.p89 * locals.var_vtv);
        let assign420_e693: f64 = (-p.p88);
        let assign420_e696: f64 = (p.p89 * locals.var_vtv);
        let assign420_e697: f64 = (assign420_e693 / assign420_e696);
        let assign420_e698: f64 = (assign420_e697).exp();
        let assign420_e701: f64 = (locals.var_imaxmod / p.p90);
        let assign420_e702: f64 = (assign420_e698 + assign420_e701);
        let assign420_e703: f64 = (assign420_e702).ln();
        let assign420_e704: f64 = (assign420_e691 * assign420_e703);
        (assign420_e704, (((p.p89 * locals.var_vtv_dn4) * assign420_e703) + (assign420_e691 * ((assign420_e698 * (-((assign420_e693 * (p.p89 * locals.var_vtv_dn4)) / (assign420_e696 * assign420_e696)))) / assign420_e702))),)
    } else {
        (locals.var_maxvibbe, locals.var_maxvibbe_dn4,)
    }
};
        locals.var_maxvibbe = assign420_e706;
        locals.var_maxvibbe_dn4 = assign420_e706_d_n4;
        locals.var_maxvibbe_rv = 0.0;

        let (assign430_e711, assign430_e711_d_n4,) = {
    if (locals.var_guard13 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibbe, locals.var_maxvibbe_dn4,)
    }
};
        locals.var_maxvibbe = assign430_e711;
        locals.var_maxvibbe_dn4 = assign430_e711_d_n4;
        locals.var_maxvibbe_rv = 0.0;

        let assign440_e716: f64 = (p.p122 / p.p28);
        let assign440_e717: f64 = (locals.var_rt).powf(assign440_e716);
        let assign440_e718: f64 = (p.p26 * assign440_e717);
        let assign440_e720: f64 = (-p.p113);
        let assign440_e723: f64 = (1.0 - locals.var_rt);
        let assign440_e724: f64 = (assign440_e720 * assign440_e723);
        let assign440_e727: f64 = (locals.var_vtv * p.p28);
        let assign440_e728: f64 = (assign440_e724 / assign440_e727);
        let assign440_e729: f64 = (assign440_e728).exp();
        let assign440_e730: f64 = (assign440_e718 * assign440_e729);
        locals.var_is_t = assign440_e730;
        locals.var_is_t_dn4 = (((p.p26 * if 0.0 == 0.0 && ((assign440_e716) as f64).is_finite() && ((assign440_e716) as f64).fract() == 0.0 { if assign440_e716 == 0.0 { 0.0 } else { (assign440_e716 * ((locals.var_rt).powf(assign440_e716 - 1.0) * locals.var_rt_dn4)) } } else { (assign440_e717 * (assign440_e716 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign440_e729) + (assign440_e718 * (assign440_e729 * ((((assign440_e720 * (-locals.var_rt_dn4)) * assign440_e727) - (assign440_e724 * (locals.var_vtv_dn4 * p.p28))) / (assign440_e727 * assign440_e727)))));
        locals.var_is_t_rv = 0.0;

        let assign450_e733: f64 = if locals.var_is_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign450_e733;
        locals.var_guard14_rv = 0.0;

        let assign460_e740: f64 = if ((p.p72 > 0.0) && (locals.var_imaxmod > p.p72)) { 1.0 } else { 0.0 };
        locals.var_guard15 = assign460_e740;
        locals.var_guard15_rv = 0.0;

        let (assign470_e769, assign470_e769_d_n4,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 != 0.0)) {
        let assign470_e746: f64 = (p.p28 * locals.var_vtv);
        let assign470_e750: f64 = (0.5 * locals.var_imaxmod);
        let assign470_e753: f64 = (4.0 / p.p72);
        let assign470_e755: f64 = (assign470_e753).powf(p.p73);
        let assign470_e756: f64 = (assign470_e750 * assign470_e755);
        let assign470_e760: f64 = (1.0 - p.p73);
        let assign470_e761: f64 = (1.0 / assign470_e760);
        let assign470_e762: f64 = (assign470_e756).powf(assign470_e761);
        let assign470_e764: f64 = (assign470_e762 / locals.var_is_t);
        let assign470_e765: f64 = (1.0 + assign470_e764);
        let assign470_e766: f64 = (assign470_e765).ln();
        let assign470_e767: f64 = (assign470_e746 * assign470_e766);
        (assign470_e767, (((p.p28 * locals.var_vtv_dn4) * assign470_e766) + (assign470_e746 * ((-((assign470_e762 * locals.var_is_t_dn4) / (locals.var_is_t * locals.var_is_t))) / assign470_e765))),)
    } else {
        (locals.var_maxvifi, locals.var_maxvifi_dn4,)
    }
};
        locals.var_maxvifi = assign470_e769;
        locals.var_maxvifi_dn4 = assign470_e769_d_n4;
        locals.var_maxvifi_rv = 0.0;

        let (assign480_e785, assign480_e785_d_n4,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard15 == 0.0)) {
        let assign480_e776: f64 = (p.p28 * locals.var_vtv);
        let assign480_e780: f64 = (locals.var_imaxmod / locals.var_is_t);
        let assign480_e781: f64 = (1.0 + assign480_e780);
        let assign480_e782: f64 = (assign480_e781).ln();
        let assign480_e783: f64 = (assign480_e776 * assign480_e782);
        (assign480_e783, (((p.p28 * locals.var_vtv_dn4) * assign480_e782) + (assign480_e776 * ((-((locals.var_imaxmod * locals.var_is_t_dn4) / (locals.var_is_t * locals.var_is_t))) / assign480_e781))),)
    } else {
        (locals.var_maxvifi, locals.var_maxvifi_dn4,)
    }
};
        locals.var_maxvifi = assign480_e785;
        locals.var_maxvifi_dn4 = assign480_e785_d_n4;
        locals.var_maxvifi_rv = 0.0;

        let (assign490_e790, assign490_e790_d_n4,) = {
    if (locals.var_guard14 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvifi, locals.var_maxvifi_dn4,)
    }
};
        locals.var_maxvifi = assign490_e790;
        locals.var_maxvifi_dn4 = assign490_e790_d_n4;
        locals.var_maxvifi_rv = 0.0;

        let assign500_e795: f64 = (p.p125 / p.p29);
        let assign500_e796: f64 = (locals.var_rt).powf(assign500_e795);
        let assign500_e797: f64 = (p.p27 * assign500_e796);
        let assign500_e799: f64 = (-p.p121);
        let assign500_e802: f64 = (1.0 - locals.var_rt);
        let assign500_e803: f64 = (assign500_e799 * assign500_e802);
        let assign500_e806: f64 = (locals.var_vtv * p.p29);
        let assign500_e807: f64 = (assign500_e803 / assign500_e806);
        let assign500_e808: f64 = (assign500_e807).exp();
        let assign500_e809: f64 = (assign500_e797 * assign500_e808);
        locals.var_isrr_t = assign500_e809;
        locals.var_isrr_t_dn4 = (((p.p27 * if 0.0 == 0.0 && ((assign500_e795) as f64).is_finite() && ((assign500_e795) as f64).fract() == 0.0 { if assign500_e795 == 0.0 { 0.0 } else { (assign500_e795 * ((locals.var_rt).powf(assign500_e795 - 1.0) * locals.var_rt_dn4)) } } else { (assign500_e796 * (assign500_e795 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign500_e808) + (assign500_e797 * (assign500_e808 * ((((assign500_e799 * (-locals.var_rt_dn4)) * assign500_e806) - (assign500_e803 * (locals.var_vtv_dn4 * p.p29))) / (assign500_e806 * assign500_e806)))));
        locals.var_isrr_t_rv = 0.0;

        let assign510_e816: f64 = if ((locals.var_is_t > 0.0) && (locals.var_isrr_t > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard16 = assign510_e816;
        locals.var_guard16_rv = 0.0;

        let assign520_e823: f64 = if ((p.p74 > 0.0) && (locals.var_imaxmod > p.p74)) { 1.0 } else { 0.0 };
        locals.var_guard17 = assign520_e823;
        locals.var_guard17_rv = 0.0;

        let (assign530_e854, assign530_e854_d_n4,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 != 0.0)) {
        let assign530_e829: f64 = (p.p29 * locals.var_vtv);
        let assign530_e833: f64 = (0.5 * locals.var_imaxmod);
        let assign530_e836: f64 = (4.0 / p.p74);
        let assign530_e838: f64 = (assign530_e836).powf(p.p73);
        let assign530_e839: f64 = (assign530_e833 * assign530_e838);
        let assign530_e843: f64 = (1.0 - p.p73);
        let assign530_e844: f64 = (1.0 / assign530_e843);
        let assign530_e845: f64 = (assign530_e839).powf(assign530_e844);
        let assign530_e848: f64 = (locals.var_is_t * locals.var_isrr_t);
        let assign530_e849: f64 = (assign530_e845 / assign530_e848);
        let assign530_e850: f64 = (1.0 + assign530_e849);
        let assign530_e851: f64 = (assign530_e850).ln();
        let assign530_e852: f64 = (assign530_e829 * assign530_e851);
        (assign530_e852, (((p.p29 * locals.var_vtv_dn4) * assign530_e851) + (assign530_e829 * ((-((assign530_e845 * ((locals.var_is_t_dn4 * locals.var_isrr_t) + (locals.var_is_t * locals.var_isrr_t_dn4))) / (assign530_e848 * assign530_e848))) / assign530_e850))),)
    } else {
        (locals.var_maxviri, locals.var_maxviri_dn4,)
    }
};
        locals.var_maxviri = assign530_e854;
        locals.var_maxviri_dn4 = assign530_e854_d_n4;
        locals.var_maxviri_rv = 0.0;

        let (assign540_e872, assign540_e872_d_n4,) = {
    if ((locals.var_guard16 != 0.0) && (locals.var_guard17 == 0.0)) {
        let assign540_e861: f64 = (p.p29 * locals.var_vtv);
        let assign540_e866: f64 = (locals.var_is_t * locals.var_isrr_t);
        let assign540_e867: f64 = (locals.var_imaxmod / assign540_e866);
        let assign540_e868: f64 = (1.0 + assign540_e867);
        let assign540_e869: f64 = (assign540_e868).ln();
        let assign540_e870: f64 = (assign540_e861 * assign540_e869);
        (assign540_e870, (((p.p29 * locals.var_vtv_dn4) * assign540_e869) + (assign540_e861 * ((-((locals.var_imaxmod * ((locals.var_is_t_dn4 * locals.var_isrr_t) + (locals.var_is_t * locals.var_isrr_t_dn4))) / (assign540_e866 * assign540_e866))) / assign540_e868))),)
    } else {
        (locals.var_maxviri, locals.var_maxviri_dn4,)
    }
};
        locals.var_maxviri = assign540_e872;
        locals.var_maxviri_dn4 = assign540_e872_d_n4;
        locals.var_maxviri_rv = 0.0;

        let (assign550_e877, assign550_e877_d_n4,) = {
    if (locals.var_guard16 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxviri, locals.var_maxviri_dn4,)
    }
};
        locals.var_maxviri = assign550_e877;
        locals.var_maxviri_dn4 = assign550_e877_d_n4;
        locals.var_maxviri_rv = 0.0;

        let assign560_e882: f64 = (p.p122 / p.p33);
        let assign560_e883: f64 = (locals.var_rt).powf(assign560_e882);
        let assign560_e884: f64 = (p.p31 * assign560_e883);
        let assign560_e886: f64 = (-p.p120);
        let assign560_e889: f64 = (1.0 - locals.var_rt);
        let assign560_e890: f64 = (assign560_e886 * assign560_e889);
        let assign560_e893: f64 = (locals.var_vtv * p.p33);
        let assign560_e894: f64 = (assign560_e890 / assign560_e893);
        let assign560_e895: f64 = (assign560_e894).exp();
        let assign560_e896: f64 = (assign560_e884 * assign560_e895);
        locals.var_isp_t = assign560_e896;
        locals.var_isp_t_dn4 = (((p.p31 * if 0.0 == 0.0 && ((assign560_e882) as f64).is_finite() && ((assign560_e882) as f64).fract() == 0.0 { if assign560_e882 == 0.0 { 0.0 } else { (assign560_e882 * ((locals.var_rt).powf(assign560_e882 - 1.0) * locals.var_rt_dn4)) } } else { (assign560_e883 * (assign560_e882 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign560_e895) + (assign560_e884 * (assign560_e895 * ((((assign560_e886 * (-locals.var_rt_dn4)) * assign560_e893) - (assign560_e890 * (locals.var_vtv_dn4 * p.p33))) / (assign560_e893 * assign560_e893)))));
        locals.var_isp_t_rv = 0.0;

        let assign570_e899: f64 = if locals.var_isp_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign570_e899;
        locals.var_guard18_rv = 0.0;

        let assign580_e906: f64 = if ((p.p75 > 0.0) && (locals.var_imaxmod > p.p75)) { 1.0 } else { 0.0 };
        locals.var_guard19 = assign580_e906;
        locals.var_guard19_rv = 0.0;

        let (assign590_e925, assign590_e925_d_n4,) = {
    if ((locals.var_guard18 != 0.0) && (locals.var_guard19 != 0.0)) {
        let assign590_e912: f64 = (p.p33 * locals.var_vtv);
        let assign590_e916: f64 = (locals.var_imaxmod * locals.var_imaxmod);
        let assign590_e918: f64 = (assign590_e916 * locals.var_iikp);
        let assign590_e920: f64 = (assign590_e918 / locals.var_isp_t);
        let assign590_e921: f64 = (1.0 + assign590_e920);
        let assign590_e922: f64 = (assign590_e921).ln();
        let assign590_e923: f64 = (assign590_e912 * assign590_e922);
        (assign590_e923, (((p.p33 * locals.var_vtv_dn4) * assign590_e922) + (assign590_e912 * ((-((assign590_e918 * locals.var_isp_t_dn4) / (locals.var_isp_t * locals.var_isp_t))) / assign590_e921))),)
    } else {
        (locals.var_maxvip, locals.var_maxvip_dn4,)
    }
};
        locals.var_maxvip = assign590_e925;
        locals.var_maxvip_dn4 = assign590_e925_d_n4;
        locals.var_maxvip_rv = 0.0;

        let (assign600_e941, assign600_e941_d_n4,) = {
    if ((locals.var_guard18 != 0.0) && (locals.var_guard19 == 0.0)) {
        let assign600_e932: f64 = (p.p33 * locals.var_vtv);
        let assign600_e936: f64 = (locals.var_imaxmod / locals.var_isp_t);
        let assign600_e937: f64 = (1.0 + assign600_e936);
        let assign600_e938: f64 = (assign600_e937).ln();
        let assign600_e939: f64 = (assign600_e932 * assign600_e938);
        (assign600_e939, (((p.p33 * locals.var_vtv_dn4) * assign600_e938) + (assign600_e932 * ((-((locals.var_imaxmod * locals.var_isp_t_dn4) / (locals.var_isp_t * locals.var_isp_t))) / assign600_e937))),)
    } else {
        (locals.var_maxvip, locals.var_maxvip_dn4,)
    }
};
        locals.var_maxvip = assign600_e941;
        locals.var_maxvip_dn4 = assign600_e941_d_n4;
        locals.var_maxvip_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign610_e946, assign610_e946_d_n4,) = {
    if (locals.var_guard18 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvip, locals.var_maxvip_dn4,)
    }
};
        locals.var_maxvip = assign610_e946;
        locals.var_maxvip_dn4 = assign610_e946_d_n4;
        locals.var_maxvip_rv = 0.0;

        let assign620_e951: f64 = (p.p123 / p.p56);
        let assign620_e952: f64 = (locals.var_rt).powf(assign620_e951);
        let assign620_e953: f64 = (p.p54 * assign620_e952);
        let assign620_e955: f64 = (-p.p114);
        let assign620_e958: f64 = (1.0 - locals.var_rt);
        let assign620_e959: f64 = (assign620_e955 * assign620_e958);
        let assign620_e962: f64 = (locals.var_vtv * p.p56);
        let assign620_e963: f64 = (assign620_e959 / assign620_e962);
        let assign620_e964: f64 = (assign620_e963).exp();
        let assign620_e965: f64 = (assign620_e953 * assign620_e964);
        locals.var_ibei_t = assign620_e965;
        locals.var_ibei_t_dn4 = (((p.p54 * if 0.0 == 0.0 && ((assign620_e951) as f64).is_finite() && ((assign620_e951) as f64).fract() == 0.0 { if assign620_e951 == 0.0 { 0.0 } else { (assign620_e951 * ((locals.var_rt).powf(assign620_e951 - 1.0) * locals.var_rt_dn4)) } } else { (assign620_e952 * (assign620_e951 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign620_e964) + (assign620_e953 * (assign620_e964 * ((((assign620_e955 * (-locals.var_rt_dn4)) * assign620_e962) - (assign620_e959 * (locals.var_vtv_dn4 * p.p56))) / (assign620_e962 * assign620_e962)))));
        locals.var_ibei_t_rv = 0.0;

        let assign630_e968: f64 = if locals.var_ibei_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign630_e968;
        locals.var_guard20_rv = 0.0;

        let (assign640_e981, assign640_e981_d_n4,) = {
    if (locals.var_guard20 != 0.0) {
        let assign640_e972: f64 = (p.p56 * locals.var_vtv);
        let assign640_e976: f64 = (locals.var_imaxmod / locals.var_ibei_t);
        let assign640_e977: f64 = (1.0 + assign640_e976);
        let assign640_e978: f64 = (assign640_e977).ln();
        let assign640_e979: f64 = (assign640_e972 * assign640_e978);
        (assign640_e979, (((p.p56 * locals.var_vtv_dn4) * assign640_e978) + (assign640_e972 * ((-((locals.var_imaxmod * locals.var_ibei_t_dn4) / (locals.var_ibei_t * locals.var_ibei_t))) / assign640_e977))),)
    } else {
        (locals.var_maxvibei, locals.var_maxvibei_dn4,)
    }
};
        locals.var_maxvibei = assign640_e981;
        locals.var_maxvibei_dn4 = assign640_e981_d_n4;
        locals.var_maxvibei_rv = 0.0;

        let (assign650_e986, assign650_e986_d_n4,) = {
    if (locals.var_guard20 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibei, locals.var_maxvibei_dn4,)
    }
};
        locals.var_maxvibei = assign650_e986;
        locals.var_maxvibei_dn4 = assign650_e986_d_n4;
        locals.var_maxvibei_rv = 0.0;

        let assign700_e1031: f64 = (p.p123 / p.p61);
        let assign700_e1032: f64 = (locals.var_rt).powf(assign700_e1031);
        let assign700_e1033: f64 = (p.p60 * assign700_e1032);
        let assign700_e1035: f64 = (-p.p115);
        let assign700_e1038: f64 = (1.0 - locals.var_rt);
        let assign700_e1039: f64 = (assign700_e1035 * assign700_e1038);
        let assign700_e1042: f64 = (locals.var_vtv * p.p61);
        let assign700_e1043: f64 = (assign700_e1039 / assign700_e1042);
        let assign700_e1044: f64 = (assign700_e1043).exp();
        let assign700_e1045: f64 = (assign700_e1033 * assign700_e1044);
        locals.var_ibci_t = assign700_e1045;
        locals.var_ibci_t_dn4 = (((p.p60 * if 0.0 == 0.0 && ((assign700_e1031) as f64).is_finite() && ((assign700_e1031) as f64).fract() == 0.0 { if assign700_e1031 == 0.0 { 0.0 } else { (assign700_e1031 * ((locals.var_rt).powf(assign700_e1031 - 1.0) * locals.var_rt_dn4)) } } else { (assign700_e1032 * (assign700_e1031 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign700_e1044) + (assign700_e1033 * (assign700_e1044 * ((((assign700_e1035 * (-locals.var_rt_dn4)) * assign700_e1042) - (assign700_e1039 * (locals.var_vtv_dn4 * p.p61))) / (assign700_e1042 * assign700_e1042)))));
        locals.var_ibci_t_rv = 0.0;

        let assign710_e1048: f64 = if locals.var_ibci_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign710_e1048;
        locals.var_guard22_rv = 0.0;

        let (assign720_e1061, assign720_e1061_d_n4,) = {
    if (locals.var_guard22 != 0.0) {
        let assign720_e1052: f64 = (p.p61 * locals.var_vtv);
        let assign720_e1056: f64 = (locals.var_imaxmod / locals.var_ibci_t);
        let assign720_e1057: f64 = (1.0 + assign720_e1056);
        let assign720_e1058: f64 = (assign720_e1057).ln();
        let assign720_e1059: f64 = (assign720_e1052 * assign720_e1058);
        (assign720_e1059, (((p.p61 * locals.var_vtv_dn4) * assign720_e1058) + (assign720_e1052 * ((-((locals.var_imaxmod * locals.var_ibci_t_dn4) / (locals.var_ibci_t * locals.var_ibci_t))) / assign720_e1057))),)
    } else {
        (locals.var_maxvibci, locals.var_maxvibci_dn4,)
    }
};
        locals.var_maxvibci = assign720_e1061;
        locals.var_maxvibci_dn4 = assign720_e1061_d_n4;
        locals.var_maxvibci_rv = 0.0;

        let (assign730_e1066, assign730_e1066_d_n4,) = {
    if (locals.var_guard22 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibci, locals.var_maxvibci_dn4,)
    }
};
        locals.var_maxvibci = assign730_e1066;
        locals.var_maxvibci_dn4 = assign730_e1066_d_n4;
        locals.var_maxvibci_rv = 0.0;

        let assign780_e1111: f64 = (p.p123 / p.p61);
        let assign780_e1112: f64 = (locals.var_rt).powf(assign780_e1111);
        let assign780_e1113: f64 = (p.p64 * assign780_e1112);
        let assign780_e1115: f64 = (-p.p115);
        let assign780_e1118: f64 = (1.0 - locals.var_rt);
        let assign780_e1119: f64 = (assign780_e1115 * assign780_e1118);
        let assign780_e1122: f64 = (locals.var_vtv * p.p61);
        let assign780_e1123: f64 = (assign780_e1119 / assign780_e1122);
        let assign780_e1124: f64 = (assign780_e1123).exp();
        let assign780_e1125: f64 = (assign780_e1113 * assign780_e1124);
        locals.var_ibeip_t = assign780_e1125;
        locals.var_ibeip_t_dn4 = (((p.p64 * if 0.0 == 0.0 && ((assign780_e1111) as f64).is_finite() && ((assign780_e1111) as f64).fract() == 0.0 { if assign780_e1111 == 0.0 { 0.0 } else { (assign780_e1111 * ((locals.var_rt).powf(assign780_e1111 - 1.0) * locals.var_rt_dn4)) } } else { (assign780_e1112 * (assign780_e1111 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign780_e1124) + (assign780_e1113 * (assign780_e1124 * ((((assign780_e1115 * (-locals.var_rt_dn4)) * assign780_e1122) - (assign780_e1119 * (locals.var_vtv_dn4 * p.p61))) / (assign780_e1122 * assign780_e1122)))));
        locals.var_ibeip_t_rv = 0.0;

        let assign790_e1128: f64 = if locals.var_ibeip_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard24 = assign790_e1128;
        locals.var_guard24_rv = 0.0;

        let (assign800_e1141, assign800_e1141_d_n4,) = {
    if (locals.var_guard24 != 0.0) {
        let assign800_e1132: f64 = (p.p61 * locals.var_vtv);
        let assign800_e1136: f64 = (locals.var_imaxmod / locals.var_ibeip_t);
        let assign800_e1137: f64 = (1.0 + assign800_e1136);
        let assign800_e1138: f64 = (assign800_e1137).ln();
        let assign800_e1139: f64 = (assign800_e1132 * assign800_e1138);
        (assign800_e1139, (((p.p61 * locals.var_vtv_dn4) * assign800_e1138) + (assign800_e1132 * ((-((locals.var_imaxmod * locals.var_ibeip_t_dn4) / (locals.var_ibeip_t * locals.var_ibeip_t))) / assign800_e1137))),)
    } else {
        (locals.var_maxvibeip, locals.var_maxvibeip_dn4,)
    }
};
        locals.var_maxvibeip = assign800_e1141;
        locals.var_maxvibeip_dn4 = assign800_e1141_d_n4;
        locals.var_maxvibeip_rv = 0.0;

        let (assign810_e1146, assign810_e1146_d_n4,) = {
    if (locals.var_guard24 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibeip, locals.var_maxvibeip_dn4,)
    }
};
        locals.var_maxvibeip = assign810_e1146;
        locals.var_maxvibeip_dn4 = assign810_e1146_d_n4;
        locals.var_maxvibeip_rv = 0.0;

        let assign860_e1191: f64 = (p.p123 / p.p67);
        let assign860_e1192: f64 = (locals.var_rt).powf(assign860_e1191);
        let assign860_e1193: f64 = (p.p66 * assign860_e1192);
        let assign860_e1195: f64 = (-p.p116);
        let assign860_e1198: f64 = (1.0 - locals.var_rt);
        let assign860_e1199: f64 = (assign860_e1195 * assign860_e1198);
        let assign860_e1202: f64 = (locals.var_vtv * p.p67);
        let assign860_e1203: f64 = (assign860_e1199 / assign860_e1202);
        let assign860_e1204: f64 = (assign860_e1203).exp();
        let assign860_e1205: f64 = (assign860_e1193 * assign860_e1204);
        locals.var_ibcip_t = assign860_e1205;
        locals.var_ibcip_t_dn4 = (((p.p66 * if 0.0 == 0.0 && ((assign860_e1191) as f64).is_finite() && ((assign860_e1191) as f64).fract() == 0.0 { if assign860_e1191 == 0.0 { 0.0 } else { (assign860_e1191 * ((locals.var_rt).powf(assign860_e1191 - 1.0) * locals.var_rt_dn4)) } } else { (assign860_e1192 * (assign860_e1191 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign860_e1204) + (assign860_e1193 * (assign860_e1204 * ((((assign860_e1195 * (-locals.var_rt_dn4)) * assign860_e1202) - (assign860_e1199 * (locals.var_vtv_dn4 * p.p67))) / (assign860_e1202 * assign860_e1202)))));
        locals.var_ibcip_t_rv = 0.0;

        let assign870_e1208: f64 = if locals.var_ibcip_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign870_e1208;
        locals.var_guard26_rv = 0.0;

        let (assign880_e1221, assign880_e1221_d_n4,) = {
    if (locals.var_guard26 != 0.0) {
        let assign880_e1212: f64 = (p.p67 * locals.var_vtv);
        let assign880_e1216: f64 = (locals.var_imaxmod / locals.var_ibcip_t);
        let assign880_e1217: f64 = (1.0 + assign880_e1216);
        let assign880_e1218: f64 = (assign880_e1217).ln();
        let assign880_e1219: f64 = (assign880_e1212 * assign880_e1218);
        (assign880_e1219, (((p.p67 * locals.var_vtv_dn4) * assign880_e1218) + (assign880_e1212 * ((-((locals.var_imaxmod * locals.var_ibcip_t_dn4) / (locals.var_ibcip_t * locals.var_ibcip_t))) / assign880_e1217))),)
    } else {
        (locals.var_maxvibcip, locals.var_maxvibcip_dn4,)
    }
};
        locals.var_maxvibcip = assign880_e1221;
        locals.var_maxvibcip_dn4 = assign880_e1221_d_n4;
        locals.var_maxvibcip_rv = 0.0;

        let (assign890_e1226, assign890_e1226_d_n4,) = {
    if (locals.var_guard26 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_maxvibcip, locals.var_maxvibcip_dn4,)
    }
};
        locals.var_maxvibcip = assign890_e1226;
        locals.var_maxvibcip_dn4 = assign890_e1226_d_n4;
        locals.var_maxvibcip_rv = 0.0;

        locals.var_dt_et = (nv4 - 0.0);
        locals.var_dt_et_dn4 = 1.0;
        locals.var_dt_et_rv = 0.0;

        let assign950_e1268: f64 = ctx_temp;
        let assign950_e1270: f64 = (assign950_e1268 + p.p0);
        let assign950_e1272: f64 = (assign950_e1270 + locals.var_dt_et);
        let assign950_e1274: f64 = (assign950_e1272 - 273.15);
        locals.var_tdevc = assign950_e1274;
        locals.var_tdevc_dn4 = locals.var_dt_et_dn4;
        locals.var_tdevc_rv = 0.0;

        let assign960_e1278: f64 = (p.p14 + 1.0);
        let assign960_e1279: f64 = if locals.var_tdevc < assign960_e1278 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign960_e1279;
        locals.var_guard28_rv = 0.0;

        let (assign970_e1290, assign970_e1290_d_n4,) = {
    if (locals.var_guard28 != 0.0) {
        let assign970_e1284: f64 = (locals.var_tdevc - p.p14);
        let assign970_e1286: f64 = (assign970_e1284 - 1.0);
        let assign970_e1287: f64 = (assign970_e1286).exp();
        let assign970_e1288: f64 = (p.p14 + assign970_e1287);
        (assign970_e1288, (assign970_e1287 * locals.var_tdevc_dn4),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign970_e1290;
        locals.var_tdevc_dn4 = assign970_e1290_d_n4;
        locals.var_tdevc_rv = 0.0;

        let assign980_e1294: f64 = (p.p15 - 1.0);
        let assign980_e1295: f64 = if locals.var_tdevc > assign980_e1294 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign980_e1295;
        locals.var_guard29_rv = 0.0;

        let (assign990_e1309, assign990_e1309_d_n4,) = {
    if ((locals.var_guard28 == 0.0) && (locals.var_guard29 != 0.0)) {
        let assign990_e1303: f64 = (p.p15 - locals.var_tdevc);
        let assign990_e1305: f64 = (assign990_e1303 - 1.0);
        let assign990_e1306: f64 = (assign990_e1305).exp();
        let assign990_e1307: f64 = (p.p15 - assign990_e1306);
        (assign990_e1307, (-(assign990_e1306 * (-locals.var_tdevc_dn4))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign990_e1309;
        locals.var_tdevc_dn4 = assign990_e1309_d_n4;
        locals.var_tdevc_rv = 0.0;

        let (assign1000_e1317, assign1000_e1317_d_n4,) = {
    if ((locals.var_guard28 == 0.0) && (locals.var_guard29 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn4,)
    }
};
        locals.var_tdevc = assign1000_e1317;
        locals.var_tdevc_dn4 = assign1000_e1317_d_n4;
        locals.var_tdevc_rv = 0.0;

        let assign1010_e1320: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign1010_e1320;
        locals.var_tdevk_dn4 = locals.var_tdevc_dn4;
        locals.var_tdevk_rv = 0.0;

        let assign1020_e1323: f64 = (1.380662e-23 * locals.var_tdevk);
        let assign1020_e1325: f64 = (assign1020_e1323 / 1.602189e-19);
        locals.var_vtv = assign1020_e1325;
        locals.var_vtv_dn4 = ((1.380662e-23 * locals.var_tdevk_dn4) / 1.602189e-19);
        locals.var_vtv_rv = 0.0;

        let assign1030_e1328: f64 = (locals.var_tdevk / locals.var_tinik);
        locals.var_rt = assign1030_e1328;
        locals.var_rt_dn4 = (locals.var_tdevk_dn4 / locals.var_tinik);
        locals.var_rt_rv = 0.0;

        let assign1040_e1331: f64 = (locals.var_tdevk - locals.var_tinik);
        locals.var_dt = assign1040_e1331;
        locals.var_dt_dn4 = locals.var_tdevk_dn4;
        locals.var_dt_rv = 0.0;

        let assign1050_e1335: f64 = (locals.var_rt).powf(p.p126);
        let assign1050_e1336: f64 = (p.p72 * assign1050_e1335);
        locals.var_ikf_t = assign1050_e1336;
        locals.var_ikf_t_dn4 = (p.p72 * if 0.0 == 0.0 && ((p.p126) as f64).is_finite() && ((p.p126) as f64).fract() == 0.0 { if p.p126 == 0.0 { 0.0 } else { (p.p126 * ((locals.var_rt).powf(p.p126 - 1.0) * locals.var_rt_dn4)) } } else { (assign1050_e1335 * (p.p126 * (locals.var_rt_dn4 / locals.var_rt))) });
        locals.var_ikf_t_rv = 0.0;

        let assign1240_e1453: f64 = (p.p122 / p.p28);
        let assign1240_e1454: f64 = (locals.var_rt).powf(assign1240_e1453);
        let assign1240_e1455: f64 = (p.p26 * assign1240_e1454);
        let assign1240_e1457: f64 = (-p.p113);
        let assign1240_e1460: f64 = (1.0 - locals.var_rt);
        let assign1240_e1461: f64 = (assign1240_e1457 * assign1240_e1460);
        let assign1240_e1464: f64 = (locals.var_vtv * p.p28);
        let assign1240_e1465: f64 = (assign1240_e1461 / assign1240_e1464);
        let assign1240_e1466: f64 = (assign1240_e1465).exp();
        let assign1240_e1467: f64 = (assign1240_e1455 * assign1240_e1466);
        locals.var_is_t = assign1240_e1467;
        locals.var_is_t_dn4 = (((p.p26 * if 0.0 == 0.0 && ((assign1240_e1453) as f64).is_finite() && ((assign1240_e1453) as f64).fract() == 0.0 { if assign1240_e1453 == 0.0 { 0.0 } else { (assign1240_e1453 * ((locals.var_rt).powf(assign1240_e1453 - 1.0) * locals.var_rt_dn4)) } } else { (assign1240_e1454 * (assign1240_e1453 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1240_e1466) + (assign1240_e1455 * (assign1240_e1466 * ((((assign1240_e1457 * (-locals.var_rt_dn4)) * assign1240_e1464) - (assign1240_e1461 * (locals.var_vtv_dn4 * p.p28))) / (assign1240_e1464 * assign1240_e1464)))));
        locals.var_is_t_rv = 0.0;

        let assign1250_e1472: f64 = (p.p125 / p.p29);
        let assign1250_e1473: f64 = (locals.var_rt).powf(assign1250_e1472);
        let assign1250_e1474: f64 = (p.p27 * assign1250_e1473);
        let assign1250_e1476: f64 = (-p.p121);
        let assign1250_e1479: f64 = (1.0 - locals.var_rt);
        let assign1250_e1480: f64 = (assign1250_e1476 * assign1250_e1479);
        let assign1250_e1483: f64 = (locals.var_vtv * p.p29);
        let assign1250_e1484: f64 = (assign1250_e1480 / assign1250_e1483);
        let assign1250_e1485: f64 = (assign1250_e1484).exp();
        let assign1250_e1486: f64 = (assign1250_e1474 * assign1250_e1485);
        locals.var_isrr_t = assign1250_e1486;
        locals.var_isrr_t_dn4 = (((p.p27 * if 0.0 == 0.0 && ((assign1250_e1472) as f64).is_finite() && ((assign1250_e1472) as f64).fract() == 0.0 { if assign1250_e1472 == 0.0 { 0.0 } else { (assign1250_e1472 * ((locals.var_rt).powf(assign1250_e1472 - 1.0) * locals.var_rt_dn4)) } } else { (assign1250_e1473 * (assign1250_e1472 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1250_e1485) + (assign1250_e1474 * (assign1250_e1485 * ((((assign1250_e1476 * (-locals.var_rt_dn4)) * assign1250_e1483) - (assign1250_e1480 * (locals.var_vtv_dn4 * p.p29))) / (assign1250_e1483 * assign1250_e1483)))));
        locals.var_isrr_t_rv = 0.0;

        let assign1260_e1491: f64 = (p.p122 / p.p33);
        let assign1260_e1492: f64 = (locals.var_rt).powf(assign1260_e1491);
        let assign1260_e1493: f64 = (p.p31 * assign1260_e1492);
        let assign1260_e1495: f64 = (-p.p120);
        let assign1260_e1498: f64 = (1.0 - locals.var_rt);
        let assign1260_e1499: f64 = (assign1260_e1495 * assign1260_e1498);
        let assign1260_e1502: f64 = (locals.var_vtv * p.p33);
        let assign1260_e1503: f64 = (assign1260_e1499 / assign1260_e1502);
        let assign1260_e1504: f64 = (assign1260_e1503).exp();
        let assign1260_e1505: f64 = (assign1260_e1493 * assign1260_e1504);
        locals.var_isp_t = assign1260_e1505;
        locals.var_isp_t_dn4 = (((p.p31 * if 0.0 == 0.0 && ((assign1260_e1491) as f64).is_finite() && ((assign1260_e1491) as f64).fract() == 0.0 { if assign1260_e1491 == 0.0 { 0.0 } else { (assign1260_e1491 * ((locals.var_rt).powf(assign1260_e1491 - 1.0) * locals.var_rt_dn4)) } } else { (assign1260_e1492 * (assign1260_e1491 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1260_e1504) + (assign1260_e1493 * (assign1260_e1504 * ((((assign1260_e1495 * (-locals.var_rt_dn4)) * assign1260_e1502) - (assign1260_e1499 * (locals.var_vtv_dn4 * p.p33))) / (assign1260_e1502 * assign1260_e1502)))));
        locals.var_isp_t_rv = 0.0;

        let assign1270_e1510: f64 = (p.p123 / p.p56);
        let assign1270_e1511: f64 = (locals.var_rt).powf(assign1270_e1510);
        let assign1270_e1512: f64 = (p.p54 * assign1270_e1511);
        let assign1270_e1514: f64 = (-p.p114);
        let assign1270_e1517: f64 = (1.0 - locals.var_rt);
        let assign1270_e1518: f64 = (assign1270_e1514 * assign1270_e1517);
        let assign1270_e1521: f64 = (locals.var_vtv * p.p56);
        let assign1270_e1522: f64 = (assign1270_e1518 / assign1270_e1521);
        let assign1270_e1523: f64 = (assign1270_e1522).exp();
        let assign1270_e1524: f64 = (assign1270_e1512 * assign1270_e1523);
        locals.var_ibei_t = assign1270_e1524;
        locals.var_ibei_t_dn4 = (((p.p54 * if 0.0 == 0.0 && ((assign1270_e1510) as f64).is_finite() && ((assign1270_e1510) as f64).fract() == 0.0 { if assign1270_e1510 == 0.0 { 0.0 } else { (assign1270_e1510 * ((locals.var_rt).powf(assign1270_e1510 - 1.0) * locals.var_rt_dn4)) } } else { (assign1270_e1511 * (assign1270_e1510 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1270_e1523) + (assign1270_e1512 * (assign1270_e1523 * ((((assign1270_e1514 * (-locals.var_rt_dn4)) * assign1270_e1521) - (assign1270_e1518 * (locals.var_vtv_dn4 * p.p56))) / (assign1270_e1521 * assign1270_e1521)))));
        locals.var_ibei_t_rv = 0.0;

        let assign1290_e1548: f64 = (p.p123 / p.p61);
        let assign1290_e1549: f64 = (locals.var_rt).powf(assign1290_e1548);
        let assign1290_e1550: f64 = (p.p60 * assign1290_e1549);
        let assign1290_e1552: f64 = (-p.p115);
        let assign1290_e1555: f64 = (1.0 - locals.var_rt);
        let assign1290_e1556: f64 = (assign1290_e1552 * assign1290_e1555);
        let assign1290_e1559: f64 = (locals.var_vtv * p.p61);
        let assign1290_e1560: f64 = (assign1290_e1556 / assign1290_e1559);
        let assign1290_e1561: f64 = (assign1290_e1560).exp();
        let assign1290_e1562: f64 = (assign1290_e1550 * assign1290_e1561);
        locals.var_ibci_t = assign1290_e1562;
        locals.var_ibci_t_dn4 = (((p.p60 * if 0.0 == 0.0 && ((assign1290_e1548) as f64).is_finite() && ((assign1290_e1548) as f64).fract() == 0.0 { if assign1290_e1548 == 0.0 { 0.0 } else { (assign1290_e1548 * ((locals.var_rt).powf(assign1290_e1548 - 1.0) * locals.var_rt_dn4)) } } else { (assign1290_e1549 * (assign1290_e1548 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1290_e1561) + (assign1290_e1550 * (assign1290_e1561 * ((((assign1290_e1552 * (-locals.var_rt_dn4)) * assign1290_e1559) - (assign1290_e1556 * (locals.var_vtv_dn4 * p.p61))) / (assign1290_e1559 * assign1290_e1559)))));
        locals.var_ibci_t_rv = 0.0;

        let assign1310_e1586: f64 = (p.p123 / p.p61);
        let assign1310_e1587: f64 = (locals.var_rt).powf(assign1310_e1586);
        let assign1310_e1588: f64 = (p.p64 * assign1310_e1587);
        let assign1310_e1590: f64 = (-p.p115);
        let assign1310_e1593: f64 = (1.0 - locals.var_rt);
        let assign1310_e1594: f64 = (assign1310_e1590 * assign1310_e1593);
        let assign1310_e1597: f64 = (locals.var_vtv * p.p61);
        let assign1310_e1598: f64 = (assign1310_e1594 / assign1310_e1597);
        let assign1310_e1599: f64 = (assign1310_e1598).exp();
        let assign1310_e1600: f64 = (assign1310_e1588 * assign1310_e1599);
        locals.var_ibeip_t = assign1310_e1600;
        locals.var_ibeip_t_dn4 = (((p.p64 * if 0.0 == 0.0 && ((assign1310_e1586) as f64).is_finite() && ((assign1310_e1586) as f64).fract() == 0.0 { if assign1310_e1586 == 0.0 { 0.0 } else { (assign1310_e1586 * ((locals.var_rt).powf(assign1310_e1586 - 1.0) * locals.var_rt_dn4)) } } else { (assign1310_e1587 * (assign1310_e1586 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1310_e1599) + (assign1310_e1588 * (assign1310_e1599 * ((((assign1310_e1590 * (-locals.var_rt_dn4)) * assign1310_e1597) - (assign1310_e1594 * (locals.var_vtv_dn4 * p.p61))) / (assign1310_e1597 * assign1310_e1597)))));
        locals.var_ibeip_t_rv = 0.0;

        let assign1330_e1624: f64 = (p.p123 / p.p67);
        let assign1330_e1625: f64 = (locals.var_rt).powf(assign1330_e1624);
        let assign1330_e1626: f64 = (p.p66 * assign1330_e1625);
        let assign1330_e1628: f64 = (-p.p116);
        let assign1330_e1631: f64 = (1.0 - locals.var_rt);
        let assign1330_e1632: f64 = (assign1330_e1628 * assign1330_e1631);
        let assign1330_e1635: f64 = (locals.var_vtv * p.p67);
        let assign1330_e1636: f64 = (assign1330_e1632 / assign1330_e1635);
        let assign1330_e1637: f64 = (assign1330_e1636).exp();
        let assign1330_e1638: f64 = (assign1330_e1626 * assign1330_e1637);
        locals.var_ibcip_t = assign1330_e1638;
        locals.var_ibcip_t_dn4 = (((p.p66 * if 0.0 == 0.0 && ((assign1330_e1624) as f64).is_finite() && ((assign1330_e1624) as f64).fract() == 0.0 { if assign1330_e1624 == 0.0 { 0.0 } else { (assign1330_e1624 * ((locals.var_rt).powf(assign1330_e1624 - 1.0) * locals.var_rt_dn4)) } } else { (assign1330_e1625 * (assign1330_e1624 * (locals.var_rt_dn4 / locals.var_rt))) }) * assign1330_e1637) + (assign1330_e1626 * (assign1330_e1637 * ((((assign1330_e1628 * (-locals.var_rt_dn4)) * assign1330_e1635) - (assign1330_e1632 * (locals.var_vtv_dn4 * p.p67))) / (assign1330_e1635 * assign1330_e1635)))));
        locals.var_ibcip_t_rv = 0.0;

        let assign1350_e1662: f64 = (locals.var_dt * p.p129);
        let assign1350_e1663: f64 = (1.0 + assign1350_e1662);
        let assign1350_e1664: f64 = (p.p28 * assign1350_e1663);
        locals.var_nf_t = assign1350_e1664;
        locals.var_nf_t_dn4 = (p.p28 * (locals.var_dt_dn4 * p.p129));
        locals.var_nf_t_rv = 0.0;

        let assign1360_e1669: f64 = (locals.var_dt * p.p129);
        let assign1360_e1670: f64 = (1.0 + assign1360_e1669);
        let assign1360_e1671: f64 = (p.p29 * assign1360_e1670);
        locals.var_nr_t = assign1360_e1671;
        locals.var_nr_t_dn4 = (p.p29 * (locals.var_dt_dn4 * p.p129));
        locals.var_nr_t_rv = 0.0;

        let assign1390_e1692: f64 = (locals.var_dt * p.p92);
        let assign1390_e1693: f64 = (p.p91 + assign1390_e1692);
        let assign1390_e1694: f64 = (locals.var_dt * assign1390_e1693);
        let assign1390_e1695: f64 = (1.0 + assign1390_e1694);
        let assign1390_e1696: f64 = (p.p88 * assign1390_e1695);
        locals.var_vbbe_t = assign1390_e1696;
        locals.var_vbbe_t_dn4 = (p.p88 * ((locals.var_dt_dn4 * assign1390_e1693) + (locals.var_dt * (locals.var_dt_dn4 * p.p92))));
        locals.var_vbbe_t_rv = 0.0;

        let assign1400_e1701: f64 = (locals.var_dt * p.p93);
        let assign1400_e1702: f64 = (1.0 + assign1400_e1701);
        let assign1400_e1703: f64 = (p.p89 * assign1400_e1702);
        locals.var_nbbe_t = assign1400_e1703;
        locals.var_nbbe_t_dn4 = (p.p89 * (locals.var_dt_dn4 * p.p93));
        locals.var_nbbe_t_rv = 0.0;

        let assign1410_e1707: f64 = (locals.var_vtv / locals.var_rt);
        let assign1410_e1708: f64 = (2.0 * assign1410_e1707);
        let assign1410_e1711: f64 = (0.5 * p.p37);
        let assign1410_e1713: f64 = (assign1410_e1711 * locals.var_rt);
        let assign1410_e1715: f64 = (assign1410_e1713 / locals.var_vtv);
        let assign1410_e1716: f64 = (assign1410_e1715).exp();
        let assign1410_e1718: f64 = (-0.5);
        let assign1410_e1720: f64 = (assign1410_e1718 * p.p37);
        let assign1410_e1722: f64 = (assign1410_e1720 * locals.var_rt);
        let assign1410_e1724: f64 = (assign1410_e1722 / locals.var_vtv);
        let assign1410_e1725: f64 = (assign1410_e1724).exp();
        let assign1410_e1726: f64 = (assign1410_e1716 - assign1410_e1725);
        let assign1410_e1727: f64 = (assign1410_e1726).ln();
        let assign1410_e1728: f64 = (assign1410_e1708 * assign1410_e1727);
        locals.var_psiio = assign1410_e1728;
        locals.var_psiio_dn4 = (((2.0 * (((locals.var_vtv_dn4 * locals.var_rt) - (locals.var_vtv * locals.var_rt_dn4)) / (locals.var_rt * locals.var_rt))) * assign1410_e1727) + (assign1410_e1708 * (((assign1410_e1716 * ((((assign1410_e1711 * locals.var_rt_dn4) * locals.var_vtv) - (assign1410_e1713 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))) - (assign1410_e1725 * ((((assign1410_e1720 * locals.var_rt_dn4) * locals.var_vtv) - (assign1410_e1722 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / assign1410_e1726)));
        locals.var_psiio_rv = 0.0;

        let assign1420_e1731: f64 = (locals.var_psiio * locals.var_rt);
        let assign1420_e1734: f64 = (3.0 * locals.var_vtv);
        let assign1420_e1736: f64 = (locals.var_rt).ln();
        let assign1420_e1737: f64 = (assign1420_e1734 * assign1420_e1736);
        let assign1420_e1738: f64 = (assign1420_e1731 - assign1420_e1737);
        let assign1420_e1742: f64 = (locals.var_rt - 1.0);
        let assign1420_e1743: f64 = (p.p114 * assign1420_e1742);
        let assign1420_e1744: f64 = (assign1420_e1738 - assign1420_e1743);
        locals.var_psiin = assign1420_e1744;
        locals.var_psiin_dn4 = ((((locals.var_psiio_dn4 * locals.var_rt) + (locals.var_psiio * locals.var_rt_dn4)) - (((3.0 * locals.var_vtv_dn4) * assign1420_e1736) + (assign1420_e1734 * (locals.var_rt_dn4 / locals.var_rt)))) - (p.p114 * locals.var_rt_dn4));
        locals.var_psiin_rv = 0.0;

        let assign1430_e1748: f64 = (2.0 * locals.var_vtv);
        let assign1430_e1754: f64 = (-locals.var_psiin);
        let assign1430_e1756: f64 = (assign1430_e1754 / locals.var_vtv);
        let assign1430_e1757: f64 = (assign1430_e1756).exp();
        let assign1430_e1758: f64 = (4.0 * assign1430_e1757);
        let assign1430_e1759: f64 = (1.0 + assign1430_e1758);
        let assign1430_e1760: f64 = (assign1430_e1759).sqrt();
        let assign1430_e1761: f64 = (1.0 + assign1430_e1760);
        let assign1430_e1762: f64 = (0.5 * assign1430_e1761);
        let assign1430_e1763: f64 = (assign1430_e1762).ln();
        let assign1430_e1764: f64 = (assign1430_e1748 * assign1430_e1763);
        let assign1430_e1765: f64 = (locals.var_psiin + assign1430_e1764);
        locals.var_pe_t = assign1430_e1765;
        locals.var_pe_t_dn4 = (locals.var_psiin_dn4 + (((2.0 * locals.var_vtv_dn4) * assign1430_e1763) + (assign1430_e1748 * ((0.5 * ((4.0 * (assign1430_e1757 * ((((-locals.var_psiin_dn4) * locals.var_vtv) - (assign1430_e1754 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / (2.0 * assign1430_e1760))) / assign1430_e1762))));
        locals.var_pe_t_rv = 0.0;

        let assign1440_e1769: f64 = (locals.var_vtv / locals.var_rt);
        let assign1440_e1770: f64 = (2.0 * assign1440_e1769);
        let assign1440_e1773: f64 = (0.5 * p.p42);
        let assign1440_e1775: f64 = (assign1440_e1773 * locals.var_rt);
        let assign1440_e1777: f64 = (assign1440_e1775 / locals.var_vtv);
        let assign1440_e1778: f64 = (assign1440_e1777).exp();
        let assign1440_e1780: f64 = (-0.5);
        let assign1440_e1782: f64 = (assign1440_e1780 * p.p42);
        let assign1440_e1784: f64 = (assign1440_e1782 * locals.var_rt);
        let assign1440_e1786: f64 = (assign1440_e1784 / locals.var_vtv);
        let assign1440_e1787: f64 = (assign1440_e1786).exp();
        let assign1440_e1788: f64 = (assign1440_e1778 - assign1440_e1787);
        let assign1440_e1789: f64 = (assign1440_e1788).ln();
        let assign1440_e1790: f64 = (assign1440_e1770 * assign1440_e1789);
        locals.var_psiio__blk37 = assign1440_e1790;
        locals.var_psiio__blk37_dn4 = (((2.0 * (((locals.var_vtv_dn4 * locals.var_rt) - (locals.var_vtv * locals.var_rt_dn4)) / (locals.var_rt * locals.var_rt))) * assign1440_e1789) + (assign1440_e1770 * (((assign1440_e1778 * ((((assign1440_e1773 * locals.var_rt_dn4) * locals.var_vtv) - (assign1440_e1775 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv))) - (assign1440_e1787 * ((((assign1440_e1782 * locals.var_rt_dn4) * locals.var_vtv) - (assign1440_e1784 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / assign1440_e1788)));
        locals.var_psiio__blk37_rv = 0.0;

        let assign1450_e1793: f64 = (locals.var_psiio__blk37 * locals.var_rt);
        let assign1450_e1796: f64 = (3.0 * locals.var_vtv);
        let assign1450_e1798: f64 = (locals.var_rt).ln();
        let assign1450_e1799: f64 = (assign1450_e1796 * assign1450_e1798);
        let assign1450_e1800: f64 = (assign1450_e1793 - assign1450_e1799);
        let assign1450_e1804: f64 = (locals.var_rt - 1.0);
        let assign1450_e1805: f64 = (p.p115 * assign1450_e1804);
        let assign1450_e1806: f64 = (assign1450_e1800 - assign1450_e1805);
        locals.var_psiin__blk38 = assign1450_e1806;
        locals.var_psiin__blk38_dn4 = ((((locals.var_psiio__blk37_dn4 * locals.var_rt) + (locals.var_psiio__blk37 * locals.var_rt_dn4)) - (((3.0 * locals.var_vtv_dn4) * assign1450_e1798) + (assign1450_e1796 * (locals.var_rt_dn4 / locals.var_rt)))) - (p.p115 * locals.var_rt_dn4));
        locals.var_psiin__blk38_rv = 0.0;

        let assign1460_e1810: f64 = (2.0 * locals.var_vtv);
        let assign1460_e1816: f64 = (-locals.var_psiin__blk38);
        let assign1460_e1818: f64 = (assign1460_e1816 / locals.var_vtv);
        let assign1460_e1819: f64 = (assign1460_e1818).exp();
        let assign1460_e1820: f64 = (4.0 * assign1460_e1819);
        let assign1460_e1821: f64 = (1.0 + assign1460_e1820);
        let assign1460_e1822: f64 = (assign1460_e1821).sqrt();
        let assign1460_e1823: f64 = (1.0 + assign1460_e1822);
        let assign1460_e1824: f64 = (0.5 * assign1460_e1823);
        let assign1460_e1825: f64 = (assign1460_e1824).ln();
        let assign1460_e1826: f64 = (assign1460_e1810 * assign1460_e1825);
        let assign1460_e1827: f64 = (locals.var_psiin__blk38 + assign1460_e1826);
        locals.var_pc_t = assign1460_e1827;
        locals.var_pc_t_dn4 = (locals.var_psiin__blk38_dn4 + (((2.0 * locals.var_vtv_dn4) * assign1460_e1825) + (assign1460_e1810 * ((0.5 * ((4.0 * (assign1460_e1819 * ((((-locals.var_psiin__blk38_dn4) * locals.var_vtv) - (assign1460_e1816 * locals.var_vtv_dn4)) / (locals.var_vtv * locals.var_vtv)))) / (2.0 * assign1460_e1822))) / assign1460_e1824))));
        locals.var_pc_t_rv = 0.0;

    }
}
