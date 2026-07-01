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
        let assign00_e1445: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e1445;

        let (assign10_e1450,) = {
    if (locals.var_guard1 != 0.0) {
        let assign10_e1448: f64 = 1.0;
        (assign10_e1448,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign10_e1450;

        let (assign20_e1456,) = {
    if (locals.var_guard1 == 0.0) {
        let assign20_e1454: f64 = (-1.0);
        (assign20_e1454,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign20_e1456;

        let assign30_e1459: f64 = (8.8541878176e-12 * 11.8);
        locals.var_epssi = assign30_e1459;

        let assign40_e1462: f64 = (273.15 + p.p38);
        locals.var_tkr = assign40_e1462;

        let assign2050_e2493: f64 = ctx_temp;
        let assign2050_e2495: f64 = (assign2050_e2493 + p.p55);
        let assign2050_e2497: f64 = (assign2050_e2495 + p.p35);
        locals.var_tka = assign2050_e2497;

        let assign2060_e2500: f64 = (locals.var_tka / locals.var_tkr);
        locals.var_rta = assign2060_e2500;

        let assign2070_e2503: f64 = (locals.var_tka - locals.var_tkr);
        locals.var_delta = assign2070_e2503;

        let assign2080_e2506: f64 = (locals.var_tka * 1.3806505e-23);
        let assign2080_e2508: f64 = (assign2080_e2506 / 1.6021918e-19);
        locals.var_phita = assign2080_e2508;

        let assign2090_e2511: f64 = (1.0 / locals.var_phita);
        locals.var_inv_phita = assign2090_e2511;

        locals.var_tkd = locals.var_tka;

        let assign2110_e2515: f64 = (locals.var_tkd * locals.var_tkd);
        locals.var_tkd_sq = assign2110_e2515;

        let assign2120_e2518: f64 = (locals.var_tkd - locals.var_tkr);
        locals.var_delt = assign2120_e2518;

        let assign2130_e2521: f64 = (locals.var_tkr / locals.var_tkd);
        locals.var_rtn = assign2130_e2521;

        let assign2140_e2523: f64 = (locals.var_rtn).ln();
        locals.var_ln_rtn = assign2140_e2523;

        let assign2150_e2526: f64 = (locals.var_tkd * 1.3806505e-23);
        let assign2150_e2528: f64 = (assign2150_e2526 / 1.6021918e-19);
        locals.var_phit = assign2150_e2528;

        let assign2160_e2531: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign2160_e2531;

        let assign2170_e2535: f64 = (9.025e-5 * locals.var_tkd);
        let assign2170_e2536: f64 = (1.179 - assign2170_e2535);
        let assign2170_e2539: f64 = (3.05e-7 * locals.var_tkd_sq);
        let assign2170_e2540: f64 = (assign2170_e2536 - assign2170_e2539);
        locals.var_eg = assign2170_e2540;

        let assign2180_e2544: f64 = (0.00045 * locals.var_tkd);
        let assign2180_e2545: f64 = (1.045 + assign2180_e2544);
        let assign2180_e2549: f64 = (0.0014 * locals.var_tkd);
        let assign2180_e2550: f64 = (0.523 + assign2180_e2549);
        let assign2180_e2553: f64 = (1.48e-6 * locals.var_tkd_sq);
        let assign2180_e2554: f64 = (assign2180_e2550 - assign2180_e2553);
        let assign2180_e2555: f64 = (assign2180_e2545 * assign2180_e2554);
        let assign2180_e2557: f64 = (assign2180_e2555 * locals.var_tkd_sq);
        let assign2180_e2559: f64 = (assign2180_e2557 / 90000.0);
        locals.var_phibfac = assign2180_e2559;

        let (assign2190_e2565,) = {
    if (locals.var_phibfac > 0.001) {
        (locals.var_phibfac,)
    } else {
        (0.001,)
    }
};
        locals.var_phibfac = assign2190_e2565;

        let assign2200_e2568: f64 = (4.0 * 1.3806505e-23);
        let assign2200_e2570: f64 = (assign2200_e2568 * locals.var_tkd);
        locals.var_nt0 = assign2200_e2570;

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

        let assign3500_e3418: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3500_e3418;

        let (assign3510_e3427,) = {
    if (locals.var_guard29 != 0.0) {
        let (assign3510_e3425,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3510_e3425,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3510_e3427;

        let (assign3520_e3434,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3520_e3431: f64 = (locals.var_nf_i + 0.5);
        let assign3520_e3432: f64 = (assign3520_e3431).floor();
        (assign3520_e3432,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3520_e3434;

        let (assign3530_e3440,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3530_e3438: f64 = (1.0 / locals.var_nf_i);
        (assign3530_e3438,)
    } else {
        (locals.var_invnf,)
    }
};
        locals.var_invnf = assign3530_e3440;

        let assign3540_e3443: f64 = (locals.var_w_i * locals.var_invnf);
        let (assign3540_e3450,) = {
    if (assign3540_e3443 > 1e-9) {
        let assign3540_e3448: f64 = (locals.var_w_i * locals.var_invnf);
        (assign3540_e3448,)
    } else {
        (1e-9,)
    }
};
        locals.var_w_i = assign3540_e3450;

        locals.var_sca_i = p.p5;

        locals.var_scb_i = p.p6;

        locals.var_scc_i = p.p7;

        let assign3590_e3462: f64 = (1e-6 / locals.var_l_i);
        locals.var_il = assign3590_e3462;

        let assign3600_e3465: f64 = (1e-6 / locals.var_w_i);
        locals.var_iw = assign3600_e3465;

        let assign3610_e3470: f64 = (p.p187 * locals.var_il);
        let assign3610_e3471: f64 = (1.0 + assign3610_e3470);
        let assign3610_e3472: f64 = (p.p186 * assign3610_e3471);
        let assign3610_e3476: f64 = (p.p188 * locals.var_iw);
        let assign3610_e3477: f64 = (1.0 + assign3610_e3476);
        let assign3610_e3478: f64 = (assign3610_e3472 * assign3610_e3477);
        locals.var_dellps = assign3610_e3478;

        let assign3620_e3483: f64 = (p.p191 * locals.var_il);
        let assign3620_e3484: f64 = (1.0 + assign3620_e3483);
        let assign3620_e3485: f64 = (p.p190 * assign3620_e3484);
        let assign3620_e3489: f64 = (p.p192 * locals.var_iw);
        let assign3620_e3490: f64 = (1.0 + assign3620_e3489);
        let assign3620_e3491: f64 = (assign3620_e3485 * assign3620_e3490);
        locals.var_delwod = assign3620_e3491;

        let assign3630_e3494: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3630_e3497: f64 = (2.0 * p.p189);
        let assign3630_e3498: f64 = (assign3630_e3494 - assign3630_e3497);
        let (assign3630_e3509,) = {
    if (assign3630_e3498 > 1e-9) {
        let assign3630_e3503: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3630_e3506: f64 = (2.0 * p.p189);
        let assign3630_e3507: f64 = (assign3630_e3503 - assign3630_e3506);
        (assign3630_e3507,)
    } else {
        (1e-9,)
    }
};
        locals.var_le = assign3630_e3509;

        let assign3640_e3512: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3640_e3515: f64 = (2.0 * p.p193);
        let assign3640_e3516: f64 = (assign3640_e3512 - assign3640_e3515);
        let (assign3640_e3527,) = {
    if (assign3640_e3516 > 1e-9) {
        let assign3640_e3521: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3640_e3524: f64 = (2.0 * p.p193);
        let assign3640_e3525: f64 = (assign3640_e3521 - assign3640_e3524);
        (assign3640_e3525,)
    } else {
        (1e-9,)
    }
};
        locals.var_we = assign3640_e3527;

        let assign3650_e3530: f64 = (1e-6 / locals.var_le);
        locals.var_ile = assign3650_e3530;

        let assign3660_e3533: f64 = (locals.var_ile * locals.var_ile);
        locals.var_ile2 = assign3660_e3533;

        let assign3670_e3536: f64 = (1e-6 / locals.var_we);
        locals.var_iwe = assign3670_e3536;

        let assign3680_e3539: f64 = (1.0 / locals.var_iwe);
        locals.var_iiwe = assign3680_e3539;

        let assign3690_e3542: f64 = (locals.var_ile * locals.var_iwe);
        locals.var_iae = assign3690_e3542;

        let assign3700_e3545: f64 = (1.0 / locals.var_iae);
        locals.var_iiae = assign3700_e3545;

        let assign3710_e3548: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3710_e3551: f64 = (2.0 * p.p189);
        let assign3710_e3552: f64 = (assign3710_e3548 - assign3710_e3551);
        let assign3710_e3554: f64 = (assign3710_e3552 + p.p194);
        let (assign3710_e3567,) = {
    if (assign3710_e3554 > 1e-9) {
        let assign3710_e3559: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3710_e3562: f64 = (2.0 * p.p189);
        let assign3710_e3563: f64 = (assign3710_e3559 - assign3710_e3562);
        let assign3710_e3565: f64 = (assign3710_e3563 + p.p194);
        (assign3710_e3565,)
    } else {
        (1e-9,)
    }
};
        locals.var_lecv = assign3710_e3567;

        let assign3720_e3570: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3720_e3573: f64 = (2.0 * p.p193);
        let assign3720_e3574: f64 = (assign3720_e3570 - assign3720_e3573);
        let assign3720_e3576: f64 = (assign3720_e3574 + p.p195);
        let (assign3720_e3589,) = {
    if (assign3720_e3576 > 1e-9) {
        let assign3720_e3581: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3720_e3584: f64 = (2.0 * p.p193);
        let assign3720_e3585: f64 = (assign3720_e3581 - assign3720_e3584);
        let assign3720_e3587: f64 = (assign3720_e3585 + p.p195);
        (assign3720_e3587,)
    } else {
        (1e-9,)
    }
};
        locals.var_wecv = assign3720_e3589;

        let assign3730_e3592: f64 = (locals.var_wecv / 1e-6);
        locals.var_iiwecv = assign3730_e3592;

        let assign3740_e3595: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3740_e3597: f64 = (assign3740_e3595 + p.p194);
        let (assign3740_e3606,) = {
    if (assign3740_e3597 > 1e-9) {
        let assign3740_e3602: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3740_e3604: f64 = (assign3740_e3602 + p.p194);
        (assign3740_e3604,)
    } else {
        (1e-9,)
    }
};
        locals.var_lcv = assign3740_e3606;

        let assign3760_e3623: f64 = (locals.var_lcv / 1e-6);
        locals.var_iilcv = assign3760_e3623;

        locals.var_vfb_p = p.p56;

        locals.var_stvfb_p = p.p57;

        locals.var_st2vfb_p = p.p58;

        locals.var_tox_p = p.p59;

        locals.var_epsrox_p = p.p60;

        locals.var_neff_p = p.p61;

        locals.var_gfacnud_p = p.p62;

        locals.var_vsbnud_p = p.p63;

        locals.var_dvsbnud_p = p.p64;

        locals.var_dphib_p = p.p65;

        locals.var_np_p = p.p66;

        locals.var_toxov_p = p.p67;

        locals.var_toxovd_p = p.p68;

        locals.var_nov_p = p.p69;

        locals.var_novd_p = p.p70;

        locals.var_ct_p = p.p71;

        locals.var_ctg_p = p.p73;

        locals.var_ctb_p = p.p72;

        locals.var_stct_p = p.p74;

        locals.var_psce_p = p.p78;

        locals.var_psced_p = p.p80;

        locals.var_psceb_p = p.p79;

        locals.var_cf_p = p.p75;

        locals.var_cfd_p = p.p77;

        locals.var_cfb_p = p.p76;

        locals.var_betn_p = p.p81;

        locals.var_stbet_p = p.p82;

        locals.var_mue_p = p.p83;

        locals.var_stmue_p = p.p84;

        locals.var_themu_p = p.p85;

        locals.var_stthemu_p = p.p86;

        locals.var_cs_p = p.p87;

        locals.var_stcs_p = p.p88;

        locals.var_thecs_p = p.p89;

        locals.var_stthecs_p = p.p90;

        locals.var_xcor_p = p.p91;

        locals.var_stxcor_p = p.p92;

        locals.var_feta_p = p.p93;

        locals.var_rs_p = p.p94;

        locals.var_strs_p = p.p95;

        locals.var_rsb_p = p.p96;

        locals.var_rsg_p = p.p97;

        locals.var_thesat_p = p.p98;

        locals.var_stthesat_p = p.p99;

        locals.var_thesatb_p = p.p100;

        locals.var_thesatg_p = p.p101;

        locals.var_thesatt_p = p.p102;

        locals.var_ax_p = p.p103;

        locals.var_alp_p = p.p104;

        locals.var_alp1_p = p.p105;

        locals.var_alp2_p = p.p106;

        locals.var_vp_p = p.p107;

        locals.var_a1_p = p.p108;

        locals.var_a2_p = p.p109;

        locals.var_sta2_p = p.p110;

        locals.var_a3_p = p.p111;

        locals.var_a4_p = p.p112;

        locals.var_imaxii_p = p.p113;

        locals.var_gco_p = p.p114;

        locals.var_iginv_p = p.p115;

        locals.var_igov_p = p.p116;

        locals.var_igovd_p = p.p117;

        locals.var_stig_p = p.p118;

        locals.var_gc2_p = p.p119;

        locals.var_gc3_p = p.p120;

        locals.var_gc2ov_p = p.p119;

        let assign4480_e3738: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4480_e3740: f64 = if assign4480_e3738 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign4480_e3740;

        let (assign4490_e3744,) = {
    if (locals.var_guard30 != 0.0) {
        (p.p121,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign4490_e3744;

        locals.var_gc3ov_p = p.p120;

        let assign4510_e3747: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4510_e3749: f64 = if assign4510_e3747 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign4510_e3749;

        let (assign4520_e3753,) = {
    if (locals.var_guard31 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign4520_e3753;

        locals.var_gc2ovd_p = locals.var_gc2ov_p;

        let assign4540_e3756: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4540_e3758: f64 = if assign4540_e3756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign4540_e3758;

        let (assign4550_e3762,) = {
    if (locals.var_guard32 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign4550_e3762;

        locals.var_gc3ovd_p = locals.var_gc3ov_p;

        let assign4570_e3765: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4570_e3767: f64 = if assign4570_e3765 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign4570_e3767;

        let (assign4580_e3771,) = {
    if (locals.var_guard33 != 0.0) {
        (p.p124,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign4580_e3771;

        locals.var_chib_p = p.p125;

        locals.var_agidl_p = p.p126;

        locals.var_agidld_p = p.p127;

        locals.var_bgidl_p = p.p128;

        locals.var_bgidld_p = p.p129;

        locals.var_stbgidl_p = p.p130;

        locals.var_stbgidld_p = p.p131;

        locals.var_cgidl_p = p.p132;

        locals.var_cgidld_p = p.p133;

        locals.var_cox_p = p.p134;

        locals.var_delvtac_p = p.p135;

        locals.var_facneffac_p = p.p136;

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        locals.var_thesatac_p = p.p98;

        let assign4720_e3786: f64 = if param_given[137] { 1.0 } else { 0.0 };
        let assign4720_e3788: f64 = if assign4720_e3786 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign4720_e3788;

        let (assign4730_e3792,) = {
    if (locals.var_guard34 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign4730_e3792;

        locals.var_axac_p = p.p103;

        let assign4750_e3795: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4750_e3797: f64 = if assign4750_e3795 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign4750_e3797;

        let (assign4760_e3801,) = {
    if (locals.var_guard35 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign4760_e3801;

        locals.var_alpac_p = p.p139;

        locals.var_alp1ac_p = p.p140;

        locals.var_cgov_p = p.p141;

        locals.var_cgovd_p = p.p142;

        locals.var_fcgovacc_p = p.p143;

        locals.var_fcgovaccd_p = p.p144;

        locals.var_cgovaccg_p = p.p145;

        locals.var_cgbov_p = p.p146;

        locals.var_cinr_p = p.p147;

        locals.var_cinrd_p = p.p148;

        locals.var_dvfbinr_p = p.p149;

        locals.var_fcinrdep_p = p.p150;

        locals.var_fcinracc_p = p.p151;

        locals.var_axinr_p = p.p152;

        locals.var_fnt_p = p.p155;

        locals.var_fntexc_p = p.p156;

        locals.var_vfbedge_p = p.p161;

        locals.var_stvfbedge_p = p.p162;

        locals.var_dphibedge_p = p.p163;

        locals.var_neffedge_p = p.p164;

        locals.var_ctedge_p = p.p165;

        locals.var_betnedge_p = p.p166;

        locals.var_stbetedge_p = p.p167;

        locals.var_psceedge_p = p.p168;

        locals.var_pscebedge_p = p.p169;

        locals.var_pscededge_p = p.p170;

        locals.var_cfedge_p = p.p171;

        locals.var_cfdedge_p = p.p173;

        locals.var_cfbedge_p = p.p172;

        let assign5240_e3851: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign5240_e3851;

        let (assign5250_e3869,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5250_e3857: f64 = (locals.var_ile).powf(p.p198);
        let assign5250_e3858: f64 = (p.p197 * assign5250_e3857);
        let assign5250_e3859: f64 = (p.p196 + assign5250_e3858);
        let assign5250_e3862: f64 = (p.p199 * locals.var_iwe);
        let assign5250_e3863: f64 = (assign5250_e3859 + assign5250_e3862);
        let assign5250_e3866: f64 = (p.p200 * locals.var_iae);
        let assign5250_e3867: f64 = (assign5250_e3863 + assign5250_e3866);
        (assign5250_e3867,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign5250_e3869;

        let (assign5260_e3885,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5260_e3874: f64 = (p.p202 * locals.var_ile);
        let assign5260_e3875: f64 = (p.p201 + assign5260_e3874);
        let assign5260_e3878: f64 = (p.p203 * locals.var_iwe);
        let assign5260_e3879: f64 = (assign5260_e3875 + assign5260_e3878);
        let assign5260_e3882: f64 = (p.p204 * locals.var_iae);
        let assign5260_e3883: f64 = (assign5260_e3879 + assign5260_e3882);
        (assign5260_e3883,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign5260_e3885;

        let (assign5270_e3889,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p205,)
    } else {
        (locals.var_st2vfb_p,)
    }
};
        locals.var_st2vfb_p = assign5270_e3889;

        let (assign5280_e3893,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p206,)
    } else {
        (locals.var_tox_p,)
    }
};
        locals.var_tox_p = assign5280_e3893;

        let (assign5290_e3897,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p207,)
    } else {
        (locals.var_epsrox_p,)
    }
};
        locals.var_epsrox_p = assign5290_e3897;

        let (assign5300_e3930,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5300_e3903: f64 = (p.p209 * locals.var_iwe);
        let assign5300_e3907: f64 = (locals.var_we / p.p210);
        let assign5300_e3908: f64 = (1.0 + assign5300_e3907);
        let assign5300_e3909: f64 = (assign5300_e3908).ln();
        let assign5300_e3910: f64 = (assign5300_e3903 * assign5300_e3909);
        let assign5300_e3911: f64 = (1.0 + assign5300_e3910);
        let (assign5300_e3927,) = {
            if (assign5300_e3911 > 0.001) {
                let assign5300_e3917: f64 = (p.p209 * locals.var_iwe);
                let assign5300_e3921: f64 = (locals.var_we / p.p210);
                let assign5300_e3922: f64 = (1.0 + assign5300_e3921);
                let assign5300_e3923: f64 = (assign5300_e3922).ln();
                let assign5300_e3924: f64 = (assign5300_e3917 * assign5300_e3923);
                let assign5300_e3925: f64 = (1.0 + assign5300_e3924);
                (assign5300_e3925,)
            } else {
                (0.001,)
            }
        };
        let assign5300_e3928: f64 = (p.p208 * assign5300_e3927);
        (assign5300_e3928,)
    } else {
        (locals.var_nsub0e,)
    }
};
        locals.var_nsub0e = assign5300_e3930;

        let (assign5310_e3963,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5310_e3936: f64 = (p.p212 * locals.var_iwe);
        let assign5310_e3940: f64 = (locals.var_we / p.p213);
        let assign5310_e3941: f64 = (1.0 + assign5310_e3940);
        let assign5310_e3942: f64 = (assign5310_e3941).ln();
        let assign5310_e3943: f64 = (assign5310_e3936 * assign5310_e3942);
        let assign5310_e3944: f64 = (1.0 + assign5310_e3943);
        let (assign5310_e3960,) = {
            if (assign5310_e3944 > 0.001) {
                let assign5310_e3950: f64 = (p.p212 * locals.var_iwe);
                let assign5310_e3954: f64 = (locals.var_we / p.p213);
                let assign5310_e3955: f64 = (1.0 + assign5310_e3954);
                let assign5310_e3956: f64 = (assign5310_e3955).ln();
                let assign5310_e3957: f64 = (assign5310_e3950 * assign5310_e3956);
                let assign5310_e3958: f64 = (1.0 + assign5310_e3957);
                (assign5310_e3958,)
            } else {
                (0.001,)
            }
        };
        let assign5310_e3961: f64 = (p.p211 * assign5310_e3960);
        (assign5310_e3961,)
    } else {
        (locals.var_npcke,)
    }
};
        locals.var_npcke = assign5310_e3963;

        let (assign5320_e3996,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5320_e3969: f64 = (p.p215 * locals.var_iwe);
        let assign5320_e3973: f64 = (locals.var_we / p.p213);
        let assign5320_e3974: f64 = (1.0 + assign5320_e3973);
        let assign5320_e3975: f64 = (assign5320_e3974).ln();
        let assign5320_e3976: f64 = (assign5320_e3969 * assign5320_e3975);
        let assign5320_e3977: f64 = (1.0 + assign5320_e3976);
        let (assign5320_e3993,) = {
            if (assign5320_e3977 > 0.001) {
                let assign5320_e3983: f64 = (p.p215 * locals.var_iwe);
                let assign5320_e3987: f64 = (locals.var_we / p.p213);
                let assign5320_e3988: f64 = (1.0 + assign5320_e3987);
                let assign5320_e3989: f64 = (assign5320_e3988).ln();
                let assign5320_e3990: f64 = (assign5320_e3983 * assign5320_e3989);
                let assign5320_e3991: f64 = (1.0 + assign5320_e3990);
                (assign5320_e3991,)
            } else {
                (0.001,)
            }
        };
        let assign5320_e3994: f64 = (p.p214 * assign5320_e3993);
        (assign5320_e3994,)
    } else {
        (locals.var_lpcke,)
    }
};
        locals.var_lpcke = assign5320_e3996;

        let assign5330_e4000: f64 = (2.0 * locals.var_lpcke);
        let assign5330_e4001: f64 = if locals.var_le > assign5330_e4000 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign5330_e4001;

        let (assign5340_e4007,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        (75000000000.0,)
    } else {
        (locals.var_aa,)
    }
};
        locals.var_aa = assign5340_e4007;

        let (assign5350_e4021,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5350_e4014: f64 = (0.5 * locals.var_npcke);
        let assign5350_e4015: f64 = (locals.var_nsub0e + assign5350_e4014);
        let assign5350_e4016: f64 = (assign5350_e4015).sqrt();
        let assign5350_e4018: f64 = (locals.var_nsub0e).sqrt();
        let assign5350_e4019: f64 = (assign5350_e4016 - assign5350_e4018);
        (assign5350_e4019,)
    } else {
        (locals.var_bb,)
    }
};
        locals.var_bb = assign5350_e4021;

        let (assign5360_e4046,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5360_e4026: f64 = (locals.var_nsub0e).sqrt();
        let assign5360_e4031: f64 = (2.0 * locals.var_lpcke);
        let assign5360_e4033: f64 = (assign5360_e4031 / locals.var_le);
        let assign5360_e4036: f64 = (locals.var_bb / locals.var_aa);
        let assign5360_e4037: f64 = (assign5360_e4036).exp();
        let assign5360_e4039: f64 = (assign5360_e4037 - 1.0);
        let assign5360_e4040: f64 = (assign5360_e4033 * assign5360_e4039);
        let assign5360_e4041: f64 = (1.0 + assign5360_e4040);
        let assign5360_e4042: f64 = (assign5360_e4041).ln();
        let assign5360_e4043: f64 = (locals.var_aa * assign5360_e4042);
        let assign5360_e4044: f64 = (assign5360_e4026 + assign5360_e4043);
        (assign5360_e4044,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5360_e4046;

        let (assign5370_e4054,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5370_e4052: f64 = (locals.var_nsub * locals.var_nsub);
        (assign5370_e4052,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5370_e4054;

        let assign5380_e4057: f64 = if locals.var_le >= locals.var_lpcke { 1.0 } else { 0.0 };
        locals.var_guard38 = assign5380_e4057;

        let (assign5390_e4072,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 != 0.0)) {
        let assign5390_e4067: f64 = (locals.var_npcke * locals.var_lpcke);
        let assign5390_e4069: f64 = (assign5390_e4067 / locals.var_le);
        let assign5390_e4070: f64 = (locals.var_nsub0e + assign5390_e4069);
        (assign5390_e4070,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5390_e4072;

        let (assign5400_e4090,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 == 0.0)) {
        let assign5400_e4085: f64 = (locals.var_le / locals.var_lpcke);
        let assign5400_e4086: f64 = (2.0 - assign5400_e4085);
        let assign5400_e4087: f64 = (locals.var_npcke * assign5400_e4086);
        let assign5400_e4088: f64 = (locals.var_nsub0e + assign5400_e4087);
        (assign5400_e4088,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5400_e4090;

        let (assign5410_e4104,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5410_e4096: f64 = (p.p216 * locals.var_ile);
        let assign5410_e4097: f64 = (1.0 - assign5410_e4096);
        let assign5410_e4100: f64 = (p.p217 * locals.var_ile2);
        let assign5410_e4101: f64 = (assign5410_e4097 - assign5410_e4100);
        let assign5410_e4102: f64 = (locals.var_nsub * assign5410_e4101);
        (assign5410_e4102,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign5410_e4104;

        let (assign5420_e4122,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5420_e4110: f64 = (locals.var_ile).powf(p.p220);
        let assign5420_e4111: f64 = (p.p219 * assign5420_e4110);
        let assign5420_e4112: f64 = (p.p218 + assign5420_e4111);
        let assign5420_e4115: f64 = (p.p221 * locals.var_iwe);
        let assign5420_e4116: f64 = (assign5420_e4112 + assign5420_e4115);
        let assign5420_e4119: f64 = (p.p222 * locals.var_iae);
        let assign5420_e4120: f64 = (assign5420_e4116 + assign5420_e4119);
        (assign5420_e4120,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign5420_e4122;

        let (assign5430_e4126,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p223,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign5430_e4126;

        let (assign5440_e4130,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p224,)
    } else {
        (locals.var_dvsbnud_p,)
    }
};
        locals.var_dvsbnud_p = assign5440_e4130;

        let (assign5450_e4148,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5450_e4136: f64 = (locals.var_ile).powf(p.p227);
        let assign5450_e4137: f64 = (p.p226 * assign5450_e4136);
        let assign5450_e4138: f64 = (p.p225 + assign5450_e4137);
        let assign5450_e4141: f64 = (p.p228 * locals.var_iwe);
        let assign5450_e4142: f64 = (assign5450_e4138 + assign5450_e4141);
        let assign5450_e4145: f64 = (p.p229 * locals.var_iae);
        let assign5450_e4146: f64 = (assign5450_e4142 + assign5450_e4145);
        (assign5450_e4146,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign5450_e4148;

        let (assign5460_e4167,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5460_e4155: f64 = (p.p231 * locals.var_ile);
        let assign5460_e4156: f64 = (1.0 + assign5460_e4155);
        let (assign5460_e4164,) = {
            if (1e-6 > assign5460_e4156) {
                (1e-6,)
            } else {
                let assign5460_e4162: f64 = (p.p231 * locals.var_ile);
                let assign5460_e4163: f64 = (1.0 + assign5460_e4162);
                (assign5460_e4163,)
            }
        };
        let assign5460_e4165: f64 = (p.p230 * assign5460_e4164);
        (assign5460_e4165,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign5460_e4167;

        let (assign5470_e4171,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p232,)
    } else {
        (locals.var_toxov_p,)
    }
};
        locals.var_toxov_p = assign5470_e4171;

        let (assign5480_e4175,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p233,)
    } else {
        (locals.var_toxovd_p,)
    }
};
        locals.var_toxovd_p = assign5480_e4175;

        let (assign5490_e4179,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p236,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign5490_e4179;

        let (assign5500_e4183,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p237,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign5500_e4183;

        let (assign5510_e4205,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5510_e4189: f64 = (locals.var_ile).powf(p.p240);
        let assign5510_e4190: f64 = (p.p239 * assign5510_e4189);
        let assign5510_e4191: f64 = (p.p238 + assign5510_e4190);
        let assign5510_e4195: f64 = (p.p241 * locals.var_iwe);
        let assign5510_e4196: f64 = (1.0 + assign5510_e4195);
        let assign5510_e4197: f64 = (assign5510_e4191 * assign5510_e4196);
        let assign5510_e4201: f64 = (p.p242 * locals.var_iae);
        let assign5510_e4202: f64 = (1.0 + assign5510_e4201);
        let assign5510_e4203: f64 = (assign5510_e4197 * assign5510_e4202);
        (assign5510_e4203,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign5510_e4205;

        let (assign5520_e4209,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p244,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign5520_e4209;

        let (assign5530_e4213,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p243,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign5530_e4213;

        let (assign5540_e4217,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p245,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign5540_e4217;

        let (assign5550_e4231,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5550_e4222: f64 = (locals.var_ile).powf(p.p247);
        let assign5550_e4223: f64 = (p.p246 * assign5550_e4222);
        let assign5550_e4227: f64 = (p.p248 * locals.var_iwe);
        let assign5550_e4228: f64 = (1.0 + assign5550_e4227);
        let assign5550_e4229: f64 = (assign5550_e4223 * assign5550_e4228);
        (assign5550_e4229,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign5550_e4231;

        let (assign5560_e4235,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p250,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign5560_e4235;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5570_e4239,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p249,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign5570_e4239;

        let (assign5580_e4253,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5580_e4244: f64 = (locals.var_ile).powf(p.p252);
        let assign5580_e4245: f64 = (p.p251 * assign5580_e4244);
        let assign5580_e4249: f64 = (p.p253 * locals.var_iwe);
        let assign5580_e4250: f64 = (1.0 + assign5580_e4249);
        let assign5580_e4251: f64 = (assign5580_e4245 * assign5580_e4250);
        (assign5580_e4251,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign5580_e4253;

        let (assign5590_e4257,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p255,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign5590_e4257;

        let (assign5600_e4261,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p254,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign5600_e4261;

        let (assign5610_e4271,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5610_e4267: f64 = (p.p258 * locals.var_iwe);
        let assign5610_e4268: f64 = (1.0 + assign5610_e4267);
        let assign5610_e4269: f64 = (p.p257 * assign5610_e4268);
        (assign5610_e4269,)
    } else {
        (locals.var_fbet1e,)
    }
};
        locals.var_fbet1e = assign5610_e4271;

        let (assign5620_e4290,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5620_e4277: f64 = (p.p260 * locals.var_iwe);
        let assign5620_e4278: f64 = (1.0 + assign5620_e4277);
        let (assign5620_e4287,) = {
            if (assign5620_e4278 > 0.001) {
                let assign5620_e4284: f64 = (p.p260 * locals.var_iwe);
                let assign5620_e4285: f64 = (1.0 + assign5620_e4284);
                (assign5620_e4285,)
            } else {
                (0.001,)
            }
        };
        let assign5620_e4288: f64 = (p.p259 * assign5620_e4287);
        (assign5620_e4288,)
    } else {
        (locals.var_lp1e,)
    }
};
        locals.var_lp1e = assign5620_e4290;

        let (assign5630_e4322,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5630_e4295: f64 = (locals.var_fbet1e * locals.var_lp1e);
        let assign5630_e4297: f64 = (assign5630_e4295 / locals.var_le);
        let assign5630_e4300: f64 = (-locals.var_le);
        let assign5630_e4302: f64 = (assign5630_e4300 / locals.var_lp1e);
        let assign5630_e4303: f64 = (assign5630_e4302).exp();
        let assign5630_e4304: f64 = (1.0 - assign5630_e4303);
        let assign5630_e4305: f64 = (assign5630_e4297 * assign5630_e4304);
        let assign5630_e4306: f64 = (1.0 + assign5630_e4305);
        let assign5630_e4309: f64 = (p.p261 * p.p262);
        let assign5630_e4311: f64 = (assign5630_e4309 / locals.var_le);
        let assign5630_e4314: f64 = (-locals.var_le);
        let assign5630_e4316: f64 = (assign5630_e4314 / p.p262);
        let assign5630_e4317: f64 = (assign5630_e4316).exp();
        let assign5630_e4318: f64 = (1.0 - assign5630_e4317);
        let assign5630_e4319: f64 = (assign5630_e4311 * assign5630_e4318);
        let assign5630_e4320: f64 = (assign5630_e4306 + assign5630_e4319);
        (assign5630_e4320,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5630_e4322;

        let (assign5640_e4331,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign5640_e4329,) = {
            if (locals.var_gpe > 1e-15) {
                (locals.var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5640_e4329,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5640_e4331;

        let (assign5650_e4350,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5650_e4336: f64 = (p.p263 * locals.var_iwe);
        let assign5650_e4337: f64 = (1.0 + assign5650_e4336);
        let assign5650_e4340: f64 = (p.p264 * locals.var_iwe);
        let assign5650_e4344: f64 = (locals.var_we / p.p265);
        let assign5650_e4345: f64 = (1.0 + assign5650_e4344);
        let assign5650_e4346: f64 = (assign5650_e4345).ln();
        let assign5650_e4347: f64 = (assign5650_e4340 * assign5650_e4346);
        let assign5650_e4348: f64 = (assign5650_e4337 + assign5650_e4347);
        (assign5650_e4348,)
    } else {
        (locals.var_gwe,)
    }
};
        locals.var_gwe = assign5650_e4350;

        let (assign5660_e4362,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5660_e4354: f64 = (p.p256 * locals.var_we);
        let assign5660_e4357: f64 = (locals.var_gpe * locals.var_le);
        let assign5660_e4358: f64 = (assign5660_e4354 / assign5660_e4357);
        let assign5660_e4360: f64 = (assign5660_e4358 * locals.var_gwe);
        (assign5660_e4360,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign5660_e4362;

        let (assign5670_e4378,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5670_e4367: f64 = (p.p267 * locals.var_ile);
        let assign5670_e4368: f64 = (p.p266 + assign5670_e4367);
        let assign5670_e4371: f64 = (p.p268 * locals.var_iwe);
        let assign5670_e4372: f64 = (assign5670_e4368 + assign5670_e4371);
        let assign5670_e4375: f64 = (p.p269 * locals.var_iae);
        let assign5670_e4376: f64 = (assign5670_e4372 + assign5670_e4375);
        (assign5670_e4376,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign5670_e4378;

        let (assign5680_e4388,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5680_e4384: f64 = (p.p271 * locals.var_iwe);
        let assign5680_e4385: f64 = (1.0 + assign5680_e4384);
        let assign5680_e4386: f64 = (p.p270 * assign5680_e4385);
        (assign5680_e4386,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign5680_e4388;

        let (assign5690_e4392,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p272,)
    } else {
        (locals.var_stmue_p,)
    }
};
        locals.var_stmue_p = assign5690_e4392;

        let (assign5700_e4396,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p273,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign5700_e4396;

        let (assign5710_e4400,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p274,)
    } else {
        (locals.var_stthemu_p,)
    }
};
        locals.var_stthemu_p = assign5710_e4400;

        let (assign5720_e4422,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5720_e4406: f64 = (locals.var_ile).powf(p.p277);
        let assign5720_e4407: f64 = (p.p276 * assign5720_e4406);
        let assign5720_e4408: f64 = (p.p275 + assign5720_e4407);
        let assign5720_e4412: f64 = (p.p278 * locals.var_iwe);
        let assign5720_e4413: f64 = (1.0 + assign5720_e4412);
        let assign5720_e4414: f64 = (assign5720_e4408 * assign5720_e4413);
        let assign5720_e4418: f64 = (p.p279 * locals.var_iae);
        let assign5720_e4419: f64 = (1.0 + assign5720_e4418);
        let assign5720_e4420: f64 = (assign5720_e4414 * assign5720_e4419);
        (assign5720_e4420,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign5720_e4422;

        let (assign5730_e4426,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p280,)
    } else {
        (locals.var_stcs_p,)
    }
};
        locals.var_stcs_p = assign5730_e4426;

        let (assign5740_e4430,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p281,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign5740_e4430;

        let (assign5750_e4434,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p282,)
    } else {
        (locals.var_stthecs_p,)
    }
};
        locals.var_stthecs_p = assign5750_e4434;

        let (assign5760_e4456,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5760_e4440: f64 = (p.p284 * locals.var_ile);
        let assign5760_e4441: f64 = (1.0 + assign5760_e4440);
        let assign5760_e4442: f64 = (p.p283 * assign5760_e4441);
        let assign5760_e4446: f64 = (p.p285 * locals.var_iwe);
        let assign5760_e4447: f64 = (1.0 + assign5760_e4446);
        let assign5760_e4448: f64 = (assign5760_e4442 * assign5760_e4447);
        let assign5760_e4452: f64 = (p.p286 * locals.var_iae);
        let assign5760_e4453: f64 = (1.0 + assign5760_e4452);
        let assign5760_e4454: f64 = (assign5760_e4448 * assign5760_e4453);
        (assign5760_e4454,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign5760_e4456;

        let (assign5770_e4460,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p287,)
    } else {
        (locals.var_stxcor_p,)
    }
};
        locals.var_stxcor_p = assign5770_e4460;

        let (assign5780_e4464,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p288,)
    } else {
        (locals.var_feta_p,)
    }
};
        locals.var_feta_p = assign5780_e4464;

        let (assign5790_e4476,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5790_e4468: f64 = (p.p289 * locals.var_iwe);
        let assign5790_e4472: f64 = (p.p290 * locals.var_iwe);
        let assign5790_e4473: f64 = (1.0 + assign5790_e4472);
        let assign5790_e4474: f64 = (assign5790_e4468 * assign5790_e4473);
        (assign5790_e4474,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign5790_e4476;

        let (assign5800_e4480,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p291,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign5800_e4480;

        let (assign5810_e4484,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p292,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign5810_e4484;

        let (assign5820_e4488,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p293,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign5820_e4488;

        let (assign5830_e4514,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5830_e4493: f64 = (p.p295 * locals.var_gwe);
        let assign5830_e4495: f64 = (assign5830_e4493 / locals.var_gpe);
        let assign5830_e4498: f64 = (locals.var_ile).powf(p.p296);
        let assign5830_e4499: f64 = (assign5830_e4495 * assign5830_e4498);
        let assign5830_e4500: f64 = (p.p294 + assign5830_e4499);
        let assign5830_e4504: f64 = (p.p297 * locals.var_iwe);
        let assign5830_e4505: f64 = (1.0 + assign5830_e4504);
        let assign5830_e4506: f64 = (assign5830_e4500 * assign5830_e4505);
        let assign5830_e4510: f64 = (p.p298 * locals.var_iae);
        let assign5830_e4511: f64 = (1.0 + assign5830_e4510);
        let assign5830_e4512: f64 = (assign5830_e4506 * assign5830_e4511);
        (assign5830_e4512,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign5830_e4514;

        let (assign5840_e4530,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5840_e4519: f64 = (p.p300 * locals.var_ile);
        let assign5840_e4520: f64 = (p.p299 + assign5840_e4519);
        let assign5840_e4523: f64 = (p.p301 * locals.var_iwe);
        let assign5840_e4524: f64 = (assign5840_e4520 + assign5840_e4523);
        let assign5840_e4527: f64 = (p.p302 * locals.var_iae);
        let assign5840_e4528: f64 = (assign5840_e4524 + assign5840_e4527);
        (assign5840_e4528,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign5840_e4530;

        let (assign5850_e4534,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p303,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign5850_e4534;

        let (assign5860_e4538,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p304,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign5860_e4538;

        let (assign5870_e4542,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p305,)
    } else {
        (locals.var_thesatt_p,)
    }
};
        locals.var_thesatt_p = assign5870_e4542;

        let (assign5880_e4552,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5880_e4548: f64 = (p.p307 * locals.var_ile);
        let assign5880_e4549: f64 = (1.0 + assign5880_e4548);
        let assign5880_e4550: f64 = (p.p306 / assign5880_e4549);
        (assign5880_e4550,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign5880_e4552;

        let (assign5890_e4566,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5890_e4557: f64 = (locals.var_ile).powf(p.p309);
        let assign5890_e4558: f64 = (p.p308 * assign5890_e4557);
        let assign5890_e4562: f64 = (p.p310 * locals.var_iwe);
        let assign5890_e4563: f64 = (1.0 + assign5890_e4562);
        let assign5890_e4564: f64 = (assign5890_e4558 * assign5890_e4563);
        (assign5890_e4564,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign5890_e4566;

        let (assign5900_e4572,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5900_e4570: f64 = (locals.var_ile).powf(p.p312);
        (assign5900_e4570,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign5900_e4572;

        let (assign5910_e4592,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5910_e4576: f64 = (p.p311 * locals.var_tmpx);
        let assign5910_e4580: f64 = (p.p314 * locals.var_iwe);
        let assign5910_e4581: f64 = (1.0 + assign5910_e4580);
        let assign5910_e4582: f64 = (assign5910_e4576 * assign5910_e4581);
        let assign5910_e4586: f64 = (p.p313 * locals.var_ile);
        let assign5910_e4588: f64 = (assign5910_e4586 * locals.var_tmpx);
        let assign5910_e4589: f64 = (1.0 + assign5910_e4588);
        let assign5910_e4590: f64 = (assign5910_e4582 / assign5910_e4589);
        (assign5910_e4590,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign5910_e4592;

        let (assign5920_e4598,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5920_e4596: f64 = (locals.var_ile).powf(p.p316);
        (assign5920_e4596,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign5920_e4598;

        let (assign5930_e4618,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5930_e4602: f64 = (p.p315 * locals.var_tmpx);
        let assign5930_e4606: f64 = (p.p318 * locals.var_iwe);
        let assign5930_e4607: f64 = (1.0 + assign5930_e4606);
        let assign5930_e4608: f64 = (assign5930_e4602 * assign5930_e4607);
        let assign5930_e4612: f64 = (p.p317 * locals.var_ile);
        let assign5930_e4614: f64 = (assign5930_e4612 * locals.var_tmpx);
        let assign5930_e4615: f64 = (1.0 + assign5930_e4614);
        let assign5930_e4616: f64 = (assign5930_e4608 / assign5930_e4615);
        (assign5930_e4616,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign5930_e4618;

        let (assign5940_e4622,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p319,)
    } else {
        (locals.var_vp_p,)
    }
};
        locals.var_vp_p = assign5940_e4622;

        let (assign5950_e4638,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5950_e4628: f64 = (p.p321 * locals.var_ile);
        let assign5950_e4629: f64 = (1.0 + assign5950_e4628);
        let assign5950_e4630: f64 = (p.p320 * assign5950_e4629);
        let assign5950_e4634: f64 = (p.p322 * locals.var_iwe);
        let assign5950_e4635: f64 = (1.0 + assign5950_e4634);
        let assign5950_e4636: f64 = (assign5950_e4630 * assign5950_e4635);
        (assign5950_e4636,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign5950_e4638;

        let (assign5960_e4642,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p323,)
    } else {
        (locals.var_a2_p,)
    }
};
        locals.var_a2_p = assign5960_e4642;

        let (assign5970_e4646,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p324,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign5970_e4646;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5980_e4662,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5980_e4652: f64 = (p.p326 * locals.var_ile);
        let assign5980_e4653: f64 = (1.0 + assign5980_e4652);
        let assign5980_e4654: f64 = (p.p325 * assign5980_e4653);
        let assign5980_e4658: f64 = (p.p327 * locals.var_iwe);
        let assign5980_e4659: f64 = (1.0 + assign5980_e4658);
        let assign5980_e4660: f64 = (assign5980_e4654 * assign5980_e4659);
        (assign5980_e4660,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign5980_e4662;

        let (assign5990_e4678,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5990_e4668: f64 = (p.p329 * locals.var_ile);
        let assign5990_e4669: f64 = (1.0 + assign5990_e4668);
        let assign5990_e4670: f64 = (p.p328 * assign5990_e4669);
        let assign5990_e4674: f64 = (p.p330 * locals.var_iwe);
        let assign5990_e4675: f64 = (1.0 + assign5990_e4674);
        let assign5990_e4676: f64 = (assign5990_e4670 * assign5990_e4675);
        (assign5990_e4676,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign5990_e4678;

        let (assign6000_e4682,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p331,)
    } else {
        (locals.var_imaxii_p,)
    }
};
        locals.var_imaxii_p = assign6000_e4682;

        let (assign6010_e4686,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p332,)
    } else {
        (locals.var_gco_p,)
    }
};
        locals.var_gco_p = assign6010_e4686;

        let (assign6020_e4692,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6020_e4690: f64 = (p.p333 / locals.var_iae);
        (assign6020_e4690,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign6020_e4692;

        let (assign6030_e4702,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6030_e4696: f64 = (p.p334 * p.p234);
        let assign6030_e4699: f64 = (1e-6 * locals.var_iwe);
        let assign6030_e4700: f64 = (assign6030_e4696 / assign6030_e4699);
        (assign6030_e4700,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign6030_e4702;

        let (assign6040_e4712,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6040_e4706: f64 = (p.p335 * p.p235);
        let assign6040_e4709: f64 = (1e-6 * locals.var_iwe);
        let assign6040_e4710: f64 = (assign6040_e4706 / assign6040_e4709);
        (assign6040_e4710,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign6040_e4712;

        let (assign6050_e4716,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p336,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign6050_e4716;

        let (assign6060_e4720,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p337,)
    } else {
        (locals.var_gc2_p,)
    }
};
        locals.var_gc2_p = assign6060_e4720;

        let (assign6070_e4724,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p338,)
    } else {
        (locals.var_gc3_p,)
    }
};
        locals.var_gc3_p = assign6070_e4724;

        let (assign6080_e4728,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p337,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6080_e4728;

        let assign6090_e4730: f64 = if param_given[339] { 1.0 } else { 0.0 };
        let assign6090_e4732: f64 = if assign6090_e4730 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign6090_e4732;

        let (assign6100_e4738,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard39 != 0.0)) {
        (p.p339,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6100_e4738;

        let (assign6110_e4742,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p338,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6110_e4742;

        let assign6120_e4744: f64 = if param_given[340] { 1.0 } else { 0.0 };
        let assign6120_e4746: f64 = if assign6120_e4744 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign6120_e4746;

        let (assign6130_e4752,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard40 != 0.0)) {
        (p.p340,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6130_e4752;

        let (assign6140_e4756,) = {
    if (locals.var_guard36 != 0.0) {
        (locals.var_gc2ov_p,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6140_e4756;

        let assign6150_e4758: f64 = if param_given[341] { 1.0 } else { 0.0 };
        let assign6150_e4760: f64 = if assign6150_e4758 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign6150_e4760;

        let (assign6160_e4766,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard41 != 0.0)) {
        (p.p341,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6160_e4766;

        let (assign6170_e4770,) = {
    if (locals.var_guard36 != 0.0) {
        (locals.var_gc3ov_p,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6170_e4770;

        let assign6180_e4772: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6180_e4774: f64 = if assign6180_e4772 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign6180_e4774;

        let (assign6190_e4780,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard42 != 0.0)) {
        (p.p342,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6190_e4780;

        let (assign6200_e4784,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p343,)
    } else {
        (locals.var_chib_p,)
    }
};
        locals.var_chib_p = assign6200_e4784;

        let (assign6210_e4794,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6210_e4788: f64 = (p.p344 * p.p234);
        let assign6210_e4791: f64 = (1e-6 * locals.var_iwe);
        let assign6210_e4792: f64 = (assign6210_e4788 / assign6210_e4791);
        (assign6210_e4792,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign6210_e4794;

        let (assign6220_e4804,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6220_e4798: f64 = (p.p345 * p.p235);
        let assign6220_e4801: f64 = (1e-6 * locals.var_iwe);
        let assign6220_e4802: f64 = (assign6220_e4798 / assign6220_e4801);
        (assign6220_e4802,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign6220_e4804;

        let (assign6230_e4808,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p346,)
    } else {
        (locals.var_bgidl_p,)
    }
};
        locals.var_bgidl_p = assign6230_e4808;

        let (assign6240_e4812,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p347,)
    } else {
        (locals.var_bgidld_p,)
    }
};
        locals.var_bgidld_p = assign6240_e4812;

        let (assign6250_e4816,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p348,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign6250_e4816;

        let (assign6260_e4820,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p349,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign6260_e4820;

        let (assign6270_e4824,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p350,)
    } else {
        (locals.var_cgidl_p,)
    }
};
        locals.var_cgidl_p = assign6270_e4824;

        let (assign6280_e4828,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p351,)
    } else {
        (locals.var_cgidld_p,)
    }
};
        locals.var_cgidld_p = assign6280_e4828;

        let (assign6290_e4840,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6290_e4832: f64 = (8.8541878176e-12 * p.p207);
        let assign6290_e4834: f64 = (assign6290_e4832 * locals.var_wecv);
        let assign6290_e4836: f64 = (assign6290_e4834 * locals.var_lecv);
        let assign6290_e4838: f64 = (assign6290_e4836 / p.p206);
        (assign6290_e4838,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign6290_e4840;

        let (assign6300_e4852,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6300_e4844: f64 = (8.8541878176e-12 * p.p207);
        let assign6300_e4846: f64 = (assign6300_e4844 * locals.var_wecv);
        let assign6300_e4848: f64 = (assign6300_e4846 * p.p234);
        let assign6300_e4850: f64 = (assign6300_e4848 / p.p232);
        (assign6300_e4850,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign6300_e4852;

        let (assign6310_e4864,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6310_e4856: f64 = (8.8541878176e-12 * p.p207);
        let assign6310_e4858: f64 = (assign6310_e4856 * locals.var_wecv);
        let assign6310_e4860: f64 = (assign6310_e4858 * p.p235);
        let assign6310_e4862: f64 = (assign6310_e4860 / p.p233);
        (assign6310_e4862,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign6310_e4864;

        let (assign6320_e4882,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6320_e4870: f64 = (locals.var_ile).powf(p.p354);
        let assign6320_e4871: f64 = (p.p353 * assign6320_e4870);
        let assign6320_e4872: f64 = (p.p352 + assign6320_e4871);
        let assign6320_e4875: f64 = (p.p355 * locals.var_iwe);
        let assign6320_e4876: f64 = (assign6320_e4872 + assign6320_e4875);
        let assign6320_e4879: f64 = (p.p356 * locals.var_iae);
        let assign6320_e4880: f64 = (assign6320_e4876 + assign6320_e4879);
        (assign6320_e4880,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign6320_e4882;

        let (assign6330_e4898,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6330_e4887: f64 = (p.p358 * locals.var_ile);
        let assign6330_e4888: f64 = (p.p357 + assign6330_e4887);
        let assign6330_e4891: f64 = (p.p359 * locals.var_iwe);
        let assign6330_e4892: f64 = (assign6330_e4888 + assign6330_e4891);
        let assign6330_e4895: f64 = (p.p360 * locals.var_iae);
        let assign6330_e4896: f64 = (assign6330_e4892 + assign6330_e4895);
        (assign6330_e4896,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign6330_e4898;

        let (assign6340_e4902,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p294,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6340_e4902;

        let assign6350_e4904: f64 = if param_given[361] { 1.0 } else { 0.0 };
        let assign6350_e4906: f64 = if assign6350_e4904 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign6350_e4906;

        let (assign6360_e4912,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard43 != 0.0)) {
        (p.p361,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6360_e4912;

        let (assign6370_e4916,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p295,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6370_e4916;

        let assign6380_e4918: f64 = if param_given[362] { 1.0 } else { 0.0 };
        let assign6380_e4920: f64 = if assign6380_e4918 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign6380_e4920;

        let (assign6390_e4926,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard44 != 0.0)) {
        (p.p362,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6390_e4926;

        let (assign6400_e4930,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p296,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6400_e4930;

        let assign6410_e4932: f64 = if param_given[363] { 1.0 } else { 0.0 };
        let assign6410_e4934: f64 = if assign6410_e4932 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign6410_e4934;

        let (assign6420_e4940,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard45 != 0.0)) {
        (p.p363,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6420_e4940;

        let (assign6430_e4944,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p297,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6430_e4944;

        let assign6440_e4946: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6440_e4948: f64 = if assign6440_e4946 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign6440_e4948;

        let (assign6450_e4954,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard46 != 0.0)) {
        (p.p364,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6450_e4954;

        let (assign6460_e4958,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p298,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6460_e4958;

        let assign6470_e4960: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6470_e4962: f64 = if assign6470_e4960 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign6470_e4962;

        let (assign6480_e4968,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard47 != 0.0)) {
        (p.p365,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6480_e4968;

        let (assign6490_e4994,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6490_e4973: f64 = (locals.var_thesatacl_i * locals.var_gwe);
        let assign6490_e4975: f64 = (assign6490_e4973 / locals.var_gpe);
        let assign6490_e4978: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
        let assign6490_e4979: f64 = (assign6490_e4975 * assign6490_e4978);
        let assign6490_e4980: f64 = (locals.var_thesataco_i + assign6490_e4979);
        let assign6490_e4984: f64 = (locals.var_thesatacw_i * locals.var_iwe);
        let assign6490_e4985: f64 = (1.0 + assign6490_e4984);
        let assign6490_e4986: f64 = (assign6490_e4980 * assign6490_e4985);
        let assign6490_e4990: f64 = (locals.var_thesataclw_i * locals.var_iae);
        let assign6490_e4991: f64 = (1.0 + assign6490_e4990);
        let assign6490_e4992: f64 = (assign6490_e4986 * assign6490_e4991);
        (assign6490_e4992,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign6490_e4994;

        let (assign6500_e4998,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p306,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6500_e4998;

        let assign6510_e5000: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6510_e5002: f64 = if assign6510_e5000 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign6510_e5002;

        let (assign6520_e5008,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard48 != 0.0)) {
        (p.p366,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6520_e5008;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6530_e5012,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p307,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6530_e5012;

        let assign6540_e5014: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6540_e5016: f64 = if assign6540_e5014 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign6540_e5016;

        let (assign6550_e5022,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard49 != 0.0)) {
        (p.p367,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6550_e5022;

        let (assign6560_e5032,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6560_e5028: f64 = (locals.var_axacl_i * locals.var_ile);
        let assign6560_e5029: f64 = (1.0 + assign6560_e5028);
        let assign6560_e5030: f64 = (locals.var_axaco_i / assign6560_e5029);
        (assign6560_e5030,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign6560_e5032;

        let (assign6570_e5046,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6570_e5037: f64 = (locals.var_ile).powf(p.p369);
        let assign6570_e5038: f64 = (p.p368 * assign6570_e5037);
        let assign6570_e5042: f64 = (p.p370 * locals.var_iwe);
        let assign6570_e5043: f64 = (1.0 + assign6570_e5042);
        let assign6570_e5044: f64 = (assign6570_e5038 * assign6570_e5043);
        (assign6570_e5044,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign6570_e5046;

        let (assign6580_e5052,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6580_e5050: f64 = (locals.var_ile).powf(p.p372);
        (assign6580_e5050,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6580_e5052;

        let (assign6590_e5072,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6590_e5056: f64 = (p.p371 * locals.var_tmpx);
        let assign6590_e5060: f64 = (p.p374 * locals.var_iwe);
        let assign6590_e5061: f64 = (1.0 + assign6590_e5060);
        let assign6590_e5062: f64 = (assign6590_e5056 * assign6590_e5061);
        let assign6590_e5066: f64 = (p.p373 * locals.var_ile);
        let assign6590_e5068: f64 = (assign6590_e5066 * locals.var_tmpx);
        let assign6590_e5069: f64 = (1.0 + assign6590_e5068);
        let assign6590_e5070: f64 = (assign6590_e5062 / assign6590_e5069);
        (assign6590_e5070,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign6590_e5072;

        let (assign6600_e5076,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p375,)
    } else {
        (locals.var_fcgovacc_p,)
    }
};
        locals.var_fcgovacc_p = assign6600_e5076;

        let (assign6610_e5080,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p376,)
    } else {
        (locals.var_fcgovaccd_p,)
    }
};
        locals.var_fcgovaccd_p = assign6610_e5080;

        let (assign6620_e5084,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p377,)
    } else {
        (locals.var_cgovaccg_p,)
    }
};
        locals.var_cgovaccg_p = assign6620_e5084;

        let (assign6630_e5090,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6630_e5088: f64 = (p.p378 * locals.var_iilcv);
        (assign6630_e5088,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign6630_e5090;

        let (assign6640_e5096,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6640_e5094: f64 = (p.p379 * locals.var_iiwecv);
        (assign6640_e5094,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign6640_e5096;

        let (assign6650_e5102,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6650_e5100: f64 = (p.p380 * locals.var_iiwecv);
        (assign6650_e5100,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign6650_e5102;

        let (assign6660_e5106,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p381,)
    } else {
        (locals.var_dvfbinr_p,)
    }
};
        locals.var_dvfbinr_p = assign6660_e5106;

        let (assign6670_e5110,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p382,)
    } else {
        (locals.var_fcinrdep_p,)
    }
};
        locals.var_fcinrdep_p = assign6670_e5110;

        let (assign6680_e5114,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p383,)
    } else {
        (locals.var_fcinracc_p,)
    }
};
        locals.var_fcinracc_p = assign6680_e5114;

        let (assign6690_e5118,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p384,)
    } else {
        (locals.var_axinr_p,)
    }
};
        locals.var_axinr_p = assign6690_e5118;

        let (assign6720_e5140,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6720_e5135: f64 = (2.0 * p.p393);
        let assign6720_e5137: f64 = (assign6720_e5135 / locals.var_le);
        let assign6720_e5138: f64 = (1.0 - assign6720_e5137);
        (assign6720_e5138,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign6720_e5140;

        let (assign6750_e5161,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p387,)
    } else {
        (locals.var_fnt_p,)
    }
};
        locals.var_fnt_p = assign6750_e5161;

        let (assign6760_e5173,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6760_e5165: f64 = (p.p388 * locals.var_betn_p);
        let assign6760_e5167: f64 = (assign6760_e5165 * locals.var_betn_p);
        let assign6760_e5169: f64 = (assign6760_e5167 * locals.var_iwe);
        let assign6760_e5171: f64 = (assign6760_e5169 * locals.var_iwe);
        (assign6760_e5171,)
    } else {
        (locals.var_fntexc_p,)
    }
};
        locals.var_fntexc_p = assign6760_e5173;

        let (assign6810_e5211,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6810_e5205: f64 = (2.0 * p.p395);
        let assign6810_e5208: f64 = (p.p396 * locals.var_we);
        let assign6810_e5209: f64 = (assign6810_e5205 + assign6810_e5208);
        (assign6810_e5209,)
    } else {
        (locals.var_we_edge,)
    }
};
        locals.var_we_edge = assign6810_e5211;

        let (assign6840_e5227,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p397,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign6840_e5227;

        let (assign6850_e5243,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6850_e5232: f64 = (p.p399 * locals.var_ile);
        let assign6850_e5233: f64 = (p.p398 + assign6850_e5232);
        let assign6850_e5236: f64 = (p.p400 * locals.var_iwe);
        let assign6850_e5237: f64 = (assign6850_e5233 + assign6850_e5236);
        let assign6850_e5240: f64 = (p.p401 * locals.var_iae);
        let assign6850_e5241: f64 = (assign6850_e5237 + assign6850_e5240);
        (assign6850_e5241,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign6850_e5243;

        let (assign6860_e5261,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6860_e5249: f64 = (locals.var_ile).powf(p.p404);
        let assign6860_e5250: f64 = (p.p403 * assign6860_e5249);
        let assign6860_e5251: f64 = (p.p402 + assign6860_e5250);
        let assign6860_e5254: f64 = (p.p405 * locals.var_iwe);
        let assign6860_e5255: f64 = (assign6860_e5251 + assign6860_e5254);
        let assign6860_e5258: f64 = (p.p406 * locals.var_iae);
        let assign6860_e5259: f64 = (assign6860_e5255 + assign6860_e5258);
        (assign6860_e5259,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign6860_e5261;

        let (assign6870_e5285,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6870_e5268: f64 = (locals.var_ile).powf(p.p409);
        let assign6870_e5269: f64 = (p.p408 * assign6870_e5268);
        let assign6870_e5270: f64 = (1.0 + assign6870_e5269);
        let assign6870_e5271: f64 = (p.p407 * assign6870_e5270);
        let assign6870_e5275: f64 = (p.p410 * locals.var_iwe);
        let assign6870_e5276: f64 = (1.0 + assign6870_e5275);
        let assign6870_e5277: f64 = (assign6870_e5271 * assign6870_e5276);
        let assign6870_e5281: f64 = (p.p411 * locals.var_iae);
        let assign6870_e5282: f64 = (1.0 + assign6870_e5281);
        let assign6870_e5283: f64 = (assign6870_e5277 * assign6870_e5282);
        (assign6870_e5283,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign6870_e5285;

        let (assign6880_e5295,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6880_e5291: f64 = (locals.var_ile).powf(p.p414);
        let assign6880_e5292: f64 = (p.p413 * assign6880_e5291);
        let assign6880_e5293: f64 = (p.p412 + assign6880_e5292);
        (assign6880_e5293,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign6880_e5295;

        let (assign6890_e5313,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6890_e5300: f64 = (p.p415 * p.p416);
        let assign6890_e5302: f64 = (assign6890_e5300 / locals.var_le);
        let assign6890_e5305: f64 = (-locals.var_le);
        let assign6890_e5307: f64 = (assign6890_e5305 / p.p416);
        let assign6890_e5308: f64 = (assign6890_e5307).exp();
        let assign6890_e5309: f64 = (1.0 - assign6890_e5308);
        let assign6890_e5310: f64 = (assign6890_e5302 * assign6890_e5309);
        let assign6890_e5311: f64 = (1.0 + assign6890_e5310);
        (assign6890_e5311,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign6890_e5313;

        let (assign6900_e5322,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign6900_e5320,) = {
            if (locals.var_gpe_edge > 1e-15) {
                (locals.var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign6900_e5320,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign6900_e5322;

        let (assign6910_e5338,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6910_e5326: f64 = (p.p256 * locals.var_we_edge);
        let assign6910_e5329: f64 = (locals.var_gpe_edge * locals.var_le);
        let assign6910_e5330: f64 = (assign6910_e5326 / assign6910_e5329);
        let assign6910_e5334: f64 = (p.p417 * locals.var_iwe);
        let assign6910_e5335: f64 = (1.0 + assign6910_e5334);
        let assign6910_e5336: f64 = (assign6910_e5330 * assign6910_e5335);
        (assign6910_e5336,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign6910_e5338;

        let (assign6920_e5354,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6920_e5343: f64 = (p.p419 * locals.var_ile);
        let assign6920_e5344: f64 = (p.p418 + assign6920_e5343);
        let assign6920_e5347: f64 = (p.p420 * locals.var_iwe);
        let assign6920_e5348: f64 = (assign6920_e5344 + assign6920_e5347);
        let assign6920_e5351: f64 = (p.p421 * locals.var_iae);
        let assign6920_e5352: f64 = (assign6920_e5348 + assign6920_e5351);
        (assign6920_e5352,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign6920_e5354;

        let (assign6930_e5368,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6930_e5359: f64 = (locals.var_ile).powf(p.p423);
        let assign6930_e5360: f64 = (p.p422 * assign6930_e5359);
        let assign6930_e5364: f64 = (p.p424 * locals.var_iwe);
        let assign6930_e5365: f64 = (1.0 + assign6930_e5364);
        let assign6930_e5366: f64 = (assign6930_e5360 * assign6930_e5365);
        (assign6930_e5366,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign6930_e5368;

        let (assign6940_e5372,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p425,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign6940_e5372;

        let (assign6950_e5376,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p426,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign6950_e5376;

        let (assign6960_e5390,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6960_e5381: f64 = (locals.var_ile).powf(p.p428);
        let assign6960_e5382: f64 = (p.p427 * assign6960_e5381);
        let assign6960_e5386: f64 = (p.p429 * locals.var_iwe);
        let assign6960_e5387: f64 = (1.0 + assign6960_e5386);
        let assign6960_e5388: f64 = (assign6960_e5382 * assign6960_e5387);
        (assign6960_e5388,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign6960_e5390;

        let (assign6970_e5394,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p431,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign6970_e5394;

        let (assign6980_e5398,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p430,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign6980_e5398;

        let (assign7040_e5440,) = {
    if (locals.var_guard36 != 0.0) {
        let assign7040_e5429: f64 = (p.p808 * locals.var_ile);
        let assign7040_e5430: f64 = (p.p807 + assign7040_e5429);
        let assign7040_e5433: f64 = (p.p809 * locals.var_iwe);
        let assign7040_e5434: f64 = (assign7040_e5430 + assign7040_e5433);
        let assign7040_e5437: f64 = (p.p810 * locals.var_iae);
        let assign7040_e5438: f64 = (assign7040_e5434 + assign7040_e5437);
        (assign7040_e5438,)
    } else {
        (locals.var_kvthowe,)
    }
};
        locals.var_kvthowe = assign7040_e5440;

        let (assign7050_e5456,) = {
    if (locals.var_guard36 != 0.0) {
        let assign7050_e5445: f64 = (p.p812 * locals.var_ile);
        let assign7050_e5446: f64 = (p.p811 + assign7050_e5445);
        let assign7050_e5449: f64 = (p.p813 * locals.var_iwe);
        let assign7050_e5450: f64 = (assign7050_e5446 + assign7050_e5449);
        let assign7050_e5453: f64 = (p.p814 * locals.var_iae);
        let assign7050_e5454: f64 = (assign7050_e5450 + assign7050_e5453);
        (assign7050_e5454,)
    } else {
        (locals.var_kuowe,)
    }
};
        locals.var_kuowe = assign7050_e5456;

        let assign7170_e5570: f64 = if (((param_given[448] || param_given[449]) || param_given[450]) || param_given[451]) { 1.0 } else { 0.0 };
        locals.var_guard51 = assign7170_e5570;

        let (assign7180_e5588,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign7180_e5577: f64 = (p.p449 * locals.var_ile);
        let assign7180_e5578: f64 = (p.p448 + assign7180_e5577);
        let assign7180_e5581: f64 = (p.p450 * locals.var_iwe);
        let assign7180_e5582: f64 = (assign7180_e5578 + assign7180_e5581);
        let assign7180_e5585: f64 = (p.p451 * locals.var_iae);
        let assign7180_e5586: f64 = (assign7180_e5582 + assign7180_e5585);
        (assign7180_e5586,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign7180_e5588;

        let assign7190_e5607: f64 = if (((param_given[452] || param_given[453]) || param_given[454]) || param_given[455]) { 1.0 } else { 0.0 };
        locals.var_guard52 = assign7190_e5607;

        let (assign7200_e5625,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard52 != 0.0)) {
        let assign7200_e5614: f64 = (p.p453 * locals.var_ile);
        let assign7200_e5615: f64 = (p.p452 + assign7200_e5614);
        let assign7200_e5618: f64 = (p.p454 * locals.var_iwe);
        let assign7200_e5619: f64 = (assign7200_e5615 + assign7200_e5618);
        let assign7200_e5622: f64 = (p.p455 * locals.var_iae);
        let assign7200_e5623: f64 = (assign7200_e5619 + assign7200_e5622);
        (assign7200_e5623,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign7200_e5625;

        let assign7210_e5644: f64 = if (((param_given[456] || param_given[457]) || param_given[458]) || param_given[459]) { 1.0 } else { 0.0 };
        locals.var_guard53 = assign7210_e5644;

        let (assign7220_e5662,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign7220_e5651: f64 = (p.p457 * locals.var_ile);
        let assign7220_e5652: f64 = (p.p456 + assign7220_e5651);
        let assign7220_e5655: f64 = (p.p458 * locals.var_iwe);
        let assign7220_e5656: f64 = (assign7220_e5652 + assign7220_e5655);
        let assign7220_e5659: f64 = (p.p459 * locals.var_iae);
        let assign7220_e5660: f64 = (assign7220_e5656 + assign7220_e5659);
        (assign7220_e5660,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign7220_e5662;

        let assign7230_e5681: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        locals.var_guard54 = assign7230_e5681;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7240_e5699,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard54 != 0.0)) {
        let assign7240_e5688: f64 = (p.p461 * locals.var_ile);
        let assign7240_e5689: f64 = (p.p460 + assign7240_e5688);
        let assign7240_e5692: f64 = (p.p462 * locals.var_iwe);
        let assign7240_e5693: f64 = (assign7240_e5689 + assign7240_e5692);
        let assign7240_e5696: f64 = (p.p463 * locals.var_iae);
        let assign7240_e5697: f64 = (assign7240_e5693 + assign7240_e5696);
        (assign7240_e5697,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign7240_e5699;

        let assign7250_e5718: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        locals.var_guard55 = assign7250_e5718;

        let (assign7260_e5736,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign7260_e5725: f64 = (p.p465 * locals.var_ile);
        let assign7260_e5726: f64 = (p.p464 + assign7260_e5725);
        let assign7260_e5729: f64 = (p.p466 * locals.var_iwe);
        let assign7260_e5730: f64 = (assign7260_e5726 + assign7260_e5729);
        let assign7260_e5733: f64 = (p.p467 * locals.var_iae);
        let assign7260_e5734: f64 = (assign7260_e5730 + assign7260_e5733);
        (assign7260_e5734,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign7260_e5736;

        let assign7270_e5755: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        locals.var_guard56 = assign7270_e5755;

        let (assign7280_e5773,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign7280_e5762: f64 = (p.p469 * locals.var_ile);
        let assign7280_e5763: f64 = (p.p468 + assign7280_e5762);
        let assign7280_e5766: f64 = (p.p470 * locals.var_iwe);
        let assign7280_e5767: f64 = (assign7280_e5763 + assign7280_e5766);
        let assign7280_e5770: f64 = (p.p471 * locals.var_iae);
        let assign7280_e5771: f64 = (assign7280_e5767 + assign7280_e5770);
        (assign7280_e5771,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign7280_e5773;

        let assign7290_e5792: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign7290_e5792;

        let (assign7300_e5810,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign7300_e5799: f64 = (p.p473 * locals.var_ile);
        let assign7300_e5800: f64 = (p.p472 + assign7300_e5799);
        let assign7300_e5803: f64 = (p.p474 * locals.var_iwe);
        let assign7300_e5804: f64 = (assign7300_e5800 + assign7300_e5803);
        let assign7300_e5807: f64 = (p.p475 * locals.var_iae);
        let assign7300_e5808: f64 = (assign7300_e5804 + assign7300_e5807);
        (assign7300_e5808,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign7300_e5810;

        let assign7310_e5829: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign7310_e5829;

        let (assign7320_e5847,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard58 != 0.0)) {
        let assign7320_e5836: f64 = (p.p477 * locals.var_ile);
        let assign7320_e5837: f64 = (p.p476 + assign7320_e5836);
        let assign7320_e5840: f64 = (p.p478 * locals.var_iwe);
        let assign7320_e5841: f64 = (assign7320_e5837 + assign7320_e5840);
        let assign7320_e5844: f64 = (p.p479 * locals.var_iae);
        let assign7320_e5845: f64 = (assign7320_e5841 + assign7320_e5844);
        (assign7320_e5845,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign7320_e5847;

        let assign7330_e5866: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign7330_e5866;

        let (assign7340_e5884,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard59 != 0.0)) {
        let assign7340_e5873: f64 = (p.p481 * locals.var_ile);
        let assign7340_e5874: f64 = (p.p480 + assign7340_e5873);
        let assign7340_e5877: f64 = (p.p482 * locals.var_iwe);
        let assign7340_e5878: f64 = (assign7340_e5874 + assign7340_e5877);
        let assign7340_e5881: f64 = (p.p483 * locals.var_iae);
        let assign7340_e5882: f64 = (assign7340_e5878 + assign7340_e5881);
        (assign7340_e5882,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign7340_e5884;

        let assign7350_e5903: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        locals.var_guard60 = assign7350_e5903;

        let (assign7360_e5921,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign7360_e5910: f64 = (p.p485 * locals.var_ile);
        let assign7360_e5911: f64 = (p.p484 + assign7360_e5910);
        let assign7360_e5914: f64 = (p.p486 * locals.var_iwe);
        let assign7360_e5915: f64 = (assign7360_e5911 + assign7360_e5914);
        let assign7360_e5918: f64 = (p.p487 * locals.var_iae);
        let assign7360_e5919: f64 = (assign7360_e5915 + assign7360_e5918);
        (assign7360_e5919,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign7360_e5921;

        let assign7370_e5940: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        locals.var_guard61 = assign7370_e5940;

        let (assign7380_e5958,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard61 != 0.0)) {
        let assign7380_e5947: f64 = (p.p493 * locals.var_ile);
        let assign7380_e5948: f64 = (p.p492 + assign7380_e5947);
        let assign7380_e5951: f64 = (p.p494 * locals.var_iwe);
        let assign7380_e5952: f64 = (assign7380_e5948 + assign7380_e5951);
        let assign7380_e5955: f64 = (p.p495 * locals.var_iae);
        let assign7380_e5956: f64 = (assign7380_e5952 + assign7380_e5955);
        (assign7380_e5956,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign7380_e5958;

        let assign7390_e5977: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        locals.var_guard62 = assign7390_e5977;

        let (assign7400_e5995,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign7400_e5984: f64 = (p.p489 * locals.var_ile);
        let assign7400_e5985: f64 = (p.p488 + assign7400_e5984);
        let assign7400_e5988: f64 = (p.p490 * locals.var_iwe);
        let assign7400_e5989: f64 = (assign7400_e5985 + assign7400_e5988);
        let assign7400_e5992: f64 = (p.p491 * locals.var_iae);
        let assign7400_e5993: f64 = (assign7400_e5989 + assign7400_e5992);
        (assign7400_e5993,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign7400_e5995;

        let assign7410_e6014: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        locals.var_guard63 = assign7410_e6014;

        let (assign7420_e6032,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard63 != 0.0)) {
        let assign7420_e6021: f64 = (p.p497 * locals.var_ile);
        let assign7420_e6022: f64 = (p.p496 + assign7420_e6021);
        let assign7420_e6025: f64 = (p.p498 * locals.var_iwe);
        let assign7420_e6026: f64 = (assign7420_e6022 + assign7420_e6025);
        let assign7420_e6029: f64 = (p.p499 * locals.var_iae);
        let assign7420_e6030: f64 = (assign7420_e6026 + assign7420_e6029);
        (assign7420_e6030,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign7420_e6032;

        let assign7430_e6051: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        locals.var_guard64 = assign7430_e6051;

        let (assign7440_e6071,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard64 != 0.0)) {
        let assign7440_e6059: f64 = (p.p501 * locals.var_ile);
        let assign7440_e6060: f64 = (p.p500 + assign7440_e6059);
        let assign7440_e6063: f64 = (p.p502 * locals.var_iwe);
        let assign7440_e6064: f64 = (assign7440_e6060 + assign7440_e6063);
        let assign7440_e6067: f64 = (p.p503 * locals.var_iae);
        let assign7440_e6068: f64 = (assign7440_e6064 + assign7440_e6067);
        let assign7440_e6069: f64 = (locals.var_ile2 * assign7440_e6068);
        (assign7440_e6069,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign7440_e6071;

        let assign7450_e6090: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        locals.var_guard65 = assign7450_e6090;

        let (assign7460_e6108,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard65 != 0.0)) {
        let assign7460_e6097: f64 = (p.p509 * locals.var_ile);
        let assign7460_e6098: f64 = (p.p508 + assign7460_e6097);
        let assign7460_e6101: f64 = (p.p510 * locals.var_iwe);
        let assign7460_e6102: f64 = (assign7460_e6098 + assign7460_e6101);
        let assign7460_e6105: f64 = (p.p511 * locals.var_iae);
        let assign7460_e6106: f64 = (assign7460_e6102 + assign7460_e6105);
        (assign7460_e6106,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign7460_e6108;

        let assign7470_e6127: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        locals.var_guard66 = assign7470_e6127;

        let (assign7480_e6145,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard66 != 0.0)) {
        let assign7480_e6134: f64 = (p.p505 * locals.var_ile);
        let assign7480_e6135: f64 = (p.p504 + assign7480_e6134);
        let assign7480_e6138: f64 = (p.p506 * locals.var_iwe);
        let assign7480_e6139: f64 = (assign7480_e6135 + assign7480_e6138);
        let assign7480_e6142: f64 = (p.p507 * locals.var_iae);
        let assign7480_e6143: f64 = (assign7480_e6139 + assign7480_e6142);
        (assign7480_e6143,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign7480_e6145;

        let assign7490_e6164: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7490_e6164;

        let (assign7500_e6184,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard67 != 0.0)) {
        let assign7500_e6172: f64 = (p.p513 * locals.var_ile);
        let assign7500_e6173: f64 = (p.p512 + assign7500_e6172);
        let assign7500_e6176: f64 = (p.p514 * locals.var_iwe);
        let assign7500_e6177: f64 = (assign7500_e6173 + assign7500_e6176);
        let assign7500_e6180: f64 = (p.p515 * locals.var_iae);
        let assign7500_e6181: f64 = (assign7500_e6177 + assign7500_e6180);
        let assign7500_e6182: f64 = (locals.var_ile2 * assign7500_e6181);
        (assign7500_e6182,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign7500_e6184;

        let assign7510_e6203: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7510_e6203;

        let (assign7520_e6221,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard68 != 0.0)) {
        let assign7520_e6210: f64 = (p.p521 * locals.var_ile);
        let assign7520_e6211: f64 = (p.p520 + assign7520_e6210);
        let assign7520_e6214: f64 = (p.p522 * locals.var_iwe);
        let assign7520_e6215: f64 = (assign7520_e6211 + assign7520_e6214);
        let assign7520_e6218: f64 = (p.p523 * locals.var_iae);
        let assign7520_e6219: f64 = (assign7520_e6215 + assign7520_e6218);
        (assign7520_e6219,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign7520_e6221;

        let assign7530_e6240: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7530_e6240;

        let (assign7540_e6258,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign7540_e6247: f64 = (p.p517 * locals.var_ile);
        let assign7540_e6248: f64 = (p.p516 + assign7540_e6247);
        let assign7540_e6251: f64 = (p.p518 * locals.var_iwe);
        let assign7540_e6252: f64 = (assign7540_e6248 + assign7540_e6251);
        let assign7540_e6255: f64 = (p.p519 * locals.var_iae);
        let assign7540_e6256: f64 = (assign7540_e6252 + assign7540_e6255);
        (assign7540_e6256,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign7540_e6258;

        let assign7550_e6277: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7550_e6277;

        let (assign7560_e6299,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard70 != 0.0)) {
        let assign7560_e6283: f64 = (locals.var_we / locals.var_le);
        let assign7560_e6287: f64 = (p.p525 * locals.var_ile);
        let assign7560_e6288: f64 = (p.p524 + assign7560_e6287);
        let assign7560_e6291: f64 = (p.p526 * locals.var_iwe);
        let assign7560_e6292: f64 = (assign7560_e6288 + assign7560_e6291);
        let assign7560_e6295: f64 = (p.p527 * locals.var_iae);
        let assign7560_e6296: f64 = (assign7560_e6292 + assign7560_e6295);
        let assign7560_e6297: f64 = (assign7560_e6283 * assign7560_e6296);
        (assign7560_e6297,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign7560_e6299;

        let assign7570_e6318: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7570_e6318;

        let (assign7580_e6336,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard71 != 0.0)) {
        let assign7580_e6325: f64 = (p.p529 * locals.var_ile);
        let assign7580_e6326: f64 = (p.p528 + assign7580_e6325);
        let assign7580_e6329: f64 = (p.p530 * locals.var_iwe);
        let assign7580_e6330: f64 = (assign7580_e6326 + assign7580_e6329);
        let assign7580_e6333: f64 = (p.p531 * locals.var_iae);
        let assign7580_e6334: f64 = (assign7580_e6330 + assign7580_e6333);
        (assign7580_e6334,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign7580_e6336;

        let assign7590_e6355: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7590_e6355;

        let (assign7600_e6373,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard72 != 0.0)) {
        let assign7600_e6362: f64 = (p.p533 * locals.var_ile);
        let assign7600_e6363: f64 = (p.p532 + assign7600_e6362);
        let assign7600_e6366: f64 = (p.p534 * locals.var_iwe);
        let assign7600_e6367: f64 = (assign7600_e6363 + assign7600_e6366);
        let assign7600_e6370: f64 = (p.p535 * locals.var_iae);
        let assign7600_e6371: f64 = (assign7600_e6367 + assign7600_e6370);
        (assign7600_e6371,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign7600_e6373;

        let assign7610_e6392: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7610_e6392;

        let (assign7620_e6410,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign7620_e6399: f64 = (p.p537 * locals.var_ile);
        let assign7620_e6400: f64 = (p.p536 + assign7620_e6399);
        let assign7620_e6403: f64 = (p.p538 * locals.var_iwe);
        let assign7620_e6404: f64 = (assign7620_e6400 + assign7620_e6403);
        let assign7620_e6407: f64 = (p.p539 * locals.var_iae);
        let assign7620_e6408: f64 = (assign7620_e6404 + assign7620_e6407);
        (assign7620_e6408,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign7620_e6410;

        let assign7630_e6429: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7630_e6429;

        let (assign7640_e6447,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard74 != 0.0)) {
        let assign7640_e6436: f64 = (p.p541 * locals.var_ile);
        let assign7640_e6437: f64 = (p.p540 + assign7640_e6436);
        let assign7640_e6440: f64 = (p.p542 * locals.var_iwe);
        let assign7640_e6441: f64 = (assign7640_e6437 + assign7640_e6440);
        let assign7640_e6444: f64 = (p.p543 * locals.var_iae);
        let assign7640_e6445: f64 = (assign7640_e6441 + assign7640_e6444);
        (assign7640_e6445,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign7640_e6447;

        let assign7650_e6466: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7650_e6466;

        let (assign7660_e6484,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign7660_e6473: f64 = (p.p545 * locals.var_ile);
        let assign7660_e6474: f64 = (p.p544 + assign7660_e6473);
        let assign7660_e6477: f64 = (p.p546 * locals.var_iwe);
        let assign7660_e6478: f64 = (assign7660_e6474 + assign7660_e6477);
        let assign7660_e6481: f64 = (p.p547 * locals.var_iae);
        let assign7660_e6482: f64 = (assign7660_e6478 + assign7660_e6481);
        (assign7660_e6482,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign7660_e6484;

        let assign7670_e6503: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7670_e6503;

        let (assign7680_e6521,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign7680_e6510: f64 = (p.p549 * locals.var_ile);
        let assign7680_e6511: f64 = (p.p548 + assign7680_e6510);
        let assign7680_e6514: f64 = (p.p550 * locals.var_iwe);
        let assign7680_e6515: f64 = (assign7680_e6511 + assign7680_e6514);
        let assign7680_e6518: f64 = (p.p551 * locals.var_iae);
        let assign7680_e6519: f64 = (assign7680_e6515 + assign7680_e6518);
        (assign7680_e6519,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign7680_e6521;

        let assign7690_e6540: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign7690_e6540;

        let (assign7700_e6560,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard77 != 0.0)) {
        let assign7700_e6548: f64 = (p.p553 * locals.var_ile);
        let assign7700_e6549: f64 = (p.p552 + assign7700_e6548);
        let assign7700_e6552: f64 = (p.p554 * locals.var_iwe);
        let assign7700_e6553: f64 = (assign7700_e6549 + assign7700_e6552);
        let assign7700_e6556: f64 = (p.p555 * locals.var_iae);
        let assign7700_e6557: f64 = (assign7700_e6553 + assign7700_e6556);
        let assign7700_e6558: f64 = (locals.var_iwe * assign7700_e6557);
        (assign7700_e6558,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign7700_e6560;

        let assign7710_e6579: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign7710_e6579;

        let (assign7720_e6597,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign7720_e6586: f64 = (p.p557 * locals.var_ile);
        let assign7720_e6587: f64 = (p.p556 + assign7720_e6586);
        let assign7720_e6590: f64 = (p.p558 * locals.var_iwe);
        let assign7720_e6591: f64 = (assign7720_e6587 + assign7720_e6590);
        let assign7720_e6594: f64 = (p.p559 * locals.var_iae);
        let assign7720_e6595: f64 = (assign7720_e6591 + assign7720_e6594);
        (assign7720_e6595,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign7720_e6597;

        let assign7730_e6616: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign7730_e6616;

        let (assign7740_e6634,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard79 != 0.0)) {
        let assign7740_e6623: f64 = (p.p561 * locals.var_ile);
        let assign7740_e6624: f64 = (p.p560 + assign7740_e6623);
        let assign7740_e6627: f64 = (p.p562 * locals.var_iwe);
        let assign7740_e6628: f64 = (assign7740_e6624 + assign7740_e6627);
        let assign7740_e6631: f64 = (p.p563 * locals.var_iae);
        let assign7740_e6632: f64 = (assign7740_e6628 + assign7740_e6631);
        (assign7740_e6632,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign7740_e6634;

        let assign7750_e6653: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        locals.var_guard80 = assign7750_e6653;

        let (assign7760_e6671,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard80 != 0.0)) {
        let assign7760_e6660: f64 = (p.p565 * locals.var_ile);
        let assign7760_e6661: f64 = (p.p564 + assign7760_e6660);
        let assign7760_e6664: f64 = (p.p566 * locals.var_iwe);
        let assign7760_e6665: f64 = (assign7760_e6661 + assign7760_e6664);
        let assign7760_e6668: f64 = (p.p567 * locals.var_iae);
        let assign7760_e6669: f64 = (assign7760_e6665 + assign7760_e6668);
        (assign7760_e6669,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign7760_e6671;

        let assign7770_e6690: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7770_e6690;

        let (assign7780_e6710,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard81 != 0.0)) {
        let assign7780_e6698: f64 = (p.p569 * locals.var_ile);
        let assign7780_e6699: f64 = (p.p568 + assign7780_e6698);
        let assign7780_e6702: f64 = (p.p570 * locals.var_iwe);
        let assign7780_e6703: f64 = (assign7780_e6699 + assign7780_e6702);
        let assign7780_e6706: f64 = (p.p571 * locals.var_iae);
        let assign7780_e6707: f64 = (assign7780_e6703 + assign7780_e6706);
        let assign7780_e6708: f64 = (locals.var_ile * assign7780_e6707);
        (assign7780_e6708,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign7780_e6710;

        let assign7790_e6729: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7790_e6729;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7800_e6747,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard82 != 0.0)) {
        let assign7800_e6736: f64 = (p.p573 * locals.var_ile);
        let assign7800_e6737: f64 = (p.p572 + assign7800_e6736);
        let assign7800_e6740: f64 = (p.p574 * locals.var_iwe);
        let assign7800_e6741: f64 = (assign7800_e6737 + assign7800_e6740);
        let assign7800_e6744: f64 = (p.p575 * locals.var_iae);
        let assign7800_e6745: f64 = (assign7800_e6741 + assign7800_e6744);
        (assign7800_e6745,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign7800_e6747;

        let assign7810_e6766: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7810_e6766;

        let (assign7820_e6784,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard83 != 0.0)) {
        let assign7820_e6773: f64 = (p.p577 * locals.var_ile);
        let assign7820_e6774: f64 = (p.p576 + assign7820_e6773);
        let assign7820_e6777: f64 = (p.p578 * locals.var_iwe);
        let assign7820_e6778: f64 = (assign7820_e6774 + assign7820_e6777);
        let assign7820_e6781: f64 = (p.p579 * locals.var_iae);
        let assign7820_e6782: f64 = (assign7820_e6778 + assign7820_e6781);
        (assign7820_e6782,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign7820_e6784;

        let assign7830_e6803: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7830_e6803;

        let (assign7840_e6821,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign7840_e6810: f64 = (p.p581 * locals.var_ile);
        let assign7840_e6811: f64 = (p.p580 + assign7840_e6810);
        let assign7840_e6814: f64 = (p.p582 * locals.var_iwe);
        let assign7840_e6815: f64 = (assign7840_e6811 + assign7840_e6814);
        let assign7840_e6818: f64 = (p.p583 * locals.var_iae);
        let assign7840_e6819: f64 = (assign7840_e6815 + assign7840_e6818);
        (assign7840_e6819,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign7840_e6821;

        let assign7850_e6840: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7850_e6840;

        let (assign7860_e6858,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign7860_e6847: f64 = (p.p585 * locals.var_ile);
        let assign7860_e6848: f64 = (p.p584 + assign7860_e6847);
        let assign7860_e6851: f64 = (p.p586 * locals.var_iwe);
        let assign7860_e6852: f64 = (assign7860_e6848 + assign7860_e6851);
        let assign7860_e6855: f64 = (p.p587 * locals.var_iae);
        let assign7860_e6856: f64 = (assign7860_e6852 + assign7860_e6855);
        (assign7860_e6856,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign7860_e6858;

        let assign7870_e6877: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7870_e6877;

        let (assign7880_e6897,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign7880_e6885: f64 = (p.p589 * locals.var_ile);
        let assign7880_e6886: f64 = (p.p588 + assign7880_e6885);
        let assign7880_e6889: f64 = (p.p590 * locals.var_iwe);
        let assign7880_e6890: f64 = (assign7880_e6886 + assign7880_e6889);
        let assign7880_e6893: f64 = (p.p591 * locals.var_iae);
        let assign7880_e6894: f64 = (assign7880_e6890 + assign7880_e6893);
        let assign7880_e6895: f64 = (locals.var_ile * assign7880_e6894);
        (assign7880_e6895,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign7880_e6897;

        let assign7890_e6916: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7890_e6916;

        let (assign7900_e6934,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7900_e6923: f64 = (p.p593 * locals.var_ile);
        let assign7900_e6924: f64 = (p.p592 + assign7900_e6923);
        let assign7900_e6927: f64 = (p.p594 * locals.var_iwe);
        let assign7900_e6928: f64 = (assign7900_e6924 + assign7900_e6927);
        let assign7900_e6931: f64 = (p.p595 * locals.var_iae);
        let assign7900_e6932: f64 = (assign7900_e6928 + assign7900_e6931);
        (assign7900_e6932,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign7900_e6934;

        let assign7910_e6953: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7910_e6953;

        let (assign7920_e6971,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard88 != 0.0)) {
        let assign7920_e6960: f64 = (p.p597 * locals.var_ile);
        let assign7920_e6961: f64 = (p.p596 + assign7920_e6960);
        let assign7920_e6964: f64 = (p.p598 * locals.var_iwe);
        let assign7920_e6965: f64 = (assign7920_e6961 + assign7920_e6964);
        let assign7920_e6968: f64 = (p.p599 * locals.var_iae);
        let assign7920_e6969: f64 = (assign7920_e6965 + assign7920_e6968);
        (assign7920_e6969,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign7920_e6971;

        let assign7930_e6990: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7930_e6990;

        let (assign7940_e7008,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard89 != 0.0)) {
        let assign7940_e6997: f64 = (p.p601 * locals.var_ile);
        let assign7940_e6998: f64 = (p.p600 + assign7940_e6997);
        let assign7940_e7001: f64 = (p.p602 * locals.var_iwe);
        let assign7940_e7002: f64 = (assign7940_e6998 + assign7940_e7001);
        let assign7940_e7005: f64 = (p.p603 * locals.var_iae);
        let assign7940_e7006: f64 = (assign7940_e7002 + assign7940_e7005);
        (assign7940_e7006,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign7940_e7008;

        let assign7950_e7027: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        locals.var_guard90 = assign7950_e7027;

        let (assign7960_e7045,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign7960_e7034: f64 = (p.p605 * locals.var_ile);
        let assign7960_e7035: f64 = (p.p604 + assign7960_e7034);
        let assign7960_e7038: f64 = (p.p606 * locals.var_iwe);
        let assign7960_e7039: f64 = (assign7960_e7035 + assign7960_e7038);
        let assign7960_e7042: f64 = (p.p607 * locals.var_iae);
        let assign7960_e7043: f64 = (assign7960_e7039 + assign7960_e7042);
        (assign7960_e7043,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign7960_e7045;

        let assign7970_e7064: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        locals.var_guard91 = assign7970_e7064;

        let (assign7980_e7082,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard91 != 0.0)) {
        let assign7980_e7071: f64 = (p.p609 * locals.var_ile);
        let assign7980_e7072: f64 = (p.p608 + assign7980_e7071);
        let assign7980_e7075: f64 = (p.p610 * locals.var_iwe);
        let assign7980_e7076: f64 = (assign7980_e7072 + assign7980_e7075);
        let assign7980_e7079: f64 = (p.p611 * locals.var_iae);
        let assign7980_e7080: f64 = (assign7980_e7076 + assign7980_e7079);
        (assign7980_e7080,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign7980_e7082;

        let assign7990_e7101: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign7990_e7101;

        let (assign8000_e7119,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign8000_e7108: f64 = (p.p613 * locals.var_ile);
        let assign8000_e7109: f64 = (p.p612 + assign8000_e7108);
        let assign8000_e7112: f64 = (p.p614 * locals.var_iwe);
        let assign8000_e7113: f64 = (assign8000_e7109 + assign8000_e7112);
        let assign8000_e7116: f64 = (p.p615 * locals.var_iae);
        let assign8000_e7117: f64 = (assign8000_e7113 + assign8000_e7116);
        (assign8000_e7117,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign8000_e7119;

        let assign8010_e7138: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        locals.var_guard93 = assign8010_e7138;

        let (assign8020_e7158,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign8020_e7146: f64 = (p.p617 * locals.var_ile);
        let assign8020_e7147: f64 = (p.p616 + assign8020_e7146);
        let assign8020_e7150: f64 = (p.p618 * locals.var_iwe);
        let assign8020_e7151: f64 = (assign8020_e7147 + assign8020_e7150);
        let assign8020_e7154: f64 = (p.p619 * locals.var_iae);
        let assign8020_e7155: f64 = (assign8020_e7151 + assign8020_e7154);
        let assign8020_e7156: f64 = (locals.var_iiae * assign8020_e7155);
        (assign8020_e7156,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign8020_e7158;

        let assign8030_e7177: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        locals.var_guard94 = assign8030_e7177;

        let (assign8040_e7197,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard94 != 0.0)) {
        let assign8040_e7185: f64 = (p.p621 * locals.var_ile);
        let assign8040_e7186: f64 = (p.p620 + assign8040_e7185);
        let assign8040_e7189: f64 = (p.p622 * locals.var_iwe);
        let assign8040_e7190: f64 = (assign8040_e7186 + assign8040_e7189);
        let assign8040_e7193: f64 = (p.p623 * locals.var_iae);
        let assign8040_e7194: f64 = (assign8040_e7190 + assign8040_e7193);
        let assign8040_e7195: f64 = (locals.var_iiwe * assign8040_e7194);
        (assign8040_e7195,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign8040_e7197;

        let assign8050_e7216: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        locals.var_guard95 = assign8050_e7216;

        let (assign8060_e7236,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign8060_e7224: f64 = (p.p625 * locals.var_ile);
        let assign8060_e7225: f64 = (p.p624 + assign8060_e7224);
        let assign8060_e7228: f64 = (p.p626 * locals.var_iwe);
        let assign8060_e7229: f64 = (assign8060_e7225 + assign8060_e7228);
        let assign8060_e7232: f64 = (p.p627 * locals.var_iae);
        let assign8060_e7233: f64 = (assign8060_e7229 + assign8060_e7232);
        let assign8060_e7234: f64 = (locals.var_iiwe * assign8060_e7233);
        (assign8060_e7234,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign8060_e7236;

        let assign8070_e7255: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        locals.var_guard96 = assign8070_e7255;

        let (assign8080_e7273,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard96 != 0.0)) {
        let assign8080_e7262: f64 = (p.p629 * locals.var_ile);
        let assign8080_e7263: f64 = (p.p628 + assign8080_e7262);
        let assign8080_e7266: f64 = (p.p630 * locals.var_iwe);
        let assign8080_e7267: f64 = (assign8080_e7263 + assign8080_e7266);
        let assign8080_e7270: f64 = (p.p631 * locals.var_iae);
        let assign8080_e7271: f64 = (assign8080_e7267 + assign8080_e7270);
        (assign8080_e7271,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign8080_e7273;

        let assign8090_e7292: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        locals.var_guard97 = assign8090_e7292;

        let (assign8100_e7312,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard97 != 0.0)) {
        let assign8100_e7300: f64 = (p.p633 * locals.var_ile);
        let assign8100_e7301: f64 = (p.p632 + assign8100_e7300);
        let assign8100_e7304: f64 = (p.p634 * locals.var_iwe);
        let assign8100_e7305: f64 = (assign8100_e7301 + assign8100_e7304);
        let assign8100_e7308: f64 = (p.p635 * locals.var_iae);
        let assign8100_e7309: f64 = (assign8100_e7305 + assign8100_e7308);
        let assign8100_e7310: f64 = (locals.var_iiwe * assign8100_e7309);
        (assign8100_e7310,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign8100_e7312;

        let assign8110_e7331: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8110_e7331;

        let (assign8120_e7351,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard98 != 0.0)) {
        let assign8120_e7339: f64 = (p.p637 * locals.var_ile);
        let assign8120_e7340: f64 = (p.p636 + assign8120_e7339);
        let assign8120_e7343: f64 = (p.p638 * locals.var_iwe);
        let assign8120_e7344: f64 = (assign8120_e7340 + assign8120_e7343);
        let assign8120_e7347: f64 = (p.p639 * locals.var_iae);
        let assign8120_e7348: f64 = (assign8120_e7344 + assign8120_e7347);
        let assign8120_e7349: f64 = (locals.var_iiwe * assign8120_e7348);
        (assign8120_e7349,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign8120_e7351;

        let assign8130_e7370: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8130_e7370;

        let (assign8140_e7388,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard99 != 0.0)) {
        let assign8140_e7377: f64 = (p.p641 * locals.var_ile);
        let assign8140_e7378: f64 = (p.p640 + assign8140_e7377);
        let assign8140_e7381: f64 = (p.p642 * locals.var_iwe);
        let assign8140_e7382: f64 = (assign8140_e7378 + assign8140_e7381);
        let assign8140_e7385: f64 = (p.p643 * locals.var_iae);
        let assign8140_e7386: f64 = (assign8140_e7382 + assign8140_e7385);
        (assign8140_e7386,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign8140_e7388;

        let assign8150_e7407: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8150_e7407;

        let (assign8160_e7425,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard100 != 0.0)) {
        let assign8160_e7414: f64 = (p.p645 * locals.var_ile);
        let assign8160_e7415: f64 = (p.p644 + assign8160_e7414);
        let assign8160_e7418: f64 = (p.p646 * locals.var_iwe);
        let assign8160_e7419: f64 = (assign8160_e7415 + assign8160_e7418);
        let assign8160_e7422: f64 = (p.p647 * locals.var_iae);
        let assign8160_e7423: f64 = (assign8160_e7419 + assign8160_e7422);
        (assign8160_e7423,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign8160_e7425;

        let assign8170_e7444: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8170_e7444;

        let (assign8180_e7468,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard101 != 0.0)) {
        let assign8180_e7450: f64 = (locals.var_iiwecv * locals.var_lecv);
        let assign8180_e7452: f64 = (assign8180_e7450 / 1e-6);
        let assign8180_e7456: f64 = (p.p649 * locals.var_ile);
        let assign8180_e7457: f64 = (p.p648 + assign8180_e7456);
        let assign8180_e7460: f64 = (p.p650 * locals.var_iwe);
        let assign8180_e7461: f64 = (assign8180_e7457 + assign8180_e7460);
        let assign8180_e7464: f64 = (p.p651 * locals.var_iae);
        let assign8180_e7465: f64 = (assign8180_e7461 + assign8180_e7464);
        let assign8180_e7466: f64 = (assign8180_e7452 * assign8180_e7465);
        (assign8180_e7466,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign8180_e7468;

        let assign8190_e7487: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8190_e7487;

        let (assign8200_e7505,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard102 != 0.0)) {
        let assign8200_e7494: f64 = (p.p653 * locals.var_ile);
        let assign8200_e7495: f64 = (p.p652 + assign8200_e7494);
        let assign8200_e7498: f64 = (p.p654 * locals.var_iwe);
        let assign8200_e7499: f64 = (assign8200_e7495 + assign8200_e7498);
        let assign8200_e7502: f64 = (p.p655 * locals.var_iae);
        let assign8200_e7503: f64 = (assign8200_e7499 + assign8200_e7502);
        (assign8200_e7503,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign8200_e7505;

        let assign8210_e7524: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8210_e7524;

        let (assign8220_e7542,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard103 != 0.0)) {
        let assign8220_e7531: f64 = (p.p657 * locals.var_ile);
        let assign8220_e7532: f64 = (p.p656 + assign8220_e7531);
        let assign8220_e7535: f64 = (p.p658 * locals.var_iwe);
        let assign8220_e7536: f64 = (assign8220_e7532 + assign8220_e7535);
        let assign8220_e7539: f64 = (p.p659 * locals.var_iae);
        let assign8220_e7540: f64 = (assign8220_e7536 + assign8220_e7539);
        (assign8220_e7540,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign8220_e7542;

        let assign8230_e7581: f64 = if (((((((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) || param_given[568]) || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8230_e7581;

        let (assign8240_e7587,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p568,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8240_e7587;

        let assign8250_e7589: f64 = if param_given[660] { 1.0 } else { 0.0 };
        let assign8250_e7591: f64 = if assign8250_e7589 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign8250_e7591;

        let (assign8260_e7599,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard105 != 0.0)) {
        (p.p660,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8260_e7599;

        let (assign8270_e7605,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p569,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8270_e7605;

        let assign8280_e7607: f64 = if param_given[661] { 1.0 } else { 0.0 };
        let assign8280_e7609: f64 = if assign8280_e7607 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign8280_e7609;

        let (assign8290_e7617,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 != 0.0)) {
        (p.p661,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8290_e7617;

        let (assign8300_e7623,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p570,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8300_e7623;

        let assign8310_e7625: f64 = if param_given[662] { 1.0 } else { 0.0 };
        let assign8310_e7627: f64 = if assign8310_e7625 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign8310_e7627;

        let (assign8320_e7635,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard107 != 0.0)) {
        (p.p662,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8320_e7635;

        let (assign8330_e7641,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p571,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8330_e7641;

        let assign8340_e7643: f64 = if param_given[663] { 1.0 } else { 0.0 };
        let assign8340_e7645: f64 = if assign8340_e7643 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign8340_e7645;

        let (assign8350_e7653,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard108 != 0.0)) {
        (p.p663,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8350_e7653;

        let (assign8360_e7673,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        let assign8360_e7661: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8360_e7662: f64 = (locals.var_poparam_i + assign8360_e7661);
        let assign8360_e7665: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8360_e7666: f64 = (assign8360_e7662 + assign8360_e7665);
        let assign8360_e7669: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8360_e7670: f64 = (assign8360_e7666 + assign8360_e7669);
        let assign8360_e7671: f64 = (locals.var_ile * assign8360_e7670);
        (assign8360_e7671,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign8360_e7673;

        let assign8370_e7712: f64 = if (((((((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) || param_given[584]) || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign8370_e7712;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign8380_e7718,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p584,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8380_e7718;

        let assign8390_e7720: f64 = if param_given[664] { 1.0 } else { 0.0 };
        let assign8390_e7722: f64 = if assign8390_e7720 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign8390_e7722;

        let (assign8400_e7730,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
        (p.p664,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8400_e7730;

        let (assign8410_e7736,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p585,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8410_e7736;

        let assign8420_e7738: f64 = if param_given[665] { 1.0 } else { 0.0 };
        let assign8420_e7740: f64 = if assign8420_e7738 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign8420_e7740;

        let (assign8430_e7748,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (p.p665,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8430_e7748;

        let (assign8440_e7754,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p586,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8440_e7754;

        let assign8450_e7756: f64 = if param_given[666] { 1.0 } else { 0.0 };
        let assign8450_e7758: f64 = if assign8450_e7756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign8450_e7758;

        let (assign8460_e7766,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (p.p666,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8460_e7766;

        let (assign8470_e7772,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p587,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8470_e7772;

        let assign8480_e7774: f64 = if param_given[667] { 1.0 } else { 0.0 };
        let assign8480_e7776: f64 = if assign8480_e7774 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8480_e7776;

        let (assign8490_e7784,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p667,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8490_e7784;

        let (assign8500_e7804,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        let assign8500_e7792: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8500_e7793: f64 = (locals.var_poparam_i + assign8500_e7792);
        let assign8500_e7796: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8500_e7797: f64 = (assign8500_e7793 + assign8500_e7796);
        let assign8500_e7800: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8500_e7801: f64 = (assign8500_e7797 + assign8500_e7800);
        let assign8500_e7802: f64 = assign8500_e7801;
        (assign8500_e7802,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign8500_e7804;

        let assign8510_e7823: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8510_e7823;

        let (assign8520_e7843,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard114 != 0.0)) {
        let assign8520_e7831: f64 = (p.p669 * locals.var_ile);
        let assign8520_e7832: f64 = (p.p668 + assign8520_e7831);
        let assign8520_e7835: f64 = (p.p670 * locals.var_iwe);
        let assign8520_e7836: f64 = (assign8520_e7832 + assign8520_e7835);
        let assign8520_e7839: f64 = (p.p671 * locals.var_iae);
        let assign8520_e7840: f64 = (assign8520_e7836 + assign8520_e7839);
        let assign8520_e7841: f64 = (locals.var_ile * assign8520_e7840);
        (assign8520_e7841,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign8520_e7843;

        let assign8530_e7862: f64 = if (((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8530_e7862;

        let (assign8540_e7882,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard115 != 0.0)) {
        let assign8540_e7870: f64 = (p.p673 * locals.var_ile);
        let assign8540_e7871: f64 = (p.p672 + assign8540_e7870);
        let assign8540_e7874: f64 = (p.p674 * locals.var_iwe);
        let assign8540_e7875: f64 = (assign8540_e7871 + assign8540_e7874);
        let assign8540_e7878: f64 = (p.p675 * locals.var_iae);
        let assign8540_e7879: f64 = (assign8540_e7875 + assign8540_e7878);
        let assign8540_e7880: f64 = (locals.var_ile * assign8540_e7879);
        (assign8540_e7880,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign8540_e7882;

        let assign8550_e7901: f64 = if (((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8550_e7901;

        let (assign8560_e7921,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard116 != 0.0)) {
        let assign8560_e7909: f64 = (p.p677 * locals.var_ile);
        let assign8560_e7910: f64 = (p.p676 + assign8560_e7909);
        let assign8560_e7913: f64 = (p.p678 * locals.var_iwe);
        let assign8560_e7914: f64 = (assign8560_e7910 + assign8560_e7913);
        let assign8560_e7917: f64 = (p.p679 * locals.var_iae);
        let assign8560_e7918: f64 = (assign8560_e7914 + assign8560_e7917);
        let assign8560_e7919: f64 = (locals.var_iiwecv * assign8560_e7918);
        (assign8560_e7919,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign8560_e7921;

        let assign8570_e7940: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8570_e7940;

        let (assign8580_e7960,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard117 != 0.0)) {
        let assign8580_e7948: f64 = (p.p681 * locals.var_ile);
        let assign8580_e7949: f64 = (p.p680 + assign8580_e7948);
        let assign8580_e7952: f64 = (p.p682 * locals.var_iwe);
        let assign8580_e7953: f64 = (assign8580_e7949 + assign8580_e7952);
        let assign8580_e7956: f64 = (p.p683 * locals.var_iae);
        let assign8580_e7957: f64 = (assign8580_e7953 + assign8580_e7956);
        let assign8580_e7958: f64 = (locals.var_iiwecv * assign8580_e7957);
        (assign8580_e7958,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign8580_e7960;

        let assign8590_e7979: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8590_e7979;

        let (assign8600_e7999,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard118 != 0.0)) {
        let assign8600_e7987: f64 = (p.p685 * locals.var_ile);
        let assign8600_e7988: f64 = (p.p684 + assign8600_e7987);
        let assign8600_e7991: f64 = (p.p686 * locals.var_iwe);
        let assign8600_e7992: f64 = (assign8600_e7988 + assign8600_e7991);
        let assign8600_e7995: f64 = (p.p687 * locals.var_iae);
        let assign8600_e7996: f64 = (assign8600_e7992 + assign8600_e7995);
        let assign8600_e7997: f64 = (locals.var_iilcv * assign8600_e7996);
        (assign8600_e7997,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign8600_e7999;

        let assign8610_e8018: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign8610_e8018;

        let (assign8620_e8038,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard119 != 0.0)) {
        let assign8620_e8026: f64 = (p.p689 * locals.var_ile);
        let assign8620_e8027: f64 = (p.p688 + assign8620_e8026);
        let assign8620_e8030: f64 = (p.p690 * locals.var_iwe);
        let assign8620_e8031: f64 = (assign8620_e8027 + assign8620_e8030);
        let assign8620_e8034: f64 = (p.p691 * locals.var_iae);
        let assign8620_e8035: f64 = (assign8620_e8031 + assign8620_e8034);
        let assign8620_e8036: f64 = (locals.var_iiwecv * assign8620_e8035);
        (assign8620_e8036,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign8620_e8038;

        let assign8630_e8057: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign8630_e8057;

        let (assign8640_e8077,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard120 != 0.0)) {
        let assign8640_e8065: f64 = (p.p693 * locals.var_ile);
        let assign8640_e8066: f64 = (p.p692 + assign8640_e8065);
        let assign8640_e8069: f64 = (p.p694 * locals.var_iwe);
        let assign8640_e8070: f64 = (assign8640_e8066 + assign8640_e8069);
        let assign8640_e8073: f64 = (p.p695 * locals.var_iae);
        let assign8640_e8074: f64 = (assign8640_e8070 + assign8640_e8073);
        let assign8640_e8075: f64 = (locals.var_iiwecv * assign8640_e8074);
        (assign8640_e8075,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign8640_e8077;

        let assign8690_e8174: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };
        locals.var_guard123 = assign8690_e8174;

        let (assign8700_e8194,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign8700_e8182: f64 = (p.p705 * locals.var_ile);
        let assign8700_e8183: f64 = (p.p704 + assign8700_e8182);
        let assign8700_e8186: f64 = (p.p706 * locals.var_iwe);
        let assign8700_e8187: f64 = (assign8700_e8183 + assign8700_e8186);
        let assign8700_e8190: f64 = (p.p707 * locals.var_iae);
        let assign8700_e8191: f64 = (assign8700_e8187 + assign8700_e8190);
        let assign8700_e8192: f64 = (locals.var_ile2 * assign8700_e8191);
        (assign8700_e8192,)
    } else {
        (locals.var_fntexc_p,)
    }
};
        locals.var_fntexc_p = assign8700_e8194;

        let assign8770_e8330: f64 = if (((param_given[720] || param_given[721]) || param_given[722]) || param_given[723]) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign8770_e8330;

        let (assign8780_e8348,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard127 != 0.0)) {
        let assign8780_e8337: f64 = (p.p721 * locals.var_ile);
        let assign8780_e8338: f64 = (p.p720 + assign8780_e8337);
        let assign8780_e8341: f64 = (p.p722 * locals.var_iwe);
        let assign8780_e8342: f64 = (assign8780_e8338 + assign8780_e8341);
        let assign8780_e8345: f64 = (p.p723 * locals.var_iae);
        let assign8780_e8346: f64 = (assign8780_e8342 + assign8780_e8345);
        (assign8780_e8346,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign8780_e8348;

        let assign8790_e8367: f64 = if (((param_given[724] || param_given[725]) || param_given[726]) || param_given[727]) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign8790_e8367;

        let (assign8800_e8385,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard128 != 0.0)) {
        let assign8800_e8374: f64 = (p.p725 * locals.var_ile);
        let assign8800_e8375: f64 = (p.p724 + assign8800_e8374);
        let assign8800_e8378: f64 = (p.p726 * locals.var_iwe);
        let assign8800_e8379: f64 = (assign8800_e8375 + assign8800_e8378);
        let assign8800_e8382: f64 = (p.p727 * locals.var_iae);
        let assign8800_e8383: f64 = (assign8800_e8379 + assign8800_e8382);
        (assign8800_e8383,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign8800_e8385;

        let assign8810_e8404: f64 = if (((param_given[728] || param_given[729]) || param_given[730]) || param_given[731]) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign8810_e8404;

        let (assign8820_e8422,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard129 != 0.0)) {
        let assign8820_e8411: f64 = (p.p729 * locals.var_ile);
        let assign8820_e8412: f64 = (p.p728 + assign8820_e8411);
        let assign8820_e8415: f64 = (p.p730 * locals.var_iwe);
        let assign8820_e8416: f64 = (assign8820_e8412 + assign8820_e8415);
        let assign8820_e8419: f64 = (p.p731 * locals.var_iae);
        let assign8820_e8420: f64 = (assign8820_e8416 + assign8820_e8419);
        (assign8820_e8420,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign8820_e8422;

        let assign8830_e8441: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign8830_e8441;

        let (assign8840_e8459,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard130 != 0.0)) {
        let assign8840_e8448: f64 = (p.p733 * locals.var_ile);
        let assign8840_e8449: f64 = (p.p732 + assign8840_e8448);
        let assign8840_e8452: f64 = (p.p734 * locals.var_iwe);
        let assign8840_e8453: f64 = (assign8840_e8449 + assign8840_e8452);
        let assign8840_e8456: f64 = (p.p735 * locals.var_iae);
        let assign8840_e8457: f64 = (assign8840_e8453 + assign8840_e8456);
        (assign8840_e8457,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign8840_e8459;

        let assign8850_e8478: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign8850_e8478;

        let (assign8860_e8496,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard131 != 0.0)) {
        let assign8860_e8485: f64 = (p.p737 * locals.var_ile);
        let assign8860_e8486: f64 = (p.p736 + assign8860_e8485);
        let assign8860_e8489: f64 = (p.p738 * locals.var_iwe);
        let assign8860_e8490: f64 = (assign8860_e8486 + assign8860_e8489);
        let assign8860_e8493: f64 = (p.p739 * locals.var_iae);
        let assign8860_e8494: f64 = (assign8860_e8490 + assign8860_e8493);
        (assign8860_e8494,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign8860_e8496;

        let assign8870_e8515: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8870_e8515;

        let (assign8880_e8537,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard132 != 0.0)) {
        let assign8880_e8521: f64 = (locals.var_we_edge / locals.var_le);
        let assign8880_e8525: f64 = (p.p741 * locals.var_ile);
        let assign8880_e8526: f64 = (p.p740 + assign8880_e8525);
        let assign8880_e8529: f64 = (p.p742 * locals.var_iwe);
        let assign8880_e8530: f64 = (assign8880_e8526 + assign8880_e8529);
        let assign8880_e8533: f64 = (p.p743 * locals.var_iae);
        let assign8880_e8534: f64 = (assign8880_e8530 + assign8880_e8533);
        let assign8880_e8535: f64 = (assign8880_e8521 * assign8880_e8534);
        (assign8880_e8535,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign8880_e8537;

        let assign8890_e8556: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8890_e8556;

        let (assign8900_e8574,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard133 != 0.0)) {
        let assign8900_e8563: f64 = (p.p745 * locals.var_ile);
        let assign8900_e8564: f64 = (p.p744 + assign8900_e8563);
        let assign8900_e8567: f64 = (p.p746 * locals.var_iwe);
        let assign8900_e8568: f64 = (assign8900_e8564 + assign8900_e8567);
        let assign8900_e8571: f64 = (p.p747 * locals.var_iae);
        let assign8900_e8572: f64 = (assign8900_e8568 + assign8900_e8571);
        (assign8900_e8572,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign8900_e8574;

        let assign8910_e8593: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8910_e8593;

        let (assign8920_e8613,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard134 != 0.0)) {
        let assign8920_e8601: f64 = (p.p749 * locals.var_ile);
        let assign8920_e8602: f64 = (p.p748 + assign8920_e8601);
        let assign8920_e8605: f64 = (p.p750 * locals.var_iwe);
        let assign8920_e8606: f64 = (assign8920_e8602 + assign8920_e8605);
        let assign8920_e8609: f64 = (p.p751 * locals.var_iae);
        let assign8920_e8610: f64 = (assign8920_e8606 + assign8920_e8609);
        let assign8920_e8611: f64 = (locals.var_ile2 * assign8920_e8610);
        (assign8920_e8611,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign8920_e8613;

        let assign8930_e8632: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8930_e8632;

        let (assign8940_e8650,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard135 != 0.0)) {
        let assign8940_e8639: f64 = (p.p753 * locals.var_ile);
        let assign8940_e8640: f64 = (p.p752 + assign8940_e8639);
        let assign8940_e8643: f64 = (p.p754 * locals.var_iwe);
        let assign8940_e8644: f64 = (assign8940_e8640 + assign8940_e8643);
        let assign8940_e8647: f64 = (p.p755 * locals.var_iae);
        let assign8940_e8648: f64 = (assign8940_e8644 + assign8940_e8647);
        (assign8940_e8648,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign8940_e8650;

        let assign8950_e8669: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign8950_e8669;

        let (assign8960_e8687,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard136 != 0.0)) {
        let assign8960_e8676: f64 = (p.p757 * locals.var_ile);
        let assign8960_e8677: f64 = (p.p756 + assign8960_e8676);
        let assign8960_e8680: f64 = (p.p758 * locals.var_iwe);
        let assign8960_e8681: f64 = (assign8960_e8677 + assign8960_e8680);
        let assign8960_e8684: f64 = (p.p759 * locals.var_iae);
        let assign8960_e8685: f64 = (assign8960_e8681 + assign8960_e8684);
        (assign8960_e8685,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign8960_e8687;

        let assign8970_e8706: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign8970_e8706;

        let (assign8980_e8726,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard137 != 0.0)) {
        let assign8980_e8714: f64 = (p.p761 * locals.var_ile);
        let assign8980_e8715: f64 = (p.p760 + assign8980_e8714);
        let assign8980_e8718: f64 = (p.p762 * locals.var_iwe);
        let assign8980_e8719: f64 = (assign8980_e8715 + assign8980_e8718);
        let assign8980_e8722: f64 = (p.p763 * locals.var_iae);
        let assign8980_e8723: f64 = (assign8980_e8719 + assign8980_e8722);
        let assign8980_e8724: f64 = (locals.var_ile2 * assign8980_e8723);
        (assign8980_e8724,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign8980_e8726;

        let assign8990_e8745: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign8990_e8745;

        let (assign9000_e8763,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard138 != 0.0)) {
        let assign9000_e8752: f64 = (p.p769 * locals.var_ile);
        let assign9000_e8753: f64 = (p.p768 + assign9000_e8752);
        let assign9000_e8756: f64 = (p.p770 * locals.var_iwe);
        let assign9000_e8757: f64 = (assign9000_e8753 + assign9000_e8756);
        let assign9000_e8760: f64 = (p.p771 * locals.var_iae);
        let assign9000_e8761: f64 = (assign9000_e8757 + assign9000_e8760);
        (assign9000_e8761,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign9000_e8763;

        let assign9010_e8782: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign9010_e8782;

        let (assign9020_e8800,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard139 != 0.0)) {
        let assign9020_e8789: f64 = (p.p765 * locals.var_ile);
        let assign9020_e8790: f64 = (p.p764 + assign9020_e8789);
        let assign9020_e8793: f64 = (p.p766 * locals.var_iwe);
        let assign9020_e8794: f64 = (assign9020_e8790 + assign9020_e8793);
        let assign9020_e8797: f64 = (p.p767 * locals.var_iae);
        let assign9020_e8798: f64 = (assign9020_e8794 + assign9020_e8797);
        (assign9020_e8798,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign9020_e8800;

        let (assign9090_e8921,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpa,)
    }
};
        locals.var_tmpa = assign9090_e8921;

        let (assign9100_e8925,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpb,)
    }
};
        locals.var_tmpb = assign9100_e8925;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign9110_e8929,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_loop_,)
    }
};
        locals.var_loop_ = assign9110_e8929;

        let (assign9120_e8933,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p788,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9120_e8933;

        let assign9130_e8935: f64 = if param_given[789] { 1.0 } else { 0.0 };
        let assign9130_e8937: f64 = if assign9130_e8935 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign9130_e8937;

        let (assign9140_e8943,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard143 != 0.0)) {
        (p.p789,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9140_e8943;

        let assign9150_e8962: f64 = if (((locals.var_sa_i > 0.0) && (locals.var_sb_i > 0.0)) && ((locals.var_nf_i == 1.0) || ((locals.var_nf_i > 1.0) && (locals.var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard144 = assign9150_e8962;

        let mut assign9160_loop_guard: usize = 0;
        while {
            let assign9160_cond_e8969: f64 = (locals.var_nf_i - 0.5);
            let assign9160_cond_e8971: f64 = if (((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) && (locals.var_loop_ < assign9160_cond_e8969)) { 1.0 } else { 0.0 };
            assign9160_cond_e8971 != 0.0
        } {
            assign9160_loop_guard += 1;
            assert!(assign9160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9160_body0_e8991,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9160_body0_e8980: f64 = (0.5 * locals.var_l_i);
        let assign9160_body0_e8981: f64 = (locals.var_sa_i + assign9160_body0_e8980);
        let assign9160_body0_e8985: f64 = (locals.var_sd_i + locals.var_l_i);
        let assign9160_body0_e8986: f64 = (locals.var_loop_ * assign9160_body0_e8985);
        let assign9160_body0_e8987: f64 = (assign9160_body0_e8981 + assign9160_body0_e8986);
        let assign9160_body0_e8988: f64 = (1.0 / assign9160_body0_e8987);
        let assign9160_body0_e8989: f64 = (locals.var_tmpa + assign9160_body0_e8988);
        (assign9160_body0_e8989,)
    } else {
        (locals.var_tmpa,)
    }
};
            locals.var_tmpa = assign9160_body0_e8991;
            let (assign9160_body1_e9011,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9160_body1_e9000: f64 = (0.5 * locals.var_l_i);
        let assign9160_body1_e9001: f64 = (locals.var_sb_i + assign9160_body1_e9000);
        let assign9160_body1_e9005: f64 = (locals.var_sd_i + locals.var_l_i);
        let assign9160_body1_e9006: f64 = (locals.var_loop_ * assign9160_body1_e9005);
        let assign9160_body1_e9007: f64 = (assign9160_body1_e9001 + assign9160_body1_e9006);
        let assign9160_body1_e9008: f64 = (1.0 / assign9160_body1_e9007);
        let assign9160_body1_e9009: f64 = (locals.var_tmpb + assign9160_body1_e9008);
        (assign9160_body1_e9009,)
    } else {
        (locals.var_tmpb,)
    }
};
            locals.var_tmpb = assign9160_body1_e9011;
            let (assign9160_body2_e9019,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9160_body2_e9017: f64 = (locals.var_loop_ + 1.0);
        (assign9160_body2_e9017,)
    } else {
        (locals.var_loop_,)
    }
};
            locals.var_loop_ = assign9160_body2_e9019;
        }

        let (assign9170_e9027,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9170_e9025: f64 = (locals.var_tmpa * locals.var_invnf);
        (assign9170_e9025,)
    } else {
        (locals.var_invsa,)
    }
};
        locals.var_invsa = assign9170_e9027;

        let (assign9180_e9035,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9180_e9033: f64 = (locals.var_tmpb * locals.var_invnf);
        (assign9180_e9033,)
    } else {
        (locals.var_invsb,)
    }
};
        locals.var_invsb = assign9180_e9035;

        let (assign9190_e9047,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9190_e9043: f64 = (0.5 * locals.var_l_i);
        let assign9190_e9044: f64 = (p.p784 + assign9190_e9043);
        let assign9190_e9045: f64 = (1.0 / assign9190_e9044);
        (assign9190_e9045,)
    } else {
        (locals.var_invsaref,)
    }
};
        locals.var_invsaref = assign9190_e9047;

        let (assign9200_e9059,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9200_e9055: f64 = (0.5 * locals.var_l_i);
        let assign9200_e9056: f64 = (p.p785 + assign9200_e9055);
        let assign9200_e9057: f64 = (1.0 / assign9200_e9056);
        (assign9200_e9057,)
    } else {
        (locals.var_invsbref,)
    }
};
        locals.var_invsbref = assign9200_e9059;

        let (assign9210_e9074,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9210_e9065: f64 = (locals.var_l_i + locals.var_dellps);
        let (assign9210_e9072,) = {
            if (assign9210_e9065 > 1e-9) {
                let assign9210_e9070: f64 = (locals.var_l_i + locals.var_dellps);
                (assign9210_e9070,)
            } else {
                (1e-9,)
            }
        };
        (assign9210_e9072,)
    } else {
        (locals.var_lx,)
    }
};
        locals.var_lx = assign9210_e9074;

        let (assign9220_e9093,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9220_e9080: f64 = (locals.var_w_i + locals.var_delwod);
        let assign9220_e9082: f64 = (assign9220_e9080 + p.p786);
        let (assign9220_e9091,) = {
            if (assign9220_e9082 > 1e-9) {
                let assign9220_e9087: f64 = (locals.var_w_i + locals.var_delwod);
                let assign9220_e9089: f64 = (assign9220_e9087 + p.p786);
                (assign9220_e9089,)
            } else {
                (1e-9,)
            }
        };
        (assign9220_e9091,)
    } else {
        (locals.var_wx,)
    }
};
        locals.var_wx = assign9220_e9093;

        let (assign9230_e9103,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9230_e9100: f64 = (locals.var_lx).powf(p.p794);
        let assign9230_e9101: f64 = (1.0 / assign9230_e9100);
        (assign9230_e9101,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign9230_e9103;

        let (assign9240_e9113,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9240_e9110: f64 = (locals.var_wx).powf(p.p795);
        let assign9240_e9111: f64 = (1.0 / assign9240_e9110);
        (assign9240_e9111,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign9240_e9113;

        let (assign9250_e9141,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9250_e9120: f64 = (p.p791 * locals.var_templ);
        let assign9250_e9121: f64 = (1.0 + assign9250_e9120);
        let assign9250_e9124: f64 = (p.p792 * locals.var_tempw);
        let assign9250_e9125: f64 = (assign9250_e9121 + assign9250_e9124);
        let assign9250_e9128: f64 = (p.p793 * locals.var_templ);
        let assign9250_e9130: f64 = (assign9250_e9128 * locals.var_tempw);
        let assign9250_e9131: f64 = (assign9250_e9125 + assign9250_e9130);
        let assign9250_e9136: f64 = (locals.var_rta - 1.0);
        let assign9250_e9137: f64 = (p.p790 * assign9250_e9136);
        let assign9250_e9138: f64 = (1.0 + assign9250_e9137);
        let assign9250_e9139: f64 = (assign9250_e9131 * assign9250_e9138);
        (assign9250_e9139,)
    } else {
        (locals.var_kstressu0,)
    }
};
        locals.var_kstressu0 = assign9250_e9141;

        let (assign9260_e9153,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9260_e9148: f64 = (locals.var_invsa + locals.var_invsb);
        let assign9260_e9149: f64 = (p.p787 * assign9260_e9148);
        let assign9260_e9151: f64 = (assign9260_e9149 / locals.var_kstressu0);
        (assign9260_e9151,)
    } else {
        (locals.var_rhobeta,)
    }
};
        locals.var_rhobeta = assign9260_e9153;

        let (assign9270_e9165,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9270_e9160: f64 = (locals.var_invsaref + locals.var_invsbref);
        let assign9270_e9161: f64 = (p.p787 * assign9270_e9160);
        let assign9270_e9163: f64 = (assign9270_e9161 / locals.var_kstressu0);
        (assign9270_e9163,)
    } else {
        (locals.var_rhobetaref,)
    }
};
        locals.var_rhobetaref = assign9270_e9165;

        let (assign9280_e9175,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9280_e9172: f64 = (locals.var_lx).powf(p.p800);
        let assign9280_e9173: f64 = (1.0 / assign9280_e9172);
        (assign9280_e9173,)
    } else {
        (locals.var_templ,)
    }
};
        locals.var_templ = assign9280_e9175;

        let (assign9290_e9185,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9290_e9182: f64 = (locals.var_wx).powf(p.p801);
        let assign9290_e9183: f64 = (1.0 / assign9290_e9182);
        (assign9290_e9183,)
    } else {
        (locals.var_tempw,)
    }
};
        locals.var_tempw = assign9290_e9185;

        let (assign9300_e9205,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9300_e9192: f64 = (p.p797 * locals.var_templ);
        let assign9300_e9193: f64 = (1.0 + assign9300_e9192);
        let assign9300_e9196: f64 = (p.p798 * locals.var_tempw);
        let assign9300_e9197: f64 = (assign9300_e9193 + assign9300_e9196);
        let assign9300_e9200: f64 = (p.p799 * locals.var_templ);
        let assign9300_e9202: f64 = (assign9300_e9200 * locals.var_tempw);
        let assign9300_e9203: f64 = (assign9300_e9197 + assign9300_e9202);
        (assign9300_e9203,)
    } else {
        (locals.var_kstressvth0,)
    }
};
        locals.var_kstressvth0 = assign9300_e9205;

        let (assign9310_e9217,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9310_e9211: f64 = (locals.var_invsa + locals.var_invsb);
        let assign9310_e9213: f64 = (assign9310_e9211 - locals.var_invsaref);
        let assign9310_e9215: f64 = (assign9310_e9213 - locals.var_invsbref);
        (assign9310_e9215,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9310_e9217;

        let (assign9320_e9229,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9320_e9223: f64 = (1.0 + locals.var_rhobeta);
        let assign9320_e9226: f64 = (1.0 + locals.var_rhobetaref);
        let assign9320_e9227: f64 = (assign9320_e9223 / assign9320_e9226);
        (assign9320_e9227,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9320_e9229;

        let (assign9330_e9237,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9330_e9235: f64 = (locals.var_betn_p * locals.var_temp00);
        (assign9330_e9235,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9330_e9237;

        let (assign9340_e9257,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9340_e9243: f64 = (locals.var_thesat_p * locals.var_temp00);
        let assign9340_e9247: f64 = (p.p788 * locals.var_rhobetaref);
        let assign9340_e9248: f64 = (1.0 + assign9340_e9247);
        let assign9340_e9249: f64 = (assign9340_e9243 * assign9340_e9248);
        let assign9340_e9253: f64 = (p.p788 * locals.var_rhobeta);
        let assign9340_e9254: f64 = (1.0 + assign9340_e9253);
        let assign9340_e9255: f64 = (assign9340_e9249 / assign9340_e9254);
        (assign9340_e9255,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign9340_e9257;

        let (assign9350_e9277,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9350_e9263: f64 = (locals.var_thesatac_p * locals.var_temp00);
        let assign9350_e9267: f64 = (locals.var_kvsatac_i * locals.var_rhobetaref);
        let assign9350_e9268: f64 = (1.0 + assign9350_e9267);
        let assign9350_e9269: f64 = (assign9350_e9263 * assign9350_e9268);
        let assign9350_e9273: f64 = (locals.var_kvsatac_i * locals.var_rhobeta);
        let assign9350_e9274: f64 = (1.0 + assign9350_e9273);
        let assign9350_e9275: f64 = (assign9350_e9269 / assign9350_e9274);
        (assign9350_e9275,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign9350_e9277;

        let (assign9360_e9285,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9360_e9283: f64 = (locals.var_betnedge_p * locals.var_temp00);
        (assign9360_e9283,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9360_e9285;

        let (assign9370_e9295,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9370_e9291: f64 = (p.p796 * locals.var_temp0);
        let assign9370_e9293: f64 = (assign9370_e9291 / locals.var_kstressvth0);
        (assign9370_e9293,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9370_e9295;

        let (assign9380_e9303,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9380_e9301: f64 = (locals.var_vfb_p + locals.var_temp00);
        (assign9380_e9301,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9380_e9303;

        let (assign9390_e9311,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9390_e9309: f64 = (locals.var_vfbedge_p + locals.var_temp00);
        (assign9390_e9309,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9390_e9311;

        let (assign9400_e9323,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9400_e9317: f64 = (p.p802 * locals.var_temp0);
        let assign9400_e9320: f64 = (locals.var_kstressvth0).powf(p.p803);
        let assign9400_e9321: f64 = (assign9400_e9317 / assign9400_e9320);
        (assign9400_e9321,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9400_e9323;

        let (assign9410_e9331,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9410_e9329: f64 = (locals.var_cf_p + locals.var_temp00);
        (assign9410_e9329,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign9410_e9331;

        let (assign9420_e9339,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard144 != 0.0)) {
        let assign9420_e9337: f64 = (locals.var_cfedge_p + locals.var_temp00);
        (assign9420_e9337,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign9420_e9339;

        let assign9430_e9354: f64 = if ((((locals.var_sca_i > 0.0) || (locals.var_scb_i > 0.0)) || (locals.var_scc_i > 0.0)) || (locals.var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard145 = assign9430_e9354;

        let assign9440_e9365: f64 = if (((locals.var_sca_i == 0.0) && (locals.var_scb_i == 0.0)) && (locals.var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard146 = assign9440_e9365;

        let (assign9450_e9375,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9450_e9373: f64 = (locals.var_sc_i + locals.var_w_i);
        (assign9450_e9373,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9450_e9375;

        let (assign9460_e9385,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9460_e9383: f64 = (1.0 / p.p804);
        (assign9460_e9383,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9460_e9385;

        let (assign9470_e9399,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9470_e9393: f64 = (p.p804 * p.p804);
        let assign9470_e9396: f64 = (locals.var_sc_i * locals.var_temp0);
        let assign9470_e9397: f64 = (assign9470_e9393 / assign9470_e9396);
        (assign9470_e9397,)
    } else {
        (locals.var_sca_i,)
    }
};
        locals.var_sca_i = assign9470_e9399;

        let (assign9480_e9439,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9480_e9407: f64 = (0.1 * locals.var_sc_i);
        let assign9480_e9410: f64 = (0.01 * p.p804);
        let assign9480_e9411: f64 = (assign9480_e9407 + assign9480_e9410);
        let assign9480_e9413: f64 = (-10.0);
        let assign9480_e9415: f64 = (assign9480_e9413 * locals.var_sc_i);
        let assign9480_e9417: f64 = (assign9480_e9415 * locals.var_temp00);
        let assign9480_e9418: f64 = (assign9480_e9417).exp();
        let assign9480_e9419: f64 = (assign9480_e9411 * assign9480_e9418);
        let assign9480_e9422: f64 = (0.1 * locals.var_temp0);
        let assign9480_e9425: f64 = (0.01 * p.p804);
        let assign9480_e9426: f64 = (assign9480_e9422 + assign9480_e9425);
        let assign9480_e9428: f64 = (-10.0);
        let assign9480_e9430: f64 = (assign9480_e9428 * locals.var_temp0);
        let assign9480_e9432: f64 = (assign9480_e9430 * locals.var_temp00);
        let assign9480_e9433: f64 = (assign9480_e9432).exp();
        let assign9480_e9434: f64 = (assign9480_e9426 * assign9480_e9433);
        let assign9480_e9435: f64 = (assign9480_e9419 - assign9480_e9434);
        let assign9480_e9437: f64 = (assign9480_e9435 / locals.var_w_i);
        (assign9480_e9437,)
    } else {
        (locals.var_scb_i,)
    }
};
        locals.var_scb_i = assign9480_e9439;

        let (assign9490_e9479,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) && (locals.var_guard146 != 0.0)) {
        let assign9490_e9447: f64 = (0.05 * locals.var_sc_i);
        let assign9490_e9450: f64 = (0.0025 * p.p804);
        let assign9490_e9451: f64 = (assign9490_e9447 + assign9490_e9450);
        let assign9490_e9453: f64 = (-20.0);
        let assign9490_e9455: f64 = (assign9490_e9453 * locals.var_sc_i);
        let assign9490_e9457: f64 = (assign9490_e9455 * locals.var_temp00);
        let assign9490_e9458: f64 = (assign9490_e9457).exp();
        let assign9490_e9459: f64 = (assign9490_e9451 * assign9490_e9458);
        let assign9490_e9462: f64 = (0.05 * locals.var_temp0);
        let assign9490_e9465: f64 = (0.0025 * p.p804);
        let assign9490_e9466: f64 = (assign9490_e9462 + assign9490_e9465);
        let assign9490_e9468: f64 = (-20.0);
        let assign9490_e9470: f64 = (assign9490_e9468 * locals.var_temp0);
        let assign9490_e9472: f64 = (assign9490_e9470 * locals.var_temp00);
        let assign9490_e9473: f64 = (assign9490_e9472).exp();
        let assign9490_e9474: f64 = (assign9490_e9466 * assign9490_e9473);
        let assign9490_e9475: f64 = (assign9490_e9459 - assign9490_e9474);
        let assign9490_e9477: f64 = (assign9490_e9475 / locals.var_w_i);
        (assign9490_e9477,)
    } else {
        (locals.var_scc_i,)
    }
};
        locals.var_scc_i = assign9490_e9479;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9500_e9493,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9500_e9486: f64 = (p.p805 * locals.var_scb_i);
        let assign9500_e9487: f64 = (locals.var_sca_i + assign9500_e9486);
        let assign9500_e9490: f64 = (p.p806 * locals.var_scc_i);
        let assign9500_e9491: f64 = (assign9500_e9487 + assign9500_e9490);
        (assign9500_e9491,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9500_e9493;

        let (assign9510_e9503,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9510_e9500: f64 = (locals.var_kvthowe * locals.var_temp0);
        let assign9510_e9501: f64 = (locals.var_vfb_p + assign9510_e9500);
        (assign9510_e9501,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9510_e9503;

        let (assign9520_e9515,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9520_e9511: f64 = (locals.var_kuowe * locals.var_temp0);
        let assign9520_e9512: f64 = (1.0 + assign9520_e9511);
        let assign9520_e9513: f64 = (locals.var_betn_p * assign9520_e9512);
        (assign9520_e9513,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9520_e9515;

        let (assign9530_e9525,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9530_e9522: f64 = (locals.var_kvthowe * locals.var_temp0);
        let assign9530_e9523: f64 = (locals.var_vfbedge_p + assign9530_e9522);
        (assign9530_e9523,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9530_e9525;

        let (assign9540_e9537,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard145 != 0.0)) {
        let assign9540_e9533: f64 = (locals.var_kuowe * locals.var_temp0);
        let assign9540_e9534: f64 = (1.0 + assign9540_e9533);
        let assign9540_e9535: f64 = (locals.var_betnedge_p * assign9540_e9534);
        (assign9540_e9535,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9540_e9537;

        locals.var_vfb_i = locals.var_vfb_p;

        locals.var_stvfb_i = locals.var_stvfb_p;

        locals.var_st2vfb_i = locals.var_st2vfb_p;

        locals.var_tox_i = locals.var_tox_p;

        locals.var_epsrox_i = locals.var_epsrox_p;

        let (assign9600_e9553,) = {
    if (locals.var_neff_p > 1e20) {
        let (assign9600_e9551,) = {
            if (locals.var_neff_p < 1e26) {
                (locals.var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9600_e9551,)
    } else {
        (1e20,)
    }
};
        locals.var_neff_i = assign9600_e9553;

        let (assign9610_e9559,) = {
    if (locals.var_gfacnud_p > 0.01) {
        (locals.var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        locals.var_gfacnud_i = assign9610_e9559;

        let (assign9620_e9565,) = {
    if (locals.var_vsbnud_p > 0.0) {
        (locals.var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        locals.var_vsbnud_i = assign9620_e9565;

        locals.var_dvsbnud_i = locals.var_dvsbnud_p;

        locals.var_dphib_i = locals.var_dphib_p;

        let (assign9650_e9573,) = {
    if (locals.var_np_p > 0.0) {
        (locals.var_np_p,)
    } else {
        (0.0,)
    }
};
        locals.var_np_i = assign9650_e9573;

        locals.var_toxov_i = locals.var_toxov_p;

        locals.var_toxovd_i = locals.var_toxovd_p;

        let (assign9680_e9586,) = {
    if (locals.var_nov_p > 1e23) {
        let (assign9680_e9584,) = {
            if (locals.var_nov_p < 1e27) {
                (locals.var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9680_e9584,)
    } else {
        (1e23,)
    }
};
        locals.var_nov_i = assign9680_e9586;

        let (assign9690_e9597,) = {
    if (locals.var_novd_p > 1e23) {
        let (assign9690_e9595,) = {
            if (locals.var_novd_p < 1e27) {
                (locals.var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9690_e9595,)
    } else {
        (1e23,)
    }
};
        locals.var_novd_i = assign9690_e9597;

        let (assign9700_e9603,) = {
    if (locals.var_ct_p > 0.0) {
        (locals.var_ct_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ct_i = assign9700_e9603;

        let (assign9710_e9614,) = {
    if (locals.var_ctb_p > 0.0) {
        let (assign9710_e9612,) = {
            if (locals.var_ctb_p < 0.5) {
                (locals.var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9710_e9612,)
    } else {
        (0.0,)
    }
};
        locals.var_ctb_i = assign9710_e9614;

        let (assign9720_e9625,) = {
    if (locals.var_ctg_p > 0.0) {
        let (assign9720_e9623,) = {
            if (locals.var_ctg_p < 1.0) {
                (locals.var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9720_e9623,)
    } else {
        (0.0,)
    }
};
        locals.var_ctg_i = assign9720_e9625;

        locals.var_stct_i = locals.var_stct_p;

        let (assign9740_e9632,) = {
    if (locals.var_cf_p > 0.0) {
        (locals.var_cf_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cf_i = assign9740_e9632;

        let (assign9750_e9643,) = {
    if (locals.var_cfb_p > 0.0) {
        let (assign9750_e9641,) = {
            if (locals.var_cfb_p < 1.0) {
                (locals.var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9750_e9641,)
    } else {
        (0.0,)
    }
};
        locals.var_cfb_i = assign9750_e9643;

        let (assign9760_e9649,) = {
    if (locals.var_cfd_p > 0.0) {
        (locals.var_cfd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfd_i = assign9760_e9649;

        let (assign9770_e9655,) = {
    if (locals.var_psce_p > 0.0) {
        (locals.var_psce_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psce_i = assign9770_e9655;

        let (assign9780_e9666,) = {
    if (locals.var_psceb_p > 0.0) {
        let (assign9780_e9664,) = {
            if (locals.var_psceb_p < 1.0) {
                (locals.var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9780_e9664,)
    } else {
        (0.0,)
    }
};
        locals.var_psceb_i = assign9780_e9666;

        let (assign9790_e9672,) = {
    if (locals.var_psced_p > 0.0) {
        (locals.var_psced_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psced_i = assign9790_e9672;

        let (assign9800_e9678,) = {
    if (locals.var_betn_p > 0.0) {
        (locals.var_betn_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betn_i = assign9800_e9678;

        locals.var_stbet_i = locals.var_stbet_p;

        let (assign9820_e9685,) = {
    if (locals.var_mue_p > 0.0) {
        (locals.var_mue_p,)
    } else {
        (0.0,)
    }
};
        locals.var_mue_i = assign9820_e9685;

        locals.var_stmue_i = locals.var_stmue_p;

        let (assign9840_e9692,) = {
    if (locals.var_themu_p > 0.0) {
        (locals.var_themu_p,)
    } else {
        (0.0,)
    }
};
        locals.var_themu_i = assign9840_e9692;

        locals.var_stthemu_i = locals.var_stthemu_p;

        let (assign9860_e9699,) = {
    if (locals.var_cs_p > 0.0) {
        (locals.var_cs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cs_i = assign9860_e9699;

        locals.var_stcs_i = locals.var_stcs_p;

        let (assign9880_e9706,) = {
    if (locals.var_thecs_p > 0.0) {
        (locals.var_thecs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thecs_i = assign9880_e9706;

        locals.var_stthecs_i = locals.var_stthecs_p;

        let (assign9900_e9713,) = {
    if (locals.var_xcor_p > 0.0) {
        (locals.var_xcor_p,)
    } else {
        (0.0,)
    }
};
        locals.var_xcor_i = assign9900_e9713;

        locals.var_stxcor_i = locals.var_stxcor_p;

        locals.var_feta_i = locals.var_feta_p;

        let (assign9930_e9721,) = {
    if (locals.var_rs_p > 0.0) {
        (locals.var_rs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rs_i = assign9930_e9721;

        locals.var_strs_i = locals.var_strs_p;

        let assign9950_e9725: f64 = (-0.5);
        let (assign9950_e9735,) = {
    if (locals.var_rsb_p > assign9950_e9725) {
        let (assign9950_e9732,) = {
            if (locals.var_rsb_p < 1.0) {
                (locals.var_rsb_p,)
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
        locals.var_rsb_i = assign9950_e9735;

        let assign9960_e9738: f64 = (-0.5);
        let (assign9960_e9743,) = {
    if (locals.var_rsg_p > assign9960_e9738) {
        (locals.var_rsg_p,)
    } else {
        let assign9960_e9742: f64 = (-0.5);
        (assign9960_e9742,)
    }
};
        locals.var_rsg_i = assign9960_e9743;

        let (assign9970_e9749,) = {
    if (locals.var_thesat_p > 0.0) {
        (locals.var_thesat_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesat_i = assign9970_e9749;

        locals.var_stthesat_i = locals.var_stthesat_p;

        let assign9990_e9753: f64 = (-0.5);
        let (assign9990_e9763,) = {
    if (locals.var_thesatb_p > assign9990_e9753) {
        let (assign9990_e9760,) = {
            if (locals.var_thesatb_p < 1.0) {
                (locals.var_thesatb_p,)
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
        locals.var_thesatb_i = assign9990_e9763;

        let assign10000_e9766: f64 = (-0.5);
        let (assign10000_e9771,) = {
    if (locals.var_thesatg_p > assign10000_e9766) {
        (locals.var_thesatg_p,)
    } else {
        let assign10000_e9770: f64 = (-0.5);
        (assign10000_e9770,)
    }
};
        locals.var_thesatg_i = assign10000_e9771;

        let (assign10010_e9777,) = {
    if (locals.var_thesatt_p > 0.01) {
        (locals.var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        locals.var_thesatt_i = assign10010_e9777;

        let (assign10020_e9783,) = {
    if (locals.var_ax_p > 2.0) {
        (locals.var_ax_p,)
    } else {
        (2.0,)
    }
};
        locals.var_ax_i = assign10020_e9783;

        let (assign10030_e9789,) = {
    if (locals.var_alp_p > 0.0) {
        (locals.var_alp_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp_i = assign10030_e9789;

        let (assign10040_e9795,) = {
    if (locals.var_alp1_p > 0.0) {
        (locals.var_alp1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1_i = assign10040_e9795;

        let (assign10050_e9801,) = {
    if (locals.var_alp2_p > 0.0) {
        (locals.var_alp2_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp2_i = assign10050_e9801;

        locals.var_vp_i = locals.var_vp_p;

        let (assign10070_e9808,) = {
    if (locals.var_a1_p > 0.0) {
        (locals.var_a1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a1_i = assign10070_e9808;

        locals.var_a2_i = locals.var_a2_p;

        locals.var_sta2_i = locals.var_sta2_p;

        let (assign10100_e9816,) = {
    if (locals.var_a3_p > 0.0) {
        (locals.var_a3_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a3_i = assign10100_e9816;

        let (assign10110_e9822,) = {
    if (locals.var_a4_p > 0.0) {
        (locals.var_a4_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a4_i = assign10110_e9822;

        let (assign10120_e9828,) = {
    if (locals.var_imaxii_p > 1e-12) {
        (locals.var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        locals.var_imaxii_i = assign10120_e9828;

        locals.var_gco_i = locals.var_gco_p;

        let (assign10140_e9835,) = {
    if (locals.var_iginv_p > 0.0) {
        (locals.var_iginv_p,)
    } else {
        (0.0,)
    }
};
        locals.var_iginv_i = assign10140_e9835;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10150_e9841,) = {
    if (locals.var_igov_p > 0.0) {
        (locals.var_igov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igov_i = assign10150_e9841;

        let (assign10160_e9847,) = {
    if (locals.var_igovd_p > 0.0) {
        (locals.var_igovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igovd_i = assign10160_e9847;

        locals.var_stig_i = locals.var_stig_p;

        locals.var_gc2_i = locals.var_gc2_p;

        locals.var_gc3_i = locals.var_gc3_p;

        locals.var_gc2ov_i = locals.var_gc2ov_p;

        locals.var_gc3ov_i = locals.var_gc3ov_p;

        locals.var_gc2ovd_i = locals.var_gc2ovd_p;

        locals.var_gc3ovd_i = locals.var_gc3ovd_p;

        locals.var_chib_i = locals.var_chib_p;

        let (assign10250_e9861,) = {
    if (locals.var_agidl_p > 0.0) {
        (locals.var_agidl_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidl_i = assign10250_e9861;

        let (assign10260_e9867,) = {
    if (locals.var_agidld_p > 0.0) {
        (locals.var_agidld_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidld_i = assign10260_e9867;

        locals.var_bgidl_i = locals.var_bgidl_p;

        locals.var_bgidld_i = locals.var_bgidld_p;

        locals.var_stbgidl_i = locals.var_stbgidl_p;

        locals.var_stbgidld_i = locals.var_stbgidld_p;

        locals.var_cgidl_i = locals.var_cgidl_p;

        locals.var_cgidld_i = locals.var_cgidld_p;

        let (assign10330_e9879,) = {
    if (locals.var_cox_p > 0.0) {
        (locals.var_cox_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cox_i = assign10330_e9879;

        locals.var_delvtac_i = locals.var_delvtac_p;

        let (assign10350_e9886,) = {
    if (locals.var_facneffac_p > 0.0) {
        (locals.var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_facneffac_i = assign10350_e9886;

        let (assign10360_e9892,) = {
    if (locals.var_thesatac_p > 0.0) {
        (locals.var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesatac_i = assign10360_e9892;

        let (assign10370_e9898,) = {
    if (locals.var_axac_p > 2.0) {
        (locals.var_axac_p,)
    } else {
        (2.0,)
    }
};
        locals.var_axac_i = assign10370_e9898;

        locals.var_alpac_i = locals.var_alpac_p;

        let (assign10390_e9905,) = {
    if (locals.var_alp1ac_p > 0.0) {
        (locals.var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1ac_i = assign10390_e9905;

        let (assign10400_e9911,) = {
    if (locals.var_cgov_p > 0.0) {
        (locals.var_cgov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgov_i = assign10400_e9911;

        let (assign10410_e9917,) = {
    if (locals.var_cgovd_p > 0.0) {
        (locals.var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgovd_i = assign10410_e9917;

        locals.var_fcgovacc_i = locals.var_fcgovacc_p;

        locals.var_fcgovaccd_i = locals.var_fcgovaccd_p;

        locals.var_cgovaccg_i = locals.var_cgovaccg_p;

        let (assign10450_e9926,) = {
    if (locals.var_cgbov_p > 0.0) {
        (locals.var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgbov_i = assign10450_e9926;

        let (assign10460_e9932,) = {
    if (locals.var_cinr_p > 0.0) {
        (locals.var_cinr_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinr_i = assign10460_e9932;

        let (assign10470_e9938,) = {
    if (locals.var_cinrd_p > 0.0) {
        (locals.var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinrd_i = assign10470_e9938;

        locals.var_dvfbinr_i = locals.var_dvfbinr_p;

        locals.var_fcinrdep_i = locals.var_fcinrdep_p;

        locals.var_fcinracc_i = locals.var_fcinracc_p;

        locals.var_axinr_i = locals.var_axinr_p;

        locals.var_fnt_i = locals.var_fnt_p;

        let (assign10550_e9961,) = {
    if (locals.var_fntexc_p > 0.0) {
        (locals.var_fntexc_p,)
    } else {
        (0.0,)
    }
};
        locals.var_fntexc_i = assign10550_e9961;

        locals.var_vfbedge_i = locals.var_vfbedge_p;

        locals.var_stvfbedge_i = locals.var_stvfbedge_p;

        locals.var_dphibedge_i = locals.var_dphibedge_p;

        let (assign10630_e9994,) = {
    if (locals.var_neffedge_p > 1e20) {
        let (assign10630_e9992,) = {
            if (locals.var_neffedge_p < 1e26) {
                (locals.var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10630_e9992,)
    } else {
        (1e20,)
    }
};
        locals.var_neffedge_i = assign10630_e9994;

        let (assign10640_e10000,) = {
    if (locals.var_ctedge_p > 0.0) {
        (locals.var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ctedge_i = assign10640_e10000;

        let (assign10650_e10006,) = {
    if (locals.var_betnedge_p > 0.0) {
        (locals.var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betnedge_i = assign10650_e10006;

        locals.var_stbetedge_i = locals.var_stbetedge_p;

        let (assign10670_e10013,) = {
    if (locals.var_psceedge_p > 0.0) {
        (locals.var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psceedge_i = assign10670_e10013;

        let (assign10680_e10024,) = {
    if (locals.var_pscebedge_p > 0.0) {
        let (assign10680_e10022,) = {
            if (locals.var_pscebedge_p < 1.0) {
                (locals.var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10680_e10022,)
    } else {
        (0.0,)
    }
};
        locals.var_pscebedge_i = assign10680_e10024;

        let (assign10690_e10030,) = {
    if (locals.var_pscededge_p > 0.0) {
        (locals.var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_pscededge_i = assign10690_e10030;

        let (assign10700_e10036,) = {
    if (locals.var_cfedge_p > 0.0) {
        (locals.var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfedge_i = assign10700_e10036;

        let (assign10710_e10047,) = {
    if (locals.var_cfbedge_p > 0.0) {
        let (assign10710_e10045,) = {
            if (locals.var_cfbedge_p < 1.0) {
                (locals.var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10710_e10045,)
    } else {
        (0.0,)
    }
};
        locals.var_cfbedge_i = assign10710_e10047;

        let (assign10720_e10053,) = {
    if (locals.var_cfdedge_p > 0.0) {
        (locals.var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfdedge_i = assign10720_e10053;

        let assign10850_e10088: f64 = (p.p31 * locals.var_nf_i);
        let (assign10850_e10095,) = {
    if (assign10850_e10088 > 0.0) {
        let assign10850_e10093: f64 = (p.p31 * locals.var_nf_i);
        (assign10850_e10093,)
    } else {
        (0.0,)
    }
};
        locals.var_mult_inst = assign10850_e10095;

        locals.var_factuo_i = p.p16;

        locals.var_delvto_i = p.p15;

        locals.var_factuoedge_i = p.p18;

        locals.var_delvtoedge_i = p.p17;

        let assign10900_e10102: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign10900_e10102;

        let (assign10910_e10106,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_toxov_i,)
    } else {
        (locals.var_toxovd_i,)
    }
};
        locals.var_toxovd_i = assign10910_e10106;

        let (assign10920_e10110,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_nov_i,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign10920_e10110;

        let (assign10930_e10114,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_agidl_i,)
    } else {
        (locals.var_agidld_i,)
    }
};
        locals.var_agidld_i = assign10930_e10114;

        let (assign10940_e10118,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_bgidl_i,)
    } else {
        (locals.var_bgidld_i,)
    }
};
        locals.var_bgidld_i = assign10940_e10118;

        let (assign10950_e10122,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_stbgidl_i,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign10950_e10122;

        let (assign10960_e10126,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_cgidl_i,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign10960_e10126;

        let (assign10970_e10130,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_igov_i,)
    } else {
        (locals.var_igovd_i,)
    }
};
        locals.var_igovd_i = assign10970_e10130;

        let (assign10980_e10134,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_gc2ov_i,)
    } else {
        (locals.var_gc2ovd_i,)
    }
};
        locals.var_gc2ovd_i = assign10980_e10134;

        let (assign10990_e10138,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_gc3ov_i,)
    } else {
        (locals.var_gc3ovd_i,)
    }
};
        locals.var_gc3ovd_i = assign10990_e10138;

        let (assign11000_e10142,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_cgov_i,)
    } else {
        (locals.var_cgovd_i,)
    }
};
        locals.var_cgovd_i = assign11000_e10142;

        let (assign11010_e10146,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_fcgovacc_i,)
    } else {
        (locals.var_fcgovaccd_i,)
    }
};
        locals.var_fcgovaccd_i = assign11010_e10146;

        let (assign11020_e10150,) = {
    if (locals.var_guard147 != 0.0) {
        (locals.var_cinr_i,)
    } else {
        (locals.var_cinrd_i,)
    }
};
        locals.var_cinrd_i = assign11020_e10150;

        let assign11040_e10157: f64 = (8.8541878176e-12 * locals.var_epsrox_i);
        locals.var_epsox = assign11040_e10157;

        let assign11050_e10160: f64 = (locals.var_epsox / locals.var_tox_i);
        locals.var_coxprime = assign11050_e10160;

        let assign11060_e10163: f64 = (locals.var_tox_i * locals.var_tox_i);
        locals.var_tox_sq = assign11060_e10163;

        let assign11070_e10166: f64 = (locals.var_coxprime / 1.6021918e-19);
        locals.var_cox_over_q = assign11070_e10166;

        let assign11080_e10169: f64 = (locals.var_facneffac_i * locals.var_neff_i);
        locals.var_neffac_i = assign11080_e10169;

        let (assign11090_e10180,) = {
    if (locals.var_neffac_i > 1e20) {
        let (assign11090_e10178,) = {
            if (locals.var_neffac_i < 1e26) {
                (locals.var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11090_e10178,)
    } else {
        (1e20,)
    }
};
        locals.var_neffac_i = assign11090_e10180;

        locals.var_qq = 0.0;

        let assign11110_e10184: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign11110_e10184;

        let (assign11120_e10196,) = {
    if (locals.var_guard148 != 0.0) {
        let assign11120_e10188: f64 = (0.4 * 5.951993);
        let assign11120_e10190: f64 = (assign11120_e10188 * p.p51);
        let assign11120_e10193: f64 = (locals.var_coxprime).powf(0.6666666666666666);
        let assign11120_e10194: f64 = (assign11120_e10190 * assign11120_e10193);
        (assign11120_e10194,)
    } else {
        (locals.var_qq,)
    }
};
        locals.var_qq = assign11120_e10196;

        let assign11130_e10199: f64 = (-1.0);
        let assign11130_e10200: f64 = if locals.var_chnl_type == assign11130_e10199 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign11130_e10200;

        let (assign11140_e10210,) = {
    if ((locals.var_guard148 != 0.0) && (locals.var_guard149 != 0.0)) {
        let assign11140_e10206: f64 = (7.448711 / 5.951993);
        let assign11140_e10208: f64 = (assign11140_e10206 * locals.var_qq);
        (assign11140_e10208,)
    } else {
        (locals.var_qq,)
    }
};
        locals.var_qq = assign11140_e10210;

        let assign11150_e10213: f64 = (1e-8 * locals.var_coxprime);
        let assign11150_e10215: f64 = (assign11150_e10213 / locals.var_epssi);
        locals.var_e_eff0 = assign11150_e10215;

        let assign11160_e10218: f64 = (0.5 * locals.var_feta_i);
        locals.var_eta_mu = assign11160_e10218;

        locals.var_eta_mu1 = 0.5;

        let assign11180_e10222: f64 = (-1.0);
        let assign11180_e10223: f64 = if locals.var_chnl_type == assign11180_e10222 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign11180_e10223;

        let (assign11190_e10229,) = {
    if (locals.var_guard150 != 0.0) {
        let assign11190_e10227: f64 = (0.3333333333333333 * locals.var_feta_i);
        (assign11190_e10227,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign11190_e10229;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11200_e10233,) = {
    if (locals.var_guard150 != 0.0) {
        (0.3333333333333333,)
    } else {
        (locals.var_eta_mu1,)
    }
};
        locals.var_eta_mu1 = assign11200_e10233;

        let assign11210_e10236: f64 = (-2.0);
        let assign11210_e10238: f64 = (assign11210_e10236 / locals.var_ax_i);
        let assign11210_e10240: f64 = (assign11210_e10238 + 1.0);
        let assign11210_e10241: f64 = (2.0_f64).powf(assign11210_e10240);
        let assign11210_e10243: f64 = (assign11210_e10241 - 1.0);
        locals.var_temp = assign11210_e10243;

        let assign11220_e10246: f64 = (locals.var_temp - 1.0);
        let assign11220_e10249: f64 = (locals.var_temp - 1.0);
        let assign11220_e10250: f64 = (assign11220_e10246 * assign11220_e10249);
        let assign11220_e10253: f64 = (4.0 * locals.var_temp);
        let (assign11220_e10260,) = {
    if (assign11220_e10253 > 0.0001) {
        let assign11220_e10258: f64 = (4.0 * locals.var_temp);
        (assign11220_e10258,)
    } else {
        (0.0001,)
    }
};
        let assign11220_e10261: f64 = (assign11220_e10250 / assign11220_e10260);
        locals.var_ar = assign11220_e10261;

        let assign11230_e10264: f64 = (-2.0);
        let assign11230_e10266: f64 = (assign11230_e10264 / locals.var_axac_i);
        let assign11230_e10268: f64 = (assign11230_e10266 + 1.0);
        let assign11230_e10269: f64 = (2.0_f64).powf(assign11230_e10268);
        let assign11230_e10271: f64 = (assign11230_e10269 - 1.0);
        locals.var_temp = assign11230_e10271;

        let assign11240_e10274: f64 = (locals.var_temp - 1.0);
        let assign11240_e10277: f64 = (locals.var_temp - 1.0);
        let assign11240_e10278: f64 = (assign11240_e10274 * assign11240_e10277);
        let assign11240_e10281: f64 = (4.0 * locals.var_temp);
        let (assign11240_e10288,) = {
    if (assign11240_e10281 > 0.0001) {
        let assign11240_e10286: f64 = (4.0 * locals.var_temp);
        (assign11240_e10286,)
    } else {
        (0.0001,)
    }
};
        let assign11240_e10289: f64 = (assign11240_e10278 / assign11240_e10288);
        locals.var_arac = assign11240_e10289;

        let assign11250_e10292: f64 = (1.0 / locals.var_vp_i);
        locals.var_inv_vp = assign11250_e10292;

        let assign11260_e10295: f64 = (locals.var_epsox / locals.var_toxov_i);
        locals.var_coxovprime = assign11260_e10295;

        let assign11270_e10298: f64 = (locals.var_epsox / locals.var_toxovd_i);
        locals.var_coxovprime_d = assign11270_e10298;

        let assign11280_e10301: f64 = (2.0 * 1.6021918e-19);
        let assign11280_e10303: f64 = (assign11280_e10301 * locals.var_nov_i);
        let assign11280_e10305: f64 = (assign11280_e10303 * locals.var_epssi);
        let assign11280_e10307: f64 = (assign11280_e10305 * locals.var_inv_phita);
        let assign11280_e10308: f64 = (assign11280_e10307).sqrt();
        let assign11280_e10310: f64 = (assign11280_e10308 / locals.var_coxovprime);
        locals.var_gov_s = assign11280_e10310;

        let assign11290_e10313: f64 = (2.0 * 1.6021918e-19);
        let assign11290_e10315: f64 = (assign11290_e10313 * locals.var_novd_i);
        let assign11290_e10317: f64 = (assign11290_e10315 * locals.var_epssi);
        let assign11290_e10319: f64 = (assign11290_e10317 * locals.var_inv_phita);
        let assign11290_e10320: f64 = (assign11290_e10319).sqrt();
        let assign11290_e10322: f64 = (assign11290_e10320 / locals.var_coxovprime_d);
        locals.var_gov_d = assign11290_e10322;

        let assign11300_e10325: f64 = (locals.var_gov_s * locals.var_gov_s);
        locals.var_gov2_s = assign11300_e10325;

        let assign11310_e10328: f64 = (locals.var_gov_d * locals.var_gov_d);
        locals.var_gov2_d = assign11310_e10328;

        let assign11320_e10331: f64 = (locals.var_cgovaccg_i * 0.005);
        let assign11320_e10333: f64 = (assign11320_e10331 * locals.var_inv_phita);
        let assign11320_e10334: f64 = (assign11320_e10333).exp();
        let assign11320_e10336: f64 = (assign11320_e10334 - 1.0);
        let assign11320_e10337: f64 = (assign11320_e10336).ln();
        let assign11320_e10339: f64 = (assign11320_e10337 / locals.var_cgovaccg_i);
        let assign11320_e10342: f64 = (0.005 * locals.var_inv_phita);
        let assign11320_e10343: f64 = (assign11320_e10342).exp();
        let assign11320_e10345: f64 = (assign11320_e10343 - 1.0);
        let assign11320_e10346: f64 = (assign11320_e10345).ln();
        let assign11320_e10347: f64 = (assign11320_e10339 - assign11320_e10346);
        locals.var_dxgb_ov_th = assign11320_e10347;

        let assign11330_e10350: f64 = (0.5 * locals.var_gov_s);
        let assign11330_e10351: f64 = (assign11330_e10350).ln();
        let assign11330_e10353: f64 = (assign11330_e10351 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_s = assign11330_e10353;

        let assign11340_e10356: f64 = (0.5 * locals.var_gov_d);
        let assign11340_e10357: f64 = (assign11340_e10356).ln();
        let assign11340_e10359: f64 = (assign11340_e10357 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_d = assign11340_e10359;

        let assign11350_e10362: f64 = (1.0 / locals.var_gov_s);
        locals.var_inv_gov = assign11350_e10362;

        let assign11360_e10365: f64 = (3.1 * locals.var_gov_s);
        let assign11360_e10367: f64 = (assign11360_e10365 + 8.5);
        locals.var_sp_ov_eps = assign11360_e10367;

        let assign11370_e10370: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_s = assign11370_e10370;

        let assign11380_e10373: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11380_e10373;

        let assign11390_e10376: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign11390_e10376;

        let (assign11400_e10382,) = {
    if (locals.var_guard151 != 0.0) {
        let assign11400_e10380: f64 = (64.0 * locals.var_inv_gov);
        (assign11400_e10380,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11400_e10382;

        let assign11410_e10385: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign11410_e10385;

        let (assign11420_e10396,) = {
    if ((locals.var_guard151 == 0.0) && (locals.var_guard152 != 0.0)) {
        let assign11420_e10392: f64 = (22.0 * locals.var_inv_gov);
        let assign11420_e10394: f64 = (assign11420_e10392 + 3.0);
        (assign11420_e10394,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11420_e10396;

        let assign11430_e10399: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign11430_e10399;

        let (assign11440_e10414,) = {
    if (((locals.var_guard151 == 0.0) && (locals.var_guard152 == 0.0)) && (locals.var_guard153 != 0.0)) {
        let assign11440_e10408: f64 = (-7.2);
        let assign11440_e10410: f64 = (assign11440_e10408 * locals.var_inv_gov);
        let assign11440_e10412: f64 = (assign11440_e10410 + 15.5);
        (assign11440_e10412,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11440_e10414;

        let (assign11450_e10425,) = {
    if (((locals.var_guard151 == 0.0) && (locals.var_guard152 == 0.0)) && (locals.var_guard153 == 0.0)) {
        (locals.var_gov_s,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11450_e10425;

        let assign11460_e10429: f64 = (locals.var_gov2_s * 0.5);
        let assign11460_e10430: f64 = (locals.var_sp_ov_delta + assign11460_e10429);
        let assign11460_e10435: f64 = (locals.var_gov2_s * 0.25);
        let assign11460_e10436: f64 = (locals.var_sp_ov_delta + assign11460_e10435);
        let assign11460_e10438: f64 = (assign11460_e10436 + locals.var_sp_ov_a_s);
        let assign11460_e10439: f64 = (assign11460_e10438).sqrt();
        let assign11460_e10440: f64 = (locals.var_gov_s * assign11460_e10439);
        let assign11460_e10441: f64 = (assign11460_e10430 - assign11460_e10440);
        locals.var_sp_ov_delta1_s = assign11460_e10441;

        let assign11470_e10444: f64 = (1.0 / locals.var_gov_d);
        locals.var_inv_gov = assign11470_e10444;

        let assign11480_e10447: f64 = (3.1 * locals.var_gov_d);
        let assign11480_e10449: f64 = (assign11480_e10447 + 8.5);
        locals.var_sp_ov_eps = assign11480_e10449;

        let assign11490_e10452: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_d = assign11490_e10452;

        let assign11500_e10455: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11500_e10455;

        let assign11510_e10458: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign11510_e10458;

        let (assign11520_e10464,) = {
    if (locals.var_guard154 != 0.0) {
        let assign11520_e10462: f64 = (64.0 * locals.var_inv_gov);
        (assign11520_e10462,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11520_e10464;

        let assign11530_e10467: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign11530_e10467;

        let (assign11540_e10478,) = {
    if ((locals.var_guard154 == 0.0) && (locals.var_guard155 != 0.0)) {
        let assign11540_e10474: f64 = (22.0 * locals.var_inv_gov);
        let assign11540_e10476: f64 = (assign11540_e10474 + 3.0);
        (assign11540_e10476,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11540_e10478;

        let assign11550_e10481: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign11550_e10481;

        let (assign11560_e10496,) = {
    if (((locals.var_guard154 == 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 != 0.0)) {
        let assign11560_e10490: f64 = (-7.2);
        let assign11560_e10492: f64 = (assign11560_e10490 * locals.var_inv_gov);
        let assign11560_e10494: f64 = (assign11560_e10492 + 15.5);
        (assign11560_e10494,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11560_e10496;

        let (assign11570_e10507,) = {
    if (((locals.var_guard154 == 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 == 0.0)) {
        (locals.var_gov_d,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11570_e10507;

        let assign11580_e10511: f64 = (locals.var_gov2_d * 0.5);
        let assign11580_e10512: f64 = (locals.var_sp_ov_delta + assign11580_e10511);
        let assign11580_e10517: f64 = (locals.var_gov2_d * 0.25);
        let assign11580_e10518: f64 = (locals.var_sp_ov_delta + assign11580_e10517);
        let assign11580_e10520: f64 = (assign11580_e10518 + locals.var_sp_ov_a_d);
        let assign11580_e10521: f64 = (assign11580_e10520).sqrt();
        let assign11580_e10522: f64 = (locals.var_gov_d * assign11580_e10521);
        let assign11580_e10523: f64 = (assign11580_e10512 - assign11580_e10522);
        locals.var_sp_ov_delta1_d = assign11580_e10523;

        let assign11590_e10526: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign11590_e10529: f64 = (2.0 * locals.var_phit);
        let assign11590_e10533: f64 = (-0.75);
        let assign11590_e10534: f64 = (locals.var_phibfac).powf(assign11590_e10533);
        let assign11590_e10535: f64 = (locals.var_neff_i * assign11590_e10534);
        let assign11590_e10537: f64 = (assign11590_e10535 * 4e-26);
        let assign11590_e10538: f64 = (assign11590_e10537).ln();
        let assign11590_e10539: f64 = (assign11590_e10529 * assign11590_e10538);
        let assign11590_e10540: f64 = (assign11590_e10526 + assign11590_e10539);
        locals.var_phib_dc = assign11590_e10540;

        let (assign11600_e10546,) = {
    if (locals.var_phib_dc > 0.05) {
        (locals.var_phib_dc,)
    } else {
        (0.05,)
    }
};
        locals.var_phib_dc = assign11600_e10546;

        let assign11610_e10549: f64 = (2.0 * 1.6021918e-19);
        let assign11610_e10551: f64 = (assign11610_e10549 * locals.var_neff_i);
        let assign11610_e10553: f64 = (assign11610_e10551 * locals.var_epssi);
        let assign11610_e10555: f64 = (assign11610_e10553 * locals.var_inv_phit);
        let assign11610_e10556: f64 = (assign11610_e10555).sqrt();
        let assign11610_e10558: f64 = (assign11610_e10556 / locals.var_coxprime);
        locals.var_g_0_dc = assign11610_e10558;

        locals.var_kp = 0.0;

        locals.var_np = 0.0;

        let assign11640_e10563: f64 = if locals.var_np_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign11640_e10563;

        let (assign11650_e10569,) = {
    if (locals.var_guard157 != 0.0) {
        let assign11650_e10567: f64 = (80000000.0 / locals.var_tox_sq);
        (assign11650_e10567,)
    } else {
        (locals.var_arg2max,)
    }
};
        locals.var_arg2max = assign11650_e10569;

        let (assign11660_e10578,) = {
    if (locals.var_guard157 != 0.0) {
        let (assign11660_e10576,) = {
            if (locals.var_np_i > locals.var_arg2max) {
                (locals.var_np_i,)
            } else {
                (locals.var_arg2max,)
            }
        };
        (assign11660_e10576,)
    } else {
        (locals.var_np,)
    }
};
        locals.var_np = assign11660_e10578;

        let (assign11670_e10587,) = {
    if (locals.var_guard157 != 0.0) {
        let (assign11670_e10585,) = {
            if (5e24 > locals.var_np) {
                (5e24,)
            } else {
                (locals.var_np,)
            }
        };
        (assign11670_e10585,)
    } else {
        (locals.var_np,)
    }
};
        locals.var_np = assign11670_e10587;

        let (assign11680_e10603,) = {
    if (locals.var_guard157 != 0.0) {
        let assign11680_e10591: f64 = (2.0 * locals.var_coxprime);
        let assign11680_e10593: f64 = (assign11680_e10591 * locals.var_coxprime);
        let assign11680_e10595: f64 = (assign11680_e10593 * locals.var_phit);
        let assign11680_e10598: f64 = (1.6021918e-19 * locals.var_np);
        let assign11680_e10600: f64 = (assign11680_e10598 * locals.var_epssi);
        let assign11680_e10601: f64 = (assign11680_e10595 / assign11680_e10600);
        (assign11680_e10601,)
    } else {
        (locals.var_kp,)
    }
};
        locals.var_kp = assign11680_e10603;

        let assign11690_e10606: f64 = (100.0 * locals.var_phit);
        let assign11690_e10608: f64 = (assign11690_e10606 * locals.var_phit);
        locals.var_qlim2 = assign11690_e10608;

        let assign11700_e10611: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign11700_e10611;

        let (assign11710_e10622,) = {
    if (locals.var_guard158 != 0.0) {
        let assign11710_e10615: f64 = (locals.var_phit * locals.var_g_0_dc);
        let assign11710_e10617: f64 = (assign11710_e10615 * locals.var_g_0_dc);
        let assign11710_e10619: f64 = (assign11710_e10617 * locals.var_phib_dc);
        let assign11710_e10620: f64 = (assign11710_e10619).sqrt();
        (assign11710_e10620,)
    } else {
        (locals.var_qb0,)
    }
};
        locals.var_qb0 = assign11710_e10622;

        let (assign11720_e10632,) = {
    if (locals.var_guard158 != 0.0) {
        let assign11720_e10626: f64 = (0.75 * locals.var_qq);
        let assign11720_e10629: f64 = (locals.var_qb0).powf(0.6666666666666666);
        let assign11720_e10630: f64 = (assign11720_e10626 * assign11720_e10629);
        (assign11720_e10630,)
    } else {
        (locals.var_dphibq,)
    }
};
        locals.var_dphibq = assign11720_e10632;

        let (assign11730_e10638,) = {
    if (locals.var_guard158 != 0.0) {
        let assign11730_e10636: f64 = (locals.var_phib_dc + locals.var_dphibq);
        (assign11730_e10636,)
    } else {
        (locals.var_phib_dc,)
    }
};
        locals.var_phib_dc = assign11730_e10638;

        let (assign11740_e10652,) = {
    if (locals.var_guard158 != 0.0) {
        let assign11740_e10644: f64 = (2.0 * 0.6666666666666666);
        let assign11740_e10646: f64 = (assign11740_e10644 * locals.var_dphibq);
        let assign11740_e10648: f64 = (assign11740_e10646 / locals.var_qb0);
        let assign11740_e10649: f64 = (1.0 + assign11740_e10648);
        let assign11740_e10650: f64 = (locals.var_g_0_dc * assign11740_e10649);
        (assign11740_e10650,)
    } else {
        (locals.var_g_0_dc,)
    }
};
        locals.var_g_0_dc = assign11740_e10652;

        let assign11750_e10654: f64 = (locals.var_phib_dc).sqrt();
        locals.var_sqrt_phib_dc = assign11750_e10654;

        let assign11760_e10657: f64 = (0.95 * locals.var_phib_dc);
        locals.var_phix_dc = assign11760_e10657;

        let assign11770_e10660: f64 = (0.0025 * locals.var_phib_dc);
        let assign11770_e10662: f64 = (assign11770_e10660 * locals.var_phib_dc);
        locals.var_aphi_dc = assign11770_e10662;

        locals.var_bphi_dc = locals.var_aphi_dc;

        let assign11790_e10666: f64 = (locals.var_bphi_dc).sqrt();
        let assign11790_e10667: f64 = (0.5 * assign11790_e10666);
        locals.var_phix2 = assign11790_e10667;

        let assign11800_e10671: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11800_e10673: f64 = assign11800_e10671;
        let assign11800_e10676: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11800_e10678: f64 = assign11800_e10676;
        let assign11800_e10681: f64 = (locals.var_phix_dc - locals.var_phix2);
        let assign11800_e10683: f64 = assign11800_e10681;
        let assign11800_e10684: f64 = (assign11800_e10678 * assign11800_e10683);
        let assign11800_e10686: f64 = (assign11800_e10684 + locals.var_aphi_dc);
        let assign11800_e10687: f64 = (assign11800_e10686).sqrt();
        let assign11800_e10688: f64 = (assign11800_e10673 - assign11800_e10687);
        let assign11800_e10689: f64 = (0.5 * assign11800_e10688);
        locals.var_phix1_dc = assign11800_e10689;

        let assign11810_e10693: f64 = (locals.var_phib_dc + locals.var_eg);
        let assign11810_e10694: f64 = (0.5 * assign11810_e10693);
        locals.var_alpha_b = assign11810_e10694;

        let assign11820_e10697: f64 = (locals.var_vsbnud_i + locals.var_phib_dc);
        let assign11820_e10698: f64 = (assign11820_e10697).sqrt();
        let assign11820_e10700: f64 = (assign11820_e10698 - locals.var_sqrt_phib_dc);
        locals.var_us1 = assign11820_e10700;

        let assign11830_e10703: f64 = (locals.var_vsbnud_i + locals.var_dvsbnud_i);
        let assign11830_e10705: f64 = (assign11830_e10703 + locals.var_phib_dc);
        let assign11830_e10706: f64 = (assign11830_e10705).sqrt();
        let assign11830_e10708: f64 = (assign11830_e10706 - locals.var_sqrt_phib_dc);
        let assign11830_e10710: f64 = (assign11830_e10708 - locals.var_us1);
        locals.var_us21 = assign11830_e10710;

        let assign11840_e10713: f64 = (locals.var_eg + locals.var_dphib_i);
        let assign11840_e10715: f64 = (assign11840_e10713 + locals.var_delvtac_i);
        let assign11840_e10718: f64 = (2.0 * locals.var_phit);
        let assign11840_e10722: f64 = (-0.75);
        let assign11840_e10723: f64 = (locals.var_phibfac).powf(assign11840_e10722);
        let assign11840_e10724: f64 = (locals.var_neffac_i * assign11840_e10723);
        let assign11840_e10726: f64 = (assign11840_e10724 * 4e-26);
        let assign11840_e10727: f64 = (assign11840_e10726).ln();
        let assign11840_e10728: f64 = (assign11840_e10718 * assign11840_e10727);
        let assign11840_e10729: f64 = (assign11840_e10715 + assign11840_e10728);
        locals.var_phib_ac = assign11840_e10729;

        let (assign11850_e10735,) = {
    if (locals.var_phib_ac > 0.05) {
        (locals.var_phib_ac,)
    } else {
        (0.05,)
    }
};
        locals.var_phib_ac = assign11850_e10735;

        let assign11860_e10738: f64 = (2.0 * 1.6021918e-19);
        let assign11860_e10740: f64 = (assign11860_e10738 * locals.var_neffac_i);
        let assign11860_e10742: f64 = (assign11860_e10740 * locals.var_epssi);
        let assign11860_e10744: f64 = (assign11860_e10742 * locals.var_inv_phit);
        let assign11860_e10745: f64 = (assign11860_e10744).sqrt();
        let assign11860_e10747: f64 = (assign11860_e10745 / locals.var_coxprime);
        locals.var_g_0_ac = assign11860_e10747;

        let assign11870_e10750: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign11870_e10750;

        let (assign11880_e10761,) = {
    if (locals.var_guard159 != 0.0) {
        let assign11880_e10754: f64 = (locals.var_phit * locals.var_g_0_ac);
        let assign11880_e10756: f64 = (assign11880_e10754 * locals.var_g_0_ac);
        let assign11880_e10758: f64 = (assign11880_e10756 * locals.var_phib_ac);
        let assign11880_e10759: f64 = (assign11880_e10758).sqrt();
        (assign11880_e10759,)
    } else {
        (locals.var_qb0,)
    }
};
        locals.var_qb0 = assign11880_e10761;

        let (assign11890_e10771,) = {
    if (locals.var_guard159 != 0.0) {
        let assign11890_e10765: f64 = (0.75 * locals.var_qq);
        let assign11890_e10768: f64 = (locals.var_qb0).powf(0.6666666666666666);
        let assign11890_e10769: f64 = (assign11890_e10765 * assign11890_e10768);
        (assign11890_e10769,)
    } else {
        (locals.var_dphibq,)
    }
};
        locals.var_dphibq = assign11890_e10771;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11900_e10777,) = {
    if (locals.var_guard159 != 0.0) {
        let assign11900_e10775: f64 = (locals.var_phib_ac + locals.var_dphibq);
        (assign11900_e10775,)
    } else {
        (locals.var_phib_ac,)
    }
};
        locals.var_phib_ac = assign11900_e10777;

        let (assign11910_e10791,) = {
    if (locals.var_guard159 != 0.0) {
        let assign11910_e10783: f64 = (2.0 * 0.6666666666666666);
        let assign11910_e10785: f64 = (assign11910_e10783 * locals.var_dphibq);
        let assign11910_e10787: f64 = (assign11910_e10785 / locals.var_qb0);
        let assign11910_e10788: f64 = (1.0 + assign11910_e10787);
        let assign11910_e10789: f64 = (locals.var_g_0_ac * assign11910_e10788);
        (assign11910_e10789,)
    } else {
        (locals.var_g_0_ac,)
    }
};
        locals.var_g_0_ac = assign11910_e10791;

        let assign11920_e10794: f64 = (0.95 * locals.var_phib_ac);
        locals.var_phix_ac = assign11920_e10794;

        let assign11930_e10797: f64 = (0.0025 * locals.var_phib_ac);
        let assign11930_e10799: f64 = (assign11930_e10797 * locals.var_phib_ac);
        locals.var_aphi_ac = assign11930_e10799;

        locals.var_bphi_ac = locals.var_aphi_ac;

        let assign11950_e10803: f64 = (locals.var_bphi_ac).sqrt();
        let assign11950_e10804: f64 = (0.5 * assign11950_e10803);
        locals.var_phix2 = assign11950_e10804;

        let assign11960_e10808: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign11960_e10810: f64 = assign11960_e10808;
        let assign11960_e10813: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign11960_e10815: f64 = assign11960_e10813;
        let assign11960_e10818: f64 = (locals.var_phix_ac - locals.var_phix2);
        let assign11960_e10820: f64 = assign11960_e10818;
        let assign11960_e10821: f64 = (assign11960_e10815 * assign11960_e10820);
        let assign11960_e10823: f64 = (assign11960_e10821 + locals.var_aphi_ac);
        let assign11960_e10824: f64 = (assign11960_e10823).sqrt();
        let assign11960_e10825: f64 = (assign11960_e10810 - assign11960_e10824);
        let assign11960_e10826: f64 = (0.5 * assign11960_e10825);
        locals.var_phix1_ac = assign11960_e10826;

        let assign11970_e10830: f64 = (locals.var_stvfb_i * locals.var_delt);
        let assign11970_e10834: f64 = (locals.var_st2vfb_i * locals.var_delt);
        let assign11970_e10835: f64 = (1.0 + assign11970_e10834);
        let assign11970_e10836: f64 = (assign11970_e10830 * assign11970_e10835);
        let assign11970_e10837: f64 = (locals.var_vfb_i + assign11970_e10836);
        let assign11970_e10839: f64 = (assign11970_e10837 + locals.var_delvto_i);
        locals.var_vfb_t = assign11970_e10839;

        let assign11980_e10842: f64 = (locals.var_stct_i * locals.var_ln_rtn);
        let assign11980_e10843: f64 = (assign11980_e10842).exp();
        locals.var_tf_ct = assign11980_e10843;

        let assign11990_e10846: f64 = (locals.var_ct_i * locals.var_tf_ct);
        locals.var_ct_t = assign11990_e10846;

        let assign12000_e10849: f64 = (locals.var_ctg_i / locals.var_rtn);
        locals.var_ctg_t = assign12000_e10849;

        let assign12010_e10852: f64 = (locals.var_stbet_i * locals.var_ln_rtn);
        let assign12010_e10853: f64 = (assign12010_e10852).exp();
        locals.var_tf_bet = assign12010_e10853;

        let assign12020_e10856: f64 = (locals.var_betn_i * locals.var_tf_bet);
        locals.var_betn_t = assign12020_e10856;

        let assign12030_e10859: f64 = (locals.var_factuo_i * locals.var_betn_t);
        let assign12030_e10861: f64 = (assign12030_e10859 * locals.var_coxprime);
        locals.var_bet_i = assign12030_e10861;

        let assign12040_e10865: f64 = (locals.var_stthemu_i * locals.var_ln_rtn);
        let assign12040_e10866: f64 = (assign12040_e10865).exp();
        let assign12040_e10867: f64 = (locals.var_themu_i * assign12040_e10866);
        locals.var_themu_t = assign12040_e10867;

        let assign12050_e10870: f64 = (locals.var_stmue_i * locals.var_ln_rtn);
        let assign12050_e10871: f64 = (assign12050_e10870).exp();
        locals.var_tf_mue = assign12050_e10871;

        let assign12060_e10874: f64 = (locals.var_mue_i * locals.var_tf_mue);
        locals.var_mue_t = assign12060_e10874;

        let assign12070_e10878: f64 = (locals.var_stthecs_i * locals.var_ln_rtn);
        let assign12070_e10879: f64 = (assign12070_e10878).exp();
        let assign12070_e10880: f64 = (locals.var_thecs_i * assign12070_e10879);
        locals.var_thecs_t = assign12070_e10880;

        let assign12080_e10883: f64 = (locals.var_stcs_i * locals.var_ln_rtn);
        let assign12080_e10884: f64 = (assign12080_e10883).exp();
        locals.var_tf_cs = assign12080_e10884;

        let assign12090_e10887: f64 = (locals.var_cs_i * locals.var_tf_cs);
        locals.var_cs_t = assign12090_e10887;

        let assign12100_e10890: f64 = (locals.var_stxcor_i * locals.var_ln_rtn);
        let assign12100_e10891: f64 = (assign12100_e10890).exp();
        locals.var_tf_xcor = assign12100_e10891;

        let assign12110_e10894: f64 = (locals.var_xcor_i * locals.var_tf_xcor);
        locals.var_xcor_t = assign12110_e10894;

        let assign12120_e10897: f64 = (locals.var_strs_i * locals.var_ln_rtn);
        let assign12120_e10898: f64 = (assign12120_e10897).exp();
        locals.var_tf_ther = assign12120_e10898;

        let assign12130_e10901: f64 = (locals.var_rs_i * locals.var_tf_ther);
        locals.var_rs_t = assign12130_e10901;

        let assign12140_e10904: f64 = (2.0 * locals.var_bet_i);
        let assign12140_e10906: f64 = (assign12140_e10904 * locals.var_rs_t);
        locals.var_ther_i = assign12140_e10906;

        let assign12150_e10909: f64 = (locals.var_stthesat_i * locals.var_ln_rtn);
        let assign12150_e10910: f64 = (assign12150_e10909).exp();
        locals.var_tf_thesat = assign12150_e10910;

        let assign12160_e10913: f64 = (locals.var_thesat_i * locals.var_tf_thesat);
        locals.var_thesat_t = assign12160_e10913;

        let assign12170_e10916: f64 = (locals.var_thesatac_i * locals.var_tf_thesat);
        locals.var_thesatac_t = assign12170_e10916;

        let assign12180_e10919: f64 = (-locals.var_sta2_i);
        let assign12180_e10921: f64 = (assign12180_e10919 * locals.var_ln_rtn);
        let assign12180_e10922: f64 = (assign12180_e10921).exp();
        let assign12180_e10923: f64 = (locals.var_a2_i * assign12180_e10922);
        locals.var_a2_t = assign12180_e10923;

        let assign12190_e10926: f64 = (locals.var_fnt_i * 4.0);
        let assign12190_e10928: f64 = (assign12190_e10926 * 1.3806505e-23);
        let assign12190_e10930: f64 = (assign12190_e10928 * locals.var_tkd);
        locals.var_nt = assign12190_e10930;

        let assign12210_e10944: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard160 = assign12210_e10944;

        let (assign12220_e10954,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12220_e10949: f64 = (locals.var_stvfbedge_i * locals.var_delt);
        let assign12220_e10950: f64 = (locals.var_vfbedge_i + assign12220_e10949);
        let assign12220_e10952: f64 = (assign12220_e10950 + locals.var_delvtoedge_i);
        (assign12220_e10952,)
    } else {
        (locals.var_vfbedge_t,)
    }
};
        locals.var_vfbedge_t = assign12220_e10954;

        let (assign12230_e10961,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12230_e10958: f64 = (locals.var_stbetedge_i * locals.var_ln_rtn);
        let assign12230_e10959: f64 = (assign12230_e10958).exp();
        (assign12230_e10959,)
    } else {
        (locals.var_tf_betedge,)
    }
};
        locals.var_tf_betedge = assign12230_e10961;

        let (assign12240_e10967,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12240_e10965: f64 = (locals.var_betnedge_i * locals.var_tf_betedge);
        (assign12240_e10965,)
    } else {
        (locals.var_betnedge_t,)
    }
};
        locals.var_betnedge_t = assign12240_e10967;

        let (assign12250_e10975,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12250_e10971: f64 = (locals.var_factuoedge_i * locals.var_betnedge_t);
        let assign12250_e10973: f64 = (assign12250_e10971 * locals.var_coxprime);
        (assign12250_e10973,)
    } else {
        (locals.var_betedge_i,)
    }
};
        locals.var_betedge_i = assign12250_e10975;

        let (assign12260_e10985,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12260_e10981: f64 = (locals.var_ctedge_i * locals.var_rtn);
        let assign12260_e10982: f64 = (1.0 + assign12260_e10981);
        let assign12260_e10983: f64 = (locals.var_phit * assign12260_e10982);
        (assign12260_e10983,)
    } else {
        (locals.var_phit0edge,)
    }
};
        locals.var_phit0edge = assign12260_e10985;

        let (assign12270_e11005,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12270_e10989: f64 = (locals.var_eg + locals.var_dphibedge_i);
        let assign12270_e10992: f64 = (2.0 * locals.var_phit0edge);
        let assign12270_e10996: f64 = (-0.75);
        let assign12270_e10997: f64 = (locals.var_phibfac).powf(assign12270_e10996);
        let assign12270_e10998: f64 = (locals.var_neffedge_i * assign12270_e10997);
        let assign12270_e11000: f64 = (assign12270_e10998 * 4e-26);
        let assign12270_e11001: f64 = (assign12270_e11000).ln();
        let assign12270_e11002: f64 = (assign12270_e10992 * assign12270_e11001);
        let assign12270_e11003: f64 = (assign12270_e10989 + assign12270_e11002);
        (assign12270_e11003,)
    } else {
        (locals.var_phibedge,)
    }
};
        locals.var_phibedge = assign12270_e11005;

        let (assign12280_e11014,) = {
    if (locals.var_guard160 != 0.0) {
        let (assign12280_e11012,) = {
            if (locals.var_phibedge > 0.05) {
                (locals.var_phibedge,)
            } else {
                (0.05,)
            }
        };
        (assign12280_e11012,)
    } else {
        (locals.var_phibedge,)
    }
};
        locals.var_phibedge = assign12280_e11014;

        let (assign12290_e11029,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12290_e11018: f64 = (2.0 * 1.6021918e-19);
        let assign12290_e11020: f64 = (assign12290_e11018 * locals.var_neffedge_i);
        let assign12290_e11022: f64 = (assign12290_e11020 * locals.var_epssi);
        let assign12290_e11024: f64 = (assign12290_e11022 * locals.var_inv_phit);
        let assign12290_e11025: f64 = (assign12290_e11024).sqrt();
        let assign12290_e11027: f64 = (assign12290_e11025 / locals.var_coxprime);
        (assign12290_e11027,)
    } else {
        (locals.var_gfedge,)
    }
};
        locals.var_gfedge = assign12290_e11029;

        let (assign12300_e11035,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12300_e11033: f64 = (locals.var_gfedge * locals.var_gfedge);
        (assign12300_e11033,)
    } else {
        (locals.var_gfedge2,)
    }
};
        locals.var_gfedge2 = assign12300_e11035;

        let (assign12310_e11040,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12310_e11038: f64 = (locals.var_gfedge2).ln();
        (assign12310_e11038,)
    } else {
        (locals.var_lngfedge2,)
    }
};
        locals.var_lngfedge2 = assign12310_e11040;

        let (assign12320_e11046,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12320_e11044: f64 = (0.95 * locals.var_phibedge);
        (assign12320_e11044,)
    } else {
        (locals.var_phixedge,)
    }
};
        locals.var_phixedge = assign12320_e11046;

        let (assign12330_e11054,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12330_e11050: f64 = (0.0025 * locals.var_phibedge);
        let assign12330_e11052: f64 = (assign12330_e11050 * locals.var_phibedge);
        (assign12330_e11052,)
    } else {
        (locals.var_aphiedge,)
    }
};
        locals.var_aphiedge = assign12330_e11054;

        let (assign12340_e11058,) = {
    if (locals.var_guard160 != 0.0) {
        (locals.var_aphiedge,)
    } else {
        (locals.var_bphiedge,)
    }
};
        locals.var_bphiedge = assign12340_e11058;

        let (assign12350_e11065,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12350_e11062: f64 = (locals.var_bphiedge).sqrt();
        let assign12350_e11063: f64 = (0.5 * assign12350_e11062);
        (assign12350_e11063,)
    } else {
        (locals.var_phix2edge,)
    }
};
        locals.var_phix2edge = assign12350_e11065;

        let (assign12360_e11090,) = {
    if (locals.var_guard160 != 0.0) {
        let assign12360_e11070: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign12360_e11072: f64 = assign12360_e11070;
        let assign12360_e11075: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign12360_e11077: f64 = assign12360_e11075;
        let assign12360_e11080: f64 = (locals.var_phixedge - locals.var_phix2edge);
        let assign12360_e11082: f64 = assign12360_e11080;
        let assign12360_e11083: f64 = (assign12360_e11077 * assign12360_e11082);
        let assign12360_e11085: f64 = (assign12360_e11083 + locals.var_aphiedge);
        let assign12360_e11086: f64 = (assign12360_e11085).sqrt();
        let assign12360_e11087: f64 = (assign12360_e11072 - assign12360_e11086);
        let assign12360_e11088: f64 = (0.5 * assign12360_e11087);
        (assign12360_e11088,)
    } else {
        (locals.var_phix1edge,)
    }
};
        locals.var_phix1edge = assign12360_e11090;

        let (assign12390_e11115,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_vfbedge_t,)
    }
};
        locals.var_vfbedge_t = assign12390_e11115;

        let (assign12400_e11120,) = {
    if (locals.var_guard160 == 0.0) {
        (1.0,)
    } else {
        (locals.var_tf_betedge,)
    }
};
        locals.var_tf_betedge = assign12400_e11120;

        let (assign12410_e11125,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_betnedge_t,)
    }
};
        locals.var_betnedge_t = assign12410_e11125;

        let (assign12420_e11130,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_betedge_i,)
    }
};
        locals.var_betedge_i = assign12420_e11130;

        let (assign12430_e11135,) = {
    if (locals.var_guard160 == 0.0) {
        (locals.var_phit,)
    } else {
        (locals.var_phit0edge,)
    }
};
        locals.var_phit0edge = assign12430_e11135;

        let (assign12440_e11140,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phibedge,)
    }
};
        locals.var_phibedge = assign12440_e11140;

        let (assign12450_e11145,) = {
    if (locals.var_guard160 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gfedge,)
    }
};
        locals.var_gfedge = assign12450_e11145;

        let (assign12460_e11150,) = {
    if (locals.var_guard160 == 0.0) {
        (1.0,)
    } else {
        (locals.var_gfedge2,)
    }
};
        locals.var_gfedge2 = assign12460_e11150;

        let (assign12470_e11155,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_lngfedge2,)
    }
};
        locals.var_lngfedge2 = assign12470_e11155;

        let (assign12480_e11160,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phixedge,)
    }
};
        locals.var_phixedge = assign12480_e11160;

        let (assign12490_e11165,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_aphiedge,)
    }
};
        locals.var_aphiedge = assign12490_e11165;

        let (assign12500_e11170,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_bphiedge,)
    }
};
        locals.var_bphiedge = assign12500_e11170;

        let (assign12510_e11175,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phix2edge,)
    }
};
        locals.var_phix2edge = assign12510_e11175;

        let (assign12520_e11180,) = {
    if (locals.var_guard160 == 0.0) {
        (0.0,)
    } else {
        (locals.var_phix1edge,)
    }
};
        locals.var_phix1edge = assign12520_e11180;

        let assign12550_e11193: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign12550_e11193;

        let assign12560_e11196: f64 = (4.0 * 0.3333333333333333);
        let assign12560_e11199: f64 = (2.0 * 1.6021918e-19);
        let assign12560_e11201: f64 = (assign12560_e11199 * 9.1093826e-31);
        let assign12560_e11203: f64 = (assign12560_e11201 * locals.var_chib_i);
        let assign12560_e11204: f64 = (assign12560_e11203).sqrt();
        let assign12560_e11205: f64 = (assign12560_e11196 * assign12560_e11204);
        let assign12560_e11207: f64 = (assign12560_e11205 / 1.05457168e-34);
        locals.var_b_fact = assign12560_e11207;

        let assign12570_e11210: f64 = (locals.var_b_fact * locals.var_tox_i);
        locals.var_bch = assign12570_e11210;

        let assign12580_e11213: f64 = (locals.var_b_fact * locals.var_toxov_i);
        locals.var_bov = assign12580_e11213;

        let assign12590_e11216: f64 = (locals.var_b_fact * locals.var_toxovd_i);
        locals.var_bov_d = assign12590_e11216;

        locals.var_gcq = 0.0;

        let assign12610_e11220: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign12610_e11220;

        let (assign12620_e11229,) = {
    if (locals.var_guard161 != 0.0) {
        let assign12620_e11223: f64 = (-0.495);
        let assign12620_e11225: f64 = (assign12620_e11223 * locals.var_gc2_i);
        let assign12620_e11227: f64 = (assign12620_e11225 / locals.var_gc3_i);
        (assign12620_e11227,)
    } else {
        (locals.var_gcq,)
    }
};
        locals.var_gcq = assign12620_e11229;

        locals.var_gcqov = 0.0;

        let assign12640_e11233: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign12640_e11233;

        let (assign12650_e11242,) = {
    if (locals.var_guard162 != 0.0) {
        let assign12650_e11236: f64 = (-0.495);
        let assign12650_e11238: f64 = (assign12650_e11236 * locals.var_gc2ov_i);
        let assign12650_e11240: f64 = (assign12650_e11238 / locals.var_gc3ov_i);
        (assign12650_e11240,)
    } else {
        (locals.var_gcqov,)
    }
};
        locals.var_gcqov = assign12650_e11242;

    }

    pub(super) fn stamp_transient_block_13(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let assign12660_e11245: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign12660_e11245;

        let (assign12670_e11254,) = {
    if (locals.var_guard163 != 0.0) {
        let assign12670_e11248: f64 = (-0.495);
        let assign12670_e11250: f64 = (assign12670_e11248 * locals.var_gc2ovd_i);
        let assign12670_e11252: f64 = (assign12670_e11250 / locals.var_gc3ovd_i);
        (assign12670_e11252,)
    } else {
        (locals.var_gcqovd,)
    }
};
        locals.var_gcqovd = assign12670_e11254;

        let assign12680_e11257: f64 = (locals.var_rta).powf(locals.var_stig_i);
        locals.var_tf_ig = assign12680_e11257;

        let assign12690_e11260: f64 = (locals.var_iginv_i * locals.var_tf_ig);
        locals.var_iginv_i = assign12690_e11260;

        let assign12700_e11263: f64 = (locals.var_igov_i * locals.var_tf_ig);
        locals.var_igov_i = assign12700_e11263;

        let assign12710_e11266: f64 = (locals.var_igovd_i * locals.var_tf_ig);
        locals.var_igovd_i = assign12710_e11266;

        let assign12720_e11269: f64 = (locals.var_agidl_i * 4e-18);
        let assign12720_e11272: f64 = (locals.var_toxov_i * locals.var_toxov_i);
        let assign12720_e11273: f64 = (assign12720_e11269 / assign12720_e11272);
        locals.var_agidls = assign12720_e11273;

        let assign12730_e11276: f64 = (locals.var_agidld_i * 4e-18);
        let assign12730_e11279: f64 = (locals.var_toxovd_i * locals.var_toxovd_i);
        let assign12730_e11280: f64 = (assign12730_e11276 / assign12730_e11279);
        locals.var_agidlds = assign12730_e11280;

        let assign12740_e11284: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign12740_e11285: f64 = (1.0 + assign12740_e11284);
        let (assign12740_e11294,) = {
    if (assign12740_e11285 > 0.0) {
        let assign12740_e11291: f64 = (locals.var_stbgidl_i * locals.var_delta);
        let assign12740_e11292: f64 = (1.0 + assign12740_e11291);
        (assign12740_e11292,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign12740_e11294;

        let assign12750_e11297: f64 = (locals.var_bgidl_i * locals.var_b_fact);
        locals.var_bgidl_t = assign12750_e11297;

        let assign12760_e11300: f64 = (locals.var_bgidl_t * locals.var_toxov_i);
        let assign12760_e11302: f64 = (assign12760_e11300 * 500000000.0);
        locals.var_bgidls = assign12760_e11302;

        let assign12770_e11306: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign12770_e11307: f64 = (1.0 + assign12770_e11306);
        let (assign12770_e11316,) = {
    if (assign12770_e11307 > 0.0) {
        let assign12770_e11313: f64 = (locals.var_stbgidld_i * locals.var_delta);
        let assign12770_e11314: f64 = (1.0 + assign12770_e11313);
        (assign12770_e11314,)
    } else {
        (0.0,)
    }
};
        locals.var_b_fact = assign12770_e11316;

        let assign12780_e11319: f64 = (locals.var_bgidld_i * locals.var_b_fact);
        locals.var_bgidld_t = assign12780_e11319;

        let assign12790_e11322: f64 = (locals.var_bgidld_t * locals.var_toxovd_i);
        let assign12790_e11324: f64 = (assign12790_e11322 * 500000000.0);
        locals.var_bgidlds = assign12790_e11324;

        locals.var_vinr_max = 0.0;

        let assign12810_e11328: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard164 = assign12810_e11328;

        let (assign12820_e11334,) = {
    if (locals.var_guard164 != 0.0) {
        let assign12820_e11332: f64 = (0.75 / locals.var_fcinracc_i);
        (assign12820_e11332,)
    } else {
        (locals.var_vinr_max,)
    }
};
        locals.var_vinr_max = assign12820_e11334;

        let assign12830_e11337: f64 = (locals.var_axinr_i * locals.var_axinr_i);
        locals.var_ainr = assign12830_e11337;

        let assign12840_e11340: f64 = (9.1093826e-31 * 1000000000.0);
        let assign12840_e11342: f64 = (assign12840_e11340 * locals.var_fntexc_i);
        locals.var_fac_exc = assign12840_e11342;

        locals.var_temp__blk936 = 0.0;
        locals.var_temp__blk936_dn5 = 0.0;
        locals.var_temp__blk936_dn6 = 0.0;
        locals.var_temp__blk936_dn7 = 0.0;
        locals.var_temp__blk936_dn8 = 0.0;

        locals.var_temp1 = 0.0;
        locals.var_temp1_dn5 = 0.0;
        locals.var_temp1_dn6 = 0.0;
        locals.var_temp1_dn7 = 0.0;
        locals.var_temp1_dn8 = 0.0;

        locals.var_temp2 = 0.0;
        locals.var_temp2_dn5 = 0.0;
        locals.var_temp2_dn6 = 0.0;
        locals.var_temp2_dn7 = 0.0;
        locals.var_temp2_dn8 = 0.0;

        let assign40320_e53455: f64 = 1.0;
        let assign40320_e53456: f64 = if locals.var_chnl_type == assign40320_e53455 { 1.0 } else { 0.0 };
        locals.var_guard1011 = assign40320_e53456;

        let (assign40330_e53460, assign40330_e53460_d_n5, assign40330_e53460_d_n6, assign40330_e53460_d_n7,) = {
    if (locals.var_guard1011 != 0.0) {
        ((nv5 - nv6), 1.0, -1.0, 0.0,)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7,)
    }
};
        locals.var_v_gs = assign40330_e53460;
        locals.var_v_gs_dn5 = assign40330_e53460_d_n5;
        locals.var_v_gs_dn6 = assign40330_e53460_d_n6;
        locals.var_v_gs_dn7 = assign40330_e53460_d_n7;

        let (assign40340_e53464, assign40340_e53464_d_n6, assign40340_e53464_d_n7,) = {
    if (locals.var_guard1011 != 0.0) {
        ((nv7 - nv6), -1.0, 1.0,)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7,)
    }
};
        locals.var_v_ds = assign40340_e53464;
        locals.var_v_ds_dn6 = assign40340_e53464_d_n6;
        locals.var_v_ds_dn7 = assign40340_e53464_d_n7;

        let (assign40350_e53468, assign40350_e53468_d_n6, assign40350_e53468_d_n7, assign40350_e53468_d_n8,) = {
    if (locals.var_guard1011 != 0.0) {
        ((nv6 - nv8), 1.0, 0.0, -1.0,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8,)
    }
};
        locals.var_v_sb = assign40350_e53468;
        locals.var_v_sb_dn6 = assign40350_e53468_d_n6;
        locals.var_v_sb_dn7 = assign40350_e53468_d_n7;
        locals.var_v_sb_dn8 = assign40350_e53468_d_n8;

        let (assign40380_e53484, assign40380_e53484_d_n5, assign40380_e53484_d_n6, assign40380_e53484_d_n7,) = {
    if (locals.var_guard1011 == 0.0) {
        let assign40380_e53482: f64 = (-(nv5 - nv6));
        (assign40380_e53482, (-1.0), 1.0, 0.0,)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7,)
    }
};
        locals.var_v_gs = assign40380_e53484;
        locals.var_v_gs_dn5 = assign40380_e53484_d_n5;
        locals.var_v_gs_dn6 = assign40380_e53484_d_n6;
        locals.var_v_gs_dn7 = assign40380_e53484_d_n7;

        let (assign40390_e53490, assign40390_e53490_d_n6, assign40390_e53490_d_n7,) = {
    if (locals.var_guard1011 == 0.0) {
        let assign40390_e53488: f64 = (-(nv7 - nv6));
        (assign40390_e53488, 1.0, (-1.0),)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7,)
    }
};
        locals.var_v_ds = assign40390_e53490;
        locals.var_v_ds_dn6 = assign40390_e53490_d_n6;
        locals.var_v_ds_dn7 = assign40390_e53490_d_n7;

        let (assign40400_e53496, assign40400_e53496_d_n6, assign40400_e53496_d_n7, assign40400_e53496_d_n8,) = {
    if (locals.var_guard1011 == 0.0) {
        let assign40400_e53494: f64 = (-(nv6 - nv8));
        (assign40400_e53494, (-1.0), 0.0, 1.0,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8,)
    }
};
        locals.var_v_sb = assign40400_e53496;
        locals.var_v_sb_dn6 = assign40400_e53496_d_n6;
        locals.var_v_sb_dn7 = assign40400_e53496_d_n7;
        locals.var_v_sb_dn8 = assign40400_e53496_d_n8;

        let assign40430_e53509: f64 = (locals.var_v_gs + locals.var_v_sb);
        locals.var_vgb = assign40430_e53509;
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

        let assign40460_e53514: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_vdbprime = assign40460_e53514;
        locals.var_vdbprime_dn6 = (locals.var_v_ds_dn6 + locals.var_v_sb_dn6);
        locals.var_vdbprime_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_vdbprime_dn8 = locals.var_v_sb_dn8;

        let assign40470_e53517: f64 = (locals.var_v_gs - locals.var_v_ds);
        locals.var_vgdprime = assign40470_e53517;
        locals.var_vgdprime_dn5 = locals.var_v_gs_dn5;
        locals.var_vgdprime_dn6 = (locals.var_v_gs_dn6 - locals.var_v_ds_dn6);
        locals.var_vgdprime_dn7 = (locals.var_v_gs_dn7 - locals.var_v_ds_dn7);

        let assign40480_e53519: f64 = (-locals.var_vgsprime);
        let assign40480_e53521: f64 = (assign40480_e53519 * locals.var_inv_phita);
        locals.var_xgs_ov = assign40480_e53521;
        locals.var_xgs_ov_dn5 = ((-locals.var_vgsprime_dn5) * locals.var_inv_phita);
        locals.var_xgs_ov_dn6 = ((-locals.var_vgsprime_dn6) * locals.var_inv_phita);
        locals.var_xgs_ov_dn7 = ((-locals.var_vgsprime_dn7) * locals.var_inv_phita);

        let assign40490_e53523: f64 = (-locals.var_vgdprime);
        let assign40490_e53525: f64 = (assign40490_e53523 * locals.var_inv_phita);
        locals.var_xgd_ov = assign40490_e53525;
        locals.var_xgd_ov_dn5 = ((-locals.var_vgdprime_dn5) * locals.var_inv_phita);
        locals.var_xgd_ov_dn6 = ((-locals.var_vgdprime_dn6) * locals.var_inv_phita);
        locals.var_xgd_ov_dn7 = ((-locals.var_vgdprime_dn7) * locals.var_inv_phita);

        let assign40500_e53528: f64 = (locals.var_vgb - locals.var_vfb_t);
        let assign40500_e53529: f64 = (-assign40500_e53528);
        let assign40500_e53531: f64 = (assign40500_e53529 * locals.var_inv_phita);
        locals.var_xgb_ov = assign40500_e53531;
        locals.var_xgb_ov_dn5 = ((-locals.var_vgb_dn5) * locals.var_inv_phita);
        locals.var_xgb_ov_dn6 = ((-locals.var_vgb_dn6) * locals.var_inv_phita);
        locals.var_xgb_ov_dn7 = ((-locals.var_vgb_dn7) * locals.var_inv_phita);
        locals.var_xgb_ov_dn8 = ((-locals.var_vgb_dn8) * locals.var_inv_phita);

        locals.var_sigvds = 1.0;

        let assign40520_e53535: f64 = if locals.var_v_ds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1012 = assign40520_e53535;

        let (assign40530_e53540,) = {
    if (locals.var_guard1012 != 0.0) {
        let assign40530_e53538: f64 = (-1.0);
        (assign40530_e53538,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign40530_e53540;

        let (assign40540_e53546, assign40540_e53546_d_n5, assign40540_e53546_d_n6, assign40540_e53546_d_n7,) = {
    if (locals.var_guard1012 != 0.0) {
        let assign40540_e53544: f64 = (locals.var_v_gs - locals.var_v_ds);
        (assign40540_e53544, locals.var_v_gs_dn5, (locals.var_v_gs_dn6 - locals.var_v_ds_dn6), (locals.var_v_gs_dn7 - locals.var_v_ds_dn7),)
    } else {
        (locals.var_v_gs, locals.var_v_gs_dn5, locals.var_v_gs_dn6, locals.var_v_gs_dn7,)
    }
};
        locals.var_v_gs = assign40540_e53546;
        locals.var_v_gs_dn5 = assign40540_e53546_d_n5;
        locals.var_v_gs_dn6 = assign40540_e53546_d_n6;
        locals.var_v_gs_dn7 = assign40540_e53546_d_n7;

        let (assign40550_e53552, assign40550_e53552_d_n6, assign40550_e53552_d_n7, assign40550_e53552_d_n8,) = {
    if (locals.var_guard1012 != 0.0) {
        let assign40550_e53550: f64 = (locals.var_v_sb + locals.var_v_ds);
        (assign40550_e53550, (locals.var_v_sb_dn6 + locals.var_v_ds_dn6), (locals.var_v_sb_dn7 + locals.var_v_ds_dn7), locals.var_v_sb_dn8,)
    } else {
        (locals.var_v_sb, locals.var_v_sb_dn6, locals.var_v_sb_dn7, locals.var_v_sb_dn8,)
    }
};
        locals.var_v_sb = assign40550_e53552;
        locals.var_v_sb_dn6 = assign40550_e53552_d_n6;
        locals.var_v_sb_dn7 = assign40550_e53552_d_n7;
        locals.var_v_sb_dn8 = assign40550_e53552_d_n8;

        let (assign40560_e53557, assign40560_e53557_d_n6, assign40560_e53557_d_n7,) = {
    if (locals.var_guard1012 != 0.0) {
        let assign40560_e53555: f64 = (-locals.var_v_ds);
        (assign40560_e53555, (-locals.var_v_ds_dn6), (-locals.var_v_ds_dn7),)
    } else {
        (locals.var_v_ds, locals.var_v_ds_dn6, locals.var_v_ds_dn7,)
    }
};
        locals.var_v_ds = assign40560_e53557;
        locals.var_v_ds_dn6 = assign40560_e53557_d_n6;
        locals.var_v_ds_dn7 = assign40560_e53557_d_n7;

        let assign40570_e53560: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_v_db = assign40570_e53560;
        locals.var_v_db_dn6 = (locals.var_v_ds_dn6 + locals.var_v_sb_dn6);
        locals.var_v_db_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_v_db_dn8 = locals.var_v_sb_dn8;

        let assign40580_e53563: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40580_e53566: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40580_e53568: f64 = (assign40580_e53566 + 0.01);
        let assign40580_e53569: f64 = (assign40580_e53568).sqrt();
        let assign40580_e53571: f64 = (assign40580_e53569 + 0.1);
        let assign40580_e53572: f64 = (assign40580_e53563 / assign40580_e53571);
        locals.var_vdsx = assign40580_e53572;
        locals.var_vdsx_dn6 = (((((locals.var_v_ds_dn6 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn6)) * assign40580_e53571) - (assign40580_e53563 * (((locals.var_v_ds_dn6 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn6)) / (2.0 * assign40580_e53569)))) / (assign40580_e53571 * assign40580_e53571));
        locals.var_vdsx_dn7 = (((((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) * assign40580_e53571) - (assign40580_e53563 * (((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) / (2.0 * assign40580_e53569)))) / (assign40580_e53571 * assign40580_e53571));

        let assign40590_e53576: f64 = (locals.var_v_db + locals.var_v_sb);
        let assign40590_e53579: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign40590_e53582: f64 = (locals.var_v_db - locals.var_v_sb);
        let assign40590_e53583: f64 = (assign40590_e53579 * assign40590_e53582);
        let assign40590_e53585: f64 = (assign40590_e53583 + locals.var_bphi_dc);
        let assign40590_e53586: f64 = (assign40590_e53585).sqrt();
        let assign40590_e53587: f64 = (assign40590_e53576 - assign40590_e53586);
        let assign40590_e53588: f64 = (0.5 * assign40590_e53587);
        let assign40590_e53590: f64 = (assign40590_e53588 + locals.var_phix_dc);
        locals.var_v_xb = assign40590_e53590;
        locals.var_v_xb_dn6 = (0.5 * ((locals.var_v_db_dn6 + locals.var_v_sb_dn6) - ((((locals.var_v_db_dn6 - locals.var_v_sb_dn6) * assign40590_e53582) + (assign40590_e53579 * (locals.var_v_db_dn6 - locals.var_v_sb_dn6))) / (2.0 * assign40590_e53586))));
        locals.var_v_xb_dn7 = (0.5 * ((locals.var_v_db_dn7 + locals.var_v_sb_dn7) - ((((locals.var_v_db_dn7 - locals.var_v_sb_dn7) * assign40590_e53582) + (assign40590_e53579 * (locals.var_v_db_dn7 - locals.var_v_sb_dn7))) / (2.0 * assign40590_e53586))));
        locals.var_v_xb_dn8 = (0.5 * ((locals.var_v_db_dn8 + locals.var_v_sb_dn8) - ((((locals.var_v_db_dn8 - locals.var_v_sb_dn8) * assign40590_e53582) + (assign40590_e53579 * (locals.var_v_db_dn8 - locals.var_v_sb_dn8))) / (2.0 * assign40590_e53586))));

        locals.var_v_xb_dc_tmp = locals.var_v_xb;
        locals.var_v_xb_dc_tmp_dn6 = locals.var_v_xb_dn6;
        locals.var_v_xb_dc_tmp_dn7 = locals.var_v_xb_dn7;
        locals.var_v_xb_dc_tmp_dn8 = locals.var_v_xb_dn8;

        let assign40610_e53596: f64 = locals.var_v_xb;
        let assign40610_e53599: f64 = locals.var_v_xb;
        let assign40610_e53602: f64 = locals.var_v_xb;
        let assign40610_e53603: f64 = (assign40610_e53599 * assign40610_e53602);
        let assign40610_e53605: f64 = (assign40610_e53603 + locals.var_aphi_dc);
        let assign40610_e53606: f64 = (assign40610_e53605).sqrt();
        let assign40610_e53607: f64 = (assign40610_e53596 - assign40610_e53606);
        let assign40610_e53608: f64 = (0.5 * assign40610_e53607);
        let assign40610_e53609: f64 = (locals.var_v_sb - assign40610_e53608);
        let assign40610_e53611: f64 = (assign40610_e53609 + locals.var_phix1_dc);
        locals.var_vsbstar_dc = assign40610_e53611;
        locals.var_vsbstar_dc_dn5 = 0.0;
        locals.var_vsbstar_dc_dn6 = (locals.var_v_sb_dn6 - (0.5 * (locals.var_v_xb_dn6 - (((locals.var_v_xb_dn6 * assign40610_e53602) + (assign40610_e53599 * locals.var_v_xb_dn6)) / (2.0 * assign40610_e53606)))));
        locals.var_vsbstar_dc_dn7 = (locals.var_v_sb_dn7 - (0.5 * (locals.var_v_xb_dn7 - (((locals.var_v_xb_dn7 * assign40610_e53602) + (assign40610_e53599 * locals.var_v_xb_dn7)) / (2.0 * assign40610_e53606)))));
        locals.var_vsbstar_dc_dn8 = (locals.var_v_sb_dn8 - (0.5 * (locals.var_v_xb_dn8 - (((locals.var_v_xb_dn8 * assign40610_e53602) + (assign40610_e53599 * locals.var_v_xb_dn8)) / (2.0 * assign40610_e53606)))));

        locals.var_vsbstar_dc_tmp = locals.var_vsbstar_dc;
        locals.var_vsbstar_dc_tmp_dn5 = locals.var_vsbstar_dc_dn5;
        locals.var_vsbstar_dc_tmp_dn6 = locals.var_vsbstar_dc_dn6;
        locals.var_vsbstar_dc_tmp_dn7 = locals.var_vsbstar_dc_dn7;
        locals.var_vsbstar_dc_tmp_dn8 = locals.var_vsbstar_dc_dn8;

        locals.var_dvbstar_dc = 0.0;
        locals.var_dvbstar_dc_dn5 = 0.0;
        locals.var_dvbstar_dc_dn6 = 0.0;
        locals.var_dvbstar_dc_dn7 = 0.0;
        locals.var_dvbstar_dc_dn8 = 0.0;

        let assign40640_e53620: f64 = if ((p.p45 != 0.0) && (locals.var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1172 = assign40640_e53620;

        let (assign40650_e53630, assign40650_e53630_d_n5, assign40650_e53630_d_n6, assign40650_e53630_d_n7, assign40650_e53630_d_n8,) = {
    if (locals.var_guard1172 != 0.0) {
        let assign40650_e53626: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40650_e53627: f64 = (0.5 * assign40650_e53626);
        let assign40650_e53628: f64 = (locals.var_vsbstar_dc + assign40650_e53627);
        (assign40650_e53628, locals.var_vsbstar_dc_dn5, (locals.var_vsbstar_dc_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vsbstar_dc_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vsbstar_dc_dn8,)
    } else {
        (locals.var_vmb, locals.var_vmb_dn5, locals.var_vmb_dn6, locals.var_vmb_dn7, locals.var_vmb_dn8,)
    }
};
        locals.var_vmb = assign40650_e53630;
        locals.var_vmb_dn5 = assign40650_e53630_d_n5;
        locals.var_vmb_dn6 = assign40650_e53630_d_n6;
        locals.var_vmb_dn7 = assign40650_e53630_d_n7;
        locals.var_vmb_dn8 = assign40650_e53630_d_n8;

        let (assign40660_e53639, assign40660_e53639_d_n5, assign40660_e53639_d_n6, assign40660_e53639_d_n7, assign40660_e53639_d_n8,) = {
    if (locals.var_guard1172 != 0.0) {
        let assign40660_e53634: f64 = (locals.var_vmb + locals.var_phib_dc);
        let assign40660_e53635: f64 = (assign40660_e53634).sqrt();
        let assign40660_e53637: f64 = (assign40660_e53635 - locals.var_sqrt_phib_dc);
        (assign40660_e53637, (locals.var_vmb_dn5 / (2.0 * assign40660_e53635)), (locals.var_vmb_dn6 / (2.0 * assign40660_e53635)), (locals.var_vmb_dn7 / (2.0 * assign40660_e53635)), (locals.var_vmb_dn8 / (2.0 * assign40660_e53635)),)
    } else {
        (locals.var_us, locals.var_us_dn5, locals.var_us_dn6, locals.var_us_dn7, locals.var_us_dn8,)
    }
};
        locals.var_us = assign40660_e53639;
        locals.var_us_dn5 = assign40660_e53639_d_n5;
        locals.var_us_dn6 = assign40660_e53639_d_n6;
        locals.var_us_dn7 = assign40660_e53639_d_n7;
        locals.var_us_dn8 = assign40660_e53639_d_n8;

        let (assign40670_e53651, assign40670_e53651_d_n5, assign40670_e53651_d_n6, assign40670_e53651_d_n7, assign40670_e53651_d_n8,) = {
    if (locals.var_guard1172 != 0.0) {
        let assign40670_e53644: f64 = (locals.var_us - locals.var_us1);
        let assign40670_e53645: f64 = (2.0 * assign40670_e53644);
        let assign40670_e53647: f64 = (assign40670_e53645 / locals.var_us21);
        let assign40670_e53649: f64 = (assign40670_e53647 - 1.0);
        (assign40670_e53649, ((2.0 * locals.var_us_dn5) / locals.var_us21), ((2.0 * locals.var_us_dn6) / locals.var_us21), ((2.0 * locals.var_us_dn7) / locals.var_us21), ((2.0 * locals.var_us_dn8) / locals.var_us21),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign40670_e53651;
        locals.var_temp__blk936_dn5 = assign40670_e53651_d_n5;
        locals.var_temp__blk936_dn6 = assign40670_e53651_d_n6;
        locals.var_temp__blk936_dn7 = assign40670_e53651_d_n7;
        locals.var_temp__blk936_dn8 = assign40670_e53651_d_n8;

        let (assign40680_e53672, assign40680_e53672_d_n5, assign40680_e53672_d_n6, assign40680_e53672_d_n7, assign40680_e53672_d_n8,) = {
    if (locals.var_guard1172 != 0.0) {
        let assign40680_e53657: f64 = (1.0 - locals.var_gfacnud_i);
        let assign40680_e53658: f64 = (0.25 * assign40680_e53657);
        let assign40680_e53660: f64 = (assign40680_e53658 * locals.var_us21);
        let assign40680_e53664: f64 = (locals.var_temp__blk936 * locals.var_temp__blk936);
        let assign40680_e53666: f64 = (assign40680_e53664 + 0.4804530139182);
        let assign40680_e53667: f64 = (assign40680_e53666).sqrt();
        let assign40680_e53668: f64 = (locals.var_temp__blk936 + assign40680_e53667);
        let assign40680_e53669: f64 = (assign40680_e53660 * assign40680_e53668);
        let assign40680_e53670: f64 = (locals.var_us - assign40680_e53669);
        (assign40680_e53670, (locals.var_us_dn5 - (assign40680_e53660 * (locals.var_temp__blk936_dn5 + (((locals.var_temp__blk936_dn5 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn5)) / (2.0 * assign40680_e53667))))), (locals.var_us_dn6 - (assign40680_e53660 * (locals.var_temp__blk936_dn6 + (((locals.var_temp__blk936_dn6 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn6)) / (2.0 * assign40680_e53667))))), (locals.var_us_dn7 - (assign40680_e53660 * (locals.var_temp__blk936_dn7 + (((locals.var_temp__blk936_dn7 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn7)) / (2.0 * assign40680_e53667))))), (locals.var_us_dn8 - (assign40680_e53660 * (locals.var_temp__blk936_dn8 + (((locals.var_temp__blk936_dn8 * locals.var_temp__blk936) + (locals.var_temp__blk936 * locals.var_temp__blk936_dn8)) / (2.0 * assign40680_e53667))))),)
    } else {
        (locals.var_usnew, locals.var_usnew_dn5, locals.var_usnew_dn6, locals.var_usnew_dn7, locals.var_usnew_dn8,)
    }
};
        locals.var_usnew = assign40680_e53672;
        locals.var_usnew_dn5 = assign40680_e53672_d_n5;
        locals.var_usnew_dn6 = assign40680_e53672_d_n6;
        locals.var_usnew_dn7 = assign40680_e53672_d_n7;
        locals.var_usnew_dn8 = assign40680_e53672_d_n8;

        let (assign40690_e53684, assign40690_e53684_d_n5, assign40690_e53684_d_n6, assign40690_e53684_d_n7, assign40690_e53684_d_n8,) = {
    if (locals.var_guard1172 != 0.0) {
        let assign40690_e53676: f64 = (locals.var_usnew * locals.var_usnew);
        let assign40690_e53679: f64 = (2.0 * locals.var_sqrt_phib_dc);
        let assign40690_e53681: f64 = (assign40690_e53679 * locals.var_usnew);
        let assign40690_e53682: f64 = (assign40690_e53676 + assign40690_e53681);
        (assign40690_e53682, (((locals.var_usnew_dn5 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn5)) + (assign40690_e53679 * locals.var_usnew_dn5)), (((locals.var_usnew_dn6 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn6)) + (assign40690_e53679 * locals.var_usnew_dn6)), (((locals.var_usnew_dn7 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn7)) + (assign40690_e53679 * locals.var_usnew_dn7)), (((locals.var_usnew_dn8 * locals.var_usnew) + (locals.var_usnew * locals.var_usnew_dn8)) + (assign40690_e53679 * locals.var_usnew_dn8)),)
    } else {
        (locals.var_vmbnew, locals.var_vmbnew_dn5, locals.var_vmbnew_dn6, locals.var_vmbnew_dn7, locals.var_vmbnew_dn8,)
    }
};
        locals.var_vmbnew = assign40690_e53684;
        locals.var_vmbnew_dn5 = assign40690_e53684_d_n5;
        locals.var_vmbnew_dn6 = assign40690_e53684_d_n6;
        locals.var_vmbnew_dn7 = assign40690_e53684_d_n7;
        locals.var_vmbnew_dn8 = assign40690_e53684_d_n8;

        let (assign40700_e53694, assign40700_e53694_d_n5, assign40700_e53694_d_n6, assign40700_e53694_d_n7, assign40700_e53694_d_n8,) = {
    if (locals.var_guard1172 != 0.0) {
        let assign40700_e53690: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40700_e53691: f64 = (0.5 * assign40700_e53690);
        let assign40700_e53692: f64 = (locals.var_vmbnew - assign40700_e53691);
        (assign40700_e53692, locals.var_vmbnew_dn5, (locals.var_vmbnew_dn6 - (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6))), (locals.var_vmbnew_dn7 - (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7))), locals.var_vmbnew_dn8,)
    } else {
        (locals.var_vsbstar_dc, locals.var_vsbstar_dc_dn5, locals.var_vsbstar_dc_dn6, locals.var_vsbstar_dc_dn7, locals.var_vsbstar_dc_dn8,)
    }
};
        locals.var_vsbstar_dc = assign40700_e53694;
        locals.var_vsbstar_dc_dn5 = assign40700_e53694_d_n5;
        locals.var_vsbstar_dc_dn6 = assign40700_e53694_d_n6;
        locals.var_vsbstar_dc_dn7 = assign40700_e53694_d_n7;
        locals.var_vsbstar_dc_dn8 = assign40700_e53694_d_n8;

        let (assign40710_e53700, assign40710_e53700_d_n5, assign40710_e53700_d_n6, assign40710_e53700_d_n7, assign40710_e53700_d_n8,) = {
    if (locals.var_guard1172 != 0.0) {
        let assign40710_e53698: f64 = (locals.var_vsbstar_dc_tmp - locals.var_vsbstar_dc);
        (assign40710_e53698, (locals.var_vsbstar_dc_tmp_dn5 - locals.var_vsbstar_dc_dn5), (locals.var_vsbstar_dc_tmp_dn6 - locals.var_vsbstar_dc_dn6), (locals.var_vsbstar_dc_tmp_dn7 - locals.var_vsbstar_dc_dn7), (locals.var_vsbstar_dc_tmp_dn8 - locals.var_vsbstar_dc_dn8),)
    } else {
        (locals.var_dvbstar_dc, locals.var_dvbstar_dc_dn5, locals.var_dvbstar_dc_dn6, locals.var_dvbstar_dc_dn7, locals.var_dvbstar_dc_dn8,)
    }
};
        locals.var_dvbstar_dc = assign40710_e53700;
        locals.var_dvbstar_dc_dn5 = assign40710_e53700_d_n5;
        locals.var_dvbstar_dc_dn6 = assign40710_e53700_d_n6;
        locals.var_dvbstar_dc_dn7 = assign40710_e53700_d_n7;
        locals.var_dvbstar_dc_dn8 = assign40710_e53700_d_n8;

        locals.var_phib = locals.var_phib_dc;

        locals.var_aphi = locals.var_aphi_dc;

        locals.var_g_0 = locals.var_g_0_dc;

        locals.var_vsbstar = locals.var_vsbstar_dc;
        locals.var_vsbstar_dn5 = locals.var_vsbstar_dc_dn5;
        locals.var_vsbstar_dn6 = locals.var_vsbstar_dc_dn6;
        locals.var_vsbstar_dn7 = locals.var_vsbstar_dc_dn7;
        locals.var_vsbstar_dn8 = locals.var_vsbstar_dc_dn8;

        locals.var_dvbstar = locals.var_dvbstar_dc;
        locals.var_dvbstar_dn5 = locals.var_dvbstar_dc_dn5;
        locals.var_dvbstar_dn6 = locals.var_dvbstar_dc_dn6;
        locals.var_dvbstar_dn7 = locals.var_dvbstar_dc_dn7;
        locals.var_dvbstar_dn8 = locals.var_dvbstar_dc_dn8;

        locals.var_thesatloc = locals.var_thesat_t;

        locals.var_arloc = locals.var_ar;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign40790_e53710: f64 = (locals.var_vgb - locals.var_dvbstar);
        let assign40790_e53712: f64 = (assign40790_e53710 - locals.var_vfb_t);
        locals.var_vgb1 = assign40790_e53712;
        locals.var_vgb1_dn5 = (locals.var_vgb_dn5 - locals.var_dvbstar_dn5);
        locals.var_vgb1_dn6 = (locals.var_vgb_dn6 - locals.var_dvbstar_dn6);
        locals.var_vgb1_dn7 = (locals.var_vgb_dn7 - locals.var_dvbstar_dn7);
        locals.var_vgb1_dn8 = (locals.var_vgb_dn8 - locals.var_dvbstar_dn8);

        let assign40800_e53717: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40800_e53718: f64 = (0.5 * assign40800_e53717);
        let assign40800_e53719: f64 = (locals.var_vsbstar + assign40800_e53718);
        locals.var_vsbx = assign40800_e53719;
        locals.var_vsbx_dn5 = locals.var_vsbstar_dn5;
        locals.var_vsbx_dn6 = (locals.var_vsbstar_dn6 + (0.5 * (locals.var_v_ds_dn6 - locals.var_vdsx_dn6)));
        locals.var_vsbx_dn7 = (locals.var_vsbstar_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7)));
        locals.var_vsbx_dn8 = locals.var_vsbstar_dn8;

        locals.var_dctg = 1.0;
        locals.var_dctg_dn5 = 0.0;
        locals.var_dctg_dn6 = 0.0;
        locals.var_dctg_dn7 = 0.0;
        locals.var_dctg_dn8 = 0.0;

        let assign40820_e53723: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1173 = assign40820_e53723;

        let (assign40830_e53729,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40830_e53727: f64 = (locals.var_phib * locals.var_inv_phit);
        (assign40830_e53727,)
    } else {
        (locals.var_xbct,)
    }
};
        locals.var_xbct = assign40830_e53729;

        let (assign40840_e53735, assign40840_e53735_d_n5, assign40840_e53735_d_n6, assign40840_e53735_d_n7, assign40840_e53735_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40840_e53733: f64 = (locals.var_vsbx * locals.var_inv_phit);
        (assign40840_e53733, (locals.var_vsbx_dn5 * locals.var_inv_phit), (locals.var_vsbx_dn6 * locals.var_inv_phit), (locals.var_vsbx_dn7 * locals.var_inv_phit), (locals.var_vsbx_dn8 * locals.var_inv_phit),)
    } else {
        (locals.var_xsbstar, locals.var_xsbstar_dn5, locals.var_xsbstar_dn6, locals.var_xsbstar_dn7, locals.var_xsbstar_dn8,)
    }
};
        locals.var_xsbstar = assign40840_e53735;
        locals.var_xsbstar_dn5 = assign40840_e53735_d_n5;
        locals.var_xsbstar_dn6 = assign40840_e53735_d_n6;
        locals.var_xsbstar_dn7 = assign40840_e53735_d_n7;
        locals.var_xsbstar_dn8 = assign40840_e53735_d_n8;

        let (assign40850_e53741, assign40850_e53741_d_n5, assign40850_e53741_d_n6, assign40850_e53741_d_n7, assign40850_e53741_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40850_e53739: f64 = (locals.var_vgb1 * locals.var_inv_phit);
        (assign40850_e53739, (locals.var_vgb1_dn5 * locals.var_inv_phit), (locals.var_vgb1_dn6 * locals.var_inv_phit), (locals.var_vgb1_dn7 * locals.var_inv_phit), (locals.var_vgb1_dn8 * locals.var_inv_phit),)
    } else {
        (locals.var_xgct, locals.var_xgct_dn5, locals.var_xgct_dn6, locals.var_xgct_dn7, locals.var_xgct_dn8,)
    }
};
        locals.var_xgct = assign40850_e53741;
        locals.var_xgct_dn5 = assign40850_e53741_d_n5;
        locals.var_xgct_dn6 = assign40850_e53741_d_n6;
        locals.var_xgct_dn7 = assign40850_e53741_d_n7;
        locals.var_xgct_dn8 = assign40850_e53741_d_n8;

        let (assign40860_e53752, assign40860_e53752_d_n5, assign40860_e53752_d_n6, assign40860_e53752_d_n7, assign40860_e53752_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40860_e53746: f64 = (0.5 * locals.var_g_0);
        let assign40860_e53748: f64 = (locals.var_xbct).sqrt();
        let assign40860_e53749: f64 = (assign40860_e53746 / assign40860_e53748);
        let assign40860_e53750: f64 = (1.0 + assign40860_e53749);
        (assign40860_e53750, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign40860_e53752;
        locals.var_temp1_dn5 = assign40860_e53752_d_n5;
        locals.var_temp1_dn6 = assign40860_e53752_d_n6;
        locals.var_temp1_dn7 = assign40860_e53752_d_n7;
        locals.var_temp1_dn8 = assign40860_e53752_d_n8;

        let (assign40870_e53761, assign40870_e53761_d_n5, assign40870_e53761_d_n6, assign40870_e53761_d_n7, assign40870_e53761_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40870_e53757: f64 = (locals.var_xbct).sqrt();
        let assign40870_e53758: f64 = (locals.var_g_0 * assign40870_e53757);
        let assign40870_e53759: f64 = (locals.var_xbct + assign40870_e53758);
        (assign40870_e53759, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign40870_e53761;
        locals.var_temp2_dn5 = assign40870_e53761_d_n5;
        locals.var_temp2_dn6 = assign40870_e53761_d_n6;
        locals.var_temp2_dn7 = assign40870_e53761_d_n7;
        locals.var_temp2_dn8 = assign40870_e53761_d_n8;

        let (assign40880_e53779, assign40880_e53779_d_n5, assign40880_e53779_d_n6, assign40880_e53779_d_n7, assign40880_e53779_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40880_e53765: f64 = (locals.var_xgct - locals.var_temp2);
        let assign40880_e53767: f64 = (assign40880_e53765 / locals.var_temp1);
        let assign40880_e53770: f64 = (0.5 * locals.var_xbct);
        let assign40880_e53771: f64 = (assign40880_e53767 + assign40880_e53770);
        let assign40880_e53774: f64 = (1.0 + locals.var_ctb_i);
        let assign40880_e53776: f64 = (assign40880_e53774 * locals.var_xsbstar);
        let assign40880_e53777: f64 = (assign40880_e53771 - assign40880_e53776);
        (assign40880_e53777, (((((locals.var_xgct_dn5 - locals.var_temp2_dn5) * locals.var_temp1) - (assign40880_e53765 * locals.var_temp1_dn5)) / (locals.var_temp1 * locals.var_temp1)) - (assign40880_e53774 * locals.var_xsbstar_dn5)), (((((locals.var_xgct_dn6 - locals.var_temp2_dn6) * locals.var_temp1) - (assign40880_e53765 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) - (assign40880_e53774 * locals.var_xsbstar_dn6)), (((((locals.var_xgct_dn7 - locals.var_temp2_dn7) * locals.var_temp1) - (assign40880_e53765 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) - (assign40880_e53774 * locals.var_xsbstar_dn7)), (((((locals.var_xgct_dn8 - locals.var_temp2_dn8) * locals.var_temp1) - (assign40880_e53765 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) - (assign40880_e53774 * locals.var_xsbstar_dn8)),)
    } else {
        (locals.var_xwict, locals.var_xwict_dn5, locals.var_xwict_dn6, locals.var_xwict_dn7, locals.var_xwict_dn8,)
    }
};
        locals.var_xwict = assign40880_e53779;
        locals.var_xwict_dn5 = assign40880_e53779_d_n5;
        locals.var_xwict_dn6 = assign40880_e53779_d_n6;
        locals.var_xwict_dn7 = assign40880_e53779_d_n7;
        locals.var_xwict_dn8 = assign40880_e53779_d_n8;

        let (assign40890_e53787,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40890_e53783: f64 = (0.5 * locals.var_xbct);
        let assign40890_e53785: f64 = (assign40890_e53783 + 2.0);
        (assign40890_e53785,)
    } else {
        (locals.var_xctmax,)
    }
};
        locals.var_xctmax = assign40890_e53787;

        let (assign40900_e53793, assign40900_e53793_d_n5, assign40900_e53793_d_n6, assign40900_e53793_d_n7, assign40900_e53793_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40900_e53791: f64 = (locals.var_xbct + locals.var_xsbstar);
        (assign40900_e53791, locals.var_xsbstar_dn5, locals.var_xsbstar_dn6, locals.var_xsbstar_dn7, locals.var_xsbstar_dn8,)
    } else {
        (locals.var_xnct, locals.var_xnct_dn5, locals.var_xnct_dn6, locals.var_xnct_dn7, locals.var_xnct_dn8,)
    }
};
        locals.var_xnct = assign40900_e53793;
        locals.var_xnct_dn5 = assign40900_e53793_d_n5;
        locals.var_xnct_dn6 = assign40900_e53793_d_n6;
        locals.var_xnct_dn7 = assign40900_e53793_d_n7;
        locals.var_xnct_dn8 = assign40900_e53793_d_n8;

        let (assign40910_e53814, assign40910_e53814_d_n5, assign40910_e53814_d_n6, assign40910_e53814_d_n7, assign40910_e53814_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40910_e53797: f64 = (locals.var_xgct - locals.var_xnct);
        let assign40910_e53800: f64 = (locals.var_xnct).sqrt();
        let assign40910_e53801: f64 = (locals.var_g_0 * assign40910_e53800);
        let assign40910_e53802: f64 = (assign40910_e53797 - assign40910_e53801);
        let assign40910_e53806: f64 = (locals.var_xbct / locals.var_g_0);
        let assign40910_e53808: f64 = (locals.var_xbct).sqrt();
        let assign40910_e53809: f64 = (assign40910_e53806 + assign40910_e53808);
        let assign40910_e53810: f64 = (assign40910_e53809).ln();
        let assign40910_e53811: f64 = (2.0 * assign40910_e53810);
        let assign40910_e53812: f64 = (assign40910_e53802 - assign40910_e53811);
        (assign40910_e53812, ((locals.var_xgct_dn5 - locals.var_xnct_dn5) - (locals.var_g_0 * (locals.var_xnct_dn5 / (2.0 * assign40910_e53800)))), ((locals.var_xgct_dn6 - locals.var_xnct_dn6) - (locals.var_g_0 * (locals.var_xnct_dn6 / (2.0 * assign40910_e53800)))), ((locals.var_xgct_dn7 - locals.var_xnct_dn7) - (locals.var_g_0 * (locals.var_xnct_dn7 / (2.0 * assign40910_e53800)))), ((locals.var_xgct_dn8 - locals.var_xnct_dn8) - (locals.var_g_0 * (locals.var_xnct_dn8 / (2.0 * assign40910_e53800)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign40910_e53814;
        locals.var_temp1_dn5 = assign40910_e53814_d_n5;
        locals.var_temp1_dn6 = assign40910_e53814_d_n6;
        locals.var_temp1_dn7 = assign40910_e53814_d_n7;
        locals.var_temp1_dn8 = assign40910_e53814_d_n8;

        let (assign40920_e53822, assign40920_e53822_d_n5, assign40920_e53822_d_n6, assign40920_e53822_d_n7, assign40920_e53822_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40920_e53818: f64 = (2.0 * locals.var_temp1);
        let assign40920_e53820: f64 = (assign40920_e53818 + locals.var_xctmax);
        (assign40920_e53820, (2.0 * locals.var_temp1_dn5), (2.0 * locals.var_temp1_dn6), (2.0 * locals.var_temp1_dn7), (2.0 * locals.var_temp1_dn8),)
    } else {
        (locals.var_xmict, locals.var_xmict_dn5, locals.var_xmict_dn6, locals.var_xmict_dn7, locals.var_xmict_dn8,)
    }
};
        locals.var_xmict = assign40920_e53822;
        locals.var_xmict_dn5 = assign40920_e53822_d_n5;
        locals.var_xmict_dn6 = assign40920_e53822_d_n6;
        locals.var_xmict_dn7 = assign40920_e53822_d_n7;
        locals.var_xmict_dn8 = assign40920_e53822_d_n8;

        let (assign40930_e53841, assign40930_e53841_d_n5, assign40930_e53841_d_n6, assign40930_e53841_d_n7, assign40930_e53841_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40930_e53827: f64 = (locals.var_xwict + locals.var_xmict);
        let assign40930_e53830: f64 = (locals.var_xwict - locals.var_xmict);
        let assign40930_e53833: f64 = (locals.var_xwict - locals.var_xmict);
        let assign40930_e53834: f64 = (assign40930_e53830 * assign40930_e53833);
        let assign40930_e53836: f64 = (assign40930_e53834 + 20.0);
        let assign40930_e53837: f64 = (assign40930_e53836).sqrt();
        let assign40930_e53838: f64 = (assign40930_e53827 + assign40930_e53837);
        let assign40930_e53839: f64 = (0.5 * assign40930_e53838);
        (assign40930_e53839, (0.5 * ((locals.var_xwict_dn5 + locals.var_xmict_dn5) + ((((locals.var_xwict_dn5 - locals.var_xmict_dn5) * assign40930_e53833) + (assign40930_e53830 * (locals.var_xwict_dn5 - locals.var_xmict_dn5))) / (2.0 * assign40930_e53837)))), (0.5 * ((locals.var_xwict_dn6 + locals.var_xmict_dn6) + ((((locals.var_xwict_dn6 - locals.var_xmict_dn6) * assign40930_e53833) + (assign40930_e53830 * (locals.var_xwict_dn6 - locals.var_xmict_dn6))) / (2.0 * assign40930_e53837)))), (0.5 * ((locals.var_xwict_dn7 + locals.var_xmict_dn7) + ((((locals.var_xwict_dn7 - locals.var_xmict_dn7) * assign40930_e53833) + (assign40930_e53830 * (locals.var_xwict_dn7 - locals.var_xmict_dn7))) / (2.0 * assign40930_e53837)))), (0.5 * ((locals.var_xwict_dn8 + locals.var_xmict_dn8) + ((((locals.var_xwict_dn8 - locals.var_xmict_dn8) * assign40930_e53833) + (assign40930_e53830 * (locals.var_xwict_dn8 - locals.var_xmict_dn8))) / (2.0 * assign40930_e53837)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign40930_e53841;
        locals.var_temp1_dn5 = assign40930_e53841_d_n5;
        locals.var_temp1_dn6 = assign40930_e53841_d_n6;
        locals.var_temp1_dn7 = assign40930_e53841_d_n7;
        locals.var_temp1_dn8 = assign40930_e53841_d_n8;

        let (assign40940_e53851, assign40940_e53851_d_n5, assign40940_e53851_d_n6, assign40940_e53851_d_n7, assign40940_e53851_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40940_e53846: f64 = (locals.var_xgct - locals.var_xsbstar);
        let assign40940_e53847: f64 = (2.0 * assign40940_e53846);
        let assign40940_e53849: f64 = (assign40940_e53847 - locals.var_xctmax);
        (assign40940_e53849, (2.0 * (locals.var_xgct_dn5 - locals.var_xsbstar_dn5)), (2.0 * (locals.var_xgct_dn6 - locals.var_xsbstar_dn6)), (2.0 * (locals.var_xgct_dn7 - locals.var_xsbstar_dn7)), (2.0 * (locals.var_xgct_dn8 - locals.var_xsbstar_dn8)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign40940_e53851;
        locals.var_temp2_dn5 = assign40940_e53851_d_n5;
        locals.var_temp2_dn6 = assign40940_e53851_d_n6;
        locals.var_temp2_dn7 = assign40940_e53851_d_n7;
        locals.var_temp2_dn8 = assign40940_e53851_d_n8;

        let (assign40950_e53870, assign40950_e53870_d_n5, assign40950_e53870_d_n6, assign40950_e53870_d_n7, assign40950_e53870_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40950_e53856: f64 = (locals.var_temp1 + locals.var_temp2);
        let assign40950_e53859: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign40950_e53862: f64 = (locals.var_temp1 - locals.var_temp2);
        let assign40950_e53863: f64 = (assign40950_e53859 * assign40950_e53862);
        let assign40950_e53865: f64 = (assign40950_e53863 + 20.0);
        let assign40950_e53866: f64 = (assign40950_e53865).sqrt();
        let assign40950_e53867: f64 = (assign40950_e53856 - assign40950_e53866);
        let assign40950_e53868: f64 = (0.5 * assign40950_e53867);
        (assign40950_e53868, (0.5 * ((locals.var_temp1_dn5 + locals.var_temp2_dn5) - ((((locals.var_temp1_dn5 - locals.var_temp2_dn5) * assign40950_e53862) + (assign40950_e53859 * (locals.var_temp1_dn5 - locals.var_temp2_dn5))) / (2.0 * assign40950_e53866)))), (0.5 * ((locals.var_temp1_dn6 + locals.var_temp2_dn6) - ((((locals.var_temp1_dn6 - locals.var_temp2_dn6) * assign40950_e53862) + (assign40950_e53859 * (locals.var_temp1_dn6 - locals.var_temp2_dn6))) / (2.0 * assign40950_e53866)))), (0.5 * ((locals.var_temp1_dn7 + locals.var_temp2_dn7) - ((((locals.var_temp1_dn7 - locals.var_temp2_dn7) * assign40950_e53862) + (assign40950_e53859 * (locals.var_temp1_dn7 - locals.var_temp2_dn7))) / (2.0 * assign40950_e53866)))), (0.5 * ((locals.var_temp1_dn8 + locals.var_temp2_dn8) - ((((locals.var_temp1_dn8 - locals.var_temp2_dn8) * assign40950_e53862) + (assign40950_e53859 * (locals.var_temp1_dn8 - locals.var_temp2_dn8))) / (2.0 * assign40950_e53866)))),)
    } else {
        (locals.var_xsubct, locals.var_xsubct_dn5, locals.var_xsubct_dn6, locals.var_xsubct_dn7, locals.var_xsubct_dn8,)
    }
};
        locals.var_xsubct = assign40950_e53870;
        locals.var_xsubct_dn5 = assign40950_e53870_d_n5;
        locals.var_xsubct_dn6 = assign40950_e53870_d_n6;
        locals.var_xsubct_dn7 = assign40950_e53870_d_n7;
        locals.var_xsubct_dn8 = assign40950_e53870_d_n8;

        let (assign40960_e53889, assign40960_e53889_d_n5, assign40960_e53889_d_n6, assign40960_e53889_d_n7, assign40960_e53889_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40960_e53875: f64 = (locals.var_xsubct + locals.var_xctmax);
        let assign40960_e53878: f64 = (locals.var_xsubct - locals.var_xctmax);
        let assign40960_e53881: f64 = (locals.var_xsubct - locals.var_xctmax);
        let assign40960_e53882: f64 = (assign40960_e53878 * assign40960_e53881);
        let assign40960_e53884: f64 = (assign40960_e53882 + 5.0);
        let assign40960_e53885: f64 = (assign40960_e53884).sqrt();
        let assign40960_e53886: f64 = (assign40960_e53875 - assign40960_e53885);
        let assign40960_e53887: f64 = (0.5 * assign40960_e53886);
        (assign40960_e53887, (0.5 * (locals.var_xsubct_dn5 - (((locals.var_xsubct_dn5 * assign40960_e53881) + (assign40960_e53878 * locals.var_xsubct_dn5)) / (2.0 * assign40960_e53885)))), (0.5 * (locals.var_xsubct_dn6 - (((locals.var_xsubct_dn6 * assign40960_e53881) + (assign40960_e53878 * locals.var_xsubct_dn6)) / (2.0 * assign40960_e53885)))), (0.5 * (locals.var_xsubct_dn7 - (((locals.var_xsubct_dn7 * assign40960_e53881) + (assign40960_e53878 * locals.var_xsubct_dn7)) / (2.0 * assign40960_e53885)))), (0.5 * (locals.var_xsubct_dn8 - (((locals.var_xsubct_dn8 * assign40960_e53881) + (assign40960_e53878 * locals.var_xsubct_dn8)) / (2.0 * assign40960_e53885)))),)
    } else {
        (locals.var_temp1, locals.var_temp1_dn5, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8,)
    }
};
        locals.var_temp1 = assign40960_e53889;
        locals.var_temp1_dn5 = assign40960_e53889_d_n5;
        locals.var_temp1_dn6 = assign40960_e53889_d_n6;
        locals.var_temp1_dn7 = assign40960_e53889_d_n7;
        locals.var_temp1_dn8 = assign40960_e53889_d_n8;

        let (assign40970_e53911, assign40970_e53911_d_n5, assign40970_e53911_d_n6, assign40970_e53911_d_n7, assign40970_e53911_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40970_e53894: f64 = (-locals.var_xctmax);
        let assign40970_e53895: f64 = (locals.var_temp1 + assign40970_e53894);
        let assign40970_e53898: f64 = (-locals.var_xctmax);
        let assign40970_e53899: f64 = (locals.var_temp1 - assign40970_e53898);
        let assign40970_e53902: f64 = (-locals.var_xctmax);
        let assign40970_e53903: f64 = (locals.var_temp1 - assign40970_e53902);
        let assign40970_e53904: f64 = (assign40970_e53899 * assign40970_e53903);
        let assign40970_e53906: f64 = (assign40970_e53904 + 20.0);
        let assign40970_e53907: f64 = (assign40970_e53906).sqrt();
        let assign40970_e53908: f64 = (assign40970_e53895 + assign40970_e53907);
        let assign40970_e53909: f64 = (0.5 * assign40970_e53908);
        (assign40970_e53909, (0.5 * (locals.var_temp1_dn5 + (((locals.var_temp1_dn5 * assign40970_e53903) + (assign40970_e53899 * locals.var_temp1_dn5)) / (2.0 * assign40970_e53907)))), (0.5 * (locals.var_temp1_dn6 + (((locals.var_temp1_dn6 * assign40970_e53903) + (assign40970_e53899 * locals.var_temp1_dn6)) / (2.0 * assign40970_e53907)))), (0.5 * (locals.var_temp1_dn7 + (((locals.var_temp1_dn7 * assign40970_e53903) + (assign40970_e53899 * locals.var_temp1_dn7)) / (2.0 * assign40970_e53907)))), (0.5 * (locals.var_temp1_dn8 + (((locals.var_temp1_dn8 * assign40970_e53903) + (assign40970_e53899 * locals.var_temp1_dn8)) / (2.0 * assign40970_e53907)))),)
    } else {
        (locals.var_xct, locals.var_xct_dn5, locals.var_xct_dn6, locals.var_xct_dn7, locals.var_xct_dn8,)
    }
};
        locals.var_xct = assign40970_e53911;
        locals.var_xct_dn5 = assign40970_e53911_d_n5;
        locals.var_xct_dn6 = assign40970_e53911_d_n6;
        locals.var_xct_dn7 = assign40970_e53911_d_n7;
        locals.var_xct_dn8 = assign40970_e53911_d_n8;

        let (assign40980_e53921, assign40980_e53921_d_n5, assign40980_e53921_d_n6, assign40980_e53921_d_n7, assign40980_e53921_d_n8,) = {
    if (locals.var_guard1173 != 0.0) {
        let assign40980_e53916: f64 = (locals.var_xct / locals.var_xctmax);
        let assign40980_e53918: f64 = (assign40980_e53916 + 1.0);
        let assign40980_e53919: f64 = (locals.var_ctg_t * assign40980_e53918);
        (assign40980_e53919, (locals.var_ctg_t * (locals.var_xct_dn5 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn6 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn7 / locals.var_xctmax)), (locals.var_ctg_t * (locals.var_xct_dn8 / locals.var_xctmax)),)
    } else {
        (locals.var_temp2, locals.var_temp2_dn5, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8,)
    }
};
        locals.var_temp2 = assign40980_e53921;
        locals.var_temp2_dn5 = assign40980_e53921_d_n5;
        locals.var_temp2_dn6 = assign40980_e53921_d_n6;
        locals.var_temp2_dn7 = assign40980_e53921_d_n7;
        locals.var_temp2_dn8 = assign40980_e53921_d_n8;

        let assign40990_e53924: f64 = (-230.25850929940458);
        let assign40990_e53925: f64 = if locals.var_temp2 > assign40990_e53924 { 1.0 } else { 0.0 };
        locals.var_guard1174 = assign40990_e53925;

        let (assign41000_e53932, assign41000_e53932_d_n5, assign41000_e53932_d_n6, assign41000_e53932_d_n7, assign41000_e53932_d_n8,) = {
    if ((locals.var_guard1173 != 0.0) && (locals.var_guard1174 != 0.0)) {
        let assign41000_e53930: f64 = (locals.var_temp2).exp();
        (assign41000_e53930, (assign41000_e53930 * locals.var_temp2_dn5), (assign41000_e53930 * locals.var_temp2_dn6), (assign41000_e53930 * locals.var_temp2_dn7), (assign41000_e53930 * locals.var_temp2_dn8),)
    } else {
        (locals.var_dctg, locals.var_dctg_dn5, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8,)
    }
};
        locals.var_dctg = assign41000_e53932;
        locals.var_dctg_dn5 = assign41000_e53932_d_n5;
        locals.var_dctg_dn6 = assign41000_e53932_d_n6;
        locals.var_dctg_dn7 = assign41000_e53932_d_n7;
        locals.var_dctg_dn8 = assign41000_e53932_d_n8;

        let (assign41010_e53964, assign41010_e53964_d_n5, assign41010_e53964_d_n6, assign41010_e53964_d_n7, assign41010_e53964_d_n8,) = {
    if ((locals.var_guard1173 != 0.0) && (locals.var_guard1174 == 0.0)) {
        let assign41010_e53940: f64 = (-230.25850929940458);
        let assign41010_e53942: f64 = (assign41010_e53940 - locals.var_temp2);
        let assign41010_e53946: f64 = (-230.25850929940458);
        let assign41010_e53948: f64 = (assign41010_e53946 - locals.var_temp2);
        let assign41010_e53951: f64 = (-230.25850929940458);
        let assign41010_e53953: f64 = (assign41010_e53951 - locals.var_temp2);
        let assign41010_e53955: f64 = (assign41010_e53953 * 0.3333333333333333);
        let assign41010_e53956: f64 = (1.0 + assign41010_e53955);
        let assign41010_e53957: f64 = (assign41010_e53948 * assign41010_e53956);
        let assign41010_e53958: f64 = (0.5 * assign41010_e53957);
        let assign41010_e53959: f64 = (1.0 + assign41010_e53958);
        let assign41010_e53960: f64 = (assign41010_e53942 * assign41010_e53959);
        let assign41010_e53961: f64 = (1.0 + assign41010_e53960);
        let assign41010_e53962: f64 = (1e-100 / assign41010_e53961);
        (assign41010_e53962, (-((1e-100 * (((-locals.var_temp2_dn5) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-locals.var_temp2_dn5) * assign41010_e53956) + (assign41010_e53948 * ((-locals.var_temp2_dn5) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-locals.var_temp2_dn6) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-locals.var_temp2_dn6) * assign41010_e53956) + (assign41010_e53948 * ((-locals.var_temp2_dn6) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-locals.var_temp2_dn7) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-locals.var_temp2_dn7) * assign41010_e53956) + (assign41010_e53948 * ((-locals.var_temp2_dn7) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))), (-((1e-100 * (((-locals.var_temp2_dn8) * assign41010_e53959) + (assign41010_e53942 * (0.5 * (((-locals.var_temp2_dn8) * assign41010_e53956) + (assign41010_e53948 * ((-locals.var_temp2_dn8) * 0.3333333333333333))))))) / (assign41010_e53961 * assign41010_e53961))),)
    } else {
        (locals.var_dctg, locals.var_dctg_dn5, locals.var_dctg_dn6, locals.var_dctg_dn7, locals.var_dctg_dn8,)
    }
};
        locals.var_dctg = assign41010_e53964;
        locals.var_dctg_dn5 = assign41010_e53964_d_n5;
        locals.var_dctg_dn6 = assign41010_e53964_d_n6;
        locals.var_dctg_dn7 = assign41010_e53964_d_n7;
        locals.var_dctg_dn8 = assign41010_e53964_d_n8;

        let assign41020_e53968: f64 = (locals.var_ct_t * locals.var_dctg);
        let assign41020_e53969: f64 = (1.0 + assign41020_e53968);
        locals.var_ct_fact = assign41020_e53969;
        locals.var_ct_fact_dn5 = (locals.var_ct_t * locals.var_dctg_dn5);
        locals.var_ct_fact_dn6 = (locals.var_ct_t * locals.var_dctg_dn6);
        locals.var_ct_fact_dn7 = (locals.var_ct_t * locals.var_dctg_dn7);
        locals.var_ct_fact_dn8 = (locals.var_ct_t * locals.var_dctg_dn8);

        let assign41030_e53972: f64 = (locals.var_phit * locals.var_ct_fact);
        locals.var_phitct = assign41030_e53972;
        locals.var_phitct_dn5 = (locals.var_phit * locals.var_ct_fact_dn5);
        locals.var_phitct_dn6 = (locals.var_phit * locals.var_ct_fact_dn6);
        locals.var_phitct_dn7 = (locals.var_phit * locals.var_ct_fact_dn7);
        locals.var_phitct_dn8 = (locals.var_phit * locals.var_ct_fact_dn8);

        let assign41040_e53977: f64 = (locals.var_psced_i * locals.var_vdsx);
        let assign41040_e53978: f64 = (1.0 + assign41040_e53977);
        let assign41040_e53979: f64 = (locals.var_psce_i * assign41040_e53978);
        let assign41040_e53983: f64 = (locals.var_psceb_i * locals.var_vsbx);
        let assign41040_e53984: f64 = (1.0 + assign41040_e53983);
        let assign41040_e53985: f64 = (assign41040_e53979 * assign41040_e53984);
        locals.var_dphit1 = assign41040_e53985;
        locals.var_dphit1_dn5 = (assign41040_e53979 * (locals.var_psceb_i * locals.var_vsbx_dn5));
        locals.var_dphit1_dn6 = (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn6)) * assign41040_e53984) + (assign41040_e53979 * (locals.var_psceb_i * locals.var_vsbx_dn6)));
        locals.var_dphit1_dn7 = (((locals.var_psce_i * (locals.var_psced_i * locals.var_vdsx_dn7)) * assign41040_e53984) + (assign41040_e53979 * (locals.var_psceb_i * locals.var_vsbx_dn7)));
        locals.var_dphit1_dn8 = (assign41040_e53979 * (locals.var_psceb_i * locals.var_vsbx_dn8));

        let assign41050_e53989: f64 = (1.0 + locals.var_dphit1);
        let assign41050_e53990: f64 = (locals.var_phitct * assign41050_e53989);
        locals.var_phit1 = assign41050_e53990;
        locals.var_phit1_dn5 = ((locals.var_phitct_dn5 * assign41050_e53989) + (locals.var_phitct * locals.var_dphit1_dn5));
        locals.var_phit1_dn6 = ((locals.var_phitct_dn6 * assign41050_e53989) + (locals.var_phitct * locals.var_dphit1_dn6));
        locals.var_phit1_dn7 = ((locals.var_phitct_dn7 * assign41050_e53989) + (locals.var_phitct * locals.var_dphit1_dn7));
        locals.var_phit1_dn8 = ((locals.var_phitct_dn8 * assign41050_e53989) + (locals.var_phitct * locals.var_dphit1_dn8));

        let assign41060_e53993: f64 = (1.0 / locals.var_phit1);
        locals.var_inv_phit1 = assign41060_e53993;
        locals.var_inv_phit1_dn5 = (-(locals.var_phit1_dn5 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn6 = (-(locals.var_phit1_dn6 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn7 = (-(locals.var_phit1_dn7 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn8 = (-(locals.var_phit1_dn8 / (locals.var_phit1 * locals.var_phit1)));

        let assign41070_e53997: f64 = (locals.var_phit * locals.var_inv_phit1);
        let assign41070_e53998: f64 = (assign41070_e53997).sqrt();
        let assign41070_e53999: f64 = (locals.var_g_0 * assign41070_e53998);
        locals.var_gf = assign41070_e53999;
        locals.var_gf_dn5 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn5) / (2.0 * assign41070_e53998)));
        locals.var_gf_dn6 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn6) / (2.0 * assign41070_e53998)));
        locals.var_gf_dn7 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn7) / (2.0 * assign41070_e53998)));
        locals.var_gf_dn8 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn8) / (2.0 * assign41070_e53998)));

        let assign41080_e54002: f64 = (locals.var_gf * locals.var_gf);
        locals.var_gf2 = assign41080_e54002;
        locals.var_gf2_dn5 = ((locals.var_gf_dn5 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn5));
        locals.var_gf2_dn6 = ((locals.var_gf_dn6 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn6));
        locals.var_gf2_dn7 = ((locals.var_gf_dn7 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn7));
        locals.var_gf2_dn8 = ((locals.var_gf_dn8 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn8));

        let assign41090_e54005: f64 = (1.0 / locals.var_gf2);
        locals.var_inv_gf2 = assign41090_e54005;
        locals.var_inv_gf2_dn5 = (-(locals.var_gf2_dn5 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn6 = (-(locals.var_gf2_dn6 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn7 = (-(locals.var_gf2_dn7 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn8 = (-(locals.var_gf2_dn8 / (locals.var_gf2 * locals.var_gf2)));

        let assign41100_e54008: f64 = (locals.var_vsbstar * locals.var_inv_phit1);
        locals.var_ux = assign41100_e54008;
        locals.var_ux_dn5 = ((locals.var_vsbstar_dn5 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn5));
        locals.var_ux_dn6 = ((locals.var_vsbstar_dn6 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn6));
        locals.var_ux_dn7 = ((locals.var_vsbstar_dn7 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn7));
        locals.var_ux_dn8 = ((locals.var_vsbstar_dn8 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn8));

        let assign41110_e54011: f64 = (locals.var_vgb1 * locals.var_inv_phit1);
        locals.var_xg = assign41110_e54011;
        locals.var_xg_dn5 = ((locals.var_vgb1_dn5 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn5));
        locals.var_xg_dn6 = ((locals.var_vgb1_dn6 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn6));
        locals.var_xg_dn7 = ((locals.var_vgb1_dn7 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn7));
        locals.var_xg_dn8 = ((locals.var_vgb1_dn8 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn8));

        let assign41120_e54014: f64 = (2.0 * locals.var_vdsx);
        let assign41120_e54019: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign41120_e54020: f64 = (1.0 + assign41120_e54019);
        let assign41120_e54021: f64 = (assign41120_e54020).sqrt();
        let assign41120_e54022: f64 = (1.0 + assign41120_e54021);
        let assign41120_e54023: f64 = (assign41120_e54014 / assign41120_e54022);
        locals.var_vdsp = assign41120_e54023;
        locals.var_vdsp_dn6 = ((((2.0 * locals.var_vdsx_dn6) * assign41120_e54022) - (assign41120_e54014 * ((locals.var_cfd_i * locals.var_vdsx_dn6) / (2.0 * assign41120_e54021)))) / (assign41120_e54022 * assign41120_e54022));
        locals.var_vdsp_dn7 = ((((2.0 * locals.var_vdsx_dn7) * assign41120_e54022) - (assign41120_e54014 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign41120_e54021)))) / (assign41120_e54022 * assign41120_e54022));

        let assign41130_e54026: f64 = (locals.var_cf_i * locals.var_vdsp);
        let assign41130_e54030: f64 = (locals.var_cfb_i * locals.var_vsbx);
        let assign41130_e54031: f64 = (1.0 + assign41130_e54030);
        let assign41130_e54032: f64 = (assign41130_e54026 * assign41130_e54031);
        locals.var_delphib = assign41130_e54032;
        locals.var_delphib_dn5 = (assign41130_e54026 * (locals.var_cfb_i * locals.var_vsbx_dn5));
        locals.var_delphib_dn6 = (((locals.var_cf_i * locals.var_vdsp_dn6) * assign41130_e54031) + (assign41130_e54026 * (locals.var_cfb_i * locals.var_vsbx_dn6)));
        locals.var_delphib_dn7 = (((locals.var_cf_i * locals.var_vdsp_dn7) * assign41130_e54031) + (assign41130_e54026 * (locals.var_cfb_i * locals.var_vsbx_dn7)));
        locals.var_delphib_dn8 = (assign41130_e54026 * (locals.var_cfb_i * locals.var_vsbx_dn8));

        let assign41140_e54035: f64 = (locals.var_phib * locals.var_inv_phit1);
        locals.var_xb = assign41140_e54035;
        locals.var_xb_dn5 = (locals.var_phib * locals.var_inv_phit1_dn5);
        locals.var_xb_dn6 = (locals.var_phib * locals.var_inv_phit1_dn6);
        locals.var_xb_dn7 = (locals.var_phib * locals.var_inv_phit1_dn7);
        locals.var_xb_dn8 = (locals.var_phib * locals.var_inv_phit1_dn8);

        let assign41150_e54038: f64 = (locals.var_v_xb * locals.var_v_xb);
        let assign41150_e54040: f64 = (assign41150_e54038 + locals.var_aphi);
        let assign41150_e54041: f64 = (assign41150_e54040).sqrt();
        locals.var_temp1 = assign41150_e54041;
        locals.var_temp1_dn5 = 0.0;
        locals.var_temp1_dn6 = (((locals.var_v_xb_dn6 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn6)) / (2.0 * assign41150_e54041));
        locals.var_temp1_dn7 = (((locals.var_v_xb_dn7 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn7)) / (2.0 * assign41150_e54041));
        locals.var_temp1_dn8 = (((locals.var_v_xb_dn8 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn8)) / (2.0 * assign41150_e54041));

        let assign41160_e54044: f64 = (locals.var_v_xb - locals.var_delphib);
        let assign41160_e54047: f64 = (locals.var_v_xb - locals.var_delphib);
        let assign41160_e54048: f64 = (assign41160_e54044 * assign41160_e54047);
        let assign41160_e54050: f64 = (assign41160_e54048 + locals.var_aphi);
        let assign41160_e54051: f64 = (assign41160_e54050).sqrt();
        locals.var_temp2 = assign41160_e54051;
        locals.var_temp2_dn5 = ((((-locals.var_delphib_dn5) * assign41160_e54047) + (assign41160_e54044 * (-locals.var_delphib_dn5))) / (2.0 * assign41160_e54051));
        locals.var_temp2_dn6 = ((((locals.var_v_xb_dn6 - locals.var_delphib_dn6) * assign41160_e54047) + (assign41160_e54044 * (locals.var_v_xb_dn6 - locals.var_delphib_dn6))) / (2.0 * assign41160_e54051));
        locals.var_temp2_dn7 = ((((locals.var_v_xb_dn7 - locals.var_delphib_dn7) * assign41160_e54047) + (assign41160_e54044 * (locals.var_v_xb_dn7 - locals.var_delphib_dn7))) / (2.0 * assign41160_e54051));
        locals.var_temp2_dn8 = ((((locals.var_v_xb_dn8 - locals.var_delphib_dn8) * assign41160_e54047) + (assign41160_e54044 * (locals.var_v_xb_dn8 - locals.var_delphib_dn8))) / (2.0 * assign41160_e54051));

        let assign41170_e54054: f64 = (0.5 * locals.var_inv_phit1);
        let assign41170_e54057: f64 = (locals.var_delphib + locals.var_temp1);
        let assign41170_e54059: f64 = (assign41170_e54057 - locals.var_temp2);
        let assign41170_e54060: f64 = (assign41170_e54054 * assign41170_e54059);
        locals.var_delxb = assign41170_e54060;
        locals.var_delxb_dn5 = (((0.5 * locals.var_inv_phit1_dn5) * assign41170_e54059) + (assign41170_e54054 * ((locals.var_delphib_dn5 + locals.var_temp1_dn5) - locals.var_temp2_dn5)));
        locals.var_delxb_dn6 = (((0.5 * locals.var_inv_phit1_dn6) * assign41170_e54059) + (assign41170_e54054 * ((locals.var_delphib_dn6 + locals.var_temp1_dn6) - locals.var_temp2_dn6)));
        locals.var_delxb_dn7 = (((0.5 * locals.var_inv_phit1_dn7) * assign41170_e54059) + (assign41170_e54054 * ((locals.var_delphib_dn7 + locals.var_temp1_dn7) - locals.var_temp2_dn7)));
        locals.var_delxb_dn8 = (((0.5 * locals.var_inv_phit1_dn8) * assign41170_e54059) + (assign41170_e54054 * ((locals.var_delphib_dn8 + locals.var_temp1_dn8) - locals.var_temp2_dn8)));

        let assign41180_e54063: f64 = (locals.var_xb + locals.var_ux);
        locals.var_xno_s = assign41180_e54063;
        locals.var_xno_s_dn5 = (locals.var_xb_dn5 + locals.var_ux_dn5);
        locals.var_xno_s_dn6 = (locals.var_xb_dn6 + locals.var_ux_dn6);
        locals.var_xno_s_dn7 = (locals.var_xb_dn7 + locals.var_ux_dn7);
        locals.var_xno_s_dn8 = (locals.var_xb_dn8 + locals.var_ux_dn8);

        let assign41190_e54066: f64 = (locals.var_xno_s - locals.var_delxb);
        locals.var_xn_s = assign41190_e54066;
        locals.var_xn_s_dn5 = (locals.var_xno_s_dn5 - locals.var_delxb_dn5);
        locals.var_xn_s_dn6 = (locals.var_xno_s_dn6 - locals.var_delxb_dn6);
        locals.var_xn_s_dn7 = (locals.var_xno_s_dn7 - locals.var_delxb_dn7);
        locals.var_xn_s_dn8 = (locals.var_xno_s_dn8 - locals.var_delxb_dn8);

        let assign41200_e54069: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1175 = assign41200_e54069;

        let assign41210_e54071: f64 = (locals.var_xn_s).abs();
        let assign41210_e54073: f64 = if assign41210_e54071 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1176 = assign41210_e54073;

    }

    pub(super) fn stamp_transient_block_15(
        locals: &mut StampLocals,
    ) {
        let (assign41220_e54093, assign41220_e54093_d_n5, assign41220_e54093_d_n6, assign41220_e54093_d_n7, assign41220_e54093_d_n8,) = {
    if ((locals.var_guard1175 != 0.0) && (locals.var_guard1176 != 0.0)) {
        let assign41220_e54082: f64 = (0.5 * locals.var_xn_s);
        let assign41220_e54086: f64 = (0.3125 * locals.var_xn_s);
        let assign41220_e54087: f64 = (1.0 - assign41220_e54086);
        let assign41220_e54088: f64 = (assign41220_e54082 * assign41220_e54087);
        let assign41220_e54089: f64 = (1.0 - assign41220_e54088);
        let assign41220_e54090: f64 = (locals.var_gf * assign41220_e54089);
        let assign41220_e54091: f64 = (1.0 + assign41220_e54090);
        (assign41220_e54091, ((locals.var_gf_dn5 * assign41220_e54089) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn5) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * locals.var_xn_s_dn5))))))), ((locals.var_gf_dn6 * assign41220_e54089) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn6) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * locals.var_xn_s_dn6))))))), ((locals.var_gf_dn7 * assign41220_e54089) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn7) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * locals.var_xn_s_dn7))))))), ((locals.var_gf_dn8 * assign41220_e54089) + (locals.var_gf * (-(((0.5 * locals.var_xn_s_dn8) * assign41220_e54087) + (assign41220_e54082 * (-(0.3125 * locals.var_xn_s_dn8))))))),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8,)
    }
};
        locals.var_nscr = assign41220_e54093;
        locals.var_nscr_dn5 = assign41220_e54093_d_n5;
        locals.var_nscr_dn6 = assign41220_e54093_d_n6;
        locals.var_nscr_dn7 = assign41220_e54093_d_n7;
        locals.var_nscr_dn8 = assign41220_e54093_d_n8;

        let assign41230_e54096: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1177 = assign41230_e54096;

        let (assign41240_e54107, assign41240_e54107_d_n5, assign41240_e54107_d_n6, assign41240_e54107_d_n7, assign41240_e54107_d_n8,) = {
    if (((locals.var_guard1175 != 0.0) && (locals.var_guard1176 == 0.0)) && (locals.var_guard1177 != 0.0)) {
        let assign41240_e54104: f64 = (-locals.var_xn_s);
        let assign41240_e54105: f64 = (assign41240_e54104).exp();
        (assign41240_e54105, (assign41240_e54105 * (-locals.var_xn_s_dn5)), (assign41240_e54105 * (-locals.var_xn_s_dn6)), (assign41240_e54105 * (-locals.var_xn_s_dn7)), (assign41240_e54105 * (-locals.var_xn_s_dn8)),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8,)
    }
};
        locals.var_delta_ns = assign41240_e54107;
        locals.var_delta_ns_dn5 = assign41240_e54107_d_n5;
        locals.var_delta_ns_dn6 = assign41240_e54107_d_n6;
        locals.var_delta_ns_dn7 = assign41240_e54107_d_n7;
        locals.var_delta_ns_dn8 = assign41240_e54107_d_n8;

        let (assign41250_e54139, assign41250_e54139_d_n5, assign41250_e54139_d_n6, assign41250_e54139_d_n7, assign41250_e54139_d_n8,) = {
    if (((locals.var_guard1175 != 0.0) && (locals.var_guard1176 == 0.0)) && (locals.var_guard1177 == 0.0)) {
        let assign41250_e54119: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41250_e54124: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41250_e54128: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41250_e54130: f64 = (assign41250_e54128 * 0.3333333333333333);
        let assign41250_e54131: f64 = (1.0 + assign41250_e54130);
        let assign41250_e54132: f64 = (assign41250_e54124 * assign41250_e54131);
        let assign41250_e54133: f64 = (0.5 * assign41250_e54132);
        let assign41250_e54134: f64 = (1.0 + assign41250_e54133);
        let assign41250_e54135: f64 = (assign41250_e54119 * assign41250_e54134);
        let assign41250_e54136: f64 = (1.0 + assign41250_e54135);
        let assign41250_e54137: f64 = (1e-200 / assign41250_e54136);
        (assign41250_e54137, (-((1e-200 * ((locals.var_xn_s_dn5 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((locals.var_xn_s_dn5 * assign41250_e54131) + (assign41250_e54124 * (locals.var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((locals.var_xn_s_dn6 * assign41250_e54131) + (assign41250_e54124 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((locals.var_xn_s_dn7 * assign41250_e54131) + (assign41250_e54124 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41250_e54134) + (assign41250_e54119 * (0.5 * ((locals.var_xn_s_dn8 * assign41250_e54131) + (assign41250_e54124 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41250_e54136 * assign41250_e54136))),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8,)
    }
};
        locals.var_delta_ns = assign41250_e54139;
        locals.var_delta_ns_dn5 = assign41250_e54139_d_n5;
        locals.var_delta_ns_dn6 = assign41250_e54139_d_n6;
        locals.var_delta_ns_dn7 = assign41250_e54139_d_n7;
        locals.var_delta_ns_dn8 = assign41250_e54139_d_n8;

        let (assign41260_e54152, assign41260_e54152_d_n5, assign41260_e54152_d_n6, assign41260_e54152_d_n7, assign41260_e54152_d_n8,) = {
    if ((locals.var_guard1175 != 0.0) && (locals.var_guard1176 == 0.0)) {
        let (assign41260_e54150,) = {
            if (locals.var_xn_s > 0.0) {
                (1.0,)
            } else {
                let assign41260_e54149: f64 = (-1.0);
                (assign41260_e54149,)
            }
        };
        (assign41260_e54150, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign41260_e54152;
        locals.var_temp__blk936_dn5 = assign41260_e54152_d_n5;
        locals.var_temp__blk936_dn6 = assign41260_e54152_d_n6;
        locals.var_temp__blk936_dn7 = assign41260_e54152_d_n7;
        locals.var_temp__blk936_dn8 = assign41260_e54152_d_n8;

        let (assign41270_e54180, assign41270_e54180_d_n5, assign41270_e54180_d_n6, assign41270_e54180_d_n7, assign41270_e54180_d_n8,) = {
    if ((locals.var_guard1175 != 0.0) && (locals.var_guard1176 == 0.0)) {
        let assign41270_e54160: f64 = (locals.var_temp__blk936 * locals.var_gf);
        let assign41270_e54165: f64 = (1.0 - locals.var_xn_s);
        let assign41270_e54166: f64 = (locals.var_delta_ns * assign41270_e54165);
        let assign41270_e54167: f64 = (1.0 - assign41270_e54166);
        let assign41270_e54168: f64 = (assign41270_e54160 * assign41270_e54167);
        let assign41270_e54173: f64 = (1.0 - locals.var_delta_ns);
        let assign41270_e54174: f64 = (locals.var_xn_s * assign41270_e54173);
        let assign41270_e54175: f64 = (assign41270_e54174).sqrt();
        let assign41270_e54176: f64 = (2.0 * assign41270_e54175);
        let assign41270_e54177: f64 = (assign41270_e54168 / assign41270_e54176);
        let assign41270_e54178: f64 = (1.0 + assign41270_e54177);
        (assign41270_e54178, (((((((locals.var_temp__blk936_dn5 * locals.var_gf) + (locals.var_temp__blk936 * locals.var_gf_dn5)) * assign41270_e54167) + (assign41270_e54160 * (-((locals.var_delta_ns_dn5 * assign41270_e54165) + (locals.var_delta_ns * (-locals.var_xn_s_dn5)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((locals.var_xn_s_dn5 * assign41270_e54173) + (locals.var_xn_s * (-locals.var_delta_ns_dn5))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((locals.var_temp__blk936_dn6 * locals.var_gf) + (locals.var_temp__blk936 * locals.var_gf_dn6)) * assign41270_e54167) + (assign41270_e54160 * (-((locals.var_delta_ns_dn6 * assign41270_e54165) + (locals.var_delta_ns * (-locals.var_xn_s_dn6)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((locals.var_xn_s_dn6 * assign41270_e54173) + (locals.var_xn_s * (-locals.var_delta_ns_dn6))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((locals.var_temp__blk936_dn7 * locals.var_gf) + (locals.var_temp__blk936 * locals.var_gf_dn7)) * assign41270_e54167) + (assign41270_e54160 * (-((locals.var_delta_ns_dn7 * assign41270_e54165) + (locals.var_delta_ns * (-locals.var_xn_s_dn7)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((locals.var_xn_s_dn7 * assign41270_e54173) + (locals.var_xn_s * (-locals.var_delta_ns_dn7))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)), (((((((locals.var_temp__blk936_dn8 * locals.var_gf) + (locals.var_temp__blk936 * locals.var_gf_dn8)) * assign41270_e54167) + (assign41270_e54160 * (-((locals.var_delta_ns_dn8 * assign41270_e54165) + (locals.var_delta_ns * (-locals.var_xn_s_dn8)))))) * assign41270_e54176) - (assign41270_e54168 * (2.0 * (((locals.var_xn_s_dn8 * assign41270_e54173) + (locals.var_xn_s * (-locals.var_delta_ns_dn8))) / (2.0 * assign41270_e54175))))) / (assign41270_e54176 * assign41270_e54176)),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8,)
    }
};
        locals.var_nscr = assign41270_e54180;
        locals.var_nscr_dn5 = assign41270_e54180_d_n5;
        locals.var_nscr_dn6 = assign41270_e54180_d_n6;
        locals.var_nscr_dn7 = assign41270_e54180_d_n7;
        locals.var_nscr_dn8 = assign41270_e54180_d_n8;

        let (assign41280_e54192, assign41280_e54192_d_n5, assign41280_e54192_d_n6, assign41280_e54192_d_n7, assign41280_e54192_d_n8,) = {
    if (locals.var_guard1175 == 0.0) {
        let assign41280_e54186: f64 = (0.5 * locals.var_gf);
        let assign41280_e54188: f64 = (locals.var_xn_s).sqrt();
        let assign41280_e54189: f64 = (assign41280_e54186 / assign41280_e54188);
        let assign41280_e54190: f64 = (1.0 + assign41280_e54189);
        (assign41280_e54190, ((((0.5 * locals.var_gf_dn5) * assign41280_e54188) - (assign41280_e54186 * (locals.var_xn_s_dn5 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * locals.var_gf_dn6) * assign41280_e54188) - (assign41280_e54186 * (locals.var_xn_s_dn6 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * locals.var_gf_dn7) * assign41280_e54188) - (assign41280_e54186 * (locals.var_xn_s_dn7 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)), ((((0.5 * locals.var_gf_dn8) * assign41280_e54188) - (assign41280_e54186 * (locals.var_xn_s_dn8 / (2.0 * assign41280_e54188)))) / (assign41280_e54188 * assign41280_e54188)),)
    } else {
        (locals.var_nscr, locals.var_nscr_dn5, locals.var_nscr_dn6, locals.var_nscr_dn7, locals.var_nscr_dn8,)
    }
};
        locals.var_nscr = assign41280_e54192;
        locals.var_nscr_dn5 = assign41280_e54192_d_n5;
        locals.var_nscr_dn6 = assign41280_e54192_d_n6;
        locals.var_nscr_dn7 = assign41280_e54192_d_n7;
        locals.var_nscr_dn8 = assign41280_e54192_d_n8;

        let assign41290_e54196: f64 = (locals.var_xn_s).sqrt();
        let assign41290_e54197: f64 = (locals.var_gf * assign41290_e54196);
        let assign41290_e54198: f64 = (locals.var_xn_s + assign41290_e54197);
        let assign41290_e54202: f64 = (locals.var_nscr - 1.0);
        let assign41290_e54203: f64 = (assign41290_e54202).ln();
        let assign41290_e54204: f64 = (locals.var_nscr * assign41290_e54203);
        let assign41290_e54205: f64 = (assign41290_e54198 - assign41290_e54204);
        locals.var_xthscr = assign41290_e54205;
        locals.var_xthscr_dn5 = ((locals.var_xn_s_dn5 + ((locals.var_gf_dn5 * assign41290_e54196) + (locals.var_gf * (locals.var_xn_s_dn5 / (2.0 * assign41290_e54196))))) - ((locals.var_nscr_dn5 * assign41290_e54203) + (locals.var_nscr * (locals.var_nscr_dn5 / assign41290_e54202))));
        locals.var_xthscr_dn6 = ((locals.var_xn_s_dn6 + ((locals.var_gf_dn6 * assign41290_e54196) + (locals.var_gf * (locals.var_xn_s_dn6 / (2.0 * assign41290_e54196))))) - ((locals.var_nscr_dn6 * assign41290_e54203) + (locals.var_nscr * (locals.var_nscr_dn6 / assign41290_e54202))));
        locals.var_xthscr_dn7 = ((locals.var_xn_s_dn7 + ((locals.var_gf_dn7 * assign41290_e54196) + (locals.var_gf * (locals.var_xn_s_dn7 / (2.0 * assign41290_e54196))))) - ((locals.var_nscr_dn7 * assign41290_e54203) + (locals.var_nscr * (locals.var_nscr_dn7 / assign41290_e54202))));
        locals.var_xthscr_dn8 = ((locals.var_xn_s_dn8 + ((locals.var_gf_dn8 * assign41290_e54196) + (locals.var_gf * (locals.var_xn_s_dn8 / (2.0 * assign41290_e54196))))) - ((locals.var_nscr_dn8 * assign41290_e54203) + (locals.var_nscr * (locals.var_nscr_dn8 / assign41290_e54202))));

        let assign41300_e54208: f64 = (locals.var_xg - locals.var_xthscr);
        let assign41300_e54210: f64 = (assign41300_e54208 / locals.var_nscr);
        locals.var_xgtscr = assign41300_e54210;
        locals.var_xgtscr_dn5 = ((((locals.var_xg_dn5 - locals.var_xthscr_dn5) * locals.var_nscr) - (assign41300_e54208 * locals.var_nscr_dn5)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn6 = ((((locals.var_xg_dn6 - locals.var_xthscr_dn6) * locals.var_nscr) - (assign41300_e54208 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn7 = ((((locals.var_xg_dn7 - locals.var_xthscr_dn7) * locals.var_nscr) - (assign41300_e54208 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn8 = ((((locals.var_xg_dn8 - locals.var_xthscr_dn8) * locals.var_nscr) - (assign41300_e54208 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr));

        let assign41310_e54213: f64 = (0.5 * locals.var_gf2);
        let assign41310_e54217: f64 = (8.0 / locals.var_gf2);
        let assign41310_e54218: f64 = (1.0 + assign41310_e54217);
        let assign41310_e54219: f64 = (assign41310_e54218).sqrt();
        let assign41310_e54221: f64 = (assign41310_e54219 - 1.0);
        let assign41310_e54222: f64 = (assign41310_e54213 * assign41310_e54221);
        locals.var_qbscr = assign41310_e54222;
        locals.var_qbscr_dn5 = (((0.5 * locals.var_gf2_dn5) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * locals.var_gf2_dn5) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41310_e54219))));
        locals.var_qbscr_dn6 = (((0.5 * locals.var_gf2_dn6) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41310_e54219))));
        locals.var_qbscr_dn7 = (((0.5 * locals.var_gf2_dn7) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41310_e54219))));
        locals.var_qbscr_dn8 = (((0.5 * locals.var_gf2_dn8) * assign41310_e54221) + (assign41310_e54213 * ((-((8.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) / (2.0 * assign41310_e54219))));

        locals.var_qiscr = 0.0;
        locals.var_qiscr_dn5 = 0.0;
        locals.var_qiscr_dn6 = 0.0;
        locals.var_qiscr_dn7 = 0.0;
        locals.var_qiscr_dn8 = 0.0;

        locals.var_fscr = 1.0;
        locals.var_fscr_dn5 = 0.0;
        locals.var_fscr_dn6 = 0.0;
        locals.var_fscr_dn7 = 0.0;
        locals.var_fscr_dn8 = 0.0;

        let assign41340_e54227: f64 = (-30.0);
        let assign41340_e54228: f64 = if locals.var_xgtscr > assign41340_e54227 { 1.0 } else { 0.0 };
        locals.var_guard1178 = assign41340_e54228;

        let (assign41350_e54236, assign41350_e54236_d_n5, assign41350_e54236_d_n6, assign41350_e54236_d_n7, assign41350_e54236_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41350_e54232: f64 = (locals.var_nscr * locals.var_xgtscr);
        let assign41350_e54234: f64 = (assign41350_e54232 - 1.0);
        (assign41350_e54234, ((locals.var_nscr_dn5 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn5)), ((locals.var_nscr_dn6 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn6)), ((locals.var_nscr_dn7 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn7)), ((locals.var_nscr_dn8 * locals.var_xgtscr) + (locals.var_nscr * locals.var_xgtscr_dn8)),)
    } else {
        (locals.var_xgtscr0, locals.var_xgtscr0_dn5, locals.var_xgtscr0_dn6, locals.var_xgtscr0_dn7, locals.var_xgtscr0_dn8,)
    }
};
        locals.var_xgtscr0 = assign41350_e54236;
        locals.var_xgtscr0_dn5 = assign41350_e54236_d_n5;
        locals.var_xgtscr0_dn6 = assign41350_e54236_d_n6;
        locals.var_xgtscr0_dn7 = assign41350_e54236_d_n7;
        locals.var_xgtscr0_dn8 = assign41350_e54236_d_n8;

        let (assign41360_e54249, assign41360_e54249_d_n5, assign41360_e54249_d_n6, assign41360_e54249_d_n7, assign41360_e54249_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41360_e54242: f64 = (locals.var_xgtscr0 * locals.var_xgtscr0);
        let assign41360_e54244: f64 = (assign41360_e54242 + 10.0);
        let assign41360_e54245: f64 = (assign41360_e54244).sqrt();
        let assign41360_e54246: f64 = (locals.var_xgtscr0 + assign41360_e54245);
        let assign41360_e54247: f64 = (0.5 * assign41360_e54246);
        (assign41360_e54247, (0.5 * (locals.var_xgtscr0_dn5 + (((locals.var_xgtscr0_dn5 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn5)) / (2.0 * assign41360_e54245)))), (0.5 * (locals.var_xgtscr0_dn6 + (((locals.var_xgtscr0_dn6 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn6)) / (2.0 * assign41360_e54245)))), (0.5 * (locals.var_xgtscr0_dn7 + (((locals.var_xgtscr0_dn7 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn7)) / (2.0 * assign41360_e54245)))), (0.5 * (locals.var_xgtscr0_dn8 + (((locals.var_xgtscr0_dn8 * locals.var_xgtscr0) + (locals.var_xgtscr0 * locals.var_xgtscr0_dn8)) / (2.0 * assign41360_e54245)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign41360_e54249;
        locals.var_temp__blk936_dn5 = assign41360_e54249_d_n5;
        locals.var_temp__blk936_dn6 = assign41360_e54249_d_n6;
        locals.var_temp__blk936_dn7 = assign41360_e54249_d_n7;
        locals.var_temp__blk936_dn8 = assign41360_e54249_d_n8;

        let (assign41370_e54256, assign41370_e54256_d_n5, assign41370_e54256_d_n6, assign41370_e54256_d_n7, assign41370_e54256_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41370_e54253: f64 = (locals.var_temp__blk936).ln();
        let assign41370_e54254: f64 = (locals.var_xgtscr - assign41370_e54253);
        (assign41370_e54254, (locals.var_xgtscr_dn5 - (locals.var_temp__blk936_dn5 / locals.var_temp__blk936)), (locals.var_xgtscr_dn6 - (locals.var_temp__blk936_dn6 / locals.var_temp__blk936)), (locals.var_xgtscr_dn7 - (locals.var_temp__blk936_dn7 / locals.var_temp__blk936)), (locals.var_xgtscr_dn8 - (locals.var_temp__blk936_dn8 / locals.var_temp__blk936)),)
    } else {
        (locals.var_qiscr0si, locals.var_qiscr0si_dn5, locals.var_qiscr0si_dn6, locals.var_qiscr0si_dn7, locals.var_qiscr0si_dn8,)
    }
};
        locals.var_qiscr0si = assign41370_e54256;
        locals.var_qiscr0si_dn5 = assign41370_e54256_d_n5;
        locals.var_qiscr0si_dn6 = assign41370_e54256_d_n6;
        locals.var_qiscr0si_dn7 = assign41370_e54256_d_n7;
        locals.var_qiscr0si_dn8 = assign41370_e54256_d_n8;

        let (assign41380_e54269, assign41380_e54269_d_n5, assign41380_e54269_d_n6, assign41380_e54269_d_n7, assign41380_e54269_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41380_e54262: f64 = (locals.var_qiscr0si * locals.var_qiscr0si);
        let assign41380_e54264: f64 = (assign41380_e54262 + 2.0);
        let assign41380_e54265: f64 = (assign41380_e54264).sqrt();
        let assign41380_e54266: f64 = (locals.var_qiscr0si + assign41380_e54265);
        let assign41380_e54267: f64 = (0.5 * assign41380_e54266);
        (assign41380_e54267, (0.5 * (locals.var_qiscr0si_dn5 + (((locals.var_qiscr0si_dn5 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn5)) / (2.0 * assign41380_e54265)))), (0.5 * (locals.var_qiscr0si_dn6 + (((locals.var_qiscr0si_dn6 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn6)) / (2.0 * assign41380_e54265)))), (0.5 * (locals.var_qiscr0si_dn7 + (((locals.var_qiscr0si_dn7 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn7)) / (2.0 * assign41380_e54265)))), (0.5 * (locals.var_qiscr0si_dn8 + (((locals.var_qiscr0si_dn8 * locals.var_qiscr0si) + (locals.var_qiscr0si * locals.var_qiscr0si_dn8)) / (2.0 * assign41380_e54265)))),)
    } else {
        (locals.var_qiscr0, locals.var_qiscr0_dn5, locals.var_qiscr0_dn6, locals.var_qiscr0_dn7, locals.var_qiscr0_dn8,)
    }
};
        locals.var_qiscr0 = assign41380_e54269;
        locals.var_qiscr0_dn5 = assign41380_e54269_d_n5;
        locals.var_qiscr0_dn6 = assign41380_e54269_d_n6;
        locals.var_qiscr0_dn7 = assign41380_e54269_d_n7;
        locals.var_qiscr0_dn8 = assign41380_e54269_d_n8;

        let assign41390_e54272: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41390_e54274: f64 = if assign41390_e54272 < 230.25850929940458 { 1.0 } else { 0.0 };
        locals.var_guard1179 = assign41390_e54274;

        let (assign41400_e54283, assign41400_e54283_d_n5, assign41400_e54283_d_n6, assign41400_e54283_d_n7, assign41400_e54283_d_n8,) = {
    if ((locals.var_guard1178 != 0.0) && (locals.var_guard1179 != 0.0)) {
        let assign41400_e54280: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41400_e54281: f64 = (assign41400_e54280).exp();
        (assign41400_e54281, (assign41400_e54281 * (locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5)), (assign41400_e54281 * (locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6)), (assign41400_e54281 * (locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7)), (assign41400_e54281 * (locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8)),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign41400_e54283;
        locals.var_temp__blk936_dn5 = assign41400_e54283_d_n5;
        locals.var_temp__blk936_dn6 = assign41400_e54283_d_n6;
        locals.var_temp__blk936_dn7 = assign41400_e54283_d_n7;
        locals.var_temp__blk936_dn8 = assign41400_e54283_d_n8;

        let (assign41410_e54318, assign41410_e54318_d_n5, assign41410_e54318_d_n6, assign41410_e54318_d_n7, assign41410_e54318_d_n8,) = {
    if ((locals.var_guard1178 != 0.0) && (locals.var_guard1179 == 0.0)) {
        let assign41410_e54292: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41410_e54294: f64 = (assign41410_e54292 - 230.25850929940458);
        let assign41410_e54299: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41410_e54301: f64 = (assign41410_e54299 - 230.25850929940458);
        let assign41410_e54305: f64 = (locals.var_xgtscr - locals.var_qiscr0);
        let assign41410_e54307: f64 = (assign41410_e54305 - 230.25850929940458);
        let assign41410_e54309: f64 = (assign41410_e54307 * 0.3333333333333333);
        let assign41410_e54310: f64 = (1.0 + assign41410_e54309);
        let assign41410_e54311: f64 = (assign41410_e54301 * assign41410_e54310);
        let assign41410_e54312: f64 = (0.5 * assign41410_e54311);
        let assign41410_e54313: f64 = (1.0 + assign41410_e54312);
        let assign41410_e54314: f64 = (assign41410_e54294 * assign41410_e54313);
        let assign41410_e54315: f64 = (1.0 + assign41410_e54314);
        let assign41410_e54316: f64 = (1e100 * assign41410_e54315);
        (assign41410_e54316, (1e100 * (((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * assign41410_e54310) + (assign41410_e54301 * ((locals.var_xgtscr_dn5 - locals.var_qiscr0_dn5) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * assign41410_e54310) + (assign41410_e54301 * ((locals.var_xgtscr_dn6 - locals.var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * assign41410_e54310) + (assign41410_e54301 * ((locals.var_xgtscr_dn7 - locals.var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41410_e54313) + (assign41410_e54294 * (0.5 * (((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * assign41410_e54310) + (assign41410_e54301 * ((locals.var_xgtscr_dn8 - locals.var_qiscr0_dn8) * 0.3333333333333333))))))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign41410_e54318;
        locals.var_temp__blk936_dn5 = assign41410_e54318_d_n5;
        locals.var_temp__blk936_dn6 = assign41410_e54318_d_n6;
        locals.var_temp__blk936_dn7 = assign41410_e54318_d_n7;
        locals.var_temp__blk936_dn8 = assign41410_e54318_d_n8;

        let (assign41420_e54324, assign41420_e54324_d_n5, assign41420_e54324_d_n6, assign41420_e54324_d_n7, assign41420_e54324_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41420_e54322: f64 = (locals.var_temp__blk936 / locals.var_nscr);
        (assign41420_e54322, (((locals.var_temp__blk936_dn5 * locals.var_nscr) - (locals.var_temp__blk936 * locals.var_nscr_dn5)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk936_dn6 * locals.var_nscr) - (locals.var_temp__blk936 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk936_dn7 * locals.var_nscr) - (locals.var_temp__blk936 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr)), (((locals.var_temp__blk936_dn8 * locals.var_nscr) - (locals.var_temp__blk936 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr)),)
    } else {
        (locals.var_dscr0, locals.var_dscr0_dn5, locals.var_dscr0_dn6, locals.var_dscr0_dn7, locals.var_dscr0_dn8,)
    }
};
        locals.var_dscr0 = assign41420_e54324;
        locals.var_dscr0_dn5 = assign41420_e54324_d_n5;
        locals.var_dscr0_dn6 = assign41420_e54324_d_n6;
        locals.var_dscr0_dn7 = assign41420_e54324_d_n7;
        locals.var_dscr0_dn8 = assign41420_e54324_d_n8;

        let (assign41430_e54334, assign41430_e54334_d_n5, assign41430_e54334_d_n6, assign41430_e54334_d_n7, assign41430_e54334_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41430_e54329: f64 = (locals.var_qiscr0 + 1.0);
        let assign41430_e54330: f64 = (2.0 * assign41430_e54329);
        let assign41430_e54332: f64 = (assign41430_e54330 - locals.var_dscr0);
        (assign41430_e54332, ((2.0 * locals.var_qiscr0_dn5) - locals.var_dscr0_dn5), ((2.0 * locals.var_qiscr0_dn6) - locals.var_dscr0_dn6), ((2.0 * locals.var_qiscr0_dn7) - locals.var_dscr0_dn7), ((2.0 * locals.var_qiscr0_dn8) - locals.var_dscr0_dn8),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign41430_e54334;
        locals.var_temp__blk936_dn5 = assign41430_e54334_d_n5;
        locals.var_temp__blk936_dn6 = assign41430_e54334_d_n6;
        locals.var_temp__blk936_dn7 = assign41430_e54334_d_n7;
        locals.var_temp__blk936_dn8 = assign41430_e54334_d_n8;

        let assign41440_e54337: f64 = if locals.var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard1180 = assign41440_e54337;

        let (assign41450_e54358, assign41450_e54358_d_n5, assign41450_e54358_d_n6, assign41450_e54358_d_n7, assign41450_e54358_d_n8,) = {
    if ((locals.var_guard1178 != 0.0) && (locals.var_guard1180 != 0.0)) {
        let assign41450_e54346: f64 = (locals.var_dscr0 * locals.var_temp__blk936);
        let assign41450_e54347: f64 = (1.0 + assign41450_e54346);
        let assign41450_e54348: f64 = (assign41450_e54347).sqrt();
        let assign41450_e54350: f64 = (assign41450_e54348 - 1.0);
        let assign41450_e54352: f64 = (assign41450_e54350 / locals.var_dscr0);
        let assign41450_e54353: f64 = (locals.var_qiscr0 - assign41450_e54352);
        let assign41450_e54355: f64 = (assign41450_e54353 + 1.0);
        let assign41450_e54356: f64 = (locals.var_nscr * assign41450_e54355);
        (assign41450_e54356, ((locals.var_nscr_dn5 * assign41450_e54355) + (locals.var_nscr * (locals.var_qiscr0_dn5 - ((((((locals.var_dscr0_dn5 * locals.var_temp__blk936) + (locals.var_dscr0 * locals.var_temp__blk936_dn5)) / (2.0 * assign41450_e54348)) * locals.var_dscr0) - (assign41450_e54350 * locals.var_dscr0_dn5)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn6 * assign41450_e54355) + (locals.var_nscr * (locals.var_qiscr0_dn6 - ((((((locals.var_dscr0_dn6 * locals.var_temp__blk936) + (locals.var_dscr0 * locals.var_temp__blk936_dn6)) / (2.0 * assign41450_e54348)) * locals.var_dscr0) - (assign41450_e54350 * locals.var_dscr0_dn6)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn7 * assign41450_e54355) + (locals.var_nscr * (locals.var_qiscr0_dn7 - ((((((locals.var_dscr0_dn7 * locals.var_temp__blk936) + (locals.var_dscr0 * locals.var_temp__blk936_dn7)) / (2.0 * assign41450_e54348)) * locals.var_dscr0) - (assign41450_e54350 * locals.var_dscr0_dn7)) / (locals.var_dscr0 * locals.var_dscr0))))), ((locals.var_nscr_dn8 * assign41450_e54355) + (locals.var_nscr * (locals.var_qiscr0_dn8 - ((((((locals.var_dscr0_dn8 * locals.var_temp__blk936) + (locals.var_dscr0 * locals.var_temp__blk936_dn8)) / (2.0 * assign41450_e54348)) * locals.var_dscr0) - (assign41450_e54350 * locals.var_dscr0_dn8)) / (locals.var_dscr0 * locals.var_dscr0))))),)
    } else {
        (locals.var_qiscr, locals.var_qiscr_dn5, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8,)
    }
};
        locals.var_qiscr = assign41450_e54358;
        locals.var_qiscr_dn5 = assign41450_e54358_d_n5;
        locals.var_qiscr_dn6 = assign41450_e54358_d_n6;
        locals.var_qiscr_dn7 = assign41450_e54358_d_n7;
        locals.var_qiscr_dn8 = assign41450_e54358_d_n8;

        let (assign41460_e54377, assign41460_e54377_d_n5, assign41460_e54377_d_n6, assign41460_e54377_d_n7, assign41460_e54377_d_n8,) = {
    if ((locals.var_guard1178 != 0.0) && (locals.var_guard1180 == 0.0)) {
        let assign41460_e54365: f64 = (locals.var_nscr * 0.5);
        let assign41460_e54367: f64 = (assign41460_e54365 * locals.var_dscr0);
        let assign41460_e54371: f64 = (0.25 * locals.var_temp__blk936);
        let assign41460_e54373: f64 = (assign41460_e54371 * locals.var_temp__blk936);
        let assign41460_e54374: f64 = (1.0 + assign41460_e54373);
        let assign41460_e54375: f64 = (assign41460_e54367 * assign41460_e54374);
        (assign41460_e54375, (((((locals.var_nscr_dn5 * 0.5) * locals.var_dscr0) + (assign41460_e54365 * locals.var_dscr0_dn5)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * locals.var_temp__blk936_dn5) * locals.var_temp__blk936) + (assign41460_e54371 * locals.var_temp__blk936_dn5)))), (((((locals.var_nscr_dn6 * 0.5) * locals.var_dscr0) + (assign41460_e54365 * locals.var_dscr0_dn6)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * locals.var_temp__blk936_dn6) * locals.var_temp__blk936) + (assign41460_e54371 * locals.var_temp__blk936_dn6)))), (((((locals.var_nscr_dn7 * 0.5) * locals.var_dscr0) + (assign41460_e54365 * locals.var_dscr0_dn7)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * locals.var_temp__blk936_dn7) * locals.var_temp__blk936) + (assign41460_e54371 * locals.var_temp__blk936_dn7)))), (((((locals.var_nscr_dn8 * 0.5) * locals.var_dscr0) + (assign41460_e54365 * locals.var_dscr0_dn8)) * assign41460_e54374) + (assign41460_e54367 * (((0.25 * locals.var_temp__blk936_dn8) * locals.var_temp__blk936) + (assign41460_e54371 * locals.var_temp__blk936_dn8)))),)
    } else {
        (locals.var_qiscr, locals.var_qiscr_dn5, locals.var_qiscr_dn6, locals.var_qiscr_dn7, locals.var_qiscr_dn8,)
    }
};
        locals.var_qiscr = assign41460_e54377;
        locals.var_qiscr_dn5 = assign41460_e54377_d_n5;
        locals.var_qiscr_dn6 = assign41460_e54377_d_n6;
        locals.var_qiscr_dn7 = assign41460_e54377_d_n7;
        locals.var_qiscr_dn8 = assign41460_e54377_d_n8;

        let (assign41470_e54402, assign41470_e54402_d_n5, assign41470_e54402_d_n6, assign41470_e54402_d_n7, assign41470_e54402_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41470_e54382: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41470_e54384: f64 = (assign41470_e54382 + 2.0);
        let assign41470_e54387: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41470_e54389: f64 = (assign41470_e54387 - 2.0);
        let assign41470_e54392: f64 = (locals.var_xg - locals.var_qiscr);
        let assign41470_e54394: f64 = (assign41470_e54392 - 2.0);
        let assign41470_e54395: f64 = (assign41470_e54389 * assign41470_e54394);
        let assign41470_e54397: f64 = (assign41470_e54395 + 1.0);
        let assign41470_e54398: f64 = (assign41470_e54397).sqrt();
        let assign41470_e54399: f64 = (assign41470_e54384 + assign41470_e54398);
        let assign41470_e54400: f64 = (0.5 * assign41470_e54399);
        (assign41470_e54400, (0.5 * ((locals.var_xg_dn5 - locals.var_qiscr_dn5) + ((((locals.var_xg_dn5 - locals.var_qiscr_dn5) * assign41470_e54394) + (assign41470_e54389 * (locals.var_xg_dn5 - locals.var_qiscr_dn5))) / (2.0 * assign41470_e54398)))), (0.5 * ((locals.var_xg_dn6 - locals.var_qiscr_dn6) + ((((locals.var_xg_dn6 - locals.var_qiscr_dn6) * assign41470_e54394) + (assign41470_e54389 * (locals.var_xg_dn6 - locals.var_qiscr_dn6))) / (2.0 * assign41470_e54398)))), (0.5 * ((locals.var_xg_dn7 - locals.var_qiscr_dn7) + ((((locals.var_xg_dn7 - locals.var_qiscr_dn7) * assign41470_e54394) + (assign41470_e54389 * (locals.var_xg_dn7 - locals.var_qiscr_dn7))) / (2.0 * assign41470_e54398)))), (0.5 * ((locals.var_xg_dn8 - locals.var_qiscr_dn8) + ((((locals.var_xg_dn8 - locals.var_qiscr_dn8) * assign41470_e54394) + (assign41470_e54389 * (locals.var_xg_dn8 - locals.var_qiscr_dn8))) / (2.0 * assign41470_e54398)))),)
    } else {
        (locals.var_temp__blk936, locals.var_temp__blk936_dn5, locals.var_temp__blk936_dn6, locals.var_temp__blk936_dn7, locals.var_temp__blk936_dn8,)
    }
};
        locals.var_temp__blk936 = assign41470_e54402;
        locals.var_temp__blk936_dn5 = assign41470_e54402_d_n5;
        locals.var_temp__blk936_dn6 = assign41470_e54402_d_n6;
        locals.var_temp__blk936_dn7 = assign41470_e54402_d_n7;
        locals.var_temp__blk936_dn8 = assign41470_e54402_d_n8;

        let (assign41480_e54419, assign41480_e54419_d_n5, assign41480_e54419_d_n6, assign41480_e54419_d_n7, assign41480_e54419_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41480_e54406: f64 = (0.5 * locals.var_gf2);
        let assign41480_e54410: f64 = (4.0 / locals.var_gf2);
        let assign41480_e54412: f64 = (assign41480_e54410 * locals.var_temp__blk936);
        let assign41480_e54413: f64 = (1.0 + assign41480_e54412);
        let assign41480_e54414: f64 = (assign41480_e54413).sqrt();
        let assign41480_e54416: f64 = (assign41480_e54414 - 1.0);
        let assign41480_e54417: f64 = (assign41480_e54406 * assign41480_e54416);
        (assign41480_e54417, (((0.5 * locals.var_gf2_dn5) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * locals.var_gf2_dn5) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk936) + (assign41480_e54410 * locals.var_temp__blk936_dn5)) / (2.0 * assign41480_e54414)))), (((0.5 * locals.var_gf2_dn6) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * locals.var_gf2_dn6) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk936) + (assign41480_e54410 * locals.var_temp__blk936_dn6)) / (2.0 * assign41480_e54414)))), (((0.5 * locals.var_gf2_dn7) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * locals.var_gf2_dn7) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk936) + (assign41480_e54410 * locals.var_temp__blk936_dn7)) / (2.0 * assign41480_e54414)))), (((0.5 * locals.var_gf2_dn8) * assign41480_e54416) + (assign41480_e54406 * ((((-((4.0 * locals.var_gf2_dn8) / (locals.var_gf2 * locals.var_gf2))) * locals.var_temp__blk936) + (assign41480_e54410 * locals.var_temp__blk936_dn8)) / (2.0 * assign41480_e54414)))),)
    } else {
        (locals.var_qbscr, locals.var_qbscr_dn5, locals.var_qbscr_dn6, locals.var_qbscr_dn7, locals.var_qbscr_dn8,)
    }
};
        locals.var_qbscr = assign41480_e54419;
        locals.var_qbscr_dn5 = assign41480_e54419_d_n5;
        locals.var_qbscr_dn6 = assign41480_e54419_d_n6;
        locals.var_qbscr_dn7 = assign41480_e54419_d_n7;
        locals.var_qbscr_dn8 = assign41480_e54419_d_n8;

        let (assign41490_e54427, assign41490_e54427_d_n5, assign41490_e54427_d_n6, assign41490_e54427_d_n7, assign41490_e54427_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41490_e54424: f64 = (locals.var_qbscr + locals.var_qiscr);
        let assign41490_e54425: f64 = (locals.var_qbscr / assign41490_e54424);
        (assign41490_e54425, (((locals.var_qbscr_dn5 * assign41490_e54424) - (locals.var_qbscr * (locals.var_qbscr_dn5 + locals.var_qiscr_dn5))) / (assign41490_e54424 * assign41490_e54424)), (((locals.var_qbscr_dn6 * assign41490_e54424) - (locals.var_qbscr * (locals.var_qbscr_dn6 + locals.var_qiscr_dn6))) / (assign41490_e54424 * assign41490_e54424)), (((locals.var_qbscr_dn7 * assign41490_e54424) - (locals.var_qbscr * (locals.var_qbscr_dn7 + locals.var_qiscr_dn7))) / (assign41490_e54424 * assign41490_e54424)), (((locals.var_qbscr_dn8 * assign41490_e54424) - (locals.var_qbscr * (locals.var_qbscr_dn8 + locals.var_qiscr_dn8))) / (assign41490_e54424 * assign41490_e54424)),)
    } else {
        (locals.var_fscr, locals.var_fscr_dn5, locals.var_fscr_dn6, locals.var_fscr_dn7, locals.var_fscr_dn8,)
    }
};
        locals.var_fscr = assign41490_e54427;
        locals.var_fscr_dn5 = assign41490_e54427_d_n5;
        locals.var_fscr_dn6 = assign41490_e54427_d_n6;
        locals.var_fscr_dn7 = assign41490_e54427_d_n7;
        locals.var_fscr_dn8 = assign41490_e54427_d_n8;

        let (assign41500_e54435, assign41500_e54435_d_n5, assign41500_e54435_d_n6, assign41500_e54435_d_n7, assign41500_e54435_d_n8,) = {
    if (locals.var_guard1178 != 0.0) {
        let assign41500_e54432: f64 = (locals.var_fscr * locals.var_delxb);
        let assign41500_e54433: f64 = (locals.var_xno_s - assign41500_e54432);
        (assign41500_e54433, (locals.var_xno_s_dn5 - ((locals.var_fscr_dn5 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn5))), (locals.var_xno_s_dn6 - ((locals.var_fscr_dn6 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn6))), (locals.var_xno_s_dn7 - ((locals.var_fscr_dn7 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn7))), (locals.var_xno_s_dn8 - ((locals.var_fscr_dn8 * locals.var_delxb) + (locals.var_fscr * locals.var_delxb_dn8))),)
    } else {
        (locals.var_xn_s, locals.var_xn_s_dn5, locals.var_xn_s_dn6, locals.var_xn_s_dn7, locals.var_xn_s_dn8,)
    }
};
        locals.var_xn_s = assign41500_e54435;
        locals.var_xn_s_dn5 = assign41500_e54435_d_n5;
        locals.var_xn_s_dn6 = assign41500_e54435_d_n6;
        locals.var_xn_s_dn7 = assign41500_e54435_d_n7;
        locals.var_xn_s_dn8 = assign41500_e54435_d_n8;

        let assign41510_e54439: f64 = (locals.var_gf * 0.7071067811865475);
        let assign41510_e54440: f64 = (1.0 + assign41510_e54439);
        locals.var_xi = assign41510_e54440;
        locals.var_xi_dn5 = (locals.var_gf_dn5 * 0.7071067811865475);
        locals.var_xi_dn6 = (locals.var_gf_dn6 * 0.7071067811865475);
        locals.var_xi_dn7 = (locals.var_gf_dn7 * 0.7071067811865475);
        locals.var_xi_dn8 = (locals.var_gf_dn8 * 0.7071067811865475);

        let assign41520_e54443: f64 = (1e-5 * locals.var_xi);
        locals.var_margin = assign41520_e54443;

        let assign41530_e54446: f64 = (1.0 / locals.var_xi);
        locals.var_inv_xi = assign41530_e54446;
        locals.var_inv_xi_dn5 = (-(locals.var_xi_dn5 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn6 = (-(locals.var_xi_dn6 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn7 = (-(locals.var_xi_dn7 / (locals.var_xi * locals.var_xi)));
        locals.var_inv_xi_dn8 = (-(locals.var_xi_dn8 / (locals.var_xi * locals.var_xi)));

        locals.var_sp_s_x1 = 0.0;
        locals.var_sp_s_x1_dn5 = 0.0;
        locals.var_sp_s_x1_dn6 = 0.0;
        locals.var_sp_s_x1_dn7 = 0.0;
        locals.var_sp_s_x1_dn8 = 0.0;

        locals.var_x_s = 0.0;
        locals.var_x_s_dn5 = 0.0;
        locals.var_x_s_dn6 = 0.0;
        locals.var_x_s_dn7 = 0.0;
        locals.var_x_s_dn8 = 0.0;

        let assign41560_e54451: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1181 = assign41560_e54451;

        let (assign41570_e54457, assign41570_e54457_d_n5, assign41570_e54457_d_n6, assign41570_e54457_d_n7, assign41570_e54457_d_n8,) = {
    if (locals.var_guard1181 != 0.0) {
        let assign41570_e54454: f64 = (-locals.var_xn_s);
        let assign41570_e54455: f64 = (assign41570_e54454).exp();
        (assign41570_e54455, (assign41570_e54455 * (-locals.var_xn_s_dn5)), (assign41570_e54455 * (-locals.var_xn_s_dn6)), (assign41570_e54455 * (-locals.var_xn_s_dn7)), (assign41570_e54455 * (-locals.var_xn_s_dn8)),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8,)
    }
};
        locals.var_delta_ns = assign41570_e54457;
        locals.var_delta_ns_dn5 = assign41570_e54457_d_n5;
        locals.var_delta_ns_dn6 = assign41570_e54457_d_n6;
        locals.var_delta_ns_dn7 = assign41570_e54457_d_n7;
        locals.var_delta_ns_dn8 = assign41570_e54457_d_n8;

        let (assign41580_e54484, assign41580_e54484_d_n5, assign41580_e54484_d_n6, assign41580_e54484_d_n7, assign41580_e54484_d_n8,) = {
    if (locals.var_guard1181 == 0.0) {
        let assign41580_e54464: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41580_e54469: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41580_e54473: f64 = (locals.var_xn_s - 460.51701859880916);
        let assign41580_e54475: f64 = (assign41580_e54473 * 0.3333333333333333);
        let assign41580_e54476: f64 = (1.0 + assign41580_e54475);
        let assign41580_e54477: f64 = (assign41580_e54469 * assign41580_e54476);
        let assign41580_e54478: f64 = (0.5 * assign41580_e54477);
        let assign41580_e54479: f64 = (1.0 + assign41580_e54478);
        let assign41580_e54480: f64 = (assign41580_e54464 * assign41580_e54479);
        let assign41580_e54481: f64 = (1.0 + assign41580_e54480);
        let assign41580_e54482: f64 = (1e-200 / assign41580_e54481);
        (assign41580_e54482, (-((1e-200 * ((locals.var_xn_s_dn5 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((locals.var_xn_s_dn5 * assign41580_e54476) + (assign41580_e54469 * (locals.var_xn_s_dn5 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((locals.var_xn_s_dn6 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((locals.var_xn_s_dn6 * assign41580_e54476) + (assign41580_e54469 * (locals.var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((locals.var_xn_s_dn7 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((locals.var_xn_s_dn7 * assign41580_e54476) + (assign41580_e54469 * (locals.var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))), (-((1e-200 * ((locals.var_xn_s_dn8 * assign41580_e54479) + (assign41580_e54464 * (0.5 * ((locals.var_xn_s_dn8 * assign41580_e54476) + (assign41580_e54469 * (locals.var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41580_e54481 * assign41580_e54481))),)
    } else {
        (locals.var_delta_ns, locals.var_delta_ns_dn5, locals.var_delta_ns_dn6, locals.var_delta_ns_dn7, locals.var_delta_ns_dn8,)
    }
};
        locals.var_delta_ns = assign41580_e54484;
        locals.var_delta_ns_dn5 = assign41580_e54484_d_n5;
        locals.var_delta_ns_dn6 = assign41580_e54484_d_n6;
        locals.var_delta_ns_dn7 = assign41580_e54484_d_n7;
        locals.var_delta_ns_dn8 = assign41580_e54484_d_n8;

        let assign41590_e54486: f64 = (locals.var_xg).abs();
        let assign41590_e54488: f64 = if assign41590_e54486 <= locals.var_margin { 1.0 } else { 0.0 };
        locals.var_guard1182 = assign41590_e54488;

    }
}
