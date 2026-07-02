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
        let assign00_e799: f64 = (273.15 + p.p15);
        locals.var_tkr = assign00_e799;

        let assign10_e800: f64 = ctx_temp;
        let assign10_e802: f64 = (assign10_e800 + p.p36);
        let assign10_e804: f64 = (assign10_e802).min(1000.0);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign10_e804, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign20_e807: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign20_e807;

        if (locals.var_guard1 != 0.0) {
            let assign30_e814: f64 = (p.p18 * locals.var_temp);
            let assign30_e815: f64 = (p.p17 + assign30_e814);
            let assign30_e816: f64 = (locals.var_temp + assign30_e815);
            let assign30_e821: f64 = (p.p18 * locals.var_temp);
            let assign30_e822: f64 = (p.p17 + assign30_e821);
            let assign30_e823: f64 = (locals.var_temp - assign30_e822);
            let assign30_e828: f64 = (p.p18 * locals.var_temp);
            let assign30_e829: f64 = (p.p17 + assign30_e828);
            let assign30_e830: f64 = (locals.var_temp - assign30_e829);
            let assign30_e831: f64 = (assign30_e823 * assign30_e830);
            let assign30_e833: f64 = (assign30_e831 + p.p19);
            let assign30_e834: f64 = (assign30_e833).sqrt();
            let assign30_e835: f64 = (assign30_e816 + assign30_e834);
            let assign30_e836: f64 = (0.5 * assign30_e835);
            (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9, ) = (assign30_e836, (0.5 * ((locals.var_temp_dn4 + (p.p18 * locals.var_temp_dn4)) + ((((locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)))) / (2.0 * assign30_e834)))), (0.5 * ((locals.var_temp_dn6 + (p.p18 * locals.var_temp_dn6)) + ((((locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)))) / (2.0 * assign30_e834)))), (0.5 * ((locals.var_temp_dn7 + (p.p18 * locals.var_temp_dn7)) + ((((locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)))) / (2.0 * assign30_e834)))), (0.5 * ((locals.var_temp_dn8 + (p.p18 * locals.var_temp_dn8)) + ((((locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)))) / (2.0 * assign30_e834)))), (0.5 * ((locals.var_temp_dn9 + (p.p18 * locals.var_temp_dn9)) + ((((locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)) * assign30_e830) + (assign30_e823 * (locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)))) / (2.0 * assign30_e834)))), );
        }

        if (locals.var_guard1 != 0.0) {
            let assign40_e844: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign40_e845: f64 = (10.0 / assign40_e844);
            let assign40_e847: f64 = (assign40_e845 + 600.0);
            let assign40_e851: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign40_e852: f64 = (10.0 / assign40_e851);
            let assign40_e854: f64 = (assign40_e852 - 600.0);
            let assign40_e858: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign40_e859: f64 = (10.0 / assign40_e858);
            let assign40_e861: f64 = (assign40_e859 - 600.0);
            let assign40_e862: f64 = (assign40_e854 * assign40_e861);
            let assign40_e864: f64 = (assign40_e862 + 0.01);
            let assign40_e865: f64 = (assign40_e864).sqrt();
            let assign40_e866: f64 = (assign40_e847 + assign40_e865);
            let assign40_e867: f64 = (0.5 * assign40_e866);
            (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9, ) = (assign40_e867, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), );
        }

        if (locals.var_guard1 == 0.0) {
            let assign50_e875: f64 = (locals.var_temp + 1.0);
            let assign50_e878: f64 = (locals.var_temp - 1.0);
            let assign50_e881: f64 = (locals.var_temp - 1.0);
            let assign50_e882: f64 = (assign50_e878 * assign50_e881);
            let assign50_e884: f64 = (assign50_e882 + 0.001);
            let assign50_e885: f64 = (assign50_e884).sqrt();
            let assign50_e886: f64 = (assign50_e875 + assign50_e885);
            let assign50_e887: f64 = (0.5 * assign50_e886);
            (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9, ) = (assign50_e887, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign50_e881) + (assign50_e878 * locals.var_temp_dn4)) / (2.0 * assign50_e885)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign50_e881) + (assign50_e878 * locals.var_temp_dn6)) / (2.0 * assign50_e885)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign50_e881) + (assign50_e878 * locals.var_temp_dn7)) / (2.0 * assign50_e885)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign50_e881) + (assign50_e878 * locals.var_temp_dn8)) / (2.0 * assign50_e885)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign50_e881) + (assign50_e878 * locals.var_temp_dn9)) / (2.0 * assign50_e885)))), );
        }

        if (locals.var_guard1 == 0.0) {
            (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9, ) = (600.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign70_e909: f64 = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p443 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign70_e909;

        if (locals.var_guard2 != 0.0) {
            locals.var_swshe_i = p.p5;
        }

        if (locals.var_guard2 == 0.0) {
            locals.var_swshe_i = 0.0;
        }

        (locals.var_dtc, locals.var_dtc_dn4, ) = (0.0, 0.0, );

        (locals.var_tkc, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9, ) = (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9, );

        let assign140_e928: f64 = (locals.var_tkc * locals.var_tkc);
        (locals.var_tkc_sq, locals.var_tkc_sq_dn4, locals.var_tkc_sq_dn6, locals.var_tkc_sq_dn7, locals.var_tkc_sq_dn8, locals.var_tkc_sq_dn9, ) = (assign140_e928, ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4)), ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6)), ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7)), ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8)), ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9)), );

        let assign150_e931: f64 = (locals.var_tkc - locals.var_tkr);
        (locals.var_dt, locals.var_dt_dn4, locals.var_dt_dn6, locals.var_dt_dn7, locals.var_dt_dn8, locals.var_dt_dn9, ) = (assign150_e931, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9, );

        let assign160_e934: f64 = (locals.var_tkc / locals.var_tkr);
        (locals.var_rt, locals.var_rt_dn4, locals.var_rt_dn6, locals.var_rt_dn7, locals.var_rt_dn8, locals.var_rt_dn9, ) = (assign160_e934, (locals.var_tkc_dn4 / locals.var_tkr), (locals.var_tkc_dn6 / locals.var_tkr), (locals.var_tkc_dn7 / locals.var_tkr), (locals.var_tkc_dn8 / locals.var_tkr), (locals.var_tkc_dn9 / locals.var_tkr), );

        let assign170_e937: f64 = (locals.var_tkr / locals.var_tkc);
        (locals.var_rtn, locals.var_rtn_dn4, locals.var_rtn_dn6, locals.var_rtn_dn7, locals.var_rtn_dn8, locals.var_rtn_dn9, ) = (assign170_e937, (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc))), );

        let assign180_e940: f64 = (locals.var_tkc * 8.617332384961e-5);
        (locals.var_phit0, locals.var_phit0_dn4, locals.var_phit0_dn6, locals.var_phit0_dn7, locals.var_phit0_dn8, locals.var_phit0_dn9, ) = (assign180_e940, (locals.var_tkc_dn4 * 8.617332384961e-5), (locals.var_tkc_dn6 * 8.617332384961e-5), (locals.var_tkc_dn7 * 8.617332384961e-5), (locals.var_tkc_dn8 * 8.617332384961e-5), (locals.var_tkc_dn9 * 8.617332384961e-5), );

        let assign190_e943: f64 = (1.0 / locals.var_phit0);
        (locals.var_inv_phit0, locals.var_inv_phit0_dn4, locals.var_inv_phit0_dn6, locals.var_inv_phit0_dn7, locals.var_inv_phit0_dn8, locals.var_inv_phit0_dn9, ) = (assign190_e943, (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0))), );

        let assign200_e946: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign200_e946;

        if (locals.var_guard83 != 0.0) {
            locals.var_adrain_i = p.p23;
            locals.var_asource_i = p.p22;
            locals.var_pdrain_i = p.p25;
            locals.var_psource_i = p.p24;
            locals.var_mult_i_int = p.p30;
            locals.var_tox1_i = p.p41;
            locals.var_tsi_i = p.p42;
            locals.var_xge_i = p.p43;
            locals.var_tox2_i = p.p44;
            locals.var_typech_i = 1.0;
        }

        let assign310_e989: f64 = if p.p45 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign310_e989;

        if ((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) {
            let assign320_e994: f64 = (-1.0);
            locals.var_typech_i = assign320_e994;
        }

        if (locals.var_guard83 != 0.0) {
            let assign330_e999: f64 = (p.p45).abs();
            let assign330_e1001: f64 = (assign330_e999).min(1e19);
            let assign330_e1003: f64 = (assign330_e1001 * 1000000.0);
            locals.var_nch_i = assign330_e1003;
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_typesub_i = 1.0;
        }

        let assign350_e1012: f64 = if p.p46 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign350_e1012;

        if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
            let assign360_e1017: f64 = (-1.0);
            locals.var_typesub_i = assign360_e1017;
        }

        if (locals.var_guard83 != 0.0) {
            let assign370_e1022: f64 = (p.p46).abs();
            let assign370_e1024: f64 = (assign370_e1022).max(1e16);
            let assign370_e1026: f64 = (assign370_e1024).min(1e21);
            let assign370_e1028: f64 = (assign370_e1026 * 1000000.0);
            locals.var_nsub_i = assign370_e1028;
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_ct_i = p.p47;
            locals.var_toxp_i = p.p48;
        }

        if (locals.var_guard83 != 0.0) {
            let assign400_e1042: f64 = (p.p49 * 1000000.0);
            locals.var_nov_i = assign400_e1042;
        }

        if (locals.var_guard83 != 0.0) {
            let assign410_e1048: f64 = (p.p50 * 1000000.0);
            locals.var_novd_i = assign410_e1048;
        }

        if (locals.var_guard83 != 0.0) {
            (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, ) = (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_stvfb_i = p.p53;
        }

        if (locals.var_guard83 != 0.0) {
            let assign450_e1066: f64 = (p.p54 * 1000000.0);
            (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9, ) = (assign450_e1066, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_cic1_i = p.p55;
            locals.var_cic2_i = p.p56;
            locals.var_psce1_i = p.p57;
        }

        if (locals.var_guard83 != 0.0) {
            let assign490_e1084: f64 = (p.p58 * locals.var_psce1_i);
            let assign490_e1086: f64 = (assign490_e1084 * locals.var_tox2_i);
            let assign490_e1088: f64 = (assign490_e1086 / locals.var_tox1_i);
            locals.var_psce2_i = assign490_e1088;
        }

        if (locals.var_guard83 != 0.0) {
            let assign500_e1094: f64 = (p.p59 * 1000000.0);
            locals.var_nsddc_i = assign500_e1094;
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_pscedlb_i = p.p60;
            locals.var_pnce_i = p.p61;
            (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, ) = (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign540_e1112: f64 = (p.p63 * locals.var_cf1_t);
            let assign540_e1114: f64 = (assign540_e1112 * locals.var_tox2_i);
            let assign540_e1116: f64 = (assign540_e1114 / locals.var_tox1_i);
            (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, ) = (assign540_e1116, (((p.p63 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 != 0.0) {
            (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9, ) = (p.p64, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cfd_i = p.p65;
            locals.var_cfdl_i = p.p66;
            locals.var_cfdlb_i = p.p67;
            (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9, ) = (p.p68, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign600_e1142: f64 = (p.p69 * locals.var_betn1_t);
            (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9, ) = (assign600_e1142, (p.p69 * locals.var_betn1_t_dn4), (p.p69 * locals.var_betn1_t_dn6), (p.p69 * locals.var_betn1_t_dn7), (p.p69 * locals.var_betn1_t_dn8), (p.p69 * locals.var_betn1_t_dn9), );
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_stbet_i = p.p70;
            locals.var_cs_t = p.p71;
            locals.var_csfi_i = p.p72;
            locals.var_csbi_i = p.p73;
            locals.var_stcs_i = p.p74;
            locals.var_thecs_t = p.p75;
            locals.var_stthecs_i = p.p76;
            locals.var_csthr_i = p.p77;
            locals.var_csthrb_i = p.p78;
            locals.var_mue_t = p.p79;
            locals.var_stmue_i = p.p80;
            locals.var_themu_t = p.p81;
            locals.var_stthemu_i = p.p82;
            locals.var_xcor_t = p.p83;
            locals.var_xcorb_i = p.p84;
            locals.var_stxcor_i = p.p85;
            locals.var_feta_i = p.p86;
            locals.var_rs_t = p.p87;
            locals.var_rsig_i = p.p88;
            locals.var_strs_i = p.p89;
            locals.var_rsg_i = p.p90;
            locals.var_thersg_i = p.p91;
            locals.var_rsb_i = p.p92;
            (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, ) = (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_stthesat_i = p.p94;
            locals.var_thesat1_i = p.p95;
            locals.var_thesat2_i = p.p96;
            locals.var_ax_i = p.p97;
            locals.var_alp_i = p.p98;
            locals.var_alp1_i = p.p99;
            locals.var_alpb_i = p.p100;
            locals.var_vp_i = p.p101;
            locals.var_vpg_i = p.p102;
            locals.var_gco_i = p.p103;
            locals.var_iginv_t = p.p104;
            locals.var_igovinv_t = p.p105;
            locals.var_igovinvd_t = p.p106;
            locals.var_fnovinv_t = p.p120;
            locals.var_fnovinvd_t = p.p121;
            locals.var_igovacc_t = p.p107;
            locals.var_igovaccd_t = p.p108;
            locals.var_stig_i = p.p109;
            locals.var_stigfn_i = p.p123;
            locals.var_gc2ch_i = p.p110;
            locals.var_gc3ch_i = p.p111;
            locals.var_gc2ovinv_i = p.p112;
            locals.var_gcovinvfn_i = p.p122;
            locals.var_gc3ovinv_i = p.p113;
            locals.var_gc2ovacc_i = p.p114;
            locals.var_gc3ovacc_i = p.p115;
            locals.var_gcdov_i = p.p116;
            locals.var_gcvdov_i = p.p117;
            locals.var_chib_i = p.p118;
            locals.var_niginv_i = p.p119;
            (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9, ) = (p.p124, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9, ) = (p.p125, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_bgidl_t = p.p126;
            locals.var_bgidld_t = p.p127;
        }

    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard83 != 0.0) {
            locals.var_stbgidl_i = p.p128;
            locals.var_stbgidld_i = p.p129;
            locals.var_cgidl_i = p.p130;
            locals.var_cgidld_i = p.p131;
            locals.var_dgidl_i = p.p132;
            locals.var_dgidld_i = p.p133;
            locals.var_a1_i = p.p147;
            locals.var_a2_t = p.p148;
            locals.var_sta2_i = p.p149;
            locals.var_a3_i = p.p150;
            locals.var_ctedge_i = p.p134;
            (locals.var_vfb1edge_t, locals.var_vfb1edge_t_dn4, locals.var_vfb1edge_t_dn6, locals.var_vfb1edge_t_dn7, locals.var_vfb1edge_t_dn8, locals.var_vfb1edge_t_dn9, ) = (p.p135, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_vfb2edge_t = p.p136;
            locals.var_stvfbedge_i = p.p137;
            locals.var_cic1edge_i = p.p138;
            locals.var_cic2edge_i = p.p139;
            (locals.var_psce1edge_i, locals.var_psce1edge_i_dn4, locals.var_psce1edge_i_dn6, locals.var_psce1edge_i_dn7, locals.var_psce1edge_i_dn8, locals.var_psce1edge_i_dn9, ) = (p.p140, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign1360_e1448: f64 = (p.p141 * locals.var_psce1edge_i);
            let assign1360_e1450: f64 = (assign1360_e1448 * locals.var_tox2_i);
            let assign1360_e1452: f64 = (assign1360_e1450 / locals.var_tox1_i);
            (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9, ) = (assign1360_e1452, (((p.p141 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 != 0.0) {
            (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9, ) = (p.p142, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign1380_e1462: f64 = (p.p143 * locals.var_cf1edge_i);
            let assign1380_e1464: f64 = (assign1380_e1462 * locals.var_tox2_i);
            let assign1380_e1466: f64 = (assign1380_e1464 / locals.var_tox1_i);
            (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9, ) = (assign1380_e1466, (((p.p143 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_cfdedge_i = p.p144;
            (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9, ) = (p.p145, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_stbetedge_i = p.p146;
            locals.var_areaq_i = p.p151;
            (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9, ) = (p.p152, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign1440_e1492: f64 = (p.p153 * 1000000.0);
            locals.var_nsdac_i = assign1440_e1492;
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_fif_i = p.p154;
            locals.var_fsceac_i = p.p155;
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, );
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, );
            locals.var_psceac1_i = locals.var_psce1_i;
            locals.var_psceac2_i = locals.var_psce2_i;
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, );
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, );
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, );
            locals.var_axac_i = locals.var_ax_i;
            locals.var_alpac_i = locals.var_alp_i;
        }

        let assign1560_e1541: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign1560_e1541;

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign1580_e1549: f64 = if param_given[156] { 1.0 } else { 0.0 };
        let assign1580_e1551: f64 = if assign1580_e1549 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign1580_e1551;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) {
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (p.p156, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign1610_e1567: f64 = if param_given[157] { 1.0 } else { 0.0 };
        let assign1610_e1569: f64 = if assign1610_e1567 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign1610_e1569;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard88 != 0.0)) {
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (p.p157, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            locals.var_psceac1_i = p.p57;
        }

        let assign1640_e1585: f64 = if param_given[158] { 1.0 } else { 0.0 };
        let assign1640_e1587: f64 = if assign1640_e1585 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign1640_e1587;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard89 != 0.0)) {
            locals.var_psceac1_i = p.p158;
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            let assign1660_e1601: f64 = (p.p58 * locals.var_psceac1_i);
            let assign1660_e1603: f64 = (assign1660_e1601 * locals.var_tox2_i);
            let assign1660_e1605: f64 = (assign1660_e1603 / locals.var_tox1_i);
            locals.var_psceac2_i = assign1660_e1605;
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign1680_e1615: f64 = if param_given[159] { 1.0 } else { 0.0 };
        let assign1680_e1617: f64 = if assign1680_e1615 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign1680_e1617;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard90 != 0.0)) {
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (p.p159, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            let assign1700_e1631: f64 = (p.p63 * locals.var_cfac1_t);
            let assign1700_e1633: f64 = (assign1700_e1631 * locals.var_tox2_i);
            let assign1700_e1635: f64 = (assign1700_e1633 / locals.var_tox1_i);
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (assign1700_e1635, (((p.p63 * locals.var_cfac1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign1720_e1645: f64 = if param_given[160] { 1.0 } else { 0.0 };
        let assign1720_e1647: f64 = if assign1720_e1645 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign1720_e1647;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard91 != 0.0)) {
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (p.p160, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            locals.var_axac_i = p.p97;
        }

        let assign1750_e1663: f64 = if param_given[161] { 1.0 } else { 0.0 };
        let assign1750_e1665: f64 = if assign1750_e1663 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign1750_e1665;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard92 != 0.0)) {
            locals.var_axac_i = p.p161;
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            locals.var_alpac_i = p.p98;
        }

        let assign1780_e1681: f64 = if param_given[162] { 1.0 } else { 0.0 };
        let assign1780_e1683: f64 = if assign1780_e1681 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign1780_e1683;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard93 != 0.0)) {
            locals.var_alpac_i = p.p162;
        }

        if (locals.var_guard83 != 0.0) {
            (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9, ) = (p.p163, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9, ) = (p.p164, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_covdl_i = p.p165;
            locals.var_covdlb_i = p.p166;
            locals.var_dvfbov_i = p.p167;
            (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9, ) = (p.p168, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9, ) = (p.p169, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_csd_i = p.p170;
            locals.var_csdbp_i = p.p171;
            (locals.var_rth_t, locals.var_rth_t_dn4, locals.var_rth_t_dn6, locals.var_rth_t_dn7, locals.var_rth_t_dn8, locals.var_rth_t_dn9, ) = (p.p172, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_strth_i = p.p173;
            locals.var_fnt_i = p.p175;
            locals.var_fntexc_i = p.p176;
            locals.var_nfa_i = p.p177;
            locals.var_nfb_i = p.p178;
            locals.var_nfc_i = p.p179;
            locals.var_nfe_i = p.p180;
            locals.var_nfeb_i = p.p181;
            (locals.var_kdrift_i, locals.var_kdrift_i_dn4, locals.var_kdrift_i_dn6, locals.var_kdrift_i_dn7, locals.var_kdrift_i_dn8, locals.var_kdrift_i_dn9, ) = (p.p183, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_kdiff_i, locals.var_kdiff_i_dn4, locals.var_kdiff_i_dn6, locals.var_kdiff_i_dn7, locals.var_kdiff_i_dn8, locals.var_kdiff_i_dn9, ) = (p.p184, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_fracinv_i = p.p185;
            locals.var_kfracinv_i = p.p186;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2080_e1808: f64 = (1.0 / p.p29);
            locals.var_invnf = assign2080_e1808;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2090_e1815: f64 = (p.p21 * locals.var_invnf);
            let assign2090_e1817: f64 = (assign2090_e1815).max(1e-9);
            locals.var_w_i = assign2090_e1817;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2100_e1824: f64 = (p.p23 * locals.var_invnf);
            locals.var_adrain_i = assign2100_e1824;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2110_e1831: f64 = (p.p22 * locals.var_invnf);
            locals.var_asource_i = assign2110_e1831;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2120_e1838: f64 = (p.p25 * locals.var_invnf);
            locals.var_pdrain_i = assign2120_e1838;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2130_e1845: f64 = (p.p24 * locals.var_invnf);
            locals.var_psource_i = assign2130_e1845;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2140_e1852: f64 = (p.p30 * p.p29);
            locals.var_mult_i_int = assign2140_e1852;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_len = 1e-6;
            locals.var_wen = 1e-6;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2170_e1869: f64 = (locals.var_len / p.p20);
            locals.var_il = assign2170_e1869;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2180_e1876: f64 = (locals.var_wen / locals.var_w_i);
            locals.var_iw = assign2180_e1876;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2190_e1885: f64 = (p.p192 * locals.var_il);
            let assign2190_e1886: f64 = (1.0 + assign2190_e1885);
            let assign2190_e1887: f64 = (p.p191 * assign2190_e1886);
            let assign2190_e1891: f64 = (p.p193 * locals.var_iw);
            let assign2190_e1892: f64 = (1.0 + assign2190_e1891);
            let assign2190_e1893: f64 = (assign2190_e1887 * assign2190_e1892);
            locals.var_dellps = assign2190_e1893;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2200_e1902: f64 = (p.p197 * locals.var_iw);
            let assign2200_e1903: f64 = (1.0 + assign2200_e1902);
            let assign2200_e1904: f64 = (p.p195 * assign2200_e1903);
            let assign2200_e1908: f64 = (p.p196 * locals.var_il);
            let assign2200_e1909: f64 = (1.0 + assign2200_e1908);
            let assign2200_e1910: f64 = (assign2200_e1904 * assign2200_e1909);
            locals.var_delwod = assign2200_e1910;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2210_e1917: f64 = (p.p20 + locals.var_dellps);
            let assign2210_e1920: f64 = (2.0 * p.p194);
            let assign2210_e1921: f64 = (assign2210_e1917 - assign2210_e1920);
            let assign2210_e1923: f64 = (assign2210_e1921).max(1e-9);
            locals.var_le = assign2210_e1923;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2220_e1930: f64 = (locals.var_w_i + locals.var_delwod);
            let assign2220_e1933: f64 = (2.0 * p.p198);
            let assign2220_e1934: f64 = (assign2220_e1930 - assign2220_e1933);
            let assign2220_e1936: f64 = (assign2220_e1934).max(1e-9);
            locals.var_we = assign2220_e1936;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2230_e1943: f64 = (p.p20 + locals.var_dellps);
            let assign2230_e1946: f64 = (2.0 * p.p194);
            let assign2230_e1947: f64 = (assign2230_e1943 - assign2230_e1946);
            let assign2230_e1949: f64 = (assign2230_e1947 + p.p199);
            let assign2230_e1951: f64 = (assign2230_e1949).max(1e-9);
            locals.var_lecv = assign2230_e1951;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2240_e1958: f64 = (locals.var_w_i + locals.var_delwod);
            let assign2240_e1961: f64 = (2.0 * p.p198);
            let assign2240_e1962: f64 = (assign2240_e1958 - assign2240_e1961);
            let assign2240_e1964: f64 = (assign2240_e1962 + p.p200);
            let assign2240_e1966: f64 = (assign2240_e1964).max(1e-9);
            locals.var_wecv = assign2240_e1966;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2250_e1973: f64 = (locals.var_len / locals.var_le);
            locals.var_ile = assign2250_e1973;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2260_e1980: f64 = (locals.var_wen / locals.var_we);
            locals.var_iwe = assign2260_e1980;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2270_e1987: f64 = (locals.var_ile * locals.var_iwe);
            locals.var_iae = assign2270_e1987;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2280_e1994: f64 = (p.p20 + locals.var_dellps);
            let assign2280_e1996: f64 = (assign2280_e1994).max(1e-9);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign2280_e1996, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2290_e2003: f64 = (locals.var_temp / locals.var_len);
            (locals.var_lphy, locals.var_lphy_dn4, locals.var_lphy_dn6, locals.var_lphy_dn7, locals.var_lphy_dn8, locals.var_lphy_dn9, ) = (assign2290_e2003, (locals.var_temp_dn4 / locals.var_len), (locals.var_temp_dn6 / locals.var_len), (locals.var_temp_dn7 / locals.var_len), (locals.var_temp_dn8 / locals.var_len), (locals.var_temp_dn9 / locals.var_len), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2300_e2010: f64 = (locals.var_w_i + locals.var_delwod);
            let assign2300_e2012: f64 = (assign2300_e2010).max(1e-9);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign2300_e2012, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2310_e2019: f64 = (locals.var_temp / locals.var_wen);
            (locals.var_wphy, locals.var_wphy_dn4, locals.var_wphy_dn6, locals.var_wphy_dn7, locals.var_wphy_dn8, locals.var_wphy_dn9, ) = (assign2310_e2019, (locals.var_temp_dn4 / locals.var_wen), (locals.var_temp_dn6 / locals.var_wen), (locals.var_temp_dn7 / locals.var_wen), (locals.var_temp_dn8 / locals.var_wen), (locals.var_temp_dn9 / locals.var_wen), );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_tox1_i = p.p201;
            locals.var_tsi_i = p.p202;
            locals.var_xge_i = p.p203;
            locals.var_tox2_i = p.p204;
            locals.var_typech_i = 1.0;
        }

        let assign2410_e2087: f64 = if p.p205 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign2410_e2087;

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard83 == 0.0) && (locals.var_guard94 != 0.0)) {
            let assign2420_e2093: f64 = (-1.0);
            locals.var_typech_i = assign2420_e2093;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2430_e2099: f64 = (p.p205).abs();
            let assign2430_e2101: f64 = (assign2430_e2099).min(1e19);
            let assign2430_e2103: f64 = (assign2430_e2101 * 1000000.0);
            locals.var_nch_i = assign2430_e2103;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_typesub_i = 1.0;
        }

        let assign2450_e2113: f64 = if p.p206 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign2450_e2113;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard95 != 0.0)) {
            let assign2460_e2119: f64 = (-1.0);
            locals.var_typesub_i = assign2460_e2119;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2470_e2125: f64 = (p.p206).abs();
            let assign2470_e2127: f64 = (assign2470_e2125).max(1e16);
            let assign2470_e2129: f64 = (assign2470_e2127).min(1e21);
            let assign2470_e2131: f64 = (assign2470_e2129 * 1000000.0);
            locals.var_nsub_i = assign2470_e2131;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_ct_i = p.p207;
            locals.var_toxp_i = p.p208;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2500_e2148: f64 = (p.p209 * 1000000.0);
            locals.var_nov_i = assign2500_e2148;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2510_e2155: f64 = (p.p210 * 1000000.0);
            locals.var_novd_i = assign2510_e2155;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2520_e2163: f64 = (locals.var_ile).powf(p.p213);
            let assign2520_e2164: f64 = (p.p212 * assign2520_e2163);
            let assign2520_e2169: f64 = (locals.var_ile).powf(p.p215);
            let assign2520_e2170: f64 = (p.p214 * assign2520_e2169);
            let assign2520_e2171: f64 = (1.0 + assign2520_e2170);
            let assign2520_e2172: f64 = (assign2520_e2164 / assign2520_e2171);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign2520_e2172, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2530_e2179: f64 = (p.p211 + locals.var_temp);
            let assign2530_e2182: f64 = (p.p216 * locals.var_iwe);
            let assign2530_e2183: f64 = (assign2530_e2179 + assign2530_e2182);
            let assign2530_e2186: f64 = (p.p217 * locals.var_iae);
            let assign2530_e2187: f64 = (assign2530_e2183 + assign2530_e2186);
            (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, ) = (assign2530_e2187, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2540_e2195: f64 = (p.p219 * locals.var_tox2_i);
            let assign2540_e2197: f64 = (assign2540_e2195 / locals.var_tox1_i);
            let assign2540_e2199: f64 = (assign2540_e2197 * locals.var_temp);
            let assign2540_e2200: f64 = (p.p218 + assign2540_e2199);
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (assign2540_e2200, (assign2540_e2197 * locals.var_temp_dn4), (assign2540_e2197 * locals.var_temp_dn6), (assign2540_e2197 * locals.var_temp_dn7), (assign2540_e2197 * locals.var_temp_dn8), (assign2540_e2197 * locals.var_temp_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2550_e2209: f64 = (p.p221 * locals.var_ile);
            let assign2550_e2210: f64 = (1.0 + assign2550_e2209);
            let assign2550_e2211: f64 = (p.p220 * assign2550_e2210);
            let assign2550_e2215: f64 = (p.p222 * locals.var_iwe);
            let assign2550_e2216: f64 = (1.0 + assign2550_e2215);
            let assign2550_e2217: f64 = (assign2550_e2211 * assign2550_e2216);
            let assign2550_e2221: f64 = (p.p223 * locals.var_iae);
            let assign2550_e2222: f64 = (1.0 + assign2550_e2221);
            let assign2550_e2223: f64 = (assign2550_e2217 * assign2550_e2222);
            locals.var_stvfb_i = assign2550_e2223;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2560_e2232: f64 = (p.p225 * locals.var_ile);
            let assign2560_e2233: f64 = (1.0 + assign2560_e2232);
            let assign2560_e2234: f64 = (p.p224 * assign2560_e2233);
            let assign2560_e2236: f64 = (assign2560_e2234 * 1000000.0);
            (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9, ) = (assign2560_e2236, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2570_e2243: f64 = (locals.var_temp0__blk79).max(1e25);
            let assign2570_e2245: f64 = (assign2570_e2243).min(1e28);
            (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9, ) = (assign2570_e2245, if assign2570_e2243 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn4 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn6 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn7 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn8 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn9 } else { 0.0 } } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cic1_i = p.p226;
            locals.var_cic2_i = p.p227;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2600_e2262: f64 = (1.0 - locals.var_xge_i);
            locals.var_one_m_xge = assign2600_e2262;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2610_e2269: f64 = (1.04479e-10 * locals.var_one_m_xge);
            let assign2610_e2272: f64 = (1.43438e-10 * locals.var_xge_i);
            let assign2610_e2273: f64 = (assign2610_e2269 + assign2610_e2272);
            locals.var_epsch = assign2610_e2273;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2620_e2280: f64 = (locals.var_epsch / 3.45313e-11);
            let assign2620_e2282: f64 = (assign2620_e2280 * locals.var_tsi_i);
            let assign2620_e2285: f64 = (locals.var_tox1_i + 4e-10);
            let assign2620_e2286: f64 = (assign2620_e2282 * assign2620_e2285);
            let assign2620_e2287: f64 = (assign2620_e2286).sqrt();
            let assign2620_e2289: f64 = (assign2620_e2287 / locals.var_le);
            locals.var_lambda_le = assign2620_e2289;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2630_e2296: f64 = (p.p228 * 2.0);
            let assign2630_e2299: f64 = (locals.var_lambda_le).powf(p.p229);
            let assign2630_e2300: f64 = (assign2630_e2296 * assign2630_e2299);
            let assign2630_e2304: f64 = (p.p230 * locals.var_iwe);
            let assign2630_e2305: f64 = (1.0 + assign2630_e2304);
            let assign2630_e2306: f64 = (assign2630_e2300 * assign2630_e2305);
            locals.var_psce_p = assign2630_e2306;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2640_e2313: f64 = (locals.var_psce_p).max(0.0);
            let assign2640_e2315: f64 = (assign2640_e2313).min(5.0);
            locals.var_psce1_i = assign2640_e2315;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2650_e2322: f64 = (p.p231 * locals.var_psce1_i);
            let assign2650_e2324: f64 = (assign2650_e2322 * locals.var_tox2_i);
            let assign2650_e2326: f64 = (assign2650_e2324 / locals.var_tox1_i);
            locals.var_psce2_i = assign2650_e2326;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2660_e2333: f64 = (p.p232 * 1000000.0);
            locals.var_nsddc_i = assign2660_e2333;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_pscedlb_i = p.p233;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2680_e2345: f64 = (p.p234 * locals.var_iwe);
            locals.var_pnce_p = assign2680_e2345;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2690_e2352: f64 = (-1.0);
            let assign2690_e2353: f64 = (locals.var_pnce_p).max(assign2690_e2352);
            let assign2690_e2355: f64 = (assign2690_e2353).min(1.0);
            locals.var_pnce_i = assign2690_e2355;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2700_e2362: f64 = (locals.var_lambda_le).powf(p.p236);
            let assign2700_e2366: f64 = (p.p237 * locals.var_iwe);
            let assign2700_e2367: f64 = (1.0 + assign2700_e2366);
            let assign2700_e2368: f64 = (assign2700_e2362 * assign2700_e2367);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign2700_e2368, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2710_e2375: f64 = (p.p235 * locals.var_temp);
            (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9, ) = (assign2710_e2375, (p.p235 * locals.var_temp_dn4), (p.p235 * locals.var_temp_dn6), (p.p235 * locals.var_temp_dn7), (p.p235 * locals.var_temp_dn8), (p.p235 * locals.var_temp_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2720_e2382: f64 = (locals.var_cf_p).max(0.0);
            (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, ) = (assign2720_e2382, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2730_e2389: f64 = (p.p238 * locals.var_cf1_t);
            let assign2730_e2391: f64 = (assign2730_e2389 * locals.var_tox2_i);
            let assign2730_e2393: f64 = (assign2730_e2391 / locals.var_tox1_i);
            (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, ) = (assign2730_e2393, (((p.p238 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p238 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p238 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p238 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p238 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2740_e2400: f64 = (p.p239 * locals.var_temp);
            (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9, ) = (assign2740_e2400, (p.p239 * locals.var_temp_dn4), (p.p239 * locals.var_temp_dn6), (p.p239 * locals.var_temp_dn7), (p.p239 * locals.var_temp_dn8), (p.p239 * locals.var_temp_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cfd_i = p.p240;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2760_e2412: f64 = (p.p241 * locals.var_ile);
            let assign2760_e2416: f64 = (p.p242 * locals.var_iwe);
            let assign2760_e2417: f64 = (1.0 + assign2760_e2416);
            let assign2760_e2419: f64 = (assign2760_e2417).max(0.001);
            let assign2760_e2420: f64 = (assign2760_e2412 / assign2760_e2419);
            locals.var_cfdl_i = assign2760_e2420;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cfdlb_i = p.p243;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2780_e2431: f64 = (-locals.var_le);
            let assign2780_e2436: f64 = (p.p248 * locals.var_iwe);
            let assign2780_e2437: f64 = (1.0 + assign2780_e2436);
            let assign2780_e2439: f64 = (assign2780_e2437).max(0.001);
            let assign2780_e2440: f64 = (p.p247 * assign2780_e2439);
            let assign2780_e2441: f64 = (assign2780_e2431 / assign2780_e2440);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign2780_e2441, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign2790_e2446: f64 = (-80.0);
        let assign2790_e2447: f64 = if locals.var_temp1 > assign2790_e2446 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign2790_e2447;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard96 != 0.0)) {
            let assign2800_e2453: f64 = (locals.var_temp1).exp();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign2800_e2453, (assign2800_e2453 * locals.var_temp1_dn4), (assign2800_e2453 * locals.var_temp1_dn6), (assign2800_e2453 * locals.var_temp1_dn7), (assign2800_e2453 * locals.var_temp1_dn8), (assign2800_e2453 * locals.var_temp1_dn9), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard96 == 0.0)) {
            let assign2810_e2464: f64 = (-locals.var_temp1);
            let assign2810_e2466: f64 = (assign2810_e2464 - 80.0);
            let assign2810_e2470: f64 = (-locals.var_temp1);
            let assign2810_e2472: f64 = (assign2810_e2470 - 80.0);
            let assign2810_e2473: f64 = (0.5 * assign2810_e2472);
            let assign2810_e2476: f64 = (-locals.var_temp1);
            let assign2810_e2478: f64 = (assign2810_e2476 - 80.0);
            let assign2810_e2480: f64 = (assign2810_e2478 * 0.3333333333333);
            let assign2810_e2481: f64 = (1.0 + assign2810_e2480);
            let assign2810_e2482: f64 = (assign2810_e2473 * assign2810_e2481);
            let assign2810_e2483: f64 = (1.0 + assign2810_e2482);
            let assign2810_e2484: f64 = (assign2810_e2466 * assign2810_e2483);
            let assign2810_e2485: f64 = (1.0 + assign2810_e2484);
            let assign2810_e2486: f64 = (1.80485e-35 / assign2810_e2485);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign2810_e2486, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-locals.var_temp1_dn4)) * assign2810_e2481) + (assign2810_e2473 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-locals.var_temp1_dn6)) * assign2810_e2481) + (assign2810_e2473 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-locals.var_temp1_dn7)) * assign2810_e2481) + (assign2810_e2473 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-locals.var_temp1_dn8)) * assign2810_e2481) + (assign2810_e2473 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-locals.var_temp1_dn9)) * assign2810_e2481) + (assign2810_e2473 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2820_e2492: f64 = (-locals.var_le);
            let assign2820_e2494: f64 = (assign2820_e2492 / p.p250);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign2820_e2494, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign2830_e2499: f64 = (-80.0);
        let assign2830_e2500: f64 = if locals.var_temp3 > assign2830_e2499 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign2830_e2500;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard97 != 0.0)) {
            let assign2840_e2506: f64 = (locals.var_temp3).exp();
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign2840_e2506, (assign2840_e2506 * locals.var_temp3_dn4), (assign2840_e2506 * locals.var_temp3_dn6), (assign2840_e2506 * locals.var_temp3_dn7), (assign2840_e2506 * locals.var_temp3_dn8), (assign2840_e2506 * locals.var_temp3_dn9), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard97 == 0.0)) {
            let assign2850_e2517: f64 = (-locals.var_temp3);
            let assign2850_e2519: f64 = (assign2850_e2517 - 80.0);
            let assign2850_e2523: f64 = (-locals.var_temp3);
            let assign2850_e2525: f64 = (assign2850_e2523 - 80.0);
            let assign2850_e2526: f64 = (0.5 * assign2850_e2525);
            let assign2850_e2529: f64 = (-locals.var_temp3);
            let assign2850_e2531: f64 = (assign2850_e2529 - 80.0);
            let assign2850_e2533: f64 = (assign2850_e2531 * 0.3333333333333);
            let assign2850_e2534: f64 = (1.0 + assign2850_e2533);
            let assign2850_e2535: f64 = (assign2850_e2526 * assign2850_e2534);
            let assign2850_e2536: f64 = (1.0 + assign2850_e2535);
            let assign2850_e2537: f64 = (assign2850_e2519 * assign2850_e2536);
            let assign2850_e2538: f64 = (1.0 + assign2850_e2537);
            let assign2850_e2539: f64 = (1.80485e-35 / assign2850_e2538);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign2850_e2539, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-locals.var_temp3_dn4)) * assign2850_e2534) + (assign2850_e2526 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-locals.var_temp3_dn6)) * assign2850_e2534) + (assign2850_e2526 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-locals.var_temp3_dn7)) * assign2850_e2534) + (assign2850_e2526 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-locals.var_temp3_dn8)) * assign2850_e2534) + (assign2850_e2526 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-locals.var_temp3_dn9)) * assign2850_e2534) + (assign2850_e2526 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2860_e2549: f64 = (p.p246 * locals.var_iwe);
            let assign2860_e2550: f64 = (1.0 + assign2860_e2549);
            let assign2860_e2551: f64 = (p.p245 * assign2860_e2550);
            let assign2860_e2554: f64 = (locals.var_temp2 - 1.0);
            let assign2860_e2555: f64 = (assign2860_e2551 * assign2860_e2554);
            let assign2860_e2557: f64 = (assign2860_e2555 / locals.var_temp1);
            let assign2860_e2558: f64 = (1.0 + assign2860_e2557);
            let assign2860_e2562: f64 = (locals.var_temp4 - 1.0);
            let assign2860_e2563: f64 = (p.p249 * assign2860_e2562);
            let assign2860_e2565: f64 = (assign2860_e2563 / locals.var_temp3);
            let assign2860_e2566: f64 = (assign2860_e2558 + assign2860_e2565);
            let assign2860_e2568: f64 = (assign2860_e2566).max(1e-6);
            (locals.var_gpe, locals.var_gpe_dn4, locals.var_gpe_dn6, locals.var_gpe_dn7, locals.var_gpe_dn8, locals.var_gpe_dn9, ) = (assign2860_e2568, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * locals.var_temp2_dn4) * locals.var_temp1) - (assign2860_e2555 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p249 * locals.var_temp4_dn4) * locals.var_temp3) - (assign2860_e2563 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * locals.var_temp2_dn6) * locals.var_temp1) - (assign2860_e2555 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p249 * locals.var_temp4_dn6) * locals.var_temp3) - (assign2860_e2563 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * locals.var_temp2_dn7) * locals.var_temp1) - (assign2860_e2555 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p249 * locals.var_temp4_dn7) * locals.var_temp3) - (assign2860_e2563 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * locals.var_temp2_dn8) * locals.var_temp1) - (assign2860_e2555 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p249 * locals.var_temp4_dn8) * locals.var_temp3) - (assign2860_e2563 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * locals.var_temp2_dn9) * locals.var_temp1) - (assign2860_e2555 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p249 * locals.var_temp4_dn9) * locals.var_temp3) - (assign2860_e2563 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2870_e2576: f64 = (p.p251 * locals.var_iwe);
            let assign2870_e2577: f64 = (1.0 + assign2870_e2576);
            let assign2870_e2580: f64 = (p.p252 * locals.var_iwe);
            let assign2870_e2584: f64 = (locals.var_we / p.p253);
            let assign2870_e2585: f64 = (1.0 + assign2870_e2584);
            let assign2870_e2586: f64 = (assign2870_e2585).ln();
            let assign2870_e2587: f64 = (assign2870_e2580 * assign2870_e2586);
            let assign2870_e2588: f64 = (assign2870_e2577 + assign2870_e2587);
            let assign2870_e2590: f64 = (assign2870_e2588).max(1e-6);
            locals.var_gwe = assign2870_e2590;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2880_e2597: f64 = (p.p244 / locals.var_gpe);
            let assign2880_e2599: f64 = (assign2880_e2597 * locals.var_gwe);
            (locals.var_ge, locals.var_ge_dn4, locals.var_ge_dn6, locals.var_ge_dn7, locals.var_ge_dn8, locals.var_ge_dn9, ) = (assign2880_e2599, ((-((p.p244 * locals.var_gpe_dn4) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p244 * locals.var_gpe_dn6) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p244 * locals.var_gpe_dn7) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p244 * locals.var_gpe_dn8) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p244 * locals.var_gpe_dn9) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2890_e2606: f64 = (locals.var_ge * locals.var_we);
            let assign2890_e2608: f64 = (assign2890_e2606 / locals.var_le);
            (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9, ) = (assign2890_e2608, ((locals.var_ge_dn4 * locals.var_we) / locals.var_le), ((locals.var_ge_dn6 * locals.var_we) / locals.var_le), ((locals.var_ge_dn7 * locals.var_we) / locals.var_le), ((locals.var_ge_dn8 * locals.var_we) / locals.var_le), ((locals.var_ge_dn9 * locals.var_we) / locals.var_le), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2900_e2615: f64 = (locals.var_betn_p).max(1e-10);
            (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9, ) = (assign2900_e2615, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2910_e2622: f64 = (p.p254 * locals.var_betn1_t);
            (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9, ) = (assign2910_e2622, (p.p254 * locals.var_betn1_t_dn4), (p.p254 * locals.var_betn1_t_dn6), (p.p254 * locals.var_betn1_t_dn7), (p.p254 * locals.var_betn1_t_dn8), (p.p254 * locals.var_betn1_t_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2920_e2631: f64 = (p.p256 * locals.var_ile);
            let assign2920_e2632: f64 = (1.0 + assign2920_e2631);
            let assign2920_e2633: f64 = (p.p255 * assign2920_e2632);
            let assign2920_e2637: f64 = (p.p257 * locals.var_iwe);
            let assign2920_e2638: f64 = (1.0 + assign2920_e2637);
            let assign2920_e2639: f64 = (assign2920_e2633 * assign2920_e2638);
            let assign2920_e2643: f64 = (p.p258 * locals.var_iae);
            let assign2920_e2644: f64 = (1.0 + assign2920_e2643);
            let assign2920_e2645: f64 = (assign2920_e2639 * assign2920_e2644);
            locals.var_stbet_i = assign2920_e2645;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2930_e2654: f64 = (locals.var_ile).powf(p.p261);
            let assign2930_e2655: f64 = (p.p260 * assign2930_e2654);
            let assign2930_e2656: f64 = (p.p259 + assign2930_e2655);
            let assign2930_e2660: f64 = (p.p262 * locals.var_iwe);
            let assign2930_e2661: f64 = (1.0 + assign2930_e2660);
            let assign2930_e2662: f64 = (assign2930_e2656 * assign2930_e2661);
            let assign2930_e2666: f64 = (p.p263 * locals.var_iae);
            let assign2930_e2667: f64 = (1.0 + assign2930_e2666);
            let assign2930_e2668: f64 = (assign2930_e2662 * assign2930_e2667);
            locals.var_cs_p = assign2930_e2668;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2940_e2675: f64 = (locals.var_cs_p).max(0.0);
            locals.var_cs_t = assign2940_e2675;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_csfi_i = p.p264;
            locals.var_csbi_i = p.p265;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2970_e2694: f64 = (p.p267 * locals.var_ile);
            let assign2970_e2695: f64 = (1.0 + assign2970_e2694);
            let assign2970_e2696: f64 = (p.p266 * assign2970_e2695);
            let assign2970_e2700: f64 = (p.p268 * locals.var_iwe);
            let assign2970_e2701: f64 = (1.0 + assign2970_e2700);
            let assign2970_e2702: f64 = (assign2970_e2696 * assign2970_e2701);
            let assign2970_e2706: f64 = (p.p269 * locals.var_iae);
            let assign2970_e2707: f64 = (1.0 + assign2970_e2706);
            let assign2970_e2708: f64 = (assign2970_e2702 * assign2970_e2707);
            locals.var_stcs_i = assign2970_e2708;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_thecs_t = p.p270;
            locals.var_stthecs_i = p.p271;
            locals.var_csthr_i = p.p272;
            locals.var_csthrb_i = p.p273;
            locals.var_mue_t = p.p274;
            locals.var_stmue_i = p.p275;
            locals.var_themu_t = p.p276;
            locals.var_stthemu_i = p.p277;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3060_e2757: f64 = (locals.var_ile).powf(p.p280);
            let assign3060_e2758: f64 = (p.p279 * assign3060_e2757);
            let assign3060_e2759: f64 = (p.p278 + assign3060_e2758);
            let assign3060_e2763: f64 = (p.p281 * locals.var_iwe);
            let assign3060_e2764: f64 = (1.0 + assign3060_e2763);
            let assign3060_e2765: f64 = (assign3060_e2759 * assign3060_e2764);
            let assign3060_e2769: f64 = (p.p282 * locals.var_iae);
            let assign3060_e2770: f64 = (1.0 + assign3060_e2769);
            let assign3060_e2771: f64 = (assign3060_e2765 * assign3060_e2770);
            locals.var_xcor_t = assign3060_e2771;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_xcorb_i = p.p283;
            locals.var_stxcor_i = p.p284;
            locals.var_feta_i = p.p285;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3100_e2793: f64 = (p.p286 * locals.var_iwe);
            let assign3100_e2797: f64 = (p.p287 * locals.var_iwe);
            let assign3100_e2798: f64 = (1.0 + assign3100_e2797);
            let assign3100_e2799: f64 = (assign3100_e2793 * assign3100_e2798);
            locals.var_rs_p = assign3100_e2799;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3110_e2806: f64 = (locals.var_rs_p).max(0.0);
            locals.var_rs_t = assign3110_e2806;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_rsig_i = p.p288;
            locals.var_strs_i = p.p289;
            locals.var_rsg_i = p.p290;
            locals.var_thersg_i = p.p291;
            locals.var_rsb_i = p.p292;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3170_e2841: f64 = (locals.var_ile).powf(p.p295);
            let assign3170_e2842: f64 = (p.p294 * assign3170_e2841);
            let assign3170_e2843: f64 = (p.p293 + assign3170_e2842);
            let assign3170_e2844: f64 = (locals.var_ge * assign3170_e2843);
            let assign3170_e2848: f64 = (p.p296 * locals.var_iwe);
            let assign3170_e2849: f64 = (1.0 + assign3170_e2848);
            let assign3170_e2850: f64 = (assign3170_e2844 * assign3170_e2849);
            let assign3170_e2854: f64 = (p.p297 * locals.var_iae);
            let assign3170_e2855: f64 = (1.0 + assign3170_e2854);
            let assign3170_e2856: f64 = (assign3170_e2850 * assign3170_e2855);
            (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9, ) = (assign3170_e2856, (((locals.var_ge_dn4 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((locals.var_ge_dn6 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((locals.var_ge_dn7 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((locals.var_ge_dn8 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((locals.var_ge_dn9 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3180_e2863: f64 = (locals.var_thesat_p).max(0.0);
            (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, ) = (assign3180_e2863, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 }, );
        }

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard83 == 0.0) {
            let assign3190_e2872: f64 = (p.p299 * locals.var_ile);
            let assign3190_e2873: f64 = (1.0 + assign3190_e2872);
            let assign3190_e2874: f64 = (p.p298 * assign3190_e2873);
            let assign3190_e2878: f64 = (p.p300 * locals.var_iwe);
            let assign3190_e2879: f64 = (1.0 + assign3190_e2878);
            let assign3190_e2880: f64 = (assign3190_e2874 * assign3190_e2879);
            let assign3190_e2884: f64 = (p.p301 * locals.var_iae);
            let assign3190_e2885: f64 = (1.0 + assign3190_e2884);
            let assign3190_e2886: f64 = (assign3190_e2880 * assign3190_e2885);
            locals.var_stthesat_i = assign3190_e2886;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_thesat1_i = p.p302;
            locals.var_thesat2_i = p.p303;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3220_e2906: f64 = (locals.var_ile).powf(p.p306);
            let assign3220_e2907: f64 = (p.p305 * assign3220_e2906);
            let assign3220_e2912: f64 = (locals.var_ile).powf(p.p308);
            let assign3220_e2913: f64 = (p.p307 * assign3220_e2912);
            let assign3220_e2914: f64 = (1.0 + assign3220_e2913);
            let assign3220_e2915: f64 = (assign3220_e2907 / assign3220_e2914);
            let assign3220_e2916: f64 = (1.0 + assign3220_e2915);
            let assign3220_e2917: f64 = (p.p304 / assign3220_e2916);
            locals.var_ax_p = assign3220_e2917;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3230_e2924: f64 = (locals.var_ax_p).max(1.0);
            let assign3230_e2926: f64 = (assign3230_e2924).min(16.0);
            locals.var_ax_i = assign3230_e2926;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3240_e2934: f64 = (locals.var_ile).powf(p.p310);
            let assign3240_e2935: f64 = (p.p309 * assign3240_e2934);
            let assign3240_e2939: f64 = (p.p313 * locals.var_iwe);
            let assign3240_e2940: f64 = (1.0 + assign3240_e2939);
            let assign3240_e2941: f64 = (assign3240_e2935 * assign3240_e2940);
            let assign3240_e2946: f64 = (locals.var_ile).powf(p.p312);
            let assign3240_e2947: f64 = (p.p311 * assign3240_e2946);
            let assign3240_e2948: f64 = (1.0 + assign3240_e2947);
            let assign3240_e2949: f64 = (assign3240_e2941 / assign3240_e2948);
            locals.var_alp_p = assign3240_e2949;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3250_e2956: f64 = (locals.var_alp_p).max(0.0);
            locals.var_alp_i = assign3250_e2956;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3260_e2964: f64 = (locals.var_ile).powf(p.p315);
            let assign3260_e2965: f64 = (p.p314 * assign3260_e2964);
            let assign3260_e2969: f64 = (p.p318 * locals.var_iwe);
            let assign3260_e2970: f64 = (1.0 + assign3260_e2969);
            let assign3260_e2971: f64 = (assign3260_e2965 * assign3260_e2970);
            let assign3260_e2976: f64 = (locals.var_ile).powf(p.p317);
            let assign3260_e2977: f64 = (p.p316 * assign3260_e2976);
            let assign3260_e2978: f64 = (1.0 + assign3260_e2977);
            let assign3260_e2979: f64 = (assign3260_e2971 / assign3260_e2978);
            locals.var_alp1_p = assign3260_e2979;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3270_e2986: f64 = (locals.var_alp1_p).max(0.0);
            locals.var_alp1_i = assign3270_e2986;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_alpb_i = p.p319;
            locals.var_vp_i = p.p320;
            locals.var_vpg_i = p.p321;
            locals.var_gco_i = p.p322;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3320_e3013: f64 = (p.p323 / locals.var_iae);
            locals.var_iginv_t = assign3320_e3013;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3330_e3020: f64 = (p.p324 / locals.var_iwe);
            locals.var_igovinv_t = assign3330_e3020;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3340_e3027: f64 = (p.p325 / locals.var_iwe);
            locals.var_igovinvd_t = assign3340_e3027;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3350_e3034: f64 = (p.p339 / locals.var_iwe);
            locals.var_fnovinv_t = assign3350_e3034;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3360_e3041: f64 = (p.p340 / locals.var_iwe);
            locals.var_fnovinvd_t = assign3360_e3041;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3370_e3048: f64 = (p.p326 / locals.var_iwe);
            locals.var_igovacc_t = assign3370_e3048;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3380_e3055: f64 = (p.p327 / locals.var_iwe);
            locals.var_igovaccd_t = assign3380_e3055;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_stig_i = p.p328;
            locals.var_stigfn_i = p.p342;
            locals.var_gc2ch_i = p.p329;
            locals.var_gc3ch_i = p.p330;
            locals.var_gc2ovinv_i = p.p331;
            locals.var_gcovinvfn_i = p.p341;
            locals.var_gc3ovinv_i = p.p332;
            locals.var_gc2ovacc_i = p.p333;
            locals.var_gc3ovacc_i = p.p334;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3480_e3107: f64 = (p.p335 * locals.var_ile);
            locals.var_gcdov_i = assign3480_e3107;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_gcvdov_i = p.p336;
            locals.var_chib_i = p.p337;
            locals.var_niginv_i = p.p338;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3520_e3130: f64 = (p.p345 / locals.var_iwe);
            let assign3520_e3131: f64 = (p.p343 + assign3520_e3130);
            locals.var_agidl_p = assign3520_e3131;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3530_e3138: f64 = (locals.var_agidl_p).max(0.0);
            (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9, ) = (assign3530_e3138, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3540_e3146: f64 = (p.p346 / locals.var_iwe);
            let assign3540_e3147: f64 = (p.p344 + assign3540_e3146);
            locals.var_agidld_p = assign3540_e3147;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3550_e3154: f64 = (locals.var_agidld_p).max(0.0);
            (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9, ) = (assign3550_e3154, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_bgidl_t = p.p347;
            locals.var_bgidld_t = p.p348;
            locals.var_stbgidl_i = p.p349;
            locals.var_stbgidld_i = p.p350;
            locals.var_cgidl_i = p.p351;
            locals.var_cgidld_i = p.p352;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3620_e3192: f64 = (p.p355 * locals.var_ile);
            let assign3620_e3193: f64 = (p.p353 + assign3620_e3192);
            locals.var_dgidl_i = assign3620_e3193;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3630_e3201: f64 = (p.p356 * locals.var_ile);
            let assign3630_e3202: f64 = (p.p354 + assign3630_e3201);
            locals.var_dgidld_i = assign3630_e3202;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3640_e3211: f64 = (p.p389 * locals.var_ile);
            let assign3640_e3212: f64 = (1.0 + assign3640_e3211);
            let assign3640_e3213: f64 = (p.p388 * assign3640_e3212);
            let assign3640_e3217: f64 = (p.p390 * locals.var_iwe);
            let assign3640_e3218: f64 = (1.0 + assign3640_e3217);
            let assign3640_e3219: f64 = (assign3640_e3213 * assign3640_e3218);
            locals.var_a1_p = assign3640_e3219;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3650_e3226: f64 = (locals.var_a1_p).max(0.0);
            locals.var_a1_i = assign3650_e3226;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_a2_t = p.p391;
            locals.var_sta2_i = p.p392;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3680_e3245: f64 = (p.p394 * locals.var_ile);
            let assign3680_e3246: f64 = (1.0 + assign3680_e3245);
            let assign3680_e3247: f64 = (p.p393 * assign3680_e3246);
            let assign3680_e3251: f64 = (p.p395 * locals.var_iwe);
            let assign3680_e3252: f64 = (1.0 + assign3680_e3251);
            let assign3680_e3253: f64 = (assign3680_e3247 * assign3680_e3252);
            locals.var_a3_p = assign3680_e3253;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3690_e3260: f64 = (locals.var_a3_p).max(0.0);
            locals.var_a3_i = assign3690_e3260;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3700_e3267: f64 = (2.0 * p.p357);
            let assign3700_e3270: f64 = (p.p358 * locals.var_we);
            let assign3700_e3271: f64 = (assign3700_e3267 + assign3700_e3270);
            locals.var_we_edge = assign3700_e3271;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_ctedge_i = p.p359;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3720_e3284: f64 = (locals.var_ile).powf(p.p362);
            let assign3720_e3285: f64 = (p.p361 * assign3720_e3284);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3720_e3285, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3730_e3292: f64 = (p.p360 + locals.var_temp);
            let assign3730_e3295: f64 = (p.p363 * locals.var_iwe);
            let assign3730_e3296: f64 = (assign3730_e3292 + assign3730_e3295);
            let assign3730_e3299: f64 = (p.p364 * locals.var_iae);
            let assign3730_e3300: f64 = (assign3730_e3296 + assign3730_e3299);
            (locals.var_vfb1edge_t, locals.var_vfb1edge_t_dn4, locals.var_vfb1edge_t_dn6, locals.var_vfb1edge_t_dn7, locals.var_vfb1edge_t_dn8, locals.var_vfb1edge_t_dn9, ) = (assign3730_e3300, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_vfb2edge_t = p.p365;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3750_e3314: f64 = (p.p367 * locals.var_ile);
            let assign3750_e3315: f64 = (1.0 + assign3750_e3314);
            let assign3750_e3316: f64 = (p.p366 * assign3750_e3315);
            let assign3750_e3320: f64 = (p.p368 * locals.var_iwe);
            let assign3750_e3321: f64 = (1.0 + assign3750_e3320);
            let assign3750_e3322: f64 = (assign3750_e3316 * assign3750_e3321);
            let assign3750_e3326: f64 = (p.p369 * locals.var_iae);
            let assign3750_e3327: f64 = (1.0 + assign3750_e3326);
            let assign3750_e3328: f64 = (assign3750_e3322 * assign3750_e3327);
            locals.var_stvfbedge_i = assign3750_e3328;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cic1edge_i = p.p370;
            locals.var_cic2edge_i = p.p371;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3780_e3345: f64 = (p.p372 * 2.0);
            let assign3780_e3348: f64 = (locals.var_lambda_le).powf(p.p373);
            let assign3780_e3349: f64 = (assign3780_e3345 * assign3780_e3348);
            let assign3780_e3353: f64 = (p.p374 * locals.var_iwe);
            let assign3780_e3354: f64 = (1.0 + assign3780_e3353);
            let assign3780_e3355: f64 = (assign3780_e3349 * assign3780_e3354);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3780_e3355, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3790_e3362: f64 = (locals.var_temp).max(0.0);
            let assign3790_e3364: f64 = (assign3790_e3362).min(5.0);
            (locals.var_psce1edge_i, locals.var_psce1edge_i_dn4, locals.var_psce1edge_i_dn6, locals.var_psce1edge_i_dn7, locals.var_psce1edge_i_dn8, locals.var_psce1edge_i_dn9, ) = (assign3790_e3364, if assign3790_e3362 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 } } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3800_e3371: f64 = (p.p375 * locals.var_psce1edge_i);
            let assign3800_e3373: f64 = (assign3800_e3371 * locals.var_tox2_i);
            let assign3800_e3375: f64 = (assign3800_e3373 / locals.var_tox1_i);
            (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9, ) = (assign3800_e3375, (((p.p375 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3810_e3382: f64 = (locals.var_lambda_le).powf(p.p377);
            let assign3810_e3386: f64 = (p.p378 * locals.var_iwe);
            let assign3810_e3387: f64 = (1.0 + assign3810_e3386);
            let assign3810_e3388: f64 = (assign3810_e3382 * assign3810_e3387);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3810_e3388, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3820_e3395: f64 = (p.p376 * locals.var_temp);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3820_e3395, (p.p376 * locals.var_temp_dn4), (p.p376 * locals.var_temp_dn6), (p.p376 * locals.var_temp_dn7), (p.p376 * locals.var_temp_dn8), (p.p376 * locals.var_temp_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3830_e3402: f64 = (locals.var_temp).max(0.0);
            (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9, ) = (assign3830_e3402, if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3840_e3409: f64 = (p.p379 * locals.var_cf1edge_i);
            let assign3840_e3411: f64 = (assign3840_e3409 * locals.var_tox2_i);
            let assign3840_e3413: f64 = (assign3840_e3411 / locals.var_tox1_i);
            (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9, ) = (assign3840_e3413, (((p.p379 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p379 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p379 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p379 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p379 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cfdedge_i = p.p380;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3860_e3426: f64 = (p.p381 * p.p382);
            let assign3860_e3428: f64 = (assign3860_e3426 / locals.var_le);
            let assign3860_e3431: f64 = (-locals.var_le);
            let assign3860_e3433: f64 = (assign3860_e3431 / p.p382);
            let assign3860_e3434: f64 = (assign3860_e3433).exp();
            let assign3860_e3435: f64 = (1.0 - assign3860_e3434);
            let assign3860_e3436: f64 = (assign3860_e3428 * assign3860_e3435);
            let assign3860_e3437: f64 = (1.0 + assign3860_e3436);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3860_e3437, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3870_e3444: f64 = (locals.var_temp).max(1e-15);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3870_e3444, if locals.var_temp >= 1e-15 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3880_e3451: f64 = (p.p244 * locals.var_we_edge);
            let assign3880_e3454: f64 = (locals.var_temp * locals.var_le);
            let assign3880_e3455: f64 = (assign3880_e3451 / assign3880_e3454);
            let assign3880_e3459: f64 = (p.p383 * locals.var_iwe);
            let assign3880_e3460: f64 = (1.0 + assign3880_e3459);
            let assign3880_e3461: f64 = (assign3880_e3455 * assign3880_e3460);
            (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9, ) = (assign3880_e3461, ((-((assign3880_e3451 * (locals.var_temp_dn4 * locals.var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (locals.var_temp_dn6 * locals.var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (locals.var_temp_dn7 * locals.var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (locals.var_temp_dn8 * locals.var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (locals.var_temp_dn9 * locals.var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3890_e3469: f64 = (p.p385 * locals.var_ile);
            let assign3890_e3470: f64 = (p.p384 + assign3890_e3469);
            let assign3890_e3473: f64 = (p.p386 * locals.var_iwe);
            let assign3890_e3474: f64 = (assign3890_e3470 + assign3890_e3473);
            let assign3890_e3477: f64 = (p.p387 * locals.var_ile);
            let assign3890_e3479: f64 = (assign3890_e3477 * locals.var_iwe);
            let assign3890_e3480: f64 = (assign3890_e3474 + assign3890_e3479);
            locals.var_stbetedge_i = assign3890_e3480;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3900_e3487: f64 = (locals.var_wecv * locals.var_lecv);
            locals.var_areaq_i = assign3900_e3487;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3910_e3495: f64 = (p.p397 * locals.var_lphy);
            let assign3910_e3496: f64 = (p.p396 + assign3910_e3495);
            (locals.var_cgbov_p, locals.var_cgbov_p_dn4, locals.var_cgbov_p_dn6, locals.var_cgbov_p_dn7, locals.var_cgbov_p_dn8, locals.var_cgbov_p_dn9, ) = (assign3910_e3496, (p.p397 * locals.var_lphy_dn4), (p.p397 * locals.var_lphy_dn6), (p.p397 * locals.var_lphy_dn7), (p.p397 * locals.var_lphy_dn8), (p.p397 * locals.var_lphy_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3920_e3503: f64 = (locals.var_cgbov_p).max(0.0);
            (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9, ) = (assign3920_e3503, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn4 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn6 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn7 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn8 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3930_e3510: f64 = (p.p398 * 1000000.0);
            locals.var_nsdac_i = assign3930_e3510;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3940_e3517: f64 = (p.p399 * locals.var_wecv);
            let assign3940_e3519: f64 = (assign3940_e3517 / locals.var_wen);
            locals.var_fif_i = assign3940_e3519;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_fsceac_i = p.p400;
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, );
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, );
            locals.var_psceac1_i = locals.var_psce1_i;
            locals.var_psceac2_i = locals.var_psce2_i;
            (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9, ) = (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9, );
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, );
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, );
            (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9, ) = (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9, );
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, );
            locals.var_axac_i = locals.var_ax_i;
            locals.var_alpac_i = locals.var_alp_i;
        }

        let assign4070_e3584: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign4070_e3584;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbaco_i = p.p211;
        }

        let assign4090_e3593: f64 = if param_given[401] { 1.0 } else { 0.0 };
        let assign4090_e3595: f64 = if assign4090_e3593 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign4090_e3595;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
            locals.var_vfbaco_i = p.p401;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbacl_i = p.p212;
        }

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign4120_e3613: f64 = if param_given[402] { 1.0 } else { 0.0 };
        let assign4120_e3615: f64 = if assign4120_e3613 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign4120_e3615;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
            locals.var_vfbacl_i = p.p402;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbaclexp_i = p.p213;
        }

        let assign4150_e3633: f64 = if param_given[403] { 1.0 } else { 0.0 };
        let assign4150_e3635: f64 = if assign4150_e3633 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign4150_e3635;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 != 0.0)) {
            locals.var_vfbaclexp_i = p.p403;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbacw_i = p.p216;
        }

        let assign4180_e3653: f64 = if param_given[406] { 1.0 } else { 0.0 };
        let assign4180_e3655: f64 = if assign4180_e3653 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign4180_e3655;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard102 != 0.0)) {
            locals.var_vfbacw_i = p.p406;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbaclw_i = p.p217;
        }

        let assign4210_e3673: f64 = if param_given[407] { 1.0 } else { 0.0 };
        let assign4210_e3675: f64 = if assign4210_e3673 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign4210_e3675;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard103 != 0.0)) {
            locals.var_vfbaclw_i = p.p407;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbacl2_i = p.p214;
        }

        let assign4240_e3693: f64 = if param_given[404] { 1.0 } else { 0.0 };
        let assign4240_e3695: f64 = if assign4240_e3693 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign4240_e3695;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard104 != 0.0)) {
            locals.var_vfbacl2_i = p.p404;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbaclexp2_i = p.p215;
        }

        let assign4270_e3713: f64 = if param_given[405] { 1.0 } else { 0.0 };
        let assign4270_e3715: f64 = if assign4270_e3713 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign4270_e3715;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard105 != 0.0)) {
            locals.var_vfbaclexp2_i = p.p405;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4290_e3732: f64 = (locals.var_ile).powf(locals.var_vfbaclexp_i);
            let assign4290_e3733: f64 = (locals.var_vfbacl_i * assign4290_e3732);
            let assign4290_e3738: f64 = (locals.var_ile).powf(locals.var_vfbaclexp2_i);
            let assign4290_e3739: f64 = (locals.var_vfbacl2_i * assign4290_e3738);
            let assign4290_e3740: f64 = (1.0 + assign4290_e3739);
            let assign4290_e3741: f64 = (assign4290_e3733 / assign4290_e3740);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign4290_e3741, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4300_e3750: f64 = (locals.var_vfbaco_i + locals.var_temp);
            let assign4300_e3753: f64 = (locals.var_vfbacw_i * locals.var_iwe);
            let assign4300_e3754: f64 = (assign4300_e3750 + assign4300_e3753);
            let assign4300_e3757: f64 = (locals.var_vfbaclw_i * locals.var_iae);
            let assign4300_e3758: f64 = (assign4300_e3754 + assign4300_e3757);
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (assign4300_e3758, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbbaco_i = p.p218;
        }

        let assign4320_e3769: f64 = if param_given[408] { 1.0 } else { 0.0 };
        let assign4320_e3771: f64 = if assign4320_e3769 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign4320_e3771;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard106 != 0.0)) {
            locals.var_vfbbaco_i = p.p408;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfblbaco_i = p.p219;
        }

        let assign4350_e3789: f64 = if param_given[409] { 1.0 } else { 0.0 };
        let assign4350_e3791: f64 = if assign4350_e3789 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign4350_e3791;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard107 != 0.0)) {
            locals.var_vfblbaco_i = p.p409;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4370_e3808: f64 = (locals.var_vfblbaco_i * locals.var_tox2_i);
            let assign4370_e3810: f64 = (assign4370_e3808 / locals.var_tox1_i);
            let assign4370_e3812: f64 = (assign4370_e3810 * locals.var_temp);
            let assign4370_e3813: f64 = (locals.var_vfbbaco_i + assign4370_e3812);
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (assign4370_e3813, (assign4370_e3810 * locals.var_temp_dn4), (assign4370_e3810 * locals.var_temp_dn6), (assign4370_e3810 * locals.var_temp_dn7), (assign4370_e3810 * locals.var_temp_dn8), (assign4370_e3810 * locals.var_temp_dn9), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_psceacl_i = p.p228;
        }

        let assign4390_e3824: f64 = if param_given[410] { 1.0 } else { 0.0 };
        let assign4390_e3826: f64 = if assign4390_e3824 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign4390_e3826;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard108 != 0.0)) {
            locals.var_psceacl_i = p.p410;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_psceaclexp_i = p.p229;
        }

        let assign4420_e3844: f64 = if param_given[411] { 1.0 } else { 0.0 };
        let assign4420_e3846: f64 = if assign4420_e3844 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign4420_e3846;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard109 != 0.0)) {
            locals.var_psceaclexp_i = p.p411;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_psceacw_i = p.p230;
        }

        let assign4450_e3864: f64 = if param_given[412] { 1.0 } else { 0.0 };
        let assign4450_e3866: f64 = if assign4450_e3864 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign4450_e3866;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard110 != 0.0)) {
            locals.var_psceacw_i = p.p412;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4470_e3882: f64 = (locals.var_psceacl_i * 2.0);
            let assign4470_e3885: f64 = (locals.var_lambda_le).powf(locals.var_psceaclexp_i);
            let assign4470_e3886: f64 = (assign4470_e3882 * assign4470_e3885);
            let assign4470_e3890: f64 = (locals.var_psceacw_i * locals.var_iwe);
            let assign4470_e3891: f64 = (1.0 + assign4470_e3890);
            let assign4470_e3892: f64 = (assign4470_e3886 * assign4470_e3891);
            locals.var_psceac_p = assign4470_e3892;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4480_e3901: f64 = (locals.var_psceac_p).max(0.0);
            let assign4480_e3903: f64 = (assign4480_e3901).min(5.0);
            locals.var_psceac1_i = assign4480_e3903;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4490_e3912: f64 = (p.p231 * locals.var_psceac1_i);
            let assign4490_e3914: f64 = (assign4490_e3912 * locals.var_tox2_i);
            let assign4490_e3916: f64 = (assign4490_e3914 / locals.var_tox1_i);
            locals.var_psceac2_i = assign4490_e3916;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_cfacl_i = p.p235;
        }

        let assign4510_e3927: f64 = if param_given[413] { 1.0 } else { 0.0 };
        let assign4510_e3929: f64 = if assign4510_e3927 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign4510_e3929;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard111 != 0.0)) {
            locals.var_cfacl_i = p.p413;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_cfaclexp_i = p.p236;
        }

        let assign4540_e3947: f64 = if param_given[414] { 1.0 } else { 0.0 };
        let assign4540_e3949: f64 = if assign4540_e3947 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign4540_e3949;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard112 != 0.0)) {
            locals.var_cfaclexp_i = p.p414;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_cfacw_i = p.p237;
        }

        let assign4570_e3967: f64 = if param_given[415] { 1.0 } else { 0.0 };
        let assign4570_e3969: f64 = if assign4570_e3967 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign4570_e3969;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard113 != 0.0)) {
            locals.var_cfacw_i = p.p415;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4590_e3985: f64 = (locals.var_lambda_le).powf(locals.var_cfaclexp_i);
            let assign4590_e3989: f64 = (locals.var_cfacw_i * locals.var_iwe);
            let assign4590_e3990: f64 = (1.0 + assign4590_e3989);
            let assign4590_e3991: f64 = (assign4590_e3985 * assign4590_e3990);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign4590_e3991, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4600_e4000: f64 = (locals.var_cfacl_i * locals.var_temp);
            (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9, ) = (assign4600_e4000, (locals.var_cfacl_i * locals.var_temp_dn4), (locals.var_cfacl_i * locals.var_temp_dn6), (locals.var_cfacl_i * locals.var_temp_dn7), (locals.var_cfacl_i * locals.var_temp_dn8), (locals.var_cfacl_i * locals.var_temp_dn9), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4610_e4009: f64 = (locals.var_cfac_p).max(0.0);
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (assign4610_e4009, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 }, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4620_e4018: f64 = (p.p238 * locals.var_cfac1_t);
            let assign4620_e4020: f64 = (assign4620_e4018 * locals.var_tox2_i);
            let assign4620_e4022: f64 = (assign4620_e4020 / locals.var_tox1_i);
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (assign4620_e4022, (((p.p238 * locals.var_cfac1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p238 * locals.var_cfac1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p238 * locals.var_cfac1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p238 * locals.var_cfac1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p238 * locals.var_cfac1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesataco_i = p.p293;
        }

        let assign4640_e4033: f64 = if param_given[416] { 1.0 } else { 0.0 };
        let assign4640_e4035: f64 = if assign4640_e4033 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign4640_e4035;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard114 != 0.0)) {
            locals.var_thesataco_i = p.p416;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesatacl_i = p.p294;
        }

        let assign4670_e4053: f64 = if param_given[417] { 1.0 } else { 0.0 };
        let assign4670_e4055: f64 = if assign4670_e4053 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign4670_e4055;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard115 != 0.0)) {
            locals.var_thesatacl_i = p.p417;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesataclexp_i = p.p295;
        }

        let assign4700_e4073: f64 = if param_given[418] { 1.0 } else { 0.0 };
        let assign4700_e4075: f64 = if assign4700_e4073 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign4700_e4075;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard116 != 0.0)) {
            locals.var_thesataclexp_i = p.p418;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesatacw_i = p.p296;
        }

        let assign4730_e4093: f64 = if param_given[419] { 1.0 } else { 0.0 };
        let assign4730_e4095: f64 = if assign4730_e4093 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign4730_e4095;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard117 != 0.0)) {
            locals.var_thesatacw_i = p.p419;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesataclw_i = p.p297;
        }

        let assign4760_e4113: f64 = if param_given[420] { 1.0 } else { 0.0 };
        let assign4760_e4115: f64 = if assign4760_e4113 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign4760_e4115;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard118 != 0.0)) {
            locals.var_thesataclw_i = p.p420;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4780_e4134: f64 = (locals.var_ile).powf(locals.var_thesataclexp_i);
            let assign4780_e4135: f64 = (locals.var_thesatacl_i * assign4780_e4134);
            let assign4780_e4136: f64 = (locals.var_thesataco_i + assign4780_e4135);
            let assign4780_e4137: f64 = (locals.var_ge * assign4780_e4136);
            let assign4780_e4141: f64 = (locals.var_thesatacw_i * locals.var_iwe);
            let assign4780_e4142: f64 = (1.0 + assign4780_e4141);
            let assign4780_e4143: f64 = (assign4780_e4137 * assign4780_e4142);
            let assign4780_e4147: f64 = (locals.var_thesataclw_i * locals.var_iae);
            let assign4780_e4148: f64 = (1.0 + assign4780_e4147);
            let assign4780_e4149: f64 = (assign4780_e4143 * assign4780_e4148);
            (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9, ) = (assign4780_e4149, (((locals.var_ge_dn4 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((locals.var_ge_dn6 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((locals.var_ge_dn7 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((locals.var_ge_dn8 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((locals.var_ge_dn9 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4790_e4158: f64 = (locals.var_thesatac_p).max(0.0);
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (assign4790_e4158, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 }, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axaco_i = p.p304;
        }

        let assign4810_e4169: f64 = if param_given[421] { 1.0 } else { 0.0 };
        let assign4810_e4171: f64 = if assign4810_e4169 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign4810_e4171;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard119 != 0.0)) {
            locals.var_axaco_i = p.p421;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axacl_i = p.p305;
        }

        let assign4840_e4189: f64 = if param_given[422] { 1.0 } else { 0.0 };
        let assign4840_e4191: f64 = if assign4840_e4189 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign4840_e4191;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard120 != 0.0)) {
            locals.var_axacl_i = p.p422;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axaclexp_i = p.p306;
        }

        let assign4870_e4209: f64 = if param_given[423] { 1.0 } else { 0.0 };
        let assign4870_e4211: f64 = if assign4870_e4209 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign4870_e4211;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard121 != 0.0)) {
            locals.var_axaclexp_i = p.p423;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axacl2_i = p.p307;
        }

        let assign4900_e4229: f64 = if param_given[424] { 1.0 } else { 0.0 };
        let assign4900_e4231: f64 = if assign4900_e4229 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign4900_e4231;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard122 != 0.0)) {
            locals.var_axacl2_i = p.p424;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axaclexp2_i = p.p308;
        }

        let assign4930_e4249: f64 = if param_given[425] { 1.0 } else { 0.0 };
        let assign4930_e4251: f64 = if assign4930_e4249 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign4930_e4251;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard123 != 0.0)) {
            locals.var_axaclexp2_i = p.p425;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4950_e4270: f64 = (locals.var_ile).powf(locals.var_axaclexp_i);
            let assign4950_e4271: f64 = (locals.var_axacl_i * assign4950_e4270);
            let assign4950_e4276: f64 = (locals.var_ile).powf(locals.var_axaclexp2_i);
            let assign4950_e4277: f64 = (locals.var_axacl2_i * assign4950_e4276);
            let assign4950_e4278: f64 = (1.0 + assign4950_e4277);
            let assign4950_e4279: f64 = (assign4950_e4271 / assign4950_e4278);
            let assign4950_e4280: f64 = (1.0 + assign4950_e4279);
            let assign4950_e4281: f64 = (locals.var_axaco_i / assign4950_e4280);
            locals.var_axac_p = assign4950_e4281;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4960_e4290: f64 = (locals.var_axac_p).max(1.0);
            let assign4960_e4292: f64 = (assign4960_e4290).min(16.0);
            locals.var_axac_i = assign4960_e4292;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpacl1_i = p.p309;
        }

        let assign4980_e4303: f64 = if param_given[426] { 1.0 } else { 0.0 };
        let assign4980_e4305: f64 = if assign4980_e4303 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign4980_e4305;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard124 != 0.0)) {
            locals.var_alpacl1_i = p.p426;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpaclexp_i = p.p310;
        }

        let assign5010_e4323: f64 = if param_given[427] { 1.0 } else { 0.0 };
        let assign5010_e4325: f64 = if assign5010_e4323 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign5010_e4325;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard125 != 0.0)) {
            locals.var_alpaclexp_i = p.p427;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpacl2_i = p.p311;
        }

        let assign5040_e4343: f64 = if param_given[428] { 1.0 } else { 0.0 };
        let assign5040_e4345: f64 = if assign5040_e4343 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign5040_e4345;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard126 != 0.0)) {
            locals.var_alpacl2_i = p.p428;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpaclexp2_i = p.p312;
        }

        let assign5070_e4363: f64 = if param_given[429] { 1.0 } else { 0.0 };
        let assign5070_e4365: f64 = if assign5070_e4363 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign5070_e4365;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard127 != 0.0)) {
            locals.var_alpaclexp2_i = p.p429;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpacw_i = p.p313;
        }

        let assign5100_e4383: f64 = if param_given[430] { 1.0 } else { 0.0 };
        let assign5100_e4385: f64 = if assign5100_e4383 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign5100_e4385;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard128 != 0.0)) {
            locals.var_alpacw_i = p.p430;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign5120_e4402: f64 = (locals.var_ile).powf(locals.var_alpaclexp_i);
            let assign5120_e4403: f64 = (locals.var_alpacl1_i * assign5120_e4402);
            let assign5120_e4407: f64 = (locals.var_alpacw_i * locals.var_iwe);
            let assign5120_e4408: f64 = (1.0 + assign5120_e4407);
            let assign5120_e4409: f64 = (assign5120_e4403 * assign5120_e4408);
            let assign5120_e4414: f64 = (locals.var_ile).powf(locals.var_alpaclexp2_i);
            let assign5120_e4415: f64 = (locals.var_alpacl2_i * assign5120_e4414);
            let assign5120_e4416: f64 = (1.0 + assign5120_e4415);
            let assign5120_e4417: f64 = (assign5120_e4409 / assign5120_e4416);
            locals.var_alpac_p = assign5120_e4417;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign5130_e4426: f64 = (locals.var_alpac_p).max(0.0);
            locals.var_alpac_i = assign5130_e4426;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5140_e4433: f64 = (3.45313e-11 / locals.var_tox1_i);
            let assign5140_e4435: f64 = (assign5140_e4433 * locals.var_wecv);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5140_e4435, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5150_e4442: f64 = (locals.var_temp * p.p431);
            (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9, ) = (assign5150_e4442, (locals.var_temp_dn4 * p.p431), (locals.var_temp_dn6 * p.p431), (locals.var_temp_dn7 * p.p431), (locals.var_temp_dn8 * p.p431), (locals.var_temp_dn9 * p.p431), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5160_e4449: f64 = (locals.var_temp * p.p432);
            (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9, ) = (assign5160_e4449, (locals.var_temp_dn4 * p.p432), (locals.var_temp_dn6 * p.p432), (locals.var_temp_dn7 * p.p432), (locals.var_temp_dn8 * p.p432), (locals.var_temp_dn9 * p.p432), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5170_e4458: f64 = (p.p434 * locals.var_wen);
            let assign5170_e4460: f64 = (assign5170_e4458 / locals.var_wecv);
            let assign5170_e4461: f64 = (1.0 + assign5170_e4460);
            let assign5170_e4463: f64 = (assign5170_e4461).max(0.001);
            let assign5170_e4464: f64 = (p.p433 / assign5170_e4463);
            locals.var_covdl_i = assign5170_e4464;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_covdlb_i = p.p435;
            locals.var_dvfbov_i = p.p436;
        }

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard83 == 0.0) {
            let assign5200_e4482: f64 = (p.p439 * locals.var_wphy);
            let assign5200_e4483: f64 = (p.p437 + assign5200_e4482);
            (locals.var_cfr_p, locals.var_cfr_p_dn4, locals.var_cfr_p_dn6, locals.var_cfr_p_dn7, locals.var_cfr_p_dn8, locals.var_cfr_p_dn9, ) = (assign5200_e4483, (p.p439 * locals.var_wphy_dn4), (p.p439 * locals.var_wphy_dn6), (p.p439 * locals.var_wphy_dn7), (p.p439 * locals.var_wphy_dn8), (p.p439 * locals.var_wphy_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5210_e4490: f64 = (locals.var_cfr_p).max(0.0);
            (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9, ) = (assign5210_e4490, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn4 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn6 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn7 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn8 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5220_e4498: f64 = (p.p440 * locals.var_wphy);
            let assign5220_e4499: f64 = (p.p438 + assign5220_e4498);
            (locals.var_cfrd_p, locals.var_cfrd_p_dn4, locals.var_cfrd_p_dn6, locals.var_cfrd_p_dn7, locals.var_cfrd_p_dn8, locals.var_cfrd_p_dn9, ) = (assign5220_e4499, (p.p440 * locals.var_wphy_dn4), (p.p440 * locals.var_wphy_dn6), (p.p440 * locals.var_wphy_dn7), (p.p440 * locals.var_wphy_dn8), (p.p440 * locals.var_wphy_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5230_e4506: f64 = (locals.var_cfrd_p).max(0.0);
            (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9, ) = (assign5230_e4506, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn4 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn6 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn7 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn8 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5240_e4513: f64 = (p.p441 * locals.var_epsch);
            let assign5240_e4515: f64 = (assign5240_e4513 * locals.var_tsi_i);
            let assign5240_e4517: f64 = (assign5240_e4515 * locals.var_we);
            let assign5240_e4519: f64 = (assign5240_e4517 / locals.var_le);
            locals.var_csd_i = assign5240_e4519;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_csdbp_i = p.p442;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5260_e4532: f64 = (p.p444 * locals.var_lphy);
            let assign5260_e4533: f64 = (1.0 + assign5260_e4532);
            let assign5260_e4536: f64 = (p.p445 * locals.var_wphy);
            let assign5260_e4537: f64 = (assign5260_e4533 + assign5260_e4536);
            let assign5260_e4540: f64 = (p.p446 * locals.var_lphy);
            let assign5260_e4542: f64 = (assign5260_e4540 * locals.var_wphy);
            let assign5260_e4543: f64 = (assign5260_e4537 + assign5260_e4542);
            let assign5260_e4545: f64 = (assign5260_e4543).max(1e-10);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5260_e4545, if assign5260_e4543 >= 1e-10 { (((p.p444 * locals.var_lphy_dn4) + (p.p445 * locals.var_wphy_dn4)) + (((p.p446 * locals.var_lphy_dn4) * locals.var_wphy) + (assign5260_e4540 * locals.var_wphy_dn4))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * locals.var_lphy_dn6) + (p.p445 * locals.var_wphy_dn6)) + (((p.p446 * locals.var_lphy_dn6) * locals.var_wphy) + (assign5260_e4540 * locals.var_wphy_dn6))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * locals.var_lphy_dn7) + (p.p445 * locals.var_wphy_dn7)) + (((p.p446 * locals.var_lphy_dn7) * locals.var_wphy) + (assign5260_e4540 * locals.var_wphy_dn7))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * locals.var_lphy_dn8) + (p.p445 * locals.var_wphy_dn8)) + (((p.p446 * locals.var_lphy_dn8) * locals.var_wphy) + (assign5260_e4540 * locals.var_wphy_dn8))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * locals.var_lphy_dn9) + (p.p445 * locals.var_wphy_dn9)) + (((p.p446 * locals.var_lphy_dn9) * locals.var_wphy) + (assign5260_e4540 * locals.var_wphy_dn9))) } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign5280_e4559: f64 = if ((p.p29 > 1.0) && (p.p28 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign5280_e4559;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
            let assign5290_e4566: f64 = (p.p28 + p.p20);
            let assign5290_e4567: f64 = (-assign5290_e4566);
            let assign5290_e4569: f64 = (assign5290_e4567 / p.p449);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign5290_e4569, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign5300_e4573: f64 = (locals.var_temp2).abs();
        let assign5300_e4575: f64 = if assign5300_e4573 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign5300_e4575;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 != 0.0)) {
            let assign5310_e4583: f64 = (locals.var_temp2).exp();
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign5310_e4583, (assign5310_e4583 * locals.var_temp2_dn4), (assign5310_e4583 * locals.var_temp2_dn6), (assign5310_e4583 * locals.var_temp2_dn7), (assign5310_e4583 * locals.var_temp2_dn8), (assign5310_e4583 * locals.var_temp2_dn9), );
        }

        let assign5320_e4588: f64 = (-80.0);
        let assign5320_e4589: f64 = if locals.var_temp2 < assign5320_e4588 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign5320_e4589;

        if ((((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 == 0.0)) && (locals.var_guard131 != 0.0)) {
            let assign5330_e4602: f64 = (-locals.var_temp2);
            let assign5330_e4604: f64 = (assign5330_e4602 - 80.0);
            let assign5330_e4608: f64 = (-locals.var_temp2);
            let assign5330_e4610: f64 = (assign5330_e4608 - 80.0);
            let assign5330_e4611: f64 = (0.5 * assign5330_e4610);
            let assign5330_e4614: f64 = (-locals.var_temp2);
            let assign5330_e4616: f64 = (assign5330_e4614 - 80.0);
            let assign5330_e4618: f64 = (assign5330_e4616 * 0.3333333333333);
            let assign5330_e4619: f64 = (1.0 + assign5330_e4618);
            let assign5330_e4620: f64 = (assign5330_e4611 * assign5330_e4619);
            let assign5330_e4621: f64 = (1.0 + assign5330_e4620);
            let assign5330_e4622: f64 = (assign5330_e4604 * assign5330_e4621);
            let assign5330_e4623: f64 = (1.0 + assign5330_e4622);
            let assign5330_e4624: f64 = (1.80485e-35 / assign5330_e4623);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign5330_e4624, (-((1.80485e-35 * (((-locals.var_temp2_dn4) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-locals.var_temp2_dn4)) * assign5330_e4619) + (assign5330_e4611 * ((-locals.var_temp2_dn4) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-locals.var_temp2_dn6) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-locals.var_temp2_dn6)) * assign5330_e4619) + (assign5330_e4611 * ((-locals.var_temp2_dn6) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-locals.var_temp2_dn7) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-locals.var_temp2_dn7)) * assign5330_e4619) + (assign5330_e4611 * ((-locals.var_temp2_dn7) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-locals.var_temp2_dn8) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-locals.var_temp2_dn8)) * assign5330_e4619) + (assign5330_e4611 * ((-locals.var_temp2_dn8) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-locals.var_temp2_dn9) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-locals.var_temp2_dn9)) * assign5330_e4619) + (assign5330_e4611 * ((-locals.var_temp2_dn9) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), );
        }

        if ((((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 == 0.0)) && (locals.var_guard131 == 0.0)) {
            let assign5340_e4641: f64 = (locals.var_temp2 - 80.0);
            let assign5340_e4646: f64 = (locals.var_temp2 - 80.0);
            let assign5340_e4647: f64 = (0.5 * assign5340_e4646);
            let assign5340_e4651: f64 = (locals.var_temp2 - 80.0);
            let assign5340_e4653: f64 = (assign5340_e4651 * 0.3333333333333);
            let assign5340_e4654: f64 = (1.0 + assign5340_e4653);
            let assign5340_e4655: f64 = (assign5340_e4647 * assign5340_e4654);
            let assign5340_e4656: f64 = (1.0 + assign5340_e4655);
            let assign5340_e4657: f64 = (assign5340_e4641 * assign5340_e4656);
            let assign5340_e4658: f64 = (1.0 + assign5340_e4657);
            let assign5340_e4659: f64 = (5.54062e34 * assign5340_e4658);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign5340_e4659, (5.54062e34 * ((locals.var_temp2_dn4 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * locals.var_temp2_dn4) * assign5340_e4654) + (assign5340_e4647 * (locals.var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn6 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * locals.var_temp2_dn6) * assign5340_e4654) + (assign5340_e4647 * (locals.var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn7 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * locals.var_temp2_dn7) * assign5340_e4654) + (assign5340_e4647 * (locals.var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn8 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * locals.var_temp2_dn8) * assign5340_e4654) + (assign5340_e4647 * (locals.var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn9 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * locals.var_temp2_dn9) * assign5340_e4654) + (assign5340_e4647 * (locals.var_temp2_dn9 * 0.3333333333333)))))), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
            let assign5350_e4668: f64 = (1.0 - locals.var_temp3);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign5350_e4668, (-locals.var_temp3_dn4), (-locals.var_temp3_dn6), (-locals.var_temp3_dn7), (-locals.var_temp3_dn8), (-locals.var_temp3_dn9), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
            let assign5360_e4677: f64 = (2.0 * p.p450);
            let assign5360_e4679: f64 = (assign5360_e4677 * locals.var_temp3);
            let assign5360_e4684: f64 = (locals.var_temp3).powf(p.p29);
            let assign5360_e4685: f64 = (1.0 - assign5360_e4684);
            let assign5360_e4687: f64 = (assign5360_e4685 / p.p29);
            let assign5360_e4688: f64 = (locals.var_temp4 - assign5360_e4687);
            let assign5360_e4689: f64 = (assign5360_e4679 * assign5360_e4688);
            let assign5360_e4692: f64 = (locals.var_temp4 * locals.var_temp4);
            let assign5360_e4693: f64 = (assign5360_e4689 / assign5360_e4692);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign5360_e4693, ((((((assign5360_e4677 * locals.var_temp3_dn4) * assign5360_e4688) + (assign5360_e4679 * (locals.var_temp4_dn4 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn4)) } } else { (assign5360_e4684 * (p.p29 * (locals.var_temp3_dn4 / locals.var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((locals.var_temp4_dn4 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn4)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * locals.var_temp3_dn6) * assign5360_e4688) + (assign5360_e4679 * (locals.var_temp4_dn6 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn6)) } } else { (assign5360_e4684 * (p.p29 * (locals.var_temp3_dn6 / locals.var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((locals.var_temp4_dn6 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn6)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * locals.var_temp3_dn7) * assign5360_e4688) + (assign5360_e4679 * (locals.var_temp4_dn7 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn7)) } } else { (assign5360_e4684 * (p.p29 * (locals.var_temp3_dn7 / locals.var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((locals.var_temp4_dn7 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn7)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * locals.var_temp3_dn8) * assign5360_e4688) + (assign5360_e4679 * (locals.var_temp4_dn8 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn8)) } } else { (assign5360_e4684 * (p.p29 * (locals.var_temp3_dn8 / locals.var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((locals.var_temp4_dn8 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn8)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * locals.var_temp3_dn9) * assign5360_e4688) + (assign5360_e4679 * (locals.var_temp4_dn9 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn9)) } } else { (assign5360_e4684 * (p.p29 * (locals.var_temp3_dn9 / locals.var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((locals.var_temp4_dn9 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn9)))) / (assign5360_e4692 * assign5360_e4692)), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5370_e4701: f64 = (1.0 + locals.var_temp1);
            let assign5370_e4702: f64 = (locals.var_temp / assign5370_e4701);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5370_e4702, (((locals.var_temp_dn4 * assign5370_e4701) - (locals.var_temp * locals.var_temp1_dn4)) / (assign5370_e4701 * assign5370_e4701)), (((locals.var_temp_dn6 * assign5370_e4701) - (locals.var_temp * locals.var_temp1_dn6)) / (assign5370_e4701 * assign5370_e4701)), (((locals.var_temp_dn7 * assign5370_e4701) - (locals.var_temp * locals.var_temp1_dn7)) / (assign5370_e4701 * assign5370_e4701)), (((locals.var_temp_dn8 * assign5370_e4701) - (locals.var_temp * locals.var_temp1_dn8)) / (assign5370_e4701 * assign5370_e4701)), (((locals.var_temp_dn9 * assign5370_e4701) - (locals.var_temp * locals.var_temp1_dn9)) / (assign5370_e4701 * assign5370_e4701)), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5380_e4709: f64 = (p.p443 / locals.var_temp);
            (locals.var_rth_p, locals.var_rth_p_dn4, locals.var_rth_p_dn6, locals.var_rth_p_dn7, locals.var_rth_p_dn8, locals.var_rth_p_dn9, ) = (assign5380_e4709, (-((p.p443 * locals.var_temp_dn4) / (locals.var_temp * locals.var_temp))), (-((p.p443 * locals.var_temp_dn6) / (locals.var_temp * locals.var_temp))), (-((p.p443 * locals.var_temp_dn7) / (locals.var_temp * locals.var_temp))), (-((p.p443 * locals.var_temp_dn8) / (locals.var_temp * locals.var_temp))), (-((p.p443 * locals.var_temp_dn9) / (locals.var_temp * locals.var_temp))), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5390_e4716: f64 = (locals.var_rth_p).max(1e-6);
            (locals.var_rth_t, locals.var_rth_t_dn4, locals.var_rth_t_dn6, locals.var_rth_t_dn7, locals.var_rth_t_dn8, locals.var_rth_t_dn9, ) = (assign5390_e4716, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn4 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn6 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn7 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn8 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_strth_i = p.p447;
            locals.var_fnt_i = p.p451;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5440_e4747: f64 = (p.p452 * locals.var_betn_p);
            let assign5440_e4749: f64 = (assign5440_e4747 * locals.var_betn_p);
            let assign5440_e4751: f64 = (assign5440_e4749 * locals.var_iwe);
            let assign5440_e4753: f64 = (assign5440_e4751 * locals.var_iwe);
            let assign5440_e4757: f64 = (p.p453 - 2.0);
            let assign5440_e4758: f64 = (locals.var_ile).powf(assign5440_e4757);
            let assign5440_e4759: f64 = (assign5440_e4753 * assign5440_e4758);
            locals.var_fntexc_i = assign5440_e4759;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5450_e4766: f64 = (p.p454 * locals.var_iae);
            let assign5450_e4769: f64 = (p.p455 * locals.var_iwe);
            let assign5450_e4770: f64 = (assign5450_e4766 + assign5450_e4769);
            locals.var_nfa_p = assign5450_e4770;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5460_e4777: f64 = (locals.var_nfa_p).max(0.0);
            locals.var_nfa_i = assign5460_e4777;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5470_e4784: f64 = (p.p456 * locals.var_iae);
            locals.var_nfb_i = assign5470_e4784;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5480_e4791: f64 = (p.p457 * locals.var_iae);
            locals.var_nfc_i = assign5480_e4791;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_nfe_i = p.p458;
            locals.var_nfeb_i = p.p459;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5520_e4814: f64 = (p.p490 * locals.var_ile);
            let assign5520_e4815: f64 = (p.p489 + assign5520_e4814);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5520_e4815, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5530_e4822: f64 = (locals.var_temp).max(0.0);
            (locals.var_kdrift_i, locals.var_kdrift_i_dn4, locals.var_kdrift_i_dn6, locals.var_kdrift_i_dn7, locals.var_kdrift_i_dn8, locals.var_kdrift_i_dn9, ) = (assign5530_e4822, if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5540_e4830: f64 = (p.p492 * locals.var_ile);
            let assign5540_e4831: f64 = (p.p491 + assign5540_e4830);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5540_e4831, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5550_e4838: f64 = (locals.var_temp).max(0.0);
            (locals.var_kdiff_i, locals.var_kdiff_i_dn4, locals.var_kdiff_i_dn6, locals.var_kdiff_i_dn7, locals.var_kdiff_i_dn8, locals.var_kdiff_i_dn9, ) = (assign5550_e4838, if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_fracinv_i = p.p493;
            locals.var_kfracinv_i = p.p494;
        }

        let assign5670_e4958: f64 = if ((((p.p461 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign5670_e4958;

        let assign5680_e4961: f64 = if p.p461 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign5680_e4961;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_tmpb = 0.0;
            locals.var_iloop = 0.0;
        }

        let mut assign5720_loop_guard: usize = 0;
        while {
            let assign5720_cond_e4998: f64 = (p.p29 - 0.5);
            let assign5720_cond_e5000: f64 = if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) && (locals.var_iloop < assign5720_cond_e4998)) { 1.0 } else { 0.0 };
            assign5720_cond_e5000 != 0.0
        } {
            assign5720_loop_guard += 1;
            assert!(assign5720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
                let assign5720_body0_e5012: f64 = (0.5 * p.p20);
                let assign5720_body0_e5013: f64 = (p.p26 + assign5720_body0_e5012);
                let assign5720_body0_e5017: f64 = (p.p28 + p.p20);
                let assign5720_body0_e5018: f64 = (locals.var_iloop * assign5720_body0_e5017);
                let assign5720_body0_e5019: f64 = (assign5720_body0_e5013 + assign5720_body0_e5018);
                let assign5720_body0_e5020: f64 = (1.0 / assign5720_body0_e5019);
                let assign5720_body0_e5021: f64 = (locals.var_tmpa + assign5720_body0_e5020);
                (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, ) = (assign5720_body0_e5021, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
                let assign5720_body1_e5035: f64 = (0.5 * p.p20);
                let assign5720_body1_e5036: f64 = (p.p27 + assign5720_body1_e5035);
                let assign5720_body1_e5040: f64 = (p.p28 + p.p20);
                let assign5720_body1_e5041: f64 = (locals.var_iloop * assign5720_body1_e5040);
                let assign5720_body1_e5042: f64 = (assign5720_body1_e5036 + assign5720_body1_e5041);
                let assign5720_body1_e5043: f64 = (1.0 / assign5720_body1_e5042);
                let assign5720_body1_e5044: f64 = (locals.var_tmpb + assign5720_body1_e5043);
                locals.var_tmpb = assign5720_body1_e5044;
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
                let assign5720_body2_e5055: f64 = (locals.var_iloop + 1.0);
                locals.var_iloop = assign5720_body2_e5055;
            }
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5730_e5066: f64 = (locals.var_tmpa / p.p29);
            (locals.var_invsa, locals.var_invsa_dn4, locals.var_invsa_dn6, locals.var_invsa_dn7, locals.var_invsa_dn8, locals.var_invsa_dn9, ) = (assign5730_e5066, (locals.var_tmpa_dn4 / p.p29), (locals.var_tmpa_dn6 / p.p29), (locals.var_tmpa_dn7 / p.p29), (locals.var_tmpa_dn8 / p.p29), (locals.var_tmpa_dn9 / p.p29), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5740_e5077: f64 = (locals.var_tmpb / p.p29);
            locals.var_invsb = assign5740_e5077;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5750_e5090: f64 = (0.5 * p.p20);
            let assign5750_e5091: f64 = (p.p462 + assign5750_e5090);
            let assign5750_e5092: f64 = (1.0 / assign5750_e5091);
            locals.var_invsaref = assign5750_e5092;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5760_e5105: f64 = (0.5 * p.p20);
            let assign5760_e5106: f64 = (p.p463 + assign5760_e5105);
            let assign5760_e5107: f64 = (1.0 / assign5760_e5106);
            locals.var_invsbref = assign5760_e5107;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5770_e5118: f64 = (p.p20 + locals.var_dellps);
            let assign5770_e5120: f64 = (assign5770_e5118).max(1e-9);
            locals.var_lx = assign5770_e5120;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5780_e5131: f64 = (locals.var_w_i + locals.var_delwod);
            let assign5780_e5133: f64 = (assign5780_e5131 + p.p464);
            let assign5780_e5135: f64 = (assign5780_e5133).max(1e-9);
            locals.var_wx = assign5780_e5135;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5790_e5147: f64 = (locals.var_lx).powf(p.p471);
            let assign5790_e5148: f64 = (1.0 / assign5790_e5147);
            locals.var_templ = assign5790_e5148;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5800_e5160: f64 = (locals.var_wx).powf(p.p472);
            let assign5800_e5161: f64 = (1.0 / assign5800_e5160);
            locals.var_tempw = assign5800_e5161;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5810_e5173: f64 = (p.p468 * locals.var_templ);
            let assign5810_e5174: f64 = (1.0 + assign5810_e5173);
            let assign5810_e5177: f64 = (p.p469 * locals.var_tempw);
            let assign5810_e5178: f64 = (assign5810_e5174 + assign5810_e5177);
            let assign5810_e5181: f64 = (p.p470 * locals.var_templ);
            let assign5810_e5183: f64 = (assign5810_e5181 * locals.var_tempw);
            let assign5810_e5184: f64 = (assign5810_e5178 + assign5810_e5183);
            let assign5810_e5189: f64 = (locals.var_rt - 1.0);
            let assign5810_e5190: f64 = (p.p467 * assign5810_e5189);
            let assign5810_e5191: f64 = (1.0 + assign5810_e5190);
            let assign5810_e5192: f64 = (assign5810_e5184 * assign5810_e5191);
            (locals.var_kstressu0, locals.var_kstressu0_dn4, locals.var_kstressu0_dn6, locals.var_kstressu0_dn7, locals.var_kstressu0_dn8, locals.var_kstressu0_dn9, ) = (assign5810_e5192, (assign5810_e5184 * (p.p467 * locals.var_rt_dn4)), (assign5810_e5184 * (p.p467 * locals.var_rt_dn6)), (assign5810_e5184 * (p.p467 * locals.var_rt_dn7)), (assign5810_e5184 * (p.p467 * locals.var_rt_dn8)), (assign5810_e5184 * (p.p467 * locals.var_rt_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5820_e5204: f64 = (locals.var_invsa + locals.var_invsb);
            let assign5820_e5205: f64 = (p.p465 * assign5820_e5204);
            let assign5820_e5207: f64 = (assign5820_e5205 / locals.var_kstressu0);
            (locals.var_rhobeta, locals.var_rhobeta_dn4, locals.var_rhobeta_dn6, locals.var_rhobeta_dn7, locals.var_rhobeta_dn8, locals.var_rhobeta_dn9, ) = (assign5820_e5207, ((((p.p465 * locals.var_invsa_dn4) * locals.var_kstressu0) - (assign5820_e5205 * locals.var_kstressu0_dn4)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p465 * locals.var_invsa_dn6) * locals.var_kstressu0) - (assign5820_e5205 * locals.var_kstressu0_dn6)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p465 * locals.var_invsa_dn7) * locals.var_kstressu0) - (assign5820_e5205 * locals.var_kstressu0_dn7)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p465 * locals.var_invsa_dn8) * locals.var_kstressu0) - (assign5820_e5205 * locals.var_kstressu0_dn8)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p465 * locals.var_invsa_dn9) * locals.var_kstressu0) - (assign5820_e5205 * locals.var_kstressu0_dn9)) / (locals.var_kstressu0 * locals.var_kstressu0)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5830_e5219: f64 = (locals.var_invsaref + locals.var_invsbref);
            let assign5830_e5220: f64 = (p.p465 * assign5830_e5219);
            let assign5830_e5222: f64 = (assign5830_e5220 / locals.var_kstressu0);
            (locals.var_rhobetaref, locals.var_rhobetaref_dn4, locals.var_rhobetaref_dn6, locals.var_rhobetaref_dn7, locals.var_rhobetaref_dn8, locals.var_rhobetaref_dn9, ) = (assign5830_e5222, (-((assign5830_e5220 * locals.var_kstressu0_dn4) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5830_e5220 * locals.var_kstressu0_dn6) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5830_e5220 * locals.var_kstressu0_dn7) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5830_e5220 * locals.var_kstressu0_dn8) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5830_e5220 * locals.var_kstressu0_dn9) / (locals.var_kstressu0 * locals.var_kstressu0))), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5840_e5234: f64 = (locals.var_lx).powf(p.p477);
            let assign5840_e5235: f64 = (1.0 / assign5840_e5234);
            locals.var_templ = assign5840_e5235;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5850_e5247: f64 = (locals.var_wx).powf(p.p478);
            let assign5850_e5248: f64 = (1.0 / assign5850_e5247);
            locals.var_tempw = assign5850_e5248;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5860_e5260: f64 = (p.p474 * locals.var_templ);
            let assign5860_e5261: f64 = (1.0 + assign5860_e5260);
            let assign5860_e5264: f64 = (p.p475 * locals.var_tempw);
            let assign5860_e5265: f64 = (assign5860_e5261 + assign5860_e5264);
            let assign5860_e5268: f64 = (p.p476 * locals.var_templ);
            let assign5860_e5270: f64 = (assign5860_e5268 * locals.var_tempw);
            let assign5860_e5271: f64 = (assign5860_e5265 + assign5860_e5270);
            let assign5860_e5273: f64 = (assign5860_e5271).max(1e-20);
            locals.var_kstressvth0 = assign5860_e5273;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5870_e5284: f64 = (locals.var_invsa + locals.var_invsb);
            let assign5870_e5286: f64 = (assign5870_e5284 - locals.var_invsaref);
            let assign5870_e5288: f64 = (assign5870_e5286 - locals.var_invsbref);
            (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9, ) = (assign5870_e5288, locals.var_invsa_dn4, locals.var_invsa_dn6, locals.var_invsa_dn7, locals.var_invsa_dn8, locals.var_invsa_dn9, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5880_e5300: f64 = (1.0 + locals.var_rhobeta);
            let assign5880_e5301: f64 = (locals.var_betn_p * assign5880_e5300);
            let assign5880_e5304: f64 = (1.0 + locals.var_rhobetaref);
            let assign5880_e5305: f64 = (assign5880_e5301 / assign5880_e5304);
            (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9, ) = (assign5880_e5305, (((((locals.var_betn_p_dn4 * assign5880_e5300) + (locals.var_betn_p * locals.var_rhobeta_dn4)) * assign5880_e5304) - (assign5880_e5301 * locals.var_rhobetaref_dn4)) / (assign5880_e5304 * assign5880_e5304)), (((((locals.var_betn_p_dn6 * assign5880_e5300) + (locals.var_betn_p * locals.var_rhobeta_dn6)) * assign5880_e5304) - (assign5880_e5301 * locals.var_rhobetaref_dn6)) / (assign5880_e5304 * assign5880_e5304)), (((((locals.var_betn_p_dn7 * assign5880_e5300) + (locals.var_betn_p * locals.var_rhobeta_dn7)) * assign5880_e5304) - (assign5880_e5301 * locals.var_rhobetaref_dn7)) / (assign5880_e5304 * assign5880_e5304)), (((((locals.var_betn_p_dn8 * assign5880_e5300) + (locals.var_betn_p * locals.var_rhobeta_dn8)) * assign5880_e5304) - (assign5880_e5301 * locals.var_rhobetaref_dn8)) / (assign5880_e5304 * assign5880_e5304)), (((((locals.var_betn_p_dn9 * assign5880_e5300) + (locals.var_betn_p * locals.var_rhobeta_dn9)) * assign5880_e5304) - (assign5880_e5301 * locals.var_rhobetaref_dn9)) / (assign5880_e5304 * assign5880_e5304)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5890_e5316: f64 = (locals.var_betn_p).max(1e-10);
            (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9, ) = (assign5890_e5316, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5900_e5327: f64 = (p.p254 * locals.var_betn1_t);
            (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9, ) = (assign5900_e5327, (p.p254 * locals.var_betn1_t_dn4), (p.p254 * locals.var_betn1_t_dn6), (p.p254 * locals.var_betn1_t_dn7), (p.p254 * locals.var_betn1_t_dn8), (p.p254 * locals.var_betn1_t_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5910_e5338: f64 = (1.0 + locals.var_rhobeta);
            let assign5910_e5342: f64 = (p.p466 * locals.var_rhobetaref);
            let assign5910_e5343: f64 = (1.0 + assign5910_e5342);
            let assign5910_e5344: f64 = (assign5910_e5338 * assign5910_e5343);
            let assign5910_e5347: f64 = (1.0 + locals.var_rhobetaref);
            let assign5910_e5351: f64 = (p.p466 * locals.var_rhobeta);
            let assign5910_e5352: f64 = (1.0 + assign5910_e5351);
            let assign5910_e5353: f64 = (assign5910_e5347 * assign5910_e5352);
            let assign5910_e5354: f64 = (assign5910_e5344 / assign5910_e5353);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5910_e5354, (((((locals.var_rhobeta_dn4 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * locals.var_rhobetaref_dn4))) * assign5910_e5353) - (assign5910_e5344 * ((locals.var_rhobetaref_dn4 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * locals.var_rhobeta_dn4))))) / (assign5910_e5353 * assign5910_e5353)), (((((locals.var_rhobeta_dn6 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * locals.var_rhobetaref_dn6))) * assign5910_e5353) - (assign5910_e5344 * ((locals.var_rhobetaref_dn6 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * locals.var_rhobeta_dn6))))) / (assign5910_e5353 * assign5910_e5353)), (((((locals.var_rhobeta_dn7 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * locals.var_rhobetaref_dn7))) * assign5910_e5353) - (assign5910_e5344 * ((locals.var_rhobetaref_dn7 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * locals.var_rhobeta_dn7))))) / (assign5910_e5353 * assign5910_e5353)), (((((locals.var_rhobeta_dn8 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * locals.var_rhobetaref_dn8))) * assign5910_e5353) - (assign5910_e5344 * ((locals.var_rhobetaref_dn8 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * locals.var_rhobeta_dn8))))) / (assign5910_e5353 * assign5910_e5353)), (((((locals.var_rhobeta_dn9 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * locals.var_rhobetaref_dn9))) * assign5910_e5353) - (assign5910_e5344 * ((locals.var_rhobetaref_dn9 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * locals.var_rhobeta_dn9))))) / (assign5910_e5353 * assign5910_e5353)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5920_e5365: f64 = (locals.var_thesat_p * locals.var_temp);
            (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9, ) = (assign5920_e5365, ((locals.var_thesat_p_dn4 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn4)), ((locals.var_thesat_p_dn6 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn6)), ((locals.var_thesat_p_dn7 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn7)), ((locals.var_thesat_p_dn8 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn8)), ((locals.var_thesat_p_dn9 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5930_e5376: f64 = (locals.var_thesat_p).max(0.0);
            (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, ) = (assign5930_e5376, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5940_e5387: f64 = (locals.var_thesatac_p * locals.var_temp);
            (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9, ) = (assign5940_e5387, ((locals.var_thesatac_p_dn4 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn4)), ((locals.var_thesatac_p_dn6 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn6)), ((locals.var_thesatac_p_dn7 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn7)), ((locals.var_thesatac_p_dn8 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn8)), ((locals.var_thesatac_p_dn9 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5950_e5398: f64 = (locals.var_thesatac_p).max(0.0);
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (assign5950_e5398, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5960_e5409: f64 = (p.p473 * locals.var_temp0__blk79);
            let assign5960_e5411: f64 = (assign5960_e5409 / locals.var_kstressvth0);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5960_e5411, ((p.p473 * locals.var_temp0__blk79_dn4) / locals.var_kstressvth0), ((p.p473 * locals.var_temp0__blk79_dn6) / locals.var_kstressvth0), ((p.p473 * locals.var_temp0__blk79_dn7) / locals.var_kstressvth0), ((p.p473 * locals.var_temp0__blk79_dn8) / locals.var_kstressvth0), ((p.p473 * locals.var_temp0__blk79_dn9) / locals.var_kstressvth0), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5970_e5422: f64 = (locals.var_vfb1_t + locals.var_temp);
            (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, ) = (assign5970_e5422, (locals.var_vfb1_t_dn4 + locals.var_temp_dn4), (locals.var_vfb1_t_dn6 + locals.var_temp_dn6), (locals.var_vfb1_t_dn7 + locals.var_temp_dn7), (locals.var_vfb1_t_dn8 + locals.var_temp_dn8), (locals.var_vfb1_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5980_e5433: f64 = (locals.var_vfb2_t + locals.var_temp);
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (assign5980_e5433, (locals.var_vfb2_t_dn4 + locals.var_temp_dn4), (locals.var_vfb2_t_dn6 + locals.var_temp_dn6), (locals.var_vfb2_t_dn7 + locals.var_temp_dn7), (locals.var_vfb2_t_dn8 + locals.var_temp_dn8), (locals.var_vfb2_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5990_e5444: f64 = (locals.var_vfbac1_t + locals.var_temp);
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (assign5990_e5444, (locals.var_vfbac1_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac1_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac1_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac1_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac1_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6000_e5455: f64 = (locals.var_vfbac2_t + locals.var_temp);
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (assign6000_e5455, (locals.var_vfbac2_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac2_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac2_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac2_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac2_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6010_e5466: f64 = (p.p479 * locals.var_temp0__blk79);
            let assign6010_e5469: f64 = (locals.var_kstressvth0).powf(p.p480);
            let assign6010_e5470: f64 = (assign6010_e5466 / assign6010_e5469);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6010_e5470, ((p.p479 * locals.var_temp0__blk79_dn4) / assign6010_e5469), ((p.p479 * locals.var_temp0__blk79_dn6) / assign6010_e5469), ((p.p479 * locals.var_temp0__blk79_dn7) / assign6010_e5469), ((p.p479 * locals.var_temp0__blk79_dn8) / assign6010_e5469), ((p.p479 * locals.var_temp0__blk79_dn9) / assign6010_e5469), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6020_e5481: f64 = (locals.var_cf_p + locals.var_temp);
            (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9, ) = (assign6020_e5481, (locals.var_cf_p_dn4 + locals.var_temp_dn4), (locals.var_cf_p_dn6 + locals.var_temp_dn6), (locals.var_cf_p_dn7 + locals.var_temp_dn7), (locals.var_cf_p_dn8 + locals.var_temp_dn8), (locals.var_cf_p_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6030_e5492: f64 = (locals.var_cf_p).max(0.0);
            (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, ) = (assign6030_e5492, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6040_e5503: f64 = (locals.var_cfac_p + locals.var_temp);
            (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9, ) = (assign6040_e5503, (locals.var_cfac_p_dn4 + locals.var_temp_dn4), (locals.var_cfac_p_dn6 + locals.var_temp_dn6), (locals.var_cfac_p_dn7 + locals.var_temp_dn7), (locals.var_cfac_p_dn8 + locals.var_temp_dn8), (locals.var_cfac_p_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6050_e5514: f64 = (locals.var_cfac_p).max(0.0);
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (assign6050_e5514, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6060_e5525: f64 = (p.p238 * locals.var_tox2_i);
            let assign6060_e5527: f64 = (assign6060_e5525 / locals.var_tox1_i);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6060_e5527, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6070_e5538: f64 = (locals.var_cf1_t * locals.var_temp);
            (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, ) = (assign6070_e5538, ((locals.var_cf1_t_dn4 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn4)), ((locals.var_cf1_t_dn6 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn6)), ((locals.var_cf1_t_dn7 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn7)), ((locals.var_cf1_t_dn8 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn8)), ((locals.var_cf1_t_dn9 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign6080_e5549: f64 = (locals.var_cfac1_t * locals.var_temp);
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (assign6080_e5549, ((locals.var_cfac1_t_dn4 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn4)), ((locals.var_cfac1_t_dn6 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn6)), ((locals.var_cfac1_t_dn7 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn7)), ((locals.var_cfac1_t_dn8 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn8)), ((locals.var_cfac1_t_dn9 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_iloop = 0.0;
        }

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6110_e5580: f64 = (-1.0);
            let assign6110_e5582: f64 = (assign6110_e5580 / p.p482);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6110_e5582, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let mut assign6120_loop_guard: usize = 0;
        while {
            let assign6120_cond_e5595: f64 = (p.p29 - 0.5);
            let assign6120_cond_e5597: f64 = if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_iloop < assign6120_cond_e5595)) { 1.0 } else { 0.0 };
            assign6120_cond_e5597 != 0.0
        } {
            assign6120_loop_guard += 1;
            assert!(assign6120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let assign6120_body0_e5601: f64 = (0.5 * p.p20);
            let assign6120_body0_e5602: f64 = (p.p26 + assign6120_body0_e5601);
            let assign6120_body0_e5606: f64 = (p.p28 + p.p20);
            let assign6120_body0_e5607: f64 = (locals.var_iloop * assign6120_body0_e5606);
            let assign6120_body0_e5608: f64 = (assign6120_body0_e5602 + assign6120_body0_e5607);
            let assign6120_body0_e5609: f64 = (-assign6120_body0_e5608);
            let assign6120_body0_e5611: f64 = (assign6120_body0_e5609 / p.p481);
            let assign6120_body0_e5613: f64 = (-80.0);
            let assign6120_body0_e5614: f64 = if assign6120_body0_e5611 > assign6120_body0_e5613 { 1.0 } else { 0.0 };
            locals.var_guard135 = assign6120_body0_e5614;
            if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard135 != 0.0)) {
                let assign6120_body1_e5627: f64 = (0.5 * p.p20);
                let assign6120_body1_e5628: f64 = (p.p26 + assign6120_body1_e5627);
                let assign6120_body1_e5632: f64 = (p.p28 + p.p20);
                let assign6120_body1_e5633: f64 = (locals.var_iloop * assign6120_body1_e5632);
                let assign6120_body1_e5634: f64 = (assign6120_body1_e5628 + assign6120_body1_e5633);
                let assign6120_body1_e5635: f64 = (-assign6120_body1_e5634);
                let assign6120_body1_e5637: f64 = (assign6120_body1_e5635 / p.p481);
                let assign6120_body1_e5638: f64 = (assign6120_body1_e5637).exp();
                (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6120_body1_e5638, 0.0, 0.0, 0.0, 0.0, 0.0, );
            }
            if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard135 == 0.0)) {
                let assign6120_body2_e5656: f64 = (0.5 * p.p20);
                let assign6120_body2_e5657: f64 = (p.p26 + assign6120_body2_e5656);
                let assign6120_body2_e5661: f64 = (p.p28 + p.p20);
                let assign6120_body2_e5662: f64 = (locals.var_iloop * assign6120_body2_e5661);
                let assign6120_body2_e5663: f64 = (assign6120_body2_e5657 + assign6120_body2_e5662);
                let assign6120_body2_e5664: f64 = (-assign6120_body2_e5663);
                let assign6120_body2_e5666: f64 = (assign6120_body2_e5664 / p.p481);
                let assign6120_body2_e5667: f64 = (-assign6120_body2_e5666);
                let assign6120_body2_e5669: f64 = (assign6120_body2_e5667 - 80.0);
                let assign6120_body2_e5675: f64 = (0.5 * p.p20);
                let assign6120_body2_e5676: f64 = (p.p26 + assign6120_body2_e5675);
                let assign6120_body2_e5680: f64 = (p.p28 + p.p20);
                let assign6120_body2_e5681: f64 = (locals.var_iloop * assign6120_body2_e5680);
                let assign6120_body2_e5682: f64 = (assign6120_body2_e5676 + assign6120_body2_e5681);
                let assign6120_body2_e5683: f64 = (-assign6120_body2_e5682);
                let assign6120_body2_e5685: f64 = (assign6120_body2_e5683 / p.p481);
                let assign6120_body2_e5686: f64 = (-assign6120_body2_e5685);
                let assign6120_body2_e5688: f64 = (assign6120_body2_e5686 - 80.0);
                let assign6120_body2_e5689: f64 = (0.5 * assign6120_body2_e5688);
                let assign6120_body2_e5694: f64 = (0.5 * p.p20);
                let assign6120_body2_e5695: f64 = (p.p26 + assign6120_body2_e5694);
                let assign6120_body2_e5699: f64 = (p.p28 + p.p20);
                let assign6120_body2_e5700: f64 = (locals.var_iloop * assign6120_body2_e5699);
                let assign6120_body2_e5701: f64 = (assign6120_body2_e5695 + assign6120_body2_e5700);
                let assign6120_body2_e5702: f64 = (-assign6120_body2_e5701);
                let assign6120_body2_e5704: f64 = (assign6120_body2_e5702 / p.p481);
                let assign6120_body2_e5705: f64 = (-assign6120_body2_e5704);
                let assign6120_body2_e5707: f64 = (assign6120_body2_e5705 - 80.0);
                let assign6120_body2_e5709: f64 = (assign6120_body2_e5707 * 0.3333333333333);
                let assign6120_body2_e5710: f64 = (1.0 + assign6120_body2_e5709);
                let assign6120_body2_e5711: f64 = (assign6120_body2_e5689 * assign6120_body2_e5710);
                let assign6120_body2_e5712: f64 = (1.0 + assign6120_body2_e5711);
                let assign6120_body2_e5713: f64 = (assign6120_body2_e5669 * assign6120_body2_e5712);
                let assign6120_body2_e5714: f64 = (1.0 + assign6120_body2_e5713);
                let assign6120_body2_e5715: f64 = (1.80485e-35 / assign6120_body2_e5714);
                (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6120_body2_e5715, 0.0, 0.0, 0.0, 0.0, 0.0, );
            }
            let assign6120_body3_e5721: f64 = (0.5 * p.p20);
            let assign6120_body3_e5722: f64 = (p.p27 + assign6120_body3_e5721);
            let assign6120_body3_e5725: f64 = (p.p29 - 1.0);
            let assign6120_body3_e5727: f64 = (assign6120_body3_e5725 - locals.var_iloop);
            let assign6120_body3_e5730: f64 = (p.p28 + p.p20);
            let assign6120_body3_e5731: f64 = (assign6120_body3_e5727 * assign6120_body3_e5730);
            let assign6120_body3_e5732: f64 = (assign6120_body3_e5722 + assign6120_body3_e5731);
            let assign6120_body3_e5733: f64 = (-assign6120_body3_e5732);
            let assign6120_body3_e5735: f64 = (assign6120_body3_e5733 / p.p481);
            let assign6120_body3_e5737: f64 = (-80.0);
            let assign6120_body3_e5738: f64 = if assign6120_body3_e5735 > assign6120_body3_e5737 { 1.0 } else { 0.0 };
            locals.var_guard136 = assign6120_body3_e5738;
            if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard136 != 0.0)) {
                let assign6120_body4_e5751: f64 = (0.5 * p.p20);
                let assign6120_body4_e5752: f64 = (p.p27 + assign6120_body4_e5751);
                let assign6120_body4_e5755: f64 = (p.p29 - 1.0);
                let assign6120_body4_e5757: f64 = (assign6120_body4_e5755 - locals.var_iloop);
                let assign6120_body4_e5760: f64 = (p.p28 + p.p20);
                let assign6120_body4_e5761: f64 = (assign6120_body4_e5757 * assign6120_body4_e5760);
                let assign6120_body4_e5762: f64 = (assign6120_body4_e5752 + assign6120_body4_e5761);
                let assign6120_body4_e5763: f64 = (-assign6120_body4_e5762);
                let assign6120_body4_e5765: f64 = (assign6120_body4_e5763 / p.p481);
                let assign6120_body4_e5766: f64 = (assign6120_body4_e5765).exp();
                (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign6120_body4_e5766, 0.0, 0.0, 0.0, 0.0, 0.0, );
            }
            if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard136 == 0.0)) {
                let assign6120_body5_e5784: f64 = (0.5 * p.p20);
                let assign6120_body5_e5785: f64 = (p.p27 + assign6120_body5_e5784);
                let assign6120_body5_e5788: f64 = (p.p29 - 1.0);
                let assign6120_body5_e5790: f64 = (assign6120_body5_e5788 - locals.var_iloop);
                let assign6120_body5_e5793: f64 = (p.p28 + p.p20);
                let assign6120_body5_e5794: f64 = (assign6120_body5_e5790 * assign6120_body5_e5793);
                let assign6120_body5_e5795: f64 = (assign6120_body5_e5785 + assign6120_body5_e5794);
                let assign6120_body5_e5796: f64 = (-assign6120_body5_e5795);
                let assign6120_body5_e5798: f64 = (assign6120_body5_e5796 / p.p481);
                let assign6120_body5_e5799: f64 = (-assign6120_body5_e5798);
                let assign6120_body5_e5801: f64 = (assign6120_body5_e5799 - 80.0);
                let assign6120_body5_e5807: f64 = (0.5 * p.p20);
                let assign6120_body5_e5808: f64 = (p.p27 + assign6120_body5_e5807);
                let assign6120_body5_e5811: f64 = (p.p29 - 1.0);
                let assign6120_body5_e5813: f64 = (assign6120_body5_e5811 - locals.var_iloop);
                let assign6120_body5_e5816: f64 = (p.p28 + p.p20);
                let assign6120_body5_e5817: f64 = (assign6120_body5_e5813 * assign6120_body5_e5816);
                let assign6120_body5_e5818: f64 = (assign6120_body5_e5808 + assign6120_body5_e5817);
                let assign6120_body5_e5819: f64 = (-assign6120_body5_e5818);
                let assign6120_body5_e5821: f64 = (assign6120_body5_e5819 / p.p481);
                let assign6120_body5_e5822: f64 = (-assign6120_body5_e5821);
                let assign6120_body5_e5824: f64 = (assign6120_body5_e5822 - 80.0);
                let assign6120_body5_e5825: f64 = (0.5 * assign6120_body5_e5824);
                let assign6120_body5_e5830: f64 = (0.5 * p.p20);
                let assign6120_body5_e5831: f64 = (p.p27 + assign6120_body5_e5830);
                let assign6120_body5_e5834: f64 = (p.p29 - 1.0);
                let assign6120_body5_e5836: f64 = (assign6120_body5_e5834 - locals.var_iloop);
                let assign6120_body5_e5839: f64 = (p.p28 + p.p20);
                let assign6120_body5_e5840: f64 = (assign6120_body5_e5836 * assign6120_body5_e5839);
                let assign6120_body5_e5841: f64 = (assign6120_body5_e5831 + assign6120_body5_e5840);
                let assign6120_body5_e5842: f64 = (-assign6120_body5_e5841);
                let assign6120_body5_e5844: f64 = (assign6120_body5_e5842 / p.p481);
                let assign6120_body5_e5845: f64 = (-assign6120_body5_e5844);
                let assign6120_body5_e5847: f64 = (assign6120_body5_e5845 - 80.0);
                let assign6120_body5_e5849: f64 = (assign6120_body5_e5847 * 0.3333333333333);
                let assign6120_body5_e5850: f64 = (1.0 + assign6120_body5_e5849);
                let assign6120_body5_e5851: f64 = (assign6120_body5_e5825 * assign6120_body5_e5850);
                let assign6120_body5_e5852: f64 = (1.0 + assign6120_body5_e5851);
                let assign6120_body5_e5853: f64 = (assign6120_body5_e5801 * assign6120_body5_e5852);
                let assign6120_body5_e5854: f64 = (1.0 + assign6120_body5_e5853);
                let assign6120_body5_e5855: f64 = (1.80485e-35 / assign6120_body5_e5854);
                (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign6120_body5_e5855, 0.0, 0.0, 0.0, 0.0, 0.0, );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
                let assign6120_body6_e5867: f64 = (1.0 - locals.var_temp1);
                let assign6120_body6_e5869: f64 = (-p.p482);
                let assign6120_body6_e5870: f64 = (assign6120_body6_e5867).powf(assign6120_body6_e5869);
                (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign6120_body6_e5870, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-locals.var_temp1_dn4))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-locals.var_temp1_dn4) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-locals.var_temp1_dn6))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-locals.var_temp1_dn6) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-locals.var_temp1_dn7))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-locals.var_temp1_dn7) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-locals.var_temp1_dn8))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-locals.var_temp1_dn8) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-locals.var_temp1_dn9))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-locals.var_temp1_dn9) / assign6120_body6_e5867))) }, );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
                let assign6120_body7_e5882: f64 = (1.0 - locals.var_temp2);
                let assign6120_body7_e5884: f64 = (-p.p482);
                let assign6120_body7_e5885: f64 = (assign6120_body7_e5882).powf(assign6120_body7_e5884);
                (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign6120_body7_e5885, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-locals.var_temp2_dn4))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-locals.var_temp2_dn4) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-locals.var_temp2_dn6))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-locals.var_temp2_dn6) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-locals.var_temp2_dn7))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-locals.var_temp2_dn7) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-locals.var_temp2_dn8))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-locals.var_temp2_dn8) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-locals.var_temp2_dn9))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-locals.var_temp2_dn9) / assign6120_body7_e5882))) }, );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
                let assign6120_body8_e5899: f64 = (locals.var_temp3 + locals.var_temp4);
                let assign6120_body8_e5900: f64 = (0.5 * assign6120_body8_e5899);
                let assign6120_body8_e5902: f64 = (assign6120_body8_e5900).powf(locals.var_temp);
                let assign6120_body8_e5903: f64 = (locals.var_tmpa + assign6120_body8_e5902);
                (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, ) = (assign6120_body8_e5903, (locals.var_tmpa_dn4 + if locals.var_temp_dn4 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_body8_e5900).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)))) } } else { (assign6120_body8_e5902 * ((locals.var_temp_dn4 * (assign6120_body8_e5900).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)) / assign6120_body8_e5900)))) }), (locals.var_tmpa_dn6 + if locals.var_temp_dn6 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_body8_e5900).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)))) } } else { (assign6120_body8_e5902 * ((locals.var_temp_dn6 * (assign6120_body8_e5900).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)) / assign6120_body8_e5900)))) }), (locals.var_tmpa_dn7 + if locals.var_temp_dn7 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_body8_e5900).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)))) } } else { (assign6120_body8_e5902 * ((locals.var_temp_dn7 * (assign6120_body8_e5900).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)) / assign6120_body8_e5900)))) }), (locals.var_tmpa_dn8 + if locals.var_temp_dn8 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_body8_e5900).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)))) } } else { (assign6120_body8_e5902 * ((locals.var_temp_dn8 * (assign6120_body8_e5900).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)) / assign6120_body8_e5900)))) }), (locals.var_tmpa_dn9 + if locals.var_temp_dn9 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_body8_e5900).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)))) } } else { (assign6120_body8_e5902 * ((locals.var_temp_dn9 * (assign6120_body8_e5900).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)) / assign6120_body8_e5900)))) }), );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
                let assign6120_body9_e5915: f64 = (locals.var_iloop + 1.0);
                locals.var_iloop = assign6120_body9_e5915;
            }
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6130_e5928: f64 = (locals.var_tmpa / p.p29);
            let assign6130_e5929: f64 = (1.0 - assign6130_e5928);
            (locals.var_str_g, locals.var_str_g_dn4, locals.var_str_g_dn6, locals.var_str_g_dn7, locals.var_str_g_dn8, locals.var_str_g_dn9, ) = (assign6130_e5929, (-(locals.var_tmpa_dn4 / p.p29)), (-(locals.var_tmpa_dn6 / p.p29)), (-(locals.var_tmpa_dn7 / p.p29)), (-(locals.var_tmpa_dn8 / p.p29)), (-(locals.var_tmpa_dn9 / p.p29)), );
        }

        let assign6140_e5935: f64 = (0.5 * p.p20);
        let assign6140_e5936: f64 = (p.p462 + assign6140_e5935);
        let assign6140_e5937: f64 = (-assign6140_e5936);
        let assign6140_e5939: f64 = (assign6140_e5937 / p.p481);
        let assign6140_e5941: f64 = (-80.0);
        let assign6140_e5942: f64 = if assign6140_e5939 > assign6140_e5941 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign6140_e5942;

        if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard137 != 0.0)) {
            let assign6150_e5955: f64 = (0.5 * p.p20);
            let assign6150_e5956: f64 = (p.p462 + assign6150_e5955);
            let assign6150_e5957: f64 = (-assign6150_e5956);
            let assign6150_e5959: f64 = (assign6150_e5957 / p.p481);
            let assign6150_e5960: f64 = (assign6150_e5959).exp();
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6150_e5960, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard137 == 0.0)) {
            let assign6160_e5978: f64 = (0.5 * p.p20);
            let assign6160_e5979: f64 = (p.p462 + assign6160_e5978);
            let assign6160_e5980: f64 = (-assign6160_e5979);
            let assign6160_e5982: f64 = (assign6160_e5980 / p.p481);
            let assign6160_e5983: f64 = (-assign6160_e5982);
            let assign6160_e5985: f64 = (assign6160_e5983 - 80.0);
            let assign6160_e5991: f64 = (0.5 * p.p20);
            let assign6160_e5992: f64 = (p.p462 + assign6160_e5991);
            let assign6160_e5993: f64 = (-assign6160_e5992);
            let assign6160_e5995: f64 = (assign6160_e5993 / p.p481);
            let assign6160_e5996: f64 = (-assign6160_e5995);
            let assign6160_e5998: f64 = (assign6160_e5996 - 80.0);
            let assign6160_e5999: f64 = (0.5 * assign6160_e5998);
            let assign6160_e6004: f64 = (0.5 * p.p20);
            let assign6160_e6005: f64 = (p.p462 + assign6160_e6004);
            let assign6160_e6006: f64 = (-assign6160_e6005);
            let assign6160_e6008: f64 = (assign6160_e6006 / p.p481);
            let assign6160_e6009: f64 = (-assign6160_e6008);
            let assign6160_e6011: f64 = (assign6160_e6009 - 80.0);
            let assign6160_e6013: f64 = (assign6160_e6011 * 0.3333333333333);
            let assign6160_e6014: f64 = (1.0 + assign6160_e6013);
            let assign6160_e6015: f64 = (assign6160_e5999 * assign6160_e6014);
            let assign6160_e6016: f64 = (1.0 + assign6160_e6015);
            let assign6160_e6017: f64 = (assign6160_e5985 * assign6160_e6016);
            let assign6160_e6018: f64 = (1.0 + assign6160_e6017);
            let assign6160_e6019: f64 = (1.80485e-35 / assign6160_e6018);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6160_e6019, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign6170_e6025: f64 = (0.5 * p.p20);
        let assign6170_e6026: f64 = (p.p463 + assign6170_e6025);
        let assign6170_e6027: f64 = (-assign6170_e6026);
        let assign6170_e6029: f64 = (assign6170_e6027 / p.p481);
        let assign6170_e6031: f64 = (-80.0);
        let assign6170_e6032: f64 = if assign6170_e6029 > assign6170_e6031 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign6170_e6032;

        if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard138 != 0.0)) {
            let assign6180_e6045: f64 = (0.5 * p.p20);
            let assign6180_e6046: f64 = (p.p463 + assign6180_e6045);
            let assign6180_e6047: f64 = (-assign6180_e6046);
            let assign6180_e6049: f64 = (assign6180_e6047 / p.p481);
            let assign6180_e6050: f64 = (assign6180_e6049).exp();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign6180_e6050, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard138 == 0.0)) {
            let assign6190_e6068: f64 = (0.5 * p.p20);
            let assign6190_e6069: f64 = (p.p463 + assign6190_e6068);
            let assign6190_e6070: f64 = (-assign6190_e6069);
            let assign6190_e6072: f64 = (assign6190_e6070 / p.p481);
            let assign6190_e6073: f64 = (-assign6190_e6072);
            let assign6190_e6075: f64 = (assign6190_e6073 - 80.0);
            let assign6190_e6081: f64 = (0.5 * p.p20);
            let assign6190_e6082: f64 = (p.p463 + assign6190_e6081);
            let assign6190_e6083: f64 = (-assign6190_e6082);
            let assign6190_e6085: f64 = (assign6190_e6083 / p.p481);
            let assign6190_e6086: f64 = (-assign6190_e6085);
            let assign6190_e6088: f64 = (assign6190_e6086 - 80.0);
            let assign6190_e6089: f64 = (0.5 * assign6190_e6088);
            let assign6190_e6094: f64 = (0.5 * p.p20);
            let assign6190_e6095: f64 = (p.p463 + assign6190_e6094);
            let assign6190_e6096: f64 = (-assign6190_e6095);
            let assign6190_e6098: f64 = (assign6190_e6096 / p.p481);
            let assign6190_e6099: f64 = (-assign6190_e6098);
            let assign6190_e6101: f64 = (assign6190_e6099 - 80.0);
            let assign6190_e6103: f64 = (assign6190_e6101 * 0.3333333333333);
            let assign6190_e6104: f64 = (1.0 + assign6190_e6103);
            let assign6190_e6105: f64 = (assign6190_e6089 * assign6190_e6104);
            let assign6190_e6106: f64 = (1.0 + assign6190_e6105);
            let assign6190_e6107: f64 = (assign6190_e6075 * assign6190_e6106);
            let assign6190_e6108: f64 = (1.0 + assign6190_e6107);
            let assign6190_e6109: f64 = (1.80485e-35 / assign6190_e6108);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign6190_e6109, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6200_e6121: f64 = (1.0 - locals.var_temp1);
            let assign6200_e6123: f64 = (-p.p482);
            let assign6200_e6124: f64 = (assign6200_e6121).powf(assign6200_e6123);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign6200_e6124, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-locals.var_temp1_dn4))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-locals.var_temp1_dn4) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-locals.var_temp1_dn6))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-locals.var_temp1_dn6) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-locals.var_temp1_dn7))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-locals.var_temp1_dn7) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-locals.var_temp1_dn8))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-locals.var_temp1_dn8) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-locals.var_temp1_dn9))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-locals.var_temp1_dn9) / assign6200_e6121))) }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6210_e6136: f64 = (1.0 - locals.var_temp2);
            let assign6210_e6138: f64 = (-p.p482);
            let assign6210_e6139: f64 = (assign6210_e6136).powf(assign6210_e6138);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign6210_e6139, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-locals.var_temp2_dn4))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-locals.var_temp2_dn4) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-locals.var_temp2_dn6))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-locals.var_temp2_dn6) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-locals.var_temp2_dn7))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-locals.var_temp2_dn7) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-locals.var_temp2_dn8))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-locals.var_temp2_dn8) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-locals.var_temp2_dn9))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-locals.var_temp2_dn9) / assign6210_e6136))) }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6220_e6153: f64 = (locals.var_temp3 + locals.var_temp4);
            let assign6220_e6154: f64 = (0.5 * assign6220_e6153);
            let assign6220_e6156: f64 = (assign6220_e6154).powf(locals.var_temp);
            let assign6220_e6157: f64 = (1.0 - assign6220_e6156);
            (locals.var_str_gref, locals.var_str_gref_dn4, locals.var_str_gref_dn6, locals.var_str_gref_dn7, locals.var_str_gref_dn8, locals.var_str_gref_dn9, ) = (assign6220_e6157, (-if locals.var_temp_dn4 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6220_e6154).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)))) } } else { (assign6220_e6156 * ((locals.var_temp_dn4 * (assign6220_e6154).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)) / assign6220_e6154)))) }), (-if locals.var_temp_dn6 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6220_e6154).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)))) } } else { (assign6220_e6156 * ((locals.var_temp_dn6 * (assign6220_e6154).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)) / assign6220_e6154)))) }), (-if locals.var_temp_dn7 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6220_e6154).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)))) } } else { (assign6220_e6156 * ((locals.var_temp_dn7 * (assign6220_e6154).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)) / assign6220_e6154)))) }), (-if locals.var_temp_dn8 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6220_e6154).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)))) } } else { (assign6220_e6156 * ((locals.var_temp_dn8 * (assign6220_e6154).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)) / assign6220_e6154)))) }), (-if locals.var_temp_dn9 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6220_e6154).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)))) } } else { (assign6220_e6156 * ((locals.var_temp_dn9 * (assign6220_e6154).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)) / assign6220_e6154)))) }), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6230_e6169: f64 = (locals.var_w_i + locals.var_delwod);
            let assign6230_e6171: f64 = (assign6230_e6169 + p.p464);
            let assign6230_e6173: f64 = (assign6230_e6171).max(1e-9);
            locals.var_wx = assign6230_e6173;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6240_e6188: f64 = (locals.var_rt - 1.0);
            let assign6240_e6189: f64 = (p.p487 * assign6240_e6188);
            let assign6240_e6190: f64 = (1.0 + assign6240_e6189);
            let assign6240_e6191: f64 = (p.p486 / assign6240_e6190);
            (locals.var_ruo, locals.var_ruo_dn4, locals.var_ruo_dn6, locals.var_ruo_dn7, locals.var_ruo_dn8, locals.var_ruo_dn9, ) = (assign6240_e6191, (-((p.p486 * (p.p487 * locals.var_rt_dn4)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * locals.var_rt_dn6)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * locals.var_rt_dn7)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * locals.var_rt_dn8)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * locals.var_rt_dn9)) / (assign6240_e6190 * assign6240_e6190))), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6250_e6203: f64 = (locals.var_ruo * locals.var_str_g);
            (locals.var_rhobeta, locals.var_rhobeta_dn4, locals.var_rhobeta_dn6, locals.var_rhobeta_dn7, locals.var_rhobeta_dn8, locals.var_rhobeta_dn9, ) = (assign6250_e6203, ((locals.var_ruo_dn4 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn4)), ((locals.var_ruo_dn6 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn6)), ((locals.var_ruo_dn7 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn7)), ((locals.var_ruo_dn8 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn8)), ((locals.var_ruo_dn9 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6260_e6215: f64 = (locals.var_ruo * locals.var_str_gref);
            (locals.var_rhobetaref, locals.var_rhobetaref_dn4, locals.var_rhobetaref_dn6, locals.var_rhobetaref_dn7, locals.var_rhobetaref_dn8, locals.var_rhobetaref_dn9, ) = (assign6260_e6215, ((locals.var_ruo_dn4 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn4)), ((locals.var_ruo_dn6 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn6)), ((locals.var_ruo_dn7 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn7)), ((locals.var_ruo_dn8 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn8)), ((locals.var_ruo_dn9 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6270_e6227: f64 = (locals.var_str_g - locals.var_str_gref);
            (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9, ) = (assign6270_e6227, (locals.var_str_g_dn4 - locals.var_str_gref_dn4), (locals.var_str_g_dn6 - locals.var_str_gref_dn6), (locals.var_str_g_dn7 - locals.var_str_gref_dn7), (locals.var_str_g_dn8 - locals.var_str_gref_dn8), (locals.var_str_g_dn9 - locals.var_str_gref_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6280_e6240: f64 = (p.p484 * locals.var_wx);
            let assign6280_e6242: f64 = (assign6280_e6240 / locals.var_wen);
            let assign6280_e6243: f64 = (1.0 + assign6280_e6242);
            let assign6280_e6245: f64 = (assign6280_e6243).max(1e-20);
            locals.var_kstressvth0 = assign6280_e6245;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6290_e6258: f64 = (1.0 + locals.var_rhobeta);
            let assign6290_e6259: f64 = (locals.var_betn_p * assign6290_e6258);
            let assign6290_e6262: f64 = (1.0 + locals.var_rhobetaref);
            let assign6290_e6263: f64 = (assign6290_e6259 / assign6290_e6262);
            (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9, ) = (assign6290_e6263, (((((locals.var_betn_p_dn4 * assign6290_e6258) + (locals.var_betn_p * locals.var_rhobeta_dn4)) * assign6290_e6262) - (assign6290_e6259 * locals.var_rhobetaref_dn4)) / (assign6290_e6262 * assign6290_e6262)), (((((locals.var_betn_p_dn6 * assign6290_e6258) + (locals.var_betn_p * locals.var_rhobeta_dn6)) * assign6290_e6262) - (assign6290_e6259 * locals.var_rhobetaref_dn6)) / (assign6290_e6262 * assign6290_e6262)), (((((locals.var_betn_p_dn7 * assign6290_e6258) + (locals.var_betn_p * locals.var_rhobeta_dn7)) * assign6290_e6262) - (assign6290_e6259 * locals.var_rhobetaref_dn7)) / (assign6290_e6262 * assign6290_e6262)), (((((locals.var_betn_p_dn8 * assign6290_e6258) + (locals.var_betn_p * locals.var_rhobeta_dn8)) * assign6290_e6262) - (assign6290_e6259 * locals.var_rhobetaref_dn8)) / (assign6290_e6262 * assign6290_e6262)), (((((locals.var_betn_p_dn9 * assign6290_e6258) + (locals.var_betn_p * locals.var_rhobeta_dn9)) * assign6290_e6262) - (assign6290_e6259 * locals.var_rhobetaref_dn9)) / (assign6290_e6262 * assign6290_e6262)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6300_e6275: f64 = (locals.var_betn_p).max(1e-10);
            (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9, ) = (assign6300_e6275, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6310_e6287: f64 = (p.p254 * locals.var_betn1_t);
            (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9, ) = (assign6310_e6287, (p.p254 * locals.var_betn1_t_dn4), (p.p254 * locals.var_betn1_t_dn6), (p.p254 * locals.var_betn1_t_dn7), (p.p254 * locals.var_betn1_t_dn8), (p.p254 * locals.var_betn1_t_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6320_e6299: f64 = (1.0 + locals.var_rhobeta);
            let assign6320_e6303: f64 = (p.p488 * locals.var_rhobetaref);
            let assign6320_e6304: f64 = (1.0 + assign6320_e6303);
            let assign6320_e6305: f64 = (assign6320_e6299 * assign6320_e6304);
            let assign6320_e6308: f64 = (1.0 + locals.var_rhobetaref);
            let assign6320_e6312: f64 = (p.p488 * locals.var_rhobeta);
            let assign6320_e6313: f64 = (1.0 + assign6320_e6312);
            let assign6320_e6314: f64 = (assign6320_e6308 * assign6320_e6313);
            let assign6320_e6315: f64 = (assign6320_e6305 / assign6320_e6314);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6320_e6315, (((((locals.var_rhobeta_dn4 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * locals.var_rhobetaref_dn4))) * assign6320_e6314) - (assign6320_e6305 * ((locals.var_rhobetaref_dn4 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * locals.var_rhobeta_dn4))))) / (assign6320_e6314 * assign6320_e6314)), (((((locals.var_rhobeta_dn6 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * locals.var_rhobetaref_dn6))) * assign6320_e6314) - (assign6320_e6305 * ((locals.var_rhobetaref_dn6 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * locals.var_rhobeta_dn6))))) / (assign6320_e6314 * assign6320_e6314)), (((((locals.var_rhobeta_dn7 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * locals.var_rhobetaref_dn7))) * assign6320_e6314) - (assign6320_e6305 * ((locals.var_rhobetaref_dn7 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * locals.var_rhobeta_dn7))))) / (assign6320_e6314 * assign6320_e6314)), (((((locals.var_rhobeta_dn8 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * locals.var_rhobetaref_dn8))) * assign6320_e6314) - (assign6320_e6305 * ((locals.var_rhobetaref_dn8 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * locals.var_rhobeta_dn8))))) / (assign6320_e6314 * assign6320_e6314)), (((((locals.var_rhobeta_dn9 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * locals.var_rhobetaref_dn9))) * assign6320_e6314) - (assign6320_e6305 * ((locals.var_rhobetaref_dn9 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * locals.var_rhobeta_dn9))))) / (assign6320_e6314 * assign6320_e6314)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6330_e6327: f64 = (locals.var_thesat_p * locals.var_temp);
            (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9, ) = (assign6330_e6327, ((locals.var_thesat_p_dn4 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn4)), ((locals.var_thesat_p_dn6 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn6)), ((locals.var_thesat_p_dn7 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn7)), ((locals.var_thesat_p_dn8 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn8)), ((locals.var_thesat_p_dn9 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6340_e6339: f64 = (locals.var_thesat_p).max(0.0);
            (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, ) = (assign6340_e6339, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6350_e6351: f64 = (locals.var_thesatac_p * locals.var_temp);
            (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9, ) = (assign6350_e6351, ((locals.var_thesatac_p_dn4 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn4)), ((locals.var_thesatac_p_dn6 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn6)), ((locals.var_thesatac_p_dn7 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn7)), ((locals.var_thesatac_p_dn8 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn8)), ((locals.var_thesatac_p_dn9 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6360_e6363: f64 = (locals.var_thesatac_p).max(0.0);
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (assign6360_e6363, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6370_e6375: f64 = (p.p483 * locals.var_temp0__blk79);
            let assign6370_e6377: f64 = (assign6370_e6375 / locals.var_kstressvth0);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6370_e6377, ((p.p483 * locals.var_temp0__blk79_dn4) / locals.var_kstressvth0), ((p.p483 * locals.var_temp0__blk79_dn6) / locals.var_kstressvth0), ((p.p483 * locals.var_temp0__blk79_dn7) / locals.var_kstressvth0), ((p.p483 * locals.var_temp0__blk79_dn8) / locals.var_kstressvth0), ((p.p483 * locals.var_temp0__blk79_dn9) / locals.var_kstressvth0), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6380_e6389: f64 = (locals.var_vfb1_t + locals.var_temp);
            (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, ) = (assign6380_e6389, (locals.var_vfb1_t_dn4 + locals.var_temp_dn4), (locals.var_vfb1_t_dn6 + locals.var_temp_dn6), (locals.var_vfb1_t_dn7 + locals.var_temp_dn7), (locals.var_vfb1_t_dn8 + locals.var_temp_dn8), (locals.var_vfb1_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6390_e6401: f64 = (locals.var_vfb2_t + locals.var_temp);
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (assign6390_e6401, (locals.var_vfb2_t_dn4 + locals.var_temp_dn4), (locals.var_vfb2_t_dn6 + locals.var_temp_dn6), (locals.var_vfb2_t_dn7 + locals.var_temp_dn7), (locals.var_vfb2_t_dn8 + locals.var_temp_dn8), (locals.var_vfb2_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6400_e6413: f64 = (locals.var_vfbac1_t + locals.var_temp);
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (assign6400_e6413, (locals.var_vfbac1_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac1_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac1_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac1_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac1_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6410_e6425: f64 = (locals.var_vfbac2_t + locals.var_temp);
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (assign6410_e6425, (locals.var_vfbac2_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac2_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac2_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac2_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac2_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6420_e6437: f64 = (p.p485 * locals.var_temp0__blk79);
            let assign6420_e6440: f64 = (locals.var_lambda_le).powf(p.p236);
            let assign6420_e6441: f64 = (assign6420_e6437 * assign6420_e6440);
            let assign6420_e6445: f64 = (p.p237 * locals.var_iwe);
            let assign6420_e6446: f64 = (1.0 + assign6420_e6445);
            let assign6420_e6447: f64 = (assign6420_e6441 * assign6420_e6446);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6420_e6447, (((p.p485 * locals.var_temp0__blk79_dn4) * assign6420_e6440) * assign6420_e6446), (((p.p485 * locals.var_temp0__blk79_dn6) * assign6420_e6440) * assign6420_e6446), (((p.p485 * locals.var_temp0__blk79_dn7) * assign6420_e6440) * assign6420_e6446), (((p.p485 * locals.var_temp0__blk79_dn8) * assign6420_e6440) * assign6420_e6446), (((p.p485 * locals.var_temp0__blk79_dn9) * assign6420_e6440) * assign6420_e6446), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6430_e6459: f64 = (locals.var_cf_p + locals.var_temp);
            (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9, ) = (assign6430_e6459, (locals.var_cf_p_dn4 + locals.var_temp_dn4), (locals.var_cf_p_dn6 + locals.var_temp_dn6), (locals.var_cf_p_dn7 + locals.var_temp_dn7), (locals.var_cf_p_dn8 + locals.var_temp_dn8), (locals.var_cf_p_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6440_e6471: f64 = (locals.var_cf_p).max(0.0);
            (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, ) = (assign6440_e6471, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6450_e6483: f64 = (locals.var_cfac_p + locals.var_temp);
            (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9, ) = (assign6450_e6483, (locals.var_cfac_p_dn4 + locals.var_temp_dn4), (locals.var_cfac_p_dn6 + locals.var_temp_dn6), (locals.var_cfac_p_dn7 + locals.var_temp_dn7), (locals.var_cfac_p_dn8 + locals.var_temp_dn8), (locals.var_cfac_p_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6460_e6495: f64 = (locals.var_cfac_p).max(0.0);
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (assign6460_e6495, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6470_e6507: f64 = (p.p238 * locals.var_tox2_i);
            let assign6470_e6509: f64 = (assign6470_e6507 / locals.var_tox1_i);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6470_e6509, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6480_e6521: f64 = (locals.var_cf1_t * locals.var_temp);
            (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, ) = (assign6480_e6521, ((locals.var_cf1_t_dn4 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn4)), ((locals.var_cf1_t_dn6 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn6)), ((locals.var_cf1_t_dn7 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn7)), ((locals.var_cf1_t_dn8 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn8)), ((locals.var_cf1_t_dn9 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6490_e6533: f64 = (locals.var_cfac1_t * locals.var_temp);
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (assign6490_e6533, ((locals.var_cfac1_t_dn4 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn4)), ((locals.var_cfac1_t_dn6 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn6)), ((locals.var_cfac1_t_dn7 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn7)), ((locals.var_cfac1_t_dn8 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn8)), ((locals.var_cfac1_t_dn9 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn9)), );
        }

        let assign6500_e6538: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign6500_e6538;

        if (locals.var_guard139 != 0.0) {
            locals.var_novd_i = locals.var_nov_i;
            locals.var_igovinvd_t = locals.var_igovinv_t;
            locals.var_fnovinvd_t = locals.var_fnovinv_t;
            locals.var_igovaccd_t = locals.var_igovacc_t;
            (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9, ) = (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9, );
            locals.var_bgidld_t = locals.var_bgidl_t;
            locals.var_stbgidld_i = locals.var_stbgidl_i;
            locals.var_cgidld_i = locals.var_cgidl_i;
            locals.var_dgidld_i = locals.var_dgidl_i;
            (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9, ) = (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9, );
            (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9, ) = (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9, );
        }

        let assign6620_e6585: f64 = (1.0 - locals.var_xge_i);
        locals.var_one_m_xge = assign6620_e6585;

        let assign6630_e6588: f64 = (1.04479e-10 * locals.var_one_m_xge);
        let assign6630_e6591: f64 = (1.43438e-10 * locals.var_xge_i);
        let assign6630_e6592: f64 = (assign6630_e6588 + assign6630_e6591);
        locals.var_epsch = assign6630_e6592;

        let assign6640_e6596: f64 = (0.000473 * locals.var_tkc_sq);
        let assign6640_e6599: f64 = (636.0 + locals.var_tkc);
        let assign6640_e6600: f64 = (assign6640_e6596 / assign6640_e6599);
        let assign6640_e6601: f64 = (1.17 - assign6640_e6600);
        (locals.var_egsi, locals.var_egsi_dn4, locals.var_egsi_dn6, locals.var_egsi_dn7, locals.var_egsi_dn8, locals.var_egsi_dn9, ) = (assign6640_e6601, (-((((0.000473 * locals.var_tkc_sq_dn4) * assign6640_e6599) - (assign6640_e6596 * locals.var_tkc_dn4)) / (assign6640_e6599 * assign6640_e6599))), (-((((0.000473 * locals.var_tkc_sq_dn6) * assign6640_e6599) - (assign6640_e6596 * locals.var_tkc_dn6)) / (assign6640_e6599 * assign6640_e6599))), (-((((0.000473 * locals.var_tkc_sq_dn7) * assign6640_e6599) - (assign6640_e6596 * locals.var_tkc_dn7)) / (assign6640_e6599 * assign6640_e6599))), (-((((0.000473 * locals.var_tkc_sq_dn8) * assign6640_e6599) - (assign6640_e6596 * locals.var_tkc_dn8)) / (assign6640_e6599 * assign6640_e6599))), (-((((0.000473 * locals.var_tkc_sq_dn9) * assign6640_e6599) - (assign6640_e6596 * locals.var_tkc_dn9)) / (assign6640_e6599 * assign6640_e6599))), );

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6650_e6605: f64 = (0.0004774 * locals.var_tkc_sq);
        let assign6650_e6608: f64 = (235.0 + locals.var_tkc);
        let assign6650_e6609: f64 = (assign6650_e6605 / assign6650_e6608);
        let assign6650_e6610: f64 = (0.744 - assign6650_e6609);
        (locals.var_egge, locals.var_egge_dn4, locals.var_egge_dn6, locals.var_egge_dn7, locals.var_egge_dn8, locals.var_egge_dn9, ) = (assign6650_e6610, (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign6650_e6608) - (assign6650_e6605 * locals.var_tkc_dn4)) / (assign6650_e6608 * assign6650_e6608))), (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign6650_e6608) - (assign6650_e6605 * locals.var_tkc_dn6)) / (assign6650_e6608 * assign6650_e6608))), (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign6650_e6608) - (assign6650_e6605 * locals.var_tkc_dn7)) / (assign6650_e6608 * assign6650_e6608))), (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign6650_e6608) - (assign6650_e6605 * locals.var_tkc_dn8)) / (assign6650_e6608 * assign6650_e6608))), (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign6650_e6608) - (assign6650_e6605 * locals.var_tkc_dn9)) / (assign6650_e6608 * assign6650_e6608))), );

        let assign6660_e6613: f64 = (locals.var_egge - locals.var_egsi);
        let assign6660_e6615: f64 = (-0.4);
        let assign6660_e6617: f64 = (assign6660_e6615 * locals.var_one_m_xge);
        let assign6660_e6618: f64 = (assign6660_e6613 + assign6660_e6617);
        let assign6660_e6620: f64 = (assign6660_e6618 * locals.var_xge_i);
        (locals.var_deg, locals.var_deg_dn4, locals.var_deg_dn6, locals.var_deg_dn7, locals.var_deg_dn8, locals.var_deg_dn9, ) = (assign6660_e6620, ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i), ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i), ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i), ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i), ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i), );

        let assign6670_e6623: f64 = (locals.var_egsi + locals.var_deg);
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, ) = (assign6670_e6623, (locals.var_egsi_dn4 + locals.var_deg_dn4), (locals.var_egsi_dn6 + locals.var_deg_dn6), (locals.var_egsi_dn7 + locals.var_deg_dn7), (locals.var_egsi_dn8 + locals.var_deg_dn8), (locals.var_egsi_dn9 + locals.var_deg_dn9), );

        let assign6680_e6626: f64 = (0.5 * locals.var_eg);
        let assign6680_e6628: f64 = (assign6680_e6626 * locals.var_inv_phit0);
        (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9, ) = (assign6680_e6628, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign6680_e6626 * locals.var_inv_phit0_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign6680_e6626 * locals.var_inv_phit0_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign6680_e6626 * locals.var_inv_phit0_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign6680_e6626 * locals.var_inv_phit0_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign6680_e6626 * locals.var_inv_phit0_dn9)), );

        (locals.var_eg_2phit0_woshe, locals.var_eg_2phit0_woshe_dn4, locals.var_eg_2phit0_woshe_dn6, locals.var_eg_2phit0_woshe_dn7, locals.var_eg_2phit0_woshe_dn8, locals.var_eg_2phit0_woshe_dn9, ) = (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9, );

        let assign6700_e6634: f64 = (10.0 * locals.var_xge_i);
        let assign6700_e6635: f64 = (assign6700_e6634).sqrt();
        let assign6700_e6636: f64 = (1.0 + assign6700_e6635);
        let assign6700_e6637: f64 = (1.0 / assign6700_e6636);
        locals.var_niratio = assign6700_e6637;

        let assign6710_e6640: f64 = (0.05 * locals.var_xge_i);
        let assign6710_e6643: f64 = (0.5 * locals.var_deg);
        let assign6710_e6644: f64 = (assign6710_e6640 - assign6710_e6643);
        (locals.var_dvfbch, locals.var_dvfbch_dn4, locals.var_dvfbch_dn6, locals.var_dvfbch_dn7, locals.var_dvfbch_dn8, locals.var_dvfbch_dn9, ) = (assign6710_e6644, (-(0.5 * locals.var_deg_dn4)), (-(0.5 * locals.var_deg_dn6)), (-(0.5 * locals.var_deg_dn7)), (-(0.5 * locals.var_deg_dn8)), (-(0.5 * locals.var_deg_dn9)), );

        let assign6720_e6647: f64 = (1.602176565e-19 * locals.var_nch_i);
        let assign6720_e6649: f64 = (assign6720_e6647 * 0.5);
        let assign6720_e6651: f64 = (assign6720_e6649 * locals.var_tsi_i);
        let assign6720_e6653: f64 = (assign6720_e6651 / 3.45313e-11);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6720_e6653, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign6730_e6656: f64 = if locals.var_typech_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign6730_e6656;

        if (locals.var_guard140 != 0.0) {
            let assign6740_e6662: f64 = (p.p13 * 4e-10);
            let assign6740_e6663: f64 = (locals.var_tox1_i + assign6740_e6662);
            let assign6740_e6664: f64 = (locals.var_temp * assign6740_e6663);
            (locals.var_dvfb1nch, locals.var_dvfb1nch_dn4, locals.var_dvfb1nch_dn6, locals.var_dvfb1nch_dn7, locals.var_dvfb1nch_dn8, locals.var_dvfb1nch_dn9, ) = (assign6740_e6664, (locals.var_temp_dn4 * assign6740_e6663), (locals.var_temp_dn6 * assign6740_e6663), (locals.var_temp_dn7 * assign6740_e6663), (locals.var_temp_dn8 * assign6740_e6663), (locals.var_temp_dn9 * assign6740_e6663), );
        }

        if (locals.var_guard140 != 0.0) {
            let assign6750_e6672: f64 = (p.p13 * 4e-10);
            let assign6750_e6673: f64 = (locals.var_tox2_i + assign6750_e6672);
            let assign6750_e6674: f64 = (locals.var_temp * assign6750_e6673);
            (locals.var_dvfb2nch, locals.var_dvfb2nch_dn4, locals.var_dvfb2nch_dn6, locals.var_dvfb2nch_dn7, locals.var_dvfb2nch_dn8, locals.var_dvfb2nch_dn9, ) = (assign6750_e6674, (locals.var_temp_dn4 * assign6750_e6673), (locals.var_temp_dn6 * assign6750_e6673), (locals.var_temp_dn7 * assign6750_e6673), (locals.var_temp_dn8 * assign6750_e6673), (locals.var_temp_dn9 * assign6750_e6673), );
        }

        if (locals.var_guard140 == 0.0) {
            let assign6760_e6680: f64 = (-locals.var_temp);
            let assign6760_e6684: f64 = (p.p13 * 4e-10);
            let assign6760_e6685: f64 = (locals.var_tox1_i + assign6760_e6684);
            let assign6760_e6686: f64 = (assign6760_e6680 * assign6760_e6685);
            (locals.var_dvfb1nch, locals.var_dvfb1nch_dn4, locals.var_dvfb1nch_dn6, locals.var_dvfb1nch_dn7, locals.var_dvfb1nch_dn8, locals.var_dvfb1nch_dn9, ) = (assign6760_e6686, ((-locals.var_temp_dn4) * assign6760_e6685), ((-locals.var_temp_dn6) * assign6760_e6685), ((-locals.var_temp_dn7) * assign6760_e6685), ((-locals.var_temp_dn8) * assign6760_e6685), ((-locals.var_temp_dn9) * assign6760_e6685), );
        }

        if (locals.var_guard140 == 0.0) {
            let assign6770_e6692: f64 = (-locals.var_temp);
            let assign6770_e6696: f64 = (p.p13 * 4e-10);
            let assign6770_e6697: f64 = (locals.var_tox2_i + assign6770_e6696);
            let assign6770_e6698: f64 = (assign6770_e6692 * assign6770_e6697);
            (locals.var_dvfb2nch, locals.var_dvfb2nch_dn4, locals.var_dvfb2nch_dn6, locals.var_dvfb2nch_dn7, locals.var_dvfb2nch_dn8, locals.var_dvfb2nch_dn9, ) = (assign6770_e6698, ((-locals.var_temp_dn4) * assign6770_e6697), ((-locals.var_temp_dn6) * assign6770_e6697), ((-locals.var_temp_dn7) * assign6770_e6697), ((-locals.var_temp_dn8) * assign6770_e6697), ((-locals.var_temp_dn9) * assign6770_e6697), );
        }

        let assign6780_e6703: f64 = (locals.var_tkc * 0.0033333333333);
        let assign6780_e6704: f64 = (assign6780_e6703).sqrt();
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6780_e6704, ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign6780_e6704)), ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign6780_e6704)), ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign6780_e6704)), ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign6780_e6704)), ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign6780_e6704)), );

        let assign6790_e6707: f64 = (4.05e25 * locals.var_temp);
        let assign6790_e6709: f64 = (assign6790_e6707 * locals.var_temp);
        let assign6790_e6711: f64 = (assign6790_e6709 * locals.var_temp);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6790_e6711, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign6790_e6707 * locals.var_temp_dn4)) * locals.var_temp) + (assign6790_e6709 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign6790_e6707 * locals.var_temp_dn6)) * locals.var_temp) + (assign6790_e6709 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign6790_e6707 * locals.var_temp_dn7)) * locals.var_temp) + (assign6790_e6709 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign6790_e6707 * locals.var_temp_dn8)) * locals.var_temp) + (assign6790_e6709 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign6790_e6707 * locals.var_temp_dn9)) * locals.var_temp) + (assign6790_e6709 * locals.var_temp_dn9)), );

        let assign6800_e6714: f64 = (locals.var_temp1 * locals.var_niratio);
        (locals.var_neff, locals.var_neff_dn4, locals.var_neff_dn6, locals.var_neff_dn7, locals.var_neff_dn8, locals.var_neff_dn9, ) = (assign6800_e6714, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio), );

        let assign6810_e6718: f64 = (0.5 * locals.var_deg);
        let assign6810_e6720: f64 = (assign6810_e6718 * locals.var_inv_phit0);
        let assign6810_e6721: f64 = (assign6810_e6720).exp();
        let assign6810_e6722: f64 = (locals.var_temp1 * assign6810_e6721);
        (locals.var_neff_poly, locals.var_neff_poly_dn4, locals.var_neff_poly_dn6, locals.var_neff_poly_dn7, locals.var_neff_poly_dn8, locals.var_neff_poly_dn9, ) = (assign6810_e6722, ((locals.var_temp1_dn4 * assign6810_e6721) + (locals.var_temp1 * (assign6810_e6721 * (((0.5 * locals.var_deg_dn4) * locals.var_inv_phit0) + (assign6810_e6718 * locals.var_inv_phit0_dn4))))), ((locals.var_temp1_dn6 * assign6810_e6721) + (locals.var_temp1 * (assign6810_e6721 * (((0.5 * locals.var_deg_dn6) * locals.var_inv_phit0) + (assign6810_e6718 * locals.var_inv_phit0_dn6))))), ((locals.var_temp1_dn7 * assign6810_e6721) + (locals.var_temp1 * (assign6810_e6721 * (((0.5 * locals.var_deg_dn7) * locals.var_inv_phit0) + (assign6810_e6718 * locals.var_inv_phit0_dn7))))), ((locals.var_temp1_dn8 * assign6810_e6721) + (locals.var_temp1 * (assign6810_e6721 * (((0.5 * locals.var_deg_dn8) * locals.var_inv_phit0) + (assign6810_e6718 * locals.var_inv_phit0_dn8))))), ((locals.var_temp1_dn9 * assign6810_e6721) + (locals.var_temp1 * (assign6810_e6721 * (((0.5 * locals.var_deg_dn9) * locals.var_inv_phit0) + (assign6810_e6718 * locals.var_inv_phit0_dn9))))), );

        let assign6820_e6726: f64 = (0.5 * locals.var_deg);
        let assign6820_e6728: f64 = (assign6820_e6726 * locals.var_inv_phit0);
        let assign6820_e6729: f64 = (assign6820_e6728).exp();
        let assign6820_e6730: f64 = (locals.var_temp1 * assign6820_e6729);
        (locals.var_neff_sub, locals.var_neff_sub_dn4, locals.var_neff_sub_dn6, locals.var_neff_sub_dn7, locals.var_neff_sub_dn8, locals.var_neff_sub_dn9, ) = (assign6820_e6730, ((locals.var_temp1_dn4 * assign6820_e6729) + (locals.var_temp1 * (assign6820_e6729 * (((0.5 * locals.var_deg_dn4) * locals.var_inv_phit0) + (assign6820_e6726 * locals.var_inv_phit0_dn4))))), ((locals.var_temp1_dn6 * assign6820_e6729) + (locals.var_temp1 * (assign6820_e6729 * (((0.5 * locals.var_deg_dn6) * locals.var_inv_phit0) + (assign6820_e6726 * locals.var_inv_phit0_dn6))))), ((locals.var_temp1_dn7 * assign6820_e6729) + (locals.var_temp1 * (assign6820_e6729 * (((0.5 * locals.var_deg_dn7) * locals.var_inv_phit0) + (assign6820_e6726 * locals.var_inv_phit0_dn7))))), ((locals.var_temp1_dn8 * assign6820_e6729) + (locals.var_temp1 * (assign6820_e6729 * (((0.5 * locals.var_deg_dn8) * locals.var_inv_phit0) + (assign6820_e6726 * locals.var_inv_phit0_dn8))))), ((locals.var_temp1_dn9 * assign6820_e6729) + (locals.var_temp1 * (assign6820_e6729 * (((0.5 * locals.var_deg_dn9) * locals.var_inv_phit0) + (assign6820_e6726 * locals.var_inv_phit0_dn9))))), );

        let assign6830_e6733: f64 = (3.45313e-11 / locals.var_tox1_i);
        locals.var_cox1init = assign6830_e6733;

        let assign6840_e6736: f64 = (3.45313e-11 / locals.var_tox2_i);
        locals.var_cox2init = assign6840_e6736;

        let assign6850_e6739: f64 = if locals.var_pnce_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign6850_e6739;

        if (locals.var_guard141 != 0.0) {
            let assign6860_e6744: f64 = (1.0 + locals.var_pnce_i);
            let assign6860_e6745: f64 = (locals.var_cox1init * assign6860_e6744);
            locals.var_cox1prime = assign6860_e6745;
        }

        if (locals.var_guard141 != 0.0) {
            locals.var_cox2prime = locals.var_cox2init;
        }

        if (locals.var_guard141 == 0.0) {
            locals.var_cox1prime = locals.var_cox1init;
        }

        if (locals.var_guard141 == 0.0) {
            let assign6890_e6762: f64 = (1.0 - locals.var_pnce_i);
            let assign6890_e6763: f64 = (locals.var_cox2init * assign6890_e6762);
            locals.var_cox2prime = assign6890_e6763;
        }

        let assign6900_e6768: f64 = (locals.var_epsch / locals.var_tsi_i);
        locals.var_csiprime_0 = assign6900_e6768;

        let assign6910_e6773: f64 = (locals.var_ct_i * locals.var_rtn);
        let assign6910_e6774: f64 = (1.0 + assign6910_e6773);
        let assign6910_e6775: f64 = (locals.var_phit0 * assign6910_e6774);
        (locals.var_phit, locals.var_phit_dn4, locals.var_phit_dn6, locals.var_phit_dn7, locals.var_phit_dn8, locals.var_phit_dn9, ) = (assign6910_e6775, ((locals.var_phit0_dn4 * assign6910_e6774) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign6910_e6774) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign6910_e6774) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign6910_e6774) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign6910_e6774) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9))), );

        let assign6920_e6778: f64 = (1.0 / locals.var_phit);
        (locals.var_inv_phit, locals.var_inv_phit_dn4, locals.var_inv_phit_dn6, locals.var_inv_phit_dn7, locals.var_inv_phit_dn8, locals.var_inv_phit_dn9, ) = (assign6920_e6778, (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit))), );

        let assign6930_e6781: f64 = (0.5 * locals.var_eg);
        let assign6930_e6783: f64 = (assign6930_e6781 * locals.var_inv_phit);
        (locals.var_eg_2phit, locals.var_eg_2phit_dn4, locals.var_eg_2phit_dn6, locals.var_eg_2phit_dn7, locals.var_eg_2phit_dn8, locals.var_eg_2phit_dn9, ) = (assign6930_e6783, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign6930_e6781 * locals.var_inv_phit_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign6930_e6781 * locals.var_inv_phit_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign6930_e6781 * locals.var_inv_phit_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign6930_e6781 * locals.var_inv_phit_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign6930_e6781 * locals.var_inv_phit_dn9)), );

        let assign6940_e6786: f64 = (locals.var_cox1prime / locals.var_csiprime_0);
        locals.var_k1_1d = assign6940_e6786;

        let assign6950_e6789: f64 = (locals.var_cox2prime / locals.var_csiprime_0);
        locals.var_k2_1d = assign6950_e6789;

        let assign6960_e6794: f64 = (1.0 / locals.var_k1_1d);
        let assign6960_e6795: f64 = (1.0 + assign6960_e6794);
        let assign6960_e6798: f64 = (1.0 / locals.var_k2_1d);
        let assign6960_e6799: f64 = (assign6960_e6795 + assign6960_e6798);
        let assign6960_e6800: f64 = (1.0 / assign6960_e6799);
        locals.var_keq_1d = assign6960_e6800;

        let assign6970_e6803: f64 = (2.0 * 1.602176565e-19);
        let assign6970_e6805: f64 = (assign6970_e6803 * locals.var_neff);
        let assign6970_e6807: f64 = (assign6970_e6805 * locals.var_epsch);
        let assign6970_e6809: f64 = (assign6970_e6807 * locals.var_inv_phit);
        (locals.var_a0_csisq, locals.var_a0_csisq_dn4, locals.var_a0_csisq_dn6, locals.var_a0_csisq_dn7, locals.var_a0_csisq_dn8, locals.var_a0_csisq_dn9, ) = (assign6970_e6809, ((((assign6970_e6803 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign6970_e6807 * locals.var_inv_phit_dn4)), ((((assign6970_e6803 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign6970_e6807 * locals.var_inv_phit_dn6)), ((((assign6970_e6803 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign6970_e6807 * locals.var_inv_phit_dn7)), ((((assign6970_e6803 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign6970_e6807 * locals.var_inv_phit_dn8)), ((((assign6970_e6803 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign6970_e6807 * locals.var_inv_phit_dn9)), );

        let assign6980_e6812: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign6980_e6814: f64 = (assign6980_e6812 / locals.var_a0_csisq);
        let assign6980_e6815: f64 = (assign6980_e6814).ln();
        let assign6980_e6817: f64 = (assign6980_e6815 - 0.6931471805599);
        (locals.var_xth_1d, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9, ) = (assign6980_e6817, ((-((assign6980_e6812 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6980_e6814), ((-((assign6980_e6812 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6980_e6814), ((-((assign6980_e6812 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6980_e6814), ((-((assign6980_e6812 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6980_e6814), ((-((assign6980_e6812 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6980_e6814), );

        let assign6990_e6820: f64 = (0.5 * 1.602176565e-19);
        let assign6990_e6822: f64 = (assign6990_e6820 * locals.var_nsddc_i);
        let assign6990_e6824: f64 = (assign6990_e6822 * locals.var_tsi_i);
        let assign6990_e6827: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign6990_e6828: f64 = (assign6990_e6824 / assign6990_e6827);
        let assign6990_e6830: f64 = (assign6990_e6828 * locals.var_inv_phit);
        (locals.var_xsddep, locals.var_xsddep_dn4, locals.var_xsddep_dn6, locals.var_xsddep_dn7, locals.var_xsddep_dn8, locals.var_xsddep_dn9, ) = (assign6990_e6830, (assign6990_e6828 * locals.var_inv_phit_dn4), (assign6990_e6828 * locals.var_inv_phit_dn6), (assign6990_e6828 * locals.var_inv_phit_dn7), (assign6990_e6828 * locals.var_inv_phit_dn8), (assign6990_e6828 * locals.var_inv_phit_dn9), );

        let assign7000_e6833: f64 = (locals.var_stcf_i * locals.var_dt);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign7000_e6833, ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4)), ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6)), ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7)), ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8)), ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9)), );

        let assign7010_e6836: f64 = (locals.var_cf1_t + locals.var_temp);
        (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9, ) = (assign7010_e6836, (locals.var_cf1_t_dn4 + locals.var_temp_dn4), (locals.var_cf1_t_dn6 + locals.var_temp_dn6), (locals.var_cf1_t_dn7 + locals.var_temp_dn7), (locals.var_cf1_t_dn8 + locals.var_temp_dn8), (locals.var_cf1_t_dn9 + locals.var_temp_dn9), );

        let assign7020_e6839: f64 = (locals.var_cf2_t + locals.var_temp);
        (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9, ) = (assign7020_e6839, (locals.var_cf2_t_dn4 + locals.var_temp_dn4), (locals.var_cf2_t_dn6 + locals.var_temp_dn6), (locals.var_cf2_t_dn7 + locals.var_temp_dn7), (locals.var_cf2_t_dn8 + locals.var_temp_dn8), (locals.var_cf2_t_dn9 + locals.var_temp_dn9), );

        let assign7030_e6842: f64 = (locals.var_cfac1_t + locals.var_temp);
        (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9, ) = (assign7030_e6842, (locals.var_cfac1_t_dn4 + locals.var_temp_dn4), (locals.var_cfac1_t_dn6 + locals.var_temp_dn6), (locals.var_cfac1_t_dn7 + locals.var_temp_dn7), (locals.var_cfac1_t_dn8 + locals.var_temp_dn8), (locals.var_cfac1_t_dn9 + locals.var_temp_dn9), );

        let assign7040_e6845: f64 = (locals.var_cfac2_t + locals.var_temp);
        (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9, ) = (assign7040_e6845, (locals.var_cfac2_t_dn4 + locals.var_temp_dn4), (locals.var_cfac2_t_dn6 + locals.var_temp_dn6), (locals.var_cfac2_t_dn7 + locals.var_temp_dn7), (locals.var_cfac2_t_dn8 + locals.var_temp_dn8), (locals.var_cfac2_t_dn9 + locals.var_temp_dn9), );

        let assign7050_e6848: f64 = (locals.var_cfd_i * locals.var_inv_phit);
        (locals.var_xd0, locals.var_xd0_dn4, locals.var_xd0_dn6, locals.var_xd0_dn7, locals.var_xd0_dn8, locals.var_xd0_dn9, ) = (assign7050_e6848, (locals.var_cfd_i * locals.var_inv_phit_dn4), (locals.var_cfd_i * locals.var_inv_phit_dn6), (locals.var_cfd_i * locals.var_inv_phit_dn7), (locals.var_cfd_i * locals.var_inv_phit_dn8), (locals.var_cfd_i * locals.var_inv_phit_dn9), );

        let assign7060_e6851: f64 = (2.0 * 1.602176565e-19);
        let assign7060_e6853: f64 = (assign7060_e6851 * locals.var_nsub_i);
        let assign7060_e6855: f64 = (assign7060_e6853 * 1.04479e-10);
        let assign7060_e6857: f64 = (assign7060_e6855 * locals.var_inv_phit0);
        let assign7060_e6858: f64 = (assign7060_e6857).sqrt();
        let assign7060_e6860: f64 = (assign7060_e6858 / locals.var_cox2prime);
        (locals.var_gfsub, locals.var_gfsub_dn4, locals.var_gfsub_dn6, locals.var_gfsub_dn7, locals.var_gfsub_dn8, locals.var_gfsub_dn9, ) = (assign7060_e6860, (((assign7060_e6855 * locals.var_inv_phit0_dn4) / (2.0 * assign7060_e6858)) / locals.var_cox2prime), (((assign7060_e6855 * locals.var_inv_phit0_dn6) / (2.0 * assign7060_e6858)) / locals.var_cox2prime), (((assign7060_e6855 * locals.var_inv_phit0_dn7) / (2.0 * assign7060_e6858)) / locals.var_cox2prime), (((assign7060_e6855 * locals.var_inv_phit0_dn8) / (2.0 * assign7060_e6858)) / locals.var_cox2prime), (((assign7060_e6855 * locals.var_inv_phit0_dn9) / (2.0 * assign7060_e6858)) / locals.var_cox2prime), );

        let assign7070_e6863: f64 = (locals.var_gfsub * locals.var_gfsub);
        (locals.var_gfsub2, locals.var_gfsub2_dn4, locals.var_gfsub2_dn6, locals.var_gfsub2_dn7, locals.var_gfsub2_dn8, locals.var_gfsub2_dn9, ) = (assign7070_e6863, ((locals.var_gfsub_dn4 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn4)), ((locals.var_gfsub_dn6 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn6)), ((locals.var_gfsub_dn7 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn7)), ((locals.var_gfsub_dn8 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn8)), ((locals.var_gfsub_dn9 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn9)), );

        let assign7080_e6866: f64 = (1.0 / locals.var_gfsub2);
        (locals.var_inv_gfsub2, locals.var_inv_gfsub2_dn4, locals.var_inv_gfsub2_dn6, locals.var_inv_gfsub2_dn7, locals.var_inv_gfsub2_dn8, locals.var_inv_gfsub2_dn9, ) = (assign7080_e6866, (-(locals.var_gfsub2_dn4 / (locals.var_gfsub2 * locals.var_gfsub2))), (-(locals.var_gfsub2_dn6 / (locals.var_gfsub2 * locals.var_gfsub2))), (-(locals.var_gfsub2_dn7 / (locals.var_gfsub2 * locals.var_gfsub2))), (-(locals.var_gfsub2_dn8 / (locals.var_gfsub2 * locals.var_gfsub2))), (-(locals.var_gfsub2_dn9 / (locals.var_gfsub2 * locals.var_gfsub2))), );

        let assign7090_e6870: f64 = (locals.var_gfsub / 1.4142135623731);
        let assign7090_e6871: f64 = (1.0 + assign7090_e6870);
        (locals.var_xisub, locals.var_xisub_dn4, locals.var_xisub_dn6, locals.var_xisub_dn7, locals.var_xisub_dn8, locals.var_xisub_dn9, ) = (assign7090_e6871, (locals.var_gfsub_dn4 / 1.4142135623731), (locals.var_gfsub_dn6 / 1.4142135623731), (locals.var_gfsub_dn7 / 1.4142135623731), (locals.var_gfsub_dn8 / 1.4142135623731), (locals.var_gfsub_dn9 / 1.4142135623731), );

        let assign7100_e6874: f64 = (1.0 / locals.var_xisub);
        (locals.var_inv_xisub, locals.var_inv_xisub_dn4, locals.var_inv_xisub_dn6, locals.var_inv_xisub_dn7, locals.var_inv_xisub_dn8, locals.var_inv_xisub_dn9, ) = (assign7100_e6874, (-(locals.var_xisub_dn4 / (locals.var_xisub * locals.var_xisub))), (-(locals.var_xisub_dn6 / (locals.var_xisub * locals.var_xisub))), (-(locals.var_xisub_dn7 / (locals.var_xisub * locals.var_xisub))), (-(locals.var_xisub_dn8 / (locals.var_xisub * locals.var_xisub))), (-(locals.var_xisub_dn9 / (locals.var_xisub * locals.var_xisub))), );

        let assign7110_e6877: f64 = (1e-5 * locals.var_xisub);
        locals.var_margin_sub = assign7110_e6877;

        let assign7120_e6880: f64 = (locals.var_nsub_i / locals.var_neff_sub);
        let assign7120_e6881: f64 = (assign7120_e6880).ln();
        let assign7120_e6883: f64 = (assign7120_e6881 + locals.var_eg_2phit0);
        (locals.var_xb_sub, locals.var_xb_sub_dn4, locals.var_xb_sub_dn6, locals.var_xb_sub_dn7, locals.var_xb_sub_dn8, locals.var_xb_sub_dn9, ) = (assign7120_e6883, (((-((locals.var_nsub_i * locals.var_neff_sub_dn4) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7120_e6880) + locals.var_eg_2phit0_dn4), (((-((locals.var_nsub_i * locals.var_neff_sub_dn6) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7120_e6880) + locals.var_eg_2phit0_dn6), (((-((locals.var_nsub_i * locals.var_neff_sub_dn7) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7120_e6880) + locals.var_eg_2phit0_dn7), (((-((locals.var_nsub_i * locals.var_neff_sub_dn8) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7120_e6880) + locals.var_eg_2phit0_dn8), (((-((locals.var_nsub_i * locals.var_neff_sub_dn9) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7120_e6880) + locals.var_eg_2phit0_dn9), );

        let assign7130_e6886: f64 = (2.0 * locals.var_xb_sub);
        (locals.var_xn_sub, locals.var_xn_sub_dn4, locals.var_xn_sub_dn6, locals.var_xn_sub_dn7, locals.var_xn_sub_dn8, locals.var_xn_sub_dn9, ) = (assign7130_e6886, (2.0 * locals.var_xb_sub_dn4), (2.0 * locals.var_xb_sub_dn6), (2.0 * locals.var_xb_sub_dn7), (2.0 * locals.var_xb_sub_dn8), (2.0 * locals.var_xb_sub_dn9), );

        let assign7140_e6889: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7140_e6889;

        if (locals.var_guard142 != 0.0) {
            let assign7150_e6894: f64 = (locals.var_typesub_i * locals.var_phit0);
            let assign7150_e6896: f64 = (assign7150_e6894 * locals.var_xb_sub);
            let assign7150_e6897: f64 = (locals.var_vfb2_t + assign7150_e6896);
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (assign7150_e6897, (locals.var_vfb2_t_dn4 + (((locals.var_typesub_i * locals.var_phit0_dn4) * locals.var_xb_sub) + (assign7150_e6894 * locals.var_xb_sub_dn4))), (locals.var_vfb2_t_dn6 + (((locals.var_typesub_i * locals.var_phit0_dn6) * locals.var_xb_sub) + (assign7150_e6894 * locals.var_xb_sub_dn6))), (locals.var_vfb2_t_dn7 + (((locals.var_typesub_i * locals.var_phit0_dn7) * locals.var_xb_sub) + (assign7150_e6894 * locals.var_xb_sub_dn7))), (locals.var_vfb2_t_dn8 + (((locals.var_typesub_i * locals.var_phit0_dn8) * locals.var_xb_sub) + (assign7150_e6894 * locals.var_xb_sub_dn8))), (locals.var_vfb2_t_dn9 + (((locals.var_typesub_i * locals.var_phit0_dn9) * locals.var_xb_sub) + (assign7150_e6894 * locals.var_xb_sub_dn9))), );
        }

        if (locals.var_guard142 != 0.0) {
            let assign7160_e6904: f64 = (locals.var_typesub_i * locals.var_phit0);
            let assign7160_e6906: f64 = (assign7160_e6904 * locals.var_xb_sub);
            let assign7160_e6907: f64 = (locals.var_vfbac2_t + assign7160_e6906);
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (assign7160_e6907, (locals.var_vfbac2_t_dn4 + (((locals.var_typesub_i * locals.var_phit0_dn4) * locals.var_xb_sub) + (assign7160_e6904 * locals.var_xb_sub_dn4))), (locals.var_vfbac2_t_dn6 + (((locals.var_typesub_i * locals.var_phit0_dn6) * locals.var_xb_sub) + (assign7160_e6904 * locals.var_xb_sub_dn6))), (locals.var_vfbac2_t_dn7 + (((locals.var_typesub_i * locals.var_phit0_dn7) * locals.var_xb_sub) + (assign7160_e6904 * locals.var_xb_sub_dn7))), (locals.var_vfbac2_t_dn8 + (((locals.var_typesub_i * locals.var_phit0_dn8) * locals.var_xb_sub) + (assign7160_e6904 * locals.var_xb_sub_dn8))), (locals.var_vfbac2_t_dn9 + (((locals.var_typesub_i * locals.var_phit0_dn9) * locals.var_xb_sub) + (assign7160_e6904 * locals.var_xb_sub_dn9))), );
        }

        (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7180_e6913: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7180_e6913;

        if (locals.var_guard143 != 0.0) {
            let assign7190_e6918: f64 = (locals.var_np_i / locals.var_neff_poly);
            let assign7190_e6919: f64 = (assign7190_e6918).ln();
            let assign7190_e6921: f64 = (assign7190_e6919 + locals.var_eg_2phit0);
            let assign7190_e6922: f64 = (locals.var_phit0 * assign7190_e6921);
            (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9, ) = (assign7190_e6922, ((locals.var_phit0_dn4 * assign7190_e6921) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7190_e6918) + locals.var_eg_2phit0_dn4))), ((locals.var_phit0_dn6 * assign7190_e6921) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7190_e6918) + locals.var_eg_2phit0_dn6))), ((locals.var_phit0_dn7 * assign7190_e6921) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7190_e6918) + locals.var_eg_2phit0_dn7))), ((locals.var_phit0_dn8 * assign7190_e6921) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7190_e6918) + locals.var_eg_2phit0_dn8))), ((locals.var_phit0_dn9 * assign7190_e6921) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7190_e6918) + locals.var_eg_2phit0_dn9))), );
        }

        let assign7200_e6927: f64 = (2.0 * 1.602176565e-19);
        let assign7200_e6929: f64 = (assign7200_e6927 * locals.var_epsch);
        let assign7200_e6931: f64 = (assign7200_e6929 * locals.var_np_i);
        let assign7200_e6932: f64 = (assign7200_e6931).sqrt();
        let assign7200_e6934: f64 = (assign7200_e6932 / locals.var_cox1init);
        (locals.var_kp, locals.var_kp_dn4, locals.var_kp_dn6, locals.var_kp_dn7, locals.var_kp_dn8, locals.var_kp_dn9, ) = (assign7200_e6934, (((assign7200_e6929 * locals.var_np_i_dn4) / (2.0 * assign7200_e6932)) / locals.var_cox1init), (((assign7200_e6929 * locals.var_np_i_dn6) / (2.0 * assign7200_e6932)) / locals.var_cox1init), (((assign7200_e6929 * locals.var_np_i_dn7) / (2.0 * assign7200_e6932)) / locals.var_cox1init), (((assign7200_e6929 * locals.var_np_i_dn8) / (2.0 * assign7200_e6932)) / locals.var_cox1init), (((assign7200_e6929 * locals.var_np_i_dn9) / (2.0 * assign7200_e6932)) / locals.var_cox1init), );

        (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9, ) = (15.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7220_e6938: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard144 = assign7220_e6938;

        if (locals.var_guard144 != 0.0) {
            let assign7230_e6944: f64 = (2970.0 / locals.var_tkd);
            let assign7230_e6945: f64 = (15.0 + assign7230_e6944);
            let assign7230_e6949: f64 = (2970.0 / locals.var_tkd);
            let assign7230_e6950: f64 = (15.0 - assign7230_e6949);
            let assign7230_e6954: f64 = (2970.0 / locals.var_tkd);
            let assign7230_e6955: f64 = (15.0 - assign7230_e6954);
            let assign7230_e6956: f64 = (assign7230_e6950 * assign7230_e6955);
            let assign7230_e6958: f64 = (assign7230_e6956 + 1e-6);
            let assign7230_e6959: f64 = (assign7230_e6958).sqrt();
            let assign7230_e6960: f64 = (assign7230_e6945 + assign7230_e6959);
            let assign7230_e6961: f64 = (0.5 * assign7230_e6960);
            (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9, ) = (assign7230_e6961, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign7230_e6955) + (assign7230_e6950 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7230_e6959)))), );
        }

        locals.var_dvfbqm = 0.0;

        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7260_e6968: f64 = (1e18 * locals.var_tsi_i);
        let assign7260_e6970: f64 = (assign7260_e6968 * locals.var_tsi_i);
        locals.var_tsisq = assign7260_e6970;

        let assign7270_e6973: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7270_e6973;

        let assign7280_e6976: f64 = 1.0;
        let assign7280_e6977: f64 = if p.p14 == assign7280_e6976 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7280_e6977;

        if ((locals.var_guard145 != 0.0) && (locals.var_guard146 != 0.0)) {
            let assign7290_e6983: f64 = (0.409618895 / locals.var_tsisq);
            locals.var_dvfbqm = assign7290_e6983;
        }

        if ((locals.var_guard145 != 0.0) && (locals.var_guard146 != 0.0)) {
            let assign7300_e6991: f64 = (0.4 * p.p13);
            let assign7300_e6993: f64 = (assign7300_e6991 * 1.27520989);
            let assign7300_e6995: f64 = (-0.3333333333333);
            let assign7300_e6998: f64 = (locals.var_phit * locals.var_tsisq);
            let assign7300_e6999: f64 = (assign7300_e6998).ln();
            let assign7300_e7000: f64 = (assign7300_e6995 * assign7300_e6999);
            let assign7300_e7001: f64 = (assign7300_e7000).exp();
            let assign7300_e7002: f64 = (assign7300_e6993 * assign7300_e7001);
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (assign7300_e7002, (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign7300_e6998)))), (assign7300_e6993 * (assign7300_e7001 * (assign7300_e6995 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign7300_e6998)))), );
        }

        if ((locals.var_guard145 != 0.0) && (locals.var_guard146 == 0.0)) {
            let assign7310_e7011: f64 = (0.723134895 / locals.var_tsisq);
            locals.var_dvfbqm = assign7310_e7011;
        }

        if ((locals.var_guard145 != 0.0) && (locals.var_guard146 == 0.0)) {
            let assign7320_e7020: f64 = (0.4 * p.p13);
            let assign7320_e7022: f64 = (assign7320_e7020 * 1.5412087);
            let assign7320_e7024: f64 = (-0.3333333333333);
            let assign7320_e7027: f64 = (locals.var_phit * locals.var_tsisq);
            let assign7320_e7028: f64 = (assign7320_e7027).ln();
            let assign7320_e7029: f64 = (assign7320_e7024 * assign7320_e7028);
            let assign7320_e7030: f64 = (assign7320_e7029).exp();
            let assign7320_e7031: f64 = (assign7320_e7022 * assign7320_e7030);
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (assign7320_e7031, (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign7320_e7027)))), (assign7320_e7022 * (assign7320_e7030 * (assign7320_e7024 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign7320_e7027)))), );
        }

        let assign7330_e7036: f64 = (p.p14 * locals.var_stvfb_i);
        let assign7330_e7038: f64 = (assign7330_e7036 * locals.var_dt);
        let assign7330_e7040: f64 = (assign7330_e7038 + locals.var_dvfbqm);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign7330_e7040, (assign7330_e7036 * locals.var_dt_dn4), (assign7330_e7036 * locals.var_dt_dn6), (assign7330_e7036 * locals.var_dt_dn7), (assign7330_e7036 * locals.var_dt_dn8), (assign7330_e7036 * locals.var_dt_dn9), );

        let assign7340_e7043: f64 = (locals.var_temp + p.p34);
        let assign7340_e7045: f64 = (assign7340_e7043 - locals.var_dvfbpdep);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign7340_e7045, (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4), (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6), (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7), (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8), (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9), );

        let assign7350_e7049: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
        let assign7350_e7051: f64 = (assign7350_e7049 + locals.var_dvfb1nch);
        let assign7350_e7052: f64 = (p.p14 * assign7350_e7051);
        let assign7350_e7054: f64 = (assign7350_e7052 + locals.var_temp1);
        (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9, ) = (assign7350_e7054, ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9), );

        let assign7360_e7058: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
        let assign7360_e7060: f64 = (assign7360_e7058 + locals.var_dvfb2nch);
        let assign7360_e7061: f64 = (p.p14 * assign7360_e7060);
        let assign7360_e7063: f64 = (assign7360_e7061 + locals.var_temp);
        (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9, ) = (assign7360_e7063, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );

        let assign7370_e7067: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
        let assign7370_e7069: f64 = (assign7370_e7067 + locals.var_dvfb1nch);
        let assign7370_e7070: f64 = (p.p14 * assign7370_e7069);
        let assign7370_e7072: f64 = (assign7370_e7070 + locals.var_temp1);
        (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9, ) = (assign7370_e7072, ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9), );

        let assign7380_e7076: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
        let assign7380_e7078: f64 = (assign7380_e7076 + locals.var_dvfb2nch);
        let assign7380_e7079: f64 = (p.p14 * assign7380_e7078);
        let assign7380_e7081: f64 = (assign7380_e7079 + locals.var_temp);
        (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9, ) = (assign7380_e7081, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );

        let assign7390_e7083: f64 = (locals.var_rtn).ln();
        (locals.var_lnrtn, locals.var_lnrtn_dn4, locals.var_lnrtn_dn6, locals.var_lnrtn_dn7, locals.var_lnrtn_dn8, locals.var_lnrtn_dn9, ) = (assign7390_e7083, (locals.var_rtn_dn4 / locals.var_rtn), (locals.var_rtn_dn6 / locals.var_rtn), (locals.var_rtn_dn7 / locals.var_rtn), (locals.var_rtn_dn8 / locals.var_rtn), (locals.var_rtn_dn9 / locals.var_rtn), );

        let assign7400_e7086: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign7400_e7087: f64 = (assign7400_e7086).exp();
        let assign7400_e7089: f64 = (assign7400_e7087 * p.p35);
        (locals.var_tf_bet, locals.var_tf_bet_dn4, locals.var_tf_bet_dn6, locals.var_tf_bet_dn7, locals.var_tf_bet_dn8, locals.var_tf_bet_dn9, ) = (assign7400_e7089, ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35), ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35), ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35), ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35), ((assign7400_e7087 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35), );

        let assign7410_e7092: f64 = (locals.var_betn1_t * locals.var_tf_bet);
        (locals.var_betn1_i, locals.var_betn1_i_dn4, locals.var_betn1_i_dn6, locals.var_betn1_i_dn7, locals.var_betn1_i_dn8, locals.var_betn1_i_dn9, ) = (assign7410_e7092, ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4)), ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6)), ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7)), ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8)), ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9)), );

        let assign7420_e7095: f64 = (locals.var_betn2_t * locals.var_tf_bet);
        (locals.var_betn2_i, locals.var_betn2_i_dn4, locals.var_betn2_i_dn6, locals.var_betn2_i_dn7, locals.var_betn2_i_dn8, locals.var_betn2_i_dn9, ) = (assign7420_e7095, ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4)), ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6)), ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7)), ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8)), ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9)), );

        let assign7430_e7098: f64 = (locals.var_stmue_i * locals.var_lnrtn);
        let assign7430_e7099: f64 = (assign7430_e7098).exp();
        (locals.var_tf_mue, locals.var_tf_mue_dn4, locals.var_tf_mue_dn6, locals.var_tf_mue_dn7, locals.var_tf_mue_dn8, locals.var_tf_mue_dn9, ) = (assign7430_e7099, (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn4)), (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn6)), (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn7)), (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn8)), (assign7430_e7099 * (locals.var_stmue_i * locals.var_lnrtn_dn9)), );

        let assign7440_e7102: f64 = (locals.var_mue_t * locals.var_tf_mue);
        (locals.var_mue_i, locals.var_mue_i_dn4, locals.var_mue_i_dn6, locals.var_mue_i_dn7, locals.var_mue_i_dn8, locals.var_mue_i_dn9, ) = (assign7440_e7102, (locals.var_mue_t * locals.var_tf_mue_dn4), (locals.var_mue_t * locals.var_tf_mue_dn6), (locals.var_mue_t * locals.var_tf_mue_dn7), (locals.var_mue_t * locals.var_tf_mue_dn8), (locals.var_mue_t * locals.var_tf_mue_dn9), );

        let assign7450_e7105: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
        let assign7450_e7106: f64 = (assign7450_e7105).exp();
        (locals.var_tf_themu, locals.var_tf_themu_dn4, locals.var_tf_themu_dn6, locals.var_tf_themu_dn7, locals.var_tf_themu_dn8, locals.var_tf_themu_dn9, ) = (assign7450_e7106, (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn4)), (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn6)), (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn7)), (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn8)), (assign7450_e7106 * (locals.var_stthemu_i * locals.var_lnrtn_dn9)), );

        let assign7460_e7109: f64 = (locals.var_themu_t * locals.var_tf_themu);
        (locals.var_themu_i, locals.var_themu_i_dn4, locals.var_themu_i_dn6, locals.var_themu_i_dn7, locals.var_themu_i_dn8, locals.var_themu_i_dn9, ) = (assign7460_e7109, (locals.var_themu_t * locals.var_tf_themu_dn4), (locals.var_themu_t * locals.var_tf_themu_dn6), (locals.var_themu_t * locals.var_tf_themu_dn7), (locals.var_themu_t * locals.var_tf_themu_dn8), (locals.var_themu_t * locals.var_tf_themu_dn9), );

        let assign7470_e7112: f64 = (locals.var_stcs_i * locals.var_lnrtn);
        let assign7470_e7113: f64 = (assign7470_e7112).exp();
        (locals.var_tf_cs, locals.var_tf_cs_dn4, locals.var_tf_cs_dn6, locals.var_tf_cs_dn7, locals.var_tf_cs_dn8, locals.var_tf_cs_dn9, ) = (assign7470_e7113, (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn4)), (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn6)), (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn7)), (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn8)), (assign7470_e7113 * (locals.var_stcs_i * locals.var_lnrtn_dn9)), );

        let assign7480_e7116: f64 = (locals.var_cs_t * locals.var_tf_cs);
        (locals.var_cs_i, locals.var_cs_i_dn4, locals.var_cs_i_dn6, locals.var_cs_i_dn7, locals.var_cs_i_dn8, locals.var_cs_i_dn9, ) = (assign7480_e7116, (locals.var_cs_t * locals.var_tf_cs_dn4), (locals.var_cs_t * locals.var_tf_cs_dn6), (locals.var_cs_t * locals.var_tf_cs_dn7), (locals.var_cs_t * locals.var_tf_cs_dn8), (locals.var_cs_t * locals.var_tf_cs_dn9), );

        let assign7490_e7119: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
        let assign7490_e7120: f64 = (assign7490_e7119).exp();
        (locals.var_tf_thecs, locals.var_tf_thecs_dn4, locals.var_tf_thecs_dn6, locals.var_tf_thecs_dn7, locals.var_tf_thecs_dn8, locals.var_tf_thecs_dn9, ) = (assign7490_e7120, (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn4)), (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn6)), (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn7)), (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn8)), (assign7490_e7120 * (locals.var_stthecs_i * locals.var_lnrtn_dn9)), );

        let assign7500_e7123: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
        (locals.var_thecs_i, locals.var_thecs_i_dn4, locals.var_thecs_i_dn6, locals.var_thecs_i_dn7, locals.var_thecs_i_dn8, locals.var_thecs_i_dn9, ) = (assign7500_e7123, (locals.var_thecs_t * locals.var_tf_thecs_dn4), (locals.var_thecs_t * locals.var_tf_thecs_dn6), (locals.var_thecs_t * locals.var_tf_thecs_dn7), (locals.var_thecs_t * locals.var_tf_thecs_dn8), (locals.var_thecs_t * locals.var_tf_thecs_dn9), );

        let assign7510_e7126: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
        let assign7510_e7127: f64 = (assign7510_e7126).exp();
        (locals.var_tf_xcor, locals.var_tf_xcor_dn4, locals.var_tf_xcor_dn6, locals.var_tf_xcor_dn7, locals.var_tf_xcor_dn8, locals.var_tf_xcor_dn9, ) = (assign7510_e7127, (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn4)), (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn6)), (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn7)), (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn8)), (assign7510_e7127 * (locals.var_stxcor_i * locals.var_lnrtn_dn9)), );

        let assign7520_e7130: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
        (locals.var_xcor_i, locals.var_xcor_i_dn4, locals.var_xcor_i_dn6, locals.var_xcor_i_dn7, locals.var_xcor_i_dn8, locals.var_xcor_i_dn9, ) = (assign7520_e7130, (locals.var_xcor_t * locals.var_tf_xcor_dn4), (locals.var_xcor_t * locals.var_tf_xcor_dn6), (locals.var_xcor_t * locals.var_tf_xcor_dn7), (locals.var_xcor_t * locals.var_tf_xcor_dn8), (locals.var_xcor_t * locals.var_tf_xcor_dn9), );

        let assign7530_e7133: f64 = (1e-8 * locals.var_phit);
        let assign7530_e7135: f64 = (assign7530_e7133 / locals.var_tsi_i);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign7530_e7135, ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i), );

        let assign7540_e7138: f64 = (locals.var_temp * locals.var_mue_i);
        (locals.var_fmue, locals.var_fmue_dn4, locals.var_fmue_dn6, locals.var_fmue_dn7, locals.var_fmue_dn8, locals.var_fmue_dn9, ) = (assign7540_e7138, ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4)), ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6)), ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7)), ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8)), ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9)), );

        let assign7550_e7142: f64 = (0.5 * locals.var_csthr_i);
        let assign7550_e7143: f64 = (1.0 / assign7550_e7142);
        locals.var_inv_qi1cs = assign7550_e7143;

        let assign7560_e7146: f64 = (locals.var_inv_qi1cs / locals.var_csthrb_i);
        locals.var_inv_qi2cs = assign7560_e7146;

        let assign7570_e7149: f64 = 1.0;
        let assign7570_e7150: f64 = if p.p14 == assign7570_e7149 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7570_e7150;

        if (locals.var_guard147 != 0.0) {
            let assign7580_e7154: f64 = (0.5 * locals.var_feta_i);
            locals.var_eta_mu = assign7580_e7154;
        }

        if (locals.var_guard147 == 0.0) {
            let assign7590_e7161: f64 = (0.3333333333333 * locals.var_feta_i);
            locals.var_eta_mu = assign7590_e7161;
        }

        let assign7600_e7166: f64 = (1.0 - locals.var_eta_mu);
        locals.var_one_m_eta = assign7600_e7166;

        let assign7610_e7169: f64 = (locals.var_strs_i * locals.var_lnrtn);
        let assign7610_e7170: f64 = (assign7610_e7169).exp();
        (locals.var_tf_ther, locals.var_tf_ther_dn4, locals.var_tf_ther_dn6, locals.var_tf_ther_dn7, locals.var_tf_ther_dn8, locals.var_tf_ther_dn9, ) = (assign7610_e7170, (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn4)), (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn6)), (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn7)), (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn8)), (assign7610_e7170 * (locals.var_strs_i * locals.var_lnrtn_dn9)), );

        let assign7620_e7173: f64 = (locals.var_rs_t * locals.var_tf_ther);
        (locals.var_rs_i, locals.var_rs_i_dn4, locals.var_rs_i_dn6, locals.var_rs_i_dn7, locals.var_rs_i_dn8, locals.var_rs_i_dn9, ) = (assign7620_e7173, (locals.var_rs_t * locals.var_tf_ther_dn4), (locals.var_rs_t * locals.var_tf_ther_dn6), (locals.var_rs_t * locals.var_tf_ther_dn7), (locals.var_rs_t * locals.var_tf_ther_dn8), (locals.var_rs_t * locals.var_tf_ther_dn9), );

        let assign7630_e7176: f64 = (2.0 * locals.var_rs_i);
        let assign7630_e7178: f64 = (assign7630_e7176 * locals.var_phit);
        (locals.var_frs, locals.var_frs_dn4, locals.var_frs_dn6, locals.var_frs_dn7, locals.var_frs_dn8, locals.var_frs_dn9, ) = (assign7630_e7178, (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn4)), (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn6)), (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn7)), (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn8)), (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign7630_e7176 * locals.var_phit_dn9)), );

        let assign7640_e7182: f64 = (16.0 / locals.var_ax_i);
        let assign7640_e7184: f64 = (assign7640_e7182 * 0.6931471805599);
        let assign7640_e7185: f64 = (assign7640_e7184).exp();
        let assign7640_e7187: f64 = (assign7640_e7185 - 1.0);
        let assign7640_e7188: f64 = (assign7640_e7187).ln();
        let assign7640_e7189: f64 = (0.375 * assign7640_e7188);
        let assign7640_e7190: f64 = (assign7640_e7189).exp();
        let assign7640_e7192: f64 = (assign7640_e7190 - 1.0);
        locals.var_gamax = assign7640_e7192;

        let assign7650_e7196: f64 = (16.0 / locals.var_axac_i);
        let assign7650_e7198: f64 = (assign7650_e7196 * 0.6931471805599);
        let assign7650_e7199: f64 = (assign7650_e7198).exp();
        let assign7650_e7201: f64 = (assign7650_e7199 - 1.0);
        let assign7650_e7202: f64 = (assign7650_e7201).ln();
        let assign7650_e7203: f64 = (0.375 * assign7650_e7202);
        let assign7650_e7204: f64 = (assign7650_e7203).exp();
        let assign7650_e7206: f64 = (assign7650_e7204 - 1.0);
        locals.var_gamax_ac = assign7650_e7206;

        let assign7660_e7209: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
        let assign7660_e7210: f64 = (assign7660_e7209).exp();
        (locals.var_tf_thesat, locals.var_tf_thesat_dn4, locals.var_tf_thesat_dn6, locals.var_tf_thesat_dn7, locals.var_tf_thesat_dn8, locals.var_tf_thesat_dn9, ) = (assign7660_e7210, (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn4)), (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn6)), (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn7)), (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn8)), (assign7660_e7210 * (locals.var_stthesat_i * locals.var_lnrtn_dn9)), );

        let assign7670_e7213: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
        let assign7670_e7215: f64 = (assign7670_e7213 * locals.var_tf_bet);
        (locals.var_thesat_i, locals.var_thesat_i_dn4, locals.var_thesat_i_dn6, locals.var_thesat_i_dn7, locals.var_thesat_i_dn8, locals.var_thesat_i_dn9, ) = (assign7670_e7215, ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn4)), ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn6)), ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn7)), ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn8)), ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7670_e7213 * locals.var_tf_bet_dn9)), );

        let assign7680_e7218: f64 = (locals.var_thesat_i * locals.var_phit);
        (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9, ) = (assign7680_e7218, ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4)), ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6)), ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7)), ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8)), ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9)), );

        let assign7690_e7221: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
        let assign7690_e7223: f64 = (assign7690_e7221 * locals.var_tf_bet);
        (locals.var_thesatac_i, locals.var_thesatac_i_dn4, locals.var_thesatac_i_dn6, locals.var_thesatac_i_dn7, locals.var_thesatac_i_dn8, locals.var_thesatac_i_dn9, ) = (assign7690_e7223, ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn4)), ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn6)), ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn7)), ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn8)), ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7690_e7221 * locals.var_tf_bet_dn9)), );

        let assign7700_e7226: f64 = (locals.var_thesatac_i * locals.var_phit);
        (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9, ) = (assign7700_e7226, ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4)), ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6)), ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7)), ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8)), ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9)), );

        let assign7710_e7229: f64 = (locals.var_alp1_i * locals.var_inv_phit);
        (locals.var_alp1_phit, locals.var_alp1_phit_dn4, locals.var_alp1_phit_dn6, locals.var_alp1_phit_dn7, locals.var_alp1_phit_dn8, locals.var_alp1_phit_dn9, ) = (assign7710_e7229, (locals.var_alp1_i * locals.var_inv_phit_dn4), (locals.var_alp1_i * locals.var_inv_phit_dn6), (locals.var_alp1_i * locals.var_inv_phit_dn7), (locals.var_alp1_i * locals.var_inv_phit_dn8), (locals.var_alp1_i * locals.var_inv_phit_dn9), );

        let assign7720_e7231: f64 = (-locals.var_stig_i);
        let assign7720_e7233: f64 = (assign7720_e7231 * locals.var_lnrtn);
        let assign7720_e7234: f64 = (assign7720_e7233).exp();
        (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9, ) = (assign7720_e7234, (assign7720_e7234 * (assign7720_e7231 * locals.var_lnrtn_dn4)), (assign7720_e7234 * (assign7720_e7231 * locals.var_lnrtn_dn6)), (assign7720_e7234 * (assign7720_e7231 * locals.var_lnrtn_dn7)), (assign7720_e7234 * (assign7720_e7231 * locals.var_lnrtn_dn8)), (assign7720_e7234 * (assign7720_e7231 * locals.var_lnrtn_dn9)), );

        let assign7730_e7237: f64 = (locals.var_iginv_t * locals.var_tf_ig);
        (locals.var_iginv_i, locals.var_iginv_i_dn4, locals.var_iginv_i_dn6, locals.var_iginv_i_dn7, locals.var_iginv_i_dn8, locals.var_iginv_i_dn9, ) = (assign7730_e7237, (locals.var_iginv_t * locals.var_tf_ig_dn4), (locals.var_iginv_t * locals.var_tf_ig_dn6), (locals.var_iginv_t * locals.var_tf_ig_dn7), (locals.var_iginv_t * locals.var_tf_ig_dn8), (locals.var_iginv_t * locals.var_tf_ig_dn9), );

        let assign7740_e7240: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
        (locals.var_igovinv_i, locals.var_igovinv_i_dn4, locals.var_igovinv_i_dn6, locals.var_igovinv_i_dn7, locals.var_igovinv_i_dn8, locals.var_igovinv_i_dn9, ) = (assign7740_e7240, (locals.var_igovinv_t * locals.var_tf_ig_dn4), (locals.var_igovinv_t * locals.var_tf_ig_dn6), (locals.var_igovinv_t * locals.var_tf_ig_dn7), (locals.var_igovinv_t * locals.var_tf_ig_dn8), (locals.var_igovinv_t * locals.var_tf_ig_dn9), );

        let assign7750_e7243: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
        (locals.var_igovinvd_i, locals.var_igovinvd_i_dn4, locals.var_igovinvd_i_dn6, locals.var_igovinvd_i_dn7, locals.var_igovinvd_i_dn8, locals.var_igovinvd_i_dn9, ) = (assign7750_e7243, (locals.var_igovinvd_t * locals.var_tf_ig_dn4), (locals.var_igovinvd_t * locals.var_tf_ig_dn6), (locals.var_igovinvd_t * locals.var_tf_ig_dn7), (locals.var_igovinvd_t * locals.var_tf_ig_dn8), (locals.var_igovinvd_t * locals.var_tf_ig_dn9), );

        let assign7760_e7246: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
        (locals.var_igovacc_i, locals.var_igovacc_i_dn4, locals.var_igovacc_i_dn6, locals.var_igovacc_i_dn7, locals.var_igovacc_i_dn8, locals.var_igovacc_i_dn9, ) = (assign7760_e7246, (locals.var_igovacc_t * locals.var_tf_ig_dn4), (locals.var_igovacc_t * locals.var_tf_ig_dn6), (locals.var_igovacc_t * locals.var_tf_ig_dn7), (locals.var_igovacc_t * locals.var_tf_ig_dn8), (locals.var_igovacc_t * locals.var_tf_ig_dn9), );

    }

    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign7770_e7249: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
        (locals.var_igovaccd_i, locals.var_igovaccd_i_dn4, locals.var_igovaccd_i_dn6, locals.var_igovaccd_i_dn7, locals.var_igovaccd_i_dn8, locals.var_igovaccd_i_dn9, ) = (assign7770_e7249, (locals.var_igovaccd_t * locals.var_tf_ig_dn4), (locals.var_igovaccd_t * locals.var_tf_ig_dn6), (locals.var_igovaccd_t * locals.var_tf_ig_dn7), (locals.var_igovaccd_t * locals.var_tf_ig_dn8), (locals.var_igovaccd_t * locals.var_tf_ig_dn9), );

        let assign7780_e7251: f64 = (-locals.var_stigfn_i);
        let assign7780_e7253: f64 = (assign7780_e7251 * locals.var_lnrtn);
        let assign7780_e7254: f64 = (assign7780_e7253).exp();
        (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9, ) = (assign7780_e7254, (assign7780_e7254 * (assign7780_e7251 * locals.var_lnrtn_dn4)), (assign7780_e7254 * (assign7780_e7251 * locals.var_lnrtn_dn6)), (assign7780_e7254 * (assign7780_e7251 * locals.var_lnrtn_dn7)), (assign7780_e7254 * (assign7780_e7251 * locals.var_lnrtn_dn8)), (assign7780_e7254 * (assign7780_e7251 * locals.var_lnrtn_dn9)), );

        let assign7790_e7257: f64 = (locals.var_fnovinv_t * locals.var_tf_ig);
        (locals.var_fnovinv_i, locals.var_fnovinv_i_dn4, locals.var_fnovinv_i_dn6, locals.var_fnovinv_i_dn7, locals.var_fnovinv_i_dn8, locals.var_fnovinv_i_dn9, ) = (assign7790_e7257, (locals.var_fnovinv_t * locals.var_tf_ig_dn4), (locals.var_fnovinv_t * locals.var_tf_ig_dn6), (locals.var_fnovinv_t * locals.var_tf_ig_dn7), (locals.var_fnovinv_t * locals.var_tf_ig_dn8), (locals.var_fnovinv_t * locals.var_tf_ig_dn9), );

        let assign7800_e7260: f64 = (locals.var_fnovinvd_t * locals.var_tf_ig);
        (locals.var_fnovinvd_i, locals.var_fnovinvd_i_dn4, locals.var_fnovinvd_i_dn6, locals.var_fnovinvd_i_dn7, locals.var_fnovinvd_i_dn8, locals.var_fnovinvd_i_dn9, ) = (assign7800_e7260, (locals.var_fnovinvd_t * locals.var_tf_ig_dn4), (locals.var_fnovinvd_t * locals.var_tf_ig_dn6), (locals.var_fnovinvd_t * locals.var_tf_ig_dn7), (locals.var_fnovinvd_t * locals.var_tf_ig_dn8), (locals.var_fnovinvd_t * locals.var_tf_ig_dn9), );

        let assign7810_e7263: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign7810_e7263;

        let assign7820_e7266: f64 = (4.0 * 0.3333333333333);
        let assign7820_e7269: f64 = (2.0 * 1.602176565e-19);
        let assign7820_e7271: f64 = (assign7820_e7269 * 9.10938291e-31);
        let assign7820_e7273: f64 = (assign7820_e7271 * locals.var_chib_i);
        let assign7820_e7274: f64 = (assign7820_e7273).sqrt();
        let assign7820_e7275: f64 = (assign7820_e7266 * assign7820_e7274);
        let assign7820_e7277: f64 = (assign7820_e7275 / 1.054571726e-34);
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign7820_e7277, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7830_e7280: f64 = (locals.var_tempm * locals.var_toxp_i);
        (locals.var_bch, locals.var_bch_dn4, locals.var_bch_dn6, locals.var_bch_dn7, locals.var_bch_dn8, locals.var_bch_dn9, ) = (assign7830_e7280, (locals.var_tempm_dn4 * locals.var_toxp_i), (locals.var_tempm_dn6 * locals.var_toxp_i), (locals.var_tempm_dn7 * locals.var_toxp_i), (locals.var_tempm_dn8 * locals.var_toxp_i), (locals.var_tempm_dn9 * locals.var_toxp_i), );

        let assign7840_e7283: f64 = (locals.var_tempm * locals.var_toxp_i);
        (locals.var_bov, locals.var_bov_dn4, locals.var_bov_dn6, locals.var_bov_dn7, locals.var_bov_dn8, locals.var_bov_dn9, ) = (assign7840_e7283, (locals.var_tempm_dn4 * locals.var_toxp_i), (locals.var_tempm_dn6 * locals.var_toxp_i), (locals.var_tempm_dn7 * locals.var_toxp_i), (locals.var_tempm_dn8 * locals.var_toxp_i), (locals.var_tempm_dn9 * locals.var_toxp_i), );

        locals.var_gcqch = 0.0;

        let assign7860_e7287: f64 = if locals.var_gc3ch_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign7860_e7287;

        if (locals.var_guard148 != 0.0) {
            let assign7870_e7290: f64 = (-0.495);
            let assign7870_e7292: f64 = (assign7870_e7290 * locals.var_gc2ch_i);
            let assign7870_e7294: f64 = (assign7870_e7292 / locals.var_gc3ch_i);
            locals.var_gcqch = assign7870_e7294;
        }

        locals.var_gcqovinv = 0.0;

        let assign7890_e7300: f64 = if locals.var_gc3ovinv_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7890_e7300;

        if (locals.var_guard149 != 0.0) {
            let assign7900_e7303: f64 = (-0.495);
            let assign7900_e7305: f64 = (assign7900_e7303 * locals.var_gc2ovinv_i);
            let assign7900_e7307: f64 = (assign7900_e7305 / locals.var_gc3ovinv_i);
            locals.var_gcqovinv = assign7900_e7307;
        }

        locals.var_gcqovacc = 0.0;

        let assign7920_e7313: f64 = if locals.var_gc3ovacc_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7920_e7313;

        if (locals.var_guard150 != 0.0) {
            let assign7930_e7316: f64 = (-0.495);
            let assign7930_e7318: f64 = (assign7930_e7316 * locals.var_gc2ovacc_i);
            let assign7930_e7320: f64 = (assign7930_e7318 / locals.var_gc3ovacc_i);
            locals.var_gcqovacc = assign7930_e7320;
        }

        let assign7940_e7325: f64 = (0.5 * locals.var_eg);
        (locals.var_alpha_b, locals.var_alpha_b_dn4, locals.var_alpha_b_dn6, locals.var_alpha_b_dn7, locals.var_alpha_b_dn8, locals.var_alpha_b_dn9, ) = (assign7940_e7325, (0.5 * locals.var_eg_dn4), (0.5 * locals.var_eg_dn6), (0.5 * locals.var_eg_dn7), (0.5 * locals.var_eg_dn8), (0.5 * locals.var_eg_dn9), );

        let assign7950_e7328: f64 = (locals.var_gco_i * locals.var_phit);
        (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9, ) = (assign7950_e7328, (locals.var_gco_i * locals.var_phit_dn4), (locals.var_gco_i * locals.var_phit_dn6), (locals.var_gco_i * locals.var_phit_dn7), (locals.var_gco_i * locals.var_phit_dn8), (locals.var_gco_i * locals.var_phit_dn9), );

        let assign7960_e7331: f64 = (locals.var_gco_i * locals.var_phit0);
        (locals.var_dov, locals.var_dov_dn4, locals.var_dov_dn6, locals.var_dov_dn7, locals.var_dov_dn8, locals.var_dov_dn9, ) = (assign7960_e7331, (locals.var_gco_i * locals.var_phit0_dn4), (locals.var_gco_i * locals.var_phit0_dn6), (locals.var_gco_i * locals.var_phit0_dn7), (locals.var_gco_i * locals.var_phit0_dn8), (locals.var_gco_i * locals.var_phit0_dn9), );

        let assign7970_e7336: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
        let assign7970_e7337: f64 = (1.0 + assign7970_e7336);
        let assign7970_e7338: f64 = (1.0 / assign7970_e7337);
        (locals.var_n_iginv, locals.var_n_iginv_dn4, locals.var_n_iginv_dn6, locals.var_n_iginv_dn7, locals.var_n_iginv_dn8, locals.var_n_iginv_dn9, ) = (assign7970_e7338, (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign7970_e7337 * assign7970_e7337))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign7970_e7337 * assign7970_e7337))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign7970_e7337 * assign7970_e7337))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign7970_e7337 * assign7970_e7337))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign7970_e7337 * assign7970_e7337))), );

        let assign7980_e7342: f64 = (locals.var_toxp_i * locals.var_toxp_i);
        let assign7980_e7343: f64 = (4e-18 / assign7980_e7342);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign7980_e7343, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7990_e7346: f64 = (locals.var_agidl_i * locals.var_temp);
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9, ) = (assign7990_e7346, ((locals.var_agidl_i_dn4 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn4)), ((locals.var_agidl_i_dn6 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn6)), ((locals.var_agidl_i_dn7 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn7)), ((locals.var_agidl_i_dn8 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn8)), ((locals.var_agidl_i_dn9 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn9)), );

        let assign8000_e7349: f64 = (locals.var_agidld_i * locals.var_temp);
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9, ) = (assign8000_e7349, ((locals.var_agidld_i_dn4 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn4)), ((locals.var_agidld_i_dn6 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn6)), ((locals.var_agidld_i_dn7 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn7)), ((locals.var_agidld_i_dn8 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn8)), ((locals.var_agidld_i_dn9 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn9)), );

        let assign8010_e7352: f64 = (locals.var_toxp_i * 500000000.0);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8010_e7352, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign8020_e7357: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign8020_e7358: f64 = (1.0 + assign8020_e7357);
        let assign8020_e7360: f64 = assign8020_e7358;
        let assign8020_e7364: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign8020_e7365: f64 = (1.0 + assign8020_e7364);
        let assign8020_e7367: f64 = assign8020_e7365;
        let assign8020_e7371: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign8020_e7372: f64 = (1.0 + assign8020_e7371);
        let assign8020_e7374: f64 = assign8020_e7372;
        let assign8020_e7375: f64 = (assign8020_e7367 * assign8020_e7374);
        let assign8020_e7377: f64 = (assign8020_e7375 + 0.01);
        let assign8020_e7378: f64 = (assign8020_e7377).sqrt();
        let assign8020_e7379: f64 = (assign8020_e7360 + assign8020_e7378);
        let assign8020_e7380: f64 = (0.5 * assign8020_e7379);
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign8020_e7380, (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign8020_e7378)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign8020_e7378)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign8020_e7378)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign8020_e7378)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign8020_e7374) + (assign8020_e7367 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign8020_e7378)))), );

        let assign8030_e7383: f64 = (locals.var_bgidl_t * locals.var_tempm);
        let assign8030_e7385: f64 = (assign8030_e7383 * locals.var_temp);
        (locals.var_bgidl_i, locals.var_bgidl_i_dn4, locals.var_bgidl_i_dn6, locals.var_bgidl_i_dn7, locals.var_bgidl_i_dn8, locals.var_bgidl_i_dn9, ) = (assign8030_e7385, (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn4)), (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn6)), (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn7)), (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn8)), (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign8030_e7383 * locals.var_temp_dn9)), );

        let assign8040_e7390: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign8040_e7391: f64 = (1.0 + assign8040_e7390);
        let assign8040_e7393: f64 = assign8040_e7391;
        let assign8040_e7397: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign8040_e7398: f64 = (1.0 + assign8040_e7397);
        let assign8040_e7400: f64 = assign8040_e7398;
        let assign8040_e7404: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign8040_e7405: f64 = (1.0 + assign8040_e7404);
        let assign8040_e7407: f64 = assign8040_e7405;
        let assign8040_e7408: f64 = (assign8040_e7400 * assign8040_e7407);
        let assign8040_e7410: f64 = (assign8040_e7408 + 0.01);
        let assign8040_e7411: f64 = (assign8040_e7410).sqrt();
        let assign8040_e7412: f64 = (assign8040_e7393 + assign8040_e7411);
        let assign8040_e7413: f64 = (0.5 * assign8040_e7412);
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign8040_e7413, (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign8040_e7411)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign8040_e7411)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign8040_e7411)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign8040_e7411)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign8040_e7407) + (assign8040_e7400 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign8040_e7411)))), );

        let assign8050_e7416: f64 = (locals.var_bgidld_t * locals.var_tempm);
        let assign8050_e7418: f64 = (assign8050_e7416 * locals.var_temp);
        (locals.var_bgidld_i, locals.var_bgidld_i_dn4, locals.var_bgidld_i_dn6, locals.var_bgidld_i_dn7, locals.var_bgidld_i_dn8, locals.var_bgidld_i_dn9, ) = (assign8050_e7418, (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn4)), (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn6)), (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn7)), (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn8)), (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign8050_e7416 * locals.var_temp_dn9)), );

        let assign8060_e7421: f64 = (-locals.var_sta2_i);
        let assign8060_e7423: f64 = (assign8060_e7421 * locals.var_lnrtn);
        let assign8060_e7424: f64 = (assign8060_e7423).exp();
        let assign8060_e7425: f64 = (locals.var_a2_t * assign8060_e7424);
        (locals.var_a2_i, locals.var_a2_i_dn4, locals.var_a2_i_dn6, locals.var_a2_i_dn7, locals.var_a2_i_dn8, locals.var_a2_i_dn9, ) = (assign8060_e7425, (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn4))), (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn6))), (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn7))), (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn8))), (locals.var_a2_t * (assign8060_e7424 * (assign8060_e7421 * locals.var_lnrtn_dn9))), );

        let assign8070_e7430: f64 = (locals.var_ctedge_i * locals.var_rtn);
        let assign8070_e7431: f64 = (1.0 + assign8070_e7430);
        let assign8070_e7432: f64 = (locals.var_phit0 * assign8070_e7431);
        (locals.var_phit_edge, locals.var_phit_edge_dn4, locals.var_phit_edge_dn6, locals.var_phit_edge_dn7, locals.var_phit_edge_dn8, locals.var_phit_edge_dn9, ) = (assign8070_e7432, ((locals.var_phit0_dn4 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign8070_e7431) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn9))), );

        let assign8080_e7435: f64 = (1.0 / locals.var_phit_edge);
        (locals.var_inv_phit_edge, locals.var_inv_phit_edge_dn4, locals.var_inv_phit_edge_dn6, locals.var_inv_phit_edge_dn7, locals.var_inv_phit_edge_dn8, locals.var_inv_phit_edge_dn9, ) = (assign8080_e7435, (-(locals.var_phit_edge_dn4 / (locals.var_phit_edge * locals.var_phit_edge))), (-(locals.var_phit_edge_dn6 / (locals.var_phit_edge * locals.var_phit_edge))), (-(locals.var_phit_edge_dn7 / (locals.var_phit_edge * locals.var_phit_edge))), (-(locals.var_phit_edge_dn8 / (locals.var_phit_edge * locals.var_phit_edge))), (-(locals.var_phit_edge_dn9 / (locals.var_phit_edge * locals.var_phit_edge))), );

        let assign8090_e7438: f64 = (2.0 * 1.602176565e-19);
        let assign8090_e7440: f64 = (assign8090_e7438 * locals.var_neff);
        let assign8090_e7442: f64 = (assign8090_e7440 * locals.var_epsch);
        let assign8090_e7444: f64 = (assign8090_e7442 * locals.var_inv_phit_edge);
        (locals.var_a0_csisq_edge, locals.var_a0_csisq_edge_dn4, locals.var_a0_csisq_edge_dn6, locals.var_a0_csisq_edge_dn7, locals.var_a0_csisq_edge_dn8, locals.var_a0_csisq_edge_dn9, ) = (assign8090_e7444, ((((assign8090_e7438 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn4)), ((((assign8090_e7438 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn6)), ((((assign8090_e7438 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn7)), ((((assign8090_e7438 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn8)), ((((assign8090_e7438 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign8090_e7442 * locals.var_inv_phit_edge_dn9)), );

        let assign8100_e7447: f64 = (p.p14 * locals.var_stvfbedge_i);
        let assign8100_e7449: f64 = (assign8100_e7447 * locals.var_dt);
        let assign8100_e7451: f64 = (assign8100_e7449 + locals.var_dvfbqm);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8100_e7451, (assign8100_e7447 * locals.var_dt_dn4), (assign8100_e7447 * locals.var_dt_dn6), (assign8100_e7447 * locals.var_dt_dn7), (assign8100_e7447 * locals.var_dt_dn8), (assign8100_e7447 * locals.var_dt_dn9), );

        let assign8110_e7455: f64 = (locals.var_vfb1edge_t + locals.var_dvfbch);
        let assign8110_e7457: f64 = (assign8110_e7455 + locals.var_dvfb1nch);
        let assign8110_e7458: f64 = (p.p14 * assign8110_e7457);
        let assign8110_e7460: f64 = (assign8110_e7458 + locals.var_temp);
        let assign8110_e7462: f64 = (assign8110_e7460 + p.p34);
        let assign8110_e7464: f64 = (assign8110_e7462 - locals.var_dvfbpdep);
        (locals.var_vfb1edge_i, locals.var_vfb1edge_i_dn4, locals.var_vfb1edge_i_dn6, locals.var_vfb1edge_i_dn7, locals.var_vfb1edge_i_dn8, locals.var_vfb1edge_i_dn9, ) = (assign8110_e7464, (((p.p14 * ((locals.var_vfb1edge_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_dn4), (((p.p14 * ((locals.var_vfb1edge_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_dn6), (((p.p14 * ((locals.var_vfb1edge_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_dn7), (((p.p14 * ((locals.var_vfb1edge_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_dn8), (((p.p14 * ((locals.var_vfb1edge_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_dn9), );

        let assign8120_e7468: f64 = (locals.var_vfb2edge_t + locals.var_dvfbch);
        let assign8120_e7470: f64 = (assign8120_e7468 + locals.var_dvfb2nch);
        let assign8120_e7471: f64 = (p.p14 * assign8120_e7470);
        let assign8120_e7473: f64 = (assign8120_e7471 + locals.var_temp);
        (locals.var_vfb2edge_i, locals.var_vfb2edge_i_dn4, locals.var_vfb2edge_i_dn6, locals.var_vfb2edge_i_dn7, locals.var_vfb2edge_i_dn8, locals.var_vfb2edge_i_dn9, ) = (assign8120_e7473, ((p.p14 * (locals.var_dvfbch_dn4 + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * (locals.var_dvfbch_dn6 + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * (locals.var_dvfbch_dn7 + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * (locals.var_dvfbch_dn8 + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * (locals.var_dvfbch_dn9 + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );

        let assign8130_e7476: f64 = (locals.var_stbetedge_i * locals.var_lnrtn);
        let assign8130_e7477: f64 = (assign8130_e7476).exp();
        let assign8130_e7479: f64 = (assign8130_e7477 * p.p35);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8130_e7479, ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn4)) * p.p35), ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn6)) * p.p35), ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn7)) * p.p35), ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn8)) * p.p35), ((assign8130_e7477 * (locals.var_stbetedge_i * locals.var_lnrtn_dn9)) * p.p35), );

        let assign8140_e7482: f64 = (locals.var_betnedge_t * locals.var_temp);
        (locals.var_betnedge_i, locals.var_betnedge_i_dn4, locals.var_betnedge_i_dn6, locals.var_betnedge_i_dn7, locals.var_betnedge_i_dn8, locals.var_betnedge_i_dn9, ) = (assign8140_e7482, ((locals.var_betnedge_t_dn4 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn4)), ((locals.var_betnedge_t_dn6 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn6)), ((locals.var_betnedge_t_dn7 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn7)), ((locals.var_betnedge_t_dn8 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn8)), ((locals.var_betnedge_t_dn9 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn9)), );

        let assign8150_e7485: f64 = (locals.var_areaq_i * locals.var_phit);
        (locals.var_area_phit, locals.var_area_phit_dn4, locals.var_area_phit_dn6, locals.var_area_phit_dn7, locals.var_area_phit_dn8, locals.var_area_phit_dn9, ) = (assign8150_e7485, (locals.var_areaq_i * locals.var_phit_dn4), (locals.var_areaq_i * locals.var_phit_dn6), (locals.var_areaq_i * locals.var_phit_dn7), (locals.var_areaq_i * locals.var_phit_dn8), (locals.var_areaq_i * locals.var_phit_dn9), );

        let assign8160_e7488: f64 = (0.25 * 1.602176565e-19);
        let assign8160_e7490: f64 = (assign8160_e7488 * locals.var_nsdac_i);
        let assign8160_e7493: f64 = (locals.var_epsch * locals.var_phit);
        let assign8160_e7494: f64 = (assign8160_e7490 / assign8160_e7493);
        (locals.var_inner_sd, locals.var_inner_sd_dn4, locals.var_inner_sd_dn6, locals.var_inner_sd_dn7, locals.var_inner_sd_dn8, locals.var_inner_sd_dn9, ) = (assign8160_e7494, (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn4)) / (assign8160_e7493 * assign8160_e7493))), (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn6)) / (assign8160_e7493 * assign8160_e7493))), (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn7)) / (assign8160_e7493 * assign8160_e7493))), (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn8)) / (assign8160_e7493 * assign8160_e7493))), (-((assign8160_e7490 * (locals.var_epsch * locals.var_phit_dn9)) / (assign8160_e7493 * assign8160_e7493))), );

        let assign8170_e7497: f64 = (locals.var_nsdac_i / locals.var_neff);
        let assign8170_e7498: f64 = (assign8170_e7497).ln();
        (locals.var_xsd, locals.var_xsd_dn4, locals.var_xsd_dn6, locals.var_xsd_dn7, locals.var_xsd_dn8, locals.var_xsd_dn9, ) = (assign8170_e7498, ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign8170_e7497), ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign8170_e7497), ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign8170_e7497), ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign8170_e7497), ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign8170_e7497), );

        let assign8180_e7501: f64 = (locals.var_fif_i * 1.25e-6);
        let assign8180_e7503: f64 = (assign8180_e7501 * locals.var_phit);
        (locals.var_fif_phit, locals.var_fif_phit_dn4, locals.var_fif_phit_dn6, locals.var_fif_phit_dn7, locals.var_fif_phit_dn8, locals.var_fif_phit_dn9, ) = (assign8180_e7503, (assign8180_e7501 * locals.var_phit_dn4), (assign8180_e7501 * locals.var_phit_dn6), (assign8180_e7501 * locals.var_phit_dn7), (assign8180_e7501 * locals.var_phit_dn8), (assign8180_e7501 * locals.var_phit_dn9), );

        let assign8190_e7506: f64 = (locals.var_epsch / 3.45313e-11);
        let assign8190_e7508: f64 = (assign8190_e7506 * locals.var_tsi_i);
        let assign8190_e7511: f64 = (locals.var_tox1_i + 4e-10);
        let assign8190_e7512: f64 = (assign8190_e7508 * assign8190_e7511);
        let assign8190_e7513: f64 = (assign8190_e7512).sqrt();
        locals.var_lambda2d = assign8190_e7513;

        let assign8200_e7516: f64 = (locals.var_strth_i * locals.var_lnrtn);
        let assign8200_e7517: f64 = (assign8200_e7516).exp();
        (locals.var_tf_rth, locals.var_tf_rth_dn4, locals.var_tf_rth_dn6, locals.var_tf_rth_dn7, locals.var_tf_rth_dn8, locals.var_tf_rth_dn9, ) = (assign8200_e7517, (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn4)), (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn6)), (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn7)), (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn8)), (assign8200_e7517 * (locals.var_strth_i * locals.var_lnrtn_dn9)), );

        let assign8210_e7520: f64 = (locals.var_rth_t * locals.var_tf_rth);
        (locals.var_rth_i, locals.var_rth_i_dn4, locals.var_rth_i_dn6, locals.var_rth_i_dn7, locals.var_rth_i_dn8, locals.var_rth_i_dn9, ) = (assign8210_e7520, ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4)), ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6)), ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7)), ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8)), ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9)), );

        let assign8220_e7523: f64 = (4.0 * 1.3806488e-23);
        let assign8220_e7525: f64 = (assign8220_e7523 * locals.var_tkc);
        locals.var_nt0_4kt = assign8220_e7525;

        let assign8230_e7528: f64 = (locals.var_fnt_i * locals.var_nt0_4kt);
        locals.var_nt = assign8230_e7528;

        locals.var_nt0 = locals.var_nt;

        let assign8250_e7532: f64 = (9.10938291e-31 * 1000000000000.0);
        let assign8250_e7534: f64 = (assign8250_e7532 * locals.var_fntexc_i);
        locals.var_fac_exc = assign8250_e7534;

        let assign8380_e7593: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign8380_e7593;

        if (locals.var_guard257 != 0.0) {
            (locals.var_dtc, locals.var_dtc_dn4, ) = ((nv4 - 0.0), 1.0, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8400_e7601: f64 = (locals.var_tkd + locals.var_dtc);
            (locals.var_tkc, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9, ) = (assign8400_e7601, (locals.var_tkd_dn4 + locals.var_dtc_dn4), locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8410_e7607: f64 = (locals.var_tkc * locals.var_tkc);
            (locals.var_tkc_sq, locals.var_tkc_sq_dn4, locals.var_tkc_sq_dn6, locals.var_tkc_sq_dn7, locals.var_tkc_sq_dn8, locals.var_tkc_sq_dn9, ) = (assign8410_e7607, ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4)), ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6)), ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7)), ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8)), ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8420_e7613: f64 = (locals.var_tkc - locals.var_tkr);
            (locals.var_dt, locals.var_dt_dn4, locals.var_dt_dn6, locals.var_dt_dn7, locals.var_dt_dn8, locals.var_dt_dn9, ) = (assign8420_e7613, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8430_e7619: f64 = (locals.var_tkc / locals.var_tkr);
            (locals.var_rt, locals.var_rt_dn4, locals.var_rt_dn6, locals.var_rt_dn7, locals.var_rt_dn8, locals.var_rt_dn9, ) = (assign8430_e7619, (locals.var_tkc_dn4 / locals.var_tkr), (locals.var_tkc_dn6 / locals.var_tkr), (locals.var_tkc_dn7 / locals.var_tkr), (locals.var_tkc_dn8 / locals.var_tkr), (locals.var_tkc_dn9 / locals.var_tkr), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8440_e7625: f64 = (locals.var_tkr / locals.var_tkc);
            (locals.var_rtn, locals.var_rtn_dn4, locals.var_rtn_dn6, locals.var_rtn_dn7, locals.var_rtn_dn8, locals.var_rtn_dn9, ) = (assign8440_e7625, (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8450_e7631: f64 = (locals.var_tkc * 8.617332384961e-5);
            (locals.var_phit0, locals.var_phit0_dn4, locals.var_phit0_dn6, locals.var_phit0_dn7, locals.var_phit0_dn8, locals.var_phit0_dn9, ) = (assign8450_e7631, (locals.var_tkc_dn4 * 8.617332384961e-5), (locals.var_tkc_dn6 * 8.617332384961e-5), (locals.var_tkc_dn7 * 8.617332384961e-5), (locals.var_tkc_dn8 * 8.617332384961e-5), (locals.var_tkc_dn9 * 8.617332384961e-5), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8460_e7637: f64 = (1.0 / locals.var_phit0);
            (locals.var_inv_phit0, locals.var_inv_phit0_dn4, locals.var_inv_phit0_dn6, locals.var_inv_phit0_dn7, locals.var_inv_phit0_dn8, locals.var_inv_phit0_dn9, ) = (assign8460_e7637, (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0))), );
        }

        let assign8470_e7642: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign8470_e7642;

        if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
            let assign8480_e7650: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign8480_e7651: f64 = (10.0 / assign8480_e7650);
            let assign8480_e7653: f64 = (assign8480_e7651 + 600.0);
            let assign8480_e7657: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign8480_e7658: f64 = (10.0 / assign8480_e7657);
            let assign8480_e7660: f64 = (assign8480_e7658 - 600.0);
            let assign8480_e7664: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign8480_e7665: f64 = (10.0 / assign8480_e7664);
            let assign8480_e7667: f64 = (assign8480_e7665 - 600.0);
            let assign8480_e7668: f64 = (assign8480_e7660 * assign8480_e7667);
            let assign8480_e7670: f64 = (assign8480_e7668 + 0.01);
            let assign8480_e7671: f64 = (assign8480_e7670).sqrt();
            let assign8480_e7672: f64 = (assign8480_e7653 + assign8480_e7671);
            let assign8480_e7673: f64 = (0.5 * assign8480_e7672);
            (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9, ) = (assign8480_e7673, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8480_e7650 * assign8480_e7650))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8480_e7657 * assign8480_e7657))) * assign8480_e7667) + (assign8480_e7660 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8480_e7664 * assign8480_e7664))))) / (2.0 * assign8480_e7671)))), );
        }

        if ((locals.var_guard257 != 0.0) && (locals.var_guard258 == 0.0)) {
            (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9, ) = (600.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8500_e7687: f64 = (0.000473 * locals.var_tkc_sq);
            let assign8500_e7690: f64 = (636.0 + locals.var_tkc);
            let assign8500_e7691: f64 = (assign8500_e7687 / assign8500_e7690);
            let assign8500_e7692: f64 = (1.17 - assign8500_e7691);
            (locals.var_egsi, locals.var_egsi_dn4, locals.var_egsi_dn6, locals.var_egsi_dn7, locals.var_egsi_dn8, locals.var_egsi_dn9, ) = (assign8500_e7692, (-((((0.000473 * locals.var_tkc_sq_dn4) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn4)) / (assign8500_e7690 * assign8500_e7690))), (-((((0.000473 * locals.var_tkc_sq_dn6) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn6)) / (assign8500_e7690 * assign8500_e7690))), (-((((0.000473 * locals.var_tkc_sq_dn7) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn7)) / (assign8500_e7690 * assign8500_e7690))), (-((((0.000473 * locals.var_tkc_sq_dn8) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn8)) / (assign8500_e7690 * assign8500_e7690))), (-((((0.000473 * locals.var_tkc_sq_dn9) * assign8500_e7690) - (assign8500_e7687 * locals.var_tkc_dn9)) / (assign8500_e7690 * assign8500_e7690))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8510_e7699: f64 = (0.0004774 * locals.var_tkc_sq);
            let assign8510_e7702: f64 = (235.0 + locals.var_tkc);
            let assign8510_e7703: f64 = (assign8510_e7699 / assign8510_e7702);
            let assign8510_e7704: f64 = (0.744 - assign8510_e7703);
            (locals.var_egge, locals.var_egge_dn4, locals.var_egge_dn6, locals.var_egge_dn7, locals.var_egge_dn8, locals.var_egge_dn9, ) = (assign8510_e7704, (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn4)) / (assign8510_e7702 * assign8510_e7702))), (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn6)) / (assign8510_e7702 * assign8510_e7702))), (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn7)) / (assign8510_e7702 * assign8510_e7702))), (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn8)) / (assign8510_e7702 * assign8510_e7702))), (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign8510_e7702) - (assign8510_e7699 * locals.var_tkc_dn9)) / (assign8510_e7702 * assign8510_e7702))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8520_e7710: f64 = (locals.var_egge - locals.var_egsi);
            let assign8520_e7712: f64 = (-0.4);
            let assign8520_e7714: f64 = (assign8520_e7712 * locals.var_one_m_xge);
            let assign8520_e7715: f64 = (assign8520_e7710 + assign8520_e7714);
            let assign8520_e7717: f64 = (assign8520_e7715 * locals.var_xge_i);
            (locals.var_deg, locals.var_deg_dn4, locals.var_deg_dn6, locals.var_deg_dn7, locals.var_deg_dn8, locals.var_deg_dn9, ) = (assign8520_e7717, ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i), ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i), ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i), ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i), ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8530_e7723: f64 = (locals.var_egsi + locals.var_deg);
            (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, ) = (assign8530_e7723, (locals.var_egsi_dn4 + locals.var_deg_dn4), (locals.var_egsi_dn6 + locals.var_deg_dn6), (locals.var_egsi_dn7 + locals.var_deg_dn7), (locals.var_egsi_dn8 + locals.var_deg_dn8), (locals.var_egsi_dn9 + locals.var_deg_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8540_e7729: f64 = (0.5 * locals.var_eg);
            let assign8540_e7731: f64 = (assign8540_e7729 * locals.var_inv_phit0);
            (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9, ) = (assign8540_e7731, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign8540_e7729 * locals.var_inv_phit0_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8550_e7737: f64 = (0.05 * locals.var_xge_i);
            let assign8550_e7740: f64 = (0.5 * locals.var_deg);
            let assign8550_e7741: f64 = (assign8550_e7737 - assign8550_e7740);
            (locals.var_dvfbch, locals.var_dvfbch_dn4, locals.var_dvfbch_dn6, locals.var_dvfbch_dn7, locals.var_dvfbch_dn8, locals.var_dvfbch_dn9, ) = (assign8550_e7741, (-(0.5 * locals.var_deg_dn4)), (-(0.5 * locals.var_deg_dn6)), (-(0.5 * locals.var_deg_dn7)), (-(0.5 * locals.var_deg_dn8)), (-(0.5 * locals.var_deg_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8560_e7747: f64 = (locals.var_tkc * 0.0033333333333);
            let assign8560_e7748: f64 = (assign8560_e7747).sqrt();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8560_e7748, ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign8560_e7748)), ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign8560_e7748)), ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign8560_e7748)), ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign8560_e7748)), ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign8560_e7748)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8570_e7754: f64 = (4.05e25 * locals.var_temp);
            let assign8570_e7756: f64 = (assign8570_e7754 * locals.var_temp);
            let assign8570_e7758: f64 = (assign8570_e7756 * locals.var_temp);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign8570_e7758, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn4)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn6)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn7)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn8)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign8570_e7754 * locals.var_temp_dn9)) * locals.var_temp) + (assign8570_e7756 * locals.var_temp_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8580_e7764: f64 = (locals.var_temp1 * locals.var_niratio);
            (locals.var_neff, locals.var_neff_dn4, locals.var_neff_dn6, locals.var_neff_dn7, locals.var_neff_dn8, locals.var_neff_dn9, ) = (assign8580_e7764, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8590_e7772: f64 = (locals.var_ct_i * locals.var_rtn);
            let assign8590_e7773: f64 = (1.0 + assign8590_e7772);
            let assign8590_e7774: f64 = (locals.var_phit0 * assign8590_e7773);
            (locals.var_phit, locals.var_phit_dn4, locals.var_phit_dn6, locals.var_phit_dn7, locals.var_phit_dn8, locals.var_phit_dn9, ) = (assign8590_e7774, ((locals.var_phit0_dn4 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign8590_e7773) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8600_e7780: f64 = (1.0 / locals.var_phit);
            (locals.var_inv_phit, locals.var_inv_phit_dn4, locals.var_inv_phit_dn6, locals.var_inv_phit_dn7, locals.var_inv_phit_dn8, locals.var_inv_phit_dn9, ) = (assign8600_e7780, (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8610_e7786: f64 = (0.5 * locals.var_eg);
            let assign8610_e7788: f64 = (assign8610_e7786 * locals.var_inv_phit);
            (locals.var_eg_2phit, locals.var_eg_2phit_dn4, locals.var_eg_2phit_dn6, locals.var_eg_2phit_dn7, locals.var_eg_2phit_dn8, locals.var_eg_2phit_dn9, ) = (assign8610_e7788, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign8610_e7786 * locals.var_inv_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8620_e7794: f64 = (2.0 * 1.602176565e-19);
            let assign8620_e7796: f64 = (assign8620_e7794 * locals.var_neff);
            let assign8620_e7798: f64 = (assign8620_e7796 * locals.var_epsch);
            let assign8620_e7800: f64 = (assign8620_e7798 * locals.var_inv_phit);
            (locals.var_a0_csisq, locals.var_a0_csisq_dn4, locals.var_a0_csisq_dn6, locals.var_a0_csisq_dn7, locals.var_a0_csisq_dn8, locals.var_a0_csisq_dn9, ) = (assign8620_e7800, ((((assign8620_e7794 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn4)), ((((assign8620_e7794 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn6)), ((((assign8620_e7794 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn7)), ((((assign8620_e7794 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn8)), ((((assign8620_e7794 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign8620_e7798 * locals.var_inv_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8630_e7806: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
            let assign8630_e7808: f64 = (assign8630_e7806 / locals.var_a0_csisq);
            let assign8630_e7809: f64 = (assign8630_e7808).ln();
            let assign8630_e7811: f64 = (assign8630_e7809 - 0.6931471805599);
            (locals.var_xth_1d, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9, ) = (assign8630_e7811, ((-((assign8630_e7806 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), ((-((assign8630_e7806 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), ((-((assign8630_e7806 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), ((-((assign8630_e7806 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), ((-((assign8630_e7806 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8630_e7808), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8640_e7817: f64 = (0.5 * 1.602176565e-19);
            let assign8640_e7819: f64 = (assign8640_e7817 * locals.var_nsddc_i);
            let assign8640_e7821: f64 = (assign8640_e7819 * locals.var_tsi_i);
            let assign8640_e7824: f64 = (locals.var_cox1prime + locals.var_cox2prime);
            let assign8640_e7825: f64 = (assign8640_e7821 / assign8640_e7824);
            let assign8640_e7827: f64 = (assign8640_e7825 * locals.var_inv_phit);
            (locals.var_xsddep, locals.var_xsddep_dn4, locals.var_xsddep_dn6, locals.var_xsddep_dn7, locals.var_xsddep_dn8, locals.var_xsddep_dn9, ) = (assign8640_e7827, (assign8640_e7825 * locals.var_inv_phit_dn4), (assign8640_e7825 * locals.var_inv_phit_dn6), (assign8640_e7825 * locals.var_inv_phit_dn7), (assign8640_e7825 * locals.var_inv_phit_dn8), (assign8640_e7825 * locals.var_inv_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8650_e7833: f64 = (locals.var_stcf_i * locals.var_dt);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8650_e7833, ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4)), ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6)), ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7)), ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8)), ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8660_e7839: f64 = (locals.var_cf1_t + locals.var_temp);
            (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9, ) = (assign8660_e7839, (locals.var_cf1_t_dn4 + locals.var_temp_dn4), (locals.var_cf1_t_dn6 + locals.var_temp_dn6), (locals.var_cf1_t_dn7 + locals.var_temp_dn7), (locals.var_cf1_t_dn8 + locals.var_temp_dn8), (locals.var_cf1_t_dn9 + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8670_e7845: f64 = (locals.var_cf2_t + locals.var_temp);
            (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9, ) = (assign8670_e7845, (locals.var_cf2_t_dn4 + locals.var_temp_dn4), (locals.var_cf2_t_dn6 + locals.var_temp_dn6), (locals.var_cf2_t_dn7 + locals.var_temp_dn7), (locals.var_cf2_t_dn8 + locals.var_temp_dn8), (locals.var_cf2_t_dn9 + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8680_e7851: f64 = (locals.var_cfd_i * locals.var_inv_phit);
            (locals.var_xd0, locals.var_xd0_dn4, locals.var_xd0_dn6, locals.var_xd0_dn7, locals.var_xd0_dn8, locals.var_xd0_dn9, ) = (assign8680_e7851, (locals.var_cfd_i * locals.var_inv_phit_dn4), (locals.var_cfd_i * locals.var_inv_phit_dn6), (locals.var_cfd_i * locals.var_inv_phit_dn7), (locals.var_cfd_i * locals.var_inv_phit_dn8), (locals.var_cfd_i * locals.var_inv_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8690_e7857: f64 = (locals.var_cfac1_t + locals.var_temp);
            (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9, ) = (assign8690_e7857, (locals.var_cfac1_t_dn4 + locals.var_temp_dn4), (locals.var_cfac1_t_dn6 + locals.var_temp_dn6), (locals.var_cfac1_t_dn7 + locals.var_temp_dn7), (locals.var_cfac1_t_dn8 + locals.var_temp_dn8), (locals.var_cfac1_t_dn9 + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8700_e7863: f64 = (locals.var_cfac2_t + locals.var_temp);
            (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9, ) = (assign8700_e7863, (locals.var_cfac2_t_dn4 + locals.var_temp_dn4), (locals.var_cfac2_t_dn6 + locals.var_temp_dn6), (locals.var_cfac2_t_dn7 + locals.var_temp_dn7), (locals.var_cfac2_t_dn8 + locals.var_temp_dn8), (locals.var_cfac2_t_dn9 + locals.var_temp_dn9), );
        }

        let assign8710_e7868: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign8710_e7868;

        if ((locals.var_guard257 != 0.0) && (locals.var_guard259 != 0.0)) {
            let assign8720_e7875: f64 = (locals.var_np_i / locals.var_neff_poly);
            let assign8720_e7876: f64 = (assign8720_e7875).ln();
            let assign8720_e7878: f64 = (assign8720_e7876 + locals.var_eg_2phit0_woshe);
            let assign8720_e7879: f64 = (locals.var_phit0 * assign8720_e7878);
            (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9, ) = (assign8720_e7879, ((locals.var_phit0_dn4 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn4))), ((locals.var_phit0_dn6 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn6))), ((locals.var_phit0_dn7 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn7))), ((locals.var_phit0_dn8 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn8))), ((locals.var_phit0_dn9 * assign8720_e7878) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8720_e7875) + locals.var_eg_2phit0_woshe_dn9))), );
        }

        let assign8730_e7884: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign8730_e7884;

        if ((locals.var_guard257 != 0.0) && (locals.var_guard260 != 0.0)) {
            let assign8740_e7892: f64 = (2970.0 / locals.var_tkd);
            let assign8740_e7893: f64 = (15.0 + assign8740_e7892);
            let assign8740_e7897: f64 = (2970.0 / locals.var_tkd);
            let assign8740_e7898: f64 = (15.0 - assign8740_e7897);
            let assign8740_e7902: f64 = (2970.0 / locals.var_tkd);
            let assign8740_e7903: f64 = (15.0 - assign8740_e7902);
            let assign8740_e7904: f64 = (assign8740_e7898 * assign8740_e7903);
            let assign8740_e7906: f64 = (assign8740_e7904 + 1e-6);
            let assign8740_e7907: f64 = (assign8740_e7906).sqrt();
            let assign8740_e7908: f64 = (assign8740_e7893 + assign8740_e7907);
            let assign8740_e7909: f64 = (0.5 * assign8740_e7908);
            (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9, ) = (assign8740_e7909, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign8740_e7903) + (assign8740_e7898 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8740_e7907)))), );
        }

        if (locals.var_guard257 != 0.0) {
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign8760_e7918: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign8760_e7918;

        let assign8770_e7921: f64 = 1.0;
        let assign8770_e7922: f64 = if p.p14 == assign8770_e7921 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign8770_e7922;

        if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 != 0.0)) {
            let assign8780_e7930: f64 = (0.4 * p.p13);
            let assign8780_e7932: f64 = (assign8780_e7930 * 1.27520989);
            let assign8780_e7934: f64 = (-0.3333333333333);
            let assign8780_e7937: f64 = (locals.var_phit * locals.var_tsisq);
            let assign8780_e7938: f64 = (assign8780_e7937).ln();
            let assign8780_e7939: f64 = (assign8780_e7934 * assign8780_e7938);
            let assign8780_e7940: f64 = (assign8780_e7939).exp();
            let assign8780_e7941: f64 = (assign8780_e7932 * assign8780_e7940);
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (assign8780_e7941, (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8780_e7937)))), (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8780_e7937)))), (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8780_e7937)))), (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8780_e7937)))), (assign8780_e7932 * (assign8780_e7940 * (assign8780_e7934 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8780_e7937)))), );
        }

        if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 == 0.0)) {
            let assign8790_e7952: f64 = (0.4 * p.p13);
            let assign8790_e7954: f64 = (assign8790_e7952 * 1.5412087);
            let assign8790_e7956: f64 = (-0.3333333333333);
            let assign8790_e7959: f64 = (locals.var_phit * locals.var_tsisq);
            let assign8790_e7960: f64 = (assign8790_e7959).ln();
            let assign8790_e7961: f64 = (assign8790_e7956 * assign8790_e7960);
            let assign8790_e7962: f64 = (assign8790_e7961).exp();
            let assign8790_e7963: f64 = (assign8790_e7954 * assign8790_e7962);
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (assign8790_e7963, (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8790_e7959)))), (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8790_e7959)))), (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8790_e7959)))), (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8790_e7959)))), (assign8790_e7954 * (assign8790_e7962 * (assign8790_e7956 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8790_e7959)))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8800_e7969: f64 = (p.p14 * locals.var_stvfb_i);
            let assign8800_e7971: f64 = (assign8800_e7969 * locals.var_dt);
            let assign8800_e7973: f64 = (assign8800_e7971 + locals.var_dvfbqm);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8800_e7973, (assign8800_e7969 * locals.var_dt_dn4), (assign8800_e7969 * locals.var_dt_dn6), (assign8800_e7969 * locals.var_dt_dn7), (assign8800_e7969 * locals.var_dt_dn8), (assign8800_e7969 * locals.var_dt_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8810_e7979: f64 = (locals.var_temp + p.p34);
            let assign8810_e7981: f64 = (assign8810_e7979 - locals.var_dvfbpdep);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign8810_e7981, (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4), (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6), (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7), (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8), (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9), );
        }

    }

    pub(super) fn stamp_transient_block_9(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        if (locals.var_guard257 != 0.0) {
            let assign8820_e7988: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
            let assign8820_e7990: f64 = (assign8820_e7988 + locals.var_dvfb1nch);
            let assign8820_e7991: f64 = (p.p14 * assign8820_e7990);
            let assign8820_e7993: f64 = (assign8820_e7991 + locals.var_temp1);
            (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9, ) = (assign8820_e7993, ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8830_e8000: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
            let assign8830_e8002: f64 = (assign8830_e8000 + locals.var_dvfb2nch);
            let assign8830_e8003: f64 = (p.p14 * assign8830_e8002);
            let assign8830_e8005: f64 = (assign8830_e8003 + locals.var_temp);
            (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9, ) = (assign8830_e8005, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8840_e8012: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
            let assign8840_e8014: f64 = (assign8840_e8012 + locals.var_dvfb1nch);
            let assign8840_e8015: f64 = (p.p14 * assign8840_e8014);
            let assign8840_e8017: f64 = (assign8840_e8015 + locals.var_temp1);
            (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9, ) = (assign8840_e8017, ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8850_e8024: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
            let assign8850_e8026: f64 = (assign8850_e8024 + locals.var_dvfb2nch);
            let assign8850_e8027: f64 = (p.p14 * assign8850_e8026);
            let assign8850_e8029: f64 = (assign8850_e8027 + locals.var_temp);
            (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9, ) = (assign8850_e8029, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8860_e8034: f64 = (locals.var_rtn).ln();
            (locals.var_lnrtn, locals.var_lnrtn_dn4, locals.var_lnrtn_dn6, locals.var_lnrtn_dn7, locals.var_lnrtn_dn8, locals.var_lnrtn_dn9, ) = (assign8860_e8034, (locals.var_rtn_dn4 / locals.var_rtn), (locals.var_rtn_dn6 / locals.var_rtn), (locals.var_rtn_dn7 / locals.var_rtn), (locals.var_rtn_dn8 / locals.var_rtn), (locals.var_rtn_dn9 / locals.var_rtn), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8870_e8040: f64 = (locals.var_stbet_i * locals.var_lnrtn);
            let assign8870_e8041: f64 = (assign8870_e8040).exp();
            let assign8870_e8043: f64 = (assign8870_e8041 * p.p35);
            (locals.var_tf_bet, locals.var_tf_bet_dn4, locals.var_tf_bet_dn6, locals.var_tf_bet_dn7, locals.var_tf_bet_dn8, locals.var_tf_bet_dn9, ) = (assign8870_e8043, ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35), ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35), ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35), ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35), ((assign8870_e8041 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8880_e8049: f64 = (locals.var_betn1_t * locals.var_tf_bet);
            (locals.var_betn1_i, locals.var_betn1_i_dn4, locals.var_betn1_i_dn6, locals.var_betn1_i_dn7, locals.var_betn1_i_dn8, locals.var_betn1_i_dn9, ) = (assign8880_e8049, ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4)), ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6)), ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7)), ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8)), ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8890_e8055: f64 = (locals.var_betn2_t * locals.var_tf_bet);
            (locals.var_betn2_i, locals.var_betn2_i_dn4, locals.var_betn2_i_dn6, locals.var_betn2_i_dn7, locals.var_betn2_i_dn8, locals.var_betn2_i_dn9, ) = (assign8890_e8055, ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4)), ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6)), ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7)), ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8)), ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8900_e8061: f64 = (locals.var_stmue_i * locals.var_lnrtn);
            let assign8900_e8062: f64 = (assign8900_e8061).exp();
            (locals.var_tf_mue, locals.var_tf_mue_dn4, locals.var_tf_mue_dn6, locals.var_tf_mue_dn7, locals.var_tf_mue_dn8, locals.var_tf_mue_dn9, ) = (assign8900_e8062, (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn4)), (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn6)), (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn7)), (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn8)), (assign8900_e8062 * (locals.var_stmue_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8910_e8068: f64 = (locals.var_mue_t * locals.var_tf_mue);
            (locals.var_mue_i, locals.var_mue_i_dn4, locals.var_mue_i_dn6, locals.var_mue_i_dn7, locals.var_mue_i_dn8, locals.var_mue_i_dn9, ) = (assign8910_e8068, (locals.var_mue_t * locals.var_tf_mue_dn4), (locals.var_mue_t * locals.var_tf_mue_dn6), (locals.var_mue_t * locals.var_tf_mue_dn7), (locals.var_mue_t * locals.var_tf_mue_dn8), (locals.var_mue_t * locals.var_tf_mue_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8920_e8074: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
            let assign8920_e8075: f64 = (assign8920_e8074).exp();
            (locals.var_tf_themu, locals.var_tf_themu_dn4, locals.var_tf_themu_dn6, locals.var_tf_themu_dn7, locals.var_tf_themu_dn8, locals.var_tf_themu_dn9, ) = (assign8920_e8075, (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn4)), (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn6)), (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn7)), (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn8)), (assign8920_e8075 * (locals.var_stthemu_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8930_e8081: f64 = (locals.var_themu_t * locals.var_tf_themu);
            (locals.var_themu_i, locals.var_themu_i_dn4, locals.var_themu_i_dn6, locals.var_themu_i_dn7, locals.var_themu_i_dn8, locals.var_themu_i_dn9, ) = (assign8930_e8081, (locals.var_themu_t * locals.var_tf_themu_dn4), (locals.var_themu_t * locals.var_tf_themu_dn6), (locals.var_themu_t * locals.var_tf_themu_dn7), (locals.var_themu_t * locals.var_tf_themu_dn8), (locals.var_themu_t * locals.var_tf_themu_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8940_e8087: f64 = (locals.var_stcs_i * locals.var_lnrtn);
            let assign8940_e8088: f64 = (assign8940_e8087).exp();
            (locals.var_tf_cs, locals.var_tf_cs_dn4, locals.var_tf_cs_dn6, locals.var_tf_cs_dn7, locals.var_tf_cs_dn8, locals.var_tf_cs_dn9, ) = (assign8940_e8088, (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn4)), (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn6)), (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn7)), (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn8)), (assign8940_e8088 * (locals.var_stcs_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8950_e8094: f64 = (locals.var_cs_t * locals.var_tf_cs);
            (locals.var_cs_i, locals.var_cs_i_dn4, locals.var_cs_i_dn6, locals.var_cs_i_dn7, locals.var_cs_i_dn8, locals.var_cs_i_dn9, ) = (assign8950_e8094, (locals.var_cs_t * locals.var_tf_cs_dn4), (locals.var_cs_t * locals.var_tf_cs_dn6), (locals.var_cs_t * locals.var_tf_cs_dn7), (locals.var_cs_t * locals.var_tf_cs_dn8), (locals.var_cs_t * locals.var_tf_cs_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8960_e8100: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
            let assign8960_e8101: f64 = (assign8960_e8100).exp();
            (locals.var_tf_thecs, locals.var_tf_thecs_dn4, locals.var_tf_thecs_dn6, locals.var_tf_thecs_dn7, locals.var_tf_thecs_dn8, locals.var_tf_thecs_dn9, ) = (assign8960_e8101, (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn4)), (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn6)), (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn7)), (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn8)), (assign8960_e8101 * (locals.var_stthecs_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8970_e8107: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
            (locals.var_thecs_i, locals.var_thecs_i_dn4, locals.var_thecs_i_dn6, locals.var_thecs_i_dn7, locals.var_thecs_i_dn8, locals.var_thecs_i_dn9, ) = (assign8970_e8107, (locals.var_thecs_t * locals.var_tf_thecs_dn4), (locals.var_thecs_t * locals.var_tf_thecs_dn6), (locals.var_thecs_t * locals.var_tf_thecs_dn7), (locals.var_thecs_t * locals.var_tf_thecs_dn8), (locals.var_thecs_t * locals.var_tf_thecs_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8980_e8113: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
            let assign8980_e8114: f64 = (assign8980_e8113).exp();
            (locals.var_tf_xcor, locals.var_tf_xcor_dn4, locals.var_tf_xcor_dn6, locals.var_tf_xcor_dn7, locals.var_tf_xcor_dn8, locals.var_tf_xcor_dn9, ) = (assign8980_e8114, (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn4)), (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn6)), (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn7)), (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn8)), (assign8980_e8114 * (locals.var_stxcor_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8990_e8120: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
            (locals.var_xcor_i, locals.var_xcor_i_dn4, locals.var_xcor_i_dn6, locals.var_xcor_i_dn7, locals.var_xcor_i_dn8, locals.var_xcor_i_dn9, ) = (assign8990_e8120, (locals.var_xcor_t * locals.var_tf_xcor_dn4), (locals.var_xcor_t * locals.var_tf_xcor_dn6), (locals.var_xcor_t * locals.var_tf_xcor_dn7), (locals.var_xcor_t * locals.var_tf_xcor_dn8), (locals.var_xcor_t * locals.var_tf_xcor_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9000_e8126: f64 = (1e-8 * locals.var_phit);
            let assign9000_e8128: f64 = (assign9000_e8126 / locals.var_tsi_i);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign9000_e8128, ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9010_e8134: f64 = (locals.var_temp * locals.var_mue_i);
            (locals.var_fmue, locals.var_fmue_dn4, locals.var_fmue_dn6, locals.var_fmue_dn7, locals.var_fmue_dn8, locals.var_fmue_dn9, ) = (assign9010_e8134, ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4)), ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6)), ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7)), ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8)), ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9020_e8140: f64 = (locals.var_strs_i * locals.var_lnrtn);
            let assign9020_e8141: f64 = (assign9020_e8140).exp();
            (locals.var_tf_ther, locals.var_tf_ther_dn4, locals.var_tf_ther_dn6, locals.var_tf_ther_dn7, locals.var_tf_ther_dn8, locals.var_tf_ther_dn9, ) = (assign9020_e8141, (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn4)), (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn6)), (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn7)), (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn8)), (assign9020_e8141 * (locals.var_strs_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9030_e8147: f64 = (locals.var_rs_t * locals.var_tf_ther);
            (locals.var_rs_i, locals.var_rs_i_dn4, locals.var_rs_i_dn6, locals.var_rs_i_dn7, locals.var_rs_i_dn8, locals.var_rs_i_dn9, ) = (assign9030_e8147, (locals.var_rs_t * locals.var_tf_ther_dn4), (locals.var_rs_t * locals.var_tf_ther_dn6), (locals.var_rs_t * locals.var_tf_ther_dn7), (locals.var_rs_t * locals.var_tf_ther_dn8), (locals.var_rs_t * locals.var_tf_ther_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9040_e8153: f64 = (2.0 * locals.var_rs_i);
            let assign9040_e8155: f64 = (assign9040_e8153 * locals.var_phit);
            (locals.var_frs, locals.var_frs_dn4, locals.var_frs_dn6, locals.var_frs_dn7, locals.var_frs_dn8, locals.var_frs_dn9, ) = (assign9040_e8155, (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn4)), (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn6)), (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn7)), (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn8)), (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign9040_e8153 * locals.var_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9050_e8161: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
            let assign9050_e8162: f64 = (assign9050_e8161).exp();
            (locals.var_tf_thesat, locals.var_tf_thesat_dn4, locals.var_tf_thesat_dn6, locals.var_tf_thesat_dn7, locals.var_tf_thesat_dn8, locals.var_tf_thesat_dn9, ) = (assign9050_e8162, (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn4)), (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn6)), (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn7)), (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn8)), (assign9050_e8162 * (locals.var_stthesat_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9060_e8168: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
            let assign9060_e8170: f64 = (assign9060_e8168 * locals.var_tf_bet);
            (locals.var_thesat_i, locals.var_thesat_i_dn4, locals.var_thesat_i_dn6, locals.var_thesat_i_dn7, locals.var_thesat_i_dn8, locals.var_thesat_i_dn9, ) = (assign9060_e8170, ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn4)), ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn6)), ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn7)), ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn8)), ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign9060_e8168 * locals.var_tf_bet_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9070_e8176: f64 = (locals.var_thesat_i * locals.var_phit);
            (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9, ) = (assign9070_e8176, ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4)), ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6)), ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7)), ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8)), ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9080_e8182: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
            let assign9080_e8184: f64 = (assign9080_e8182 * locals.var_tf_bet);
            (locals.var_thesatac_i, locals.var_thesatac_i_dn4, locals.var_thesatac_i_dn6, locals.var_thesatac_i_dn7, locals.var_thesatac_i_dn8, locals.var_thesatac_i_dn9, ) = (assign9080_e8184, ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn4)), ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn6)), ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn7)), ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn8)), ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign9080_e8182 * locals.var_tf_bet_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9090_e8190: f64 = (locals.var_thesatac_i * locals.var_phit);
            (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9, ) = (assign9090_e8190, ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4)), ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6)), ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7)), ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8)), ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9100_e8196: f64 = (locals.var_alp1_i * locals.var_inv_phit);
            (locals.var_alp1_phit, locals.var_alp1_phit_dn4, locals.var_alp1_phit_dn6, locals.var_alp1_phit_dn7, locals.var_alp1_phit_dn8, locals.var_alp1_phit_dn9, ) = (assign9100_e8196, (locals.var_alp1_i * locals.var_inv_phit_dn4), (locals.var_alp1_i * locals.var_inv_phit_dn6), (locals.var_alp1_i * locals.var_inv_phit_dn7), (locals.var_alp1_i * locals.var_inv_phit_dn8), (locals.var_alp1_i * locals.var_inv_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9110_e8201: f64 = (-locals.var_stig_i);
            let assign9110_e8203: f64 = (assign9110_e8201 * locals.var_lnrtn);
            let assign9110_e8204: f64 = (assign9110_e8203).exp();
            (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9, ) = (assign9110_e8204, (assign9110_e8204 * (assign9110_e8201 * locals.var_lnrtn_dn4)), (assign9110_e8204 * (assign9110_e8201 * locals.var_lnrtn_dn6)), (assign9110_e8204 * (assign9110_e8201 * locals.var_lnrtn_dn7)), (assign9110_e8204 * (assign9110_e8201 * locals.var_lnrtn_dn8)), (assign9110_e8204 * (assign9110_e8201 * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9120_e8210: f64 = (locals.var_iginv_t * locals.var_tf_ig);
            (locals.var_iginv_i, locals.var_iginv_i_dn4, locals.var_iginv_i_dn6, locals.var_iginv_i_dn7, locals.var_iginv_i_dn8, locals.var_iginv_i_dn9, ) = (assign9120_e8210, (locals.var_iginv_t * locals.var_tf_ig_dn4), (locals.var_iginv_t * locals.var_tf_ig_dn6), (locals.var_iginv_t * locals.var_tf_ig_dn7), (locals.var_iginv_t * locals.var_tf_ig_dn8), (locals.var_iginv_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9130_e8216: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
            (locals.var_igovinv_i, locals.var_igovinv_i_dn4, locals.var_igovinv_i_dn6, locals.var_igovinv_i_dn7, locals.var_igovinv_i_dn8, locals.var_igovinv_i_dn9, ) = (assign9130_e8216, (locals.var_igovinv_t * locals.var_tf_ig_dn4), (locals.var_igovinv_t * locals.var_tf_ig_dn6), (locals.var_igovinv_t * locals.var_tf_ig_dn7), (locals.var_igovinv_t * locals.var_tf_ig_dn8), (locals.var_igovinv_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9140_e8222: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
            (locals.var_igovinvd_i, locals.var_igovinvd_i_dn4, locals.var_igovinvd_i_dn6, locals.var_igovinvd_i_dn7, locals.var_igovinvd_i_dn8, locals.var_igovinvd_i_dn9, ) = (assign9140_e8222, (locals.var_igovinvd_t * locals.var_tf_ig_dn4), (locals.var_igovinvd_t * locals.var_tf_ig_dn6), (locals.var_igovinvd_t * locals.var_tf_ig_dn7), (locals.var_igovinvd_t * locals.var_tf_ig_dn8), (locals.var_igovinvd_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9150_e8228: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
            (locals.var_igovacc_i, locals.var_igovacc_i_dn4, locals.var_igovacc_i_dn6, locals.var_igovacc_i_dn7, locals.var_igovacc_i_dn8, locals.var_igovacc_i_dn9, ) = (assign9150_e8228, (locals.var_igovacc_t * locals.var_tf_ig_dn4), (locals.var_igovacc_t * locals.var_tf_ig_dn6), (locals.var_igovacc_t * locals.var_tf_ig_dn7), (locals.var_igovacc_t * locals.var_tf_ig_dn8), (locals.var_igovacc_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9160_e8234: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
            (locals.var_igovaccd_i, locals.var_igovaccd_i_dn4, locals.var_igovaccd_i_dn6, locals.var_igovaccd_i_dn7, locals.var_igovaccd_i_dn8, locals.var_igovaccd_i_dn9, ) = (assign9160_e8234, (locals.var_igovaccd_t * locals.var_tf_ig_dn4), (locals.var_igovaccd_t * locals.var_tf_ig_dn6), (locals.var_igovaccd_t * locals.var_tf_ig_dn7), (locals.var_igovaccd_t * locals.var_tf_ig_dn8), (locals.var_igovaccd_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9170_e8239: f64 = (-locals.var_stigfn_i);
            let assign9170_e8241: f64 = (assign9170_e8239 * locals.var_lnrtn);
            let assign9170_e8242: f64 = (assign9170_e8241).exp();
            (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9, ) = (assign9170_e8242, (assign9170_e8242 * (assign9170_e8239 * locals.var_lnrtn_dn4)), (assign9170_e8242 * (assign9170_e8239 * locals.var_lnrtn_dn6)), (assign9170_e8242 * (assign9170_e8239 * locals.var_lnrtn_dn7)), (assign9170_e8242 * (assign9170_e8239 * locals.var_lnrtn_dn8)), (assign9170_e8242 * (assign9170_e8239 * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9180_e8248: f64 = (locals.var_fnovinv_t * locals.var_tf_ig);
            (locals.var_fnovinv_i, locals.var_fnovinv_i_dn4, locals.var_fnovinv_i_dn6, locals.var_fnovinv_i_dn7, locals.var_fnovinv_i_dn8, locals.var_fnovinv_i_dn9, ) = (assign9180_e8248, (locals.var_fnovinv_t * locals.var_tf_ig_dn4), (locals.var_fnovinv_t * locals.var_tf_ig_dn6), (locals.var_fnovinv_t * locals.var_tf_ig_dn7), (locals.var_fnovinv_t * locals.var_tf_ig_dn8), (locals.var_fnovinv_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9190_e8254: f64 = (locals.var_fnovinvd_t * locals.var_tf_ig);
            (locals.var_fnovinvd_i, locals.var_fnovinvd_i_dn4, locals.var_fnovinvd_i_dn6, locals.var_fnovinvd_i_dn7, locals.var_fnovinvd_i_dn8, locals.var_fnovinvd_i_dn9, ) = (assign9190_e8254, (locals.var_fnovinvd_t * locals.var_tf_ig_dn4), (locals.var_fnovinvd_t * locals.var_tf_ig_dn6), (locals.var_fnovinvd_t * locals.var_tf_ig_dn7), (locals.var_fnovinvd_t * locals.var_tf_ig_dn8), (locals.var_fnovinvd_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9200_e8260: f64 = (0.5 * locals.var_eg);
            (locals.var_alpha_b, locals.var_alpha_b_dn4, locals.var_alpha_b_dn6, locals.var_alpha_b_dn7, locals.var_alpha_b_dn8, locals.var_alpha_b_dn9, ) = (assign9200_e8260, (0.5 * locals.var_eg_dn4), (0.5 * locals.var_eg_dn6), (0.5 * locals.var_eg_dn7), (0.5 * locals.var_eg_dn8), (0.5 * locals.var_eg_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9210_e8266: f64 = (locals.var_gco_i * locals.var_phit);
            (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9, ) = (assign9210_e8266, (locals.var_gco_i * locals.var_phit_dn4), (locals.var_gco_i * locals.var_phit_dn6), (locals.var_gco_i * locals.var_phit_dn7), (locals.var_gco_i * locals.var_phit_dn8), (locals.var_gco_i * locals.var_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9220_e8272: f64 = (locals.var_gco_i * locals.var_phit0);
            (locals.var_dov, locals.var_dov_dn4, locals.var_dov_dn6, locals.var_dov_dn7, locals.var_dov_dn8, locals.var_dov_dn9, ) = (assign9220_e8272, (locals.var_gco_i * locals.var_phit0_dn4), (locals.var_gco_i * locals.var_phit0_dn6), (locals.var_gco_i * locals.var_phit0_dn7), (locals.var_gco_i * locals.var_phit0_dn8), (locals.var_gco_i * locals.var_phit0_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9230_e8280: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
            let assign9230_e8281: f64 = (1.0 + assign9230_e8280);
            let assign9230_e8282: f64 = (1.0 / assign9230_e8281);
            (locals.var_n_iginv, locals.var_n_iginv_dn4, locals.var_n_iginv_dn6, locals.var_n_iginv_dn7, locals.var_n_iginv_dn8, locals.var_n_iginv_dn9, ) = (assign9230_e8282, (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign9230_e8281 * assign9230_e8281))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign9230_e8281 * assign9230_e8281))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign9230_e8281 * assign9230_e8281))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign9230_e8281 * assign9230_e8281))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign9230_e8281 * assign9230_e8281))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9240_e8288: f64 = (locals.var_toxp_i * 500000000.0);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign9240_e8288, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9250_e8296: f64 = (locals.var_stbgidl_i * locals.var_dt);
            let assign9250_e8297: f64 = (1.0 + assign9250_e8296);
            let assign9250_e8299: f64 = assign9250_e8297;
            let assign9250_e8303: f64 = (locals.var_stbgidl_i * locals.var_dt);
            let assign9250_e8304: f64 = (1.0 + assign9250_e8303);
            let assign9250_e8306: f64 = assign9250_e8304;
            let assign9250_e8310: f64 = (locals.var_stbgidl_i * locals.var_dt);
            let assign9250_e8311: f64 = (1.0 + assign9250_e8310);
            let assign9250_e8313: f64 = assign9250_e8311;
            let assign9250_e8314: f64 = (assign9250_e8306 * assign9250_e8313);
            let assign9250_e8316: f64 = (assign9250_e8314 + 0.01);
            let assign9250_e8317: f64 = (assign9250_e8316).sqrt();
            let assign9250_e8318: f64 = (assign9250_e8299 + assign9250_e8317);
            let assign9250_e8319: f64 = (0.5 * assign9250_e8318);
            (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign9250_e8319, (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign9250_e8317)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign9250_e8317)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign9250_e8317)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign9250_e8317)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign9250_e8313) + (assign9250_e8306 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign9250_e8317)))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9260_e8325: f64 = (locals.var_bgidl_t * locals.var_tempm);
            let assign9260_e8327: f64 = (assign9260_e8325 * locals.var_temp);
            (locals.var_bgidl_i, locals.var_bgidl_i_dn4, locals.var_bgidl_i_dn6, locals.var_bgidl_i_dn7, locals.var_bgidl_i_dn8, locals.var_bgidl_i_dn9, ) = (assign9260_e8327, (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn4)), (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn6)), (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn7)), (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn8)), (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9260_e8325 * locals.var_temp_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9270_e8335: f64 = (locals.var_stbgidld_i * locals.var_dt);
            let assign9270_e8336: f64 = (1.0 + assign9270_e8335);
            let assign9270_e8338: f64 = assign9270_e8336;
            let assign9270_e8342: f64 = (locals.var_stbgidld_i * locals.var_dt);
            let assign9270_e8343: f64 = (1.0 + assign9270_e8342);
            let assign9270_e8345: f64 = assign9270_e8343;
            let assign9270_e8349: f64 = (locals.var_stbgidld_i * locals.var_dt);
            let assign9270_e8350: f64 = (1.0 + assign9270_e8349);
            let assign9270_e8352: f64 = assign9270_e8350;
            let assign9270_e8353: f64 = (assign9270_e8345 * assign9270_e8352);
            let assign9270_e8355: f64 = (assign9270_e8353 + 0.01);
            let assign9270_e8356: f64 = (assign9270_e8355).sqrt();
            let assign9270_e8357: f64 = (assign9270_e8338 + assign9270_e8356);
            let assign9270_e8358: f64 = (0.5 * assign9270_e8357);
            (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign9270_e8358, (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign9270_e8356)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign9270_e8356)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign9270_e8356)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign9270_e8356)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign9270_e8352) + (assign9270_e8345 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign9270_e8356)))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9280_e8364: f64 = (locals.var_bgidld_t * locals.var_tempm);
            let assign9280_e8366: f64 = (assign9280_e8364 * locals.var_temp);
            (locals.var_bgidld_i, locals.var_bgidld_i_dn4, locals.var_bgidld_i_dn6, locals.var_bgidld_i_dn7, locals.var_bgidld_i_dn8, locals.var_bgidld_i_dn9, ) = (assign9280_e8366, (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn4)), (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn6)), (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn7)), (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn8)), (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9280_e8364 * locals.var_temp_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9290_e8372: f64 = (-locals.var_sta2_i);
            let assign9290_e8374: f64 = (assign9290_e8372 * locals.var_lnrtn);
            let assign9290_e8375: f64 = (assign9290_e8374).exp();
            let assign9290_e8376: f64 = (locals.var_a2_t * assign9290_e8375);
            (locals.var_a2_i, locals.var_a2_i_dn4, locals.var_a2_i_dn6, locals.var_a2_i_dn7, locals.var_a2_i_dn8, locals.var_a2_i_dn9, ) = (assign9290_e8376, (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn4))), (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn6))), (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn7))), (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn8))), (locals.var_a2_t * (assign9290_e8375 * (assign9290_e8372 * locals.var_lnrtn_dn9))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9300_e8382: f64 = (locals.var_areaq_i * locals.var_phit);
            (locals.var_area_phit, locals.var_area_phit_dn4, locals.var_area_phit_dn6, locals.var_area_phit_dn7, locals.var_area_phit_dn8, locals.var_area_phit_dn9, ) = (assign9300_e8382, (locals.var_areaq_i * locals.var_phit_dn4), (locals.var_areaq_i * locals.var_phit_dn6), (locals.var_areaq_i * locals.var_phit_dn7), (locals.var_areaq_i * locals.var_phit_dn8), (locals.var_areaq_i * locals.var_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9310_e8388: f64 = (0.25 * 1.602176565e-19);
            let assign9310_e8390: f64 = (assign9310_e8388 * locals.var_nsdac_i);
            let assign9310_e8393: f64 = (locals.var_epsch * locals.var_phit);
            let assign9310_e8394: f64 = (assign9310_e8390 / assign9310_e8393);
            (locals.var_inner_sd, locals.var_inner_sd_dn4, locals.var_inner_sd_dn6, locals.var_inner_sd_dn7, locals.var_inner_sd_dn8, locals.var_inner_sd_dn9, ) = (assign9310_e8394, (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn4)) / (assign9310_e8393 * assign9310_e8393))), (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn6)) / (assign9310_e8393 * assign9310_e8393))), (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn7)) / (assign9310_e8393 * assign9310_e8393))), (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn8)) / (assign9310_e8393 * assign9310_e8393))), (-((assign9310_e8390 * (locals.var_epsch * locals.var_phit_dn9)) / (assign9310_e8393 * assign9310_e8393))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9320_e8400: f64 = (locals.var_nsdac_i / locals.var_neff);
            let assign9320_e8401: f64 = (assign9320_e8400).ln();
            (locals.var_xsd, locals.var_xsd_dn4, locals.var_xsd_dn6, locals.var_xsd_dn7, locals.var_xsd_dn8, locals.var_xsd_dn9, ) = (assign9320_e8401, ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign9320_e8400), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9330_e8407: f64 = (locals.var_fif_i * 1.25e-6);
            let assign9330_e8409: f64 = (assign9330_e8407 * locals.var_phit);
            (locals.var_fif_phit, locals.var_fif_phit_dn4, locals.var_fif_phit_dn6, locals.var_fif_phit_dn7, locals.var_fif_phit_dn8, locals.var_fif_phit_dn9, ) = (assign9330_e8409, (assign9330_e8407 * locals.var_phit_dn4), (assign9330_e8407 * locals.var_phit_dn6), (assign9330_e8407 * locals.var_phit_dn7), (assign9330_e8407 * locals.var_phit_dn8), (assign9330_e8407 * locals.var_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9340_e8415: f64 = (locals.var_strth_i * locals.var_lnrtn);
            let assign9340_e8416: f64 = (assign9340_e8415).exp();
            (locals.var_tf_rth, locals.var_tf_rth_dn4, locals.var_tf_rth_dn6, locals.var_tf_rth_dn7, locals.var_tf_rth_dn8, locals.var_tf_rth_dn9, ) = (assign9340_e8416, (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn4)), (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn6)), (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn7)), (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn8)), (assign9340_e8416 * (locals.var_strth_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9350_e8422: f64 = (locals.var_rth_t * locals.var_tf_rth);
            (locals.var_rth_i, locals.var_rth_i_dn4, locals.var_rth_i_dn6, locals.var_rth_i_dn7, locals.var_rth_i_dn8, locals.var_rth_i_dn9, ) = (assign9350_e8422, ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4)), ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6)), ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7)), ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8)), ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9360_e8428: f64 = (4.0 * 1.3806488e-23);
            let assign9360_e8430: f64 = (assign9360_e8428 * locals.var_tkc);
            locals.var_nt0_4kt = assign9360_e8430;
        }

        if (locals.var_guard257 != 0.0) {
            let assign9370_e8436: f64 = (locals.var_fnt_i * locals.var_nt0_4kt);
            locals.var_nt = assign9370_e8436;
        }

        let assign9380_e8441: f64 = 1.0;
        let assign9380_e8442: f64 = if p.p14 == assign9380_e8441 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign9380_e8442;

        if (locals.var_guard263 != 0.0) {
            (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9, ) = ((nv9 - nv6), -1.0, 1.0, );
            (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7, ) = ((nv7 - nv6), -1.0, 1.0, );
            (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8, ) = ((nv6 - nv8), 1.0, -1.0, );
        }

        if (locals.var_guard263 == 0.0) {
            let assign9420_e8458: f64 = (-(nv9 - nv6));
            (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9, ) = (assign9420_e8458, 1.0, (-1.0), );
        }

        if (locals.var_guard263 == 0.0) {
            let assign9430_e8464: f64 = (-(nv7 - nv6));
            (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7, ) = (assign9430_e8464, 1.0, (-1.0), );
        }

        if (locals.var_guard263 == 0.0) {
            let assign9440_e8470: f64 = (-(nv6 - nv8));
            (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8, ) = (assign9440_e8470, (-1.0), 1.0, );
        }

        let assign9450_e8474: f64 = (-locals.var_vdsu);
        (locals.var_vsdu, locals.var_vsdu_dn6, locals.var_vsdu_dn7, ) = (assign9450_e8474, (-locals.var_vdsu_dn6), (-locals.var_vdsu_dn7), );

        let assign9460_e8477: f64 = (locals.var_vgsu + locals.var_vsdu);
        (locals.var_vgdu, locals.var_vgdu_dn6, locals.var_vgdu_dn7, locals.var_vgdu_dn9, ) = (assign9460_e8477, (locals.var_vgsu_dn6 + locals.var_vsdu_dn6), locals.var_vsdu_dn7, locals.var_vgsu_dn9, );

        let assign9470_e8480: f64 = (locals.var_vdsu + locals.var_vsbu);
        (locals.var_vdbu, locals.var_vdbu_dn6, locals.var_vdbu_dn7, locals.var_vdbu_dn8, ) = (assign9470_e8480, (locals.var_vdsu_dn6 + locals.var_vsbu_dn6), locals.var_vdsu_dn7, locals.var_vsbu_dn8, );

        let assign9480_e8483: f64 = if locals.var_vdsu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign9480_e8483;

        if (locals.var_guard264 != 0.0) {
            let assign9490_e8486: f64 = (-1.0);
            locals.var_sigvds = assign9490_e8486;
        }

        if (locals.var_guard264 != 0.0) {
            (locals.var_vds, locals.var_vds_dn6, locals.var_vds_dn7, ) = (locals.var_vsdu, locals.var_vsdu_dn6, locals.var_vsdu_dn7, );
            (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn9, ) = (locals.var_vgdu, locals.var_vgdu_dn6, locals.var_vgdu_dn7, locals.var_vgdu_dn9, );
            (locals.var_vsb, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn8, ) = (locals.var_vdbu, locals.var_vdbu_dn6, locals.var_vdbu_dn7, locals.var_vdbu_dn8, );
        }

        if (locals.var_guard264 == 0.0) {
            locals.var_sigvds = 1.0;
            (locals.var_vds, locals.var_vds_dn6, locals.var_vds_dn7, ) = (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7, );
            (locals.var_vgs, locals.var_vgs_dn6, locals.var_vgs_dn7, locals.var_vgs_dn9, ) = (locals.var_vgsu, locals.var_vgsu_dn6, 0.0, locals.var_vgsu_dn9, );
            (locals.var_vsb, locals.var_vsb_dn6, locals.var_vsb_dn7, locals.var_vsb_dn8, ) = (locals.var_vsbu, locals.var_vsbu_dn6, 0.0, locals.var_vsbu_dn8, );
        }

        let assign9570_e8523: f64 = (locals.var_vgs + locals.var_vsb);
        (locals.var_vgb, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9, ) = (assign9570_e8523, (locals.var_vgs_dn6 + locals.var_vsb_dn6), (locals.var_vgs_dn7 + locals.var_vsb_dn7), locals.var_vsb_dn8, locals.var_vgs_dn9, );

        let assign9580_e8526: f64 = (locals.var_vds * locals.var_inv_phit);
        (locals.var_xd, locals.var_xd_dn4, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, ) = (assign9580_e8526, (locals.var_vds * locals.var_inv_phit_dn4), ((locals.var_vds_dn6 * locals.var_inv_phit) + (locals.var_vds * locals.var_inv_phit_dn6)), ((locals.var_vds_dn7 * locals.var_inv_phit) + (locals.var_vds * locals.var_inv_phit_dn7)), (locals.var_vds * locals.var_inv_phit_dn8), (locals.var_vds * locals.var_inv_phit_dn9), );

        let assign9590_e8529: f64 = (locals.var_vds * locals.var_vds);
        let assign9590_e8531: f64 = (assign9590_e8529 + 0.01);
        let assign9590_e8532: f64 = (assign9590_e8531).sqrt();
        let assign9590_e8534: f64 = (assign9590_e8532 - 0.1);
        let assign9590_e8536: f64 = (assign9590_e8534 * locals.var_inv_phit);
        (locals.var_xdsx, locals.var_xdsx_dn4, locals.var_xdsx_dn6, locals.var_xdsx_dn7, locals.var_xdsx_dn8, locals.var_xdsx_dn9, ) = (assign9590_e8536, (assign9590_e8534 * locals.var_inv_phit_dn4), (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign9590_e8532)) * locals.var_inv_phit) + (assign9590_e8534 * locals.var_inv_phit_dn6)), (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign9590_e8532)) * locals.var_inv_phit) + (assign9590_e8534 * locals.var_inv_phit_dn7)), (assign9590_e8534 * locals.var_inv_phit_dn8), (assign9590_e8534 * locals.var_inv_phit_dn9), );

        let assign9600_e8540: f64 = (locals.var_xd - locals.var_xdsx);
        let assign9600_e8541: f64 = (0.5 * assign9600_e8540);
        (locals.var_dxdsx, locals.var_dxdsx_dn4, locals.var_dxdsx_dn6, locals.var_dxdsx_dn7, locals.var_dxdsx_dn8, locals.var_dxdsx_dn9, ) = (assign9600_e8541, (0.5 * (locals.var_xd_dn4 - locals.var_xdsx_dn4)), (0.5 * (locals.var_xd_dn6 - locals.var_xdsx_dn6)), (0.5 * (locals.var_xd_dn7 - locals.var_xdsx_dn7)), (0.5 * (locals.var_xd_dn8 - locals.var_xdsx_dn8)), (0.5 * (locals.var_xd_dn9 - locals.var_xdsx_dn9)), );

        (locals.var_vfb1_loc, locals.var_vfb1_loc_dn4, locals.var_vfb1_loc_dn6, locals.var_vfb1_loc_dn7, locals.var_vfb1_loc_dn8, locals.var_vfb1_loc_dn9, ) = (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9, );

        (locals.var_vfb2_loc, locals.var_vfb2_loc_dn4, locals.var_vfb2_loc_dn6, locals.var_vfb2_loc_dn7, locals.var_vfb2_loc_dn8, locals.var_vfb2_loc_dn9, ) = (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9, );

        locals.var_psce1_loc = locals.var_psce1_i;

        locals.var_psce2_loc = locals.var_psce2_i;

        (locals.var_cf1_loc, locals.var_cf1_loc_dn4, locals.var_cf1_loc_dn6, locals.var_cf1_loc_dn7, locals.var_cf1_loc_dn8, locals.var_cf1_loc_dn9, ) = (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9, );

        (locals.var_cf2_loc, locals.var_cf2_loc_dn4, locals.var_cf2_loc_dn6, locals.var_cf2_loc_dn7, locals.var_cf2_loc_dn8, locals.var_cf2_loc_dn9, ) = (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9, );

        (locals.var_sat_phit_loc, locals.var_sat_phit_loc_dn4, locals.var_sat_phit_loc_dn6, locals.var_sat_phit_loc_dn7, locals.var_sat_phit_loc_dn8, locals.var_sat_phit_loc_dn9, ) = (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9, );

        locals.var_gamax_loc = locals.var_gamax;

        locals.var_alp_loc = locals.var_alp_i;

        let assign9700_e8553: f64 = (locals.var_vgs - locals.var_vfb1_loc);
        let assign9700_e8555: f64 = (assign9700_e8553 * locals.var_inv_phit);
        let assign9700_e8557: f64 = (assign9700_e8555 - locals.var_dxdsx);
        let assign9700_e8559: f64 = (assign9700_e8557 - locals.var_eg_2phit0);
        (locals.var_xg10, locals.var_xg10_dn4, locals.var_xg10_dn6, locals.var_xg10_dn7, locals.var_xg10_dn8, locals.var_xg10_dn9, ) = (assign9700_e8559, (((((-locals.var_vfb1_loc_dn4) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4) - locals.var_eg_2phit0_dn4), (((((locals.var_vgs_dn6 - locals.var_vfb1_loc_dn6) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6) - locals.var_eg_2phit0_dn6), (((((locals.var_vgs_dn7 - locals.var_vfb1_loc_dn7) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7) - locals.var_eg_2phit0_dn7), (((((-locals.var_vfb1_loc_dn8) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8) - locals.var_eg_2phit0_dn8), (((((locals.var_vgs_dn9 - locals.var_vfb1_loc_dn9) * locals.var_inv_phit) + (assign9700_e8553 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9) - locals.var_eg_2phit0_dn9), );

        let assign9710_e8561: f64 = (-locals.var_vsb);
        let assign9710_e8563: f64 = (assign9710_e8561 - locals.var_vfb2_loc);
        let assign9710_e8565: f64 = (assign9710_e8563 * locals.var_inv_phit);
        let assign9710_e8567: f64 = (assign9710_e8565 - locals.var_dxdsx);
        (locals.var_xg20shift, locals.var_xg20shift_dn4, locals.var_xg20shift_dn6, locals.var_xg20shift_dn7, locals.var_xg20shift_dn8, locals.var_xg20shift_dn9, ) = (assign9710_e8567, ((((-locals.var_vfb2_loc_dn4) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_loc_dn6) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_loc_dn7) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_loc_dn8) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8), ((((-locals.var_vfb2_loc_dn9) * locals.var_inv_phit) + (assign9710_e8563 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9), );

        let assign9720_e8570: f64 = (locals.var_xg20shift - locals.var_eg_2phit0);
        (locals.var_xg20, locals.var_xg20_dn4, locals.var_xg20_dn6, locals.var_xg20_dn7, locals.var_xg20_dn8, locals.var_xg20_dn9, ) = (assign9720_e8570, (locals.var_xg20shift_dn4 - locals.var_eg_2phit0_dn4), (locals.var_xg20shift_dn6 - locals.var_eg_2phit0_dn6), (locals.var_xg20shift_dn7 - locals.var_eg_2phit0_dn7), (locals.var_xg20shift_dn8 - locals.var_eg_2phit0_dn8), (locals.var_xg20shift_dn9 - locals.var_eg_2phit0_dn9), );

        let assign9730_e8573: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign9730_e8573;

        if (locals.var_guard531 != 0.0) {
            let assign9740_e8577: f64 = (p.p14 * locals.var_typesub_i);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign9740_e8577, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9750_e8583: f64 = (1.0 + locals.var_k1_1d);
            let assign9750_e8586: f64 = (1.0 + locals.var_k2_1d);
            let assign9750_e8587: f64 = (assign9750_e8583 / assign9750_e8586);
            (locals.var_exp_dxth, locals.var_exp_dxth_dn4, locals.var_exp_dxth_dn6, locals.var_exp_dxth_dn7, locals.var_exp_dxth_dn8, locals.var_exp_dxth_dn9, ) = (assign9750_e8587, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9760_e8592: f64 = (locals.var_exp_dxth).ln();
            (locals.var_dxth, locals.var_dxth_dn4, locals.var_dxth_dn6, locals.var_dxth_dn7, locals.var_dxth_dn8, locals.var_dxth_dn9, ) = (assign9760_e8592, (locals.var_exp_dxth_dn4 / locals.var_exp_dxth), (locals.var_exp_dxth_dn6 / locals.var_exp_dxth), (locals.var_exp_dxth_dn7 / locals.var_exp_dxth), (locals.var_exp_dxth_dn8 / locals.var_exp_dxth), (locals.var_exp_dxth_dn9 / locals.var_exp_dxth), );
        }

        let assign9770_e8597: f64 = if locals.var_dxth > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign9770_e8597;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard532 != 0.0)) {
            let assign9780_e8603: f64 = (2.0 * locals.var_dxth);
            let assign9780_e8606: f64 = (locals.var_exp_dxth + 1.0);
            let assign9780_e8607: f64 = (assign9780_e8603 * assign9780_e8606);
            let assign9780_e8610: f64 = (locals.var_exp_dxth - 1.0);
            let assign9780_e8611: f64 = (assign9780_e8607 / assign9780_e8610);
            (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9, ) = (assign9780_e8611, ((((((2.0 * locals.var_dxth_dn4) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn4)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn4)) / (assign9780_e8610 * assign9780_e8610)), ((((((2.0 * locals.var_dxth_dn6) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn6)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn6)) / (assign9780_e8610 * assign9780_e8610)), ((((((2.0 * locals.var_dxth_dn7) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn7)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn7)) / (assign9780_e8610 * assign9780_e8610)), ((((((2.0 * locals.var_dxth_dn8) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn8)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn8)) / (assign9780_e8610 * assign9780_e8610)), ((((((2.0 * locals.var_dxth_dn9) * assign9780_e8606) + (assign9780_e8603 * locals.var_exp_dxth_dn9)) * assign9780_e8610) - (assign9780_e8607 * locals.var_exp_dxth_dn9)) / (assign9780_e8610 * assign9780_e8610)), );
        }

    }

    pub(super) fn stamp_transient_block_10(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard531 != 0.0) && (locals.var_guard532 == 0.0)) {
            let assign9790_e8621: f64 = (2.0 + locals.var_dxth);
            let assign9790_e8622: f64 = (2.0 * assign9790_e8621);
            (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9, ) = (assign9790_e8622, (2.0 * locals.var_dxth_dn4), (2.0 * locals.var_dxth_dn6), (2.0 * locals.var_dxth_dn7), (2.0 * locals.var_dxth_dn8), (2.0 * locals.var_dxth_dn9), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9800_e8629: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
            let assign9800_e8630: f64 = (locals.var_a0_csisq / assign9800_e8629);
            (locals.var_a0, locals.var_a0_dn4, locals.var_a0_dn6, locals.var_a0_dn7, locals.var_a0_dn8, locals.var_a0_dn9, ) = (assign9800_e8630, (locals.var_a0_csisq_dn4 / assign9800_e8629), (locals.var_a0_csisq_dn6 / assign9800_e8629), (locals.var_a0_csisq_dn7 / assign9800_e8629), (locals.var_a0_csisq_dn8 / assign9800_e8629), (locals.var_a0_csisq_dn9 / assign9800_e8629), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9810_e8636: f64 = (1.0 / locals.var_k1_1d);
            (locals.var_inv_k1, locals.var_inv_k1_dn4, locals.var_inv_k1_dn6, locals.var_inv_k1_dn7, locals.var_inv_k1_dn8, locals.var_inv_k1_dn9, ) = (assign9810_e8636, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9820_e8642: f64 = (1.0 / locals.var_k2_1d);
            (locals.var_inv_k2, locals.var_inv_k2_dn4, locals.var_inv_k2_dn6, locals.var_inv_k2_dn7, locals.var_inv_k2_dn8, locals.var_inv_k2_dn9, ) = (assign9820_e8642, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9830_e8649: f64 = (1.0 + locals.var_inv_k1);
            let assign9830_e8651: f64 = (assign9830_e8649 + locals.var_inv_k2);
            let assign9830_e8652: f64 = (1.0 / assign9830_e8651);
            (locals.var_keq, locals.var_keq_dn4, locals.var_keq_dn6, locals.var_keq_dn7, locals.var_keq_dn8, locals.var_keq_dn9, ) = (assign9830_e8652, (-((locals.var_inv_k1_dn4 + locals.var_inv_k2_dn4) / (assign9830_e8651 * assign9830_e8651))), (-((locals.var_inv_k1_dn6 + locals.var_inv_k2_dn6) / (assign9830_e8651 * assign9830_e8651))), (-((locals.var_inv_k1_dn7 + locals.var_inv_k2_dn7) / (assign9830_e8651 * assign9830_e8651))), (-((locals.var_inv_k1_dn8 + locals.var_inv_k2_dn8) / (assign9830_e8651 * assign9830_e8651))), (-((locals.var_inv_k1_dn9 + locals.var_inv_k2_dn9) / (assign9830_e8651 * assign9830_e8651))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9840_e8659: f64 = (locals.var_xg10 - locals.var_xg20);
            let assign9840_e8660: f64 = (locals.var_keq * assign9840_e8659);
            (locals.var_dx_wi, locals.var_dx_wi_dn4, locals.var_dx_wi_dn6, locals.var_dx_wi_dn7, locals.var_dx_wi_dn8, locals.var_dx_wi_dn9, ) = (assign9840_e8660, ((locals.var_keq_dn4 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn4 - locals.var_xg20_dn4))), ((locals.var_keq_dn6 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn6 - locals.var_xg20_dn6))), ((locals.var_keq_dn7 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn7 - locals.var_xg20_dn7))), ((locals.var_keq_dn8 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn8 - locals.var_xg20_dn8))), ((locals.var_keq_dn9 * assign9840_e8659) + (locals.var_keq * (locals.var_xg10_dn9 - locals.var_xg20_dn9))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9850_e8667: f64 = (locals.var_dx_wi * locals.var_inv_k1);
            let assign9850_e8668: f64 = (locals.var_xg10 - assign9850_e8667);
            (locals.var_x1_wi0, locals.var_x1_wi0_dn4, locals.var_x1_wi0_dn6, locals.var_x1_wi0_dn7, locals.var_x1_wi0_dn8, locals.var_x1_wi0_dn9, ) = (assign9850_e8668, (locals.var_xg10_dn4 - ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4))), (locals.var_xg10_dn6 - ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6))), (locals.var_xg10_dn7 - ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7))), (locals.var_xg10_dn8 - ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8))), (locals.var_xg10_dn9 - ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9860_e8675: f64 = (locals.var_dx_wi * locals.var_inv_k2);
            let assign9860_e8676: f64 = (locals.var_xg20 + assign9860_e8675);
            (locals.var_x2_wi0, locals.var_x2_wi0_dn4, locals.var_x2_wi0_dn6, locals.var_x2_wi0_dn7, locals.var_x2_wi0_dn8, locals.var_x2_wi0_dn9, ) = (assign9860_e8676, (locals.var_xg20_dn4 + ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4))), (locals.var_xg20_dn6 + ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6))), (locals.var_xg20_dn7 + ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7))), (locals.var_xg20_dn8 + ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8))), (locals.var_xg20_dn9 + ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9870_e8683: f64 = (locals.var_k1_1d + 1.0);
            let assign9870_e8684: f64 = (1.0 / assign9870_e8683);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign9870_e8684, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9880_e8691: f64 = (locals.var_k2_1d + 1.0);
            let assign9880_e8692: f64 = (1.0 / assign9880_e8691);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign9880_e8692, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9890_e8699: f64 = (locals.var_k2_1d * locals.var_q_temp2);
            let assign9890_e8700: f64 = (locals.var_k1_1d + assign9890_e8699);
            let assign9890_e8702: f64 = (assign9890_e8700 * locals.var_diff_min);
            let assign9890_e8704: f64 = (assign9890_e8702 / locals.var_a0);
            let assign9890_e8705: f64 = (assign9890_e8704).ln();
            let assign9890_e8707: f64 = assign9890_e8705;
            let assign9890_e8709: f64 = (assign9890_e8707 + 1.5);
            (locals.var_q_x1sat, locals.var_q_x1sat_dn4, locals.var_q_x1sat_dn6, locals.var_q_x1sat_dn7, locals.var_q_x1sat_dn8, locals.var_q_x1sat_dn9, ) = (assign9890_e8709, (((((((locals.var_k2_1d * locals.var_q_temp2_dn4) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), (((((((locals.var_k2_1d * locals.var_q_temp2_dn6) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), (((((((locals.var_k2_1d * locals.var_q_temp2_dn7) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), (((((((locals.var_k2_1d * locals.var_q_temp2_dn8) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), (((((((locals.var_k2_1d * locals.var_q_temp2_dn9) * locals.var_diff_min) + (assign9890_e8700 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign9890_e8702 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign9890_e8704), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9900_e8716: f64 = (locals.var_k1_1d * locals.var_q_temp1);
            let assign9900_e8717: f64 = (locals.var_k2_1d + assign9900_e8716);
            let assign9900_e8719: f64 = (assign9900_e8717 * locals.var_diff_min);
            let assign9900_e8721: f64 = (assign9900_e8719 / locals.var_a0);
            let assign9900_e8722: f64 = (assign9900_e8721).ln();
            let assign9900_e8724: f64 = assign9900_e8722;
            let assign9900_e8726: f64 = (assign9900_e8724 + 1.5);
            (locals.var_q_x2sat, locals.var_q_x2sat_dn4, locals.var_q_x2sat_dn6, locals.var_q_x2sat_dn7, locals.var_q_x2sat_dn8, locals.var_q_x2sat_dn9, ) = (assign9900_e8726, (((((((locals.var_k1_1d * locals.var_q_temp1_dn4) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), (((((((locals.var_k1_1d * locals.var_q_temp1_dn6) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), (((((((locals.var_k1_1d * locals.var_q_temp1_dn7) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), (((((((locals.var_k1_1d * locals.var_q_temp1_dn8) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), (((((((locals.var_k1_1d * locals.var_q_temp1_dn9) * locals.var_diff_min) + (assign9900_e8717 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign9900_e8719 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign9900_e8721), );
        }

        let assign9910_e8731: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign9910_e8733: f64 = (assign9910_e8731 / 1.5);
        let assign9910_e8735: f64 = if assign9910_e8733 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard533 = assign9910_e8735;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard533 != 0.0)) {
            let assign9920_e8742: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
            let assign9920_e8744: f64 = (assign9920_e8742 / 1.5);
            let assign9920_e8745: f64 = (assign9920_e8744).exp();
            let assign9920_e8746: f64 = (1.0 + assign9920_e8745);
            let assign9920_e8747: f64 = (assign9920_e8746).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign9920_e8747, ((assign9920_e8745 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) / 1.5)) / assign9920_e8746), ((assign9920_e8745 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) / 1.5)) / assign9920_e8746), ((assign9920_e8745 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) / 1.5)) / assign9920_e8746), ((assign9920_e8745 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) / 1.5)) / assign9920_e8746), ((assign9920_e8745 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) / 1.5)) / assign9920_e8746), );
        }

        if ((locals.var_guard531 != 0.0) && (locals.var_guard533 == 0.0)) {
            let assign9930_e8756: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
            let assign9930_e8758: f64 = (assign9930_e8756 / 1.5);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign9930_e8758, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) / 1.5), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) / 1.5), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) / 1.5), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) / 1.5), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) / 1.5), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9940_e8765: f64 = (1.5 * locals.var_q_temp3);
            let assign9940_e8766: f64 = (locals.var_q_x1sat - assign9940_e8765);
            (locals.var_q_x1, locals.var_q_x1_dn4, locals.var_q_x1_dn6, locals.var_q_x1_dn7, locals.var_q_x1_dn8, locals.var_q_x1_dn9, ) = (assign9940_e8766, (locals.var_q_x1sat_dn4 - (1.5 * locals.var_q_temp3_dn4)), (locals.var_q_x1sat_dn6 - (1.5 * locals.var_q_temp3_dn6)), (locals.var_q_x1sat_dn7 - (1.5 * locals.var_q_temp3_dn7)), (locals.var_q_x1sat_dn8 - (1.5 * locals.var_q_temp3_dn8)), (locals.var_q_x1sat_dn9 - (1.5 * locals.var_q_temp3_dn9)), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9950_e8772: f64 = (locals.var_k2_1d * locals.var_xg20);
            let assign9950_e8774: f64 = (assign9950_e8772 + locals.var_q_x1);
            let assign9950_e8776: f64 = (assign9950_e8774 * locals.var_q_temp2);
            (locals.var_q_x2_wi, locals.var_q_x2_wi_dn4, locals.var_q_x2_wi_dn6, locals.var_q_x2_wi_dn7, locals.var_q_x2_wi_dn8, locals.var_q_x2_wi_dn9, ) = (assign9950_e8776, ((((locals.var_k2_1d * locals.var_xg20_dn4) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn4)), ((((locals.var_k2_1d * locals.var_xg20_dn6) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn6)), ((((locals.var_k2_1d * locals.var_xg20_dn7) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn7)), ((((locals.var_k2_1d * locals.var_xg20_dn8) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn8)), ((((locals.var_k2_1d * locals.var_xg20_dn9) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign9950_e8774 * locals.var_q_temp2_dn9)), );
        }

        let assign9960_e8781: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign9960_e8783: f64 = (assign9960_e8781 / 1.5);
        let assign9960_e8785: f64 = if assign9960_e8783 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard534 = assign9960_e8785;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard534 != 0.0)) {
            let assign9970_e8792: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
            let assign9970_e8794: f64 = (assign9970_e8792 / 1.5);
            let assign9970_e8795: f64 = (assign9970_e8794).exp();
            let assign9970_e8796: f64 = (1.0 + assign9970_e8795);
            let assign9970_e8797: f64 = (assign9970_e8796).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign9970_e8797, ((assign9970_e8795 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) / 1.5)) / assign9970_e8796), ((assign9970_e8795 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) / 1.5)) / assign9970_e8796), ((assign9970_e8795 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) / 1.5)) / assign9970_e8796), ((assign9970_e8795 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) / 1.5)) / assign9970_e8796), ((assign9970_e8795 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) / 1.5)) / assign9970_e8796), );
        }

        if ((locals.var_guard531 != 0.0) && (locals.var_guard534 == 0.0)) {
            let assign9980_e8806: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
            let assign9980_e8808: f64 = (assign9980_e8806 / 1.5);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign9980_e8808, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) / 1.5), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) / 1.5), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) / 1.5), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) / 1.5), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) / 1.5), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9990_e8815: f64 = (1.5 * locals.var_q_temp3);
            let assign9990_e8816: f64 = (locals.var_q_x2sat - assign9990_e8815);
            (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9, ) = (assign9990_e8816, (locals.var_q_x2sat_dn4 - (1.5 * locals.var_q_temp3_dn4)), (locals.var_q_x2sat_dn6 - (1.5 * locals.var_q_temp3_dn6)), (locals.var_q_x2sat_dn7 - (1.5 * locals.var_q_temp3_dn7)), (locals.var_q_x2sat_dn8 - (1.5 * locals.var_q_temp3_dn8)), (locals.var_q_x2sat_dn9 - (1.5 * locals.var_q_temp3_dn9)), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign10000_e8822: f64 = (locals.var_temp * locals.var_temp0);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign10000_e8822, ((locals.var_temp_dn4 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn4)), ((locals.var_temp_dn6 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn6)), ((locals.var_temp_dn7 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn7)), ((locals.var_temp_dn8 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn8)), ((locals.var_temp_dn9 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn9)), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign10010_e8828: f64 = (locals.var_temp * locals.var_xg20);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign10010_e8828, ((locals.var_temp_dn4 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn4)), ((locals.var_temp_dn6 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn6)), ((locals.var_temp_dn7 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn7)), ((locals.var_temp_dn8 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn8)), ((locals.var_temp_dn9 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn9)), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign10020_e8834: f64 = (locals.var_temp1 - locals.var_temp2);
            (locals.var_spsub_xgb, locals.var_spsub_xgb_dn4, locals.var_spsub_xgb_dn6, locals.var_spsub_xgb_dn7, locals.var_spsub_xgb_dn8, locals.var_spsub_xgb_dn9, ) = (assign10020_e8834, (locals.var_temp1_dn4 - locals.var_temp2_dn4), (locals.var_temp1_dn6 - locals.var_temp2_dn6), (locals.var_temp1_dn7 - locals.var_temp2_dn7), (locals.var_temp1_dn8 - locals.var_temp2_dn8), (locals.var_temp1_dn9 - locals.var_temp2_dn9), );
        }

        let assign10030_e8838: f64 = (-locals.var_xn_sub);
        let assign10030_e8839: f64 = (assign10030_e8838).abs();
        let assign10030_e8841: f64 = if assign10030_e8839 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard535 = assign10030_e8841;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard535 != 0.0)) {
            let assign10040_e8846: f64 = (-locals.var_xn_sub);
            let assign10040_e8847: f64 = (assign10040_e8846).exp();
            (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9, ) = (assign10040_e8847, (assign10040_e8847 * (-locals.var_xn_sub_dn4)), (assign10040_e8847 * (-locals.var_xn_sub_dn6)), (assign10040_e8847 * (-locals.var_xn_sub_dn7)), (assign10040_e8847 * (-locals.var_xn_sub_dn8)), (assign10040_e8847 * (-locals.var_xn_sub_dn9)), );
        }

        let assign10050_e8851: f64 = (-locals.var_xn_sub);
        let assign10050_e8853: f64 = (-80.0);
        let assign10050_e8854: f64 = if assign10050_e8851 < assign10050_e8853 { 1.0 } else { 0.0 };
        locals.var_guard536 = assign10050_e8854;

        if (((locals.var_guard531 != 0.0) && (locals.var_guard535 == 0.0)) && (locals.var_guard536 != 0.0)) {
            let assign10060_e8864: f64 = (-locals.var_xn_sub);
            let assign10060_e8865: f64 = (-assign10060_e8864);
            let assign10060_e8867: f64 = (assign10060_e8865 - 80.0);
            let assign10060_e8871: f64 = (-locals.var_xn_sub);
            let assign10060_e8872: f64 = (-assign10060_e8871);
            let assign10060_e8874: f64 = (assign10060_e8872 - 80.0);
            let assign10060_e8875: f64 = (0.5 * assign10060_e8874);
            let assign10060_e8878: f64 = (-locals.var_xn_sub);
            let assign10060_e8879: f64 = (-assign10060_e8878);
            let assign10060_e8881: f64 = (assign10060_e8879 - 80.0);
            let assign10060_e8883: f64 = (assign10060_e8881 * 0.3333333333333);
            let assign10060_e8884: f64 = (1.0 + assign10060_e8883);
            let assign10060_e8885: f64 = (assign10060_e8875 * assign10060_e8884);
            let assign10060_e8886: f64 = (1.0 + assign10060_e8885);
            let assign10060_e8887: f64 = (assign10060_e8867 * assign10060_e8886);
            let assign10060_e8888: f64 = (1.0 + assign10060_e8887);
            let assign10060_e8889: f64 = (1.80485e-35 / assign10060_e8888);
            (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9, ) = (assign10060_e8889, (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn4)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn4))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn4)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn6)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn6))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn6)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn7)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn7))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn7)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn8)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn8))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn8)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn9)) * assign10060_e8886) + (assign10060_e8867 * (((0.5 * (-(-locals.var_xn_sub_dn9))) * assign10060_e8884) + (assign10060_e8875 * ((-(-locals.var_xn_sub_dn9)) * 0.3333333333333)))))) / (assign10060_e8888 * assign10060_e8888))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard535 == 0.0)) && (locals.var_guard536 == 0.0)) {
            let assign10070_e8902: f64 = (-locals.var_xn_sub);
            let assign10070_e8904: f64 = (assign10070_e8902 - 80.0);
            let assign10070_e8908: f64 = (-locals.var_xn_sub);
            let assign10070_e8910: f64 = (assign10070_e8908 - 80.0);
            let assign10070_e8911: f64 = (0.5 * assign10070_e8910);
            let assign10070_e8914: f64 = (-locals.var_xn_sub);
            let assign10070_e8916: f64 = (assign10070_e8914 - 80.0);
            let assign10070_e8918: f64 = (assign10070_e8916 * 0.3333333333333);
            let assign10070_e8919: f64 = (1.0 + assign10070_e8918);
            let assign10070_e8920: f64 = (assign10070_e8911 * assign10070_e8919);
            let assign10070_e8921: f64 = (1.0 + assign10070_e8920);
            let assign10070_e8922: f64 = (assign10070_e8904 * assign10070_e8921);
            let assign10070_e8923: f64 = (1.0 + assign10070_e8922);
            let assign10070_e8924: f64 = (5.54062e34 * assign10070_e8923);
            (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9, ) = (assign10070_e8924, (5.54062e34 * (((-locals.var_xn_sub_dn4) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn4)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn6) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn6)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn7) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn7)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn8) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn8)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn9) * assign10070_e8921) + (assign10070_e8904 * (((0.5 * (-locals.var_xn_sub_dn9)) * assign10070_e8919) + (assign10070_e8911 * ((-locals.var_xn_sub_dn9) * 0.3333333333333)))))), );
        }

        let assign10080_e8928: f64 = (locals.var_spsub_xgb).abs();
        let assign10080_e8930: f64 = if assign10080_e8928 <= locals.var_margin_sub { 1.0 } else { 0.0 };
        locals.var_guard537 = assign10080_e8930;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard537 != 0.0)) {
            let assign10090_e8936: f64 = (locals.var_inv_xisub * locals.var_inv_xisub);
            let assign10090_e8938: f64 = (assign10090_e8936 * 0.1666666666667);
            let assign10090_e8940: f64 = (assign10090_e8938 / 1.4142135623731);
            (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9, ) = (assign10090_e8940, ((((locals.var_inv_xisub_dn4 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn4)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn6 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn6)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn7 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn7)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn8 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn8)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn9 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn9)) * 0.1666666666667) / 1.4142135623731), );
        }

        if ((locals.var_guard531 != 0.0) && (locals.var_guard537 != 0.0)) {
            let assign10100_e8948: f64 = (locals.var_spsub_xgb * locals.var_inv_xisub);
            let assign10100_e8953: f64 = (1.0 - locals.var_spsub_delta);
            let assign10100_e8954: f64 = (locals.var_spsub_xgb * assign10100_e8953);
            let assign10100_e8956: f64 = (assign10100_e8954 * locals.var_gfsub);
            let assign10100_e8958: f64 = (assign10100_e8956 * locals.var_spsub_temp1);
            let assign10100_e8959: f64 = (1.0 + assign10100_e8958);
            let assign10100_e8960: f64 = (assign10100_e8948 * assign10100_e8959);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign10100_e8960, ((((locals.var_spsub_xgb_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn4)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn4 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn4))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn4)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn4)))), ((((locals.var_spsub_xgb_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn6)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn6 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn6))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn6)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn6)))), ((((locals.var_spsub_xgb_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn7)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn7 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn7))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn7)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn7)))), ((((locals.var_spsub_xgb_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn8)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn8 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn8))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn8)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn8)))), ((((locals.var_spsub_xgb_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn9)) * assign10100_e8959) + (assign10100_e8948 * ((((((locals.var_spsub_xgb_dn9 * assign10100_e8953) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn9))) * locals.var_gfsub) + (assign10100_e8954 * locals.var_gfsub_dn9)) * locals.var_spsub_temp1) + (assign10100_e8956 * locals.var_spsub_temp1_dn9)))), );
        }

        let assign10110_e8965: f64 = (-locals.var_margin_sub);
        let assign10110_e8966: f64 = if locals.var_spsub_xgb < assign10110_e8965 { 1.0 } else { 0.0 };
        locals.var_guard538 = assign10110_e8966;

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10120_e8974: f64 = (-locals.var_spsub_xgb);
            (locals.var_spsub_yg, locals.var_spsub_yg_dn4, locals.var_spsub_yg_dn6, locals.var_spsub_yg_dn7, locals.var_spsub_yg_dn8, locals.var_spsub_yg_dn9, ) = (assign10120_e8974, (-locals.var_spsub_xgb_dn4), (-locals.var_spsub_xgb_dn6), (-locals.var_spsub_xgb_dn7), (-locals.var_spsub_xgb_dn8), (-locals.var_spsub_xgb_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10130_e8986: f64 = (locals.var_spsub_yg * locals.var_inv_xisub);
            let assign10130_e8987: f64 = (1.25 * assign10130_e8986);
            (locals.var_spsub_ysub, locals.var_spsub_ysub_dn4, locals.var_spsub_ysub_dn6, locals.var_spsub_ysub_dn7, locals.var_spsub_ysub_dn8, locals.var_spsub_ysub_dn9, ) = (assign10130_e8987, (1.25 * ((locals.var_spsub_yg_dn4 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn4))), (1.25 * ((locals.var_spsub_yg_dn6 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn6))), (1.25 * ((locals.var_spsub_yg_dn7 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn7))), (1.25 * ((locals.var_spsub_yg_dn8 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn8))), (1.25 * ((locals.var_spsub_yg_dn9 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10140_e8999: f64 = (locals.var_spsub_ysub + 10.0);
            let assign10140_e9002: f64 = (locals.var_spsub_ysub - 6.0);
            let assign10140_e9005: f64 = (locals.var_spsub_ysub - 6.0);
            let assign10140_e9006: f64 = (assign10140_e9002 * assign10140_e9005);
            let assign10140_e9008: f64 = (assign10140_e9006 + 64.0);
            let assign10140_e9009: f64 = (assign10140_e9008).sqrt();
            let assign10140_e9010: f64 = (assign10140_e8999 - assign10140_e9009);
            let assign10140_e9011: f64 = (0.5 * assign10140_e9010);
            (locals.var_spsub_eta, locals.var_spsub_eta_dn4, locals.var_spsub_eta_dn6, locals.var_spsub_eta_dn7, locals.var_spsub_eta_dn8, locals.var_spsub_eta_dn9, ) = (assign10140_e9011, (0.5 * (locals.var_spsub_ysub_dn4 - (((locals.var_spsub_ysub_dn4 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn4)) / (2.0 * assign10140_e9009)))), (0.5 * (locals.var_spsub_ysub_dn6 - (((locals.var_spsub_ysub_dn6 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn6)) / (2.0 * assign10140_e9009)))), (0.5 * (locals.var_spsub_ysub_dn7 - (((locals.var_spsub_ysub_dn7 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn7)) / (2.0 * assign10140_e9009)))), (0.5 * (locals.var_spsub_ysub_dn8 - (((locals.var_spsub_ysub_dn8 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn8)) / (2.0 * assign10140_e9009)))), (0.5 * (locals.var_spsub_ysub_dn9 - (((locals.var_spsub_ysub_dn9 * assign10140_e9005) + (assign10140_e9002 * locals.var_spsub_ysub_dn9)) / (2.0 * assign10140_e9009)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10150_e9022: f64 = (locals.var_spsub_yg - locals.var_spsub_eta);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10150_e9022, (locals.var_spsub_yg_dn4 - locals.var_spsub_eta_dn4), (locals.var_spsub_yg_dn6 - locals.var_spsub_eta_dn6), (locals.var_spsub_yg_dn7 - locals.var_spsub_eta_dn7), (locals.var_spsub_yg_dn8 - locals.var_spsub_eta_dn8), (locals.var_spsub_yg_dn9 - locals.var_spsub_eta_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10160_e9033: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
            let assign10160_e9037: f64 = (locals.var_spsub_eta + 1.0);
            let assign10160_e9038: f64 = (locals.var_gfsub2 * assign10160_e9037);
            let assign10160_e9039: f64 = (assign10160_e9033 + assign10160_e9038);
            (locals.var_spsub_a, locals.var_spsub_a_dn4, locals.var_spsub_a_dn6, locals.var_spsub_a_dn7, locals.var_spsub_a_dn8, locals.var_spsub_a_dn9, ) = (assign10160_e9039, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) + ((locals.var_gfsub2_dn4 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn4))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) + ((locals.var_gfsub2_dn6 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn6))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) + ((locals.var_gfsub2_dn7 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn7))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) + ((locals.var_gfsub2_dn8 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn8))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) + ((locals.var_gfsub2_dn9 * assign10160_e9037) + (locals.var_gfsub2 * locals.var_spsub_eta_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10170_e9050: f64 = (2.0 * locals.var_spsub_temp);
            let assign10170_e9052: f64 = (assign10170_e9050 - locals.var_gfsub2);
            (locals.var_spsub_c, locals.var_spsub_c_dn4, locals.var_spsub_c_dn6, locals.var_spsub_c_dn7, locals.var_spsub_c_dn8, locals.var_spsub_c_dn9, ) = (assign10170_e9052, ((2.0 * locals.var_spsub_temp_dn4) - locals.var_gfsub2_dn4), ((2.0 * locals.var_spsub_temp_dn6) - locals.var_gfsub2_dn6), ((2.0 * locals.var_spsub_temp_dn7) - locals.var_gfsub2_dn7), ((2.0 * locals.var_spsub_temp_dn8) - locals.var_gfsub2_dn8), ((2.0 * locals.var_spsub_temp_dn9) - locals.var_gfsub2_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10180_e9062: f64 = (-locals.var_spsub_eta);
            let assign10180_e9065: f64 = (locals.var_spsub_a * locals.var_inv_gfsub2);
            let assign10180_e9066: f64 = (assign10180_e9065).ln();
            let assign10180_e9067: f64 = (assign10180_e9062 + assign10180_e9066);
            (locals.var_spsub_tau, locals.var_spsub_tau_dn4, locals.var_spsub_tau_dn6, locals.var_spsub_tau_dn7, locals.var_spsub_tau_dn8, locals.var_spsub_tau_dn9, ) = (assign10180_e9067, ((-locals.var_spsub_eta_dn4) + (((locals.var_spsub_a_dn4 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn4)) / assign10180_e9065)), ((-locals.var_spsub_eta_dn6) + (((locals.var_spsub_a_dn6 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn6)) / assign10180_e9065)), ((-locals.var_spsub_eta_dn7) + (((locals.var_spsub_a_dn7 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn7)) / assign10180_e9065)), ((-locals.var_spsub_eta_dn8) + (((locals.var_spsub_a_dn8 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn8)) / assign10180_e9065)), ((-locals.var_spsub_eta_dn9) + (((locals.var_spsub_a_dn9 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn9)) / assign10180_e9065)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10190_e9078: f64 = (locals.var_spsub_a + locals.var_spsub_c);
            (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, ) = (assign10190_e9078, (locals.var_spsub_a_dn4 + locals.var_spsub_c_dn4), (locals.var_spsub_a_dn6 + locals.var_spsub_c_dn6), (locals.var_spsub_a_dn7 + locals.var_spsub_c_dn7), (locals.var_spsub_a_dn8 + locals.var_spsub_c_dn8), (locals.var_spsub_a_dn9 + locals.var_spsub_c_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10200_e9089: f64 = (locals.var_nu * locals.var_nu);
            let assign10200_e9093: f64 = (0.5 * locals.var_spsub_c);
            let assign10200_e9095: f64 = (assign10200_e9093 * locals.var_spsub_c);
            let assign10200_e9097: f64 = (assign10200_e9095 - locals.var_spsub_a);
            let assign10200_e9098: f64 = (locals.var_spsub_tau * assign10200_e9097);
            let assign10200_e9099: f64 = (assign10200_e9089 + assign10200_e9098);
            (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, ) = (assign10200_e9099, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_spsub_tau_dn4 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn4) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn4)) - locals.var_spsub_a_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_spsub_tau_dn6 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn6) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn6)) - locals.var_spsub_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_spsub_tau_dn7 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn7) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn7)) - locals.var_spsub_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_spsub_tau_dn8 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn8) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn8)) - locals.var_spsub_a_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_spsub_tau_dn9 * assign10200_e9097) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn9) * locals.var_spsub_c) + (assign10200_e9093 * locals.var_spsub_c_dn9)) - locals.var_spsub_a_dn9)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10210_e9111: f64 = (locals.var_spsub_a * locals.var_nu);
            let assign10210_e9113: f64 = (assign10210_e9111 * locals.var_spsub_tau);
            let assign10210_e9117: f64 = (locals.var_nu / locals.var_mutau);
            let assign10210_e9119: f64 = (assign10210_e9117 * locals.var_spsub_tau);
            let assign10210_e9121: f64 = (assign10210_e9119 * locals.var_spsub_tau);
            let assign10210_e9123: f64 = (assign10210_e9121 * locals.var_spsub_c);
            let assign10210_e9126: f64 = (locals.var_spsub_c * locals.var_spsub_c);
            let assign10210_e9128: f64 = (assign10210_e9126 * 0.3333333333333);
            let assign10210_e9130: f64 = (assign10210_e9128 - locals.var_spsub_a);
            let assign10210_e9131: f64 = (assign10210_e9123 * assign10210_e9130);
            let assign10210_e9132: f64 = (locals.var_mutau + assign10210_e9131);
            let assign10210_e9133: f64 = (assign10210_e9113 / assign10210_e9132);
            let assign10210_e9134: f64 = (locals.var_spsub_eta + assign10210_e9133);
            (locals.var_spsub_y0, locals.var_spsub_y0_dn4, locals.var_spsub_y0_dn6, locals.var_spsub_y0_dn7, locals.var_spsub_y0_dn8, locals.var_spsub_y0_dn9, ) = (assign10210_e9134, (locals.var_spsub_eta_dn4 + (((((((locals.var_spsub_a_dn4 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn4)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn4)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn4)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn4)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn4)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn4 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn4)) * 0.3333333333333) - locals.var_spsub_a_dn4)))))) / (assign10210_e9132 * assign10210_e9132))), (locals.var_spsub_eta_dn6 + (((((((locals.var_spsub_a_dn6 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn6)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn6)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn6)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn6)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn6)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn6 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn6)) * 0.3333333333333) - locals.var_spsub_a_dn6)))))) / (assign10210_e9132 * assign10210_e9132))), (locals.var_spsub_eta_dn7 + (((((((locals.var_spsub_a_dn7 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn7)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn7)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn7)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn7)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn7)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn7 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn7)) * 0.3333333333333) - locals.var_spsub_a_dn7)))))) / (assign10210_e9132 * assign10210_e9132))), (locals.var_spsub_eta_dn8 + (((((((locals.var_spsub_a_dn8 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn8)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn8)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn8)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn8)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn8)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn8 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn8)) * 0.3333333333333) - locals.var_spsub_a_dn8)))))) / (assign10210_e9132 * assign10210_e9132))), (locals.var_spsub_eta_dn9 + (((((((locals.var_spsub_a_dn9 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn9)) * locals.var_spsub_tau) + (assign10210_e9111 * locals.var_spsub_tau_dn9)) * assign10210_e9132) - (assign10210_e9113 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10210_e9117 * locals.var_spsub_tau_dn9)) * locals.var_spsub_tau) + (assign10210_e9119 * locals.var_spsub_tau_dn9)) * locals.var_spsub_c) + (assign10210_e9121 * locals.var_spsub_c_dn9)) * assign10210_e9130) + (assign10210_e9123 * ((((locals.var_spsub_c_dn9 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn9)) * 0.3333333333333) - locals.var_spsub_a_dn9)))))) / (assign10210_e9132 * assign10210_e9132))), );
        }

        let assign10220_e9139: f64 = if locals.var_spsub_y0 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard539 = assign10220_e9139;

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) {
            let assign10230_e9149: f64 = (locals.var_spsub_y0).exp();
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10230_e9149, (assign10230_e9149 * locals.var_spsub_y0_dn4), (assign10230_e9149 * locals.var_spsub_y0_dn6), (assign10230_e9149 * locals.var_spsub_y0_dn7), (assign10230_e9149 * locals.var_spsub_y0_dn8), (assign10230_e9149 * locals.var_spsub_y0_dn9), );
        }

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 == 0.0)) {
            let assign10240_e9165: f64 = (locals.var_spsub_y0 - 80.0);
            let assign10240_e9170: f64 = (locals.var_spsub_y0 - 80.0);
            let assign10240_e9171: f64 = (0.5 * assign10240_e9170);
            let assign10240_e9175: f64 = (locals.var_spsub_y0 - 80.0);
            let assign10240_e9177: f64 = (assign10240_e9175 * 0.3333333333333);
            let assign10240_e9178: f64 = (1.0 + assign10240_e9177);
            let assign10240_e9179: f64 = (assign10240_e9171 * assign10240_e9178);
            let assign10240_e9180: f64 = (1.0 + assign10240_e9179);
            let assign10240_e9181: f64 = (assign10240_e9165 * assign10240_e9180);
            let assign10240_e9182: f64 = (1.0 + assign10240_e9181);
            let assign10240_e9183: f64 = (5.54062e34 * assign10240_e9182);
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10240_e9183, (5.54062e34 * ((locals.var_spsub_y0_dn4 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn4) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn6 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn6) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn7 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn7) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn8 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn8) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn9 * assign10240_e9180) + (assign10240_e9165 * (((0.5 * locals.var_spsub_y0_dn9) * assign10240_e9178) + (assign10240_e9171 * (locals.var_spsub_y0_dn9 * 0.3333333333333)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10250_e9194: f64 = (1.0 / locals.var_spsub_delta0);
            (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9, ) = (assign10250_e9194, (-(locals.var_spsub_delta0_dn4 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn6 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn7 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn8 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn9 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10260_e9207: f64 = (locals.var_spsub_y0 * locals.var_spsub_y0);
            let assign10260_e9208: f64 = (2.0 + assign10260_e9207);
            let assign10260_e9209: f64 = (1.0 / assign10260_e9208);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10260_e9209, (-(((locals.var_spsub_y0_dn4 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn4)) / (assign10260_e9208 * assign10260_e9208))), (-(((locals.var_spsub_y0_dn6 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn6)) / (assign10260_e9208 * assign10260_e9208))), (-(((locals.var_spsub_y0_dn7 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn7)) / (assign10260_e9208 * assign10260_e9208))), (-(((locals.var_spsub_y0_dn8 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn8)) / (assign10260_e9208 * assign10260_e9208))), (-(((locals.var_spsub_y0_dn9 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn9)) / (assign10260_e9208 * assign10260_e9208))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10270_e9220: f64 = (locals.var_spsub_y0 * locals.var_spsub_y0);
            let assign10270_e9222: f64 = (assign10270_e9220 * locals.var_spsub_temp);
            (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9, ) = (assign10270_e9222, ((((locals.var_spsub_y0_dn4 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn4)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn4)), ((((locals.var_spsub_y0_dn6 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn6)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn6)), ((((locals.var_spsub_y0_dn7 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn7)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn7)), ((((locals.var_spsub_y0_dn8 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn8)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn8)), ((((locals.var_spsub_y0_dn9 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn9)) * locals.var_spsub_temp) + (assign10270_e9220 * locals.var_spsub_temp_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10280_e9234: f64 = (locals.var_spsub_y0 * locals.var_spsub_temp);
            let assign10280_e9236: f64 = (assign10280_e9234 * locals.var_spsub_temp);
            let assign10280_e9237: f64 = (4.0 * assign10280_e9236);
            (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9, ) = (assign10280_e9237, (4.0 * ((((locals.var_spsub_y0_dn4 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn4))), (4.0 * ((((locals.var_spsub_y0_dn6 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn6))), (4.0 * ((((locals.var_spsub_y0_dn7 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn7))), (4.0 * ((((locals.var_spsub_y0_dn8 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn8))), (4.0 * ((((locals.var_spsub_y0_dn9 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10280_e9234 * locals.var_spsub_temp_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10290_e9248: f64 = (8.0 * locals.var_spsub_temp);
            let assign10290_e9251: f64 = (12.0 * locals.var_spsub_xi0);
            let assign10290_e9252: f64 = (assign10290_e9248 - assign10290_e9251);
            let assign10290_e9254: f64 = (assign10290_e9252 * locals.var_spsub_temp);
            let assign10290_e9256: f64 = (assign10290_e9254 * locals.var_spsub_temp);
            (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9, ) = (assign10290_e9256, ((((((8.0 * locals.var_spsub_temp_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn4)), ((((((8.0 * locals.var_spsub_temp_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn6)), ((((((8.0 * locals.var_spsub_temp_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn7)), ((((((8.0 * locals.var_spsub_temp_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn8)), ((((((8.0 * locals.var_spsub_temp_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp) + (assign10290_e9252 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10290_e9254 * locals.var_spsub_temp_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10300_e9267: f64 = (locals.var_spsub_yg - locals.var_spsub_y0);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10300_e9267, (locals.var_spsub_yg_dn4 - locals.var_spsub_y0_dn4), (locals.var_spsub_yg_dn6 - locals.var_spsub_y0_dn6), (locals.var_spsub_yg_dn7 - locals.var_spsub_y0_dn7), (locals.var_spsub_yg_dn8 - locals.var_spsub_y0_dn8), (locals.var_spsub_yg_dn9 - locals.var_spsub_y0_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10310_e9278: f64 = (locals.var_spsub_delta * locals.var_spsub_delta1);
            (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9, ) = (assign10310_e9278, ((locals.var_spsub_delta_dn4 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn4)), ((locals.var_spsub_delta_dn6 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn6)), ((locals.var_spsub_delta_dn7 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn7)), ((locals.var_spsub_delta_dn8 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn8)), ((locals.var_spsub_delta_dn9 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10320_e9289: f64 = (2.0 * locals.var_spsub_temp);
            let assign10320_e9293: f64 = (locals.var_spsub_delta0 - 1.0);
            let assign10320_e9295: f64 = (assign10320_e9293 - locals.var_spsub_temp1);
            let assign10320_e9299: f64 = (1.0 - locals.var_spsub_xi1);
            let assign10320_e9300: f64 = (locals.var_spsub_delta * assign10320_e9299);
            let assign10320_e9301: f64 = (assign10320_e9295 + assign10320_e9300);
            let assign10320_e9302: f64 = (locals.var_gfsub2 * assign10320_e9301);
            let assign10320_e9303: f64 = (assign10320_e9289 + assign10320_e9302);
            (locals.var_spsub_pc, locals.var_spsub_pc_dn4, locals.var_spsub_pc_dn6, locals.var_spsub_pc_dn7, locals.var_spsub_pc_dn8, locals.var_spsub_pc_dn9, ) = (assign10320_e9303, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn4 - locals.var_spsub_temp1_dn4) + ((locals.var_spsub_delta_dn4 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn4))))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn6 - locals.var_spsub_temp1_dn6) + ((locals.var_spsub_delta_dn6 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn6))))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn7 - locals.var_spsub_temp1_dn7) + ((locals.var_spsub_delta_dn7 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn7))))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn8 - locals.var_spsub_temp1_dn8) + ((locals.var_spsub_delta_dn8 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn8))))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10320_e9301) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn9 - locals.var_spsub_temp1_dn9) + ((locals.var_spsub_delta_dn9 * assign10320_e9299) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn9))))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10330_e9314: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
            let assign10330_e9318: f64 = (locals.var_spsub_delta0 - locals.var_spsub_y0);
            let assign10330_e9320: f64 = (assign10330_e9318 - 1.0);
            let assign10330_e9322: f64 = (assign10330_e9320 + locals.var_spsub_temp1);
            let assign10330_e9326: f64 = (locals.var_spsub_y0 - 1.0);
            let assign10330_e9328: f64 = (assign10330_e9326 - locals.var_spsub_xi0);
            let assign10330_e9329: f64 = (locals.var_spsub_delta * assign10330_e9328);
            let assign10330_e9330: f64 = (assign10330_e9322 + assign10330_e9329);
            let assign10330_e9331: f64 = (locals.var_gfsub2 * assign10330_e9330);
            let assign10330_e9332: f64 = (assign10330_e9314 - assign10330_e9331);
            (locals.var_spsub_qc, locals.var_spsub_qc_dn4, locals.var_spsub_qc_dn6, locals.var_spsub_qc_dn7, locals.var_spsub_qc_dn8, locals.var_spsub_qc_dn9, ) = (assign10330_e9332, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn4 - locals.var_spsub_y0_dn4) + locals.var_spsub_temp1_dn4) + ((locals.var_spsub_delta_dn4 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn4 - locals.var_spsub_xi0_dn4))))))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn6 - locals.var_spsub_y0_dn6) + locals.var_spsub_temp1_dn6) + ((locals.var_spsub_delta_dn6 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn6 - locals.var_spsub_xi0_dn6))))))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn7 - locals.var_spsub_y0_dn7) + locals.var_spsub_temp1_dn7) + ((locals.var_spsub_delta_dn7 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn7 - locals.var_spsub_xi0_dn7))))))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn8 - locals.var_spsub_y0_dn8) + locals.var_spsub_temp1_dn8) + ((locals.var_spsub_delta_dn8 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn8 - locals.var_spsub_xi0_dn8))))))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10330_e9330) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn9 - locals.var_spsub_y0_dn9) + locals.var_spsub_temp1_dn9) + ((locals.var_spsub_delta_dn9 * assign10330_e9328) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn9 - locals.var_spsub_xi0_dn9))))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10340_e9345: f64 = (locals.var_spsub_delta0 + locals.var_spsub_temp1);
            let assign10340_e9348: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
            let assign10340_e9349: f64 = (assign10340_e9345 - assign10340_e9348);
            let assign10340_e9350: f64 = (locals.var_gfsub2 * assign10340_e9349);
            let assign10340_e9351: f64 = (2.0 - assign10340_e9350);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10340_e9351, (-((locals.var_gfsub2_dn4 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn4 + locals.var_spsub_temp1_dn4) - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4)))))), (-((locals.var_gfsub2_dn6 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn6 + locals.var_spsub_temp1_dn6) - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6)))))), (-((locals.var_gfsub2_dn7 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn7 + locals.var_spsub_temp1_dn7) - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7)))))), (-((locals.var_gfsub2_dn8 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn8 + locals.var_spsub_temp1_dn8) - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8)))))), (-((locals.var_gfsub2_dn9 * assign10340_e9349) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn9 + locals.var_spsub_temp1_dn9) - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10350_e9362: f64 = (locals.var_spsub_pc * locals.var_spsub_pc);
            let assign10350_e9366: f64 = (locals.var_spsub_qc * locals.var_spsub_temp);
            let assign10350_e9367: f64 = (2.0 * assign10350_e9366);
            let assign10350_e9368: f64 = (assign10350_e9362 - assign10350_e9367);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10350_e9368, (((locals.var_spsub_pc_dn4 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn4)) - (2.0 * ((locals.var_spsub_qc_dn4 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn4)))), (((locals.var_spsub_pc_dn6 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn6)) - (2.0 * ((locals.var_spsub_qc_dn6 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn6)))), (((locals.var_spsub_pc_dn7 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn7)) - (2.0 * ((locals.var_spsub_qc_dn7 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn7)))), (((locals.var_spsub_pc_dn8 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn8)) - (2.0 * ((locals.var_spsub_qc_dn8 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn8)))), (((locals.var_spsub_pc_dn9 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn9)) - (2.0 * ((locals.var_spsub_qc_dn9 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn9)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10360_e9378: f64 = (-locals.var_spsub_y0);
            let assign10360_e9383: f64 = (locals.var_spsub_temp).sqrt();
            let assign10360_e9384: f64 = (locals.var_spsub_pc + assign10360_e9383);
            let assign10360_e9385: f64 = (locals.var_spsub_qc / assign10360_e9384);
            let assign10360_e9386: f64 = (2.0 * assign10360_e9385);
            let assign10360_e9387: f64 = (assign10360_e9378 - assign10360_e9386);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign10360_e9387, ((-locals.var_spsub_y0_dn4) - (2.0 * (((locals.var_spsub_qc_dn4 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn4 + (locals.var_spsub_temp_dn4 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), ((-locals.var_spsub_y0_dn6) - (2.0 * (((locals.var_spsub_qc_dn6 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn6 + (locals.var_spsub_temp_dn6 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), ((-locals.var_spsub_y0_dn7) - (2.0 * (((locals.var_spsub_qc_dn7 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn7 + (locals.var_spsub_temp_dn7 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), ((-locals.var_spsub_y0_dn8) - (2.0 * (((locals.var_spsub_qc_dn8 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn8 + (locals.var_spsub_temp_dn8 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), ((-locals.var_spsub_y0_dn9) - (2.0 * (((locals.var_spsub_qc_dn9 * assign10360_e9384) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn9 + (locals.var_spsub_temp_dn9 / (2.0 * assign10360_e9383))))) / (assign10360_e9384 * assign10360_e9384)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10370_e9401: f64 = (locals.var_gfsub * 0.732464877560822);
            let assign10370_e9402: f64 = (1.25 + assign10370_e9401);
            let assign10370_e9403: f64 = (1.0 / assign10370_e9402);
            (locals.var_spsub_xg1, locals.var_spsub_xg1_dn4, locals.var_spsub_xg1_dn6, locals.var_spsub_xg1_dn7, locals.var_spsub_xg1_dn8, locals.var_spsub_xg1_dn9, ) = (assign10370_e9403, (-((locals.var_gfsub_dn4 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), (-((locals.var_gfsub_dn6 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), (-((locals.var_gfsub_dn7 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), (-((locals.var_gfsub_dn8 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), (-((locals.var_gfsub_dn9 * 0.732464877560822) / (assign10370_e9402 * assign10370_e9402))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10380_e9415: f64 = (1.25 * locals.var_xisub);
            let assign10380_e9417: f64 = (assign10380_e9415 * locals.var_spsub_xg1);
            let assign10380_e9419: f64 = (assign10380_e9417 - 1.0);
            let assign10380_e9421: f64 = (assign10380_e9419 * locals.var_spsub_xg1);
            (locals.var_spsub_a_fac, locals.var_spsub_a_fac_dn4, locals.var_spsub_a_fac_dn6, locals.var_spsub_a_fac_dn7, locals.var_spsub_a_fac_dn8, locals.var_spsub_a_fac_dn9, ) = (assign10380_e9421, (((((1.25 * locals.var_xisub_dn4) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn4)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn4)), (((((1.25 * locals.var_xisub_dn6) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn6)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn6)), (((((1.25 * locals.var_xisub_dn7) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn7)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn7)), (((((1.25 * locals.var_xisub_dn8) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn8)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn8)), (((((1.25 * locals.var_xisub_dn9) * locals.var_spsub_xg1) + (assign10380_e9415 * locals.var_spsub_xg1_dn9)) * locals.var_spsub_xg1) + (assign10380_e9419 * locals.var_spsub_xg1_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10390_e9433: f64 = (locals.var_spsub_xgb * locals.var_inv_xisub);
            let assign10390_e9437: f64 = (locals.var_spsub_a_fac * locals.var_spsub_xgb);
            let assign10390_e9438: f64 = (1.0 + assign10390_e9437);
            let assign10390_e9439: f64 = (assign10390_e9433 * assign10390_e9438);
            (locals.var_spsub_xbar, locals.var_spsub_xbar_dn4, locals.var_spsub_xbar_dn6, locals.var_spsub_xbar_dn7, locals.var_spsub_xbar_dn8, locals.var_spsub_xbar_dn9, ) = (assign10390_e9439, ((((locals.var_spsub_xgb_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn4)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn4 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn4)))), ((((locals.var_spsub_xgb_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn6)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn6 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn6)))), ((((locals.var_spsub_xgb_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn7)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn7 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn7)))), ((((locals.var_spsub_xgb_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn8)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn8 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn8)))), ((((locals.var_spsub_xgb_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn9)) * assign10390_e9438) + (assign10390_e9433 * ((locals.var_spsub_a_fac_dn9 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn9)))), );
        }

        let assign10400_e9443: f64 = (-locals.var_spsub_xbar);
        let assign10400_e9445: f64 = (-80.0);
        let assign10400_e9446: f64 = if assign10400_e9443 > assign10400_e9445 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign10400_e9446;

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard540 != 0.0)) {
            let assign10410_e9457: f64 = (-locals.var_spsub_xbar);
            let assign10410_e9458: f64 = (assign10410_e9457).exp();
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10410_e9458, (assign10410_e9458 * (-locals.var_spsub_xbar_dn4)), (assign10410_e9458 * (-locals.var_spsub_xbar_dn6)), (assign10410_e9458 * (-locals.var_spsub_xbar_dn7)), (assign10410_e9458 * (-locals.var_spsub_xbar_dn8)), (assign10410_e9458 * (-locals.var_spsub_xbar_dn9)), );
        }

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard540 == 0.0)) {
            let assign10420_e9474: f64 = (-locals.var_spsub_xbar);
            let assign10420_e9475: f64 = (-assign10420_e9474);
            let assign10420_e9477: f64 = (assign10420_e9475 - 80.0);
            let assign10420_e9481: f64 = (-locals.var_spsub_xbar);
            let assign10420_e9482: f64 = (-assign10420_e9481);
            let assign10420_e9484: f64 = (assign10420_e9482 - 80.0);
            let assign10420_e9485: f64 = (0.5 * assign10420_e9484);
            let assign10420_e9488: f64 = (-locals.var_spsub_xbar);
            let assign10420_e9489: f64 = (-assign10420_e9488);
            let assign10420_e9491: f64 = (assign10420_e9489 - 80.0);
            let assign10420_e9493: f64 = (assign10420_e9491 * 0.3333333333333);
            let assign10420_e9494: f64 = (1.0 + assign10420_e9493);
            let assign10420_e9495: f64 = (assign10420_e9485 * assign10420_e9494);
            let assign10420_e9496: f64 = (1.0 + assign10420_e9495);
            let assign10420_e9497: f64 = (assign10420_e9477 * assign10420_e9496);
            let assign10420_e9498: f64 = (1.0 + assign10420_e9497);
            let assign10420_e9499: f64 = (1.80485e-35 / assign10420_e9498);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10420_e9499, (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn4)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn4))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn4)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn6)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn6))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn6)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn7)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn7))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn7)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn8)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn8))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn8)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn9)) * assign10420_e9496) + (assign10420_e9477 * (((0.5 * (-(-locals.var_spsub_xbar_dn9))) * assign10420_e9494) + (assign10420_e9485 * ((-(-locals.var_spsub_xbar_dn9)) * 0.3333333333333)))))) / (assign10420_e9498 * assign10420_e9498))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10430_e9511: f64 = (1.0 - locals.var_spsub_temp);
            (locals.var_spsub_w, locals.var_spsub_w_dn4, locals.var_spsub_w_dn6, locals.var_spsub_w_dn7, locals.var_spsub_w_dn8, locals.var_spsub_w_dn9, ) = (assign10430_e9511, (-locals.var_spsub_temp_dn4), (-locals.var_spsub_temp_dn6), (-locals.var_spsub_temp_dn7), (-locals.var_spsub_temp_dn8), (-locals.var_spsub_temp_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10440_e9524: f64 = (locals.var_gfsub2 * 0.5);
            let assign10440_e9525: f64 = (locals.var_spsub_xgb + assign10440_e9524);
            let assign10440_e9530: f64 = (locals.var_gfsub2 * 0.25);
            let assign10440_e9531: f64 = (locals.var_spsub_xgb + assign10440_e9530);
            let assign10440_e9533: f64 = (assign10440_e9531 - locals.var_spsub_w);
            let assign10440_e9534: f64 = (assign10440_e9533).sqrt();
            let assign10440_e9535: f64 = (locals.var_gfsub * assign10440_e9534);
            let assign10440_e9536: f64 = (assign10440_e9525 - assign10440_e9535);
            (locals.var_spsub_x1, locals.var_spsub_x1_dn4, locals.var_spsub_x1_dn6, locals.var_spsub_x1_dn7, locals.var_spsub_x1_dn8, locals.var_spsub_x1_dn9, ) = (assign10440_e9536, ((locals.var_spsub_xgb_dn4 + (locals.var_gfsub2_dn4 * 0.5)) - ((locals.var_gfsub_dn4 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn4 + (locals.var_gfsub2_dn4 * 0.25)) - locals.var_spsub_w_dn4) / (2.0 * assign10440_e9534))))), ((locals.var_spsub_xgb_dn6 + (locals.var_gfsub2_dn6 * 0.5)) - ((locals.var_gfsub_dn6 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn6 + (locals.var_gfsub2_dn6 * 0.25)) - locals.var_spsub_w_dn6) / (2.0 * assign10440_e9534))))), ((locals.var_spsub_xgb_dn7 + (locals.var_gfsub2_dn7 * 0.5)) - ((locals.var_gfsub_dn7 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn7 + (locals.var_gfsub2_dn7 * 0.25)) - locals.var_spsub_w_dn7) / (2.0 * assign10440_e9534))))), ((locals.var_spsub_xgb_dn8 + (locals.var_gfsub2_dn8 * 0.5)) - ((locals.var_gfsub_dn8 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn8 + (locals.var_gfsub2_dn8 * 0.25)) - locals.var_spsub_w_dn8) / (2.0 * assign10440_e9534))))), ((locals.var_spsub_xgb_dn9 + (locals.var_gfsub2_dn9 * 0.5)) - ((locals.var_gfsub_dn9 * assign10440_e9534) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn9 + (locals.var_gfsub2_dn9 * 0.25)) - locals.var_spsub_w_dn9) / (2.0 * assign10440_e9534))))), );
        }

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10450_e9548: f64 = (locals.var_xn_sub + 3.0);
            (locals.var_spsub_bx, locals.var_spsub_bx_dn4, locals.var_spsub_bx_dn6, locals.var_spsub_bx_dn7, locals.var_spsub_bx_dn8, locals.var_spsub_bx_dn9, ) = (assign10450_e9548, locals.var_xn_sub_dn4, locals.var_xn_sub_dn6, locals.var_xn_sub_dn7, locals.var_xn_sub_dn8, locals.var_xn_sub_dn9, );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10460_e9561: f64 = (locals.var_spsub_x1 + locals.var_spsub_bx);
            let assign10460_e9564: f64 = (locals.var_spsub_x1 - locals.var_spsub_bx);
            let assign10460_e9567: f64 = (locals.var_spsub_x1 - locals.var_spsub_bx);
            let assign10460_e9568: f64 = (assign10460_e9564 * assign10460_e9567);
            let assign10460_e9570: f64 = (assign10460_e9568 + 5.0);
            let assign10460_e9571: f64 = (assign10460_e9570).sqrt();
            let assign10460_e9572: f64 = (assign10460_e9561 - assign10460_e9571);
            let assign10460_e9573: f64 = (0.5 * assign10460_e9572);
            let assign10460_e9578: f64 = (locals.var_spsub_bx * locals.var_spsub_bx);
            let assign10460_e9580: f64 = (assign10460_e9578 + 5.0);
            let assign10460_e9581: f64 = (assign10460_e9580).sqrt();
            let assign10460_e9582: f64 = (locals.var_spsub_bx - assign10460_e9581);
            let assign10460_e9583: f64 = (0.5 * assign10460_e9582);
            let assign10460_e9584: f64 = (assign10460_e9573 - assign10460_e9583);
            (locals.var_spsub_eta, locals.var_spsub_eta_dn4, locals.var_spsub_eta_dn6, locals.var_spsub_eta_dn7, locals.var_spsub_eta_dn8, locals.var_spsub_eta_dn9, ) = (assign10460_e9584, ((0.5 * ((locals.var_spsub_x1_dn4 + locals.var_spsub_bx_dn4) - ((((locals.var_spsub_x1_dn4 - locals.var_spsub_bx_dn4) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn4 - locals.var_spsub_bx_dn4))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn4 - (((locals.var_spsub_bx_dn4 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn4)) / (2.0 * assign10460_e9581))))), ((0.5 * ((locals.var_spsub_x1_dn6 + locals.var_spsub_bx_dn6) - ((((locals.var_spsub_x1_dn6 - locals.var_spsub_bx_dn6) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn6 - locals.var_spsub_bx_dn6))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn6 - (((locals.var_spsub_bx_dn6 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn6)) / (2.0 * assign10460_e9581))))), ((0.5 * ((locals.var_spsub_x1_dn7 + locals.var_spsub_bx_dn7) - ((((locals.var_spsub_x1_dn7 - locals.var_spsub_bx_dn7) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn7 - locals.var_spsub_bx_dn7))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn7 - (((locals.var_spsub_bx_dn7 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn7)) / (2.0 * assign10460_e9581))))), ((0.5 * ((locals.var_spsub_x1_dn8 + locals.var_spsub_bx_dn8) - ((((locals.var_spsub_x1_dn8 - locals.var_spsub_bx_dn8) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn8 - locals.var_spsub_bx_dn8))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn8 - (((locals.var_spsub_bx_dn8 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn8)) / (2.0 * assign10460_e9581))))), ((0.5 * ((locals.var_spsub_x1_dn9 + locals.var_spsub_bx_dn9) - ((((locals.var_spsub_x1_dn9 - locals.var_spsub_bx_dn9) * assign10460_e9567) + (assign10460_e9564 * (locals.var_spsub_x1_dn9 - locals.var_spsub_bx_dn9))) / (2.0 * assign10460_e9571)))) - (0.5 * (locals.var_spsub_bx_dn9 - (((locals.var_spsub_bx_dn9 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn9)) / (2.0 * assign10460_e9581))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10470_e9596: f64 = (locals.var_spsub_xgb - locals.var_spsub_eta);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10470_e9596, (locals.var_spsub_xgb_dn4 - locals.var_spsub_eta_dn4), (locals.var_spsub_xgb_dn6 - locals.var_spsub_eta_dn6), (locals.var_spsub_xgb_dn7 - locals.var_spsub_eta_dn7), (locals.var_spsub_xgb_dn8 - locals.var_spsub_eta_dn8), (locals.var_spsub_xgb_dn9 - locals.var_spsub_eta_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10480_e9607: f64 = (-locals.var_spsub_eta);
            let assign10480_e9608: f64 = (assign10480_e9607).exp();
            (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9, ) = (assign10480_e9608, (assign10480_e9608 * (-locals.var_spsub_eta_dn4)), (assign10480_e9608 * (-locals.var_spsub_eta_dn6)), (assign10480_e9608 * (-locals.var_spsub_eta_dn7)), (assign10480_e9608 * (-locals.var_spsub_eta_dn8)), (assign10480_e9608 * (-locals.var_spsub_eta_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10490_e9622: f64 = (locals.var_spsub_eta * locals.var_spsub_eta);
            let assign10490_e9623: f64 = (2.0 + assign10490_e9622);
            let assign10490_e9624: f64 = (1.0 / assign10490_e9623);
            (locals.var_spsub_temp2, locals.var_spsub_temp2_dn4, locals.var_spsub_temp2_dn6, locals.var_spsub_temp2_dn7, locals.var_spsub_temp2_dn8, locals.var_spsub_temp2_dn9, ) = (assign10490_e9624, (-(((locals.var_spsub_eta_dn4 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn4)) / (assign10490_e9623 * assign10490_e9623))), (-(((locals.var_spsub_eta_dn6 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn6)) / (assign10490_e9623 * assign10490_e9623))), (-(((locals.var_spsub_eta_dn7 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn7)) / (assign10490_e9623 * assign10490_e9623))), (-(((locals.var_spsub_eta_dn8 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn8)) / (assign10490_e9623 * assign10490_e9623))), (-(((locals.var_spsub_eta_dn9 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn9)) / (assign10490_e9623 * assign10490_e9623))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10500_e9636: f64 = (locals.var_spsub_eta * locals.var_spsub_eta);
            let assign10500_e9638: f64 = (assign10500_e9636 * locals.var_spsub_temp2);
            (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9, ) = (assign10500_e9638, ((((locals.var_spsub_eta_dn4 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn4)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn4)), ((((locals.var_spsub_eta_dn6 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn6)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn6)), ((((locals.var_spsub_eta_dn7 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn7)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn7)), ((((locals.var_spsub_eta_dn8 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn8)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn8)), ((((locals.var_spsub_eta_dn9 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn9)) * locals.var_spsub_temp2) + (assign10500_e9636 * locals.var_spsub_temp2_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10510_e9651: f64 = (locals.var_spsub_eta * locals.var_spsub_temp2);
            let assign10510_e9653: f64 = (assign10510_e9651 * locals.var_spsub_temp2);
            let assign10510_e9654: f64 = (4.0 * assign10510_e9653);
            (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9, ) = (assign10510_e9654, (4.0 * ((((locals.var_spsub_eta_dn4 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn4)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn4))), (4.0 * ((((locals.var_spsub_eta_dn6 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn6)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn6))), (4.0 * ((((locals.var_spsub_eta_dn7 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn7)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn7))), (4.0 * ((((locals.var_spsub_eta_dn8 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn8)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn8))), (4.0 * ((((locals.var_spsub_eta_dn9 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn9)) * locals.var_spsub_temp2) + (assign10510_e9651 * locals.var_spsub_temp2_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10520_e9666: f64 = (8.0 * locals.var_spsub_temp2);
            let assign10520_e9669: f64 = (12.0 * locals.var_spsub_xi0);
            let assign10520_e9670: f64 = (assign10520_e9666 - assign10520_e9669);
            let assign10520_e9672: f64 = (assign10520_e9670 * locals.var_spsub_temp2);
            let assign10520_e9674: f64 = (assign10520_e9672 * locals.var_spsub_temp2);
            (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9, ) = (assign10520_e9674, ((((((8.0 * locals.var_spsub_temp2_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn4)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn4)), ((((((8.0 * locals.var_spsub_temp2_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn6)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn6)), ((((((8.0 * locals.var_spsub_temp2_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn7)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn7)), ((((((8.0 * locals.var_spsub_temp2_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn8)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn8)), ((((((8.0 * locals.var_spsub_temp2_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp2) + (assign10520_e9670 * locals.var_spsub_temp2_dn9)) * locals.var_spsub_temp2) + (assign10520_e9672 * locals.var_spsub_temp2_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10530_e9687: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
            let assign10530_e9691: f64 = (locals.var_spsub_temp1 + locals.var_spsub_eta);
            let assign10530_e9693: f64 = (assign10530_e9691 - 1.0);
            let assign10530_e9697: f64 = (locals.var_spsub_eta + 1.0);
            let assign10530_e9699: f64 = (assign10530_e9697 + locals.var_spsub_xi0);
            let assign10530_e9700: f64 = (locals.var_spsub_delta * assign10530_e9699);
            let assign10530_e9701: f64 = (assign10530_e9693 - assign10530_e9700);
            let assign10530_e9702: f64 = (locals.var_gfsub2 * assign10530_e9701);
            let assign10530_e9703: f64 = (assign10530_e9687 - assign10530_e9702);
            let assign10530_e9704: f64 = (1e-40_f64).max(assign10530_e9703);
            (locals.var_spsub_a, locals.var_spsub_a_dn4, locals.var_spsub_a_dn6, locals.var_spsub_a_dn7, locals.var_spsub_a_dn8, locals.var_spsub_a_dn9, ) = (assign10530_e9704, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn4 + locals.var_spsub_eta_dn4) - ((locals.var_spsub_delta_dn4 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn4 + locals.var_spsub_xi0_dn4))))))) }, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn6 + locals.var_spsub_eta_dn6) - ((locals.var_spsub_delta_dn6 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn6 + locals.var_spsub_xi0_dn6))))))) }, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn7 + locals.var_spsub_eta_dn7) - ((locals.var_spsub_delta_dn7 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn7 + locals.var_spsub_xi0_dn7))))))) }, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn8 + locals.var_spsub_eta_dn8) - ((locals.var_spsub_delta_dn8 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn8 + locals.var_spsub_xi0_dn8))))))) }, if 1e-40 >= assign10530_e9703 { 0.0 } else { (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10530_e9701) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn9 + locals.var_spsub_eta_dn9) - ((locals.var_spsub_delta_dn9 * assign10530_e9699) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn9 + locals.var_spsub_xi0_dn9))))))) }, );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10540_e9720: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
            let assign10540_e9721: f64 = (locals.var_spsub_temp1 - assign10540_e9720);
            let assign10540_e9722: f64 = (locals.var_gfsub2 * assign10540_e9721);
            let assign10540_e9723: f64 = (0.5 * assign10540_e9722);
            let assign10540_e9724: f64 = (1.0 - assign10540_e9723);
            (locals.var_spsub_b, locals.var_spsub_b_dn4, locals.var_spsub_b_dn6, locals.var_spsub_b_dn7, locals.var_spsub_b_dn8, locals.var_spsub_b_dn9, ) = (assign10540_e9724, (-(0.5 * ((locals.var_gfsub2_dn4 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn4 - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4))))))), (-(0.5 * ((locals.var_gfsub2_dn6 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn6 - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6))))))), (-(0.5 * ((locals.var_gfsub2_dn7 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn7 - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7))))))), (-(0.5 * ((locals.var_gfsub2_dn8 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn8 - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8))))))), (-(0.5 * ((locals.var_gfsub2_dn9 * assign10540_e9721) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn9 - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9))))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10550_e9736: f64 = (2.0 * locals.var_spsub_temp);
            let assign10550_e9740: f64 = (1.0 - locals.var_spsub_temp1);
            let assign10550_e9744: f64 = (1.0 + locals.var_spsub_xi1);
            let assign10550_e9745: f64 = (locals.var_spsub_delta * assign10550_e9744);
            let assign10550_e9746: f64 = (assign10550_e9740 - assign10550_e9745);
            let assign10550_e9747: f64 = (locals.var_gfsub2 * assign10550_e9746);
            let assign10550_e9748: f64 = (assign10550_e9736 + assign10550_e9747);
            (locals.var_spsub_c, locals.var_spsub_c_dn4, locals.var_spsub_c_dn6, locals.var_spsub_c_dn7, locals.var_spsub_c_dn8, locals.var_spsub_c_dn9, ) = (assign10550_e9748, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn4) - ((locals.var_spsub_delta_dn4 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn4)))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn6) - ((locals.var_spsub_delta_dn6 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn6)))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn7) - ((locals.var_spsub_delta_dn7 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn7)))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn8) - ((locals.var_spsub_delta_dn8 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn8)))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10550_e9746) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn9) - ((locals.var_spsub_delta_dn9 * assign10550_e9744) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10560_e9760: f64 = (locals.var_xn_sub - locals.var_spsub_eta);
            let assign10560_e9763: f64 = (locals.var_spsub_a / locals.var_gfsub2);
            let assign10560_e9764: f64 = (assign10560_e9763).ln();
            let assign10560_e9765: f64 = (assign10560_e9760 + assign10560_e9764);
            (locals.var_spsub_tau, locals.var_spsub_tau_dn4, locals.var_spsub_tau_dn6, locals.var_spsub_tau_dn7, locals.var_spsub_tau_dn8, locals.var_spsub_tau_dn9, ) = (assign10560_e9765, ((locals.var_xn_sub_dn4 - locals.var_spsub_eta_dn4) + ((((locals.var_spsub_a_dn4 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn4)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), ((locals.var_xn_sub_dn6 - locals.var_spsub_eta_dn6) + ((((locals.var_spsub_a_dn6 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn6)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), ((locals.var_xn_sub_dn7 - locals.var_spsub_eta_dn7) + ((((locals.var_spsub_a_dn7 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn7)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), ((locals.var_xn_sub_dn8 - locals.var_spsub_eta_dn8) + ((((locals.var_spsub_a_dn8 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn8)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), ((locals.var_xn_sub_dn9 - locals.var_spsub_eta_dn9) + ((((locals.var_spsub_a_dn9 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn9)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10560_e9763)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10570_e9777: f64 = (locals.var_spsub_a + locals.var_spsub_c);
            (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, ) = (assign10570_e9777, (locals.var_spsub_a_dn4 + locals.var_spsub_c_dn4), (locals.var_spsub_a_dn6 + locals.var_spsub_c_dn6), (locals.var_spsub_a_dn7 + locals.var_spsub_c_dn7), (locals.var_spsub_a_dn8 + locals.var_spsub_c_dn8), (locals.var_spsub_a_dn9 + locals.var_spsub_c_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10580_e9789: f64 = (locals.var_nu * locals.var_nu);
            let assign10580_e9793: f64 = (0.5 * locals.var_spsub_c);
            let assign10580_e9795: f64 = (assign10580_e9793 * locals.var_spsub_c);
            let assign10580_e9798: f64 = (locals.var_spsub_a * locals.var_spsub_b);
            let assign10580_e9799: f64 = (assign10580_e9795 - assign10580_e9798);
            let assign10580_e9800: f64 = (locals.var_spsub_tau * assign10580_e9799);
            let assign10580_e9801: f64 = (assign10580_e9789 + assign10580_e9800);
            (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, ) = (assign10580_e9801, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_spsub_tau_dn4 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn4) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn4)) - ((locals.var_spsub_a_dn4 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_spsub_tau_dn6 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn6) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn6)) - ((locals.var_spsub_a_dn6 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_spsub_tau_dn7 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn7) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn7)) - ((locals.var_spsub_a_dn7 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_spsub_tau_dn8 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn8) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn8)) - ((locals.var_spsub_a_dn8 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_spsub_tau_dn9 * assign10580_e9799) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn9) * locals.var_spsub_c) + (assign10580_e9793 * locals.var_spsub_c_dn9)) - ((locals.var_spsub_a_dn9 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10590_e9814: f64 = (locals.var_spsub_a * locals.var_nu);
            let assign10590_e9816: f64 = (assign10590_e9814 * locals.var_spsub_tau);
            let assign10590_e9820: f64 = (locals.var_nu / locals.var_mutau);
            let assign10590_e9822: f64 = (assign10590_e9820 * locals.var_spsub_tau);
            let assign10590_e9824: f64 = (assign10590_e9822 * locals.var_spsub_tau);
            let assign10590_e9826: f64 = (assign10590_e9824 * locals.var_spsub_c);
            let assign10590_e9829: f64 = (locals.var_spsub_c * locals.var_spsub_c);
            let assign10590_e9831: f64 = (assign10590_e9829 * 0.3333333333333);
            let assign10590_e9834: f64 = (locals.var_spsub_a * locals.var_spsub_b);
            let assign10590_e9835: f64 = (assign10590_e9831 - assign10590_e9834);
            let assign10590_e9836: f64 = (assign10590_e9826 * assign10590_e9835);
            let assign10590_e9837: f64 = (locals.var_mutau + assign10590_e9836);
            let assign10590_e9838: f64 = (assign10590_e9816 / assign10590_e9837);
            let assign10590_e9839: f64 = (locals.var_spsub_eta + assign10590_e9838);
            (locals.var_spsub_x0, locals.var_spsub_x0_dn4, locals.var_spsub_x0_dn6, locals.var_spsub_x0_dn7, locals.var_spsub_x0_dn8, locals.var_spsub_x0_dn9, ) = (assign10590_e9839, (locals.var_spsub_eta_dn4 + (((((((locals.var_spsub_a_dn4 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn4)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn4)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn4)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn4)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn4)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn4 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn4)) * 0.3333333333333) - ((locals.var_spsub_a_dn4 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn4)))))))) / (assign10590_e9837 * assign10590_e9837))), (locals.var_spsub_eta_dn6 + (((((((locals.var_spsub_a_dn6 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn6)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn6)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn6)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn6)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn6)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn6 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn6)) * 0.3333333333333) - ((locals.var_spsub_a_dn6 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn6)))))))) / (assign10590_e9837 * assign10590_e9837))), (locals.var_spsub_eta_dn7 + (((((((locals.var_spsub_a_dn7 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn7)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn7)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn7)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn7)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn7)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn7 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn7)) * 0.3333333333333) - ((locals.var_spsub_a_dn7 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn7)))))))) / (assign10590_e9837 * assign10590_e9837))), (locals.var_spsub_eta_dn8 + (((((((locals.var_spsub_a_dn8 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn8)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn8)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn8)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn8)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn8)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn8 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn8)) * 0.3333333333333) - ((locals.var_spsub_a_dn8 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn8)))))))) / (assign10590_e9837 * assign10590_e9837))), (locals.var_spsub_eta_dn9 + (((((((locals.var_spsub_a_dn9 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn9)) * locals.var_spsub_tau) + (assign10590_e9814 * locals.var_spsub_tau_dn9)) * assign10590_e9837) - (assign10590_e9816 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10590_e9820 * locals.var_spsub_tau_dn9)) * locals.var_spsub_tau) + (assign10590_e9822 * locals.var_spsub_tau_dn9)) * locals.var_spsub_c) + (assign10590_e9824 * locals.var_spsub_c_dn9)) * assign10590_e9835) + (assign10590_e9826 * ((((locals.var_spsub_c_dn9 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn9)) * 0.3333333333333) - ((locals.var_spsub_a_dn9 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn9)))))))) / (assign10590_e9837 * assign10590_e9837))), );
        }

        let assign10600_e9844: f64 = if locals.var_spsub_x0 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign10600_e9844;

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
            let assign10610_e9855: f64 = (locals.var_spsub_x0).exp();
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10610_e9855, (assign10610_e9855 * locals.var_spsub_x0_dn4), (assign10610_e9855 * locals.var_spsub_x0_dn6), (assign10610_e9855 * locals.var_spsub_x0_dn7), (assign10610_e9855 * locals.var_spsub_x0_dn8), (assign10610_e9855 * locals.var_spsub_x0_dn9), );
        }

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
            let assign10620_e9869: f64 = (1.0 / locals.var_spsub_delta0);
            (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9, ) = (assign10620_e9869, (-(locals.var_spsub_delta0_dn4 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn6 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn7 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn8 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn9 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), );
        }

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
            let assign10630_e9883: f64 = (locals.var_spsub_delta * locals.var_spsub_delta0);
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10630_e9883, ((locals.var_spsub_delta_dn4 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn4)), ((locals.var_spsub_delta_dn6 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn6)), ((locals.var_spsub_delta_dn7 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn7)), ((locals.var_spsub_delta_dn8 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn8)), ((locals.var_spsub_delta_dn9 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn9)), );
        }

        let assign10640_e9889: f64 = (locals.var_xn_sub - 80.0);
        let assign10640_e9890: f64 = if locals.var_spsub_x0 > assign10640_e9889 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign10640_e9890;

        if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 != 0.0)) {
            let assign10650_e9905: f64 = (locals.var_spsub_x0 - locals.var_xn_sub);
            let assign10650_e9906: f64 = (assign10650_e9905).exp();
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10650_e9906, (assign10650_e9906 * (locals.var_spsub_x0_dn4 - locals.var_xn_sub_dn4)), (assign10650_e9906 * (locals.var_spsub_x0_dn6 - locals.var_xn_sub_dn6)), (assign10650_e9906 * (locals.var_spsub_x0_dn7 - locals.var_xn_sub_dn7)), (assign10650_e9906 * (locals.var_spsub_x0_dn8 - locals.var_xn_sub_dn8)), (assign10650_e9906 * (locals.var_spsub_x0_dn9 - locals.var_xn_sub_dn9)), );
        }

        if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 != 0.0)) {
            let assign10660_e9923: f64 = (locals.var_spsub_delta / locals.var_spsub_delta0);
            (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9, ) = (assign10660_e9923, (((locals.var_spsub_delta_dn4 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn4)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn6 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn6)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn7 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn7)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn8 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn8)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn9 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn9)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), );
        }

        if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 == 0.0)) {
            let assign10670_e9943: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
            let assign10670_e9945: f64 = (assign10670_e9943 - 80.0);
            let assign10670_e9950: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
            let assign10670_e9952: f64 = (assign10670_e9950 - 80.0);
            let assign10670_e9953: f64 = (0.5 * assign10670_e9952);
            let assign10670_e9957: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
            let assign10670_e9959: f64 = (assign10670_e9957 - 80.0);
            let assign10670_e9961: f64 = (assign10670_e9959 * 0.3333333333333);
            let assign10670_e9962: f64 = (1.0 + assign10670_e9961);
            let assign10670_e9963: f64 = (assign10670_e9953 * assign10670_e9962);
            let assign10670_e9964: f64 = (1.0 + assign10670_e9963);
            let assign10670_e9965: f64 = (assign10670_e9945 * assign10670_e9964);
            let assign10670_e9966: f64 = (1.0 + assign10670_e9965);
            let assign10670_e9967: f64 = (1.80485e-35 / assign10670_e9966);
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10670_e9967, (-((1.80485e-35 * (((locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), (-((1.80485e-35 * (((locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), (-((1.80485e-35 * (((locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), (-((1.80485e-35 * (((locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), (-((1.80485e-35 * (((locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9) * assign10670_e9964) + (assign10670_e9945 * (((0.5 * (locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9)) * assign10670_e9962) + (assign10670_e9953 * ((locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9) * 0.3333333333333)))))) / (assign10670_e9966 * assign10670_e9966))), );
        }

        if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 == 0.0)) {
            let assign10680_e9987: f64 = (locals.var_spsub_x0 - 80.0);
            let assign10680_e9992: f64 = (locals.var_spsub_x0 - 80.0);
            let assign10680_e9993: f64 = (0.5 * assign10680_e9992);
            let assign10680_e9997: f64 = (locals.var_spsub_x0 - 80.0);
            let assign10680_e9999: f64 = (assign10680_e9997 * 0.3333333333333);
            let assign10680_e10000: f64 = (1.0 + assign10680_e9999);
            let assign10680_e10001: f64 = (assign10680_e9993 * assign10680_e10000);
            let assign10680_e10002: f64 = (1.0 + assign10680_e10001);
            let assign10680_e10003: f64 = (assign10680_e9987 * assign10680_e10002);
            let assign10680_e10004: f64 = (1.0 + assign10680_e10003);
            let assign10680_e10005: f64 = (1.80485e-35 / assign10680_e10004);
            (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9, ) = (assign10680_e10005, (-((1.80485e-35 * ((locals.var_spsub_x0_dn4 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn4) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn4 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn6 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn6) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn6 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn7 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn7) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn7 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn8 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn8) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn8 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn9 * assign10680_e10002) + (assign10680_e9987 * (((0.5 * locals.var_spsub_x0_dn9) * assign10680_e10000) + (assign10680_e9993 * (locals.var_spsub_x0_dn9 * 0.3333333333333)))))) / (assign10680_e10004 * assign10680_e10004))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10690_e10019: f64 = (locals.var_spsub_x0 * locals.var_spsub_x0);
            let assign10690_e10020: f64 = (2.0 + assign10690_e10019);
            let assign10690_e10021: f64 = (1.0 / assign10690_e10020);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10690_e10021, (-(((locals.var_spsub_x0_dn4 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn4)) / (assign10690_e10020 * assign10690_e10020))), (-(((locals.var_spsub_x0_dn6 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn6)) / (assign10690_e10020 * assign10690_e10020))), (-(((locals.var_spsub_x0_dn7 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn7)) / (assign10690_e10020 * assign10690_e10020))), (-(((locals.var_spsub_x0_dn8 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn8)) / (assign10690_e10020 * assign10690_e10020))), (-(((locals.var_spsub_x0_dn9 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn9)) / (assign10690_e10020 * assign10690_e10020))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10700_e10033: f64 = (locals.var_spsub_x0 * locals.var_spsub_x0);
            let assign10700_e10035: f64 = (assign10700_e10033 * locals.var_spsub_temp);
            (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9, ) = (assign10700_e10035, ((((locals.var_spsub_x0_dn4 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn4)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn4)), ((((locals.var_spsub_x0_dn6 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn6)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn6)), ((((locals.var_spsub_x0_dn7 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn7)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn7)), ((((locals.var_spsub_x0_dn8 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn8)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn8)), ((((locals.var_spsub_x0_dn9 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn9)) * locals.var_spsub_temp) + (assign10700_e10033 * locals.var_spsub_temp_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10710_e10048: f64 = (locals.var_spsub_x0 * locals.var_spsub_temp);
            let assign10710_e10050: f64 = (assign10710_e10048 * locals.var_spsub_temp);
            let assign10710_e10051: f64 = (4.0 * assign10710_e10050);
            (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9, ) = (assign10710_e10051, (4.0 * ((((locals.var_spsub_x0_dn4 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn4))), (4.0 * ((((locals.var_spsub_x0_dn6 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn6))), (4.0 * ((((locals.var_spsub_x0_dn7 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn7))), (4.0 * ((((locals.var_spsub_x0_dn8 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn8))), (4.0 * ((((locals.var_spsub_x0_dn9 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10710_e10048 * locals.var_spsub_temp_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10720_e10063: f64 = (8.0 * locals.var_spsub_temp);
            let assign10720_e10066: f64 = (12.0 * locals.var_spsub_xi0);
            let assign10720_e10067: f64 = (assign10720_e10063 - assign10720_e10066);
            let assign10720_e10069: f64 = (assign10720_e10067 * locals.var_spsub_temp);
            let assign10720_e10071: f64 = (assign10720_e10069 * locals.var_spsub_temp);
            (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9, ) = (assign10720_e10071, ((((((8.0 * locals.var_spsub_temp_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn4)), ((((((8.0 * locals.var_spsub_temp_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn6)), ((((((8.0 * locals.var_spsub_temp_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn7)), ((((((8.0 * locals.var_spsub_temp_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn8)), ((((((8.0 * locals.var_spsub_temp_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp) + (assign10720_e10067 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10720_e10069 * locals.var_spsub_temp_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10730_e10083: f64 = (locals.var_spsub_xgb - locals.var_spsub_x0);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10730_e10083, (locals.var_spsub_xgb_dn4 - locals.var_spsub_x0_dn4), (locals.var_spsub_xgb_dn6 - locals.var_spsub_x0_dn6), (locals.var_spsub_xgb_dn7 - locals.var_spsub_x0_dn7), (locals.var_spsub_xgb_dn8 - locals.var_spsub_x0_dn8), (locals.var_spsub_xgb_dn9 - locals.var_spsub_x0_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10740_e10095: f64 = (2.0 * locals.var_spsub_temp);
            let assign10740_e10099: f64 = (1.0 - locals.var_spsub_delta1);
            let assign10740_e10101: f64 = (assign10740_e10099 + locals.var_spsub_delta0);
            let assign10740_e10105: f64 = (1.0 + locals.var_spsub_xi1);
            let assign10740_e10106: f64 = (locals.var_spsub_delta * assign10740_e10105);
            let assign10740_e10107: f64 = (assign10740_e10101 - assign10740_e10106);
            let assign10740_e10108: f64 = (locals.var_gfsub2 * assign10740_e10107);
            let assign10740_e10109: f64 = (assign10740_e10095 + assign10740_e10108);
            (locals.var_spsub_pc, locals.var_spsub_pc_dn4, locals.var_spsub_pc_dn6, locals.var_spsub_pc_dn7, locals.var_spsub_pc_dn8, locals.var_spsub_pc_dn9, ) = (assign10740_e10109, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn4) + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn4)))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn6) + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn6)))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn7) + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn7)))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn8) + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn8)))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10740_e10107) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn9) + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * assign10740_e10105) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10750_e10121: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
            let assign10750_e10125: f64 = (locals.var_spsub_delta1 + locals.var_spsub_x0);
            let assign10750_e10127: f64 = (assign10750_e10125 - 1.0);
            let assign10750_e10129: f64 = (assign10750_e10127 + locals.var_spsub_delta0);
            let assign10750_e10133: f64 = (locals.var_spsub_x0 + 1.0);
            let assign10750_e10135: f64 = (assign10750_e10133 + locals.var_spsub_xi0);
            let assign10750_e10136: f64 = (locals.var_spsub_delta * assign10750_e10135);
            let assign10750_e10137: f64 = (assign10750_e10129 - assign10750_e10136);
            let assign10750_e10138: f64 = (locals.var_gfsub2 * assign10750_e10137);
            let assign10750_e10139: f64 = (assign10750_e10121 - assign10750_e10138);
            (locals.var_spsub_qc, locals.var_spsub_qc_dn4, locals.var_spsub_qc_dn6, locals.var_spsub_qc_dn7, locals.var_spsub_qc_dn8, locals.var_spsub_qc_dn9, ) = (assign10750_e10139, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn4 + locals.var_spsub_x0_dn4) + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn4 + locals.var_spsub_xi0_dn4))))))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn6 + locals.var_spsub_x0_dn6) + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn6 + locals.var_spsub_xi0_dn6))))))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn7 + locals.var_spsub_x0_dn7) + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn7 + locals.var_spsub_xi0_dn7))))))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn8 + locals.var_spsub_x0_dn8) + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn8 + locals.var_spsub_xi0_dn8))))))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10750_e10137) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn9 + locals.var_spsub_x0_dn9) + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * assign10750_e10135) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn9 + locals.var_spsub_xi0_dn9))))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10760_e10153: f64 = (locals.var_spsub_delta1 + locals.var_spsub_delta0);
            let assign10760_e10156: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
            let assign10760_e10157: f64 = (assign10760_e10153 - assign10760_e10156);
            let assign10760_e10158: f64 = (locals.var_gfsub2 * assign10760_e10157);
            let assign10760_e10159: f64 = (2.0 - assign10760_e10158);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10760_e10159, (-((locals.var_gfsub2_dn4 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn4 + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4)))))), (-((locals.var_gfsub2_dn6 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn6 + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6)))))), (-((locals.var_gfsub2_dn7 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn7 + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7)))))), (-((locals.var_gfsub2_dn8 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn8 + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8)))))), (-((locals.var_gfsub2_dn9 * assign10760_e10157) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn9 + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10770_e10171: f64 = (locals.var_spsub_pc * locals.var_spsub_pc);
            let assign10770_e10175: f64 = (locals.var_spsub_qc * locals.var_spsub_temp);
            let assign10770_e10176: f64 = (2.0 * assign10770_e10175);
            let assign10770_e10177: f64 = (assign10770_e10171 - assign10770_e10176);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10770_e10177, (((locals.var_spsub_pc_dn4 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn4)) - (2.0 * ((locals.var_spsub_qc_dn4 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn4)))), (((locals.var_spsub_pc_dn6 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn6)) - (2.0 * ((locals.var_spsub_qc_dn6 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn6)))), (((locals.var_spsub_pc_dn7 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn7)) - (2.0 * ((locals.var_spsub_qc_dn7 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn7)))), (((locals.var_spsub_pc_dn8 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn8)) - (2.0 * ((locals.var_spsub_qc_dn8 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn8)))), (((locals.var_spsub_pc_dn9 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn9)) - (2.0 * ((locals.var_spsub_qc_dn9 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn9)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10780_e10192: f64 = (locals.var_spsub_temp).sqrt();
            let assign10780_e10193: f64 = (locals.var_spsub_pc + assign10780_e10192);
            let assign10780_e10194: f64 = (locals.var_spsub_qc / assign10780_e10193);
            let assign10780_e10195: f64 = (2.0 * assign10780_e10194);
            let assign10780_e10196: f64 = (locals.var_spsub_x0 + assign10780_e10195);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign10780_e10196, (locals.var_spsub_x0_dn4 + (2.0 * (((locals.var_spsub_qc_dn4 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn4 + (locals.var_spsub_temp_dn4 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), (locals.var_spsub_x0_dn6 + (2.0 * (((locals.var_spsub_qc_dn6 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn6 + (locals.var_spsub_temp_dn6 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), (locals.var_spsub_x0_dn7 + (2.0 * (((locals.var_spsub_qc_dn7 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn7 + (locals.var_spsub_temp_dn7 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), (locals.var_spsub_x0_dn8 + (2.0 * (((locals.var_spsub_qc_dn8 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn8 + (locals.var_spsub_temp_dn8 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), (locals.var_spsub_x0_dn9 + (2.0 * (((locals.var_spsub_qc_dn9 * assign10780_e10193) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn9 + (locals.var_spsub_temp_dn9 / (2.0 * assign10780_e10192))))) / (assign10780_e10193 * assign10780_e10193)))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign10790_e10203: f64 = (locals.var_temp3 + locals.var_temp2);
            let assign10790_e10204: f64 = (locals.var_temp * assign10790_e10203);
            (locals.var_xg2eff, locals.var_xg2eff_dn4, locals.var_xg2eff_dn6, locals.var_xg2eff_dn7, locals.var_xg2eff_dn8, locals.var_xg2eff_dn9, ) = (assign10790_e10204, ((locals.var_temp_dn4 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn4 + locals.var_temp2_dn4))), ((locals.var_temp_dn6 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn6 + locals.var_temp2_dn6))), ((locals.var_temp_dn7 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn7 + locals.var_temp2_dn7))), ((locals.var_temp_dn8 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn8 + locals.var_temp2_dn8))), ((locals.var_temp_dn9 * assign10790_e10203) + (locals.var_temp * (locals.var_temp3_dn9 + locals.var_temp2_dn9))), );
        }

        if (locals.var_guard531 == 0.0) {
            (locals.var_xg2eff, locals.var_xg2eff_dn4, locals.var_xg2eff_dn6, locals.var_xg2eff_dn7, locals.var_xg2eff_dn8, locals.var_xg2eff_dn9, ) = (locals.var_xg20, locals.var_xg20_dn4, locals.var_xg20_dn6, locals.var_xg20_dn7, locals.var_xg20_dn8, locals.var_xg20_dn9, );
        }

        let assign10810_e10215: f64 = (locals.var_xg10 - locals.var_xg2eff);
        let assign10810_e10216: f64 = (locals.var_keq_1d * assign10810_e10215);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign10810_e10216, (locals.var_keq_1d * (locals.var_xg10_dn4 - locals.var_xg2eff_dn4)), (locals.var_keq_1d * (locals.var_xg10_dn6 - locals.var_xg2eff_dn6)), (locals.var_keq_1d * (locals.var_xg10_dn7 - locals.var_xg2eff_dn7)), (locals.var_keq_1d * (locals.var_xg10_dn8 - locals.var_xg2eff_dn8)), (locals.var_keq_1d * (locals.var_xg10_dn9 - locals.var_xg2eff_dn9)), );

        let assign10820_e10219: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign10820_e10219;

        if (locals.var_guard543 != 0.0) {
            let assign10830_e10224: f64 = (locals.var_temp + locals.var_emin);
            let assign10830_e10227: f64 = (locals.var_temp - locals.var_emin);
            let assign10830_e10230: f64 = (locals.var_temp - locals.var_emin);
            let assign10830_e10231: f64 = (assign10830_e10227 * assign10830_e10230);
            let assign10830_e10234: f64 = (locals.var_emin * locals.var_emin);
            let assign10830_e10235: f64 = (assign10830_e10231 + assign10830_e10234);
            let assign10830_e10236: f64 = (assign10830_e10235).sqrt();
            let assign10830_e10237: f64 = (assign10830_e10224 + assign10830_e10236);
            let assign10830_e10238: f64 = (0.5 * assign10830_e10237);
            (locals.var_e1, locals.var_e1_dn4, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, ) = (assign10830_e10238, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign10830_e10236)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign10830_e10236)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign10830_e10236)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign10830_e10236)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign10830_e10230) + (assign10830_e10227 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign10830_e10236)))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10840_e10244: f64 = (-locals.var_temp);
            let assign10840_e10246: f64 = (assign10840_e10244 + locals.var_emin);
            let assign10840_e10248: f64 = (-locals.var_temp);
            let assign10840_e10250: f64 = (assign10840_e10248 - locals.var_emin);
            let assign10840_e10252: f64 = (-locals.var_temp);
            let assign10840_e10254: f64 = (assign10840_e10252 - locals.var_emin);
            let assign10840_e10255: f64 = (assign10840_e10250 * assign10840_e10254);
            let assign10840_e10258: f64 = (locals.var_emin * locals.var_emin);
            let assign10840_e10259: f64 = (assign10840_e10255 + assign10840_e10258);
            let assign10840_e10260: f64 = (assign10840_e10259).sqrt();
            let assign10840_e10261: f64 = (assign10840_e10246 + assign10840_e10260);
            let assign10840_e10262: f64 = (0.5 * assign10840_e10261);
            (locals.var_e2, locals.var_e2_dn4, locals.var_e2_dn6, locals.var_e2_dn7, locals.var_e2_dn8, locals.var_e2_dn9, ) = (assign10840_e10262, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign10840_e10260)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign10840_e10260)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign10840_e10260)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign10840_e10260)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign10840_e10254) + (assign10840_e10250 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign10840_e10260)))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10850_e10268: f64 = (-0.3333333333333);
            let assign10850_e10270: f64 = (locals.var_e1).ln();
            let assign10850_e10271: f64 = (assign10850_e10268 * assign10850_e10270);
            let assign10850_e10272: f64 = (assign10850_e10271).exp();
            let assign10850_e10273: f64 = (locals.var_qq * assign10850_e10272);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign10850_e10273, ((locals.var_qq_dn4 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn4 / locals.var_e1))))), ((locals.var_qq_dn6 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn6 / locals.var_e1))))), ((locals.var_qq_dn7 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn7 / locals.var_e1))))), ((locals.var_qq_dn8 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn8 / locals.var_e1))))), ((locals.var_qq_dn9 * assign10850_e10272) + (locals.var_qq * (assign10850_e10272 * (assign10850_e10268 * (locals.var_e1_dn9 / locals.var_e1))))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10860_e10279: f64 = (-0.3333333333333);
            let assign10860_e10281: f64 = (locals.var_e2).ln();
            let assign10860_e10282: f64 = (assign10860_e10279 * assign10860_e10281);
            let assign10860_e10283: f64 = (assign10860_e10282).exp();
            let assign10860_e10284: f64 = (locals.var_qq * assign10860_e10283);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign10860_e10284, ((locals.var_qq_dn4 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn4 / locals.var_e2))))), ((locals.var_qq_dn6 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn6 / locals.var_e2))))), ((locals.var_qq_dn7 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn7 / locals.var_e2))))), ((locals.var_qq_dn8 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn8 / locals.var_e2))))), ((locals.var_qq_dn9 * assign10860_e10283) + (locals.var_qq * (assign10860_e10283 * (assign10860_e10279 * (locals.var_e2_dn9 / locals.var_e2))))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10870_e10290: f64 = (1.0 - locals.var_temp1);
            let assign10870_e10292: f64 = (assign10870_e10290 - locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign10870_e10292, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10880_e10298: f64 = (locals.var_csiprime_0 / locals.var_temp3);
            (locals.var_csiprime, locals.var_csiprime_dn4, locals.var_csiprime_dn6, locals.var_csiprime_dn7, locals.var_csiprime_dn8, locals.var_csiprime_dn9, ) = (assign10880_e10298, (-((locals.var_csiprime_0 * locals.var_temp3_dn4) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn6) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn7) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn8) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn9) / (locals.var_temp3 * locals.var_temp3))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10890_e10305: f64 = (locals.var_k1_1d * locals.var_temp1);
            let assign10890_e10306: f64 = (1.0 + assign10890_e10305);
            (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9, ) = (assign10890_e10306, (locals.var_k1_1d * locals.var_temp1_dn4), (locals.var_k1_1d * locals.var_temp1_dn6), (locals.var_k1_1d * locals.var_temp1_dn7), (locals.var_k1_1d * locals.var_temp1_dn8), (locals.var_k1_1d * locals.var_temp1_dn9), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10900_e10313: f64 = (locals.var_k2_1d * locals.var_temp2);
            let assign10900_e10314: f64 = (1.0 + assign10900_e10313);
            (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9, ) = (assign10900_e10314, (locals.var_k2_1d * locals.var_temp2_dn4), (locals.var_k2_1d * locals.var_temp2_dn6), (locals.var_k2_1d * locals.var_temp2_dn7), (locals.var_k2_1d * locals.var_temp2_dn8), (locals.var_k2_1d * locals.var_temp2_dn9), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10910_e10320: f64 = (locals.var_k1_1d * locals.var_temp3);
            let assign10910_e10322: f64 = (assign10910_e10320 / locals.var_tox1fact);
            (locals.var_k1_1d_qm, locals.var_k1_1d_qm_dn4, locals.var_k1_1d_qm_dn6, locals.var_k1_1d_qm_dn7, locals.var_k1_1d_qm_dn8, locals.var_k1_1d_qm_dn9, ) = (assign10910_e10322, ((((locals.var_k1_1d * locals.var_temp3_dn4) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn4)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn6)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn7)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn8)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * locals.var_tox1fact) - (assign10910_e10320 * locals.var_tox1fact_dn9)) / (locals.var_tox1fact * locals.var_tox1fact)), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10920_e10328: f64 = (locals.var_k2_1d * locals.var_temp3);
            let assign10920_e10330: f64 = (assign10920_e10328 / locals.var_tox2fact);
            (locals.var_k2_1d_qm, locals.var_k2_1d_qm_dn4, locals.var_k2_1d_qm_dn6, locals.var_k2_1d_qm_dn7, locals.var_k2_1d_qm_dn8, locals.var_k2_1d_qm_dn9, ) = (assign10920_e10330, ((((locals.var_k2_1d * locals.var_temp3_dn4) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn4)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn6)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn7)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn8)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * locals.var_tox2fact) - (assign10920_e10328 * locals.var_tox2fact_dn9)) / (locals.var_tox2fact * locals.var_tox2fact)), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10930_e10338: f64 = (1.0 / locals.var_k1_1d_qm);
            let assign10930_e10339: f64 = (1.0 + assign10930_e10338);
            let assign10930_e10342: f64 = (1.0 / locals.var_k2_1d_qm);
            let assign10930_e10343: f64 = (assign10930_e10339 + assign10930_e10342);
            let assign10930_e10344: f64 = (1.0 / assign10930_e10343);
            (locals.var_keq_1d_qm, locals.var_keq_1d_qm_dn4, locals.var_keq_1d_qm_dn6, locals.var_keq_1d_qm_dn7, locals.var_keq_1d_qm_dn8, locals.var_keq_1d_qm_dn9, ) = (assign10930_e10344, (-(((-(locals.var_k1_1d_qm_dn4 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn4 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), (-(((-(locals.var_k1_1d_qm_dn6 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn6 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), (-(((-(locals.var_k1_1d_qm_dn7 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn7 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), (-(((-(locals.var_k1_1d_qm_dn8 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn8 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), (-(((-(locals.var_k1_1d_qm_dn9 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn9 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10930_e10343 * assign10930_e10343))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10940_e10351: f64 = (locals.var_k1_1d_qm * locals.var_temp1);
            let assign10940_e10352: f64 = (1.0 + assign10940_e10351);
            (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9, ) = (assign10940_e10352, ((locals.var_k1_1d_qm_dn4 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn4)), ((locals.var_k1_1d_qm_dn6 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn6)), ((locals.var_k1_1d_qm_dn7 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn7)), ((locals.var_k1_1d_qm_dn8 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn8)), ((locals.var_k1_1d_qm_dn9 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn9)), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10950_e10359: f64 = (locals.var_k2_1d_qm * locals.var_temp2);
            let assign10950_e10360: f64 = (1.0 + assign10950_e10359);
            (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9, ) = (assign10950_e10360, ((locals.var_k2_1d_qm_dn4 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn4)), ((locals.var_k2_1d_qm_dn6 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn6)), ((locals.var_k2_1d_qm_dn7 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn7)), ((locals.var_k2_1d_qm_dn8 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn8)), ((locals.var_k2_1d_qm_dn9 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn9)), );
        }

        if (locals.var_guard543 == 0.0) {
            (locals.var_csiprime, locals.var_csiprime_dn4, locals.var_csiprime_dn6, locals.var_csiprime_dn7, locals.var_csiprime_dn8, locals.var_csiprime_dn9, ) = (locals.var_csiprime_0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_k1_1d_qm, locals.var_k1_1d_qm_dn4, locals.var_k1_1d_qm_dn6, locals.var_k1_1d_qm_dn7, locals.var_k1_1d_qm_dn8, locals.var_k1_1d_qm_dn9, ) = (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_k2_1d_qm, locals.var_k2_1d_qm_dn4, locals.var_k2_1d_qm_dn6, locals.var_k2_1d_qm_dn7, locals.var_k2_1d_qm_dn8, locals.var_k2_1d_qm_dn9, ) = (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_keq_1d_qm, locals.var_keq_1d_qm_dn4, locals.var_keq_1d_qm_dn6, locals.var_keq_1d_qm_dn7, locals.var_keq_1d_qm_dn8, locals.var_keq_1d_qm_dn9, ) = (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign11020_e10396: f64 = (locals.var_xg10 - locals.var_xg2eff);
        let assign11020_e10397: f64 = (locals.var_keq_1d_qm * assign11020_e10396);
        (locals.var_dx_wi_1d, locals.var_dx_wi_1d_dn4, locals.var_dx_wi_1d_dn6, locals.var_dx_wi_1d_dn7, locals.var_dx_wi_1d_dn8, locals.var_dx_wi_1d_dn9, ) = (assign11020_e10397, ((locals.var_keq_1d_qm_dn4 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn4 - locals.var_xg2eff_dn4))), ((locals.var_keq_1d_qm_dn6 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn6 - locals.var_xg2eff_dn6))), ((locals.var_keq_1d_qm_dn7 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn7 - locals.var_xg2eff_dn7))), ((locals.var_keq_1d_qm_dn8 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn8 - locals.var_xg2eff_dn8))), ((locals.var_keq_1d_qm_dn9 * assign11020_e10396) + (locals.var_keq_1d_qm * (locals.var_xg10_dn9 - locals.var_xg2eff_dn9))), );

        let assign11030_e10400: f64 = if locals.var_dx_wi_1d > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign11030_e10400;

        let assign11040_e10402: f64 = (-locals.var_dx_wi_1d);
        let assign11040_e10404: f64 = if assign11040_e10402 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign11040_e10404;

        if ((locals.var_guard544 != 0.0) && (locals.var_guard545 != 0.0)) {
            let assign11050_e10410: f64 = (-locals.var_dx_wi_1d);
            let assign11050_e10411: f64 = (assign11050_e10410).exp();
            let assign11050_e10412: f64 = (1.0 + assign11050_e10411);
            let assign11050_e10413: f64 = (assign11050_e10412).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign11050_e10413, ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn4)) / assign11050_e10412), ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn6)) / assign11050_e10412), ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn7)) / assign11050_e10412), ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn8)) / assign11050_e10412), ((assign11050_e10411 * (-locals.var_dx_wi_1d_dn9)) / assign11050_e10412), );
        }

        if ((locals.var_guard544 != 0.0) && (locals.var_guard545 == 0.0)) {
            let assign11060_e10421: f64 = (-locals.var_dx_wi_1d);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign11060_e10421, (-locals.var_dx_wi_1d_dn4), (-locals.var_dx_wi_1d_dn6), (-locals.var_dx_wi_1d_dn7), (-locals.var_dx_wi_1d_dn8), (-locals.var_dx_wi_1d_dn9), );
        }

        if (locals.var_guard544 != 0.0) {
            let assign11070_e10428: f64 = (locals.var_dx_wi_1d / locals.var_k1_1d_qm);
            let assign11070_e10429: f64 = (locals.var_xg10 - assign11070_e10428);
            let assign11070_e10431: f64 = (assign11070_e10429 + locals.var_temp);
            let assign11070_e10433: f64 = (assign11070_e10431 - 0.6931471805599);
            (locals.var_x_wi_1d, locals.var_x_wi_1d_dn4, locals.var_x_wi_1d_dn6, locals.var_x_wi_1d_dn7, locals.var_x_wi_1d_dn8, locals.var_x_wi_1d_dn9, ) = (assign11070_e10433, ((locals.var_xg10_dn4 - (((locals.var_dx_wi_1d_dn4 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn4)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn4), ((locals.var_xg10_dn6 - (((locals.var_dx_wi_1d_dn6 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn6)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn6), ((locals.var_xg10_dn7 - (((locals.var_dx_wi_1d_dn7 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn7)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn7), ((locals.var_xg10_dn8 - (((locals.var_dx_wi_1d_dn8 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn8)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn8), ((locals.var_xg10_dn9 - (((locals.var_dx_wi_1d_dn9 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn9)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn9), );
        }

        let assign11080_e10438: f64 = if locals.var_dx_wi_1d < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign11080_e10438;

        if ((locals.var_guard544 == 0.0) && (locals.var_guard546 != 0.0)) {
            let assign11090_e10445: f64 = (locals.var_dx_wi_1d).exp();
            let assign11090_e10446: f64 = (1.0 + assign11090_e10445);
            let assign11090_e10447: f64 = (assign11090_e10446).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign11090_e10447, ((assign11090_e10445 * locals.var_dx_wi_1d_dn4) / assign11090_e10446), ((assign11090_e10445 * locals.var_dx_wi_1d_dn6) / assign11090_e10446), ((assign11090_e10445 * locals.var_dx_wi_1d_dn7) / assign11090_e10446), ((assign11090_e10445 * locals.var_dx_wi_1d_dn8) / assign11090_e10446), ((assign11090_e10445 * locals.var_dx_wi_1d_dn9) / assign11090_e10446), );
        }

        if ((locals.var_guard544 == 0.0) && (locals.var_guard546 == 0.0)) {
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (locals.var_dx_wi_1d, locals.var_dx_wi_1d_dn4, locals.var_dx_wi_1d_dn6, locals.var_dx_wi_1d_dn7, locals.var_dx_wi_1d_dn8, locals.var_dx_wi_1d_dn9, );
        }

        if (locals.var_guard544 == 0.0) {
            let assign11110_e10463: f64 = (locals.var_dx_wi_1d / locals.var_k2_1d_qm);
            let assign11110_e10464: f64 = (locals.var_xg2eff + assign11110_e10463);
            let assign11110_e10466: f64 = (assign11110_e10464 + locals.var_temp);
            let assign11110_e10468: f64 = (assign11110_e10466 - 0.6931471805599);
            (locals.var_x_wi_1d, locals.var_x_wi_1d_dn4, locals.var_x_wi_1d_dn6, locals.var_x_wi_1d_dn7, locals.var_x_wi_1d_dn8, locals.var_x_wi_1d_dn9, ) = (assign11110_e10468, ((locals.var_xg2eff_dn4 + (((locals.var_dx_wi_1d_dn4 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn4)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn4), ((locals.var_xg2eff_dn6 + (((locals.var_dx_wi_1d_dn6 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn6)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn6), ((locals.var_xg2eff_dn7 + (((locals.var_dx_wi_1d_dn7 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn7)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn7), ((locals.var_xg2eff_dn8 + (((locals.var_dx_wi_1d_dn8 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn8)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn8), ((locals.var_xg2eff_dn9 + (((locals.var_dx_wi_1d_dn9 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn9)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn9), );
        }

        let assign11120_e10474: f64 = (locals.var_x_wi_1d + locals.var_xth_1d);
        let assign11120_e10477: f64 = (locals.var_x_wi_1d - locals.var_xth_1d);
        let assign11120_e10480: f64 = (locals.var_x_wi_1d - locals.var_xth_1d);
        let assign11120_e10481: f64 = (assign11120_e10477 * assign11120_e10480);
        let assign11120_e10483: f64 = (assign11120_e10481 + 4.0);
        let assign11120_e10484: f64 = (assign11120_e10483).sqrt();
        let assign11120_e10485: f64 = (assign11120_e10474 - assign11120_e10484);
        let assign11120_e10486: f64 = (0.5 * assign11120_e10485);
        (locals.var_x_1d, locals.var_x_1d_dn4, locals.var_x_1d_dn6, locals.var_x_1d_dn7, locals.var_x_1d_dn8, locals.var_x_1d_dn9, ) = (assign11120_e10486, (0.5 * ((locals.var_x_wi_1d_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_x_wi_1d_dn4 - locals.var_xth_1d_dn4) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign11120_e10484)))), (0.5 * ((locals.var_x_wi_1d_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_x_wi_1d_dn6 - locals.var_xth_1d_dn6) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign11120_e10484)))), (0.5 * ((locals.var_x_wi_1d_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_x_wi_1d_dn7 - locals.var_xth_1d_dn7) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign11120_e10484)))), (0.5 * ((locals.var_x_wi_1d_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_x_wi_1d_dn8 - locals.var_xth_1d_dn8) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign11120_e10484)))), (0.5 * ((locals.var_x_wi_1d_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_x_wi_1d_dn9 - locals.var_xth_1d_dn9) * assign11120_e10480) + (assign11120_e10477 * (locals.var_x_wi_1d_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign11120_e10484)))), );

        let assign11130_e10491: f64 = (locals.var_xth_1d - locals.var_x_1d);
        let assign11130_e10492: f64 = (2.0 * assign11130_e10491);
        let assign11130_e10494: f64 = (assign11130_e10492 / locals.var_xsddep);
        let assign11130_e10495: f64 = (1.0 + assign11130_e10494);
        let assign11130_e10496: f64 = (assign11130_e10495).sqrt();
        let assign11130_e10498: f64 = (assign11130_e10496 - 1.0);
        (locals.var_dleff, locals.var_dleff_dn4, locals.var_dleff_dn6, locals.var_dleff_dn7, locals.var_dleff_dn8, locals.var_dleff_dn9, ) = (assign11130_e10498, (((((2.0 * (locals.var_xth_1d_dn4 - locals.var_x_1d_dn4)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn4)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496)), (((((2.0 * (locals.var_xth_1d_dn6 - locals.var_x_1d_dn6)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn6)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496)), (((((2.0 * (locals.var_xth_1d_dn7 - locals.var_x_1d_dn7)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn7)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496)), (((((2.0 * (locals.var_xth_1d_dn8 - locals.var_x_1d_dn8)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn8)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496)), (((((2.0 * (locals.var_xth_1d_dn9 - locals.var_x_1d_dn9)) * locals.var_xsddep) - (assign11130_e10492 * locals.var_xsddep_dn9)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11130_e10496)), );

        let assign11140_e10502: f64 = (locals.var_xsddep * locals.var_dleff);
        let assign11140_e10503: f64 = (locals.var_x_1d + assign11140_e10502);
        (locals.var_xedge, locals.var_xedge_dn4, locals.var_xedge_dn6, locals.var_xedge_dn7, locals.var_xedge_dn8, locals.var_xedge_dn9, ) = (assign11140_e10503, (locals.var_x_1d_dn4 + ((locals.var_xsddep_dn4 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn4))), (locals.var_x_1d_dn6 + ((locals.var_xsddep_dn6 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn6))), (locals.var_x_1d_dn7 + ((locals.var_xsddep_dn7 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn7))), (locals.var_x_1d_dn8 + ((locals.var_xsddep_dn8 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn8))), (locals.var_x_1d_dn9 + ((locals.var_xsddep_dn9 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn9))), );

    }

    pub(super) fn stamp_transient_block_12(
        locals: &mut StampLocals,
    ) {
        let assign11150_e10508: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11150_e10509: f64 = (1.0 + assign11150_e10508);
        let assign11150_e10511: f64 = (assign11150_e10509 + 0.5);
        let assign11150_e10515: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11150_e10516: f64 = (1.0 + assign11150_e10515);
        let assign11150_e10518: f64 = (assign11150_e10516 - 0.5);
        let assign11150_e10522: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11150_e10523: f64 = (1.0 + assign11150_e10522);
        let assign11150_e10525: f64 = (assign11150_e10523 - 0.5);
        let assign11150_e10526: f64 = (assign11150_e10518 * assign11150_e10525);
        let assign11150_e10528: f64 = (assign11150_e10526 + 0.01);
        let assign11150_e10529: f64 = (assign11150_e10528).sqrt();
        let assign11150_e10530: f64 = (assign11150_e10511 + assign11150_e10529);
        let assign11150_e10531: f64 = (0.5 * assign11150_e10530);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign11150_e10531, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn4) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn4))) / (2.0 * assign11150_e10529)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn6) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn6))) / (2.0 * assign11150_e10529)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn7) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn7))) / (2.0 * assign11150_e10529)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn8) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn8))) / (2.0 * assign11150_e10529)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn9) * assign11150_e10525) + (assign11150_e10518 * (locals.var_pscedlb_i * locals.var_xg20shift_dn9))) / (2.0 * assign11150_e10529)))), );

        let assign11160_e10536: f64 = (locals.var_psce1_loc * locals.var_temp);
        let assign11160_e10537: f64 = (1.0 + assign11160_e10536);
        let assign11160_e10538: f64 = (1.0 / assign11160_e10537);
        (locals.var_sce1, locals.var_sce1_dn4, locals.var_sce1_dn6, locals.var_sce1_dn7, locals.var_sce1_dn8, locals.var_sce1_dn9, ) = (assign11160_e10538, (-((locals.var_psce1_loc * locals.var_temp_dn4) / (assign11160_e10537 * assign11160_e10537))), (-((locals.var_psce1_loc * locals.var_temp_dn6) / (assign11160_e10537 * assign11160_e10537))), (-((locals.var_psce1_loc * locals.var_temp_dn7) / (assign11160_e10537 * assign11160_e10537))), (-((locals.var_psce1_loc * locals.var_temp_dn8) / (assign11160_e10537 * assign11160_e10537))), (-((locals.var_psce1_loc * locals.var_temp_dn9) / (assign11160_e10537 * assign11160_e10537))), );

        let assign11170_e10543: f64 = (locals.var_psce2_loc * locals.var_temp);
        let assign11170_e10544: f64 = (1.0 + assign11170_e10543);
        let assign11170_e10545: f64 = (1.0 / assign11170_e10544);
        (locals.var_sce2, locals.var_sce2_dn4, locals.var_sce2_dn6, locals.var_sce2_dn7, locals.var_sce2_dn8, locals.var_sce2_dn9, ) = (assign11170_e10545, (-((locals.var_psce2_loc * locals.var_temp_dn4) / (assign11170_e10544 * assign11170_e10544))), (-((locals.var_psce2_loc * locals.var_temp_dn6) / (assign11170_e10544 * assign11170_e10544))), (-((locals.var_psce2_loc * locals.var_temp_dn7) / (assign11170_e10544 * assign11170_e10544))), (-((locals.var_psce2_loc * locals.var_temp_dn8) / (assign11170_e10544 * assign11170_e10544))), (-((locals.var_psce2_loc * locals.var_temp_dn9) / (assign11170_e10544 * assign11170_e10544))), );

        let assign11180_e10548: f64 = (2.0 * locals.var_xd0);
        let assign11180_e10552: f64 = (locals.var_xdsx / locals.var_xd0);
        let assign11180_e10553: f64 = (1.0 + assign11180_e10552);
        let assign11180_e10554: f64 = (assign11180_e10553).sqrt();
        let assign11180_e10556: f64 = (assign11180_e10554 - 1.0);
        let assign11180_e10557: f64 = (assign11180_e10548 * assign11180_e10556);
        let assign11180_e10561: f64 = (locals.var_cfdl_i * locals.var_dleff);
        let assign11180_e10562: f64 = (1.0 + assign11180_e10561);
        let assign11180_e10563: f64 = (assign11180_e10557 * assign11180_e10562);
        let assign11180_e10567: f64 = (locals.var_cfdlb_i * locals.var_xg20shift);
        let assign11180_e10568: f64 = (1.0 + assign11180_e10567);
        let assign11180_e10569: f64 = (assign11180_e10563 * assign11180_e10568);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign11180_e10569, (((((((2.0 * locals.var_xd0_dn4) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn4 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn4)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn4))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn4))), (((((((2.0 * locals.var_xd0_dn6) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn6 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn6)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn6))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn6))), (((((((2.0 * locals.var_xd0_dn7) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn7 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn7)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn7))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn7))), (((((((2.0 * locals.var_xd0_dn8) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn8 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn8)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn8))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn8))), (((((((2.0 * locals.var_xd0_dn9) * assign11180_e10556) + (assign11180_e10548 * ((((locals.var_xdsx_dn9 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn9)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11180_e10554)))) * assign11180_e10562) + (assign11180_e10557 * (locals.var_cfdl_i * locals.var_dleff_dn9))) * assign11180_e10568) + (assign11180_e10563 * (locals.var_cfdlb_i * locals.var_xg20shift_dn9))), );

        let assign11190_e10572: f64 = (locals.var_cf1_loc * locals.var_temp);
        (locals.var_dxg1_dibl, locals.var_dxg1_dibl_dn4, locals.var_dxg1_dibl_dn6, locals.var_dxg1_dibl_dn7, locals.var_dxg1_dibl_dn8, locals.var_dxg1_dibl_dn9, ) = (assign11190_e10572, ((locals.var_cf1_loc_dn4 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn4)), ((locals.var_cf1_loc_dn6 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn6)), ((locals.var_cf1_loc_dn7 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn7)), ((locals.var_cf1_loc_dn8 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn8)), ((locals.var_cf1_loc_dn9 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn9)), );

        let assign11200_e10575: f64 = (locals.var_cf2_loc * locals.var_temp);
        (locals.var_dxg2_dibl, locals.var_dxg2_dibl_dn4, locals.var_dxg2_dibl_dn6, locals.var_dxg2_dibl_dn7, locals.var_dxg2_dibl_dn8, locals.var_dxg2_dibl_dn9, ) = (assign11200_e10575, ((locals.var_cf2_loc_dn4 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn4)), ((locals.var_cf2_loc_dn6 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn6)), ((locals.var_cf2_loc_dn7 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn7)), ((locals.var_cf2_loc_dn8 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn8)), ((locals.var_cf2_loc_dn9 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn9)), );

        let assign11210_e10578: f64 = (locals.var_xg10 - locals.var_xedge);
        let assign11210_e10580: f64 = (assign11210_e10578 + locals.var_dxg1_dibl);
        let assign11210_e10582: f64 = (assign11210_e10580 * locals.var_sce1);
        let assign11210_e10584: f64 = (assign11210_e10582 + locals.var_xedge);
        let assign11210_e10586: f64 = (assign11210_e10584 + locals.var_dxdsx);
        (locals.var_xg1, locals.var_xg1_dn4, locals.var_xg1_dn6, locals.var_xg1_dn7, locals.var_xg1_dn8, locals.var_xg1_dn9, ) = (assign11210_e10586, ((((((locals.var_xg10_dn4 - locals.var_xedge_dn4) + locals.var_dxg1_dibl_dn4) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn4)) + locals.var_xedge_dn4) + locals.var_dxdsx_dn4), ((((((locals.var_xg10_dn6 - locals.var_xedge_dn6) + locals.var_dxg1_dibl_dn6) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn6)) + locals.var_xedge_dn6) + locals.var_dxdsx_dn6), ((((((locals.var_xg10_dn7 - locals.var_xedge_dn7) + locals.var_dxg1_dibl_dn7) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn7)) + locals.var_xedge_dn7) + locals.var_dxdsx_dn7), ((((((locals.var_xg10_dn8 - locals.var_xedge_dn8) + locals.var_dxg1_dibl_dn8) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn8)) + locals.var_xedge_dn8) + locals.var_dxdsx_dn8), ((((((locals.var_xg10_dn9 - locals.var_xedge_dn9) + locals.var_dxg1_dibl_dn9) * locals.var_sce1) + (assign11210_e10580 * locals.var_sce1_dn9)) + locals.var_xedge_dn9) + locals.var_dxdsx_dn9), );

        let assign11220_e10589: f64 = (locals.var_xg2eff - locals.var_xedge);
        let assign11220_e10591: f64 = (assign11220_e10589 + locals.var_dxg2_dibl);
        let assign11220_e10593: f64 = (assign11220_e10591 * locals.var_sce2);
        let assign11220_e10595: f64 = (assign11220_e10593 + locals.var_xedge);
        let assign11220_e10597: f64 = (assign11220_e10595 + locals.var_dxdsx);
        (locals.var_xg2, locals.var_xg2_dn4, locals.var_xg2_dn6, locals.var_xg2_dn7, locals.var_xg2_dn8, locals.var_xg2_dn9, ) = (assign11220_e10597, ((((((locals.var_xg2eff_dn4 - locals.var_xedge_dn4) + locals.var_dxg2_dibl_dn4) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn4)) + locals.var_xedge_dn4) + locals.var_dxdsx_dn4), ((((((locals.var_xg2eff_dn6 - locals.var_xedge_dn6) + locals.var_dxg2_dibl_dn6) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn6)) + locals.var_xedge_dn6) + locals.var_dxdsx_dn6), ((((((locals.var_xg2eff_dn7 - locals.var_xedge_dn7) + locals.var_dxg2_dibl_dn7) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn7)) + locals.var_xedge_dn7) + locals.var_dxdsx_dn7), ((((((locals.var_xg2eff_dn8 - locals.var_xedge_dn8) + locals.var_dxg2_dibl_dn8) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn8)) + locals.var_xedge_dn8) + locals.var_dxdsx_dn8), ((((((locals.var_xg2eff_dn9 - locals.var_xedge_dn9) + locals.var_dxg2_dibl_dn9) * locals.var_sce2) + (assign11220_e10591 * locals.var_sce2_dn9)) + locals.var_xedge_dn9) + locals.var_dxdsx_dn9), );

        let assign11230_e10603: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11230_e10604: f64 = (locals.var_cic1_i * assign11230_e10603);
        let assign11230_e10605: f64 = (locals.var_xg2 + assign11230_e10604);
        let assign11230_e10607: f64 = (assign11230_e10605 + locals.var_xsatmax);
        let assign11230_e10612: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11230_e10613: f64 = (locals.var_cic1_i * assign11230_e10612);
        let assign11230_e10614: f64 = (locals.var_xg2 + assign11230_e10613);
        let assign11230_e10616: f64 = (assign11230_e10614 - locals.var_xsatmax);
        let assign11230_e10621: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11230_e10622: f64 = (locals.var_cic1_i * assign11230_e10621);
        let assign11230_e10623: f64 = (locals.var_xg2 + assign11230_e10622);
        let assign11230_e10625: f64 = (assign11230_e10623 - locals.var_xsatmax);
        let assign11230_e10626: f64 = (assign11230_e10616 * assign11230_e10625);
        let assign11230_e10628: f64 = (assign11230_e10626 + 0.01);
        let assign11230_e10629: f64 = (assign11230_e10628).sqrt();
        let assign11230_e10630: f64 = (assign11230_e10607 - assign11230_e10629);
        let assign11230_e10631: f64 = (0.5 * assign11230_e10630);
        (locals.var_xg1x, locals.var_xg1x_dn4, locals.var_xg1x_dn6, locals.var_xg1x_dn7, locals.var_xg1x_dn8, locals.var_xg1x_dn9, ) = (assign11230_e10631, (0.5 * (((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) - locals.var_xsatmax_dn4) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign11230_e10629)))), (0.5 * (((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) - locals.var_xsatmax_dn6) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign11230_e10629)))), (0.5 * (((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) - locals.var_xsatmax_dn7) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign11230_e10629)))), (0.5 * (((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) - locals.var_xsatmax_dn8) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign11230_e10629)))), (0.5 * (((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) - locals.var_xsatmax_dn9) * assign11230_e10625) + (assign11230_e10616 * ((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign11230_e10629)))), );

        let assign11240_e10637: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11240_e10638: f64 = (locals.var_cic2_i * assign11240_e10637);
        let assign11240_e10639: f64 = (locals.var_xg1 + assign11240_e10638);
        let assign11240_e10641: f64 = (assign11240_e10639 + locals.var_xsatmax);
        let assign11240_e10646: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11240_e10647: f64 = (locals.var_cic2_i * assign11240_e10646);
        let assign11240_e10648: f64 = (locals.var_xg1 + assign11240_e10647);
        let assign11240_e10650: f64 = (assign11240_e10648 - locals.var_xsatmax);
        let assign11240_e10655: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11240_e10656: f64 = (locals.var_cic2_i * assign11240_e10655);
        let assign11240_e10657: f64 = (locals.var_xg1 + assign11240_e10656);
        let assign11240_e10659: f64 = (assign11240_e10657 - locals.var_xsatmax);
        let assign11240_e10660: f64 = (assign11240_e10650 * assign11240_e10659);
        let assign11240_e10662: f64 = (assign11240_e10660 + 0.01);
        let assign11240_e10663: f64 = (assign11240_e10662).sqrt();
        let assign11240_e10664: f64 = (assign11240_e10641 - assign11240_e10663);
        let assign11240_e10665: f64 = (0.5 * assign11240_e10664);
        (locals.var_xg2x, locals.var_xg2x_dn4, locals.var_xg2x_dn6, locals.var_xg2x_dn7, locals.var_xg2x_dn8, locals.var_xg2x_dn9, ) = (assign11240_e10665, (0.5 * (((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) - locals.var_xsatmax_dn4) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign11240_e10663)))), (0.5 * (((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) - locals.var_xsatmax_dn6) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign11240_e10663)))), (0.5 * (((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) - locals.var_xsatmax_dn7) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign11240_e10663)))), (0.5 * (((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) - locals.var_xsatmax_dn8) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign11240_e10663)))), (0.5 * (((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) - locals.var_xsatmax_dn9) * assign11240_e10659) + (assign11240_e10650 * ((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign11240_e10663)))), );

        let assign11250_e10668: f64 = (locals.var_k1_1d_qm / locals.var_sce1);
        (locals.var_k1, locals.var_k1_dn4, locals.var_k1_dn6, locals.var_k1_dn7, locals.var_k1_dn8, locals.var_k1_dn9, ) = (assign11250_e10668, (((locals.var_k1_1d_qm_dn4 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn4)) / (locals.var_sce1 * locals.var_sce1)), (((locals.var_k1_1d_qm_dn6 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn6)) / (locals.var_sce1 * locals.var_sce1)), (((locals.var_k1_1d_qm_dn7 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn7)) / (locals.var_sce1 * locals.var_sce1)), (((locals.var_k1_1d_qm_dn8 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn8)) / (locals.var_sce1 * locals.var_sce1)), (((locals.var_k1_1d_qm_dn9 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn9)) / (locals.var_sce1 * locals.var_sce1)), );

        let assign11260_e10671: f64 = (locals.var_k2_1d_qm / locals.var_sce2);
        (locals.var_k2, locals.var_k2_dn4, locals.var_k2_dn6, locals.var_k2_dn7, locals.var_k2_dn8, locals.var_k2_dn9, ) = (assign11260_e10671, (((locals.var_k2_1d_qm_dn4 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn4)) / (locals.var_sce2 * locals.var_sce2)), (((locals.var_k2_1d_qm_dn6 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn6)) / (locals.var_sce2 * locals.var_sce2)), (((locals.var_k2_1d_qm_dn7 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn7)) / (locals.var_sce2 * locals.var_sce2)), (((locals.var_k2_1d_qm_dn8 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn8)) / (locals.var_sce2 * locals.var_sce2)), (((locals.var_k2_1d_qm_dn9 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn9)) / (locals.var_sce2 * locals.var_sce2)), );

        let assign11270_e10674: f64 = (1.0 / locals.var_k1);
        (locals.var_inv_k1, locals.var_inv_k1_dn4, locals.var_inv_k1_dn6, locals.var_inv_k1_dn7, locals.var_inv_k1_dn8, locals.var_inv_k1_dn9, ) = (assign11270_e10674, (-(locals.var_k1_dn4 / (locals.var_k1 * locals.var_k1))), (-(locals.var_k1_dn6 / (locals.var_k1 * locals.var_k1))), (-(locals.var_k1_dn7 / (locals.var_k1 * locals.var_k1))), (-(locals.var_k1_dn8 / (locals.var_k1 * locals.var_k1))), (-(locals.var_k1_dn9 / (locals.var_k1 * locals.var_k1))), );

        let assign11280_e10677: f64 = (1.0 / locals.var_k2);
        (locals.var_inv_k2, locals.var_inv_k2_dn4, locals.var_inv_k2_dn6, locals.var_inv_k2_dn7, locals.var_inv_k2_dn8, locals.var_inv_k2_dn9, ) = (assign11280_e10677, (-(locals.var_k2_dn4 / (locals.var_k2 * locals.var_k2))), (-(locals.var_k2_dn6 / (locals.var_k2 * locals.var_k2))), (-(locals.var_k2_dn7 / (locals.var_k2 * locals.var_k2))), (-(locals.var_k2_dn8 / (locals.var_k2 * locals.var_k2))), (-(locals.var_k2_dn9 / (locals.var_k2 * locals.var_k2))), );

        let assign11290_e10681: f64 = (1.0 + locals.var_inv_k1);
        let assign11290_e10683: f64 = (assign11290_e10681 + locals.var_inv_k2);
        let assign11290_e10684: f64 = (1.0 / assign11290_e10683);
        (locals.var_keq, locals.var_keq_dn4, locals.var_keq_dn6, locals.var_keq_dn7, locals.var_keq_dn8, locals.var_keq_dn9, ) = (assign11290_e10684, (-((locals.var_inv_k1_dn4 + locals.var_inv_k2_dn4) / (assign11290_e10683 * assign11290_e10683))), (-((locals.var_inv_k1_dn6 + locals.var_inv_k2_dn6) / (assign11290_e10683 * assign11290_e10683))), (-((locals.var_inv_k1_dn7 + locals.var_inv_k2_dn7) / (assign11290_e10683 * assign11290_e10683))), (-((locals.var_inv_k1_dn8 + locals.var_inv_k2_dn8) / (assign11290_e10683 * assign11290_e10683))), (-((locals.var_inv_k1_dn9 + locals.var_inv_k2_dn9) / (assign11290_e10683 * assign11290_e10683))), );

        let assign11300_e10688: f64 = (locals.var_csiprime * locals.var_csiprime);
        let assign11300_e10689: f64 = (locals.var_a0_csisq / assign11300_e10688);
        (locals.var_a0, locals.var_a0_dn4, locals.var_a0_dn6, locals.var_a0_dn7, locals.var_a0_dn8, locals.var_a0_dn9, ) = (assign11300_e10689, (((locals.var_a0_csisq_dn4 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn4 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn4)))) / (assign11300_e10688 * assign11300_e10688)), (((locals.var_a0_csisq_dn6 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn6 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn6)))) / (assign11300_e10688 * assign11300_e10688)), (((locals.var_a0_csisq_dn7 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn7 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn7)))) / (assign11300_e10688 * assign11300_e10688)), (((locals.var_a0_csisq_dn8 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn8 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn8)))) / (assign11300_e10688 * assign11300_e10688)), (((locals.var_a0_csisq_dn9 * assign11300_e10688) - (locals.var_a0_csisq * ((locals.var_csiprime_dn9 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn9)))) / (assign11300_e10688 * assign11300_e10688)), );

        let assign11310_e10692: f64 = (1.0 + locals.var_k1);
        let assign11310_e10695: f64 = (1.0 + locals.var_k2);
        let assign11310_e10696: f64 = (assign11310_e10692 / assign11310_e10695);
        (locals.var_exp_dxth, locals.var_exp_dxth_dn4, locals.var_exp_dxth_dn6, locals.var_exp_dxth_dn7, locals.var_exp_dxth_dn8, locals.var_exp_dxth_dn9, ) = (assign11310_e10696, (((locals.var_k1_dn4 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn4)) / (assign11310_e10695 * assign11310_e10695)), (((locals.var_k1_dn6 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn6)) / (assign11310_e10695 * assign11310_e10695)), (((locals.var_k1_dn7 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn7)) / (assign11310_e10695 * assign11310_e10695)), (((locals.var_k1_dn8 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn8)) / (assign11310_e10695 * assign11310_e10695)), (((locals.var_k1_dn9 * assign11310_e10695) - (assign11310_e10692 * locals.var_k2_dn9)) / (assign11310_e10695 * assign11310_e10695)), );

        let assign11320_e10698: f64 = (locals.var_exp_dxth).ln();
        (locals.var_dxth, locals.var_dxth_dn4, locals.var_dxth_dn6, locals.var_dxth_dn7, locals.var_dxth_dn8, locals.var_dxth_dn9, ) = (assign11320_e10698, (locals.var_exp_dxth_dn4 / locals.var_exp_dxth), (locals.var_exp_dxth_dn6 / locals.var_exp_dxth), (locals.var_exp_dxth_dn7 / locals.var_exp_dxth), (locals.var_exp_dxth_dn8 / locals.var_exp_dxth), (locals.var_exp_dxth_dn9 / locals.var_exp_dxth), );

        let assign11330_e10701: f64 = if locals.var_dxth > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign11330_e10701;

        if (locals.var_guard547 != 0.0) {
            let assign11340_e10705: f64 = (2.0 * locals.var_dxth);
            let assign11340_e10708: f64 = (locals.var_exp_dxth + 1.0);
            let assign11340_e10709: f64 = (assign11340_e10705 * assign11340_e10708);
            let assign11340_e10712: f64 = (locals.var_exp_dxth - 1.0);
            let assign11340_e10713: f64 = (assign11340_e10709 / assign11340_e10712);
            (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9, ) = (assign11340_e10713, ((((((2.0 * locals.var_dxth_dn4) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn4)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn4)) / (assign11340_e10712 * assign11340_e10712)), ((((((2.0 * locals.var_dxth_dn6) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn6)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn6)) / (assign11340_e10712 * assign11340_e10712)), ((((((2.0 * locals.var_dxth_dn7) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn7)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn7)) / (assign11340_e10712 * assign11340_e10712)), ((((((2.0 * locals.var_dxth_dn8) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn8)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn8)) / (assign11340_e10712 * assign11340_e10712)), ((((((2.0 * locals.var_dxth_dn9) * assign11340_e10708) + (assign11340_e10705 * locals.var_exp_dxth_dn9)) * assign11340_e10712) - (assign11340_e10709 * locals.var_exp_dxth_dn9)) / (assign11340_e10712 * assign11340_e10712)), );
        }

        if (locals.var_guard547 == 0.0) {
            let assign11350_e10721: f64 = (2.0 + locals.var_dxth);
            let assign11350_e10722: f64 = (2.0 * assign11350_e10721);
            (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9, ) = (assign11350_e10722, (2.0 * locals.var_dxth_dn4), (2.0 * locals.var_dxth_dn6), (2.0 * locals.var_dxth_dn7), (2.0 * locals.var_dxth_dn8), (2.0 * locals.var_dxth_dn9), );
        }

        let assign11360_e10728: f64 = (locals.var_xg1x - locals.var_xg2x);
        let assign11360_e10729: f64 = (locals.var_keq * assign11360_e10728);
        (locals.var_dx_wi, locals.var_dx_wi_dn4, locals.var_dx_wi_dn6, locals.var_dx_wi_dn7, locals.var_dx_wi_dn8, locals.var_dx_wi_dn9, ) = (assign11360_e10729, ((locals.var_keq_dn4 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn4 - locals.var_xg2x_dn4))), ((locals.var_keq_dn6 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn6 - locals.var_xg2x_dn6))), ((locals.var_keq_dn7 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn7 - locals.var_xg2x_dn7))), ((locals.var_keq_dn8 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn8 - locals.var_xg2x_dn8))), ((locals.var_keq_dn9 * assign11360_e10728) + (locals.var_keq * (locals.var_xg1x_dn9 - locals.var_xg2x_dn9))), );

        let assign11370_e10732: f64 = (locals.var_dx_wi * locals.var_dx_wi);
        (locals.var_dx_wisq, locals.var_dx_wisq_dn4, locals.var_dx_wisq_dn6, locals.var_dx_wisq_dn7, locals.var_dx_wisq_dn8, locals.var_dx_wisq_dn9, ) = (assign11370_e10732, ((locals.var_dx_wi_dn4 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn4)), ((locals.var_dx_wi_dn6 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn6)), ((locals.var_dx_wi_dn7 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn7)), ((locals.var_dx_wi_dn8 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn8)), ((locals.var_dx_wi_dn9 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn9)), );

        let assign11380_e10736: f64 = (locals.var_dx_wi * locals.var_inv_k1);
        let assign11380_e10737: f64 = (locals.var_xg1x - assign11380_e10736);
        (locals.var_x1_wi0, locals.var_x1_wi0_dn4, locals.var_x1_wi0_dn6, locals.var_x1_wi0_dn7, locals.var_x1_wi0_dn8, locals.var_x1_wi0_dn9, ) = (assign11380_e10737, (locals.var_xg1x_dn4 - ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4))), (locals.var_xg1x_dn6 - ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6))), (locals.var_xg1x_dn7 - ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7))), (locals.var_xg1x_dn8 - ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8))), (locals.var_xg1x_dn9 - ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9))), );

        let assign11390_e10741: f64 = (locals.var_dx_wi * locals.var_inv_k2);
        let assign11390_e10742: f64 = (locals.var_xg2x + assign11390_e10741);
        (locals.var_x2_wi0, locals.var_x2_wi0_dn4, locals.var_x2_wi0_dn6, locals.var_x2_wi0_dn7, locals.var_x2_wi0_dn8, locals.var_x2_wi0_dn9, ) = (assign11390_e10742, (locals.var_xg2x_dn4 + ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4))), (locals.var_xg2x_dn6 + ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6))), (locals.var_xg2x_dn7 + ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7))), (locals.var_xg2x_dn8 + ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8))), (locals.var_xg2x_dn9 + ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9))), );

        let assign11400_e10746: f64 = (locals.var_k1 + 1.0);
        let assign11400_e10747: f64 = (1.0 / assign11400_e10746);
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11400_e10747, (-(locals.var_k1_dn4 / (assign11400_e10746 * assign11400_e10746))), (-(locals.var_k1_dn6 / (assign11400_e10746 * assign11400_e10746))), (-(locals.var_k1_dn7 / (assign11400_e10746 * assign11400_e10746))), (-(locals.var_k1_dn8 / (assign11400_e10746 * assign11400_e10746))), (-(locals.var_k1_dn9 / (assign11400_e10746 * assign11400_e10746))), );

        let assign11410_e10751: f64 = (locals.var_k2 + 1.0);
        let assign11410_e10752: f64 = (1.0 / assign11410_e10751);
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11410_e10752, (-(locals.var_k2_dn4 / (assign11410_e10751 * assign11410_e10751))), (-(locals.var_k2_dn6 / (assign11410_e10751 * assign11410_e10751))), (-(locals.var_k2_dn7 / (assign11410_e10751 * assign11410_e10751))), (-(locals.var_k2_dn8 / (assign11410_e10751 * assign11410_e10751))), (-(locals.var_k2_dn9 / (assign11410_e10751 * assign11410_e10751))), );

        let assign11420_e10756: f64 = (locals.var_k2 * locals.var_q_temp2);
        let assign11420_e10757: f64 = (locals.var_k1 + assign11420_e10756);
        let assign11420_e10759: f64 = (assign11420_e10757 * locals.var_diff_min);
        let assign11420_e10761: f64 = (assign11420_e10759 / locals.var_a0);
        let assign11420_e10762: f64 = (assign11420_e10761).ln();
        let assign11420_e10764: f64 = assign11420_e10762;
        let assign11420_e10766: f64 = (assign11420_e10764 + 3.0);
        (locals.var_q_x1sat, locals.var_q_x1sat_dn4, locals.var_q_x1sat_dn6, locals.var_q_x1sat_dn7, locals.var_q_x1sat_dn8, locals.var_q_x1sat_dn9, ) = (assign11420_e10766, (((((((locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn4))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761), (((((((locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn6))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761), (((((((locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn7))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761), (((((((locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn8))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761), (((((((locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn9))) * locals.var_diff_min) + (assign11420_e10757 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign11420_e10759 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign11420_e10761), );

        let assign11430_e10770: f64 = (locals.var_k1 * locals.var_q_temp1);
        let assign11430_e10771: f64 = (locals.var_k2 + assign11430_e10770);
        let assign11430_e10773: f64 = (assign11430_e10771 * locals.var_diff_min);
        let assign11430_e10775: f64 = (assign11430_e10773 / locals.var_a0);
        let assign11430_e10776: f64 = (assign11430_e10775).ln();
        let assign11430_e10778: f64 = assign11430_e10776;
        let assign11430_e10780: f64 = (assign11430_e10778 + 3.0);
        (locals.var_q_x2sat, locals.var_q_x2sat_dn4, locals.var_q_x2sat_dn6, locals.var_q_x2sat_dn7, locals.var_q_x2sat_dn8, locals.var_q_x2sat_dn9, ) = (assign11430_e10780, (((((((locals.var_k2_dn4 + ((locals.var_k1_dn4 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn4))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775), (((((((locals.var_k2_dn6 + ((locals.var_k1_dn6 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn6))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775), (((((((locals.var_k2_dn7 + ((locals.var_k1_dn7 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn7))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775), (((((((locals.var_k2_dn8 + ((locals.var_k1_dn8 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn8))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775), (((((((locals.var_k2_dn9 + ((locals.var_k1_dn9 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn9))) * locals.var_diff_min) + (assign11430_e10771 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign11430_e10773 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign11430_e10775), );

        let assign11440_e10783: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign11440_e10785: f64 = (assign11440_e10783 * 0.3333333333333);
        let assign11440_e10787: f64 = if assign11440_e10785 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign11440_e10787;

        if (locals.var_guard548 != 0.0) {
            let assign11450_e10792: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
            let assign11450_e10794: f64 = (assign11450_e10792 * 0.3333333333333);
            let assign11450_e10795: f64 = (assign11450_e10794).exp();
            let assign11450_e10796: f64 = (1.0 + assign11450_e10795);
            let assign11450_e10797: f64 = (assign11450_e10796).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11450_e10797, ((assign11450_e10795 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333)) / assign11450_e10796), ((assign11450_e10795 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333)) / assign11450_e10796), ((assign11450_e10795 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333)) / assign11450_e10796), ((assign11450_e10795 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333)) / assign11450_e10796), ((assign11450_e10795 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333)) / assign11450_e10796), );
        }

        if (locals.var_guard548 == 0.0) {
            let assign11460_e10804: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
            let assign11460_e10806: f64 = (assign11460_e10804 * 0.3333333333333);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11460_e10806, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333), );
        }

        let assign11470_e10812: f64 = (3.0 * locals.var_q_temp3);
        let assign11470_e10813: f64 = (locals.var_q_x1sat - assign11470_e10812);
        (locals.var_q_x1, locals.var_q_x1_dn4, locals.var_q_x1_dn6, locals.var_q_x1_dn7, locals.var_q_x1_dn8, locals.var_q_x1_dn9, ) = (assign11470_e10813, (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4)), (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6)), (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7)), (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8)), (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9)), );

        let assign11480_e10816: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign11480_e10818: f64 = (assign11480_e10816 * 0.3333333333333);
        let assign11480_e10820: f64 = if assign11480_e10818 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign11480_e10820;

        if (locals.var_guard549 != 0.0) {
            let assign11490_e10825: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
            let assign11490_e10827: f64 = (assign11490_e10825 * 0.3333333333333);
            let assign11490_e10828: f64 = (assign11490_e10827).exp();
            let assign11490_e10829: f64 = (1.0 + assign11490_e10828);
            let assign11490_e10830: f64 = (assign11490_e10829).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11490_e10830, ((assign11490_e10828 * ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333)) / assign11490_e10829), ((assign11490_e10828 * ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333)) / assign11490_e10829), ((assign11490_e10828 * ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333)) / assign11490_e10829), ((assign11490_e10828 * ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333)) / assign11490_e10829), ((assign11490_e10828 * ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333)) / assign11490_e10829), );
        }

        if (locals.var_guard549 == 0.0) {
            let assign11500_e10837: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
            let assign11500_e10839: f64 = (assign11500_e10837 * 0.3333333333333);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11500_e10839, ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333), );
        }

        let assign11510_e10845: f64 = (3.0 * locals.var_q_temp3);
        let assign11510_e10846: f64 = (locals.var_q_x2sat - assign11510_e10845);
        (locals.var_q_x2, locals.var_q_x2_dn4, locals.var_q_x2_dn6, locals.var_q_x2_dn7, locals.var_q_x2_dn8, locals.var_q_x2_dn9, ) = (assign11510_e10846, (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4)), (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6)), (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7)), (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8)), (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9)), );

        let assign11520_e10849: f64 = (locals.var_k1 * locals.var_xg1x);
        let assign11520_e10851: f64 = (assign11520_e10849 + locals.var_q_x2);
        let assign11520_e10853: f64 = (assign11520_e10851 * locals.var_q_temp1);
        (locals.var_q_x1_wi, locals.var_q_x1_wi_dn4, locals.var_q_x1_wi_dn6, locals.var_q_x1_wi_dn7, locals.var_q_x1_wi_dn8, locals.var_q_x1_wi_dn9, ) = (assign11520_e10853, (((((locals.var_k1_dn4 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn4)) + locals.var_q_x2_dn4) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn4)), (((((locals.var_k1_dn6 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn6)) + locals.var_q_x2_dn6) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn6)), (((((locals.var_k1_dn7 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn7)) + locals.var_q_x2_dn7) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn7)), (((((locals.var_k1_dn8 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn8)) + locals.var_q_x2_dn8) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn8)), (((((locals.var_k1_dn9 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn9)) + locals.var_q_x2_dn9) * locals.var_q_temp1) + (assign11520_e10851 * locals.var_q_temp1_dn9)), );

        let assign11530_e10856: f64 = (locals.var_k2 * locals.var_xg2x);
        let assign11530_e10858: f64 = (assign11530_e10856 + locals.var_q_x1);
        let assign11530_e10860: f64 = (assign11530_e10858 * locals.var_q_temp2);
        (locals.var_q_x2_wi, locals.var_q_x2_wi_dn4, locals.var_q_x2_wi_dn6, locals.var_q_x2_wi_dn7, locals.var_q_x2_wi_dn8, locals.var_q_x2_wi_dn9, ) = (assign11530_e10860, (((((locals.var_k2_dn4 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn4)) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn4)), (((((locals.var_k2_dn6 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn6)) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn6)), (((((locals.var_k2_dn7 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn7)) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn7)), (((((locals.var_k2_dn8 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn8)) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn8)), (((((locals.var_k2_dn9 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn9)) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign11530_e10858 * locals.var_q_temp2_dn9)), );

        let assign11540_e10863: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign11540_e10865: f64 = (assign11540_e10863 * 0.3333333333333);
        let assign11540_e10867: f64 = if assign11540_e10865 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign11540_e10867;

        if (locals.var_guard550 != 0.0) {
            let assign11550_e10872: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
            let assign11550_e10874: f64 = (assign11550_e10872 * 0.3333333333333);
            let assign11550_e10875: f64 = (assign11550_e10874).exp();
            let assign11550_e10876: f64 = (1.0 + assign11550_e10875);
            let assign11550_e10877: f64 = (assign11550_e10876).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11550_e10877, ((assign11550_e10875 * ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333)) / assign11550_e10876), ((assign11550_e10875 * ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333)) / assign11550_e10876), ((assign11550_e10875 * ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333)) / assign11550_e10876), ((assign11550_e10875 * ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333)) / assign11550_e10876), ((assign11550_e10875 * ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333)) / assign11550_e10876), );
        }

        if (locals.var_guard550 == 0.0) {
            let assign11560_e10884: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
            let assign11560_e10886: f64 = (assign11560_e10884 * 0.3333333333333);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11560_e10886, ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333), );
        }

        let assign11570_e10892: f64 = (3.0 * locals.var_q_temp3);
        let assign11570_e10893: f64 = (locals.var_q_x1sat - assign11570_e10892);
        (locals.var_q_x1, locals.var_q_x1_dn4, locals.var_q_x1_dn6, locals.var_q_x1_dn7, locals.var_q_x1_dn8, locals.var_q_x1_dn9, ) = (assign11570_e10893, (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4)), (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6)), (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7)), (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8)), (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9)), );

        let assign11580_e10896: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign11580_e10898: f64 = (assign11580_e10896 * 0.3333333333333);
        let assign11580_e10900: f64 = if assign11580_e10898 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign11580_e10900;

        if (locals.var_guard551 != 0.0) {
            let assign11590_e10905: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
            let assign11590_e10907: f64 = (assign11590_e10905 * 0.3333333333333);
            let assign11590_e10908: f64 = (assign11590_e10907).exp();
            let assign11590_e10909: f64 = (1.0 + assign11590_e10908);
            let assign11590_e10910: f64 = (assign11590_e10909).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11590_e10910, ((assign11590_e10908 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333)) / assign11590_e10909), ((assign11590_e10908 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333)) / assign11590_e10909), ((assign11590_e10908 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333)) / assign11590_e10909), ((assign11590_e10908 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333)) / assign11590_e10909), ((assign11590_e10908 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333)) / assign11590_e10909), );
        }

        if (locals.var_guard551 == 0.0) {
            let assign11600_e10917: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
            let assign11600_e10919: f64 = (assign11600_e10917 * 0.3333333333333);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11600_e10919, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333), );
        }

        let assign11610_e10925: f64 = (3.0 * locals.var_q_temp3);
        let assign11610_e10926: f64 = (locals.var_q_x2sat - assign11610_e10925);
        (locals.var_q_x2, locals.var_q_x2_dn4, locals.var_q_x2_dn6, locals.var_q_x2_dn7, locals.var_q_x2_dn8, locals.var_q_x2_dn9, ) = (assign11610_e10926, (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4)), (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6)), (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7)), (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8)), (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9)), );

        let assign11620_e10929: f64 = (locals.var_xg1x - locals.var_q_x1);
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign11620_e10929, (locals.var_xg1x_dn4 - locals.var_q_x1_dn4), (locals.var_xg1x_dn6 - locals.var_q_x1_dn6), (locals.var_xg1x_dn7 - locals.var_q_x1_dn7), (locals.var_xg1x_dn8 - locals.var_q_x1_dn8), (locals.var_xg1x_dn9 - locals.var_q_x1_dn9), );

        let assign11630_e10932: f64 = (locals.var_xg2x - locals.var_q_x2);
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9, ) = (assign11630_e10932, (locals.var_xg2x_dn4 - locals.var_q_x2_dn4), (locals.var_xg2x_dn6 - locals.var_q_x2_dn6), (locals.var_xg2x_dn7 - locals.var_q_x2_dn7), (locals.var_xg2x_dn8 - locals.var_q_x2_dn8), (locals.var_xg2x_dn9 - locals.var_q_x2_dn9), );

        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign11660_e10937: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign11660_e10937, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign11670_e10940: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11670_e10942: f64 = assign11670_e10940;
        let assign11670_e10944: f64 = if assign11670_e10942 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign11670_e10944;

        if (locals.var_guard552 != 0.0) {
            let assign11680_e10948: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign11680_e10950: f64 = assign11680_e10948;
            let assign11680_e10951: f64 = (assign11680_e10950).exp();
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11680_e10951, (assign11680_e10951 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign11680_e10951 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign11680_e10951 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign11680_e10951 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign11680_e10951 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)), );
        }

        if (locals.var_guard552 == 0.0) {
            let assign11690_e10960: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign11690_e10962: f64 = assign11690_e10960;
            let assign11690_e10964: f64 = (assign11690_e10962 - 80.0);
            let assign11690_e10969: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign11690_e10971: f64 = assign11690_e10969;
            let assign11690_e10973: f64 = (assign11690_e10971 - 80.0);
            let assign11690_e10974: f64 = (0.5 * assign11690_e10973);
            let assign11690_e10978: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign11690_e10980: f64 = assign11690_e10978;
            let assign11690_e10982: f64 = (assign11690_e10980 - 80.0);
            let assign11690_e10984: f64 = (assign11690_e10982 * 0.3333333333333);
            let assign11690_e10985: f64 = (1.0 + assign11690_e10984);
            let assign11690_e10986: f64 = (assign11690_e10974 * assign11690_e10985);
            let assign11690_e10987: f64 = (1.0 + assign11690_e10986);
            let assign11690_e10988: f64 = (assign11690_e10964 * assign11690_e10987);
            let assign11690_e10989: f64 = (1.0 + assign11690_e10988);
            let assign11690_e10990: f64 = (5.54062e34 * assign11690_e10989);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11690_e10990, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign11690_e10987) + (assign11690_e10964 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign11690_e10985) + (assign11690_e10974 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))), );
        }

        let assign11700_e10995: f64 = (locals.var_a0 * locals.var_q_temp1);
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9, ) = (assign11700_e10995, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)), );

        let assign11710_e10998: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign11710_e11000: f64 = (assign11710_e10998 - locals.var_q_aexp);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign11710_e11000, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9), );

        let assign11720_e11003: f64 = (2.0 * locals.var_k1);
        let assign11720_e11005: f64 = (assign11720_e11003 * locals.var_q_k1q1);
        let assign11720_e11007: f64 = (assign11720_e11005 + locals.var_q_aexp);
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9, ) = (assign11720_e11007, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign11720_e11003 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9), );

        let assign11730_e11010: f64 = (2.0 * locals.var_k1);
        let assign11730_e11012: f64 = (assign11730_e11010 * locals.var_k1);
        let assign11730_e11014: f64 = (assign11730_e11012 - locals.var_q_aexp);
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9, ) = (assign11730_e11014, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign11730_e11010 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9), );

        let assign11740_e11017: f64 = (-0.005);
        let assign11740_e11018: f64 = if locals.var_q_qsq < assign11740_e11017 { 1.0 } else { 0.0 };
        locals.var_guard553 = assign11740_e11018;

        if (locals.var_guard553 != 0.0) {
            let assign11750_e11021: f64 = (locals.var_q_qsq).abs();
            let assign11750_e11022: f64 = (assign11750_e11021).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign11750_e11022, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign11750_e11022)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign11750_e11022)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign11750_e11022)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign11750_e11022)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign11750_e11022)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11760_e11029: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign11760_e11030: f64 = (assign11760_e11029).tan();
            let assign11760_e11031: f64 = (locals.var_q_rac_qsq / assign11760_e11030);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign11760_e11031, (((locals.var_q_rac_qsq_dn4 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), (((locals.var_q_rac_qsq_dn6 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), (((locals.var_q_rac_qsq_dn7 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), (((locals.var_q_rac_qsq_dn8 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), (((locals.var_q_rac_qsq_dn9 * assign11760_e11030) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign11760_e11029).cos() * (assign11760_e11029).cos())))) / (assign11760_e11030 * assign11760_e11030)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11770_e11037: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign11770_e11039: f64 = (assign11770_e11037 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11770_e11039, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign11770_e11037 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11780_e11047: f64 = (2.0 - locals.var_q_qcoth);
            let assign11780_e11048: f64 = (locals.var_q_qcoth * assign11780_e11047);
            let assign11780_e11049: f64 = (locals.var_q_qsq + assign11780_e11048);
            let assign11780_e11051: f64 = (assign11780_e11049 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign11780_e11051, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign11780_e11047) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign11780_e11049 * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11790_e11058: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign11790_e11061: f64 = (1.0 + locals.var_q_qcoth);
            let assign11790_e11062: f64 = (assign11790_e11058 * assign11790_e11061);
            let assign11790_e11063: f64 = (locals.var_q_d1_qsq - assign11790_e11062);
            let assign11790_e11065: f64 = (assign11790_e11063 * locals.var_q_temp1);
            let assign11790_e11068: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign11790_e11070: f64 = (assign11790_e11068 / locals.var_q_d1_qsq);
            let assign11790_e11071: f64 = (assign11790_e11065 + assign11790_e11070);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign11790_e11071, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign11790_e11061) + (assign11790_e11058 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign11790_e11063 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign11790_e11068 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11800_e11078: f64 = (0.5 * locals.var_q_qcoth);
            let assign11800_e11079: f64 = (1.0 - assign11800_e11078);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11800_e11079, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11810_e11085: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign11810_e11087: f64 = (assign11810_e11085 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign11810_e11087, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11085 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11820_e11093: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign11820_e11098: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign11820_e11099: f64 = (locals.var_q_d1_ln + assign11820_e11098);
            let assign11820_e11100: f64 = (locals.var_q_d1_qsq * assign11820_e11099);
            let assign11820_e11101: f64 = (assign11820_e11093 - assign11820_e11100);
            let assign11820_e11103: f64 = (assign11820_e11101 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign11820_e11103, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign11820_e11099) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign11820_e11101 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        let assign11830_e11108: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard554 = assign11830_e11108;

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11840_e11114: f64 = (locals.var_q_qsq).abs();
            let assign11840_e11115: f64 = (assign11840_e11114).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign11840_e11115, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign11840_e11115)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign11840_e11115)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign11840_e11115)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign11840_e11115)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign11840_e11115)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11850_e11123: f64 = (-locals.var_q_rac_qsq);
            let assign11850_e11124: f64 = (assign11850_e11123).exp();
            (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (assign11850_e11124, (assign11850_e11124 * (-locals.var_q_rac_qsq_dn4)), (assign11850_e11124 * (-locals.var_q_rac_qsq_dn6)), (assign11850_e11124 * (-locals.var_q_rac_qsq_dn7)), (assign11850_e11124 * (-locals.var_q_rac_qsq_dn8)), (assign11850_e11124 * (-locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11860_e11134: f64 = (1.0 + locals.var_q_invexpq);
            let assign11860_e11135: f64 = (locals.var_q_rac_qsq * assign11860_e11134);
            let assign11860_e11138: f64 = (1.0 - locals.var_q_invexpq);
            let assign11860_e11139: f64 = (assign11860_e11135 / assign11860_e11138);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign11860_e11139, (((((locals.var_q_rac_qsq_dn4 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn4))) / (assign11860_e11138 * assign11860_e11138)), (((((locals.var_q_rac_qsq_dn6 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn6))) / (assign11860_e11138 * assign11860_e11138)), (((((locals.var_q_rac_qsq_dn7 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn7))) / (assign11860_e11138 * assign11860_e11138)), (((((locals.var_q_rac_qsq_dn8 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn8))) / (assign11860_e11138 * assign11860_e11138)), (((((locals.var_q_rac_qsq_dn9 * assign11860_e11134) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign11860_e11138) - (assign11860_e11135 * (-locals.var_q_invexpq_dn9))) / (assign11860_e11138 * assign11860_e11138)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11870_e11148: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign11870_e11150: f64 = (assign11870_e11148 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11870_e11150, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign11870_e11148 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11880_e11161: f64 = (2.0 - locals.var_q_qcoth);
            let assign11880_e11162: f64 = (locals.var_q_qcoth * assign11880_e11161);
            let assign11880_e11163: f64 = (locals.var_q_qsq + assign11880_e11162);
            let assign11880_e11165: f64 = (assign11880_e11163 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign11880_e11165, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign11880_e11161) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign11880_e11163 * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11890_e11175: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign11890_e11178: f64 = (1.0 + locals.var_q_qcoth);
            let assign11890_e11179: f64 = (assign11890_e11175 * assign11890_e11178);
            let assign11890_e11180: f64 = (locals.var_q_d1_qsq - assign11890_e11179);
            let assign11890_e11182: f64 = (assign11890_e11180 * locals.var_q_temp1);
            let assign11890_e11185: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign11890_e11187: f64 = (assign11890_e11185 / locals.var_q_d1_qsq);
            let assign11890_e11188: f64 = (assign11890_e11182 + assign11890_e11187);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign11890_e11188, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign11890_e11178) + (assign11890_e11175 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign11890_e11180 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign11890_e11185 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11900_e11198: f64 = (0.5 * locals.var_q_qcoth);
            let assign11900_e11199: f64 = (1.0 - assign11900_e11198);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11900_e11199, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11910_e11208: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign11910_e11210: f64 = (assign11910_e11208 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign11910_e11210, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11910_e11208 * locals.var_q_temp2_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11920_e11219: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign11920_e11224: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign11920_e11225: f64 = (locals.var_q_d1_ln + assign11920_e11224);
            let assign11920_e11226: f64 = (locals.var_q_d1_qsq * assign11920_e11225);
            let assign11920_e11227: f64 = (assign11920_e11219 - assign11920_e11226);
            let assign11920_e11229: f64 = (assign11920_e11227 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign11920_e11229, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign11920_e11225) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign11920_e11227 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11930_e11241: f64 = (locals.var_q_qsq * 0.0166666666667);
            let assign11930_e11245: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign11930_e11249: f64 = (locals.var_q_qsq * 0.025);
            let assign11930_e11250: f64 = (1.0 - assign11930_e11249);
            let assign11930_e11251: f64 = (assign11930_e11245 * assign11930_e11250);
            let assign11930_e11252: f64 = (1.0 - assign11930_e11251);
            let assign11930_e11253: f64 = (assign11930_e11241 * assign11930_e11252);
            let assign11930_e11254: f64 = (1.0 - assign11930_e11253);
            let assign11930_e11255: f64 = (0.1666666666667 * assign11930_e11254);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11930_e11255, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign11930_e11252) + (assign11930_e11241 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign11930_e11250) + (assign11930_e11245 * (-(locals.var_q_qsq_dn9 * 0.025))))))))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11940_e11266: f64 = (locals.var_q_qsq * locals.var_q_temp3);
            let assign11940_e11267: f64 = (2.0 + assign11940_e11266);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign11940_e11267, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)), );
        }

    }

    pub(super) fn stamp_transient_block_13(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11950_e11279: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign11950_e11283: f64 = (locals.var_q_qsq * 0.0357142857143);
            let assign11950_e11287: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign11950_e11288: f64 = (1.0 - assign11950_e11287);
            let assign11950_e11289: f64 = (assign11950_e11283 * assign11950_e11288);
            let assign11950_e11290: f64 = (1.0 - assign11950_e11289);
            let assign11950_e11291: f64 = (assign11950_e11279 * assign11950_e11290);
            let assign11950_e11292: f64 = (1.0 - assign11950_e11291);
            let assign11950_e11293: f64 = (0.1666666666667 * assign11950_e11292);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11950_e11293, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign11950_e11290) + (assign11950_e11279 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign11950_e11288) + (assign11950_e11283 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11960_e11303: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign11960_e11303, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11970_e11315: f64 = (locals.var_q_qsq * 0.0714285714286);
            let assign11970_e11319: f64 = (0.05 * locals.var_q_qsq);
            let assign11970_e11323: f64 = (0.0420875420875421 * locals.var_q_qsq);
            let assign11970_e11324: f64 = (1.0 - assign11970_e11323);
            let assign11970_e11325: f64 = (assign11970_e11319 * assign11970_e11324);
            let assign11970_e11326: f64 = (1.0 - assign11970_e11325);
            let assign11970_e11327: f64 = (assign11970_e11315 * assign11970_e11326);
            let assign11970_e11328: f64 = (1.0 - assign11970_e11327);
            let assign11970_e11329: f64 = (0.0055555555556 * assign11970_e11328);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11970_e11329, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn4) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn6) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn7) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn8) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign11970_e11326) + (assign11970_e11315 * (-(((0.05 * locals.var_q_qsq_dn9) * assign11970_e11324) + (assign11970_e11319 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11980_e11339: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
            let assign11980_e11342: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
            let assign11980_e11344: f64 = (assign11980_e11342 * locals.var_q_temp2);
            let assign11980_e11345: f64 = (assign11980_e11339 - assign11980_e11344);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign11980_e11345, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign11980_e11342 * locals.var_q_temp2_dn9))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11990_e11354: f64 = (-0.5);
            let assign11990_e11356: f64 = (assign11990_e11354 * locals.var_q_d1_qsq);
            let assign11990_e11358: f64 = (assign11990_e11356 * locals.var_q_temp3);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign11990_e11358, (((assign11990_e11354 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn4)), (((assign11990_e11354 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn6)), (((assign11990_e11354 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn7)), (((assign11990_e11354 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn8)), (((assign11990_e11354 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign11990_e11356 * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign12000_e11367: f64 = (-0.5);
            let assign12000_e11369: f64 = (assign12000_e11367 * locals.var_q_d2_qsq);
            let assign12000_e11371: f64 = (assign12000_e11369 * locals.var_q_temp3);
            let assign12000_e11374: f64 = (0.25 * 0.0055555555556);
            let assign12000_e11376: f64 = (assign12000_e11374 * locals.var_q_d1_qsq);
            let assign12000_e11378: f64 = (assign12000_e11376 * locals.var_q_d1_qsq);
            let assign12000_e11382: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign12000_e11386: f64 = (0.075 * locals.var_q_qsq);
            let assign12000_e11387: f64 = (2.0 - assign12000_e11386);
            let assign12000_e11388: f64 = (assign12000_e11382 * assign12000_e11387);
            let assign12000_e11389: f64 = (1.0 - assign12000_e11388);
            let assign12000_e11390: f64 = (assign12000_e11378 * assign12000_e11389);
            let assign12000_e11391: f64 = (assign12000_e11371 + assign12000_e11390);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign12000_e11391, ((((assign12000_e11367 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn4)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn4)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign12000_e11367 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn6)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn6)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign12000_e11367 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn7)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn7)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign12000_e11367 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn8)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn8)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign12000_e11367 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign12000_e11369 * locals.var_q_temp3_dn9)) + (((((assign12000_e11374 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign12000_e11376 * locals.var_q_d1_qsq_dn9)) * assign12000_e11389) + (assign12000_e11378 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign12000_e11387) + (assign12000_e11382 * (-(0.075 * locals.var_q_qsq_dn9)))))))), );
        }

        let assign12010_e11396: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard555 = assign12010_e11396;

        if (locals.var_guard555 != 0.0) {
            let assign12020_e11400: f64 = (4.0 * locals.var_q_qsq);
            let assign12020_e11405: f64 = (2.0 - locals.var_q_invexpq);
            let assign12020_e11406: f64 = (locals.var_q_invexpq * assign12020_e11405);
            let assign12020_e11407: f64 = (1.0 - assign12020_e11406);
            let assign12020_e11408: f64 = (assign12020_e11400 / assign12020_e11407);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign12020_e11408, ((((4.0 * locals.var_q_qsq_dn4) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn4 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign12020_e11407 * assign12020_e11407)), ((((4.0 * locals.var_q_qsq_dn6) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn6 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign12020_e11407 * assign12020_e11407)), ((((4.0 * locals.var_q_qsq_dn7) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn7 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign12020_e11407 * assign12020_e11407)), ((((4.0 * locals.var_q_qsq_dn8) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn8 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign12020_e11407 * assign12020_e11407)), ((((4.0 * locals.var_q_qsq_dn9) * assign12020_e11407) - (assign12020_e11400 * (-((locals.var_q_invexpq_dn9 * assign12020_e11405) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign12020_e11407 * assign12020_e11407)), );
        }

        if (locals.var_guard555 != 0.0) {
            let assign12030_e11414: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign12030_e11414, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)), );
        }

        if (locals.var_guard555 != 0.0) {
            let assign12040_e11419: f64 = (locals.var_q_temp2).ln();
            let assign12040_e11421: f64 = (assign12040_e11419 - locals.var_q_rac_qsq);
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign12040_e11421, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9), );
        }

        let assign12050_e11426: f64 = (-0.005);
        let assign12050_e11427: f64 = if locals.var_q_qsq < assign12050_e11426 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign12050_e11427;

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
            let assign12060_e11434: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign12060_e11435: f64 = (assign12060_e11434).sin();
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign12060_e11435, ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign12060_e11434).cos() * (0.5 * locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
            let assign12070_e11443: f64 = (-locals.var_q_qsq);
            let assign12070_e11446: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
            let assign12070_e11447: f64 = (assign12070_e11443 / assign12070_e11446);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign12070_e11447, ((((-locals.var_q_qsq_dn4) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign12070_e11446 * assign12070_e11446)), ((((-locals.var_q_qsq_dn6) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign12070_e11446 * assign12070_e11446)), ((((-locals.var_q_qsq_dn7) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign12070_e11446 * assign12070_e11446)), ((((-locals.var_q_qsq_dn8) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign12070_e11446 * assign12070_e11446)), ((((-locals.var_q_qsq_dn9) * assign12070_e11446) - (assign12070_e11443 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign12070_e11446 * assign12070_e11446)), );
        }

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
            let assign12080_e11455: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign12080_e11455, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 == 0.0)) {
            let assign12090_e11466: f64 = (locals.var_q_qsq * 0.3333333333333);
            let assign12090_e11470: f64 = (0.05 * locals.var_q_qsq);
            let assign12090_e11474: f64 = (0.0396825396825397 * locals.var_q_qsq);
            let assign12090_e11475: f64 = (1.0 - assign12090_e11474);
            let assign12090_e11476: f64 = (assign12090_e11470 * assign12090_e11475);
            let assign12090_e11477: f64 = (1.0 - assign12090_e11476);
            let assign12090_e11478: f64 = (assign12090_e11466 * assign12090_e11477);
            let assign12090_e11479: f64 = (4.0 - assign12090_e11478);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign12090_e11479, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn4) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn6) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn7) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn8) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign12090_e11477) + (assign12090_e11466 * (-(((0.05 * locals.var_q_qsq_dn9) * assign12090_e11475) + (assign12090_e11470 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))), );
        }

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 == 0.0)) {
            let assign12100_e11488: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign12100_e11488, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        let assign12110_e11493: f64 = (1.01 * locals.var_q_k1q1);
        let assign12110_e11495: f64 = (assign12110_e11493 + locals.var_q_qcoth);
        let assign12110_e11497: f64 = if assign12110_e11495 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign12110_e11497;

        if (locals.var_guard557 != 0.0) {
            let assign12120_e11501: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign12120_e11501, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9), );
        }

        if (locals.var_guard557 != 0.0) {
            let assign12130_e11507: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign12130_e11507, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9), );
        }

        if (locals.var_guard557 != 0.0) {
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12150_e11519: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
            let assign12150_e11520: f64 = (1.0 / assign12150_e11519);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign12150_e11520, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign12150_e11519 * assign12150_e11519))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign12150_e11519 * assign12150_e11519))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign12150_e11519 * assign12150_e11519))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign12150_e11519 * assign12150_e11519))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign12150_e11519 * assign12150_e11519))), );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12160_e11527: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign12160_e11527, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9), );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12170_e11534: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
            let assign12170_e11536: f64 = (assign12170_e11534 * locals.var_q_temp2);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign12170_e11536, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign12170_e11534 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12180_e11543: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
            let assign12180_e11545: f64 = (assign12180_e11543 - locals.var_q_aexp);
            let assign12180_e11548: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
            let assign12180_e11549: f64 = (assign12180_e11545 - assign12180_e11548);
            let assign12180_e11551: f64 = (assign12180_e11549 * locals.var_q_temp2);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign12180_e11551, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign12180_e11549 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12190_e11558: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
            let assign12190_e11561: f64 = (2.0 * locals.var_q_temp3);
            let assign12190_e11563: f64 = (assign12190_e11561 * locals.var_q_d1_expnum);
            let assign12190_e11564: f64 = (assign12190_e11558 + assign12190_e11563);
            let assign12190_e11566: f64 = (assign12190_e11564 + locals.var_q_aexp);
            let assign12190_e11570: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
            let assign12190_e11571: f64 = (locals.var_q_d2_ln + assign12190_e11570);
            let assign12190_e11573: f64 = (assign12190_e11571 * locals.var_q_sh_term);
            let assign12190_e11574: f64 = (assign12190_e11566 - assign12190_e11573);
            let assign12190_e11576: f64 = (assign12190_e11574 * locals.var_q_temp2);
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (assign12190_e11576, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign12190_e11561 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign12190_e11571 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign12190_e11574 * locals.var_q_temp2_dn9)), );
        }

        let assign12200_e11581: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard558 = assign12200_e11581;

        if (locals.var_guard558 != 0.0) {
            let assign12210_e11584: f64 = (locals.var_q_expnum).ln();
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign12210_e11584, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum), );
        }

        if (locals.var_guard558 != 0.0) {
            let assign12220_e11590: f64 = (1.0 / locals.var_q_expnum);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12220_e11590, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))), );
        }

        if (locals.var_guard558 != 0.0) {
            let assign12230_e11596: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign12230_e11596, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard558 != 0.0) {
            let assign12240_e11602: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
            let assign12240_e11605: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
            let assign12240_e11606: f64 = (assign12240_e11602 - assign12240_e11605);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign12240_e11606, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))), );
        }

        if (locals.var_guard558 == 0.0) {
            let assign12250_e11613: f64 = (locals.var_q_k1q1 + 0.6931471805599);
            let assign12250_e11615: f64 = (-locals.var_q_k1q1);
            let assign12250_e11616: f64 = (assign12250_e11615).ln();
            let assign12250_e11617: f64 = (assign12250_e11613 + assign12250_e11616);
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign12250_e11617, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign12250_e11615)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign12250_e11615)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign12250_e11615)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign12250_e11615)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign12250_e11615)), );
        }

        if (locals.var_guard558 == 0.0) {
            let assign12260_e11624: f64 = (1.0 / locals.var_q1s);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12260_e11624, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))), );
        }

        if (locals.var_guard558 == 0.0) {
            let assign12270_e11631: f64 = (locals.var_k1 + locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign12270_e11631, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9), );
        }

        if (locals.var_guard558 == 0.0) {
            let assign12280_e11637: f64 = (-locals.var_q_temp1);
            let assign12280_e11639: f64 = (assign12280_e11637 * locals.var_q_temp1);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign12280_e11639, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign12280_e11637 * locals.var_q_temp1_dn9)), );
        }

        let assign12290_e11644: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign12290_e11646: f64 = (assign12290_e11644 + locals.var_q1s);
        let assign12290_e11649: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign12290_e11650: f64 = (assign12290_e11646 + assign12290_e11649);
        let assign12290_e11652: f64 = (assign12290_e11650 - locals.var_q_ln_term);
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9, ) = (assign12290_e11652, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9), );

        let assign12300_e11656: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign12300_e11657: f64 = (1.0 + assign12300_e11656);
        let assign12300_e11659: f64 = (assign12300_e11657 - locals.var_q_d1_ln);
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9, ) = (assign12300_e11659, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9), );

        let assign12310_e11662: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign12310_e11664: f64 = (assign12310_e11662 - locals.var_q_d2_ln);
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9, ) = (assign12310_e11664, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9), );

        let assign12320_e11668: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign12320_e11669: f64 = (locals.var_q_k1q1 + assign12320_e11668);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign12320_e11669, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))), );

        let assign12330_e11673: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign12330_e11674: f64 = (locals.var_k1 + assign12330_e11673);
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9, ) = (assign12330_e11674, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))), );

        let assign12340_e11677: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9, ) = (assign12340_e11677, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)), );

        let assign12350_e11680: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign12350_e11682: f64 = (assign12350_e11680 - locals.var_q_aexp);
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign12350_e11682, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9), );

        let assign12360_e11685: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign12360_e11688: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign12360_e11689: f64 = (assign12360_e11685 + assign12360_e11688);
        let assign12360_e11691: f64 = (assign12360_e11689 + locals.var_q_aexp);
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign12360_e11691, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9), );

        let assign12370_e11694: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign12370_e11697: f64 = (2.0 * locals.var_q_d1_qi);
        let assign12370_e11699: f64 = (assign12370_e11697 * locals.var_q_d1_expnum);
        let assign12370_e11700: f64 = (assign12370_e11694 + assign12370_e11699);
        let assign12370_e11703: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign12370_e11704: f64 = (assign12370_e11700 + assign12370_e11703);
        let assign12370_e11706: f64 = (assign12370_e11704 - locals.var_q_aexp);
        (locals.var_q_d2_zero, locals.var_q_d2_zero_dn4, locals.var_q_d2_zero_dn6, locals.var_q_d2_zero_dn7, locals.var_q_d2_zero_dn8, locals.var_q_d2_zero_dn9, ) = (assign12370_e11706, (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4), (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6), (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7), (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8), (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign12370_e11697 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9), );

        let assign12380_e11709: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign12380_e11712: f64 = (0.5 * locals.var_q_zero);
        let assign12380_e11714: f64 = (assign12380_e11712 * locals.var_q_d2_zero);
        let assign12380_e11715: f64 = (assign12380_e11709 - assign12380_e11714);
        (locals.var_q_temp, locals.var_q_temp_dn4, locals.var_q_temp_dn6, locals.var_q_temp_dn7, locals.var_q_temp_dn8, locals.var_q_temp_dn9, ) = (assign12380_e11715, (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn4))), (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn6))), (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn7))), (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn8))), (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign12380_e11712 * locals.var_q_d2_zero_dn9))), );

        let assign12390_e11717: f64 = (-locals.var_q_zero);
        let assign12390_e11719: f64 = (assign12390_e11717 * locals.var_q_d1_zero);
        let assign12390_e11721: f64 = (assign12390_e11719 * locals.var_q_temp);
        let assign12390_e11724: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign12390_e11726: f64 = (assign12390_e11724 + 1e-200);
        let assign12390_e11727: f64 = (assign12390_e11721 / assign12390_e11726);
        (locals.var_q_eps2, locals.var_q_eps2_dn4, locals.var_q_eps2_dn6, locals.var_q_eps2_dn7, locals.var_q_eps2_dn8, locals.var_q_eps2_dn9, ) = (assign12390_e11727, ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn4)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign12390_e11726 * assign12390_e11726)), ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn6)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign12390_e11726 * assign12390_e11726)), ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn7)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign12390_e11726 * assign12390_e11726)), ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn8)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign12390_e11726 * assign12390_e11726)), ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign12390_e11717 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign12390_e11719 * locals.var_q_temp_dn9)) * assign12390_e11726) - (assign12390_e11721 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign12390_e11726 * assign12390_e11726)), );

        let assign12400_e11730: f64 = (locals.var_q1s + locals.var_q_eps2);
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign12400_e11730, (locals.var_q1s_dn4 + locals.var_q_eps2_dn4), (locals.var_q1s_dn6 + locals.var_q_eps2_dn6), (locals.var_q1s_dn7 + locals.var_q_eps2_dn7), (locals.var_q1s_dn8 + locals.var_q_eps2_dn8), (locals.var_q1s_dn9 + locals.var_q_eps2_dn9), );

        let assign12410_e11733: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign12410_e11733, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign12420_e11736: f64 = (locals.var_k2 * locals.var_q2s);
        (locals.var_q_k2q2, locals.var_q_k2q2_dn4, locals.var_q_k2q2_dn6, locals.var_q_k2q2_dn7, locals.var_q_k2q2_dn8, locals.var_q_k2q2_dn9, ) = (assign12420_e11736, ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4)), ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6)), ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7)), ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8)), ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9)), );

        let assign12430_e11739: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign12430_e11739, (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9), );

        let assign12440_e11743: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign12440_e11744: f64 = (1.0 + assign12440_e11743);
        (locals.var_q_a, locals.var_q_a_dn4, locals.var_q_a_dn6, locals.var_q_a_dn7, locals.var_q_a_dn8, locals.var_q_a_dn9, ) = (assign12440_e11744, (0.065345483024 * locals.var_q_qi_int_dn4), (0.065345483024 * locals.var_q_qi_int_dn6), (0.065345483024 * locals.var_q_qi_int_dn7), (0.065345483024 * locals.var_q_qi_int_dn8), (0.065345483024 * locals.var_q_qi_int_dn9), );

        let assign12450_e11748: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign12450_e11749: f64 = (39.478417604 + assign12450_e11748);
        let assign12450_e11752: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12450_e11753: f64 = (assign12450_e11749 + assign12450_e11752);
        (locals.var_q_b, locals.var_q_b_dn4, locals.var_q_b_dn6, locals.var_q_b_dn7, locals.var_q_b_dn8, locals.var_q_b_dn9, ) = (assign12450_e11753, ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))), ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))), ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))), ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))), ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))), );

        let assign12460_e11757: f64 = (2.0 * locals.var_q_qi_int);
        let assign12460_e11760: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12460_e11761: f64 = (assign12460_e11757 + assign12460_e11760);
        let assign12460_e11762: f64 = (39.478417604 * assign12460_e11761);
        (locals.var_q_c, locals.var_q_c_dn4, locals.var_q_c_dn6, locals.var_q_c_dn7, locals.var_q_c_dn8, locals.var_q_c_dn9, ) = (assign12460_e11762, (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)))), );

        let assign12470_e11765: f64 = (locals.var_q_b * locals.var_q_b);
        let assign12470_e11768: f64 = (4.0 * locals.var_q_a);
        let assign12470_e11770: f64 = (assign12470_e11768 * locals.var_q_c);
        let assign12470_e11771: f64 = (assign12470_e11765 - assign12470_e11770);
        let assign12470_e11772: f64 = (assign12470_e11771).sqrt();
        (locals.var_q_disc, locals.var_q_disc_dn4, locals.var_q_disc_dn6, locals.var_q_disc_dn7, locals.var_q_disc_dn8, locals.var_q_disc_dn9, ) = (assign12470_e11772, ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn4))) / (2.0 * assign12470_e11772)), ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn6))) / (2.0 * assign12470_e11772)), ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn7))) / (2.0 * assign12470_e11772)), ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn8))) / (2.0 * assign12470_e11772)), ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign12470_e11768 * locals.var_q_c_dn9))) / (2.0 * assign12470_e11772)), );

        let assign12480_e11775: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign12480_e11778: f64 = (2.0 * locals.var_q_a);
        let assign12480_e11779: f64 = (assign12480_e11775 / assign12480_e11778);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign12480_e11779, ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn4))) / (assign12480_e11778 * assign12480_e11778)), ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn6))) / (assign12480_e11778 * assign12480_e11778)), ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn7))) / (assign12480_e11778 * assign12480_e11778)), ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn8))) / (assign12480_e11778 * assign12480_e11778)), ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign12480_e11778) - (assign12480_e11775 * (2.0 * locals.var_q_a_dn9))) / (assign12480_e11778 * assign12480_e11778)), );

        let assign12490_e11782: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12490_e11784: f64 = (assign12490_e11782 - locals.var_q_qsq);
        (locals.var_q_delta, locals.var_q_delta_dn4, locals.var_q_delta_dn6, locals.var_q_delta_dn7, locals.var_q_delta_dn8, locals.var_q_delta_dn9, ) = (assign12490_e11784, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9), );

        let assign12500_e11787: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign12500_e11787;

        if (locals.var_guard559 != 0.0) {
            let assign12510_e11792: f64 = (locals.var_q_delta / locals.var_a0);
            let assign12510_e11793: f64 = (assign12510_e11792).ln();
            let assign12510_e11795: f64 = assign12510_e11793;
            let assign12510_e11797: f64 = (assign12510_e11795 - locals.var_xg1x);
            let assign12510_e11799: f64 = (assign12510_e11797 + locals.var_q1s);
            let assign12510_e11800: f64 = (locals.var_q_delta * assign12510_e11799);
            (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign12510_e11800, ((locals.var_q_delta_dn4 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn4) + locals.var_q1s_dn4))), ((locals.var_q_delta_dn6 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn6) + locals.var_q1s_dn6))), ((locals.var_q_delta_dn7 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn7) + locals.var_q1s_dn7))), ((locals.var_q_delta_dn8 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn8) + locals.var_q1s_dn8))), ((locals.var_q_delta_dn9 * assign12510_e11799) + (locals.var_q_delta * ((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign12510_e11792) - locals.var_xg1x_dn9) + locals.var_q1s_dn9))), );
        }

        if (locals.var_guard559 != 0.0) {
            let assign12520_e11806: f64 = (2.0 * locals.var_k1);
            let assign12520_e11808: f64 = (assign12520_e11806 * locals.var_q_k1q1);
            let assign12520_e11810: f64 = (assign12520_e11808 + locals.var_q_delta);
            (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign12520_e11810, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12520_e11806 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9), );
        }

        if (locals.var_guard559 != 0.0) {
            let assign12530_e11816: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12530_e11818: f64 = (assign12530_e11816 - locals.var_q_x1sat);
            locals.var_q_dx1 = assign12530_e11818;
        }

        let assign12540_e11830: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign12540_e11832: f64 = (locals.var_k1).ln();
        let assign12540_e11833: f64 = (assign12540_e11830 + assign12540_e11832);
        let assign12540_e11840: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign12540_e11833 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard560 = assign12540_e11840;

        if ((locals.var_guard559 != 0.0) && (locals.var_guard560 != 0.0)) {
            let assign12550_e11847: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
            let assign12550_e11848: f64 = (locals.var_q1s - assign12550_e11847);
            (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign12550_e11848, (locals.var_q1s_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), );
        }

        let assign12560_e11853: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign12560_e11853, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign12570_e11856: f64 = (locals.var_k2 * locals.var_q2s);
        (locals.var_q_k2q2, locals.var_q_k2q2_dn4, locals.var_q_k2q2_dn6, locals.var_q_k2q2_dn7, locals.var_q_k2q2_dn8, locals.var_q_k2q2_dn9, ) = (assign12570_e11856, ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4)), ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6)), ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7)), ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8)), ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9)), );

        let assign12580_e11859: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign12580_e11859, (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9), );

        let assign12590_e11863: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign12590_e11864: f64 = (1.0 + assign12590_e11863);
        (locals.var_q_a, locals.var_q_a_dn4, locals.var_q_a_dn6, locals.var_q_a_dn7, locals.var_q_a_dn8, locals.var_q_a_dn9, ) = (assign12590_e11864, (0.065345483024 * locals.var_q_qi_int_dn4), (0.065345483024 * locals.var_q_qi_int_dn6), (0.065345483024 * locals.var_q_qi_int_dn7), (0.065345483024 * locals.var_q_qi_int_dn8), (0.065345483024 * locals.var_q_qi_int_dn9), );

        let assign12600_e11868: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign12600_e11869: f64 = (39.478417604 + assign12600_e11868);
        let assign12600_e11872: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12600_e11873: f64 = (assign12600_e11869 + assign12600_e11872);
        (locals.var_q_b, locals.var_q_b_dn4, locals.var_q_b_dn6, locals.var_q_b_dn7, locals.var_q_b_dn8, locals.var_q_b_dn9, ) = (assign12600_e11873, ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))), ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))), ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))), ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))), ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))), );

        let assign12610_e11877: f64 = (2.0 * locals.var_q_qi_int);
        let assign12610_e11880: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12610_e11881: f64 = (assign12610_e11877 + assign12610_e11880);
        let assign12610_e11882: f64 = (39.478417604 * assign12610_e11881);
        (locals.var_q_c, locals.var_q_c_dn4, locals.var_q_c_dn6, locals.var_q_c_dn7, locals.var_q_c_dn8, locals.var_q_c_dn9, ) = (assign12610_e11882, (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)))), );

        let assign12620_e11885: f64 = (locals.var_q_b * locals.var_q_b);
        let assign12620_e11888: f64 = (4.0 * locals.var_q_a);
        let assign12620_e11890: f64 = (assign12620_e11888 * locals.var_q_c);
        let assign12620_e11891: f64 = (assign12620_e11885 - assign12620_e11890);
        let assign12620_e11892: f64 = (assign12620_e11891).sqrt();
        (locals.var_q_disc, locals.var_q_disc_dn4, locals.var_q_disc_dn6, locals.var_q_disc_dn7, locals.var_q_disc_dn8, locals.var_q_disc_dn9, ) = (assign12620_e11892, ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn4))) / (2.0 * assign12620_e11892)), ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn6))) / (2.0 * assign12620_e11892)), ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn7))) / (2.0 * assign12620_e11892)), ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn8))) / (2.0 * assign12620_e11892)), ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign12620_e11888 * locals.var_q_c_dn9))) / (2.0 * assign12620_e11892)), );

        let assign12630_e11895: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign12630_e11898: f64 = (2.0 * locals.var_q_a);
        let assign12630_e11899: f64 = (assign12630_e11895 / assign12630_e11898);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign12630_e11899, ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn4))) / (assign12630_e11898 * assign12630_e11898)), ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn6))) / (assign12630_e11898 * assign12630_e11898)), ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn7))) / (assign12630_e11898 * assign12630_e11898)), ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn8))) / (assign12630_e11898 * assign12630_e11898)), ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign12630_e11898) - (assign12630_e11895 * (2.0 * locals.var_q_a_dn9))) / (assign12630_e11898 * assign12630_e11898)), );

        let assign12640_e11902: f64 = (-0.005);
        let assign12640_e11903: f64 = if locals.var_q_qsq < assign12640_e11902 { 1.0 } else { 0.0 };
        locals.var_guard561 = assign12640_e11903;

        if (locals.var_guard561 != 0.0) {
            let assign12650_e11906: f64 = (locals.var_q_qsq).abs();
            let assign12650_e11907: f64 = (assign12650_e11906).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign12650_e11907, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12650_e11907)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12650_e11907)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12650_e11907)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12650_e11907)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12650_e11907)), );
        }

        if (locals.var_guard561 != 0.0) {
            let assign12660_e11914: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign12660_e11915: f64 = (assign12660_e11914).tan();
            let assign12660_e11916: f64 = (locals.var_q_rac_qsq / assign12660_e11915);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12660_e11916, (((locals.var_q_rac_qsq_dn4 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), (((locals.var_q_rac_qsq_dn6 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), (((locals.var_q_rac_qsq_dn7 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), (((locals.var_q_rac_qsq_dn8 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), (((locals.var_q_rac_qsq_dn9 * assign12660_e11915) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign12660_e11914).cos() * (assign12660_e11914).cos())))) / (assign12660_e11915 * assign12660_e11915)), );
        }

        if (locals.var_guard561 != 0.0) {
            let assign12670_e11925: f64 = (2.0 - locals.var_q_qcoth);
            let assign12670_e11926: f64 = (locals.var_q_qcoth * assign12670_e11925);
            let assign12670_e11927: f64 = (locals.var_q_qsq + assign12670_e11926);
            let assign12670_e11928: f64 = (0.25 * assign12670_e11927);
            let assign12670_e11930: f64 = (assign12670_e11928 / locals.var_q_qsq);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12670_e11930, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12670_e11925) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign12670_e11928 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        let assign12680_e11935: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard562 = assign12680_e11935;

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
            let assign12690_e11941: f64 = (locals.var_q_qsq).abs();
            let assign12690_e11942: f64 = (assign12690_e11941).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign12690_e11942, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12690_e11942)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12690_e11942)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12690_e11942)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12690_e11942)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12690_e11942)), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
            let assign12700_e11950: f64 = (-locals.var_q_rac_qsq);
            let assign12700_e11951: f64 = (assign12700_e11950).exp();
            (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (assign12700_e11951, (assign12700_e11951 * (-locals.var_q_rac_qsq_dn4)), (assign12700_e11951 * (-locals.var_q_rac_qsq_dn6)), (assign12700_e11951 * (-locals.var_q_rac_qsq_dn7)), (assign12700_e11951 * (-locals.var_q_rac_qsq_dn8)), (assign12700_e11951 * (-locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
            let assign12710_e11961: f64 = (1.0 + locals.var_q_invexpq);
            let assign12710_e11962: f64 = (locals.var_q_rac_qsq * assign12710_e11961);
            let assign12710_e11965: f64 = (1.0 - locals.var_q_invexpq);
            let assign12710_e11966: f64 = (assign12710_e11962 / assign12710_e11965);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12710_e11966, (((((locals.var_q_rac_qsq_dn4 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn4))) / (assign12710_e11965 * assign12710_e11965)), (((((locals.var_q_rac_qsq_dn6 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn6))) / (assign12710_e11965 * assign12710_e11965)), (((((locals.var_q_rac_qsq_dn7 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn7))) / (assign12710_e11965 * assign12710_e11965)), (((((locals.var_q_rac_qsq_dn8 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn8))) / (assign12710_e11965 * assign12710_e11965)), (((((locals.var_q_rac_qsq_dn9 * assign12710_e11961) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign12710_e11965) - (assign12710_e11962 * (-locals.var_q_invexpq_dn9))) / (assign12710_e11965 * assign12710_e11965)), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
            let assign12720_e11978: f64 = (2.0 - locals.var_q_qcoth);
            let assign12720_e11979: f64 = (locals.var_q_qcoth * assign12720_e11978);
            let assign12720_e11980: f64 = (locals.var_q_qsq + assign12720_e11979);
            let assign12720_e11981: f64 = (0.25 * assign12720_e11980);
            let assign12720_e11983: f64 = (assign12720_e11981 / locals.var_q_qsq);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12720_e11983, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12720_e11978) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign12720_e11981 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 == 0.0)) {
            let assign12730_e11994: f64 = (locals.var_q_qsq * 0.1666666666667);
            let assign12730_e11998: f64 = (locals.var_q_qsq * 0.0166666666667);
            let assign12730_e12002: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign12730_e12003: f64 = (1.0 - assign12730_e12002);
            let assign12730_e12004: f64 = (assign12730_e11998 * assign12730_e12003);
            let assign12730_e12005: f64 = (1.0 - assign12730_e12004);
            let assign12730_e12006: f64 = (assign12730_e11994 * assign12730_e12005);
            let assign12730_e12007: f64 = (2.0 + assign12730_e12006);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12730_e12007, (((locals.var_q_qsq_dn4 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq_dn6 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq_dn7 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq_dn8 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq_dn9 * 0.1666666666667) * assign12730_e12005) + (assign12730_e11994 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign12730_e12003) + (assign12730_e11998 * (-(locals.var_q_qsq_dn9 * 0.0238095238095))))))), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 == 0.0)) {
            let assign12740_e12019: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign12740_e12023: f64 = (locals.var_q_qsq * 0.0357142857143);
            let assign12740_e12027: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign12740_e12028: f64 = (1.0 - assign12740_e12027);
            let assign12740_e12029: f64 = (assign12740_e12023 * assign12740_e12028);
            let assign12740_e12030: f64 = (1.0 - assign12740_e12029);
            let assign12740_e12031: f64 = (assign12740_e12019 * assign12740_e12030);
            let assign12740_e12032: f64 = (1.0 - assign12740_e12031);
            let assign12740_e12033: f64 = (0.1666666666667 * assign12740_e12032);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12740_e12033, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign12740_e12030) + (assign12740_e12019 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign12740_e12028) + (assign12740_e12023 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))), );
        }

        let assign12750_e12039: f64 = (locals.var_q_qi_int * locals.var_q_qcoth);
        let assign12750_e12042: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12750_e12043: f64 = (assign12750_e12039 + assign12750_e12042);
        let assign12750_e12045: f64 = (assign12750_e12043 + locals.var_q_qsq);
        let assign12750_e12048: f64 = (locals.var_q_qi_int * locals.var_q_d1_qcoth);
        let assign12750_e12050: f64 = (assign12750_e12048 + 1.0);
        let assign12750_e12051: f64 = (assign12750_e12045 / assign12750_e12050);
        let assign12750_e12052: f64 = (locals.var_q_qsq - assign12750_e12051);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign12750_e12052, (locals.var_q_qsq_dn4 - (((((((locals.var_q_qi_int_dn4 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn4)) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))) + locals.var_q_qsq_dn4) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn4 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn4)))) / (assign12750_e12050 * assign12750_e12050))), (locals.var_q_qsq_dn6 - (((((((locals.var_q_qi_int_dn6 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn6)) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))) + locals.var_q_qsq_dn6) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn6 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn6)))) / (assign12750_e12050 * assign12750_e12050))), (locals.var_q_qsq_dn7 - (((((((locals.var_q_qi_int_dn7 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn7)) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))) + locals.var_q_qsq_dn7) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn7 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn7)))) / (assign12750_e12050 * assign12750_e12050))), (locals.var_q_qsq_dn8 - (((((((locals.var_q_qi_int_dn8 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn8)) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))) + locals.var_q_qsq_dn8) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn8 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn8)))) / (assign12750_e12050 * assign12750_e12050))), (locals.var_q_qsq_dn9 - (((((((locals.var_q_qi_int_dn9 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn9)) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))) + locals.var_q_qsq_dn9) * assign12750_e12050) - (assign12750_e12045 * ((locals.var_q_qi_int_dn9 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn9)))) / (assign12750_e12050 * assign12750_e12050))), );

        let assign12760_e12055: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12760_e12057: f64 = (assign12760_e12055 - locals.var_q_qsq);
        (locals.var_q_delta, locals.var_q_delta_dn4, locals.var_q_delta_dn6, locals.var_q_delta_dn7, locals.var_q_delta_dn8, locals.var_q_delta_dn9, ) = (assign12760_e12057, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9), );

        let assign12770_e12060: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign12770_e12060;

    }

    pub(super) fn stamp_transient_block_14(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard563 != 0.0) {
            let assign12780_e12065: f64 = (locals.var_q_delta / locals.var_a0);
            let assign12780_e12066: f64 = (assign12780_e12065).ln();
            let assign12780_e12068: f64 = assign12780_e12066;
            let assign12780_e12070: f64 = (assign12780_e12068 - locals.var_xg1x);
            let assign12780_e12072: f64 = (assign12780_e12070 + locals.var_q1s);
            let assign12780_e12073: f64 = (locals.var_q_delta * assign12780_e12072);
            (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign12780_e12073, ((locals.var_q_delta_dn4 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn4) + locals.var_q1s_dn4))), ((locals.var_q_delta_dn6 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn6) + locals.var_q1s_dn6))), ((locals.var_q_delta_dn7 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn7) + locals.var_q1s_dn7))), ((locals.var_q_delta_dn8 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn8) + locals.var_q1s_dn8))), ((locals.var_q_delta_dn9 * assign12780_e12072) + (locals.var_q_delta * ((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign12780_e12065) - locals.var_xg1x_dn9) + locals.var_q1s_dn9))), );
        }

        if (locals.var_guard563 != 0.0) {
            let assign12790_e12079: f64 = (2.0 * locals.var_k1);
            let assign12790_e12081: f64 = (assign12790_e12079 * locals.var_q_k1q1);
            let assign12790_e12083: f64 = (assign12790_e12081 + locals.var_q_delta);
            (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign12790_e12083, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12790_e12079 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9), );
        }

        if (locals.var_guard563 != 0.0) {
            let assign12800_e12089: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12800_e12091: f64 = (assign12800_e12089 - locals.var_q_x1sat);
            locals.var_q_dx1 = assign12800_e12091;
        }

        let assign12810_e12103: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign12810_e12105: f64 = (locals.var_k1).ln();
        let assign12810_e12106: f64 = (assign12810_e12103 + assign12810_e12105);
        let assign12810_e12113: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign12810_e12106 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard564 = assign12810_e12113;

        if ((locals.var_guard563 != 0.0) && (locals.var_guard564 != 0.0)) {
            let assign12820_e12120: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
            let assign12820_e12121: f64 = (locals.var_q1s - assign12820_e12120);
            (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign12820_e12121, (locals.var_q1s_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), );
        }

        let assign12830_e12126: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign12830_e12126, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign12840_e12129: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12840_e12131: f64 = assign12840_e12129;
        let assign12840_e12133: f64 = if assign12840_e12131 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard565 = assign12840_e12133;

        if (locals.var_guard565 != 0.0) {
            let assign12850_e12137: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12850_e12139: f64 = assign12850_e12137;
            let assign12850_e12140: f64 = (assign12850_e12139).exp();
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12850_e12140, (assign12850_e12140 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign12850_e12140 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign12850_e12140 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign12850_e12140 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign12850_e12140 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)), );
        }

        if (locals.var_guard565 == 0.0) {
            let assign12860_e12149: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12860_e12151: f64 = assign12860_e12149;
            let assign12860_e12153: f64 = (assign12860_e12151 - 80.0);
            let assign12860_e12158: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12860_e12160: f64 = assign12860_e12158;
            let assign12860_e12162: f64 = (assign12860_e12160 - 80.0);
            let assign12860_e12163: f64 = (0.5 * assign12860_e12162);
            let assign12860_e12167: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12860_e12169: f64 = assign12860_e12167;
            let assign12860_e12171: f64 = (assign12860_e12169 - 80.0);
            let assign12860_e12173: f64 = (assign12860_e12171 * 0.3333333333333);
            let assign12860_e12174: f64 = (1.0 + assign12860_e12173);
            let assign12860_e12175: f64 = (assign12860_e12163 * assign12860_e12174);
            let assign12860_e12176: f64 = (1.0 + assign12860_e12175);
            let assign12860_e12177: f64 = (assign12860_e12153 * assign12860_e12176);
            let assign12860_e12178: f64 = (1.0 + assign12860_e12177);
            let assign12860_e12179: f64 = (5.54062e34 * assign12860_e12178);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12860_e12179, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign12860_e12176) + (assign12860_e12153 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign12860_e12174) + (assign12860_e12163 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))), );
        }

        let assign12870_e12184: f64 = (locals.var_a0 * locals.var_q_temp1);
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9, ) = (assign12870_e12184, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)), );

        let assign12880_e12187: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12880_e12189: f64 = (assign12880_e12187 - locals.var_q_aexp);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign12880_e12189, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9), );

        let assign12890_e12192: f64 = (2.0 * locals.var_k1);
        let assign12890_e12194: f64 = (assign12890_e12192 * locals.var_q_k1q1);
        let assign12890_e12196: f64 = (assign12890_e12194 + locals.var_q_aexp);
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9, ) = (assign12890_e12196, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12890_e12192 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9), );

        let assign12900_e12199: f64 = (2.0 * locals.var_k1);
        let assign12900_e12201: f64 = (assign12900_e12199 * locals.var_k1);
        let assign12900_e12203: f64 = (assign12900_e12201 - locals.var_q_aexp);
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9, ) = (assign12900_e12203, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign12900_e12199 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9), );

        let assign12910_e12206: f64 = (-0.005);
        let assign12910_e12207: f64 = if locals.var_q_qsq < assign12910_e12206 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign12910_e12207;

        if (locals.var_guard566 != 0.0) {
            let assign12920_e12210: f64 = (locals.var_q_qsq).abs();
            let assign12920_e12211: f64 = (assign12920_e12210).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign12920_e12211, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12920_e12211)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12920_e12211)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12920_e12211)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12920_e12211)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12920_e12211)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12930_e12218: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign12930_e12219: f64 = (assign12930_e12218).tan();
            let assign12930_e12220: f64 = (locals.var_q_rac_qsq / assign12930_e12219);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12930_e12220, (((locals.var_q_rac_qsq_dn4 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), (((locals.var_q_rac_qsq_dn6 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), (((locals.var_q_rac_qsq_dn7 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), (((locals.var_q_rac_qsq_dn8 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), (((locals.var_q_rac_qsq_dn9 * assign12930_e12219) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign12930_e12218).cos() * (assign12930_e12218).cos())))) / (assign12930_e12219 * assign12930_e12219)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12940_e12226: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign12940_e12228: f64 = (assign12940_e12226 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12940_e12228, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign12940_e12226 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12950_e12236: f64 = (2.0 - locals.var_q_qcoth);
            let assign12950_e12237: f64 = (locals.var_q_qcoth * assign12950_e12236);
            let assign12950_e12238: f64 = (locals.var_q_qsq + assign12950_e12237);
            let assign12950_e12240: f64 = (assign12950_e12238 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12950_e12240, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12950_e12236) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign12950_e12238 * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12960_e12247: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign12960_e12250: f64 = (1.0 + locals.var_q_qcoth);
            let assign12960_e12251: f64 = (assign12960_e12247 * assign12960_e12250);
            let assign12960_e12252: f64 = (locals.var_q_d1_qsq - assign12960_e12251);
            let assign12960_e12254: f64 = (assign12960_e12252 * locals.var_q_temp1);
            let assign12960_e12257: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign12960_e12259: f64 = (assign12960_e12257 / locals.var_q_d1_qsq);
            let assign12960_e12260: f64 = (assign12960_e12254 + assign12960_e12259);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign12960_e12260, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign12960_e12250) + (assign12960_e12247 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign12960_e12252 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign12960_e12257 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12970_e12267: f64 = (0.5 * locals.var_q_qcoth);
            let assign12970_e12268: f64 = (1.0 - assign12970_e12267);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign12970_e12268, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12980_e12274: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign12980_e12276: f64 = (assign12980_e12274 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign12980_e12276, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12274 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12990_e12282: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign12990_e12287: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign12990_e12288: f64 = (locals.var_q_d1_ln + assign12990_e12287);
            let assign12990_e12289: f64 = (locals.var_q_d1_qsq * assign12990_e12288);
            let assign12990_e12290: f64 = (assign12990_e12282 - assign12990_e12289);
            let assign12990_e12292: f64 = (assign12990_e12290 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign12990_e12292, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign12990_e12288) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign12990_e12290 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        let assign13000_e12297: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard567 = assign13000_e12297;

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13010_e12303: f64 = (locals.var_q_qsq).abs();
            let assign13010_e12304: f64 = (assign13010_e12303).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign13010_e12304, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13010_e12304)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13010_e12304)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13010_e12304)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13010_e12304)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13010_e12304)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13020_e12312: f64 = (-locals.var_q_rac_qsq);
            let assign13020_e12313: f64 = (assign13020_e12312).exp();
            (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (assign13020_e12313, (assign13020_e12313 * (-locals.var_q_rac_qsq_dn4)), (assign13020_e12313 * (-locals.var_q_rac_qsq_dn6)), (assign13020_e12313 * (-locals.var_q_rac_qsq_dn7)), (assign13020_e12313 * (-locals.var_q_rac_qsq_dn8)), (assign13020_e12313 * (-locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13030_e12323: f64 = (1.0 + locals.var_q_invexpq);
            let assign13030_e12324: f64 = (locals.var_q_rac_qsq * assign13030_e12323);
            let assign13030_e12327: f64 = (1.0 - locals.var_q_invexpq);
            let assign13030_e12328: f64 = (assign13030_e12324 / assign13030_e12327);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13030_e12328, (((((locals.var_q_rac_qsq_dn4 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn4))) / (assign13030_e12327 * assign13030_e12327)), (((((locals.var_q_rac_qsq_dn6 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn6))) / (assign13030_e12327 * assign13030_e12327)), (((((locals.var_q_rac_qsq_dn7 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn7))) / (assign13030_e12327 * assign13030_e12327)), (((((locals.var_q_rac_qsq_dn8 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn8))) / (assign13030_e12327 * assign13030_e12327)), (((((locals.var_q_rac_qsq_dn9 * assign13030_e12323) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign13030_e12327) - (assign13030_e12324 * (-locals.var_q_invexpq_dn9))) / (assign13030_e12327 * assign13030_e12327)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13040_e12337: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign13040_e12339: f64 = (assign13040_e12337 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13040_e12339, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13040_e12337 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13050_e12350: f64 = (2.0 - locals.var_q_qcoth);
            let assign13050_e12351: f64 = (locals.var_q_qcoth * assign13050_e12350);
            let assign13050_e12352: f64 = (locals.var_q_qsq + assign13050_e12351);
            let assign13050_e12354: f64 = (assign13050_e12352 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13050_e12354, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13050_e12350) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13050_e12352 * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13060_e12364: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign13060_e12367: f64 = (1.0 + locals.var_q_qcoth);
            let assign13060_e12368: f64 = (assign13060_e12364 * assign13060_e12367);
            let assign13060_e12369: f64 = (locals.var_q_d1_qsq - assign13060_e12368);
            let assign13060_e12371: f64 = (assign13060_e12369 * locals.var_q_temp1);
            let assign13060_e12374: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign13060_e12376: f64 = (assign13060_e12374 / locals.var_q_d1_qsq);
            let assign13060_e12377: f64 = (assign13060_e12371 + assign13060_e12376);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13060_e12377, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13060_e12367) + (assign13060_e12364 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13060_e12369 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13060_e12374 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13070_e12387: f64 = (0.5 * locals.var_q_qcoth);
            let assign13070_e12388: f64 = (1.0 - assign13070_e12387);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13070_e12388, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13080_e12397: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign13080_e12399: f64 = (assign13080_e12397 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13080_e12399, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13080_e12397 * locals.var_q_temp2_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign13090_e12408: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign13090_e12413: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign13090_e12414: f64 = (locals.var_q_d1_ln + assign13090_e12413);
            let assign13090_e12415: f64 = (locals.var_q_d1_qsq * assign13090_e12414);
            let assign13090_e12416: f64 = (assign13090_e12408 - assign13090_e12415);
            let assign13090_e12418: f64 = (assign13090_e12416 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13090_e12418, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13090_e12414) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13090_e12416 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13100_e12430: f64 = (locals.var_q_qsq * 0.0166666666667);
            let assign13100_e12434: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign13100_e12438: f64 = (locals.var_q_qsq * 0.025);
            let assign13100_e12439: f64 = (1.0 - assign13100_e12438);
            let assign13100_e12440: f64 = (assign13100_e12434 * assign13100_e12439);
            let assign13100_e12441: f64 = (1.0 - assign13100_e12440);
            let assign13100_e12442: f64 = (assign13100_e12430 * assign13100_e12441);
            let assign13100_e12443: f64 = (1.0 - assign13100_e12442);
            let assign13100_e12444: f64 = (0.1666666666667 * assign13100_e12443);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign13100_e12444, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign13100_e12441) + (assign13100_e12430 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13100_e12439) + (assign13100_e12434 * (-(locals.var_q_qsq_dn9 * 0.025))))))))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13110_e12455: f64 = (locals.var_q_qsq * locals.var_q_temp3);
            let assign13110_e12456: f64 = (2.0 + assign13110_e12455);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13110_e12456, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13120_e12468: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign13120_e12472: f64 = (locals.var_q_qsq * 0.0357142857143);
            let assign13120_e12476: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign13120_e12477: f64 = (1.0 - assign13120_e12476);
            let assign13120_e12478: f64 = (assign13120_e12472 * assign13120_e12477);
            let assign13120_e12479: f64 = (1.0 - assign13120_e12478);
            let assign13120_e12480: f64 = (assign13120_e12468 * assign13120_e12479);
            let assign13120_e12481: f64 = (1.0 - assign13120_e12480);
            let assign13120_e12482: f64 = (0.1666666666667 * assign13120_e12481);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13120_e12482, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign13120_e12479) + (assign13120_e12468 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign13120_e12477) + (assign13120_e12472 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13130_e12492: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13130_e12492, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13140_e12504: f64 = (locals.var_q_qsq * 0.0714285714286);
            let assign13140_e12508: f64 = (0.05 * locals.var_q_qsq);
            let assign13140_e12512: f64 = (0.0420875420875421 * locals.var_q_qsq);
            let assign13140_e12513: f64 = (1.0 - assign13140_e12512);
            let assign13140_e12514: f64 = (assign13140_e12508 * assign13140_e12513);
            let assign13140_e12515: f64 = (1.0 - assign13140_e12514);
            let assign13140_e12516: f64 = (assign13140_e12504 * assign13140_e12515);
            let assign13140_e12517: f64 = (1.0 - assign13140_e12516);
            let assign13140_e12518: f64 = (0.0055555555556 * assign13140_e12517);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13140_e12518, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign13140_e12515) + (assign13140_e12504 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13140_e12513) + (assign13140_e12508 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13150_e12528: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
            let assign13150_e12531: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
            let assign13150_e12533: f64 = (assign13150_e12531 * locals.var_q_temp2);
            let assign13150_e12534: f64 = (assign13150_e12528 - assign13150_e12533);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13150_e12534, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign13150_e12531 * locals.var_q_temp2_dn9))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13160_e12543: f64 = (-0.5);
            let assign13160_e12545: f64 = (assign13160_e12543 * locals.var_q_d1_qsq);
            let assign13160_e12547: f64 = (assign13160_e12545 * locals.var_q_temp3);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13160_e12547, (((assign13160_e12543 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn4)), (((assign13160_e12543 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn6)), (((assign13160_e12543 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn7)), (((assign13160_e12543 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn8)), (((assign13160_e12543 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign13160_e12545 * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13170_e12556: f64 = (-0.5);
            let assign13170_e12558: f64 = (assign13170_e12556 * locals.var_q_d2_qsq);
            let assign13170_e12560: f64 = (assign13170_e12558 * locals.var_q_temp3);
            let assign13170_e12563: f64 = (0.25 * 0.0055555555556);
            let assign13170_e12565: f64 = (assign13170_e12563 * locals.var_q_d1_qsq);
            let assign13170_e12567: f64 = (assign13170_e12565 * locals.var_q_d1_qsq);
            let assign13170_e12571: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign13170_e12575: f64 = (0.075 * locals.var_q_qsq);
            let assign13170_e12576: f64 = (2.0 - assign13170_e12575);
            let assign13170_e12577: f64 = (assign13170_e12571 * assign13170_e12576);
            let assign13170_e12578: f64 = (1.0 - assign13170_e12577);
            let assign13170_e12579: f64 = (assign13170_e12567 * assign13170_e12578);
            let assign13170_e12580: f64 = (assign13170_e12560 + assign13170_e12579);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13170_e12580, ((((assign13170_e12556 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn4)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn4)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign13170_e12556 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn6)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn6)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign13170_e12556 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn7)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn7)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign13170_e12556 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn8)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn8)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign13170_e12556 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign13170_e12558 * locals.var_q_temp3_dn9)) + (((((assign13170_e12563 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign13170_e12565 * locals.var_q_d1_qsq_dn9)) * assign13170_e12578) + (assign13170_e12567 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13170_e12576) + (assign13170_e12571 * (-(0.075 * locals.var_q_qsq_dn9)))))))), );
        }

        let assign13180_e12585: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign13180_e12585;

        if (locals.var_guard568 != 0.0) {
            let assign13190_e12589: f64 = (4.0 * locals.var_q_qsq);
            let assign13190_e12594: f64 = (2.0 - locals.var_q_invexpq);
            let assign13190_e12595: f64 = (locals.var_q_invexpq * assign13190_e12594);
            let assign13190_e12596: f64 = (1.0 - assign13190_e12595);
            let assign13190_e12597: f64 = (assign13190_e12589 / assign13190_e12596);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13190_e12597, ((((4.0 * locals.var_q_qsq_dn4) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn4 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign13190_e12596 * assign13190_e12596)), ((((4.0 * locals.var_q_qsq_dn6) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn6 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign13190_e12596 * assign13190_e12596)), ((((4.0 * locals.var_q_qsq_dn7) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn7 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign13190_e12596 * assign13190_e12596)), ((((4.0 * locals.var_q_qsq_dn8) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn8 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign13190_e12596 * assign13190_e12596)), ((((4.0 * locals.var_q_qsq_dn9) * assign13190_e12596) - (assign13190_e12589 * (-((locals.var_q_invexpq_dn9 * assign13190_e12594) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign13190_e12596 * assign13190_e12596)), );
        }

        if (locals.var_guard568 != 0.0) {
            let assign13200_e12603: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13200_e12603, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)), );
        }

        if (locals.var_guard568 != 0.0) {
            let assign13210_e12608: f64 = (locals.var_q_temp2).ln();
            let assign13210_e12610: f64 = (assign13210_e12608 - locals.var_q_rac_qsq);
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13210_e12610, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9), );
        }

        let assign13220_e12615: f64 = (-0.005);
        let assign13220_e12616: f64 = if locals.var_q_qsq < assign13220_e12615 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign13220_e12616;

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
            let assign13230_e12623: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign13230_e12624: f64 = (assign13230_e12623).sin();
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13230_e12624, ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign13230_e12623).cos() * (0.5 * locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
            let assign13240_e12632: f64 = (-locals.var_q_qsq);
            let assign13240_e12635: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
            let assign13240_e12636: f64 = (assign13240_e12632 / assign13240_e12635);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13240_e12636, ((((-locals.var_q_qsq_dn4) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign13240_e12635 * assign13240_e12635)), ((((-locals.var_q_qsq_dn6) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign13240_e12635 * assign13240_e12635)), ((((-locals.var_q_qsq_dn7) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign13240_e12635 * assign13240_e12635)), ((((-locals.var_q_qsq_dn8) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign13240_e12635 * assign13240_e12635)), ((((-locals.var_q_qsq_dn9) * assign13240_e12635) - (assign13240_e12632 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign13240_e12635 * assign13240_e12635)), );
        }

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
            let assign13250_e12644: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13250_e12644, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 == 0.0)) {
            let assign13260_e12655: f64 = (locals.var_q_qsq * 0.3333333333333);
            let assign13260_e12659: f64 = (0.05 * locals.var_q_qsq);
            let assign13260_e12663: f64 = (0.0396825396825397 * locals.var_q_qsq);
            let assign13260_e12664: f64 = (1.0 - assign13260_e12663);
            let assign13260_e12665: f64 = (assign13260_e12659 * assign13260_e12664);
            let assign13260_e12666: f64 = (1.0 - assign13260_e12665);
            let assign13260_e12667: f64 = (assign13260_e12655 * assign13260_e12666);
            let assign13260_e12668: f64 = (4.0 - assign13260_e12667);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13260_e12668, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign13260_e12666) + (assign13260_e12655 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13260_e12664) + (assign13260_e12659 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))), );
        }

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 == 0.0)) {
            let assign13270_e12677: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13270_e12677, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        let assign13280_e12682: f64 = (1.01 * locals.var_q_k1q1);
        let assign13280_e12684: f64 = (assign13280_e12682 + locals.var_q_qcoth);
        let assign13280_e12686: f64 = if assign13280_e12684 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign13280_e12686;

        if (locals.var_guard570 != 0.0) {
            let assign13290_e12690: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign13290_e12690, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9), );
        }

        if (locals.var_guard570 != 0.0) {
            let assign13300_e12696: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign13300_e12696, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9), );
        }

        if (locals.var_guard570 != 0.0) {
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13320_e12708: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
            let assign13320_e12709: f64 = (1.0 / assign13320_e12708);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13320_e12709, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign13320_e12708 * assign13320_e12708))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign13320_e12708 * assign13320_e12708))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign13320_e12708 * assign13320_e12708))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign13320_e12708 * assign13320_e12708))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign13320_e12708 * assign13320_e12708))), );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13330_e12716: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign13330_e12716, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9), );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13340_e12723: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
            let assign13340_e12725: f64 = (assign13340_e12723 * locals.var_q_temp2);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign13340_e12725, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign13340_e12723 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13350_e12732: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
            let assign13350_e12734: f64 = (assign13350_e12732 - locals.var_q_aexp);
            let assign13350_e12737: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
            let assign13350_e12738: f64 = (assign13350_e12734 - assign13350_e12737);
            let assign13350_e12740: f64 = (assign13350_e12738 * locals.var_q_temp2);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign13350_e12740, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign13350_e12738 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13360_e12747: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
            let assign13360_e12750: f64 = (2.0 * locals.var_q_temp3);
            let assign13360_e12752: f64 = (assign13360_e12750 * locals.var_q_d1_expnum);
            let assign13360_e12753: f64 = (assign13360_e12747 + assign13360_e12752);
            let assign13360_e12755: f64 = (assign13360_e12753 + locals.var_q_aexp);
            let assign13360_e12759: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
            let assign13360_e12760: f64 = (locals.var_q_d2_ln + assign13360_e12759);
            let assign13360_e12762: f64 = (assign13360_e12760 * locals.var_q_sh_term);
            let assign13360_e12763: f64 = (assign13360_e12755 - assign13360_e12762);
            let assign13360_e12765: f64 = (assign13360_e12763 * locals.var_q_temp2);
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (assign13360_e12765, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign13360_e12750 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign13360_e12760 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign13360_e12763 * locals.var_q_temp2_dn9)), );
        }

        let assign13370_e12770: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign13370_e12770;

        if (locals.var_guard571 != 0.0) {
            let assign13380_e12773: f64 = (locals.var_q_expnum).ln();
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign13380_e12773, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum), );
        }

        if (locals.var_guard571 != 0.0) {
            let assign13390_e12779: f64 = (1.0 / locals.var_q_expnum);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13390_e12779, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))), );
        }

        if (locals.var_guard571 != 0.0) {
            let assign13400_e12785: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign13400_e12785, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard571 != 0.0) {
            let assign13410_e12791: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
            let assign13410_e12794: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
            let assign13410_e12795: f64 = (assign13410_e12791 - assign13410_e12794);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign13410_e12795, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))), );
        }

        if (locals.var_guard571 == 0.0) {
            let assign13420_e12802: f64 = (locals.var_q_k1q1 + 0.6931471805599);
            let assign13420_e12804: f64 = (-locals.var_q_k1q1);
            let assign13420_e12805: f64 = (assign13420_e12804).ln();
            let assign13420_e12806: f64 = (assign13420_e12802 + assign13420_e12805);
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign13420_e12806, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign13420_e12804)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign13420_e12804)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign13420_e12804)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign13420_e12804)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign13420_e12804)), );
        }

        if (locals.var_guard571 == 0.0) {
            let assign13430_e12813: f64 = (1.0 / locals.var_q1s);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13430_e12813, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))), );
        }

        if (locals.var_guard571 == 0.0) {
            let assign13440_e12820: f64 = (locals.var_k1 + locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign13440_e12820, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9), );
        }

        if (locals.var_guard571 == 0.0) {
            let assign13450_e12826: f64 = (-locals.var_q_temp1);
            let assign13450_e12828: f64 = (assign13450_e12826 * locals.var_q_temp1);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign13450_e12828, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign13450_e12826 * locals.var_q_temp1_dn9)), );
        }

        let assign13460_e12833: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign13460_e12835: f64 = (assign13460_e12833 + locals.var_q1s);
        let assign13460_e12838: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign13460_e12839: f64 = (assign13460_e12835 + assign13460_e12838);
        let assign13460_e12841: f64 = (assign13460_e12839 - locals.var_q_ln_term);
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9, ) = (assign13460_e12841, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9), );

        let assign13470_e12845: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign13470_e12846: f64 = (1.0 + assign13470_e12845);
        let assign13470_e12848: f64 = (assign13470_e12846 - locals.var_q_d1_ln);
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9, ) = (assign13470_e12848, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9), );

        let assign13480_e12851: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign13480_e12853: f64 = (assign13480_e12851 - locals.var_q_d2_ln);
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9, ) = (assign13480_e12853, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9), );

        let assign13490_e12857: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign13490_e12858: f64 = (locals.var_q_k1q1 + assign13490_e12857);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign13490_e12858, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))), );

        let assign13500_e12862: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign13500_e12863: f64 = (locals.var_k1 + assign13500_e12862);
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9, ) = (assign13500_e12863, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))), );

        let assign13510_e12866: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9, ) = (assign13510_e12866, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)), );

        let assign13520_e12869: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign13520_e12871: f64 = (assign13520_e12869 - locals.var_q_aexp);
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign13520_e12871, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9), );

    }

    pub(super) fn stamp_transient_block_15(
        locals: &mut StampLocals,
    ) {
        let assign13530_e12874: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign13530_e12877: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign13530_e12878: f64 = (assign13530_e12874 + assign13530_e12877);
        let assign13530_e12880: f64 = (assign13530_e12878 + locals.var_q_aexp);
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign13530_e12880, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9), );

        let assign13540_e12883: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign13540_e12886: f64 = (2.0 * locals.var_q_d1_qi);
        let assign13540_e12888: f64 = (assign13540_e12886 * locals.var_q_d1_expnum);
        let assign13540_e12889: f64 = (assign13540_e12883 + assign13540_e12888);
        let assign13540_e12892: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign13540_e12893: f64 = (assign13540_e12889 + assign13540_e12892);
        let assign13540_e12895: f64 = (assign13540_e12893 - locals.var_q_aexp);
        (locals.var_q_d2_zero, locals.var_q_d2_zero_dn4, locals.var_q_d2_zero_dn6, locals.var_q_d2_zero_dn7, locals.var_q_d2_zero_dn8, locals.var_q_d2_zero_dn9, ) = (assign13540_e12895, (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4), (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6), (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7), (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8), (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign13540_e12886 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9), );

        let assign13550_e12898: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign13550_e12901: f64 = (0.5 * locals.var_q_zero);
        let assign13550_e12903: f64 = (assign13550_e12901 * locals.var_q_d2_zero);
        let assign13550_e12904: f64 = (assign13550_e12898 - assign13550_e12903);
        (locals.var_q_temp, locals.var_q_temp_dn4, locals.var_q_temp_dn6, locals.var_q_temp_dn7, locals.var_q_temp_dn8, locals.var_q_temp_dn9, ) = (assign13550_e12904, (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn4))), (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn6))), (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn7))), (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn8))), (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign13550_e12901 * locals.var_q_d2_zero_dn9))), );

        let assign13560_e12906: f64 = (-locals.var_q_zero);
        let assign13560_e12908: f64 = (assign13560_e12906 * locals.var_q_d1_zero);
        let assign13560_e12910: f64 = (assign13560_e12908 * locals.var_q_temp);
        let assign13560_e12913: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign13560_e12915: f64 = (assign13560_e12913 + 1e-200);
        let assign13560_e12916: f64 = (assign13560_e12910 / assign13560_e12915);
        (locals.var_q_eps2, locals.var_q_eps2_dn4, locals.var_q_eps2_dn6, locals.var_q_eps2_dn7, locals.var_q_eps2_dn8, locals.var_q_eps2_dn9, ) = (assign13560_e12916, ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn4)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign13560_e12915 * assign13560_e12915)), ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn6)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign13560_e12915 * assign13560_e12915)), ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn7)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign13560_e12915 * assign13560_e12915)), ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn8)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign13560_e12915 * assign13560_e12915)), ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign13560_e12906 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign13560_e12908 * locals.var_q_temp_dn9)) * assign13560_e12915) - (assign13560_e12910 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign13560_e12915 * assign13560_e12915)), );

        let assign13570_e12919: f64 = (locals.var_q1s + locals.var_q_eps2);
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign13570_e12919, (locals.var_q1s_dn4 + locals.var_q_eps2_dn4), (locals.var_q1s_dn6 + locals.var_q_eps2_dn6), (locals.var_q1s_dn7 + locals.var_q_eps2_dn7), (locals.var_q1s_dn8 + locals.var_q_eps2_dn8), (locals.var_q1s_dn9 + locals.var_q_eps2_dn9), );

        let assign13580_e12922: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign13580_e12922, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign13590_e12925: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13590_e12927: f64 = assign13590_e12925;
        let assign13590_e12929: f64 = if assign13590_e12927 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign13590_e12929;

        if (locals.var_guard572 != 0.0) {
            let assign13600_e12933: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign13600_e12935: f64 = assign13600_e12933;
            let assign13600_e12936: f64 = (assign13600_e12935).exp();
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13600_e12936, (assign13600_e12936 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign13600_e12936 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign13600_e12936 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign13600_e12936 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign13600_e12936 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)), );
        }

        if (locals.var_guard572 == 0.0) {
            let assign13610_e12945: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign13610_e12947: f64 = assign13610_e12945;
            let assign13610_e12949: f64 = (assign13610_e12947 - 80.0);
            let assign13610_e12954: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign13610_e12956: f64 = assign13610_e12954;
            let assign13610_e12958: f64 = (assign13610_e12956 - 80.0);
            let assign13610_e12959: f64 = (0.5 * assign13610_e12958);
            let assign13610_e12963: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign13610_e12965: f64 = assign13610_e12963;
            let assign13610_e12967: f64 = (assign13610_e12965 - 80.0);
            let assign13610_e12969: f64 = (assign13610_e12967 * 0.3333333333333);
            let assign13610_e12970: f64 = (1.0 + assign13610_e12969);
            let assign13610_e12971: f64 = (assign13610_e12959 * assign13610_e12970);
            let assign13610_e12972: f64 = (1.0 + assign13610_e12971);
            let assign13610_e12973: f64 = (assign13610_e12949 * assign13610_e12972);
            let assign13610_e12974: f64 = (1.0 + assign13610_e12973);
            let assign13610_e12975: f64 = (5.54062e34 * assign13610_e12974);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13610_e12975, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign13610_e12972) + (assign13610_e12949 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign13610_e12970) + (assign13610_e12959 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))), );
        }

        let assign13620_e12980: f64 = (locals.var_a0 * locals.var_q_temp1);
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9, ) = (assign13620_e12980, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)), );

        let assign13630_e12983: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign13630_e12985: f64 = (assign13630_e12983 - locals.var_q_aexp);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign13630_e12985, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9), );

        let assign13640_e12988: f64 = (2.0 * locals.var_k1);
        let assign13640_e12990: f64 = (assign13640_e12988 * locals.var_q_k1q1);
        let assign13640_e12992: f64 = (assign13640_e12990 + locals.var_q_aexp);
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9, ) = (assign13640_e12992, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign13640_e12988 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9), );

        let assign13650_e12995: f64 = (2.0 * locals.var_k1);
        let assign13650_e12997: f64 = (assign13650_e12995 * locals.var_k1);
        let assign13650_e12999: f64 = (assign13650_e12997 - locals.var_q_aexp);
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9, ) = (assign13650_e12999, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign13650_e12995 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9), );

        let assign13660_e13002: f64 = (-0.005);
        let assign13660_e13003: f64 = if locals.var_q_qsq < assign13660_e13002 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign13660_e13003;

        if (locals.var_guard573 != 0.0) {
            let assign13670_e13006: f64 = (locals.var_q_qsq).abs();
            let assign13670_e13007: f64 = (assign13670_e13006).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign13670_e13007, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13670_e13007)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13670_e13007)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13670_e13007)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13670_e13007)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13670_e13007)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13680_e13014: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign13680_e13015: f64 = (assign13680_e13014).tan();
            let assign13680_e13016: f64 = (locals.var_q_rac_qsq / assign13680_e13015);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13680_e13016, (((locals.var_q_rac_qsq_dn4 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), (((locals.var_q_rac_qsq_dn6 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), (((locals.var_q_rac_qsq_dn7 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), (((locals.var_q_rac_qsq_dn8 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), (((locals.var_q_rac_qsq_dn9 * assign13680_e13015) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign13680_e13014).cos() * (assign13680_e13014).cos())))) / (assign13680_e13015 * assign13680_e13015)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13690_e13022: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign13690_e13024: f64 = (assign13690_e13022 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13690_e13024, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13690_e13022 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13700_e13032: f64 = (2.0 - locals.var_q_qcoth);
            let assign13700_e13033: f64 = (locals.var_q_qcoth * assign13700_e13032);
            let assign13700_e13034: f64 = (locals.var_q_qsq + assign13700_e13033);
            let assign13700_e13036: f64 = (assign13700_e13034 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13700_e13036, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13700_e13032) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13700_e13034 * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13710_e13043: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign13710_e13046: f64 = (1.0 + locals.var_q_qcoth);
            let assign13710_e13047: f64 = (assign13710_e13043 * assign13710_e13046);
            let assign13710_e13048: f64 = (locals.var_q_d1_qsq - assign13710_e13047);
            let assign13710_e13050: f64 = (assign13710_e13048 * locals.var_q_temp1);
            let assign13710_e13053: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign13710_e13055: f64 = (assign13710_e13053 / locals.var_q_d1_qsq);
            let assign13710_e13056: f64 = (assign13710_e13050 + assign13710_e13055);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13710_e13056, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13710_e13046) + (assign13710_e13043 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13710_e13048 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13710_e13053 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13720_e13063: f64 = (0.5 * locals.var_q_qcoth);
            let assign13720_e13064: f64 = (1.0 - assign13720_e13063);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13720_e13064, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13730_e13070: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign13730_e13072: f64 = (assign13730_e13070 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13730_e13072, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13070 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13740_e13078: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign13740_e13083: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign13740_e13084: f64 = (locals.var_q_d1_ln + assign13740_e13083);
            let assign13740_e13085: f64 = (locals.var_q_d1_qsq * assign13740_e13084);
            let assign13740_e13086: f64 = (assign13740_e13078 - assign13740_e13085);
            let assign13740_e13088: f64 = (assign13740_e13086 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13740_e13088, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13740_e13084) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13740_e13086 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        let assign13750_e13093: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign13750_e13093;

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13760_e13099: f64 = (locals.var_q_qsq).abs();
            let assign13760_e13100: f64 = (assign13760_e13099).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign13760_e13100, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13760_e13100)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13760_e13100)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13760_e13100)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13760_e13100)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13760_e13100)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13770_e13108: f64 = (-locals.var_q_rac_qsq);
            let assign13770_e13109: f64 = (assign13770_e13108).exp();
            (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (assign13770_e13109, (assign13770_e13109 * (-locals.var_q_rac_qsq_dn4)), (assign13770_e13109 * (-locals.var_q_rac_qsq_dn6)), (assign13770_e13109 * (-locals.var_q_rac_qsq_dn7)), (assign13770_e13109 * (-locals.var_q_rac_qsq_dn8)), (assign13770_e13109 * (-locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13780_e13119: f64 = (1.0 + locals.var_q_invexpq);
            let assign13780_e13120: f64 = (locals.var_q_rac_qsq * assign13780_e13119);
            let assign13780_e13123: f64 = (1.0 - locals.var_q_invexpq);
            let assign13780_e13124: f64 = (assign13780_e13120 / assign13780_e13123);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13780_e13124, (((((locals.var_q_rac_qsq_dn4 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn4))) / (assign13780_e13123 * assign13780_e13123)), (((((locals.var_q_rac_qsq_dn6 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn6))) / (assign13780_e13123 * assign13780_e13123)), (((((locals.var_q_rac_qsq_dn7 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn7))) / (assign13780_e13123 * assign13780_e13123)), (((((locals.var_q_rac_qsq_dn8 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn8))) / (assign13780_e13123 * assign13780_e13123)), (((((locals.var_q_rac_qsq_dn9 * assign13780_e13119) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign13780_e13123) - (assign13780_e13120 * (-locals.var_q_invexpq_dn9))) / (assign13780_e13123 * assign13780_e13123)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13790_e13133: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign13790_e13135: f64 = (assign13790_e13133 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13790_e13135, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13790_e13133 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13800_e13146: f64 = (2.0 - locals.var_q_qcoth);
            let assign13800_e13147: f64 = (locals.var_q_qcoth * assign13800_e13146);
            let assign13800_e13148: f64 = (locals.var_q_qsq + assign13800_e13147);
            let assign13800_e13150: f64 = (assign13800_e13148 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13800_e13150, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13800_e13146) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13800_e13148 * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13810_e13160: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign13810_e13163: f64 = (1.0 + locals.var_q_qcoth);
            let assign13810_e13164: f64 = (assign13810_e13160 * assign13810_e13163);
            let assign13810_e13165: f64 = (locals.var_q_d1_qsq - assign13810_e13164);
            let assign13810_e13167: f64 = (assign13810_e13165 * locals.var_q_temp1);
            let assign13810_e13170: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign13810_e13172: f64 = (assign13810_e13170 / locals.var_q_d1_qsq);
            let assign13810_e13173: f64 = (assign13810_e13167 + assign13810_e13172);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13810_e13173, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13810_e13163) + (assign13810_e13160 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13810_e13165 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13810_e13170 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13820_e13183: f64 = (0.5 * locals.var_q_qcoth);
            let assign13820_e13184: f64 = (1.0 - assign13820_e13183);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13820_e13184, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13830_e13193: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign13830_e13195: f64 = (assign13830_e13193 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13830_e13195, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13830_e13193 * locals.var_q_temp2_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13840_e13204: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign13840_e13209: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign13840_e13210: f64 = (locals.var_q_d1_ln + assign13840_e13209);
            let assign13840_e13211: f64 = (locals.var_q_d1_qsq * assign13840_e13210);
            let assign13840_e13212: f64 = (assign13840_e13204 - assign13840_e13211);
            let assign13840_e13214: f64 = (assign13840_e13212 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13840_e13214, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13840_e13210) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13840_e13212 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13850_e13226: f64 = (locals.var_q_qsq * 0.0166666666667);
            let assign13850_e13230: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign13850_e13234: f64 = (locals.var_q_qsq * 0.025);
            let assign13850_e13235: f64 = (1.0 - assign13850_e13234);
            let assign13850_e13236: f64 = (assign13850_e13230 * assign13850_e13235);
            let assign13850_e13237: f64 = (1.0 - assign13850_e13236);
            let assign13850_e13238: f64 = (assign13850_e13226 * assign13850_e13237);
            let assign13850_e13239: f64 = (1.0 - assign13850_e13238);
            let assign13850_e13240: f64 = (0.1666666666667 * assign13850_e13239);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign13850_e13240, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign13850_e13237) + (assign13850_e13226 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13850_e13235) + (assign13850_e13230 * (-(locals.var_q_qsq_dn9 * 0.025))))))))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13860_e13251: f64 = (locals.var_q_qsq * locals.var_q_temp3);
            let assign13860_e13252: f64 = (2.0 + assign13860_e13251);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13860_e13252, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13870_e13264: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign13870_e13268: f64 = (locals.var_q_qsq * 0.0357142857143);
            let assign13870_e13272: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign13870_e13273: f64 = (1.0 - assign13870_e13272);
            let assign13870_e13274: f64 = (assign13870_e13268 * assign13870_e13273);
            let assign13870_e13275: f64 = (1.0 - assign13870_e13274);
            let assign13870_e13276: f64 = (assign13870_e13264 * assign13870_e13275);
            let assign13870_e13277: f64 = (1.0 - assign13870_e13276);
            let assign13870_e13278: f64 = (0.1666666666667 * assign13870_e13277);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13870_e13278, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign13870_e13275) + (assign13870_e13264 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign13870_e13273) + (assign13870_e13268 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13880_e13288: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13880_e13288, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13890_e13300: f64 = (locals.var_q_qsq * 0.0714285714286);
            let assign13890_e13304: f64 = (0.05 * locals.var_q_qsq);
            let assign13890_e13308: f64 = (0.0420875420875421 * locals.var_q_qsq);
            let assign13890_e13309: f64 = (1.0 - assign13890_e13308);
            let assign13890_e13310: f64 = (assign13890_e13304 * assign13890_e13309);
            let assign13890_e13311: f64 = (1.0 - assign13890_e13310);
            let assign13890_e13312: f64 = (assign13890_e13300 * assign13890_e13311);
            let assign13890_e13313: f64 = (1.0 - assign13890_e13312);
            let assign13890_e13314: f64 = (0.0055555555556 * assign13890_e13313);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13890_e13314, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign13890_e13311) + (assign13890_e13300 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13890_e13309) + (assign13890_e13304 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13900_e13324: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
            let assign13900_e13327: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
            let assign13900_e13329: f64 = (assign13900_e13327 * locals.var_q_temp2);
            let assign13900_e13330: f64 = (assign13900_e13324 - assign13900_e13329);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13900_e13330, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign13900_e13327 * locals.var_q_temp2_dn9))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13910_e13339: f64 = (-0.5);
            let assign13910_e13341: f64 = (assign13910_e13339 * locals.var_q_d1_qsq);
            let assign13910_e13343: f64 = (assign13910_e13341 * locals.var_q_temp3);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13910_e13343, (((assign13910_e13339 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn4)), (((assign13910_e13339 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn6)), (((assign13910_e13339 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn7)), (((assign13910_e13339 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn8)), (((assign13910_e13339 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign13910_e13341 * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13920_e13352: f64 = (-0.5);
            let assign13920_e13354: f64 = (assign13920_e13352 * locals.var_q_d2_qsq);
            let assign13920_e13356: f64 = (assign13920_e13354 * locals.var_q_temp3);
            let assign13920_e13359: f64 = (0.25 * 0.0055555555556);
            let assign13920_e13361: f64 = (assign13920_e13359 * locals.var_q_d1_qsq);
            let assign13920_e13363: f64 = (assign13920_e13361 * locals.var_q_d1_qsq);
            let assign13920_e13367: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign13920_e13371: f64 = (0.075 * locals.var_q_qsq);
            let assign13920_e13372: f64 = (2.0 - assign13920_e13371);
            let assign13920_e13373: f64 = (assign13920_e13367 * assign13920_e13372);
            let assign13920_e13374: f64 = (1.0 - assign13920_e13373);
            let assign13920_e13375: f64 = (assign13920_e13363 * assign13920_e13374);
            let assign13920_e13376: f64 = (assign13920_e13356 + assign13920_e13375);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13920_e13376, ((((assign13920_e13352 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn4)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn4)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign13920_e13352 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn6)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn6)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign13920_e13352 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn7)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn7)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign13920_e13352 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn8)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn8)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign13920_e13352 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign13920_e13354 * locals.var_q_temp3_dn9)) + (((((assign13920_e13359 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign13920_e13361 * locals.var_q_d1_qsq_dn9)) * assign13920_e13374) + (assign13920_e13363 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13920_e13372) + (assign13920_e13367 * (-(0.075 * locals.var_q_qsq_dn9)))))))), );
        }

        let assign13930_e13381: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign13930_e13381;

        if (locals.var_guard575 != 0.0) {
            let assign13940_e13385: f64 = (4.0 * locals.var_q_qsq);
            let assign13940_e13390: f64 = (2.0 - locals.var_q_invexpq);
            let assign13940_e13391: f64 = (locals.var_q_invexpq * assign13940_e13390);
            let assign13940_e13392: f64 = (1.0 - assign13940_e13391);
            let assign13940_e13393: f64 = (assign13940_e13385 / assign13940_e13392);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13940_e13393, ((((4.0 * locals.var_q_qsq_dn4) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn4 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign13940_e13392 * assign13940_e13392)), ((((4.0 * locals.var_q_qsq_dn6) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn6 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign13940_e13392 * assign13940_e13392)), ((((4.0 * locals.var_q_qsq_dn7) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn7 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign13940_e13392 * assign13940_e13392)), ((((4.0 * locals.var_q_qsq_dn8) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn8 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign13940_e13392 * assign13940_e13392)), ((((4.0 * locals.var_q_qsq_dn9) * assign13940_e13392) - (assign13940_e13385 * (-((locals.var_q_invexpq_dn9 * assign13940_e13390) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign13940_e13392 * assign13940_e13392)), );
        }

        if (locals.var_guard575 != 0.0) {
            let assign13950_e13399: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13950_e13399, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)), );
        }

        if (locals.var_guard575 != 0.0) {
            let assign13960_e13404: f64 = (locals.var_q_temp2).ln();
            let assign13960_e13406: f64 = (assign13960_e13404 - locals.var_q_rac_qsq);
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13960_e13406, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9), );
        }

        let assign13970_e13411: f64 = (-0.005);
        let assign13970_e13412: f64 = if locals.var_q_qsq < assign13970_e13411 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign13970_e13412;

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
            let assign13980_e13419: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign13980_e13420: f64 = (assign13980_e13419).sin();
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13980_e13420, ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign13980_e13419).cos() * (0.5 * locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
            let assign13990_e13428: f64 = (-locals.var_q_qsq);
            let assign13990_e13431: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
            let assign13990_e13432: f64 = (assign13990_e13428 / assign13990_e13431);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13990_e13432, ((((-locals.var_q_qsq_dn4) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign13990_e13431 * assign13990_e13431)), ((((-locals.var_q_qsq_dn6) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign13990_e13431 * assign13990_e13431)), ((((-locals.var_q_qsq_dn7) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign13990_e13431 * assign13990_e13431)), ((((-locals.var_q_qsq_dn8) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign13990_e13431 * assign13990_e13431)), ((((-locals.var_q_qsq_dn9) * assign13990_e13431) - (assign13990_e13428 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign13990_e13431 * assign13990_e13431)), );
        }

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
            let assign14000_e13440: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign14000_e13440, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 == 0.0)) {
            let assign14010_e13451: f64 = (locals.var_q_qsq * 0.3333333333333);
            let assign14010_e13455: f64 = (0.05 * locals.var_q_qsq);
            let assign14010_e13459: f64 = (0.0396825396825397 * locals.var_q_qsq);
            let assign14010_e13460: f64 = (1.0 - assign14010_e13459);
            let assign14010_e13461: f64 = (assign14010_e13455 * assign14010_e13460);
            let assign14010_e13462: f64 = (1.0 - assign14010_e13461);
            let assign14010_e13463: f64 = (assign14010_e13451 * assign14010_e13462);
            let assign14010_e13464: f64 = (4.0 - assign14010_e13463);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign14010_e13464, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn4) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn6) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn7) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn8) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign14010_e13462) + (assign14010_e13451 * (-(((0.05 * locals.var_q_qsq_dn9) * assign14010_e13460) + (assign14010_e13455 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))), );
        }

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 == 0.0)) {
            let assign14020_e13473: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign14020_e13473, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        let assign14030_e13478: f64 = (1.01 * locals.var_q_k1q1);
        let assign14030_e13480: f64 = (assign14030_e13478 + locals.var_q_qcoth);
        let assign14030_e13482: f64 = if assign14030_e13480 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign14030_e13482;

        if (locals.var_guard577 != 0.0) {
            let assign14040_e13486: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign14040_e13486, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9), );
        }

        if (locals.var_guard577 != 0.0) {
            let assign14050_e13492: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign14050_e13492, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9), );
        }

        if (locals.var_guard577 != 0.0) {
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, );
        }

        if (locals.var_guard577 == 0.0) {
            let assign14070_e13504: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
            let assign14070_e13505: f64 = (1.0 / assign14070_e13504);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign14070_e13505, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign14070_e13504 * assign14070_e13504))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign14070_e13504 * assign14070_e13504))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign14070_e13504 * assign14070_e13504))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign14070_e13504 * assign14070_e13504))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign14070_e13504 * assign14070_e13504))), );
        }

        if (locals.var_guard577 == 0.0) {
            let assign14080_e13512: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign14080_e13512, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9), );
        }

        if (locals.var_guard577 == 0.0) {
            let assign14090_e13519: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
            let assign14090_e13521: f64 = (assign14090_e13519 * locals.var_q_temp2);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign14090_e13521, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign14090_e13519 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard577 == 0.0) {
            let assign14100_e13528: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
            let assign14100_e13530: f64 = (assign14100_e13528 - locals.var_q_aexp);
            let assign14100_e13533: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
            let assign14100_e13534: f64 = (assign14100_e13530 - assign14100_e13533);
            let assign14100_e13536: f64 = (assign14100_e13534 * locals.var_q_temp2);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign14100_e13536, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14100_e13534 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard577 == 0.0) {
            let assign14110_e13543: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
            let assign14110_e13546: f64 = (2.0 * locals.var_q_temp3);
            let assign14110_e13548: f64 = (assign14110_e13546 * locals.var_q_d1_expnum);
            let assign14110_e13549: f64 = (assign14110_e13543 + assign14110_e13548);
            let assign14110_e13551: f64 = (assign14110_e13549 + locals.var_q_aexp);
            let assign14110_e13555: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
            let assign14110_e13556: f64 = (locals.var_q_d2_ln + assign14110_e13555);
            let assign14110_e13558: f64 = (assign14110_e13556 * locals.var_q_sh_term);
            let assign14110_e13559: f64 = (assign14110_e13551 - assign14110_e13558);
            let assign14110_e13561: f64 = (assign14110_e13559 * locals.var_q_temp2);
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (assign14110_e13561, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign14110_e13546 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign14110_e13556 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14110_e13559 * locals.var_q_temp2_dn9)), );
        }

        let assign14120_e13566: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign14120_e13566;

        if (locals.var_guard578 != 0.0) {
            let assign14130_e13569: f64 = (locals.var_q_expnum).ln();
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign14130_e13569, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum), );
        }

        if (locals.var_guard578 != 0.0) {
            let assign14140_e13575: f64 = (1.0 / locals.var_q_expnum);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign14140_e13575, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))), );
        }

        if (locals.var_guard578 != 0.0) {
            let assign14150_e13581: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign14150_e13581, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard578 != 0.0) {
            let assign14160_e13587: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
            let assign14160_e13590: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
            let assign14160_e13591: f64 = (assign14160_e13587 - assign14160_e13590);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign14160_e13591, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))), );
        }

        if (locals.var_guard578 == 0.0) {
            let assign14170_e13598: f64 = (locals.var_q_k1q1 + 0.6931471805599);
            let assign14170_e13600: f64 = (-locals.var_q_k1q1);
            let assign14170_e13601: f64 = (assign14170_e13600).ln();
            let assign14170_e13602: f64 = (assign14170_e13598 + assign14170_e13601);
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign14170_e13602, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign14170_e13600)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign14170_e13600)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign14170_e13600)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign14170_e13600)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign14170_e13600)), );
        }

        if (locals.var_guard578 == 0.0) {
            let assign14180_e13609: f64 = (1.0 / locals.var_q1s);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign14180_e13609, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))), );
        }

        if (locals.var_guard578 == 0.0) {
            let assign14190_e13616: f64 = (locals.var_k1 + locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign14190_e13616, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9), );
        }

        if (locals.var_guard578 == 0.0) {
            let assign14200_e13622: f64 = (-locals.var_q_temp1);
            let assign14200_e13624: f64 = (assign14200_e13622 * locals.var_q_temp1);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign14200_e13624, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign14200_e13622 * locals.var_q_temp1_dn9)), );
        }

        let assign14210_e13629: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign14210_e13631: f64 = (assign14210_e13629 + locals.var_q1s);
        let assign14210_e13634: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign14210_e13635: f64 = (assign14210_e13631 + assign14210_e13634);
        let assign14210_e13637: f64 = (assign14210_e13635 - locals.var_q_ln_term);
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9, ) = (assign14210_e13637, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9), );

        let assign14220_e13641: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign14220_e13642: f64 = (1.0 + assign14220_e13641);
        let assign14220_e13644: f64 = (assign14220_e13642 - locals.var_q_d1_ln);
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9, ) = (assign14220_e13644, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9), );

        let assign14230_e13647: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign14230_e13649: f64 = (assign14230_e13647 - locals.var_q_d2_ln);
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9, ) = (assign14230_e13649, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9), );

        let assign14240_e13653: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign14240_e13654: f64 = (locals.var_q_k1q1 + assign14240_e13653);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign14240_e13654, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))), );

        let assign14250_e13658: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign14250_e13659: f64 = (locals.var_k1 + assign14250_e13658);
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9, ) = (assign14250_e13659, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))), );

        let assign14260_e13662: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9, ) = (assign14260_e13662, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)), );

        let assign14270_e13665: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign14270_e13667: f64 = (assign14270_e13665 - locals.var_q_aexp);
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign14270_e13667, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9), );

        let assign14280_e13670: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign14280_e13673: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign14280_e13674: f64 = (assign14280_e13670 + assign14280_e13673);
        let assign14280_e13676: f64 = (assign14280_e13674 + locals.var_q_aexp);
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign14280_e13676, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9), );

    }
}
