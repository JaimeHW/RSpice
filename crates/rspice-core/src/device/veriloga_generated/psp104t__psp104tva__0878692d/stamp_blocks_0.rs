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
        let assign00_e1484: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign00_e1484;

        let (assign10_e1489,) = {
    if (locals.var_guard1 != 0.0) {
        let assign10_e1487: f64 = 1.0;
        (assign10_e1487,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign10_e1489;

        let (assign20_e1495,) = {
    if (locals.var_guard1 == 0.0) {
        let assign20_e1493: f64 = (-1.0);
        (assign20_e1493,)
    } else {
        (locals.var_chnl_type,)
    }
};
        locals.var_chnl_type = assign20_e1495;

        let assign30_e1498: f64 = (8.8541878176e-12 * 11.8);
        locals.var_epssi = assign30_e1498;

        let assign40_e1501: f64 = (273.15 + p.p38);
        locals.var_tkr = assign40_e1501;

        let assign2050_e2532: f64 = ctx_temp;
        let assign2050_e2534: f64 = (assign2050_e2532 + p.p55);
        let assign2050_e2536: f64 = (assign2050_e2534 + p.p35);
        locals.var_tka = assign2050_e2536;

        let assign2060_e2539: f64 = (locals.var_tka / locals.var_tkr);
        locals.var_rta = assign2060_e2539;

        let assign2070_e2542: f64 = (locals.var_tka - locals.var_tkr);
        locals.var_delta = assign2070_e2542;

        let assign2080_e2545: f64 = (locals.var_tka * 1.3806505e-23);
        let assign2080_e2547: f64 = (assign2080_e2545 / 1.6021918e-19);
        locals.var_phita = assign2080_e2547;

        let assign2090_e2550: f64 = (1.0 / locals.var_phita);
        locals.var_inv_phita = assign2090_e2550;

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

        let assign3390_e3398: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard29 = assign3390_e3398;

        let (assign3400_e3407,) = {
    if (locals.var_guard29 != 0.0) {
        let (assign3400_e3405,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3400_e3405,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3400_e3407;

        let (assign3410_e3414,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3410_e3411: f64 = (locals.var_nf_i + 0.5);
        let assign3410_e3412: f64 = (assign3410_e3411).floor();
        (assign3410_e3412,)
    } else {
        (locals.var_nf_i,)
    }
};
        locals.var_nf_i = assign3410_e3414;

        let (assign3420_e3420,) = {
    if (locals.var_guard29 != 0.0) {
        let assign3420_e3418: f64 = (1.0 / locals.var_nf_i);
        (assign3420_e3418,)
    } else {
        (locals.var_invnf,)
    }
};
        locals.var_invnf = assign3420_e3420;

        let assign3430_e3423: f64 = (locals.var_w_i * locals.var_invnf);
        let (assign3430_e3430,) = {
    if (assign3430_e3423 > 1e-9) {
        let assign3430_e3428: f64 = (locals.var_w_i * locals.var_invnf);
        (assign3430_e3428,)
    } else {
        (1e-9,)
    }
};
        locals.var_w_i = assign3430_e3430;

        locals.var_sca_i = p.p5;

        locals.var_scb_i = p.p6;

        locals.var_scc_i = p.p7;

        let assign3480_e3442: f64 = (1e-6 / locals.var_l_i);
        locals.var_il = assign3480_e3442;

        let assign3490_e3445: f64 = (1e-6 / locals.var_w_i);
        locals.var_iw = assign3490_e3445;

        let assign3500_e3450: f64 = (p.p190 * locals.var_il);
        let assign3500_e3451: f64 = (1.0 + assign3500_e3450);
        let assign3500_e3452: f64 = (p.p189 * assign3500_e3451);
        let assign3500_e3456: f64 = (p.p191 * locals.var_iw);
        let assign3500_e3457: f64 = (1.0 + assign3500_e3456);
        let assign3500_e3458: f64 = (assign3500_e3452 * assign3500_e3457);
        locals.var_dellps = assign3500_e3458;

        let assign3510_e3463: f64 = (p.p194 * locals.var_il);
        let assign3510_e3464: f64 = (1.0 + assign3510_e3463);
        let assign3510_e3465: f64 = (p.p193 * assign3510_e3464);
        let assign3510_e3469: f64 = (p.p195 * locals.var_iw);
        let assign3510_e3470: f64 = (1.0 + assign3510_e3469);
        let assign3510_e3471: f64 = (assign3510_e3465 * assign3510_e3470);
        locals.var_delwod = assign3510_e3471;

        let assign3520_e3474: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3520_e3477: f64 = (2.0 * p.p192);
        let assign3520_e3478: f64 = (assign3520_e3474 - assign3520_e3477);
        let (assign3520_e3489,) = {
    if (assign3520_e3478 > 1e-9) {
        let assign3520_e3483: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3520_e3486: f64 = (2.0 * p.p192);
        let assign3520_e3487: f64 = (assign3520_e3483 - assign3520_e3486);
        (assign3520_e3487,)
    } else {
        (1e-9,)
    }
};
        locals.var_le = assign3520_e3489;

        let assign3530_e3492: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3530_e3495: f64 = (2.0 * p.p196);
        let assign3530_e3496: f64 = (assign3530_e3492 - assign3530_e3495);
        let (assign3530_e3507,) = {
    if (assign3530_e3496 > 1e-9) {
        let assign3530_e3501: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3530_e3504: f64 = (2.0 * p.p196);
        let assign3530_e3505: f64 = (assign3530_e3501 - assign3530_e3504);
        (assign3530_e3505,)
    } else {
        (1e-9,)
    }
};
        locals.var_we = assign3530_e3507;

        let assign3540_e3510: f64 = (1e-6 / locals.var_le);
        locals.var_ile = assign3540_e3510;

        let assign3550_e3513: f64 = (locals.var_ile * locals.var_ile);
        locals.var_ile2 = assign3550_e3513;

        let assign3560_e3516: f64 = (1e-6 / locals.var_we);
        locals.var_iwe = assign3560_e3516;

        let assign3570_e3519: f64 = (1.0 / locals.var_iwe);
        locals.var_iiwe = assign3570_e3519;

        let assign3580_e3522: f64 = (locals.var_ile * locals.var_iwe);
        locals.var_iae = assign3580_e3522;

        let assign3590_e3525: f64 = (1.0 / locals.var_iae);
        locals.var_iiae = assign3590_e3525;

        let assign3600_e3528: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3600_e3531: f64 = (2.0 * p.p192);
        let assign3600_e3532: f64 = (assign3600_e3528 - assign3600_e3531);
        let assign3600_e3534: f64 = (assign3600_e3532 + p.p197);
        let (assign3600_e3547,) = {
    if (assign3600_e3534 > 1e-9) {
        let assign3600_e3539: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3600_e3542: f64 = (2.0 * p.p192);
        let assign3600_e3543: f64 = (assign3600_e3539 - assign3600_e3542);
        let assign3600_e3545: f64 = (assign3600_e3543 + p.p197);
        (assign3600_e3545,)
    } else {
        (1e-9,)
    }
};
        locals.var_lecv = assign3600_e3547;

        let assign3610_e3550: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3610_e3553: f64 = (2.0 * p.p196);
        let assign3610_e3554: f64 = (assign3610_e3550 - assign3610_e3553);
        let assign3610_e3556: f64 = (assign3610_e3554 + p.p198);
        let (assign3610_e3569,) = {
    if (assign3610_e3556 > 1e-9) {
        let assign3610_e3561: f64 = (locals.var_w_i + locals.var_delwod);
        let assign3610_e3564: f64 = (2.0 * p.p196);
        let assign3610_e3565: f64 = (assign3610_e3561 - assign3610_e3564);
        let assign3610_e3567: f64 = (assign3610_e3565 + p.p198);
        (assign3610_e3567,)
    } else {
        (1e-9,)
    }
};
        locals.var_wecv = assign3610_e3569;

        let assign3620_e3572: f64 = (locals.var_wecv / 1e-6);
        locals.var_iiwecv = assign3620_e3572;

        let assign3630_e3575: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3630_e3577: f64 = (assign3630_e3575 + p.p197);
        let (assign3630_e3586,) = {
    if (assign3630_e3577 > 1e-9) {
        let assign3630_e3582: f64 = (locals.var_l_i + locals.var_dellps);
        let assign3630_e3584: f64 = (assign3630_e3582 + p.p197);
        (assign3630_e3584,)
    } else {
        (1e-9,)
    }
};
        locals.var_lcv = assign3630_e3586;

        let assign3650_e3603: f64 = (locals.var_lcv / 1e-6);
        locals.var_iilcv = assign3650_e3603;

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

        let assign4370_e3718: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4370_e3720: f64 = if assign4370_e3718 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign4370_e3720;

        let (assign4380_e3724,) = {
    if (locals.var_guard30 != 0.0) {
        (p.p121,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign4380_e3724;

        locals.var_gc3ov_p = p.p120;

        let assign4400_e3727: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4400_e3729: f64 = if assign4400_e3727 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign4400_e3729;

        let (assign4410_e3733,) = {
    if (locals.var_guard31 != 0.0) {
        (p.p122,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign4410_e3733;

        locals.var_gc2ovd_p = locals.var_gc2ov_p;

        let assign4430_e3736: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4430_e3738: f64 = if assign4430_e3736 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign4430_e3738;

        let (assign4440_e3742,) = {
    if (locals.var_guard32 != 0.0) {
        (p.p123,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign4440_e3742;

        locals.var_gc3ovd_p = locals.var_gc3ov_p;

        let assign4460_e3745: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4460_e3747: f64 = if assign4460_e3745 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard33 = assign4460_e3747;

        let (assign4470_e3751,) = {
    if (locals.var_guard33 != 0.0) {
        (p.p124,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign4470_e3751;

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

        locals.var_thesatac_p = p.p98;

        let assign4610_e3766: f64 = if param_given[137] { 1.0 } else { 0.0 };
        let assign4610_e3768: f64 = if assign4610_e3766 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard34 = assign4610_e3768;

        let (assign4620_e3772,) = {
    if (locals.var_guard34 != 0.0) {
        (p.p137,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign4620_e3772;

        locals.var_axac_p = p.p103;

        let assign4640_e3775: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4640_e3777: f64 = if assign4640_e3775 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard35 = assign4640_e3777;

        let (assign4650_e3781,) = {
    if (locals.var_guard35 != 0.0) {
        (p.p138,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign4650_e3781;

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

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        locals.var_rse_p = p.p180;

        locals.var_rde_p = p.p181;

        locals.var_rth_p = p.p186;

        let assign5160_e3834: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard36 = assign5160_e3834;

        let (assign5170_e3852,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5170_e3840: f64 = (locals.var_ile).powf(p.p201);
        let assign5170_e3841: f64 = (p.p200 * assign5170_e3840);
        let assign5170_e3842: f64 = (p.p199 + assign5170_e3841);
        let assign5170_e3845: f64 = (p.p202 * locals.var_iwe);
        let assign5170_e3846: f64 = (assign5170_e3842 + assign5170_e3845);
        let assign5170_e3849: f64 = (p.p203 * locals.var_iae);
        let assign5170_e3850: f64 = (assign5170_e3846 + assign5170_e3849);
        (assign5170_e3850,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign5170_e3852;

        let (assign5180_e3868,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5180_e3857: f64 = (p.p205 * locals.var_ile);
        let assign5180_e3858: f64 = (p.p204 + assign5180_e3857);
        let assign5180_e3861: f64 = (p.p206 * locals.var_iwe);
        let assign5180_e3862: f64 = (assign5180_e3858 + assign5180_e3861);
        let assign5180_e3865: f64 = (p.p207 * locals.var_iae);
        let assign5180_e3866: f64 = (assign5180_e3862 + assign5180_e3865);
        (assign5180_e3866,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign5180_e3868;

        let (assign5190_e3872,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p208,)
    } else {
        (locals.var_st2vfb_p,)
    }
};
        locals.var_st2vfb_p = assign5190_e3872;

        let (assign5200_e3876,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p209,)
    } else {
        (locals.var_tox_p,)
    }
};
        locals.var_tox_p = assign5200_e3876;

        let (assign5210_e3880,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p210,)
    } else {
        (locals.var_epsrox_p,)
    }
};
        locals.var_epsrox_p = assign5210_e3880;

        let (assign5220_e3913,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5220_e3886: f64 = (p.p212 * locals.var_iwe);
        let assign5220_e3890: f64 = (locals.var_we / p.p213);
        let assign5220_e3891: f64 = (1.0 + assign5220_e3890);
        let assign5220_e3892: f64 = (assign5220_e3891).ln();
        let assign5220_e3893: f64 = (assign5220_e3886 * assign5220_e3892);
        let assign5220_e3894: f64 = (1.0 + assign5220_e3893);
        let (assign5220_e3910,) = {
            if (assign5220_e3894 > 0.001) {
                let assign5220_e3900: f64 = (p.p212 * locals.var_iwe);
                let assign5220_e3904: f64 = (locals.var_we / p.p213);
                let assign5220_e3905: f64 = (1.0 + assign5220_e3904);
                let assign5220_e3906: f64 = (assign5220_e3905).ln();
                let assign5220_e3907: f64 = (assign5220_e3900 * assign5220_e3906);
                let assign5220_e3908: f64 = (1.0 + assign5220_e3907);
                (assign5220_e3908,)
            } else {
                (0.001,)
            }
        };
        let assign5220_e3911: f64 = (p.p211 * assign5220_e3910);
        (assign5220_e3911,)
    } else {
        (locals.var_nsub0e,)
    }
};
        locals.var_nsub0e = assign5220_e3913;

        let (assign5230_e3946,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5230_e3919: f64 = (p.p215 * locals.var_iwe);
        let assign5230_e3923: f64 = (locals.var_we / p.p216);
        let assign5230_e3924: f64 = (1.0 + assign5230_e3923);
        let assign5230_e3925: f64 = (assign5230_e3924).ln();
        let assign5230_e3926: f64 = (assign5230_e3919 * assign5230_e3925);
        let assign5230_e3927: f64 = (1.0 + assign5230_e3926);
        let (assign5230_e3943,) = {
            if (assign5230_e3927 > 0.001) {
                let assign5230_e3933: f64 = (p.p215 * locals.var_iwe);
                let assign5230_e3937: f64 = (locals.var_we / p.p216);
                let assign5230_e3938: f64 = (1.0 + assign5230_e3937);
                let assign5230_e3939: f64 = (assign5230_e3938).ln();
                let assign5230_e3940: f64 = (assign5230_e3933 * assign5230_e3939);
                let assign5230_e3941: f64 = (1.0 + assign5230_e3940);
                (assign5230_e3941,)
            } else {
                (0.001,)
            }
        };
        let assign5230_e3944: f64 = (p.p214 * assign5230_e3943);
        (assign5230_e3944,)
    } else {
        (locals.var_npcke,)
    }
};
        locals.var_npcke = assign5230_e3946;

        let (assign5240_e3979,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5240_e3952: f64 = (p.p218 * locals.var_iwe);
        let assign5240_e3956: f64 = (locals.var_we / p.p216);
        let assign5240_e3957: f64 = (1.0 + assign5240_e3956);
        let assign5240_e3958: f64 = (assign5240_e3957).ln();
        let assign5240_e3959: f64 = (assign5240_e3952 * assign5240_e3958);
        let assign5240_e3960: f64 = (1.0 + assign5240_e3959);
        let (assign5240_e3976,) = {
            if (assign5240_e3960 > 0.001) {
                let assign5240_e3966: f64 = (p.p218 * locals.var_iwe);
                let assign5240_e3970: f64 = (locals.var_we / p.p216);
                let assign5240_e3971: f64 = (1.0 + assign5240_e3970);
                let assign5240_e3972: f64 = (assign5240_e3971).ln();
                let assign5240_e3973: f64 = (assign5240_e3966 * assign5240_e3972);
                let assign5240_e3974: f64 = (1.0 + assign5240_e3973);
                (assign5240_e3974,)
            } else {
                (0.001,)
            }
        };
        let assign5240_e3977: f64 = (p.p217 * assign5240_e3976);
        (assign5240_e3977,)
    } else {
        (locals.var_lpcke,)
    }
};
        locals.var_lpcke = assign5240_e3979;

        let assign5250_e3983: f64 = (2.0 * locals.var_lpcke);
        let assign5250_e3984: f64 = if locals.var_le > assign5250_e3983 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign5250_e3984;

        let (assign5260_e3990,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        (75000000000.0,)
    } else {
        (locals.var_aa,)
    }
};
        locals.var_aa = assign5260_e3990;

        let (assign5270_e4004,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5270_e3997: f64 = (0.5 * locals.var_npcke);
        let assign5270_e3998: f64 = (locals.var_nsub0e + assign5270_e3997);
        let assign5270_e3999: f64 = (assign5270_e3998).sqrt();
        let assign5270_e4001: f64 = (locals.var_nsub0e).sqrt();
        let assign5270_e4002: f64 = (assign5270_e3999 - assign5270_e4001);
        (assign5270_e4002,)
    } else {
        (locals.var_bb,)
    }
};
        locals.var_bb = assign5270_e4004;

        let (assign5280_e4029,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5280_e4009: f64 = (locals.var_nsub0e).sqrt();
        let assign5280_e4014: f64 = (2.0 * locals.var_lpcke);
        let assign5280_e4016: f64 = (assign5280_e4014 / locals.var_le);
        let assign5280_e4019: f64 = (locals.var_bb / locals.var_aa);
        let assign5280_e4020: f64 = (assign5280_e4019).exp();
        let assign5280_e4022: f64 = (assign5280_e4020 - 1.0);
        let assign5280_e4023: f64 = (assign5280_e4016 * assign5280_e4022);
        let assign5280_e4024: f64 = (1.0 + assign5280_e4023);
        let assign5280_e4025: f64 = (assign5280_e4024).ln();
        let assign5280_e4026: f64 = (locals.var_aa * assign5280_e4025);
        let assign5280_e4027: f64 = (assign5280_e4009 + assign5280_e4026);
        (assign5280_e4027,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5280_e4029;

        let (assign5290_e4037,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard37 != 0.0)) {
        let assign5290_e4035: f64 = (locals.var_nsub * locals.var_nsub);
        (assign5290_e4035,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5290_e4037;

        let assign5300_e4040: f64 = if locals.var_le >= locals.var_lpcke { 1.0 } else { 0.0 };
        locals.var_guard38 = assign5300_e4040;

        let (assign5310_e4055,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 != 0.0)) {
        let assign5310_e4050: f64 = (locals.var_npcke * locals.var_lpcke);
        let assign5310_e4052: f64 = (assign5310_e4050 / locals.var_le);
        let assign5310_e4053: f64 = (locals.var_nsub0e + assign5310_e4052);
        (assign5310_e4053,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5310_e4055;

        let (assign5320_e4073,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard37 == 0.0)) && (locals.var_guard38 == 0.0)) {
        let assign5320_e4068: f64 = (locals.var_le / locals.var_lpcke);
        let assign5320_e4069: f64 = (2.0 - assign5320_e4068);
        let assign5320_e4070: f64 = (locals.var_npcke * assign5320_e4069);
        let assign5320_e4071: f64 = (locals.var_nsub0e + assign5320_e4070);
        (assign5320_e4071,)
    } else {
        (locals.var_nsub,)
    }
};
        locals.var_nsub = assign5320_e4073;

        let (assign5330_e4087,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5330_e4079: f64 = (p.p219 * locals.var_ile);
        let assign5330_e4080: f64 = (1.0 - assign5330_e4079);
        let assign5330_e4083: f64 = (p.p220 * locals.var_ile2);
        let assign5330_e4084: f64 = (assign5330_e4080 - assign5330_e4083);
        let assign5330_e4085: f64 = (locals.var_nsub * assign5330_e4084);
        (assign5330_e4085,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign5330_e4087;

        let (assign5340_e4105,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5340_e4093: f64 = (locals.var_ile).powf(p.p223);
        let assign5340_e4094: f64 = (p.p222 * assign5340_e4093);
        let assign5340_e4095: f64 = (p.p221 + assign5340_e4094);
        let assign5340_e4098: f64 = (p.p224 * locals.var_iwe);
        let assign5340_e4099: f64 = (assign5340_e4095 + assign5340_e4098);
        let assign5340_e4102: f64 = (p.p225 * locals.var_iae);
        let assign5340_e4103: f64 = (assign5340_e4099 + assign5340_e4102);
        (assign5340_e4103,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign5340_e4105;

        let (assign5350_e4109,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p226,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign5350_e4109;

        let (assign5360_e4113,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p227,)
    } else {
        (locals.var_dvsbnud_p,)
    }
};
        locals.var_dvsbnud_p = assign5360_e4113;

        let (assign5370_e4131,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5370_e4119: f64 = (locals.var_ile).powf(p.p230);
        let assign5370_e4120: f64 = (p.p229 * assign5370_e4119);
        let assign5370_e4121: f64 = (p.p228 + assign5370_e4120);
        let assign5370_e4124: f64 = (p.p231 * locals.var_iwe);
        let assign5370_e4125: f64 = (assign5370_e4121 + assign5370_e4124);
        let assign5370_e4128: f64 = (p.p232 * locals.var_iae);
        let assign5370_e4129: f64 = (assign5370_e4125 + assign5370_e4128);
        (assign5370_e4129,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign5370_e4131;

        let (assign5380_e4150,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5380_e4138: f64 = (p.p234 * locals.var_ile);
        let assign5380_e4139: f64 = (1.0 + assign5380_e4138);
        let (assign5380_e4147,) = {
            if (1e-6 > assign5380_e4139) {
                (1e-6,)
            } else {
                let assign5380_e4145: f64 = (p.p234 * locals.var_ile);
                let assign5380_e4146: f64 = (1.0 + assign5380_e4145);
                (assign5380_e4146,)
            }
        };
        let assign5380_e4148: f64 = (p.p233 * assign5380_e4147);
        (assign5380_e4148,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign5380_e4150;

        let (assign5390_e4154,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p235,)
    } else {
        (locals.var_toxov_p,)
    }
};
        locals.var_toxov_p = assign5390_e4154;

        let (assign5400_e4158,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p236,)
    } else {
        (locals.var_toxovd_p,)
    }
};
        locals.var_toxovd_p = assign5400_e4158;

        let (assign5410_e4162,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p239,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign5410_e4162;

        let (assign5420_e4166,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p240,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign5420_e4166;

        let (assign5430_e4188,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5430_e4172: f64 = (locals.var_ile).powf(p.p243);
        let assign5430_e4173: f64 = (p.p242 * assign5430_e4172);
        let assign5430_e4174: f64 = (p.p241 + assign5430_e4173);
        let assign5430_e4178: f64 = (p.p244 * locals.var_iwe);
        let assign5430_e4179: f64 = (1.0 + assign5430_e4178);
        let assign5430_e4180: f64 = (assign5430_e4174 * assign5430_e4179);
        let assign5430_e4184: f64 = (p.p245 * locals.var_iae);
        let assign5430_e4185: f64 = (1.0 + assign5430_e4184);
        let assign5430_e4186: f64 = (assign5430_e4180 * assign5430_e4185);
        (assign5430_e4186,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign5430_e4188;

        let (assign5440_e4192,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p247,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign5440_e4192;

        let (assign5450_e4196,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p246,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign5450_e4196;

        let (assign5460_e4200,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p248,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign5460_e4200;

        let (assign5470_e4214,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5470_e4205: f64 = (locals.var_ile).powf(p.p250);
        let assign5470_e4206: f64 = (p.p249 * assign5470_e4205);
        let assign5470_e4210: f64 = (p.p251 * locals.var_iwe);
        let assign5470_e4211: f64 = (1.0 + assign5470_e4210);
        let assign5470_e4212: f64 = (assign5470_e4206 * assign5470_e4211);
        (assign5470_e4212,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign5470_e4214;

        let (assign5480_e4218,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p253,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign5480_e4218;

        let (assign5490_e4222,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p252,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign5490_e4222;

        let (assign5500_e4236,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5500_e4227: f64 = (locals.var_ile).powf(p.p255);
        let assign5500_e4228: f64 = (p.p254 * assign5500_e4227);
        let assign5500_e4232: f64 = (p.p256 * locals.var_iwe);
        let assign5500_e4233: f64 = (1.0 + assign5500_e4232);
        let assign5500_e4234: f64 = (assign5500_e4228 * assign5500_e4233);
        (assign5500_e4234,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign5500_e4236;

        let (assign5510_e4240,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p258,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign5510_e4240;

        let (assign5520_e4244,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p257,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign5520_e4244;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5530_e4254,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5530_e4250: f64 = (p.p261 * locals.var_iwe);
        let assign5530_e4251: f64 = (1.0 + assign5530_e4250);
        let assign5530_e4252: f64 = (p.p260 * assign5530_e4251);
        (assign5530_e4252,)
    } else {
        (locals.var_fbet1e,)
    }
};
        locals.var_fbet1e = assign5530_e4254;

        let (assign5540_e4273,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5540_e4260: f64 = (p.p263 * locals.var_iwe);
        let assign5540_e4261: f64 = (1.0 + assign5540_e4260);
        let (assign5540_e4270,) = {
            if (assign5540_e4261 > 0.001) {
                let assign5540_e4267: f64 = (p.p263 * locals.var_iwe);
                let assign5540_e4268: f64 = (1.0 + assign5540_e4267);
                (assign5540_e4268,)
            } else {
                (0.001,)
            }
        };
        let assign5540_e4271: f64 = (p.p262 * assign5540_e4270);
        (assign5540_e4271,)
    } else {
        (locals.var_lp1e,)
    }
};
        locals.var_lp1e = assign5540_e4273;

        let (assign5550_e4305,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5550_e4278: f64 = (locals.var_fbet1e * locals.var_lp1e);
        let assign5550_e4280: f64 = (assign5550_e4278 / locals.var_le);
        let assign5550_e4283: f64 = (-locals.var_le);
        let assign5550_e4285: f64 = (assign5550_e4283 / locals.var_lp1e);
        let assign5550_e4286: f64 = (assign5550_e4285).exp();
        let assign5550_e4287: f64 = (1.0 - assign5550_e4286);
        let assign5550_e4288: f64 = (assign5550_e4280 * assign5550_e4287);
        let assign5550_e4289: f64 = (1.0 + assign5550_e4288);
        let assign5550_e4292: f64 = (p.p264 * p.p265);
        let assign5550_e4294: f64 = (assign5550_e4292 / locals.var_le);
        let assign5550_e4297: f64 = (-locals.var_le);
        let assign5550_e4299: f64 = (assign5550_e4297 / p.p265);
        let assign5550_e4300: f64 = (assign5550_e4299).exp();
        let assign5550_e4301: f64 = (1.0 - assign5550_e4300);
        let assign5550_e4302: f64 = (assign5550_e4294 * assign5550_e4301);
        let assign5550_e4303: f64 = (assign5550_e4289 + assign5550_e4302);
        (assign5550_e4303,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5550_e4305;

        let (assign5560_e4314,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign5560_e4312,) = {
            if (locals.var_gpe > 1e-15) {
                (locals.var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5560_e4312,)
    } else {
        (locals.var_gpe,)
    }
};
        locals.var_gpe = assign5560_e4314;

        let (assign5570_e4333,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5570_e4319: f64 = (p.p266 * locals.var_iwe);
        let assign5570_e4320: f64 = (1.0 + assign5570_e4319);
        let assign5570_e4323: f64 = (p.p267 * locals.var_iwe);
        let assign5570_e4327: f64 = (locals.var_we / p.p268);
        let assign5570_e4328: f64 = (1.0 + assign5570_e4327);
        let assign5570_e4329: f64 = (assign5570_e4328).ln();
        let assign5570_e4330: f64 = (assign5570_e4323 * assign5570_e4329);
        let assign5570_e4331: f64 = (assign5570_e4320 + assign5570_e4330);
        (assign5570_e4331,)
    } else {
        (locals.var_gwe,)
    }
};
        locals.var_gwe = assign5570_e4333;

        let (assign5580_e4345,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5580_e4337: f64 = (p.p259 * locals.var_we);
        let assign5580_e4340: f64 = (locals.var_gpe * locals.var_le);
        let assign5580_e4341: f64 = (assign5580_e4337 / assign5580_e4340);
        let assign5580_e4343: f64 = (assign5580_e4341 * locals.var_gwe);
        (assign5580_e4343,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign5580_e4345;

        let (assign5590_e4361,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5590_e4350: f64 = (p.p270 * locals.var_ile);
        let assign5590_e4351: f64 = (p.p269 + assign5590_e4350);
        let assign5590_e4354: f64 = (p.p271 * locals.var_iwe);
        let assign5590_e4355: f64 = (assign5590_e4351 + assign5590_e4354);
        let assign5590_e4358: f64 = (p.p272 * locals.var_iae);
        let assign5590_e4359: f64 = (assign5590_e4355 + assign5590_e4358);
        (assign5590_e4359,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign5590_e4361;

        let (assign5600_e4371,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5600_e4367: f64 = (p.p274 * locals.var_iwe);
        let assign5600_e4368: f64 = (1.0 + assign5600_e4367);
        let assign5600_e4369: f64 = (p.p273 * assign5600_e4368);
        (assign5600_e4369,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign5600_e4371;

        let (assign5610_e4375,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p275,)
    } else {
        (locals.var_stmue_p,)
    }
};
        locals.var_stmue_p = assign5610_e4375;

        let (assign5620_e4379,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p276,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign5620_e4379;

        let (assign5630_e4383,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p277,)
    } else {
        (locals.var_stthemu_p,)
    }
};
        locals.var_stthemu_p = assign5630_e4383;

        let (assign5640_e4405,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5640_e4389: f64 = (locals.var_ile).powf(p.p280);
        let assign5640_e4390: f64 = (p.p279 * assign5640_e4389);
        let assign5640_e4391: f64 = (p.p278 + assign5640_e4390);
        let assign5640_e4395: f64 = (p.p281 * locals.var_iwe);
        let assign5640_e4396: f64 = (1.0 + assign5640_e4395);
        let assign5640_e4397: f64 = (assign5640_e4391 * assign5640_e4396);
        let assign5640_e4401: f64 = (p.p282 * locals.var_iae);
        let assign5640_e4402: f64 = (1.0 + assign5640_e4401);
        let assign5640_e4403: f64 = (assign5640_e4397 * assign5640_e4402);
        (assign5640_e4403,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign5640_e4405;

        let (assign5650_e4409,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p283,)
    } else {
        (locals.var_stcs_p,)
    }
};
        locals.var_stcs_p = assign5650_e4409;

        let (assign5660_e4413,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p284,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign5660_e4413;

        let (assign5670_e4417,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p285,)
    } else {
        (locals.var_stthecs_p,)
    }
};
        locals.var_stthecs_p = assign5670_e4417;

        let (assign5680_e4439,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5680_e4423: f64 = (p.p287 * locals.var_ile);
        let assign5680_e4424: f64 = (1.0 + assign5680_e4423);
        let assign5680_e4425: f64 = (p.p286 * assign5680_e4424);
        let assign5680_e4429: f64 = (p.p288 * locals.var_iwe);
        let assign5680_e4430: f64 = (1.0 + assign5680_e4429);
        let assign5680_e4431: f64 = (assign5680_e4425 * assign5680_e4430);
        let assign5680_e4435: f64 = (p.p289 * locals.var_iae);
        let assign5680_e4436: f64 = (1.0 + assign5680_e4435);
        let assign5680_e4437: f64 = (assign5680_e4431 * assign5680_e4436);
        (assign5680_e4437,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign5680_e4439;

        let (assign5690_e4443,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p290,)
    } else {
        (locals.var_stxcor_p,)
    }
};
        locals.var_stxcor_p = assign5690_e4443;

        let (assign5700_e4447,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p291,)
    } else {
        (locals.var_feta_p,)
    }
};
        locals.var_feta_p = assign5700_e4447;

        let (assign5710_e4459,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5710_e4451: f64 = (p.p292 * locals.var_iwe);
        let assign5710_e4455: f64 = (p.p293 * locals.var_iwe);
        let assign5710_e4456: f64 = (1.0 + assign5710_e4455);
        let assign5710_e4457: f64 = (assign5710_e4451 * assign5710_e4456);
        (assign5710_e4457,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign5710_e4459;

        let (assign5720_e4463,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p294,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign5720_e4463;

        let (assign5730_e4467,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p295,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign5730_e4467;

        let (assign5740_e4471,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p296,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign5740_e4471;

        let (assign5750_e4497,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5750_e4476: f64 = (p.p298 * locals.var_gwe);
        let assign5750_e4478: f64 = (assign5750_e4476 / locals.var_gpe);
        let assign5750_e4481: f64 = (locals.var_ile).powf(p.p299);
        let assign5750_e4482: f64 = (assign5750_e4478 * assign5750_e4481);
        let assign5750_e4483: f64 = (p.p297 + assign5750_e4482);
        let assign5750_e4487: f64 = (p.p300 * locals.var_iwe);
        let assign5750_e4488: f64 = (1.0 + assign5750_e4487);
        let assign5750_e4489: f64 = (assign5750_e4483 * assign5750_e4488);
        let assign5750_e4493: f64 = (p.p301 * locals.var_iae);
        let assign5750_e4494: f64 = (1.0 + assign5750_e4493);
        let assign5750_e4495: f64 = (assign5750_e4489 * assign5750_e4494);
        (assign5750_e4495,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign5750_e4497;

        let (assign5760_e4513,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5760_e4502: f64 = (p.p303 * locals.var_ile);
        let assign5760_e4503: f64 = (p.p302 + assign5760_e4502);
        let assign5760_e4506: f64 = (p.p304 * locals.var_iwe);
        let assign5760_e4507: f64 = (assign5760_e4503 + assign5760_e4506);
        let assign5760_e4510: f64 = (p.p305 * locals.var_iae);
        let assign5760_e4511: f64 = (assign5760_e4507 + assign5760_e4510);
        (assign5760_e4511,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign5760_e4513;

        let (assign5770_e4517,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p306,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign5770_e4517;

        let (assign5780_e4521,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p307,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign5780_e4521;

        let (assign5790_e4525,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p308,)
    } else {
        (locals.var_thesatt_p,)
    }
};
        locals.var_thesatt_p = assign5790_e4525;

        let (assign5800_e4535,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5800_e4531: f64 = (p.p310 * locals.var_ile);
        let assign5800_e4532: f64 = (1.0 + assign5800_e4531);
        let assign5800_e4533: f64 = (p.p309 / assign5800_e4532);
        (assign5800_e4533,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign5800_e4535;

        let (assign5810_e4549,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5810_e4540: f64 = (locals.var_ile).powf(p.p312);
        let assign5810_e4541: f64 = (p.p311 * assign5810_e4540);
        let assign5810_e4545: f64 = (p.p313 * locals.var_iwe);
        let assign5810_e4546: f64 = (1.0 + assign5810_e4545);
        let assign5810_e4547: f64 = (assign5810_e4541 * assign5810_e4546);
        (assign5810_e4547,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign5810_e4549;

        let (assign5820_e4555,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5820_e4553: f64 = (locals.var_ile).powf(p.p315);
        (assign5820_e4553,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign5820_e4555;

        let (assign5830_e4575,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5830_e4559: f64 = (p.p314 * locals.var_tmpx);
        let assign5830_e4563: f64 = (p.p317 * locals.var_iwe);
        let assign5830_e4564: f64 = (1.0 + assign5830_e4563);
        let assign5830_e4565: f64 = (assign5830_e4559 * assign5830_e4564);
        let assign5830_e4569: f64 = (p.p316 * locals.var_ile);
        let assign5830_e4571: f64 = (assign5830_e4569 * locals.var_tmpx);
        let assign5830_e4572: f64 = (1.0 + assign5830_e4571);
        let assign5830_e4573: f64 = (assign5830_e4565 / assign5830_e4572);
        (assign5830_e4573,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign5830_e4575;

        let (assign5840_e4581,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5840_e4579: f64 = (locals.var_ile).powf(p.p319);
        (assign5840_e4579,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign5840_e4581;

        let (assign5850_e4601,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5850_e4585: f64 = (p.p318 * locals.var_tmpx);
        let assign5850_e4589: f64 = (p.p321 * locals.var_iwe);
        let assign5850_e4590: f64 = (1.0 + assign5850_e4589);
        let assign5850_e4591: f64 = (assign5850_e4585 * assign5850_e4590);
        let assign5850_e4595: f64 = (p.p320 * locals.var_ile);
        let assign5850_e4597: f64 = (assign5850_e4595 * locals.var_tmpx);
        let assign5850_e4598: f64 = (1.0 + assign5850_e4597);
        let assign5850_e4599: f64 = (assign5850_e4591 / assign5850_e4598);
        (assign5850_e4599,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign5850_e4601;

        let (assign5860_e4605,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p322,)
    } else {
        (locals.var_vp_p,)
    }
};
        locals.var_vp_p = assign5860_e4605;

        let (assign5870_e4621,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5870_e4611: f64 = (p.p324 * locals.var_ile);
        let assign5870_e4612: f64 = (1.0 + assign5870_e4611);
        let assign5870_e4613: f64 = (p.p323 * assign5870_e4612);
        let assign5870_e4617: f64 = (p.p325 * locals.var_iwe);
        let assign5870_e4618: f64 = (1.0 + assign5870_e4617);
        let assign5870_e4619: f64 = (assign5870_e4613 * assign5870_e4618);
        (assign5870_e4619,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign5870_e4621;

        let (assign5880_e4625,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p326,)
    } else {
        (locals.var_a2_p,)
    }
};
        locals.var_a2_p = assign5880_e4625;

        let (assign5890_e4629,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p327,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign5890_e4629;

        let (assign5900_e4645,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5900_e4635: f64 = (p.p329 * locals.var_ile);
        let assign5900_e4636: f64 = (1.0 + assign5900_e4635);
        let assign5900_e4637: f64 = (p.p328 * assign5900_e4636);
        let assign5900_e4641: f64 = (p.p330 * locals.var_iwe);
        let assign5900_e4642: f64 = (1.0 + assign5900_e4641);
        let assign5900_e4643: f64 = (assign5900_e4637 * assign5900_e4642);
        (assign5900_e4643,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign5900_e4645;

        let (assign5910_e4661,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5910_e4651: f64 = (p.p332 * locals.var_ile);
        let assign5910_e4652: f64 = (1.0 + assign5910_e4651);
        let assign5910_e4653: f64 = (p.p331 * assign5910_e4652);
        let assign5910_e4657: f64 = (p.p333 * locals.var_iwe);
        let assign5910_e4658: f64 = (1.0 + assign5910_e4657);
        let assign5910_e4659: f64 = (assign5910_e4653 * assign5910_e4658);
        (assign5910_e4659,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign5910_e4661;

        let (assign5920_e4665,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p334,)
    } else {
        (locals.var_imaxii_p,)
    }
};
        locals.var_imaxii_p = assign5920_e4665;

        let (assign5930_e4669,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p335,)
    } else {
        (locals.var_gco_p,)
    }
};
        locals.var_gco_p = assign5930_e4669;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign5940_e4675,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5940_e4673: f64 = (p.p336 / locals.var_iae);
        (assign5940_e4673,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign5940_e4675;

        let (assign5950_e4685,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5950_e4679: f64 = (p.p337 * p.p237);
        let assign5950_e4682: f64 = (1e-6 * locals.var_iwe);
        let assign5950_e4683: f64 = (assign5950_e4679 / assign5950_e4682);
        (assign5950_e4683,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign5950_e4685;

        let (assign5960_e4695,) = {
    if (locals.var_guard36 != 0.0) {
        let assign5960_e4689: f64 = (p.p338 * p.p238);
        let assign5960_e4692: f64 = (1e-6 * locals.var_iwe);
        let assign5960_e4693: f64 = (assign5960_e4689 / assign5960_e4692);
        (assign5960_e4693,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign5960_e4695;

        let (assign5970_e4699,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p339,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign5970_e4699;

        let (assign5980_e4703,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p340,)
    } else {
        (locals.var_gc2_p,)
    }
};
        locals.var_gc2_p = assign5980_e4703;

        let (assign5990_e4707,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p341,)
    } else {
        (locals.var_gc3_p,)
    }
};
        locals.var_gc3_p = assign5990_e4707;

        let (assign6000_e4711,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p340,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6000_e4711;

        let assign6010_e4713: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6010_e4715: f64 = if assign6010_e4713 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard39 = assign6010_e4715;

        let (assign6020_e4721,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard39 != 0.0)) {
        (p.p342,)
    } else {
        (locals.var_gc2ov_p,)
    }
};
        locals.var_gc2ov_p = assign6020_e4721;

        let (assign6030_e4725,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p341,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6030_e4725;

        let assign6040_e4727: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6040_e4729: f64 = if assign6040_e4727 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign6040_e4729;

        let (assign6050_e4735,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard40 != 0.0)) {
        (p.p343,)
    } else {
        (locals.var_gc3ov_p,)
    }
};
        locals.var_gc3ov_p = assign6050_e4735;

        let (assign6060_e4739,) = {
    if (locals.var_guard36 != 0.0) {
        (locals.var_gc2ov_p,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6060_e4739;

        let assign6070_e4741: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6070_e4743: f64 = if assign6070_e4741 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign6070_e4743;

        let (assign6080_e4749,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard41 != 0.0)) {
        (p.p344,)
    } else {
        (locals.var_gc2ovd_p,)
    }
};
        locals.var_gc2ovd_p = assign6080_e4749;

        let (assign6090_e4753,) = {
    if (locals.var_guard36 != 0.0) {
        (locals.var_gc3ov_p,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6090_e4753;

        let assign6100_e4755: f64 = if param_given[345] { 1.0 } else { 0.0 };
        let assign6100_e4757: f64 = if assign6100_e4755 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign6100_e4757;

        let (assign6110_e4763,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard42 != 0.0)) {
        (p.p345,)
    } else {
        (locals.var_gc3ovd_p,)
    }
};
        locals.var_gc3ovd_p = assign6110_e4763;

        let (assign6120_e4767,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p346,)
    } else {
        (locals.var_chib_p,)
    }
};
        locals.var_chib_p = assign6120_e4767;

        let (assign6130_e4777,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6130_e4771: f64 = (p.p347 * p.p237);
        let assign6130_e4774: f64 = (1e-6 * locals.var_iwe);
        let assign6130_e4775: f64 = (assign6130_e4771 / assign6130_e4774);
        (assign6130_e4775,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign6130_e4777;

        let (assign6140_e4787,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6140_e4781: f64 = (p.p348 * p.p238);
        let assign6140_e4784: f64 = (1e-6 * locals.var_iwe);
        let assign6140_e4785: f64 = (assign6140_e4781 / assign6140_e4784);
        (assign6140_e4785,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign6140_e4787;

        let (assign6150_e4791,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p349,)
    } else {
        (locals.var_bgidl_p,)
    }
};
        locals.var_bgidl_p = assign6150_e4791;

        let (assign6160_e4795,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p350,)
    } else {
        (locals.var_bgidld_p,)
    }
};
        locals.var_bgidld_p = assign6160_e4795;

        let (assign6170_e4799,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p351,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign6170_e4799;

        let (assign6180_e4803,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p352,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign6180_e4803;

        let (assign6190_e4807,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p353,)
    } else {
        (locals.var_cgidl_p,)
    }
};
        locals.var_cgidl_p = assign6190_e4807;

        let (assign6200_e4811,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p354,)
    } else {
        (locals.var_cgidld_p,)
    }
};
        locals.var_cgidld_p = assign6200_e4811;

        let (assign6210_e4823,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6210_e4815: f64 = (8.8541878176e-12 * p.p210);
        let assign6210_e4817: f64 = (assign6210_e4815 * locals.var_wecv);
        let assign6210_e4819: f64 = (assign6210_e4817 * locals.var_lecv);
        let assign6210_e4821: f64 = (assign6210_e4819 / p.p209);
        (assign6210_e4821,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign6210_e4823;

        let (assign6220_e4835,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6220_e4827: f64 = (8.8541878176e-12 * p.p210);
        let assign6220_e4829: f64 = (assign6220_e4827 * locals.var_wecv);
        let assign6220_e4831: f64 = (assign6220_e4829 * p.p237);
        let assign6220_e4833: f64 = (assign6220_e4831 / p.p235);
        (assign6220_e4833,)
    } else {
        (locals.var_cgov_p,)
    }
};
        locals.var_cgov_p = assign6220_e4835;

        let (assign6230_e4847,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6230_e4839: f64 = (8.8541878176e-12 * p.p210);
        let assign6230_e4841: f64 = (assign6230_e4839 * locals.var_wecv);
        let assign6230_e4843: f64 = (assign6230_e4841 * p.p238);
        let assign6230_e4845: f64 = (assign6230_e4843 / p.p236);
        (assign6230_e4845,)
    } else {
        (locals.var_cgovd_p,)
    }
};
        locals.var_cgovd_p = assign6230_e4847;

        let (assign6240_e4865,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6240_e4853: f64 = (locals.var_ile).powf(p.p357);
        let assign6240_e4854: f64 = (p.p356 * assign6240_e4853);
        let assign6240_e4855: f64 = (p.p355 + assign6240_e4854);
        let assign6240_e4858: f64 = (p.p358 * locals.var_iwe);
        let assign6240_e4859: f64 = (assign6240_e4855 + assign6240_e4858);
        let assign6240_e4862: f64 = (p.p359 * locals.var_iae);
        let assign6240_e4863: f64 = (assign6240_e4859 + assign6240_e4862);
        (assign6240_e4863,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign6240_e4865;

        let (assign6250_e4881,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6250_e4870: f64 = (p.p361 * locals.var_ile);
        let assign6250_e4871: f64 = (p.p360 + assign6250_e4870);
        let assign6250_e4874: f64 = (p.p362 * locals.var_iwe);
        let assign6250_e4875: f64 = (assign6250_e4871 + assign6250_e4874);
        let assign6250_e4878: f64 = (p.p363 * locals.var_iae);
        let assign6250_e4879: f64 = (assign6250_e4875 + assign6250_e4878);
        (assign6250_e4879,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign6250_e4881;

        let (assign6260_e4885,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p297,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6260_e4885;

        let assign6270_e4887: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6270_e4889: f64 = if assign6270_e4887 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign6270_e4889;

        let (assign6280_e4895,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard43 != 0.0)) {
        (p.p364,)
    } else {
        (locals.var_thesataco_i,)
    }
};
        locals.var_thesataco_i = assign6280_e4895;

        let (assign6290_e4899,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p298,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6290_e4899;

        let assign6300_e4901: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6300_e4903: f64 = if assign6300_e4901 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard44 = assign6300_e4903;

        let (assign6310_e4909,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard44 != 0.0)) {
        (p.p365,)
    } else {
        (locals.var_thesatacl_i,)
    }
};
        locals.var_thesatacl_i = assign6310_e4909;

        let (assign6320_e4913,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p299,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6320_e4913;

        let assign6330_e4915: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6330_e4917: f64 = if assign6330_e4915 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard45 = assign6330_e4917;

        let (assign6340_e4923,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard45 != 0.0)) {
        (p.p366,)
    } else {
        (locals.var_thesataclexp_i,)
    }
};
        locals.var_thesataclexp_i = assign6340_e4923;

        let (assign6350_e4927,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p300,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6350_e4927;

        let assign6360_e4929: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6360_e4931: f64 = if assign6360_e4929 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard46 = assign6360_e4931;

        let (assign6370_e4937,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard46 != 0.0)) {
        (p.p367,)
    } else {
        (locals.var_thesatacw_i,)
    }
};
        locals.var_thesatacw_i = assign6370_e4937;

        let (assign6380_e4941,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p301,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6380_e4941;

        let assign6390_e4943: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6390_e4945: f64 = if assign6390_e4943 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard47 = assign6390_e4945;

        let (assign6400_e4951,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard47 != 0.0)) {
        (p.p368,)
    } else {
        (locals.var_thesataclw_i,)
    }
};
        locals.var_thesataclw_i = assign6400_e4951;

        let (assign6410_e4977,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6410_e4956: f64 = (locals.var_thesatacl_i * locals.var_gwe);
        let assign6410_e4958: f64 = (assign6410_e4956 / locals.var_gpe);
        let assign6410_e4961: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
        let assign6410_e4962: f64 = (assign6410_e4958 * assign6410_e4961);
        let assign6410_e4963: f64 = (locals.var_thesataco_i + assign6410_e4962);
        let assign6410_e4967: f64 = (locals.var_thesatacw_i * locals.var_iwe);
        let assign6410_e4968: f64 = (1.0 + assign6410_e4967);
        let assign6410_e4969: f64 = (assign6410_e4963 * assign6410_e4968);
        let assign6410_e4973: f64 = (locals.var_thesataclw_i * locals.var_iae);
        let assign6410_e4974: f64 = (1.0 + assign6410_e4973);
        let assign6410_e4975: f64 = (assign6410_e4969 * assign6410_e4974);
        (assign6410_e4975,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign6410_e4977;

        let (assign6420_e4981,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p309,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6420_e4981;

        let assign6430_e4983: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6430_e4985: f64 = if assign6430_e4983 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard48 = assign6430_e4985;

        let (assign6440_e4991,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard48 != 0.0)) {
        (p.p369,)
    } else {
        (locals.var_axaco_i,)
    }
};
        locals.var_axaco_i = assign6440_e4991;

        let (assign6450_e4995,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p310,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6450_e4995;

        let assign6460_e4997: f64 = if param_given[370] { 1.0 } else { 0.0 };
        let assign6460_e4999: f64 = if assign6460_e4997 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard49 = assign6460_e4999;

        let (assign6470_e5005,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard49 != 0.0)) {
        (p.p370,)
    } else {
        (locals.var_axacl_i,)
    }
};
        locals.var_axacl_i = assign6470_e5005;

        let (assign6480_e5015,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6480_e5011: f64 = (locals.var_axacl_i * locals.var_ile);
        let assign6480_e5012: f64 = (1.0 + assign6480_e5011);
        let assign6480_e5013: f64 = (locals.var_axaco_i / assign6480_e5012);
        (assign6480_e5013,)
    } else {
        (locals.var_axac_p,)
    }
};
        locals.var_axac_p = assign6480_e5015;

        let (assign6490_e5029,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6490_e5020: f64 = (locals.var_ile).powf(p.p372);
        let assign6490_e5021: f64 = (p.p371 * assign6490_e5020);
        let assign6490_e5025: f64 = (p.p373 * locals.var_iwe);
        let assign6490_e5026: f64 = (1.0 + assign6490_e5025);
        let assign6490_e5027: f64 = (assign6490_e5021 * assign6490_e5026);
        (assign6490_e5027,)
    } else {
        (locals.var_alpac_p,)
    }
};
        locals.var_alpac_p = assign6490_e5029;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign6500_e5035,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6500_e5033: f64 = (locals.var_ile).powf(p.p375);
        (assign6500_e5033,)
    } else {
        (locals.var_tmpx,)
    }
};
        locals.var_tmpx = assign6500_e5035;

        let (assign6510_e5055,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6510_e5039: f64 = (p.p374 * locals.var_tmpx);
        let assign6510_e5043: f64 = (p.p377 * locals.var_iwe);
        let assign6510_e5044: f64 = (1.0 + assign6510_e5043);
        let assign6510_e5045: f64 = (assign6510_e5039 * assign6510_e5044);
        let assign6510_e5049: f64 = (p.p376 * locals.var_ile);
        let assign6510_e5051: f64 = (assign6510_e5049 * locals.var_tmpx);
        let assign6510_e5052: f64 = (1.0 + assign6510_e5051);
        let assign6510_e5053: f64 = (assign6510_e5045 / assign6510_e5052);
        (assign6510_e5053,)
    } else {
        (locals.var_alp1ac_p,)
    }
};
        locals.var_alp1ac_p = assign6510_e5055;

        let (assign6520_e5059,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p378,)
    } else {
        (locals.var_fcgovacc_p,)
    }
};
        locals.var_fcgovacc_p = assign6520_e5059;

        let (assign6530_e5063,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p379,)
    } else {
        (locals.var_fcgovaccd_p,)
    }
};
        locals.var_fcgovaccd_p = assign6530_e5063;

        let (assign6540_e5067,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p380,)
    } else {
        (locals.var_cgovaccg_p,)
    }
};
        locals.var_cgovaccg_p = assign6540_e5067;

        let (assign6550_e5073,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6550_e5071: f64 = (p.p381 * locals.var_iilcv);
        (assign6550_e5071,)
    } else {
        (locals.var_cgbov_p,)
    }
};
        locals.var_cgbov_p = assign6550_e5073;

        let (assign6560_e5079,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6560_e5077: f64 = (p.p382 * locals.var_iiwecv);
        (assign6560_e5077,)
    } else {
        (locals.var_cinr_p,)
    }
};
        locals.var_cinr_p = assign6560_e5079;

        let (assign6570_e5085,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6570_e5083: f64 = (p.p383 * locals.var_iiwecv);
        (assign6570_e5083,)
    } else {
        (locals.var_cinrd_p,)
    }
};
        locals.var_cinrd_p = assign6570_e5085;

        let (assign6580_e5089,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p384,)
    } else {
        (locals.var_dvfbinr_p,)
    }
};
        locals.var_dvfbinr_p = assign6580_e5089;

        let (assign6590_e5093,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p385,)
    } else {
        (locals.var_fcinrdep_p,)
    }
};
        locals.var_fcinrdep_p = assign6590_e5093;

        let (assign6600_e5097,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p386,)
    } else {
        (locals.var_fcinracc_p,)
    }
};
        locals.var_fcinracc_p = assign6600_e5097;

        let (assign6610_e5101,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p387,)
    } else {
        (locals.var_axinr_p,)
    }
};
        locals.var_axinr_p = assign6610_e5101;

        let (assign6640_e5123,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6640_e5118: f64 = (2.0 * p.p396);
        let assign6640_e5120: f64 = (assign6640_e5118 / locals.var_le);
        let assign6640_e5121: f64 = (1.0 - assign6640_e5120);
        (assign6640_e5121,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign6640_e5123;

        let (assign6670_e5144,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p390,)
    } else {
        (locals.var_fnt_p,)
    }
};
        locals.var_fnt_p = assign6670_e5144;

        let (assign6680_e5156,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6680_e5148: f64 = (p.p391 * locals.var_betn_p);
        let assign6680_e5150: f64 = (assign6680_e5148 * locals.var_betn_p);
        let assign6680_e5152: f64 = (assign6680_e5150 * locals.var_iwe);
        let assign6680_e5154: f64 = (assign6680_e5152 * locals.var_iwe);
        (assign6680_e5154,)
    } else {
        (locals.var_fntexc_p,)
    }
};
        locals.var_fntexc_p = assign6680_e5156;

        let (assign6730_e5194,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6730_e5188: f64 = (2.0 * p.p398);
        let assign6730_e5191: f64 = (p.p399 * locals.var_we);
        let assign6730_e5192: f64 = (assign6730_e5188 + assign6730_e5191);
        (assign6730_e5192,)
    } else {
        (locals.var_we_edge,)
    }
};
        locals.var_we_edge = assign6730_e5194;

        let (assign6760_e5210,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p400,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign6760_e5210;

        let (assign6770_e5226,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6770_e5215: f64 = (p.p402 * locals.var_ile);
        let assign6770_e5216: f64 = (p.p401 + assign6770_e5215);
        let assign6770_e5219: f64 = (p.p403 * locals.var_iwe);
        let assign6770_e5220: f64 = (assign6770_e5216 + assign6770_e5219);
        let assign6770_e5223: f64 = (p.p404 * locals.var_iae);
        let assign6770_e5224: f64 = (assign6770_e5220 + assign6770_e5223);
        (assign6770_e5224,)
    } else {
        (locals.var_stvfbedge_p,)
    }
};
        locals.var_stvfbedge_p = assign6770_e5226;

        let (assign6780_e5244,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6780_e5232: f64 = (locals.var_ile).powf(p.p407);
        let assign6780_e5233: f64 = (p.p406 * assign6780_e5232);
        let assign6780_e5234: f64 = (p.p405 + assign6780_e5233);
        let assign6780_e5237: f64 = (p.p408 * locals.var_iwe);
        let assign6780_e5238: f64 = (assign6780_e5234 + assign6780_e5237);
        let assign6780_e5241: f64 = (p.p409 * locals.var_iae);
        let assign6780_e5242: f64 = (assign6780_e5238 + assign6780_e5241);
        (assign6780_e5242,)
    } else {
        (locals.var_dphibedge_p,)
    }
};
        locals.var_dphibedge_p = assign6780_e5244;

        let (assign6790_e5268,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6790_e5251: f64 = (locals.var_ile).powf(p.p412);
        let assign6790_e5252: f64 = (p.p411 * assign6790_e5251);
        let assign6790_e5253: f64 = (1.0 + assign6790_e5252);
        let assign6790_e5254: f64 = (p.p410 * assign6790_e5253);
        let assign6790_e5258: f64 = (p.p413 * locals.var_iwe);
        let assign6790_e5259: f64 = (1.0 + assign6790_e5258);
        let assign6790_e5260: f64 = (assign6790_e5254 * assign6790_e5259);
        let assign6790_e5264: f64 = (p.p414 * locals.var_iae);
        let assign6790_e5265: f64 = (1.0 + assign6790_e5264);
        let assign6790_e5266: f64 = (assign6790_e5260 * assign6790_e5265);
        (assign6790_e5266,)
    } else {
        (locals.var_neffedge_p,)
    }
};
        locals.var_neffedge_p = assign6790_e5268;

        let (assign6800_e5278,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6800_e5274: f64 = (locals.var_ile).powf(p.p417);
        let assign6800_e5275: f64 = (p.p416 * assign6800_e5274);
        let assign6800_e5276: f64 = (p.p415 + assign6800_e5275);
        (assign6800_e5276,)
    } else {
        (locals.var_ctedge_p,)
    }
};
        locals.var_ctedge_p = assign6800_e5278;

        let (assign6810_e5296,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6810_e5283: f64 = (p.p418 * p.p419);
        let assign6810_e5285: f64 = (assign6810_e5283 / locals.var_le);
        let assign6810_e5288: f64 = (-locals.var_le);
        let assign6810_e5290: f64 = (assign6810_e5288 / p.p419);
        let assign6810_e5291: f64 = (assign6810_e5290).exp();
        let assign6810_e5292: f64 = (1.0 - assign6810_e5291);
        let assign6810_e5293: f64 = (assign6810_e5285 * assign6810_e5292);
        let assign6810_e5294: f64 = (1.0 + assign6810_e5293);
        (assign6810_e5294,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign6810_e5296;

        let (assign6820_e5305,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign6820_e5303,) = {
            if (locals.var_gpe_edge > 1e-15) {
                (locals.var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign6820_e5303,)
    } else {
        (locals.var_gpe_edge,)
    }
};
        locals.var_gpe_edge = assign6820_e5305;

        let (assign6830_e5321,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6830_e5309: f64 = (p.p259 * locals.var_we_edge);
        let assign6830_e5312: f64 = (locals.var_gpe_edge * locals.var_le);
        let assign6830_e5313: f64 = (assign6830_e5309 / assign6830_e5312);
        let assign6830_e5317: f64 = (p.p420 * locals.var_iwe);
        let assign6830_e5318: f64 = (1.0 + assign6830_e5317);
        let assign6830_e5319: f64 = (assign6830_e5313 * assign6830_e5318);
        (assign6830_e5319,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign6830_e5321;

        let (assign6840_e5337,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6840_e5326: f64 = (p.p422 * locals.var_ile);
        let assign6840_e5327: f64 = (p.p421 + assign6840_e5326);
        let assign6840_e5330: f64 = (p.p423 * locals.var_iwe);
        let assign6840_e5331: f64 = (assign6840_e5327 + assign6840_e5330);
        let assign6840_e5334: f64 = (p.p424 * locals.var_iae);
        let assign6840_e5335: f64 = (assign6840_e5331 + assign6840_e5334);
        (assign6840_e5335,)
    } else {
        (locals.var_stbetedge_p,)
    }
};
        locals.var_stbetedge_p = assign6840_e5337;

        let (assign6850_e5351,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6850_e5342: f64 = (locals.var_ile).powf(p.p426);
        let assign6850_e5343: f64 = (p.p425 * assign6850_e5342);
        let assign6850_e5347: f64 = (p.p427 * locals.var_iwe);
        let assign6850_e5348: f64 = (1.0 + assign6850_e5347);
        let assign6850_e5349: f64 = (assign6850_e5343 * assign6850_e5348);
        (assign6850_e5349,)
    } else {
        (locals.var_psceedge_p,)
    }
};
        locals.var_psceedge_p = assign6850_e5351;

        let (assign6860_e5355,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p428,)
    } else {
        (locals.var_pscebedge_p,)
    }
};
        locals.var_pscebedge_p = assign6860_e5355;

        let (assign6870_e5359,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p429,)
    } else {
        (locals.var_pscededge_p,)
    }
};
        locals.var_pscededge_p = assign6870_e5359;

        let (assign6880_e5373,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6880_e5364: f64 = (locals.var_ile).powf(p.p431);
        let assign6880_e5365: f64 = (p.p430 * assign6880_e5364);
        let assign6880_e5369: f64 = (p.p432 * locals.var_iwe);
        let assign6880_e5370: f64 = (1.0 + assign6880_e5369);
        let assign6880_e5371: f64 = (assign6880_e5365 * assign6880_e5370);
        (assign6880_e5371,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign6880_e5373;

        let (assign6890_e5377,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p434,)
    } else {
        (locals.var_cfdedge_p,)
    }
};
        locals.var_cfdedge_p = assign6890_e5377;

        let (assign6900_e5381,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p433,)
    } else {
        (locals.var_cfbedge_p,)
    }
};
        locals.var_cfbedge_p = assign6900_e5381;

        let (assign6960_e5423,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6960_e5412: f64 = (p.p832 * locals.var_ile);
        let assign6960_e5413: f64 = (p.p831 + assign6960_e5412);
        let assign6960_e5416: f64 = (p.p833 * locals.var_iwe);
        let assign6960_e5417: f64 = (assign6960_e5413 + assign6960_e5416);
        let assign6960_e5420: f64 = (p.p834 * locals.var_iae);
        let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
        (assign6960_e5421,)
    } else {
        (locals.var_kvthowe,)
    }
};
        locals.var_kvthowe = assign6960_e5423;

        let (assign6970_e5439,) = {
    if (locals.var_guard36 != 0.0) {
        let assign6970_e5428: f64 = (p.p836 * locals.var_ile);
        let assign6970_e5429: f64 = (p.p835 + assign6970_e5428);
        let assign6970_e5432: f64 = (p.p837 * locals.var_iwe);
        let assign6970_e5433: f64 = (assign6970_e5429 + assign6970_e5432);
        let assign6970_e5436: f64 = (p.p838 * locals.var_iae);
        let assign6970_e5437: f64 = (assign6970_e5433 + assign6970_e5436);
        (assign6970_e5437,)
    } else {
        (locals.var_kuowe,)
    }
};
        locals.var_kuowe = assign6970_e5439;

        let (assign6990_e5476,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign6990_e5474,) = {
            if (p.p445 > 0.0) {
                (p.p445,)
            } else {
                (0.0,)
            }
        };
        (assign6990_e5474,)
    } else {
        (locals.var_rsh_i,)
    }
};
        locals.var_rsh_i = assign6990_e5476;

        let (assign7000_e5485,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign7000_e5483,) = {
            if (p.p446 > 0.0) {
                (p.p446,)
            } else {
                (0.0,)
            }
        };
        (assign7000_e5483,)
    } else {
        (locals.var_rshd_i,)
    }
};
        locals.var_rshd_i = assign7000_e5485;

        let assign7010_e5488: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign7010_e5488;

        let (assign7020_e5494,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard50 != 0.0)) {
        (locals.var_rsh_i,)
    } else {
        (locals.var_rshd_i,)
    }
};
        locals.var_rshd_i = assign7020_e5494;

        let (assign7030_e5502,) = {
    if (locals.var_guard36 != 0.0) {
        let assign7030_e5498: f64 = (locals.var_nf_i * p.p12);
        let assign7030_e5500: f64 = (assign7030_e5498 * locals.var_rsh_i);
        (assign7030_e5500,)
    } else {
        (locals.var_rse_p,)
    }
};
        locals.var_rse_p = assign7030_e5502;

        let (assign7040_e5510,) = {
    if (locals.var_guard36 != 0.0) {
        let assign7040_e5506: f64 = (locals.var_nf_i * p.p13);
        let assign7040_e5508: f64 = (assign7040_e5506 * locals.var_rshd_i);
        (assign7040_e5508,)
    } else {
        (locals.var_rde_p,)
    }
};
        locals.var_rde_p = assign7040_e5510;

        let (assign7090_e5546,) = {
    if (locals.var_guard36 != 0.0) {
        let assign7090_e5540: f64 = (p.p454 / locals.var_ile);
        let assign7090_e5541: f64 = (1.0 + assign7090_e5540);
        let assign7090_e5543: f64 = (assign7090_e5541 / locals.var_iwe);
        let assign7090_e5544: f64 = (p.p453 + assign7090_e5543);
        (assign7090_e5544,)
    } else {
        (locals.var_deltarth,)
    }
};
        locals.var_deltarth = assign7090_e5546;

        let (assign7100_e5555,) = {
    if (locals.var_guard36 != 0.0) {
        let (assign7100_e5553,) = {
            if (locals.var_deltarth > 1e-6) {
                (locals.var_deltarth,)
            } else {
                (1e-6,)
            }
        };
        (assign7100_e5553,)
    } else {
        (locals.var_deltarth,)
    }
};
        locals.var_deltarth = assign7100_e5555;

        let (assign7110_e5563,) = {
    if (locals.var_guard36 != 0.0) {
        let assign7110_e5560: f64 = (p.p452 / locals.var_deltarth);
        let assign7110_e5561: f64 = (p.p451 + assign7110_e5560);
        (assign7110_e5561,)
    } else {
        (locals.var_rth_p,)
    }
};
        locals.var_rth_p = assign7110_e5563;

        let assign7140_e5602: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        locals.var_guard51 = assign7140_e5602;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7150_e5620,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard51 != 0.0)) {
        let assign7150_e5609: f64 = (p.p461 * locals.var_ile);
        let assign7150_e5610: f64 = (p.p460 + assign7150_e5609);
        let assign7150_e5613: f64 = (p.p462 * locals.var_iwe);
        let assign7150_e5614: f64 = (assign7150_e5610 + assign7150_e5613);
        let assign7150_e5617: f64 = (p.p463 * locals.var_iae);
        let assign7150_e5618: f64 = (assign7150_e5614 + assign7150_e5617);
        (assign7150_e5618,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign7150_e5620;

        let assign7160_e5639: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        locals.var_guard52 = assign7160_e5639;

        let (assign7170_e5657,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard52 != 0.0)) {
        let assign7170_e5646: f64 = (p.p465 * locals.var_ile);
        let assign7170_e5647: f64 = (p.p464 + assign7170_e5646);
        let assign7170_e5650: f64 = (p.p466 * locals.var_iwe);
        let assign7170_e5651: f64 = (assign7170_e5647 + assign7170_e5650);
        let assign7170_e5654: f64 = (p.p467 * locals.var_iae);
        let assign7170_e5655: f64 = (assign7170_e5651 + assign7170_e5654);
        (assign7170_e5655,)
    } else {
        (locals.var_stvfb_p,)
    }
};
        locals.var_stvfb_p = assign7170_e5657;

        let assign7180_e5676: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        locals.var_guard53 = assign7180_e5676;

        let (assign7190_e5694,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard53 != 0.0)) {
        let assign7190_e5683: f64 = (p.p469 * locals.var_ile);
        let assign7190_e5684: f64 = (p.p468 + assign7190_e5683);
        let assign7190_e5687: f64 = (p.p470 * locals.var_iwe);
        let assign7190_e5688: f64 = (assign7190_e5684 + assign7190_e5687);
        let assign7190_e5691: f64 = (p.p471 * locals.var_iae);
        let assign7190_e5692: f64 = (assign7190_e5688 + assign7190_e5691);
        (assign7190_e5692,)
    } else {
        (locals.var_neff_p,)
    }
};
        locals.var_neff_p = assign7190_e5694;

        let assign7200_e5713: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        locals.var_guard54 = assign7200_e5713;

        let (assign7210_e5731,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard54 != 0.0)) {
        let assign7210_e5720: f64 = (p.p473 * locals.var_ile);
        let assign7210_e5721: f64 = (p.p472 + assign7210_e5720);
        let assign7210_e5724: f64 = (p.p474 * locals.var_iwe);
        let assign7210_e5725: f64 = (assign7210_e5721 + assign7210_e5724);
        let assign7210_e5728: f64 = (p.p475 * locals.var_iae);
        let assign7210_e5729: f64 = (assign7210_e5725 + assign7210_e5728);
        (assign7210_e5729,)
    } else {
        (locals.var_gfacnud_p,)
    }
};
        locals.var_gfacnud_p = assign7210_e5731;

        let assign7220_e5750: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        locals.var_guard55 = assign7220_e5750;

        let (assign7230_e5768,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard55 != 0.0)) {
        let assign7230_e5757: f64 = (p.p477 * locals.var_ile);
        let assign7230_e5758: f64 = (p.p476 + assign7230_e5757);
        let assign7230_e5761: f64 = (p.p478 * locals.var_iwe);
        let assign7230_e5762: f64 = (assign7230_e5758 + assign7230_e5761);
        let assign7230_e5765: f64 = (p.p479 * locals.var_iae);
        let assign7230_e5766: f64 = (assign7230_e5762 + assign7230_e5765);
        (assign7230_e5766,)
    } else {
        (locals.var_vsbnud_p,)
    }
};
        locals.var_vsbnud_p = assign7230_e5768;

        let assign7240_e5787: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        locals.var_guard56 = assign7240_e5787;

        let (assign7250_e5805,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard56 != 0.0)) {
        let assign7250_e5794: f64 = (p.p481 * locals.var_ile);
        let assign7250_e5795: f64 = (p.p480 + assign7250_e5794);
        let assign7250_e5798: f64 = (p.p482 * locals.var_iwe);
        let assign7250_e5799: f64 = (assign7250_e5795 + assign7250_e5798);
        let assign7250_e5802: f64 = (p.p483 * locals.var_iae);
        let assign7250_e5803: f64 = (assign7250_e5799 + assign7250_e5802);
        (assign7250_e5803,)
    } else {
        (locals.var_dphib_p,)
    }
};
        locals.var_dphib_p = assign7250_e5805;

        let assign7260_e5824: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        locals.var_guard57 = assign7260_e5824;

        let (assign7270_e5842,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard57 != 0.0)) {
        let assign7270_e5831: f64 = (p.p485 * locals.var_ile);
        let assign7270_e5832: f64 = (p.p484 + assign7270_e5831);
        let assign7270_e5835: f64 = (p.p486 * locals.var_iwe);
        let assign7270_e5836: f64 = (assign7270_e5832 + assign7270_e5835);
        let assign7270_e5839: f64 = (p.p487 * locals.var_iae);
        let assign7270_e5840: f64 = (assign7270_e5836 + assign7270_e5839);
        (assign7270_e5840,)
    } else {
        (locals.var_np_p,)
    }
};
        locals.var_np_p = assign7270_e5842;

        let assign7280_e5861: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        locals.var_guard58 = assign7280_e5861;

        let (assign7290_e5879,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard58 != 0.0)) {
        let assign7290_e5868: f64 = (p.p489 * locals.var_ile);
        let assign7290_e5869: f64 = (p.p488 + assign7290_e5868);
        let assign7290_e5872: f64 = (p.p490 * locals.var_iwe);
        let assign7290_e5873: f64 = (assign7290_e5869 + assign7290_e5872);
        let assign7290_e5876: f64 = (p.p491 * locals.var_iae);
        let assign7290_e5877: f64 = (assign7290_e5873 + assign7290_e5876);
        (assign7290_e5877,)
    } else {
        (locals.var_nov_p,)
    }
};
        locals.var_nov_p = assign7290_e5879;

        let assign7300_e5898: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        locals.var_guard59 = assign7300_e5898;

        let (assign7310_e5916,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard59 != 0.0)) {
        let assign7310_e5905: f64 = (p.p493 * locals.var_ile);
        let assign7310_e5906: f64 = (p.p492 + assign7310_e5905);
        let assign7310_e5909: f64 = (p.p494 * locals.var_iwe);
        let assign7310_e5910: f64 = (assign7310_e5906 + assign7310_e5909);
        let assign7310_e5913: f64 = (p.p495 * locals.var_iae);
        let assign7310_e5914: f64 = (assign7310_e5910 + assign7310_e5913);
        (assign7310_e5914,)
    } else {
        (locals.var_novd_p,)
    }
};
        locals.var_novd_p = assign7310_e5916;

        let assign7320_e5935: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        locals.var_guard60 = assign7320_e5935;

        let (assign7330_e5953,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard60 != 0.0)) {
        let assign7330_e5942: f64 = (p.p497 * locals.var_ile);
        let assign7330_e5943: f64 = (p.p496 + assign7330_e5942);
        let assign7330_e5946: f64 = (p.p498 * locals.var_iwe);
        let assign7330_e5947: f64 = (assign7330_e5943 + assign7330_e5946);
        let assign7330_e5950: f64 = (p.p499 * locals.var_iae);
        let assign7330_e5951: f64 = (assign7330_e5947 + assign7330_e5950);
        (assign7330_e5951,)
    } else {
        (locals.var_ct_p,)
    }
};
        locals.var_ct_p = assign7330_e5953;

        let assign7340_e5972: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        locals.var_guard61 = assign7340_e5972;

        let (assign7350_e5990,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard61 != 0.0)) {
        let assign7350_e5979: f64 = (p.p505 * locals.var_ile);
        let assign7350_e5980: f64 = (p.p504 + assign7350_e5979);
        let assign7350_e5983: f64 = (p.p506 * locals.var_iwe);
        let assign7350_e5984: f64 = (assign7350_e5980 + assign7350_e5983);
        let assign7350_e5987: f64 = (p.p507 * locals.var_iae);
        let assign7350_e5988: f64 = (assign7350_e5984 + assign7350_e5987);
        (assign7350_e5988,)
    } else {
        (locals.var_ctg_p,)
    }
};
        locals.var_ctg_p = assign7350_e5990;

        let assign7360_e6009: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        locals.var_guard62 = assign7360_e6009;

        let (assign7370_e6027,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard62 != 0.0)) {
        let assign7370_e6016: f64 = (p.p501 * locals.var_ile);
        let assign7370_e6017: f64 = (p.p500 + assign7370_e6016);
        let assign7370_e6020: f64 = (p.p502 * locals.var_iwe);
        let assign7370_e6021: f64 = (assign7370_e6017 + assign7370_e6020);
        let assign7370_e6024: f64 = (p.p503 * locals.var_iae);
        let assign7370_e6025: f64 = (assign7370_e6021 + assign7370_e6024);
        (assign7370_e6025,)
    } else {
        (locals.var_ctb_p,)
    }
};
        locals.var_ctb_p = assign7370_e6027;

        let assign7380_e6046: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        locals.var_guard63 = assign7380_e6046;

        let (assign7390_e6064,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard63 != 0.0)) {
        let assign7390_e6053: f64 = (p.p509 * locals.var_ile);
        let assign7390_e6054: f64 = (p.p508 + assign7390_e6053);
        let assign7390_e6057: f64 = (p.p510 * locals.var_iwe);
        let assign7390_e6058: f64 = (assign7390_e6054 + assign7390_e6057);
        let assign7390_e6061: f64 = (p.p511 * locals.var_iae);
        let assign7390_e6062: f64 = (assign7390_e6058 + assign7390_e6061);
        (assign7390_e6062,)
    } else {
        (locals.var_stct_p,)
    }
};
        locals.var_stct_p = assign7390_e6064;

        let assign7400_e6083: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        locals.var_guard64 = assign7400_e6083;

        let (assign7410_e6103,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard64 != 0.0)) {
        let assign7410_e6091: f64 = (p.p513 * locals.var_ile);
        let assign7410_e6092: f64 = (p.p512 + assign7410_e6091);
        let assign7410_e6095: f64 = (p.p514 * locals.var_iwe);
        let assign7410_e6096: f64 = (assign7410_e6092 + assign7410_e6095);
        let assign7410_e6099: f64 = (p.p515 * locals.var_iae);
        let assign7410_e6100: f64 = (assign7410_e6096 + assign7410_e6099);
        let assign7410_e6101: f64 = (locals.var_ile2 * assign7410_e6100);
        (assign7410_e6101,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign7410_e6103;

        let assign7420_e6122: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        locals.var_guard65 = assign7420_e6122;

        let (assign7430_e6140,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard65 != 0.0)) {
        let assign7430_e6129: f64 = (p.p521 * locals.var_ile);
        let assign7430_e6130: f64 = (p.p520 + assign7430_e6129);
        let assign7430_e6133: f64 = (p.p522 * locals.var_iwe);
        let assign7430_e6134: f64 = (assign7430_e6130 + assign7430_e6133);
        let assign7430_e6137: f64 = (p.p523 * locals.var_iae);
        let assign7430_e6138: f64 = (assign7430_e6134 + assign7430_e6137);
        (assign7430_e6138,)
    } else {
        (locals.var_cfd_p,)
    }
};
        locals.var_cfd_p = assign7430_e6140;

        let assign7440_e6159: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        locals.var_guard66 = assign7440_e6159;

        let (assign7450_e6177,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard66 != 0.0)) {
        let assign7450_e6166: f64 = (p.p517 * locals.var_ile);
        let assign7450_e6167: f64 = (p.p516 + assign7450_e6166);
        let assign7450_e6170: f64 = (p.p518 * locals.var_iwe);
        let assign7450_e6171: f64 = (assign7450_e6167 + assign7450_e6170);
        let assign7450_e6174: f64 = (p.p519 * locals.var_iae);
        let assign7450_e6175: f64 = (assign7450_e6171 + assign7450_e6174);
        (assign7450_e6175,)
    } else {
        (locals.var_cfb_p,)
    }
};
        locals.var_cfb_p = assign7450_e6177;

        let assign7460_e6196: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        locals.var_guard67 = assign7460_e6196;

        let (assign7470_e6216,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard67 != 0.0)) {
        let assign7470_e6204: f64 = (p.p525 * locals.var_ile);
        let assign7470_e6205: f64 = (p.p524 + assign7470_e6204);
        let assign7470_e6208: f64 = (p.p526 * locals.var_iwe);
        let assign7470_e6209: f64 = (assign7470_e6205 + assign7470_e6208);
        let assign7470_e6212: f64 = (p.p527 * locals.var_iae);
        let assign7470_e6213: f64 = (assign7470_e6209 + assign7470_e6212);
        let assign7470_e6214: f64 = (locals.var_ile2 * assign7470_e6213);
        (assign7470_e6214,)
    } else {
        (locals.var_psce_p,)
    }
};
        locals.var_psce_p = assign7470_e6216;

        let assign7480_e6235: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        locals.var_guard68 = assign7480_e6235;

        let (assign7490_e6253,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard68 != 0.0)) {
        let assign7490_e6242: f64 = (p.p533 * locals.var_ile);
        let assign7490_e6243: f64 = (p.p532 + assign7490_e6242);
        let assign7490_e6246: f64 = (p.p534 * locals.var_iwe);
        let assign7490_e6247: f64 = (assign7490_e6243 + assign7490_e6246);
        let assign7490_e6250: f64 = (p.p535 * locals.var_iae);
        let assign7490_e6251: f64 = (assign7490_e6247 + assign7490_e6250);
        (assign7490_e6251,)
    } else {
        (locals.var_psced_p,)
    }
};
        locals.var_psced_p = assign7490_e6253;

        let assign7500_e6272: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        locals.var_guard69 = assign7500_e6272;

        let (assign7510_e6290,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard69 != 0.0)) {
        let assign7510_e6279: f64 = (p.p529 * locals.var_ile);
        let assign7510_e6280: f64 = (p.p528 + assign7510_e6279);
        let assign7510_e6283: f64 = (p.p530 * locals.var_iwe);
        let assign7510_e6284: f64 = (assign7510_e6280 + assign7510_e6283);
        let assign7510_e6287: f64 = (p.p531 * locals.var_iae);
        let assign7510_e6288: f64 = (assign7510_e6284 + assign7510_e6287);
        (assign7510_e6288,)
    } else {
        (locals.var_psceb_p,)
    }
};
        locals.var_psceb_p = assign7510_e6290;

        let assign7520_e6309: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        locals.var_guard70 = assign7520_e6309;

        let (assign7530_e6331,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard70 != 0.0)) {
        let assign7530_e6315: f64 = (locals.var_we / locals.var_le);
        let assign7530_e6319: f64 = (p.p537 * locals.var_ile);
        let assign7530_e6320: f64 = (p.p536 + assign7530_e6319);
        let assign7530_e6323: f64 = (p.p538 * locals.var_iwe);
        let assign7530_e6324: f64 = (assign7530_e6320 + assign7530_e6323);
        let assign7530_e6327: f64 = (p.p539 * locals.var_iae);
        let assign7530_e6328: f64 = (assign7530_e6324 + assign7530_e6327);
        let assign7530_e6329: f64 = (assign7530_e6315 * assign7530_e6328);
        (assign7530_e6329,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign7530_e6331;

        let assign7540_e6350: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        locals.var_guard71 = assign7540_e6350;

        let (assign7550_e6368,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard71 != 0.0)) {
        let assign7550_e6357: f64 = (p.p541 * locals.var_ile);
        let assign7550_e6358: f64 = (p.p540 + assign7550_e6357);
        let assign7550_e6361: f64 = (p.p542 * locals.var_iwe);
        let assign7550_e6362: f64 = (assign7550_e6358 + assign7550_e6361);
        let assign7550_e6365: f64 = (p.p543 * locals.var_iae);
        let assign7550_e6366: f64 = (assign7550_e6362 + assign7550_e6365);
        (assign7550_e6366,)
    } else {
        (locals.var_stbet_p,)
    }
};
        locals.var_stbet_p = assign7550_e6368;

        let assign7560_e6387: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        locals.var_guard72 = assign7560_e6387;

        let (assign7570_e6405,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard72 != 0.0)) {
        let assign7570_e6394: f64 = (p.p545 * locals.var_ile);
        let assign7570_e6395: f64 = (p.p544 + assign7570_e6394);
        let assign7570_e6398: f64 = (p.p546 * locals.var_iwe);
        let assign7570_e6399: f64 = (assign7570_e6395 + assign7570_e6398);
        let assign7570_e6402: f64 = (p.p547 * locals.var_iae);
        let assign7570_e6403: f64 = (assign7570_e6399 + assign7570_e6402);
        (assign7570_e6403,)
    } else {
        (locals.var_mue_p,)
    }
};
        locals.var_mue_p = assign7570_e6405;

        let assign7580_e6424: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        locals.var_guard73 = assign7580_e6424;

        let (assign7590_e6442,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard73 != 0.0)) {
        let assign7590_e6431: f64 = (p.p549 * locals.var_ile);
        let assign7590_e6432: f64 = (p.p548 + assign7590_e6431);
        let assign7590_e6435: f64 = (p.p550 * locals.var_iwe);
        let assign7590_e6436: f64 = (assign7590_e6432 + assign7590_e6435);
        let assign7590_e6439: f64 = (p.p551 * locals.var_iae);
        let assign7590_e6440: f64 = (assign7590_e6436 + assign7590_e6439);
        (assign7590_e6440,)
    } else {
        (locals.var_themu_p,)
    }
};
        locals.var_themu_p = assign7590_e6442;

        let assign7600_e6461: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7600_e6461;

        let (assign7610_e6479,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard74 != 0.0)) {
        let assign7610_e6468: f64 = (p.p553 * locals.var_ile);
        let assign7610_e6469: f64 = (p.p552 + assign7610_e6468);
        let assign7610_e6472: f64 = (p.p554 * locals.var_iwe);
        let assign7610_e6473: f64 = (assign7610_e6469 + assign7610_e6472);
        let assign7610_e6476: f64 = (p.p555 * locals.var_iae);
        let assign7610_e6477: f64 = (assign7610_e6473 + assign7610_e6476);
        (assign7610_e6477,)
    } else {
        (locals.var_cs_p,)
    }
};
        locals.var_cs_p = assign7610_e6479;

        let assign7620_e6498: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7620_e6498;

        let (assign7630_e6516,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard75 != 0.0)) {
        let assign7630_e6505: f64 = (p.p557 * locals.var_ile);
        let assign7630_e6506: f64 = (p.p556 + assign7630_e6505);
        let assign7630_e6509: f64 = (p.p558 * locals.var_iwe);
        let assign7630_e6510: f64 = (assign7630_e6506 + assign7630_e6509);
        let assign7630_e6513: f64 = (p.p559 * locals.var_iae);
        let assign7630_e6514: f64 = (assign7630_e6510 + assign7630_e6513);
        (assign7630_e6514,)
    } else {
        (locals.var_thecs_p,)
    }
};
        locals.var_thecs_p = assign7630_e6516;

        let assign7640_e6535: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        locals.var_guard76 = assign7640_e6535;

        let (assign7650_e6553,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard76 != 0.0)) {
        let assign7650_e6542: f64 = (p.p561 * locals.var_ile);
        let assign7650_e6543: f64 = (p.p560 + assign7650_e6542);
        let assign7650_e6546: f64 = (p.p562 * locals.var_iwe);
        let assign7650_e6547: f64 = (assign7650_e6543 + assign7650_e6546);
        let assign7650_e6550: f64 = (p.p563 * locals.var_iae);
        let assign7650_e6551: f64 = (assign7650_e6547 + assign7650_e6550);
        (assign7650_e6551,)
    } else {
        (locals.var_xcor_p,)
    }
};
        locals.var_xcor_p = assign7650_e6553;

        let assign7660_e6572: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        locals.var_guard77 = assign7660_e6572;

        let (assign7670_e6592,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard77 != 0.0)) {
        let assign7670_e6580: f64 = (p.p565 * locals.var_ile);
        let assign7670_e6581: f64 = (p.p564 + assign7670_e6580);
        let assign7670_e6584: f64 = (p.p566 * locals.var_iwe);
        let assign7670_e6585: f64 = (assign7670_e6581 + assign7670_e6584);
        let assign7670_e6588: f64 = (p.p567 * locals.var_iae);
        let assign7670_e6589: f64 = (assign7670_e6585 + assign7670_e6588);
        let assign7670_e6590: f64 = (locals.var_iwe * assign7670_e6589);
        (assign7670_e6590,)
    } else {
        (locals.var_rs_p,)
    }
};
        locals.var_rs_p = assign7670_e6592;

        let assign7680_e6611: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        locals.var_guard78 = assign7680_e6611;

        let (assign7690_e6629,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard78 != 0.0)) {
        let assign7690_e6618: f64 = (p.p569 * locals.var_ile);
        let assign7690_e6619: f64 = (p.p568 + assign7690_e6618);
        let assign7690_e6622: f64 = (p.p570 * locals.var_iwe);
        let assign7690_e6623: f64 = (assign7690_e6619 + assign7690_e6622);
        let assign7690_e6626: f64 = (p.p571 * locals.var_iae);
        let assign7690_e6627: f64 = (assign7690_e6623 + assign7690_e6626);
        (assign7690_e6627,)
    } else {
        (locals.var_strs_p,)
    }
};
        locals.var_strs_p = assign7690_e6629;

        let assign7700_e6648: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        locals.var_guard79 = assign7700_e6648;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign7710_e6666,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard79 != 0.0)) {
        let assign7710_e6655: f64 = (p.p573 * locals.var_ile);
        let assign7710_e6656: f64 = (p.p572 + assign7710_e6655);
        let assign7710_e6659: f64 = (p.p574 * locals.var_iwe);
        let assign7710_e6660: f64 = (assign7710_e6656 + assign7710_e6659);
        let assign7710_e6663: f64 = (p.p575 * locals.var_iae);
        let assign7710_e6664: f64 = (assign7710_e6660 + assign7710_e6663);
        (assign7710_e6664,)
    } else {
        (locals.var_rsb_p,)
    }
};
        locals.var_rsb_p = assign7710_e6666;

        let assign7720_e6685: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        locals.var_guard80 = assign7720_e6685;

        let (assign7730_e6703,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard80 != 0.0)) {
        let assign7730_e6692: f64 = (p.p577 * locals.var_ile);
        let assign7730_e6693: f64 = (p.p576 + assign7730_e6692);
        let assign7730_e6696: f64 = (p.p578 * locals.var_iwe);
        let assign7730_e6697: f64 = (assign7730_e6693 + assign7730_e6696);
        let assign7730_e6700: f64 = (p.p579 * locals.var_iae);
        let assign7730_e6701: f64 = (assign7730_e6697 + assign7730_e6700);
        (assign7730_e6701,)
    } else {
        (locals.var_rsg_p,)
    }
};
        locals.var_rsg_p = assign7730_e6703;

        let assign7740_e6722: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        locals.var_guard81 = assign7740_e6722;

        let (assign7750_e6742,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard81 != 0.0)) {
        let assign7750_e6730: f64 = (p.p581 * locals.var_ile);
        let assign7750_e6731: f64 = (p.p580 + assign7750_e6730);
        let assign7750_e6734: f64 = (p.p582 * locals.var_iwe);
        let assign7750_e6735: f64 = (assign7750_e6731 + assign7750_e6734);
        let assign7750_e6738: f64 = (p.p583 * locals.var_iae);
        let assign7750_e6739: f64 = (assign7750_e6735 + assign7750_e6738);
        let assign7750_e6740: f64 = (locals.var_ile * assign7750_e6739);
        (assign7750_e6740,)
    } else {
        (locals.var_thesat_p,)
    }
};
        locals.var_thesat_p = assign7750_e6742;

        let assign7760_e6761: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        locals.var_guard82 = assign7760_e6761;

        let (assign7770_e6779,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard82 != 0.0)) {
        let assign7770_e6768: f64 = (p.p585 * locals.var_ile);
        let assign7770_e6769: f64 = (p.p584 + assign7770_e6768);
        let assign7770_e6772: f64 = (p.p586 * locals.var_iwe);
        let assign7770_e6773: f64 = (assign7770_e6769 + assign7770_e6772);
        let assign7770_e6776: f64 = (p.p587 * locals.var_iae);
        let assign7770_e6777: f64 = (assign7770_e6773 + assign7770_e6776);
        (assign7770_e6777,)
    } else {
        (locals.var_stthesat_p,)
    }
};
        locals.var_stthesat_p = assign7770_e6779;

        let assign7780_e6798: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        locals.var_guard83 = assign7780_e6798;

        let (assign7790_e6816,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard83 != 0.0)) {
        let assign7790_e6805: f64 = (p.p589 * locals.var_ile);
        let assign7790_e6806: f64 = (p.p588 + assign7790_e6805);
        let assign7790_e6809: f64 = (p.p590 * locals.var_iwe);
        let assign7790_e6810: f64 = (assign7790_e6806 + assign7790_e6809);
        let assign7790_e6813: f64 = (p.p591 * locals.var_iae);
        let assign7790_e6814: f64 = (assign7790_e6810 + assign7790_e6813);
        (assign7790_e6814,)
    } else {
        (locals.var_thesatb_p,)
    }
};
        locals.var_thesatb_p = assign7790_e6816;

        let assign7800_e6835: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        locals.var_guard84 = assign7800_e6835;

        let (assign7810_e6853,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard84 != 0.0)) {
        let assign7810_e6842: f64 = (p.p593 * locals.var_ile);
        let assign7810_e6843: f64 = (p.p592 + assign7810_e6842);
        let assign7810_e6846: f64 = (p.p594 * locals.var_iwe);
        let assign7810_e6847: f64 = (assign7810_e6843 + assign7810_e6846);
        let assign7810_e6850: f64 = (p.p595 * locals.var_iae);
        let assign7810_e6851: f64 = (assign7810_e6847 + assign7810_e6850);
        (assign7810_e6851,)
    } else {
        (locals.var_thesatg_p,)
    }
};
        locals.var_thesatg_p = assign7810_e6853;

        let assign7820_e6872: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        locals.var_guard85 = assign7820_e6872;

        let (assign7830_e6890,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard85 != 0.0)) {
        let assign7830_e6879: f64 = (p.p597 * locals.var_ile);
        let assign7830_e6880: f64 = (p.p596 + assign7830_e6879);
        let assign7830_e6883: f64 = (p.p598 * locals.var_iwe);
        let assign7830_e6884: f64 = (assign7830_e6880 + assign7830_e6883);
        let assign7830_e6887: f64 = (p.p599 * locals.var_iae);
        let assign7830_e6888: f64 = (assign7830_e6884 + assign7830_e6887);
        (assign7830_e6888,)
    } else {
        (locals.var_ax_p,)
    }
};
        locals.var_ax_p = assign7830_e6890;

        let assign7840_e6909: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        locals.var_guard86 = assign7840_e6909;

        let (assign7850_e6929,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard86 != 0.0)) {
        let assign7850_e6917: f64 = (p.p601 * locals.var_ile);
        let assign7850_e6918: f64 = (p.p600 + assign7850_e6917);
        let assign7850_e6921: f64 = (p.p602 * locals.var_iwe);
        let assign7850_e6922: f64 = (assign7850_e6918 + assign7850_e6921);
        let assign7850_e6925: f64 = (p.p603 * locals.var_iae);
        let assign7850_e6926: f64 = (assign7850_e6922 + assign7850_e6925);
        let assign7850_e6927: f64 = (locals.var_ile * assign7850_e6926);
        (assign7850_e6927,)
    } else {
        (locals.var_alp_p,)
    }
};
        locals.var_alp_p = assign7850_e6929;

        let assign7860_e6948: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        locals.var_guard87 = assign7860_e6948;

        let (assign7870_e6966,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard87 != 0.0)) {
        let assign7870_e6955: f64 = (p.p605 * locals.var_ile);
        let assign7870_e6956: f64 = (p.p604 + assign7870_e6955);
        let assign7870_e6959: f64 = (p.p606 * locals.var_iwe);
        let assign7870_e6960: f64 = (assign7870_e6956 + assign7870_e6959);
        let assign7870_e6963: f64 = (p.p607 * locals.var_iae);
        let assign7870_e6964: f64 = (assign7870_e6960 + assign7870_e6963);
        (assign7870_e6964,)
    } else {
        (locals.var_alp1_p,)
    }
};
        locals.var_alp1_p = assign7870_e6966;

        let assign7880_e6985: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        locals.var_guard88 = assign7880_e6985;

        let (assign7890_e7003,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard88 != 0.0)) {
        let assign7890_e6992: f64 = (p.p609 * locals.var_ile);
        let assign7890_e6993: f64 = (p.p608 + assign7890_e6992);
        let assign7890_e6996: f64 = (p.p610 * locals.var_iwe);
        let assign7890_e6997: f64 = (assign7890_e6993 + assign7890_e6996);
        let assign7890_e7000: f64 = (p.p611 * locals.var_iae);
        let assign7890_e7001: f64 = (assign7890_e6997 + assign7890_e7000);
        (assign7890_e7001,)
    } else {
        (locals.var_alp2_p,)
    }
};
        locals.var_alp2_p = assign7890_e7003;

        let assign7900_e7022: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        locals.var_guard89 = assign7900_e7022;

        let (assign7910_e7040,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard89 != 0.0)) {
        let assign7910_e7029: f64 = (p.p613 * locals.var_ile);
        let assign7910_e7030: f64 = (p.p612 + assign7910_e7029);
        let assign7910_e7033: f64 = (p.p614 * locals.var_iwe);
        let assign7910_e7034: f64 = (assign7910_e7030 + assign7910_e7033);
        let assign7910_e7037: f64 = (p.p615 * locals.var_iae);
        let assign7910_e7038: f64 = (assign7910_e7034 + assign7910_e7037);
        (assign7910_e7038,)
    } else {
        (locals.var_a1_p,)
    }
};
        locals.var_a1_p = assign7910_e7040;

        let assign7920_e7059: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        locals.var_guard90 = assign7920_e7059;

        let (assign7930_e7077,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard90 != 0.0)) {
        let assign7930_e7066: f64 = (p.p617 * locals.var_ile);
        let assign7930_e7067: f64 = (p.p616 + assign7930_e7066);
        let assign7930_e7070: f64 = (p.p618 * locals.var_iwe);
        let assign7930_e7071: f64 = (assign7930_e7067 + assign7930_e7070);
        let assign7930_e7074: f64 = (p.p619 * locals.var_iae);
        let assign7930_e7075: f64 = (assign7930_e7071 + assign7930_e7074);
        (assign7930_e7075,)
    } else {
        (locals.var_sta2_p,)
    }
};
        locals.var_sta2_p = assign7930_e7077;

        let assign7940_e7096: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        locals.var_guard91 = assign7940_e7096;

        let (assign7950_e7114,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard91 != 0.0)) {
        let assign7950_e7103: f64 = (p.p621 * locals.var_ile);
        let assign7950_e7104: f64 = (p.p620 + assign7950_e7103);
        let assign7950_e7107: f64 = (p.p622 * locals.var_iwe);
        let assign7950_e7108: f64 = (assign7950_e7104 + assign7950_e7107);
        let assign7950_e7111: f64 = (p.p623 * locals.var_iae);
        let assign7950_e7112: f64 = (assign7950_e7108 + assign7950_e7111);
        (assign7950_e7112,)
    } else {
        (locals.var_a3_p,)
    }
};
        locals.var_a3_p = assign7950_e7114;

        let assign7960_e7133: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        locals.var_guard92 = assign7960_e7133;

        let (assign7970_e7151,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard92 != 0.0)) {
        let assign7970_e7140: f64 = (p.p625 * locals.var_ile);
        let assign7970_e7141: f64 = (p.p624 + assign7970_e7140);
        let assign7970_e7144: f64 = (p.p626 * locals.var_iwe);
        let assign7970_e7145: f64 = (assign7970_e7141 + assign7970_e7144);
        let assign7970_e7148: f64 = (p.p627 * locals.var_iae);
        let assign7970_e7149: f64 = (assign7970_e7145 + assign7970_e7148);
        (assign7970_e7149,)
    } else {
        (locals.var_a4_p,)
    }
};
        locals.var_a4_p = assign7970_e7151;

        let assign7980_e7170: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        locals.var_guard93 = assign7980_e7170;

        let (assign7990_e7190,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard93 != 0.0)) {
        let assign7990_e7178: f64 = (p.p629 * locals.var_ile);
        let assign7990_e7179: f64 = (p.p628 + assign7990_e7178);
        let assign7990_e7182: f64 = (p.p630 * locals.var_iwe);
        let assign7990_e7183: f64 = (assign7990_e7179 + assign7990_e7182);
        let assign7990_e7186: f64 = (p.p631 * locals.var_iae);
        let assign7990_e7187: f64 = (assign7990_e7183 + assign7990_e7186);
        let assign7990_e7188: f64 = (locals.var_iiae * assign7990_e7187);
        (assign7990_e7188,)
    } else {
        (locals.var_iginv_p,)
    }
};
        locals.var_iginv_p = assign7990_e7190;

        let assign8000_e7209: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        locals.var_guard94 = assign8000_e7209;

        let (assign8010_e7229,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard94 != 0.0)) {
        let assign8010_e7217: f64 = (p.p633 * locals.var_ile);
        let assign8010_e7218: f64 = (p.p632 + assign8010_e7217);
        let assign8010_e7221: f64 = (p.p634 * locals.var_iwe);
        let assign8010_e7222: f64 = (assign8010_e7218 + assign8010_e7221);
        let assign8010_e7225: f64 = (p.p635 * locals.var_iae);
        let assign8010_e7226: f64 = (assign8010_e7222 + assign8010_e7225);
        let assign8010_e7227: f64 = (locals.var_iiwe * assign8010_e7226);
        (assign8010_e7227,)
    } else {
        (locals.var_igov_p,)
    }
};
        locals.var_igov_p = assign8010_e7229;

        let assign8020_e7248: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        locals.var_guard95 = assign8020_e7248;

        let (assign8030_e7268,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard95 != 0.0)) {
        let assign8030_e7256: f64 = (p.p637 * locals.var_ile);
        let assign8030_e7257: f64 = (p.p636 + assign8030_e7256);
        let assign8030_e7260: f64 = (p.p638 * locals.var_iwe);
        let assign8030_e7261: f64 = (assign8030_e7257 + assign8030_e7260);
        let assign8030_e7264: f64 = (p.p639 * locals.var_iae);
        let assign8030_e7265: f64 = (assign8030_e7261 + assign8030_e7264);
        let assign8030_e7266: f64 = (locals.var_iiwe * assign8030_e7265);
        (assign8030_e7266,)
    } else {
        (locals.var_igovd_p,)
    }
};
        locals.var_igovd_p = assign8030_e7268;

        let assign8040_e7287: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        locals.var_guard96 = assign8040_e7287;

        let (assign8050_e7305,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard96 != 0.0)) {
        let assign8050_e7294: f64 = (p.p641 * locals.var_ile);
        let assign8050_e7295: f64 = (p.p640 + assign8050_e7294);
        let assign8050_e7298: f64 = (p.p642 * locals.var_iwe);
        let assign8050_e7299: f64 = (assign8050_e7295 + assign8050_e7298);
        let assign8050_e7302: f64 = (p.p643 * locals.var_iae);
        let assign8050_e7303: f64 = (assign8050_e7299 + assign8050_e7302);
        (assign8050_e7303,)
    } else {
        (locals.var_stig_p,)
    }
};
        locals.var_stig_p = assign8050_e7305;

        let assign8060_e7324: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        locals.var_guard97 = assign8060_e7324;

        let (assign8070_e7344,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard97 != 0.0)) {
        let assign8070_e7332: f64 = (p.p645 * locals.var_ile);
        let assign8070_e7333: f64 = (p.p644 + assign8070_e7332);
        let assign8070_e7336: f64 = (p.p646 * locals.var_iwe);
        let assign8070_e7337: f64 = (assign8070_e7333 + assign8070_e7336);
        let assign8070_e7340: f64 = (p.p647 * locals.var_iae);
        let assign8070_e7341: f64 = (assign8070_e7337 + assign8070_e7340);
        let assign8070_e7342: f64 = (locals.var_iiwe * assign8070_e7341);
        (assign8070_e7342,)
    } else {
        (locals.var_agidl_p,)
    }
};
        locals.var_agidl_p = assign8070_e7344;

        let assign8080_e7363: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        locals.var_guard98 = assign8080_e7363;

        let (assign8090_e7383,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard98 != 0.0)) {
        let assign8090_e7371: f64 = (p.p649 * locals.var_ile);
        let assign8090_e7372: f64 = (p.p648 + assign8090_e7371);
        let assign8090_e7375: f64 = (p.p650 * locals.var_iwe);
        let assign8090_e7376: f64 = (assign8090_e7372 + assign8090_e7375);
        let assign8090_e7379: f64 = (p.p651 * locals.var_iae);
        let assign8090_e7380: f64 = (assign8090_e7376 + assign8090_e7379);
        let assign8090_e7381: f64 = (locals.var_iiwe * assign8090_e7380);
        (assign8090_e7381,)
    } else {
        (locals.var_agidld_p,)
    }
};
        locals.var_agidld_p = assign8090_e7383;

        let assign8100_e7402: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        locals.var_guard99 = assign8100_e7402;

        let (assign8110_e7420,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard99 != 0.0)) {
        let assign8110_e7409: f64 = (p.p653 * locals.var_ile);
        let assign8110_e7410: f64 = (p.p652 + assign8110_e7409);
        let assign8110_e7413: f64 = (p.p654 * locals.var_iwe);
        let assign8110_e7414: f64 = (assign8110_e7410 + assign8110_e7413);
        let assign8110_e7417: f64 = (p.p655 * locals.var_iae);
        let assign8110_e7418: f64 = (assign8110_e7414 + assign8110_e7417);
        (assign8110_e7418,)
    } else {
        (locals.var_stbgidl_p,)
    }
};
        locals.var_stbgidl_p = assign8110_e7420;

        let assign8120_e7439: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        locals.var_guard100 = assign8120_e7439;

        let (assign8130_e7457,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard100 != 0.0)) {
        let assign8130_e7446: f64 = (p.p657 * locals.var_ile);
        let assign8130_e7447: f64 = (p.p656 + assign8130_e7446);
        let assign8130_e7450: f64 = (p.p658 * locals.var_iwe);
        let assign8130_e7451: f64 = (assign8130_e7447 + assign8130_e7450);
        let assign8130_e7454: f64 = (p.p659 * locals.var_iae);
        let assign8130_e7455: f64 = (assign8130_e7451 + assign8130_e7454);
        (assign8130_e7455,)
    } else {
        (locals.var_stbgidld_p,)
    }
};
        locals.var_stbgidld_p = assign8130_e7457;

        let assign8140_e7476: f64 = if (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) { 1.0 } else { 0.0 };
        locals.var_guard101 = assign8140_e7476;

        let (assign8150_e7500,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard101 != 0.0)) {
        let assign8150_e7482: f64 = (locals.var_iiwecv * locals.var_lecv);
        let assign8150_e7484: f64 = (assign8150_e7482 / 1e-6);
        let assign8150_e7488: f64 = (p.p661 * locals.var_ile);
        let assign8150_e7489: f64 = (p.p660 + assign8150_e7488);
        let assign8150_e7492: f64 = (p.p662 * locals.var_iwe);
        let assign8150_e7493: f64 = (assign8150_e7489 + assign8150_e7492);
        let assign8150_e7496: f64 = (p.p663 * locals.var_iae);
        let assign8150_e7497: f64 = (assign8150_e7493 + assign8150_e7496);
        let assign8150_e7498: f64 = (assign8150_e7484 * assign8150_e7497);
        (assign8150_e7498,)
    } else {
        (locals.var_cox_p,)
    }
};
        locals.var_cox_p = assign8150_e7500;

        let assign8160_e7519: f64 = if (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) { 1.0 } else { 0.0 };
        locals.var_guard102 = assign8160_e7519;

        let (assign8170_e7537,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard102 != 0.0)) {
        let assign8170_e7526: f64 = (p.p665 * locals.var_ile);
        let assign8170_e7527: f64 = (p.p664 + assign8170_e7526);
        let assign8170_e7530: f64 = (p.p666 * locals.var_iwe);
        let assign8170_e7531: f64 = (assign8170_e7527 + assign8170_e7530);
        let assign8170_e7534: f64 = (p.p667 * locals.var_iae);
        let assign8170_e7535: f64 = (assign8170_e7531 + assign8170_e7534);
        (assign8170_e7535,)
    } else {
        (locals.var_delvtac_p,)
    }
};
        locals.var_delvtac_p = assign8170_e7537;

        let assign8180_e7556: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        locals.var_guard103 = assign8180_e7556;

        let (assign8190_e7574,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard103 != 0.0)) {
        let assign8190_e7563: f64 = (p.p669 * locals.var_ile);
        let assign8190_e7564: f64 = (p.p668 + assign8190_e7563);
        let assign8190_e7567: f64 = (p.p670 * locals.var_iwe);
        let assign8190_e7568: f64 = (assign8190_e7564 + assign8190_e7567);
        let assign8190_e7571: f64 = (p.p671 * locals.var_iae);
        let assign8190_e7572: f64 = (assign8190_e7568 + assign8190_e7571);
        (assign8190_e7572,)
    } else {
        (locals.var_facneffac_p,)
    }
};
        locals.var_facneffac_p = assign8190_e7574;

        let assign8200_e7613: f64 = if (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        locals.var_guard104 = assign8200_e7613;

        let (assign8210_e7619,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p580,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8210_e7619;

        let assign8220_e7621: f64 = if param_given[672] { 1.0 } else { 0.0 };
        let assign8220_e7623: f64 = if assign8220_e7621 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign8220_e7623;

        let (assign8230_e7631,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard105 != 0.0)) {
        (p.p672,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8230_e7631;

        let (assign8240_e7637,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p581,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8240_e7637;

        let assign8250_e7639: f64 = if param_given[673] { 1.0 } else { 0.0 };
        let assign8250_e7641: f64 = if assign8250_e7639 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign8250_e7641;

        let (assign8260_e7649,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard106 != 0.0)) {
        (p.p673,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8260_e7649;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign8270_e7655,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p582,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8270_e7655;

        let assign8280_e7657: f64 = if param_given[674] { 1.0 } else { 0.0 };
        let assign8280_e7659: f64 = if assign8280_e7657 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign8280_e7659;

        let (assign8290_e7667,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard107 != 0.0)) {
        (p.p674,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8290_e7667;

        let (assign8300_e7673,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        (p.p583,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8300_e7673;

        let assign8310_e7675: f64 = if param_given[675] { 1.0 } else { 0.0 };
        let assign8310_e7677: f64 = if assign8310_e7675 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign8310_e7677;

        let (assign8320_e7685,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) && (locals.var_guard108 != 0.0)) {
        (p.p675,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8320_e7685;

        let (assign8330_e7705,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard104 != 0.0)) {
        let assign8330_e7693: f64 = (locals.var_plparam_i * locals.var_ile);
        let assign8330_e7694: f64 = (locals.var_poparam_i + assign8330_e7693);
        let assign8330_e7697: f64 = (locals.var_pwparam_i * locals.var_iwe);
        let assign8330_e7698: f64 = (assign8330_e7694 + assign8330_e7697);
        let assign8330_e7701: f64 = (locals.var_plwparam_i * locals.var_iae);
        let assign8330_e7702: f64 = (assign8330_e7698 + assign8330_e7701);
        let assign8330_e7703: f64 = (locals.var_ile * assign8330_e7702);
        (assign8330_e7703,)
    } else {
        (locals.var_thesatac_p,)
    }
};
        locals.var_thesatac_p = assign8330_e7705;

        let assign8340_e7744: f64 = if (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        locals.var_guard109 = assign8340_e7744;

        let (assign8350_e7750,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p596,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8350_e7750;

        let assign8360_e7752: f64 = if param_given[676] { 1.0 } else { 0.0 };
        let assign8360_e7754: f64 = if assign8360_e7752 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign8360_e7754;

        let (assign8370_e7762,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard110 != 0.0)) {
        (p.p676,)
    } else {
        (locals.var_poparam_i,)
    }
};
        locals.var_poparam_i = assign8370_e7762;

        let (assign8380_e7768,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p597,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8380_e7768;

        let assign8390_e7770: f64 = if param_given[677] { 1.0 } else { 0.0 };
        let assign8390_e7772: f64 = if assign8390_e7770 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign8390_e7772;

        let (assign8400_e7780,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard111 != 0.0)) {
        (p.p677,)
    } else {
        (locals.var_plparam_i,)
    }
};
        locals.var_plparam_i = assign8400_e7780;

        let (assign8410_e7786,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p598,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8410_e7786;

        let assign8420_e7788: f64 = if param_given[678] { 1.0 } else { 0.0 };
        let assign8420_e7790: f64 = if assign8420_e7788 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign8420_e7790;

        let (assign8430_e7798,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard112 != 0.0)) {
        (p.p678,)
    } else {
        (locals.var_pwparam_i,)
    }
};
        locals.var_pwparam_i = assign8430_e7798;

        let (assign8440_e7804,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) {
        (p.p599,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8440_e7804;

        let assign8450_e7806: f64 = if param_given[679] { 1.0 } else { 0.0 };
        let assign8450_e7808: f64 = if assign8450_e7806 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign8450_e7808;

        let (assign8460_e7816,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard109 != 0.0)) && (locals.var_guard113 != 0.0)) {
        (p.p679,)
    } else {
        (locals.var_plwparam_i,)
    }
};
        locals.var_plwparam_i = assign8460_e7816;

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

        let assign8480_e7855: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign8480_e7855;

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

        let assign8500_e7894: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        locals.var_guard115 = assign8500_e7894;

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

        let assign8520_e7933: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        locals.var_guard116 = assign8520_e7933;

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

        let assign8540_e7972: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        locals.var_guard117 = assign8540_e7972;

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

        let assign8560_e8011: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };
        locals.var_guard118 = assign8560_e8011;

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

        let assign8580_e8050: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };
        locals.var_guard119 = assign8580_e8050;

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

        let assign8600_e8089: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };
        locals.var_guard120 = assign8600_e8089;

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

        let assign8660_e8206: f64 = if (((param_given[716] || param_given[717]) || param_given[718]) || param_given[719]) { 1.0 } else { 0.0 };
        locals.var_guard123 = assign8660_e8206;

        let (assign8670_e8226,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard123 != 0.0)) {
        let assign8670_e8214: f64 = (p.p717 * locals.var_ile);
        let assign8670_e8215: f64 = (p.p716 + assign8670_e8214);
        let assign8670_e8218: f64 = (p.p718 * locals.var_iwe);
        let assign8670_e8219: f64 = (assign8670_e8215 + assign8670_e8218);
        let assign8670_e8222: f64 = (p.p719 * locals.var_iae);
        let assign8670_e8223: f64 = (assign8670_e8219 + assign8670_e8222);
        let assign8670_e8224: f64 = (locals.var_ile2 * assign8670_e8223);
        (assign8670_e8224,)
    } else {
        (locals.var_fntexc_p,)
    }
};
        locals.var_fntexc_p = assign8670_e8226;

        let assign8740_e8362: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        locals.var_guard127 = assign8740_e8362;

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

        let assign8760_e8399: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        locals.var_guard128 = assign8760_e8399;

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

        let assign8780_e8436: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign8780_e8436;

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

        let assign8800_e8473: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        locals.var_guard130 = assign8800_e8473;

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

        let assign8820_e8510: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        locals.var_guard131 = assign8820_e8510;

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

        let assign8840_e8547: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        locals.var_guard132 = assign8840_e8547;

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

        let assign8860_e8588: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign8860_e8588;

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

        let assign8880_e8625: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        locals.var_guard134 = assign8880_e8625;

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

        let assign8900_e8664: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        locals.var_guard135 = assign8900_e8664;

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

        let assign8920_e8701: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        locals.var_guard136 = assign8920_e8701;

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

        let assign8940_e8738: f64 = if (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign8940_e8738;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
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

        let assign8960_e8777: f64 = if (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]) { 1.0 } else { 0.0 };
        locals.var_guard138 = assign8960_e8777;

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

        let assign8980_e8814: f64 = if (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]) { 1.0 } else { 0.0 };
        locals.var_guard139 = assign8980_e8814;

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

        let assign9060_e8968: f64 = if (((param_given[796] || param_given[797]) || param_given[798]) || param_given[799]) { 1.0 } else { 0.0 };
        locals.var_guard143 = assign9060_e8968;

        let (assign9070_e8988,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard143 != 0.0)) {
        let assign9070_e8976: f64 = (p.p797 * locals.var_ile);
        let assign9070_e8977: f64 = (p.p796 + assign9070_e8976);
        let assign9070_e8980: f64 = (p.p798 * locals.var_iwe);
        let assign9070_e8981: f64 = (assign9070_e8977 + assign9070_e8980);
        let assign9070_e8984: f64 = (p.p799 * locals.var_iae);
        let assign9070_e8985: f64 = (assign9070_e8981 + assign9070_e8984);
        let assign9070_e8986: f64 = (locals.var_iae * assign9070_e8985);
        (assign9070_e8986,)
    } else {
        (locals.var_rth_p,)
    }
};
        locals.var_rth_p = assign9070_e8988;

        let (assign9120_e9068,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpa,)
    }
};
        locals.var_tmpa = assign9120_e9068;

        let (assign9130_e9072,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_tmpb,)
    }
};
        locals.var_tmpb = assign9130_e9072;

        let (assign9140_e9076,) = {
    if (locals.var_guard36 != 0.0) {
        (0.0,)
    } else {
        (locals.var_loop_,)
    }
};
        locals.var_loop_ = assign9140_e9076;

        let (assign9150_e9080,) = {
    if (locals.var_guard36 != 0.0) {
        (p.p812,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9150_e9080;

        let assign9160_e9082: f64 = if param_given[813] { 1.0 } else { 0.0 };
        let assign9160_e9084: f64 = if assign9160_e9082 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign9160_e9084;

        let (assign9170_e9090,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard146 != 0.0)) {
        (p.p813,)
    } else {
        (locals.var_kvsatac_i,)
    }
};
        locals.var_kvsatac_i = assign9170_e9090;

        let assign9180_e9109: f64 = if (((locals.var_sa_i > 0.0) && (locals.var_sb_i > 0.0)) && ((locals.var_nf_i == 1.0) || ((locals.var_nf_i > 1.0) && (locals.var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard147 = assign9180_e9109;

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
            let (assign9190_body2_e9166,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9190_body2_e9164: f64 = (locals.var_loop_ + 1.0);
        (assign9190_body2_e9164,)
    } else {
        (locals.var_loop_,)
    }
};
            locals.var_loop_ = assign9190_body2_e9166;
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

        let (assign9210_e9182,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9210_e9180: f64 = (locals.var_tmpb * locals.var_invnf);
        (assign9210_e9180,)
    } else {
        (locals.var_invsb,)
    }
};
        locals.var_invsb = assign9210_e9182;

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

        let (assign9360_e9384,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9360_e9382: f64 = (locals.var_betn_p * locals.var_temp00);
        (assign9360_e9382,)
    } else {
        (locals.var_betn_p,)
    }
};
        locals.var_betn_p = assign9360_e9384;

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

        let (assign9390_e9432,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9390_e9430: f64 = (locals.var_betnedge_p * locals.var_temp00);
        (assign9390_e9430,)
    } else {
        (locals.var_betnedge_p,)
    }
};
        locals.var_betnedge_p = assign9390_e9432;

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

        let (assign9410_e9450,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9410_e9448: f64 = (locals.var_vfb_p + locals.var_temp00);
        (assign9410_e9448,)
    } else {
        (locals.var_vfb_p,)
    }
};
        locals.var_vfb_p = assign9410_e9450;

        let (assign9420_e9458,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9420_e9456: f64 = (locals.var_vfbedge_p + locals.var_temp00);
        (assign9420_e9456,)
    } else {
        (locals.var_vfbedge_p,)
    }
};
        locals.var_vfbedge_p = assign9420_e9458;

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

        let (assign9440_e9478,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9440_e9476: f64 = (locals.var_cf_p + locals.var_temp00);
        (assign9440_e9476,)
    } else {
        (locals.var_cf_p,)
    }
};
        locals.var_cf_p = assign9440_e9478;

        let (assign9450_e9486,) = {
    if ((locals.var_guard36 != 0.0) && (locals.var_guard147 != 0.0)) {
        let assign9450_e9484: f64 = (locals.var_cfedge_p + locals.var_temp00);
        (assign9450_e9484,)
    } else {
        (locals.var_cfedge_p,)
    }
};
        locals.var_cfedge_p = assign9450_e9486;

        let assign9460_e9501: f64 = if ((((locals.var_sca_i > 0.0) || (locals.var_scb_i > 0.0)) || (locals.var_scc_i > 0.0)) || (locals.var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard148 = assign9460_e9501;

        let assign9470_e9512: f64 = if (((locals.var_sca_i == 0.0) && (locals.var_scb_i == 0.0)) && (locals.var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard149 = assign9470_e9512;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9480_e9522,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign9480_e9520: f64 = (locals.var_sc_i + locals.var_w_i);
        (assign9480_e9520,)
    } else {
        (locals.var_temp0,)
    }
};
        locals.var_temp0 = assign9480_e9522;

        let (assign9490_e9532,) = {
    if (((locals.var_guard36 != 0.0) && (locals.var_guard148 != 0.0)) && (locals.var_guard149 != 0.0)) {
        let assign9490_e9530: f64 = (1.0 / p.p828);
        (assign9490_e9530,)
    } else {
        (locals.var_temp00,)
    }
};
        locals.var_temp00 = assign9490_e9532;

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

        locals.var_vfb_i = locals.var_vfb_p;

        locals.var_stvfb_i = locals.var_stvfb_p;

        locals.var_st2vfb_i = locals.var_st2vfb_p;

        locals.var_tox_i = locals.var_tox_p;

        locals.var_epsrox_i = locals.var_epsrox_p;

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

        let (assign9640_e9706,) = {
    if (locals.var_gfacnud_p > 0.01) {
        (locals.var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        locals.var_gfacnud_i = assign9640_e9706;

        let (assign9650_e9712,) = {
    if (locals.var_vsbnud_p > 0.0) {
        (locals.var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        locals.var_vsbnud_i = assign9650_e9712;

        locals.var_dvsbnud_i = locals.var_dvsbnud_p;

        locals.var_dphib_i = locals.var_dphib_p;

        let (assign9680_e9720,) = {
    if (locals.var_np_p > 0.0) {
        (locals.var_np_p,)
    } else {
        (0.0,)
    }
};
        locals.var_np_i = assign9680_e9720;

        locals.var_toxov_i = locals.var_toxov_p;

        locals.var_toxovd_i = locals.var_toxovd_p;

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

        let (assign9730_e9750,) = {
    if (locals.var_ct_p > 0.0) {
        (locals.var_ct_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ct_i = assign9730_e9750;

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

        locals.var_stct_i = locals.var_stct_p;

        let (assign9770_e9779,) = {
    if (locals.var_cf_p > 0.0) {
        (locals.var_cf_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cf_i = assign9770_e9779;

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

        let (assign9790_e9796,) = {
    if (locals.var_cfd_p > 0.0) {
        (locals.var_cfd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfd_i = assign9790_e9796;

        let (assign9800_e9802,) = {
    if (locals.var_psce_p > 0.0) {
        (locals.var_psce_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psce_i = assign9800_e9802;

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

        let (assign9820_e9819,) = {
    if (locals.var_psced_p > 0.0) {
        (locals.var_psced_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psced_i = assign9820_e9819;

        let (assign9830_e9825,) = {
    if (locals.var_betn_p > 0.0) {
        (locals.var_betn_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betn_i = assign9830_e9825;

        locals.var_stbet_i = locals.var_stbet_p;

        let (assign9850_e9832,) = {
    if (locals.var_mue_p > 0.0) {
        (locals.var_mue_p,)
    } else {
        (0.0,)
    }
};
        locals.var_mue_i = assign9850_e9832;

        locals.var_stmue_i = locals.var_stmue_p;

        let (assign9870_e9839,) = {
    if (locals.var_themu_p > 0.0) {
        (locals.var_themu_p,)
    } else {
        (0.0,)
    }
};
        locals.var_themu_i = assign9870_e9839;

        locals.var_stthemu_i = locals.var_stthemu_p;

        let (assign9890_e9846,) = {
    if (locals.var_cs_p > 0.0) {
        (locals.var_cs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cs_i = assign9890_e9846;

        locals.var_stcs_i = locals.var_stcs_p;

        let (assign9910_e9853,) = {
    if (locals.var_thecs_p > 0.0) {
        (locals.var_thecs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thecs_i = assign9910_e9853;

        locals.var_stthecs_i = locals.var_stthecs_p;

        let (assign9930_e9860,) = {
    if (locals.var_xcor_p > 0.0) {
        (locals.var_xcor_p,)
    } else {
        (0.0,)
    }
};
        locals.var_xcor_i = assign9930_e9860;

        locals.var_stxcor_i = locals.var_stxcor_p;

        locals.var_feta_i = locals.var_feta_p;

        let (assign9960_e9868,) = {
    if (locals.var_rs_p > 0.0) {
        (locals.var_rs_p,)
    } else {
        (0.0,)
    }
};
        locals.var_rs_i = assign9960_e9868;

        locals.var_strs_i = locals.var_strs_p;

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

        let (assign10000_e9896,) = {
    if (locals.var_thesat_p > 0.0) {
        (locals.var_thesat_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesat_i = assign10000_e9896;

        locals.var_stthesat_i = locals.var_stthesat_p;

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

        let (assign10040_e9924,) = {
    if (locals.var_thesatt_p > 0.01) {
        (locals.var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        locals.var_thesatt_i = assign10040_e9924;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10050_e9930,) = {
    if (locals.var_ax_p > 2.0) {
        (locals.var_ax_p,)
    } else {
        (2.0,)
    }
};
        locals.var_ax_i = assign10050_e9930;

        let (assign10060_e9936,) = {
    if (locals.var_alp_p > 0.0) {
        (locals.var_alp_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp_i = assign10060_e9936;

        let (assign10070_e9942,) = {
    if (locals.var_alp1_p > 0.0) {
        (locals.var_alp1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1_i = assign10070_e9942;

        let (assign10080_e9948,) = {
    if (locals.var_alp2_p > 0.0) {
        (locals.var_alp2_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp2_i = assign10080_e9948;

        locals.var_vp_i = locals.var_vp_p;

        let (assign10100_e9955,) = {
    if (locals.var_a1_p > 0.0) {
        (locals.var_a1_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a1_i = assign10100_e9955;

        locals.var_a2_i = locals.var_a2_p;

        locals.var_sta2_i = locals.var_sta2_p;

        let (assign10130_e9963,) = {
    if (locals.var_a3_p > 0.0) {
        (locals.var_a3_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a3_i = assign10130_e9963;

        let (assign10140_e9969,) = {
    if (locals.var_a4_p > 0.0) {
        (locals.var_a4_p,)
    } else {
        (0.0,)
    }
};
        locals.var_a4_i = assign10140_e9969;

        let (assign10150_e9975,) = {
    if (locals.var_imaxii_p > 1e-12) {
        (locals.var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        locals.var_imaxii_i = assign10150_e9975;

        locals.var_gco_i = locals.var_gco_p;

        let (assign10170_e9982,) = {
    if (locals.var_iginv_p > 0.0) {
        (locals.var_iginv_p,)
    } else {
        (0.0,)
    }
};
        locals.var_iginv_i = assign10170_e9982;

        let (assign10180_e9988,) = {
    if (locals.var_igov_p > 0.0) {
        (locals.var_igov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igov_i = assign10180_e9988;

        let (assign10190_e9994,) = {
    if (locals.var_igovd_p > 0.0) {
        (locals.var_igovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_igovd_i = assign10190_e9994;

        locals.var_stig_i = locals.var_stig_p;

        locals.var_gc2_i = locals.var_gc2_p;

        locals.var_gc3_i = locals.var_gc3_p;

        locals.var_gc2ov_i = locals.var_gc2ov_p;

        locals.var_gc3ov_i = locals.var_gc3ov_p;

        locals.var_gc2ovd_i = locals.var_gc2ovd_p;

        locals.var_gc3ovd_i = locals.var_gc3ovd_p;

        locals.var_chib_i = locals.var_chib_p;

        let (assign10280_e10008,) = {
    if (locals.var_agidl_p > 0.0) {
        (locals.var_agidl_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidl_i = assign10280_e10008;

        let (assign10290_e10014,) = {
    if (locals.var_agidld_p > 0.0) {
        (locals.var_agidld_p,)
    } else {
        (0.0,)
    }
};
        locals.var_agidld_i = assign10290_e10014;

        locals.var_bgidl_i = locals.var_bgidl_p;

        locals.var_bgidld_i = locals.var_bgidld_p;

        locals.var_stbgidl_i = locals.var_stbgidl_p;

        locals.var_stbgidld_i = locals.var_stbgidld_p;

        locals.var_cgidl_i = locals.var_cgidl_p;

        locals.var_cgidld_i = locals.var_cgidld_p;

        let (assign10360_e10026,) = {
    if (locals.var_cox_p > 0.0) {
        (locals.var_cox_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cox_i = assign10360_e10026;

        locals.var_delvtac_i = locals.var_delvtac_p;

        let (assign10380_e10033,) = {
    if (locals.var_facneffac_p > 0.0) {
        (locals.var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_facneffac_i = assign10380_e10033;

        let (assign10390_e10039,) = {
    if (locals.var_thesatac_p > 0.0) {
        (locals.var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_thesatac_i = assign10390_e10039;

        let (assign10400_e10045,) = {
    if (locals.var_axac_p > 2.0) {
        (locals.var_axac_p,)
    } else {
        (2.0,)
    }
};
        locals.var_axac_i = assign10400_e10045;

        locals.var_alpac_i = locals.var_alpac_p;

        let (assign10420_e10052,) = {
    if (locals.var_alp1ac_p > 0.0) {
        (locals.var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        locals.var_alp1ac_i = assign10420_e10052;

        let (assign10430_e10058,) = {
    if (locals.var_cgov_p > 0.0) {
        (locals.var_cgov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgov_i = assign10430_e10058;

        let (assign10440_e10064,) = {
    if (locals.var_cgovd_p > 0.0) {
        (locals.var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgovd_i = assign10440_e10064;

        locals.var_fcgovacc_i = locals.var_fcgovacc_p;

        locals.var_fcgovaccd_i = locals.var_fcgovaccd_p;

        locals.var_cgovaccg_i = locals.var_cgovaccg_p;

        let (assign10480_e10073,) = {
    if (locals.var_cgbov_p > 0.0) {
        (locals.var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cgbov_i = assign10480_e10073;

        let (assign10490_e10079,) = {
    if (locals.var_cinr_p > 0.0) {
        (locals.var_cinr_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinr_i = assign10490_e10079;

        let (assign10500_e10085,) = {
    if (locals.var_cinrd_p > 0.0) {
        (locals.var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cinrd_i = assign10500_e10085;

        locals.var_dvfbinr_i = locals.var_dvfbinr_p;

        locals.var_fcinrdep_i = locals.var_fcinrdep_p;

        locals.var_fcinracc_i = locals.var_fcinracc_p;

        locals.var_axinr_i = locals.var_axinr_p;

        locals.var_fnt_i = locals.var_fnt_p;

        let (assign10580_e10108,) = {
    if (locals.var_fntexc_p > 0.0) {
        (locals.var_fntexc_p,)
    } else {
        (0.0,)
    }
};
        locals.var_fntexc_i = assign10580_e10108;

        locals.var_vfbedge_i = locals.var_vfbedge_p;

        locals.var_stvfbedge_i = locals.var_stvfbedge_p;

        locals.var_dphibedge_i = locals.var_dphibedge_p;

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

        let (assign10670_e10147,) = {
    if (locals.var_ctedge_p > 0.0) {
        (locals.var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_ctedge_i = assign10670_e10147;

        let (assign10680_e10153,) = {
    if (locals.var_betnedge_p > 0.0) {
        (locals.var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_betnedge_i = assign10680_e10153;

        locals.var_stbetedge_i = locals.var_stbetedge_p;

        let (assign10700_e10160,) = {
    if (locals.var_psceedge_p > 0.0) {
        (locals.var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_psceedge_i = assign10700_e10160;

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

        let (assign10720_e10177,) = {
    if (locals.var_pscededge_p > 0.0) {
        (locals.var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_pscededge_i = assign10720_e10177;

        let (assign10730_e10183,) = {
    if (locals.var_cfedge_p > 0.0) {
        (locals.var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfedge_i = assign10730_e10183;

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

        let (assign10750_e10200,) = {
    if (locals.var_cfdedge_p > 0.0) {
        (locals.var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        locals.var_cfdedge_i = assign10750_e10200;

        locals.var_rse_i = locals.var_rse_p;

        locals.var_rde_i = locals.var_rde_p;

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

        locals.var_factuo_i = p.p16;

        locals.var_delvto_i = p.p15;

        locals.var_factuoedge_i = p.p18;

        locals.var_delvtoedge_i = p.p17;

        let assign10960_e10262: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign10960_e10262;

        let (assign10970_e10266,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_toxov_i,)
    } else {
        (locals.var_toxovd_i,)
    }
};
        locals.var_toxovd_i = assign10970_e10266;

        let (assign10980_e10270,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_nov_i,)
    } else {
        (locals.var_novd_i,)
    }
};
        locals.var_novd_i = assign10980_e10270;

        let (assign10990_e10274,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_agidl_i,)
    } else {
        (locals.var_agidld_i,)
    }
};
        locals.var_agidld_i = assign10990_e10274;

        let (assign11000_e10278,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_bgidl_i,)
    } else {
        (locals.var_bgidld_i,)
    }
};
        locals.var_bgidld_i = assign11000_e10278;

        let (assign11010_e10282,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_stbgidl_i,)
    } else {
        (locals.var_stbgidld_i,)
    }
};
        locals.var_stbgidld_i = assign11010_e10282;

        let (assign11020_e10286,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_cgidl_i,)
    } else {
        (locals.var_cgidld_i,)
    }
};
        locals.var_cgidld_i = assign11020_e10286;

        let (assign11030_e10290,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_igov_i,)
    } else {
        (locals.var_igovd_i,)
    }
};
        locals.var_igovd_i = assign11030_e10290;

        let (assign11040_e10294,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_gc2ov_i,)
    } else {
        (locals.var_gc2ovd_i,)
    }
};
        locals.var_gc2ovd_i = assign11040_e10294;

        let (assign11050_e10298,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_gc3ov_i,)
    } else {
        (locals.var_gc3ovd_i,)
    }
};
        locals.var_gc3ovd_i = assign11050_e10298;

        let (assign11060_e10302,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_cgov_i,)
    } else {
        (locals.var_cgovd_i,)
    }
};
        locals.var_cgovd_i = assign11060_e10302;

        let (assign11070_e10306,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_fcgovacc_i,)
    } else {
        (locals.var_fcgovaccd_i,)
    }
};
        locals.var_fcgovaccd_i = assign11070_e10306;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11080_e10310,) = {
    if (locals.var_guard150 != 0.0) {
        (locals.var_cinr_i,)
    } else {
        (locals.var_cinrd_i,)
    }
};
        locals.var_cinrd_i = assign11080_e10310;

        let assign11100_e10317: f64 = (8.8541878176e-12 * locals.var_epsrox_i);
        locals.var_epsox = assign11100_e10317;

        let assign11110_e10320: f64 = (locals.var_epsox / locals.var_tox_i);
        locals.var_coxprime = assign11110_e10320;

        let assign11120_e10323: f64 = (locals.var_tox_i * locals.var_tox_i);
        locals.var_tox_sq = assign11120_e10323;

        let assign11130_e10326: f64 = (locals.var_coxprime / 1.6021918e-19);
        locals.var_cox_over_q = assign11130_e10326;

        let assign11140_e10329: f64 = (locals.var_facneffac_i * locals.var_neff_i);
        locals.var_neffac_i = assign11140_e10329;

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

        locals.var_qq = 0.0;

        let assign11170_e10344: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign11170_e10344;

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

        let assign11190_e10359: f64 = (-1.0);
        let assign11190_e10360: f64 = if locals.var_chnl_type == assign11190_e10359 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign11190_e10360;

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

        let assign11210_e10373: f64 = (1e-8 * locals.var_coxprime);
        let assign11210_e10375: f64 = (assign11210_e10373 / locals.var_epssi);
        locals.var_e_eff0 = assign11210_e10375;

        let assign11220_e10378: f64 = (0.5 * locals.var_feta_i);
        locals.var_eta_mu = assign11220_e10378;

        locals.var_eta_mu1 = 0.5;

        let assign11240_e10382: f64 = (-1.0);
        let assign11240_e10383: f64 = if locals.var_chnl_type == assign11240_e10382 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign11240_e10383;

        let (assign11250_e10389,) = {
    if (locals.var_guard153 != 0.0) {
        let assign11250_e10387: f64 = (0.3333333333333333 * locals.var_feta_i);
        (assign11250_e10387,)
    } else {
        (locals.var_eta_mu,)
    }
};
        locals.var_eta_mu = assign11250_e10389;

        let (assign11260_e10393,) = {
    if (locals.var_guard153 != 0.0) {
        (0.3333333333333333,)
    } else {
        (locals.var_eta_mu1,)
    }
};
        locals.var_eta_mu1 = assign11260_e10393;

        let assign11270_e10396: f64 = (-2.0);
        let assign11270_e10398: f64 = (assign11270_e10396 / locals.var_ax_i);
        let assign11270_e10400: f64 = (assign11270_e10398 + 1.0);
        let assign11270_e10401: f64 = (2.0_f64).powf(assign11270_e10400);
        let assign11270_e10403: f64 = (assign11270_e10401 - 1.0);
        locals.var_temp = assign11270_e10403;

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

        let assign11290_e10424: f64 = (-2.0);
        let assign11290_e10426: f64 = (assign11290_e10424 / locals.var_axac_i);
        let assign11290_e10428: f64 = (assign11290_e10426 + 1.0);
        let assign11290_e10429: f64 = (2.0_f64).powf(assign11290_e10428);
        let assign11290_e10431: f64 = (assign11290_e10429 - 1.0);
        locals.var_temp = assign11290_e10431;

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

        let assign11310_e10452: f64 = (1.0 / locals.var_vp_i);
        locals.var_inv_vp = assign11310_e10452;

        let assign11320_e10455: f64 = (locals.var_epsox / locals.var_toxov_i);
        locals.var_coxovprime = assign11320_e10455;

        let assign11330_e10458: f64 = (locals.var_epsox / locals.var_toxovd_i);
        locals.var_coxovprime_d = assign11330_e10458;

        let assign11340_e10461: f64 = (2.0 * 1.6021918e-19);
        let assign11340_e10463: f64 = (assign11340_e10461 * locals.var_nov_i);
        let assign11340_e10465: f64 = (assign11340_e10463 * locals.var_epssi);
        let assign11340_e10467: f64 = (assign11340_e10465 * locals.var_inv_phita);
        let assign11340_e10468: f64 = (assign11340_e10467).sqrt();
        let assign11340_e10470: f64 = (assign11340_e10468 / locals.var_coxovprime);
        locals.var_gov_s = assign11340_e10470;

        let assign11350_e10473: f64 = (2.0 * 1.6021918e-19);
        let assign11350_e10475: f64 = (assign11350_e10473 * locals.var_novd_i);
        let assign11350_e10477: f64 = (assign11350_e10475 * locals.var_epssi);
        let assign11350_e10479: f64 = (assign11350_e10477 * locals.var_inv_phita);
        let assign11350_e10480: f64 = (assign11350_e10479).sqrt();
        let assign11350_e10482: f64 = (assign11350_e10480 / locals.var_coxovprime_d);
        locals.var_gov_d = assign11350_e10482;

        let assign11360_e10485: f64 = (locals.var_gov_s * locals.var_gov_s);
        locals.var_gov2_s = assign11360_e10485;

        let assign11370_e10488: f64 = (locals.var_gov_d * locals.var_gov_d);
        locals.var_gov2_d = assign11370_e10488;

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

        let assign11390_e10510: f64 = (0.5 * locals.var_gov_s);
        let assign11390_e10511: f64 = (assign11390_e10510).ln();
        let assign11390_e10513: f64 = (assign11390_e10511 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_s = assign11390_e10513;

        let assign11400_e10516: f64 = (0.5 * locals.var_gov_d);
        let assign11400_e10517: f64 = (assign11400_e10516).ln();
        let assign11400_e10519: f64 = (assign11400_e10517 + locals.var_dxgb_ov_th);
        locals.var_dxgb_ov_d = assign11400_e10519;

        let assign11410_e10522: f64 = (1.0 / locals.var_gov_s);
        locals.var_inv_gov = assign11410_e10522;

        let assign11420_e10525: f64 = (3.1 * locals.var_gov_s);
        let assign11420_e10527: f64 = (assign11420_e10525 + 8.5);
        locals.var_sp_ov_eps = assign11420_e10527;

        let assign11430_e10530: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_s = assign11430_e10530;

        let assign11440_e10533: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11440_e10533;

        let assign11450_e10536: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign11450_e10536;

        let (assign11460_e10542,) = {
    if (locals.var_guard154 != 0.0) {
        let assign11460_e10540: f64 = (64.0 * locals.var_inv_gov);
        (assign11460_e10540,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11460_e10542;

        let assign11470_e10545: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign11470_e10545;

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

        let assign11490_e10559: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign11490_e10559;

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

        let (assign11510_e10585,) = {
    if (((locals.var_guard154 == 0.0) && (locals.var_guard155 == 0.0)) && (locals.var_guard156 == 0.0)) {
        (locals.var_gov_s,)
    } else {
        (locals.var_sp_ov_a_s,)
    }
};
        locals.var_sp_ov_a_s = assign11510_e10585;

        let assign11520_e10589: f64 = (locals.var_gov2_s * 0.5);
        let assign11520_e10590: f64 = (locals.var_sp_ov_delta + assign11520_e10589);
        let assign11520_e10595: f64 = (locals.var_gov2_s * 0.25);
        let assign11520_e10596: f64 = (locals.var_sp_ov_delta + assign11520_e10595);
        let assign11520_e10598: f64 = (assign11520_e10596 + locals.var_sp_ov_a_s);
        let assign11520_e10599: f64 = (assign11520_e10598).sqrt();
        let assign11520_e10600: f64 = (locals.var_gov_s * assign11520_e10599);
        let assign11520_e10601: f64 = (assign11520_e10590 - assign11520_e10600);
        locals.var_sp_ov_delta1_s = assign11520_e10601;

        let assign11530_e10604: f64 = (1.0 / locals.var_gov_d);
        locals.var_inv_gov = assign11530_e10604;

        let assign11540_e10607: f64 = (3.1 * locals.var_gov_d);
        let assign11540_e10609: f64 = (assign11540_e10607 + 8.5);
        locals.var_sp_ov_eps = assign11540_e10609;

        let assign11550_e10612: f64 = (locals.var_sp_ov_eps * locals.var_sp_ov_eps);
        locals.var_sp_ov_eps2_d = assign11550_e10612;

        let assign11560_e10615: f64 = (0.5 * locals.var_sp_ov_eps);
        locals.var_sp_ov_delta = assign11560_e10615;

        let assign11570_e10618: f64 = if locals.var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign11570_e10618;

        let (assign11580_e10624,) = {
    if (locals.var_guard157 != 0.0) {
        let assign11580_e10622: f64 = (64.0 * locals.var_inv_gov);
        (assign11580_e10622,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11580_e10624;

        let assign11590_e10627: f64 = if locals.var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign11590_e10627;

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

        let assign11610_e10641: f64 = if locals.var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign11610_e10641;

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

        let (assign11630_e10667,) = {
    if (((locals.var_guard157 == 0.0) && (locals.var_guard158 == 0.0)) && (locals.var_guard159 == 0.0)) {
        (locals.var_gov_d,)
    } else {
        (locals.var_sp_ov_a_d,)
    }
};
        locals.var_sp_ov_a_d = assign11630_e10667;

        let assign11640_e10671: f64 = (locals.var_gov2_d * 0.5);
        let assign11640_e10672: f64 = (locals.var_sp_ov_delta + assign11640_e10671);
        let assign11640_e10677: f64 = (locals.var_gov2_d * 0.25);
        let assign11640_e10678: f64 = (locals.var_sp_ov_delta + assign11640_e10677);
        let assign11640_e10680: f64 = (assign11640_e10678 + locals.var_sp_ov_a_d);
        let assign11640_e10681: f64 = (assign11640_e10680).sqrt();
        let assign11640_e10682: f64 = (locals.var_gov_d * assign11640_e10681);
        let assign11640_e10683: f64 = (assign11640_e10672 - assign11640_e10682);
        locals.var_sp_ov_delta1_d = assign11640_e10683;

        let assign11650_e10686: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign11650_e10686;

        let assign11660_e10689: f64 = (4.0 * 0.3333333333333333);
        let assign11660_e10692: f64 = (2.0 * 1.6021918e-19);
        let assign11660_e10694: f64 = (assign11660_e10692 * 9.1093826e-31);
        let assign11660_e10696: f64 = (assign11660_e10694 * locals.var_chib_i);
        let assign11660_e10697: f64 = (assign11660_e10696).sqrt();
        let assign11660_e10698: f64 = (assign11660_e10689 * assign11660_e10697);
        let assign11660_e10700: f64 = (assign11660_e10698 / 1.05457168e-34);
        locals.var_b_fact = assign11660_e10700;

        let assign11670_e10703: f64 = (locals.var_b_fact * locals.var_tox_i);
        locals.var_bch = assign11670_e10703;

        let assign11680_e10706: f64 = (locals.var_b_fact * locals.var_toxov_i);
        locals.var_bov = assign11680_e10706;

        let assign11690_e10709: f64 = (locals.var_b_fact * locals.var_toxovd_i);
        locals.var_bov_d = assign11690_e10709;

        locals.var_gcq = 0.0;

        let assign11710_e10713: f64 = if locals.var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign11710_e10713;

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

        locals.var_gcqov = 0.0;

        let assign11740_e10726: f64 = if locals.var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign11740_e10726;

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

        let assign11760_e10738: f64 = if locals.var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign11760_e10738;

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

        let assign11780_e10750: f64 = (locals.var_rta).powf(locals.var_stig_i);
        locals.var_tf_ig = assign11780_e10750;

        let assign11790_e10753: f64 = (locals.var_iginv_i * locals.var_tf_ig);
        locals.var_iginv_i = assign11790_e10753;

        let assign11800_e10756: f64 = (locals.var_igov_i * locals.var_tf_ig);
        locals.var_igov_i = assign11800_e10756;

        let assign11810_e10759: f64 = (locals.var_igovd_i * locals.var_tf_ig);
        locals.var_igovd_i = assign11810_e10759;

        let assign11820_e10762: f64 = (locals.var_agidl_i * 4e-18);
        let assign11820_e10765: f64 = (locals.var_toxov_i * locals.var_toxov_i);
        let assign11820_e10766: f64 = (assign11820_e10762 / assign11820_e10765);
        locals.var_agidls = assign11820_e10766;

        let assign11830_e10769: f64 = (locals.var_agidld_i * 4e-18);
        let assign11830_e10772: f64 = (locals.var_toxovd_i * locals.var_toxovd_i);
        let assign11830_e10773: f64 = (assign11830_e10769 / assign11830_e10772);
        locals.var_agidlds = assign11830_e10773;

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

        let assign11850_e10790: f64 = (locals.var_bgidl_i * locals.var_b_fact);
        locals.var_bgidl_t = assign11850_e10790;

        let assign11860_e10793: f64 = (locals.var_bgidl_t * locals.var_toxov_i);
        let assign11860_e10795: f64 = (assign11860_e10793 * 500000000.0);
        locals.var_bgidls = assign11860_e10795;

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

        let assign11880_e10812: f64 = (locals.var_bgidld_i * locals.var_b_fact);
        locals.var_bgidld_t = assign11880_e10812;

        let assign11890_e10815: f64 = (locals.var_bgidld_t * locals.var_toxovd_i);
        let assign11890_e10817: f64 = (assign11890_e10815 * 500000000.0);
        locals.var_bgidlds = assign11890_e10817;

        locals.var_vinr_max = 0.0;

        let assign11910_e10821: f64 = if locals.var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign11910_e10821;

        let (assign11920_e10827,) = {
    if (locals.var_guard163 != 0.0) {
        let assign11920_e10825: f64 = (0.75 / locals.var_fcinracc_i);
        (assign11920_e10825,)
    } else {
        (locals.var_vinr_max,)
    }
};
        locals.var_vinr_max = assign11920_e10827;

        let assign11930_e10830: f64 = (locals.var_axinr_i * locals.var_axinr_i);
        locals.var_ainr = assign11930_e10830;

        let assign11950_e10838: f64 = (9.1093826e-31 * 1000000000.0);
        let assign11950_e10840: f64 = (assign11950_e10838 * locals.var_fntexc_i);
        locals.var_fac_exc = assign11950_e10840;

        let assign11990_e10857: f64 = if locals.var_rse_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard165 = assign11990_e10857;

    }

    pub(super) fn stamp_transient_block_12(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign12000_e10863,) = {
    if (locals.var_guard165 != 0.0) {
        let assign12000_e10861: f64 = (1.0 / locals.var_rse_i);
        (assign12000_e10861,)
    } else {
        (locals.var_gsource,)
    }
};
        locals.var_gsource = assign12000_e10863;

        let (assign12010_e10868,) = {
    if (locals.var_guard165 == 0.0) {
        (0.0,)
    } else {
        (locals.var_gsource,)
    }
};
        locals.var_gsource = assign12010_e10868;

        let assign12020_e10871: f64 = if locals.var_rde_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard166 = assign12020_e10871;

        let (assign12030_e10877,) = {
    if (locals.var_guard166 != 0.0) {
        let assign12030_e10875: f64 = (1.0 / locals.var_rde_i);
        (assign12030_e10875,)
    } else {
        (locals.var_gdrain,)
    }
};
        locals.var_gdrain = assign12030_e10877;

        let (assign12040_e10882,) = {
    if (locals.var_guard166 == 0.0) {
        (0.0,)
    } else {
        (locals.var_gdrain,)
    }
};
        locals.var_gdrain = assign12040_e10882;

        locals.var_temp__blk949 = 0.0;
        locals.var_temp__blk949_dn4 = 0.0;
        locals.var_temp__blk949_dn6 = 0.0;
        locals.var_temp__blk949_dn7 = 0.0;
        locals.var_temp__blk949_dn8 = 0.0;
        locals.var_temp__blk949_dn9 = 0.0;

        locals.var_temp1 = 0.0;
        locals.var_temp1_dn4 = 0.0;
        locals.var_temp1_dn6 = 0.0;
        locals.var_temp1_dn7 = 0.0;
        locals.var_temp1_dn8 = 0.0;
        locals.var_temp1_dn9 = 0.0;

        locals.var_temp2 = 0.0;
        locals.var_temp2_dn4 = 0.0;
        locals.var_temp2_dn6 = 0.0;
        locals.var_temp2_dn7 = 0.0;
        locals.var_temp2_dn8 = 0.0;
        locals.var_temp2_dn9 = 0.0;

        let assign39430_e52953: f64 = (locals.var_tka + (nv4 - 0.0));
        locals.var_tkd = assign39430_e52953;
        locals.var_tkd_dn4 = 1.0;

        let assign39440_e52956: f64 = (locals.var_tkd * locals.var_tkd);
        locals.var_tkd_sq = assign39440_e52956;
        locals.var_tkd_sq_dn4 = ((locals.var_tkd_dn4 * locals.var_tkd) + (locals.var_tkd * locals.var_tkd_dn4));

        let assign39450_e52959: f64 = (locals.var_tkd - locals.var_tkr);
        locals.var_delt = assign39450_e52959;
        locals.var_delt_dn4 = locals.var_tkd_dn4;

        let assign39460_e52962: f64 = (locals.var_tkr / locals.var_tkd);
        locals.var_rtn = assign39460_e52962;
        locals.var_rtn_dn4 = (-((locals.var_tkr * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)));

        let assign39470_e52964: f64 = (locals.var_rtn).ln();
        locals.var_ln_rtn = assign39470_e52964;
        locals.var_ln_rtn_dn4 = (locals.var_rtn_dn4 / locals.var_rtn);

        let assign39480_e52967: f64 = (locals.var_tkd * 1.3806505e-23);
        let assign39480_e52969: f64 = (assign39480_e52967 / 1.6021918e-19);
        locals.var_phit = assign39480_e52969;
        locals.var_phit_dn4 = ((locals.var_tkd_dn4 * 1.3806505e-23) / 1.6021918e-19);

        let assign39490_e52972: f64 = (1.0 / locals.var_phit);
        locals.var_inv_phit = assign39490_e52972;
        locals.var_inv_phit_dn4 = (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit)));

        let assign39500_e52976: f64 = (9.025e-5 * locals.var_tkd);
        let assign39500_e52977: f64 = (1.179 - assign39500_e52976);
        let assign39500_e52980: f64 = (3.05e-7 * locals.var_tkd_sq);
        let assign39500_e52981: f64 = (assign39500_e52977 - assign39500_e52980);
        locals.var_eg = assign39500_e52981;
        locals.var_eg_dn4 = ((-(9.025e-5 * locals.var_tkd_dn4)) - (3.05e-7 * locals.var_tkd_sq_dn4));

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

        let (assign39520_e53006, assign39520_e53006_d_n4,) = {
    if (locals.var_phibfac > 0.001) {
        (locals.var_phibfac, locals.var_phibfac_dn4,)
    } else {
        (0.001, 0.0,)
    }
};
        locals.var_phibfac = assign39520_e53006;
        locals.var_phibfac_dn4 = assign39520_e53006_d_n4;

        let assign39530_e53009: f64 = (4.0 * 1.3806505e-23);
        let assign39530_e53011: f64 = (assign39530_e53009 * locals.var_tkd);
        locals.var_nt0 = assign39530_e53011;
        locals.var_nt0_dn4 = (assign39530_e53009 * locals.var_tkd_dn4);

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

        let (assign39550_e53034, assign39550_e53034_d_n4,) = {
    if (locals.var_phib_dc > 0.05) {
        (locals.var_phib_dc, locals.var_phib_dc_dn4,)
    } else {
        (0.05, 0.0,)
    }
};
        locals.var_phib_dc = assign39550_e53034;
        locals.var_phib_dc_dn4 = assign39550_e53034_d_n4;

        let assign39560_e53037: f64 = (2.0 * 1.6021918e-19);
        let assign39560_e53039: f64 = (assign39560_e53037 * locals.var_neff_i);
        let assign39560_e53041: f64 = (assign39560_e53039 * locals.var_epssi);
        let assign39560_e53043: f64 = (assign39560_e53041 * locals.var_inv_phit);
        let assign39560_e53044: f64 = (assign39560_e53043).sqrt();
        let assign39560_e53046: f64 = (assign39560_e53044 / locals.var_coxprime);
        locals.var_g_0_dc = assign39560_e53046;
        locals.var_g_0_dc_dn4 = (((assign39560_e53041 * locals.var_inv_phit_dn4) / (2.0 * assign39560_e53044)) / locals.var_coxprime);

        locals.var_kp = 0.0;
        locals.var_kp_dn4 = 0.0;

        locals.var_np = 0.0;

        let assign39590_e53051: f64 = if locals.var_np_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1024 = assign39590_e53051;

        let (assign39600_e53057,) = {
    if (locals.var_guard1024 != 0.0) {
        let assign39600_e53055: f64 = (80000000.0 / locals.var_tox_sq);
        (assign39600_e53055,)
    } else {
        (locals.var_arg2max,)
    }
};
        locals.var_arg2max = assign39600_e53057;

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

        let assign39640_e53094: f64 = (100.0 * locals.var_phit);
        let assign39640_e53096: f64 = (assign39640_e53094 * locals.var_phit);
        locals.var_qlim2 = assign39640_e53096;
        locals.var_qlim2_dn4 = (((100.0 * locals.var_phit_dn4) * locals.var_phit) + (assign39640_e53094 * locals.var_phit_dn4));

        let assign39650_e53099: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1025 = assign39650_e53099;

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

        let assign39700_e53142: f64 = (locals.var_phib_dc).sqrt();
        locals.var_sqrt_phib_dc = assign39700_e53142;
        locals.var_sqrt_phib_dc_dn4 = (locals.var_phib_dc_dn4 / (2.0 * assign39700_e53142));

        let assign39710_e53145: f64 = (0.95 * locals.var_phib_dc);
        locals.var_phix_dc = assign39710_e53145;
        locals.var_phix_dc_dn4 = (0.95 * locals.var_phib_dc_dn4);

        let assign39720_e53148: f64 = (0.0025 * locals.var_phib_dc);
        let assign39720_e53150: f64 = (assign39720_e53148 * locals.var_phib_dc);
        locals.var_aphi_dc = assign39720_e53150;
        locals.var_aphi_dc_dn4 = (((0.0025 * locals.var_phib_dc_dn4) * locals.var_phib_dc) + (assign39720_e53148 * locals.var_phib_dc_dn4));

        locals.var_bphi_dc = locals.var_aphi_dc;
        locals.var_bphi_dc_dn4 = locals.var_aphi_dc_dn4;

        let assign39740_e53154: f64 = (locals.var_bphi_dc).sqrt();
        let assign39740_e53155: f64 = (0.5 * assign39740_e53154);
        locals.var_phix2 = assign39740_e53155;
        locals.var_phix2_dn4 = (0.5 * (locals.var_bphi_dc_dn4 / (2.0 * assign39740_e53154)));

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

        let assign39760_e53181: f64 = (locals.var_phib_dc + locals.var_eg);
        let assign39760_e53182: f64 = (0.5 * assign39760_e53181);
        locals.var_alpha_b = assign39760_e53182;
        locals.var_alpha_b_dn4 = (0.5 * (locals.var_phib_dc_dn4 + locals.var_eg_dn4));

        let assign39770_e53185: f64 = (locals.var_vsbnud_i + locals.var_phib_dc);
        let assign39770_e53186: f64 = (assign39770_e53185).sqrt();
        let assign39770_e53188: f64 = (assign39770_e53186 - locals.var_sqrt_phib_dc);
        locals.var_us1 = assign39770_e53188;
        locals.var_us1_dn4 = ((locals.var_phib_dc_dn4 / (2.0 * assign39770_e53186)) - locals.var_sqrt_phib_dc_dn4);

        let assign39780_e53191: f64 = (locals.var_vsbnud_i + locals.var_dvsbnud_i);
        let assign39780_e53193: f64 = (assign39780_e53191 + locals.var_phib_dc);
        let assign39780_e53194: f64 = (assign39780_e53193).sqrt();
        let assign39780_e53196: f64 = (assign39780_e53194 - locals.var_sqrt_phib_dc);
        let assign39780_e53198: f64 = (assign39780_e53196 - locals.var_us1);
        locals.var_us21 = assign39780_e53198;
        locals.var_us21_dn4 = (((locals.var_phib_dc_dn4 / (2.0 * assign39780_e53194)) - locals.var_sqrt_phib_dc_dn4) - locals.var_us1_dn4);

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

        let (assign39800_e53223, assign39800_e53223_d_n4,) = {
    if (locals.var_phib_ac > 0.05) {
        (locals.var_phib_ac, locals.var_phib_ac_dn4,)
    } else {
        (0.05, 0.0,)
    }
};
        locals.var_phib_ac = assign39800_e53223;
        locals.var_phib_ac_dn4 = assign39800_e53223_d_n4;

        let assign39810_e53226: f64 = (2.0 * 1.6021918e-19);
        let assign39810_e53228: f64 = (assign39810_e53226 * locals.var_neffac_i);
        let assign39810_e53230: f64 = (assign39810_e53228 * locals.var_epssi);
        let assign39810_e53232: f64 = (assign39810_e53230 * locals.var_inv_phit);
        let assign39810_e53233: f64 = (assign39810_e53232).sqrt();
        let assign39810_e53235: f64 = (assign39810_e53233 / locals.var_coxprime);
        locals.var_g_0_ac = assign39810_e53235;
        locals.var_g_0_ac_dn4 = (((assign39810_e53230 * locals.var_inv_phit_dn4) / (2.0 * assign39810_e53233)) / locals.var_coxprime);

        let assign39820_e53238: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1026 = assign39820_e53238;

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

        let assign39870_e53282: f64 = (0.95 * locals.var_phib_ac);
        locals.var_phix_ac = assign39870_e53282;
        locals.var_phix_ac_dn4 = (0.95 * locals.var_phib_ac_dn4);

        let assign39880_e53285: f64 = (0.0025 * locals.var_phib_ac);
        let assign39880_e53287: f64 = (assign39880_e53285 * locals.var_phib_ac);
        locals.var_aphi_ac = assign39880_e53287;
        locals.var_aphi_ac_dn4 = (((0.0025 * locals.var_phib_ac_dn4) * locals.var_phib_ac) + (assign39880_e53285 * locals.var_phib_ac_dn4));

        locals.var_bphi_ac = locals.var_aphi_ac;
        locals.var_bphi_ac_dn4 = locals.var_aphi_ac_dn4;

        let assign39900_e53291: f64 = (locals.var_bphi_ac).sqrt();
        let assign39900_e53292: f64 = (0.5 * assign39900_e53291);
        locals.var_phix2 = assign39900_e53292;
        locals.var_phix2_dn4 = (0.5 * (locals.var_bphi_ac_dn4 / (2.0 * assign39900_e53291)));

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

        let assign39920_e53318: f64 = (locals.var_stvfb_i * locals.var_delt);
        let assign39920_e53322: f64 = (locals.var_st2vfb_i * locals.var_delt);
        let assign39920_e53323: f64 = (1.0 + assign39920_e53322);
        let assign39920_e53324: f64 = (assign39920_e53318 * assign39920_e53323);
        let assign39920_e53325: f64 = (locals.var_vfb_i + assign39920_e53324);
        let assign39920_e53327: f64 = (assign39920_e53325 + locals.var_delvto_i);
        locals.var_vfb_t = assign39920_e53327;
        locals.var_vfb_t_dn4 = (((locals.var_stvfb_i * locals.var_delt_dn4) * assign39920_e53323) + (assign39920_e53318 * (locals.var_st2vfb_i * locals.var_delt_dn4)));

        let assign39930_e53330: f64 = (locals.var_stct_i * locals.var_ln_rtn);
        let assign39930_e53331: f64 = (assign39930_e53330).exp();
        locals.var_tf_ct = assign39930_e53331;
        locals.var_tf_ct_dn4 = (assign39930_e53331 * (locals.var_stct_i * locals.var_ln_rtn_dn4));

        let assign39940_e53334: f64 = (locals.var_ct_i * locals.var_tf_ct);
        locals.var_ct_t = assign39940_e53334;
        locals.var_ct_t_dn4 = (locals.var_ct_i * locals.var_tf_ct_dn4);

        let assign39950_e53337: f64 = (locals.var_ctg_i / locals.var_rtn);
        locals.var_ctg_t = assign39950_e53337;
        locals.var_ctg_t_dn4 = (-((locals.var_ctg_i * locals.var_rtn_dn4) / (locals.var_rtn * locals.var_rtn)));

        let assign39960_e53340: f64 = (locals.var_stbet_i * locals.var_ln_rtn);
        let assign39960_e53341: f64 = (assign39960_e53340).exp();
        locals.var_tf_bet = assign39960_e53341;
        locals.var_tf_bet_dn4 = (assign39960_e53341 * (locals.var_stbet_i * locals.var_ln_rtn_dn4));

        let assign39970_e53344: f64 = (locals.var_betn_i * locals.var_tf_bet);
        locals.var_betn_t = assign39970_e53344;
        locals.var_betn_t_dn4 = (locals.var_betn_i * locals.var_tf_bet_dn4);

        let assign39980_e53347: f64 = (locals.var_factuo_i * locals.var_betn_t);
        let assign39980_e53349: f64 = (assign39980_e53347 * locals.var_coxprime);
        locals.var_bet_i = assign39980_e53349;
        locals.var_bet_i_dn4 = ((locals.var_factuo_i * locals.var_betn_t_dn4) * locals.var_coxprime);

        let assign39990_e53353: f64 = (locals.var_stthemu_i * locals.var_ln_rtn);
        let assign39990_e53354: f64 = (assign39990_e53353).exp();
        let assign39990_e53355: f64 = (locals.var_themu_i * assign39990_e53354);
        locals.var_themu_t = assign39990_e53355;
        locals.var_themu_t_dn4 = (locals.var_themu_i * (assign39990_e53354 * (locals.var_stthemu_i * locals.var_ln_rtn_dn4)));

        let assign40000_e53358: f64 = (locals.var_stmue_i * locals.var_ln_rtn);
        let assign40000_e53359: f64 = (assign40000_e53358).exp();
        locals.var_tf_mue = assign40000_e53359;
        locals.var_tf_mue_dn4 = (assign40000_e53359 * (locals.var_stmue_i * locals.var_ln_rtn_dn4));

        let assign40010_e53362: f64 = (locals.var_mue_i * locals.var_tf_mue);
        locals.var_mue_t = assign40010_e53362;
        locals.var_mue_t_dn4 = (locals.var_mue_i * locals.var_tf_mue_dn4);

    }

    pub(super) fn stamp_transient_block_13(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let assign40020_e53366: f64 = (locals.var_stthecs_i * locals.var_ln_rtn);
        let assign40020_e53367: f64 = (assign40020_e53366).exp();
        let assign40020_e53368: f64 = (locals.var_thecs_i * assign40020_e53367);
        locals.var_thecs_t = assign40020_e53368;
        locals.var_thecs_t_dn4 = (locals.var_thecs_i * (assign40020_e53367 * (locals.var_stthecs_i * locals.var_ln_rtn_dn4)));

        let assign40030_e53371: f64 = (locals.var_stcs_i * locals.var_ln_rtn);
        let assign40030_e53372: f64 = (assign40030_e53371).exp();
        locals.var_tf_cs = assign40030_e53372;
        locals.var_tf_cs_dn4 = (assign40030_e53372 * (locals.var_stcs_i * locals.var_ln_rtn_dn4));

        let assign40040_e53375: f64 = (locals.var_cs_i * locals.var_tf_cs);
        locals.var_cs_t = assign40040_e53375;
        locals.var_cs_t_dn4 = (locals.var_cs_i * locals.var_tf_cs_dn4);

        let assign40050_e53378: f64 = (locals.var_stxcor_i * locals.var_ln_rtn);
        let assign40050_e53379: f64 = (assign40050_e53378).exp();
        locals.var_tf_xcor = assign40050_e53379;
        locals.var_tf_xcor_dn4 = (assign40050_e53379 * (locals.var_stxcor_i * locals.var_ln_rtn_dn4));

        let assign40060_e53382: f64 = (locals.var_xcor_i * locals.var_tf_xcor);
        locals.var_xcor_t = assign40060_e53382;
        locals.var_xcor_t_dn4 = (locals.var_xcor_i * locals.var_tf_xcor_dn4);

        let assign40070_e53385: f64 = (locals.var_strs_i * locals.var_ln_rtn);
        let assign40070_e53386: f64 = (assign40070_e53385).exp();
        locals.var_tf_ther = assign40070_e53386;
        locals.var_tf_ther_dn4 = (assign40070_e53386 * (locals.var_strs_i * locals.var_ln_rtn_dn4));

        let assign40080_e53389: f64 = (locals.var_rs_i * locals.var_tf_ther);
        locals.var_rs_t = assign40080_e53389;
        locals.var_rs_t_dn4 = (locals.var_rs_i * locals.var_tf_ther_dn4);

        let assign40090_e53392: f64 = (2.0 * locals.var_bet_i);
        let assign40090_e53394: f64 = (assign40090_e53392 * locals.var_rs_t);
        locals.var_ther_i = assign40090_e53394;
        locals.var_ther_i_dn4 = (((2.0 * locals.var_bet_i_dn4) * locals.var_rs_t) + (assign40090_e53392 * locals.var_rs_t_dn4));

        let assign40100_e53397: f64 = (locals.var_stthesat_i * locals.var_ln_rtn);
        let assign40100_e53398: f64 = (assign40100_e53397).exp();
        locals.var_tf_thesat = assign40100_e53398;
        locals.var_tf_thesat_dn4 = (assign40100_e53398 * (locals.var_stthesat_i * locals.var_ln_rtn_dn4));

        let assign40110_e53401: f64 = (locals.var_thesat_i * locals.var_tf_thesat);
        locals.var_thesat_t = assign40110_e53401;
        locals.var_thesat_t_dn4 = (locals.var_thesat_i * locals.var_tf_thesat_dn4);

        let assign40120_e53404: f64 = (locals.var_thesatac_i * locals.var_tf_thesat);
        locals.var_thesatac_t = assign40120_e53404;
        locals.var_thesatac_t_dn4 = (locals.var_thesatac_i * locals.var_tf_thesat_dn4);

        let assign40130_e53407: f64 = (-locals.var_sta2_i);
        let assign40130_e53409: f64 = (assign40130_e53407 * locals.var_ln_rtn);
        let assign40130_e53410: f64 = (assign40130_e53409).exp();
        let assign40130_e53411: f64 = (locals.var_a2_i * assign40130_e53410);
        locals.var_a2_t = assign40130_e53411;
        locals.var_a2_t_dn4 = (locals.var_a2_i * (assign40130_e53410 * (assign40130_e53407 * locals.var_ln_rtn_dn4)));

        let assign40140_e53414: f64 = (locals.var_fnt_i * 4.0);
        let assign40140_e53416: f64 = (assign40140_e53414 * 1.3806505e-23);
        let assign40140_e53418: f64 = (assign40140_e53416 * locals.var_tkd);
        locals.var_nt = assign40140_e53418;
        locals.var_nt_dn4 = (assign40140_e53416 * locals.var_tkd_dn4);

        let assign40160_e53432: f64 = if ((p.p46 != 0.0) && (locals.var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1027 = assign40160_e53432;

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

        let (assign40290_e53546, assign40290_e53546_d_n4,) = {
    if (locals.var_guard1027 != 0.0) {
        (locals.var_aphiedge, locals.var_aphiedge_dn4,)
    } else {
        (locals.var_bphiedge, locals.var_bphiedge_dn4,)
    }
};
        locals.var_bphiedge = assign40290_e53546;
        locals.var_bphiedge_dn4 = assign40290_e53546_d_n4;

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

        let (assign40340_e53603, assign40340_e53603_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vfbedge_t, locals.var_vfbedge_t_dn4,)
    }
};
        locals.var_vfbedge_t = assign40340_e53603;
        locals.var_vfbedge_t_dn4 = assign40340_e53603_d_n4;

        let (assign40350_e53608, assign40350_e53608_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (locals.var_tf_betedge, locals.var_tf_betedge_dn4,)
    }
};
        locals.var_tf_betedge = assign40350_e53608;
        locals.var_tf_betedge_dn4 = assign40350_e53608_d_n4;

        let (assign40360_e53613, assign40360_e53613_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_betnedge_t, locals.var_betnedge_t_dn4,)
    }
};
        locals.var_betnedge_t = assign40360_e53613;
        locals.var_betnedge_t_dn4 = assign40360_e53613_d_n4;

        let (assign40370_e53618, assign40370_e53618_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_betedge_i, locals.var_betedge_i_dn4,)
    }
};
        locals.var_betedge_i = assign40370_e53618;
        locals.var_betedge_i_dn4 = assign40370_e53618_d_n4;

        let (assign40380_e53623, assign40380_e53623_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_phit0edge, locals.var_phit0edge_dn4,)
    }
};
        locals.var_phit0edge = assign40380_e53623;
        locals.var_phit0edge_dn4 = assign40380_e53623_d_n4;

        let (assign40390_e53628, assign40390_e53628_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_phibedge, locals.var_phibedge_dn4,)
    }
};
        locals.var_phibedge = assign40390_e53628;
        locals.var_phibedge_dn4 = assign40390_e53628_d_n4;

        let (assign40400_e53633, assign40400_e53633_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (locals.var_gfedge, locals.var_gfedge_dn4,)
    }
};
        locals.var_gfedge = assign40400_e53633;
        locals.var_gfedge_dn4 = assign40400_e53633_d_n4;

        let (assign40410_e53638, assign40410_e53638_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (locals.var_gfedge2, locals.var_gfedge2_dn4,)
    }
};
        locals.var_gfedge2 = assign40410_e53638;
        locals.var_gfedge2_dn4 = assign40410_e53638_d_n4;

        let (assign40420_e53643, assign40420_e53643_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_lngfedge2, locals.var_lngfedge2_dn4,)
    }
};
        locals.var_lngfedge2 = assign40420_e53643;
        locals.var_lngfedge2_dn4 = assign40420_e53643_d_n4;

        let (assign40430_e53648, assign40430_e53648_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_phixedge, locals.var_phixedge_dn4,)
    }
};
        locals.var_phixedge = assign40430_e53648;
        locals.var_phixedge_dn4 = assign40430_e53648_d_n4;

        let (assign40440_e53653, assign40440_e53653_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_aphiedge, locals.var_aphiedge_dn4,)
    }
};
        locals.var_aphiedge = assign40440_e53653;
        locals.var_aphiedge_dn4 = assign40440_e53653_d_n4;

        let (assign40450_e53658, assign40450_e53658_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_bphiedge, locals.var_bphiedge_dn4,)
    }
};
        locals.var_bphiedge = assign40450_e53658;
        locals.var_bphiedge_dn4 = assign40450_e53658_d_n4;

        let (assign40460_e53663, assign40460_e53663_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_phix2edge, locals.var_phix2edge_dn4,)
    }
};
        locals.var_phix2edge = assign40460_e53663;
        locals.var_phix2edge_dn4 = assign40460_e53663_d_n4;

        let (assign40470_e53668, assign40470_e53668_d_n4,) = {
    if (locals.var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_phix1edge, locals.var_phix1edge_dn4,)
    }
};
        locals.var_phix1edge = assign40470_e53668;
        locals.var_phix1edge_dn4 = assign40470_e53668_d_n4;

        let assign40500_e53681: f64 = 1.0;
        let assign40500_e53682: f64 = if locals.var_chnl_type == assign40500_e53681 { 1.0 } else { 0.0 };
        locals.var_guard1028 = assign40500_e53682;

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

        let assign40610_e53735: f64 = (locals.var_v_gs + locals.var_v_sb);
        locals.var_vgb = assign40610_e53735;
        locals.var_vgb_dn6 = locals.var_v_gs_dn6;
        locals.var_vgb_dn7 = (locals.var_v_gs_dn7 + locals.var_v_sb_dn7);
        locals.var_vgb_dn8 = (locals.var_v_gs_dn8 + locals.var_v_sb_dn8);
        locals.var_vgb_dn9 = locals.var_v_sb_dn9;

        locals.var_vgsprime = locals.var_v_gs;
        locals.var_vgsprime_dn6 = locals.var_v_gs_dn6;
        locals.var_vgsprime_dn7 = locals.var_v_gs_dn7;
        locals.var_vgsprime_dn8 = locals.var_v_gs_dn8;

        locals.var_vsbprime = locals.var_v_sb;
        locals.var_vsbprime_dn7 = locals.var_v_sb_dn7;
        locals.var_vsbprime_dn8 = locals.var_v_sb_dn8;
        locals.var_vsbprime_dn9 = locals.var_v_sb_dn9;

        let assign40640_e53740: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_vdbprime = assign40640_e53740;
        locals.var_vdbprime_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_vdbprime_dn8 = (locals.var_v_ds_dn8 + locals.var_v_sb_dn8);
        locals.var_vdbprime_dn9 = locals.var_v_sb_dn9;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign40650_e53743: f64 = (locals.var_v_gs - locals.var_v_ds);
        locals.var_vgdprime = assign40650_e53743;
        locals.var_vgdprime_dn6 = locals.var_v_gs_dn6;
        locals.var_vgdprime_dn7 = (locals.var_v_gs_dn7 - locals.var_v_ds_dn7);
        locals.var_vgdprime_dn8 = (locals.var_v_gs_dn8 - locals.var_v_ds_dn8);

        let assign40660_e53745: f64 = (-locals.var_vgsprime);
        let assign40660_e53747: f64 = (assign40660_e53745 * locals.var_inv_phita);
        locals.var_xgs_ov = assign40660_e53747;
        locals.var_xgs_ov_dn6 = ((-locals.var_vgsprime_dn6) * locals.var_inv_phita);
        locals.var_xgs_ov_dn7 = ((-locals.var_vgsprime_dn7) * locals.var_inv_phita);
        locals.var_xgs_ov_dn8 = ((-locals.var_vgsprime_dn8) * locals.var_inv_phita);

        let assign40670_e53749: f64 = (-locals.var_vgdprime);
        let assign40670_e53751: f64 = (assign40670_e53749 * locals.var_inv_phita);
        locals.var_xgd_ov = assign40670_e53751;
        locals.var_xgd_ov_dn6 = ((-locals.var_vgdprime_dn6) * locals.var_inv_phita);
        locals.var_xgd_ov_dn7 = ((-locals.var_vgdprime_dn7) * locals.var_inv_phita);
        locals.var_xgd_ov_dn8 = ((-locals.var_vgdprime_dn8) * locals.var_inv_phita);

        let assign40680_e53754: f64 = (locals.var_vgb - locals.var_vfb_t);
        let assign40680_e53755: f64 = (-assign40680_e53754);
        let assign40680_e53757: f64 = (assign40680_e53755 * locals.var_inv_phita);
        locals.var_xgb_ov = assign40680_e53757;
        locals.var_xgb_ov_dn4 = ((-(-locals.var_vfb_t_dn4)) * locals.var_inv_phita);
        locals.var_xgb_ov_dn6 = ((-locals.var_vgb_dn6) * locals.var_inv_phita);
        locals.var_xgb_ov_dn7 = ((-locals.var_vgb_dn7) * locals.var_inv_phita);
        locals.var_xgb_ov_dn8 = ((-locals.var_vgb_dn8) * locals.var_inv_phita);
        locals.var_xgb_ov_dn9 = ((-locals.var_vgb_dn9) * locals.var_inv_phita);

        locals.var_sigvds = 1.0;

        let assign40700_e53761: f64 = if locals.var_v_ds < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1029 = assign40700_e53761;

        let (assign40710_e53766,) = {
    if (locals.var_guard1029 != 0.0) {
        let assign40710_e53764: f64 = (-1.0);
        (assign40710_e53764,)
    } else {
        (locals.var_sigvds,)
    }
};
        locals.var_sigvds = assign40710_e53766;

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

        let assign40750_e53786: f64 = (locals.var_v_ds + locals.var_v_sb);
        locals.var_v_db = assign40750_e53786;
        locals.var_v_db_dn7 = (locals.var_v_ds_dn7 + locals.var_v_sb_dn7);
        locals.var_v_db_dn8 = (locals.var_v_ds_dn8 + locals.var_v_sb_dn8);
        locals.var_v_db_dn9 = locals.var_v_sb_dn9;

        let assign40760_e53789: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40760_e53792: f64 = (locals.var_v_ds * locals.var_v_ds);
        let assign40760_e53794: f64 = (assign40760_e53792 + 0.01);
        let assign40760_e53795: f64 = (assign40760_e53794).sqrt();
        let assign40760_e53797: f64 = (assign40760_e53795 + 0.1);
        let assign40760_e53798: f64 = (assign40760_e53789 / assign40760_e53797);
        locals.var_vdsx = assign40760_e53798;
        locals.var_vdsx_dn7 = (((((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) * assign40760_e53797) - (assign40760_e53789 * (((locals.var_v_ds_dn7 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn7)) / (2.0 * assign40760_e53795)))) / (assign40760_e53797 * assign40760_e53797));
        locals.var_vdsx_dn8 = (((((locals.var_v_ds_dn8 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn8)) * assign40760_e53797) - (assign40760_e53789 * (((locals.var_v_ds_dn8 * locals.var_v_ds) + (locals.var_v_ds * locals.var_v_ds_dn8)) / (2.0 * assign40760_e53795)))) / (assign40760_e53797 * assign40760_e53797));

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

        locals.var_v_xb_dc_tmp = locals.var_v_xb;
        locals.var_v_xb_dc_tmp_dn4 = locals.var_v_xb_dn4;
        locals.var_v_xb_dc_tmp_dn7 = locals.var_v_xb_dn7;
        locals.var_v_xb_dc_tmp_dn8 = locals.var_v_xb_dn8;
        locals.var_v_xb_dc_tmp_dn9 = locals.var_v_xb_dn9;

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

        locals.var_vsbstar_dc_tmp = locals.var_vsbstar_dc;
        locals.var_vsbstar_dc_tmp_dn4 = locals.var_vsbstar_dc_dn4;
        locals.var_vsbstar_dc_tmp_dn6 = locals.var_vsbstar_dc_dn6;
        locals.var_vsbstar_dc_tmp_dn7 = locals.var_vsbstar_dc_dn7;
        locals.var_vsbstar_dc_tmp_dn8 = locals.var_vsbstar_dc_dn8;
        locals.var_vsbstar_dc_tmp_dn9 = locals.var_vsbstar_dc_dn9;

        locals.var_dvbstar_dc = 0.0;
        locals.var_dvbstar_dc_dn4 = 0.0;
        locals.var_dvbstar_dc_dn6 = 0.0;
        locals.var_dvbstar_dc_dn7 = 0.0;
        locals.var_dvbstar_dc_dn8 = 0.0;
        locals.var_dvbstar_dc_dn9 = 0.0;

        let assign40820_e53846: f64 = if ((p.p45 != 0.0) && (locals.var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard1189 = assign40820_e53846;

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

        locals.var_phib = locals.var_phib_dc;
        locals.var_phib_dn4 = locals.var_phib_dc_dn4;

        locals.var_aphi = locals.var_aphi_dc;
        locals.var_aphi_dn4 = locals.var_aphi_dc_dn4;

        locals.var_g_0 = locals.var_g_0_dc;
        locals.var_g_0_dn4 = locals.var_g_0_dc_dn4;

        locals.var_vsbstar = locals.var_vsbstar_dc;
        locals.var_vsbstar_dn4 = locals.var_vsbstar_dc_dn4;
        locals.var_vsbstar_dn6 = locals.var_vsbstar_dc_dn6;
        locals.var_vsbstar_dn7 = locals.var_vsbstar_dc_dn7;
        locals.var_vsbstar_dn8 = locals.var_vsbstar_dc_dn8;
        locals.var_vsbstar_dn9 = locals.var_vsbstar_dc_dn9;

        locals.var_dvbstar = locals.var_dvbstar_dc;
        locals.var_dvbstar_dn4 = locals.var_dvbstar_dc_dn4;
        locals.var_dvbstar_dn6 = locals.var_dvbstar_dc_dn6;
        locals.var_dvbstar_dn7 = locals.var_dvbstar_dc_dn7;
        locals.var_dvbstar_dn8 = locals.var_dvbstar_dc_dn8;
        locals.var_dvbstar_dn9 = locals.var_dvbstar_dc_dn9;

        locals.var_thesatloc = locals.var_thesat_t;
        locals.var_thesatloc_dn4 = locals.var_thesat_t_dn4;

        locals.var_arloc = locals.var_ar;

        let assign40970_e53936: f64 = (locals.var_vgb - locals.var_dvbstar);
        let assign40970_e53938: f64 = (assign40970_e53936 - locals.var_vfb_t);
        locals.var_vgb1 = assign40970_e53938;
        locals.var_vgb1_dn4 = ((-locals.var_dvbstar_dn4) - locals.var_vfb_t_dn4);
        locals.var_vgb1_dn6 = (locals.var_vgb_dn6 - locals.var_dvbstar_dn6);
        locals.var_vgb1_dn7 = (locals.var_vgb_dn7 - locals.var_dvbstar_dn7);
        locals.var_vgb1_dn8 = (locals.var_vgb_dn8 - locals.var_dvbstar_dn8);
        locals.var_vgb1_dn9 = (locals.var_vgb_dn9 - locals.var_dvbstar_dn9);

        let assign40980_e53943: f64 = (locals.var_v_ds - locals.var_vdsx);
        let assign40980_e53944: f64 = (0.5 * assign40980_e53943);
        let assign40980_e53945: f64 = (locals.var_vsbstar + assign40980_e53944);
        locals.var_vsbx = assign40980_e53945;
        locals.var_vsbx_dn4 = locals.var_vsbstar_dn4;
        locals.var_vsbx_dn6 = locals.var_vsbstar_dn6;
        locals.var_vsbx_dn7 = (locals.var_vsbstar_dn7 + (0.5 * (locals.var_v_ds_dn7 - locals.var_vdsx_dn7)));
        locals.var_vsbx_dn8 = (locals.var_vsbstar_dn8 + (0.5 * (locals.var_v_ds_dn8 - locals.var_vdsx_dn8)));
        locals.var_vsbx_dn9 = locals.var_vsbstar_dn9;

        locals.var_dctg = 1.0;
        locals.var_dctg_dn4 = 0.0;
        locals.var_dctg_dn6 = 0.0;
        locals.var_dctg_dn7 = 0.0;
        locals.var_dctg_dn8 = 0.0;
        locals.var_dctg_dn9 = 0.0;

        let assign41000_e53949: f64 = if locals.var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1190 = assign41000_e53949;

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

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign41170_e54150: f64 = (-230.25850929940458);
        let assign41170_e54151: f64 = if locals.var_temp2 > assign41170_e54150 { 1.0 } else { 0.0 };
        locals.var_guard1191 = assign41170_e54151;

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

        let assign41200_e54194: f64 = (locals.var_ct_t * locals.var_dctg);
        let assign41200_e54195: f64 = (1.0 + assign41200_e54194);
        locals.var_ct_fact = assign41200_e54195;
        locals.var_ct_fact_dn4 = ((locals.var_ct_t_dn4 * locals.var_dctg) + (locals.var_ct_t * locals.var_dctg_dn4));
        locals.var_ct_fact_dn6 = (locals.var_ct_t * locals.var_dctg_dn6);
        locals.var_ct_fact_dn7 = (locals.var_ct_t * locals.var_dctg_dn7);
        locals.var_ct_fact_dn8 = (locals.var_ct_t * locals.var_dctg_dn8);
        locals.var_ct_fact_dn9 = (locals.var_ct_t * locals.var_dctg_dn9);

        let assign41210_e54198: f64 = (locals.var_phit * locals.var_ct_fact);
        locals.var_phitct = assign41210_e54198;
        locals.var_phitct_dn4 = ((locals.var_phit_dn4 * locals.var_ct_fact) + (locals.var_phit * locals.var_ct_fact_dn4));
        locals.var_phitct_dn6 = (locals.var_phit * locals.var_ct_fact_dn6);
        locals.var_phitct_dn7 = (locals.var_phit * locals.var_ct_fact_dn7);
        locals.var_phitct_dn8 = (locals.var_phit * locals.var_ct_fact_dn8);
        locals.var_phitct_dn9 = (locals.var_phit * locals.var_ct_fact_dn9);

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

        let assign41230_e54215: f64 = (1.0 + locals.var_dphit1);
        let assign41230_e54216: f64 = (locals.var_phitct * assign41230_e54215);
        locals.var_phit1 = assign41230_e54216;
        locals.var_phit1_dn4 = ((locals.var_phitct_dn4 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn4));
        locals.var_phit1_dn6 = ((locals.var_phitct_dn6 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn6));
        locals.var_phit1_dn7 = ((locals.var_phitct_dn7 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn7));
        locals.var_phit1_dn8 = ((locals.var_phitct_dn8 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn8));
        locals.var_phit1_dn9 = ((locals.var_phitct_dn9 * assign41230_e54215) + (locals.var_phitct * locals.var_dphit1_dn9));

        let assign41240_e54219: f64 = (1.0 / locals.var_phit1);
        locals.var_inv_phit1 = assign41240_e54219;
        locals.var_inv_phit1_dn4 = (-(locals.var_phit1_dn4 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn6 = (-(locals.var_phit1_dn6 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn7 = (-(locals.var_phit1_dn7 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn8 = (-(locals.var_phit1_dn8 / (locals.var_phit1 * locals.var_phit1)));
        locals.var_inv_phit1_dn9 = (-(locals.var_phit1_dn9 / (locals.var_phit1 * locals.var_phit1)));

        let assign41250_e54223: f64 = (locals.var_phit * locals.var_inv_phit1);
        let assign41250_e54224: f64 = (assign41250_e54223).sqrt();
        let assign41250_e54225: f64 = (locals.var_g_0 * assign41250_e54224);
        locals.var_gf = assign41250_e54225;
        locals.var_gf_dn4 = ((locals.var_g_0_dn4 * assign41250_e54224) + (locals.var_g_0 * (((locals.var_phit_dn4 * locals.var_inv_phit1) + (locals.var_phit * locals.var_inv_phit1_dn4)) / (2.0 * assign41250_e54224))));
        locals.var_gf_dn6 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn6) / (2.0 * assign41250_e54224)));
        locals.var_gf_dn7 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn7) / (2.0 * assign41250_e54224)));
        locals.var_gf_dn8 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn8) / (2.0 * assign41250_e54224)));
        locals.var_gf_dn9 = (locals.var_g_0 * ((locals.var_phit * locals.var_inv_phit1_dn9) / (2.0 * assign41250_e54224)));

        let assign41260_e54228: f64 = (locals.var_gf * locals.var_gf);
        locals.var_gf2 = assign41260_e54228;
        locals.var_gf2_dn4 = ((locals.var_gf_dn4 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn4));
        locals.var_gf2_dn6 = ((locals.var_gf_dn6 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn6));
        locals.var_gf2_dn7 = ((locals.var_gf_dn7 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn7));
        locals.var_gf2_dn8 = ((locals.var_gf_dn8 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn8));
        locals.var_gf2_dn9 = ((locals.var_gf_dn9 * locals.var_gf) + (locals.var_gf * locals.var_gf_dn9));

        let assign41270_e54231: f64 = (1.0 / locals.var_gf2);
        locals.var_inv_gf2 = assign41270_e54231;
        locals.var_inv_gf2_dn4 = (-(locals.var_gf2_dn4 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn6 = (-(locals.var_gf2_dn6 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn7 = (-(locals.var_gf2_dn7 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn8 = (-(locals.var_gf2_dn8 / (locals.var_gf2 * locals.var_gf2)));
        locals.var_inv_gf2_dn9 = (-(locals.var_gf2_dn9 / (locals.var_gf2 * locals.var_gf2)));

        let assign41280_e54234: f64 = (locals.var_vsbstar * locals.var_inv_phit1);
        locals.var_ux = assign41280_e54234;
        locals.var_ux_dn4 = ((locals.var_vsbstar_dn4 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn4));
        locals.var_ux_dn6 = ((locals.var_vsbstar_dn6 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn6));
        locals.var_ux_dn7 = ((locals.var_vsbstar_dn7 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn7));
        locals.var_ux_dn8 = ((locals.var_vsbstar_dn8 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn8));
        locals.var_ux_dn9 = ((locals.var_vsbstar_dn9 * locals.var_inv_phit1) + (locals.var_vsbstar * locals.var_inv_phit1_dn9));

        let assign41290_e54237: f64 = (locals.var_vgb1 * locals.var_inv_phit1);
        locals.var_xg = assign41290_e54237;
        locals.var_xg_dn4 = ((locals.var_vgb1_dn4 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn4));
        locals.var_xg_dn6 = ((locals.var_vgb1_dn6 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn6));
        locals.var_xg_dn7 = ((locals.var_vgb1_dn7 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn7));
        locals.var_xg_dn8 = ((locals.var_vgb1_dn8 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn8));
        locals.var_xg_dn9 = ((locals.var_vgb1_dn9 * locals.var_inv_phit1) + (locals.var_vgb1 * locals.var_inv_phit1_dn9));

        let assign41300_e54240: f64 = (2.0 * locals.var_vdsx);
        let assign41300_e54245: f64 = (locals.var_cfd_i * locals.var_vdsx);
        let assign41300_e54246: f64 = (1.0 + assign41300_e54245);
        let assign41300_e54247: f64 = (assign41300_e54246).sqrt();
        let assign41300_e54248: f64 = (1.0 + assign41300_e54247);
        let assign41300_e54249: f64 = (assign41300_e54240 / assign41300_e54248);
        locals.var_vdsp = assign41300_e54249;
        locals.var_vdsp_dn7 = ((((2.0 * locals.var_vdsx_dn7) * assign41300_e54248) - (assign41300_e54240 * ((locals.var_cfd_i * locals.var_vdsx_dn7) / (2.0 * assign41300_e54247)))) / (assign41300_e54248 * assign41300_e54248));
        locals.var_vdsp_dn8 = ((((2.0 * locals.var_vdsx_dn8) * assign41300_e54248) - (assign41300_e54240 * ((locals.var_cfd_i * locals.var_vdsx_dn8) / (2.0 * assign41300_e54247)))) / (assign41300_e54248 * assign41300_e54248));

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

        let assign41320_e54261: f64 = (locals.var_phib * locals.var_inv_phit1);
        locals.var_xb = assign41320_e54261;
        locals.var_xb_dn4 = ((locals.var_phib_dn4 * locals.var_inv_phit1) + (locals.var_phib * locals.var_inv_phit1_dn4));
        locals.var_xb_dn6 = (locals.var_phib * locals.var_inv_phit1_dn6);
        locals.var_xb_dn7 = (locals.var_phib * locals.var_inv_phit1_dn7);
        locals.var_xb_dn8 = (locals.var_phib * locals.var_inv_phit1_dn8);
        locals.var_xb_dn9 = (locals.var_phib * locals.var_inv_phit1_dn9);

        let assign41330_e54264: f64 = (locals.var_v_xb * locals.var_v_xb);
        let assign41330_e54266: f64 = (assign41330_e54264 + locals.var_aphi);
        let assign41330_e54267: f64 = (assign41330_e54266).sqrt();
        locals.var_temp1 = assign41330_e54267;
        locals.var_temp1_dn4 = ((((locals.var_v_xb_dn4 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn4)) + locals.var_aphi_dn4) / (2.0 * assign41330_e54267));
        locals.var_temp1_dn6 = 0.0;
        locals.var_temp1_dn7 = (((locals.var_v_xb_dn7 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn7)) / (2.0 * assign41330_e54267));
        locals.var_temp1_dn8 = (((locals.var_v_xb_dn8 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn8)) / (2.0 * assign41330_e54267));
        locals.var_temp1_dn9 = (((locals.var_v_xb_dn9 * locals.var_v_xb) + (locals.var_v_xb * locals.var_v_xb_dn9)) / (2.0 * assign41330_e54267));

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

        let assign41360_e54289: f64 = (locals.var_xb + locals.var_ux);
        locals.var_xno_s = assign41360_e54289;
        locals.var_xno_s_dn4 = (locals.var_xb_dn4 + locals.var_ux_dn4);
        locals.var_xno_s_dn6 = (locals.var_xb_dn6 + locals.var_ux_dn6);
        locals.var_xno_s_dn7 = (locals.var_xb_dn7 + locals.var_ux_dn7);
        locals.var_xno_s_dn8 = (locals.var_xb_dn8 + locals.var_ux_dn8);
        locals.var_xno_s_dn9 = (locals.var_xb_dn9 + locals.var_ux_dn9);

        let assign41370_e54292: f64 = (locals.var_xno_s - locals.var_delxb);
        locals.var_xn_s = assign41370_e54292;
        locals.var_xn_s_dn4 = (locals.var_xno_s_dn4 - locals.var_delxb_dn4);
        locals.var_xn_s_dn6 = (locals.var_xno_s_dn6 - locals.var_delxb_dn6);
        locals.var_xn_s_dn7 = (locals.var_xno_s_dn7 - locals.var_delxb_dn7);
        locals.var_xn_s_dn8 = (locals.var_xno_s_dn8 - locals.var_delxb_dn8);
        locals.var_xn_s_dn9 = (locals.var_xno_s_dn9 - locals.var_delxb_dn9);

        let assign41380_e54295: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1192 = assign41380_e54295;

        let assign41390_e54297: f64 = (locals.var_xn_s).abs();
        let assign41390_e54299: f64 = if assign41390_e54297 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1193 = assign41390_e54299;

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

        let assign41410_e54322: f64 = if locals.var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        locals.var_guard1194 = assign41410_e54322;

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

        let assign41480_e54434: f64 = (locals.var_xg - locals.var_xthscr);
        let assign41480_e54436: f64 = (assign41480_e54434 / locals.var_nscr);
        locals.var_xgtscr = assign41480_e54436;
        locals.var_xgtscr_dn4 = ((((locals.var_xg_dn4 - locals.var_xthscr_dn4) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn4)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn6 = ((((locals.var_xg_dn6 - locals.var_xthscr_dn6) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn6)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn7 = ((((locals.var_xg_dn7 - locals.var_xthscr_dn7) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn7)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn8 = ((((locals.var_xg_dn8 - locals.var_xthscr_dn8) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn8)) / (locals.var_nscr * locals.var_nscr));
        locals.var_xgtscr_dn9 = ((((locals.var_xg_dn9 - locals.var_xthscr_dn9) * locals.var_nscr) - (assign41480_e54434 * locals.var_nscr_dn9)) / (locals.var_nscr * locals.var_nscr));

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

    }
}
