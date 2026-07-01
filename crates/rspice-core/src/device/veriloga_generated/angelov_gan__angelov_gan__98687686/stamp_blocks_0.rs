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
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        locals.var_vgsdel = (nv12 - nv8);
        locals.var_vgsdel_dn8 = -1.0;
        locals.var_vgsdel_dn12 = 1.0;

        locals.var_vgd = (nv10 - nv5);
        locals.var_vgd_dn5 = -1.0;
        locals.var_vgd_dn10 = 1.0;

        let assign20_e573: f64 = (-locals.var_vgd);
        locals.var_vdg = assign20_e573;
        locals.var_vdg_dn5 = (-locals.var_vgd_dn5);
        locals.var_vdg_dn10 = (-locals.var_vgd_dn10);

        locals.var_vds = (nv5 - nv8);
        locals.var_vds_dn5 = 1.0;
        locals.var_vds_dn8 = -1.0;

        locals.var_vgsc = (nv11 - nv8);
        locals.var_vgsc_dn8 = -1.0;
        locals.var_vgsc_dn11 = 1.0;

        locals.var_vgdc = locals.var_vgd;
        locals.var_vgdc_dn5 = locals.var_vgd_dn5;
        locals.var_vgdc_dn10 = locals.var_vgd_dn10;

        locals.var_vrf = (nv4 - nv8);
        locals.var_vrf_dn4 = 1.0;
        locals.var_vrf_dn8 = -1.0;

        locals.var_qgd = 0.0;
        locals.var_qgd_dn3 = 0.0;
        locals.var_qgd_dn5 = 0.0;
        locals.var_qgd_dn8 = 0.0;
        locals.var_qgd_dn10 = 0.0;
        locals.var_qgd_dn11 = 0.0;

        locals.var_qgs = 0.0;
        locals.var_qgs_dn3 = 0.0;
        locals.var_qgs_dn5 = 0.0;
        locals.var_qgs_dn8 = 0.0;
        locals.var_qgs_dn10 = 0.0;
        locals.var_qgs_dn11 = 0.0;

        locals.var_cgd = 0.0;
        locals.var_cgd_dn3 = 0.0;
        locals.var_cgd_dn5 = 0.0;
        locals.var_cgd_dn8 = 0.0;
        locals.var_cgd_dn10 = 0.0;
        locals.var_cgd_dn11 = 0.0;

        locals.var_cgs = 0.0;
        locals.var_cgs_dn3 = 0.0;
        locals.var_cgs_dn5 = 0.0;
        locals.var_cgs_dn8 = 0.0;
        locals.var_cgs_dn10 = 0.0;
        locals.var_cgs_dn11 = 0.0;

        let assign150_e587: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard1 = assign150_e587;

        let (assign160_e593, assign160_e593_d_n3,) = {
    if (locals.var_guard1 != 0.0) {
        let assign160_e591: f64 = (p.p3 + 273.15);
        (assign160_e591, 0.0,)
    } else {
        (locals.var_t, locals.var_t_dn3,)
    }
};
        locals.var_t = assign160_e593;
        locals.var_t_dn3 = assign160_e593_d_n3;

        let (assign170_e600, assign170_e600_d_n3,) = {
    if (locals.var_guard1 == 0.0) {
        let assign170_e596: f64 = ctx_temp;
        let assign170_e598: f64 = (assign170_e596 + p.p2);
        (assign170_e598, 0.0,)
    } else {
        (locals.var_t, locals.var_t_dn3,)
    }
};
        locals.var_t = assign170_e600;
        locals.var_t_dn3 = assign170_e600_d_n3;

        let assign180_e602: f64 = if param_given[100] { 1.0 } else { 0.0 };
        locals.var_guard2 = assign180_e602;

        let (assign190_e608,) = {
    if (locals.var_guard2 != 0.0) {
        let assign190_e606: f64 = (p.p100 + 273.15);
        (assign190_e606,)
    } else {
        (locals.var_t_nom,)
    }
};
        locals.var_t_nom = assign190_e608;

        let (assign200_e615,) = {
    if (locals.var_guard2 == 0.0) {
        let assign200_e613: f64 = (27.0 + 273.15);
        (assign200_e613,)
    } else {
        (locals.var_t_nom,)
    }
};
        locals.var_t_nom = assign200_e615;

        let (assign210_e622, assign210_e622_d_n3,) = {
    if (p.p1 != 0.0) {
        let assign210_e619: f64 = ((nv3 - 0.0)).abs();
        let assign210_e620: f64 = (locals.var_t + assign210_e619);
        (assign210_e620, (locals.var_t_dn3 + if (nv3 - 0.0) >= 0.0 { 1.0 } else { (-1.0) }),)
    } else {
        (locals.var_t, locals.var_t_dn3,)
    }
};
        locals.var_t = assign210_e622;
        locals.var_t_dn3 = assign210_e622_d_n3;

        let assign220_e624: f64 = (locals.var_t * THERMAL_VOLTAGE_PER_K);
        locals.var_vth = assign220_e624;
        locals.var_vth_dn3 = (locals.var_t_dn3 * THERMAL_VOLTAGE_PER_K);

        let assign230_e627: f64 = (locals.var_t - locals.var_t_nom);
        let assign230_e628: f64 = (assign230_e627).abs();
        locals.var_delta_t = assign230_e628;
        locals.var_delta_t_dn3 = if assign230_e627 >= 0.0 { locals.var_t_dn3 } else { (-locals.var_t_dn3) };

        let assign240_e635: f64 = if ((locals.var_delta_t > 0.0) || (p.p66 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard3 = assign240_e635;

        let (assign280_e679, assign280_e679_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign280_e674: f64 = (locals.var_delta_t).abs();
        let assign280_e675: f64 = (p.p72 * assign280_e674);
        let assign280_e676: f64 = (1.0 + assign280_e675);
        let assign280_e677: f64 = (p.p26 * assign280_e676);
        (assign280_e677, (p.p26 * (p.p72 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })),)
    } else {
        (locals.var_cgs0_t, locals.var_cgs0_t_dn3,)
    }
};
        locals.var_cgs0_t = assign280_e679;
        locals.var_cgs0_t_dn3 = assign280_e679_d_n3;

        let (assign290_e690, assign290_e690_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign290_e685: f64 = (locals.var_delta_t).abs();
        let assign290_e686: f64 = (p.p73 * assign290_e685);
        let assign290_e687: f64 = (1.0 + assign290_e686);
        let assign290_e688: f64 = (p.p29 * assign290_e687);
        (assign290_e688, (p.p29 * (p.p73 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })),)
    } else {
        (locals.var_cgd0_t, locals.var_cgd0_t_dn3,)
    }
};
        locals.var_cgd0_t = assign290_e690;
        locals.var_cgd0_t_dn3 = assign290_e690_d_n3;

        let (assign300_e701, assign300_e701_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign300_e696: f64 = (locals.var_delta_t).abs();
        let assign300_e697: f64 = (p.p74 * assign300_e696);
        let assign300_e698: f64 = (1.0 + assign300_e697);
        let assign300_e699: f64 = (p.p58 * assign300_e698);
        (assign300_e699, (p.p58 * (p.p74 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })),)
    } else {
        (locals.var_rc_t, locals.var_rc_t_dn3,)
    }
};
        locals.var_rc_t = assign300_e701;
        locals.var_rc_t_dn3 = assign300_e701_d_n3;

        let (assign320_e720, assign320_e720_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign320_e717: f64 = (p.p78 * locals.var_delta_t);
        let assign320_e718: f64 = (p.p9 + assign320_e717);
        (assign320_e718, (p.p78 * locals.var_delta_t_dn3),)
    } else {
        (locals.var_vpks_t, locals.var_vpks_t_dn3,)
    }
};
        locals.var_vpks_t = assign320_e720;
        locals.var_vpks_t_dn3 = assign320_e720_d_n3;

        let (assign330_e730, assign330_e730_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign330_e726: f64 = (p.p71 * locals.var_delta_t);
        let assign330_e727: f64 = (1.0 + assign330_e726);
        let assign330_e728: f64 = (p.p30 * assign330_e727);
        (assign330_e728, (p.p30 * (p.p71 * locals.var_delta_t_dn3)),)
    } else {
        (locals.var_p10_t, locals.var_p10_t_dn3,)
    }
};
        locals.var_p10_t = assign330_e730;
        locals.var_p10_t_dn3 = assign330_e730_d_n3;

        let (assign340_e740, assign340_e740_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign340_e736: f64 = (p.p71 * locals.var_delta_t);
        let assign340_e737: f64 = (1.0 + assign340_e736);
        let assign340_e738: f64 = (p.p36 * assign340_e737);
        (assign340_e738, (p.p36 * (p.p71 * locals.var_delta_t_dn3)),)
    } else {
        (locals.var_p40_t, locals.var_p40_t_dn3,)
    }
};
        locals.var_p40_t = assign340_e740;
        locals.var_p40_t_dn3 = assign340_e740_d_n3;

        let (assign350_e748, assign350_e748_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign350_e745: f64 = (p.p79 * locals.var_delta_t);
        let assign350_e746: f64 = (p.p45 + assign350_e745);
        (assign350_e746, (p.p79 * locals.var_delta_t_dn3),)
    } else {
        (locals.var_vjg_t, locals.var_vjg_t_dn3,)
    }
};
        locals.var_vjg_t = assign350_e748;
        locals.var_vjg_t_dn3 = assign350_e748_d_n3;

        let (assign360_e756, assign360_e756_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign360_e753: f64 = (p.p81 * locals.var_delta_t);
        let assign360_e754: f64 = (p.p21 + assign360_e753);
        (assign360_e754, (p.p81 * locals.var_delta_t_dn3),)
    } else {
        (locals.var_vtr_t, locals.var_vtr_t_dn3,)
    }
};
        locals.var_vtr_t = assign360_e756;
        locals.var_vtr_t_dn3 = assign360_e756_d_n3;

        let assign370_e767: f64 = if (((p.p4 == 1.0) || (p.p4 == 4.0)) && (p.p6 == 4.0)) { 1.0 } else { 0.0 };
        locals.var_guard4 = assign370_e767;

        let (assign390_e795, assign390_e795_d_n3,) = {
    if ((locals.var_guard3 != 0.0) && (locals.var_guard4 != 0.0)) {
        let assign390_e790: f64 = (locals.var_delta_t * locals.var_delta_t);
        let assign390_e791: f64 = (p.p75 * assign390_e790);
        let assign390_e792: f64 = (1.0 + assign390_e791);
        let assign390_e793: f64 = (p.p63 * assign390_e792);
        (assign390_e793, (p.p63 * (p.p75 * ((locals.var_delta_t_dn3 * locals.var_delta_t) + (locals.var_delta_t * locals.var_delta_t_dn3)))),)
    } else {
        (locals.var_cdel_t, locals.var_cdel_t_dn3,)
    }
};
        locals.var_cdel_t = assign390_e795;
        locals.var_cdel_t_dn3 = assign390_e795_d_n3;

        let (assign410_e823, assign410_e823_d_n3,) = {
    if ((locals.var_guard3 != 0.0) && (locals.var_guard4 == 0.0)) {
        let assign410_e818: f64 = (locals.var_delta_t).abs();
        let assign410_e819: f64 = (p.p75 * assign410_e818);
        let assign410_e820: f64 = (1.0 + assign410_e819);
        let assign410_e821: f64 = (p.p63 * assign410_e820);
        (assign410_e821, (p.p63 * (p.p75 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })),)
    } else {
        (locals.var_cdel_t, locals.var_cdel_t_dn3,)
    }
};
        locals.var_cdel_t = assign410_e823;
        locals.var_cdel_t_dn3 = assign410_e823_d_n3;

        let (assign440_e838, assign440_e838_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p26, 0.0,)
    } else {
        (locals.var_cgs0_t, locals.var_cgs0_t_dn3,)
    }
};
        locals.var_cgs0_t = assign440_e838;
        locals.var_cgs0_t_dn3 = assign440_e838_d_n3;

        let (assign450_e843, assign450_e843_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p29, 0.0,)
    } else {
        (locals.var_cgd0_t, locals.var_cgd0_t_dn3,)
    }
};
        locals.var_cgd0_t = assign450_e843;
        locals.var_cgd0_t_dn3 = assign450_e843_d_n3;

        let (assign460_e848, assign460_e848_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p58, 0.0,)
    } else {
        (locals.var_rc_t, locals.var_rc_t_dn3,)
    }
};
        locals.var_rc_t = assign460_e848;
        locals.var_rc_t_dn3 = assign460_e848_d_n3;

        let (assign490_e863, assign490_e863_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p63, 0.0,)
    } else {
        (locals.var_cdel_t, locals.var_cdel_t_dn3,)
    }
};
        locals.var_cdel_t = assign490_e863;
        locals.var_cdel_t_dn3 = assign490_e863_d_n3;

        let (assign500_e868, assign500_e868_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p9, 0.0,)
    } else {
        (locals.var_vpks_t, locals.var_vpks_t_dn3,)
    }
};
        locals.var_vpks_t = assign500_e868;
        locals.var_vpks_t_dn3 = assign500_e868_d_n3;

        let (assign510_e873, assign510_e873_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p30, 0.0,)
    } else {
        (locals.var_p10_t, locals.var_p10_t_dn3,)
    }
};
        locals.var_p10_t = assign510_e873;
        locals.var_p10_t_dn3 = assign510_e873_d_n3;

        let (assign520_e878, assign520_e878_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p36, 0.0,)
    } else {
        (locals.var_p40_t, locals.var_p40_t_dn3,)
    }
};
        locals.var_p40_t = assign520_e878;
        locals.var_p40_t_dn3 = assign520_e878_d_n3;

        let (assign530_e883, assign530_e883_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p45, 0.0,)
    } else {
        (locals.var_vjg_t, locals.var_vjg_t_dn3,)
    }
};
        locals.var_vjg_t = assign530_e883;
        locals.var_vjg_t_dn3 = assign530_e883_d_n3;

        let (assign540_e888, assign540_e888_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p21, 0.0,)
    } else {
        (locals.var_vtr_t, locals.var_vtr_t_dn3,)
    }
};
        locals.var_vtr_t = assign540_e888;
        locals.var_vtr_t_dn3 = assign540_e888_d_n3;

        let assign550_e894: f64 = if ((!param_given[43]) && param_given[44]) { 1.0 } else { 0.0 };
        locals.var_guard5 = assign550_e894;

        let (assign560_e902, assign560_e902_d_n3,) = {
    if (locals.var_guard5 != 0.0) {
        let assign560_e898: f64 = (0.5 / p.p44);
        let assign560_e900: f64 = (assign560_e898 / locals.var_vth);
        (assign560_e900, (-((assign560_e898 * locals.var_vth_dn3) / (locals.var_vth * locals.var_vth))),)
    } else {
        (locals.var_pg_param, locals.var_pg_param_dn3,)
    }
};
        locals.var_pg_param = assign560_e902;
        locals.var_pg_param_dn3 = assign560_e902_d_n3;

        let (assign570_e907, assign570_e907_d_n3,) = {
    if (locals.var_guard5 == 0.0) {
        (p.p43, 0.0,)
    } else {
        (locals.var_pg_param, locals.var_pg_param_dn3,)
    }
};
        locals.var_pg_param = assign570_e907;
        locals.var_pg_param_dn3 = assign570_e907_d_n3;

        let assign580_e910: f64 = (p.p19 * locals.var_vds);
        let assign580_e911: f64 = (assign580_e910).cosh();
        locals.var_t0 = assign580_e911;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = ((assign580_e910).sinh() * (p.p19 * locals.var_vds_dn5));
        locals.var_t0_dn8 = ((assign580_e910).sinh() * (p.p19 * locals.var_vds_dn8));
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_db1 = 0.0;

        let assign590_e914: f64 = (p.p64 * locals.var_vrf);
        locals.var_vbg = assign590_e914;
        locals.var_vbg_dn4 = (p.p64 * locals.var_vrf_dn4);
        locals.var_vbg_dn8 = (p.p64 * locals.var_vrf_dn8);

        let assign600_e921: f64 = (locals.var_t0 * locals.var_t0);
        let assign600_e922: f64 = (1e-12 + assign600_e921);
        let assign600_e923: f64 = (p.p18 / assign600_e922);
        let assign600_e924: f64 = (1.0 + assign600_e923);
        let assign600_e925: f64 = (p.p11 * assign600_e924);
        locals.var_p1m = assign600_e925;
        locals.var_p1m_dn3 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn4 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn5 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn8 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn10 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn12 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_db1 = (p.p11 * (-((p.p18 * ((locals.var_t0_db1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_db1))) / (assign600_e922 * assign600_e922))));

        let assign610_e930: f64 = (locals.var_delta_t).abs();
        let assign610_e931: f64 = (p.p69 * assign610_e930);
        let assign610_e932: f64 = (1.0 + assign610_e931);
        let assign610_e933: f64 = (locals.var_p1m * assign610_e932);
        locals.var_p1_t = assign610_e933;
        locals.var_p1_t_dn3 = ((locals.var_p1m_dn3 * assign610_e932) + (locals.var_p1m * (p.p69 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })));
        locals.var_p1_t_dn4 = (locals.var_p1m_dn4 * assign610_e932);
        locals.var_p1_t_dn5 = (locals.var_p1m_dn5 * assign610_e932);
        locals.var_p1_t_dn8 = (locals.var_p1m_dn8 * assign610_e932);
        locals.var_p1_t_dn10 = (locals.var_p1m_dn10 * assign610_e932);
        locals.var_p1_t_dn12 = (locals.var_p1m_dn12 * assign610_e932);
        locals.var_p1_t_db1 = (locals.var_p1m_db1 * assign610_e932);

        let assign620_e938: f64 = (locals.var_delta_t).abs();
        let assign620_e939: f64 = (p.p70 * assign620_e938);
        let assign620_e940: f64 = (1.0 + assign620_e939);
        let assign620_e941: f64 = (p.p13 * assign620_e940);
        locals.var_p3_t = assign620_e941;
        locals.var_p3_t_dn3 = (p.p13 * (p.p70 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) }));

        let assign630_e944: f64 = (locals.var_vpks_t - p.p10);
        let assign630_e948: f64 = (p.p15 * locals.var_vds);
        let assign630_e949: f64 = (assign630_e948).tanh();
        let assign630_e950: f64 = (p.p10 * assign630_e949);
        let assign630_e951: f64 = (assign630_e944 + assign630_e950);
        let assign630_e953: f64 = (assign630_e951 - locals.var_vbg);
        let assign630_e957: f64 = (locals.var_vdg - locals.var_vtr_t);
        let assign630_e958: f64 = (p.p22 * assign630_e957);
        let assign630_e961: f64 = (locals.var_vdg - locals.var_vtr_t);
        let assign630_e962: f64 = (assign630_e958 * assign630_e961);
        let assign630_e963: f64 = (assign630_e953 - assign630_e962);
        locals.var_vpkm = assign630_e963;
        locals.var_vpkm_dn3 = (locals.var_vpks_t_dn3 - (((p.p22 * (-locals.var_vtr_t_dn3)) * assign630_e961) + (assign630_e958 * (-locals.var_vtr_t_dn3))));
        locals.var_vpkm_dn4 = (-locals.var_vbg_dn4);
        locals.var_vpkm_dn5 = ((p.p10 * ((p.p15 * locals.var_vds_dn5) / ((assign630_e948).cosh() * (assign630_e948).cosh()))) - (((p.p22 * locals.var_vdg_dn5) * assign630_e961) + (assign630_e958 * locals.var_vdg_dn5)));
        locals.var_vpkm_dn8 = ((p.p10 * ((p.p15 * locals.var_vds_dn8) / ((assign630_e948).cosh() * (assign630_e948).cosh()))) - locals.var_vbg_dn8);
        locals.var_vpkm_dn10 = (-(((p.p22 * locals.var_vdg_dn10) * assign630_e961) + (assign630_e958 * locals.var_vdg_dn10)));

        let assign640_e968: f64 = (locals.var_delta_t).abs();
        let assign640_e969: f64 = (p.p78 * assign640_e968);
        let assign640_e970: f64 = (1.0 + assign640_e969);
        let assign640_e971: f64 = (locals.var_vpkm * assign640_e970);
        locals.var_vpkm_t = assign640_e971;
        locals.var_vpkm_t_dn3 = ((locals.var_vpkm_dn3 * assign640_e970) + (locals.var_vpkm * (p.p78 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })));
        locals.var_vpkm_t_dn4 = (locals.var_vpkm_dn4 * assign640_e970);
        locals.var_vpkm_t_dn5 = (locals.var_vpkm_dn5 * assign640_e970);
        locals.var_vpkm_t_dn8 = (locals.var_vpkm_dn8 * assign640_e970);
        locals.var_vpkm_t_dn10 = (locals.var_vpkm_dn10 * assign640_e970);

        let assign650_e974: f64 = (locals.var_vgsdel - locals.var_vpkm_t);
        locals.var_t1 = assign650_e974;
        locals.var_t1_dn3 = (-locals.var_vpkm_t_dn3);
        locals.var_t1_dn4 = (-locals.var_vpkm_t_dn4);
        locals.var_t1_dn5 = (-locals.var_vpkm_t_dn5);
        locals.var_t1_dn8 = (locals.var_vgsdel_dn8 - locals.var_vpkm_t_dn8);
        locals.var_t1_dn10 = (-locals.var_vpkm_t_dn10);
        locals.var_t1_dn12 = locals.var_vgsdel_dn12;
        locals.var_t1_db1 = 0.0;

        let assign660_e977: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign660_e977;
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn12 = ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12));
        locals.var_t2_db1 = ((locals.var_t1_db1 * locals.var_t1) + (locals.var_t1 * locals.var_t1_db1));

        let assign670_e980: f64 = (locals.var_p1_t * locals.var_t1);
        let assign670_e983: f64 = (p.p12 * locals.var_t2);
        let assign670_e984: f64 = (assign670_e980 + assign670_e983);
        let assign670_e987: f64 = (locals.var_p3_t * locals.var_t1);
        let assign670_e989: f64 = (assign670_e987 * locals.var_t2);
        let assign670_e990: f64 = (assign670_e984 + assign670_e989);
        locals.var_psi = assign670_e990;
        locals.var_psi_dn3 = ((((locals.var_p1_t_dn3 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn3)) + (p.p12 * locals.var_t2_dn3)) + ((((locals.var_p3_t_dn3 * locals.var_t1) + (locals.var_p3_t * locals.var_t1_dn3)) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn3)));
        locals.var_psi_dn4 = ((((locals.var_p1_t_dn4 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn4)) + (p.p12 * locals.var_t2_dn4)) + (((locals.var_p3_t * locals.var_t1_dn4) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn4)));
        locals.var_psi_dn5 = ((((locals.var_p1_t_dn5 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn5)) + (p.p12 * locals.var_t2_dn5)) + (((locals.var_p3_t * locals.var_t1_dn5) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn5)));
        locals.var_psi_dn8 = ((((locals.var_p1_t_dn8 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn8)) + (p.p12 * locals.var_t2_dn8)) + (((locals.var_p3_t * locals.var_t1_dn8) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn8)));
        locals.var_psi_dn10 = ((((locals.var_p1_t_dn10 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn10)) + (p.p12 * locals.var_t2_dn10)) + (((locals.var_p3_t * locals.var_t1_dn10) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn10)));
        locals.var_psi_dn12 = ((((locals.var_p1_t_dn12 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn12)) + (p.p12 * locals.var_t2_dn12)) + (((locals.var_p3_t * locals.var_t1_dn12) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn12)));
        locals.var_psi_db1 = ((((locals.var_p1_t_db1 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_db1)) + (p.p12 * locals.var_t2_db1)) + (((locals.var_p3_t * locals.var_t1_db1) * locals.var_t2) + (assign670_e987 * locals.var_t2_db1)));

        let assign680_e993: f64 = (locals.var_psi).tanh();
        let assign680_e994: f64 = (1.0 + assign680_e993);
        locals.var_tanh_psi = assign680_e994;
        locals.var_tanh_psi_dn3 = (locals.var_psi_dn3 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn4 = (locals.var_psi_dn4 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn5 = (locals.var_psi_dn5 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn8 = (locals.var_psi_dn8 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn10 = (locals.var_psi_dn10 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn12 = (locals.var_psi_dn12 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_db1 = (locals.var_psi_db1 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign690_e998: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign690_e1000: f64 = (-locals.var_psi);
        let assign690_e1001: f64 = { let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign690_e1002: f64 = (assign690_e998 - assign690_e1001);
        let assign690_e1003: f64 = (0.5 * assign690_e1002);
        let assign690_e1004: f64 = (assign690_e1003).tanh();
        let assign690_e1005: f64 = (1.0 + assign690_e1004);
        locals.var_tanh_psi1 = assign690_e1005;
        locals.var_tanh_psi1_dn3 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn4 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn5 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn8 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn10 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn10) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn10)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn12 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn12) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn12)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_db1 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_db1) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_db1)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));

        let assign720_e1017: f64 = if p.p4 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign720_e1017;

        let assign730_e1020: f64 = if p.p4 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign730_e1020;

        let assign740_e1023: f64 = if p.p4 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign740_e1023;

        let assign750_e1026: f64 = if p.p4 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign750_e1026;

        let (assign780_e1059, assign780_e1059_d_n3, assign780_e1059_d_n4, assign780_e1059_d_n5, assign780_e1059_d_n8, assign780_e1059_d_n10, assign780_e1059_d_n12, assign780_e1059_d_b1,) = {
    if ((locals.var_guard7 != 0.0) && (locals.var_guard6 == 0.0)) {
        let assign780_e1057: f64 = (locals.var_vgd - locals.var_vpkm_t);
        (assign780_e1057, (-locals.var_vpkm_t_dn3), (-locals.var_vpkm_t_dn4), (locals.var_vgd_dn5 - locals.var_vpkm_t_dn5), (-locals.var_vpkm_t_dn8), (locals.var_vgd_dn10 - locals.var_vpkm_t_dn10), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign780_e1059;
        locals.var_t0_dn3 = assign780_e1059_d_n3;
        locals.var_t0_dn4 = assign780_e1059_d_n4;
        locals.var_t0_dn5 = assign780_e1059_d_n5;
        locals.var_t0_dn8 = assign780_e1059_d_n8;
        locals.var_t0_dn10 = assign780_e1059_d_n10;
        locals.var_t0_dn12 = assign780_e1059_d_n12;
        locals.var_t0_db1 = assign780_e1059_d_b1;

        let (assign790_e1068, assign790_e1068_d_n3, assign790_e1068_d_n4, assign790_e1068_d_n5, assign790_e1068_d_n8, assign790_e1068_d_n10, assign790_e1068_d_n12, assign790_e1068_d_b1,) = {
    if ((locals.var_guard7 != 0.0) && (locals.var_guard6 == 0.0)) {
        let assign790_e1066: f64 = (locals.var_t0 * locals.var_t0);
        (assign790_e1066, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)), ((locals.var_t0_db1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_db1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn12, locals.var_t1_db1,)
    }
};
        locals.var_t1 = assign790_e1068;
        locals.var_t1_dn3 = assign790_e1068_d_n3;
        locals.var_t1_dn4 = assign790_e1068_d_n4;
        locals.var_t1_dn5 = assign790_e1068_d_n5;
        locals.var_t1_dn8 = assign790_e1068_d_n8;
        locals.var_t1_dn10 = assign790_e1068_d_n10;
        locals.var_t1_dn12 = assign790_e1068_d_n12;
        locals.var_t1_db1 = assign790_e1068_d_b1;

        let (assign800_e1077, assign800_e1077_d_n3, assign800_e1077_d_n4, assign800_e1077_d_n5, assign800_e1077_d_n8, assign800_e1077_d_n10, assign800_e1077_d_n12, assign800_e1077_d_b1,) = {
    if ((locals.var_guard7 != 0.0) && (locals.var_guard6 == 0.0)) {
        let assign800_e1075: f64 = (locals.var_t1 * locals.var_t0);
        (assign800_e1075, ((locals.var_t1_dn3 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn3)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn12 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn12)), ((locals.var_t1_db1 * locals.var_t0) + (locals.var_t1 * locals.var_t0_db1)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn12, locals.var_t2_db1,)
    }
};
        locals.var_t2 = assign800_e1077;
        locals.var_t2_dn3 = assign800_e1077_d_n3;
        locals.var_t2_dn4 = assign800_e1077_d_n4;
        locals.var_t2_dn5 = assign800_e1077_d_n5;
        locals.var_t2_dn8 = assign800_e1077_d_n8;
        locals.var_t2_dn10 = assign800_e1077_d_n10;
        locals.var_t2_dn12 = assign800_e1077_d_n12;
        locals.var_t2_db1 = assign800_e1077_d_b1;

        let (assign900_e1216, assign900_e1216_d_n3, assign900_e1216_d_n4, assign900_e1216_d_n5, assign900_e1216_d_n8, assign900_e1216_d_n10, assign900_e1216_d_n12, assign900_e1216_d_b1,) = {
    if ((locals.var_guard8 != 0.0) && (!((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)))) {
        let assign900_e1214: f64 = (locals.var_vgsdel - locals.var_vpkm_t);
        (assign900_e1214, (-locals.var_vpkm_t_dn3), (-locals.var_vpkm_t_dn4), (-locals.var_vpkm_t_dn5), (locals.var_vgsdel_dn8 - locals.var_vpkm_t_dn8), (-locals.var_vpkm_t_dn10), locals.var_vgsdel_dn12, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign900_e1216;
        locals.var_t0_dn3 = assign900_e1216_d_n3;
        locals.var_t0_dn4 = assign900_e1216_d_n4;
        locals.var_t0_dn5 = assign900_e1216_d_n5;
        locals.var_t0_dn8 = assign900_e1216_d_n8;
        locals.var_t0_dn10 = assign900_e1216_d_n10;
        locals.var_t0_dn12 = assign900_e1216_d_n12;
        locals.var_t0_db1 = assign900_e1216_d_b1;

        let (assign910_e1227, assign910_e1227_d_n3, assign910_e1227_d_n4, assign910_e1227_d_n5, assign910_e1227_d_n8, assign910_e1227_d_n10, assign910_e1227_d_n12, assign910_e1227_d_b1,) = {
    if ((locals.var_guard8 != 0.0) && (!((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)))) {
        let assign910_e1225: f64 = (locals.var_t0 * locals.var_t0);
        (assign910_e1225, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)), ((locals.var_t0_db1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_db1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn12, locals.var_t1_db1,)
    }
};
        locals.var_t1 = assign910_e1227;
        locals.var_t1_dn3 = assign910_e1227_d_n3;
        locals.var_t1_dn4 = assign910_e1227_d_n4;
        locals.var_t1_dn5 = assign910_e1227_d_n5;
        locals.var_t1_dn8 = assign910_e1227_d_n8;
        locals.var_t1_dn10 = assign910_e1227_d_n10;
        locals.var_t1_dn12 = assign910_e1227_d_n12;
        locals.var_t1_db1 = assign910_e1227_d_b1;

        let (assign920_e1248, assign920_e1248_d_n3, assign920_e1248_d_n4, assign920_e1248_d_n5, assign920_e1248_d_n8, assign920_e1248_d_n10, assign920_e1248_d_n12, assign920_e1248_d_b1,) = {
    if ((locals.var_guard8 != 0.0) && (!((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)))) {
        let assign920_e1238: f64 = (p.p12 * locals.var_t1);
        let assign920_e1239: f64 = (locals.var_t0 + assign920_e1238);
        let assign920_e1242: f64 = (locals.var_p3_t * locals.var_t1);
        let assign920_e1244: f64 = (assign920_e1242 * locals.var_t0);
        let assign920_e1245: f64 = (assign920_e1239 + assign920_e1244);
        let assign920_e1246: f64 = (locals.var_p1_t * assign920_e1245);
        (assign920_e1246, ((locals.var_p1_t_dn3 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn3 + (p.p12 * locals.var_t1_dn3)) + ((((locals.var_p3_t_dn3 * locals.var_t1) + (locals.var_p3_t * locals.var_t1_dn3)) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn3))))), ((locals.var_p1_t_dn4 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn4 + (p.p12 * locals.var_t1_dn4)) + (((locals.var_p3_t * locals.var_t1_dn4) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn4))))), ((locals.var_p1_t_dn5 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn5 + (p.p12 * locals.var_t1_dn5)) + (((locals.var_p3_t * locals.var_t1_dn5) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn5))))), ((locals.var_p1_t_dn8 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn8 + (p.p12 * locals.var_t1_dn8)) + (((locals.var_p3_t * locals.var_t1_dn8) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn8))))), ((locals.var_p1_t_dn10 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn10 + (p.p12 * locals.var_t1_dn10)) + (((locals.var_p3_t * locals.var_t1_dn10) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn10))))), ((locals.var_p1_t_dn12 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn12 + (p.p12 * locals.var_t1_dn12)) + (((locals.var_p3_t * locals.var_t1_dn12) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn12))))), ((locals.var_p1_t_db1 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_db1 + (p.p12 * locals.var_t1_db1)) + (((locals.var_p3_t * locals.var_t1_db1) * locals.var_t0) + (assign920_e1242 * locals.var_t0_db1))))),)
    } else {
        (locals.var_psi, locals.var_psi_dn3, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn12, locals.var_psi_db1,)
    }
};
        locals.var_psi = assign920_e1248;
        locals.var_psi_dn3 = assign920_e1248_d_n3;
        locals.var_psi_dn4 = assign920_e1248_d_n4;
        locals.var_psi_dn5 = assign920_e1248_d_n5;
        locals.var_psi_dn8 = assign920_e1248_d_n8;
        locals.var_psi_dn10 = assign920_e1248_d_n10;
        locals.var_psi_dn12 = assign920_e1248_d_n12;
        locals.var_psi_db1 = assign920_e1248_d_b1;

        let (assign930_e1267, assign930_e1267_d_n3, assign930_e1267_d_n4, assign930_e1267_d_n5, assign930_e1267_d_n8, assign930_e1267_d_n10, assign930_e1267_d_n12, assign930_e1267_d_b1,) = {
    if ((locals.var_guard8 != 0.0) && (!((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)))) {
        let assign930_e1258: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign930_e1260: f64 = (-locals.var_psi);
        let assign930_e1261: f64 = { let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign930_e1262: f64 = (assign930_e1258 - assign930_e1261);
        let assign930_e1263: f64 = (0.5 * assign930_e1262);
        let assign930_e1264: f64 = (assign930_e1263).tanh();
        let assign930_e1265: f64 = (1.0 + assign930_e1264);
        (assign930_e1265, ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn10) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn10)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn12) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn12)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_db1) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_db1)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())),)
    } else {
        (locals.var_tanh_psi1, locals.var_tanh_psi1_dn3, locals.var_tanh_psi1_dn4, locals.var_tanh_psi1_dn5, locals.var_tanh_psi1_dn8, locals.var_tanh_psi1_dn10, locals.var_tanh_psi1_dn12, locals.var_tanh_psi1_db1,)
    }
};
        locals.var_tanh_psi1 = assign930_e1267;
        locals.var_tanh_psi1_dn3 = assign930_e1267_d_n3;
        locals.var_tanh_psi1_dn4 = assign930_e1267_d_n4;
        locals.var_tanh_psi1_dn5 = assign930_e1267_d_n5;
        locals.var_tanh_psi1_dn8 = assign930_e1267_d_n8;
        locals.var_tanh_psi1_dn10 = assign930_e1267_d_n10;
        locals.var_tanh_psi1_dn12 = assign930_e1267_d_n12;
        locals.var_tanh_psi1_db1 = assign930_e1267_d_b1;

        let (assign980_e1346, assign980_e1346_d_n3, assign980_e1346_d_n4, assign980_e1346_d_n5, assign980_e1346_d_n8, assign980_e1346_d_n10, assign980_e1346_d_n12, assign980_e1346_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign980_e1344: f64 = (locals.var_vgsdel - locals.var_vpkm_t);
        (assign980_e1344, (-locals.var_vpkm_t_dn3), (-locals.var_vpkm_t_dn4), (-locals.var_vpkm_t_dn5), (locals.var_vgsdel_dn8 - locals.var_vpkm_t_dn8), (-locals.var_vpkm_t_dn10), locals.var_vgsdel_dn12, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign980_e1346;
        locals.var_t0_dn3 = assign980_e1346_d_n3;
        locals.var_t0_dn4 = assign980_e1346_d_n4;
        locals.var_t0_dn5 = assign980_e1346_d_n5;
        locals.var_t0_dn8 = assign980_e1346_d_n8;
        locals.var_t0_dn10 = assign980_e1346_d_n10;
        locals.var_t0_dn12 = assign980_e1346_d_n12;
        locals.var_t0_db1 = assign980_e1346_d_b1;

        let (assign990_e1359, assign990_e1359_d_n3, assign990_e1359_d_n4, assign990_e1359_d_n5, assign990_e1359_d_n8, assign990_e1359_d_n10, assign990_e1359_d_n12, assign990_e1359_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign990_e1357: f64 = (locals.var_t0 * locals.var_t0);
        (assign990_e1357, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)), ((locals.var_t0_db1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_db1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn12, locals.var_t1_db1,)
    }
};
        locals.var_t1 = assign990_e1359;
        locals.var_t1_dn3 = assign990_e1359_d_n3;
        locals.var_t1_dn4 = assign990_e1359_d_n4;
        locals.var_t1_dn5 = assign990_e1359_d_n5;
        locals.var_t1_dn8 = assign990_e1359_d_n8;
        locals.var_t1_dn10 = assign990_e1359_d_n10;
        locals.var_t1_dn12 = assign990_e1359_d_n12;
        locals.var_t1_db1 = assign990_e1359_d_b1;

        let (assign1000_e1382, assign1000_e1382_d_n3, assign1000_e1382_d_n4, assign1000_e1382_d_n5, assign1000_e1382_d_n8, assign1000_e1382_d_n10, assign1000_e1382_d_n12, assign1000_e1382_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign1000_e1372: f64 = (p.p12 * locals.var_t1);
        let assign1000_e1373: f64 = (locals.var_t0 + assign1000_e1372);
        let assign1000_e1376: f64 = (locals.var_p3_t * locals.var_t1);
        let assign1000_e1378: f64 = (assign1000_e1376 * locals.var_t0);
        let assign1000_e1379: f64 = (assign1000_e1373 + assign1000_e1378);
        let assign1000_e1380: f64 = (locals.var_p1_t * assign1000_e1379);
        (assign1000_e1380, ((locals.var_p1_t_dn3 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn3 + (p.p12 * locals.var_t1_dn3)) + ((((locals.var_p3_t_dn3 * locals.var_t1) + (locals.var_p3_t * locals.var_t1_dn3)) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn3))))), ((locals.var_p1_t_dn4 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn4 + (p.p12 * locals.var_t1_dn4)) + (((locals.var_p3_t * locals.var_t1_dn4) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn4))))), ((locals.var_p1_t_dn5 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn5 + (p.p12 * locals.var_t1_dn5)) + (((locals.var_p3_t * locals.var_t1_dn5) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn5))))), ((locals.var_p1_t_dn8 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn8 + (p.p12 * locals.var_t1_dn8)) + (((locals.var_p3_t * locals.var_t1_dn8) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn8))))), ((locals.var_p1_t_dn10 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn10 + (p.p12 * locals.var_t1_dn10)) + (((locals.var_p3_t * locals.var_t1_dn10) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn10))))), ((locals.var_p1_t_dn12 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn12 + (p.p12 * locals.var_t1_dn12)) + (((locals.var_p3_t * locals.var_t1_dn12) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn12))))), ((locals.var_p1_t_db1 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_db1 + (p.p12 * locals.var_t1_db1)) + (((locals.var_p3_t * locals.var_t1_db1) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_db1))))),)
    } else {
        (locals.var_psi, locals.var_psi_dn3, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn12, locals.var_psi_db1,)
    }
};
        locals.var_psi = assign1000_e1382;
        locals.var_psi_dn3 = assign1000_e1382_d_n3;
        locals.var_psi_dn4 = assign1000_e1382_d_n4;
        locals.var_psi_dn5 = assign1000_e1382_d_n5;
        locals.var_psi_dn8 = assign1000_e1382_d_n8;
        locals.var_psi_dn10 = assign1000_e1382_d_n10;
        locals.var_psi_dn12 = assign1000_e1382_d_n12;
        locals.var_psi_db1 = assign1000_e1382_d_b1;

        let (assign1010_e1395, assign1010_e1395_d_n3, assign1010_e1395_d_n4, assign1010_e1395_d_n5, assign1010_e1395_d_n8, assign1010_e1395_d_n10, assign1010_e1395_d_n12, assign1010_e1395_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign1010_e1393: f64 = (locals.var_vgd - locals.var_vpkm_t);
        (assign1010_e1393, (-locals.var_vpkm_t_dn3), (-locals.var_vpkm_t_dn4), (locals.var_vgd_dn5 - locals.var_vpkm_t_dn5), (-locals.var_vpkm_t_dn8), (locals.var_vgd_dn10 - locals.var_vpkm_t_dn10), 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn12, locals.var_t2_db1,)
    }
};
        locals.var_t2 = assign1010_e1395;
        locals.var_t2_dn3 = assign1010_e1395_d_n3;
        locals.var_t2_dn4 = assign1010_e1395_d_n4;
        locals.var_t2_dn5 = assign1010_e1395_d_n5;
        locals.var_t2_dn8 = assign1010_e1395_d_n8;
        locals.var_t2_dn10 = assign1010_e1395_d_n10;
        locals.var_t2_dn12 = assign1010_e1395_d_n12;
        locals.var_t2_db1 = assign1010_e1395_d_b1;

        let (assign1040_e1452, assign1040_e1452_d_n3, assign1040_e1452_d_n4, assign1040_e1452_d_n5, assign1040_e1452_d_n8, assign1040_e1452_d_n10, assign1040_e1452_d_n12, assign1040_e1452_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign1040_e1443: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1040_e1445: f64 = (-locals.var_psi);
        let assign1040_e1446: f64 = { let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1040_e1447: f64 = (assign1040_e1443 - assign1040_e1446);
        let assign1040_e1448: f64 = (0.5 * assign1040_e1447);
        let assign1040_e1449: f64 = (assign1040_e1448).tanh();
        let assign1040_e1450: f64 = (1.0 + assign1040_e1449);
        (assign1040_e1450, ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn10) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn10)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn12) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn12)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_db1) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_db1)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())),)
    } else {
        (locals.var_tanh_psi1, locals.var_tanh_psi1_dn3, locals.var_tanh_psi1_dn4, locals.var_tanh_psi1_dn5, locals.var_tanh_psi1_dn8, locals.var_tanh_psi1_dn10, locals.var_tanh_psi1_dn12, locals.var_tanh_psi1_db1,)
    }
};
        locals.var_tanh_psi1 = assign1040_e1452;
        locals.var_tanh_psi1_dn3 = assign1040_e1452_d_n3;
        locals.var_tanh_psi1_dn4 = assign1040_e1452_d_n4;
        locals.var_tanh_psi1_dn5 = assign1040_e1452_d_n5;
        locals.var_tanh_psi1_dn8 = assign1040_e1452_d_n8;
        locals.var_tanh_psi1_dn10 = assign1040_e1452_d_n10;
        locals.var_tanh_psi1_dn12 = assign1040_e1452_d_n12;
        locals.var_tanh_psi1_db1 = assign1040_e1452_d_b1;

        let assign1200_e1748: f64 = if (((p.p4 == 0.0) || (p.p4 == 1.0)) || (p.p4 == 4.0)) { 1.0 } else { 0.0 };
        locals.var_guard11 = assign1200_e1748;

        let (assign1210_e1758, assign1210_e1758_d_n3, assign1210_e1758_d_n4, assign1210_e1758_d_n5, assign1210_e1758_d_n8, assign1210_e1758_d_n10, assign1210_e1758_d_n12, assign1210_e1758_d_b1,) = {
    if (locals.var_guard11 != 0.0) {
        let assign1210_e1754: f64 = (1.0 + locals.var_tanh_psi);
        let assign1210_e1755: f64 = (locals.var_rc_t / assign1210_e1754);
        let assign1210_e1756: f64 = (p.p57 + assign1210_e1755);
        (assign1210_e1756, (((locals.var_rc_t_dn3 * assign1210_e1754) - (locals.var_rc_t * locals.var_tanh_psi_dn3)) / (assign1210_e1754 * assign1210_e1754)), (-((locals.var_rc_t * locals.var_tanh_psi_dn4) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_dn5) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_dn8) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_dn10) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_dn12) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_db1) / (assign1210_e1754 * assign1210_e1754))),)
    } else {
        (locals.var_rc1, locals.var_rc1_dn3, locals.var_rc1_dn4, locals.var_rc1_dn5, locals.var_rc1_dn8, locals.var_rc1_dn10, locals.var_rc1_dn12, locals.var_rc1_db1,)
    }
};
        locals.var_rc1 = assign1210_e1758;
        locals.var_rc1_dn3 = assign1210_e1758_d_n3;
        locals.var_rc1_dn4 = assign1210_e1758_d_n4;
        locals.var_rc1_dn5 = assign1210_e1758_d_n5;
        locals.var_rc1_dn8 = assign1210_e1758_d_n8;
        locals.var_rc1_dn10 = assign1210_e1758_d_n10;
        locals.var_rc1_dn12 = assign1210_e1758_d_n12;
        locals.var_rc1_db1 = assign1210_e1758_d_b1;

        let (assign1220_e1766, assign1220_e1766_d_n3, assign1220_e1766_d_n4, assign1220_e1766_d_n5, assign1220_e1766_d_n8, assign1220_e1766_d_n10, assign1220_e1766_d_n12, assign1220_e1766_d_b1,) = {
    if (locals.var_guard11 != 0.0) {
        let assign1220_e1763: f64 = (p.p48 * locals.var_tanh_psi);
        let assign1220_e1764: f64 = (p.p47 + assign1220_e1763);
        (assign1220_e1764, (p.p48 * locals.var_tanh_psi_dn3), (p.p48 * locals.var_tanh_psi_dn4), (p.p48 * locals.var_tanh_psi_dn5), (p.p48 * locals.var_tanh_psi_dn8), (p.p48 * locals.var_tanh_psi_dn10), (p.p48 * locals.var_tanh_psi_dn12), (p.p48 * locals.var_tanh_psi_db1),)
    } else {
        (locals.var_rd1, locals.var_rd1_dn3, locals.var_rd1_dn4, locals.var_rd1_dn5, locals.var_rd1_dn8, locals.var_rd1_dn10, locals.var_rd1_dn12, locals.var_rd1_db1,)
    }
};
        locals.var_rd1 = assign1220_e1766;
        locals.var_rd1_dn3 = assign1220_e1766_d_n3;
        locals.var_rd1_dn4 = assign1220_e1766_d_n4;
        locals.var_rd1_dn5 = assign1220_e1766_d_n5;
        locals.var_rd1_dn8 = assign1220_e1766_d_n8;
        locals.var_rd1_dn10 = assign1220_e1766_d_n10;
        locals.var_rd1_dn12 = assign1220_e1766_d_n12;
        locals.var_rd1_db1 = assign1220_e1766_d_b1;

        let (assign1230_e1774, assign1230_e1774_d_n3, assign1230_e1774_d_n4, assign1230_e1774_d_n5, assign1230_e1774_d_n8, assign1230_e1774_d_n10, assign1230_e1774_d_n12, assign1230_e1774_d_b1,) = {
    if (locals.var_guard11 != 0.0) {
        let assign1230_e1771: f64 = (p.p48 * locals.var_tanh_psi);
        let assign1230_e1772: f64 = (p.p50 + assign1230_e1771);
        (assign1230_e1772, (p.p48 * locals.var_tanh_psi_dn3), (p.p48 * locals.var_tanh_psi_dn4), (p.p48 * locals.var_tanh_psi_dn5), (p.p48 * locals.var_tanh_psi_dn8), (p.p48 * locals.var_tanh_psi_dn10), (p.p48 * locals.var_tanh_psi_dn12), (p.p48 * locals.var_tanh_psi_db1),)
    } else {
        (locals.var_rs1, locals.var_rs1_dn3, locals.var_rs1_dn4, locals.var_rs1_dn5, locals.var_rs1_dn8, locals.var_rs1_dn10, locals.var_rs1_dn12, locals.var_rs1_db1,)
    }
};
        locals.var_rs1 = assign1230_e1774;
        locals.var_rs1_dn3 = assign1230_e1774_d_n3;
        locals.var_rs1_dn4 = assign1230_e1774_d_n4;
        locals.var_rs1_dn5 = assign1230_e1774_d_n5;
        locals.var_rs1_dn8 = assign1230_e1774_d_n8;
        locals.var_rs1_dn10 = assign1230_e1774_d_n10;
        locals.var_rs1_dn12 = assign1230_e1774_d_n12;
        locals.var_rs1_db1 = assign1230_e1774_d_b1;

        let (assign1240_e1785, assign1240_e1785_d_n3, assign1240_e1785_d_n4, assign1240_e1785_d_n5, assign1240_e1785_d_n8, assign1240_e1785_d_n10, assign1240_e1785_d_n12, assign1240_e1785_d_b1,) = {
    if (locals.var_guard11 == 0.0) {
        let assign1240_e1781: f64 = (1.0 + locals.var_tanh_psi1);
        let assign1240_e1782: f64 = (locals.var_rc_t / assign1240_e1781);
        let assign1240_e1783: f64 = (p.p57 + assign1240_e1782);
        (assign1240_e1783, (((locals.var_rc_t_dn3 * assign1240_e1781) - (locals.var_rc_t * locals.var_tanh_psi1_dn3)) / (assign1240_e1781 * assign1240_e1781)), (-((locals.var_rc_t * locals.var_tanh_psi1_dn4) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_dn5) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_dn8) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_dn10) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_dn12) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_db1) / (assign1240_e1781 * assign1240_e1781))),)
    } else {
        (locals.var_rc1, locals.var_rc1_dn3, locals.var_rc1_dn4, locals.var_rc1_dn5, locals.var_rc1_dn8, locals.var_rc1_dn10, locals.var_rc1_dn12, locals.var_rc1_db1,)
    }
};
        locals.var_rc1 = assign1240_e1785;
        locals.var_rc1_dn3 = assign1240_e1785_d_n3;
        locals.var_rc1_dn4 = assign1240_e1785_d_n4;
        locals.var_rc1_dn5 = assign1240_e1785_d_n5;
        locals.var_rc1_dn8 = assign1240_e1785_d_n8;
        locals.var_rc1_dn10 = assign1240_e1785_d_n10;
        locals.var_rc1_dn12 = assign1240_e1785_d_n12;
        locals.var_rc1_db1 = assign1240_e1785_d_b1;

        let (assign1250_e1794, assign1250_e1794_d_n3, assign1250_e1794_d_n4, assign1250_e1794_d_n5, assign1250_e1794_d_n8, assign1250_e1794_d_n10, assign1250_e1794_d_n12, assign1250_e1794_d_b1,) = {
    if (locals.var_guard11 == 0.0) {
        let assign1250_e1791: f64 = (p.p48 * locals.var_tanh_psi1);
        let assign1250_e1792: f64 = (p.p47 + assign1250_e1791);
        (assign1250_e1792, (p.p48 * locals.var_tanh_psi1_dn3), (p.p48 * locals.var_tanh_psi1_dn4), (p.p48 * locals.var_tanh_psi1_dn5), (p.p48 * locals.var_tanh_psi1_dn8), (p.p48 * locals.var_tanh_psi1_dn10), (p.p48 * locals.var_tanh_psi1_dn12), (p.p48 * locals.var_tanh_psi1_db1),)
    } else {
        (locals.var_rd1, locals.var_rd1_dn3, locals.var_rd1_dn4, locals.var_rd1_dn5, locals.var_rd1_dn8, locals.var_rd1_dn10, locals.var_rd1_dn12, locals.var_rd1_db1,)
    }
};
        locals.var_rd1 = assign1250_e1794;
        locals.var_rd1_dn3 = assign1250_e1794_d_n3;
        locals.var_rd1_dn4 = assign1250_e1794_d_n4;
        locals.var_rd1_dn5 = assign1250_e1794_d_n5;
        locals.var_rd1_dn8 = assign1250_e1794_d_n8;
        locals.var_rd1_dn10 = assign1250_e1794_d_n10;
        locals.var_rd1_dn12 = assign1250_e1794_d_n12;
        locals.var_rd1_db1 = assign1250_e1794_d_b1;

        let (assign1260_e1803, assign1260_e1803_d_n3, assign1260_e1803_d_n4, assign1260_e1803_d_n5, assign1260_e1803_d_n8, assign1260_e1803_d_n10, assign1260_e1803_d_n12, assign1260_e1803_d_b1,) = {
    if (locals.var_guard11 == 0.0) {
        let assign1260_e1800: f64 = (p.p48 * locals.var_tanh_psi1);
        let assign1260_e1801: f64 = (p.p50 + assign1260_e1800);
        (assign1260_e1801, (p.p48 * locals.var_tanh_psi1_dn3), (p.p48 * locals.var_tanh_psi1_dn4), (p.p48 * locals.var_tanh_psi1_dn5), (p.p48 * locals.var_tanh_psi1_dn8), (p.p48 * locals.var_tanh_psi1_dn10), (p.p48 * locals.var_tanh_psi1_dn12), (p.p48 * locals.var_tanh_psi1_db1),)
    } else {
        (locals.var_rs1, locals.var_rs1_dn3, locals.var_rs1_dn4, locals.var_rs1_dn5, locals.var_rs1_dn8, locals.var_rs1_dn10, locals.var_rs1_dn12, locals.var_rs1_db1,)
    }
};
        locals.var_rs1 = assign1260_e1803;
        locals.var_rs1_dn3 = assign1260_e1803_d_n3;
        locals.var_rs1_dn4 = assign1260_e1803_d_n4;
        locals.var_rs1_dn5 = assign1260_e1803_d_n5;
        locals.var_rs1_dn8 = assign1260_e1803_d_n8;
        locals.var_rs1_dn10 = assign1260_e1803_d_n10;
        locals.var_rs1_dn12 = assign1260_e1803_d_n12;
        locals.var_rs1_db1 = assign1260_e1803_d_b1;

        let assign1270_e1808: f64 = (locals.var_delta_t).abs();
        let assign1270_e1809: f64 = (p.p76 * assign1270_e1808);
        let assign1270_e1810: f64 = (1.0 + assign1270_e1809);
        let assign1270_e1811: f64 = (locals.var_rs1 * assign1270_e1810);
        locals.var_rs_t = assign1270_e1811;
        locals.var_rs_t_dn3 = ((locals.var_rs1_dn3 * assign1270_e1810) + (locals.var_rs1 * (p.p76 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })));
        locals.var_rs_t_dn4 = (locals.var_rs1_dn4 * assign1270_e1810);
        locals.var_rs_t_dn5 = (locals.var_rs1_dn5 * assign1270_e1810);
        locals.var_rs_t_dn8 = (locals.var_rs1_dn8 * assign1270_e1810);
        locals.var_rs_t_dn10 = (locals.var_rs1_dn10 * assign1270_e1810);
        locals.var_rs_t_dn12 = (locals.var_rs1_dn12 * assign1270_e1810);
        locals.var_rs_t_db1 = (locals.var_rs1_db1 * assign1270_e1810);

        let assign1280_e1816: f64 = (locals.var_delta_t).abs();
        let assign1280_e1817: f64 = (p.p76 * assign1280_e1816);
        let assign1280_e1818: f64 = (1.0 + assign1280_e1817);
        let assign1280_e1819: f64 = (locals.var_rd1 * assign1280_e1818);
        locals.var_rd1_t = assign1280_e1819;
        locals.var_rd1_t_dn3 = ((locals.var_rd1_dn3 * assign1280_e1818) + (locals.var_rd1 * (p.p76 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })));
        locals.var_rd1_t_dn4 = (locals.var_rd1_dn4 * assign1280_e1818);
        locals.var_rd1_t_dn5 = (locals.var_rd1_dn5 * assign1280_e1818);
        locals.var_rd1_t_dn8 = (locals.var_rd1_dn8 * assign1280_e1818);
        locals.var_rd1_t_dn10 = (locals.var_rd1_dn10 * assign1280_e1818);
        locals.var_rd1_t_dn12 = (locals.var_rd1_dn12 * assign1280_e1818);
        locals.var_rd1_t_db1 = (locals.var_rd1_db1 * assign1280_e1818);

        let assign1300_e1830: f64 = if p.p5 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign1300_e1830;

        let (assign1310_e1841, assign1310_e1841_d_n3, assign1310_e1841_d_n4, assign1310_e1841_d_n5, assign1310_e1841_d_n8, assign1310_e1841_d_n10, assign1310_e1841_d_n12, assign1310_e1841_d_b1,) = {
    if (locals.var_guard12 != 0.0) {
        let assign1310_e1834: f64 = (-1.0);
        let assign1310_e1836: f64 = (assign1310_e1834 * locals.var_vjg_t);
        let assign1310_e1837: f64 = (assign1310_e1836).tanh();
        let assign1310_e1838: f64 = (locals.var_pg_param * assign1310_e1837);
        let assign1310_e1839: f64 = { let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1310_e1839, ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((locals.var_pg_param_dn3 * assign1310_e1837) + (locals.var_pg_param * ((assign1310_e1834 * locals.var_vjg_t_dn3) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign1310_e1841;
        locals.var_t0_dn3 = assign1310_e1841_d_n3;
        locals.var_t0_dn4 = assign1310_e1841_d_n4;
        locals.var_t0_dn5 = assign1310_e1841_d_n5;
        locals.var_t0_dn8 = assign1310_e1841_d_n8;
        locals.var_t0_dn10 = assign1310_e1841_d_n10;
        locals.var_t0_dn12 = assign1310_e1841_d_n12;
        locals.var_t0_db1 = assign1310_e1841_d_b1;

        let (assign1360_e1880, assign1360_e1880_d_n3, assign1360_e1880_d_n4, assign1360_e1880_d_n5, assign1360_e1880_d_n8, assign1360_e1880_d_n10, assign1360_e1880_d_n12, assign1360_e1880_d_b1,) = {
    if (locals.var_guard12 == 0.0) {
        let assign1360_e1875: f64 = (-locals.var_pg_param);
        let assign1360_e1877: f64 = (assign1360_e1875 * locals.var_vjg_t);
        let assign1360_e1878: f64 = { let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1360_e1878, ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-locals.var_pg_param_dn3) * locals.var_vjg_t) + (assign1360_e1875 * locals.var_vjg_t_dn3))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign1360_e1880;
        locals.var_t0_dn3 = assign1360_e1880_d_n3;
        locals.var_t0_dn4 = assign1360_e1880_d_n4;
        locals.var_t0_dn5 = assign1360_e1880_d_n5;
        locals.var_t0_dn8 = assign1360_e1880_d_n8;
        locals.var_t0_dn10 = assign1360_e1880_d_n10;
        locals.var_t0_dn12 = assign1360_e1880_d_n12;
        locals.var_t0_db1 = assign1360_e1880_d_b1;

        let assign1500_e2001: f64 = (p.p31 * locals.var_vgsc);
        let assign1500_e2002: f64 = (locals.var_p10_t + assign1500_e2001);
        let assign1500_e2005: f64 = (p.p38 * locals.var_vds);
        let assign1500_e2006: f64 = (assign1500_e2002 + assign1500_e2005);
        locals.var_psi_1 = assign1500_e2006;
        locals.var_psi_1_dn3 = locals.var_p10_t_dn3;
        locals.var_psi_1_dn5 = (p.p38 * locals.var_vds_dn5);
        locals.var_psi_1_dn8 = ((p.p31 * locals.var_vgsc_dn8) + (p.p38 * locals.var_vds_dn8));
        locals.var_psi_1_dn11 = (p.p31 * locals.var_vgsc_dn11);

        let assign1510_e2009: f64 = (locals.var_psi_1).tanh();
        let assign1510_e2010: f64 = (1.0 + assign1510_e2009);
        locals.var_tanh1 = assign1510_e2010;
        locals.var_tanh1_dn3 = (locals.var_psi_1_dn3 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn5 = (locals.var_psi_1_dn5 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn8 = (locals.var_psi_1_dn8 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn11 = (locals.var_psi_1_dn11 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));

        let assign1520_e2014: f64 = (p.p33 * locals.var_vds);
        let assign1520_e2015: f64 = (p.p32 + assign1520_e2014);
        locals.var_psi_2 = assign1520_e2015;
        locals.var_psi_2_dn5 = (p.p33 * locals.var_vds_dn5);
        locals.var_psi_2_dn8 = (p.p33 * locals.var_vds_dn8);

        let assign1530_e2018: f64 = (locals.var_psi_2).tanh();
        let assign1530_e2019: f64 = (1.0 + assign1530_e2018);
        locals.var_tanh2 = assign1530_e2019;
        locals.var_tanh2_dn5 = (locals.var_psi_2_dn5 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh()));
        locals.var_tanh2_dn8 = (locals.var_psi_2_dn8 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh()));

        let assign1540_e2023: f64 = (p.p35 * locals.var_vds);
        let assign1540_e2024: f64 = (p.p34 - assign1540_e2023);
        locals.var_psi_3 = assign1540_e2024;
        locals.var_psi_3_dn5 = (-(p.p35 * locals.var_vds_dn5));
        locals.var_psi_3_dn8 = (-(p.p35 * locals.var_vds_dn8));

        let assign1550_e2027: f64 = (locals.var_psi_3).tanh();
        let assign1550_e2028: f64 = (1.0 + assign1550_e2027);
        let assign1550_e2030: f64 = (assign1550_e2028 - p.p38);
        locals.var_tanh3 = assign1550_e2030;
        locals.var_tanh3_dn5 = (locals.var_psi_3_dn5 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh()));
        locals.var_tanh3_dn8 = (locals.var_psi_3_dn8 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh()));

        let assign1560_e2034: f64 = (p.p37 * locals.var_vgdc);
        let assign1560_e2035: f64 = (locals.var_p40_t + assign1560_e2034);
        let assign1560_e2038: f64 = (p.p38 * locals.var_vds);
        let assign1560_e2039: f64 = (assign1560_e2035 - assign1560_e2038);
        locals.var_psi_4 = assign1560_e2039;
        locals.var_psi_4_dn3 = locals.var_p40_t_dn3;
        locals.var_psi_4_dn5 = ((p.p37 * locals.var_vgdc_dn5) - (p.p38 * locals.var_vds_dn5));
        locals.var_psi_4_dn8 = (-(p.p38 * locals.var_vds_dn8));
        locals.var_psi_4_dn10 = (p.p37 * locals.var_vgdc_dn10);

        let assign1570_e2042: f64 = (locals.var_psi_4).tanh();
        let assign1570_e2043: f64 = (1.0 + assign1570_e2042);
        locals.var_tanh4 = assign1570_e2043;
        locals.var_tanh4_dn3 = (locals.var_psi_4_dn3 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn5 = (locals.var_psi_4_dn5 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn8 = (locals.var_psi_4_dn8 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn10 = (locals.var_psi_4_dn10 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));

        let assign1580_e2046: f64 = if p.p6 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1580_e2046;

        let assign1590_e2049: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1590_e2049;

        let assign1600_e2052: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1600_e2052;

        let assign1610_e2055: f64 = if p.p6 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1610_e2055;

        let assign1620_e2058: f64 = if p.p6 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign1620_e2058;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1630_e2062, assign1630_e2062_d_n3, assign1630_e2062_d_n5, assign1630_e2062_d_n8, assign1630_e2062_d_n10, assign1630_e2062_d_n11,) = {
    if (locals.var_guard14 != 0.0) {
        (p.p25, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1630_e2062;
        locals.var_cgs_dn3 = assign1630_e2062_d_n3;
        locals.var_cgs_dn5 = assign1630_e2062_d_n5;
        locals.var_cgs_dn8 = assign1630_e2062_d_n8;
        locals.var_cgs_dn10 = assign1630_e2062_d_n10;
        locals.var_cgs_dn11 = assign1630_e2062_d_n11;

        let (assign1640_e2066, assign1640_e2066_d_n3, assign1640_e2066_d_n5, assign1640_e2066_d_n8, assign1640_e2066_d_n10, assign1640_e2066_d_n11,) = {
    if (locals.var_guard14 != 0.0) {
        (p.p27, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1640_e2066;
        locals.var_cgd_dn3 = assign1640_e2066_d_n3;
        locals.var_cgd_dn5 = assign1640_e2066_d_n5;
        locals.var_cgd_dn8 = assign1640_e2066_d_n8;
        locals.var_cgd_dn10 = assign1640_e2066_d_n10;
        locals.var_cgd_dn11 = assign1640_e2066_d_n11;

        let (assign1650_e2079, assign1650_e2079_d_n3, assign1650_e2079_d_n5, assign1650_e2079_d_n8, assign1650_e2079_d_n10, assign1650_e2079_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (locals.var_guard14 == 0.0)) {
        let assign1650_e2074: f64 = (locals.var_cgs0_t * locals.var_tanh1);
        let assign1650_e2076: f64 = (assign1650_e2074 * locals.var_tanh2);
        let assign1650_e2077: f64 = (p.p25 + assign1650_e2076);
        (assign1650_e2077, (((locals.var_cgs0_t_dn3 * locals.var_tanh1) + (locals.var_cgs0_t * locals.var_tanh1_dn3)) * locals.var_tanh2), (((locals.var_cgs0_t * locals.var_tanh1_dn5) * locals.var_tanh2) + (assign1650_e2074 * locals.var_tanh2_dn5)), (((locals.var_cgs0_t * locals.var_tanh1_dn8) * locals.var_tanh2) + (assign1650_e2074 * locals.var_tanh2_dn8)), 0.0, ((locals.var_cgs0_t * locals.var_tanh1_dn11) * locals.var_tanh2),)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1650_e2079;
        locals.var_cgs_dn3 = assign1650_e2079_d_n3;
        locals.var_cgs_dn5 = assign1650_e2079_d_n5;
        locals.var_cgs_dn8 = assign1650_e2079_d_n8;
        locals.var_cgs_dn10 = assign1650_e2079_d_n10;
        locals.var_cgs_dn11 = assign1650_e2079_d_n11;

        let (assign1660_e2096, assign1660_e2096_d_n3, assign1660_e2096_d_n5, assign1660_e2096_d_n8, assign1660_e2096_d_n10, assign1660_e2096_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (locals.var_guard14 == 0.0)) {
        let assign1660_e2088: f64 = (locals.var_tanh3 * locals.var_tanh4);
        let assign1660_e2091: f64 = (2.0 * p.p38);
        let assign1660_e2092: f64 = (assign1660_e2088 + assign1660_e2091);
        let assign1660_e2093: f64 = (locals.var_cgd0_t * assign1660_e2092);
        let assign1660_e2094: f64 = (p.p27 + assign1660_e2093);
        (assign1660_e2094, ((locals.var_cgd0_t_dn3 * assign1660_e2092) + (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn3))), (locals.var_cgd0_t * ((locals.var_tanh3_dn5 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn5))), (locals.var_cgd0_t * ((locals.var_tanh3_dn8 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn8))), (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn10)), 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1660_e2096;
        locals.var_cgd_dn3 = assign1660_e2096_d_n3;
        locals.var_cgd_dn5 = assign1660_e2096_d_n5;
        locals.var_cgd_dn8 = assign1660_e2096_d_n8;
        locals.var_cgd_dn10 = assign1660_e2096_d_n10;
        locals.var_cgd_dn11 = assign1660_e2096_d_n11;

        let (assign1670_e2107, assign1670_e2107_d_n5, assign1670_e2107_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1670_e2105: f64 = (locals.var_tanh2 - p.p38);
        (assign1670_e2105, locals.var_tanh2_dn5, locals.var_tanh2_dn8,)
    } else {
        (locals.var_tanh2, locals.var_tanh2_dn5, locals.var_tanh2_dn8,)
    }
};
        locals.var_tanh2 = assign1670_e2107;
        locals.var_tanh2_dn5 = assign1670_e2107_d_n5;
        locals.var_tanh2_dn8 = assign1670_e2107_d_n8;

        let (assign1680_e2121, assign1680_e2121_d_n3, assign1680_e2121_d_n5, assign1680_e2121_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1680_e2117: f64 = (p.p38 * locals.var_vds);
        let assign1680_e2118: f64 = (locals.var_p10_t + assign1680_e2117);
        let assign1680_e2119: f64 = (assign1680_e2118).cosh();
        (assign1680_e2119, ((assign1680_e2118).sinh() * locals.var_p10_t_dn3), ((assign1680_e2118).sinh() * (p.p38 * locals.var_vds_dn5)), ((assign1680_e2118).sinh() * (p.p38 * locals.var_vds_dn8)),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn8,)
    }
};
        locals.var_cosh0 = assign1680_e2121;
        locals.var_cosh0_dn3 = assign1680_e2121_d_n3;
        locals.var_cosh0_dn5 = assign1680_e2121_d_n5;
        locals.var_cosh0_dn8 = assign1680_e2121_d_n8;

        let (assign1690_e2131, assign1690_e2131_d_n3, assign1690_e2131_d_n5, assign1690_e2131_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1690_e2129: f64 = (locals.var_cosh0).ln();
        (assign1690_e2129, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn8 / locals.var_cosh0),)
    } else {
        (locals.var_lc10, locals.var_lc10_dn3, locals.var_lc10_dn5, locals.var_lc10_dn8,)
    }
};
        locals.var_lc10 = assign1690_e2131;
        locals.var_lc10_dn3 = assign1690_e2131_d_n3;
        locals.var_lc10_dn5 = assign1690_e2131_d_n5;
        locals.var_lc10_dn8 = assign1690_e2131_d_n8;

        let (assign1700_e2141, assign1700_e2141_d_n3, assign1700_e2141_d_n5, assign1700_e2141_d_n8, assign1700_e2141_d_n10, assign1700_e2141_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1700_e2139: f64 = (locals.var_psi_1).cosh();
        (assign1700_e2139, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn3), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn5), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn8), 0.0, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn11),)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn8, locals.var_cosh1_dn10, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1700_e2141;
        locals.var_cosh1_dn3 = assign1700_e2141_d_n3;
        locals.var_cosh1_dn5 = assign1700_e2141_d_n5;
        locals.var_cosh1_dn8 = assign1700_e2141_d_n8;
        locals.var_cosh1_dn10 = assign1700_e2141_d_n10;
        locals.var_cosh1_dn11 = assign1700_e2141_d_n11;

        let (assign1710_e2151, assign1710_e2151_d_n3, assign1710_e2151_d_n5, assign1710_e2151_d_n8, assign1710_e2151_d_n10, assign1710_e2151_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1710_e2149: f64 = (locals.var_cosh1).ln();
        (assign1710_e2149, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn10 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc1, locals.var_lc1_dn3, locals.var_lc1_dn5, locals.var_lc1_dn8, locals.var_lc1_dn10, locals.var_lc1_dn11,)
    }
};
        locals.var_lc1 = assign1710_e2151;
        locals.var_lc1_dn3 = assign1710_e2151_d_n3;
        locals.var_lc1_dn5 = assign1710_e2151_d_n5;
        locals.var_lc1_dn8 = assign1710_e2151_d_n8;
        locals.var_lc1_dn10 = assign1710_e2151_d_n10;
        locals.var_lc1_dn11 = assign1710_e2151_d_n11;

        let (assign1720_e2166, assign1720_e2166_d_n3, assign1720_e2166_d_n5, assign1720_e2166_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1720_e2161: f64 = (p.p38 * locals.var_vds);
        let assign1720_e2162: f64 = (locals.var_p10_t + assign1720_e2161);
        let assign1720_e2164: f64 = (assign1720_e2162 + locals.var_lc10);
        (assign1720_e2164, (locals.var_p10_t_dn3 + locals.var_lc10_dn3), ((p.p38 * locals.var_vds_dn5) + locals.var_lc10_dn5), ((p.p38 * locals.var_vds_dn8) + locals.var_lc10_dn8),)
    } else {
        (locals.var_qgs0, locals.var_qgs0_dn3, locals.var_qgs0_dn5, locals.var_qgs0_dn8,)
    }
};
        locals.var_qgs0 = assign1720_e2166;
        locals.var_qgs0_dn3 = assign1720_e2166_d_n3;
        locals.var_qgs0_dn5 = assign1720_e2166_d_n5;
        locals.var_qgs0_dn8 = assign1720_e2166_d_n8;

        let (assign1730_e2195, assign1730_e2195_d_n3, assign1730_e2195_d_n5, assign1730_e2195_d_n8, assign1730_e2195_d_n10, assign1730_e2195_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1730_e2176: f64 = (locals.var_psi_1 + locals.var_lc1);
        let assign1730_e2178: f64 = (assign1730_e2176 - locals.var_qgs0);
        let assign1730_e2180: f64 = (assign1730_e2178 * locals.var_tanh2);
        let assign1730_e2182: f64 = (assign1730_e2180 / p.p31);
        let assign1730_e2185: f64 = (2.0 * p.p38);
        let assign1730_e2187: f64 = (assign1730_e2185 * locals.var_vgsc);
        let assign1730_e2188: f64 = (assign1730_e2182 + assign1730_e2187);
        let assign1730_e2189: f64 = (locals.var_cgs0_t * assign1730_e2188);
        let assign1730_e2192: f64 = (p.p25 * locals.var_vgsc);
        let assign1730_e2193: f64 = (assign1730_e2189 + assign1730_e2192);
        (assign1730_e2193, ((locals.var_cgs0_t_dn3 * assign1730_e2188) + (locals.var_cgs0_t * ((((locals.var_psi_1_dn3 + locals.var_lc1_dn3) - locals.var_qgs0_dn3) * locals.var_tanh2) / p.p31))), (locals.var_cgs0_t * (((((locals.var_psi_1_dn5 + locals.var_lc1_dn5) - locals.var_qgs0_dn5) * locals.var_tanh2) + (assign1730_e2178 * locals.var_tanh2_dn5)) / p.p31)), ((locals.var_cgs0_t * ((((((locals.var_psi_1_dn8 + locals.var_lc1_dn8) - locals.var_qgs0_dn8) * locals.var_tanh2) + (assign1730_e2178 * locals.var_tanh2_dn8)) / p.p31) + (assign1730_e2185 * locals.var_vgsc_dn8))) + (p.p25 * locals.var_vgsc_dn8)), (locals.var_cgs0_t * ((locals.var_lc1_dn10 * locals.var_tanh2) / p.p31)), ((locals.var_cgs0_t * ((((locals.var_psi_1_dn11 + locals.var_lc1_dn11) * locals.var_tanh2) / p.p31) + (assign1730_e2185 * locals.var_vgsc_dn11))) + (p.p25 * locals.var_vgsc_dn11)),)
    } else {
        (locals.var_qgs, locals.var_qgs_dn3, locals.var_qgs_dn5, locals.var_qgs_dn8, locals.var_qgs_dn10, locals.var_qgs_dn11,)
    }
};
        locals.var_qgs = assign1730_e2195;
        locals.var_qgs_dn3 = assign1730_e2195_d_n3;
        locals.var_qgs_dn5 = assign1730_e2195_d_n5;
        locals.var_qgs_dn8 = assign1730_e2195_d_n8;
        locals.var_qgs_dn10 = assign1730_e2195_d_n10;
        locals.var_qgs_dn11 = assign1730_e2195_d_n11;

        let (assign1740_e2209, assign1740_e2209_d_n3, assign1740_e2209_d_n5, assign1740_e2209_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1740_e2205: f64 = (p.p38 * locals.var_vds);
        let assign1740_e2206: f64 = (locals.var_p40_t - assign1740_e2205);
        let assign1740_e2207: f64 = (assign1740_e2206).cosh();
        (assign1740_e2207, ((assign1740_e2206).sinh() * locals.var_p40_t_dn3), ((assign1740_e2206).sinh() * (-(p.p38 * locals.var_vds_dn5))), ((assign1740_e2206).sinh() * (-(p.p38 * locals.var_vds_dn8))),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn8,)
    }
};
        locals.var_cosh0 = assign1740_e2209;
        locals.var_cosh0_dn3 = assign1740_e2209_d_n3;
        locals.var_cosh0_dn5 = assign1740_e2209_d_n5;
        locals.var_cosh0_dn8 = assign1740_e2209_d_n8;

        let (assign1750_e2219, assign1750_e2219_d_n3, assign1750_e2219_d_n5, assign1750_e2219_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1750_e2217: f64 = (locals.var_cosh0).ln();
        (assign1750_e2217, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn8 / locals.var_cosh0),)
    } else {
        (locals.var_lc40, locals.var_lc40_dn3, locals.var_lc40_dn5, locals.var_lc40_dn8,)
    }
};
        locals.var_lc40 = assign1750_e2219;
        locals.var_lc40_dn3 = assign1750_e2219_d_n3;
        locals.var_lc40_dn5 = assign1750_e2219_d_n5;
        locals.var_lc40_dn8 = assign1750_e2219_d_n8;

        let (assign1760_e2229, assign1760_e2229_d_n3, assign1760_e2229_d_n5, assign1760_e2229_d_n8, assign1760_e2229_d_n10, assign1760_e2229_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1760_e2227: f64 = (locals.var_psi_4).cosh();
        (assign1760_e2227, ((locals.var_psi_4).sinh() * locals.var_psi_4_dn3), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn5), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn8), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn10), 0.0,)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn8, locals.var_cosh1_dn10, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1760_e2229;
        locals.var_cosh1_dn3 = assign1760_e2229_d_n3;
        locals.var_cosh1_dn5 = assign1760_e2229_d_n5;
        locals.var_cosh1_dn8 = assign1760_e2229_d_n8;
        locals.var_cosh1_dn10 = assign1760_e2229_d_n10;
        locals.var_cosh1_dn11 = assign1760_e2229_d_n11;

        let (assign1770_e2239, assign1770_e2239_d_n3, assign1770_e2239_d_n5, assign1770_e2239_d_n8, assign1770_e2239_d_n10, assign1770_e2239_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1770_e2237: f64 = (locals.var_cosh1).ln();
        (assign1770_e2237, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn10 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc4, locals.var_lc4_dn3, locals.var_lc4_dn5, locals.var_lc4_dn8, locals.var_lc4_dn10, locals.var_lc4_dn11,)
    }
};
        locals.var_lc4 = assign1770_e2239;
        locals.var_lc4_dn3 = assign1770_e2239_d_n3;
        locals.var_lc4_dn5 = assign1770_e2239_d_n5;
        locals.var_lc4_dn8 = assign1770_e2239_d_n8;
        locals.var_lc4_dn10 = assign1770_e2239_d_n10;
        locals.var_lc4_dn11 = assign1770_e2239_d_n11;

        let (assign1780_e2254, assign1780_e2254_d_n3, assign1780_e2254_d_n5, assign1780_e2254_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1780_e2249: f64 = (p.p38 * locals.var_vds);
        let assign1780_e2250: f64 = (locals.var_p40_t - assign1780_e2249);
        let assign1780_e2252: f64 = (assign1780_e2250 + locals.var_lc40);
        (assign1780_e2252, (locals.var_p40_t_dn3 + locals.var_lc40_dn3), ((-(p.p38 * locals.var_vds_dn5)) + locals.var_lc40_dn5), ((-(p.p38 * locals.var_vds_dn8)) + locals.var_lc40_dn8),)
    } else {
        (locals.var_qgd0, locals.var_qgd0_dn3, locals.var_qgd0_dn5, locals.var_qgd0_dn8,)
    }
};
        locals.var_qgd0 = assign1780_e2254;
        locals.var_qgd0_dn3 = assign1780_e2254_d_n3;
        locals.var_qgd0_dn5 = assign1780_e2254_d_n5;
        locals.var_qgd0_dn8 = assign1780_e2254_d_n8;

        let (assign1790_e2283, assign1790_e2283_d_n3, assign1790_e2283_d_n5, assign1790_e2283_d_n8, assign1790_e2283_d_n10, assign1790_e2283_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1790_e2264: f64 = (locals.var_psi_4 + locals.var_lc4);
        let assign1790_e2266: f64 = (assign1790_e2264 - locals.var_qgd0);
        let assign1790_e2268: f64 = (assign1790_e2266 * locals.var_tanh3);
        let assign1790_e2270: f64 = (assign1790_e2268 / p.p37);
        let assign1790_e2273: f64 = (2.0 * p.p38);
        let assign1790_e2275: f64 = (assign1790_e2273 * locals.var_vgdc);
        let assign1790_e2276: f64 = (assign1790_e2270 + assign1790_e2275);
        let assign1790_e2277: f64 = (locals.var_cgd0_t * assign1790_e2276);
        let assign1790_e2280: f64 = (p.p27 * locals.var_vgdc);
        let assign1790_e2281: f64 = (assign1790_e2277 + assign1790_e2280);
        (assign1790_e2281, ((locals.var_cgd0_t_dn3 * assign1790_e2276) + (locals.var_cgd0_t * ((((locals.var_psi_4_dn3 + locals.var_lc4_dn3) - locals.var_qgd0_dn3) * locals.var_tanh3) / p.p37))), ((locals.var_cgd0_t * ((((((locals.var_psi_4_dn5 + locals.var_lc4_dn5) - locals.var_qgd0_dn5) * locals.var_tanh3) + (assign1790_e2266 * locals.var_tanh3_dn5)) / p.p37) + (assign1790_e2273 * locals.var_vgdc_dn5))) + (p.p27 * locals.var_vgdc_dn5)), (locals.var_cgd0_t * (((((locals.var_psi_4_dn8 + locals.var_lc4_dn8) - locals.var_qgd0_dn8) * locals.var_tanh3) + (assign1790_e2266 * locals.var_tanh3_dn8)) / p.p37)), ((locals.var_cgd0_t * ((((locals.var_psi_4_dn10 + locals.var_lc4_dn10) * locals.var_tanh3) / p.p37) + (assign1790_e2273 * locals.var_vgdc_dn10))) + (p.p27 * locals.var_vgdc_dn10)), (locals.var_cgd0_t * ((locals.var_lc4_dn11 * locals.var_tanh3) / p.p37)),)
    } else {
        (locals.var_qgd, locals.var_qgd_dn3, locals.var_qgd_dn5, locals.var_qgd_dn8, locals.var_qgd_dn10, locals.var_qgd_dn11,)
    }
};
        locals.var_qgd = assign1790_e2283;
        locals.var_qgd_dn3 = assign1790_e2283_d_n3;
        locals.var_qgd_dn5 = assign1790_e2283_d_n5;
        locals.var_qgd_dn8 = assign1790_e2283_d_n8;
        locals.var_qgd_dn10 = assign1790_e2283_d_n10;
        locals.var_qgd_dn11 = assign1790_e2283_d_n11;

        let (assign1800_e2294, assign1800_e2294_d_n3, assign1800_e2294_d_n5, assign1800_e2294_d_n8, assign1800_e2294_d_n10, assign1800_e2294_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1800_e2292: f64 = locals.var_qgs_dn11;
        (assign1800_e2292, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1800_e2294;
        locals.var_cgs_dn3 = assign1800_e2294_d_n3;
        locals.var_cgs_dn5 = assign1800_e2294_d_n5;
        locals.var_cgs_dn8 = assign1800_e2294_d_n8;
        locals.var_cgs_dn10 = assign1800_e2294_d_n10;
        locals.var_cgs_dn11 = assign1800_e2294_d_n11;

        let (assign1810_e2305, assign1810_e2305_d_n3, assign1810_e2305_d_n5, assign1810_e2305_d_n8, assign1810_e2305_d_n10, assign1810_e2305_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1810_e2303: f64 = locals.var_qgd_dn10;
        (assign1810_e2303, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1810_e2305;
        locals.var_cgd_dn3 = assign1810_e2305_d_n3;
        locals.var_cgd_dn5 = assign1810_e2305_d_n5;
        locals.var_cgd_dn8 = assign1810_e2305_d_n8;
        locals.var_cgd_dn10 = assign1810_e2305_d_n10;
        locals.var_cgd_dn11 = assign1810_e2305_d_n11;

        let (assign1820_e2320, assign1820_e2320_d_n8, assign1820_e2320_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1820_e2316: f64 = (locals.var_vgsc / p.p40);
        let assign1820_e2318: f64 = (assign1820_e2316 - 1.0);
        (assign1820_e2318, (locals.var_vgsc_dn8 / p.p40), (locals.var_vgsc_dn11 / p.p40),)
    } else {
        (locals.var_y, locals.var_y_dn8, locals.var_y_dn11,)
    }
};
        locals.var_y = assign1820_e2320;
        locals.var_y_dn8 = assign1820_e2320_d_n8;
        locals.var_y_dn11 = assign1820_e2320_d_n11;

        let (assign1830_e2331,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        (0.5,)
    } else {
        (locals.var_mjc,)
    }
};
        locals.var_mjc = assign1830_e2331;

        let (assign1840_e2363, assign1840_e2363_d_n8, assign1840_e2363_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1840_e2343: f64 = (locals.var_y * locals.var_y);
        let assign1840_e2344: f64 = (p.p41 + assign1840_e2343);
        let assign1840_e2346: f64 = (-1.0);
        let assign1840_e2348: f64 = (assign1840_e2346 - locals.var_mjc);
        let assign1840_e2349: f64 = (assign1840_e2344).powf(assign1840_e2348);
        let assign1840_e2354: f64 = (2.0 * locals.var_mjc);
        let assign1840_e2355: f64 = (1.0 - assign1840_e2354);
        let assign1840_e2358: f64 = (locals.var_y * locals.var_y);
        let assign1840_e2359: f64 = (assign1840_e2355 * assign1840_e2358);
        let assign1840_e2360: f64 = (p.p41 + assign1840_e2359);
        let assign1840_e2361: f64 = (assign1840_e2349 * assign1840_e2360);
        (assign1840_e2361, ((if 0.0 == 0.0 && ((assign1840_e2348) as f64).is_finite() && ((assign1840_e2348) as f64).fract() == 0.0 { if assign1840_e2348 == 0.0 { 0.0 } else { (assign1840_e2348 * ((assign1840_e2344).powf(assign1840_e2348 - 1.0) * ((locals.var_y_dn8 * locals.var_y) + (locals.var_y * locals.var_y_dn8)))) } } else { (assign1840_e2349 * (assign1840_e2348 * (((locals.var_y_dn8 * locals.var_y) + (locals.var_y * locals.var_y_dn8)) / assign1840_e2344))) } * assign1840_e2360) + (assign1840_e2349 * (assign1840_e2355 * ((locals.var_y_dn8 * locals.var_y) + (locals.var_y * locals.var_y_dn8))))), ((if 0.0 == 0.0 && ((assign1840_e2348) as f64).is_finite() && ((assign1840_e2348) as f64).fract() == 0.0 { if assign1840_e2348 == 0.0 { 0.0 } else { (assign1840_e2348 * ((assign1840_e2344).powf(assign1840_e2348 - 1.0) * ((locals.var_y_dn11 * locals.var_y) + (locals.var_y * locals.var_y_dn11)))) } } else { (assign1840_e2349 * (assign1840_e2348 * (((locals.var_y_dn11 * locals.var_y) + (locals.var_y * locals.var_y_dn11)) / assign1840_e2344))) } * assign1840_e2360) + (assign1840_e2349 * (assign1840_e2355 * ((locals.var_y_dn11 * locals.var_y) + (locals.var_y * locals.var_y_dn11))))),)
    } else {
        (locals.var_cgsdepl, locals.var_cgsdepl_dn8, locals.var_cgsdepl_dn11,)
    }
};
        locals.var_cgsdepl = assign1840_e2363;
        locals.var_cgsdepl_dn8 = assign1840_e2363_d_n8;
        locals.var_cgsdepl_dn11 = assign1840_e2363_d_n11;

        let (assign1850_e2385, assign1850_e2385_d_n3, assign1850_e2385_d_n5, assign1850_e2385_d_n8, assign1850_e2385_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1850_e2378: f64 = (p.p38 * locals.var_vds);
        let assign1850_e2379: f64 = (locals.var_vgsc + assign1850_e2378);
        let assign1850_e2380: f64 = (p.p31 * assign1850_e2379);
        let assign1850_e2381: f64 = (locals.var_p10_t + assign1850_e2380);
        let assign1850_e2382: f64 = (assign1850_e2381).tanh();
        let assign1850_e2383: f64 = (1.0 + assign1850_e2382);
        (assign1850_e2383, (locals.var_p10_t_dn3 / ((assign1850_e2381).cosh() * (assign1850_e2381).cosh())), ((p.p31 * (p.p38 * locals.var_vds_dn5)) / ((assign1850_e2381).cosh() * (assign1850_e2381).cosh())), ((p.p31 * (locals.var_vgsc_dn8 + (p.p38 * locals.var_vds_dn8))) / ((assign1850_e2381).cosh() * (assign1850_e2381).cosh())), ((p.p31 * locals.var_vgsc_dn11) / ((assign1850_e2381).cosh() * (assign1850_e2381).cosh())),)
    } else {
        (locals.var_tanh1, locals.var_tanh1_dn3, locals.var_tanh1_dn5, locals.var_tanh1_dn8, locals.var_tanh1_dn11,)
    }
};
        locals.var_tanh1 = assign1850_e2385;
        locals.var_tanh1_dn3 = assign1850_e2385_d_n3;
        locals.var_tanh1_dn5 = assign1850_e2385_d_n5;
        locals.var_tanh1_dn8 = assign1850_e2385_d_n8;
        locals.var_tanh1_dn11 = assign1850_e2385_d_n11;

        let (assign1860_e2403, assign1860_e2403_d_n5, assign1860_e2403_d_n8,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1860_e2398: f64 = (p.p33 * locals.var_vds);
        let assign1860_e2399: f64 = (p.p32 + assign1860_e2398);
        let assign1860_e2400: f64 = (assign1860_e2399).tanh();
        let assign1860_e2401: f64 = (1.0 + assign1860_e2400);
        (assign1860_e2401, ((p.p33 * locals.var_vds_dn5) / ((assign1860_e2399).cosh() * (assign1860_e2399).cosh())), ((p.p33 * locals.var_vds_dn8) / ((assign1860_e2399).cosh() * (assign1860_e2399).cosh())),)
    } else {
        (locals.var_tanh2, locals.var_tanh2_dn5, locals.var_tanh2_dn8,)
    }
};
        locals.var_tanh2 = assign1860_e2403;
        locals.var_tanh2_dn5 = assign1860_e2403_d_n5;
        locals.var_tanh2_dn8 = assign1860_e2403_d_n8;

        let (assign1870_e2423, assign1870_e2423_d_n5, assign1870_e2423_d_n8,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1870_e2414: f64 = (1.0 - p.p38);
        let assign1870_e2418: f64 = (p.p35 * locals.var_vds);
        let assign1870_e2419: f64 = (p.p34 - assign1870_e2418);
        let assign1870_e2420: f64 = (assign1870_e2419).tanh();
        let assign1870_e2421: f64 = (assign1870_e2414 + assign1870_e2420);
        (assign1870_e2421, ((-(p.p35 * locals.var_vds_dn5)) / ((assign1870_e2419).cosh() * (assign1870_e2419).cosh())), ((-(p.p35 * locals.var_vds_dn8)) / ((assign1870_e2419).cosh() * (assign1870_e2419).cosh())),)
    } else {
        (locals.var_tanh3, locals.var_tanh3_dn5, locals.var_tanh3_dn8,)
    }
};
        locals.var_tanh3 = assign1870_e2423;
        locals.var_tanh3_dn5 = assign1870_e2423_d_n5;
        locals.var_tanh3_dn8 = assign1870_e2423_d_n8;

        let (assign1880_e2447, assign1880_e2447_d_n3, assign1880_e2447_d_n5, assign1880_e2447_d_n8, assign1880_e2447_d_n10,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1880_e2439: f64 = (1.0 - p.p38);
        let assign1880_e2440: f64 = (locals.var_vds * assign1880_e2439);
        let assign1880_e2441: f64 = (locals.var_vgdc + assign1880_e2440);
        let assign1880_e2442: f64 = (p.p37 * assign1880_e2441);
        let assign1880_e2443: f64 = (locals.var_p40_t + assign1880_e2442);
        let assign1880_e2444: f64 = (assign1880_e2443).tanh();
        let assign1880_e2445: f64 = (1.0 + assign1880_e2444);
        (assign1880_e2445, (locals.var_p40_t_dn3 / ((assign1880_e2443).cosh() * (assign1880_e2443).cosh())), ((p.p37 * (locals.var_vgdc_dn5 + (locals.var_vds_dn5 * assign1880_e2439))) / ((assign1880_e2443).cosh() * (assign1880_e2443).cosh())), ((p.p37 * (locals.var_vds_dn8 * assign1880_e2439)) / ((assign1880_e2443).cosh() * (assign1880_e2443).cosh())), ((p.p37 * locals.var_vgdc_dn10) / ((assign1880_e2443).cosh() * (assign1880_e2443).cosh())),)
    } else {
        (locals.var_tanh4, locals.var_tanh4_dn3, locals.var_tanh4_dn5, locals.var_tanh4_dn8, locals.var_tanh4_dn10,)
    }
};
        locals.var_tanh4 = assign1880_e2447;
        locals.var_tanh4_dn3 = assign1880_e2447_d_n3;
        locals.var_tanh4_dn5 = assign1880_e2447_d_n5;
        locals.var_tanh4_dn8 = assign1880_e2447_d_n8;
        locals.var_tanh4_dn10 = assign1880_e2447_d_n10;

        let (assign1890_e2468, assign1890_e2468_d_n3, assign1890_e2468_d_n5, assign1890_e2468_d_n8, assign1890_e2468_d_n10, assign1890_e2468_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1890_e2460: f64 = (p.p39 * locals.var_cgsdepl);
        let assign1890_e2461: f64 = (locals.var_tanh1 + assign1890_e2460);
        let assign1890_e2462: f64 = (locals.var_cgs0_t * assign1890_e2461);
        let assign1890_e2464: f64 = (assign1890_e2462 * locals.var_tanh2);
        let assign1890_e2466: f64 = (assign1890_e2464 + p.p25);
        (assign1890_e2466, (((locals.var_cgs0_t_dn3 * assign1890_e2461) + (locals.var_cgs0_t * locals.var_tanh1_dn3)) * locals.var_tanh2), (((locals.var_cgs0_t * locals.var_tanh1_dn5) * locals.var_tanh2) + (assign1890_e2462 * locals.var_tanh2_dn5)), (((locals.var_cgs0_t * (locals.var_tanh1_dn8 + (p.p39 * locals.var_cgsdepl_dn8))) * locals.var_tanh2) + (assign1890_e2462 * locals.var_tanh2_dn8)), 0.0, ((locals.var_cgs0_t * (locals.var_tanh1_dn11 + (p.p39 * locals.var_cgsdepl_dn11))) * locals.var_tanh2),)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1890_e2468;
        locals.var_cgs_dn3 = assign1890_e2468_d_n3;
        locals.var_cgs_dn5 = assign1890_e2468_d_n5;
        locals.var_cgs_dn8 = assign1890_e2468_d_n8;
        locals.var_cgs_dn10 = assign1890_e2468_d_n10;
        locals.var_cgs_dn11 = assign1890_e2468_d_n11;

        let (assign1900_e2489, assign1900_e2489_d_n3, assign1900_e2489_d_n5, assign1900_e2489_d_n8, assign1900_e2489_d_n10, assign1900_e2489_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1900_e2480: f64 = (locals.var_tanh3 * locals.var_tanh4);
        let assign1900_e2483: f64 = (2.0 * p.p38);
        let assign1900_e2484: f64 = (assign1900_e2480 + assign1900_e2483);
        let assign1900_e2485: f64 = (locals.var_cgd0_t * assign1900_e2484);
        let assign1900_e2487: f64 = (assign1900_e2485 + p.p27);
        (assign1900_e2487, ((locals.var_cgd0_t_dn3 * assign1900_e2484) + (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn3))), (locals.var_cgd0_t * ((locals.var_tanh3_dn5 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn5))), (locals.var_cgd0_t * ((locals.var_tanh3_dn8 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn8))), (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn10)), 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1900_e2489;
        locals.var_cgd_dn3 = assign1900_e2489_d_n3;
        locals.var_cgd_dn5 = assign1900_e2489_d_n5;
        locals.var_cgd_dn8 = assign1900_e2489_d_n8;
        locals.var_cgd_dn10 = assign1900_e2489_d_n10;
        locals.var_cgd_dn11 = assign1900_e2489_d_n11;

        let (assign1910_e2507, assign1910_e2507_d_n3, assign1910_e2507_d_n5, assign1910_e2507_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1910_e2503: f64 = (p.p38 * locals.var_vds);
        let assign1910_e2504: f64 = (locals.var_p10_t + assign1910_e2503);
        let assign1910_e2505: f64 = (assign1910_e2504).cosh();
        (assign1910_e2505, ((assign1910_e2504).sinh() * locals.var_p10_t_dn3), ((assign1910_e2504).sinh() * (p.p38 * locals.var_vds_dn5)), ((assign1910_e2504).sinh() * (p.p38 * locals.var_vds_dn8)),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn8,)
    }
};
        locals.var_cosh0 = assign1910_e2507;
        locals.var_cosh0_dn3 = assign1910_e2507_d_n3;
        locals.var_cosh0_dn5 = assign1910_e2507_d_n5;
        locals.var_cosh0_dn8 = assign1910_e2507_d_n8;

        let (assign1920_e2521, assign1920_e2521_d_n3, assign1920_e2521_d_n5, assign1920_e2521_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1920_e2519: f64 = (locals.var_cosh0).ln();
        (assign1920_e2519, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn8 / locals.var_cosh0),)
    } else {
        (locals.var_lc10, locals.var_lc10_dn3, locals.var_lc10_dn5, locals.var_lc10_dn8,)
    }
};
        locals.var_lc10 = assign1920_e2521;
        locals.var_lc10_dn3 = assign1920_e2521_d_n3;
        locals.var_lc10_dn5 = assign1920_e2521_d_n5;
        locals.var_lc10_dn8 = assign1920_e2521_d_n8;

        let (assign1930_e2535, assign1930_e2535_d_n3, assign1930_e2535_d_n5, assign1930_e2535_d_n8, assign1930_e2535_d_n10, assign1930_e2535_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1930_e2533: f64 = (locals.var_psi_1).cosh();
        (assign1930_e2533, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn3), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn5), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn8), 0.0, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn11),)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn8, locals.var_cosh1_dn10, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1930_e2535;
        locals.var_cosh1_dn3 = assign1930_e2535_d_n3;
        locals.var_cosh1_dn5 = assign1930_e2535_d_n5;
        locals.var_cosh1_dn8 = assign1930_e2535_d_n8;
        locals.var_cosh1_dn10 = assign1930_e2535_d_n10;
        locals.var_cosh1_dn11 = assign1930_e2535_d_n11;

        let (assign1940_e2549, assign1940_e2549_d_n3, assign1940_e2549_d_n5, assign1940_e2549_d_n8, assign1940_e2549_d_n10, assign1940_e2549_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1940_e2547: f64 = (locals.var_cosh1).ln();
        (assign1940_e2547, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn10 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc1, locals.var_lc1_dn3, locals.var_lc1_dn5, locals.var_lc1_dn8, locals.var_lc1_dn10, locals.var_lc1_dn11,)
    }
};
        locals.var_lc1 = assign1940_e2549;
        locals.var_lc1_dn3 = assign1940_e2549_d_n3;
        locals.var_lc1_dn5 = assign1940_e2549_d_n5;
        locals.var_lc1_dn8 = assign1940_e2549_d_n8;
        locals.var_lc1_dn10 = assign1940_e2549_d_n10;
        locals.var_lc1_dn11 = assign1940_e2549_d_n11;

    }

    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        branches: &[usize; Instance::BRANCH_COUNT],
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
        let bi1 = ctx.branch_current(branches[1]);
        let (assign1950_e2562,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        (0.5,)
    } else {
        (locals.var_mjc,)
    }
};
        locals.var_mjc = assign1950_e2562;

        let (assign1960_e2593, assign1960_e2593_d_n8, assign1960_e2593_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1960_e2576: f64 = (p.p40 + locals.var_vgsc);
        let assign1960_e2577: f64 = (p.p39 * assign1960_e2576);
        let assign1960_e2580: f64 = (-1.0);
        let assign1960_e2583: f64 = (locals.var_vgsc / p.p40);
        let assign1960_e2584: f64 = (assign1960_e2580 + assign1960_e2583);
        let assign1960_e2586: f64 = (assign1960_e2584).powf(2.0);
        let assign1960_e2587: f64 = (p.p41 + assign1960_e2586);
        let assign1960_e2589: f64 = (-locals.var_mjc);
        let assign1960_e2590: f64 = (assign1960_e2587).powf(assign1960_e2589);
        let assign1960_e2591: f64 = (assign1960_e2577 * assign1960_e2590);
        (assign1960_e2591, (((p.p39 * locals.var_vgsc_dn8) * assign1960_e2590) + (assign1960_e2577 * if 0.0 == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (locals.var_vgsc_dn8 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((locals.var_vgsc_dn8 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (locals.var_vgsc_dn8 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((locals.var_vgsc_dn8 / p.p40) / assign1960_e2584))) } / assign1960_e2587))) })), (((p.p39 * locals.var_vgsc_dn11) * assign1960_e2590) + (assign1960_e2577 * if 0.0 == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (locals.var_vgsc_dn11 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((locals.var_vgsc_dn11 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (locals.var_vgsc_dn11 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((locals.var_vgsc_dn11 / p.p40) / assign1960_e2584))) } / assign1960_e2587))) })),)
    } else {
        (locals.var_qgsdepl, locals.var_qgsdepl_dn8, locals.var_qgsdepl_dn11,)
    }
};
        locals.var_qgsdepl = assign1960_e2593;
        locals.var_qgsdepl_dn8 = assign1960_e2593_d_n8;
        locals.var_qgsdepl_dn11 = assign1960_e2593_d_n11;

        let (assign1970_e2615,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1970_e2606: f64 = (p.p39 * p.p40);
        let assign1970_e2609: f64 = (p.p41 + 1.0);
        let assign1970_e2611: f64 = (-locals.var_mjc);
        let assign1970_e2612: f64 = (assign1970_e2609).powf(assign1970_e2611);
        let assign1970_e2613: f64 = (assign1970_e2606 * assign1970_e2612);
        (assign1970_e2613,)
    } else {
        (locals.var_qgsdepl0,)
    }
};
        locals.var_qgsdepl0 = assign1970_e2615;

        let (assign1980_e2634, assign1980_e2634_d_n3, assign1980_e2634_d_n5, assign1980_e2634_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1980_e2629: f64 = (p.p38 * locals.var_vds);
        let assign1980_e2630: f64 = (locals.var_p10_t + assign1980_e2629);
        let assign1980_e2632: f64 = (assign1980_e2630 + locals.var_lc10);
        (assign1980_e2632, (locals.var_p10_t_dn3 + locals.var_lc10_dn3), ((p.p38 * locals.var_vds_dn5) + locals.var_lc10_dn5), ((p.p38 * locals.var_vds_dn8) + locals.var_lc10_dn8),)
    } else {
        (locals.var_qgs0, locals.var_qgs0_dn3, locals.var_qgs0_dn5, locals.var_qgs0_dn8,)
    }
};
        locals.var_qgs0 = assign1980_e2634;
        locals.var_qgs0_dn3 = assign1980_e2634_d_n3;
        locals.var_qgs0_dn5 = assign1980_e2634_d_n5;
        locals.var_qgs0_dn8 = assign1980_e2634_d_n8;

        let (assign1990_e2676, assign1990_e2676_d_n3, assign1990_e2676_d_n5, assign1990_e2676_d_n8, assign1990_e2676_d_n10, assign1990_e2676_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1990_e2648: f64 = (locals.var_psi_1 + locals.var_lc1);
        let assign1990_e2650: f64 = (assign1990_e2648 - locals.var_qgs0);
        let assign1990_e2652: f64 = (assign1990_e2650 + locals.var_qgsdepl);
        let assign1990_e2654: f64 = (assign1990_e2652 - locals.var_qgsdepl0);
        let assign1990_e2657: f64 = (1.0 - p.p38);
        let assign1990_e2659: f64 = (locals.var_psi_2).tanh();
        let assign1990_e2660: f64 = (assign1990_e2657 + assign1990_e2659);
        let assign1990_e2661: f64 = (assign1990_e2654 * assign1990_e2660);
        let assign1990_e2663: f64 = (assign1990_e2661 / p.p31);
        let assign1990_e2666: f64 = (2.0 * p.p38);
        let assign1990_e2668: f64 = (assign1990_e2666 * locals.var_vgsc);
        let assign1990_e2669: f64 = (assign1990_e2663 + assign1990_e2668);
        let assign1990_e2670: f64 = (locals.var_cgs0_t * assign1990_e2669);
        let assign1990_e2673: f64 = (p.p25 * locals.var_vgsc);
        let assign1990_e2674: f64 = (assign1990_e2670 + assign1990_e2673);
        (assign1990_e2674, ((locals.var_cgs0_t_dn3 * assign1990_e2669) + (locals.var_cgs0_t * ((((locals.var_psi_1_dn3 + locals.var_lc1_dn3) - locals.var_qgs0_dn3) * assign1990_e2660) / p.p31))), (locals.var_cgs0_t * (((((locals.var_psi_1_dn5 + locals.var_lc1_dn5) - locals.var_qgs0_dn5) * assign1990_e2660) + (assign1990_e2654 * (locals.var_psi_2_dn5 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh())))) / p.p31)), ((locals.var_cgs0_t * (((((((locals.var_psi_1_dn8 + locals.var_lc1_dn8) - locals.var_qgs0_dn8) + locals.var_qgsdepl_dn8) * assign1990_e2660) + (assign1990_e2654 * (locals.var_psi_2_dn8 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * locals.var_vgsc_dn8))) + (p.p25 * locals.var_vgsc_dn8)), (locals.var_cgs0_t * ((locals.var_lc1_dn10 * assign1990_e2660) / p.p31)), ((locals.var_cgs0_t * (((((locals.var_psi_1_dn11 + locals.var_lc1_dn11) + locals.var_qgsdepl_dn11) * assign1990_e2660) / p.p31) + (assign1990_e2666 * locals.var_vgsc_dn11))) + (p.p25 * locals.var_vgsc_dn11)),)
    } else {
        (locals.var_qgs, locals.var_qgs_dn3, locals.var_qgs_dn5, locals.var_qgs_dn8, locals.var_qgs_dn10, locals.var_qgs_dn11,)
    }
};
        locals.var_qgs = assign1990_e2676;
        locals.var_qgs_dn3 = assign1990_e2676_d_n3;
        locals.var_qgs_dn5 = assign1990_e2676_d_n5;
        locals.var_qgs_dn8 = assign1990_e2676_d_n8;
        locals.var_qgs_dn10 = assign1990_e2676_d_n10;
        locals.var_qgs_dn11 = assign1990_e2676_d_n11;

        let (assign2000_e2694, assign2000_e2694_d_n3, assign2000_e2694_d_n5, assign2000_e2694_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2000_e2690: f64 = (p.p38 * locals.var_vds);
        let assign2000_e2691: f64 = (locals.var_p40_t - assign2000_e2690);
        let assign2000_e2692: f64 = (assign2000_e2691).cosh();
        (assign2000_e2692, ((assign2000_e2691).sinh() * locals.var_p40_t_dn3), ((assign2000_e2691).sinh() * (-(p.p38 * locals.var_vds_dn5))), ((assign2000_e2691).sinh() * (-(p.p38 * locals.var_vds_dn8))),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn8,)
    }
};
        locals.var_cosh0 = assign2000_e2694;
        locals.var_cosh0_dn3 = assign2000_e2694_d_n3;
        locals.var_cosh0_dn5 = assign2000_e2694_d_n5;
        locals.var_cosh0_dn8 = assign2000_e2694_d_n8;

        let (assign2010_e2708, assign2010_e2708_d_n3, assign2010_e2708_d_n5, assign2010_e2708_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2010_e2706: f64 = (locals.var_cosh0).ln();
        (assign2010_e2706, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn8 / locals.var_cosh0),)
    } else {
        (locals.var_lc40, locals.var_lc40_dn3, locals.var_lc40_dn5, locals.var_lc40_dn8,)
    }
};
        locals.var_lc40 = assign2010_e2708;
        locals.var_lc40_dn3 = assign2010_e2708_d_n3;
        locals.var_lc40_dn5 = assign2010_e2708_d_n5;
        locals.var_lc40_dn8 = assign2010_e2708_d_n8;

        let (assign2020_e2722, assign2020_e2722_d_n3, assign2020_e2722_d_n5, assign2020_e2722_d_n8, assign2020_e2722_d_n10, assign2020_e2722_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2020_e2720: f64 = (locals.var_psi_4).cosh();
        (assign2020_e2720, ((locals.var_psi_4).sinh() * locals.var_psi_4_dn3), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn5), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn8), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn10), 0.0,)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn8, locals.var_cosh1_dn10, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign2020_e2722;
        locals.var_cosh1_dn3 = assign2020_e2722_d_n3;
        locals.var_cosh1_dn5 = assign2020_e2722_d_n5;
        locals.var_cosh1_dn8 = assign2020_e2722_d_n8;
        locals.var_cosh1_dn10 = assign2020_e2722_d_n10;
        locals.var_cosh1_dn11 = assign2020_e2722_d_n11;

        let (assign2030_e2736, assign2030_e2736_d_n3, assign2030_e2736_d_n5, assign2030_e2736_d_n8, assign2030_e2736_d_n10, assign2030_e2736_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2030_e2734: f64 = (locals.var_cosh1).ln();
        (assign2030_e2734, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn10 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc4, locals.var_lc4_dn3, locals.var_lc4_dn5, locals.var_lc4_dn8, locals.var_lc4_dn10, locals.var_lc4_dn11,)
    }
};
        locals.var_lc4 = assign2030_e2736;
        locals.var_lc4_dn3 = assign2030_e2736_d_n3;
        locals.var_lc4_dn5 = assign2030_e2736_d_n5;
        locals.var_lc4_dn8 = assign2030_e2736_d_n8;
        locals.var_lc4_dn10 = assign2030_e2736_d_n10;
        locals.var_lc4_dn11 = assign2030_e2736_d_n11;

        let (assign2040_e2755, assign2040_e2755_d_n3, assign2040_e2755_d_n5, assign2040_e2755_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2040_e2750: f64 = (p.p38 * locals.var_vds);
        let assign2040_e2751: f64 = (locals.var_p40_t - assign2040_e2750);
        let assign2040_e2753: f64 = (assign2040_e2751 + locals.var_lc40);
        (assign2040_e2753, (locals.var_p40_t_dn3 + locals.var_lc40_dn3), ((-(p.p38 * locals.var_vds_dn5)) + locals.var_lc40_dn5), ((-(p.p38 * locals.var_vds_dn8)) + locals.var_lc40_dn8),)
    } else {
        (locals.var_qgd0, locals.var_qgd0_dn3, locals.var_qgd0_dn5, locals.var_qgd0_dn8,)
    }
};
        locals.var_qgd0 = assign2040_e2755;
        locals.var_qgd0_dn3 = assign2040_e2755_d_n3;
        locals.var_qgd0_dn5 = assign2040_e2755_d_n5;
        locals.var_qgd0_dn8 = assign2040_e2755_d_n8;

        let (assign2050_e2793, assign2050_e2793_d_n3, assign2050_e2793_d_n5, assign2050_e2793_d_n8, assign2050_e2793_d_n10, assign2050_e2793_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2050_e2769: f64 = (locals.var_psi_4 + locals.var_lc4);
        let assign2050_e2771: f64 = (assign2050_e2769 - locals.var_qgd0);
        let assign2050_e2774: f64 = (1.0 - p.p38);
        let assign2050_e2776: f64 = (locals.var_psi_3).tanh();
        let assign2050_e2777: f64 = (assign2050_e2774 + assign2050_e2776);
        let assign2050_e2778: f64 = (assign2050_e2771 * assign2050_e2777);
        let assign2050_e2780: f64 = (assign2050_e2778 / p.p37);
        let assign2050_e2783: f64 = (2.0 * p.p38);
        let assign2050_e2785: f64 = (assign2050_e2783 * locals.var_vgdc);
        let assign2050_e2786: f64 = (assign2050_e2780 + assign2050_e2785);
        let assign2050_e2787: f64 = (locals.var_cgd0_t * assign2050_e2786);
        let assign2050_e2790: f64 = (p.p27 * locals.var_vgdc);
        let assign2050_e2791: f64 = (assign2050_e2787 + assign2050_e2790);
        (assign2050_e2791, ((locals.var_cgd0_t_dn3 * assign2050_e2786) + (locals.var_cgd0_t * ((((locals.var_psi_4_dn3 + locals.var_lc4_dn3) - locals.var_qgd0_dn3) * assign2050_e2777) / p.p37))), ((locals.var_cgd0_t * ((((((locals.var_psi_4_dn5 + locals.var_lc4_dn5) - locals.var_qgd0_dn5) * assign2050_e2777) + (assign2050_e2771 * (locals.var_psi_3_dn5 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * locals.var_vgdc_dn5))) + (p.p27 * locals.var_vgdc_dn5)), (locals.var_cgd0_t * (((((locals.var_psi_4_dn8 + locals.var_lc4_dn8) - locals.var_qgd0_dn8) * assign2050_e2777) + (assign2050_e2771 * (locals.var_psi_3_dn8 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh())))) / p.p37)), ((locals.var_cgd0_t * ((((locals.var_psi_4_dn10 + locals.var_lc4_dn10) * assign2050_e2777) / p.p37) + (assign2050_e2783 * locals.var_vgdc_dn10))) + (p.p27 * locals.var_vgdc_dn10)), (locals.var_cgd0_t * ((locals.var_lc4_dn11 * assign2050_e2777) / p.p37)),)
    } else {
        (locals.var_qgd, locals.var_qgd_dn3, locals.var_qgd_dn5, locals.var_qgd_dn8, locals.var_qgd_dn10, locals.var_qgd_dn11,)
    }
};
        locals.var_qgd = assign2050_e2793;
        locals.var_qgd_dn3 = assign2050_e2793_d_n3;
        locals.var_qgd_dn5 = assign2050_e2793_d_n5;
        locals.var_qgd_dn8 = assign2050_e2793_d_n8;
        locals.var_qgd_dn10 = assign2050_e2793_d_n10;
        locals.var_qgd_dn11 = assign2050_e2793_d_n11;

        let (assign2060_e2808, assign2060_e2808_d_n3, assign2060_e2808_d_n5, assign2060_e2808_d_n8, assign2060_e2808_d_n10, assign2060_e2808_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2060_e2806: f64 = locals.var_qgs_dn11;
        (assign2060_e2806, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign2060_e2808;
        locals.var_cgs_dn3 = assign2060_e2808_d_n3;
        locals.var_cgs_dn5 = assign2060_e2808_d_n5;
        locals.var_cgs_dn8 = assign2060_e2808_d_n8;
        locals.var_cgs_dn10 = assign2060_e2808_d_n10;
        locals.var_cgs_dn11 = assign2060_e2808_d_n11;

        let (assign2070_e2823, assign2070_e2823_d_n3, assign2070_e2823_d_n5, assign2070_e2823_d_n8, assign2070_e2823_d_n10, assign2070_e2823_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2070_e2821: f64 = locals.var_qgd_dn10;
        (assign2070_e2821, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign2070_e2823;
        locals.var_cgd_dn3 = assign2070_e2823_d_n3;
        locals.var_cgd_dn5 = assign2070_e2823_d_n5;
        locals.var_cgd_dn8 = assign2070_e2823_d_n8;
        locals.var_cgd_dn10 = assign2070_e2823_d_n10;
        locals.var_cgd_dn11 = assign2070_e2823_d_n11;

        let assign2080_e2830: f64 = if ((p.p6 == 2.0) || (p.p6 == 4.0)) { 1.0 } else { 0.0 };
        locals.var_guard19 = assign2080_e2830;

        let assign2090_e2833: f64 = (p.p55 * bi1);
        let assign2090_e2834: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, assign2090_e2833);
        locals.var_t0 = assign2090_e2834;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_db1 = (p.p55 * ddt_scale);

        let assign2100_e2837: f64 = if p.p58 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign2100_e2837;

        let assign2110_e2844: f64 = if ((p.p63 > 0.0) || (p.p62 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard21 = assign2110_e2844;

        let assign2150_e2856: f64 = if p.p46 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard25 = assign2150_e2856;

        let assign2160_e2859: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign2160_e2859;

        let assign2170_e2866: f64 = if ((p.p47 > 0.0) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard27 = assign2170_e2866;

        let assign2210_e2882: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign2210_e2882;

        let assign2220_e2885: f64 = if p.p7 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign2220_e2885;

        let (assign2310_e3018, assign2310_e3018_d_n3,) = {
    if (((locals.var_guard29 != 0.0) && (locals.var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let assign2310_e3005: f64 = (4.0 * 1.3806503e-23);
        let assign2310_e3007: f64 = (assign2310_e3005 * locals.var_t);
        let assign2310_e3009: f64 = (assign2310_e3007 * p.p88);
        let assign2310_e3011: f64 = (assign2310_e3009 * locals.var_cgs0_t);
        let assign2310_e3014: f64 = (p.p87 * p.p86);
        let assign2310_e3015: f64 = (assign2310_e3014).sqrt();
        let assign2310_e3016: f64 = (assign2310_e3011 * assign2310_e3015);
        (assign2310_e3016, (((((assign2310_e3005 * locals.var_t_dn3) * p.p88) * locals.var_cgs0_t) + (assign2310_e3009 * locals.var_cgs0_t_dn3)) * assign2310_e3015),)
    } else {
        (locals.var_k, locals.var_k_dn3,)
    }
};
        locals.var_k = assign2310_e3018;
        locals.var_k_dn3 = assign2310_e3018_d_n3;

        let (assign2340_e3055, assign2340_e3055_d_n3,) = {
    if (((locals.var_guard29 != 0.0) && (locals.var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let assign2340_e3053: f64 = (locals.var_k * 3.141592653589793);
        (assign2340_e3053, (locals.var_k_dn3 * 3.141592653589793),)
    } else {
        (locals.var_ci, locals.var_ci_dn3,)
    }
};
        locals.var_ci = assign2340_e3055;
        locals.var_ci_dn3 = assign2340_e3055_d_n3;

        let assign2380_e3083: f64 = if p.p1 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign2380_e3083;

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
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        locals.var_vgsdel = (nv12 - nv8);
        locals.var_vgsdel_dn8 = -1.0;
        locals.var_vgsdel_dn12 = 1.0;
        locals.var_vgsdel_rv = 0.0;

        locals.var_vgd = (nv10 - nv5);
        locals.var_vgd_dn5 = -1.0;
        locals.var_vgd_dn10 = 1.0;
        locals.var_vgd_rv = 0.0;

        let assign20_e573: f64 = (-locals.var_vgd);
        locals.var_vdg = assign20_e573;
        locals.var_vdg_dn5 = (-locals.var_vgd_dn5);
        locals.var_vdg_dn10 = (-locals.var_vgd_dn10);
        locals.var_vdg_rv = 0.0;

        locals.var_vds = (nv5 - nv8);
        locals.var_vds_dn5 = 1.0;
        locals.var_vds_dn8 = -1.0;
        locals.var_vds_rv = 0.0;

        locals.var_vgsc = (nv11 - nv8);
        locals.var_vgsc_dn8 = -1.0;
        locals.var_vgsc_dn11 = 1.0;
        locals.var_vgsc_rv = 0.0;

        locals.var_vgdc = locals.var_vgd;
        locals.var_vgdc_dn5 = locals.var_vgd_dn5;
        locals.var_vgdc_dn10 = locals.var_vgd_dn10;
        locals.var_vgdc_rv = 0.0;

        locals.var_vrf = (nv4 - nv8);
        locals.var_vrf_dn4 = 1.0;
        locals.var_vrf_dn8 = -1.0;
        locals.var_vrf_rv = 0.0;

        locals.var_qgd = 0.0;
        locals.var_qgd_dn3 = 0.0;
        locals.var_qgd_dn5 = 0.0;
        locals.var_qgd_dn8 = 0.0;
        locals.var_qgd_dn10 = 0.0;
        locals.var_qgd_dn11 = 0.0;
        locals.var_qgd_rv = 0.0;

        locals.var_qgs = 0.0;
        locals.var_qgs_dn3 = 0.0;
        locals.var_qgs_dn5 = 0.0;
        locals.var_qgs_dn8 = 0.0;
        locals.var_qgs_dn10 = 0.0;
        locals.var_qgs_dn11 = 0.0;
        locals.var_qgs_rv = 0.0;

        locals.var_cgd = 0.0;
        locals.var_cgd_dn3 = 0.0;
        locals.var_cgd_dn5 = 0.0;
        locals.var_cgd_dn8 = 0.0;
        locals.var_cgd_dn10 = 0.0;
        locals.var_cgd_dn11 = 0.0;
        locals.var_cgd_rv = 0.0;

        locals.var_cgs = 0.0;
        locals.var_cgs_dn3 = 0.0;
        locals.var_cgs_dn5 = 0.0;
        locals.var_cgs_dn8 = 0.0;
        locals.var_cgs_dn10 = 0.0;
        locals.var_cgs_dn11 = 0.0;
        locals.var_cgs_rv = 0.0;

        let assign150_e587: f64 = if param_given[3] { 1.0 } else { 0.0 };
        locals.var_guard1 = assign150_e587;
        locals.var_guard1_rv = 0.0;

        let (assign160_e593, assign160_e593_d_n3,) = {
    if (locals.var_guard1 != 0.0) {
        let assign160_e591: f64 = (p.p3 + 273.15);
        (assign160_e591, 0.0,)
    } else {
        (locals.var_t, locals.var_t_dn3,)
    }
};
        locals.var_t = assign160_e593;
        locals.var_t_dn3 = assign160_e593_d_n3;
        locals.var_t_rv = 0.0;

        let (assign170_e600, assign170_e600_d_n3,) = {
    if (locals.var_guard1 == 0.0) {
        let assign170_e596: f64 = ctx_temp;
        let assign170_e598: f64 = (assign170_e596 + p.p2);
        (assign170_e598, 0.0,)
    } else {
        (locals.var_t, locals.var_t_dn3,)
    }
};
        locals.var_t = assign170_e600;
        locals.var_t_dn3 = assign170_e600_d_n3;
        locals.var_t_rv = 0.0;

        let assign180_e602: f64 = if param_given[100] { 1.0 } else { 0.0 };
        locals.var_guard2 = assign180_e602;
        locals.var_guard2_rv = 0.0;

        let (assign190_e608,) = {
    if (locals.var_guard2 != 0.0) {
        let assign190_e606: f64 = (p.p100 + 273.15);
        (assign190_e606,)
    } else {
        (locals.var_t_nom,)
    }
};
        locals.var_t_nom = assign190_e608;
        locals.var_t_nom_rv = 0.0;

        let (assign200_e615,) = {
    if (locals.var_guard2 == 0.0) {
        let assign200_e613: f64 = (27.0 + 273.15);
        (assign200_e613,)
    } else {
        (locals.var_t_nom,)
    }
};
        locals.var_t_nom = assign200_e615;
        locals.var_t_nom_rv = 0.0;

        let (assign210_e622, assign210_e622_d_n3,) = {
    if (p.p1 != 0.0) {
        let assign210_e619: f64 = ((nv3 - 0.0)).abs();
        let assign210_e620: f64 = (locals.var_t + assign210_e619);
        (assign210_e620, (locals.var_t_dn3 + if (nv3 - 0.0) >= 0.0 { 1.0 } else { (-1.0) }),)
    } else {
        (locals.var_t, locals.var_t_dn3,)
    }
};
        locals.var_t = assign210_e622;
        locals.var_t_dn3 = assign210_e622_d_n3;
        locals.var_t_rv = 0.0;

        let assign220_e624: f64 = (locals.var_t * THERMAL_VOLTAGE_PER_K);
        locals.var_vth = assign220_e624;
        locals.var_vth_dn3 = (locals.var_t_dn3 * THERMAL_VOLTAGE_PER_K);
        locals.var_vth_rv = 0.0;

        let assign230_e627: f64 = (locals.var_t - locals.var_t_nom);
        let assign230_e628: f64 = (assign230_e627).abs();
        locals.var_delta_t = assign230_e628;
        locals.var_delta_t_dn3 = if assign230_e627 >= 0.0 { locals.var_t_dn3 } else { (-locals.var_t_dn3) };
        locals.var_delta_t_rv = 0.0;

        let assign240_e635: f64 = if ((locals.var_delta_t > 0.0) || (p.p66 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard3 = assign240_e635;
        locals.var_guard3_rv = 0.0;

        let (assign280_e679, assign280_e679_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign280_e674: f64 = (locals.var_delta_t).abs();
        let assign280_e675: f64 = (p.p72 * assign280_e674);
        let assign280_e676: f64 = (1.0 + assign280_e675);
        let assign280_e677: f64 = (p.p26 * assign280_e676);
        (assign280_e677, (p.p26 * (p.p72 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })),)
    } else {
        (locals.var_cgs0_t, locals.var_cgs0_t_dn3,)
    }
};
        locals.var_cgs0_t = assign280_e679;
        locals.var_cgs0_t_dn3 = assign280_e679_d_n3;
        locals.var_cgs0_t_rv = 0.0;

        let (assign290_e690, assign290_e690_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign290_e685: f64 = (locals.var_delta_t).abs();
        let assign290_e686: f64 = (p.p73 * assign290_e685);
        let assign290_e687: f64 = (1.0 + assign290_e686);
        let assign290_e688: f64 = (p.p29 * assign290_e687);
        (assign290_e688, (p.p29 * (p.p73 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })),)
    } else {
        (locals.var_cgd0_t, locals.var_cgd0_t_dn3,)
    }
};
        locals.var_cgd0_t = assign290_e690;
        locals.var_cgd0_t_dn3 = assign290_e690_d_n3;
        locals.var_cgd0_t_rv = 0.0;

        let (assign300_e701, assign300_e701_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign300_e696: f64 = (locals.var_delta_t).abs();
        let assign300_e697: f64 = (p.p74 * assign300_e696);
        let assign300_e698: f64 = (1.0 + assign300_e697);
        let assign300_e699: f64 = (p.p58 * assign300_e698);
        (assign300_e699, (p.p58 * (p.p74 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })),)
    } else {
        (locals.var_rc_t, locals.var_rc_t_dn3,)
    }
};
        locals.var_rc_t = assign300_e701;
        locals.var_rc_t_dn3 = assign300_e701_d_n3;
        locals.var_rc_t_rv = 0.0;

        let (assign320_e720, assign320_e720_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign320_e717: f64 = (p.p78 * locals.var_delta_t);
        let assign320_e718: f64 = (p.p9 + assign320_e717);
        (assign320_e718, (p.p78 * locals.var_delta_t_dn3),)
    } else {
        (locals.var_vpks_t, locals.var_vpks_t_dn3,)
    }
};
        locals.var_vpks_t = assign320_e720;
        locals.var_vpks_t_dn3 = assign320_e720_d_n3;
        locals.var_vpks_t_rv = 0.0;

        let (assign330_e730, assign330_e730_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign330_e726: f64 = (p.p71 * locals.var_delta_t);
        let assign330_e727: f64 = (1.0 + assign330_e726);
        let assign330_e728: f64 = (p.p30 * assign330_e727);
        (assign330_e728, (p.p30 * (p.p71 * locals.var_delta_t_dn3)),)
    } else {
        (locals.var_p10_t, locals.var_p10_t_dn3,)
    }
};
        locals.var_p10_t = assign330_e730;
        locals.var_p10_t_dn3 = assign330_e730_d_n3;
        locals.var_p10_t_rv = 0.0;

        let (assign340_e740, assign340_e740_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign340_e736: f64 = (p.p71 * locals.var_delta_t);
        let assign340_e737: f64 = (1.0 + assign340_e736);
        let assign340_e738: f64 = (p.p36 * assign340_e737);
        (assign340_e738, (p.p36 * (p.p71 * locals.var_delta_t_dn3)),)
    } else {
        (locals.var_p40_t, locals.var_p40_t_dn3,)
    }
};
        locals.var_p40_t = assign340_e740;
        locals.var_p40_t_dn3 = assign340_e740_d_n3;
        locals.var_p40_t_rv = 0.0;

        let (assign350_e748, assign350_e748_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign350_e745: f64 = (p.p79 * locals.var_delta_t);
        let assign350_e746: f64 = (p.p45 + assign350_e745);
        (assign350_e746, (p.p79 * locals.var_delta_t_dn3),)
    } else {
        (locals.var_vjg_t, locals.var_vjg_t_dn3,)
    }
};
        locals.var_vjg_t = assign350_e748;
        locals.var_vjg_t_dn3 = assign350_e748_d_n3;
        locals.var_vjg_t_rv = 0.0;

        let (assign360_e756, assign360_e756_d_n3,) = {
    if (locals.var_guard3 != 0.0) {
        let assign360_e753: f64 = (p.p81 * locals.var_delta_t);
        let assign360_e754: f64 = (p.p21 + assign360_e753);
        (assign360_e754, (p.p81 * locals.var_delta_t_dn3),)
    } else {
        (locals.var_vtr_t, locals.var_vtr_t_dn3,)
    }
};
        locals.var_vtr_t = assign360_e756;
        locals.var_vtr_t_dn3 = assign360_e756_d_n3;
        locals.var_vtr_t_rv = 0.0;

        let assign370_e767: f64 = if (((p.p4 == 1.0) || (p.p4 == 4.0)) && (p.p6 == 4.0)) { 1.0 } else { 0.0 };
        locals.var_guard4 = assign370_e767;
        locals.var_guard4_rv = 0.0;

        let (assign390_e795, assign390_e795_d_n3,) = {
    if ((locals.var_guard3 != 0.0) && (locals.var_guard4 != 0.0)) {
        let assign390_e790: f64 = (locals.var_delta_t * locals.var_delta_t);
        let assign390_e791: f64 = (p.p75 * assign390_e790);
        let assign390_e792: f64 = (1.0 + assign390_e791);
        let assign390_e793: f64 = (p.p63 * assign390_e792);
        (assign390_e793, (p.p63 * (p.p75 * ((locals.var_delta_t_dn3 * locals.var_delta_t) + (locals.var_delta_t * locals.var_delta_t_dn3)))),)
    } else {
        (locals.var_cdel_t, locals.var_cdel_t_dn3,)
    }
};
        locals.var_cdel_t = assign390_e795;
        locals.var_cdel_t_dn3 = assign390_e795_d_n3;
        locals.var_cdel_t_rv = 0.0;

        let (assign410_e823, assign410_e823_d_n3,) = {
    if ((locals.var_guard3 != 0.0) && (locals.var_guard4 == 0.0)) {
        let assign410_e818: f64 = (locals.var_delta_t).abs();
        let assign410_e819: f64 = (p.p75 * assign410_e818);
        let assign410_e820: f64 = (1.0 + assign410_e819);
        let assign410_e821: f64 = (p.p63 * assign410_e820);
        (assign410_e821, (p.p63 * (p.p75 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })),)
    } else {
        (locals.var_cdel_t, locals.var_cdel_t_dn3,)
    }
};
        locals.var_cdel_t = assign410_e823;
        locals.var_cdel_t_dn3 = assign410_e823_d_n3;
        locals.var_cdel_t_rv = 0.0;

        let (assign440_e838, assign440_e838_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p26, 0.0,)
    } else {
        (locals.var_cgs0_t, locals.var_cgs0_t_dn3,)
    }
};
        locals.var_cgs0_t = assign440_e838;
        locals.var_cgs0_t_dn3 = assign440_e838_d_n3;
        locals.var_cgs0_t_rv = 0.0;

        let (assign450_e843, assign450_e843_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p29, 0.0,)
    } else {
        (locals.var_cgd0_t, locals.var_cgd0_t_dn3,)
    }
};
        locals.var_cgd0_t = assign450_e843;
        locals.var_cgd0_t_dn3 = assign450_e843_d_n3;
        locals.var_cgd0_t_rv = 0.0;

        let (assign460_e848, assign460_e848_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p58, 0.0,)
    } else {
        (locals.var_rc_t, locals.var_rc_t_dn3,)
    }
};
        locals.var_rc_t = assign460_e848;
        locals.var_rc_t_dn3 = assign460_e848_d_n3;
        locals.var_rc_t_rv = 0.0;

        let (assign490_e863, assign490_e863_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p63, 0.0,)
    } else {
        (locals.var_cdel_t, locals.var_cdel_t_dn3,)
    }
};
        locals.var_cdel_t = assign490_e863;
        locals.var_cdel_t_dn3 = assign490_e863_d_n3;
        locals.var_cdel_t_rv = 0.0;

        let (assign500_e868, assign500_e868_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p9, 0.0,)
    } else {
        (locals.var_vpks_t, locals.var_vpks_t_dn3,)
    }
};
        locals.var_vpks_t = assign500_e868;
        locals.var_vpks_t_dn3 = assign500_e868_d_n3;
        locals.var_vpks_t_rv = 0.0;

        let (assign510_e873, assign510_e873_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p30, 0.0,)
    } else {
        (locals.var_p10_t, locals.var_p10_t_dn3,)
    }
};
        locals.var_p10_t = assign510_e873;
        locals.var_p10_t_dn3 = assign510_e873_d_n3;
        locals.var_p10_t_rv = 0.0;

        let (assign520_e878, assign520_e878_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p36, 0.0,)
    } else {
        (locals.var_p40_t, locals.var_p40_t_dn3,)
    }
};
        locals.var_p40_t = assign520_e878;
        locals.var_p40_t_dn3 = assign520_e878_d_n3;
        locals.var_p40_t_rv = 0.0;

        let (assign530_e883, assign530_e883_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p45, 0.0,)
    } else {
        (locals.var_vjg_t, locals.var_vjg_t_dn3,)
    }
};
        locals.var_vjg_t = assign530_e883;
        locals.var_vjg_t_dn3 = assign530_e883_d_n3;
        locals.var_vjg_t_rv = 0.0;

        let (assign540_e888, assign540_e888_d_n3,) = {
    if (locals.var_guard3 == 0.0) {
        (p.p21, 0.0,)
    } else {
        (locals.var_vtr_t, locals.var_vtr_t_dn3,)
    }
};
        locals.var_vtr_t = assign540_e888;
        locals.var_vtr_t_dn3 = assign540_e888_d_n3;
        locals.var_vtr_t_rv = 0.0;

        let assign550_e894: f64 = if ((!param_given[43]) && param_given[44]) { 1.0 } else { 0.0 };
        locals.var_guard5 = assign550_e894;
        locals.var_guard5_rv = 0.0;

        let (assign560_e902, assign560_e902_d_n3,) = {
    if (locals.var_guard5 != 0.0) {
        let assign560_e898: f64 = (0.5 / p.p44);
        let assign560_e900: f64 = (assign560_e898 / locals.var_vth);
        (assign560_e900, (-((assign560_e898 * locals.var_vth_dn3) / (locals.var_vth * locals.var_vth))),)
    } else {
        (locals.var_pg_param, locals.var_pg_param_dn3,)
    }
};
        locals.var_pg_param = assign560_e902;
        locals.var_pg_param_dn3 = assign560_e902_d_n3;
        locals.var_pg_param_rv = 0.0;

        let (assign570_e907, assign570_e907_d_n3,) = {
    if (locals.var_guard5 == 0.0) {
        (p.p43, 0.0,)
    } else {
        (locals.var_pg_param, locals.var_pg_param_dn3,)
    }
};
        locals.var_pg_param = assign570_e907;
        locals.var_pg_param_dn3 = assign570_e907_d_n3;
        locals.var_pg_param_rv = 0.0;

        let assign580_e910: f64 = (p.p19 * locals.var_vds);
        let assign580_e911: f64 = (assign580_e910).cosh();
        locals.var_t0 = assign580_e911;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = ((assign580_e910).sinh() * (p.p19 * locals.var_vds_dn5));
        locals.var_t0_dn8 = ((assign580_e910).sinh() * (p.p19 * locals.var_vds_dn8));
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_db1 = 0.0;
        locals.var_t0_rv = 0.0;
        locals.var_t0_rdb1 = 0.0;

        let assign590_e914: f64 = (p.p64 * locals.var_vrf);
        locals.var_vbg = assign590_e914;
        locals.var_vbg_dn4 = (p.p64 * locals.var_vrf_dn4);
        locals.var_vbg_dn8 = (p.p64 * locals.var_vrf_dn8);
        locals.var_vbg_rv = 0.0;

        let assign600_e921: f64 = (locals.var_t0 * locals.var_t0);
        let assign600_e922: f64 = (1e-12 + assign600_e921);
        let assign600_e923: f64 = (p.p18 / assign600_e922);
        let assign600_e924: f64 = (1.0 + assign600_e923);
        let assign600_e925: f64 = (p.p11 * assign600_e924);
        locals.var_p1m = assign600_e925;
        locals.var_p1m_dn3 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn4 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn5 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn8 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn10 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_dn12 = (p.p11 * (-((p.p18 * ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_db1 = (p.p11 * (-((p.p18 * ((locals.var_t0_db1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_db1))) / (assign600_e922 * assign600_e922))));
        locals.var_p1m_rv = 0.0;
        locals.var_p1m_rdb1 = 0.0;

        let assign610_e930: f64 = (locals.var_delta_t).abs();
        let assign610_e931: f64 = (p.p69 * assign610_e930);
        let assign610_e932: f64 = (1.0 + assign610_e931);
        let assign610_e933: f64 = (locals.var_p1m * assign610_e932);
        locals.var_p1_t = assign610_e933;
        locals.var_p1_t_dn3 = ((locals.var_p1m_dn3 * assign610_e932) + (locals.var_p1m * (p.p69 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })));
        locals.var_p1_t_dn4 = (locals.var_p1m_dn4 * assign610_e932);
        locals.var_p1_t_dn5 = (locals.var_p1m_dn5 * assign610_e932);
        locals.var_p1_t_dn8 = (locals.var_p1m_dn8 * assign610_e932);
        locals.var_p1_t_dn10 = (locals.var_p1m_dn10 * assign610_e932);
        locals.var_p1_t_dn12 = (locals.var_p1m_dn12 * assign610_e932);
        locals.var_p1_t_db1 = (locals.var_p1m_db1 * assign610_e932);
        locals.var_p1_t_rv = 0.0;
        locals.var_p1_t_rdb1 = 0.0;

        let assign620_e938: f64 = (locals.var_delta_t).abs();
        let assign620_e939: f64 = (p.p70 * assign620_e938);
        let assign620_e940: f64 = (1.0 + assign620_e939);
        let assign620_e941: f64 = (p.p13 * assign620_e940);
        locals.var_p3_t = assign620_e941;
        locals.var_p3_t_dn3 = (p.p13 * (p.p70 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) }));
        locals.var_p3_t_rv = 0.0;

        let assign630_e944: f64 = (locals.var_vpks_t - p.p10);
        let assign630_e948: f64 = (p.p15 * locals.var_vds);
        let assign630_e949: f64 = (assign630_e948).tanh();
        let assign630_e950: f64 = (p.p10 * assign630_e949);
        let assign630_e951: f64 = (assign630_e944 + assign630_e950);
        let assign630_e953: f64 = (assign630_e951 - locals.var_vbg);
        let assign630_e957: f64 = (locals.var_vdg - locals.var_vtr_t);
        let assign630_e958: f64 = (p.p22 * assign630_e957);
        let assign630_e961: f64 = (locals.var_vdg - locals.var_vtr_t);
        let assign630_e962: f64 = (assign630_e958 * assign630_e961);
        let assign630_e963: f64 = (assign630_e953 - assign630_e962);
        locals.var_vpkm = assign630_e963;
        locals.var_vpkm_dn3 = (locals.var_vpks_t_dn3 - (((p.p22 * (-locals.var_vtr_t_dn3)) * assign630_e961) + (assign630_e958 * (-locals.var_vtr_t_dn3))));
        locals.var_vpkm_dn4 = (-locals.var_vbg_dn4);
        locals.var_vpkm_dn5 = ((p.p10 * ((p.p15 * locals.var_vds_dn5) / ((assign630_e948).cosh() * (assign630_e948).cosh()))) - (((p.p22 * locals.var_vdg_dn5) * assign630_e961) + (assign630_e958 * locals.var_vdg_dn5)));
        locals.var_vpkm_dn8 = ((p.p10 * ((p.p15 * locals.var_vds_dn8) / ((assign630_e948).cosh() * (assign630_e948).cosh()))) - locals.var_vbg_dn8);
        locals.var_vpkm_dn10 = (-(((p.p22 * locals.var_vdg_dn10) * assign630_e961) + (assign630_e958 * locals.var_vdg_dn10)));
        locals.var_vpkm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign640_e968: f64 = (locals.var_delta_t).abs();
        let assign640_e969: f64 = (p.p78 * assign640_e968);
        let assign640_e970: f64 = (1.0 + assign640_e969);
        let assign640_e971: f64 = (locals.var_vpkm * assign640_e970);
        locals.var_vpkm_t = assign640_e971;
        locals.var_vpkm_t_dn3 = ((locals.var_vpkm_dn3 * assign640_e970) + (locals.var_vpkm * (p.p78 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })));
        locals.var_vpkm_t_dn4 = (locals.var_vpkm_dn4 * assign640_e970);
        locals.var_vpkm_t_dn5 = (locals.var_vpkm_dn5 * assign640_e970);
        locals.var_vpkm_t_dn8 = (locals.var_vpkm_dn8 * assign640_e970);
        locals.var_vpkm_t_dn10 = (locals.var_vpkm_dn10 * assign640_e970);
        locals.var_vpkm_t_rv = 0.0;

        let assign650_e974: f64 = (locals.var_vgsdel - locals.var_vpkm_t);
        locals.var_t1 = assign650_e974;
        locals.var_t1_dn3 = (-locals.var_vpkm_t_dn3);
        locals.var_t1_dn4 = (-locals.var_vpkm_t_dn4);
        locals.var_t1_dn5 = (-locals.var_vpkm_t_dn5);
        locals.var_t1_dn8 = (locals.var_vgsdel_dn8 - locals.var_vpkm_t_dn8);
        locals.var_t1_dn10 = (-locals.var_vpkm_t_dn10);
        locals.var_t1_dn12 = locals.var_vgsdel_dn12;
        locals.var_t1_db1 = 0.0;
        locals.var_t1_rv = 0.0;
        locals.var_t1_rdb1 = 0.0;

        let assign660_e977: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign660_e977;
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn12 = ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12));
        locals.var_t2_db1 = ((locals.var_t1_db1 * locals.var_t1) + (locals.var_t1 * locals.var_t1_db1));
        locals.var_t2_rv = 0.0;
        locals.var_t2_rdb1 = 0.0;

        let assign670_e980: f64 = (locals.var_p1_t * locals.var_t1);
        let assign670_e983: f64 = (p.p12 * locals.var_t2);
        let assign670_e984: f64 = (assign670_e980 + assign670_e983);
        let assign670_e987: f64 = (locals.var_p3_t * locals.var_t1);
        let assign670_e989: f64 = (assign670_e987 * locals.var_t2);
        let assign670_e990: f64 = (assign670_e984 + assign670_e989);
        locals.var_psi = assign670_e990;
        locals.var_psi_dn3 = ((((locals.var_p1_t_dn3 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn3)) + (p.p12 * locals.var_t2_dn3)) + ((((locals.var_p3_t_dn3 * locals.var_t1) + (locals.var_p3_t * locals.var_t1_dn3)) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn3)));
        locals.var_psi_dn4 = ((((locals.var_p1_t_dn4 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn4)) + (p.p12 * locals.var_t2_dn4)) + (((locals.var_p3_t * locals.var_t1_dn4) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn4)));
        locals.var_psi_dn5 = ((((locals.var_p1_t_dn5 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn5)) + (p.p12 * locals.var_t2_dn5)) + (((locals.var_p3_t * locals.var_t1_dn5) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn5)));
        locals.var_psi_dn8 = ((((locals.var_p1_t_dn8 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn8)) + (p.p12 * locals.var_t2_dn8)) + (((locals.var_p3_t * locals.var_t1_dn8) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn8)));
        locals.var_psi_dn10 = ((((locals.var_p1_t_dn10 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn10)) + (p.p12 * locals.var_t2_dn10)) + (((locals.var_p3_t * locals.var_t1_dn10) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn10)));
        locals.var_psi_dn12 = ((((locals.var_p1_t_dn12 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_dn12)) + (p.p12 * locals.var_t2_dn12)) + (((locals.var_p3_t * locals.var_t1_dn12) * locals.var_t2) + (assign670_e987 * locals.var_t2_dn12)));
        locals.var_psi_db1 = ((((locals.var_p1_t_db1 * locals.var_t1) + (locals.var_p1_t * locals.var_t1_db1)) + (p.p12 * locals.var_t2_db1)) + (((locals.var_p3_t * locals.var_t1_db1) * locals.var_t2) + (assign670_e987 * locals.var_t2_db1)));
        locals.var_psi_rv = 0.0;
        locals.var_psi_rdb1 = 0.0;

        let assign680_e993: f64 = (locals.var_psi).tanh();
        let assign680_e994: f64 = (1.0 + assign680_e993);
        locals.var_tanh_psi = assign680_e994;
        locals.var_tanh_psi_dn3 = (locals.var_psi_dn3 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn4 = (locals.var_psi_dn4 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn5 = (locals.var_psi_dn5 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn8 = (locals.var_psi_dn8 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn10 = (locals.var_psi_dn10 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_dn12 = (locals.var_psi_dn12 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_db1 = (locals.var_psi_db1 / ((locals.var_psi).cosh() * (locals.var_psi).cosh()));
        locals.var_tanh_psi_rv = 0.0;
        locals.var_tanh_psi_rdb1 = 0.0;

        let assign690_e998: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign690_e1000: f64 = (-locals.var_psi);
        let assign690_e1001: f64 = { let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign690_e1002: f64 = (assign690_e998 - assign690_e1001);
        let assign690_e1003: f64 = (0.5 * assign690_e1002);
        let assign690_e1004: f64 = (assign690_e1003).tanh();
        let assign690_e1005: f64 = (1.0 + assign690_e1004);
        locals.var_tanh_psi1 = assign690_e1005;
        locals.var_tanh_psi1_dn3 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn4 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn5 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn8 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn10 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn10) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn10)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_dn12 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn12) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn12)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_db1 = ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_db1) - ({ let limexp_arg = assign690_e1000; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_db1)))) / ((assign690_e1003).cosh() * (assign690_e1003).cosh()));
        locals.var_tanh_psi1_rv = 0.0;
        locals.var_tanh_psi1_rdb1 = 0.0;

        let assign720_e1017: f64 = if p.p4 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign720_e1017;
        locals.var_guard6_rv = 0.0;

        let assign730_e1020: f64 = if p.p4 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard7 = assign730_e1020;
        locals.var_guard7_rv = 0.0;

        let assign740_e1023: f64 = if p.p4 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign740_e1023;
        locals.var_guard8_rv = 0.0;

        let assign750_e1026: f64 = if p.p4 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign750_e1026;
        locals.var_guard9_rv = 0.0;

        let (assign780_e1059, assign780_e1059_d_n3, assign780_e1059_d_n4, assign780_e1059_d_n5, assign780_e1059_d_n8, assign780_e1059_d_n10, assign780_e1059_d_n12, assign780_e1059_d_b1,) = {
    if ((locals.var_guard7 != 0.0) && (locals.var_guard6 == 0.0)) {
        let assign780_e1057: f64 = (locals.var_vgd - locals.var_vpkm_t);
        (assign780_e1057, (-locals.var_vpkm_t_dn3), (-locals.var_vpkm_t_dn4), (locals.var_vgd_dn5 - locals.var_vpkm_t_dn5), (-locals.var_vpkm_t_dn8), (locals.var_vgd_dn10 - locals.var_vpkm_t_dn10), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign780_e1059;
        locals.var_t0_dn3 = assign780_e1059_d_n3;
        locals.var_t0_dn4 = assign780_e1059_d_n4;
        locals.var_t0_dn5 = assign780_e1059_d_n5;
        locals.var_t0_dn8 = assign780_e1059_d_n8;
        locals.var_t0_dn10 = assign780_e1059_d_n10;
        locals.var_t0_dn12 = assign780_e1059_d_n12;
        locals.var_t0_db1 = assign780_e1059_d_b1;
        locals.var_t0_rv = 0.0;
        locals.var_t0_rdb1 = 0.0;

        let (assign790_e1068, assign790_e1068_d_n3, assign790_e1068_d_n4, assign790_e1068_d_n5, assign790_e1068_d_n8, assign790_e1068_d_n10, assign790_e1068_d_n12, assign790_e1068_d_b1,) = {
    if ((locals.var_guard7 != 0.0) && (locals.var_guard6 == 0.0)) {
        let assign790_e1066: f64 = (locals.var_t0 * locals.var_t0);
        (assign790_e1066, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)), ((locals.var_t0_db1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_db1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn12, locals.var_t1_db1,)
    }
};
        locals.var_t1 = assign790_e1068;
        locals.var_t1_dn3 = assign790_e1068_d_n3;
        locals.var_t1_dn4 = assign790_e1068_d_n4;
        locals.var_t1_dn5 = assign790_e1068_d_n5;
        locals.var_t1_dn8 = assign790_e1068_d_n8;
        locals.var_t1_dn10 = assign790_e1068_d_n10;
        locals.var_t1_dn12 = assign790_e1068_d_n12;
        locals.var_t1_db1 = assign790_e1068_d_b1;
        locals.var_t1_rv = 0.0;
        locals.var_t1_rdb1 = 0.0;

        let (assign800_e1077, assign800_e1077_d_n3, assign800_e1077_d_n4, assign800_e1077_d_n5, assign800_e1077_d_n8, assign800_e1077_d_n10, assign800_e1077_d_n12, assign800_e1077_d_b1,) = {
    if ((locals.var_guard7 != 0.0) && (locals.var_guard6 == 0.0)) {
        let assign800_e1075: f64 = (locals.var_t1 * locals.var_t0);
        (assign800_e1075, ((locals.var_t1_dn3 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn3)), ((locals.var_t1_dn4 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn4)), ((locals.var_t1_dn5 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn5)), ((locals.var_t1_dn8 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn8)), ((locals.var_t1_dn10 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn10)), ((locals.var_t1_dn12 * locals.var_t0) + (locals.var_t1 * locals.var_t0_dn12)), ((locals.var_t1_db1 * locals.var_t0) + (locals.var_t1 * locals.var_t0_db1)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn12, locals.var_t2_db1,)
    }
};
        locals.var_t2 = assign800_e1077;
        locals.var_t2_dn3 = assign800_e1077_d_n3;
        locals.var_t2_dn4 = assign800_e1077_d_n4;
        locals.var_t2_dn5 = assign800_e1077_d_n5;
        locals.var_t2_dn8 = assign800_e1077_d_n8;
        locals.var_t2_dn10 = assign800_e1077_d_n10;
        locals.var_t2_dn12 = assign800_e1077_d_n12;
        locals.var_t2_db1 = assign800_e1077_d_b1;
        locals.var_t2_rv = 0.0;
        locals.var_t2_rdb1 = 0.0;

        let (assign900_e1216, assign900_e1216_d_n3, assign900_e1216_d_n4, assign900_e1216_d_n5, assign900_e1216_d_n8, assign900_e1216_d_n10, assign900_e1216_d_n12, assign900_e1216_d_b1,) = {
    if ((locals.var_guard8 != 0.0) && (!((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)))) {
        let assign900_e1214: f64 = (locals.var_vgsdel - locals.var_vpkm_t);
        (assign900_e1214, (-locals.var_vpkm_t_dn3), (-locals.var_vpkm_t_dn4), (-locals.var_vpkm_t_dn5), (locals.var_vgsdel_dn8 - locals.var_vpkm_t_dn8), (-locals.var_vpkm_t_dn10), locals.var_vgsdel_dn12, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign900_e1216;
        locals.var_t0_dn3 = assign900_e1216_d_n3;
        locals.var_t0_dn4 = assign900_e1216_d_n4;
        locals.var_t0_dn5 = assign900_e1216_d_n5;
        locals.var_t0_dn8 = assign900_e1216_d_n8;
        locals.var_t0_dn10 = assign900_e1216_d_n10;
        locals.var_t0_dn12 = assign900_e1216_d_n12;
        locals.var_t0_db1 = assign900_e1216_d_b1;
        locals.var_t0_rv = 0.0;
        locals.var_t0_rdb1 = 0.0;

        let (assign910_e1227, assign910_e1227_d_n3, assign910_e1227_d_n4, assign910_e1227_d_n5, assign910_e1227_d_n8, assign910_e1227_d_n10, assign910_e1227_d_n12, assign910_e1227_d_b1,) = {
    if ((locals.var_guard8 != 0.0) && (!((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)))) {
        let assign910_e1225: f64 = (locals.var_t0 * locals.var_t0);
        (assign910_e1225, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)), ((locals.var_t0_db1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_db1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn12, locals.var_t1_db1,)
    }
};
        locals.var_t1 = assign910_e1227;
        locals.var_t1_dn3 = assign910_e1227_d_n3;
        locals.var_t1_dn4 = assign910_e1227_d_n4;
        locals.var_t1_dn5 = assign910_e1227_d_n5;
        locals.var_t1_dn8 = assign910_e1227_d_n8;
        locals.var_t1_dn10 = assign910_e1227_d_n10;
        locals.var_t1_dn12 = assign910_e1227_d_n12;
        locals.var_t1_db1 = assign910_e1227_d_b1;
        locals.var_t1_rv = 0.0;
        locals.var_t1_rdb1 = 0.0;

        let (assign920_e1248, assign920_e1248_d_n3, assign920_e1248_d_n4, assign920_e1248_d_n5, assign920_e1248_d_n8, assign920_e1248_d_n10, assign920_e1248_d_n12, assign920_e1248_d_b1,) = {
    if ((locals.var_guard8 != 0.0) && (!((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)))) {
        let assign920_e1238: f64 = (p.p12 * locals.var_t1);
        let assign920_e1239: f64 = (locals.var_t0 + assign920_e1238);
        let assign920_e1242: f64 = (locals.var_p3_t * locals.var_t1);
        let assign920_e1244: f64 = (assign920_e1242 * locals.var_t0);
        let assign920_e1245: f64 = (assign920_e1239 + assign920_e1244);
        let assign920_e1246: f64 = (locals.var_p1_t * assign920_e1245);
        (assign920_e1246, ((locals.var_p1_t_dn3 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn3 + (p.p12 * locals.var_t1_dn3)) + ((((locals.var_p3_t_dn3 * locals.var_t1) + (locals.var_p3_t * locals.var_t1_dn3)) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn3))))), ((locals.var_p1_t_dn4 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn4 + (p.p12 * locals.var_t1_dn4)) + (((locals.var_p3_t * locals.var_t1_dn4) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn4))))), ((locals.var_p1_t_dn5 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn5 + (p.p12 * locals.var_t1_dn5)) + (((locals.var_p3_t * locals.var_t1_dn5) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn5))))), ((locals.var_p1_t_dn8 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn8 + (p.p12 * locals.var_t1_dn8)) + (((locals.var_p3_t * locals.var_t1_dn8) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn8))))), ((locals.var_p1_t_dn10 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn10 + (p.p12 * locals.var_t1_dn10)) + (((locals.var_p3_t * locals.var_t1_dn10) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn10))))), ((locals.var_p1_t_dn12 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_dn12 + (p.p12 * locals.var_t1_dn12)) + (((locals.var_p3_t * locals.var_t1_dn12) * locals.var_t0) + (assign920_e1242 * locals.var_t0_dn12))))), ((locals.var_p1_t_db1 * assign920_e1245) + (locals.var_p1_t * ((locals.var_t0_db1 + (p.p12 * locals.var_t1_db1)) + (((locals.var_p3_t * locals.var_t1_db1) * locals.var_t0) + (assign920_e1242 * locals.var_t0_db1))))),)
    } else {
        (locals.var_psi, locals.var_psi_dn3, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn12, locals.var_psi_db1,)
    }
};
        locals.var_psi = assign920_e1248;
        locals.var_psi_dn3 = assign920_e1248_d_n3;
        locals.var_psi_dn4 = assign920_e1248_d_n4;
        locals.var_psi_dn5 = assign920_e1248_d_n5;
        locals.var_psi_dn8 = assign920_e1248_d_n8;
        locals.var_psi_dn10 = assign920_e1248_d_n10;
        locals.var_psi_dn12 = assign920_e1248_d_n12;
        locals.var_psi_db1 = assign920_e1248_d_b1;
        locals.var_psi_rv = 0.0;
        locals.var_psi_rdb1 = 0.0;

        let (assign930_e1267, assign930_e1267_d_n3, assign930_e1267_d_n4, assign930_e1267_d_n5, assign930_e1267_d_n8, assign930_e1267_d_n10, assign930_e1267_d_n12, assign930_e1267_d_b1,) = {
    if ((locals.var_guard8 != 0.0) && (!((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)))) {
        let assign930_e1258: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign930_e1260: f64 = (-locals.var_psi);
        let assign930_e1261: f64 = { let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign930_e1262: f64 = (assign930_e1258 - assign930_e1261);
        let assign930_e1263: f64 = (0.5 * assign930_e1262);
        let assign930_e1264: f64 = (assign930_e1263).tanh();
        let assign930_e1265: f64 = (1.0 + assign930_e1264);
        (assign930_e1265, ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn10) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn10)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn12) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn12)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_db1) - ({ let limexp_arg = assign930_e1260; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_db1)))) / ((assign930_e1263).cosh() * (assign930_e1263).cosh())),)
    } else {
        (locals.var_tanh_psi1, locals.var_tanh_psi1_dn3, locals.var_tanh_psi1_dn4, locals.var_tanh_psi1_dn5, locals.var_tanh_psi1_dn8, locals.var_tanh_psi1_dn10, locals.var_tanh_psi1_dn12, locals.var_tanh_psi1_db1,)
    }
};
        locals.var_tanh_psi1 = assign930_e1267;
        locals.var_tanh_psi1_dn3 = assign930_e1267_d_n3;
        locals.var_tanh_psi1_dn4 = assign930_e1267_d_n4;
        locals.var_tanh_psi1_dn5 = assign930_e1267_d_n5;
        locals.var_tanh_psi1_dn8 = assign930_e1267_d_n8;
        locals.var_tanh_psi1_dn10 = assign930_e1267_d_n10;
        locals.var_tanh_psi1_dn12 = assign930_e1267_d_n12;
        locals.var_tanh_psi1_db1 = assign930_e1267_d_b1;
        locals.var_tanh_psi1_rv = 0.0;
        locals.var_tanh_psi1_rdb1 = 0.0;

        let (assign980_e1346, assign980_e1346_d_n3, assign980_e1346_d_n4, assign980_e1346_d_n5, assign980_e1346_d_n8, assign980_e1346_d_n10, assign980_e1346_d_n12, assign980_e1346_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign980_e1344: f64 = (locals.var_vgsdel - locals.var_vpkm_t);
        (assign980_e1344, (-locals.var_vpkm_t_dn3), (-locals.var_vpkm_t_dn4), (-locals.var_vpkm_t_dn5), (locals.var_vgsdel_dn8 - locals.var_vpkm_t_dn8), (-locals.var_vpkm_t_dn10), locals.var_vgsdel_dn12, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign980_e1346;
        locals.var_t0_dn3 = assign980_e1346_d_n3;
        locals.var_t0_dn4 = assign980_e1346_d_n4;
        locals.var_t0_dn5 = assign980_e1346_d_n5;
        locals.var_t0_dn8 = assign980_e1346_d_n8;
        locals.var_t0_dn10 = assign980_e1346_d_n10;
        locals.var_t0_dn12 = assign980_e1346_d_n12;
        locals.var_t0_db1 = assign980_e1346_d_b1;
        locals.var_t0_rv = 0.0;
        locals.var_t0_rdb1 = 0.0;

        let (assign990_e1359, assign990_e1359_d_n3, assign990_e1359_d_n4, assign990_e1359_d_n5, assign990_e1359_d_n8, assign990_e1359_d_n10, assign990_e1359_d_n12, assign990_e1359_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign990_e1357: f64 = (locals.var_t0 * locals.var_t0);
        (assign990_e1357, ((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)), ((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)), ((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)), ((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)), ((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)), ((locals.var_t0_dn12 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn12)), ((locals.var_t0_db1 * locals.var_t0) + (locals.var_t0 * locals.var_t0_db1)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn12, locals.var_t1_db1,)
    }
};
        locals.var_t1 = assign990_e1359;
        locals.var_t1_dn3 = assign990_e1359_d_n3;
        locals.var_t1_dn4 = assign990_e1359_d_n4;
        locals.var_t1_dn5 = assign990_e1359_d_n5;
        locals.var_t1_dn8 = assign990_e1359_d_n8;
        locals.var_t1_dn10 = assign990_e1359_d_n10;
        locals.var_t1_dn12 = assign990_e1359_d_n12;
        locals.var_t1_db1 = assign990_e1359_d_b1;
        locals.var_t1_rv = 0.0;
        locals.var_t1_rdb1 = 0.0;

        let (assign1000_e1382, assign1000_e1382_d_n3, assign1000_e1382_d_n4, assign1000_e1382_d_n5, assign1000_e1382_d_n8, assign1000_e1382_d_n10, assign1000_e1382_d_n12, assign1000_e1382_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign1000_e1372: f64 = (p.p12 * locals.var_t1);
        let assign1000_e1373: f64 = (locals.var_t0 + assign1000_e1372);
        let assign1000_e1376: f64 = (locals.var_p3_t * locals.var_t1);
        let assign1000_e1378: f64 = (assign1000_e1376 * locals.var_t0);
        let assign1000_e1379: f64 = (assign1000_e1373 + assign1000_e1378);
        let assign1000_e1380: f64 = (locals.var_p1_t * assign1000_e1379);
        (assign1000_e1380, ((locals.var_p1_t_dn3 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn3 + (p.p12 * locals.var_t1_dn3)) + ((((locals.var_p3_t_dn3 * locals.var_t1) + (locals.var_p3_t * locals.var_t1_dn3)) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn3))))), ((locals.var_p1_t_dn4 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn4 + (p.p12 * locals.var_t1_dn4)) + (((locals.var_p3_t * locals.var_t1_dn4) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn4))))), ((locals.var_p1_t_dn5 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn5 + (p.p12 * locals.var_t1_dn5)) + (((locals.var_p3_t * locals.var_t1_dn5) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn5))))), ((locals.var_p1_t_dn8 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn8 + (p.p12 * locals.var_t1_dn8)) + (((locals.var_p3_t * locals.var_t1_dn8) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn8))))), ((locals.var_p1_t_dn10 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn10 + (p.p12 * locals.var_t1_dn10)) + (((locals.var_p3_t * locals.var_t1_dn10) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn10))))), ((locals.var_p1_t_dn12 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_dn12 + (p.p12 * locals.var_t1_dn12)) + (((locals.var_p3_t * locals.var_t1_dn12) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_dn12))))), ((locals.var_p1_t_db1 * assign1000_e1379) + (locals.var_p1_t * ((locals.var_t0_db1 + (p.p12 * locals.var_t1_db1)) + (((locals.var_p3_t * locals.var_t1_db1) * locals.var_t0) + (assign1000_e1376 * locals.var_t0_db1))))),)
    } else {
        (locals.var_psi, locals.var_psi_dn3, locals.var_psi_dn4, locals.var_psi_dn5, locals.var_psi_dn8, locals.var_psi_dn10, locals.var_psi_dn12, locals.var_psi_db1,)
    }
};
        locals.var_psi = assign1000_e1382;
        locals.var_psi_dn3 = assign1000_e1382_d_n3;
        locals.var_psi_dn4 = assign1000_e1382_d_n4;
        locals.var_psi_dn5 = assign1000_e1382_d_n5;
        locals.var_psi_dn8 = assign1000_e1382_d_n8;
        locals.var_psi_dn10 = assign1000_e1382_d_n10;
        locals.var_psi_dn12 = assign1000_e1382_d_n12;
        locals.var_psi_db1 = assign1000_e1382_d_b1;
        locals.var_psi_rv = 0.0;
        locals.var_psi_rdb1 = 0.0;

        let (assign1010_e1395, assign1010_e1395_d_n3, assign1010_e1395_d_n4, assign1010_e1395_d_n5, assign1010_e1395_d_n8, assign1010_e1395_d_n10, assign1010_e1395_d_n12, assign1010_e1395_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign1010_e1393: f64 = (locals.var_vgd - locals.var_vpkm_t);
        (assign1010_e1393, (-locals.var_vpkm_t_dn3), (-locals.var_vpkm_t_dn4), (locals.var_vgd_dn5 - locals.var_vpkm_t_dn5), (-locals.var_vpkm_t_dn8), (locals.var_vgd_dn10 - locals.var_vpkm_t_dn10), 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn12, locals.var_t2_db1,)
    }
};
        locals.var_t2 = assign1010_e1395;
        locals.var_t2_dn3 = assign1010_e1395_d_n3;
        locals.var_t2_dn4 = assign1010_e1395_d_n4;
        locals.var_t2_dn5 = assign1010_e1395_d_n5;
        locals.var_t2_dn8 = assign1010_e1395_d_n8;
        locals.var_t2_dn10 = assign1010_e1395_d_n10;
        locals.var_t2_dn12 = assign1010_e1395_d_n12;
        locals.var_t2_db1 = assign1010_e1395_d_b1;
        locals.var_t2_rv = 0.0;
        locals.var_t2_rdb1 = 0.0;

        let (assign1040_e1452, assign1040_e1452_d_n3, assign1040_e1452_d_n4, assign1040_e1452_d_n5, assign1040_e1452_d_n8, assign1040_e1452_d_n10, assign1040_e1452_d_n12, assign1040_e1452_d_b1,) = {
    if ((locals.var_guard9 != 0.0) && (!(((locals.var_guard6 != 0.0) || (locals.var_guard7 != 0.0)) || (locals.var_guard8 != 0.0)))) {
        let assign1040_e1443: f64 = { let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1040_e1445: f64 = (-locals.var_psi);
        let assign1040_e1446: f64 = { let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        let assign1040_e1447: f64 = (assign1040_e1443 - assign1040_e1446);
        let assign1040_e1448: f64 = (0.5 * assign1040_e1447);
        let assign1040_e1449: f64 = (assign1040_e1448).tanh();
        let assign1040_e1450: f64 = (1.0 + assign1040_e1449);
        (assign1040_e1450, ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn3) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn3)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn4) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn4)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn5) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn5)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn8) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn8)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn10) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn10)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_dn12) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_dn12)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())), ((0.5 * (({ let limexp_arg = locals.var_psi; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * locals.var_psi_db1) - ({ let limexp_arg = assign1040_e1445; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (-locals.var_psi_db1)))) / ((assign1040_e1448).cosh() * (assign1040_e1448).cosh())),)
    } else {
        (locals.var_tanh_psi1, locals.var_tanh_psi1_dn3, locals.var_tanh_psi1_dn4, locals.var_tanh_psi1_dn5, locals.var_tanh_psi1_dn8, locals.var_tanh_psi1_dn10, locals.var_tanh_psi1_dn12, locals.var_tanh_psi1_db1,)
    }
};
        locals.var_tanh_psi1 = assign1040_e1452;
        locals.var_tanh_psi1_dn3 = assign1040_e1452_d_n3;
        locals.var_tanh_psi1_dn4 = assign1040_e1452_d_n4;
        locals.var_tanh_psi1_dn5 = assign1040_e1452_d_n5;
        locals.var_tanh_psi1_dn8 = assign1040_e1452_d_n8;
        locals.var_tanh_psi1_dn10 = assign1040_e1452_d_n10;
        locals.var_tanh_psi1_dn12 = assign1040_e1452_d_n12;
        locals.var_tanh_psi1_db1 = assign1040_e1452_d_b1;
        locals.var_tanh_psi1_rv = 0.0;
        locals.var_tanh_psi1_rdb1 = 0.0;

        let assign1200_e1748: f64 = if (((p.p4 == 0.0) || (p.p4 == 1.0)) || (p.p4 == 4.0)) { 1.0 } else { 0.0 };
        locals.var_guard11 = assign1200_e1748;
        locals.var_guard11_rv = 0.0;

        let (assign1210_e1758, assign1210_e1758_d_n3, assign1210_e1758_d_n4, assign1210_e1758_d_n5, assign1210_e1758_d_n8, assign1210_e1758_d_n10, assign1210_e1758_d_n12, assign1210_e1758_d_b1,) = {
    if (locals.var_guard11 != 0.0) {
        let assign1210_e1754: f64 = (1.0 + locals.var_tanh_psi);
        let assign1210_e1755: f64 = (locals.var_rc_t / assign1210_e1754);
        let assign1210_e1756: f64 = (p.p57 + assign1210_e1755);
        (assign1210_e1756, (((locals.var_rc_t_dn3 * assign1210_e1754) - (locals.var_rc_t * locals.var_tanh_psi_dn3)) / (assign1210_e1754 * assign1210_e1754)), (-((locals.var_rc_t * locals.var_tanh_psi_dn4) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_dn5) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_dn8) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_dn10) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_dn12) / (assign1210_e1754 * assign1210_e1754))), (-((locals.var_rc_t * locals.var_tanh_psi_db1) / (assign1210_e1754 * assign1210_e1754))),)
    } else {
        (locals.var_rc1, locals.var_rc1_dn3, locals.var_rc1_dn4, locals.var_rc1_dn5, locals.var_rc1_dn8, locals.var_rc1_dn10, locals.var_rc1_dn12, locals.var_rc1_db1,)
    }
};
        locals.var_rc1 = assign1210_e1758;
        locals.var_rc1_dn3 = assign1210_e1758_d_n3;
        locals.var_rc1_dn4 = assign1210_e1758_d_n4;
        locals.var_rc1_dn5 = assign1210_e1758_d_n5;
        locals.var_rc1_dn8 = assign1210_e1758_d_n8;
        locals.var_rc1_dn10 = assign1210_e1758_d_n10;
        locals.var_rc1_dn12 = assign1210_e1758_d_n12;
        locals.var_rc1_db1 = assign1210_e1758_d_b1;
        locals.var_rc1_rv = 0.0;
        locals.var_rc1_rdb1 = 0.0;

        let (assign1220_e1766, assign1220_e1766_d_n3, assign1220_e1766_d_n4, assign1220_e1766_d_n5, assign1220_e1766_d_n8, assign1220_e1766_d_n10, assign1220_e1766_d_n12, assign1220_e1766_d_b1,) = {
    if (locals.var_guard11 != 0.0) {
        let assign1220_e1763: f64 = (p.p48 * locals.var_tanh_psi);
        let assign1220_e1764: f64 = (p.p47 + assign1220_e1763);
        (assign1220_e1764, (p.p48 * locals.var_tanh_psi_dn3), (p.p48 * locals.var_tanh_psi_dn4), (p.p48 * locals.var_tanh_psi_dn5), (p.p48 * locals.var_tanh_psi_dn8), (p.p48 * locals.var_tanh_psi_dn10), (p.p48 * locals.var_tanh_psi_dn12), (p.p48 * locals.var_tanh_psi_db1),)
    } else {
        (locals.var_rd1, locals.var_rd1_dn3, locals.var_rd1_dn4, locals.var_rd1_dn5, locals.var_rd1_dn8, locals.var_rd1_dn10, locals.var_rd1_dn12, locals.var_rd1_db1,)
    }
};
        locals.var_rd1 = assign1220_e1766;
        locals.var_rd1_dn3 = assign1220_e1766_d_n3;
        locals.var_rd1_dn4 = assign1220_e1766_d_n4;
        locals.var_rd1_dn5 = assign1220_e1766_d_n5;
        locals.var_rd1_dn8 = assign1220_e1766_d_n8;
        locals.var_rd1_dn10 = assign1220_e1766_d_n10;
        locals.var_rd1_dn12 = assign1220_e1766_d_n12;
        locals.var_rd1_db1 = assign1220_e1766_d_b1;
        locals.var_rd1_rv = 0.0;
        locals.var_rd1_rdb1 = 0.0;

        let (assign1230_e1774, assign1230_e1774_d_n3, assign1230_e1774_d_n4, assign1230_e1774_d_n5, assign1230_e1774_d_n8, assign1230_e1774_d_n10, assign1230_e1774_d_n12, assign1230_e1774_d_b1,) = {
    if (locals.var_guard11 != 0.0) {
        let assign1230_e1771: f64 = (p.p48 * locals.var_tanh_psi);
        let assign1230_e1772: f64 = (p.p50 + assign1230_e1771);
        (assign1230_e1772, (p.p48 * locals.var_tanh_psi_dn3), (p.p48 * locals.var_tanh_psi_dn4), (p.p48 * locals.var_tanh_psi_dn5), (p.p48 * locals.var_tanh_psi_dn8), (p.p48 * locals.var_tanh_psi_dn10), (p.p48 * locals.var_tanh_psi_dn12), (p.p48 * locals.var_tanh_psi_db1),)
    } else {
        (locals.var_rs1, locals.var_rs1_dn3, locals.var_rs1_dn4, locals.var_rs1_dn5, locals.var_rs1_dn8, locals.var_rs1_dn10, locals.var_rs1_dn12, locals.var_rs1_db1,)
    }
};
        locals.var_rs1 = assign1230_e1774;
        locals.var_rs1_dn3 = assign1230_e1774_d_n3;
        locals.var_rs1_dn4 = assign1230_e1774_d_n4;
        locals.var_rs1_dn5 = assign1230_e1774_d_n5;
        locals.var_rs1_dn8 = assign1230_e1774_d_n8;
        locals.var_rs1_dn10 = assign1230_e1774_d_n10;
        locals.var_rs1_dn12 = assign1230_e1774_d_n12;
        locals.var_rs1_db1 = assign1230_e1774_d_b1;
        locals.var_rs1_rv = 0.0;
        locals.var_rs1_rdb1 = 0.0;

        let (assign1240_e1785, assign1240_e1785_d_n3, assign1240_e1785_d_n4, assign1240_e1785_d_n5, assign1240_e1785_d_n8, assign1240_e1785_d_n10, assign1240_e1785_d_n12, assign1240_e1785_d_b1,) = {
    if (locals.var_guard11 == 0.0) {
        let assign1240_e1781: f64 = (1.0 + locals.var_tanh_psi1);
        let assign1240_e1782: f64 = (locals.var_rc_t / assign1240_e1781);
        let assign1240_e1783: f64 = (p.p57 + assign1240_e1782);
        (assign1240_e1783, (((locals.var_rc_t_dn3 * assign1240_e1781) - (locals.var_rc_t * locals.var_tanh_psi1_dn3)) / (assign1240_e1781 * assign1240_e1781)), (-((locals.var_rc_t * locals.var_tanh_psi1_dn4) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_dn5) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_dn8) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_dn10) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_dn12) / (assign1240_e1781 * assign1240_e1781))), (-((locals.var_rc_t * locals.var_tanh_psi1_db1) / (assign1240_e1781 * assign1240_e1781))),)
    } else {
        (locals.var_rc1, locals.var_rc1_dn3, locals.var_rc1_dn4, locals.var_rc1_dn5, locals.var_rc1_dn8, locals.var_rc1_dn10, locals.var_rc1_dn12, locals.var_rc1_db1,)
    }
};
        locals.var_rc1 = assign1240_e1785;
        locals.var_rc1_dn3 = assign1240_e1785_d_n3;
        locals.var_rc1_dn4 = assign1240_e1785_d_n4;
        locals.var_rc1_dn5 = assign1240_e1785_d_n5;
        locals.var_rc1_dn8 = assign1240_e1785_d_n8;
        locals.var_rc1_dn10 = assign1240_e1785_d_n10;
        locals.var_rc1_dn12 = assign1240_e1785_d_n12;
        locals.var_rc1_db1 = assign1240_e1785_d_b1;
        locals.var_rc1_rv = 0.0;
        locals.var_rc1_rdb1 = 0.0;

        let (assign1250_e1794, assign1250_e1794_d_n3, assign1250_e1794_d_n4, assign1250_e1794_d_n5, assign1250_e1794_d_n8, assign1250_e1794_d_n10, assign1250_e1794_d_n12, assign1250_e1794_d_b1,) = {
    if (locals.var_guard11 == 0.0) {
        let assign1250_e1791: f64 = (p.p48 * locals.var_tanh_psi1);
        let assign1250_e1792: f64 = (p.p47 + assign1250_e1791);
        (assign1250_e1792, (p.p48 * locals.var_tanh_psi1_dn3), (p.p48 * locals.var_tanh_psi1_dn4), (p.p48 * locals.var_tanh_psi1_dn5), (p.p48 * locals.var_tanh_psi1_dn8), (p.p48 * locals.var_tanh_psi1_dn10), (p.p48 * locals.var_tanh_psi1_dn12), (p.p48 * locals.var_tanh_psi1_db1),)
    } else {
        (locals.var_rd1, locals.var_rd1_dn3, locals.var_rd1_dn4, locals.var_rd1_dn5, locals.var_rd1_dn8, locals.var_rd1_dn10, locals.var_rd1_dn12, locals.var_rd1_db1,)
    }
};
        locals.var_rd1 = assign1250_e1794;
        locals.var_rd1_dn3 = assign1250_e1794_d_n3;
        locals.var_rd1_dn4 = assign1250_e1794_d_n4;
        locals.var_rd1_dn5 = assign1250_e1794_d_n5;
        locals.var_rd1_dn8 = assign1250_e1794_d_n8;
        locals.var_rd1_dn10 = assign1250_e1794_d_n10;
        locals.var_rd1_dn12 = assign1250_e1794_d_n12;
        locals.var_rd1_db1 = assign1250_e1794_d_b1;
        locals.var_rd1_rv = 0.0;
        locals.var_rd1_rdb1 = 0.0;

        let (assign1260_e1803, assign1260_e1803_d_n3, assign1260_e1803_d_n4, assign1260_e1803_d_n5, assign1260_e1803_d_n8, assign1260_e1803_d_n10, assign1260_e1803_d_n12, assign1260_e1803_d_b1,) = {
    if (locals.var_guard11 == 0.0) {
        let assign1260_e1800: f64 = (p.p48 * locals.var_tanh_psi1);
        let assign1260_e1801: f64 = (p.p50 + assign1260_e1800);
        (assign1260_e1801, (p.p48 * locals.var_tanh_psi1_dn3), (p.p48 * locals.var_tanh_psi1_dn4), (p.p48 * locals.var_tanh_psi1_dn5), (p.p48 * locals.var_tanh_psi1_dn8), (p.p48 * locals.var_tanh_psi1_dn10), (p.p48 * locals.var_tanh_psi1_dn12), (p.p48 * locals.var_tanh_psi1_db1),)
    } else {
        (locals.var_rs1, locals.var_rs1_dn3, locals.var_rs1_dn4, locals.var_rs1_dn5, locals.var_rs1_dn8, locals.var_rs1_dn10, locals.var_rs1_dn12, locals.var_rs1_db1,)
    }
};
        locals.var_rs1 = assign1260_e1803;
        locals.var_rs1_dn3 = assign1260_e1803_d_n3;
        locals.var_rs1_dn4 = assign1260_e1803_d_n4;
        locals.var_rs1_dn5 = assign1260_e1803_d_n5;
        locals.var_rs1_dn8 = assign1260_e1803_d_n8;
        locals.var_rs1_dn10 = assign1260_e1803_d_n10;
        locals.var_rs1_dn12 = assign1260_e1803_d_n12;
        locals.var_rs1_db1 = assign1260_e1803_d_b1;
        locals.var_rs1_rv = 0.0;
        locals.var_rs1_rdb1 = 0.0;

        let assign1270_e1808: f64 = (locals.var_delta_t).abs();
        let assign1270_e1809: f64 = (p.p76 * assign1270_e1808);
        let assign1270_e1810: f64 = (1.0 + assign1270_e1809);
        let assign1270_e1811: f64 = (locals.var_rs1 * assign1270_e1810);
        locals.var_rs_t = assign1270_e1811;
        locals.var_rs_t_dn3 = ((locals.var_rs1_dn3 * assign1270_e1810) + (locals.var_rs1 * (p.p76 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })));
        locals.var_rs_t_dn4 = (locals.var_rs1_dn4 * assign1270_e1810);
        locals.var_rs_t_dn5 = (locals.var_rs1_dn5 * assign1270_e1810);
        locals.var_rs_t_dn8 = (locals.var_rs1_dn8 * assign1270_e1810);
        locals.var_rs_t_dn10 = (locals.var_rs1_dn10 * assign1270_e1810);
        locals.var_rs_t_dn12 = (locals.var_rs1_dn12 * assign1270_e1810);
        locals.var_rs_t_db1 = (locals.var_rs1_db1 * assign1270_e1810);
        locals.var_rs_t_rv = 0.0;
        locals.var_rs_t_rdb1 = 0.0;

        let assign1280_e1816: f64 = (locals.var_delta_t).abs();
        let assign1280_e1817: f64 = (p.p76 * assign1280_e1816);
        let assign1280_e1818: f64 = (1.0 + assign1280_e1817);
        let assign1280_e1819: f64 = (locals.var_rd1 * assign1280_e1818);
        locals.var_rd1_t = assign1280_e1819;
        locals.var_rd1_t_dn3 = ((locals.var_rd1_dn3 * assign1280_e1818) + (locals.var_rd1 * (p.p76 * if locals.var_delta_t >= 0.0 { locals.var_delta_t_dn3 } else { (-locals.var_delta_t_dn3) })));
        locals.var_rd1_t_dn4 = (locals.var_rd1_dn4 * assign1280_e1818);
        locals.var_rd1_t_dn5 = (locals.var_rd1_dn5 * assign1280_e1818);
        locals.var_rd1_t_dn8 = (locals.var_rd1_dn8 * assign1280_e1818);
        locals.var_rd1_t_dn10 = (locals.var_rd1_dn10 * assign1280_e1818);
        locals.var_rd1_t_dn12 = (locals.var_rd1_dn12 * assign1280_e1818);
        locals.var_rd1_t_db1 = (locals.var_rd1_db1 * assign1280_e1818);
        locals.var_rd1_t_rv = 0.0;
        locals.var_rd1_t_rdb1 = 0.0;

        let assign1300_e1830: f64 = if p.p5 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign1300_e1830;
        locals.var_guard12_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1310_e1841, assign1310_e1841_d_n3, assign1310_e1841_d_n4, assign1310_e1841_d_n5, assign1310_e1841_d_n8, assign1310_e1841_d_n10, assign1310_e1841_d_n12, assign1310_e1841_d_b1,) = {
    if (locals.var_guard12 != 0.0) {
        let assign1310_e1834: f64 = (-1.0);
        let assign1310_e1836: f64 = (assign1310_e1834 * locals.var_vjg_t);
        let assign1310_e1837: f64 = (assign1310_e1836).tanh();
        let assign1310_e1838: f64 = (locals.var_pg_param * assign1310_e1837);
        let assign1310_e1839: f64 = { let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1310_e1839, ({ let limexp_arg = assign1310_e1838; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * ((locals.var_pg_param_dn3 * assign1310_e1837) + (locals.var_pg_param * ((assign1310_e1834 * locals.var_vjg_t_dn3) / ((assign1310_e1836).cosh() * (assign1310_e1836).cosh()))))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign1310_e1841;
        locals.var_t0_dn3 = assign1310_e1841_d_n3;
        locals.var_t0_dn4 = assign1310_e1841_d_n4;
        locals.var_t0_dn5 = assign1310_e1841_d_n5;
        locals.var_t0_dn8 = assign1310_e1841_d_n8;
        locals.var_t0_dn10 = assign1310_e1841_d_n10;
        locals.var_t0_dn12 = assign1310_e1841_d_n12;
        locals.var_t0_db1 = assign1310_e1841_d_b1;
        locals.var_t0_rv = 0.0;
        locals.var_t0_rdb1 = 0.0;

        let (assign1360_e1880, assign1360_e1880_d_n3, assign1360_e1880_d_n4, assign1360_e1880_d_n5, assign1360_e1880_d_n8, assign1360_e1880_d_n10, assign1360_e1880_d_n12, assign1360_e1880_d_b1,) = {
    if (locals.var_guard12 == 0.0) {
        let assign1360_e1875: f64 = (-locals.var_pg_param);
        let assign1360_e1877: f64 = (assign1360_e1875 * locals.var_vjg_t);
        let assign1360_e1878: f64 = { let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX * (1.0 + (limexp_arg - 80.0)) } };
        (assign1360_e1878, ({ let limexp_arg = assign1360_e1877; if limexp_arg < 80.0 { limexp_arg.exp() } else { LIMEXP_MAX } } * (((-locals.var_pg_param_dn3) * locals.var_vjg_t) + (assign1360_e1875 * locals.var_vjg_t_dn3))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn12, locals.var_t0_db1,)
    }
};
        locals.var_t0 = assign1360_e1880;
        locals.var_t0_dn3 = assign1360_e1880_d_n3;
        locals.var_t0_dn4 = assign1360_e1880_d_n4;
        locals.var_t0_dn5 = assign1360_e1880_d_n5;
        locals.var_t0_dn8 = assign1360_e1880_d_n8;
        locals.var_t0_dn10 = assign1360_e1880_d_n10;
        locals.var_t0_dn12 = assign1360_e1880_d_n12;
        locals.var_t0_db1 = assign1360_e1880_d_b1;
        locals.var_t0_rv = 0.0;
        locals.var_t0_rdb1 = 0.0;

        let assign1500_e2001: f64 = (p.p31 * locals.var_vgsc);
        let assign1500_e2002: f64 = (locals.var_p10_t + assign1500_e2001);
        let assign1500_e2005: f64 = (p.p38 * locals.var_vds);
        let assign1500_e2006: f64 = (assign1500_e2002 + assign1500_e2005);
        locals.var_psi_1 = assign1500_e2006;
        locals.var_psi_1_dn3 = locals.var_p10_t_dn3;
        locals.var_psi_1_dn5 = (p.p38 * locals.var_vds_dn5);
        locals.var_psi_1_dn8 = ((p.p31 * locals.var_vgsc_dn8) + (p.p38 * locals.var_vds_dn8));
        locals.var_psi_1_dn11 = (p.p31 * locals.var_vgsc_dn11);
        locals.var_psi_1_rv = 0.0;

        let assign1510_e2009: f64 = (locals.var_psi_1).tanh();
        let assign1510_e2010: f64 = (1.0 + assign1510_e2009);
        locals.var_tanh1 = assign1510_e2010;
        locals.var_tanh1_dn3 = (locals.var_psi_1_dn3 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn5 = (locals.var_psi_1_dn5 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn8 = (locals.var_psi_1_dn8 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_dn11 = (locals.var_psi_1_dn11 / ((locals.var_psi_1).cosh() * (locals.var_psi_1).cosh()));
        locals.var_tanh1_rv = 0.0;

        let assign1520_e2014: f64 = (p.p33 * locals.var_vds);
        let assign1520_e2015: f64 = (p.p32 + assign1520_e2014);
        locals.var_psi_2 = assign1520_e2015;
        locals.var_psi_2_dn5 = (p.p33 * locals.var_vds_dn5);
        locals.var_psi_2_dn8 = (p.p33 * locals.var_vds_dn8);
        locals.var_psi_2_rv = 0.0;

        let assign1530_e2018: f64 = (locals.var_psi_2).tanh();
        let assign1530_e2019: f64 = (1.0 + assign1530_e2018);
        locals.var_tanh2 = assign1530_e2019;
        locals.var_tanh2_dn5 = (locals.var_psi_2_dn5 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh()));
        locals.var_tanh2_dn8 = (locals.var_psi_2_dn8 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh()));
        locals.var_tanh2_rv = 0.0;

        let assign1540_e2023: f64 = (p.p35 * locals.var_vds);
        let assign1540_e2024: f64 = (p.p34 - assign1540_e2023);
        locals.var_psi_3 = assign1540_e2024;
        locals.var_psi_3_dn5 = (-(p.p35 * locals.var_vds_dn5));
        locals.var_psi_3_dn8 = (-(p.p35 * locals.var_vds_dn8));
        locals.var_psi_3_rv = 0.0;

        let assign1550_e2027: f64 = (locals.var_psi_3).tanh();
        let assign1550_e2028: f64 = (1.0 + assign1550_e2027);
        let assign1550_e2030: f64 = (assign1550_e2028 - p.p38);
        locals.var_tanh3 = assign1550_e2030;
        locals.var_tanh3_dn5 = (locals.var_psi_3_dn5 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh()));
        locals.var_tanh3_dn8 = (locals.var_psi_3_dn8 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh()));
        locals.var_tanh3_rv = 0.0;

        let assign1560_e2034: f64 = (p.p37 * locals.var_vgdc);
        let assign1560_e2035: f64 = (locals.var_p40_t + assign1560_e2034);
        let assign1560_e2038: f64 = (p.p38 * locals.var_vds);
        let assign1560_e2039: f64 = (assign1560_e2035 - assign1560_e2038);
        locals.var_psi_4 = assign1560_e2039;
        locals.var_psi_4_dn3 = locals.var_p40_t_dn3;
        locals.var_psi_4_dn5 = ((p.p37 * locals.var_vgdc_dn5) - (p.p38 * locals.var_vds_dn5));
        locals.var_psi_4_dn8 = (-(p.p38 * locals.var_vds_dn8));
        locals.var_psi_4_dn10 = (p.p37 * locals.var_vgdc_dn10);
        locals.var_psi_4_rv = 0.0;

        let assign1570_e2042: f64 = (locals.var_psi_4).tanh();
        let assign1570_e2043: f64 = (1.0 + assign1570_e2042);
        locals.var_tanh4 = assign1570_e2043;
        locals.var_tanh4_dn3 = (locals.var_psi_4_dn3 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn5 = (locals.var_psi_4_dn5 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn8 = (locals.var_psi_4_dn8 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_dn10 = (locals.var_psi_4_dn10 / ((locals.var_psi_4).cosh() * (locals.var_psi_4).cosh()));
        locals.var_tanh4_rv = 0.0;

        let assign1580_e2046: f64 = if p.p6 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign1580_e2046;
        locals.var_guard14_rv = 0.0;

        let assign1590_e2049: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign1590_e2049;
        locals.var_guard15_rv = 0.0;

        let assign1600_e2052: f64 = if p.p6 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign1600_e2052;
        locals.var_guard16_rv = 0.0;

        let assign1610_e2055: f64 = if p.p6 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign1610_e2055;
        locals.var_guard17_rv = 0.0;

        let assign1620_e2058: f64 = if p.p6 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign1620_e2058;
        locals.var_guard18_rv = 0.0;

        let (assign1630_e2062, assign1630_e2062_d_n3, assign1630_e2062_d_n5, assign1630_e2062_d_n8, assign1630_e2062_d_n10, assign1630_e2062_d_n11,) = {
    if (locals.var_guard14 != 0.0) {
        (p.p25, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1630_e2062;
        locals.var_cgs_dn3 = assign1630_e2062_d_n3;
        locals.var_cgs_dn5 = assign1630_e2062_d_n5;
        locals.var_cgs_dn8 = assign1630_e2062_d_n8;
        locals.var_cgs_dn10 = assign1630_e2062_d_n10;
        locals.var_cgs_dn11 = assign1630_e2062_d_n11;
        locals.var_cgs_rv = 0.0;

        let (assign1640_e2066, assign1640_e2066_d_n3, assign1640_e2066_d_n5, assign1640_e2066_d_n8, assign1640_e2066_d_n10, assign1640_e2066_d_n11,) = {
    if (locals.var_guard14 != 0.0) {
        (p.p27, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1640_e2066;
        locals.var_cgd_dn3 = assign1640_e2066_d_n3;
        locals.var_cgd_dn5 = assign1640_e2066_d_n5;
        locals.var_cgd_dn8 = assign1640_e2066_d_n8;
        locals.var_cgd_dn10 = assign1640_e2066_d_n10;
        locals.var_cgd_dn11 = assign1640_e2066_d_n11;
        locals.var_cgd_rv = 0.0;

        let (assign1650_e2079, assign1650_e2079_d_n3, assign1650_e2079_d_n5, assign1650_e2079_d_n8, assign1650_e2079_d_n10, assign1650_e2079_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (locals.var_guard14 == 0.0)) {
        let assign1650_e2074: f64 = (locals.var_cgs0_t * locals.var_tanh1);
        let assign1650_e2076: f64 = (assign1650_e2074 * locals.var_tanh2);
        let assign1650_e2077: f64 = (p.p25 + assign1650_e2076);
        (assign1650_e2077, (((locals.var_cgs0_t_dn3 * locals.var_tanh1) + (locals.var_cgs0_t * locals.var_tanh1_dn3)) * locals.var_tanh2), (((locals.var_cgs0_t * locals.var_tanh1_dn5) * locals.var_tanh2) + (assign1650_e2074 * locals.var_tanh2_dn5)), (((locals.var_cgs0_t * locals.var_tanh1_dn8) * locals.var_tanh2) + (assign1650_e2074 * locals.var_tanh2_dn8)), 0.0, ((locals.var_cgs0_t * locals.var_tanh1_dn11) * locals.var_tanh2),)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1650_e2079;
        locals.var_cgs_dn3 = assign1650_e2079_d_n3;
        locals.var_cgs_dn5 = assign1650_e2079_d_n5;
        locals.var_cgs_dn8 = assign1650_e2079_d_n8;
        locals.var_cgs_dn10 = assign1650_e2079_d_n10;
        locals.var_cgs_dn11 = assign1650_e2079_d_n11;
        locals.var_cgs_rv = 0.0;

        let (assign1660_e2096, assign1660_e2096_d_n3, assign1660_e2096_d_n5, assign1660_e2096_d_n8, assign1660_e2096_d_n10, assign1660_e2096_d_n11,) = {
    if ((locals.var_guard15 != 0.0) && (locals.var_guard14 == 0.0)) {
        let assign1660_e2088: f64 = (locals.var_tanh3 * locals.var_tanh4);
        let assign1660_e2091: f64 = (2.0 * p.p38);
        let assign1660_e2092: f64 = (assign1660_e2088 + assign1660_e2091);
        let assign1660_e2093: f64 = (locals.var_cgd0_t * assign1660_e2092);
        let assign1660_e2094: f64 = (p.p27 + assign1660_e2093);
        (assign1660_e2094, ((locals.var_cgd0_t_dn3 * assign1660_e2092) + (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn3))), (locals.var_cgd0_t * ((locals.var_tanh3_dn5 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn5))), (locals.var_cgd0_t * ((locals.var_tanh3_dn8 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn8))), (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn10)), 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1660_e2096;
        locals.var_cgd_dn3 = assign1660_e2096_d_n3;
        locals.var_cgd_dn5 = assign1660_e2096_d_n5;
        locals.var_cgd_dn8 = assign1660_e2096_d_n8;
        locals.var_cgd_dn10 = assign1660_e2096_d_n10;
        locals.var_cgd_dn11 = assign1660_e2096_d_n11;
        locals.var_cgd_rv = 0.0;

        let (assign1670_e2107, assign1670_e2107_d_n5, assign1670_e2107_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1670_e2105: f64 = (locals.var_tanh2 - p.p38);
        (assign1670_e2105, locals.var_tanh2_dn5, locals.var_tanh2_dn8,)
    } else {
        (locals.var_tanh2, locals.var_tanh2_dn5, locals.var_tanh2_dn8,)
    }
};
        locals.var_tanh2 = assign1670_e2107;
        locals.var_tanh2_dn5 = assign1670_e2107_d_n5;
        locals.var_tanh2_dn8 = assign1670_e2107_d_n8;
        locals.var_tanh2_rv = 0.0;

        let (assign1680_e2121, assign1680_e2121_d_n3, assign1680_e2121_d_n5, assign1680_e2121_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1680_e2117: f64 = (p.p38 * locals.var_vds);
        let assign1680_e2118: f64 = (locals.var_p10_t + assign1680_e2117);
        let assign1680_e2119: f64 = (assign1680_e2118).cosh();
        (assign1680_e2119, ((assign1680_e2118).sinh() * locals.var_p10_t_dn3), ((assign1680_e2118).sinh() * (p.p38 * locals.var_vds_dn5)), ((assign1680_e2118).sinh() * (p.p38 * locals.var_vds_dn8)),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn8,)
    }
};
        locals.var_cosh0 = assign1680_e2121;
        locals.var_cosh0_dn3 = assign1680_e2121_d_n3;
        locals.var_cosh0_dn5 = assign1680_e2121_d_n5;
        locals.var_cosh0_dn8 = assign1680_e2121_d_n8;
        locals.var_cosh0_rv = 0.0;

        let (assign1690_e2131, assign1690_e2131_d_n3, assign1690_e2131_d_n5, assign1690_e2131_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1690_e2129: f64 = (locals.var_cosh0).ln();
        (assign1690_e2129, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn8 / locals.var_cosh0),)
    } else {
        (locals.var_lc10, locals.var_lc10_dn3, locals.var_lc10_dn5, locals.var_lc10_dn8,)
    }
};
        locals.var_lc10 = assign1690_e2131;
        locals.var_lc10_dn3 = assign1690_e2131_d_n3;
        locals.var_lc10_dn5 = assign1690_e2131_d_n5;
        locals.var_lc10_dn8 = assign1690_e2131_d_n8;
        locals.var_lc10_rv = 0.0;

        let (assign1700_e2141, assign1700_e2141_d_n3, assign1700_e2141_d_n5, assign1700_e2141_d_n8, assign1700_e2141_d_n10, assign1700_e2141_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1700_e2139: f64 = (locals.var_psi_1).cosh();
        (assign1700_e2139, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn3), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn5), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn8), 0.0, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn11),)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn8, locals.var_cosh1_dn10, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1700_e2141;
        locals.var_cosh1_dn3 = assign1700_e2141_d_n3;
        locals.var_cosh1_dn5 = assign1700_e2141_d_n5;
        locals.var_cosh1_dn8 = assign1700_e2141_d_n8;
        locals.var_cosh1_dn10 = assign1700_e2141_d_n10;
        locals.var_cosh1_dn11 = assign1700_e2141_d_n11;
        locals.var_cosh1_rv = 0.0;

        let (assign1710_e2151, assign1710_e2151_d_n3, assign1710_e2151_d_n5, assign1710_e2151_d_n8, assign1710_e2151_d_n10, assign1710_e2151_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1710_e2149: f64 = (locals.var_cosh1).ln();
        (assign1710_e2149, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn10 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc1, locals.var_lc1_dn3, locals.var_lc1_dn5, locals.var_lc1_dn8, locals.var_lc1_dn10, locals.var_lc1_dn11,)
    }
};
        locals.var_lc1 = assign1710_e2151;
        locals.var_lc1_dn3 = assign1710_e2151_d_n3;
        locals.var_lc1_dn5 = assign1710_e2151_d_n5;
        locals.var_lc1_dn8 = assign1710_e2151_d_n8;
        locals.var_lc1_dn10 = assign1710_e2151_d_n10;
        locals.var_lc1_dn11 = assign1710_e2151_d_n11;
        locals.var_lc1_rv = 0.0;

        let (assign1720_e2166, assign1720_e2166_d_n3, assign1720_e2166_d_n5, assign1720_e2166_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1720_e2161: f64 = (p.p38 * locals.var_vds);
        let assign1720_e2162: f64 = (locals.var_p10_t + assign1720_e2161);
        let assign1720_e2164: f64 = (assign1720_e2162 + locals.var_lc10);
        (assign1720_e2164, (locals.var_p10_t_dn3 + locals.var_lc10_dn3), ((p.p38 * locals.var_vds_dn5) + locals.var_lc10_dn5), ((p.p38 * locals.var_vds_dn8) + locals.var_lc10_dn8),)
    } else {
        (locals.var_qgs0, locals.var_qgs0_dn3, locals.var_qgs0_dn5, locals.var_qgs0_dn8,)
    }
};
        locals.var_qgs0 = assign1720_e2166;
        locals.var_qgs0_dn3 = assign1720_e2166_d_n3;
        locals.var_qgs0_dn5 = assign1720_e2166_d_n5;
        locals.var_qgs0_dn8 = assign1720_e2166_d_n8;
        locals.var_qgs0_rv = 0.0;

        let (assign1730_e2195, assign1730_e2195_d_n3, assign1730_e2195_d_n5, assign1730_e2195_d_n8, assign1730_e2195_d_n10, assign1730_e2195_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1730_e2176: f64 = (locals.var_psi_1 + locals.var_lc1);
        let assign1730_e2178: f64 = (assign1730_e2176 - locals.var_qgs0);
        let assign1730_e2180: f64 = (assign1730_e2178 * locals.var_tanh2);
        let assign1730_e2182: f64 = (assign1730_e2180 / p.p31);
        let assign1730_e2185: f64 = (2.0 * p.p38);
        let assign1730_e2187: f64 = (assign1730_e2185 * locals.var_vgsc);
        let assign1730_e2188: f64 = (assign1730_e2182 + assign1730_e2187);
        let assign1730_e2189: f64 = (locals.var_cgs0_t * assign1730_e2188);
        let assign1730_e2192: f64 = (p.p25 * locals.var_vgsc);
        let assign1730_e2193: f64 = (assign1730_e2189 + assign1730_e2192);
        (assign1730_e2193, ((locals.var_cgs0_t_dn3 * assign1730_e2188) + (locals.var_cgs0_t * ((((locals.var_psi_1_dn3 + locals.var_lc1_dn3) - locals.var_qgs0_dn3) * locals.var_tanh2) / p.p31))), (locals.var_cgs0_t * (((((locals.var_psi_1_dn5 + locals.var_lc1_dn5) - locals.var_qgs0_dn5) * locals.var_tanh2) + (assign1730_e2178 * locals.var_tanh2_dn5)) / p.p31)), ((locals.var_cgs0_t * ((((((locals.var_psi_1_dn8 + locals.var_lc1_dn8) - locals.var_qgs0_dn8) * locals.var_tanh2) + (assign1730_e2178 * locals.var_tanh2_dn8)) / p.p31) + (assign1730_e2185 * locals.var_vgsc_dn8))) + (p.p25 * locals.var_vgsc_dn8)), (locals.var_cgs0_t * ((locals.var_lc1_dn10 * locals.var_tanh2) / p.p31)), ((locals.var_cgs0_t * ((((locals.var_psi_1_dn11 + locals.var_lc1_dn11) * locals.var_tanh2) / p.p31) + (assign1730_e2185 * locals.var_vgsc_dn11))) + (p.p25 * locals.var_vgsc_dn11)),)
    } else {
        (locals.var_qgs, locals.var_qgs_dn3, locals.var_qgs_dn5, locals.var_qgs_dn8, locals.var_qgs_dn10, locals.var_qgs_dn11,)
    }
};
        locals.var_qgs = assign1730_e2195;
        locals.var_qgs_dn3 = assign1730_e2195_d_n3;
        locals.var_qgs_dn5 = assign1730_e2195_d_n5;
        locals.var_qgs_dn8 = assign1730_e2195_d_n8;
        locals.var_qgs_dn10 = assign1730_e2195_d_n10;
        locals.var_qgs_dn11 = assign1730_e2195_d_n11;
        locals.var_qgs_rv = 0.0;

        let (assign1740_e2209, assign1740_e2209_d_n3, assign1740_e2209_d_n5, assign1740_e2209_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1740_e2205: f64 = (p.p38 * locals.var_vds);
        let assign1740_e2206: f64 = (locals.var_p40_t - assign1740_e2205);
        let assign1740_e2207: f64 = (assign1740_e2206).cosh();
        (assign1740_e2207, ((assign1740_e2206).sinh() * locals.var_p40_t_dn3), ((assign1740_e2206).sinh() * (-(p.p38 * locals.var_vds_dn5))), ((assign1740_e2206).sinh() * (-(p.p38 * locals.var_vds_dn8))),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn8,)
    }
};
        locals.var_cosh0 = assign1740_e2209;
        locals.var_cosh0_dn3 = assign1740_e2209_d_n3;
        locals.var_cosh0_dn5 = assign1740_e2209_d_n5;
        locals.var_cosh0_dn8 = assign1740_e2209_d_n8;
        locals.var_cosh0_rv = 0.0;

        let (assign1750_e2219, assign1750_e2219_d_n3, assign1750_e2219_d_n5, assign1750_e2219_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1750_e2217: f64 = (locals.var_cosh0).ln();
        (assign1750_e2217, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn8 / locals.var_cosh0),)
    } else {
        (locals.var_lc40, locals.var_lc40_dn3, locals.var_lc40_dn5, locals.var_lc40_dn8,)
    }
};
        locals.var_lc40 = assign1750_e2219;
        locals.var_lc40_dn3 = assign1750_e2219_d_n3;
        locals.var_lc40_dn5 = assign1750_e2219_d_n5;
        locals.var_lc40_dn8 = assign1750_e2219_d_n8;
        locals.var_lc40_rv = 0.0;

        let (assign1760_e2229, assign1760_e2229_d_n3, assign1760_e2229_d_n5, assign1760_e2229_d_n8, assign1760_e2229_d_n10, assign1760_e2229_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1760_e2227: f64 = (locals.var_psi_4).cosh();
        (assign1760_e2227, ((locals.var_psi_4).sinh() * locals.var_psi_4_dn3), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn5), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn8), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn10), 0.0,)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn8, locals.var_cosh1_dn10, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1760_e2229;
        locals.var_cosh1_dn3 = assign1760_e2229_d_n3;
        locals.var_cosh1_dn5 = assign1760_e2229_d_n5;
        locals.var_cosh1_dn8 = assign1760_e2229_d_n8;
        locals.var_cosh1_dn10 = assign1760_e2229_d_n10;
        locals.var_cosh1_dn11 = assign1760_e2229_d_n11;
        locals.var_cosh1_rv = 0.0;

        let (assign1770_e2239, assign1770_e2239_d_n3, assign1770_e2239_d_n5, assign1770_e2239_d_n8, assign1770_e2239_d_n10, assign1770_e2239_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1770_e2237: f64 = (locals.var_cosh1).ln();
        (assign1770_e2237, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn10 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc4, locals.var_lc4_dn3, locals.var_lc4_dn5, locals.var_lc4_dn8, locals.var_lc4_dn10, locals.var_lc4_dn11,)
    }
};
        locals.var_lc4 = assign1770_e2239;
        locals.var_lc4_dn3 = assign1770_e2239_d_n3;
        locals.var_lc4_dn5 = assign1770_e2239_d_n5;
        locals.var_lc4_dn8 = assign1770_e2239_d_n8;
        locals.var_lc4_dn10 = assign1770_e2239_d_n10;
        locals.var_lc4_dn11 = assign1770_e2239_d_n11;
        locals.var_lc4_rv = 0.0;

        let (assign1780_e2254, assign1780_e2254_d_n3, assign1780_e2254_d_n5, assign1780_e2254_d_n8,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1780_e2249: f64 = (p.p38 * locals.var_vds);
        let assign1780_e2250: f64 = (locals.var_p40_t - assign1780_e2249);
        let assign1780_e2252: f64 = (assign1780_e2250 + locals.var_lc40);
        (assign1780_e2252, (locals.var_p40_t_dn3 + locals.var_lc40_dn3), ((-(p.p38 * locals.var_vds_dn5)) + locals.var_lc40_dn5), ((-(p.p38 * locals.var_vds_dn8)) + locals.var_lc40_dn8),)
    } else {
        (locals.var_qgd0, locals.var_qgd0_dn3, locals.var_qgd0_dn5, locals.var_qgd0_dn8,)
    }
};
        locals.var_qgd0 = assign1780_e2254;
        locals.var_qgd0_dn3 = assign1780_e2254_d_n3;
        locals.var_qgd0_dn5 = assign1780_e2254_d_n5;
        locals.var_qgd0_dn8 = assign1780_e2254_d_n8;
        locals.var_qgd0_rv = 0.0;

        let (assign1790_e2283, assign1790_e2283_d_n3, assign1790_e2283_d_n5, assign1790_e2283_d_n8, assign1790_e2283_d_n10, assign1790_e2283_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1790_e2264: f64 = (locals.var_psi_4 + locals.var_lc4);
        let assign1790_e2266: f64 = (assign1790_e2264 - locals.var_qgd0);
        let assign1790_e2268: f64 = (assign1790_e2266 * locals.var_tanh3);
        let assign1790_e2270: f64 = (assign1790_e2268 / p.p37);
        let assign1790_e2273: f64 = (2.0 * p.p38);
        let assign1790_e2275: f64 = (assign1790_e2273 * locals.var_vgdc);
        let assign1790_e2276: f64 = (assign1790_e2270 + assign1790_e2275);
        let assign1790_e2277: f64 = (locals.var_cgd0_t * assign1790_e2276);
        let assign1790_e2280: f64 = (p.p27 * locals.var_vgdc);
        let assign1790_e2281: f64 = (assign1790_e2277 + assign1790_e2280);
        (assign1790_e2281, ((locals.var_cgd0_t_dn3 * assign1790_e2276) + (locals.var_cgd0_t * ((((locals.var_psi_4_dn3 + locals.var_lc4_dn3) - locals.var_qgd0_dn3) * locals.var_tanh3) / p.p37))), ((locals.var_cgd0_t * ((((((locals.var_psi_4_dn5 + locals.var_lc4_dn5) - locals.var_qgd0_dn5) * locals.var_tanh3) + (assign1790_e2266 * locals.var_tanh3_dn5)) / p.p37) + (assign1790_e2273 * locals.var_vgdc_dn5))) + (p.p27 * locals.var_vgdc_dn5)), (locals.var_cgd0_t * (((((locals.var_psi_4_dn8 + locals.var_lc4_dn8) - locals.var_qgd0_dn8) * locals.var_tanh3) + (assign1790_e2266 * locals.var_tanh3_dn8)) / p.p37)), ((locals.var_cgd0_t * ((((locals.var_psi_4_dn10 + locals.var_lc4_dn10) * locals.var_tanh3) / p.p37) + (assign1790_e2273 * locals.var_vgdc_dn10))) + (p.p27 * locals.var_vgdc_dn10)), (locals.var_cgd0_t * ((locals.var_lc4_dn11 * locals.var_tanh3) / p.p37)),)
    } else {
        (locals.var_qgd, locals.var_qgd_dn3, locals.var_qgd_dn5, locals.var_qgd_dn8, locals.var_qgd_dn10, locals.var_qgd_dn11,)
    }
};
        locals.var_qgd = assign1790_e2283;
        locals.var_qgd_dn3 = assign1790_e2283_d_n3;
        locals.var_qgd_dn5 = assign1790_e2283_d_n5;
        locals.var_qgd_dn8 = assign1790_e2283_d_n8;
        locals.var_qgd_dn10 = assign1790_e2283_d_n10;
        locals.var_qgd_dn11 = assign1790_e2283_d_n11;
        locals.var_qgd_rv = 0.0;

        let (assign1800_e2294, assign1800_e2294_d_n3, assign1800_e2294_d_n5, assign1800_e2294_d_n8, assign1800_e2294_d_n10, assign1800_e2294_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1800_e2292: f64 = locals.var_qgs_dn11;
        (assign1800_e2292, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1800_e2294;
        locals.var_cgs_dn3 = assign1800_e2294_d_n3;
        locals.var_cgs_dn5 = assign1800_e2294_d_n5;
        locals.var_cgs_dn8 = assign1800_e2294_d_n8;
        locals.var_cgs_dn10 = assign1800_e2294_d_n10;
        locals.var_cgs_dn11 = assign1800_e2294_d_n11;
        locals.var_cgs_rv = 0.0;

        let (assign1810_e2305, assign1810_e2305_d_n3, assign1810_e2305_d_n5, assign1810_e2305_d_n8, assign1810_e2305_d_n10, assign1810_e2305_d_n11,) = {
    if ((locals.var_guard16 != 0.0) && (!((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)))) {
        let assign1810_e2303: f64 = locals.var_qgd_dn10;
        (assign1810_e2303, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1810_e2305;
        locals.var_cgd_dn3 = assign1810_e2305_d_n3;
        locals.var_cgd_dn5 = assign1810_e2305_d_n5;
        locals.var_cgd_dn8 = assign1810_e2305_d_n8;
        locals.var_cgd_dn10 = assign1810_e2305_d_n10;
        locals.var_cgd_dn11 = assign1810_e2305_d_n11;
        locals.var_cgd_rv = 0.0;

        let (assign1820_e2320, assign1820_e2320_d_n8, assign1820_e2320_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1820_e2316: f64 = (locals.var_vgsc / p.p40);
        let assign1820_e2318: f64 = (assign1820_e2316 - 1.0);
        (assign1820_e2318, (locals.var_vgsc_dn8 / p.p40), (locals.var_vgsc_dn11 / p.p40),)
    } else {
        (locals.var_y, locals.var_y_dn8, locals.var_y_dn11,)
    }
};
        locals.var_y = assign1820_e2320;
        locals.var_y_dn8 = assign1820_e2320_d_n8;
        locals.var_y_dn11 = assign1820_e2320_d_n11;
        locals.var_y_rv = 0.0;

        let (assign1830_e2331,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        (0.5,)
    } else {
        (locals.var_mjc,)
    }
};
        locals.var_mjc = assign1830_e2331;
        locals.var_mjc_rv = 0.0;

        let (assign1840_e2363, assign1840_e2363_d_n8, assign1840_e2363_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1840_e2343: f64 = (locals.var_y * locals.var_y);
        let assign1840_e2344: f64 = (p.p41 + assign1840_e2343);
        let assign1840_e2346: f64 = (-1.0);
        let assign1840_e2348: f64 = (assign1840_e2346 - locals.var_mjc);
        let assign1840_e2349: f64 = (assign1840_e2344).powf(assign1840_e2348);
        let assign1840_e2354: f64 = (2.0 * locals.var_mjc);
        let assign1840_e2355: f64 = (1.0 - assign1840_e2354);
        let assign1840_e2358: f64 = (locals.var_y * locals.var_y);
        let assign1840_e2359: f64 = (assign1840_e2355 * assign1840_e2358);
        let assign1840_e2360: f64 = (p.p41 + assign1840_e2359);
        let assign1840_e2361: f64 = (assign1840_e2349 * assign1840_e2360);
        (assign1840_e2361, ((if 0.0 == 0.0 && ((assign1840_e2348) as f64).is_finite() && ((assign1840_e2348) as f64).fract() == 0.0 { if assign1840_e2348 == 0.0 { 0.0 } else { (assign1840_e2348 * ((assign1840_e2344).powf(assign1840_e2348 - 1.0) * ((locals.var_y_dn8 * locals.var_y) + (locals.var_y * locals.var_y_dn8)))) } } else { (assign1840_e2349 * (assign1840_e2348 * (((locals.var_y_dn8 * locals.var_y) + (locals.var_y * locals.var_y_dn8)) / assign1840_e2344))) } * assign1840_e2360) + (assign1840_e2349 * (assign1840_e2355 * ((locals.var_y_dn8 * locals.var_y) + (locals.var_y * locals.var_y_dn8))))), ((if 0.0 == 0.0 && ((assign1840_e2348) as f64).is_finite() && ((assign1840_e2348) as f64).fract() == 0.0 { if assign1840_e2348 == 0.0 { 0.0 } else { (assign1840_e2348 * ((assign1840_e2344).powf(assign1840_e2348 - 1.0) * ((locals.var_y_dn11 * locals.var_y) + (locals.var_y * locals.var_y_dn11)))) } } else { (assign1840_e2349 * (assign1840_e2348 * (((locals.var_y_dn11 * locals.var_y) + (locals.var_y * locals.var_y_dn11)) / assign1840_e2344))) } * assign1840_e2360) + (assign1840_e2349 * (assign1840_e2355 * ((locals.var_y_dn11 * locals.var_y) + (locals.var_y * locals.var_y_dn11))))),)
    } else {
        (locals.var_cgsdepl, locals.var_cgsdepl_dn8, locals.var_cgsdepl_dn11,)
    }
};
        locals.var_cgsdepl = assign1840_e2363;
        locals.var_cgsdepl_dn8 = assign1840_e2363_d_n8;
        locals.var_cgsdepl_dn11 = assign1840_e2363_d_n11;
        locals.var_cgsdepl_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        branches: &[usize; Instance::BRANCH_COUNT],
        locals: &mut StampLocals,
    ) {
        let bi1 = ctx.branch_current(branches[1]);
        let (assign1850_e2385, assign1850_e2385_d_n3, assign1850_e2385_d_n5, assign1850_e2385_d_n8, assign1850_e2385_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1850_e2378: f64 = (p.p38 * locals.var_vds);
        let assign1850_e2379: f64 = (locals.var_vgsc + assign1850_e2378);
        let assign1850_e2380: f64 = (p.p31 * assign1850_e2379);
        let assign1850_e2381: f64 = (locals.var_p10_t + assign1850_e2380);
        let assign1850_e2382: f64 = (assign1850_e2381).tanh();
        let assign1850_e2383: f64 = (1.0 + assign1850_e2382);
        (assign1850_e2383, (locals.var_p10_t_dn3 / ((assign1850_e2381).cosh() * (assign1850_e2381).cosh())), ((p.p31 * (p.p38 * locals.var_vds_dn5)) / ((assign1850_e2381).cosh() * (assign1850_e2381).cosh())), ((p.p31 * (locals.var_vgsc_dn8 + (p.p38 * locals.var_vds_dn8))) / ((assign1850_e2381).cosh() * (assign1850_e2381).cosh())), ((p.p31 * locals.var_vgsc_dn11) / ((assign1850_e2381).cosh() * (assign1850_e2381).cosh())),)
    } else {
        (locals.var_tanh1, locals.var_tanh1_dn3, locals.var_tanh1_dn5, locals.var_tanh1_dn8, locals.var_tanh1_dn11,)
    }
};
        locals.var_tanh1 = assign1850_e2385;
        locals.var_tanh1_dn3 = assign1850_e2385_d_n3;
        locals.var_tanh1_dn5 = assign1850_e2385_d_n5;
        locals.var_tanh1_dn8 = assign1850_e2385_d_n8;
        locals.var_tanh1_dn11 = assign1850_e2385_d_n11;
        locals.var_tanh1_rv = 0.0;

        let (assign1860_e2403, assign1860_e2403_d_n5, assign1860_e2403_d_n8,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1860_e2398: f64 = (p.p33 * locals.var_vds);
        let assign1860_e2399: f64 = (p.p32 + assign1860_e2398);
        let assign1860_e2400: f64 = (assign1860_e2399).tanh();
        let assign1860_e2401: f64 = (1.0 + assign1860_e2400);
        (assign1860_e2401, ((p.p33 * locals.var_vds_dn5) / ((assign1860_e2399).cosh() * (assign1860_e2399).cosh())), ((p.p33 * locals.var_vds_dn8) / ((assign1860_e2399).cosh() * (assign1860_e2399).cosh())),)
    } else {
        (locals.var_tanh2, locals.var_tanh2_dn5, locals.var_tanh2_dn8,)
    }
};
        locals.var_tanh2 = assign1860_e2403;
        locals.var_tanh2_dn5 = assign1860_e2403_d_n5;
        locals.var_tanh2_dn8 = assign1860_e2403_d_n8;
        locals.var_tanh2_rv = 0.0;

        let (assign1870_e2423, assign1870_e2423_d_n5, assign1870_e2423_d_n8,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1870_e2414: f64 = (1.0 - p.p38);
        let assign1870_e2418: f64 = (p.p35 * locals.var_vds);
        let assign1870_e2419: f64 = (p.p34 - assign1870_e2418);
        let assign1870_e2420: f64 = (assign1870_e2419).tanh();
        let assign1870_e2421: f64 = (assign1870_e2414 + assign1870_e2420);
        (assign1870_e2421, ((-(p.p35 * locals.var_vds_dn5)) / ((assign1870_e2419).cosh() * (assign1870_e2419).cosh())), ((-(p.p35 * locals.var_vds_dn8)) / ((assign1870_e2419).cosh() * (assign1870_e2419).cosh())),)
    } else {
        (locals.var_tanh3, locals.var_tanh3_dn5, locals.var_tanh3_dn8,)
    }
};
        locals.var_tanh3 = assign1870_e2423;
        locals.var_tanh3_dn5 = assign1870_e2423_d_n5;
        locals.var_tanh3_dn8 = assign1870_e2423_d_n8;
        locals.var_tanh3_rv = 0.0;

        let (assign1880_e2447, assign1880_e2447_d_n3, assign1880_e2447_d_n5, assign1880_e2447_d_n8, assign1880_e2447_d_n10,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1880_e2439: f64 = (1.0 - p.p38);
        let assign1880_e2440: f64 = (locals.var_vds * assign1880_e2439);
        let assign1880_e2441: f64 = (locals.var_vgdc + assign1880_e2440);
        let assign1880_e2442: f64 = (p.p37 * assign1880_e2441);
        let assign1880_e2443: f64 = (locals.var_p40_t + assign1880_e2442);
        let assign1880_e2444: f64 = (assign1880_e2443).tanh();
        let assign1880_e2445: f64 = (1.0 + assign1880_e2444);
        (assign1880_e2445, (locals.var_p40_t_dn3 / ((assign1880_e2443).cosh() * (assign1880_e2443).cosh())), ((p.p37 * (locals.var_vgdc_dn5 + (locals.var_vds_dn5 * assign1880_e2439))) / ((assign1880_e2443).cosh() * (assign1880_e2443).cosh())), ((p.p37 * (locals.var_vds_dn8 * assign1880_e2439)) / ((assign1880_e2443).cosh() * (assign1880_e2443).cosh())), ((p.p37 * locals.var_vgdc_dn10) / ((assign1880_e2443).cosh() * (assign1880_e2443).cosh())),)
    } else {
        (locals.var_tanh4, locals.var_tanh4_dn3, locals.var_tanh4_dn5, locals.var_tanh4_dn8, locals.var_tanh4_dn10,)
    }
};
        locals.var_tanh4 = assign1880_e2447;
        locals.var_tanh4_dn3 = assign1880_e2447_d_n3;
        locals.var_tanh4_dn5 = assign1880_e2447_d_n5;
        locals.var_tanh4_dn8 = assign1880_e2447_d_n8;
        locals.var_tanh4_dn10 = assign1880_e2447_d_n10;
        locals.var_tanh4_rv = 0.0;

        let (assign1890_e2468, assign1890_e2468_d_n3, assign1890_e2468_d_n5, assign1890_e2468_d_n8, assign1890_e2468_d_n10, assign1890_e2468_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1890_e2460: f64 = (p.p39 * locals.var_cgsdepl);
        let assign1890_e2461: f64 = (locals.var_tanh1 + assign1890_e2460);
        let assign1890_e2462: f64 = (locals.var_cgs0_t * assign1890_e2461);
        let assign1890_e2464: f64 = (assign1890_e2462 * locals.var_tanh2);
        let assign1890_e2466: f64 = (assign1890_e2464 + p.p25);
        (assign1890_e2466, (((locals.var_cgs0_t_dn3 * assign1890_e2461) + (locals.var_cgs0_t * locals.var_tanh1_dn3)) * locals.var_tanh2), (((locals.var_cgs0_t * locals.var_tanh1_dn5) * locals.var_tanh2) + (assign1890_e2462 * locals.var_tanh2_dn5)), (((locals.var_cgs0_t * (locals.var_tanh1_dn8 + (p.p39 * locals.var_cgsdepl_dn8))) * locals.var_tanh2) + (assign1890_e2462 * locals.var_tanh2_dn8)), 0.0, ((locals.var_cgs0_t * (locals.var_tanh1_dn11 + (p.p39 * locals.var_cgsdepl_dn11))) * locals.var_tanh2),)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign1890_e2468;
        locals.var_cgs_dn3 = assign1890_e2468_d_n3;
        locals.var_cgs_dn5 = assign1890_e2468_d_n5;
        locals.var_cgs_dn8 = assign1890_e2468_d_n8;
        locals.var_cgs_dn10 = assign1890_e2468_d_n10;
        locals.var_cgs_dn11 = assign1890_e2468_d_n11;
        locals.var_cgs_rv = 0.0;

        let (assign1900_e2489, assign1900_e2489_d_n3, assign1900_e2489_d_n5, assign1900_e2489_d_n8, assign1900_e2489_d_n10, assign1900_e2489_d_n11,) = {
    if ((locals.var_guard17 != 0.0) && (!(((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)))) {
        let assign1900_e2480: f64 = (locals.var_tanh3 * locals.var_tanh4);
        let assign1900_e2483: f64 = (2.0 * p.p38);
        let assign1900_e2484: f64 = (assign1900_e2480 + assign1900_e2483);
        let assign1900_e2485: f64 = (locals.var_cgd0_t * assign1900_e2484);
        let assign1900_e2487: f64 = (assign1900_e2485 + p.p27);
        (assign1900_e2487, ((locals.var_cgd0_t_dn3 * assign1900_e2484) + (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn3))), (locals.var_cgd0_t * ((locals.var_tanh3_dn5 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn5))), (locals.var_cgd0_t * ((locals.var_tanh3_dn8 * locals.var_tanh4) + (locals.var_tanh3 * locals.var_tanh4_dn8))), (locals.var_cgd0_t * (locals.var_tanh3 * locals.var_tanh4_dn10)), 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign1900_e2489;
        locals.var_cgd_dn3 = assign1900_e2489_d_n3;
        locals.var_cgd_dn5 = assign1900_e2489_d_n5;
        locals.var_cgd_dn8 = assign1900_e2489_d_n8;
        locals.var_cgd_dn10 = assign1900_e2489_d_n10;
        locals.var_cgd_dn11 = assign1900_e2489_d_n11;
        locals.var_cgd_rv = 0.0;

        let (assign1910_e2507, assign1910_e2507_d_n3, assign1910_e2507_d_n5, assign1910_e2507_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1910_e2503: f64 = (p.p38 * locals.var_vds);
        let assign1910_e2504: f64 = (locals.var_p10_t + assign1910_e2503);
        let assign1910_e2505: f64 = (assign1910_e2504).cosh();
        (assign1910_e2505, ((assign1910_e2504).sinh() * locals.var_p10_t_dn3), ((assign1910_e2504).sinh() * (p.p38 * locals.var_vds_dn5)), ((assign1910_e2504).sinh() * (p.p38 * locals.var_vds_dn8)),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn8,)
    }
};
        locals.var_cosh0 = assign1910_e2507;
        locals.var_cosh0_dn3 = assign1910_e2507_d_n3;
        locals.var_cosh0_dn5 = assign1910_e2507_d_n5;
        locals.var_cosh0_dn8 = assign1910_e2507_d_n8;
        locals.var_cosh0_rv = 0.0;

        let (assign1920_e2521, assign1920_e2521_d_n3, assign1920_e2521_d_n5, assign1920_e2521_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1920_e2519: f64 = (locals.var_cosh0).ln();
        (assign1920_e2519, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn8 / locals.var_cosh0),)
    } else {
        (locals.var_lc10, locals.var_lc10_dn3, locals.var_lc10_dn5, locals.var_lc10_dn8,)
    }
};
        locals.var_lc10 = assign1920_e2521;
        locals.var_lc10_dn3 = assign1920_e2521_d_n3;
        locals.var_lc10_dn5 = assign1920_e2521_d_n5;
        locals.var_lc10_dn8 = assign1920_e2521_d_n8;
        locals.var_lc10_rv = 0.0;

        let (assign1930_e2535, assign1930_e2535_d_n3, assign1930_e2535_d_n5, assign1930_e2535_d_n8, assign1930_e2535_d_n10, assign1930_e2535_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1930_e2533: f64 = (locals.var_psi_1).cosh();
        (assign1930_e2533, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn3), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn5), ((locals.var_psi_1).sinh() * locals.var_psi_1_dn8), 0.0, ((locals.var_psi_1).sinh() * locals.var_psi_1_dn11),)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn8, locals.var_cosh1_dn10, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign1930_e2535;
        locals.var_cosh1_dn3 = assign1930_e2535_d_n3;
        locals.var_cosh1_dn5 = assign1930_e2535_d_n5;
        locals.var_cosh1_dn8 = assign1930_e2535_d_n8;
        locals.var_cosh1_dn10 = assign1930_e2535_d_n10;
        locals.var_cosh1_dn11 = assign1930_e2535_d_n11;
        locals.var_cosh1_rv = 0.0;

        let (assign1940_e2549, assign1940_e2549_d_n3, assign1940_e2549_d_n5, assign1940_e2549_d_n8, assign1940_e2549_d_n10, assign1940_e2549_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1940_e2547: f64 = (locals.var_cosh1).ln();
        (assign1940_e2547, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn10 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc1, locals.var_lc1_dn3, locals.var_lc1_dn5, locals.var_lc1_dn8, locals.var_lc1_dn10, locals.var_lc1_dn11,)
    }
};
        locals.var_lc1 = assign1940_e2549;
        locals.var_lc1_dn3 = assign1940_e2549_d_n3;
        locals.var_lc1_dn5 = assign1940_e2549_d_n5;
        locals.var_lc1_dn8 = assign1940_e2549_d_n8;
        locals.var_lc1_dn10 = assign1940_e2549_d_n10;
        locals.var_lc1_dn11 = assign1940_e2549_d_n11;
        locals.var_lc1_rv = 0.0;

        let (assign1950_e2562,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        (0.5,)
    } else {
        (locals.var_mjc,)
    }
};
        locals.var_mjc = assign1950_e2562;
        locals.var_mjc_rv = 0.0;

        let (assign1960_e2593, assign1960_e2593_d_n8, assign1960_e2593_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1960_e2576: f64 = (p.p40 + locals.var_vgsc);
        let assign1960_e2577: f64 = (p.p39 * assign1960_e2576);
        let assign1960_e2580: f64 = (-1.0);
        let assign1960_e2583: f64 = (locals.var_vgsc / p.p40);
        let assign1960_e2584: f64 = (assign1960_e2580 + assign1960_e2583);
        let assign1960_e2586: f64 = (assign1960_e2584).powf(2.0);
        let assign1960_e2587: f64 = (p.p41 + assign1960_e2586);
        let assign1960_e2589: f64 = (-locals.var_mjc);
        let assign1960_e2590: f64 = (assign1960_e2587).powf(assign1960_e2589);
        let assign1960_e2591: f64 = (assign1960_e2577 * assign1960_e2590);
        (assign1960_e2591, (((p.p39 * locals.var_vgsc_dn8) * assign1960_e2590) + (assign1960_e2577 * if 0.0 == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (locals.var_vgsc_dn8 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((locals.var_vgsc_dn8 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (locals.var_vgsc_dn8 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((locals.var_vgsc_dn8 / p.p40) / assign1960_e2584))) } / assign1960_e2587))) })), (((p.p39 * locals.var_vgsc_dn11) * assign1960_e2590) + (assign1960_e2577 * if 0.0 == 0.0 && ((assign1960_e2589) as f64).is_finite() && ((assign1960_e2589) as f64).fract() == 0.0 { if assign1960_e2589 == 0.0 { 0.0 } else { (assign1960_e2589 * ((assign1960_e2587).powf(assign1960_e2589 - 1.0) * if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (locals.var_vgsc_dn11 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((locals.var_vgsc_dn11 / p.p40) / assign1960_e2584))) })) } } else { (assign1960_e2590 * (assign1960_e2589 * (if 0.0 == 0.0 && ((2.0) as f64).is_finite() && ((2.0) as f64).fract() == 0.0 { if 2.0 == 0.0 { 0.0 } else { (2.0 * ((assign1960_e2584).powf(2.0 - 1.0) * (locals.var_vgsc_dn11 / p.p40))) } } else { (assign1960_e2586 * (2.0 * ((locals.var_vgsc_dn11 / p.p40) / assign1960_e2584))) } / assign1960_e2587))) })),)
    } else {
        (locals.var_qgsdepl, locals.var_qgsdepl_dn8, locals.var_qgsdepl_dn11,)
    }
};
        locals.var_qgsdepl = assign1960_e2593;
        locals.var_qgsdepl_dn8 = assign1960_e2593_d_n8;
        locals.var_qgsdepl_dn11 = assign1960_e2593_d_n11;
        locals.var_qgsdepl_rv = 0.0;

        let (assign1970_e2615,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1970_e2606: f64 = (p.p39 * p.p40);
        let assign1970_e2609: f64 = (p.p41 + 1.0);
        let assign1970_e2611: f64 = (-locals.var_mjc);
        let assign1970_e2612: f64 = (assign1970_e2609).powf(assign1970_e2611);
        let assign1970_e2613: f64 = (assign1970_e2606 * assign1970_e2612);
        (assign1970_e2613,)
    } else {
        (locals.var_qgsdepl0,)
    }
};
        locals.var_qgsdepl0 = assign1970_e2615;
        locals.var_qgsdepl0_rv = 0.0;

        let (assign1980_e2634, assign1980_e2634_d_n3, assign1980_e2634_d_n5, assign1980_e2634_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1980_e2629: f64 = (p.p38 * locals.var_vds);
        let assign1980_e2630: f64 = (locals.var_p10_t + assign1980_e2629);
        let assign1980_e2632: f64 = (assign1980_e2630 + locals.var_lc10);
        (assign1980_e2632, (locals.var_p10_t_dn3 + locals.var_lc10_dn3), ((p.p38 * locals.var_vds_dn5) + locals.var_lc10_dn5), ((p.p38 * locals.var_vds_dn8) + locals.var_lc10_dn8),)
    } else {
        (locals.var_qgs0, locals.var_qgs0_dn3, locals.var_qgs0_dn5, locals.var_qgs0_dn8,)
    }
};
        locals.var_qgs0 = assign1980_e2634;
        locals.var_qgs0_dn3 = assign1980_e2634_d_n3;
        locals.var_qgs0_dn5 = assign1980_e2634_d_n5;
        locals.var_qgs0_dn8 = assign1980_e2634_d_n8;
        locals.var_qgs0_rv = 0.0;

        let (assign1990_e2676, assign1990_e2676_d_n3, assign1990_e2676_d_n5, assign1990_e2676_d_n8, assign1990_e2676_d_n10, assign1990_e2676_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign1990_e2648: f64 = (locals.var_psi_1 + locals.var_lc1);
        let assign1990_e2650: f64 = (assign1990_e2648 - locals.var_qgs0);
        let assign1990_e2652: f64 = (assign1990_e2650 + locals.var_qgsdepl);
        let assign1990_e2654: f64 = (assign1990_e2652 - locals.var_qgsdepl0);
        let assign1990_e2657: f64 = (1.0 - p.p38);
        let assign1990_e2659: f64 = (locals.var_psi_2).tanh();
        let assign1990_e2660: f64 = (assign1990_e2657 + assign1990_e2659);
        let assign1990_e2661: f64 = (assign1990_e2654 * assign1990_e2660);
        let assign1990_e2663: f64 = (assign1990_e2661 / p.p31);
        let assign1990_e2666: f64 = (2.0 * p.p38);
        let assign1990_e2668: f64 = (assign1990_e2666 * locals.var_vgsc);
        let assign1990_e2669: f64 = (assign1990_e2663 + assign1990_e2668);
        let assign1990_e2670: f64 = (locals.var_cgs0_t * assign1990_e2669);
        let assign1990_e2673: f64 = (p.p25 * locals.var_vgsc);
        let assign1990_e2674: f64 = (assign1990_e2670 + assign1990_e2673);
        (assign1990_e2674, ((locals.var_cgs0_t_dn3 * assign1990_e2669) + (locals.var_cgs0_t * ((((locals.var_psi_1_dn3 + locals.var_lc1_dn3) - locals.var_qgs0_dn3) * assign1990_e2660) / p.p31))), (locals.var_cgs0_t * (((((locals.var_psi_1_dn5 + locals.var_lc1_dn5) - locals.var_qgs0_dn5) * assign1990_e2660) + (assign1990_e2654 * (locals.var_psi_2_dn5 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh())))) / p.p31)), ((locals.var_cgs0_t * (((((((locals.var_psi_1_dn8 + locals.var_lc1_dn8) - locals.var_qgs0_dn8) + locals.var_qgsdepl_dn8) * assign1990_e2660) + (assign1990_e2654 * (locals.var_psi_2_dn8 / ((locals.var_psi_2).cosh() * (locals.var_psi_2).cosh())))) / p.p31) + (assign1990_e2666 * locals.var_vgsc_dn8))) + (p.p25 * locals.var_vgsc_dn8)), (locals.var_cgs0_t * ((locals.var_lc1_dn10 * assign1990_e2660) / p.p31)), ((locals.var_cgs0_t * (((((locals.var_psi_1_dn11 + locals.var_lc1_dn11) + locals.var_qgsdepl_dn11) * assign1990_e2660) / p.p31) + (assign1990_e2666 * locals.var_vgsc_dn11))) + (p.p25 * locals.var_vgsc_dn11)),)
    } else {
        (locals.var_qgs, locals.var_qgs_dn3, locals.var_qgs_dn5, locals.var_qgs_dn8, locals.var_qgs_dn10, locals.var_qgs_dn11,)
    }
};
        locals.var_qgs = assign1990_e2676;
        locals.var_qgs_dn3 = assign1990_e2676_d_n3;
        locals.var_qgs_dn5 = assign1990_e2676_d_n5;
        locals.var_qgs_dn8 = assign1990_e2676_d_n8;
        locals.var_qgs_dn10 = assign1990_e2676_d_n10;
        locals.var_qgs_dn11 = assign1990_e2676_d_n11;
        locals.var_qgs_rv = 0.0;

        let (assign2000_e2694, assign2000_e2694_d_n3, assign2000_e2694_d_n5, assign2000_e2694_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2000_e2690: f64 = (p.p38 * locals.var_vds);
        let assign2000_e2691: f64 = (locals.var_p40_t - assign2000_e2690);
        let assign2000_e2692: f64 = (assign2000_e2691).cosh();
        (assign2000_e2692, ((assign2000_e2691).sinh() * locals.var_p40_t_dn3), ((assign2000_e2691).sinh() * (-(p.p38 * locals.var_vds_dn5))), ((assign2000_e2691).sinh() * (-(p.p38 * locals.var_vds_dn8))),)
    } else {
        (locals.var_cosh0, locals.var_cosh0_dn3, locals.var_cosh0_dn5, locals.var_cosh0_dn8,)
    }
};
        locals.var_cosh0 = assign2000_e2694;
        locals.var_cosh0_dn3 = assign2000_e2694_d_n3;
        locals.var_cosh0_dn5 = assign2000_e2694_d_n5;
        locals.var_cosh0_dn8 = assign2000_e2694_d_n8;
        locals.var_cosh0_rv = 0.0;

        let (assign2010_e2708, assign2010_e2708_d_n3, assign2010_e2708_d_n5, assign2010_e2708_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2010_e2706: f64 = (locals.var_cosh0).ln();
        (assign2010_e2706, (locals.var_cosh0_dn3 / locals.var_cosh0), (locals.var_cosh0_dn5 / locals.var_cosh0), (locals.var_cosh0_dn8 / locals.var_cosh0),)
    } else {
        (locals.var_lc40, locals.var_lc40_dn3, locals.var_lc40_dn5, locals.var_lc40_dn8,)
    }
};
        locals.var_lc40 = assign2010_e2708;
        locals.var_lc40_dn3 = assign2010_e2708_d_n3;
        locals.var_lc40_dn5 = assign2010_e2708_d_n5;
        locals.var_lc40_dn8 = assign2010_e2708_d_n8;
        locals.var_lc40_rv = 0.0;

        let (assign2020_e2722, assign2020_e2722_d_n3, assign2020_e2722_d_n5, assign2020_e2722_d_n8, assign2020_e2722_d_n10, assign2020_e2722_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2020_e2720: f64 = (locals.var_psi_4).cosh();
        (assign2020_e2720, ((locals.var_psi_4).sinh() * locals.var_psi_4_dn3), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn5), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn8), ((locals.var_psi_4).sinh() * locals.var_psi_4_dn10), 0.0,)
    } else {
        (locals.var_cosh1, locals.var_cosh1_dn3, locals.var_cosh1_dn5, locals.var_cosh1_dn8, locals.var_cosh1_dn10, locals.var_cosh1_dn11,)
    }
};
        locals.var_cosh1 = assign2020_e2722;
        locals.var_cosh1_dn3 = assign2020_e2722_d_n3;
        locals.var_cosh1_dn5 = assign2020_e2722_d_n5;
        locals.var_cosh1_dn8 = assign2020_e2722_d_n8;
        locals.var_cosh1_dn10 = assign2020_e2722_d_n10;
        locals.var_cosh1_dn11 = assign2020_e2722_d_n11;
        locals.var_cosh1_rv = 0.0;

        let (assign2030_e2736, assign2030_e2736_d_n3, assign2030_e2736_d_n5, assign2030_e2736_d_n8, assign2030_e2736_d_n10, assign2030_e2736_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2030_e2734: f64 = (locals.var_cosh1).ln();
        (assign2030_e2734, (locals.var_cosh1_dn3 / locals.var_cosh1), (locals.var_cosh1_dn5 / locals.var_cosh1), (locals.var_cosh1_dn8 / locals.var_cosh1), (locals.var_cosh1_dn10 / locals.var_cosh1), (locals.var_cosh1_dn11 / locals.var_cosh1),)
    } else {
        (locals.var_lc4, locals.var_lc4_dn3, locals.var_lc4_dn5, locals.var_lc4_dn8, locals.var_lc4_dn10, locals.var_lc4_dn11,)
    }
};
        locals.var_lc4 = assign2030_e2736;
        locals.var_lc4_dn3 = assign2030_e2736_d_n3;
        locals.var_lc4_dn5 = assign2030_e2736_d_n5;
        locals.var_lc4_dn8 = assign2030_e2736_d_n8;
        locals.var_lc4_dn10 = assign2030_e2736_d_n10;
        locals.var_lc4_dn11 = assign2030_e2736_d_n11;
        locals.var_lc4_rv = 0.0;

        let (assign2040_e2755, assign2040_e2755_d_n3, assign2040_e2755_d_n5, assign2040_e2755_d_n8,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2040_e2750: f64 = (p.p38 * locals.var_vds);
        let assign2040_e2751: f64 = (locals.var_p40_t - assign2040_e2750);
        let assign2040_e2753: f64 = (assign2040_e2751 + locals.var_lc40);
        (assign2040_e2753, (locals.var_p40_t_dn3 + locals.var_lc40_dn3), ((-(p.p38 * locals.var_vds_dn5)) + locals.var_lc40_dn5), ((-(p.p38 * locals.var_vds_dn8)) + locals.var_lc40_dn8),)
    } else {
        (locals.var_qgd0, locals.var_qgd0_dn3, locals.var_qgd0_dn5, locals.var_qgd0_dn8,)
    }
};
        locals.var_qgd0 = assign2040_e2755;
        locals.var_qgd0_dn3 = assign2040_e2755_d_n3;
        locals.var_qgd0_dn5 = assign2040_e2755_d_n5;
        locals.var_qgd0_dn8 = assign2040_e2755_d_n8;
        locals.var_qgd0_rv = 0.0;

        let (assign2050_e2793, assign2050_e2793_d_n3, assign2050_e2793_d_n5, assign2050_e2793_d_n8, assign2050_e2793_d_n10, assign2050_e2793_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2050_e2769: f64 = (locals.var_psi_4 + locals.var_lc4);
        let assign2050_e2771: f64 = (assign2050_e2769 - locals.var_qgd0);
        let assign2050_e2774: f64 = (1.0 - p.p38);
        let assign2050_e2776: f64 = (locals.var_psi_3).tanh();
        let assign2050_e2777: f64 = (assign2050_e2774 + assign2050_e2776);
        let assign2050_e2778: f64 = (assign2050_e2771 * assign2050_e2777);
        let assign2050_e2780: f64 = (assign2050_e2778 / p.p37);
        let assign2050_e2783: f64 = (2.0 * p.p38);
        let assign2050_e2785: f64 = (assign2050_e2783 * locals.var_vgdc);
        let assign2050_e2786: f64 = (assign2050_e2780 + assign2050_e2785);
        let assign2050_e2787: f64 = (locals.var_cgd0_t * assign2050_e2786);
        let assign2050_e2790: f64 = (p.p27 * locals.var_vgdc);
        let assign2050_e2791: f64 = (assign2050_e2787 + assign2050_e2790);
        (assign2050_e2791, ((locals.var_cgd0_t_dn3 * assign2050_e2786) + (locals.var_cgd0_t * ((((locals.var_psi_4_dn3 + locals.var_lc4_dn3) - locals.var_qgd0_dn3) * assign2050_e2777) / p.p37))), ((locals.var_cgd0_t * ((((((locals.var_psi_4_dn5 + locals.var_lc4_dn5) - locals.var_qgd0_dn5) * assign2050_e2777) + (assign2050_e2771 * (locals.var_psi_3_dn5 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh())))) / p.p37) + (assign2050_e2783 * locals.var_vgdc_dn5))) + (p.p27 * locals.var_vgdc_dn5)), (locals.var_cgd0_t * (((((locals.var_psi_4_dn8 + locals.var_lc4_dn8) - locals.var_qgd0_dn8) * assign2050_e2777) + (assign2050_e2771 * (locals.var_psi_3_dn8 / ((locals.var_psi_3).cosh() * (locals.var_psi_3).cosh())))) / p.p37)), ((locals.var_cgd0_t * ((((locals.var_psi_4_dn10 + locals.var_lc4_dn10) * assign2050_e2777) / p.p37) + (assign2050_e2783 * locals.var_vgdc_dn10))) + (p.p27 * locals.var_vgdc_dn10)), (locals.var_cgd0_t * ((locals.var_lc4_dn11 * assign2050_e2777) / p.p37)),)
    } else {
        (locals.var_qgd, locals.var_qgd_dn3, locals.var_qgd_dn5, locals.var_qgd_dn8, locals.var_qgd_dn10, locals.var_qgd_dn11,)
    }
};
        locals.var_qgd = assign2050_e2793;
        locals.var_qgd_dn3 = assign2050_e2793_d_n3;
        locals.var_qgd_dn5 = assign2050_e2793_d_n5;
        locals.var_qgd_dn8 = assign2050_e2793_d_n8;
        locals.var_qgd_dn10 = assign2050_e2793_d_n10;
        locals.var_qgd_dn11 = assign2050_e2793_d_n11;
        locals.var_qgd_rv = 0.0;

        let (assign2060_e2808, assign2060_e2808_d_n3, assign2060_e2808_d_n5, assign2060_e2808_d_n8, assign2060_e2808_d_n10, assign2060_e2808_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2060_e2806: f64 = locals.var_qgs_dn11;
        (assign2060_e2806, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgs, locals.var_cgs_dn3, locals.var_cgs_dn5, locals.var_cgs_dn8, locals.var_cgs_dn10, locals.var_cgs_dn11,)
    }
};
        locals.var_cgs = assign2060_e2808;
        locals.var_cgs_dn3 = assign2060_e2808_d_n3;
        locals.var_cgs_dn5 = assign2060_e2808_d_n5;
        locals.var_cgs_dn8 = assign2060_e2808_d_n8;
        locals.var_cgs_dn10 = assign2060_e2808_d_n10;
        locals.var_cgs_dn11 = assign2060_e2808_d_n11;
        locals.var_cgs_rv = 0.0;

        let (assign2070_e2823, assign2070_e2823_d_n3, assign2070_e2823_d_n5, assign2070_e2823_d_n8, assign2070_e2823_d_n10, assign2070_e2823_d_n11,) = {
    if ((locals.var_guard18 != 0.0) && (!((((locals.var_guard14 != 0.0) || (locals.var_guard15 != 0.0)) || (locals.var_guard16 != 0.0)) || (locals.var_guard17 != 0.0)))) {
        let assign2070_e2821: f64 = locals.var_qgd_dn10;
        (assign2070_e2821, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cgd, locals.var_cgd_dn3, locals.var_cgd_dn5, locals.var_cgd_dn8, locals.var_cgd_dn10, locals.var_cgd_dn11,)
    }
};
        locals.var_cgd = assign2070_e2823;
        locals.var_cgd_dn3 = assign2070_e2823_d_n3;
        locals.var_cgd_dn5 = assign2070_e2823_d_n5;
        locals.var_cgd_dn8 = assign2070_e2823_d_n8;
        locals.var_cgd_dn10 = assign2070_e2823_d_n10;
        locals.var_cgd_dn11 = assign2070_e2823_d_n11;
        locals.var_cgd_rv = 0.0;

        let assign2080_e2830: f64 = if ((p.p6 == 2.0) || (p.p6 == 4.0)) { 1.0 } else { 0.0 };
        locals.var_guard19 = assign2080_e2830;
        locals.var_guard19_rv = 0.0;

        let assign2090_e2833: f64 = (p.p55 * bi1);
        let assign2090_e2834_q: f64 = assign2090_e2833;
        locals.var_t0 = assign2090_e2833;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_db1 = p.p55;
        locals.var_t0_rv = assign2090_e2834_q;
        locals.var_t0_rdb1 = p.p55;

        let assign2100_e2837: f64 = if p.p58 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard20 = assign2100_e2837;
        locals.var_guard20_rv = 0.0;

        let assign2110_e2844: f64 = if ((p.p63 > 0.0) || (p.p62 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard21 = assign2110_e2844;
        locals.var_guard21_rv = 0.0;

        let assign2160_e2859: f64 = if p.p50 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard26 = assign2160_e2859;
        locals.var_guard26_rv = 0.0;

        let assign2170_e2866: f64 = if ((p.p47 > 0.0) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard27 = assign2170_e2866;
        locals.var_guard27_rv = 0.0;

        let assign2210_e2882: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard28 = assign2210_e2882;
        locals.var_guard28_rv = 0.0;

        let assign2220_e2885: f64 = if p.p7 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign2220_e2885;
        locals.var_guard29_rv = 0.0;

        let (assign2310_e3018, assign2310_e3018_d_n3,) = {
    if (((locals.var_guard29 != 0.0) && (locals.var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let assign2310_e3005: f64 = (4.0 * 1.3806503e-23);
        let assign2310_e3007: f64 = (assign2310_e3005 * locals.var_t);
        let assign2310_e3009: f64 = (assign2310_e3007 * p.p88);
        let assign2310_e3011: f64 = (assign2310_e3009 * locals.var_cgs0_t);
        let assign2310_e3014: f64 = (p.p87 * p.p86);
        let assign2310_e3015: f64 = (assign2310_e3014).sqrt();
        let assign2310_e3016: f64 = (assign2310_e3011 * assign2310_e3015);
        (assign2310_e3016, (((((assign2310_e3005 * locals.var_t_dn3) * p.p88) * locals.var_cgs0_t) + (assign2310_e3009 * locals.var_cgs0_t_dn3)) * assign2310_e3015),)
    } else {
        (locals.var_k, locals.var_k_dn3,)
    }
};
        locals.var_k = assign2310_e3018;
        locals.var_k_dn3 = assign2310_e3018_d_n3;
        locals.var_k_rv = 0.0;

        let (assign2340_e3055, assign2340_e3055_d_n3,) = {
    if (((locals.var_guard29 != 0.0) && (locals.var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let assign2340_e3053: f64 = (locals.var_k * 3.141592653589793);
        (assign2340_e3053, (locals.var_k_dn3 * 3.141592653589793),)
    } else {
        (locals.var_ci, locals.var_ci_dn3,)
    }
};
        locals.var_ci = assign2340_e3055;
        locals.var_ci_dn3 = assign2340_e3055_d_n3;
        locals.var_ci_rv = 0.0;

        let assign2380_e3083: f64 = if p.p1 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign2380_e3083;
        locals.var_guard44_rv = 0.0;

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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let bi7 = ctx.branch_current(branches[7]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi11 = ctx.branch_current(branches[11]);
        let bi14 = ctx.branch_current(branches[14]);
        let bi15 = ctx.branch_current(branches[15]);
        let bi18 = ctx.branch_current(branches[18]);
        let eq3_e114: f64 = (p.p56 / 3.0);
        let eq3_e116: f64 = (eq3_e114 * bi0);
        let eq3_e117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq3_e116);
        let eq3_value: f64 = eq3_e117;
        stamper.stamp_potential_branch1_local(
            0,
            eq3_value,
            0,
            (eq3_e114 * ddt_scale),
        );
        let (eq7_e125, eq7_e125_d_n3, eq7_e125_d_n5, eq7_e125_d_n8, eq7_e125_d_n10, eq7_e125_d_n11,) = {
    if (locals.var_guard19 != 0.0) {
        let eq7_e123: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qgd);
        (eq7_e123, (locals.var_qgd_dn3 * ddt_scale), (locals.var_qgd_dn5 * ddt_scale), (locals.var_qgd_dn8 * ddt_scale), (locals.var_qgd_dn10 * ddt_scale), (locals.var_qgd_dn11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e125;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (eq7_value),
            [3, 5, 8, 10, 11],
            [multiplicity * (eq7_e125_d_n3), multiplicity * (eq7_e125_d_n5), multiplicity * (eq7_e125_d_n8), multiplicity * (eq7_e125_d_n10), multiplicity * (eq7_e125_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq8_e130, eq8_e130_d_n3, eq8_e130_d_n5, eq8_e130_d_n8, eq8_e130_d_n10, eq8_e130_d_n11,) = {
    if (locals.var_guard19 != 0.0) {
        let eq8_e128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qgs);
        (eq8_e128, (locals.var_qgs_dn3 * ddt_scale), (locals.var_qgs_dn5 * ddt_scale), (locals.var_qgs_dn8 * ddt_scale), (locals.var_qgs_dn10 * ddt_scale), (locals.var_qgs_dn11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e130;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(11),
            Some(8),
            multiplicity * (eq8_value),
            [3, 5, 8, 10, 11],
            [multiplicity * (eq8_e130_d_n3), multiplicity * (eq8_e130_d_n5), multiplicity * (eq8_e130_d_n8), multiplicity * (eq8_e130_d_n10), multiplicity * (eq8_e130_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq9_e138, eq9_e138_d_n3, eq9_e138_d_n5, eq9_e138_d_n8, eq9_e138_d_n10, eq9_e138_d_n11,) = {
    if (locals.var_guard19 == 0.0) {
        let eq9_e135: f64 = (locals.var_cgd * locals.var_vgdc);
        let eq9_e135_d_n3: f64 = (locals.var_cgd_dn3 * locals.var_vgdc);
        let eq9_e135_d_n5: f64 = ((locals.var_cgd_dn5 * locals.var_vgdc) + (locals.var_cgd * locals.var_vgdc_dn5));
        let eq9_e135_d_n8: f64 = (locals.var_cgd_dn8 * locals.var_vgdc);
        let eq9_e135_d_n10: f64 = ((locals.var_cgd_dn10 * locals.var_vgdc) + (locals.var_cgd * locals.var_vgdc_dn10));
        let eq9_e135_d_n11: f64 = (locals.var_cgd_dn11 * locals.var_vgdc);
        let eq9_e136: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq9_e135);
        (eq9_e136, (eq9_e135_d_n3 * ddt_scale), (eq9_e135_d_n5 * ddt_scale), (eq9_e135_d_n8 * ddt_scale), (eq9_e135_d_n10 * ddt_scale), (eq9_e135_d_n11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e138;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(10),
            Some(5),
            multiplicity * (eq9_value),
            [3, 5, 8, 10, 11],
            [multiplicity * (eq9_e138_d_n3), multiplicity * (eq9_e138_d_n5), multiplicity * (eq9_e138_d_n8), multiplicity * (eq9_e138_d_n10), multiplicity * (eq9_e138_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq10_e146, eq10_e146_d_n3, eq10_e146_d_n5, eq10_e146_d_n8, eq10_e146_d_n10, eq10_e146_d_n11,) = {
    if (locals.var_guard19 == 0.0) {
        let eq10_e143: f64 = (locals.var_cgs * locals.var_vgsc);
        let eq10_e143_d_n3: f64 = (locals.var_cgs_dn3 * locals.var_vgsc);
        let eq10_e143_d_n5: f64 = (locals.var_cgs_dn5 * locals.var_vgsc);
        let eq10_e143_d_n8: f64 = ((locals.var_cgs_dn8 * locals.var_vgsc) + (locals.var_cgs * locals.var_vgsc_dn8));
        let eq10_e143_d_n10: f64 = (locals.var_cgs_dn10 * locals.var_vgsc);
        let eq10_e143_d_n11: f64 = ((locals.var_cgs_dn11 * locals.var_vgsc) + (locals.var_cgs * locals.var_vgsc_dn11));
        let eq10_e144: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq10_e143);
        (eq10_e144, (eq10_e143_d_n3 * ddt_scale), (eq10_e143_d_n5 * ddt_scale), (eq10_e143_d_n8 * ddt_scale), (eq10_e143_d_n10 * ddt_scale), (eq10_e143_d_n11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e146;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(11),
            Some(8),
            multiplicity * (eq10_value),
            [3, 5, 8, 10, 11],
            [multiplicity * (eq10_e146_d_n3), multiplicity * (eq10_e146_d_n5), multiplicity * (eq10_e146_d_n8), multiplicity * (eq10_e146_d_n10), multiplicity * (eq10_e146_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq15_e169, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n8, eq15_e169_d_n10, eq15_e169_d_n12, eq15_e169_d_b1,) = {
    if (locals.var_guard20 != 0.0) {
        let eq15_e165: f64 = (bi1 * locals.var_rc1);
        let eq15_e165_d_n3: f64 = (bi1 * locals.var_rc1_dn3);
        let eq15_e165_d_n4: f64 = (bi1 * locals.var_rc1_dn4);
        let eq15_e165_d_n5: f64 = (bi1 * locals.var_rc1_dn5);
        let eq15_e165_d_n8: f64 = (bi1 * locals.var_rc1_dn8);
        let eq15_e165_d_n10: f64 = (bi1 * locals.var_rc1_dn10);
        let eq15_e165_d_n12: f64 = (bi1 * locals.var_rc1_dn12);
        let eq15_e165_d_b1: f64 = (locals.var_rc1 + (bi1 * locals.var_rc1_db1));
        let eq15_e167: f64 = (eq15_e165 + locals.var_t0);
        let eq15_e167_d_n3: f64 = (eq15_e165_d_n3 + locals.var_t0_dn3);
        let eq15_e167_d_n4: f64 = (eq15_e165_d_n4 + locals.var_t0_dn4);
        let eq15_e167_d_n5: f64 = (eq15_e165_d_n5 + locals.var_t0_dn5);
        let eq15_e167_d_n8: f64 = (eq15_e165_d_n8 + locals.var_t0_dn8);
        let eq15_e167_d_n10: f64 = (eq15_e165_d_n10 + locals.var_t0_dn10);
        let eq15_e167_d_n12: f64 = (eq15_e165_d_n12 + locals.var_t0_dn12);
        let eq15_e167_d_b1: f64 = (eq15_e165_d_b1 + locals.var_t0_db1);
        (eq15_e167, eq15_e167_d_n3, eq15_e167_d_n4, eq15_e167_d_n5, eq15_e167_d_n8, eq15_e167_d_n10, eq15_e167_d_n12, eq15_e167_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e169;
        stamper.stamp_potential_sparse_local::<6, 1>(
            1,
            eq15_value,
            [3, 4, 5, 8, 10, 12],
            [eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n8, eq15_e169_d_n10, eq15_e169_d_n12],
            [1],
            [eq15_e169_d_b1],
        );
        let (eq18_e187, eq18_e187_d_n3, eq18_e187_d_n8, eq18_e187_d_n12,) = {
    if (locals.var_guard21 != 0.0) {
        let eq18_e184: f64 = (locals.var_cdel_t * (nv12 - nv8));
        let eq18_e184_d_n3: f64 = (locals.var_cdel_t_dn3 * (nv12 - nv8));
        let eq18_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq18_e184);
        (eq18_e185, (eq18_e184_d_n3 * ddt_scale), ((-locals.var_cdel_t) * ddt_scale), (locals.var_cdel_t * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e187;
        stamper.stamp_current_node3_local(
            Some(12),
            Some(8),
            multiplicity * (eq18_value),
            3,
            multiplicity * (eq18_e187_d_n3),
            8,
            multiplicity * (eq18_e187_d_n8),
            12,
            multiplicity * (eq18_e187_d_n12),
        );
        let (eq28_e247, eq28_e247_d_b7,) = {
    if (locals.var_guard25 != 0.0) {
        let eq28_e245: f64 = (bi7 * p.p46);
        (eq28_e245, p.p46,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e247;
        stamper.stamp_potential_branch1_local(
            7,
            eq28_value,
            7,
            eq28_e247_d_b7,
        );
        let eq31_e269: f64 = (p.p54 * bi10);
        let eq31_e270: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq31_e269);
        let eq31_value: f64 = eq31_e270;
        stamper.stamp_potential_branch1_local(
            10,
            eq31_value,
            10,
            (p.p54 * ddt_scale),
        );
        let (eq32_e276, eq32_e276_d_n3, eq32_e276_d_n4, eq32_e276_d_n5, eq32_e276_d_n8, eq32_e276_d_n10, eq32_e276_d_n12, eq32_e276_d_b1, eq32_e276_d_b11,) = {
    if (locals.var_guard26 != 0.0) {
        let eq32_e274: f64 = (bi11 * locals.var_rs_t);
        let eq32_e274_d_n3: f64 = (bi11 * locals.var_rs_t_dn3);
        let eq32_e274_d_n4: f64 = (bi11 * locals.var_rs_t_dn4);
        let eq32_e274_d_n5: f64 = (bi11 * locals.var_rs_t_dn5);
        let eq32_e274_d_n8: f64 = (bi11 * locals.var_rs_t_dn8);
        let eq32_e274_d_n10: f64 = (bi11 * locals.var_rs_t_dn10);
        let eq32_e274_d_n12: f64 = (bi11 * locals.var_rs_t_dn12);
        let eq32_e274_d_b1: f64 = (bi11 * locals.var_rs_t_db1);
        (eq32_e274, eq32_e274_d_n3, eq32_e274_d_n4, eq32_e274_d_n5, eq32_e274_d_n8, eq32_e274_d_n10, eq32_e274_d_n12, eq32_e274_d_b1, locals.var_rs_t,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e276;
        stamper.stamp_potential_sparse_local::<6, 2>(
            11,
            eq32_value,
            [3, 4, 5, 8, 10, 12],
            [eq32_e276_d_n3, eq32_e276_d_n4, eq32_e276_d_n5, eq32_e276_d_n8, eq32_e276_d_n10, eq32_e276_d_n12],
            [1, 11],
            [eq32_e276_d_b1, eq32_e276_d_b11],
        );
        let eq35_e298: f64 = (p.p53 * bi14);
        let eq35_e299: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq35_e298);
        let eq35_value: f64 = eq35_e299;
        stamper.stamp_potential_branch1_local(
            14,
            eq35_value,
            14,
            (p.p53 * ddt_scale),
        );
        let (eq36_e305, eq36_e305_d_n3, eq36_e305_d_n4, eq36_e305_d_n5, eq36_e305_d_n8, eq36_e305_d_n10, eq36_e305_d_n12, eq36_e305_d_b1, eq36_e305_d_b15,) = {
    if (locals.var_guard27 != 0.0) {
        let eq36_e303: f64 = (bi15 * locals.var_rd1_t);
        let eq36_e303_d_n3: f64 = (bi15 * locals.var_rd1_t_dn3);
        let eq36_e303_d_n4: f64 = (bi15 * locals.var_rd1_t_dn4);
        let eq36_e303_d_n5: f64 = (bi15 * locals.var_rd1_t_dn5);
        let eq36_e303_d_n8: f64 = (bi15 * locals.var_rd1_t_dn8);
        let eq36_e303_d_n10: f64 = (bi15 * locals.var_rd1_t_dn10);
        let eq36_e303_d_n12: f64 = (bi15 * locals.var_rd1_t_dn12);
        let eq36_e303_d_b1: f64 = (bi15 * locals.var_rd1_t_db1);
        (eq36_e303, eq36_e303_d_n3, eq36_e303_d_n4, eq36_e303_d_n5, eq36_e303_d_n8, eq36_e303_d_n10, eq36_e303_d_n12, eq36_e303_d_b1, locals.var_rd1_t,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e305;
        stamper.stamp_potential_sparse_local::<6, 2>(
            15,
            eq36_value,
            [3, 4, 5, 8, 10, 12],
            [eq36_e305_d_n3, eq36_e305_d_n4, eq36_e305_d_n5, eq36_e305_d_n8, eq36_e305_d_n10, eq36_e305_d_n12],
            [1, 15],
            [eq36_e305_d_b1, eq36_e305_d_b15],
        );
        let eq39_e327: f64 = (p.p52 * bi18);
        let eq39_e328: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq39_e327);
        let eq39_value: f64 = eq39_e328;
        stamper.stamp_potential_branch1_local(
            18,
            eq39_value,
            18,
            (p.p52 * ddt_scale),
        );
        let (eq51_e429, eq51_e429_d_n3, eq51_e429_d_n17,) = {
    if (((locals.var_guard29 != 0.0) && (locals.var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let eq51_e424: f64 = (-locals.var_ci);
        let eq51_e426: f64 = (eq51_e424 * (nv17 - 0.0));
        let eq51_e426_d_n3: f64 = ((-locals.var_ci_dn3) * (nv17 - 0.0));
        let eq51_e427: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq51_e426);
        (eq51_e427, (eq51_e426_d_n3 * ddt_scale), (eq51_e424 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e429;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(5),
            multiplicity * (eq51_value),
            3,
            multiplicity * (eq51_e429_d_n3),
            17,
            multiplicity * (eq51_e429_d_n17),
        );
        let (eq64_e562, eq64_e562_d_n3,) = {
    if (locals.var_guard44 != 0.0) {
        let eq64_e559: f64 = (p.p67 * (nv3 - 0.0));
        let eq64_e560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq64_e559);
        (eq64_e560, (p.p67 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e562;
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * (eq64_value),
            3,
            multiplicity * (eq64_e562_d_n3),
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let bi10 = ctx.branch_current(branches[10]);
        let bi14 = ctx.branch_current(branches[14]);
        let bi18 = ctx.branch_current(branches[18]);
        let eq3_e114: f64 = (p.p56 / 3.0);
        let eq3_e116: f64 = (eq3_e114 * bi0);
        let eq3_e117_q: f64 = eq3_e116;
        stamper.stamp_potential_reactive_branch1(
            branches[0],
            branches[0],
            eq3_e114,
        );
        let (eq7_e125, eq7_e125_d_n3, eq7_e125_d_n5, eq7_e125_d_n8, eq7_e125_d_n10, eq7_e125_d_n11, eq7_e125_q,) = {
    if (locals.var_guard19 != 0.0) {
        let eq7_e123_q: f64 = locals.var_qgd;
        (locals.var_qgd, locals.var_qgd_dn3, locals.var_qgd_dn5, locals.var_qgd_dn8, locals.var_qgd_dn10, locals.var_qgd_dn11, eq7_e123_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_reactive_node_derivatives: [f64; 19] = [0.0, 0.0, 0.0, eq7_e125_d_n3, 0.0, eq7_e125_d_n5, 0.0, 0.0, eq7_e125_d_n8, 0.0, eq7_e125_d_n10, eq7_e125_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq7_reactive_branch_derivatives: [f64; 19] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq7_reactive_node_derivatives,
            branches,
            &eq7_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq8_e130, eq8_e130_d_n3, eq8_e130_d_n5, eq8_e130_d_n8, eq8_e130_d_n10, eq8_e130_d_n11, eq8_e130_q,) = {
    if (locals.var_guard19 != 0.0) {
        let eq8_e128_q: f64 = locals.var_qgs;
        (locals.var_qgs, locals.var_qgs_dn3, locals.var_qgs_dn5, locals.var_qgs_dn8, locals.var_qgs_dn10, locals.var_qgs_dn11, eq8_e128_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 19] = [0.0, 0.0, 0.0, eq8_e130_d_n3, 0.0, eq8_e130_d_n5, 0.0, 0.0, eq8_e130_d_n8, 0.0, eq8_e130_d_n10, eq8_e130_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq8_reactive_branch_derivatives: [f64; 19] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq9_e138, eq9_e138_d_n3, eq9_e138_d_n5, eq9_e138_d_n8, eq9_e138_d_n10, eq9_e138_d_n11, eq9_e138_q,) = {
    if (locals.var_guard19 == 0.0) {
        let eq9_e135: f64 = (locals.var_cgd * locals.var_vgdc);
        let eq9_e135_d_n3: f64 = (locals.var_cgd_dn3 * locals.var_vgdc);
        let eq9_e135_d_n5: f64 = ((locals.var_cgd_dn5 * locals.var_vgdc) + (locals.var_cgd * locals.var_vgdc_dn5));
        let eq9_e135_d_n8: f64 = (locals.var_cgd_dn8 * locals.var_vgdc);
        let eq9_e135_d_n10: f64 = ((locals.var_cgd_dn10 * locals.var_vgdc) + (locals.var_cgd * locals.var_vgdc_dn10));
        let eq9_e135_d_n11: f64 = (locals.var_cgd_dn11 * locals.var_vgdc);
        let eq9_e136_q: f64 = eq9_e135;
        (eq9_e135, eq9_e135_d_n3, eq9_e135_d_n5, eq9_e135_d_n8, eq9_e135_d_n10, eq9_e135_d_n11, eq9_e136_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq9_reactive_node_derivatives: [f64; 19] = [0.0, 0.0, 0.0, eq9_e138_d_n3, 0.0, eq9_e138_d_n5, 0.0, 0.0, eq9_e138_d_n8, 0.0, eq9_e138_d_n10, eq9_e138_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq9_reactive_branch_derivatives: [f64; 19] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq10_e146, eq10_e146_d_n3, eq10_e146_d_n5, eq10_e146_d_n8, eq10_e146_d_n10, eq10_e146_d_n11, eq10_e146_q,) = {
    if (locals.var_guard19 == 0.0) {
        let eq10_e143: f64 = (locals.var_cgs * locals.var_vgsc);
        let eq10_e143_d_n3: f64 = (locals.var_cgs_dn3 * locals.var_vgsc);
        let eq10_e143_d_n5: f64 = (locals.var_cgs_dn5 * locals.var_vgsc);
        let eq10_e143_d_n8: f64 = ((locals.var_cgs_dn8 * locals.var_vgsc) + (locals.var_cgs * locals.var_vgsc_dn8));
        let eq10_e143_d_n10: f64 = (locals.var_cgs_dn10 * locals.var_vgsc);
        let eq10_e143_d_n11: f64 = ((locals.var_cgs_dn11 * locals.var_vgsc) + (locals.var_cgs * locals.var_vgsc_dn11));
        let eq10_e144_q: f64 = eq10_e143;
        (eq10_e143, eq10_e143_d_n3, eq10_e143_d_n5, eq10_e143_d_n8, eq10_e143_d_n10, eq10_e143_d_n11, eq10_e144_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_reactive_node_derivatives: [f64; 19] = [0.0, 0.0, 0.0, eq10_e146_d_n3, 0.0, eq10_e146_d_n5, 0.0, 0.0, eq10_e146_d_n8, 0.0, eq10_e146_d_n10, eq10_e146_d_n11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let eq10_reactive_branch_derivatives: [f64; 19] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq15_e169, eq15_e169_d_n3, eq15_e169_d_n4, eq15_e169_d_n5, eq15_e169_d_n8, eq15_e169_d_n10, eq15_e169_d_n12, eq15_e169_d_b1, eq15_e169_q, eq15_e169_q_d_b1,) = {
    if (locals.var_guard20 != 0.0) {
        let eq15_e165: f64 = (bi1 * locals.var_rc1);
        let eq15_e165_d_n3: f64 = (bi1 * locals.var_rc1_dn3);
        let eq15_e165_d_n4: f64 = (bi1 * locals.var_rc1_dn4);
        let eq15_e165_d_n5: f64 = (bi1 * locals.var_rc1_dn5);
        let eq15_e165_d_n8: f64 = (bi1 * locals.var_rc1_dn8);
        let eq15_e165_d_n10: f64 = (bi1 * locals.var_rc1_dn10);
        let eq15_e165_d_n12: f64 = (bi1 * locals.var_rc1_dn12);
        let eq15_e165_d_b1: f64 = (locals.var_rc1 + (bi1 * locals.var_rc1_db1));
        let eq15_e166_q: f64 = locals.var_t0_rv;
        let eq15_e167: f64 = (eq15_e165 + locals.var_t0);
        let eq15_e167_d_n3: f64 = (eq15_e165_d_n3 + locals.var_t0_dn3);
        let eq15_e167_d_n4: f64 = (eq15_e165_d_n4 + locals.var_t0_dn4);
        let eq15_e167_d_n5: f64 = (eq15_e165_d_n5 + locals.var_t0_dn5);
        let eq15_e167_d_n8: f64 = (eq15_e165_d_n8 + locals.var_t0_dn8);
        let eq15_e167_d_n10: f64 = (eq15_e165_d_n10 + locals.var_t0_dn10);
        let eq15_e167_d_n12: f64 = (eq15_e165_d_n12 + locals.var_t0_dn12);
        let eq15_e167_d_b1: f64 = (eq15_e165_d_b1 + locals.var_t0_db1);
        let eq15_e167_q: f64 = eq15_e166_q;
        (eq15_e167, eq15_e167_d_n3, eq15_e167_d_n4, eq15_e167_d_n5, eq15_e167_d_n8, eq15_e167_d_n10, eq15_e167_d_n12, eq15_e167_d_b1, eq15_e167_q, locals.var_t0_rdb1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_potential_reactive_branch1(
            branches[1],
            branches[1],
            eq15_e169_q_d_b1,
        );
        let (eq18_e187, eq18_e187_d_n3, eq18_e187_d_n8, eq18_e187_d_n12, eq18_e187_q,) = {
    if (locals.var_guard21 != 0.0) {
        let eq18_e184: f64 = (locals.var_cdel_t * (nv12 - nv8));
        let eq18_e184_d_n3: f64 = (locals.var_cdel_t_dn3 * (nv12 - nv8));
        let eq18_e185_q: f64 = eq18_e184;
        (eq18_e184, eq18_e184_d_n3, (-locals.var_cdel_t), locals.var_cdel_t, eq18_e185_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node3(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes[3],
            multiplicity * (eq18_e187_d_n3),
            nodes[8],
            multiplicity * (eq18_e187_d_n8),
            nodes[12],
            multiplicity * (eq18_e187_d_n12),
        );
        let eq31_e269: f64 = (p.p54 * bi10);
        let eq31_e270_q: f64 = eq31_e269;
        stamper.stamp_potential_reactive_branch1(
            branches[10],
            branches[10],
            p.p54,
        );
        let eq35_e298: f64 = (p.p53 * bi14);
        let eq35_e299_q: f64 = eq35_e298;
        stamper.stamp_potential_reactive_branch1(
            branches[14],
            branches[14],
            p.p53,
        );
        let eq39_e327: f64 = (p.p52 * bi18);
        let eq39_e328_q: f64 = eq39_e327;
        stamper.stamp_potential_reactive_branch1(
            branches[18],
            branches[18],
            p.p52,
        );
        let (eq51_e429, eq51_e429_d_n3, eq51_e429_d_n17, eq51_e429_q,) = {
    if (((locals.var_guard29 != 0.0) && (locals.var_guard28 == 0.0)) && (p.p0 != 0.0)) {
        let eq51_e424: f64 = (-locals.var_ci);
        let eq51_e426: f64 = (eq51_e424 * (nv17 - 0.0));
        let eq51_e426_d_n3: f64 = ((-locals.var_ci_dn3) * (nv17 - 0.0));
        let eq51_e427_q: f64 = eq51_e426;
        (eq51_e426, eq51_e426_d_n3, eq51_e424, eq51_e427_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes[3],
            multiplicity * (eq51_e429_d_n3),
            nodes[17],
            multiplicity * (eq51_e429_d_n17),
        );
        let (eq64_e562, eq64_e562_d_n3, eq64_e562_q,) = {
    if (locals.var_guard44 != 0.0) {
        let eq64_e559: f64 = (p.p67 * (nv3 - 0.0));
        let eq64_e560_q: f64 = eq64_e559;
        (eq64_e559, p.p67, eq64_e560_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * (eq64_e562_d_n3),
        );
    }
}
