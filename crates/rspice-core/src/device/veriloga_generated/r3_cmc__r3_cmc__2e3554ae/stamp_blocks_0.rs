#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign30_e272: f64 = multiplicity;
        locals.var_mmod = assign30_e272;

        let assign40_e275: f64 = 0.0;
        locals.var_gmin = assign40_e275;

        let assign50_e279: f64 = (0.01 * p.p23);
        let assign50_e280: f64 = (1.0 - assign50_e279);
        let assign50_e282: f64 = (assign50_e280 * p.p22);
        let assign50_e284: f64 = (assign50_e282 * 1000000.0);
        locals.var_lfactor = assign50_e284;

        let assign60_e287: f64 = (locals.var_lfactor * locals.var_lfactor);
        locals.var_afactor = assign60_e287;

        let assign70_e290: f64 = (273.15 + p.p28);
        locals.var_tinik = assign70_e290;

        let assign90_e293: f64 = ctx_temp;
        let assign90_e295: f64 = (assign90_e293 + p.p9);
        let assign90_e297: f64 = (assign90_e295 - 273.15);
        locals.var_tdevc = assign90_e297;
        locals.var_tdevc_dn3 = 0.0;

        let assign120_e307: f64 = (p.p35 + 1.0);
        let assign120_e308: f64 = if locals.var_tdevc < assign120_e307 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign120_e308;

        let (assign130_e319, assign130_e319_d_n3,) = {
    if (locals.var_guard104 != 0.0) {
        let assign130_e313: f64 = (locals.var_tdevc - p.p35);
        let assign130_e315: f64 = (assign130_e313 - 1.0);
        let assign130_e316: f64 = (assign130_e315).exp();
        let assign130_e317: f64 = (p.p35 + assign130_e316);
        (assign130_e317, (assign130_e316 * locals.var_tdevc_dn3),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign130_e319;
        locals.var_tdevc_dn3 = assign130_e319_d_n3;

        let assign140_e323: f64 = (p.p36 - 1.0);
        let assign140_e324: f64 = if locals.var_tdevc > assign140_e323 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign140_e324;

        let (assign150_e338, assign150_e338_d_n3,) = {
    if ((locals.var_guard104 == 0.0) && (locals.var_guard105 != 0.0)) {
        let assign150_e332: f64 = (p.p36 - locals.var_tdevc);
        let assign150_e334: f64 = (assign150_e332 - 1.0);
        let assign150_e335: f64 = (assign150_e334).exp();
        let assign150_e336: f64 = (p.p36 - assign150_e335);
        (assign150_e336, (-(assign150_e335 * (-locals.var_tdevc_dn3))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign150_e338;
        locals.var_tdevc_dn3 = assign150_e338_d_n3;

        let (assign160_e346, assign160_e346_d_n3,) = {
    if ((locals.var_guard104 == 0.0) && (locals.var_guard105 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign160_e346;
        locals.var_tdevc_dn3 = assign160_e346_d_n3;

        let assign170_e349: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign170_e349;
        locals.var_tdevk_dn3 = locals.var_tdevc_dn3;

        let assign180_e352: f64 = (1.3806505e-23 * locals.var_tdevk);
        let assign180_e354: f64 = (assign180_e352 / 1.60217653e-19);
        locals.var_phi_t0 = assign180_e354;
        locals.var_phi_t0_dn3 = ((1.3806505e-23 * locals.var_tdevk_dn3) / 1.60217653e-19);

        let assign190_e357: f64 = (locals.var_tdevk / locals.var_tinik);
        locals.var_rt = assign190_e357;
        locals.var_rt_dn3 = (locals.var_tdevk_dn3 / locals.var_tinik);

        let assign200_e360: f64 = (locals.var_tdevk - locals.var_tinik);
        locals.var_dt = assign200_e360;
        locals.var_dt_dn3 = locals.var_tdevk_dn3;

        let assign210_e363: f64 = (p.p0 * locals.var_lfactor);
        locals.var_w_um = assign210_e363;

        let assign220_e366: f64 = (p.p1 * locals.var_lfactor);
        locals.var_l_um = assign220_e366;

        let assign270_e381: f64 = (p.p2 * locals.var_lfactor);
        locals.var_wd_um = assign270_e381;

        let assign280_e384: f64 = (p.p3 * locals.var_afactor);
        locals.var_a1_um2 = assign280_e384;
        locals.var_a1_um2_dn1 = (locals.var_a1_dn1 * locals.var_afactor);
        locals.var_a1_um2_dn3 = (locals.var_a1_dn3 * locals.var_afactor);
        locals.var_a1_um2_dn4 = (locals.var_a1_dn4 * locals.var_afactor);
        locals.var_a1_um2_dn5 = (locals.var_a1_dn5 * locals.var_afactor);

        let assign290_e387: f64 = (p.p4 * locals.var_lfactor);
        locals.var_p1_um = assign290_e387;

        let assign300_e390: f64 = (p.p6 * locals.var_afactor);
        locals.var_a2_um2 = assign300_e390;
        locals.var_a2_um2_dn1 = (locals.var_a2_dn1 * locals.var_afactor);
        locals.var_a2_um2_dn3 = (locals.var_a2_dn3 * locals.var_afactor);
        locals.var_a2_um2_dn4 = (locals.var_a2_dn4 * locals.var_afactor);
        locals.var_a2_um2_dn5 = (locals.var_a2_dn5 * locals.var_afactor);

        let assign310_e393: f64 = (p.p7 * locals.var_lfactor);
        locals.var_p2_um = assign310_e393;

        let assign320_e396: f64 = (locals.var_l_um * locals.var_w_um);
        locals.var_a_um2 = assign320_e396;

        let assign330_e399: f64 = (2.0 * locals.var_l_um);
        let assign330_e402: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign330_e405: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign330_e406: f64 = (assign330_e402 + assign330_e405);
        let assign330_e408: f64 = (assign330_e406 * locals.var_w_um);
        let assign330_e409: f64 = (assign330_e399 + assign330_e408);
        locals.var_p_um = assign330_e409;

        let assign340_e413: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign340_e416: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign340_e417: f64 = (assign340_e413 + assign340_e416);
        let assign340_e418: f64 = (0.5 * assign340_e417);
        let assign340_e422: f64 = (p.p44 / locals.var_w_um);
        let assign340_e423: f64 = (p.p43 + assign340_e422);
        let assign340_e424: f64 = (assign340_e418 * assign340_e423);
        locals.var_xleff = assign340_e424;

        let assign350_e427: f64 = (locals.var_w_um + p.p38);
        let assign350_e430: f64 = (p.p39 / locals.var_w_um);
        let assign350_e431: f64 = (assign350_e427 + assign350_e430);
        let assign350_e435: f64 = (-locals.var_w_um);
        let assign350_e437: f64 = (assign350_e435 / p.p41);
        let assign350_e438: f64 = (assign350_e437).exp();
        let assign350_e439: f64 = (1.0 - assign350_e438);
        let assign350_e440: f64 = (p.p42 * assign350_e439);
        let assign350_e441: f64 = (assign350_e431 + assign350_e440);
        let assign350_e445: f64 = (p.p40 * locals.var_wd_um);
        let assign350_e447: f64 = (assign350_e445 / locals.var_a_um2);
        let assign350_e448: f64 = (1.0 - assign350_e447);
        let assign350_e449: f64 = (assign350_e441 / assign350_e448);
        locals.var_weff_um = assign350_e449;

        let assign360_e452: f64 = (locals.var_l_um + locals.var_xleff);
        locals.var_leff_um = assign360_e452;

        let (assign370_e456,) = {
    if (p.p127 != 0.0) {
        (locals.var_weff_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign370_e456;

        let (assign380_e460,) = {
    if (p.p127 != 0.0) {
        (locals.var_leff_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign380_e460;

        let (assign390_e465,) = {
    if (p.p127 == 0.0) {
        (locals.var_w_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign390_e465;

        let (assign400_e470,) = {
    if (p.p127 == 0.0) {
        (locals.var_l_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign400_e470;

        let (assign410_e487,) = {
    if (p.p16 != 0.0) {
        let assign410_e475: f64 = (p.p119 * p.p122);
        let assign410_e476: f64 = (locals.var_weff_um + assign410_e475);
        let assign410_e479: f64 = (p.p11 * p.p125);
        let assign410_e482: f64 = (locals.var_mmod * locals.var_len);
        let assign410_e483: f64 = (assign410_e482).sqrt();
        let assign410_e484: f64 = (assign410_e479 / assign410_e483);
        let assign410_e485: f64 = (assign410_e476 + assign410_e484);
        (assign410_e485,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign410_e487;

        let (assign420_e504,) = {
    if (p.p16 != 0.0) {
        let assign420_e492: f64 = (p.p120 * p.p123);
        let assign420_e493: f64 = (locals.var_leff_um + assign420_e492);
        let assign420_e496: f64 = (p.p12 * p.p126);
        let assign420_e499: f64 = (locals.var_mmod * locals.var_wid);
        let assign420_e500: f64 = (assign420_e499).sqrt();
        let assign420_e501: f64 = (assign420_e496 / assign420_e500);
        let assign420_e502: f64 = (assign420_e493 + assign420_e501);
        (assign420_e502,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign420_e504;

        let (assign430_e524,) = {
    if (p.p16 != 0.0) {
        let assign430_e509: f64 = (p.p118 * p.p121);
        let assign430_e512: f64 = (p.p10 * p.p124);
        let assign430_e515: f64 = (locals.var_mmod * locals.var_len);
        let assign430_e517: f64 = (assign430_e515 * locals.var_wid);
        let assign430_e518: f64 = (assign430_e517).sqrt();
        let assign430_e519: f64 = (assign430_e512 / assign430_e518);
        let assign430_e520: f64 = (assign430_e509 + assign430_e519);
        let assign430_e521: f64 = (0.01 * assign430_e520);
        let assign430_e522: f64 = (assign430_e521).exp();
        (assign430_e522,)
    } else {
        (locals.var_delr_rsh,)
    }
};
        locals.var_delr_rsh = assign430_e524;

        let assign440_e535: f64 = if ((p.p119 != 0.0) && ((p.p125 > 0.0) || (p.p122 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign440_e535;

        let (assign450_e547,) = {
    if ((p.p16 == 0.0) && (locals.var_guard110 != 0.0)) {
        let assign450_e543: f64 = (locals.var_mmod * locals.var_len);
        let assign450_e544: f64 = (assign450_e543).sqrt();
        let assign450_e545: f64 = (p.p125 / assign450_e544);
        (assign450_e545,)
    } else {
        (locals.var_fctr1,)
    }
};
        locals.var_fctr1 = assign450_e547;

        let (assign460_e565,) = {
    if ((p.p16 == 0.0) && (locals.var_guard110 != 0.0)) {
        let assign460_e556: f64 = (p.p122 * p.p122);
        let assign460_e559: f64 = (locals.var_fctr1 * locals.var_fctr1);
        let assign460_e560: f64 = (assign460_e556 + assign460_e559);
        let assign460_e561: f64 = (assign460_e560).sqrt();
        let assign460_e562: f64 = (p.p119 * assign460_e561);
        let assign460_e563: f64 = (locals.var_weff_um + assign460_e562);
        (assign460_e563,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign460_e565;

        let assign470_e576: f64 = if ((p.p120 != 0.0) && ((p.p126 > 0.0) || (p.p123 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard111 = assign470_e576;

        let (assign480_e588,) = {
    if ((p.p16 == 0.0) && (locals.var_guard111 != 0.0)) {
        let assign480_e584: f64 = (locals.var_mmod * locals.var_wid);
        let assign480_e585: f64 = (assign480_e584).sqrt();
        let assign480_e586: f64 = (p.p126 / assign480_e585);
        (assign480_e586,)
    } else {
        (locals.var_fctr1,)
    }
};
        locals.var_fctr1 = assign480_e588;

        let (assign490_e606,) = {
    if ((p.p16 == 0.0) && (locals.var_guard111 != 0.0)) {
        let assign490_e597: f64 = (p.p123 * p.p123);
        let assign490_e600: f64 = (locals.var_fctr1 * locals.var_fctr1);
        let assign490_e601: f64 = (assign490_e597 + assign490_e600);
        let assign490_e602: f64 = (assign490_e601).sqrt();
        let assign490_e603: f64 = (p.p120 * assign490_e602);
        let assign490_e604: f64 = (locals.var_leff_um + assign490_e603);
        (assign490_e604,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign490_e606;

        let assign500_e617: f64 = if ((p.p118 != 0.0) && ((p.p124 > 0.0) || (p.p121 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard112 = assign500_e617;

        let (assign510_e631,) = {
    if ((p.p16 == 0.0) && (locals.var_guard112 != 0.0)) {
        let assign510_e625: f64 = (locals.var_mmod * locals.var_len);
        let assign510_e627: f64 = (assign510_e625 * locals.var_wid);
        let assign510_e628: f64 = (assign510_e627).sqrt();
        let assign510_e629: f64 = (p.p124 / assign510_e628);
        (assign510_e629,)
    } else {
        (locals.var_fctr1,)
    }
};
        locals.var_fctr1 = assign510_e631;

        let (assign520_e650,) = {
    if ((p.p16 == 0.0) && (locals.var_guard112 != 0.0)) {
        let assign520_e638: f64 = (0.01 * p.p118);
        let assign520_e641: f64 = (p.p121 * p.p121);
        let assign520_e644: f64 = (locals.var_fctr1 * locals.var_fctr1);
        let assign520_e645: f64 = (assign520_e641 + assign520_e644);
        let assign520_e646: f64 = (assign520_e645).sqrt();
        let assign520_e647: f64 = (assign520_e638 * assign520_e646);
        let assign520_e648: f64 = (assign520_e647).exp();
        (assign520_e648,)
    } else {
        (locals.var_delr_rsh,)
    }
};
        locals.var_delr_rsh = assign520_e650;

        let (assign530_e658,) = {
    if ((p.p16 == 0.0) && (locals.var_guard112 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_delr_rsh,)
    }
};
        locals.var_delr_rsh = assign530_e658;

        let assign560_e667: f64 = (locals.var_leff_um + p.p45);
        locals.var_leffe_um = assign560_e667;

        let (assign580_e674,) = {
    if (p.p53 != 0.0) {
        (locals.var_weff_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign580_e674;

        let (assign590_e678,) = {
    if (p.p53 != 0.0) {
        (locals.var_leff_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign590_e678;

        let (assign600_e683,) = {
    if (p.p53 == 0.0) {
        (locals.var_w_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign600_e683;

        let (assign610_e688,) = {
    if (p.p53 == 0.0) {
        (locals.var_l_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign610_e688;

        let assign620_e692: f64 = (locals.var_wid).powf(p.p56);
        let assign620_e693: f64 = (1.0 / assign620_e692);
        locals.var_iw_dpwe = assign620_e693;

        let assign630_e697: f64 = (locals.var_len).powf(p.p58);
        let assign630_e698: f64 = (1.0 / assign630_e697);
        locals.var_il_dple = assign630_e698;

        let assign640_e703: f64 = (p.p55 * locals.var_iw_dpwe);
        let assign640_e704: f64 = (1.0 + assign640_e703);
        let assign640_e705: f64 = (p.p54 * assign640_e704);
        let assign640_e709: f64 = (p.p57 * locals.var_il_dple);
        let assign640_e710: f64 = (1.0 + assign640_e709);
        let assign640_e711: f64 = (assign640_e705 * assign640_e710);
        let assign640_e715: f64 = (p.p59 * locals.var_iw_dpwe);
        let assign640_e717: f64 = (assign640_e715 * locals.var_il_dple);
        let assign640_e718: f64 = (1.0 + assign640_e717);
        let assign640_e719: f64 = (assign640_e711 * assign640_e718);
        let assign640_e725: f64 = (locals.var_dt * p.p104);
        let assign640_e726: f64 = (p.p103 + assign640_e725);
        let assign640_e727: f64 = (locals.var_dt * assign640_e726);
        let assign640_e728: f64 = (1.0 + assign640_e727);
        let assign640_e729: f64 = (assign640_e719 * assign640_e728);
        locals.var_dp_i = assign640_e729;
        locals.var_dp_i_dn3 = (assign640_e719 * ((locals.var_dt_dn3 * assign640_e726) + (locals.var_dt * (locals.var_dt_dn3 * p.p104))));

        let (assign650_e735, assign650_e735_d_n3,) = {
    if (locals.var_dp_i > 0.1) {
        (locals.var_dp_i, locals.var_dp_i_dn3,)
    } else {
        (0.1, 0.0,)
    }
};
        locals.var_dp_i = assign650_e735;
        locals.var_dp_i_dn3 = assign650_e735_d_n3;

        let assign660_e737: f64 = (locals.var_dp_i).sqrt();
        let assign660_e740: f64 = (locals.var_dp_i + 10000.0);
        let assign660_e741: f64 = (assign660_e737 / assign660_e740);
        locals.var_dfmin = assign660_e741;
        locals.var_dfmin_dn3 = ((((locals.var_dp_i_dn3 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * locals.var_dp_i_dn3)) / (assign660_e740 * assign660_e740));

        let (assign670_e759,) = {
    if (p.p15 != 0.0) {
        (0.0,)
    } else {
        let assign670_e747: f64 = (p.p50 * locals.var_len);
        let assign670_e750: f64 = (p.p51 * locals.var_wid);
        let assign670_e751: f64 = (assign670_e747 + assign670_e750);
        let assign670_e753: f64 = (assign670_e751 + p.p52);
        let assign670_e756: f64 = (locals.var_len * locals.var_wid);
        let assign670_e757: f64 = (assign670_e753 / assign670_e756);
        let assign670_e758: f64 = (p.p49 + assign670_e757);
        (assign670_e758,)
    }
};
        locals.var_df = assign670_e759;

        let assign680_e762: f64 = if locals.var_df < locals.var_dfmin { 1.0 } else { 0.0 };
        locals.var_guard116 = assign680_e762;

        let (assign690_e771,) = {
    if (locals.var_guard116 != 0.0) {
        let (assign690_e769,) = {
            if (locals.var_df > 0.0) {
                (locals.var_df,)
            } else {
                (0.0,)
            }
        };
        (assign690_e769,)
    } else {
        (locals.var_df,)
    }
};
        locals.var_df = assign690_e771;

        let (assign700_e777, assign700_e777_d_n3,) = {
    if (locals.var_guard116 != 0.0) {
        let assign700_e775: f64 = (locals.var_dfmin * locals.var_dfmin);
        (assign700_e775, ((locals.var_dfmin_dn3 * locals.var_dfmin) + (locals.var_dfmin * locals.var_dfmin_dn3)),)
    } else {
        (locals.var_dfsq, locals.var_dfsq_dn3,)
    }
};
        locals.var_dfsq = assign700_e777;
        locals.var_dfsq_dn3 = assign700_e777_d_n3;

        let (assign710_e784, assign710_e784_d_n3,) = {
    if (locals.var_guard116 == 0.0) {
        let assign710_e782: f64 = (locals.var_df * locals.var_df);
        (assign710_e782, 0.0,)
    } else {
        (locals.var_dfsq, locals.var_dfsq_dn3,)
    }
};
        locals.var_dfsq = assign710_e784;
        locals.var_dfsq_dn3 = assign710_e784_d_n3;

        let assign720_e787: f64 = (0.5 / locals.var_dfsq);
        let assign720_e790: f64 = (locals.var_dp_i * 0.5);
        let assign720_e791: f64 = (assign720_e787 - assign720_e790);
        locals.var_vpo = assign720_e791;
        locals.var_vpo_dn3 = ((-((0.5 * locals.var_dfsq_dn3) / (locals.var_dfsq * locals.var_dfsq))) - (locals.var_dp_i_dn3 * 0.5));

        let assign730_e794: f64 = if p.p63 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign730_e794;

        let (assign740_e804, assign740_e804_d_n3,) = {
    if (locals.var_guard117 != 0.0) {
        let assign740_e799: f64 = (2.0 * p.p64);
        let assign740_e801: f64 = (assign740_e799 / locals.var_dfsq);
        let assign740_e802: f64 = (locals.var_vpo - assign740_e801);
        (assign740_e802, (locals.var_vpo_dn3 - (-((assign740_e799 * locals.var_dfsq_dn3) / (locals.var_dfsq * locals.var_dfsq)))),)
    } else {
        (locals.var_vpoe, locals.var_vpoe_dn3,)
    }
};
        locals.var_vpoe = assign740_e804;
        locals.var_vpoe_dn3 = assign740_e804_d_n3;

    }

    pub(super) fn stamp_transient_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (assign750_e814,) = {
    if (locals.var_guard117 != 0.0) {
        let assign750_e808: f64 = (0.1666666666666667 / locals.var_dfsq);
        let assign750_e811: f64 = (locals.var_dp_i * 0.5);
        let assign750_e812: f64 = (assign750_e808 - assign750_e811);
        (assign750_e812,)
    } else {
        (locals.var_v1cx,)
    }
};
        locals.var_v1cx = assign750_e814;

        let assign760_e817: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign760_e817;

        let (assign770_e831, assign770_e831_d_n3,) = {
    if ((locals.var_guard117 == 0.0) && (locals.var_guard118 != 0.0)) {
        let assign770_e825: f64 = (2.0 * p.p64);
        let assign770_e827: f64 = (assign770_e825 / locals.var_dfsq);
        let assign770_e828: f64 = (assign770_e827).sqrt();
        let assign770_e829: f64 = (locals.var_vpo - assign770_e828);
        (assign770_e829, (locals.var_vpo_dn3 - ((-((assign770_e825 * locals.var_dfsq_dn3) / (locals.var_dfsq * locals.var_dfsq))) / (2.0 * assign770_e828))),)
    } else {
        (locals.var_vpoe, locals.var_vpoe_dn3,)
    }
};
        locals.var_vpoe = assign770_e831;
        locals.var_vpoe_dn3 = assign770_e831_d_n3;

        let (assign780_e838,) = {
    if ((locals.var_guard117 == 0.0) && (locals.var_guard118 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_v1cx,)
    }
};
        locals.var_v1cx = assign780_e838;

        let (assign790_e846, assign790_e846_d_n3,) = {
    if ((locals.var_guard117 == 0.0) && (locals.var_guard118 == 0.0)) {
        (locals.var_vpo, locals.var_vpo_dn3,)
    } else {
        (locals.var_vpoe, locals.var_vpoe_dn3,)
    }
};
        locals.var_vpoe = assign790_e846;
        locals.var_vpoe_dn3 = assign790_e846_d_n3;

        let (assign800_e854,) = {
    if ((locals.var_guard117 == 0.0) && (locals.var_guard118 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_v1cx,)
    }
};
        locals.var_v1cx = assign800_e854;

        let assign810_e859: f64 = (p.p48 / locals.var_leff_um);
        let assign810_e860: f64 = (1.0 + assign810_e859);
        let assign810_e861: f64 = (p.p47 / assign810_e860);
        locals.var_ats_i = assign810_e861;

        let assign820_e864: f64 = if p.p63 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign820_e864;

        let (assign830_e870, assign830_e870_d_n3,) = {
    if (locals.var_guard119 != 0.0) {
        let assign830_e868: f64 = (p.p46 * locals.var_phi_t0);
        (assign830_e868, (p.p46 * locals.var_phi_t0_dn3),)
    } else {
        (locals.var_nsteff, locals.var_nsteff_dn3,)
    }
};
        locals.var_nsteff = assign830_e870;
        locals.var_nsteff_dn3 = assign830_e870_d_n3;

        let (assign840_e891, assign840_e891_d_n3,) = {
    if (locals.var_guard119 != 0.0) {
        let (assign840_e889, assign840_e889_d_n3,) = {
            if (p.p63 > 2.0) {
                let assign840_e877: f64 = (0.55 * locals.var_phi_t0);
                let assign840_e880: f64 = (-locals.var_ats_i);
                let assign840_e882: f64 = (assign840_e880 / locals.var_phi_t0);
                let assign840_e883: f64 = (assign840_e882).exp();
                let assign840_e884: f64 = (1.0 + assign840_e883);
                let assign840_e885: f64 = (assign840_e877 * assign840_e884);
                (assign840_e885, (((0.55 * locals.var_phi_t0_dn3) * assign840_e884) + (assign840_e877 * (assign840_e883 * (-((assign840_e880 * locals.var_phi_t0_dn3) / (locals.var_phi_t0 * locals.var_phi_t0)))))),)
            } else {
                let assign840_e888: f64 = (1.1 * locals.var_phi_t0);
                (assign840_e888, (1.1 * locals.var_phi_t0_dn3),)
            }
        };
        (assign840_e889, assign840_e889_d_n3,)
    } else {
        (locals.var_atspo, locals.var_atspo_dn3,)
    }
};
        locals.var_atspo = assign840_e891;
        locals.var_atspo_dn3 = assign840_e891_d_n3;

        let assign850_e894: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign850_e894;

        let (assign860_e905, assign860_e905_d_n3,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 != 0.0)) {
        let assign860_e901: f64 = (2.0 * p.p46);
        let assign860_e903: f64 = (assign860_e901 * locals.var_phi_t0);
        (assign860_e903, (assign860_e901 * locals.var_phi_t0_dn3),)
    } else {
        (locals.var_nsteff, locals.var_nsteff_dn3,)
    }
};
        locals.var_nsteff = assign860_e905;
        locals.var_nsteff_dn3 = assign860_e905_d_n3;

        let (assign870_e916, assign870_e916_d_n3,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 != 0.0)) {
        let assign870_e912: f64 = (4.0 * locals.var_ats_i);
        let assign870_e914: f64 = (assign870_e912 * locals.var_ats_i);
        (assign870_e914, 0.0,)
    } else {
        (locals.var_atspo, locals.var_atspo_dn3,)
    }
};
        locals.var_atspo = assign870_e916;
        locals.var_atspo_dn3 = assign870_e916_d_n3;

        let (assign880_e926, assign880_e926_d_n3,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 == 0.0)) {
        let assign880_e924: f64 = (p.p46 * locals.var_phi_t0);
        (assign880_e924, (p.p46 * locals.var_phi_t0_dn3),)
    } else {
        (locals.var_nsteff, locals.var_nsteff_dn3,)
    }
};
        locals.var_nsteff = assign880_e926;
        locals.var_nsteff_dn3 = assign880_e926_d_n3;

        let (assign890_e938, assign890_e938_d_n3,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 == 0.0)) {
        let assign890_e934: f64 = (4.0 * locals.var_ats_i);
        let assign890_e936: f64 = (assign890_e934 * locals.var_ats_i);
        (assign890_e936, 0.0,)
    } else {
        (locals.var_atspo, locals.var_atspo_dn3,)
    }
};
        locals.var_atspo = assign890_e938;
        locals.var_atspo_dn3 = assign890_e938_d_n3;

        let assign900_e941: f64 = (p.p37 * locals.var_delr_rsh);
        let assign900_e944: f64 = (locals.var_leff_um / locals.var_weff_um);
        let assign900_e945: f64 = (assign900_e941 * assign900_e944);
        let assign900_e949: f64 = (locals.var_dp_i).sqrt();
        let assign900_e950: f64 = (locals.var_df * assign900_e949);
        let assign900_e951: f64 = (1.0 - assign900_e950);
        let assign900_e952: f64 = (assign900_e945 * assign900_e951);
        locals.var_r0 = assign900_e952;
        locals.var_r0_dn3 = (assign900_e945 * (-(locals.var_df * (locals.var_dp_i_dn3 / (2.0 * assign900_e949)))));

        let assign920_e962: f64 = if ((p.p66 > 0.0) && (p.p5 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard122 = assign920_e962;

        let (assign930_e972,) = {
    if (locals.var_guard122 != 0.0) {
        let assign930_e967: f64 = (p.p67 / locals.var_w_um);
        let assign930_e968: f64 = (p.p66 + assign930_e967);
        let assign930_e970: f64 = (assign930_e968 / p.p5);
        (assign930_e970,)
    } else {
        (locals.var_rc1_tnom,)
    }
};
        locals.var_rc1_tnom = assign930_e972;

        let (assign940_e977,) = {
    if (locals.var_guard122 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rc1_tnom,)
    }
};
        locals.var_rc1_tnom = assign940_e977;

        let assign950_e984: f64 = if ((p.p66 > 0.0) && (p.p8 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard123 = assign950_e984;

        let (assign960_e994,) = {
    if (locals.var_guard123 != 0.0) {
        let assign960_e989: f64 = (p.p67 / locals.var_w_um);
        let assign960_e990: f64 = (p.p66 + assign960_e989);
        let assign960_e992: f64 = (assign960_e990 / p.p8);
        (assign960_e992,)
    } else {
        (locals.var_rc2_tnom,)
    }
};
        locals.var_rc2_tnom = assign960_e994;

        let (assign970_e999,) = {
    if (locals.var_guard123 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rc2_tnom,)
    }
};
        locals.var_rc2_tnom = assign970_e999;

        let (assign980_e1003, assign980_e1003_d_n3,) = {
    if (p.p15 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_gth, locals.var_gth_dn3,)
    }
};
        locals.var_gth = assign980_e1003;
        locals.var_gth_dn3 = assign980_e1003_d_n3;

        let (assign990_e1007,) = {
    if (p.p15 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign990_e1007;

        let (assign1000_e1030, assign1000_e1030_d_n3,) = {
    if (p.p15 == 0.0) {
        let assign1000_e1013: f64 = (p.p111 * locals.var_p_um);
        let assign1000_e1014: f64 = (p.p110 + assign1000_e1013);
        let assign1000_e1017: f64 = (p.p112 * locals.var_a_um2);
        let assign1000_e1018: f64 = (assign1000_e1014 + assign1000_e1017);
        let assign1000_e1022: f64 = (p.p5 + p.p8);
        let assign1000_e1023: f64 = (p.p113 * assign1000_e1022);
        let assign1000_e1024: f64 = (assign1000_e1018 + assign1000_e1023);
        let assign1000_e1027: f64 = (locals.var_rt).powf(p.p109);
        let assign1000_e1028: f64 = (assign1000_e1024 * assign1000_e1027);
        (assign1000_e1028, (assign1000_e1024 * if 0.0 == 0.0 && ((p.p109) as f64).is_finite() && ((p.p109) as f64).fract() == 0.0 { if p.p109 == 0.0 { 0.0 } else { (p.p109 * ((locals.var_rt).powf(p.p109 - 1.0) * locals.var_rt_dn3)) } } else { (assign1000_e1027 * (p.p109 * (locals.var_rt_dn3 / locals.var_rt))) }),)
    } else {
        (locals.var_gth, locals.var_gth_dn3,)
    }
};
        locals.var_gth = assign1000_e1030;
        locals.var_gth_dn3 = assign1000_e1030_d_n3;

        let (assign1010_e1049,) = {
    if (p.p15 == 0.0) {
        let assign1010_e1036: f64 = (p.p115 * locals.var_p_um);
        let assign1010_e1037: f64 = (p.p114 + assign1010_e1036);
        let assign1010_e1040: f64 = (p.p116 * locals.var_a_um2);
        let assign1010_e1041: f64 = (assign1010_e1037 + assign1010_e1040);
        let assign1010_e1045: f64 = (p.p5 + p.p8);
        let assign1010_e1046: f64 = (p.p117 * assign1010_e1045);
        let assign1010_e1047: f64 = (assign1010_e1041 + assign1010_e1046);
        (assign1010_e1047,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign1010_e1049;

        let assign1020_e1053: f64 = (p.p97 / locals.var_weff_um);
        let assign1020_e1054: f64 = (p.p93 + assign1020_e1053);
        let assign1020_e1058: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign1020_e1061: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign1020_e1062: f64 = (assign1020_e1058 + assign1020_e1061);
        let assign1020_e1063: f64 = (0.5 * assign1020_e1062);
        let assign1020_e1067: f64 = (p.p99 / locals.var_weff_um);
        let assign1020_e1068: f64 = (p.p95 + assign1020_e1067);
        let assign1020_e1069: f64 = (assign1020_e1063 * assign1020_e1068);
        let assign1020_e1071: f64 = (assign1020_e1069 / locals.var_leff_um);
        let assign1020_e1072: f64 = (assign1020_e1054 + assign1020_e1071);
        locals.var_tc1e = assign1020_e1072;

        let assign1030_e1076: f64 = (p.p98 / locals.var_weff_um);
        let assign1030_e1077: f64 = (p.p94 + assign1030_e1076);
        let assign1030_e1081: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign1030_e1084: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign1030_e1085: f64 = (assign1030_e1081 + assign1030_e1084);
        let assign1030_e1086: f64 = (0.5 * assign1030_e1085);
        let assign1030_e1090: f64 = (p.p100 / locals.var_weff_um);
        let assign1030_e1091: f64 = (p.p96 + assign1030_e1090);
        let assign1030_e1092: f64 = (assign1030_e1086 * assign1030_e1091);
        let assign1030_e1094: f64 = (assign1030_e1092 / locals.var_leff_um);
        let assign1030_e1095: f64 = (assign1030_e1077 + assign1030_e1094);
        locals.var_tc2e = assign1030_e1095;

        let assign1040_e1098: f64 = (p.p71 * locals.var_a1_um2);
        let assign1040_e1101: f64 = (p.p78 * locals.var_p1_um);
        let assign1040_e1102: f64 = (assign1040_e1098 + assign1040_e1101);
        locals.var_cf1 = assign1040_e1102;
        locals.var_cf1_dn1 = (p.p71 * locals.var_a1_um2_dn1);
        locals.var_cf1_dn3 = (p.p71 * locals.var_a1_um2_dn3);
        locals.var_cf1_dn4 = (p.p71 * locals.var_a1_um2_dn4);
        locals.var_cf1_dn5 = (p.p71 * locals.var_a1_um2_dn5);

        let assign1050_e1105: f64 = (p.p71 * locals.var_a2_um2);
        let assign1050_e1108: f64 = (p.p78 * locals.var_p2_um);
        let assign1050_e1109: f64 = (assign1050_e1105 + assign1050_e1108);
        locals.var_cf2 = assign1050_e1109;
        locals.var_cf2_dn1 = (p.p71 * locals.var_a2_um2_dn1);
        locals.var_cf2_dn3 = (p.p71 * locals.var_a2_um2_dn3);
        locals.var_cf2_dn4 = (p.p71 * locals.var_a2_um2_dn4);
        locals.var_cf2_dn5 = (p.p71 * locals.var_a2_um2_dn5);

        let assign1060_e1112: f64 = (p.p72 * locals.var_a1_um2);
        let assign1060_e1115: f64 = (p.p79 * locals.var_p1_um);
        let assign1060_e1116: f64 = (assign1060_e1112 + assign1060_e1115);
        locals.var_cj1 = assign1060_e1116;

        let assign1070_e1119: f64 = (p.p72 * locals.var_a2_um2);
        let assign1070_e1122: f64 = (p.p79 * locals.var_p2_um);
        let assign1070_e1123: f64 = (assign1070_e1119 + assign1070_e1122);
        locals.var_cj2 = assign1070_e1123;

        locals.var_dt_et = (nv3 - 0.0);
        locals.var_dt_et_dn3 = 1.0;

        let assign1090_e1126: f64 = (-p.p21);
        let assign1090_e1128: f64 = (assign1090_e1126 * (nv5 - nv4));
        locals.var_vrb = assign1090_e1128;
        locals.var_vrb_dn4 = (-assign1090_e1126);
        locals.var_vrb_dn5 = assign1090_e1126;

        let assign1100_e1130: f64 = (-p.p21);
        let assign1100_e1132: f64 = (assign1100_e1130 * (nv1 - nv4));
        locals.var_vc1 = assign1100_e1132;
        locals.var_vc1_dn1 = assign1100_e1130;
        locals.var_vc1_dn4 = (-assign1100_e1130);

        let assign1110_e1134: f64 = (-p.p21);
        let assign1110_e1136: f64 = (assign1110_e1134 * (nv1 - nv5));
        locals.var_vc2 = assign1110_e1136;
        locals.var_vc2_dn1 = assign1110_e1134;
        locals.var_vc2_dn5 = (-assign1110_e1134);

        let assign1120_e1137: f64 = ctx_temp;
        let assign1120_e1139: f64 = (assign1120_e1137 + p.p9);
        let assign1120_e1141: f64 = (assign1120_e1139 + locals.var_dt_et);
        let assign1120_e1143: f64 = (assign1120_e1141 - 273.15);
        locals.var_tdevc = assign1120_e1143;
        locals.var_tdevc_dn3 = locals.var_dt_et_dn3;

        let assign1130_e1147: f64 = (p.p35 + 1.0);
        let assign1130_e1148: f64 = if locals.var_tdevc < assign1130_e1147 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign1130_e1148;

        let (assign1140_e1159, assign1140_e1159_d_n3,) = {
    if (locals.var_guard124 != 0.0) {
        let assign1140_e1153: f64 = (locals.var_tdevc - p.p35);
        let assign1140_e1155: f64 = (assign1140_e1153 - 1.0);
        let assign1140_e1156: f64 = (assign1140_e1155).exp();
        let assign1140_e1157: f64 = (p.p35 + assign1140_e1156);
        (assign1140_e1157, (assign1140_e1156 * locals.var_tdevc_dn3),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign1140_e1159;
        locals.var_tdevc_dn3 = assign1140_e1159_d_n3;

        let assign1150_e1163: f64 = (p.p36 - 1.0);
        let assign1150_e1164: f64 = if locals.var_tdevc > assign1150_e1163 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign1150_e1164;

        let (assign1160_e1178, assign1160_e1178_d_n3,) = {
    if ((locals.var_guard124 == 0.0) && (locals.var_guard125 != 0.0)) {
        let assign1160_e1172: f64 = (p.p36 - locals.var_tdevc);
        let assign1160_e1174: f64 = (assign1160_e1172 - 1.0);
        let assign1160_e1175: f64 = (assign1160_e1174).exp();
        let assign1160_e1176: f64 = (p.p36 - assign1160_e1175);
        (assign1160_e1176, (-(assign1160_e1175 * (-locals.var_tdevc_dn3))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign1160_e1178;
        locals.var_tdevc_dn3 = assign1160_e1178_d_n3;

        let (assign1170_e1186, assign1170_e1186_d_n3,) = {
    if ((locals.var_guard124 == 0.0) && (locals.var_guard125 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign1170_e1186;
        locals.var_tdevc_dn3 = assign1170_e1186_d_n3;

        let assign1180_e1189: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign1180_e1189;
        locals.var_tdevk_dn3 = locals.var_tdevc_dn3;

        let assign1190_e1192: f64 = (1.3806505e-23 * locals.var_tdevk);
        let assign1190_e1194: f64 = (assign1190_e1192 / 1.60217653e-19);
        locals.var_phi_t = assign1190_e1194;
        locals.var_phi_t_dn3 = ((1.3806505e-23 * locals.var_tdevk_dn3) / 1.60217653e-19);

        let assign1200_e1197: f64 = (locals.var_tdevk / locals.var_tinik);
        locals.var_rt = assign1200_e1197;
        locals.var_rt_dn3 = (locals.var_tdevk_dn3 / locals.var_tinik);

        let assign1210_e1200: f64 = (locals.var_tdevk - locals.var_tinik);
        locals.var_dt = assign1210_e1200;
        locals.var_dt_dn3 = locals.var_tdevk_dn3;

        let assign1220_e1206: f64 = (locals.var_dt * locals.var_tc2e);
        let assign1220_e1207: f64 = (locals.var_tc1e + assign1220_e1206);
        let assign1220_e1208: f64 = (locals.var_dt * assign1220_e1207);
        let assign1220_e1209: f64 = (1.0 + assign1220_e1208);
        locals.var_tcr = assign1220_e1209;
        locals.var_tcr_dn3 = ((locals.var_dt_dn3 * assign1220_e1207) + (locals.var_dt * (locals.var_dt_dn3 * locals.var_tc2e)));

        let assign1230_e1213: f64 = (0.01 + 0.1);
        let assign1230_e1214: f64 = if locals.var_tcr < assign1230_e1213 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign1230_e1214;

        let (assign1240_e1229, assign1240_e1229_d_n3,) = {
    if (locals.var_guard126 != 0.0) {
        let assign1240_e1221: f64 = (locals.var_tcr - 0.01);
        let assign1240_e1222: f64 = (10.0 * assign1240_e1221);
        let assign1240_e1224: f64 = (assign1240_e1222 - 1.0);
        let assign1240_e1225: f64 = (assign1240_e1224).exp();
        let assign1240_e1226: f64 = (0.1 * assign1240_e1225);
        let assign1240_e1227: f64 = (0.01 + assign1240_e1226);
        (assign1240_e1227, (0.1 * (assign1240_e1225 * (10.0 * locals.var_tcr_dn3))),)
    } else {
        (locals.var_tcr, locals.var_tcr_dn3,)
    }
};
        locals.var_tcr = assign1240_e1229;
        locals.var_tcr_dn3 = assign1240_e1229_d_n3;

        let (assign1250_e1234, assign1250_e1234_d_n3,) = {
    if (locals.var_guard126 == 0.0) {
        (locals.var_tcr, locals.var_tcr_dn3,)
    } else {
        (locals.var_tcr, locals.var_tcr_dn3,)
    }
};
        locals.var_tcr = assign1250_e1234;
        locals.var_tcr_dn3 = assign1250_e1234_d_n3;

        let (assign1260_e1249, assign1260_e1249_d_n3,) = {
    if (p.p63 != 0.0) {
        let assign1260_e1241: f64 = (locals.var_dp_i).sqrt();
        let assign1260_e1242: f64 = (locals.var_df * assign1260_e1241);
        let assign1260_e1243: f64 = (1.0 - assign1260_e1242);
        let assign1260_e1244: f64 = (locals.var_r0 * assign1260_e1243);
        let assign1260_e1246: f64 = (assign1260_e1244 * locals.var_tcr);
        let assign1260_e1247: f64 = (1.0 / assign1260_e1246);
        (assign1260_e1247, (-(((((locals.var_r0_dn3 * assign1260_e1243) + (locals.var_r0 * (-(locals.var_df * (locals.var_dp_i_dn3 / (2.0 * assign1260_e1241)))))) * locals.var_tcr) + (assign1260_e1244 * locals.var_tcr_dn3)) / (assign1260_e1246 * assign1260_e1246))),)
    } else {
        (locals.var_gf, locals.var_gf_dn3,)
    }
};
        locals.var_gf = assign1260_e1249;
        locals.var_gf_dn3 = assign1260_e1249_d_n3;

        let (assign1270_e1258, assign1270_e1258_d_n3,) = {
    if (p.p63 == 0.0) {
        let assign1270_e1255: f64 = (locals.var_r0 * locals.var_tcr);
        let assign1270_e1256: f64 = (1.0 / assign1270_e1255);
        (assign1270_e1256, (-(((locals.var_r0_dn3 * locals.var_tcr) + (locals.var_r0 * locals.var_tcr_dn3)) / (assign1270_e1255 * assign1270_e1255))),)
    } else {
        (locals.var_gf, locals.var_gf_dn3,)
    }
};
        locals.var_gf = assign1270_e1258;
        locals.var_gf_dn3 = assign1270_e1258_d_n3;

        let assign1280_e1264: f64 = (locals.var_dt * p.p102);
        let assign1280_e1265: f64 = (p.p101 + assign1280_e1264);
        let assign1280_e1266: f64 = (locals.var_dt * assign1280_e1265);
        let assign1280_e1267: f64 = (1.0 + assign1280_e1266);
        locals.var_tcrc = assign1280_e1267;
        locals.var_tcrc_dn3 = ((locals.var_dt_dn3 * assign1280_e1265) + (locals.var_dt * (locals.var_dt_dn3 * p.p102)));

        let assign1290_e1271: f64 = (0.01 + 0.1);
        let assign1290_e1272: f64 = if locals.var_tcrc < assign1290_e1271 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign1290_e1272;

        let (assign1300_e1287, assign1300_e1287_d_n3,) = {
    if (locals.var_guard127 != 0.0) {
        let assign1300_e1279: f64 = (locals.var_tcrc - 0.01);
        let assign1300_e1280: f64 = (10.0 * assign1300_e1279);
        let assign1300_e1282: f64 = (assign1300_e1280 - 1.0);
        let assign1300_e1283: f64 = (assign1300_e1282).exp();
        let assign1300_e1284: f64 = (0.1 * assign1300_e1283);
        let assign1300_e1285: f64 = (0.01 + assign1300_e1284);
        (assign1300_e1285, (0.1 * (assign1300_e1283 * (10.0 * locals.var_tcrc_dn3))),)
    } else {
        (locals.var_tcrc, locals.var_tcrc_dn3,)
    }
};
        locals.var_tcrc = assign1300_e1287;
        locals.var_tcrc_dn3 = assign1300_e1287_d_n3;

        let (assign1310_e1292, assign1310_e1292_d_n3,) = {
    if (locals.var_guard127 == 0.0) {
        (locals.var_tcrc, locals.var_tcrc_dn3,)
    } else {
        (locals.var_tcrc, locals.var_tcrc_dn3,)
    }
};
        locals.var_tcrc = assign1310_e1292;
        locals.var_tcrc_dn3 = assign1310_e1292_d_n3;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign1320_e1295: f64 = (locals.var_rt).powf(p.p92);
        locals.var_tcvsat = assign1320_e1295;
        locals.var_tcvsat_dn3 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((locals.var_rt).powf(p.p92 - 1.0) * locals.var_rt_dn3)) } } else { (assign1320_e1295 * (p.p92 * (locals.var_rt_dn3 / locals.var_rt))) };

        let assign1330_e1298: f64 = if p.p69 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign1330_e1298;

        let (assign1340_e1319, assign1340_e1319_d_n3,) = {
    if (locals.var_guard128 != 0.0) {
        let assign1340_e1302: f64 = (-p.p90);
        let assign1340_e1305: f64 = (1.0 - locals.var_rt);
        let assign1340_e1306: f64 = (assign1340_e1302 * assign1340_e1305);
        let assign1340_e1308: f64 = (assign1340_e1306 / locals.var_phi_t);
        let assign1340_e1311: f64 = (locals.var_rt).ln();
        let assign1340_e1312: f64 = (p.p91 * assign1340_e1311);
        let assign1340_e1313: f64 = (assign1340_e1308 + assign1340_e1312);
        let assign1340_e1315: f64 = (assign1340_e1313 / p.p70);
        let assign1340_e1316: f64 = (assign1340_e1315).exp();
        let assign1340_e1317: f64 = (p.p69 * assign1340_e1316);
        (assign1340_e1317, (p.p69 * (assign1340_e1316 * ((((((assign1340_e1302 * (-locals.var_rt_dn3)) * locals.var_phi_t) - (assign1340_e1306 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)) + (p.p91 * (locals.var_rt_dn3 / locals.var_rt))) / p.p70))),)
    } else {
        (locals.var_isa_t, locals.var_isa_t_dn3,)
    }
};
        locals.var_isa_t = assign1340_e1319;
        locals.var_isa_t_dn3 = assign1340_e1319_d_n3;

        let (assign1350_e1332, assign1350_e1332_d_n3,) = {
    if (locals.var_guard128 != 0.0) {
        let assign1350_e1323: f64 = (p.p70 * locals.var_phi_t);
        let assign1350_e1327: f64 = (p.p27 / locals.var_isa_t);
        let assign1350_e1328: f64 = (1.0 + assign1350_e1327);
        let assign1350_e1329: f64 = (assign1350_e1328).ln();
        let assign1350_e1330: f64 = (assign1350_e1323 * assign1350_e1329);
        (assign1350_e1330, (((p.p70 * locals.var_phi_t_dn3) * assign1350_e1329) + (assign1350_e1323 * ((-((p.p27 * locals.var_isa_t_dn3) / (locals.var_isa_t * locals.var_isa_t))) / assign1350_e1328))),)
    } else {
        (locals.var_vmax_a, locals.var_vmax_a_dn3,)
    }
};
        locals.var_vmax_a = assign1350_e1332;
        locals.var_vmax_a_dn3 = assign1350_e1332_d_n3;

        let (assign1360_e1337, assign1360_e1337_d_n3,) = {
    if (locals.var_guard128 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_isa_t, locals.var_isa_t_dn3,)
    }
};
        locals.var_isa_t = assign1360_e1337;
        locals.var_isa_t_dn3 = assign1360_e1337_d_n3;

        let (assign1370_e1342, assign1370_e1342_d_n3,) = {
    if (locals.var_guard128 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vmax_a, locals.var_vmax_a_dn3,)
    }
};
        locals.var_vmax_a = assign1370_e1342;
        locals.var_vmax_a_dn3 = assign1370_e1342_d_n3;

        let assign1380_e1345: f64 = if p.p76 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign1380_e1345;

        let (assign1390_e1366, assign1390_e1366_d_n3,) = {
    if (locals.var_guard129 != 0.0) {
        let assign1390_e1349: f64 = (-p.p90);
        let assign1390_e1352: f64 = (1.0 - locals.var_rt);
        let assign1390_e1353: f64 = (assign1390_e1349 * assign1390_e1352);
        let assign1390_e1355: f64 = (assign1390_e1353 / locals.var_phi_t);
        let assign1390_e1358: f64 = (locals.var_rt).ln();
        let assign1390_e1359: f64 = (p.p91 * assign1390_e1358);
        let assign1390_e1360: f64 = (assign1390_e1355 + assign1390_e1359);
        let assign1390_e1362: f64 = (assign1390_e1360 / p.p77);
        let assign1390_e1363: f64 = (assign1390_e1362).exp();
        let assign1390_e1364: f64 = (p.p76 * assign1390_e1363);
        (assign1390_e1364, (p.p76 * (assign1390_e1363 * ((((((assign1390_e1349 * (-locals.var_rt_dn3)) * locals.var_phi_t) - (assign1390_e1353 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)) + (p.p91 * (locals.var_rt_dn3 / locals.var_rt))) / p.p77))),)
    } else {
        (locals.var_isp_t, locals.var_isp_t_dn3,)
    }
};
        locals.var_isp_t = assign1390_e1366;
        locals.var_isp_t_dn3 = assign1390_e1366_d_n3;

        let (assign1400_e1379, assign1400_e1379_d_n3,) = {
    if (locals.var_guard129 != 0.0) {
        let assign1400_e1370: f64 = (p.p77 * locals.var_phi_t);
        let assign1400_e1374: f64 = (p.p27 / locals.var_isp_t);
        let assign1400_e1375: f64 = (1.0 + assign1400_e1374);
        let assign1400_e1376: f64 = (assign1400_e1375).ln();
        let assign1400_e1377: f64 = (assign1400_e1370 * assign1400_e1376);
        (assign1400_e1377, (((p.p77 * locals.var_phi_t_dn3) * assign1400_e1376) + (assign1400_e1370 * ((-((p.p27 * locals.var_isp_t_dn3) / (locals.var_isp_t * locals.var_isp_t))) / assign1400_e1375))),)
    } else {
        (locals.var_vmax_p, locals.var_vmax_p_dn3,)
    }
};
        locals.var_vmax_p = assign1400_e1379;
        locals.var_vmax_p_dn3 = assign1400_e1379_d_n3;

        let (assign1410_e1384, assign1410_e1384_d_n3,) = {
    if (locals.var_guard129 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_isp_t, locals.var_isp_t_dn3,)
    }
};
        locals.var_isp_t = assign1410_e1384;
        locals.var_isp_t_dn3 = assign1410_e1384_d_n3;

        let (assign1420_e1389, assign1420_e1389_d_n3,) = {
    if (locals.var_guard129 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_vmax_p, locals.var_vmax_p_dn3,)
    }
};
        locals.var_vmax_p = assign1420_e1389;
        locals.var_vmax_p_dn3 = assign1420_e1389_d_n3;

        let assign1430_e1392: f64 = (locals.var_a1_um2 * locals.var_isa_t);
        let assign1430_e1395: f64 = (locals.var_p1_um * locals.var_isp_t);
        let assign1430_e1396: f64 = (assign1430_e1392 + assign1430_e1395);
        locals.var_is1 = assign1430_e1396;

        let assign1440_e1399: f64 = (locals.var_a2_um2 * locals.var_isa_t);
        let assign1440_e1402: f64 = (locals.var_p2_um * locals.var_isp_t);
        let assign1440_e1403: f64 = (assign1440_e1399 + assign1440_e1402);
        locals.var_is2 = assign1440_e1403;

        let assign1450_e1406: f64 = if p.p72 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign1450_e1406;

        let (assign1460_e1434, assign1460_e1434_d_n3,) = {
    if (locals.var_guard130 != 0.0) {
        let assign1460_e1411: f64 = (locals.var_phi_t / locals.var_rt);
        let assign1460_e1412: f64 = (2.0 * assign1460_e1411);
        let assign1460_e1415: f64 = (0.5 * p.p73);
        let assign1460_e1417: f64 = (assign1460_e1415 * locals.var_rt);
        let assign1460_e1419: f64 = (assign1460_e1417 / locals.var_phi_t);
        let assign1460_e1420: f64 = (assign1460_e1419).exp();
        let assign1460_e1422: f64 = (-0.5);
        let assign1460_e1424: f64 = (assign1460_e1422 * p.p73);
        let assign1460_e1426: f64 = (assign1460_e1424 * locals.var_rt);
        let assign1460_e1428: f64 = (assign1460_e1426 / locals.var_phi_t);
        let assign1460_e1429: f64 = (assign1460_e1428).exp();
        let assign1460_e1430: f64 = (assign1460_e1420 - assign1460_e1429);
        let assign1460_e1431: f64 = (assign1460_e1430).ln();
        let assign1460_e1432: f64 = (assign1460_e1412 * assign1460_e1431);
        (assign1460_e1432, (((2.0 * (((locals.var_phi_t_dn3 * locals.var_rt) - (locals.var_phi_t * locals.var_rt_dn3)) / (locals.var_rt * locals.var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * locals.var_rt_dn3) * locals.var_phi_t) - (assign1460_e1417 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * locals.var_rt_dn3) * locals.var_phi_t) - (assign1460_e1426 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)))) / assign1460_e1430))),)
    } else {
        (locals.var_psiio, locals.var_psiio_dn3,)
    }
};
        locals.var_psiio = assign1460_e1434;
        locals.var_psiio_dn3 = assign1460_e1434_d_n3;

        let (assign1470_e1453, assign1470_e1453_d_n3,) = {
    if (locals.var_guard130 != 0.0) {
        let assign1470_e1438: f64 = (locals.var_psiio * locals.var_rt);
        let assign1470_e1441: f64 = (3.0 * locals.var_phi_t);
        let assign1470_e1443: f64 = (locals.var_rt).ln();
        let assign1470_e1444: f64 = (assign1470_e1441 * assign1470_e1443);
        let assign1470_e1445: f64 = (assign1470_e1438 - assign1470_e1444);
        let assign1470_e1449: f64 = (locals.var_rt - 1.0);
        let assign1470_e1450: f64 = (p.p90 * assign1470_e1449);
        let assign1470_e1451: f64 = (assign1470_e1445 - assign1470_e1450);
        (assign1470_e1451, ((((locals.var_psiio_dn3 * locals.var_rt) + (locals.var_psiio * locals.var_rt_dn3)) - (((3.0 * locals.var_phi_t_dn3) * assign1470_e1443) + (assign1470_e1441 * (locals.var_rt_dn3 / locals.var_rt)))) - (p.p90 * locals.var_rt_dn3)),)
    } else {
        (locals.var_psiin, locals.var_psiin_dn3,)
    }
};
        locals.var_psiin = assign1470_e1453;
        locals.var_psiin_dn3 = assign1470_e1453_d_n3;

        let (assign1480_e1477, assign1480_e1477_d_n3,) = {
    if (locals.var_guard130 != 0.0) {
        let assign1480_e1458: f64 = (2.0 * locals.var_phi_t);
        let assign1480_e1464: f64 = (-locals.var_psiin);
        let assign1480_e1466: f64 = (assign1480_e1464 / locals.var_phi_t);
        let assign1480_e1467: f64 = (assign1480_e1466).exp();
        let assign1480_e1468: f64 = (4.0 * assign1480_e1467);
        let assign1480_e1469: f64 = (1.0 + assign1480_e1468);
        let assign1480_e1470: f64 = (assign1480_e1469).sqrt();
        let assign1480_e1471: f64 = (1.0 + assign1480_e1470);
        let assign1480_e1472: f64 = (0.5 * assign1480_e1471);
        let assign1480_e1473: f64 = (assign1480_e1472).ln();
        let assign1480_e1474: f64 = (assign1480_e1458 * assign1480_e1473);
        let assign1480_e1475: f64 = (locals.var_psiin + assign1480_e1474);
        (assign1480_e1475, (locals.var_psiin_dn3 + (((2.0 * locals.var_phi_t_dn3) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-locals.var_psiin_dn3) * locals.var_phi_t) - (assign1480_e1464 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))),)
    } else {
        (locals.var_pa_t, locals.var_pa_t_dn3,)
    }
};
        locals.var_pa_t = assign1480_e1477;
        locals.var_pa_t_dn3 = assign1480_e1477_d_n3;

        let (assign1490_e1487, assign1490_e1487_d_n3,) = {
    if (locals.var_guard130 != 0.0) {
        let assign1490_e1482: f64 = (p.p73 / locals.var_pa_t);
        let assign1490_e1484: f64 = (assign1490_e1482).powf(p.p74);
        let assign1490_e1485: f64 = (p.p72 * assign1490_e1484);
        (assign1490_e1485, (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * locals.var_pa_t_dn3) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * locals.var_pa_t_dn3) / (locals.var_pa_t * locals.var_pa_t))) / assign1490_e1482))) }),)
    } else {
        (locals.var_cja_t, locals.var_cja_t_dn3,)
    }
};
        locals.var_cja_t = assign1490_e1487;
        locals.var_cja_t_dn3 = assign1490_e1487_d_n3;

        let (assign1500_e1492, assign1500_e1492_d_n3,) = {
    if (locals.var_guard130 == 0.0) {
        (p.p73, 0.0,)
    } else {
        (locals.var_pa_t, locals.var_pa_t_dn3,)
    }
};
        locals.var_pa_t = assign1500_e1492;
        locals.var_pa_t_dn3 = assign1500_e1492_d_n3;

        let (assign1510_e1497, assign1510_e1497_d_n3,) = {
    if (locals.var_guard130 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_cja_t, locals.var_cja_t_dn3,)
    }
};
        locals.var_cja_t = assign1510_e1497;
        locals.var_cja_t_dn3 = assign1510_e1497_d_n3;

        let assign1520_e1500: f64 = if p.p79 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign1520_e1500;

        let (assign1530_e1528, assign1530_e1528_d_n3,) = {
    if (locals.var_guard133 != 0.0) {
        let assign1530_e1505: f64 = (locals.var_phi_t / locals.var_rt);
        let assign1530_e1506: f64 = (2.0 * assign1530_e1505);
        let assign1530_e1509: f64 = (0.5 * p.p80);
        let assign1530_e1511: f64 = (assign1530_e1509 * locals.var_rt);
        let assign1530_e1513: f64 = (assign1530_e1511 / locals.var_phi_t);
        let assign1530_e1514: f64 = (assign1530_e1513).exp();
        let assign1530_e1516: f64 = (-0.5);
        let assign1530_e1518: f64 = (assign1530_e1516 * p.p80);
        let assign1530_e1520: f64 = (assign1530_e1518 * locals.var_rt);
        let assign1530_e1522: f64 = (assign1530_e1520 / locals.var_phi_t);
        let assign1530_e1523: f64 = (assign1530_e1522).exp();
        let assign1530_e1524: f64 = (assign1530_e1514 - assign1530_e1523);
        let assign1530_e1525: f64 = (assign1530_e1524).ln();
        let assign1530_e1526: f64 = (assign1530_e1506 * assign1530_e1525);
        (assign1530_e1526, (((2.0 * (((locals.var_phi_t_dn3 * locals.var_rt) - (locals.var_phi_t * locals.var_rt_dn3)) / (locals.var_rt * locals.var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * locals.var_rt_dn3) * locals.var_phi_t) - (assign1530_e1511 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * locals.var_rt_dn3) * locals.var_phi_t) - (assign1530_e1520 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)))) / assign1530_e1524))),)
    } else {
        (locals.var_psiio__blk134, locals.var_psiio__blk134_dn3,)
    }
};
        locals.var_psiio__blk134 = assign1530_e1528;
        locals.var_psiio__blk134_dn3 = assign1530_e1528_d_n3;

        let (assign1540_e1547, assign1540_e1547_d_n3,) = {
    if (locals.var_guard133 != 0.0) {
        let assign1540_e1532: f64 = (locals.var_psiio__blk134 * locals.var_rt);
        let assign1540_e1535: f64 = (3.0 * locals.var_phi_t);
        let assign1540_e1537: f64 = (locals.var_rt).ln();
        let assign1540_e1538: f64 = (assign1540_e1535 * assign1540_e1537);
        let assign1540_e1539: f64 = (assign1540_e1532 - assign1540_e1538);
        let assign1540_e1543: f64 = (locals.var_rt - 1.0);
        let assign1540_e1544: f64 = (p.p90 * assign1540_e1543);
        let assign1540_e1545: f64 = (assign1540_e1539 - assign1540_e1544);
        (assign1540_e1545, ((((locals.var_psiio__blk134_dn3 * locals.var_rt) + (locals.var_psiio__blk134 * locals.var_rt_dn3)) - (((3.0 * locals.var_phi_t_dn3) * assign1540_e1537) + (assign1540_e1535 * (locals.var_rt_dn3 / locals.var_rt)))) - (p.p90 * locals.var_rt_dn3)),)
    } else {
        (locals.var_psiin__blk135, locals.var_psiin__blk135_dn3,)
    }
};
        locals.var_psiin__blk135 = assign1540_e1547;
        locals.var_psiin__blk135_dn3 = assign1540_e1547_d_n3;

        let (assign1550_e1571, assign1550_e1571_d_n3,) = {
    if (locals.var_guard133 != 0.0) {
        let assign1550_e1552: f64 = (2.0 * locals.var_phi_t);
        let assign1550_e1558: f64 = (-locals.var_psiin__blk135);
        let assign1550_e1560: f64 = (assign1550_e1558 / locals.var_phi_t);
        let assign1550_e1561: f64 = (assign1550_e1560).exp();
        let assign1550_e1562: f64 = (4.0 * assign1550_e1561);
        let assign1550_e1563: f64 = (1.0 + assign1550_e1562);
        let assign1550_e1564: f64 = (assign1550_e1563).sqrt();
        let assign1550_e1565: f64 = (1.0 + assign1550_e1564);
        let assign1550_e1566: f64 = (0.5 * assign1550_e1565);
        let assign1550_e1567: f64 = (assign1550_e1566).ln();
        let assign1550_e1568: f64 = (assign1550_e1552 * assign1550_e1567);
        let assign1550_e1569: f64 = (locals.var_psiin__blk135 + assign1550_e1568);
        (assign1550_e1569, (locals.var_psiin__blk135_dn3 + (((2.0 * locals.var_phi_t_dn3) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-locals.var_psiin__blk135_dn3) * locals.var_phi_t) - (assign1550_e1558 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))),)
    } else {
        (locals.var_pp_t, locals.var_pp_t_dn3,)
    }
};
        locals.var_pp_t = assign1550_e1571;
        locals.var_pp_t_dn3 = assign1550_e1571_d_n3;

        let (assign1560_e1581, assign1560_e1581_d_n3,) = {
    if (locals.var_guard133 != 0.0) {
        let assign1560_e1576: f64 = (p.p80 / locals.var_pp_t);
        let assign1560_e1578: f64 = (assign1560_e1576).powf(p.p81);
        let assign1560_e1579: f64 = (p.p79 * assign1560_e1578);
        (assign1560_e1579, (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * locals.var_pp_t_dn3) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * locals.var_pp_t_dn3) / (locals.var_pp_t * locals.var_pp_t))) / assign1560_e1576))) }),)
    } else {
        (locals.var_cjp_t, locals.var_cjp_t_dn3,)
    }
};
        locals.var_cjp_t = assign1560_e1581;
        locals.var_cjp_t_dn3 = assign1560_e1581_d_n3;

        let (assign1570_e1586, assign1570_e1586_d_n3,) = {
    if (locals.var_guard133 == 0.0) {
        (p.p80, 0.0,)
    } else {
        (locals.var_pp_t, locals.var_pp_t_dn3,)
    }
};
        locals.var_pp_t = assign1570_e1586;
        locals.var_pp_t_dn3 = assign1570_e1586_d_n3;

        let (assign1580_e1591, assign1580_e1591_d_n3,) = {
    if (locals.var_guard133 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_cjp_t, locals.var_cjp_t_dn3,)
    }
};
        locals.var_cjp_t = assign1580_e1591;
        locals.var_cjp_t_dn3 = assign1580_e1591_d_n3;

        let assign1610_e1607: f64 = if p.p83 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard136 = assign1610_e1607;

        let (assign1620_e1621, assign1620_e1621_d_n3,) = {
    if (locals.var_guard136 != 0.0) {
        let assign1620_e1615: f64 = (locals.var_dt * p.p106);
        let assign1620_e1616: f64 = (p.p105 + assign1620_e1615);
        let assign1620_e1617: f64 = (locals.var_dt * assign1620_e1616);
        let assign1620_e1618: f64 = (1.0 + assign1620_e1617);
        let assign1620_e1619: f64 = (p.p83 * assign1620_e1618);
        (assign1620_e1619, (p.p83 * ((locals.var_dt_dn3 * assign1620_e1616) + (locals.var_dt * (locals.var_dt_dn3 * p.p106)))),)
    } else {
        (locals.var_vbv_t, locals.var_vbv_t_dn3,)
    }
};
        locals.var_vbv_t = assign1620_e1621;
        locals.var_vbv_t_dn3 = assign1620_e1621_d_n3;

        let (assign1630_e1630, assign1630_e1630_d_n3,) = {
    if (locals.var_guard136 != 0.0) {
        let (assign1630_e1628, assign1630_e1628_d_n3,) = {
            if (locals.var_vbv_t > 0.0) {
                (locals.var_vbv_t, locals.var_vbv_t_dn3,)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign1630_e1628, assign1630_e1628_d_n3,)
    } else {
        (locals.var_vbv_t, locals.var_vbv_t_dn3,)
    }
};
        locals.var_vbv_t = assign1630_e1630;
        locals.var_vbv_t_dn3 = assign1630_e1630_d_n3;

        let (assign1640_e1640, assign1640_e1640_d_n3,) = {
    if (locals.var_guard136 != 0.0) {
        let assign1640_e1636: f64 = (p.p107 * locals.var_dt);
        let assign1640_e1637: f64 = (1.0 + assign1640_e1636);
        let assign1640_e1638: f64 = (p.p85 * assign1640_e1637);
        (assign1640_e1638, (p.p85 * (p.p107 * locals.var_dt_dn3)),)
    } else {
        (locals.var_nbv_t, locals.var_nbv_t_dn3,)
    }
};
        locals.var_nbv_t = assign1640_e1640;
        locals.var_nbv_t_dn3 = assign1640_e1640_d_n3;

        let (assign1650_e1659, assign1650_e1659_d_n3,) = {
    if (locals.var_guard136 != 0.0) {
        let assign1650_e1644: f64 = (locals.var_nbv_t * locals.var_phi_t);
        let assign1650_e1646: f64 = (-locals.var_vbv_t);
        let assign1650_e1649: f64 = (locals.var_nbv_t * locals.var_phi_t);
        let assign1650_e1650: f64 = (assign1650_e1646 / assign1650_e1649);
        let assign1650_e1651: f64 = (assign1650_e1650).exp();
        let assign1650_e1654: f64 = (p.p27 / p.p84);
        let assign1650_e1655: f64 = (assign1650_e1651 + assign1650_e1654);
        let assign1650_e1656: f64 = (assign1650_e1655).ln();
        let assign1650_e1657: f64 = (assign1650_e1644 * assign1650_e1656);
        (assign1650_e1657, ((((locals.var_nbv_t_dn3 * locals.var_phi_t) + (locals.var_nbv_t * locals.var_phi_t_dn3)) * assign1650_e1656) + (assign1650_e1644 * ((assign1650_e1651 * ((((-locals.var_vbv_t_dn3) * assign1650_e1649) - (assign1650_e1646 * ((locals.var_nbv_t_dn3 * locals.var_phi_t) + (locals.var_nbv_t * locals.var_phi_t_dn3)))) / (assign1650_e1649 * assign1650_e1649))) / assign1650_e1655))),)
    } else {
        (locals.var_vmax_b, locals.var_vmax_b_dn3,)
    }
};
        locals.var_vmax_b = assign1650_e1659;
        locals.var_vmax_b_dn3 = assign1650_e1659_d_n3;

        let (assign1660_e1664, assign1660_e1664_d_n3,) = {
    if (locals.var_guard136 == 0.0) {
        (p.p83, 0.0,)
    } else {
        (locals.var_vbv_t, locals.var_vbv_t_dn3,)
    }
};
        locals.var_vbv_t = assign1660_e1664;
        locals.var_vbv_t_dn3 = assign1660_e1664_d_n3;

        let (assign1670_e1669, assign1670_e1669_d_n3,) = {
    if (locals.var_guard136 == 0.0) {
        (p.p85, 0.0,)
    } else {
        (locals.var_nbv_t, locals.var_nbv_t_dn3,)
    }
};
        locals.var_nbv_t = assign1670_e1669;
        locals.var_nbv_t_dn3 = assign1670_e1669_d_n3;

        let (assign1680_e1674, assign1680_e1674_d_n3,) = {
    if (locals.var_guard136 == 0.0) {
        (1.0, 0.0,)
    } else {
        (locals.var_vmax_b, locals.var_vmax_b_dn3,)
    }
};
        locals.var_vmax_b = assign1680_e1674;
        locals.var_vmax_b_dn3 = assign1680_e1674_d_n3;

        let assign1690_e1680: f64 = if ((p.p60 > 0.0) && (p.p15 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign1690_e1680;

        let (assign1700_e1690, assign1700_e1690_d_n3,) = {
    if ((locals.var_guard137 != 0.0) && (p.p62 != 0.0)) {
        let assign1700_e1686: f64 = (p.p61 * locals.var_tcvsat);
        let assign1700_e1688: f64 = (assign1700_e1686 * locals.var_tcr);
        (assign1700_e1688, (((p.p61 * locals.var_tcvsat_dn3) * locals.var_tcr) + (assign1700_e1686 * locals.var_tcr_dn3)),)
    } else {
        (locals.var_ecorn_t, locals.var_ecorn_t_dn3,)
    }
};
        locals.var_ecorn_t = assign1700_e1690;
        locals.var_ecorn_t_dn3 = assign1700_e1690_d_n3;

        let (assign1710_e1700, assign1710_e1700_d_n3,) = {
    if ((locals.var_guard137 != 0.0) && (p.p62 != 0.0)) {
        let assign1710_e1696: f64 = (p.p60 * locals.var_tcvsat);
        let assign1710_e1698: f64 = (assign1710_e1696 * locals.var_tcr);
        (assign1710_e1698, (((p.p60 * locals.var_tcvsat_dn3) * locals.var_tcr) + (assign1710_e1696 * locals.var_tcr_dn3)),)
    } else {
        (locals.var_ecrit_t, locals.var_ecrit_t_dn3,)
    }
};
        locals.var_ecrit_t = assign1710_e1700;
        locals.var_ecrit_t_dn3 = assign1710_e1700_d_n3;

        let (assign1720_e1707, assign1720_e1707_d_n3,) = {
    if ((locals.var_guard137 != 0.0) && (p.p62 == 0.0)) {
        (p.p61, 0.0,)
    } else {
        (locals.var_ecorn_t, locals.var_ecorn_t_dn3,)
    }
};
        locals.var_ecorn_t = assign1720_e1707;
        locals.var_ecorn_t_dn3 = assign1720_e1707_d_n3;

        let (assign1730_e1714, assign1730_e1714_d_n3,) = {
    if ((locals.var_guard137 != 0.0) && (p.p62 == 0.0)) {
        (p.p60, 0.0,)
    } else {
        (locals.var_ecrit_t, locals.var_ecrit_t_dn3,)
    }
};
        locals.var_ecrit_t = assign1730_e1714;
        locals.var_ecrit_t_dn3 = assign1730_e1714_d_n3;

        let (assign1740_e1737, assign1740_e1737_d_n3,) = {
    if (locals.var_guard137 != 0.0) {
        let assign1740_e1718: f64 = (locals.var_ecorn_t * locals.var_ecorn_t);
        let assign1740_e1721: f64 = (4.0 * p.p65);
        let assign1740_e1723: f64 = (assign1740_e1721 * p.p65);
        let assign1740_e1725: f64 = (assign1740_e1723 * locals.var_ecrit_t);
        let assign1740_e1727: f64 = (assign1740_e1725 * locals.var_ecrit_t);
        let assign1740_e1728: f64 = (assign1740_e1718 + assign1740_e1727);
        let assign1740_e1729: f64 = (assign1740_e1728).sqrt();
        let assign1740_e1732: f64 = (2.0 * p.p65);
        let assign1740_e1734: f64 = (assign1740_e1732 * locals.var_ecrit_t);
        let assign1740_e1735: f64 = (assign1740_e1729 - assign1740_e1734);
        (assign1740_e1735, (((((locals.var_ecorn_t_dn3 * locals.var_ecorn_t) + (locals.var_ecorn_t * locals.var_ecorn_t_dn3)) + (((assign1740_e1723 * locals.var_ecrit_t_dn3) * locals.var_ecrit_t) + (assign1740_e1725 * locals.var_ecrit_t_dn3))) / (2.0 * assign1740_e1729)) - (assign1740_e1732 * locals.var_ecrit_t_dn3)),)
    } else {
        (locals.var_ecrneff, locals.var_ecrneff_dn3,)
    }
};
        locals.var_ecrneff = assign1740_e1737;
        locals.var_ecrneff_dn3 = assign1740_e1737_d_n3;

        let (assign1750_e1745, assign1750_e1745_d_n3,) = {
    if (locals.var_guard137 != 0.0) {
        let assign1750_e1741: f64 = (p.p65 * locals.var_ecrneff);
        let assign1750_e1743: f64 = (assign1750_e1741 / locals.var_ecrit_t);
        (assign1750_e1743, ((((p.p65 * locals.var_ecrneff_dn3) * locals.var_ecrit_t) - (assign1750_e1741 * locals.var_ecrit_t_dn3)) / (locals.var_ecrit_t * locals.var_ecrit_t)),)
    } else {
        (locals.var_dufctr, locals.var_dufctr_dn3,)
    }
};
        locals.var_dufctr = assign1750_e1745;
        locals.var_dufctr_dn3 = assign1750_e1745_d_n3;

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1760_e1760, assign1760_e1760_d_n3,) = {
    if (locals.var_guard137 != 0.0) {
        let assign1760_e1749: f64 = (locals.var_ecrneff * locals.var_ecrneff);
        let assign1760_e1752: f64 = (locals.var_ecrit_t * locals.var_ecrit_t);
        let assign1760_e1753: f64 = (assign1760_e1749 / assign1760_e1752);
        let assign1760_e1756: f64 = (4.0 * locals.var_dufctr);
        let assign1760_e1757: f64 = (assign1760_e1753 + assign1760_e1756);
        let assign1760_e1758: f64 = (assign1760_e1757).sqrt();
        (assign1760_e1758, (((((((locals.var_ecrneff_dn3 * locals.var_ecrneff) + (locals.var_ecrneff * locals.var_ecrneff_dn3)) * assign1760_e1752) - (assign1760_e1749 * ((locals.var_ecrit_t_dn3 * locals.var_ecrit_t) + (locals.var_ecrit_t * locals.var_ecrit_t_dn3)))) / (assign1760_e1752 * assign1760_e1752)) + (4.0 * locals.var_dufctr_dn3)) / (2.0 * assign1760_e1758)),)
    } else {
        (locals.var_uoff, locals.var_uoff_dn3,)
    }
};
        locals.var_uoff = assign1760_e1760;
        locals.var_uoff_dn3 = assign1760_e1760_d_n3;

        let (assign1770_e1766, assign1770_e1766_d_n3,) = {
    if (locals.var_guard137 != 0.0) {
        let assign1770_e1764: f64 = (locals.var_ecrit_t - locals.var_ecorn_t);
        (assign1770_e1764, (locals.var_ecrit_t_dn3 - locals.var_ecorn_t_dn3),)
    } else {
        (locals.var_de, locals.var_de_dn3,)
    }
};
        locals.var_de = assign1770_e1766;
        locals.var_de_dn3 = assign1770_e1766_d_n3;

        let (assign1780_e1772, assign1780_e1772_d_n3,) = {
    if (locals.var_guard137 != 0.0) {
        let assign1780_e1770: f64 = (1.0 / locals.var_ecrit_t);
        (assign1780_e1770, (-(locals.var_ecrit_t_dn3 / (locals.var_ecrit_t * locals.var_ecrit_t))),)
    } else {
        (locals.var_iecrit, locals.var_iecrit_dn3,)
    }
};
        locals.var_iecrit = assign1780_e1772;
        locals.var_iecrit_dn3 = assign1780_e1772_d_n3;

        let (assign1790_e1777, assign1790_e1777_d_n3,) = {
    if (locals.var_guard137 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_ecrneff, locals.var_ecrneff_dn3,)
    }
};
        locals.var_ecrneff = assign1790_e1777;
        locals.var_ecrneff_dn3 = assign1790_e1777_d_n3;

        let (assign1800_e1782, assign1800_e1782_d_n3,) = {
    if (locals.var_guard137 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_dufctr, locals.var_dufctr_dn3,)
    }
};
        locals.var_dufctr = assign1800_e1782;
        locals.var_dufctr_dn3 = assign1800_e1782_d_n3;

        let (assign1810_e1787, assign1810_e1787_d_n3,) = {
    if (locals.var_guard137 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_uoff, locals.var_uoff_dn3,)
    }
};
        locals.var_uoff = assign1810_e1787;
        locals.var_uoff_dn3 = assign1810_e1787_d_n3;

        let (assign1820_e1792, assign1820_e1792_d_n3,) = {
    if (locals.var_guard137 == 0.0) {
        (1000.0, 0.0,)
    } else {
        (locals.var_de, locals.var_de_dn3,)
    }
};
        locals.var_de = assign1820_e1792;
        locals.var_de_dn3 = assign1820_e1792_d_n3;

        let (assign1830_e1797, assign1830_e1797_d_n3,) = {
    if (locals.var_guard137 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_iecrit, locals.var_iecrit_dn3,)
    }
};
        locals.var_iecrit = assign1830_e1797;
        locals.var_iecrit_dn3 = assign1830_e1797_d_n3;

        let assign1840_e1800: f64 = (locals.var_leffe_um * locals.var_de);
        locals.var_lde = assign1840_e1800;
        locals.var_lde_dn3 = (locals.var_leffe_um * locals.var_de_dn3);

        let assign1850_e1803: f64 = if locals.var_lde > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign1850_e1803;

        let (assign1860_e1807, assign1860_e1807_d_n3,) = {
    if (locals.var_guard138 != 0.0) {
        (100000.0, 0.0,)
    } else {
        (locals.var_lde, locals.var_lde_dn3,)
    }
};
        locals.var_lde = assign1860_e1807;
        locals.var_lde_dn3 = assign1860_e1807_d_n3;

        let assign1870_e1810: f64 = if locals.var_vrb < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign1870_e1810;

        let (assign1880_e1815,) = {
    if (locals.var_guard189 != 0.0) {
        let assign1880_e1813: f64 = (-1.0);
        (assign1880_e1813,)
    } else {
        (locals.var_sdflip,)
    }
};
        locals.var_sdflip = assign1880_e1815;

        let (assign1890_e1820, assign1890_e1820_d_n1, assign1890_e1820_d_n4, assign1890_e1820_d_n5,) = {
    if (locals.var_guard189 != 0.0) {
        let assign1890_e1818: f64 = (-locals.var_vc2);
        (assign1890_e1818, (-locals.var_vc2_dn1), 0.0, (-locals.var_vc2_dn5),)
    } else {
        (locals.var_v1ci, locals.var_v1ci_dn1, locals.var_v1ci_dn4, locals.var_v1ci_dn5,)
    }
};
        locals.var_v1ci = assign1890_e1820;
        locals.var_v1ci_dn1 = assign1890_e1820_d_n1;
        locals.var_v1ci_dn4 = assign1890_e1820_d_n4;
        locals.var_v1ci_dn5 = assign1890_e1820_d_n5;

        let (assign1900_e1825, assign1900_e1825_d_n4, assign1900_e1825_d_n5,) = {
    if (locals.var_guard189 != 0.0) {
        let assign1900_e1823: f64 = (-locals.var_vrb);
        (assign1900_e1823, (-locals.var_vrb_dn4), (-locals.var_vrb_dn5),)
    } else {
        (locals.var_vrbi, locals.var_vrbi_dn4, locals.var_vrbi_dn5,)
    }
};
        locals.var_vrbi = assign1900_e1825;
        locals.var_vrbi_dn4 = assign1900_e1825_d_n4;
        locals.var_vrbi_dn5 = assign1900_e1825_d_n5;

        let (assign1910_e1830,) = {
    if (locals.var_guard189 == 0.0) {
        (1.0,)
    } else {
        (locals.var_sdflip,)
    }
};
        locals.var_sdflip = assign1910_e1830;

        let (assign1920_e1836, assign1920_e1836_d_n1, assign1920_e1836_d_n4, assign1920_e1836_d_n5,) = {
    if (locals.var_guard189 == 0.0) {
        let assign1920_e1834: f64 = (-locals.var_vc1);
        (assign1920_e1834, (-locals.var_vc1_dn1), (-locals.var_vc1_dn4), 0.0,)
    } else {
        (locals.var_v1ci, locals.var_v1ci_dn1, locals.var_v1ci_dn4, locals.var_v1ci_dn5,)
    }
};
        locals.var_v1ci = assign1920_e1836;
        locals.var_v1ci_dn1 = assign1920_e1836_d_n1;
        locals.var_v1ci_dn4 = assign1920_e1836_d_n4;
        locals.var_v1ci_dn5 = assign1920_e1836_d_n5;

        let (assign1930_e1841, assign1930_e1841_d_n4, assign1930_e1841_d_n5,) = {
    if (locals.var_guard189 == 0.0) {
        (locals.var_vrb, locals.var_vrb_dn4, locals.var_vrb_dn5,)
    } else {
        (locals.var_vrbi, locals.var_vrbi_dn4, locals.var_vrbi_dn5,)
    }
};
        locals.var_vrbi = assign1930_e1841;
        locals.var_vrbi_dn4 = assign1930_e1841_d_n4;
        locals.var_vrbi_dn5 = assign1930_e1841_d_n5;

        let assign1940_e1844: f64 = if locals.var_v1ci > locals.var_vpoe { 1.0 } else { 0.0 };
        locals.var_guard190 = assign1940_e1844;

        let (assign1950_e1860, assign1950_e1860_d_n1, assign1950_e1860_d_n3, assign1950_e1860_d_n4, assign1950_e1860_d_n5,) = {
    if (locals.var_guard190 != 0.0) {
        let assign1950_e1851: f64 = (locals.var_vpoe - locals.var_v1ci);
        let assign1950_e1853: f64 = (assign1950_e1851 / locals.var_nsteff);
        let assign1950_e1854: f64 = (assign1950_e1853).exp();
        let assign1950_e1855: f64 = (1.0 + assign1950_e1854);
        let assign1950_e1856: f64 = (assign1950_e1855).ln();
        let assign1950_e1857: f64 = (locals.var_nsteff * assign1950_e1856);
        let assign1950_e1858: f64 = (locals.var_vpoe - assign1950_e1857);
        (assign1950_e1858, (-(locals.var_nsteff * ((assign1950_e1854 * ((-locals.var_v1ci_dn1) / locals.var_nsteff)) / assign1950_e1855))), (locals.var_vpoe_dn3 - ((locals.var_nsteff_dn3 * assign1950_e1856) + (locals.var_nsteff * ((assign1950_e1854 * (((locals.var_vpoe_dn3 * locals.var_nsteff) - (assign1950_e1851 * locals.var_nsteff_dn3)) / (locals.var_nsteff * locals.var_nsteff))) / assign1950_e1855)))), (-(locals.var_nsteff * ((assign1950_e1854 * ((-locals.var_v1ci_dn4) / locals.var_nsteff)) / assign1950_e1855))), (-(locals.var_nsteff * ((assign1950_e1854 * ((-locals.var_v1ci_dn5) / locals.var_nsteff)) / assign1950_e1855))),)
    } else {
        (locals.var_v1cl, locals.var_v1cl_dn1, locals.var_v1cl_dn3, locals.var_v1cl_dn4, locals.var_v1cl_dn5,)
    }
};
        locals.var_v1cl = assign1950_e1860;
        locals.var_v1cl_dn1 = assign1950_e1860_d_n1;
        locals.var_v1cl_dn3 = assign1950_e1860_d_n3;
        locals.var_v1cl_dn4 = assign1950_e1860_d_n4;
        locals.var_v1cl_dn5 = assign1950_e1860_d_n5;

        let (assign1960_e1877, assign1960_e1877_d_n1, assign1960_e1877_d_n3, assign1960_e1877_d_n4, assign1960_e1877_d_n5,) = {
    if (locals.var_guard190 == 0.0) {
        let assign1960_e1868: f64 = (locals.var_v1ci - locals.var_vpoe);
        let assign1960_e1870: f64 = (assign1960_e1868 / locals.var_nsteff);
        let assign1960_e1871: f64 = (assign1960_e1870).exp();
        let assign1960_e1872: f64 = (1.0 + assign1960_e1871);
        let assign1960_e1873: f64 = (assign1960_e1872).ln();
        let assign1960_e1874: f64 = (locals.var_nsteff * assign1960_e1873);
        let assign1960_e1875: f64 = (locals.var_v1ci - assign1960_e1874);
        (assign1960_e1875, (locals.var_v1ci_dn1 - (locals.var_nsteff * ((assign1960_e1871 * (locals.var_v1ci_dn1 / locals.var_nsteff)) / assign1960_e1872))), (-((locals.var_nsteff_dn3 * assign1960_e1873) + (locals.var_nsteff * ((assign1960_e1871 * ((((-locals.var_vpoe_dn3) * locals.var_nsteff) - (assign1960_e1868 * locals.var_nsteff_dn3)) / (locals.var_nsteff * locals.var_nsteff))) / assign1960_e1872)))), (locals.var_v1ci_dn4 - (locals.var_nsteff * ((assign1960_e1871 * (locals.var_v1ci_dn4 / locals.var_nsteff)) / assign1960_e1872))), (locals.var_v1ci_dn5 - (locals.var_nsteff * ((assign1960_e1871 * (locals.var_v1ci_dn5 / locals.var_nsteff)) / assign1960_e1872))),)
    } else {
        (locals.var_v1cl, locals.var_v1cl_dn1, locals.var_v1cl_dn3, locals.var_v1cl_dn4, locals.var_v1cl_dn5,)
    }
};
        locals.var_v1cl = assign1960_e1877;
        locals.var_v1cl_dn1 = assign1960_e1877_d_n1;
        locals.var_v1cl_dn3 = assign1960_e1877_d_n3;
        locals.var_v1cl_dn4 = assign1960_e1877_d_n4;
        locals.var_v1cl_dn5 = assign1960_e1877_d_n5;

        let assign1970_e1880: f64 = (-0.4);
        let assign1970_e1885: f64 = (locals.var_vpoe - locals.var_v1cl);
        let (assign1970_e1891,) = {
    if (locals.var_vrbi < assign1970_e1885) {
        (locals.var_vrbi,)
    } else {
        let assign1970_e1890: f64 = (locals.var_vpoe - locals.var_v1cl);
        (assign1970_e1890,)
    }
};
        let assign1970_e1892: f64 = (locals.var_dp_i + assign1970_e1891);
        let assign1970_e1893: f64 = (assign1970_e1880 * assign1970_e1892);
        let assign1970_e1894: f64 = if locals.var_v1cl < assign1970_e1893 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign1970_e1894;

        let (assign1980_e1914, assign1980_e1914_d_n1, assign1980_e1914_d_n3, assign1980_e1914_d_n4, assign1980_e1914_d_n5,) = {
    if ((p.p63 != 0.0) && (locals.var_guard191 != 0.0)) {
        let assign1980_e1899: f64 = (-0.4);
        let assign1980_e1904: f64 = (locals.var_vpoe - locals.var_v1cl);
        let (assign1980_e1910, assign1980_e1910_d_n1, assign1980_e1910_d_n3, assign1980_e1910_d_n4, assign1980_e1910_d_n5,) = {
            if (locals.var_vrbi < assign1980_e1904) {
                (locals.var_vrbi, 0.0, 0.0, locals.var_vrbi_dn4, locals.var_vrbi_dn5,)
            } else {
                let assign1980_e1909: f64 = (locals.var_vpoe - locals.var_v1cl);
                (assign1980_e1909, (-locals.var_v1cl_dn1), (locals.var_vpoe_dn3 - locals.var_v1cl_dn3), (-locals.var_v1cl_dn4), (-locals.var_v1cl_dn5),)
            }
        };
        let assign1980_e1911: f64 = (locals.var_dp_i + assign1980_e1910);
        let assign1980_e1912: f64 = (assign1980_e1899 * assign1980_e1911);
        (assign1980_e1912, (assign1980_e1899 * assign1980_e1910_d_n1), (assign1980_e1899 * (locals.var_dp_i_dn3 + assign1980_e1910_d_n3)), (assign1980_e1899 * assign1980_e1910_d_n4), (assign1980_e1899 * assign1980_e1910_d_n5),)
    } else {
        (locals.var_v1c, locals.var_v1c_dn1, locals.var_v1c_dn3, locals.var_v1c_dn4, locals.var_v1c_dn5,)
    }
};
        locals.var_v1c = assign1980_e1914;
        locals.var_v1c_dn1 = assign1980_e1914_d_n1;
        locals.var_v1c_dn3 = assign1980_e1914_d_n3;
        locals.var_v1c_dn4 = assign1980_e1914_d_n4;
        locals.var_v1c_dn5 = assign1980_e1914_d_n5;

        let (assign1990_e1921, assign1990_e1921_d_n1, assign1990_e1921_d_n3, assign1990_e1921_d_n4, assign1990_e1921_d_n5,) = {
    if ((p.p63 != 0.0) && (locals.var_guard191 == 0.0)) {
        (locals.var_v1cl, locals.var_v1cl_dn1, locals.var_v1cl_dn3, locals.var_v1cl_dn4, locals.var_v1cl_dn5,)
    } else {
        (locals.var_v1c, locals.var_v1c_dn1, locals.var_v1c_dn3, locals.var_v1c_dn4, locals.var_v1c_dn5,)
    }
};
        locals.var_v1c = assign1990_e1921;
        locals.var_v1c_dn1 = assign1990_e1921_d_n1;
        locals.var_v1c_dn3 = assign1990_e1921_d_n3;
        locals.var_v1c_dn4 = assign1990_e1921_d_n4;
        locals.var_v1c_dn5 = assign1990_e1921_d_n5;

        let assign2000_e1924: f64 = (-0.4);
        let assign2000_e1926: f64 = (assign2000_e1924 * locals.var_dp_i);
        let assign2000_e1927: f64 = if locals.var_v1cl < assign2000_e1926 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign2000_e1927;

        let (assign2010_e1937, assign2010_e1937_d_n1, assign2010_e1937_d_n3, assign2010_e1937_d_n4, assign2010_e1937_d_n5,) = {
    if ((p.p63 == 0.0) && (locals.var_guard192 != 0.0)) {
        let assign2010_e1933: f64 = (-0.4);
        let assign2010_e1935: f64 = (assign2010_e1933 * locals.var_dp_i);
        (assign2010_e1935, 0.0, (assign2010_e1933 * locals.var_dp_i_dn3), 0.0, 0.0,)
    } else {
        (locals.var_v1c, locals.var_v1c_dn1, locals.var_v1c_dn3, locals.var_v1c_dn4, locals.var_v1c_dn5,)
    }
};
        locals.var_v1c = assign2010_e1937;
        locals.var_v1c_dn1 = assign2010_e1937_d_n1;
        locals.var_v1c_dn3 = assign2010_e1937_d_n3;
        locals.var_v1c_dn4 = assign2010_e1937_d_n4;
        locals.var_v1c_dn5 = assign2010_e1937_d_n5;

        let (assign2020_e1945, assign2020_e1945_d_n1, assign2020_e1945_d_n3, assign2020_e1945_d_n4, assign2020_e1945_d_n5,) = {
    if ((p.p63 == 0.0) && (locals.var_guard192 == 0.0)) {
        (locals.var_v1cl, locals.var_v1cl_dn1, locals.var_v1cl_dn3, locals.var_v1cl_dn4, locals.var_v1cl_dn5,)
    } else {
        (locals.var_v1c, locals.var_v1c_dn1, locals.var_v1c_dn3, locals.var_v1c_dn4, locals.var_v1c_dn5,)
    }
};
        locals.var_v1c = assign2020_e1945;
        locals.var_v1c_dn1 = assign2020_e1945_d_n1;
        locals.var_v1c_dn3 = assign2020_e1945_d_n3;
        locals.var_v1c_dn4 = assign2020_e1945_d_n4;
        locals.var_v1c_dn5 = assign2020_e1945_d_n5;

        let assign2030_e1949: f64 = (2.0 * locals.var_v1c);
        let assign2030_e1950: f64 = (locals.var_dp_i + assign2030_e1949);
        locals.var_pe = assign2030_e1950;
        locals.var_pe_dn1 = (2.0 * locals.var_v1c_dn1);
        locals.var_pe_dn3 = (locals.var_dp_i_dn3 + (2.0 * locals.var_v1c_dn3));
        locals.var_pe_dn4 = (2.0 * locals.var_v1c_dn4);
        locals.var_pe_dn5 = (2.0 * locals.var_v1c_dn5);

        let assign2040_e1953: f64 = if locals.var_iecrit > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign2040_e1953;

        let (assign2050_e1963, assign2050_e1963_d_n1, assign2050_e1963_d_n3, assign2050_e1963_d_n4, assign2050_e1963_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2050_e1957: f64 = (locals.var_dfsq * locals.var_pe);
        let assign2050_e1959: f64 = (assign2050_e1957 * locals.var_pe);
        let assign2050_e1961: f64 = (assign2050_e1959 - locals.var_pe);
        (assign2050_e1961, ((((locals.var_dfsq * locals.var_pe_dn1) * locals.var_pe) + (assign2050_e1957 * locals.var_pe_dn1)) - locals.var_pe_dn1), (((((locals.var_dfsq_dn3 * locals.var_pe) + (locals.var_dfsq * locals.var_pe_dn3)) * locals.var_pe) + (assign2050_e1957 * locals.var_pe_dn3)) - locals.var_pe_dn3), ((((locals.var_dfsq * locals.var_pe_dn4) * locals.var_pe) + (assign2050_e1957 * locals.var_pe_dn4)) - locals.var_pe_dn4), ((((locals.var_dfsq * locals.var_pe_dn5) * locals.var_pe) + (assign2050_e1957 * locals.var_pe_dn5)) - locals.var_pe_dn5),)
    } else {
        (locals.var_a0, locals.var_a0_dn1, locals.var_a0_dn3, locals.var_a0_dn4, locals.var_a0_dn5,)
    }
};
        locals.var_a0 = assign2050_e1963;
        locals.var_a0_dn1 = assign2050_e1963_d_n1;
        locals.var_a0_dn3 = assign2050_e1963_d_n3;
        locals.var_a0_dn4 = assign2050_e1963_d_n4;
        locals.var_a0_dn5 = assign2050_e1963_d_n5;

        let (assign2060_e1974, assign2060_e1974_d_n1, assign2060_e1974_d_n3, assign2060_e1974_d_n4, assign2060_e1974_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2060_e1966: f64 = (-1.0);
        let assign2060_e1969: f64 = (3.0 * locals.var_dfsq);
        let assign2060_e1971: f64 = (assign2060_e1969 * locals.var_pe);
        let assign2060_e1972: f64 = (assign2060_e1966 + assign2060_e1971);
        (assign2060_e1972, (assign2060_e1969 * locals.var_pe_dn1), (((3.0 * locals.var_dfsq_dn3) * locals.var_pe) + (assign2060_e1969 * locals.var_pe_dn3)), (assign2060_e1969 * locals.var_pe_dn4), (assign2060_e1969 * locals.var_pe_dn5),)
    } else {
        (p.p3, locals.var_a1_dn1, locals.var_a1_dn3, locals.var_a1_dn4, locals.var_a1_dn5,)
    }
};
        locals.var_a1 = assign2060_e1974;
        locals.var_a1_dn1 = assign2060_e1974_d_n1;
        locals.var_a1_dn3 = assign2060_e1974_d_n3;
        locals.var_a1_dn4 = assign2060_e1974_d_n4;
        locals.var_a1_dn5 = assign2060_e1974_d_n5;

        let (assign2070_e1986, assign2070_e1986_d_n1, assign2070_e1986_d_n3, assign2070_e1986_d_n4, assign2070_e1986_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2070_e1979: f64 = (9.0 / 4.0);
        let assign2070_e1982: f64 = (locals.var_pe / locals.var_lde);
        let assign2070_e1983: f64 = (assign2070_e1979 + assign2070_e1982);
        let assign2070_e1984: f64 = (locals.var_dfsq * assign2070_e1983);
        (assign2070_e1984, (locals.var_dfsq * (locals.var_pe_dn1 / locals.var_lde)), ((locals.var_dfsq_dn3 * assign2070_e1983) + (locals.var_dfsq * (((locals.var_pe_dn3 * locals.var_lde) - (locals.var_pe * locals.var_lde_dn3)) / (locals.var_lde * locals.var_lde)))), (locals.var_dfsq * (locals.var_pe_dn4 / locals.var_lde)), (locals.var_dfsq * (locals.var_pe_dn5 / locals.var_lde)),)
    } else {
        (p.p6, locals.var_a2_dn1, locals.var_a2_dn3, locals.var_a2_dn4, locals.var_a2_dn5,)
    }
};
        locals.var_a2 = assign2070_e1986;
        locals.var_a2_dn1 = assign2070_e1986_d_n1;
        locals.var_a2_dn3 = assign2070_e1986_d_n3;
        locals.var_a2_dn4 = assign2070_e1986_d_n4;
        locals.var_a2_dn5 = assign2070_e1986_d_n5;

        let (assign2080_e1994, assign2080_e1994_d_n3,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2080_e1990: f64 = (1.5 * locals.var_dfsq);
        let assign2080_e1992: f64 = (assign2080_e1990 / locals.var_lde);
        (assign2080_e1992, ((((1.5 * locals.var_dfsq_dn3) * locals.var_lde) - (assign2080_e1990 * locals.var_lde_dn3)) / (locals.var_lde * locals.var_lde)),)
    } else {
        (locals.var_a3, locals.var_a3_dn3,)
    }
};
        locals.var_a3 = assign2080_e1994;
        locals.var_a3_dn3 = assign2080_e1994_d_n3;

        let (assign2090_e2004, assign2090_e2004_d_n3,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2090_e1998: f64 = (4.0 * locals.var_lde);
        let assign2090_e2000: f64 = (assign2090_e1998 * locals.var_lde);
        let assign2090_e2002: f64 = (assign2090_e2000 / locals.var_dfsq);
        (assign2090_e2002, ((((((4.0 * locals.var_lde_dn3) * locals.var_lde) + (assign2090_e1998 * locals.var_lde_dn3)) * locals.var_dfsq) - (assign2090_e2000 * locals.var_dfsq_dn3)) / (locals.var_dfsq * locals.var_dfsq)),)
    } else {
        (locals.var_a4, locals.var_a4_dn3,)
    }
};
        locals.var_a4 = assign2090_e2004;
        locals.var_a4_dn3 = assign2090_e2004_d_n3;

        let (assign2100_e2010, assign2100_e2010_d_n1, assign2100_e2010_d_n3, assign2100_e2010_d_n4, assign2100_e2010_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2100_e2008: f64 = (locals.var_a0 * locals.var_a4);
        (assign2100_e2008, (locals.var_a0_dn1 * locals.var_a4), ((locals.var_a0_dn3 * locals.var_a4) + (locals.var_a0 * locals.var_a4_dn3)), (locals.var_a0_dn4 * locals.var_a4), (locals.var_a0_dn5 * locals.var_a4),)
    } else {
        (locals.var_dvar, locals.var_dvar_dn1, locals.var_dvar_dn3, locals.var_dvar_dn4, locals.var_dvar_dn5,)
    }
};
        locals.var_dvar = assign2100_e2010;
        locals.var_dvar_dn1 = assign2100_e2010_d_n1;
        locals.var_dvar_dn3 = assign2100_e2010_d_n3;
        locals.var_dvar_dn4 = assign2100_e2010_d_n4;
        locals.var_dvar_dn5 = assign2100_e2010_d_n5;

        let (assign2110_e2016, assign2110_e2016_d_n1, assign2110_e2016_d_n3, assign2110_e2016_d_n4, assign2110_e2016_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2110_e2014: f64 = (p.p3 * locals.var_a4);
        (assign2110_e2014, (locals.var_a1_dn1 * locals.var_a4), ((locals.var_a1_dn3 * locals.var_a4) + (p.p3 * locals.var_a4_dn3)), (locals.var_a1_dn4 * locals.var_a4), (locals.var_a1_dn5 * locals.var_a4),)
    } else {
        (locals.var_cvar, locals.var_cvar_dn1, locals.var_cvar_dn3, locals.var_cvar_dn4, locals.var_cvar_dn5,)
    }
};
        locals.var_cvar = assign2110_e2016;
        locals.var_cvar_dn1 = assign2110_e2016_d_n1;
        locals.var_cvar_dn3 = assign2110_e2016_d_n3;
        locals.var_cvar_dn4 = assign2110_e2016_d_n4;
        locals.var_cvar_dn5 = assign2110_e2016_d_n5;

        let (assign2120_e2022, assign2120_e2022_d_n1, assign2120_e2022_d_n3, assign2120_e2022_d_n4, assign2120_e2022_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2120_e2020: f64 = (p.p6 * locals.var_a4);
        (assign2120_e2020, (locals.var_a2_dn1 * locals.var_a4), ((locals.var_a2_dn3 * locals.var_a4) + (p.p6 * locals.var_a4_dn3)), (locals.var_a2_dn4 * locals.var_a4), (locals.var_a2_dn5 * locals.var_a4),)
    } else {
        (locals.var_bvar, locals.var_bvar_dn1, locals.var_bvar_dn3, locals.var_bvar_dn4, locals.var_bvar_dn5,)
    }
};
        locals.var_bvar = assign2120_e2022;
        locals.var_bvar_dn1 = assign2120_e2022_d_n1;
        locals.var_bvar_dn3 = assign2120_e2022_d_n3;
        locals.var_bvar_dn4 = assign2120_e2022_d_n4;
        locals.var_bvar_dn5 = assign2120_e2022_d_n5;

        let (assign2130_e2028, assign2130_e2028_d_n3,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2130_e2026: f64 = (locals.var_a3 * locals.var_a4);
        (assign2130_e2026, ((locals.var_a3_dn3 * locals.var_a4) + (locals.var_a3 * locals.var_a4_dn3)),)
    } else {
        (locals.var_avar, locals.var_avar_dn3,)
    }
};
        locals.var_avar = assign2130_e2028;
        locals.var_avar_dn3 = assign2130_e2028_d_n3;

        let (assign2140_e2034, assign2140_e2034_d_n3,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2140_e2032: f64 = (locals.var_avar * locals.var_avar);
        (assign2140_e2032, ((locals.var_avar_dn3 * locals.var_avar) + (locals.var_avar * locals.var_avar_dn3)),)
    } else {
        (locals.var_asq, locals.var_asq_dn3,)
    }
};
        locals.var_asq = assign2140_e2034;
        locals.var_asq_dn3 = assign2140_e2034_d_n3;

        let (assign2150_e2039, assign2150_e2039_d_n1, assign2150_e2039_d_n3, assign2150_e2039_d_n4, assign2150_e2039_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2150_e2037: f64 = (-locals.var_bvar);
        (assign2150_e2037, (-locals.var_bvar_dn1), (-locals.var_bvar_dn3), (-locals.var_bvar_dn4), (-locals.var_bvar_dn5),)
    } else {
        (locals.var_pvar, locals.var_pvar_dn1, locals.var_pvar_dn3, locals.var_pvar_dn4, locals.var_pvar_dn5,)
    }
};
        locals.var_pvar = assign2150_e2039;
        locals.var_pvar_dn1 = assign2150_e2039_d_n1;
        locals.var_pvar_dn3 = assign2150_e2039_d_n3;
        locals.var_pvar_dn4 = assign2150_e2039_d_n4;
        locals.var_pvar_dn5 = assign2150_e2039_d_n5;

        let (assign2160_e2049, assign2160_e2049_d_n1, assign2160_e2049_d_n3, assign2160_e2049_d_n4, assign2160_e2049_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2160_e2043: f64 = (locals.var_avar * locals.var_cvar);
        let assign2160_e2046: f64 = (4.0 * locals.var_dvar);
        let assign2160_e2047: f64 = (assign2160_e2043 - assign2160_e2046);
        (assign2160_e2047, ((locals.var_avar * locals.var_cvar_dn1) - (4.0 * locals.var_dvar_dn1)), (((locals.var_avar_dn3 * locals.var_cvar) + (locals.var_avar * locals.var_cvar_dn3)) - (4.0 * locals.var_dvar_dn3)), ((locals.var_avar * locals.var_cvar_dn4) - (4.0 * locals.var_dvar_dn4)), ((locals.var_avar * locals.var_cvar_dn5) - (4.0 * locals.var_dvar_dn5)),)
    } else {
        (locals.var_qvar, locals.var_qvar_dn1, locals.var_qvar_dn3, locals.var_qvar_dn4, locals.var_qvar_dn5,)
    }
};
        locals.var_qvar = assign2160_e2049;
        locals.var_qvar_dn1 = assign2160_e2049_d_n1;
        locals.var_qvar_dn3 = assign2160_e2049_d_n3;
        locals.var_qvar_dn4 = assign2160_e2049_d_n4;
        locals.var_qvar_dn5 = assign2160_e2049_d_n5;

        let (assign2170_e2065, assign2170_e2065_d_n1, assign2170_e2065_d_n3, assign2170_e2065_d_n4, assign2170_e2065_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2170_e2053: f64 = (4.0 * locals.var_bvar);
        let assign2170_e2055: f64 = (assign2170_e2053 * locals.var_dvar);
        let assign2170_e2058: f64 = (locals.var_cvar * locals.var_cvar);
        let assign2170_e2059: f64 = (assign2170_e2055 - assign2170_e2058);
        let assign2170_e2062: f64 = (locals.var_dvar * locals.var_asq);
        let assign2170_e2063: f64 = (assign2170_e2059 - assign2170_e2062);
        (assign2170_e2063, (((((4.0 * locals.var_bvar_dn1) * locals.var_dvar) + (assign2170_e2053 * locals.var_dvar_dn1)) - ((locals.var_cvar_dn1 * locals.var_cvar) + (locals.var_cvar * locals.var_cvar_dn1))) - (locals.var_dvar_dn1 * locals.var_asq)), (((((4.0 * locals.var_bvar_dn3) * locals.var_dvar) + (assign2170_e2053 * locals.var_dvar_dn3)) - ((locals.var_cvar_dn3 * locals.var_cvar) + (locals.var_cvar * locals.var_cvar_dn3))) - ((locals.var_dvar_dn3 * locals.var_asq) + (locals.var_dvar * locals.var_asq_dn3))), (((((4.0 * locals.var_bvar_dn4) * locals.var_dvar) + (assign2170_e2053 * locals.var_dvar_dn4)) - ((locals.var_cvar_dn4 * locals.var_cvar) + (locals.var_cvar * locals.var_cvar_dn4))) - (locals.var_dvar_dn4 * locals.var_asq)), (((((4.0 * locals.var_bvar_dn5) * locals.var_dvar) + (assign2170_e2053 * locals.var_dvar_dn5)) - ((locals.var_cvar_dn5 * locals.var_cvar) + (locals.var_cvar * locals.var_cvar_dn5))) - (locals.var_dvar_dn5 * locals.var_asq)),)
    } else {
        (locals.var_rvar, locals.var_rvar_dn1, locals.var_rvar_dn3, locals.var_rvar_dn4, locals.var_rvar_dn5,)
    }
};
        locals.var_rvar = assign2170_e2065;
        locals.var_rvar_dn1 = assign2170_e2065_d_n1;
        locals.var_rvar_dn3 = assign2170_e2065_d_n3;
        locals.var_rvar_dn4 = assign2170_e2065_d_n4;
        locals.var_rvar_dn5 = assign2170_e2065_d_n5;

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2180_e2075, assign2180_e2075_d_n1, assign2180_e2075_d_n3, assign2180_e2075_d_n4, assign2180_e2075_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2180_e2070: f64 = (locals.var_pvar * locals.var_pvar);
        let assign2180_e2072: f64 = (assign2180_e2070 * 0.3333333333333333);
        let assign2180_e2073: f64 = (locals.var_qvar - assign2180_e2072);
        (assign2180_e2073, (locals.var_qvar_dn1 - (((locals.var_pvar_dn1 * locals.var_pvar) + (locals.var_pvar * locals.var_pvar_dn1)) * 0.3333333333333333)), (locals.var_qvar_dn3 - (((locals.var_pvar_dn3 * locals.var_pvar) + (locals.var_pvar * locals.var_pvar_dn3)) * 0.3333333333333333)), (locals.var_qvar_dn4 - (((locals.var_pvar_dn4 * locals.var_pvar) + (locals.var_pvar * locals.var_pvar_dn4)) * 0.3333333333333333)), (locals.var_qvar_dn5 - (((locals.var_pvar_dn5 * locals.var_pvar) + (locals.var_pvar * locals.var_pvar_dn5)) * 0.3333333333333333)),)
    } else {
        (locals.var_aa, locals.var_aa_dn1, locals.var_aa_dn3, locals.var_aa_dn4, locals.var_aa_dn5,)
    }
};
        locals.var_aa = assign2180_e2075;
        locals.var_aa_dn1 = assign2180_e2075_d_n1;
        locals.var_aa_dn3 = assign2180_e2075_d_n3;
        locals.var_aa_dn4 = assign2180_e2075_d_n4;
        locals.var_aa_dn5 = assign2180_e2075_d_n5;

        let (assign2190_e2089, assign2190_e2089_d_n1, assign2190_e2089_d_n3, assign2190_e2089_d_n4, assign2190_e2089_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2190_e2082: f64 = (2.0 * locals.var_aa);
        let assign2190_e2083: f64 = (locals.var_qvar + assign2190_e2082);
        let assign2190_e2084: f64 = (locals.var_pvar * assign2190_e2083);
        let assign2190_e2086: f64 = (assign2190_e2084 / 9.0);
        let assign2190_e2087: f64 = (locals.var_rvar - assign2190_e2086);
        (assign2190_e2087, (locals.var_rvar_dn1 - (((locals.var_pvar_dn1 * assign2190_e2083) + (locals.var_pvar * (locals.var_qvar_dn1 + (2.0 * locals.var_aa_dn1)))) / 9.0)), (locals.var_rvar_dn3 - (((locals.var_pvar_dn3 * assign2190_e2083) + (locals.var_pvar * (locals.var_qvar_dn3 + (2.0 * locals.var_aa_dn3)))) / 9.0)), (locals.var_rvar_dn4 - (((locals.var_pvar_dn4 * assign2190_e2083) + (locals.var_pvar * (locals.var_qvar_dn4 + (2.0 * locals.var_aa_dn4)))) / 9.0)), (locals.var_rvar_dn5 - (((locals.var_pvar_dn5 * assign2190_e2083) + (locals.var_pvar * (locals.var_qvar_dn5 + (2.0 * locals.var_aa_dn5)))) / 9.0)),)
    } else {
        (locals.var_bb, locals.var_bb_dn1, locals.var_bb_dn3, locals.var_bb_dn4, locals.var_bb_dn5,)
    }
};
        locals.var_bb = assign2190_e2089;
        locals.var_bb_dn1 = assign2190_e2089_d_n1;
        locals.var_bb_dn3 = assign2190_e2089_d_n3;
        locals.var_bb_dn4 = assign2190_e2089_d_n4;
        locals.var_bb_dn5 = assign2190_e2089_d_n5;

        let (assign2200_e2099, assign2200_e2099_d_n1, assign2200_e2099_d_n3, assign2200_e2099_d_n4, assign2200_e2099_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2200_e2093: f64 = (locals.var_aa * locals.var_aa);
        let assign2200_e2095: f64 = (assign2200_e2093 * locals.var_aa);
        let assign2200_e2097: f64 = (assign2200_e2095 / 27.0);
        (assign2200_e2097, (((((locals.var_aa_dn1 * locals.var_aa) + (locals.var_aa * locals.var_aa_dn1)) * locals.var_aa) + (assign2200_e2093 * locals.var_aa_dn1)) / 27.0), (((((locals.var_aa_dn3 * locals.var_aa) + (locals.var_aa * locals.var_aa_dn3)) * locals.var_aa) + (assign2200_e2093 * locals.var_aa_dn3)) / 27.0), (((((locals.var_aa_dn4 * locals.var_aa) + (locals.var_aa * locals.var_aa_dn4)) * locals.var_aa) + (assign2200_e2093 * locals.var_aa_dn4)) / 27.0), (((((locals.var_aa_dn5 * locals.var_aa) + (locals.var_aa * locals.var_aa_dn5)) * locals.var_aa) + (assign2200_e2093 * locals.var_aa_dn5)) / 27.0),)
    } else {
        (locals.var_aa3d27, locals.var_aa3d27_dn1, locals.var_aa3d27_dn3, locals.var_aa3d27_dn4, locals.var_aa3d27_dn5,)
    }
};
        locals.var_aa3d27 = assign2200_e2099;
        locals.var_aa3d27_dn1 = assign2200_e2099_d_n1;
        locals.var_aa3d27_dn3 = assign2200_e2099_d_n3;
        locals.var_aa3d27_dn4 = assign2200_e2099_d_n4;
        locals.var_aa3d27_dn5 = assign2200_e2099_d_n5;

        let (assign2210_e2109, assign2210_e2109_d_n1, assign2210_e2109_d_n3, assign2210_e2109_d_n4, assign2210_e2109_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2210_e2103: f64 = (0.25 * locals.var_bb);
        let assign2210_e2105: f64 = (assign2210_e2103 * locals.var_bb);
        let assign2210_e2107: f64 = (assign2210_e2105 + locals.var_aa3d27);
        (assign2210_e2107, ((((0.25 * locals.var_bb_dn1) * locals.var_bb) + (assign2210_e2103 * locals.var_bb_dn1)) + locals.var_aa3d27_dn1), ((((0.25 * locals.var_bb_dn3) * locals.var_bb) + (assign2210_e2103 * locals.var_bb_dn3)) + locals.var_aa3d27_dn3), ((((0.25 * locals.var_bb_dn4) * locals.var_bb) + (assign2210_e2103 * locals.var_bb_dn4)) + locals.var_aa3d27_dn4), ((((0.25 * locals.var_bb_dn5) * locals.var_bb) + (assign2210_e2103 * locals.var_bb_dn5)) + locals.var_aa3d27_dn5),)
    } else {
        (locals.var_dd, locals.var_dd_dn1, locals.var_dd_dn3, locals.var_dd_dn4, locals.var_dd_dn5,)
    }
};
        locals.var_dd = assign2210_e2109;
        locals.var_dd_dn1 = assign2210_e2109_d_n1;
        locals.var_dd_dn3 = assign2210_e2109_d_n3;
        locals.var_dd_dn4 = assign2210_e2109_d_n4;
        locals.var_dd_dn5 = assign2210_e2109_d_n5;

        let (assign2220_e2114, assign2220_e2114_d_n1, assign2220_e2114_d_n3, assign2220_e2114_d_n4, assign2220_e2114_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2220_e2112: f64 = (locals.var_dd).sqrt();
        (assign2220_e2112, (locals.var_dd_dn1 / (2.0 * assign2220_e2112)), (locals.var_dd_dn3 / (2.0 * assign2220_e2112)), (locals.var_dd_dn4 / (2.0 * assign2220_e2112)), (locals.var_dd_dn5 / (2.0 * assign2220_e2112)),)
    } else {
        (locals.var_sd, locals.var_sd_dn1, locals.var_sd_dn3, locals.var_sd_dn4, locals.var_sd_dn5,)
    }
};
        locals.var_sd = assign2220_e2114;
        locals.var_sd_dn1 = assign2220_e2114_d_n1;
        locals.var_sd_dn3 = assign2220_e2114_d_n3;
        locals.var_sd_dn4 = assign2220_e2114_d_n4;
        locals.var_sd_dn5 = assign2220_e2114_d_n5;

        let assign2230_e2117: f64 = if locals.var_bb < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign2230_e2117;

        let (assign2240_e2128, assign2240_e2128_d_n1, assign2240_e2128_d_n3, assign2240_e2128_d_n4, assign2240_e2128_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard194 != 0.0)) {
        let assign2240_e2122: f64 = (-0.5);
        let assign2240_e2124: f64 = (assign2240_e2122 * locals.var_bb);
        let assign2240_e2126: f64 = (assign2240_e2124 + locals.var_sd);
        (assign2240_e2126, ((assign2240_e2122 * locals.var_bb_dn1) + locals.var_sd_dn1), ((assign2240_e2122 * locals.var_bb_dn3) + locals.var_sd_dn3), ((assign2240_e2122 * locals.var_bb_dn4) + locals.var_sd_dn4), ((assign2240_e2122 * locals.var_bb_dn5) + locals.var_sd_dn5),)
    } else {
        (locals.var_rp, locals.var_rp_dn1, locals.var_rp_dn3, locals.var_rp_dn4, locals.var_rp_dn5,)
    }
};
        locals.var_rp = assign2240_e2128;
        locals.var_rp_dn1 = assign2240_e2128_d_n1;
        locals.var_rp_dn3 = assign2240_e2128_d_n3;
        locals.var_rp_dn4 = assign2240_e2128_d_n4;
        locals.var_rp_dn5 = assign2240_e2128_d_n5;

        let (assign2250_e2137, assign2250_e2137_d_n1, assign2250_e2137_d_n3, assign2250_e2137_d_n4, assign2250_e2137_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard194 != 0.0)) {
        let assign2250_e2133: f64 = (-locals.var_aa3d27);
        let assign2250_e2135: f64 = (assign2250_e2133 / locals.var_rp);
        (assign2250_e2135, ((((-locals.var_aa3d27_dn1) * locals.var_rp) - (assign2250_e2133 * locals.var_rp_dn1)) / (locals.var_rp * locals.var_rp)), ((((-locals.var_aa3d27_dn3) * locals.var_rp) - (assign2250_e2133 * locals.var_rp_dn3)) / (locals.var_rp * locals.var_rp)), ((((-locals.var_aa3d27_dn4) * locals.var_rp) - (assign2250_e2133 * locals.var_rp_dn4)) / (locals.var_rp * locals.var_rp)), ((((-locals.var_aa3d27_dn5) * locals.var_rp) - (assign2250_e2133 * locals.var_rp_dn5)) / (locals.var_rp * locals.var_rp)),)
    } else {
        (locals.var_rm, locals.var_rm_dn1, locals.var_rm_dn3, locals.var_rm_dn4, locals.var_rm_dn5,)
    }
};
        locals.var_rm = assign2250_e2137;
        locals.var_rm_dn1 = assign2250_e2137_d_n1;
        locals.var_rm_dn3 = assign2250_e2137_d_n3;
        locals.var_rm_dn4 = assign2250_e2137_d_n4;
        locals.var_rm_dn5 = assign2250_e2137_d_n5;

        let (assign2260_e2149, assign2260_e2149_d_n1, assign2260_e2149_d_n3, assign2260_e2149_d_n4, assign2260_e2149_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard194 == 0.0)) {
        let assign2260_e2143: f64 = (-0.5);
        let assign2260_e2145: f64 = (assign2260_e2143 * locals.var_bb);
        let assign2260_e2147: f64 = (assign2260_e2145 - locals.var_sd);
        (assign2260_e2147, ((assign2260_e2143 * locals.var_bb_dn1) - locals.var_sd_dn1), ((assign2260_e2143 * locals.var_bb_dn3) - locals.var_sd_dn3), ((assign2260_e2143 * locals.var_bb_dn4) - locals.var_sd_dn4), ((assign2260_e2143 * locals.var_bb_dn5) - locals.var_sd_dn5),)
    } else {
        (locals.var_rm, locals.var_rm_dn1, locals.var_rm_dn3, locals.var_rm_dn4, locals.var_rm_dn5,)
    }
};
        locals.var_rm = assign2260_e2149;
        locals.var_rm_dn1 = assign2260_e2149_d_n1;
        locals.var_rm_dn3 = assign2260_e2149_d_n3;
        locals.var_rm_dn4 = assign2260_e2149_d_n4;
        locals.var_rm_dn5 = assign2260_e2149_d_n5;

        let (assign2270_e2159, assign2270_e2159_d_n1, assign2270_e2159_d_n3, assign2270_e2159_d_n4, assign2270_e2159_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard194 == 0.0)) {
        let assign2270_e2155: f64 = (-locals.var_aa3d27);
        let assign2270_e2157: f64 = (assign2270_e2155 / locals.var_rm);
        (assign2270_e2157, ((((-locals.var_aa3d27_dn1) * locals.var_rm) - (assign2270_e2155 * locals.var_rm_dn1)) / (locals.var_rm * locals.var_rm)), ((((-locals.var_aa3d27_dn3) * locals.var_rm) - (assign2270_e2155 * locals.var_rm_dn3)) / (locals.var_rm * locals.var_rm)), ((((-locals.var_aa3d27_dn4) * locals.var_rm) - (assign2270_e2155 * locals.var_rm_dn4)) / (locals.var_rm * locals.var_rm)), ((((-locals.var_aa3d27_dn5) * locals.var_rm) - (assign2270_e2155 * locals.var_rm_dn5)) / (locals.var_rm * locals.var_rm)),)
    } else {
        (locals.var_rp, locals.var_rp_dn1, locals.var_rp_dn3, locals.var_rp_dn4, locals.var_rp_dn5,)
    }
};
        locals.var_rp = assign2270_e2159;
        locals.var_rp_dn1 = assign2270_e2159_d_n1;
        locals.var_rp_dn3 = assign2270_e2159_d_n3;
        locals.var_rp_dn4 = assign2270_e2159_d_n4;
        locals.var_rp_dn5 = assign2270_e2159_d_n5;

        let assign2280_e2162: f64 = if locals.var_rp > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard195 = assign2280_e2162;

        let (assign2290_e2170, assign2290_e2170_d_n1, assign2290_e2170_d_n3, assign2290_e2170_d_n4, assign2290_e2170_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard195 != 0.0)) {
        let assign2290_e2168: f64 = (locals.var_rp).powf(0.3333333333333333);
        (assign2290_e2168, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_rp).powf(0.3333333333333333 - 1.0) * locals.var_rp_dn1)) } } else { (assign2290_e2168 * (0.3333333333333333 * (locals.var_rp_dn1 / locals.var_rp))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_rp).powf(0.3333333333333333 - 1.0) * locals.var_rp_dn3)) } } else { (assign2290_e2168 * (0.3333333333333333 * (locals.var_rp_dn3 / locals.var_rp))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_rp).powf(0.3333333333333333 - 1.0) * locals.var_rp_dn4)) } } else { (assign2290_e2168 * (0.3333333333333333 * (locals.var_rp_dn4 / locals.var_rp))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_rp).powf(0.3333333333333333 - 1.0) * locals.var_rp_dn5)) } } else { (assign2290_e2168 * (0.3333333333333333 * (locals.var_rp_dn5 / locals.var_rp))) },)
    } else {
        (locals.var_avar2, locals.var_avar2_dn1, locals.var_avar2_dn3, locals.var_avar2_dn4, locals.var_avar2_dn5,)
    }
};
        locals.var_avar2 = assign2290_e2170;
        locals.var_avar2_dn1 = assign2290_e2170_d_n1;
        locals.var_avar2_dn3 = assign2290_e2170_d_n3;
        locals.var_avar2_dn4 = assign2290_e2170_d_n4;
        locals.var_avar2_dn5 = assign2290_e2170_d_n5;

        let assign2300_e2173: f64 = (-1e-6);
        let assign2300_e2174: f64 = if locals.var_rp < assign2300_e2173 { 1.0 } else { 0.0 };
        locals.var_guard196 = assign2300_e2174;

        let (assign2310_e2187, assign2310_e2187_d_n1, assign2310_e2187_d_n3, assign2310_e2187_d_n4, assign2310_e2187_d_n5,) = {
    if (((locals.var_guard193 != 0.0) && (locals.var_guard195 == 0.0)) && (locals.var_guard196 != 0.0)) {
        let assign2310_e2182: f64 = (-locals.var_rp);
        let assign2310_e2184: f64 = (assign2310_e2182).powf(0.3333333333333333);
        let assign2310_e2185: f64 = (-assign2310_e2184);
        (assign2310_e2185, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign2310_e2182).powf(0.3333333333333333 - 1.0) * (-locals.var_rp_dn1))) } } else { (assign2310_e2184 * (0.3333333333333333 * ((-locals.var_rp_dn1) / assign2310_e2182))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign2310_e2182).powf(0.3333333333333333 - 1.0) * (-locals.var_rp_dn3))) } } else { (assign2310_e2184 * (0.3333333333333333 * ((-locals.var_rp_dn3) / assign2310_e2182))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign2310_e2182).powf(0.3333333333333333 - 1.0) * (-locals.var_rp_dn4))) } } else { (assign2310_e2184 * (0.3333333333333333 * ((-locals.var_rp_dn4) / assign2310_e2182))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign2310_e2182).powf(0.3333333333333333 - 1.0) * (-locals.var_rp_dn5))) } } else { (assign2310_e2184 * (0.3333333333333333 * ((-locals.var_rp_dn5) / assign2310_e2182))) }),)
    } else {
        (locals.var_avar2, locals.var_avar2_dn1, locals.var_avar2_dn3, locals.var_avar2_dn4, locals.var_avar2_dn5,)
    }
};
        locals.var_avar2 = assign2310_e2187;
        locals.var_avar2_dn1 = assign2310_e2187_d_n1;
        locals.var_avar2_dn3 = assign2310_e2187_d_n3;
        locals.var_avar2_dn4 = assign2310_e2187_d_n4;
        locals.var_avar2_dn5 = assign2310_e2187_d_n5;

        let (assign2320_e2199, assign2320_e2199_d_n1, assign2320_e2199_d_n3, assign2320_e2199_d_n4, assign2320_e2199_d_n5,) = {
    if (((locals.var_guard193 != 0.0) && (locals.var_guard195 == 0.0)) && (locals.var_guard196 == 0.0)) {
        let assign2320_e2197: f64 = (10000.0 * locals.var_rp);
        (assign2320_e2197, (10000.0 * locals.var_rp_dn1), (10000.0 * locals.var_rp_dn3), (10000.0 * locals.var_rp_dn4), (10000.0 * locals.var_rp_dn5),)
    } else {
        (locals.var_avar2, locals.var_avar2_dn1, locals.var_avar2_dn3, locals.var_avar2_dn4, locals.var_avar2_dn5,)
    }
};
        locals.var_avar2 = assign2320_e2199;
        locals.var_avar2_dn1 = assign2320_e2199_d_n1;
        locals.var_avar2_dn3 = assign2320_e2199_d_n3;
        locals.var_avar2_dn4 = assign2320_e2199_d_n4;
        locals.var_avar2_dn5 = assign2320_e2199_d_n5;

        let assign2330_e2202: f64 = if locals.var_rm > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard197 = assign2330_e2202;

        let (assign2340_e2210, assign2340_e2210_d_n1, assign2340_e2210_d_n3, assign2340_e2210_d_n4, assign2340_e2210_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard197 != 0.0)) {
        let assign2340_e2208: f64 = (locals.var_rm).powf(0.3333333333333333);
        (assign2340_e2208, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_rm).powf(0.3333333333333333 - 1.0) * locals.var_rm_dn1)) } } else { (assign2340_e2208 * (0.3333333333333333 * (locals.var_rm_dn1 / locals.var_rm))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_rm).powf(0.3333333333333333 - 1.0) * locals.var_rm_dn3)) } } else { (assign2340_e2208 * (0.3333333333333333 * (locals.var_rm_dn3 / locals.var_rm))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_rm).powf(0.3333333333333333 - 1.0) * locals.var_rm_dn4)) } } else { (assign2340_e2208 * (0.3333333333333333 * (locals.var_rm_dn4 / locals.var_rm))) }, if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((locals.var_rm).powf(0.3333333333333333 - 1.0) * locals.var_rm_dn5)) } } else { (assign2340_e2208 * (0.3333333333333333 * (locals.var_rm_dn5 / locals.var_rm))) },)
    } else {
        (locals.var_bvar2, locals.var_bvar2_dn1, locals.var_bvar2_dn3, locals.var_bvar2_dn4, locals.var_bvar2_dn5,)
    }
};
        locals.var_bvar2 = assign2340_e2210;
        locals.var_bvar2_dn1 = assign2340_e2210_d_n1;
        locals.var_bvar2_dn3 = assign2340_e2210_d_n3;
        locals.var_bvar2_dn4 = assign2340_e2210_d_n4;
        locals.var_bvar2_dn5 = assign2340_e2210_d_n5;

        let assign2350_e2213: f64 = (-1e-6);
        let assign2350_e2214: f64 = if locals.var_rm < assign2350_e2213 { 1.0 } else { 0.0 };
        locals.var_guard198 = assign2350_e2214;

        let (assign2360_e2227, assign2360_e2227_d_n1, assign2360_e2227_d_n3, assign2360_e2227_d_n4, assign2360_e2227_d_n5,) = {
    if (((locals.var_guard193 != 0.0) && (locals.var_guard197 == 0.0)) && (locals.var_guard198 != 0.0)) {
        let assign2360_e2222: f64 = (-locals.var_rm);
        let assign2360_e2224: f64 = (assign2360_e2222).powf(0.3333333333333333);
        let assign2360_e2225: f64 = (-assign2360_e2224);
        (assign2360_e2225, (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign2360_e2222).powf(0.3333333333333333 - 1.0) * (-locals.var_rm_dn1))) } } else { (assign2360_e2224 * (0.3333333333333333 * ((-locals.var_rm_dn1) / assign2360_e2222))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign2360_e2222).powf(0.3333333333333333 - 1.0) * (-locals.var_rm_dn3))) } } else { (assign2360_e2224 * (0.3333333333333333 * ((-locals.var_rm_dn3) / assign2360_e2222))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign2360_e2222).powf(0.3333333333333333 - 1.0) * (-locals.var_rm_dn4))) } } else { (assign2360_e2224 * (0.3333333333333333 * ((-locals.var_rm_dn4) / assign2360_e2222))) }), (-if 0.0 == 0.0 && ((0.3333333333333333) as f64).is_finite() && ((0.3333333333333333) as f64).fract() == 0.0 { if 0.3333333333333333 == 0.0 { 0.0 } else { (0.3333333333333333 * ((assign2360_e2222).powf(0.3333333333333333 - 1.0) * (-locals.var_rm_dn5))) } } else { (assign2360_e2224 * (0.3333333333333333 * ((-locals.var_rm_dn5) / assign2360_e2222))) }),)
    } else {
        (locals.var_bvar2, locals.var_bvar2_dn1, locals.var_bvar2_dn3, locals.var_bvar2_dn4, locals.var_bvar2_dn5,)
    }
};
        locals.var_bvar2 = assign2360_e2227;
        locals.var_bvar2_dn1 = assign2360_e2227_d_n1;
        locals.var_bvar2_dn3 = assign2360_e2227_d_n3;
        locals.var_bvar2_dn4 = assign2360_e2227_d_n4;
        locals.var_bvar2_dn5 = assign2360_e2227_d_n5;

        let (assign2370_e2239, assign2370_e2239_d_n1, assign2370_e2239_d_n3, assign2370_e2239_d_n4, assign2370_e2239_d_n5,) = {
    if (((locals.var_guard193 != 0.0) && (locals.var_guard197 == 0.0)) && (locals.var_guard198 == 0.0)) {
        let assign2370_e2237: f64 = (10000.0 * locals.var_rm);
        (assign2370_e2237, (10000.0 * locals.var_rm_dn1), (10000.0 * locals.var_rm_dn3), (10000.0 * locals.var_rm_dn4), (10000.0 * locals.var_rm_dn5),)
    } else {
        (locals.var_bvar2, locals.var_bvar2_dn1, locals.var_bvar2_dn3, locals.var_bvar2_dn4, locals.var_bvar2_dn5,)
    }
};
        locals.var_bvar2 = assign2370_e2239;
        locals.var_bvar2_dn1 = assign2370_e2239_d_n1;
        locals.var_bvar2_dn3 = assign2370_e2239_d_n3;
        locals.var_bvar2_dn4 = assign2370_e2239_d_n4;
        locals.var_bvar2_dn5 = assign2370_e2239_d_n5;

        let (assign2380_e2249, assign2380_e2249_d_n1, assign2380_e2249_d_n3, assign2380_e2249_d_n4, assign2380_e2249_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2380_e2243: f64 = (locals.var_avar2 + locals.var_bvar2);
        let assign2380_e2246: f64 = (locals.var_pvar * 0.3333333333333333);
        let assign2380_e2247: f64 = (assign2380_e2243 - assign2380_e2246);
        (assign2380_e2247, ((locals.var_avar2_dn1 + locals.var_bvar2_dn1) - (locals.var_pvar_dn1 * 0.3333333333333333)), ((locals.var_avar2_dn3 + locals.var_bvar2_dn3) - (locals.var_pvar_dn3 * 0.3333333333333333)), ((locals.var_avar2_dn4 + locals.var_bvar2_dn4) - (locals.var_pvar_dn4 * 0.3333333333333333)), ((locals.var_avar2_dn5 + locals.var_bvar2_dn5) - (locals.var_pvar_dn5 * 0.3333333333333333)),)
    } else {
        (locals.var_yvar, locals.var_yvar_dn1, locals.var_yvar_dn3, locals.var_yvar_dn4, locals.var_yvar_dn5,)
    }
};
        locals.var_yvar = assign2380_e2249;
        locals.var_yvar_dn1 = assign2380_e2249_d_n1;
        locals.var_yvar_dn3 = assign2380_e2249_d_n3;
        locals.var_yvar_dn4 = assign2380_e2249_d_n4;
        locals.var_yvar_dn5 = assign2380_e2249_d_n5;

        let (assign2390_e2260, assign2390_e2260_d_n1, assign2390_e2260_d_n3, assign2390_e2260_d_n4, assign2390_e2260_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2390_e2253: f64 = (0.25 * locals.var_asq);
        let assign2390_e2255: f64 = (assign2390_e2253 - locals.var_bvar);
        let assign2390_e2257: f64 = (assign2390_e2255 + locals.var_yvar);
        let assign2390_e2258: f64 = (assign2390_e2257).sqrt();
        (assign2390_e2258, (((-locals.var_bvar_dn1) + locals.var_yvar_dn1) / (2.0 * assign2390_e2258)), ((((0.25 * locals.var_asq_dn3) - locals.var_bvar_dn3) + locals.var_yvar_dn3) / (2.0 * assign2390_e2258)), (((-locals.var_bvar_dn4) + locals.var_yvar_dn4) / (2.0 * assign2390_e2258)), (((-locals.var_bvar_dn5) + locals.var_yvar_dn5) / (2.0 * assign2390_e2258)),)
    } else {
        (locals.var_rvar, locals.var_rvar_dn1, locals.var_rvar_dn3, locals.var_rvar_dn4, locals.var_rvar_dn5,)
    }
};
        locals.var_rvar = assign2390_e2260;
        locals.var_rvar_dn1 = assign2390_e2260_d_n1;
        locals.var_rvar_dn3 = assign2390_e2260_d_n3;
        locals.var_rvar_dn4 = assign2390_e2260_d_n4;
        locals.var_rvar_dn5 = assign2390_e2260_d_n5;

        let (assign2400_e2274, assign2400_e2274_d_n1, assign2400_e2274_d_n3, assign2400_e2274_d_n4, assign2400_e2274_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2400_e2264: f64 = (0.75 * locals.var_asq);
        let assign2400_e2267: f64 = (locals.var_rvar * locals.var_rvar);
        let assign2400_e2268: f64 = (assign2400_e2264 - assign2400_e2267);
        let assign2400_e2271: f64 = (2.0 * locals.var_bvar);
        let assign2400_e2272: f64 = (assign2400_e2268 - assign2400_e2271);
        (assign2400_e2272, ((-((locals.var_rvar_dn1 * locals.var_rvar) + (locals.var_rvar * locals.var_rvar_dn1))) - (2.0 * locals.var_bvar_dn1)), (((0.75 * locals.var_asq_dn3) - ((locals.var_rvar_dn3 * locals.var_rvar) + (locals.var_rvar * locals.var_rvar_dn3))) - (2.0 * locals.var_bvar_dn3)), ((-((locals.var_rvar_dn4 * locals.var_rvar) + (locals.var_rvar * locals.var_rvar_dn4))) - (2.0 * locals.var_bvar_dn4)), ((-((locals.var_rvar_dn5 * locals.var_rvar) + (locals.var_rvar * locals.var_rvar_dn5))) - (2.0 * locals.var_bvar_dn5)),)
    } else {
        (locals.var_val1, locals.var_val1_dn1, locals.var_val1_dn3, locals.var_val1_dn4, locals.var_val1_dn5,)
    }
};
        locals.var_val1 = assign2400_e2274;
        locals.var_val1_dn1 = assign2400_e2274_d_n1;
        locals.var_val1_dn3 = assign2400_e2274_d_n3;
        locals.var_val1_dn4 = assign2400_e2274_d_n4;
        locals.var_val1_dn5 = assign2400_e2274_d_n5;

        let (assign2410_e2292, assign2410_e2292_d_n1, assign2410_e2292_d_n3, assign2410_e2292_d_n4, assign2410_e2292_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2410_e2278: f64 = (locals.var_avar * locals.var_bvar);
        let assign2410_e2281: f64 = (2.0 * locals.var_cvar);
        let assign2410_e2282: f64 = (assign2410_e2278 - assign2410_e2281);
        let assign2410_e2285: f64 = (0.25 * locals.var_asq);
        let assign2410_e2287: f64 = (assign2410_e2285 * locals.var_avar);
        let assign2410_e2288: f64 = (assign2410_e2282 - assign2410_e2287);
        let assign2410_e2290: f64 = (assign2410_e2288 / locals.var_rvar);
        (assign2410_e2290, (((((locals.var_avar * locals.var_bvar_dn1) - (2.0 * locals.var_cvar_dn1)) * locals.var_rvar) - (assign2410_e2288 * locals.var_rvar_dn1)) / (locals.var_rvar * locals.var_rvar)), (((((((locals.var_avar_dn3 * locals.var_bvar) + (locals.var_avar * locals.var_bvar_dn3)) - (2.0 * locals.var_cvar_dn3)) - (((0.25 * locals.var_asq_dn3) * locals.var_avar) + (assign2410_e2285 * locals.var_avar_dn3))) * locals.var_rvar) - (assign2410_e2288 * locals.var_rvar_dn3)) / (locals.var_rvar * locals.var_rvar)), (((((locals.var_avar * locals.var_bvar_dn4) - (2.0 * locals.var_cvar_dn4)) * locals.var_rvar) - (assign2410_e2288 * locals.var_rvar_dn4)) / (locals.var_rvar * locals.var_rvar)), (((((locals.var_avar * locals.var_bvar_dn5) - (2.0 * locals.var_cvar_dn5)) * locals.var_rvar) - (assign2410_e2288 * locals.var_rvar_dn5)) / (locals.var_rvar * locals.var_rvar)),)
    } else {
        (locals.var_val2, locals.var_val2_dn1, locals.var_val2_dn3, locals.var_val2_dn4, locals.var_val2_dn5,)
    }
};
        locals.var_val2 = assign2410_e2292;
        locals.var_val2_dn1 = assign2410_e2292_d_n1;
        locals.var_val2_dn3 = assign2410_e2292_d_n3;
        locals.var_val2_dn4 = assign2410_e2292_d_n4;
        locals.var_val2_dn5 = assign2410_e2292_d_n5;

        let (assign2420_e2298, assign2420_e2298_d_n1, assign2420_e2298_d_n3, assign2420_e2298_d_n4, assign2420_e2298_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2420_e2296: f64 = (locals.var_val1 + locals.var_val2);
        (assign2420_e2296, (locals.var_val1_dn1 + locals.var_val2_dn1), (locals.var_val1_dn3 + locals.var_val2_dn3), (locals.var_val1_dn4 + locals.var_val2_dn4), (locals.var_val1_dn5 + locals.var_val2_dn5),)
    } else {
        (locals.var_arg1, locals.var_arg1_dn1, locals.var_arg1_dn3, locals.var_arg1_dn4, locals.var_arg1_dn5,)
    }
};
        locals.var_arg1 = assign2420_e2298;
        locals.var_arg1_dn1 = assign2420_e2298_d_n1;
        locals.var_arg1_dn3 = assign2420_e2298_d_n3;
        locals.var_arg1_dn4 = assign2420_e2298_d_n4;
        locals.var_arg1_dn5 = assign2420_e2298_d_n5;

        let assign2430_e2301: f64 = if locals.var_arg1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard199 = assign2430_e2301;

        let (assign2440_e2308, assign2440_e2308_d_n1, assign2440_e2308_d_n3, assign2440_e2308_d_n4, assign2440_e2308_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign2440_e2306: f64 = (locals.var_arg1).sqrt();
        (assign2440_e2306, (locals.var_arg1_dn1 / (2.0 * assign2440_e2306)), (locals.var_arg1_dn3 / (2.0 * assign2440_e2306)), (locals.var_arg1_dn4 / (2.0 * assign2440_e2306)), (locals.var_arg1_dn5 / (2.0 * assign2440_e2306)),)
    } else {
        (locals.var_dore, locals.var_dore_dn1, locals.var_dore_dn3, locals.var_dore_dn4, locals.var_dore_dn5,)
    }
};
        locals.var_dore = assign2440_e2308;
        locals.var_dore_dn1 = assign2440_e2308_d_n1;
        locals.var_dore_dn3 = assign2440_e2308_d_n3;
        locals.var_dore_dn4 = assign2440_e2308_d_n4;
        locals.var_dore_dn5 = assign2440_e2308_d_n5;

        let (assign2450_e2323, assign2450_e2323_d_n1, assign2450_e2323_d_n3, assign2450_e2323_d_n4, assign2450_e2323_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard199 != 0.0)) {
        let assign2450_e2313: f64 = (-0.25);
        let assign2450_e2315: f64 = (assign2450_e2313 * locals.var_avar);
        let assign2450_e2319: f64 = (locals.var_dore + locals.var_rvar);
        let assign2450_e2320: f64 = (0.5 * assign2450_e2319);
        let assign2450_e2321: f64 = (assign2450_e2315 + assign2450_e2320);
        (assign2450_e2321, (0.5 * (locals.var_dore_dn1 + locals.var_rvar_dn1)), ((assign2450_e2313 * locals.var_avar_dn3) + (0.5 * (locals.var_dore_dn3 + locals.var_rvar_dn3))), (0.5 * (locals.var_dore_dn4 + locals.var_rvar_dn4)), (0.5 * (locals.var_dore_dn5 + locals.var_rvar_dn5)),)
    } else {
        (locals.var_vsat, locals.var_vsat_dn1, locals.var_vsat_dn3, locals.var_vsat_dn4, locals.var_vsat_dn5,)
    }
};
        locals.var_vsat = assign2450_e2323;
        locals.var_vsat_dn1 = assign2450_e2323_d_n1;
        locals.var_vsat_dn3 = assign2450_e2323_d_n3;
        locals.var_vsat_dn4 = assign2450_e2323_d_n4;
        locals.var_vsat_dn5 = assign2450_e2323_d_n5;

        let (assign2460_e2332, assign2460_e2332_d_n1, assign2460_e2332_d_n3, assign2460_e2332_d_n4, assign2460_e2332_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard199 == 0.0)) {
        let assign2460_e2330: f64 = (locals.var_val1 - locals.var_val2);
        (assign2460_e2330, (locals.var_val1_dn1 - locals.var_val2_dn1), (locals.var_val1_dn3 - locals.var_val2_dn3), (locals.var_val1_dn4 - locals.var_val2_dn4), (locals.var_val1_dn5 - locals.var_val2_dn5),)
    } else {
        (locals.var_arg2, locals.var_arg2_dn1, locals.var_arg2_dn3, locals.var_arg2_dn4, locals.var_arg2_dn5,)
    }
};
        locals.var_arg2 = assign2460_e2332;
        locals.var_arg2_dn1 = assign2460_e2332_d_n1;
        locals.var_arg2_dn3 = assign2460_e2332_d_n3;
        locals.var_arg2_dn4 = assign2460_e2332_d_n4;
        locals.var_arg2_dn5 = assign2460_e2332_d_n5;

        let (assign2470_e2345, assign2470_e2345_d_n1, assign2470_e2345_d_n3, assign2470_e2345_d_n4, assign2470_e2345_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard199 == 0.0)) {
        let assign2470_e2339: f64 = (locals.var_arg2 * locals.var_arg2);
        let assign2470_e2341: f64 = (assign2470_e2339 + 0.0001);
        let assign2470_e2342: f64 = (assign2470_e2341).sqrt();
        let assign2470_e2343: f64 = (assign2470_e2342).sqrt();
        (assign2470_e2343, ((((locals.var_arg2_dn1 * locals.var_arg2) + (locals.var_arg2 * locals.var_arg2_dn1)) / (2.0 * assign2470_e2342)) / (2.0 * assign2470_e2343)), ((((locals.var_arg2_dn3 * locals.var_arg2) + (locals.var_arg2 * locals.var_arg2_dn3)) / (2.0 * assign2470_e2342)) / (2.0 * assign2470_e2343)), ((((locals.var_arg2_dn4 * locals.var_arg2) + (locals.var_arg2 * locals.var_arg2_dn4)) / (2.0 * assign2470_e2342)) / (2.0 * assign2470_e2343)), ((((locals.var_arg2_dn5 * locals.var_arg2) + (locals.var_arg2 * locals.var_arg2_dn5)) / (2.0 * assign2470_e2342)) / (2.0 * assign2470_e2343)),)
    } else {
        (locals.var_dore, locals.var_dore_dn1, locals.var_dore_dn3, locals.var_dore_dn4, locals.var_dore_dn5,)
    }
};
        locals.var_dore = assign2470_e2345;
        locals.var_dore_dn1 = assign2470_e2345_d_n1;
        locals.var_dore_dn3 = assign2470_e2345_d_n3;
        locals.var_dore_dn4 = assign2470_e2345_d_n4;
        locals.var_dore_dn5 = assign2470_e2345_d_n5;

        let (assign2480_e2361, assign2480_e2361_d_n1, assign2480_e2361_d_n3, assign2480_e2361_d_n4, assign2480_e2361_d_n5,) = {
    if ((locals.var_guard193 != 0.0) && (locals.var_guard199 == 0.0)) {
        let assign2480_e2351: f64 = (-0.25);
        let assign2480_e2353: f64 = (assign2480_e2351 * locals.var_avar);
        let assign2480_e2357: f64 = (locals.var_dore - locals.var_rvar);
        let assign2480_e2358: f64 = (0.5 * assign2480_e2357);
        let assign2480_e2359: f64 = (assign2480_e2353 + assign2480_e2358);
        (assign2480_e2359, (0.5 * (locals.var_dore_dn1 - locals.var_rvar_dn1)), ((assign2480_e2351 * locals.var_avar_dn3) + (0.5 * (locals.var_dore_dn3 - locals.var_rvar_dn3))), (0.5 * (locals.var_dore_dn4 - locals.var_rvar_dn4)), (0.5 * (locals.var_dore_dn5 - locals.var_rvar_dn5)),)
    } else {
        (locals.var_vsat, locals.var_vsat_dn1, locals.var_vsat_dn3, locals.var_vsat_dn4, locals.var_vsat_dn5,)
    }
};
        locals.var_vsat = assign2480_e2361;
        locals.var_vsat_dn1 = assign2480_e2361_d_n1;
        locals.var_vsat_dn3 = assign2480_e2361_d_n3;
        locals.var_vsat_dn4 = assign2480_e2361_d_n4;
        locals.var_vsat_dn5 = assign2480_e2361_d_n5;

        let assign2490_e2364: f64 = if locals.var_v1c > locals.var_v1cx { 1.0 } else { 0.0 };
        locals.var_guard200 = assign2490_e2364;

        let (assign2500_e2375, assign2500_e2375_d_n1, assign2500_e2375_d_n3, assign2500_e2375_d_n4, assign2500_e2375_d_n5,) = {
    if ((locals.var_guard193 == 0.0) && (locals.var_guard200 != 0.0)) {
        let assign2500_e2372: f64 = (locals.var_vpo - locals.var_v1c);
        let assign2500_e2373: f64 = (locals.var_dfsq * assign2500_e2372);
        (assign2500_e2373, (locals.var_dfsq * (-locals.var_v1c_dn1)), ((locals.var_dfsq_dn3 * assign2500_e2372) + (locals.var_dfsq * (locals.var_vpo_dn3 - locals.var_v1c_dn3))), (locals.var_dfsq * (-locals.var_v1c_dn4)), (locals.var_dfsq * (-locals.var_v1c_dn5)),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn1, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5,)
    }
};
        locals.var_tmp = assign2500_e2375;
        locals.var_tmp_dn1 = assign2500_e2375_d_n1;
        locals.var_tmp_dn3 = assign2500_e2375_d_n3;
        locals.var_tmp_dn4 = assign2500_e2375_d_n4;
        locals.var_tmp_dn5 = assign2500_e2375_d_n5;

        let (assign2510_e2405, assign2510_e2405_d_n1, assign2510_e2405_d_n3, assign2510_e2405_d_n4, assign2510_e2405_d_n5,) = {
    if ((locals.var_guard193 == 0.0) && (locals.var_guard200 != 0.0)) {
        let assign2510_e2384: f64 = (2.0 * locals.var_tmp);
        let assign2510_e2385: f64 = (1.0 - assign2510_e2384);
        let assign2510_e2386: f64 = (2.0 * assign2510_e2385);
        let assign2510_e2389: f64 = (locals.var_vpo - locals.var_v1c);
        let assign2510_e2390: f64 = (assign2510_e2386 * assign2510_e2389);
        let assign2510_e2394: f64 = (3.0 * locals.var_tmp);
        let assign2510_e2395: f64 = (1.0 - assign2510_e2394);
        let assign2510_e2399: f64 = (1.5 * locals.var_tmp);
        let assign2510_e2400: f64 = (1.0 - assign2510_e2399);
        let assign2510_e2401: f64 = (assign2510_e2400).sqrt();
        let assign2510_e2402: f64 = (assign2510_e2395 + assign2510_e2401);
        let assign2510_e2403: f64 = (assign2510_e2390 / assign2510_e2402);
        (assign2510_e2403, ((((((2.0 * (-(2.0 * locals.var_tmp_dn1))) * assign2510_e2389) + (assign2510_e2386 * (-locals.var_v1c_dn1))) * assign2510_e2402) - (assign2510_e2390 * ((-(3.0 * locals.var_tmp_dn1)) + ((-(1.5 * locals.var_tmp_dn1)) / (2.0 * assign2510_e2401))))) / (assign2510_e2402 * assign2510_e2402)), ((((((2.0 * (-(2.0 * locals.var_tmp_dn3))) * assign2510_e2389) + (assign2510_e2386 * (locals.var_vpo_dn3 - locals.var_v1c_dn3))) * assign2510_e2402) - (assign2510_e2390 * ((-(3.0 * locals.var_tmp_dn3)) + ((-(1.5 * locals.var_tmp_dn3)) / (2.0 * assign2510_e2401))))) / (assign2510_e2402 * assign2510_e2402)), ((((((2.0 * (-(2.0 * locals.var_tmp_dn4))) * assign2510_e2389) + (assign2510_e2386 * (-locals.var_v1c_dn4))) * assign2510_e2402) - (assign2510_e2390 * ((-(3.0 * locals.var_tmp_dn4)) + ((-(1.5 * locals.var_tmp_dn4)) / (2.0 * assign2510_e2401))))) / (assign2510_e2402 * assign2510_e2402)), ((((((2.0 * (-(2.0 * locals.var_tmp_dn5))) * assign2510_e2389) + (assign2510_e2386 * (-locals.var_v1c_dn5))) * assign2510_e2402) - (assign2510_e2390 * ((-(3.0 * locals.var_tmp_dn5)) + ((-(1.5 * locals.var_tmp_dn5)) / (2.0 * assign2510_e2401))))) / (assign2510_e2402 * assign2510_e2402)),)
    } else {
        (locals.var_vsat, locals.var_vsat_dn1, locals.var_vsat_dn3, locals.var_vsat_dn4, locals.var_vsat_dn5,)
    }
};
        locals.var_vsat = assign2510_e2405;
        locals.var_vsat_dn1 = assign2510_e2405_d_n1;
        locals.var_vsat_dn3 = assign2510_e2405_d_n3;
        locals.var_vsat_dn4 = assign2510_e2405_d_n4;
        locals.var_vsat_dn5 = assign2510_e2405_d_n5;

        let (assign2520_e2417, assign2520_e2417_d_n1, assign2520_e2417_d_n3, assign2520_e2417_d_n4, assign2520_e2417_d_n5,) = {
    if ((locals.var_guard193 == 0.0) && (locals.var_guard200 == 0.0)) {
        let assign2520_e2413: f64 = (3.0 * locals.var_dfsq);
        let assign2520_e2415: f64 = (assign2520_e2413 * locals.var_pe);
        (assign2520_e2415, (assign2520_e2413 * locals.var_pe_dn1), (((3.0 * locals.var_dfsq_dn3) * locals.var_pe) + (assign2520_e2413 * locals.var_pe_dn3)), (assign2520_e2413 * locals.var_pe_dn4), (assign2520_e2413 * locals.var_pe_dn5),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn1, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5,)
    }
};
        locals.var_tmp = assign2520_e2417;
        locals.var_tmp_dn1 = assign2520_e2417_d_n1;
        locals.var_tmp_dn3 = assign2520_e2417_d_n3;
        locals.var_tmp_dn4 = assign2520_e2417_d_n4;
        locals.var_tmp_dn5 = assign2520_e2417_d_n5;

        let (assign2530_e2436, assign2530_e2436_d_n1, assign2530_e2436_d_n3, assign2530_e2436_d_n4, assign2530_e2436_d_n5,) = {
    if ((locals.var_guard193 == 0.0) && (locals.var_guard200 == 0.0)) {
        let assign2530_e2425: f64 = (1.0 - locals.var_tmp);
        let assign2530_e2428: f64 = (1.0 + locals.var_tmp);
        let assign2530_e2429: f64 = (assign2530_e2428).sqrt();
        let assign2530_e2430: f64 = (assign2530_e2425 + assign2530_e2429);
        let assign2530_e2433: f64 = (4.5 * locals.var_dfsq);
        let assign2530_e2434: f64 = (assign2530_e2430 / assign2530_e2433);
        (assign2530_e2434, (((-locals.var_tmp_dn1) + (locals.var_tmp_dn1 / (2.0 * assign2530_e2429))) / assign2530_e2433), (((((-locals.var_tmp_dn3) + (locals.var_tmp_dn3 / (2.0 * assign2530_e2429))) * assign2530_e2433) - (assign2530_e2430 * (4.5 * locals.var_dfsq_dn3))) / (assign2530_e2433 * assign2530_e2433)), (((-locals.var_tmp_dn4) + (locals.var_tmp_dn4 / (2.0 * assign2530_e2429))) / assign2530_e2433), (((-locals.var_tmp_dn5) + (locals.var_tmp_dn5 / (2.0 * assign2530_e2429))) / assign2530_e2433),)
    } else {
        (locals.var_vsat, locals.var_vsat_dn1, locals.var_vsat_dn3, locals.var_vsat_dn4, locals.var_vsat_dn5,)
    }
};
        locals.var_vsat = assign2530_e2436;
        locals.var_vsat_dn1 = assign2530_e2436_d_n1;
        locals.var_vsat_dn3 = assign2530_e2436_d_n3;
        locals.var_vsat_dn4 = assign2530_e2436_d_n4;
        locals.var_vsat_dn5 = assign2530_e2436_d_n5;

        let assign2540_e2443: f64 = if ((p.p63 > 1.0) && (locals.var_df > 1e-9)) { 1.0 } else { 0.0 };
        locals.var_guard201 = assign2540_e2443;

        let (assign2550_e2449, assign2550_e2449_d_n1, assign2550_e2449_d_n3, assign2550_e2449_d_n4, assign2550_e2449_d_n5,) = {
    if (locals.var_guard201 != 0.0) {
        let assign2550_e2447: f64 = (locals.var_vsat + locals.var_phi_t0);
        (assign2550_e2447, locals.var_vsat_dn1, (locals.var_vsat_dn3 + locals.var_phi_t0_dn3), locals.var_vsat_dn4, locals.var_vsat_dn5,)
    } else {
        (locals.var_vsatphi, locals.var_vsatphi_dn1, locals.var_vsatphi_dn3, locals.var_vsatphi_dn4, locals.var_vsatphi_dn5,)
    }
};
        locals.var_vsatphi = assign2550_e2449;
        locals.var_vsatphi_dn1 = assign2550_e2449_d_n1;
        locals.var_vsatphi_dn3 = assign2550_e2449_d_n3;
        locals.var_vsatphi_dn4 = assign2550_e2449_d_n4;
        locals.var_vsatphi_dn5 = assign2550_e2449_d_n5;

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2560_e2458, assign2560_e2458_d_n1, assign2560_e2458_d_n3, assign2560_e2458_d_n4, assign2560_e2458_d_n5,) = {
    if (locals.var_guard201 != 0.0) {
        let assign2560_e2454: f64 = (locals.var_pe + locals.var_vsat);
        let assign2560_e2455: f64 = (assign2560_e2454).sqrt();
        let assign2560_e2456: f64 = (locals.var_df * assign2560_e2455);
        (assign2560_e2456, (locals.var_df * ((locals.var_pe_dn1 + locals.var_vsat_dn1) / (2.0 * assign2560_e2455))), (locals.var_df * ((locals.var_pe_dn3 + locals.var_vsat_dn3) / (2.0 * assign2560_e2455))), (locals.var_df * ((locals.var_pe_dn4 + locals.var_vsat_dn4) / (2.0 * assign2560_e2455))), (locals.var_df * ((locals.var_pe_dn5 + locals.var_vsat_dn5) / (2.0 * assign2560_e2455))),)
    } else {
        (locals.var_fsatphi, locals.var_fsatphi_dn1, locals.var_fsatphi_dn3, locals.var_fsatphi_dn4, locals.var_fsatphi_dn5,)
    }
};
        locals.var_fsatphi = assign2560_e2458;
        locals.var_fsatphi_dn1 = assign2560_e2458_d_n1;
        locals.var_fsatphi_dn3 = assign2560_e2458_d_n3;
        locals.var_fsatphi_dn4 = assign2560_e2458_d_n4;
        locals.var_fsatphi_dn5 = assign2560_e2458_d_n5;

        let assign2570_e2461: f64 = if locals.var_iecrit > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard202 = assign2570_e2461;

        let (assign2580_e2475, assign2580_e2475_d_n1, assign2580_e2475_d_n3, assign2580_e2475_d_n4, assign2580_e2475_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard202 != 0.0)) {
        let assign2580_e2468: f64 = (locals.var_vsatphi / locals.var_leffe_um);
        let assign2580_e2470: f64 = (assign2580_e2468 - locals.var_ecrneff);
        let assign2580_e2471: f64 = (0.5 * assign2580_e2470);
        let assign2580_e2473: f64 = (assign2580_e2471 * locals.var_iecrit);
        (assign2580_e2473, ((0.5 * (locals.var_vsatphi_dn1 / locals.var_leffe_um)) * locals.var_iecrit), (((0.5 * ((locals.var_vsatphi_dn3 / locals.var_leffe_um) - locals.var_ecrneff_dn3)) * locals.var_iecrit) + (assign2580_e2471 * locals.var_iecrit_dn3)), ((0.5 * (locals.var_vsatphi_dn4 / locals.var_leffe_um)) * locals.var_iecrit), ((0.5 * (locals.var_vsatphi_dn5 / locals.var_leffe_um)) * locals.var_iecrit),)
    } else {
        (locals.var_fctrm, locals.var_fctrm_dn1, locals.var_fctrm_dn3, locals.var_fctrm_dn4, locals.var_fctrm_dn5,)
    }
};
        locals.var_fctrm = assign2580_e2475;
        locals.var_fctrm_dn1 = assign2580_e2475_d_n1;
        locals.var_fctrm_dn3 = assign2580_e2475_d_n3;
        locals.var_fctrm_dn4 = assign2580_e2475_d_n4;
        locals.var_fctrm_dn5 = assign2580_e2475_d_n5;

        let (assign2590_e2489, assign2590_e2489_d_n1, assign2590_e2489_d_n3, assign2590_e2489_d_n4, assign2590_e2489_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard202 != 0.0)) {
        let assign2590_e2482: f64 = (locals.var_vsatphi / locals.var_leffe_um);
        let assign2590_e2484: f64 = (assign2590_e2482 + locals.var_ecrneff);
        let assign2590_e2485: f64 = (0.5 * assign2590_e2484);
        let assign2590_e2487: f64 = (assign2590_e2485 * locals.var_iecrit);
        (assign2590_e2487, ((0.5 * (locals.var_vsatphi_dn1 / locals.var_leffe_um)) * locals.var_iecrit), (((0.5 * ((locals.var_vsatphi_dn3 / locals.var_leffe_um) + locals.var_ecrneff_dn3)) * locals.var_iecrit) + (assign2590_e2485 * locals.var_iecrit_dn3)), ((0.5 * (locals.var_vsatphi_dn4 / locals.var_leffe_um)) * locals.var_iecrit), ((0.5 * (locals.var_vsatphi_dn5 / locals.var_leffe_um)) * locals.var_iecrit),)
    } else {
        (locals.var_fctrp, locals.var_fctrp_dn1, locals.var_fctrp_dn3, locals.var_fctrp_dn4, locals.var_fctrp_dn5,)
    }
};
        locals.var_fctrp = assign2590_e2489;
        locals.var_fctrp_dn1 = assign2590_e2489_d_n1;
        locals.var_fctrp_dn3 = assign2590_e2489_d_n3;
        locals.var_fctrp_dn4 = assign2590_e2489_d_n4;
        locals.var_fctrp_dn5 = assign2590_e2489_d_n5;

        let (assign2600_e2500, assign2600_e2500_d_n1, assign2600_e2500_d_n3, assign2600_e2500_d_n4, assign2600_e2500_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard202 != 0.0)) {
        let assign2600_e2495: f64 = (locals.var_fctrm * locals.var_fctrm);
        let assign2600_e2497: f64 = (assign2600_e2495 + locals.var_dufctr);
        let assign2600_e2498: f64 = (assign2600_e2497).sqrt();
        (assign2600_e2498, (((locals.var_fctrm_dn1 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn1)) / (2.0 * assign2600_e2498)), ((((locals.var_fctrm_dn3 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn3)) + locals.var_dufctr_dn3) / (2.0 * assign2600_e2498)), (((locals.var_fctrm_dn4 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn4)) / (2.0 * assign2600_e2498)), (((locals.var_fctrm_dn5 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn5)) / (2.0 * assign2600_e2498)),)
    } else {
        (locals.var_sqrtm, locals.var_sqrtm_dn1, locals.var_sqrtm_dn3, locals.var_sqrtm_dn4, locals.var_sqrtm_dn5,)
    }
};
        locals.var_sqrtm = assign2600_e2500;
        locals.var_sqrtm_dn1 = assign2600_e2500_d_n1;
        locals.var_sqrtm_dn3 = assign2600_e2500_d_n3;
        locals.var_sqrtm_dn4 = assign2600_e2500_d_n4;
        locals.var_sqrtm_dn5 = assign2600_e2500_d_n5;

        let (assign2610_e2511, assign2610_e2511_d_n1, assign2610_e2511_d_n3, assign2610_e2511_d_n4, assign2610_e2511_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard202 != 0.0)) {
        let assign2610_e2506: f64 = (locals.var_fctrp * locals.var_fctrp);
        let assign2610_e2508: f64 = (assign2610_e2506 + locals.var_dufctr);
        let assign2610_e2509: f64 = (assign2610_e2508).sqrt();
        (assign2610_e2509, (((locals.var_fctrp_dn1 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn1)) / (2.0 * assign2610_e2509)), ((((locals.var_fctrp_dn3 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn3)) + locals.var_dufctr_dn3) / (2.0 * assign2610_e2509)), (((locals.var_fctrp_dn4 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn4)) / (2.0 * assign2610_e2509)), (((locals.var_fctrp_dn5 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn5)) / (2.0 * assign2610_e2509)),)
    } else {
        (locals.var_sqrtp, locals.var_sqrtp_dn1, locals.var_sqrtp_dn3, locals.var_sqrtp_dn4, locals.var_sqrtp_dn5,)
    }
};
        locals.var_sqrtp = assign2610_e2511;
        locals.var_sqrtp_dn1 = assign2610_e2511_d_n1;
        locals.var_sqrtp_dn3 = assign2610_e2511_d_n3;
        locals.var_sqrtp_dn4 = assign2610_e2511_d_n4;
        locals.var_sqrtp_dn5 = assign2610_e2511_d_n5;

        let (assign2620_e2521, assign2620_e2521_d_n1, assign2620_e2521_d_n3, assign2620_e2521_d_n4, assign2620_e2521_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard202 != 0.0)) {
        let assign2620_e2517: f64 = (locals.var_sqrtm + locals.var_sqrtp);
        let assign2620_e2519: f64 = (assign2620_e2517 - locals.var_uoff);
        (assign2620_e2519, (locals.var_sqrtm_dn1 + locals.var_sqrtp_dn1), ((locals.var_sqrtm_dn3 + locals.var_sqrtp_dn3) - locals.var_uoff_dn3), (locals.var_sqrtm_dn4 + locals.var_sqrtp_dn4), (locals.var_sqrtm_dn5 + locals.var_sqrtp_dn5),)
    } else {
        (locals.var_rmu, locals.var_rmu_dn1, locals.var_rmu_dn3, locals.var_rmu_dn4, locals.var_rmu_dn5,)
    }
};
        locals.var_rmu = assign2620_e2521;
        locals.var_rmu_dn1 = assign2620_e2521_d_n1;
        locals.var_rmu_dn3 = assign2620_e2521_d_n3;
        locals.var_rmu_dn4 = assign2620_e2521_d_n4;
        locals.var_rmu_dn5 = assign2620_e2521_d_n5;

        let (assign2630_e2539, assign2630_e2539_d_n1, assign2630_e2539_d_n3, assign2630_e2539_d_n4, assign2630_e2539_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard202 != 0.0)) {
        let assign2630_e2528: f64 = (locals.var_fctrm / locals.var_sqrtm);
        let assign2630_e2531: f64 = (locals.var_fctrp / locals.var_sqrtp);
        let assign2630_e2532: f64 = (assign2630_e2528 + assign2630_e2531);
        let assign2630_e2533: f64 = (0.5 * assign2630_e2532);
        let assign2630_e2535: f64 = (assign2630_e2533 * locals.var_iecrit);
        let assign2630_e2537: f64 = (assign2630_e2535 / locals.var_leffe_um);
        (assign2630_e2537, (((0.5 * ((((locals.var_fctrm_dn1 * locals.var_sqrtm) - (locals.var_fctrm * locals.var_sqrtm_dn1)) / (locals.var_sqrtm * locals.var_sqrtm)) + (((locals.var_fctrp_dn1 * locals.var_sqrtp) - (locals.var_fctrp * locals.var_sqrtp_dn1)) / (locals.var_sqrtp * locals.var_sqrtp)))) * locals.var_iecrit) / locals.var_leffe_um), ((((0.5 * ((((locals.var_fctrm_dn3 * locals.var_sqrtm) - (locals.var_fctrm * locals.var_sqrtm_dn3)) / (locals.var_sqrtm * locals.var_sqrtm)) + (((locals.var_fctrp_dn3 * locals.var_sqrtp) - (locals.var_fctrp * locals.var_sqrtp_dn3)) / (locals.var_sqrtp * locals.var_sqrtp)))) * locals.var_iecrit) + (assign2630_e2533 * locals.var_iecrit_dn3)) / locals.var_leffe_um), (((0.5 * ((((locals.var_fctrm_dn4 * locals.var_sqrtm) - (locals.var_fctrm * locals.var_sqrtm_dn4)) / (locals.var_sqrtm * locals.var_sqrtm)) + (((locals.var_fctrp_dn4 * locals.var_sqrtp) - (locals.var_fctrp * locals.var_sqrtp_dn4)) / (locals.var_sqrtp * locals.var_sqrtp)))) * locals.var_iecrit) / locals.var_leffe_um), (((0.5 * ((((locals.var_fctrm_dn5 * locals.var_sqrtm) - (locals.var_fctrm * locals.var_sqrtm_dn5)) / (locals.var_sqrtm * locals.var_sqrtm)) + (((locals.var_fctrp_dn5 * locals.var_sqrtp) - (locals.var_fctrp * locals.var_sqrtp_dn5)) / (locals.var_sqrtp * locals.var_sqrtp)))) * locals.var_iecrit) / locals.var_leffe_um),)
    } else {
        (locals.var_drmu, locals.var_drmu_dn1, locals.var_drmu_dn3, locals.var_drmu_dn4, locals.var_drmu_dn5,)
    }
};
        locals.var_drmu = assign2630_e2539;
        locals.var_drmu_dn1 = assign2630_e2539_d_n1;
        locals.var_drmu_dn3 = assign2630_e2539_d_n3;
        locals.var_drmu_dn4 = assign2630_e2539_d_n4;
        locals.var_drmu_dn5 = assign2630_e2539_d_n5;

        let (assign2640_e2564, assign2640_e2564_d_n1, assign2640_e2564_d_n3, assign2640_e2564_d_n4, assign2640_e2564_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard202 != 0.0)) {
        let assign2640_e2545: f64 = (2.0 * locals.var_fsatphi);
        let assign2640_e2548: f64 = (1.0 - locals.var_fsatphi);
        let assign2640_e2549: f64 = (assign2640_e2545 * assign2640_e2548);
        let assign2640_e2553: f64 = (locals.var_drmu * locals.var_vsatphi);
        let assign2640_e2556: f64 = (1.0 + locals.var_rmu);
        let assign2640_e2557: f64 = (assign2640_e2553 / assign2640_e2556);
        let assign2640_e2558: f64 = (1.0 - assign2640_e2557);
        let assign2640_e2559: f64 = (assign2640_e2549 * assign2640_e2558);
        let assign2640_e2561: f64 = (assign2640_e2559 / locals.var_vsatphi);
        let assign2640_e2562: f64 = (assign2640_e2561).sqrt();
        (assign2640_e2562, (((((((((2.0 * locals.var_fsatphi_dn1) * assign2640_e2548) + (assign2640_e2545 * (-locals.var_fsatphi_dn1))) * assign2640_e2558) + (assign2640_e2549 * (-(((((locals.var_drmu_dn1 * locals.var_vsatphi) + (locals.var_drmu * locals.var_vsatphi_dn1)) * assign2640_e2556) - (assign2640_e2553 * locals.var_rmu_dn1)) / (assign2640_e2556 * assign2640_e2556))))) * locals.var_vsatphi) - (assign2640_e2559 * locals.var_vsatphi_dn1)) / (locals.var_vsatphi * locals.var_vsatphi)) / (2.0 * assign2640_e2562)), (((((((((2.0 * locals.var_fsatphi_dn3) * assign2640_e2548) + (assign2640_e2545 * (-locals.var_fsatphi_dn3))) * assign2640_e2558) + (assign2640_e2549 * (-(((((locals.var_drmu_dn3 * locals.var_vsatphi) + (locals.var_drmu * locals.var_vsatphi_dn3)) * assign2640_e2556) - (assign2640_e2553 * locals.var_rmu_dn3)) / (assign2640_e2556 * assign2640_e2556))))) * locals.var_vsatphi) - (assign2640_e2559 * locals.var_vsatphi_dn3)) / (locals.var_vsatphi * locals.var_vsatphi)) / (2.0 * assign2640_e2562)), (((((((((2.0 * locals.var_fsatphi_dn4) * assign2640_e2548) + (assign2640_e2545 * (-locals.var_fsatphi_dn4))) * assign2640_e2558) + (assign2640_e2549 * (-(((((locals.var_drmu_dn4 * locals.var_vsatphi) + (locals.var_drmu * locals.var_vsatphi_dn4)) * assign2640_e2556) - (assign2640_e2553 * locals.var_rmu_dn4)) / (assign2640_e2556 * assign2640_e2556))))) * locals.var_vsatphi) - (assign2640_e2559 * locals.var_vsatphi_dn4)) / (locals.var_vsatphi * locals.var_vsatphi)) / (2.0 * assign2640_e2562)), (((((((((2.0 * locals.var_fsatphi_dn5) * assign2640_e2548) + (assign2640_e2545 * (-locals.var_fsatphi_dn5))) * assign2640_e2558) + (assign2640_e2549 * (-(((((locals.var_drmu_dn5 * locals.var_vsatphi) + (locals.var_drmu * locals.var_vsatphi_dn5)) * assign2640_e2556) - (assign2640_e2553 * locals.var_rmu_dn5)) / (assign2640_e2556 * assign2640_e2556))))) * locals.var_vsatphi) - (assign2640_e2559 * locals.var_vsatphi_dn5)) / (locals.var_vsatphi * locals.var_vsatphi)) / (2.0 * assign2640_e2562)),)
    } else {
        (locals.var_dfe, locals.var_dfe_dn1, locals.var_dfe_dn3, locals.var_dfe_dn4, locals.var_dfe_dn5,)
    }
};
        locals.var_dfe = assign2640_e2564;
        locals.var_dfe_dn1 = assign2640_e2564_d_n1;
        locals.var_dfe_dn3 = assign2640_e2564_d_n3;
        locals.var_dfe_dn4 = assign2640_e2564_d_n4;
        locals.var_dfe_dn5 = assign2640_e2564_d_n5;

        let (assign2650_e2580, assign2650_e2580_d_n1, assign2650_e2580_d_n3, assign2650_e2580_d_n4, assign2650_e2580_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard202 == 0.0)) {
        let assign2650_e2571: f64 = (2.0 * locals.var_fsatphi);
        let assign2650_e2574: f64 = (1.0 - locals.var_fsatphi);
        let assign2650_e2575: f64 = (assign2650_e2571 * assign2650_e2574);
        let assign2650_e2577: f64 = (assign2650_e2575 / locals.var_vsatphi);
        let assign2650_e2578: f64 = (assign2650_e2577).sqrt();
        (assign2650_e2578, (((((((2.0 * locals.var_fsatphi_dn1) * assign2650_e2574) + (assign2650_e2571 * (-locals.var_fsatphi_dn1))) * locals.var_vsatphi) - (assign2650_e2575 * locals.var_vsatphi_dn1)) / (locals.var_vsatphi * locals.var_vsatphi)) / (2.0 * assign2650_e2578)), (((((((2.0 * locals.var_fsatphi_dn3) * assign2650_e2574) + (assign2650_e2571 * (-locals.var_fsatphi_dn3))) * locals.var_vsatphi) - (assign2650_e2575 * locals.var_vsatphi_dn3)) / (locals.var_vsatphi * locals.var_vsatphi)) / (2.0 * assign2650_e2578)), (((((((2.0 * locals.var_fsatphi_dn4) * assign2650_e2574) + (assign2650_e2571 * (-locals.var_fsatphi_dn4))) * locals.var_vsatphi) - (assign2650_e2575 * locals.var_vsatphi_dn4)) / (locals.var_vsatphi * locals.var_vsatphi)) / (2.0 * assign2650_e2578)), (((((((2.0 * locals.var_fsatphi_dn5) * assign2650_e2574) + (assign2650_e2571 * (-locals.var_fsatphi_dn5))) * locals.var_vsatphi) - (assign2650_e2575 * locals.var_vsatphi_dn5)) / (locals.var_vsatphi * locals.var_vsatphi)) / (2.0 * assign2650_e2578)),)
    } else {
        (locals.var_dfe, locals.var_dfe_dn1, locals.var_dfe_dn3, locals.var_dfe_dn4, locals.var_dfe_dn5,)
    }
};
        locals.var_dfe = assign2650_e2580;
        locals.var_dfe_dn1 = assign2650_e2580_d_n1;
        locals.var_dfe_dn3 = assign2650_e2580_d_n3;
        locals.var_dfe_dn4 = assign2650_e2580_d_n4;
        locals.var_dfe_dn5 = assign2650_e2580_d_n5;

        let (assign2660_e2594, assign2660_e2594_d_n1, assign2660_e2594_d_n3, assign2660_e2594_d_n4, assign2660_e2594_d_n5,) = {
    if (locals.var_guard201 != 0.0) {
        let assign2660_e2585: f64 = (locals.var_pe + locals.var_vsat);
        let assign2660_e2586: f64 = (locals.var_dfsq * assign2660_e2585);
        let assign2660_e2589: f64 = (locals.var_dfe * locals.var_dfe);
        let assign2660_e2590: f64 = (assign2660_e2586 / assign2660_e2589);
        let assign2660_e2592: f64 = (assign2660_e2590 - locals.var_vsatphi);
        (assign2660_e2592, (((((locals.var_dfsq * (locals.var_pe_dn1 + locals.var_vsat_dn1)) * assign2660_e2589) - (assign2660_e2586 * ((locals.var_dfe_dn1 * locals.var_dfe) + (locals.var_dfe * locals.var_dfe_dn1)))) / (assign2660_e2589 * assign2660_e2589)) - locals.var_vsatphi_dn1), ((((((locals.var_dfsq_dn3 * assign2660_e2585) + (locals.var_dfsq * (locals.var_pe_dn3 + locals.var_vsat_dn3))) * assign2660_e2589) - (assign2660_e2586 * ((locals.var_dfe_dn3 * locals.var_dfe) + (locals.var_dfe * locals.var_dfe_dn3)))) / (assign2660_e2589 * assign2660_e2589)) - locals.var_vsatphi_dn3), (((((locals.var_dfsq * (locals.var_pe_dn4 + locals.var_vsat_dn4)) * assign2660_e2589) - (assign2660_e2586 * ((locals.var_dfe_dn4 * locals.var_dfe) + (locals.var_dfe * locals.var_dfe_dn4)))) / (assign2660_e2589 * assign2660_e2589)) - locals.var_vsatphi_dn4), (((((locals.var_dfsq * (locals.var_pe_dn5 + locals.var_vsat_dn5)) * assign2660_e2589) - (assign2660_e2586 * ((locals.var_dfe_dn5 * locals.var_dfe) + (locals.var_dfe * locals.var_dfe_dn5)))) / (assign2660_e2589 * assign2660_e2589)) - locals.var_vsatphi_dn5),)
    } else {
        (locals.var_dpee, locals.var_dpee_dn1, locals.var_dpee_dn3, locals.var_dpee_dn4, locals.var_dpee_dn5,)
    }
};
        locals.var_dpee = assign2660_e2594;
        locals.var_dpee_dn1 = assign2660_e2594_d_n1;
        locals.var_dpee_dn3 = assign2660_e2594_d_n3;
        locals.var_dpee_dn4 = assign2660_e2594_d_n4;
        locals.var_dpee_dn5 = assign2660_e2594_d_n5;

        let (assign2670_e2606, assign2670_e2606_d_n1, assign2670_e2606_d_n3, assign2670_e2606_d_n4, assign2670_e2606_d_n5,) = {
    if (locals.var_guard201 != 0.0) {
        let assign2670_e2599: f64 = (p.p47 * locals.var_vsat);
        let assign2670_e2602: f64 = (p.p47 + locals.var_vsatphi);
        let assign2670_e2603: f64 = (assign2670_e2599 / assign2670_e2602);
        let assign2670_e2604: f64 = (locals.var_atspo + assign2670_e2603);
        (assign2670_e2604, ((((p.p47 * locals.var_vsat_dn1) * assign2670_e2602) - (assign2670_e2599 * locals.var_vsatphi_dn1)) / (assign2670_e2602 * assign2670_e2602)), (locals.var_atspo_dn3 + ((((p.p47 * locals.var_vsat_dn3) * assign2670_e2602) - (assign2670_e2599 * locals.var_vsatphi_dn3)) / (assign2670_e2602 * assign2670_e2602))), ((((p.p47 * locals.var_vsat_dn4) * assign2670_e2602) - (assign2670_e2599 * locals.var_vsatphi_dn4)) / (assign2670_e2602 * assign2670_e2602)), ((((p.p47 * locals.var_vsat_dn5) * assign2670_e2602) - (assign2670_e2599 * locals.var_vsatphi_dn5)) / (assign2670_e2602 * assign2670_e2602)),)
    } else {
        (locals.var_atseff, locals.var_atseff_dn1, locals.var_atseff_dn3, locals.var_atseff_dn4, locals.var_atseff_dn5,)
    }
};
        locals.var_atseff = assign2670_e2606;
        locals.var_atseff_dn1 = assign2670_e2606_d_n1;
        locals.var_atseff_dn3 = assign2670_e2606_d_n3;
        locals.var_atseff_dn4 = assign2670_e2606_d_n4;
        locals.var_atseff_dn5 = assign2670_e2606_d_n5;

        let (assign2680_e2614, assign2680_e2614_d_n1, assign2680_e2614_d_n3, assign2680_e2614_d_n4, assign2680_e2614_d_n5,) = {
    if (locals.var_guard201 != 0.0) {
        let assign2680_e2610: f64 = (4.0 * locals.var_atseff);
        let assign2680_e2612: f64 = (assign2680_e2610 * locals.var_atseff);
        (assign2680_e2612, (((4.0 * locals.var_atseff_dn1) * locals.var_atseff) + (assign2680_e2610 * locals.var_atseff_dn1)), (((4.0 * locals.var_atseff_dn3) * locals.var_atseff) + (assign2680_e2610 * locals.var_atseff_dn3)), (((4.0 * locals.var_atseff_dn4) * locals.var_atseff) + (assign2680_e2610 * locals.var_atseff_dn4)), (((4.0 * locals.var_atseff_dn5) * locals.var_atseff) + (assign2680_e2610 * locals.var_atseff_dn5)),)
    } else {
        (locals.var_fouratsq, locals.var_fouratsq_dn1, locals.var_fouratsq_dn3, locals.var_fouratsq_dn4, locals.var_fouratsq_dn5,)
    }
};
        locals.var_fouratsq = assign2680_e2614;
        locals.var_fouratsq_dn1 = assign2680_e2614_d_n1;
        locals.var_fouratsq_dn3 = assign2680_e2614_d_n3;
        locals.var_fouratsq_dn4 = assign2680_e2614_d_n4;
        locals.var_fouratsq_dn5 = assign2680_e2614_d_n5;

        let (assign2690_e2644, assign2690_e2644_d_n1, assign2690_e2644_d_n3, assign2690_e2644_d_n4, assign2690_e2644_d_n5,) = {
    if (locals.var_guard201 != 0.0) {
        let assign2690_e2618: f64 = (2.0 * locals.var_vrbi);
        let assign2690_e2620: f64 = (assign2690_e2618 * locals.var_vsatphi);
        let assign2690_e2623: f64 = (locals.var_vrbi - locals.var_vsatphi);
        let assign2690_e2626: f64 = (locals.var_vrbi - locals.var_vsatphi);
        let assign2690_e2627: f64 = (assign2690_e2623 * assign2690_e2626);
        let assign2690_e2629: f64 = (assign2690_e2627 + locals.var_fouratsq);
        let assign2690_e2630: f64 = (assign2690_e2629).sqrt();
        let assign2690_e2633: f64 = (locals.var_vrbi + locals.var_vsatphi);
        let assign2690_e2636: f64 = (locals.var_vrbi + locals.var_vsatphi);
        let assign2690_e2637: f64 = (assign2690_e2633 * assign2690_e2636);
        let assign2690_e2639: f64 = (assign2690_e2637 + locals.var_fouratsq);
        let assign2690_e2640: f64 = (assign2690_e2639).sqrt();
        let assign2690_e2641: f64 = (assign2690_e2630 + assign2690_e2640);
        let assign2690_e2642: f64 = (assign2690_e2620 / assign2690_e2641);
        (assign2690_e2642, ((((assign2690_e2618 * locals.var_vsatphi_dn1) * assign2690_e2641) - (assign2690_e2620 * ((((((-locals.var_vsatphi_dn1) * assign2690_e2626) + (assign2690_e2623 * (-locals.var_vsatphi_dn1))) + locals.var_fouratsq_dn1) / (2.0 * assign2690_e2630)) + ((((locals.var_vsatphi_dn1 * assign2690_e2636) + (assign2690_e2633 * locals.var_vsatphi_dn1)) + locals.var_fouratsq_dn1) / (2.0 * assign2690_e2640))))) / (assign2690_e2641 * assign2690_e2641)), ((((assign2690_e2618 * locals.var_vsatphi_dn3) * assign2690_e2641) - (assign2690_e2620 * ((((((-locals.var_vsatphi_dn3) * assign2690_e2626) + (assign2690_e2623 * (-locals.var_vsatphi_dn3))) + locals.var_fouratsq_dn3) / (2.0 * assign2690_e2630)) + ((((locals.var_vsatphi_dn3 * assign2690_e2636) + (assign2690_e2633 * locals.var_vsatphi_dn3)) + locals.var_fouratsq_dn3) / (2.0 * assign2690_e2640))))) / (assign2690_e2641 * assign2690_e2641)), ((((((2.0 * locals.var_vrbi_dn4) * locals.var_vsatphi) + (assign2690_e2618 * locals.var_vsatphi_dn4)) * assign2690_e2641) - (assign2690_e2620 * ((((((locals.var_vrbi_dn4 - locals.var_vsatphi_dn4) * assign2690_e2626) + (assign2690_e2623 * (locals.var_vrbi_dn4 - locals.var_vsatphi_dn4))) + locals.var_fouratsq_dn4) / (2.0 * assign2690_e2630)) + (((((locals.var_vrbi_dn4 + locals.var_vsatphi_dn4) * assign2690_e2636) + (assign2690_e2633 * (locals.var_vrbi_dn4 + locals.var_vsatphi_dn4))) + locals.var_fouratsq_dn4) / (2.0 * assign2690_e2640))))) / (assign2690_e2641 * assign2690_e2641)), ((((((2.0 * locals.var_vrbi_dn5) * locals.var_vsatphi) + (assign2690_e2618 * locals.var_vsatphi_dn5)) * assign2690_e2641) - (assign2690_e2620 * ((((((locals.var_vrbi_dn5 - locals.var_vsatphi_dn5) * assign2690_e2626) + (assign2690_e2623 * (locals.var_vrbi_dn5 - locals.var_vsatphi_dn5))) + locals.var_fouratsq_dn5) / (2.0 * assign2690_e2630)) + (((((locals.var_vrbi_dn5 + locals.var_vsatphi_dn5) * assign2690_e2636) + (assign2690_e2633 * (locals.var_vrbi_dn5 + locals.var_vsatphi_dn5))) + locals.var_fouratsq_dn5) / (2.0 * assign2690_e2640))))) / (assign2690_e2641 * assign2690_e2641)),)
    } else {
        (locals.var_vrbeff, locals.var_vrbeff_dn1, locals.var_vrbeff_dn3, locals.var_vrbeff_dn4, locals.var_vrbeff_dn5,)
    }
};
        locals.var_vrbeff = assign2690_e2644;
        locals.var_vrbeff_dn1 = assign2690_e2644_d_n1;
        locals.var_vrbeff_dn3 = assign2690_e2644_d_n3;
        locals.var_vrbeff_dn4 = assign2690_e2644_d_n4;
        locals.var_vrbeff_dn5 = assign2690_e2644_d_n5;

        let assign2700_e2647: f64 = if p.p63 > 2.0 { 1.0 } else { 0.0 };
        locals.var_guard203 = assign2700_e2647;

        let (assign2710_e2661, assign2710_e2661_d_n1, assign2710_e2661_d_n3, assign2710_e2661_d_n4, assign2710_e2661_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard203 != 0.0)) {
        let assign2710_e2654: f64 = (p.p47 * locals.var_vrbeff);
        let assign2710_e2657: f64 = (p.p47 + locals.var_vsatphi);
        let assign2710_e2658: f64 = (assign2710_e2654 / assign2710_e2657);
        let assign2710_e2659: f64 = (locals.var_atspo + assign2710_e2658);
        (assign2710_e2659, ((((p.p47 * locals.var_vrbeff_dn1) * assign2710_e2657) - (assign2710_e2654 * locals.var_vsatphi_dn1)) / (assign2710_e2657 * assign2710_e2657)), (locals.var_atspo_dn3 + ((((p.p47 * locals.var_vrbeff_dn3) * assign2710_e2657) - (assign2710_e2654 * locals.var_vsatphi_dn3)) / (assign2710_e2657 * assign2710_e2657))), ((((p.p47 * locals.var_vrbeff_dn4) * assign2710_e2657) - (assign2710_e2654 * locals.var_vsatphi_dn4)) / (assign2710_e2657 * assign2710_e2657)), ((((p.p47 * locals.var_vrbeff_dn5) * assign2710_e2657) - (assign2710_e2654 * locals.var_vsatphi_dn5)) / (assign2710_e2657 * assign2710_e2657)),)
    } else {
        (locals.var_atseff, locals.var_atseff_dn1, locals.var_atseff_dn3, locals.var_atseff_dn4, locals.var_atseff_dn5,)
    }
};
        locals.var_atseff = assign2710_e2661;
        locals.var_atseff_dn1 = assign2710_e2661_d_n1;
        locals.var_atseff_dn3 = assign2710_e2661_d_n3;
        locals.var_atseff_dn4 = assign2710_e2661_d_n4;
        locals.var_atseff_dn5 = assign2710_e2661_d_n5;

        let (assign2720_e2671, assign2720_e2671_d_n1, assign2720_e2671_d_n3, assign2720_e2671_d_n4, assign2720_e2671_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard203 != 0.0)) {
        let assign2720_e2667: f64 = (4.0 * locals.var_atseff);
        let assign2720_e2669: f64 = (assign2720_e2667 * locals.var_atseff);
        (assign2720_e2669, (((4.0 * locals.var_atseff_dn1) * locals.var_atseff) + (assign2720_e2667 * locals.var_atseff_dn1)), (((4.0 * locals.var_atseff_dn3) * locals.var_atseff) + (assign2720_e2667 * locals.var_atseff_dn3)), (((4.0 * locals.var_atseff_dn4) * locals.var_atseff) + (assign2720_e2667 * locals.var_atseff_dn4)), (((4.0 * locals.var_atseff_dn5) * locals.var_atseff) + (assign2720_e2667 * locals.var_atseff_dn5)),)
    } else {
        (locals.var_fouratsq, locals.var_fouratsq_dn1, locals.var_fouratsq_dn3, locals.var_fouratsq_dn4, locals.var_fouratsq_dn5,)
    }
};
        locals.var_fouratsq = assign2720_e2671;
        locals.var_fouratsq_dn1 = assign2720_e2671_d_n1;
        locals.var_fouratsq_dn3 = assign2720_e2671_d_n3;
        locals.var_fouratsq_dn4 = assign2720_e2671_d_n4;
        locals.var_fouratsq_dn5 = assign2720_e2671_d_n5;

        let (assign2730_e2703, assign2730_e2703_d_n1, assign2730_e2703_d_n3, assign2730_e2703_d_n4, assign2730_e2703_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard203 != 0.0)) {
        let assign2730_e2677: f64 = (2.0 * locals.var_vrbi);
        let assign2730_e2679: f64 = (assign2730_e2677 * locals.var_vsatphi);
        let assign2730_e2682: f64 = (locals.var_vrbi - locals.var_vsatphi);
        let assign2730_e2685: f64 = (locals.var_vrbi - locals.var_vsatphi);
        let assign2730_e2686: f64 = (assign2730_e2682 * assign2730_e2685);
        let assign2730_e2688: f64 = (assign2730_e2686 + locals.var_fouratsq);
        let assign2730_e2689: f64 = (assign2730_e2688).sqrt();
        let assign2730_e2692: f64 = (locals.var_vrbi + locals.var_vsatphi);
        let assign2730_e2695: f64 = (locals.var_vrbi + locals.var_vsatphi);
        let assign2730_e2696: f64 = (assign2730_e2692 * assign2730_e2695);
        let assign2730_e2698: f64 = (assign2730_e2696 + locals.var_fouratsq);
        let assign2730_e2699: f64 = (assign2730_e2698).sqrt();
        let assign2730_e2700: f64 = (assign2730_e2689 + assign2730_e2699);
        let assign2730_e2701: f64 = (assign2730_e2679 / assign2730_e2700);
        (assign2730_e2701, ((((assign2730_e2677 * locals.var_vsatphi_dn1) * assign2730_e2700) - (assign2730_e2679 * ((((((-locals.var_vsatphi_dn1) * assign2730_e2685) + (assign2730_e2682 * (-locals.var_vsatphi_dn1))) + locals.var_fouratsq_dn1) / (2.0 * assign2730_e2689)) + ((((locals.var_vsatphi_dn1 * assign2730_e2695) + (assign2730_e2692 * locals.var_vsatphi_dn1)) + locals.var_fouratsq_dn1) / (2.0 * assign2730_e2699))))) / (assign2730_e2700 * assign2730_e2700)), ((((assign2730_e2677 * locals.var_vsatphi_dn3) * assign2730_e2700) - (assign2730_e2679 * ((((((-locals.var_vsatphi_dn3) * assign2730_e2685) + (assign2730_e2682 * (-locals.var_vsatphi_dn3))) + locals.var_fouratsq_dn3) / (2.0 * assign2730_e2689)) + ((((locals.var_vsatphi_dn3 * assign2730_e2695) + (assign2730_e2692 * locals.var_vsatphi_dn3)) + locals.var_fouratsq_dn3) / (2.0 * assign2730_e2699))))) / (assign2730_e2700 * assign2730_e2700)), ((((((2.0 * locals.var_vrbi_dn4) * locals.var_vsatphi) + (assign2730_e2677 * locals.var_vsatphi_dn4)) * assign2730_e2700) - (assign2730_e2679 * ((((((locals.var_vrbi_dn4 - locals.var_vsatphi_dn4) * assign2730_e2685) + (assign2730_e2682 * (locals.var_vrbi_dn4 - locals.var_vsatphi_dn4))) + locals.var_fouratsq_dn4) / (2.0 * assign2730_e2689)) + (((((locals.var_vrbi_dn4 + locals.var_vsatphi_dn4) * assign2730_e2695) + (assign2730_e2692 * (locals.var_vrbi_dn4 + locals.var_vsatphi_dn4))) + locals.var_fouratsq_dn4) / (2.0 * assign2730_e2699))))) / (assign2730_e2700 * assign2730_e2700)), ((((((2.0 * locals.var_vrbi_dn5) * locals.var_vsatphi) + (assign2730_e2677 * locals.var_vsatphi_dn5)) * assign2730_e2700) - (assign2730_e2679 * ((((((locals.var_vrbi_dn5 - locals.var_vsatphi_dn5) * assign2730_e2685) + (assign2730_e2682 * (locals.var_vrbi_dn5 - locals.var_vsatphi_dn5))) + locals.var_fouratsq_dn5) / (2.0 * assign2730_e2689)) + (((((locals.var_vrbi_dn5 + locals.var_vsatphi_dn5) * assign2730_e2695) + (assign2730_e2692 * (locals.var_vrbi_dn5 + locals.var_vsatphi_dn5))) + locals.var_fouratsq_dn5) / (2.0 * assign2730_e2699))))) / (assign2730_e2700 * assign2730_e2700)),)
    } else {
        (locals.var_vrbeff, locals.var_vrbeff_dn1, locals.var_vrbeff_dn3, locals.var_vrbeff_dn4, locals.var_vrbeff_dn5,)
    }
};
        locals.var_vrbeff = assign2730_e2703;
        locals.var_vrbeff_dn1 = assign2730_e2703_d_n1;
        locals.var_vrbeff_dn3 = assign2730_e2703_d_n3;
        locals.var_vrbeff_dn4 = assign2730_e2703_d_n4;
        locals.var_vrbeff_dn5 = assign2730_e2703_d_n5;

        let (assign2740_e2714, assign2740_e2714_d_n1, assign2740_e2714_d_n3, assign2740_e2714_d_n4, assign2740_e2714_d_n5,) = {
    if (locals.var_guard201 != 0.0) {
        let assign2740_e2709: f64 = (locals.var_dpee + locals.var_vrbeff);
        let assign2740_e2710: f64 = (assign2740_e2709).sqrt();
        let assign2740_e2711: f64 = (locals.var_dfe * assign2740_e2710);
        let assign2740_e2712: f64 = (1.0 - assign2740_e2711);
        (assign2740_e2712, (-((locals.var_dfe_dn1 * assign2740_e2710) + (locals.var_dfe * ((locals.var_dpee_dn1 + locals.var_vrbeff_dn1) / (2.0 * assign2740_e2710))))), (-((locals.var_dfe_dn3 * assign2740_e2710) + (locals.var_dfe * ((locals.var_dpee_dn3 + locals.var_vrbeff_dn3) / (2.0 * assign2740_e2710))))), (-((locals.var_dfe_dn4 * assign2740_e2710) + (locals.var_dfe * ((locals.var_dpee_dn4 + locals.var_vrbeff_dn4) / (2.0 * assign2740_e2710))))), (-((locals.var_dfe_dn5 * assign2740_e2710) + (locals.var_dfe * ((locals.var_dpee_dn5 + locals.var_vrbeff_dn5) / (2.0 * assign2740_e2710))))),)
    } else {
        (locals.var_dpfctr, locals.var_dpfctr_dn1, locals.var_dpfctr_dn3, locals.var_dpfctr_dn4, locals.var_dpfctr_dn5,)
    }
};
        locals.var_dpfctr = assign2740_e2714;
        locals.var_dpfctr_dn1 = assign2740_e2714_d_n1;
        locals.var_dpfctr_dn3 = assign2740_e2714_d_n3;
        locals.var_dpfctr_dn4 = assign2740_e2714_d_n4;
        locals.var_dpfctr_dn5 = assign2740_e2714_d_n5;

        let assign2750_e2717: f64 = if locals.var_iecrit > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard204 = assign2750_e2717;

        let (assign2760_e2731, assign2760_e2731_d_n1, assign2760_e2731_d_n3, assign2760_e2731_d_n4, assign2760_e2731_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign2760_e2724: f64 = (locals.var_vrbeff / locals.var_leffe_um);
        let assign2760_e2726: f64 = (assign2760_e2724 - locals.var_ecrneff);
        let assign2760_e2727: f64 = (0.5 * assign2760_e2726);
        let assign2760_e2729: f64 = (assign2760_e2727 * locals.var_iecrit);
        (assign2760_e2729, ((0.5 * (locals.var_vrbeff_dn1 / locals.var_leffe_um)) * locals.var_iecrit), (((0.5 * ((locals.var_vrbeff_dn3 / locals.var_leffe_um) - locals.var_ecrneff_dn3)) * locals.var_iecrit) + (assign2760_e2727 * locals.var_iecrit_dn3)), ((0.5 * (locals.var_vrbeff_dn4 / locals.var_leffe_um)) * locals.var_iecrit), ((0.5 * (locals.var_vrbeff_dn5 / locals.var_leffe_um)) * locals.var_iecrit),)
    } else {
        (locals.var_fctrm, locals.var_fctrm_dn1, locals.var_fctrm_dn3, locals.var_fctrm_dn4, locals.var_fctrm_dn5,)
    }
};
        locals.var_fctrm = assign2760_e2731;
        locals.var_fctrm_dn1 = assign2760_e2731_d_n1;
        locals.var_fctrm_dn3 = assign2760_e2731_d_n3;
        locals.var_fctrm_dn4 = assign2760_e2731_d_n4;
        locals.var_fctrm_dn5 = assign2760_e2731_d_n5;

        let (assign2770_e2745, assign2770_e2745_d_n1, assign2770_e2745_d_n3, assign2770_e2745_d_n4, assign2770_e2745_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign2770_e2738: f64 = (locals.var_vrbeff / locals.var_leffe_um);
        let assign2770_e2740: f64 = (assign2770_e2738 + locals.var_ecrneff);
        let assign2770_e2741: f64 = (0.5 * assign2770_e2740);
        let assign2770_e2743: f64 = (assign2770_e2741 * locals.var_iecrit);
        (assign2770_e2743, ((0.5 * (locals.var_vrbeff_dn1 / locals.var_leffe_um)) * locals.var_iecrit), (((0.5 * ((locals.var_vrbeff_dn3 / locals.var_leffe_um) + locals.var_ecrneff_dn3)) * locals.var_iecrit) + (assign2770_e2741 * locals.var_iecrit_dn3)), ((0.5 * (locals.var_vrbeff_dn4 / locals.var_leffe_um)) * locals.var_iecrit), ((0.5 * (locals.var_vrbeff_dn5 / locals.var_leffe_um)) * locals.var_iecrit),)
    } else {
        (locals.var_fctrp, locals.var_fctrp_dn1, locals.var_fctrp_dn3, locals.var_fctrp_dn4, locals.var_fctrp_dn5,)
    }
};
        locals.var_fctrp = assign2770_e2745;
        locals.var_fctrp_dn1 = assign2770_e2745_d_n1;
        locals.var_fctrp_dn3 = assign2770_e2745_d_n3;
        locals.var_fctrp_dn4 = assign2770_e2745_d_n4;
        locals.var_fctrp_dn5 = assign2770_e2745_d_n5;

        let (assign2780_e2756, assign2780_e2756_d_n1, assign2780_e2756_d_n3, assign2780_e2756_d_n4, assign2780_e2756_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign2780_e2751: f64 = (locals.var_fctrm * locals.var_fctrm);
        let assign2780_e2753: f64 = (assign2780_e2751 + locals.var_dufctr);
        let assign2780_e2754: f64 = (assign2780_e2753).sqrt();
        (assign2780_e2754, (((locals.var_fctrm_dn1 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn1)) / (2.0 * assign2780_e2754)), ((((locals.var_fctrm_dn3 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn3)) + locals.var_dufctr_dn3) / (2.0 * assign2780_e2754)), (((locals.var_fctrm_dn4 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn4)) / (2.0 * assign2780_e2754)), (((locals.var_fctrm_dn5 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn5)) / (2.0 * assign2780_e2754)),)
    } else {
        (locals.var_sqrtm, locals.var_sqrtm_dn1, locals.var_sqrtm_dn3, locals.var_sqrtm_dn4, locals.var_sqrtm_dn5,)
    }
};
        locals.var_sqrtm = assign2780_e2756;
        locals.var_sqrtm_dn1 = assign2780_e2756_d_n1;
        locals.var_sqrtm_dn3 = assign2780_e2756_d_n3;
        locals.var_sqrtm_dn4 = assign2780_e2756_d_n4;
        locals.var_sqrtm_dn5 = assign2780_e2756_d_n5;

        let (assign2790_e2767, assign2790_e2767_d_n1, assign2790_e2767_d_n3, assign2790_e2767_d_n4, assign2790_e2767_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign2790_e2762: f64 = (locals.var_fctrp * locals.var_fctrp);
        let assign2790_e2764: f64 = (assign2790_e2762 + locals.var_dufctr);
        let assign2790_e2765: f64 = (assign2790_e2764).sqrt();
        (assign2790_e2765, (((locals.var_fctrp_dn1 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn1)) / (2.0 * assign2790_e2765)), ((((locals.var_fctrp_dn3 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn3)) + locals.var_dufctr_dn3) / (2.0 * assign2790_e2765)), (((locals.var_fctrp_dn4 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn4)) / (2.0 * assign2790_e2765)), (((locals.var_fctrp_dn5 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn5)) / (2.0 * assign2790_e2765)),)
    } else {
        (locals.var_sqrtp, locals.var_sqrtp_dn1, locals.var_sqrtp_dn3, locals.var_sqrtp_dn4, locals.var_sqrtp_dn5,)
    }
};
        locals.var_sqrtp = assign2790_e2767;
        locals.var_sqrtp_dn1 = assign2790_e2767_d_n1;
        locals.var_sqrtp_dn3 = assign2790_e2767_d_n3;
        locals.var_sqrtp_dn4 = assign2790_e2767_d_n4;
        locals.var_sqrtp_dn5 = assign2790_e2767_d_n5;

        let (assign2800_e2777, assign2800_e2777_d_n1, assign2800_e2777_d_n3, assign2800_e2777_d_n4, assign2800_e2777_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard204 != 0.0)) {
        let assign2800_e2773: f64 = (locals.var_sqrtm + locals.var_sqrtp);
        let assign2800_e2775: f64 = (assign2800_e2773 - locals.var_uoff);
        (assign2800_e2775, (locals.var_sqrtm_dn1 + locals.var_sqrtp_dn1), ((locals.var_sqrtm_dn3 + locals.var_sqrtp_dn3) - locals.var_uoff_dn3), (locals.var_sqrtm_dn4 + locals.var_sqrtp_dn4), (locals.var_sqrtm_dn5 + locals.var_sqrtp_dn5),)
    } else {
        (locals.var_rmu, locals.var_rmu_dn1, locals.var_rmu_dn3, locals.var_rmu_dn4, locals.var_rmu_dn5,)
    }
};
        locals.var_rmu = assign2800_e2777;
        locals.var_rmu_dn1 = assign2800_e2777_d_n1;
        locals.var_rmu_dn3 = assign2800_e2777_d_n3;
        locals.var_rmu_dn4 = assign2800_e2777_d_n4;
        locals.var_rmu_dn5 = assign2800_e2777_d_n5;

        let (assign2810_e2784, assign2810_e2784_d_n1, assign2810_e2784_d_n3, assign2810_e2784_d_n4, assign2810_e2784_d_n5,) = {
    if ((locals.var_guard201 != 0.0) && (locals.var_guard204 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rmu, locals.var_rmu_dn1, locals.var_rmu_dn3, locals.var_rmu_dn4, locals.var_rmu_dn5,)
    }
};
        locals.var_rmu = assign2810_e2784;
        locals.var_rmu_dn1 = assign2810_e2784_d_n1;
        locals.var_rmu_dn3 = assign2810_e2784_d_n3;
        locals.var_rmu_dn4 = assign2810_e2784_d_n4;
        locals.var_rmu_dn5 = assign2810_e2784_d_n5;

        let (assign2820_e2815, assign2820_e2815_d_n1, assign2820_e2815_d_n3, assign2820_e2815_d_n4, assign2820_e2815_d_n5,) = {
    if (locals.var_guard201 == 0.0) {
        let assign2820_e2789: f64 = (2.0 * locals.var_vrbi);
        let assign2820_e2791: f64 = (assign2820_e2789 * locals.var_vsat);
        let assign2820_e2794: f64 = (locals.var_vrbi - locals.var_vsat);
        let assign2820_e2797: f64 = (locals.var_vrbi - locals.var_vsat);
        let assign2820_e2798: f64 = (assign2820_e2794 * assign2820_e2797);
        let assign2820_e2800: f64 = (assign2820_e2798 + locals.var_atspo);
        let assign2820_e2801: f64 = (assign2820_e2800).sqrt();
        let assign2820_e2804: f64 = (locals.var_vrbi + locals.var_vsat);
        let assign2820_e2807: f64 = (locals.var_vrbi + locals.var_vsat);
        let assign2820_e2808: f64 = (assign2820_e2804 * assign2820_e2807);
        let assign2820_e2810: f64 = (assign2820_e2808 + locals.var_atspo);
        let assign2820_e2811: f64 = (assign2820_e2810).sqrt();
        let assign2820_e2812: f64 = (assign2820_e2801 + assign2820_e2811);
        let assign2820_e2813: f64 = (assign2820_e2791 / assign2820_e2812);
        (assign2820_e2813, ((((assign2820_e2789 * locals.var_vsat_dn1) * assign2820_e2812) - (assign2820_e2791 * (((((-locals.var_vsat_dn1) * assign2820_e2797) + (assign2820_e2794 * (-locals.var_vsat_dn1))) / (2.0 * assign2820_e2801)) + (((locals.var_vsat_dn1 * assign2820_e2807) + (assign2820_e2804 * locals.var_vsat_dn1)) / (2.0 * assign2820_e2811))))) / (assign2820_e2812 * assign2820_e2812)), ((((assign2820_e2789 * locals.var_vsat_dn3) * assign2820_e2812) - (assign2820_e2791 * ((((((-locals.var_vsat_dn3) * assign2820_e2797) + (assign2820_e2794 * (-locals.var_vsat_dn3))) + locals.var_atspo_dn3) / (2.0 * assign2820_e2801)) + ((((locals.var_vsat_dn3 * assign2820_e2807) + (assign2820_e2804 * locals.var_vsat_dn3)) + locals.var_atspo_dn3) / (2.0 * assign2820_e2811))))) / (assign2820_e2812 * assign2820_e2812)), ((((((2.0 * locals.var_vrbi_dn4) * locals.var_vsat) + (assign2820_e2789 * locals.var_vsat_dn4)) * assign2820_e2812) - (assign2820_e2791 * (((((locals.var_vrbi_dn4 - locals.var_vsat_dn4) * assign2820_e2797) + (assign2820_e2794 * (locals.var_vrbi_dn4 - locals.var_vsat_dn4))) / (2.0 * assign2820_e2801)) + ((((locals.var_vrbi_dn4 + locals.var_vsat_dn4) * assign2820_e2807) + (assign2820_e2804 * (locals.var_vrbi_dn4 + locals.var_vsat_dn4))) / (2.0 * assign2820_e2811))))) / (assign2820_e2812 * assign2820_e2812)), ((((((2.0 * locals.var_vrbi_dn5) * locals.var_vsat) + (assign2820_e2789 * locals.var_vsat_dn5)) * assign2820_e2812) - (assign2820_e2791 * (((((locals.var_vrbi_dn5 - locals.var_vsat_dn5) * assign2820_e2797) + (assign2820_e2794 * (locals.var_vrbi_dn5 - locals.var_vsat_dn5))) / (2.0 * assign2820_e2801)) + ((((locals.var_vrbi_dn5 + locals.var_vsat_dn5) * assign2820_e2807) + (assign2820_e2804 * (locals.var_vrbi_dn5 + locals.var_vsat_dn5))) / (2.0 * assign2820_e2811))))) / (assign2820_e2812 * assign2820_e2812)),)
    } else {
        (locals.var_vrbeff, locals.var_vrbeff_dn1, locals.var_vrbeff_dn3, locals.var_vrbeff_dn4, locals.var_vrbeff_dn5,)
    }
};
        locals.var_vrbeff = assign2820_e2815;
        locals.var_vrbeff_dn1 = assign2820_e2815_d_n1;
        locals.var_vrbeff_dn3 = assign2820_e2815_d_n3;
        locals.var_vrbeff_dn4 = assign2820_e2815_d_n4;
        locals.var_vrbeff_dn5 = assign2820_e2815_d_n5;

        let assign2830_e2818: f64 = if locals.var_iecrit > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard205 = assign2830_e2818;

        let (assign2840_e2833, assign2840_e2833_d_n1, assign2840_e2833_d_n3, assign2840_e2833_d_n4, assign2840_e2833_d_n5,) = {
    if ((locals.var_guard201 == 0.0) && (locals.var_guard205 != 0.0)) {
        let assign2840_e2826: f64 = (locals.var_vrbeff / locals.var_leffe_um);
        let assign2840_e2828: f64 = (assign2840_e2826 - locals.var_ecrneff);
        let assign2840_e2829: f64 = (0.5 * assign2840_e2828);
        let assign2840_e2831: f64 = (assign2840_e2829 * locals.var_iecrit);
        (assign2840_e2831, ((0.5 * (locals.var_vrbeff_dn1 / locals.var_leffe_um)) * locals.var_iecrit), (((0.5 * ((locals.var_vrbeff_dn3 / locals.var_leffe_um) - locals.var_ecrneff_dn3)) * locals.var_iecrit) + (assign2840_e2829 * locals.var_iecrit_dn3)), ((0.5 * (locals.var_vrbeff_dn4 / locals.var_leffe_um)) * locals.var_iecrit), ((0.5 * (locals.var_vrbeff_dn5 / locals.var_leffe_um)) * locals.var_iecrit),)
    } else {
        (locals.var_fctrm, locals.var_fctrm_dn1, locals.var_fctrm_dn3, locals.var_fctrm_dn4, locals.var_fctrm_dn5,)
    }
};
        locals.var_fctrm = assign2840_e2833;
        locals.var_fctrm_dn1 = assign2840_e2833_d_n1;
        locals.var_fctrm_dn3 = assign2840_e2833_d_n3;
        locals.var_fctrm_dn4 = assign2840_e2833_d_n4;
        locals.var_fctrm_dn5 = assign2840_e2833_d_n5;

        let (assign2850_e2848, assign2850_e2848_d_n1, assign2850_e2848_d_n3, assign2850_e2848_d_n4, assign2850_e2848_d_n5,) = {
    if ((locals.var_guard201 == 0.0) && (locals.var_guard205 != 0.0)) {
        let assign2850_e2841: f64 = (locals.var_vrbeff / locals.var_leffe_um);
        let assign2850_e2843: f64 = (assign2850_e2841 + locals.var_ecrneff);
        let assign2850_e2844: f64 = (0.5 * assign2850_e2843);
        let assign2850_e2846: f64 = (assign2850_e2844 * locals.var_iecrit);
        (assign2850_e2846, ((0.5 * (locals.var_vrbeff_dn1 / locals.var_leffe_um)) * locals.var_iecrit), (((0.5 * ((locals.var_vrbeff_dn3 / locals.var_leffe_um) + locals.var_ecrneff_dn3)) * locals.var_iecrit) + (assign2850_e2844 * locals.var_iecrit_dn3)), ((0.5 * (locals.var_vrbeff_dn4 / locals.var_leffe_um)) * locals.var_iecrit), ((0.5 * (locals.var_vrbeff_dn5 / locals.var_leffe_um)) * locals.var_iecrit),)
    } else {
        (locals.var_fctrp, locals.var_fctrp_dn1, locals.var_fctrp_dn3, locals.var_fctrp_dn4, locals.var_fctrp_dn5,)
    }
};
        locals.var_fctrp = assign2850_e2848;
        locals.var_fctrp_dn1 = assign2850_e2848_d_n1;
        locals.var_fctrp_dn3 = assign2850_e2848_d_n3;
        locals.var_fctrp_dn4 = assign2850_e2848_d_n4;
        locals.var_fctrp_dn5 = assign2850_e2848_d_n5;

        let (assign2860_e2860, assign2860_e2860_d_n1, assign2860_e2860_d_n3, assign2860_e2860_d_n4, assign2860_e2860_d_n5,) = {
    if ((locals.var_guard201 == 0.0) && (locals.var_guard205 != 0.0)) {
        let assign2860_e2855: f64 = (locals.var_fctrm * locals.var_fctrm);
        let assign2860_e2857: f64 = (assign2860_e2855 + locals.var_dufctr);
        let assign2860_e2858: f64 = (assign2860_e2857).sqrt();
        (assign2860_e2858, (((locals.var_fctrm_dn1 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn1)) / (2.0 * assign2860_e2858)), ((((locals.var_fctrm_dn3 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn3)) + locals.var_dufctr_dn3) / (2.0 * assign2860_e2858)), (((locals.var_fctrm_dn4 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn4)) / (2.0 * assign2860_e2858)), (((locals.var_fctrm_dn5 * locals.var_fctrm) + (locals.var_fctrm * locals.var_fctrm_dn5)) / (2.0 * assign2860_e2858)),)
    } else {
        (locals.var_sqrtm, locals.var_sqrtm_dn1, locals.var_sqrtm_dn3, locals.var_sqrtm_dn4, locals.var_sqrtm_dn5,)
    }
};
        locals.var_sqrtm = assign2860_e2860;
        locals.var_sqrtm_dn1 = assign2860_e2860_d_n1;
        locals.var_sqrtm_dn3 = assign2860_e2860_d_n3;
        locals.var_sqrtm_dn4 = assign2860_e2860_d_n4;
        locals.var_sqrtm_dn5 = assign2860_e2860_d_n5;

        let (assign2870_e2872, assign2870_e2872_d_n1, assign2870_e2872_d_n3, assign2870_e2872_d_n4, assign2870_e2872_d_n5,) = {
    if ((locals.var_guard201 == 0.0) && (locals.var_guard205 != 0.0)) {
        let assign2870_e2867: f64 = (locals.var_fctrp * locals.var_fctrp);
        let assign2870_e2869: f64 = (assign2870_e2867 + locals.var_dufctr);
        let assign2870_e2870: f64 = (assign2870_e2869).sqrt();
        (assign2870_e2870, (((locals.var_fctrp_dn1 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn1)) / (2.0 * assign2870_e2870)), ((((locals.var_fctrp_dn3 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn3)) + locals.var_dufctr_dn3) / (2.0 * assign2870_e2870)), (((locals.var_fctrp_dn4 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn4)) / (2.0 * assign2870_e2870)), (((locals.var_fctrp_dn5 * locals.var_fctrp) + (locals.var_fctrp * locals.var_fctrp_dn5)) / (2.0 * assign2870_e2870)),)
    } else {
        (locals.var_sqrtp, locals.var_sqrtp_dn1, locals.var_sqrtp_dn3, locals.var_sqrtp_dn4, locals.var_sqrtp_dn5,)
    }
};
        locals.var_sqrtp = assign2870_e2872;
        locals.var_sqrtp_dn1 = assign2870_e2872_d_n1;
        locals.var_sqrtp_dn3 = assign2870_e2872_d_n3;
        locals.var_sqrtp_dn4 = assign2870_e2872_d_n4;
        locals.var_sqrtp_dn5 = assign2870_e2872_d_n5;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2880_e2883, assign2880_e2883_d_n1, assign2880_e2883_d_n3, assign2880_e2883_d_n4, assign2880_e2883_d_n5,) = {
    if ((locals.var_guard201 == 0.0) && (locals.var_guard205 != 0.0)) {
        let assign2880_e2879: f64 = (locals.var_sqrtm + locals.var_sqrtp);
        let assign2880_e2881: f64 = (assign2880_e2879 - locals.var_uoff);
        (assign2880_e2881, (locals.var_sqrtm_dn1 + locals.var_sqrtp_dn1), ((locals.var_sqrtm_dn3 + locals.var_sqrtp_dn3) - locals.var_uoff_dn3), (locals.var_sqrtm_dn4 + locals.var_sqrtp_dn4), (locals.var_sqrtm_dn5 + locals.var_sqrtp_dn5),)
    } else {
        (locals.var_rmu, locals.var_rmu_dn1, locals.var_rmu_dn3, locals.var_rmu_dn4, locals.var_rmu_dn5,)
    }
};
        locals.var_rmu = assign2880_e2883;
        locals.var_rmu_dn1 = assign2880_e2883_d_n1;
        locals.var_rmu_dn3 = assign2880_e2883_d_n3;
        locals.var_rmu_dn4 = assign2880_e2883_d_n4;
        locals.var_rmu_dn5 = assign2880_e2883_d_n5;

        let (assign2890_e2891, assign2890_e2891_d_n1, assign2890_e2891_d_n3, assign2890_e2891_d_n4, assign2890_e2891_d_n5,) = {
    if ((locals.var_guard201 == 0.0) && (locals.var_guard205 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rmu, locals.var_rmu_dn1, locals.var_rmu_dn3, locals.var_rmu_dn4, locals.var_rmu_dn5,)
    }
};
        locals.var_rmu = assign2890_e2891;
        locals.var_rmu_dn1 = assign2890_e2891_d_n1;
        locals.var_rmu_dn3 = assign2890_e2891_d_n3;
        locals.var_rmu_dn4 = assign2890_e2891_d_n4;
        locals.var_rmu_dn5 = assign2890_e2891_d_n5;

        let (assign2900_e2903, assign2900_e2903_d_n1, assign2900_e2903_d_n3, assign2900_e2903_d_n4, assign2900_e2903_d_n5,) = {
    if (locals.var_guard201 == 0.0) {
        let assign2900_e2898: f64 = (locals.var_pe + locals.var_vrbeff);
        let assign2900_e2899: f64 = (assign2900_e2898).sqrt();
        let assign2900_e2900: f64 = (locals.var_df * assign2900_e2899);
        let assign2900_e2901: f64 = (1.0 - assign2900_e2900);
        (assign2900_e2901, (-(locals.var_df * ((locals.var_pe_dn1 + locals.var_vrbeff_dn1) / (2.0 * assign2900_e2899)))), (-(locals.var_df * ((locals.var_pe_dn3 + locals.var_vrbeff_dn3) / (2.0 * assign2900_e2899)))), (-(locals.var_df * ((locals.var_pe_dn4 + locals.var_vrbeff_dn4) / (2.0 * assign2900_e2899)))), (-(locals.var_df * ((locals.var_pe_dn5 + locals.var_vrbeff_dn5) / (2.0 * assign2900_e2899)))),)
    } else {
        (locals.var_dpfctr, locals.var_dpfctr_dn1, locals.var_dpfctr_dn3, locals.var_dpfctr_dn4, locals.var_dpfctr_dn5,)
    }
};
        locals.var_dpfctr = assign2900_e2903;
        locals.var_dpfctr_dn1 = assign2900_e2903_d_n1;
        locals.var_dpfctr_dn3 = assign2900_e2903_d_n3;
        locals.var_dpfctr_dn4 = assign2900_e2903_d_n4;
        locals.var_dpfctr_dn5 = assign2900_e2903_d_n5;

        let assign2910_e2906: f64 = if locals.var_dpfctr < p.p64 { 1.0 } else { 0.0 };
        locals.var_guard206 = assign2910_e2906;

        let (assign2920_e2910, assign2920_e2910_d_n1, assign2920_e2910_d_n3, assign2920_e2910_d_n4, assign2920_e2910_d_n5,) = {
    if (locals.var_guard206 != 0.0) {
        (p.p64, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dpfctr, locals.var_dpfctr_dn1, locals.var_dpfctr_dn3, locals.var_dpfctr_dn4, locals.var_dpfctr_dn5,)
    }
};
        locals.var_dpfctr = assign2920_e2910;
        locals.var_dpfctr_dn1 = assign2920_e2910_d_n1;
        locals.var_dpfctr_dn3 = assign2920_e2910_d_n3;
        locals.var_dpfctr_dn4 = assign2920_e2910_d_n4;
        locals.var_dpfctr_dn5 = assign2920_e2910_d_n5;

        let assign2930_e2913: f64 = (locals.var_gf * locals.var_dpfctr);
        let assign2930_e2916: f64 = (1.0 + locals.var_rmu);
        let assign2930_e2917: f64 = (assign2930_e2913 / assign2930_e2916);
        locals.var_geff = assign2930_e2917;
        locals.var_geff_dn1 = ((((locals.var_gf * locals.var_dpfctr_dn1) * assign2930_e2916) - (assign2930_e2913 * locals.var_rmu_dn1)) / (assign2930_e2916 * assign2930_e2916));
        locals.var_geff_dn3 = (((((locals.var_gf_dn3 * locals.var_dpfctr) + (locals.var_gf * locals.var_dpfctr_dn3)) * assign2930_e2916) - (assign2930_e2913 * locals.var_rmu_dn3)) / (assign2930_e2916 * assign2930_e2916));
        locals.var_geff_dn4 = ((((locals.var_gf * locals.var_dpfctr_dn4) * assign2930_e2916) - (assign2930_e2913 * locals.var_rmu_dn4)) / (assign2930_e2916 * assign2930_e2916));
        locals.var_geff_dn5 = ((((locals.var_gf * locals.var_dpfctr_dn5) * assign2930_e2916) - (assign2930_e2913 * locals.var_rmu_dn5)) / (assign2930_e2916 * assign2930_e2916));

        let assign2940_e2920: f64 = (locals.var_sdflip * locals.var_geff);
        let assign2940_e2922: f64 = (assign2940_e2920 * locals.var_vrbeff);
        locals.var_irb = assign2940_e2922;
        locals.var_irb_dn1 = (((locals.var_sdflip * locals.var_geff_dn1) * locals.var_vrbeff) + (assign2940_e2920 * locals.var_vrbeff_dn1));
        locals.var_irb_dn3 = (((locals.var_sdflip * locals.var_geff_dn3) * locals.var_vrbeff) + (assign2940_e2920 * locals.var_vrbeff_dn3));
        locals.var_irb_dn4 = (((locals.var_sdflip * locals.var_geff_dn4) * locals.var_vrbeff) + (assign2940_e2920 * locals.var_vrbeff_dn4));
        locals.var_irb_dn5 = (((locals.var_sdflip * locals.var_geff_dn5) * locals.var_vrbeff) + (assign2940_e2920 * locals.var_vrbeff_dn5));

        let assign2950_e2925: f64 = if locals.var_is1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard207 = assign2950_e2925;

        let (assign2960_e2931, assign2960_e2931_d_n1, assign2960_e2931_d_n3, assign2960_e2931_d_n4, assign2960_e2931_d_n5,) = {
    if (locals.var_guard207 != 0.0) {
        let assign2960_e2929: f64 = (locals.var_a1_um2 * locals.var_isa_t);
        (assign2960_e2929, (locals.var_a1_um2_dn1 * locals.var_isa_t), ((locals.var_a1_um2_dn3 * locals.var_isa_t) + (locals.var_a1_um2 * locals.var_isa_t_dn3)), (locals.var_a1_um2_dn4 * locals.var_isa_t), (locals.var_a1_um2_dn5 * locals.var_isa_t),)
    } else {
        (locals.var_aisa, locals.var_aisa_dn1, locals.var_aisa_dn3, locals.var_aisa_dn4, locals.var_aisa_dn5,)
    }
};
        locals.var_aisa = assign2960_e2931;
        locals.var_aisa_dn1 = assign2960_e2931_d_n1;
        locals.var_aisa_dn3 = assign2960_e2931_d_n3;
        locals.var_aisa_dn4 = assign2960_e2931_d_n4;
        locals.var_aisa_dn5 = assign2960_e2931_d_n5;

        let (assign2970_e2937, assign2970_e2937_d_n3,) = {
    if (locals.var_guard207 != 0.0) {
        let assign2970_e2935: f64 = (locals.var_p1_um * locals.var_isp_t);
        (assign2970_e2935, (locals.var_p1_um * locals.var_isp_t_dn3),)
    } else {
        (locals.var_pisp, locals.var_pisp_dn3,)
    }
};
        locals.var_pisp = assign2970_e2937;
        locals.var_pisp_dn3 = assign2970_e2937_d_n3;

        let assign2980_e2940: f64 = if locals.var_aisa > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard214 = assign2980_e2940;

        let (assign2990_e2950, assign2990_e2950_d_n3,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard214 != 0.0)) {
        let assign2990_e2947: f64 = (p.p70 * locals.var_phi_t);
        let assign2990_e2948: f64 = (1.0 / assign2990_e2947);
        (assign2990_e2948, (-((p.p70 * locals.var_phi_t_dn3) / (assign2990_e2947 * assign2990_e2947))),)
    } else {
        (locals.var_argx, locals.var_argx_dn3,)
    }
};
        locals.var_argx = assign2990_e2950;
        locals.var_argx_dn3 = assign2990_e2950_d_n3;

        let assign3000_e2953: f64 = if locals.var_vc1 < locals.var_vmax_a { 1.0 } else { 0.0 };
        locals.var_guard215 = assign3000_e2953;

        let (assign3010_e2964, assign3010_e2964_d_n1, assign3010_e2964_d_n3, assign3010_e2964_d_n4,) = {
    if (((locals.var_guard207 != 0.0) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 != 0.0)) {
        let assign3010_e2961: f64 = (locals.var_vc1 * locals.var_argx);
        let assign3010_e2962: f64 = (assign3010_e2961).exp();
        (assign3010_e2962, (assign3010_e2962 * (locals.var_vc1_dn1 * locals.var_argx)), (assign3010_e2962 * (locals.var_vc1 * locals.var_argx_dn3)), (assign3010_e2962 * (locals.var_vc1_dn4 * locals.var_argx)),)
    } else {
        (locals.var_expx, locals.var_expx_dn1, locals.var_expx_dn3, locals.var_expx_dn4,)
    }
};
        locals.var_expx = assign3010_e2964;
        locals.var_expx_dn1 = assign3010_e2964_d_n1;
        locals.var_expx_dn3 = assign3010_e2964_d_n3;
        locals.var_expx_dn4 = assign3010_e2964_d_n4;

        let (assign3020_e2984, assign3020_e2984_d_n1, assign3020_e2984_d_n3, assign3020_e2984_d_n4,) = {
    if (((locals.var_guard207 != 0.0) && (locals.var_guard214 != 0.0)) && (locals.var_guard215 == 0.0)) {
        let assign3020_e2973: f64 = (locals.var_vmax_a * locals.var_argx);
        let assign3020_e2974: f64 = (assign3020_e2973).exp();
        let assign3020_e2978: f64 = (locals.var_vc1 - locals.var_vmax_a);
        let assign3020_e2980: f64 = (assign3020_e2978 * locals.var_argx);
        let assign3020_e2981: f64 = (1.0 + assign3020_e2980);
        let assign3020_e2982: f64 = (assign3020_e2974 * assign3020_e2981);
        (assign3020_e2982, (assign3020_e2974 * (locals.var_vc1_dn1 * locals.var_argx)), (((assign3020_e2974 * ((locals.var_vmax_a_dn3 * locals.var_argx) + (locals.var_vmax_a * locals.var_argx_dn3))) * assign3020_e2981) + (assign3020_e2974 * (((-locals.var_vmax_a_dn3) * locals.var_argx) + (assign3020_e2978 * locals.var_argx_dn3)))), (assign3020_e2974 * (locals.var_vc1_dn4 * locals.var_argx)),)
    } else {
        (locals.var_expx, locals.var_expx_dn1, locals.var_expx_dn3, locals.var_expx_dn4,)
    }
};
        locals.var_expx = assign3020_e2984;
        locals.var_expx_dn1 = assign3020_e2984_d_n1;
        locals.var_expx_dn3 = assign3020_e2984_d_n3;
        locals.var_expx_dn4 = assign3020_e2984_d_n4;

        let (assign3030_e2994, assign3030_e2994_d_n1, assign3030_e2994_d_n3, assign3030_e2994_d_n4, assign3030_e2994_d_n5,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard214 != 0.0)) {
        let assign3030_e2991: f64 = (locals.var_expx - 1.0);
        let assign3030_e2992: f64 = (locals.var_aisa * assign3030_e2991);
        (assign3030_e2992, ((locals.var_aisa_dn1 * assign3030_e2991) + (locals.var_aisa * locals.var_expx_dn1)), ((locals.var_aisa_dn3 * assign3030_e2991) + (locals.var_aisa * locals.var_expx_dn3)), ((locals.var_aisa_dn4 * assign3030_e2991) + (locals.var_aisa * locals.var_expx_dn4)), (locals.var_aisa_dn5 * assign3030_e2991),)
    } else {
        (locals.var_pnjia, locals.var_pnjia_dn1, locals.var_pnjia_dn3, locals.var_pnjia_dn4, locals.var_pnjia_dn5,)
    }
};
        locals.var_pnjia = assign3030_e2994;
        locals.var_pnjia_dn1 = assign3030_e2994_d_n1;
        locals.var_pnjia_dn3 = assign3030_e2994_d_n3;
        locals.var_pnjia_dn4 = assign3030_e2994_d_n4;
        locals.var_pnjia_dn5 = assign3030_e2994_d_n5;

        let (assign3040_e3001, assign3040_e3001_d_n1, assign3040_e3001_d_n3, assign3040_e3001_d_n4, assign3040_e3001_d_n5,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard214 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pnjia, locals.var_pnjia_dn1, locals.var_pnjia_dn3, locals.var_pnjia_dn4, locals.var_pnjia_dn5,)
    }
};
        locals.var_pnjia = assign3040_e3001;
        locals.var_pnjia_dn1 = assign3040_e3001_d_n1;
        locals.var_pnjia_dn3 = assign3040_e3001_d_n3;
        locals.var_pnjia_dn4 = assign3040_e3001_d_n4;
        locals.var_pnjia_dn5 = assign3040_e3001_d_n5;

        let assign3050_e3004: f64 = if locals.var_pisp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard216 = assign3050_e3004;

        let (assign3060_e3014, assign3060_e3014_d_n3,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard216 != 0.0)) {
        let assign3060_e3011: f64 = (p.p77 * locals.var_phi_t);
        let assign3060_e3012: f64 = (1.0 / assign3060_e3011);
        (assign3060_e3012, (-((p.p77 * locals.var_phi_t_dn3) / (assign3060_e3011 * assign3060_e3011))),)
    } else {
        (locals.var_argx, locals.var_argx_dn3,)
    }
};
        locals.var_argx = assign3060_e3014;
        locals.var_argx_dn3 = assign3060_e3014_d_n3;

        let assign3070_e3017: f64 = if locals.var_vc1 < locals.var_vmax_p { 1.0 } else { 0.0 };
        locals.var_guard217 = assign3070_e3017;

        let (assign3080_e3028, assign3080_e3028_d_n1, assign3080_e3028_d_n3, assign3080_e3028_d_n4,) = {
    if (((locals.var_guard207 != 0.0) && (locals.var_guard216 != 0.0)) && (locals.var_guard217 != 0.0)) {
        let assign3080_e3025: f64 = (locals.var_vc1 * locals.var_argx);
        let assign3080_e3026: f64 = (assign3080_e3025).exp();
        (assign3080_e3026, (assign3080_e3026 * (locals.var_vc1_dn1 * locals.var_argx)), (assign3080_e3026 * (locals.var_vc1 * locals.var_argx_dn3)), (assign3080_e3026 * (locals.var_vc1_dn4 * locals.var_argx)),)
    } else {
        (locals.var_expx, locals.var_expx_dn1, locals.var_expx_dn3, locals.var_expx_dn4,)
    }
};
        locals.var_expx = assign3080_e3028;
        locals.var_expx_dn1 = assign3080_e3028_d_n1;
        locals.var_expx_dn3 = assign3080_e3028_d_n3;
        locals.var_expx_dn4 = assign3080_e3028_d_n4;

        let (assign3090_e3048, assign3090_e3048_d_n1, assign3090_e3048_d_n3, assign3090_e3048_d_n4,) = {
    if (((locals.var_guard207 != 0.0) && (locals.var_guard216 != 0.0)) && (locals.var_guard217 == 0.0)) {
        let assign3090_e3037: f64 = (locals.var_vmax_p * locals.var_argx);
        let assign3090_e3038: f64 = (assign3090_e3037).exp();
        let assign3090_e3042: f64 = (locals.var_vc1 - locals.var_vmax_p);
        let assign3090_e3044: f64 = (assign3090_e3042 * locals.var_argx);
        let assign3090_e3045: f64 = (1.0 + assign3090_e3044);
        let assign3090_e3046: f64 = (assign3090_e3038 * assign3090_e3045);
        (assign3090_e3046, (assign3090_e3038 * (locals.var_vc1_dn1 * locals.var_argx)), (((assign3090_e3038 * ((locals.var_vmax_p_dn3 * locals.var_argx) + (locals.var_vmax_p * locals.var_argx_dn3))) * assign3090_e3045) + (assign3090_e3038 * (((-locals.var_vmax_p_dn3) * locals.var_argx) + (assign3090_e3042 * locals.var_argx_dn3)))), (assign3090_e3038 * (locals.var_vc1_dn4 * locals.var_argx)),)
    } else {
        (locals.var_expx, locals.var_expx_dn1, locals.var_expx_dn3, locals.var_expx_dn4,)
    }
};
        locals.var_expx = assign3090_e3048;
        locals.var_expx_dn1 = assign3090_e3048_d_n1;
        locals.var_expx_dn3 = assign3090_e3048_d_n3;
        locals.var_expx_dn4 = assign3090_e3048_d_n4;

        let (assign3100_e3058, assign3100_e3058_d_n1, assign3100_e3058_d_n3, assign3100_e3058_d_n4,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard216 != 0.0)) {
        let assign3100_e3055: f64 = (locals.var_expx - 1.0);
        let assign3100_e3056: f64 = (locals.var_pisp * assign3100_e3055);
        (assign3100_e3056, (locals.var_pisp * locals.var_expx_dn1), ((locals.var_pisp_dn3 * assign3100_e3055) + (locals.var_pisp * locals.var_expx_dn3)), (locals.var_pisp * locals.var_expx_dn4),)
    } else {
        (locals.var_pnjip, locals.var_pnjip_dn1, locals.var_pnjip_dn3, locals.var_pnjip_dn4,)
    }
};
        locals.var_pnjip = assign3100_e3058;
        locals.var_pnjip_dn1 = assign3100_e3058_d_n1;
        locals.var_pnjip_dn3 = assign3100_e3058_d_n3;
        locals.var_pnjip_dn4 = assign3100_e3058_d_n4;

        let (assign3110_e3065, assign3110_e3065_d_n1, assign3110_e3065_d_n3, assign3110_e3065_d_n4,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard216 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pnjip, locals.var_pnjip_dn1, locals.var_pnjip_dn3, locals.var_pnjip_dn4,)
    }
};
        locals.var_pnjip = assign3110_e3065;
        locals.var_pnjip_dn1 = assign3110_e3065_d_n1;
        locals.var_pnjip_dn3 = assign3110_e3065_d_n3;
        locals.var_pnjip_dn4 = assign3110_e3065_d_n4;

        let (assign3120_e3071, assign3120_e3071_d_n1, assign3120_e3071_d_n3, assign3120_e3071_d_n4, assign3120_e3071_d_n5,) = {
    if (locals.var_guard207 != 0.0) {
        let assign3120_e3069: f64 = (locals.var_pnjia + locals.var_pnjip);
        (assign3120_e3069, (locals.var_pnjia_dn1 + locals.var_pnjip_dn1), (locals.var_pnjia_dn3 + locals.var_pnjip_dn3), (locals.var_pnjia_dn4 + locals.var_pnjip_dn4), locals.var_pnjia_dn5,)
    } else {
        (locals.var_id1, locals.var_id1_dn1, locals.var_id1_dn3, locals.var_id1_dn4, locals.var_id1_dn5,)
    }
};
        locals.var_id1 = assign3120_e3071;
        locals.var_id1_dn1 = assign3120_e3071_d_n1;
        locals.var_id1_dn3 = assign3120_e3071_d_n3;
        locals.var_id1_dn4 = assign3120_e3071_d_n4;
        locals.var_id1_dn5 = assign3120_e3071_d_n5;

        let assign3130_e3074: f64 = if locals.var_vbv_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard221 = assign3130_e3074;

        let (assign3140_e3083, assign3140_e3083_d_n1, assign3140_e3083_d_n3, assign3140_e3083_d_n4,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard221 != 0.0)) {
        let assign3140_e3079: f64 = (-locals.var_vbv_t);
        let assign3140_e3081: f64 = (assign3140_e3079 - locals.var_vc1);
        (assign3140_e3081, (-locals.var_vc1_dn1), (-locals.var_vbv_t_dn3), (-locals.var_vc1_dn4),)
    } else {
        (locals.var_vbkd, locals.var_vbkd_dn1, locals.var_vbkd_dn3, locals.var_vbkd_dn4,)
    }
};
        locals.var_vbkd = assign3140_e3083;
        locals.var_vbkd_dn1 = assign3140_e3083_d_n1;
        locals.var_vbkd_dn3 = assign3140_e3083_d_n3;
        locals.var_vbkd_dn4 = assign3140_e3083_d_n4;

        let (assign3150_e3093, assign3150_e3093_d_n3,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard221 != 0.0)) {
        let assign3150_e3090: f64 = (locals.var_nbv_t * locals.var_phi_t);
        let assign3150_e3091: f64 = (1.0 / assign3150_e3090);
        (assign3150_e3091, (-(((locals.var_nbv_t_dn3 * locals.var_phi_t) + (locals.var_nbv_t * locals.var_phi_t_dn3)) / (assign3150_e3090 * assign3150_e3090))),)
    } else {
        (locals.var_argx__blk219, locals.var_argx__blk219_dn3,)
    }
};
        locals.var_argx__blk219 = assign3150_e3093;
        locals.var_argx__blk219_dn3 = assign3150_e3093_d_n3;

        let assign3160_e3096: f64 = if locals.var_vbkd < locals.var_vmax_b { 1.0 } else { 0.0 };
        locals.var_guard222 = assign3160_e3096;

        let (assign3170_e3107, assign3170_e3107_d_n1, assign3170_e3107_d_n3, assign3170_e3107_d_n4,) = {
    if (((locals.var_guard207 != 0.0) && (locals.var_guard221 != 0.0)) && (locals.var_guard222 != 0.0)) {
        let assign3170_e3104: f64 = (locals.var_vbkd * locals.var_argx__blk219);
        let assign3170_e3105: f64 = (assign3170_e3104).exp();
        (assign3170_e3105, (assign3170_e3105 * (locals.var_vbkd_dn1 * locals.var_argx__blk219)), (assign3170_e3105 * ((locals.var_vbkd_dn3 * locals.var_argx__blk219) + (locals.var_vbkd * locals.var_argx__blk219_dn3))), (assign3170_e3105 * (locals.var_vbkd_dn4 * locals.var_argx__blk219)),)
    } else {
        (locals.var_expx__blk220, locals.var_expx__blk220_dn1, locals.var_expx__blk220_dn3, locals.var_expx__blk220_dn4,)
    }
};
        locals.var_expx__blk220 = assign3170_e3107;
        locals.var_expx__blk220_dn1 = assign3170_e3107_d_n1;
        locals.var_expx__blk220_dn3 = assign3170_e3107_d_n3;
        locals.var_expx__blk220_dn4 = assign3170_e3107_d_n4;

        let (assign3180_e3127, assign3180_e3127_d_n1, assign3180_e3127_d_n3, assign3180_e3127_d_n4,) = {
    if (((locals.var_guard207 != 0.0) && (locals.var_guard221 != 0.0)) && (locals.var_guard222 == 0.0)) {
        let assign3180_e3116: f64 = (locals.var_vmax_b * locals.var_argx__blk219);
        let assign3180_e3117: f64 = (assign3180_e3116).exp();
        let assign3180_e3121: f64 = (locals.var_vbkd - locals.var_vmax_b);
        let assign3180_e3123: f64 = (assign3180_e3121 * locals.var_argx__blk219);
        let assign3180_e3124: f64 = (1.0 + assign3180_e3123);
        let assign3180_e3125: f64 = (assign3180_e3117 * assign3180_e3124);
        (assign3180_e3125, (assign3180_e3117 * (locals.var_vbkd_dn1 * locals.var_argx__blk219)), (((assign3180_e3117 * ((locals.var_vmax_b_dn3 * locals.var_argx__blk219) + (locals.var_vmax_b * locals.var_argx__blk219_dn3))) * assign3180_e3124) + (assign3180_e3117 * (((locals.var_vbkd_dn3 - locals.var_vmax_b_dn3) * locals.var_argx__blk219) + (assign3180_e3121 * locals.var_argx__blk219_dn3)))), (assign3180_e3117 * (locals.var_vbkd_dn4 * locals.var_argx__blk219)),)
    } else {
        (locals.var_expx__blk220, locals.var_expx__blk220_dn1, locals.var_expx__blk220_dn3, locals.var_expx__blk220_dn4,)
    }
};
        locals.var_expx__blk220 = assign3180_e3127;
        locals.var_expx__blk220_dn1 = assign3180_e3127_d_n1;
        locals.var_expx__blk220_dn3 = assign3180_e3127_d_n3;
        locals.var_expx__blk220_dn4 = assign3180_e3127_d_n4;

        let (assign3190_e3142, assign3190_e3142_d_n1, assign3190_e3142_d_n3, assign3190_e3142_d_n4,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard221 != 0.0)) {
        let assign3190_e3132: f64 = (-p.p84);
        let assign3190_e3135: f64 = (-locals.var_vbv_t);
        let assign3190_e3137: f64 = (assign3190_e3135 * locals.var_argx__blk219);
        let assign3190_e3138: f64 = (assign3190_e3137).exp();
        let assign3190_e3139: f64 = (locals.var_expx__blk220 - assign3190_e3138);
        let assign3190_e3140: f64 = (assign3190_e3132 * assign3190_e3139);
        (assign3190_e3140, (assign3190_e3132 * locals.var_expx__blk220_dn1), (assign3190_e3132 * (locals.var_expx__blk220_dn3 - (assign3190_e3138 * (((-locals.var_vbv_t_dn3) * locals.var_argx__blk219) + (assign3190_e3135 * locals.var_argx__blk219_dn3))))), (assign3190_e3132 * locals.var_expx__blk220_dn4),)
    } else {
        (locals.var_ib1, locals.var_ib1_dn1, locals.var_ib1_dn3, locals.var_ib1_dn4,)
    }
};
        locals.var_ib1 = assign3190_e3142;
        locals.var_ib1_dn1 = assign3190_e3142_d_n1;
        locals.var_ib1_dn3 = assign3190_e3142_d_n3;
        locals.var_ib1_dn4 = assign3190_e3142_d_n4;

        let (assign3200_e3149, assign3200_e3149_d_n1, assign3200_e3149_d_n3, assign3200_e3149_d_n4,) = {
    if ((locals.var_guard207 != 0.0) && (locals.var_guard221 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ib1, locals.var_ib1_dn1, locals.var_ib1_dn3, locals.var_ib1_dn4,)
    }
};
        locals.var_ib1 = assign3200_e3149;
        locals.var_ib1_dn1 = assign3200_e3149_d_n1;
        locals.var_ib1_dn3 = assign3200_e3149_d_n3;
        locals.var_ib1_dn4 = assign3200_e3149_d_n4;

        let (assign3210_e3159, assign3210_e3159_d_n1, assign3210_e3159_d_n3, assign3210_e3159_d_n4, assign3210_e3159_d_n5,) = {
    if (locals.var_guard207 != 0.0) {
        let assign3210_e3153: f64 = (locals.var_id1 + locals.var_ib1);
        let assign3210_e3156: f64 = (locals.var_gmin * locals.var_vc1);
        let assign3210_e3157: f64 = (assign3210_e3153 + assign3210_e3156);
        (assign3210_e3157, ((locals.var_id1_dn1 + locals.var_ib1_dn1) + (locals.var_gmin * locals.var_vc1_dn1)), (locals.var_id1_dn3 + locals.var_ib1_dn3), ((locals.var_id1_dn4 + locals.var_ib1_dn4) + (locals.var_gmin * locals.var_vc1_dn4)), locals.var_id1_dn5,)
    } else {
        (locals.var_ip1, locals.var_ip1_dn1, locals.var_ip1_dn3, locals.var_ip1_dn4, locals.var_ip1_dn5,)
    }
};
        locals.var_ip1 = assign3210_e3159;
        locals.var_ip1_dn1 = assign3210_e3159_d_n1;
        locals.var_ip1_dn3 = assign3210_e3159_d_n3;
        locals.var_ip1_dn4 = assign3210_e3159_d_n4;
        locals.var_ip1_dn5 = assign3210_e3159_d_n5;

        let (assign3220_e3164, assign3220_e3164_d_n1, assign3220_e3164_d_n3, assign3220_e3164_d_n4, assign3220_e3164_d_n5,) = {
    if (locals.var_guard207 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_id1, locals.var_id1_dn1, locals.var_id1_dn3, locals.var_id1_dn4, locals.var_id1_dn5,)
    }
};
        locals.var_id1 = assign3220_e3164;
        locals.var_id1_dn1 = assign3220_e3164_d_n1;
        locals.var_id1_dn3 = assign3220_e3164_d_n3;
        locals.var_id1_dn4 = assign3220_e3164_d_n4;
        locals.var_id1_dn5 = assign3220_e3164_d_n5;

        let (assign3230_e3169, assign3230_e3169_d_n1, assign3230_e3169_d_n3, assign3230_e3169_d_n4,) = {
    if (locals.var_guard207 == 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ib1, locals.var_ib1_dn1, locals.var_ib1_dn3, locals.var_ib1_dn4,)
    }
};
        locals.var_ib1 = assign3230_e3169;
        locals.var_ib1_dn1 = assign3230_e3169_d_n1;
        locals.var_ib1_dn3 = assign3230_e3169_d_n3;
        locals.var_ib1_dn4 = assign3230_e3169_d_n4;

        let (assign3240_e3174, assign3240_e3174_d_n1, assign3240_e3174_d_n3, assign3240_e3174_d_n4, assign3240_e3174_d_n5,) = {
    if (locals.var_guard207 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ip1, locals.var_ip1_dn1, locals.var_ip1_dn3, locals.var_ip1_dn4, locals.var_ip1_dn5,)
    }
};
        locals.var_ip1 = assign3240_e3174;
        locals.var_ip1_dn1 = assign3240_e3174_d_n1;
        locals.var_ip1_dn3 = assign3240_e3174_d_n3;
        locals.var_ip1_dn4 = assign3240_e3174_d_n4;
        locals.var_ip1_dn5 = assign3240_e3174_d_n5;

        let assign3250_e3177: f64 = if locals.var_is2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard223 = assign3250_e3177;

        let (assign3260_e3183, assign3260_e3183_d_n1, assign3260_e3183_d_n3, assign3260_e3183_d_n4, assign3260_e3183_d_n5,) = {
    if (locals.var_guard223 != 0.0) {
        let assign3260_e3181: f64 = (locals.var_a2_um2 * locals.var_isa_t);
        (assign3260_e3181, (locals.var_a2_um2_dn1 * locals.var_isa_t), ((locals.var_a2_um2_dn3 * locals.var_isa_t) + (locals.var_a2_um2 * locals.var_isa_t_dn3)), (locals.var_a2_um2_dn4 * locals.var_isa_t), (locals.var_a2_um2_dn5 * locals.var_isa_t),)
    } else {
        (locals.var_aisa__blk224, locals.var_aisa__blk224_dn1, locals.var_aisa__blk224_dn3, locals.var_aisa__blk224_dn4, locals.var_aisa__blk224_dn5,)
    }
};
        locals.var_aisa__blk224 = assign3260_e3183;
        locals.var_aisa__blk224_dn1 = assign3260_e3183_d_n1;
        locals.var_aisa__blk224_dn3 = assign3260_e3183_d_n3;
        locals.var_aisa__blk224_dn4 = assign3260_e3183_d_n4;
        locals.var_aisa__blk224_dn5 = assign3260_e3183_d_n5;

        let (assign3270_e3189, assign3270_e3189_d_n3,) = {
    if (locals.var_guard223 != 0.0) {
        let assign3270_e3187: f64 = (locals.var_p2_um * locals.var_isp_t);
        (assign3270_e3187, (locals.var_p2_um * locals.var_isp_t_dn3),)
    } else {
        (locals.var_pisp__blk225, locals.var_pisp__blk225_dn3,)
    }
};
        locals.var_pisp__blk225 = assign3270_e3189;
        locals.var_pisp__blk225_dn3 = assign3270_e3189_d_n3;

        let assign3280_e3192: f64 = if locals.var_aisa__blk224 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard230 = assign3280_e3192;

        let (assign3290_e3202, assign3290_e3202_d_n3,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard230 != 0.0)) {
        let assign3290_e3199: f64 = (p.p70 * locals.var_phi_t);
        let assign3290_e3200: f64 = (1.0 / assign3290_e3199);
        (assign3290_e3200, (-((p.p70 * locals.var_phi_t_dn3) / (assign3290_e3199 * assign3290_e3199))),)
    } else {
        (locals.var_argx__blk226, locals.var_argx__blk226_dn3,)
    }
};
        locals.var_argx__blk226 = assign3290_e3202;
        locals.var_argx__blk226_dn3 = assign3290_e3202_d_n3;

        let assign3300_e3205: f64 = if locals.var_vc2 < locals.var_vmax_a { 1.0 } else { 0.0 };
        locals.var_guard231 = assign3300_e3205;

        let (assign3310_e3216, assign3310_e3216_d_n1, assign3310_e3216_d_n3, assign3310_e3216_d_n5,) = {
    if (((locals.var_guard223 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 != 0.0)) {
        let assign3310_e3213: f64 = (locals.var_vc2 * locals.var_argx__blk226);
        let assign3310_e3214: f64 = (assign3310_e3213).exp();
        (assign3310_e3214, (assign3310_e3214 * (locals.var_vc2_dn1 * locals.var_argx__blk226)), (assign3310_e3214 * (locals.var_vc2 * locals.var_argx__blk226_dn3)), (assign3310_e3214 * (locals.var_vc2_dn5 * locals.var_argx__blk226)),)
    } else {
        (locals.var_expx__blk227, locals.var_expx__blk227_dn1, locals.var_expx__blk227_dn3, locals.var_expx__blk227_dn5,)
    }
};
        locals.var_expx__blk227 = assign3310_e3216;
        locals.var_expx__blk227_dn1 = assign3310_e3216_d_n1;
        locals.var_expx__blk227_dn3 = assign3310_e3216_d_n3;
        locals.var_expx__blk227_dn5 = assign3310_e3216_d_n5;

        let (assign3320_e3236, assign3320_e3236_d_n1, assign3320_e3236_d_n3, assign3320_e3236_d_n5,) = {
    if (((locals.var_guard223 != 0.0) && (locals.var_guard230 != 0.0)) && (locals.var_guard231 == 0.0)) {
        let assign3320_e3225: f64 = (locals.var_vmax_a * locals.var_argx__blk226);
        let assign3320_e3226: f64 = (assign3320_e3225).exp();
        let assign3320_e3230: f64 = (locals.var_vc2 - locals.var_vmax_a);
        let assign3320_e3232: f64 = (assign3320_e3230 * locals.var_argx__blk226);
        let assign3320_e3233: f64 = (1.0 + assign3320_e3232);
        let assign3320_e3234: f64 = (assign3320_e3226 * assign3320_e3233);
        (assign3320_e3234, (assign3320_e3226 * (locals.var_vc2_dn1 * locals.var_argx__blk226)), (((assign3320_e3226 * ((locals.var_vmax_a_dn3 * locals.var_argx__blk226) + (locals.var_vmax_a * locals.var_argx__blk226_dn3))) * assign3320_e3233) + (assign3320_e3226 * (((-locals.var_vmax_a_dn3) * locals.var_argx__blk226) + (assign3320_e3230 * locals.var_argx__blk226_dn3)))), (assign3320_e3226 * (locals.var_vc2_dn5 * locals.var_argx__blk226)),)
    } else {
        (locals.var_expx__blk227, locals.var_expx__blk227_dn1, locals.var_expx__blk227_dn3, locals.var_expx__blk227_dn5,)
    }
};
        locals.var_expx__blk227 = assign3320_e3236;
        locals.var_expx__blk227_dn1 = assign3320_e3236_d_n1;
        locals.var_expx__blk227_dn3 = assign3320_e3236_d_n3;
        locals.var_expx__blk227_dn5 = assign3320_e3236_d_n5;

    }

    pub(super) fn stamp_transient_block_7(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let bi0 = ctx.branch_current(branches[0]);
        let bi1 = ctx.branch_current(branches[1]);
        let (assign3330_e3246, assign3330_e3246_d_n1, assign3330_e3246_d_n3, assign3330_e3246_d_n4, assign3330_e3246_d_n5,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard230 != 0.0)) {
        let assign3330_e3243: f64 = (locals.var_expx__blk227 - 1.0);
        let assign3330_e3244: f64 = (locals.var_aisa__blk224 * assign3330_e3243);
        (assign3330_e3244, ((locals.var_aisa__blk224_dn1 * assign3330_e3243) + (locals.var_aisa__blk224 * locals.var_expx__blk227_dn1)), ((locals.var_aisa__blk224_dn3 * assign3330_e3243) + (locals.var_aisa__blk224 * locals.var_expx__blk227_dn3)), (locals.var_aisa__blk224_dn4 * assign3330_e3243), ((locals.var_aisa__blk224_dn5 * assign3330_e3243) + (locals.var_aisa__blk224 * locals.var_expx__blk227_dn5)),)
    } else {
        (locals.var_pnjia__blk228, locals.var_pnjia__blk228_dn1, locals.var_pnjia__blk228_dn3, locals.var_pnjia__blk228_dn4, locals.var_pnjia__blk228_dn5,)
    }
};
        locals.var_pnjia__blk228 = assign3330_e3246;
        locals.var_pnjia__blk228_dn1 = assign3330_e3246_d_n1;
        locals.var_pnjia__blk228_dn3 = assign3330_e3246_d_n3;
        locals.var_pnjia__blk228_dn4 = assign3330_e3246_d_n4;
        locals.var_pnjia__blk228_dn5 = assign3330_e3246_d_n5;

        let (assign3340_e3253, assign3340_e3253_d_n1, assign3340_e3253_d_n3, assign3340_e3253_d_n4, assign3340_e3253_d_n5,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard230 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pnjia__blk228, locals.var_pnjia__blk228_dn1, locals.var_pnjia__blk228_dn3, locals.var_pnjia__blk228_dn4, locals.var_pnjia__blk228_dn5,)
    }
};
        locals.var_pnjia__blk228 = assign3340_e3253;
        locals.var_pnjia__blk228_dn1 = assign3340_e3253_d_n1;
        locals.var_pnjia__blk228_dn3 = assign3340_e3253_d_n3;
        locals.var_pnjia__blk228_dn4 = assign3340_e3253_d_n4;
        locals.var_pnjia__blk228_dn5 = assign3340_e3253_d_n5;

        let assign3350_e3256: f64 = if locals.var_pisp__blk225 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard232 = assign3350_e3256;

        let (assign3360_e3266, assign3360_e3266_d_n3,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard232 != 0.0)) {
        let assign3360_e3263: f64 = (p.p77 * locals.var_phi_t);
        let assign3360_e3264: f64 = (1.0 / assign3360_e3263);
        (assign3360_e3264, (-((p.p77 * locals.var_phi_t_dn3) / (assign3360_e3263 * assign3360_e3263))),)
    } else {
        (locals.var_argx__blk226, locals.var_argx__blk226_dn3,)
    }
};
        locals.var_argx__blk226 = assign3360_e3266;
        locals.var_argx__blk226_dn3 = assign3360_e3266_d_n3;

        let assign3370_e3269: f64 = if locals.var_vc2 < locals.var_vmax_p { 1.0 } else { 0.0 };
        locals.var_guard233 = assign3370_e3269;

        let (assign3380_e3280, assign3380_e3280_d_n1, assign3380_e3280_d_n3, assign3380_e3280_d_n5,) = {
    if (((locals.var_guard223 != 0.0) && (locals.var_guard232 != 0.0)) && (locals.var_guard233 != 0.0)) {
        let assign3380_e3277: f64 = (locals.var_vc2 * locals.var_argx__blk226);
        let assign3380_e3278: f64 = (assign3380_e3277).exp();
        (assign3380_e3278, (assign3380_e3278 * (locals.var_vc2_dn1 * locals.var_argx__blk226)), (assign3380_e3278 * (locals.var_vc2 * locals.var_argx__blk226_dn3)), (assign3380_e3278 * (locals.var_vc2_dn5 * locals.var_argx__blk226)),)
    } else {
        (locals.var_expx__blk227, locals.var_expx__blk227_dn1, locals.var_expx__blk227_dn3, locals.var_expx__blk227_dn5,)
    }
};
        locals.var_expx__blk227 = assign3380_e3280;
        locals.var_expx__blk227_dn1 = assign3380_e3280_d_n1;
        locals.var_expx__blk227_dn3 = assign3380_e3280_d_n3;
        locals.var_expx__blk227_dn5 = assign3380_e3280_d_n5;

        let (assign3390_e3300, assign3390_e3300_d_n1, assign3390_e3300_d_n3, assign3390_e3300_d_n5,) = {
    if (((locals.var_guard223 != 0.0) && (locals.var_guard232 != 0.0)) && (locals.var_guard233 == 0.0)) {
        let assign3390_e3289: f64 = (locals.var_vmax_p * locals.var_argx__blk226);
        let assign3390_e3290: f64 = (assign3390_e3289).exp();
        let assign3390_e3294: f64 = (locals.var_vc2 - locals.var_vmax_p);
        let assign3390_e3296: f64 = (assign3390_e3294 * locals.var_argx__blk226);
        let assign3390_e3297: f64 = (1.0 + assign3390_e3296);
        let assign3390_e3298: f64 = (assign3390_e3290 * assign3390_e3297);
        (assign3390_e3298, (assign3390_e3290 * (locals.var_vc2_dn1 * locals.var_argx__blk226)), (((assign3390_e3290 * ((locals.var_vmax_p_dn3 * locals.var_argx__blk226) + (locals.var_vmax_p * locals.var_argx__blk226_dn3))) * assign3390_e3297) + (assign3390_e3290 * (((-locals.var_vmax_p_dn3) * locals.var_argx__blk226) + (assign3390_e3294 * locals.var_argx__blk226_dn3)))), (assign3390_e3290 * (locals.var_vc2_dn5 * locals.var_argx__blk226)),)
    } else {
        (locals.var_expx__blk227, locals.var_expx__blk227_dn1, locals.var_expx__blk227_dn3, locals.var_expx__blk227_dn5,)
    }
};
        locals.var_expx__blk227 = assign3390_e3300;
        locals.var_expx__blk227_dn1 = assign3390_e3300_d_n1;
        locals.var_expx__blk227_dn3 = assign3390_e3300_d_n3;
        locals.var_expx__blk227_dn5 = assign3390_e3300_d_n5;

        let (assign3400_e3310, assign3400_e3310_d_n1, assign3400_e3310_d_n3, assign3400_e3310_d_n5,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard232 != 0.0)) {
        let assign3400_e3307: f64 = (locals.var_expx__blk227 - 1.0);
        let assign3400_e3308: f64 = (locals.var_pisp__blk225 * assign3400_e3307);
        (assign3400_e3308, (locals.var_pisp__blk225 * locals.var_expx__blk227_dn1), ((locals.var_pisp__blk225_dn3 * assign3400_e3307) + (locals.var_pisp__blk225 * locals.var_expx__blk227_dn3)), (locals.var_pisp__blk225 * locals.var_expx__blk227_dn5),)
    } else {
        (locals.var_pnjip__blk229, locals.var_pnjip__blk229_dn1, locals.var_pnjip__blk229_dn3, locals.var_pnjip__blk229_dn5,)
    }
};
        locals.var_pnjip__blk229 = assign3400_e3310;
        locals.var_pnjip__blk229_dn1 = assign3400_e3310_d_n1;
        locals.var_pnjip__blk229_dn3 = assign3400_e3310_d_n3;
        locals.var_pnjip__blk229_dn5 = assign3400_e3310_d_n5;

        let (assign3410_e3317, assign3410_e3317_d_n1, assign3410_e3317_d_n3, assign3410_e3317_d_n5,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard232 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pnjip__blk229, locals.var_pnjip__blk229_dn1, locals.var_pnjip__blk229_dn3, locals.var_pnjip__blk229_dn5,)
    }
};
        locals.var_pnjip__blk229 = assign3410_e3317;
        locals.var_pnjip__blk229_dn1 = assign3410_e3317_d_n1;
        locals.var_pnjip__blk229_dn3 = assign3410_e3317_d_n3;
        locals.var_pnjip__blk229_dn5 = assign3410_e3317_d_n5;

        let (assign3420_e3323, assign3420_e3323_d_n1, assign3420_e3323_d_n3, assign3420_e3323_d_n4, assign3420_e3323_d_n5,) = {
    if (locals.var_guard223 != 0.0) {
        let assign3420_e3321: f64 = (locals.var_pnjia__blk228 + locals.var_pnjip__blk229);
        (assign3420_e3321, (locals.var_pnjia__blk228_dn1 + locals.var_pnjip__blk229_dn1), (locals.var_pnjia__blk228_dn3 + locals.var_pnjip__blk229_dn3), locals.var_pnjia__blk228_dn4, (locals.var_pnjia__blk228_dn5 + locals.var_pnjip__blk229_dn5),)
    } else {
        (locals.var_id2, locals.var_id2_dn1, locals.var_id2_dn3, locals.var_id2_dn4, locals.var_id2_dn5,)
    }
};
        locals.var_id2 = assign3420_e3323;
        locals.var_id2_dn1 = assign3420_e3323_d_n1;
        locals.var_id2_dn3 = assign3420_e3323_d_n3;
        locals.var_id2_dn4 = assign3420_e3323_d_n4;
        locals.var_id2_dn5 = assign3420_e3323_d_n5;

        let assign3430_e3326: f64 = if locals.var_vbv_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard237 = assign3430_e3326;

        let (assign3440_e3335, assign3440_e3335_d_n1, assign3440_e3335_d_n3, assign3440_e3335_d_n5,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard237 != 0.0)) {
        let assign3440_e3331: f64 = (-locals.var_vbv_t);
        let assign3440_e3333: f64 = (assign3440_e3331 - locals.var_vc2);
        (assign3440_e3333, (-locals.var_vc2_dn1), (-locals.var_vbv_t_dn3), (-locals.var_vc2_dn5),)
    } else {
        (locals.var_vbkd__blk234, locals.var_vbkd__blk234_dn1, locals.var_vbkd__blk234_dn3, locals.var_vbkd__blk234_dn5,)
    }
};
        locals.var_vbkd__blk234 = assign3440_e3335;
        locals.var_vbkd__blk234_dn1 = assign3440_e3335_d_n1;
        locals.var_vbkd__blk234_dn3 = assign3440_e3335_d_n3;
        locals.var_vbkd__blk234_dn5 = assign3440_e3335_d_n5;

        let (assign3450_e3345, assign3450_e3345_d_n3,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard237 != 0.0)) {
        let assign3450_e3342: f64 = (locals.var_nbv_t * locals.var_phi_t);
        let assign3450_e3343: f64 = (1.0 / assign3450_e3342);
        (assign3450_e3343, (-(((locals.var_nbv_t_dn3 * locals.var_phi_t) + (locals.var_nbv_t * locals.var_phi_t_dn3)) / (assign3450_e3342 * assign3450_e3342))),)
    } else {
        (locals.var_argx__blk235, locals.var_argx__blk235_dn3,)
    }
};
        locals.var_argx__blk235 = assign3450_e3345;
        locals.var_argx__blk235_dn3 = assign3450_e3345_d_n3;

        let assign3460_e3348: f64 = if locals.var_vbkd__blk234 < locals.var_vmax_b { 1.0 } else { 0.0 };
        locals.var_guard238 = assign3460_e3348;

        let (assign3470_e3359, assign3470_e3359_d_n1, assign3470_e3359_d_n3, assign3470_e3359_d_n5,) = {
    if (((locals.var_guard223 != 0.0) && (locals.var_guard237 != 0.0)) && (locals.var_guard238 != 0.0)) {
        let assign3470_e3356: f64 = (locals.var_vbkd__blk234 * locals.var_argx__blk235);
        let assign3470_e3357: f64 = (assign3470_e3356).exp();
        (assign3470_e3357, (assign3470_e3357 * (locals.var_vbkd__blk234_dn1 * locals.var_argx__blk235)), (assign3470_e3357 * ((locals.var_vbkd__blk234_dn3 * locals.var_argx__blk235) + (locals.var_vbkd__blk234 * locals.var_argx__blk235_dn3))), (assign3470_e3357 * (locals.var_vbkd__blk234_dn5 * locals.var_argx__blk235)),)
    } else {
        (locals.var_expx__blk236, locals.var_expx__blk236_dn1, locals.var_expx__blk236_dn3, locals.var_expx__blk236_dn5,)
    }
};
        locals.var_expx__blk236 = assign3470_e3359;
        locals.var_expx__blk236_dn1 = assign3470_e3359_d_n1;
        locals.var_expx__blk236_dn3 = assign3470_e3359_d_n3;
        locals.var_expx__blk236_dn5 = assign3470_e3359_d_n5;

        let (assign3480_e3379, assign3480_e3379_d_n1, assign3480_e3379_d_n3, assign3480_e3379_d_n5,) = {
    if (((locals.var_guard223 != 0.0) && (locals.var_guard237 != 0.0)) && (locals.var_guard238 == 0.0)) {
        let assign3480_e3368: f64 = (locals.var_vmax_b * locals.var_argx__blk235);
        let assign3480_e3369: f64 = (assign3480_e3368).exp();
        let assign3480_e3373: f64 = (locals.var_vbkd__blk234 - locals.var_vmax_b);
        let assign3480_e3375: f64 = (assign3480_e3373 * locals.var_argx__blk235);
        let assign3480_e3376: f64 = (1.0 + assign3480_e3375);
        let assign3480_e3377: f64 = (assign3480_e3369 * assign3480_e3376);
        (assign3480_e3377, (assign3480_e3369 * (locals.var_vbkd__blk234_dn1 * locals.var_argx__blk235)), (((assign3480_e3369 * ((locals.var_vmax_b_dn3 * locals.var_argx__blk235) + (locals.var_vmax_b * locals.var_argx__blk235_dn3))) * assign3480_e3376) + (assign3480_e3369 * (((locals.var_vbkd__blk234_dn3 - locals.var_vmax_b_dn3) * locals.var_argx__blk235) + (assign3480_e3373 * locals.var_argx__blk235_dn3)))), (assign3480_e3369 * (locals.var_vbkd__blk234_dn5 * locals.var_argx__blk235)),)
    } else {
        (locals.var_expx__blk236, locals.var_expx__blk236_dn1, locals.var_expx__blk236_dn3, locals.var_expx__blk236_dn5,)
    }
};
        locals.var_expx__blk236 = assign3480_e3379;
        locals.var_expx__blk236_dn1 = assign3480_e3379_d_n1;
        locals.var_expx__blk236_dn3 = assign3480_e3379_d_n3;
        locals.var_expx__blk236_dn5 = assign3480_e3379_d_n5;

        let (assign3490_e3394, assign3490_e3394_d_n1, assign3490_e3394_d_n3, assign3490_e3394_d_n5,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard237 != 0.0)) {
        let assign3490_e3384: f64 = (-p.p84);
        let assign3490_e3387: f64 = (-locals.var_vbv_t);
        let assign3490_e3389: f64 = (assign3490_e3387 * locals.var_argx__blk235);
        let assign3490_e3390: f64 = (assign3490_e3389).exp();
        let assign3490_e3391: f64 = (locals.var_expx__blk236 - assign3490_e3390);
        let assign3490_e3392: f64 = (assign3490_e3384 * assign3490_e3391);
        (assign3490_e3392, (assign3490_e3384 * locals.var_expx__blk236_dn1), (assign3490_e3384 * (locals.var_expx__blk236_dn3 - (assign3490_e3390 * (((-locals.var_vbv_t_dn3) * locals.var_argx__blk235) + (assign3490_e3387 * locals.var_argx__blk235_dn3))))), (assign3490_e3384 * locals.var_expx__blk236_dn5),)
    } else {
        (locals.var_ib2, locals.var_ib2_dn1, locals.var_ib2_dn3, locals.var_ib2_dn5,)
    }
};
        locals.var_ib2 = assign3490_e3394;
        locals.var_ib2_dn1 = assign3490_e3394_d_n1;
        locals.var_ib2_dn3 = assign3490_e3394_d_n3;
        locals.var_ib2_dn5 = assign3490_e3394_d_n5;

        let (assign3500_e3401, assign3500_e3401_d_n1, assign3500_e3401_d_n3, assign3500_e3401_d_n5,) = {
    if ((locals.var_guard223 != 0.0) && (locals.var_guard237 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ib2, locals.var_ib2_dn1, locals.var_ib2_dn3, locals.var_ib2_dn5,)
    }
};
        locals.var_ib2 = assign3500_e3401;
        locals.var_ib2_dn1 = assign3500_e3401_d_n1;
        locals.var_ib2_dn3 = assign3500_e3401_d_n3;
        locals.var_ib2_dn5 = assign3500_e3401_d_n5;

        let (assign3510_e3411, assign3510_e3411_d_n1, assign3510_e3411_d_n3, assign3510_e3411_d_n4, assign3510_e3411_d_n5,) = {
    if (locals.var_guard223 != 0.0) {
        let assign3510_e3405: f64 = (locals.var_id2 + locals.var_ib2);
        let assign3510_e3408: f64 = (locals.var_gmin * locals.var_vc2);
        let assign3510_e3409: f64 = (assign3510_e3405 + assign3510_e3408);
        (assign3510_e3409, ((locals.var_id2_dn1 + locals.var_ib2_dn1) + (locals.var_gmin * locals.var_vc2_dn1)), (locals.var_id2_dn3 + locals.var_ib2_dn3), locals.var_id2_dn4, ((locals.var_id2_dn5 + locals.var_ib2_dn5) + (locals.var_gmin * locals.var_vc2_dn5)),)
    } else {
        (locals.var_ip2, locals.var_ip2_dn1, locals.var_ip2_dn3, locals.var_ip2_dn4, locals.var_ip2_dn5,)
    }
};
        locals.var_ip2 = assign3510_e3411;
        locals.var_ip2_dn1 = assign3510_e3411_d_n1;
        locals.var_ip2_dn3 = assign3510_e3411_d_n3;
        locals.var_ip2_dn4 = assign3510_e3411_d_n4;
        locals.var_ip2_dn5 = assign3510_e3411_d_n5;

        let (assign3520_e3416, assign3520_e3416_d_n1, assign3520_e3416_d_n3, assign3520_e3416_d_n4, assign3520_e3416_d_n5,) = {
    if (locals.var_guard223 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_id2, locals.var_id2_dn1, locals.var_id2_dn3, locals.var_id2_dn4, locals.var_id2_dn5,)
    }
};
        locals.var_id2 = assign3520_e3416;
        locals.var_id2_dn1 = assign3520_e3416_d_n1;
        locals.var_id2_dn3 = assign3520_e3416_d_n3;
        locals.var_id2_dn4 = assign3520_e3416_d_n4;
        locals.var_id2_dn5 = assign3520_e3416_d_n5;

        let (assign3530_e3421, assign3530_e3421_d_n1, assign3530_e3421_d_n3, assign3530_e3421_d_n5,) = {
    if (locals.var_guard223 == 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ib2, locals.var_ib2_dn1, locals.var_ib2_dn3, locals.var_ib2_dn5,)
    }
};
        locals.var_ib2 = assign3530_e3421;
        locals.var_ib2_dn1 = assign3530_e3421_d_n1;
        locals.var_ib2_dn3 = assign3530_e3421_d_n3;
        locals.var_ib2_dn5 = assign3530_e3421_d_n5;

        let (assign3540_e3426, assign3540_e3426_d_n1, assign3540_e3426_d_n3, assign3540_e3426_d_n4, assign3540_e3426_d_n5,) = {
    if (locals.var_guard223 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ip2, locals.var_ip2_dn1, locals.var_ip2_dn3, locals.var_ip2_dn4, locals.var_ip2_dn5,)
    }
};
        locals.var_ip2 = assign3540_e3426;
        locals.var_ip2_dn1 = assign3540_e3426_d_n1;
        locals.var_ip2_dn3 = assign3540_e3426_d_n3;
        locals.var_ip2_dn4 = assign3540_e3426_d_n4;
        locals.var_ip2_dn5 = assign3540_e3426_d_n5;

        let assign3550_e3429: f64 = (locals.var_irb * locals.var_vrb);
        let assign3550_e3432: f64 = (locals.var_ip1 * locals.var_vc1);
        let assign3550_e3433: f64 = (assign3550_e3429 + assign3550_e3432);
        let assign3550_e3436: f64 = (locals.var_ip2 * locals.var_vc2);
        let assign3550_e3437: f64 = (assign3550_e3433 + assign3550_e3436);
        let assign3550_e3440: f64 = (bi0 * (nv0 - nv4));
        let assign3550_e3441: f64 = (assign3550_e3437 + assign3550_e3440);
        let assign3550_e3444: f64 = (bi1 * (nv2 - nv5));
        let assign3550_e3445: f64 = (assign3550_e3441 + assign3550_e3444);
        locals.var_power = assign3550_e3445;
        locals.var_power_dn0 = bi0;
        locals.var_power_dn1 = (((locals.var_irb_dn1 * locals.var_vrb) + ((locals.var_ip1_dn1 * locals.var_vc1) + (locals.var_ip1 * locals.var_vc1_dn1))) + ((locals.var_ip2_dn1 * locals.var_vc2) + (locals.var_ip2 * locals.var_vc2_dn1)));
        locals.var_power_dn2 = bi1;
        locals.var_power_dn3 = (((locals.var_irb_dn3 * locals.var_vrb) + (locals.var_ip1_dn3 * locals.var_vc1)) + (locals.var_ip2_dn3 * locals.var_vc2));
        locals.var_power_dn4 = (((((locals.var_irb_dn4 * locals.var_vrb) + (locals.var_irb * locals.var_vrb_dn4)) + ((locals.var_ip1_dn4 * locals.var_vc1) + (locals.var_ip1 * locals.var_vc1_dn4))) + (locals.var_ip2_dn4 * locals.var_vc2)) + (-bi0));
        locals.var_power_dn5 = (((((locals.var_irb_dn5 * locals.var_vrb) + (locals.var_irb * locals.var_vrb_dn5)) + (locals.var_ip1_dn5 * locals.var_vc1)) + ((locals.var_ip2_dn5 * locals.var_vc2) + (locals.var_ip2 * locals.var_vc2_dn5))) + (-bi1));
        locals.var_power_db0 = (nv0 - nv4);
        locals.var_power_db1 = (nv2 - nv5);

        let assign3560_e3453: f64 = if (((locals.var_gth > 0.0) && (p.p14 != 0.0)) && (p.p15 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard239 = assign3560_e3453;

        let (assign3570_e3458, assign3570_e3458_d_n0, assign3570_e3458_d_n1, assign3570_e3458_d_n2, assign3570_e3458_d_n3, assign3570_e3458_d_n4, assign3570_e3458_d_n5, assign3570_e3458_d_b0, assign3570_e3458_d_b1,) = {
    if (locals.var_guard239 != 0.0) {
        let assign3570_e3456: f64 = (-locals.var_power);
        (assign3570_e3456, (-locals.var_power_dn0), (-locals.var_power_dn1), (-locals.var_power_dn2), (-locals.var_power_dn3), (-locals.var_power_dn4), (-locals.var_power_dn5), (-locals.var_power_db0), (-locals.var_power_db1),)
    } else {
        (locals.var_ith, locals.var_ith_dn0, locals.var_ith_dn1, locals.var_ith_dn2, locals.var_ith_dn3, locals.var_ith_dn4, locals.var_ith_dn5, locals.var_ith_db0, locals.var_ith_db1,)
    }
};
        locals.var_ith = assign3570_e3458;
        locals.var_ith_dn0 = assign3570_e3458_d_n0;
        locals.var_ith_dn1 = assign3570_e3458_d_n1;
        locals.var_ith_dn2 = assign3570_e3458_d_n2;
        locals.var_ith_dn3 = assign3570_e3458_d_n3;
        locals.var_ith_dn4 = assign3570_e3458_d_n4;
        locals.var_ith_dn5 = assign3570_e3458_d_n5;
        locals.var_ith_db0 = assign3570_e3458_d_b0;
        locals.var_ith_db1 = assign3570_e3458_d_b1;

        let assign3580_e3461: f64 = if p.p109 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign3580_e3461;

        let (assign3590_e3469, assign3590_e3469_d_n3,) = {
    if ((locals.var_guard239 != 0.0) && (locals.var_guard240 != 0.0)) {
        let assign3590_e3467: f64 = (locals.var_gth * locals.var_dt_et);
        (assign3590_e3467, ((locals.var_gth_dn3 * locals.var_dt_et) + (locals.var_gth * locals.var_dt_et_dn3)),)
    } else {
        (locals.var_irth, locals.var_irth_dn3,)
    }
};
        locals.var_irth = assign3590_e3469;
        locals.var_irth_dn3 = assign3590_e3469_d_n3;

        let (assign3600_e3480,) = {
    if ((locals.var_guard239 != 0.0) && (locals.var_guard240 == 0.0)) {
        let assign3600_e3474: f64 = ctx_temp;
        let assign3600_e3476: f64 = (assign3600_e3474 + p.p9);
        let assign3600_e3478: f64 = (assign3600_e3476 - 273.15);
        (assign3600_e3478,)
    } else {
        (locals.var_tambc,)
    }
};
        locals.var_tambc = assign3600_e3480;

        let assign3610_e3484: f64 = (p.p35 + 1.0);
        let assign3610_e3485: f64 = if locals.var_tambc < assign3610_e3484 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign3610_e3485;

        let (assign3620_e3501,) = {
    if (((locals.var_guard239 != 0.0) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign3620_e3495: f64 = (locals.var_tambc - p.p35);
        let assign3620_e3497: f64 = (assign3620_e3495 - 1.0);
        let assign3620_e3498: f64 = (assign3620_e3497).exp();
        let assign3620_e3499: f64 = (p.p35 + assign3620_e3498);
        (assign3620_e3499,)
    } else {
        (locals.var_tambc,)
    }
};
        locals.var_tambc = assign3620_e3501;

        let assign3630_e3505: f64 = (p.p36 - 1.0);
        let assign3630_e3506: f64 = if locals.var_tambc > assign3630_e3505 { 1.0 } else { 0.0 };
        locals.var_guard242 = assign3630_e3506;

        let (assign3640_e3525,) = {
    if ((((locals.var_guard239 != 0.0) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 != 0.0)) {
        let assign3640_e3519: f64 = (p.p36 - locals.var_tambc);
        let assign3640_e3521: f64 = (assign3640_e3519 - 1.0);
        let assign3640_e3522: f64 = (assign3640_e3521).exp();
        let assign3640_e3523: f64 = (p.p36 - assign3640_e3522);
        (assign3640_e3523,)
    } else {
        (locals.var_tambc,)
    }
};
        locals.var_tambc = assign3640_e3525;

        let (assign3650_e3538,) = {
    if ((((locals.var_guard239 != 0.0) && (locals.var_guard240 == 0.0)) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 == 0.0)) {
        (locals.var_tambc,)
    } else {
        (locals.var_tambc,)
    }
};
        locals.var_tambc = assign3650_e3538;

        let (assign3660_e3547,) = {
    if ((locals.var_guard239 != 0.0) && (locals.var_guard240 == 0.0)) {
        let assign3660_e3545: f64 = (locals.var_tambc + 273.15);
        (assign3660_e3545,)
    } else {
        (locals.var_tambk,)
    }
};
        locals.var_tambk = assign3660_e3547;

        let assign3670_e3550: f64 = (p.p109 + 1.0);
        let assign3670_e3551: f64 = (assign3670_e3550).abs();
        let assign3670_e3553: f64 = if assign3670_e3551 > 0.1 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign3670_e3553;

        let (assign3680_e3580, assign3680_e3580_d_n3,) = {
    if (((locals.var_guard239 != 0.0) && (locals.var_guard240 == 0.0)) && (locals.var_guard243 != 0.0)) {
        let assign3680_e3562: f64 = (locals.var_gth * locals.var_tambk);
        let assign3680_e3566: f64 = (locals.var_dt_et / locals.var_tambk);
        let assign3680_e3567: f64 = (1.0 + assign3680_e3566);
        let assign3680_e3570: f64 = (1.0 + p.p109);
        let assign3680_e3571: f64 = (assign3680_e3567).powf(assign3680_e3570);
        let assign3680_e3573: f64 = (assign3680_e3571 - 1.0);
        let assign3680_e3574: f64 = (assign3680_e3562 * assign3680_e3573);
        let assign3680_e3577: f64 = (1.0 + p.p109);
        let assign3680_e3578: f64 = (assign3680_e3574 / assign3680_e3577);
        (assign3680_e3578, ((((locals.var_gth_dn3 * locals.var_tambk) * assign3680_e3573) + (assign3680_e3562 * if 0.0 == 0.0 && ((assign3680_e3570) as f64).is_finite() && ((assign3680_e3570) as f64).fract() == 0.0 { if assign3680_e3570 == 0.0 { 0.0 } else { (assign3680_e3570 * ((assign3680_e3567).powf(assign3680_e3570 - 1.0) * (locals.var_dt_et_dn3 / locals.var_tambk))) } } else { (assign3680_e3571 * (assign3680_e3570 * ((locals.var_dt_et_dn3 / locals.var_tambk) / assign3680_e3567))) })) / assign3680_e3577),)
    } else {
        (locals.var_irth, locals.var_irth_dn3,)
    }
};
        locals.var_irth = assign3680_e3580;
        locals.var_irth_dn3 = assign3680_e3580_d_n3;

        let (assign3690_e3602, assign3690_e3602_d_n3,) = {
    if (((locals.var_guard239 != 0.0) && (locals.var_guard240 == 0.0)) && (locals.var_guard243 == 0.0)) {
        let assign3690_e3590: f64 = (locals.var_gth * locals.var_dt_et);
        let assign3690_e3594: f64 = (0.5 * p.p109);
        let assign3690_e3596: f64 = (assign3690_e3594 * locals.var_dt_et);
        let assign3690_e3598: f64 = (assign3690_e3596 / locals.var_tambk);
        let assign3690_e3599: f64 = (1.0 + assign3690_e3598);
        let assign3690_e3600: f64 = (assign3690_e3590 * assign3690_e3599);
        (assign3690_e3600, ((((locals.var_gth_dn3 * locals.var_dt_et) + (locals.var_gth * locals.var_dt_et_dn3)) * assign3690_e3599) + (assign3690_e3590 * ((assign3690_e3594 * locals.var_dt_et_dn3) / locals.var_tambk))),)
    } else {
        (locals.var_irth, locals.var_irth_dn3,)
    }
};
        locals.var_irth = assign3690_e3602;
        locals.var_irth_dn3 = assign3690_e3602_d_n3;

        let (assign3700_e3607, assign3700_e3607_d_n0, assign3700_e3607_d_n1, assign3700_e3607_d_n2, assign3700_e3607_d_n3, assign3700_e3607_d_n4, assign3700_e3607_d_n5, assign3700_e3607_d_b0, assign3700_e3607_d_b1,) = {
    if (locals.var_guard239 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ith, locals.var_ith_dn0, locals.var_ith_dn1, locals.var_ith_dn2, locals.var_ith_dn3, locals.var_ith_dn4, locals.var_ith_dn5, locals.var_ith_db0, locals.var_ith_db1,)
    }
};
        locals.var_ith = assign3700_e3607;
        locals.var_ith_dn0 = assign3700_e3607_d_n0;
        locals.var_ith_dn1 = assign3700_e3607_d_n1;
        locals.var_ith_dn2 = assign3700_e3607_d_n2;
        locals.var_ith_dn3 = assign3700_e3607_d_n3;
        locals.var_ith_dn4 = assign3700_e3607_d_n4;
        locals.var_ith_dn5 = assign3700_e3607_d_n5;
        locals.var_ith_db0 = assign3700_e3607_d_b0;
        locals.var_ith_db1 = assign3700_e3607_d_b1;

        let (assign3710_e3614, assign3710_e3614_d_n3,) = {
    if (locals.var_guard239 == 0.0) {
        let assign3710_e3612: f64 = (1000000.0 * locals.var_dt_et);
        (assign3710_e3612, (1000000.0 * locals.var_dt_et_dn3),)
    } else {
        (locals.var_irth, locals.var_irth_dn3,)
    }
};
        locals.var_irth = assign3710_e3614;
        locals.var_irth_dn3 = assign3710_e3614_d_n3;

        let assign3720_e3616: f64 = (-p.p21);
        let assign3720_e3618: f64 = (assign3720_e3616 * locals.var_irb);
        locals.var_irb = assign3720_e3618;
        locals.var_irb_dn1 = (assign3720_e3616 * locals.var_irb_dn1);
        locals.var_irb_dn3 = (assign3720_e3616 * locals.var_irb_dn3);
        locals.var_irb_dn4 = (assign3720_e3616 * locals.var_irb_dn4);
        locals.var_irb_dn5 = (assign3720_e3616 * locals.var_irb_dn5);

        let assign3730_e3620: f64 = (-p.p21);
        let assign3730_e3622: f64 = (assign3730_e3620 * locals.var_ip1);
        locals.var_ip1 = assign3730_e3622;
        locals.var_ip1_dn1 = (assign3730_e3620 * locals.var_ip1_dn1);
        locals.var_ip1_dn3 = (assign3730_e3620 * locals.var_ip1_dn3);
        locals.var_ip1_dn4 = (assign3730_e3620 * locals.var_ip1_dn4);
        locals.var_ip1_dn5 = (assign3730_e3620 * locals.var_ip1_dn5);

        let assign3740_e3624: f64 = (-p.p21);
        let assign3740_e3626: f64 = (assign3740_e3624 * locals.var_ip2);
        locals.var_ip2 = assign3740_e3626;
        locals.var_ip2_dn1 = (assign3740_e3624 * locals.var_ip2_dn1);
        locals.var_ip2_dn3 = (assign3740_e3624 * locals.var_ip2_dn3);
        locals.var_ip2_dn4 = (assign3740_e3624 * locals.var_ip2_dn4);
        locals.var_ip2_dn5 = (assign3740_e3624 * locals.var_ip2_dn5);

        let assign3800_e3655: f64 = if locals.var_cj1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign3800_e3655;

        let (assign3810_e3676, assign3810_e3676_d_n1, assign3810_e3676_d_n3, assign3810_e3676_d_n4, assign3810_e3676_d_n5,) = {
    if ((locals.var_guard249 != 0.0) && (p.p63 != 0.0)) {
        let assign3810_e3662: f64 = (locals.var_vc1 - locals.var_vpo);
        let assign3810_e3665: f64 = (locals.var_vc1 + locals.var_vpo);
        let assign3810_e3668: f64 = (locals.var_vc1 + locals.var_vpo);
        let assign3810_e3669: f64 = (assign3810_e3665 * assign3810_e3668);
        let assign3810_e3671: f64 = (assign3810_e3669 + 0.04);
        let assign3810_e3672: f64 = (assign3810_e3671).sqrt();
        let assign3810_e3673: f64 = (assign3810_e3662 + assign3810_e3672);
        let assign3810_e3674: f64 = (0.5 * assign3810_e3673);
        (assign3810_e3674, (0.5 * (locals.var_vc1_dn1 + (((locals.var_vc1_dn1 * assign3810_e3668) + (assign3810_e3665 * locals.var_vc1_dn1)) / (2.0 * assign3810_e3672)))), (0.5 * ((-locals.var_vpo_dn3) + (((locals.var_vpo_dn3 * assign3810_e3668) + (assign3810_e3665 * locals.var_vpo_dn3)) / (2.0 * assign3810_e3672)))), (0.5 * (locals.var_vc1_dn4 + (((locals.var_vc1_dn4 * assign3810_e3668) + (assign3810_e3665 * locals.var_vc1_dn4)) / (2.0 * assign3810_e3672)))), 0.0,)
    } else {
        (locals.var_vcl, locals.var_vcl_dn1, locals.var_vcl_dn3, locals.var_vcl_dn4, locals.var_vcl_dn5,)
    }
};
        locals.var_vcl = assign3810_e3676;
        locals.var_vcl_dn1 = assign3810_e3676_d_n1;
        locals.var_vcl_dn3 = assign3810_e3676_d_n3;
        locals.var_vcl_dn4 = assign3810_e3676_d_n4;
        locals.var_vcl_dn5 = assign3810_e3676_d_n5;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3820_e3683, assign3820_e3683_d_n1, assign3820_e3683_d_n3, assign3820_e3683_d_n4, assign3820_e3683_d_n5,) = {
    if ((locals.var_guard249 != 0.0) && (p.p63 == 0.0)) {
        (locals.var_vc1, locals.var_vc1_dn1, 0.0, locals.var_vc1_dn4, 0.0,)
    } else {
        (locals.var_vcl, locals.var_vcl_dn1, locals.var_vcl_dn3, locals.var_vcl_dn4, locals.var_vcl_dn5,)
    }
};
        locals.var_vcl = assign3820_e3683;
        locals.var_vcl_dn1 = assign3820_e3683_d_n1;
        locals.var_vcl_dn3 = assign3820_e3683_d_n3;
        locals.var_vcl_dn4 = assign3820_e3683_d_n4;
        locals.var_vcl_dn5 = assign3820_e3683_d_n5;

        let (assign3830_e3689, assign3830_e3689_d_n1, assign3830_e3689_d_n3, assign3830_e3689_d_n4, assign3830_e3689_d_n5,) = {
    if (locals.var_guard249 != 0.0) {
        let assign3830_e3687: f64 = (locals.var_a1_um2 * locals.var_cja_t);
        (assign3830_e3687, (locals.var_a1_um2_dn1 * locals.var_cja_t), ((locals.var_a1_um2_dn3 * locals.var_cja_t) + (locals.var_a1_um2 * locals.var_cja_t_dn3)), (locals.var_a1_um2_dn4 * locals.var_cja_t), (locals.var_a1_um2_dn5 * locals.var_cja_t),)
    } else {
        (locals.var_acja, locals.var_acja_dn1, locals.var_acja_dn3, locals.var_acja_dn4, locals.var_acja_dn5,)
    }
};
        locals.var_acja = assign3830_e3689;
        locals.var_acja_dn1 = assign3830_e3689_d_n1;
        locals.var_acja_dn3 = assign3830_e3689_d_n3;
        locals.var_acja_dn4 = assign3830_e3689_d_n4;
        locals.var_acja_dn5 = assign3830_e3689_d_n5;

        let (assign3840_e3695, assign3840_e3695_d_n3,) = {
    if (locals.var_guard249 != 0.0) {
        let assign3840_e3693: f64 = (locals.var_p1_um * locals.var_cjp_t);
        (assign3840_e3693, (locals.var_p1_um * locals.var_cjp_t_dn3),)
    } else {
        (locals.var_pcjp, locals.var_pcjp_dn3,)
    }
};
        locals.var_pcjp = assign3840_e3695;
        locals.var_pcjp_dn3 = assign3840_e3695_d_n3;

        let assign3850_e3698: f64 = if locals.var_acja > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign3850_e3698;

        let (assign3860_e3707, assign3860_e3707_d_n3,) = {
    if ((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) {
        let assign3860_e3703: f64 = (-locals.var_pa_t);
        let assign3860_e3705: f64 = (assign3860_e3703 * p.p68);
        (assign3860_e3705, ((-locals.var_pa_t_dn3) * p.p68),)
    } else {
        (locals.var_dv0, locals.var_dv0_dn3,)
    }
};
        locals.var_dv0 = assign3860_e3707;
        locals.var_dv0_dn3 = assign3860_e3707_d_n3;

        let assign3870_e3710: f64 = if p.p75 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign3870_e3710;

        let (assign3880_e3720, assign3880_e3720_d_n1, assign3880_e3720_d_n3, assign3880_e3720_d_n4, assign3880_e3720_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) {
        let assign3880_e3718: f64 = (locals.var_vcl + locals.var_dv0);
        (assign3880_e3718, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dvh, locals.var_dvh_dn1, locals.var_dvh_dn3, locals.var_dvh_dn4, locals.var_dvh_dn5,)
    }
};
        locals.var_dvh = assign3880_e3720;
        locals.var_dvh_dn1 = assign3880_e3720_d_n1;
        locals.var_dvh_dn3 = assign3880_e3720_d_n3;
        locals.var_dvh_dn4 = assign3880_e3720_d_n4;
        locals.var_dvh_dn5 = assign3880_e3720_d_n5;

        let assign3890_e3723: f64 = if locals.var_dvh > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign3890_e3723;

        let (assign3900_e3738,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign3900_e3733: f64 = (1.0 - p.p68);
        let assign3900_e3735: f64 = (-p.p74);
        let assign3900_e3736: f64 = (assign3900_e3733).powf(assign3900_e3735);
        (assign3900_e3736,)
    } else {
        (locals.var_pwq,)
    }
};
        locals.var_pwq = assign3900_e3738;

        let (assign3910_e3760, assign3910_e3760_d_n1, assign3910_e3760_d_n3, assign3910_e3760_d_n4, assign3910_e3760_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign3910_e3751: f64 = (1.0 - p.p68);
        let assign3910_e3752: f64 = (locals.var_pwq * assign3910_e3751);
        let assign3910_e3753: f64 = (1.0 - assign3910_e3752);
        let assign3910_e3754: f64 = (locals.var_pa_t * assign3910_e3753);
        let assign3910_e3757: f64 = (1.0 - p.p74);
        let assign3910_e3758: f64 = (assign3910_e3754 / assign3910_e3757);
        (assign3910_e3758, 0.0, ((locals.var_pa_t_dn3 * assign3910_e3753) / assign3910_e3757), 0.0, 0.0,)
    } else {
        (locals.var_qlo, locals.var_qlo_dn1, locals.var_qlo_dn3, locals.var_qlo_dn4, locals.var_qlo_dn5,)
    }
};
        locals.var_qlo = assign3910_e3760;
        locals.var_qlo_dn1 = assign3910_e3760_d_n1;
        locals.var_qlo_dn3 = assign3910_e3760_d_n3;
        locals.var_qlo_dn4 = assign3910_e3760_d_n4;
        locals.var_qlo_dn5 = assign3910_e3760_d_n5;

        let (assign3920_e3786, assign3920_e3786_d_n1, assign3920_e3786_d_n3, assign3920_e3786_d_n4, assign3920_e3786_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign3920_e3772: f64 = (0.5 * p.p74);
        let assign3920_e3774: f64 = (assign3920_e3772 * locals.var_dvh);
        let assign3920_e3778: f64 = (1.0 - p.p68);
        let assign3920_e3779: f64 = (locals.var_pa_t * assign3920_e3778);
        let assign3920_e3780: f64 = (assign3920_e3774 / assign3920_e3779);
        let assign3920_e3781: f64 = (1.0 + assign3920_e3780);
        let assign3920_e3782: f64 = (locals.var_dvh * assign3920_e3781);
        let assign3920_e3784: f64 = (assign3920_e3782 * locals.var_pwq);
        (assign3920_e3784, (((locals.var_dvh_dn1 * assign3920_e3781) + (locals.var_dvh * ((assign3920_e3772 * locals.var_dvh_dn1) / assign3920_e3779))) * locals.var_pwq), (((locals.var_dvh_dn3 * assign3920_e3781) + (locals.var_dvh * ((((assign3920_e3772 * locals.var_dvh_dn3) * assign3920_e3779) - (assign3920_e3774 * (locals.var_pa_t_dn3 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * locals.var_pwq), (((locals.var_dvh_dn4 * assign3920_e3781) + (locals.var_dvh * ((assign3920_e3772 * locals.var_dvh_dn4) / assign3920_e3779))) * locals.var_pwq), (((locals.var_dvh_dn5 * assign3920_e3781) + (locals.var_dvh * ((assign3920_e3772 * locals.var_dvh_dn5) / assign3920_e3779))) * locals.var_pwq),)
    } else {
        (locals.var_qhi, locals.var_qhi_dn1, locals.var_qhi_dn3, locals.var_qhi_dn4, locals.var_qhi_dn5,)
    }
};
        locals.var_qhi = assign3920_e3786;
        locals.var_qhi_dn1 = assign3920_e3786_d_n1;
        locals.var_qhi_dn3 = assign3920_e3786_d_n3;
        locals.var_qhi_dn4 = assign3920_e3786_d_n4;
        locals.var_qhi_dn5 = assign3920_e3786_d_n5;

        let (assign3930_e3813, assign3930_e3813_d_n1, assign3930_e3813_d_n3, assign3930_e3813_d_n4, assign3930_e3813_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign3930_e3800: f64 = (locals.var_vcl / locals.var_pa_t);
        let assign3930_e3801: f64 = (1.0 - assign3930_e3800);
        let assign3930_e3804: f64 = (1.0 - p.p74);
        let assign3930_e3805: f64 = (assign3930_e3801).powf(assign3930_e3804);
        let assign3930_e3806: f64 = (1.0 - assign3930_e3805);
        let assign3930_e3807: f64 = (locals.var_pa_t * assign3930_e3806);
        let assign3930_e3810: f64 = (1.0 - p.p74);
        let assign3930_e3811: f64 = (assign3930_e3807 / assign3930_e3810);
        (assign3930_e3811, ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(locals.var_vcl_dn1 / locals.var_pa_t)))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(locals.var_vcl_dn1 / locals.var_pa_t)) / assign3930_e3801))) })) / assign3930_e3810), (((locals.var_pa_t_dn3 * assign3930_e3806) + (locals.var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((locals.var_vcl_dn3 * locals.var_pa_t) - (locals.var_vcl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((locals.var_vcl_dn3 * locals.var_pa_t) - (locals.var_vcl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(locals.var_vcl_dn4 / locals.var_pa_t)))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(locals.var_vcl_dn4 / locals.var_pa_t)) / assign3930_e3801))) })) / assign3930_e3810), ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(locals.var_vcl_dn5 / locals.var_pa_t)))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(locals.var_vcl_dn5 / locals.var_pa_t)) / assign3930_e3801))) })) / assign3930_e3810),)
    } else {
        (locals.var_qlo, locals.var_qlo_dn1, locals.var_qlo_dn3, locals.var_qlo_dn4, locals.var_qlo_dn5,)
    }
};
        locals.var_qlo = assign3930_e3813;
        locals.var_qlo_dn1 = assign3930_e3813_d_n1;
        locals.var_qlo_dn3 = assign3930_e3813_d_n3;
        locals.var_qlo_dn4 = assign3930_e3813_d_n4;
        locals.var_qlo_dn5 = assign3930_e3813_d_n5;

        let (assign3940_e3824, assign3940_e3824_d_n1, assign3940_e3824_d_n3, assign3940_e3824_d_n4, assign3940_e3824_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi, locals.var_qhi_dn1, locals.var_qhi_dn3, locals.var_qhi_dn4, locals.var_qhi_dn5,)
    }
};
        locals.var_qhi = assign3940_e3824;
        locals.var_qhi_dn1 = assign3940_e3824_d_n1;
        locals.var_qhi_dn3 = assign3940_e3824_d_n3;
        locals.var_qhi_dn4 = assign3940_e3824_d_n4;
        locals.var_qhi_dn5 = assign3940_e3824_d_n5;

        let (assign3950_e3834, assign3950_e3834_d_n1, assign3950_e3834_d_n3, assign3950_e3834_d_n4, assign3950_e3834_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) {
        let assign3950_e3832: f64 = (locals.var_qlo + locals.var_qhi);
        (assign3950_e3832, (locals.var_qlo_dn1 + locals.var_qhi_dn1), (locals.var_qlo_dn3 + locals.var_qhi_dn3), (locals.var_qlo_dn4 + locals.var_qhi_dn4), (locals.var_qlo_dn5 + locals.var_qhi_dn5),)
    } else {
        (locals.var_arga, locals.var_arga_dn1, locals.var_arga_dn3, locals.var_arga_dn4, locals.var_arga_dn5,)
    }
};
        locals.var_arga = assign3950_e3834;
        locals.var_arga_dn1 = assign3950_e3834_d_n1;
        locals.var_arga_dn3 = assign3950_e3834_d_n3;
        locals.var_arga_dn4 = assign3950_e3834_d_n4;
        locals.var_arga_dn5 = assign3950_e3834_d_n5;

        let (assign3960_e3852, assign3960_e3852_d_n3,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign3960_e3843: f64 = (locals.var_dv0 * locals.var_dv0);
        let assign3960_e3846: f64 = (4.0 * p.p75);
        let assign3960_e3848: f64 = (assign3960_e3846 * p.p75);
        let assign3960_e3849: f64 = (assign3960_e3843 + assign3960_e3848);
        let assign3960_e3850: f64 = (assign3960_e3849).sqrt();
        (assign3960_e3850, (((locals.var_dv0_dn3 * locals.var_dv0) + (locals.var_dv0 * locals.var_dv0_dn3)) / (2.0 * assign3960_e3850)),)
    } else {
        (locals.var_mv0, locals.var_mv0_dn3,)
    }
};
        locals.var_mv0 = assign3960_e3852;
        locals.var_mv0_dn3 = assign3960_e3852_d_n3;

        let (assign3970_e3866, assign3970_e3866_d_n3,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign3970_e3860: f64 = (-0.5);
        let assign3970_e3863: f64 = (locals.var_dv0 + locals.var_mv0);
        let assign3970_e3864: f64 = (assign3970_e3860 * assign3970_e3863);
        (assign3970_e3864, (assign3970_e3860 * (locals.var_dv0_dn3 + locals.var_mv0_dn3)),)
    } else {
        (locals.var_vl0, locals.var_vl0_dn3,)
    }
};
        locals.var_vl0 = assign3970_e3866;
        locals.var_vl0_dn3 = assign3970_e3866_d_n3;

        let (assign3980_e3877, assign3980_e3877_d_n1, assign3980_e3877_d_n3, assign3980_e3877_d_n4, assign3980_e3877_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign3980_e3875: f64 = (locals.var_vcl + locals.var_dv0);
        (assign3980_e3875, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dv, locals.var_dv_dn1, locals.var_dv_dn3, locals.var_dv_dn4, locals.var_dv_dn5,)
    }
};
        locals.var_dv = assign3980_e3877;
        locals.var_dv_dn1 = assign3980_e3877_d_n1;
        locals.var_dv_dn3 = assign3980_e3877_d_n3;
        locals.var_dv_dn4 = assign3980_e3877_d_n4;
        locals.var_dv_dn5 = assign3980_e3877_d_n5;

        let (assign3990_e3895, assign3990_e3895_d_n1, assign3990_e3895_d_n3, assign3990_e3895_d_n4, assign3990_e3895_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign3990_e3886: f64 = (locals.var_dv * locals.var_dv);
        let assign3990_e3889: f64 = (4.0 * p.p75);
        let assign3990_e3891: f64 = (assign3990_e3889 * p.p75);
        let assign3990_e3892: f64 = (assign3990_e3886 + assign3990_e3891);
        let assign3990_e3893: f64 = (assign3990_e3892).sqrt();
        (assign3990_e3893, (((locals.var_dv_dn1 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn1)) / (2.0 * assign3990_e3893)), (((locals.var_dv_dn3 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn3)) / (2.0 * assign3990_e3893)), (((locals.var_dv_dn4 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn4)) / (2.0 * assign3990_e3893)), (((locals.var_dv_dn5 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn5)) / (2.0 * assign3990_e3893)),)
    } else {
        (locals.var_mv, locals.var_mv_dn1, locals.var_mv_dn3, locals.var_mv_dn4, locals.var_mv_dn5,)
    }
};
        locals.var_mv = assign3990_e3895;
        locals.var_mv_dn1 = assign3990_e3895_d_n1;
        locals.var_mv_dn3 = assign3990_e3895_d_n3;
        locals.var_mv_dn4 = assign3990_e3895_d_n4;
        locals.var_mv_dn5 = assign3990_e3895_d_n5;

        let (assign4000_e3910, assign4000_e3910_d_n1, assign4000_e3910_d_n3, assign4000_e3910_d_n4, assign4000_e3910_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign4000_e3905: f64 = (locals.var_dv - locals.var_mv);
        let assign4000_e3906: f64 = (0.5 * assign4000_e3905);
        let assign4000_e3908: f64 = (assign4000_e3906 - locals.var_dv0);
        (assign4000_e3908, (0.5 * (locals.var_dv_dn1 - locals.var_mv_dn1)), ((0.5 * (locals.var_dv_dn3 - locals.var_mv_dn3)) - locals.var_dv0_dn3), (0.5 * (locals.var_dv_dn4 - locals.var_mv_dn4)), (0.5 * (locals.var_dv_dn5 - locals.var_mv_dn5)),)
    } else {
        (locals.var_vl, locals.var_vl_dn1, locals.var_vl_dn3, locals.var_vl_dn4, locals.var_vl_dn5,)
    }
};
        locals.var_vl = assign4000_e3910;
        locals.var_vl_dn1 = assign4000_e3910_d_n1;
        locals.var_vl_dn3 = assign4000_e3910_d_n3;
        locals.var_vl_dn4 = assign4000_e3910_d_n4;
        locals.var_vl_dn5 = assign4000_e3910_d_n5;

        let (assign4010_e3934, assign4010_e3934_d_n1, assign4010_e3934_d_n3, assign4010_e3934_d_n4, assign4010_e3934_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign4010_e3918: f64 = (-locals.var_pa_t);
        let assign4010_e3922: f64 = (locals.var_vl / locals.var_pa_t);
        let assign4010_e3923: f64 = (1.0 - assign4010_e3922);
        let assign4010_e3926: f64 = (1.0 - p.p74);
        let assign4010_e3927: f64 = (assign4010_e3923).powf(assign4010_e3926);
        let assign4010_e3928: f64 = (assign4010_e3918 * assign4010_e3927);
        let assign4010_e3931: f64 = (1.0 - p.p74);
        let assign4010_e3932: f64 = (assign4010_e3928 / assign4010_e3931);
        (assign4010_e3932, ((assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(locals.var_vl_dn1 / locals.var_pa_t)))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(locals.var_vl_dn1 / locals.var_pa_t)) / assign4010_e3923))) }) / assign4010_e3931), ((((-locals.var_pa_t_dn3) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((locals.var_vl_dn3 * locals.var_pa_t) - (locals.var_vl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((locals.var_vl_dn3 * locals.var_pa_t) - (locals.var_vl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(locals.var_vl_dn4 / locals.var_pa_t)))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(locals.var_vl_dn4 / locals.var_pa_t)) / assign4010_e3923))) }) / assign4010_e3931), ((assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(locals.var_vl_dn5 / locals.var_pa_t)))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(locals.var_vl_dn5 / locals.var_pa_t)) / assign4010_e3923))) }) / assign4010_e3931),)
    } else {
        (locals.var_qlo, locals.var_qlo_dn1, locals.var_qlo_dn3, locals.var_qlo_dn4, locals.var_qlo_dn5,)
    }
};
        locals.var_qlo = assign4010_e3934;
        locals.var_qlo_dn1 = assign4010_e3934_d_n1;
        locals.var_qlo_dn3 = assign4010_e3934_d_n3;
        locals.var_qlo_dn4 = assign4010_e3934_d_n4;
        locals.var_qlo_dn5 = assign4010_e3934_d_n5;

        let (assign4020_e3974, assign4020_e3974_d_n1, assign4020_e3974_d_n3, assign4020_e3974_d_n4, assign4020_e3974_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign4020_e3944: f64 = (1.0 - p.p68);
        let assign4020_e3946: f64 = (-p.p74);
        let assign4020_e3947: f64 = (assign4020_e3944).powf(assign4020_e3946);
        let assign4020_e3950: f64 = (locals.var_vcl - locals.var_vl);
        let assign4020_e3952: f64 = (assign4020_e3950 + locals.var_vl0);
        let assign4020_e3953: f64 = (assign4020_e3947 * assign4020_e3952);
        let assign4020_e3957: f64 = (0.5 * p.p74);
        let assign4020_e3960: f64 = (locals.var_vcl - locals.var_vl);
        let assign4020_e3962: f64 = (assign4020_e3960 + locals.var_vl0);
        let assign4020_e3963: f64 = (assign4020_e3957 * assign4020_e3962);
        let assign4020_e3967: f64 = (1.0 - p.p68);
        let assign4020_e3968: f64 = (locals.var_pa_t * assign4020_e3967);
        let assign4020_e3969: f64 = (assign4020_e3963 / assign4020_e3968);
        let assign4020_e3970: f64 = (1.0 + assign4020_e3969);
        let assign4020_e3971: f64 = (assign4020_e3953 * assign4020_e3970);
        let assign4020_e3972: f64 = (locals.var_qlo + assign4020_e3971);
        (assign4020_e3972, (locals.var_qlo_dn1 + (((assign4020_e3947 * (locals.var_vcl_dn1 - locals.var_vl_dn1)) * assign4020_e3970) + (assign4020_e3953 * ((assign4020_e3957 * (locals.var_vcl_dn1 - locals.var_vl_dn1)) / assign4020_e3968)))), (locals.var_qlo_dn3 + (((assign4020_e3947 * ((locals.var_vcl_dn3 - locals.var_vl_dn3) + locals.var_vl0_dn3)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((locals.var_vcl_dn3 - locals.var_vl_dn3) + locals.var_vl0_dn3)) * assign4020_e3968) - (assign4020_e3963 * (locals.var_pa_t_dn3 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (locals.var_qlo_dn4 + (((assign4020_e3947 * (locals.var_vcl_dn4 - locals.var_vl_dn4)) * assign4020_e3970) + (assign4020_e3953 * ((assign4020_e3957 * (locals.var_vcl_dn4 - locals.var_vl_dn4)) / assign4020_e3968)))), (locals.var_qlo_dn5 + (((assign4020_e3947 * (locals.var_vcl_dn5 - locals.var_vl_dn5)) * assign4020_e3970) + (assign4020_e3953 * ((assign4020_e3957 * (locals.var_vcl_dn5 - locals.var_vl_dn5)) / assign4020_e3968)))),)
    } else {
        (locals.var_arga, locals.var_arga_dn1, locals.var_arga_dn3, locals.var_arga_dn4, locals.var_arga_dn5,)
    }
};
        locals.var_arga = assign4020_e3974;
        locals.var_arga_dn1 = assign4020_e3974_d_n1;
        locals.var_arga_dn3 = assign4020_e3974_d_n3;
        locals.var_arga_dn4 = assign4020_e3974_d_n4;
        locals.var_arga_dn5 = assign4020_e3974_d_n5;

        let (assign4030_e3981, assign4030_e3981_d_n1, assign4030_e3981_d_n3, assign4030_e3981_d_n4, assign4030_e3981_d_n5,) = {
    if ((locals.var_guard249 != 0.0) && (locals.var_guard254 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arga, locals.var_arga_dn1, locals.var_arga_dn3, locals.var_arga_dn4, locals.var_arga_dn5,)
    }
};
        locals.var_arga = assign4030_e3981;
        locals.var_arga_dn1 = assign4030_e3981_d_n1;
        locals.var_arga_dn3 = assign4030_e3981_d_n3;
        locals.var_arga_dn4 = assign4030_e3981_d_n4;
        locals.var_arga_dn5 = assign4030_e3981_d_n5;

        let assign4040_e3984: f64 = if locals.var_pcjp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign4040_e3984;

        let (assign4050_e3993, assign4050_e3993_d_n3,) = {
    if ((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign4050_e3989: f64 = (-locals.var_pp_t);
        let assign4050_e3991: f64 = (assign4050_e3989 * p.p68);
        (assign4050_e3991, ((-locals.var_pp_t_dn3) * p.p68),)
    } else {
        (locals.var_dv0__blk268, locals.var_dv0__blk268_dn3,)
    }
};
        locals.var_dv0__blk268 = assign4050_e3993;
        locals.var_dv0__blk268_dn3 = assign4050_e3993_d_n3;

        let assign4060_e3996: f64 = if p.p82 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign4060_e3996;

        let (assign4070_e4006, assign4070_e4006_d_n1, assign4070_e4006_d_n3, assign4070_e4006_d_n4, assign4070_e4006_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) {
        let assign4070_e4004: f64 = (locals.var_vcl + locals.var_dv0__blk268);
        (assign4070_e4004, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk268_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dvh__blk269, locals.var_dvh__blk269_dn1, locals.var_dvh__blk269_dn3, locals.var_dvh__blk269_dn4, locals.var_dvh__blk269_dn5,)
    }
};
        locals.var_dvh__blk269 = assign4070_e4006;
        locals.var_dvh__blk269_dn1 = assign4070_e4006_d_n1;
        locals.var_dvh__blk269_dn3 = assign4070_e4006_d_n3;
        locals.var_dvh__blk269_dn4 = assign4070_e4006_d_n4;
        locals.var_dvh__blk269_dn5 = assign4070_e4006_d_n5;

        let assign4080_e4009: f64 = if locals.var_dvh__blk269 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign4080_e4009;

        let (assign4090_e4024,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign4090_e4019: f64 = (1.0 - p.p68);
        let assign4090_e4021: f64 = (-p.p81);
        let assign4090_e4022: f64 = (assign4090_e4019).powf(assign4090_e4021);
        (assign4090_e4022,)
    } else {
        (locals.var_pwq__blk270,)
    }
};
        locals.var_pwq__blk270 = assign4090_e4024;

        let (assign4100_e4046, assign4100_e4046_d_n1, assign4100_e4046_d_n3, assign4100_e4046_d_n4, assign4100_e4046_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign4100_e4037: f64 = (1.0 - p.p68);
        let assign4100_e4038: f64 = (locals.var_pwq__blk270 * assign4100_e4037);
        let assign4100_e4039: f64 = (1.0 - assign4100_e4038);
        let assign4100_e4040: f64 = (locals.var_pp_t * assign4100_e4039);
        let assign4100_e4043: f64 = (1.0 - p.p81);
        let assign4100_e4044: f64 = (assign4100_e4040 / assign4100_e4043);
        (assign4100_e4044, 0.0, ((locals.var_pp_t_dn3 * assign4100_e4039) / assign4100_e4043), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk271, locals.var_qlo__blk271_dn1, locals.var_qlo__blk271_dn3, locals.var_qlo__blk271_dn4, locals.var_qlo__blk271_dn5,)
    }
};
        locals.var_qlo__blk271 = assign4100_e4046;
        locals.var_qlo__blk271_dn1 = assign4100_e4046_d_n1;
        locals.var_qlo__blk271_dn3 = assign4100_e4046_d_n3;
        locals.var_qlo__blk271_dn4 = assign4100_e4046_d_n4;
        locals.var_qlo__blk271_dn5 = assign4100_e4046_d_n5;

        let (assign4110_e4072, assign4110_e4072_d_n1, assign4110_e4072_d_n3, assign4110_e4072_d_n4, assign4110_e4072_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign4110_e4058: f64 = (0.5 * p.p81);
        let assign4110_e4060: f64 = (assign4110_e4058 * locals.var_dvh__blk269);
        let assign4110_e4064: f64 = (1.0 - p.p68);
        let assign4110_e4065: f64 = (locals.var_pp_t * assign4110_e4064);
        let assign4110_e4066: f64 = (assign4110_e4060 / assign4110_e4065);
        let assign4110_e4067: f64 = (1.0 + assign4110_e4066);
        let assign4110_e4068: f64 = (locals.var_dvh__blk269 * assign4110_e4067);
        let assign4110_e4070: f64 = (assign4110_e4068 * locals.var_pwq__blk270);
        (assign4110_e4070, (((locals.var_dvh__blk269_dn1 * assign4110_e4067) + (locals.var_dvh__blk269 * ((assign4110_e4058 * locals.var_dvh__blk269_dn1) / assign4110_e4065))) * locals.var_pwq__blk270), (((locals.var_dvh__blk269_dn3 * assign4110_e4067) + (locals.var_dvh__blk269 * ((((assign4110_e4058 * locals.var_dvh__blk269_dn3) * assign4110_e4065) - (assign4110_e4060 * (locals.var_pp_t_dn3 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * locals.var_pwq__blk270), (((locals.var_dvh__blk269_dn4 * assign4110_e4067) + (locals.var_dvh__blk269 * ((assign4110_e4058 * locals.var_dvh__blk269_dn4) / assign4110_e4065))) * locals.var_pwq__blk270), (((locals.var_dvh__blk269_dn5 * assign4110_e4067) + (locals.var_dvh__blk269 * ((assign4110_e4058 * locals.var_dvh__blk269_dn5) / assign4110_e4065))) * locals.var_pwq__blk270),)
    } else {
        (locals.var_qhi__blk272, locals.var_qhi__blk272_dn1, locals.var_qhi__blk272_dn3, locals.var_qhi__blk272_dn4, locals.var_qhi__blk272_dn5,)
    }
};
        locals.var_qhi__blk272 = assign4110_e4072;
        locals.var_qhi__blk272_dn1 = assign4110_e4072_d_n1;
        locals.var_qhi__blk272_dn3 = assign4110_e4072_d_n3;
        locals.var_qhi__blk272_dn4 = assign4110_e4072_d_n4;
        locals.var_qhi__blk272_dn5 = assign4110_e4072_d_n5;

        let (assign4120_e4099, assign4120_e4099_d_n1, assign4120_e4099_d_n3, assign4120_e4099_d_n4, assign4120_e4099_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 == 0.0)) {
        let assign4120_e4086: f64 = (locals.var_vcl / locals.var_pp_t);
        let assign4120_e4087: f64 = (1.0 - assign4120_e4086);
        let assign4120_e4090: f64 = (1.0 - p.p81);
        let assign4120_e4091: f64 = (assign4120_e4087).powf(assign4120_e4090);
        let assign4120_e4092: f64 = (1.0 - assign4120_e4091);
        let assign4120_e4093: f64 = (locals.var_pp_t * assign4120_e4092);
        let assign4120_e4096: f64 = (1.0 - p.p81);
        let assign4120_e4097: f64 = (assign4120_e4093 / assign4120_e4096);
        (assign4120_e4097, ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(locals.var_vcl_dn1 / locals.var_pp_t)))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(locals.var_vcl_dn1 / locals.var_pp_t)) / assign4120_e4087))) })) / assign4120_e4096), (((locals.var_pp_t_dn3 * assign4120_e4092) + (locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((locals.var_vcl_dn3 * locals.var_pp_t) - (locals.var_vcl * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((locals.var_vcl_dn3 * locals.var_pp_t) - (locals.var_vcl * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(locals.var_vcl_dn4 / locals.var_pp_t)))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(locals.var_vcl_dn4 / locals.var_pp_t)) / assign4120_e4087))) })) / assign4120_e4096), ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(locals.var_vcl_dn5 / locals.var_pp_t)))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(locals.var_vcl_dn5 / locals.var_pp_t)) / assign4120_e4087))) })) / assign4120_e4096),)
    } else {
        (locals.var_qlo__blk271, locals.var_qlo__blk271_dn1, locals.var_qlo__blk271_dn3, locals.var_qlo__blk271_dn4, locals.var_qlo__blk271_dn5,)
    }
};
        locals.var_qlo__blk271 = assign4120_e4099;
        locals.var_qlo__blk271_dn1 = assign4120_e4099_d_n1;
        locals.var_qlo__blk271_dn3 = assign4120_e4099_d_n3;
        locals.var_qlo__blk271_dn4 = assign4120_e4099_d_n4;
        locals.var_qlo__blk271_dn5 = assign4120_e4099_d_n5;

        let (assign4130_e4110, assign4130_e4110_d_n1, assign4130_e4110_d_n3, assign4130_e4110_d_n4, assign4130_e4110_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk272, locals.var_qhi__blk272_dn1, locals.var_qhi__blk272_dn3, locals.var_qhi__blk272_dn4, locals.var_qhi__blk272_dn5,)
    }
};
        locals.var_qhi__blk272 = assign4130_e4110;
        locals.var_qhi__blk272_dn1 = assign4130_e4110_d_n1;
        locals.var_qhi__blk272_dn3 = assign4130_e4110_d_n3;
        locals.var_qhi__blk272_dn4 = assign4130_e4110_d_n4;
        locals.var_qhi__blk272_dn5 = assign4130_e4110_d_n5;

        let (assign4140_e4120, assign4140_e4120_d_n1, assign4140_e4120_d_n3, assign4140_e4120_d_n4, assign4140_e4120_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) {
        let assign4140_e4118: f64 = (locals.var_qlo__blk271 + locals.var_qhi__blk272);
        (assign4140_e4118, (locals.var_qlo__blk271_dn1 + locals.var_qhi__blk272_dn1), (locals.var_qlo__blk271_dn3 + locals.var_qhi__blk272_dn3), (locals.var_qlo__blk271_dn4 + locals.var_qhi__blk272_dn4), (locals.var_qlo__blk271_dn5 + locals.var_qhi__blk272_dn5),)
    } else {
        (locals.var_argp, locals.var_argp_dn1, locals.var_argp_dn3, locals.var_argp_dn4, locals.var_argp_dn5,)
    }
};
        locals.var_argp = assign4140_e4120;
        locals.var_argp_dn1 = assign4140_e4120_d_n1;
        locals.var_argp_dn3 = assign4140_e4120_d_n3;
        locals.var_argp_dn4 = assign4140_e4120_d_n4;
        locals.var_argp_dn5 = assign4140_e4120_d_n5;

        let (assign4150_e4138, assign4150_e4138_d_n3,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4150_e4129: f64 = (locals.var_dv0__blk268 * locals.var_dv0__blk268);
        let assign4150_e4132: f64 = (4.0 * p.p82);
        let assign4150_e4134: f64 = (assign4150_e4132 * p.p82);
        let assign4150_e4135: f64 = (assign4150_e4129 + assign4150_e4134);
        let assign4150_e4136: f64 = (assign4150_e4135).sqrt();
        (assign4150_e4136, (((locals.var_dv0__blk268_dn3 * locals.var_dv0__blk268) + (locals.var_dv0__blk268 * locals.var_dv0__blk268_dn3)) / (2.0 * assign4150_e4136)),)
    } else {
        (locals.var_mv0__blk273, locals.var_mv0__blk273_dn3,)
    }
};
        locals.var_mv0__blk273 = assign4150_e4138;
        locals.var_mv0__blk273_dn3 = assign4150_e4138_d_n3;

        let (assign4160_e4152, assign4160_e4152_d_n3,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4160_e4146: f64 = (-0.5);
        let assign4160_e4149: f64 = (locals.var_dv0__blk268 + locals.var_mv0__blk273);
        let assign4160_e4150: f64 = (assign4160_e4146 * assign4160_e4149);
        (assign4160_e4150, (assign4160_e4146 * (locals.var_dv0__blk268_dn3 + locals.var_mv0__blk273_dn3)),)
    } else {
        (locals.var_vl0__blk274, locals.var_vl0__blk274_dn3,)
    }
};
        locals.var_vl0__blk274 = assign4160_e4152;
        locals.var_vl0__blk274_dn3 = assign4160_e4152_d_n3;

        let (assign4170_e4163, assign4170_e4163_d_n1, assign4170_e4163_d_n3, assign4170_e4163_d_n4, assign4170_e4163_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4170_e4161: f64 = (locals.var_vcl + locals.var_dv0__blk268);
        (assign4170_e4161, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk268_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dv__blk275, locals.var_dv__blk275_dn1, locals.var_dv__blk275_dn3, locals.var_dv__blk275_dn4, locals.var_dv__blk275_dn5,)
    }
};
        locals.var_dv__blk275 = assign4170_e4163;
        locals.var_dv__blk275_dn1 = assign4170_e4163_d_n1;
        locals.var_dv__blk275_dn3 = assign4170_e4163_d_n3;
        locals.var_dv__blk275_dn4 = assign4170_e4163_d_n4;
        locals.var_dv__blk275_dn5 = assign4170_e4163_d_n5;

        let (assign4180_e4181, assign4180_e4181_d_n1, assign4180_e4181_d_n3, assign4180_e4181_d_n4, assign4180_e4181_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4180_e4172: f64 = (locals.var_dv__blk275 * locals.var_dv__blk275);
        let assign4180_e4175: f64 = (4.0 * p.p82);
        let assign4180_e4177: f64 = (assign4180_e4175 * p.p82);
        let assign4180_e4178: f64 = (assign4180_e4172 + assign4180_e4177);
        let assign4180_e4179: f64 = (assign4180_e4178).sqrt();
        (assign4180_e4179, (((locals.var_dv__blk275_dn1 * locals.var_dv__blk275) + (locals.var_dv__blk275 * locals.var_dv__blk275_dn1)) / (2.0 * assign4180_e4179)), (((locals.var_dv__blk275_dn3 * locals.var_dv__blk275) + (locals.var_dv__blk275 * locals.var_dv__blk275_dn3)) / (2.0 * assign4180_e4179)), (((locals.var_dv__blk275_dn4 * locals.var_dv__blk275) + (locals.var_dv__blk275 * locals.var_dv__blk275_dn4)) / (2.0 * assign4180_e4179)), (((locals.var_dv__blk275_dn5 * locals.var_dv__blk275) + (locals.var_dv__blk275 * locals.var_dv__blk275_dn5)) / (2.0 * assign4180_e4179)),)
    } else {
        (locals.var_mv__blk276, locals.var_mv__blk276_dn1, locals.var_mv__blk276_dn3, locals.var_mv__blk276_dn4, locals.var_mv__blk276_dn5,)
    }
};
        locals.var_mv__blk276 = assign4180_e4181;
        locals.var_mv__blk276_dn1 = assign4180_e4181_d_n1;
        locals.var_mv__blk276_dn3 = assign4180_e4181_d_n3;
        locals.var_mv__blk276_dn4 = assign4180_e4181_d_n4;
        locals.var_mv__blk276_dn5 = assign4180_e4181_d_n5;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4190_e4196, assign4190_e4196_d_n1, assign4190_e4196_d_n3, assign4190_e4196_d_n4, assign4190_e4196_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4190_e4191: f64 = (locals.var_dv__blk275 - locals.var_mv__blk276);
        let assign4190_e4192: f64 = (0.5 * assign4190_e4191);
        let assign4190_e4194: f64 = (assign4190_e4192 - locals.var_dv0__blk268);
        (assign4190_e4194, (0.5 * (locals.var_dv__blk275_dn1 - locals.var_mv__blk276_dn1)), ((0.5 * (locals.var_dv__blk275_dn3 - locals.var_mv__blk276_dn3)) - locals.var_dv0__blk268_dn3), (0.5 * (locals.var_dv__blk275_dn4 - locals.var_mv__blk276_dn4)), (0.5 * (locals.var_dv__blk275_dn5 - locals.var_mv__blk276_dn5)),)
    } else {
        (locals.var_vl__blk277, locals.var_vl__blk277_dn1, locals.var_vl__blk277_dn3, locals.var_vl__blk277_dn4, locals.var_vl__blk277_dn5,)
    }
};
        locals.var_vl__blk277 = assign4190_e4196;
        locals.var_vl__blk277_dn1 = assign4190_e4196_d_n1;
        locals.var_vl__blk277_dn3 = assign4190_e4196_d_n3;
        locals.var_vl__blk277_dn4 = assign4190_e4196_d_n4;
        locals.var_vl__blk277_dn5 = assign4190_e4196_d_n5;

        let (assign4200_e4220, assign4200_e4220_d_n1, assign4200_e4220_d_n3, assign4200_e4220_d_n4, assign4200_e4220_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4200_e4204: f64 = (-locals.var_pp_t);
        let assign4200_e4208: f64 = (locals.var_vl__blk277 / locals.var_pp_t);
        let assign4200_e4209: f64 = (1.0 - assign4200_e4208);
        let assign4200_e4212: f64 = (1.0 - p.p81);
        let assign4200_e4213: f64 = (assign4200_e4209).powf(assign4200_e4212);
        let assign4200_e4214: f64 = (assign4200_e4204 * assign4200_e4213);
        let assign4200_e4217: f64 = (1.0 - p.p81);
        let assign4200_e4218: f64 = (assign4200_e4214 / assign4200_e4217);
        (assign4200_e4218, ((assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(locals.var_vl__blk277_dn1 / locals.var_pp_t)))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(locals.var_vl__blk277_dn1 / locals.var_pp_t)) / assign4200_e4209))) }) / assign4200_e4217), ((((-locals.var_pp_t_dn3) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((locals.var_vl__blk277_dn3 * locals.var_pp_t) - (locals.var_vl__blk277 * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((locals.var_vl__blk277_dn3 * locals.var_pp_t) - (locals.var_vl__blk277 * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(locals.var_vl__blk277_dn4 / locals.var_pp_t)))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(locals.var_vl__blk277_dn4 / locals.var_pp_t)) / assign4200_e4209))) }) / assign4200_e4217), ((assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(locals.var_vl__blk277_dn5 / locals.var_pp_t)))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(locals.var_vl__blk277_dn5 / locals.var_pp_t)) / assign4200_e4209))) }) / assign4200_e4217),)
    } else {
        (locals.var_qlo__blk271, locals.var_qlo__blk271_dn1, locals.var_qlo__blk271_dn3, locals.var_qlo__blk271_dn4, locals.var_qlo__blk271_dn5,)
    }
};
        locals.var_qlo__blk271 = assign4200_e4220;
        locals.var_qlo__blk271_dn1 = assign4200_e4220_d_n1;
        locals.var_qlo__blk271_dn3 = assign4200_e4220_d_n3;
        locals.var_qlo__blk271_dn4 = assign4200_e4220_d_n4;
        locals.var_qlo__blk271_dn5 = assign4200_e4220_d_n5;

        let (assign4210_e4260, assign4210_e4260_d_n1, assign4210_e4260_d_n3, assign4210_e4260_d_n4, assign4210_e4260_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4210_e4230: f64 = (1.0 - p.p68);
        let assign4210_e4232: f64 = (-p.p81);
        let assign4210_e4233: f64 = (assign4210_e4230).powf(assign4210_e4232);
        let assign4210_e4236: f64 = (locals.var_vcl - locals.var_vl__blk277);
        let assign4210_e4238: f64 = (assign4210_e4236 + locals.var_vl0__blk274);
        let assign4210_e4239: f64 = (assign4210_e4233 * assign4210_e4238);
        let assign4210_e4243: f64 = (0.5 * p.p81);
        let assign4210_e4246: f64 = (locals.var_vcl - locals.var_vl__blk277);
        let assign4210_e4248: f64 = (assign4210_e4246 + locals.var_vl0__blk274);
        let assign4210_e4249: f64 = (assign4210_e4243 * assign4210_e4248);
        let assign4210_e4253: f64 = (1.0 - p.p68);
        let assign4210_e4254: f64 = (locals.var_pp_t * assign4210_e4253);
        let assign4210_e4255: f64 = (assign4210_e4249 / assign4210_e4254);
        let assign4210_e4256: f64 = (1.0 + assign4210_e4255);
        let assign4210_e4257: f64 = (assign4210_e4239 * assign4210_e4256);
        let assign4210_e4258: f64 = (locals.var_qlo__blk271 + assign4210_e4257);
        (assign4210_e4258, (locals.var_qlo__blk271_dn1 + (((assign4210_e4233 * (locals.var_vcl_dn1 - locals.var_vl__blk277_dn1)) * assign4210_e4256) + (assign4210_e4239 * ((assign4210_e4243 * (locals.var_vcl_dn1 - locals.var_vl__blk277_dn1)) / assign4210_e4254)))), (locals.var_qlo__blk271_dn3 + (((assign4210_e4233 * ((locals.var_vcl_dn3 - locals.var_vl__blk277_dn3) + locals.var_vl0__blk274_dn3)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((locals.var_vcl_dn3 - locals.var_vl__blk277_dn3) + locals.var_vl0__blk274_dn3)) * assign4210_e4254) - (assign4210_e4249 * (locals.var_pp_t_dn3 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (locals.var_qlo__blk271_dn4 + (((assign4210_e4233 * (locals.var_vcl_dn4 - locals.var_vl__blk277_dn4)) * assign4210_e4256) + (assign4210_e4239 * ((assign4210_e4243 * (locals.var_vcl_dn4 - locals.var_vl__blk277_dn4)) / assign4210_e4254)))), (locals.var_qlo__blk271_dn5 + (((assign4210_e4233 * (locals.var_vcl_dn5 - locals.var_vl__blk277_dn5)) * assign4210_e4256) + (assign4210_e4239 * ((assign4210_e4243 * (locals.var_vcl_dn5 - locals.var_vl__blk277_dn5)) / assign4210_e4254)))),)
    } else {
        (locals.var_argp, locals.var_argp_dn1, locals.var_argp_dn3, locals.var_argp_dn4, locals.var_argp_dn5,)
    }
};
        locals.var_argp = assign4210_e4260;
        locals.var_argp_dn1 = assign4210_e4260_d_n1;
        locals.var_argp_dn3 = assign4210_e4260_d_n3;
        locals.var_argp_dn4 = assign4210_e4260_d_n4;
        locals.var_argp_dn5 = assign4210_e4260_d_n5;

        let (assign4220_e4267, assign4220_e4267_d_n1, assign4220_e4267_d_n3, assign4220_e4267_d_n4, assign4220_e4267_d_n5,) = {
    if ((locals.var_guard249 != 0.0) && (locals.var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_argp, locals.var_argp_dn1, locals.var_argp_dn3, locals.var_argp_dn4, locals.var_argp_dn5,)
    }
};
        locals.var_argp = assign4220_e4267;
        locals.var_argp_dn1 = assign4220_e4267_d_n1;
        locals.var_argp_dn3 = assign4220_e4267_d_n3;
        locals.var_argp_dn4 = assign4220_e4267_d_n4;
        locals.var_argp_dn5 = assign4220_e4267_d_n5;

        let (assign4230_e4277, assign4230_e4277_d_n1, assign4230_e4277_d_n3, assign4230_e4277_d_n4, assign4230_e4277_d_n5,) = {
    if (locals.var_guard249 != 0.0) {
        let assign4230_e4271: f64 = (locals.var_acja * locals.var_arga);
        let assign4230_e4274: f64 = (locals.var_pcjp * locals.var_argp);
        let assign4230_e4275: f64 = (assign4230_e4271 + assign4230_e4274);
        (assign4230_e4275, (((locals.var_acja_dn1 * locals.var_arga) + (locals.var_acja * locals.var_arga_dn1)) + (locals.var_pcjp * locals.var_argp_dn1)), (((locals.var_acja_dn3 * locals.var_arga) + (locals.var_acja * locals.var_arga_dn3)) + ((locals.var_pcjp_dn3 * locals.var_argp) + (locals.var_pcjp * locals.var_argp_dn3))), (((locals.var_acja_dn4 * locals.var_arga) + (locals.var_acja * locals.var_arga_dn4)) + (locals.var_pcjp * locals.var_argp_dn4)), (((locals.var_acja_dn5 * locals.var_arga) + (locals.var_acja * locals.var_arga_dn5)) + (locals.var_pcjp * locals.var_argp_dn5)),)
    } else {
        (locals.var_qcp1, locals.var_qcp1_dn1, locals.var_qcp1_dn3, locals.var_qcp1_dn4, locals.var_qcp1_dn5,)
    }
};
        locals.var_qcp1 = assign4230_e4277;
        locals.var_qcp1_dn1 = assign4230_e4277_d_n1;
        locals.var_qcp1_dn3 = assign4230_e4277_d_n3;
        locals.var_qcp1_dn4 = assign4230_e4277_d_n4;
        locals.var_qcp1_dn5 = assign4230_e4277_d_n5;

        let (assign4240_e4282, assign4240_e4282_d_n1, assign4240_e4282_d_n3, assign4240_e4282_d_n4, assign4240_e4282_d_n5,) = {
    if (locals.var_guard249 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qcp1, locals.var_qcp1_dn1, locals.var_qcp1_dn3, locals.var_qcp1_dn4, locals.var_qcp1_dn5,)
    }
};
        locals.var_qcp1 = assign4240_e4282;
        locals.var_qcp1_dn1 = assign4240_e4282_d_n1;
        locals.var_qcp1_dn3 = assign4240_e4282_d_n3;
        locals.var_qcp1_dn4 = assign4240_e4282_d_n4;
        locals.var_qcp1_dn5 = assign4240_e4282_d_n5;

        let assign4250_e4285: f64 = if locals.var_cj2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign4250_e4285;

        let (assign4260_e4306, assign4260_e4306_d_n1, assign4260_e4306_d_n3, assign4260_e4306_d_n4, assign4260_e4306_d_n5,) = {
    if ((locals.var_guard280 != 0.0) && (p.p63 != 0.0)) {
        let assign4260_e4292: f64 = (locals.var_vc2 - locals.var_vpo);
        let assign4260_e4295: f64 = (locals.var_vc2 + locals.var_vpo);
        let assign4260_e4298: f64 = (locals.var_vc2 + locals.var_vpo);
        let assign4260_e4299: f64 = (assign4260_e4295 * assign4260_e4298);
        let assign4260_e4301: f64 = (assign4260_e4299 + 0.04);
        let assign4260_e4302: f64 = (assign4260_e4301).sqrt();
        let assign4260_e4303: f64 = (assign4260_e4292 + assign4260_e4302);
        let assign4260_e4304: f64 = (0.5 * assign4260_e4303);
        (assign4260_e4304, (0.5 * (locals.var_vc2_dn1 + (((locals.var_vc2_dn1 * assign4260_e4298) + (assign4260_e4295 * locals.var_vc2_dn1)) / (2.0 * assign4260_e4302)))), (0.5 * ((-locals.var_vpo_dn3) + (((locals.var_vpo_dn3 * assign4260_e4298) + (assign4260_e4295 * locals.var_vpo_dn3)) / (2.0 * assign4260_e4302)))), 0.0, (0.5 * (locals.var_vc2_dn5 + (((locals.var_vc2_dn5 * assign4260_e4298) + (assign4260_e4295 * locals.var_vc2_dn5)) / (2.0 * assign4260_e4302)))),)
    } else {
        (locals.var_vcl, locals.var_vcl_dn1, locals.var_vcl_dn3, locals.var_vcl_dn4, locals.var_vcl_dn5,)
    }
};
        locals.var_vcl = assign4260_e4306;
        locals.var_vcl_dn1 = assign4260_e4306_d_n1;
        locals.var_vcl_dn3 = assign4260_e4306_d_n3;
        locals.var_vcl_dn4 = assign4260_e4306_d_n4;
        locals.var_vcl_dn5 = assign4260_e4306_d_n5;

        let (assign4270_e4313, assign4270_e4313_d_n1, assign4270_e4313_d_n3, assign4270_e4313_d_n4, assign4270_e4313_d_n5,) = {
    if ((locals.var_guard280 != 0.0) && (p.p63 == 0.0)) {
        (locals.var_vc2, locals.var_vc2_dn1, 0.0, 0.0, locals.var_vc2_dn5,)
    } else {
        (locals.var_vcl, locals.var_vcl_dn1, locals.var_vcl_dn3, locals.var_vcl_dn4, locals.var_vcl_dn5,)
    }
};
        locals.var_vcl = assign4270_e4313;
        locals.var_vcl_dn1 = assign4270_e4313_d_n1;
        locals.var_vcl_dn3 = assign4270_e4313_d_n3;
        locals.var_vcl_dn4 = assign4270_e4313_d_n4;
        locals.var_vcl_dn5 = assign4270_e4313_d_n5;

        let (assign4280_e4319, assign4280_e4319_d_n1, assign4280_e4319_d_n3, assign4280_e4319_d_n4, assign4280_e4319_d_n5,) = {
    if (locals.var_guard280 != 0.0) {
        let assign4280_e4317: f64 = (locals.var_a2_um2 * locals.var_cja_t);
        (assign4280_e4317, (locals.var_a2_um2_dn1 * locals.var_cja_t), ((locals.var_a2_um2_dn3 * locals.var_cja_t) + (locals.var_a2_um2 * locals.var_cja_t_dn3)), (locals.var_a2_um2_dn4 * locals.var_cja_t), (locals.var_a2_um2_dn5 * locals.var_cja_t),)
    } else {
        (locals.var_acja__blk281, locals.var_acja__blk281_dn1, locals.var_acja__blk281_dn3, locals.var_acja__blk281_dn4, locals.var_acja__blk281_dn5,)
    }
};
        locals.var_acja__blk281 = assign4280_e4319;
        locals.var_acja__blk281_dn1 = assign4280_e4319_d_n1;
        locals.var_acja__blk281_dn3 = assign4280_e4319_d_n3;
        locals.var_acja__blk281_dn4 = assign4280_e4319_d_n4;
        locals.var_acja__blk281_dn5 = assign4280_e4319_d_n5;

        let (assign4290_e4325, assign4290_e4325_d_n3,) = {
    if (locals.var_guard280 != 0.0) {
        let assign4290_e4323: f64 = (locals.var_p2_um * locals.var_cjp_t);
        (assign4290_e4323, (locals.var_p2_um * locals.var_cjp_t_dn3),)
    } else {
        (locals.var_pcjp__blk282, locals.var_pcjp__blk282_dn3,)
    }
};
        locals.var_pcjp__blk282 = assign4290_e4325;
        locals.var_pcjp__blk282_dn3 = assign4290_e4325_d_n3;

        let assign4300_e4328: f64 = if locals.var_acja__blk281 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign4300_e4328;

        let (assign4310_e4337, assign4310_e4337_d_n3,) = {
    if ((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) {
        let assign4310_e4333: f64 = (-locals.var_pa_t);
        let assign4310_e4335: f64 = (assign4310_e4333 * p.p68);
        (assign4310_e4335, ((-locals.var_pa_t_dn3) * p.p68),)
    } else {
        (locals.var_dv0__blk286, locals.var_dv0__blk286_dn3,)
    }
};
        locals.var_dv0__blk286 = assign4310_e4337;
        locals.var_dv0__blk286_dn3 = assign4310_e4337_d_n3;

        let assign4320_e4340: f64 = if p.p75 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign4320_e4340;

        let (assign4330_e4350, assign4330_e4350_d_n1, assign4330_e4350_d_n3, assign4330_e4350_d_n4, assign4330_e4350_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) {
        let assign4330_e4348: f64 = (locals.var_vcl + locals.var_dv0__blk286);
        (assign4330_e4348, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk286_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dvh__blk287, locals.var_dvh__blk287_dn1, locals.var_dvh__blk287_dn3, locals.var_dvh__blk287_dn4, locals.var_dvh__blk287_dn5,)
    }
};
        locals.var_dvh__blk287 = assign4330_e4350;
        locals.var_dvh__blk287_dn1 = assign4330_e4350_d_n1;
        locals.var_dvh__blk287_dn3 = assign4330_e4350_d_n3;
        locals.var_dvh__blk287_dn4 = assign4330_e4350_d_n4;
        locals.var_dvh__blk287_dn5 = assign4330_e4350_d_n5;

        let assign4340_e4353: f64 = if locals.var_dvh__blk287 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign4340_e4353;

        let (assign4350_e4368,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 != 0.0)) {
        let assign4350_e4363: f64 = (1.0 - p.p68);
        let assign4350_e4365: f64 = (-p.p74);
        let assign4350_e4366: f64 = (assign4350_e4363).powf(assign4350_e4365);
        (assign4350_e4366,)
    } else {
        (locals.var_pwq__blk288,)
    }
};
        locals.var_pwq__blk288 = assign4350_e4368;

        let (assign4360_e4390, assign4360_e4390_d_n1, assign4360_e4390_d_n3, assign4360_e4390_d_n4, assign4360_e4390_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 != 0.0)) {
        let assign4360_e4381: f64 = (1.0 - p.p68);
        let assign4360_e4382: f64 = (locals.var_pwq__blk288 * assign4360_e4381);
        let assign4360_e4383: f64 = (1.0 - assign4360_e4382);
        let assign4360_e4384: f64 = (locals.var_pa_t * assign4360_e4383);
        let assign4360_e4387: f64 = (1.0 - p.p74);
        let assign4360_e4388: f64 = (assign4360_e4384 / assign4360_e4387);
        (assign4360_e4388, 0.0, ((locals.var_pa_t_dn3 * assign4360_e4383) / assign4360_e4387), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk289, locals.var_qlo__blk289_dn1, locals.var_qlo__blk289_dn3, locals.var_qlo__blk289_dn4, locals.var_qlo__blk289_dn5,)
    }
};
        locals.var_qlo__blk289 = assign4360_e4390;
        locals.var_qlo__blk289_dn1 = assign4360_e4390_d_n1;
        locals.var_qlo__blk289_dn3 = assign4360_e4390_d_n3;
        locals.var_qlo__blk289_dn4 = assign4360_e4390_d_n4;
        locals.var_qlo__blk289_dn5 = assign4360_e4390_d_n5;

        let (assign4370_e4416, assign4370_e4416_d_n1, assign4370_e4416_d_n3, assign4370_e4416_d_n4, assign4370_e4416_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 != 0.0)) {
        let assign4370_e4402: f64 = (0.5 * p.p74);
        let assign4370_e4404: f64 = (assign4370_e4402 * locals.var_dvh__blk287);
        let assign4370_e4408: f64 = (1.0 - p.p68);
        let assign4370_e4409: f64 = (locals.var_pa_t * assign4370_e4408);
        let assign4370_e4410: f64 = (assign4370_e4404 / assign4370_e4409);
        let assign4370_e4411: f64 = (1.0 + assign4370_e4410);
        let assign4370_e4412: f64 = (locals.var_dvh__blk287 * assign4370_e4411);
        let assign4370_e4414: f64 = (assign4370_e4412 * locals.var_pwq__blk288);
        (assign4370_e4414, (((locals.var_dvh__blk287_dn1 * assign4370_e4411) + (locals.var_dvh__blk287 * ((assign4370_e4402 * locals.var_dvh__blk287_dn1) / assign4370_e4409))) * locals.var_pwq__blk288), (((locals.var_dvh__blk287_dn3 * assign4370_e4411) + (locals.var_dvh__blk287 * ((((assign4370_e4402 * locals.var_dvh__blk287_dn3) * assign4370_e4409) - (assign4370_e4404 * (locals.var_pa_t_dn3 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * locals.var_pwq__blk288), (((locals.var_dvh__blk287_dn4 * assign4370_e4411) + (locals.var_dvh__blk287 * ((assign4370_e4402 * locals.var_dvh__blk287_dn4) / assign4370_e4409))) * locals.var_pwq__blk288), (((locals.var_dvh__blk287_dn5 * assign4370_e4411) + (locals.var_dvh__blk287 * ((assign4370_e4402 * locals.var_dvh__blk287_dn5) / assign4370_e4409))) * locals.var_pwq__blk288),)
    } else {
        (locals.var_qhi__blk290, locals.var_qhi__blk290_dn1, locals.var_qhi__blk290_dn3, locals.var_qhi__blk290_dn4, locals.var_qhi__blk290_dn5,)
    }
};
        locals.var_qhi__blk290 = assign4370_e4416;
        locals.var_qhi__blk290_dn1 = assign4370_e4416_d_n1;
        locals.var_qhi__blk290_dn3 = assign4370_e4416_d_n3;
        locals.var_qhi__blk290_dn4 = assign4370_e4416_d_n4;
        locals.var_qhi__blk290_dn5 = assign4370_e4416_d_n5;

        let (assign4380_e4443, assign4380_e4443_d_n1, assign4380_e4443_d_n3, assign4380_e4443_d_n4, assign4380_e4443_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 == 0.0)) {
        let assign4380_e4430: f64 = (locals.var_vcl / locals.var_pa_t);
        let assign4380_e4431: f64 = (1.0 - assign4380_e4430);
        let assign4380_e4434: f64 = (1.0 - p.p74);
        let assign4380_e4435: f64 = (assign4380_e4431).powf(assign4380_e4434);
        let assign4380_e4436: f64 = (1.0 - assign4380_e4435);
        let assign4380_e4437: f64 = (locals.var_pa_t * assign4380_e4436);
        let assign4380_e4440: f64 = (1.0 - p.p74);
        let assign4380_e4441: f64 = (assign4380_e4437 / assign4380_e4440);
        (assign4380_e4441, ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(locals.var_vcl_dn1 / locals.var_pa_t)))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(locals.var_vcl_dn1 / locals.var_pa_t)) / assign4380_e4431))) })) / assign4380_e4440), (((locals.var_pa_t_dn3 * assign4380_e4436) + (locals.var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((locals.var_vcl_dn3 * locals.var_pa_t) - (locals.var_vcl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((locals.var_vcl_dn3 * locals.var_pa_t) - (locals.var_vcl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(locals.var_vcl_dn4 / locals.var_pa_t)))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(locals.var_vcl_dn4 / locals.var_pa_t)) / assign4380_e4431))) })) / assign4380_e4440), ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(locals.var_vcl_dn5 / locals.var_pa_t)))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(locals.var_vcl_dn5 / locals.var_pa_t)) / assign4380_e4431))) })) / assign4380_e4440),)
    } else {
        (locals.var_qlo__blk289, locals.var_qlo__blk289_dn1, locals.var_qlo__blk289_dn3, locals.var_qlo__blk289_dn4, locals.var_qlo__blk289_dn5,)
    }
};
        locals.var_qlo__blk289 = assign4380_e4443;
        locals.var_qlo__blk289_dn1 = assign4380_e4443_d_n1;
        locals.var_qlo__blk289_dn3 = assign4380_e4443_d_n3;
        locals.var_qlo__blk289_dn4 = assign4380_e4443_d_n4;
        locals.var_qlo__blk289_dn5 = assign4380_e4443_d_n5;

        let (assign4390_e4454, assign4390_e4454_d_n1, assign4390_e4454_d_n3, assign4390_e4454_d_n4, assign4390_e4454_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk290, locals.var_qhi__blk290_dn1, locals.var_qhi__blk290_dn3, locals.var_qhi__blk290_dn4, locals.var_qhi__blk290_dn5,)
    }
};
        locals.var_qhi__blk290 = assign4390_e4454;
        locals.var_qhi__blk290_dn1 = assign4390_e4454_d_n1;
        locals.var_qhi__blk290_dn3 = assign4390_e4454_d_n3;
        locals.var_qhi__blk290_dn4 = assign4390_e4454_d_n4;
        locals.var_qhi__blk290_dn5 = assign4390_e4454_d_n5;

        let (assign4400_e4464, assign4400_e4464_d_n1, assign4400_e4464_d_n3, assign4400_e4464_d_n4, assign4400_e4464_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) {
        let assign4400_e4462: f64 = (locals.var_qlo__blk289 + locals.var_qhi__blk290);
        (assign4400_e4462, (locals.var_qlo__blk289_dn1 + locals.var_qhi__blk290_dn1), (locals.var_qlo__blk289_dn3 + locals.var_qhi__blk290_dn3), (locals.var_qlo__blk289_dn4 + locals.var_qhi__blk290_dn4), (locals.var_qlo__blk289_dn5 + locals.var_qhi__blk290_dn5),)
    } else {
        (locals.var_arga__blk283, locals.var_arga__blk283_dn1, locals.var_arga__blk283_dn3, locals.var_arga__blk283_dn4, locals.var_arga__blk283_dn5,)
    }
};
        locals.var_arga__blk283 = assign4400_e4464;
        locals.var_arga__blk283_dn1 = assign4400_e4464_d_n1;
        locals.var_arga__blk283_dn3 = assign4400_e4464_d_n3;
        locals.var_arga__blk283_dn4 = assign4400_e4464_d_n4;
        locals.var_arga__blk283_dn5 = assign4400_e4464_d_n5;

        let (assign4410_e4482, assign4410_e4482_d_n3,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4410_e4473: f64 = (locals.var_dv0__blk286 * locals.var_dv0__blk286);
        let assign4410_e4476: f64 = (4.0 * p.p75);
        let assign4410_e4478: f64 = (assign4410_e4476 * p.p75);
        let assign4410_e4479: f64 = (assign4410_e4473 + assign4410_e4478);
        let assign4410_e4480: f64 = (assign4410_e4479).sqrt();
        (assign4410_e4480, (((locals.var_dv0__blk286_dn3 * locals.var_dv0__blk286) + (locals.var_dv0__blk286 * locals.var_dv0__blk286_dn3)) / (2.0 * assign4410_e4480)),)
    } else {
        (locals.var_mv0__blk291, locals.var_mv0__blk291_dn3,)
    }
};
        locals.var_mv0__blk291 = assign4410_e4482;
        locals.var_mv0__blk291_dn3 = assign4410_e4482_d_n3;

        let (assign4420_e4496, assign4420_e4496_d_n3,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4420_e4490: f64 = (-0.5);
        let assign4420_e4493: f64 = (locals.var_dv0__blk286 + locals.var_mv0__blk291);
        let assign4420_e4494: f64 = (assign4420_e4490 * assign4420_e4493);
        (assign4420_e4494, (assign4420_e4490 * (locals.var_dv0__blk286_dn3 + locals.var_mv0__blk291_dn3)),)
    } else {
        (locals.var_vl0__blk292, locals.var_vl0__blk292_dn3,)
    }
};
        locals.var_vl0__blk292 = assign4420_e4496;
        locals.var_vl0__blk292_dn3 = assign4420_e4496_d_n3;

        let (assign4430_e4507, assign4430_e4507_d_n1, assign4430_e4507_d_n3, assign4430_e4507_d_n4, assign4430_e4507_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4430_e4505: f64 = (locals.var_vcl + locals.var_dv0__blk286);
        (assign4430_e4505, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk286_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dv__blk293, locals.var_dv__blk293_dn1, locals.var_dv__blk293_dn3, locals.var_dv__blk293_dn4, locals.var_dv__blk293_dn5,)
    }
};
        locals.var_dv__blk293 = assign4430_e4507;
        locals.var_dv__blk293_dn1 = assign4430_e4507_d_n1;
        locals.var_dv__blk293_dn3 = assign4430_e4507_d_n3;
        locals.var_dv__blk293_dn4 = assign4430_e4507_d_n4;
        locals.var_dv__blk293_dn5 = assign4430_e4507_d_n5;

        let (assign4440_e4525, assign4440_e4525_d_n1, assign4440_e4525_d_n3, assign4440_e4525_d_n4, assign4440_e4525_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4440_e4516: f64 = (locals.var_dv__blk293 * locals.var_dv__blk293);
        let assign4440_e4519: f64 = (4.0 * p.p75);
        let assign4440_e4521: f64 = (assign4440_e4519 * p.p75);
        let assign4440_e4522: f64 = (assign4440_e4516 + assign4440_e4521);
        let assign4440_e4523: f64 = (assign4440_e4522).sqrt();
        (assign4440_e4523, (((locals.var_dv__blk293_dn1 * locals.var_dv__blk293) + (locals.var_dv__blk293 * locals.var_dv__blk293_dn1)) / (2.0 * assign4440_e4523)), (((locals.var_dv__blk293_dn3 * locals.var_dv__blk293) + (locals.var_dv__blk293 * locals.var_dv__blk293_dn3)) / (2.0 * assign4440_e4523)), (((locals.var_dv__blk293_dn4 * locals.var_dv__blk293) + (locals.var_dv__blk293 * locals.var_dv__blk293_dn4)) / (2.0 * assign4440_e4523)), (((locals.var_dv__blk293_dn5 * locals.var_dv__blk293) + (locals.var_dv__blk293 * locals.var_dv__blk293_dn5)) / (2.0 * assign4440_e4523)),)
    } else {
        (locals.var_mv__blk294, locals.var_mv__blk294_dn1, locals.var_mv__blk294_dn3, locals.var_mv__blk294_dn4, locals.var_mv__blk294_dn5,)
    }
};
        locals.var_mv__blk294 = assign4440_e4525;
        locals.var_mv__blk294_dn1 = assign4440_e4525_d_n1;
        locals.var_mv__blk294_dn3 = assign4440_e4525_d_n3;
        locals.var_mv__blk294_dn4 = assign4440_e4525_d_n4;
        locals.var_mv__blk294_dn5 = assign4440_e4525_d_n5;

        let (assign4450_e4540, assign4450_e4540_d_n1, assign4450_e4540_d_n3, assign4450_e4540_d_n4, assign4450_e4540_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4450_e4535: f64 = (locals.var_dv__blk293 - locals.var_mv__blk294);
        let assign4450_e4536: f64 = (0.5 * assign4450_e4535);
        let assign4450_e4538: f64 = (assign4450_e4536 - locals.var_dv0__blk286);
        (assign4450_e4538, (0.5 * (locals.var_dv__blk293_dn1 - locals.var_mv__blk294_dn1)), ((0.5 * (locals.var_dv__blk293_dn3 - locals.var_mv__blk294_dn3)) - locals.var_dv0__blk286_dn3), (0.5 * (locals.var_dv__blk293_dn4 - locals.var_mv__blk294_dn4)), (0.5 * (locals.var_dv__blk293_dn5 - locals.var_mv__blk294_dn5)),)
    } else {
        (locals.var_vl__blk295, locals.var_vl__blk295_dn1, locals.var_vl__blk295_dn3, locals.var_vl__blk295_dn4, locals.var_vl__blk295_dn5,)
    }
};
        locals.var_vl__blk295 = assign4450_e4540;
        locals.var_vl__blk295_dn1 = assign4450_e4540_d_n1;
        locals.var_vl__blk295_dn3 = assign4450_e4540_d_n3;
        locals.var_vl__blk295_dn4 = assign4450_e4540_d_n4;
        locals.var_vl__blk295_dn5 = assign4450_e4540_d_n5;

        let (assign4460_e4564, assign4460_e4564_d_n1, assign4460_e4564_d_n3, assign4460_e4564_d_n4, assign4460_e4564_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4460_e4548: f64 = (-locals.var_pa_t);
        let assign4460_e4552: f64 = (locals.var_vl__blk295 / locals.var_pa_t);
        let assign4460_e4553: f64 = (1.0 - assign4460_e4552);
        let assign4460_e4556: f64 = (1.0 - p.p74);
        let assign4460_e4557: f64 = (assign4460_e4553).powf(assign4460_e4556);
        let assign4460_e4558: f64 = (assign4460_e4548 * assign4460_e4557);
        let assign4460_e4561: f64 = (1.0 - p.p74);
        let assign4460_e4562: f64 = (assign4460_e4558 / assign4460_e4561);
        (assign4460_e4562, ((assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(locals.var_vl__blk295_dn1 / locals.var_pa_t)))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(locals.var_vl__blk295_dn1 / locals.var_pa_t)) / assign4460_e4553))) }) / assign4460_e4561), ((((-locals.var_pa_t_dn3) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((locals.var_vl__blk295_dn3 * locals.var_pa_t) - (locals.var_vl__blk295 * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((locals.var_vl__blk295_dn3 * locals.var_pa_t) - (locals.var_vl__blk295 * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(locals.var_vl__blk295_dn4 / locals.var_pa_t)))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(locals.var_vl__blk295_dn4 / locals.var_pa_t)) / assign4460_e4553))) }) / assign4460_e4561), ((assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(locals.var_vl__blk295_dn5 / locals.var_pa_t)))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(locals.var_vl__blk295_dn5 / locals.var_pa_t)) / assign4460_e4553))) }) / assign4460_e4561),)
    } else {
        (locals.var_qlo__blk289, locals.var_qlo__blk289_dn1, locals.var_qlo__blk289_dn3, locals.var_qlo__blk289_dn4, locals.var_qlo__blk289_dn5,)
    }
};
        locals.var_qlo__blk289 = assign4460_e4564;
        locals.var_qlo__blk289_dn1 = assign4460_e4564_d_n1;
        locals.var_qlo__blk289_dn3 = assign4460_e4564_d_n3;
        locals.var_qlo__blk289_dn4 = assign4460_e4564_d_n4;
        locals.var_qlo__blk289_dn5 = assign4460_e4564_d_n5;

        let (assign4470_e4604, assign4470_e4604_d_n1, assign4470_e4604_d_n3, assign4470_e4604_d_n4, assign4470_e4604_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4470_e4574: f64 = (1.0 - p.p68);
        let assign4470_e4576: f64 = (-p.p74);
        let assign4470_e4577: f64 = (assign4470_e4574).powf(assign4470_e4576);
        let assign4470_e4580: f64 = (locals.var_vcl - locals.var_vl__blk295);
        let assign4470_e4582: f64 = (assign4470_e4580 + locals.var_vl0__blk292);
        let assign4470_e4583: f64 = (assign4470_e4577 * assign4470_e4582);
        let assign4470_e4587: f64 = (0.5 * p.p74);
        let assign4470_e4590: f64 = (locals.var_vcl - locals.var_vl__blk295);
        let assign4470_e4592: f64 = (assign4470_e4590 + locals.var_vl0__blk292);
        let assign4470_e4593: f64 = (assign4470_e4587 * assign4470_e4592);
        let assign4470_e4597: f64 = (1.0 - p.p68);
        let assign4470_e4598: f64 = (locals.var_pa_t * assign4470_e4597);
        let assign4470_e4599: f64 = (assign4470_e4593 / assign4470_e4598);
        let assign4470_e4600: f64 = (1.0 + assign4470_e4599);
        let assign4470_e4601: f64 = (assign4470_e4583 * assign4470_e4600);
        let assign4470_e4602: f64 = (locals.var_qlo__blk289 + assign4470_e4601);
        (assign4470_e4602, (locals.var_qlo__blk289_dn1 + (((assign4470_e4577 * (locals.var_vcl_dn1 - locals.var_vl__blk295_dn1)) * assign4470_e4600) + (assign4470_e4583 * ((assign4470_e4587 * (locals.var_vcl_dn1 - locals.var_vl__blk295_dn1)) / assign4470_e4598)))), (locals.var_qlo__blk289_dn3 + (((assign4470_e4577 * ((locals.var_vcl_dn3 - locals.var_vl__blk295_dn3) + locals.var_vl0__blk292_dn3)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((locals.var_vcl_dn3 - locals.var_vl__blk295_dn3) + locals.var_vl0__blk292_dn3)) * assign4470_e4598) - (assign4470_e4593 * (locals.var_pa_t_dn3 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (locals.var_qlo__blk289_dn4 + (((assign4470_e4577 * (locals.var_vcl_dn4 - locals.var_vl__blk295_dn4)) * assign4470_e4600) + (assign4470_e4583 * ((assign4470_e4587 * (locals.var_vcl_dn4 - locals.var_vl__blk295_dn4)) / assign4470_e4598)))), (locals.var_qlo__blk289_dn5 + (((assign4470_e4577 * (locals.var_vcl_dn5 - locals.var_vl__blk295_dn5)) * assign4470_e4600) + (assign4470_e4583 * ((assign4470_e4587 * (locals.var_vcl_dn5 - locals.var_vl__blk295_dn5)) / assign4470_e4598)))),)
    } else {
        (locals.var_arga__blk283, locals.var_arga__blk283_dn1, locals.var_arga__blk283_dn3, locals.var_arga__blk283_dn4, locals.var_arga__blk283_dn5,)
    }
};
        locals.var_arga__blk283 = assign4470_e4604;
        locals.var_arga__blk283_dn1 = assign4470_e4604_d_n1;
        locals.var_arga__blk283_dn3 = assign4470_e4604_d_n3;
        locals.var_arga__blk283_dn4 = assign4470_e4604_d_n4;
        locals.var_arga__blk283_dn5 = assign4470_e4604_d_n5;

        let (assign4480_e4611, assign4480_e4611_d_n1, assign4480_e4611_d_n3, assign4480_e4611_d_n4, assign4480_e4611_d_n5,) = {
    if ((locals.var_guard280 != 0.0) && (locals.var_guard285 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arga__blk283, locals.var_arga__blk283_dn1, locals.var_arga__blk283_dn3, locals.var_arga__blk283_dn4, locals.var_arga__blk283_dn5,)
    }
};
        locals.var_arga__blk283 = assign4480_e4611;
        locals.var_arga__blk283_dn1 = assign4480_e4611_d_n1;
        locals.var_arga__blk283_dn3 = assign4480_e4611_d_n3;
        locals.var_arga__blk283_dn4 = assign4480_e4611_d_n4;
        locals.var_arga__blk283_dn5 = assign4480_e4611_d_n5;

        let assign4490_e4614: f64 = if locals.var_pcjp__blk282 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign4490_e4614;

        let (assign4500_e4623, assign4500_e4623_d_n3,) = {
    if ((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign4500_e4619: f64 = (-locals.var_pp_t);
        let assign4500_e4621: f64 = (assign4500_e4619 * p.p68);
        (assign4500_e4621, ((-locals.var_pp_t_dn3) * p.p68),)
    } else {
        (locals.var_dv0__blk299, locals.var_dv0__blk299_dn3,)
    }
};
        locals.var_dv0__blk299 = assign4500_e4623;
        locals.var_dv0__blk299_dn3 = assign4500_e4623_d_n3;

        let assign4510_e4626: f64 = if p.p82 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign4510_e4626;

        let (assign4520_e4636, assign4520_e4636_d_n1, assign4520_e4636_d_n3, assign4520_e4636_d_n4, assign4520_e4636_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign4520_e4634: f64 = (locals.var_vcl + locals.var_dv0__blk299);
        (assign4520_e4634, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk299_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dvh__blk300, locals.var_dvh__blk300_dn1, locals.var_dvh__blk300_dn3, locals.var_dvh__blk300_dn4, locals.var_dvh__blk300_dn5,)
    }
};
        locals.var_dvh__blk300 = assign4520_e4636;
        locals.var_dvh__blk300_dn1 = assign4520_e4636_d_n1;
        locals.var_dvh__blk300_dn3 = assign4520_e4636_d_n3;
        locals.var_dvh__blk300_dn4 = assign4520_e4636_d_n4;
        locals.var_dvh__blk300_dn5 = assign4520_e4636_d_n5;

        let assign4530_e4639: f64 = if locals.var_dvh__blk300 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign4530_e4639;

        let (assign4540_e4654,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign4540_e4649: f64 = (1.0 - p.p68);
        let assign4540_e4651: f64 = (-p.p81);
        let assign4540_e4652: f64 = (assign4540_e4649).powf(assign4540_e4651);
        (assign4540_e4652,)
    } else {
        (locals.var_pwq__blk301,)
    }
};
        locals.var_pwq__blk301 = assign4540_e4654;

        let (assign4550_e4676, assign4550_e4676_d_n1, assign4550_e4676_d_n3, assign4550_e4676_d_n4, assign4550_e4676_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign4550_e4667: f64 = (1.0 - p.p68);
        let assign4550_e4668: f64 = (locals.var_pwq__blk301 * assign4550_e4667);
        let assign4550_e4669: f64 = (1.0 - assign4550_e4668);
        let assign4550_e4670: f64 = (locals.var_pp_t * assign4550_e4669);
        let assign4550_e4673: f64 = (1.0 - p.p81);
        let assign4550_e4674: f64 = (assign4550_e4670 / assign4550_e4673);
        (assign4550_e4674, 0.0, ((locals.var_pp_t_dn3 * assign4550_e4669) / assign4550_e4673), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk302, locals.var_qlo__blk302_dn1, locals.var_qlo__blk302_dn3, locals.var_qlo__blk302_dn4, locals.var_qlo__blk302_dn5,)
    }
};
        locals.var_qlo__blk302 = assign4550_e4676;
        locals.var_qlo__blk302_dn1 = assign4550_e4676_d_n1;
        locals.var_qlo__blk302_dn3 = assign4550_e4676_d_n3;
        locals.var_qlo__blk302_dn4 = assign4550_e4676_d_n4;
        locals.var_qlo__blk302_dn5 = assign4550_e4676_d_n5;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4560_e4702, assign4560_e4702_d_n1, assign4560_e4702_d_n3, assign4560_e4702_d_n4, assign4560_e4702_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign4560_e4688: f64 = (0.5 * p.p81);
        let assign4560_e4690: f64 = (assign4560_e4688 * locals.var_dvh__blk300);
        let assign4560_e4694: f64 = (1.0 - p.p68);
        let assign4560_e4695: f64 = (locals.var_pp_t * assign4560_e4694);
        let assign4560_e4696: f64 = (assign4560_e4690 / assign4560_e4695);
        let assign4560_e4697: f64 = (1.0 + assign4560_e4696);
        let assign4560_e4698: f64 = (locals.var_dvh__blk300 * assign4560_e4697);
        let assign4560_e4700: f64 = (assign4560_e4698 * locals.var_pwq__blk301);
        (assign4560_e4700, (((locals.var_dvh__blk300_dn1 * assign4560_e4697) + (locals.var_dvh__blk300 * ((assign4560_e4688 * locals.var_dvh__blk300_dn1) / assign4560_e4695))) * locals.var_pwq__blk301), (((locals.var_dvh__blk300_dn3 * assign4560_e4697) + (locals.var_dvh__blk300 * ((((assign4560_e4688 * locals.var_dvh__blk300_dn3) * assign4560_e4695) - (assign4560_e4690 * (locals.var_pp_t_dn3 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * locals.var_pwq__blk301), (((locals.var_dvh__blk300_dn4 * assign4560_e4697) + (locals.var_dvh__blk300 * ((assign4560_e4688 * locals.var_dvh__blk300_dn4) / assign4560_e4695))) * locals.var_pwq__blk301), (((locals.var_dvh__blk300_dn5 * assign4560_e4697) + (locals.var_dvh__blk300 * ((assign4560_e4688 * locals.var_dvh__blk300_dn5) / assign4560_e4695))) * locals.var_pwq__blk301),)
    } else {
        (locals.var_qhi__blk303, locals.var_qhi__blk303_dn1, locals.var_qhi__blk303_dn3, locals.var_qhi__blk303_dn4, locals.var_qhi__blk303_dn5,)
    }
};
        locals.var_qhi__blk303 = assign4560_e4702;
        locals.var_qhi__blk303_dn1 = assign4560_e4702_d_n1;
        locals.var_qhi__blk303_dn3 = assign4560_e4702_d_n3;
        locals.var_qhi__blk303_dn4 = assign4560_e4702_d_n4;
        locals.var_qhi__blk303_dn5 = assign4560_e4702_d_n5;

        let (assign4570_e4729, assign4570_e4729_d_n1, assign4570_e4729_d_n3, assign4570_e4729_d_n4, assign4570_e4729_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 == 0.0)) {
        let assign4570_e4716: f64 = (locals.var_vcl / locals.var_pp_t);
        let assign4570_e4717: f64 = (1.0 - assign4570_e4716);
        let assign4570_e4720: f64 = (1.0 - p.p81);
        let assign4570_e4721: f64 = (assign4570_e4717).powf(assign4570_e4720);
        let assign4570_e4722: f64 = (1.0 - assign4570_e4721);
        let assign4570_e4723: f64 = (locals.var_pp_t * assign4570_e4722);
        let assign4570_e4726: f64 = (1.0 - p.p81);
        let assign4570_e4727: f64 = (assign4570_e4723 / assign4570_e4726);
        (assign4570_e4727, ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(locals.var_vcl_dn1 / locals.var_pp_t)))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(locals.var_vcl_dn1 / locals.var_pp_t)) / assign4570_e4717))) })) / assign4570_e4726), (((locals.var_pp_t_dn3 * assign4570_e4722) + (locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((locals.var_vcl_dn3 * locals.var_pp_t) - (locals.var_vcl * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((locals.var_vcl_dn3 * locals.var_pp_t) - (locals.var_vcl * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(locals.var_vcl_dn4 / locals.var_pp_t)))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(locals.var_vcl_dn4 / locals.var_pp_t)) / assign4570_e4717))) })) / assign4570_e4726), ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(locals.var_vcl_dn5 / locals.var_pp_t)))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(locals.var_vcl_dn5 / locals.var_pp_t)) / assign4570_e4717))) })) / assign4570_e4726),)
    } else {
        (locals.var_qlo__blk302, locals.var_qlo__blk302_dn1, locals.var_qlo__blk302_dn3, locals.var_qlo__blk302_dn4, locals.var_qlo__blk302_dn5,)
    }
};
        locals.var_qlo__blk302 = assign4570_e4729;
        locals.var_qlo__blk302_dn1 = assign4570_e4729_d_n1;
        locals.var_qlo__blk302_dn3 = assign4570_e4729_d_n3;
        locals.var_qlo__blk302_dn4 = assign4570_e4729_d_n4;
        locals.var_qlo__blk302_dn5 = assign4570_e4729_d_n5;

        let (assign4580_e4740, assign4580_e4740_d_n1, assign4580_e4740_d_n3, assign4580_e4740_d_n4, assign4580_e4740_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk303, locals.var_qhi__blk303_dn1, locals.var_qhi__blk303_dn3, locals.var_qhi__blk303_dn4, locals.var_qhi__blk303_dn5,)
    }
};
        locals.var_qhi__blk303 = assign4580_e4740;
        locals.var_qhi__blk303_dn1 = assign4580_e4740_d_n1;
        locals.var_qhi__blk303_dn3 = assign4580_e4740_d_n3;
        locals.var_qhi__blk303_dn4 = assign4580_e4740_d_n4;
        locals.var_qhi__blk303_dn5 = assign4580_e4740_d_n5;

        let (assign4590_e4750, assign4590_e4750_d_n1, assign4590_e4750_d_n3, assign4590_e4750_d_n4, assign4590_e4750_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign4590_e4748: f64 = (locals.var_qlo__blk302 + locals.var_qhi__blk303);
        (assign4590_e4748, (locals.var_qlo__blk302_dn1 + locals.var_qhi__blk303_dn1), (locals.var_qlo__blk302_dn3 + locals.var_qhi__blk303_dn3), (locals.var_qlo__blk302_dn4 + locals.var_qhi__blk303_dn4), (locals.var_qlo__blk302_dn5 + locals.var_qhi__blk303_dn5),)
    } else {
        (locals.var_argp__blk284, locals.var_argp__blk284_dn1, locals.var_argp__blk284_dn3, locals.var_argp__blk284_dn4, locals.var_argp__blk284_dn5,)
    }
};
        locals.var_argp__blk284 = assign4590_e4750;
        locals.var_argp__blk284_dn1 = assign4590_e4750_d_n1;
        locals.var_argp__blk284_dn3 = assign4590_e4750_d_n3;
        locals.var_argp__blk284_dn4 = assign4590_e4750_d_n4;
        locals.var_argp__blk284_dn5 = assign4590_e4750_d_n5;

        let (assign4600_e4768, assign4600_e4768_d_n3,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4600_e4759: f64 = (locals.var_dv0__blk299 * locals.var_dv0__blk299);
        let assign4600_e4762: f64 = (4.0 * p.p82);
        let assign4600_e4764: f64 = (assign4600_e4762 * p.p82);
        let assign4600_e4765: f64 = (assign4600_e4759 + assign4600_e4764);
        let assign4600_e4766: f64 = (assign4600_e4765).sqrt();
        (assign4600_e4766, (((locals.var_dv0__blk299_dn3 * locals.var_dv0__blk299) + (locals.var_dv0__blk299 * locals.var_dv0__blk299_dn3)) / (2.0 * assign4600_e4766)),)
    } else {
        (locals.var_mv0__blk304, locals.var_mv0__blk304_dn3,)
    }
};
        locals.var_mv0__blk304 = assign4600_e4768;
        locals.var_mv0__blk304_dn3 = assign4600_e4768_d_n3;

        let (assign4610_e4782, assign4610_e4782_d_n3,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4610_e4776: f64 = (-0.5);
        let assign4610_e4779: f64 = (locals.var_dv0__blk299 + locals.var_mv0__blk304);
        let assign4610_e4780: f64 = (assign4610_e4776 * assign4610_e4779);
        (assign4610_e4780, (assign4610_e4776 * (locals.var_dv0__blk299_dn3 + locals.var_mv0__blk304_dn3)),)
    } else {
        (locals.var_vl0__blk305, locals.var_vl0__blk305_dn3,)
    }
};
        locals.var_vl0__blk305 = assign4610_e4782;
        locals.var_vl0__blk305_dn3 = assign4610_e4782_d_n3;

        let (assign4620_e4793, assign4620_e4793_d_n1, assign4620_e4793_d_n3, assign4620_e4793_d_n4, assign4620_e4793_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4620_e4791: f64 = (locals.var_vcl + locals.var_dv0__blk299);
        (assign4620_e4791, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk299_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dv__blk306, locals.var_dv__blk306_dn1, locals.var_dv__blk306_dn3, locals.var_dv__blk306_dn4, locals.var_dv__blk306_dn5,)
    }
};
        locals.var_dv__blk306 = assign4620_e4793;
        locals.var_dv__blk306_dn1 = assign4620_e4793_d_n1;
        locals.var_dv__blk306_dn3 = assign4620_e4793_d_n3;
        locals.var_dv__blk306_dn4 = assign4620_e4793_d_n4;
        locals.var_dv__blk306_dn5 = assign4620_e4793_d_n5;

        let (assign4630_e4811, assign4630_e4811_d_n1, assign4630_e4811_d_n3, assign4630_e4811_d_n4, assign4630_e4811_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4630_e4802: f64 = (locals.var_dv__blk306 * locals.var_dv__blk306);
        let assign4630_e4805: f64 = (4.0 * p.p82);
        let assign4630_e4807: f64 = (assign4630_e4805 * p.p82);
        let assign4630_e4808: f64 = (assign4630_e4802 + assign4630_e4807);
        let assign4630_e4809: f64 = (assign4630_e4808).sqrt();
        (assign4630_e4809, (((locals.var_dv__blk306_dn1 * locals.var_dv__blk306) + (locals.var_dv__blk306 * locals.var_dv__blk306_dn1)) / (2.0 * assign4630_e4809)), (((locals.var_dv__blk306_dn3 * locals.var_dv__blk306) + (locals.var_dv__blk306 * locals.var_dv__blk306_dn3)) / (2.0 * assign4630_e4809)), (((locals.var_dv__blk306_dn4 * locals.var_dv__blk306) + (locals.var_dv__blk306 * locals.var_dv__blk306_dn4)) / (2.0 * assign4630_e4809)), (((locals.var_dv__blk306_dn5 * locals.var_dv__blk306) + (locals.var_dv__blk306 * locals.var_dv__blk306_dn5)) / (2.0 * assign4630_e4809)),)
    } else {
        (locals.var_mv__blk307, locals.var_mv__blk307_dn1, locals.var_mv__blk307_dn3, locals.var_mv__blk307_dn4, locals.var_mv__blk307_dn5,)
    }
};
        locals.var_mv__blk307 = assign4630_e4811;
        locals.var_mv__blk307_dn1 = assign4630_e4811_d_n1;
        locals.var_mv__blk307_dn3 = assign4630_e4811_d_n3;
        locals.var_mv__blk307_dn4 = assign4630_e4811_d_n4;
        locals.var_mv__blk307_dn5 = assign4630_e4811_d_n5;

        let (assign4640_e4826, assign4640_e4826_d_n1, assign4640_e4826_d_n3, assign4640_e4826_d_n4, assign4640_e4826_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4640_e4821: f64 = (locals.var_dv__blk306 - locals.var_mv__blk307);
        let assign4640_e4822: f64 = (0.5 * assign4640_e4821);
        let assign4640_e4824: f64 = (assign4640_e4822 - locals.var_dv0__blk299);
        (assign4640_e4824, (0.5 * (locals.var_dv__blk306_dn1 - locals.var_mv__blk307_dn1)), ((0.5 * (locals.var_dv__blk306_dn3 - locals.var_mv__blk307_dn3)) - locals.var_dv0__blk299_dn3), (0.5 * (locals.var_dv__blk306_dn4 - locals.var_mv__blk307_dn4)), (0.5 * (locals.var_dv__blk306_dn5 - locals.var_mv__blk307_dn5)),)
    } else {
        (locals.var_vl__blk308, locals.var_vl__blk308_dn1, locals.var_vl__blk308_dn3, locals.var_vl__blk308_dn4, locals.var_vl__blk308_dn5,)
    }
};
        locals.var_vl__blk308 = assign4640_e4826;
        locals.var_vl__blk308_dn1 = assign4640_e4826_d_n1;
        locals.var_vl__blk308_dn3 = assign4640_e4826_d_n3;
        locals.var_vl__blk308_dn4 = assign4640_e4826_d_n4;
        locals.var_vl__blk308_dn5 = assign4640_e4826_d_n5;

        let (assign4650_e4850, assign4650_e4850_d_n1, assign4650_e4850_d_n3, assign4650_e4850_d_n4, assign4650_e4850_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4650_e4834: f64 = (-locals.var_pp_t);
        let assign4650_e4838: f64 = (locals.var_vl__blk308 / locals.var_pp_t);
        let assign4650_e4839: f64 = (1.0 - assign4650_e4838);
        let assign4650_e4842: f64 = (1.0 - p.p81);
        let assign4650_e4843: f64 = (assign4650_e4839).powf(assign4650_e4842);
        let assign4650_e4844: f64 = (assign4650_e4834 * assign4650_e4843);
        let assign4650_e4847: f64 = (1.0 - p.p81);
        let assign4650_e4848: f64 = (assign4650_e4844 / assign4650_e4847);
        (assign4650_e4848, ((assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(locals.var_vl__blk308_dn1 / locals.var_pp_t)))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(locals.var_vl__blk308_dn1 / locals.var_pp_t)) / assign4650_e4839))) }) / assign4650_e4847), ((((-locals.var_pp_t_dn3) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((locals.var_vl__blk308_dn3 * locals.var_pp_t) - (locals.var_vl__blk308 * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((locals.var_vl__blk308_dn3 * locals.var_pp_t) - (locals.var_vl__blk308 * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(locals.var_vl__blk308_dn4 / locals.var_pp_t)))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(locals.var_vl__blk308_dn4 / locals.var_pp_t)) / assign4650_e4839))) }) / assign4650_e4847), ((assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(locals.var_vl__blk308_dn5 / locals.var_pp_t)))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(locals.var_vl__blk308_dn5 / locals.var_pp_t)) / assign4650_e4839))) }) / assign4650_e4847),)
    } else {
        (locals.var_qlo__blk302, locals.var_qlo__blk302_dn1, locals.var_qlo__blk302_dn3, locals.var_qlo__blk302_dn4, locals.var_qlo__blk302_dn5,)
    }
};
        locals.var_qlo__blk302 = assign4650_e4850;
        locals.var_qlo__blk302_dn1 = assign4650_e4850_d_n1;
        locals.var_qlo__blk302_dn3 = assign4650_e4850_d_n3;
        locals.var_qlo__blk302_dn4 = assign4650_e4850_d_n4;
        locals.var_qlo__blk302_dn5 = assign4650_e4850_d_n5;

        let (assign4660_e4890, assign4660_e4890_d_n1, assign4660_e4890_d_n3, assign4660_e4890_d_n4, assign4660_e4890_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 == 0.0)) {
        let assign4660_e4860: f64 = (1.0 - p.p68);
        let assign4660_e4862: f64 = (-p.p81);
        let assign4660_e4863: f64 = (assign4660_e4860).powf(assign4660_e4862);
        let assign4660_e4866: f64 = (locals.var_vcl - locals.var_vl__blk308);
        let assign4660_e4868: f64 = (assign4660_e4866 + locals.var_vl0__blk305);
        let assign4660_e4869: f64 = (assign4660_e4863 * assign4660_e4868);
        let assign4660_e4873: f64 = (0.5 * p.p81);
        let assign4660_e4876: f64 = (locals.var_vcl - locals.var_vl__blk308);
        let assign4660_e4878: f64 = (assign4660_e4876 + locals.var_vl0__blk305);
        let assign4660_e4879: f64 = (assign4660_e4873 * assign4660_e4878);
        let assign4660_e4883: f64 = (1.0 - p.p68);
        let assign4660_e4884: f64 = (locals.var_pp_t * assign4660_e4883);
        let assign4660_e4885: f64 = (assign4660_e4879 / assign4660_e4884);
        let assign4660_e4886: f64 = (1.0 + assign4660_e4885);
        let assign4660_e4887: f64 = (assign4660_e4869 * assign4660_e4886);
        let assign4660_e4888: f64 = (locals.var_qlo__blk302 + assign4660_e4887);
        (assign4660_e4888, (locals.var_qlo__blk302_dn1 + (((assign4660_e4863 * (locals.var_vcl_dn1 - locals.var_vl__blk308_dn1)) * assign4660_e4886) + (assign4660_e4869 * ((assign4660_e4873 * (locals.var_vcl_dn1 - locals.var_vl__blk308_dn1)) / assign4660_e4884)))), (locals.var_qlo__blk302_dn3 + (((assign4660_e4863 * ((locals.var_vcl_dn3 - locals.var_vl__blk308_dn3) + locals.var_vl0__blk305_dn3)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((locals.var_vcl_dn3 - locals.var_vl__blk308_dn3) + locals.var_vl0__blk305_dn3)) * assign4660_e4884) - (assign4660_e4879 * (locals.var_pp_t_dn3 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (locals.var_qlo__blk302_dn4 + (((assign4660_e4863 * (locals.var_vcl_dn4 - locals.var_vl__blk308_dn4)) * assign4660_e4886) + (assign4660_e4869 * ((assign4660_e4873 * (locals.var_vcl_dn4 - locals.var_vl__blk308_dn4)) / assign4660_e4884)))), (locals.var_qlo__blk302_dn5 + (((assign4660_e4863 * (locals.var_vcl_dn5 - locals.var_vl__blk308_dn5)) * assign4660_e4886) + (assign4660_e4869 * ((assign4660_e4873 * (locals.var_vcl_dn5 - locals.var_vl__blk308_dn5)) / assign4660_e4884)))),)
    } else {
        (locals.var_argp__blk284, locals.var_argp__blk284_dn1, locals.var_argp__blk284_dn3, locals.var_argp__blk284_dn4, locals.var_argp__blk284_dn5,)
    }
};
        locals.var_argp__blk284 = assign4660_e4890;
        locals.var_argp__blk284_dn1 = assign4660_e4890_d_n1;
        locals.var_argp__blk284_dn3 = assign4660_e4890_d_n3;
        locals.var_argp__blk284_dn4 = assign4660_e4890_d_n4;
        locals.var_argp__blk284_dn5 = assign4660_e4890_d_n5;

        let (assign4670_e4897, assign4670_e4897_d_n1, assign4670_e4897_d_n3, assign4670_e4897_d_n4, assign4670_e4897_d_n5,) = {
    if ((locals.var_guard280 != 0.0) && (locals.var_guard298 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_argp__blk284, locals.var_argp__blk284_dn1, locals.var_argp__blk284_dn3, locals.var_argp__blk284_dn4, locals.var_argp__blk284_dn5,)
    }
};
        locals.var_argp__blk284 = assign4670_e4897;
        locals.var_argp__blk284_dn1 = assign4670_e4897_d_n1;
        locals.var_argp__blk284_dn3 = assign4670_e4897_d_n3;
        locals.var_argp__blk284_dn4 = assign4670_e4897_d_n4;
        locals.var_argp__blk284_dn5 = assign4670_e4897_d_n5;

        let (assign4680_e4907, assign4680_e4907_d_n1, assign4680_e4907_d_n3, assign4680_e4907_d_n4, assign4680_e4907_d_n5,) = {
    if (locals.var_guard280 != 0.0) {
        let assign4680_e4901: f64 = (locals.var_acja__blk281 * locals.var_arga__blk283);
        let assign4680_e4904: f64 = (locals.var_pcjp__blk282 * locals.var_argp__blk284);
        let assign4680_e4905: f64 = (assign4680_e4901 + assign4680_e4904);
        (assign4680_e4905, (((locals.var_acja__blk281_dn1 * locals.var_arga__blk283) + (locals.var_acja__blk281 * locals.var_arga__blk283_dn1)) + (locals.var_pcjp__blk282 * locals.var_argp__blk284_dn1)), (((locals.var_acja__blk281_dn3 * locals.var_arga__blk283) + (locals.var_acja__blk281 * locals.var_arga__blk283_dn3)) + ((locals.var_pcjp__blk282_dn3 * locals.var_argp__blk284) + (locals.var_pcjp__blk282 * locals.var_argp__blk284_dn3))), (((locals.var_acja__blk281_dn4 * locals.var_arga__blk283) + (locals.var_acja__blk281 * locals.var_arga__blk283_dn4)) + (locals.var_pcjp__blk282 * locals.var_argp__blk284_dn4)), (((locals.var_acja__blk281_dn5 * locals.var_arga__blk283) + (locals.var_acja__blk281 * locals.var_arga__blk283_dn5)) + (locals.var_pcjp__blk282 * locals.var_argp__blk284_dn5)),)
    } else {
        (locals.var_qcp2, locals.var_qcp2_dn1, locals.var_qcp2_dn3, locals.var_qcp2_dn4, locals.var_qcp2_dn5,)
    }
};
        locals.var_qcp2 = assign4680_e4907;
        locals.var_qcp2_dn1 = assign4680_e4907_d_n1;
        locals.var_qcp2_dn3 = assign4680_e4907_d_n3;
        locals.var_qcp2_dn4 = assign4680_e4907_d_n4;
        locals.var_qcp2_dn5 = assign4680_e4907_d_n5;

        let (assign4690_e4912, assign4690_e4912_d_n1, assign4690_e4912_d_n3, assign4690_e4912_d_n4, assign4690_e4912_d_n5,) = {
    if (locals.var_guard280 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qcp2, locals.var_qcp2_dn1, locals.var_qcp2_dn3, locals.var_qcp2_dn4, locals.var_qcp2_dn5,)
    }
};
        locals.var_qcp2 = assign4690_e4912;
        locals.var_qcp2_dn1 = assign4690_e4912_d_n1;
        locals.var_qcp2_dn3 = assign4690_e4912_d_n3;
        locals.var_qcp2_dn4 = assign4690_e4912_d_n4;
        locals.var_qcp2_dn5 = assign4690_e4912_d_n5;

        let assign4700_e4916: f64 = (locals.var_cf1 * locals.var_vc1);
        let assign4700_e4917: f64 = (locals.var_qcp1 + assign4700_e4916);
        locals.var_qcp1 = assign4700_e4917;
        locals.var_qcp1_dn1 = (locals.var_qcp1_dn1 + ((locals.var_cf1_dn1 * locals.var_vc1) + (locals.var_cf1 * locals.var_vc1_dn1)));
        locals.var_qcp1_dn3 = (locals.var_qcp1_dn3 + (locals.var_cf1_dn3 * locals.var_vc1));
        locals.var_qcp1_dn4 = (locals.var_qcp1_dn4 + ((locals.var_cf1_dn4 * locals.var_vc1) + (locals.var_cf1 * locals.var_vc1_dn4)));
        locals.var_qcp1_dn5 = (locals.var_qcp1_dn5 + (locals.var_cf1_dn5 * locals.var_vc1));

        let assign4710_e4921: f64 = (locals.var_cf2 * locals.var_vc2);
        let assign4710_e4922: f64 = (locals.var_qcp2 + assign4710_e4921);
        locals.var_qcp2 = assign4710_e4922;
        locals.var_qcp2_dn1 = (locals.var_qcp2_dn1 + ((locals.var_cf2_dn1 * locals.var_vc2) + (locals.var_cf2 * locals.var_vc2_dn1)));
        locals.var_qcp2_dn3 = (locals.var_qcp2_dn3 + (locals.var_cf2_dn3 * locals.var_vc2));
        locals.var_qcp2_dn4 = (locals.var_qcp2_dn4 + (locals.var_cf2_dn4 * locals.var_vc2));
        locals.var_qcp2_dn5 = (locals.var_qcp2_dn5 + ((locals.var_cf2_dn5 * locals.var_vc2) + (locals.var_cf2 * locals.var_vc2_dn5)));

        let assign4720_e4924: f64 = (-p.p21);
        let assign4720_e4926: f64 = (assign4720_e4924 * locals.var_qcp1);
        locals.var_qcp1 = assign4720_e4926;
        locals.var_qcp1_dn1 = (assign4720_e4924 * locals.var_qcp1_dn1);
        locals.var_qcp1_dn3 = (assign4720_e4924 * locals.var_qcp1_dn3);
        locals.var_qcp1_dn4 = (assign4720_e4924 * locals.var_qcp1_dn4);
        locals.var_qcp1_dn5 = (assign4720_e4924 * locals.var_qcp1_dn5);

        let assign4730_e4928: f64 = (-p.p21);
        let assign4730_e4930: f64 = (assign4730_e4928 * locals.var_qcp2);
        locals.var_qcp2 = assign4730_e4930;
        locals.var_qcp2_dn1 = (assign4730_e4928 * locals.var_qcp2_dn1);
        locals.var_qcp2_dn3 = (assign4730_e4928 * locals.var_qcp2_dn3);
        locals.var_qcp2_dn4 = (assign4730_e4928 * locals.var_qcp2_dn4);
        locals.var_qcp2_dn5 = (assign4730_e4928 * locals.var_qcp2_dn5);

        let assign4740_e4933: f64 = (locals.var_dt_et * locals.var_cth);
        locals.var_qcth = assign4740_e4933;
        locals.var_qcth_dn3 = (locals.var_dt_et_dn3 * locals.var_cth);

        let assign4750_e4936: f64 = (locals.var_rc1_tnom / locals.var_mmod);
        let assign4750_e4938: f64 = if assign4750_e4936 <= p.p26 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign4750_e4938;

        let assign4760_e4941: f64 = (locals.var_rc2_tnom / locals.var_mmod);
        let assign4760_e4943: f64 = if assign4760_e4941 <= p.p26 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign4760_e4943;

        let (assign4770_e4949,) = {
    if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
        (locals.var_leff_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign4770_e4949;

        let (assign4780_e4955,) = {
    if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
        (locals.var_weff_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign4780_e4955;

        let (assign4790_e4962,) = {
    if ((p.p13 != 0.0) && (p.p89 == 0.0)) {
        (locals.var_l_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign4790_e4962;

        let (assign4800_e4969,) = {
    if ((p.p13 != 0.0) && (p.p89 == 0.0)) {
        (locals.var_w_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign4800_e4969;

        locals.var_r0 = locals.var_r0;
        locals.var_r0_dn3 = locals.var_r0_dn3;

        locals.var_cth = locals.var_cth;

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        multiplicity: f64,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let assign30_e272: f64 = multiplicity;
        locals.var_mmod = assign30_e272;
        locals.var_mmod_rv = 0.0;

        let assign50_e279: f64 = (0.01 * p.p23);
        let assign50_e280: f64 = (1.0 - assign50_e279);
        let assign50_e282: f64 = (assign50_e280 * p.p22);
        let assign50_e284: f64 = (assign50_e282 * 1000000.0);
        locals.var_lfactor = assign50_e284;
        locals.var_lfactor_rv = 0.0;

        let assign60_e287: f64 = (locals.var_lfactor * locals.var_lfactor);
        locals.var_afactor = assign60_e287;
        locals.var_afactor_rv = 0.0;

        let assign70_e290: f64 = (273.15 + p.p28);
        locals.var_tinik = assign70_e290;
        locals.var_tinik_rv = 0.0;

        let assign90_e293: f64 = ctx_temp;
        let assign90_e295: f64 = (assign90_e293 + p.p9);
        let assign90_e297: f64 = (assign90_e295 - 273.15);
        locals.var_tdevc = assign90_e297;
        locals.var_tdevc_dn3 = 0.0;
        locals.var_tdevc_rv = 0.0;

        let assign120_e307: f64 = (p.p35 + 1.0);
        let assign120_e308: f64 = if locals.var_tdevc < assign120_e307 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign120_e308;
        locals.var_guard104_rv = 0.0;

        let (assign130_e319, assign130_e319_d_n3,) = {
    if (locals.var_guard104 != 0.0) {
        let assign130_e313: f64 = (locals.var_tdevc - p.p35);
        let assign130_e315: f64 = (assign130_e313 - 1.0);
        let assign130_e316: f64 = (assign130_e315).exp();
        let assign130_e317: f64 = (p.p35 + assign130_e316);
        (assign130_e317, (assign130_e316 * locals.var_tdevc_dn3),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign130_e319;
        locals.var_tdevc_dn3 = assign130_e319_d_n3;
        locals.var_tdevc_rv = 0.0;

        let assign140_e323: f64 = (p.p36 - 1.0);
        let assign140_e324: f64 = if locals.var_tdevc > assign140_e323 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign140_e324;
        locals.var_guard105_rv = 0.0;

        let (assign150_e338, assign150_e338_d_n3,) = {
    if ((locals.var_guard104 == 0.0) && (locals.var_guard105 != 0.0)) {
        let assign150_e332: f64 = (p.p36 - locals.var_tdevc);
        let assign150_e334: f64 = (assign150_e332 - 1.0);
        let assign150_e335: f64 = (assign150_e334).exp();
        let assign150_e336: f64 = (p.p36 - assign150_e335);
        (assign150_e336, (-(assign150_e335 * (-locals.var_tdevc_dn3))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign150_e338;
        locals.var_tdevc_dn3 = assign150_e338_d_n3;
        locals.var_tdevc_rv = 0.0;

        let (assign160_e346, assign160_e346_d_n3,) = {
    if ((locals.var_guard104 == 0.0) && (locals.var_guard105 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign160_e346;
        locals.var_tdevc_dn3 = assign160_e346_d_n3;
        locals.var_tdevc_rv = 0.0;

        let assign170_e349: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign170_e349;
        locals.var_tdevk_dn3 = locals.var_tdevc_dn3;
        locals.var_tdevk_rv = 0.0;

        let assign180_e352: f64 = (1.3806505e-23 * locals.var_tdevk);
        let assign180_e354: f64 = (assign180_e352 / 1.60217653e-19);
        locals.var_phi_t0 = assign180_e354;
        locals.var_phi_t0_dn3 = ((1.3806505e-23 * locals.var_tdevk_dn3) / 1.60217653e-19);
        locals.var_phi_t0_rv = 0.0;

        let assign190_e357: f64 = (locals.var_tdevk / locals.var_tinik);
        locals.var_rt = assign190_e357;
        locals.var_rt_dn3 = (locals.var_tdevk_dn3 / locals.var_tinik);
        locals.var_rt_rv = 0.0;

        let assign200_e360: f64 = (locals.var_tdevk - locals.var_tinik);
        locals.var_dt = assign200_e360;
        locals.var_dt_dn3 = locals.var_tdevk_dn3;
        locals.var_dt_rv = 0.0;

        let assign210_e363: f64 = (p.p0 * locals.var_lfactor);
        locals.var_w_um = assign210_e363;
        locals.var_w_um_rv = 0.0;

        let assign220_e366: f64 = (p.p1 * locals.var_lfactor);
        locals.var_l_um = assign220_e366;
        locals.var_l_um_rv = 0.0;

        let assign270_e381: f64 = (p.p2 * locals.var_lfactor);
        locals.var_wd_um = assign270_e381;
        locals.var_wd_um_rv = 0.0;

        let assign280_e384: f64 = (p.p3 * locals.var_afactor);
        locals.var_a1_um2 = assign280_e384;
        locals.var_a1_um2_dn1 = (locals.var_a1_dn1 * locals.var_afactor);
        locals.var_a1_um2_dn3 = (locals.var_a1_dn3 * locals.var_afactor);
        locals.var_a1_um2_dn4 = (locals.var_a1_dn4 * locals.var_afactor);
        locals.var_a1_um2_dn5 = (locals.var_a1_dn5 * locals.var_afactor);
        locals.var_a1_um2_rv = 0.0;

        let assign290_e387: f64 = (p.p4 * locals.var_lfactor);
        locals.var_p1_um = assign290_e387;
        locals.var_p1_um_rv = 0.0;

        let assign300_e390: f64 = (p.p6 * locals.var_afactor);
        locals.var_a2_um2 = assign300_e390;
        locals.var_a2_um2_dn1 = (locals.var_a2_dn1 * locals.var_afactor);
        locals.var_a2_um2_dn3 = (locals.var_a2_dn3 * locals.var_afactor);
        locals.var_a2_um2_dn4 = (locals.var_a2_dn4 * locals.var_afactor);
        locals.var_a2_um2_dn5 = (locals.var_a2_dn5 * locals.var_afactor);
        locals.var_a2_um2_rv = 0.0;

        let assign310_e393: f64 = (p.p7 * locals.var_lfactor);
        locals.var_p2_um = assign310_e393;
        locals.var_p2_um_rv = 0.0;

        let assign320_e396: f64 = (locals.var_l_um * locals.var_w_um);
        locals.var_a_um2 = assign320_e396;
        locals.var_a_um2_rv = 0.0;

        let assign330_e399: f64 = (2.0 * locals.var_l_um);
        let assign330_e402: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign330_e405: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign330_e406: f64 = (assign330_e402 + assign330_e405);
        let assign330_e408: f64 = (assign330_e406 * locals.var_w_um);
        let assign330_e409: f64 = (assign330_e399 + assign330_e408);
        locals.var_p_um = assign330_e409;
        locals.var_p_um_rv = 0.0;

        let assign340_e413: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign340_e416: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign340_e417: f64 = (assign340_e413 + assign340_e416);
        let assign340_e418: f64 = (0.5 * assign340_e417);
        let assign340_e422: f64 = (p.p44 / locals.var_w_um);
        let assign340_e423: f64 = (p.p43 + assign340_e422);
        let assign340_e424: f64 = (assign340_e418 * assign340_e423);
        locals.var_xleff = assign340_e424;
        locals.var_xleff_rv = 0.0;

        let assign350_e427: f64 = (locals.var_w_um + p.p38);
        let assign350_e430: f64 = (p.p39 / locals.var_w_um);
        let assign350_e431: f64 = (assign350_e427 + assign350_e430);
        let assign350_e435: f64 = (-locals.var_w_um);
        let assign350_e437: f64 = (assign350_e435 / p.p41);
        let assign350_e438: f64 = (assign350_e437).exp();
        let assign350_e439: f64 = (1.0 - assign350_e438);
        let assign350_e440: f64 = (p.p42 * assign350_e439);
        let assign350_e441: f64 = (assign350_e431 + assign350_e440);
        let assign350_e445: f64 = (p.p40 * locals.var_wd_um);
        let assign350_e447: f64 = (assign350_e445 / locals.var_a_um2);
        let assign350_e448: f64 = (1.0 - assign350_e447);
        let assign350_e449: f64 = (assign350_e441 / assign350_e448);
        locals.var_weff_um = assign350_e449;
        locals.var_weff_um_rv = 0.0;

        let assign360_e452: f64 = (locals.var_l_um + locals.var_xleff);
        locals.var_leff_um = assign360_e452;
        locals.var_leff_um_rv = 0.0;

        let (assign370_e456,) = {
    if (p.p127 != 0.0) {
        (locals.var_weff_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign370_e456;
        locals.var_wid_rv = 0.0;

        let (assign380_e460,) = {
    if (p.p127 != 0.0) {
        (locals.var_leff_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign380_e460;
        locals.var_len_rv = 0.0;

        let (assign390_e465,) = {
    if (p.p127 == 0.0) {
        (locals.var_w_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign390_e465;
        locals.var_wid_rv = 0.0;

        let (assign400_e470,) = {
    if (p.p127 == 0.0) {
        (locals.var_l_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign400_e470;
        locals.var_len_rv = 0.0;

        let (assign410_e487,) = {
    if (p.p16 != 0.0) {
        let assign410_e475: f64 = (p.p119 * p.p122);
        let assign410_e476: f64 = (locals.var_weff_um + assign410_e475);
        let assign410_e479: f64 = (p.p11 * p.p125);
        let assign410_e482: f64 = (locals.var_mmod * locals.var_len);
        let assign410_e483: f64 = (assign410_e482).sqrt();
        let assign410_e484: f64 = (assign410_e479 / assign410_e483);
        let assign410_e485: f64 = (assign410_e476 + assign410_e484);
        (assign410_e485,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign410_e487;
        locals.var_weff_um_rv = 0.0;

        let (assign420_e504,) = {
    if (p.p16 != 0.0) {
        let assign420_e492: f64 = (p.p120 * p.p123);
        let assign420_e493: f64 = (locals.var_leff_um + assign420_e492);
        let assign420_e496: f64 = (p.p12 * p.p126);
        let assign420_e499: f64 = (locals.var_mmod * locals.var_wid);
        let assign420_e500: f64 = (assign420_e499).sqrt();
        let assign420_e501: f64 = (assign420_e496 / assign420_e500);
        let assign420_e502: f64 = (assign420_e493 + assign420_e501);
        (assign420_e502,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign420_e504;
        locals.var_leff_um_rv = 0.0;

        let assign440_e535: f64 = if ((p.p119 != 0.0) && ((p.p125 > 0.0) || (p.p122 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign440_e535;
        locals.var_guard110_rv = 0.0;

        let (assign450_e547,) = {
    if ((p.p16 == 0.0) && (locals.var_guard110 != 0.0)) {
        let assign450_e543: f64 = (locals.var_mmod * locals.var_len);
        let assign450_e544: f64 = (assign450_e543).sqrt();
        let assign450_e545: f64 = (p.p125 / assign450_e544);
        (assign450_e545,)
    } else {
        (locals.var_fctr1,)
    }
};
        locals.var_fctr1 = assign450_e547;
        locals.var_fctr1_rv = 0.0;

        let (assign460_e565,) = {
    if ((p.p16 == 0.0) && (locals.var_guard110 != 0.0)) {
        let assign460_e556: f64 = (p.p122 * p.p122);
        let assign460_e559: f64 = (locals.var_fctr1 * locals.var_fctr1);
        let assign460_e560: f64 = (assign460_e556 + assign460_e559);
        let assign460_e561: f64 = (assign460_e560).sqrt();
        let assign460_e562: f64 = (p.p119 * assign460_e561);
        let assign460_e563: f64 = (locals.var_weff_um + assign460_e562);
        (assign460_e563,)
    } else {
        (locals.var_weff_um,)
    }
};
        locals.var_weff_um = assign460_e565;
        locals.var_weff_um_rv = 0.0;

        let assign470_e576: f64 = if ((p.p120 != 0.0) && ((p.p126 > 0.0) || (p.p123 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard111 = assign470_e576;
        locals.var_guard111_rv = 0.0;

        let (assign480_e588,) = {
    if ((p.p16 == 0.0) && (locals.var_guard111 != 0.0)) {
        let assign480_e584: f64 = (locals.var_mmod * locals.var_wid);
        let assign480_e585: f64 = (assign480_e584).sqrt();
        let assign480_e586: f64 = (p.p126 / assign480_e585);
        (assign480_e586,)
    } else {
        (locals.var_fctr1,)
    }
};
        locals.var_fctr1 = assign480_e588;
        locals.var_fctr1_rv = 0.0;

        let (assign490_e606,) = {
    if ((p.p16 == 0.0) && (locals.var_guard111 != 0.0)) {
        let assign490_e597: f64 = (p.p123 * p.p123);
        let assign490_e600: f64 = (locals.var_fctr1 * locals.var_fctr1);
        let assign490_e601: f64 = (assign490_e597 + assign490_e600);
        let assign490_e602: f64 = (assign490_e601).sqrt();
        let assign490_e603: f64 = (p.p120 * assign490_e602);
        let assign490_e604: f64 = (locals.var_leff_um + assign490_e603);
        (assign490_e604,)
    } else {
        (locals.var_leff_um,)
    }
};
        locals.var_leff_um = assign490_e606;
        locals.var_leff_um_rv = 0.0;

        let assign500_e617: f64 = if ((p.p118 != 0.0) && ((p.p124 > 0.0) || (p.p121 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard112 = assign500_e617;
        locals.var_guard112_rv = 0.0;

        let (assign510_e631,) = {
    if ((p.p16 == 0.0) && (locals.var_guard112 != 0.0)) {
        let assign510_e625: f64 = (locals.var_mmod * locals.var_len);
        let assign510_e627: f64 = (assign510_e625 * locals.var_wid);
        let assign510_e628: f64 = (assign510_e627).sqrt();
        let assign510_e629: f64 = (p.p124 / assign510_e628);
        (assign510_e629,)
    } else {
        (locals.var_fctr1,)
    }
};
        locals.var_fctr1 = assign510_e631;
        locals.var_fctr1_rv = 0.0;

        let assign560_e667: f64 = (locals.var_leff_um + p.p45);
        locals.var_leffe_um = assign560_e667;
        locals.var_leffe_um_rv = 0.0;

        let (assign580_e674,) = {
    if (p.p53 != 0.0) {
        (locals.var_weff_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign580_e674;
        locals.var_wid_rv = 0.0;

        let (assign590_e678,) = {
    if (p.p53 != 0.0) {
        (locals.var_leff_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign590_e678;
        locals.var_len_rv = 0.0;

        let (assign600_e683,) = {
    if (p.p53 == 0.0) {
        (locals.var_w_um,)
    } else {
        (locals.var_wid,)
    }
};
        locals.var_wid = assign600_e683;
        locals.var_wid_rv = 0.0;

        let (assign610_e688,) = {
    if (p.p53 == 0.0) {
        (locals.var_l_um,)
    } else {
        (locals.var_len,)
    }
};
        locals.var_len = assign610_e688;
        locals.var_len_rv = 0.0;

        let assign620_e692: f64 = (locals.var_wid).powf(p.p56);
        let assign620_e693: f64 = (1.0 / assign620_e692);
        locals.var_iw_dpwe = assign620_e693;
        locals.var_iw_dpwe_rv = 0.0;

        let assign630_e697: f64 = (locals.var_len).powf(p.p58);
        let assign630_e698: f64 = (1.0 / assign630_e697);
        locals.var_il_dple = assign630_e698;
        locals.var_il_dple_rv = 0.0;

        let assign640_e703: f64 = (p.p55 * locals.var_iw_dpwe);
        let assign640_e704: f64 = (1.0 + assign640_e703);
        let assign640_e705: f64 = (p.p54 * assign640_e704);
        let assign640_e709: f64 = (p.p57 * locals.var_il_dple);
        let assign640_e710: f64 = (1.0 + assign640_e709);
        let assign640_e711: f64 = (assign640_e705 * assign640_e710);
        let assign640_e715: f64 = (p.p59 * locals.var_iw_dpwe);
        let assign640_e717: f64 = (assign640_e715 * locals.var_il_dple);
        let assign640_e718: f64 = (1.0 + assign640_e717);
        let assign640_e719: f64 = (assign640_e711 * assign640_e718);
        let assign640_e725: f64 = (locals.var_dt * p.p104);
        let assign640_e726: f64 = (p.p103 + assign640_e725);
        let assign640_e727: f64 = (locals.var_dt * assign640_e726);
        let assign640_e728: f64 = (1.0 + assign640_e727);
        let assign640_e729: f64 = (assign640_e719 * assign640_e728);
        locals.var_dp_i = assign640_e729;
        locals.var_dp_i_dn3 = (assign640_e719 * ((locals.var_dt_dn3 * assign640_e726) + (locals.var_dt * (locals.var_dt_dn3 * p.p104))));
        locals.var_dp_i_rv = 0.0;

        let (assign650_e735, assign650_e735_d_n3,) = {
    if (locals.var_dp_i > 0.1) {
        (locals.var_dp_i, locals.var_dp_i_dn3,)
    } else {
        (0.1, 0.0,)
    }
};
        locals.var_dp_i = assign650_e735;
        locals.var_dp_i_dn3 = assign650_e735_d_n3;
        locals.var_dp_i_rv = 0.0;

        let assign660_e737: f64 = (locals.var_dp_i).sqrt();
        let assign660_e740: f64 = (locals.var_dp_i + 10000.0);
        let assign660_e741: f64 = (assign660_e737 / assign660_e740);
        locals.var_dfmin = assign660_e741;
        locals.var_dfmin_dn3 = ((((locals.var_dp_i_dn3 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * locals.var_dp_i_dn3)) / (assign660_e740 * assign660_e740));
        locals.var_dfmin_rv = 0.0;

        let (assign670_e759,) = {
    if (p.p15 != 0.0) {
        (0.0,)
    } else {
        let assign670_e747: f64 = (p.p50 * locals.var_len);
        let assign670_e750: f64 = (p.p51 * locals.var_wid);
        let assign670_e751: f64 = (assign670_e747 + assign670_e750);
        let assign670_e753: f64 = (assign670_e751 + p.p52);
        let assign670_e756: f64 = (locals.var_len * locals.var_wid);
        let assign670_e757: f64 = (assign670_e753 / assign670_e756);
        let assign670_e758: f64 = (p.p49 + assign670_e757);
        (assign670_e758,)
    }
};
        locals.var_df = assign670_e759;
        locals.var_df_rv = 0.0;

        let assign680_e762: f64 = if locals.var_df < locals.var_dfmin { 1.0 } else { 0.0 };
        locals.var_guard116 = assign680_e762;
        locals.var_guard116_rv = 0.0;

        let (assign690_e771,) = {
    if (locals.var_guard116 != 0.0) {
        let (assign690_e769,) = {
            if (locals.var_df > 0.0) {
                (locals.var_df,)
            } else {
                (0.0,)
            }
        };
        (assign690_e769,)
    } else {
        (locals.var_df,)
    }
};
        locals.var_df = assign690_e771;
        locals.var_df_rv = 0.0;

        let (assign700_e777, assign700_e777_d_n3,) = {
    if (locals.var_guard116 != 0.0) {
        let assign700_e775: f64 = (locals.var_dfmin * locals.var_dfmin);
        (assign700_e775, ((locals.var_dfmin_dn3 * locals.var_dfmin) + (locals.var_dfmin * locals.var_dfmin_dn3)),)
    } else {
        (locals.var_dfsq, locals.var_dfsq_dn3,)
    }
};
        locals.var_dfsq = assign700_e777;
        locals.var_dfsq_dn3 = assign700_e777_d_n3;
        locals.var_dfsq_rv = 0.0;

        let (assign710_e784, assign710_e784_d_n3,) = {
    if (locals.var_guard116 == 0.0) {
        let assign710_e782: f64 = (locals.var_df * locals.var_df);
        (assign710_e782, 0.0,)
    } else {
        (locals.var_dfsq, locals.var_dfsq_dn3,)
    }
};
        locals.var_dfsq = assign710_e784;
        locals.var_dfsq_dn3 = assign710_e784_d_n3;
        locals.var_dfsq_rv = 0.0;

        let assign720_e787: f64 = (0.5 / locals.var_dfsq);
        let assign720_e790: f64 = (locals.var_dp_i * 0.5);
        let assign720_e791: f64 = (assign720_e787 - assign720_e790);
        locals.var_vpo = assign720_e791;
        locals.var_vpo_dn3 = ((-((0.5 * locals.var_dfsq_dn3) / (locals.var_dfsq * locals.var_dfsq))) - (locals.var_dp_i_dn3 * 0.5));
        locals.var_vpo_rv = 0.0;

        let assign730_e794: f64 = if p.p63 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign730_e794;
        locals.var_guard117_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (assign740_e804, assign740_e804_d_n3,) = {
    if (locals.var_guard117 != 0.0) {
        let assign740_e799: f64 = (2.0 * p.p64);
        let assign740_e801: f64 = (assign740_e799 / locals.var_dfsq);
        let assign740_e802: f64 = (locals.var_vpo - assign740_e801);
        (assign740_e802, (locals.var_vpo_dn3 - (-((assign740_e799 * locals.var_dfsq_dn3) / (locals.var_dfsq * locals.var_dfsq)))),)
    } else {
        (locals.var_vpoe, locals.var_vpoe_dn3,)
    }
};
        locals.var_vpoe = assign740_e804;
        locals.var_vpoe_dn3 = assign740_e804_d_n3;
        locals.var_vpoe_rv = 0.0;

        let assign760_e817: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign760_e817;
        locals.var_guard118_rv = 0.0;

        let (assign770_e831, assign770_e831_d_n3,) = {
    if ((locals.var_guard117 == 0.0) && (locals.var_guard118 != 0.0)) {
        let assign770_e825: f64 = (2.0 * p.p64);
        let assign770_e827: f64 = (assign770_e825 / locals.var_dfsq);
        let assign770_e828: f64 = (assign770_e827).sqrt();
        let assign770_e829: f64 = (locals.var_vpo - assign770_e828);
        (assign770_e829, (locals.var_vpo_dn3 - ((-((assign770_e825 * locals.var_dfsq_dn3) / (locals.var_dfsq * locals.var_dfsq))) / (2.0 * assign770_e828))),)
    } else {
        (locals.var_vpoe, locals.var_vpoe_dn3,)
    }
};
        locals.var_vpoe = assign770_e831;
        locals.var_vpoe_dn3 = assign770_e831_d_n3;
        locals.var_vpoe_rv = 0.0;

        let (assign790_e846, assign790_e846_d_n3,) = {
    if ((locals.var_guard117 == 0.0) && (locals.var_guard118 == 0.0)) {
        (locals.var_vpo, locals.var_vpo_dn3,)
    } else {
        (locals.var_vpoe, locals.var_vpoe_dn3,)
    }
};
        locals.var_vpoe = assign790_e846;
        locals.var_vpoe_dn3 = assign790_e846_d_n3;
        locals.var_vpoe_rv = 0.0;

        let assign820_e864: f64 = if p.p63 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign820_e864;
        locals.var_guard119_rv = 0.0;

        let (assign830_e870, assign830_e870_d_n3,) = {
    if (locals.var_guard119 != 0.0) {
        let assign830_e868: f64 = (p.p46 * locals.var_phi_t0);
        (assign830_e868, (p.p46 * locals.var_phi_t0_dn3),)
    } else {
        (locals.var_nsteff, locals.var_nsteff_dn3,)
    }
};
        locals.var_nsteff = assign830_e870;
        locals.var_nsteff_dn3 = assign830_e870_d_n3;
        locals.var_nsteff_rv = 0.0;

        let assign850_e894: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign850_e894;
        locals.var_guard120_rv = 0.0;

        let (assign860_e905, assign860_e905_d_n3,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 != 0.0)) {
        let assign860_e901: f64 = (2.0 * p.p46);
        let assign860_e903: f64 = (assign860_e901 * locals.var_phi_t0);
        (assign860_e903, (assign860_e901 * locals.var_phi_t0_dn3),)
    } else {
        (locals.var_nsteff, locals.var_nsteff_dn3,)
    }
};
        locals.var_nsteff = assign860_e905;
        locals.var_nsteff_dn3 = assign860_e905_d_n3;
        locals.var_nsteff_rv = 0.0;

        let (assign880_e926, assign880_e926_d_n3,) = {
    if ((locals.var_guard119 == 0.0) && (locals.var_guard120 == 0.0)) {
        let assign880_e924: f64 = (p.p46 * locals.var_phi_t0);
        (assign880_e924, (p.p46 * locals.var_phi_t0_dn3),)
    } else {
        (locals.var_nsteff, locals.var_nsteff_dn3,)
    }
};
        locals.var_nsteff = assign880_e926;
        locals.var_nsteff_dn3 = assign880_e926_d_n3;
        locals.var_nsteff_rv = 0.0;

        let (assign990_e1007,) = {
    if (p.p15 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign990_e1007;
        locals.var_cth_rv = 0.0;

        let (assign1010_e1049,) = {
    if (p.p15 == 0.0) {
        let assign1010_e1036: f64 = (p.p115 * locals.var_p_um);
        let assign1010_e1037: f64 = (p.p114 + assign1010_e1036);
        let assign1010_e1040: f64 = (p.p116 * locals.var_a_um2);
        let assign1010_e1041: f64 = (assign1010_e1037 + assign1010_e1040);
        let assign1010_e1045: f64 = (p.p5 + p.p8);
        let assign1010_e1046: f64 = (p.p117 * assign1010_e1045);
        let assign1010_e1047: f64 = (assign1010_e1041 + assign1010_e1046);
        (assign1010_e1047,)
    } else {
        (locals.var_cth,)
    }
};
        locals.var_cth = assign1010_e1049;
        locals.var_cth_rv = 0.0;

        let assign1020_e1053: f64 = (p.p97 / locals.var_weff_um);
        let assign1020_e1054: f64 = (p.p93 + assign1020_e1053);
        let assign1020_e1058: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign1020_e1061: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign1020_e1062: f64 = (assign1020_e1058 + assign1020_e1061);
        let assign1020_e1063: f64 = (0.5 * assign1020_e1062);
        let assign1020_e1067: f64 = (p.p99 / locals.var_weff_um);
        let assign1020_e1068: f64 = (p.p95 + assign1020_e1067);
        let assign1020_e1069: f64 = (assign1020_e1063 * assign1020_e1068);
        let assign1020_e1071: f64 = (assign1020_e1069 / locals.var_leff_um);
        let assign1020_e1072: f64 = (assign1020_e1054 + assign1020_e1071);
        locals.var_tc1e = assign1020_e1072;
        locals.var_tc1e_rv = 0.0;

        let assign1030_e1076: f64 = (p.p98 / locals.var_weff_um);
        let assign1030_e1077: f64 = (p.p94 + assign1030_e1076);
        let assign1030_e1081: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign1030_e1084: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign1030_e1085: f64 = (assign1030_e1081 + assign1030_e1084);
        let assign1030_e1086: f64 = (0.5 * assign1030_e1085);
        let assign1030_e1090: f64 = (p.p100 / locals.var_weff_um);
        let assign1030_e1091: f64 = (p.p96 + assign1030_e1090);
        let assign1030_e1092: f64 = (assign1030_e1086 * assign1030_e1091);
        let assign1030_e1094: f64 = (assign1030_e1092 / locals.var_leff_um);
        let assign1030_e1095: f64 = (assign1030_e1077 + assign1030_e1094);
        locals.var_tc2e = assign1030_e1095;
        locals.var_tc2e_rv = 0.0;

        let assign1040_e1098: f64 = (p.p71 * locals.var_a1_um2);
        let assign1040_e1101: f64 = (p.p78 * locals.var_p1_um);
        let assign1040_e1102: f64 = (assign1040_e1098 + assign1040_e1101);
        locals.var_cf1 = assign1040_e1102;
        locals.var_cf1_dn1 = (p.p71 * locals.var_a1_um2_dn1);
        locals.var_cf1_dn3 = (p.p71 * locals.var_a1_um2_dn3);
        locals.var_cf1_dn4 = (p.p71 * locals.var_a1_um2_dn4);
        locals.var_cf1_dn5 = (p.p71 * locals.var_a1_um2_dn5);
        locals.var_cf1_rv = 0.0;

        let assign1050_e1105: f64 = (p.p71 * locals.var_a2_um2);
        let assign1050_e1108: f64 = (p.p78 * locals.var_p2_um);
        let assign1050_e1109: f64 = (assign1050_e1105 + assign1050_e1108);
        locals.var_cf2 = assign1050_e1109;
        locals.var_cf2_dn1 = (p.p71 * locals.var_a2_um2_dn1);
        locals.var_cf2_dn3 = (p.p71 * locals.var_a2_um2_dn3);
        locals.var_cf2_dn4 = (p.p71 * locals.var_a2_um2_dn4);
        locals.var_cf2_dn5 = (p.p71 * locals.var_a2_um2_dn5);
        locals.var_cf2_rv = 0.0;

        let assign1060_e1112: f64 = (p.p72 * locals.var_a1_um2);
        let assign1060_e1115: f64 = (p.p79 * locals.var_p1_um);
        let assign1060_e1116: f64 = (assign1060_e1112 + assign1060_e1115);
        locals.var_cj1 = assign1060_e1116;
        locals.var_cj1_dn1 = (p.p72 * locals.var_a1_um2_dn1);
        locals.var_cj1_dn3 = (p.p72 * locals.var_a1_um2_dn3);
        locals.var_cj1_dn4 = (p.p72 * locals.var_a1_um2_dn4);
        locals.var_cj1_dn5 = (p.p72 * locals.var_a1_um2_dn5);
        locals.var_cj1_rv = 0.0;

        let assign1070_e1119: f64 = (p.p72 * locals.var_a2_um2);
        let assign1070_e1122: f64 = (p.p79 * locals.var_p2_um);
        let assign1070_e1123: f64 = (assign1070_e1119 + assign1070_e1122);
        locals.var_cj2 = assign1070_e1123;
        locals.var_cj2_dn1 = (p.p72 * locals.var_a2_um2_dn1);
        locals.var_cj2_dn3 = (p.p72 * locals.var_a2_um2_dn3);
        locals.var_cj2_dn4 = (p.p72 * locals.var_a2_um2_dn4);
        locals.var_cj2_dn5 = (p.p72 * locals.var_a2_um2_dn5);
        locals.var_cj2_rv = 0.0;

        locals.var_dt_et = (nv3 - 0.0);
        locals.var_dt_et_dn3 = 1.0;
        locals.var_dt_et_rv = 0.0;

        let assign1090_e1126: f64 = (-p.p21);
        let assign1090_e1128: f64 = (assign1090_e1126 * (nv5 - nv4));
        locals.var_vrb = assign1090_e1128;
        locals.var_vrb_dn4 = (-assign1090_e1126);
        locals.var_vrb_dn5 = assign1090_e1126;
        locals.var_vrb_rv = 0.0;

        let assign1100_e1130: f64 = (-p.p21);
        let assign1100_e1132: f64 = (assign1100_e1130 * (nv1 - nv4));
        locals.var_vc1 = assign1100_e1132;
        locals.var_vc1_dn1 = assign1100_e1130;
        locals.var_vc1_dn4 = (-assign1100_e1130);
        locals.var_vc1_rv = 0.0;

        let assign1110_e1134: f64 = (-p.p21);
        let assign1110_e1136: f64 = (assign1110_e1134 * (nv1 - nv5));
        locals.var_vc2 = assign1110_e1136;
        locals.var_vc2_dn1 = assign1110_e1134;
        locals.var_vc2_dn5 = (-assign1110_e1134);
        locals.var_vc2_rv = 0.0;

        let assign1120_e1137: f64 = ctx_temp;
        let assign1120_e1139: f64 = (assign1120_e1137 + p.p9);
        let assign1120_e1141: f64 = (assign1120_e1139 + locals.var_dt_et);
        let assign1120_e1143: f64 = (assign1120_e1141 - 273.15);
        locals.var_tdevc = assign1120_e1143;
        locals.var_tdevc_dn3 = locals.var_dt_et_dn3;
        locals.var_tdevc_rv = 0.0;

        let assign1130_e1147: f64 = (p.p35 + 1.0);
        let assign1130_e1148: f64 = if locals.var_tdevc < assign1130_e1147 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign1130_e1148;
        locals.var_guard124_rv = 0.0;

        let (assign1140_e1159, assign1140_e1159_d_n3,) = {
    if (locals.var_guard124 != 0.0) {
        let assign1140_e1153: f64 = (locals.var_tdevc - p.p35);
        let assign1140_e1155: f64 = (assign1140_e1153 - 1.0);
        let assign1140_e1156: f64 = (assign1140_e1155).exp();
        let assign1140_e1157: f64 = (p.p35 + assign1140_e1156);
        (assign1140_e1157, (assign1140_e1156 * locals.var_tdevc_dn3),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign1140_e1159;
        locals.var_tdevc_dn3 = assign1140_e1159_d_n3;
        locals.var_tdevc_rv = 0.0;

        let assign1150_e1163: f64 = (p.p36 - 1.0);
        let assign1150_e1164: f64 = if locals.var_tdevc > assign1150_e1163 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign1150_e1164;
        locals.var_guard125_rv = 0.0;

        let (assign1160_e1178, assign1160_e1178_d_n3,) = {
    if ((locals.var_guard124 == 0.0) && (locals.var_guard125 != 0.0)) {
        let assign1160_e1172: f64 = (p.p36 - locals.var_tdevc);
        let assign1160_e1174: f64 = (assign1160_e1172 - 1.0);
        let assign1160_e1175: f64 = (assign1160_e1174).exp();
        let assign1160_e1176: f64 = (p.p36 - assign1160_e1175);
        (assign1160_e1176, (-(assign1160_e1175 * (-locals.var_tdevc_dn3))),)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign1160_e1178;
        locals.var_tdevc_dn3 = assign1160_e1178_d_n3;
        locals.var_tdevc_rv = 0.0;

        let (assign1170_e1186, assign1170_e1186_d_n3,) = {
    if ((locals.var_guard124 == 0.0) && (locals.var_guard125 == 0.0)) {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    } else {
        (locals.var_tdevc, locals.var_tdevc_dn3,)
    }
};
        locals.var_tdevc = assign1170_e1186;
        locals.var_tdevc_dn3 = assign1170_e1186_d_n3;
        locals.var_tdevc_rv = 0.0;

        let assign1180_e1189: f64 = (locals.var_tdevc + 273.15);
        locals.var_tdevk = assign1180_e1189;
        locals.var_tdevk_dn3 = locals.var_tdevc_dn3;
        locals.var_tdevk_rv = 0.0;

        let assign1190_e1192: f64 = (1.3806505e-23 * locals.var_tdevk);
        let assign1190_e1194: f64 = (assign1190_e1192 / 1.60217653e-19);
        locals.var_phi_t = assign1190_e1194;
        locals.var_phi_t_dn3 = ((1.3806505e-23 * locals.var_tdevk_dn3) / 1.60217653e-19);
        locals.var_phi_t_rv = 0.0;

        let assign1200_e1197: f64 = (locals.var_tdevk / locals.var_tinik);
        locals.var_rt = assign1200_e1197;
        locals.var_rt_dn3 = (locals.var_tdevk_dn3 / locals.var_tinik);
        locals.var_rt_rv = 0.0;

        let assign1210_e1200: f64 = (locals.var_tdevk - locals.var_tinik);
        locals.var_dt = assign1210_e1200;
        locals.var_dt_dn3 = locals.var_tdevk_dn3;
        locals.var_dt_rv = 0.0;

        let assign1220_e1206: f64 = (locals.var_dt * locals.var_tc2e);
        let assign1220_e1207: f64 = (locals.var_tc1e + assign1220_e1206);
        let assign1220_e1208: f64 = (locals.var_dt * assign1220_e1207);
        let assign1220_e1209: f64 = (1.0 + assign1220_e1208);
        locals.var_tcr = assign1220_e1209;
        locals.var_tcr_dn3 = ((locals.var_dt_dn3 * assign1220_e1207) + (locals.var_dt * (locals.var_dt_dn3 * locals.var_tc2e)));
        locals.var_tcr_rv = 0.0;

        let assign1230_e1213: f64 = (0.01 + 0.1);
        let assign1230_e1214: f64 = if locals.var_tcr < assign1230_e1213 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign1230_e1214;
        locals.var_guard126_rv = 0.0;

        let (assign1240_e1229, assign1240_e1229_d_n3,) = {
    if (locals.var_guard126 != 0.0) {
        let assign1240_e1221: f64 = (locals.var_tcr - 0.01);
        let assign1240_e1222: f64 = (10.0 * assign1240_e1221);
        let assign1240_e1224: f64 = (assign1240_e1222 - 1.0);
        let assign1240_e1225: f64 = (assign1240_e1224).exp();
        let assign1240_e1226: f64 = (0.1 * assign1240_e1225);
        let assign1240_e1227: f64 = (0.01 + assign1240_e1226);
        (assign1240_e1227, (0.1 * (assign1240_e1225 * (10.0 * locals.var_tcr_dn3))),)
    } else {
        (locals.var_tcr, locals.var_tcr_dn3,)
    }
};
        locals.var_tcr = assign1240_e1229;
        locals.var_tcr_dn3 = assign1240_e1229_d_n3;
        locals.var_tcr_rv = 0.0;

        let (assign1250_e1234, assign1250_e1234_d_n3,) = {
    if (locals.var_guard126 == 0.0) {
        (locals.var_tcr, locals.var_tcr_dn3,)
    } else {
        (locals.var_tcr, locals.var_tcr_dn3,)
    }
};
        locals.var_tcr = assign1250_e1234;
        locals.var_tcr_dn3 = assign1250_e1234_d_n3;
        locals.var_tcr_rv = 0.0;

        let assign1320_e1295: f64 = (locals.var_rt).powf(p.p92);
        locals.var_tcvsat = assign1320_e1295;
        locals.var_tcvsat_dn3 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((locals.var_rt).powf(p.p92 - 1.0) * locals.var_rt_dn3)) } } else { (assign1320_e1295 * (p.p92 * (locals.var_rt_dn3 / locals.var_rt))) };
        locals.var_tcvsat_rv = 0.0;

        let assign1450_e1406: f64 = if p.p72 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign1450_e1406;
        locals.var_guard130_rv = 0.0;

        let (assign1460_e1434, assign1460_e1434_d_n3,) = {
    if (locals.var_guard130 != 0.0) {
        let assign1460_e1411: f64 = (locals.var_phi_t / locals.var_rt);
        let assign1460_e1412: f64 = (2.0 * assign1460_e1411);
        let assign1460_e1415: f64 = (0.5 * p.p73);
        let assign1460_e1417: f64 = (assign1460_e1415 * locals.var_rt);
        let assign1460_e1419: f64 = (assign1460_e1417 / locals.var_phi_t);
        let assign1460_e1420: f64 = (assign1460_e1419).exp();
        let assign1460_e1422: f64 = (-0.5);
        let assign1460_e1424: f64 = (assign1460_e1422 * p.p73);
        let assign1460_e1426: f64 = (assign1460_e1424 * locals.var_rt);
        let assign1460_e1428: f64 = (assign1460_e1426 / locals.var_phi_t);
        let assign1460_e1429: f64 = (assign1460_e1428).exp();
        let assign1460_e1430: f64 = (assign1460_e1420 - assign1460_e1429);
        let assign1460_e1431: f64 = (assign1460_e1430).ln();
        let assign1460_e1432: f64 = (assign1460_e1412 * assign1460_e1431);
        (assign1460_e1432, (((2.0 * (((locals.var_phi_t_dn3 * locals.var_rt) - (locals.var_phi_t * locals.var_rt_dn3)) / (locals.var_rt * locals.var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * locals.var_rt_dn3) * locals.var_phi_t) - (assign1460_e1417 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * locals.var_rt_dn3) * locals.var_phi_t) - (assign1460_e1426 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)))) / assign1460_e1430))),)
    } else {
        (locals.var_psiio, locals.var_psiio_dn3,)
    }
};
        locals.var_psiio = assign1460_e1434;
        locals.var_psiio_dn3 = assign1460_e1434_d_n3;
        locals.var_psiio_rv = 0.0;

        let (assign1470_e1453, assign1470_e1453_d_n3,) = {
    if (locals.var_guard130 != 0.0) {
        let assign1470_e1438: f64 = (locals.var_psiio * locals.var_rt);
        let assign1470_e1441: f64 = (3.0 * locals.var_phi_t);
        let assign1470_e1443: f64 = (locals.var_rt).ln();
        let assign1470_e1444: f64 = (assign1470_e1441 * assign1470_e1443);
        let assign1470_e1445: f64 = (assign1470_e1438 - assign1470_e1444);
        let assign1470_e1449: f64 = (locals.var_rt - 1.0);
        let assign1470_e1450: f64 = (p.p90 * assign1470_e1449);
        let assign1470_e1451: f64 = (assign1470_e1445 - assign1470_e1450);
        (assign1470_e1451, ((((locals.var_psiio_dn3 * locals.var_rt) + (locals.var_psiio * locals.var_rt_dn3)) - (((3.0 * locals.var_phi_t_dn3) * assign1470_e1443) + (assign1470_e1441 * (locals.var_rt_dn3 / locals.var_rt)))) - (p.p90 * locals.var_rt_dn3)),)
    } else {
        (locals.var_psiin, locals.var_psiin_dn3,)
    }
};
        locals.var_psiin = assign1470_e1453;
        locals.var_psiin_dn3 = assign1470_e1453_d_n3;
        locals.var_psiin_rv = 0.0;

        let (assign1480_e1477, assign1480_e1477_d_n3,) = {
    if (locals.var_guard130 != 0.0) {
        let assign1480_e1458: f64 = (2.0 * locals.var_phi_t);
        let assign1480_e1464: f64 = (-locals.var_psiin);
        let assign1480_e1466: f64 = (assign1480_e1464 / locals.var_phi_t);
        let assign1480_e1467: f64 = (assign1480_e1466).exp();
        let assign1480_e1468: f64 = (4.0 * assign1480_e1467);
        let assign1480_e1469: f64 = (1.0 + assign1480_e1468);
        let assign1480_e1470: f64 = (assign1480_e1469).sqrt();
        let assign1480_e1471: f64 = (1.0 + assign1480_e1470);
        let assign1480_e1472: f64 = (0.5 * assign1480_e1471);
        let assign1480_e1473: f64 = (assign1480_e1472).ln();
        let assign1480_e1474: f64 = (assign1480_e1458 * assign1480_e1473);
        let assign1480_e1475: f64 = (locals.var_psiin + assign1480_e1474);
        (assign1480_e1475, (locals.var_psiin_dn3 + (((2.0 * locals.var_phi_t_dn3) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-locals.var_psiin_dn3) * locals.var_phi_t) - (assign1480_e1464 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))),)
    } else {
        (locals.var_pa_t, locals.var_pa_t_dn3,)
    }
};
        locals.var_pa_t = assign1480_e1477;
        locals.var_pa_t_dn3 = assign1480_e1477_d_n3;
        locals.var_pa_t_rv = 0.0;

        let (assign1490_e1487, assign1490_e1487_d_n3,) = {
    if (locals.var_guard130 != 0.0) {
        let assign1490_e1482: f64 = (p.p73 / locals.var_pa_t);
        let assign1490_e1484: f64 = (assign1490_e1482).powf(p.p74);
        let assign1490_e1485: f64 = (p.p72 * assign1490_e1484);
        (assign1490_e1485, (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * locals.var_pa_t_dn3) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * locals.var_pa_t_dn3) / (locals.var_pa_t * locals.var_pa_t))) / assign1490_e1482))) }),)
    } else {
        (locals.var_cja_t, locals.var_cja_t_dn3,)
    }
};
        locals.var_cja_t = assign1490_e1487;
        locals.var_cja_t_dn3 = assign1490_e1487_d_n3;
        locals.var_cja_t_rv = 0.0;

        let (assign1500_e1492, assign1500_e1492_d_n3,) = {
    if (locals.var_guard130 == 0.0) {
        (p.p73, 0.0,)
    } else {
        (locals.var_pa_t, locals.var_pa_t_dn3,)
    }
};
        locals.var_pa_t = assign1500_e1492;
        locals.var_pa_t_dn3 = assign1500_e1492_d_n3;
        locals.var_pa_t_rv = 0.0;

        let (assign1510_e1497, assign1510_e1497_d_n3,) = {
    if (locals.var_guard130 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_cja_t, locals.var_cja_t_dn3,)
    }
};
        locals.var_cja_t = assign1510_e1497;
        locals.var_cja_t_dn3 = assign1510_e1497_d_n3;
        locals.var_cja_t_rv = 0.0;

        let assign1520_e1500: f64 = if p.p79 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign1520_e1500;
        locals.var_guard133_rv = 0.0;

        let (assign1530_e1528, assign1530_e1528_d_n3,) = {
    if (locals.var_guard133 != 0.0) {
        let assign1530_e1505: f64 = (locals.var_phi_t / locals.var_rt);
        let assign1530_e1506: f64 = (2.0 * assign1530_e1505);
        let assign1530_e1509: f64 = (0.5 * p.p80);
        let assign1530_e1511: f64 = (assign1530_e1509 * locals.var_rt);
        let assign1530_e1513: f64 = (assign1530_e1511 / locals.var_phi_t);
        let assign1530_e1514: f64 = (assign1530_e1513).exp();
        let assign1530_e1516: f64 = (-0.5);
        let assign1530_e1518: f64 = (assign1530_e1516 * p.p80);
        let assign1530_e1520: f64 = (assign1530_e1518 * locals.var_rt);
        let assign1530_e1522: f64 = (assign1530_e1520 / locals.var_phi_t);
        let assign1530_e1523: f64 = (assign1530_e1522).exp();
        let assign1530_e1524: f64 = (assign1530_e1514 - assign1530_e1523);
        let assign1530_e1525: f64 = (assign1530_e1524).ln();
        let assign1530_e1526: f64 = (assign1530_e1506 * assign1530_e1525);
        (assign1530_e1526, (((2.0 * (((locals.var_phi_t_dn3 * locals.var_rt) - (locals.var_phi_t * locals.var_rt_dn3)) / (locals.var_rt * locals.var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * locals.var_rt_dn3) * locals.var_phi_t) - (assign1530_e1511 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * locals.var_rt_dn3) * locals.var_phi_t) - (assign1530_e1520 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)))) / assign1530_e1524))),)
    } else {
        (locals.var_psiio__blk134, locals.var_psiio__blk134_dn3,)
    }
};
        locals.var_psiio__blk134 = assign1530_e1528;
        locals.var_psiio__blk134_dn3 = assign1530_e1528_d_n3;
        locals.var_psiio__blk134_rv = 0.0;

        let (assign1540_e1547, assign1540_e1547_d_n3,) = {
    if (locals.var_guard133 != 0.0) {
        let assign1540_e1532: f64 = (locals.var_psiio__blk134 * locals.var_rt);
        let assign1540_e1535: f64 = (3.0 * locals.var_phi_t);
        let assign1540_e1537: f64 = (locals.var_rt).ln();
        let assign1540_e1538: f64 = (assign1540_e1535 * assign1540_e1537);
        let assign1540_e1539: f64 = (assign1540_e1532 - assign1540_e1538);
        let assign1540_e1543: f64 = (locals.var_rt - 1.0);
        let assign1540_e1544: f64 = (p.p90 * assign1540_e1543);
        let assign1540_e1545: f64 = (assign1540_e1539 - assign1540_e1544);
        (assign1540_e1545, ((((locals.var_psiio__blk134_dn3 * locals.var_rt) + (locals.var_psiio__blk134 * locals.var_rt_dn3)) - (((3.0 * locals.var_phi_t_dn3) * assign1540_e1537) + (assign1540_e1535 * (locals.var_rt_dn3 / locals.var_rt)))) - (p.p90 * locals.var_rt_dn3)),)
    } else {
        (locals.var_psiin__blk135, locals.var_psiin__blk135_dn3,)
    }
};
        locals.var_psiin__blk135 = assign1540_e1547;
        locals.var_psiin__blk135_dn3 = assign1540_e1547_d_n3;
        locals.var_psiin__blk135_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign1550_e1571, assign1550_e1571_d_n3,) = {
    if (locals.var_guard133 != 0.0) {
        let assign1550_e1552: f64 = (2.0 * locals.var_phi_t);
        let assign1550_e1558: f64 = (-locals.var_psiin__blk135);
        let assign1550_e1560: f64 = (assign1550_e1558 / locals.var_phi_t);
        let assign1550_e1561: f64 = (assign1550_e1560).exp();
        let assign1550_e1562: f64 = (4.0 * assign1550_e1561);
        let assign1550_e1563: f64 = (1.0 + assign1550_e1562);
        let assign1550_e1564: f64 = (assign1550_e1563).sqrt();
        let assign1550_e1565: f64 = (1.0 + assign1550_e1564);
        let assign1550_e1566: f64 = (0.5 * assign1550_e1565);
        let assign1550_e1567: f64 = (assign1550_e1566).ln();
        let assign1550_e1568: f64 = (assign1550_e1552 * assign1550_e1567);
        let assign1550_e1569: f64 = (locals.var_psiin__blk135 + assign1550_e1568);
        (assign1550_e1569, (locals.var_psiin__blk135_dn3 + (((2.0 * locals.var_phi_t_dn3) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-locals.var_psiin__blk135_dn3) * locals.var_phi_t) - (assign1550_e1558 * locals.var_phi_t_dn3)) / (locals.var_phi_t * locals.var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))),)
    } else {
        (locals.var_pp_t, locals.var_pp_t_dn3,)
    }
};
        locals.var_pp_t = assign1550_e1571;
        locals.var_pp_t_dn3 = assign1550_e1571_d_n3;
        locals.var_pp_t_rv = 0.0;

        let (assign1560_e1581, assign1560_e1581_d_n3,) = {
    if (locals.var_guard133 != 0.0) {
        let assign1560_e1576: f64 = (p.p80 / locals.var_pp_t);
        let assign1560_e1578: f64 = (assign1560_e1576).powf(p.p81);
        let assign1560_e1579: f64 = (p.p79 * assign1560_e1578);
        (assign1560_e1579, (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * locals.var_pp_t_dn3) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * locals.var_pp_t_dn3) / (locals.var_pp_t * locals.var_pp_t))) / assign1560_e1576))) }),)
    } else {
        (locals.var_cjp_t, locals.var_cjp_t_dn3,)
    }
};
        locals.var_cjp_t = assign1560_e1581;
        locals.var_cjp_t_dn3 = assign1560_e1581_d_n3;
        locals.var_cjp_t_rv = 0.0;

        let (assign1570_e1586, assign1570_e1586_d_n3,) = {
    if (locals.var_guard133 == 0.0) {
        (p.p80, 0.0,)
    } else {
        (locals.var_pp_t, locals.var_pp_t_dn3,)
    }
};
        locals.var_pp_t = assign1570_e1586;
        locals.var_pp_t_dn3 = assign1570_e1586_d_n3;
        locals.var_pp_t_rv = 0.0;

        let (assign1580_e1591, assign1580_e1591_d_n3,) = {
    if (locals.var_guard133 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_cjp_t, locals.var_cjp_t_dn3,)
    }
};
        locals.var_cjp_t = assign1580_e1591;
        locals.var_cjp_t_dn3 = assign1580_e1591_d_n3;
        locals.var_cjp_t_rv = 0.0;

        let assign1690_e1680: f64 = if ((p.p60 > 0.0) && (p.p15 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard137 = assign1690_e1680;
        locals.var_guard137_rv = 0.0;

        let (assign1700_e1690, assign1700_e1690_d_n3,) = {
    if ((locals.var_guard137 != 0.0) && (p.p62 != 0.0)) {
        let assign1700_e1686: f64 = (p.p61 * locals.var_tcvsat);
        let assign1700_e1688: f64 = (assign1700_e1686 * locals.var_tcr);
        (assign1700_e1688, (((p.p61 * locals.var_tcvsat_dn3) * locals.var_tcr) + (assign1700_e1686 * locals.var_tcr_dn3)),)
    } else {
        (locals.var_ecorn_t, locals.var_ecorn_t_dn3,)
    }
};
        locals.var_ecorn_t = assign1700_e1690;
        locals.var_ecorn_t_dn3 = assign1700_e1690_d_n3;
        locals.var_ecorn_t_rv = 0.0;

        let (assign1710_e1700, assign1710_e1700_d_n3,) = {
    if ((locals.var_guard137 != 0.0) && (p.p62 != 0.0)) {
        let assign1710_e1696: f64 = (p.p60 * locals.var_tcvsat);
        let assign1710_e1698: f64 = (assign1710_e1696 * locals.var_tcr);
        (assign1710_e1698, (((p.p60 * locals.var_tcvsat_dn3) * locals.var_tcr) + (assign1710_e1696 * locals.var_tcr_dn3)),)
    } else {
        (locals.var_ecrit_t, locals.var_ecrit_t_dn3,)
    }
};
        locals.var_ecrit_t = assign1710_e1700;
        locals.var_ecrit_t_dn3 = assign1710_e1700_d_n3;
        locals.var_ecrit_t_rv = 0.0;

        let (assign1720_e1707, assign1720_e1707_d_n3,) = {
    if ((locals.var_guard137 != 0.0) && (p.p62 == 0.0)) {
        (p.p61, 0.0,)
    } else {
        (locals.var_ecorn_t, locals.var_ecorn_t_dn3,)
    }
};
        locals.var_ecorn_t = assign1720_e1707;
        locals.var_ecorn_t_dn3 = assign1720_e1707_d_n3;
        locals.var_ecorn_t_rv = 0.0;

        let (assign1730_e1714, assign1730_e1714_d_n3,) = {
    if ((locals.var_guard137 != 0.0) && (p.p62 == 0.0)) {
        (p.p60, 0.0,)
    } else {
        (locals.var_ecrit_t, locals.var_ecrit_t_dn3,)
    }
};
        locals.var_ecrit_t = assign1730_e1714;
        locals.var_ecrit_t_dn3 = assign1730_e1714_d_n3;
        locals.var_ecrit_t_rv = 0.0;

        let (assign1770_e1766, assign1770_e1766_d_n3,) = {
    if (locals.var_guard137 != 0.0) {
        let assign1770_e1764: f64 = (locals.var_ecrit_t - locals.var_ecorn_t);
        (assign1770_e1764, (locals.var_ecrit_t_dn3 - locals.var_ecorn_t_dn3),)
    } else {
        (locals.var_de, locals.var_de_dn3,)
    }
};
        locals.var_de = assign1770_e1766;
        locals.var_de_dn3 = assign1770_e1766_d_n3;
        locals.var_de_rv = 0.0;

        let (assign1780_e1772, assign1780_e1772_d_n3,) = {
    if (locals.var_guard137 != 0.0) {
        let assign1780_e1770: f64 = (1.0 / locals.var_ecrit_t);
        (assign1780_e1770, (-(locals.var_ecrit_t_dn3 / (locals.var_ecrit_t * locals.var_ecrit_t))),)
    } else {
        (locals.var_iecrit, locals.var_iecrit_dn3,)
    }
};
        locals.var_iecrit = assign1780_e1772;
        locals.var_iecrit_dn3 = assign1780_e1772_d_n3;
        locals.var_iecrit_rv = 0.0;

        let (assign1820_e1792, assign1820_e1792_d_n3,) = {
    if (locals.var_guard137 == 0.0) {
        (1000.0, 0.0,)
    } else {
        (locals.var_de, locals.var_de_dn3,)
    }
};
        locals.var_de = assign1820_e1792;
        locals.var_de_dn3 = assign1820_e1792_d_n3;
        locals.var_de_rv = 0.0;

        let (assign1830_e1797, assign1830_e1797_d_n3,) = {
    if (locals.var_guard137 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_iecrit, locals.var_iecrit_dn3,)
    }
};
        locals.var_iecrit = assign1830_e1797;
        locals.var_iecrit_dn3 = assign1830_e1797_d_n3;
        locals.var_iecrit_rv = 0.0;

        let assign1840_e1800: f64 = (locals.var_leffe_um * locals.var_de);
        locals.var_lde = assign1840_e1800;
        locals.var_lde_dn3 = (locals.var_leffe_um * locals.var_de_dn3);
        locals.var_lde_rv = 0.0;

        let assign1850_e1803: f64 = if locals.var_lde > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign1850_e1803;
        locals.var_guard138_rv = 0.0;

        let (assign1860_e1807, assign1860_e1807_d_n3,) = {
    if (locals.var_guard138 != 0.0) {
        (100000.0, 0.0,)
    } else {
        (locals.var_lde, locals.var_lde_dn3,)
    }
};
        locals.var_lde = assign1860_e1807;
        locals.var_lde_dn3 = assign1860_e1807_d_n3;
        locals.var_lde_rv = 0.0;

        let assign1870_e1810: f64 = if locals.var_vrb < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign1870_e1810;
        locals.var_guard189_rv = 0.0;

        let (assign1890_e1820, assign1890_e1820_d_n1, assign1890_e1820_d_n4, assign1890_e1820_d_n5,) = {
    if (locals.var_guard189 != 0.0) {
        let assign1890_e1818: f64 = (-locals.var_vc2);
        (assign1890_e1818, (-locals.var_vc2_dn1), 0.0, (-locals.var_vc2_dn5),)
    } else {
        (locals.var_v1ci, locals.var_v1ci_dn1, locals.var_v1ci_dn4, locals.var_v1ci_dn5,)
    }
};
        locals.var_v1ci = assign1890_e1820;
        locals.var_v1ci_dn1 = assign1890_e1820_d_n1;
        locals.var_v1ci_dn4 = assign1890_e1820_d_n4;
        locals.var_v1ci_dn5 = assign1890_e1820_d_n5;
        locals.var_v1ci_rv = 0.0;

        let (assign1900_e1825, assign1900_e1825_d_n4, assign1900_e1825_d_n5,) = {
    if (locals.var_guard189 != 0.0) {
        let assign1900_e1823: f64 = (-locals.var_vrb);
        (assign1900_e1823, (-locals.var_vrb_dn4), (-locals.var_vrb_dn5),)
    } else {
        (locals.var_vrbi, locals.var_vrbi_dn4, locals.var_vrbi_dn5,)
    }
};
        locals.var_vrbi = assign1900_e1825;
        locals.var_vrbi_dn4 = assign1900_e1825_d_n4;
        locals.var_vrbi_dn5 = assign1900_e1825_d_n5;
        locals.var_vrbi_rv = 0.0;

        let (assign1920_e1836, assign1920_e1836_d_n1, assign1920_e1836_d_n4, assign1920_e1836_d_n5,) = {
    if (locals.var_guard189 == 0.0) {
        let assign1920_e1834: f64 = (-locals.var_vc1);
        (assign1920_e1834, (-locals.var_vc1_dn1), (-locals.var_vc1_dn4), 0.0,)
    } else {
        (locals.var_v1ci, locals.var_v1ci_dn1, locals.var_v1ci_dn4, locals.var_v1ci_dn5,)
    }
};
        locals.var_v1ci = assign1920_e1836;
        locals.var_v1ci_dn1 = assign1920_e1836_d_n1;
        locals.var_v1ci_dn4 = assign1920_e1836_d_n4;
        locals.var_v1ci_dn5 = assign1920_e1836_d_n5;
        locals.var_v1ci_rv = 0.0;

        let (assign1930_e1841, assign1930_e1841_d_n4, assign1930_e1841_d_n5,) = {
    if (locals.var_guard189 == 0.0) {
        (locals.var_vrb, locals.var_vrb_dn4, locals.var_vrb_dn5,)
    } else {
        (locals.var_vrbi, locals.var_vrbi_dn4, locals.var_vrbi_dn5,)
    }
};
        locals.var_vrbi = assign1930_e1841;
        locals.var_vrbi_dn4 = assign1930_e1841_d_n4;
        locals.var_vrbi_dn5 = assign1930_e1841_d_n5;
        locals.var_vrbi_rv = 0.0;

        let assign1940_e1844: f64 = if locals.var_v1ci > locals.var_vpoe { 1.0 } else { 0.0 };
        locals.var_guard190 = assign1940_e1844;
        locals.var_guard190_rv = 0.0;

        let (assign1950_e1860, assign1950_e1860_d_n1, assign1950_e1860_d_n3, assign1950_e1860_d_n4, assign1950_e1860_d_n5,) = {
    if (locals.var_guard190 != 0.0) {
        let assign1950_e1851: f64 = (locals.var_vpoe - locals.var_v1ci);
        let assign1950_e1853: f64 = (assign1950_e1851 / locals.var_nsteff);
        let assign1950_e1854: f64 = (assign1950_e1853).exp();
        let assign1950_e1855: f64 = (1.0 + assign1950_e1854);
        let assign1950_e1856: f64 = (assign1950_e1855).ln();
        let assign1950_e1857: f64 = (locals.var_nsteff * assign1950_e1856);
        let assign1950_e1858: f64 = (locals.var_vpoe - assign1950_e1857);
        (assign1950_e1858, (-(locals.var_nsteff * ((assign1950_e1854 * ((-locals.var_v1ci_dn1) / locals.var_nsteff)) / assign1950_e1855))), (locals.var_vpoe_dn3 - ((locals.var_nsteff_dn3 * assign1950_e1856) + (locals.var_nsteff * ((assign1950_e1854 * (((locals.var_vpoe_dn3 * locals.var_nsteff) - (assign1950_e1851 * locals.var_nsteff_dn3)) / (locals.var_nsteff * locals.var_nsteff))) / assign1950_e1855)))), (-(locals.var_nsteff * ((assign1950_e1854 * ((-locals.var_v1ci_dn4) / locals.var_nsteff)) / assign1950_e1855))), (-(locals.var_nsteff * ((assign1950_e1854 * ((-locals.var_v1ci_dn5) / locals.var_nsteff)) / assign1950_e1855))),)
    } else {
        (locals.var_v1cl, locals.var_v1cl_dn1, locals.var_v1cl_dn3, locals.var_v1cl_dn4, locals.var_v1cl_dn5,)
    }
};
        locals.var_v1cl = assign1950_e1860;
        locals.var_v1cl_dn1 = assign1950_e1860_d_n1;
        locals.var_v1cl_dn3 = assign1950_e1860_d_n3;
        locals.var_v1cl_dn4 = assign1950_e1860_d_n4;
        locals.var_v1cl_dn5 = assign1950_e1860_d_n5;
        locals.var_v1cl_rv = 0.0;

        let (assign1960_e1877, assign1960_e1877_d_n1, assign1960_e1877_d_n3, assign1960_e1877_d_n4, assign1960_e1877_d_n5,) = {
    if (locals.var_guard190 == 0.0) {
        let assign1960_e1868: f64 = (locals.var_v1ci - locals.var_vpoe);
        let assign1960_e1870: f64 = (assign1960_e1868 / locals.var_nsteff);
        let assign1960_e1871: f64 = (assign1960_e1870).exp();
        let assign1960_e1872: f64 = (1.0 + assign1960_e1871);
        let assign1960_e1873: f64 = (assign1960_e1872).ln();
        let assign1960_e1874: f64 = (locals.var_nsteff * assign1960_e1873);
        let assign1960_e1875: f64 = (locals.var_v1ci - assign1960_e1874);
        (assign1960_e1875, (locals.var_v1ci_dn1 - (locals.var_nsteff * ((assign1960_e1871 * (locals.var_v1ci_dn1 / locals.var_nsteff)) / assign1960_e1872))), (-((locals.var_nsteff_dn3 * assign1960_e1873) + (locals.var_nsteff * ((assign1960_e1871 * ((((-locals.var_vpoe_dn3) * locals.var_nsteff) - (assign1960_e1868 * locals.var_nsteff_dn3)) / (locals.var_nsteff * locals.var_nsteff))) / assign1960_e1872)))), (locals.var_v1ci_dn4 - (locals.var_nsteff * ((assign1960_e1871 * (locals.var_v1ci_dn4 / locals.var_nsteff)) / assign1960_e1872))), (locals.var_v1ci_dn5 - (locals.var_nsteff * ((assign1960_e1871 * (locals.var_v1ci_dn5 / locals.var_nsteff)) / assign1960_e1872))),)
    } else {
        (locals.var_v1cl, locals.var_v1cl_dn1, locals.var_v1cl_dn3, locals.var_v1cl_dn4, locals.var_v1cl_dn5,)
    }
};
        locals.var_v1cl = assign1960_e1877;
        locals.var_v1cl_dn1 = assign1960_e1877_d_n1;
        locals.var_v1cl_dn3 = assign1960_e1877_d_n3;
        locals.var_v1cl_dn4 = assign1960_e1877_d_n4;
        locals.var_v1cl_dn5 = assign1960_e1877_d_n5;
        locals.var_v1cl_rv = 0.0;

        let assign1970_e1880: f64 = (-0.4);
        let assign1970_e1885: f64 = (locals.var_vpoe - locals.var_v1cl);
        let (assign1970_e1891, assign1970_e1891_d_n1, assign1970_e1891_d_n3, assign1970_e1891_d_n4, assign1970_e1891_d_n5,) = {
    if (locals.var_vrbi < assign1970_e1885) {
        (locals.var_vrbi, 0.0, 0.0, locals.var_vrbi_dn4, locals.var_vrbi_dn5,)
    } else {
        let assign1970_e1890: f64 = (locals.var_vpoe - locals.var_v1cl);
        (assign1970_e1890, (-locals.var_v1cl_dn1), (locals.var_vpoe_dn3 - locals.var_v1cl_dn3), (-locals.var_v1cl_dn4), (-locals.var_v1cl_dn5),)
    }
};
        let assign1970_e1892: f64 = (locals.var_dp_i + assign1970_e1891);
        let assign1970_e1893: f64 = (assign1970_e1880 * assign1970_e1892);
        let assign1970_e1894: f64 = if locals.var_v1cl < assign1970_e1893 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign1970_e1894;
        locals.var_guard191_rv = 0.0;

        let (assign1980_e1914, assign1980_e1914_d_n1, assign1980_e1914_d_n3, assign1980_e1914_d_n4, assign1980_e1914_d_n5,) = {
    if ((p.p63 != 0.0) && (locals.var_guard191 != 0.0)) {
        let assign1980_e1899: f64 = (-0.4);
        let assign1980_e1904: f64 = (locals.var_vpoe - locals.var_v1cl);
        let (assign1980_e1910, assign1980_e1910_d_n1, assign1980_e1910_d_n3, assign1980_e1910_d_n4, assign1980_e1910_d_n5,) = {
            if (locals.var_vrbi < assign1980_e1904) {
                (locals.var_vrbi, 0.0, 0.0, locals.var_vrbi_dn4, locals.var_vrbi_dn5,)
            } else {
                let assign1980_e1909: f64 = (locals.var_vpoe - locals.var_v1cl);
                (assign1980_e1909, (-locals.var_v1cl_dn1), (locals.var_vpoe_dn3 - locals.var_v1cl_dn3), (-locals.var_v1cl_dn4), (-locals.var_v1cl_dn5),)
            }
        };
        let assign1980_e1911: f64 = (locals.var_dp_i + assign1980_e1910);
        let assign1980_e1912: f64 = (assign1980_e1899 * assign1980_e1911);
        (assign1980_e1912, (assign1980_e1899 * assign1980_e1910_d_n1), (assign1980_e1899 * (locals.var_dp_i_dn3 + assign1980_e1910_d_n3)), (assign1980_e1899 * assign1980_e1910_d_n4), (assign1980_e1899 * assign1980_e1910_d_n5),)
    } else {
        (locals.var_v1c, locals.var_v1c_dn1, locals.var_v1c_dn3, locals.var_v1c_dn4, locals.var_v1c_dn5,)
    }
};
        locals.var_v1c = assign1980_e1914;
        locals.var_v1c_dn1 = assign1980_e1914_d_n1;
        locals.var_v1c_dn3 = assign1980_e1914_d_n3;
        locals.var_v1c_dn4 = assign1980_e1914_d_n4;
        locals.var_v1c_dn5 = assign1980_e1914_d_n5;
        locals.var_v1c_rv = 0.0;

        let (assign1990_e1921, assign1990_e1921_d_n1, assign1990_e1921_d_n3, assign1990_e1921_d_n4, assign1990_e1921_d_n5,) = {
    if ((p.p63 != 0.0) && (locals.var_guard191 == 0.0)) {
        (locals.var_v1cl, locals.var_v1cl_dn1, locals.var_v1cl_dn3, locals.var_v1cl_dn4, locals.var_v1cl_dn5,)
    } else {
        (locals.var_v1c, locals.var_v1c_dn1, locals.var_v1c_dn3, locals.var_v1c_dn4, locals.var_v1c_dn5,)
    }
};
        locals.var_v1c = assign1990_e1921;
        locals.var_v1c_dn1 = assign1990_e1921_d_n1;
        locals.var_v1c_dn3 = assign1990_e1921_d_n3;
        locals.var_v1c_dn4 = assign1990_e1921_d_n4;
        locals.var_v1c_dn5 = assign1990_e1921_d_n5;
        locals.var_v1c_rv = 0.0;

        let assign2000_e1924: f64 = (-0.4);
        let assign2000_e1926: f64 = (assign2000_e1924 * locals.var_dp_i);
        let assign2000_e1927: f64 = if locals.var_v1cl < assign2000_e1926 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign2000_e1927;
        locals.var_guard192_rv = 0.0;

        let (assign2010_e1937, assign2010_e1937_d_n1, assign2010_e1937_d_n3, assign2010_e1937_d_n4, assign2010_e1937_d_n5,) = {
    if ((p.p63 == 0.0) && (locals.var_guard192 != 0.0)) {
        let assign2010_e1933: f64 = (-0.4);
        let assign2010_e1935: f64 = (assign2010_e1933 * locals.var_dp_i);
        (assign2010_e1935, 0.0, (assign2010_e1933 * locals.var_dp_i_dn3), 0.0, 0.0,)
    } else {
        (locals.var_v1c, locals.var_v1c_dn1, locals.var_v1c_dn3, locals.var_v1c_dn4, locals.var_v1c_dn5,)
    }
};
        locals.var_v1c = assign2010_e1937;
        locals.var_v1c_dn1 = assign2010_e1937_d_n1;
        locals.var_v1c_dn3 = assign2010_e1937_d_n3;
        locals.var_v1c_dn4 = assign2010_e1937_d_n4;
        locals.var_v1c_dn5 = assign2010_e1937_d_n5;
        locals.var_v1c_rv = 0.0;

        let (assign2020_e1945, assign2020_e1945_d_n1, assign2020_e1945_d_n3, assign2020_e1945_d_n4, assign2020_e1945_d_n5,) = {
    if ((p.p63 == 0.0) && (locals.var_guard192 == 0.0)) {
        (locals.var_v1cl, locals.var_v1cl_dn1, locals.var_v1cl_dn3, locals.var_v1cl_dn4, locals.var_v1cl_dn5,)
    } else {
        (locals.var_v1c, locals.var_v1c_dn1, locals.var_v1c_dn3, locals.var_v1c_dn4, locals.var_v1c_dn5,)
    }
};
        locals.var_v1c = assign2020_e1945;
        locals.var_v1c_dn1 = assign2020_e1945_d_n1;
        locals.var_v1c_dn3 = assign2020_e1945_d_n3;
        locals.var_v1c_dn4 = assign2020_e1945_d_n4;
        locals.var_v1c_dn5 = assign2020_e1945_d_n5;
        locals.var_v1c_rv = 0.0;

        let assign2030_e1949: f64 = (2.0 * locals.var_v1c);
        let assign2030_e1950: f64 = (locals.var_dp_i + assign2030_e1949);
        locals.var_pe = assign2030_e1950;
        locals.var_pe_dn1 = (2.0 * locals.var_v1c_dn1);
        locals.var_pe_dn3 = (locals.var_dp_i_dn3 + (2.0 * locals.var_v1c_dn3));
        locals.var_pe_dn4 = (2.0 * locals.var_v1c_dn4);
        locals.var_pe_dn5 = (2.0 * locals.var_v1c_dn5);
        locals.var_pe_rv = 0.0;

        let assign2040_e1953: f64 = if locals.var_iecrit > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign2040_e1953;
        locals.var_guard193_rv = 0.0;

        let (assign2060_e1974, assign2060_e1974_d_n1, assign2060_e1974_d_n3, assign2060_e1974_d_n4, assign2060_e1974_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2060_e1966: f64 = (-1.0);
        let assign2060_e1969: f64 = (3.0 * locals.var_dfsq);
        let assign2060_e1971: f64 = (assign2060_e1969 * locals.var_pe);
        let assign2060_e1972: f64 = (assign2060_e1966 + assign2060_e1971);
        (assign2060_e1972, (assign2060_e1969 * locals.var_pe_dn1), (((3.0 * locals.var_dfsq_dn3) * locals.var_pe) + (assign2060_e1969 * locals.var_pe_dn3)), (assign2060_e1969 * locals.var_pe_dn4), (assign2060_e1969 * locals.var_pe_dn5),)
    } else {
        (p.p3, locals.var_a1_dn1, locals.var_a1_dn3, locals.var_a1_dn4, locals.var_a1_dn5,)
    }
};
        locals.var_a1 = assign2060_e1974;
        locals.var_a1_dn1 = assign2060_e1974_d_n1;
        locals.var_a1_dn3 = assign2060_e1974_d_n3;
        locals.var_a1_dn4 = assign2060_e1974_d_n4;
        locals.var_a1_dn5 = assign2060_e1974_d_n5;
        locals.var_a1_rv = 0.0;

        let (assign2070_e1986, assign2070_e1986_d_n1, assign2070_e1986_d_n3, assign2070_e1986_d_n4, assign2070_e1986_d_n5,) = {
    if (locals.var_guard193 != 0.0) {
        let assign2070_e1979: f64 = (9.0 / 4.0);
        let assign2070_e1982: f64 = (locals.var_pe / locals.var_lde);
        let assign2070_e1983: f64 = (assign2070_e1979 + assign2070_e1982);
        let assign2070_e1984: f64 = (locals.var_dfsq * assign2070_e1983);
        (assign2070_e1984, (locals.var_dfsq * (locals.var_pe_dn1 / locals.var_lde)), ((locals.var_dfsq_dn3 * assign2070_e1983) + (locals.var_dfsq * (((locals.var_pe_dn3 * locals.var_lde) - (locals.var_pe * locals.var_lde_dn3)) / (locals.var_lde * locals.var_lde)))), (locals.var_dfsq * (locals.var_pe_dn4 / locals.var_lde)), (locals.var_dfsq * (locals.var_pe_dn5 / locals.var_lde)),)
    } else {
        (p.p6, locals.var_a2_dn1, locals.var_a2_dn3, locals.var_a2_dn4, locals.var_a2_dn5,)
    }
};
        locals.var_a2 = assign2070_e1986;
        locals.var_a2_dn1 = assign2070_e1986_d_n1;
        locals.var_a2_dn3 = assign2070_e1986_d_n3;
        locals.var_a2_dn4 = assign2070_e1986_d_n4;
        locals.var_a2_dn5 = assign2070_e1986_d_n5;
        locals.var_a2_rv = 0.0;

        let assign3800_e3655: f64 = if locals.var_cj1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign3800_e3655;
        locals.var_guard249_rv = 0.0;

        let (assign3810_e3676, assign3810_e3676_d_n1, assign3810_e3676_d_n3, assign3810_e3676_d_n4, assign3810_e3676_d_n5,) = {
    if ((locals.var_guard249 != 0.0) && (p.p63 != 0.0)) {
        let assign3810_e3662: f64 = (locals.var_vc1 - locals.var_vpo);
        let assign3810_e3665: f64 = (locals.var_vc1 + locals.var_vpo);
        let assign3810_e3668: f64 = (locals.var_vc1 + locals.var_vpo);
        let assign3810_e3669: f64 = (assign3810_e3665 * assign3810_e3668);
        let assign3810_e3671: f64 = (assign3810_e3669 + 0.04);
        let assign3810_e3672: f64 = (assign3810_e3671).sqrt();
        let assign3810_e3673: f64 = (assign3810_e3662 + assign3810_e3672);
        let assign3810_e3674: f64 = (0.5 * assign3810_e3673);
        (assign3810_e3674, (0.5 * (locals.var_vc1_dn1 + (((locals.var_vc1_dn1 * assign3810_e3668) + (assign3810_e3665 * locals.var_vc1_dn1)) / (2.0 * assign3810_e3672)))), (0.5 * ((-locals.var_vpo_dn3) + (((locals.var_vpo_dn3 * assign3810_e3668) + (assign3810_e3665 * locals.var_vpo_dn3)) / (2.0 * assign3810_e3672)))), (0.5 * (locals.var_vc1_dn4 + (((locals.var_vc1_dn4 * assign3810_e3668) + (assign3810_e3665 * locals.var_vc1_dn4)) / (2.0 * assign3810_e3672)))), 0.0,)
    } else {
        (locals.var_vcl, locals.var_vcl_dn1, locals.var_vcl_dn3, locals.var_vcl_dn4, locals.var_vcl_dn5,)
    }
};
        locals.var_vcl = assign3810_e3676;
        locals.var_vcl_dn1 = assign3810_e3676_d_n1;
        locals.var_vcl_dn3 = assign3810_e3676_d_n3;
        locals.var_vcl_dn4 = assign3810_e3676_d_n4;
        locals.var_vcl_dn5 = assign3810_e3676_d_n5;
        locals.var_vcl_rv = 0.0;

        let (assign3820_e3683, assign3820_e3683_d_n1, assign3820_e3683_d_n3, assign3820_e3683_d_n4, assign3820_e3683_d_n5,) = {
    if ((locals.var_guard249 != 0.0) && (p.p63 == 0.0)) {
        (locals.var_vc1, locals.var_vc1_dn1, 0.0, locals.var_vc1_dn4, 0.0,)
    } else {
        (locals.var_vcl, locals.var_vcl_dn1, locals.var_vcl_dn3, locals.var_vcl_dn4, locals.var_vcl_dn5,)
    }
};
        locals.var_vcl = assign3820_e3683;
        locals.var_vcl_dn1 = assign3820_e3683_d_n1;
        locals.var_vcl_dn3 = assign3820_e3683_d_n3;
        locals.var_vcl_dn4 = assign3820_e3683_d_n4;
        locals.var_vcl_dn5 = assign3820_e3683_d_n5;
        locals.var_vcl_rv = 0.0;

        let (assign3830_e3689, assign3830_e3689_d_n1, assign3830_e3689_d_n3, assign3830_e3689_d_n4, assign3830_e3689_d_n5,) = {
    if (locals.var_guard249 != 0.0) {
        let assign3830_e3687: f64 = (locals.var_a1_um2 * locals.var_cja_t);
        (assign3830_e3687, (locals.var_a1_um2_dn1 * locals.var_cja_t), ((locals.var_a1_um2_dn3 * locals.var_cja_t) + (locals.var_a1_um2 * locals.var_cja_t_dn3)), (locals.var_a1_um2_dn4 * locals.var_cja_t), (locals.var_a1_um2_dn5 * locals.var_cja_t),)
    } else {
        (locals.var_acja, locals.var_acja_dn1, locals.var_acja_dn3, locals.var_acja_dn4, locals.var_acja_dn5,)
    }
};
        locals.var_acja = assign3830_e3689;
        locals.var_acja_dn1 = assign3830_e3689_d_n1;
        locals.var_acja_dn3 = assign3830_e3689_d_n3;
        locals.var_acja_dn4 = assign3830_e3689_d_n4;
        locals.var_acja_dn5 = assign3830_e3689_d_n5;
        locals.var_acja_rv = 0.0;

        let (assign3840_e3695, assign3840_e3695_d_n3,) = {
    if (locals.var_guard249 != 0.0) {
        let assign3840_e3693: f64 = (locals.var_p1_um * locals.var_cjp_t);
        (assign3840_e3693, (locals.var_p1_um * locals.var_cjp_t_dn3),)
    } else {
        (locals.var_pcjp, locals.var_pcjp_dn3,)
    }
};
        locals.var_pcjp = assign3840_e3695;
        locals.var_pcjp_dn3 = assign3840_e3695_d_n3;
        locals.var_pcjp_rv = 0.0;

        let assign3850_e3698: f64 = if locals.var_acja > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign3850_e3698;
        locals.var_guard254_rv = 0.0;

        let (assign3860_e3707, assign3860_e3707_d_n3,) = {
    if ((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) {
        let assign3860_e3703: f64 = (-locals.var_pa_t);
        let assign3860_e3705: f64 = (assign3860_e3703 * p.p68);
        (assign3860_e3705, ((-locals.var_pa_t_dn3) * p.p68),)
    } else {
        (locals.var_dv0, locals.var_dv0_dn3,)
    }
};
        locals.var_dv0 = assign3860_e3707;
        locals.var_dv0_dn3 = assign3860_e3707_d_n3;
        locals.var_dv0_rv = 0.0;

        let assign3870_e3710: f64 = if p.p75 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign3870_e3710;
        locals.var_guard265_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign3880_e3720, assign3880_e3720_d_n1, assign3880_e3720_d_n3, assign3880_e3720_d_n4, assign3880_e3720_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) {
        let assign3880_e3718: f64 = (locals.var_vcl + locals.var_dv0);
        (assign3880_e3718, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dvh, locals.var_dvh_dn1, locals.var_dvh_dn3, locals.var_dvh_dn4, locals.var_dvh_dn5,)
    }
};
        locals.var_dvh = assign3880_e3720;
        locals.var_dvh_dn1 = assign3880_e3720_d_n1;
        locals.var_dvh_dn3 = assign3880_e3720_d_n3;
        locals.var_dvh_dn4 = assign3880_e3720_d_n4;
        locals.var_dvh_dn5 = assign3880_e3720_d_n5;
        locals.var_dvh_rv = 0.0;

        let assign3890_e3723: f64 = if locals.var_dvh > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard266 = assign3890_e3723;
        locals.var_guard266_rv = 0.0;

        let (assign3900_e3738,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign3900_e3733: f64 = (1.0 - p.p68);
        let assign3900_e3735: f64 = (-p.p74);
        let assign3900_e3736: f64 = (assign3900_e3733).powf(assign3900_e3735);
        (assign3900_e3736,)
    } else {
        (locals.var_pwq,)
    }
};
        locals.var_pwq = assign3900_e3738;
        locals.var_pwq_rv = 0.0;

        let (assign3910_e3760, assign3910_e3760_d_n1, assign3910_e3760_d_n3, assign3910_e3760_d_n4, assign3910_e3760_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign3910_e3751: f64 = (1.0 - p.p68);
        let assign3910_e3752: f64 = (locals.var_pwq * assign3910_e3751);
        let assign3910_e3753: f64 = (1.0 - assign3910_e3752);
        let assign3910_e3754: f64 = (locals.var_pa_t * assign3910_e3753);
        let assign3910_e3757: f64 = (1.0 - p.p74);
        let assign3910_e3758: f64 = (assign3910_e3754 / assign3910_e3757);
        (assign3910_e3758, 0.0, ((locals.var_pa_t_dn3 * assign3910_e3753) / assign3910_e3757), 0.0, 0.0,)
    } else {
        (locals.var_qlo, locals.var_qlo_dn1, locals.var_qlo_dn3, locals.var_qlo_dn4, locals.var_qlo_dn5,)
    }
};
        locals.var_qlo = assign3910_e3760;
        locals.var_qlo_dn1 = assign3910_e3760_d_n1;
        locals.var_qlo_dn3 = assign3910_e3760_d_n3;
        locals.var_qlo_dn4 = assign3910_e3760_d_n4;
        locals.var_qlo_dn5 = assign3910_e3760_d_n5;
        locals.var_qlo_rv = 0.0;

        let (assign3920_e3786, assign3920_e3786_d_n1, assign3920_e3786_d_n3, assign3920_e3786_d_n4, assign3920_e3786_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign3920_e3772: f64 = (0.5 * p.p74);
        let assign3920_e3774: f64 = (assign3920_e3772 * locals.var_dvh);
        let assign3920_e3778: f64 = (1.0 - p.p68);
        let assign3920_e3779: f64 = (locals.var_pa_t * assign3920_e3778);
        let assign3920_e3780: f64 = (assign3920_e3774 / assign3920_e3779);
        let assign3920_e3781: f64 = (1.0 + assign3920_e3780);
        let assign3920_e3782: f64 = (locals.var_dvh * assign3920_e3781);
        let assign3920_e3784: f64 = (assign3920_e3782 * locals.var_pwq);
        (assign3920_e3784, (((locals.var_dvh_dn1 * assign3920_e3781) + (locals.var_dvh * ((assign3920_e3772 * locals.var_dvh_dn1) / assign3920_e3779))) * locals.var_pwq), (((locals.var_dvh_dn3 * assign3920_e3781) + (locals.var_dvh * ((((assign3920_e3772 * locals.var_dvh_dn3) * assign3920_e3779) - (assign3920_e3774 * (locals.var_pa_t_dn3 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * locals.var_pwq), (((locals.var_dvh_dn4 * assign3920_e3781) + (locals.var_dvh * ((assign3920_e3772 * locals.var_dvh_dn4) / assign3920_e3779))) * locals.var_pwq), (((locals.var_dvh_dn5 * assign3920_e3781) + (locals.var_dvh * ((assign3920_e3772 * locals.var_dvh_dn5) / assign3920_e3779))) * locals.var_pwq),)
    } else {
        (locals.var_qhi, locals.var_qhi_dn1, locals.var_qhi_dn3, locals.var_qhi_dn4, locals.var_qhi_dn5,)
    }
};
        locals.var_qhi = assign3920_e3786;
        locals.var_qhi_dn1 = assign3920_e3786_d_n1;
        locals.var_qhi_dn3 = assign3920_e3786_d_n3;
        locals.var_qhi_dn4 = assign3920_e3786_d_n4;
        locals.var_qhi_dn5 = assign3920_e3786_d_n5;
        locals.var_qhi_rv = 0.0;

        let (assign3930_e3813, assign3930_e3813_d_n1, assign3930_e3813_d_n3, assign3930_e3813_d_n4, assign3930_e3813_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign3930_e3800: f64 = (locals.var_vcl / locals.var_pa_t);
        let assign3930_e3801: f64 = (1.0 - assign3930_e3800);
        let assign3930_e3804: f64 = (1.0 - p.p74);
        let assign3930_e3805: f64 = (assign3930_e3801).powf(assign3930_e3804);
        let assign3930_e3806: f64 = (1.0 - assign3930_e3805);
        let assign3930_e3807: f64 = (locals.var_pa_t * assign3930_e3806);
        let assign3930_e3810: f64 = (1.0 - p.p74);
        let assign3930_e3811: f64 = (assign3930_e3807 / assign3930_e3810);
        (assign3930_e3811, ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(locals.var_vcl_dn1 / locals.var_pa_t)))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(locals.var_vcl_dn1 / locals.var_pa_t)) / assign3930_e3801))) })) / assign3930_e3810), (((locals.var_pa_t_dn3 * assign3930_e3806) + (locals.var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((locals.var_vcl_dn3 * locals.var_pa_t) - (locals.var_vcl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((locals.var_vcl_dn3 * locals.var_pa_t) - (locals.var_vcl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(locals.var_vcl_dn4 / locals.var_pa_t)))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(locals.var_vcl_dn4 / locals.var_pa_t)) / assign3930_e3801))) })) / assign3930_e3810), ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(locals.var_vcl_dn5 / locals.var_pa_t)))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(locals.var_vcl_dn5 / locals.var_pa_t)) / assign3930_e3801))) })) / assign3930_e3810),)
    } else {
        (locals.var_qlo, locals.var_qlo_dn1, locals.var_qlo_dn3, locals.var_qlo_dn4, locals.var_qlo_dn5,)
    }
};
        locals.var_qlo = assign3930_e3813;
        locals.var_qlo_dn1 = assign3930_e3813_d_n1;
        locals.var_qlo_dn3 = assign3930_e3813_d_n3;
        locals.var_qlo_dn4 = assign3930_e3813_d_n4;
        locals.var_qlo_dn5 = assign3930_e3813_d_n5;
        locals.var_qlo_rv = 0.0;

        let (assign3940_e3824, assign3940_e3824_d_n1, assign3940_e3824_d_n3, assign3940_e3824_d_n4, assign3940_e3824_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) && (locals.var_guard266 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi, locals.var_qhi_dn1, locals.var_qhi_dn3, locals.var_qhi_dn4, locals.var_qhi_dn5,)
    }
};
        locals.var_qhi = assign3940_e3824;
        locals.var_qhi_dn1 = assign3940_e3824_d_n1;
        locals.var_qhi_dn3 = assign3940_e3824_d_n3;
        locals.var_qhi_dn4 = assign3940_e3824_d_n4;
        locals.var_qhi_dn5 = assign3940_e3824_d_n5;
        locals.var_qhi_rv = 0.0;

        let (assign3950_e3834, assign3950_e3834_d_n1, assign3950_e3834_d_n3, assign3950_e3834_d_n4, assign3950_e3834_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 != 0.0)) {
        let assign3950_e3832: f64 = (locals.var_qlo + locals.var_qhi);
        (assign3950_e3832, (locals.var_qlo_dn1 + locals.var_qhi_dn1), (locals.var_qlo_dn3 + locals.var_qhi_dn3), (locals.var_qlo_dn4 + locals.var_qhi_dn4), (locals.var_qlo_dn5 + locals.var_qhi_dn5),)
    } else {
        (locals.var_arga, locals.var_arga_dn1, locals.var_arga_dn3, locals.var_arga_dn4, locals.var_arga_dn5,)
    }
};
        locals.var_arga = assign3950_e3834;
        locals.var_arga_dn1 = assign3950_e3834_d_n1;
        locals.var_arga_dn3 = assign3950_e3834_d_n3;
        locals.var_arga_dn4 = assign3950_e3834_d_n4;
        locals.var_arga_dn5 = assign3950_e3834_d_n5;
        locals.var_arga_rv = 0.0;

        let (assign3960_e3852, assign3960_e3852_d_n3,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign3960_e3843: f64 = (locals.var_dv0 * locals.var_dv0);
        let assign3960_e3846: f64 = (4.0 * p.p75);
        let assign3960_e3848: f64 = (assign3960_e3846 * p.p75);
        let assign3960_e3849: f64 = (assign3960_e3843 + assign3960_e3848);
        let assign3960_e3850: f64 = (assign3960_e3849).sqrt();
        (assign3960_e3850, (((locals.var_dv0_dn3 * locals.var_dv0) + (locals.var_dv0 * locals.var_dv0_dn3)) / (2.0 * assign3960_e3850)),)
    } else {
        (locals.var_mv0, locals.var_mv0_dn3,)
    }
};
        locals.var_mv0 = assign3960_e3852;
        locals.var_mv0_dn3 = assign3960_e3852_d_n3;
        locals.var_mv0_rv = 0.0;

        let (assign3970_e3866, assign3970_e3866_d_n3,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign3970_e3860: f64 = (-0.5);
        let assign3970_e3863: f64 = (locals.var_dv0 + locals.var_mv0);
        let assign3970_e3864: f64 = (assign3970_e3860 * assign3970_e3863);
        (assign3970_e3864, (assign3970_e3860 * (locals.var_dv0_dn3 + locals.var_mv0_dn3)),)
    } else {
        (locals.var_vl0, locals.var_vl0_dn3,)
    }
};
        locals.var_vl0 = assign3970_e3866;
        locals.var_vl0_dn3 = assign3970_e3866_d_n3;
        locals.var_vl0_rv = 0.0;

        let (assign3980_e3877, assign3980_e3877_d_n1, assign3980_e3877_d_n3, assign3980_e3877_d_n4, assign3980_e3877_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign3980_e3875: f64 = (locals.var_vcl + locals.var_dv0);
        (assign3980_e3875, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dv, locals.var_dv_dn1, locals.var_dv_dn3, locals.var_dv_dn4, locals.var_dv_dn5,)
    }
};
        locals.var_dv = assign3980_e3877;
        locals.var_dv_dn1 = assign3980_e3877_d_n1;
        locals.var_dv_dn3 = assign3980_e3877_d_n3;
        locals.var_dv_dn4 = assign3980_e3877_d_n4;
        locals.var_dv_dn5 = assign3980_e3877_d_n5;
        locals.var_dv_rv = 0.0;

        let (assign3990_e3895, assign3990_e3895_d_n1, assign3990_e3895_d_n3, assign3990_e3895_d_n4, assign3990_e3895_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign3990_e3886: f64 = (locals.var_dv * locals.var_dv);
        let assign3990_e3889: f64 = (4.0 * p.p75);
        let assign3990_e3891: f64 = (assign3990_e3889 * p.p75);
        let assign3990_e3892: f64 = (assign3990_e3886 + assign3990_e3891);
        let assign3990_e3893: f64 = (assign3990_e3892).sqrt();
        (assign3990_e3893, (((locals.var_dv_dn1 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn1)) / (2.0 * assign3990_e3893)), (((locals.var_dv_dn3 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn3)) / (2.0 * assign3990_e3893)), (((locals.var_dv_dn4 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn4)) / (2.0 * assign3990_e3893)), (((locals.var_dv_dn5 * locals.var_dv) + (locals.var_dv * locals.var_dv_dn5)) / (2.0 * assign3990_e3893)),)
    } else {
        (locals.var_mv, locals.var_mv_dn1, locals.var_mv_dn3, locals.var_mv_dn4, locals.var_mv_dn5,)
    }
};
        locals.var_mv = assign3990_e3895;
        locals.var_mv_dn1 = assign3990_e3895_d_n1;
        locals.var_mv_dn3 = assign3990_e3895_d_n3;
        locals.var_mv_dn4 = assign3990_e3895_d_n4;
        locals.var_mv_dn5 = assign3990_e3895_d_n5;
        locals.var_mv_rv = 0.0;

        let (assign4000_e3910, assign4000_e3910_d_n1, assign4000_e3910_d_n3, assign4000_e3910_d_n4, assign4000_e3910_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign4000_e3905: f64 = (locals.var_dv - locals.var_mv);
        let assign4000_e3906: f64 = (0.5 * assign4000_e3905);
        let assign4000_e3908: f64 = (assign4000_e3906 - locals.var_dv0);
        (assign4000_e3908, (0.5 * (locals.var_dv_dn1 - locals.var_mv_dn1)), ((0.5 * (locals.var_dv_dn3 - locals.var_mv_dn3)) - locals.var_dv0_dn3), (0.5 * (locals.var_dv_dn4 - locals.var_mv_dn4)), (0.5 * (locals.var_dv_dn5 - locals.var_mv_dn5)),)
    } else {
        (locals.var_vl, locals.var_vl_dn1, locals.var_vl_dn3, locals.var_vl_dn4, locals.var_vl_dn5,)
    }
};
        locals.var_vl = assign4000_e3910;
        locals.var_vl_dn1 = assign4000_e3910_d_n1;
        locals.var_vl_dn3 = assign4000_e3910_d_n3;
        locals.var_vl_dn4 = assign4000_e3910_d_n4;
        locals.var_vl_dn5 = assign4000_e3910_d_n5;
        locals.var_vl_rv = 0.0;

        let (assign4010_e3934, assign4010_e3934_d_n1, assign4010_e3934_d_n3, assign4010_e3934_d_n4, assign4010_e3934_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign4010_e3918: f64 = (-locals.var_pa_t);
        let assign4010_e3922: f64 = (locals.var_vl / locals.var_pa_t);
        let assign4010_e3923: f64 = (1.0 - assign4010_e3922);
        let assign4010_e3926: f64 = (1.0 - p.p74);
        let assign4010_e3927: f64 = (assign4010_e3923).powf(assign4010_e3926);
        let assign4010_e3928: f64 = (assign4010_e3918 * assign4010_e3927);
        let assign4010_e3931: f64 = (1.0 - p.p74);
        let assign4010_e3932: f64 = (assign4010_e3928 / assign4010_e3931);
        (assign4010_e3932, ((assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(locals.var_vl_dn1 / locals.var_pa_t)))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(locals.var_vl_dn1 / locals.var_pa_t)) / assign4010_e3923))) }) / assign4010_e3931), ((((-locals.var_pa_t_dn3) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((locals.var_vl_dn3 * locals.var_pa_t) - (locals.var_vl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((locals.var_vl_dn3 * locals.var_pa_t) - (locals.var_vl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(locals.var_vl_dn4 / locals.var_pa_t)))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(locals.var_vl_dn4 / locals.var_pa_t)) / assign4010_e3923))) }) / assign4010_e3931), ((assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(locals.var_vl_dn5 / locals.var_pa_t)))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(locals.var_vl_dn5 / locals.var_pa_t)) / assign4010_e3923))) }) / assign4010_e3931),)
    } else {
        (locals.var_qlo, locals.var_qlo_dn1, locals.var_qlo_dn3, locals.var_qlo_dn4, locals.var_qlo_dn5,)
    }
};
        locals.var_qlo = assign4010_e3934;
        locals.var_qlo_dn1 = assign4010_e3934_d_n1;
        locals.var_qlo_dn3 = assign4010_e3934_d_n3;
        locals.var_qlo_dn4 = assign4010_e3934_d_n4;
        locals.var_qlo_dn5 = assign4010_e3934_d_n5;
        locals.var_qlo_rv = 0.0;

        let (assign4020_e3974, assign4020_e3974_d_n1, assign4020_e3974_d_n3, assign4020_e3974_d_n4, assign4020_e3974_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard265 == 0.0)) {
        let assign4020_e3944: f64 = (1.0 - p.p68);
        let assign4020_e3946: f64 = (-p.p74);
        let assign4020_e3947: f64 = (assign4020_e3944).powf(assign4020_e3946);
        let assign4020_e3950: f64 = (locals.var_vcl - locals.var_vl);
        let assign4020_e3952: f64 = (assign4020_e3950 + locals.var_vl0);
        let assign4020_e3953: f64 = (assign4020_e3947 * assign4020_e3952);
        let assign4020_e3957: f64 = (0.5 * p.p74);
        let assign4020_e3960: f64 = (locals.var_vcl - locals.var_vl);
        let assign4020_e3962: f64 = (assign4020_e3960 + locals.var_vl0);
        let assign4020_e3963: f64 = (assign4020_e3957 * assign4020_e3962);
        let assign4020_e3967: f64 = (1.0 - p.p68);
        let assign4020_e3968: f64 = (locals.var_pa_t * assign4020_e3967);
        let assign4020_e3969: f64 = (assign4020_e3963 / assign4020_e3968);
        let assign4020_e3970: f64 = (1.0 + assign4020_e3969);
        let assign4020_e3971: f64 = (assign4020_e3953 * assign4020_e3970);
        let assign4020_e3972: f64 = (locals.var_qlo + assign4020_e3971);
        (assign4020_e3972, (locals.var_qlo_dn1 + (((assign4020_e3947 * (locals.var_vcl_dn1 - locals.var_vl_dn1)) * assign4020_e3970) + (assign4020_e3953 * ((assign4020_e3957 * (locals.var_vcl_dn1 - locals.var_vl_dn1)) / assign4020_e3968)))), (locals.var_qlo_dn3 + (((assign4020_e3947 * ((locals.var_vcl_dn3 - locals.var_vl_dn3) + locals.var_vl0_dn3)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((locals.var_vcl_dn3 - locals.var_vl_dn3) + locals.var_vl0_dn3)) * assign4020_e3968) - (assign4020_e3963 * (locals.var_pa_t_dn3 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (locals.var_qlo_dn4 + (((assign4020_e3947 * (locals.var_vcl_dn4 - locals.var_vl_dn4)) * assign4020_e3970) + (assign4020_e3953 * ((assign4020_e3957 * (locals.var_vcl_dn4 - locals.var_vl_dn4)) / assign4020_e3968)))), (locals.var_qlo_dn5 + (((assign4020_e3947 * (locals.var_vcl_dn5 - locals.var_vl_dn5)) * assign4020_e3970) + (assign4020_e3953 * ((assign4020_e3957 * (locals.var_vcl_dn5 - locals.var_vl_dn5)) / assign4020_e3968)))),)
    } else {
        (locals.var_arga, locals.var_arga_dn1, locals.var_arga_dn3, locals.var_arga_dn4, locals.var_arga_dn5,)
    }
};
        locals.var_arga = assign4020_e3974;
        locals.var_arga_dn1 = assign4020_e3974_d_n1;
        locals.var_arga_dn3 = assign4020_e3974_d_n3;
        locals.var_arga_dn4 = assign4020_e3974_d_n4;
        locals.var_arga_dn5 = assign4020_e3974_d_n5;
        locals.var_arga_rv = 0.0;

        let (assign4030_e3981, assign4030_e3981_d_n1, assign4030_e3981_d_n3, assign4030_e3981_d_n4, assign4030_e3981_d_n5,) = {
    if ((locals.var_guard249 != 0.0) && (locals.var_guard254 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arga, locals.var_arga_dn1, locals.var_arga_dn3, locals.var_arga_dn4, locals.var_arga_dn5,)
    }
};
        locals.var_arga = assign4030_e3981;
        locals.var_arga_dn1 = assign4030_e3981_d_n1;
        locals.var_arga_dn3 = assign4030_e3981_d_n3;
        locals.var_arga_dn4 = assign4030_e3981_d_n4;
        locals.var_arga_dn5 = assign4030_e3981_d_n5;
        locals.var_arga_rv = 0.0;

        let assign4040_e3984: f64 = if locals.var_pcjp > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign4040_e3984;
        locals.var_guard267_rv = 0.0;

        let (assign4050_e3993, assign4050_e3993_d_n3,) = {
    if ((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign4050_e3989: f64 = (-locals.var_pp_t);
        let assign4050_e3991: f64 = (assign4050_e3989 * p.p68);
        (assign4050_e3991, ((-locals.var_pp_t_dn3) * p.p68),)
    } else {
        (locals.var_dv0__blk268, locals.var_dv0__blk268_dn3,)
    }
};
        locals.var_dv0__blk268 = assign4050_e3993;
        locals.var_dv0__blk268_dn3 = assign4050_e3993_d_n3;
        locals.var_dv0__blk268_rv = 0.0;

        let assign4060_e3996: f64 = if p.p82 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign4060_e3996;
        locals.var_guard278_rv = 0.0;

        let (assign4070_e4006, assign4070_e4006_d_n1, assign4070_e4006_d_n3, assign4070_e4006_d_n4, assign4070_e4006_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) {
        let assign4070_e4004: f64 = (locals.var_vcl + locals.var_dv0__blk268);
        (assign4070_e4004, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk268_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dvh__blk269, locals.var_dvh__blk269_dn1, locals.var_dvh__blk269_dn3, locals.var_dvh__blk269_dn4, locals.var_dvh__blk269_dn5,)
    }
};
        locals.var_dvh__blk269 = assign4070_e4006;
        locals.var_dvh__blk269_dn1 = assign4070_e4006_d_n1;
        locals.var_dvh__blk269_dn3 = assign4070_e4006_d_n3;
        locals.var_dvh__blk269_dn4 = assign4070_e4006_d_n4;
        locals.var_dvh__blk269_dn5 = assign4070_e4006_d_n5;
        locals.var_dvh__blk269_rv = 0.0;

        let assign4080_e4009: f64 = if locals.var_dvh__blk269 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign4080_e4009;
        locals.var_guard279_rv = 0.0;

        let (assign4090_e4024,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign4090_e4019: f64 = (1.0 - p.p68);
        let assign4090_e4021: f64 = (-p.p81);
        let assign4090_e4022: f64 = (assign4090_e4019).powf(assign4090_e4021);
        (assign4090_e4022,)
    } else {
        (locals.var_pwq__blk270,)
    }
};
        locals.var_pwq__blk270 = assign4090_e4024;
        locals.var_pwq__blk270_rv = 0.0;

        let (assign4100_e4046, assign4100_e4046_d_n1, assign4100_e4046_d_n3, assign4100_e4046_d_n4, assign4100_e4046_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign4100_e4037: f64 = (1.0 - p.p68);
        let assign4100_e4038: f64 = (locals.var_pwq__blk270 * assign4100_e4037);
        let assign4100_e4039: f64 = (1.0 - assign4100_e4038);
        let assign4100_e4040: f64 = (locals.var_pp_t * assign4100_e4039);
        let assign4100_e4043: f64 = (1.0 - p.p81);
        let assign4100_e4044: f64 = (assign4100_e4040 / assign4100_e4043);
        (assign4100_e4044, 0.0, ((locals.var_pp_t_dn3 * assign4100_e4039) / assign4100_e4043), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk271, locals.var_qlo__blk271_dn1, locals.var_qlo__blk271_dn3, locals.var_qlo__blk271_dn4, locals.var_qlo__blk271_dn5,)
    }
};
        locals.var_qlo__blk271 = assign4100_e4046;
        locals.var_qlo__blk271_dn1 = assign4100_e4046_d_n1;
        locals.var_qlo__blk271_dn3 = assign4100_e4046_d_n3;
        locals.var_qlo__blk271_dn4 = assign4100_e4046_d_n4;
        locals.var_qlo__blk271_dn5 = assign4100_e4046_d_n5;
        locals.var_qlo__blk271_rv = 0.0;

        let (assign4110_e4072, assign4110_e4072_d_n1, assign4110_e4072_d_n3, assign4110_e4072_d_n4, assign4110_e4072_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign4110_e4058: f64 = (0.5 * p.p81);
        let assign4110_e4060: f64 = (assign4110_e4058 * locals.var_dvh__blk269);
        let assign4110_e4064: f64 = (1.0 - p.p68);
        let assign4110_e4065: f64 = (locals.var_pp_t * assign4110_e4064);
        let assign4110_e4066: f64 = (assign4110_e4060 / assign4110_e4065);
        let assign4110_e4067: f64 = (1.0 + assign4110_e4066);
        let assign4110_e4068: f64 = (locals.var_dvh__blk269 * assign4110_e4067);
        let assign4110_e4070: f64 = (assign4110_e4068 * locals.var_pwq__blk270);
        (assign4110_e4070, (((locals.var_dvh__blk269_dn1 * assign4110_e4067) + (locals.var_dvh__blk269 * ((assign4110_e4058 * locals.var_dvh__blk269_dn1) / assign4110_e4065))) * locals.var_pwq__blk270), (((locals.var_dvh__blk269_dn3 * assign4110_e4067) + (locals.var_dvh__blk269 * ((((assign4110_e4058 * locals.var_dvh__blk269_dn3) * assign4110_e4065) - (assign4110_e4060 * (locals.var_pp_t_dn3 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * locals.var_pwq__blk270), (((locals.var_dvh__blk269_dn4 * assign4110_e4067) + (locals.var_dvh__blk269 * ((assign4110_e4058 * locals.var_dvh__blk269_dn4) / assign4110_e4065))) * locals.var_pwq__blk270), (((locals.var_dvh__blk269_dn5 * assign4110_e4067) + (locals.var_dvh__blk269 * ((assign4110_e4058 * locals.var_dvh__blk269_dn5) / assign4110_e4065))) * locals.var_pwq__blk270),)
    } else {
        (locals.var_qhi__blk272, locals.var_qhi__blk272_dn1, locals.var_qhi__blk272_dn3, locals.var_qhi__blk272_dn4, locals.var_qhi__blk272_dn5,)
    }
};
        locals.var_qhi__blk272 = assign4110_e4072;
        locals.var_qhi__blk272_dn1 = assign4110_e4072_d_n1;
        locals.var_qhi__blk272_dn3 = assign4110_e4072_d_n3;
        locals.var_qhi__blk272_dn4 = assign4110_e4072_d_n4;
        locals.var_qhi__blk272_dn5 = assign4110_e4072_d_n5;
        locals.var_qhi__blk272_rv = 0.0;

        let (assign4120_e4099, assign4120_e4099_d_n1, assign4120_e4099_d_n3, assign4120_e4099_d_n4, assign4120_e4099_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 == 0.0)) {
        let assign4120_e4086: f64 = (locals.var_vcl / locals.var_pp_t);
        let assign4120_e4087: f64 = (1.0 - assign4120_e4086);
        let assign4120_e4090: f64 = (1.0 - p.p81);
        let assign4120_e4091: f64 = (assign4120_e4087).powf(assign4120_e4090);
        let assign4120_e4092: f64 = (1.0 - assign4120_e4091);
        let assign4120_e4093: f64 = (locals.var_pp_t * assign4120_e4092);
        let assign4120_e4096: f64 = (1.0 - p.p81);
        let assign4120_e4097: f64 = (assign4120_e4093 / assign4120_e4096);
        (assign4120_e4097, ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(locals.var_vcl_dn1 / locals.var_pp_t)))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(locals.var_vcl_dn1 / locals.var_pp_t)) / assign4120_e4087))) })) / assign4120_e4096), (((locals.var_pp_t_dn3 * assign4120_e4092) + (locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((locals.var_vcl_dn3 * locals.var_pp_t) - (locals.var_vcl * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((locals.var_vcl_dn3 * locals.var_pp_t) - (locals.var_vcl * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(locals.var_vcl_dn4 / locals.var_pp_t)))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(locals.var_vcl_dn4 / locals.var_pp_t)) / assign4120_e4087))) })) / assign4120_e4096), ((locals.var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(locals.var_vcl_dn5 / locals.var_pp_t)))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(locals.var_vcl_dn5 / locals.var_pp_t)) / assign4120_e4087))) })) / assign4120_e4096),)
    } else {
        (locals.var_qlo__blk271, locals.var_qlo__blk271_dn1, locals.var_qlo__blk271_dn3, locals.var_qlo__blk271_dn4, locals.var_qlo__blk271_dn5,)
    }
};
        locals.var_qlo__blk271 = assign4120_e4099;
        locals.var_qlo__blk271_dn1 = assign4120_e4099_d_n1;
        locals.var_qlo__blk271_dn3 = assign4120_e4099_d_n3;
        locals.var_qlo__blk271_dn4 = assign4120_e4099_d_n4;
        locals.var_qlo__blk271_dn5 = assign4120_e4099_d_n5;
        locals.var_qlo__blk271_rv = 0.0;

        let (assign4130_e4110, assign4130_e4110_d_n1, assign4130_e4110_d_n3, assign4130_e4110_d_n4, assign4130_e4110_d_n5,) = {
    if ((((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk272, locals.var_qhi__blk272_dn1, locals.var_qhi__blk272_dn3, locals.var_qhi__blk272_dn4, locals.var_qhi__blk272_dn5,)
    }
};
        locals.var_qhi__blk272 = assign4130_e4110;
        locals.var_qhi__blk272_dn1 = assign4130_e4110_d_n1;
        locals.var_qhi__blk272_dn3 = assign4130_e4110_d_n3;
        locals.var_qhi__blk272_dn4 = assign4130_e4110_d_n4;
        locals.var_qhi__blk272_dn5 = assign4130_e4110_d_n5;
        locals.var_qhi__blk272_rv = 0.0;

        let (assign4140_e4120, assign4140_e4120_d_n1, assign4140_e4120_d_n3, assign4140_e4120_d_n4, assign4140_e4120_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 != 0.0)) {
        let assign4140_e4118: f64 = (locals.var_qlo__blk271 + locals.var_qhi__blk272);
        (assign4140_e4118, (locals.var_qlo__blk271_dn1 + locals.var_qhi__blk272_dn1), (locals.var_qlo__blk271_dn3 + locals.var_qhi__blk272_dn3), (locals.var_qlo__blk271_dn4 + locals.var_qhi__blk272_dn4), (locals.var_qlo__blk271_dn5 + locals.var_qhi__blk272_dn5),)
    } else {
        (locals.var_argp, locals.var_argp_dn1, locals.var_argp_dn3, locals.var_argp_dn4, locals.var_argp_dn5,)
    }
};
        locals.var_argp = assign4140_e4120;
        locals.var_argp_dn1 = assign4140_e4120_d_n1;
        locals.var_argp_dn3 = assign4140_e4120_d_n3;
        locals.var_argp_dn4 = assign4140_e4120_d_n4;
        locals.var_argp_dn5 = assign4140_e4120_d_n5;
        locals.var_argp_rv = 0.0;

        let (assign4150_e4138, assign4150_e4138_d_n3,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4150_e4129: f64 = (locals.var_dv0__blk268 * locals.var_dv0__blk268);
        let assign4150_e4132: f64 = (4.0 * p.p82);
        let assign4150_e4134: f64 = (assign4150_e4132 * p.p82);
        let assign4150_e4135: f64 = (assign4150_e4129 + assign4150_e4134);
        let assign4150_e4136: f64 = (assign4150_e4135).sqrt();
        (assign4150_e4136, (((locals.var_dv0__blk268_dn3 * locals.var_dv0__blk268) + (locals.var_dv0__blk268 * locals.var_dv0__blk268_dn3)) / (2.0 * assign4150_e4136)),)
    } else {
        (locals.var_mv0__blk273, locals.var_mv0__blk273_dn3,)
    }
};
        locals.var_mv0__blk273 = assign4150_e4138;
        locals.var_mv0__blk273_dn3 = assign4150_e4138_d_n3;
        locals.var_mv0__blk273_rv = 0.0;

        let (assign4160_e4152, assign4160_e4152_d_n3,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4160_e4146: f64 = (-0.5);
        let assign4160_e4149: f64 = (locals.var_dv0__blk268 + locals.var_mv0__blk273);
        let assign4160_e4150: f64 = (assign4160_e4146 * assign4160_e4149);
        (assign4160_e4150, (assign4160_e4146 * (locals.var_dv0__blk268_dn3 + locals.var_mv0__blk273_dn3)),)
    } else {
        (locals.var_vl0__blk274, locals.var_vl0__blk274_dn3,)
    }
};
        locals.var_vl0__blk274 = assign4160_e4152;
        locals.var_vl0__blk274_dn3 = assign4160_e4152_d_n3;
        locals.var_vl0__blk274_rv = 0.0;

        let (assign4170_e4163, assign4170_e4163_d_n1, assign4170_e4163_d_n3, assign4170_e4163_d_n4, assign4170_e4163_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4170_e4161: f64 = (locals.var_vcl + locals.var_dv0__blk268);
        (assign4170_e4161, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk268_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dv__blk275, locals.var_dv__blk275_dn1, locals.var_dv__blk275_dn3, locals.var_dv__blk275_dn4, locals.var_dv__blk275_dn5,)
    }
};
        locals.var_dv__blk275 = assign4170_e4163;
        locals.var_dv__blk275_dn1 = assign4170_e4163_d_n1;
        locals.var_dv__blk275_dn3 = assign4170_e4163_d_n3;
        locals.var_dv__blk275_dn4 = assign4170_e4163_d_n4;
        locals.var_dv__blk275_dn5 = assign4170_e4163_d_n5;
        locals.var_dv__blk275_rv = 0.0;

        let (assign4180_e4181, assign4180_e4181_d_n1, assign4180_e4181_d_n3, assign4180_e4181_d_n4, assign4180_e4181_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4180_e4172: f64 = (locals.var_dv__blk275 * locals.var_dv__blk275);
        let assign4180_e4175: f64 = (4.0 * p.p82);
        let assign4180_e4177: f64 = (assign4180_e4175 * p.p82);
        let assign4180_e4178: f64 = (assign4180_e4172 + assign4180_e4177);
        let assign4180_e4179: f64 = (assign4180_e4178).sqrt();
        (assign4180_e4179, (((locals.var_dv__blk275_dn1 * locals.var_dv__blk275) + (locals.var_dv__blk275 * locals.var_dv__blk275_dn1)) / (2.0 * assign4180_e4179)), (((locals.var_dv__blk275_dn3 * locals.var_dv__blk275) + (locals.var_dv__blk275 * locals.var_dv__blk275_dn3)) / (2.0 * assign4180_e4179)), (((locals.var_dv__blk275_dn4 * locals.var_dv__blk275) + (locals.var_dv__blk275 * locals.var_dv__blk275_dn4)) / (2.0 * assign4180_e4179)), (((locals.var_dv__blk275_dn5 * locals.var_dv__blk275) + (locals.var_dv__blk275 * locals.var_dv__blk275_dn5)) / (2.0 * assign4180_e4179)),)
    } else {
        (locals.var_mv__blk276, locals.var_mv__blk276_dn1, locals.var_mv__blk276_dn3, locals.var_mv__blk276_dn4, locals.var_mv__blk276_dn5,)
    }
};
        locals.var_mv__blk276 = assign4180_e4181;
        locals.var_mv__blk276_dn1 = assign4180_e4181_d_n1;
        locals.var_mv__blk276_dn3 = assign4180_e4181_d_n3;
        locals.var_mv__blk276_dn4 = assign4180_e4181_d_n4;
        locals.var_mv__blk276_dn5 = assign4180_e4181_d_n5;
        locals.var_mv__blk276_rv = 0.0;

        let (assign4190_e4196, assign4190_e4196_d_n1, assign4190_e4196_d_n3, assign4190_e4196_d_n4, assign4190_e4196_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4190_e4191: f64 = (locals.var_dv__blk275 - locals.var_mv__blk276);
        let assign4190_e4192: f64 = (0.5 * assign4190_e4191);
        let assign4190_e4194: f64 = (assign4190_e4192 - locals.var_dv0__blk268);
        (assign4190_e4194, (0.5 * (locals.var_dv__blk275_dn1 - locals.var_mv__blk276_dn1)), ((0.5 * (locals.var_dv__blk275_dn3 - locals.var_mv__blk276_dn3)) - locals.var_dv0__blk268_dn3), (0.5 * (locals.var_dv__blk275_dn4 - locals.var_mv__blk276_dn4)), (0.5 * (locals.var_dv__blk275_dn5 - locals.var_mv__blk276_dn5)),)
    } else {
        (locals.var_vl__blk277, locals.var_vl__blk277_dn1, locals.var_vl__blk277_dn3, locals.var_vl__blk277_dn4, locals.var_vl__blk277_dn5,)
    }
};
        locals.var_vl__blk277 = assign4190_e4196;
        locals.var_vl__blk277_dn1 = assign4190_e4196_d_n1;
        locals.var_vl__blk277_dn3 = assign4190_e4196_d_n3;
        locals.var_vl__blk277_dn4 = assign4190_e4196_d_n4;
        locals.var_vl__blk277_dn5 = assign4190_e4196_d_n5;
        locals.var_vl__blk277_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4200_e4220, assign4200_e4220_d_n1, assign4200_e4220_d_n3, assign4200_e4220_d_n4, assign4200_e4220_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4200_e4204: f64 = (-locals.var_pp_t);
        let assign4200_e4208: f64 = (locals.var_vl__blk277 / locals.var_pp_t);
        let assign4200_e4209: f64 = (1.0 - assign4200_e4208);
        let assign4200_e4212: f64 = (1.0 - p.p81);
        let assign4200_e4213: f64 = (assign4200_e4209).powf(assign4200_e4212);
        let assign4200_e4214: f64 = (assign4200_e4204 * assign4200_e4213);
        let assign4200_e4217: f64 = (1.0 - p.p81);
        let assign4200_e4218: f64 = (assign4200_e4214 / assign4200_e4217);
        (assign4200_e4218, ((assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(locals.var_vl__blk277_dn1 / locals.var_pp_t)))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(locals.var_vl__blk277_dn1 / locals.var_pp_t)) / assign4200_e4209))) }) / assign4200_e4217), ((((-locals.var_pp_t_dn3) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((locals.var_vl__blk277_dn3 * locals.var_pp_t) - (locals.var_vl__blk277 * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((locals.var_vl__blk277_dn3 * locals.var_pp_t) - (locals.var_vl__blk277 * locals.var_pp_t_dn3)) / (locals.var_pp_t * locals.var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(locals.var_vl__blk277_dn4 / locals.var_pp_t)))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(locals.var_vl__blk277_dn4 / locals.var_pp_t)) / assign4200_e4209))) }) / assign4200_e4217), ((assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(locals.var_vl__blk277_dn5 / locals.var_pp_t)))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(locals.var_vl__blk277_dn5 / locals.var_pp_t)) / assign4200_e4209))) }) / assign4200_e4217),)
    } else {
        (locals.var_qlo__blk271, locals.var_qlo__blk271_dn1, locals.var_qlo__blk271_dn3, locals.var_qlo__blk271_dn4, locals.var_qlo__blk271_dn5,)
    }
};
        locals.var_qlo__blk271 = assign4200_e4220;
        locals.var_qlo__blk271_dn1 = assign4200_e4220_d_n1;
        locals.var_qlo__blk271_dn3 = assign4200_e4220_d_n3;
        locals.var_qlo__blk271_dn4 = assign4200_e4220_d_n4;
        locals.var_qlo__blk271_dn5 = assign4200_e4220_d_n5;
        locals.var_qlo__blk271_rv = 0.0;

        let (assign4210_e4260, assign4210_e4260_d_n1, assign4210_e4260_d_n3, assign4210_e4260_d_n4, assign4210_e4260_d_n5,) = {
    if (((locals.var_guard249 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard278 == 0.0)) {
        let assign4210_e4230: f64 = (1.0 - p.p68);
        let assign4210_e4232: f64 = (-p.p81);
        let assign4210_e4233: f64 = (assign4210_e4230).powf(assign4210_e4232);
        let assign4210_e4236: f64 = (locals.var_vcl - locals.var_vl__blk277);
        let assign4210_e4238: f64 = (assign4210_e4236 + locals.var_vl0__blk274);
        let assign4210_e4239: f64 = (assign4210_e4233 * assign4210_e4238);
        let assign4210_e4243: f64 = (0.5 * p.p81);
        let assign4210_e4246: f64 = (locals.var_vcl - locals.var_vl__blk277);
        let assign4210_e4248: f64 = (assign4210_e4246 + locals.var_vl0__blk274);
        let assign4210_e4249: f64 = (assign4210_e4243 * assign4210_e4248);
        let assign4210_e4253: f64 = (1.0 - p.p68);
        let assign4210_e4254: f64 = (locals.var_pp_t * assign4210_e4253);
        let assign4210_e4255: f64 = (assign4210_e4249 / assign4210_e4254);
        let assign4210_e4256: f64 = (1.0 + assign4210_e4255);
        let assign4210_e4257: f64 = (assign4210_e4239 * assign4210_e4256);
        let assign4210_e4258: f64 = (locals.var_qlo__blk271 + assign4210_e4257);
        (assign4210_e4258, (locals.var_qlo__blk271_dn1 + (((assign4210_e4233 * (locals.var_vcl_dn1 - locals.var_vl__blk277_dn1)) * assign4210_e4256) + (assign4210_e4239 * ((assign4210_e4243 * (locals.var_vcl_dn1 - locals.var_vl__blk277_dn1)) / assign4210_e4254)))), (locals.var_qlo__blk271_dn3 + (((assign4210_e4233 * ((locals.var_vcl_dn3 - locals.var_vl__blk277_dn3) + locals.var_vl0__blk274_dn3)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((locals.var_vcl_dn3 - locals.var_vl__blk277_dn3) + locals.var_vl0__blk274_dn3)) * assign4210_e4254) - (assign4210_e4249 * (locals.var_pp_t_dn3 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (locals.var_qlo__blk271_dn4 + (((assign4210_e4233 * (locals.var_vcl_dn4 - locals.var_vl__blk277_dn4)) * assign4210_e4256) + (assign4210_e4239 * ((assign4210_e4243 * (locals.var_vcl_dn4 - locals.var_vl__blk277_dn4)) / assign4210_e4254)))), (locals.var_qlo__blk271_dn5 + (((assign4210_e4233 * (locals.var_vcl_dn5 - locals.var_vl__blk277_dn5)) * assign4210_e4256) + (assign4210_e4239 * ((assign4210_e4243 * (locals.var_vcl_dn5 - locals.var_vl__blk277_dn5)) / assign4210_e4254)))),)
    } else {
        (locals.var_argp, locals.var_argp_dn1, locals.var_argp_dn3, locals.var_argp_dn4, locals.var_argp_dn5,)
    }
};
        locals.var_argp = assign4210_e4260;
        locals.var_argp_dn1 = assign4210_e4260_d_n1;
        locals.var_argp_dn3 = assign4210_e4260_d_n3;
        locals.var_argp_dn4 = assign4210_e4260_d_n4;
        locals.var_argp_dn5 = assign4210_e4260_d_n5;
        locals.var_argp_rv = 0.0;

        let (assign4220_e4267, assign4220_e4267_d_n1, assign4220_e4267_d_n3, assign4220_e4267_d_n4, assign4220_e4267_d_n5,) = {
    if ((locals.var_guard249 != 0.0) && (locals.var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_argp, locals.var_argp_dn1, locals.var_argp_dn3, locals.var_argp_dn4, locals.var_argp_dn5,)
    }
};
        locals.var_argp = assign4220_e4267;
        locals.var_argp_dn1 = assign4220_e4267_d_n1;
        locals.var_argp_dn3 = assign4220_e4267_d_n3;
        locals.var_argp_dn4 = assign4220_e4267_d_n4;
        locals.var_argp_dn5 = assign4220_e4267_d_n5;
        locals.var_argp_rv = 0.0;

        let (assign4230_e4277, assign4230_e4277_d_n1, assign4230_e4277_d_n3, assign4230_e4277_d_n4, assign4230_e4277_d_n5,) = {
    if (locals.var_guard249 != 0.0) {
        let assign4230_e4271: f64 = (locals.var_acja * locals.var_arga);
        let assign4230_e4274: f64 = (locals.var_pcjp * locals.var_argp);
        let assign4230_e4275: f64 = (assign4230_e4271 + assign4230_e4274);
        (assign4230_e4275, (((locals.var_acja_dn1 * locals.var_arga) + (locals.var_acja * locals.var_arga_dn1)) + (locals.var_pcjp * locals.var_argp_dn1)), (((locals.var_acja_dn3 * locals.var_arga) + (locals.var_acja * locals.var_arga_dn3)) + ((locals.var_pcjp_dn3 * locals.var_argp) + (locals.var_pcjp * locals.var_argp_dn3))), (((locals.var_acja_dn4 * locals.var_arga) + (locals.var_acja * locals.var_arga_dn4)) + (locals.var_pcjp * locals.var_argp_dn4)), (((locals.var_acja_dn5 * locals.var_arga) + (locals.var_acja * locals.var_arga_dn5)) + (locals.var_pcjp * locals.var_argp_dn5)),)
    } else {
        (locals.var_qcp1, locals.var_qcp1_dn1, locals.var_qcp1_dn3, locals.var_qcp1_dn4, locals.var_qcp1_dn5,)
    }
};
        locals.var_qcp1 = assign4230_e4277;
        locals.var_qcp1_dn1 = assign4230_e4277_d_n1;
        locals.var_qcp1_dn3 = assign4230_e4277_d_n3;
        locals.var_qcp1_dn4 = assign4230_e4277_d_n4;
        locals.var_qcp1_dn5 = assign4230_e4277_d_n5;
        locals.var_qcp1_rv = 0.0;

        let (assign4240_e4282, assign4240_e4282_d_n1, assign4240_e4282_d_n3, assign4240_e4282_d_n4, assign4240_e4282_d_n5,) = {
    if (locals.var_guard249 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qcp1, locals.var_qcp1_dn1, locals.var_qcp1_dn3, locals.var_qcp1_dn4, locals.var_qcp1_dn5,)
    }
};
        locals.var_qcp1 = assign4240_e4282;
        locals.var_qcp1_dn1 = assign4240_e4282_d_n1;
        locals.var_qcp1_dn3 = assign4240_e4282_d_n3;
        locals.var_qcp1_dn4 = assign4240_e4282_d_n4;
        locals.var_qcp1_dn5 = assign4240_e4282_d_n5;
        locals.var_qcp1_rv = 0.0;

        let assign4250_e4285: f64 = if locals.var_cj2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign4250_e4285;
        locals.var_guard280_rv = 0.0;

        let (assign4260_e4306, assign4260_e4306_d_n1, assign4260_e4306_d_n3, assign4260_e4306_d_n4, assign4260_e4306_d_n5,) = {
    if ((locals.var_guard280 != 0.0) && (p.p63 != 0.0)) {
        let assign4260_e4292: f64 = (locals.var_vc2 - locals.var_vpo);
        let assign4260_e4295: f64 = (locals.var_vc2 + locals.var_vpo);
        let assign4260_e4298: f64 = (locals.var_vc2 + locals.var_vpo);
        let assign4260_e4299: f64 = (assign4260_e4295 * assign4260_e4298);
        let assign4260_e4301: f64 = (assign4260_e4299 + 0.04);
        let assign4260_e4302: f64 = (assign4260_e4301).sqrt();
        let assign4260_e4303: f64 = (assign4260_e4292 + assign4260_e4302);
        let assign4260_e4304: f64 = (0.5 * assign4260_e4303);
        (assign4260_e4304, (0.5 * (locals.var_vc2_dn1 + (((locals.var_vc2_dn1 * assign4260_e4298) + (assign4260_e4295 * locals.var_vc2_dn1)) / (2.0 * assign4260_e4302)))), (0.5 * ((-locals.var_vpo_dn3) + (((locals.var_vpo_dn3 * assign4260_e4298) + (assign4260_e4295 * locals.var_vpo_dn3)) / (2.0 * assign4260_e4302)))), 0.0, (0.5 * (locals.var_vc2_dn5 + (((locals.var_vc2_dn5 * assign4260_e4298) + (assign4260_e4295 * locals.var_vc2_dn5)) / (2.0 * assign4260_e4302)))),)
    } else {
        (locals.var_vcl, locals.var_vcl_dn1, locals.var_vcl_dn3, locals.var_vcl_dn4, locals.var_vcl_dn5,)
    }
};
        locals.var_vcl = assign4260_e4306;
        locals.var_vcl_dn1 = assign4260_e4306_d_n1;
        locals.var_vcl_dn3 = assign4260_e4306_d_n3;
        locals.var_vcl_dn4 = assign4260_e4306_d_n4;
        locals.var_vcl_dn5 = assign4260_e4306_d_n5;
        locals.var_vcl_rv = 0.0;

        let (assign4270_e4313, assign4270_e4313_d_n1, assign4270_e4313_d_n3, assign4270_e4313_d_n4, assign4270_e4313_d_n5,) = {
    if ((locals.var_guard280 != 0.0) && (p.p63 == 0.0)) {
        (locals.var_vc2, locals.var_vc2_dn1, 0.0, 0.0, locals.var_vc2_dn5,)
    } else {
        (locals.var_vcl, locals.var_vcl_dn1, locals.var_vcl_dn3, locals.var_vcl_dn4, locals.var_vcl_dn5,)
    }
};
        locals.var_vcl = assign4270_e4313;
        locals.var_vcl_dn1 = assign4270_e4313_d_n1;
        locals.var_vcl_dn3 = assign4270_e4313_d_n3;
        locals.var_vcl_dn4 = assign4270_e4313_d_n4;
        locals.var_vcl_dn5 = assign4270_e4313_d_n5;
        locals.var_vcl_rv = 0.0;

        let (assign4280_e4319, assign4280_e4319_d_n1, assign4280_e4319_d_n3, assign4280_e4319_d_n4, assign4280_e4319_d_n5,) = {
    if (locals.var_guard280 != 0.0) {
        let assign4280_e4317: f64 = (locals.var_a2_um2 * locals.var_cja_t);
        (assign4280_e4317, (locals.var_a2_um2_dn1 * locals.var_cja_t), ((locals.var_a2_um2_dn3 * locals.var_cja_t) + (locals.var_a2_um2 * locals.var_cja_t_dn3)), (locals.var_a2_um2_dn4 * locals.var_cja_t), (locals.var_a2_um2_dn5 * locals.var_cja_t),)
    } else {
        (locals.var_acja__blk281, locals.var_acja__blk281_dn1, locals.var_acja__blk281_dn3, locals.var_acja__blk281_dn4, locals.var_acja__blk281_dn5,)
    }
};
        locals.var_acja__blk281 = assign4280_e4319;
        locals.var_acja__blk281_dn1 = assign4280_e4319_d_n1;
        locals.var_acja__blk281_dn3 = assign4280_e4319_d_n3;
        locals.var_acja__blk281_dn4 = assign4280_e4319_d_n4;
        locals.var_acja__blk281_dn5 = assign4280_e4319_d_n5;
        locals.var_acja__blk281_rv = 0.0;

        let (assign4290_e4325, assign4290_e4325_d_n3,) = {
    if (locals.var_guard280 != 0.0) {
        let assign4290_e4323: f64 = (locals.var_p2_um * locals.var_cjp_t);
        (assign4290_e4323, (locals.var_p2_um * locals.var_cjp_t_dn3),)
    } else {
        (locals.var_pcjp__blk282, locals.var_pcjp__blk282_dn3,)
    }
};
        locals.var_pcjp__blk282 = assign4290_e4325;
        locals.var_pcjp__blk282_dn3 = assign4290_e4325_d_n3;
        locals.var_pcjp__blk282_rv = 0.0;

        let assign4300_e4328: f64 = if locals.var_acja__blk281 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign4300_e4328;
        locals.var_guard285_rv = 0.0;

        let (assign4310_e4337, assign4310_e4337_d_n3,) = {
    if ((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) {
        let assign4310_e4333: f64 = (-locals.var_pa_t);
        let assign4310_e4335: f64 = (assign4310_e4333 * p.p68);
        (assign4310_e4335, ((-locals.var_pa_t_dn3) * p.p68),)
    } else {
        (locals.var_dv0__blk286, locals.var_dv0__blk286_dn3,)
    }
};
        locals.var_dv0__blk286 = assign4310_e4337;
        locals.var_dv0__blk286_dn3 = assign4310_e4337_d_n3;
        locals.var_dv0__blk286_rv = 0.0;

        let assign4320_e4340: f64 = if p.p75 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign4320_e4340;
        locals.var_guard296_rv = 0.0;

        let (assign4330_e4350, assign4330_e4350_d_n1, assign4330_e4350_d_n3, assign4330_e4350_d_n4, assign4330_e4350_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) {
        let assign4330_e4348: f64 = (locals.var_vcl + locals.var_dv0__blk286);
        (assign4330_e4348, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk286_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dvh__blk287, locals.var_dvh__blk287_dn1, locals.var_dvh__blk287_dn3, locals.var_dvh__blk287_dn4, locals.var_dvh__blk287_dn5,)
    }
};
        locals.var_dvh__blk287 = assign4330_e4350;
        locals.var_dvh__blk287_dn1 = assign4330_e4350_d_n1;
        locals.var_dvh__blk287_dn3 = assign4330_e4350_d_n3;
        locals.var_dvh__blk287_dn4 = assign4330_e4350_d_n4;
        locals.var_dvh__blk287_dn5 = assign4330_e4350_d_n5;
        locals.var_dvh__blk287_rv = 0.0;

        let assign4340_e4353: f64 = if locals.var_dvh__blk287 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign4340_e4353;
        locals.var_guard297_rv = 0.0;

        let (assign4350_e4368,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 != 0.0)) {
        let assign4350_e4363: f64 = (1.0 - p.p68);
        let assign4350_e4365: f64 = (-p.p74);
        let assign4350_e4366: f64 = (assign4350_e4363).powf(assign4350_e4365);
        (assign4350_e4366,)
    } else {
        (locals.var_pwq__blk288,)
    }
};
        locals.var_pwq__blk288 = assign4350_e4368;
        locals.var_pwq__blk288_rv = 0.0;

        let (assign4360_e4390, assign4360_e4390_d_n1, assign4360_e4390_d_n3, assign4360_e4390_d_n4, assign4360_e4390_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 != 0.0)) {
        let assign4360_e4381: f64 = (1.0 - p.p68);
        let assign4360_e4382: f64 = (locals.var_pwq__blk288 * assign4360_e4381);
        let assign4360_e4383: f64 = (1.0 - assign4360_e4382);
        let assign4360_e4384: f64 = (locals.var_pa_t * assign4360_e4383);
        let assign4360_e4387: f64 = (1.0 - p.p74);
        let assign4360_e4388: f64 = (assign4360_e4384 / assign4360_e4387);
        (assign4360_e4388, 0.0, ((locals.var_pa_t_dn3 * assign4360_e4383) / assign4360_e4387), 0.0, 0.0,)
    } else {
        (locals.var_qlo__blk289, locals.var_qlo__blk289_dn1, locals.var_qlo__blk289_dn3, locals.var_qlo__blk289_dn4, locals.var_qlo__blk289_dn5,)
    }
};
        locals.var_qlo__blk289 = assign4360_e4390;
        locals.var_qlo__blk289_dn1 = assign4360_e4390_d_n1;
        locals.var_qlo__blk289_dn3 = assign4360_e4390_d_n3;
        locals.var_qlo__blk289_dn4 = assign4360_e4390_d_n4;
        locals.var_qlo__blk289_dn5 = assign4360_e4390_d_n5;
        locals.var_qlo__blk289_rv = 0.0;

        let (assign4370_e4416, assign4370_e4416_d_n1, assign4370_e4416_d_n3, assign4370_e4416_d_n4, assign4370_e4416_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 != 0.0)) {
        let assign4370_e4402: f64 = (0.5 * p.p74);
        let assign4370_e4404: f64 = (assign4370_e4402 * locals.var_dvh__blk287);
        let assign4370_e4408: f64 = (1.0 - p.p68);
        let assign4370_e4409: f64 = (locals.var_pa_t * assign4370_e4408);
        let assign4370_e4410: f64 = (assign4370_e4404 / assign4370_e4409);
        let assign4370_e4411: f64 = (1.0 + assign4370_e4410);
        let assign4370_e4412: f64 = (locals.var_dvh__blk287 * assign4370_e4411);
        let assign4370_e4414: f64 = (assign4370_e4412 * locals.var_pwq__blk288);
        (assign4370_e4414, (((locals.var_dvh__blk287_dn1 * assign4370_e4411) + (locals.var_dvh__blk287 * ((assign4370_e4402 * locals.var_dvh__blk287_dn1) / assign4370_e4409))) * locals.var_pwq__blk288), (((locals.var_dvh__blk287_dn3 * assign4370_e4411) + (locals.var_dvh__blk287 * ((((assign4370_e4402 * locals.var_dvh__blk287_dn3) * assign4370_e4409) - (assign4370_e4404 * (locals.var_pa_t_dn3 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * locals.var_pwq__blk288), (((locals.var_dvh__blk287_dn4 * assign4370_e4411) + (locals.var_dvh__blk287 * ((assign4370_e4402 * locals.var_dvh__blk287_dn4) / assign4370_e4409))) * locals.var_pwq__blk288), (((locals.var_dvh__blk287_dn5 * assign4370_e4411) + (locals.var_dvh__blk287 * ((assign4370_e4402 * locals.var_dvh__blk287_dn5) / assign4370_e4409))) * locals.var_pwq__blk288),)
    } else {
        (locals.var_qhi__blk290, locals.var_qhi__blk290_dn1, locals.var_qhi__blk290_dn3, locals.var_qhi__blk290_dn4, locals.var_qhi__blk290_dn5,)
    }
};
        locals.var_qhi__blk290 = assign4370_e4416;
        locals.var_qhi__blk290_dn1 = assign4370_e4416_d_n1;
        locals.var_qhi__blk290_dn3 = assign4370_e4416_d_n3;
        locals.var_qhi__blk290_dn4 = assign4370_e4416_d_n4;
        locals.var_qhi__blk290_dn5 = assign4370_e4416_d_n5;
        locals.var_qhi__blk290_rv = 0.0;

        let (assign4380_e4443, assign4380_e4443_d_n1, assign4380_e4443_d_n3, assign4380_e4443_d_n4, assign4380_e4443_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 == 0.0)) {
        let assign4380_e4430: f64 = (locals.var_vcl / locals.var_pa_t);
        let assign4380_e4431: f64 = (1.0 - assign4380_e4430);
        let assign4380_e4434: f64 = (1.0 - p.p74);
        let assign4380_e4435: f64 = (assign4380_e4431).powf(assign4380_e4434);
        let assign4380_e4436: f64 = (1.0 - assign4380_e4435);
        let assign4380_e4437: f64 = (locals.var_pa_t * assign4380_e4436);
        let assign4380_e4440: f64 = (1.0 - p.p74);
        let assign4380_e4441: f64 = (assign4380_e4437 / assign4380_e4440);
        (assign4380_e4441, ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(locals.var_vcl_dn1 / locals.var_pa_t)))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(locals.var_vcl_dn1 / locals.var_pa_t)) / assign4380_e4431))) })) / assign4380_e4440), (((locals.var_pa_t_dn3 * assign4380_e4436) + (locals.var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((locals.var_vcl_dn3 * locals.var_pa_t) - (locals.var_vcl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((locals.var_vcl_dn3 * locals.var_pa_t) - (locals.var_vcl * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(locals.var_vcl_dn4 / locals.var_pa_t)))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(locals.var_vcl_dn4 / locals.var_pa_t)) / assign4380_e4431))) })) / assign4380_e4440), ((locals.var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(locals.var_vcl_dn5 / locals.var_pa_t)))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(locals.var_vcl_dn5 / locals.var_pa_t)) / assign4380_e4431))) })) / assign4380_e4440),)
    } else {
        (locals.var_qlo__blk289, locals.var_qlo__blk289_dn1, locals.var_qlo__blk289_dn3, locals.var_qlo__blk289_dn4, locals.var_qlo__blk289_dn5,)
    }
};
        locals.var_qlo__blk289 = assign4380_e4443;
        locals.var_qlo__blk289_dn1 = assign4380_e4443_d_n1;
        locals.var_qlo__blk289_dn3 = assign4380_e4443_d_n3;
        locals.var_qlo__blk289_dn4 = assign4380_e4443_d_n4;
        locals.var_qlo__blk289_dn5 = assign4380_e4443_d_n5;
        locals.var_qlo__blk289_rv = 0.0;

        let (assign4390_e4454, assign4390_e4454_d_n1, assign4390_e4454_d_n3, assign4390_e4454_d_n4, assign4390_e4454_d_n5,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) && (locals.var_guard297 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qhi__blk290, locals.var_qhi__blk290_dn1, locals.var_qhi__blk290_dn3, locals.var_qhi__blk290_dn4, locals.var_qhi__blk290_dn5,)
    }
};
        locals.var_qhi__blk290 = assign4390_e4454;
        locals.var_qhi__blk290_dn1 = assign4390_e4454_d_n1;
        locals.var_qhi__blk290_dn3 = assign4390_e4454_d_n3;
        locals.var_qhi__blk290_dn4 = assign4390_e4454_d_n4;
        locals.var_qhi__blk290_dn5 = assign4390_e4454_d_n5;
        locals.var_qhi__blk290_rv = 0.0;

        let (assign4400_e4464, assign4400_e4464_d_n1, assign4400_e4464_d_n3, assign4400_e4464_d_n4, assign4400_e4464_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 != 0.0)) {
        let assign4400_e4462: f64 = (locals.var_qlo__blk289 + locals.var_qhi__blk290);
        (assign4400_e4462, (locals.var_qlo__blk289_dn1 + locals.var_qhi__blk290_dn1), (locals.var_qlo__blk289_dn3 + locals.var_qhi__blk290_dn3), (locals.var_qlo__blk289_dn4 + locals.var_qhi__blk290_dn4), (locals.var_qlo__blk289_dn5 + locals.var_qhi__blk290_dn5),)
    } else {
        (locals.var_arga__blk283, locals.var_arga__blk283_dn1, locals.var_arga__blk283_dn3, locals.var_arga__blk283_dn4, locals.var_arga__blk283_dn5,)
    }
};
        locals.var_arga__blk283 = assign4400_e4464;
        locals.var_arga__blk283_dn1 = assign4400_e4464_d_n1;
        locals.var_arga__blk283_dn3 = assign4400_e4464_d_n3;
        locals.var_arga__blk283_dn4 = assign4400_e4464_d_n4;
        locals.var_arga__blk283_dn5 = assign4400_e4464_d_n5;
        locals.var_arga__blk283_rv = 0.0;

        let (assign4410_e4482, assign4410_e4482_d_n3,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4410_e4473: f64 = (locals.var_dv0__blk286 * locals.var_dv0__blk286);
        let assign4410_e4476: f64 = (4.0 * p.p75);
        let assign4410_e4478: f64 = (assign4410_e4476 * p.p75);
        let assign4410_e4479: f64 = (assign4410_e4473 + assign4410_e4478);
        let assign4410_e4480: f64 = (assign4410_e4479).sqrt();
        (assign4410_e4480, (((locals.var_dv0__blk286_dn3 * locals.var_dv0__blk286) + (locals.var_dv0__blk286 * locals.var_dv0__blk286_dn3)) / (2.0 * assign4410_e4480)),)
    } else {
        (locals.var_mv0__blk291, locals.var_mv0__blk291_dn3,)
    }
};
        locals.var_mv0__blk291 = assign4410_e4482;
        locals.var_mv0__blk291_dn3 = assign4410_e4482_d_n3;
        locals.var_mv0__blk291_rv = 0.0;

        let (assign4420_e4496, assign4420_e4496_d_n3,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4420_e4490: f64 = (-0.5);
        let assign4420_e4493: f64 = (locals.var_dv0__blk286 + locals.var_mv0__blk291);
        let assign4420_e4494: f64 = (assign4420_e4490 * assign4420_e4493);
        (assign4420_e4494, (assign4420_e4490 * (locals.var_dv0__blk286_dn3 + locals.var_mv0__blk291_dn3)),)
    } else {
        (locals.var_vl0__blk292, locals.var_vl0__blk292_dn3,)
    }
};
        locals.var_vl0__blk292 = assign4420_e4496;
        locals.var_vl0__blk292_dn3 = assign4420_e4496_d_n3;
        locals.var_vl0__blk292_rv = 0.0;

        let (assign4430_e4507, assign4430_e4507_d_n1, assign4430_e4507_d_n3, assign4430_e4507_d_n4, assign4430_e4507_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4430_e4505: f64 = (locals.var_vcl + locals.var_dv0__blk286);
        (assign4430_e4505, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk286_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dv__blk293, locals.var_dv__blk293_dn1, locals.var_dv__blk293_dn3, locals.var_dv__blk293_dn4, locals.var_dv__blk293_dn5,)
    }
};
        locals.var_dv__blk293 = assign4430_e4507;
        locals.var_dv__blk293_dn1 = assign4430_e4507_d_n1;
        locals.var_dv__blk293_dn3 = assign4430_e4507_d_n3;
        locals.var_dv__blk293_dn4 = assign4430_e4507_d_n4;
        locals.var_dv__blk293_dn5 = assign4430_e4507_d_n5;
        locals.var_dv__blk293_rv = 0.0;

        let (assign4440_e4525, assign4440_e4525_d_n1, assign4440_e4525_d_n3, assign4440_e4525_d_n4, assign4440_e4525_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4440_e4516: f64 = (locals.var_dv__blk293 * locals.var_dv__blk293);
        let assign4440_e4519: f64 = (4.0 * p.p75);
        let assign4440_e4521: f64 = (assign4440_e4519 * p.p75);
        let assign4440_e4522: f64 = (assign4440_e4516 + assign4440_e4521);
        let assign4440_e4523: f64 = (assign4440_e4522).sqrt();
        (assign4440_e4523, (((locals.var_dv__blk293_dn1 * locals.var_dv__blk293) + (locals.var_dv__blk293 * locals.var_dv__blk293_dn1)) / (2.0 * assign4440_e4523)), (((locals.var_dv__blk293_dn3 * locals.var_dv__blk293) + (locals.var_dv__blk293 * locals.var_dv__blk293_dn3)) / (2.0 * assign4440_e4523)), (((locals.var_dv__blk293_dn4 * locals.var_dv__blk293) + (locals.var_dv__blk293 * locals.var_dv__blk293_dn4)) / (2.0 * assign4440_e4523)), (((locals.var_dv__blk293_dn5 * locals.var_dv__blk293) + (locals.var_dv__blk293 * locals.var_dv__blk293_dn5)) / (2.0 * assign4440_e4523)),)
    } else {
        (locals.var_mv__blk294, locals.var_mv__blk294_dn1, locals.var_mv__blk294_dn3, locals.var_mv__blk294_dn4, locals.var_mv__blk294_dn5,)
    }
};
        locals.var_mv__blk294 = assign4440_e4525;
        locals.var_mv__blk294_dn1 = assign4440_e4525_d_n1;
        locals.var_mv__blk294_dn3 = assign4440_e4525_d_n3;
        locals.var_mv__blk294_dn4 = assign4440_e4525_d_n4;
        locals.var_mv__blk294_dn5 = assign4440_e4525_d_n5;
        locals.var_mv__blk294_rv = 0.0;

        let (assign4450_e4540, assign4450_e4540_d_n1, assign4450_e4540_d_n3, assign4450_e4540_d_n4, assign4450_e4540_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4450_e4535: f64 = (locals.var_dv__blk293 - locals.var_mv__blk294);
        let assign4450_e4536: f64 = (0.5 * assign4450_e4535);
        let assign4450_e4538: f64 = (assign4450_e4536 - locals.var_dv0__blk286);
        (assign4450_e4538, (0.5 * (locals.var_dv__blk293_dn1 - locals.var_mv__blk294_dn1)), ((0.5 * (locals.var_dv__blk293_dn3 - locals.var_mv__blk294_dn3)) - locals.var_dv0__blk286_dn3), (0.5 * (locals.var_dv__blk293_dn4 - locals.var_mv__blk294_dn4)), (0.5 * (locals.var_dv__blk293_dn5 - locals.var_mv__blk294_dn5)),)
    } else {
        (locals.var_vl__blk295, locals.var_vl__blk295_dn1, locals.var_vl__blk295_dn3, locals.var_vl__blk295_dn4, locals.var_vl__blk295_dn5,)
    }
};
        locals.var_vl__blk295 = assign4450_e4540;
        locals.var_vl__blk295_dn1 = assign4450_e4540_d_n1;
        locals.var_vl__blk295_dn3 = assign4450_e4540_d_n3;
        locals.var_vl__blk295_dn4 = assign4450_e4540_d_n4;
        locals.var_vl__blk295_dn5 = assign4450_e4540_d_n5;
        locals.var_vl__blk295_rv = 0.0;

        let (assign4460_e4564, assign4460_e4564_d_n1, assign4460_e4564_d_n3, assign4460_e4564_d_n4, assign4460_e4564_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4460_e4548: f64 = (-locals.var_pa_t);
        let assign4460_e4552: f64 = (locals.var_vl__blk295 / locals.var_pa_t);
        let assign4460_e4553: f64 = (1.0 - assign4460_e4552);
        let assign4460_e4556: f64 = (1.0 - p.p74);
        let assign4460_e4557: f64 = (assign4460_e4553).powf(assign4460_e4556);
        let assign4460_e4558: f64 = (assign4460_e4548 * assign4460_e4557);
        let assign4460_e4561: f64 = (1.0 - p.p74);
        let assign4460_e4562: f64 = (assign4460_e4558 / assign4460_e4561);
        (assign4460_e4562, ((assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(locals.var_vl__blk295_dn1 / locals.var_pa_t)))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(locals.var_vl__blk295_dn1 / locals.var_pa_t)) / assign4460_e4553))) }) / assign4460_e4561), ((((-locals.var_pa_t_dn3) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((locals.var_vl__blk295_dn3 * locals.var_pa_t) - (locals.var_vl__blk295 * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((locals.var_vl__blk295_dn3 * locals.var_pa_t) - (locals.var_vl__blk295 * locals.var_pa_t_dn3)) / (locals.var_pa_t * locals.var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(locals.var_vl__blk295_dn4 / locals.var_pa_t)))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(locals.var_vl__blk295_dn4 / locals.var_pa_t)) / assign4460_e4553))) }) / assign4460_e4561), ((assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(locals.var_vl__blk295_dn5 / locals.var_pa_t)))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(locals.var_vl__blk295_dn5 / locals.var_pa_t)) / assign4460_e4553))) }) / assign4460_e4561),)
    } else {
        (locals.var_qlo__blk289, locals.var_qlo__blk289_dn1, locals.var_qlo__blk289_dn3, locals.var_qlo__blk289_dn4, locals.var_qlo__blk289_dn5,)
    }
};
        locals.var_qlo__blk289 = assign4460_e4564;
        locals.var_qlo__blk289_dn1 = assign4460_e4564_d_n1;
        locals.var_qlo__blk289_dn3 = assign4460_e4564_d_n3;
        locals.var_qlo__blk289_dn4 = assign4460_e4564_d_n4;
        locals.var_qlo__blk289_dn5 = assign4460_e4564_d_n5;
        locals.var_qlo__blk289_rv = 0.0;

        let (assign4470_e4604, assign4470_e4604_d_n1, assign4470_e4604_d_n3, assign4470_e4604_d_n4, assign4470_e4604_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard285 != 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign4470_e4574: f64 = (1.0 - p.p68);
        let assign4470_e4576: f64 = (-p.p74);
        let assign4470_e4577: f64 = (assign4470_e4574).powf(assign4470_e4576);
        let assign4470_e4580: f64 = (locals.var_vcl - locals.var_vl__blk295);
        let assign4470_e4582: f64 = (assign4470_e4580 + locals.var_vl0__blk292);
        let assign4470_e4583: f64 = (assign4470_e4577 * assign4470_e4582);
        let assign4470_e4587: f64 = (0.5 * p.p74);
        let assign4470_e4590: f64 = (locals.var_vcl - locals.var_vl__blk295);
        let assign4470_e4592: f64 = (assign4470_e4590 + locals.var_vl0__blk292);
        let assign4470_e4593: f64 = (assign4470_e4587 * assign4470_e4592);
        let assign4470_e4597: f64 = (1.0 - p.p68);
        let assign4470_e4598: f64 = (locals.var_pa_t * assign4470_e4597);
        let assign4470_e4599: f64 = (assign4470_e4593 / assign4470_e4598);
        let assign4470_e4600: f64 = (1.0 + assign4470_e4599);
        let assign4470_e4601: f64 = (assign4470_e4583 * assign4470_e4600);
        let assign4470_e4602: f64 = (locals.var_qlo__blk289 + assign4470_e4601);
        (assign4470_e4602, (locals.var_qlo__blk289_dn1 + (((assign4470_e4577 * (locals.var_vcl_dn1 - locals.var_vl__blk295_dn1)) * assign4470_e4600) + (assign4470_e4583 * ((assign4470_e4587 * (locals.var_vcl_dn1 - locals.var_vl__blk295_dn1)) / assign4470_e4598)))), (locals.var_qlo__blk289_dn3 + (((assign4470_e4577 * ((locals.var_vcl_dn3 - locals.var_vl__blk295_dn3) + locals.var_vl0__blk292_dn3)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((locals.var_vcl_dn3 - locals.var_vl__blk295_dn3) + locals.var_vl0__blk292_dn3)) * assign4470_e4598) - (assign4470_e4593 * (locals.var_pa_t_dn3 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (locals.var_qlo__blk289_dn4 + (((assign4470_e4577 * (locals.var_vcl_dn4 - locals.var_vl__blk295_dn4)) * assign4470_e4600) + (assign4470_e4583 * ((assign4470_e4587 * (locals.var_vcl_dn4 - locals.var_vl__blk295_dn4)) / assign4470_e4598)))), (locals.var_qlo__blk289_dn5 + (((assign4470_e4577 * (locals.var_vcl_dn5 - locals.var_vl__blk295_dn5)) * assign4470_e4600) + (assign4470_e4583 * ((assign4470_e4587 * (locals.var_vcl_dn5 - locals.var_vl__blk295_dn5)) / assign4470_e4598)))),)
    } else {
        (locals.var_arga__blk283, locals.var_arga__blk283_dn1, locals.var_arga__blk283_dn3, locals.var_arga__blk283_dn4, locals.var_arga__blk283_dn5,)
    }
};
        locals.var_arga__blk283 = assign4470_e4604;
        locals.var_arga__blk283_dn1 = assign4470_e4604_d_n1;
        locals.var_arga__blk283_dn3 = assign4470_e4604_d_n3;
        locals.var_arga__blk283_dn4 = assign4470_e4604_d_n4;
        locals.var_arga__blk283_dn5 = assign4470_e4604_d_n5;
        locals.var_arga__blk283_rv = 0.0;

        let (assign4480_e4611, assign4480_e4611_d_n1, assign4480_e4611_d_n3, assign4480_e4611_d_n4, assign4480_e4611_d_n5,) = {
    if ((locals.var_guard280 != 0.0) && (locals.var_guard285 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arga__blk283, locals.var_arga__blk283_dn1, locals.var_arga__blk283_dn3, locals.var_arga__blk283_dn4, locals.var_arga__blk283_dn5,)
    }
};
        locals.var_arga__blk283 = assign4480_e4611;
        locals.var_arga__blk283_dn1 = assign4480_e4611_d_n1;
        locals.var_arga__blk283_dn3 = assign4480_e4611_d_n3;
        locals.var_arga__blk283_dn4 = assign4480_e4611_d_n4;
        locals.var_arga__blk283_dn5 = assign4480_e4611_d_n5;
        locals.var_arga__blk283_rv = 0.0;

        let assign4490_e4614: f64 = if locals.var_pcjp__blk282 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign4490_e4614;
        locals.var_guard298_rv = 0.0;

        let (assign4500_e4623, assign4500_e4623_d_n3,) = {
    if ((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign4500_e4619: f64 = (-locals.var_pp_t);
        let assign4500_e4621: f64 = (assign4500_e4619 * p.p68);
        (assign4500_e4621, ((-locals.var_pp_t_dn3) * p.p68),)
    } else {
        (locals.var_dv0__blk299, locals.var_dv0__blk299_dn3,)
    }
};
        locals.var_dv0__blk299 = assign4500_e4623;
        locals.var_dv0__blk299_dn3 = assign4500_e4623_d_n3;
        locals.var_dv0__blk299_rv = 0.0;

        let assign4510_e4626: f64 = if p.p82 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign4510_e4626;
        locals.var_guard309_rv = 0.0;

        let (assign4520_e4636, assign4520_e4636_d_n1, assign4520_e4636_d_n3, assign4520_e4636_d_n4, assign4520_e4636_d_n5,) = {
    if (((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) {
        let assign4520_e4634: f64 = (locals.var_vcl + locals.var_dv0__blk299);
        (assign4520_e4634, locals.var_vcl_dn1, (locals.var_vcl_dn3 + locals.var_dv0__blk299_dn3), locals.var_vcl_dn4, locals.var_vcl_dn5,)
    } else {
        (locals.var_dvh__blk300, locals.var_dvh__blk300_dn1, locals.var_dvh__blk300_dn3, locals.var_dvh__blk300_dn4, locals.var_dvh__blk300_dn5,)
    }
};
        locals.var_dvh__blk300 = assign4520_e4636;
        locals.var_dvh__blk300_dn1 = assign4520_e4636_d_n1;
        locals.var_dvh__blk300_dn3 = assign4520_e4636_d_n3;
        locals.var_dvh__blk300_dn4 = assign4520_e4636_d_n4;
        locals.var_dvh__blk300_dn5 = assign4520_e4636_d_n5;
        locals.var_dvh__blk300_rv = 0.0;

        let assign4530_e4639: f64 = if locals.var_dvh__blk300 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign4530_e4639;
        locals.var_guard310_rv = 0.0;

        let (assign4540_e4654,) = {
    if ((((locals.var_guard280 != 0.0) && (locals.var_guard298 != 0.0)) && (locals.var_guard309 != 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign4540_e4649: f64 = (1.0 - p.p68);
        let assign4540_e4651: f64 = (-p.p81);
        let assign4540_e4652: f64 = (assign4540_e4649).powf(assign4540_e4651);
        (assign4540_e4652,)
    } else {
        (locals.var_pwq__blk301,)
    }
};
        locals.var_pwq__blk301 = assign4540_e4654;
        locals.var_pwq__blk301_rv = 0.0;

    }
}
