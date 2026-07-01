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
        let assign00_e1569: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e1569;

        let (assign10_e1574,) = {
    if (locals.var_guard1 != 0.0) {
        let assign10_e1572: f64 = 1.0;
        (assign10_e1572,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign10_e1574;

        let (assign20_e1580,) = {
    if (locals.var_guard1 == 0.0) {
        let assign20_e1578: f64 = (-1.0);
        (assign20_e1578,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign20_e1580;

        let assign30_e1583: f64 = (8.8541878176e-12 * 11.8);
        locals.var_epssi = assign30_e1583;

        let assign40_e1586: f64 = if p.p51 < 0.5 { 1.0 } else { 0.0 };
        locals.var_guard2 = assign40_e1586;

        let (assign50_e1590,) = {
    if (locals.var_guard2 != 0.0) {
        (0.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign50_e1590;

        let assign60_e1593: f64 = if p.p51 < 1.5 { 1.0 } else { 0.0 };
        locals.var_guard3 = assign60_e1593;

        let (assign70_e1600,) = {
    if ((locals.var_guard2 == 0.0) && (locals.var_guard3 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign70_e1600;

        let assign80_e1603: f64 = if p.p51 < 2.5 { 1.0 } else { 0.0 };
        locals.var_guard4 = assign80_e1603;

        let (assign90_e1613,) = {
    if (((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign90_e1613;

        let assign100_e1616: f64 = if p.p51 < 4.0 { 1.0 } else { 0.0 };
        locals.var_guard5 = assign100_e1616;

        let (assign110_e1629,) = {
    if ((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign110_e1629;

        let assign120_e1632: f64 = if p.p51 < 7.0 { 1.0 } else { 0.0 };
        locals.var_guard6 = assign120_e1632;

        let (assign130_e1648,) = {
    if (((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 == 0.0)) && (locals.var_guard6 != 0.0)) {
        (5.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign130_e1648;

        let (assign140_e1665,) = {
    if (((((locals.var_guard2 == 0.0) && (locals.var_guard3 == 0.0)) && (locals.var_guard4 == 0.0)) && (locals.var_guard5 == 0.0)) && (locals.var_guard6 == 0.0)) {
        (9.0,)
    } else {
        (locals.var_swnqs_i,)
    }
};
        locals.var_swnqs_i = assign140_e1665;

        locals.var_vnorm = 10.0;

        let assign170_e1670: f64 = (1.0 / locals.var_vnorm);
        locals.var_vnorm_inv = assign170_e1670;

        let assign180_e1673: f64 = (273.15 + p.p38);
        locals.var_tkr = assign180_e1673;

        let assign2190_e2704: f64 = ctx_temp;
        let assign2190_e2706: f64 = (assign2190_e2704 + p.p56);
        let assign2190_e2708: f64 = (assign2190_e2706 + p.p35);
        locals.var_tka = assign2190_e2708;

        let assign2200_e2711: f64 = (locals.var_tka / locals.var_tkr);
        locals.var_rta = assign2200_e2711;

        let assign2210_e2714: f64 = (locals.var_tka - locals.var_tkr);
        locals.var_delta = assign2210_e2714;

        let assign2220_e2717: f64 = (locals.var_tka * 1.3806505e-23);
        let assign2220_e2719: f64 = (assign2220_e2717 / 1.6021918e-19);
        locals.var_phita = assign2220_e2719;

        let assign2230_e2722: f64 = (1.0 / locals.var_phita);
        locals.var_inv_phita = assign2230_e2722;

        locals.var_tkd = locals.var_tka;

        let assign2250_e2726: f64 = (locals.var_tkd * locals.var_tkd);
        locals.var_tkd_sq = assign2250_e2726;

        let assign2260_e2729: f64 = (locals.var_tkd - locals.var_tkr);
        locals.var_delt = assign2260_e2729;

        let assign2270_e2732: f64 = (locals.var_tkr / locals.var_tkd);
        locals.var_rtn = assign2270_e2732;

        let assign2280_e2734: f64 = (locals.var_rtn).ln();
        locals.var_ln_rtn = assign2280_e2734;

        let assign2290_e2737: f64 = (locals.var_tkd * 1.3806505e-23);
        let assign2290_e2739: f64 = (assign2290_e2737 / 1.6021918e-19);
        locals.var_phit = assign2290_e2739;

        let assign2300_e2742: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign2300_e2742;

        let assign2310_e2746: f64 = (9.025e-5 * locals.var_tkd);
        let assign2310_e2747: f64 = (1.179 - assign2310_e2746);
        let assign2310_e2750: f64 = (3.05e-7 * locals.var_tkd_sq);
        let assign2310_e2751: f64 = (assign2310_e2747 - assign2310_e2750);
        locals.var_eg = assign2310_e2751;

        let assign2320_e2755: f64 = (0.00045 * locals.var_tkd);
        let assign2320_e2756: f64 = (1.045 + assign2320_e2755);
        let assign2320_e2760: f64 = (0.0014 * locals.var_tkd);
        let assign2320_e2761: f64 = (0.523 + assign2320_e2760);
        let assign2320_e2764: f64 = (1.48e-6 * locals.var_tkd_sq);
        let assign2320_e2765: f64 = (assign2320_e2761 - assign2320_e2764);
        let assign2320_e2766: f64 = (assign2320_e2756 * assign2320_e2765);
        let assign2320_e2768: f64 = (assign2320_e2766 * locals.var_tkd_sq);
        let assign2320_e2770: f64 = (assign2320_e2768 / 90000.0);
        locals.var_phibfac = assign2320_e2770;

        let (assign2330_e2776,) = {
    if (locals.var_phibfac > 0.001) {
        (locals.var_phibfac,)
    } else {
        (0.001,)
    }
};
        locals.var_phibfac = assign2330_e2776;

        let assign2340_e2779: f64 = (4.0 * 1.3806505e-23);
        let assign2340_e2781: f64 = (assign2340_e2779 * locals.var_tkd);
        locals.var_nt0 = assign2340_e2781;

        locals.var_nf_i = 1.0;

        locals.var_invnf = 1.0;

        locals.var_le = 0.0;

        locals.var_we = 0.0;

        locals.var_l_i = p.p0;

        locals.var_w_i = p.p1;

        locals.var_sa_i = p.p2;

        locals.var_sb_i = p.p3;

        locals.var_sd_i = p.p4;

        locals.var_sc_i = p.p8;

        let assign3640_e3629: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign3640_e3629;

        let (assign3650_e3638,) = {
    if (locals.var_guard34 != 0.0) {
        let (assign3650_e3636,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3650_e3636,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3650_e3638;

        let (assign3660_e3645,) = {
    if (locals.var_guard34 != 0.0) {
        let assign3660_e3642: f64 = (locals.var_nf_i + 0.5);
        let assign3660_e3643: f64 = (assign3660_e3642).floor();
        (assign3660_e3643,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3660_e3645;

        let (assign3670_e3651,) = {
    if (locals.var_guard34 != 0.0) {
        let assign3670_e3649: f64 = (1.0 / locals.var_nf_i);
        (assign3670_e3649,)
    } else {
        (locals.var_invnf,)
    }
};
        locals.var_invnf = assign3670_e3651;

        let assign3680_e3654: f64 = (locals.var_w_i * locals.var_invnf);
        let (assign3680_e3661,) = {
    if (assign3680_e3654 > 1e-9) {
        let assign3680_e3659: f64 = (locals.var_w_i * locals.var_invnf);
        (assign3680_e3659,)
    } else {
        (1e-9,)
    }
};
        locals.var_w_i = assign3680_e3661;

        locals.var_sca_i = p.p5;

        locals.var_scb_i = p.p6;

        locals.var_scc_i = p.p7;

        let assign3730_e3673: f64 = (1e-6 / locals.var_l_i);
        locals.var_il = assign3730_e3673;

        let assign3740_e3676: f64 = (1e-6 / locals.var_w_i);
        locals.var_iw = assign3740_e3676;

        let assign3750_e3681: f64 = (p.p189 * locals.var_il);
        let assign3750_e3682: f64 = (1.0 + assign3750_e3681);
        let assign3750_e3683: f64 = (p.p188 * assign3750_e3682);
        let assign3750_e3687: f64 = (p.p190 * locals.var_iw);
        let assign3750_e3688: f64 = (1.0 + assign3750_e3687);
        let assign3750_e3689: f64 = (assign3750_e3683 * assign3750_e3688);
        locals.var_dellps = assign3750_e3689;

        let assign3760_e3694: f64 = (p.p193 * locals.var_il);
        let assign3760_e3695: f64 = (1.0 + assign3760_e3694);
        let assign3760_e3696: f64 = (p.p192 * assign3760_e3695);
        let assign3760_e3700: f64 = (p.p194 * locals.var_iw);
        let assign3760_e3701: f64 = (1.0 + assign3760_e3700);
        let assign3760_e3702: f64 = (assign3760_e3696 * assign3760_e3701);
        locals.var_delwod = assign3760_e3702;

        let assign3770_e3705: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3770_e3708: f64 = (2.0 * p.p191);
        let assign3770_e3709: f64 = (assign3770_e3705 - assign3770_e3708);
        let (assign3770_e3720,) = {
    if (assign3770_e3709 > 1e-9) {
        let assign3770_e3714: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3770_e3717: f64 = (2.0 * p.p191);
        let assign3770_e3718: f64 = (assign3770_e3714 - assign3770_e3717);
        (assign3770_e3718,)
    } else {
        (1e-9,)
    }
};
        locals.var_le = assign3770_e3720;

        let assign3780_e3723: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3780_e3726: f64 = (2.0 * p.p195);
        let assign3780_e3727: f64 = (assign3780_e3723 - assign3780_e3726);
        let (assign3780_e3738,) = {
    if (assign3780_e3727 > 1e-9) {
        let assign3780_e3732: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3780_e3735: f64 = (2.0 * p.p195);
        let assign3780_e3736: f64 = (assign3780_e3732 - assign3780_e3735);
        (assign3780_e3736,)
    } else {
        (1e-9,)
    }
};
        locals.var_we = assign3780_e3738;

        let assign3790_e3741: f64 = (1e-6 / locals.var_le);
        locals.var_ile = assign3790_e3741;

        let assign3800_e3744: f64 = (locals.var_ile * locals.var_ile);
        locals.var_ile2 = assign3800_e3744;

        let assign3810_e3747: f64 = (1e-6 / locals.var_we);
        locals.var_iwe = assign3810_e3747;

        let assign3820_e3750: f64 = (1.0 / locals.var_iwe);
        locals.var_iiwe = assign3820_e3750;

        let assign3830_e3753: f64 = (locals.var_ile * locals.var_iwe);
        locals.var_iae = assign3830_e3753;

        let assign3840_e3756: f64 = (1.0 / locals.var_iae);
        locals.var_iiae = assign3840_e3756;

        let assign3850_e3759: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3850_e3762: f64 = (2.0 * p.p191);
        let assign3850_e3763: f64 = (assign3850_e3759 - assign3850_e3762);
        let assign3850_e3765: f64 = (assign3850_e3763 + p.p196);
        let (assign3850_e3778,) = {
    if (assign3850_e3765 > 1e-9) {
        let assign3850_e3770: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3850_e3773: f64 = (2.0 * p.p191);
        let assign3850_e3774: f64 = (assign3850_e3770 - assign3850_e3773);
        let assign3850_e3776: f64 = (assign3850_e3774 + p.p196);
        (assign3850_e3776,)
    } else {
        (1e-9,)
    }
};
        locals.var_lecv = assign3850_e3778;

        let assign3860_e3781: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3860_e3784: f64 = (2.0 * p.p195);
        let assign3860_e3785: f64 = (assign3860_e3781 - assign3860_e3784);
        let assign3860_e3787: f64 = (assign3860_e3785 + p.p197);
        let (assign3860_e3800,) = {
    if (assign3860_e3787 > 1e-9) {
        let assign3860_e3792: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3860_e3795: f64 = (2.0 * p.p195);
        let assign3860_e3796: f64 = (assign3860_e3792 - assign3860_e3795);
        let assign3860_e3798: f64 = (assign3860_e3796 + p.p197);
        (assign3860_e3798,)
    } else {
        (1e-9,)
    }
};
        locals.var_wecv = assign3860_e3800;

        let assign3870_e3803: f64 = (locals.var_wecv / 1e-6);
        locals.var_iiwecv = assign3870_e3803;

        let assign3880_e3806: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3880_e3808: f64 = (assign3880_e3806 + p.p196);
        let (assign3880_e3817,) = {
    if (assign3880_e3808 > 1e-9) {
        let assign3880_e3813: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3880_e3815: f64 = (assign3880_e3813 + p.p196);
        (assign3880_e3815,)
    } else {
        (1e-9,)
    }
};
        locals.var_lcv = assign3880_e3817;

        let assign3900_e3834: f64 = (locals.var_lcv / 1e-6);
        locals.var_iilcv = assign3900_e3834;

        locals.var_vfb_p = p.p57;

        locals.var_stvfb_p = p.p58;

        locals.var_st2vfb_p = p.p59;

        locals.var_tox_p = p.p60;

        locals.var_epsrox_p = p.p61;

        locals.var_neff_p = p.p62;

        locals.var_gfacnud_p = p.p63;

        locals.var_vsbnud_p = p.p64;

        locals.var_dvsbnud_p = p.p65;

        locals.var_dphib_p = p.p66;

        locals.var_np_p = p.p67;

        locals.var_toxov_p = p.p68;

        locals.var_toxovd_p = p.p69;

        locals.var_nov_p = p.p70;

        locals.var_novd_p = p.p71;

        locals.var_ct_p = p.p72;

        locals.var_ctg_p = p.p74;

        locals.var_ctb_p = p.p73;

        locals.var_stct_p = p.p75;

        locals.var_psce_p = p.p79;

        locals.var_psced_p = p.p81;

        locals.var_psceb_p = p.p80;

        locals.var_cf_p = p.p76;

        locals.var_cfd_p = p.p78;

        locals.var_cfb_p = p.p77;

        locals.var_betn_p = p.p82;

        locals.var_stbet_p = p.p83;

        locals.var_mue_p = p.p84;

        locals.var_stmue_p = p.p85;

        locals.var_themu_p = p.p86;

        locals.var_stthemu_p = p.p87;

        locals.var_cs_p = p.p88;

        locals.var_stcs_p = p.p89;

        locals.var_thecs_p = p.p90;

        locals.var_stthecs_p = p.p91;

        locals.var_xcor_p = p.p92;

        locals.var_stxcor_p = p.p93;

        locals.var_feta_p = p.p94;

        locals.var_rs_p = p.p95;

        locals.var_strs_p = p.p96;

        locals.var_rsb_p = p.p97;

        locals.var_rsg_p = p.p98;

        locals.var_thesat_p = p.p99;

        locals.var_stthesat_p = p.p100;

        locals.var_thesatb_p = p.p101;

        locals.var_thesatg_p = p.p102;

        locals.var_thesatt_p = p.p103;

        locals.var_ax_p = p.p104;

        locals.var_alp_p = p.p105;

        locals.var_alp1_p = p.p106;

        locals.var_alp2_p = p.p107;

        locals.var_vp_p = p.p108;

        locals.var_a1_p = p.p109;

        locals.var_a2_p = p.p110;

        locals.var_sta2_p = p.p111;

        locals.var_a3_p = p.p112;

        locals.var_a4_p = p.p113;

        locals.var_imaxii_p = p.p114;

        locals.var_gco_p = p.p115;

        locals.var_iginv_p = p.p116;

        locals.var_igov_p = p.p117;

        locals.var_igovd_p = p.p118;

        locals.var_stig_p = p.p119;

        locals.var_gc2_p = p.p120;

        locals.var_gc3_p = p.p121;

        locals.var_gc2ov_p = p.p120;

        let assign4620_e3949: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4620_e3951: f64 = if assign4620_e3949 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign4620_e3951;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign4630_e3955,) = {
    if (locals.var_guard35 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign4630_e3955;

        locals.var_gc3ov_p = p.p121;

        let assign4650_e3958: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4650_e3960: f64 = if assign4650_e3958 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign4650_e3960;

        let (assign4660_e3964,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign4660_e3964;

        locals.var_gc2ovd_p = locals.var_gc2ov_p;

        let assign4680_e3967: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4680_e3969: f64 = if assign4680_e3967 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign4680_e3969;

        let (assign4690_e3973,) = {
    if (locals.var_guard37 != 0.0) {
        (p.p124,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign4690_e3973;

        locals.var_gc3ovd_p = locals.var_gc3ov_p;

        let assign4710_e3976: f64 = if param_given[125] { 1.0 } else { 0.0 };
        let assign4710_e3978: f64 = if assign4710_e3976 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard38 = assign4710_e3978;

        let (assign4720_e3982,) = {
    if (locals.var_guard38 != 0.0) {
        (p.p125,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign4720_e3982;

        locals.var_chib_p = p.p126;

        locals.var_agidl_p = p.p127;

        locals.var_agidld_p = p.p128;

        locals.var_bgidl_p = p.p129;

        locals.var_bgidld_p = p.p130;

        locals.var_stbgidl_p = p.p131;

        locals.var_stbgidld_p = p.p132;

        locals.var_cgidl_p = p.p133;

        locals.var_cgidld_p = p.p134;

        locals.var_cox_p = p.p135;

        locals.var_delvtac_p = p.p136;

        locals.var_facneffac_p = p.p137;

        locals.var_thesatac_p = p.p99;

        let assign4860_e3997: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4860_e3999: f64 = if assign4860_e3997 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign4860_e3999;

        let (assign4870_e4003,) = {
    if (locals.var_guard39 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign4870_e4003;

        locals.var_axac_p = p.p104;

        let assign4890_e4006: f64 = if param_given[139] { 1.0 } else { 0.0 };
        let assign4890_e4008: f64 = if assign4890_e4006 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign4890_e4008;

        let (assign4900_e4012,) = {
    if (locals.var_guard40 != 0.0) {
        (p.p139,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign4900_e4012;

        locals.var_alpac_p = p.p140;

        locals.var_alp1ac_p = p.p141;

        locals.var_cgov_p = p.p142;

        locals.var_cgovd_p = p.p143;

        locals.var_fcgovacc_p = p.p144;

        locals.var_fcgovaccd_p = p.p145;

        locals.var_cgovaccg_p = p.p146;

        locals.var_cgbov_p = p.p147;

        locals.var_cinr_p = p.p148;

        locals.var_cinrd_p = p.p149;

        locals.var_dvfbinr_p = p.p150;

        locals.var_fcinrdep_p = p.p151;

        locals.var_fcinracc_p = p.p152;

        locals.var_axinr_p = p.p153;

        locals.var_fnt_p = p.p156;

        locals.var_fntexc_p = p.p157;

        locals.var_vfbedge_p = p.p162;

        locals.var_stvfbedge_p = p.p163;

        locals.var_dphibedge_p = p.p164;

        locals.var_neffedge_p = p.p165;

        locals.var_ctedge_p = p.p166;

        locals.var_betnedge_p = p.p167;

        locals.var_stbetedge_p = p.p168;

        locals.var_psceedge_p = p.p169;

        locals.var_pscebedge_p = p.p170;

        locals.var_pscededge_p = p.p171;

        locals.var_cfedge_p = p.p172;

        locals.var_cfdedge_p = p.p174;

        locals.var_cfbedge_p = p.p173;

        locals.var_munqs_p = p.p187;

        let assign5390_e4063: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign5390_e4063;

        let (assign5400_e4081,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5400_e4069: f64 = (locals.var_ile).powf(p.p200);
        let assign5400_e4070: f64 = (p.p199 * assign5400_e4069);
        let assign5400_e4071: f64 = (p.p198 + assign5400_e4070);
        let assign5400_e4074: f64 = (p.p201 * locals.var_iwe);
        let assign5400_e4075: f64 = (assign5400_e4071 + assign5400_e4074);
        let assign5400_e4078: f64 = (p.p202 * locals.var_iae);
        let assign5400_e4079: f64 = (assign5400_e4075 + assign5400_e4078);
        (assign5400_e4079,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign5400_e4081;

        let (assign5410_e4097,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5410_e4086: f64 = (p.p204 * locals.var_ile);
        let assign5410_e4087: f64 = (p.p203 + assign5410_e4086);
        let assign5410_e4090: f64 = (p.p205 * locals.var_iwe);
        let assign5410_e4091: f64 = (assign5410_e4087 + assign5410_e4090);
        let assign5410_e4094: f64 = (p.p206 * locals.var_iae);
        let assign5410_e4095: f64 = (assign5410_e4091 + assign5410_e4094);
        (assign5410_e4095,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign5410_e4097;

        let (assign5420_e4101,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p207,)
    } else {
        (locals.var_st2vfb_p,)
    }
};
        locals.var_st2vfb_p = assign5420_e4101;

        let (assign5430_e4105,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p208,)
    } else {
        (locals.var_tox_p,)
    }
};
        locals.var_tox_p = assign5430_e4105;

        let (assign5440_e4109,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p209,)
    } else {
        (locals.var_epsrox_p,)
    }
};
        locals.var_epsrox_p = assign5440_e4109;

        let (assign5450_e4142,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5450_e4115: f64 = (p.p211 * locals.var_iwe);
        let assign5450_e4119: f64 = (locals.var_we / p.p212);
        let assign5450_e4120: f64 = (1.0 + assign5450_e4119);
        let assign5450_e4121: f64 = (assign5450_e4120).ln();
        let assign5450_e4122: f64 = (assign5450_e4115 * assign5450_e4121);
        let assign5450_e4123: f64 = (1.0 + assign5450_e4122);
        let (assign5450_e4139,) = {
            if (assign5450_e4123 > 0.001) {
                let assign5450_e4129: f64 = (p.p211 * locals.var_iwe);
                let assign5450_e4133: f64 = (locals.var_we / p.p212);
                let assign5450_e4134: f64 = (1.0 + assign5450_e4133);
                let assign5450_e4135: f64 = (assign5450_e4134).ln();
                let assign5450_e4136: f64 = (assign5450_e4129 * assign5450_e4135);
                let assign5450_e4137: f64 = (1.0 + assign5450_e4136);
                (assign5450_e4137,)
            } else {
                (0.001,)
            }
        };
        let assign5450_e4140: f64 = (p.p210 * assign5450_e4139);
        (assign5450_e4140,)
    } else {
        (locals.var_nsub0e,)
    }
};
        locals.var_nsub0e = assign5450_e4142;

        let (assign5460_e4175,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5460_e4148: f64 = (p.p214 * locals.var_iwe);
        let assign5460_e4152: f64 = (locals.var_we / p.p215);
        let assign5460_e4153: f64 = (1.0 + assign5460_e4152);
        let assign5460_e4154: f64 = (assign5460_e4153).ln();
        let assign5460_e4155: f64 = (assign5460_e4148 * assign5460_e4154);
        let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
        let (assign5460_e4172,) = {
            if (assign5460_e4156 > 0.001) {
                let assign5460_e4162: f64 = (p.p214 * locals.var_iwe);
                let assign5460_e4166: f64 = (locals.var_we / p.p215);
                let assign5460_e4167: f64 = (1.0 + assign5460_e4166);
                let assign5460_e4168: f64 = (assign5460_e4167).ln();
                let assign5460_e4169: f64 = (assign5460_e4162 * assign5460_e4168);
                let assign5460_e4170: f64 = (1.0 + assign5460_e4169);
                (assign5460_e4170,)
            } else {
                (0.001,)
            }
        };
        let assign5460_e4173: f64 = (p.p213 * assign5460_e4172);
        (assign5460_e4173,)
    } else {
        (locals.var_npcke,)
    }
};
        locals.var_npcke = assign5460_e4175;

        let (assign5470_e4208,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5470_e4181: f64 = (p.p217 * locals.var_iwe);
        let assign5470_e4185: f64 = (locals.var_we / p.p215);
        let assign5470_e4186: f64 = (1.0 + assign5470_e4185);
        let assign5470_e4187: f64 = (assign5470_e4186).ln();
        let assign5470_e4188: f64 = (assign5470_e4181 * assign5470_e4187);
        let assign5470_e4189: f64 = (1.0 + assign5470_e4188);
        let (assign5470_e4205,) = {
            if (assign5470_e4189 > 0.001) {
                let assign5470_e4195: f64 = (p.p217 * locals.var_iwe);
                let assign5470_e4199: f64 = (locals.var_we / p.p215);
                let assign5470_e4200: f64 = (1.0 + assign5470_e4199);
                let assign5470_e4201: f64 = (assign5470_e4200).ln();
                let assign5470_e4202: f64 = (assign5470_e4195 * assign5470_e4201);
                let assign5470_e4203: f64 = (1.0 + assign5470_e4202);
                (assign5470_e4203,)
            } else {
                (0.001,)
            }
        };
        let assign5470_e4206: f64 = (p.p216 * assign5470_e4205);
        (assign5470_e4206,)
    } else {
        (locals.var_lpcke,)
    }
};
        locals.var_lpcke = assign5470_e4208;

        let assign5480_e4212: f64 = (2.0 * locals.var_lpcke);
        let assign5480_e4213: f64 = if locals.var_le > assign5480_e4212 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign5480_e4213;

        let (assign5490_e4219,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        (75000000000.0,)
    } else {
        (locals.var_aa,)
    }
};
        locals.var_aa = assign5490_e4219;

        let (assign5500_e4233,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        let assign5500_e4226: f64 = (0.5 * locals.var_npcke);
        let assign5500_e4227: f64 = (locals.var_nsub0e + assign5500_e4226);
        let assign5500_e4228: f64 = (assign5500_e4227).sqrt();
        let assign5500_e4230: f64 = (locals.var_nsub0e).sqrt();
        let assign5500_e4231: f64 = (assign5500_e4228 - assign5500_e4230);
        (assign5500_e4231,)
    } else {
        (locals.var_bb,)
    }
};
        locals.var_bb = assign5500_e4233;

        let (assign5510_e4258,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        let assign5510_e4238: f64 = (locals.var_nsub0e).sqrt();
        let assign5510_e4243: f64 = (2.0 * locals.var_lpcke);
        let assign5510_e4245: f64 = (assign5510_e4243 / locals.var_le);
        let assign5510_e4248: f64 = (locals.var_bb / locals.var_aa);
        let assign5510_e4249: f64 = (assign5510_e4248).exp();
        let assign5510_e4251: f64 = (assign5510_e4249 - 1.0);
        let assign5510_e4252: f64 = (assign5510_e4245 * assign5510_e4251);
        let assign5510_e4253: f64 = (1.0 + assign5510_e4252);
        let assign5510_e4254: f64 = (assign5510_e4253).ln();
        let assign5510_e4255: f64 = (locals.var_aa * assign5510_e4254);
        let assign5510_e4256: f64 = (assign5510_e4238 + assign5510_e4255);
        (assign5510_e4256,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5510_e4258;

        let (assign5520_e4266,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard42 != 0.0)) {
        let assign5520_e4264: f64 = (locals.var_nsub * locals.var_nsub);
        (assign5520_e4264,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5520_e4266;

        let assign5530_e4269: f64 = if locals.var_le >= locals.var_lpcke { 1.0 } else { 0.0 };
        locals.var_guard43 = assign5530_e4269;

        let (assign5540_e4284,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 != 0.0)) {
        let assign5540_e4279: f64 = (locals.var_npcke * locals.var_lpcke);
        let assign5540_e4281: f64 = (assign5540_e4279 / locals.var_le);
        let assign5540_e4282: f64 = (locals.var_nsub0e + assign5540_e4281);
        (assign5540_e4282,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5540_e4284;

        let (assign5550_e4302,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 == 0.0)) {
        let assign5550_e4297: f64 = (locals.var_le / locals.var_lpcke);
        let assign5550_e4298: f64 = (2.0 - assign5550_e4297);
        let assign5550_e4299: f64 = (locals.var_npcke * assign5550_e4298);
        let assign5550_e4300: f64 = (locals.var_nsub0e + assign5550_e4299);
        (assign5550_e4300,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5550_e4302;

        let (assign5560_e4316,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5560_e4308: f64 = (p.p218 * locals.var_ile);
        let assign5560_e4309: f64 = (1.0 - assign5560_e4308);
        let assign5560_e4312: f64 = (p.p219 * locals.var_ile2);
        let assign5560_e4313: f64 = (assign5560_e4309 - assign5560_e4312);
        let assign5560_e4314: f64 = (locals.var_nsub * assign5560_e4313);
        (assign5560_e4314,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign5560_e4316;

        let (assign5570_e4334,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5570_e4322: f64 = (locals.var_ile).powf(p.p222);
        let assign5570_e4323: f64 = (p.p221 * assign5570_e4322);
        let assign5570_e4324: f64 = (p.p220 + assign5570_e4323);
        let assign5570_e4327: f64 = (p.p223 * locals.var_iwe);
        let assign5570_e4328: f64 = (assign5570_e4324 + assign5570_e4327);
        let assign5570_e4331: f64 = (p.p224 * locals.var_iae);
        let assign5570_e4332: f64 = (assign5570_e4328 + assign5570_e4331);
        (assign5570_e4332,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign5570_e4334;

        let (assign5580_e4338,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p225,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign5580_e4338;

        let (assign5590_e4342,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p226,)
    } else {
        (locals.var_dvsbnud_p,)
    }
};
        locals.var_dvsbnud_p = assign5590_e4342;

        let (assign5600_e4360,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5600_e4348: f64 = (locals.var_ile).powf(p.p229);
        let assign5600_e4349: f64 = (p.p228 * assign5600_e4348);
        let assign5600_e4350: f64 = (p.p227 + assign5600_e4349);
        let assign5600_e4353: f64 = (p.p230 * locals.var_iwe);
        let assign5600_e4354: f64 = (assign5600_e4350 + assign5600_e4353);
        let assign5600_e4357: f64 = (p.p231 * locals.var_iae);
        let assign5600_e4358: f64 = (assign5600_e4354 + assign5600_e4357);
        (assign5600_e4358,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign5600_e4360;

        let (assign5610_e4379,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5610_e4367: f64 = (p.p233 * locals.var_ile);
        let assign5610_e4368: f64 = (1.0 + assign5610_e4367);
        let (assign5610_e4376,) = {
            if (1e-6 > assign5610_e4368) {
                (1e-6,)
            } else {
                let assign5610_e4374: f64 = (p.p233 * locals.var_ile);
                let assign5610_e4375: f64 = (1.0 + assign5610_e4374);
                (assign5610_e4375,)
            }
        };
        let assign5610_e4377: f64 = (p.p232 * assign5610_e4376);
        (assign5610_e4377,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign5610_e4379;

        let (assign5620_e4383,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p234,)
    } else {
        (locals.var_toxov_p,)
    }
};
        locals.var_toxov_p = assign5620_e4383;

        let (assign5630_e4387,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p235,)
    } else {
        (locals.var_toxovd_p,)
    }
};
        locals.var_toxovd_p = assign5630_e4387;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5640_e4391,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p238,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign5640_e4391;

        let (assign5650_e4395,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p239,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign5650_e4395;

        let (assign5660_e4417,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5660_e4401: f64 = (locals.var_ile).powf(p.p242);
        let assign5660_e4402: f64 = (p.p241 * assign5660_e4401);
        let assign5660_e4403: f64 = (p.p240 + assign5660_e4402);
        let assign5660_e4407: f64 = (p.p243 * locals.var_iwe);
        let assign5660_e4408: f64 = (1.0 + assign5660_e4407);
        let assign5660_e4409: f64 = (assign5660_e4403 * assign5660_e4408);
        let assign5660_e4413: f64 = (p.p244 * locals.var_iae);
        let assign5660_e4414: f64 = (1.0 + assign5660_e4413);
        let assign5660_e4415: f64 = (assign5660_e4409 * assign5660_e4414);
        (assign5660_e4415,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign5660_e4417;

        let (assign5670_e4421,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p246,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign5670_e4421;

        let (assign5680_e4425,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p245,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign5680_e4425;

        let (assign5690_e4429,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p247,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign5690_e4429;

        let (assign5700_e4443,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5700_e4434: f64 = (locals.var_ile).powf(p.p249);
        let assign5700_e4435: f64 = (p.p248 * assign5700_e4434);
        let assign5700_e4439: f64 = (p.p250 * locals.var_iwe);
        let assign5700_e4440: f64 = (1.0 + assign5700_e4439);
        let assign5700_e4441: f64 = (assign5700_e4435 * assign5700_e4440);
        (assign5700_e4441,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign5700_e4443;

        let (assign5710_e4447,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p252,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign5710_e4447;

        let (assign5720_e4451,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p251,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign5720_e4451;

        let (assign5730_e4465,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5730_e4456: f64 = (locals.var_ile).powf(p.p254);
        let assign5730_e4457: f64 = (p.p253 * assign5730_e4456);
        let assign5730_e4461: f64 = (p.p255 * locals.var_iwe);
        let assign5730_e4462: f64 = (1.0 + assign5730_e4461);
        let assign5730_e4463: f64 = (assign5730_e4457 * assign5730_e4462);
        (assign5730_e4463,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign5730_e4465;

        let (assign5740_e4469,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p257,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign5740_e4469;

        let (assign5750_e4473,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p256,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign5750_e4473;

        let (assign5760_e4483,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5760_e4479: f64 = (p.p260 * locals.var_iwe);
        let assign5760_e4480: f64 = (1.0 + assign5760_e4479);
        let assign5760_e4481: f64 = (p.p259 * assign5760_e4480);
        (assign5760_e4481,)
    } else {
        (locals.var_fbet1e,)
    }
};
        locals.var_fbet1e = assign5760_e4483;

        let (assign5770_e4502,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5770_e4489: f64 = (p.p262 * locals.var_iwe);
        let assign5770_e4490: f64 = (1.0 + assign5770_e4489);
        let (assign5770_e4499,) = {
            if (assign5770_e4490 > 0.001) {
                let assign5770_e4496: f64 = (p.p262 * locals.var_iwe);
                let assign5770_e4497: f64 = (1.0 + assign5770_e4496);
                (assign5770_e4497,)
            } else {
                (0.001,)
            }
        };
        let assign5770_e4500: f64 = (p.p261 * assign5770_e4499);
        (assign5770_e4500,)
    } else {
        (locals.var_lp1e,)
    }
};
        locals.var_lp1e = assign5770_e4502;

        let (assign5780_e4534,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5780_e4507: f64 = (locals.var_fbet1e * locals.var_lp1e);
        let assign5780_e4509: f64 = (assign5780_e4507 / locals.var_le);
        let assign5780_e4512: f64 = (-locals.var_le);
        let assign5780_e4514: f64 = (assign5780_e4512 / locals.var_lp1e);
        let assign5780_e4515: f64 = (assign5780_e4514).exp();
        let assign5780_e4516: f64 = (1.0 - assign5780_e4515);
        let assign5780_e4517: f64 = (assign5780_e4509 * assign5780_e4516);
        let assign5780_e4518: f64 = (1.0 + assign5780_e4517);
        let assign5780_e4521: f64 = (p.p263 * p.p264);
        let assign5780_e4523: f64 = (assign5780_e4521 / locals.var_le);
        let assign5780_e4526: f64 = (-locals.var_le);
        let assign5780_e4528: f64 = (assign5780_e4526 / p.p264);
        let assign5780_e4529: f64 = (assign5780_e4528).exp();
        let assign5780_e4530: f64 = (1.0 - assign5780_e4529);
        let assign5780_e4531: f64 = (assign5780_e4523 * assign5780_e4530);
        let assign5780_e4532: f64 = (assign5780_e4518 + assign5780_e4531);
        (assign5780_e4532,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5780_e4534;

        let (assign5790_e4543,) = {
    if (locals.var_guard41 != 0.0) {
        let (assign5790_e4541,) = {
            if (locals.var_gpe > 1e-15) {
                (locals.var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5790_e4541,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5790_e4543;

        let (assign5800_e4562,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5800_e4548: f64 = (p.p265 * locals.var_iwe);
        let assign5800_e4549: f64 = (1.0 + assign5800_e4548);
        let assign5800_e4552: f64 = (p.p266 * locals.var_iwe);
        let assign5800_e4556: f64 = (locals.var_we / p.p267);
        let assign5800_e4557: f64 = (1.0 + assign5800_e4556);
        let assign5800_e4558: f64 = (assign5800_e4557).ln();
        let assign5800_e4559: f64 = (assign5800_e4552 * assign5800_e4558);
        let assign5800_e4560: f64 = (assign5800_e4549 + assign5800_e4559);
        (assign5800_e4560,)
    } else {
        (locals.var_gwe,)
    }
};
        locals.var_gwe = assign5800_e4562;

        let (assign5810_e4574,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5810_e4566: f64 = (p.p258 * locals.var_we);
        let assign5810_e4569: f64 = (locals.var_gpe * locals.var_le);
        let assign5810_e4570: f64 = (assign5810_e4566 / assign5810_e4569);
        let assign5810_e4572: f64 = (assign5810_e4570 * locals.var_gwe);
        (assign5810_e4572,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign5810_e4574;

        let (assign5820_e4590,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5820_e4579: f64 = (p.p269 * locals.var_ile);
        let assign5820_e4580: f64 = (p.p268 + assign5820_e4579);
        let assign5820_e4583: f64 = (p.p270 * locals.var_iwe);
        let assign5820_e4584: f64 = (assign5820_e4580 + assign5820_e4583);
        let assign5820_e4587: f64 = (p.p271 * locals.var_iae);
        let assign5820_e4588: f64 = (assign5820_e4584 + assign5820_e4587);
        (assign5820_e4588,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign5820_e4590;

        let (assign5830_e4600,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5830_e4596: f64 = (p.p273 * locals.var_iwe);
        let assign5830_e4597: f64 = (1.0 + assign5830_e4596);
        let assign5830_e4598: f64 = (p.p272 * assign5830_e4597);
        (assign5830_e4598,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign5830_e4600;

        let (assign5840_e4604,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p274,)
    } else {
        (locals.var_stmue_p,)
    }
};
        locals.var_stmue_p = assign5840_e4604;

        let (assign5850_e4608,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p275,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign5850_e4608;

        let (assign5860_e4612,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_stthemu_p,)
    }
};
        locals.var_stthemu_p = assign5860_e4612;

        let (assign5870_e4634,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5870_e4618: f64 = (locals.var_ile).powf(p.p279);
        let assign5870_e4619: f64 = (p.p278 * assign5870_e4618);
        let assign5870_e4620: f64 = (p.p277 + assign5870_e4619);
        let assign5870_e4624: f64 = (p.p280 * locals.var_iwe);
        let assign5870_e4625: f64 = (1.0 + assign5870_e4624);
        let assign5870_e4626: f64 = (assign5870_e4620 * assign5870_e4625);
        let assign5870_e4630: f64 = (p.p281 * locals.var_iae);
        let assign5870_e4631: f64 = (1.0 + assign5870_e4630);
        let assign5870_e4632: f64 = (assign5870_e4626 * assign5870_e4631);
        (assign5870_e4632,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign5870_e4634;

        let (assign5880_e4638,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p282,)
    } else {
        (locals.var_stcs_p,)
    }
};
        locals.var_stcs_p = assign5880_e4638;

        let (assign5890_e4642,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p283,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign5890_e4642;

        let (assign5900_e4646,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p284,)
    } else {
        (locals.var_stthecs_p,)
    }
};
        locals.var_stthecs_p = assign5900_e4646;

        let (assign5910_e4668,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5910_e4652: f64 = (p.p286 * locals.var_ile);
        let assign5910_e4653: f64 = (1.0 + assign5910_e4652);
        let assign5910_e4654: f64 = (p.p285 * assign5910_e4653);
        let assign5910_e4658: f64 = (p.p287 * locals.var_iwe);
        let assign5910_e4659: f64 = (1.0 + assign5910_e4658);
        let assign5910_e4660: f64 = (assign5910_e4654 * assign5910_e4659);
        let assign5910_e4664: f64 = (p.p288 * locals.var_iae);
        let assign5910_e4665: f64 = (1.0 + assign5910_e4664);
        let assign5910_e4666: f64 = (assign5910_e4660 * assign5910_e4665);
        (assign5910_e4666,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign5910_e4668;

        let (assign5920_e4672,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p289,)
    } else {
        (locals.var_stxcor_p,)
    }
};
        locals.var_stxcor_p = assign5920_e4672;

        let (assign5930_e4676,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p290,)
    } else {
        (locals.var_feta_p,)
    }
};
        locals.var_feta_p = assign5930_e4676;

        let (assign5940_e4688,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5940_e4680: f64 = (p.p291 * locals.var_iwe);
        let assign5940_e4684: f64 = (p.p292 * locals.var_iwe);
        let assign5940_e4685: f64 = (1.0 + assign5940_e4684);
        let assign5940_e4686: f64 = (assign5940_e4680 * assign5940_e4685);
        (assign5940_e4686,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign5940_e4688;

        let (assign5950_e4692,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p293,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign5950_e4692;

        let (assign5960_e4696,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p294,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign5960_e4696;

        let (assign5970_e4700,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p295,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign5970_e4700;

        let (assign5980_e4726,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5980_e4705: f64 = (p.p297 * locals.var_gwe);
        let assign5980_e4707: f64 = (assign5980_e4705 / locals.var_gpe);
        let assign5980_e4710: f64 = (locals.var_ile).powf(p.p298);
        let assign5980_e4711: f64 = (assign5980_e4707 * assign5980_e4710);
        let assign5980_e4712: f64 = (p.p296 + assign5980_e4711);
        let assign5980_e4716: f64 = (p.p299 * locals.var_iwe);
        let assign5980_e4717: f64 = (1.0 + assign5980_e4716);
        let assign5980_e4718: f64 = (assign5980_e4712 * assign5980_e4717);
        let assign5980_e4722: f64 = (p.p300 * locals.var_iae);
        let assign5980_e4723: f64 = (1.0 + assign5980_e4722);
        let assign5980_e4724: f64 = (assign5980_e4718 * assign5980_e4723);
        (assign5980_e4724,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign5980_e4726;

        let (assign5990_e4742,) = {
    if (locals.var_guard41 != 0.0) {
        let assign5990_e4731: f64 = (p.p302 * locals.var_ile);
        let assign5990_e4732: f64 = (p.p301 + assign5990_e4731);
        let assign5990_e4735: f64 = (p.p303 * locals.var_iwe);
        let assign5990_e4736: f64 = (assign5990_e4732 + assign5990_e4735);
        let assign5990_e4739: f64 = (p.p304 * locals.var_iae);
        let assign5990_e4740: f64 = (assign5990_e4736 + assign5990_e4739);
        (assign5990_e4740,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign5990_e4742;

        let (assign6000_e4746,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p305,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign6000_e4746;

        let (assign6010_e4750,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p306,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign6010_e4750;

        let (assign6020_e4754,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p307,)
    } else {
        (locals.var_thesatt_p,)
    }
};
        locals.var_thesatt_p = assign6020_e4754;

        let (assign6030_e4764,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6030_e4760: f64 = (p.p309 * locals.var_ile);
        let assign6030_e4761: f64 = (1.0 + assign6030_e4760);
        let assign6030_e4762: f64 = (p.p308 / assign6030_e4761);
        (assign6030_e4762,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign6030_e4764;

        let (assign6040_e4778,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6040_e4769: f64 = (locals.var_ile).powf(p.p311);
        let assign6040_e4770: f64 = (p.p310 * assign6040_e4769);
        let assign6040_e4774: f64 = (p.p312 * locals.var_iwe);
        let assign6040_e4775: f64 = (1.0 + assign6040_e4774);
        let assign6040_e4776: f64 = (assign6040_e4770 * assign6040_e4775);
        (assign6040_e4776,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign6040_e4778;

        let (assign6050_e4784,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6050_e4782: f64 = (locals.var_ile).powf(p.p314);
        (assign6050_e4782,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6050_e4784;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6060_e4804,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6060_e4788: f64 = (p.p313 * locals.var_tmpx);
        let assign6060_e4792: f64 = (p.p316 * locals.var_iwe);
        let assign6060_e4793: f64 = (1.0 + assign6060_e4792);
        let assign6060_e4794: f64 = (assign6060_e4788 * assign6060_e4793);
        let assign6060_e4798: f64 = (p.p315 * locals.var_ile);
        let assign6060_e4800: f64 = (assign6060_e4798 * locals.var_tmpx);
        let assign6060_e4801: f64 = (1.0 + assign6060_e4800);
        let assign6060_e4802: f64 = (assign6060_e4794 / assign6060_e4801);
        (assign6060_e4802,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign6060_e4804;

        let (assign6070_e4810,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6070_e4808: f64 = (locals.var_ile).powf(p.p318);
        (assign6070_e4808,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6070_e4810;

        let (assign6080_e4830,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6080_e4814: f64 = (p.p317 * locals.var_tmpx);
        let assign6080_e4818: f64 = (p.p320 * locals.var_iwe);
        let assign6080_e4819: f64 = (1.0 + assign6080_e4818);
        let assign6080_e4820: f64 = (assign6080_e4814 * assign6080_e4819);
        let assign6080_e4824: f64 = (p.p319 * locals.var_ile);
        let assign6080_e4826: f64 = (assign6080_e4824 * locals.var_tmpx);
        let assign6080_e4827: f64 = (1.0 + assign6080_e4826);
        let assign6080_e4828: f64 = (assign6080_e4820 / assign6080_e4827);
        (assign6080_e4828,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign6080_e4830;

        let (assign6090_e4834,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p321,)
    } else {
        (locals.var_vp_p,)
    }
};
        locals.var_vp_p = assign6090_e4834;

        let (assign6100_e4850,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6100_e4840: f64 = (p.p323 * locals.var_ile);
        let assign6100_e4841: f64 = (1.0 + assign6100_e4840);
        let assign6100_e4842: f64 = (p.p322 * assign6100_e4841);
        let assign6100_e4846: f64 = (p.p324 * locals.var_iwe);
        let assign6100_e4847: f64 = (1.0 + assign6100_e4846);
        let assign6100_e4848: f64 = (assign6100_e4842 * assign6100_e4847);
        (assign6100_e4848,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign6100_e4850;

        let (assign6110_e4854,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p325,)
    } else {
        (locals.var_a2_p,)
    }
};
        locals.var_a2_p = assign6110_e4854;

        let (assign6120_e4858,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p326,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign6120_e4858;

        let (assign6130_e4874,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6130_e4864: f64 = (p.p328 * locals.var_ile);
        let assign6130_e4865: f64 = (1.0 + assign6130_e4864);
        let assign6130_e4866: f64 = (p.p327 * assign6130_e4865);
        let assign6130_e4870: f64 = (p.p329 * locals.var_iwe);
        let assign6130_e4871: f64 = (1.0 + assign6130_e4870);
        let assign6130_e4872: f64 = (assign6130_e4866 * assign6130_e4871);
        (assign6130_e4872,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign6130_e4874;

        let (assign6140_e4890,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6140_e4880: f64 = (p.p331 * locals.var_ile);
        let assign6140_e4881: f64 = (1.0 + assign6140_e4880);
        let assign6140_e4882: f64 = (p.p330 * assign6140_e4881);
        let assign6140_e4886: f64 = (p.p332 * locals.var_iwe);
        let assign6140_e4887: f64 = (1.0 + assign6140_e4886);
        let assign6140_e4888: f64 = (assign6140_e4882 * assign6140_e4887);
        (assign6140_e4888,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign6140_e4890;

        let (assign6150_e4894,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p333,)
    } else {
        (locals.var_imaxii_p,)
    }
};
        locals.var_imaxii_p = assign6150_e4894;

        let (assign6160_e4898,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p334,)
    } else {
        (locals.var_gco_p,)
    }
};
        locals.var_gco_p = assign6160_e4898;

        let (assign6170_e4904,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6170_e4902: f64 = (p.p335 / locals.var_iae);
        (assign6170_e4902,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign6170_e4904;

        let (assign6180_e4914,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6180_e4908: f64 = (p.p336 * p.p236);
        let assign6180_e4911: f64 = (1e-6 * locals.var_iwe);
        let assign6180_e4912: f64 = (assign6180_e4908 / assign6180_e4911);
        (assign6180_e4912,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign6180_e4914;

        let (assign6190_e4924,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6190_e4918: f64 = (p.p337 * p.p237);
        let assign6190_e4921: f64 = (1e-6 * locals.var_iwe);
        let assign6190_e4922: f64 = (assign6190_e4918 / assign6190_e4921);
        (assign6190_e4922,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign6190_e4924;

        let (assign6200_e4928,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p338,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign6200_e4928;

        let (assign6210_e4932,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p339,)
    } else {
        (locals.var_gc2_p,)
    }
};
        locals.var_gc2_p = assign6210_e4932;

        let (assign6220_e4936,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p340,)
    } else {
        (locals.var_gc3_p,)
    }
};
        locals.var_gc3_p = assign6220_e4936;

        let (assign6230_e4940,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p339,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6230_e4940;

        let assign6240_e4942: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6240_e4944: f64 = if assign6240_e4942 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign6240_e4944;

        let (assign6250_e4950,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard44 != 0.0)) {
        (p.p341,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6250_e4950;

        let (assign6260_e4954,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p340,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6260_e4954;

        let assign6270_e4956: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6270_e4958: f64 = if assign6270_e4956 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign6270_e4958;

        let (assign6280_e4964,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard45 != 0.0)) {
        (p.p342,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6280_e4964;

        let (assign6290_e4968,) = {
    if (locals.var_guard41 != 0.0) {
        (locals.var_gc2ov_p,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6290_e4968;

        let assign6300_e4970: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6300_e4972: f64 = if assign6300_e4970 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign6300_e4972;

        let (assign6310_e4978,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard46 != 0.0)) {
        (p.p343,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6310_e4978;

        let (assign6320_e4982,) = {
    if (locals.var_guard41 != 0.0) {
        (locals.var_gc3ov_p,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6320_e4982;

        let assign6330_e4984: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6330_e4986: f64 = if assign6330_e4984 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign6330_e4986;

        let (assign6340_e4992,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard47 != 0.0)) {
        (p.p344,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6340_e4992;

        let (assign6350_e4996,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p345,)
    } else {
        (locals.var_chib_p,)
    }
};
        locals.var_chib_p = assign6350_e4996;

        let (assign6360_e5006,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6360_e5000: f64 = (p.p346 * p.p236);
        let assign6360_e5003: f64 = (1e-6 * locals.var_iwe);
        let assign6360_e5004: f64 = (assign6360_e5000 / assign6360_e5003);
        (assign6360_e5004,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign6360_e5006;

        let (assign6370_e5016,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6370_e5010: f64 = (p.p347 * p.p237);
        let assign6370_e5013: f64 = (1e-6 * locals.var_iwe);
        let assign6370_e5014: f64 = (assign6370_e5010 / assign6370_e5013);
        (assign6370_e5014,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign6370_e5016;

        let (assign6380_e5020,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p348,)
    } else {
        (locals.var_bgidl_p,)
    }
};
        locals.var_bgidl_p = assign6380_e5020;

        let (assign6390_e5024,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p349,)
    } else {
        (locals.var_bgidld_p,)
    }
};
        locals.var_bgidld_p = assign6390_e5024;

        let (assign6400_e5028,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p350,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign6400_e5028;

        let (assign6410_e5032,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p351,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign6410_e5032;

        let (assign6420_e5036,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p352,)
    } else {
        (locals.var_cgidl_p,)
    }
};
        locals.var_cgidl_p = assign6420_e5036;

        let (assign6430_e5040,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p353,)
    } else {
        (locals.var_cgidld_p,)
    }
};
        locals.var_cgidld_p = assign6430_e5040;

        let (assign6440_e5052,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6440_e5044: f64 = (8.8541878176e-12 * p.p209);
        let assign6440_e5046: f64 = (assign6440_e5044 * locals.var_wecv);
        let assign6440_e5048: f64 = (assign6440_e5046 * locals.var_lecv);
        let assign6440_e5050: f64 = (assign6440_e5048 / p.p208);
        (assign6440_e5050,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign6440_e5052;

        let (assign6450_e5064,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6450_e5056: f64 = (8.8541878176e-12 * p.p209);
        let assign6450_e5058: f64 = (assign6450_e5056 * locals.var_wecv);
        let assign6450_e5060: f64 = (assign6450_e5058 * p.p236);
        let assign6450_e5062: f64 = (assign6450_e5060 / p.p234);
        (assign6450_e5062,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign6450_e5064;

        let (assign6460_e5076,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6460_e5068: f64 = (8.8541878176e-12 * p.p209);
        let assign6460_e5070: f64 = (assign6460_e5068 * locals.var_wecv);
        let assign6460_e5072: f64 = (assign6460_e5070 * p.p237);
        let assign6460_e5074: f64 = (assign6460_e5072 / p.p235);
        (assign6460_e5074,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign6460_e5076;

        let (assign6470_e5094,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6470_e5082: f64 = (locals.var_ile).powf(p.p356);
        let assign6470_e5083: f64 = (p.p355 * assign6470_e5082);
        let assign6470_e5084: f64 = (p.p354 + assign6470_e5083);
        let assign6470_e5087: f64 = (p.p357 * locals.var_iwe);
        let assign6470_e5088: f64 = (assign6470_e5084 + assign6470_e5087);
        let assign6470_e5091: f64 = (p.p358 * locals.var_iae);
        let assign6470_e5092: f64 = (assign6470_e5088 + assign6470_e5091);
        (assign6470_e5092,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign6470_e5094;

        let (assign6480_e5110,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6480_e5099: f64 = (p.p360 * locals.var_ile);
        let assign6480_e5100: f64 = (p.p359 + assign6480_e5099);
        let assign6480_e5103: f64 = (p.p361 * locals.var_iwe);
        let assign6480_e5104: f64 = (assign6480_e5100 + assign6480_e5103);
        let assign6480_e5107: f64 = (p.p362 * locals.var_iae);
        let assign6480_e5108: f64 = (assign6480_e5104 + assign6480_e5107);
        (assign6480_e5108,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign6480_e5110;

        let (assign6490_e5114,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p296,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6490_e5114;

        let assign6500_e5116: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6500_e5118: f64 = if assign6500_e5116 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign6500_e5118;

        let (assign6510_e5124,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard48 != 0.0)) {
        (p.p363,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6510_e5124;

        let (assign6520_e5128,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p297,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6520_e5128;

        let assign6530_e5130: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6530_e5132: f64 = if assign6530_e5130 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign6530_e5132;

        let (assign6540_e5138,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard49 != 0.0)) {
        (p.p364,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6540_e5138;

        let (assign6550_e5142,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p298,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6550_e5142;

        let assign6560_e5144: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6560_e5146: f64 = if assign6560_e5144 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign6560_e5146;

        let (assign6570_e5152,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard50 != 0.0)) {
        (p.p365,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6570_e5152;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6580_e5156,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p299,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6580_e5156;

        let assign6590_e5158: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6590_e5160: f64 = if assign6590_e5158 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign6590_e5160;

        let (assign6600_e5166,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard51 != 0.0)) {
        (p.p366,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6600_e5166;

        let (assign6610_e5170,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p300,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6610_e5170;

        let assign6620_e5172: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6620_e5174: f64 = if assign6620_e5172 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard52 = assign6620_e5174;

        let (assign6630_e5180,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard52 != 0.0)) {
        (p.p367,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6630_e5180;

        let (assign6640_e5206,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6640_e5185: f64 = (locals.var_thesatacl_i * locals.var_gwe);
        let assign6640_e5187: f64 = (assign6640_e5185 / locals.var_gpe);
        let assign6640_e5190: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
        let assign6640_e5191: f64 = (assign6640_e5187 * assign6640_e5190);
        let assign6640_e5192: f64 = (locals.var_thesataco_i + assign6640_e5191);
        let assign6640_e5196: f64 = (locals.var_thesatacw_i * locals.var_iwe);
        let assign6640_e5197: f64 = (1.0 + assign6640_e5196);
        let assign6640_e5198: f64 = (assign6640_e5192 * assign6640_e5197);
        let assign6640_e5202: f64 = (locals.var_thesataclw_i * locals.var_iae);
        let assign6640_e5203: f64 = (1.0 + assign6640_e5202);
        let assign6640_e5204: f64 = (assign6640_e5198 * assign6640_e5203);
        (assign6640_e5204,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign6640_e5206;

        let (assign6650_e5210,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p308,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6650_e5210;

        let assign6660_e5212: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6660_e5214: f64 = if assign6660_e5212 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign6660_e5214;

        let (assign6670_e5220,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard53 != 0.0)) {
        (p.p368,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6670_e5220;

        let (assign6680_e5224,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p309,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6680_e5224;

        let assign6690_e5226: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6690_e5228: f64 = if assign6690_e5226 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign6690_e5228;

        let (assign6700_e5234,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard54 != 0.0)) {
        (p.p369,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6700_e5234;

        let (assign6710_e5244,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6710_e5240: f64 = (locals.var_axacl_i * locals.var_ile);
        let assign6710_e5241: f64 = (1.0 + assign6710_e5240);
        let assign6710_e5242: f64 = (locals.var_axaco_i / assign6710_e5241);
        (assign6710_e5242,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign6710_e5244;

        let (assign6720_e5258,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6720_e5249: f64 = (locals.var_ile).powf(p.p371);
        let assign6720_e5250: f64 = (p.p370 * assign6720_e5249);
        let assign6720_e5254: f64 = (p.p372 * locals.var_iwe);
        let assign6720_e5255: f64 = (1.0 + assign6720_e5254);
        let assign6720_e5256: f64 = (assign6720_e5250 * assign6720_e5255);
        (assign6720_e5256,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign6720_e5258;

        let (assign6730_e5264,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6730_e5262: f64 = (locals.var_ile).powf(p.p374);
        (assign6730_e5262,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6730_e5264;

        let (assign6740_e5284,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6740_e5268: f64 = (p.p373 * locals.var_tmpx);
        let assign6740_e5272: f64 = (p.p376 * locals.var_iwe);
        let assign6740_e5273: f64 = (1.0 + assign6740_e5272);
        let assign6740_e5274: f64 = (assign6740_e5268 * assign6740_e5273);
        let assign6740_e5278: f64 = (p.p375 * locals.var_ile);
        let assign6740_e5280: f64 = (assign6740_e5278 * locals.var_tmpx);
        let assign6740_e5281: f64 = (1.0 + assign6740_e5280);
        let assign6740_e5282: f64 = (assign6740_e5274 / assign6740_e5281);
        (assign6740_e5282,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign6740_e5284;

        let (assign6750_e5288,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p377,)
    } else {
        (locals.var_fcgovacc_p,)
    }
};
        locals.var_fcgovacc_p = assign6750_e5288;

        let (assign6760_e5292,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p378,)
    } else {
        (locals.var_fcgovaccd_p,)
    }
};
        locals.var_fcgovaccd_p = assign6760_e5292;

        let (assign6770_e5296,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p379,)
    } else {
        (locals.var_cgovaccg_p,)
    }
};
        locals.var_cgovaccg_p = assign6770_e5296;

        let (assign6780_e5302,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6780_e5300: f64 = (p.p380 * locals.var_iilcv);
        (assign6780_e5300,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign6780_e5302;

        let (assign6790_e5308,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6790_e5306: f64 = (p.p381 * locals.var_iiwecv);
        (assign6790_e5306,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign6790_e5308;

        let (assign6800_e5314,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6800_e5312: f64 = (p.p382 * locals.var_iiwecv);
        (assign6800_e5312,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign6800_e5314;

        let (assign6810_e5318,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p383,)
    } else {
        (locals.var_dvfbinr_p,)
    }
};
        locals.var_dvfbinr_p = assign6810_e5318;

        let (assign6820_e5322,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p384,)
    } else {
        (locals.var_fcinrdep_p,)
    }
};
        locals.var_fcinrdep_p = assign6820_e5322;

        let (assign6830_e5326,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p385,)
    } else {
        (locals.var_fcinracc_p,)
    }
};
        locals.var_fcinracc_p = assign6830_e5326;

        let (assign6840_e5330,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p386,)
    } else {
        (locals.var_axinr_p,)
    }
};
        locals.var_axinr_p = assign6840_e5330;

        let (assign6870_e5352,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6870_e5347: f64 = (2.0 * p.p395);
        let assign6870_e5349: f64 = (assign6870_e5347 / locals.var_le);
        let assign6870_e5350: f64 = (1.0 - assign6870_e5349);
        (assign6870_e5350,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign6870_e5352;

        let (assign6900_e5373,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p389,)
    } else {
        (locals.var_fnt_p,)
    }
};
        locals.var_fnt_p = assign6900_e5373;

        let (assign6910_e5385,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6910_e5377: f64 = (p.p390 * locals.var_betn_p);
        let assign6910_e5379: f64 = (assign6910_e5377 * locals.var_betn_p);
        let assign6910_e5381: f64 = (assign6910_e5379 * locals.var_iwe);
        let assign6910_e5383: f64 = (assign6910_e5381 * locals.var_iwe);
        (assign6910_e5383,)
    } else {
        (locals.var_fntexc_p,)
    }
};
        locals.var_fntexc_p = assign6910_e5385;

        let (assign6960_e5423,) = {
    if (locals.var_guard41 != 0.0) {
        let assign6960_e5417: f64 = (2.0 * p.p397);
        let assign6960_e5420: f64 = (p.p398 * locals.var_we);
        let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
        (assign6960_e5421,)
    } else {
        (locals.var_we_edge,)
    }
};
        locals.var_we_edge = assign6960_e5423;

        let (assign6990_e5439,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p399,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign6990_e5439;

        let (assign7000_e5455,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7000_e5444: f64 = (p.p401 * locals.var_ile);
        let assign7000_e5445: f64 = (p.p400 + assign7000_e5444);
        let assign7000_e5448: f64 = (p.p402 * locals.var_iwe);
        let assign7000_e5449: f64 = (assign7000_e5445 + assign7000_e5448);
        let assign7000_e5452: f64 = (p.p403 * locals.var_iae);
        let assign7000_e5453: f64 = (assign7000_e5449 + assign7000_e5452);
        (assign7000_e5453,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign7000_e5455;

        let (assign7010_e5473,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7010_e5461: f64 = (locals.var_ile).powf(p.p406);
        let assign7010_e5462: f64 = (p.p405 * assign7010_e5461);
        let assign7010_e5463: f64 = (p.p404 + assign7010_e5462);
        let assign7010_e5466: f64 = (p.p407 * locals.var_iwe);
        let assign7010_e5467: f64 = (assign7010_e5463 + assign7010_e5466);
        let assign7010_e5470: f64 = (p.p408 * locals.var_iae);
        let assign7010_e5471: f64 = (assign7010_e5467 + assign7010_e5470);
        (assign7010_e5471,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign7010_e5473;

        let (assign7020_e5497,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7020_e5480: f64 = (locals.var_ile).powf(p.p411);
        let assign7020_e5481: f64 = (p.p410 * assign7020_e5480);
        let assign7020_e5482: f64 = (1.0 + assign7020_e5481);
        let assign7020_e5483: f64 = (p.p409 * assign7020_e5482);
        let assign7020_e5487: f64 = (p.p412 * locals.var_iwe);
        let assign7020_e5488: f64 = (1.0 + assign7020_e5487);
        let assign7020_e5489: f64 = (assign7020_e5483 * assign7020_e5488);
        let assign7020_e5493: f64 = (p.p413 * locals.var_iae);
        let assign7020_e5494: f64 = (1.0 + assign7020_e5493);
        let assign7020_e5495: f64 = (assign7020_e5489 * assign7020_e5494);
        (assign7020_e5495,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign7020_e5497;

        let (assign7030_e5507,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7030_e5503: f64 = (locals.var_ile).powf(p.p416);
        let assign7030_e5504: f64 = (p.p415 * assign7030_e5503);
        let assign7030_e5505: f64 = (p.p414 + assign7030_e5504);
        (assign7030_e5505,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign7030_e5507;

        let (assign7040_e5525,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7040_e5512: f64 = (p.p417 * p.p418);
        let assign7040_e5514: f64 = (assign7040_e5512 / locals.var_le);
        let assign7040_e5517: f64 = (-locals.var_le);
        let assign7040_e5519: f64 = (assign7040_e5517 / p.p418);
        let assign7040_e5520: f64 = (assign7040_e5519).exp();
        let assign7040_e5521: f64 = (1.0 - assign7040_e5520);
        let assign7040_e5522: f64 = (assign7040_e5514 * assign7040_e5521);
        let assign7040_e5523: f64 = (1.0 + assign7040_e5522);
        (assign7040_e5523,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign7040_e5525;

        let (assign7050_e5534,) = {
    if (locals.var_guard41 != 0.0) {
        let (assign7050_e5532,) = {
            if (locals.var_gpe_edge > 1e-15) {
                (locals.var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign7050_e5532,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign7050_e5534;

        let (assign7060_e5550,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7060_e5538: f64 = (p.p258 * locals.var_we_edge);
        let assign7060_e5541: f64 = (locals.var_gpe_edge * locals.var_le);
        let assign7060_e5542: f64 = (assign7060_e5538 / assign7060_e5541);
        let assign7060_e5546: f64 = (p.p419 * locals.var_iwe);
        let assign7060_e5547: f64 = (1.0 + assign7060_e5546);
        let assign7060_e5548: f64 = (assign7060_e5542 * assign7060_e5547);
        (assign7060_e5548,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign7060_e5550;

        let (assign7070_e5566,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7070_e5555: f64 = (p.p421 * locals.var_ile);
        let assign7070_e5556: f64 = (p.p420 + assign7070_e5555);
        let assign7070_e5559: f64 = (p.p422 * locals.var_iwe);
        let assign7070_e5560: f64 = (assign7070_e5556 + assign7070_e5559);
        let assign7070_e5563: f64 = (p.p423 * locals.var_iae);
        let assign7070_e5564: f64 = (assign7070_e5560 + assign7070_e5563);
        (assign7070_e5564,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign7070_e5566;

        let (assign7080_e5580,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7080_e5571: f64 = (locals.var_ile).powf(p.p425);
        let assign7080_e5572: f64 = (p.p424 * assign7080_e5571);
        let assign7080_e5576: f64 = (p.p426 * locals.var_iwe);
        let assign7080_e5577: f64 = (1.0 + assign7080_e5576);
        let assign7080_e5578: f64 = (assign7080_e5572 * assign7080_e5577);
        (assign7080_e5578,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign7080_e5580;

        let (assign7090_e5584,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p427,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign7090_e5584;

        let (assign7100_e5588,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p428,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign7100_e5588;

        let (assign7110_e5602,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7110_e5593: f64 = (locals.var_ile).powf(p.p430);
        let assign7110_e5594: f64 = (p.p429 * assign7110_e5593);
        let assign7110_e5598: f64 = (p.p431 * locals.var_iwe);
        let assign7110_e5599: f64 = (1.0 + assign7110_e5598);
        let assign7110_e5600: f64 = (assign7110_e5594 * assign7110_e5599);
        (assign7110_e5600,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign7110_e5602;

        let (assign7120_e5606,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p433,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign7120_e5606;

        let (assign7130_e5610,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p432,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign7130_e5610;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7190_e5652,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7190_e5641: f64 = (p.p815 * locals.var_ile);
        let assign7190_e5642: f64 = (p.p814 + assign7190_e5641);
        let assign7190_e5645: f64 = (p.p816 * locals.var_iwe);
        let assign7190_e5646: f64 = (assign7190_e5642 + assign7190_e5645);
        let assign7190_e5649: f64 = (p.p817 * locals.var_iae);
        let assign7190_e5650: f64 = (assign7190_e5646 + assign7190_e5649);
        (assign7190_e5650,)
    } else {
        (locals.var_kvthowe,)
    }
};
        locals.var_kvthowe = assign7190_e5652;

        let (assign7200_e5668,) = {
    if (locals.var_guard41 != 0.0) {
        let assign7200_e5657: f64 = (p.p819 * locals.var_ile);
        let assign7200_e5658: f64 = (p.p818 + assign7200_e5657);
        let assign7200_e5661: f64 = (p.p820 * locals.var_iwe);
        let assign7200_e5662: f64 = (assign7200_e5658 + assign7200_e5661);
        let assign7200_e5665: f64 = (p.p821 * locals.var_iae);
        let assign7200_e5666: f64 = (assign7200_e5662 + assign7200_e5665);
        (assign7200_e5666,)
    } else {
        (locals.var_kuowe,)
    }
};
        locals.var_kuowe = assign7200_e5668;

        let (assign7320_e5767,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p450,)
    } else {
        (locals.var_munqs_p,)
    }
};
        locals.var_munqs_p = assign7320_e5767;

        let assign7330_e5786: f64 = if (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]) { 1.0 } else { 0.0 };
        locals.var_guard56 = assign7330_e5786;

        let (assign7340_e5804,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign7340_e5793: f64 = (p.p452 * locals.var_ile);
        let assign7340_e5794: f64 = (p.p451 + assign7340_e5793);
        let assign7340_e5797: f64 = (p.p453 * locals.var_iwe);
        let assign7340_e5798: f64 = (assign7340_e5794 + assign7340_e5797);
        let assign7340_e5801: f64 = (p.p454 * locals.var_iae);
        let assign7340_e5802: f64 = (assign7340_e5798 + assign7340_e5801);
        (assign7340_e5802,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign7340_e5804;

        let assign7350_e5823: f64 = if (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign7350_e5823;

        let (assign7360_e5841,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign7360_e5830: f64 = (p.p456 * locals.var_ile);
        let assign7360_e5831: f64 = (p.p455 + assign7360_e5830);
        let assign7360_e5834: f64 = (p.p457 * locals.var_iwe);
        let assign7360_e5835: f64 = (assign7360_e5831 + assign7360_e5834);
        let assign7360_e5838: f64 = (p.p458 * locals.var_iae);
        let assign7360_e5839: f64 = (assign7360_e5835 + assign7360_e5838);
        (assign7360_e5839,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign7360_e5841;

        let assign7370_e5860: f64 = if (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign7370_e5860;

        let (assign7380_e5878,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard58 != 0.0)) {
        let assign7380_e5867: f64 = (p.p460 * locals.var_ile);
        let assign7380_e5868: f64 = (p.p459 + assign7380_e5867);
        let assign7380_e5871: f64 = (p.p461 * locals.var_iwe);
        let assign7380_e5872: f64 = (assign7380_e5868 + assign7380_e5871);
        let assign7380_e5875: f64 = (p.p462 * locals.var_iae);
        let assign7380_e5876: f64 = (assign7380_e5872 + assign7380_e5875);
        (assign7380_e5876,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign7380_e5878;

        let assign7390_e5897: f64 = if (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign7390_e5897;

        let (assign7400_e5915,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard59 != 0.0)) {
        let assign7400_e5904: f64 = (p.p464 * locals.var_ile);
        let assign7400_e5905: f64 = (p.p463 + assign7400_e5904);
        let assign7400_e5908: f64 = (p.p465 * locals.var_iwe);
        let assign7400_e5909: f64 = (assign7400_e5905 + assign7400_e5908);
        let assign7400_e5912: f64 = (p.p466 * locals.var_iae);
        let assign7400_e5913: f64 = (assign7400_e5909 + assign7400_e5912);
        (assign7400_e5913,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign7400_e5915;

        let assign7410_e5934: f64 = if (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]) { 1.0 } else { 0.0 };
        locals.var_guard60 = assign7410_e5934;

        let (assign7420_e5952,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign7420_e5941: f64 = (p.p468 * locals.var_ile);
        let assign7420_e5942: f64 = (p.p467 + assign7420_e5941);
        let assign7420_e5945: f64 = (p.p469 * locals.var_iwe);
        let assign7420_e5946: f64 = (assign7420_e5942 + assign7420_e5945);
        let assign7420_e5949: f64 = (p.p470 * locals.var_iae);
        let assign7420_e5950: f64 = (assign7420_e5946 + assign7420_e5949);
        (assign7420_e5950,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign7420_e5952;

        let assign7430_e5971: f64 = if (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]) { 1.0 } else { 0.0 };
        locals.var_guard61 = assign7430_e5971;

        let (assign7440_e5989,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard61 != 0.0)) {
        let assign7440_e5978: f64 = (p.p472 * locals.var_ile);
        let assign7440_e5979: f64 = (p.p471 + assign7440_e5978);
        let assign7440_e5982: f64 = (p.p473 * locals.var_iwe);
        let assign7440_e5983: f64 = (assign7440_e5979 + assign7440_e5982);
        let assign7440_e5986: f64 = (p.p474 * locals.var_iae);
        let assign7440_e5987: f64 = (assign7440_e5983 + assign7440_e5986);
        (assign7440_e5987,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign7440_e5989;

        let assign7450_e6008: f64 = if (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]) { 1.0 } else { 0.0 };
        locals.var_guard62 = assign7450_e6008;

        let (assign7460_e6026,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign7460_e6015: f64 = (p.p476 * locals.var_ile);
        let assign7460_e6016: f64 = (p.p475 + assign7460_e6015);
        let assign7460_e6019: f64 = (p.p477 * locals.var_iwe);
        let assign7460_e6020: f64 = (assign7460_e6016 + assign7460_e6019);
        let assign7460_e6023: f64 = (p.p478 * locals.var_iae);
        let assign7460_e6024: f64 = (assign7460_e6020 + assign7460_e6023);
        (assign7460_e6024,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign7460_e6026;

        let assign7470_e6045: f64 = if (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]) { 1.0 } else { 0.0 };
        locals.var_guard63 = assign7470_e6045;

        let (assign7480_e6063,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard63 != 0.0)) {
        let assign7480_e6052: f64 = (p.p480 * locals.var_ile);
        let assign7480_e6053: f64 = (p.p479 + assign7480_e6052);
        let assign7480_e6056: f64 = (p.p481 * locals.var_iwe);
        let assign7480_e6057: f64 = (assign7480_e6053 + assign7480_e6056);
        let assign7480_e6060: f64 = (p.p482 * locals.var_iae);
        let assign7480_e6061: f64 = (assign7480_e6057 + assign7480_e6060);
        (assign7480_e6061,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign7480_e6063;

        let assign7490_e6082: f64 = if (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]) { 1.0 } else { 0.0 };
        locals.var_guard64 = assign7490_e6082;

        let (assign7500_e6100,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard64 != 0.0)) {
        let assign7500_e6089: f64 = (p.p484 * locals.var_ile);
        let assign7500_e6090: f64 = (p.p483 + assign7500_e6089);
        let assign7500_e6093: f64 = (p.p485 * locals.var_iwe);
        let assign7500_e6094: f64 = (assign7500_e6090 + assign7500_e6093);
        let assign7500_e6097: f64 = (p.p486 * locals.var_iae);
        let assign7500_e6098: f64 = (assign7500_e6094 + assign7500_e6097);
        (assign7500_e6098,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign7500_e6100;

        let assign7510_e6119: f64 = if (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]) { 1.0 } else { 0.0 };
        locals.var_guard65 = assign7510_e6119;

        let (assign7520_e6137,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard65 != 0.0)) {
        let assign7520_e6126: f64 = (p.p488 * locals.var_ile);
        let assign7520_e6127: f64 = (p.p487 + assign7520_e6126);
        let assign7520_e6130: f64 = (p.p489 * locals.var_iwe);
        let assign7520_e6131: f64 = (assign7520_e6127 + assign7520_e6130);
        let assign7520_e6134: f64 = (p.p490 * locals.var_iae);
        let assign7520_e6135: f64 = (assign7520_e6131 + assign7520_e6134);
        (assign7520_e6135,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign7520_e6137;

        let assign7530_e6156: f64 = if (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]) { 1.0 } else { 0.0 };
        locals.var_guard66 = assign7530_e6156;

        let (assign7540_e6174,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard66 != 0.0)) {
        let assign7540_e6163: f64 = (p.p496 * locals.var_ile);
        let assign7540_e6164: f64 = (p.p495 + assign7540_e6163);
        let assign7540_e6167: f64 = (p.p497 * locals.var_iwe);
        let assign7540_e6168: f64 = (assign7540_e6164 + assign7540_e6167);
        let assign7540_e6171: f64 = (p.p498 * locals.var_iae);
        let assign7540_e6172: f64 = (assign7540_e6168 + assign7540_e6171);
        (assign7540_e6172,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign7540_e6174;

        let assign7550_e6193: f64 = if (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7550_e6193;

        let (assign7560_e6211,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard67 != 0.0)) {
        let assign7560_e6200: f64 = (p.p492 * locals.var_ile);
        let assign7560_e6201: f64 = (p.p491 + assign7560_e6200);
        let assign7560_e6204: f64 = (p.p493 * locals.var_iwe);
        let assign7560_e6205: f64 = (assign7560_e6201 + assign7560_e6204);
        let assign7560_e6208: f64 = (p.p494 * locals.var_iae);
        let assign7560_e6209: f64 = (assign7560_e6205 + assign7560_e6208);
        (assign7560_e6209,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign7560_e6211;

        let assign7570_e6230: f64 = if (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7570_e6230;

        let (assign7580_e6248,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard68 != 0.0)) {
        let assign7580_e6237: f64 = (p.p500 * locals.var_ile);
        let assign7580_e6238: f64 = (p.p499 + assign7580_e6237);
        let assign7580_e6241: f64 = (p.p501 * locals.var_iwe);
        let assign7580_e6242: f64 = (assign7580_e6238 + assign7580_e6241);
        let assign7580_e6245: f64 = (p.p502 * locals.var_iae);
        let assign7580_e6246: f64 = (assign7580_e6242 + assign7580_e6245);
        (assign7580_e6246,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign7580_e6248;

        let assign7590_e6267: f64 = if (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7590_e6267;

        let (assign7600_e6287,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign7600_e6275: f64 = (p.p504 * locals.var_ile);
        let assign7600_e6276: f64 = (p.p503 + assign7600_e6275);
        let assign7600_e6279: f64 = (p.p505 * locals.var_iwe);
        let assign7600_e6280: f64 = (assign7600_e6276 + assign7600_e6279);
        let assign7600_e6283: f64 = (p.p506 * locals.var_iae);
        let assign7600_e6284: f64 = (assign7600_e6280 + assign7600_e6283);
        let assign7600_e6285: f64 = (locals.var_ile2 * assign7600_e6284);
        (assign7600_e6285,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign7600_e6287;

        let assign7610_e6306: f64 = if (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7610_e6306;

        let (assign7620_e6324,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard70 != 0.0)) {
        let assign7620_e6313: f64 = (p.p512 * locals.var_ile);
        let assign7620_e6314: f64 = (p.p511 + assign7620_e6313);
        let assign7620_e6317: f64 = (p.p513 * locals.var_iwe);
        let assign7620_e6318: f64 = (assign7620_e6314 + assign7620_e6317);
        let assign7620_e6321: f64 = (p.p514 * locals.var_iae);
        let assign7620_e6322: f64 = (assign7620_e6318 + assign7620_e6321);
        (assign7620_e6322,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign7620_e6324;

        let assign7630_e6343: f64 = if (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7630_e6343;

        let (assign7640_e6361,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard71 != 0.0)) {
        let assign7640_e6350: f64 = (p.p508 * locals.var_ile);
        let assign7640_e6351: f64 = (p.p507 + assign7640_e6350);
        let assign7640_e6354: f64 = (p.p509 * locals.var_iwe);
        let assign7640_e6355: f64 = (assign7640_e6351 + assign7640_e6354);
        let assign7640_e6358: f64 = (p.p510 * locals.var_iae);
        let assign7640_e6359: f64 = (assign7640_e6355 + assign7640_e6358);
        (assign7640_e6359,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign7640_e6361;

        let assign7650_e6380: f64 = if (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7650_e6380;

        let (assign7660_e6400,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard72 != 0.0)) {
        let assign7660_e6388: f64 = (p.p516 * locals.var_ile);
        let assign7660_e6389: f64 = (p.p515 + assign7660_e6388);
        let assign7660_e6392: f64 = (p.p517 * locals.var_iwe);
        let assign7660_e6393: f64 = (assign7660_e6389 + assign7660_e6392);
        let assign7660_e6396: f64 = (p.p518 * locals.var_iae);
        let assign7660_e6397: f64 = (assign7660_e6393 + assign7660_e6396);
        let assign7660_e6398: f64 = (locals.var_ile2 * assign7660_e6397);
        (assign7660_e6398,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign7660_e6400;

        let assign7670_e6419: f64 = if (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7670_e6419;

        let (assign7680_e6437,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign7680_e6426: f64 = (p.p524 * locals.var_ile);
        let assign7680_e6427: f64 = (p.p523 + assign7680_e6426);
        let assign7680_e6430: f64 = (p.p525 * locals.var_iwe);
        let assign7680_e6431: f64 = (assign7680_e6427 + assign7680_e6430);
        let assign7680_e6434: f64 = (p.p526 * locals.var_iae);
        let assign7680_e6435: f64 = (assign7680_e6431 + assign7680_e6434);
        (assign7680_e6435,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign7680_e6437;

        let assign7690_e6456: f64 = if (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7690_e6456;

        let (assign7700_e6474,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard74 != 0.0)) {
        let assign7700_e6463: f64 = (p.p520 * locals.var_ile);
        let assign7700_e6464: f64 = (p.p519 + assign7700_e6463);
        let assign7700_e6467: f64 = (p.p521 * locals.var_iwe);
        let assign7700_e6468: f64 = (assign7700_e6464 + assign7700_e6467);
        let assign7700_e6471: f64 = (p.p522 * locals.var_iae);
        let assign7700_e6472: f64 = (assign7700_e6468 + assign7700_e6471);
        (assign7700_e6472,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign7700_e6474;

        let assign7710_e6493: f64 = if (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7710_e6493;

        let (assign7720_e6515,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign7720_e6499: f64 = (locals.var_we / locals.var_le);
        let assign7720_e6503: f64 = (p.p528 * locals.var_ile);
        let assign7720_e6504: f64 = (p.p527 + assign7720_e6503);
        let assign7720_e6507: f64 = (p.p529 * locals.var_iwe);
        let assign7720_e6508: f64 = (assign7720_e6504 + assign7720_e6507);
        let assign7720_e6511: f64 = (p.p530 * locals.var_iae);
        let assign7720_e6512: f64 = (assign7720_e6508 + assign7720_e6511);
        let assign7720_e6513: f64 = (assign7720_e6499 * assign7720_e6512);
        (assign7720_e6513,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign7720_e6515;

        let assign7730_e6534: f64 = if (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7730_e6534;

        let (assign7740_e6552,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign7740_e6541: f64 = (p.p532 * locals.var_ile);
        let assign7740_e6542: f64 = (p.p531 + assign7740_e6541);
        let assign7740_e6545: f64 = (p.p533 * locals.var_iwe);
        let assign7740_e6546: f64 = (assign7740_e6542 + assign7740_e6545);
        let assign7740_e6549: f64 = (p.p534 * locals.var_iae);
        let assign7740_e6550: f64 = (assign7740_e6546 + assign7740_e6549);
        (assign7740_e6550,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign7740_e6552;

        let assign7750_e6571: f64 = if (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign7750_e6571;

        let (assign7760_e6589,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard77 != 0.0)) {
        let assign7760_e6578: f64 = (p.p536 * locals.var_ile);
        let assign7760_e6579: f64 = (p.p535 + assign7760_e6578);
        let assign7760_e6582: f64 = (p.p537 * locals.var_iwe);
        let assign7760_e6583: f64 = (assign7760_e6579 + assign7760_e6582);
        let assign7760_e6586: f64 = (p.p538 * locals.var_iae);
        let assign7760_e6587: f64 = (assign7760_e6583 + assign7760_e6586);
        (assign7760_e6587,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign7760_e6589;

        let assign7770_e6608: f64 = if (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign7770_e6608;

        let (assign7780_e6626,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign7780_e6615: f64 = (p.p540 * locals.var_ile);
        let assign7780_e6616: f64 = (p.p539 + assign7780_e6615);
        let assign7780_e6619: f64 = (p.p541 * locals.var_iwe);
        let assign7780_e6620: f64 = (assign7780_e6616 + assign7780_e6619);
        let assign7780_e6623: f64 = (p.p542 * locals.var_iae);
        let assign7780_e6624: f64 = (assign7780_e6620 + assign7780_e6623);
        (assign7780_e6624,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign7780_e6626;

        let assign7790_e6645: f64 = if (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign7790_e6645;

        let (assign7800_e6663,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard79 != 0.0)) {
        let assign7800_e6652: f64 = (p.p544 * locals.var_ile);
        let assign7800_e6653: f64 = (p.p543 + assign7800_e6652);
        let assign7800_e6656: f64 = (p.p545 * locals.var_iwe);
        let assign7800_e6657: f64 = (assign7800_e6653 + assign7800_e6656);
        let assign7800_e6660: f64 = (p.p546 * locals.var_iae);
        let assign7800_e6661: f64 = (assign7800_e6657 + assign7800_e6660);
        (assign7800_e6661,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign7800_e6663;

        let assign7810_e6682: f64 = if (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]) { 1.0 } else { 0.0 };
        locals.var_guard80 = assign7810_e6682;

        let (assign7820_e6700,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard80 != 0.0)) {
        let assign7820_e6689: f64 = (p.p548 * locals.var_ile);
        let assign7820_e6690: f64 = (p.p547 + assign7820_e6689);
        let assign7820_e6693: f64 = (p.p549 * locals.var_iwe);
        let assign7820_e6694: f64 = (assign7820_e6690 + assign7820_e6693);
        let assign7820_e6697: f64 = (p.p550 * locals.var_iae);
        let assign7820_e6698: f64 = (assign7820_e6694 + assign7820_e6697);
        (assign7820_e6698,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign7820_e6700;

        let assign7830_e6719: f64 = if (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]) { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7830_e6719;

        let (assign7840_e6737,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard81 != 0.0)) {
        let assign7840_e6726: f64 = (p.p552 * locals.var_ile);
        let assign7840_e6727: f64 = (p.p551 + assign7840_e6726);
        let assign7840_e6730: f64 = (p.p553 * locals.var_iwe);
        let assign7840_e6731: f64 = (assign7840_e6727 + assign7840_e6730);
        let assign7840_e6734: f64 = (p.p554 * locals.var_iae);
        let assign7840_e6735: f64 = (assign7840_e6731 + assign7840_e6734);
        (assign7840_e6735,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign7840_e6737;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign7850_e6756: f64 = if (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7850_e6756;

        let (assign7860_e6776,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard82 != 0.0)) {
        let assign7860_e6764: f64 = (p.p556 * locals.var_ile);
        let assign7860_e6765: f64 = (p.p555 + assign7860_e6764);
        let assign7860_e6768: f64 = (p.p557 * locals.var_iwe);
        let assign7860_e6769: f64 = (assign7860_e6765 + assign7860_e6768);
        let assign7860_e6772: f64 = (p.p558 * locals.var_iae);
        let assign7860_e6773: f64 = (assign7860_e6769 + assign7860_e6772);
        let assign7860_e6774: f64 = (locals.var_iwe * assign7860_e6773);
        (assign7860_e6774,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign7860_e6776;

        let assign7870_e6795: f64 = if (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7870_e6795;

        let (assign7880_e6813,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard83 != 0.0)) {
        let assign7880_e6802: f64 = (p.p560 * locals.var_ile);
        let assign7880_e6803: f64 = (p.p559 + assign7880_e6802);
        let assign7880_e6806: f64 = (p.p561 * locals.var_iwe);
        let assign7880_e6807: f64 = (assign7880_e6803 + assign7880_e6806);
        let assign7880_e6810: f64 = (p.p562 * locals.var_iae);
        let assign7880_e6811: f64 = (assign7880_e6807 + assign7880_e6810);
        (assign7880_e6811,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign7880_e6813;

        let assign7890_e6832: f64 = if (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]) { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7890_e6832;

        let (assign7900_e6850,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign7900_e6839: f64 = (p.p564 * locals.var_ile);
        let assign7900_e6840: f64 = (p.p563 + assign7900_e6839);
        let assign7900_e6843: f64 = (p.p565 * locals.var_iwe);
        let assign7900_e6844: f64 = (assign7900_e6840 + assign7900_e6843);
        let assign7900_e6847: f64 = (p.p566 * locals.var_iae);
        let assign7900_e6848: f64 = (assign7900_e6844 + assign7900_e6847);
        (assign7900_e6848,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign7900_e6850;

        let assign7910_e6869: f64 = if (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]) { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7910_e6869;

        let (assign7920_e6887,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign7920_e6876: f64 = (p.p568 * locals.var_ile);
        let assign7920_e6877: f64 = (p.p567 + assign7920_e6876);
        let assign7920_e6880: f64 = (p.p569 * locals.var_iwe);
        let assign7920_e6881: f64 = (assign7920_e6877 + assign7920_e6880);
        let assign7920_e6884: f64 = (p.p570 * locals.var_iae);
        let assign7920_e6885: f64 = (assign7920_e6881 + assign7920_e6884);
        (assign7920_e6885,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign7920_e6887;

        let assign7930_e6906: f64 = if (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7930_e6906;

        let (assign7940_e6926,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign7940_e6914: f64 = (p.p572 * locals.var_ile);
        let assign7940_e6915: f64 = (p.p571 + assign7940_e6914);
        let assign7940_e6918: f64 = (p.p573 * locals.var_iwe);
        let assign7940_e6919: f64 = (assign7940_e6915 + assign7940_e6918);
        let assign7940_e6922: f64 = (p.p574 * locals.var_iae);
        let assign7940_e6923: f64 = (assign7940_e6919 + assign7940_e6922);
        let assign7940_e6924: f64 = (locals.var_ile * assign7940_e6923);
        (assign7940_e6924,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign7940_e6926;

        let assign7950_e6945: f64 = if (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]) { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7950_e6945;

        let (assign7960_e6963,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7960_e6952: f64 = (p.p576 * locals.var_ile);
        let assign7960_e6953: f64 = (p.p575 + assign7960_e6952);
        let assign7960_e6956: f64 = (p.p577 * locals.var_iwe);
        let assign7960_e6957: f64 = (assign7960_e6953 + assign7960_e6956);
        let assign7960_e6960: f64 = (p.p578 * locals.var_iae);
        let assign7960_e6961: f64 = (assign7960_e6957 + assign7960_e6960);
        (assign7960_e6961,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign7960_e6963;

        let assign7970_e6982: f64 = if (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]) { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7970_e6982;

        let (assign7980_e7000,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard88 != 0.0)) {
        let assign7980_e6989: f64 = (p.p580 * locals.var_ile);
        let assign7980_e6990: f64 = (p.p579 + assign7980_e6989);
        let assign7980_e6993: f64 = (p.p581 * locals.var_iwe);
        let assign7980_e6994: f64 = (assign7980_e6990 + assign7980_e6993);
        let assign7980_e6997: f64 = (p.p582 * locals.var_iae);
        let assign7980_e6998: f64 = (assign7980_e6994 + assign7980_e6997);
        (assign7980_e6998,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign7980_e7000;

        let assign7990_e7019: f64 = if (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7990_e7019;

        let (assign8000_e7037,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard89 != 0.0)) {
        let assign8000_e7026: f64 = (p.p584 * locals.var_ile);
        let assign8000_e7027: f64 = (p.p583 + assign8000_e7026);
        let assign8000_e7030: f64 = (p.p585 * locals.var_iwe);
        let assign8000_e7031: f64 = (assign8000_e7027 + assign8000_e7030);
        let assign8000_e7034: f64 = (p.p586 * locals.var_iae);
        let assign8000_e7035: f64 = (assign8000_e7031 + assign8000_e7034);
        (assign8000_e7035,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign8000_e7037;

        let assign8010_e7056: f64 = if (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        locals.var_guard90 = assign8010_e7056;

        let (assign8020_e7074,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign8020_e7063: f64 = (p.p588 * locals.var_ile);
        let assign8020_e7064: f64 = (p.p587 + assign8020_e7063);
        let assign8020_e7067: f64 = (p.p589 * locals.var_iwe);
        let assign8020_e7068: f64 = (assign8020_e7064 + assign8020_e7067);
        let assign8020_e7071: f64 = (p.p590 * locals.var_iae);
        let assign8020_e7072: f64 = (assign8020_e7068 + assign8020_e7071);
        (assign8020_e7072,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign8020_e7074;

        let assign8030_e7093: f64 = if (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]) { 1.0 } else { 0.0 };
        locals.var_guard91 = assign8030_e7093;

        let (assign8040_e7113,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard91 != 0.0)) {
        let assign8040_e7101: f64 = (p.p592 * locals.var_ile);
        let assign8040_e7102: f64 = (p.p591 + assign8040_e7101);
        let assign8040_e7105: f64 = (p.p593 * locals.var_iwe);
        let assign8040_e7106: f64 = (assign8040_e7102 + assign8040_e7105);
        let assign8040_e7109: f64 = (p.p594 * locals.var_iae);
        let assign8040_e7110: f64 = (assign8040_e7106 + assign8040_e7109);
        let assign8040_e7111: f64 = (locals.var_ile * assign8040_e7110);
        (assign8040_e7111,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign8040_e7113;

        let assign8050_e7132: f64 = if (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign8050_e7132;

        let (assign8060_e7150,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign8060_e7139: f64 = (p.p596 * locals.var_ile);
        let assign8060_e7140: f64 = (p.p595 + assign8060_e7139);
        let assign8060_e7143: f64 = (p.p597 * locals.var_iwe);
        let assign8060_e7144: f64 = (assign8060_e7140 + assign8060_e7143);
        let assign8060_e7147: f64 = (p.p598 * locals.var_iae);
        let assign8060_e7148: f64 = (assign8060_e7144 + assign8060_e7147);
        (assign8060_e7148,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign8060_e7150;

        let assign8070_e7169: f64 = if (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]) { 1.0 } else { 0.0 };
        locals.var_guard93 = assign8070_e7169;

        let (assign8080_e7187,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign8080_e7176: f64 = (p.p600 * locals.var_ile);
        let assign8080_e7177: f64 = (p.p599 + assign8080_e7176);
        let assign8080_e7180: f64 = (p.p601 * locals.var_iwe);
        let assign8080_e7181: f64 = (assign8080_e7177 + assign8080_e7180);
        let assign8080_e7184: f64 = (p.p602 * locals.var_iae);
        let assign8080_e7185: f64 = (assign8080_e7181 + assign8080_e7184);
        (assign8080_e7185,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign8080_e7187;

        let assign8090_e7206: f64 = if (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]) { 1.0 } else { 0.0 };
        locals.var_guard94 = assign8090_e7206;

        let (assign8100_e7224,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard94 != 0.0)) {
        let assign8100_e7213: f64 = (p.p604 * locals.var_ile);
        let assign8100_e7214: f64 = (p.p603 + assign8100_e7213);
        let assign8100_e7217: f64 = (p.p605 * locals.var_iwe);
        let assign8100_e7218: f64 = (assign8100_e7214 + assign8100_e7217);
        let assign8100_e7221: f64 = (p.p606 * locals.var_iae);
        let assign8100_e7222: f64 = (assign8100_e7218 + assign8100_e7221);
        (assign8100_e7222,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign8100_e7224;

        let assign8110_e7243: f64 = if (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]) { 1.0 } else { 0.0 };
        locals.var_guard95 = assign8110_e7243;

        let (assign8120_e7261,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign8120_e7250: f64 = (p.p608 * locals.var_ile);
        let assign8120_e7251: f64 = (p.p607 + assign8120_e7250);
        let assign8120_e7254: f64 = (p.p609 * locals.var_iwe);
        let assign8120_e7255: f64 = (assign8120_e7251 + assign8120_e7254);
        let assign8120_e7258: f64 = (p.p610 * locals.var_iae);
        let assign8120_e7259: f64 = (assign8120_e7255 + assign8120_e7258);
        (assign8120_e7259,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign8120_e7261;

        let assign8130_e7280: f64 = if (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]) { 1.0 } else { 0.0 };
        locals.var_guard96 = assign8130_e7280;

        let (assign8140_e7298,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard96 != 0.0)) {
        let assign8140_e7287: f64 = (p.p612 * locals.var_ile);
        let assign8140_e7288: f64 = (p.p611 + assign8140_e7287);
        let assign8140_e7291: f64 = (p.p613 * locals.var_iwe);
        let assign8140_e7292: f64 = (assign8140_e7288 + assign8140_e7291);
        let assign8140_e7295: f64 = (p.p614 * locals.var_iae);
        let assign8140_e7296: f64 = (assign8140_e7292 + assign8140_e7295);
        (assign8140_e7296,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign8140_e7298;

        let assign8150_e7317: f64 = if (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]) { 1.0 } else { 0.0 };
        locals.var_guard97 = assign8150_e7317;

        let (assign8160_e7335,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard97 != 0.0)) {
        let assign8160_e7324: f64 = (p.p616 * locals.var_ile);
        let assign8160_e7325: f64 = (p.p615 + assign8160_e7324);
        let assign8160_e7328: f64 = (p.p617 * locals.var_iwe);
        let assign8160_e7329: f64 = (assign8160_e7325 + assign8160_e7328);
        let assign8160_e7332: f64 = (p.p618 * locals.var_iae);
        let assign8160_e7333: f64 = (assign8160_e7329 + assign8160_e7332);
        (assign8160_e7333,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign8160_e7335;

        let assign8170_e7354: f64 = if (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8170_e7354;

        let (assign8180_e7374,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard98 != 0.0)) {
        let assign8180_e7362: f64 = (p.p620 * locals.var_ile);
        let assign8180_e7363: f64 = (p.p619 + assign8180_e7362);
        let assign8180_e7366: f64 = (p.p621 * locals.var_iwe);
        let assign8180_e7367: f64 = (assign8180_e7363 + assign8180_e7366);
        let assign8180_e7370: f64 = (p.p622 * locals.var_iae);
        let assign8180_e7371: f64 = (assign8180_e7367 + assign8180_e7370);
        let assign8180_e7372: f64 = (locals.var_iiae * assign8180_e7371);
        (assign8180_e7372,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign8180_e7374;

        let assign8190_e7393: f64 = if (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8190_e7393;

        let (assign8200_e7413,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard99 != 0.0)) {
        let assign8200_e7401: f64 = (p.p624 * locals.var_ile);
        let assign8200_e7402: f64 = (p.p623 + assign8200_e7401);
        let assign8200_e7405: f64 = (p.p625 * locals.var_iwe);
        let assign8200_e7406: f64 = (assign8200_e7402 + assign8200_e7405);
        let assign8200_e7409: f64 = (p.p626 * locals.var_iae);
        let assign8200_e7410: f64 = (assign8200_e7406 + assign8200_e7409);
        let assign8200_e7411: f64 = (locals.var_iiwe * assign8200_e7410);
        (assign8200_e7411,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign8200_e7413;

        let assign8210_e7432: f64 = if (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8210_e7432;

        let (assign8220_e7452,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard100 != 0.0)) {
        let assign8220_e7440: f64 = (p.p628 * locals.var_ile);
        let assign8220_e7441: f64 = (p.p627 + assign8220_e7440);
        let assign8220_e7444: f64 = (p.p629 * locals.var_iwe);
        let assign8220_e7445: f64 = (assign8220_e7441 + assign8220_e7444);
        let assign8220_e7448: f64 = (p.p630 * locals.var_iae);
        let assign8220_e7449: f64 = (assign8220_e7445 + assign8220_e7448);
        let assign8220_e7450: f64 = (locals.var_iiwe * assign8220_e7449);
        (assign8220_e7450,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign8220_e7452;

        let assign8230_e7471: f64 = if (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]) { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8230_e7471;

        let (assign8240_e7489,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard101 != 0.0)) {
        let assign8240_e7478: f64 = (p.p632 * locals.var_ile);
        let assign8240_e7479: f64 = (p.p631 + assign8240_e7478);
        let assign8240_e7482: f64 = (p.p633 * locals.var_iwe);
        let assign8240_e7483: f64 = (assign8240_e7479 + assign8240_e7482);
        let assign8240_e7486: f64 = (p.p634 * locals.var_iae);
        let assign8240_e7487: f64 = (assign8240_e7483 + assign8240_e7486);
        (assign8240_e7487,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign8240_e7489;

        let assign8250_e7508: f64 = if (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8250_e7508;

        let (assign8260_e7528,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard102 != 0.0)) {
        let assign8260_e7516: f64 = (p.p636 * locals.var_ile);
        let assign8260_e7517: f64 = (p.p635 + assign8260_e7516);
        let assign8260_e7520: f64 = (p.p637 * locals.var_iwe);
        let assign8260_e7521: f64 = (assign8260_e7517 + assign8260_e7520);
        let assign8260_e7524: f64 = (p.p638 * locals.var_iae);
        let assign8260_e7525: f64 = (assign8260_e7521 + assign8260_e7524);
        let assign8260_e7526: f64 = (locals.var_iiwe * assign8260_e7525);
        (assign8260_e7526,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign8260_e7528;

        let assign8270_e7547: f64 = if (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8270_e7547;

        let (assign8280_e7567,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard103 != 0.0)) {
        let assign8280_e7555: f64 = (p.p640 * locals.var_ile);
        let assign8280_e7556: f64 = (p.p639 + assign8280_e7555);
        let assign8280_e7559: f64 = (p.p641 * locals.var_iwe);
        let assign8280_e7560: f64 = (assign8280_e7556 + assign8280_e7559);
        let assign8280_e7563: f64 = (p.p642 * locals.var_iae);
        let assign8280_e7564: f64 = (assign8280_e7560 + assign8280_e7563);
        let assign8280_e7565: f64 = (locals.var_iiwe * assign8280_e7564);
        (assign8280_e7565,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign8280_e7567;

        let assign8290_e7586: f64 = if (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8290_e7586;

        let (assign8300_e7604,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard104 != 0.0)) {
        let assign8300_e7593: f64 = (p.p644 * locals.var_ile);
        let assign8300_e7594: f64 = (p.p643 + assign8300_e7593);
        let assign8300_e7597: f64 = (p.p645 * locals.var_iwe);
        let assign8300_e7598: f64 = (assign8300_e7594 + assign8300_e7597);
        let assign8300_e7601: f64 = (p.p646 * locals.var_iae);
        let assign8300_e7602: f64 = (assign8300_e7598 + assign8300_e7601);
        (assign8300_e7602,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign8300_e7604;

        let assign8310_e7623: f64 = if (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]) { 1.0 } else { 0.0 };
        locals.var_guard105 = assign8310_e7623;

        let (assign8320_e7641,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard105 != 0.0)) {
        let assign8320_e7630: f64 = (p.p648 * locals.var_ile);
        let assign8320_e7631: f64 = (p.p647 + assign8320_e7630);
        let assign8320_e7634: f64 = (p.p649 * locals.var_iwe);
        let assign8320_e7635: f64 = (assign8320_e7631 + assign8320_e7634);
        let assign8320_e7638: f64 = (p.p650 * locals.var_iae);
        let assign8320_e7639: f64 = (assign8320_e7635 + assign8320_e7638);
        (assign8320_e7639,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign8320_e7641;

        let assign8330_e7660: f64 = if (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]) { 1.0 } else { 0.0 };
        locals.var_guard106 = assign8330_e7660;

        let (assign8340_e7684,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard106 != 0.0)) {
        let assign8340_e7666: f64 = (locals.var_iiwecv * locals.var_lecv);
        let assign8340_e7668: f64 = (assign8340_e7666 / 1e-6);
        let assign8340_e7672: f64 = (p.p652 * locals.var_ile);
        let assign8340_e7673: f64 = (p.p651 + assign8340_e7672);
        let assign8340_e7676: f64 = (p.p653 * locals.var_iwe);
        let assign8340_e7677: f64 = (assign8340_e7673 + assign8340_e7676);
        let assign8340_e7680: f64 = (p.p654 * locals.var_iae);
        let assign8340_e7681: f64 = (assign8340_e7677 + assign8340_e7680);
        let assign8340_e7682: f64 = (assign8340_e7668 * assign8340_e7681);
        (assign8340_e7682,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign8340_e7684;

        let assign8350_e7703: f64 = if (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]) { 1.0 } else { 0.0 };
        locals.var_guard107 = assign8350_e7703;

        let (assign8360_e7721,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard107 != 0.0)) {
        let assign8360_e7710: f64 = (p.p656 * locals.var_ile);
        let assign8360_e7711: f64 = (p.p655 + assign8360_e7710);
        let assign8360_e7714: f64 = (p.p657 * locals.var_iwe);
        let assign8360_e7715: f64 = (assign8360_e7711 + assign8360_e7714);
        let assign8360_e7718: f64 = (p.p658 * locals.var_iae);
        let assign8360_e7719: f64 = (assign8360_e7715 + assign8360_e7718);
        (assign8360_e7719,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign8360_e7721;

        let assign8370_e7740: f64 = if (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]) { 1.0 } else { 0.0 };
        locals.var_guard108 = assign8370_e7740;

        let (assign8380_e7758,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard108 != 0.0)) {
        let assign8380_e7747: f64 = (p.p660 * locals.var_ile);
        let assign8380_e7748: f64 = (p.p659 + assign8380_e7747);
        let assign8380_e7751: f64 = (p.p661 * locals.var_iwe);
        let assign8380_e7752: f64 = (assign8380_e7748 + assign8380_e7751);
        let assign8380_e7755: f64 = (p.p662 * locals.var_iae);
        let assign8380_e7756: f64 = (assign8380_e7752 + assign8380_e7755);
        (assign8380_e7756,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign8380_e7758;

        let assign8390_e7797: f64 = if (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign8390_e7797;

        let (assign8400_e7803,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p571,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8400_e7803;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign8410_e7805: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8410_e7807: f64 = if assign8410_e7805 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign8410_e7807;

        let (assign8420_e7815,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
        (p.p663,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8420_e7815;

        let (assign8430_e7821,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p572,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8430_e7821;

        let assign8440_e7823: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8440_e7825: f64 = if assign8440_e7823 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign8440_e7825;

        let (assign8450_e7833,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (p.p664,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8450_e7833;

        let (assign8460_e7839,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p573,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8460_e7839;

        let assign8470_e7841: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8470_e7843: f64 = if assign8470_e7841 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign8470_e7843;

        let (assign8480_e7851,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (p.p665,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8480_e7851;

        let (assign8490_e7857,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p574,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8490_e7857;

        let assign8500_e7859: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8500_e7861: f64 = if assign8500_e7859 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8500_e7861;

        let (assign8510_e7869,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p666,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8510_e7869;

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

        let assign8530_e7928: f64 = if (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8530_e7928;

        let (assign8540_e7934,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        (p.p587,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8540_e7934;

        let assign8550_e7936: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8550_e7938: f64 = if assign8550_e7936 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8550_e7938;

        let (assign8560_e7946,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard115 != 0.0)) {
        (p.p667,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8560_e7946;

        let (assign8570_e7952,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        (p.p588,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8570_e7952;

        let assign8580_e7954: f64 = if param_given[668] { 1.0 } else { 0.0 };
        let assign8580_e7956: f64 = if assign8580_e7954 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8580_e7956;

        let (assign8590_e7964,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard116 != 0.0)) {
        (p.p668,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8590_e7964;

        let (assign8600_e7970,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        (p.p589,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8600_e7970;

        let assign8610_e7972: f64 = if param_given[669] { 1.0 } else { 0.0 };
        let assign8610_e7974: f64 = if assign8610_e7972 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8610_e7974;

        let (assign8620_e7982,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard117 != 0.0)) {
        (p.p669,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8620_e7982;

        let (assign8630_e7988,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) {
        (p.p590,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8630_e7988;

        let assign8640_e7990: f64 = if param_given[670] { 1.0 } else { 0.0 };
        let assign8640_e7992: f64 = if assign8640_e7990 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8640_e7992;

        let (assign8650_e8000,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard114 != 0.0)) && (locals.var_guard118 != 0.0)) {
        (p.p670,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8650_e8000;

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

        let assign8670_e8039: f64 = if (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign8670_e8039;

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

        let assign8690_e8078: f64 = if (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign8690_e8078;

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

        let assign8710_e8117: f64 = if (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]) { 1.0 } else { 0.0 };
        locals.var_guard121 = assign8710_e8117;

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

        let assign8730_e8156: f64 = if (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]) { 1.0 } else { 0.0 };
        locals.var_guard122 = assign8730_e8156;

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

        let assign8750_e8195: f64 = if (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]) { 1.0 } else { 0.0 };
        locals.var_guard123 = assign8750_e8195;

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

        let assign8770_e8234: f64 = if (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]) { 1.0 } else { 0.0 };
        locals.var_guard124 = assign8770_e8234;

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

        let assign8790_e8273: f64 = if (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]) { 1.0 } else { 0.0 };
        locals.var_guard125 = assign8790_e8273;

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

        let assign8850_e8390: f64 = if (((param_given[707] || param_given[708]) || param_given[709]) || param_given[710]) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign8850_e8390;

        let (assign8860_e8410,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard128 != 0.0)) {
        let assign8860_e8398: f64 = (p.p708 * locals.var_ile);
        let assign8860_e8399: f64 = (p.p707 + assign8860_e8398);
        let assign8860_e8402: f64 = (p.p709 * locals.var_iwe);
        let assign8860_e8403: f64 = (assign8860_e8399 + assign8860_e8402);
        let assign8860_e8406: f64 = (p.p710 * locals.var_iae);
        let assign8860_e8407: f64 = (assign8860_e8403 + assign8860_e8406);
        let assign8860_e8408: f64 = (locals.var_ile2 * assign8860_e8407);
        (assign8860_e8408,)
    } else {
        (locals.var_fntexc_p,)
    }
};
        locals.var_fntexc_p = assign8860_e8410;

        let assign8930_e8546: f64 = if (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8930_e8546;

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

        let assign8950_e8583: f64 = if (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8950_e8583;

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

        let assign8970_e8620: f64 = if (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8970_e8620;

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

        let assign8990_e8657: f64 = if (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]) { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8990_e8657;

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

        let assign9010_e8694: f64 = if (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign9010_e8694;

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

        let assign9030_e8731: f64 = if (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign9030_e8731;

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

        let assign9050_e8772: f64 = if (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign9050_e8772;

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

        let assign9070_e8809: f64 = if (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign9070_e8809;

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

        let assign9090_e8848: f64 = if (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]) { 1.0 } else { 0.0 };
        locals.var_guard140 = assign9090_e8848;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
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

        let assign9110_e8885: f64 = if (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]) { 1.0 } else { 0.0 };
        locals.var_guard141 = assign9110_e8885;

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

        let assign9130_e8922: f64 = if (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]) { 1.0 } else { 0.0 };
        locals.var_guard142 = assign9130_e8922;

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

        let assign9150_e8961: f64 = if (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]) { 1.0 } else { 0.0 };
        locals.var_guard143 = assign9150_e8961;

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

        let assign9170_e8998: f64 = if (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign9170_e8998;

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

        let assign9250_e9152: f64 = if (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign9250_e9152;

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

        let (assign9270_e9174,) = {
    if (locals.var_guard41 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpa,)
    }
};
        locals.var_tmpa = assign9270_e9174;

        let (assign9280_e9178,) = {
    if (locals.var_guard41 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpb,)
    }
};
        locals.var_tmpb = assign9280_e9178;

        let (assign9290_e9182,) = {
    if (locals.var_guard41 != 0.0) {
        (0.0,)
    } else {
        (locals.var_loop_,)
    }
};
        locals.var_loop_ = assign9290_e9182;

        let (assign9300_e9186,) = {
    if (locals.var_guard41 != 0.0) {
        (p.p795,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9300_e9186;

        let assign9310_e9188: f64 = if param_given[796] { 1.0 } else { 0.0 };
        let assign9310_e9190: f64 = if assign9310_e9188 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign9310_e9190;

        let (assign9320_e9196,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard149 != 0.0)) {
        (p.p796,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9320_e9196;

        let assign9330_e9215: f64 = if (((locals.var_sa_i > 0.0) && (locals.var_sb_i > 0.0)) && ((locals.var_nf_i == 1.0) || ((locals.var_nf_i > 1.0) && (locals.var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard150 = assign9330_e9215;

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
            let (assign9340_body2_e9272,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9340_body2_e9270: f64 = (locals.var_loop_ + 1.0);
        (assign9340_body2_e9270,)
    } else {
        (locals.var_loop_,)
    }
};
            locals.var_loop_ = assign9340_body2_e9272;
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

        let (assign9360_e9288,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9360_e9286: f64 = (locals.var_tmpb * locals.var_invnf);
        (assign9360_e9286,)
    } else {
        (locals.var_invsb,)
    }
};
        locals.var_invsb = assign9360_e9288;

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

        let (assign9510_e9490,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9510_e9488: f64 = (locals.var_betn_p * locals.var_temp00);
        (assign9510_e9488,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9510_e9490;

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

        let (assign9540_e9538,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9540_e9536: f64 = (locals.var_betnedge_p * locals.var_temp00);
        (assign9540_e9536,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9540_e9538;

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

        let (assign9560_e9556,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9560_e9554: f64 = (locals.var_vfb_p + locals.var_temp00);
        (assign9560_e9554,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9560_e9556;

        let (assign9570_e9564,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9570_e9562: f64 = (locals.var_vfbedge_p + locals.var_temp00);
        (assign9570_e9562,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9570_e9564;

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

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9590_e9584,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9590_e9582: f64 = (locals.var_cf_p + locals.var_temp00);
        (assign9590_e9582,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign9590_e9584;

        let (assign9600_e9592,) = {
    if ((locals.var_guard41 != 0.0) && (locals.var_guard150 != 0.0)) {
        let assign9600_e9590: f64 = (locals.var_cfedge_p + locals.var_temp00);
        (assign9600_e9590,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign9600_e9592;

        let assign9610_e9607: f64 = if ((((locals.var_sca_i > 0.0) || (locals.var_scb_i > 0.0)) || (locals.var_scc_i > 0.0)) || (locals.var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard151 = assign9610_e9607;

        let assign9620_e9618: f64 = if (((locals.var_sca_i == 0.0) && (locals.var_scb_i == 0.0)) && (locals.var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard152 = assign9620_e9618;

        let (assign9630_e9628,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
        let assign9630_e9626: f64 = (locals.var_sc_i + locals.var_w_i);
        (assign9630_e9626,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9630_e9628;

        let (assign9640_e9638,) = {
    if (((locals.var_guard41 != 0.0) && (locals.var_guard151 != 0.0)) && (locals.var_guard152 != 0.0)) {
        let assign9640_e9636: f64 = (1.0 / p.p811);
        (assign9640_e9636,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9640_e9638;

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

        locals.var_vfb_i = locals.var_vfb_p;

        locals.var_stvfb_i = locals.var_stvfb_p;

        locals.var_st2vfb_i = locals.var_st2vfb_p;

        locals.var_tox_i = locals.var_tox_p;

        locals.var_epsrox_i = locals.var_epsrox_p;

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

        let (assign9790_e9812,) = {
    if (locals.var_gfacnud_p > 0.01) {
        (locals.var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        locals.var_gfacnud_i = assign9790_e9812;

        let (assign9800_e9818,) = {
    if (locals.var_vsbnud_p > 0.0) {
        (locals.var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        locals.var_vsbnud_i = assign9800_e9818;

        locals.var_dvsbnud_i = locals.var_dvsbnud_p;

        locals.var_dphib_i = locals.var_dphib_p;

        let (assign9830_e9826,) = {
    if (locals.var_np_p > 0.0) {
        (locals.var_np_p,)
    } else {
        (0.0,)
    }
};
        locals.var_np_i = assign9830_e9826;

        locals.var_toxov_i = locals.var_toxov_p;

        locals.var_toxovd_i = locals.var_toxovd_p;

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

        let (assign9880_e9856,) = {
    if (locals.var_ct_p > 0.0) {
        (locals.var_ct_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ct_i = assign9880_e9856;

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

        locals.var_stct_i = locals.var_stct_p;

        let (assign9920_e9885,) = {
    if (locals.var_cf_p > 0.0) {
        (locals.var_cf_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cf_i = assign9920_e9885;

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

        let (assign9940_e9902,) = {
    if (locals.var_cfd_p > 0.0) {
        (locals.var_cfd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfd_i = assign9940_e9902;

        let (assign9950_e9908,) = {
    if (locals.var_psce_p > 0.0) {
        (locals.var_psce_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psce_i = assign9950_e9908;

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

        let (assign9970_e9925,) = {
    if (locals.var_psced_p > 0.0) {
        (locals.var_psced_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psced_i = assign9970_e9925;

        let (assign9980_e9931,) = {
    if (locals.var_betn_p > 0.0) {
        (locals.var_betn_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betn_i = assign9980_e9931;

        locals.var_stbet_i = locals.var_stbet_p;

        let (assign10000_e9938,) = {
    if (locals.var_mue_p > 0.0) {
        (locals.var_mue_p,)
    } else {
        (0.0,)
    }
};
        locals.var_mue_i = assign10000_e9938;

        locals.var_stmue_i = locals.var_stmue_p;

        let (assign10020_e9945,) = {
    if (locals.var_themu_p > 0.0) {
        (locals.var_themu_p,)
    } else {
        (0.0,)
    }
};
        locals.var_themu_i = assign10020_e9945;

        locals.var_stthemu_i = locals.var_stthemu_p;

        let (assign10040_e9952,) = {
    if (locals.var_cs_p > 0.0) {
        (locals.var_cs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cs_i = assign10040_e9952;

        locals.var_stcs_i = locals.var_stcs_p;

        let (assign10060_e9959,) = {
    if (locals.var_thecs_p > 0.0) {
        (locals.var_thecs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thecs_i = assign10060_e9959;

        locals.var_stthecs_i = locals.var_stthecs_p;

        let (assign10080_e9966,) = {
    if (locals.var_xcor_p > 0.0) {
        (locals.var_xcor_p,)
    } else {
        (0.0,)
    }
};
        locals.var_xcor_i = assign10080_e9966;

        locals.var_stxcor_i = locals.var_stxcor_p;

        locals.var_feta_i = locals.var_feta_p;

        let (assign10110_e9974,) = {
    if (locals.var_rs_p > 0.0) {
        (locals.var_rs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rs_i = assign10110_e9974;

        locals.var_strs_i = locals.var_strs_p;

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

        let (assign10150_e10002,) = {
    if (locals.var_thesat_p > 0.0) {
        (locals.var_thesat_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesat_i = assign10150_e10002;

        locals.var_stthesat_i = locals.var_stthesat_p;

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

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let (assign10190_e10030,) = {
    if (locals.var_thesatt_p > 0.01) {
        (locals.var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        locals.var_thesatt_i = assign10190_e10030;

        let (assign10200_e10036,) = {
    if (locals.var_ax_p > 2.0) {
        (locals.var_ax_p,)
    } else {
        (2.0,)
    }
};
        locals.var_ax_i = assign10200_e10036;

        let (assign10210_e10042,) = {
    if (locals.var_alp_p > 0.0) {
        (locals.var_alp_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp_i = assign10210_e10042;

        let (assign10220_e10048,) = {
    if (locals.var_alp1_p > 0.0) {
        (locals.var_alp1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1_i = assign10220_e10048;

        let (assign10230_e10054,) = {
    if (locals.var_alp2_p > 0.0) {
        (locals.var_alp2_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp2_i = assign10230_e10054;

        locals.var_vp_i = locals.var_vp_p;

        let (assign10250_e10061,) = {
    if (locals.var_a1_p > 0.0) {
        (locals.var_a1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a1_i = assign10250_e10061;

        locals.var_a2_i = locals.var_a2_p;

        locals.var_sta2_i = locals.var_sta2_p;

        let (assign10280_e10069,) = {
    if (locals.var_a3_p > 0.0) {
        (locals.var_a3_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a3_i = assign10280_e10069;

        let (assign10290_e10075,) = {
    if (locals.var_a4_p > 0.0) {
        (locals.var_a4_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a4_i = assign10290_e10075;

        let (assign10300_e10081,) = {
    if (locals.var_imaxii_p > 1e-12) {
        (locals.var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        locals.var_imaxii_i = assign10300_e10081;

        locals.var_gco_i = locals.var_gco_p;

        let (assign10320_e10088,) = {
    if (locals.var_iginv_p > 0.0) {
        (locals.var_iginv_p,)
    } else {
        (0.0,)
    }
};
        locals.var_iginv_i = assign10320_e10088;

        let (assign10330_e10094,) = {
    if (locals.var_igov_p > 0.0) {
        (locals.var_igov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igov_i = assign10330_e10094;

        let (assign10340_e10100,) = {
    if (locals.var_igovd_p > 0.0) {
        (locals.var_igovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igovd_i = assign10340_e10100;

        locals.var_stig_i = locals.var_stig_p;

        locals.var_gc2_i = locals.var_gc2_p;

        locals.var_gc3_i = locals.var_gc3_p;

        locals.var_gc2ov_i = locals.var_gc2ov_p;

        locals.var_gc3ov_i = locals.var_gc3ov_p;

        locals.var_gc2ovd_i = locals.var_gc2ovd_p;

        locals.var_gc3ovd_i = locals.var_gc3ovd_p;

        locals.var_chib_i = locals.var_chib_p;

        let (assign10430_e10114,) = {
    if (locals.var_agidl_p > 0.0) {
        (locals.var_agidl_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidl_i = assign10430_e10114;

        let (assign10440_e10120,) = {
    if (locals.var_agidld_p > 0.0) {
        (locals.var_agidld_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidld_i = assign10440_e10120;

        locals.var_bgidl_i = locals.var_bgidl_p;

        locals.var_bgidld_i = locals.var_bgidld_p;

        locals.var_stbgidl_i = locals.var_stbgidl_p;

        locals.var_stbgidld_i = locals.var_stbgidld_p;

        locals.var_cgidl_i = locals.var_cgidl_p;

        locals.var_cgidld_i = locals.var_cgidld_p;

        let (assign10510_e10132,) = {
    if (locals.var_cox_p > 0.0) {
        (locals.var_cox_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cox_i = assign10510_e10132;

        locals.var_delvtac_i = locals.var_delvtac_p;

        let (assign10530_e10139,) = {
    if (locals.var_facneffac_p > 0.0) {
        (locals.var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_facneffac_i = assign10530_e10139;

        let (assign10540_e10145,) = {
    if (locals.var_thesatac_p > 0.0) {
        (locals.var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesatac_i = assign10540_e10145;

        let (assign10550_e10151,) = {
    if (locals.var_axac_p > 2.0) {
        (locals.var_axac_p,)
    } else {
        (2.0,)
    }
};
        locals.var_axac_i = assign10550_e10151;

        locals.var_alpac_i = locals.var_alpac_p;

        let (assign10570_e10158,) = {
    if (locals.var_alp1ac_p > 0.0) {
        (locals.var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1ac_i = assign10570_e10158;

        let (assign10580_e10164,) = {
    if (locals.var_cgov_p > 0.0) {
        (locals.var_cgov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgov_i = assign10580_e10164;

        let (assign10590_e10170,) = {
    if (locals.var_cgovd_p > 0.0) {
        (locals.var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgovd_i = assign10590_e10170;

        locals.var_fcgovacc_i = locals.var_fcgovacc_p;

        locals.var_fcgovaccd_i = locals.var_fcgovaccd_p;

        locals.var_cgovaccg_i = locals.var_cgovaccg_p;

        let (assign10630_e10179,) = {
    if (locals.var_cgbov_p > 0.0) {
        (locals.var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgbov_i = assign10630_e10179;

        let (assign10640_e10185,) = {
    if (locals.var_cinr_p > 0.0) {
        (locals.var_cinr_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinr_i = assign10640_e10185;

        let (assign10650_e10191,) = {
    if (locals.var_cinrd_p > 0.0) {
        (locals.var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinrd_i = assign10650_e10191;

        locals.var_dvfbinr_i = locals.var_dvfbinr_p;

        locals.var_fcinrdep_i = locals.var_fcinrdep_p;

        locals.var_fcinracc_i = locals.var_fcinracc_p;

        locals.var_axinr_i = locals.var_axinr_p;

        locals.var_fnt_i = locals.var_fnt_p;

        let (assign10730_e10214,) = {
    if (locals.var_fntexc_p > 0.0) {
        (locals.var_fntexc_p,)
    } else {
        (0.0,)
    }
};
        locals.var_fntexc_i = assign10730_e10214;

        locals.var_vfbedge_i = locals.var_vfbedge_p;

        locals.var_stvfbedge_i = locals.var_stvfbedge_p;

        locals.var_dphibedge_i = locals.var_dphibedge_p;

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

        let (assign10820_e10253,) = {
    if (locals.var_ctedge_p > 0.0) {
        (locals.var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ctedge_i = assign10820_e10253;

        let (assign10830_e10259,) = {
    if (locals.var_betnedge_p > 0.0) {
        (locals.var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betnedge_i = assign10830_e10259;

        locals.var_stbetedge_i = locals.var_stbetedge_p;

        let (assign10850_e10266,) = {
    if (locals.var_psceedge_p > 0.0) {
        (locals.var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psceedge_i = assign10850_e10266;

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

        let (assign10870_e10283,) = {
    if (locals.var_pscededge_p > 0.0) {
        (locals.var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_pscededge_i = assign10870_e10283;

        let (assign10880_e10289,) = {
    if (locals.var_cfedge_p > 0.0) {
        (locals.var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfedge_i = assign10880_e10289;

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

        let (assign10900_e10306,) = {
    if (locals.var_cfdedge_p > 0.0) {
        (locals.var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfdedge_i = assign10900_e10306;

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

        locals.var_factuo_i = p.p16;

        locals.var_delvto_i = p.p15;

        locals.var_factuoedge_i = p.p18;

        locals.var_delvtoedge_i = p.p17;

        let (assign11080_e10358,) = {
    if (locals.var_munqs_p > 0.0) {
        (locals.var_munqs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_munqs_i = assign11080_e10358;

        let assign11090_e10361: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign11090_e10361;

        let (assign11100_e10365,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_toxov_i,)
    } else {
        (locals.var_toxovd_i,)
    }
};
        locals.var_toxovd_i = assign11100_e10365;

        let (assign11110_e10369,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_nov_i,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign11110_e10369;

        let (assign11120_e10373,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_agidl_i,)
    } else {
        (locals.var_agidld_i,)
    }
};
        locals.var_agidld_i = assign11120_e10373;

        let (assign11130_e10377,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_bgidl_i,)
    } else {
        (locals.var_bgidld_i,)
    }
};
        locals.var_bgidld_i = assign11130_e10377;

        let (assign11140_e10381,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_stbgidl_i,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign11140_e10381;

        let (assign11150_e10385,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_cgidl_i,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign11150_e10385;

        let (assign11160_e10389,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_igov_i,)
    } else {
        (locals.var_igovd_i,)
    }
};
        locals.var_igovd_i = assign11160_e10389;

        let (assign11170_e10393,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_gc2ov_i,)
    } else {
        (locals.var_gc2ovd_i,)
    }
};
        locals.var_gc2ovd_i = assign11170_e10393;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11180_e10397,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_gc3ov_i,)
    } else {
        (locals.var_gc3ovd_i,)
    }
};
        locals.var_gc3ovd_i = assign11180_e10397;

        let (assign11190_e10401,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_cgov_i,)
    } else {
        (locals.var_cgovd_i,)
    }
};
        locals.var_cgovd_i = assign11190_e10401;

        let (assign11200_e10405,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_fcgovacc_i,)
    } else {
        (locals.var_fcgovaccd_i,)
    }
};
        locals.var_fcgovaccd_i = assign11200_e10405;

        let (assign11210_e10409,) = {
    if (locals.var_guard153 != 0.0) {
        (locals.var_cinr_i,)
    } else {
        (locals.var_cinrd_i,)
    }
};
        locals.var_cinrd_i = assign11210_e10409;

        let assign11230_e10416: f64 = (8.8541878176e-12 * locals.var_epsrox_i);
        locals.var_epsox = assign11230_e10416;

        let assign11240_e10419: f64 = (locals.var_epsox / locals.var_tox_i);
        locals.var_coxprime = assign11240_e10419;

        let assign11250_e10422: f64 = (locals.var_tox_i * locals.var_tox_i);
        locals.var_tox_sq = assign11250_e10422;

        let assign11260_e10425: f64 = (locals.var_coxprime / 1.6021918e-19);
        locals.var_cox_over_q = assign11260_e10425;

        let assign11270_e10428: f64 = (locals.var_facneffac_i * locals.var_neff_i);
        locals.var_neffac_i = assign11270_e10428;

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

        locals.var_qq = 0.0;

        let assign11300_e10443: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign11300_e10443;

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

        let assign11320_e10458: f64 = (-1.0);
        let assign11320_e10459: f64 = if locals.var_chnl_type == assign11320_e10458 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign11320_e10459;

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

        let assign11340_e10472: f64 = (1e-8 * locals.var_coxprime);
        let assign11340_e10474: f64 = (assign11340_e10472 / locals.var_epssi);
        locals.var_e_eff0 = assign11340_e10474;

        let assign11350_e10477: f64 = (0.5 * locals.var_feta_i);
        locals.var_eta_mu = assign11350_e10477;

        locals.var_eta_mu1 = 0.5;

        let assign11370_e10481: f64 = (-1.0);
        let assign11370_e10482: f64 = if locals.var_chnl_type == assign11370_e10481 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign11370_e10482;

        let (assign11380_e10488,) = {
    if (locals.var_guard156 != 0.0) {
        let assign11380_e10486: f64 = (0.3333333333333333 * locals.var_feta_i);
        (assign11380_e10486,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign11380_e10488;

        let (assign11390_e10492,) = {
    if (locals.var_guard156 != 0.0) {
        (0.3333333333333333,)
    } else {
        (locals.var_eta_mu1,)
    }
};
        locals.var_eta_mu1 = assign11390_e10492;

        let assign11400_e10495: f64 = (-2.0);
        let assign11400_e10497: f64 = (assign11400_e10495 / locals.var_ax_i);
        let assign11400_e10499: f64 = (assign11400_e10497 + 1.0);
        let assign11400_e10500: f64 = (2.0_f64).powf(assign11400_e10499);
        let assign11400_e10502: f64 = (assign11400_e10500 - 1.0);
        locals.var_temp = assign11400_e10502;

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

        let assign11420_e10523: f64 = (-2.0);
        let assign11420_e10525: f64 = (assign11420_e10523 / locals.var_axac_i);
        let assign11420_e10527: f64 = (assign11420_e10525 + 1.0);
        let assign11420_e10528: f64 = (2.0_f64).powf(assign11420_e10527);
        let assign11420_e10530: f64 = (assign11420_e10528 - 1.0);
        locals.var_temp = assign11420_e10530;

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

        let assign11440_e10551: f64 = (1.0 / locals.var_vp_i);
        locals.var_inv_vp = assign11440_e10551;

        let assign11450_e10554: f64 = (locals.var_epsox / locals.var_toxov_i);
        locals.var_coxovprime = assign11450_e10554;

        let assign11460_e10557: f64 = (locals.var_epsox / locals.var_toxovd_i);
        locals.var_coxovprime_d = assign11460_e10557;

        let assign11470_e10560: f64 = (2.0 * 1.6021918e-19);
        let assign11470_e10562: f64 = (assign11470_e10560 * locals.var_nov_i);
        let assign11470_e10564: f64 = (assign11470_e10562 * locals.var_epssi);
        let assign11470_e10566: f64 = (assign11470_e10564 * locals.var_inv_phita);
        let assign11470_e10567: f64 = (assign11470_e10566).sqrt();
        let assign11470_e10569: f64 = (assign11470_e10567 / locals.var_coxovprime);
        locals.var_gov_s = assign11470_e10569;

        let assign11480_e10572: f64 = (2.0 * 1.6021918e-19);
        let assign11480_e10574: f64 = (assign11480_e10572 * locals.var_novd_i);
        let assign11480_e10576: f64 = (assign11480_e10574 * locals.var_epssi);
        let assign11480_e10578: f64 = (assign11480_e10576 * locals.var_inv_phita);
        let assign11480_e10579: f64 = (assign11480_e10578).sqrt();
        let assign11480_e10581: f64 = (assign11480_e10579 / locals.var_coxovprime_d);
        locals.var_gov_d = assign11480_e10581;

        let assign11490_e10584: f64 = (locals.var_gov_s * locals.var_gov_s);
        locals.var_gov2_s = assign11490_e10584;

        let assign11500_e10587: f64 = (locals.var_gov_d * locals.var_gov_d);
        locals.var_gov2_d = assign11500_e10587;

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

        let assign11520_e10609: f64 = (0.5 * locals.var_gov_s);
        let assign11520_e10610: f64 = (assign11520_e10609).ln();
        let assign11520_e10612: f64 = (assign11520_e10610 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_s = assign11520_e10612;

        let assign11530_e10615: f64 = (0.5 * locals.var_gov_d);
        let assign11530_e10616: f64 = (assign11530_e10615).ln();
        let assign11530_e10618: f64 = (assign11530_e10616 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_d = assign11530_e10618;

        let assign11540_e10621: f64 = (1.0 / locals.var_gov_s);
        locals.var_inv_gov = assign11540_e10621;

        let assign11550_e10624: f64 = (3.1 * locals.var_gov_s);
        let assign11550_e10626: f64 = (assign11550_e10624 + 8.5);
        locals.var_sp_ov_eps = assign11550_e10626;

        let assign11560_e10629: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_s = assign11560_e10629;

        let assign11570_e10632: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11570_e10632;

        let assign11580_e10635: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign11580_e10635;

        let (assign11590_e10641,) = {
    if (locals.var_guard157 != 0.0) {
        let assign11590_e10639: f64 = (64.0 * locals.var_inv_gov);
        (assign11590_e10639,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11590_e10641;

        let assign11600_e10644: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign11600_e10644;

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

        let assign11620_e10658: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign11620_e10658;

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

        let (assign11640_e10684,) = {
    if (((locals.var_guard157 == 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) {
        (locals.var_gov_s,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11640_e10684;

        let assign11650_e10688: f64 = (locals.var_gov2_s * 0.5);
        let assign11650_e10689: f64 = (locals.var_sp_ov_delta + assign11650_e10688);
        let assign11650_e10694: f64 = (locals.var_gov2_s * 0.25);
        let assign11650_e10695: f64 = (locals.var_sp_ov_delta + assign11650_e10694);
        let assign11650_e10697: f64 = (assign11650_e10695 + locals.var_sp_ov_a_s);
        let assign11650_e10698: f64 = (assign11650_e10697).sqrt();
        let assign11650_e10699: f64 = (locals.var_gov_s * assign11650_e10698);
        let assign11650_e10700: f64 = (assign11650_e10689 - assign11650_e10699);
        locals.var_sp_ov_delta1_s = assign11650_e10700;

        let assign11660_e10703: f64 = (1.0 / locals.var_gov_d);
        locals.var_inv_gov = assign11660_e10703;

        let assign11670_e10706: f64 = (3.1 * locals.var_gov_d);
        let assign11670_e10708: f64 = (assign11670_e10706 + 8.5);
        locals.var_sp_ov_eps = assign11670_e10708;

        let assign11680_e10711: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_d = assign11680_e10711;

        let assign11690_e10714: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11690_e10714;

        let assign11700_e10717: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign11700_e10717;

        let (assign11710_e10723,) = {
    if (locals.var_guard160 != 0.0) {
        let assign11710_e10721: f64 = (64.0 * locals.var_inv_gov);
        (assign11710_e10721,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11710_e10723;

        let assign11720_e10726: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign11720_e10726;

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

        let assign11740_e10740: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign11740_e10740;

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

        let (assign11760_e10766,) = {
    if (((locals.var_guard160 == 0.0) && (locals.var_guard161 == 0.0)) && (locals.var_guard162 == 0.0)) {
        (locals.var_gov_d,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11760_e10766;

        let assign11770_e10770: f64 = (locals.var_gov2_d * 0.5);
        let assign11770_e10771: f64 = (locals.var_sp_ov_delta + assign11770_e10770);
        let assign11770_e10776: f64 = (locals.var_gov2_d * 0.25);
        let assign11770_e10777: f64 = (locals.var_sp_ov_delta + assign11770_e10776);
        let assign11770_e10779: f64 = (assign11770_e10777 + locals.var_sp_ov_a_d);
        let assign11770_e10780: f64 = (assign11770_e10779).sqrt();
        let assign11770_e10781: f64 = (locals.var_gov_d * assign11770_e10780);
        let assign11770_e10782: f64 = (assign11770_e10771 - assign11770_e10781);
        locals.var_sp_ov_delta1_d = assign11770_e10782;

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

        let (assign11790_e10805,) = {
    if (locals.var_phib_dc > 0.05) {
        (locals.var_phib_dc,)
    } else {
        (0.05,)
    }
};
        locals.var_phib_dc = assign11790_e10805;

        let assign11800_e10808: f64 = (2.0 * 1.6021918e-19);
        let assign11800_e10810: f64 = (assign11800_e10808 * locals.var_neff_i);
        let assign11800_e10812: f64 = (assign11800_e10810 * locals.var_epssi);
        let assign11800_e10814: f64 = (assign11800_e10812 * locals.var_inv_phit);
        let assign11800_e10815: f64 = (assign11800_e10814).sqrt();
        let assign11800_e10817: f64 = (assign11800_e10815 / locals.var_coxprime);
        locals.var_g_0_dc = assign11800_e10817;

        locals.var_kp = 0.0;

        locals.var_np = 0.0;

        let assign11830_e10822: f64 = if locals.var_np_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign11830_e10822;

        let (assign11840_e10828,) = {
    if (locals.var_guard163 != 0.0) {
        let assign11840_e10826: f64 = (80000000.0 / locals.var_tox_sq);
        (assign11840_e10826,)
    } else {
        (locals.var_arg2max,)
    }
};
        locals.var_arg2max = assign11840_e10828;

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

        let assign11880_e10865: f64 = (100.0 * locals.var_phit);
        let assign11880_e10867: f64 = (assign11880_e10865 * locals.var_phit);
        locals.var_qlim2 = assign11880_e10867;

        let assign11890_e10870: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard164 = assign11890_e10870;

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

        let (assign11920_e10897,) = {
    if (locals.var_guard164 != 0.0) {
        let assign11920_e10895: f64 = (locals.var_phib_dc + locals.var_dphibq);
        (assign11920_e10895,)
    } else {
        (locals.var_phib_dc,)
    }
};
        locals.var_phib_dc = assign11920_e10897;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign11940_e10913: f64 = (locals.var_phib_dc).sqrt();
        locals.var_sqrt_phib_dc = assign11940_e10913;

        let assign11950_e10916: f64 = (0.95 * locals.var_phib_dc);
        locals.var_phix_dc = assign11950_e10916;

        let assign11960_e10919: f64 = (0.0025 * locals.var_phib_dc);
        let assign11960_e10921: f64 = (assign11960_e10919 * locals.var_phib_dc);
        locals.var_aphi_dc = assign11960_e10921;

        locals.var_bphi_dc = locals.var_aphi_dc;

        let assign11980_e10925: f64 = (locals.var_bphi_dc).sqrt();
        let assign11980_e10926: f64 = (0.5 * assign11980_e10925);
        locals.var_phix2 = assign11980_e10926;

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

        let assign12000_e10952: f64 = (locals.var_phib_dc + locals.var_eg);
        let assign12000_e10953: f64 = (0.5 * assign12000_e10952);
        locals.var_alpha_b = assign12000_e10953;

        let assign12010_e10956: f64 = (locals.var_vsbnud_i + locals.var_phib_dc);
        let assign12010_e10957: f64 = (assign12010_e10956).sqrt();
        let assign12010_e10959: f64 = (assign12010_e10957 - locals.var_sqrt_phib_dc);
        locals.var_us1 = assign12010_e10959;

        let assign12020_e10962: f64 = (locals.var_vsbnud_i + locals.var_dvsbnud_i);
        let assign12020_e10964: f64 = (assign12020_e10962 + locals.var_phib_dc);
        let assign12020_e10965: f64 = (assign12020_e10964).sqrt();
        let assign12020_e10967: f64 = (assign12020_e10965 - locals.var_sqrt_phib_dc);
        let assign12020_e10969: f64 = (assign12020_e10967 - locals.var_us1);
        locals.var_us21 = assign12020_e10969;

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

        let (assign12040_e10994,) = {
    if (locals.var_phib_ac > 0.05) {
        (locals.var_phib_ac,)
    } else {
        (0.05,)
    }
};
        locals.var_phib_ac = assign12040_e10994;

        let assign12050_e10997: f64 = (2.0 * 1.6021918e-19);
        let assign12050_e10999: f64 = (assign12050_e10997 * locals.var_neffac_i);
        let assign12050_e11001: f64 = (assign12050_e10999 * locals.var_epssi);
        let assign12050_e11003: f64 = (assign12050_e11001 * locals.var_inv_phit);
        let assign12050_e11004: f64 = (assign12050_e11003).sqrt();
        let assign12050_e11006: f64 = (assign12050_e11004 / locals.var_coxprime);
        locals.var_g_0_ac = assign12050_e11006;

        let assign12060_e11009: f64 = if p.p52 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign12060_e11009;

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

        let (assign12090_e11036,) = {
    if (locals.var_guard165 != 0.0) {
        let assign12090_e11034: f64 = (locals.var_phib_ac + locals.var_dphibq);
        (assign12090_e11034,)
    } else {
        (locals.var_phib_ac,)
    }
};
        locals.var_phib_ac = assign12090_e11036;

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

        let assign12110_e11053: f64 = (0.95 * locals.var_phib_ac);
        locals.var_phix_ac = assign12110_e11053;

        let assign12120_e11056: f64 = (0.0025 * locals.var_phib_ac);
        let assign12120_e11058: f64 = (assign12120_e11056 * locals.var_phib_ac);
        locals.var_aphi_ac = assign12120_e11058;

        locals.var_bphi_ac = locals.var_aphi_ac;

        let assign12140_e11062: f64 = (locals.var_bphi_ac).sqrt();
        let assign12140_e11063: f64 = (0.5 * assign12140_e11062);
        locals.var_phix2 = assign12140_e11063;

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

        let assign12160_e11089: f64 = (locals.var_stvfb_i * locals.var_delt);
        let assign12160_e11093: f64 = (locals.var_st2vfb_i * locals.var_delt);
        let assign12160_e11094: f64 = (1.0 + assign12160_e11093);
        let assign12160_e11095: f64 = (assign12160_e11089 * assign12160_e11094);
        let assign12160_e11096: f64 = (locals.var_vfb_i + assign12160_e11095);
        let assign12160_e11098: f64 = (assign12160_e11096 + locals.var_delvto_i);
        locals.var_vfb_t = assign12160_e11098;

        let assign12170_e11101: f64 = (locals.var_stct_i * locals.var_ln_rtn);
        let assign12170_e11102: f64 = (assign12170_e11101).exp();
        locals.var_tf_ct = assign12170_e11102;

        let assign12180_e11105: f64 = (locals.var_ct_i * locals.var_tf_ct);
        locals.var_ct_t = assign12180_e11105;

        let assign12190_e11108: f64 = (locals.var_ctg_i / locals.var_rtn);
        locals.var_ctg_t = assign12190_e11108;

        let assign12200_e11111: f64 = (locals.var_stbet_i * locals.var_ln_rtn);
        let assign12200_e11112: f64 = (assign12200_e11111).exp();
        locals.var_tf_bet = assign12200_e11112;

        let assign12210_e11115: f64 = (locals.var_betn_i * locals.var_tf_bet);
        locals.var_betn_t = assign12210_e11115;

        let assign12220_e11118: f64 = (locals.var_factuo_i * locals.var_betn_t);
        let assign12220_e11120: f64 = (assign12220_e11118 * locals.var_coxprime);
        locals.var_bet_i = assign12220_e11120;

        let assign12230_e11124: f64 = (locals.var_stthemu_i * locals.var_ln_rtn);
        let assign12230_e11125: f64 = (assign12230_e11124).exp();
        let assign12230_e11126: f64 = (locals.var_themu_i * assign12230_e11125);
        locals.var_themu_t = assign12230_e11126;

        let assign12240_e11129: f64 = (locals.var_stmue_i * locals.var_ln_rtn);
        let assign12240_e11130: f64 = (assign12240_e11129).exp();
        locals.var_tf_mue = assign12240_e11130;

        let assign12250_e11133: f64 = (locals.var_mue_i * locals.var_tf_mue);
        locals.var_mue_t = assign12250_e11133;

        let assign12260_e11137: f64 = (locals.var_stthecs_i * locals.var_ln_rtn);
        let assign12260_e11138: f64 = (assign12260_e11137).exp();
        let assign12260_e11139: f64 = (locals.var_thecs_i * assign12260_e11138);
        locals.var_thecs_t = assign12260_e11139;

        let assign12270_e11142: f64 = (locals.var_stcs_i * locals.var_ln_rtn);
        let assign12270_e11143: f64 = (assign12270_e11142).exp();
        locals.var_tf_cs = assign12270_e11143;

        let assign12280_e11146: f64 = (locals.var_cs_i * locals.var_tf_cs);
        locals.var_cs_t = assign12280_e11146;

        let assign12290_e11149: f64 = (locals.var_stxcor_i * locals.var_ln_rtn);
        let assign12290_e11150: f64 = (assign12290_e11149).exp();
        locals.var_tf_xcor = assign12290_e11150;

        let assign12300_e11153: f64 = (locals.var_xcor_i * locals.var_tf_xcor);
        locals.var_xcor_t = assign12300_e11153;

        let assign12310_e11156: f64 = (locals.var_strs_i * locals.var_ln_rtn);
        let assign12310_e11157: f64 = (assign12310_e11156).exp();
        locals.var_tf_ther = assign12310_e11157;

        let assign12320_e11160: f64 = (locals.var_rs_i * locals.var_tf_ther);
        locals.var_rs_t = assign12320_e11160;

        let assign12330_e11163: f64 = (2.0 * locals.var_bet_i);
        let assign12330_e11165: f64 = (assign12330_e11163 * locals.var_rs_t);
        locals.var_ther_i = assign12330_e11165;

        let assign12340_e11168: f64 = (locals.var_stthesat_i * locals.var_ln_rtn);
        let assign12340_e11169: f64 = (assign12340_e11168).exp();
        locals.var_tf_thesat = assign12340_e11169;

        let assign12350_e11172: f64 = (locals.var_thesat_i * locals.var_tf_thesat);
        locals.var_thesat_t = assign12350_e11172;

        let assign12360_e11175: f64 = (locals.var_thesatac_i * locals.var_tf_thesat);
        locals.var_thesatac_t = assign12360_e11175;

        let assign12370_e11178: f64 = (-locals.var_sta2_i);
        let assign12370_e11180: f64 = (assign12370_e11178 * locals.var_ln_rtn);
        let assign12370_e11181: f64 = (assign12370_e11180).exp();
        let assign12370_e11182: f64 = (locals.var_a2_i * assign12370_e11181);
        locals.var_a2_t = assign12370_e11182;

        let assign12380_e11185: f64 = (locals.var_fnt_i * 4.0);
        let assign12380_e11187: f64 = (assign12380_e11185 * 1.3806505e-23);
        let assign12380_e11189: f64 = (assign12380_e11187 * locals.var_tkd);
        locals.var_nt = assign12380_e11189;

        let assign12400_e11203: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard166 = assign12400_e11203;

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

        let (assign12430_e11226,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12430_e11224: f64 = (locals.var_betnedge_i * locals.var_tf_betedge);
        (assign12430_e11224,)
    } else {
        (locals.var_betnedge_t,)
    }
};
        locals.var_betnedge_t = assign12430_e11226;

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

        let (assign12490_e11294,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12490_e11292: f64 = (locals.var_gfedge * locals.var_gfedge);
        (assign12490_e11292,)
    } else {
        (locals.var_gfedge2,)
    }
};
        locals.var_gfedge2 = assign12490_e11294;

        let (assign12500_e11299,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12500_e11297: f64 = (locals.var_gfedge2).ln();
        (assign12500_e11297,)
    } else {
        (locals.var_lngfedge2,)
    }
};
        locals.var_lngfedge2 = assign12500_e11299;

        let (assign12510_e11305,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12510_e11303: f64 = (0.95 * locals.var_phibedge);
        (assign12510_e11303,)
    } else {
        (locals.var_phixedge,)
    }
};
        locals.var_phixedge = assign12510_e11305;

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

        let (assign12530_e11317,) = {
    if (locals.var_guard166 != 0.0) {
        (locals.var_aphiedge,)
    } else {
        (locals.var_bphiedge,)
    }
};
        locals.var_bphiedge = assign12530_e11317;

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

        let (assign12580_e11374,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_vfbedge_t,)
    }
};
        locals.var_vfbedge_t = assign12580_e11374;

        let (assign12590_e11379,) = {
    if (locals.var_guard166 == 0.0) {
        (1.0,)
    } else {
        (locals.var_tf_betedge,)
    }
};
        locals.var_tf_betedge = assign12590_e11379;

        let (assign12600_e11384,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_betnedge_t,)
    }
};
        locals.var_betnedge_t = assign12600_e11384;

        let (assign12610_e11389,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_betedge_i,)
    }
};
        locals.var_betedge_i = assign12610_e11389;

        let (assign12620_e11394,) = {
    if (locals.var_guard166 == 0.0) {
        (locals.var_phit,)
    } else {
        (locals.var_phit0edge,)
    }
};
        locals.var_phit0edge = assign12620_e11394;

        let (assign12630_e11399,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phibedge,)
    }
};
        locals.var_phibedge = assign12630_e11399;

        let (assign12640_e11404,) = {
    if (locals.var_guard166 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gfedge,)
    }
};
        locals.var_gfedge = assign12640_e11404;

    }

    pub(super) fn stamp_transient_block_13(
        ctx: &GeneratedEvalContext<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign12650_e11409,) = {
    if (locals.var_guard166 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gfedge2,)
    }
};
        locals.var_gfedge2 = assign12650_e11409;

        let (assign12660_e11414,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_lngfedge2,)
    }
};
        locals.var_lngfedge2 = assign12660_e11414;

        let (assign12670_e11419,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phixedge,)
    }
};
        locals.var_phixedge = assign12670_e11419;

        let (assign12680_e11424,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_aphiedge,)
    }
};
        locals.var_aphiedge = assign12680_e11424;

        let (assign12690_e11429,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_bphiedge,)
    }
};
        locals.var_bphiedge = assign12690_e11429;

        let (assign12700_e11434,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phix2edge,)
    }
};
        locals.var_phix2edge = assign12700_e11434;

        let (assign12710_e11439,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phix1edge,)
    }
};
        locals.var_phix1edge = assign12710_e11439;

        let assign12740_e11452: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign12740_e11452;

        let assign12750_e11455: f64 = (4.0 * 0.3333333333333333);
        let assign12750_e11458: f64 = (2.0 * 1.6021918e-19);
        let assign12750_e11460: f64 = (assign12750_e11458 * 9.1093826e-31);
        let assign12750_e11462: f64 = (assign12750_e11460 * locals.var_chib_i);
        let assign12750_e11463: f64 = (assign12750_e11462).sqrt();
        let assign12750_e11464: f64 = (assign12750_e11455 * assign12750_e11463);
        let assign12750_e11466: f64 = (assign12750_e11464 / 1.05457168e-34);
        locals.var_b_fact = assign12750_e11466;

        let assign12760_e11469: f64 = (locals.var_b_fact * locals.var_tox_i);
        locals.var_bch = assign12760_e11469;

        let assign12770_e11472: f64 = (locals.var_b_fact * locals.var_toxov_i);
        locals.var_bov = assign12770_e11472;

        let assign12780_e11475: f64 = (locals.var_b_fact * locals.var_toxovd_i);
        locals.var_bov_d = assign12780_e11475;

        locals.var_gcq = 0.0;

        let assign12800_e11479: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard167 = assign12800_e11479;

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

        locals.var_gcqov = 0.0;

        let assign12830_e11492: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign12830_e11492;

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

        let assign12850_e11504: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign12850_e11504;

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

        let assign12870_e11516: f64 = (locals.var_rta).powf(locals.var_stig_i);
        locals.var_tf_ig = assign12870_e11516;

        let assign12880_e11519: f64 = (locals.var_iginv_i * locals.var_tf_ig);
        locals.var_iginv_i = assign12880_e11519;

        let assign12890_e11522: f64 = (locals.var_igov_i * locals.var_tf_ig);
        locals.var_igov_i = assign12890_e11522;

        let assign12900_e11525: f64 = (locals.var_igovd_i * locals.var_tf_ig);
        locals.var_igovd_i = assign12900_e11525;

        let assign12910_e11528: f64 = (locals.var_agidl_i * 4e-18);
        let assign12910_e11531: f64 = (locals.var_toxov_i * locals.var_toxov_i);
        let assign12910_e11532: f64 = (assign12910_e11528 / assign12910_e11531);
        locals.var_agidls = assign12910_e11532;

        let assign12920_e11535: f64 = (locals.var_agidld_i * 4e-18);
        let assign12920_e11538: f64 = (locals.var_toxovd_i * locals.var_toxovd_i);
        let assign12920_e11539: f64 = (assign12920_e11535 / assign12920_e11538);
        locals.var_agidlds = assign12920_e11539;

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

        let assign12940_e11556: f64 = (locals.var_bgidl_i * locals.var_b_fact);
        locals.var_bgidl_t = assign12940_e11556;

        let assign12950_e11559: f64 = (locals.var_bgidl_t * locals.var_toxov_i);
        let assign12950_e11561: f64 = (assign12950_e11559 * 500000000.0);
        locals.var_bgidls = assign12950_e11561;

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

        let assign12970_e11578: f64 = (locals.var_bgidld_i * locals.var_b_fact);
        locals.var_bgidld_t = assign12970_e11578;

        let assign12980_e11581: f64 = (locals.var_bgidld_t * locals.var_toxovd_i);
        let assign12980_e11583: f64 = (assign12980_e11581 * 500000000.0);
        locals.var_bgidlds = assign12980_e11583;

        locals.var_vinr_max = 0.0;

        let assign13000_e11587: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign13000_e11587;

        let (assign13010_e11593,) = {
    if (locals.var_guard170 != 0.0) {
        let assign13010_e11591: f64 = (0.75 / locals.var_fcinracc_i);
        (assign13010_e11591,)
    } else {
        (locals.var_vinr_max,)
    }
};
        locals.var_vinr_max = assign13010_e11593;

        let assign13020_e11596: f64 = (locals.var_axinr_i * locals.var_axinr_i);
        locals.var_ainr = assign13020_e11596;

        let assign13030_e11599: f64 = (9.1093826e-31 * 1000000000.0);
        let assign13030_e11601: f64 = (assign13030_e11599 * locals.var_fntexc_i);
        locals.var_fac_exc = assign13030_e11601;

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

        let assign40530_e53716: f64 = 1.0;
        let assign40530_e53717: f64 = if locals.var_chnl_type == assign40530_e53716 { 1.0 } else { 0.0 };
        locals.var_guard1113 = assign40530_e53717;

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

        let assign40640_e53770: f64 = (locals.var_v_gs + locals.var_v_sb);
        locals.var_vgb = assign40640_e53770;
        locals.var_vgb_dn5 = locals.var_v_gs_dn5;
        locals.var_vgb_dn6 = (locals.var_v_gs_dn6 + locals.var_v_sb_dn6);
        locals.var_vgb_dn7 = (locals.var_v_gs_dn7 + locals.var_v_sb_dn7);
        locals.var_vgb_dn8 = locals.var_v_sb_dn8;

        locals.var_vgsprime = locals.var_v_gs;
        locals.var_vgsprime_dn5 = locals.var_v_gs_dn5;
        locals.var_vgsprime_dn6 = locals.var_v_gs_dn6;
        locals.var_vgsprime_dn7 = locals.var_v_gs_dn7;

        locals.var_vsbprime = locals.var_v_sb;
        locals.var_vsbprime_dn6 = locals.var_v_sb_dn6;
        locals.var_vsbprime_dn7 = locals.var_v_sb_dn7;
        locals.var_vsbprime_dn8 = locals.var_v_sb_dn8;

        let assign40670_e53775: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_vdbprime = assign40670_e53775;
        locals.var_vdbprime_dn6 = (locals.var_v_ds_dn6 + locals.var_v_sb_dn6);
        locals.var_vdbprime_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_vdbprime_dn8 = locals.var_v_sb_dn8;

        let assign40680_e53778: f64 = (locals.var_v_gs - locals.var_v_ds);
        locals.var_vgdprime = assign40680_e53778;
        locals.var_vgdprime_dn5 = locals.var_v_gs_dn5;
        locals.var_vgdprime_dn6 = (locals.var_v_gs_dn6 - locals.var_v_ds_dn6);
        locals.var_vgdprime_dn7 = (locals.var_v_gs_dn7 - locals.var_v_ds_dn7);

        let assign40690_e53780: f64 = (-locals.var_vgsprime);
        let assign40690_e53782: f64 = (assign40690_e53780 * locals.var_inv_phita);
        locals.var_xgs_ov = assign40690_e53782;
        locals.var_xgs_ov_dn5 = ((-locals.var_vgsprime_dn5) * locals.var_inv_phita);
        locals.var_xgs_ov_dn6 = ((-locals.var_vgsprime_dn6) * locals.var_inv_phita);
        locals.var_xgs_ov_dn7 = ((-locals.var_vgsprime_dn7) * locals.var_inv_phita);

        let assign40700_e53784: f64 = (-locals.var_vgdprime);
        let assign40700_e53786: f64 = (assign40700_e53784 * locals.var_inv_phita);
        locals.var_xgd_ov = assign40700_e53786;
        locals.var_xgd_ov_dn5 = ((-locals.var_vgdprime_dn5) * locals.var_inv_phita);
        locals.var_xgd_ov_dn6 = ((-locals.var_vgdprime_dn6) * locals.var_inv_phita);
        locals.var_xgd_ov_dn7 = ((-locals.var_vgdprime_dn7) * locals.var_inv_phita);

        let assign40710_e53789: f64 = (locals.var_vgb - locals.var_vfb_t);
        let assign40710_e53790: f64 = (-assign40710_e53789);
        let assign40710_e53792: f64 = (assign40710_e53790 * locals.var_inv_phita);
        locals.var_xgb_ov = assign40710_e53792;
        locals.var_xgb_ov_dn5 = ((-locals.var_vgb_dn5) * locals.var_inv_phita);
        locals.var_xgb_ov_dn6 = ((-locals.var_vgb_dn6) * locals.var_inv_phita);
        locals.var_xgb_ov_dn7 = ((-locals.var_vgb_dn7) * locals.var_inv_phita);
        locals.var_xgb_ov_dn8 = ((-locals.var_vgb_dn8) * locals.var_inv_phita);

        locals.var_sigvds = 1.0;

        let assign40730_e53796: f64 = if locals.var_v_ds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1114 = assign40730_e53796;

        let (assign40740_e53801,) = {
    if (locals.var_guard1114 != 0.0) {
        let assign40740_e53799: f64 = (-1.0);
        (assign40740_e53799,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign40740_e53801;

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

        let assign40780_e53821: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_v_db = assign40780_e53821;
        locals.var_v_db_dn6 = (locals.var_v_ds_dn6 + locals.var_v_sb_dn6);
        locals.var_v_db_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_v_db_dn8 = locals.var_v_sb_dn8;

        let assign40790_e53824: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40790_e53827: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40790_e53829: f64 = (assign40790_e53827 + 0.01);
        let assign40790_e53830: f64 = (assign40790_e53829).sqrt();
        let assign40790_e53832: f64 = (assign40790_e53830 + 0.1);
        let assign40790_e53833: f64 = (assign40790_e53824 / assign40790_e53832);
        locals.var_vdsx = assign40790_e53833;
        locals.var_vdsx_dn6 = (((((locals.var_v_ds_dn6 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn6)) * assign40790_e53832) - (assign40790_e53824 * (((locals.var_v_ds_dn6 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn6)) / (2.0 * assign40790_e53830)))) / (assign40790_e53832 * assign40790_e53832));
        locals.var_vdsx_dn7 = (((((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) * assign40790_e53832) - (assign40790_e53824 * (((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) / (2.0 * assign40790_e53830)))) / (assign40790_e53832 * assign40790_e53832));

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

        locals.var_v_xb_dc_tmp = locals.var_v_xb;
        locals.var_v_xb_dc_tmp_dn6 = locals.var_v_xb_dn6;
        locals.var_v_xb_dc_tmp_dn7 = locals.var_v_xb_dn7;
        locals.var_v_xb_dc_tmp_dn8 = locals.var_v_xb_dn8;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign40850_e53881: f64 = if ((p.p45 != 0.0) && (locals.var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1274 = assign40850_e53881;

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

        locals.var_phib = locals.var_phib_dc;

        locals.var_aphi = locals.var_aphi_dc;

        locals.var_g_0 = locals.var_g_0_dc;

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

        locals.var_thesatloc = locals.var_thesat_t;

        locals.var_arloc = locals.var_ar;

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

        let assign41030_e53984: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1275 = assign41030_e53984;

        let (assign41040_e53990,) = {
    if (locals.var_guard1275 != 0.0) {
        let assign41040_e53988: f64 = (locals.var_phib * locals.var_inv_phit);
        (assign41040_e53988,)
    } else {
        (locals.var_xbct,)
    }
};
        locals.var_xbct = assign41040_e53990;

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

    }

    pub(super) fn stamp_transient_block_15(
        locals: &mut StampLocals,
    ) {
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

        let assign41200_e54185: f64 = (-230.25850929940458);
        let assign41200_e54186: f64 = if locals.var_temp2 > assign41200_e54185 { 1.0 } else { 0.0 };
        locals.var_guard1276 = assign41200_e54186;

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

        let assign41330_e54275: f64 = (2.0 * locals.var_vdsx);
        let assign41330_e54280: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign41330_e54281: f64 = (1.0 + assign41330_e54280);
        let assign41330_e54282: f64 = (assign41330_e54281).sqrt();
        let assign41330_e54283: f64 = (1.0 + assign41330_e54282);
        let assign41330_e54284: f64 = (assign41330_e54275 / assign41330_e54283);
        locals.var_vdsp = assign41330_e54284;
        locals.var_vdsp_dn6 = ((((2.0 * locals.var_vdsx_dn6) * assign41330_e54283) - (assign41330_e54275 * ((locals.var_cfd_i * locals.var_vdsx_dn6) / (2.0 * assign41330_e54282)))) / (assign41330_e54283 * assign41330_e54283));
        locals.var_vdsp_dn7 = ((((2.0 * locals.var_vdsx_dn7) * assign41330_e54283) - (assign41330_e54275 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign41330_e54282)))) / (assign41330_e54283 * assign41330_e54283));

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

    }
}
