#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign00_e727: f64 = (273.15 + p.p15);
        locals.var_tkr = assign00_e727;

        let assign10_e728: f64 = ctx_temp;
        let assign10_e730: f64 = (assign10_e728 + p.p36);
        let assign10_e732: f64 = (assign10_e730).min(1000.0);
        locals.var_temp = assign10_e732;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;

        let assign20_e735: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign20_e735;

        let (assign30_e766, assign30_e766_d_n4, assign30_e766_d_n6, assign30_e766_d_n7, assign30_e766_d_n8, assign30_e766_d_n9,) = {
    if (locals.var_guard1 != 0.0) {
        let assign30_e742: f64 = (p.p18 * locals.var_temp);
        let assign30_e743: f64 = (p.p17 + assign30_e742);
        let assign30_e744: f64 = (locals.var_temp + assign30_e743);
        let assign30_e749: f64 = (p.p18 * locals.var_temp);
        let assign30_e750: f64 = (p.p17 + assign30_e749);
        let assign30_e751: f64 = (locals.var_temp - assign30_e750);
        let assign30_e756: f64 = (p.p18 * locals.var_temp);
        let assign30_e757: f64 = (p.p17 + assign30_e756);
        let assign30_e758: f64 = (locals.var_temp - assign30_e757);
        let assign30_e759: f64 = (assign30_e751 * assign30_e758);
        let assign30_e761: f64 = (assign30_e759 + p.p19);
        let assign30_e762: f64 = (assign30_e761).sqrt();
        let assign30_e763: f64 = (assign30_e744 + assign30_e762);
        let assign30_e764: f64 = (0.5 * assign30_e763);
        (assign30_e764, (0.5 * ((locals.var_temp_dn4 + (p.p18 * locals.var_temp_dn4)) + ((((locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn6 + (p.p18 * locals.var_temp_dn6)) + ((((locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn7 + (p.p18 * locals.var_temp_dn7)) + ((((locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn8 + (p.p18 * locals.var_temp_dn8)) + ((((locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn9 + (p.p18 * locals.var_temp_dn9)) + ((((locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)))) / (2.0 * assign30_e762)))),)
    } else {
        (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    }
};
        locals.var_tkd = assign30_e766;
        locals.var_tkd_dn4 = assign30_e766_d_n4;
        locals.var_tkd_dn6 = assign30_e766_d_n6;
        locals.var_tkd_dn7 = assign30_e766_d_n7;
        locals.var_tkd_dn8 = assign30_e766_d_n8;
        locals.var_tkd_dn9 = assign30_e766_d_n9;

        let (assign40_e797, assign40_e797_d_n4, assign40_e797_d_n6, assign40_e797_d_n7, assign40_e797_d_n8, assign40_e797_d_n9,) = {
    if (locals.var_guard1 != 0.0) {
        let assign40_e772: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e773: f64 = (10.0 / assign40_e772);
        let assign40_e775: f64 = (assign40_e773 + 600.0);
        let assign40_e779: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e780: f64 = (10.0 / assign40_e779);
        let assign40_e782: f64 = (assign40_e780 - 600.0);
        let assign40_e786: f64 = (locals.var_tkd * 8.617332384961e-5);
        let assign40_e787: f64 = (10.0 / assign40_e786);
        let assign40_e789: f64 = (assign40_e787 - 600.0);
        let assign40_e790: f64 = (assign40_e782 * assign40_e789);
        let assign40_e792: f64 = (assign40_e790 + 0.01);
        let assign40_e793: f64 = (assign40_e792).sqrt();
        let assign40_e794: f64 = (assign40_e775 + assign40_e793);
        let assign40_e795: f64 = (0.5 * assign40_e794);
        (assign40_e795, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))),)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign40_e797;
        locals.var_xsatmax_dn4 = assign40_e797_d_n4;
        locals.var_xsatmax_dn6 = assign40_e797_d_n6;
        locals.var_xsatmax_dn7 = assign40_e797_d_n7;
        locals.var_xsatmax_dn8 = assign40_e797_d_n8;
        locals.var_xsatmax_dn9 = assign40_e797_d_n9;

        let (assign50_e817, assign50_e817_d_n4, assign50_e817_d_n6, assign50_e817_d_n7, assign50_e817_d_n8, assign50_e817_d_n9,) = {
    if (locals.var_guard1 == 0.0) {
        let assign50_e803: f64 = (locals.var_temp + 1.0);
        let assign50_e806: f64 = (locals.var_temp - 1.0);
        let assign50_e809: f64 = (locals.var_temp - 1.0);
        let assign50_e810: f64 = (assign50_e806 * assign50_e809);
        let assign50_e812: f64 = (assign50_e810 + 0.001);
        let assign50_e813: f64 = (assign50_e812).sqrt();
        let assign50_e814: f64 = (assign50_e803 + assign50_e813);
        let assign50_e815: f64 = (0.5 * assign50_e814);
        (assign50_e815, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign50_e809) + (assign50_e806 * locals.var_temp_dn4)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign50_e809) + (assign50_e806 * locals.var_temp_dn6)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign50_e809) + (assign50_e806 * locals.var_temp_dn7)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign50_e809) + (assign50_e806 * locals.var_temp_dn8)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign50_e809) + (assign50_e806 * locals.var_temp_dn9)) / (2.0 * assign50_e813)))),)
    } else {
        (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9,)
    }
};
        locals.var_tkd = assign50_e817;
        locals.var_tkd_dn4 = assign50_e817_d_n4;
        locals.var_tkd_dn6 = assign50_e817_d_n6;
        locals.var_tkd_dn7 = assign50_e817_d_n7;
        locals.var_tkd_dn8 = assign50_e817_d_n8;
        locals.var_tkd_dn9 = assign50_e817_d_n9;

        let (assign60_e822, assign60_e822_d_n4, assign60_e822_d_n6, assign60_e822_d_n7, assign60_e822_d_n8, assign60_e822_d_n9,) = {
    if (locals.var_guard1 == 0.0) {
        (600.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9,)
    }
};
        locals.var_xsatmax = assign60_e822;
        locals.var_xsatmax_dn4 = assign60_e822_d_n4;
        locals.var_xsatmax_dn6 = assign60_e822_d_n6;
        locals.var_xsatmax_dn7 = assign60_e822_d_n7;
        locals.var_xsatmax_dn8 = assign60_e822_d_n8;
        locals.var_xsatmax_dn9 = assign60_e822_d_n9;

        let assign70_e837: f64 = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p439 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign70_e837;

        let (assign80_e841,) = {
    if (locals.var_guard2 != 0.0) {
        (p.p5,)
    } else {
        (locals.var_swshe_i,)
    }
};
        locals.var_swshe_i = assign80_e841;

        let (assign90_e846,) = {
    if (locals.var_guard2 == 0.0) {
        (0.0,)
    } else {
        (locals.var_swshe_i,)
    }
};
        locals.var_swshe_i = assign90_e846;

        locals.var_dtc = 0.0;
        locals.var_dtc_dn4 = 0.0;

        locals.var_tkc = locals.var_tkd;
        locals.var_tkc_dn4 = locals.var_tkd_dn4;
        locals.var_tkc_dn6 = locals.var_tkd_dn6;
        locals.var_tkc_dn7 = locals.var_tkd_dn7;
        locals.var_tkc_dn8 = locals.var_tkd_dn8;
        locals.var_tkc_dn9 = locals.var_tkd_dn9;

        let assign140_e856: f64 = (locals.var_tkc * locals.var_tkc);
        locals.var_tkc_sq = assign140_e856;
        locals.var_tkc_sq_dn4 = ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4));
        locals.var_tkc_sq_dn6 = ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6));
        locals.var_tkc_sq_dn7 = ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7));
        locals.var_tkc_sq_dn8 = ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8));
        locals.var_tkc_sq_dn9 = ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9));

        let assign150_e859: f64 = (locals.var_tkc - locals.var_tkr);
        locals.var_dt = assign150_e859;
        locals.var_dt_dn4 = locals.var_tkc_dn4;
        locals.var_dt_dn6 = locals.var_tkc_dn6;
        locals.var_dt_dn7 = locals.var_tkc_dn7;
        locals.var_dt_dn8 = locals.var_tkc_dn8;
        locals.var_dt_dn9 = locals.var_tkc_dn9;

        let assign160_e862: f64 = (locals.var_tkc / locals.var_tkr);
        locals.var_rt = assign160_e862;
        locals.var_rt_dn4 = (locals.var_tkc_dn4 / locals.var_tkr);
        locals.var_rt_dn6 = (locals.var_tkc_dn6 / locals.var_tkr);
        locals.var_rt_dn7 = (locals.var_tkc_dn7 / locals.var_tkr);
        locals.var_rt_dn8 = (locals.var_tkc_dn8 / locals.var_tkr);
        locals.var_rt_dn9 = (locals.var_tkc_dn9 / locals.var_tkr);

        let assign170_e865: f64 = (locals.var_tkr / locals.var_tkc);
        locals.var_rtn = assign170_e865;
        locals.var_rtn_dn4 = (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn6 = (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn7 = (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn8 = (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc)));
        locals.var_rtn_dn9 = (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc)));

        let assign180_e868: f64 = (locals.var_tkc * 8.617332384961e-5);
        locals.var_phit0 = assign180_e868;
        locals.var_phit0_dn4 = (locals.var_tkc_dn4 * 8.617332384961e-5);
        locals.var_phit0_dn6 = (locals.var_tkc_dn6 * 8.617332384961e-5);
        locals.var_phit0_dn7 = (locals.var_tkc_dn7 * 8.617332384961e-5);
        locals.var_phit0_dn8 = (locals.var_tkc_dn8 * 8.617332384961e-5);
        locals.var_phit0_dn9 = (locals.var_tkc_dn9 * 8.617332384961e-5);

        let assign190_e871: f64 = (1.0 / locals.var_phit0);
        locals.var_inv_phit0 = assign190_e871;
        locals.var_inv_phit0_dn4 = (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn6 = (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn7 = (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn8 = (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0)));
        locals.var_inv_phit0_dn9 = (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0)));

        let assign200_e874: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign200_e874;

        let (assign210_e878,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p23,)
    } else {
        (locals.var_adrain_i,)
    }
};
        locals.var_adrain_i = assign210_e878;

        let (assign220_e882,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p22,)
    } else {
        (locals.var_asource_i,)
    }
};
        locals.var_asource_i = assign220_e882;

        let (assign230_e886,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p25,)
    } else {
        (locals.var_pdrain_i,)
    }
};
        locals.var_pdrain_i = assign230_e886;

        let (assign240_e890,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p24,)
    } else {
        (locals.var_psource_i,)
    }
};
        locals.var_psource_i = assign240_e890;

        let (assign250_e894,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p30,)
    } else {
        (locals.var_mult_i_int,)
    }
};
        locals.var_mult_i_int = assign250_e894;

        let (assign260_e898,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p41,)
    } else {
        (locals.var_tox1_i,)
    }
};
        locals.var_tox1_i = assign260_e898;

        let (assign270_e902,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_tsi_i,)
    }
};
        locals.var_tsi_i = assign270_e902;

        let (assign280_e906,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p43,)
    } else {
        (locals.var_xge_i,)
    }
};
        locals.var_xge_i = assign280_e906;

        let (assign290_e910,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p44,)
    } else {
        (locals.var_tox2_i,)
    }
};
        locals.var_tox2_i = assign290_e910;

        let (assign300_e914,) = {
    if (locals.var_guard83 != 0.0) {
        (1.0,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign300_e914;

        let assign310_e917: f64 = if p.p45 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign310_e917;

        let (assign320_e924,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign320_e922: f64 = (-1.0);
        (assign320_e922,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign320_e924;

        let (assign330_e933,) = {
    if (locals.var_guard83 != 0.0) {
        let assign330_e927: f64 = (p.p45).abs();
        let assign330_e929: f64 = (assign330_e927).min(1e19);
        let assign330_e931: f64 = (assign330_e929 * 1000000.0);
        (assign330_e931,)
    } else {
        (locals.var_nch_i,)
    }
};
        locals.var_nch_i = assign330_e933;

        let (assign340_e937,) = {
    if (locals.var_guard83 != 0.0) {
        (1.0,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign340_e937;

        let assign350_e940: f64 = if p.p46 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign350_e940;

        let (assign360_e947,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign360_e945: f64 = (-1.0);
        (assign360_e945,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign360_e947;

        let (assign370_e958,) = {
    if (locals.var_guard83 != 0.0) {
        let assign370_e950: f64 = (p.p46).abs();
        let assign370_e952: f64 = (assign370_e950).max(1e16);
        let assign370_e954: f64 = (assign370_e952).min(1e21);
        let assign370_e956: f64 = (assign370_e954 * 1000000.0);
        (assign370_e956,)
    } else {
        (locals.var_nsub_i,)
    }
};
        locals.var_nsub_i = assign370_e958;

        let (assign380_e962,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_ct_i,)
    }
};
        locals.var_ct_i = assign380_e962;

        let (assign390_e966,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p48,)
    } else {
        (locals.var_toxp_i,)
    }
};
        locals.var_toxp_i = assign390_e966;

        let (assign400_e972,) = {
    if (locals.var_guard83 != 0.0) {
        let assign400_e970: f64 = (p.p49 * 1000000.0);
        (assign400_e970,)
    } else {
        (locals.var_nov_i,)
    }
};
        locals.var_nov_i = assign400_e972;

        let (assign410_e978,) = {
    if (locals.var_guard83 != 0.0) {
        let assign410_e976: f64 = (p.p50 * 1000000.0);
        (assign410_e976,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign410_e978;

        let (assign420_e982, assign420_e982_d_n4, assign420_e982_d_n6, assign420_e982_d_n7, assign420_e982_d_n8, assign420_e982_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign420_e982;
        locals.var_vfb1_t_dn4 = assign420_e982_d_n4;
        locals.var_vfb1_t_dn6 = assign420_e982_d_n6;
        locals.var_vfb1_t_dn7 = assign420_e982_d_n7;
        locals.var_vfb1_t_dn8 = assign420_e982_d_n8;
        locals.var_vfb1_t_dn9 = assign420_e982_d_n9;

        let (assign430_e986, assign430_e986_d_n4, assign430_e986_d_n6, assign430_e986_d_n7, assign430_e986_d_n8, assign430_e986_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign430_e986;
        locals.var_vfb2_t_dn4 = assign430_e986_d_n4;
        locals.var_vfb2_t_dn6 = assign430_e986_d_n6;
        locals.var_vfb2_t_dn7 = assign430_e986_d_n7;
        locals.var_vfb2_t_dn8 = assign430_e986_d_n8;
        locals.var_vfb2_t_dn9 = assign430_e986_d_n9;

        let (assign440_e990,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p53,)
    } else {
        (locals.var_stvfb_i,)
    }
};
        locals.var_stvfb_i = assign440_e990;

        let (assign450_e996, assign450_e996_d_n4, assign450_e996_d_n6, assign450_e996_d_n7, assign450_e996_d_n8, assign450_e996_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign450_e994: f64 = (p.p54 * 1000000.0);
        (assign450_e994, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9,)
    }
};
        locals.var_np_i = assign450_e996;
        locals.var_np_i_dn4 = assign450_e996_d_n4;
        locals.var_np_i_dn6 = assign450_e996_d_n6;
        locals.var_np_i_dn7 = assign450_e996_d_n7;
        locals.var_np_i_dn8 = assign450_e996_d_n8;
        locals.var_np_i_dn9 = assign450_e996_d_n9;

        let (assign460_e1000,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p55,)
    } else {
        (locals.var_cic1_i,)
    }
};
        locals.var_cic1_i = assign460_e1000;

        let (assign470_e1004,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p56,)
    } else {
        (locals.var_cic2_i,)
    }
};
        locals.var_cic2_i = assign470_e1004;

        let (assign480_e1008,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p57,)
    } else {
        (locals.var_psce1_i,)
    }
};
        locals.var_psce1_i = assign480_e1008;

        let (assign490_e1018,) = {
    if (locals.var_guard83 != 0.0) {
        let assign490_e1012: f64 = (p.p58 * locals.var_psce1_i);
        let assign490_e1014: f64 = (assign490_e1012 * locals.var_tox2_i);
        let assign490_e1016: f64 = (assign490_e1014 / locals.var_tox1_i);
        (assign490_e1016,)
    } else {
        (locals.var_psce2_i,)
    }
};
        locals.var_psce2_i = assign490_e1018;

        let (assign500_e1024,) = {
    if (locals.var_guard83 != 0.0) {
        let assign500_e1022: f64 = (p.p59 * 1000000.0);
        (assign500_e1022,)
    } else {
        (locals.var_nsddc_i,)
    }
};
        locals.var_nsddc_i = assign500_e1024;

        let (assign510_e1028,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p60,)
    } else {
        (locals.var_pscedlb_i,)
    }
};
        locals.var_pscedlb_i = assign510_e1028;

        let (assign520_e1032,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p61,)
    } else {
        (locals.var_pnce_i,)
    }
};
        locals.var_pnce_i = assign520_e1032;

        let (assign530_e1036, assign530_e1036_d_n4, assign530_e1036_d_n6, assign530_e1036_d_n7, assign530_e1036_d_n8, assign530_e1036_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign530_e1036;
        locals.var_cf1_t_dn4 = assign530_e1036_d_n4;
        locals.var_cf1_t_dn6 = assign530_e1036_d_n6;
        locals.var_cf1_t_dn7 = assign530_e1036_d_n7;
        locals.var_cf1_t_dn8 = assign530_e1036_d_n8;
        locals.var_cf1_t_dn9 = assign530_e1036_d_n9;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign540_e1046, assign540_e1046_d_n4, assign540_e1046_d_n6, assign540_e1046_d_n7, assign540_e1046_d_n8, assign540_e1046_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign540_e1040: f64 = (p.p63 * locals.var_cf1_t);
        let assign540_e1042: f64 = (assign540_e1040 * locals.var_tox2_i);
        let assign540_e1044: f64 = (assign540_e1042 / locals.var_tox1_i);
        (assign540_e1044, (((p.p63 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign540_e1046;
        locals.var_cf2_t_dn4 = assign540_e1046_d_n4;
        locals.var_cf2_t_dn6 = assign540_e1046_d_n6;
        locals.var_cf2_t_dn7 = assign540_e1046_d_n7;
        locals.var_cf2_t_dn8 = assign540_e1046_d_n8;
        locals.var_cf2_t_dn9 = assign540_e1046_d_n9;

        let (assign550_e1050, assign550_e1050_d_n4, assign550_e1050_d_n6, assign550_e1050_d_n7, assign550_e1050_d_n8, assign550_e1050_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p64, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9,)
    }
};
        locals.var_stcf_i = assign550_e1050;
        locals.var_stcf_i_dn4 = assign550_e1050_d_n4;
        locals.var_stcf_i_dn6 = assign550_e1050_d_n6;
        locals.var_stcf_i_dn7 = assign550_e1050_d_n7;
        locals.var_stcf_i_dn8 = assign550_e1050_d_n8;
        locals.var_stcf_i_dn9 = assign550_e1050_d_n9;

        let (assign560_e1054,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p65,)
    } else {
        (locals.var_cfd_i,)
    }
};
        locals.var_cfd_i = assign560_e1054;

        let (assign570_e1058,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p66,)
    } else {
        (locals.var_cfdl_i,)
    }
};
        locals.var_cfdl_i = assign570_e1058;

        let (assign580_e1062,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p67,)
    } else {
        (locals.var_cfdlb_i,)
    }
};
        locals.var_cfdlb_i = assign580_e1062;

        let (assign590_e1066, assign590_e1066_d_n4, assign590_e1066_d_n6, assign590_e1066_d_n7, assign590_e1066_d_n8, assign590_e1066_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p68, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign590_e1066;
        locals.var_betn1_t_dn4 = assign590_e1066_d_n4;
        locals.var_betn1_t_dn6 = assign590_e1066_d_n6;
        locals.var_betn1_t_dn7 = assign590_e1066_d_n7;
        locals.var_betn1_t_dn8 = assign590_e1066_d_n8;
        locals.var_betn1_t_dn9 = assign590_e1066_d_n9;

        let (assign600_e1072, assign600_e1072_d_n4, assign600_e1072_d_n6, assign600_e1072_d_n7, assign600_e1072_d_n8, assign600_e1072_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign600_e1070: f64 = (p.p69 * locals.var_betn1_t);
        (assign600_e1070, (p.p69 * locals.var_betn1_t_dn4), (p.p69 * locals.var_betn1_t_dn6), (p.p69 * locals.var_betn1_t_dn7), (p.p69 * locals.var_betn1_t_dn8), (p.p69 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign600_e1072;
        locals.var_betn2_t_dn4 = assign600_e1072_d_n4;
        locals.var_betn2_t_dn6 = assign600_e1072_d_n6;
        locals.var_betn2_t_dn7 = assign600_e1072_d_n7;
        locals.var_betn2_t_dn8 = assign600_e1072_d_n8;
        locals.var_betn2_t_dn9 = assign600_e1072_d_n9;

        let (assign610_e1076,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p70,)
    } else {
        (locals.var_stbet_i,)
    }
};
        locals.var_stbet_i = assign610_e1076;

        let (assign620_e1080,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p71,)
    } else {
        (locals.var_cs_t,)
    }
};
        locals.var_cs_t = assign620_e1080;

        let (assign630_e1084,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p72,)
    } else {
        (locals.var_csfi_i,)
    }
};
        locals.var_csfi_i = assign630_e1084;

        let (assign640_e1088,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p73,)
    } else {
        (locals.var_csbi_i,)
    }
};
        locals.var_csbi_i = assign640_e1088;

        let (assign650_e1092,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p74,)
    } else {
        (locals.var_stcs_i,)
    }
};
        locals.var_stcs_i = assign650_e1092;

        let (assign660_e1096,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p75,)
    } else {
        (locals.var_thecs_t,)
    }
};
        locals.var_thecs_t = assign660_e1096;

        let (assign670_e1100,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p76,)
    } else {
        (locals.var_stthecs_i,)
    }
};
        locals.var_stthecs_i = assign670_e1100;

        let (assign680_e1104,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p77,)
    } else {
        (locals.var_csthr_i,)
    }
};
        locals.var_csthr_i = assign680_e1104;

        let (assign690_e1108,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p78,)
    } else {
        (locals.var_csthrb_i,)
    }
};
        locals.var_csthrb_i = assign690_e1108;

        let (assign700_e1112,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p79,)
    } else {
        (locals.var_mue_t,)
    }
};
        locals.var_mue_t = assign700_e1112;

        let (assign710_e1116,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p80,)
    } else {
        (locals.var_stmue_i,)
    }
};
        locals.var_stmue_i = assign710_e1116;

        let (assign720_e1120,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p81,)
    } else {
        (locals.var_themu_t,)
    }
};
        locals.var_themu_t = assign720_e1120;

        let (assign730_e1124,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p82,)
    } else {
        (locals.var_stthemu_i,)
    }
};
        locals.var_stthemu_i = assign730_e1124;

        let (assign740_e1128,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p83,)
    } else {
        (locals.var_xcor_t,)
    }
};
        locals.var_xcor_t = assign740_e1128;

        let (assign750_e1132,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p84,)
    } else {
        (locals.var_xcorb_i,)
    }
};
        locals.var_xcorb_i = assign750_e1132;

        let (assign760_e1136,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p85,)
    } else {
        (locals.var_stxcor_i,)
    }
};
        locals.var_stxcor_i = assign760_e1136;

        let (assign770_e1140,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p86,)
    } else {
        (locals.var_feta_i,)
    }
};
        locals.var_feta_i = assign770_e1140;

        let (assign780_e1144,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p87,)
    } else {
        (locals.var_rs_t,)
    }
};
        locals.var_rs_t = assign780_e1144;

        let (assign790_e1148,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p88,)
    } else {
        (locals.var_rsig_i,)
    }
};
        locals.var_rsig_i = assign790_e1148;

        let (assign800_e1152,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p89,)
    } else {
        (locals.var_strs_i,)
    }
};
        locals.var_strs_i = assign800_e1152;

        let (assign810_e1156,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p90,)
    } else {
        (locals.var_rsg_i,)
    }
};
        locals.var_rsg_i = assign810_e1156;

        let (assign820_e1160,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p91,)
    } else {
        (locals.var_thersg_i,)
    }
};
        locals.var_thersg_i = assign820_e1160;

        let (assign830_e1164,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p92,)
    } else {
        (locals.var_rsb_i,)
    }
};
        locals.var_rsb_i = assign830_e1164;

        let (assign840_e1168, assign840_e1168_d_n4, assign840_e1168_d_n6, assign840_e1168_d_n7, assign840_e1168_d_n8, assign840_e1168_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign840_e1168;
        locals.var_thesat_t_dn4 = assign840_e1168_d_n4;
        locals.var_thesat_t_dn6 = assign840_e1168_d_n6;
        locals.var_thesat_t_dn7 = assign840_e1168_d_n7;
        locals.var_thesat_t_dn8 = assign840_e1168_d_n8;
        locals.var_thesat_t_dn9 = assign840_e1168_d_n9;

        let (assign850_e1172,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p94,)
    } else {
        (locals.var_stthesat_i,)
    }
};
        locals.var_stthesat_i = assign850_e1172;

        let (assign860_e1176,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_thesat1_i,)
    }
};
        locals.var_thesat1_i = assign860_e1176;

        let (assign870_e1180,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p96,)
    } else {
        (locals.var_thesat2_i,)
    }
};
        locals.var_thesat2_i = assign870_e1180;

        let (assign880_e1184,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p97,)
    } else {
        (locals.var_ax_i,)
    }
};
        locals.var_ax_i = assign880_e1184;

        let (assign890_e1188,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p98,)
    } else {
        (locals.var_alp_i,)
    }
};
        locals.var_alp_i = assign890_e1188;

        let (assign900_e1192,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p99,)
    } else {
        (locals.var_alp1_i,)
    }
};
        locals.var_alp1_i = assign900_e1192;

        let (assign910_e1196,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p100,)
    } else {
        (locals.var_alpb_i,)
    }
};
        locals.var_alpb_i = assign910_e1196;

        let (assign920_e1200,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p101,)
    } else {
        (locals.var_vp_i,)
    }
};
        locals.var_vp_i = assign920_e1200;

        let (assign930_e1204,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p102,)
    } else {
        (locals.var_vpg_i,)
    }
};
        locals.var_vpg_i = assign930_e1204;

        let (assign940_e1208,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p103,)
    } else {
        (locals.var_gco_i,)
    }
};
        locals.var_gco_i = assign940_e1208;

        let (assign950_e1212,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p104,)
    } else {
        (locals.var_iginv_t,)
    }
};
        locals.var_iginv_t = assign950_e1212;

        let (assign960_e1216,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p105,)
    } else {
        (locals.var_igovinv_t,)
    }
};
        locals.var_igovinv_t = assign960_e1216;

        let (assign970_e1220,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p106,)
    } else {
        (locals.var_igovinvd_t,)
    }
};
        locals.var_igovinvd_t = assign970_e1220;

        let (assign980_e1224,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p120,)
    } else {
        (locals.var_fnovinv_t,)
    }
};
        locals.var_fnovinv_t = assign980_e1224;

        let (assign990_e1228,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p121,)
    } else {
        (locals.var_fnovinvd_t,)
    }
};
        locals.var_fnovinvd_t = assign990_e1228;

        let (assign1000_e1232,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p107,)
    } else {
        (locals.var_igovacc_t,)
    }
};
        locals.var_igovacc_t = assign1000_e1232;

        let (assign1010_e1236,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p108,)
    } else {
        (locals.var_igovaccd_t,)
    }
};
        locals.var_igovaccd_t = assign1010_e1236;

        let (assign1020_e1240,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p109,)
    } else {
        (locals.var_stig_i,)
    }
};
        locals.var_stig_i = assign1020_e1240;

        let (assign1030_e1244,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_stigfn_i,)
    }
};
        locals.var_stigfn_i = assign1030_e1244;

        let (assign1040_e1248,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p110,)
    } else {
        (locals.var_gc2ch_i,)
    }
};
        locals.var_gc2ch_i = assign1040_e1248;

        let (assign1050_e1252,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p111,)
    } else {
        (locals.var_gc3ch_i,)
    }
};
        locals.var_gc3ch_i = assign1050_e1252;

        let (assign1060_e1256,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p112,)
    } else {
        (locals.var_gc2ovinv_i,)
    }
};
        locals.var_gc2ovinv_i = assign1060_e1256;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1070_e1260,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gcovinvfn_i,)
    }
};
        locals.var_gcovinvfn_i = assign1070_e1260;

        let (assign1080_e1264,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p113,)
    } else {
        (locals.var_gc3ovinv_i,)
    }
};
        locals.var_gc3ovinv_i = assign1080_e1264;

        let (assign1090_e1268,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p114,)
    } else {
        (locals.var_gc2ovacc_i,)
    }
};
        locals.var_gc2ovacc_i = assign1090_e1268;

        let (assign1100_e1272,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p115,)
    } else {
        (locals.var_gc3ovacc_i,)
    }
};
        locals.var_gc3ovacc_i = assign1100_e1272;

        let (assign1110_e1276,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p116,)
    } else {
        (locals.var_gcdov_i,)
    }
};
        locals.var_gcdov_i = assign1110_e1276;

        let (assign1120_e1280,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p117,)
    } else {
        (locals.var_gcvdov_i,)
    }
};
        locals.var_gcvdov_i = assign1120_e1280;

        let (assign1130_e1284,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p118,)
    } else {
        (locals.var_chib_i,)
    }
};
        locals.var_chib_i = assign1130_e1284;

        let (assign1140_e1288,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p119,)
    } else {
        (locals.var_niginv_i,)
    }
};
        locals.var_niginv_i = assign1140_e1288;

        let (assign1150_e1292, assign1150_e1292_d_n4, assign1150_e1292_d_n6, assign1150_e1292_d_n7, assign1150_e1292_d_n8, assign1150_e1292_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p124, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9,)
    }
};
        locals.var_agidl_i = assign1150_e1292;
        locals.var_agidl_i_dn4 = assign1150_e1292_d_n4;
        locals.var_agidl_i_dn6 = assign1150_e1292_d_n6;
        locals.var_agidl_i_dn7 = assign1150_e1292_d_n7;
        locals.var_agidl_i_dn8 = assign1150_e1292_d_n8;
        locals.var_agidl_i_dn9 = assign1150_e1292_d_n9;

        let (assign1160_e1296, assign1160_e1296_d_n4, assign1160_e1296_d_n6, assign1160_e1296_d_n7, assign1160_e1296_d_n8, assign1160_e1296_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p125, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9,)
    }
};
        locals.var_agidld_i = assign1160_e1296;
        locals.var_agidld_i_dn4 = assign1160_e1296_d_n4;
        locals.var_agidld_i_dn6 = assign1160_e1296_d_n6;
        locals.var_agidld_i_dn7 = assign1160_e1296_d_n7;
        locals.var_agidld_i_dn8 = assign1160_e1296_d_n8;
        locals.var_agidld_i_dn9 = assign1160_e1296_d_n9;

        let (assign1170_e1300,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p126,)
    } else {
        (locals.var_bgidl_t,)
    }
};
        locals.var_bgidl_t = assign1170_e1300;

        let (assign1180_e1304,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p127,)
    } else {
        (locals.var_bgidld_t,)
    }
};
        locals.var_bgidld_t = assign1180_e1304;

        let (assign1190_e1308,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p128,)
    } else {
        (locals.var_stbgidl_i,)
    }
};
        locals.var_stbgidl_i = assign1190_e1308;

        let (assign1200_e1312,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p129,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign1200_e1312;

        let (assign1210_e1316,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p130,)
    } else {
        (locals.var_cgidl_i,)
    }
};
        locals.var_cgidl_i = assign1210_e1316;

        let (assign1220_e1320,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p131,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign1220_e1320;

        let (assign1230_e1324,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p132,)
    } else {
        (locals.var_dgidl_i,)
    }
};
        locals.var_dgidl_i = assign1230_e1324;

        let (assign1240_e1328,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p133,)
    } else {
        (locals.var_dgidld_i,)
    }
};
        locals.var_dgidld_i = assign1240_e1328;

        let (assign1250_e1332,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p147,)
    } else {
        (locals.var_a1_i,)
    }
};
        locals.var_a1_i = assign1250_e1332;

        let (assign1260_e1336,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p148,)
    } else {
        (locals.var_a2_t,)
    }
};
        locals.var_a2_t = assign1260_e1336;

        let (assign1270_e1340,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p149,)
    } else {
        (locals.var_sta2_i,)
    }
};
        locals.var_sta2_i = assign1270_e1340;

        let (assign1280_e1344,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p150,)
    } else {
        (locals.var_a3_i,)
    }
};
        locals.var_a3_i = assign1280_e1344;

        let (assign1290_e1348,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p134,)
    } else {
        (locals.var_ctedge_i,)
    }
};
        locals.var_ctedge_i = assign1290_e1348;

        let (assign1300_e1352, assign1300_e1352_d_n4, assign1300_e1352_d_n6, assign1300_e1352_d_n7, assign1300_e1352_d_n8, assign1300_e1352_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p135, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb1edge_t, locals.var_vfb1edge_t_dn4, locals.var_vfb1edge_t_dn6, locals.var_vfb1edge_t_dn7, locals.var_vfb1edge_t_dn8, locals.var_vfb1edge_t_dn9,)
    }
};
        locals.var_vfb1edge_t = assign1300_e1352;
        locals.var_vfb1edge_t_dn4 = assign1300_e1352_d_n4;
        locals.var_vfb1edge_t_dn6 = assign1300_e1352_d_n6;
        locals.var_vfb1edge_t_dn7 = assign1300_e1352_d_n7;
        locals.var_vfb1edge_t_dn8 = assign1300_e1352_d_n8;
        locals.var_vfb1edge_t_dn9 = assign1300_e1352_d_n9;

        let (assign1310_e1356,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p136,)
    } else {
        (locals.var_vfb2edge_t,)
    }
};
        locals.var_vfb2edge_t = assign1310_e1356;

        let (assign1320_e1360,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_stvfbedge_i,)
    }
};
        locals.var_stvfbedge_i = assign1320_e1360;

        let (assign1330_e1364,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_cic1edge_i,)
    }
};
        locals.var_cic1edge_i = assign1330_e1364;

        let (assign1340_e1368,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p139,)
    } else {
        (locals.var_cic2edge_i,)
    }
};
        locals.var_cic2edge_i = assign1340_e1368;

        let (assign1350_e1372, assign1350_e1372_d_n4, assign1350_e1372_d_n6, assign1350_e1372_d_n7, assign1350_e1372_d_n8, assign1350_e1372_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p140, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_psce1edge_i, locals.var_psce1edge_i_dn4, locals.var_psce1edge_i_dn6, locals.var_psce1edge_i_dn7, locals.var_psce1edge_i_dn8, locals.var_psce1edge_i_dn9,)
    }
};
        locals.var_psce1edge_i = assign1350_e1372;
        locals.var_psce1edge_i_dn4 = assign1350_e1372_d_n4;
        locals.var_psce1edge_i_dn6 = assign1350_e1372_d_n6;
        locals.var_psce1edge_i_dn7 = assign1350_e1372_d_n7;
        locals.var_psce1edge_i_dn8 = assign1350_e1372_d_n8;
        locals.var_psce1edge_i_dn9 = assign1350_e1372_d_n9;

        let (assign1360_e1382, assign1360_e1382_d_n4, assign1360_e1382_d_n6, assign1360_e1382_d_n7, assign1360_e1382_d_n8, assign1360_e1382_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1360_e1376: f64 = (p.p141 * locals.var_psce1edge_i);
        let assign1360_e1378: f64 = (assign1360_e1376 * locals.var_tox2_i);
        let assign1360_e1380: f64 = (assign1360_e1378 / locals.var_tox1_i);
        (assign1360_e1380, (((p.p141 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9,)
    }
};
        locals.var_psce2edge_i = assign1360_e1382;
        locals.var_psce2edge_i_dn4 = assign1360_e1382_d_n4;
        locals.var_psce2edge_i_dn6 = assign1360_e1382_d_n6;
        locals.var_psce2edge_i_dn7 = assign1360_e1382_d_n7;
        locals.var_psce2edge_i_dn8 = assign1360_e1382_d_n8;
        locals.var_psce2edge_i_dn9 = assign1360_e1382_d_n9;

        let (assign1370_e1386, assign1370_e1386_d_n4, assign1370_e1386_d_n6, assign1370_e1386_d_n7, assign1370_e1386_d_n8, assign1370_e1386_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p142, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9,)
    }
};
        locals.var_cf1edge_i = assign1370_e1386;
        locals.var_cf1edge_i_dn4 = assign1370_e1386_d_n4;
        locals.var_cf1edge_i_dn6 = assign1370_e1386_d_n6;
        locals.var_cf1edge_i_dn7 = assign1370_e1386_d_n7;
        locals.var_cf1edge_i_dn8 = assign1370_e1386_d_n8;
        locals.var_cf1edge_i_dn9 = assign1370_e1386_d_n9;

        let (assign1380_e1396, assign1380_e1396_d_n4, assign1380_e1396_d_n6, assign1380_e1396_d_n7, assign1380_e1396_d_n8, assign1380_e1396_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1380_e1390: f64 = (p.p143 * locals.var_cf1edge_i);
        let assign1380_e1392: f64 = (assign1380_e1390 * locals.var_tox2_i);
        let assign1380_e1394: f64 = (assign1380_e1392 / locals.var_tox1_i);
        (assign1380_e1394, (((p.p143 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9,)
    }
};
        locals.var_cf2edge_i = assign1380_e1396;
        locals.var_cf2edge_i_dn4 = assign1380_e1396_d_n4;
        locals.var_cf2edge_i_dn6 = assign1380_e1396_d_n6;
        locals.var_cf2edge_i_dn7 = assign1380_e1396_d_n7;
        locals.var_cf2edge_i_dn8 = assign1380_e1396_d_n8;
        locals.var_cf2edge_i_dn9 = assign1380_e1396_d_n9;

        let (assign1390_e1400,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p144,)
    } else {
        (locals.var_cfdedge_i,)
    }
};
        locals.var_cfdedge_i = assign1390_e1400;

        let (assign1400_e1404, assign1400_e1404_d_n4, assign1400_e1404_d_n6, assign1400_e1404_d_n7, assign1400_e1404_d_n8, assign1400_e1404_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p145, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9,)
    }
};
        locals.var_betnedge_t = assign1400_e1404;
        locals.var_betnedge_t_dn4 = assign1400_e1404_d_n4;
        locals.var_betnedge_t_dn6 = assign1400_e1404_d_n6;
        locals.var_betnedge_t_dn7 = assign1400_e1404_d_n7;
        locals.var_betnedge_t_dn8 = assign1400_e1404_d_n8;
        locals.var_betnedge_t_dn9 = assign1400_e1404_d_n9;

        let (assign1410_e1408,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p146,)
    } else {
        (locals.var_stbetedge_i,)
    }
};
        locals.var_stbetedge_i = assign1410_e1408;

        let (assign1420_e1412,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p151,)
    } else {
        (locals.var_areaq_i,)
    }
};
        locals.var_areaq_i = assign1420_e1412;

        let (assign1430_e1416, assign1430_e1416_d_n4, assign1430_e1416_d_n6, assign1430_e1416_d_n7, assign1430_e1416_d_n8, assign1430_e1416_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p152, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9,)
    }
};
        locals.var_cgbov_i = assign1430_e1416;
        locals.var_cgbov_i_dn4 = assign1430_e1416_d_n4;
        locals.var_cgbov_i_dn6 = assign1430_e1416_d_n6;
        locals.var_cgbov_i_dn7 = assign1430_e1416_d_n7;
        locals.var_cgbov_i_dn8 = assign1430_e1416_d_n8;
        locals.var_cgbov_i_dn9 = assign1430_e1416_d_n9;

        let (assign1440_e1422,) = {
    if (locals.var_guard83 != 0.0) {
        let assign1440_e1420: f64 = (p.p153 * 1000000.0);
        (assign1440_e1420,)
    } else {
        (locals.var_nsdac_i,)
    }
};
        locals.var_nsdac_i = assign1440_e1422;

        let (assign1450_e1426,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p154,)
    } else {
        (locals.var_fif_i,)
    }
};
        locals.var_fif_i = assign1450_e1426;

        let (assign1460_e1430,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p155,)
    } else {
        (locals.var_fsceac_i,)
    }
};
        locals.var_fsceac_i = assign1460_e1430;

        let (assign1470_e1434, assign1470_e1434_d_n4, assign1470_e1434_d_n6, assign1470_e1434_d_n7, assign1470_e1434_d_n8, assign1470_e1434_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign1470_e1434;
        locals.var_vfbac1_t_dn4 = assign1470_e1434_d_n4;
        locals.var_vfbac1_t_dn6 = assign1470_e1434_d_n6;
        locals.var_vfbac1_t_dn7 = assign1470_e1434_d_n7;
        locals.var_vfbac1_t_dn8 = assign1470_e1434_d_n8;
        locals.var_vfbac1_t_dn9 = assign1470_e1434_d_n9;

        let (assign1480_e1438, assign1480_e1438_d_n4, assign1480_e1438_d_n6, assign1480_e1438_d_n7, assign1480_e1438_d_n8, assign1480_e1438_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign1480_e1438;
        locals.var_vfbac2_t_dn4 = assign1480_e1438_d_n4;
        locals.var_vfbac2_t_dn6 = assign1480_e1438_d_n6;
        locals.var_vfbac2_t_dn7 = assign1480_e1438_d_n7;
        locals.var_vfbac2_t_dn8 = assign1480_e1438_d_n8;
        locals.var_vfbac2_t_dn9 = assign1480_e1438_d_n9;

        let (assign1490_e1442,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_psce1_i,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign1490_e1442;

        let (assign1500_e1446,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_psce2_i,)
    } else {
        (locals.var_psceac2_i,)
    }
};
        locals.var_psceac2_i = assign1500_e1446;

        let (assign1510_e1450, assign1510_e1450_d_n4, assign1510_e1450_d_n6, assign1510_e1450_d_n7, assign1510_e1450_d_n8, assign1510_e1450_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign1510_e1450;
        locals.var_cfac1_t_dn4 = assign1510_e1450_d_n4;
        locals.var_cfac1_t_dn6 = assign1510_e1450_d_n6;
        locals.var_cfac1_t_dn7 = assign1510_e1450_d_n7;
        locals.var_cfac1_t_dn8 = assign1510_e1450_d_n8;
        locals.var_cfac1_t_dn9 = assign1510_e1450_d_n9;

        let (assign1520_e1454, assign1520_e1454_d_n4, assign1520_e1454_d_n6, assign1520_e1454_d_n7, assign1520_e1454_d_n8, assign1520_e1454_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign1520_e1454;
        locals.var_cfac2_t_dn4 = assign1520_e1454_d_n4;
        locals.var_cfac2_t_dn6 = assign1520_e1454_d_n6;
        locals.var_cfac2_t_dn7 = assign1520_e1454_d_n7;
        locals.var_cfac2_t_dn8 = assign1520_e1454_d_n8;
        locals.var_cfac2_t_dn9 = assign1520_e1454_d_n9;

        let (assign1530_e1458, assign1530_e1458_d_n4, assign1530_e1458_d_n6, assign1530_e1458_d_n7, assign1530_e1458_d_n8, assign1530_e1458_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign1530_e1458;
        locals.var_thesatac_t_dn4 = assign1530_e1458_d_n4;
        locals.var_thesatac_t_dn6 = assign1530_e1458_d_n6;
        locals.var_thesatac_t_dn7 = assign1530_e1458_d_n7;
        locals.var_thesatac_t_dn8 = assign1530_e1458_d_n8;
        locals.var_thesatac_t_dn9 = assign1530_e1458_d_n9;

        let (assign1540_e1462,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_ax_i,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign1540_e1462;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign1550_e1466,) = {
    if (locals.var_guard83 != 0.0) {
        (locals.var_alp_i,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign1550_e1466;

        let assign1560_e1469: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign1560_e1469;

        let (assign1570_e1475, assign1570_e1475_d_n4, assign1570_e1475_d_n6, assign1570_e1475_d_n7, assign1570_e1475_d_n8, assign1570_e1475_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign1570_e1475;
        locals.var_vfbac1_t_dn4 = assign1570_e1475_d_n4;
        locals.var_vfbac1_t_dn6 = assign1570_e1475_d_n6;
        locals.var_vfbac1_t_dn7 = assign1570_e1475_d_n7;
        locals.var_vfbac1_t_dn8 = assign1570_e1475_d_n8;
        locals.var_vfbac1_t_dn9 = assign1570_e1475_d_n9;

        let assign1580_e1477: f64 = if param_given[156] { 1.0 } else { 0.0 };
        let assign1580_e1479: f64 = if assign1580_e1477 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign1580_e1479;

        let (assign1590_e1487, assign1590_e1487_d_n4, assign1590_e1487_d_n6, assign1590_e1487_d_n7, assign1590_e1487_d_n8, assign1590_e1487_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) {
        (p.p156, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign1590_e1487;
        locals.var_vfbac1_t_dn4 = assign1590_e1487_d_n4;
        locals.var_vfbac1_t_dn6 = assign1590_e1487_d_n6;
        locals.var_vfbac1_t_dn7 = assign1590_e1487_d_n7;
        locals.var_vfbac1_t_dn8 = assign1590_e1487_d_n8;
        locals.var_vfbac1_t_dn9 = assign1590_e1487_d_n9;

        let (assign1600_e1493, assign1600_e1493_d_n4, assign1600_e1493_d_n6, assign1600_e1493_d_n7, assign1600_e1493_d_n8, assign1600_e1493_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign1600_e1493;
        locals.var_vfbac2_t_dn4 = assign1600_e1493_d_n4;
        locals.var_vfbac2_t_dn6 = assign1600_e1493_d_n6;
        locals.var_vfbac2_t_dn7 = assign1600_e1493_d_n7;
        locals.var_vfbac2_t_dn8 = assign1600_e1493_d_n8;
        locals.var_vfbac2_t_dn9 = assign1600_e1493_d_n9;

        let assign1610_e1495: f64 = if param_given[157] { 1.0 } else { 0.0 };
        let assign1610_e1497: f64 = if assign1610_e1495 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign1610_e1497;

        let (assign1620_e1505, assign1620_e1505_d_n4, assign1620_e1505_d_n6, assign1620_e1505_d_n7, assign1620_e1505_d_n8, assign1620_e1505_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard88 != 0.0)) {
        (p.p157, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign1620_e1505;
        locals.var_vfbac2_t_dn4 = assign1620_e1505_d_n4;
        locals.var_vfbac2_t_dn6 = assign1620_e1505_d_n6;
        locals.var_vfbac2_t_dn7 = assign1620_e1505_d_n7;
        locals.var_vfbac2_t_dn8 = assign1620_e1505_d_n8;
        locals.var_vfbac2_t_dn9 = assign1620_e1505_d_n9;

        let (assign1630_e1511,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p57,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign1630_e1511;

        let assign1640_e1513: f64 = if param_given[158] { 1.0 } else { 0.0 };
        let assign1640_e1515: f64 = if assign1640_e1513 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign1640_e1515;

        let (assign1650_e1523,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard89 != 0.0)) {
        (p.p158,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign1650_e1523;

        let (assign1660_e1535,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign1660_e1529: f64 = (p.p58 * locals.var_psceac1_i);
        let assign1660_e1531: f64 = (assign1660_e1529 * locals.var_tox2_i);
        let assign1660_e1533: f64 = (assign1660_e1531 / locals.var_tox1_i);
        (assign1660_e1533,)
    } else {
        (locals.var_psceac2_i,)
    }
};
        locals.var_psceac2_i = assign1660_e1535;

        let (assign1670_e1541, assign1670_e1541_d_n4, assign1670_e1541_d_n6, assign1670_e1541_d_n7, assign1670_e1541_d_n8, assign1670_e1541_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign1670_e1541;
        locals.var_cfac1_t_dn4 = assign1670_e1541_d_n4;
        locals.var_cfac1_t_dn6 = assign1670_e1541_d_n6;
        locals.var_cfac1_t_dn7 = assign1670_e1541_d_n7;
        locals.var_cfac1_t_dn8 = assign1670_e1541_d_n8;
        locals.var_cfac1_t_dn9 = assign1670_e1541_d_n9;

        let assign1680_e1543: f64 = if param_given[159] { 1.0 } else { 0.0 };
        let assign1680_e1545: f64 = if assign1680_e1543 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign1680_e1545;

        let (assign1690_e1553, assign1690_e1553_d_n4, assign1690_e1553_d_n6, assign1690_e1553_d_n7, assign1690_e1553_d_n8, assign1690_e1553_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard90 != 0.0)) {
        (p.p159, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign1690_e1553;
        locals.var_cfac1_t_dn4 = assign1690_e1553_d_n4;
        locals.var_cfac1_t_dn6 = assign1690_e1553_d_n6;
        locals.var_cfac1_t_dn7 = assign1690_e1553_d_n7;
        locals.var_cfac1_t_dn8 = assign1690_e1553_d_n8;
        locals.var_cfac1_t_dn9 = assign1690_e1553_d_n9;

        let (assign1700_e1565, assign1700_e1565_d_n4, assign1700_e1565_d_n6, assign1700_e1565_d_n7, assign1700_e1565_d_n8, assign1700_e1565_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign1700_e1559: f64 = (p.p63 * locals.var_cfac1_t);
        let assign1700_e1561: f64 = (assign1700_e1559 * locals.var_tox2_i);
        let assign1700_e1563: f64 = (assign1700_e1561 / locals.var_tox1_i);
        (assign1700_e1563, (((p.p63 * locals.var_cfac1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign1700_e1565;
        locals.var_cfac2_t_dn4 = assign1700_e1565_d_n4;
        locals.var_cfac2_t_dn6 = assign1700_e1565_d_n6;
        locals.var_cfac2_t_dn7 = assign1700_e1565_d_n7;
        locals.var_cfac2_t_dn8 = assign1700_e1565_d_n8;
        locals.var_cfac2_t_dn9 = assign1700_e1565_d_n9;

        let (assign1710_e1571, assign1710_e1571_d_n4, assign1710_e1571_d_n6, assign1710_e1571_d_n7, assign1710_e1571_d_n8, assign1710_e1571_d_n9,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign1710_e1571;
        locals.var_thesatac_t_dn4 = assign1710_e1571_d_n4;
        locals.var_thesatac_t_dn6 = assign1710_e1571_d_n6;
        locals.var_thesatac_t_dn7 = assign1710_e1571_d_n7;
        locals.var_thesatac_t_dn8 = assign1710_e1571_d_n8;
        locals.var_thesatac_t_dn9 = assign1710_e1571_d_n9;

        let assign1720_e1573: f64 = if param_given[160] { 1.0 } else { 0.0 };
        let assign1720_e1575: f64 = if assign1720_e1573 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign1720_e1575;

        let (assign1730_e1583, assign1730_e1583_d_n4, assign1730_e1583_d_n6, assign1730_e1583_d_n7, assign1730_e1583_d_n8, assign1730_e1583_d_n9,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard91 != 0.0)) {
        (p.p160, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign1730_e1583;
        locals.var_thesatac_t_dn4 = assign1730_e1583_d_n4;
        locals.var_thesatac_t_dn6 = assign1730_e1583_d_n6;
        locals.var_thesatac_t_dn7 = assign1730_e1583_d_n7;
        locals.var_thesatac_t_dn8 = assign1730_e1583_d_n8;
        locals.var_thesatac_t_dn9 = assign1730_e1583_d_n9;

        let (assign1740_e1589,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p97,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign1740_e1589;

        let assign1750_e1591: f64 = if param_given[161] { 1.0 } else { 0.0 };
        let assign1750_e1593: f64 = if assign1750_e1591 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign1750_e1593;

        let (assign1760_e1601,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard92 != 0.0)) {
        (p.p161,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign1760_e1601;

        let (assign1770_e1607,) = {
    if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
        (p.p98,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign1770_e1607;

        let assign1780_e1609: f64 = if param_given[162] { 1.0 } else { 0.0 };
        let assign1780_e1611: f64 = if assign1780_e1609 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign1780_e1611;

        let (assign1790_e1619,) = {
    if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard93 != 0.0)) {
        (p.p162,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign1790_e1619;

        let (assign1800_e1623, assign1800_e1623_d_n4, assign1800_e1623_d_n6, assign1800_e1623_d_n7, assign1800_e1623_d_n8, assign1800_e1623_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p163, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9,)
    }
};
        locals.var_cov_i = assign1800_e1623;
        locals.var_cov_i_dn4 = assign1800_e1623_d_n4;
        locals.var_cov_i_dn6 = assign1800_e1623_d_n6;
        locals.var_cov_i_dn7 = assign1800_e1623_d_n7;
        locals.var_cov_i_dn8 = assign1800_e1623_d_n8;
        locals.var_cov_i_dn9 = assign1800_e1623_d_n9;

        let (assign1810_e1627, assign1810_e1627_d_n4, assign1810_e1627_d_n6, assign1810_e1627_d_n7, assign1810_e1627_d_n8, assign1810_e1627_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p164, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9,)
    }
};
        locals.var_covd_i = assign1810_e1627;
        locals.var_covd_i_dn4 = assign1810_e1627_d_n4;
        locals.var_covd_i_dn6 = assign1810_e1627_d_n6;
        locals.var_covd_i_dn7 = assign1810_e1627_d_n7;
        locals.var_covd_i_dn8 = assign1810_e1627_d_n8;
        locals.var_covd_i_dn9 = assign1810_e1627_d_n9;

        let (assign1820_e1631,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p165,)
    } else {
        (locals.var_covdl_i,)
    }
};
        locals.var_covdl_i = assign1820_e1631;

        let (assign1830_e1635,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p166,)
    } else {
        (locals.var_covdlb_i,)
    }
};
        locals.var_covdlb_i = assign1830_e1635;

        let (assign1840_e1639,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p167,)
    } else {
        (locals.var_dvfbov_i,)
    }
};
        locals.var_dvfbov_i = assign1840_e1639;

        let (assign1850_e1643, assign1850_e1643_d_n4, assign1850_e1643_d_n6, assign1850_e1643_d_n7, assign1850_e1643_d_n8, assign1850_e1643_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p168, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9,)
    }
};
        locals.var_cfr_i = assign1850_e1643;
        locals.var_cfr_i_dn4 = assign1850_e1643_d_n4;
        locals.var_cfr_i_dn6 = assign1850_e1643_d_n6;
        locals.var_cfr_i_dn7 = assign1850_e1643_d_n7;
        locals.var_cfr_i_dn8 = assign1850_e1643_d_n8;
        locals.var_cfr_i_dn9 = assign1850_e1643_d_n9;

        let (assign1860_e1647, assign1860_e1647_d_n4, assign1860_e1647_d_n6, assign1860_e1647_d_n7, assign1860_e1647_d_n8, assign1860_e1647_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p169, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9,)
    }
};
        locals.var_cfrd_i = assign1860_e1647;
        locals.var_cfrd_i_dn4 = assign1860_e1647_d_n4;
        locals.var_cfrd_i_dn6 = assign1860_e1647_d_n6;
        locals.var_cfrd_i_dn7 = assign1860_e1647_d_n7;
        locals.var_cfrd_i_dn8 = assign1860_e1647_d_n8;
        locals.var_cfrd_i_dn9 = assign1860_e1647_d_n9;

        let (assign1870_e1651,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p170,)
    } else {
        (locals.var_csd_i,)
    }
};
        locals.var_csd_i = assign1870_e1651;

        let (assign1880_e1655,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p171,)
    } else {
        (locals.var_csdbp_i,)
    }
};
        locals.var_csdbp_i = assign1880_e1655;

        let (assign1890_e1659, assign1890_e1659_d_n4, assign1890_e1659_d_n6, assign1890_e1659_d_n7, assign1890_e1659_d_n8, assign1890_e1659_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p172, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth_t, locals.var_rth_t_dn4, locals.var_rth_t_dn6, locals.var_rth_t_dn7, locals.var_rth_t_dn8, locals.var_rth_t_dn9,)
    }
};
        locals.var_rth_t = assign1890_e1659;
        locals.var_rth_t_dn4 = assign1890_e1659_d_n4;
        locals.var_rth_t_dn6 = assign1890_e1659_d_n6;
        locals.var_rth_t_dn7 = assign1890_e1659_d_n7;
        locals.var_rth_t_dn8 = assign1890_e1659_d_n8;
        locals.var_rth_t_dn9 = assign1890_e1659_d_n9;

        let (assign1900_e1663,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p173,)
    } else {
        (locals.var_strth_i,)
    }
};
        locals.var_strth_i = assign1900_e1663;

        let (assign1910_e1667, assign1910_e1667_d_n4, assign1910_e1667_d_n6, assign1910_e1667_d_n7, assign1910_e1667_d_n8, assign1910_e1667_d_n9,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p174, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cth_i, locals.var_cth_i_dn4, locals.var_cth_i_dn6, locals.var_cth_i_dn7, locals.var_cth_i_dn8, locals.var_cth_i_dn9,)
    }
};
        locals.var_cth_i = assign1910_e1667;
        locals.var_cth_i_dn4 = assign1910_e1667_d_n4;
        locals.var_cth_i_dn6 = assign1910_e1667_d_n6;
        locals.var_cth_i_dn7 = assign1910_e1667_d_n7;
        locals.var_cth_i_dn8 = assign1910_e1667_d_n8;
        locals.var_cth_i_dn9 = assign1910_e1667_d_n9;

        let (assign1920_e1671,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p175,)
    } else {
        (locals.var_fnt_i,)
    }
};
        locals.var_fnt_i = assign1920_e1671;

        let (assign1930_e1675,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p176,)
    } else {
        (locals.var_fntexc_i,)
    }
};
        locals.var_fntexc_i = assign1930_e1675;

        let (assign1940_e1679,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p177,)
    } else {
        (locals.var_nfa_i,)
    }
};
        locals.var_nfa_i = assign1940_e1679;

        let (assign1950_e1683,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p178,)
    } else {
        (locals.var_nfb_i,)
    }
};
        locals.var_nfb_i = assign1950_e1683;

        let (assign1960_e1687,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p179,)
    } else {
        (locals.var_nfc_i,)
    }
};
        locals.var_nfc_i = assign1960_e1687;

        let (assign1970_e1691,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p180,)
    } else {
        (locals.var_nfe_i,)
    }
};
        locals.var_nfe_i = assign1970_e1691;

        let (assign1980_e1695,) = {
    if (locals.var_guard83 != 0.0) {
        (p.p181,)
    } else {
        (locals.var_nfeb_i,)
    }
};
        locals.var_nfeb_i = assign1980_e1695;

        let (assign2040_e1722,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2040_e1720: f64 = (1.0 / p.p29);
        (assign2040_e1720,)
    } else {
        (locals.var_invnf,)
    }
};
        locals.var_invnf = assign2040_e1722;

        let (assign2050_e1731,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2050_e1727: f64 = (p.p21 * locals.var_invnf);
        let assign2050_e1729: f64 = (assign2050_e1727).max(1e-9);
        (assign2050_e1729,)
    } else {
        (locals.var_w_i,)
    }
};
        locals.var_w_i = assign2050_e1731;

        let (assign2060_e1738,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2060_e1736: f64 = (p.p23 * locals.var_invnf);
        (assign2060_e1736,)
    } else {
        (locals.var_adrain_i,)
    }
};
        locals.var_adrain_i = assign2060_e1738;

        let (assign2070_e1745,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2070_e1743: f64 = (p.p22 * locals.var_invnf);
        (assign2070_e1743,)
    } else {
        (locals.var_asource_i,)
    }
};
        locals.var_asource_i = assign2070_e1745;

        let (assign2080_e1752,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2080_e1750: f64 = (p.p25 * locals.var_invnf);
        (assign2080_e1750,)
    } else {
        (locals.var_pdrain_i,)
    }
};
        locals.var_pdrain_i = assign2080_e1752;

        let (assign2090_e1759,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2090_e1757: f64 = (p.p24 * locals.var_invnf);
        (assign2090_e1757,)
    } else {
        (locals.var_psource_i,)
    }
};
        locals.var_psource_i = assign2090_e1759;

        let (assign2100_e1766,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2100_e1764: f64 = (p.p30 * p.p29);
        (assign2100_e1764,)
    } else {
        (locals.var_mult_i_int,)
    }
};
        locals.var_mult_i_int = assign2100_e1766;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2110_e1771,) = {
    if (locals.var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign2110_e1771;

        let (assign2120_e1776,) = {
    if (locals.var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (locals.var_wen,)
    }
};
        locals.var_wen = assign2120_e1776;

        let (assign2130_e1783,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2130_e1781: f64 = (locals.var_len / p.p20);
        (assign2130_e1781,)
    } else {
        (locals.var_il,)
    }
};
        locals.var_il = assign2130_e1783;

        let (assign2140_e1790,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2140_e1788: f64 = (locals.var_wen / locals.var_w_i);
        (assign2140_e1788,)
    } else {
        (locals.var_iw,)
    }
};
        locals.var_iw = assign2140_e1790;

        let (assign2150_e1807,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2150_e1797: f64 = (p.p188 * locals.var_il);
        let assign2150_e1798: f64 = (1.0 + assign2150_e1797);
        let assign2150_e1799: f64 = (p.p187 * assign2150_e1798);
        let assign2150_e1803: f64 = (p.p189 * locals.var_iw);
        let assign2150_e1804: f64 = (1.0 + assign2150_e1803);
        let assign2150_e1805: f64 = (assign2150_e1799 * assign2150_e1804);
        (assign2150_e1805,)
    } else {
        (locals.var_dellps,)
    }
};
        locals.var_dellps = assign2150_e1807;

        let (assign2160_e1824,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2160_e1814: f64 = (p.p193 * locals.var_iw);
        let assign2160_e1815: f64 = (1.0 + assign2160_e1814);
        let assign2160_e1816: f64 = (p.p191 * assign2160_e1815);
        let assign2160_e1820: f64 = (p.p192 * locals.var_il);
        let assign2160_e1821: f64 = (1.0 + assign2160_e1820);
        let assign2160_e1822: f64 = (assign2160_e1816 * assign2160_e1821);
        (assign2160_e1822,)
    } else {
        (locals.var_delwod,)
    }
};
        locals.var_delwod = assign2160_e1824;

        let (assign2170_e1837,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2170_e1829: f64 = (p.p20 + locals.var_dellps);
        let assign2170_e1832: f64 = (2.0 * p.p190);
        let assign2170_e1833: f64 = (assign2170_e1829 - assign2170_e1832);
        let assign2170_e1835: f64 = (assign2170_e1833).max(1e-9);
        (assign2170_e1835,)
    } else {
        (locals.var_le,)
    }
};
        locals.var_le = assign2170_e1837;

        let (assign2180_e1850,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2180_e1842: f64 = (locals.var_w_i + locals.var_delwod);
        let assign2180_e1845: f64 = (2.0 * p.p194);
        let assign2180_e1846: f64 = (assign2180_e1842 - assign2180_e1845);
        let assign2180_e1848: f64 = (assign2180_e1846).max(1e-9);
        (assign2180_e1848,)
    } else {
        (locals.var_we,)
    }
};
        locals.var_we = assign2180_e1850;

        let (assign2190_e1865,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2190_e1855: f64 = (p.p20 + locals.var_dellps);
        let assign2190_e1858: f64 = (2.0 * p.p190);
        let assign2190_e1859: f64 = (assign2190_e1855 - assign2190_e1858);
        let assign2190_e1861: f64 = (assign2190_e1859 + p.p195);
        let assign2190_e1863: f64 = (assign2190_e1861).max(1e-9);
        (assign2190_e1863,)
    } else {
        (locals.var_lecv,)
    }
};
        locals.var_lecv = assign2190_e1865;

        let (assign2200_e1880,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2200_e1870: f64 = (locals.var_w_i + locals.var_delwod);
        let assign2200_e1873: f64 = (2.0 * p.p194);
        let assign2200_e1874: f64 = (assign2200_e1870 - assign2200_e1873);
        let assign2200_e1876: f64 = (assign2200_e1874 + p.p196);
        let assign2200_e1878: f64 = (assign2200_e1876).max(1e-9);
        (assign2200_e1878,)
    } else {
        (locals.var_wecv,)
    }
};
        locals.var_wecv = assign2200_e1880;

        let (assign2210_e1887,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2210_e1885: f64 = (locals.var_len / locals.var_le);
        (assign2210_e1885,)
    } else {
        (locals.var_ile,)
    }
};
        locals.var_ile = assign2210_e1887;

        let (assign2220_e1894,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2220_e1892: f64 = (locals.var_wen / locals.var_we);
        (assign2220_e1892,)
    } else {
        (locals.var_iwe,)
    }
};
        locals.var_iwe = assign2220_e1894;

        let (assign2230_e1901,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2230_e1899: f64 = (locals.var_ile * locals.var_iwe);
        (assign2230_e1899,)
    } else {
        (locals.var_iae,)
    }
};
        locals.var_iae = assign2230_e1901;

        let (assign2240_e1910, assign2240_e1910_d_n4, assign2240_e1910_d_n6, assign2240_e1910_d_n7, assign2240_e1910_d_n8, assign2240_e1910_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2240_e1906: f64 = (p.p20 + locals.var_dellps);
        let assign2240_e1908: f64 = (assign2240_e1906).max(1e-9);
        (assign2240_e1908, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign2240_e1910;
        locals.var_temp_dn4 = assign2240_e1910_d_n4;
        locals.var_temp_dn6 = assign2240_e1910_d_n6;
        locals.var_temp_dn7 = assign2240_e1910_d_n7;
        locals.var_temp_dn8 = assign2240_e1910_d_n8;
        locals.var_temp_dn9 = assign2240_e1910_d_n9;

        let (assign2250_e1917, assign2250_e1917_d_n4, assign2250_e1917_d_n6, assign2250_e1917_d_n7, assign2250_e1917_d_n8, assign2250_e1917_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2250_e1915: f64 = (locals.var_temp / locals.var_len);
        (assign2250_e1915, (locals.var_temp_dn4 / locals.var_len), (locals.var_temp_dn6 / locals.var_len), (locals.var_temp_dn7 / locals.var_len), (locals.var_temp_dn8 / locals.var_len), (locals.var_temp_dn9 / locals.var_len),)
    } else {
        (locals.var_lphy, locals.var_lphy_dn4, locals.var_lphy_dn6, locals.var_lphy_dn7, locals.var_lphy_dn8, locals.var_lphy_dn9,)
    }
};
        locals.var_lphy = assign2250_e1917;
        locals.var_lphy_dn4 = assign2250_e1917_d_n4;
        locals.var_lphy_dn6 = assign2250_e1917_d_n6;
        locals.var_lphy_dn7 = assign2250_e1917_d_n7;
        locals.var_lphy_dn8 = assign2250_e1917_d_n8;
        locals.var_lphy_dn9 = assign2250_e1917_d_n9;

        let (assign2260_e1926, assign2260_e1926_d_n4, assign2260_e1926_d_n6, assign2260_e1926_d_n7, assign2260_e1926_d_n8, assign2260_e1926_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2260_e1922: f64 = (locals.var_w_i + locals.var_delwod);
        let assign2260_e1924: f64 = (assign2260_e1922).max(1e-9);
        (assign2260_e1924, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign2260_e1926;
        locals.var_temp_dn4 = assign2260_e1926_d_n4;
        locals.var_temp_dn6 = assign2260_e1926_d_n6;
        locals.var_temp_dn7 = assign2260_e1926_d_n7;
        locals.var_temp_dn8 = assign2260_e1926_d_n8;
        locals.var_temp_dn9 = assign2260_e1926_d_n9;

        let (assign2270_e1933, assign2270_e1933_d_n4, assign2270_e1933_d_n6, assign2270_e1933_d_n7, assign2270_e1933_d_n8, assign2270_e1933_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2270_e1931: f64 = (locals.var_temp / locals.var_wen);
        (assign2270_e1931, (locals.var_temp_dn4 / locals.var_wen), (locals.var_temp_dn6 / locals.var_wen), (locals.var_temp_dn7 / locals.var_wen), (locals.var_temp_dn8 / locals.var_wen), (locals.var_temp_dn9 / locals.var_wen),)
    } else {
        (locals.var_wphy, locals.var_wphy_dn4, locals.var_wphy_dn6, locals.var_wphy_dn7, locals.var_wphy_dn8, locals.var_wphy_dn9,)
    }
};
        locals.var_wphy = assign2270_e1933;
        locals.var_wphy_dn4 = assign2270_e1933_d_n4;
        locals.var_wphy_dn6 = assign2270_e1933_d_n6;
        locals.var_wphy_dn7 = assign2270_e1933_d_n7;
        locals.var_wphy_dn8 = assign2270_e1933_d_n8;
        locals.var_wphy_dn9 = assign2270_e1933_d_n9;

        let (assign2320_e1976,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p197,)
    } else {
        (locals.var_tox1_i,)
    }
};
        locals.var_tox1_i = assign2320_e1976;

        let (assign2330_e1981,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p198,)
    } else {
        (locals.var_tsi_i,)
    }
};
        locals.var_tsi_i = assign2330_e1981;

        let (assign2340_e1986,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p199,)
    } else {
        (locals.var_xge_i,)
    }
};
        locals.var_xge_i = assign2340_e1986;

        let (assign2350_e1991,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p200,)
    } else {
        (locals.var_tox2_i,)
    }
};
        locals.var_tox2_i = assign2350_e1991;

        let (assign2360_e1996,) = {
    if (locals.var_guard83 == 0.0) {
        (1.0,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign2360_e1996;

        let assign2370_e1999: f64 = if p.p201 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign2370_e1999;

        let (assign2380_e2007,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard94 != 0.0)) {
        let assign2380_e2005: f64 = (-1.0);
        (assign2380_e2005,)
    } else {
        (locals.var_typech_i,)
    }
};
        locals.var_typech_i = assign2380_e2007;

        let (assign2390_e2017,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2390_e2011: f64 = (p.p201).abs();
        let assign2390_e2013: f64 = (assign2390_e2011).min(1e19);
        let assign2390_e2015: f64 = (assign2390_e2013 * 1000000.0);
        (assign2390_e2015,)
    } else {
        (locals.var_nch_i,)
    }
};
        locals.var_nch_i = assign2390_e2017;

        let (assign2400_e2022,) = {
    if (locals.var_guard83 == 0.0) {
        (1.0,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign2400_e2022;

        let assign2410_e2025: f64 = if p.p202 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign2410_e2025;

        let (assign2420_e2033,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard95 != 0.0)) {
        let assign2420_e2031: f64 = (-1.0);
        (assign2420_e2031,)
    } else {
        (locals.var_typesub_i,)
    }
};
        locals.var_typesub_i = assign2420_e2033;

        let (assign2430_e2045,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2430_e2037: f64 = (p.p202).abs();
        let assign2430_e2039: f64 = (assign2430_e2037).max(1e16);
        let assign2430_e2041: f64 = (assign2430_e2039).min(1e21);
        let assign2430_e2043: f64 = (assign2430_e2041 * 1000000.0);
        (assign2430_e2043,)
    } else {
        (locals.var_nsub_i,)
    }
};
        locals.var_nsub_i = assign2430_e2045;

        let (assign2440_e2050,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p203,)
    } else {
        (locals.var_ct_i,)
    }
};
        locals.var_ct_i = assign2440_e2050;

        let (assign2450_e2055,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p204,)
    } else {
        (locals.var_toxp_i,)
    }
};
        locals.var_toxp_i = assign2450_e2055;

        let (assign2460_e2062,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2460_e2060: f64 = (p.p205 * 1000000.0);
        (assign2460_e2060,)
    } else {
        (locals.var_nov_i,)
    }
};
        locals.var_nov_i = assign2460_e2062;

        let (assign2470_e2069,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2470_e2067: f64 = (p.p206 * 1000000.0);
        (assign2470_e2067,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign2470_e2069;

        let (assign2480_e2086, assign2480_e2086_d_n4, assign2480_e2086_d_n6, assign2480_e2086_d_n7, assign2480_e2086_d_n8, assign2480_e2086_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2480_e2075: f64 = (locals.var_ile).powf(p.p209);
        let assign2480_e2076: f64 = (p.p208 * assign2480_e2075);
        let assign2480_e2081: f64 = (locals.var_ile).powf(p.p211);
        let assign2480_e2082: f64 = (p.p210 * assign2480_e2081);
        let assign2480_e2083: f64 = (1.0 + assign2480_e2082);
        let assign2480_e2084: f64 = (assign2480_e2076 / assign2480_e2083);
        (assign2480_e2084, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign2480_e2086;
        locals.var_temp_dn4 = assign2480_e2086_d_n4;
        locals.var_temp_dn6 = assign2480_e2086_d_n6;
        locals.var_temp_dn7 = assign2480_e2086_d_n7;
        locals.var_temp_dn8 = assign2480_e2086_d_n8;
        locals.var_temp_dn9 = assign2480_e2086_d_n9;

        let (assign2490_e2101, assign2490_e2101_d_n4, assign2490_e2101_d_n6, assign2490_e2101_d_n7, assign2490_e2101_d_n8, assign2490_e2101_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2490_e2091: f64 = (p.p207 + locals.var_temp);
        let assign2490_e2094: f64 = (p.p212 * locals.var_iwe);
        let assign2490_e2095: f64 = (assign2490_e2091 + assign2490_e2094);
        let assign2490_e2098: f64 = (p.p213 * locals.var_iae);
        let assign2490_e2099: f64 = (assign2490_e2095 + assign2490_e2098);
        (assign2490_e2099, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign2490_e2101;
        locals.var_vfb1_t_dn4 = assign2490_e2101_d_n4;
        locals.var_vfb1_t_dn6 = assign2490_e2101_d_n6;
        locals.var_vfb1_t_dn7 = assign2490_e2101_d_n7;
        locals.var_vfb1_t_dn8 = assign2490_e2101_d_n8;
        locals.var_vfb1_t_dn9 = assign2490_e2101_d_n9;

        let (assign2500_e2114, assign2500_e2114_d_n4, assign2500_e2114_d_n6, assign2500_e2114_d_n7, assign2500_e2114_d_n8, assign2500_e2114_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2500_e2107: f64 = (p.p215 * locals.var_tox2_i);
        let assign2500_e2109: f64 = (assign2500_e2107 / locals.var_tox1_i);
        let assign2500_e2111: f64 = (assign2500_e2109 * locals.var_temp);
        let assign2500_e2112: f64 = (p.p214 + assign2500_e2111);
        (assign2500_e2112, (assign2500_e2109 * locals.var_temp_dn4), (assign2500_e2109 * locals.var_temp_dn6), (assign2500_e2109 * locals.var_temp_dn7), (assign2500_e2109 * locals.var_temp_dn8), (assign2500_e2109 * locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign2500_e2114;
        locals.var_vfb2_t_dn4 = assign2500_e2114_d_n4;
        locals.var_vfb2_t_dn6 = assign2500_e2114_d_n6;
        locals.var_vfb2_t_dn7 = assign2500_e2114_d_n7;
        locals.var_vfb2_t_dn8 = assign2500_e2114_d_n8;
        locals.var_vfb2_t_dn9 = assign2500_e2114_d_n9;

        let (assign2510_e2137,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2510_e2121: f64 = (p.p217 * locals.var_ile);
        let assign2510_e2122: f64 = (1.0 + assign2510_e2121);
        let assign2510_e2123: f64 = (p.p216 * assign2510_e2122);
        let assign2510_e2127: f64 = (p.p218 * locals.var_iwe);
        let assign2510_e2128: f64 = (1.0 + assign2510_e2127);
        let assign2510_e2129: f64 = (assign2510_e2123 * assign2510_e2128);
        let assign2510_e2133: f64 = (p.p219 * locals.var_iae);
        let assign2510_e2134: f64 = (1.0 + assign2510_e2133);
        let assign2510_e2135: f64 = (assign2510_e2129 * assign2510_e2134);
        (assign2510_e2135,)
    } else {
        (locals.var_stvfb_i,)
    }
};
        locals.var_stvfb_i = assign2510_e2137;

        let (assign2520_e2150, assign2520_e2150_d_n4, assign2520_e2150_d_n6, assign2520_e2150_d_n7, assign2520_e2150_d_n8, assign2520_e2150_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2520_e2144: f64 = (p.p221 * locals.var_ile);
        let assign2520_e2145: f64 = (1.0 + assign2520_e2144);
        let assign2520_e2146: f64 = (p.p220 * assign2520_e2145);
        let assign2520_e2148: f64 = (assign2520_e2146 * 1000000.0);
        (assign2520_e2148, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9,)
    }
};
        locals.var_temp0__blk79 = assign2520_e2150;
        locals.var_temp0__blk79_dn4 = assign2520_e2150_d_n4;
        locals.var_temp0__blk79_dn6 = assign2520_e2150_d_n6;
        locals.var_temp0__blk79_dn7 = assign2520_e2150_d_n7;
        locals.var_temp0__blk79_dn8 = assign2520_e2150_d_n8;
        locals.var_temp0__blk79_dn9 = assign2520_e2150_d_n9;

        let (assign2530_e2159, assign2530_e2159_d_n4, assign2530_e2159_d_n6, assign2530_e2159_d_n7, assign2530_e2159_d_n8, assign2530_e2159_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2530_e2155: f64 = (locals.var_temp0__blk79).max(1e25);
        let assign2530_e2157: f64 = (assign2530_e2155).min(1e28);
        (assign2530_e2157, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn4 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn6 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn7 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn8 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn9 } else { 0.0 } } else { 0.0 },)
    } else {
        (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9,)
    }
};
        locals.var_np_i = assign2530_e2159;
        locals.var_np_i_dn4 = assign2530_e2159_d_n4;
        locals.var_np_i_dn6 = assign2530_e2159_d_n6;
        locals.var_np_i_dn7 = assign2530_e2159_d_n7;
        locals.var_np_i_dn8 = assign2530_e2159_d_n8;
        locals.var_np_i_dn9 = assign2530_e2159_d_n9;

        let (assign2540_e2164,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p222,)
    } else {
        (locals.var_cic1_i,)
    }
};
        locals.var_cic1_i = assign2540_e2164;

        let (assign2550_e2169,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p223,)
    } else {
        (locals.var_cic2_i,)
    }
};
        locals.var_cic2_i = assign2550_e2169;

        let (assign2560_e2176,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2560_e2174: f64 = (1.0 - locals.var_xge_i);
        (assign2560_e2174,)
    } else {
        (locals.var_one_m_xge,)
    }
};
        locals.var_one_m_xge = assign2560_e2176;

        let (assign2570_e2187,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2570_e2181: f64 = (1.04479e-10 * locals.var_one_m_xge);
        let assign2570_e2184: f64 = (1.43438e-10 * locals.var_xge_i);
        let assign2570_e2185: f64 = (assign2570_e2181 + assign2570_e2184);
        (assign2570_e2185,)
    } else {
        (locals.var_epsch,)
    }
};
        locals.var_epsch = assign2570_e2187;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2580_e2203,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2580_e2192: f64 = (locals.var_epsch / 3.45313e-11);
        let assign2580_e2194: f64 = (assign2580_e2192 * locals.var_tsi_i);
        let assign2580_e2197: f64 = (locals.var_tox1_i + 4e-10);
        let assign2580_e2198: f64 = (assign2580_e2194 * assign2580_e2197);
        let assign2580_e2199: f64 = (assign2580_e2198).sqrt();
        let assign2580_e2201: f64 = (assign2580_e2199 / locals.var_le);
        (assign2580_e2201,)
    } else {
        (locals.var_lambda_le,)
    }
};
        locals.var_lambda_le = assign2580_e2203;

        let (assign2590_e2220,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2590_e2208: f64 = (p.p224 * 2.0);
        let assign2590_e2211: f64 = (locals.var_lambda_le).powf(p.p225);
        let assign2590_e2212: f64 = (assign2590_e2208 * assign2590_e2211);
        let assign2590_e2216: f64 = (p.p226 * locals.var_iwe);
        let assign2590_e2217: f64 = (1.0 + assign2590_e2216);
        let assign2590_e2218: f64 = (assign2590_e2212 * assign2590_e2217);
        (assign2590_e2218,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign2590_e2220;

        let (assign2600_e2229,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2600_e2225: f64 = (locals.var_psce_p).max(0.0);
        let assign2600_e2227: f64 = (assign2600_e2225).min(5.0);
        (assign2600_e2227,)
    } else {
        (locals.var_psce1_i,)
    }
};
        locals.var_psce1_i = assign2600_e2229;

        let (assign2610_e2240,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2610_e2234: f64 = (p.p227 * locals.var_psce1_i);
        let assign2610_e2236: f64 = (assign2610_e2234 * locals.var_tox2_i);
        let assign2610_e2238: f64 = (assign2610_e2236 / locals.var_tox1_i);
        (assign2610_e2238,)
    } else {
        (locals.var_psce2_i,)
    }
};
        locals.var_psce2_i = assign2610_e2240;

        let (assign2620_e2247,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2620_e2245: f64 = (p.p228 * 1000000.0);
        (assign2620_e2245,)
    } else {
        (locals.var_nsddc_i,)
    }
};
        locals.var_nsddc_i = assign2620_e2247;

        let (assign2630_e2252,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p229,)
    } else {
        (locals.var_pscedlb_i,)
    }
};
        locals.var_pscedlb_i = assign2630_e2252;

        let (assign2640_e2259,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2640_e2257: f64 = (p.p230 * locals.var_iwe);
        (assign2640_e2257,)
    } else {
        (locals.var_pnce_p,)
    }
};
        locals.var_pnce_p = assign2640_e2259;

        let (assign2650_e2269,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2650_e2264: f64 = (-1.0);
        let assign2650_e2265: f64 = (locals.var_pnce_p).max(assign2650_e2264);
        let assign2650_e2267: f64 = (assign2650_e2265).min(1.0);
        (assign2650_e2267,)
    } else {
        (locals.var_pnce_i,)
    }
};
        locals.var_pnce_i = assign2650_e2269;

        let (assign2660_e2282, assign2660_e2282_d_n4, assign2660_e2282_d_n6, assign2660_e2282_d_n7, assign2660_e2282_d_n8, assign2660_e2282_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2660_e2274: f64 = (locals.var_lambda_le).powf(p.p232);
        let assign2660_e2278: f64 = (p.p233 * locals.var_iwe);
        let assign2660_e2279: f64 = (1.0 + assign2660_e2278);
        let assign2660_e2280: f64 = (assign2660_e2274 * assign2660_e2279);
        (assign2660_e2280, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign2660_e2282;
        locals.var_temp_dn4 = assign2660_e2282_d_n4;
        locals.var_temp_dn6 = assign2660_e2282_d_n6;
        locals.var_temp_dn7 = assign2660_e2282_d_n7;
        locals.var_temp_dn8 = assign2660_e2282_d_n8;
        locals.var_temp_dn9 = assign2660_e2282_d_n9;

        let (assign2670_e2289, assign2670_e2289_d_n4, assign2670_e2289_d_n6, assign2670_e2289_d_n7, assign2670_e2289_d_n8, assign2670_e2289_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2670_e2287: f64 = (p.p231 * locals.var_temp);
        (assign2670_e2287, (p.p231 * locals.var_temp_dn4), (p.p231 * locals.var_temp_dn6), (p.p231 * locals.var_temp_dn7), (p.p231 * locals.var_temp_dn8), (p.p231 * locals.var_temp_dn9),)
    } else {
        (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9,)
    }
};
        locals.var_cf_p = assign2670_e2289;
        locals.var_cf_p_dn4 = assign2670_e2289_d_n4;
        locals.var_cf_p_dn6 = assign2670_e2289_d_n6;
        locals.var_cf_p_dn7 = assign2670_e2289_d_n7;
        locals.var_cf_p_dn8 = assign2670_e2289_d_n8;
        locals.var_cf_p_dn9 = assign2670_e2289_d_n9;

        let (assign2680_e2296, assign2680_e2296_d_n4, assign2680_e2296_d_n6, assign2680_e2296_d_n7, assign2680_e2296_d_n8, assign2680_e2296_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2680_e2294: f64 = (locals.var_cf_p).max(0.0);
        (assign2680_e2294, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign2680_e2296;
        locals.var_cf1_t_dn4 = assign2680_e2296_d_n4;
        locals.var_cf1_t_dn6 = assign2680_e2296_d_n6;
        locals.var_cf1_t_dn7 = assign2680_e2296_d_n7;
        locals.var_cf1_t_dn8 = assign2680_e2296_d_n8;
        locals.var_cf1_t_dn9 = assign2680_e2296_d_n9;

        let (assign2690_e2307, assign2690_e2307_d_n4, assign2690_e2307_d_n6, assign2690_e2307_d_n7, assign2690_e2307_d_n8, assign2690_e2307_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2690_e2301: f64 = (p.p234 * locals.var_cf1_t);
        let assign2690_e2303: f64 = (assign2690_e2301 * locals.var_tox2_i);
        let assign2690_e2305: f64 = (assign2690_e2303 / locals.var_tox1_i);
        (assign2690_e2305, (((p.p234 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign2690_e2307;
        locals.var_cf2_t_dn4 = assign2690_e2307_d_n4;
        locals.var_cf2_t_dn6 = assign2690_e2307_d_n6;
        locals.var_cf2_t_dn7 = assign2690_e2307_d_n7;
        locals.var_cf2_t_dn8 = assign2690_e2307_d_n8;
        locals.var_cf2_t_dn9 = assign2690_e2307_d_n9;

        let (assign2700_e2314, assign2700_e2314_d_n4, assign2700_e2314_d_n6, assign2700_e2314_d_n7, assign2700_e2314_d_n8, assign2700_e2314_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2700_e2312: f64 = (p.p235 * locals.var_temp);
        (assign2700_e2312, (p.p235 * locals.var_temp_dn4), (p.p235 * locals.var_temp_dn6), (p.p235 * locals.var_temp_dn7), (p.p235 * locals.var_temp_dn8), (p.p235 * locals.var_temp_dn9),)
    } else {
        (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9,)
    }
};
        locals.var_stcf_i = assign2700_e2314;
        locals.var_stcf_i_dn4 = assign2700_e2314_d_n4;
        locals.var_stcf_i_dn6 = assign2700_e2314_d_n6;
        locals.var_stcf_i_dn7 = assign2700_e2314_d_n7;
        locals.var_stcf_i_dn8 = assign2700_e2314_d_n8;
        locals.var_stcf_i_dn9 = assign2700_e2314_d_n9;

        let (assign2710_e2319,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p236,)
    } else {
        (locals.var_cfd_i,)
    }
};
        locals.var_cfd_i = assign2710_e2319;

        let (assign2720_e2334,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2720_e2324: f64 = (p.p237 * locals.var_ile);
        let assign2720_e2328: f64 = (p.p238 * locals.var_iwe);
        let assign2720_e2329: f64 = (1.0 + assign2720_e2328);
        let assign2720_e2331: f64 = (assign2720_e2329).max(0.001);
        let assign2720_e2332: f64 = (assign2720_e2324 / assign2720_e2331);
        (assign2720_e2332,)
    } else {
        (locals.var_cfdl_i,)
    }
};
        locals.var_cfdl_i = assign2720_e2334;

        let (assign2730_e2339,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p239,)
    } else {
        (locals.var_cfdlb_i,)
    }
};
        locals.var_cfdlb_i = assign2730_e2339;

        let (assign2740_e2355, assign2740_e2355_d_n4, assign2740_e2355_d_n6, assign2740_e2355_d_n7, assign2740_e2355_d_n8, assign2740_e2355_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2740_e2343: f64 = (-locals.var_le);
        let assign2740_e2348: f64 = (p.p244 * locals.var_iwe);
        let assign2740_e2349: f64 = (1.0 + assign2740_e2348);
        let assign2740_e2351: f64 = (assign2740_e2349).max(0.001);
        let assign2740_e2352: f64 = (p.p243 * assign2740_e2351);
        let assign2740_e2353: f64 = (assign2740_e2343 / assign2740_e2352);
        (assign2740_e2353, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign2740_e2355;
        locals.var_temp1_dn4 = assign2740_e2355_d_n4;
        locals.var_temp1_dn6 = assign2740_e2355_d_n6;
        locals.var_temp1_dn7 = assign2740_e2355_d_n7;
        locals.var_temp1_dn8 = assign2740_e2355_d_n8;
        locals.var_temp1_dn9 = assign2740_e2355_d_n9;

        let assign2750_e2358: f64 = (-80.0);
        let assign2750_e2359: f64 = if locals.var_temp1 > assign2750_e2358 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign2750_e2359;

        let (assign2760_e2367, assign2760_e2367_d_n4, assign2760_e2367_d_n6, assign2760_e2367_d_n7, assign2760_e2367_d_n8, assign2760_e2367_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard96 != 0.0)) {
        let assign2760_e2365: f64 = (locals.var_temp1).exp();
        (assign2760_e2365, (assign2760_e2365 * locals.var_temp1_dn4), (assign2760_e2365 * locals.var_temp1_dn6), (assign2760_e2365 * locals.var_temp1_dn7), (assign2760_e2365 * locals.var_temp1_dn8), (assign2760_e2365 * locals.var_temp1_dn9),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign2760_e2367;
        locals.var_temp2_dn4 = assign2760_e2367_d_n4;
        locals.var_temp2_dn6 = assign2760_e2367_d_n6;
        locals.var_temp2_dn7 = assign2760_e2367_d_n7;
        locals.var_temp2_dn8 = assign2760_e2367_d_n8;
        locals.var_temp2_dn9 = assign2760_e2367_d_n9;

        let (assign2770_e2400, assign2770_e2400_d_n4, assign2770_e2400_d_n6, assign2770_e2400_d_n7, assign2770_e2400_d_n8, assign2770_e2400_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard96 == 0.0)) {
        let assign2770_e2376: f64 = (-locals.var_temp1);
        let assign2770_e2378: f64 = (assign2770_e2376 - 80.0);
        let assign2770_e2382: f64 = (-locals.var_temp1);
        let assign2770_e2384: f64 = (assign2770_e2382 - 80.0);
        let assign2770_e2385: f64 = (0.5 * assign2770_e2384);
        let assign2770_e2388: f64 = (-locals.var_temp1);
        let assign2770_e2390: f64 = (assign2770_e2388 - 80.0);
        let assign2770_e2392: f64 = (assign2770_e2390 * 0.3333333333333);
        let assign2770_e2393: f64 = (1.0 + assign2770_e2392);
        let assign2770_e2394: f64 = (assign2770_e2385 * assign2770_e2393);
        let assign2770_e2395: f64 = (1.0 + assign2770_e2394);
        let assign2770_e2396: f64 = (assign2770_e2378 * assign2770_e2395);
        let assign2770_e2397: f64 = (1.0 + assign2770_e2396);
        let assign2770_e2398: f64 = (1.80485e-35 / assign2770_e2397);
        (assign2770_e2398, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn4)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn6)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn7)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn8)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn9)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign2770_e2400;
        locals.var_temp2_dn4 = assign2770_e2400_d_n4;
        locals.var_temp2_dn6 = assign2770_e2400_d_n6;
        locals.var_temp2_dn7 = assign2770_e2400_d_n7;
        locals.var_temp2_dn8 = assign2770_e2400_d_n8;
        locals.var_temp2_dn9 = assign2770_e2400_d_n9;

        let (assign2780_e2408, assign2780_e2408_d_n4, assign2780_e2408_d_n6, assign2780_e2408_d_n7, assign2780_e2408_d_n8, assign2780_e2408_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2780_e2404: f64 = (-locals.var_le);
        let assign2780_e2406: f64 = (assign2780_e2404 / p.p246);
        (assign2780_e2406, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign2780_e2408;
        locals.var_temp3_dn4 = assign2780_e2408_d_n4;
        locals.var_temp3_dn6 = assign2780_e2408_d_n6;
        locals.var_temp3_dn7 = assign2780_e2408_d_n7;
        locals.var_temp3_dn8 = assign2780_e2408_d_n8;
        locals.var_temp3_dn9 = assign2780_e2408_d_n9;

        let assign2790_e2411: f64 = (-80.0);
        let assign2790_e2412: f64 = if locals.var_temp3 > assign2790_e2411 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign2790_e2412;

        let (assign2800_e2420, assign2800_e2420_d_n4, assign2800_e2420_d_n6, assign2800_e2420_d_n7, assign2800_e2420_d_n8, assign2800_e2420_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard97 != 0.0)) {
        let assign2800_e2418: f64 = (locals.var_temp3).exp();
        (assign2800_e2418, (assign2800_e2418 * locals.var_temp3_dn4), (assign2800_e2418 * locals.var_temp3_dn6), (assign2800_e2418 * locals.var_temp3_dn7), (assign2800_e2418 * locals.var_temp3_dn8), (assign2800_e2418 * locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign2800_e2420;
        locals.var_temp4_dn4 = assign2800_e2420_d_n4;
        locals.var_temp4_dn6 = assign2800_e2420_d_n6;
        locals.var_temp4_dn7 = assign2800_e2420_d_n7;
        locals.var_temp4_dn8 = assign2800_e2420_d_n8;
        locals.var_temp4_dn9 = assign2800_e2420_d_n9;

        let (assign2810_e2453, assign2810_e2453_d_n4, assign2810_e2453_d_n6, assign2810_e2453_d_n7, assign2810_e2453_d_n8, assign2810_e2453_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard97 == 0.0)) {
        let assign2810_e2429: f64 = (-locals.var_temp3);
        let assign2810_e2431: f64 = (assign2810_e2429 - 80.0);
        let assign2810_e2435: f64 = (-locals.var_temp3);
        let assign2810_e2437: f64 = (assign2810_e2435 - 80.0);
        let assign2810_e2438: f64 = (0.5 * assign2810_e2437);
        let assign2810_e2441: f64 = (-locals.var_temp3);
        let assign2810_e2443: f64 = (assign2810_e2441 - 80.0);
        let assign2810_e2445: f64 = (assign2810_e2443 * 0.3333333333333);
        let assign2810_e2446: f64 = (1.0 + assign2810_e2445);
        let assign2810_e2447: f64 = (assign2810_e2438 * assign2810_e2446);
        let assign2810_e2448: f64 = (1.0 + assign2810_e2447);
        let assign2810_e2449: f64 = (assign2810_e2431 * assign2810_e2448);
        let assign2810_e2450: f64 = (1.0 + assign2810_e2449);
        let assign2810_e2451: f64 = (1.80485e-35 / assign2810_e2450);
        (assign2810_e2451, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn4)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn6)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn7)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn8)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn9)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign2810_e2453;
        locals.var_temp4_dn4 = assign2810_e2453_d_n4;
        locals.var_temp4_dn6 = assign2810_e2453_d_n6;
        locals.var_temp4_dn7 = assign2810_e2453_d_n7;
        locals.var_temp4_dn8 = assign2810_e2453_d_n8;
        locals.var_temp4_dn9 = assign2810_e2453_d_n9;

        let (assign2820_e2482, assign2820_e2482_d_n4, assign2820_e2482_d_n6, assign2820_e2482_d_n7, assign2820_e2482_d_n8, assign2820_e2482_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2820_e2461: f64 = (p.p242 * locals.var_iwe);
        let assign2820_e2462: f64 = (1.0 + assign2820_e2461);
        let assign2820_e2463: f64 = (p.p241 * assign2820_e2462);
        let assign2820_e2466: f64 = (locals.var_temp2 - 1.0);
        let assign2820_e2467: f64 = (assign2820_e2463 * assign2820_e2466);
        let assign2820_e2469: f64 = (assign2820_e2467 / locals.var_temp1);
        let assign2820_e2470: f64 = (1.0 + assign2820_e2469);
        let assign2820_e2474: f64 = (locals.var_temp4 - 1.0);
        let assign2820_e2475: f64 = (p.p245 * assign2820_e2474);
        let assign2820_e2477: f64 = (assign2820_e2475 / locals.var_temp3);
        let assign2820_e2478: f64 = (assign2820_e2470 + assign2820_e2477);
        let assign2820_e2480: f64 = (assign2820_e2478).max(1e-6);
        (assign2820_e2480, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn4) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn4) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn6) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn6) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn7) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn7) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn8) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn8) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn9) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn9) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 },)
    } else {
        (locals.var_gpe, locals.var_gpe_dn4, locals.var_gpe_dn6, locals.var_gpe_dn7, locals.var_gpe_dn8, locals.var_gpe_dn9,)
    }
};
        locals.var_gpe = assign2820_e2482;
        locals.var_gpe_dn4 = assign2820_e2482_d_n4;
        locals.var_gpe_dn6 = assign2820_e2482_d_n6;
        locals.var_gpe_dn7 = assign2820_e2482_d_n7;
        locals.var_gpe_dn8 = assign2820_e2482_d_n8;
        locals.var_gpe_dn9 = assign2820_e2482_d_n9;

        let (assign2830_e2504,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2830_e2488: f64 = (p.p247 * locals.var_iwe);
        let assign2830_e2489: f64 = (1.0 + assign2830_e2488);
        let assign2830_e2492: f64 = (p.p248 * locals.var_iwe);
        let assign2830_e2496: f64 = (locals.var_we / p.p249);
        let assign2830_e2497: f64 = (1.0 + assign2830_e2496);
        let assign2830_e2498: f64 = (assign2830_e2497).ln();
        let assign2830_e2499: f64 = (assign2830_e2492 * assign2830_e2498);
        let assign2830_e2500: f64 = (assign2830_e2489 + assign2830_e2499);
        let assign2830_e2502: f64 = (assign2830_e2500).max(1e-6);
        (assign2830_e2502,)
    } else {
        (locals.var_gwe,)
    }
};
        locals.var_gwe = assign2830_e2504;

        let (assign2840_e2513, assign2840_e2513_d_n4, assign2840_e2513_d_n6, assign2840_e2513_d_n7, assign2840_e2513_d_n8, assign2840_e2513_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2840_e2509: f64 = (p.p240 / locals.var_gpe);
        let assign2840_e2511: f64 = (assign2840_e2509 * locals.var_gwe);
        (assign2840_e2511, ((-((p.p240 * locals.var_gpe_dn4) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn6) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn7) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn8) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn9) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe),)
    } else {
        (locals.var_ge, locals.var_ge_dn4, locals.var_ge_dn6, locals.var_ge_dn7, locals.var_ge_dn8, locals.var_ge_dn9,)
    }
};
        locals.var_ge = assign2840_e2513;
        locals.var_ge_dn4 = assign2840_e2513_d_n4;
        locals.var_ge_dn6 = assign2840_e2513_d_n6;
        locals.var_ge_dn7 = assign2840_e2513_d_n7;
        locals.var_ge_dn8 = assign2840_e2513_d_n8;
        locals.var_ge_dn9 = assign2840_e2513_d_n9;

        let (assign2850_e2522, assign2850_e2522_d_n4, assign2850_e2522_d_n6, assign2850_e2522_d_n7, assign2850_e2522_d_n8, assign2850_e2522_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2850_e2518: f64 = (locals.var_ge * locals.var_we);
        let assign2850_e2520: f64 = (assign2850_e2518 / locals.var_le);
        (assign2850_e2520, ((locals.var_ge_dn4 * locals.var_we) / locals.var_le), ((locals.var_ge_dn6 * locals.var_we) / locals.var_le), ((locals.var_ge_dn7 * locals.var_we) / locals.var_le), ((locals.var_ge_dn8 * locals.var_we) / locals.var_le), ((locals.var_ge_dn9 * locals.var_we) / locals.var_le),)
    } else {
        (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9,)
    }
};
        locals.var_betn_p = assign2850_e2522;
        locals.var_betn_p_dn4 = assign2850_e2522_d_n4;
        locals.var_betn_p_dn6 = assign2850_e2522_d_n6;
        locals.var_betn_p_dn7 = assign2850_e2522_d_n7;
        locals.var_betn_p_dn8 = assign2850_e2522_d_n8;
        locals.var_betn_p_dn9 = assign2850_e2522_d_n9;

        let (assign2860_e2529, assign2860_e2529_d_n4, assign2860_e2529_d_n6, assign2860_e2529_d_n7, assign2860_e2529_d_n8, assign2860_e2529_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2860_e2527: f64 = (locals.var_betn_p).max(1e-10);
        (assign2860_e2527, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign2860_e2529;
        locals.var_betn1_t_dn4 = assign2860_e2529_d_n4;
        locals.var_betn1_t_dn6 = assign2860_e2529_d_n6;
        locals.var_betn1_t_dn7 = assign2860_e2529_d_n7;
        locals.var_betn1_t_dn8 = assign2860_e2529_d_n8;
        locals.var_betn1_t_dn9 = assign2860_e2529_d_n9;

        let (assign2870_e2536, assign2870_e2536_d_n4, assign2870_e2536_d_n6, assign2870_e2536_d_n7, assign2870_e2536_d_n8, assign2870_e2536_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2870_e2534: f64 = (p.p250 * locals.var_betn1_t);
        (assign2870_e2534, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign2870_e2536;
        locals.var_betn2_t_dn4 = assign2870_e2536_d_n4;
        locals.var_betn2_t_dn6 = assign2870_e2536_d_n6;
        locals.var_betn2_t_dn7 = assign2870_e2536_d_n7;
        locals.var_betn2_t_dn8 = assign2870_e2536_d_n8;
        locals.var_betn2_t_dn9 = assign2870_e2536_d_n9;

        let (assign2880_e2559,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2880_e2543: f64 = (p.p252 * locals.var_ile);
        let assign2880_e2544: f64 = (1.0 + assign2880_e2543);
        let assign2880_e2545: f64 = (p.p251 * assign2880_e2544);
        let assign2880_e2549: f64 = (p.p253 * locals.var_iwe);
        let assign2880_e2550: f64 = (1.0 + assign2880_e2549);
        let assign2880_e2551: f64 = (assign2880_e2545 * assign2880_e2550);
        let assign2880_e2555: f64 = (p.p254 * locals.var_iae);
        let assign2880_e2556: f64 = (1.0 + assign2880_e2555);
        let assign2880_e2557: f64 = (assign2880_e2551 * assign2880_e2556);
        (assign2880_e2557,)
    } else {
        (locals.var_stbet_i,)
    }
};
        locals.var_stbet_i = assign2880_e2559;

        let (assign2890_e2582,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2890_e2566: f64 = (locals.var_ile).powf(p.p257);
        let assign2890_e2567: f64 = (p.p256 * assign2890_e2566);
        let assign2890_e2568: f64 = (p.p255 + assign2890_e2567);
        let assign2890_e2572: f64 = (p.p258 * locals.var_iwe);
        let assign2890_e2573: f64 = (1.0 + assign2890_e2572);
        let assign2890_e2574: f64 = (assign2890_e2568 * assign2890_e2573);
        let assign2890_e2578: f64 = (p.p259 * locals.var_iae);
        let assign2890_e2579: f64 = (1.0 + assign2890_e2578);
        let assign2890_e2580: f64 = (assign2890_e2574 * assign2890_e2579);
        (assign2890_e2580,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign2890_e2582;

        let (assign2900_e2589,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2900_e2587: f64 = (locals.var_cs_p).max(0.0);
        (assign2900_e2587,)
    } else {
        (locals.var_cs_t,)
    }
};
        locals.var_cs_t = assign2900_e2589;

        let (assign2910_e2594,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p260,)
    } else {
        (locals.var_csfi_i,)
    }
};
        locals.var_csfi_i = assign2910_e2594;

        let (assign2920_e2599,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p261,)
    } else {
        (locals.var_csbi_i,)
    }
};
        locals.var_csbi_i = assign2920_e2599;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2930_e2622,) = {
    if (locals.var_guard83 == 0.0) {
        let assign2930_e2606: f64 = (p.p263 * locals.var_ile);
        let assign2930_e2607: f64 = (1.0 + assign2930_e2606);
        let assign2930_e2608: f64 = (p.p262 * assign2930_e2607);
        let assign2930_e2612: f64 = (p.p264 * locals.var_iwe);
        let assign2930_e2613: f64 = (1.0 + assign2930_e2612);
        let assign2930_e2614: f64 = (assign2930_e2608 * assign2930_e2613);
        let assign2930_e2618: f64 = (p.p265 * locals.var_iae);
        let assign2930_e2619: f64 = (1.0 + assign2930_e2618);
        let assign2930_e2620: f64 = (assign2930_e2614 * assign2930_e2619);
        (assign2930_e2620,)
    } else {
        (locals.var_stcs_i,)
    }
};
        locals.var_stcs_i = assign2930_e2622;

        let (assign2940_e2627,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p266,)
    } else {
        (locals.var_thecs_t,)
    }
};
        locals.var_thecs_t = assign2940_e2627;

        let (assign2950_e2632,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p267,)
    } else {
        (locals.var_stthecs_i,)
    }
};
        locals.var_stthecs_i = assign2950_e2632;

        let (assign2960_e2637,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p268,)
    } else {
        (locals.var_csthr_i,)
    }
};
        locals.var_csthr_i = assign2960_e2637;

        let (assign2970_e2642,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p269,)
    } else {
        (locals.var_csthrb_i,)
    }
};
        locals.var_csthrb_i = assign2970_e2642;

        let (assign2980_e2647,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p270,)
    } else {
        (locals.var_mue_t,)
    }
};
        locals.var_mue_t = assign2980_e2647;

        let (assign2990_e2652,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p271,)
    } else {
        (locals.var_stmue_i,)
    }
};
        locals.var_stmue_i = assign2990_e2652;

        let (assign3000_e2657,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p272,)
    } else {
        (locals.var_themu_t,)
    }
};
        locals.var_themu_t = assign3000_e2657;

        let (assign3010_e2662,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p273,)
    } else {
        (locals.var_stthemu_i,)
    }
};
        locals.var_stthemu_i = assign3010_e2662;

        let (assign3020_e2685,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3020_e2669: f64 = (locals.var_ile).powf(p.p276);
        let assign3020_e2670: f64 = (p.p275 * assign3020_e2669);
        let assign3020_e2671: f64 = (p.p274 + assign3020_e2670);
        let assign3020_e2675: f64 = (p.p277 * locals.var_iwe);
        let assign3020_e2676: f64 = (1.0 + assign3020_e2675);
        let assign3020_e2677: f64 = (assign3020_e2671 * assign3020_e2676);
        let assign3020_e2681: f64 = (p.p278 * locals.var_iae);
        let assign3020_e2682: f64 = (1.0 + assign3020_e2681);
        let assign3020_e2683: f64 = (assign3020_e2677 * assign3020_e2682);
        (assign3020_e2683,)
    } else {
        (locals.var_xcor_t,)
    }
};
        locals.var_xcor_t = assign3020_e2685;

        let (assign3030_e2690,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p279,)
    } else {
        (locals.var_xcorb_i,)
    }
};
        locals.var_xcorb_i = assign3030_e2690;

        let (assign3040_e2695,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p280,)
    } else {
        (locals.var_stxcor_i,)
    }
};
        locals.var_stxcor_i = assign3040_e2695;

        let (assign3050_e2700,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p281,)
    } else {
        (locals.var_feta_i,)
    }
};
        locals.var_feta_i = assign3050_e2700;

        let (assign3060_e2713,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3060_e2705: f64 = (p.p282 * locals.var_iwe);
        let assign3060_e2709: f64 = (p.p283 * locals.var_iwe);
        let assign3060_e2710: f64 = (1.0 + assign3060_e2709);
        let assign3060_e2711: f64 = (assign3060_e2705 * assign3060_e2710);
        (assign3060_e2711,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign3060_e2713;

        let (assign3070_e2720,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3070_e2718: f64 = (locals.var_rs_p).max(0.0);
        (assign3070_e2718,)
    } else {
        (locals.var_rs_t,)
    }
};
        locals.var_rs_t = assign3070_e2720;

        let (assign3080_e2725,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p284,)
    } else {
        (locals.var_rsig_i,)
    }
};
        locals.var_rsig_i = assign3080_e2725;

        let (assign3090_e2730,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p285,)
    } else {
        (locals.var_strs_i,)
    }
};
        locals.var_strs_i = assign3090_e2730;

        let (assign3100_e2735,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p286,)
    } else {
        (locals.var_rsg_i,)
    }
};
        locals.var_rsg_i = assign3100_e2735;

        let (assign3110_e2740,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p287,)
    } else {
        (locals.var_thersg_i,)
    }
};
        locals.var_thersg_i = assign3110_e2740;

        let (assign3120_e2745,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p288,)
    } else {
        (locals.var_rsb_i,)
    }
};
        locals.var_rsb_i = assign3120_e2745;

        let (assign3130_e2770, assign3130_e2770_d_n4, assign3130_e2770_d_n6, assign3130_e2770_d_n7, assign3130_e2770_d_n8, assign3130_e2770_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3130_e2753: f64 = (locals.var_ile).powf(p.p291);
        let assign3130_e2754: f64 = (p.p290 * assign3130_e2753);
        let assign3130_e2755: f64 = (p.p289 + assign3130_e2754);
        let assign3130_e2756: f64 = (locals.var_ge * assign3130_e2755);
        let assign3130_e2760: f64 = (p.p292 * locals.var_iwe);
        let assign3130_e2761: f64 = (1.0 + assign3130_e2760);
        let assign3130_e2762: f64 = (assign3130_e2756 * assign3130_e2761);
        let assign3130_e2766: f64 = (p.p293 * locals.var_iae);
        let assign3130_e2767: f64 = (1.0 + assign3130_e2766);
        let assign3130_e2768: f64 = (assign3130_e2762 * assign3130_e2767);
        (assign3130_e2768, (((locals.var_ge_dn4 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn6 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn7 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn8 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn9 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767),)
    } else {
        (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9,)
    }
};
        locals.var_thesat_p = assign3130_e2770;
        locals.var_thesat_p_dn4 = assign3130_e2770_d_n4;
        locals.var_thesat_p_dn6 = assign3130_e2770_d_n6;
        locals.var_thesat_p_dn7 = assign3130_e2770_d_n7;
        locals.var_thesat_p_dn8 = assign3130_e2770_d_n8;
        locals.var_thesat_p_dn9 = assign3130_e2770_d_n9;

        let (assign3140_e2777, assign3140_e2777_d_n4, assign3140_e2777_d_n6, assign3140_e2777_d_n7, assign3140_e2777_d_n8, assign3140_e2777_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3140_e2775: f64 = (locals.var_thesat_p).max(0.0);
        (assign3140_e2775, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign3140_e2777;
        locals.var_thesat_t_dn4 = assign3140_e2777_d_n4;
        locals.var_thesat_t_dn6 = assign3140_e2777_d_n6;
        locals.var_thesat_t_dn7 = assign3140_e2777_d_n7;
        locals.var_thesat_t_dn8 = assign3140_e2777_d_n8;
        locals.var_thesat_t_dn9 = assign3140_e2777_d_n9;

        let (assign3150_e2800,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3150_e2784: f64 = (p.p295 * locals.var_ile);
        let assign3150_e2785: f64 = (1.0 + assign3150_e2784);
        let assign3150_e2786: f64 = (p.p294 * assign3150_e2785);
        let assign3150_e2790: f64 = (p.p296 * locals.var_iwe);
        let assign3150_e2791: f64 = (1.0 + assign3150_e2790);
        let assign3150_e2792: f64 = (assign3150_e2786 * assign3150_e2791);
        let assign3150_e2796: f64 = (p.p297 * locals.var_iae);
        let assign3150_e2797: f64 = (1.0 + assign3150_e2796);
        let assign3150_e2798: f64 = (assign3150_e2792 * assign3150_e2797);
        (assign3150_e2798,)
    } else {
        (locals.var_stthesat_i,)
    }
};
        locals.var_stthesat_i = assign3150_e2800;

        let (assign3160_e2805,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p298,)
    } else {
        (locals.var_thesat1_i,)
    }
};
        locals.var_thesat1_i = assign3160_e2805;

        let (assign3170_e2810,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p299,)
    } else {
        (locals.var_thesat2_i,)
    }
};
        locals.var_thesat2_i = assign3170_e2810;

        let (assign3180_e2831,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3180_e2818: f64 = (locals.var_ile).powf(p.p302);
        let assign3180_e2819: f64 = (p.p301 * assign3180_e2818);
        let assign3180_e2824: f64 = (locals.var_ile).powf(p.p304);
        let assign3180_e2825: f64 = (p.p303 * assign3180_e2824);
        let assign3180_e2826: f64 = (1.0 + assign3180_e2825);
        let assign3180_e2827: f64 = (assign3180_e2819 / assign3180_e2826);
        let assign3180_e2828: f64 = (1.0 + assign3180_e2827);
        let assign3180_e2829: f64 = (p.p300 / assign3180_e2828);
        (assign3180_e2829,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign3180_e2831;

        let (assign3190_e2840,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3190_e2836: f64 = (locals.var_ax_p).max(1.0);
        let assign3190_e2838: f64 = (assign3190_e2836).min(16.0);
        (assign3190_e2838,)
    } else {
        (locals.var_ax_i,)
    }
};
        locals.var_ax_i = assign3190_e2840;

        let (assign3200_e2863,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3200_e2846: f64 = (locals.var_ile).powf(p.p306);
        let assign3200_e2847: f64 = (p.p305 * assign3200_e2846);
        let assign3200_e2851: f64 = (p.p309 * locals.var_iwe);
        let assign3200_e2852: f64 = (1.0 + assign3200_e2851);
        let assign3200_e2853: f64 = (assign3200_e2847 * assign3200_e2852);
        let assign3200_e2858: f64 = (locals.var_ile).powf(p.p308);
        let assign3200_e2859: f64 = (p.p307 * assign3200_e2858);
        let assign3200_e2860: f64 = (1.0 + assign3200_e2859);
        let assign3200_e2861: f64 = (assign3200_e2853 / assign3200_e2860);
        (assign3200_e2861,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign3200_e2863;

        let (assign3210_e2870,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3210_e2868: f64 = (locals.var_alp_p).max(0.0);
        (assign3210_e2868,)
    } else {
        (locals.var_alp_i,)
    }
};
        locals.var_alp_i = assign3210_e2870;

        let (assign3220_e2893,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3220_e2876: f64 = (locals.var_ile).powf(p.p311);
        let assign3220_e2877: f64 = (p.p310 * assign3220_e2876);
        let assign3220_e2881: f64 = (p.p314 * locals.var_iwe);
        let assign3220_e2882: f64 = (1.0 + assign3220_e2881);
        let assign3220_e2883: f64 = (assign3220_e2877 * assign3220_e2882);
        let assign3220_e2888: f64 = (locals.var_ile).powf(p.p313);
        let assign3220_e2889: f64 = (p.p312 * assign3220_e2888);
        let assign3220_e2890: f64 = (1.0 + assign3220_e2889);
        let assign3220_e2891: f64 = (assign3220_e2883 / assign3220_e2890);
        (assign3220_e2891,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign3220_e2893;

        let (assign3230_e2900,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3230_e2898: f64 = (locals.var_alp1_p).max(0.0);
        (assign3230_e2898,)
    } else {
        (locals.var_alp1_i,)
    }
};
        locals.var_alp1_i = assign3230_e2900;

        let (assign3240_e2905,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p315,)
    } else {
        (locals.var_alpb_i,)
    }
};
        locals.var_alpb_i = assign3240_e2905;

        let (assign3250_e2910,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p316,)
    } else {
        (locals.var_vp_i,)
    }
};
        locals.var_vp_i = assign3250_e2910;

        let (assign3260_e2915,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p317,)
    } else {
        (locals.var_vpg_i,)
    }
};
        locals.var_vpg_i = assign3260_e2915;

        let (assign3270_e2920,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p318,)
    } else {
        (locals.var_gco_i,)
    }
};
        locals.var_gco_i = assign3270_e2920;

        let (assign3280_e2927,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3280_e2925: f64 = (p.p319 / locals.var_iae);
        (assign3280_e2925,)
    } else {
        (locals.var_iginv_t,)
    }
};
        locals.var_iginv_t = assign3280_e2927;

        let (assign3290_e2934,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3290_e2932: f64 = (p.p320 / locals.var_iwe);
        (assign3290_e2932,)
    } else {
        (locals.var_igovinv_t,)
    }
};
        locals.var_igovinv_t = assign3290_e2934;

        let (assign3300_e2941,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3300_e2939: f64 = (p.p321 / locals.var_iwe);
        (assign3300_e2939,)
    } else {
        (locals.var_igovinvd_t,)
    }
};
        locals.var_igovinvd_t = assign3300_e2941;

        let (assign3310_e2948,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3310_e2946: f64 = (p.p335 / locals.var_iwe);
        (assign3310_e2946,)
    } else {
        (locals.var_fnovinv_t,)
    }
};
        locals.var_fnovinv_t = assign3310_e2948;

        let (assign3320_e2955,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3320_e2953: f64 = (p.p336 / locals.var_iwe);
        (assign3320_e2953,)
    } else {
        (locals.var_fnovinvd_t,)
    }
};
        locals.var_fnovinvd_t = assign3320_e2955;

        let (assign3330_e2962,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3330_e2960: f64 = (p.p322 / locals.var_iwe);
        (assign3330_e2960,)
    } else {
        (locals.var_igovacc_t,)
    }
};
        locals.var_igovacc_t = assign3330_e2962;

        let (assign3340_e2969,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3340_e2967: f64 = (p.p323 / locals.var_iwe);
        (assign3340_e2967,)
    } else {
        (locals.var_igovaccd_t,)
    }
};
        locals.var_igovaccd_t = assign3340_e2969;

        let (assign3350_e2974,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p324,)
    } else {
        (locals.var_stig_i,)
    }
};
        locals.var_stig_i = assign3350_e2974;

        let (assign3360_e2979,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p338,)
    } else {
        (locals.var_stigfn_i,)
    }
};
        locals.var_stigfn_i = assign3360_e2979;

        let (assign3370_e2984,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p325,)
    } else {
        (locals.var_gc2ch_i,)
    }
};
        locals.var_gc2ch_i = assign3370_e2984;

        let (assign3380_e2989,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p326,)
    } else {
        (locals.var_gc3ch_i,)
    }
};
        locals.var_gc3ch_i = assign3380_e2989;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3390_e2994,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p327,)
    } else {
        (locals.var_gc2ovinv_i,)
    }
};
        locals.var_gc2ovinv_i = assign3390_e2994;

        let (assign3400_e2999,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p337,)
    } else {
        (locals.var_gcovinvfn_i,)
    }
};
        locals.var_gcovinvfn_i = assign3400_e2999;

        let (assign3410_e3004,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p328,)
    } else {
        (locals.var_gc3ovinv_i,)
    }
};
        locals.var_gc3ovinv_i = assign3410_e3004;

        let (assign3420_e3009,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p329,)
    } else {
        (locals.var_gc2ovacc_i,)
    }
};
        locals.var_gc2ovacc_i = assign3420_e3009;

        let (assign3430_e3014,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p330,)
    } else {
        (locals.var_gc3ovacc_i,)
    }
};
        locals.var_gc3ovacc_i = assign3430_e3014;

        let (assign3440_e3021,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3440_e3019: f64 = (p.p331 * locals.var_ile);
        (assign3440_e3019,)
    } else {
        (locals.var_gcdov_i,)
    }
};
        locals.var_gcdov_i = assign3440_e3021;

        let (assign3450_e3026,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p332,)
    } else {
        (locals.var_gcvdov_i,)
    }
};
        locals.var_gcvdov_i = assign3450_e3026;

        let (assign3460_e3031,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p333,)
    } else {
        (locals.var_chib_i,)
    }
};
        locals.var_chib_i = assign3460_e3031;

        let (assign3470_e3036,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p334,)
    } else {
        (locals.var_niginv_i,)
    }
};
        locals.var_niginv_i = assign3470_e3036;

        let (assign3480_e3045,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3480_e3042: f64 = (p.p341 / locals.var_iwe);
        let assign3480_e3043: f64 = (p.p339 + assign3480_e3042);
        (assign3480_e3043,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign3480_e3045;

        let (assign3490_e3052, assign3490_e3052_d_n4, assign3490_e3052_d_n6, assign3490_e3052_d_n7, assign3490_e3052_d_n8, assign3490_e3052_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3490_e3050: f64 = (locals.var_agidl_p).max(0.0);
        (assign3490_e3050, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9,)
    }
};
        locals.var_agidl_i = assign3490_e3052;
        locals.var_agidl_i_dn4 = assign3490_e3052_d_n4;
        locals.var_agidl_i_dn6 = assign3490_e3052_d_n6;
        locals.var_agidl_i_dn7 = assign3490_e3052_d_n7;
        locals.var_agidl_i_dn8 = assign3490_e3052_d_n8;
        locals.var_agidl_i_dn9 = assign3490_e3052_d_n9;

        let (assign3500_e3061,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3500_e3058: f64 = (p.p342 / locals.var_iwe);
        let assign3500_e3059: f64 = (p.p340 + assign3500_e3058);
        (assign3500_e3059,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign3500_e3061;

        let (assign3510_e3068, assign3510_e3068_d_n4, assign3510_e3068_d_n6, assign3510_e3068_d_n7, assign3510_e3068_d_n8, assign3510_e3068_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3510_e3066: f64 = (locals.var_agidld_p).max(0.0);
        (assign3510_e3066, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9,)
    }
};
        locals.var_agidld_i = assign3510_e3068;
        locals.var_agidld_i_dn4 = assign3510_e3068_d_n4;
        locals.var_agidld_i_dn6 = assign3510_e3068_d_n6;
        locals.var_agidld_i_dn7 = assign3510_e3068_d_n7;
        locals.var_agidld_i_dn8 = assign3510_e3068_d_n8;
        locals.var_agidld_i_dn9 = assign3510_e3068_d_n9;

        let (assign3520_e3073,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p343,)
    } else {
        (locals.var_bgidl_t,)
    }
};
        locals.var_bgidl_t = assign3520_e3073;

        let (assign3530_e3078,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p344,)
    } else {
        (locals.var_bgidld_t,)
    }
};
        locals.var_bgidld_t = assign3530_e3078;

        let (assign3540_e3083,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p345,)
    } else {
        (locals.var_stbgidl_i,)
    }
};
        locals.var_stbgidl_i = assign3540_e3083;

        let (assign3550_e3088,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p346,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign3550_e3088;

        let (assign3560_e3093,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p347,)
    } else {
        (locals.var_cgidl_i,)
    }
};
        locals.var_cgidl_i = assign3560_e3093;

        let (assign3570_e3098,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p348,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign3570_e3098;

        let (assign3580_e3107,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3580_e3104: f64 = (p.p351 * locals.var_ile);
        let assign3580_e3105: f64 = (p.p349 + assign3580_e3104);
        (assign3580_e3105,)
    } else {
        (locals.var_dgidl_i,)
    }
};
        locals.var_dgidl_i = assign3580_e3107;

        let (assign3590_e3116,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3590_e3113: f64 = (p.p352 * locals.var_ile);
        let assign3590_e3114: f64 = (p.p350 + assign3590_e3113);
        (assign3590_e3114,)
    } else {
        (locals.var_dgidld_i,)
    }
};
        locals.var_dgidld_i = assign3590_e3116;

        let (assign3600_e3133,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3600_e3123: f64 = (p.p385 * locals.var_ile);
        let assign3600_e3124: f64 = (1.0 + assign3600_e3123);
        let assign3600_e3125: f64 = (p.p384 * assign3600_e3124);
        let assign3600_e3129: f64 = (p.p386 * locals.var_iwe);
        let assign3600_e3130: f64 = (1.0 + assign3600_e3129);
        let assign3600_e3131: f64 = (assign3600_e3125 * assign3600_e3130);
        (assign3600_e3131,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign3600_e3133;

        let (assign3610_e3140,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3610_e3138: f64 = (locals.var_a1_p).max(0.0);
        (assign3610_e3138,)
    } else {
        (locals.var_a1_i,)
    }
};
        locals.var_a1_i = assign3610_e3140;

        let (assign3620_e3145,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p387,)
    } else {
        (locals.var_a2_t,)
    }
};
        locals.var_a2_t = assign3620_e3145;

        let (assign3630_e3150,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p388,)
    } else {
        (locals.var_sta2_i,)
    }
};
        locals.var_sta2_i = assign3630_e3150;

        let (assign3640_e3167,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3640_e3157: f64 = (p.p390 * locals.var_ile);
        let assign3640_e3158: f64 = (1.0 + assign3640_e3157);
        let assign3640_e3159: f64 = (p.p389 * assign3640_e3158);
        let assign3640_e3163: f64 = (p.p391 * locals.var_iwe);
        let assign3640_e3164: f64 = (1.0 + assign3640_e3163);
        let assign3640_e3165: f64 = (assign3640_e3159 * assign3640_e3164);
        (assign3640_e3165,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign3640_e3167;

        let (assign3650_e3174,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3650_e3172: f64 = (locals.var_a3_p).max(0.0);
        (assign3650_e3172,)
    } else {
        (locals.var_a3_i,)
    }
};
        locals.var_a3_i = assign3650_e3174;

        let (assign3660_e3185,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3660_e3179: f64 = (2.0 * p.p353);
        let assign3660_e3182: f64 = (p.p354 * locals.var_we);
        let assign3660_e3183: f64 = (assign3660_e3179 + assign3660_e3182);
        (assign3660_e3183,)
    } else {
        (locals.var_we_edge,)
    }
};
        locals.var_we_edge = assign3660_e3185;

        let (assign3670_e3190,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p355,)
    } else {
        (locals.var_ctedge_i,)
    }
};
        locals.var_ctedge_i = assign3670_e3190;

        let (assign3680_e3199, assign3680_e3199_d_n4, assign3680_e3199_d_n6, assign3680_e3199_d_n7, assign3680_e3199_d_n8, assign3680_e3199_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3680_e3196: f64 = (locals.var_ile).powf(p.p358);
        let assign3680_e3197: f64 = (p.p357 * assign3680_e3196);
        (assign3680_e3197, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3680_e3199;
        locals.var_temp_dn4 = assign3680_e3199_d_n4;
        locals.var_temp_dn6 = assign3680_e3199_d_n6;
        locals.var_temp_dn7 = assign3680_e3199_d_n7;
        locals.var_temp_dn8 = assign3680_e3199_d_n8;
        locals.var_temp_dn9 = assign3680_e3199_d_n9;

        let (assign3690_e3214, assign3690_e3214_d_n4, assign3690_e3214_d_n6, assign3690_e3214_d_n7, assign3690_e3214_d_n8, assign3690_e3214_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3690_e3204: f64 = (p.p356 + locals.var_temp);
        let assign3690_e3207: f64 = (p.p359 * locals.var_iwe);
        let assign3690_e3208: f64 = (assign3690_e3204 + assign3690_e3207);
        let assign3690_e3211: f64 = (p.p360 * locals.var_iae);
        let assign3690_e3212: f64 = (assign3690_e3208 + assign3690_e3211);
        (assign3690_e3212, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_vfb1edge_t, locals.var_vfb1edge_t_dn4, locals.var_vfb1edge_t_dn6, locals.var_vfb1edge_t_dn7, locals.var_vfb1edge_t_dn8, locals.var_vfb1edge_t_dn9,)
    }
};
        locals.var_vfb1edge_t = assign3690_e3214;
        locals.var_vfb1edge_t_dn4 = assign3690_e3214_d_n4;
        locals.var_vfb1edge_t_dn6 = assign3690_e3214_d_n6;
        locals.var_vfb1edge_t_dn7 = assign3690_e3214_d_n7;
        locals.var_vfb1edge_t_dn8 = assign3690_e3214_d_n8;
        locals.var_vfb1edge_t_dn9 = assign3690_e3214_d_n9;

        let (assign3700_e3219,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p361,)
    } else {
        (locals.var_vfb2edge_t,)
    }
};
        locals.var_vfb2edge_t = assign3700_e3219;

        let (assign3710_e3242,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3710_e3226: f64 = (p.p363 * locals.var_ile);
        let assign3710_e3227: f64 = (1.0 + assign3710_e3226);
        let assign3710_e3228: f64 = (p.p362 * assign3710_e3227);
        let assign3710_e3232: f64 = (p.p364 * locals.var_iwe);
        let assign3710_e3233: f64 = (1.0 + assign3710_e3232);
        let assign3710_e3234: f64 = (assign3710_e3228 * assign3710_e3233);
        let assign3710_e3238: f64 = (p.p365 * locals.var_iae);
        let assign3710_e3239: f64 = (1.0 + assign3710_e3238);
        let assign3710_e3240: f64 = (assign3710_e3234 * assign3710_e3239);
        (assign3710_e3240,)
    } else {
        (locals.var_stvfbedge_i,)
    }
};
        locals.var_stvfbedge_i = assign3710_e3242;

        let (assign3720_e3247,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p366,)
    } else {
        (locals.var_cic1edge_i,)
    }
};
        locals.var_cic1edge_i = assign3720_e3247;

        let (assign3730_e3252,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p367,)
    } else {
        (locals.var_cic2edge_i,)
    }
};
        locals.var_cic2edge_i = assign3730_e3252;

        let (assign3740_e3269, assign3740_e3269_d_n4, assign3740_e3269_d_n6, assign3740_e3269_d_n7, assign3740_e3269_d_n8, assign3740_e3269_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3740_e3257: f64 = (p.p368 * 2.0);
        let assign3740_e3260: f64 = (locals.var_lambda_le).powf(p.p369);
        let assign3740_e3261: f64 = (assign3740_e3257 * assign3740_e3260);
        let assign3740_e3265: f64 = (p.p370 * locals.var_iwe);
        let assign3740_e3266: f64 = (1.0 + assign3740_e3265);
        let assign3740_e3267: f64 = (assign3740_e3261 * assign3740_e3266);
        (assign3740_e3267, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3740_e3269;
        locals.var_temp_dn4 = assign3740_e3269_d_n4;
        locals.var_temp_dn6 = assign3740_e3269_d_n6;
        locals.var_temp_dn7 = assign3740_e3269_d_n7;
        locals.var_temp_dn8 = assign3740_e3269_d_n8;
        locals.var_temp_dn9 = assign3740_e3269_d_n9;

        let (assign3750_e3278, assign3750_e3278_d_n4, assign3750_e3278_d_n6, assign3750_e3278_d_n7, assign3750_e3278_d_n8, assign3750_e3278_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3750_e3274: f64 = (locals.var_temp).max(0.0);
        let assign3750_e3276: f64 = (assign3750_e3274).min(5.0);
        (assign3750_e3276, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 } } else { 0.0 },)
    } else {
        (locals.var_psce1edge_i, locals.var_psce1edge_i_dn4, locals.var_psce1edge_i_dn6, locals.var_psce1edge_i_dn7, locals.var_psce1edge_i_dn8, locals.var_psce1edge_i_dn9,)
    }
};
        locals.var_psce1edge_i = assign3750_e3278;
        locals.var_psce1edge_i_dn4 = assign3750_e3278_d_n4;
        locals.var_psce1edge_i_dn6 = assign3750_e3278_d_n6;
        locals.var_psce1edge_i_dn7 = assign3750_e3278_d_n7;
        locals.var_psce1edge_i_dn8 = assign3750_e3278_d_n8;
        locals.var_psce1edge_i_dn9 = assign3750_e3278_d_n9;

        let (assign3760_e3289, assign3760_e3289_d_n4, assign3760_e3289_d_n6, assign3760_e3289_d_n7, assign3760_e3289_d_n8, assign3760_e3289_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3760_e3283: f64 = (p.p371 * locals.var_psce1edge_i);
        let assign3760_e3285: f64 = (assign3760_e3283 * locals.var_tox2_i);
        let assign3760_e3287: f64 = (assign3760_e3285 / locals.var_tox1_i);
        (assign3760_e3287, (((p.p371 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9,)
    }
};
        locals.var_psce2edge_i = assign3760_e3289;
        locals.var_psce2edge_i_dn4 = assign3760_e3289_d_n4;
        locals.var_psce2edge_i_dn6 = assign3760_e3289_d_n6;
        locals.var_psce2edge_i_dn7 = assign3760_e3289_d_n7;
        locals.var_psce2edge_i_dn8 = assign3760_e3289_d_n8;
        locals.var_psce2edge_i_dn9 = assign3760_e3289_d_n9;

        let (assign3770_e3302, assign3770_e3302_d_n4, assign3770_e3302_d_n6, assign3770_e3302_d_n7, assign3770_e3302_d_n8, assign3770_e3302_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3770_e3294: f64 = (locals.var_lambda_le).powf(p.p373);
        let assign3770_e3298: f64 = (p.p374 * locals.var_iwe);
        let assign3770_e3299: f64 = (1.0 + assign3770_e3298);
        let assign3770_e3300: f64 = (assign3770_e3294 * assign3770_e3299);
        (assign3770_e3300, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3770_e3302;
        locals.var_temp_dn4 = assign3770_e3302_d_n4;
        locals.var_temp_dn6 = assign3770_e3302_d_n6;
        locals.var_temp_dn7 = assign3770_e3302_d_n7;
        locals.var_temp_dn8 = assign3770_e3302_d_n8;
        locals.var_temp_dn9 = assign3770_e3302_d_n9;

        let (assign3780_e3309, assign3780_e3309_d_n4, assign3780_e3309_d_n6, assign3780_e3309_d_n7, assign3780_e3309_d_n8, assign3780_e3309_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3780_e3307: f64 = (p.p372 * locals.var_temp);
        (assign3780_e3307, (p.p372 * locals.var_temp_dn4), (p.p372 * locals.var_temp_dn6), (p.p372 * locals.var_temp_dn7), (p.p372 * locals.var_temp_dn8), (p.p372 * locals.var_temp_dn9),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3780_e3309;
        locals.var_temp_dn4 = assign3780_e3309_d_n4;
        locals.var_temp_dn6 = assign3780_e3309_d_n6;
        locals.var_temp_dn7 = assign3780_e3309_d_n7;
        locals.var_temp_dn8 = assign3780_e3309_d_n8;
        locals.var_temp_dn9 = assign3780_e3309_d_n9;

        let (assign3790_e3316, assign3790_e3316_d_n4, assign3790_e3316_d_n6, assign3790_e3316_d_n7, assign3790_e3316_d_n8, assign3790_e3316_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3790_e3314: f64 = (locals.var_temp).max(0.0);
        (assign3790_e3314, if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 },)
    } else {
        (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9,)
    }
};
        locals.var_cf1edge_i = assign3790_e3316;
        locals.var_cf1edge_i_dn4 = assign3790_e3316_d_n4;
        locals.var_cf1edge_i_dn6 = assign3790_e3316_d_n6;
        locals.var_cf1edge_i_dn7 = assign3790_e3316_d_n7;
        locals.var_cf1edge_i_dn8 = assign3790_e3316_d_n8;
        locals.var_cf1edge_i_dn9 = assign3790_e3316_d_n9;

        let (assign3800_e3327, assign3800_e3327_d_n4, assign3800_e3327_d_n6, assign3800_e3327_d_n7, assign3800_e3327_d_n8, assign3800_e3327_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3800_e3321: f64 = (p.p375 * locals.var_cf1edge_i);
        let assign3800_e3323: f64 = (assign3800_e3321 * locals.var_tox2_i);
        let assign3800_e3325: f64 = (assign3800_e3323 / locals.var_tox1_i);
        (assign3800_e3325, (((p.p375 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9,)
    }
};
        locals.var_cf2edge_i = assign3800_e3327;
        locals.var_cf2edge_i_dn4 = assign3800_e3327_d_n4;
        locals.var_cf2edge_i_dn6 = assign3800_e3327_d_n6;
        locals.var_cf2edge_i_dn7 = assign3800_e3327_d_n7;
        locals.var_cf2edge_i_dn8 = assign3800_e3327_d_n8;
        locals.var_cf2edge_i_dn9 = assign3800_e3327_d_n9;

        let (assign3810_e3332,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p376,)
    } else {
        (locals.var_cfdedge_i,)
    }
};
        locals.var_cfdedge_i = assign3810_e3332;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign3820_e3351, assign3820_e3351_d_n4, assign3820_e3351_d_n6, assign3820_e3351_d_n7, assign3820_e3351_d_n8, assign3820_e3351_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3820_e3338: f64 = (p.p377 * p.p378);
        let assign3820_e3340: f64 = (assign3820_e3338 / locals.var_le);
        let assign3820_e3343: f64 = (-locals.var_le);
        let assign3820_e3345: f64 = (assign3820_e3343 / p.p378);
        let assign3820_e3346: f64 = (assign3820_e3345).exp();
        let assign3820_e3347: f64 = (1.0 - assign3820_e3346);
        let assign3820_e3348: f64 = (assign3820_e3340 * assign3820_e3347);
        let assign3820_e3349: f64 = (1.0 + assign3820_e3348);
        (assign3820_e3349, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3820_e3351;
        locals.var_temp_dn4 = assign3820_e3351_d_n4;
        locals.var_temp_dn6 = assign3820_e3351_d_n6;
        locals.var_temp_dn7 = assign3820_e3351_d_n7;
        locals.var_temp_dn8 = assign3820_e3351_d_n8;
        locals.var_temp_dn9 = assign3820_e3351_d_n9;

        let (assign3830_e3358, assign3830_e3358_d_n4, assign3830_e3358_d_n6, assign3830_e3358_d_n7, assign3830_e3358_d_n8, assign3830_e3358_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3830_e3356: f64 = (locals.var_temp).max(1e-15);
        (assign3830_e3356, if locals.var_temp >= 1e-15 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn9 } else { 0.0 },)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign3830_e3358;
        locals.var_temp_dn4 = assign3830_e3358_d_n4;
        locals.var_temp_dn6 = assign3830_e3358_d_n6;
        locals.var_temp_dn7 = assign3830_e3358_d_n7;
        locals.var_temp_dn8 = assign3830_e3358_d_n8;
        locals.var_temp_dn9 = assign3830_e3358_d_n9;

        let (assign3840_e3375, assign3840_e3375_d_n4, assign3840_e3375_d_n6, assign3840_e3375_d_n7, assign3840_e3375_d_n8, assign3840_e3375_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3840_e3363: f64 = (p.p240 * locals.var_we_edge);
        let assign3840_e3366: f64 = (locals.var_temp * locals.var_le);
        let assign3840_e3367: f64 = (assign3840_e3363 / assign3840_e3366);
        let assign3840_e3371: f64 = (p.p379 * locals.var_iwe);
        let assign3840_e3372: f64 = (1.0 + assign3840_e3371);
        let assign3840_e3373: f64 = (assign3840_e3367 * assign3840_e3372);
        (assign3840_e3373, ((-((assign3840_e3363 * (locals.var_temp_dn4 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn6 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn7 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn8 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn9 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372),)
    } else {
        (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9,)
    }
};
        locals.var_betnedge_t = assign3840_e3375;
        locals.var_betnedge_t_dn4 = assign3840_e3375_d_n4;
        locals.var_betnedge_t_dn6 = assign3840_e3375_d_n6;
        locals.var_betnedge_t_dn7 = assign3840_e3375_d_n7;
        locals.var_betnedge_t_dn8 = assign3840_e3375_d_n8;
        locals.var_betnedge_t_dn9 = assign3840_e3375_d_n9;

        let (assign3850_e3394,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3850_e3381: f64 = (p.p381 * locals.var_ile);
        let assign3850_e3382: f64 = (p.p380 + assign3850_e3381);
        let assign3850_e3385: f64 = (p.p382 * locals.var_iwe);
        let assign3850_e3386: f64 = (assign3850_e3382 + assign3850_e3385);
        let assign3850_e3389: f64 = (p.p383 * locals.var_ile);
        let assign3850_e3391: f64 = (assign3850_e3389 * locals.var_iwe);
        let assign3850_e3392: f64 = (assign3850_e3386 + assign3850_e3391);
        (assign3850_e3392,)
    } else {
        (locals.var_stbetedge_i,)
    }
};
        locals.var_stbetedge_i = assign3850_e3394;

        let (assign3860_e3401,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3860_e3399: f64 = (locals.var_wecv * locals.var_lecv);
        (assign3860_e3399,)
    } else {
        (locals.var_areaq_i,)
    }
};
        locals.var_areaq_i = assign3860_e3401;

        let (assign3870_e3410, assign3870_e3410_d_n4, assign3870_e3410_d_n6, assign3870_e3410_d_n7, assign3870_e3410_d_n8, assign3870_e3410_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3870_e3407: f64 = (p.p393 * locals.var_lphy);
        let assign3870_e3408: f64 = (p.p392 + assign3870_e3407);
        (assign3870_e3408, (p.p393 * locals.var_lphy_dn4), (p.p393 * locals.var_lphy_dn6), (p.p393 * locals.var_lphy_dn7), (p.p393 * locals.var_lphy_dn8), (p.p393 * locals.var_lphy_dn9),)
    } else {
        (locals.var_cgbov_p, locals.var_cgbov_p_dn4, locals.var_cgbov_p_dn6, locals.var_cgbov_p_dn7, locals.var_cgbov_p_dn8, locals.var_cgbov_p_dn9,)
    }
};
        locals.var_cgbov_p = assign3870_e3410;
        locals.var_cgbov_p_dn4 = assign3870_e3410_d_n4;
        locals.var_cgbov_p_dn6 = assign3870_e3410_d_n6;
        locals.var_cgbov_p_dn7 = assign3870_e3410_d_n7;
        locals.var_cgbov_p_dn8 = assign3870_e3410_d_n8;
        locals.var_cgbov_p_dn9 = assign3870_e3410_d_n9;

        let (assign3880_e3417, assign3880_e3417_d_n4, assign3880_e3417_d_n6, assign3880_e3417_d_n7, assign3880_e3417_d_n8, assign3880_e3417_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3880_e3415: f64 = (locals.var_cgbov_p).max(0.0);
        (assign3880_e3415, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn4 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn6 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn7 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn8 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9,)
    }
};
        locals.var_cgbov_i = assign3880_e3417;
        locals.var_cgbov_i_dn4 = assign3880_e3417_d_n4;
        locals.var_cgbov_i_dn6 = assign3880_e3417_d_n6;
        locals.var_cgbov_i_dn7 = assign3880_e3417_d_n7;
        locals.var_cgbov_i_dn8 = assign3880_e3417_d_n8;
        locals.var_cgbov_i_dn9 = assign3880_e3417_d_n9;

        let (assign3890_e3424,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3890_e3422: f64 = (p.p394 * 1000000.0);
        (assign3890_e3422,)
    } else {
        (locals.var_nsdac_i,)
    }
};
        locals.var_nsdac_i = assign3890_e3424;

        let (assign3900_e3433,) = {
    if (locals.var_guard83 == 0.0) {
        let assign3900_e3429: f64 = (p.p395 * locals.var_wecv);
        let assign3900_e3431: f64 = (assign3900_e3429 / locals.var_wen);
        (assign3900_e3431,)
    } else {
        (locals.var_fif_i,)
    }
};
        locals.var_fif_i = assign3900_e3433;

        let (assign3910_e3438,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p396,)
    } else {
        (locals.var_fsceac_i,)
    }
};
        locals.var_fsceac_i = assign3910_e3438;

        let (assign3920_e3443, assign3920_e3443_d_n4, assign3920_e3443_d_n6, assign3920_e3443_d_n7, assign3920_e3443_d_n8, assign3920_e3443_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign3920_e3443;
        locals.var_vfbac1_t_dn4 = assign3920_e3443_d_n4;
        locals.var_vfbac1_t_dn6 = assign3920_e3443_d_n6;
        locals.var_vfbac1_t_dn7 = assign3920_e3443_d_n7;
        locals.var_vfbac1_t_dn8 = assign3920_e3443_d_n8;
        locals.var_vfbac1_t_dn9 = assign3920_e3443_d_n9;

        let (assign3930_e3448, assign3930_e3448_d_n4, assign3930_e3448_d_n6, assign3930_e3448_d_n7, assign3930_e3448_d_n8, assign3930_e3448_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign3930_e3448;
        locals.var_vfbac2_t_dn4 = assign3930_e3448_d_n4;
        locals.var_vfbac2_t_dn6 = assign3930_e3448_d_n6;
        locals.var_vfbac2_t_dn7 = assign3930_e3448_d_n7;
        locals.var_vfbac2_t_dn8 = assign3930_e3448_d_n8;
        locals.var_vfbac2_t_dn9 = assign3930_e3448_d_n9;

        let (assign3940_e3453,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_psce1_i,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign3940_e3453;

        let (assign3950_e3458,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_psce2_i,)
    } else {
        (locals.var_psceac2_i,)
    }
};
        locals.var_psceac2_i = assign3950_e3458;

        let (assign3960_e3463, assign3960_e3463_d_n4, assign3960_e3463_d_n6, assign3960_e3463_d_n7, assign3960_e3463_d_n8, assign3960_e3463_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9,)
    } else {
        (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9,)
    }
};
        locals.var_cfac_p = assign3960_e3463;
        locals.var_cfac_p_dn4 = assign3960_e3463_d_n4;
        locals.var_cfac_p_dn6 = assign3960_e3463_d_n6;
        locals.var_cfac_p_dn7 = assign3960_e3463_d_n7;
        locals.var_cfac_p_dn8 = assign3960_e3463_d_n8;
        locals.var_cfac_p_dn9 = assign3960_e3463_d_n9;

        let (assign3970_e3468, assign3970_e3468_d_n4, assign3970_e3468_d_n6, assign3970_e3468_d_n7, assign3970_e3468_d_n8, assign3970_e3468_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign3970_e3468;
        locals.var_cfac1_t_dn4 = assign3970_e3468_d_n4;
        locals.var_cfac1_t_dn6 = assign3970_e3468_d_n6;
        locals.var_cfac1_t_dn7 = assign3970_e3468_d_n7;
        locals.var_cfac1_t_dn8 = assign3970_e3468_d_n8;
        locals.var_cfac1_t_dn9 = assign3970_e3468_d_n9;

        let (assign3980_e3473, assign3980_e3473_d_n4, assign3980_e3473_d_n6, assign3980_e3473_d_n7, assign3980_e3473_d_n8, assign3980_e3473_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign3980_e3473;
        locals.var_cfac2_t_dn4 = assign3980_e3473_d_n4;
        locals.var_cfac2_t_dn6 = assign3980_e3473_d_n6;
        locals.var_cfac2_t_dn7 = assign3980_e3473_d_n7;
        locals.var_cfac2_t_dn8 = assign3980_e3473_d_n8;
        locals.var_cfac2_t_dn9 = assign3980_e3473_d_n9;

        let (assign3990_e3478, assign3990_e3478_d_n4, assign3990_e3478_d_n6, assign3990_e3478_d_n7, assign3990_e3478_d_n8, assign3990_e3478_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9,)
    } else {
        (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9,)
    }
};
        locals.var_thesatac_p = assign3990_e3478;
        locals.var_thesatac_p_dn4 = assign3990_e3478_d_n4;
        locals.var_thesatac_p_dn6 = assign3990_e3478_d_n6;
        locals.var_thesatac_p_dn7 = assign3990_e3478_d_n7;
        locals.var_thesatac_p_dn8 = assign3990_e3478_d_n8;
        locals.var_thesatac_p_dn9 = assign3990_e3478_d_n9;

        let (assign4000_e3483, assign4000_e3483_d_n4, assign4000_e3483_d_n6, assign4000_e3483_d_n7, assign4000_e3483_d_n8, assign4000_e3483_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign4000_e3483;
        locals.var_thesatac_t_dn4 = assign4000_e3483_d_n4;
        locals.var_thesatac_t_dn6 = assign4000_e3483_d_n6;
        locals.var_thesatac_t_dn7 = assign4000_e3483_d_n7;
        locals.var_thesatac_t_dn8 = assign4000_e3483_d_n8;
        locals.var_thesatac_t_dn9 = assign4000_e3483_d_n9;

        let (assign4010_e3488,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_ax_i,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign4010_e3488;

        let (assign4020_e3493,) = {
    if (locals.var_guard83 == 0.0) {
        (locals.var_alp_i,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign4020_e3493;

        let assign4030_e3496: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign4030_e3496;

        let (assign4040_e3503,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p207,)
    } else {
        (locals.var_vfbaco_i,)
    }
};
        locals.var_vfbaco_i = assign4040_e3503;

        let assign4050_e3505: f64 = if param_given[397] { 1.0 } else { 0.0 };
        let assign4050_e3507: f64 = if assign4050_e3505 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign4050_e3507;

        let (assign4060_e3516,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
        (p.p397,)
    } else {
        (locals.var_vfbaco_i,)
    }
};
        locals.var_vfbaco_i = assign4060_e3516;

        let (assign4070_e3523,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p208,)
    } else {
        (locals.var_vfbacl_i,)
    }
};
        locals.var_vfbacl_i = assign4070_e3523;

        let assign4080_e3525: f64 = if param_given[398] { 1.0 } else { 0.0 };
        let assign4080_e3527: f64 = if assign4080_e3525 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign4080_e3527;

        let (assign4090_e3536,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
        (p.p398,)
    } else {
        (locals.var_vfbacl_i,)
    }
};
        locals.var_vfbacl_i = assign4090_e3536;

        let (assign4100_e3543,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p209,)
    } else {
        (locals.var_vfbaclexp_i,)
    }
};
        locals.var_vfbaclexp_i = assign4100_e3543;

        let assign4110_e3545: f64 = if param_given[399] { 1.0 } else { 0.0 };
        let assign4110_e3547: f64 = if assign4110_e3545 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign4110_e3547;

        let (assign4120_e3556,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 != 0.0)) {
        (p.p399,)
    } else {
        (locals.var_vfbaclexp_i,)
    }
};
        locals.var_vfbaclexp_i = assign4120_e3556;

        let (assign4130_e3563,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p212,)
    } else {
        (locals.var_vfbacw_i,)
    }
};
        locals.var_vfbacw_i = assign4130_e3563;

        let assign4140_e3565: f64 = if param_given[402] { 1.0 } else { 0.0 };
        let assign4140_e3567: f64 = if assign4140_e3565 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign4140_e3567;

        let (assign4150_e3576,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard102 != 0.0)) {
        (p.p402,)
    } else {
        (locals.var_vfbacw_i,)
    }
};
        locals.var_vfbacw_i = assign4150_e3576;

        let (assign4160_e3583,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p213,)
    } else {
        (locals.var_vfbaclw_i,)
    }
};
        locals.var_vfbaclw_i = assign4160_e3583;

        let assign4170_e3585: f64 = if param_given[403] { 1.0 } else { 0.0 };
        let assign4170_e3587: f64 = if assign4170_e3585 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign4170_e3587;

        let (assign4180_e3596,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard103 != 0.0)) {
        (p.p403,)
    } else {
        (locals.var_vfbaclw_i,)
    }
};
        locals.var_vfbaclw_i = assign4180_e3596;

        let (assign4190_e3603,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p210,)
    } else {
        (locals.var_vfbacl2_i,)
    }
};
        locals.var_vfbacl2_i = assign4190_e3603;

        let assign4200_e3605: f64 = if param_given[400] { 1.0 } else { 0.0 };
        let assign4200_e3607: f64 = if assign4200_e3605 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign4200_e3607;

        let (assign4210_e3616,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard104 != 0.0)) {
        (p.p400,)
    } else {
        (locals.var_vfbacl2_i,)
    }
};
        locals.var_vfbacl2_i = assign4210_e3616;

        let (assign4220_e3623,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p211,)
    } else {
        (locals.var_vfbaclexp2_i,)
    }
};
        locals.var_vfbaclexp2_i = assign4220_e3623;

        let assign4230_e3625: f64 = if param_given[401] { 1.0 } else { 0.0 };
        let assign4230_e3627: f64 = if assign4230_e3625 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign4230_e3627;

        let (assign4240_e3636,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard105 != 0.0)) {
        (p.p401,)
    } else {
        (locals.var_vfbaclexp2_i,)
    }
};
        locals.var_vfbaclexp2_i = assign4240_e3636;

        let (assign4250_e3655, assign4250_e3655_d_n4, assign4250_e3655_d_n6, assign4250_e3655_d_n7, assign4250_e3655_d_n8, assign4250_e3655_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4250_e3644: f64 = (locals.var_ile).powf(locals.var_vfbaclexp_i);
        let assign4250_e3645: f64 = (locals.var_vfbacl_i * assign4250_e3644);
        let assign4250_e3650: f64 = (locals.var_ile).powf(locals.var_vfbaclexp2_i);
        let assign4250_e3651: f64 = (locals.var_vfbacl2_i * assign4250_e3650);
        let assign4250_e3652: f64 = (1.0 + assign4250_e3651);
        let assign4250_e3653: f64 = (assign4250_e3645 / assign4250_e3652);
        (assign4250_e3653, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign4250_e3655;
        locals.var_temp_dn4 = assign4250_e3655_d_n4;
        locals.var_temp_dn6 = assign4250_e3655_d_n6;
        locals.var_temp_dn7 = assign4250_e3655_d_n7;
        locals.var_temp_dn8 = assign4250_e3655_d_n8;
        locals.var_temp_dn9 = assign4250_e3655_d_n9;

        let (assign4260_e3672, assign4260_e3672_d_n4, assign4260_e3672_d_n6, assign4260_e3672_d_n7, assign4260_e3672_d_n8, assign4260_e3672_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4260_e3662: f64 = (locals.var_vfbaco_i + locals.var_temp);
        let assign4260_e3665: f64 = (locals.var_vfbacw_i * locals.var_iwe);
        let assign4260_e3666: f64 = (assign4260_e3662 + assign4260_e3665);
        let assign4260_e3669: f64 = (locals.var_vfbaclw_i * locals.var_iae);
        let assign4260_e3670: f64 = (assign4260_e3666 + assign4260_e3669);
        (assign4260_e3670, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign4260_e3672;
        locals.var_vfbac1_t_dn4 = assign4260_e3672_d_n4;
        locals.var_vfbac1_t_dn6 = assign4260_e3672_d_n6;
        locals.var_vfbac1_t_dn7 = assign4260_e3672_d_n7;
        locals.var_vfbac1_t_dn8 = assign4260_e3672_d_n8;
        locals.var_vfbac1_t_dn9 = assign4260_e3672_d_n9;

        let (assign4270_e3679,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p214,)
    } else {
        (locals.var_vfbbaco_i,)
    }
};
        locals.var_vfbbaco_i = assign4270_e3679;

        let assign4280_e3681: f64 = if param_given[404] { 1.0 } else { 0.0 };
        let assign4280_e3683: f64 = if assign4280_e3681 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign4280_e3683;

        let (assign4290_e3692,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard106 != 0.0)) {
        (p.p404,)
    } else {
        (locals.var_vfbbaco_i,)
    }
};
        locals.var_vfbbaco_i = assign4290_e3692;

        let (assign4300_e3699,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p215,)
    } else {
        (locals.var_vfblbaco_i,)
    }
};
        locals.var_vfblbaco_i = assign4300_e3699;

        let assign4310_e3701: f64 = if param_given[405] { 1.0 } else { 0.0 };
        let assign4310_e3703: f64 = if assign4310_e3701 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign4310_e3703;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign4320_e3712,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard107 != 0.0)) {
        (p.p405,)
    } else {
        (locals.var_vfblbaco_i,)
    }
};
        locals.var_vfblbaco_i = assign4320_e3712;

        let (assign4330_e3727, assign4330_e3727_d_n4, assign4330_e3727_d_n6, assign4330_e3727_d_n7, assign4330_e3727_d_n8, assign4330_e3727_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4330_e3720: f64 = (locals.var_vfblbaco_i * locals.var_tox2_i);
        let assign4330_e3722: f64 = (assign4330_e3720 / locals.var_tox1_i);
        let assign4330_e3724: f64 = (assign4330_e3722 * locals.var_temp);
        let assign4330_e3725: f64 = (locals.var_vfbbaco_i + assign4330_e3724);
        (assign4330_e3725, (assign4330_e3722 * locals.var_temp_dn4), (assign4330_e3722 * locals.var_temp_dn6), (assign4330_e3722 * locals.var_temp_dn7), (assign4330_e3722 * locals.var_temp_dn8), (assign4330_e3722 * locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign4330_e3727;
        locals.var_vfbac2_t_dn4 = assign4330_e3727_d_n4;
        locals.var_vfbac2_t_dn6 = assign4330_e3727_d_n6;
        locals.var_vfbac2_t_dn7 = assign4330_e3727_d_n7;
        locals.var_vfbac2_t_dn8 = assign4330_e3727_d_n8;
        locals.var_vfbac2_t_dn9 = assign4330_e3727_d_n9;

        let (assign4340_e3734,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p224,)
    } else {
        (locals.var_psceacl_i,)
    }
};
        locals.var_psceacl_i = assign4340_e3734;

        let assign4350_e3736: f64 = if param_given[406] { 1.0 } else { 0.0 };
        let assign4350_e3738: f64 = if assign4350_e3736 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign4350_e3738;

        let (assign4360_e3747,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard108 != 0.0)) {
        (p.p406,)
    } else {
        (locals.var_psceacl_i,)
    }
};
        locals.var_psceacl_i = assign4360_e3747;

        let (assign4370_e3754,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p225,)
    } else {
        (locals.var_psceaclexp_i,)
    }
};
        locals.var_psceaclexp_i = assign4370_e3754;

        let assign4380_e3756: f64 = if param_given[407] { 1.0 } else { 0.0 };
        let assign4380_e3758: f64 = if assign4380_e3756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign4380_e3758;

        let (assign4390_e3767,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard109 != 0.0)) {
        (p.p407,)
    } else {
        (locals.var_psceaclexp_i,)
    }
};
        locals.var_psceaclexp_i = assign4390_e3767;

        let (assign4400_e3774,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p226,)
    } else {
        (locals.var_psceacw_i,)
    }
};
        locals.var_psceacw_i = assign4400_e3774;

        let assign4410_e3776: f64 = if param_given[408] { 1.0 } else { 0.0 };
        let assign4410_e3778: f64 = if assign4410_e3776 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign4410_e3778;

        let (assign4420_e3787,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard110 != 0.0)) {
        (p.p408,)
    } else {
        (locals.var_psceacw_i,)
    }
};
        locals.var_psceacw_i = assign4420_e3787;

        let (assign4430_e3806,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4430_e3794: f64 = (locals.var_psceacl_i * 2.0);
        let assign4430_e3797: f64 = (locals.var_lambda_le).powf(locals.var_psceaclexp_i);
        let assign4430_e3798: f64 = (assign4430_e3794 * assign4430_e3797);
        let assign4430_e3802: f64 = (locals.var_psceacw_i * locals.var_iwe);
        let assign4430_e3803: f64 = (1.0 + assign4430_e3802);
        let assign4430_e3804: f64 = (assign4430_e3798 * assign4430_e3803);
        (assign4430_e3804,)
    } else {
        (locals.var_psceac_p,)
    }
};
        locals.var_psceac_p = assign4430_e3806;

        let (assign4440_e3817,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4440_e3813: f64 = (locals.var_psceac_p).max(0.0);
        let assign4440_e3815: f64 = (assign4440_e3813).min(5.0);
        (assign4440_e3815,)
    } else {
        (locals.var_psceac1_i,)
    }
};
        locals.var_psceac1_i = assign4440_e3817;

        let (assign4450_e3830,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4450_e3824: f64 = (p.p227 * locals.var_psceac1_i);
        let assign4450_e3826: f64 = (assign4450_e3824 * locals.var_tox2_i);
        let assign4450_e3828: f64 = (assign4450_e3826 / locals.var_tox1_i);
        (assign4450_e3828,)
    } else {
        (locals.var_psceac2_i,)
    }
};
        locals.var_psceac2_i = assign4450_e3830;

        let (assign4460_e3837,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p231,)
    } else {
        (locals.var_cfacl_i,)
    }
};
        locals.var_cfacl_i = assign4460_e3837;

        let assign4470_e3839: f64 = if param_given[409] { 1.0 } else { 0.0 };
        let assign4470_e3841: f64 = if assign4470_e3839 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign4470_e3841;

        let (assign4480_e3850,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (p.p409,)
    } else {
        (locals.var_cfacl_i,)
    }
};
        locals.var_cfacl_i = assign4480_e3850;

        let (assign4490_e3857,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p232,)
    } else {
        (locals.var_cfaclexp_i,)
    }
};
        locals.var_cfaclexp_i = assign4490_e3857;

        let assign4500_e3859: f64 = if param_given[410] { 1.0 } else { 0.0 };
        let assign4500_e3861: f64 = if assign4500_e3859 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign4500_e3861;

        let (assign4510_e3870,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (p.p410,)
    } else {
        (locals.var_cfaclexp_i,)
    }
};
        locals.var_cfaclexp_i = assign4510_e3870;

        let (assign4520_e3877,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p233,)
    } else {
        (locals.var_cfacw_i,)
    }
};
        locals.var_cfacw_i = assign4520_e3877;

        let assign4530_e3879: f64 = if param_given[411] { 1.0 } else { 0.0 };
        let assign4530_e3881: f64 = if assign4530_e3879 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign4530_e3881;

        let (assign4540_e3890,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p411,)
    } else {
        (locals.var_cfacw_i,)
    }
};
        locals.var_cfacw_i = assign4540_e3890;

        let (assign4550_e3905, assign4550_e3905_d_n4, assign4550_e3905_d_n6, assign4550_e3905_d_n7, assign4550_e3905_d_n8, assign4550_e3905_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4550_e3897: f64 = (locals.var_lambda_le).powf(locals.var_cfaclexp_i);
        let assign4550_e3901: f64 = (locals.var_cfacw_i * locals.var_iwe);
        let assign4550_e3902: f64 = (1.0 + assign4550_e3901);
        let assign4550_e3903: f64 = (assign4550_e3897 * assign4550_e3902);
        (assign4550_e3903, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign4550_e3905;
        locals.var_temp_dn4 = assign4550_e3905_d_n4;
        locals.var_temp_dn6 = assign4550_e3905_d_n6;
        locals.var_temp_dn7 = assign4550_e3905_d_n7;
        locals.var_temp_dn8 = assign4550_e3905_d_n8;
        locals.var_temp_dn9 = assign4550_e3905_d_n9;

        let (assign4560_e3914, assign4560_e3914_d_n4, assign4560_e3914_d_n6, assign4560_e3914_d_n7, assign4560_e3914_d_n8, assign4560_e3914_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4560_e3912: f64 = (locals.var_cfacl_i * locals.var_temp);
        (assign4560_e3912, (locals.var_cfacl_i * locals.var_temp_dn4), (locals.var_cfacl_i * locals.var_temp_dn6), (locals.var_cfacl_i * locals.var_temp_dn7), (locals.var_cfacl_i * locals.var_temp_dn8), (locals.var_cfacl_i * locals.var_temp_dn9),)
    } else {
        (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9,)
    }
};
        locals.var_cfac_p = assign4560_e3914;
        locals.var_cfac_p_dn4 = assign4560_e3914_d_n4;
        locals.var_cfac_p_dn6 = assign4560_e3914_d_n6;
        locals.var_cfac_p_dn7 = assign4560_e3914_d_n7;
        locals.var_cfac_p_dn8 = assign4560_e3914_d_n8;
        locals.var_cfac_p_dn9 = assign4560_e3914_d_n9;

        let (assign4570_e3923, assign4570_e3923_d_n4, assign4570_e3923_d_n6, assign4570_e3923_d_n7, assign4570_e3923_d_n8, assign4570_e3923_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4570_e3921: f64 = (locals.var_cfac_p).max(0.0);
        (assign4570_e3921, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign4570_e3923;
        locals.var_cfac1_t_dn4 = assign4570_e3923_d_n4;
        locals.var_cfac1_t_dn6 = assign4570_e3923_d_n6;
        locals.var_cfac1_t_dn7 = assign4570_e3923_d_n7;
        locals.var_cfac1_t_dn8 = assign4570_e3923_d_n8;
        locals.var_cfac1_t_dn9 = assign4570_e3923_d_n9;

        let (assign4580_e3936, assign4580_e3936_d_n4, assign4580_e3936_d_n6, assign4580_e3936_d_n7, assign4580_e3936_d_n8, assign4580_e3936_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4580_e3930: f64 = (p.p234 * locals.var_cfac1_t);
        let assign4580_e3932: f64 = (assign4580_e3930 * locals.var_tox2_i);
        let assign4580_e3934: f64 = (assign4580_e3932 / locals.var_tox1_i);
        (assign4580_e3934, (((p.p234 * locals.var_cfac1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i),)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign4580_e3936;
        locals.var_cfac2_t_dn4 = assign4580_e3936_d_n4;
        locals.var_cfac2_t_dn6 = assign4580_e3936_d_n6;
        locals.var_cfac2_t_dn7 = assign4580_e3936_d_n7;
        locals.var_cfac2_t_dn8 = assign4580_e3936_d_n8;
        locals.var_cfac2_t_dn9 = assign4580_e3936_d_n9;

        let (assign4590_e3943,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p289,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign4590_e3943;

        let assign4600_e3945: f64 = if param_given[412] { 1.0 } else { 0.0 };
        let assign4600_e3947: f64 = if assign4600_e3945 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign4600_e3947;

        let (assign4610_e3956,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard114 != 0.0)) {
        (p.p412,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign4610_e3956;

        let (assign4620_e3963,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p290,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign4620_e3963;

        let assign4630_e3965: f64 = if param_given[413] { 1.0 } else { 0.0 };
        let assign4630_e3967: f64 = if assign4630_e3965 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign4630_e3967;

        let (assign4640_e3976,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard115 != 0.0)) {
        (p.p413,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign4640_e3976;

        let (assign4650_e3983,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p291,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign4650_e3983;

        let assign4660_e3985: f64 = if param_given[414] { 1.0 } else { 0.0 };
        let assign4660_e3987: f64 = if assign4660_e3985 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign4660_e3987;

        let (assign4670_e3996,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard116 != 0.0)) {
        (p.p414,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign4670_e3996;

        let (assign4680_e4003,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p292,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign4680_e4003;

        let assign4690_e4005: f64 = if param_given[415] { 1.0 } else { 0.0 };
        let assign4690_e4007: f64 = if assign4690_e4005 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign4690_e4007;

        let (assign4700_e4016,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard117 != 0.0)) {
        (p.p415,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign4700_e4016;

        let (assign4710_e4023,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p293,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign4710_e4023;

        let assign4720_e4025: f64 = if param_given[416] { 1.0 } else { 0.0 };
        let assign4720_e4027: f64 = if assign4720_e4025 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign4720_e4027;

        let (assign4730_e4036,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard118 != 0.0)) {
        (p.p416,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign4730_e4036;

        let (assign4740_e4063, assign4740_e4063_d_n4, assign4740_e4063_d_n6, assign4740_e4063_d_n7, assign4740_e4063_d_n8, assign4740_e4063_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4740_e4046: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
        let assign4740_e4047: f64 = (locals.var_thesatacl_i * assign4740_e4046);
        let assign4740_e4048: f64 = (locals.var_thesataco_i + assign4740_e4047);
        let assign4740_e4049: f64 = (locals.var_ge * assign4740_e4048);
        let assign4740_e4053: f64 = (locals.var_thesatacw_i * locals.var_iwe);
        let assign4740_e4054: f64 = (1.0 + assign4740_e4053);
        let assign4740_e4055: f64 = (assign4740_e4049 * assign4740_e4054);
        let assign4740_e4059: f64 = (locals.var_thesataclw_i * locals.var_iae);
        let assign4740_e4060: f64 = (1.0 + assign4740_e4059);
        let assign4740_e4061: f64 = (assign4740_e4055 * assign4740_e4060);
        (assign4740_e4061, (((locals.var_ge_dn4 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn6 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn7 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn8 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn9 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060),)
    } else {
        (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9,)
    }
};
        locals.var_thesatac_p = assign4740_e4063;
        locals.var_thesatac_p_dn4 = assign4740_e4063_d_n4;
        locals.var_thesatac_p_dn6 = assign4740_e4063_d_n6;
        locals.var_thesatac_p_dn7 = assign4740_e4063_d_n7;
        locals.var_thesatac_p_dn8 = assign4740_e4063_d_n8;
        locals.var_thesatac_p_dn9 = assign4740_e4063_d_n9;

        let (assign4750_e4072, assign4750_e4072_d_n4, assign4750_e4072_d_n6, assign4750_e4072_d_n7, assign4750_e4072_d_n8, assign4750_e4072_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4750_e4070: f64 = (locals.var_thesatac_p).max(0.0);
        (assign4750_e4070, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign4750_e4072;
        locals.var_thesatac_t_dn4 = assign4750_e4072_d_n4;
        locals.var_thesatac_t_dn6 = assign4750_e4072_d_n6;
        locals.var_thesatac_t_dn7 = assign4750_e4072_d_n7;
        locals.var_thesatac_t_dn8 = assign4750_e4072_d_n8;
        locals.var_thesatac_t_dn9 = assign4750_e4072_d_n9;

        let (assign4760_e4079,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p300,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign4760_e4079;

        let assign4770_e4081: f64 = if param_given[417] { 1.0 } else { 0.0 };
        let assign4770_e4083: f64 = if assign4770_e4081 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign4770_e4083;

        let (assign4780_e4092,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard119 != 0.0)) {
        (p.p417,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign4780_e4092;

        let (assign4790_e4099,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p301,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign4790_e4099;

        let assign4800_e4101: f64 = if param_given[418] { 1.0 } else { 0.0 };
        let assign4800_e4103: f64 = if assign4800_e4101 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign4800_e4103;

        let (assign4810_e4112,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard120 != 0.0)) {
        (p.p418,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign4810_e4112;

        let (assign4820_e4119,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p302,)
    } else {
        (locals.var_axaclexp_i,)
    }
};
        locals.var_axaclexp_i = assign4820_e4119;

        let assign4830_e4121: f64 = if param_given[419] { 1.0 } else { 0.0 };
        let assign4830_e4123: f64 = if assign4830_e4121 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign4830_e4123;

        let (assign4840_e4132,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard121 != 0.0)) {
        (p.p419,)
    } else {
        (locals.var_axaclexp_i,)
    }
};
        locals.var_axaclexp_i = assign4840_e4132;

        let (assign4850_e4139,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p303,)
    } else {
        (locals.var_axacl2_i,)
    }
};
        locals.var_axacl2_i = assign4850_e4139;

        let assign4860_e4141: f64 = if param_given[420] { 1.0 } else { 0.0 };
        let assign4860_e4143: f64 = if assign4860_e4141 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign4860_e4143;

        let (assign4870_e4152,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard122 != 0.0)) {
        (p.p420,)
    } else {
        (locals.var_axacl2_i,)
    }
};
        locals.var_axacl2_i = assign4870_e4152;

        let (assign4880_e4159,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p304,)
    } else {
        (locals.var_axaclexp2_i,)
    }
};
        locals.var_axaclexp2_i = assign4880_e4159;

        let assign4890_e4161: f64 = if param_given[421] { 1.0 } else { 0.0 };
        let assign4890_e4163: f64 = if assign4890_e4161 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign4890_e4163;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign4900_e4172,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard123 != 0.0)) {
        (p.p421,)
    } else {
        (locals.var_axaclexp2_i,)
    }
};
        locals.var_axaclexp2_i = assign4900_e4172;

        let (assign4910_e4195,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4910_e4182: f64 = (locals.var_ile).powf(locals.var_axaclexp_i);
        let assign4910_e4183: f64 = (locals.var_axacl_i * assign4910_e4182);
        let assign4910_e4188: f64 = (locals.var_ile).powf(locals.var_axaclexp2_i);
        let assign4910_e4189: f64 = (locals.var_axacl2_i * assign4910_e4188);
        let assign4910_e4190: f64 = (1.0 + assign4910_e4189);
        let assign4910_e4191: f64 = (assign4910_e4183 / assign4910_e4190);
        let assign4910_e4192: f64 = (1.0 + assign4910_e4191);
        let assign4910_e4193: f64 = (locals.var_axaco_i / assign4910_e4192);
        (assign4910_e4193,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign4910_e4195;

        let (assign4920_e4206,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign4920_e4202: f64 = (locals.var_axac_p).max(1.0);
        let assign4920_e4204: f64 = (assign4920_e4202).min(16.0);
        (assign4920_e4204,)
    } else {
        (locals.var_axac_i,)
    }
};
        locals.var_axac_i = assign4920_e4206;

        let (assign4930_e4213,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p305,)
    } else {
        (locals.var_alpacl1_i,)
    }
};
        locals.var_alpacl1_i = assign4930_e4213;

        let assign4940_e4215: f64 = if param_given[422] { 1.0 } else { 0.0 };
        let assign4940_e4217: f64 = if assign4940_e4215 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign4940_e4217;

        let (assign4950_e4226,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard124 != 0.0)) {
        (p.p422,)
    } else {
        (locals.var_alpacl1_i,)
    }
};
        locals.var_alpacl1_i = assign4950_e4226;

        let (assign4960_e4233,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p306,)
    } else {
        (locals.var_alpaclexp_i,)
    }
};
        locals.var_alpaclexp_i = assign4960_e4233;

        let assign4970_e4235: f64 = if param_given[423] { 1.0 } else { 0.0 };
        let assign4970_e4237: f64 = if assign4970_e4235 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign4970_e4237;

        let (assign4980_e4246,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard125 != 0.0)) {
        (p.p423,)
    } else {
        (locals.var_alpaclexp_i,)
    }
};
        locals.var_alpaclexp_i = assign4980_e4246;

        let (assign4990_e4253,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p307,)
    } else {
        (locals.var_alpacl2_i,)
    }
};
        locals.var_alpacl2_i = assign4990_e4253;

        let assign5000_e4255: f64 = if param_given[424] { 1.0 } else { 0.0 };
        let assign5000_e4257: f64 = if assign5000_e4255 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign5000_e4257;

        let (assign5010_e4266,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard126 != 0.0)) {
        (p.p424,)
    } else {
        (locals.var_alpacl2_i,)
    }
};
        locals.var_alpacl2_i = assign5010_e4266;

        let (assign5020_e4273,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p308,)
    } else {
        (locals.var_alpaclexp2_i,)
    }
};
        locals.var_alpaclexp2_i = assign5020_e4273;

        let assign5030_e4275: f64 = if param_given[425] { 1.0 } else { 0.0 };
        let assign5030_e4277: f64 = if assign5030_e4275 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign5030_e4277;

        let (assign5040_e4286,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard127 != 0.0)) {
        (p.p425,)
    } else {
        (locals.var_alpaclexp2_i,)
    }
};
        locals.var_alpaclexp2_i = assign5040_e4286;

        let (assign5050_e4293,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        (p.p309,)
    } else {
        (locals.var_alpacw_i,)
    }
};
        locals.var_alpacw_i = assign5050_e4293;

        let assign5060_e4295: f64 = if param_given[426] { 1.0 } else { 0.0 };
        let assign5060_e4297: f64 = if assign5060_e4295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign5060_e4297;

        let (assign5070_e4306,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard128 != 0.0)) {
        (p.p426,)
    } else {
        (locals.var_alpacw_i,)
    }
};
        locals.var_alpacw_i = assign5070_e4306;

        let (assign5080_e4331,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5080_e4314: f64 = (locals.var_ile).powf(locals.var_alpaclexp_i);
        let assign5080_e4315: f64 = (locals.var_alpacl1_i * assign5080_e4314);
        let assign5080_e4319: f64 = (locals.var_alpacw_i * locals.var_iwe);
        let assign5080_e4320: f64 = (1.0 + assign5080_e4319);
        let assign5080_e4321: f64 = (assign5080_e4315 * assign5080_e4320);
        let assign5080_e4326: f64 = (locals.var_ile).powf(locals.var_alpaclexp2_i);
        let assign5080_e4327: f64 = (locals.var_alpacl2_i * assign5080_e4326);
        let assign5080_e4328: f64 = (1.0 + assign5080_e4327);
        let assign5080_e4329: f64 = (assign5080_e4321 / assign5080_e4328);
        (assign5080_e4329,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign5080_e4331;

        let (assign5090_e4340,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
        let assign5090_e4338: f64 = (locals.var_alpac_p).max(0.0);
        (assign5090_e4338,)
    } else {
        (locals.var_alpac_i,)
    }
};
        locals.var_alpac_i = assign5090_e4340;

        let (assign5100_e4349, assign5100_e4349_d_n4, assign5100_e4349_d_n6, assign5100_e4349_d_n7, assign5100_e4349_d_n8, assign5100_e4349_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5100_e4345: f64 = (3.45313e-11 / locals.var_tox1_i);
        let assign5100_e4347: f64 = (assign5100_e4345 * locals.var_wecv);
        (assign5100_e4347, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5100_e4349;
        locals.var_temp_dn4 = assign5100_e4349_d_n4;
        locals.var_temp_dn6 = assign5100_e4349_d_n6;
        locals.var_temp_dn7 = assign5100_e4349_d_n7;
        locals.var_temp_dn8 = assign5100_e4349_d_n8;
        locals.var_temp_dn9 = assign5100_e4349_d_n9;

        let (assign5110_e4356, assign5110_e4356_d_n4, assign5110_e4356_d_n6, assign5110_e4356_d_n7, assign5110_e4356_d_n8, assign5110_e4356_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5110_e4354: f64 = (locals.var_temp * p.p427);
        (assign5110_e4354, (locals.var_temp_dn4 * p.p427), (locals.var_temp_dn6 * p.p427), (locals.var_temp_dn7 * p.p427), (locals.var_temp_dn8 * p.p427), (locals.var_temp_dn9 * p.p427),)
    } else {
        (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9,)
    }
};
        locals.var_cov_i = assign5110_e4356;
        locals.var_cov_i_dn4 = assign5110_e4356_d_n4;
        locals.var_cov_i_dn6 = assign5110_e4356_d_n6;
        locals.var_cov_i_dn7 = assign5110_e4356_d_n7;
        locals.var_cov_i_dn8 = assign5110_e4356_d_n8;
        locals.var_cov_i_dn9 = assign5110_e4356_d_n9;

        let (assign5120_e4363, assign5120_e4363_d_n4, assign5120_e4363_d_n6, assign5120_e4363_d_n7, assign5120_e4363_d_n8, assign5120_e4363_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5120_e4361: f64 = (locals.var_temp * p.p428);
        (assign5120_e4361, (locals.var_temp_dn4 * p.p428), (locals.var_temp_dn6 * p.p428), (locals.var_temp_dn7 * p.p428), (locals.var_temp_dn8 * p.p428), (locals.var_temp_dn9 * p.p428),)
    } else {
        (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9,)
    }
};
        locals.var_covd_i = assign5120_e4363;
        locals.var_covd_i_dn4 = assign5120_e4363_d_n4;
        locals.var_covd_i_dn6 = assign5120_e4363_d_n6;
        locals.var_covd_i_dn7 = assign5120_e4363_d_n7;
        locals.var_covd_i_dn8 = assign5120_e4363_d_n8;
        locals.var_covd_i_dn9 = assign5120_e4363_d_n9;

        let (assign5130_e4378,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5130_e4370: f64 = (p.p430 * locals.var_wen);
        let assign5130_e4372: f64 = (assign5130_e4370 / locals.var_wecv);
        let assign5130_e4373: f64 = (1.0 + assign5130_e4372);
        let assign5130_e4375: f64 = (assign5130_e4373).max(0.001);
        let assign5130_e4376: f64 = (p.p429 / assign5130_e4375);
        (assign5130_e4376,)
    } else {
        (locals.var_covdl_i,)
    }
};
        locals.var_covdl_i = assign5130_e4378;

        let (assign5140_e4383,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p431,)
    } else {
        (locals.var_covdlb_i,)
    }
};
        locals.var_covdlb_i = assign5140_e4383;

        let (assign5150_e4388,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p432,)
    } else {
        (locals.var_dvfbov_i,)
    }
};
        locals.var_dvfbov_i = assign5150_e4388;

        let (assign5160_e4397, assign5160_e4397_d_n4, assign5160_e4397_d_n6, assign5160_e4397_d_n7, assign5160_e4397_d_n8, assign5160_e4397_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5160_e4394: f64 = (p.p435 * locals.var_wphy);
        let assign5160_e4395: f64 = (p.p433 + assign5160_e4394);
        (assign5160_e4395, (p.p435 * locals.var_wphy_dn4), (p.p435 * locals.var_wphy_dn6), (p.p435 * locals.var_wphy_dn7), (p.p435 * locals.var_wphy_dn8), (p.p435 * locals.var_wphy_dn9),)
    } else {
        (locals.var_cfr_p, locals.var_cfr_p_dn4, locals.var_cfr_p_dn6, locals.var_cfr_p_dn7, locals.var_cfr_p_dn8, locals.var_cfr_p_dn9,)
    }
};
        locals.var_cfr_p = assign5160_e4397;
        locals.var_cfr_p_dn4 = assign5160_e4397_d_n4;
        locals.var_cfr_p_dn6 = assign5160_e4397_d_n6;
        locals.var_cfr_p_dn7 = assign5160_e4397_d_n7;
        locals.var_cfr_p_dn8 = assign5160_e4397_d_n8;
        locals.var_cfr_p_dn9 = assign5160_e4397_d_n9;

        let (assign5170_e4404, assign5170_e4404_d_n4, assign5170_e4404_d_n6, assign5170_e4404_d_n7, assign5170_e4404_d_n8, assign5170_e4404_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5170_e4402: f64 = (locals.var_cfr_p).max(0.0);
        (assign5170_e4402, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn4 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn6 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn7 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn8 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9,)
    }
};
        locals.var_cfr_i = assign5170_e4404;
        locals.var_cfr_i_dn4 = assign5170_e4404_d_n4;
        locals.var_cfr_i_dn6 = assign5170_e4404_d_n6;
        locals.var_cfr_i_dn7 = assign5170_e4404_d_n7;
        locals.var_cfr_i_dn8 = assign5170_e4404_d_n8;
        locals.var_cfr_i_dn9 = assign5170_e4404_d_n9;

        let (assign5180_e4413, assign5180_e4413_d_n4, assign5180_e4413_d_n6, assign5180_e4413_d_n7, assign5180_e4413_d_n8, assign5180_e4413_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5180_e4410: f64 = (p.p436 * locals.var_wphy);
        let assign5180_e4411: f64 = (p.p434 + assign5180_e4410);
        (assign5180_e4411, (p.p436 * locals.var_wphy_dn4), (p.p436 * locals.var_wphy_dn6), (p.p436 * locals.var_wphy_dn7), (p.p436 * locals.var_wphy_dn8), (p.p436 * locals.var_wphy_dn9),)
    } else {
        (locals.var_cfrd_p, locals.var_cfrd_p_dn4, locals.var_cfrd_p_dn6, locals.var_cfrd_p_dn7, locals.var_cfrd_p_dn8, locals.var_cfrd_p_dn9,)
    }
};
        locals.var_cfrd_p = assign5180_e4413;
        locals.var_cfrd_p_dn4 = assign5180_e4413_d_n4;
        locals.var_cfrd_p_dn6 = assign5180_e4413_d_n6;
        locals.var_cfrd_p_dn7 = assign5180_e4413_d_n7;
        locals.var_cfrd_p_dn8 = assign5180_e4413_d_n8;
        locals.var_cfrd_p_dn9 = assign5180_e4413_d_n9;

        let (assign5190_e4420, assign5190_e4420_d_n4, assign5190_e4420_d_n6, assign5190_e4420_d_n7, assign5190_e4420_d_n8, assign5190_e4420_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5190_e4418: f64 = (locals.var_cfrd_p).max(0.0);
        (assign5190_e4418, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn4 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn6 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn7 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn8 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9,)
    }
};
        locals.var_cfrd_i = assign5190_e4420;
        locals.var_cfrd_i_dn4 = assign5190_e4420_d_n4;
        locals.var_cfrd_i_dn6 = assign5190_e4420_d_n6;
        locals.var_cfrd_i_dn7 = assign5190_e4420_d_n7;
        locals.var_cfrd_i_dn8 = assign5190_e4420_d_n8;
        locals.var_cfrd_i_dn9 = assign5190_e4420_d_n9;

        let (assign5200_e4433,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5200_e4425: f64 = (p.p437 * locals.var_epsch);
        let assign5200_e4427: f64 = (assign5200_e4425 * locals.var_tsi_i);
        let assign5200_e4429: f64 = (assign5200_e4427 * locals.var_we);
        let assign5200_e4431: f64 = (assign5200_e4429 / locals.var_le);
        (assign5200_e4431,)
    } else {
        (locals.var_csd_i,)
    }
};
        locals.var_csd_i = assign5200_e4433;

        let (assign5210_e4438,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p438,)
    } else {
        (locals.var_csdbp_i,)
    }
};
        locals.var_csdbp_i = assign5210_e4438;

        let (assign5220_e4459, assign5220_e4459_d_n4, assign5220_e4459_d_n6, assign5220_e4459_d_n7, assign5220_e4459_d_n8, assign5220_e4459_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5220_e4444: f64 = (p.p440 * locals.var_lphy);
        let assign5220_e4445: f64 = (1.0 + assign5220_e4444);
        let assign5220_e4448: f64 = (p.p441 * locals.var_wphy);
        let assign5220_e4449: f64 = (assign5220_e4445 + assign5220_e4448);
        let assign5220_e4452: f64 = (p.p442 * locals.var_lphy);
        let assign5220_e4454: f64 = (assign5220_e4452 * locals.var_wphy);
        let assign5220_e4455: f64 = (assign5220_e4449 + assign5220_e4454);
        let assign5220_e4457: f64 = (assign5220_e4455).max(1e-10);
        (assign5220_e4457, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn4) + (p.p441 * locals.var_wphy_dn4)) + (((p.p442 * locals.var_lphy_dn4) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn4))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn6) + (p.p441 * locals.var_wphy_dn6)) + (((p.p442 * locals.var_lphy_dn6) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn6))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn7) + (p.p441 * locals.var_wphy_dn7)) + (((p.p442 * locals.var_lphy_dn7) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn7))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn8) + (p.p441 * locals.var_wphy_dn8)) + (((p.p442 * locals.var_lphy_dn8) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn8))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn9) + (p.p441 * locals.var_wphy_dn9)) + (((p.p442 * locals.var_lphy_dn9) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn9))) } else { 0.0 },)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5220_e4459;
        locals.var_temp_dn4 = assign5220_e4459_d_n4;
        locals.var_temp_dn6 = assign5220_e4459_d_n6;
        locals.var_temp_dn7 = assign5220_e4459_d_n7;
        locals.var_temp_dn8 = assign5220_e4459_d_n8;
        locals.var_temp_dn9 = assign5220_e4459_d_n9;

        let (assign5230_e4464, assign5230_e4464_d_n4, assign5230_e4464_d_n6, assign5230_e4464_d_n7, assign5230_e4464_d_n8, assign5230_e4464_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign5230_e4464;
        locals.var_temp1_dn4 = assign5230_e4464_d_n4;
        locals.var_temp1_dn6 = assign5230_e4464_d_n6;
        locals.var_temp1_dn7 = assign5230_e4464_d_n7;
        locals.var_temp1_dn8 = assign5230_e4464_d_n8;
        locals.var_temp1_dn9 = assign5230_e4464_d_n9;

        let assign5240_e4471: f64 = if ((p.p29 > 1.0) && (p.p28 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign5240_e4471;

        let (assign5250_e4483, assign5250_e4483_d_n4, assign5250_e4483_d_n6, assign5250_e4483_d_n7, assign5250_e4483_d_n8, assign5250_e4483_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
        let assign5250_e4478: f64 = (p.p28 + p.p20);
        let assign5250_e4479: f64 = (-assign5250_e4478);
        let assign5250_e4481: f64 = (assign5250_e4479 / p.p445);
        (assign5250_e4481, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign5250_e4483;
        locals.var_temp2_dn4 = assign5250_e4483_d_n4;
        locals.var_temp2_dn6 = assign5250_e4483_d_n6;
        locals.var_temp2_dn7 = assign5250_e4483_d_n7;
        locals.var_temp2_dn8 = assign5250_e4483_d_n8;
        locals.var_temp2_dn9 = assign5250_e4483_d_n9;

        let assign5260_e4485: f64 = (locals.var_temp2).abs();
        let assign5260_e4487: f64 = if assign5260_e4485 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign5260_e4487;

        let (assign5270_e4497, assign5270_e4497_d_n4, assign5270_e4497_d_n6, assign5270_e4497_d_n7, assign5270_e4497_d_n8, assign5270_e4497_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 != 0.0)) {
        let assign5270_e4495: f64 = (locals.var_temp2).exp();
        (assign5270_e4495, (assign5270_e4495 * locals.var_temp2_dn4), (assign5270_e4495 * locals.var_temp2_dn6), (assign5270_e4495 * locals.var_temp2_dn7), (assign5270_e4495 * locals.var_temp2_dn8), (assign5270_e4495 * locals.var_temp2_dn9),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign5270_e4497;
        locals.var_temp3_dn4 = assign5270_e4497_d_n4;
        locals.var_temp3_dn6 = assign5270_e4497_d_n6;
        locals.var_temp3_dn7 = assign5270_e4497_d_n7;
        locals.var_temp3_dn8 = assign5270_e4497_d_n8;
        locals.var_temp3_dn9 = assign5270_e4497_d_n9;

        let assign5280_e4500: f64 = (-80.0);
        let assign5280_e4501: f64 = if locals.var_temp2 < assign5280_e4500 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign5280_e4501;

        let (assign5290_e4538, assign5290_e4538_d_n4, assign5290_e4538_d_n6, assign5290_e4538_d_n7, assign5290_e4538_d_n8, assign5290_e4538_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 == 0.0)) && (locals.var_guard131 != 0.0)) {
        let assign5290_e4514: f64 = (-locals.var_temp2);
        let assign5290_e4516: f64 = (assign5290_e4514 - 80.0);
        let assign5290_e4520: f64 = (-locals.var_temp2);
        let assign5290_e4522: f64 = (assign5290_e4520 - 80.0);
        let assign5290_e4523: f64 = (0.5 * assign5290_e4522);
        let assign5290_e4526: f64 = (-locals.var_temp2);
        let assign5290_e4528: f64 = (assign5290_e4526 - 80.0);
        let assign5290_e4530: f64 = (assign5290_e4528 * 0.3333333333333);
        let assign5290_e4531: f64 = (1.0 + assign5290_e4530);
        let assign5290_e4532: f64 = (assign5290_e4523 * assign5290_e4531);
        let assign5290_e4533: f64 = (1.0 + assign5290_e4532);
        let assign5290_e4534: f64 = (assign5290_e4516 * assign5290_e4533);
        let assign5290_e4535: f64 = (1.0 + assign5290_e4534);
        let assign5290_e4536: f64 = (1.80485e-35 / assign5290_e4535);
        (assign5290_e4536, (-((1.80485e-35 * (((-locals.var_temp2_dn4) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn4)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn4) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn6) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn6)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn6) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn7) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn7)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn7) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn8) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn8)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn8) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn9) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn9)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn9) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign5290_e4538;
        locals.var_temp3_dn4 = assign5290_e4538_d_n4;
        locals.var_temp3_dn6 = assign5290_e4538_d_n6;
        locals.var_temp3_dn7 = assign5290_e4538_d_n7;
        locals.var_temp3_dn8 = assign5290_e4538_d_n8;
        locals.var_temp3_dn9 = assign5290_e4538_d_n9;

        let (assign5300_e4573, assign5300_e4573_d_n4, assign5300_e4573_d_n6, assign5300_e4573_d_n7, assign5300_e4573_d_n8, assign5300_e4573_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 == 0.0)) && (locals.var_guard131 == 0.0)) {
        let assign5300_e4553: f64 = (locals.var_temp2 - 80.0);
        let assign5300_e4558: f64 = (locals.var_temp2 - 80.0);
        let assign5300_e4559: f64 = (0.5 * assign5300_e4558);
        let assign5300_e4563: f64 = (locals.var_temp2 - 80.0);
        let assign5300_e4565: f64 = (assign5300_e4563 * 0.3333333333333);
        let assign5300_e4566: f64 = (1.0 + assign5300_e4565);
        let assign5300_e4567: f64 = (assign5300_e4559 * assign5300_e4566);
        let assign5300_e4568: f64 = (1.0 + assign5300_e4567);
        let assign5300_e4569: f64 = (assign5300_e4553 * assign5300_e4568);
        let assign5300_e4570: f64 = (1.0 + assign5300_e4569);
        let assign5300_e4571: f64 = (5.54062e34 * assign5300_e4570);
        (assign5300_e4571, (5.54062e34 * ((locals.var_temp2_dn4 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn4) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn6 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn6) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn7 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn7) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn8 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn8) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn9 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn9) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn9 * 0.3333333333333)))))),)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign5300_e4573;
        locals.var_temp3_dn4 = assign5300_e4573_d_n4;
        locals.var_temp3_dn6 = assign5300_e4573_d_n6;
        locals.var_temp3_dn7 = assign5300_e4573_d_n7;
        locals.var_temp3_dn8 = assign5300_e4573_d_n8;
        locals.var_temp3_dn9 = assign5300_e4573_d_n9;

        let (assign5310_e4582, assign5310_e4582_d_n4, assign5310_e4582_d_n6, assign5310_e4582_d_n7, assign5310_e4582_d_n8, assign5310_e4582_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
        let assign5310_e4580: f64 = (1.0 - locals.var_temp3);
        (assign5310_e4580, (-locals.var_temp3_dn4), (-locals.var_temp3_dn6), (-locals.var_temp3_dn7), (-locals.var_temp3_dn8), (-locals.var_temp3_dn9),)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign5310_e4582;
        locals.var_temp4_dn4 = assign5310_e4582_d_n4;
        locals.var_temp4_dn6 = assign5310_e4582_d_n6;
        locals.var_temp4_dn7 = assign5310_e4582_d_n7;
        locals.var_temp4_dn8 = assign5310_e4582_d_n8;
        locals.var_temp4_dn9 = assign5310_e4582_d_n9;

        let (assign5320_e4607, assign5320_e4607_d_n4, assign5320_e4607_d_n6, assign5320_e4607_d_n7, assign5320_e4607_d_n8, assign5320_e4607_d_n9,) = {
    if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
        let assign5320_e4589: f64 = (2.0 * p.p446);
        let assign5320_e4591: f64 = (assign5320_e4589 * locals.var_temp3);
        let assign5320_e4596: f64 = (locals.var_temp3).powf(p.p29);
        let assign5320_e4597: f64 = (1.0 - assign5320_e4596);
        let assign5320_e4599: f64 = (assign5320_e4597 / p.p29);
        let assign5320_e4600: f64 = (locals.var_temp4 - assign5320_e4599);
        let assign5320_e4601: f64 = (assign5320_e4591 * assign5320_e4600);
        let assign5320_e4604: f64 = (locals.var_temp4 * locals.var_temp4);
        let assign5320_e4605: f64 = (assign5320_e4601 / assign5320_e4604);
        (assign5320_e4605, ((((((assign5320_e4589 * locals.var_temp3_dn4) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn4 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn4)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn4 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn4 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn4)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn6) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn6 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn6)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn6 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn6 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn6)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn7) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn7 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn7)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn7 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn7 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn7)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn8) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn8 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn8)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn8 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn8 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn8)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn9) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn9 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn9)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn9 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn9 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn9)))) / (assign5320_e4604 * assign5320_e4604)),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign5320_e4607;
        locals.var_temp1_dn4 = assign5320_e4607_d_n4;
        locals.var_temp1_dn6 = assign5320_e4607_d_n6;
        locals.var_temp1_dn7 = assign5320_e4607_d_n7;
        locals.var_temp1_dn8 = assign5320_e4607_d_n8;
        locals.var_temp1_dn9 = assign5320_e4607_d_n9;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5330_e4616, assign5330_e4616_d_n4, assign5330_e4616_d_n6, assign5330_e4616_d_n7, assign5330_e4616_d_n8, assign5330_e4616_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5330_e4613: f64 = (1.0 + locals.var_temp1);
        let assign5330_e4614: f64 = (locals.var_temp / assign5330_e4613);
        (assign5330_e4614, (((locals.var_temp_dn4 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn4)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn6 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn6)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn7 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn7)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn8 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn8)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn9 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn9)) / (assign5330_e4613 * assign5330_e4613)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5330_e4616;
        locals.var_temp_dn4 = assign5330_e4616_d_n4;
        locals.var_temp_dn6 = assign5330_e4616_d_n6;
        locals.var_temp_dn7 = assign5330_e4616_d_n7;
        locals.var_temp_dn8 = assign5330_e4616_d_n8;
        locals.var_temp_dn9 = assign5330_e4616_d_n9;

        let (assign5340_e4623, assign5340_e4623_d_n4, assign5340_e4623_d_n6, assign5340_e4623_d_n7, assign5340_e4623_d_n8, assign5340_e4623_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5340_e4621: f64 = (p.p439 / locals.var_temp);
        (assign5340_e4621, (-((p.p439 * locals.var_temp_dn4) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn6) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn7) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn8) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn9) / (locals.var_temp * locals.var_temp))),)
    } else {
        (locals.var_rth_p, locals.var_rth_p_dn4, locals.var_rth_p_dn6, locals.var_rth_p_dn7, locals.var_rth_p_dn8, locals.var_rth_p_dn9,)
    }
};
        locals.var_rth_p = assign5340_e4623;
        locals.var_rth_p_dn4 = assign5340_e4623_d_n4;
        locals.var_rth_p_dn6 = assign5340_e4623_d_n6;
        locals.var_rth_p_dn7 = assign5340_e4623_d_n7;
        locals.var_rth_p_dn8 = assign5340_e4623_d_n8;
        locals.var_rth_p_dn9 = assign5340_e4623_d_n9;

        let (assign5350_e4630, assign5350_e4630_d_n4, assign5350_e4630_d_n6, assign5350_e4630_d_n7, assign5350_e4630_d_n8, assign5350_e4630_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5350_e4628: f64 = (locals.var_rth_p).max(1e-6);
        (assign5350_e4628, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn4 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn6 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn7 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn8 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_rth_t, locals.var_rth_t_dn4, locals.var_rth_t_dn6, locals.var_rth_t_dn7, locals.var_rth_t_dn8, locals.var_rth_t_dn9,)
    }
};
        locals.var_rth_t = assign5350_e4630;
        locals.var_rth_t_dn4 = assign5350_e4630_d_n4;
        locals.var_rth_t_dn6 = assign5350_e4630_d_n6;
        locals.var_rth_t_dn7 = assign5350_e4630_d_n7;
        locals.var_rth_t_dn8 = assign5350_e4630_d_n8;
        locals.var_rth_t_dn9 = assign5350_e4630_d_n9;

        let (assign5360_e4635,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p443,)
    } else {
        (locals.var_strth_i,)
    }
};
        locals.var_strth_i = assign5360_e4635;

        let (assign5370_e4642, assign5370_e4642_d_n4, assign5370_e4642_d_n6, assign5370_e4642_d_n7, assign5370_e4642_d_n8, assign5370_e4642_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5370_e4640: f64 = (p.p444 * locals.var_temp);
        (assign5370_e4640, (p.p444 * locals.var_temp_dn4), (p.p444 * locals.var_temp_dn6), (p.p444 * locals.var_temp_dn7), (p.p444 * locals.var_temp_dn8), (p.p444 * locals.var_temp_dn9),)
    } else {
        (locals.var_cth_p, locals.var_cth_p_dn4, locals.var_cth_p_dn6, locals.var_cth_p_dn7, locals.var_cth_p_dn8, locals.var_cth_p_dn9,)
    }
};
        locals.var_cth_p = assign5370_e4642;
        locals.var_cth_p_dn4 = assign5370_e4642_d_n4;
        locals.var_cth_p_dn6 = assign5370_e4642_d_n6;
        locals.var_cth_p_dn7 = assign5370_e4642_d_n7;
        locals.var_cth_p_dn8 = assign5370_e4642_d_n8;
        locals.var_cth_p_dn9 = assign5370_e4642_d_n9;

        let (assign5380_e4649, assign5380_e4649_d_n4, assign5380_e4649_d_n6, assign5380_e4649_d_n7, assign5380_e4649_d_n8, assign5380_e4649_d_n9,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5380_e4647: f64 = (locals.var_cth_p).max(0.0);
        (assign5380_e4647, if locals.var_cth_p >= 0.0 { locals.var_cth_p_dn4 } else { 0.0 }, if locals.var_cth_p >= 0.0 { locals.var_cth_p_dn6 } else { 0.0 }, if locals.var_cth_p >= 0.0 { locals.var_cth_p_dn7 } else { 0.0 }, if locals.var_cth_p >= 0.0 { locals.var_cth_p_dn8 } else { 0.0 }, if locals.var_cth_p >= 0.0 { locals.var_cth_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cth_i, locals.var_cth_i_dn4, locals.var_cth_i_dn6, locals.var_cth_i_dn7, locals.var_cth_i_dn8, locals.var_cth_i_dn9,)
    }
};
        locals.var_cth_i = assign5380_e4649;
        locals.var_cth_i_dn4 = assign5380_e4649_d_n4;
        locals.var_cth_i_dn6 = assign5380_e4649_d_n6;
        locals.var_cth_i_dn7 = assign5380_e4649_d_n7;
        locals.var_cth_i_dn8 = assign5380_e4649_d_n8;
        locals.var_cth_i_dn9 = assign5380_e4649_d_n9;

        let (assign5390_e4654,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p447,)
    } else {
        (locals.var_fnt_i,)
    }
};
        locals.var_fnt_i = assign5390_e4654;

        let (assign5400_e4673,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5400_e4659: f64 = (p.p448 * locals.var_betn_p);
        let assign5400_e4661: f64 = (assign5400_e4659 * locals.var_betn_p);
        let assign5400_e4663: f64 = (assign5400_e4661 * locals.var_iwe);
        let assign5400_e4665: f64 = (assign5400_e4663 * locals.var_iwe);
        let assign5400_e4669: f64 = (p.p449 - 2.0);
        let assign5400_e4670: f64 = (locals.var_ile).powf(assign5400_e4669);
        let assign5400_e4671: f64 = (assign5400_e4665 * assign5400_e4670);
        (assign5400_e4671,)
    } else {
        (locals.var_fntexc_i,)
    }
};
        locals.var_fntexc_i = assign5400_e4673;

        let (assign5410_e4684,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5410_e4678: f64 = (p.p450 * locals.var_iae);
        let assign5410_e4681: f64 = (p.p451 * locals.var_iwe);
        let assign5410_e4682: f64 = (assign5410_e4678 + assign5410_e4681);
        (assign5410_e4682,)
    } else {
        (locals.var_nfa_p,)
    }
};
        locals.var_nfa_p = assign5410_e4684;

        let (assign5420_e4691,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5420_e4689: f64 = (locals.var_nfa_p).max(0.0);
        (assign5420_e4689,)
    } else {
        (locals.var_nfa_i,)
    }
};
        locals.var_nfa_i = assign5420_e4691;

        let (assign5430_e4698,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5430_e4696: f64 = (p.p452 * locals.var_iae);
        (assign5430_e4696,)
    } else {
        (locals.var_nfb_i,)
    }
};
        locals.var_nfb_i = assign5430_e4698;

        let (assign5440_e4705,) = {
    if (locals.var_guard83 == 0.0) {
        let assign5440_e4703: f64 = (p.p453 * locals.var_iae);
        (assign5440_e4703,)
    } else {
        (locals.var_nfc_i,)
    }
};
        locals.var_nfc_i = assign5440_e4705;

        let (assign5450_e4710,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p454,)
    } else {
        (locals.var_nfe_i,)
    }
};
        locals.var_nfe_i = assign5450_e4710;

        let (assign5460_e4715,) = {
    if (locals.var_guard83 == 0.0) {
        (p.p455,)
    } else {
        (locals.var_nfeb_i,)
    }
};
        locals.var_nfeb_i = assign5460_e4715;

        let assign5570_e4828: f64 = if ((((p.p457 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign5570_e4828;

        let assign5580_e4831: f64 = if p.p457 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign5580_e4831;

        let (assign5590_e4840, assign5590_e4840_d_n4, assign5590_e4840_d_n6, assign5590_e4840_d_n7, assign5590_e4840_d_n8, assign5590_e4840_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    }
};
        locals.var_tmpa = assign5590_e4840;
        locals.var_tmpa_dn4 = assign5590_e4840_d_n4;
        locals.var_tmpa_dn6 = assign5590_e4840_d_n6;
        locals.var_tmpa_dn7 = assign5590_e4840_d_n7;
        locals.var_tmpa_dn8 = assign5590_e4840_d_n8;
        locals.var_tmpa_dn9 = assign5590_e4840_d_n9;

        let (assign5600_e4849,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_tmpb,)
    }
};
        locals.var_tmpb = assign5600_e4849;

        let (assign5610_e4858,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_iloop,)
    }
};
        locals.var_iloop = assign5610_e4858;

        let mut assign5620_loop_guard: usize = 0;
        while {
            let assign5620_cond_e4868: f64 = (p.p29 - 0.5);
            let assign5620_cond_e4870: f64 = if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) && (locals.var_iloop < assign5620_cond_e4868)) { 1.0 } else { 0.0 };
            assign5620_cond_e4870 != 0.0
        } {
            assign5620_loop_guard += 1;
            assert!(assign5620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5620_body0_e4893, assign5620_body0_e4893_d_n4, assign5620_body0_e4893_d_n6, assign5620_body0_e4893_d_n7, assign5620_body0_e4893_d_n8, assign5620_body0_e4893_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5620_body0_e4882: f64 = (0.5 * p.p20);
        let assign5620_body0_e4883: f64 = (p.p26 + assign5620_body0_e4882);
        let assign5620_body0_e4887: f64 = (p.p28 + p.p20);
        let assign5620_body0_e4888: f64 = (locals.var_iloop * assign5620_body0_e4887);
        let assign5620_body0_e4889: f64 = (assign5620_body0_e4883 + assign5620_body0_e4888);
        let assign5620_body0_e4890: f64 = (1.0 / assign5620_body0_e4889);
        let assign5620_body0_e4891: f64 = (locals.var_tmpa + assign5620_body0_e4890);
        (assign5620_body0_e4891, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    } else {
        (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    }
};
            locals.var_tmpa = assign5620_body0_e4893;
            locals.var_tmpa_dn4 = assign5620_body0_e4893_d_n4;
            locals.var_tmpa_dn6 = assign5620_body0_e4893_d_n6;
            locals.var_tmpa_dn7 = assign5620_body0_e4893_d_n7;
            locals.var_tmpa_dn8 = assign5620_body0_e4893_d_n8;
            locals.var_tmpa_dn9 = assign5620_body0_e4893_d_n9;
            let (assign5620_body1_e4916,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5620_body1_e4905: f64 = (0.5 * p.p20);
        let assign5620_body1_e4906: f64 = (p.p27 + assign5620_body1_e4905);
        let assign5620_body1_e4910: f64 = (p.p28 + p.p20);
        let assign5620_body1_e4911: f64 = (locals.var_iloop * assign5620_body1_e4910);
        let assign5620_body1_e4912: f64 = (assign5620_body1_e4906 + assign5620_body1_e4911);
        let assign5620_body1_e4913: f64 = (1.0 / assign5620_body1_e4912);
        let assign5620_body1_e4914: f64 = (locals.var_tmpb + assign5620_body1_e4913);
        (assign5620_body1_e4914,)
    } else {
        (locals.var_tmpb,)
    }
};
            locals.var_tmpb = assign5620_body1_e4916;
            let (assign5620_body2_e4927,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5620_body2_e4925: f64 = (locals.var_iloop + 1.0);
        (assign5620_body2_e4925,)
    } else {
        (locals.var_iloop,)
    }
};
            locals.var_iloop = assign5620_body2_e4927;
        }

        let (assign5630_e4938, assign5630_e4938_d_n4, assign5630_e4938_d_n6, assign5630_e4938_d_n7, assign5630_e4938_d_n8, assign5630_e4938_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5630_e4936: f64 = (locals.var_tmpa / p.p29);
        (assign5630_e4936, (locals.var_tmpa_dn4 / p.p29), (locals.var_tmpa_dn6 / p.p29), (locals.var_tmpa_dn7 / p.p29), (locals.var_tmpa_dn8 / p.p29), (locals.var_tmpa_dn9 / p.p29),)
    } else {
        (locals.var_invsa, locals.var_invsa_dn4, locals.var_invsa_dn6, locals.var_invsa_dn7, locals.var_invsa_dn8, locals.var_invsa_dn9,)
    }
};
        locals.var_invsa = assign5630_e4938;
        locals.var_invsa_dn4 = assign5630_e4938_d_n4;
        locals.var_invsa_dn6 = assign5630_e4938_d_n6;
        locals.var_invsa_dn7 = assign5630_e4938_d_n7;
        locals.var_invsa_dn8 = assign5630_e4938_d_n8;
        locals.var_invsa_dn9 = assign5630_e4938_d_n9;

        let (assign5640_e4949,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5640_e4947: f64 = (locals.var_tmpb / p.p29);
        (assign5640_e4947,)
    } else {
        (locals.var_invsb,)
    }
};
        locals.var_invsb = assign5640_e4949;

        let (assign5650_e4964,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5650_e4960: f64 = (0.5 * p.p20);
        let assign5650_e4961: f64 = (p.p458 + assign5650_e4960);
        let assign5650_e4962: f64 = (1.0 / assign5650_e4961);
        (assign5650_e4962,)
    } else {
        (locals.var_invsaref,)
    }
};
        locals.var_invsaref = assign5650_e4964;

        let (assign5660_e4979,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5660_e4975: f64 = (0.5 * p.p20);
        let assign5660_e4976: f64 = (p.p459 + assign5660_e4975);
        let assign5660_e4977: f64 = (1.0 / assign5660_e4976);
        (assign5660_e4977,)
    } else {
        (locals.var_invsbref,)
    }
};
        locals.var_invsbref = assign5660_e4979;

        let (assign5670_e4992,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5670_e4988: f64 = (p.p20 + locals.var_dellps);
        let assign5670_e4990: f64 = (assign5670_e4988).max(1e-9);
        (assign5670_e4990,)
    } else {
        (locals.var_lx,)
    }
};
        locals.var_lx = assign5670_e4992;

        let (assign5680_e5007,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5680_e5001: f64 = (locals.var_w_i + locals.var_delwod);
        let assign5680_e5003: f64 = (assign5680_e5001 + p.p460);
        let assign5680_e5005: f64 = (assign5680_e5003).max(1e-9);
        (assign5680_e5005,)
    } else {
        (locals.var_wx,)
    }
};
        locals.var_wx = assign5680_e5007;

        let (assign5690_e5020,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5690_e5017: f64 = (locals.var_lx).powf(p.p467);
        let assign5690_e5018: f64 = (1.0 / assign5690_e5017);
        (assign5690_e5018,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign5690_e5020;

        let (assign5700_e5033,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5700_e5030: f64 = (locals.var_wx).powf(p.p468);
        let assign5700_e5031: f64 = (1.0 / assign5700_e5030);
        (assign5700_e5031,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign5700_e5033;

        let (assign5710_e5064, assign5710_e5064_d_n4, assign5710_e5064_d_n6, assign5710_e5064_d_n7, assign5710_e5064_d_n8, assign5710_e5064_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5710_e5043: f64 = (p.p464 * locals.var_templ);
        let assign5710_e5044: f64 = (1.0 + assign5710_e5043);
        let assign5710_e5047: f64 = (p.p465 * locals.var_tempw);
        let assign5710_e5048: f64 = (assign5710_e5044 + assign5710_e5047);
        let assign5710_e5051: f64 = (p.p466 * locals.var_templ);
        let assign5710_e5053: f64 = (assign5710_e5051 * locals.var_tempw);
        let assign5710_e5054: f64 = (assign5710_e5048 + assign5710_e5053);
        let assign5710_e5059: f64 = (locals.var_rt - 1.0);
        let assign5710_e5060: f64 = (p.p463 * assign5710_e5059);
        let assign5710_e5061: f64 = (1.0 + assign5710_e5060);
        let assign5710_e5062: f64 = (assign5710_e5054 * assign5710_e5061);
        (assign5710_e5062, (assign5710_e5054 * (p.p463 * locals.var_rt_dn4)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn6)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn7)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn8)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn9)),)
    } else {
        (locals.var_kstressu0, locals.var_kstressu0_dn4, locals.var_kstressu0_dn6, locals.var_kstressu0_dn7, locals.var_kstressu0_dn8, locals.var_kstressu0_dn9,)
    }
};
        locals.var_kstressu0 = assign5710_e5064;
        locals.var_kstressu0_dn4 = assign5710_e5064_d_n4;
        locals.var_kstressu0_dn6 = assign5710_e5064_d_n6;
        locals.var_kstressu0_dn7 = assign5710_e5064_d_n7;
        locals.var_kstressu0_dn8 = assign5710_e5064_d_n8;
        locals.var_kstressu0_dn9 = assign5710_e5064_d_n9;

        let (assign5720_e5079, assign5720_e5079_d_n4, assign5720_e5079_d_n6, assign5720_e5079_d_n7, assign5720_e5079_d_n8, assign5720_e5079_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5720_e5074: f64 = (locals.var_invsa + locals.var_invsb);
        let assign5720_e5075: f64 = (p.p461 * assign5720_e5074);
        let assign5720_e5077: f64 = (assign5720_e5075 / locals.var_kstressu0);
        (assign5720_e5077, ((((p.p461 * locals.var_invsa_dn4) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn4)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn6) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn6)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn7) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn7)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn8) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn8)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn9) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn9)) / (locals.var_kstressu0 * locals.var_kstressu0)),)
    } else {
        (locals.var_rhobeta, locals.var_rhobeta_dn4, locals.var_rhobeta_dn6, locals.var_rhobeta_dn7, locals.var_rhobeta_dn8, locals.var_rhobeta_dn9,)
    }
};
        locals.var_rhobeta = assign5720_e5079;
        locals.var_rhobeta_dn4 = assign5720_e5079_d_n4;
        locals.var_rhobeta_dn6 = assign5720_e5079_d_n6;
        locals.var_rhobeta_dn7 = assign5720_e5079_d_n7;
        locals.var_rhobeta_dn8 = assign5720_e5079_d_n8;
        locals.var_rhobeta_dn9 = assign5720_e5079_d_n9;

        let (assign5730_e5094, assign5730_e5094_d_n4, assign5730_e5094_d_n6, assign5730_e5094_d_n7, assign5730_e5094_d_n8, assign5730_e5094_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5730_e5089: f64 = (locals.var_invsaref + locals.var_invsbref);
        let assign5730_e5090: f64 = (p.p461 * assign5730_e5089);
        let assign5730_e5092: f64 = (assign5730_e5090 / locals.var_kstressu0);
        (assign5730_e5092, (-((assign5730_e5090 * locals.var_kstressu0_dn4) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn6) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn7) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn8) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn9) / (locals.var_kstressu0 * locals.var_kstressu0))),)
    } else {
        (locals.var_rhobetaref, locals.var_rhobetaref_dn4, locals.var_rhobetaref_dn6, locals.var_rhobetaref_dn7, locals.var_rhobetaref_dn8, locals.var_rhobetaref_dn9,)
    }
};
        locals.var_rhobetaref = assign5730_e5094;
        locals.var_rhobetaref_dn4 = assign5730_e5094_d_n4;
        locals.var_rhobetaref_dn6 = assign5730_e5094_d_n6;
        locals.var_rhobetaref_dn7 = assign5730_e5094_d_n7;
        locals.var_rhobetaref_dn8 = assign5730_e5094_d_n8;
        locals.var_rhobetaref_dn9 = assign5730_e5094_d_n9;

        let (assign5740_e5107,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5740_e5104: f64 = (locals.var_lx).powf(p.p473);
        let assign5740_e5105: f64 = (1.0 / assign5740_e5104);
        (assign5740_e5105,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign5740_e5107;

        let (assign5750_e5120,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5750_e5117: f64 = (locals.var_wx).powf(p.p474);
        let assign5750_e5118: f64 = (1.0 / assign5750_e5117);
        (assign5750_e5118,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign5750_e5120;

        let (assign5760_e5145,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5760_e5130: f64 = (p.p470 * locals.var_templ);
        let assign5760_e5131: f64 = (1.0 + assign5760_e5130);
        let assign5760_e5134: f64 = (p.p471 * locals.var_tempw);
        let assign5760_e5135: f64 = (assign5760_e5131 + assign5760_e5134);
        let assign5760_e5138: f64 = (p.p472 * locals.var_templ);
        let assign5760_e5140: f64 = (assign5760_e5138 * locals.var_tempw);
        let assign5760_e5141: f64 = (assign5760_e5135 + assign5760_e5140);
        let assign5760_e5143: f64 = (assign5760_e5141).max(1e-20);
        (assign5760_e5143,)
    } else {
        (locals.var_kstressvth0,)
    }
};
        locals.var_kstressvth0 = assign5760_e5145;

        let (assign5770_e5160, assign5770_e5160_d_n4, assign5770_e5160_d_n6, assign5770_e5160_d_n7, assign5770_e5160_d_n8, assign5770_e5160_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5770_e5154: f64 = (locals.var_invsa + locals.var_invsb);
        let assign5770_e5156: f64 = (assign5770_e5154 - locals.var_invsaref);
        let assign5770_e5158: f64 = (assign5770_e5156 - locals.var_invsbref);
        (assign5770_e5158, locals.var_invsa_dn4, locals.var_invsa_dn6, locals.var_invsa_dn7, locals.var_invsa_dn8, locals.var_invsa_dn9,)
    } else {
        (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9,)
    }
};
        locals.var_temp0__blk79 = assign5770_e5160;
        locals.var_temp0__blk79_dn4 = assign5770_e5160_d_n4;
        locals.var_temp0__blk79_dn6 = assign5770_e5160_d_n6;
        locals.var_temp0__blk79_dn7 = assign5770_e5160_d_n7;
        locals.var_temp0__blk79_dn8 = assign5770_e5160_d_n8;
        locals.var_temp0__blk79_dn9 = assign5770_e5160_d_n9;

        let (assign5780_e5177, assign5780_e5177_d_n4, assign5780_e5177_d_n6, assign5780_e5177_d_n7, assign5780_e5177_d_n8, assign5780_e5177_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5780_e5170: f64 = (1.0 + locals.var_rhobeta);
        let assign5780_e5171: f64 = (locals.var_betn_p * assign5780_e5170);
        let assign5780_e5174: f64 = (1.0 + locals.var_rhobetaref);
        let assign5780_e5175: f64 = (assign5780_e5171 / assign5780_e5174);
        (assign5780_e5175, (((((locals.var_betn_p_dn4 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn4)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn4)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn6 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn6)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn6)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn7 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn7)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn7)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn8 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn8)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn8)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn9 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn9)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn9)) / (assign5780_e5174 * assign5780_e5174)),)
    } else {
        (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9,)
    }
};
        locals.var_betn_p = assign5780_e5177;
        locals.var_betn_p_dn4 = assign5780_e5177_d_n4;
        locals.var_betn_p_dn6 = assign5780_e5177_d_n6;
        locals.var_betn_p_dn7 = assign5780_e5177_d_n7;
        locals.var_betn_p_dn8 = assign5780_e5177_d_n8;
        locals.var_betn_p_dn9 = assign5780_e5177_d_n9;

        let (assign5790_e5188, assign5790_e5188_d_n4, assign5790_e5188_d_n6, assign5790_e5188_d_n7, assign5790_e5188_d_n8, assign5790_e5188_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5790_e5186: f64 = (locals.var_betn_p).max(1e-10);
        (assign5790_e5186, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign5790_e5188;
        locals.var_betn1_t_dn4 = assign5790_e5188_d_n4;
        locals.var_betn1_t_dn6 = assign5790_e5188_d_n6;
        locals.var_betn1_t_dn7 = assign5790_e5188_d_n7;
        locals.var_betn1_t_dn8 = assign5790_e5188_d_n8;
        locals.var_betn1_t_dn9 = assign5790_e5188_d_n9;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5800_e5199, assign5800_e5199_d_n4, assign5800_e5199_d_n6, assign5800_e5199_d_n7, assign5800_e5199_d_n8, assign5800_e5199_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5800_e5197: f64 = (p.p250 * locals.var_betn1_t);
        (assign5800_e5197, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign5800_e5199;
        locals.var_betn2_t_dn4 = assign5800_e5199_d_n4;
        locals.var_betn2_t_dn6 = assign5800_e5199_d_n6;
        locals.var_betn2_t_dn7 = assign5800_e5199_d_n7;
        locals.var_betn2_t_dn8 = assign5800_e5199_d_n8;
        locals.var_betn2_t_dn9 = assign5800_e5199_d_n9;

        let (assign5810_e5226, assign5810_e5226_d_n4, assign5810_e5226_d_n6, assign5810_e5226_d_n7, assign5810_e5226_d_n8, assign5810_e5226_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5810_e5208: f64 = (1.0 + locals.var_rhobeta);
        let assign5810_e5212: f64 = (p.p462 * locals.var_rhobetaref);
        let assign5810_e5213: f64 = (1.0 + assign5810_e5212);
        let assign5810_e5214: f64 = (assign5810_e5208 * assign5810_e5213);
        let assign5810_e5217: f64 = (1.0 + locals.var_rhobetaref);
        let assign5810_e5221: f64 = (p.p462 * locals.var_rhobeta);
        let assign5810_e5222: f64 = (1.0 + assign5810_e5221);
        let assign5810_e5223: f64 = (assign5810_e5217 * assign5810_e5222);
        let assign5810_e5224: f64 = (assign5810_e5214 / assign5810_e5223);
        (assign5810_e5224, (((((locals.var_rhobeta_dn4 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn4))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn4 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn4))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn6 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn6))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn6 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn6))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn7 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn7))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn7 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn7))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn8 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn8))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn8 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn8))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn9 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn9))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn9 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn9))))) / (assign5810_e5223 * assign5810_e5223)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5810_e5226;
        locals.var_temp_dn4 = assign5810_e5226_d_n4;
        locals.var_temp_dn6 = assign5810_e5226_d_n6;
        locals.var_temp_dn7 = assign5810_e5226_d_n7;
        locals.var_temp_dn8 = assign5810_e5226_d_n8;
        locals.var_temp_dn9 = assign5810_e5226_d_n9;

        let (assign5820_e5237, assign5820_e5237_d_n4, assign5820_e5237_d_n6, assign5820_e5237_d_n7, assign5820_e5237_d_n8, assign5820_e5237_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5820_e5235: f64 = (locals.var_thesat_p * locals.var_temp);
        (assign5820_e5235, ((locals.var_thesat_p_dn4 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn4)), ((locals.var_thesat_p_dn6 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn6)), ((locals.var_thesat_p_dn7 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn7)), ((locals.var_thesat_p_dn8 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn8)), ((locals.var_thesat_p_dn9 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn9)),)
    } else {
        (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9,)
    }
};
        locals.var_thesat_p = assign5820_e5237;
        locals.var_thesat_p_dn4 = assign5820_e5237_d_n4;
        locals.var_thesat_p_dn6 = assign5820_e5237_d_n6;
        locals.var_thesat_p_dn7 = assign5820_e5237_d_n7;
        locals.var_thesat_p_dn8 = assign5820_e5237_d_n8;
        locals.var_thesat_p_dn9 = assign5820_e5237_d_n9;

        let (assign5830_e5248, assign5830_e5248_d_n4, assign5830_e5248_d_n6, assign5830_e5248_d_n7, assign5830_e5248_d_n8, assign5830_e5248_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5830_e5246: f64 = (locals.var_thesat_p).max(0.0);
        (assign5830_e5246, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign5830_e5248;
        locals.var_thesat_t_dn4 = assign5830_e5248_d_n4;
        locals.var_thesat_t_dn6 = assign5830_e5248_d_n6;
        locals.var_thesat_t_dn7 = assign5830_e5248_d_n7;
        locals.var_thesat_t_dn8 = assign5830_e5248_d_n8;
        locals.var_thesat_t_dn9 = assign5830_e5248_d_n9;

        let (assign5840_e5259, assign5840_e5259_d_n4, assign5840_e5259_d_n6, assign5840_e5259_d_n7, assign5840_e5259_d_n8, assign5840_e5259_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5840_e5257: f64 = (locals.var_thesatac_p * locals.var_temp);
        (assign5840_e5257, ((locals.var_thesatac_p_dn4 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn4)), ((locals.var_thesatac_p_dn6 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn6)), ((locals.var_thesatac_p_dn7 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn7)), ((locals.var_thesatac_p_dn8 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn8)), ((locals.var_thesatac_p_dn9 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn9)),)
    } else {
        (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9,)
    }
};
        locals.var_thesatac_p = assign5840_e5259;
        locals.var_thesatac_p_dn4 = assign5840_e5259_d_n4;
        locals.var_thesatac_p_dn6 = assign5840_e5259_d_n6;
        locals.var_thesatac_p_dn7 = assign5840_e5259_d_n7;
        locals.var_thesatac_p_dn8 = assign5840_e5259_d_n8;
        locals.var_thesatac_p_dn9 = assign5840_e5259_d_n9;

        let (assign5850_e5270, assign5850_e5270_d_n4, assign5850_e5270_d_n6, assign5850_e5270_d_n7, assign5850_e5270_d_n8, assign5850_e5270_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5850_e5268: f64 = (locals.var_thesatac_p).max(0.0);
        (assign5850_e5268, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign5850_e5270;
        locals.var_thesatac_t_dn4 = assign5850_e5270_d_n4;
        locals.var_thesatac_t_dn6 = assign5850_e5270_d_n6;
        locals.var_thesatac_t_dn7 = assign5850_e5270_d_n7;
        locals.var_thesatac_t_dn8 = assign5850_e5270_d_n8;
        locals.var_thesatac_t_dn9 = assign5850_e5270_d_n9;

        let (assign5860_e5283, assign5860_e5283_d_n4, assign5860_e5283_d_n6, assign5860_e5283_d_n7, assign5860_e5283_d_n8, assign5860_e5283_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5860_e5279: f64 = (p.p469 * locals.var_temp0__blk79);
        let assign5860_e5281: f64 = (assign5860_e5279 / locals.var_kstressvth0);
        (assign5860_e5281, ((p.p469 * locals.var_temp0__blk79_dn4) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn6) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn7) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn8) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn9) / locals.var_kstressvth0),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5860_e5283;
        locals.var_temp_dn4 = assign5860_e5283_d_n4;
        locals.var_temp_dn6 = assign5860_e5283_d_n6;
        locals.var_temp_dn7 = assign5860_e5283_d_n7;
        locals.var_temp_dn8 = assign5860_e5283_d_n8;
        locals.var_temp_dn9 = assign5860_e5283_d_n9;

        let (assign5870_e5294, assign5870_e5294_d_n4, assign5870_e5294_d_n6, assign5870_e5294_d_n7, assign5870_e5294_d_n8, assign5870_e5294_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5870_e5292: f64 = (locals.var_vfb1_t + locals.var_temp);
        (assign5870_e5292, (locals.var_vfb1_t_dn4 + locals.var_temp_dn4), (locals.var_vfb1_t_dn6 + locals.var_temp_dn6), (locals.var_vfb1_t_dn7 + locals.var_temp_dn7), (locals.var_vfb1_t_dn8 + locals.var_temp_dn8), (locals.var_vfb1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign5870_e5294;
        locals.var_vfb1_t_dn4 = assign5870_e5294_d_n4;
        locals.var_vfb1_t_dn6 = assign5870_e5294_d_n6;
        locals.var_vfb1_t_dn7 = assign5870_e5294_d_n7;
        locals.var_vfb1_t_dn8 = assign5870_e5294_d_n8;
        locals.var_vfb1_t_dn9 = assign5870_e5294_d_n9;

        let (assign5880_e5305, assign5880_e5305_d_n4, assign5880_e5305_d_n6, assign5880_e5305_d_n7, assign5880_e5305_d_n8, assign5880_e5305_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5880_e5303: f64 = (locals.var_vfb2_t + locals.var_temp);
        (assign5880_e5303, (locals.var_vfb2_t_dn4 + locals.var_temp_dn4), (locals.var_vfb2_t_dn6 + locals.var_temp_dn6), (locals.var_vfb2_t_dn7 + locals.var_temp_dn7), (locals.var_vfb2_t_dn8 + locals.var_temp_dn8), (locals.var_vfb2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign5880_e5305;
        locals.var_vfb2_t_dn4 = assign5880_e5305_d_n4;
        locals.var_vfb2_t_dn6 = assign5880_e5305_d_n6;
        locals.var_vfb2_t_dn7 = assign5880_e5305_d_n7;
        locals.var_vfb2_t_dn8 = assign5880_e5305_d_n8;
        locals.var_vfb2_t_dn9 = assign5880_e5305_d_n9;

        let (assign5890_e5316, assign5890_e5316_d_n4, assign5890_e5316_d_n6, assign5890_e5316_d_n7, assign5890_e5316_d_n8, assign5890_e5316_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5890_e5314: f64 = (locals.var_vfbac1_t + locals.var_temp);
        (assign5890_e5314, (locals.var_vfbac1_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac1_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac1_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac1_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign5890_e5316;
        locals.var_vfbac1_t_dn4 = assign5890_e5316_d_n4;
        locals.var_vfbac1_t_dn6 = assign5890_e5316_d_n6;
        locals.var_vfbac1_t_dn7 = assign5890_e5316_d_n7;
        locals.var_vfbac1_t_dn8 = assign5890_e5316_d_n8;
        locals.var_vfbac1_t_dn9 = assign5890_e5316_d_n9;

        let (assign5900_e5327, assign5900_e5327_d_n4, assign5900_e5327_d_n6, assign5900_e5327_d_n7, assign5900_e5327_d_n8, assign5900_e5327_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5900_e5325: f64 = (locals.var_vfbac2_t + locals.var_temp);
        (assign5900_e5325, (locals.var_vfbac2_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac2_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac2_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac2_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign5900_e5327;
        locals.var_vfbac2_t_dn4 = assign5900_e5327_d_n4;
        locals.var_vfbac2_t_dn6 = assign5900_e5327_d_n6;
        locals.var_vfbac2_t_dn7 = assign5900_e5327_d_n7;
        locals.var_vfbac2_t_dn8 = assign5900_e5327_d_n8;
        locals.var_vfbac2_t_dn9 = assign5900_e5327_d_n9;

        let (assign5910_e5342, assign5910_e5342_d_n4, assign5910_e5342_d_n6, assign5910_e5342_d_n7, assign5910_e5342_d_n8, assign5910_e5342_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5910_e5336: f64 = (p.p475 * locals.var_temp0__blk79);
        let assign5910_e5339: f64 = (locals.var_kstressvth0).powf(p.p476);
        let assign5910_e5340: f64 = (assign5910_e5336 / assign5910_e5339);
        (assign5910_e5340, ((p.p475 * locals.var_temp0__blk79_dn4) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn6) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn7) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn8) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn9) / assign5910_e5339),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5910_e5342;
        locals.var_temp_dn4 = assign5910_e5342_d_n4;
        locals.var_temp_dn6 = assign5910_e5342_d_n6;
        locals.var_temp_dn7 = assign5910_e5342_d_n7;
        locals.var_temp_dn8 = assign5910_e5342_d_n8;
        locals.var_temp_dn9 = assign5910_e5342_d_n9;

        let (assign5920_e5353, assign5920_e5353_d_n4, assign5920_e5353_d_n6, assign5920_e5353_d_n7, assign5920_e5353_d_n8, assign5920_e5353_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5920_e5351: f64 = (locals.var_cf_p + locals.var_temp);
        (assign5920_e5351, (locals.var_cf_p_dn4 + locals.var_temp_dn4), (locals.var_cf_p_dn6 + locals.var_temp_dn6), (locals.var_cf_p_dn7 + locals.var_temp_dn7), (locals.var_cf_p_dn8 + locals.var_temp_dn8), (locals.var_cf_p_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9,)
    }
};
        locals.var_cf_p = assign5920_e5353;
        locals.var_cf_p_dn4 = assign5920_e5353_d_n4;
        locals.var_cf_p_dn6 = assign5920_e5353_d_n6;
        locals.var_cf_p_dn7 = assign5920_e5353_d_n7;
        locals.var_cf_p_dn8 = assign5920_e5353_d_n8;
        locals.var_cf_p_dn9 = assign5920_e5353_d_n9;

        let (assign5930_e5364, assign5930_e5364_d_n4, assign5930_e5364_d_n6, assign5930_e5364_d_n7, assign5930_e5364_d_n8, assign5930_e5364_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5930_e5362: f64 = (locals.var_cf_p).max(0.0);
        (assign5930_e5362, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign5930_e5364;
        locals.var_cf1_t_dn4 = assign5930_e5364_d_n4;
        locals.var_cf1_t_dn6 = assign5930_e5364_d_n6;
        locals.var_cf1_t_dn7 = assign5930_e5364_d_n7;
        locals.var_cf1_t_dn8 = assign5930_e5364_d_n8;
        locals.var_cf1_t_dn9 = assign5930_e5364_d_n9;

        let (assign5940_e5375, assign5940_e5375_d_n4, assign5940_e5375_d_n6, assign5940_e5375_d_n7, assign5940_e5375_d_n8, assign5940_e5375_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5940_e5373: f64 = (locals.var_cfac_p + locals.var_temp);
        (assign5940_e5373, (locals.var_cfac_p_dn4 + locals.var_temp_dn4), (locals.var_cfac_p_dn6 + locals.var_temp_dn6), (locals.var_cfac_p_dn7 + locals.var_temp_dn7), (locals.var_cfac_p_dn8 + locals.var_temp_dn8), (locals.var_cfac_p_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9,)
    }
};
        locals.var_cfac_p = assign5940_e5375;
        locals.var_cfac_p_dn4 = assign5940_e5375_d_n4;
        locals.var_cfac_p_dn6 = assign5940_e5375_d_n6;
        locals.var_cfac_p_dn7 = assign5940_e5375_d_n7;
        locals.var_cfac_p_dn8 = assign5940_e5375_d_n8;
        locals.var_cfac_p_dn9 = assign5940_e5375_d_n9;

        let (assign5950_e5386, assign5950_e5386_d_n4, assign5950_e5386_d_n6, assign5950_e5386_d_n7, assign5950_e5386_d_n8, assign5950_e5386_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5950_e5384: f64 = (locals.var_cfac_p).max(0.0);
        (assign5950_e5384, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign5950_e5386;
        locals.var_cfac1_t_dn4 = assign5950_e5386_d_n4;
        locals.var_cfac1_t_dn6 = assign5950_e5386_d_n6;
        locals.var_cfac1_t_dn7 = assign5950_e5386_d_n7;
        locals.var_cfac1_t_dn8 = assign5950_e5386_d_n8;
        locals.var_cfac1_t_dn9 = assign5950_e5386_d_n9;

        let (assign5960_e5399, assign5960_e5399_d_n4, assign5960_e5399_d_n6, assign5960_e5399_d_n7, assign5960_e5399_d_n8, assign5960_e5399_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5960_e5395: f64 = (p.p234 * locals.var_tox2_i);
        let assign5960_e5397: f64 = (assign5960_e5395 / locals.var_tox1_i);
        (assign5960_e5397, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign5960_e5399;
        locals.var_temp_dn4 = assign5960_e5399_d_n4;
        locals.var_temp_dn6 = assign5960_e5399_d_n6;
        locals.var_temp_dn7 = assign5960_e5399_d_n7;
        locals.var_temp_dn8 = assign5960_e5399_d_n8;
        locals.var_temp_dn9 = assign5960_e5399_d_n9;

        let (assign5970_e5410, assign5970_e5410_d_n4, assign5970_e5410_d_n6, assign5970_e5410_d_n7, assign5970_e5410_d_n8, assign5970_e5410_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5970_e5408: f64 = (locals.var_cf1_t * locals.var_temp);
        (assign5970_e5408, ((locals.var_cf1_t_dn4 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn4)), ((locals.var_cf1_t_dn6 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn6)), ((locals.var_cf1_t_dn7 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn7)), ((locals.var_cf1_t_dn8 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn8)), ((locals.var_cf1_t_dn9 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn9)),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign5970_e5410;
        locals.var_cf2_t_dn4 = assign5970_e5410_d_n4;
        locals.var_cf2_t_dn6 = assign5970_e5410_d_n6;
        locals.var_cf2_t_dn7 = assign5970_e5410_d_n7;
        locals.var_cf2_t_dn8 = assign5970_e5410_d_n8;
        locals.var_cf2_t_dn9 = assign5970_e5410_d_n9;

        let (assign5980_e5421, assign5980_e5421_d_n4, assign5980_e5421_d_n6, assign5980_e5421_d_n7, assign5980_e5421_d_n8, assign5980_e5421_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
        let assign5980_e5419: f64 = (locals.var_cfac1_t * locals.var_temp);
        (assign5980_e5419, ((locals.var_cfac1_t_dn4 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn4)), ((locals.var_cfac1_t_dn6 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn6)), ((locals.var_cfac1_t_dn7 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn7)), ((locals.var_cfac1_t_dn8 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn8)), ((locals.var_cfac1_t_dn9 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn9)),)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign5980_e5421;
        locals.var_cfac2_t_dn4 = assign5980_e5421_d_n4;
        locals.var_cfac2_t_dn6 = assign5980_e5421_d_n6;
        locals.var_cfac2_t_dn7 = assign5980_e5421_d_n7;
        locals.var_cfac2_t_dn8 = assign5980_e5421_d_n8;
        locals.var_cfac2_t_dn9 = assign5980_e5421_d_n9;

        let (assign5990_e5431, assign5990_e5431_d_n4, assign5990_e5431_d_n6, assign5990_e5431_d_n7, assign5990_e5431_d_n8, assign5990_e5431_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    }
};
        locals.var_tmpa = assign5990_e5431;
        locals.var_tmpa_dn4 = assign5990_e5431_d_n4;
        locals.var_tmpa_dn6 = assign5990_e5431_d_n6;
        locals.var_tmpa_dn7 = assign5990_e5431_d_n7;
        locals.var_tmpa_dn8 = assign5990_e5431_d_n8;
        locals.var_tmpa_dn9 = assign5990_e5431_d_n9;

        let (assign6000_e5441,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_iloop,)
    }
};
        locals.var_iloop = assign6000_e5441;

        let (assign6010_e5454, assign6010_e5454_d_n4, assign6010_e5454_d_n6, assign6010_e5454_d_n7, assign6010_e5454_d_n8, assign6010_e5454_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6010_e5450: f64 = (-1.0);
        let assign6010_e5452: f64 = (assign6010_e5450 / p.p478);
        (assign6010_e5452, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6010_e5454;
        locals.var_temp_dn4 = assign6010_e5454_d_n4;
        locals.var_temp_dn6 = assign6010_e5454_d_n6;
        locals.var_temp_dn7 = assign6010_e5454_d_n7;
        locals.var_temp_dn8 = assign6010_e5454_d_n8;
        locals.var_temp_dn9 = assign6010_e5454_d_n9;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign6020_loop_guard: usize = 0;
        while {
            let assign6020_cond_e5465: f64 = (p.p29 - 0.5);
            let assign6020_cond_e5467: f64 = if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_iloop < assign6020_cond_e5465)) { 1.0 } else { 0.0 };
            assign6020_cond_e5467 != 0.0
        } {
            assign6020_loop_guard += 1;
            assert!(assign6020_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let assign6020_body0_e5471: f64 = (0.5 * p.p20);
            let assign6020_body0_e5472: f64 = (p.p26 + assign6020_body0_e5471);
            let assign6020_body0_e5476: f64 = (p.p28 + p.p20);
            let assign6020_body0_e5477: f64 = (locals.var_iloop * assign6020_body0_e5476);
            let assign6020_body0_e5478: f64 = (assign6020_body0_e5472 + assign6020_body0_e5477);
            let assign6020_body0_e5479: f64 = (-assign6020_body0_e5478);
            let assign6020_body0_e5481: f64 = (assign6020_body0_e5479 / p.p477);
            let assign6020_body0_e5483: f64 = (-80.0);
            let assign6020_body0_e5484: f64 = if assign6020_body0_e5481 > assign6020_body0_e5483 { 1.0 } else { 0.0 };
            locals.var_guard135 = assign6020_body0_e5484;
            let (assign6020_body1_e5510, assign6020_body1_e5510_d_n4, assign6020_body1_e5510_d_n6, assign6020_body1_e5510_d_n7, assign6020_body1_e5510_d_n8, assign6020_body1_e5510_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard135 != 0.0)) {
        let assign6020_body1_e5497: f64 = (0.5 * p.p20);
        let assign6020_body1_e5498: f64 = (p.p26 + assign6020_body1_e5497);
        let assign6020_body1_e5502: f64 = (p.p28 + p.p20);
        let assign6020_body1_e5503: f64 = (locals.var_iloop * assign6020_body1_e5502);
        let assign6020_body1_e5504: f64 = (assign6020_body1_e5498 + assign6020_body1_e5503);
        let assign6020_body1_e5505: f64 = (-assign6020_body1_e5504);
        let assign6020_body1_e5507: f64 = (assign6020_body1_e5505 / p.p477);
        let assign6020_body1_e5508: f64 = (assign6020_body1_e5507).exp();
        (assign6020_body1_e5508, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
            locals.var_temp1 = assign6020_body1_e5510;
            locals.var_temp1_dn4 = assign6020_body1_e5510_d_n4;
            locals.var_temp1_dn6 = assign6020_body1_e5510_d_n6;
            locals.var_temp1_dn7 = assign6020_body1_e5510_d_n7;
            locals.var_temp1_dn8 = assign6020_body1_e5510_d_n8;
            locals.var_temp1_dn9 = assign6020_body1_e5510_d_n9;
            let (assign6020_body2_e5587, assign6020_body2_e5587_d_n4, assign6020_body2_e5587_d_n6, assign6020_body2_e5587_d_n7, assign6020_body2_e5587_d_n8, assign6020_body2_e5587_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard135 == 0.0)) {
        let assign6020_body2_e5526: f64 = (0.5 * p.p20);
        let assign6020_body2_e5527: f64 = (p.p26 + assign6020_body2_e5526);
        let assign6020_body2_e5531: f64 = (p.p28 + p.p20);
        let assign6020_body2_e5532: f64 = (locals.var_iloop * assign6020_body2_e5531);
        let assign6020_body2_e5533: f64 = (assign6020_body2_e5527 + assign6020_body2_e5532);
        let assign6020_body2_e5534: f64 = (-assign6020_body2_e5533);
        let assign6020_body2_e5536: f64 = (assign6020_body2_e5534 / p.p477);
        let assign6020_body2_e5537: f64 = (-assign6020_body2_e5536);
        let assign6020_body2_e5539: f64 = (assign6020_body2_e5537 - 80.0);
        let assign6020_body2_e5545: f64 = (0.5 * p.p20);
        let assign6020_body2_e5546: f64 = (p.p26 + assign6020_body2_e5545);
        let assign6020_body2_e5550: f64 = (p.p28 + p.p20);
        let assign6020_body2_e5551: f64 = (locals.var_iloop * assign6020_body2_e5550);
        let assign6020_body2_e5552: f64 = (assign6020_body2_e5546 + assign6020_body2_e5551);
        let assign6020_body2_e5553: f64 = (-assign6020_body2_e5552);
        let assign6020_body2_e5555: f64 = (assign6020_body2_e5553 / p.p477);
        let assign6020_body2_e5556: f64 = (-assign6020_body2_e5555);
        let assign6020_body2_e5558: f64 = (assign6020_body2_e5556 - 80.0);
        let assign6020_body2_e5559: f64 = (0.5 * assign6020_body2_e5558);
        let assign6020_body2_e5564: f64 = (0.5 * p.p20);
        let assign6020_body2_e5565: f64 = (p.p26 + assign6020_body2_e5564);
        let assign6020_body2_e5569: f64 = (p.p28 + p.p20);
        let assign6020_body2_e5570: f64 = (locals.var_iloop * assign6020_body2_e5569);
        let assign6020_body2_e5571: f64 = (assign6020_body2_e5565 + assign6020_body2_e5570);
        let assign6020_body2_e5572: f64 = (-assign6020_body2_e5571);
        let assign6020_body2_e5574: f64 = (assign6020_body2_e5572 / p.p477);
        let assign6020_body2_e5575: f64 = (-assign6020_body2_e5574);
        let assign6020_body2_e5577: f64 = (assign6020_body2_e5575 - 80.0);
        let assign6020_body2_e5579: f64 = (assign6020_body2_e5577 * 0.3333333333333);
        let assign6020_body2_e5580: f64 = (1.0 + assign6020_body2_e5579);
        let assign6020_body2_e5581: f64 = (assign6020_body2_e5559 * assign6020_body2_e5580);
        let assign6020_body2_e5582: f64 = (1.0 + assign6020_body2_e5581);
        let assign6020_body2_e5583: f64 = (assign6020_body2_e5539 * assign6020_body2_e5582);
        let assign6020_body2_e5584: f64 = (1.0 + assign6020_body2_e5583);
        let assign6020_body2_e5585: f64 = (1.80485e-35 / assign6020_body2_e5584);
        (assign6020_body2_e5585, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
            locals.var_temp1 = assign6020_body2_e5587;
            locals.var_temp1_dn4 = assign6020_body2_e5587_d_n4;
            locals.var_temp1_dn6 = assign6020_body2_e5587_d_n6;
            locals.var_temp1_dn7 = assign6020_body2_e5587_d_n7;
            locals.var_temp1_dn8 = assign6020_body2_e5587_d_n8;
            locals.var_temp1_dn9 = assign6020_body2_e5587_d_n9;
            let assign6020_body3_e5591: f64 = (0.5 * p.p20);
            let assign6020_body3_e5592: f64 = (p.p27 + assign6020_body3_e5591);
            let assign6020_body3_e5595: f64 = (p.p29 - 1.0);
            let assign6020_body3_e5597: f64 = (assign6020_body3_e5595 - locals.var_iloop);
            let assign6020_body3_e5600: f64 = (p.p28 + p.p20);
            let assign6020_body3_e5601: f64 = (assign6020_body3_e5597 * assign6020_body3_e5600);
            let assign6020_body3_e5602: f64 = (assign6020_body3_e5592 + assign6020_body3_e5601);
            let assign6020_body3_e5603: f64 = (-assign6020_body3_e5602);
            let assign6020_body3_e5605: f64 = (assign6020_body3_e5603 / p.p477);
            let assign6020_body3_e5607: f64 = (-80.0);
            let assign6020_body3_e5608: f64 = if assign6020_body3_e5605 > assign6020_body3_e5607 { 1.0 } else { 0.0 };
            locals.var_guard136 = assign6020_body3_e5608;
            let (assign6020_body4_e5638, assign6020_body4_e5638_d_n4, assign6020_body4_e5638_d_n6, assign6020_body4_e5638_d_n7, assign6020_body4_e5638_d_n8, assign6020_body4_e5638_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard136 != 0.0)) {
        let assign6020_body4_e5621: f64 = (0.5 * p.p20);
        let assign6020_body4_e5622: f64 = (p.p27 + assign6020_body4_e5621);
        let assign6020_body4_e5625: f64 = (p.p29 - 1.0);
        let assign6020_body4_e5627: f64 = (assign6020_body4_e5625 - locals.var_iloop);
        let assign6020_body4_e5630: f64 = (p.p28 + p.p20);
        let assign6020_body4_e5631: f64 = (assign6020_body4_e5627 * assign6020_body4_e5630);
        let assign6020_body4_e5632: f64 = (assign6020_body4_e5622 + assign6020_body4_e5631);
        let assign6020_body4_e5633: f64 = (-assign6020_body4_e5632);
        let assign6020_body4_e5635: f64 = (assign6020_body4_e5633 / p.p477);
        let assign6020_body4_e5636: f64 = (assign6020_body4_e5635).exp();
        (assign6020_body4_e5636, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
            locals.var_temp2 = assign6020_body4_e5638;
            locals.var_temp2_dn4 = assign6020_body4_e5638_d_n4;
            locals.var_temp2_dn6 = assign6020_body4_e5638_d_n6;
            locals.var_temp2_dn7 = assign6020_body4_e5638_d_n7;
            locals.var_temp2_dn8 = assign6020_body4_e5638_d_n8;
            locals.var_temp2_dn9 = assign6020_body4_e5638_d_n9;
            let (assign6020_body5_e5727, assign6020_body5_e5727_d_n4, assign6020_body5_e5727_d_n6, assign6020_body5_e5727_d_n7, assign6020_body5_e5727_d_n8, assign6020_body5_e5727_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard136 == 0.0)) {
        let assign6020_body5_e5654: f64 = (0.5 * p.p20);
        let assign6020_body5_e5655: f64 = (p.p27 + assign6020_body5_e5654);
        let assign6020_body5_e5658: f64 = (p.p29 - 1.0);
        let assign6020_body5_e5660: f64 = (assign6020_body5_e5658 - locals.var_iloop);
        let assign6020_body5_e5663: f64 = (p.p28 + p.p20);
        let assign6020_body5_e5664: f64 = (assign6020_body5_e5660 * assign6020_body5_e5663);
        let assign6020_body5_e5665: f64 = (assign6020_body5_e5655 + assign6020_body5_e5664);
        let assign6020_body5_e5666: f64 = (-assign6020_body5_e5665);
        let assign6020_body5_e5668: f64 = (assign6020_body5_e5666 / p.p477);
        let assign6020_body5_e5669: f64 = (-assign6020_body5_e5668);
        let assign6020_body5_e5671: f64 = (assign6020_body5_e5669 - 80.0);
        let assign6020_body5_e5677: f64 = (0.5 * p.p20);
        let assign6020_body5_e5678: f64 = (p.p27 + assign6020_body5_e5677);
        let assign6020_body5_e5681: f64 = (p.p29 - 1.0);
        let assign6020_body5_e5683: f64 = (assign6020_body5_e5681 - locals.var_iloop);
        let assign6020_body5_e5686: f64 = (p.p28 + p.p20);
        let assign6020_body5_e5687: f64 = (assign6020_body5_e5683 * assign6020_body5_e5686);
        let assign6020_body5_e5688: f64 = (assign6020_body5_e5678 + assign6020_body5_e5687);
        let assign6020_body5_e5689: f64 = (-assign6020_body5_e5688);
        let assign6020_body5_e5691: f64 = (assign6020_body5_e5689 / p.p477);
        let assign6020_body5_e5692: f64 = (-assign6020_body5_e5691);
        let assign6020_body5_e5694: f64 = (assign6020_body5_e5692 - 80.0);
        let assign6020_body5_e5695: f64 = (0.5 * assign6020_body5_e5694);
        let assign6020_body5_e5700: f64 = (0.5 * p.p20);
        let assign6020_body5_e5701: f64 = (p.p27 + assign6020_body5_e5700);
        let assign6020_body5_e5704: f64 = (p.p29 - 1.0);
        let assign6020_body5_e5706: f64 = (assign6020_body5_e5704 - locals.var_iloop);
        let assign6020_body5_e5709: f64 = (p.p28 + p.p20);
        let assign6020_body5_e5710: f64 = (assign6020_body5_e5706 * assign6020_body5_e5709);
        let assign6020_body5_e5711: f64 = (assign6020_body5_e5701 + assign6020_body5_e5710);
        let assign6020_body5_e5712: f64 = (-assign6020_body5_e5711);
        let assign6020_body5_e5714: f64 = (assign6020_body5_e5712 / p.p477);
        let assign6020_body5_e5715: f64 = (-assign6020_body5_e5714);
        let assign6020_body5_e5717: f64 = (assign6020_body5_e5715 - 80.0);
        let assign6020_body5_e5719: f64 = (assign6020_body5_e5717 * 0.3333333333333);
        let assign6020_body5_e5720: f64 = (1.0 + assign6020_body5_e5719);
        let assign6020_body5_e5721: f64 = (assign6020_body5_e5695 * assign6020_body5_e5720);
        let assign6020_body5_e5722: f64 = (1.0 + assign6020_body5_e5721);
        let assign6020_body5_e5723: f64 = (assign6020_body5_e5671 * assign6020_body5_e5722);
        let assign6020_body5_e5724: f64 = (1.0 + assign6020_body5_e5723);
        let assign6020_body5_e5725: f64 = (1.80485e-35 / assign6020_body5_e5724);
        (assign6020_body5_e5725, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
            locals.var_temp2 = assign6020_body5_e5727;
            locals.var_temp2_dn4 = assign6020_body5_e5727_d_n4;
            locals.var_temp2_dn6 = assign6020_body5_e5727_d_n6;
            locals.var_temp2_dn7 = assign6020_body5_e5727_d_n7;
            locals.var_temp2_dn8 = assign6020_body5_e5727_d_n8;
            locals.var_temp2_dn9 = assign6020_body5_e5727_d_n9;
            let (assign6020_body6_e5742, assign6020_body6_e5742_d_n4, assign6020_body6_e5742_d_n6, assign6020_body6_e5742_d_n7, assign6020_body6_e5742_d_n8, assign6020_body6_e5742_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6020_body6_e5737: f64 = (1.0 - locals.var_temp1);
        let assign6020_body6_e5739: f64 = (-p.p478);
        let assign6020_body6_e5740: f64 = (assign6020_body6_e5737).powf(assign6020_body6_e5739);
        (assign6020_body6_e5740, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn4))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn4) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn6))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn6) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn7))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn7) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn8))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn8) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn9))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn9) / assign6020_body6_e5737))) },)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
            locals.var_temp3 = assign6020_body6_e5742;
            locals.var_temp3_dn4 = assign6020_body6_e5742_d_n4;
            locals.var_temp3_dn6 = assign6020_body6_e5742_d_n6;
            locals.var_temp3_dn7 = assign6020_body6_e5742_d_n7;
            locals.var_temp3_dn8 = assign6020_body6_e5742_d_n8;
            locals.var_temp3_dn9 = assign6020_body6_e5742_d_n9;
            let (assign6020_body7_e5757, assign6020_body7_e5757_d_n4, assign6020_body7_e5757_d_n6, assign6020_body7_e5757_d_n7, assign6020_body7_e5757_d_n8, assign6020_body7_e5757_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6020_body7_e5752: f64 = (1.0 - locals.var_temp2);
        let assign6020_body7_e5754: f64 = (-p.p478);
        let assign6020_body7_e5755: f64 = (assign6020_body7_e5752).powf(assign6020_body7_e5754);
        (assign6020_body7_e5755, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn4))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn4) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn6))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn6) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn7))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn7) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn8))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn8) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn9))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn9) / assign6020_body7_e5752))) },)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
            locals.var_temp4 = assign6020_body7_e5757;
            locals.var_temp4_dn4 = assign6020_body7_e5757_d_n4;
            locals.var_temp4_dn6 = assign6020_body7_e5757_d_n6;
            locals.var_temp4_dn7 = assign6020_body7_e5757_d_n7;
            locals.var_temp4_dn8 = assign6020_body7_e5757_d_n8;
            locals.var_temp4_dn9 = assign6020_body7_e5757_d_n9;
            let (assign6020_body8_e5775, assign6020_body8_e5775_d_n4, assign6020_body8_e5775_d_n6, assign6020_body8_e5775_d_n7, assign6020_body8_e5775_d_n8, assign6020_body8_e5775_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6020_body8_e5769: f64 = (locals.var_temp3 + locals.var_temp4);
        let assign6020_body8_e5770: f64 = (0.5 * assign6020_body8_e5769);
        let assign6020_body8_e5772: f64 = (assign6020_body8_e5770).powf(locals.var_temp);
        let assign6020_body8_e5773: f64 = (locals.var_tmpa + assign6020_body8_e5772);
        (assign6020_body8_e5773, (locals.var_tmpa_dn4 + if locals.var_temp_dn4 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn4 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn6 + if locals.var_temp_dn6 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn6 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn7 + if locals.var_temp_dn7 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn7 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn8 + if locals.var_temp_dn8 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn8 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn9 + if locals.var_temp_dn9 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn9 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)) / assign6020_body8_e5770)))) }),)
    } else {
        (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9,)
    }
};
            locals.var_tmpa = assign6020_body8_e5775;
            locals.var_tmpa_dn4 = assign6020_body8_e5775_d_n4;
            locals.var_tmpa_dn6 = assign6020_body8_e5775_d_n6;
            locals.var_tmpa_dn7 = assign6020_body8_e5775_d_n7;
            locals.var_tmpa_dn8 = assign6020_body8_e5775_d_n8;
            locals.var_tmpa_dn9 = assign6020_body8_e5775_d_n9;
            let (assign6020_body9_e5787,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6020_body9_e5785: f64 = (locals.var_iloop + 1.0);
        (assign6020_body9_e5785,)
    } else {
        (locals.var_iloop,)
    }
};
            locals.var_iloop = assign6020_body9_e5787;
        }

        let (assign6030_e5801, assign6030_e5801_d_n4, assign6030_e5801_d_n6, assign6030_e5801_d_n7, assign6030_e5801_d_n8, assign6030_e5801_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6030_e5798: f64 = (locals.var_tmpa / p.p29);
        let assign6030_e5799: f64 = (1.0 - assign6030_e5798);
        (assign6030_e5799, (-(locals.var_tmpa_dn4 / p.p29)), (-(locals.var_tmpa_dn6 / p.p29)), (-(locals.var_tmpa_dn7 / p.p29)), (-(locals.var_tmpa_dn8 / p.p29)), (-(locals.var_tmpa_dn9 / p.p29)),)
    } else {
        (locals.var_str_g, locals.var_str_g_dn4, locals.var_str_g_dn6, locals.var_str_g_dn7, locals.var_str_g_dn8, locals.var_str_g_dn9,)
    }
};
        locals.var_str_g = assign6030_e5801;
        locals.var_str_g_dn4 = assign6030_e5801_d_n4;
        locals.var_str_g_dn6 = assign6030_e5801_d_n6;
        locals.var_str_g_dn7 = assign6030_e5801_d_n7;
        locals.var_str_g_dn8 = assign6030_e5801_d_n8;
        locals.var_str_g_dn9 = assign6030_e5801_d_n9;

        let assign6040_e5805: f64 = (0.5 * p.p20);
        let assign6040_e5806: f64 = (p.p458 + assign6040_e5805);
        let assign6040_e5807: f64 = (-assign6040_e5806);
        let assign6040_e5809: f64 = (assign6040_e5807 / p.p477);
        let assign6040_e5811: f64 = (-80.0);
        let assign6040_e5812: f64 = if assign6040_e5809 > assign6040_e5811 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign6040_e5812;

        let (assign6050_e5832, assign6050_e5832_d_n4, assign6050_e5832_d_n6, assign6050_e5832_d_n7, assign6050_e5832_d_n8, assign6050_e5832_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard137 != 0.0)) {
        let assign6050_e5825: f64 = (0.5 * p.p20);
        let assign6050_e5826: f64 = (p.p458 + assign6050_e5825);
        let assign6050_e5827: f64 = (-assign6050_e5826);
        let assign6050_e5829: f64 = (assign6050_e5827 / p.p477);
        let assign6050_e5830: f64 = (assign6050_e5829).exp();
        (assign6050_e5830, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign6050_e5832;
        locals.var_temp1_dn4 = assign6050_e5832_d_n4;
        locals.var_temp1_dn6 = assign6050_e5832_d_n6;
        locals.var_temp1_dn7 = assign6050_e5832_d_n7;
        locals.var_temp1_dn8 = assign6050_e5832_d_n8;
        locals.var_temp1_dn9 = assign6050_e5832_d_n9;

        let (assign6060_e5891, assign6060_e5891_d_n4, assign6060_e5891_d_n6, assign6060_e5891_d_n7, assign6060_e5891_d_n8, assign6060_e5891_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard137 == 0.0)) {
        let assign6060_e5848: f64 = (0.5 * p.p20);
        let assign6060_e5849: f64 = (p.p458 + assign6060_e5848);
        let assign6060_e5850: f64 = (-assign6060_e5849);
        let assign6060_e5852: f64 = (assign6060_e5850 / p.p477);
        let assign6060_e5853: f64 = (-assign6060_e5852);
        let assign6060_e5855: f64 = (assign6060_e5853 - 80.0);
        let assign6060_e5861: f64 = (0.5 * p.p20);
        let assign6060_e5862: f64 = (p.p458 + assign6060_e5861);
        let assign6060_e5863: f64 = (-assign6060_e5862);
        let assign6060_e5865: f64 = (assign6060_e5863 / p.p477);
        let assign6060_e5866: f64 = (-assign6060_e5865);
        let assign6060_e5868: f64 = (assign6060_e5866 - 80.0);
        let assign6060_e5869: f64 = (0.5 * assign6060_e5868);
        let assign6060_e5874: f64 = (0.5 * p.p20);
        let assign6060_e5875: f64 = (p.p458 + assign6060_e5874);
        let assign6060_e5876: f64 = (-assign6060_e5875);
        let assign6060_e5878: f64 = (assign6060_e5876 / p.p477);
        let assign6060_e5879: f64 = (-assign6060_e5878);
        let assign6060_e5881: f64 = (assign6060_e5879 - 80.0);
        let assign6060_e5883: f64 = (assign6060_e5881 * 0.3333333333333);
        let assign6060_e5884: f64 = (1.0 + assign6060_e5883);
        let assign6060_e5885: f64 = (assign6060_e5869 * assign6060_e5884);
        let assign6060_e5886: f64 = (1.0 + assign6060_e5885);
        let assign6060_e5887: f64 = (assign6060_e5855 * assign6060_e5886);
        let assign6060_e5888: f64 = (1.0 + assign6060_e5887);
        let assign6060_e5889: f64 = (1.80485e-35 / assign6060_e5888);
        (assign6060_e5889, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9,)
    }
};
        locals.var_temp1 = assign6060_e5891;
        locals.var_temp1_dn4 = assign6060_e5891_d_n4;
        locals.var_temp1_dn6 = assign6060_e5891_d_n6;
        locals.var_temp1_dn7 = assign6060_e5891_d_n7;
        locals.var_temp1_dn8 = assign6060_e5891_d_n8;
        locals.var_temp1_dn9 = assign6060_e5891_d_n9;

        let assign6070_e5895: f64 = (0.5 * p.p20);
        let assign6070_e5896: f64 = (p.p459 + assign6070_e5895);
        let assign6070_e5897: f64 = (-assign6070_e5896);
        let assign6070_e5899: f64 = (assign6070_e5897 / p.p477);
        let assign6070_e5901: f64 = (-80.0);
        let assign6070_e5902: f64 = if assign6070_e5899 > assign6070_e5901 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign6070_e5902;

        let (assign6080_e5922, assign6080_e5922_d_n4, assign6080_e5922_d_n6, assign6080_e5922_d_n7, assign6080_e5922_d_n8, assign6080_e5922_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard138 != 0.0)) {
        let assign6080_e5915: f64 = (0.5 * p.p20);
        let assign6080_e5916: f64 = (p.p459 + assign6080_e5915);
        let assign6080_e5917: f64 = (-assign6080_e5916);
        let assign6080_e5919: f64 = (assign6080_e5917 / p.p477);
        let assign6080_e5920: f64 = (assign6080_e5919).exp();
        (assign6080_e5920, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign6080_e5922;
        locals.var_temp2_dn4 = assign6080_e5922_d_n4;
        locals.var_temp2_dn6 = assign6080_e5922_d_n6;
        locals.var_temp2_dn7 = assign6080_e5922_d_n7;
        locals.var_temp2_dn8 = assign6080_e5922_d_n8;
        locals.var_temp2_dn9 = assign6080_e5922_d_n9;

        let (assign6090_e5981, assign6090_e5981_d_n4, assign6090_e5981_d_n6, assign6090_e5981_d_n7, assign6090_e5981_d_n8, assign6090_e5981_d_n9,) = {
    if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard138 == 0.0)) {
        let assign6090_e5938: f64 = (0.5 * p.p20);
        let assign6090_e5939: f64 = (p.p459 + assign6090_e5938);
        let assign6090_e5940: f64 = (-assign6090_e5939);
        let assign6090_e5942: f64 = (assign6090_e5940 / p.p477);
        let assign6090_e5943: f64 = (-assign6090_e5942);
        let assign6090_e5945: f64 = (assign6090_e5943 - 80.0);
        let assign6090_e5951: f64 = (0.5 * p.p20);
        let assign6090_e5952: f64 = (p.p459 + assign6090_e5951);
        let assign6090_e5953: f64 = (-assign6090_e5952);
        let assign6090_e5955: f64 = (assign6090_e5953 / p.p477);
        let assign6090_e5956: f64 = (-assign6090_e5955);
        let assign6090_e5958: f64 = (assign6090_e5956 - 80.0);
        let assign6090_e5959: f64 = (0.5 * assign6090_e5958);
        let assign6090_e5964: f64 = (0.5 * p.p20);
        let assign6090_e5965: f64 = (p.p459 + assign6090_e5964);
        let assign6090_e5966: f64 = (-assign6090_e5965);
        let assign6090_e5968: f64 = (assign6090_e5966 / p.p477);
        let assign6090_e5969: f64 = (-assign6090_e5968);
        let assign6090_e5971: f64 = (assign6090_e5969 - 80.0);
        let assign6090_e5973: f64 = (assign6090_e5971 * 0.3333333333333);
        let assign6090_e5974: f64 = (1.0 + assign6090_e5973);
        let assign6090_e5975: f64 = (assign6090_e5959 * assign6090_e5974);
        let assign6090_e5976: f64 = (1.0 + assign6090_e5975);
        let assign6090_e5977: f64 = (assign6090_e5945 * assign6090_e5976);
        let assign6090_e5978: f64 = (1.0 + assign6090_e5977);
        let assign6090_e5979: f64 = (1.80485e-35 / assign6090_e5978);
        (assign6090_e5979, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9,)
    }
};
        locals.var_temp2 = assign6090_e5981;
        locals.var_temp2_dn4 = assign6090_e5981_d_n4;
        locals.var_temp2_dn6 = assign6090_e5981_d_n6;
        locals.var_temp2_dn7 = assign6090_e5981_d_n7;
        locals.var_temp2_dn8 = assign6090_e5981_d_n8;
        locals.var_temp2_dn9 = assign6090_e5981_d_n9;

        let (assign6100_e5996, assign6100_e5996_d_n4, assign6100_e5996_d_n6, assign6100_e5996_d_n7, assign6100_e5996_d_n8, assign6100_e5996_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6100_e5991: f64 = (1.0 - locals.var_temp1);
        let assign6100_e5993: f64 = (-p.p478);
        let assign6100_e5994: f64 = (assign6100_e5991).powf(assign6100_e5993);
        (assign6100_e5994, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn4))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn4) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn6))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn6) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn7))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn7) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn8))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn8) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn9))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn9) / assign6100_e5991))) },)
    } else {
        (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9,)
    }
};
        locals.var_temp3 = assign6100_e5996;
        locals.var_temp3_dn4 = assign6100_e5996_d_n4;
        locals.var_temp3_dn6 = assign6100_e5996_d_n6;
        locals.var_temp3_dn7 = assign6100_e5996_d_n7;
        locals.var_temp3_dn8 = assign6100_e5996_d_n8;
        locals.var_temp3_dn9 = assign6100_e5996_d_n9;

        let (assign6110_e6011, assign6110_e6011_d_n4, assign6110_e6011_d_n6, assign6110_e6011_d_n7, assign6110_e6011_d_n8, assign6110_e6011_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6110_e6006: f64 = (1.0 - locals.var_temp2);
        let assign6110_e6008: f64 = (-p.p478);
        let assign6110_e6009: f64 = (assign6110_e6006).powf(assign6110_e6008);
        (assign6110_e6009, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn4))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn4) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn6))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn6) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn7))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn7) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn8))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn8) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn9))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn9) / assign6110_e6006))) },)
    } else {
        (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9,)
    }
};
        locals.var_temp4 = assign6110_e6011;
        locals.var_temp4_dn4 = assign6110_e6011_d_n4;
        locals.var_temp4_dn6 = assign6110_e6011_d_n6;
        locals.var_temp4_dn7 = assign6110_e6011_d_n7;
        locals.var_temp4_dn8 = assign6110_e6011_d_n8;
        locals.var_temp4_dn9 = assign6110_e6011_d_n9;

        let (assign6120_e6029, assign6120_e6029_d_n4, assign6120_e6029_d_n6, assign6120_e6029_d_n7, assign6120_e6029_d_n8, assign6120_e6029_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6120_e6023: f64 = (locals.var_temp3 + locals.var_temp4);
        let assign6120_e6024: f64 = (0.5 * assign6120_e6023);
        let assign6120_e6026: f64 = (assign6120_e6024).powf(locals.var_temp);
        let assign6120_e6027: f64 = (1.0 - assign6120_e6026);
        (assign6120_e6027, (-if locals.var_temp_dn4 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn4 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)) / assign6120_e6024)))) }), (-if locals.var_temp_dn6 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn6 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)) / assign6120_e6024)))) }), (-if locals.var_temp_dn7 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn7 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)) / assign6120_e6024)))) }), (-if locals.var_temp_dn8 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn8 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)) / assign6120_e6024)))) }), (-if locals.var_temp_dn9 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn9 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)) / assign6120_e6024)))) }),)
    } else {
        (locals.var_str_gref, locals.var_str_gref_dn4, locals.var_str_gref_dn6, locals.var_str_gref_dn7, locals.var_str_gref_dn8, locals.var_str_gref_dn9,)
    }
};
        locals.var_str_gref = assign6120_e6029;
        locals.var_str_gref_dn4 = assign6120_e6029_d_n4;
        locals.var_str_gref_dn6 = assign6120_e6029_d_n6;
        locals.var_str_gref_dn7 = assign6120_e6029_d_n7;
        locals.var_str_gref_dn8 = assign6120_e6029_d_n8;
        locals.var_str_gref_dn9 = assign6120_e6029_d_n9;

        let (assign6130_e6045,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6130_e6039: f64 = (locals.var_w_i + locals.var_delwod);
        let assign6130_e6041: f64 = (assign6130_e6039 + p.p460);
        let assign6130_e6043: f64 = (assign6130_e6041).max(1e-9);
        (assign6130_e6043,)
    } else {
        (locals.var_wx,)
    }
};
        locals.var_wx = assign6130_e6045;

        let (assign6140_e6063, assign6140_e6063_d_n4, assign6140_e6063_d_n6, assign6140_e6063_d_n7, assign6140_e6063_d_n8, assign6140_e6063_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6140_e6058: f64 = (locals.var_rt - 1.0);
        let assign6140_e6059: f64 = (p.p483 * assign6140_e6058);
        let assign6140_e6060: f64 = (1.0 + assign6140_e6059);
        let assign6140_e6061: f64 = (p.p482 / assign6140_e6060);
        (assign6140_e6061, (-((p.p482 * (p.p483 * locals.var_rt_dn4)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn6)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn7)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn8)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn9)) / (assign6140_e6060 * assign6140_e6060))),)
    } else {
        (locals.var_ruo, locals.var_ruo_dn4, locals.var_ruo_dn6, locals.var_ruo_dn7, locals.var_ruo_dn8, locals.var_ruo_dn9,)
    }
};
        locals.var_ruo = assign6140_e6063;
        locals.var_ruo_dn4 = assign6140_e6063_d_n4;
        locals.var_ruo_dn6 = assign6140_e6063_d_n6;
        locals.var_ruo_dn7 = assign6140_e6063_d_n7;
        locals.var_ruo_dn8 = assign6140_e6063_d_n8;
        locals.var_ruo_dn9 = assign6140_e6063_d_n9;

        let (assign6150_e6075, assign6150_e6075_d_n4, assign6150_e6075_d_n6, assign6150_e6075_d_n7, assign6150_e6075_d_n8, assign6150_e6075_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6150_e6073: f64 = (locals.var_ruo * locals.var_str_g);
        (assign6150_e6073, ((locals.var_ruo_dn4 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn4)), ((locals.var_ruo_dn6 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn6)), ((locals.var_ruo_dn7 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn7)), ((locals.var_ruo_dn8 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn8)), ((locals.var_ruo_dn9 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn9)),)
    } else {
        (locals.var_rhobeta, locals.var_rhobeta_dn4, locals.var_rhobeta_dn6, locals.var_rhobeta_dn7, locals.var_rhobeta_dn8, locals.var_rhobeta_dn9,)
    }
};
        locals.var_rhobeta = assign6150_e6075;
        locals.var_rhobeta_dn4 = assign6150_e6075_d_n4;
        locals.var_rhobeta_dn6 = assign6150_e6075_d_n6;
        locals.var_rhobeta_dn7 = assign6150_e6075_d_n7;
        locals.var_rhobeta_dn8 = assign6150_e6075_d_n8;
        locals.var_rhobeta_dn9 = assign6150_e6075_d_n9;

        let (assign6160_e6087, assign6160_e6087_d_n4, assign6160_e6087_d_n6, assign6160_e6087_d_n7, assign6160_e6087_d_n8, assign6160_e6087_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6160_e6085: f64 = (locals.var_ruo * locals.var_str_gref);
        (assign6160_e6085, ((locals.var_ruo_dn4 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn4)), ((locals.var_ruo_dn6 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn6)), ((locals.var_ruo_dn7 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn7)), ((locals.var_ruo_dn8 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn8)), ((locals.var_ruo_dn9 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn9)),)
    } else {
        (locals.var_rhobetaref, locals.var_rhobetaref_dn4, locals.var_rhobetaref_dn6, locals.var_rhobetaref_dn7, locals.var_rhobetaref_dn8, locals.var_rhobetaref_dn9,)
    }
};
        locals.var_rhobetaref = assign6160_e6087;
        locals.var_rhobetaref_dn4 = assign6160_e6087_d_n4;
        locals.var_rhobetaref_dn6 = assign6160_e6087_d_n6;
        locals.var_rhobetaref_dn7 = assign6160_e6087_d_n7;
        locals.var_rhobetaref_dn8 = assign6160_e6087_d_n8;
        locals.var_rhobetaref_dn9 = assign6160_e6087_d_n9;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6170_e6099, assign6170_e6099_d_n4, assign6170_e6099_d_n6, assign6170_e6099_d_n7, assign6170_e6099_d_n8, assign6170_e6099_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6170_e6097: f64 = (locals.var_str_g - locals.var_str_gref);
        (assign6170_e6097, (locals.var_str_g_dn4 - locals.var_str_gref_dn4), (locals.var_str_g_dn6 - locals.var_str_gref_dn6), (locals.var_str_g_dn7 - locals.var_str_gref_dn7), (locals.var_str_g_dn8 - locals.var_str_gref_dn8), (locals.var_str_g_dn9 - locals.var_str_gref_dn9),)
    } else {
        (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9,)
    }
};
        locals.var_temp0__blk79 = assign6170_e6099;
        locals.var_temp0__blk79_dn4 = assign6170_e6099_d_n4;
        locals.var_temp0__blk79_dn6 = assign6170_e6099_d_n6;
        locals.var_temp0__blk79_dn7 = assign6170_e6099_d_n7;
        locals.var_temp0__blk79_dn8 = assign6170_e6099_d_n8;
        locals.var_temp0__blk79_dn9 = assign6170_e6099_d_n9;

        let (assign6180_e6117,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6180_e6110: f64 = (p.p480 * locals.var_wx);
        let assign6180_e6112: f64 = (assign6180_e6110 / locals.var_wen);
        let assign6180_e6113: f64 = (1.0 + assign6180_e6112);
        let assign6180_e6115: f64 = (assign6180_e6113).max(1e-20);
        (assign6180_e6115,)
    } else {
        (locals.var_kstressvth0,)
    }
};
        locals.var_kstressvth0 = assign6180_e6117;

        let (assign6190_e6135, assign6190_e6135_d_n4, assign6190_e6135_d_n6, assign6190_e6135_d_n7, assign6190_e6135_d_n8, assign6190_e6135_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6190_e6128: f64 = (1.0 + locals.var_rhobeta);
        let assign6190_e6129: f64 = (locals.var_betn_p * assign6190_e6128);
        let assign6190_e6132: f64 = (1.0 + locals.var_rhobetaref);
        let assign6190_e6133: f64 = (assign6190_e6129 / assign6190_e6132);
        (assign6190_e6133, (((((locals.var_betn_p_dn4 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn4)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn4)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn6 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn6)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn6)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn7 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn7)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn7)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn8 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn8)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn8)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn9 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn9)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn9)) / (assign6190_e6132 * assign6190_e6132)),)
    } else {
        (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9,)
    }
};
        locals.var_betn_p = assign6190_e6135;
        locals.var_betn_p_dn4 = assign6190_e6135_d_n4;
        locals.var_betn_p_dn6 = assign6190_e6135_d_n6;
        locals.var_betn_p_dn7 = assign6190_e6135_d_n7;
        locals.var_betn_p_dn8 = assign6190_e6135_d_n8;
        locals.var_betn_p_dn9 = assign6190_e6135_d_n9;

        let (assign6200_e6147, assign6200_e6147_d_n4, assign6200_e6147_d_n6, assign6200_e6147_d_n7, assign6200_e6147_d_n8, assign6200_e6147_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6200_e6145: f64 = (locals.var_betn_p).max(1e-10);
        (assign6200_e6145, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9,)
    }
};
        locals.var_betn1_t = assign6200_e6147;
        locals.var_betn1_t_dn4 = assign6200_e6147_d_n4;
        locals.var_betn1_t_dn6 = assign6200_e6147_d_n6;
        locals.var_betn1_t_dn7 = assign6200_e6147_d_n7;
        locals.var_betn1_t_dn8 = assign6200_e6147_d_n8;
        locals.var_betn1_t_dn9 = assign6200_e6147_d_n9;

        let (assign6210_e6159, assign6210_e6159_d_n4, assign6210_e6159_d_n6, assign6210_e6159_d_n7, assign6210_e6159_d_n8, assign6210_e6159_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6210_e6157: f64 = (p.p250 * locals.var_betn1_t);
        (assign6210_e6157, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9),)
    } else {
        (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9,)
    }
};
        locals.var_betn2_t = assign6210_e6159;
        locals.var_betn2_t_dn4 = assign6210_e6159_d_n4;
        locals.var_betn2_t_dn6 = assign6210_e6159_d_n6;
        locals.var_betn2_t_dn7 = assign6210_e6159_d_n7;
        locals.var_betn2_t_dn8 = assign6210_e6159_d_n8;
        locals.var_betn2_t_dn9 = assign6210_e6159_d_n9;

        let (assign6220_e6187, assign6220_e6187_d_n4, assign6220_e6187_d_n6, assign6220_e6187_d_n7, assign6220_e6187_d_n8, assign6220_e6187_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6220_e6169: f64 = (1.0 + locals.var_rhobeta);
        let assign6220_e6173: f64 = (p.p484 * locals.var_rhobetaref);
        let assign6220_e6174: f64 = (1.0 + assign6220_e6173);
        let assign6220_e6175: f64 = (assign6220_e6169 * assign6220_e6174);
        let assign6220_e6178: f64 = (1.0 + locals.var_rhobetaref);
        let assign6220_e6182: f64 = (p.p484 * locals.var_rhobeta);
        let assign6220_e6183: f64 = (1.0 + assign6220_e6182);
        let assign6220_e6184: f64 = (assign6220_e6178 * assign6220_e6183);
        let assign6220_e6185: f64 = (assign6220_e6175 / assign6220_e6184);
        (assign6220_e6185, (((((locals.var_rhobeta_dn4 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn4))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn4 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn4))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn6 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn6))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn6 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn6))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn7 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn7))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn7 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn7))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn8 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn8))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn8 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn8))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn9 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn9))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn9 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn9))))) / (assign6220_e6184 * assign6220_e6184)),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6220_e6187;
        locals.var_temp_dn4 = assign6220_e6187_d_n4;
        locals.var_temp_dn6 = assign6220_e6187_d_n6;
        locals.var_temp_dn7 = assign6220_e6187_d_n7;
        locals.var_temp_dn8 = assign6220_e6187_d_n8;
        locals.var_temp_dn9 = assign6220_e6187_d_n9;

        let (assign6230_e6199, assign6230_e6199_d_n4, assign6230_e6199_d_n6, assign6230_e6199_d_n7, assign6230_e6199_d_n8, assign6230_e6199_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6230_e6197: f64 = (locals.var_thesat_p * locals.var_temp);
        (assign6230_e6197, ((locals.var_thesat_p_dn4 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn4)), ((locals.var_thesat_p_dn6 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn6)), ((locals.var_thesat_p_dn7 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn7)), ((locals.var_thesat_p_dn8 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn8)), ((locals.var_thesat_p_dn9 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn9)),)
    } else {
        (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9,)
    }
};
        locals.var_thesat_p = assign6230_e6199;
        locals.var_thesat_p_dn4 = assign6230_e6199_d_n4;
        locals.var_thesat_p_dn6 = assign6230_e6199_d_n6;
        locals.var_thesat_p_dn7 = assign6230_e6199_d_n7;
        locals.var_thesat_p_dn8 = assign6230_e6199_d_n8;
        locals.var_thesat_p_dn9 = assign6230_e6199_d_n9;

        let (assign6240_e6211, assign6240_e6211_d_n4, assign6240_e6211_d_n6, assign6240_e6211_d_n7, assign6240_e6211_d_n8, assign6240_e6211_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6240_e6209: f64 = (locals.var_thesat_p).max(0.0);
        (assign6240_e6209, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9,)
    }
};
        locals.var_thesat_t = assign6240_e6211;
        locals.var_thesat_t_dn4 = assign6240_e6211_d_n4;
        locals.var_thesat_t_dn6 = assign6240_e6211_d_n6;
        locals.var_thesat_t_dn7 = assign6240_e6211_d_n7;
        locals.var_thesat_t_dn8 = assign6240_e6211_d_n8;
        locals.var_thesat_t_dn9 = assign6240_e6211_d_n9;

        let (assign6250_e6223, assign6250_e6223_d_n4, assign6250_e6223_d_n6, assign6250_e6223_d_n7, assign6250_e6223_d_n8, assign6250_e6223_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6250_e6221: f64 = (locals.var_thesatac_p * locals.var_temp);
        (assign6250_e6221, ((locals.var_thesatac_p_dn4 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn4)), ((locals.var_thesatac_p_dn6 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn6)), ((locals.var_thesatac_p_dn7 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn7)), ((locals.var_thesatac_p_dn8 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn8)), ((locals.var_thesatac_p_dn9 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn9)),)
    } else {
        (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9,)
    }
};
        locals.var_thesatac_p = assign6250_e6223;
        locals.var_thesatac_p_dn4 = assign6250_e6223_d_n4;
        locals.var_thesatac_p_dn6 = assign6250_e6223_d_n6;
        locals.var_thesatac_p_dn7 = assign6250_e6223_d_n7;
        locals.var_thesatac_p_dn8 = assign6250_e6223_d_n8;
        locals.var_thesatac_p_dn9 = assign6250_e6223_d_n9;

        let (assign6260_e6235, assign6260_e6235_d_n4, assign6260_e6235_d_n6, assign6260_e6235_d_n7, assign6260_e6235_d_n8, assign6260_e6235_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6260_e6233: f64 = (locals.var_thesatac_p).max(0.0);
        (assign6260_e6233, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9,)
    }
};
        locals.var_thesatac_t = assign6260_e6235;
        locals.var_thesatac_t_dn4 = assign6260_e6235_d_n4;
        locals.var_thesatac_t_dn6 = assign6260_e6235_d_n6;
        locals.var_thesatac_t_dn7 = assign6260_e6235_d_n7;
        locals.var_thesatac_t_dn8 = assign6260_e6235_d_n8;
        locals.var_thesatac_t_dn9 = assign6260_e6235_d_n9;

        let (assign6270_e6249, assign6270_e6249_d_n4, assign6270_e6249_d_n6, assign6270_e6249_d_n7, assign6270_e6249_d_n8, assign6270_e6249_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6270_e6245: f64 = (p.p479 * locals.var_temp0__blk79);
        let assign6270_e6247: f64 = (assign6270_e6245 / locals.var_kstressvth0);
        (assign6270_e6247, ((p.p479 * locals.var_temp0__blk79_dn4) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn6) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn7) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn8) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn9) / locals.var_kstressvth0),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6270_e6249;
        locals.var_temp_dn4 = assign6270_e6249_d_n4;
        locals.var_temp_dn6 = assign6270_e6249_d_n6;
        locals.var_temp_dn7 = assign6270_e6249_d_n7;
        locals.var_temp_dn8 = assign6270_e6249_d_n8;
        locals.var_temp_dn9 = assign6270_e6249_d_n9;

        let (assign6280_e6261, assign6280_e6261_d_n4, assign6280_e6261_d_n6, assign6280_e6261_d_n7, assign6280_e6261_d_n8, assign6280_e6261_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6280_e6259: f64 = (locals.var_vfb1_t + locals.var_temp);
        (assign6280_e6259, (locals.var_vfb1_t_dn4 + locals.var_temp_dn4), (locals.var_vfb1_t_dn6 + locals.var_temp_dn6), (locals.var_vfb1_t_dn7 + locals.var_temp_dn7), (locals.var_vfb1_t_dn8 + locals.var_temp_dn8), (locals.var_vfb1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9,)
    }
};
        locals.var_vfb1_t = assign6280_e6261;
        locals.var_vfb1_t_dn4 = assign6280_e6261_d_n4;
        locals.var_vfb1_t_dn6 = assign6280_e6261_d_n6;
        locals.var_vfb1_t_dn7 = assign6280_e6261_d_n7;
        locals.var_vfb1_t_dn8 = assign6280_e6261_d_n8;
        locals.var_vfb1_t_dn9 = assign6280_e6261_d_n9;

        let (assign6290_e6273, assign6290_e6273_d_n4, assign6290_e6273_d_n6, assign6290_e6273_d_n7, assign6290_e6273_d_n8, assign6290_e6273_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6290_e6271: f64 = (locals.var_vfb2_t + locals.var_temp);
        (assign6290_e6271, (locals.var_vfb2_t_dn4 + locals.var_temp_dn4), (locals.var_vfb2_t_dn6 + locals.var_temp_dn6), (locals.var_vfb2_t_dn7 + locals.var_temp_dn7), (locals.var_vfb2_t_dn8 + locals.var_temp_dn8), (locals.var_vfb2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign6290_e6273;
        locals.var_vfb2_t_dn4 = assign6290_e6273_d_n4;
        locals.var_vfb2_t_dn6 = assign6290_e6273_d_n6;
        locals.var_vfb2_t_dn7 = assign6290_e6273_d_n7;
        locals.var_vfb2_t_dn8 = assign6290_e6273_d_n8;
        locals.var_vfb2_t_dn9 = assign6290_e6273_d_n9;

        let (assign6300_e6285, assign6300_e6285_d_n4, assign6300_e6285_d_n6, assign6300_e6285_d_n7, assign6300_e6285_d_n8, assign6300_e6285_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6300_e6283: f64 = (locals.var_vfbac1_t + locals.var_temp);
        (assign6300_e6283, (locals.var_vfbac1_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac1_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac1_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac1_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac1_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9,)
    }
};
        locals.var_vfbac1_t = assign6300_e6285;
        locals.var_vfbac1_t_dn4 = assign6300_e6285_d_n4;
        locals.var_vfbac1_t_dn6 = assign6300_e6285_d_n6;
        locals.var_vfbac1_t_dn7 = assign6300_e6285_d_n7;
        locals.var_vfbac1_t_dn8 = assign6300_e6285_d_n8;
        locals.var_vfbac1_t_dn9 = assign6300_e6285_d_n9;

        let (assign6310_e6297, assign6310_e6297_d_n4, assign6310_e6297_d_n6, assign6310_e6297_d_n7, assign6310_e6297_d_n8, assign6310_e6297_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6310_e6295: f64 = (locals.var_vfbac2_t + locals.var_temp);
        (assign6310_e6295, (locals.var_vfbac2_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac2_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac2_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac2_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac2_t_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign6310_e6297;
        locals.var_vfbac2_t_dn4 = assign6310_e6297_d_n4;
        locals.var_vfbac2_t_dn6 = assign6310_e6297_d_n6;
        locals.var_vfbac2_t_dn7 = assign6310_e6297_d_n7;
        locals.var_vfbac2_t_dn8 = assign6310_e6297_d_n8;
        locals.var_vfbac2_t_dn9 = assign6310_e6297_d_n9;

        let (assign6320_e6319, assign6320_e6319_d_n4, assign6320_e6319_d_n6, assign6320_e6319_d_n7, assign6320_e6319_d_n8, assign6320_e6319_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6320_e6307: f64 = (p.p481 * locals.var_temp0__blk79);
        let assign6320_e6310: f64 = (locals.var_lambda_le).powf(p.p232);
        let assign6320_e6311: f64 = (assign6320_e6307 * assign6320_e6310);
        let assign6320_e6315: f64 = (p.p233 * locals.var_iwe);
        let assign6320_e6316: f64 = (1.0 + assign6320_e6315);
        let assign6320_e6317: f64 = (assign6320_e6311 * assign6320_e6316);
        (assign6320_e6317, (((p.p481 * locals.var_temp0__blk79_dn4) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn6) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn7) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn8) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn9) * assign6320_e6310) * assign6320_e6316),)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6320_e6319;
        locals.var_temp_dn4 = assign6320_e6319_d_n4;
        locals.var_temp_dn6 = assign6320_e6319_d_n6;
        locals.var_temp_dn7 = assign6320_e6319_d_n7;
        locals.var_temp_dn8 = assign6320_e6319_d_n8;
        locals.var_temp_dn9 = assign6320_e6319_d_n9;

        let (assign6330_e6331, assign6330_e6331_d_n4, assign6330_e6331_d_n6, assign6330_e6331_d_n7, assign6330_e6331_d_n8, assign6330_e6331_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6330_e6329: f64 = (locals.var_cf_p + locals.var_temp);
        (assign6330_e6329, (locals.var_cf_p_dn4 + locals.var_temp_dn4), (locals.var_cf_p_dn6 + locals.var_temp_dn6), (locals.var_cf_p_dn7 + locals.var_temp_dn7), (locals.var_cf_p_dn8 + locals.var_temp_dn8), (locals.var_cf_p_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9,)
    }
};
        locals.var_cf_p = assign6330_e6331;
        locals.var_cf_p_dn4 = assign6330_e6331_d_n4;
        locals.var_cf_p_dn6 = assign6330_e6331_d_n6;
        locals.var_cf_p_dn7 = assign6330_e6331_d_n7;
        locals.var_cf_p_dn8 = assign6330_e6331_d_n8;
        locals.var_cf_p_dn9 = assign6330_e6331_d_n9;

        let (assign6340_e6343, assign6340_e6343_d_n4, assign6340_e6343_d_n6, assign6340_e6343_d_n7, assign6340_e6343_d_n8, assign6340_e6343_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6340_e6341: f64 = (locals.var_cf_p).max(0.0);
        (assign6340_e6341, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9,)
    }
};
        locals.var_cf1_t = assign6340_e6343;
        locals.var_cf1_t_dn4 = assign6340_e6343_d_n4;
        locals.var_cf1_t_dn6 = assign6340_e6343_d_n6;
        locals.var_cf1_t_dn7 = assign6340_e6343_d_n7;
        locals.var_cf1_t_dn8 = assign6340_e6343_d_n8;
        locals.var_cf1_t_dn9 = assign6340_e6343_d_n9;

        let (assign6350_e6355, assign6350_e6355_d_n4, assign6350_e6355_d_n6, assign6350_e6355_d_n7, assign6350_e6355_d_n8, assign6350_e6355_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6350_e6353: f64 = (locals.var_cfac_p + locals.var_temp);
        (assign6350_e6353, (locals.var_cfac_p_dn4 + locals.var_temp_dn4), (locals.var_cfac_p_dn6 + locals.var_temp_dn6), (locals.var_cfac_p_dn7 + locals.var_temp_dn7), (locals.var_cfac_p_dn8 + locals.var_temp_dn8), (locals.var_cfac_p_dn9 + locals.var_temp_dn9),)
    } else {
        (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9,)
    }
};
        locals.var_cfac_p = assign6350_e6355;
        locals.var_cfac_p_dn4 = assign6350_e6355_d_n4;
        locals.var_cfac_p_dn6 = assign6350_e6355_d_n6;
        locals.var_cfac_p_dn7 = assign6350_e6355_d_n7;
        locals.var_cfac_p_dn8 = assign6350_e6355_d_n8;
        locals.var_cfac_p_dn9 = assign6350_e6355_d_n9;

        let (assign6360_e6367, assign6360_e6367_d_n4, assign6360_e6367_d_n6, assign6360_e6367_d_n7, assign6360_e6367_d_n8, assign6360_e6367_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6360_e6365: f64 = (locals.var_cfac_p).max(0.0);
        (assign6360_e6365, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9,)
    }
};
        locals.var_cfac1_t = assign6360_e6367;
        locals.var_cfac1_t_dn4 = assign6360_e6367_d_n4;
        locals.var_cfac1_t_dn6 = assign6360_e6367_d_n6;
        locals.var_cfac1_t_dn7 = assign6360_e6367_d_n7;
        locals.var_cfac1_t_dn8 = assign6360_e6367_d_n8;
        locals.var_cfac1_t_dn9 = assign6360_e6367_d_n9;

        let (assign6370_e6381, assign6370_e6381_d_n4, assign6370_e6381_d_n6, assign6370_e6381_d_n7, assign6370_e6381_d_n8, assign6370_e6381_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6370_e6377: f64 = (p.p234 * locals.var_tox2_i);
        let assign6370_e6379: f64 = (assign6370_e6377 / locals.var_tox1_i);
        (assign6370_e6379, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9,)
    }
};
        locals.var_temp = assign6370_e6381;
        locals.var_temp_dn4 = assign6370_e6381_d_n4;
        locals.var_temp_dn6 = assign6370_e6381_d_n6;
        locals.var_temp_dn7 = assign6370_e6381_d_n7;
        locals.var_temp_dn8 = assign6370_e6381_d_n8;
        locals.var_temp_dn9 = assign6370_e6381_d_n9;

        let (assign6380_e6393, assign6380_e6393_d_n4, assign6380_e6393_d_n6, assign6380_e6393_d_n7, assign6380_e6393_d_n8, assign6380_e6393_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6380_e6391: f64 = (locals.var_cf1_t * locals.var_temp);
        (assign6380_e6391, ((locals.var_cf1_t_dn4 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn4)), ((locals.var_cf1_t_dn6 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn6)), ((locals.var_cf1_t_dn7 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn7)), ((locals.var_cf1_t_dn8 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn8)), ((locals.var_cf1_t_dn9 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn9)),)
    } else {
        (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9,)
    }
};
        locals.var_cf2_t = assign6380_e6393;
        locals.var_cf2_t_dn4 = assign6380_e6393_d_n4;
        locals.var_cf2_t_dn6 = assign6380_e6393_d_n6;
        locals.var_cf2_t_dn7 = assign6380_e6393_d_n7;
        locals.var_cf2_t_dn8 = assign6380_e6393_d_n8;
        locals.var_cf2_t_dn9 = assign6380_e6393_d_n9;

        let (assign6390_e6405, assign6390_e6405_d_n4, assign6390_e6405_d_n6, assign6390_e6405_d_n7, assign6390_e6405_d_n8, assign6390_e6405_d_n9,) = {
    if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
        let assign6390_e6403: f64 = (locals.var_cfac1_t * locals.var_temp);
        (assign6390_e6403, ((locals.var_cfac1_t_dn4 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn4)), ((locals.var_cfac1_t_dn6 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn6)), ((locals.var_cfac1_t_dn7 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn7)), ((locals.var_cfac1_t_dn8 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn8)), ((locals.var_cfac1_t_dn9 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn9)),)
    } else {
        (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9,)
    }
};
        locals.var_cfac2_t = assign6390_e6405;
        locals.var_cfac2_t_dn4 = assign6390_e6405_d_n4;
        locals.var_cfac2_t_dn6 = assign6390_e6405_d_n6;
        locals.var_cfac2_t_dn7 = assign6390_e6405_d_n7;
        locals.var_cfac2_t_dn8 = assign6390_e6405_d_n8;
        locals.var_cfac2_t_dn9 = assign6390_e6405_d_n9;

        let assign6400_e6408: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign6400_e6408;

        let (assign6410_e6412,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_nov_i,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign6410_e6412;

        let (assign6420_e6416,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_igovinv_t,)
    } else {
        (locals.var_igovinvd_t,)
    }
};
        locals.var_igovinvd_t = assign6420_e6416;

        let (assign6430_e6420,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_fnovinv_t,)
    } else {
        (locals.var_fnovinvd_t,)
    }
};
        locals.var_fnovinvd_t = assign6430_e6420;

        let (assign6440_e6424,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_igovacc_t,)
    } else {
        (locals.var_igovaccd_t,)
    }
};
        locals.var_igovaccd_t = assign6440_e6424;

        let (assign6450_e6428, assign6450_e6428_d_n4, assign6450_e6428_d_n6, assign6450_e6428_d_n7, assign6450_e6428_d_n8, assign6450_e6428_d_n9,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9,)
    } else {
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9,)
    }
};
        locals.var_agidld_i = assign6450_e6428;
        locals.var_agidld_i_dn4 = assign6450_e6428_d_n4;
        locals.var_agidld_i_dn6 = assign6450_e6428_d_n6;
        locals.var_agidld_i_dn7 = assign6450_e6428_d_n7;
        locals.var_agidld_i_dn8 = assign6450_e6428_d_n8;
        locals.var_agidld_i_dn9 = assign6450_e6428_d_n9;

        let (assign6460_e6432,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_bgidl_t,)
    } else {
        (locals.var_bgidld_t,)
    }
};
        locals.var_bgidld_t = assign6460_e6432;

        let (assign6470_e6436,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_stbgidl_i,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign6470_e6436;

        let (assign6480_e6440,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_cgidl_i,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign6480_e6440;

        let (assign6490_e6444,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_dgidl_i,)
    } else {
        (locals.var_dgidld_i,)
    }
};
        locals.var_dgidld_i = assign6490_e6444;

        let (assign6500_e6448, assign6500_e6448_d_n4, assign6500_e6448_d_n6, assign6500_e6448_d_n7, assign6500_e6448_d_n8, assign6500_e6448_d_n9,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9,)
    } else {
        (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9,)
    }
};
        locals.var_covd_i = assign6500_e6448;
        locals.var_covd_i_dn4 = assign6500_e6448_d_n4;
        locals.var_covd_i_dn6 = assign6500_e6448_d_n6;
        locals.var_covd_i_dn7 = assign6500_e6448_d_n7;
        locals.var_covd_i_dn8 = assign6500_e6448_d_n8;
        locals.var_covd_i_dn9 = assign6500_e6448_d_n9;

        let (assign6510_e6452, assign6510_e6452_d_n4, assign6510_e6452_d_n6, assign6510_e6452_d_n7, assign6510_e6452_d_n8, assign6510_e6452_d_n9,) = {
    if (locals.var_guard139 != 0.0) {
        (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9,)
    } else {
        (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9,)
    }
};
        locals.var_cfrd_i = assign6510_e6452;
        locals.var_cfrd_i_dn4 = assign6510_e6452_d_n4;
        locals.var_cfrd_i_dn6 = assign6510_e6452_d_n6;
        locals.var_cfrd_i_dn7 = assign6510_e6452_d_n7;
        locals.var_cfrd_i_dn8 = assign6510_e6452_d_n8;
        locals.var_cfrd_i_dn9 = assign6510_e6452_d_n9;

        let assign6520_e6455: f64 = (1.0 - locals.var_xge_i);
        locals.var_one_m_xge = assign6520_e6455;

        let assign6530_e6458: f64 = (1.04479e-10 * locals.var_one_m_xge);
        let assign6530_e6461: f64 = (1.43438e-10 * locals.var_xge_i);
        let assign6530_e6462: f64 = (assign6530_e6458 + assign6530_e6461);
        locals.var_epsch = assign6530_e6462;

        let assign6540_e6466: f64 = (0.000473 * locals.var_tkc_sq);
        let assign6540_e6469: f64 = (636.0 + locals.var_tkc);
        let assign6540_e6470: f64 = (assign6540_e6466 / assign6540_e6469);
        let assign6540_e6471: f64 = (1.17 - assign6540_e6470);
        locals.var_egsi = assign6540_e6471;
        locals.var_egsi_dn4 = (-((((0.000473 * locals.var_tkc_sq_dn4) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn4)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_dn6 = (-((((0.000473 * locals.var_tkc_sq_dn6) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn6)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_dn7 = (-((((0.000473 * locals.var_tkc_sq_dn7) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn7)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_dn8 = (-((((0.000473 * locals.var_tkc_sq_dn8) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn8)) / (assign6540_e6469 * assign6540_e6469)));
        locals.var_egsi_dn9 = (-((((0.000473 * locals.var_tkc_sq_dn9) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn9)) / (assign6540_e6469 * assign6540_e6469)));

        let assign6550_e6475: f64 = (0.0004774 * locals.var_tkc_sq);
        let assign6550_e6478: f64 = (235.0 + locals.var_tkc);
        let assign6550_e6479: f64 = (assign6550_e6475 / assign6550_e6478);
        let assign6550_e6480: f64 = (0.744 - assign6550_e6479);
        locals.var_egge = assign6550_e6480;
        locals.var_egge_dn4 = (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn4)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_dn6 = (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn6)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_dn7 = (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn7)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_dn8 = (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn8)) / (assign6550_e6478 * assign6550_e6478)));
        locals.var_egge_dn9 = (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn9)) / (assign6550_e6478 * assign6550_e6478)));

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6560_e6483: f64 = (locals.var_egge - locals.var_egsi);
        let assign6560_e6485: f64 = (-0.4);
        let assign6560_e6487: f64 = (assign6560_e6485 * locals.var_one_m_xge);
        let assign6560_e6488: f64 = (assign6560_e6483 + assign6560_e6487);
        let assign6560_e6490: f64 = (assign6560_e6488 * locals.var_xge_i);
        locals.var_deg = assign6560_e6490;
        locals.var_deg_dn4 = ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i);
        locals.var_deg_dn6 = ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i);
        locals.var_deg_dn7 = ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i);
        locals.var_deg_dn8 = ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i);
        locals.var_deg_dn9 = ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i);

        let assign6570_e6493: f64 = (locals.var_egsi + locals.var_deg);
        locals.var_eg = assign6570_e6493;
        locals.var_eg_dn4 = (locals.var_egsi_dn4 + locals.var_deg_dn4);
        locals.var_eg_dn6 = (locals.var_egsi_dn6 + locals.var_deg_dn6);
        locals.var_eg_dn7 = (locals.var_egsi_dn7 + locals.var_deg_dn7);
        locals.var_eg_dn8 = (locals.var_egsi_dn8 + locals.var_deg_dn8);
        locals.var_eg_dn9 = (locals.var_egsi_dn9 + locals.var_deg_dn9);

        let assign6580_e6496: f64 = (0.5 * locals.var_eg);
        let assign6580_e6498: f64 = (assign6580_e6496 * locals.var_inv_phit0);
        locals.var_eg_2phit0 = assign6580_e6498;
        locals.var_eg_2phit0_dn4 = (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn4));
        locals.var_eg_2phit0_dn6 = (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn6));
        locals.var_eg_2phit0_dn7 = (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn7));
        locals.var_eg_2phit0_dn8 = (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn8));
        locals.var_eg_2phit0_dn9 = (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn9));

        locals.var_eg_2phit0_woshe = locals.var_eg_2phit0;
        locals.var_eg_2phit0_woshe_dn4 = locals.var_eg_2phit0_dn4;
        locals.var_eg_2phit0_woshe_dn6 = locals.var_eg_2phit0_dn6;
        locals.var_eg_2phit0_woshe_dn7 = locals.var_eg_2phit0_dn7;
        locals.var_eg_2phit0_woshe_dn8 = locals.var_eg_2phit0_dn8;
        locals.var_eg_2phit0_woshe_dn9 = locals.var_eg_2phit0_dn9;

        let assign6600_e6504: f64 = (10.0 * locals.var_xge_i);
        let assign6600_e6505: f64 = (assign6600_e6504).sqrt();
        let assign6600_e6506: f64 = (1.0 + assign6600_e6505);
        let assign6600_e6507: f64 = (1.0 / assign6600_e6506);
        locals.var_niratio = assign6600_e6507;

        let assign6610_e6510: f64 = (0.05 * locals.var_xge_i);
        let assign6610_e6513: f64 = (0.5 * locals.var_deg);
        let assign6610_e6514: f64 = (assign6610_e6510 - assign6610_e6513);
        locals.var_dvfbch = assign6610_e6514;
        locals.var_dvfbch_dn4 = (-(0.5 * locals.var_deg_dn4));
        locals.var_dvfbch_dn6 = (-(0.5 * locals.var_deg_dn6));
        locals.var_dvfbch_dn7 = (-(0.5 * locals.var_deg_dn7));
        locals.var_dvfbch_dn8 = (-(0.5 * locals.var_deg_dn8));
        locals.var_dvfbch_dn9 = (-(0.5 * locals.var_deg_dn9));

        let assign6620_e6517: f64 = (1.602176565e-19 * locals.var_nch_i);
        let assign6620_e6519: f64 = (assign6620_e6517 * 0.5);
        let assign6620_e6521: f64 = (assign6620_e6519 * locals.var_tsi_i);
        let assign6620_e6523: f64 = (assign6620_e6521 / 3.45313e-11);
        locals.var_temp = assign6620_e6523;
        locals.var_temp_dn4 = 0.0;
        locals.var_temp_dn6 = 0.0;
        locals.var_temp_dn7 = 0.0;
        locals.var_temp_dn8 = 0.0;
        locals.var_temp_dn9 = 0.0;

        let assign6630_e6526: f64 = if locals.var_typech_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign6630_e6526;

        let (assign6640_e6536, assign6640_e6536_d_n4, assign6640_e6536_d_n6, assign6640_e6536_d_n7, assign6640_e6536_d_n8, assign6640_e6536_d_n9,) = {
    if (locals.var_guard140 != 0.0) {
        let assign6640_e6532: f64 = (p.p13 * 4e-10);
        let assign6640_e6533: f64 = (locals.var_tox1_i + assign6640_e6532);
        let assign6640_e6534: f64 = (locals.var_temp * assign6640_e6533);
        (assign6640_e6534, (locals.var_temp_dn4 * assign6640_e6533), (locals.var_temp_dn6 * assign6640_e6533), (locals.var_temp_dn7 * assign6640_e6533), (locals.var_temp_dn8 * assign6640_e6533), (locals.var_temp_dn9 * assign6640_e6533),)
    } else {
        (locals.var_dvfb1nch, locals.var_dvfb1nch_dn4, locals.var_dvfb1nch_dn6, locals.var_dvfb1nch_dn7, locals.var_dvfb1nch_dn8, locals.var_dvfb1nch_dn9,)
    }
};
        locals.var_dvfb1nch = assign6640_e6536;
        locals.var_dvfb1nch_dn4 = assign6640_e6536_d_n4;
        locals.var_dvfb1nch_dn6 = assign6640_e6536_d_n6;
        locals.var_dvfb1nch_dn7 = assign6640_e6536_d_n7;
        locals.var_dvfb1nch_dn8 = assign6640_e6536_d_n8;
        locals.var_dvfb1nch_dn9 = assign6640_e6536_d_n9;

        let (assign6650_e6546, assign6650_e6546_d_n4, assign6650_e6546_d_n6, assign6650_e6546_d_n7, assign6650_e6546_d_n8, assign6650_e6546_d_n9,) = {
    if (locals.var_guard140 != 0.0) {
        let assign6650_e6542: f64 = (p.p13 * 4e-10);
        let assign6650_e6543: f64 = (locals.var_tox2_i + assign6650_e6542);
        let assign6650_e6544: f64 = (locals.var_temp * assign6650_e6543);
        (assign6650_e6544, (locals.var_temp_dn4 * assign6650_e6543), (locals.var_temp_dn6 * assign6650_e6543), (locals.var_temp_dn7 * assign6650_e6543), (locals.var_temp_dn8 * assign6650_e6543), (locals.var_temp_dn9 * assign6650_e6543),)
    } else {
        (locals.var_dvfb2nch, locals.var_dvfb2nch_dn4, locals.var_dvfb2nch_dn6, locals.var_dvfb2nch_dn7, locals.var_dvfb2nch_dn8, locals.var_dvfb2nch_dn9,)
    }
};
        locals.var_dvfb2nch = assign6650_e6546;
        locals.var_dvfb2nch_dn4 = assign6650_e6546_d_n4;
        locals.var_dvfb2nch_dn6 = assign6650_e6546_d_n6;
        locals.var_dvfb2nch_dn7 = assign6650_e6546_d_n7;
        locals.var_dvfb2nch_dn8 = assign6650_e6546_d_n8;
        locals.var_dvfb2nch_dn9 = assign6650_e6546_d_n9;

        let (assign6660_e6558, assign6660_e6558_d_n4, assign6660_e6558_d_n6, assign6660_e6558_d_n7, assign6660_e6558_d_n8, assign6660_e6558_d_n9,) = {
    if (locals.var_guard140 == 0.0) {
        let assign6660_e6550: f64 = (-locals.var_temp);
        let assign6660_e6554: f64 = (p.p13 * 4e-10);
        let assign6660_e6555: f64 = (locals.var_tox1_i + assign6660_e6554);
        let assign6660_e6556: f64 = (assign6660_e6550 * assign6660_e6555);
        (assign6660_e6556, ((-locals.var_temp_dn4) * assign6660_e6555), ((-locals.var_temp_dn6) * assign6660_e6555), ((-locals.var_temp_dn7) * assign6660_e6555), ((-locals.var_temp_dn8) * assign6660_e6555), ((-locals.var_temp_dn9) * assign6660_e6555),)
    } else {
        (locals.var_dvfb1nch, locals.var_dvfb1nch_dn4, locals.var_dvfb1nch_dn6, locals.var_dvfb1nch_dn7, locals.var_dvfb1nch_dn8, locals.var_dvfb1nch_dn9,)
    }
};
        locals.var_dvfb1nch = assign6660_e6558;
        locals.var_dvfb1nch_dn4 = assign6660_e6558_d_n4;
        locals.var_dvfb1nch_dn6 = assign6660_e6558_d_n6;
        locals.var_dvfb1nch_dn7 = assign6660_e6558_d_n7;
        locals.var_dvfb1nch_dn8 = assign6660_e6558_d_n8;
        locals.var_dvfb1nch_dn9 = assign6660_e6558_d_n9;

        let (assign6670_e6570, assign6670_e6570_d_n4, assign6670_e6570_d_n6, assign6670_e6570_d_n7, assign6670_e6570_d_n8, assign6670_e6570_d_n9,) = {
    if (locals.var_guard140 == 0.0) {
        let assign6670_e6562: f64 = (-locals.var_temp);
        let assign6670_e6566: f64 = (p.p13 * 4e-10);
        let assign6670_e6567: f64 = (locals.var_tox2_i + assign6670_e6566);
        let assign6670_e6568: f64 = (assign6670_e6562 * assign6670_e6567);
        (assign6670_e6568, ((-locals.var_temp_dn4) * assign6670_e6567), ((-locals.var_temp_dn6) * assign6670_e6567), ((-locals.var_temp_dn7) * assign6670_e6567), ((-locals.var_temp_dn8) * assign6670_e6567), ((-locals.var_temp_dn9) * assign6670_e6567),)
    } else {
        (locals.var_dvfb2nch, locals.var_dvfb2nch_dn4, locals.var_dvfb2nch_dn6, locals.var_dvfb2nch_dn7, locals.var_dvfb2nch_dn8, locals.var_dvfb2nch_dn9,)
    }
};
        locals.var_dvfb2nch = assign6670_e6570;
        locals.var_dvfb2nch_dn4 = assign6670_e6570_d_n4;
        locals.var_dvfb2nch_dn6 = assign6670_e6570_d_n6;
        locals.var_dvfb2nch_dn7 = assign6670_e6570_d_n7;
        locals.var_dvfb2nch_dn8 = assign6670_e6570_d_n8;
        locals.var_dvfb2nch_dn9 = assign6670_e6570_d_n9;

        let assign6680_e6573: f64 = (locals.var_tkc * 0.0033333333333);
        let assign6680_e6574: f64 = (assign6680_e6573).sqrt();
        locals.var_temp = assign6680_e6574;
        locals.var_temp_dn4 = ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_dn6 = ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_dn7 = ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_dn8 = ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign6680_e6574));
        locals.var_temp_dn9 = ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign6680_e6574));

        let assign6690_e6577: f64 = (4.05e25 * locals.var_temp);
        let assign6690_e6579: f64 = (assign6690_e6577 * locals.var_temp);
        let assign6690_e6581: f64 = (assign6690_e6579 * locals.var_temp);
        locals.var_temp1 = assign6690_e6581;
        locals.var_temp1_dn4 = (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn4)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn4));
        locals.var_temp1_dn6 = (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn6)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn6));
        locals.var_temp1_dn7 = (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn7)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn7));
        locals.var_temp1_dn8 = (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn8)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn8));
        locals.var_temp1_dn9 = (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn9)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn9));

        let assign6700_e6584: f64 = (locals.var_temp1 * locals.var_niratio);
        locals.var_neff = assign6700_e6584;
        locals.var_neff_dn4 = (locals.var_temp1_dn4 * locals.var_niratio);
        locals.var_neff_dn6 = (locals.var_temp1_dn6 * locals.var_niratio);
        locals.var_neff_dn7 = (locals.var_temp1_dn7 * locals.var_niratio);
        locals.var_neff_dn8 = (locals.var_temp1_dn8 * locals.var_niratio);
        locals.var_neff_dn9 = (locals.var_temp1_dn9 * locals.var_niratio);

        let assign6710_e6588: f64 = (0.5 * locals.var_deg);
        let assign6710_e6590: f64 = (assign6710_e6588 * locals.var_inv_phit0);
        let assign6710_e6591: f64 = (assign6710_e6590).exp();
        let assign6710_e6592: f64 = (locals.var_temp1 * assign6710_e6591);
        locals.var_neff_poly = assign6710_e6592;
        locals.var_neff_poly_dn4 = ((locals.var_temp1_dn4 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn4) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn4)))));
        locals.var_neff_poly_dn6 = ((locals.var_temp1_dn6 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn6) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn6)))));
        locals.var_neff_poly_dn7 = ((locals.var_temp1_dn7 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn7) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn7)))));
        locals.var_neff_poly_dn8 = ((locals.var_temp1_dn8 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn8) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn8)))));
        locals.var_neff_poly_dn9 = ((locals.var_temp1_dn9 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn9) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn9)))));

        let assign6720_e6596: f64 = (0.5 * locals.var_deg);
        let assign6720_e6598: f64 = (assign6720_e6596 * locals.var_inv_phit0);
        let assign6720_e6599: f64 = (assign6720_e6598).exp();
        let assign6720_e6600: f64 = (locals.var_temp1 * assign6720_e6599);
        locals.var_neff_sub = assign6720_e6600;
        locals.var_neff_sub_dn4 = ((locals.var_temp1_dn4 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn4) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn4)))));
        locals.var_neff_sub_dn6 = ((locals.var_temp1_dn6 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn6) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn6)))));
        locals.var_neff_sub_dn7 = ((locals.var_temp1_dn7 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn7) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn7)))));
        locals.var_neff_sub_dn8 = ((locals.var_temp1_dn8 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn8) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn8)))));
        locals.var_neff_sub_dn9 = ((locals.var_temp1_dn9 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn9) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn9)))));

        let assign6730_e6603: f64 = (3.45313e-11 / locals.var_tox1_i);
        locals.var_cox1init = assign6730_e6603;

        let assign6740_e6606: f64 = (3.45313e-11 / locals.var_tox2_i);
        locals.var_cox2init = assign6740_e6606;

        let assign6750_e6609: f64 = if locals.var_pnce_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign6750_e6609;

        let (assign6760_e6617,) = {
    if (locals.var_guard141 != 0.0) {
        let assign6760_e6614: f64 = (1.0 + locals.var_pnce_i);
        let assign6760_e6615: f64 = (locals.var_cox1init * assign6760_e6614);
        (assign6760_e6615,)
    } else {
        (locals.var_cox1prime,)
    }
};
        locals.var_cox1prime = assign6760_e6617;

        let (assign6770_e6621,) = {
    if (locals.var_guard141 != 0.0) {
        (locals.var_cox2init,)
    } else {
        (locals.var_cox2prime,)
    }
};
        locals.var_cox2prime = assign6770_e6621;

        let (assign6780_e6626,) = {
    if (locals.var_guard141 == 0.0) {
        (locals.var_cox1init,)
    } else {
        (locals.var_cox1prime,)
    }
};
        locals.var_cox1prime = assign6780_e6626;

        let (assign6790_e6635,) = {
    if (locals.var_guard141 == 0.0) {
        let assign6790_e6632: f64 = (1.0 - locals.var_pnce_i);
        let assign6790_e6633: f64 = (locals.var_cox2init * assign6790_e6632);
        (assign6790_e6633,)
    } else {
        (locals.var_cox2prime,)
    }
};
        locals.var_cox2prime = assign6790_e6635;

        let assign6800_e6638: f64 = (locals.var_epsch / locals.var_tsi_i);
        locals.var_csiprime_0 = assign6800_e6638;

        let assign6810_e6643: f64 = (locals.var_ct_i * locals.var_rtn);
        let assign6810_e6644: f64 = (1.0 + assign6810_e6643);
        let assign6810_e6645: f64 = (locals.var_phit0 * assign6810_e6644);
        locals.var_phit = assign6810_e6645;
        locals.var_phit_dn4 = ((locals.var_phit0_dn4 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4)));
        locals.var_phit_dn6 = ((locals.var_phit0_dn6 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6)));
        locals.var_phit_dn7 = ((locals.var_phit0_dn7 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7)));
        locals.var_phit_dn8 = ((locals.var_phit0_dn8 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8)));
        locals.var_phit_dn9 = ((locals.var_phit0_dn9 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9)));

        let assign6820_e6648: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign6820_e6648;
        locals.var_inv_phit_dn4 = (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_dn6 = (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_dn7 = (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_dn8 = (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit)));
        locals.var_inv_phit_dn9 = (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit)));

        let assign6830_e6651: f64 = (0.5 * locals.var_eg);
        let assign6830_e6653: f64 = (assign6830_e6651 * locals.var_inv_phit);
        locals.var_eg_2phit = assign6830_e6653;
        locals.var_eg_2phit_dn4 = (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn4));
        locals.var_eg_2phit_dn6 = (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn6));
        locals.var_eg_2phit_dn7 = (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn7));
        locals.var_eg_2phit_dn8 = (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn8));
        locals.var_eg_2phit_dn9 = (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn9));

        let assign6840_e6656: f64 = (locals.var_cox1prime / locals.var_csiprime_0);
        locals.var_k1_1d = assign6840_e6656;

        let assign6850_e6659: f64 = (locals.var_cox2prime / locals.var_csiprime_0);
        locals.var_k2_1d = assign6850_e6659;

        let assign6860_e6664: f64 = (1.0 / locals.var_k1_1d);
        let assign6860_e6665: f64 = (1.0 + assign6860_e6664);
        let assign6860_e6668: f64 = (1.0 / locals.var_k2_1d);
        let assign6860_e6669: f64 = (assign6860_e6665 + assign6860_e6668);
        let assign6860_e6670: f64 = (1.0 / assign6860_e6669);
        locals.var_keq_1d = assign6860_e6670;

        let assign6870_e6673: f64 = (2.0 * 1.602176565e-19);
        let assign6870_e6675: f64 = (assign6870_e6673 * locals.var_neff);
        let assign6870_e6677: f64 = (assign6870_e6675 * locals.var_epsch);
        let assign6870_e6679: f64 = (assign6870_e6677 * locals.var_inv_phit);
        locals.var_a0_csisq = assign6870_e6679;
        locals.var_a0_csisq_dn4 = ((((assign6870_e6673 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn4));
        locals.var_a0_csisq_dn6 = ((((assign6870_e6673 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn6));
        locals.var_a0_csisq_dn7 = ((((assign6870_e6673 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn7));
        locals.var_a0_csisq_dn8 = ((((assign6870_e6673 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn8));
        locals.var_a0_csisq_dn9 = ((((assign6870_e6673 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn9));

        let assign6880_e6682: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign6880_e6684: f64 = (assign6880_e6682 / locals.var_a0_csisq);
        let assign6880_e6685: f64 = (assign6880_e6684).ln();
        let assign6880_e6687: f64 = (assign6880_e6685 - 0.6931471805599);
        locals.var_xth_1d = assign6880_e6687;
        locals.var_xth_1d_dn4 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_dn6 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_dn7 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_dn8 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);
        locals.var_xth_1d_dn9 = ((-((assign6880_e6682 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684);

        let assign6890_e6690: f64 = (0.5 * 1.602176565e-19);
        let assign6890_e6692: f64 = (assign6890_e6690 * locals.var_nsddc_i);
        let assign6890_e6694: f64 = (assign6890_e6692 * locals.var_tsi_i);
        let assign6890_e6697: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign6890_e6698: f64 = (assign6890_e6694 / assign6890_e6697);
        let assign6890_e6700: f64 = (assign6890_e6698 * locals.var_inv_phit);
        locals.var_xsddep = assign6890_e6700;
        locals.var_xsddep_dn4 = (assign6890_e6698 * locals.var_inv_phit_dn4);
        locals.var_xsddep_dn6 = (assign6890_e6698 * locals.var_inv_phit_dn6);
        locals.var_xsddep_dn7 = (assign6890_e6698 * locals.var_inv_phit_dn7);
        locals.var_xsddep_dn8 = (assign6890_e6698 * locals.var_inv_phit_dn8);
        locals.var_xsddep_dn9 = (assign6890_e6698 * locals.var_inv_phit_dn9);

        let assign6900_e6703: f64 = (locals.var_stcf_i * locals.var_dt);
        locals.var_temp = assign6900_e6703;
        locals.var_temp_dn4 = ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4));
        locals.var_temp_dn6 = ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6));
        locals.var_temp_dn7 = ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7));
        locals.var_temp_dn8 = ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8));
        locals.var_temp_dn9 = ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9));

        let assign6910_e6706: f64 = (locals.var_cf1_t + locals.var_temp);
        locals.var_cf1_i = assign6910_e6706;
        locals.var_cf1_i_dn4 = (locals.var_cf1_t_dn4 + locals.var_temp_dn4);
        locals.var_cf1_i_dn6 = (locals.var_cf1_t_dn6 + locals.var_temp_dn6);
        locals.var_cf1_i_dn7 = (locals.var_cf1_t_dn7 + locals.var_temp_dn7);
        locals.var_cf1_i_dn8 = (locals.var_cf1_t_dn8 + locals.var_temp_dn8);
        locals.var_cf1_i_dn9 = (locals.var_cf1_t_dn9 + locals.var_temp_dn9);

        let assign6920_e6709: f64 = (locals.var_cf2_t + locals.var_temp);
        locals.var_cf2_i = assign6920_e6709;
        locals.var_cf2_i_dn4 = (locals.var_cf2_t_dn4 + locals.var_temp_dn4);
        locals.var_cf2_i_dn6 = (locals.var_cf2_t_dn6 + locals.var_temp_dn6);
        locals.var_cf2_i_dn7 = (locals.var_cf2_t_dn7 + locals.var_temp_dn7);
        locals.var_cf2_i_dn8 = (locals.var_cf2_t_dn8 + locals.var_temp_dn8);
        locals.var_cf2_i_dn9 = (locals.var_cf2_t_dn9 + locals.var_temp_dn9);

        let assign6930_e6712: f64 = (locals.var_cfac1_t + locals.var_temp);
        locals.var_cfac1_i = assign6930_e6712;
        locals.var_cfac1_i_dn4 = (locals.var_cfac1_t_dn4 + locals.var_temp_dn4);
        locals.var_cfac1_i_dn6 = (locals.var_cfac1_t_dn6 + locals.var_temp_dn6);
        locals.var_cfac1_i_dn7 = (locals.var_cfac1_t_dn7 + locals.var_temp_dn7);
        locals.var_cfac1_i_dn8 = (locals.var_cfac1_t_dn8 + locals.var_temp_dn8);
        locals.var_cfac1_i_dn9 = (locals.var_cfac1_t_dn9 + locals.var_temp_dn9);

        let assign6940_e6715: f64 = (locals.var_cfac2_t + locals.var_temp);
        locals.var_cfac2_i = assign6940_e6715;
        locals.var_cfac2_i_dn4 = (locals.var_cfac2_t_dn4 + locals.var_temp_dn4);
        locals.var_cfac2_i_dn6 = (locals.var_cfac2_t_dn6 + locals.var_temp_dn6);
        locals.var_cfac2_i_dn7 = (locals.var_cfac2_t_dn7 + locals.var_temp_dn7);
        locals.var_cfac2_i_dn8 = (locals.var_cfac2_t_dn8 + locals.var_temp_dn8);
        locals.var_cfac2_i_dn9 = (locals.var_cfac2_t_dn9 + locals.var_temp_dn9);

        let assign6950_e6718: f64 = (locals.var_cfd_i * locals.var_inv_phit);
        locals.var_xd0 = assign6950_e6718;
        locals.var_xd0_dn4 = (locals.var_cfd_i * locals.var_inv_phit_dn4);
        locals.var_xd0_dn6 = (locals.var_cfd_i * locals.var_inv_phit_dn6);
        locals.var_xd0_dn7 = (locals.var_cfd_i * locals.var_inv_phit_dn7);
        locals.var_xd0_dn8 = (locals.var_cfd_i * locals.var_inv_phit_dn8);
        locals.var_xd0_dn9 = (locals.var_cfd_i * locals.var_inv_phit_dn9);

        let assign6960_e6721: f64 = (2.0 * 1.602176565e-19);
        let assign6960_e6723: f64 = (assign6960_e6721 * locals.var_nsub_i);
        let assign6960_e6725: f64 = (assign6960_e6723 * 1.04479e-10);
        let assign6960_e6727: f64 = (assign6960_e6725 * locals.var_inv_phit0);
        let assign6960_e6728: f64 = (assign6960_e6727).sqrt();
        let assign6960_e6730: f64 = (assign6960_e6728 / locals.var_cox2prime);
        locals.var_gfsub = assign6960_e6730;
        locals.var_gfsub_dn4 = (((assign6960_e6725 * locals.var_inv_phit0_dn4) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_dn6 = (((assign6960_e6725 * locals.var_inv_phit0_dn6) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_dn7 = (((assign6960_e6725 * locals.var_inv_phit0_dn7) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_dn8 = (((assign6960_e6725 * locals.var_inv_phit0_dn8) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);
        locals.var_gfsub_dn9 = (((assign6960_e6725 * locals.var_inv_phit0_dn9) / (2.0 * assign6960_e6728)) / locals.var_cox2prime);

        let assign6970_e6733: f64 = (locals.var_gfsub * locals.var_gfsub);
        locals.var_gfsub2 = assign6970_e6733;
        locals.var_gfsub2_dn4 = ((locals.var_gfsub_dn4 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn4));
        locals.var_gfsub2_dn6 = ((locals.var_gfsub_dn6 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn6));
        locals.var_gfsub2_dn7 = ((locals.var_gfsub_dn7 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn7));
        locals.var_gfsub2_dn8 = ((locals.var_gfsub_dn8 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn8));
        locals.var_gfsub2_dn9 = ((locals.var_gfsub_dn9 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn9));

        let assign6980_e6736: f64 = (1.0 / locals.var_gfsub2);
        locals.var_inv_gfsub2 = assign6980_e6736;
        locals.var_inv_gfsub2_dn4 = (-(locals.var_gfsub2_dn4 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_dn6 = (-(locals.var_gfsub2_dn6 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_dn7 = (-(locals.var_gfsub2_dn7 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_dn8 = (-(locals.var_gfsub2_dn8 / (locals.var_gfsub2 * locals.var_gfsub2)));
        locals.var_inv_gfsub2_dn9 = (-(locals.var_gfsub2_dn9 / (locals.var_gfsub2 * locals.var_gfsub2)));

        let assign6990_e6740: f64 = (locals.var_gfsub / 1.4142135623731);
        let assign6990_e6741: f64 = (1.0 + assign6990_e6740);
        locals.var_xisub = assign6990_e6741;
        locals.var_xisub_dn4 = (locals.var_gfsub_dn4 / 1.4142135623731);
        locals.var_xisub_dn6 = (locals.var_gfsub_dn6 / 1.4142135623731);
        locals.var_xisub_dn7 = (locals.var_gfsub_dn7 / 1.4142135623731);
        locals.var_xisub_dn8 = (locals.var_gfsub_dn8 / 1.4142135623731);
        locals.var_xisub_dn9 = (locals.var_gfsub_dn9 / 1.4142135623731);

        let assign7000_e6744: f64 = (1.0 / locals.var_xisub);
        locals.var_inv_xisub = assign7000_e6744;
        locals.var_inv_xisub_dn4 = (-(locals.var_xisub_dn4 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_dn6 = (-(locals.var_xisub_dn6 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_dn7 = (-(locals.var_xisub_dn7 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_dn8 = (-(locals.var_xisub_dn8 / (locals.var_xisub * locals.var_xisub)));
        locals.var_inv_xisub_dn9 = (-(locals.var_xisub_dn9 / (locals.var_xisub * locals.var_xisub)));

        let assign7010_e6747: f64 = (1e-5 * locals.var_xisub);
        locals.var_margin_sub = assign7010_e6747;

        let assign7020_e6750: f64 = (locals.var_nsub_i / locals.var_neff_sub);
        let assign7020_e6751: f64 = (assign7020_e6750).ln();
        let assign7020_e6753: f64 = (assign7020_e6751 + locals.var_eg_2phit0);
        locals.var_xb_sub = assign7020_e6753;
        locals.var_xb_sub_dn4 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn4) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn4);
        locals.var_xb_sub_dn6 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn6) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn6);
        locals.var_xb_sub_dn7 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn7) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn7);
        locals.var_xb_sub_dn8 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn8) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn8);
        locals.var_xb_sub_dn9 = (((-((locals.var_nsub_i * locals.var_neff_sub_dn9) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn9);

        let assign7030_e6756: f64 = (2.0 * locals.var_xb_sub);
        locals.var_xn_sub = assign7030_e6756;
        locals.var_xn_sub_dn4 = (2.0 * locals.var_xb_sub_dn4);
        locals.var_xn_sub_dn6 = (2.0 * locals.var_xb_sub_dn6);
        locals.var_xn_sub_dn7 = (2.0 * locals.var_xb_sub_dn7);
        locals.var_xn_sub_dn8 = (2.0 * locals.var_xb_sub_dn8);
        locals.var_xn_sub_dn9 = (2.0 * locals.var_xb_sub_dn9);

        let assign7040_e6759: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7040_e6759;

        let (assign7050_e6769, assign7050_e6769_d_n4, assign7050_e6769_d_n6, assign7050_e6769_d_n7, assign7050_e6769_d_n8, assign7050_e6769_d_n9,) = {
    if (locals.var_guard142 != 0.0) {
        let assign7050_e6764: f64 = (locals.var_typesub_i * locals.var_phit0);
        let assign7050_e6766: f64 = (assign7050_e6764 * locals.var_xb_sub);
        let assign7050_e6767: f64 = (locals.var_vfb2_t + assign7050_e6766);
        (assign7050_e6767, (locals.var_vfb2_t_dn4 + (((locals.var_typesub_i * locals.var_phit0_dn4) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn4))), (locals.var_vfb2_t_dn6 + (((locals.var_typesub_i * locals.var_phit0_dn6) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn6))), (locals.var_vfb2_t_dn7 + (((locals.var_typesub_i * locals.var_phit0_dn7) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn7))), (locals.var_vfb2_t_dn8 + (((locals.var_typesub_i * locals.var_phit0_dn8) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn8))), (locals.var_vfb2_t_dn9 + (((locals.var_typesub_i * locals.var_phit0_dn9) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn9))),)
    } else {
        (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9,)
    }
};
        locals.var_vfb2_t = assign7050_e6769;
        locals.var_vfb2_t_dn4 = assign7050_e6769_d_n4;
        locals.var_vfb2_t_dn6 = assign7050_e6769_d_n6;
        locals.var_vfb2_t_dn7 = assign7050_e6769_d_n7;
        locals.var_vfb2_t_dn8 = assign7050_e6769_d_n8;
        locals.var_vfb2_t_dn9 = assign7050_e6769_d_n9;

        let (assign7060_e6779, assign7060_e6779_d_n4, assign7060_e6779_d_n6, assign7060_e6779_d_n7, assign7060_e6779_d_n8, assign7060_e6779_d_n9,) = {
    if (locals.var_guard142 != 0.0) {
        let assign7060_e6774: f64 = (locals.var_typesub_i * locals.var_phit0);
        let assign7060_e6776: f64 = (assign7060_e6774 * locals.var_xb_sub);
        let assign7060_e6777: f64 = (locals.var_vfbac2_t + assign7060_e6776);
        (assign7060_e6777, (locals.var_vfbac2_t_dn4 + (((locals.var_typesub_i * locals.var_phit0_dn4) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn4))), (locals.var_vfbac2_t_dn6 + (((locals.var_typesub_i * locals.var_phit0_dn6) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn6))), (locals.var_vfbac2_t_dn7 + (((locals.var_typesub_i * locals.var_phit0_dn7) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn7))), (locals.var_vfbac2_t_dn8 + (((locals.var_typesub_i * locals.var_phit0_dn8) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn8))), (locals.var_vfbac2_t_dn9 + (((locals.var_typesub_i * locals.var_phit0_dn9) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn9))),)
    } else {
        (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9,)
    }
};
        locals.var_vfbac2_t = assign7060_e6779;
        locals.var_vfbac2_t_dn4 = assign7060_e6779_d_n4;
        locals.var_vfbac2_t_dn6 = assign7060_e6779_d_n6;
        locals.var_vfbac2_t_dn7 = assign7060_e6779_d_n7;
        locals.var_vfbac2_t_dn8 = assign7060_e6779_d_n8;
        locals.var_vfbac2_t_dn9 = assign7060_e6779_d_n9;

        locals.var_dvfbpdep = 0.0;
        locals.var_dvfbpdep_dn4 = 0.0;
        locals.var_dvfbpdep_dn6 = 0.0;
        locals.var_dvfbpdep_dn7 = 0.0;
        locals.var_dvfbpdep_dn8 = 0.0;
        locals.var_dvfbpdep_dn9 = 0.0;

        let assign7080_e6783: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7080_e6783;

        let (assign7090_e6794, assign7090_e6794_d_n4, assign7090_e6794_d_n6, assign7090_e6794_d_n7, assign7090_e6794_d_n8, assign7090_e6794_d_n9,) = {
    if (locals.var_guard143 != 0.0) {
        let assign7090_e6788: f64 = (locals.var_np_i / locals.var_neff_poly);
        let assign7090_e6789: f64 = (assign7090_e6788).ln();
        let assign7090_e6791: f64 = (assign7090_e6789 + locals.var_eg_2phit0);
        let assign7090_e6792: f64 = (locals.var_phit0 * assign7090_e6791);
        (assign7090_e6792, ((locals.var_phit0_dn4 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn4))), ((locals.var_phit0_dn6 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn6))), ((locals.var_phit0_dn7 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn7))), ((locals.var_phit0_dn8 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn8))), ((locals.var_phit0_dn9 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn9))),)
    } else {
        (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9,)
    }
};
        locals.var_dvfbpdep = assign7090_e6794;
        locals.var_dvfbpdep_dn4 = assign7090_e6794_d_n4;
        locals.var_dvfbpdep_dn6 = assign7090_e6794_d_n6;
        locals.var_dvfbpdep_dn7 = assign7090_e6794_d_n7;
        locals.var_dvfbpdep_dn8 = assign7090_e6794_d_n8;
        locals.var_dvfbpdep_dn9 = assign7090_e6794_d_n9;

        let assign7100_e6797: f64 = (2.0 * 1.602176565e-19);
        let assign7100_e6799: f64 = (assign7100_e6797 * locals.var_epsch);
        let assign7100_e6801: f64 = (assign7100_e6799 * locals.var_np_i);
        let assign7100_e6802: f64 = (assign7100_e6801).sqrt();
        let assign7100_e6804: f64 = (assign7100_e6802 / locals.var_cox1init);
        locals.var_kp = assign7100_e6804;
        locals.var_kp_dn4 = (((assign7100_e6799 * locals.var_np_i_dn4) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_dn6 = (((assign7100_e6799 * locals.var_np_i_dn6) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_dn7 = (((assign7100_e6799 * locals.var_np_i_dn7) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_dn8 = (((assign7100_e6799 * locals.var_np_i_dn8) / (2.0 * assign7100_e6802)) / locals.var_cox1init);
        locals.var_kp_dn9 = (((assign7100_e6799 * locals.var_np_i_dn9) / (2.0 * assign7100_e6802)) / locals.var_cox1init);

        locals.var_emin = 15.0;
        locals.var_emin_dn4 = 0.0;
        locals.var_emin_dn6 = 0.0;
        locals.var_emin_dn7 = 0.0;
        locals.var_emin_dn8 = 0.0;
        locals.var_emin_dn9 = 0.0;

    }
}
