#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        locals.var_vgs = (nv8 - nv5);
        locals.var_vgs_dn5 = -1.0;
        locals.var_vgs_dn8 = 1.0;

        locals.var_vgd = (nv4 - nv3);
        locals.var_vgd_dn3 = -1.0;
        locals.var_vgd_dn4 = 1.0;

        let assign20_e564: f64 = (-locals.var_vgd);
        locals.var_vdg = assign20_e564;
        locals.var_vdg_dn3 = (-locals.var_vgd_dn3);
        locals.var_vdg_dn4 = (-locals.var_vgd_dn4);

        locals.var_vds = (nv3 - nv5);
        locals.var_vds_dn3 = 1.0;
        locals.var_vds_dn5 = -1.0;

        locals.var_vgsc = locals.var_vgs;
        locals.var_vgsc_dn5 = locals.var_vgs_dn5;
        locals.var_vgsc_dn8 = locals.var_vgs_dn8;

        locals.var_vgdc = (nv7 - nv3);
        locals.var_vgdc_dn3 = -1.0;
        locals.var_vgdc_dn7 = 1.0;

        locals.var_qgd = 0.0;
        locals.var_qgd_dn3 = 0.0;
        locals.var_qgd_dn5 = 0.0;
        locals.var_qgd_dn7 = 0.0;
        locals.var_qgd_dn8 = 0.0;
        locals.var_qgd_dn11 = 0.0;

        locals.var_qgs = 0.0;
        locals.var_qgs_dn3 = 0.0;
        locals.var_qgs_dn5 = 0.0;
        locals.var_qgs_dn7 = 0.0;
        locals.var_qgs_dn8 = 0.0;
        locals.var_qgs_dn11 = 0.0;

        locals.var_cgd = 0.0;
        locals.var_cgd_dn3 = 0.0;
        locals.var_cgd_dn5 = 0.0;
        locals.var_cgd_dn7 = 0.0;
        locals.var_cgd_dn8 = 0.0;
        locals.var_cgd_dn11 = 0.0;

        locals.var_cgs = 0.0;
        locals.var_cgs_dn3 = 0.0;
        locals.var_cgs_dn5 = 0.0;
        locals.var_cgs_dn7 = 0.0;
        locals.var_cgs_dn8 = 0.0;
        locals.var_cgs_dn11 = 0.0;

        let assign120_e575: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard1 = assign120_e575;

        let (assign130_e581, assign130_e581_d_n11,) = {
    if (locals.var_guard1 != 0.0) {
        let assign130_e579: f64 = (p.p3 + 273.15);
        (assign130_e579, 0.0,)
    } else {
        (locals.var_t, locals.var_t_dn11,)
    }
};
        locals.var_t = assign130_e581;
        locals.var_t_dn11 = assign130_e581_d_n11;

        let (assign140_e588, assign140_e588_d_n11,) = {
    if (locals.var_guard1 == 0.0) {
        let assign140_e584: f64 = ctx_temp;
        let assign140_e586: f64 = (assign140_e584 + p.p2);
        (assign140_e586, 0.0,)
    } else {
        (locals.var_t, locals.var_t_dn11,)
    }
};
        locals.var_t = assign140_e588;
        locals.var_t_dn11 = assign140_e588_d_n11;

        let assign150_e590: f64 = if param_given[85] { 1.0 } else { 0.0 };
        locals.var_guard2 = assign150_e590;

        let (assign160_e596,) = {
    if (locals.var_guard2 != 0.0) {
        let assign160_e594: f64 = (p.p85 + 273.15);
        (assign160_e594,)
    } else {
        (locals.var_t_nom,)
    }
};
        locals.var_t_nom = assign160_e596;

        let (assign170_e603,) = {
    if (locals.var_guard2 == 0.0) {
        let assign170_e601: f64 = (27.0 + 273.15);
        (assign170_e601,)
    } else {
        (locals.var_t_nom,)
    }
};
        locals.var_t_nom = assign170_e603;

        let (assign180_e610, assign180_e610_d_n11,) = {
    if (p.p1 != 0.0) {
        let assign180_e607: f64 = ((nv11 - 0.0)).abs();
        let assign180_e608: f64 = (locals.var_t + assign180_e607);
        (assign180_e608, (locals.var_t_dn11 + if (nv11 - 0.0) >= 0.0 { 1.0 } else { (-1.0) }),)
    } else {
        (locals.var_t, locals.var_t_dn11,)
    }
};
        locals.var_t = assign180_e610;
        locals.var_t_dn11 = assign180_e610_d_n11;

        let assign190_e612: f64 = (locals.var_t * THERMAL_VOLTAGE_PER_K);
        locals.var_vth = assign190_e612;
        locals.var_vth_dn11 = (locals.var_t_dn11 * THERMAL_VOLTAGE_PER_K);

        let assign200_e615: f64 = (locals.var_t - locals.var_t_nom);
        let assign200_e616: f64 = (assign200_e615).abs();
        locals.var_delta_t = assign200_e616;
        locals.var_delta_t_dn11 = if assign200_e615 >= 0.0 { locals.var_t_dn11 } else { (-locals.var_t_dn11) };

        let assign210_e623: f64 = if ((locals.var_delta_t > 0.0) || (p.p57 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard3 = assign210_e623;

        let (assign240_e653, assign240_e653_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign240_e649: f64 = (p.p60 * locals.var_delta_t);
        let assign240_e650: f64 = (1.0 + assign240_e649);
        let assign240_e651: f64 = (p.p11 * assign240_e650);
        (assign240_e651, (p.p11 * (p.p60 * locals.var_delta_t_dn11)),)
    } else {
        (locals.var_p1_t, locals.var_p1_t_dn11,)
    }
};
        locals.var_p1_t = assign240_e653;
        locals.var_p1_t_dn11 = assign240_e653_d_n11;

        let (assign260_e673, assign260_e673_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign260_e669: f64 = (p.p61 * locals.var_delta_t);
        let assign260_e670: f64 = (1.0 + assign260_e669);
        let assign260_e671: f64 = (p.p25 * assign260_e670);
        (assign260_e671, (p.p25 * (p.p61 * locals.var_delta_t_dn11)),)
    } else {
        (locals.var_cgs0_t, locals.var_cgs0_t_dn11,)
    }
};
        locals.var_cgs0_t = assign260_e673;
        locals.var_cgs0_t_dn11 = assign260_e673_d_n11;

        let (assign270_e683, assign270_e683_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign270_e679: f64 = (p.p62 * locals.var_delta_t);
        let assign270_e680: f64 = (1.0 + assign270_e679);
        let assign270_e681: f64 = (p.p28 * assign270_e680);
        (assign270_e681, (p.p28 * (p.p62 * locals.var_delta_t_dn11)),)
    } else {
        (locals.var_cgd0_t, locals.var_cgd0_t_dn11,)
    }
};
        locals.var_cgd0_t = assign270_e683;
        locals.var_cgd0_t_dn11 = assign270_e683_d_n11;

        let (assign300_e711, assign300_e711_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign300_e708: f64 = (p.p68 * locals.var_delta_t);
        let assign300_e709: f64 = (p.p9 + assign300_e708);
        (assign300_e709, (p.p68 * locals.var_delta_t_dn11),)
    } else {
        (locals.var_vpks_t, locals.var_vpks_t_dn11,)
    }
};
        locals.var_vpks_t = assign300_e711;
        locals.var_vpks_t_dn11 = assign300_e711_d_n11;

        let (assign310_e721, assign310_e721_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign310_e716: f64 = (p.p30 * p.p68);
        let assign310_e718: f64 = (assign310_e716 * locals.var_delta_t);
        let assign310_e719: f64 = (p.p29 + assign310_e718);
        (assign310_e719, (assign310_e716 * locals.var_delta_t_dn11),)
    } else {
        (locals.var_p10_t, locals.var_p10_t_dn11,)
    }
};
        locals.var_p10_t = assign310_e721;
        locals.var_p10_t_dn11 = assign310_e721_d_n11;

        let (assign320_e731, assign320_e731_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign320_e726: f64 = (p.p36 * p.p68);
        let assign320_e728: f64 = (assign320_e726 * locals.var_delta_t);
        let assign320_e729: f64 = (p.p35 + assign320_e728);
        (assign320_e729, (assign320_e726 * locals.var_delta_t_dn11),)
    } else {
        (locals.var_p40_t, locals.var_p40_t_dn11,)
    }
};
        locals.var_p40_t = assign320_e731;
        locals.var_p40_t_dn11 = assign320_e731_d_n11;

        let (assign330_e739, assign330_e739_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign330_e736: f64 = (p.p69 * locals.var_delta_t);
        let assign330_e737: f64 = (p.p41 + assign330_e736);
        (assign330_e737, (p.p69 * locals.var_delta_t_dn11),)
    } else {
        (locals.var_vjg_t, locals.var_vjg_t_dn11,)
    }
};
        locals.var_vjg_t = assign330_e739;
        locals.var_vjg_t_dn11 = assign330_e739_d_n11;

        let (assign340_e747, assign340_e747_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign340_e744: f64 = (p.p70 * locals.var_delta_t);
        let assign340_e745: f64 = (p.p21 + assign340_e744);
        (assign340_e745, (p.p70 * locals.var_delta_t_dn11),)
    } else {
        (locals.var_vtr_t, locals.var_vtr_t_dn11,)
    }
};
        locals.var_vtr_t = assign340_e747;
        locals.var_vtr_t_dn11 = assign340_e747_d_n11;

        let (assign360_e757, assign360_e757_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p11, 0.0,)
    } else {
        (locals.var_p1_t, locals.var_p1_t_dn11,)
    }
};
        locals.var_p1_t = assign360_e757;
        locals.var_p1_t_dn11 = assign360_e757_d_n11;

        let (assign380_e767, assign380_e767_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p25, 0.0,)
    } else {
        (locals.var_cgs0_t, locals.var_cgs0_t_dn11,)
    }
};
        locals.var_cgs0_t = assign380_e767;
        locals.var_cgs0_t_dn11 = assign380_e767_d_n11;

        let (assign390_e772, assign390_e772_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p28, 0.0,)
    } else {
        (locals.var_cgd0_t, locals.var_cgd0_t_dn11,)
    }
};
        locals.var_cgd0_t = assign390_e772;
        locals.var_cgd0_t_dn11 = assign390_e772_d_n11;

        let (assign420_e787, assign420_e787_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p9, 0.0,)
    } else {
        (locals.var_vpks_t, locals.var_vpks_t_dn11,)
    }
};
        locals.var_vpks_t = assign420_e787;
        locals.var_vpks_t_dn11 = assign420_e787_d_n11;

        let (assign430_e792, assign430_e792_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p29, 0.0,)
    } else {
        (locals.var_p10_t, locals.var_p10_t_dn11,)
    }
};
        locals.var_p10_t = assign430_e792;
        locals.var_p10_t_dn11 = assign430_e792_d_n11;

        let (assign440_e797, assign440_e797_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p35, 0.0,)
    } else {
        (locals.var_p40_t, locals.var_p40_t_dn11,)
    }
};
        locals.var_p40_t = assign440_e797;
        locals.var_p40_t_dn11 = assign440_e797_d_n11;

        let (assign450_e802, assign450_e802_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p41, 0.0,)
    } else {
        (locals.var_vjg_t, locals.var_vjg_t_dn11,)
    }
};
        locals.var_vjg_t = assign450_e802;
        locals.var_vjg_t_dn11 = assign450_e802_d_n11;

        let (assign460_e807, assign460_e807_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p21, 0.0,)
    } else {
        (locals.var_vtr_t, locals.var_vtr_t_dn11,)
    }
};
        locals.var_vtr_t = assign460_e807;
        locals.var_vtr_t_dn11 = assign460_e807_d_n11;

        let assign470_e813: f64 = if ((!param_given[39]) && param_given[40]) { 1.0 } else { 0.0 };
        locals.var_guard4 = assign470_e813;

        let (assign480_e821, assign480_e821_d_n11,) = {
    if (locals.var_guard4 != 0.0) {
        let assign480_e817: f64 = (0.5 / p.p40);
        let assign480_e819: f64 = (assign480_e817 / locals.var_vth);
        (assign480_e819, (-((assign480_e817 * locals.var_vth_dn11) / (locals.var_vth * locals.var_vth))),)
    } else {
        (locals.var_pg_param, locals.var_pg_param_dn11,)
    }
};
        locals.var_pg_param = assign480_e821;
        locals.var_pg_param_dn11 = assign480_e821_d_n11;

        let (assign490_e826, assign490_e826_d_n11,) = {
    if (locals.var_guard4 == 0.0) {
        (p.p39, 0.0,)
    } else {
        (locals.var_pg_param, locals.var_pg_param_dn11,)
    }
};
        locals.var_pg_param = assign490_e826;
        locals.var_pg_param_dn11 = assign490_e826_d_n11;

        let assign500_e829: f64 = (p.p19 * locals.var_vds);
        let assign500_e830: f64 = (assign500_e829).cosh();
        locals.var_t0 = assign500_e830;
        locals.var_t0_dn3 = ((assign500_e829).sinh() * (p.p19 * locals.var_vds_dn3));
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = ((assign500_e829).sinh() * (p.p19 * locals.var_vds_dn5));
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn11 = 0.0;

        let assign510_e836: f64 = (locals.var_t0 * locals.var_t0);
        let assign510_e837: f64 = (p.p18 / assign510_e836);
        let assign510_e838: f64 = (1.0 + assign510_e837);
        let assign510_e839: f64 = (locals.var_p1_t * assign510_e838);
        locals.var_p1m = assign510_e839;
        locals.var_p1m_dn3 = (locals.var_p1_t * (-((p.p18 * ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3))) / (assign510_e836 * assign510_e836))));
        locals.var_p1m_dn4 = (locals.var_p1_t * (-((p.p18 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4))) / (assign510_e836 * assign510_e836))));
        locals.var_p1m_dn5 = (locals.var_p1_t * (-((p.p18 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5))) / (assign510_e836 * assign510_e836))));
        locals.var_p1m_dn8 = (locals.var_p1_t * (-((p.p18 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8))) / (assign510_e836 * assign510_e836))));
        locals.var_p1m_dn11 = ((locals.var_p1_t_dn11 * assign510_e838) + (locals.var_p1_t * (-((p.p18 * ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11))) / (assign510_e836 * assign510_e836)))));

        let assign520_e842: f64 = (locals.var_vpks_t - p.p10);
        let assign520_e846: f64 = (p.p15 * locals.var_vds);
        let assign520_e847: f64 = (assign520_e846).tanh();
        let assign520_e848: f64 = (p.p10 * assign520_e847);
        let assign520_e849: f64 = (assign520_e842 + assign520_e848);
        let assign520_e853: f64 = (locals.var_vdg - p.p21);
        let assign520_e854: f64 = (p.p22 * assign520_e853);
        let assign520_e857: f64 = (locals.var_vdg - locals.var_vtr_t);
        let assign520_e858: f64 = (assign520_e854 * assign520_e857);
        let assign520_e859: f64 = (assign520_e849 - assign520_e858);
        locals.var_vpkm = assign520_e859;
        locals.var_vpkm_dn3 = ((p.p10 * ((p.p15 * locals.var_vds_dn3) / ((assign520_e846).cosh() * (assign520_e846).cosh()))) - (((p.p22 * locals.var_vdg_dn3) * assign520_e857) + (assign520_e854 * locals.var_vdg_dn3)));
        locals.var_vpkm_dn4 = (-(((p.p22 * locals.var_vdg_dn4) * assign520_e857) + (assign520_e854 * locals.var_vdg_dn4)));
        locals.var_vpkm_dn5 = (p.p10 * ((p.p15 * locals.var_vds_dn5) / ((assign520_e846).cosh() * (assign520_e846).cosh())));
        locals.var_vpkm_dn11 = (locals.var_vpks_t_dn11 - (assign520_e854 * (-locals.var_vtr_t_dn11)));

        let assign530_e862: f64 = (locals.var_vgs - locals.var_vpkm);
        locals.var_t1 = assign530_e862;
        locals.var_t1_dn3 = (-locals.var_vpkm_dn3);
        locals.var_t1_dn4 = (-locals.var_vpkm_dn4);
        locals.var_t1_dn5 = (locals.var_vgs_dn5 - locals.var_vpkm_dn5);
        locals.var_t1_dn8 = locals.var_vgs_dn8;
        locals.var_t1_dn11 = (-locals.var_vpkm_dn11);

        let assign540_e865: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign540_e865;
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));

        let assign550_e868: f64 = (locals.var_p1m * locals.var_t1);
        let assign550_e871: f64 = (p.p12 * locals.var_t2);
        let assign550_e872: f64 = (assign550_e868 + assign550_e871);
        let assign550_e875: f64 = (p.p13 * locals.var_t1);
        let assign550_e877: f64 = (assign550_e875 * locals.var_t2);
        let assign550_e878: f64 = (assign550_e872 + assign550_e877);
        locals.var_psi = assign550_e878;
        locals.var_psi_dn3 = ((((locals.var_p1m_dn3 * locals.var_t1) + (locals.var_p1m * locals.var_t1_dn3)) + (p.p12 * locals.var_t2_dn3)) + (((p.p13 * locals.var_t1_dn3) * locals.var_t2) + (assign550_e875 * locals.var_t2_dn3)));
        locals.var_psi_dn4 = ((((locals.var_p1m_dn4 * locals.var_t1) + (locals.var_p1m * locals.var_t1_dn4)) + (p.p12 * locals.var_t2_dn4)) + (((p.p13 * locals.var_t1_dn4) * locals.var_t2) + (assign550_e875 * locals.var_t2_dn4)));
        locals.var_psi_dn5 = ((((locals.var_p1m_dn5 * locals.var_t1) + (locals.var_p1m * locals.var_t1_dn5)) + (p.p12 * locals.var_t2_dn5)) + (((p.p13 * locals.var_t1_dn5) * locals.var_t2) + (assign550_e875 * locals.var_t2_dn5)));
        locals.var_psi_dn8 = ((((locals.var_p1m_dn8 * locals.var_t1) + (locals.var_p1m * locals.var_t1_dn8)) + (p.p12 * locals.var_t2_dn8)) + (((p.p13 * locals.var_t1_dn8) * locals.var_t2) + (assign550_e875 * locals.var_t2_dn8)));
        locals.var_psi_dn11 = ((((locals.var_p1m_dn11 * locals.var_t1) + (locals.var_p1m * locals.var_t1_dn11)) + (p.p12 * locals.var_t2_dn11)) + (((p.p13 * locals.var_t1_dn11) * locals.var_t2) + (assign550_e875 * locals.var_t2_dn11)));

        let assign560_e881: f64 = (locals.var_psi).tanh();
        let assign560_e882: f64 = (1.0 + assign560_e881);
        locals.var_tanh_psi = assign560_e882;
        locals.var_tanh_psi_dn3 = (locals.var_psi_dn3 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn4 = (locals.var_psi_dn4 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn5 = (locals.var_psi_dn5 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn8 = (locals.var_psi_dn8 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn11 = (locals.var_psi_dn11 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));

        let assign570_e886: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign570_e888: f64 = (-locals.var_psi);
        let assign570_e889: f64 = { let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign570_e890: f64 = (assign570_e886 - assign570_e889);
        let assign570_e891: f64 = (0.5 * assign570_e890);
        let assign570_e892: f64 = (assign570_e891).tanh();
        let assign570_e893: f64 = (1.0 + assign570_e892);
        locals.var_tanh_psi1 = assign570_e893;
        locals.var_tanh_psi1_dn3 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        locals.var_tanh_psi1_dn4 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        locals.var_tanh_psi1_dn5 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        locals.var_tanh_psi1_dn8 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));
        locals.var_tanh_psi1_dn11 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn11) - ({ let limexp_arg = assign570_e888; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn11)))) / ((assign570_e891).cosh() * (assign570_e891).cosh()));

        let assign600_e905: f64 = if p.p4 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign600_e905;

        let assign610_e908: f64 = if p.p4 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign610_e908;

        let assign620_e911: f64 = if p.p4 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign620_e911;

        let assign630_e914: f64 = if p.p4 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign630_e914;

        let (assign650_e944, assign650_e944_d_n3, assign650_e944_d_n4, assign650_e944_d_n5, assign650_e944_d_n8, assign650_e944_d_n11,) = {
    if ((locals.var_guard6 != 0.0) && (locals.var_guard5 == 0.0)) {
        let assign650_e942: f64 = (locals.var_vgd - locals.var_vpkm);
        (assign650_e942, (locals.var_vgd_dn3 - locals.var_vpkm_dn3), (locals.var_vgd_dn4 - locals.var_vpkm_dn4), (-locals.var_vpkm_dn5), 0.0, (-locals.var_vpkm_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign650_e944;
        locals.var_t0_dn3 = assign650_e944_d_n3;
        locals.var_t0_dn4 = assign650_e944_d_n4;
        locals.var_t0_dn5 = assign650_e944_d_n5;
        locals.var_t0_dn8 = assign650_e944_d_n8;
        locals.var_t0_dn11 = assign650_e944_d_n11;

        let (assign660_e953, assign660_e953_d_n3, assign660_e953_d_n4, assign660_e953_d_n5, assign660_e953_d_n8, assign660_e953_d_n11,) = {
    if ((locals.var_guard6 != 0.0) && (locals.var_guard5 == 0.0)) {
        let assign660_e951: f64 = (locals.var_t0 * locals.var_t0);
        (assign660_e951, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign660_e953;
        locals.var_t1_dn3 = assign660_e953_d_n3;
        locals.var_t1_dn4 = assign660_e953_d_n4;
        locals.var_t1_dn5 = assign660_e953_d_n5;
        locals.var_t1_dn8 = assign660_e953_d_n8;
        locals.var_t1_dn11 = assign660_e953_d_n11;

        let (assign670_e962, assign670_e962_d_n3, assign670_e962_d_n4, assign670_e962_d_n5, assign670_e962_d_n8, assign670_e962_d_n11,) = {
    if ((locals.var_guard6 != 0.0) && (locals.var_guard5 == 0.0)) {
        let assign670_e960: f64 = (locals.var_t1 * locals.var_t0);
        (assign670_e960, ((locals.var_t1_dn3 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn3)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn11 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn11)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn8, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign670_e962;
        locals.var_t2_dn3 = assign670_e962_d_n3;
        locals.var_t2_dn4 = assign670_e962_d_n4;
        locals.var_t2_dn5 = assign670_e962_d_n5;
        locals.var_t2_dn8 = assign670_e962_d_n8;
        locals.var_t2_dn11 = assign670_e962_d_n11;

        let (assign770_e1099, assign770_e1099_d_n3, assign770_e1099_d_n4, assign770_e1099_d_n5, assign770_e1099_d_n8, assign770_e1099_d_n11,) = {
    if ((locals.var_guard7 != 0.0) && (!((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)))) {
        let assign770_e1097: f64 = (locals.var_vgs - locals.var_vpkm);
        (assign770_e1097, (-locals.var_vpkm_dn3), (-locals.var_vpkm_dn4), (locals.var_vgs_dn5 - locals.var_vpkm_dn5), locals.var_vgs_dn8, (-locals.var_vpkm_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign770_e1099;
        locals.var_t0_dn3 = assign770_e1099_d_n3;
        locals.var_t0_dn4 = assign770_e1099_d_n4;
        locals.var_t0_dn5 = assign770_e1099_d_n5;
        locals.var_t0_dn8 = assign770_e1099_d_n8;
        locals.var_t0_dn11 = assign770_e1099_d_n11;

        let (assign780_e1110, assign780_e1110_d_n3, assign780_e1110_d_n4, assign780_e1110_d_n5, assign780_e1110_d_n8, assign780_e1110_d_n11,) = {
    if ((locals.var_guard7 != 0.0) && (!((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)))) {
        let assign780_e1108: f64 = (locals.var_t0 * locals.var_t0);
        (assign780_e1108, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign780_e1110;
        locals.var_t1_dn3 = assign780_e1110_d_n3;
        locals.var_t1_dn4 = assign780_e1110_d_n4;
        locals.var_t1_dn5 = assign780_e1110_d_n5;
        locals.var_t1_dn8 = assign780_e1110_d_n8;
        locals.var_t1_dn11 = assign780_e1110_d_n11;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign790_e1131, assign790_e1131_d_n3, assign790_e1131_d_n4, assign790_e1131_d_n5, assign790_e1131_d_n8, assign790_e1131_d_n11,) = {
    if ((locals.var_guard7 != 0.0) && (!((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)))) {
        let assign790_e1121: f64 = (p.p12 * locals.var_t1);
        let assign790_e1122: f64 = (locals.var_t0 + assign790_e1121);
        let assign790_e1125: f64 = (p.p13 * locals.var_t1);
        let assign790_e1127: f64 = (assign790_e1125 * locals.var_t0);
        let assign790_e1128: f64 = (assign790_e1122 + assign790_e1127);
        let assign790_e1129: f64 = (locals.var_p1m * assign790_e1128);
        (assign790_e1129, ((locals.var_p1m_dn3 * assign790_e1128) + (locals.var_p1m * ((locals.var_t0_dn3 + (p.p12 * locals.var_t1_dn3)) + (((p.p13 * locals.var_t1_dn3) * locals.var_t0) + (assign790_e1125 * locals.var_t0_dn3))))), ((locals.var_p1m_dn4 * assign790_e1128) + (locals.var_p1m * ((locals.var_t0_dn4 + (p.p12 * locals.var_t1_dn4)) + (((p.p13 * locals.var_t1_dn4) * locals.var_t0) + (assign790_e1125 * locals.var_t0_dn4))))), ((locals.var_p1m_dn5 * assign790_e1128) + (locals.var_p1m * ((locals.var_t0_dn5 + (p.p12 * locals.var_t1_dn5)) + (((p.p13 * locals.var_t1_dn5) * locals.var_t0) + (assign790_e1125 * locals.var_t0_dn5))))), ((locals.var_p1m_dn8 * assign790_e1128) + (locals.var_p1m * ((locals.var_t0_dn8 + (p.p12 * locals.var_t1_dn8)) + (((p.p13 * locals.var_t1_dn8) * locals.var_t0) + (assign790_e1125 * locals.var_t0_dn8))))), ((locals.var_p1m_dn11 * assign790_e1128) + (locals.var_p1m * ((locals.var_t0_dn11 + (p.p12 * locals.var_t1_dn11)) + (((p.p13 * locals.var_t1_dn11) * locals.var_t0) + (assign790_e1125 * locals.var_t0_dn11))))),)
    } else {
        (locals.var_psi, locals.var_psi_dn3, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn8, locals.var_psi_dn11,)
    }
};
        locals.var_psi = assign790_e1131;
        locals.var_psi_dn3 = assign790_e1131_d_n3;
        locals.var_psi_dn4 = assign790_e1131_d_n4;
        locals.var_psi_dn5 = assign790_e1131_d_n5;
        locals.var_psi_dn8 = assign790_e1131_d_n8;
        locals.var_psi_dn11 = assign790_e1131_d_n11;

        let (assign800_e1150, assign800_e1150_d_n3, assign800_e1150_d_n4, assign800_e1150_d_n5, assign800_e1150_d_n8, assign800_e1150_d_n11,) = {
    if ((locals.var_guard7 != 0.0) && (!((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)))) {
        let assign800_e1141: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign800_e1143: f64 = (-locals.var_psi);
        let assign800_e1144: f64 = { let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign800_e1145: f64 = (assign800_e1141 - assign800_e1144);
        let assign800_e1146: f64 = (0.5 * assign800_e1145);
        let assign800_e1147: f64 = (assign800_e1146).tanh();
        let assign800_e1148: f64 = (1.0 + assign800_e1147);
        (assign800_e1148, ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn11) - ({ let limexp_arg = assign800_e1143; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn11)))) / ((assign800_e1146).cosh() * (assign800_e1146).cosh())),)
    } else {
        (locals.var_tanh_psi1, locals.var_tanh_psi1_dn3, locals.var_tanh_psi1_dn4, locals.var_tanh_psi1_dn5, locals.var_tanh_psi1_dn8, locals.var_tanh_psi1_dn11,)
    }
};
        locals.var_tanh_psi1 = assign800_e1150;
        locals.var_tanh_psi1_dn3 = assign800_e1150_d_n3;
        locals.var_tanh_psi1_dn4 = assign800_e1150_d_n4;
        locals.var_tanh_psi1_dn5 = assign800_e1150_d_n5;
        locals.var_tanh_psi1_dn8 = assign800_e1150_d_n8;
        locals.var_tanh_psi1_dn11 = assign800_e1150_d_n11;

        let (assign850_e1227, assign850_e1227_d_n3, assign850_e1227_d_n4, assign850_e1227_d_n5, assign850_e1227_d_n8, assign850_e1227_d_n11,) = {
    if ((locals.var_guard8 != 0.0) && (!(((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)) || (locals.var_guard7 != 0.0)))) {
        let assign850_e1225: f64 = (locals.var_vgs - locals.var_vpkm);
        (assign850_e1225, (-locals.var_vpkm_dn3), (-locals.var_vpkm_dn4), (locals.var_vgs_dn5 - locals.var_vpkm_dn5), locals.var_vgs_dn8, (-locals.var_vpkm_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign850_e1227;
        locals.var_t0_dn3 = assign850_e1227_d_n3;
        locals.var_t0_dn4 = assign850_e1227_d_n4;
        locals.var_t0_dn5 = assign850_e1227_d_n5;
        locals.var_t0_dn8 = assign850_e1227_d_n8;
        locals.var_t0_dn11 = assign850_e1227_d_n11;

        let (assign860_e1240, assign860_e1240_d_n3, assign860_e1240_d_n4, assign860_e1240_d_n5, assign860_e1240_d_n8, assign860_e1240_d_n11,) = {
    if ((locals.var_guard8 != 0.0) && (!(((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)) || (locals.var_guard7 != 0.0)))) {
        let assign860_e1238: f64 = (locals.var_t0 * locals.var_t0);
        (assign860_e1238, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign860_e1240;
        locals.var_t1_dn3 = assign860_e1240_d_n3;
        locals.var_t1_dn4 = assign860_e1240_d_n4;
        locals.var_t1_dn5 = assign860_e1240_d_n5;
        locals.var_t1_dn8 = assign860_e1240_d_n8;
        locals.var_t1_dn11 = assign860_e1240_d_n11;

        let (assign870_e1263, assign870_e1263_d_n3, assign870_e1263_d_n4, assign870_e1263_d_n5, assign870_e1263_d_n8, assign870_e1263_d_n11,) = {
    if ((locals.var_guard8 != 0.0) && (!(((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)) || (locals.var_guard7 != 0.0)))) {
        let assign870_e1253: f64 = (p.p12 * locals.var_t1);
        let assign870_e1254: f64 = (locals.var_t0 + assign870_e1253);
        let assign870_e1257: f64 = (p.p13 * locals.var_t1);
        let assign870_e1259: f64 = (assign870_e1257 * locals.var_t0);
        let assign870_e1260: f64 = (assign870_e1254 + assign870_e1259);
        let assign870_e1261: f64 = (locals.var_p1m * assign870_e1260);
        (assign870_e1261, ((locals.var_p1m_dn3 * assign870_e1260) + (locals.var_p1m * ((locals.var_t0_dn3 + (p.p12 * locals.var_t1_dn3)) + (((p.p13 * locals.var_t1_dn3) * locals.var_t0) + (assign870_e1257 * locals.var_t0_dn3))))), ((locals.var_p1m_dn4 * assign870_e1260) + (locals.var_p1m * ((locals.var_t0_dn4 + (p.p12 * locals.var_t1_dn4)) + (((p.p13 * locals.var_t1_dn4) * locals.var_t0) + (assign870_e1257 * locals.var_t0_dn4))))), ((locals.var_p1m_dn5 * assign870_e1260) + (locals.var_p1m * ((locals.var_t0_dn5 + (p.p12 * locals.var_t1_dn5)) + (((p.p13 * locals.var_t1_dn5) * locals.var_t0) + (assign870_e1257 * locals.var_t0_dn5))))), ((locals.var_p1m_dn8 * assign870_e1260) + (locals.var_p1m * ((locals.var_t0_dn8 + (p.p12 * locals.var_t1_dn8)) + (((p.p13 * locals.var_t1_dn8) * locals.var_t0) + (assign870_e1257 * locals.var_t0_dn8))))), ((locals.var_p1m_dn11 * assign870_e1260) + (locals.var_p1m * ((locals.var_t0_dn11 + (p.p12 * locals.var_t1_dn11)) + (((p.p13 * locals.var_t1_dn11) * locals.var_t0) + (assign870_e1257 * locals.var_t0_dn11))))),)
    } else {
        (locals.var_psi, locals.var_psi_dn3, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn8, locals.var_psi_dn11,)
    }
};
        locals.var_psi = assign870_e1263;
        locals.var_psi_dn3 = assign870_e1263_d_n3;
        locals.var_psi_dn4 = assign870_e1263_d_n4;
        locals.var_psi_dn5 = assign870_e1263_d_n5;
        locals.var_psi_dn8 = assign870_e1263_d_n8;
        locals.var_psi_dn11 = assign870_e1263_d_n11;

        let (assign880_e1276, assign880_e1276_d_n3, assign880_e1276_d_n4, assign880_e1276_d_n5, assign880_e1276_d_n8, assign880_e1276_d_n11,) = {
    if ((locals.var_guard8 != 0.0) && (!(((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)) || (locals.var_guard7 != 0.0)))) {
        let assign880_e1274: f64 = (locals.var_vgd - locals.var_vpkm);
        (assign880_e1274, (locals.var_vgd_dn3 - locals.var_vpkm_dn3), (locals.var_vgd_dn4 - locals.var_vpkm_dn4), (-locals.var_vpkm_dn5), 0.0, (-locals.var_vpkm_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn8, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign880_e1276;
        locals.var_t2_dn3 = assign880_e1276_d_n3;
        locals.var_t2_dn4 = assign880_e1276_d_n4;
        locals.var_t2_dn5 = assign880_e1276_d_n5;
        locals.var_t2_dn8 = assign880_e1276_d_n8;
        locals.var_t2_dn11 = assign880_e1276_d_n11;

        let (assign910_e1333, assign910_e1333_d_n3, assign910_e1333_d_n4, assign910_e1333_d_n5, assign910_e1333_d_n8, assign910_e1333_d_n11,) = {
    if ((locals.var_guard8 != 0.0) && (!(((locals.var_guard5 != 0.0) || (locals.var_guard6 != 0.0)) || (locals.var_guard7 != 0.0)))) {
        let assign910_e1324: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign910_e1326: f64 = (-locals.var_psi);
        let assign910_e1327: f64 = { let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign910_e1328: f64 = (assign910_e1324 - assign910_e1327);
        let assign910_e1329: f64 = (0.5 * assign910_e1328);
        let assign910_e1330: f64 = (assign910_e1329).tanh();
        let assign910_e1331: f64 = (1.0 + assign910_e1330);
        (assign910_e1331, ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn11) - ({ let limexp_arg = assign910_e1326; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn11)))) / ((assign910_e1329).cosh() * (assign910_e1329).cosh())),)
    } else {
        (locals.var_tanh_psi1, locals.var_tanh_psi1_dn3, locals.var_tanh_psi1_dn4, locals.var_tanh_psi1_dn5, locals.var_tanh_psi1_dn8, locals.var_tanh_psi1_dn11,)
    }
};
        locals.var_tanh_psi1 = assign910_e1333;
        locals.var_tanh_psi1_dn3 = assign910_e1333_d_n3;
        locals.var_tanh_psi1_dn4 = assign910_e1333_d_n4;
        locals.var_tanh_psi1_dn5 = assign910_e1333_d_n5;
        locals.var_tanh_psi1_dn8 = assign910_e1333_d_n8;
        locals.var_tanh_psi1_dn11 = assign910_e1333_d_n11;

        let assign1020_e1517: f64 = if ((p.p4 == 0.0) || (p.p4 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard9 = assign1020_e1517;

        let (assign1040_e1535, assign1040_e1535_d_n3, assign1040_e1535_d_n4, assign1040_e1535_d_n5, assign1040_e1535_d_n8, assign1040_e1535_d_n11,) = {
    if (locals.var_guard9 != 0.0) {
        let assign1040_e1532: f64 = (p.p44 * locals.var_tanh_psi);
        let assign1040_e1533: f64 = (p.p43 + assign1040_e1532);
        (assign1040_e1533, (p.p44 * locals.var_tanh_psi_dn3), (p.p44 * locals.var_tanh_psi_dn4), (p.p44 * locals.var_tanh_psi_dn5), (p.p44 * locals.var_tanh_psi_dn8), (p.p44 * locals.var_tanh_psi_dn11),)
    } else {
        (locals.var_rd1, locals.var_rd1_dn3, locals.var_rd1_dn4, locals.var_rd1_dn5, locals.var_rd1_dn8, locals.var_rd1_dn11,)
    }
};
        locals.var_rd1 = assign1040_e1535;
        locals.var_rd1_dn3 = assign1040_e1535_d_n3;
        locals.var_rd1_dn4 = assign1040_e1535_d_n4;
        locals.var_rd1_dn5 = assign1040_e1535_d_n5;
        locals.var_rd1_dn8 = assign1040_e1535_d_n8;
        locals.var_rd1_dn11 = assign1040_e1535_d_n11;

        let (assign1050_e1543, assign1050_e1543_d_n3, assign1050_e1543_d_n4, assign1050_e1543_d_n5, assign1050_e1543_d_n8, assign1050_e1543_d_n11,) = {
    if (locals.var_guard9 != 0.0) {
        let assign1050_e1540: f64 = (p.p44 * locals.var_tanh_psi);
        let assign1050_e1541: f64 = (p.p46 + assign1050_e1540);
        (assign1050_e1541, (p.p44 * locals.var_tanh_psi_dn3), (p.p44 * locals.var_tanh_psi_dn4), (p.p44 * locals.var_tanh_psi_dn5), (p.p44 * locals.var_tanh_psi_dn8), (p.p44 * locals.var_tanh_psi_dn11),)
    } else {
        (locals.var_rs1, locals.var_rs1_dn3, locals.var_rs1_dn4, locals.var_rs1_dn5, locals.var_rs1_dn8, locals.var_rs1_dn11,)
    }
};
        locals.var_rs1 = assign1050_e1543;
        locals.var_rs1_dn3 = assign1050_e1543_d_n3;
        locals.var_rs1_dn4 = assign1050_e1543_d_n4;
        locals.var_rs1_dn5 = assign1050_e1543_d_n5;
        locals.var_rs1_dn8 = assign1050_e1543_d_n8;
        locals.var_rs1_dn11 = assign1050_e1543_d_n11;

        let (assign1070_e1563, assign1070_e1563_d_n3, assign1070_e1563_d_n4, assign1070_e1563_d_n5, assign1070_e1563_d_n8, assign1070_e1563_d_n11,) = {
    if (locals.var_guard9 == 0.0) {
        let assign1070_e1560: f64 = (p.p44 * locals.var_tanh_psi1);
        let assign1070_e1561: f64 = (p.p43 + assign1070_e1560);
        (assign1070_e1561, (p.p44 * locals.var_tanh_psi1_dn3), (p.p44 * locals.var_tanh_psi1_dn4), (p.p44 * locals.var_tanh_psi1_dn5), (p.p44 * locals.var_tanh_psi1_dn8), (p.p44 * locals.var_tanh_psi1_dn11),)
    } else {
        (locals.var_rd1, locals.var_rd1_dn3, locals.var_rd1_dn4, locals.var_rd1_dn5, locals.var_rd1_dn8, locals.var_rd1_dn11,)
    }
};
        locals.var_rd1 = assign1070_e1563;
        locals.var_rd1_dn3 = assign1070_e1563_d_n3;
        locals.var_rd1_dn4 = assign1070_e1563_d_n4;
        locals.var_rd1_dn5 = assign1070_e1563_d_n5;
        locals.var_rd1_dn8 = assign1070_e1563_d_n8;
        locals.var_rd1_dn11 = assign1070_e1563_d_n11;

        let (assign1080_e1572, assign1080_e1572_d_n3, assign1080_e1572_d_n4, assign1080_e1572_d_n5, assign1080_e1572_d_n8, assign1080_e1572_d_n11,) = {
    if (locals.var_guard9 == 0.0) {
        let assign1080_e1569: f64 = (p.p44 * locals.var_tanh_psi1);
        let assign1080_e1570: f64 = (p.p46 + assign1080_e1569);
        (assign1080_e1570, (p.p44 * locals.var_tanh_psi1_dn3), (p.p44 * locals.var_tanh_psi1_dn4), (p.p44 * locals.var_tanh_psi1_dn5), (p.p44 * locals.var_tanh_psi1_dn8), (p.p44 * locals.var_tanh_psi1_dn11),)
    } else {
        (locals.var_rs1, locals.var_rs1_dn3, locals.var_rs1_dn4, locals.var_rs1_dn5, locals.var_rs1_dn8, locals.var_rs1_dn11,)
    }
};
        locals.var_rs1 = assign1080_e1572;
        locals.var_rs1_dn3 = assign1080_e1572_d_n3;
        locals.var_rs1_dn4 = assign1080_e1572_d_n4;
        locals.var_rs1_dn5 = assign1080_e1572_d_n5;
        locals.var_rs1_dn8 = assign1080_e1572_d_n8;
        locals.var_rs1_dn11 = assign1080_e1572_d_n11;

        let assign1090_e1577: f64 = if ((locals.var_delta_t != 0.0) || (p.p57 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard10 = assign1090_e1577;

        let (assign1100_e1587, assign1100_e1587_d_n3, assign1100_e1587_d_n4, assign1100_e1587_d_n5, assign1100_e1587_d_n8, assign1100_e1587_d_n11,) = {
    if (locals.var_guard10 != 0.0) {
        let assign1100_e1583: f64 = (p.p66 * locals.var_delta_t);
        let assign1100_e1584: f64 = (1.0 + assign1100_e1583);
        let assign1100_e1585: f64 = (locals.var_rs1 * assign1100_e1584);
        (assign1100_e1585, (locals.var_rs1_dn3 * assign1100_e1584), (locals.var_rs1_dn4 * assign1100_e1584), (locals.var_rs1_dn5 * assign1100_e1584), (locals.var_rs1_dn8 * assign1100_e1584), ((locals.var_rs1_dn11 * assign1100_e1584) + (locals.var_rs1 * (p.p66 * locals.var_delta_t_dn11))),)
    } else {
        (locals.var_rs_t, locals.var_rs_t_dn3, locals.var_rs_t_dn4, locals.var_rs_t_dn5, locals.var_rs_t_dn8, locals.var_rs_t_dn11,)
    }
};
        locals.var_rs_t = assign1100_e1587;
        locals.var_rs_t_dn3 = assign1100_e1587_d_n3;
        locals.var_rs_t_dn4 = assign1100_e1587_d_n4;
        locals.var_rs_t_dn5 = assign1100_e1587_d_n5;
        locals.var_rs_t_dn8 = assign1100_e1587_d_n8;
        locals.var_rs_t_dn11 = assign1100_e1587_d_n11;

        let (assign1110_e1597, assign1110_e1597_d_n3, assign1110_e1597_d_n4, assign1110_e1597_d_n5, assign1110_e1597_d_n8, assign1110_e1597_d_n11,) = {
    if (locals.var_guard10 != 0.0) {
        let assign1110_e1593: f64 = (p.p66 * locals.var_delta_t);
        let assign1110_e1594: f64 = (1.0 + assign1110_e1593);
        let assign1110_e1595: f64 = (locals.var_rd1 * assign1110_e1594);
        (assign1110_e1595, (locals.var_rd1_dn3 * assign1110_e1594), (locals.var_rd1_dn4 * assign1110_e1594), (locals.var_rd1_dn5 * assign1110_e1594), (locals.var_rd1_dn8 * assign1110_e1594), ((locals.var_rd1_dn11 * assign1110_e1594) + (locals.var_rd1 * (p.p66 * locals.var_delta_t_dn11))),)
    } else {
        (locals.var_rd1_t, locals.var_rd1_t_dn3, locals.var_rd1_t_dn4, locals.var_rd1_t_dn5, locals.var_rd1_t_dn8, locals.var_rd1_t_dn11,)
    }
};
        locals.var_rd1_t = assign1110_e1597;
        locals.var_rd1_t_dn3 = assign1110_e1597_d_n3;
        locals.var_rd1_t_dn4 = assign1110_e1597_d_n4;
        locals.var_rd1_t_dn5 = assign1110_e1597_d_n5;
        locals.var_rd1_t_dn8 = assign1110_e1597_d_n8;
        locals.var_rd1_t_dn11 = assign1110_e1597_d_n11;

        let (assign1130_e1612, assign1130_e1612_d_n3, assign1130_e1612_d_n4, assign1130_e1612_d_n5, assign1130_e1612_d_n8, assign1130_e1612_d_n11,) = {
    if (locals.var_guard10 == 0.0) {
        (locals.var_rd1, locals.var_rd1_dn3, locals.var_rd1_dn4, locals.var_rd1_dn5, locals.var_rd1_dn8, locals.var_rd1_dn11,)
    } else {
        (locals.var_rd1_t, locals.var_rd1_t_dn3, locals.var_rd1_t_dn4, locals.var_rd1_t_dn5, locals.var_rd1_t_dn8, locals.var_rd1_t_dn11,)
    }
};
        locals.var_rd1_t = assign1130_e1612;
        locals.var_rd1_t_dn3 = assign1130_e1612_d_n3;
        locals.var_rd1_t_dn4 = assign1130_e1612_d_n4;
        locals.var_rd1_t_dn5 = assign1130_e1612_d_n5;
        locals.var_rd1_t_dn8 = assign1130_e1612_d_n8;
        locals.var_rd1_t_dn11 = assign1130_e1612_d_n11;

        let (assign1140_e1617, assign1140_e1617_d_n3, assign1140_e1617_d_n4, assign1140_e1617_d_n5, assign1140_e1617_d_n8, assign1140_e1617_d_n11,) = {
    if (locals.var_guard10 == 0.0) {
        (locals.var_rs1, locals.var_rs1_dn3, locals.var_rs1_dn4, locals.var_rs1_dn5, locals.var_rs1_dn8, locals.var_rs1_dn11,)
    } else {
        (locals.var_rs_t, locals.var_rs_t_dn3, locals.var_rs_t_dn4, locals.var_rs_t_dn5, locals.var_rs_t_dn8, locals.var_rs_t_dn11,)
    }
};
        locals.var_rs_t = assign1140_e1617;
        locals.var_rs_t_dn3 = assign1140_e1617_d_n3;
        locals.var_rs_t_dn4 = assign1140_e1617_d_n4;
        locals.var_rs_t_dn5 = assign1140_e1617_d_n5;
        locals.var_rs_t_dn8 = assign1140_e1617_d_n8;
        locals.var_rs_t_dn11 = assign1140_e1617_d_n11;

        let assign1160_e1625: f64 = if p.p5 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign1160_e1625;

        let (assign1170_e1636, assign1170_e1636_d_n3, assign1170_e1636_d_n4, assign1170_e1636_d_n5, assign1170_e1636_d_n8, assign1170_e1636_d_n11,) = {
    if (locals.var_guard11 != 0.0) {
        let assign1170_e1629: f64 = (-1.0);
        let assign1170_e1631: f64 = (assign1170_e1629 * locals.var_vjg_t);
        let assign1170_e1632: f64 = (assign1170_e1631).tanh();
        let assign1170_e1633: f64 = (locals.var_pg_param * assign1170_e1632);
        let assign1170_e1634: f64 = { let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1170_e1634, 0.0, 0.0, 0.0, 0.0, ({ let limexp_arg = assign1170_e1633; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((locals.var_pg_param_dn11 * assign1170_e1632) + (locals.var_pg_param * ((assign1170_e1629 * locals.var_vjg_t_dn11) / ((assign1170_e1631).cosh() * (assign1170_e1631).cosh()))))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign1170_e1636;
        locals.var_t0_dn3 = assign1170_e1636_d_n3;
        locals.var_t0_dn4 = assign1170_e1636_d_n4;
        locals.var_t0_dn5 = assign1170_e1636_d_n5;
        locals.var_t0_dn8 = assign1170_e1636_d_n8;
        locals.var_t0_dn11 = assign1170_e1636_d_n11;

        let (assign1200_e1661, assign1200_e1661_d_n3, assign1200_e1661_d_n4, assign1200_e1661_d_n5, assign1200_e1661_d_n8, assign1200_e1661_d_n11,) = {
    if (locals.var_guard11 == 0.0) {
        let assign1200_e1656: f64 = (-locals.var_pg_param);
        let assign1200_e1658: f64 = (assign1200_e1656 * locals.var_vjg_t);
        let assign1200_e1659: f64 = { let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1200_e1659, 0.0, 0.0, 0.0, 0.0, ({ let limexp_arg = assign1200_e1658; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-locals.var_pg_param_dn11) * locals.var_vjg_t) + (assign1200_e1656 * locals.var_vjg_t_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign1200_e1661;
        locals.var_t0_dn3 = assign1200_e1661_d_n3;
        locals.var_t0_dn4 = assign1200_e1661_d_n4;
        locals.var_t0_dn5 = assign1200_e1661_d_n5;
        locals.var_t0_dn8 = assign1200_e1661_d_n8;
        locals.var_t0_dn11 = assign1200_e1661_d_n11;

        let assign1280_e1724: f64 = (p.p30 * locals.var_vgsc);
        let assign1280_e1725: f64 = (locals.var_p10_t + assign1280_e1724);
        let assign1280_e1728: f64 = (p.p37 * locals.var_vds);
        let assign1280_e1729: f64 = (assign1280_e1725 + assign1280_e1728);
        locals.var_psi_1 = assign1280_e1729;
        locals.var_psi_1_dn3 = (p.p37 * locals.var_vds_dn3);
        locals.var_psi_1_dn5 = ((p.p30 * locals.var_vgsc_dn5) + (p.p37 * locals.var_vds_dn5));
        locals.var_psi_1_dn8 = (p.p30 * locals.var_vgsc_dn8);
        locals.var_psi_1_dn11 = locals.var_p10_t_dn11;

        let assign1290_e1732: f64 = (locals.var_psi_1).tanh();
        let assign1290_e1733: f64 = (1.0 + assign1290_e1732);
        locals.var_tanh1 = assign1290_e1733;
        locals.var_tanh1_dn3 = (locals.var_psi_1_dn3 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn5 = (locals.var_psi_1_dn5 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn8 = (locals.var_psi_1_dn8 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn11 = (locals.var_psi_1_dn11 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));

        let assign1300_e1737: f64 = (p.p32 * locals.var_vds);
        let assign1300_e1738: f64 = (p.p31 + assign1300_e1737);
        locals.var_psi_2 = assign1300_e1738;
        locals.var_psi_2_dn3 = (p.p32 * locals.var_vds_dn3);
        locals.var_psi_2_dn5 = (p.p32 * locals.var_vds_dn5);

        let assign1310_e1741: f64 = (locals.var_psi_2).tanh();
        let assign1310_e1742: f64 = (1.0 + assign1310_e1741);
        locals.var_tanh2 = assign1310_e1742;
        locals.var_tanh2_dn3 = (locals.var_psi_2_dn3 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh()));
        locals.var_tanh2_dn5 = (locals.var_psi_2_dn5 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh()));

        let assign1320_e1746: f64 = (p.p34 * locals.var_vds);
        let assign1320_e1747: f64 = (p.p33 - assign1320_e1746);
        locals.var_psi_3 = assign1320_e1747;
        locals.var_psi_3_dn3 = (-(p.p34 * locals.var_vds_dn3));
        locals.var_psi_3_dn5 = (-(p.p34 * locals.var_vds_dn5));

        let assign1330_e1750: f64 = (locals.var_psi_3).tanh();
        let assign1330_e1751: f64 = (1.0 + assign1330_e1750);
        let assign1330_e1753: f64 = (assign1330_e1751 - p.p37);
        locals.var_tanh3 = assign1330_e1753;
        locals.var_tanh3_dn3 = (locals.var_psi_3_dn3 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh()));
        locals.var_tanh3_dn5 = (locals.var_psi_3_dn5 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh()));

        let assign1340_e1757: f64 = (p.p36 * locals.var_vgdc);
        let assign1340_e1758: f64 = (locals.var_p40_t + assign1340_e1757);
        let assign1340_e1761: f64 = (p.p37 * locals.var_vds);
        let assign1340_e1762: f64 = (assign1340_e1758 - assign1340_e1761);
        locals.var_psi_4 = assign1340_e1762;
        locals.var_psi_4_dn3 = ((p.p36 * locals.var_vgdc_dn3) - (p.p37 * locals.var_vds_dn3));
        locals.var_psi_4_dn5 = (-(p.p37 * locals.var_vds_dn5));
        locals.var_psi_4_dn7 = (p.p36 * locals.var_vgdc_dn7);
        locals.var_psi_4_dn11 = locals.var_p40_t_dn11;

        let assign1350_e1765: f64 = (locals.var_psi_4).tanh();
        let assign1350_e1766: f64 = (1.0 + assign1350_e1765);
        locals.var_tanh4 = assign1350_e1766;
        locals.var_tanh4_dn3 = (locals.var_psi_4_dn3 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn5 = (locals.var_psi_4_dn5 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn7 = (locals.var_psi_4_dn7 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn11 = (locals.var_psi_4_dn11 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));

        let assign1360_e1769: f64 = if p.p6 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign1360_e1769;

        let assign1370_e1772: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1370_e1772;

        let assign1380_e1775: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1380_e1775;

        let (assign1390_e1779, assign1390_e1779_d_n3, assign1390_e1779_d_n5, assign1390_e1779_d_n7, assign1390_e1779_d_n8, assign1390_e1779_d_n11,) = {
    if (locals.var_guard13 != 0.0) {
        (p.p24, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn7, locals.var_cgs_dn8, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1390_e1779;
        locals.var_cgs_dn3 = assign1390_e1779_d_n3;
        locals.var_cgs_dn5 = assign1390_e1779_d_n5;
        locals.var_cgs_dn7 = assign1390_e1779_d_n7;
        locals.var_cgs_dn8 = assign1390_e1779_d_n8;
        locals.var_cgs_dn11 = assign1390_e1779_d_n11;

        let (assign1400_e1783, assign1400_e1783_d_n3, assign1400_e1783_d_n5, assign1400_e1783_d_n7, assign1400_e1783_d_n8, assign1400_e1783_d_n11,) = {
    if (locals.var_guard13 != 0.0) {
        (p.p26, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn7, locals.var_cgd_dn8, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1400_e1783;
        locals.var_cgd_dn3 = assign1400_e1783_d_n3;
        locals.var_cgd_dn5 = assign1400_e1783_d_n5;
        locals.var_cgd_dn7 = assign1400_e1783_d_n7;
        locals.var_cgd_dn8 = assign1400_e1783_d_n8;
        locals.var_cgd_dn11 = assign1400_e1783_d_n11;

        let (assign1410_e1796, assign1410_e1796_d_n3, assign1410_e1796_d_n5, assign1410_e1796_d_n7, assign1410_e1796_d_n8, assign1410_e1796_d_n11,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign1410_e1791: f64 = (locals.var_cgs0_t * locals.var_tanh1);
        let assign1410_e1793: f64 = (assign1410_e1791 * locals.var_tanh2);
        let assign1410_e1794: f64 = (p.p24 + assign1410_e1793);
        (assign1410_e1794, (((locals.var_cgs0_t * locals.var_tanh1_dn3) * locals.var_tanh2) + (assign1410_e1791 * locals.var_tanh2_dn3)), (((locals.var_cgs0_t * locals.var_tanh1_dn5) * locals.var_tanh2) + (assign1410_e1791 * locals.var_tanh2_dn5)), 0.0, ((locals.var_cgs0_t * locals.var_tanh1_dn8) * locals.var_tanh2), (((locals.var_cgs0_t_dn11 * locals.var_tanh1) + (locals.var_cgs0_t * locals.var_tanh1_dn11)) * locals.var_tanh2),)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn7, locals.var_cgs_dn8, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1410_e1796;
        locals.var_cgs_dn3 = assign1410_e1796_d_n3;
        locals.var_cgs_dn5 = assign1410_e1796_d_n5;
        locals.var_cgs_dn7 = assign1410_e1796_d_n7;
        locals.var_cgs_dn8 = assign1410_e1796_d_n8;
        locals.var_cgs_dn11 = assign1410_e1796_d_n11;

        let (assign1420_e1813, assign1420_e1813_d_n3, assign1420_e1813_d_n5, assign1420_e1813_d_n7, assign1420_e1813_d_n8, assign1420_e1813_d_n11,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign1420_e1805: f64 = (locals.var_tanh3 * locals.var_tanh4);
        let assign1420_e1808: f64 = (2.0 * p.p37);
        let assign1420_e1809: f64 = (assign1420_e1805 + assign1420_e1808);
        let assign1420_e1810: f64 = (locals.var_cgd0_t * assign1420_e1809);
        let assign1420_e1811: f64 = (p.p26 + assign1420_e1810);
        (assign1420_e1811, (locals.var_cgd0_t * ((locals.var_tanh3_dn3 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn3))), (locals.var_cgd0_t * ((locals.var_tanh3_dn5 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn5))), (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn7)), 0.0, ((locals.var_cgd0_t_dn11 * assign1420_e1809) + (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn11))),)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn7, locals.var_cgd_dn8, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1420_e1813;
        locals.var_cgd_dn3 = assign1420_e1813_d_n3;
        locals.var_cgd_dn5 = assign1420_e1813_d_n5;
        locals.var_cgd_dn7 = assign1420_e1813_d_n7;
        locals.var_cgd_dn8 = assign1420_e1813_d_n8;
        locals.var_cgd_dn11 = assign1420_e1813_d_n11;

        let (assign1430_e1824, assign1430_e1824_d_n3, assign1430_e1824_d_n5,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1430_e1822: f64 = (locals.var_tanh2 - p.p37);
        (assign1430_e1822, locals.var_tanh2_dn3, locals.var_tanh2_dn5,)
    } else {
        (locals.var_tanh2, locals.var_tanh2_dn3, locals.var_tanh2_dn5,)
    }
};
        locals.var_tanh2 = assign1430_e1824;
        locals.var_tanh2_dn3 = assign1430_e1824_d_n3;
        locals.var_tanh2_dn5 = assign1430_e1824_d_n5;

        let (assign1440_e1838, assign1440_e1838_d_n3, assign1440_e1838_d_n5, assign1440_e1838_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1440_e1834: f64 = (p.p37 * locals.var_vds);
        let assign1440_e1835: f64 = (locals.var_p10_t + assign1440_e1834);
        let assign1440_e1836: f64 = (assign1440_e1835).cosh();
        (assign1440_e1836, ((assign1440_e1835).sinh() * (p.p37 * locals.var_vds_dn3)), ((assign1440_e1835).sinh() * (p.p37 * locals.var_vds_dn5)), ((assign1440_e1835).sinh() * locals.var_p10_t_dn11),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn11,)
    }
};
        locals.var_cosh0 = assign1440_e1838;
        locals.var_cosh0_dn3 = assign1440_e1838_d_n3;
        locals.var_cosh0_dn5 = assign1440_e1838_d_n5;
        locals.var_cosh0_dn11 = assign1440_e1838_d_n11;

        let (assign1450_e1848, assign1450_e1848_d_n3, assign1450_e1848_d_n5, assign1450_e1848_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1450_e1846: f64 = (locals.var_cosh0).ln();
        (assign1450_e1846, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn11 / locals.var_cosh0),)
    } else {
        (locals.var_lc10, locals.var_lc10_dn3, locals.var_lc10_dn5, locals.var_lc10_dn11,)
    }
};
        locals.var_lc10 = assign1450_e1848;
        locals.var_lc10_dn3 = assign1450_e1848_d_n3;
        locals.var_lc10_dn5 = assign1450_e1848_d_n5;
        locals.var_lc10_dn11 = assign1450_e1848_d_n11;

        let (assign1460_e1858, assign1460_e1858_d_n3, assign1460_e1858_d_n5, assign1460_e1858_d_n7, assign1460_e1858_d_n8, assign1460_e1858_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1460_e1856: f64 = (locals.var_psi_1).cosh();
        (assign1460_e1856, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn3), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn5), 0.0, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn8), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn11),)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn7, locals.var_cosh1_dn8, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1460_e1858;
        locals.var_cosh1_dn3 = assign1460_e1858_d_n3;
        locals.var_cosh1_dn5 = assign1460_e1858_d_n5;
        locals.var_cosh1_dn7 = assign1460_e1858_d_n7;
        locals.var_cosh1_dn8 = assign1460_e1858_d_n8;
        locals.var_cosh1_dn11 = assign1460_e1858_d_n11;

        let (assign1470_e1868, assign1470_e1868_d_n3, assign1470_e1868_d_n5, assign1470_e1868_d_n7, assign1470_e1868_d_n8, assign1470_e1868_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1470_e1866: f64 = (locals.var_cosh1).ln();
        (assign1470_e1866, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn7 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc1, locals.var_lc1_dn3, locals.var_lc1_dn5, locals.var_lc1_dn7, locals.var_lc1_dn8, locals.var_lc1_dn11,)
    }
};
        locals.var_lc1 = assign1470_e1868;
        locals.var_lc1_dn3 = assign1470_e1868_d_n3;
        locals.var_lc1_dn5 = assign1470_e1868_d_n5;
        locals.var_lc1_dn7 = assign1470_e1868_d_n7;
        locals.var_lc1_dn8 = assign1470_e1868_d_n8;
        locals.var_lc1_dn11 = assign1470_e1868_d_n11;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1480_e1883, assign1480_e1883_d_n3, assign1480_e1883_d_n5, assign1480_e1883_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1480_e1878: f64 = (p.p37 * locals.var_vds);
        let assign1480_e1879: f64 = (locals.var_p10_t + assign1480_e1878);
        let assign1480_e1881: f64 = (assign1480_e1879 + locals.var_lc10);
        (assign1480_e1881, ((p.p37 * locals.var_vds_dn3) + locals.var_lc10_dn3), ((p.p37 * locals.var_vds_dn5) + locals.var_lc10_dn5), (locals.var_p10_t_dn11 + locals.var_lc10_dn11),)
    } else {
        (locals.var_qgs0, locals.var_qgs0_dn3, locals.var_qgs0_dn5, locals.var_qgs0_dn11,)
    }
};
        locals.var_qgs0 = assign1480_e1883;
        locals.var_qgs0_dn3 = assign1480_e1883_d_n3;
        locals.var_qgs0_dn5 = assign1480_e1883_d_n5;
        locals.var_qgs0_dn11 = assign1480_e1883_d_n11;

        let (assign1490_e1912, assign1490_e1912_d_n3, assign1490_e1912_d_n5, assign1490_e1912_d_n7, assign1490_e1912_d_n8, assign1490_e1912_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1490_e1893: f64 = (locals.var_psi_1 + locals.var_lc1);
        let assign1490_e1895: f64 = (assign1490_e1893 - locals.var_qgs0);
        let assign1490_e1897: f64 = (assign1490_e1895 * locals.var_tanh2);
        let assign1490_e1899: f64 = (assign1490_e1897 / p.p30);
        let assign1490_e1902: f64 = (2.0 * p.p37);
        let assign1490_e1904: f64 = (assign1490_e1902 * locals.var_vgsc);
        let assign1490_e1905: f64 = (assign1490_e1899 + assign1490_e1904);
        let assign1490_e1906: f64 = (locals.var_cgs0_t * assign1490_e1905);
        let assign1490_e1909: f64 = (p.p24 * locals.var_vgsc);
        let assign1490_e1910: f64 = (assign1490_e1906 + assign1490_e1909);
        (assign1490_e1910, (locals.var_cgs0_t * (((((locals.var_psi_1_dn3 + locals.var_lc1_dn3) - locals.var_qgs0_dn3) * locals.var_tanh2) + (assign1490_e1895 * locals.var_tanh2_dn3)) / p.p30)), ((locals.var_cgs0_t * ((((((locals.var_psi_1_dn5 + locals.var_lc1_dn5) - locals.var_qgs0_dn5) * locals.var_tanh2) + (assign1490_e1895 * locals.var_tanh2_dn5)) / p.p30) + (assign1490_e1902 * locals.var_vgsc_dn5))) + (p.p24 * locals.var_vgsc_dn5)), (locals.var_cgs0_t * ((locals.var_lc1_dn7 * locals.var_tanh2) / p.p30)), ((locals.var_cgs0_t * ((((locals.var_psi_1_dn8 + locals.var_lc1_dn8) * locals.var_tanh2) / p.p30) + (assign1490_e1902 * locals.var_vgsc_dn8))) + (p.p24 * locals.var_vgsc_dn8)), ((locals.var_cgs0_t_dn11 * assign1490_e1905) + (locals.var_cgs0_t * ((((locals.var_psi_1_dn11 + locals.var_lc1_dn11) - locals.var_qgs0_dn11) * locals.var_tanh2) / p.p30))),)
    } else {
        (locals.var_qgs, locals.var_qgs_dn3, locals.var_qgs_dn5, locals.var_qgs_dn7, locals.var_qgs_dn8, locals.var_qgs_dn11,)
    }
};
        locals.var_qgs = assign1490_e1912;
        locals.var_qgs_dn3 = assign1490_e1912_d_n3;
        locals.var_qgs_dn5 = assign1490_e1912_d_n5;
        locals.var_qgs_dn7 = assign1490_e1912_d_n7;
        locals.var_qgs_dn8 = assign1490_e1912_d_n8;
        locals.var_qgs_dn11 = assign1490_e1912_d_n11;

        let (assign1500_e1926, assign1500_e1926_d_n3, assign1500_e1926_d_n5, assign1500_e1926_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1500_e1922: f64 = (p.p37 * locals.var_vds);
        let assign1500_e1923: f64 = (locals.var_p40_t - assign1500_e1922);
        let assign1500_e1924: f64 = (assign1500_e1923).cosh();
        (assign1500_e1924, ((assign1500_e1923).sinh() * (-(p.p37 * locals.var_vds_dn3))), ((assign1500_e1923).sinh() * (-(p.p37 * locals.var_vds_dn5))), ((assign1500_e1923).sinh() * locals.var_p40_t_dn11),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn11,)
    }
};
        locals.var_cosh0 = assign1500_e1926;
        locals.var_cosh0_dn3 = assign1500_e1926_d_n3;
        locals.var_cosh0_dn5 = assign1500_e1926_d_n5;
        locals.var_cosh0_dn11 = assign1500_e1926_d_n11;

        let (assign1510_e1936, assign1510_e1936_d_n3, assign1510_e1936_d_n5, assign1510_e1936_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1510_e1934: f64 = (locals.var_cosh0).ln();
        (assign1510_e1934, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn11 / locals.var_cosh0),)
    } else {
        (locals.var_lc40, locals.var_lc40_dn3, locals.var_lc40_dn5, locals.var_lc40_dn11,)
    }
};
        locals.var_lc40 = assign1510_e1936;
        locals.var_lc40_dn3 = assign1510_e1936_d_n3;
        locals.var_lc40_dn5 = assign1510_e1936_d_n5;
        locals.var_lc40_dn11 = assign1510_e1936_d_n11;

        let (assign1520_e1946, assign1520_e1946_d_n3, assign1520_e1946_d_n5, assign1520_e1946_d_n7, assign1520_e1946_d_n8, assign1520_e1946_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1520_e1944: f64 = (locals.var_psi_4).cosh();
        (assign1520_e1944, ((locals.var_psi_4).sinh() * locals.var_psi_4_dn3), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn5), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn7), 0.0, ((locals.var_psi_4).sinh() * locals.var_psi_4_dn11),)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn7, locals.var_cosh1_dn8, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1520_e1946;
        locals.var_cosh1_dn3 = assign1520_e1946_d_n3;
        locals.var_cosh1_dn5 = assign1520_e1946_d_n5;
        locals.var_cosh1_dn7 = assign1520_e1946_d_n7;
        locals.var_cosh1_dn8 = assign1520_e1946_d_n8;
        locals.var_cosh1_dn11 = assign1520_e1946_d_n11;

        let (assign1530_e1956, assign1530_e1956_d_n3, assign1530_e1956_d_n5, assign1530_e1956_d_n7, assign1530_e1956_d_n8, assign1530_e1956_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1530_e1954: f64 = (locals.var_cosh1).ln();
        (assign1530_e1954, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn7 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc4, locals.var_lc4_dn3, locals.var_lc4_dn5, locals.var_lc4_dn7, locals.var_lc4_dn8, locals.var_lc4_dn11,)
    }
};
        locals.var_lc4 = assign1530_e1956;
        locals.var_lc4_dn3 = assign1530_e1956_d_n3;
        locals.var_lc4_dn5 = assign1530_e1956_d_n5;
        locals.var_lc4_dn7 = assign1530_e1956_d_n7;
        locals.var_lc4_dn8 = assign1530_e1956_d_n8;
        locals.var_lc4_dn11 = assign1530_e1956_d_n11;

        let (assign1540_e1971, assign1540_e1971_d_n3, assign1540_e1971_d_n5, assign1540_e1971_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1540_e1966: f64 = (p.p37 * locals.var_vds);
        let assign1540_e1967: f64 = (locals.var_p40_t - assign1540_e1966);
        let assign1540_e1969: f64 = (assign1540_e1967 + locals.var_lc40);
        (assign1540_e1969, ((-(p.p37 * locals.var_vds_dn3)) + locals.var_lc40_dn3), ((-(p.p37 * locals.var_vds_dn5)) + locals.var_lc40_dn5), (locals.var_p40_t_dn11 + locals.var_lc40_dn11),)
    } else {
        (locals.var_qgd0, locals.var_qgd0_dn3, locals.var_qgd0_dn5, locals.var_qgd0_dn11,)
    }
};
        locals.var_qgd0 = assign1540_e1971;
        locals.var_qgd0_dn3 = assign1540_e1971_d_n3;
        locals.var_qgd0_dn5 = assign1540_e1971_d_n5;
        locals.var_qgd0_dn11 = assign1540_e1971_d_n11;

        let (assign1550_e2000, assign1550_e2000_d_n3, assign1550_e2000_d_n5, assign1550_e2000_d_n7, assign1550_e2000_d_n8, assign1550_e2000_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1550_e1981: f64 = (locals.var_psi_4 + locals.var_lc4);
        let assign1550_e1983: f64 = (assign1550_e1981 - locals.var_qgd0);
        let assign1550_e1985: f64 = (assign1550_e1983 * locals.var_tanh3);
        let assign1550_e1987: f64 = (assign1550_e1985 / p.p36);
        let assign1550_e1990: f64 = (2.0 * p.p37);
        let assign1550_e1992: f64 = (assign1550_e1990 * locals.var_vgdc);
        let assign1550_e1993: f64 = (assign1550_e1987 + assign1550_e1992);
        let assign1550_e1994: f64 = (locals.var_cgd0_t * assign1550_e1993);
        let assign1550_e1997: f64 = (p.p26 * locals.var_vgdc);
        let assign1550_e1998: f64 = (assign1550_e1994 + assign1550_e1997);
        (assign1550_e1998, ((locals.var_cgd0_t * ((((((locals.var_psi_4_dn3 + locals.var_lc4_dn3) - locals.var_qgd0_dn3) * locals.var_tanh3) + (assign1550_e1983 * locals.var_tanh3_dn3)) / p.p36) + (assign1550_e1990 * locals.var_vgdc_dn3))) + (p.p26 * locals.var_vgdc_dn3)), (locals.var_cgd0_t * (((((locals.var_psi_4_dn5 + locals.var_lc4_dn5) - locals.var_qgd0_dn5) * locals.var_tanh3) + (assign1550_e1983 * locals.var_tanh3_dn5)) / p.p36)), ((locals.var_cgd0_t * ((((locals.var_psi_4_dn7 + locals.var_lc4_dn7) * locals.var_tanh3) / p.p36) + (assign1550_e1990 * locals.var_vgdc_dn7))) + (p.p26 * locals.var_vgdc_dn7)), (locals.var_cgd0_t * ((locals.var_lc4_dn8 * locals.var_tanh3) / p.p36)), ((locals.var_cgd0_t_dn11 * assign1550_e1993) + (locals.var_cgd0_t * ((((locals.var_psi_4_dn11 + locals.var_lc4_dn11) - locals.var_qgd0_dn11) * locals.var_tanh3) / p.p36))),)
    } else {
        (locals.var_qgd, locals.var_qgd_dn3, locals.var_qgd_dn5, locals.var_qgd_dn7, locals.var_qgd_dn8, locals.var_qgd_dn11,)
    }
};
        locals.var_qgd = assign1550_e2000;
        locals.var_qgd_dn3 = assign1550_e2000_d_n3;
        locals.var_qgd_dn5 = assign1550_e2000_d_n5;
        locals.var_qgd_dn7 = assign1550_e2000_d_n7;
        locals.var_qgd_dn8 = assign1550_e2000_d_n8;
        locals.var_qgd_dn11 = assign1550_e2000_d_n11;

        let (assign1560_e2011, assign1560_e2011_d_n3, assign1560_e2011_d_n5, assign1560_e2011_d_n7, assign1560_e2011_d_n8, assign1560_e2011_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1560_e2009: f64 = locals.var_qgs_dn8;
        (assign1560_e2009, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn7, locals.var_cgs_dn8, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1560_e2011;
        locals.var_cgs_dn3 = assign1560_e2011_d_n3;
        locals.var_cgs_dn5 = assign1560_e2011_d_n5;
        locals.var_cgs_dn7 = assign1560_e2011_d_n7;
        locals.var_cgs_dn8 = assign1560_e2011_d_n8;
        locals.var_cgs_dn11 = assign1560_e2011_d_n11;

        let (assign1570_e2022, assign1570_e2022_d_n3, assign1570_e2022_d_n5, assign1570_e2022_d_n7, assign1570_e2022_d_n8, assign1570_e2022_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1570_e2020: f64 = locals.var_qgd_dn7;
        (assign1570_e2020, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn7, locals.var_cgd_dn8, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1570_e2022;
        locals.var_cgd_dn3 = assign1570_e2022_d_n3;
        locals.var_cgd_dn5 = assign1570_e2022_d_n5;
        locals.var_cgd_dn7 = assign1570_e2022_d_n7;
        locals.var_cgd_dn8 = assign1570_e2022_d_n8;
        locals.var_cgd_dn11 = assign1570_e2022_d_n11;

        let assign1580_e2025: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1580_e2025;

        let assign1630_e2040: f64 = if p.p42 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1630_e2040;

        let assign1640_e2043: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign1640_e2043;

        let assign1650_e2046: f64 = if p.p46 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard23 = assign1650_e2046;

        let assign1660_e2053: f64 = if ((p.p43 > 0.0) || (p.p44 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard24 = assign1660_e2053;

        let assign1670_e2056: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign1670_e2056;

        let assign1680_e2059: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign1680_e2059;

        let assign1690_e2062: f64 = if p.p7 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign1690_e2062;

        let (assign1790_e2206, assign1790_e2206_d_n11,) = {
    if (((locals.var_guard27 != 0.0) && (locals.var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let assign1790_e2193: f64 = (4.0 * 1.3806503e-23);
        let assign1790_e2195: f64 = (assign1790_e2193 * locals.var_t);
        let assign1790_e2197: f64 = (assign1790_e2195 * p.p73);
        let assign1790_e2199: f64 = (assign1790_e2197 * locals.var_cgs0_t);
        let assign1790_e2202: f64 = (p.p72 * p.p71);
        let assign1790_e2203: f64 = (assign1790_e2202).sqrt();
        let assign1790_e2204: f64 = (assign1790_e2199 * assign1790_e2203);
        (assign1790_e2204, (((((assign1790_e2193 * locals.var_t_dn11) * p.p73) * locals.var_cgs0_t) + (assign1790_e2197 * locals.var_cgs0_t_dn11)) * assign1790_e2203),)
    } else {
        (locals.var_k, locals.var_k_dn11,)
    }
};
        locals.var_k = assign1790_e2206;
        locals.var_k_dn11 = assign1790_e2206_d_n11;

        let (assign1820_e2243, assign1820_e2243_d_n11,) = {
    if (((locals.var_guard27 != 0.0) && (locals.var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let assign1820_e2241: f64 = (locals.var_k * 3.141592653589793);
        (assign1820_e2241, (locals.var_k_dn11 * 3.141592653589793),)
    } else {
        (locals.var_ci, locals.var_ci_dn11,)
    }
};
        locals.var_ci = assign1820_e2243;
        locals.var_ci_dn11 = assign1820_e2243_d_n11;

        let assign1860_e2271: f64 = if ((p.p1 != 0.0) && (p.p57 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard43 = assign1860_e2271;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        locals.var_vgs = (nv8 - nv5);
        locals.var_vgs_dn5 = -1.0;
        locals.var_vgs_dn8 = 1.0;
        locals.var_vgs_rv = 0.0;

        locals.var_vds = (nv3 - nv5);
        locals.var_vds_dn3 = 1.0;
        locals.var_vds_dn5 = -1.0;
        locals.var_vds_rv = 0.0;

        locals.var_vgsc = locals.var_vgs;
        locals.var_vgsc_dn5 = locals.var_vgs_dn5;
        locals.var_vgsc_dn8 = locals.var_vgs_dn8;
        locals.var_vgsc_rv = 0.0;

        locals.var_vgdc = (nv7 - nv3);
        locals.var_vgdc_dn3 = -1.0;
        locals.var_vgdc_dn7 = 1.0;
        locals.var_vgdc_rv = 0.0;

        locals.var_qgd = 0.0;
        locals.var_qgd_dn3 = 0.0;
        locals.var_qgd_dn5 = 0.0;
        locals.var_qgd_dn7 = 0.0;
        locals.var_qgd_dn8 = 0.0;
        locals.var_qgd_dn11 = 0.0;
        locals.var_qgd_rv = 0.0;

        locals.var_qgs = 0.0;
        locals.var_qgs_dn3 = 0.0;
        locals.var_qgs_dn5 = 0.0;
        locals.var_qgs_dn7 = 0.0;
        locals.var_qgs_dn8 = 0.0;
        locals.var_qgs_dn11 = 0.0;
        locals.var_qgs_rv = 0.0;

        locals.var_cgd = 0.0;
        locals.var_cgd_dn3 = 0.0;
        locals.var_cgd_dn5 = 0.0;
        locals.var_cgd_dn7 = 0.0;
        locals.var_cgd_dn8 = 0.0;
        locals.var_cgd_dn11 = 0.0;
        locals.var_cgd_rv = 0.0;

        locals.var_cgs = 0.0;
        locals.var_cgs_dn3 = 0.0;
        locals.var_cgs_dn5 = 0.0;
        locals.var_cgs_dn7 = 0.0;
        locals.var_cgs_dn8 = 0.0;
        locals.var_cgs_dn11 = 0.0;
        locals.var_cgs_rv = 0.0;

        let assign120_e575: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard1 = assign120_e575;
        locals.var_guard1_rv = 0.0;

        let (assign130_e581, assign130_e581_d_n11,) = {
    if (locals.var_guard1 != 0.0) {
        let assign130_e579: f64 = (p.p3 + 273.15);
        (assign130_e579, 0.0,)
    } else {
        (locals.var_t, locals.var_t_dn11,)
    }
};
        locals.var_t = assign130_e581;
        locals.var_t_dn11 = assign130_e581_d_n11;
        locals.var_t_rv = 0.0;

        let (assign140_e588, assign140_e588_d_n11,) = {
    if (locals.var_guard1 == 0.0) {
        let assign140_e584: f64 = ctx_temp;
        let assign140_e586: f64 = (assign140_e584 + p.p2);
        (assign140_e586, 0.0,)
    } else {
        (locals.var_t, locals.var_t_dn11,)
    }
};
        locals.var_t = assign140_e588;
        locals.var_t_dn11 = assign140_e588_d_n11;
        locals.var_t_rv = 0.0;

        let assign150_e590: f64 = if param_given[85] { 1.0 } else { 0.0 };
        locals.var_guard2 = assign150_e590;
        locals.var_guard2_rv = 0.0;

        let (assign160_e596,) = {
    if (locals.var_guard2 != 0.0) {
        let assign160_e594: f64 = (p.p85 + 273.15);
        (assign160_e594,)
    } else {
        (locals.var_t_nom,)
    }
};
        locals.var_t_nom = assign160_e596;
        locals.var_t_nom_rv = 0.0;

        let (assign170_e603,) = {
    if (locals.var_guard2 == 0.0) {
        let assign170_e601: f64 = (27.0 + 273.15);
        (assign170_e601,)
    } else {
        (locals.var_t_nom,)
    }
};
        locals.var_t_nom = assign170_e603;
        locals.var_t_nom_rv = 0.0;

        let (assign180_e610, assign180_e610_d_n11,) = {
    if (p.p1 != 0.0) {
        let assign180_e607: f64 = ((nv11 - 0.0)).abs();
        let assign180_e608: f64 = (locals.var_t + assign180_e607);
        (assign180_e608, (locals.var_t_dn11 + if (nv11 - 0.0) >= 0.0 { 1.0 } else { (-1.0) }),)
    } else {
        (locals.var_t, locals.var_t_dn11,)
    }
};
        locals.var_t = assign180_e610;
        locals.var_t_dn11 = assign180_e610_d_n11;
        locals.var_t_rv = 0.0;

        let assign200_e615: f64 = (locals.var_t - locals.var_t_nom);
        let assign200_e616: f64 = (assign200_e615).abs();
        locals.var_delta_t = assign200_e616;
        locals.var_delta_t_dn11 = if assign200_e615 >= 0.0 { locals.var_t_dn11 } else { (-locals.var_t_dn11) };
        locals.var_delta_t_rv = 0.0;

        let assign210_e623: f64 = if ((locals.var_delta_t > 0.0) || (p.p57 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard3 = assign210_e623;
        locals.var_guard3_rv = 0.0;

        let (assign260_e673, assign260_e673_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign260_e669: f64 = (p.p61 * locals.var_delta_t);
        let assign260_e670: f64 = (1.0 + assign260_e669);
        let assign260_e671: f64 = (p.p25 * assign260_e670);
        (assign260_e671, (p.p25 * (p.p61 * locals.var_delta_t_dn11)),)
    } else {
        (locals.var_cgs0_t, locals.var_cgs0_t_dn11,)
    }
};
        locals.var_cgs0_t = assign260_e673;
        locals.var_cgs0_t_dn11 = assign260_e673_d_n11;
        locals.var_cgs0_t_rv = 0.0;

        let (assign270_e683, assign270_e683_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign270_e679: f64 = (p.p62 * locals.var_delta_t);
        let assign270_e680: f64 = (1.0 + assign270_e679);
        let assign270_e681: f64 = (p.p28 * assign270_e680);
        (assign270_e681, (p.p28 * (p.p62 * locals.var_delta_t_dn11)),)
    } else {
        (locals.var_cgd0_t, locals.var_cgd0_t_dn11,)
    }
};
        locals.var_cgd0_t = assign270_e683;
        locals.var_cgd0_t_dn11 = assign270_e683_d_n11;
        locals.var_cgd0_t_rv = 0.0;

        let (assign310_e721, assign310_e721_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign310_e716: f64 = (p.p30 * p.p68);
        let assign310_e718: f64 = (assign310_e716 * locals.var_delta_t);
        let assign310_e719: f64 = (p.p29 + assign310_e718);
        (assign310_e719, (assign310_e716 * locals.var_delta_t_dn11),)
    } else {
        (locals.var_p10_t, locals.var_p10_t_dn11,)
    }
};
        locals.var_p10_t = assign310_e721;
        locals.var_p10_t_dn11 = assign310_e721_d_n11;
        locals.var_p10_t_rv = 0.0;

        let (assign320_e731, assign320_e731_d_n11,) = {
    if (locals.var_guard3 != 0.0) {
        let assign320_e726: f64 = (p.p36 * p.p68);
        let assign320_e728: f64 = (assign320_e726 * locals.var_delta_t);
        let assign320_e729: f64 = (p.p35 + assign320_e728);
        (assign320_e729, (assign320_e726 * locals.var_delta_t_dn11),)
    } else {
        (locals.var_p40_t, locals.var_p40_t_dn11,)
    }
};
        locals.var_p40_t = assign320_e731;
        locals.var_p40_t_dn11 = assign320_e731_d_n11;
        locals.var_p40_t_rv = 0.0;

        let (assign380_e767, assign380_e767_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p25, 0.0,)
    } else {
        (locals.var_cgs0_t, locals.var_cgs0_t_dn11,)
    }
};
        locals.var_cgs0_t = assign380_e767;
        locals.var_cgs0_t_dn11 = assign380_e767_d_n11;
        locals.var_cgs0_t_rv = 0.0;

        let (assign390_e772, assign390_e772_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p28, 0.0,)
    } else {
        (locals.var_cgd0_t, locals.var_cgd0_t_dn11,)
    }
};
        locals.var_cgd0_t = assign390_e772;
        locals.var_cgd0_t_dn11 = assign390_e772_d_n11;
        locals.var_cgd0_t_rv = 0.0;

        let (assign430_e792, assign430_e792_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p29, 0.0,)
    } else {
        (locals.var_p10_t, locals.var_p10_t_dn11,)
    }
};
        locals.var_p10_t = assign430_e792;
        locals.var_p10_t_dn11 = assign430_e792_d_n11;
        locals.var_p10_t_rv = 0.0;

        let (assign440_e797, assign440_e797_d_n11,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p35, 0.0,)
    } else {
        (locals.var_p40_t, locals.var_p40_t_dn11,)
    }
};
        locals.var_p40_t = assign440_e797;
        locals.var_p40_t_dn11 = assign440_e797_d_n11;
        locals.var_p40_t_rv = 0.0;

        let assign1280_e1724: f64 = (p.p30 * locals.var_vgsc);
        let assign1280_e1725: f64 = (locals.var_p10_t + assign1280_e1724);
        let assign1280_e1728: f64 = (p.p37 * locals.var_vds);
        let assign1280_e1729: f64 = (assign1280_e1725 + assign1280_e1728);
        locals.var_psi_1 = assign1280_e1729;
        locals.var_psi_1_dn3 = (p.p37 * locals.var_vds_dn3);
        locals.var_psi_1_dn5 = ((p.p30 * locals.var_vgsc_dn5) + (p.p37 * locals.var_vds_dn5));
        locals.var_psi_1_dn8 = (p.p30 * locals.var_vgsc_dn8);
        locals.var_psi_1_dn11 = locals.var_p10_t_dn11;
        locals.var_psi_1_rv = 0.0;

        let assign1290_e1732: f64 = (locals.var_psi_1).tanh();
        let assign1290_e1733: f64 = (1.0 + assign1290_e1732);
        locals.var_tanh1 = assign1290_e1733;
        locals.var_tanh1_dn3 = (locals.var_psi_1_dn3 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn5 = (locals.var_psi_1_dn5 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn8 = (locals.var_psi_1_dn8 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn11 = (locals.var_psi_1_dn11 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_rv = 0.0;

        let assign1300_e1737: f64 = (p.p32 * locals.var_vds);
        let assign1300_e1738: f64 = (p.p31 + assign1300_e1737);
        locals.var_psi_2 = assign1300_e1738;
        locals.var_psi_2_dn3 = (p.p32 * locals.var_vds_dn3);
        locals.var_psi_2_dn5 = (p.p32 * locals.var_vds_dn5);
        locals.var_psi_2_rv = 0.0;

        let assign1310_e1741: f64 = (locals.var_psi_2).tanh();
        let assign1310_e1742: f64 = (1.0 + assign1310_e1741);
        locals.var_tanh2 = assign1310_e1742;
        locals.var_tanh2_dn3 = (locals.var_psi_2_dn3 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh()));
        locals.var_tanh2_dn5 = (locals.var_psi_2_dn5 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh()));
        locals.var_tanh2_rv = 0.0;

        let assign1320_e1746: f64 = (p.p34 * locals.var_vds);
        let assign1320_e1747: f64 = (p.p33 - assign1320_e1746);
        locals.var_psi_3 = assign1320_e1747;
        locals.var_psi_3_dn3 = (-(p.p34 * locals.var_vds_dn3));
        locals.var_psi_3_dn5 = (-(p.p34 * locals.var_vds_dn5));
        locals.var_psi_3_rv = 0.0;

        let assign1330_e1750: f64 = (locals.var_psi_3).tanh();
        let assign1330_e1751: f64 = (1.0 + assign1330_e1750);
        let assign1330_e1753: f64 = (assign1330_e1751 - p.p37);
        locals.var_tanh3 = assign1330_e1753;
        locals.var_tanh3_dn3 = (locals.var_psi_3_dn3 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh()));
        locals.var_tanh3_dn5 = (locals.var_psi_3_dn5 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh()));
        locals.var_tanh3_rv = 0.0;

        let assign1340_e1757: f64 = (p.p36 * locals.var_vgdc);
        let assign1340_e1758: f64 = (locals.var_p40_t + assign1340_e1757);
        let assign1340_e1761: f64 = (p.p37 * locals.var_vds);
        let assign1340_e1762: f64 = (assign1340_e1758 - assign1340_e1761);
        locals.var_psi_4 = assign1340_e1762;
        locals.var_psi_4_dn3 = ((p.p36 * locals.var_vgdc_dn3) - (p.p37 * locals.var_vds_dn3));
        locals.var_psi_4_dn5 = (-(p.p37 * locals.var_vds_dn5));
        locals.var_psi_4_dn7 = (p.p36 * locals.var_vgdc_dn7);
        locals.var_psi_4_dn11 = locals.var_p40_t_dn11;
        locals.var_psi_4_rv = 0.0;

        let assign1350_e1765: f64 = (locals.var_psi_4).tanh();
        let assign1350_e1766: f64 = (1.0 + assign1350_e1765);
        locals.var_tanh4 = assign1350_e1766;
        locals.var_tanh4_dn3 = (locals.var_psi_4_dn3 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn5 = (locals.var_psi_4_dn5 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn7 = (locals.var_psi_4_dn7 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn11 = (locals.var_psi_4_dn11 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_rv = 0.0;

        let assign1360_e1769: f64 = if p.p6 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign1360_e1769;
        locals.var_guard13_rv = 0.0;

        let assign1370_e1772: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1370_e1772;
        locals.var_guard14_rv = 0.0;

        let assign1380_e1775: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1380_e1775;
        locals.var_guard15_rv = 0.0;

        let (assign1390_e1779, assign1390_e1779_d_n3, assign1390_e1779_d_n5, assign1390_e1779_d_n7, assign1390_e1779_d_n8, assign1390_e1779_d_n11,) = {
    if (locals.var_guard13 != 0.0) {
        (p.p24, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn7, locals.var_cgs_dn8, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1390_e1779;
        locals.var_cgs_dn3 = assign1390_e1779_d_n3;
        locals.var_cgs_dn5 = assign1390_e1779_d_n5;
        locals.var_cgs_dn7 = assign1390_e1779_d_n7;
        locals.var_cgs_dn8 = assign1390_e1779_d_n8;
        locals.var_cgs_dn11 = assign1390_e1779_d_n11;
        locals.var_cgs_rv = 0.0;

        let (assign1400_e1783, assign1400_e1783_d_n3, assign1400_e1783_d_n5, assign1400_e1783_d_n7, assign1400_e1783_d_n8, assign1400_e1783_d_n11,) = {
    if (locals.var_guard13 != 0.0) {
        (p.p26, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn7, locals.var_cgd_dn8, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1400_e1783;
        locals.var_cgd_dn3 = assign1400_e1783_d_n3;
        locals.var_cgd_dn5 = assign1400_e1783_d_n5;
        locals.var_cgd_dn7 = assign1400_e1783_d_n7;
        locals.var_cgd_dn8 = assign1400_e1783_d_n8;
        locals.var_cgd_dn11 = assign1400_e1783_d_n11;
        locals.var_cgd_rv = 0.0;

        let (assign1410_e1796, assign1410_e1796_d_n3, assign1410_e1796_d_n5, assign1410_e1796_d_n7, assign1410_e1796_d_n8, assign1410_e1796_d_n11,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign1410_e1791: f64 = (locals.var_cgs0_t * locals.var_tanh1);
        let assign1410_e1793: f64 = (assign1410_e1791 * locals.var_tanh2);
        let assign1410_e1794: f64 = (p.p24 + assign1410_e1793);
        (assign1410_e1794, (((locals.var_cgs0_t * locals.var_tanh1_dn3) * locals.var_tanh2) + (assign1410_e1791 * locals.var_tanh2_dn3)), (((locals.var_cgs0_t * locals.var_tanh1_dn5) * locals.var_tanh2) + (assign1410_e1791 * locals.var_tanh2_dn5)), 0.0, ((locals.var_cgs0_t * locals.var_tanh1_dn8) * locals.var_tanh2), (((locals.var_cgs0_t_dn11 * locals.var_tanh1) + (locals.var_cgs0_t * locals.var_tanh1_dn11)) * locals.var_tanh2),)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn7, locals.var_cgs_dn8, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1410_e1796;
        locals.var_cgs_dn3 = assign1410_e1796_d_n3;
        locals.var_cgs_dn5 = assign1410_e1796_d_n5;
        locals.var_cgs_dn7 = assign1410_e1796_d_n7;
        locals.var_cgs_dn8 = assign1410_e1796_d_n8;
        locals.var_cgs_dn11 = assign1410_e1796_d_n11;
        locals.var_cgs_rv = 0.0;

        let (assign1420_e1813, assign1420_e1813_d_n3, assign1420_e1813_d_n5, assign1420_e1813_d_n7, assign1420_e1813_d_n8, assign1420_e1813_d_n11,) = {
    if ((locals.var_guard14 != 0.0) && (locals.var_guard13 == 0.0)) {
        let assign1420_e1805: f64 = (locals.var_tanh3 * locals.var_tanh4);
        let assign1420_e1808: f64 = (2.0 * p.p37);
        let assign1420_e1809: f64 = (assign1420_e1805 + assign1420_e1808);
        let assign1420_e1810: f64 = (locals.var_cgd0_t * assign1420_e1809);
        let assign1420_e1811: f64 = (p.p26 + assign1420_e1810);
        (assign1420_e1811, (locals.var_cgd0_t * ((locals.var_tanh3_dn3 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn3))), (locals.var_cgd0_t * ((locals.var_tanh3_dn5 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn5))), (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn7)), 0.0, ((locals.var_cgd0_t_dn11 * assign1420_e1809) + (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn11))),)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn7, locals.var_cgd_dn8, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1420_e1813;
        locals.var_cgd_dn3 = assign1420_e1813_d_n3;
        locals.var_cgd_dn5 = assign1420_e1813_d_n5;
        locals.var_cgd_dn7 = assign1420_e1813_d_n7;
        locals.var_cgd_dn8 = assign1420_e1813_d_n8;
        locals.var_cgd_dn11 = assign1420_e1813_d_n11;
        locals.var_cgd_rv = 0.0;

        let (assign1430_e1824, assign1430_e1824_d_n3, assign1430_e1824_d_n5,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1430_e1822: f64 = (locals.var_tanh2 - p.p37);
        (assign1430_e1822, locals.var_tanh2_dn3, locals.var_tanh2_dn5,)
    } else {
        (locals.var_tanh2, locals.var_tanh2_dn3, locals.var_tanh2_dn5,)
    }
};
        locals.var_tanh2 = assign1430_e1824;
        locals.var_tanh2_dn3 = assign1430_e1824_d_n3;
        locals.var_tanh2_dn5 = assign1430_e1824_d_n5;
        locals.var_tanh2_rv = 0.0;

        let (assign1440_e1838, assign1440_e1838_d_n3, assign1440_e1838_d_n5, assign1440_e1838_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1440_e1834: f64 = (p.p37 * locals.var_vds);
        let assign1440_e1835: f64 = (locals.var_p10_t + assign1440_e1834);
        let assign1440_e1836: f64 = (assign1440_e1835).cosh();
        (assign1440_e1836, ((assign1440_e1835).sinh() * (p.p37 * locals.var_vds_dn3)), ((assign1440_e1835).sinh() * (p.p37 * locals.var_vds_dn5)), ((assign1440_e1835).sinh() * locals.var_p10_t_dn11),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn11,)
    }
};
        locals.var_cosh0 = assign1440_e1838;
        locals.var_cosh0_dn3 = assign1440_e1838_d_n3;
        locals.var_cosh0_dn5 = assign1440_e1838_d_n5;
        locals.var_cosh0_dn11 = assign1440_e1838_d_n11;
        locals.var_cosh0_rv = 0.0;

        let (assign1450_e1848, assign1450_e1848_d_n3, assign1450_e1848_d_n5, assign1450_e1848_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1450_e1846: f64 = (locals.var_cosh0).ln();
        (assign1450_e1846, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn11 / locals.var_cosh0),)
    } else {
        (locals.var_lc10, locals.var_lc10_dn3, locals.var_lc10_dn5, locals.var_lc10_dn11,)
    }
};
        locals.var_lc10 = assign1450_e1848;
        locals.var_lc10_dn3 = assign1450_e1848_d_n3;
        locals.var_lc10_dn5 = assign1450_e1848_d_n5;
        locals.var_lc10_dn11 = assign1450_e1848_d_n11;
        locals.var_lc10_rv = 0.0;

        let (assign1460_e1858, assign1460_e1858_d_n3, assign1460_e1858_d_n5, assign1460_e1858_d_n7, assign1460_e1858_d_n8, assign1460_e1858_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1460_e1856: f64 = (locals.var_psi_1).cosh();
        (assign1460_e1856, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn3), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn5), 0.0, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn8), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn11),)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn7, locals.var_cosh1_dn8, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1460_e1858;
        locals.var_cosh1_dn3 = assign1460_e1858_d_n3;
        locals.var_cosh1_dn5 = assign1460_e1858_d_n5;
        locals.var_cosh1_dn7 = assign1460_e1858_d_n7;
        locals.var_cosh1_dn8 = assign1460_e1858_d_n8;
        locals.var_cosh1_dn11 = assign1460_e1858_d_n11;
        locals.var_cosh1_rv = 0.0;

        let (assign1470_e1868, assign1470_e1868_d_n3, assign1470_e1868_d_n5, assign1470_e1868_d_n7, assign1470_e1868_d_n8, assign1470_e1868_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1470_e1866: f64 = (locals.var_cosh1).ln();
        (assign1470_e1866, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn7 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc1, locals.var_lc1_dn3, locals.var_lc1_dn5, locals.var_lc1_dn7, locals.var_lc1_dn8, locals.var_lc1_dn11,)
    }
};
        locals.var_lc1 = assign1470_e1868;
        locals.var_lc1_dn3 = assign1470_e1868_d_n3;
        locals.var_lc1_dn5 = assign1470_e1868_d_n5;
        locals.var_lc1_dn7 = assign1470_e1868_d_n7;
        locals.var_lc1_dn8 = assign1470_e1868_d_n8;
        locals.var_lc1_dn11 = assign1470_e1868_d_n11;
        locals.var_lc1_rv = 0.0;

        let (assign1480_e1883, assign1480_e1883_d_n3, assign1480_e1883_d_n5, assign1480_e1883_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1480_e1878: f64 = (p.p37 * locals.var_vds);
        let assign1480_e1879: f64 = (locals.var_p10_t + assign1480_e1878);
        let assign1480_e1881: f64 = (assign1480_e1879 + locals.var_lc10);
        (assign1480_e1881, ((p.p37 * locals.var_vds_dn3) + locals.var_lc10_dn3), ((p.p37 * locals.var_vds_dn5) + locals.var_lc10_dn5), (locals.var_p10_t_dn11 + locals.var_lc10_dn11),)
    } else {
        (locals.var_qgs0, locals.var_qgs0_dn3, locals.var_qgs0_dn5, locals.var_qgs0_dn11,)
    }
};
        locals.var_qgs0 = assign1480_e1883;
        locals.var_qgs0_dn3 = assign1480_e1883_d_n3;
        locals.var_qgs0_dn5 = assign1480_e1883_d_n5;
        locals.var_qgs0_dn11 = assign1480_e1883_d_n11;
        locals.var_qgs0_rv = 0.0;

        let (assign1490_e1912, assign1490_e1912_d_n3, assign1490_e1912_d_n5, assign1490_e1912_d_n7, assign1490_e1912_d_n8, assign1490_e1912_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1490_e1893: f64 = (locals.var_psi_1 + locals.var_lc1);
        let assign1490_e1895: f64 = (assign1490_e1893 - locals.var_qgs0);
        let assign1490_e1897: f64 = (assign1490_e1895 * locals.var_tanh2);
        let assign1490_e1899: f64 = (assign1490_e1897 / p.p30);
        let assign1490_e1902: f64 = (2.0 * p.p37);
        let assign1490_e1904: f64 = (assign1490_e1902 * locals.var_vgsc);
        let assign1490_e1905: f64 = (assign1490_e1899 + assign1490_e1904);
        let assign1490_e1906: f64 = (locals.var_cgs0_t * assign1490_e1905);
        let assign1490_e1909: f64 = (p.p24 * locals.var_vgsc);
        let assign1490_e1910: f64 = (assign1490_e1906 + assign1490_e1909);
        (assign1490_e1910, (locals.var_cgs0_t * (((((locals.var_psi_1_dn3 + locals.var_lc1_dn3) - locals.var_qgs0_dn3) * locals.var_tanh2) + (assign1490_e1895 * locals.var_tanh2_dn3)) / p.p30)), ((locals.var_cgs0_t * ((((((locals.var_psi_1_dn5 + locals.var_lc1_dn5) - locals.var_qgs0_dn5) * locals.var_tanh2) + (assign1490_e1895 * locals.var_tanh2_dn5)) / p.p30) + (assign1490_e1902 * locals.var_vgsc_dn5))) + (p.p24 * locals.var_vgsc_dn5)), (locals.var_cgs0_t * ((locals.var_lc1_dn7 * locals.var_tanh2) / p.p30)), ((locals.var_cgs0_t * ((((locals.var_psi_1_dn8 + locals.var_lc1_dn8) * locals.var_tanh2) / p.p30) + (assign1490_e1902 * locals.var_vgsc_dn8))) + (p.p24 * locals.var_vgsc_dn8)), ((locals.var_cgs0_t_dn11 * assign1490_e1905) + (locals.var_cgs0_t * ((((locals.var_psi_1_dn11 + locals.var_lc1_dn11) - locals.var_qgs0_dn11) * locals.var_tanh2) / p.p30))),)
    } else {
        (locals.var_qgs, locals.var_qgs_dn3, locals.var_qgs_dn5, locals.var_qgs_dn7, locals.var_qgs_dn8, locals.var_qgs_dn11,)
    }
};
        locals.var_qgs = assign1490_e1912;
        locals.var_qgs_dn3 = assign1490_e1912_d_n3;
        locals.var_qgs_dn5 = assign1490_e1912_d_n5;
        locals.var_qgs_dn7 = assign1490_e1912_d_n7;
        locals.var_qgs_dn8 = assign1490_e1912_d_n8;
        locals.var_qgs_dn11 = assign1490_e1912_d_n11;
        locals.var_qgs_rv = 0.0;

        let (assign1500_e1926, assign1500_e1926_d_n3, assign1500_e1926_d_n5, assign1500_e1926_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1500_e1922: f64 = (p.p37 * locals.var_vds);
        let assign1500_e1923: f64 = (locals.var_p40_t - assign1500_e1922);
        let assign1500_e1924: f64 = (assign1500_e1923).cosh();
        (assign1500_e1924, ((assign1500_e1923).sinh() * (-(p.p37 * locals.var_vds_dn3))), ((assign1500_e1923).sinh() * (-(p.p37 * locals.var_vds_dn5))), ((assign1500_e1923).sinh() * locals.var_p40_t_dn11),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn11,)
    }
};
        locals.var_cosh0 = assign1500_e1926;
        locals.var_cosh0_dn3 = assign1500_e1926_d_n3;
        locals.var_cosh0_dn5 = assign1500_e1926_d_n5;
        locals.var_cosh0_dn11 = assign1500_e1926_d_n11;
        locals.var_cosh0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1510_e1936, assign1510_e1936_d_n3, assign1510_e1936_d_n5, assign1510_e1936_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1510_e1934: f64 = (locals.var_cosh0).ln();
        (assign1510_e1934, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn11 / locals.var_cosh0),)
    } else {
        (locals.var_lc40, locals.var_lc40_dn3, locals.var_lc40_dn5, locals.var_lc40_dn11,)
    }
};
        locals.var_lc40 = assign1510_e1936;
        locals.var_lc40_dn3 = assign1510_e1936_d_n3;
        locals.var_lc40_dn5 = assign1510_e1936_d_n5;
        locals.var_lc40_dn11 = assign1510_e1936_d_n11;
        locals.var_lc40_rv = 0.0;

        let (assign1520_e1946, assign1520_e1946_d_n3, assign1520_e1946_d_n5, assign1520_e1946_d_n7, assign1520_e1946_d_n8, assign1520_e1946_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1520_e1944: f64 = (locals.var_psi_4).cosh();
        (assign1520_e1944, ((locals.var_psi_4).sinh() * locals.var_psi_4_dn3), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn5), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn7), 0.0, ((locals.var_psi_4).sinh() * locals.var_psi_4_dn11),)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn7, locals.var_cosh1_dn8, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1520_e1946;
        locals.var_cosh1_dn3 = assign1520_e1946_d_n3;
        locals.var_cosh1_dn5 = assign1520_e1946_d_n5;
        locals.var_cosh1_dn7 = assign1520_e1946_d_n7;
        locals.var_cosh1_dn8 = assign1520_e1946_d_n8;
        locals.var_cosh1_dn11 = assign1520_e1946_d_n11;
        locals.var_cosh1_rv = 0.0;

        let (assign1530_e1956, assign1530_e1956_d_n3, assign1530_e1956_d_n5, assign1530_e1956_d_n7, assign1530_e1956_d_n8, assign1530_e1956_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1530_e1954: f64 = (locals.var_cosh1).ln();
        (assign1530_e1954, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn7 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc4, locals.var_lc4_dn3, locals.var_lc4_dn5, locals.var_lc4_dn7, locals.var_lc4_dn8, locals.var_lc4_dn11,)
    }
};
        locals.var_lc4 = assign1530_e1956;
        locals.var_lc4_dn3 = assign1530_e1956_d_n3;
        locals.var_lc4_dn5 = assign1530_e1956_d_n5;
        locals.var_lc4_dn7 = assign1530_e1956_d_n7;
        locals.var_lc4_dn8 = assign1530_e1956_d_n8;
        locals.var_lc4_dn11 = assign1530_e1956_d_n11;
        locals.var_lc4_rv = 0.0;

        let (assign1540_e1971, assign1540_e1971_d_n3, assign1540_e1971_d_n5, assign1540_e1971_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1540_e1966: f64 = (p.p37 * locals.var_vds);
        let assign1540_e1967: f64 = (locals.var_p40_t - assign1540_e1966);
        let assign1540_e1969: f64 = (assign1540_e1967 + locals.var_lc40);
        (assign1540_e1969, ((-(p.p37 * locals.var_vds_dn3)) + locals.var_lc40_dn3), ((-(p.p37 * locals.var_vds_dn5)) + locals.var_lc40_dn5), (locals.var_p40_t_dn11 + locals.var_lc40_dn11),)
    } else {
        (locals.var_qgd0, locals.var_qgd0_dn3, locals.var_qgd0_dn5, locals.var_qgd0_dn11,)
    }
};
        locals.var_qgd0 = assign1540_e1971;
        locals.var_qgd0_dn3 = assign1540_e1971_d_n3;
        locals.var_qgd0_dn5 = assign1540_e1971_d_n5;
        locals.var_qgd0_dn11 = assign1540_e1971_d_n11;
        locals.var_qgd0_rv = 0.0;

        let (assign1550_e2000, assign1550_e2000_d_n3, assign1550_e2000_d_n5, assign1550_e2000_d_n7, assign1550_e2000_d_n8, assign1550_e2000_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1550_e1981: f64 = (locals.var_psi_4 + locals.var_lc4);
        let assign1550_e1983: f64 = (assign1550_e1981 - locals.var_qgd0);
        let assign1550_e1985: f64 = (assign1550_e1983 * locals.var_tanh3);
        let assign1550_e1987: f64 = (assign1550_e1985 / p.p36);
        let assign1550_e1990: f64 = (2.0 * p.p37);
        let assign1550_e1992: f64 = (assign1550_e1990 * locals.var_vgdc);
        let assign1550_e1993: f64 = (assign1550_e1987 + assign1550_e1992);
        let assign1550_e1994: f64 = (locals.var_cgd0_t * assign1550_e1993);
        let assign1550_e1997: f64 = (p.p26 * locals.var_vgdc);
        let assign1550_e1998: f64 = (assign1550_e1994 + assign1550_e1997);
        (assign1550_e1998, ((locals.var_cgd0_t * ((((((locals.var_psi_4_dn3 + locals.var_lc4_dn3) - locals.var_qgd0_dn3) * locals.var_tanh3) + (assign1550_e1983 * locals.var_tanh3_dn3)) / p.p36) + (assign1550_e1990 * locals.var_vgdc_dn3))) + (p.p26 * locals.var_vgdc_dn3)), (locals.var_cgd0_t * (((((locals.var_psi_4_dn5 + locals.var_lc4_dn5) - locals.var_qgd0_dn5) * locals.var_tanh3) + (assign1550_e1983 * locals.var_tanh3_dn5)) / p.p36)), ((locals.var_cgd0_t * ((((locals.var_psi_4_dn7 + locals.var_lc4_dn7) * locals.var_tanh3) / p.p36) + (assign1550_e1990 * locals.var_vgdc_dn7))) + (p.p26 * locals.var_vgdc_dn7)), (locals.var_cgd0_t * ((locals.var_lc4_dn8 * locals.var_tanh3) / p.p36)), ((locals.var_cgd0_t_dn11 * assign1550_e1993) + (locals.var_cgd0_t * ((((locals.var_psi_4_dn11 + locals.var_lc4_dn11) - locals.var_qgd0_dn11) * locals.var_tanh3) / p.p36))),)
    } else {
        (locals.var_qgd, locals.var_qgd_dn3, locals.var_qgd_dn5, locals.var_qgd_dn7, locals.var_qgd_dn8, locals.var_qgd_dn11,)
    }
};
        locals.var_qgd = assign1550_e2000;
        locals.var_qgd_dn3 = assign1550_e2000_d_n3;
        locals.var_qgd_dn5 = assign1550_e2000_d_n5;
        locals.var_qgd_dn7 = assign1550_e2000_d_n7;
        locals.var_qgd_dn8 = assign1550_e2000_d_n8;
        locals.var_qgd_dn11 = assign1550_e2000_d_n11;
        locals.var_qgd_rv = 0.0;

        let (assign1560_e2011, assign1560_e2011_d_n3, assign1560_e2011_d_n5, assign1560_e2011_d_n7, assign1560_e2011_d_n8, assign1560_e2011_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1560_e2009: f64 = locals.var_qgs_dn8;
        (assign1560_e2009, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn7, locals.var_cgs_dn8, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1560_e2011;
        locals.var_cgs_dn3 = assign1560_e2011_d_n3;
        locals.var_cgs_dn5 = assign1560_e2011_d_n5;
        locals.var_cgs_dn7 = assign1560_e2011_d_n7;
        locals.var_cgs_dn8 = assign1560_e2011_d_n8;
        locals.var_cgs_dn11 = assign1560_e2011_d_n11;
        locals.var_cgs_rv = 0.0;

        let (assign1570_e2022, assign1570_e2022_d_n3, assign1570_e2022_d_n5, assign1570_e2022_d_n7, assign1570_e2022_d_n8, assign1570_e2022_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (!((locals.var_guard13 != 0.0) || (locals.var_guard14 != 0.0)))) {
        let assign1570_e2020: f64 = locals.var_qgd_dn7;
        (assign1570_e2020, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn7, locals.var_cgd_dn8, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1570_e2022;
        locals.var_cgd_dn3 = assign1570_e2022_d_n3;
        locals.var_cgd_dn5 = assign1570_e2022_d_n5;
        locals.var_cgd_dn7 = assign1570_e2022_d_n7;
        locals.var_cgd_dn8 = assign1570_e2022_d_n8;
        locals.var_cgd_dn11 = assign1570_e2022_d_n11;
        locals.var_cgd_rv = 0.0;

        let assign1580_e2025: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1580_e2025;
        locals.var_guard16_rv = 0.0;

        let assign1630_e2040: f64 = if p.p42 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard21 = assign1630_e2040;
        locals.var_guard21_rv = 0.0;

        let assign1640_e2043: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign1640_e2043;
        locals.var_guard22_rv = 0.0;

        let assign1660_e2053: f64 = if ((p.p43 > 0.0) || (p.p44 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard24 = assign1660_e2053;
        locals.var_guard24_rv = 0.0;

        let assign1670_e2056: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign1670_e2056;
        locals.var_guard25_rv = 0.0;

        let assign1680_e2059: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign1680_e2059;
        locals.var_guard26_rv = 0.0;

        let assign1690_e2062: f64 = if p.p7 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard27 = assign1690_e2062;
        locals.var_guard27_rv = 0.0;

        let (assign1790_e2206, assign1790_e2206_d_n11,) = {
    if (((locals.var_guard27 != 0.0) && (locals.var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let assign1790_e2193: f64 = (4.0 * 1.3806503e-23);
        let assign1790_e2195: f64 = (assign1790_e2193 * locals.var_t);
        let assign1790_e2197: f64 = (assign1790_e2195 * p.p73);
        let assign1790_e2199: f64 = (assign1790_e2197 * locals.var_cgs0_t);
        let assign1790_e2202: f64 = (p.p72 * p.p71);
        let assign1790_e2203: f64 = (assign1790_e2202).sqrt();
        let assign1790_e2204: f64 = (assign1790_e2199 * assign1790_e2203);
        (assign1790_e2204, (((((assign1790_e2193 * locals.var_t_dn11) * p.p73) * locals.var_cgs0_t) + (assign1790_e2197 * locals.var_cgs0_t_dn11)) * assign1790_e2203),)
    } else {
        (locals.var_k, locals.var_k_dn11,)
    }
};
        locals.var_k = assign1790_e2206;
        locals.var_k_dn11 = assign1790_e2206_d_n11;
        locals.var_k_rv = 0.0;

        let (assign1820_e2243, assign1820_e2243_d_n11,) = {
    if (((locals.var_guard27 != 0.0) && (locals.var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let assign1820_e2241: f64 = (locals.var_k * 3.141592653589793);
        (assign1820_e2241, (locals.var_k_dn11 * 3.141592653589793),)
    } else {
        (locals.var_ci, locals.var_ci_dn11,)
    }
};
        locals.var_ci = assign1820_e2243;
        locals.var_ci_dn11 = assign1820_e2243_d_n11;
        locals.var_ci_rv = 0.0;

        let assign1860_e2271: f64 = if ((p.p1 != 0.0) && (p.p57 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard43 = assign1860_e2271;
        locals.var_guard43_rv = 0.0;

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi5 = ctx.branch_current(branches[5]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi14 = ctx.branch_current(branches[14]);
        let eq3_e99: f64 = (p.p51 / 3.0);
        let eq3_e101: f64 = (eq3_e99 * bi0);
        let eq3_e102: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq3_e101);
        let eq3_value: f64 = eq3_e102;
        stamper.stamp_potential_branch1_local(
            0,
            eq3_value,
            0,
            (eq3_e99 * ddt_scale),
        );
        let (eq7_e110, eq7_e110_d_n3, eq7_e110_d_n5, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n11,) = {
    if (locals.var_guard16 != 0.0) {
        let eq7_e108: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qgd);
        (eq7_e108, (locals.var_qgd_dn3 * ddt_scale), (locals.var_qgd_dn5 * ddt_scale), (locals.var_qgd_dn7 * ddt_scale), (locals.var_qgd_dn8 * ddt_scale), (locals.var_qgd_dn11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e110;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq7_value),
            [3, 5, 7, 8, 11],
            [multiplicity * (eq7_e110_d_n3), multiplicity * (eq7_e110_d_n5), multiplicity * (eq7_e110_d_n7), multiplicity * (eq7_e110_d_n8), multiplicity * (eq7_e110_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq8_e115, eq8_e115_d_n3, eq8_e115_d_n5, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n11,) = {
    if (locals.var_guard16 != 0.0) {
        let eq8_e113: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qgs);
        (eq8_e113, (locals.var_qgs_dn3 * ddt_scale), (locals.var_qgs_dn5 * ddt_scale), (locals.var_qgs_dn7 * ddt_scale), (locals.var_qgs_dn8 * ddt_scale), (locals.var_qgs_dn11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e115;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq8_value),
            [3, 5, 7, 8, 11],
            [multiplicity * (eq8_e115_d_n3), multiplicity * (eq8_e115_d_n5), multiplicity * (eq8_e115_d_n7), multiplicity * (eq8_e115_d_n8), multiplicity * (eq8_e115_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq9_e123, eq9_e123_d_n3, eq9_e123_d_n5, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n11,) = {
    if (locals.var_guard16 == 0.0) {
        let eq9_e120: f64 = (locals.var_cgd * locals.var_vgdc);
        let eq9_e120_d_n3: f64 = ((locals.var_cgd_dn3 * locals.var_vgdc) + (locals.var_cgd * locals.var_vgdc_dn3));
        let eq9_e120_d_n5: f64 = (locals.var_cgd_dn5 * locals.var_vgdc);
        let eq9_e120_d_n7: f64 = ((locals.var_cgd_dn7 * locals.var_vgdc) + (locals.var_cgd * locals.var_vgdc_dn7));
        let eq9_e120_d_n8: f64 = (locals.var_cgd_dn8 * locals.var_vgdc);
        let eq9_e120_d_n11: f64 = (locals.var_cgd_dn11 * locals.var_vgdc);
        let eq9_e121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq9_e120);
        (eq9_e121, (eq9_e120_d_n3 * ddt_scale), (eq9_e120_d_n5 * ddt_scale), (eq9_e120_d_n7 * ddt_scale), (eq9_e120_d_n8 * ddt_scale), (eq9_e120_d_n11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e123;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq9_value),
            [3, 5, 7, 8, 11],
            [multiplicity * (eq9_e123_d_n3), multiplicity * (eq9_e123_d_n5), multiplicity * (eq9_e123_d_n7), multiplicity * (eq9_e123_d_n8), multiplicity * (eq9_e123_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq10_e131, eq10_e131_d_n3, eq10_e131_d_n5, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n11,) = {
    if (locals.var_guard16 == 0.0) {
        let eq10_e128: f64 = (locals.var_cgs * locals.var_vgsc);
        let eq10_e128_d_n3: f64 = (locals.var_cgs_dn3 * locals.var_vgsc);
        let eq10_e128_d_n5: f64 = ((locals.var_cgs_dn5 * locals.var_vgsc) + (locals.var_cgs * locals.var_vgsc_dn5));
        let eq10_e128_d_n7: f64 = (locals.var_cgs_dn7 * locals.var_vgsc);
        let eq10_e128_d_n8: f64 = ((locals.var_cgs_dn8 * locals.var_vgsc) + (locals.var_cgs * locals.var_vgsc_dn8));
        let eq10_e128_d_n11: f64 = (locals.var_cgs_dn11 * locals.var_vgsc);
        let eq10_e129: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq10_e128);
        (eq10_e129, (eq10_e128_d_n3 * ddt_scale), (eq10_e128_d_n5 * ddt_scale), (eq10_e128_d_n7 * ddt_scale), (eq10_e128_d_n8 * ddt_scale), (eq10_e128_d_n11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e131;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq10_value),
            [3, 5, 7, 8, 11],
            [multiplicity * (eq10_e131_d_n3), multiplicity * (eq10_e131_d_n5), multiplicity * (eq10_e131_d_n7), multiplicity * (eq10_e131_d_n8), multiplicity * (eq10_e131_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq24_e211, eq24_e211_d_b5,) = {
    if (locals.var_guard21 != 0.0) {
        let eq24_e209: f64 = (bi5 * p.p42);
        (eq24_e209, p.p42,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e211;
        stamper.stamp_potential_branch1_local(
            5,
            eq24_value,
            5,
            eq24_e211_d_b5,
        );
        let (eq25_e218, eq25_e218_d_b5,) = {
    if (locals.var_guard21 != 0.0) {
        let eq25_e215: f64 = (p.p50 * bi5);
        let eq25_e216: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq25_e215);
        (eq25_e216, (p.p50 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e218;
        stamper.stamp_potential_branch1_local(
            6,
            eq25_value,
            5,
            eq25_e218_d_b5,
        );
        let (eq27_e242, eq27_e242_d_b5,) = {
    if ((locals.var_guard21 == 0.0) && (locals.var_guard22 != 0.0)) {
        let eq27_e239: f64 = (p.p50 * bi5);
        let eq27_e240: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq27_e239);
        (eq27_e240, (p.p50 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e242;
        stamper.stamp_potential_branch1_local(
            8,
            eq27_value,
            5,
            eq27_e242_d_b5,
        );
        let (eq29_e256, eq29_e256_d_n3, eq29_e256_d_n4, eq29_e256_d_n5, eq29_e256_d_n8, eq29_e256_d_n11, eq29_e256_d_b10,) = {
    if (locals.var_guard23 != 0.0) {
        let eq29_e254: f64 = (bi10 * locals.var_rs_t);
        let eq29_e254_d_n3: f64 = (bi10 * locals.var_rs_t_dn3);
        let eq29_e254_d_n4: f64 = (bi10 * locals.var_rs_t_dn4);
        let eq29_e254_d_n5: f64 = (bi10 * locals.var_rs_t_dn5);
        let eq29_e254_d_n8: f64 = (bi10 * locals.var_rs_t_dn8);
        let eq29_e254_d_n11: f64 = (bi10 * locals.var_rs_t_dn11);
        (eq29_e254, eq29_e254_d_n3, eq29_e254_d_n4, eq29_e254_d_n5, eq29_e254_d_n8, eq29_e254_d_n11, locals.var_rs_t,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e256;
        stamper.stamp_potential_sparse_local::<5, 1>(
            10,
            eq29_value,
            [3, 4, 5, 8, 11],
            [eq29_e256_d_n3, eq29_e256_d_n4, eq29_e256_d_n5, eq29_e256_d_n8, eq29_e256_d_n11],
            [10],
            [eq29_e256_d_b10],
        );
        let eq32_e278: f64 = (p.p49 * bi13);
        let eq32_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq32_e278);
        let eq32_value: f64 = eq32_e279;
        stamper.stamp_potential_branch1_local(
            13,
            eq32_value,
            13,
            (p.p49 * ddt_scale),
        );
        let (eq33_e285, eq33_e285_d_n3, eq33_e285_d_n4, eq33_e285_d_n5, eq33_e285_d_n8, eq33_e285_d_n11, eq33_e285_d_b14,) = {
    if (locals.var_guard24 != 0.0) {
        let eq33_e283: f64 = (bi14 * locals.var_rd1_t);
        let eq33_e283_d_n3: f64 = (bi14 * locals.var_rd1_t_dn3);
        let eq33_e283_d_n4: f64 = (bi14 * locals.var_rd1_t_dn4);
        let eq33_e283_d_n5: f64 = (bi14 * locals.var_rd1_t_dn5);
        let eq33_e283_d_n8: f64 = (bi14 * locals.var_rd1_t_dn8);
        let eq33_e283_d_n11: f64 = (bi14 * locals.var_rd1_t_dn11);
        (eq33_e283, eq33_e283_d_n3, eq33_e283_d_n4, eq33_e283_d_n5, eq33_e283_d_n8, eq33_e283_d_n11, locals.var_rd1_t,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e285;
        stamper.stamp_potential_sparse_local::<5, 1>(
            14,
            eq33_value,
            [3, 4, 5, 8, 11],
            [eq33_e285_d_n3, eq33_e285_d_n4, eq33_e285_d_n5, eq33_e285_d_n8, eq33_e285_d_n11],
            [14],
            [eq33_e285_d_b14],
        );
        let (eq34_e292, eq34_e292_d_b14,) = {
    if (locals.var_guard24 != 0.0) {
        let eq34_e289: f64 = (p.p48 * bi14);
        let eq34_e290: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq34_e289);
        (eq34_e290, (p.p48 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e292;
        stamper.stamp_potential_branch1_local(
            15,
            eq34_value,
            14,
            eq34_e292_d_b14,
        );
        let (eq36_e316, eq36_e316_d_b14,) = {
    if ((locals.var_guard24 == 0.0) && (locals.var_guard25 != 0.0)) {
        let eq36_e313: f64 = (p.p48 * bi14);
        let eq36_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq36_e313);
        (eq36_e314, (p.p48 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e316;
        stamper.stamp_potential_branch1_local(
            17,
            eq36_value,
            14,
            eq36_e316_d_b14,
        );
        let (eq46_e420, eq46_e420_d_n11, eq46_e420_d_n14,) = {
    if (((locals.var_guard27 != 0.0) && (locals.var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-locals.var_ci);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = ((-locals.var_ci_dn11) * (nv14 - 0.0));
        let eq46_e418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq46_e417);
        (eq46_e418, (eq46_e417_d_n11 * ddt_scale), (eq46_e415 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e420;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (eq46_value),
            11,
            multiplicity * (eq46_e420_d_n11),
            14,
            multiplicity * (eq46_e420_d_n14),
        );
        let (eq57_e532, eq57_e532_d_n11,) = {
    if (locals.var_guard43 != 0.0) {
        let eq57_e529: f64 = (p.p58 * (nv11 - 0.0));
        let eq57_e530: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq57_e529);
        (eq57_e530, (p.p58 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e532;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (eq57_value),
            11,
            multiplicity * (eq57_e532_d_n11),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi5 = ctx.branch_current(branches[5]);
        let bi13 = ctx.branch_current(branches[13]);
        let bi14 = ctx.branch_current(branches[14]);
        let eq3_e99: f64 = (p.p51 / 3.0);
        let eq3_e101: f64 = (eq3_e99 * bi0);
        let eq3_e102_q: f64 = eq3_e101;
        stamper.stamp_potential_reactive_branch1(
            branches[0],
            branches[0],
            eq3_e99,
        );
        let (eq7_e110, eq7_e110_d_n3, eq7_e110_d_n5, eq7_e110_d_n7, eq7_e110_d_n8, eq7_e110_d_n11, eq7_e110_q,) = {
    if (locals.var_guard16 != 0.0) {
        let eq7_e108_q: f64 = locals.var_qgd;
        (locals.var_qgd, locals.var_qgd_dn3, locals.var_qgd_dn5, locals.var_qgd_dn7, locals.var_qgd_dn8, locals.var_qgd_dn11, eq7_e108_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 16] = [0.0, 0.0, 0.0, eq7_e110_d_n3, 0.0, eq7_e110_d_n5, 0.0, eq7_e110_d_n7, eq7_e110_d_n8, 0.0, 0.0, eq7_e110_d_n11, 0.0, 0.0, 0.0, 0.0];
        let eq7_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e115, eq8_e115_d_n3, eq8_e115_d_n5, eq8_e115_d_n7, eq8_e115_d_n8, eq8_e115_d_n11, eq8_e115_q,) = {
    if (locals.var_guard16 != 0.0) {
        let eq8_e113_q: f64 = locals.var_qgs;
        (locals.var_qgs, locals.var_qgs_dn3, locals.var_qgs_dn5, locals.var_qgs_dn7, locals.var_qgs_dn8, locals.var_qgs_dn11, eq8_e113_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 16] = [0.0, 0.0, 0.0, eq8_e115_d_n3, 0.0, eq8_e115_d_n5, 0.0, eq8_e115_d_n7, eq8_e115_d_n8, 0.0, 0.0, eq8_e115_d_n11, 0.0, 0.0, 0.0, 0.0];
        let eq8_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e123, eq9_e123_d_n3, eq9_e123_d_n5, eq9_e123_d_n7, eq9_e123_d_n8, eq9_e123_d_n11, eq9_e123_q,) = {
    if (locals.var_guard16 == 0.0) {
        let eq9_e120: f64 = (locals.var_cgd * locals.var_vgdc);
        let eq9_e120_d_n3: f64 = ((locals.var_cgd_dn3 * locals.var_vgdc) + (locals.var_cgd * locals.var_vgdc_dn3));
        let eq9_e120_d_n5: f64 = (locals.var_cgd_dn5 * locals.var_vgdc);
        let eq9_e120_d_n7: f64 = ((locals.var_cgd_dn7 * locals.var_vgdc) + (locals.var_cgd * locals.var_vgdc_dn7));
        let eq9_e120_d_n8: f64 = (locals.var_cgd_dn8 * locals.var_vgdc);
        let eq9_e120_d_n11: f64 = (locals.var_cgd_dn11 * locals.var_vgdc);
        let eq9_e121_q: f64 = eq9_e120;
        (eq9_e120, eq9_e120_d_n3, eq9_e120_d_n5, eq9_e120_d_n7, eq9_e120_d_n8, eq9_e120_d_n11, eq9_e121_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_reactive_node_derivatives: [f64; 16] = [0.0, 0.0, 0.0, eq9_e123_d_n3, 0.0, eq9_e123_d_n5, 0.0, eq9_e123_d_n7, eq9_e123_d_n8, 0.0, 0.0, eq9_e123_d_n11, 0.0, 0.0, 0.0, 0.0];
        let eq9_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e131, eq10_e131_d_n3, eq10_e131_d_n5, eq10_e131_d_n7, eq10_e131_d_n8, eq10_e131_d_n11, eq10_e131_q,) = {
    if (locals.var_guard16 == 0.0) {
        let eq10_e128: f64 = (locals.var_cgs * locals.var_vgsc);
        let eq10_e128_d_n3: f64 = (locals.var_cgs_dn3 * locals.var_vgsc);
        let eq10_e128_d_n5: f64 = ((locals.var_cgs_dn5 * locals.var_vgsc) + (locals.var_cgs * locals.var_vgsc_dn5));
        let eq10_e128_d_n7: f64 = (locals.var_cgs_dn7 * locals.var_vgsc);
        let eq10_e128_d_n8: f64 = ((locals.var_cgs_dn8 * locals.var_vgsc) + (locals.var_cgs * locals.var_vgsc_dn8));
        let eq10_e128_d_n11: f64 = (locals.var_cgs_dn11 * locals.var_vgsc);
        let eq10_e129_q: f64 = eq10_e128;
        (eq10_e128, eq10_e128_d_n3, eq10_e128_d_n5, eq10_e128_d_n7, eq10_e128_d_n8, eq10_e128_d_n11, eq10_e129_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 16] = [0.0, 0.0, 0.0, eq10_e131_d_n3, 0.0, eq10_e131_d_n5, 0.0, eq10_e131_d_n7, eq10_e131_d_n8, 0.0, 0.0, eq10_e131_d_n11, 0.0, 0.0, 0.0, 0.0];
        let eq10_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq25_e218, eq25_e218_d_b5, eq25_e218_q,) = {
    if (locals.var_guard21 != 0.0) {
        let eq25_e215: f64 = (p.p50 * bi5);
        let eq25_e216_q: f64 = eq25_e215;
        (eq25_e215, p.p50, eq25_e216_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[6],
            branches[5],
            eq25_e218_d_b5,
        );
        let (eq27_e242, eq27_e242_d_b5, eq27_e242_q,) = {
    if ((locals.var_guard21 == 0.0) && (locals.var_guard22 != 0.0)) {
        let eq27_e239: f64 = (p.p50 * bi5);
        let eq27_e240_q: f64 = eq27_e239;
        (eq27_e239, p.p50, eq27_e240_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[8],
            branches[5],
            eq27_e242_d_b5,
        );
        let eq32_e278: f64 = (p.p49 * bi13);
        let eq32_e279_q: f64 = eq32_e278;
        stamper.stamp_potential_reactive_branch1(
            branches[13],
            branches[13],
            p.p49,
        );
        let (eq34_e292, eq34_e292_d_b14, eq34_e292_q,) = {
    if (locals.var_guard24 != 0.0) {
        let eq34_e289: f64 = (p.p48 * bi14);
        let eq34_e290_q: f64 = eq34_e289;
        (eq34_e289, p.p48, eq34_e290_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[15],
            branches[14],
            eq34_e292_d_b14,
        );
        let (eq36_e316, eq36_e316_d_b14, eq36_e316_q,) = {
    if ((locals.var_guard24 == 0.0) && (locals.var_guard25 != 0.0)) {
        let eq36_e313: f64 = (p.p48 * bi14);
        let eq36_e314_q: f64 = eq36_e313;
        (eq36_e313, p.p48, eq36_e314_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[17],
            branches[14],
            eq36_e316_d_b14,
        );
        let (eq46_e420, eq46_e420_d_n11, eq46_e420_d_n14, eq46_e420_q,) = {
    if (((locals.var_guard27 != 0.0) && (locals.var_guard26 == 0.0)) && (p.p0 != 0.0)) {
        let eq46_e415: f64 = (-locals.var_ci);
        let eq46_e417: f64 = (eq46_e415 * (nv14 - 0.0));
        let eq46_e417_d_n11: f64 = ((-locals.var_ci_dn11) * (nv14 - 0.0));
        let eq46_e418_q: f64 = eq46_e417;
        (eq46_e417, eq46_e417_d_n11, eq46_e415, eq46_e418_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[11],
            multiplicity * (eq46_e420_d_n11),
            nodes[14],
            multiplicity * (eq46_e420_d_n14),
        );
        let (eq57_e532, eq57_e532_d_n11, eq57_e532_q,) = {
    if (locals.var_guard43 != 0.0) {
        let eq57_e529: f64 = (p.p58 * (nv11 - 0.0));
        let eq57_e530_q: f64 = eq57_e529;
        (eq57_e529, p.p58, eq57_e530_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[11]),
            None,
            nodes[11],
            multiplicity * (eq57_e532_d_n11),
        );
    }
}
