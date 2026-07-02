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
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign10_e732, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign20_e735: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1 = assign20_e735;

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
            (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9, ) = (assign30_e764, (0.5 * ((locals.var_temp_dn4 + (p.p18 * locals.var_temp_dn4)) + ((((locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn4 - (p.p18 * locals.var_temp_dn4)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn6 + (p.p18 * locals.var_temp_dn6)) + ((((locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn6 - (p.p18 * locals.var_temp_dn6)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn7 + (p.p18 * locals.var_temp_dn7)) + ((((locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn7 - (p.p18 * locals.var_temp_dn7)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn8 + (p.p18 * locals.var_temp_dn8)) + ((((locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn8 - (p.p18 * locals.var_temp_dn8)))) / (2.0 * assign30_e762)))), (0.5 * ((locals.var_temp_dn9 + (p.p18 * locals.var_temp_dn9)) + ((((locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)) * assign30_e758) + (assign30_e751 * (locals.var_temp_dn9 - (p.p18 * locals.var_temp_dn9)))) / (2.0 * assign30_e762)))), );
        }

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
            (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9, ) = (assign40_e795, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), );
        }

        if (locals.var_guard1 == 0.0) {
            let assign50_e803: f64 = (locals.var_temp + 1.0);
            let assign50_e806: f64 = (locals.var_temp - 1.0);
            let assign50_e809: f64 = (locals.var_temp - 1.0);
            let assign50_e810: f64 = (assign50_e806 * assign50_e809);
            let assign50_e812: f64 = (assign50_e810 + 0.001);
            let assign50_e813: f64 = (assign50_e812).sqrt();
            let assign50_e814: f64 = (assign50_e803 + assign50_e813);
            let assign50_e815: f64 = (0.5 * assign50_e814);
            (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9, ) = (assign50_e815, (0.5 * (locals.var_temp_dn4 + (((locals.var_temp_dn4 * assign50_e809) + (assign50_e806 * locals.var_temp_dn4)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn6 + (((locals.var_temp_dn6 * assign50_e809) + (assign50_e806 * locals.var_temp_dn6)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn7 + (((locals.var_temp_dn7 * assign50_e809) + (assign50_e806 * locals.var_temp_dn7)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn8 + (((locals.var_temp_dn8 * assign50_e809) + (assign50_e806 * locals.var_temp_dn8)) / (2.0 * assign50_e813)))), (0.5 * (locals.var_temp_dn9 + (((locals.var_temp_dn9 * assign50_e809) + (assign50_e806 * locals.var_temp_dn9)) / (2.0 * assign50_e813)))), );
        }

        if (locals.var_guard1 == 0.0) {
            (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9, ) = (600.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign70_e837: f64 = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p439 > 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard2 = assign70_e837;

        if (locals.var_guard2 != 0.0) {
            locals.var_swshe_i = p.p5;
        }

        if (locals.var_guard2 == 0.0) {
            locals.var_swshe_i = 0.0;
        }

        (locals.var_dtc, locals.var_dtc_dn4, ) = (0.0, 0.0, );

        (locals.var_tkc, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9, ) = (locals.var_tkd, locals.var_tkd_dn4, locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9, );

        let assign140_e856: f64 = (locals.var_tkc * locals.var_tkc);
        (locals.var_tkc_sq, locals.var_tkc_sq_dn4, locals.var_tkc_sq_dn6, locals.var_tkc_sq_dn7, locals.var_tkc_sq_dn8, locals.var_tkc_sq_dn9, ) = (assign140_e856, ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4)), ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6)), ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7)), ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8)), ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9)), );

        let assign150_e859: f64 = (locals.var_tkc - locals.var_tkr);
        (locals.var_dt, locals.var_dt_dn4, locals.var_dt_dn6, locals.var_dt_dn7, locals.var_dt_dn8, locals.var_dt_dn9, ) = (assign150_e859, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9, );

        let assign160_e862: f64 = (locals.var_tkc / locals.var_tkr);
        (locals.var_rt, locals.var_rt_dn4, locals.var_rt_dn6, locals.var_rt_dn7, locals.var_rt_dn8, locals.var_rt_dn9, ) = (assign160_e862, (locals.var_tkc_dn4 / locals.var_tkr), (locals.var_tkc_dn6 / locals.var_tkr), (locals.var_tkc_dn7 / locals.var_tkr), (locals.var_tkc_dn8 / locals.var_tkr), (locals.var_tkc_dn9 / locals.var_tkr), );

        let assign170_e865: f64 = (locals.var_tkr / locals.var_tkc);
        (locals.var_rtn, locals.var_rtn_dn4, locals.var_rtn_dn6, locals.var_rtn_dn7, locals.var_rtn_dn8, locals.var_rtn_dn9, ) = (assign170_e865, (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc))), );

        let assign180_e868: f64 = (locals.var_tkc * 8.617332384961e-5);
        (locals.var_phit0, locals.var_phit0_dn4, locals.var_phit0_dn6, locals.var_phit0_dn7, locals.var_phit0_dn8, locals.var_phit0_dn9, ) = (assign180_e868, (locals.var_tkc_dn4 * 8.617332384961e-5), (locals.var_tkc_dn6 * 8.617332384961e-5), (locals.var_tkc_dn7 * 8.617332384961e-5), (locals.var_tkc_dn8 * 8.617332384961e-5), (locals.var_tkc_dn9 * 8.617332384961e-5), );

        let assign190_e871: f64 = (1.0 / locals.var_phit0);
        (locals.var_inv_phit0, locals.var_inv_phit0_dn4, locals.var_inv_phit0_dn6, locals.var_inv_phit0_dn7, locals.var_inv_phit0_dn8, locals.var_inv_phit0_dn9, ) = (assign190_e871, (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0))), );

        let assign200_e874: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard83 = assign200_e874;

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

        let assign310_e917: f64 = if p.p45 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard84 = assign310_e917;

        if ((locals.var_guard83 != 0.0) && (locals.var_guard84 != 0.0)) {
            let assign320_e922: f64 = (-1.0);
            locals.var_typech_i = assign320_e922;
        }

        if (locals.var_guard83 != 0.0) {
            let assign330_e927: f64 = (p.p45).abs();
            let assign330_e929: f64 = (assign330_e927).min(1e19);
            let assign330_e931: f64 = (assign330_e929 * 1000000.0);
            locals.var_nch_i = assign330_e931;
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_typesub_i = 1.0;
        }

        let assign350_e940: f64 = if p.p46 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard85 = assign350_e940;

        if ((locals.var_guard83 != 0.0) && (locals.var_guard85 != 0.0)) {
            let assign360_e945: f64 = (-1.0);
            locals.var_typesub_i = assign360_e945;
        }

        if (locals.var_guard83 != 0.0) {
            let assign370_e950: f64 = (p.p46).abs();
            let assign370_e952: f64 = (assign370_e950).max(1e16);
            let assign370_e954: f64 = (assign370_e952).min(1e21);
            let assign370_e956: f64 = (assign370_e954 * 1000000.0);
            locals.var_nsub_i = assign370_e956;
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_ct_i = p.p47;
            locals.var_toxp_i = p.p48;
        }

        if (locals.var_guard83 != 0.0) {
            let assign400_e970: f64 = (p.p49 * 1000000.0);
            locals.var_nov_i = assign400_e970;
        }

        if (locals.var_guard83 != 0.0) {
            let assign410_e976: f64 = (p.p50 * 1000000.0);
            locals.var_novd_i = assign410_e976;
        }

        if (locals.var_guard83 != 0.0) {
            (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, ) = (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_stvfb_i = p.p53;
        }

        if (locals.var_guard83 != 0.0) {
            let assign450_e994: f64 = (p.p54 * 1000000.0);
            (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9, ) = (assign450_e994, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_cic1_i = p.p55;
            locals.var_cic2_i = p.p56;
            locals.var_psce1_i = p.p57;
        }

        if (locals.var_guard83 != 0.0) {
            let assign490_e1012: f64 = (p.p58 * locals.var_psce1_i);
            let assign490_e1014: f64 = (assign490_e1012 * locals.var_tox2_i);
            let assign490_e1016: f64 = (assign490_e1014 / locals.var_tox1_i);
            locals.var_psce2_i = assign490_e1016;
        }

        if (locals.var_guard83 != 0.0) {
            let assign500_e1022: f64 = (p.p59 * 1000000.0);
            locals.var_nsddc_i = assign500_e1022;
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_pscedlb_i = p.p60;
            locals.var_pnce_i = p.p61;
            (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, ) = (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign540_e1040: f64 = (p.p63 * locals.var_cf1_t);
            let assign540_e1042: f64 = (assign540_e1040 * locals.var_tox2_i);
            let assign540_e1044: f64 = (assign540_e1042 / locals.var_tox1_i);
            (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, ) = (assign540_e1044, (((p.p63 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 != 0.0) {
            (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9, ) = (p.p64, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_cfd_i = p.p65;
            locals.var_cfdl_i = p.p66;
            locals.var_cfdlb_i = p.p67;
            (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9, ) = (p.p68, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign600_e1070: f64 = (p.p69 * locals.var_betn1_t);
            (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9, ) = (assign600_e1070, (p.p69 * locals.var_betn1_t_dn4), (p.p69 * locals.var_betn1_t_dn6), (p.p69 * locals.var_betn1_t_dn7), (p.p69 * locals.var_betn1_t_dn8), (p.p69 * locals.var_betn1_t_dn9), );
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
            let assign1360_e1376: f64 = (p.p141 * locals.var_psce1edge_i);
            let assign1360_e1378: f64 = (assign1360_e1376 * locals.var_tox2_i);
            let assign1360_e1380: f64 = (assign1360_e1378 / locals.var_tox1_i);
            (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9, ) = (assign1360_e1380, (((p.p141 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p141 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 != 0.0) {
            (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9, ) = (p.p142, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign1380_e1390: f64 = (p.p143 * locals.var_cf1edge_i);
            let assign1380_e1392: f64 = (assign1380_e1390 * locals.var_tox2_i);
            let assign1380_e1394: f64 = (assign1380_e1392 / locals.var_tox1_i);
            (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9, ) = (assign1380_e1394, (((p.p143 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p143 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 != 0.0) {
            locals.var_cfdedge_i = p.p144;
            (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9, ) = (p.p145, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_stbetedge_i = p.p146;
            locals.var_areaq_i = p.p151;
            (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9, ) = (p.p152, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 != 0.0) {
            let assign1440_e1420: f64 = (p.p153 * 1000000.0);
            locals.var_nsdac_i = assign1440_e1420;
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

        let assign1560_e1469: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard86 = assign1560_e1469;

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign1580_e1477: f64 = if param_given[156] { 1.0 } else { 0.0 };
        let assign1580_e1479: f64 = if assign1580_e1477 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard87 = assign1580_e1479;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard87 != 0.0)) {
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (p.p156, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign1610_e1495: f64 = if param_given[157] { 1.0 } else { 0.0 };
        let assign1610_e1497: f64 = if assign1610_e1495 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard88 = assign1610_e1497;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard88 != 0.0)) {
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (p.p157, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            locals.var_psceac1_i = p.p57;
        }

        let assign1640_e1513: f64 = if param_given[158] { 1.0 } else { 0.0 };
        let assign1640_e1515: f64 = if assign1640_e1513 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard89 = assign1640_e1515;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard89 != 0.0)) {
            locals.var_psceac1_i = p.p158;
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            let assign1660_e1529: f64 = (p.p58 * locals.var_psceac1_i);
            let assign1660_e1531: f64 = (assign1660_e1529 * locals.var_tox2_i);
            let assign1660_e1533: f64 = (assign1660_e1531 / locals.var_tox1_i);
            locals.var_psceac2_i = assign1660_e1533;
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign1680_e1543: f64 = if param_given[159] { 1.0 } else { 0.0 };
        let assign1680_e1545: f64 = if assign1680_e1543 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard90 = assign1680_e1545;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard90 != 0.0)) {
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (p.p159, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            let assign1700_e1559: f64 = (p.p63 * locals.var_cfac1_t);
            let assign1700_e1561: f64 = (assign1700_e1559 * locals.var_tox2_i);
            let assign1700_e1563: f64 = (assign1700_e1561 / locals.var_tox1_i);
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (assign1700_e1563, (((p.p63 * locals.var_cfac1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p63 * locals.var_cfac1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign1720_e1573: f64 = if param_given[160] { 1.0 } else { 0.0 };
        let assign1720_e1575: f64 = if assign1720_e1573 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard91 = assign1720_e1575;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard91 != 0.0)) {
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (p.p160, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            locals.var_axac_i = p.p97;
        }

        let assign1750_e1591: f64 = if param_given[161] { 1.0 } else { 0.0 };
        let assign1750_e1593: f64 = if assign1750_e1591 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard92 = assign1750_e1593;

        if (((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) && (locals.var_guard92 != 0.0)) {
            locals.var_axac_i = p.p161;
        }

        if ((locals.var_guard83 != 0.0) && (locals.var_guard86 != 0.0)) {
            locals.var_alpac_i = p.p98;
        }

        let assign1780_e1609: f64 = if param_given[162] { 1.0 } else { 0.0 };
        let assign1780_e1611: f64 = if assign1780_e1609 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard93 = assign1780_e1611;

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
        }

        if (locals.var_guard83 == 0.0) {
            let assign2040_e1720: f64 = (1.0 / p.p29);
            locals.var_invnf = assign2040_e1720;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2050_e1727: f64 = (p.p21 * locals.var_invnf);
            let assign2050_e1729: f64 = (assign2050_e1727).max(1e-9);
            locals.var_w_i = assign2050_e1729;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2060_e1736: f64 = (p.p23 * locals.var_invnf);
            locals.var_adrain_i = assign2060_e1736;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2070_e1743: f64 = (p.p22 * locals.var_invnf);
            locals.var_asource_i = assign2070_e1743;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2080_e1750: f64 = (p.p25 * locals.var_invnf);
            locals.var_pdrain_i = assign2080_e1750;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2090_e1757: f64 = (p.p24 * locals.var_invnf);
            locals.var_psource_i = assign2090_e1757;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2100_e1764: f64 = (p.p30 * p.p29);
            locals.var_mult_i_int = assign2100_e1764;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_len = 1e-6;
            locals.var_wen = 1e-6;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2130_e1781: f64 = (locals.var_len / p.p20);
            locals.var_il = assign2130_e1781;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2140_e1788: f64 = (locals.var_wen / locals.var_w_i);
            locals.var_iw = assign2140_e1788;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2150_e1797: f64 = (p.p188 * locals.var_il);
            let assign2150_e1798: f64 = (1.0 + assign2150_e1797);
            let assign2150_e1799: f64 = (p.p187 * assign2150_e1798);
            let assign2150_e1803: f64 = (p.p189 * locals.var_iw);
            let assign2150_e1804: f64 = (1.0 + assign2150_e1803);
            let assign2150_e1805: f64 = (assign2150_e1799 * assign2150_e1804);
            locals.var_dellps = assign2150_e1805;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2160_e1814: f64 = (p.p193 * locals.var_iw);
            let assign2160_e1815: f64 = (1.0 + assign2160_e1814);
            let assign2160_e1816: f64 = (p.p191 * assign2160_e1815);
            let assign2160_e1820: f64 = (p.p192 * locals.var_il);
            let assign2160_e1821: f64 = (1.0 + assign2160_e1820);
            let assign2160_e1822: f64 = (assign2160_e1816 * assign2160_e1821);
            locals.var_delwod = assign2160_e1822;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2170_e1829: f64 = (p.p20 + locals.var_dellps);
            let assign2170_e1832: f64 = (2.0 * p.p190);
            let assign2170_e1833: f64 = (assign2170_e1829 - assign2170_e1832);
            let assign2170_e1835: f64 = (assign2170_e1833).max(1e-9);
            locals.var_le = assign2170_e1835;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2180_e1842: f64 = (locals.var_w_i + locals.var_delwod);
            let assign2180_e1845: f64 = (2.0 * p.p194);
            let assign2180_e1846: f64 = (assign2180_e1842 - assign2180_e1845);
            let assign2180_e1848: f64 = (assign2180_e1846).max(1e-9);
            locals.var_we = assign2180_e1848;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2190_e1855: f64 = (p.p20 + locals.var_dellps);
            let assign2190_e1858: f64 = (2.0 * p.p190);
            let assign2190_e1859: f64 = (assign2190_e1855 - assign2190_e1858);
            let assign2190_e1861: f64 = (assign2190_e1859 + p.p195);
            let assign2190_e1863: f64 = (assign2190_e1861).max(1e-9);
            locals.var_lecv = assign2190_e1863;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2200_e1870: f64 = (locals.var_w_i + locals.var_delwod);
            let assign2200_e1873: f64 = (2.0 * p.p194);
            let assign2200_e1874: f64 = (assign2200_e1870 - assign2200_e1873);
            let assign2200_e1876: f64 = (assign2200_e1874 + p.p196);
            let assign2200_e1878: f64 = (assign2200_e1876).max(1e-9);
            locals.var_wecv = assign2200_e1878;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2210_e1885: f64 = (locals.var_len / locals.var_le);
            locals.var_ile = assign2210_e1885;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2220_e1892: f64 = (locals.var_wen / locals.var_we);
            locals.var_iwe = assign2220_e1892;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2230_e1899: f64 = (locals.var_ile * locals.var_iwe);
            locals.var_iae = assign2230_e1899;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2240_e1906: f64 = (p.p20 + locals.var_dellps);
            let assign2240_e1908: f64 = (assign2240_e1906).max(1e-9);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign2240_e1908, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2250_e1915: f64 = (locals.var_temp / locals.var_len);
            (locals.var_lphy, locals.var_lphy_dn4, locals.var_lphy_dn6, locals.var_lphy_dn7, locals.var_lphy_dn8, locals.var_lphy_dn9, ) = (assign2250_e1915, (locals.var_temp_dn4 / locals.var_len), (locals.var_temp_dn6 / locals.var_len), (locals.var_temp_dn7 / locals.var_len), (locals.var_temp_dn8 / locals.var_len), (locals.var_temp_dn9 / locals.var_len), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2260_e1922: f64 = (locals.var_w_i + locals.var_delwod);
            let assign2260_e1924: f64 = (assign2260_e1922).max(1e-9);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign2260_e1924, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2270_e1931: f64 = (locals.var_temp / locals.var_wen);
            (locals.var_wphy, locals.var_wphy_dn4, locals.var_wphy_dn6, locals.var_wphy_dn7, locals.var_wphy_dn8, locals.var_wphy_dn9, ) = (assign2270_e1931, (locals.var_temp_dn4 / locals.var_wen), (locals.var_temp_dn6 / locals.var_wen), (locals.var_temp_dn7 / locals.var_wen), (locals.var_temp_dn8 / locals.var_wen), (locals.var_temp_dn9 / locals.var_wen), );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_tox1_i = p.p197;
            locals.var_tsi_i = p.p198;
            locals.var_xge_i = p.p199;
            locals.var_tox2_i = p.p200;
            locals.var_typech_i = 1.0;
        }

        let assign2370_e1999: f64 = if p.p201 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard94 = assign2370_e1999;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard94 != 0.0)) {
            let assign2380_e2005: f64 = (-1.0);
            locals.var_typech_i = assign2380_e2005;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2390_e2011: f64 = (p.p201).abs();
            let assign2390_e2013: f64 = (assign2390_e2011).min(1e19);
            let assign2390_e2015: f64 = (assign2390_e2013 * 1000000.0);
            locals.var_nch_i = assign2390_e2015;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_typesub_i = 1.0;
        }

    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign2410_e2025: f64 = if p.p202 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard95 = assign2410_e2025;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard95 != 0.0)) {
            let assign2420_e2031: f64 = (-1.0);
            locals.var_typesub_i = assign2420_e2031;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2430_e2037: f64 = (p.p202).abs();
            let assign2430_e2039: f64 = (assign2430_e2037).max(1e16);
            let assign2430_e2041: f64 = (assign2430_e2039).min(1e21);
            let assign2430_e2043: f64 = (assign2430_e2041 * 1000000.0);
            locals.var_nsub_i = assign2430_e2043;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_ct_i = p.p203;
            locals.var_toxp_i = p.p204;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2460_e2060: f64 = (p.p205 * 1000000.0);
            locals.var_nov_i = assign2460_e2060;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2470_e2067: f64 = (p.p206 * 1000000.0);
            locals.var_novd_i = assign2470_e2067;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2480_e2075: f64 = (locals.var_ile).powf(p.p209);
            let assign2480_e2076: f64 = (p.p208 * assign2480_e2075);
            let assign2480_e2081: f64 = (locals.var_ile).powf(p.p211);
            let assign2480_e2082: f64 = (p.p210 * assign2480_e2081);
            let assign2480_e2083: f64 = (1.0 + assign2480_e2082);
            let assign2480_e2084: f64 = (assign2480_e2076 / assign2480_e2083);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign2480_e2084, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2490_e2091: f64 = (p.p207 + locals.var_temp);
            let assign2490_e2094: f64 = (p.p212 * locals.var_iwe);
            let assign2490_e2095: f64 = (assign2490_e2091 + assign2490_e2094);
            let assign2490_e2098: f64 = (p.p213 * locals.var_iae);
            let assign2490_e2099: f64 = (assign2490_e2095 + assign2490_e2098);
            (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, ) = (assign2490_e2099, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2500_e2107: f64 = (p.p215 * locals.var_tox2_i);
            let assign2500_e2109: f64 = (assign2500_e2107 / locals.var_tox1_i);
            let assign2500_e2111: f64 = (assign2500_e2109 * locals.var_temp);
            let assign2500_e2112: f64 = (p.p214 + assign2500_e2111);
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (assign2500_e2112, (assign2500_e2109 * locals.var_temp_dn4), (assign2500_e2109 * locals.var_temp_dn6), (assign2500_e2109 * locals.var_temp_dn7), (assign2500_e2109 * locals.var_temp_dn8), (assign2500_e2109 * locals.var_temp_dn9), );
        }

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
            locals.var_stvfb_i = assign2510_e2135;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2520_e2144: f64 = (p.p221 * locals.var_ile);
            let assign2520_e2145: f64 = (1.0 + assign2520_e2144);
            let assign2520_e2146: f64 = (p.p220 * assign2520_e2145);
            let assign2520_e2148: f64 = (assign2520_e2146 * 1000000.0);
            (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9, ) = (assign2520_e2148, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2530_e2155: f64 = (locals.var_temp0__blk79).max(1e25);
            let assign2530_e2157: f64 = (assign2530_e2155).min(1e28);
            (locals.var_np_i, locals.var_np_i_dn4, locals.var_np_i_dn6, locals.var_np_i_dn7, locals.var_np_i_dn8, locals.var_np_i_dn9, ) = (assign2530_e2157, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn4 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn6 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn7 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn8 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if locals.var_temp0__blk79 >= 1e25 { locals.var_temp0__blk79_dn9 } else { 0.0 } } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cic1_i = p.p222;
            locals.var_cic2_i = p.p223;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2560_e2174: f64 = (1.0 - locals.var_xge_i);
            locals.var_one_m_xge = assign2560_e2174;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2570_e2181: f64 = (1.04479e-10 * locals.var_one_m_xge);
            let assign2570_e2184: f64 = (1.43438e-10 * locals.var_xge_i);
            let assign2570_e2185: f64 = (assign2570_e2181 + assign2570_e2184);
            locals.var_epsch = assign2570_e2185;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2580_e2192: f64 = (locals.var_epsch / 3.45313e-11);
            let assign2580_e2194: f64 = (assign2580_e2192 * locals.var_tsi_i);
            let assign2580_e2197: f64 = (locals.var_tox1_i + 4e-10);
            let assign2580_e2198: f64 = (assign2580_e2194 * assign2580_e2197);
            let assign2580_e2199: f64 = (assign2580_e2198).sqrt();
            let assign2580_e2201: f64 = (assign2580_e2199 / locals.var_le);
            locals.var_lambda_le = assign2580_e2201;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2590_e2208: f64 = (p.p224 * 2.0);
            let assign2590_e2211: f64 = (locals.var_lambda_le).powf(p.p225);
            let assign2590_e2212: f64 = (assign2590_e2208 * assign2590_e2211);
            let assign2590_e2216: f64 = (p.p226 * locals.var_iwe);
            let assign2590_e2217: f64 = (1.0 + assign2590_e2216);
            let assign2590_e2218: f64 = (assign2590_e2212 * assign2590_e2217);
            locals.var_psce_p = assign2590_e2218;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2600_e2225: f64 = (locals.var_psce_p).max(0.0);
            let assign2600_e2227: f64 = (assign2600_e2225).min(5.0);
            locals.var_psce1_i = assign2600_e2227;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2610_e2234: f64 = (p.p227 * locals.var_psce1_i);
            let assign2610_e2236: f64 = (assign2610_e2234 * locals.var_tox2_i);
            let assign2610_e2238: f64 = (assign2610_e2236 / locals.var_tox1_i);
            locals.var_psce2_i = assign2610_e2238;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2620_e2245: f64 = (p.p228 * 1000000.0);
            locals.var_nsddc_i = assign2620_e2245;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_pscedlb_i = p.p229;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2640_e2257: f64 = (p.p230 * locals.var_iwe);
            locals.var_pnce_p = assign2640_e2257;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2650_e2264: f64 = (-1.0);
            let assign2650_e2265: f64 = (locals.var_pnce_p).max(assign2650_e2264);
            let assign2650_e2267: f64 = (assign2650_e2265).min(1.0);
            locals.var_pnce_i = assign2650_e2267;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2660_e2274: f64 = (locals.var_lambda_le).powf(p.p232);
            let assign2660_e2278: f64 = (p.p233 * locals.var_iwe);
            let assign2660_e2279: f64 = (1.0 + assign2660_e2278);
            let assign2660_e2280: f64 = (assign2660_e2274 * assign2660_e2279);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign2660_e2280, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2670_e2287: f64 = (p.p231 * locals.var_temp);
            (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9, ) = (assign2670_e2287, (p.p231 * locals.var_temp_dn4), (p.p231 * locals.var_temp_dn6), (p.p231 * locals.var_temp_dn7), (p.p231 * locals.var_temp_dn8), (p.p231 * locals.var_temp_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2680_e2294: f64 = (locals.var_cf_p).max(0.0);
            (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, ) = (assign2680_e2294, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2690_e2301: f64 = (p.p234 * locals.var_cf1_t);
            let assign2690_e2303: f64 = (assign2690_e2301 * locals.var_tox2_i);
            let assign2690_e2305: f64 = (assign2690_e2303 / locals.var_tox1_i);
            (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, ) = (assign2690_e2305, (((p.p234 * locals.var_cf1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cf1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2700_e2312: f64 = (p.p235 * locals.var_temp);
            (locals.var_stcf_i, locals.var_stcf_i_dn4, locals.var_stcf_i_dn6, locals.var_stcf_i_dn7, locals.var_stcf_i_dn8, locals.var_stcf_i_dn9, ) = (assign2700_e2312, (p.p235 * locals.var_temp_dn4), (p.p235 * locals.var_temp_dn6), (p.p235 * locals.var_temp_dn7), (p.p235 * locals.var_temp_dn8), (p.p235 * locals.var_temp_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cfd_i = p.p236;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2720_e2324: f64 = (p.p237 * locals.var_ile);
            let assign2720_e2328: f64 = (p.p238 * locals.var_iwe);
            let assign2720_e2329: f64 = (1.0 + assign2720_e2328);
            let assign2720_e2331: f64 = (assign2720_e2329).max(0.001);
            let assign2720_e2332: f64 = (assign2720_e2324 / assign2720_e2331);
            locals.var_cfdl_i = assign2720_e2332;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cfdlb_i = p.p239;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2740_e2343: f64 = (-locals.var_le);
            let assign2740_e2348: f64 = (p.p244 * locals.var_iwe);
            let assign2740_e2349: f64 = (1.0 + assign2740_e2348);
            let assign2740_e2351: f64 = (assign2740_e2349).max(0.001);
            let assign2740_e2352: f64 = (p.p243 * assign2740_e2351);
            let assign2740_e2353: f64 = (assign2740_e2343 / assign2740_e2352);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign2740_e2353, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign2750_e2358: f64 = (-80.0);
        let assign2750_e2359: f64 = if locals.var_temp1 > assign2750_e2358 { 1.0 } else { 0.0 };
        locals.var_guard96 = assign2750_e2359;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard96 != 0.0)) {
            let assign2760_e2365: f64 = (locals.var_temp1).exp();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign2760_e2365, (assign2760_e2365 * locals.var_temp1_dn4), (assign2760_e2365 * locals.var_temp1_dn6), (assign2760_e2365 * locals.var_temp1_dn7), (assign2760_e2365 * locals.var_temp1_dn8), (assign2760_e2365 * locals.var_temp1_dn9), );
        }

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
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign2770_e2398, (-((1.80485e-35 * (((-locals.var_temp1_dn4) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn4)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn4) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn6) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn6)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn6) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn7) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn7)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn7) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn8) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn8)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn8) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-locals.var_temp1_dn9) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-locals.var_temp1_dn9)) * assign2770_e2393) + (assign2770_e2385 * ((-locals.var_temp1_dn9) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2780_e2404: f64 = (-locals.var_le);
            let assign2780_e2406: f64 = (assign2780_e2404 / p.p246);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign2780_e2406, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign2790_e2411: f64 = (-80.0);
        let assign2790_e2412: f64 = if locals.var_temp3 > assign2790_e2411 { 1.0 } else { 0.0 };
        locals.var_guard97 = assign2790_e2412;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard97 != 0.0)) {
            let assign2800_e2418: f64 = (locals.var_temp3).exp();
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign2800_e2418, (assign2800_e2418 * locals.var_temp3_dn4), (assign2800_e2418 * locals.var_temp3_dn6), (assign2800_e2418 * locals.var_temp3_dn7), (assign2800_e2418 * locals.var_temp3_dn8), (assign2800_e2418 * locals.var_temp3_dn9), );
        }

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
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign2810_e2451, (-((1.80485e-35 * (((-locals.var_temp3_dn4) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn4)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn4) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn6) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn6)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn6) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn7) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn7)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn7) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn8) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn8)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn8) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-locals.var_temp3_dn9) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-locals.var_temp3_dn9)) * assign2810_e2446) + (assign2810_e2438 * ((-locals.var_temp3_dn9) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), );
        }

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
            (locals.var_gpe, locals.var_gpe_dn4, locals.var_gpe_dn6, locals.var_gpe_dn7, locals.var_gpe_dn8, locals.var_gpe_dn9, ) = (assign2820_e2480, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn4) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn4)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn4) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn4)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn6) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn6)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn6) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn6)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn7) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn7)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn7) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn7)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn8) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn8)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn8) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn8)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * locals.var_temp2_dn9) * locals.var_temp1) - (assign2820_e2467 * locals.var_temp1_dn9)) / (locals.var_temp1 * locals.var_temp1)) + ((((p.p245 * locals.var_temp4_dn9) * locals.var_temp3) - (assign2820_e2475 * locals.var_temp3_dn9)) / (locals.var_temp3 * locals.var_temp3))) } else { 0.0 }, );
        }

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
            locals.var_gwe = assign2830_e2502;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2840_e2509: f64 = (p.p240 / locals.var_gpe);
            let assign2840_e2511: f64 = (assign2840_e2509 * locals.var_gwe);
            (locals.var_ge, locals.var_ge_dn4, locals.var_ge_dn6, locals.var_ge_dn7, locals.var_ge_dn8, locals.var_ge_dn9, ) = (assign2840_e2511, ((-((p.p240 * locals.var_gpe_dn4) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn6) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn7) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn8) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), ((-((p.p240 * locals.var_gpe_dn9) / (locals.var_gpe * locals.var_gpe))) * locals.var_gwe), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2850_e2518: f64 = (locals.var_ge * locals.var_we);
            let assign2850_e2520: f64 = (assign2850_e2518 / locals.var_le);
            (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9, ) = (assign2850_e2520, ((locals.var_ge_dn4 * locals.var_we) / locals.var_le), ((locals.var_ge_dn6 * locals.var_we) / locals.var_le), ((locals.var_ge_dn7 * locals.var_we) / locals.var_le), ((locals.var_ge_dn8 * locals.var_we) / locals.var_le), ((locals.var_ge_dn9 * locals.var_we) / locals.var_le), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2860_e2527: f64 = (locals.var_betn_p).max(1e-10);
            (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9, ) = (assign2860_e2527, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign2870_e2534: f64 = (p.p250 * locals.var_betn1_t);
            (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9, ) = (assign2870_e2534, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9), );
        }

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
            locals.var_stbet_i = assign2880_e2557;
        }

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
            locals.var_cs_p = assign2890_e2580;
        }

        if (locals.var_guard83 == 0.0) {
            let assign2900_e2587: f64 = (locals.var_cs_p).max(0.0);
            locals.var_cs_t = assign2900_e2587;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_csfi_i = p.p260;
            locals.var_csbi_i = p.p261;
        }

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
            locals.var_stcs_i = assign2930_e2620;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_thecs_t = p.p266;
            locals.var_stthecs_i = p.p267;
            locals.var_csthr_i = p.p268;
            locals.var_csthrb_i = p.p269;
            locals.var_mue_t = p.p270;
            locals.var_stmue_i = p.p271;
            locals.var_themu_t = p.p272;
            locals.var_stthemu_i = p.p273;
        }

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
            locals.var_xcor_t = assign3020_e2683;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_xcorb_i = p.p279;
            locals.var_stxcor_i = p.p280;
            locals.var_feta_i = p.p281;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3060_e2705: f64 = (p.p282 * locals.var_iwe);
            let assign3060_e2709: f64 = (p.p283 * locals.var_iwe);
            let assign3060_e2710: f64 = (1.0 + assign3060_e2709);
            let assign3060_e2711: f64 = (assign3060_e2705 * assign3060_e2710);
            locals.var_rs_p = assign3060_e2711;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3070_e2718: f64 = (locals.var_rs_p).max(0.0);
            locals.var_rs_t = assign3070_e2718;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_rsig_i = p.p284;
            locals.var_strs_i = p.p285;
            locals.var_rsg_i = p.p286;
            locals.var_thersg_i = p.p287;
            locals.var_rsb_i = p.p288;
        }

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
            (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9, ) = (assign3130_e2768, (((locals.var_ge_dn4 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn6 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn7 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn8 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((locals.var_ge_dn9 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3140_e2775: f64 = (locals.var_thesat_p).max(0.0);
            (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, ) = (assign3140_e2775, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 }, );
        }

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
            locals.var_stthesat_i = assign3150_e2798;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_thesat1_i = p.p298;
            locals.var_thesat2_i = p.p299;
        }

    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard83 == 0.0) {
            let assign3180_e2818: f64 = (locals.var_ile).powf(p.p302);
            let assign3180_e2819: f64 = (p.p301 * assign3180_e2818);
            let assign3180_e2824: f64 = (locals.var_ile).powf(p.p304);
            let assign3180_e2825: f64 = (p.p303 * assign3180_e2824);
            let assign3180_e2826: f64 = (1.0 + assign3180_e2825);
            let assign3180_e2827: f64 = (assign3180_e2819 / assign3180_e2826);
            let assign3180_e2828: f64 = (1.0 + assign3180_e2827);
            let assign3180_e2829: f64 = (p.p300 / assign3180_e2828);
            locals.var_ax_p = assign3180_e2829;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3190_e2836: f64 = (locals.var_ax_p).max(1.0);
            let assign3190_e2838: f64 = (assign3190_e2836).min(16.0);
            locals.var_ax_i = assign3190_e2838;
        }

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
            locals.var_alp_p = assign3200_e2861;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3210_e2868: f64 = (locals.var_alp_p).max(0.0);
            locals.var_alp_i = assign3210_e2868;
        }

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
            locals.var_alp1_p = assign3220_e2891;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3230_e2898: f64 = (locals.var_alp1_p).max(0.0);
            locals.var_alp1_i = assign3230_e2898;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_alpb_i = p.p315;
            locals.var_vp_i = p.p316;
            locals.var_vpg_i = p.p317;
            locals.var_gco_i = p.p318;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3280_e2925: f64 = (p.p319 / locals.var_iae);
            locals.var_iginv_t = assign3280_e2925;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3290_e2932: f64 = (p.p320 / locals.var_iwe);
            locals.var_igovinv_t = assign3290_e2932;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3300_e2939: f64 = (p.p321 / locals.var_iwe);
            locals.var_igovinvd_t = assign3300_e2939;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3310_e2946: f64 = (p.p335 / locals.var_iwe);
            locals.var_fnovinv_t = assign3310_e2946;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3320_e2953: f64 = (p.p336 / locals.var_iwe);
            locals.var_fnovinvd_t = assign3320_e2953;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3330_e2960: f64 = (p.p322 / locals.var_iwe);
            locals.var_igovacc_t = assign3330_e2960;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3340_e2967: f64 = (p.p323 / locals.var_iwe);
            locals.var_igovaccd_t = assign3340_e2967;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_stig_i = p.p324;
            locals.var_stigfn_i = p.p338;
            locals.var_gc2ch_i = p.p325;
            locals.var_gc3ch_i = p.p326;
            locals.var_gc2ovinv_i = p.p327;
            locals.var_gcovinvfn_i = p.p337;
            locals.var_gc3ovinv_i = p.p328;
            locals.var_gc2ovacc_i = p.p329;
            locals.var_gc3ovacc_i = p.p330;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3440_e3019: f64 = (p.p331 * locals.var_ile);
            locals.var_gcdov_i = assign3440_e3019;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_gcvdov_i = p.p332;
            locals.var_chib_i = p.p333;
            locals.var_niginv_i = p.p334;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3480_e3042: f64 = (p.p341 / locals.var_iwe);
            let assign3480_e3043: f64 = (p.p339 + assign3480_e3042);
            locals.var_agidl_p = assign3480_e3043;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3490_e3050: f64 = (locals.var_agidl_p).max(0.0);
            (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9, ) = (assign3490_e3050, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3500_e3058: f64 = (p.p342 / locals.var_iwe);
            let assign3500_e3059: f64 = (p.p340 + assign3500_e3058);
            locals.var_agidld_p = assign3500_e3059;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3510_e3066: f64 = (locals.var_agidld_p).max(0.0);
            (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9, ) = (assign3510_e3066, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_bgidl_t = p.p343;
            locals.var_bgidld_t = p.p344;
            locals.var_stbgidl_i = p.p345;
            locals.var_stbgidld_i = p.p346;
            locals.var_cgidl_i = p.p347;
            locals.var_cgidld_i = p.p348;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3580_e3104: f64 = (p.p351 * locals.var_ile);
            let assign3580_e3105: f64 = (p.p349 + assign3580_e3104);
            locals.var_dgidl_i = assign3580_e3105;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3590_e3113: f64 = (p.p352 * locals.var_ile);
            let assign3590_e3114: f64 = (p.p350 + assign3590_e3113);
            locals.var_dgidld_i = assign3590_e3114;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3600_e3123: f64 = (p.p385 * locals.var_ile);
            let assign3600_e3124: f64 = (1.0 + assign3600_e3123);
            let assign3600_e3125: f64 = (p.p384 * assign3600_e3124);
            let assign3600_e3129: f64 = (p.p386 * locals.var_iwe);
            let assign3600_e3130: f64 = (1.0 + assign3600_e3129);
            let assign3600_e3131: f64 = (assign3600_e3125 * assign3600_e3130);
            locals.var_a1_p = assign3600_e3131;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3610_e3138: f64 = (locals.var_a1_p).max(0.0);
            locals.var_a1_i = assign3610_e3138;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_a2_t = p.p387;
            locals.var_sta2_i = p.p388;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3640_e3157: f64 = (p.p390 * locals.var_ile);
            let assign3640_e3158: f64 = (1.0 + assign3640_e3157);
            let assign3640_e3159: f64 = (p.p389 * assign3640_e3158);
            let assign3640_e3163: f64 = (p.p391 * locals.var_iwe);
            let assign3640_e3164: f64 = (1.0 + assign3640_e3163);
            let assign3640_e3165: f64 = (assign3640_e3159 * assign3640_e3164);
            locals.var_a3_p = assign3640_e3165;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3650_e3172: f64 = (locals.var_a3_p).max(0.0);
            locals.var_a3_i = assign3650_e3172;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3660_e3179: f64 = (2.0 * p.p353);
            let assign3660_e3182: f64 = (p.p354 * locals.var_we);
            let assign3660_e3183: f64 = (assign3660_e3179 + assign3660_e3182);
            locals.var_we_edge = assign3660_e3183;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_ctedge_i = p.p355;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3680_e3196: f64 = (locals.var_ile).powf(p.p358);
            let assign3680_e3197: f64 = (p.p357 * assign3680_e3196);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3680_e3197, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3690_e3204: f64 = (p.p356 + locals.var_temp);
            let assign3690_e3207: f64 = (p.p359 * locals.var_iwe);
            let assign3690_e3208: f64 = (assign3690_e3204 + assign3690_e3207);
            let assign3690_e3211: f64 = (p.p360 * locals.var_iae);
            let assign3690_e3212: f64 = (assign3690_e3208 + assign3690_e3211);
            (locals.var_vfb1edge_t, locals.var_vfb1edge_t_dn4, locals.var_vfb1edge_t_dn6, locals.var_vfb1edge_t_dn7, locals.var_vfb1edge_t_dn8, locals.var_vfb1edge_t_dn9, ) = (assign3690_e3212, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_vfb2edge_t = p.p361;
        }

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
            locals.var_stvfbedge_i = assign3710_e3240;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cic1edge_i = p.p366;
            locals.var_cic2edge_i = p.p367;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3740_e3257: f64 = (p.p368 * 2.0);
            let assign3740_e3260: f64 = (locals.var_lambda_le).powf(p.p369);
            let assign3740_e3261: f64 = (assign3740_e3257 * assign3740_e3260);
            let assign3740_e3265: f64 = (p.p370 * locals.var_iwe);
            let assign3740_e3266: f64 = (1.0 + assign3740_e3265);
            let assign3740_e3267: f64 = (assign3740_e3261 * assign3740_e3266);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3740_e3267, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3750_e3274: f64 = (locals.var_temp).max(0.0);
            let assign3750_e3276: f64 = (assign3750_e3274).min(5.0);
            (locals.var_psce1edge_i, locals.var_psce1edge_i_dn4, locals.var_psce1edge_i_dn6, locals.var_psce1edge_i_dn7, locals.var_psce1edge_i_dn8, locals.var_psce1edge_i_dn9, ) = (assign3750_e3276, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 } } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3760_e3283: f64 = (p.p371 * locals.var_psce1edge_i);
            let assign3760_e3285: f64 = (assign3760_e3283 * locals.var_tox2_i);
            let assign3760_e3287: f64 = (assign3760_e3285 / locals.var_tox1_i);
            (locals.var_psce2edge_i, locals.var_psce2edge_i_dn4, locals.var_psce2edge_i_dn6, locals.var_psce2edge_i_dn7, locals.var_psce2edge_i_dn8, locals.var_psce2edge_i_dn9, ) = (assign3760_e3287, (((p.p371 * locals.var_psce1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p371 * locals.var_psce1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3770_e3294: f64 = (locals.var_lambda_le).powf(p.p373);
            let assign3770_e3298: f64 = (p.p374 * locals.var_iwe);
            let assign3770_e3299: f64 = (1.0 + assign3770_e3298);
            let assign3770_e3300: f64 = (assign3770_e3294 * assign3770_e3299);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3770_e3300, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3780_e3307: f64 = (p.p372 * locals.var_temp);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3780_e3307, (p.p372 * locals.var_temp_dn4), (p.p372 * locals.var_temp_dn6), (p.p372 * locals.var_temp_dn7), (p.p372 * locals.var_temp_dn8), (p.p372 * locals.var_temp_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3790_e3314: f64 = (locals.var_temp).max(0.0);
            (locals.var_cf1edge_i, locals.var_cf1edge_i_dn4, locals.var_cf1edge_i_dn6, locals.var_cf1edge_i_dn7, locals.var_cf1edge_i_dn8, locals.var_cf1edge_i_dn9, ) = (assign3790_e3314, if locals.var_temp >= 0.0 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 0.0 { locals.var_temp_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3800_e3321: f64 = (p.p375 * locals.var_cf1edge_i);
            let assign3800_e3323: f64 = (assign3800_e3321 * locals.var_tox2_i);
            let assign3800_e3325: f64 = (assign3800_e3323 / locals.var_tox1_i);
            (locals.var_cf2edge_i, locals.var_cf2edge_i_dn4, locals.var_cf2edge_i_dn6, locals.var_cf2edge_i_dn7, locals.var_cf2edge_i_dn8, locals.var_cf2edge_i_dn9, ) = (assign3800_e3325, (((p.p375 * locals.var_cf1edge_i_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p375 * locals.var_cf1edge_i_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_cfdedge_i = p.p376;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3820_e3338: f64 = (p.p377 * p.p378);
            let assign3820_e3340: f64 = (assign3820_e3338 / locals.var_le);
            let assign3820_e3343: f64 = (-locals.var_le);
            let assign3820_e3345: f64 = (assign3820_e3343 / p.p378);
            let assign3820_e3346: f64 = (assign3820_e3345).exp();
            let assign3820_e3347: f64 = (1.0 - assign3820_e3346);
            let assign3820_e3348: f64 = (assign3820_e3340 * assign3820_e3347);
            let assign3820_e3349: f64 = (1.0 + assign3820_e3348);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3820_e3349, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3830_e3356: f64 = (locals.var_temp).max(1e-15);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign3830_e3356, if locals.var_temp >= 1e-15 { locals.var_temp_dn4 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn6 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn7 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn8 } else { 0.0 }, if locals.var_temp >= 1e-15 { locals.var_temp_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3840_e3363: f64 = (p.p240 * locals.var_we_edge);
            let assign3840_e3366: f64 = (locals.var_temp * locals.var_le);
            let assign3840_e3367: f64 = (assign3840_e3363 / assign3840_e3366);
            let assign3840_e3371: f64 = (p.p379 * locals.var_iwe);
            let assign3840_e3372: f64 = (1.0 + assign3840_e3371);
            let assign3840_e3373: f64 = (assign3840_e3367 * assign3840_e3372);
            (locals.var_betnedge_t, locals.var_betnedge_t_dn4, locals.var_betnedge_t_dn6, locals.var_betnedge_t_dn7, locals.var_betnedge_t_dn8, locals.var_betnedge_t_dn9, ) = (assign3840_e3373, ((-((assign3840_e3363 * (locals.var_temp_dn4 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn6 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn7 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn8 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (locals.var_temp_dn9 * locals.var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3850_e3381: f64 = (p.p381 * locals.var_ile);
            let assign3850_e3382: f64 = (p.p380 + assign3850_e3381);
            let assign3850_e3385: f64 = (p.p382 * locals.var_iwe);
            let assign3850_e3386: f64 = (assign3850_e3382 + assign3850_e3385);
            let assign3850_e3389: f64 = (p.p383 * locals.var_ile);
            let assign3850_e3391: f64 = (assign3850_e3389 * locals.var_iwe);
            let assign3850_e3392: f64 = (assign3850_e3386 + assign3850_e3391);
            locals.var_stbetedge_i = assign3850_e3392;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3860_e3399: f64 = (locals.var_wecv * locals.var_lecv);
            locals.var_areaq_i = assign3860_e3399;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3870_e3407: f64 = (p.p393 * locals.var_lphy);
            let assign3870_e3408: f64 = (p.p392 + assign3870_e3407);
            (locals.var_cgbov_p, locals.var_cgbov_p_dn4, locals.var_cgbov_p_dn6, locals.var_cgbov_p_dn7, locals.var_cgbov_p_dn8, locals.var_cgbov_p_dn9, ) = (assign3870_e3408, (p.p393 * locals.var_lphy_dn4), (p.p393 * locals.var_lphy_dn6), (p.p393 * locals.var_lphy_dn7), (p.p393 * locals.var_lphy_dn8), (p.p393 * locals.var_lphy_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3880_e3415: f64 = (locals.var_cgbov_p).max(0.0);
            (locals.var_cgbov_i, locals.var_cgbov_i_dn4, locals.var_cgbov_i_dn6, locals.var_cgbov_i_dn7, locals.var_cgbov_i_dn8, locals.var_cgbov_i_dn9, ) = (assign3880_e3415, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn4 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn6 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn7 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn8 } else { 0.0 }, if locals.var_cgbov_p >= 0.0 { locals.var_cgbov_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign3890_e3422: f64 = (p.p394 * 1000000.0);
            locals.var_nsdac_i = assign3890_e3422;
        }

        if (locals.var_guard83 == 0.0) {
            let assign3900_e3429: f64 = (p.p395 * locals.var_wecv);
            let assign3900_e3431: f64 = (assign3900_e3429 / locals.var_wen);
            locals.var_fif_i = assign3900_e3431;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_fsceac_i = p.p396;
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

        let assign4030_e3496: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard98 = assign4030_e3496;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbaco_i = p.p207;
        }

        let assign4050_e3505: f64 = if param_given[397] { 1.0 } else { 0.0 };
        let assign4050_e3507: f64 = if assign4050_e3505 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard99 = assign4050_e3507;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard99 != 0.0)) {
            locals.var_vfbaco_i = p.p397;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbacl_i = p.p208;
        }

        let assign4080_e3525: f64 = if param_given[398] { 1.0 } else { 0.0 };
        let assign4080_e3527: f64 = if assign4080_e3525 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard100 = assign4080_e3527;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard100 != 0.0)) {
            locals.var_vfbacl_i = p.p398;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbaclexp_i = p.p209;
        }

        let assign4110_e3545: f64 = if param_given[399] { 1.0 } else { 0.0 };
        let assign4110_e3547: f64 = if assign4110_e3545 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard101 = assign4110_e3547;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard101 != 0.0)) {
            locals.var_vfbaclexp_i = p.p399;
        }

    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbacw_i = p.p212;
        }

        let assign4140_e3565: f64 = if param_given[402] { 1.0 } else { 0.0 };
        let assign4140_e3567: f64 = if assign4140_e3565 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard102 = assign4140_e3567;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard102 != 0.0)) {
            locals.var_vfbacw_i = p.p402;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbaclw_i = p.p213;
        }

        let assign4170_e3585: f64 = if param_given[403] { 1.0 } else { 0.0 };
        let assign4170_e3587: f64 = if assign4170_e3585 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard103 = assign4170_e3587;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard103 != 0.0)) {
            locals.var_vfbaclw_i = p.p403;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbacl2_i = p.p210;
        }

        let assign4200_e3605: f64 = if param_given[400] { 1.0 } else { 0.0 };
        let assign4200_e3607: f64 = if assign4200_e3605 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard104 = assign4200_e3607;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard104 != 0.0)) {
            locals.var_vfbacl2_i = p.p400;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbaclexp2_i = p.p211;
        }

        let assign4230_e3625: f64 = if param_given[401] { 1.0 } else { 0.0 };
        let assign4230_e3627: f64 = if assign4230_e3625 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard105 = assign4230_e3627;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard105 != 0.0)) {
            locals.var_vfbaclexp2_i = p.p401;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4250_e3644: f64 = (locals.var_ile).powf(locals.var_vfbaclexp_i);
            let assign4250_e3645: f64 = (locals.var_vfbacl_i * assign4250_e3644);
            let assign4250_e3650: f64 = (locals.var_ile).powf(locals.var_vfbaclexp2_i);
            let assign4250_e3651: f64 = (locals.var_vfbacl2_i * assign4250_e3650);
            let assign4250_e3652: f64 = (1.0 + assign4250_e3651);
            let assign4250_e3653: f64 = (assign4250_e3645 / assign4250_e3652);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign4250_e3653, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4260_e3662: f64 = (locals.var_vfbaco_i + locals.var_temp);
            let assign4260_e3665: f64 = (locals.var_vfbacw_i * locals.var_iwe);
            let assign4260_e3666: f64 = (assign4260_e3662 + assign4260_e3665);
            let assign4260_e3669: f64 = (locals.var_vfbaclw_i * locals.var_iae);
            let assign4260_e3670: f64 = (assign4260_e3666 + assign4260_e3669);
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (assign4260_e3670, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfbbaco_i = p.p214;
        }

        let assign4280_e3681: f64 = if param_given[404] { 1.0 } else { 0.0 };
        let assign4280_e3683: f64 = if assign4280_e3681 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard106 = assign4280_e3683;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard106 != 0.0)) {
            locals.var_vfbbaco_i = p.p404;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_vfblbaco_i = p.p215;
        }

        let assign4310_e3701: f64 = if param_given[405] { 1.0 } else { 0.0 };
        let assign4310_e3703: f64 = if assign4310_e3701 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard107 = assign4310_e3703;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard107 != 0.0)) {
            locals.var_vfblbaco_i = p.p405;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4330_e3720: f64 = (locals.var_vfblbaco_i * locals.var_tox2_i);
            let assign4330_e3722: f64 = (assign4330_e3720 / locals.var_tox1_i);
            let assign4330_e3724: f64 = (assign4330_e3722 * locals.var_temp);
            let assign4330_e3725: f64 = (locals.var_vfbbaco_i + assign4330_e3724);
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (assign4330_e3725, (assign4330_e3722 * locals.var_temp_dn4), (assign4330_e3722 * locals.var_temp_dn6), (assign4330_e3722 * locals.var_temp_dn7), (assign4330_e3722 * locals.var_temp_dn8), (assign4330_e3722 * locals.var_temp_dn9), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_psceacl_i = p.p224;
        }

        let assign4350_e3736: f64 = if param_given[406] { 1.0 } else { 0.0 };
        let assign4350_e3738: f64 = if assign4350_e3736 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard108 = assign4350_e3738;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard108 != 0.0)) {
            locals.var_psceacl_i = p.p406;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_psceaclexp_i = p.p225;
        }

        let assign4380_e3756: f64 = if param_given[407] { 1.0 } else { 0.0 };
        let assign4380_e3758: f64 = if assign4380_e3756 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard109 = assign4380_e3758;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard109 != 0.0)) {
            locals.var_psceaclexp_i = p.p407;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_psceacw_i = p.p226;
        }

        let assign4410_e3776: f64 = if param_given[408] { 1.0 } else { 0.0 };
        let assign4410_e3778: f64 = if assign4410_e3776 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard110 = assign4410_e3778;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard110 != 0.0)) {
            locals.var_psceacw_i = p.p408;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4430_e3794: f64 = (locals.var_psceacl_i * 2.0);
            let assign4430_e3797: f64 = (locals.var_lambda_le).powf(locals.var_psceaclexp_i);
            let assign4430_e3798: f64 = (assign4430_e3794 * assign4430_e3797);
            let assign4430_e3802: f64 = (locals.var_psceacw_i * locals.var_iwe);
            let assign4430_e3803: f64 = (1.0 + assign4430_e3802);
            let assign4430_e3804: f64 = (assign4430_e3798 * assign4430_e3803);
            locals.var_psceac_p = assign4430_e3804;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4440_e3813: f64 = (locals.var_psceac_p).max(0.0);
            let assign4440_e3815: f64 = (assign4440_e3813).min(5.0);
            locals.var_psceac1_i = assign4440_e3815;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4450_e3824: f64 = (p.p227 * locals.var_psceac1_i);
            let assign4450_e3826: f64 = (assign4450_e3824 * locals.var_tox2_i);
            let assign4450_e3828: f64 = (assign4450_e3826 / locals.var_tox1_i);
            locals.var_psceac2_i = assign4450_e3828;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_cfacl_i = p.p231;
        }

        let assign4470_e3839: f64 = if param_given[409] { 1.0 } else { 0.0 };
        let assign4470_e3841: f64 = if assign4470_e3839 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard111 = assign4470_e3841;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard111 != 0.0)) {
            locals.var_cfacl_i = p.p409;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_cfaclexp_i = p.p232;
        }

        let assign4500_e3859: f64 = if param_given[410] { 1.0 } else { 0.0 };
        let assign4500_e3861: f64 = if assign4500_e3859 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard112 = assign4500_e3861;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard112 != 0.0)) {
            locals.var_cfaclexp_i = p.p410;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_cfacw_i = p.p233;
        }

        let assign4530_e3879: f64 = if param_given[411] { 1.0 } else { 0.0 };
        let assign4530_e3881: f64 = if assign4530_e3879 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign4530_e3881;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard113 != 0.0)) {
            locals.var_cfacw_i = p.p411;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4550_e3897: f64 = (locals.var_lambda_le).powf(locals.var_cfaclexp_i);
            let assign4550_e3901: f64 = (locals.var_cfacw_i * locals.var_iwe);
            let assign4550_e3902: f64 = (1.0 + assign4550_e3901);
            let assign4550_e3903: f64 = (assign4550_e3897 * assign4550_e3902);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign4550_e3903, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4560_e3912: f64 = (locals.var_cfacl_i * locals.var_temp);
            (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9, ) = (assign4560_e3912, (locals.var_cfacl_i * locals.var_temp_dn4), (locals.var_cfacl_i * locals.var_temp_dn6), (locals.var_cfacl_i * locals.var_temp_dn7), (locals.var_cfacl_i * locals.var_temp_dn8), (locals.var_cfacl_i * locals.var_temp_dn9), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4570_e3921: f64 = (locals.var_cfac_p).max(0.0);
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (assign4570_e3921, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 }, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4580_e3930: f64 = (p.p234 * locals.var_cfac1_t);
            let assign4580_e3932: f64 = (assign4580_e3930 * locals.var_tox2_i);
            let assign4580_e3934: f64 = (assign4580_e3932 / locals.var_tox1_i);
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (assign4580_e3934, (((p.p234 * locals.var_cfac1_t_dn4) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn6) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn7) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn8) * locals.var_tox2_i) / locals.var_tox1_i), (((p.p234 * locals.var_cfac1_t_dn9) * locals.var_tox2_i) / locals.var_tox1_i), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesataco_i = p.p289;
        }

        let assign4600_e3945: f64 = if param_given[412] { 1.0 } else { 0.0 };
        let assign4600_e3947: f64 = if assign4600_e3945 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign4600_e3947;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard114 != 0.0)) {
            locals.var_thesataco_i = p.p412;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesatacl_i = p.p290;
        }

        let assign4630_e3965: f64 = if param_given[413] { 1.0 } else { 0.0 };
        let assign4630_e3967: f64 = if assign4630_e3965 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign4630_e3967;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard115 != 0.0)) {
            locals.var_thesatacl_i = p.p413;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesataclexp_i = p.p291;
        }

        let assign4660_e3985: f64 = if param_given[414] { 1.0 } else { 0.0 };
        let assign4660_e3987: f64 = if assign4660_e3985 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign4660_e3987;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard116 != 0.0)) {
            locals.var_thesataclexp_i = p.p414;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesatacw_i = p.p292;
        }

        let assign4690_e4005: f64 = if param_given[415] { 1.0 } else { 0.0 };
        let assign4690_e4007: f64 = if assign4690_e4005 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign4690_e4007;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard117 != 0.0)) {
            locals.var_thesatacw_i = p.p415;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_thesataclw_i = p.p293;
        }

        let assign4720_e4025: f64 = if param_given[416] { 1.0 } else { 0.0 };
        let assign4720_e4027: f64 = if assign4720_e4025 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign4720_e4027;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard118 != 0.0)) {
            locals.var_thesataclw_i = p.p416;
        }

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
            (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9, ) = (assign4740_e4061, (((locals.var_ge_dn4 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn6 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn7 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn8 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((locals.var_ge_dn9 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4750_e4070: f64 = (locals.var_thesatac_p).max(0.0);
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (assign4750_e4070, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 }, );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axaco_i = p.p300;
        }

        let assign4770_e4081: f64 = if param_given[417] { 1.0 } else { 0.0 };
        let assign4770_e4083: f64 = if assign4770_e4081 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign4770_e4083;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard119 != 0.0)) {
            locals.var_axaco_i = p.p417;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axacl_i = p.p301;
        }

        let assign4800_e4101: f64 = if param_given[418] { 1.0 } else { 0.0 };
        let assign4800_e4103: f64 = if assign4800_e4101 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign4800_e4103;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard120 != 0.0)) {
            locals.var_axacl_i = p.p418;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axaclexp_i = p.p302;
        }

        let assign4830_e4121: f64 = if param_given[419] { 1.0 } else { 0.0 };
        let assign4830_e4123: f64 = if assign4830_e4121 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign4830_e4123;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard121 != 0.0)) {
            locals.var_axaclexp_i = p.p419;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axacl2_i = p.p303;
        }

        let assign4860_e4141: f64 = if param_given[420] { 1.0 } else { 0.0 };
        let assign4860_e4143: f64 = if assign4860_e4141 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign4860_e4143;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard122 != 0.0)) {
            locals.var_axacl2_i = p.p420;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_axaclexp2_i = p.p304;
        }

        let assign4890_e4161: f64 = if param_given[421] { 1.0 } else { 0.0 };
        let assign4890_e4163: f64 = if assign4890_e4161 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign4890_e4163;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard123 != 0.0)) {
            locals.var_axaclexp2_i = p.p421;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4910_e4182: f64 = (locals.var_ile).powf(locals.var_axaclexp_i);
            let assign4910_e4183: f64 = (locals.var_axacl_i * assign4910_e4182);
            let assign4910_e4188: f64 = (locals.var_ile).powf(locals.var_axaclexp2_i);
            let assign4910_e4189: f64 = (locals.var_axacl2_i * assign4910_e4188);
            let assign4910_e4190: f64 = (1.0 + assign4910_e4189);
            let assign4910_e4191: f64 = (assign4910_e4183 / assign4910_e4190);
            let assign4910_e4192: f64 = (1.0 + assign4910_e4191);
            let assign4910_e4193: f64 = (locals.var_axaco_i / assign4910_e4192);
            locals.var_axac_p = assign4910_e4193;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign4920_e4202: f64 = (locals.var_axac_p).max(1.0);
            let assign4920_e4204: f64 = (assign4920_e4202).min(16.0);
            locals.var_axac_i = assign4920_e4204;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpacl1_i = p.p305;
        }

        let assign4940_e4215: f64 = if param_given[422] { 1.0 } else { 0.0 };
        let assign4940_e4217: f64 = if assign4940_e4215 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign4940_e4217;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard124 != 0.0)) {
            locals.var_alpacl1_i = p.p422;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpaclexp_i = p.p306;
        }

        let assign4970_e4235: f64 = if param_given[423] { 1.0 } else { 0.0 };
        let assign4970_e4237: f64 = if assign4970_e4235 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign4970_e4237;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard125 != 0.0)) {
            locals.var_alpaclexp_i = p.p423;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpacl2_i = p.p307;
        }

        let assign5000_e4255: f64 = if param_given[424] { 1.0 } else { 0.0 };
        let assign5000_e4257: f64 = if assign5000_e4255 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign5000_e4257;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard126 != 0.0)) {
            locals.var_alpacl2_i = p.p424;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpaclexp2_i = p.p308;
        }

        let assign5030_e4275: f64 = if param_given[425] { 1.0 } else { 0.0 };
        let assign5030_e4277: f64 = if assign5030_e4275 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign5030_e4277;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard127 != 0.0)) {
            locals.var_alpaclexp2_i = p.p425;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            locals.var_alpacw_i = p.p309;
        }

        let assign5060_e4295: f64 = if param_given[426] { 1.0 } else { 0.0 };
        let assign5060_e4297: f64 = if assign5060_e4295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign5060_e4297;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) && (locals.var_guard128 != 0.0)) {
            locals.var_alpacw_i = p.p426;
        }

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
            locals.var_alpac_p = assign5080_e4329;
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard98 != 0.0)) {
            let assign5090_e4338: f64 = (locals.var_alpac_p).max(0.0);
            locals.var_alpac_i = assign5090_e4338;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5100_e4345: f64 = (3.45313e-11 / locals.var_tox1_i);
            let assign5100_e4347: f64 = (assign5100_e4345 * locals.var_wecv);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5100_e4347, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5110_e4354: f64 = (locals.var_temp * p.p427);
            (locals.var_cov_i, locals.var_cov_i_dn4, locals.var_cov_i_dn6, locals.var_cov_i_dn7, locals.var_cov_i_dn8, locals.var_cov_i_dn9, ) = (assign5110_e4354, (locals.var_temp_dn4 * p.p427), (locals.var_temp_dn6 * p.p427), (locals.var_temp_dn7 * p.p427), (locals.var_temp_dn8 * p.p427), (locals.var_temp_dn9 * p.p427), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5120_e4361: f64 = (locals.var_temp * p.p428);
            (locals.var_covd_i, locals.var_covd_i_dn4, locals.var_covd_i_dn6, locals.var_covd_i_dn7, locals.var_covd_i_dn8, locals.var_covd_i_dn9, ) = (assign5120_e4361, (locals.var_temp_dn4 * p.p428), (locals.var_temp_dn6 * p.p428), (locals.var_temp_dn7 * p.p428), (locals.var_temp_dn8 * p.p428), (locals.var_temp_dn9 * p.p428), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5130_e4370: f64 = (p.p430 * locals.var_wen);
            let assign5130_e4372: f64 = (assign5130_e4370 / locals.var_wecv);
            let assign5130_e4373: f64 = (1.0 + assign5130_e4372);
            let assign5130_e4375: f64 = (assign5130_e4373).max(0.001);
            let assign5130_e4376: f64 = (p.p429 / assign5130_e4375);
            locals.var_covdl_i = assign5130_e4376;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_covdlb_i = p.p431;
            locals.var_dvfbov_i = p.p432;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5160_e4394: f64 = (p.p435 * locals.var_wphy);
            let assign5160_e4395: f64 = (p.p433 + assign5160_e4394);
            (locals.var_cfr_p, locals.var_cfr_p_dn4, locals.var_cfr_p_dn6, locals.var_cfr_p_dn7, locals.var_cfr_p_dn8, locals.var_cfr_p_dn9, ) = (assign5160_e4395, (p.p435 * locals.var_wphy_dn4), (p.p435 * locals.var_wphy_dn6), (p.p435 * locals.var_wphy_dn7), (p.p435 * locals.var_wphy_dn8), (p.p435 * locals.var_wphy_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5170_e4402: f64 = (locals.var_cfr_p).max(0.0);
            (locals.var_cfr_i, locals.var_cfr_i_dn4, locals.var_cfr_i_dn6, locals.var_cfr_i_dn7, locals.var_cfr_i_dn8, locals.var_cfr_i_dn9, ) = (assign5170_e4402, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn4 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn6 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn7 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn8 } else { 0.0 }, if locals.var_cfr_p >= 0.0 { locals.var_cfr_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5180_e4410: f64 = (p.p436 * locals.var_wphy);
            let assign5180_e4411: f64 = (p.p434 + assign5180_e4410);
            (locals.var_cfrd_p, locals.var_cfrd_p_dn4, locals.var_cfrd_p_dn6, locals.var_cfrd_p_dn7, locals.var_cfrd_p_dn8, locals.var_cfrd_p_dn9, ) = (assign5180_e4411, (p.p436 * locals.var_wphy_dn4), (p.p436 * locals.var_wphy_dn6), (p.p436 * locals.var_wphy_dn7), (p.p436 * locals.var_wphy_dn8), (p.p436 * locals.var_wphy_dn9), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5190_e4418: f64 = (locals.var_cfrd_p).max(0.0);
            (locals.var_cfrd_i, locals.var_cfrd_i_dn4, locals.var_cfrd_i_dn6, locals.var_cfrd_i_dn7, locals.var_cfrd_i_dn8, locals.var_cfrd_i_dn9, ) = (assign5190_e4418, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn4 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn6 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn7 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn8 } else { 0.0 }, if locals.var_cfrd_p >= 0.0 { locals.var_cfrd_p_dn9 } else { 0.0 }, );
        }

    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard83 == 0.0) {
            let assign5200_e4425: f64 = (p.p437 * locals.var_epsch);
            let assign5200_e4427: f64 = (assign5200_e4425 * locals.var_tsi_i);
            let assign5200_e4429: f64 = (assign5200_e4427 * locals.var_we);
            let assign5200_e4431: f64 = (assign5200_e4429 / locals.var_le);
            locals.var_csd_i = assign5200_e4431;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_csdbp_i = p.p438;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5220_e4444: f64 = (p.p440 * locals.var_lphy);
            let assign5220_e4445: f64 = (1.0 + assign5220_e4444);
            let assign5220_e4448: f64 = (p.p441 * locals.var_wphy);
            let assign5220_e4449: f64 = (assign5220_e4445 + assign5220_e4448);
            let assign5220_e4452: f64 = (p.p442 * locals.var_lphy);
            let assign5220_e4454: f64 = (assign5220_e4452 * locals.var_wphy);
            let assign5220_e4455: f64 = (assign5220_e4449 + assign5220_e4454);
            let assign5220_e4457: f64 = (assign5220_e4455).max(1e-10);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5220_e4457, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn4) + (p.p441 * locals.var_wphy_dn4)) + (((p.p442 * locals.var_lphy_dn4) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn4))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn6) + (p.p441 * locals.var_wphy_dn6)) + (((p.p442 * locals.var_lphy_dn6) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn6))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn7) + (p.p441 * locals.var_wphy_dn7)) + (((p.p442 * locals.var_lphy_dn7) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn7))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn8) + (p.p441 * locals.var_wphy_dn8)) + (((p.p442 * locals.var_lphy_dn8) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn8))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * locals.var_lphy_dn9) + (p.p441 * locals.var_wphy_dn9)) + (((p.p442 * locals.var_lphy_dn9) * locals.var_wphy) + (assign5220_e4452 * locals.var_wphy_dn9))) } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign5240_e4471: f64 = if ((p.p29 > 1.0) && (p.p28 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard129 = assign5240_e4471;

        if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
            let assign5250_e4478: f64 = (p.p28 + p.p20);
            let assign5250_e4479: f64 = (-assign5250_e4478);
            let assign5250_e4481: f64 = (assign5250_e4479 / p.p445);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign5250_e4481, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign5260_e4485: f64 = (locals.var_temp2).abs();
        let assign5260_e4487: f64 = if assign5260_e4485 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign5260_e4487;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) && (locals.var_guard130 != 0.0)) {
            let assign5270_e4495: f64 = (locals.var_temp2).exp();
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign5270_e4495, (assign5270_e4495 * locals.var_temp2_dn4), (assign5270_e4495 * locals.var_temp2_dn6), (assign5270_e4495 * locals.var_temp2_dn7), (assign5270_e4495 * locals.var_temp2_dn8), (assign5270_e4495 * locals.var_temp2_dn9), );
        }

        let assign5280_e4500: f64 = (-80.0);
        let assign5280_e4501: f64 = if locals.var_temp2 < assign5280_e4500 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign5280_e4501;

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
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign5290_e4536, (-((1.80485e-35 * (((-locals.var_temp2_dn4) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn4)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn4) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn6) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn6)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn6) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn7) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn7)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn7) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn8) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn8)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn8) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-locals.var_temp2_dn9) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-locals.var_temp2_dn9)) * assign5290_e4531) + (assign5290_e4523 * ((-locals.var_temp2_dn9) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), );
        }

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
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign5300_e4571, (5.54062e34 * ((locals.var_temp2_dn4 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn4) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn6 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn6) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn7 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn7) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn8 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn8) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_temp2_dn9 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * locals.var_temp2_dn9) * assign5300_e4566) + (assign5300_e4559 * (locals.var_temp2_dn9 * 0.3333333333333)))))), );
        }

        if ((locals.var_guard83 == 0.0) && (locals.var_guard129 != 0.0)) {
            let assign5310_e4580: f64 = (1.0 - locals.var_temp3);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign5310_e4580, (-locals.var_temp3_dn4), (-locals.var_temp3_dn6), (-locals.var_temp3_dn7), (-locals.var_temp3_dn8), (-locals.var_temp3_dn9), );
        }

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
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign5320_e4605, ((((((assign5320_e4589 * locals.var_temp3_dn4) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn4 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn4)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn4 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn4 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn4)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn6) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn6 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn6)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn6 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn6 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn6)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn7) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn7 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn7)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn7 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn7 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn7)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn8) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn8 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn8)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn8 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn8 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn8)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * locals.var_temp3_dn9) * assign5320_e4600) + (assign5320_e4591 * (locals.var_temp4_dn9 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((locals.var_temp3).powf(p.p29 - 1.0) * locals.var_temp3_dn9)) } } else { (assign5320_e4596 * (p.p29 * (locals.var_temp3_dn9 / locals.var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((locals.var_temp4_dn9 * locals.var_temp4) + (locals.var_temp4 * locals.var_temp4_dn9)))) / (assign5320_e4604 * assign5320_e4604)), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5330_e4613: f64 = (1.0 + locals.var_temp1);
            let assign5330_e4614: f64 = (locals.var_temp / assign5330_e4613);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5330_e4614, (((locals.var_temp_dn4 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn4)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn6 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn6)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn7 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn7)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn8 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn8)) / (assign5330_e4613 * assign5330_e4613)), (((locals.var_temp_dn9 * assign5330_e4613) - (locals.var_temp * locals.var_temp1_dn9)) / (assign5330_e4613 * assign5330_e4613)), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5340_e4621: f64 = (p.p439 / locals.var_temp);
            (locals.var_rth_p, locals.var_rth_p_dn4, locals.var_rth_p_dn6, locals.var_rth_p_dn7, locals.var_rth_p_dn8, locals.var_rth_p_dn9, ) = (assign5340_e4621, (-((p.p439 * locals.var_temp_dn4) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn6) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn7) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn8) / (locals.var_temp * locals.var_temp))), (-((p.p439 * locals.var_temp_dn9) / (locals.var_temp * locals.var_temp))), );
        }

        if (locals.var_guard83 == 0.0) {
            let assign5350_e4628: f64 = (locals.var_rth_p).max(1e-6);
            (locals.var_rth_t, locals.var_rth_t_dn4, locals.var_rth_t_dn6, locals.var_rth_t_dn7, locals.var_rth_t_dn8, locals.var_rth_t_dn9, ) = (assign5350_e4628, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn4 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn6 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn7 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn8 } else { 0.0 }, if locals.var_rth_p >= 1e-6 { locals.var_rth_p_dn9 } else { 0.0 }, );
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_strth_i = p.p443;
            locals.var_fnt_i = p.p447;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5400_e4659: f64 = (p.p448 * locals.var_betn_p);
            let assign5400_e4661: f64 = (assign5400_e4659 * locals.var_betn_p);
            let assign5400_e4663: f64 = (assign5400_e4661 * locals.var_iwe);
            let assign5400_e4665: f64 = (assign5400_e4663 * locals.var_iwe);
            let assign5400_e4669: f64 = (p.p449 - 2.0);
            let assign5400_e4670: f64 = (locals.var_ile).powf(assign5400_e4669);
            let assign5400_e4671: f64 = (assign5400_e4665 * assign5400_e4670);
            locals.var_fntexc_i = assign5400_e4671;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5410_e4678: f64 = (p.p450 * locals.var_iae);
            let assign5410_e4681: f64 = (p.p451 * locals.var_iwe);
            let assign5410_e4682: f64 = (assign5410_e4678 + assign5410_e4681);
            locals.var_nfa_p = assign5410_e4682;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5420_e4689: f64 = (locals.var_nfa_p).max(0.0);
            locals.var_nfa_i = assign5420_e4689;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5430_e4696: f64 = (p.p452 * locals.var_iae);
            locals.var_nfb_i = assign5430_e4696;
        }

        if (locals.var_guard83 == 0.0) {
            let assign5440_e4703: f64 = (p.p453 * locals.var_iae);
            locals.var_nfc_i = assign5440_e4703;
        }

        if (locals.var_guard83 == 0.0) {
            locals.var_nfe_i = p.p454;
            locals.var_nfeb_i = p.p455;
        }

        let assign5570_e4828: f64 = if ((((p.p457 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard133 = assign5570_e4828;

        let assign5580_e4831: f64 = if p.p457 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign5580_e4831;

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_tmpb = 0.0;
            locals.var_iloop = 0.0;
        }

        let mut assign5620_loop_guard: usize = 0;
        while {
            let assign5620_cond_e4868: f64 = (p.p29 - 0.5);
            let assign5620_cond_e4870: f64 = if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) && (locals.var_iloop < assign5620_cond_e4868)) { 1.0 } else { 0.0 };
            assign5620_cond_e4870 != 0.0
        } {
            assign5620_loop_guard += 1;
            assert!(assign5620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
                let assign5620_body0_e4882: f64 = (0.5 * p.p20);
                let assign5620_body0_e4883: f64 = (p.p26 + assign5620_body0_e4882);
                let assign5620_body0_e4887: f64 = (p.p28 + p.p20);
                let assign5620_body0_e4888: f64 = (locals.var_iloop * assign5620_body0_e4887);
                let assign5620_body0_e4889: f64 = (assign5620_body0_e4883 + assign5620_body0_e4888);
                let assign5620_body0_e4890: f64 = (1.0 / assign5620_body0_e4889);
                let assign5620_body0_e4891: f64 = (locals.var_tmpa + assign5620_body0_e4890);
                (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, ) = (assign5620_body0_e4891, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
                let assign5620_body1_e4905: f64 = (0.5 * p.p20);
                let assign5620_body1_e4906: f64 = (p.p27 + assign5620_body1_e4905);
                let assign5620_body1_e4910: f64 = (p.p28 + p.p20);
                let assign5620_body1_e4911: f64 = (locals.var_iloop * assign5620_body1_e4910);
                let assign5620_body1_e4912: f64 = (assign5620_body1_e4906 + assign5620_body1_e4911);
                let assign5620_body1_e4913: f64 = (1.0 / assign5620_body1_e4912);
                let assign5620_body1_e4914: f64 = (locals.var_tmpb + assign5620_body1_e4913);
                locals.var_tmpb = assign5620_body1_e4914;
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
                let assign5620_body2_e4925: f64 = (locals.var_iloop + 1.0);
                locals.var_iloop = assign5620_body2_e4925;
            }
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5630_e4936: f64 = (locals.var_tmpa / p.p29);
            (locals.var_invsa, locals.var_invsa_dn4, locals.var_invsa_dn6, locals.var_invsa_dn7, locals.var_invsa_dn8, locals.var_invsa_dn9, ) = (assign5630_e4936, (locals.var_tmpa_dn4 / p.p29), (locals.var_tmpa_dn6 / p.p29), (locals.var_tmpa_dn7 / p.p29), (locals.var_tmpa_dn8 / p.p29), (locals.var_tmpa_dn9 / p.p29), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5640_e4947: f64 = (locals.var_tmpb / p.p29);
            locals.var_invsb = assign5640_e4947;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5650_e4960: f64 = (0.5 * p.p20);
            let assign5650_e4961: f64 = (p.p458 + assign5650_e4960);
            let assign5650_e4962: f64 = (1.0 / assign5650_e4961);
            locals.var_invsaref = assign5650_e4962;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5660_e4975: f64 = (0.5 * p.p20);
            let assign5660_e4976: f64 = (p.p459 + assign5660_e4975);
            let assign5660_e4977: f64 = (1.0 / assign5660_e4976);
            locals.var_invsbref = assign5660_e4977;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5670_e4988: f64 = (p.p20 + locals.var_dellps);
            let assign5670_e4990: f64 = (assign5670_e4988).max(1e-9);
            locals.var_lx = assign5670_e4990;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5680_e5001: f64 = (locals.var_w_i + locals.var_delwod);
            let assign5680_e5003: f64 = (assign5680_e5001 + p.p460);
            let assign5680_e5005: f64 = (assign5680_e5003).max(1e-9);
            locals.var_wx = assign5680_e5005;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5690_e5017: f64 = (locals.var_lx).powf(p.p467);
            let assign5690_e5018: f64 = (1.0 / assign5690_e5017);
            locals.var_templ = assign5690_e5018;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5700_e5030: f64 = (locals.var_wx).powf(p.p468);
            let assign5700_e5031: f64 = (1.0 / assign5700_e5030);
            locals.var_tempw = assign5700_e5031;
        }

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
            (locals.var_kstressu0, locals.var_kstressu0_dn4, locals.var_kstressu0_dn6, locals.var_kstressu0_dn7, locals.var_kstressu0_dn8, locals.var_kstressu0_dn9, ) = (assign5710_e5062, (assign5710_e5054 * (p.p463 * locals.var_rt_dn4)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn6)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn7)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn8)), (assign5710_e5054 * (p.p463 * locals.var_rt_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5720_e5074: f64 = (locals.var_invsa + locals.var_invsb);
            let assign5720_e5075: f64 = (p.p461 * assign5720_e5074);
            let assign5720_e5077: f64 = (assign5720_e5075 / locals.var_kstressu0);
            (locals.var_rhobeta, locals.var_rhobeta_dn4, locals.var_rhobeta_dn6, locals.var_rhobeta_dn7, locals.var_rhobeta_dn8, locals.var_rhobeta_dn9, ) = (assign5720_e5077, ((((p.p461 * locals.var_invsa_dn4) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn4)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn6) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn6)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn7) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn7)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn8) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn8)) / (locals.var_kstressu0 * locals.var_kstressu0)), ((((p.p461 * locals.var_invsa_dn9) * locals.var_kstressu0) - (assign5720_e5075 * locals.var_kstressu0_dn9)) / (locals.var_kstressu0 * locals.var_kstressu0)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5730_e5089: f64 = (locals.var_invsaref + locals.var_invsbref);
            let assign5730_e5090: f64 = (p.p461 * assign5730_e5089);
            let assign5730_e5092: f64 = (assign5730_e5090 / locals.var_kstressu0);
            (locals.var_rhobetaref, locals.var_rhobetaref_dn4, locals.var_rhobetaref_dn6, locals.var_rhobetaref_dn7, locals.var_rhobetaref_dn8, locals.var_rhobetaref_dn9, ) = (assign5730_e5092, (-((assign5730_e5090 * locals.var_kstressu0_dn4) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn6) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn7) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn8) / (locals.var_kstressu0 * locals.var_kstressu0))), (-((assign5730_e5090 * locals.var_kstressu0_dn9) / (locals.var_kstressu0 * locals.var_kstressu0))), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5740_e5104: f64 = (locals.var_lx).powf(p.p473);
            let assign5740_e5105: f64 = (1.0 / assign5740_e5104);
            locals.var_templ = assign5740_e5105;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5750_e5117: f64 = (locals.var_wx).powf(p.p474);
            let assign5750_e5118: f64 = (1.0 / assign5750_e5117);
            locals.var_tempw = assign5750_e5118;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5760_e5130: f64 = (p.p470 * locals.var_templ);
            let assign5760_e5131: f64 = (1.0 + assign5760_e5130);
            let assign5760_e5134: f64 = (p.p471 * locals.var_tempw);
            let assign5760_e5135: f64 = (assign5760_e5131 + assign5760_e5134);
            let assign5760_e5138: f64 = (p.p472 * locals.var_templ);
            let assign5760_e5140: f64 = (assign5760_e5138 * locals.var_tempw);
            let assign5760_e5141: f64 = (assign5760_e5135 + assign5760_e5140);
            let assign5760_e5143: f64 = (assign5760_e5141).max(1e-20);
            locals.var_kstressvth0 = assign5760_e5143;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5770_e5154: f64 = (locals.var_invsa + locals.var_invsb);
            let assign5770_e5156: f64 = (assign5770_e5154 - locals.var_invsaref);
            let assign5770_e5158: f64 = (assign5770_e5156 - locals.var_invsbref);
            (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9, ) = (assign5770_e5158, locals.var_invsa_dn4, locals.var_invsa_dn6, locals.var_invsa_dn7, locals.var_invsa_dn8, locals.var_invsa_dn9, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5780_e5170: f64 = (1.0 + locals.var_rhobeta);
            let assign5780_e5171: f64 = (locals.var_betn_p * assign5780_e5170);
            let assign5780_e5174: f64 = (1.0 + locals.var_rhobetaref);
            let assign5780_e5175: f64 = (assign5780_e5171 / assign5780_e5174);
            (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9, ) = (assign5780_e5175, (((((locals.var_betn_p_dn4 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn4)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn4)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn6 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn6)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn6)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn7 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn7)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn7)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn8 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn8)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn8)) / (assign5780_e5174 * assign5780_e5174)), (((((locals.var_betn_p_dn9 * assign5780_e5170) + (locals.var_betn_p * locals.var_rhobeta_dn9)) * assign5780_e5174) - (assign5780_e5171 * locals.var_rhobetaref_dn9)) / (assign5780_e5174 * assign5780_e5174)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5790_e5186: f64 = (locals.var_betn_p).max(1e-10);
            (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9, ) = (assign5790_e5186, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5800_e5197: f64 = (p.p250 * locals.var_betn1_t);
            (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9, ) = (assign5800_e5197, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9), );
        }

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
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5810_e5224, (((((locals.var_rhobeta_dn4 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn4))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn4 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn4))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn6 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn6))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn6 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn6))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn7 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn7))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn7 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn7))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn8 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn8))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn8 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn8))))) / (assign5810_e5223 * assign5810_e5223)), (((((locals.var_rhobeta_dn9 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * locals.var_rhobetaref_dn9))) * assign5810_e5223) - (assign5810_e5214 * ((locals.var_rhobetaref_dn9 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * locals.var_rhobeta_dn9))))) / (assign5810_e5223 * assign5810_e5223)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5820_e5235: f64 = (locals.var_thesat_p * locals.var_temp);
            (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9, ) = (assign5820_e5235, ((locals.var_thesat_p_dn4 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn4)), ((locals.var_thesat_p_dn6 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn6)), ((locals.var_thesat_p_dn7 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn7)), ((locals.var_thesat_p_dn8 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn8)), ((locals.var_thesat_p_dn9 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5830_e5246: f64 = (locals.var_thesat_p).max(0.0);
            (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, ) = (assign5830_e5246, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5840_e5257: f64 = (locals.var_thesatac_p * locals.var_temp);
            (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9, ) = (assign5840_e5257, ((locals.var_thesatac_p_dn4 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn4)), ((locals.var_thesatac_p_dn6 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn6)), ((locals.var_thesatac_p_dn7 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn7)), ((locals.var_thesatac_p_dn8 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn8)), ((locals.var_thesatac_p_dn9 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5850_e5268: f64 = (locals.var_thesatac_p).max(0.0);
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (assign5850_e5268, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5860_e5279: f64 = (p.p469 * locals.var_temp0__blk79);
            let assign5860_e5281: f64 = (assign5860_e5279 / locals.var_kstressvth0);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5860_e5281, ((p.p469 * locals.var_temp0__blk79_dn4) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn6) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn7) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn8) / locals.var_kstressvth0), ((p.p469 * locals.var_temp0__blk79_dn9) / locals.var_kstressvth0), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5870_e5292: f64 = (locals.var_vfb1_t + locals.var_temp);
            (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, ) = (assign5870_e5292, (locals.var_vfb1_t_dn4 + locals.var_temp_dn4), (locals.var_vfb1_t_dn6 + locals.var_temp_dn6), (locals.var_vfb1_t_dn7 + locals.var_temp_dn7), (locals.var_vfb1_t_dn8 + locals.var_temp_dn8), (locals.var_vfb1_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5880_e5303: f64 = (locals.var_vfb2_t + locals.var_temp);
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (assign5880_e5303, (locals.var_vfb2_t_dn4 + locals.var_temp_dn4), (locals.var_vfb2_t_dn6 + locals.var_temp_dn6), (locals.var_vfb2_t_dn7 + locals.var_temp_dn7), (locals.var_vfb2_t_dn8 + locals.var_temp_dn8), (locals.var_vfb2_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5890_e5314: f64 = (locals.var_vfbac1_t + locals.var_temp);
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (assign5890_e5314, (locals.var_vfbac1_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac1_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac1_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac1_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac1_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5900_e5325: f64 = (locals.var_vfbac2_t + locals.var_temp);
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (assign5900_e5325, (locals.var_vfbac2_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac2_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac2_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac2_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac2_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5910_e5336: f64 = (p.p475 * locals.var_temp0__blk79);
            let assign5910_e5339: f64 = (locals.var_kstressvth0).powf(p.p476);
            let assign5910_e5340: f64 = (assign5910_e5336 / assign5910_e5339);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5910_e5340, ((p.p475 * locals.var_temp0__blk79_dn4) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn6) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn7) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn8) / assign5910_e5339), ((p.p475 * locals.var_temp0__blk79_dn9) / assign5910_e5339), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5920_e5351: f64 = (locals.var_cf_p + locals.var_temp);
            (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9, ) = (assign5920_e5351, (locals.var_cf_p_dn4 + locals.var_temp_dn4), (locals.var_cf_p_dn6 + locals.var_temp_dn6), (locals.var_cf_p_dn7 + locals.var_temp_dn7), (locals.var_cf_p_dn8 + locals.var_temp_dn8), (locals.var_cf_p_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5930_e5362: f64 = (locals.var_cf_p).max(0.0);
            (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, ) = (assign5930_e5362, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5940_e5373: f64 = (locals.var_cfac_p + locals.var_temp);
            (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9, ) = (assign5940_e5373, (locals.var_cfac_p_dn4 + locals.var_temp_dn4), (locals.var_cfac_p_dn6 + locals.var_temp_dn6), (locals.var_cfac_p_dn7 + locals.var_temp_dn7), (locals.var_cfac_p_dn8 + locals.var_temp_dn8), (locals.var_cfac_p_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5950_e5384: f64 = (locals.var_cfac_p).max(0.0);
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (assign5950_e5384, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5960_e5395: f64 = (p.p234 * locals.var_tox2_i);
            let assign5960_e5397: f64 = (assign5960_e5395 / locals.var_tox1_i);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign5960_e5397, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5970_e5408: f64 = (locals.var_cf1_t * locals.var_temp);
            (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, ) = (assign5970_e5408, ((locals.var_cf1_t_dn4 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn4)), ((locals.var_cf1_t_dn6 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn6)), ((locals.var_cf1_t_dn7 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn7)), ((locals.var_cf1_t_dn8 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn8)), ((locals.var_cf1_t_dn9 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 != 0.0)) {
            let assign5980_e5419: f64 = (locals.var_cfac1_t * locals.var_temp);
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (assign5980_e5419, ((locals.var_cfac1_t_dn4 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn4)), ((locals.var_cfac1_t_dn6 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn6)), ((locals.var_cfac1_t_dn7 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn7)), ((locals.var_cfac1_t_dn8 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn8)), ((locals.var_cfac1_t_dn9 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            locals.var_iloop = 0.0;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6010_e5450: f64 = (-1.0);
            let assign6010_e5452: f64 = (assign6010_e5450 / p.p478);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6010_e5452, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

    }

    pub(super) fn stamp_transient_block_6(
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
            if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard135 != 0.0)) {
                let assign6020_body1_e5497: f64 = (0.5 * p.p20);
                let assign6020_body1_e5498: f64 = (p.p26 + assign6020_body1_e5497);
                let assign6020_body1_e5502: f64 = (p.p28 + p.p20);
                let assign6020_body1_e5503: f64 = (locals.var_iloop * assign6020_body1_e5502);
                let assign6020_body1_e5504: f64 = (assign6020_body1_e5498 + assign6020_body1_e5503);
                let assign6020_body1_e5505: f64 = (-assign6020_body1_e5504);
                let assign6020_body1_e5507: f64 = (assign6020_body1_e5505 / p.p477);
                let assign6020_body1_e5508: f64 = (assign6020_body1_e5507).exp();
                (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6020_body1_e5508, 0.0, 0.0, 0.0, 0.0, 0.0, );
            }
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
                (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6020_body2_e5585, 0.0, 0.0, 0.0, 0.0, 0.0, );
            }
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
                (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign6020_body4_e5636, 0.0, 0.0, 0.0, 0.0, 0.0, );
            }
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
                (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign6020_body5_e5725, 0.0, 0.0, 0.0, 0.0, 0.0, );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
                let assign6020_body6_e5737: f64 = (1.0 - locals.var_temp1);
                let assign6020_body6_e5739: f64 = (-p.p478);
                let assign6020_body6_e5740: f64 = (assign6020_body6_e5737).powf(assign6020_body6_e5739);
                (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign6020_body6_e5740, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn4))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn4) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn6))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn6) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn7))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn7) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn8))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn8) / assign6020_body6_e5737))) }, if 0.0 == 0.0 && ((assign6020_body6_e5739) as f64).is_finite() && ((assign6020_body6_e5739) as f64).fract() == 0.0 { if assign6020_body6_e5739 == 0.0 { 0.0 } else { (assign6020_body6_e5739 * ((assign6020_body6_e5737).powf(assign6020_body6_e5739 - 1.0) * (-locals.var_temp1_dn9))) } } else { (assign6020_body6_e5740 * (assign6020_body6_e5739 * ((-locals.var_temp1_dn9) / assign6020_body6_e5737))) }, );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
                let assign6020_body7_e5752: f64 = (1.0 - locals.var_temp2);
                let assign6020_body7_e5754: f64 = (-p.p478);
                let assign6020_body7_e5755: f64 = (assign6020_body7_e5752).powf(assign6020_body7_e5754);
                (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign6020_body7_e5755, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn4))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn4) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn6))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn6) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn7))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn7) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn8))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn8) / assign6020_body7_e5752))) }, if 0.0 == 0.0 && ((assign6020_body7_e5754) as f64).is_finite() && ((assign6020_body7_e5754) as f64).fract() == 0.0 { if assign6020_body7_e5754 == 0.0 { 0.0 } else { (assign6020_body7_e5754 * ((assign6020_body7_e5752).powf(assign6020_body7_e5754 - 1.0) * (-locals.var_temp2_dn9))) } } else { (assign6020_body7_e5755 * (assign6020_body7_e5754 * ((-locals.var_temp2_dn9) / assign6020_body7_e5752))) }, );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
                let assign6020_body8_e5769: f64 = (locals.var_temp3 + locals.var_temp4);
                let assign6020_body8_e5770: f64 = (0.5 * assign6020_body8_e5769);
                let assign6020_body8_e5772: f64 = (assign6020_body8_e5770).powf(locals.var_temp);
                let assign6020_body8_e5773: f64 = (locals.var_tmpa + assign6020_body8_e5772);
                (locals.var_tmpa, locals.var_tmpa_dn4, locals.var_tmpa_dn6, locals.var_tmpa_dn7, locals.var_tmpa_dn8, locals.var_tmpa_dn9, ) = (assign6020_body8_e5773, (locals.var_tmpa_dn4 + if locals.var_temp_dn4 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn4 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn6 + if locals.var_temp_dn6 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn6 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn7 + if locals.var_temp_dn7 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn7 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn8 + if locals.var_temp_dn8 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn8 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)) / assign6020_body8_e5770)))) }), (locals.var_tmpa_dn9 + if locals.var_temp_dn9 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6020_body8_e5770).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)))) } } else { (assign6020_body8_e5772 * ((locals.var_temp_dn9 * (assign6020_body8_e5770).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)) / assign6020_body8_e5770)))) }), );
            }
            if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
                let assign6020_body9_e5785: f64 = (locals.var_iloop + 1.0);
                locals.var_iloop = assign6020_body9_e5785;
            }
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6030_e5798: f64 = (locals.var_tmpa / p.p29);
            let assign6030_e5799: f64 = (1.0 - assign6030_e5798);
            (locals.var_str_g, locals.var_str_g_dn4, locals.var_str_g_dn6, locals.var_str_g_dn7, locals.var_str_g_dn8, locals.var_str_g_dn9, ) = (assign6030_e5799, (-(locals.var_tmpa_dn4 / p.p29)), (-(locals.var_tmpa_dn6 / p.p29)), (-(locals.var_tmpa_dn7 / p.p29)), (-(locals.var_tmpa_dn8 / p.p29)), (-(locals.var_tmpa_dn9 / p.p29)), );
        }

        let assign6040_e5805: f64 = (0.5 * p.p20);
        let assign6040_e5806: f64 = (p.p458 + assign6040_e5805);
        let assign6040_e5807: f64 = (-assign6040_e5806);
        let assign6040_e5809: f64 = (assign6040_e5807 / p.p477);
        let assign6040_e5811: f64 = (-80.0);
        let assign6040_e5812: f64 = if assign6040_e5809 > assign6040_e5811 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign6040_e5812;

        if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard137 != 0.0)) {
            let assign6050_e5825: f64 = (0.5 * p.p20);
            let assign6050_e5826: f64 = (p.p458 + assign6050_e5825);
            let assign6050_e5827: f64 = (-assign6050_e5826);
            let assign6050_e5829: f64 = (assign6050_e5827 / p.p477);
            let assign6050_e5830: f64 = (assign6050_e5829).exp();
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6050_e5830, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

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
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6060_e5889, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign6070_e5895: f64 = (0.5 * p.p20);
        let assign6070_e5896: f64 = (p.p459 + assign6070_e5895);
        let assign6070_e5897: f64 = (-assign6070_e5896);
        let assign6070_e5899: f64 = (assign6070_e5897 / p.p477);
        let assign6070_e5901: f64 = (-80.0);
        let assign6070_e5902: f64 = if assign6070_e5899 > assign6070_e5901 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign6070_e5902;

        if ((((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) && (locals.var_guard138 != 0.0)) {
            let assign6080_e5915: f64 = (0.5 * p.p20);
            let assign6080_e5916: f64 = (p.p459 + assign6080_e5915);
            let assign6080_e5917: f64 = (-assign6080_e5916);
            let assign6080_e5919: f64 = (assign6080_e5917 / p.p477);
            let assign6080_e5920: f64 = (assign6080_e5919).exp();
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign6080_e5920, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

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
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign6090_e5979, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6100_e5991: f64 = (1.0 - locals.var_temp1);
            let assign6100_e5993: f64 = (-p.p478);
            let assign6100_e5994: f64 = (assign6100_e5991).powf(assign6100_e5993);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign6100_e5994, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn4))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn4) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn6))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn6) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn7))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn7) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn8))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn8) / assign6100_e5991))) }, if 0.0 == 0.0 && ((assign6100_e5993) as f64).is_finite() && ((assign6100_e5993) as f64).fract() == 0.0 { if assign6100_e5993 == 0.0 { 0.0 } else { (assign6100_e5993 * ((assign6100_e5991).powf(assign6100_e5993 - 1.0) * (-locals.var_temp1_dn9))) } } else { (assign6100_e5994 * (assign6100_e5993 * ((-locals.var_temp1_dn9) / assign6100_e5991))) }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6110_e6006: f64 = (1.0 - locals.var_temp2);
            let assign6110_e6008: f64 = (-p.p478);
            let assign6110_e6009: f64 = (assign6110_e6006).powf(assign6110_e6008);
            (locals.var_temp4, locals.var_temp4_dn4, locals.var_temp4_dn6, locals.var_temp4_dn7, locals.var_temp4_dn8, locals.var_temp4_dn9, ) = (assign6110_e6009, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn4))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn4) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn6))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn6) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn7))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn7) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn8))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn8) / assign6110_e6006))) }, if 0.0 == 0.0 && ((assign6110_e6008) as f64).is_finite() && ((assign6110_e6008) as f64).fract() == 0.0 { if assign6110_e6008 == 0.0 { 0.0 } else { (assign6110_e6008 * ((assign6110_e6006).powf(assign6110_e6008 - 1.0) * (-locals.var_temp2_dn9))) } } else { (assign6110_e6009 * (assign6110_e6008 * ((-locals.var_temp2_dn9) / assign6110_e6006))) }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6120_e6023: f64 = (locals.var_temp3 + locals.var_temp4);
            let assign6120_e6024: f64 = (0.5 * assign6120_e6023);
            let assign6120_e6026: f64 = (assign6120_e6024).powf(locals.var_temp);
            let assign6120_e6027: f64 = (1.0 - assign6120_e6026);
            (locals.var_str_gref, locals.var_str_gref_dn4, locals.var_str_gref_dn6, locals.var_str_gref_dn7, locals.var_str_gref_dn8, locals.var_str_gref_dn9, ) = (assign6120_e6027, (-if locals.var_temp_dn4 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn4 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn4 + locals.var_temp4_dn4)) / assign6120_e6024)))) }), (-if locals.var_temp_dn6 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn6 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn6 + locals.var_temp4_dn6)) / assign6120_e6024)))) }), (-if locals.var_temp_dn7 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn7 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn7 + locals.var_temp4_dn7)) / assign6120_e6024)))) }), (-if locals.var_temp_dn8 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn8 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn8 + locals.var_temp4_dn8)) / assign6120_e6024)))) }), (-if locals.var_temp_dn9 == 0.0 && ((locals.var_temp) as f64).is_finite() && ((locals.var_temp) as f64).fract() == 0.0 { if locals.var_temp == 0.0 { 0.0 } else { (locals.var_temp * ((assign6120_e6024).powf(locals.var_temp - 1.0) * (0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)))) } } else { (assign6120_e6026 * ((locals.var_temp_dn9 * (assign6120_e6024).ln()) + (locals.var_temp * ((0.5 * (locals.var_temp3_dn9 + locals.var_temp4_dn9)) / assign6120_e6024)))) }), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6130_e6039: f64 = (locals.var_w_i + locals.var_delwod);
            let assign6130_e6041: f64 = (assign6130_e6039 + p.p460);
            let assign6130_e6043: f64 = (assign6130_e6041).max(1e-9);
            locals.var_wx = assign6130_e6043;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6140_e6058: f64 = (locals.var_rt - 1.0);
            let assign6140_e6059: f64 = (p.p483 * assign6140_e6058);
            let assign6140_e6060: f64 = (1.0 + assign6140_e6059);
            let assign6140_e6061: f64 = (p.p482 / assign6140_e6060);
            (locals.var_ruo, locals.var_ruo_dn4, locals.var_ruo_dn6, locals.var_ruo_dn7, locals.var_ruo_dn8, locals.var_ruo_dn9, ) = (assign6140_e6061, (-((p.p482 * (p.p483 * locals.var_rt_dn4)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn6)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn7)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn8)) / (assign6140_e6060 * assign6140_e6060))), (-((p.p482 * (p.p483 * locals.var_rt_dn9)) / (assign6140_e6060 * assign6140_e6060))), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6150_e6073: f64 = (locals.var_ruo * locals.var_str_g);
            (locals.var_rhobeta, locals.var_rhobeta_dn4, locals.var_rhobeta_dn6, locals.var_rhobeta_dn7, locals.var_rhobeta_dn8, locals.var_rhobeta_dn9, ) = (assign6150_e6073, ((locals.var_ruo_dn4 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn4)), ((locals.var_ruo_dn6 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn6)), ((locals.var_ruo_dn7 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn7)), ((locals.var_ruo_dn8 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn8)), ((locals.var_ruo_dn9 * locals.var_str_g) + (locals.var_ruo * locals.var_str_g_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6160_e6085: f64 = (locals.var_ruo * locals.var_str_gref);
            (locals.var_rhobetaref, locals.var_rhobetaref_dn4, locals.var_rhobetaref_dn6, locals.var_rhobetaref_dn7, locals.var_rhobetaref_dn8, locals.var_rhobetaref_dn9, ) = (assign6160_e6085, ((locals.var_ruo_dn4 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn4)), ((locals.var_ruo_dn6 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn6)), ((locals.var_ruo_dn7 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn7)), ((locals.var_ruo_dn8 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn8)), ((locals.var_ruo_dn9 * locals.var_str_gref) + (locals.var_ruo * locals.var_str_gref_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6170_e6097: f64 = (locals.var_str_g - locals.var_str_gref);
            (locals.var_temp0__blk79, locals.var_temp0__blk79_dn4, locals.var_temp0__blk79_dn6, locals.var_temp0__blk79_dn7, locals.var_temp0__blk79_dn8, locals.var_temp0__blk79_dn9, ) = (assign6170_e6097, (locals.var_str_g_dn4 - locals.var_str_gref_dn4), (locals.var_str_g_dn6 - locals.var_str_gref_dn6), (locals.var_str_g_dn7 - locals.var_str_gref_dn7), (locals.var_str_g_dn8 - locals.var_str_gref_dn8), (locals.var_str_g_dn9 - locals.var_str_gref_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6180_e6110: f64 = (p.p480 * locals.var_wx);
            let assign6180_e6112: f64 = (assign6180_e6110 / locals.var_wen);
            let assign6180_e6113: f64 = (1.0 + assign6180_e6112);
            let assign6180_e6115: f64 = (assign6180_e6113).max(1e-20);
            locals.var_kstressvth0 = assign6180_e6115;
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6190_e6128: f64 = (1.0 + locals.var_rhobeta);
            let assign6190_e6129: f64 = (locals.var_betn_p * assign6190_e6128);
            let assign6190_e6132: f64 = (1.0 + locals.var_rhobetaref);
            let assign6190_e6133: f64 = (assign6190_e6129 / assign6190_e6132);
            (locals.var_betn_p, locals.var_betn_p_dn4, locals.var_betn_p_dn6, locals.var_betn_p_dn7, locals.var_betn_p_dn8, locals.var_betn_p_dn9, ) = (assign6190_e6133, (((((locals.var_betn_p_dn4 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn4)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn4)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn6 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn6)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn6)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn7 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn7)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn7)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn8 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn8)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn8)) / (assign6190_e6132 * assign6190_e6132)), (((((locals.var_betn_p_dn9 * assign6190_e6128) + (locals.var_betn_p * locals.var_rhobeta_dn9)) * assign6190_e6132) - (assign6190_e6129 * locals.var_rhobetaref_dn9)) / (assign6190_e6132 * assign6190_e6132)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6200_e6145: f64 = (locals.var_betn_p).max(1e-10);
            (locals.var_betn1_t, locals.var_betn1_t_dn4, locals.var_betn1_t_dn6, locals.var_betn1_t_dn7, locals.var_betn1_t_dn8, locals.var_betn1_t_dn9, ) = (assign6200_e6145, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn4 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn6 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn7 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn8 } else { 0.0 }, if locals.var_betn_p >= 1e-10 { locals.var_betn_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6210_e6157: f64 = (p.p250 * locals.var_betn1_t);
            (locals.var_betn2_t, locals.var_betn2_t_dn4, locals.var_betn2_t_dn6, locals.var_betn2_t_dn7, locals.var_betn2_t_dn8, locals.var_betn2_t_dn9, ) = (assign6210_e6157, (p.p250 * locals.var_betn1_t_dn4), (p.p250 * locals.var_betn1_t_dn6), (p.p250 * locals.var_betn1_t_dn7), (p.p250 * locals.var_betn1_t_dn8), (p.p250 * locals.var_betn1_t_dn9), );
        }

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
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6220_e6185, (((((locals.var_rhobeta_dn4 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn4))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn4 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn4))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn6 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn6))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn6 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn6))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn7 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn7))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn7 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn7))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn8 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn8))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn8 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn8))))) / (assign6220_e6184 * assign6220_e6184)), (((((locals.var_rhobeta_dn9 * assign6220_e6174) + (assign6220_e6169 * (p.p484 * locals.var_rhobetaref_dn9))) * assign6220_e6184) - (assign6220_e6175 * ((locals.var_rhobetaref_dn9 * assign6220_e6183) + (assign6220_e6178 * (p.p484 * locals.var_rhobeta_dn9))))) / (assign6220_e6184 * assign6220_e6184)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6230_e6197: f64 = (locals.var_thesat_p * locals.var_temp);
            (locals.var_thesat_p, locals.var_thesat_p_dn4, locals.var_thesat_p_dn6, locals.var_thesat_p_dn7, locals.var_thesat_p_dn8, locals.var_thesat_p_dn9, ) = (assign6230_e6197, ((locals.var_thesat_p_dn4 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn4)), ((locals.var_thesat_p_dn6 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn6)), ((locals.var_thesat_p_dn7 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn7)), ((locals.var_thesat_p_dn8 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn8)), ((locals.var_thesat_p_dn9 * locals.var_temp) + (locals.var_thesat_p * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6240_e6209: f64 = (locals.var_thesat_p).max(0.0);
            (locals.var_thesat_t, locals.var_thesat_t_dn4, locals.var_thesat_t_dn6, locals.var_thesat_t_dn7, locals.var_thesat_t_dn8, locals.var_thesat_t_dn9, ) = (assign6240_e6209, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn4 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn6 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn7 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn8 } else { 0.0 }, if locals.var_thesat_p >= 0.0 { locals.var_thesat_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6250_e6221: f64 = (locals.var_thesatac_p * locals.var_temp);
            (locals.var_thesatac_p, locals.var_thesatac_p_dn4, locals.var_thesatac_p_dn6, locals.var_thesatac_p_dn7, locals.var_thesatac_p_dn8, locals.var_thesatac_p_dn9, ) = (assign6250_e6221, ((locals.var_thesatac_p_dn4 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn4)), ((locals.var_thesatac_p_dn6 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn6)), ((locals.var_thesatac_p_dn7 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn7)), ((locals.var_thesatac_p_dn8 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn8)), ((locals.var_thesatac_p_dn9 * locals.var_temp) + (locals.var_thesatac_p * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6260_e6233: f64 = (locals.var_thesatac_p).max(0.0);
            (locals.var_thesatac_t, locals.var_thesatac_t_dn4, locals.var_thesatac_t_dn6, locals.var_thesatac_t_dn7, locals.var_thesatac_t_dn8, locals.var_thesatac_t_dn9, ) = (assign6260_e6233, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn4 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn6 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn7 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn8 } else { 0.0 }, if locals.var_thesatac_p >= 0.0 { locals.var_thesatac_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6270_e6245: f64 = (p.p479 * locals.var_temp0__blk79);
            let assign6270_e6247: f64 = (assign6270_e6245 / locals.var_kstressvth0);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6270_e6247, ((p.p479 * locals.var_temp0__blk79_dn4) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn6) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn7) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn8) / locals.var_kstressvth0), ((p.p479 * locals.var_temp0__blk79_dn9) / locals.var_kstressvth0), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6280_e6259: f64 = (locals.var_vfb1_t + locals.var_temp);
            (locals.var_vfb1_t, locals.var_vfb1_t_dn4, locals.var_vfb1_t_dn6, locals.var_vfb1_t_dn7, locals.var_vfb1_t_dn8, locals.var_vfb1_t_dn9, ) = (assign6280_e6259, (locals.var_vfb1_t_dn4 + locals.var_temp_dn4), (locals.var_vfb1_t_dn6 + locals.var_temp_dn6), (locals.var_vfb1_t_dn7 + locals.var_temp_dn7), (locals.var_vfb1_t_dn8 + locals.var_temp_dn8), (locals.var_vfb1_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6290_e6271: f64 = (locals.var_vfb2_t + locals.var_temp);
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (assign6290_e6271, (locals.var_vfb2_t_dn4 + locals.var_temp_dn4), (locals.var_vfb2_t_dn6 + locals.var_temp_dn6), (locals.var_vfb2_t_dn7 + locals.var_temp_dn7), (locals.var_vfb2_t_dn8 + locals.var_temp_dn8), (locals.var_vfb2_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6300_e6283: f64 = (locals.var_vfbac1_t + locals.var_temp);
            (locals.var_vfbac1_t, locals.var_vfbac1_t_dn4, locals.var_vfbac1_t_dn6, locals.var_vfbac1_t_dn7, locals.var_vfbac1_t_dn8, locals.var_vfbac1_t_dn9, ) = (assign6300_e6283, (locals.var_vfbac1_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac1_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac1_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac1_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac1_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6310_e6295: f64 = (locals.var_vfbac2_t + locals.var_temp);
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (assign6310_e6295, (locals.var_vfbac2_t_dn4 + locals.var_temp_dn4), (locals.var_vfbac2_t_dn6 + locals.var_temp_dn6), (locals.var_vfbac2_t_dn7 + locals.var_temp_dn7), (locals.var_vfbac2_t_dn8 + locals.var_temp_dn8), (locals.var_vfbac2_t_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6320_e6307: f64 = (p.p481 * locals.var_temp0__blk79);
            let assign6320_e6310: f64 = (locals.var_lambda_le).powf(p.p232);
            let assign6320_e6311: f64 = (assign6320_e6307 * assign6320_e6310);
            let assign6320_e6315: f64 = (p.p233 * locals.var_iwe);
            let assign6320_e6316: f64 = (1.0 + assign6320_e6315);
            let assign6320_e6317: f64 = (assign6320_e6311 * assign6320_e6316);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6320_e6317, (((p.p481 * locals.var_temp0__blk79_dn4) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn6) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn7) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn8) * assign6320_e6310) * assign6320_e6316), (((p.p481 * locals.var_temp0__blk79_dn9) * assign6320_e6310) * assign6320_e6316), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6330_e6329: f64 = (locals.var_cf_p + locals.var_temp);
            (locals.var_cf_p, locals.var_cf_p_dn4, locals.var_cf_p_dn6, locals.var_cf_p_dn7, locals.var_cf_p_dn8, locals.var_cf_p_dn9, ) = (assign6330_e6329, (locals.var_cf_p_dn4 + locals.var_temp_dn4), (locals.var_cf_p_dn6 + locals.var_temp_dn6), (locals.var_cf_p_dn7 + locals.var_temp_dn7), (locals.var_cf_p_dn8 + locals.var_temp_dn8), (locals.var_cf_p_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6340_e6341: f64 = (locals.var_cf_p).max(0.0);
            (locals.var_cf1_t, locals.var_cf1_t_dn4, locals.var_cf1_t_dn6, locals.var_cf1_t_dn7, locals.var_cf1_t_dn8, locals.var_cf1_t_dn9, ) = (assign6340_e6341, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn4 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn6 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn7 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn8 } else { 0.0 }, if locals.var_cf_p >= 0.0 { locals.var_cf_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6350_e6353: f64 = (locals.var_cfac_p + locals.var_temp);
            (locals.var_cfac_p, locals.var_cfac_p_dn4, locals.var_cfac_p_dn6, locals.var_cfac_p_dn7, locals.var_cfac_p_dn8, locals.var_cfac_p_dn9, ) = (assign6350_e6353, (locals.var_cfac_p_dn4 + locals.var_temp_dn4), (locals.var_cfac_p_dn6 + locals.var_temp_dn6), (locals.var_cfac_p_dn7 + locals.var_temp_dn7), (locals.var_cfac_p_dn8 + locals.var_temp_dn8), (locals.var_cfac_p_dn9 + locals.var_temp_dn9), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6360_e6365: f64 = (locals.var_cfac_p).max(0.0);
            (locals.var_cfac1_t, locals.var_cfac1_t_dn4, locals.var_cfac1_t_dn6, locals.var_cfac1_t_dn7, locals.var_cfac1_t_dn8, locals.var_cfac1_t_dn9, ) = (assign6360_e6365, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn4 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn6 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn7 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn8 } else { 0.0 }, if locals.var_cfac_p >= 0.0 { locals.var_cfac_p_dn9 } else { 0.0 }, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6370_e6377: f64 = (p.p234 * locals.var_tox2_i);
            let assign6370_e6379: f64 = (assign6370_e6377 / locals.var_tox1_i);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6370_e6379, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6380_e6391: f64 = (locals.var_cf1_t * locals.var_temp);
            (locals.var_cf2_t, locals.var_cf2_t_dn4, locals.var_cf2_t_dn6, locals.var_cf2_t_dn7, locals.var_cf2_t_dn8, locals.var_cf2_t_dn9, ) = (assign6380_e6391, ((locals.var_cf1_t_dn4 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn4)), ((locals.var_cf1_t_dn6 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn6)), ((locals.var_cf1_t_dn7 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn7)), ((locals.var_cf1_t_dn8 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn8)), ((locals.var_cf1_t_dn9 * locals.var_temp) + (locals.var_cf1_t * locals.var_temp_dn9)), );
        }

        if (((locals.var_guard83 == 0.0) && (locals.var_guard133 != 0.0)) && (locals.var_guard134 == 0.0)) {
            let assign6390_e6403: f64 = (locals.var_cfac1_t * locals.var_temp);
            (locals.var_cfac2_t, locals.var_cfac2_t_dn4, locals.var_cfac2_t_dn6, locals.var_cfac2_t_dn7, locals.var_cfac2_t_dn8, locals.var_cfac2_t_dn9, ) = (assign6390_e6403, ((locals.var_cfac1_t_dn4 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn4)), ((locals.var_cfac1_t_dn6 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn6)), ((locals.var_cfac1_t_dn7 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn7)), ((locals.var_cfac1_t_dn8 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn8)), ((locals.var_cfac1_t_dn9 * locals.var_temp) + (locals.var_cfac1_t * locals.var_temp_dn9)), );
        }

        let assign6400_e6408: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign6400_e6408;

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
        (locals.var_egsi, locals.var_egsi_dn4, locals.var_egsi_dn6, locals.var_egsi_dn7, locals.var_egsi_dn8, locals.var_egsi_dn9, ) = (assign6540_e6471, (-((((0.000473 * locals.var_tkc_sq_dn4) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn4)) / (assign6540_e6469 * assign6540_e6469))), (-((((0.000473 * locals.var_tkc_sq_dn6) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn6)) / (assign6540_e6469 * assign6540_e6469))), (-((((0.000473 * locals.var_tkc_sq_dn7) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn7)) / (assign6540_e6469 * assign6540_e6469))), (-((((0.000473 * locals.var_tkc_sq_dn8) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn8)) / (assign6540_e6469 * assign6540_e6469))), (-((((0.000473 * locals.var_tkc_sq_dn9) * assign6540_e6469) - (assign6540_e6466 * locals.var_tkc_dn9)) / (assign6540_e6469 * assign6540_e6469))), );

        let assign6550_e6475: f64 = (0.0004774 * locals.var_tkc_sq);
        let assign6550_e6478: f64 = (235.0 + locals.var_tkc);
        let assign6550_e6479: f64 = (assign6550_e6475 / assign6550_e6478);
        let assign6550_e6480: f64 = (0.744 - assign6550_e6479);
        (locals.var_egge, locals.var_egge_dn4, locals.var_egge_dn6, locals.var_egge_dn7, locals.var_egge_dn8, locals.var_egge_dn9, ) = (assign6550_e6480, (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn4)) / (assign6550_e6478 * assign6550_e6478))), (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn6)) / (assign6550_e6478 * assign6550_e6478))), (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn7)) / (assign6550_e6478 * assign6550_e6478))), (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn8)) / (assign6550_e6478 * assign6550_e6478))), (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign6550_e6478) - (assign6550_e6475 * locals.var_tkc_dn9)) / (assign6550_e6478 * assign6550_e6478))), );

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6560_e6483: f64 = (locals.var_egge - locals.var_egsi);
        let assign6560_e6485: f64 = (-0.4);
        let assign6560_e6487: f64 = (assign6560_e6485 * locals.var_one_m_xge);
        let assign6560_e6488: f64 = (assign6560_e6483 + assign6560_e6487);
        let assign6560_e6490: f64 = (assign6560_e6488 * locals.var_xge_i);
        (locals.var_deg, locals.var_deg_dn4, locals.var_deg_dn6, locals.var_deg_dn7, locals.var_deg_dn8, locals.var_deg_dn9, ) = (assign6560_e6490, ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i), ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i), ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i), ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i), ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i), );

        let assign6570_e6493: f64 = (locals.var_egsi + locals.var_deg);
        (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, ) = (assign6570_e6493, (locals.var_egsi_dn4 + locals.var_deg_dn4), (locals.var_egsi_dn6 + locals.var_deg_dn6), (locals.var_egsi_dn7 + locals.var_deg_dn7), (locals.var_egsi_dn8 + locals.var_deg_dn8), (locals.var_egsi_dn9 + locals.var_deg_dn9), );

        let assign6580_e6496: f64 = (0.5 * locals.var_eg);
        let assign6580_e6498: f64 = (assign6580_e6496 * locals.var_inv_phit0);
        (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9, ) = (assign6580_e6498, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign6580_e6496 * locals.var_inv_phit0_dn9)), );

        (locals.var_eg_2phit0_woshe, locals.var_eg_2phit0_woshe_dn4, locals.var_eg_2phit0_woshe_dn6, locals.var_eg_2phit0_woshe_dn7, locals.var_eg_2phit0_woshe_dn8, locals.var_eg_2phit0_woshe_dn9, ) = (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9, );

        let assign6600_e6504: f64 = (10.0 * locals.var_xge_i);
        let assign6600_e6505: f64 = (assign6600_e6504).sqrt();
        let assign6600_e6506: f64 = (1.0 + assign6600_e6505);
        let assign6600_e6507: f64 = (1.0 / assign6600_e6506);
        locals.var_niratio = assign6600_e6507;

        let assign6610_e6510: f64 = (0.05 * locals.var_xge_i);
        let assign6610_e6513: f64 = (0.5 * locals.var_deg);
        let assign6610_e6514: f64 = (assign6610_e6510 - assign6610_e6513);
        (locals.var_dvfbch, locals.var_dvfbch_dn4, locals.var_dvfbch_dn6, locals.var_dvfbch_dn7, locals.var_dvfbch_dn8, locals.var_dvfbch_dn9, ) = (assign6610_e6514, (-(0.5 * locals.var_deg_dn4)), (-(0.5 * locals.var_deg_dn6)), (-(0.5 * locals.var_deg_dn7)), (-(0.5 * locals.var_deg_dn8)), (-(0.5 * locals.var_deg_dn9)), );

        let assign6620_e6517: f64 = (1.602176565e-19 * locals.var_nch_i);
        let assign6620_e6519: f64 = (assign6620_e6517 * 0.5);
        let assign6620_e6521: f64 = (assign6620_e6519 * locals.var_tsi_i);
        let assign6620_e6523: f64 = (assign6620_e6521 / 3.45313e-11);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6620_e6523, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign6630_e6526: f64 = if locals.var_typech_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign6630_e6526;

        if (locals.var_guard140 != 0.0) {
            let assign6640_e6532: f64 = (p.p13 * 4e-10);
            let assign6640_e6533: f64 = (locals.var_tox1_i + assign6640_e6532);
            let assign6640_e6534: f64 = (locals.var_temp * assign6640_e6533);
            (locals.var_dvfb1nch, locals.var_dvfb1nch_dn4, locals.var_dvfb1nch_dn6, locals.var_dvfb1nch_dn7, locals.var_dvfb1nch_dn8, locals.var_dvfb1nch_dn9, ) = (assign6640_e6534, (locals.var_temp_dn4 * assign6640_e6533), (locals.var_temp_dn6 * assign6640_e6533), (locals.var_temp_dn7 * assign6640_e6533), (locals.var_temp_dn8 * assign6640_e6533), (locals.var_temp_dn9 * assign6640_e6533), );
        }

        if (locals.var_guard140 != 0.0) {
            let assign6650_e6542: f64 = (p.p13 * 4e-10);
            let assign6650_e6543: f64 = (locals.var_tox2_i + assign6650_e6542);
            let assign6650_e6544: f64 = (locals.var_temp * assign6650_e6543);
            (locals.var_dvfb2nch, locals.var_dvfb2nch_dn4, locals.var_dvfb2nch_dn6, locals.var_dvfb2nch_dn7, locals.var_dvfb2nch_dn8, locals.var_dvfb2nch_dn9, ) = (assign6650_e6544, (locals.var_temp_dn4 * assign6650_e6543), (locals.var_temp_dn6 * assign6650_e6543), (locals.var_temp_dn7 * assign6650_e6543), (locals.var_temp_dn8 * assign6650_e6543), (locals.var_temp_dn9 * assign6650_e6543), );
        }

        if (locals.var_guard140 == 0.0) {
            let assign6660_e6550: f64 = (-locals.var_temp);
            let assign6660_e6554: f64 = (p.p13 * 4e-10);
            let assign6660_e6555: f64 = (locals.var_tox1_i + assign6660_e6554);
            let assign6660_e6556: f64 = (assign6660_e6550 * assign6660_e6555);
            (locals.var_dvfb1nch, locals.var_dvfb1nch_dn4, locals.var_dvfb1nch_dn6, locals.var_dvfb1nch_dn7, locals.var_dvfb1nch_dn8, locals.var_dvfb1nch_dn9, ) = (assign6660_e6556, ((-locals.var_temp_dn4) * assign6660_e6555), ((-locals.var_temp_dn6) * assign6660_e6555), ((-locals.var_temp_dn7) * assign6660_e6555), ((-locals.var_temp_dn8) * assign6660_e6555), ((-locals.var_temp_dn9) * assign6660_e6555), );
        }

        if (locals.var_guard140 == 0.0) {
            let assign6670_e6562: f64 = (-locals.var_temp);
            let assign6670_e6566: f64 = (p.p13 * 4e-10);
            let assign6670_e6567: f64 = (locals.var_tox2_i + assign6670_e6566);
            let assign6670_e6568: f64 = (assign6670_e6562 * assign6670_e6567);
            (locals.var_dvfb2nch, locals.var_dvfb2nch_dn4, locals.var_dvfb2nch_dn6, locals.var_dvfb2nch_dn7, locals.var_dvfb2nch_dn8, locals.var_dvfb2nch_dn9, ) = (assign6670_e6568, ((-locals.var_temp_dn4) * assign6670_e6567), ((-locals.var_temp_dn6) * assign6670_e6567), ((-locals.var_temp_dn7) * assign6670_e6567), ((-locals.var_temp_dn8) * assign6670_e6567), ((-locals.var_temp_dn9) * assign6670_e6567), );
        }

        let assign6680_e6573: f64 = (locals.var_tkc * 0.0033333333333);
        let assign6680_e6574: f64 = (assign6680_e6573).sqrt();
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6680_e6574, ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign6680_e6574)), ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign6680_e6574)), ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign6680_e6574)), ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign6680_e6574)), ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign6680_e6574)), );

        let assign6690_e6577: f64 = (4.05e25 * locals.var_temp);
        let assign6690_e6579: f64 = (assign6690_e6577 * locals.var_temp);
        let assign6690_e6581: f64 = (assign6690_e6579 * locals.var_temp);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign6690_e6581, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn4)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn6)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn7)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn8)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign6690_e6577 * locals.var_temp_dn9)) * locals.var_temp) + (assign6690_e6579 * locals.var_temp_dn9)), );

        let assign6700_e6584: f64 = (locals.var_temp1 * locals.var_niratio);
        (locals.var_neff, locals.var_neff_dn4, locals.var_neff_dn6, locals.var_neff_dn7, locals.var_neff_dn8, locals.var_neff_dn9, ) = (assign6700_e6584, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio), );

        let assign6710_e6588: f64 = (0.5 * locals.var_deg);
        let assign6710_e6590: f64 = (assign6710_e6588 * locals.var_inv_phit0);
        let assign6710_e6591: f64 = (assign6710_e6590).exp();
        let assign6710_e6592: f64 = (locals.var_temp1 * assign6710_e6591);
        (locals.var_neff_poly, locals.var_neff_poly_dn4, locals.var_neff_poly_dn6, locals.var_neff_poly_dn7, locals.var_neff_poly_dn8, locals.var_neff_poly_dn9, ) = (assign6710_e6592, ((locals.var_temp1_dn4 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn4) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn4))))), ((locals.var_temp1_dn6 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn6) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn6))))), ((locals.var_temp1_dn7 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn7) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn7))))), ((locals.var_temp1_dn8 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn8) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn8))))), ((locals.var_temp1_dn9 * assign6710_e6591) + (locals.var_temp1 * (assign6710_e6591 * (((0.5 * locals.var_deg_dn9) * locals.var_inv_phit0) + (assign6710_e6588 * locals.var_inv_phit0_dn9))))), );

        let assign6720_e6596: f64 = (0.5 * locals.var_deg);
        let assign6720_e6598: f64 = (assign6720_e6596 * locals.var_inv_phit0);
        let assign6720_e6599: f64 = (assign6720_e6598).exp();
        let assign6720_e6600: f64 = (locals.var_temp1 * assign6720_e6599);
        (locals.var_neff_sub, locals.var_neff_sub_dn4, locals.var_neff_sub_dn6, locals.var_neff_sub_dn7, locals.var_neff_sub_dn8, locals.var_neff_sub_dn9, ) = (assign6720_e6600, ((locals.var_temp1_dn4 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn4) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn4))))), ((locals.var_temp1_dn6 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn6) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn6))))), ((locals.var_temp1_dn7 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn7) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn7))))), ((locals.var_temp1_dn8 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn8) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn8))))), ((locals.var_temp1_dn9 * assign6720_e6599) + (locals.var_temp1 * (assign6720_e6599 * (((0.5 * locals.var_deg_dn9) * locals.var_inv_phit0) + (assign6720_e6596 * locals.var_inv_phit0_dn9))))), );

        let assign6730_e6603: f64 = (3.45313e-11 / locals.var_tox1_i);
        locals.var_cox1init = assign6730_e6603;

        let assign6740_e6606: f64 = (3.45313e-11 / locals.var_tox2_i);
        locals.var_cox2init = assign6740_e6606;

        let assign6750_e6609: f64 = if locals.var_pnce_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign6750_e6609;

        if (locals.var_guard141 != 0.0) {
            let assign6760_e6614: f64 = (1.0 + locals.var_pnce_i);
            let assign6760_e6615: f64 = (locals.var_cox1init * assign6760_e6614);
            locals.var_cox1prime = assign6760_e6615;
        }

        if (locals.var_guard141 != 0.0) {
            locals.var_cox2prime = locals.var_cox2init;
        }

        if (locals.var_guard141 == 0.0) {
            locals.var_cox1prime = locals.var_cox1init;
        }

        if (locals.var_guard141 == 0.0) {
            let assign6790_e6632: f64 = (1.0 - locals.var_pnce_i);
            let assign6790_e6633: f64 = (locals.var_cox2init * assign6790_e6632);
            locals.var_cox2prime = assign6790_e6633;
        }

        let assign6800_e6638: f64 = (locals.var_epsch / locals.var_tsi_i);
        locals.var_csiprime_0 = assign6800_e6638;

        let assign6810_e6643: f64 = (locals.var_ct_i * locals.var_rtn);
        let assign6810_e6644: f64 = (1.0 + assign6810_e6643);
        let assign6810_e6645: f64 = (locals.var_phit0 * assign6810_e6644);
        (locals.var_phit, locals.var_phit_dn4, locals.var_phit_dn6, locals.var_phit_dn7, locals.var_phit_dn8, locals.var_phit_dn9, ) = (assign6810_e6645, ((locals.var_phit0_dn4 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign6810_e6644) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9))), );

        let assign6820_e6648: f64 = (1.0 / locals.var_phit);
        (locals.var_inv_phit, locals.var_inv_phit_dn4, locals.var_inv_phit_dn6, locals.var_inv_phit_dn7, locals.var_inv_phit_dn8, locals.var_inv_phit_dn9, ) = (assign6820_e6648, (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit))), );

        let assign6830_e6651: f64 = (0.5 * locals.var_eg);
        let assign6830_e6653: f64 = (assign6830_e6651 * locals.var_inv_phit);
        (locals.var_eg_2phit, locals.var_eg_2phit_dn4, locals.var_eg_2phit_dn6, locals.var_eg_2phit_dn7, locals.var_eg_2phit_dn8, locals.var_eg_2phit_dn9, ) = (assign6830_e6653, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign6830_e6651 * locals.var_inv_phit_dn9)), );

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
        (locals.var_a0_csisq, locals.var_a0_csisq_dn4, locals.var_a0_csisq_dn6, locals.var_a0_csisq_dn7, locals.var_a0_csisq_dn8, locals.var_a0_csisq_dn9, ) = (assign6870_e6679, ((((assign6870_e6673 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn4)), ((((assign6870_e6673 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn6)), ((((assign6870_e6673 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn7)), ((((assign6870_e6673 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn8)), ((((assign6870_e6673 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign6870_e6677 * locals.var_inv_phit_dn9)), );

        let assign6880_e6682: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
        let assign6880_e6684: f64 = (assign6880_e6682 / locals.var_a0_csisq);
        let assign6880_e6685: f64 = (assign6880_e6684).ln();
        let assign6880_e6687: f64 = (assign6880_e6685 - 0.6931471805599);
        (locals.var_xth_1d, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9, ) = (assign6880_e6687, ((-((assign6880_e6682 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684), ((-((assign6880_e6682 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684), ((-((assign6880_e6682 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684), ((-((assign6880_e6682 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684), ((-((assign6880_e6682 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign6880_e6684), );

        let assign6890_e6690: f64 = (0.5 * 1.602176565e-19);
        let assign6890_e6692: f64 = (assign6890_e6690 * locals.var_nsddc_i);
        let assign6890_e6694: f64 = (assign6890_e6692 * locals.var_tsi_i);
        let assign6890_e6697: f64 = (locals.var_cox1prime + locals.var_cox2prime);
        let assign6890_e6698: f64 = (assign6890_e6694 / assign6890_e6697);
        let assign6890_e6700: f64 = (assign6890_e6698 * locals.var_inv_phit);
        (locals.var_xsddep, locals.var_xsddep_dn4, locals.var_xsddep_dn6, locals.var_xsddep_dn7, locals.var_xsddep_dn8, locals.var_xsddep_dn9, ) = (assign6890_e6700, (assign6890_e6698 * locals.var_inv_phit_dn4), (assign6890_e6698 * locals.var_inv_phit_dn6), (assign6890_e6698 * locals.var_inv_phit_dn7), (assign6890_e6698 * locals.var_inv_phit_dn8), (assign6890_e6698 * locals.var_inv_phit_dn9), );

        let assign6900_e6703: f64 = (locals.var_stcf_i * locals.var_dt);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign6900_e6703, ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4)), ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6)), ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7)), ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8)), ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9)), );

        let assign6910_e6706: f64 = (locals.var_cf1_t + locals.var_temp);
        (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9, ) = (assign6910_e6706, (locals.var_cf1_t_dn4 + locals.var_temp_dn4), (locals.var_cf1_t_dn6 + locals.var_temp_dn6), (locals.var_cf1_t_dn7 + locals.var_temp_dn7), (locals.var_cf1_t_dn8 + locals.var_temp_dn8), (locals.var_cf1_t_dn9 + locals.var_temp_dn9), );

        let assign6920_e6709: f64 = (locals.var_cf2_t + locals.var_temp);
        (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9, ) = (assign6920_e6709, (locals.var_cf2_t_dn4 + locals.var_temp_dn4), (locals.var_cf2_t_dn6 + locals.var_temp_dn6), (locals.var_cf2_t_dn7 + locals.var_temp_dn7), (locals.var_cf2_t_dn8 + locals.var_temp_dn8), (locals.var_cf2_t_dn9 + locals.var_temp_dn9), );

        let assign6930_e6712: f64 = (locals.var_cfac1_t + locals.var_temp);
        (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9, ) = (assign6930_e6712, (locals.var_cfac1_t_dn4 + locals.var_temp_dn4), (locals.var_cfac1_t_dn6 + locals.var_temp_dn6), (locals.var_cfac1_t_dn7 + locals.var_temp_dn7), (locals.var_cfac1_t_dn8 + locals.var_temp_dn8), (locals.var_cfac1_t_dn9 + locals.var_temp_dn9), );

        let assign6940_e6715: f64 = (locals.var_cfac2_t + locals.var_temp);
        (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9, ) = (assign6940_e6715, (locals.var_cfac2_t_dn4 + locals.var_temp_dn4), (locals.var_cfac2_t_dn6 + locals.var_temp_dn6), (locals.var_cfac2_t_dn7 + locals.var_temp_dn7), (locals.var_cfac2_t_dn8 + locals.var_temp_dn8), (locals.var_cfac2_t_dn9 + locals.var_temp_dn9), );

        let assign6950_e6718: f64 = (locals.var_cfd_i * locals.var_inv_phit);
        (locals.var_xd0, locals.var_xd0_dn4, locals.var_xd0_dn6, locals.var_xd0_dn7, locals.var_xd0_dn8, locals.var_xd0_dn9, ) = (assign6950_e6718, (locals.var_cfd_i * locals.var_inv_phit_dn4), (locals.var_cfd_i * locals.var_inv_phit_dn6), (locals.var_cfd_i * locals.var_inv_phit_dn7), (locals.var_cfd_i * locals.var_inv_phit_dn8), (locals.var_cfd_i * locals.var_inv_phit_dn9), );

        let assign6960_e6721: f64 = (2.0 * 1.602176565e-19);
        let assign6960_e6723: f64 = (assign6960_e6721 * locals.var_nsub_i);
        let assign6960_e6725: f64 = (assign6960_e6723 * 1.04479e-10);
        let assign6960_e6727: f64 = (assign6960_e6725 * locals.var_inv_phit0);
        let assign6960_e6728: f64 = (assign6960_e6727).sqrt();
        let assign6960_e6730: f64 = (assign6960_e6728 / locals.var_cox2prime);
        (locals.var_gfsub, locals.var_gfsub_dn4, locals.var_gfsub_dn6, locals.var_gfsub_dn7, locals.var_gfsub_dn8, locals.var_gfsub_dn9, ) = (assign6960_e6730, (((assign6960_e6725 * locals.var_inv_phit0_dn4) / (2.0 * assign6960_e6728)) / locals.var_cox2prime), (((assign6960_e6725 * locals.var_inv_phit0_dn6) / (2.0 * assign6960_e6728)) / locals.var_cox2prime), (((assign6960_e6725 * locals.var_inv_phit0_dn7) / (2.0 * assign6960_e6728)) / locals.var_cox2prime), (((assign6960_e6725 * locals.var_inv_phit0_dn8) / (2.0 * assign6960_e6728)) / locals.var_cox2prime), (((assign6960_e6725 * locals.var_inv_phit0_dn9) / (2.0 * assign6960_e6728)) / locals.var_cox2prime), );

        let assign6970_e6733: f64 = (locals.var_gfsub * locals.var_gfsub);
        (locals.var_gfsub2, locals.var_gfsub2_dn4, locals.var_gfsub2_dn6, locals.var_gfsub2_dn7, locals.var_gfsub2_dn8, locals.var_gfsub2_dn9, ) = (assign6970_e6733, ((locals.var_gfsub_dn4 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn4)), ((locals.var_gfsub_dn6 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn6)), ((locals.var_gfsub_dn7 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn7)), ((locals.var_gfsub_dn8 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn8)), ((locals.var_gfsub_dn9 * locals.var_gfsub) + (locals.var_gfsub * locals.var_gfsub_dn9)), );

        let assign6980_e6736: f64 = (1.0 / locals.var_gfsub2);
        (locals.var_inv_gfsub2, locals.var_inv_gfsub2_dn4, locals.var_inv_gfsub2_dn6, locals.var_inv_gfsub2_dn7, locals.var_inv_gfsub2_dn8, locals.var_inv_gfsub2_dn9, ) = (assign6980_e6736, (-(locals.var_gfsub2_dn4 / (locals.var_gfsub2 * locals.var_gfsub2))), (-(locals.var_gfsub2_dn6 / (locals.var_gfsub2 * locals.var_gfsub2))), (-(locals.var_gfsub2_dn7 / (locals.var_gfsub2 * locals.var_gfsub2))), (-(locals.var_gfsub2_dn8 / (locals.var_gfsub2 * locals.var_gfsub2))), (-(locals.var_gfsub2_dn9 / (locals.var_gfsub2 * locals.var_gfsub2))), );

        let assign6990_e6740: f64 = (locals.var_gfsub / 1.4142135623731);
        let assign6990_e6741: f64 = (1.0 + assign6990_e6740);
        (locals.var_xisub, locals.var_xisub_dn4, locals.var_xisub_dn6, locals.var_xisub_dn7, locals.var_xisub_dn8, locals.var_xisub_dn9, ) = (assign6990_e6741, (locals.var_gfsub_dn4 / 1.4142135623731), (locals.var_gfsub_dn6 / 1.4142135623731), (locals.var_gfsub_dn7 / 1.4142135623731), (locals.var_gfsub_dn8 / 1.4142135623731), (locals.var_gfsub_dn9 / 1.4142135623731), );

        let assign7000_e6744: f64 = (1.0 / locals.var_xisub);
        (locals.var_inv_xisub, locals.var_inv_xisub_dn4, locals.var_inv_xisub_dn6, locals.var_inv_xisub_dn7, locals.var_inv_xisub_dn8, locals.var_inv_xisub_dn9, ) = (assign7000_e6744, (-(locals.var_xisub_dn4 / (locals.var_xisub * locals.var_xisub))), (-(locals.var_xisub_dn6 / (locals.var_xisub * locals.var_xisub))), (-(locals.var_xisub_dn7 / (locals.var_xisub * locals.var_xisub))), (-(locals.var_xisub_dn8 / (locals.var_xisub * locals.var_xisub))), (-(locals.var_xisub_dn9 / (locals.var_xisub * locals.var_xisub))), );

        let assign7010_e6747: f64 = (1e-5 * locals.var_xisub);
        locals.var_margin_sub = assign7010_e6747;

        let assign7020_e6750: f64 = (locals.var_nsub_i / locals.var_neff_sub);
        let assign7020_e6751: f64 = (assign7020_e6750).ln();
        let assign7020_e6753: f64 = (assign7020_e6751 + locals.var_eg_2phit0);
        (locals.var_xb_sub, locals.var_xb_sub_dn4, locals.var_xb_sub_dn6, locals.var_xb_sub_dn7, locals.var_xb_sub_dn8, locals.var_xb_sub_dn9, ) = (assign7020_e6753, (((-((locals.var_nsub_i * locals.var_neff_sub_dn4) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn4), (((-((locals.var_nsub_i * locals.var_neff_sub_dn6) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn6), (((-((locals.var_nsub_i * locals.var_neff_sub_dn7) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn7), (((-((locals.var_nsub_i * locals.var_neff_sub_dn8) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn8), (((-((locals.var_nsub_i * locals.var_neff_sub_dn9) / (locals.var_neff_sub * locals.var_neff_sub))) / assign7020_e6750) + locals.var_eg_2phit0_dn9), );

        let assign7030_e6756: f64 = (2.0 * locals.var_xb_sub);
        (locals.var_xn_sub, locals.var_xn_sub_dn4, locals.var_xn_sub_dn6, locals.var_xn_sub_dn7, locals.var_xn_sub_dn8, locals.var_xn_sub_dn9, ) = (assign7030_e6756, (2.0 * locals.var_xb_sub_dn4), (2.0 * locals.var_xb_sub_dn6), (2.0 * locals.var_xb_sub_dn7), (2.0 * locals.var_xb_sub_dn8), (2.0 * locals.var_xb_sub_dn9), );

        let assign7040_e6759: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7040_e6759;

        if (locals.var_guard142 != 0.0) {
            let assign7050_e6764: f64 = (locals.var_typesub_i * locals.var_phit0);
            let assign7050_e6766: f64 = (assign7050_e6764 * locals.var_xb_sub);
            let assign7050_e6767: f64 = (locals.var_vfb2_t + assign7050_e6766);
            (locals.var_vfb2_t, locals.var_vfb2_t_dn4, locals.var_vfb2_t_dn6, locals.var_vfb2_t_dn7, locals.var_vfb2_t_dn8, locals.var_vfb2_t_dn9, ) = (assign7050_e6767, (locals.var_vfb2_t_dn4 + (((locals.var_typesub_i * locals.var_phit0_dn4) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn4))), (locals.var_vfb2_t_dn6 + (((locals.var_typesub_i * locals.var_phit0_dn6) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn6))), (locals.var_vfb2_t_dn7 + (((locals.var_typesub_i * locals.var_phit0_dn7) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn7))), (locals.var_vfb2_t_dn8 + (((locals.var_typesub_i * locals.var_phit0_dn8) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn8))), (locals.var_vfb2_t_dn9 + (((locals.var_typesub_i * locals.var_phit0_dn9) * locals.var_xb_sub) + (assign7050_e6764 * locals.var_xb_sub_dn9))), );
        }

        if (locals.var_guard142 != 0.0) {
            let assign7060_e6774: f64 = (locals.var_typesub_i * locals.var_phit0);
            let assign7060_e6776: f64 = (assign7060_e6774 * locals.var_xb_sub);
            let assign7060_e6777: f64 = (locals.var_vfbac2_t + assign7060_e6776);
            (locals.var_vfbac2_t, locals.var_vfbac2_t_dn4, locals.var_vfbac2_t_dn6, locals.var_vfbac2_t_dn7, locals.var_vfbac2_t_dn8, locals.var_vfbac2_t_dn9, ) = (assign7060_e6777, (locals.var_vfbac2_t_dn4 + (((locals.var_typesub_i * locals.var_phit0_dn4) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn4))), (locals.var_vfbac2_t_dn6 + (((locals.var_typesub_i * locals.var_phit0_dn6) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn6))), (locals.var_vfbac2_t_dn7 + (((locals.var_typesub_i * locals.var_phit0_dn7) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn7))), (locals.var_vfbac2_t_dn8 + (((locals.var_typesub_i * locals.var_phit0_dn8) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn8))), (locals.var_vfbac2_t_dn9 + (((locals.var_typesub_i * locals.var_phit0_dn9) * locals.var_xb_sub) + (assign7060_e6774 * locals.var_xb_sub_dn9))), );
        }

        (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7080_e6783: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7080_e6783;

        if (locals.var_guard143 != 0.0) {
            let assign7090_e6788: f64 = (locals.var_np_i / locals.var_neff_poly);
            let assign7090_e6789: f64 = (assign7090_e6788).ln();
            let assign7090_e6791: f64 = (assign7090_e6789 + locals.var_eg_2phit0);
            let assign7090_e6792: f64 = (locals.var_phit0 * assign7090_e6791);
            (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9, ) = (assign7090_e6792, ((locals.var_phit0_dn4 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn4))), ((locals.var_phit0_dn6 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn6))), ((locals.var_phit0_dn7 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn7))), ((locals.var_phit0_dn8 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn8))), ((locals.var_phit0_dn9 * assign7090_e6791) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign7090_e6788) + locals.var_eg_2phit0_dn9))), );
        }

        let assign7100_e6797: f64 = (2.0 * 1.602176565e-19);
        let assign7100_e6799: f64 = (assign7100_e6797 * locals.var_epsch);
        let assign7100_e6801: f64 = (assign7100_e6799 * locals.var_np_i);
        let assign7100_e6802: f64 = (assign7100_e6801).sqrt();
        let assign7100_e6804: f64 = (assign7100_e6802 / locals.var_cox1init);
        (locals.var_kp, locals.var_kp_dn4, locals.var_kp_dn6, locals.var_kp_dn7, locals.var_kp_dn8, locals.var_kp_dn9, ) = (assign7100_e6804, (((assign7100_e6799 * locals.var_np_i_dn4) / (2.0 * assign7100_e6802)) / locals.var_cox1init), (((assign7100_e6799 * locals.var_np_i_dn6) / (2.0 * assign7100_e6802)) / locals.var_cox1init), (((assign7100_e6799 * locals.var_np_i_dn7) / (2.0 * assign7100_e6802)) / locals.var_cox1init), (((assign7100_e6799 * locals.var_np_i_dn8) / (2.0 * assign7100_e6802)) / locals.var_cox1init), (((assign7100_e6799 * locals.var_np_i_dn9) / (2.0 * assign7100_e6802)) / locals.var_cox1init), );

        (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9, ) = (15.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7120_e6808: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard144 = assign7120_e6808;

        if (locals.var_guard144 != 0.0) {
            let assign7130_e6814: f64 = (2970.0 / locals.var_tkd);
            let assign7130_e6815: f64 = (15.0 + assign7130_e6814);
            let assign7130_e6819: f64 = (2970.0 / locals.var_tkd);
            let assign7130_e6820: f64 = (15.0 - assign7130_e6819);
            let assign7130_e6824: f64 = (2970.0 / locals.var_tkd);
            let assign7130_e6825: f64 = (15.0 - assign7130_e6824);
            let assign7130_e6826: f64 = (assign7130_e6820 * assign7130_e6825);
            let assign7130_e6828: f64 = (assign7130_e6826 + 1e-6);
            let assign7130_e6829: f64 = (assign7130_e6828).sqrt();
            let assign7130_e6830: f64 = (assign7130_e6815 + assign7130_e6829);
            let assign7130_e6831: f64 = (0.5 * assign7130_e6830);
            (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9, ) = (assign7130_e6831, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign7130_e6825) + (assign7130_e6820 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign7130_e6829)))), );
        }

        locals.var_dvfbqm = 0.0;

        (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7160_e6838: f64 = (1e18 * locals.var_tsi_i);
        let assign7160_e6840: f64 = (assign7160_e6838 * locals.var_tsi_i);
        locals.var_tsisq = assign7160_e6840;

        let assign7170_e6843: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7170_e6843;

        let assign7180_e6846: f64 = 1.0;
        let assign7180_e6847: f64 = if p.p14 == assign7180_e6846 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7180_e6847;

        if ((locals.var_guard145 != 0.0) && (locals.var_guard146 != 0.0)) {
            let assign7190_e6853: f64 = (0.409618895 / locals.var_tsisq);
            locals.var_dvfbqm = assign7190_e6853;
        }

        if ((locals.var_guard145 != 0.0) && (locals.var_guard146 != 0.0)) {
            let assign7200_e6861: f64 = (0.4 * p.p13);
            let assign7200_e6863: f64 = (assign7200_e6861 * 1.27520989);
            let assign7200_e6865: f64 = (-0.3333333333333);
            let assign7200_e6868: f64 = (locals.var_phit * locals.var_tsisq);
            let assign7200_e6869: f64 = (assign7200_e6868).ln();
            let assign7200_e6870: f64 = (assign7200_e6865 * assign7200_e6869);
            let assign7200_e6871: f64 = (assign7200_e6870).exp();
            let assign7200_e6872: f64 = (assign7200_e6863 * assign7200_e6871);
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (assign7200_e6872, (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign7200_e6868)))), (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign7200_e6868)))), (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign7200_e6868)))), (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign7200_e6868)))), (assign7200_e6863 * (assign7200_e6871 * (assign7200_e6865 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign7200_e6868)))), );
        }

        if ((locals.var_guard145 != 0.0) && (locals.var_guard146 == 0.0)) {
            let assign7210_e6881: f64 = (0.723134895 / locals.var_tsisq);
            locals.var_dvfbqm = assign7210_e6881;
        }

        if ((locals.var_guard145 != 0.0) && (locals.var_guard146 == 0.0)) {
            let assign7220_e6890: f64 = (0.4 * p.p13);
            let assign7220_e6892: f64 = (assign7220_e6890 * 1.5412087);
            let assign7220_e6894: f64 = (-0.3333333333333);
            let assign7220_e6897: f64 = (locals.var_phit * locals.var_tsisq);
            let assign7220_e6898: f64 = (assign7220_e6897).ln();
            let assign7220_e6899: f64 = (assign7220_e6894 * assign7220_e6898);
            let assign7220_e6900: f64 = (assign7220_e6899).exp();
            let assign7220_e6901: f64 = (assign7220_e6892 * assign7220_e6900);
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (assign7220_e6901, (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign7220_e6897)))), (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign7220_e6897)))), (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign7220_e6897)))), (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign7220_e6897)))), (assign7220_e6892 * (assign7220_e6900 * (assign7220_e6894 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign7220_e6897)))), );
        }

        let assign7230_e6906: f64 = (p.p14 * locals.var_stvfb_i);
        let assign7230_e6908: f64 = (assign7230_e6906 * locals.var_dt);
        let assign7230_e6910: f64 = (assign7230_e6908 + locals.var_dvfbqm);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign7230_e6910, (assign7230_e6906 * locals.var_dt_dn4), (assign7230_e6906 * locals.var_dt_dn6), (assign7230_e6906 * locals.var_dt_dn7), (assign7230_e6906 * locals.var_dt_dn8), (assign7230_e6906 * locals.var_dt_dn9), );

        let assign7240_e6913: f64 = (locals.var_temp + p.p34);
        let assign7240_e6915: f64 = (assign7240_e6913 - locals.var_dvfbpdep);
        (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign7240_e6915, (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4), (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6), (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7), (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8), (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9), );

        let assign7250_e6919: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
        let assign7250_e6921: f64 = (assign7250_e6919 + locals.var_dvfb1nch);
        let assign7250_e6922: f64 = (p.p14 * assign7250_e6921);
        let assign7250_e6924: f64 = (assign7250_e6922 + locals.var_temp1);
        (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9, ) = (assign7250_e6924, ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9), );

        let assign7260_e6928: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
        let assign7260_e6930: f64 = (assign7260_e6928 + locals.var_dvfb2nch);
        let assign7260_e6931: f64 = (p.p14 * assign7260_e6930);
        let assign7260_e6933: f64 = (assign7260_e6931 + locals.var_temp);
        (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9, ) = (assign7260_e6933, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );

        let assign7270_e6937: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
        let assign7270_e6939: f64 = (assign7270_e6937 + locals.var_dvfb1nch);
        let assign7270_e6940: f64 = (p.p14 * assign7270_e6939);
        let assign7270_e6942: f64 = (assign7270_e6940 + locals.var_temp1);
        (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9, ) = (assign7270_e6942, ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9), );

        let assign7280_e6946: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
        let assign7280_e6948: f64 = (assign7280_e6946 + locals.var_dvfb2nch);
        let assign7280_e6949: f64 = (p.p14 * assign7280_e6948);
        let assign7280_e6951: f64 = (assign7280_e6949 + locals.var_temp);
        (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9, ) = (assign7280_e6951, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );

        let assign7290_e6953: f64 = (locals.var_rtn).ln();
        (locals.var_lnrtn, locals.var_lnrtn_dn4, locals.var_lnrtn_dn6, locals.var_lnrtn_dn7, locals.var_lnrtn_dn8, locals.var_lnrtn_dn9, ) = (assign7290_e6953, (locals.var_rtn_dn4 / locals.var_rtn), (locals.var_rtn_dn6 / locals.var_rtn), (locals.var_rtn_dn7 / locals.var_rtn), (locals.var_rtn_dn8 / locals.var_rtn), (locals.var_rtn_dn9 / locals.var_rtn), );

        let assign7300_e6956: f64 = (locals.var_stbet_i * locals.var_lnrtn);
        let assign7300_e6957: f64 = (assign7300_e6956).exp();
        let assign7300_e6959: f64 = (assign7300_e6957 * p.p35);
        (locals.var_tf_bet, locals.var_tf_bet_dn4, locals.var_tf_bet_dn6, locals.var_tf_bet_dn7, locals.var_tf_bet_dn8, locals.var_tf_bet_dn9, ) = (assign7300_e6959, ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35), ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35), ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35), ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35), ((assign7300_e6957 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35), );

        let assign7310_e6962: f64 = (locals.var_betn1_t * locals.var_tf_bet);
        (locals.var_betn1_i, locals.var_betn1_i_dn4, locals.var_betn1_i_dn6, locals.var_betn1_i_dn7, locals.var_betn1_i_dn8, locals.var_betn1_i_dn9, ) = (assign7310_e6962, ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4)), ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6)), ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7)), ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8)), ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9)), );

        let assign7320_e6965: f64 = (locals.var_betn2_t * locals.var_tf_bet);
        (locals.var_betn2_i, locals.var_betn2_i_dn4, locals.var_betn2_i_dn6, locals.var_betn2_i_dn7, locals.var_betn2_i_dn8, locals.var_betn2_i_dn9, ) = (assign7320_e6965, ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4)), ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6)), ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7)), ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8)), ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9)), );

        let assign7330_e6968: f64 = (locals.var_stmue_i * locals.var_lnrtn);
        let assign7330_e6969: f64 = (assign7330_e6968).exp();
        (locals.var_tf_mue, locals.var_tf_mue_dn4, locals.var_tf_mue_dn6, locals.var_tf_mue_dn7, locals.var_tf_mue_dn8, locals.var_tf_mue_dn9, ) = (assign7330_e6969, (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn4)), (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn6)), (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn7)), (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn8)), (assign7330_e6969 * (locals.var_stmue_i * locals.var_lnrtn_dn9)), );

        let assign7340_e6972: f64 = (locals.var_mue_t * locals.var_tf_mue);
        (locals.var_mue_i, locals.var_mue_i_dn4, locals.var_mue_i_dn6, locals.var_mue_i_dn7, locals.var_mue_i_dn8, locals.var_mue_i_dn9, ) = (assign7340_e6972, (locals.var_mue_t * locals.var_tf_mue_dn4), (locals.var_mue_t * locals.var_tf_mue_dn6), (locals.var_mue_t * locals.var_tf_mue_dn7), (locals.var_mue_t * locals.var_tf_mue_dn8), (locals.var_mue_t * locals.var_tf_mue_dn9), );

        let assign7350_e6975: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
        let assign7350_e6976: f64 = (assign7350_e6975).exp();
        (locals.var_tf_themu, locals.var_tf_themu_dn4, locals.var_tf_themu_dn6, locals.var_tf_themu_dn7, locals.var_tf_themu_dn8, locals.var_tf_themu_dn9, ) = (assign7350_e6976, (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn4)), (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn6)), (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn7)), (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn8)), (assign7350_e6976 * (locals.var_stthemu_i * locals.var_lnrtn_dn9)), );

        let assign7360_e6979: f64 = (locals.var_themu_t * locals.var_tf_themu);
        (locals.var_themu_i, locals.var_themu_i_dn4, locals.var_themu_i_dn6, locals.var_themu_i_dn7, locals.var_themu_i_dn8, locals.var_themu_i_dn9, ) = (assign7360_e6979, (locals.var_themu_t * locals.var_tf_themu_dn4), (locals.var_themu_t * locals.var_tf_themu_dn6), (locals.var_themu_t * locals.var_tf_themu_dn7), (locals.var_themu_t * locals.var_tf_themu_dn8), (locals.var_themu_t * locals.var_tf_themu_dn9), );

        let assign7370_e6982: f64 = (locals.var_stcs_i * locals.var_lnrtn);
        let assign7370_e6983: f64 = (assign7370_e6982).exp();
        (locals.var_tf_cs, locals.var_tf_cs_dn4, locals.var_tf_cs_dn6, locals.var_tf_cs_dn7, locals.var_tf_cs_dn8, locals.var_tf_cs_dn9, ) = (assign7370_e6983, (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn4)), (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn6)), (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn7)), (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn8)), (assign7370_e6983 * (locals.var_stcs_i * locals.var_lnrtn_dn9)), );

        let assign7380_e6986: f64 = (locals.var_cs_t * locals.var_tf_cs);
        (locals.var_cs_i, locals.var_cs_i_dn4, locals.var_cs_i_dn6, locals.var_cs_i_dn7, locals.var_cs_i_dn8, locals.var_cs_i_dn9, ) = (assign7380_e6986, (locals.var_cs_t * locals.var_tf_cs_dn4), (locals.var_cs_t * locals.var_tf_cs_dn6), (locals.var_cs_t * locals.var_tf_cs_dn7), (locals.var_cs_t * locals.var_tf_cs_dn8), (locals.var_cs_t * locals.var_tf_cs_dn9), );

        let assign7390_e6989: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
        let assign7390_e6990: f64 = (assign7390_e6989).exp();
        (locals.var_tf_thecs, locals.var_tf_thecs_dn4, locals.var_tf_thecs_dn6, locals.var_tf_thecs_dn7, locals.var_tf_thecs_dn8, locals.var_tf_thecs_dn9, ) = (assign7390_e6990, (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn4)), (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn6)), (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn7)), (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn8)), (assign7390_e6990 * (locals.var_stthecs_i * locals.var_lnrtn_dn9)), );

        let assign7400_e6993: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
        (locals.var_thecs_i, locals.var_thecs_i_dn4, locals.var_thecs_i_dn6, locals.var_thecs_i_dn7, locals.var_thecs_i_dn8, locals.var_thecs_i_dn9, ) = (assign7400_e6993, (locals.var_thecs_t * locals.var_tf_thecs_dn4), (locals.var_thecs_t * locals.var_tf_thecs_dn6), (locals.var_thecs_t * locals.var_tf_thecs_dn7), (locals.var_thecs_t * locals.var_tf_thecs_dn8), (locals.var_thecs_t * locals.var_tf_thecs_dn9), );

        let assign7410_e6996: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
        let assign7410_e6997: f64 = (assign7410_e6996).exp();
        (locals.var_tf_xcor, locals.var_tf_xcor_dn4, locals.var_tf_xcor_dn6, locals.var_tf_xcor_dn7, locals.var_tf_xcor_dn8, locals.var_tf_xcor_dn9, ) = (assign7410_e6997, (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn4)), (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn6)), (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn7)), (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn8)), (assign7410_e6997 * (locals.var_stxcor_i * locals.var_lnrtn_dn9)), );

        let assign7420_e7000: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
        (locals.var_xcor_i, locals.var_xcor_i_dn4, locals.var_xcor_i_dn6, locals.var_xcor_i_dn7, locals.var_xcor_i_dn8, locals.var_xcor_i_dn9, ) = (assign7420_e7000, (locals.var_xcor_t * locals.var_tf_xcor_dn4), (locals.var_xcor_t * locals.var_tf_xcor_dn6), (locals.var_xcor_t * locals.var_tf_xcor_dn7), (locals.var_xcor_t * locals.var_tf_xcor_dn8), (locals.var_xcor_t * locals.var_tf_xcor_dn9), );

        let assign7430_e7003: f64 = (1e-8 * locals.var_phit);
        let assign7430_e7005: f64 = (assign7430_e7003 / locals.var_tsi_i);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign7430_e7005, ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i), );

        let assign7440_e7008: f64 = (locals.var_temp * locals.var_mue_i);
        (locals.var_fmue, locals.var_fmue_dn4, locals.var_fmue_dn6, locals.var_fmue_dn7, locals.var_fmue_dn8, locals.var_fmue_dn9, ) = (assign7440_e7008, ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4)), ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6)), ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7)), ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8)), ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9)), );

        let assign7450_e7012: f64 = (0.5 * locals.var_csthr_i);
        let assign7450_e7013: f64 = (1.0 / assign7450_e7012);
        locals.var_inv_qi1cs = assign7450_e7013;

        let assign7460_e7016: f64 = (locals.var_inv_qi1cs / locals.var_csthrb_i);
        locals.var_inv_qi2cs = assign7460_e7016;

        let assign7470_e7019: f64 = 1.0;
        let assign7470_e7020: f64 = if p.p14 == assign7470_e7019 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7470_e7020;

        if (locals.var_guard147 != 0.0) {
            let assign7480_e7024: f64 = (0.5 * locals.var_feta_i);
            locals.var_eta_mu = assign7480_e7024;
        }

        if (locals.var_guard147 == 0.0) {
            let assign7490_e7031: f64 = (0.3333333333333 * locals.var_feta_i);
            locals.var_eta_mu = assign7490_e7031;
        }

        let assign7500_e7036: f64 = (1.0 - locals.var_eta_mu);
        locals.var_one_m_eta = assign7500_e7036;

        let assign7510_e7039: f64 = (locals.var_strs_i * locals.var_lnrtn);
        let assign7510_e7040: f64 = (assign7510_e7039).exp();
        (locals.var_tf_ther, locals.var_tf_ther_dn4, locals.var_tf_ther_dn6, locals.var_tf_ther_dn7, locals.var_tf_ther_dn8, locals.var_tf_ther_dn9, ) = (assign7510_e7040, (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn4)), (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn6)), (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn7)), (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn8)), (assign7510_e7040 * (locals.var_strs_i * locals.var_lnrtn_dn9)), );

        let assign7520_e7043: f64 = (locals.var_rs_t * locals.var_tf_ther);
        (locals.var_rs_i, locals.var_rs_i_dn4, locals.var_rs_i_dn6, locals.var_rs_i_dn7, locals.var_rs_i_dn8, locals.var_rs_i_dn9, ) = (assign7520_e7043, (locals.var_rs_t * locals.var_tf_ther_dn4), (locals.var_rs_t * locals.var_tf_ther_dn6), (locals.var_rs_t * locals.var_tf_ther_dn7), (locals.var_rs_t * locals.var_tf_ther_dn8), (locals.var_rs_t * locals.var_tf_ther_dn9), );

        let assign7530_e7046: f64 = (2.0 * locals.var_rs_i);
        let assign7530_e7048: f64 = (assign7530_e7046 * locals.var_phit);
        (locals.var_frs, locals.var_frs_dn4, locals.var_frs_dn6, locals.var_frs_dn7, locals.var_frs_dn8, locals.var_frs_dn9, ) = (assign7530_e7048, (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn4)), (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn6)), (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn7)), (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn8)), (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign7530_e7046 * locals.var_phit_dn9)), );

        let assign7540_e7052: f64 = (16.0 / locals.var_ax_i);
        let assign7540_e7054: f64 = (assign7540_e7052 * 0.6931471805599);
        let assign7540_e7055: f64 = (assign7540_e7054).exp();
        let assign7540_e7057: f64 = (assign7540_e7055 - 1.0);
        let assign7540_e7058: f64 = (assign7540_e7057).ln();
        let assign7540_e7059: f64 = (0.375 * assign7540_e7058);
        let assign7540_e7060: f64 = (assign7540_e7059).exp();
        let assign7540_e7062: f64 = (assign7540_e7060 - 1.0);
        locals.var_gamax = assign7540_e7062;

        let assign7550_e7066: f64 = (16.0 / locals.var_axac_i);
        let assign7550_e7068: f64 = (assign7550_e7066 * 0.6931471805599);
        let assign7550_e7069: f64 = (assign7550_e7068).exp();
        let assign7550_e7071: f64 = (assign7550_e7069 - 1.0);
        let assign7550_e7072: f64 = (assign7550_e7071).ln();
        let assign7550_e7073: f64 = (0.375 * assign7550_e7072);
        let assign7550_e7074: f64 = (assign7550_e7073).exp();
        let assign7550_e7076: f64 = (assign7550_e7074 - 1.0);
        locals.var_gamax_ac = assign7550_e7076;

        let assign7560_e7079: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
        let assign7560_e7080: f64 = (assign7560_e7079).exp();
        (locals.var_tf_thesat, locals.var_tf_thesat_dn4, locals.var_tf_thesat_dn6, locals.var_tf_thesat_dn7, locals.var_tf_thesat_dn8, locals.var_tf_thesat_dn9, ) = (assign7560_e7080, (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn4)), (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn6)), (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn7)), (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn8)), (assign7560_e7080 * (locals.var_stthesat_i * locals.var_lnrtn_dn9)), );

        let assign7570_e7083: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
        let assign7570_e7085: f64 = (assign7570_e7083 * locals.var_tf_bet);
        (locals.var_thesat_i, locals.var_thesat_i_dn4, locals.var_thesat_i_dn6, locals.var_thesat_i_dn7, locals.var_thesat_i_dn8, locals.var_thesat_i_dn9, ) = (assign7570_e7085, ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn4)), ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn6)), ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn7)), ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn8)), ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7570_e7083 * locals.var_tf_bet_dn9)), );

        let assign7580_e7088: f64 = (locals.var_thesat_i * locals.var_phit);
        (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9, ) = (assign7580_e7088, ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4)), ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6)), ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7)), ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8)), ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9)), );

        let assign7590_e7091: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
        let assign7590_e7093: f64 = (assign7590_e7091 * locals.var_tf_bet);
        (locals.var_thesatac_i, locals.var_thesatac_i_dn4, locals.var_thesatac_i_dn6, locals.var_thesatac_i_dn7, locals.var_thesatac_i_dn8, locals.var_thesatac_i_dn9, ) = (assign7590_e7093, ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn4)), ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn6)), ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn7)), ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn8)), ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign7590_e7091 * locals.var_tf_bet_dn9)), );

        let assign7600_e7096: f64 = (locals.var_thesatac_i * locals.var_phit);
        (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9, ) = (assign7600_e7096, ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4)), ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6)), ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7)), ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8)), ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9)), );

        let assign7610_e7099: f64 = (locals.var_alp1_i * locals.var_inv_phit);
        (locals.var_alp1_phit, locals.var_alp1_phit_dn4, locals.var_alp1_phit_dn6, locals.var_alp1_phit_dn7, locals.var_alp1_phit_dn8, locals.var_alp1_phit_dn9, ) = (assign7610_e7099, (locals.var_alp1_i * locals.var_inv_phit_dn4), (locals.var_alp1_i * locals.var_inv_phit_dn6), (locals.var_alp1_i * locals.var_inv_phit_dn7), (locals.var_alp1_i * locals.var_inv_phit_dn8), (locals.var_alp1_i * locals.var_inv_phit_dn9), );

        let assign7620_e7101: f64 = (-locals.var_stig_i);
        let assign7620_e7103: f64 = (assign7620_e7101 * locals.var_lnrtn);
        let assign7620_e7104: f64 = (assign7620_e7103).exp();
        (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9, ) = (assign7620_e7104, (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn4)), (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn6)), (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn7)), (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn8)), (assign7620_e7104 * (assign7620_e7101 * locals.var_lnrtn_dn9)), );

        let assign7630_e7107: f64 = (locals.var_iginv_t * locals.var_tf_ig);
        (locals.var_iginv_i, locals.var_iginv_i_dn4, locals.var_iginv_i_dn6, locals.var_iginv_i_dn7, locals.var_iginv_i_dn8, locals.var_iginv_i_dn9, ) = (assign7630_e7107, (locals.var_iginv_t * locals.var_tf_ig_dn4), (locals.var_iginv_t * locals.var_tf_ig_dn6), (locals.var_iginv_t * locals.var_tf_ig_dn7), (locals.var_iginv_t * locals.var_tf_ig_dn8), (locals.var_iginv_t * locals.var_tf_ig_dn9), );

        let assign7640_e7110: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
        (locals.var_igovinv_i, locals.var_igovinv_i_dn4, locals.var_igovinv_i_dn6, locals.var_igovinv_i_dn7, locals.var_igovinv_i_dn8, locals.var_igovinv_i_dn9, ) = (assign7640_e7110, (locals.var_igovinv_t * locals.var_tf_ig_dn4), (locals.var_igovinv_t * locals.var_tf_ig_dn6), (locals.var_igovinv_t * locals.var_tf_ig_dn7), (locals.var_igovinv_t * locals.var_tf_ig_dn8), (locals.var_igovinv_t * locals.var_tf_ig_dn9), );

        let assign7650_e7113: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
        (locals.var_igovinvd_i, locals.var_igovinvd_i_dn4, locals.var_igovinvd_i_dn6, locals.var_igovinvd_i_dn7, locals.var_igovinvd_i_dn8, locals.var_igovinvd_i_dn9, ) = (assign7650_e7113, (locals.var_igovinvd_t * locals.var_tf_ig_dn4), (locals.var_igovinvd_t * locals.var_tf_ig_dn6), (locals.var_igovinvd_t * locals.var_tf_ig_dn7), (locals.var_igovinvd_t * locals.var_tf_ig_dn8), (locals.var_igovinvd_t * locals.var_tf_ig_dn9), );

        let assign7660_e7116: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
        (locals.var_igovacc_i, locals.var_igovacc_i_dn4, locals.var_igovacc_i_dn6, locals.var_igovacc_i_dn7, locals.var_igovacc_i_dn8, locals.var_igovacc_i_dn9, ) = (assign7660_e7116, (locals.var_igovacc_t * locals.var_tf_ig_dn4), (locals.var_igovacc_t * locals.var_tf_ig_dn6), (locals.var_igovacc_t * locals.var_tf_ig_dn7), (locals.var_igovacc_t * locals.var_tf_ig_dn8), (locals.var_igovacc_t * locals.var_tf_ig_dn9), );

        let assign7670_e7119: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
        (locals.var_igovaccd_i, locals.var_igovaccd_i_dn4, locals.var_igovaccd_i_dn6, locals.var_igovaccd_i_dn7, locals.var_igovaccd_i_dn8, locals.var_igovaccd_i_dn9, ) = (assign7670_e7119, (locals.var_igovaccd_t * locals.var_tf_ig_dn4), (locals.var_igovaccd_t * locals.var_tf_ig_dn6), (locals.var_igovaccd_t * locals.var_tf_ig_dn7), (locals.var_igovaccd_t * locals.var_tf_ig_dn8), (locals.var_igovaccd_t * locals.var_tf_ig_dn9), );

    }

    pub(super) fn stamp_transient_block_8(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let assign7680_e7121: f64 = (-locals.var_stigfn_i);
        let assign7680_e7123: f64 = (assign7680_e7121 * locals.var_lnrtn);
        let assign7680_e7124: f64 = (assign7680_e7123).exp();
        (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9, ) = (assign7680_e7124, (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn4)), (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn6)), (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn7)), (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn8)), (assign7680_e7124 * (assign7680_e7121 * locals.var_lnrtn_dn9)), );

        let assign7690_e7127: f64 = (locals.var_fnovinv_t * locals.var_tf_ig);
        (locals.var_fnovinv_i, locals.var_fnovinv_i_dn4, locals.var_fnovinv_i_dn6, locals.var_fnovinv_i_dn7, locals.var_fnovinv_i_dn8, locals.var_fnovinv_i_dn9, ) = (assign7690_e7127, (locals.var_fnovinv_t * locals.var_tf_ig_dn4), (locals.var_fnovinv_t * locals.var_tf_ig_dn6), (locals.var_fnovinv_t * locals.var_tf_ig_dn7), (locals.var_fnovinv_t * locals.var_tf_ig_dn8), (locals.var_fnovinv_t * locals.var_tf_ig_dn9), );

        let assign7700_e7130: f64 = (locals.var_fnovinvd_t * locals.var_tf_ig);
        (locals.var_fnovinvd_i, locals.var_fnovinvd_i_dn4, locals.var_fnovinvd_i_dn6, locals.var_fnovinvd_i_dn7, locals.var_fnovinvd_i_dn8, locals.var_fnovinvd_i_dn9, ) = (assign7700_e7130, (locals.var_fnovinvd_t * locals.var_tf_ig_dn4), (locals.var_fnovinvd_t * locals.var_tf_ig_dn6), (locals.var_fnovinvd_t * locals.var_tf_ig_dn7), (locals.var_fnovinvd_t * locals.var_tf_ig_dn8), (locals.var_fnovinvd_t * locals.var_tf_ig_dn9), );

        let assign7710_e7133: f64 = (1.0 / locals.var_chib_i);
        locals.var_inv_chib = assign7710_e7133;

        let assign7720_e7136: f64 = (4.0 * 0.3333333333333);
        let assign7720_e7139: f64 = (2.0 * 1.602176565e-19);
        let assign7720_e7141: f64 = (assign7720_e7139 * 9.10938291e-31);
        let assign7720_e7143: f64 = (assign7720_e7141 * locals.var_chib_i);
        let assign7720_e7144: f64 = (assign7720_e7143).sqrt();
        let assign7720_e7145: f64 = (assign7720_e7136 * assign7720_e7144);
        let assign7720_e7147: f64 = (assign7720_e7145 / 1.054571726e-34);
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign7720_e7147, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7730_e7150: f64 = (locals.var_tempm * locals.var_toxp_i);
        (locals.var_bch, locals.var_bch_dn4, locals.var_bch_dn6, locals.var_bch_dn7, locals.var_bch_dn8, locals.var_bch_dn9, ) = (assign7730_e7150, (locals.var_tempm_dn4 * locals.var_toxp_i), (locals.var_tempm_dn6 * locals.var_toxp_i), (locals.var_tempm_dn7 * locals.var_toxp_i), (locals.var_tempm_dn8 * locals.var_toxp_i), (locals.var_tempm_dn9 * locals.var_toxp_i), );

        let assign7740_e7153: f64 = (locals.var_tempm * locals.var_toxp_i);
        (locals.var_bov, locals.var_bov_dn4, locals.var_bov_dn6, locals.var_bov_dn7, locals.var_bov_dn8, locals.var_bov_dn9, ) = (assign7740_e7153, (locals.var_tempm_dn4 * locals.var_toxp_i), (locals.var_tempm_dn6 * locals.var_toxp_i), (locals.var_tempm_dn7 * locals.var_toxp_i), (locals.var_tempm_dn8 * locals.var_toxp_i), (locals.var_tempm_dn9 * locals.var_toxp_i), );

        locals.var_gcqch = 0.0;

        let assign7760_e7157: f64 = if locals.var_gc3ch_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign7760_e7157;

        if (locals.var_guard148 != 0.0) {
            let assign7770_e7160: f64 = (-0.495);
            let assign7770_e7162: f64 = (assign7770_e7160 * locals.var_gc2ch_i);
            let assign7770_e7164: f64 = (assign7770_e7162 / locals.var_gc3ch_i);
            locals.var_gcqch = assign7770_e7164;
        }

        locals.var_gcqovinv = 0.0;

        let assign7790_e7170: f64 = if locals.var_gc3ovinv_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7790_e7170;

        if (locals.var_guard149 != 0.0) {
            let assign7800_e7173: f64 = (-0.495);
            let assign7800_e7175: f64 = (assign7800_e7173 * locals.var_gc2ovinv_i);
            let assign7800_e7177: f64 = (assign7800_e7175 / locals.var_gc3ovinv_i);
            locals.var_gcqovinv = assign7800_e7177;
        }

        locals.var_gcqovacc = 0.0;

        let assign7820_e7183: f64 = if locals.var_gc3ovacc_i < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7820_e7183;

        if (locals.var_guard150 != 0.0) {
            let assign7830_e7186: f64 = (-0.495);
            let assign7830_e7188: f64 = (assign7830_e7186 * locals.var_gc2ovacc_i);
            let assign7830_e7190: f64 = (assign7830_e7188 / locals.var_gc3ovacc_i);
            locals.var_gcqovacc = assign7830_e7190;
        }

        let assign7840_e7195: f64 = (0.5 * locals.var_eg);
        (locals.var_alpha_b, locals.var_alpha_b_dn4, locals.var_alpha_b_dn6, locals.var_alpha_b_dn7, locals.var_alpha_b_dn8, locals.var_alpha_b_dn9, ) = (assign7840_e7195, (0.5 * locals.var_eg_dn4), (0.5 * locals.var_eg_dn6), (0.5 * locals.var_eg_dn7), (0.5 * locals.var_eg_dn8), (0.5 * locals.var_eg_dn9), );

        let assign7850_e7198: f64 = (locals.var_gco_i * locals.var_phit);
        (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9, ) = (assign7850_e7198, (locals.var_gco_i * locals.var_phit_dn4), (locals.var_gco_i * locals.var_phit_dn6), (locals.var_gco_i * locals.var_phit_dn7), (locals.var_gco_i * locals.var_phit_dn8), (locals.var_gco_i * locals.var_phit_dn9), );

        let assign7860_e7201: f64 = (locals.var_gco_i * locals.var_phit0);
        (locals.var_dov, locals.var_dov_dn4, locals.var_dov_dn6, locals.var_dov_dn7, locals.var_dov_dn8, locals.var_dov_dn9, ) = (assign7860_e7201, (locals.var_gco_i * locals.var_phit0_dn4), (locals.var_gco_i * locals.var_phit0_dn6), (locals.var_gco_i * locals.var_phit0_dn7), (locals.var_gco_i * locals.var_phit0_dn8), (locals.var_gco_i * locals.var_phit0_dn9), );

        let assign7870_e7206: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
        let assign7870_e7207: f64 = (1.0 + assign7870_e7206);
        let assign7870_e7208: f64 = (1.0 / assign7870_e7207);
        (locals.var_n_iginv, locals.var_n_iginv_dn4, locals.var_n_iginv_dn6, locals.var_n_iginv_dn7, locals.var_n_iginv_dn8, locals.var_n_iginv_dn9, ) = (assign7870_e7208, (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign7870_e7207 * assign7870_e7207))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign7870_e7207 * assign7870_e7207))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign7870_e7207 * assign7870_e7207))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign7870_e7207 * assign7870_e7207))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign7870_e7207 * assign7870_e7207))), );

        let assign7880_e7212: f64 = (locals.var_toxp_i * locals.var_toxp_i);
        let assign7880_e7213: f64 = (4e-18 / assign7880_e7212);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign7880_e7213, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7890_e7216: f64 = (locals.var_agidl_i * locals.var_temp);
        (locals.var_agidl_i, locals.var_agidl_i_dn4, locals.var_agidl_i_dn6, locals.var_agidl_i_dn7, locals.var_agidl_i_dn8, locals.var_agidl_i_dn9, ) = (assign7890_e7216, ((locals.var_agidl_i_dn4 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn4)), ((locals.var_agidl_i_dn6 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn6)), ((locals.var_agidl_i_dn7 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn7)), ((locals.var_agidl_i_dn8 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn8)), ((locals.var_agidl_i_dn9 * locals.var_temp) + (locals.var_agidl_i * locals.var_temp_dn9)), );

        let assign7900_e7219: f64 = (locals.var_agidld_i * locals.var_temp);
        (locals.var_agidld_i, locals.var_agidld_i_dn4, locals.var_agidld_i_dn6, locals.var_agidld_i_dn7, locals.var_agidld_i_dn8, locals.var_agidld_i_dn9, ) = (assign7900_e7219, ((locals.var_agidld_i_dn4 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn4)), ((locals.var_agidld_i_dn6 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn6)), ((locals.var_agidld_i_dn7 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn7)), ((locals.var_agidld_i_dn8 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn8)), ((locals.var_agidld_i_dn9 * locals.var_temp) + (locals.var_agidld_i * locals.var_temp_dn9)), );

        let assign7910_e7222: f64 = (locals.var_toxp_i * 500000000.0);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign7910_e7222, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign7920_e7227: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7228: f64 = (1.0 + assign7920_e7227);
        let assign7920_e7230: f64 = assign7920_e7228;
        let assign7920_e7234: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7235: f64 = (1.0 + assign7920_e7234);
        let assign7920_e7237: f64 = assign7920_e7235;
        let assign7920_e7241: f64 = (locals.var_stbgidl_i * locals.var_dt);
        let assign7920_e7242: f64 = (1.0 + assign7920_e7241);
        let assign7920_e7244: f64 = assign7920_e7242;
        let assign7920_e7245: f64 = (assign7920_e7237 * assign7920_e7244);
        let assign7920_e7247: f64 = (assign7920_e7245 + 0.01);
        let assign7920_e7248: f64 = (assign7920_e7247).sqrt();
        let assign7920_e7249: f64 = (assign7920_e7230 + assign7920_e7248);
        let assign7920_e7250: f64 = (0.5 * assign7920_e7249);
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign7920_e7250, (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign7920_e7248)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign7920_e7248)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign7920_e7248)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign7920_e7248)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign7920_e7244) + (assign7920_e7237 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign7920_e7248)))), );

        let assign7930_e7253: f64 = (locals.var_bgidl_t * locals.var_tempm);
        let assign7930_e7255: f64 = (assign7930_e7253 * locals.var_temp);
        (locals.var_bgidl_i, locals.var_bgidl_i_dn4, locals.var_bgidl_i_dn6, locals.var_bgidl_i_dn7, locals.var_bgidl_i_dn8, locals.var_bgidl_i_dn9, ) = (assign7930_e7255, (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn4)), (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn6)), (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn7)), (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn8)), (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign7930_e7253 * locals.var_temp_dn9)), );

        let assign7940_e7260: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7261: f64 = (1.0 + assign7940_e7260);
        let assign7940_e7263: f64 = assign7940_e7261;
        let assign7940_e7267: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7268: f64 = (1.0 + assign7940_e7267);
        let assign7940_e7270: f64 = assign7940_e7268;
        let assign7940_e7274: f64 = (locals.var_stbgidld_i * locals.var_dt);
        let assign7940_e7275: f64 = (1.0 + assign7940_e7274);
        let assign7940_e7277: f64 = assign7940_e7275;
        let assign7940_e7278: f64 = (assign7940_e7270 * assign7940_e7277);
        let assign7940_e7280: f64 = (assign7940_e7278 + 0.01);
        let assign7940_e7281: f64 = (assign7940_e7280).sqrt();
        let assign7940_e7282: f64 = (assign7940_e7263 + assign7940_e7281);
        let assign7940_e7283: f64 = (0.5 * assign7940_e7282);
        (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign7940_e7283, (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign7940_e7281)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign7940_e7281)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign7940_e7281)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign7940_e7281)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign7940_e7277) + (assign7940_e7270 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign7940_e7281)))), );

        let assign7950_e7286: f64 = (locals.var_bgidld_t * locals.var_tempm);
        let assign7950_e7288: f64 = (assign7950_e7286 * locals.var_temp);
        (locals.var_bgidld_i, locals.var_bgidld_i_dn4, locals.var_bgidld_i_dn6, locals.var_bgidld_i_dn7, locals.var_bgidld_i_dn8, locals.var_bgidld_i_dn9, ) = (assign7950_e7288, (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn4)), (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn6)), (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn7)), (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn8)), (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign7950_e7286 * locals.var_temp_dn9)), );

        let assign7960_e7291: f64 = (-locals.var_sta2_i);
        let assign7960_e7293: f64 = (assign7960_e7291 * locals.var_lnrtn);
        let assign7960_e7294: f64 = (assign7960_e7293).exp();
        let assign7960_e7295: f64 = (locals.var_a2_t * assign7960_e7294);
        (locals.var_a2_i, locals.var_a2_i_dn4, locals.var_a2_i_dn6, locals.var_a2_i_dn7, locals.var_a2_i_dn8, locals.var_a2_i_dn9, ) = (assign7960_e7295, (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn4))), (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn6))), (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn7))), (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn8))), (locals.var_a2_t * (assign7960_e7294 * (assign7960_e7291 * locals.var_lnrtn_dn9))), );

        let assign7970_e7300: f64 = (locals.var_ctedge_i * locals.var_rtn);
        let assign7970_e7301: f64 = (1.0 + assign7970_e7300);
        let assign7970_e7302: f64 = (locals.var_phit0 * assign7970_e7301);
        (locals.var_phit_edge, locals.var_phit_edge_dn4, locals.var_phit_edge_dn6, locals.var_phit_edge_dn7, locals.var_phit_edge_dn8, locals.var_phit_edge_dn9, ) = (assign7970_e7302, ((locals.var_phit0_dn4 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign7970_e7301) + (locals.var_phit0 * (locals.var_ctedge_i * locals.var_rtn_dn9))), );

        let assign7980_e7305: f64 = (1.0 / locals.var_phit_edge);
        (locals.var_inv_phit_edge, locals.var_inv_phit_edge_dn4, locals.var_inv_phit_edge_dn6, locals.var_inv_phit_edge_dn7, locals.var_inv_phit_edge_dn8, locals.var_inv_phit_edge_dn9, ) = (assign7980_e7305, (-(locals.var_phit_edge_dn4 / (locals.var_phit_edge * locals.var_phit_edge))), (-(locals.var_phit_edge_dn6 / (locals.var_phit_edge * locals.var_phit_edge))), (-(locals.var_phit_edge_dn7 / (locals.var_phit_edge * locals.var_phit_edge))), (-(locals.var_phit_edge_dn8 / (locals.var_phit_edge * locals.var_phit_edge))), (-(locals.var_phit_edge_dn9 / (locals.var_phit_edge * locals.var_phit_edge))), );

        let assign7990_e7308: f64 = (2.0 * 1.602176565e-19);
        let assign7990_e7310: f64 = (assign7990_e7308 * locals.var_neff);
        let assign7990_e7312: f64 = (assign7990_e7310 * locals.var_epsch);
        let assign7990_e7314: f64 = (assign7990_e7312 * locals.var_inv_phit_edge);
        (locals.var_a0_csisq_edge, locals.var_a0_csisq_edge_dn4, locals.var_a0_csisq_edge_dn6, locals.var_a0_csisq_edge_dn7, locals.var_a0_csisq_edge_dn8, locals.var_a0_csisq_edge_dn9, ) = (assign7990_e7314, ((((assign7990_e7308 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn4)), ((((assign7990_e7308 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn6)), ((((assign7990_e7308 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn7)), ((((assign7990_e7308 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn8)), ((((assign7990_e7308 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit_edge) + (assign7990_e7312 * locals.var_inv_phit_edge_dn9)), );

        let assign8000_e7317: f64 = (p.p14 * locals.var_stvfbedge_i);
        let assign8000_e7319: f64 = (assign8000_e7317 * locals.var_dt);
        let assign8000_e7321: f64 = (assign8000_e7319 + locals.var_dvfbqm);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8000_e7321, (assign8000_e7317 * locals.var_dt_dn4), (assign8000_e7317 * locals.var_dt_dn6), (assign8000_e7317 * locals.var_dt_dn7), (assign8000_e7317 * locals.var_dt_dn8), (assign8000_e7317 * locals.var_dt_dn9), );

        let assign8010_e7325: f64 = (locals.var_vfb1edge_t + locals.var_dvfbch);
        let assign8010_e7327: f64 = (assign8010_e7325 + locals.var_dvfb1nch);
        let assign8010_e7328: f64 = (p.p14 * assign8010_e7327);
        let assign8010_e7330: f64 = (assign8010_e7328 + locals.var_temp);
        let assign8010_e7332: f64 = (assign8010_e7330 + p.p34);
        let assign8010_e7334: f64 = (assign8010_e7332 - locals.var_dvfbpdep);
        (locals.var_vfb1edge_i, locals.var_vfb1edge_i_dn4, locals.var_vfb1edge_i_dn6, locals.var_vfb1edge_i_dn7, locals.var_vfb1edge_i_dn8, locals.var_vfb1edge_i_dn9, ) = (assign8010_e7334, (((p.p14 * ((locals.var_vfb1edge_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp_dn4) - locals.var_dvfbpdep_dn4), (((p.p14 * ((locals.var_vfb1edge_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp_dn6) - locals.var_dvfbpdep_dn6), (((p.p14 * ((locals.var_vfb1edge_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp_dn7) - locals.var_dvfbpdep_dn7), (((p.p14 * ((locals.var_vfb1edge_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp_dn8) - locals.var_dvfbpdep_dn8), (((p.p14 * ((locals.var_vfb1edge_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp_dn9) - locals.var_dvfbpdep_dn9), );

        let assign8020_e7338: f64 = (locals.var_vfb2edge_t + locals.var_dvfbch);
        let assign8020_e7340: f64 = (assign8020_e7338 + locals.var_dvfb2nch);
        let assign8020_e7341: f64 = (p.p14 * assign8020_e7340);
        let assign8020_e7343: f64 = (assign8020_e7341 + locals.var_temp);
        (locals.var_vfb2edge_i, locals.var_vfb2edge_i_dn4, locals.var_vfb2edge_i_dn6, locals.var_vfb2edge_i_dn7, locals.var_vfb2edge_i_dn8, locals.var_vfb2edge_i_dn9, ) = (assign8020_e7343, ((p.p14 * (locals.var_dvfbch_dn4 + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * (locals.var_dvfbch_dn6 + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * (locals.var_dvfbch_dn7 + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * (locals.var_dvfbch_dn8 + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * (locals.var_dvfbch_dn9 + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );

        let assign8030_e7346: f64 = (locals.var_stbetedge_i * locals.var_lnrtn);
        let assign8030_e7347: f64 = (assign8030_e7346).exp();
        let assign8030_e7349: f64 = (assign8030_e7347 * p.p35);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8030_e7349, ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn4)) * p.p35), ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn6)) * p.p35), ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn7)) * p.p35), ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn8)) * p.p35), ((assign8030_e7347 * (locals.var_stbetedge_i * locals.var_lnrtn_dn9)) * p.p35), );

        let assign8040_e7352: f64 = (locals.var_betnedge_t * locals.var_temp);
        (locals.var_betnedge_i, locals.var_betnedge_i_dn4, locals.var_betnedge_i_dn6, locals.var_betnedge_i_dn7, locals.var_betnedge_i_dn8, locals.var_betnedge_i_dn9, ) = (assign8040_e7352, ((locals.var_betnedge_t_dn4 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn4)), ((locals.var_betnedge_t_dn6 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn6)), ((locals.var_betnedge_t_dn7 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn7)), ((locals.var_betnedge_t_dn8 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn8)), ((locals.var_betnedge_t_dn9 * locals.var_temp) + (locals.var_betnedge_t * locals.var_temp_dn9)), );

        let assign8050_e7355: f64 = (locals.var_areaq_i * locals.var_phit);
        (locals.var_area_phit, locals.var_area_phit_dn4, locals.var_area_phit_dn6, locals.var_area_phit_dn7, locals.var_area_phit_dn8, locals.var_area_phit_dn9, ) = (assign8050_e7355, (locals.var_areaq_i * locals.var_phit_dn4), (locals.var_areaq_i * locals.var_phit_dn6), (locals.var_areaq_i * locals.var_phit_dn7), (locals.var_areaq_i * locals.var_phit_dn8), (locals.var_areaq_i * locals.var_phit_dn9), );

        let assign8060_e7358: f64 = (0.25 * 1.602176565e-19);
        let assign8060_e7360: f64 = (assign8060_e7358 * locals.var_nsdac_i);
        let assign8060_e7363: f64 = (locals.var_epsch * locals.var_phit);
        let assign8060_e7364: f64 = (assign8060_e7360 / assign8060_e7363);
        (locals.var_inner_sd, locals.var_inner_sd_dn4, locals.var_inner_sd_dn6, locals.var_inner_sd_dn7, locals.var_inner_sd_dn8, locals.var_inner_sd_dn9, ) = (assign8060_e7364, (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn4)) / (assign8060_e7363 * assign8060_e7363))), (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn6)) / (assign8060_e7363 * assign8060_e7363))), (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn7)) / (assign8060_e7363 * assign8060_e7363))), (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn8)) / (assign8060_e7363 * assign8060_e7363))), (-((assign8060_e7360 * (locals.var_epsch * locals.var_phit_dn9)) / (assign8060_e7363 * assign8060_e7363))), );

        let assign8070_e7367: f64 = (locals.var_nsdac_i / locals.var_neff);
        let assign8070_e7368: f64 = (assign8070_e7367).ln();
        (locals.var_xsd, locals.var_xsd_dn4, locals.var_xsd_dn6, locals.var_xsd_dn7, locals.var_xsd_dn8, locals.var_xsd_dn9, ) = (assign8070_e7368, ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign8070_e7367), ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign8070_e7367), ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign8070_e7367), ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign8070_e7367), ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign8070_e7367), );

        let assign8080_e7371: f64 = (locals.var_fif_i * 1.25e-6);
        let assign8080_e7373: f64 = (assign8080_e7371 * locals.var_phit);
        (locals.var_fif_phit, locals.var_fif_phit_dn4, locals.var_fif_phit_dn6, locals.var_fif_phit_dn7, locals.var_fif_phit_dn8, locals.var_fif_phit_dn9, ) = (assign8080_e7373, (assign8080_e7371 * locals.var_phit_dn4), (assign8080_e7371 * locals.var_phit_dn6), (assign8080_e7371 * locals.var_phit_dn7), (assign8080_e7371 * locals.var_phit_dn8), (assign8080_e7371 * locals.var_phit_dn9), );

        let assign8090_e7376: f64 = (locals.var_epsch / 3.45313e-11);
        let assign8090_e7378: f64 = (assign8090_e7376 * locals.var_tsi_i);
        let assign8090_e7381: f64 = (locals.var_tox1_i + 4e-10);
        let assign8090_e7382: f64 = (assign8090_e7378 * assign8090_e7381);
        let assign8090_e7383: f64 = (assign8090_e7382).sqrt();
        locals.var_lambda2d = assign8090_e7383;

        let assign8100_e7386: f64 = (locals.var_strth_i * locals.var_lnrtn);
        let assign8100_e7387: f64 = (assign8100_e7386).exp();
        (locals.var_tf_rth, locals.var_tf_rth_dn4, locals.var_tf_rth_dn6, locals.var_tf_rth_dn7, locals.var_tf_rth_dn8, locals.var_tf_rth_dn9, ) = (assign8100_e7387, (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn4)), (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn6)), (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn7)), (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn8)), (assign8100_e7387 * (locals.var_strth_i * locals.var_lnrtn_dn9)), );

        let assign8110_e7390: f64 = (locals.var_rth_t * locals.var_tf_rth);
        (locals.var_rth_i, locals.var_rth_i_dn4, locals.var_rth_i_dn6, locals.var_rth_i_dn7, locals.var_rth_i_dn8, locals.var_rth_i_dn9, ) = (assign8110_e7390, ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4)), ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6)), ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7)), ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8)), ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9)), );

        let assign8120_e7393: f64 = (4.0 * 1.3806488e-23);
        let assign8120_e7395: f64 = (assign8120_e7393 * locals.var_tkc);
        locals.var_nt0_4kt = assign8120_e7395;

        let assign8130_e7398: f64 = (locals.var_fnt_i * locals.var_nt0_4kt);
        locals.var_nt = assign8130_e7398;

        locals.var_nt0 = locals.var_nt;

        let assign8150_e7402: f64 = (9.10938291e-31 * 1000000000000.0);
        let assign8150_e7404: f64 = (assign8150_e7402 * locals.var_fntexc_i);
        locals.var_fac_exc = assign8150_e7404;

        let assign8280_e7463: f64 = if locals.var_swshe_i > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign8280_e7463;

        if (locals.var_guard257 != 0.0) {
            (locals.var_dtc, locals.var_dtc_dn4, ) = ((nv4 - 0.0), 1.0, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8300_e7471: f64 = (locals.var_tkd + locals.var_dtc);
            (locals.var_tkc, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9, ) = (assign8300_e7471, (locals.var_tkd_dn4 + locals.var_dtc_dn4), locals.var_tkd_dn6, locals.var_tkd_dn7, locals.var_tkd_dn8, locals.var_tkd_dn9, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8310_e7477: f64 = (locals.var_tkc * locals.var_tkc);
            (locals.var_tkc_sq, locals.var_tkc_sq_dn4, locals.var_tkc_sq_dn6, locals.var_tkc_sq_dn7, locals.var_tkc_sq_dn8, locals.var_tkc_sq_dn9, ) = (assign8310_e7477, ((locals.var_tkc_dn4 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn4)), ((locals.var_tkc_dn6 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn6)), ((locals.var_tkc_dn7 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn7)), ((locals.var_tkc_dn8 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn8)), ((locals.var_tkc_dn9 * locals.var_tkc) + (locals.var_tkc * locals.var_tkc_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8320_e7483: f64 = (locals.var_tkc - locals.var_tkr);
            (locals.var_dt, locals.var_dt_dn4, locals.var_dt_dn6, locals.var_dt_dn7, locals.var_dt_dn8, locals.var_dt_dn9, ) = (assign8320_e7483, locals.var_tkc_dn4, locals.var_tkc_dn6, locals.var_tkc_dn7, locals.var_tkc_dn8, locals.var_tkc_dn9, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8330_e7489: f64 = (locals.var_tkc / locals.var_tkr);
            (locals.var_rt, locals.var_rt_dn4, locals.var_rt_dn6, locals.var_rt_dn7, locals.var_rt_dn8, locals.var_rt_dn9, ) = (assign8330_e7489, (locals.var_tkc_dn4 / locals.var_tkr), (locals.var_tkc_dn6 / locals.var_tkr), (locals.var_tkc_dn7 / locals.var_tkr), (locals.var_tkc_dn8 / locals.var_tkr), (locals.var_tkc_dn9 / locals.var_tkr), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8340_e7495: f64 = (locals.var_tkr / locals.var_tkc);
            (locals.var_rtn, locals.var_rtn_dn4, locals.var_rtn_dn6, locals.var_rtn_dn7, locals.var_rtn_dn8, locals.var_rtn_dn9, ) = (assign8340_e7495, (-((locals.var_tkr * locals.var_tkc_dn4) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn6) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn7) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn8) / (locals.var_tkc * locals.var_tkc))), (-((locals.var_tkr * locals.var_tkc_dn9) / (locals.var_tkc * locals.var_tkc))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8350_e7501: f64 = (locals.var_tkc * 8.617332384961e-5);
            (locals.var_phit0, locals.var_phit0_dn4, locals.var_phit0_dn6, locals.var_phit0_dn7, locals.var_phit0_dn8, locals.var_phit0_dn9, ) = (assign8350_e7501, (locals.var_tkc_dn4 * 8.617332384961e-5), (locals.var_tkc_dn6 * 8.617332384961e-5), (locals.var_tkc_dn7 * 8.617332384961e-5), (locals.var_tkc_dn8 * 8.617332384961e-5), (locals.var_tkc_dn9 * 8.617332384961e-5), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8360_e7507: f64 = (1.0 / locals.var_phit0);
            (locals.var_inv_phit0, locals.var_inv_phit0_dn4, locals.var_inv_phit0_dn6, locals.var_inv_phit0_dn7, locals.var_inv_phit0_dn8, locals.var_inv_phit0_dn9, ) = (assign8360_e7507, (-(locals.var_phit0_dn4 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn6 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn7 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn8 / (locals.var_phit0 * locals.var_phit0))), (-(locals.var_phit0_dn9 / (locals.var_phit0 * locals.var_phit0))), );
        }

        let assign8370_e7512: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign8370_e7512;

        if ((locals.var_guard257 != 0.0) && (locals.var_guard258 != 0.0)) {
            let assign8380_e7520: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign8380_e7521: f64 = (10.0 / assign8380_e7520);
            let assign8380_e7523: f64 = (assign8380_e7521 + 600.0);
            let assign8380_e7527: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign8380_e7528: f64 = (10.0 / assign8380_e7527);
            let assign8380_e7530: f64 = (assign8380_e7528 - 600.0);
            let assign8380_e7534: f64 = (locals.var_tkd * 8.617332384961e-5);
            let assign8380_e7535: f64 = (10.0 / assign8380_e7534);
            let assign8380_e7537: f64 = (assign8380_e7535 - 600.0);
            let assign8380_e7538: f64 = (assign8380_e7530 * assign8380_e7537);
            let assign8380_e7540: f64 = (assign8380_e7538 + 0.01);
            let assign8380_e7541: f64 = (assign8380_e7540).sqrt();
            let assign8380_e7542: f64 = (assign8380_e7523 + assign8380_e7541);
            let assign8380_e7543: f64 = (0.5 * assign8380_e7542);
            (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9, ) = (assign8380_e7543, (0.5 * ((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn4 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn6 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn7 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn8 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), (0.5 * ((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7520 * assign8380_e7520))) + ((((-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7527 * assign8380_e7527))) * assign8380_e7537) + (assign8380_e7530 * (-((10.0 * (locals.var_tkd_dn9 * 8.617332384961e-5)) / (assign8380_e7534 * assign8380_e7534))))) / (2.0 * assign8380_e7541)))), );
        }

        if ((locals.var_guard257 != 0.0) && (locals.var_guard258 == 0.0)) {
            (locals.var_xsatmax, locals.var_xsatmax_dn4, locals.var_xsatmax_dn6, locals.var_xsatmax_dn7, locals.var_xsatmax_dn8, locals.var_xsatmax_dn9, ) = (600.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8400_e7557: f64 = (0.000473 * locals.var_tkc_sq);
            let assign8400_e7560: f64 = (636.0 + locals.var_tkc);
            let assign8400_e7561: f64 = (assign8400_e7557 / assign8400_e7560);
            let assign8400_e7562: f64 = (1.17 - assign8400_e7561);
            (locals.var_egsi, locals.var_egsi_dn4, locals.var_egsi_dn6, locals.var_egsi_dn7, locals.var_egsi_dn8, locals.var_egsi_dn9, ) = (assign8400_e7562, (-((((0.000473 * locals.var_tkc_sq_dn4) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn4)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn6) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn6)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn7) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn7)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn8) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn8)) / (assign8400_e7560 * assign8400_e7560))), (-((((0.000473 * locals.var_tkc_sq_dn9) * assign8400_e7560) - (assign8400_e7557 * locals.var_tkc_dn9)) / (assign8400_e7560 * assign8400_e7560))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8410_e7569: f64 = (0.0004774 * locals.var_tkc_sq);
            let assign8410_e7572: f64 = (235.0 + locals.var_tkc);
            let assign8410_e7573: f64 = (assign8410_e7569 / assign8410_e7572);
            let assign8410_e7574: f64 = (0.744 - assign8410_e7573);
            (locals.var_egge, locals.var_egge_dn4, locals.var_egge_dn6, locals.var_egge_dn7, locals.var_egge_dn8, locals.var_egge_dn9, ) = (assign8410_e7574, (-((((0.0004774 * locals.var_tkc_sq_dn4) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn4)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn6) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn6)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn7) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn7)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn8) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn8)) / (assign8410_e7572 * assign8410_e7572))), (-((((0.0004774 * locals.var_tkc_sq_dn9) * assign8410_e7572) - (assign8410_e7569 * locals.var_tkc_dn9)) / (assign8410_e7572 * assign8410_e7572))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8420_e7580: f64 = (locals.var_egge - locals.var_egsi);
            let assign8420_e7582: f64 = (-0.4);
            let assign8420_e7584: f64 = (assign8420_e7582 * locals.var_one_m_xge);
            let assign8420_e7585: f64 = (assign8420_e7580 + assign8420_e7584);
            let assign8420_e7587: f64 = (assign8420_e7585 * locals.var_xge_i);
            (locals.var_deg, locals.var_deg_dn4, locals.var_deg_dn6, locals.var_deg_dn7, locals.var_deg_dn8, locals.var_deg_dn9, ) = (assign8420_e7587, ((locals.var_egge_dn4 - locals.var_egsi_dn4) * locals.var_xge_i), ((locals.var_egge_dn6 - locals.var_egsi_dn6) * locals.var_xge_i), ((locals.var_egge_dn7 - locals.var_egsi_dn7) * locals.var_xge_i), ((locals.var_egge_dn8 - locals.var_egsi_dn8) * locals.var_xge_i), ((locals.var_egge_dn9 - locals.var_egsi_dn9) * locals.var_xge_i), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8430_e7593: f64 = (locals.var_egsi + locals.var_deg);
            (locals.var_eg, locals.var_eg_dn4, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, ) = (assign8430_e7593, (locals.var_egsi_dn4 + locals.var_deg_dn4), (locals.var_egsi_dn6 + locals.var_deg_dn6), (locals.var_egsi_dn7 + locals.var_deg_dn7), (locals.var_egsi_dn8 + locals.var_deg_dn8), (locals.var_egsi_dn9 + locals.var_deg_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8440_e7599: f64 = (0.5 * locals.var_eg);
            let assign8440_e7601: f64 = (assign8440_e7599 * locals.var_inv_phit0);
            (locals.var_eg_2phit0, locals.var_eg_2phit0_dn4, locals.var_eg_2phit0_dn6, locals.var_eg_2phit0_dn7, locals.var_eg_2phit0_dn8, locals.var_eg_2phit0_dn9, ) = (assign8440_e7601, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit0) + (assign8440_e7599 * locals.var_inv_phit0_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8450_e7607: f64 = (0.05 * locals.var_xge_i);
            let assign8450_e7610: f64 = (0.5 * locals.var_deg);
            let assign8450_e7611: f64 = (assign8450_e7607 - assign8450_e7610);
            (locals.var_dvfbch, locals.var_dvfbch_dn4, locals.var_dvfbch_dn6, locals.var_dvfbch_dn7, locals.var_dvfbch_dn8, locals.var_dvfbch_dn9, ) = (assign8450_e7611, (-(0.5 * locals.var_deg_dn4)), (-(0.5 * locals.var_deg_dn6)), (-(0.5 * locals.var_deg_dn7)), (-(0.5 * locals.var_deg_dn8)), (-(0.5 * locals.var_deg_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8460_e7617: f64 = (locals.var_tkc * 0.0033333333333);
            let assign8460_e7618: f64 = (assign8460_e7617).sqrt();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8460_e7618, ((locals.var_tkc_dn4 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn6 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn7 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn8 * 0.0033333333333) / (2.0 * assign8460_e7618)), ((locals.var_tkc_dn9 * 0.0033333333333) / (2.0 * assign8460_e7618)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8470_e7624: f64 = (4.05e25 * locals.var_temp);
            let assign8470_e7626: f64 = (assign8470_e7624 * locals.var_temp);
            let assign8470_e7628: f64 = (assign8470_e7626 * locals.var_temp);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign8470_e7628, (((((4.05e25 * locals.var_temp_dn4) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn4)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn4)), (((((4.05e25 * locals.var_temp_dn6) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn6)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn6)), (((((4.05e25 * locals.var_temp_dn7) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn7)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn7)), (((((4.05e25 * locals.var_temp_dn8) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn8)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn8)), (((((4.05e25 * locals.var_temp_dn9) * locals.var_temp) + (assign8470_e7624 * locals.var_temp_dn9)) * locals.var_temp) + (assign8470_e7626 * locals.var_temp_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8480_e7634: f64 = (locals.var_temp1 * locals.var_niratio);
            (locals.var_neff, locals.var_neff_dn4, locals.var_neff_dn6, locals.var_neff_dn7, locals.var_neff_dn8, locals.var_neff_dn9, ) = (assign8480_e7634, (locals.var_temp1_dn4 * locals.var_niratio), (locals.var_temp1_dn6 * locals.var_niratio), (locals.var_temp1_dn7 * locals.var_niratio), (locals.var_temp1_dn8 * locals.var_niratio), (locals.var_temp1_dn9 * locals.var_niratio), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8490_e7642: f64 = (locals.var_ct_i * locals.var_rtn);
            let assign8490_e7643: f64 = (1.0 + assign8490_e7642);
            let assign8490_e7644: f64 = (locals.var_phit0 * assign8490_e7643);
            (locals.var_phit, locals.var_phit_dn4, locals.var_phit_dn6, locals.var_phit_dn7, locals.var_phit_dn8, locals.var_phit_dn9, ) = (assign8490_e7644, ((locals.var_phit0_dn4 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn4))), ((locals.var_phit0_dn6 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn6))), ((locals.var_phit0_dn7 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn7))), ((locals.var_phit0_dn8 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn8))), ((locals.var_phit0_dn9 * assign8490_e7643) + (locals.var_phit0 * (locals.var_ct_i * locals.var_rtn_dn9))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8500_e7650: f64 = (1.0 / locals.var_phit);
            (locals.var_inv_phit, locals.var_inv_phit_dn4, locals.var_inv_phit_dn6, locals.var_inv_phit_dn7, locals.var_inv_phit_dn8, locals.var_inv_phit_dn9, ) = (assign8500_e7650, (-(locals.var_phit_dn4 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn6 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn7 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn8 / (locals.var_phit * locals.var_phit))), (-(locals.var_phit_dn9 / (locals.var_phit * locals.var_phit))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8510_e7656: f64 = (0.5 * locals.var_eg);
            let assign8510_e7658: f64 = (assign8510_e7656 * locals.var_inv_phit);
            (locals.var_eg_2phit, locals.var_eg_2phit_dn4, locals.var_eg_2phit_dn6, locals.var_eg_2phit_dn7, locals.var_eg_2phit_dn8, locals.var_eg_2phit_dn9, ) = (assign8510_e7658, (((0.5 * locals.var_eg_dn4) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn4)), (((0.5 * locals.var_eg_dn6) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn6)), (((0.5 * locals.var_eg_dn7) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn7)), (((0.5 * locals.var_eg_dn8) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn8)), (((0.5 * locals.var_eg_dn9) * locals.var_inv_phit) + (assign8510_e7656 * locals.var_inv_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8520_e7664: f64 = (2.0 * 1.602176565e-19);
            let assign8520_e7666: f64 = (assign8520_e7664 * locals.var_neff);
            let assign8520_e7668: f64 = (assign8520_e7666 * locals.var_epsch);
            let assign8520_e7670: f64 = (assign8520_e7668 * locals.var_inv_phit);
            (locals.var_a0_csisq, locals.var_a0_csisq_dn4, locals.var_a0_csisq_dn6, locals.var_a0_csisq_dn7, locals.var_a0_csisq_dn8, locals.var_a0_csisq_dn9, ) = (assign8520_e7670, ((((assign8520_e7664 * locals.var_neff_dn4) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn4)), ((((assign8520_e7664 * locals.var_neff_dn6) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn6)), ((((assign8520_e7664 * locals.var_neff_dn7) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn7)), ((((assign8520_e7664 * locals.var_neff_dn8) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn8)), ((((assign8520_e7664 * locals.var_neff_dn9) * locals.var_epsch) * locals.var_inv_phit) + (assign8520_e7668 * locals.var_inv_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8530_e7676: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
            let assign8530_e7678: f64 = (assign8530_e7676 / locals.var_a0_csisq);
            let assign8530_e7679: f64 = (assign8530_e7678).ln();
            let assign8530_e7681: f64 = (assign8530_e7679 - 0.6931471805599);
            (locals.var_xth_1d, locals.var_xth_1d_dn4, locals.var_xth_1d_dn6, locals.var_xth_1d_dn7, locals.var_xth_1d_dn8, locals.var_xth_1d_dn9, ) = (assign8530_e7681, ((-((assign8530_e7676 * locals.var_a0_csisq_dn4) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn6) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn7) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn8) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), ((-((assign8530_e7676 * locals.var_a0_csisq_dn9) / (locals.var_a0_csisq * locals.var_a0_csisq))) / assign8530_e7678), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8540_e7687: f64 = (0.5 * 1.602176565e-19);
            let assign8540_e7689: f64 = (assign8540_e7687 * locals.var_nsddc_i);
            let assign8540_e7691: f64 = (assign8540_e7689 * locals.var_tsi_i);
            let assign8540_e7694: f64 = (locals.var_cox1prime + locals.var_cox2prime);
            let assign8540_e7695: f64 = (assign8540_e7691 / assign8540_e7694);
            let assign8540_e7697: f64 = (assign8540_e7695 * locals.var_inv_phit);
            (locals.var_xsddep, locals.var_xsddep_dn4, locals.var_xsddep_dn6, locals.var_xsddep_dn7, locals.var_xsddep_dn8, locals.var_xsddep_dn9, ) = (assign8540_e7697, (assign8540_e7695 * locals.var_inv_phit_dn4), (assign8540_e7695 * locals.var_inv_phit_dn6), (assign8540_e7695 * locals.var_inv_phit_dn7), (assign8540_e7695 * locals.var_inv_phit_dn8), (assign8540_e7695 * locals.var_inv_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8550_e7703: f64 = (locals.var_stcf_i * locals.var_dt);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8550_e7703, ((locals.var_stcf_i_dn4 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn4)), ((locals.var_stcf_i_dn6 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn6)), ((locals.var_stcf_i_dn7 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn7)), ((locals.var_stcf_i_dn8 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn8)), ((locals.var_stcf_i_dn9 * locals.var_dt) + (locals.var_stcf_i * locals.var_dt_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8560_e7709: f64 = (locals.var_cf1_t + locals.var_temp);
            (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9, ) = (assign8560_e7709, (locals.var_cf1_t_dn4 + locals.var_temp_dn4), (locals.var_cf1_t_dn6 + locals.var_temp_dn6), (locals.var_cf1_t_dn7 + locals.var_temp_dn7), (locals.var_cf1_t_dn8 + locals.var_temp_dn8), (locals.var_cf1_t_dn9 + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8570_e7715: f64 = (locals.var_cf2_t + locals.var_temp);
            (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9, ) = (assign8570_e7715, (locals.var_cf2_t_dn4 + locals.var_temp_dn4), (locals.var_cf2_t_dn6 + locals.var_temp_dn6), (locals.var_cf2_t_dn7 + locals.var_temp_dn7), (locals.var_cf2_t_dn8 + locals.var_temp_dn8), (locals.var_cf2_t_dn9 + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8580_e7721: f64 = (locals.var_cfd_i * locals.var_inv_phit);
            (locals.var_xd0, locals.var_xd0_dn4, locals.var_xd0_dn6, locals.var_xd0_dn7, locals.var_xd0_dn8, locals.var_xd0_dn9, ) = (assign8580_e7721, (locals.var_cfd_i * locals.var_inv_phit_dn4), (locals.var_cfd_i * locals.var_inv_phit_dn6), (locals.var_cfd_i * locals.var_inv_phit_dn7), (locals.var_cfd_i * locals.var_inv_phit_dn8), (locals.var_cfd_i * locals.var_inv_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8590_e7727: f64 = (locals.var_cfac1_t + locals.var_temp);
            (locals.var_cfac1_i, locals.var_cfac1_i_dn4, locals.var_cfac1_i_dn6, locals.var_cfac1_i_dn7, locals.var_cfac1_i_dn8, locals.var_cfac1_i_dn9, ) = (assign8590_e7727, (locals.var_cfac1_t_dn4 + locals.var_temp_dn4), (locals.var_cfac1_t_dn6 + locals.var_temp_dn6), (locals.var_cfac1_t_dn7 + locals.var_temp_dn7), (locals.var_cfac1_t_dn8 + locals.var_temp_dn8), (locals.var_cfac1_t_dn9 + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8600_e7733: f64 = (locals.var_cfac2_t + locals.var_temp);
            (locals.var_cfac2_i, locals.var_cfac2_i_dn4, locals.var_cfac2_i_dn6, locals.var_cfac2_i_dn7, locals.var_cfac2_i_dn8, locals.var_cfac2_i_dn9, ) = (assign8600_e7733, (locals.var_cfac2_t_dn4 + locals.var_temp_dn4), (locals.var_cfac2_t_dn6 + locals.var_temp_dn6), (locals.var_cfac2_t_dn7 + locals.var_temp_dn7), (locals.var_cfac2_t_dn8 + locals.var_temp_dn8), (locals.var_cfac2_t_dn9 + locals.var_temp_dn9), );
        }

        let assign8610_e7738: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign8610_e7738;

        if ((locals.var_guard257 != 0.0) && (locals.var_guard259 != 0.0)) {
            let assign8620_e7745: f64 = (locals.var_np_i / locals.var_neff_poly);
            let assign8620_e7746: f64 = (assign8620_e7745).ln();
            let assign8620_e7748: f64 = (assign8620_e7746 + locals.var_eg_2phit0_woshe);
            let assign8620_e7749: f64 = (locals.var_phit0 * assign8620_e7748);
            (locals.var_dvfbpdep, locals.var_dvfbpdep_dn4, locals.var_dvfbpdep_dn6, locals.var_dvfbpdep_dn7, locals.var_dvfbpdep_dn8, locals.var_dvfbpdep_dn9, ) = (assign8620_e7749, ((locals.var_phit0_dn4 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn4 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn4)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn4))), ((locals.var_phit0_dn6 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn6 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn6)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn6))), ((locals.var_phit0_dn7 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn7 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn7)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn7))), ((locals.var_phit0_dn8 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn8 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn8)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn8))), ((locals.var_phit0_dn9 * assign8620_e7748) + (locals.var_phit0 * (((((locals.var_np_i_dn9 * locals.var_neff_poly) - (locals.var_np_i * locals.var_neff_poly_dn9)) / (locals.var_neff_poly * locals.var_neff_poly)) / assign8620_e7745) + locals.var_eg_2phit0_woshe_dn9))), );
        }

        let assign8630_e7754: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign8630_e7754;

        if ((locals.var_guard257 != 0.0) && (locals.var_guard260 != 0.0)) {
            let assign8640_e7762: f64 = (2970.0 / locals.var_tkd);
            let assign8640_e7763: f64 = (15.0 + assign8640_e7762);
            let assign8640_e7767: f64 = (2970.0 / locals.var_tkd);
            let assign8640_e7768: f64 = (15.0 - assign8640_e7767);
            let assign8640_e7772: f64 = (2970.0 / locals.var_tkd);
            let assign8640_e7773: f64 = (15.0 - assign8640_e7772);
            let assign8640_e7774: f64 = (assign8640_e7768 * assign8640_e7773);
            let assign8640_e7776: f64 = (assign8640_e7774 + 1e-6);
            let assign8640_e7777: f64 = (assign8640_e7776).sqrt();
            let assign8640_e7778: f64 = (assign8640_e7763 + assign8640_e7777);
            let assign8640_e7779: f64 = (0.5 * assign8640_e7778);
            (locals.var_emin, locals.var_emin_dn4, locals.var_emin_dn6, locals.var_emin_dn7, locals.var_emin_dn8, locals.var_emin_dn9, ) = (assign8640_e7779, (0.5 * ((-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn4) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn6) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn7) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn8) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), (0.5 * ((-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd))) + ((((-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))) * assign8640_e7773) + (assign8640_e7768 * (-(-((2970.0 * locals.var_tkd_dn9) / (locals.var_tkd * locals.var_tkd)))))) / (2.0 * assign8640_e7777)))), );
        }

        if (locals.var_guard257 != 0.0) {
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign8660_e7788: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign8660_e7788;

        let assign8670_e7791: f64 = 1.0;
        let assign8670_e7792: f64 = if p.p14 == assign8670_e7791 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign8670_e7792;

        if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 != 0.0)) {
            let assign8680_e7800: f64 = (0.4 * p.p13);
            let assign8680_e7802: f64 = (assign8680_e7800 * 1.27520989);
            let assign8680_e7804: f64 = (-0.3333333333333);
            let assign8680_e7807: f64 = (locals.var_phit * locals.var_tsisq);
            let assign8680_e7808: f64 = (assign8680_e7807).ln();
            let assign8680_e7809: f64 = (assign8680_e7804 * assign8680_e7808);
            let assign8680_e7810: f64 = (assign8680_e7809).exp();
            let assign8680_e7811: f64 = (assign8680_e7802 * assign8680_e7810);
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (assign8680_e7811, (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8680_e7807)))), (assign8680_e7802 * (assign8680_e7810 * (assign8680_e7804 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8680_e7807)))), );
        }

        if (((locals.var_guard257 != 0.0) && (locals.var_guard261 != 0.0)) && (locals.var_guard262 == 0.0)) {
            let assign8690_e7822: f64 = (0.4 * p.p13);
            let assign8690_e7824: f64 = (assign8690_e7822 * 1.5412087);
            let assign8690_e7826: f64 = (-0.3333333333333);
            let assign8690_e7829: f64 = (locals.var_phit * locals.var_tsisq);
            let assign8690_e7830: f64 = (assign8690_e7829).ln();
            let assign8690_e7831: f64 = (assign8690_e7826 * assign8690_e7830);
            let assign8690_e7832: f64 = (assign8690_e7831).exp();
            let assign8690_e7833: f64 = (assign8690_e7824 * assign8690_e7832);
            (locals.var_qq, locals.var_qq_dn4, locals.var_qq_dn6, locals.var_qq_dn7, locals.var_qq_dn8, locals.var_qq_dn9, ) = (assign8690_e7833, (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn4 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn6 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn7 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn8 * locals.var_tsisq) / assign8690_e7829)))), (assign8690_e7824 * (assign8690_e7832 * (assign8690_e7826 * ((locals.var_phit_dn9 * locals.var_tsisq) / assign8690_e7829)))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8700_e7839: f64 = (p.p14 * locals.var_stvfb_i);
            let assign8700_e7841: f64 = (assign8700_e7839 * locals.var_dt);
            let assign8700_e7843: f64 = (assign8700_e7841 + locals.var_dvfbqm);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8700_e7843, (assign8700_e7839 * locals.var_dt_dn4), (assign8700_e7839 * locals.var_dt_dn6), (assign8700_e7839 * locals.var_dt_dn7), (assign8700_e7839 * locals.var_dt_dn8), (assign8700_e7839 * locals.var_dt_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8710_e7849: f64 = (locals.var_temp + p.p34);
            let assign8710_e7851: f64 = (assign8710_e7849 - locals.var_dvfbpdep);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign8710_e7851, (locals.var_temp_dn4 - locals.var_dvfbpdep_dn4), (locals.var_temp_dn6 - locals.var_dvfbpdep_dn6), (locals.var_temp_dn7 - locals.var_dvfbpdep_dn7), (locals.var_temp_dn8 - locals.var_dvfbpdep_dn8), (locals.var_temp_dn9 - locals.var_dvfbpdep_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8720_e7858: f64 = (locals.var_vfb1_t + locals.var_dvfbch);
            let assign8720_e7860: f64 = (assign8720_e7858 + locals.var_dvfb1nch);
            let assign8720_e7861: f64 = (p.p14 * assign8720_e7860);
            let assign8720_e7863: f64 = (assign8720_e7861 + locals.var_temp1);
            (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9, ) = (assign8720_e7863, ((p.p14 * ((locals.var_vfb1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfb1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfb1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfb1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfb1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9), );
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
            let assign8730_e7870: f64 = (locals.var_vfb2_t + locals.var_dvfbch);
            let assign8730_e7872: f64 = (assign8730_e7870 + locals.var_dvfb2nch);
            let assign8730_e7873: f64 = (p.p14 * assign8730_e7872);
            let assign8730_e7875: f64 = (assign8730_e7873 + locals.var_temp);
            (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9, ) = (assign8730_e7875, ((p.p14 * ((locals.var_vfb2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfb2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfb2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfb2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfb2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8740_e7882: f64 = (locals.var_vfbac1_t + locals.var_dvfbch);
            let assign8740_e7884: f64 = (assign8740_e7882 + locals.var_dvfb1nch);
            let assign8740_e7885: f64 = (p.p14 * assign8740_e7884);
            let assign8740_e7887: f64 = (assign8740_e7885 + locals.var_temp1);
            (locals.var_vfbac1_i, locals.var_vfbac1_i_dn4, locals.var_vfbac1_i_dn6, locals.var_vfbac1_i_dn7, locals.var_vfbac1_i_dn8, locals.var_vfbac1_i_dn9, ) = (assign8740_e7887, ((p.p14 * ((locals.var_vfbac1_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb1nch_dn4)) + locals.var_temp1_dn4), ((p.p14 * ((locals.var_vfbac1_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb1nch_dn6)) + locals.var_temp1_dn6), ((p.p14 * ((locals.var_vfbac1_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb1nch_dn7)) + locals.var_temp1_dn7), ((p.p14 * ((locals.var_vfbac1_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb1nch_dn8)) + locals.var_temp1_dn8), ((p.p14 * ((locals.var_vfbac1_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb1nch_dn9)) + locals.var_temp1_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8750_e7894: f64 = (locals.var_vfbac2_t + locals.var_dvfbch);
            let assign8750_e7896: f64 = (assign8750_e7894 + locals.var_dvfb2nch);
            let assign8750_e7897: f64 = (p.p14 * assign8750_e7896);
            let assign8750_e7899: f64 = (assign8750_e7897 + locals.var_temp);
            (locals.var_vfbac2_i, locals.var_vfbac2_i_dn4, locals.var_vfbac2_i_dn6, locals.var_vfbac2_i_dn7, locals.var_vfbac2_i_dn8, locals.var_vfbac2_i_dn9, ) = (assign8750_e7899, ((p.p14 * ((locals.var_vfbac2_t_dn4 + locals.var_dvfbch_dn4) + locals.var_dvfb2nch_dn4)) + locals.var_temp_dn4), ((p.p14 * ((locals.var_vfbac2_t_dn6 + locals.var_dvfbch_dn6) + locals.var_dvfb2nch_dn6)) + locals.var_temp_dn6), ((p.p14 * ((locals.var_vfbac2_t_dn7 + locals.var_dvfbch_dn7) + locals.var_dvfb2nch_dn7)) + locals.var_temp_dn7), ((p.p14 * ((locals.var_vfbac2_t_dn8 + locals.var_dvfbch_dn8) + locals.var_dvfb2nch_dn8)) + locals.var_temp_dn8), ((p.p14 * ((locals.var_vfbac2_t_dn9 + locals.var_dvfbch_dn9) + locals.var_dvfb2nch_dn9)) + locals.var_temp_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8760_e7904: f64 = (locals.var_rtn).ln();
            (locals.var_lnrtn, locals.var_lnrtn_dn4, locals.var_lnrtn_dn6, locals.var_lnrtn_dn7, locals.var_lnrtn_dn8, locals.var_lnrtn_dn9, ) = (assign8760_e7904, (locals.var_rtn_dn4 / locals.var_rtn), (locals.var_rtn_dn6 / locals.var_rtn), (locals.var_rtn_dn7 / locals.var_rtn), (locals.var_rtn_dn8 / locals.var_rtn), (locals.var_rtn_dn9 / locals.var_rtn), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8770_e7910: f64 = (locals.var_stbet_i * locals.var_lnrtn);
            let assign8770_e7911: f64 = (assign8770_e7910).exp();
            let assign8770_e7913: f64 = (assign8770_e7911 * p.p35);
            (locals.var_tf_bet, locals.var_tf_bet_dn4, locals.var_tf_bet_dn6, locals.var_tf_bet_dn7, locals.var_tf_bet_dn8, locals.var_tf_bet_dn9, ) = (assign8770_e7913, ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn4)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn6)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn7)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn8)) * p.p35), ((assign8770_e7911 * (locals.var_stbet_i * locals.var_lnrtn_dn9)) * p.p35), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8780_e7919: f64 = (locals.var_betn1_t * locals.var_tf_bet);
            (locals.var_betn1_i, locals.var_betn1_i_dn4, locals.var_betn1_i_dn6, locals.var_betn1_i_dn7, locals.var_betn1_i_dn8, locals.var_betn1_i_dn9, ) = (assign8780_e7919, ((locals.var_betn1_t_dn4 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn4)), ((locals.var_betn1_t_dn6 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn6)), ((locals.var_betn1_t_dn7 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn7)), ((locals.var_betn1_t_dn8 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn8)), ((locals.var_betn1_t_dn9 * locals.var_tf_bet) + (locals.var_betn1_t * locals.var_tf_bet_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8790_e7925: f64 = (locals.var_betn2_t * locals.var_tf_bet);
            (locals.var_betn2_i, locals.var_betn2_i_dn4, locals.var_betn2_i_dn6, locals.var_betn2_i_dn7, locals.var_betn2_i_dn8, locals.var_betn2_i_dn9, ) = (assign8790_e7925, ((locals.var_betn2_t_dn4 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn4)), ((locals.var_betn2_t_dn6 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn6)), ((locals.var_betn2_t_dn7 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn7)), ((locals.var_betn2_t_dn8 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn8)), ((locals.var_betn2_t_dn9 * locals.var_tf_bet) + (locals.var_betn2_t * locals.var_tf_bet_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8800_e7931: f64 = (locals.var_stmue_i * locals.var_lnrtn);
            let assign8800_e7932: f64 = (assign8800_e7931).exp();
            (locals.var_tf_mue, locals.var_tf_mue_dn4, locals.var_tf_mue_dn6, locals.var_tf_mue_dn7, locals.var_tf_mue_dn8, locals.var_tf_mue_dn9, ) = (assign8800_e7932, (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn4)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn6)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn7)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn8)), (assign8800_e7932 * (locals.var_stmue_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8810_e7938: f64 = (locals.var_mue_t * locals.var_tf_mue);
            (locals.var_mue_i, locals.var_mue_i_dn4, locals.var_mue_i_dn6, locals.var_mue_i_dn7, locals.var_mue_i_dn8, locals.var_mue_i_dn9, ) = (assign8810_e7938, (locals.var_mue_t * locals.var_tf_mue_dn4), (locals.var_mue_t * locals.var_tf_mue_dn6), (locals.var_mue_t * locals.var_tf_mue_dn7), (locals.var_mue_t * locals.var_tf_mue_dn8), (locals.var_mue_t * locals.var_tf_mue_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8820_e7944: f64 = (locals.var_stthemu_i * locals.var_lnrtn);
            let assign8820_e7945: f64 = (assign8820_e7944).exp();
            (locals.var_tf_themu, locals.var_tf_themu_dn4, locals.var_tf_themu_dn6, locals.var_tf_themu_dn7, locals.var_tf_themu_dn8, locals.var_tf_themu_dn9, ) = (assign8820_e7945, (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn4)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn6)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn7)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn8)), (assign8820_e7945 * (locals.var_stthemu_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8830_e7951: f64 = (locals.var_themu_t * locals.var_tf_themu);
            (locals.var_themu_i, locals.var_themu_i_dn4, locals.var_themu_i_dn6, locals.var_themu_i_dn7, locals.var_themu_i_dn8, locals.var_themu_i_dn9, ) = (assign8830_e7951, (locals.var_themu_t * locals.var_tf_themu_dn4), (locals.var_themu_t * locals.var_tf_themu_dn6), (locals.var_themu_t * locals.var_tf_themu_dn7), (locals.var_themu_t * locals.var_tf_themu_dn8), (locals.var_themu_t * locals.var_tf_themu_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8840_e7957: f64 = (locals.var_stcs_i * locals.var_lnrtn);
            let assign8840_e7958: f64 = (assign8840_e7957).exp();
            (locals.var_tf_cs, locals.var_tf_cs_dn4, locals.var_tf_cs_dn6, locals.var_tf_cs_dn7, locals.var_tf_cs_dn8, locals.var_tf_cs_dn9, ) = (assign8840_e7958, (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn4)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn6)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn7)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn8)), (assign8840_e7958 * (locals.var_stcs_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8850_e7964: f64 = (locals.var_cs_t * locals.var_tf_cs);
            (locals.var_cs_i, locals.var_cs_i_dn4, locals.var_cs_i_dn6, locals.var_cs_i_dn7, locals.var_cs_i_dn8, locals.var_cs_i_dn9, ) = (assign8850_e7964, (locals.var_cs_t * locals.var_tf_cs_dn4), (locals.var_cs_t * locals.var_tf_cs_dn6), (locals.var_cs_t * locals.var_tf_cs_dn7), (locals.var_cs_t * locals.var_tf_cs_dn8), (locals.var_cs_t * locals.var_tf_cs_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8860_e7970: f64 = (locals.var_stthecs_i * locals.var_lnrtn);
            let assign8860_e7971: f64 = (assign8860_e7970).exp();
            (locals.var_tf_thecs, locals.var_tf_thecs_dn4, locals.var_tf_thecs_dn6, locals.var_tf_thecs_dn7, locals.var_tf_thecs_dn8, locals.var_tf_thecs_dn9, ) = (assign8860_e7971, (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn4)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn6)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn7)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn8)), (assign8860_e7971 * (locals.var_stthecs_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8870_e7977: f64 = (locals.var_thecs_t * locals.var_tf_thecs);
            (locals.var_thecs_i, locals.var_thecs_i_dn4, locals.var_thecs_i_dn6, locals.var_thecs_i_dn7, locals.var_thecs_i_dn8, locals.var_thecs_i_dn9, ) = (assign8870_e7977, (locals.var_thecs_t * locals.var_tf_thecs_dn4), (locals.var_thecs_t * locals.var_tf_thecs_dn6), (locals.var_thecs_t * locals.var_tf_thecs_dn7), (locals.var_thecs_t * locals.var_tf_thecs_dn8), (locals.var_thecs_t * locals.var_tf_thecs_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8880_e7983: f64 = (locals.var_stxcor_i * locals.var_lnrtn);
            let assign8880_e7984: f64 = (assign8880_e7983).exp();
            (locals.var_tf_xcor, locals.var_tf_xcor_dn4, locals.var_tf_xcor_dn6, locals.var_tf_xcor_dn7, locals.var_tf_xcor_dn8, locals.var_tf_xcor_dn9, ) = (assign8880_e7984, (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn4)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn6)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn7)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn8)), (assign8880_e7984 * (locals.var_stxcor_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8890_e7990: f64 = (locals.var_xcor_t * locals.var_tf_xcor);
            (locals.var_xcor_i, locals.var_xcor_i_dn4, locals.var_xcor_i_dn6, locals.var_xcor_i_dn7, locals.var_xcor_i_dn8, locals.var_xcor_i_dn9, ) = (assign8890_e7990, (locals.var_xcor_t * locals.var_tf_xcor_dn4), (locals.var_xcor_t * locals.var_tf_xcor_dn6), (locals.var_xcor_t * locals.var_tf_xcor_dn7), (locals.var_xcor_t * locals.var_tf_xcor_dn8), (locals.var_xcor_t * locals.var_tf_xcor_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8900_e7996: f64 = (1e-8 * locals.var_phit);
            let assign8900_e7998: f64 = (assign8900_e7996 / locals.var_tsi_i);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign8900_e7998, ((1e-8 * locals.var_phit_dn4) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn6) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn7) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn8) / locals.var_tsi_i), ((1e-8 * locals.var_phit_dn9) / locals.var_tsi_i), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8910_e8004: f64 = (locals.var_temp * locals.var_mue_i);
            (locals.var_fmue, locals.var_fmue_dn4, locals.var_fmue_dn6, locals.var_fmue_dn7, locals.var_fmue_dn8, locals.var_fmue_dn9, ) = (assign8910_e8004, ((locals.var_temp_dn4 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn4)), ((locals.var_temp_dn6 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn6)), ((locals.var_temp_dn7 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn7)), ((locals.var_temp_dn8 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn8)), ((locals.var_temp_dn9 * locals.var_mue_i) + (locals.var_temp * locals.var_mue_i_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8920_e8010: f64 = (locals.var_strs_i * locals.var_lnrtn);
            let assign8920_e8011: f64 = (assign8920_e8010).exp();
            (locals.var_tf_ther, locals.var_tf_ther_dn4, locals.var_tf_ther_dn6, locals.var_tf_ther_dn7, locals.var_tf_ther_dn8, locals.var_tf_ther_dn9, ) = (assign8920_e8011, (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn4)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn6)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn7)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn8)), (assign8920_e8011 * (locals.var_strs_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8930_e8017: f64 = (locals.var_rs_t * locals.var_tf_ther);
            (locals.var_rs_i, locals.var_rs_i_dn4, locals.var_rs_i_dn6, locals.var_rs_i_dn7, locals.var_rs_i_dn8, locals.var_rs_i_dn9, ) = (assign8930_e8017, (locals.var_rs_t * locals.var_tf_ther_dn4), (locals.var_rs_t * locals.var_tf_ther_dn6), (locals.var_rs_t * locals.var_tf_ther_dn7), (locals.var_rs_t * locals.var_tf_ther_dn8), (locals.var_rs_t * locals.var_tf_ther_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8940_e8023: f64 = (2.0 * locals.var_rs_i);
            let assign8940_e8025: f64 = (assign8940_e8023 * locals.var_phit);
            (locals.var_frs, locals.var_frs_dn4, locals.var_frs_dn6, locals.var_frs_dn7, locals.var_frs_dn8, locals.var_frs_dn9, ) = (assign8940_e8025, (((2.0 * locals.var_rs_i_dn4) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn4)), (((2.0 * locals.var_rs_i_dn6) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn6)), (((2.0 * locals.var_rs_i_dn7) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn7)), (((2.0 * locals.var_rs_i_dn8) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn8)), (((2.0 * locals.var_rs_i_dn9) * locals.var_phit) + (assign8940_e8023 * locals.var_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8950_e8031: f64 = (locals.var_stthesat_i * locals.var_lnrtn);
            let assign8950_e8032: f64 = (assign8950_e8031).exp();
            (locals.var_tf_thesat, locals.var_tf_thesat_dn4, locals.var_tf_thesat_dn6, locals.var_tf_thesat_dn7, locals.var_tf_thesat_dn8, locals.var_tf_thesat_dn9, ) = (assign8950_e8032, (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn4)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn6)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn7)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn8)), (assign8950_e8032 * (locals.var_stthesat_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8960_e8038: f64 = (locals.var_thesat_t * locals.var_tf_thesat);
            let assign8960_e8040: f64 = (assign8960_e8038 * locals.var_tf_bet);
            (locals.var_thesat_i, locals.var_thesat_i_dn4, locals.var_thesat_i_dn6, locals.var_thesat_i_dn7, locals.var_thesat_i_dn8, locals.var_thesat_i_dn9, ) = (assign8960_e8040, ((((locals.var_thesat_t_dn4 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn4)), ((((locals.var_thesat_t_dn6 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn6)), ((((locals.var_thesat_t_dn7 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn7)), ((((locals.var_thesat_t_dn8 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn8)), ((((locals.var_thesat_t_dn9 * locals.var_tf_thesat) + (locals.var_thesat_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign8960_e8038 * locals.var_tf_bet_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8970_e8046: f64 = (locals.var_thesat_i * locals.var_phit);
            (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9, ) = (assign8970_e8046, ((locals.var_thesat_i_dn4 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn4)), ((locals.var_thesat_i_dn6 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn6)), ((locals.var_thesat_i_dn7 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn7)), ((locals.var_thesat_i_dn8 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn8)), ((locals.var_thesat_i_dn9 * locals.var_phit) + (locals.var_thesat_i * locals.var_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8980_e8052: f64 = (locals.var_thesatac_t * locals.var_tf_thesat);
            let assign8980_e8054: f64 = (assign8980_e8052 * locals.var_tf_bet);
            (locals.var_thesatac_i, locals.var_thesatac_i_dn4, locals.var_thesatac_i_dn6, locals.var_thesatac_i_dn7, locals.var_thesatac_i_dn8, locals.var_thesatac_i_dn9, ) = (assign8980_e8054, ((((locals.var_thesatac_t_dn4 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn4)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn4)), ((((locals.var_thesatac_t_dn6 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn6)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn6)), ((((locals.var_thesatac_t_dn7 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn7)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn7)), ((((locals.var_thesatac_t_dn8 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn8)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn8)), ((((locals.var_thesatac_t_dn9 * locals.var_tf_thesat) + (locals.var_thesatac_t * locals.var_tf_thesat_dn9)) * locals.var_tf_bet) + (assign8980_e8052 * locals.var_tf_bet_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign8990_e8060: f64 = (locals.var_thesatac_i * locals.var_phit);
            (locals.var_sat_phit_ac, locals.var_sat_phit_ac_dn4, locals.var_sat_phit_ac_dn6, locals.var_sat_phit_ac_dn7, locals.var_sat_phit_ac_dn8, locals.var_sat_phit_ac_dn9, ) = (assign8990_e8060, ((locals.var_thesatac_i_dn4 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn4)), ((locals.var_thesatac_i_dn6 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn6)), ((locals.var_thesatac_i_dn7 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn7)), ((locals.var_thesatac_i_dn8 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn8)), ((locals.var_thesatac_i_dn9 * locals.var_phit) + (locals.var_thesatac_i * locals.var_phit_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9000_e8066: f64 = (locals.var_alp1_i * locals.var_inv_phit);
            (locals.var_alp1_phit, locals.var_alp1_phit_dn4, locals.var_alp1_phit_dn6, locals.var_alp1_phit_dn7, locals.var_alp1_phit_dn8, locals.var_alp1_phit_dn9, ) = (assign9000_e8066, (locals.var_alp1_i * locals.var_inv_phit_dn4), (locals.var_alp1_i * locals.var_inv_phit_dn6), (locals.var_alp1_i * locals.var_inv_phit_dn7), (locals.var_alp1_i * locals.var_inv_phit_dn8), (locals.var_alp1_i * locals.var_inv_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9010_e8071: f64 = (-locals.var_stig_i);
            let assign9010_e8073: f64 = (assign9010_e8071 * locals.var_lnrtn);
            let assign9010_e8074: f64 = (assign9010_e8073).exp();
            (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9, ) = (assign9010_e8074, (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn4)), (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn6)), (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn7)), (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn8)), (assign9010_e8074 * (assign9010_e8071 * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9020_e8080: f64 = (locals.var_iginv_t * locals.var_tf_ig);
            (locals.var_iginv_i, locals.var_iginv_i_dn4, locals.var_iginv_i_dn6, locals.var_iginv_i_dn7, locals.var_iginv_i_dn8, locals.var_iginv_i_dn9, ) = (assign9020_e8080, (locals.var_iginv_t * locals.var_tf_ig_dn4), (locals.var_iginv_t * locals.var_tf_ig_dn6), (locals.var_iginv_t * locals.var_tf_ig_dn7), (locals.var_iginv_t * locals.var_tf_ig_dn8), (locals.var_iginv_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9030_e8086: f64 = (locals.var_igovinv_t * locals.var_tf_ig);
            (locals.var_igovinv_i, locals.var_igovinv_i_dn4, locals.var_igovinv_i_dn6, locals.var_igovinv_i_dn7, locals.var_igovinv_i_dn8, locals.var_igovinv_i_dn9, ) = (assign9030_e8086, (locals.var_igovinv_t * locals.var_tf_ig_dn4), (locals.var_igovinv_t * locals.var_tf_ig_dn6), (locals.var_igovinv_t * locals.var_tf_ig_dn7), (locals.var_igovinv_t * locals.var_tf_ig_dn8), (locals.var_igovinv_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9040_e8092: f64 = (locals.var_igovinvd_t * locals.var_tf_ig);
            (locals.var_igovinvd_i, locals.var_igovinvd_i_dn4, locals.var_igovinvd_i_dn6, locals.var_igovinvd_i_dn7, locals.var_igovinvd_i_dn8, locals.var_igovinvd_i_dn9, ) = (assign9040_e8092, (locals.var_igovinvd_t * locals.var_tf_ig_dn4), (locals.var_igovinvd_t * locals.var_tf_ig_dn6), (locals.var_igovinvd_t * locals.var_tf_ig_dn7), (locals.var_igovinvd_t * locals.var_tf_ig_dn8), (locals.var_igovinvd_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9050_e8098: f64 = (locals.var_igovacc_t * locals.var_tf_ig);
            (locals.var_igovacc_i, locals.var_igovacc_i_dn4, locals.var_igovacc_i_dn6, locals.var_igovacc_i_dn7, locals.var_igovacc_i_dn8, locals.var_igovacc_i_dn9, ) = (assign9050_e8098, (locals.var_igovacc_t * locals.var_tf_ig_dn4), (locals.var_igovacc_t * locals.var_tf_ig_dn6), (locals.var_igovacc_t * locals.var_tf_ig_dn7), (locals.var_igovacc_t * locals.var_tf_ig_dn8), (locals.var_igovacc_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9060_e8104: f64 = (locals.var_igovaccd_t * locals.var_tf_ig);
            (locals.var_igovaccd_i, locals.var_igovaccd_i_dn4, locals.var_igovaccd_i_dn6, locals.var_igovaccd_i_dn7, locals.var_igovaccd_i_dn8, locals.var_igovaccd_i_dn9, ) = (assign9060_e8104, (locals.var_igovaccd_t * locals.var_tf_ig_dn4), (locals.var_igovaccd_t * locals.var_tf_ig_dn6), (locals.var_igovaccd_t * locals.var_tf_ig_dn7), (locals.var_igovaccd_t * locals.var_tf_ig_dn8), (locals.var_igovaccd_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9070_e8109: f64 = (-locals.var_stigfn_i);
            let assign9070_e8111: f64 = (assign9070_e8109 * locals.var_lnrtn);
            let assign9070_e8112: f64 = (assign9070_e8111).exp();
            (locals.var_tf_ig, locals.var_tf_ig_dn4, locals.var_tf_ig_dn6, locals.var_tf_ig_dn7, locals.var_tf_ig_dn8, locals.var_tf_ig_dn9, ) = (assign9070_e8112, (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn4)), (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn6)), (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn7)), (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn8)), (assign9070_e8112 * (assign9070_e8109 * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9080_e8118: f64 = (locals.var_fnovinv_t * locals.var_tf_ig);
            (locals.var_fnovinv_i, locals.var_fnovinv_i_dn4, locals.var_fnovinv_i_dn6, locals.var_fnovinv_i_dn7, locals.var_fnovinv_i_dn8, locals.var_fnovinv_i_dn9, ) = (assign9080_e8118, (locals.var_fnovinv_t * locals.var_tf_ig_dn4), (locals.var_fnovinv_t * locals.var_tf_ig_dn6), (locals.var_fnovinv_t * locals.var_tf_ig_dn7), (locals.var_fnovinv_t * locals.var_tf_ig_dn8), (locals.var_fnovinv_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9090_e8124: f64 = (locals.var_fnovinvd_t * locals.var_tf_ig);
            (locals.var_fnovinvd_i, locals.var_fnovinvd_i_dn4, locals.var_fnovinvd_i_dn6, locals.var_fnovinvd_i_dn7, locals.var_fnovinvd_i_dn8, locals.var_fnovinvd_i_dn9, ) = (assign9090_e8124, (locals.var_fnovinvd_t * locals.var_tf_ig_dn4), (locals.var_fnovinvd_t * locals.var_tf_ig_dn6), (locals.var_fnovinvd_t * locals.var_tf_ig_dn7), (locals.var_fnovinvd_t * locals.var_tf_ig_dn8), (locals.var_fnovinvd_t * locals.var_tf_ig_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9100_e8130: f64 = (0.5 * locals.var_eg);
            (locals.var_alpha_b, locals.var_alpha_b_dn4, locals.var_alpha_b_dn6, locals.var_alpha_b_dn7, locals.var_alpha_b_dn8, locals.var_alpha_b_dn9, ) = (assign9100_e8130, (0.5 * locals.var_eg_dn4), (0.5 * locals.var_eg_dn6), (0.5 * locals.var_eg_dn7), (0.5 * locals.var_eg_dn8), (0.5 * locals.var_eg_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9110_e8136: f64 = (locals.var_gco_i * locals.var_phit);
            (locals.var_dch, locals.var_dch_dn4, locals.var_dch_dn6, locals.var_dch_dn7, locals.var_dch_dn8, locals.var_dch_dn9, ) = (assign9110_e8136, (locals.var_gco_i * locals.var_phit_dn4), (locals.var_gco_i * locals.var_phit_dn6), (locals.var_gco_i * locals.var_phit_dn7), (locals.var_gco_i * locals.var_phit_dn8), (locals.var_gco_i * locals.var_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9120_e8142: f64 = (locals.var_gco_i * locals.var_phit0);
            (locals.var_dov, locals.var_dov_dn4, locals.var_dov_dn6, locals.var_dov_dn7, locals.var_dov_dn8, locals.var_dov_dn9, ) = (assign9120_e8142, (locals.var_gco_i * locals.var_phit0_dn4), (locals.var_gco_i * locals.var_phit0_dn6), (locals.var_gco_i * locals.var_phit0_dn7), (locals.var_gco_i * locals.var_phit0_dn8), (locals.var_gco_i * locals.var_phit0_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9130_e8150: f64 = (locals.var_niginv_i * locals.var_eg_2phit);
            let assign9130_e8151: f64 = (1.0 + assign9130_e8150);
            let assign9130_e8152: f64 = (1.0 / assign9130_e8151);
            (locals.var_n_iginv, locals.var_n_iginv_dn4, locals.var_n_iginv_dn6, locals.var_n_iginv_dn7, locals.var_n_iginv_dn8, locals.var_n_iginv_dn9, ) = (assign9130_e8152, (-((locals.var_niginv_i * locals.var_eg_2phit_dn4) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn6) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn7) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn8) / (assign9130_e8151 * assign9130_e8151))), (-((locals.var_niginv_i * locals.var_eg_2phit_dn9) / (assign9130_e8151 * assign9130_e8151))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9140_e8158: f64 = (locals.var_toxp_i * 500000000.0);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign9140_e8158, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9150_e8166: f64 = (locals.var_stbgidl_i * locals.var_dt);
            let assign9150_e8167: f64 = (1.0 + assign9150_e8166);
            let assign9150_e8169: f64 = assign9150_e8167;
            let assign9150_e8173: f64 = (locals.var_stbgidl_i * locals.var_dt);
            let assign9150_e8174: f64 = (1.0 + assign9150_e8173);
            let assign9150_e8176: f64 = assign9150_e8174;
            let assign9150_e8180: f64 = (locals.var_stbgidl_i * locals.var_dt);
            let assign9150_e8181: f64 = (1.0 + assign9150_e8180);
            let assign9150_e8183: f64 = assign9150_e8181;
            let assign9150_e8184: f64 = (assign9150_e8176 * assign9150_e8183);
            let assign9150_e8186: f64 = (assign9150_e8184 + 0.01);
            let assign9150_e8187: f64 = (assign9150_e8186).sqrt();
            let assign9150_e8188: f64 = (assign9150_e8169 + assign9150_e8187);
            let assign9150_e8189: f64 = (0.5 * assign9150_e8188);
            (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign9150_e8189, (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn4) + ((((locals.var_stbgidl_i * locals.var_dt_dn4) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn4))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn6) + ((((locals.var_stbgidl_i * locals.var_dt_dn6) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn6))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn7) + ((((locals.var_stbgidl_i * locals.var_dt_dn7) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn7))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn8) + ((((locals.var_stbgidl_i * locals.var_dt_dn8) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn8))) / (2.0 * assign9150_e8187)))), (0.5 * ((locals.var_stbgidl_i * locals.var_dt_dn9) + ((((locals.var_stbgidl_i * locals.var_dt_dn9) * assign9150_e8183) + (assign9150_e8176 * (locals.var_stbgidl_i * locals.var_dt_dn9))) / (2.0 * assign9150_e8187)))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9160_e8195: f64 = (locals.var_bgidl_t * locals.var_tempm);
            let assign9160_e8197: f64 = (assign9160_e8195 * locals.var_temp);
            (locals.var_bgidl_i, locals.var_bgidl_i_dn4, locals.var_bgidl_i_dn6, locals.var_bgidl_i_dn7, locals.var_bgidl_i_dn8, locals.var_bgidl_i_dn9, ) = (assign9160_e8197, (((locals.var_bgidl_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn4)), (((locals.var_bgidl_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn6)), (((locals.var_bgidl_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn7)), (((locals.var_bgidl_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn8)), (((locals.var_bgidl_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9160_e8195 * locals.var_temp_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9170_e8205: f64 = (locals.var_stbgidld_i * locals.var_dt);
            let assign9170_e8206: f64 = (1.0 + assign9170_e8205);
            let assign9170_e8208: f64 = assign9170_e8206;
            let assign9170_e8212: f64 = (locals.var_stbgidld_i * locals.var_dt);
            let assign9170_e8213: f64 = (1.0 + assign9170_e8212);
            let assign9170_e8215: f64 = assign9170_e8213;
            let assign9170_e8219: f64 = (locals.var_stbgidld_i * locals.var_dt);
            let assign9170_e8220: f64 = (1.0 + assign9170_e8219);
            let assign9170_e8222: f64 = assign9170_e8220;
            let assign9170_e8223: f64 = (assign9170_e8215 * assign9170_e8222);
            let assign9170_e8225: f64 = (assign9170_e8223 + 0.01);
            let assign9170_e8226: f64 = (assign9170_e8225).sqrt();
            let assign9170_e8227: f64 = (assign9170_e8208 + assign9170_e8226);
            let assign9170_e8228: f64 = (0.5 * assign9170_e8227);
            (locals.var_tempm, locals.var_tempm_dn4, locals.var_tempm_dn6, locals.var_tempm_dn7, locals.var_tempm_dn8, locals.var_tempm_dn9, ) = (assign9170_e8228, (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn4) + ((((locals.var_stbgidld_i * locals.var_dt_dn4) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn4))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn6) + ((((locals.var_stbgidld_i * locals.var_dt_dn6) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn6))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn7) + ((((locals.var_stbgidld_i * locals.var_dt_dn7) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn7))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn8) + ((((locals.var_stbgidld_i * locals.var_dt_dn8) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn8))) / (2.0 * assign9170_e8226)))), (0.5 * ((locals.var_stbgidld_i * locals.var_dt_dn9) + ((((locals.var_stbgidld_i * locals.var_dt_dn9) * assign9170_e8222) + (assign9170_e8215 * (locals.var_stbgidld_i * locals.var_dt_dn9))) / (2.0 * assign9170_e8226)))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9180_e8234: f64 = (locals.var_bgidld_t * locals.var_tempm);
            let assign9180_e8236: f64 = (assign9180_e8234 * locals.var_temp);
            (locals.var_bgidld_i, locals.var_bgidld_i_dn4, locals.var_bgidld_i_dn6, locals.var_bgidld_i_dn7, locals.var_bgidld_i_dn8, locals.var_bgidld_i_dn9, ) = (assign9180_e8236, (((locals.var_bgidld_t * locals.var_tempm_dn4) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn4)), (((locals.var_bgidld_t * locals.var_tempm_dn6) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn6)), (((locals.var_bgidld_t * locals.var_tempm_dn7) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn7)), (((locals.var_bgidld_t * locals.var_tempm_dn8) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn8)), (((locals.var_bgidld_t * locals.var_tempm_dn9) * locals.var_temp) + (assign9180_e8234 * locals.var_temp_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9190_e8242: f64 = (-locals.var_sta2_i);
            let assign9190_e8244: f64 = (assign9190_e8242 * locals.var_lnrtn);
            let assign9190_e8245: f64 = (assign9190_e8244).exp();
            let assign9190_e8246: f64 = (locals.var_a2_t * assign9190_e8245);
            (locals.var_a2_i, locals.var_a2_i_dn4, locals.var_a2_i_dn6, locals.var_a2_i_dn7, locals.var_a2_i_dn8, locals.var_a2_i_dn9, ) = (assign9190_e8246, (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn4))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn6))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn7))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn8))), (locals.var_a2_t * (assign9190_e8245 * (assign9190_e8242 * locals.var_lnrtn_dn9))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9200_e8252: f64 = (locals.var_areaq_i * locals.var_phit);
            (locals.var_area_phit, locals.var_area_phit_dn4, locals.var_area_phit_dn6, locals.var_area_phit_dn7, locals.var_area_phit_dn8, locals.var_area_phit_dn9, ) = (assign9200_e8252, (locals.var_areaq_i * locals.var_phit_dn4), (locals.var_areaq_i * locals.var_phit_dn6), (locals.var_areaq_i * locals.var_phit_dn7), (locals.var_areaq_i * locals.var_phit_dn8), (locals.var_areaq_i * locals.var_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9210_e8258: f64 = (0.25 * 1.602176565e-19);
            let assign9210_e8260: f64 = (assign9210_e8258 * locals.var_nsdac_i);
            let assign9210_e8263: f64 = (locals.var_epsch * locals.var_phit);
            let assign9210_e8264: f64 = (assign9210_e8260 / assign9210_e8263);
            (locals.var_inner_sd, locals.var_inner_sd_dn4, locals.var_inner_sd_dn6, locals.var_inner_sd_dn7, locals.var_inner_sd_dn8, locals.var_inner_sd_dn9, ) = (assign9210_e8264, (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn4)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn6)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn7)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn8)) / (assign9210_e8263 * assign9210_e8263))), (-((assign9210_e8260 * (locals.var_epsch * locals.var_phit_dn9)) / (assign9210_e8263 * assign9210_e8263))), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9220_e8270: f64 = (locals.var_nsdac_i / locals.var_neff);
            let assign9220_e8271: f64 = (assign9220_e8270).ln();
            (locals.var_xsd, locals.var_xsd_dn4, locals.var_xsd_dn6, locals.var_xsd_dn7, locals.var_xsd_dn8, locals.var_xsd_dn9, ) = (assign9220_e8271, ((-((locals.var_nsdac_i * locals.var_neff_dn4) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn6) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn7) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn8) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), ((-((locals.var_nsdac_i * locals.var_neff_dn9) / (locals.var_neff * locals.var_neff))) / assign9220_e8270), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9230_e8277: f64 = (locals.var_fif_i * 1.25e-6);
            let assign9230_e8279: f64 = (assign9230_e8277 * locals.var_phit);
            (locals.var_fif_phit, locals.var_fif_phit_dn4, locals.var_fif_phit_dn6, locals.var_fif_phit_dn7, locals.var_fif_phit_dn8, locals.var_fif_phit_dn9, ) = (assign9230_e8279, (assign9230_e8277 * locals.var_phit_dn4), (assign9230_e8277 * locals.var_phit_dn6), (assign9230_e8277 * locals.var_phit_dn7), (assign9230_e8277 * locals.var_phit_dn8), (assign9230_e8277 * locals.var_phit_dn9), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9240_e8285: f64 = (locals.var_strth_i * locals.var_lnrtn);
            let assign9240_e8286: f64 = (assign9240_e8285).exp();
            (locals.var_tf_rth, locals.var_tf_rth_dn4, locals.var_tf_rth_dn6, locals.var_tf_rth_dn7, locals.var_tf_rth_dn8, locals.var_tf_rth_dn9, ) = (assign9240_e8286, (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn4)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn6)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn7)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn8)), (assign9240_e8286 * (locals.var_strth_i * locals.var_lnrtn_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9250_e8292: f64 = (locals.var_rth_t * locals.var_tf_rth);
            (locals.var_rth_i, locals.var_rth_i_dn4, locals.var_rth_i_dn6, locals.var_rth_i_dn7, locals.var_rth_i_dn8, locals.var_rth_i_dn9, ) = (assign9250_e8292, ((locals.var_rth_t_dn4 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn4)), ((locals.var_rth_t_dn6 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn6)), ((locals.var_rth_t_dn7 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn7)), ((locals.var_rth_t_dn8 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn8)), ((locals.var_rth_t_dn9 * locals.var_tf_rth) + (locals.var_rth_t * locals.var_tf_rth_dn9)), );
        }

        if (locals.var_guard257 != 0.0) {
            let assign9260_e8298: f64 = (4.0 * 1.3806488e-23);
            let assign9260_e8300: f64 = (assign9260_e8298 * locals.var_tkc);
            locals.var_nt0_4kt = assign9260_e8300;
        }

        if (locals.var_guard257 != 0.0) {
            let assign9270_e8306: f64 = (locals.var_fnt_i * locals.var_nt0_4kt);
            locals.var_nt = assign9270_e8306;
        }

        let assign9280_e8311: f64 = 1.0;
        let assign9280_e8312: f64 = if p.p14 == assign9280_e8311 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign9280_e8312;

        if (locals.var_guard263 != 0.0) {
            (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9, ) = ((nv9 - nv6), -1.0, 1.0, );
            (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7, ) = ((nv7 - nv6), -1.0, 1.0, );
            (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8, ) = ((nv6 - nv8), 1.0, -1.0, );
        }

        if (locals.var_guard263 == 0.0) {
            let assign9320_e8328: f64 = (-(nv9 - nv6));
            (locals.var_vgsu, locals.var_vgsu_dn6, locals.var_vgsu_dn9, ) = (assign9320_e8328, 1.0, (-1.0), );
        }

        if (locals.var_guard263 == 0.0) {
            let assign9330_e8334: f64 = (-(nv7 - nv6));
            (locals.var_vdsu, locals.var_vdsu_dn6, locals.var_vdsu_dn7, ) = (assign9330_e8334, 1.0, (-1.0), );
        }

        if (locals.var_guard263 == 0.0) {
            let assign9340_e8340: f64 = (-(nv6 - nv8));
            (locals.var_vsbu, locals.var_vsbu_dn6, locals.var_vsbu_dn8, ) = (assign9340_e8340, (-1.0), 1.0, );
        }

        let assign9350_e8344: f64 = (-locals.var_vdsu);
        (locals.var_vsdu, locals.var_vsdu_dn6, locals.var_vsdu_dn7, ) = (assign9350_e8344, (-locals.var_vdsu_dn6), (-locals.var_vdsu_dn7), );

        let assign9360_e8347: f64 = (locals.var_vgsu + locals.var_vsdu);
        (locals.var_vgdu, locals.var_vgdu_dn6, locals.var_vgdu_dn7, locals.var_vgdu_dn9, ) = (assign9360_e8347, (locals.var_vgsu_dn6 + locals.var_vsdu_dn6), locals.var_vsdu_dn7, locals.var_vgsu_dn9, );

        let assign9370_e8350: f64 = (locals.var_vdsu + locals.var_vsbu);
        (locals.var_vdbu, locals.var_vdbu_dn6, locals.var_vdbu_dn7, locals.var_vdbu_dn8, ) = (assign9370_e8350, (locals.var_vdsu_dn6 + locals.var_vsbu_dn6), locals.var_vdsu_dn7, locals.var_vsbu_dn8, );

        let assign9380_e8353: f64 = if locals.var_vdsu < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign9380_e8353;

        if (locals.var_guard264 != 0.0) {
            let assign9390_e8356: f64 = (-1.0);
            locals.var_sigvds = assign9390_e8356;
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

        let assign9470_e8393: f64 = (locals.var_vgs + locals.var_vsb);
        (locals.var_vgb, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9, ) = (assign9470_e8393, (locals.var_vgs_dn6 + locals.var_vsb_dn6), (locals.var_vgs_dn7 + locals.var_vsb_dn7), locals.var_vsb_dn8, locals.var_vgs_dn9, );

        let assign9480_e8396: f64 = (locals.var_vds * locals.var_inv_phit);
        (locals.var_xd, locals.var_xd_dn4, locals.var_xd_dn6, locals.var_xd_dn7, locals.var_xd_dn8, locals.var_xd_dn9, ) = (assign9480_e8396, (locals.var_vds * locals.var_inv_phit_dn4), ((locals.var_vds_dn6 * locals.var_inv_phit) + (locals.var_vds * locals.var_inv_phit_dn6)), ((locals.var_vds_dn7 * locals.var_inv_phit) + (locals.var_vds * locals.var_inv_phit_dn7)), (locals.var_vds * locals.var_inv_phit_dn8), (locals.var_vds * locals.var_inv_phit_dn9), );

        let assign9490_e8399: f64 = (locals.var_vds * locals.var_vds);
        let assign9490_e8401: f64 = (assign9490_e8399 + 0.01);
        let assign9490_e8402: f64 = (assign9490_e8401).sqrt();
        let assign9490_e8404: f64 = (assign9490_e8402 - 0.1);
        let assign9490_e8406: f64 = (assign9490_e8404 * locals.var_inv_phit);
        (locals.var_xdsx, locals.var_xdsx_dn4, locals.var_xdsx_dn6, locals.var_xdsx_dn7, locals.var_xdsx_dn8, locals.var_xdsx_dn9, ) = (assign9490_e8406, (assign9490_e8404 * locals.var_inv_phit_dn4), (((((locals.var_vds_dn6 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn6)) / (2.0 * assign9490_e8402)) * locals.var_inv_phit) + (assign9490_e8404 * locals.var_inv_phit_dn6)), (((((locals.var_vds_dn7 * locals.var_vds) + (locals.var_vds * locals.var_vds_dn7)) / (2.0 * assign9490_e8402)) * locals.var_inv_phit) + (assign9490_e8404 * locals.var_inv_phit_dn7)), (assign9490_e8404 * locals.var_inv_phit_dn8), (assign9490_e8404 * locals.var_inv_phit_dn9), );

        let assign9500_e8410: f64 = (locals.var_xd - locals.var_xdsx);
        let assign9500_e8411: f64 = (0.5 * assign9500_e8410);
        (locals.var_dxdsx, locals.var_dxdsx_dn4, locals.var_dxdsx_dn6, locals.var_dxdsx_dn7, locals.var_dxdsx_dn8, locals.var_dxdsx_dn9, ) = (assign9500_e8411, (0.5 * (locals.var_xd_dn4 - locals.var_xdsx_dn4)), (0.5 * (locals.var_xd_dn6 - locals.var_xdsx_dn6)), (0.5 * (locals.var_xd_dn7 - locals.var_xdsx_dn7)), (0.5 * (locals.var_xd_dn8 - locals.var_xdsx_dn8)), (0.5 * (locals.var_xd_dn9 - locals.var_xdsx_dn9)), );

        (locals.var_vfb1_loc, locals.var_vfb1_loc_dn4, locals.var_vfb1_loc_dn6, locals.var_vfb1_loc_dn7, locals.var_vfb1_loc_dn8, locals.var_vfb1_loc_dn9, ) = (locals.var_vfb1_i, locals.var_vfb1_i_dn4, locals.var_vfb1_i_dn6, locals.var_vfb1_i_dn7, locals.var_vfb1_i_dn8, locals.var_vfb1_i_dn9, );

        (locals.var_vfb2_loc, locals.var_vfb2_loc_dn4, locals.var_vfb2_loc_dn6, locals.var_vfb2_loc_dn7, locals.var_vfb2_loc_dn8, locals.var_vfb2_loc_dn9, ) = (locals.var_vfb2_i, locals.var_vfb2_i_dn4, locals.var_vfb2_i_dn6, locals.var_vfb2_i_dn7, locals.var_vfb2_i_dn8, locals.var_vfb2_i_dn9, );

        locals.var_psce1_loc = locals.var_psce1_i;

        locals.var_psce2_loc = locals.var_psce2_i;

        (locals.var_cf1_loc, locals.var_cf1_loc_dn4, locals.var_cf1_loc_dn6, locals.var_cf1_loc_dn7, locals.var_cf1_loc_dn8, locals.var_cf1_loc_dn9, ) = (locals.var_cf1_i, locals.var_cf1_i_dn4, locals.var_cf1_i_dn6, locals.var_cf1_i_dn7, locals.var_cf1_i_dn8, locals.var_cf1_i_dn9, );

        (locals.var_cf2_loc, locals.var_cf2_loc_dn4, locals.var_cf2_loc_dn6, locals.var_cf2_loc_dn7, locals.var_cf2_loc_dn8, locals.var_cf2_loc_dn9, ) = (locals.var_cf2_i, locals.var_cf2_i_dn4, locals.var_cf2_i_dn6, locals.var_cf2_i_dn7, locals.var_cf2_i_dn8, locals.var_cf2_i_dn9, );

        (locals.var_sat_phit_loc, locals.var_sat_phit_loc_dn4, locals.var_sat_phit_loc_dn6, locals.var_sat_phit_loc_dn7, locals.var_sat_phit_loc_dn8, locals.var_sat_phit_loc_dn9, ) = (locals.var_sat_phit, locals.var_sat_phit_dn4, locals.var_sat_phit_dn6, locals.var_sat_phit_dn7, locals.var_sat_phit_dn8, locals.var_sat_phit_dn9, );

        locals.var_gamax_loc = locals.var_gamax;

        locals.var_alp_loc = locals.var_alp_i;

        let assign9600_e8423: f64 = (locals.var_vgs - locals.var_vfb1_loc);
        let assign9600_e8425: f64 = (assign9600_e8423 * locals.var_inv_phit);
        let assign9600_e8427: f64 = (assign9600_e8425 - locals.var_dxdsx);
        let assign9600_e8429: f64 = (assign9600_e8427 - locals.var_eg_2phit0);
        (locals.var_xg10, locals.var_xg10_dn4, locals.var_xg10_dn6, locals.var_xg10_dn7, locals.var_xg10_dn8, locals.var_xg10_dn9, ) = (assign9600_e8429, (((((-locals.var_vfb1_loc_dn4) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4) - locals.var_eg_2phit0_dn4), (((((locals.var_vgs_dn6 - locals.var_vfb1_loc_dn6) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6) - locals.var_eg_2phit0_dn6), (((((locals.var_vgs_dn7 - locals.var_vfb1_loc_dn7) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7) - locals.var_eg_2phit0_dn7), (((((-locals.var_vfb1_loc_dn8) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8) - locals.var_eg_2phit0_dn8), (((((locals.var_vgs_dn9 - locals.var_vfb1_loc_dn9) * locals.var_inv_phit) + (assign9600_e8423 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9) - locals.var_eg_2phit0_dn9), );

        let assign9610_e8431: f64 = (-locals.var_vsb);
        let assign9610_e8433: f64 = (assign9610_e8431 - locals.var_vfb2_loc);
        let assign9610_e8435: f64 = (assign9610_e8433 * locals.var_inv_phit);
        let assign9610_e8437: f64 = (assign9610_e8435 - locals.var_dxdsx);
        (locals.var_xg20shift, locals.var_xg20shift_dn4, locals.var_xg20shift_dn6, locals.var_xg20shift_dn7, locals.var_xg20shift_dn8, locals.var_xg20shift_dn9, ) = (assign9610_e8437, ((((-locals.var_vfb2_loc_dn4) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn4)) - locals.var_dxdsx_dn4), (((((-locals.var_vsb_dn6) - locals.var_vfb2_loc_dn6) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn6)) - locals.var_dxdsx_dn6), (((((-locals.var_vsb_dn7) - locals.var_vfb2_loc_dn7) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn7)) - locals.var_dxdsx_dn7), (((((-locals.var_vsb_dn8) - locals.var_vfb2_loc_dn8) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn8)) - locals.var_dxdsx_dn8), ((((-locals.var_vfb2_loc_dn9) * locals.var_inv_phit) + (assign9610_e8433 * locals.var_inv_phit_dn9)) - locals.var_dxdsx_dn9), );

        let assign9620_e8440: f64 = (locals.var_xg20shift - locals.var_eg_2phit0);
        (locals.var_xg20, locals.var_xg20_dn4, locals.var_xg20_dn6, locals.var_xg20_dn7, locals.var_xg20_dn8, locals.var_xg20_dn9, ) = (assign9620_e8440, (locals.var_xg20shift_dn4 - locals.var_eg_2phit0_dn4), (locals.var_xg20shift_dn6 - locals.var_eg_2phit0_dn6), (locals.var_xg20shift_dn7 - locals.var_eg_2phit0_dn7), (locals.var_xg20shift_dn8 - locals.var_eg_2phit0_dn8), (locals.var_xg20shift_dn9 - locals.var_eg_2phit0_dn9), );

        let assign9630_e8443: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard531 = assign9630_e8443;

        if (locals.var_guard531 != 0.0) {
            let assign9640_e8447: f64 = (p.p14 * locals.var_typesub_i);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign9640_e8447, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9650_e8453: f64 = (1.0 + locals.var_k1_1d);
            let assign9650_e8456: f64 = (1.0 + locals.var_k2_1d);
            let assign9650_e8457: f64 = (assign9650_e8453 / assign9650_e8456);
            (locals.var_exp_dxth, locals.var_exp_dxth_dn4, locals.var_exp_dxth_dn6, locals.var_exp_dxth_dn7, locals.var_exp_dxth_dn8, locals.var_exp_dxth_dn9, ) = (assign9650_e8457, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9660_e8462: f64 = (locals.var_exp_dxth).ln();
            (locals.var_dxth, locals.var_dxth_dn4, locals.var_dxth_dn6, locals.var_dxth_dn7, locals.var_dxth_dn8, locals.var_dxth_dn9, ) = (assign9660_e8462, (locals.var_exp_dxth_dn4 / locals.var_exp_dxth), (locals.var_exp_dxth_dn6 / locals.var_exp_dxth), (locals.var_exp_dxth_dn7 / locals.var_exp_dxth), (locals.var_exp_dxth_dn8 / locals.var_exp_dxth), (locals.var_exp_dxth_dn9 / locals.var_exp_dxth), );
        }

        let assign9670_e8467: f64 = if locals.var_dxth > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard532 = assign9670_e8467;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard532 != 0.0)) {
            let assign9680_e8473: f64 = (2.0 * locals.var_dxth);
            let assign9680_e8476: f64 = (locals.var_exp_dxth + 1.0);
            let assign9680_e8477: f64 = (assign9680_e8473 * assign9680_e8476);
            let assign9680_e8480: f64 = (locals.var_exp_dxth - 1.0);
            let assign9680_e8481: f64 = (assign9680_e8477 / assign9680_e8480);
            (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9, ) = (assign9680_e8481, ((((((2.0 * locals.var_dxth_dn4) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn4)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn4)) / (assign9680_e8480 * assign9680_e8480)), ((((((2.0 * locals.var_dxth_dn6) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn6)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn6)) / (assign9680_e8480 * assign9680_e8480)), ((((((2.0 * locals.var_dxth_dn7) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn7)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn7)) / (assign9680_e8480 * assign9680_e8480)), ((((((2.0 * locals.var_dxth_dn8) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn8)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn8)) / (assign9680_e8480 * assign9680_e8480)), ((((((2.0 * locals.var_dxth_dn9) * assign9680_e8476) + (assign9680_e8473 * locals.var_exp_dxth_dn9)) * assign9680_e8480) - (assign9680_e8477 * locals.var_exp_dxth_dn9)) / (assign9680_e8480 * assign9680_e8480)), );
        }

        if ((locals.var_guard531 != 0.0) && (locals.var_guard532 == 0.0)) {
            let assign9690_e8491: f64 = (2.0 + locals.var_dxth);
            let assign9690_e8492: f64 = (2.0 * assign9690_e8491);
            (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9, ) = (assign9690_e8492, (2.0 * locals.var_dxth_dn4), (2.0 * locals.var_dxth_dn6), (2.0 * locals.var_dxth_dn7), (2.0 * locals.var_dxth_dn8), (2.0 * locals.var_dxth_dn9), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9700_e8499: f64 = (locals.var_csiprime_0 * locals.var_csiprime_0);
            let assign9700_e8500: f64 = (locals.var_a0_csisq / assign9700_e8499);
            (locals.var_a0, locals.var_a0_dn4, locals.var_a0_dn6, locals.var_a0_dn7, locals.var_a0_dn8, locals.var_a0_dn9, ) = (assign9700_e8500, (locals.var_a0_csisq_dn4 / assign9700_e8499), (locals.var_a0_csisq_dn6 / assign9700_e8499), (locals.var_a0_csisq_dn7 / assign9700_e8499), (locals.var_a0_csisq_dn8 / assign9700_e8499), (locals.var_a0_csisq_dn9 / assign9700_e8499), );
        }

    }

    pub(super) fn stamp_transient_block_10(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard531 != 0.0) {
            let assign9710_e8506: f64 = (1.0 / locals.var_k1_1d);
            (locals.var_inv_k1, locals.var_inv_k1_dn4, locals.var_inv_k1_dn6, locals.var_inv_k1_dn7, locals.var_inv_k1_dn8, locals.var_inv_k1_dn9, ) = (assign9710_e8506, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9720_e8512: f64 = (1.0 / locals.var_k2_1d);
            (locals.var_inv_k2, locals.var_inv_k2_dn4, locals.var_inv_k2_dn6, locals.var_inv_k2_dn7, locals.var_inv_k2_dn8, locals.var_inv_k2_dn9, ) = (assign9720_e8512, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9730_e8519: f64 = (1.0 + locals.var_inv_k1);
            let assign9730_e8521: f64 = (assign9730_e8519 + locals.var_inv_k2);
            let assign9730_e8522: f64 = (1.0 / assign9730_e8521);
            (locals.var_keq, locals.var_keq_dn4, locals.var_keq_dn6, locals.var_keq_dn7, locals.var_keq_dn8, locals.var_keq_dn9, ) = (assign9730_e8522, (-((locals.var_inv_k1_dn4 + locals.var_inv_k2_dn4) / (assign9730_e8521 * assign9730_e8521))), (-((locals.var_inv_k1_dn6 + locals.var_inv_k2_dn6) / (assign9730_e8521 * assign9730_e8521))), (-((locals.var_inv_k1_dn7 + locals.var_inv_k2_dn7) / (assign9730_e8521 * assign9730_e8521))), (-((locals.var_inv_k1_dn8 + locals.var_inv_k2_dn8) / (assign9730_e8521 * assign9730_e8521))), (-((locals.var_inv_k1_dn9 + locals.var_inv_k2_dn9) / (assign9730_e8521 * assign9730_e8521))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9740_e8529: f64 = (locals.var_xg10 - locals.var_xg20);
            let assign9740_e8530: f64 = (locals.var_keq * assign9740_e8529);
            (locals.var_dx_wi, locals.var_dx_wi_dn4, locals.var_dx_wi_dn6, locals.var_dx_wi_dn7, locals.var_dx_wi_dn8, locals.var_dx_wi_dn9, ) = (assign9740_e8530, ((locals.var_keq_dn4 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn4 - locals.var_xg20_dn4))), ((locals.var_keq_dn6 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn6 - locals.var_xg20_dn6))), ((locals.var_keq_dn7 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn7 - locals.var_xg20_dn7))), ((locals.var_keq_dn8 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn8 - locals.var_xg20_dn8))), ((locals.var_keq_dn9 * assign9740_e8529) + (locals.var_keq * (locals.var_xg10_dn9 - locals.var_xg20_dn9))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9750_e8537: f64 = (locals.var_dx_wi * locals.var_inv_k1);
            let assign9750_e8538: f64 = (locals.var_xg10 - assign9750_e8537);
            (locals.var_x1_wi0, locals.var_x1_wi0_dn4, locals.var_x1_wi0_dn6, locals.var_x1_wi0_dn7, locals.var_x1_wi0_dn8, locals.var_x1_wi0_dn9, ) = (assign9750_e8538, (locals.var_xg10_dn4 - ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4))), (locals.var_xg10_dn6 - ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6))), (locals.var_xg10_dn7 - ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7))), (locals.var_xg10_dn8 - ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8))), (locals.var_xg10_dn9 - ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9760_e8545: f64 = (locals.var_dx_wi * locals.var_inv_k2);
            let assign9760_e8546: f64 = (locals.var_xg20 + assign9760_e8545);
            (locals.var_x2_wi0, locals.var_x2_wi0_dn4, locals.var_x2_wi0_dn6, locals.var_x2_wi0_dn7, locals.var_x2_wi0_dn8, locals.var_x2_wi0_dn9, ) = (assign9760_e8546, (locals.var_xg20_dn4 + ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4))), (locals.var_xg20_dn6 + ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6))), (locals.var_xg20_dn7 + ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7))), (locals.var_xg20_dn8 + ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8))), (locals.var_xg20_dn9 + ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9770_e8553: f64 = (locals.var_k1_1d + 1.0);
            let assign9770_e8554: f64 = (1.0 / assign9770_e8553);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign9770_e8554, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9780_e8561: f64 = (locals.var_k2_1d + 1.0);
            let assign9780_e8562: f64 = (1.0 / assign9780_e8561);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign9780_e8562, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9790_e8569: f64 = (locals.var_k2_1d * locals.var_q_temp2);
            let assign9790_e8570: f64 = (locals.var_k1_1d + assign9790_e8569);
            let assign9790_e8572: f64 = (assign9790_e8570 * locals.var_diff_min);
            let assign9790_e8574: f64 = (assign9790_e8572 / locals.var_a0);
            let assign9790_e8575: f64 = (assign9790_e8574).ln();
            let assign9790_e8577: f64 = assign9790_e8575;
            let assign9790_e8579: f64 = (assign9790_e8577 + 1.5);
            (locals.var_q_x1sat, locals.var_q_x1sat_dn4, locals.var_q_x1sat_dn6, locals.var_q_x1sat_dn7, locals.var_q_x1sat_dn8, locals.var_q_x1sat_dn9, ) = (assign9790_e8579, (((((((locals.var_k2_1d * locals.var_q_temp2_dn4) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), (((((((locals.var_k2_1d * locals.var_q_temp2_dn6) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), (((((((locals.var_k2_1d * locals.var_q_temp2_dn7) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), (((((((locals.var_k2_1d * locals.var_q_temp2_dn8) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), (((((((locals.var_k2_1d * locals.var_q_temp2_dn9) * locals.var_diff_min) + (assign9790_e8570 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign9790_e8572 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign9790_e8574), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9800_e8586: f64 = (locals.var_k1_1d * locals.var_q_temp1);
            let assign9800_e8587: f64 = (locals.var_k2_1d + assign9800_e8586);
            let assign9800_e8589: f64 = (assign9800_e8587 * locals.var_diff_min);
            let assign9800_e8591: f64 = (assign9800_e8589 / locals.var_a0);
            let assign9800_e8592: f64 = (assign9800_e8591).ln();
            let assign9800_e8594: f64 = assign9800_e8592;
            let assign9800_e8596: f64 = (assign9800_e8594 + 1.5);
            (locals.var_q_x2sat, locals.var_q_x2sat_dn4, locals.var_q_x2sat_dn6, locals.var_q_x2sat_dn7, locals.var_q_x2sat_dn8, locals.var_q_x2sat_dn9, ) = (assign9800_e8596, (((((((locals.var_k1_1d * locals.var_q_temp1_dn4) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), (((((((locals.var_k1_1d * locals.var_q_temp1_dn6) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), (((((((locals.var_k1_1d * locals.var_q_temp1_dn7) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), (((((((locals.var_k1_1d * locals.var_q_temp1_dn8) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), (((((((locals.var_k1_1d * locals.var_q_temp1_dn9) * locals.var_diff_min) + (assign9800_e8587 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign9800_e8589 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign9800_e8591), );
        }

        let assign9810_e8601: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign9810_e8603: f64 = (assign9810_e8601 / 1.5);
        let assign9810_e8605: f64 = if assign9810_e8603 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard533 = assign9810_e8605;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard533 != 0.0)) {
            let assign9820_e8612: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
            let assign9820_e8614: f64 = (assign9820_e8612 / 1.5);
            let assign9820_e8615: f64 = (assign9820_e8614).exp();
            let assign9820_e8616: f64 = (1.0 + assign9820_e8615);
            let assign9820_e8617: f64 = (assign9820_e8616).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign9820_e8617, ((assign9820_e8615 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) / 1.5)) / assign9820_e8616), ((assign9820_e8615 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) / 1.5)) / assign9820_e8616), ((assign9820_e8615 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) / 1.5)) / assign9820_e8616), ((assign9820_e8615 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) / 1.5)) / assign9820_e8616), ((assign9820_e8615 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) / 1.5)) / assign9820_e8616), );
        }

        if ((locals.var_guard531 != 0.0) && (locals.var_guard533 == 0.0)) {
            let assign9830_e8626: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
            let assign9830_e8628: f64 = (assign9830_e8626 / 1.5);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign9830_e8628, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) / 1.5), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) / 1.5), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) / 1.5), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) / 1.5), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) / 1.5), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9840_e8635: f64 = (1.5 * locals.var_q_temp3);
            let assign9840_e8636: f64 = (locals.var_q_x1sat - assign9840_e8635);
            (locals.var_q_x1, locals.var_q_x1_dn4, locals.var_q_x1_dn6, locals.var_q_x1_dn7, locals.var_q_x1_dn8, locals.var_q_x1_dn9, ) = (assign9840_e8636, (locals.var_q_x1sat_dn4 - (1.5 * locals.var_q_temp3_dn4)), (locals.var_q_x1sat_dn6 - (1.5 * locals.var_q_temp3_dn6)), (locals.var_q_x1sat_dn7 - (1.5 * locals.var_q_temp3_dn7)), (locals.var_q_x1sat_dn8 - (1.5 * locals.var_q_temp3_dn8)), (locals.var_q_x1sat_dn9 - (1.5 * locals.var_q_temp3_dn9)), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9850_e8642: f64 = (locals.var_k2_1d * locals.var_xg20);
            let assign9850_e8644: f64 = (assign9850_e8642 + locals.var_q_x1);
            let assign9850_e8646: f64 = (assign9850_e8644 * locals.var_q_temp2);
            (locals.var_q_x2_wi, locals.var_q_x2_wi_dn4, locals.var_q_x2_wi_dn6, locals.var_q_x2_wi_dn7, locals.var_q_x2_wi_dn8, locals.var_q_x2_wi_dn9, ) = (assign9850_e8646, ((((locals.var_k2_1d * locals.var_xg20_dn4) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn4)), ((((locals.var_k2_1d * locals.var_xg20_dn6) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn6)), ((((locals.var_k2_1d * locals.var_xg20_dn7) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn7)), ((((locals.var_k2_1d * locals.var_xg20_dn8) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn8)), ((((locals.var_k2_1d * locals.var_xg20_dn9) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign9850_e8644 * locals.var_q_temp2_dn9)), );
        }

        let assign9860_e8651: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign9860_e8653: f64 = (assign9860_e8651 / 1.5);
        let assign9860_e8655: f64 = if assign9860_e8653 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard534 = assign9860_e8655;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard534 != 0.0)) {
            let assign9870_e8662: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
            let assign9870_e8664: f64 = (assign9870_e8662 / 1.5);
            let assign9870_e8665: f64 = (assign9870_e8664).exp();
            let assign9870_e8666: f64 = (1.0 + assign9870_e8665);
            let assign9870_e8667: f64 = (assign9870_e8666).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign9870_e8667, ((assign9870_e8665 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) / 1.5)) / assign9870_e8666), ((assign9870_e8665 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) / 1.5)) / assign9870_e8666), ((assign9870_e8665 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) / 1.5)) / assign9870_e8666), ((assign9870_e8665 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) / 1.5)) / assign9870_e8666), ((assign9870_e8665 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) / 1.5)) / assign9870_e8666), );
        }

        if ((locals.var_guard531 != 0.0) && (locals.var_guard534 == 0.0)) {
            let assign9880_e8676: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
            let assign9880_e8678: f64 = (assign9880_e8676 / 1.5);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign9880_e8678, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) / 1.5), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) / 1.5), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) / 1.5), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) / 1.5), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) / 1.5), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9890_e8685: f64 = (1.5 * locals.var_q_temp3);
            let assign9890_e8686: f64 = (locals.var_q_x2sat - assign9890_e8685);
            (locals.var_temp0, locals.var_temp0_dn4, locals.var_temp0_dn6, locals.var_temp0_dn7, locals.var_temp0_dn8, locals.var_temp0_dn9, ) = (assign9890_e8686, (locals.var_q_x2sat_dn4 - (1.5 * locals.var_q_temp3_dn4)), (locals.var_q_x2sat_dn6 - (1.5 * locals.var_q_temp3_dn6)), (locals.var_q_x2sat_dn7 - (1.5 * locals.var_q_temp3_dn7)), (locals.var_q_x2sat_dn8 - (1.5 * locals.var_q_temp3_dn8)), (locals.var_q_x2sat_dn9 - (1.5 * locals.var_q_temp3_dn9)), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9900_e8692: f64 = (locals.var_temp * locals.var_temp0);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign9900_e8692, ((locals.var_temp_dn4 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn4)), ((locals.var_temp_dn6 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn6)), ((locals.var_temp_dn7 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn7)), ((locals.var_temp_dn8 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn8)), ((locals.var_temp_dn9 * locals.var_temp0) + (locals.var_temp * locals.var_temp0_dn9)), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9910_e8698: f64 = (locals.var_temp * locals.var_xg20);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign9910_e8698, ((locals.var_temp_dn4 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn4)), ((locals.var_temp_dn6 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn6)), ((locals.var_temp_dn7 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn7)), ((locals.var_temp_dn8 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn8)), ((locals.var_temp_dn9 * locals.var_xg20) + (locals.var_temp * locals.var_xg20_dn9)), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign9920_e8704: f64 = (locals.var_temp1 - locals.var_temp2);
            (locals.var_spsub_xgb, locals.var_spsub_xgb_dn4, locals.var_spsub_xgb_dn6, locals.var_spsub_xgb_dn7, locals.var_spsub_xgb_dn8, locals.var_spsub_xgb_dn9, ) = (assign9920_e8704, (locals.var_temp1_dn4 - locals.var_temp2_dn4), (locals.var_temp1_dn6 - locals.var_temp2_dn6), (locals.var_temp1_dn7 - locals.var_temp2_dn7), (locals.var_temp1_dn8 - locals.var_temp2_dn8), (locals.var_temp1_dn9 - locals.var_temp2_dn9), );
        }

        let assign9930_e8708: f64 = (-locals.var_xn_sub);
        let assign9930_e8709: f64 = (assign9930_e8708).abs();
        let assign9930_e8711: f64 = if assign9930_e8709 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard535 = assign9930_e8711;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard535 != 0.0)) {
            let assign9940_e8716: f64 = (-locals.var_xn_sub);
            let assign9940_e8717: f64 = (assign9940_e8716).exp();
            (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9, ) = (assign9940_e8717, (assign9940_e8717 * (-locals.var_xn_sub_dn4)), (assign9940_e8717 * (-locals.var_xn_sub_dn6)), (assign9940_e8717 * (-locals.var_xn_sub_dn7)), (assign9940_e8717 * (-locals.var_xn_sub_dn8)), (assign9940_e8717 * (-locals.var_xn_sub_dn9)), );
        }

        let assign9950_e8721: f64 = (-locals.var_xn_sub);
        let assign9950_e8723: f64 = (-80.0);
        let assign9950_e8724: f64 = if assign9950_e8721 < assign9950_e8723 { 1.0 } else { 0.0 };
        locals.var_guard536 = assign9950_e8724;

        if (((locals.var_guard531 != 0.0) && (locals.var_guard535 == 0.0)) && (locals.var_guard536 != 0.0)) {
            let assign9960_e8734: f64 = (-locals.var_xn_sub);
            let assign9960_e8735: f64 = (-assign9960_e8734);
            let assign9960_e8737: f64 = (assign9960_e8735 - 80.0);
            let assign9960_e8741: f64 = (-locals.var_xn_sub);
            let assign9960_e8742: f64 = (-assign9960_e8741);
            let assign9960_e8744: f64 = (assign9960_e8742 - 80.0);
            let assign9960_e8745: f64 = (0.5 * assign9960_e8744);
            let assign9960_e8748: f64 = (-locals.var_xn_sub);
            let assign9960_e8749: f64 = (-assign9960_e8748);
            let assign9960_e8751: f64 = (assign9960_e8749 - 80.0);
            let assign9960_e8753: f64 = (assign9960_e8751 * 0.3333333333333);
            let assign9960_e8754: f64 = (1.0 + assign9960_e8753);
            let assign9960_e8755: f64 = (assign9960_e8745 * assign9960_e8754);
            let assign9960_e8756: f64 = (1.0 + assign9960_e8755);
            let assign9960_e8757: f64 = (assign9960_e8737 * assign9960_e8756);
            let assign9960_e8758: f64 = (1.0 + assign9960_e8757);
            let assign9960_e8759: f64 = (1.80485e-35 / assign9960_e8758);
            (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9, ) = (assign9960_e8759, (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn4)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn4))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn4)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn6)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn6))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn6)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn7)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn7))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn7)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn8)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn8))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn8)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), (-((1.80485e-35 * (((-(-locals.var_xn_sub_dn9)) * assign9960_e8756) + (assign9960_e8737 * (((0.5 * (-(-locals.var_xn_sub_dn9))) * assign9960_e8754) + (assign9960_e8745 * ((-(-locals.var_xn_sub_dn9)) * 0.3333333333333)))))) / (assign9960_e8758 * assign9960_e8758))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard535 == 0.0)) && (locals.var_guard536 == 0.0)) {
            let assign9970_e8772: f64 = (-locals.var_xn_sub);
            let assign9970_e8774: f64 = (assign9970_e8772 - 80.0);
            let assign9970_e8778: f64 = (-locals.var_xn_sub);
            let assign9970_e8780: f64 = (assign9970_e8778 - 80.0);
            let assign9970_e8781: f64 = (0.5 * assign9970_e8780);
            let assign9970_e8784: f64 = (-locals.var_xn_sub);
            let assign9970_e8786: f64 = (assign9970_e8784 - 80.0);
            let assign9970_e8788: f64 = (assign9970_e8786 * 0.3333333333333);
            let assign9970_e8789: f64 = (1.0 + assign9970_e8788);
            let assign9970_e8790: f64 = (assign9970_e8781 * assign9970_e8789);
            let assign9970_e8791: f64 = (1.0 + assign9970_e8790);
            let assign9970_e8792: f64 = (assign9970_e8774 * assign9970_e8791);
            let assign9970_e8793: f64 = (1.0 + assign9970_e8792);
            let assign9970_e8794: f64 = (5.54062e34 * assign9970_e8793);
            (locals.var_spsub_delta, locals.var_spsub_delta_dn4, locals.var_spsub_delta_dn6, locals.var_spsub_delta_dn7, locals.var_spsub_delta_dn8, locals.var_spsub_delta_dn9, ) = (assign9970_e8794, (5.54062e34 * (((-locals.var_xn_sub_dn4) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn4)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn4) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn6) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn6)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn6) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn7) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn7)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn7) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn8) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn8)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn8) * 0.3333333333333)))))), (5.54062e34 * (((-locals.var_xn_sub_dn9) * assign9970_e8791) + (assign9970_e8774 * (((0.5 * (-locals.var_xn_sub_dn9)) * assign9970_e8789) + (assign9970_e8781 * ((-locals.var_xn_sub_dn9) * 0.3333333333333)))))), );
        }

        let assign9980_e8798: f64 = (locals.var_spsub_xgb).abs();
        let assign9980_e8800: f64 = if assign9980_e8798 <= locals.var_margin_sub { 1.0 } else { 0.0 };
        locals.var_guard537 = assign9980_e8800;

        if ((locals.var_guard531 != 0.0) && (locals.var_guard537 != 0.0)) {
            let assign9990_e8806: f64 = (locals.var_inv_xisub * locals.var_inv_xisub);
            let assign9990_e8808: f64 = (assign9990_e8806 * 0.1666666666667);
            let assign9990_e8810: f64 = (assign9990_e8808 / 1.4142135623731);
            (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9, ) = (assign9990_e8810, ((((locals.var_inv_xisub_dn4 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn4)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn6 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn6)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn7 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn7)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn8 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn8)) * 0.1666666666667) / 1.4142135623731), ((((locals.var_inv_xisub_dn9 * locals.var_inv_xisub) + (locals.var_inv_xisub * locals.var_inv_xisub_dn9)) * 0.1666666666667) / 1.4142135623731), );
        }

        if ((locals.var_guard531 != 0.0) && (locals.var_guard537 != 0.0)) {
            let assign10000_e8818: f64 = (locals.var_spsub_xgb * locals.var_inv_xisub);
            let assign10000_e8823: f64 = (1.0 - locals.var_spsub_delta);
            let assign10000_e8824: f64 = (locals.var_spsub_xgb * assign10000_e8823);
            let assign10000_e8826: f64 = (assign10000_e8824 * locals.var_gfsub);
            let assign10000_e8828: f64 = (assign10000_e8826 * locals.var_spsub_temp1);
            let assign10000_e8829: f64 = (1.0 + assign10000_e8828);
            let assign10000_e8830: f64 = (assign10000_e8818 * assign10000_e8829);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign10000_e8830, ((((locals.var_spsub_xgb_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn4)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn4 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn4))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn4)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn4)))), ((((locals.var_spsub_xgb_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn6)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn6 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn6))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn6)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn6)))), ((((locals.var_spsub_xgb_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn7)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn7 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn7))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn7)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn7)))), ((((locals.var_spsub_xgb_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn8)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn8 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn8))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn8)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn8)))), ((((locals.var_spsub_xgb_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn9)) * assign10000_e8829) + (assign10000_e8818 * ((((((locals.var_spsub_xgb_dn9 * assign10000_e8823) + (locals.var_spsub_xgb * (-locals.var_spsub_delta_dn9))) * locals.var_gfsub) + (assign10000_e8824 * locals.var_gfsub_dn9)) * locals.var_spsub_temp1) + (assign10000_e8826 * locals.var_spsub_temp1_dn9)))), );
        }

        let assign10010_e8835: f64 = (-locals.var_margin_sub);
        let assign10010_e8836: f64 = if locals.var_spsub_xgb < assign10010_e8835 { 1.0 } else { 0.0 };
        locals.var_guard538 = assign10010_e8836;

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10020_e8844: f64 = (-locals.var_spsub_xgb);
            (locals.var_spsub_yg, locals.var_spsub_yg_dn4, locals.var_spsub_yg_dn6, locals.var_spsub_yg_dn7, locals.var_spsub_yg_dn8, locals.var_spsub_yg_dn9, ) = (assign10020_e8844, (-locals.var_spsub_xgb_dn4), (-locals.var_spsub_xgb_dn6), (-locals.var_spsub_xgb_dn7), (-locals.var_spsub_xgb_dn8), (-locals.var_spsub_xgb_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10030_e8856: f64 = (locals.var_spsub_yg * locals.var_inv_xisub);
            let assign10030_e8857: f64 = (1.25 * assign10030_e8856);
            (locals.var_spsub_ysub, locals.var_spsub_ysub_dn4, locals.var_spsub_ysub_dn6, locals.var_spsub_ysub_dn7, locals.var_spsub_ysub_dn8, locals.var_spsub_ysub_dn9, ) = (assign10030_e8857, (1.25 * ((locals.var_spsub_yg_dn4 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn4))), (1.25 * ((locals.var_spsub_yg_dn6 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn6))), (1.25 * ((locals.var_spsub_yg_dn7 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn7))), (1.25 * ((locals.var_spsub_yg_dn8 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn8))), (1.25 * ((locals.var_spsub_yg_dn9 * locals.var_inv_xisub) + (locals.var_spsub_yg * locals.var_inv_xisub_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10040_e8869: f64 = (locals.var_spsub_ysub + 10.0);
            let assign10040_e8872: f64 = (locals.var_spsub_ysub - 6.0);
            let assign10040_e8875: f64 = (locals.var_spsub_ysub - 6.0);
            let assign10040_e8876: f64 = (assign10040_e8872 * assign10040_e8875);
            let assign10040_e8878: f64 = (assign10040_e8876 + 64.0);
            let assign10040_e8879: f64 = (assign10040_e8878).sqrt();
            let assign10040_e8880: f64 = (assign10040_e8869 - assign10040_e8879);
            let assign10040_e8881: f64 = (0.5 * assign10040_e8880);
            (locals.var_spsub_eta, locals.var_spsub_eta_dn4, locals.var_spsub_eta_dn6, locals.var_spsub_eta_dn7, locals.var_spsub_eta_dn8, locals.var_spsub_eta_dn9, ) = (assign10040_e8881, (0.5 * (locals.var_spsub_ysub_dn4 - (((locals.var_spsub_ysub_dn4 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn4)) / (2.0 * assign10040_e8879)))), (0.5 * (locals.var_spsub_ysub_dn6 - (((locals.var_spsub_ysub_dn6 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn6)) / (2.0 * assign10040_e8879)))), (0.5 * (locals.var_spsub_ysub_dn7 - (((locals.var_spsub_ysub_dn7 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn7)) / (2.0 * assign10040_e8879)))), (0.5 * (locals.var_spsub_ysub_dn8 - (((locals.var_spsub_ysub_dn8 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn8)) / (2.0 * assign10040_e8879)))), (0.5 * (locals.var_spsub_ysub_dn9 - (((locals.var_spsub_ysub_dn9 * assign10040_e8875) + (assign10040_e8872 * locals.var_spsub_ysub_dn9)) / (2.0 * assign10040_e8879)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10050_e8892: f64 = (locals.var_spsub_yg - locals.var_spsub_eta);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10050_e8892, (locals.var_spsub_yg_dn4 - locals.var_spsub_eta_dn4), (locals.var_spsub_yg_dn6 - locals.var_spsub_eta_dn6), (locals.var_spsub_yg_dn7 - locals.var_spsub_eta_dn7), (locals.var_spsub_yg_dn8 - locals.var_spsub_eta_dn8), (locals.var_spsub_yg_dn9 - locals.var_spsub_eta_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10060_e8903: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
            let assign10060_e8907: f64 = (locals.var_spsub_eta + 1.0);
            let assign10060_e8908: f64 = (locals.var_gfsub2 * assign10060_e8907);
            let assign10060_e8909: f64 = (assign10060_e8903 + assign10060_e8908);
            (locals.var_spsub_a, locals.var_spsub_a_dn4, locals.var_spsub_a_dn6, locals.var_spsub_a_dn7, locals.var_spsub_a_dn8, locals.var_spsub_a_dn9, ) = (assign10060_e8909, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) + ((locals.var_gfsub2_dn4 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn4))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) + ((locals.var_gfsub2_dn6 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn6))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) + ((locals.var_gfsub2_dn7 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn7))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) + ((locals.var_gfsub2_dn8 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn8))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) + ((locals.var_gfsub2_dn9 * assign10060_e8907) + (locals.var_gfsub2 * locals.var_spsub_eta_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10070_e8920: f64 = (2.0 * locals.var_spsub_temp);
            let assign10070_e8922: f64 = (assign10070_e8920 - locals.var_gfsub2);
            (locals.var_spsub_c, locals.var_spsub_c_dn4, locals.var_spsub_c_dn6, locals.var_spsub_c_dn7, locals.var_spsub_c_dn8, locals.var_spsub_c_dn9, ) = (assign10070_e8922, ((2.0 * locals.var_spsub_temp_dn4) - locals.var_gfsub2_dn4), ((2.0 * locals.var_spsub_temp_dn6) - locals.var_gfsub2_dn6), ((2.0 * locals.var_spsub_temp_dn7) - locals.var_gfsub2_dn7), ((2.0 * locals.var_spsub_temp_dn8) - locals.var_gfsub2_dn8), ((2.0 * locals.var_spsub_temp_dn9) - locals.var_gfsub2_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10080_e8932: f64 = (-locals.var_spsub_eta);
            let assign10080_e8935: f64 = (locals.var_spsub_a * locals.var_inv_gfsub2);
            let assign10080_e8936: f64 = (assign10080_e8935).ln();
            let assign10080_e8937: f64 = (assign10080_e8932 + assign10080_e8936);
            (locals.var_spsub_tau, locals.var_spsub_tau_dn4, locals.var_spsub_tau_dn6, locals.var_spsub_tau_dn7, locals.var_spsub_tau_dn8, locals.var_spsub_tau_dn9, ) = (assign10080_e8937, ((-locals.var_spsub_eta_dn4) + (((locals.var_spsub_a_dn4 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn4)) / assign10080_e8935)), ((-locals.var_spsub_eta_dn6) + (((locals.var_spsub_a_dn6 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn6)) / assign10080_e8935)), ((-locals.var_spsub_eta_dn7) + (((locals.var_spsub_a_dn7 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn7)) / assign10080_e8935)), ((-locals.var_spsub_eta_dn8) + (((locals.var_spsub_a_dn8 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn8)) / assign10080_e8935)), ((-locals.var_spsub_eta_dn9) + (((locals.var_spsub_a_dn9 * locals.var_inv_gfsub2) + (locals.var_spsub_a * locals.var_inv_gfsub2_dn9)) / assign10080_e8935)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10090_e8948: f64 = (locals.var_spsub_a + locals.var_spsub_c);
            (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, ) = (assign10090_e8948, (locals.var_spsub_a_dn4 + locals.var_spsub_c_dn4), (locals.var_spsub_a_dn6 + locals.var_spsub_c_dn6), (locals.var_spsub_a_dn7 + locals.var_spsub_c_dn7), (locals.var_spsub_a_dn8 + locals.var_spsub_c_dn8), (locals.var_spsub_a_dn9 + locals.var_spsub_c_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10100_e8959: f64 = (locals.var_nu * locals.var_nu);
            let assign10100_e8963: f64 = (0.5 * locals.var_spsub_c);
            let assign10100_e8965: f64 = (assign10100_e8963 * locals.var_spsub_c);
            let assign10100_e8967: f64 = (assign10100_e8965 - locals.var_spsub_a);
            let assign10100_e8968: f64 = (locals.var_spsub_tau * assign10100_e8967);
            let assign10100_e8969: f64 = (assign10100_e8959 + assign10100_e8968);
            (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, ) = (assign10100_e8969, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_spsub_tau_dn4 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn4) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn4)) - locals.var_spsub_a_dn4)))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_spsub_tau_dn6 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn6) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn6)) - locals.var_spsub_a_dn6)))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_spsub_tau_dn7 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn7) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn7)) - locals.var_spsub_a_dn7)))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_spsub_tau_dn8 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn8) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn8)) - locals.var_spsub_a_dn8)))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_spsub_tau_dn9 * assign10100_e8967) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn9) * locals.var_spsub_c) + (assign10100_e8963 * locals.var_spsub_c_dn9)) - locals.var_spsub_a_dn9)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10110_e8981: f64 = (locals.var_spsub_a * locals.var_nu);
            let assign10110_e8983: f64 = (assign10110_e8981 * locals.var_spsub_tau);
            let assign10110_e8987: f64 = (locals.var_nu / locals.var_mutau);
            let assign10110_e8989: f64 = (assign10110_e8987 * locals.var_spsub_tau);
            let assign10110_e8991: f64 = (assign10110_e8989 * locals.var_spsub_tau);
            let assign10110_e8993: f64 = (assign10110_e8991 * locals.var_spsub_c);
            let assign10110_e8996: f64 = (locals.var_spsub_c * locals.var_spsub_c);
            let assign10110_e8998: f64 = (assign10110_e8996 * 0.3333333333333);
            let assign10110_e9000: f64 = (assign10110_e8998 - locals.var_spsub_a);
            let assign10110_e9001: f64 = (assign10110_e8993 * assign10110_e9000);
            let assign10110_e9002: f64 = (locals.var_mutau + assign10110_e9001);
            let assign10110_e9003: f64 = (assign10110_e8983 / assign10110_e9002);
            let assign10110_e9004: f64 = (locals.var_spsub_eta + assign10110_e9003);
            (locals.var_spsub_y0, locals.var_spsub_y0_dn4, locals.var_spsub_y0_dn6, locals.var_spsub_y0_dn7, locals.var_spsub_y0_dn8, locals.var_spsub_y0_dn9, ) = (assign10110_e9004, (locals.var_spsub_eta_dn4 + (((((((locals.var_spsub_a_dn4 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn4)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn4)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn4)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn4)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn4)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn4 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn4)) * 0.3333333333333) - locals.var_spsub_a_dn4)))))) / (assign10110_e9002 * assign10110_e9002))), (locals.var_spsub_eta_dn6 + (((((((locals.var_spsub_a_dn6 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn6)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn6)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn6)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn6)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn6)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn6 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn6)) * 0.3333333333333) - locals.var_spsub_a_dn6)))))) / (assign10110_e9002 * assign10110_e9002))), (locals.var_spsub_eta_dn7 + (((((((locals.var_spsub_a_dn7 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn7)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn7)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn7)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn7)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn7)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn7 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn7)) * 0.3333333333333) - locals.var_spsub_a_dn7)))))) / (assign10110_e9002 * assign10110_e9002))), (locals.var_spsub_eta_dn8 + (((((((locals.var_spsub_a_dn8 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn8)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn8)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn8)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn8)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn8)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn8 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn8)) * 0.3333333333333) - locals.var_spsub_a_dn8)))))) / (assign10110_e9002 * assign10110_e9002))), (locals.var_spsub_eta_dn9 + (((((((locals.var_spsub_a_dn9 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn9)) * locals.var_spsub_tau) + (assign10110_e8981 * locals.var_spsub_tau_dn9)) * assign10110_e9002) - (assign10110_e8983 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10110_e8987 * locals.var_spsub_tau_dn9)) * locals.var_spsub_tau) + (assign10110_e8989 * locals.var_spsub_tau_dn9)) * locals.var_spsub_c) + (assign10110_e8991 * locals.var_spsub_c_dn9)) * assign10110_e9000) + (assign10110_e8993 * ((((locals.var_spsub_c_dn9 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn9)) * 0.3333333333333) - locals.var_spsub_a_dn9)))))) / (assign10110_e9002 * assign10110_e9002))), );
        }

        let assign10120_e9009: f64 = if locals.var_spsub_y0 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard539 = assign10120_e9009;

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 != 0.0)) {
            let assign10130_e9019: f64 = (locals.var_spsub_y0).exp();
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10130_e9019, (assign10130_e9019 * locals.var_spsub_y0_dn4), (assign10130_e9019 * locals.var_spsub_y0_dn6), (assign10130_e9019 * locals.var_spsub_y0_dn7), (assign10130_e9019 * locals.var_spsub_y0_dn8), (assign10130_e9019 * locals.var_spsub_y0_dn9), );
        }

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) && (locals.var_guard539 == 0.0)) {
            let assign10140_e9035: f64 = (locals.var_spsub_y0 - 80.0);
            let assign10140_e9040: f64 = (locals.var_spsub_y0 - 80.0);
            let assign10140_e9041: f64 = (0.5 * assign10140_e9040);
            let assign10140_e9045: f64 = (locals.var_spsub_y0 - 80.0);
            let assign10140_e9047: f64 = (assign10140_e9045 * 0.3333333333333);
            let assign10140_e9048: f64 = (1.0 + assign10140_e9047);
            let assign10140_e9049: f64 = (assign10140_e9041 * assign10140_e9048);
            let assign10140_e9050: f64 = (1.0 + assign10140_e9049);
            let assign10140_e9051: f64 = (assign10140_e9035 * assign10140_e9050);
            let assign10140_e9052: f64 = (1.0 + assign10140_e9051);
            let assign10140_e9053: f64 = (5.54062e34 * assign10140_e9052);
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10140_e9053, (5.54062e34 * ((locals.var_spsub_y0_dn4 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn4) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn4 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn6 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn6) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn6 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn7 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn7) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn7 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn8 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn8) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn8 * 0.3333333333333)))))), (5.54062e34 * ((locals.var_spsub_y0_dn9 * assign10140_e9050) + (assign10140_e9035 * (((0.5 * locals.var_spsub_y0_dn9) * assign10140_e9048) + (assign10140_e9041 * (locals.var_spsub_y0_dn9 * 0.3333333333333)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10150_e9064: f64 = (1.0 / locals.var_spsub_delta0);
            (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9, ) = (assign10150_e9064, (-(locals.var_spsub_delta0_dn4 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn6 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn7 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn8 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn9 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10160_e9077: f64 = (locals.var_spsub_y0 * locals.var_spsub_y0);
            let assign10160_e9078: f64 = (2.0 + assign10160_e9077);
            let assign10160_e9079: f64 = (1.0 / assign10160_e9078);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10160_e9079, (-(((locals.var_spsub_y0_dn4 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn4)) / (assign10160_e9078 * assign10160_e9078))), (-(((locals.var_spsub_y0_dn6 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn6)) / (assign10160_e9078 * assign10160_e9078))), (-(((locals.var_spsub_y0_dn7 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn7)) / (assign10160_e9078 * assign10160_e9078))), (-(((locals.var_spsub_y0_dn8 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn8)) / (assign10160_e9078 * assign10160_e9078))), (-(((locals.var_spsub_y0_dn9 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn9)) / (assign10160_e9078 * assign10160_e9078))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10170_e9090: f64 = (locals.var_spsub_y0 * locals.var_spsub_y0);
            let assign10170_e9092: f64 = (assign10170_e9090 * locals.var_spsub_temp);
            (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9, ) = (assign10170_e9092, ((((locals.var_spsub_y0_dn4 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn4)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn4)), ((((locals.var_spsub_y0_dn6 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn6)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn6)), ((((locals.var_spsub_y0_dn7 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn7)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn7)), ((((locals.var_spsub_y0_dn8 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn8)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn8)), ((((locals.var_spsub_y0_dn9 * locals.var_spsub_y0) + (locals.var_spsub_y0 * locals.var_spsub_y0_dn9)) * locals.var_spsub_temp) + (assign10170_e9090 * locals.var_spsub_temp_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10180_e9104: f64 = (locals.var_spsub_y0 * locals.var_spsub_temp);
            let assign10180_e9106: f64 = (assign10180_e9104 * locals.var_spsub_temp);
            let assign10180_e9107: f64 = (4.0 * assign10180_e9106);
            (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9, ) = (assign10180_e9107, (4.0 * ((((locals.var_spsub_y0_dn4 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn4))), (4.0 * ((((locals.var_spsub_y0_dn6 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn6))), (4.0 * ((((locals.var_spsub_y0_dn7 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn7))), (4.0 * ((((locals.var_spsub_y0_dn8 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn8))), (4.0 * ((((locals.var_spsub_y0_dn9 * locals.var_spsub_temp) + (locals.var_spsub_y0 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10180_e9104 * locals.var_spsub_temp_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10190_e9118: f64 = (8.0 * locals.var_spsub_temp);
            let assign10190_e9121: f64 = (12.0 * locals.var_spsub_xi0);
            let assign10190_e9122: f64 = (assign10190_e9118 - assign10190_e9121);
            let assign10190_e9124: f64 = (assign10190_e9122 * locals.var_spsub_temp);
            let assign10190_e9126: f64 = (assign10190_e9124 * locals.var_spsub_temp);
            (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9, ) = (assign10190_e9126, ((((((8.0 * locals.var_spsub_temp_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn4)), ((((((8.0 * locals.var_spsub_temp_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn6)), ((((((8.0 * locals.var_spsub_temp_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn7)), ((((((8.0 * locals.var_spsub_temp_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn8)), ((((((8.0 * locals.var_spsub_temp_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp) + (assign10190_e9122 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10190_e9124 * locals.var_spsub_temp_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10200_e9137: f64 = (locals.var_spsub_yg - locals.var_spsub_y0);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10200_e9137, (locals.var_spsub_yg_dn4 - locals.var_spsub_y0_dn4), (locals.var_spsub_yg_dn6 - locals.var_spsub_y0_dn6), (locals.var_spsub_yg_dn7 - locals.var_spsub_y0_dn7), (locals.var_spsub_yg_dn8 - locals.var_spsub_y0_dn8), (locals.var_spsub_yg_dn9 - locals.var_spsub_y0_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10210_e9148: f64 = (locals.var_spsub_delta * locals.var_spsub_delta1);
            (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9, ) = (assign10210_e9148, ((locals.var_spsub_delta_dn4 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn4)), ((locals.var_spsub_delta_dn6 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn6)), ((locals.var_spsub_delta_dn7 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn7)), ((locals.var_spsub_delta_dn8 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn8)), ((locals.var_spsub_delta_dn9 * locals.var_spsub_delta1) + (locals.var_spsub_delta * locals.var_spsub_delta1_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10220_e9159: f64 = (2.0 * locals.var_spsub_temp);
            let assign10220_e9163: f64 = (locals.var_spsub_delta0 - 1.0);
            let assign10220_e9165: f64 = (assign10220_e9163 - locals.var_spsub_temp1);
            let assign10220_e9169: f64 = (1.0 - locals.var_spsub_xi1);
            let assign10220_e9170: f64 = (locals.var_spsub_delta * assign10220_e9169);
            let assign10220_e9171: f64 = (assign10220_e9165 + assign10220_e9170);
            let assign10220_e9172: f64 = (locals.var_gfsub2 * assign10220_e9171);
            let assign10220_e9173: f64 = (assign10220_e9159 + assign10220_e9172);
            (locals.var_spsub_pc, locals.var_spsub_pc_dn4, locals.var_spsub_pc_dn6, locals.var_spsub_pc_dn7, locals.var_spsub_pc_dn8, locals.var_spsub_pc_dn9, ) = (assign10220_e9173, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn4 - locals.var_spsub_temp1_dn4) + ((locals.var_spsub_delta_dn4 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn4))))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn6 - locals.var_spsub_temp1_dn6) + ((locals.var_spsub_delta_dn6 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn6))))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn7 - locals.var_spsub_temp1_dn7) + ((locals.var_spsub_delta_dn7 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn7))))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn8 - locals.var_spsub_temp1_dn8) + ((locals.var_spsub_delta_dn8 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn8))))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10220_e9171) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn9 - locals.var_spsub_temp1_dn9) + ((locals.var_spsub_delta_dn9 * assign10220_e9169) + (locals.var_spsub_delta * (-locals.var_spsub_xi1_dn9))))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10230_e9184: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
            let assign10230_e9188: f64 = (locals.var_spsub_delta0 - locals.var_spsub_y0);
            let assign10230_e9190: f64 = (assign10230_e9188 - 1.0);
            let assign10230_e9192: f64 = (assign10230_e9190 + locals.var_spsub_temp1);
            let assign10230_e9196: f64 = (locals.var_spsub_y0 - 1.0);
            let assign10230_e9198: f64 = (assign10230_e9196 - locals.var_spsub_xi0);
            let assign10230_e9199: f64 = (locals.var_spsub_delta * assign10230_e9198);
            let assign10230_e9200: f64 = (assign10230_e9192 + assign10230_e9199);
            let assign10230_e9201: f64 = (locals.var_gfsub2 * assign10230_e9200);
            let assign10230_e9202: f64 = (assign10230_e9184 - assign10230_e9201);
            (locals.var_spsub_qc, locals.var_spsub_qc_dn4, locals.var_spsub_qc_dn6, locals.var_spsub_qc_dn7, locals.var_spsub_qc_dn8, locals.var_spsub_qc_dn9, ) = (assign10230_e9202, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn4 - locals.var_spsub_y0_dn4) + locals.var_spsub_temp1_dn4) + ((locals.var_spsub_delta_dn4 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn4 - locals.var_spsub_xi0_dn4))))))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn6 - locals.var_spsub_y0_dn6) + locals.var_spsub_temp1_dn6) + ((locals.var_spsub_delta_dn6 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn6 - locals.var_spsub_xi0_dn6))))))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn7 - locals.var_spsub_y0_dn7) + locals.var_spsub_temp1_dn7) + ((locals.var_spsub_delta_dn7 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn7 - locals.var_spsub_xi0_dn7))))))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn8 - locals.var_spsub_y0_dn8) + locals.var_spsub_temp1_dn8) + ((locals.var_spsub_delta_dn8 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn8 - locals.var_spsub_xi0_dn8))))))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10230_e9200) + (locals.var_gfsub2 * (((locals.var_spsub_delta0_dn9 - locals.var_spsub_y0_dn9) + locals.var_spsub_temp1_dn9) + ((locals.var_spsub_delta_dn9 * assign10230_e9198) + (locals.var_spsub_delta * (locals.var_spsub_y0_dn9 - locals.var_spsub_xi0_dn9))))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10240_e9215: f64 = (locals.var_spsub_delta0 + locals.var_spsub_temp1);
            let assign10240_e9218: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
            let assign10240_e9219: f64 = (assign10240_e9215 - assign10240_e9218);
            let assign10240_e9220: f64 = (locals.var_gfsub2 * assign10240_e9219);
            let assign10240_e9221: f64 = (2.0 - assign10240_e9220);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10240_e9221, (-((locals.var_gfsub2_dn4 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn4 + locals.var_spsub_temp1_dn4) - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4)))))), (-((locals.var_gfsub2_dn6 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn6 + locals.var_spsub_temp1_dn6) - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6)))))), (-((locals.var_gfsub2_dn7 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn7 + locals.var_spsub_temp1_dn7) - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7)))))), (-((locals.var_gfsub2_dn8 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn8 + locals.var_spsub_temp1_dn8) - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8)))))), (-((locals.var_gfsub2_dn9 * assign10240_e9219) + (locals.var_gfsub2 * ((locals.var_spsub_delta0_dn9 + locals.var_spsub_temp1_dn9) - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10250_e9232: f64 = (locals.var_spsub_pc * locals.var_spsub_pc);
            let assign10250_e9236: f64 = (locals.var_spsub_qc * locals.var_spsub_temp);
            let assign10250_e9237: f64 = (2.0 * assign10250_e9236);
            let assign10250_e9238: f64 = (assign10250_e9232 - assign10250_e9237);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10250_e9238, (((locals.var_spsub_pc_dn4 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn4)) - (2.0 * ((locals.var_spsub_qc_dn4 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn4)))), (((locals.var_spsub_pc_dn6 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn6)) - (2.0 * ((locals.var_spsub_qc_dn6 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn6)))), (((locals.var_spsub_pc_dn7 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn7)) - (2.0 * ((locals.var_spsub_qc_dn7 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn7)))), (((locals.var_spsub_pc_dn8 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn8)) - (2.0 * ((locals.var_spsub_qc_dn8 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn8)))), (((locals.var_spsub_pc_dn9 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn9)) - (2.0 * ((locals.var_spsub_qc_dn9 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn9)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 != 0.0)) {
            let assign10260_e9248: f64 = (-locals.var_spsub_y0);
            let assign10260_e9253: f64 = (locals.var_spsub_temp).sqrt();
            let assign10260_e9254: f64 = (locals.var_spsub_pc + assign10260_e9253);
            let assign10260_e9255: f64 = (locals.var_spsub_qc / assign10260_e9254);
            let assign10260_e9256: f64 = (2.0 * assign10260_e9255);
            let assign10260_e9257: f64 = (assign10260_e9248 - assign10260_e9256);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign10260_e9257, ((-locals.var_spsub_y0_dn4) - (2.0 * (((locals.var_spsub_qc_dn4 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn4 + (locals.var_spsub_temp_dn4 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), ((-locals.var_spsub_y0_dn6) - (2.0 * (((locals.var_spsub_qc_dn6 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn6 + (locals.var_spsub_temp_dn6 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), ((-locals.var_spsub_y0_dn7) - (2.0 * (((locals.var_spsub_qc_dn7 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn7 + (locals.var_spsub_temp_dn7 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), ((-locals.var_spsub_y0_dn8) - (2.0 * (((locals.var_spsub_qc_dn8 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn8 + (locals.var_spsub_temp_dn8 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), ((-locals.var_spsub_y0_dn9) - (2.0 * (((locals.var_spsub_qc_dn9 * assign10260_e9254) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn9 + (locals.var_spsub_temp_dn9 / (2.0 * assign10260_e9253))))) / (assign10260_e9254 * assign10260_e9254)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10270_e9271: f64 = (locals.var_gfsub * 0.732464877560822);
            let assign10270_e9272: f64 = (1.25 + assign10270_e9271);
            let assign10270_e9273: f64 = (1.0 / assign10270_e9272);
            (locals.var_spsub_xg1, locals.var_spsub_xg1_dn4, locals.var_spsub_xg1_dn6, locals.var_spsub_xg1_dn7, locals.var_spsub_xg1_dn8, locals.var_spsub_xg1_dn9, ) = (assign10270_e9273, (-((locals.var_gfsub_dn4 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), (-((locals.var_gfsub_dn6 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), (-((locals.var_gfsub_dn7 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), (-((locals.var_gfsub_dn8 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), (-((locals.var_gfsub_dn9 * 0.732464877560822) / (assign10270_e9272 * assign10270_e9272))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10280_e9285: f64 = (1.25 * locals.var_xisub);
            let assign10280_e9287: f64 = (assign10280_e9285 * locals.var_spsub_xg1);
            let assign10280_e9289: f64 = (assign10280_e9287 - 1.0);
            let assign10280_e9291: f64 = (assign10280_e9289 * locals.var_spsub_xg1);
            (locals.var_spsub_a_fac, locals.var_spsub_a_fac_dn4, locals.var_spsub_a_fac_dn6, locals.var_spsub_a_fac_dn7, locals.var_spsub_a_fac_dn8, locals.var_spsub_a_fac_dn9, ) = (assign10280_e9291, (((((1.25 * locals.var_xisub_dn4) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn4)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn4)), (((((1.25 * locals.var_xisub_dn6) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn6)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn6)), (((((1.25 * locals.var_xisub_dn7) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn7)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn7)), (((((1.25 * locals.var_xisub_dn8) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn8)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn8)), (((((1.25 * locals.var_xisub_dn9) * locals.var_spsub_xg1) + (assign10280_e9285 * locals.var_spsub_xg1_dn9)) * locals.var_spsub_xg1) + (assign10280_e9289 * locals.var_spsub_xg1_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10290_e9303: f64 = (locals.var_spsub_xgb * locals.var_inv_xisub);
            let assign10290_e9307: f64 = (locals.var_spsub_a_fac * locals.var_spsub_xgb);
            let assign10290_e9308: f64 = (1.0 + assign10290_e9307);
            let assign10290_e9309: f64 = (assign10290_e9303 * assign10290_e9308);
            (locals.var_spsub_xbar, locals.var_spsub_xbar_dn4, locals.var_spsub_xbar_dn6, locals.var_spsub_xbar_dn7, locals.var_spsub_xbar_dn8, locals.var_spsub_xbar_dn9, ) = (assign10290_e9309, ((((locals.var_spsub_xgb_dn4 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn4)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn4 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn4)))), ((((locals.var_spsub_xgb_dn6 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn6)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn6 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn6)))), ((((locals.var_spsub_xgb_dn7 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn7)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn7 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn7)))), ((((locals.var_spsub_xgb_dn8 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn8)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn8 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn8)))), ((((locals.var_spsub_xgb_dn9 * locals.var_inv_xisub) + (locals.var_spsub_xgb * locals.var_inv_xisub_dn9)) * assign10290_e9308) + (assign10290_e9303 * ((locals.var_spsub_a_fac_dn9 * locals.var_spsub_xgb) + (locals.var_spsub_a_fac * locals.var_spsub_xgb_dn9)))), );
        }

        let assign10300_e9313: f64 = (-locals.var_spsub_xbar);
        let assign10300_e9315: f64 = (-80.0);
        let assign10300_e9316: f64 = if assign10300_e9313 > assign10300_e9315 { 1.0 } else { 0.0 };
        locals.var_guard540 = assign10300_e9316;

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard540 != 0.0)) {
            let assign10310_e9327: f64 = (-locals.var_spsub_xbar);
            let assign10310_e9328: f64 = (assign10310_e9327).exp();
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10310_e9328, (assign10310_e9328 * (-locals.var_spsub_xbar_dn4)), (assign10310_e9328 * (-locals.var_spsub_xbar_dn6)), (assign10310_e9328 * (-locals.var_spsub_xbar_dn7)), (assign10310_e9328 * (-locals.var_spsub_xbar_dn8)), (assign10310_e9328 * (-locals.var_spsub_xbar_dn9)), );
        }

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard540 == 0.0)) {
            let assign10320_e9344: f64 = (-locals.var_spsub_xbar);
            let assign10320_e9345: f64 = (-assign10320_e9344);
            let assign10320_e9347: f64 = (assign10320_e9345 - 80.0);
            let assign10320_e9351: f64 = (-locals.var_spsub_xbar);
            let assign10320_e9352: f64 = (-assign10320_e9351);
            let assign10320_e9354: f64 = (assign10320_e9352 - 80.0);
            let assign10320_e9355: f64 = (0.5 * assign10320_e9354);
            let assign10320_e9358: f64 = (-locals.var_spsub_xbar);
            let assign10320_e9359: f64 = (-assign10320_e9358);
            let assign10320_e9361: f64 = (assign10320_e9359 - 80.0);
            let assign10320_e9363: f64 = (assign10320_e9361 * 0.3333333333333);
            let assign10320_e9364: f64 = (1.0 + assign10320_e9363);
            let assign10320_e9365: f64 = (assign10320_e9355 * assign10320_e9364);
            let assign10320_e9366: f64 = (1.0 + assign10320_e9365);
            let assign10320_e9367: f64 = (assign10320_e9347 * assign10320_e9366);
            let assign10320_e9368: f64 = (1.0 + assign10320_e9367);
            let assign10320_e9369: f64 = (1.80485e-35 / assign10320_e9368);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10320_e9369, (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn4)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn4))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn4)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn6)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn6))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn6)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn7)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn7))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn7)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn8)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn8))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn8)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), (-((1.80485e-35 * (((-(-locals.var_spsub_xbar_dn9)) * assign10320_e9366) + (assign10320_e9347 * (((0.5 * (-(-locals.var_spsub_xbar_dn9))) * assign10320_e9364) + (assign10320_e9355 * ((-(-locals.var_spsub_xbar_dn9)) * 0.3333333333333)))))) / (assign10320_e9368 * assign10320_e9368))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10330_e9381: f64 = (1.0 - locals.var_spsub_temp);
            (locals.var_spsub_w, locals.var_spsub_w_dn4, locals.var_spsub_w_dn6, locals.var_spsub_w_dn7, locals.var_spsub_w_dn8, locals.var_spsub_w_dn9, ) = (assign10330_e9381, (-locals.var_spsub_temp_dn4), (-locals.var_spsub_temp_dn6), (-locals.var_spsub_temp_dn7), (-locals.var_spsub_temp_dn8), (-locals.var_spsub_temp_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10340_e9394: f64 = (locals.var_gfsub2 * 0.5);
            let assign10340_e9395: f64 = (locals.var_spsub_xgb + assign10340_e9394);
            let assign10340_e9400: f64 = (locals.var_gfsub2 * 0.25);
            let assign10340_e9401: f64 = (locals.var_spsub_xgb + assign10340_e9400);
            let assign10340_e9403: f64 = (assign10340_e9401 - locals.var_spsub_w);
            let assign10340_e9404: f64 = (assign10340_e9403).sqrt();
            let assign10340_e9405: f64 = (locals.var_gfsub * assign10340_e9404);
            let assign10340_e9406: f64 = (assign10340_e9395 - assign10340_e9405);
            (locals.var_spsub_x1, locals.var_spsub_x1_dn4, locals.var_spsub_x1_dn6, locals.var_spsub_x1_dn7, locals.var_spsub_x1_dn8, locals.var_spsub_x1_dn9, ) = (assign10340_e9406, ((locals.var_spsub_xgb_dn4 + (locals.var_gfsub2_dn4 * 0.5)) - ((locals.var_gfsub_dn4 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn4 + (locals.var_gfsub2_dn4 * 0.25)) - locals.var_spsub_w_dn4) / (2.0 * assign10340_e9404))))), ((locals.var_spsub_xgb_dn6 + (locals.var_gfsub2_dn6 * 0.5)) - ((locals.var_gfsub_dn6 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn6 + (locals.var_gfsub2_dn6 * 0.25)) - locals.var_spsub_w_dn6) / (2.0 * assign10340_e9404))))), ((locals.var_spsub_xgb_dn7 + (locals.var_gfsub2_dn7 * 0.5)) - ((locals.var_gfsub_dn7 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn7 + (locals.var_gfsub2_dn7 * 0.25)) - locals.var_spsub_w_dn7) / (2.0 * assign10340_e9404))))), ((locals.var_spsub_xgb_dn8 + (locals.var_gfsub2_dn8 * 0.5)) - ((locals.var_gfsub_dn8 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn8 + (locals.var_gfsub2_dn8 * 0.25)) - locals.var_spsub_w_dn8) / (2.0 * assign10340_e9404))))), ((locals.var_spsub_xgb_dn9 + (locals.var_gfsub2_dn9 * 0.5)) - ((locals.var_gfsub_dn9 * assign10340_e9404) + (locals.var_gfsub * (((locals.var_spsub_xgb_dn9 + (locals.var_gfsub2_dn9 * 0.25)) - locals.var_spsub_w_dn9) / (2.0 * assign10340_e9404))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10350_e9418: f64 = (locals.var_xn_sub + 3.0);
            (locals.var_spsub_bx, locals.var_spsub_bx_dn4, locals.var_spsub_bx_dn6, locals.var_spsub_bx_dn7, locals.var_spsub_bx_dn8, locals.var_spsub_bx_dn9, ) = (assign10350_e9418, locals.var_xn_sub_dn4, locals.var_xn_sub_dn6, locals.var_xn_sub_dn7, locals.var_xn_sub_dn8, locals.var_xn_sub_dn9, );
        }

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10360_e9431: f64 = (locals.var_spsub_x1 + locals.var_spsub_bx);
            let assign10360_e9434: f64 = (locals.var_spsub_x1 - locals.var_spsub_bx);
            let assign10360_e9437: f64 = (locals.var_spsub_x1 - locals.var_spsub_bx);
            let assign10360_e9438: f64 = (assign10360_e9434 * assign10360_e9437);
            let assign10360_e9440: f64 = (assign10360_e9438 + 5.0);
            let assign10360_e9441: f64 = (assign10360_e9440).sqrt();
            let assign10360_e9442: f64 = (assign10360_e9431 - assign10360_e9441);
            let assign10360_e9443: f64 = (0.5 * assign10360_e9442);
            let assign10360_e9448: f64 = (locals.var_spsub_bx * locals.var_spsub_bx);
            let assign10360_e9450: f64 = (assign10360_e9448 + 5.0);
            let assign10360_e9451: f64 = (assign10360_e9450).sqrt();
            let assign10360_e9452: f64 = (locals.var_spsub_bx - assign10360_e9451);
            let assign10360_e9453: f64 = (0.5 * assign10360_e9452);
            let assign10360_e9454: f64 = (assign10360_e9443 - assign10360_e9453);
            (locals.var_spsub_eta, locals.var_spsub_eta_dn4, locals.var_spsub_eta_dn6, locals.var_spsub_eta_dn7, locals.var_spsub_eta_dn8, locals.var_spsub_eta_dn9, ) = (assign10360_e9454, ((0.5 * ((locals.var_spsub_x1_dn4 + locals.var_spsub_bx_dn4) - ((((locals.var_spsub_x1_dn4 - locals.var_spsub_bx_dn4) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn4 - locals.var_spsub_bx_dn4))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn4 - (((locals.var_spsub_bx_dn4 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn4)) / (2.0 * assign10360_e9451))))), ((0.5 * ((locals.var_spsub_x1_dn6 + locals.var_spsub_bx_dn6) - ((((locals.var_spsub_x1_dn6 - locals.var_spsub_bx_dn6) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn6 - locals.var_spsub_bx_dn6))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn6 - (((locals.var_spsub_bx_dn6 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn6)) / (2.0 * assign10360_e9451))))), ((0.5 * ((locals.var_spsub_x1_dn7 + locals.var_spsub_bx_dn7) - ((((locals.var_spsub_x1_dn7 - locals.var_spsub_bx_dn7) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn7 - locals.var_spsub_bx_dn7))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn7 - (((locals.var_spsub_bx_dn7 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn7)) / (2.0 * assign10360_e9451))))), ((0.5 * ((locals.var_spsub_x1_dn8 + locals.var_spsub_bx_dn8) - ((((locals.var_spsub_x1_dn8 - locals.var_spsub_bx_dn8) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn8 - locals.var_spsub_bx_dn8))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn8 - (((locals.var_spsub_bx_dn8 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn8)) / (2.0 * assign10360_e9451))))), ((0.5 * ((locals.var_spsub_x1_dn9 + locals.var_spsub_bx_dn9) - ((((locals.var_spsub_x1_dn9 - locals.var_spsub_bx_dn9) * assign10360_e9437) + (assign10360_e9434 * (locals.var_spsub_x1_dn9 - locals.var_spsub_bx_dn9))) / (2.0 * assign10360_e9441)))) - (0.5 * (locals.var_spsub_bx_dn9 - (((locals.var_spsub_bx_dn9 * locals.var_spsub_bx) + (locals.var_spsub_bx * locals.var_spsub_bx_dn9)) / (2.0 * assign10360_e9451))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10370_e9466: f64 = (locals.var_spsub_xgb - locals.var_spsub_eta);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10370_e9466, (locals.var_spsub_xgb_dn4 - locals.var_spsub_eta_dn4), (locals.var_spsub_xgb_dn6 - locals.var_spsub_eta_dn6), (locals.var_spsub_xgb_dn7 - locals.var_spsub_eta_dn7), (locals.var_spsub_xgb_dn8 - locals.var_spsub_eta_dn8), (locals.var_spsub_xgb_dn9 - locals.var_spsub_eta_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10380_e9477: f64 = (-locals.var_spsub_eta);
            let assign10380_e9478: f64 = (assign10380_e9477).exp();
            (locals.var_spsub_temp1, locals.var_spsub_temp1_dn4, locals.var_spsub_temp1_dn6, locals.var_spsub_temp1_dn7, locals.var_spsub_temp1_dn8, locals.var_spsub_temp1_dn9, ) = (assign10380_e9478, (assign10380_e9478 * (-locals.var_spsub_eta_dn4)), (assign10380_e9478 * (-locals.var_spsub_eta_dn6)), (assign10380_e9478 * (-locals.var_spsub_eta_dn7)), (assign10380_e9478 * (-locals.var_spsub_eta_dn8)), (assign10380_e9478 * (-locals.var_spsub_eta_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10390_e9492: f64 = (locals.var_spsub_eta * locals.var_spsub_eta);
            let assign10390_e9493: f64 = (2.0 + assign10390_e9492);
            let assign10390_e9494: f64 = (1.0 / assign10390_e9493);
            (locals.var_spsub_temp2, locals.var_spsub_temp2_dn4, locals.var_spsub_temp2_dn6, locals.var_spsub_temp2_dn7, locals.var_spsub_temp2_dn8, locals.var_spsub_temp2_dn9, ) = (assign10390_e9494, (-(((locals.var_spsub_eta_dn4 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn4)) / (assign10390_e9493 * assign10390_e9493))), (-(((locals.var_spsub_eta_dn6 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn6)) / (assign10390_e9493 * assign10390_e9493))), (-(((locals.var_spsub_eta_dn7 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn7)) / (assign10390_e9493 * assign10390_e9493))), (-(((locals.var_spsub_eta_dn8 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn8)) / (assign10390_e9493 * assign10390_e9493))), (-(((locals.var_spsub_eta_dn9 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn9)) / (assign10390_e9493 * assign10390_e9493))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10400_e9506: f64 = (locals.var_spsub_eta * locals.var_spsub_eta);
            let assign10400_e9508: f64 = (assign10400_e9506 * locals.var_spsub_temp2);
            (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9, ) = (assign10400_e9508, ((((locals.var_spsub_eta_dn4 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn4)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn4)), ((((locals.var_spsub_eta_dn6 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn6)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn6)), ((((locals.var_spsub_eta_dn7 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn7)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn7)), ((((locals.var_spsub_eta_dn8 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn8)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn8)), ((((locals.var_spsub_eta_dn9 * locals.var_spsub_eta) + (locals.var_spsub_eta * locals.var_spsub_eta_dn9)) * locals.var_spsub_temp2) + (assign10400_e9506 * locals.var_spsub_temp2_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10410_e9521: f64 = (locals.var_spsub_eta * locals.var_spsub_temp2);
            let assign10410_e9523: f64 = (assign10410_e9521 * locals.var_spsub_temp2);
            let assign10410_e9524: f64 = (4.0 * assign10410_e9523);
            (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9, ) = (assign10410_e9524, (4.0 * ((((locals.var_spsub_eta_dn4 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn4)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn4))), (4.0 * ((((locals.var_spsub_eta_dn6 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn6)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn6))), (4.0 * ((((locals.var_spsub_eta_dn7 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn7)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn7))), (4.0 * ((((locals.var_spsub_eta_dn8 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn8)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn8))), (4.0 * ((((locals.var_spsub_eta_dn9 * locals.var_spsub_temp2) + (locals.var_spsub_eta * locals.var_spsub_temp2_dn9)) * locals.var_spsub_temp2) + (assign10410_e9521 * locals.var_spsub_temp2_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10420_e9536: f64 = (8.0 * locals.var_spsub_temp2);
            let assign10420_e9539: f64 = (12.0 * locals.var_spsub_xi0);
            let assign10420_e9540: f64 = (assign10420_e9536 - assign10420_e9539);
            let assign10420_e9542: f64 = (assign10420_e9540 * locals.var_spsub_temp2);
            let assign10420_e9544: f64 = (assign10420_e9542 * locals.var_spsub_temp2);
            (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9, ) = (assign10420_e9544, ((((((8.0 * locals.var_spsub_temp2_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn4)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn4)), ((((((8.0 * locals.var_spsub_temp2_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn6)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn6)), ((((((8.0 * locals.var_spsub_temp2_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn7)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn7)), ((((((8.0 * locals.var_spsub_temp2_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn8)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn8)), ((((((8.0 * locals.var_spsub_temp2_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp2) + (assign10420_e9540 * locals.var_spsub_temp2_dn9)) * locals.var_spsub_temp2) + (assign10420_e9542 * locals.var_spsub_temp2_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10430_e9557: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
            let assign10430_e9561: f64 = (locals.var_spsub_temp1 + locals.var_spsub_eta);
            let assign10430_e9563: f64 = (assign10430_e9561 - 1.0);
            let assign10430_e9567: f64 = (locals.var_spsub_eta + 1.0);
            let assign10430_e9569: f64 = (assign10430_e9567 + locals.var_spsub_xi0);
            let assign10430_e9570: f64 = (locals.var_spsub_delta * assign10430_e9569);
            let assign10430_e9571: f64 = (assign10430_e9563 - assign10430_e9570);
            let assign10430_e9572: f64 = (locals.var_gfsub2 * assign10430_e9571);
            let assign10430_e9573: f64 = (assign10430_e9557 - assign10430_e9572);
            let assign10430_e9574: f64 = (1e-40_f64).max(assign10430_e9573);
            (locals.var_spsub_a, locals.var_spsub_a_dn4, locals.var_spsub_a_dn6, locals.var_spsub_a_dn7, locals.var_spsub_a_dn8, locals.var_spsub_a_dn9, ) = (assign10430_e9574, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn4 + locals.var_spsub_eta_dn4) - ((locals.var_spsub_delta_dn4 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn4 + locals.var_spsub_xi0_dn4))))))) }, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn6 + locals.var_spsub_eta_dn6) - ((locals.var_spsub_delta_dn6 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn6 + locals.var_spsub_xi0_dn6))))))) }, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn7 + locals.var_spsub_eta_dn7) - ((locals.var_spsub_delta_dn7 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn7 + locals.var_spsub_xi0_dn7))))))) }, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn8 + locals.var_spsub_eta_dn8) - ((locals.var_spsub_delta_dn8 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn8 + locals.var_spsub_xi0_dn8))))))) }, if 1e-40 >= assign10430_e9573 { 0.0 } else { (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10430_e9571) + (locals.var_gfsub2 * ((locals.var_spsub_temp1_dn9 + locals.var_spsub_eta_dn9) - ((locals.var_spsub_delta_dn9 * assign10430_e9569) + (locals.var_spsub_delta * (locals.var_spsub_eta_dn9 + locals.var_spsub_xi0_dn9))))))) }, );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10440_e9590: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
            let assign10440_e9591: f64 = (locals.var_spsub_temp1 - assign10440_e9590);
            let assign10440_e9592: f64 = (locals.var_gfsub2 * assign10440_e9591);
            let assign10440_e9593: f64 = (0.5 * assign10440_e9592);
            let assign10440_e9594: f64 = (1.0 - assign10440_e9593);
            (locals.var_spsub_b, locals.var_spsub_b_dn4, locals.var_spsub_b_dn6, locals.var_spsub_b_dn7, locals.var_spsub_b_dn8, locals.var_spsub_b_dn9, ) = (assign10440_e9594, (-(0.5 * ((locals.var_gfsub2_dn4 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn4 - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4))))))), (-(0.5 * ((locals.var_gfsub2_dn6 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn6 - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6))))))), (-(0.5 * ((locals.var_gfsub2_dn7 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn7 - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7))))))), (-(0.5 * ((locals.var_gfsub2_dn8 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn8 - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8))))))), (-(0.5 * ((locals.var_gfsub2_dn9 * assign10440_e9591) + (locals.var_gfsub2 * (locals.var_spsub_temp1_dn9 - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9))))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10450_e9606: f64 = (2.0 * locals.var_spsub_temp);
            let assign10450_e9610: f64 = (1.0 - locals.var_spsub_temp1);
            let assign10450_e9614: f64 = (1.0 + locals.var_spsub_xi1);
            let assign10450_e9615: f64 = (locals.var_spsub_delta * assign10450_e9614);
            let assign10450_e9616: f64 = (assign10450_e9610 - assign10450_e9615);
            let assign10450_e9617: f64 = (locals.var_gfsub2 * assign10450_e9616);
            let assign10450_e9618: f64 = (assign10450_e9606 + assign10450_e9617);
            (locals.var_spsub_c, locals.var_spsub_c_dn4, locals.var_spsub_c_dn6, locals.var_spsub_c_dn7, locals.var_spsub_c_dn8, locals.var_spsub_c_dn9, ) = (assign10450_e9618, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn4) - ((locals.var_spsub_delta_dn4 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn4)))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn6) - ((locals.var_spsub_delta_dn6 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn6)))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn7) - ((locals.var_spsub_delta_dn7 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn7)))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn8) - ((locals.var_spsub_delta_dn8 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn8)))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10450_e9616) + (locals.var_gfsub2 * ((-locals.var_spsub_temp1_dn9) - ((locals.var_spsub_delta_dn9 * assign10450_e9614) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10460_e9630: f64 = (locals.var_xn_sub - locals.var_spsub_eta);
            let assign10460_e9633: f64 = (locals.var_spsub_a / locals.var_gfsub2);
            let assign10460_e9634: f64 = (assign10460_e9633).ln();
            let assign10460_e9635: f64 = (assign10460_e9630 + assign10460_e9634);
            (locals.var_spsub_tau, locals.var_spsub_tau_dn4, locals.var_spsub_tau_dn6, locals.var_spsub_tau_dn7, locals.var_spsub_tau_dn8, locals.var_spsub_tau_dn9, ) = (assign10460_e9635, ((locals.var_xn_sub_dn4 - locals.var_spsub_eta_dn4) + ((((locals.var_spsub_a_dn4 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn4)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), ((locals.var_xn_sub_dn6 - locals.var_spsub_eta_dn6) + ((((locals.var_spsub_a_dn6 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn6)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), ((locals.var_xn_sub_dn7 - locals.var_spsub_eta_dn7) + ((((locals.var_spsub_a_dn7 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn7)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), ((locals.var_xn_sub_dn8 - locals.var_spsub_eta_dn8) + ((((locals.var_spsub_a_dn8 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn8)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), ((locals.var_xn_sub_dn9 - locals.var_spsub_eta_dn9) + ((((locals.var_spsub_a_dn9 * locals.var_gfsub2) - (locals.var_spsub_a * locals.var_gfsub2_dn9)) / (locals.var_gfsub2 * locals.var_gfsub2)) / assign10460_e9633)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10470_e9647: f64 = (locals.var_spsub_a + locals.var_spsub_c);
            (locals.var_nu, locals.var_nu_dn4, locals.var_nu_dn6, locals.var_nu_dn7, locals.var_nu_dn8, locals.var_nu_dn9, ) = (assign10470_e9647, (locals.var_spsub_a_dn4 + locals.var_spsub_c_dn4), (locals.var_spsub_a_dn6 + locals.var_spsub_c_dn6), (locals.var_spsub_a_dn7 + locals.var_spsub_c_dn7), (locals.var_spsub_a_dn8 + locals.var_spsub_c_dn8), (locals.var_spsub_a_dn9 + locals.var_spsub_c_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10480_e9659: f64 = (locals.var_nu * locals.var_nu);
            let assign10480_e9663: f64 = (0.5 * locals.var_spsub_c);
            let assign10480_e9665: f64 = (assign10480_e9663 * locals.var_spsub_c);
            let assign10480_e9668: f64 = (locals.var_spsub_a * locals.var_spsub_b);
            let assign10480_e9669: f64 = (assign10480_e9665 - assign10480_e9668);
            let assign10480_e9670: f64 = (locals.var_spsub_tau * assign10480_e9669);
            let assign10480_e9671: f64 = (assign10480_e9659 + assign10480_e9670);
            (locals.var_mutau, locals.var_mutau_dn4, locals.var_mutau_dn6, locals.var_mutau_dn7, locals.var_mutau_dn8, locals.var_mutau_dn9, ) = (assign10480_e9671, (((locals.var_nu_dn4 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn4)) + ((locals.var_spsub_tau_dn4 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn4) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn4)) - ((locals.var_spsub_a_dn4 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn4)))))), (((locals.var_nu_dn6 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn6)) + ((locals.var_spsub_tau_dn6 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn6) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn6)) - ((locals.var_spsub_a_dn6 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn6)))))), (((locals.var_nu_dn7 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn7)) + ((locals.var_spsub_tau_dn7 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn7) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn7)) - ((locals.var_spsub_a_dn7 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn7)))))), (((locals.var_nu_dn8 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn8)) + ((locals.var_spsub_tau_dn8 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn8) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn8)) - ((locals.var_spsub_a_dn8 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn8)))))), (((locals.var_nu_dn9 * locals.var_nu) + (locals.var_nu * locals.var_nu_dn9)) + ((locals.var_spsub_tau_dn9 * assign10480_e9669) + (locals.var_spsub_tau * ((((0.5 * locals.var_spsub_c_dn9) * locals.var_spsub_c) + (assign10480_e9663 * locals.var_spsub_c_dn9)) - ((locals.var_spsub_a_dn9 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10490_e9684: f64 = (locals.var_spsub_a * locals.var_nu);
            let assign10490_e9686: f64 = (assign10490_e9684 * locals.var_spsub_tau);
            let assign10490_e9690: f64 = (locals.var_nu / locals.var_mutau);
            let assign10490_e9692: f64 = (assign10490_e9690 * locals.var_spsub_tau);
            let assign10490_e9694: f64 = (assign10490_e9692 * locals.var_spsub_tau);
            let assign10490_e9696: f64 = (assign10490_e9694 * locals.var_spsub_c);
            let assign10490_e9699: f64 = (locals.var_spsub_c * locals.var_spsub_c);
            let assign10490_e9701: f64 = (assign10490_e9699 * 0.3333333333333);
            let assign10490_e9704: f64 = (locals.var_spsub_a * locals.var_spsub_b);
            let assign10490_e9705: f64 = (assign10490_e9701 - assign10490_e9704);
            let assign10490_e9706: f64 = (assign10490_e9696 * assign10490_e9705);
            let assign10490_e9707: f64 = (locals.var_mutau + assign10490_e9706);
            let assign10490_e9708: f64 = (assign10490_e9686 / assign10490_e9707);
            let assign10490_e9709: f64 = (locals.var_spsub_eta + assign10490_e9708);
            (locals.var_spsub_x0, locals.var_spsub_x0_dn4, locals.var_spsub_x0_dn6, locals.var_spsub_x0_dn7, locals.var_spsub_x0_dn8, locals.var_spsub_x0_dn9, ) = (assign10490_e9709, (locals.var_spsub_eta_dn4 + (((((((locals.var_spsub_a_dn4 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn4)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn4)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn4 + (((((((((((locals.var_nu_dn4 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn4)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn4)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn4)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn4)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn4 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn4)) * 0.3333333333333) - ((locals.var_spsub_a_dn4 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn4)))))))) / (assign10490_e9707 * assign10490_e9707))), (locals.var_spsub_eta_dn6 + (((((((locals.var_spsub_a_dn6 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn6)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn6)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn6 + (((((((((((locals.var_nu_dn6 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn6)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn6)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn6)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn6)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn6 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn6)) * 0.3333333333333) - ((locals.var_spsub_a_dn6 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn6)))))))) / (assign10490_e9707 * assign10490_e9707))), (locals.var_spsub_eta_dn7 + (((((((locals.var_spsub_a_dn7 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn7)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn7)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn7 + (((((((((((locals.var_nu_dn7 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn7)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn7)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn7)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn7)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn7 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn7)) * 0.3333333333333) - ((locals.var_spsub_a_dn7 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn7)))))))) / (assign10490_e9707 * assign10490_e9707))), (locals.var_spsub_eta_dn8 + (((((((locals.var_spsub_a_dn8 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn8)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn8)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn8 + (((((((((((locals.var_nu_dn8 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn8)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn8)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn8)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn8)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn8 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn8)) * 0.3333333333333) - ((locals.var_spsub_a_dn8 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn8)))))))) / (assign10490_e9707 * assign10490_e9707))), (locals.var_spsub_eta_dn9 + (((((((locals.var_spsub_a_dn9 * locals.var_nu) + (locals.var_spsub_a * locals.var_nu_dn9)) * locals.var_spsub_tau) + (assign10490_e9684 * locals.var_spsub_tau_dn9)) * assign10490_e9707) - (assign10490_e9686 * (locals.var_mutau_dn9 + (((((((((((locals.var_nu_dn9 * locals.var_mutau) - (locals.var_nu * locals.var_mutau_dn9)) / (locals.var_mutau * locals.var_mutau)) * locals.var_spsub_tau) + (assign10490_e9690 * locals.var_spsub_tau_dn9)) * locals.var_spsub_tau) + (assign10490_e9692 * locals.var_spsub_tau_dn9)) * locals.var_spsub_c) + (assign10490_e9694 * locals.var_spsub_c_dn9)) * assign10490_e9705) + (assign10490_e9696 * ((((locals.var_spsub_c_dn9 * locals.var_spsub_c) + (locals.var_spsub_c * locals.var_spsub_c_dn9)) * 0.3333333333333) - ((locals.var_spsub_a_dn9 * locals.var_spsub_b) + (locals.var_spsub_a * locals.var_spsub_b_dn9)))))))) / (assign10490_e9707 * assign10490_e9707))), );
        }

        let assign10500_e9714: f64 = if locals.var_spsub_x0 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard541 = assign10500_e9714;

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
            let assign10510_e9725: f64 = (locals.var_spsub_x0).exp();
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10510_e9725, (assign10510_e9725 * locals.var_spsub_x0_dn4), (assign10510_e9725 * locals.var_spsub_x0_dn6), (assign10510_e9725 * locals.var_spsub_x0_dn7), (assign10510_e9725 * locals.var_spsub_x0_dn8), (assign10510_e9725 * locals.var_spsub_x0_dn9), );
        }

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
            let assign10520_e9739: f64 = (1.0 / locals.var_spsub_delta0);
            (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9, ) = (assign10520_e9739, (-(locals.var_spsub_delta0_dn4 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn6 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn7 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn8 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), (-(locals.var_spsub_delta0_dn9 / (locals.var_spsub_delta0 * locals.var_spsub_delta0))), );
        }

        if ((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 != 0.0)) {
            let assign10530_e9753: f64 = (locals.var_spsub_delta * locals.var_spsub_delta0);
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10530_e9753, ((locals.var_spsub_delta_dn4 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn4)), ((locals.var_spsub_delta_dn6 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn6)), ((locals.var_spsub_delta_dn7 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn7)), ((locals.var_spsub_delta_dn8 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn8)), ((locals.var_spsub_delta_dn9 * locals.var_spsub_delta0) + (locals.var_spsub_delta * locals.var_spsub_delta0_dn9)), );
        }

        let assign10540_e9759: f64 = (locals.var_xn_sub - 80.0);
        let assign10540_e9760: f64 = if locals.var_spsub_x0 > assign10540_e9759 { 1.0 } else { 0.0 };
        locals.var_guard542 = assign10540_e9760;

        if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 != 0.0)) {
            let assign10550_e9775: f64 = (locals.var_spsub_x0 - locals.var_xn_sub);
            let assign10550_e9776: f64 = (assign10550_e9775).exp();
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10550_e9776, (assign10550_e9776 * (locals.var_spsub_x0_dn4 - locals.var_xn_sub_dn4)), (assign10550_e9776 * (locals.var_spsub_x0_dn6 - locals.var_xn_sub_dn6)), (assign10550_e9776 * (locals.var_spsub_x0_dn7 - locals.var_xn_sub_dn7)), (assign10550_e9776 * (locals.var_spsub_x0_dn8 - locals.var_xn_sub_dn8)), (assign10550_e9776 * (locals.var_spsub_x0_dn9 - locals.var_xn_sub_dn9)), );
        }

        if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 != 0.0)) {
            let assign10560_e9793: f64 = (locals.var_spsub_delta / locals.var_spsub_delta0);
            (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9, ) = (assign10560_e9793, (((locals.var_spsub_delta_dn4 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn4)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn6 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn6)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn7 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn7)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn8 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn8)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), (((locals.var_spsub_delta_dn9 * locals.var_spsub_delta0) - (locals.var_spsub_delta * locals.var_spsub_delta0_dn9)) / (locals.var_spsub_delta0 * locals.var_spsub_delta0)), );
        }

        if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 == 0.0)) {
            let assign10570_e9813: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
            let assign10570_e9815: f64 = (assign10570_e9813 - 80.0);
            let assign10570_e9820: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
            let assign10570_e9822: f64 = (assign10570_e9820 - 80.0);
            let assign10570_e9823: f64 = (0.5 * assign10570_e9822);
            let assign10570_e9827: f64 = (locals.var_xn_sub - locals.var_spsub_x0);
            let assign10570_e9829: f64 = (assign10570_e9827 - 80.0);
            let assign10570_e9831: f64 = (assign10570_e9829 * 0.3333333333333);
            let assign10570_e9832: f64 = (1.0 + assign10570_e9831);
            let assign10570_e9833: f64 = (assign10570_e9823 * assign10570_e9832);
            let assign10570_e9834: f64 = (1.0 + assign10570_e9833);
            let assign10570_e9835: f64 = (assign10570_e9815 * assign10570_e9834);
            let assign10570_e9836: f64 = (1.0 + assign10570_e9835);
            let assign10570_e9837: f64 = (1.80485e-35 / assign10570_e9836);
            (locals.var_spsub_delta0, locals.var_spsub_delta0_dn4, locals.var_spsub_delta0_dn6, locals.var_spsub_delta0_dn7, locals.var_spsub_delta0_dn8, locals.var_spsub_delta0_dn9, ) = (assign10570_e9837, (-((1.80485e-35 * (((locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn4 - locals.var_spsub_x0_dn4) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), (-((1.80485e-35 * (((locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn6 - locals.var_spsub_x0_dn6) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), (-((1.80485e-35 * (((locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn7 - locals.var_spsub_x0_dn7) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), (-((1.80485e-35 * (((locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn8 - locals.var_spsub_x0_dn8) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), (-((1.80485e-35 * (((locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9) * assign10570_e9834) + (assign10570_e9815 * (((0.5 * (locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9)) * assign10570_e9832) + (assign10570_e9823 * ((locals.var_xn_sub_dn9 - locals.var_spsub_x0_dn9) * 0.3333333333333)))))) / (assign10570_e9836 * assign10570_e9836))), );
        }

        if (((((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) && (locals.var_guard541 == 0.0)) && (locals.var_guard542 == 0.0)) {
            let assign10580_e9857: f64 = (locals.var_spsub_x0 - 80.0);
            let assign10580_e9862: f64 = (locals.var_spsub_x0 - 80.0);
            let assign10580_e9863: f64 = (0.5 * assign10580_e9862);
            let assign10580_e9867: f64 = (locals.var_spsub_x0 - 80.0);
            let assign10580_e9869: f64 = (assign10580_e9867 * 0.3333333333333);
            let assign10580_e9870: f64 = (1.0 + assign10580_e9869);
            let assign10580_e9871: f64 = (assign10580_e9863 * assign10580_e9870);
            let assign10580_e9872: f64 = (1.0 + assign10580_e9871);
            let assign10580_e9873: f64 = (assign10580_e9857 * assign10580_e9872);
            let assign10580_e9874: f64 = (1.0 + assign10580_e9873);
            let assign10580_e9875: f64 = (1.80485e-35 / assign10580_e9874);
            (locals.var_spsub_delta1, locals.var_spsub_delta1_dn4, locals.var_spsub_delta1_dn6, locals.var_spsub_delta1_dn7, locals.var_spsub_delta1_dn8, locals.var_spsub_delta1_dn9, ) = (assign10580_e9875, (-((1.80485e-35 * ((locals.var_spsub_x0_dn4 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn4) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn4 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn6 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn6) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn6 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn7 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn7) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn7 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn8 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn8) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn8 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), (-((1.80485e-35 * ((locals.var_spsub_x0_dn9 * assign10580_e9872) + (assign10580_e9857 * (((0.5 * locals.var_spsub_x0_dn9) * assign10580_e9870) + (assign10580_e9863 * (locals.var_spsub_x0_dn9 * 0.3333333333333)))))) / (assign10580_e9874 * assign10580_e9874))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10590_e9889: f64 = (locals.var_spsub_x0 * locals.var_spsub_x0);
            let assign10590_e9890: f64 = (2.0 + assign10590_e9889);
            let assign10590_e9891: f64 = (1.0 / assign10590_e9890);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10590_e9891, (-(((locals.var_spsub_x0_dn4 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn4)) / (assign10590_e9890 * assign10590_e9890))), (-(((locals.var_spsub_x0_dn6 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn6)) / (assign10590_e9890 * assign10590_e9890))), (-(((locals.var_spsub_x0_dn7 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn7)) / (assign10590_e9890 * assign10590_e9890))), (-(((locals.var_spsub_x0_dn8 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn8)) / (assign10590_e9890 * assign10590_e9890))), (-(((locals.var_spsub_x0_dn9 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn9)) / (assign10590_e9890 * assign10590_e9890))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10600_e9903: f64 = (locals.var_spsub_x0 * locals.var_spsub_x0);
            let assign10600_e9905: f64 = (assign10600_e9903 * locals.var_spsub_temp);
            (locals.var_spsub_xi0, locals.var_spsub_xi0_dn4, locals.var_spsub_xi0_dn6, locals.var_spsub_xi0_dn7, locals.var_spsub_xi0_dn8, locals.var_spsub_xi0_dn9, ) = (assign10600_e9905, ((((locals.var_spsub_x0_dn4 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn4)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn4)), ((((locals.var_spsub_x0_dn6 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn6)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn6)), ((((locals.var_spsub_x0_dn7 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn7)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn7)), ((((locals.var_spsub_x0_dn8 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn8)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn8)), ((((locals.var_spsub_x0_dn9 * locals.var_spsub_x0) + (locals.var_spsub_x0 * locals.var_spsub_x0_dn9)) * locals.var_spsub_temp) + (assign10600_e9903 * locals.var_spsub_temp_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10610_e9918: f64 = (locals.var_spsub_x0 * locals.var_spsub_temp);
            let assign10610_e9920: f64 = (assign10610_e9918 * locals.var_spsub_temp);
            let assign10610_e9921: f64 = (4.0 * assign10610_e9920);
            (locals.var_spsub_xi1, locals.var_spsub_xi1_dn4, locals.var_spsub_xi1_dn6, locals.var_spsub_xi1_dn7, locals.var_spsub_xi1_dn8, locals.var_spsub_xi1_dn9, ) = (assign10610_e9921, (4.0 * ((((locals.var_spsub_x0_dn4 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn4))), (4.0 * ((((locals.var_spsub_x0_dn6 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn6))), (4.0 * ((((locals.var_spsub_x0_dn7 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn7))), (4.0 * ((((locals.var_spsub_x0_dn8 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn8))), (4.0 * ((((locals.var_spsub_x0_dn9 * locals.var_spsub_temp) + (locals.var_spsub_x0 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10610_e9918 * locals.var_spsub_temp_dn9))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10620_e9933: f64 = (8.0 * locals.var_spsub_temp);
            let assign10620_e9936: f64 = (12.0 * locals.var_spsub_xi0);
            let assign10620_e9937: f64 = (assign10620_e9933 - assign10620_e9936);
            let assign10620_e9939: f64 = (assign10620_e9937 * locals.var_spsub_temp);
            let assign10620_e9941: f64 = (assign10620_e9939 * locals.var_spsub_temp);
            (locals.var_spsub_xi2, locals.var_spsub_xi2_dn4, locals.var_spsub_xi2_dn6, locals.var_spsub_xi2_dn7, locals.var_spsub_xi2_dn8, locals.var_spsub_xi2_dn9, ) = (assign10620_e9941, ((((((8.0 * locals.var_spsub_temp_dn4) - (12.0 * locals.var_spsub_xi0_dn4)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn4)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn4)), ((((((8.0 * locals.var_spsub_temp_dn6) - (12.0 * locals.var_spsub_xi0_dn6)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn6)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn6)), ((((((8.0 * locals.var_spsub_temp_dn7) - (12.0 * locals.var_spsub_xi0_dn7)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn7)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn7)), ((((((8.0 * locals.var_spsub_temp_dn8) - (12.0 * locals.var_spsub_xi0_dn8)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn8)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn8)), ((((((8.0 * locals.var_spsub_temp_dn9) - (12.0 * locals.var_spsub_xi0_dn9)) * locals.var_spsub_temp) + (assign10620_e9937 * locals.var_spsub_temp_dn9)) * locals.var_spsub_temp) + (assign10620_e9939 * locals.var_spsub_temp_dn9)), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10630_e9953: f64 = (locals.var_spsub_xgb - locals.var_spsub_x0);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10630_e9953, (locals.var_spsub_xgb_dn4 - locals.var_spsub_x0_dn4), (locals.var_spsub_xgb_dn6 - locals.var_spsub_x0_dn6), (locals.var_spsub_xgb_dn7 - locals.var_spsub_x0_dn7), (locals.var_spsub_xgb_dn8 - locals.var_spsub_x0_dn8), (locals.var_spsub_xgb_dn9 - locals.var_spsub_x0_dn9), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10640_e9965: f64 = (2.0 * locals.var_spsub_temp);
            let assign10640_e9969: f64 = (1.0 - locals.var_spsub_delta1);
            let assign10640_e9971: f64 = (assign10640_e9969 + locals.var_spsub_delta0);
            let assign10640_e9975: f64 = (1.0 + locals.var_spsub_xi1);
            let assign10640_e9976: f64 = (locals.var_spsub_delta * assign10640_e9975);
            let assign10640_e9977: f64 = (assign10640_e9971 - assign10640_e9976);
            let assign10640_e9978: f64 = (locals.var_gfsub2 * assign10640_e9977);
            let assign10640_e9979: f64 = (assign10640_e9965 + assign10640_e9978);
            (locals.var_spsub_pc, locals.var_spsub_pc_dn4, locals.var_spsub_pc_dn6, locals.var_spsub_pc_dn7, locals.var_spsub_pc_dn8, locals.var_spsub_pc_dn9, ) = (assign10640_e9979, ((2.0 * locals.var_spsub_temp_dn4) + ((locals.var_gfsub2_dn4 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn4) + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn4)))))), ((2.0 * locals.var_spsub_temp_dn6) + ((locals.var_gfsub2_dn6 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn6) + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn6)))))), ((2.0 * locals.var_spsub_temp_dn7) + ((locals.var_gfsub2_dn7 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn7) + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn7)))))), ((2.0 * locals.var_spsub_temp_dn8) + ((locals.var_gfsub2_dn8 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn8) + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn8)))))), ((2.0 * locals.var_spsub_temp_dn9) + ((locals.var_gfsub2_dn9 * assign10640_e9977) + (locals.var_gfsub2 * (((-locals.var_spsub_delta1_dn9) + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * assign10640_e9975) + (locals.var_spsub_delta * locals.var_spsub_xi1_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10650_e9991: f64 = (locals.var_spsub_temp * locals.var_spsub_temp);
            let assign10650_e9995: f64 = (locals.var_spsub_delta1 + locals.var_spsub_x0);
            let assign10650_e9997: f64 = (assign10650_e9995 - 1.0);
            let assign10650_e9999: f64 = (assign10650_e9997 + locals.var_spsub_delta0);
            let assign10650_e10003: f64 = (locals.var_spsub_x0 + 1.0);
            let assign10650_e10005: f64 = (assign10650_e10003 + locals.var_spsub_xi0);
            let assign10650_e10006: f64 = (locals.var_spsub_delta * assign10650_e10005);
            let assign10650_e10007: f64 = (assign10650_e9999 - assign10650_e10006);
            let assign10650_e10008: f64 = (locals.var_gfsub2 * assign10650_e10007);
            let assign10650_e10009: f64 = (assign10650_e9991 - assign10650_e10008);
            (locals.var_spsub_qc, locals.var_spsub_qc_dn4, locals.var_spsub_qc_dn6, locals.var_spsub_qc_dn7, locals.var_spsub_qc_dn8, locals.var_spsub_qc_dn9, ) = (assign10650_e10009, (((locals.var_spsub_temp_dn4 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn4)) - ((locals.var_gfsub2_dn4 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn4 + locals.var_spsub_x0_dn4) + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn4 + locals.var_spsub_xi0_dn4))))))), (((locals.var_spsub_temp_dn6 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn6)) - ((locals.var_gfsub2_dn6 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn6 + locals.var_spsub_x0_dn6) + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn6 + locals.var_spsub_xi0_dn6))))))), (((locals.var_spsub_temp_dn7 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn7)) - ((locals.var_gfsub2_dn7 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn7 + locals.var_spsub_x0_dn7) + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn7 + locals.var_spsub_xi0_dn7))))))), (((locals.var_spsub_temp_dn8 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn8)) - ((locals.var_gfsub2_dn8 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn8 + locals.var_spsub_x0_dn8) + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn8 + locals.var_spsub_xi0_dn8))))))), (((locals.var_spsub_temp_dn9 * locals.var_spsub_temp) + (locals.var_spsub_temp * locals.var_spsub_temp_dn9)) - ((locals.var_gfsub2_dn9 * assign10650_e10007) + (locals.var_gfsub2 * (((locals.var_spsub_delta1_dn9 + locals.var_spsub_x0_dn9) + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * assign10650_e10005) + (locals.var_spsub_delta * (locals.var_spsub_x0_dn9 + locals.var_spsub_xi0_dn9))))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10660_e10023: f64 = (locals.var_spsub_delta1 + locals.var_spsub_delta0);
            let assign10660_e10026: f64 = (locals.var_spsub_delta * locals.var_spsub_xi2);
            let assign10660_e10027: f64 = (assign10660_e10023 - assign10660_e10026);
            let assign10660_e10028: f64 = (locals.var_gfsub2 * assign10660_e10027);
            let assign10660_e10029: f64 = (2.0 - assign10660_e10028);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10660_e10029, (-((locals.var_gfsub2_dn4 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn4 + locals.var_spsub_delta0_dn4) - ((locals.var_spsub_delta_dn4 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn4)))))), (-((locals.var_gfsub2_dn6 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn6 + locals.var_spsub_delta0_dn6) - ((locals.var_spsub_delta_dn6 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn6)))))), (-((locals.var_gfsub2_dn7 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn7 + locals.var_spsub_delta0_dn7) - ((locals.var_spsub_delta_dn7 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn7)))))), (-((locals.var_gfsub2_dn8 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn8 + locals.var_spsub_delta0_dn8) - ((locals.var_spsub_delta_dn8 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn8)))))), (-((locals.var_gfsub2_dn9 * assign10660_e10027) + (locals.var_gfsub2 * ((locals.var_spsub_delta1_dn9 + locals.var_spsub_delta0_dn9) - ((locals.var_spsub_delta_dn9 * locals.var_spsub_xi2) + (locals.var_spsub_delta * locals.var_spsub_xi2_dn9)))))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10670_e10041: f64 = (locals.var_spsub_pc * locals.var_spsub_pc);
            let assign10670_e10045: f64 = (locals.var_spsub_qc * locals.var_spsub_temp);
            let assign10670_e10046: f64 = (2.0 * assign10670_e10045);
            let assign10670_e10047: f64 = (assign10670_e10041 - assign10670_e10046);
            (locals.var_spsub_temp, locals.var_spsub_temp_dn4, locals.var_spsub_temp_dn6, locals.var_spsub_temp_dn7, locals.var_spsub_temp_dn8, locals.var_spsub_temp_dn9, ) = (assign10670_e10047, (((locals.var_spsub_pc_dn4 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn4)) - (2.0 * ((locals.var_spsub_qc_dn4 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn4)))), (((locals.var_spsub_pc_dn6 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn6)) - (2.0 * ((locals.var_spsub_qc_dn6 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn6)))), (((locals.var_spsub_pc_dn7 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn7)) - (2.0 * ((locals.var_spsub_qc_dn7 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn7)))), (((locals.var_spsub_pc_dn8 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn8)) - (2.0 * ((locals.var_spsub_qc_dn8 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn8)))), (((locals.var_spsub_pc_dn9 * locals.var_spsub_pc) + (locals.var_spsub_pc * locals.var_spsub_pc_dn9)) - (2.0 * ((locals.var_spsub_qc_dn9 * locals.var_spsub_temp) + (locals.var_spsub_qc * locals.var_spsub_temp_dn9)))), );
        }

        if (((locals.var_guard531 != 0.0) && (locals.var_guard537 == 0.0)) && (locals.var_guard538 == 0.0)) {
            let assign10680_e10062: f64 = (locals.var_spsub_temp).sqrt();
            let assign10680_e10063: f64 = (locals.var_spsub_pc + assign10680_e10062);
            let assign10680_e10064: f64 = (locals.var_spsub_qc / assign10680_e10063);
            let assign10680_e10065: f64 = (2.0 * assign10680_e10064);
            let assign10680_e10066: f64 = (locals.var_spsub_x0 + assign10680_e10065);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign10680_e10066, (locals.var_spsub_x0_dn4 + (2.0 * (((locals.var_spsub_qc_dn4 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn4 + (locals.var_spsub_temp_dn4 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), (locals.var_spsub_x0_dn6 + (2.0 * (((locals.var_spsub_qc_dn6 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn6 + (locals.var_spsub_temp_dn6 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), (locals.var_spsub_x0_dn7 + (2.0 * (((locals.var_spsub_qc_dn7 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn7 + (locals.var_spsub_temp_dn7 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), (locals.var_spsub_x0_dn8 + (2.0 * (((locals.var_spsub_qc_dn8 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn8 + (locals.var_spsub_temp_dn8 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), (locals.var_spsub_x0_dn9 + (2.0 * (((locals.var_spsub_qc_dn9 * assign10680_e10063) - (locals.var_spsub_qc * (locals.var_spsub_pc_dn9 + (locals.var_spsub_temp_dn9 / (2.0 * assign10680_e10062))))) / (assign10680_e10063 * assign10680_e10063)))), );
        }

        if (locals.var_guard531 != 0.0) {
            let assign10690_e10073: f64 = (locals.var_temp3 + locals.var_temp2);
            let assign10690_e10074: f64 = (locals.var_temp * assign10690_e10073);
            (locals.var_xg2eff, locals.var_xg2eff_dn4, locals.var_xg2eff_dn6, locals.var_xg2eff_dn7, locals.var_xg2eff_dn8, locals.var_xg2eff_dn9, ) = (assign10690_e10074, ((locals.var_temp_dn4 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn4 + locals.var_temp2_dn4))), ((locals.var_temp_dn6 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn6 + locals.var_temp2_dn6))), ((locals.var_temp_dn7 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn7 + locals.var_temp2_dn7))), ((locals.var_temp_dn8 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn8 + locals.var_temp2_dn8))), ((locals.var_temp_dn9 * assign10690_e10073) + (locals.var_temp * (locals.var_temp3_dn9 + locals.var_temp2_dn9))), );
        }

        if (locals.var_guard531 == 0.0) {
            (locals.var_xg2eff, locals.var_xg2eff_dn4, locals.var_xg2eff_dn6, locals.var_xg2eff_dn7, locals.var_xg2eff_dn8, locals.var_xg2eff_dn9, ) = (locals.var_xg20, locals.var_xg20_dn4, locals.var_xg20_dn6, locals.var_xg20_dn7, locals.var_xg20_dn8, locals.var_xg20_dn9, );
        }

        let assign10710_e10085: f64 = (locals.var_xg10 - locals.var_xg2eff);
        let assign10710_e10086: f64 = (locals.var_keq_1d * assign10710_e10085);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign10710_e10086, (locals.var_keq_1d * (locals.var_xg10_dn4 - locals.var_xg2eff_dn4)), (locals.var_keq_1d * (locals.var_xg10_dn6 - locals.var_xg2eff_dn6)), (locals.var_keq_1d * (locals.var_xg10_dn7 - locals.var_xg2eff_dn7)), (locals.var_keq_1d * (locals.var_xg10_dn8 - locals.var_xg2eff_dn8)), (locals.var_keq_1d * (locals.var_xg10_dn9 - locals.var_xg2eff_dn9)), );

        let assign10720_e10089: f64 = if p.p13 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard543 = assign10720_e10089;

        if (locals.var_guard543 != 0.0) {
            let assign10730_e10094: f64 = (locals.var_temp + locals.var_emin);
            let assign10730_e10097: f64 = (locals.var_temp - locals.var_emin);
            let assign10730_e10100: f64 = (locals.var_temp - locals.var_emin);
            let assign10730_e10101: f64 = (assign10730_e10097 * assign10730_e10100);
            let assign10730_e10104: f64 = (locals.var_emin * locals.var_emin);
            let assign10730_e10105: f64 = (assign10730_e10101 + assign10730_e10104);
            let assign10730_e10106: f64 = (assign10730_e10105).sqrt();
            let assign10730_e10107: f64 = (assign10730_e10094 + assign10730_e10106);
            let assign10730_e10108: f64 = (0.5 * assign10730_e10107);
            (locals.var_e1, locals.var_e1_dn4, locals.var_e1_dn6, locals.var_e1_dn7, locals.var_e1_dn8, locals.var_e1_dn9, ) = (assign10730_e10108, (0.5 * ((locals.var_temp_dn4 + locals.var_emin_dn4) + (((((locals.var_temp_dn4 - locals.var_emin_dn4) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn4 - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign10730_e10106)))), (0.5 * ((locals.var_temp_dn6 + locals.var_emin_dn6) + (((((locals.var_temp_dn6 - locals.var_emin_dn6) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn6 - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign10730_e10106)))), (0.5 * ((locals.var_temp_dn7 + locals.var_emin_dn7) + (((((locals.var_temp_dn7 - locals.var_emin_dn7) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn7 - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign10730_e10106)))), (0.5 * ((locals.var_temp_dn8 + locals.var_emin_dn8) + (((((locals.var_temp_dn8 - locals.var_emin_dn8) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn8 - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign10730_e10106)))), (0.5 * ((locals.var_temp_dn9 + locals.var_emin_dn9) + (((((locals.var_temp_dn9 - locals.var_emin_dn9) * assign10730_e10100) + (assign10730_e10097 * (locals.var_temp_dn9 - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign10730_e10106)))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10740_e10114: f64 = (-locals.var_temp);
            let assign10740_e10116: f64 = (assign10740_e10114 + locals.var_emin);
            let assign10740_e10118: f64 = (-locals.var_temp);
            let assign10740_e10120: f64 = (assign10740_e10118 - locals.var_emin);
            let assign10740_e10122: f64 = (-locals.var_temp);
            let assign10740_e10124: f64 = (assign10740_e10122 - locals.var_emin);
            let assign10740_e10125: f64 = (assign10740_e10120 * assign10740_e10124);
            let assign10740_e10128: f64 = (locals.var_emin * locals.var_emin);
            let assign10740_e10129: f64 = (assign10740_e10125 + assign10740_e10128);
            let assign10740_e10130: f64 = (assign10740_e10129).sqrt();
            let assign10740_e10131: f64 = (assign10740_e10116 + assign10740_e10130);
            let assign10740_e10132: f64 = (0.5 * assign10740_e10131);
            (locals.var_e2, locals.var_e2_dn4, locals.var_e2_dn6, locals.var_e2_dn7, locals.var_e2_dn8, locals.var_e2_dn9, ) = (assign10740_e10132, (0.5 * (((-locals.var_temp_dn4) + locals.var_emin_dn4) + ((((((-locals.var_temp_dn4) - locals.var_emin_dn4) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn4) - locals.var_emin_dn4))) + ((locals.var_emin_dn4 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn4))) / (2.0 * assign10740_e10130)))), (0.5 * (((-locals.var_temp_dn6) + locals.var_emin_dn6) + ((((((-locals.var_temp_dn6) - locals.var_emin_dn6) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn6) - locals.var_emin_dn6))) + ((locals.var_emin_dn6 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn6))) / (2.0 * assign10740_e10130)))), (0.5 * (((-locals.var_temp_dn7) + locals.var_emin_dn7) + ((((((-locals.var_temp_dn7) - locals.var_emin_dn7) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn7) - locals.var_emin_dn7))) + ((locals.var_emin_dn7 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn7))) / (2.0 * assign10740_e10130)))), (0.5 * (((-locals.var_temp_dn8) + locals.var_emin_dn8) + ((((((-locals.var_temp_dn8) - locals.var_emin_dn8) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn8) - locals.var_emin_dn8))) + ((locals.var_emin_dn8 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn8))) / (2.0 * assign10740_e10130)))), (0.5 * (((-locals.var_temp_dn9) + locals.var_emin_dn9) + ((((((-locals.var_temp_dn9) - locals.var_emin_dn9) * assign10740_e10124) + (assign10740_e10120 * ((-locals.var_temp_dn9) - locals.var_emin_dn9))) + ((locals.var_emin_dn9 * locals.var_emin) + (locals.var_emin * locals.var_emin_dn9))) / (2.0 * assign10740_e10130)))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10750_e10138: f64 = (-0.3333333333333);
            let assign10750_e10140: f64 = (locals.var_e1).ln();
            let assign10750_e10141: f64 = (assign10750_e10138 * assign10750_e10140);
            let assign10750_e10142: f64 = (assign10750_e10141).exp();
            let assign10750_e10143: f64 = (locals.var_qq * assign10750_e10142);
            (locals.var_temp1, locals.var_temp1_dn4, locals.var_temp1_dn6, locals.var_temp1_dn7, locals.var_temp1_dn8, locals.var_temp1_dn9, ) = (assign10750_e10143, ((locals.var_qq_dn4 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn4 / locals.var_e1))))), ((locals.var_qq_dn6 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn6 / locals.var_e1))))), ((locals.var_qq_dn7 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn7 / locals.var_e1))))), ((locals.var_qq_dn8 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn8 / locals.var_e1))))), ((locals.var_qq_dn9 * assign10750_e10142) + (locals.var_qq * (assign10750_e10142 * (assign10750_e10138 * (locals.var_e1_dn9 / locals.var_e1))))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10760_e10149: f64 = (-0.3333333333333);
            let assign10760_e10151: f64 = (locals.var_e2).ln();
            let assign10760_e10152: f64 = (assign10760_e10149 * assign10760_e10151);
            let assign10760_e10153: f64 = (assign10760_e10152).exp();
            let assign10760_e10154: f64 = (locals.var_qq * assign10760_e10153);
            (locals.var_temp2, locals.var_temp2_dn4, locals.var_temp2_dn6, locals.var_temp2_dn7, locals.var_temp2_dn8, locals.var_temp2_dn9, ) = (assign10760_e10154, ((locals.var_qq_dn4 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn4 / locals.var_e2))))), ((locals.var_qq_dn6 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn6 / locals.var_e2))))), ((locals.var_qq_dn7 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn7 / locals.var_e2))))), ((locals.var_qq_dn8 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn8 / locals.var_e2))))), ((locals.var_qq_dn9 * assign10760_e10153) + (locals.var_qq * (assign10760_e10153 * (assign10760_e10149 * (locals.var_e2_dn9 / locals.var_e2))))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10770_e10160: f64 = (1.0 - locals.var_temp1);
            let assign10770_e10162: f64 = (assign10770_e10160 - locals.var_temp2);
            (locals.var_temp3, locals.var_temp3_dn4, locals.var_temp3_dn6, locals.var_temp3_dn7, locals.var_temp3_dn8, locals.var_temp3_dn9, ) = (assign10770_e10162, ((-locals.var_temp1_dn4) - locals.var_temp2_dn4), ((-locals.var_temp1_dn6) - locals.var_temp2_dn6), ((-locals.var_temp1_dn7) - locals.var_temp2_dn7), ((-locals.var_temp1_dn8) - locals.var_temp2_dn8), ((-locals.var_temp1_dn9) - locals.var_temp2_dn9), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10780_e10168: f64 = (locals.var_csiprime_0 / locals.var_temp3);
            (locals.var_csiprime, locals.var_csiprime_dn4, locals.var_csiprime_dn6, locals.var_csiprime_dn7, locals.var_csiprime_dn8, locals.var_csiprime_dn9, ) = (assign10780_e10168, (-((locals.var_csiprime_0 * locals.var_temp3_dn4) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn6) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn7) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn8) / (locals.var_temp3 * locals.var_temp3))), (-((locals.var_csiprime_0 * locals.var_temp3_dn9) / (locals.var_temp3 * locals.var_temp3))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10790_e10175: f64 = (locals.var_k1_1d * locals.var_temp1);
            let assign10790_e10176: f64 = (1.0 + assign10790_e10175);
            (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9, ) = (assign10790_e10176, (locals.var_k1_1d * locals.var_temp1_dn4), (locals.var_k1_1d * locals.var_temp1_dn6), (locals.var_k1_1d * locals.var_temp1_dn7), (locals.var_k1_1d * locals.var_temp1_dn8), (locals.var_k1_1d * locals.var_temp1_dn9), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10800_e10183: f64 = (locals.var_k2_1d * locals.var_temp2);
            let assign10800_e10184: f64 = (1.0 + assign10800_e10183);
            (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9, ) = (assign10800_e10184, (locals.var_k2_1d * locals.var_temp2_dn4), (locals.var_k2_1d * locals.var_temp2_dn6), (locals.var_k2_1d * locals.var_temp2_dn7), (locals.var_k2_1d * locals.var_temp2_dn8), (locals.var_k2_1d * locals.var_temp2_dn9), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10810_e10190: f64 = (locals.var_k1_1d * locals.var_temp3);
            let assign10810_e10192: f64 = (assign10810_e10190 / locals.var_tox1fact);
            (locals.var_k1_1d_qm, locals.var_k1_1d_qm_dn4, locals.var_k1_1d_qm_dn6, locals.var_k1_1d_qm_dn7, locals.var_k1_1d_qm_dn8, locals.var_k1_1d_qm_dn9, ) = (assign10810_e10192, ((((locals.var_k1_1d * locals.var_temp3_dn4) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn4)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn6) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn6)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn7) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn7)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn8) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn8)) / (locals.var_tox1fact * locals.var_tox1fact)), ((((locals.var_k1_1d * locals.var_temp3_dn9) * locals.var_tox1fact) - (assign10810_e10190 * locals.var_tox1fact_dn9)) / (locals.var_tox1fact * locals.var_tox1fact)), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10820_e10198: f64 = (locals.var_k2_1d * locals.var_temp3);
            let assign10820_e10200: f64 = (assign10820_e10198 / locals.var_tox2fact);
            (locals.var_k2_1d_qm, locals.var_k2_1d_qm_dn4, locals.var_k2_1d_qm_dn6, locals.var_k2_1d_qm_dn7, locals.var_k2_1d_qm_dn8, locals.var_k2_1d_qm_dn9, ) = (assign10820_e10200, ((((locals.var_k2_1d * locals.var_temp3_dn4) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn4)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn6) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn6)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn7) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn7)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn8) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn8)) / (locals.var_tox2fact * locals.var_tox2fact)), ((((locals.var_k2_1d * locals.var_temp3_dn9) * locals.var_tox2fact) - (assign10820_e10198 * locals.var_tox2fact_dn9)) / (locals.var_tox2fact * locals.var_tox2fact)), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10830_e10208: f64 = (1.0 / locals.var_k1_1d_qm);
            let assign10830_e10209: f64 = (1.0 + assign10830_e10208);
            let assign10830_e10212: f64 = (1.0 / locals.var_k2_1d_qm);
            let assign10830_e10213: f64 = (assign10830_e10209 + assign10830_e10212);
            let assign10830_e10214: f64 = (1.0 / assign10830_e10213);
            (locals.var_keq_1d_qm, locals.var_keq_1d_qm_dn4, locals.var_keq_1d_qm_dn6, locals.var_keq_1d_qm_dn7, locals.var_keq_1d_qm_dn8, locals.var_keq_1d_qm_dn9, ) = (assign10830_e10214, (-(((-(locals.var_k1_1d_qm_dn4 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn4 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), (-(((-(locals.var_k1_1d_qm_dn6 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn6 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), (-(((-(locals.var_k1_1d_qm_dn7 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn7 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), (-(((-(locals.var_k1_1d_qm_dn8 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn8 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), (-(((-(locals.var_k1_1d_qm_dn9 / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + (-(locals.var_k2_1d_qm_dn9 / (locals.var_k2_1d_qm * locals.var_k2_1d_qm)))) / (assign10830_e10213 * assign10830_e10213))), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10840_e10221: f64 = (locals.var_k1_1d_qm * locals.var_temp1);
            let assign10840_e10222: f64 = (1.0 + assign10840_e10221);
            (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9, ) = (assign10840_e10222, ((locals.var_k1_1d_qm_dn4 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn4)), ((locals.var_k1_1d_qm_dn6 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn6)), ((locals.var_k1_1d_qm_dn7 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn7)), ((locals.var_k1_1d_qm_dn8 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn8)), ((locals.var_k1_1d_qm_dn9 * locals.var_temp1) + (locals.var_k1_1d_qm * locals.var_temp1_dn9)), );
        }

        if (locals.var_guard543 != 0.0) {
            let assign10850_e10229: f64 = (locals.var_k2_1d_qm * locals.var_temp2);
            let assign10850_e10230: f64 = (1.0 + assign10850_e10229);
            (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9, ) = (assign10850_e10230, ((locals.var_k2_1d_qm_dn4 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn4)), ((locals.var_k2_1d_qm_dn6 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn6)), ((locals.var_k2_1d_qm_dn7 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn7)), ((locals.var_k2_1d_qm_dn8 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn8)), ((locals.var_k2_1d_qm_dn9 * locals.var_temp2) + (locals.var_k2_1d_qm * locals.var_temp2_dn9)), );
        }

        if (locals.var_guard543 == 0.0) {
            (locals.var_csiprime, locals.var_csiprime_dn4, locals.var_csiprime_dn6, locals.var_csiprime_dn7, locals.var_csiprime_dn8, locals.var_csiprime_dn9, ) = (locals.var_csiprime_0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_k1_1d_qm, locals.var_k1_1d_qm_dn4, locals.var_k1_1d_qm_dn6, locals.var_k1_1d_qm_dn7, locals.var_k1_1d_qm_dn8, locals.var_k1_1d_qm_dn9, ) = (locals.var_k1_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_k2_1d_qm, locals.var_k2_1d_qm_dn4, locals.var_k2_1d_qm_dn6, locals.var_k2_1d_qm_dn7, locals.var_k2_1d_qm_dn8, locals.var_k2_1d_qm_dn9, ) = (locals.var_k2_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_keq_1d_qm, locals.var_keq_1d_qm_dn4, locals.var_keq_1d_qm_dn6, locals.var_keq_1d_qm_dn7, locals.var_keq_1d_qm_dn8, locals.var_keq_1d_qm_dn9, ) = (locals.var_keq_1d, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_tox1fact, locals.var_tox1fact_dn4, locals.var_tox1fact_dn6, locals.var_tox1fact_dn7, locals.var_tox1fact_dn8, locals.var_tox1fact_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
            (locals.var_tox2fact, locals.var_tox2fact_dn4, locals.var_tox2fact_dn6, locals.var_tox2fact_dn7, locals.var_tox2fact_dn8, locals.var_tox2fact_dn9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, );
        }

        let assign10920_e10266: f64 = (locals.var_xg10 - locals.var_xg2eff);
        let assign10920_e10267: f64 = (locals.var_keq_1d_qm * assign10920_e10266);
        (locals.var_dx_wi_1d, locals.var_dx_wi_1d_dn4, locals.var_dx_wi_1d_dn6, locals.var_dx_wi_1d_dn7, locals.var_dx_wi_1d_dn8, locals.var_dx_wi_1d_dn9, ) = (assign10920_e10267, ((locals.var_keq_1d_qm_dn4 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn4 - locals.var_xg2eff_dn4))), ((locals.var_keq_1d_qm_dn6 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn6 - locals.var_xg2eff_dn6))), ((locals.var_keq_1d_qm_dn7 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn7 - locals.var_xg2eff_dn7))), ((locals.var_keq_1d_qm_dn8 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn8 - locals.var_xg2eff_dn8))), ((locals.var_keq_1d_qm_dn9 * assign10920_e10266) + (locals.var_keq_1d_qm * (locals.var_xg10_dn9 - locals.var_xg2eff_dn9))), );

        let assign10930_e10270: f64 = if locals.var_dx_wi_1d > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard544 = assign10930_e10270;

        let assign10940_e10272: f64 = (-locals.var_dx_wi_1d);
        let assign10940_e10274: f64 = if assign10940_e10272 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard545 = assign10940_e10274;

        if ((locals.var_guard544 != 0.0) && (locals.var_guard545 != 0.0)) {
            let assign10950_e10280: f64 = (-locals.var_dx_wi_1d);
            let assign10950_e10281: f64 = (assign10950_e10280).exp();
            let assign10950_e10282: f64 = (1.0 + assign10950_e10281);
            let assign10950_e10283: f64 = (assign10950_e10282).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign10950_e10283, ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn4)) / assign10950_e10282), ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn6)) / assign10950_e10282), ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn7)) / assign10950_e10282), ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn8)) / assign10950_e10282), ((assign10950_e10281 * (-locals.var_dx_wi_1d_dn9)) / assign10950_e10282), );
        }

        if ((locals.var_guard544 != 0.0) && (locals.var_guard545 == 0.0)) {
            let assign10960_e10291: f64 = (-locals.var_dx_wi_1d);
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign10960_e10291, (-locals.var_dx_wi_1d_dn4), (-locals.var_dx_wi_1d_dn6), (-locals.var_dx_wi_1d_dn7), (-locals.var_dx_wi_1d_dn8), (-locals.var_dx_wi_1d_dn9), );
        }

        if (locals.var_guard544 != 0.0) {
            let assign10970_e10298: f64 = (locals.var_dx_wi_1d / locals.var_k1_1d_qm);
            let assign10970_e10299: f64 = (locals.var_xg10 - assign10970_e10298);
            let assign10970_e10301: f64 = (assign10970_e10299 + locals.var_temp);
            let assign10970_e10303: f64 = (assign10970_e10301 - 0.6931471805599);
            (locals.var_x_wi_1d, locals.var_x_wi_1d_dn4, locals.var_x_wi_1d_dn6, locals.var_x_wi_1d_dn7, locals.var_x_wi_1d_dn8, locals.var_x_wi_1d_dn9, ) = (assign10970_e10303, ((locals.var_xg10_dn4 - (((locals.var_dx_wi_1d_dn4 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn4)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn4), ((locals.var_xg10_dn6 - (((locals.var_dx_wi_1d_dn6 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn6)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn6), ((locals.var_xg10_dn7 - (((locals.var_dx_wi_1d_dn7 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn7)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn7), ((locals.var_xg10_dn8 - (((locals.var_dx_wi_1d_dn8 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn8)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn8), ((locals.var_xg10_dn9 - (((locals.var_dx_wi_1d_dn9 * locals.var_k1_1d_qm) - (locals.var_dx_wi_1d * locals.var_k1_1d_qm_dn9)) / (locals.var_k1_1d_qm * locals.var_k1_1d_qm))) + locals.var_temp_dn9), );
        }

        let assign10980_e10308: f64 = if locals.var_dx_wi_1d < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard546 = assign10980_e10308;

        if ((locals.var_guard544 == 0.0) && (locals.var_guard546 != 0.0)) {
            let assign10990_e10315: f64 = (locals.var_dx_wi_1d).exp();
            let assign10990_e10316: f64 = (1.0 + assign10990_e10315);
            let assign10990_e10317: f64 = (assign10990_e10316).ln();
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign10990_e10317, ((assign10990_e10315 * locals.var_dx_wi_1d_dn4) / assign10990_e10316), ((assign10990_e10315 * locals.var_dx_wi_1d_dn6) / assign10990_e10316), ((assign10990_e10315 * locals.var_dx_wi_1d_dn7) / assign10990_e10316), ((assign10990_e10315 * locals.var_dx_wi_1d_dn8) / assign10990_e10316), ((assign10990_e10315 * locals.var_dx_wi_1d_dn9) / assign10990_e10316), );
        }

        if ((locals.var_guard544 == 0.0) && (locals.var_guard546 == 0.0)) {
            (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (locals.var_dx_wi_1d, locals.var_dx_wi_1d_dn4, locals.var_dx_wi_1d_dn6, locals.var_dx_wi_1d_dn7, locals.var_dx_wi_1d_dn8, locals.var_dx_wi_1d_dn9, );
        }

        if (locals.var_guard544 == 0.0) {
            let assign11010_e10333: f64 = (locals.var_dx_wi_1d / locals.var_k2_1d_qm);
            let assign11010_e10334: f64 = (locals.var_xg2eff + assign11010_e10333);
            let assign11010_e10336: f64 = (assign11010_e10334 + locals.var_temp);
            let assign11010_e10338: f64 = (assign11010_e10336 - 0.6931471805599);
            (locals.var_x_wi_1d, locals.var_x_wi_1d_dn4, locals.var_x_wi_1d_dn6, locals.var_x_wi_1d_dn7, locals.var_x_wi_1d_dn8, locals.var_x_wi_1d_dn9, ) = (assign11010_e10338, ((locals.var_xg2eff_dn4 + (((locals.var_dx_wi_1d_dn4 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn4)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn4), ((locals.var_xg2eff_dn6 + (((locals.var_dx_wi_1d_dn6 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn6)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn6), ((locals.var_xg2eff_dn7 + (((locals.var_dx_wi_1d_dn7 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn7)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn7), ((locals.var_xg2eff_dn8 + (((locals.var_dx_wi_1d_dn8 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn8)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn8), ((locals.var_xg2eff_dn9 + (((locals.var_dx_wi_1d_dn9 * locals.var_k2_1d_qm) - (locals.var_dx_wi_1d * locals.var_k2_1d_qm_dn9)) / (locals.var_k2_1d_qm * locals.var_k2_1d_qm))) + locals.var_temp_dn9), );
        }

        let assign11020_e10344: f64 = (locals.var_x_wi_1d + locals.var_xth_1d);
        let assign11020_e10347: f64 = (locals.var_x_wi_1d - locals.var_xth_1d);
        let assign11020_e10350: f64 = (locals.var_x_wi_1d - locals.var_xth_1d);
        let assign11020_e10351: f64 = (assign11020_e10347 * assign11020_e10350);
        let assign11020_e10353: f64 = (assign11020_e10351 + 4.0);
        let assign11020_e10354: f64 = (assign11020_e10353).sqrt();
        let assign11020_e10355: f64 = (assign11020_e10344 - assign11020_e10354);
        let assign11020_e10356: f64 = (0.5 * assign11020_e10355);
        (locals.var_x_1d, locals.var_x_1d_dn4, locals.var_x_1d_dn6, locals.var_x_1d_dn7, locals.var_x_1d_dn8, locals.var_x_1d_dn9, ) = (assign11020_e10356, (0.5 * ((locals.var_x_wi_1d_dn4 + locals.var_xth_1d_dn4) - ((((locals.var_x_wi_1d_dn4 - locals.var_xth_1d_dn4) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn4 - locals.var_xth_1d_dn4))) / (2.0 * assign11020_e10354)))), (0.5 * ((locals.var_x_wi_1d_dn6 + locals.var_xth_1d_dn6) - ((((locals.var_x_wi_1d_dn6 - locals.var_xth_1d_dn6) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn6 - locals.var_xth_1d_dn6))) / (2.0 * assign11020_e10354)))), (0.5 * ((locals.var_x_wi_1d_dn7 + locals.var_xth_1d_dn7) - ((((locals.var_x_wi_1d_dn7 - locals.var_xth_1d_dn7) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn7 - locals.var_xth_1d_dn7))) / (2.0 * assign11020_e10354)))), (0.5 * ((locals.var_x_wi_1d_dn8 + locals.var_xth_1d_dn8) - ((((locals.var_x_wi_1d_dn8 - locals.var_xth_1d_dn8) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn8 - locals.var_xth_1d_dn8))) / (2.0 * assign11020_e10354)))), (0.5 * ((locals.var_x_wi_1d_dn9 + locals.var_xth_1d_dn9) - ((((locals.var_x_wi_1d_dn9 - locals.var_xth_1d_dn9) * assign11020_e10350) + (assign11020_e10347 * (locals.var_x_wi_1d_dn9 - locals.var_xth_1d_dn9))) / (2.0 * assign11020_e10354)))), );

        let assign11030_e10361: f64 = (locals.var_xth_1d - locals.var_x_1d);
        let assign11030_e10362: f64 = (2.0 * assign11030_e10361);
        let assign11030_e10364: f64 = (assign11030_e10362 / locals.var_xsddep);
        let assign11030_e10365: f64 = (1.0 + assign11030_e10364);
        let assign11030_e10366: f64 = (assign11030_e10365).sqrt();
        let assign11030_e10368: f64 = (assign11030_e10366 - 1.0);
        (locals.var_dleff, locals.var_dleff_dn4, locals.var_dleff_dn6, locals.var_dleff_dn7, locals.var_dleff_dn8, locals.var_dleff_dn9, ) = (assign11030_e10368, (((((2.0 * (locals.var_xth_1d_dn4 - locals.var_x_1d_dn4)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn4)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366)), (((((2.0 * (locals.var_xth_1d_dn6 - locals.var_x_1d_dn6)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn6)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366)), (((((2.0 * (locals.var_xth_1d_dn7 - locals.var_x_1d_dn7)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn7)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366)), (((((2.0 * (locals.var_xth_1d_dn8 - locals.var_x_1d_dn8)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn8)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366)), (((((2.0 * (locals.var_xth_1d_dn9 - locals.var_x_1d_dn9)) * locals.var_xsddep) - (assign11030_e10362 * locals.var_xsddep_dn9)) / (locals.var_xsddep * locals.var_xsddep)) / (2.0 * assign11030_e10366)), );

        let assign11040_e10372: f64 = (locals.var_xsddep * locals.var_dleff);
        let assign11040_e10373: f64 = (locals.var_x_1d + assign11040_e10372);
        (locals.var_xedge, locals.var_xedge_dn4, locals.var_xedge_dn6, locals.var_xedge_dn7, locals.var_xedge_dn8, locals.var_xedge_dn9, ) = (assign11040_e10373, (locals.var_x_1d_dn4 + ((locals.var_xsddep_dn4 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn4))), (locals.var_x_1d_dn6 + ((locals.var_xsddep_dn6 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn6))), (locals.var_x_1d_dn7 + ((locals.var_xsddep_dn7 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn7))), (locals.var_x_1d_dn8 + ((locals.var_xsddep_dn8 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn8))), (locals.var_x_1d_dn9 + ((locals.var_xsddep_dn9 * locals.var_dleff) + (locals.var_xsddep * locals.var_dleff_dn9))), );

    }

    pub(super) fn stamp_transient_block_12(
        locals: &mut StampLocals,
    ) {
        let assign11050_e10378: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11050_e10379: f64 = (1.0 + assign11050_e10378);
        let assign11050_e10381: f64 = (assign11050_e10379 + 0.5);
        let assign11050_e10385: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11050_e10386: f64 = (1.0 + assign11050_e10385);
        let assign11050_e10388: f64 = (assign11050_e10386 - 0.5);
        let assign11050_e10392: f64 = (locals.var_pscedlb_i * locals.var_xg20shift);
        let assign11050_e10393: f64 = (1.0 + assign11050_e10392);
        let assign11050_e10395: f64 = (assign11050_e10393 - 0.5);
        let assign11050_e10396: f64 = (assign11050_e10388 * assign11050_e10395);
        let assign11050_e10398: f64 = (assign11050_e10396 + 0.01);
        let assign11050_e10399: f64 = (assign11050_e10398).sqrt();
        let assign11050_e10400: f64 = (assign11050_e10381 + assign11050_e10399);
        let assign11050_e10401: f64 = (0.5 * assign11050_e10400);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign11050_e10401, (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn4) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn4) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn4))) / (2.0 * assign11050_e10399)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn6) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn6) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn6))) / (2.0 * assign11050_e10399)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn7) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn7) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn7))) / (2.0 * assign11050_e10399)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn8) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn8) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn8))) / (2.0 * assign11050_e10399)))), (0.5 * ((locals.var_pscedlb_i * locals.var_xg20shift_dn9) + ((((locals.var_pscedlb_i * locals.var_xg20shift_dn9) * assign11050_e10395) + (assign11050_e10388 * (locals.var_pscedlb_i * locals.var_xg20shift_dn9))) / (2.0 * assign11050_e10399)))), );

        let assign11060_e10406: f64 = (locals.var_psce1_loc * locals.var_temp);
        let assign11060_e10407: f64 = (1.0 + assign11060_e10406);
        let assign11060_e10408: f64 = (1.0 / assign11060_e10407);
        (locals.var_sce1, locals.var_sce1_dn4, locals.var_sce1_dn6, locals.var_sce1_dn7, locals.var_sce1_dn8, locals.var_sce1_dn9, ) = (assign11060_e10408, (-((locals.var_psce1_loc * locals.var_temp_dn4) / (assign11060_e10407 * assign11060_e10407))), (-((locals.var_psce1_loc * locals.var_temp_dn6) / (assign11060_e10407 * assign11060_e10407))), (-((locals.var_psce1_loc * locals.var_temp_dn7) / (assign11060_e10407 * assign11060_e10407))), (-((locals.var_psce1_loc * locals.var_temp_dn8) / (assign11060_e10407 * assign11060_e10407))), (-((locals.var_psce1_loc * locals.var_temp_dn9) / (assign11060_e10407 * assign11060_e10407))), );

        let assign11070_e10413: f64 = (locals.var_psce2_loc * locals.var_temp);
        let assign11070_e10414: f64 = (1.0 + assign11070_e10413);
        let assign11070_e10415: f64 = (1.0 / assign11070_e10414);
        (locals.var_sce2, locals.var_sce2_dn4, locals.var_sce2_dn6, locals.var_sce2_dn7, locals.var_sce2_dn8, locals.var_sce2_dn9, ) = (assign11070_e10415, (-((locals.var_psce2_loc * locals.var_temp_dn4) / (assign11070_e10414 * assign11070_e10414))), (-((locals.var_psce2_loc * locals.var_temp_dn6) / (assign11070_e10414 * assign11070_e10414))), (-((locals.var_psce2_loc * locals.var_temp_dn7) / (assign11070_e10414 * assign11070_e10414))), (-((locals.var_psce2_loc * locals.var_temp_dn8) / (assign11070_e10414 * assign11070_e10414))), (-((locals.var_psce2_loc * locals.var_temp_dn9) / (assign11070_e10414 * assign11070_e10414))), );

        let assign11080_e10418: f64 = (2.0 * locals.var_xd0);
        let assign11080_e10422: f64 = (locals.var_xdsx / locals.var_xd0);
        let assign11080_e10423: f64 = (1.0 + assign11080_e10422);
        let assign11080_e10424: f64 = (assign11080_e10423).sqrt();
        let assign11080_e10426: f64 = (assign11080_e10424 - 1.0);
        let assign11080_e10427: f64 = (assign11080_e10418 * assign11080_e10426);
        let assign11080_e10431: f64 = (locals.var_cfdl_i * locals.var_dleff);
        let assign11080_e10432: f64 = (1.0 + assign11080_e10431);
        let assign11080_e10433: f64 = (assign11080_e10427 * assign11080_e10432);
        let assign11080_e10437: f64 = (locals.var_cfdlb_i * locals.var_xg20shift);
        let assign11080_e10438: f64 = (1.0 + assign11080_e10437);
        let assign11080_e10439: f64 = (assign11080_e10433 * assign11080_e10438);
        (locals.var_temp, locals.var_temp_dn4, locals.var_temp_dn6, locals.var_temp_dn7, locals.var_temp_dn8, locals.var_temp_dn9, ) = (assign11080_e10439, (((((((2.0 * locals.var_xd0_dn4) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn4 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn4)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn4))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn4))), (((((((2.0 * locals.var_xd0_dn6) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn6 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn6)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn6))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn6))), (((((((2.0 * locals.var_xd0_dn7) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn7 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn7)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn7))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn7))), (((((((2.0 * locals.var_xd0_dn8) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn8 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn8)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn8))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn8))), (((((((2.0 * locals.var_xd0_dn9) * assign11080_e10426) + (assign11080_e10418 * ((((locals.var_xdsx_dn9 * locals.var_xd0) - (locals.var_xdsx * locals.var_xd0_dn9)) / (locals.var_xd0 * locals.var_xd0)) / (2.0 * assign11080_e10424)))) * assign11080_e10432) + (assign11080_e10427 * (locals.var_cfdl_i * locals.var_dleff_dn9))) * assign11080_e10438) + (assign11080_e10433 * (locals.var_cfdlb_i * locals.var_xg20shift_dn9))), );

        let assign11090_e10442: f64 = (locals.var_cf1_loc * locals.var_temp);
        (locals.var_dxg1_dibl, locals.var_dxg1_dibl_dn4, locals.var_dxg1_dibl_dn6, locals.var_dxg1_dibl_dn7, locals.var_dxg1_dibl_dn8, locals.var_dxg1_dibl_dn9, ) = (assign11090_e10442, ((locals.var_cf1_loc_dn4 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn4)), ((locals.var_cf1_loc_dn6 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn6)), ((locals.var_cf1_loc_dn7 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn7)), ((locals.var_cf1_loc_dn8 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn8)), ((locals.var_cf1_loc_dn9 * locals.var_temp) + (locals.var_cf1_loc * locals.var_temp_dn9)), );

        let assign11100_e10445: f64 = (locals.var_cf2_loc * locals.var_temp);
        (locals.var_dxg2_dibl, locals.var_dxg2_dibl_dn4, locals.var_dxg2_dibl_dn6, locals.var_dxg2_dibl_dn7, locals.var_dxg2_dibl_dn8, locals.var_dxg2_dibl_dn9, ) = (assign11100_e10445, ((locals.var_cf2_loc_dn4 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn4)), ((locals.var_cf2_loc_dn6 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn6)), ((locals.var_cf2_loc_dn7 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn7)), ((locals.var_cf2_loc_dn8 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn8)), ((locals.var_cf2_loc_dn9 * locals.var_temp) + (locals.var_cf2_loc * locals.var_temp_dn9)), );

        let assign11110_e10448: f64 = (locals.var_xg10 - locals.var_xedge);
        let assign11110_e10450: f64 = (assign11110_e10448 + locals.var_dxg1_dibl);
        let assign11110_e10452: f64 = (assign11110_e10450 * locals.var_sce1);
        let assign11110_e10454: f64 = (assign11110_e10452 + locals.var_xedge);
        let assign11110_e10456: f64 = (assign11110_e10454 + locals.var_dxdsx);
        (locals.var_xg1, locals.var_xg1_dn4, locals.var_xg1_dn6, locals.var_xg1_dn7, locals.var_xg1_dn8, locals.var_xg1_dn9, ) = (assign11110_e10456, ((((((locals.var_xg10_dn4 - locals.var_xedge_dn4) + locals.var_dxg1_dibl_dn4) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn4)) + locals.var_xedge_dn4) + locals.var_dxdsx_dn4), ((((((locals.var_xg10_dn6 - locals.var_xedge_dn6) + locals.var_dxg1_dibl_dn6) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn6)) + locals.var_xedge_dn6) + locals.var_dxdsx_dn6), ((((((locals.var_xg10_dn7 - locals.var_xedge_dn7) + locals.var_dxg1_dibl_dn7) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn7)) + locals.var_xedge_dn7) + locals.var_dxdsx_dn7), ((((((locals.var_xg10_dn8 - locals.var_xedge_dn8) + locals.var_dxg1_dibl_dn8) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn8)) + locals.var_xedge_dn8) + locals.var_dxdsx_dn8), ((((((locals.var_xg10_dn9 - locals.var_xedge_dn9) + locals.var_dxg1_dibl_dn9) * locals.var_sce1) + (assign11110_e10450 * locals.var_sce1_dn9)) + locals.var_xedge_dn9) + locals.var_dxdsx_dn9), );

        let assign11120_e10459: f64 = (locals.var_xg2eff - locals.var_xedge);
        let assign11120_e10461: f64 = (assign11120_e10459 + locals.var_dxg2_dibl);
        let assign11120_e10463: f64 = (assign11120_e10461 * locals.var_sce2);
        let assign11120_e10465: f64 = (assign11120_e10463 + locals.var_xedge);
        let assign11120_e10467: f64 = (assign11120_e10465 + locals.var_dxdsx);
        (locals.var_xg2, locals.var_xg2_dn4, locals.var_xg2_dn6, locals.var_xg2_dn7, locals.var_xg2_dn8, locals.var_xg2_dn9, ) = (assign11120_e10467, ((((((locals.var_xg2eff_dn4 - locals.var_xedge_dn4) + locals.var_dxg2_dibl_dn4) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn4)) + locals.var_xedge_dn4) + locals.var_dxdsx_dn4), ((((((locals.var_xg2eff_dn6 - locals.var_xedge_dn6) + locals.var_dxg2_dibl_dn6) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn6)) + locals.var_xedge_dn6) + locals.var_dxdsx_dn6), ((((((locals.var_xg2eff_dn7 - locals.var_xedge_dn7) + locals.var_dxg2_dibl_dn7) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn7)) + locals.var_xedge_dn7) + locals.var_dxdsx_dn7), ((((((locals.var_xg2eff_dn8 - locals.var_xedge_dn8) + locals.var_dxg2_dibl_dn8) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn8)) + locals.var_xedge_dn8) + locals.var_dxdsx_dn8), ((((((locals.var_xg2eff_dn9 - locals.var_xedge_dn9) + locals.var_dxg2_dibl_dn9) * locals.var_sce2) + (assign11120_e10461 * locals.var_sce2_dn9)) + locals.var_xedge_dn9) + locals.var_dxdsx_dn9), );

        let assign11130_e10473: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11130_e10474: f64 = (locals.var_cic1_i * assign11130_e10473);
        let assign11130_e10475: f64 = (locals.var_xg2 + assign11130_e10474);
        let assign11130_e10477: f64 = (assign11130_e10475 + locals.var_xsatmax);
        let assign11130_e10482: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11130_e10483: f64 = (locals.var_cic1_i * assign11130_e10482);
        let assign11130_e10484: f64 = (locals.var_xg2 + assign11130_e10483);
        let assign11130_e10486: f64 = (assign11130_e10484 - locals.var_xsatmax);
        let assign11130_e10491: f64 = (locals.var_xg1 - locals.var_xg2);
        let assign11130_e10492: f64 = (locals.var_cic1_i * assign11130_e10491);
        let assign11130_e10493: f64 = (locals.var_xg2 + assign11130_e10492);
        let assign11130_e10495: f64 = (assign11130_e10493 - locals.var_xsatmax);
        let assign11130_e10496: f64 = (assign11130_e10486 * assign11130_e10495);
        let assign11130_e10498: f64 = (assign11130_e10496 + 0.01);
        let assign11130_e10499: f64 = (assign11130_e10498).sqrt();
        let assign11130_e10500: f64 = (assign11130_e10477 - assign11130_e10499);
        let assign11130_e10501: f64 = (0.5 * assign11130_e10500);
        (locals.var_xg1x, locals.var_xg1x_dn4, locals.var_xg1x_dn6, locals.var_xg1x_dn7, locals.var_xg1x_dn8, locals.var_xg1x_dn9, ) = (assign11130_e10501, (0.5 * (((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) - locals.var_xsatmax_dn4) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn4 + (locals.var_cic1_i * (locals.var_xg1_dn4 - locals.var_xg2_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign11130_e10499)))), (0.5 * (((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) - locals.var_xsatmax_dn6) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn6 + (locals.var_cic1_i * (locals.var_xg1_dn6 - locals.var_xg2_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign11130_e10499)))), (0.5 * (((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) - locals.var_xsatmax_dn7) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn7 + (locals.var_cic1_i * (locals.var_xg1_dn7 - locals.var_xg2_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign11130_e10499)))), (0.5 * (((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) - locals.var_xsatmax_dn8) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn8 + (locals.var_cic1_i * (locals.var_xg1_dn8 - locals.var_xg2_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign11130_e10499)))), (0.5 * (((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) - locals.var_xsatmax_dn9) * assign11130_e10495) + (assign11130_e10486 * ((locals.var_xg2_dn9 + (locals.var_cic1_i * (locals.var_xg1_dn9 - locals.var_xg2_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign11130_e10499)))), );

        let assign11140_e10507: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11140_e10508: f64 = (locals.var_cic2_i * assign11140_e10507);
        let assign11140_e10509: f64 = (locals.var_xg1 + assign11140_e10508);
        let assign11140_e10511: f64 = (assign11140_e10509 + locals.var_xsatmax);
        let assign11140_e10516: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11140_e10517: f64 = (locals.var_cic2_i * assign11140_e10516);
        let assign11140_e10518: f64 = (locals.var_xg1 + assign11140_e10517);
        let assign11140_e10520: f64 = (assign11140_e10518 - locals.var_xsatmax);
        let assign11140_e10525: f64 = (locals.var_xg2 - locals.var_xg1);
        let assign11140_e10526: f64 = (locals.var_cic2_i * assign11140_e10525);
        let assign11140_e10527: f64 = (locals.var_xg1 + assign11140_e10526);
        let assign11140_e10529: f64 = (assign11140_e10527 - locals.var_xsatmax);
        let assign11140_e10530: f64 = (assign11140_e10520 * assign11140_e10529);
        let assign11140_e10532: f64 = (assign11140_e10530 + 0.01);
        let assign11140_e10533: f64 = (assign11140_e10532).sqrt();
        let assign11140_e10534: f64 = (assign11140_e10511 - assign11140_e10533);
        let assign11140_e10535: f64 = (0.5 * assign11140_e10534);
        (locals.var_xg2x, locals.var_xg2x_dn4, locals.var_xg2x_dn6, locals.var_xg2x_dn7, locals.var_xg2x_dn8, locals.var_xg2x_dn9, ) = (assign11140_e10535, (0.5 * (((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) + locals.var_xsatmax_dn4) - (((((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) - locals.var_xsatmax_dn4) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn4 + (locals.var_cic2_i * (locals.var_xg2_dn4 - locals.var_xg1_dn4))) - locals.var_xsatmax_dn4))) / (2.0 * assign11140_e10533)))), (0.5 * (((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) + locals.var_xsatmax_dn6) - (((((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) - locals.var_xsatmax_dn6) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn6 + (locals.var_cic2_i * (locals.var_xg2_dn6 - locals.var_xg1_dn6))) - locals.var_xsatmax_dn6))) / (2.0 * assign11140_e10533)))), (0.5 * (((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) + locals.var_xsatmax_dn7) - (((((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) - locals.var_xsatmax_dn7) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn7 + (locals.var_cic2_i * (locals.var_xg2_dn7 - locals.var_xg1_dn7))) - locals.var_xsatmax_dn7))) / (2.0 * assign11140_e10533)))), (0.5 * (((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) + locals.var_xsatmax_dn8) - (((((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) - locals.var_xsatmax_dn8) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn8 + (locals.var_cic2_i * (locals.var_xg2_dn8 - locals.var_xg1_dn8))) - locals.var_xsatmax_dn8))) / (2.0 * assign11140_e10533)))), (0.5 * (((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) + locals.var_xsatmax_dn9) - (((((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) - locals.var_xsatmax_dn9) * assign11140_e10529) + (assign11140_e10520 * ((locals.var_xg1_dn9 + (locals.var_cic2_i * (locals.var_xg2_dn9 - locals.var_xg1_dn9))) - locals.var_xsatmax_dn9))) / (2.0 * assign11140_e10533)))), );

        let assign11150_e10538: f64 = (locals.var_k1_1d_qm / locals.var_sce1);
        (locals.var_k1, locals.var_k1_dn4, locals.var_k1_dn6, locals.var_k1_dn7, locals.var_k1_dn8, locals.var_k1_dn9, ) = (assign11150_e10538, (((locals.var_k1_1d_qm_dn4 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn4)) / (locals.var_sce1 * locals.var_sce1)), (((locals.var_k1_1d_qm_dn6 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn6)) / (locals.var_sce1 * locals.var_sce1)), (((locals.var_k1_1d_qm_dn7 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn7)) / (locals.var_sce1 * locals.var_sce1)), (((locals.var_k1_1d_qm_dn8 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn8)) / (locals.var_sce1 * locals.var_sce1)), (((locals.var_k1_1d_qm_dn9 * locals.var_sce1) - (locals.var_k1_1d_qm * locals.var_sce1_dn9)) / (locals.var_sce1 * locals.var_sce1)), );

        let assign11160_e10541: f64 = (locals.var_k2_1d_qm / locals.var_sce2);
        (locals.var_k2, locals.var_k2_dn4, locals.var_k2_dn6, locals.var_k2_dn7, locals.var_k2_dn8, locals.var_k2_dn9, ) = (assign11160_e10541, (((locals.var_k2_1d_qm_dn4 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn4)) / (locals.var_sce2 * locals.var_sce2)), (((locals.var_k2_1d_qm_dn6 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn6)) / (locals.var_sce2 * locals.var_sce2)), (((locals.var_k2_1d_qm_dn7 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn7)) / (locals.var_sce2 * locals.var_sce2)), (((locals.var_k2_1d_qm_dn8 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn8)) / (locals.var_sce2 * locals.var_sce2)), (((locals.var_k2_1d_qm_dn9 * locals.var_sce2) - (locals.var_k2_1d_qm * locals.var_sce2_dn9)) / (locals.var_sce2 * locals.var_sce2)), );

        let assign11170_e10544: f64 = (1.0 / locals.var_k1);
        (locals.var_inv_k1, locals.var_inv_k1_dn4, locals.var_inv_k1_dn6, locals.var_inv_k1_dn7, locals.var_inv_k1_dn8, locals.var_inv_k1_dn9, ) = (assign11170_e10544, (-(locals.var_k1_dn4 / (locals.var_k1 * locals.var_k1))), (-(locals.var_k1_dn6 / (locals.var_k1 * locals.var_k1))), (-(locals.var_k1_dn7 / (locals.var_k1 * locals.var_k1))), (-(locals.var_k1_dn8 / (locals.var_k1 * locals.var_k1))), (-(locals.var_k1_dn9 / (locals.var_k1 * locals.var_k1))), );

        let assign11180_e10547: f64 = (1.0 / locals.var_k2);
        (locals.var_inv_k2, locals.var_inv_k2_dn4, locals.var_inv_k2_dn6, locals.var_inv_k2_dn7, locals.var_inv_k2_dn8, locals.var_inv_k2_dn9, ) = (assign11180_e10547, (-(locals.var_k2_dn4 / (locals.var_k2 * locals.var_k2))), (-(locals.var_k2_dn6 / (locals.var_k2 * locals.var_k2))), (-(locals.var_k2_dn7 / (locals.var_k2 * locals.var_k2))), (-(locals.var_k2_dn8 / (locals.var_k2 * locals.var_k2))), (-(locals.var_k2_dn9 / (locals.var_k2 * locals.var_k2))), );

        let assign11190_e10551: f64 = (1.0 + locals.var_inv_k1);
        let assign11190_e10553: f64 = (assign11190_e10551 + locals.var_inv_k2);
        let assign11190_e10554: f64 = (1.0 / assign11190_e10553);
        (locals.var_keq, locals.var_keq_dn4, locals.var_keq_dn6, locals.var_keq_dn7, locals.var_keq_dn8, locals.var_keq_dn9, ) = (assign11190_e10554, (-((locals.var_inv_k1_dn4 + locals.var_inv_k2_dn4) / (assign11190_e10553 * assign11190_e10553))), (-((locals.var_inv_k1_dn6 + locals.var_inv_k2_dn6) / (assign11190_e10553 * assign11190_e10553))), (-((locals.var_inv_k1_dn7 + locals.var_inv_k2_dn7) / (assign11190_e10553 * assign11190_e10553))), (-((locals.var_inv_k1_dn8 + locals.var_inv_k2_dn8) / (assign11190_e10553 * assign11190_e10553))), (-((locals.var_inv_k1_dn9 + locals.var_inv_k2_dn9) / (assign11190_e10553 * assign11190_e10553))), );

        let assign11200_e10558: f64 = (locals.var_csiprime * locals.var_csiprime);
        let assign11200_e10559: f64 = (locals.var_a0_csisq / assign11200_e10558);
        (locals.var_a0, locals.var_a0_dn4, locals.var_a0_dn6, locals.var_a0_dn7, locals.var_a0_dn8, locals.var_a0_dn9, ) = (assign11200_e10559, (((locals.var_a0_csisq_dn4 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn4 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn4)))) / (assign11200_e10558 * assign11200_e10558)), (((locals.var_a0_csisq_dn6 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn6 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn6)))) / (assign11200_e10558 * assign11200_e10558)), (((locals.var_a0_csisq_dn7 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn7 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn7)))) / (assign11200_e10558 * assign11200_e10558)), (((locals.var_a0_csisq_dn8 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn8 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn8)))) / (assign11200_e10558 * assign11200_e10558)), (((locals.var_a0_csisq_dn9 * assign11200_e10558) - (locals.var_a0_csisq * ((locals.var_csiprime_dn9 * locals.var_csiprime) + (locals.var_csiprime * locals.var_csiprime_dn9)))) / (assign11200_e10558 * assign11200_e10558)), );

        let assign11210_e10562: f64 = (1.0 + locals.var_k1);
        let assign11210_e10565: f64 = (1.0 + locals.var_k2);
        let assign11210_e10566: f64 = (assign11210_e10562 / assign11210_e10565);
        (locals.var_exp_dxth, locals.var_exp_dxth_dn4, locals.var_exp_dxth_dn6, locals.var_exp_dxth_dn7, locals.var_exp_dxth_dn8, locals.var_exp_dxth_dn9, ) = (assign11210_e10566, (((locals.var_k1_dn4 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn4)) / (assign11210_e10565 * assign11210_e10565)), (((locals.var_k1_dn6 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn6)) / (assign11210_e10565 * assign11210_e10565)), (((locals.var_k1_dn7 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn7)) / (assign11210_e10565 * assign11210_e10565)), (((locals.var_k1_dn8 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn8)) / (assign11210_e10565 * assign11210_e10565)), (((locals.var_k1_dn9 * assign11210_e10565) - (assign11210_e10562 * locals.var_k2_dn9)) / (assign11210_e10565 * assign11210_e10565)), );

        let assign11220_e10568: f64 = (locals.var_exp_dxth).ln();
        (locals.var_dxth, locals.var_dxth_dn4, locals.var_dxth_dn6, locals.var_dxth_dn7, locals.var_dxth_dn8, locals.var_dxth_dn9, ) = (assign11220_e10568, (locals.var_exp_dxth_dn4 / locals.var_exp_dxth), (locals.var_exp_dxth_dn6 / locals.var_exp_dxth), (locals.var_exp_dxth_dn7 / locals.var_exp_dxth), (locals.var_exp_dxth_dn8 / locals.var_exp_dxth), (locals.var_exp_dxth_dn9 / locals.var_exp_dxth), );

        let assign11230_e10571: f64 = if locals.var_dxth > 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard547 = assign11230_e10571;

        if (locals.var_guard547 != 0.0) {
            let assign11240_e10575: f64 = (2.0 * locals.var_dxth);
            let assign11240_e10578: f64 = (locals.var_exp_dxth + 1.0);
            let assign11240_e10579: f64 = (assign11240_e10575 * assign11240_e10578);
            let assign11240_e10582: f64 = (locals.var_exp_dxth - 1.0);
            let assign11240_e10583: f64 = (assign11240_e10579 / assign11240_e10582);
            (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9, ) = (assign11240_e10583, ((((((2.0 * locals.var_dxth_dn4) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn4)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn4)) / (assign11240_e10582 * assign11240_e10582)), ((((((2.0 * locals.var_dxth_dn6) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn6)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn6)) / (assign11240_e10582 * assign11240_e10582)), ((((((2.0 * locals.var_dxth_dn7) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn7)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn7)) / (assign11240_e10582 * assign11240_e10582)), ((((((2.0 * locals.var_dxth_dn8) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn8)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn8)) / (assign11240_e10582 * assign11240_e10582)), ((((((2.0 * locals.var_dxth_dn9) * assign11240_e10578) + (assign11240_e10575 * locals.var_exp_dxth_dn9)) * assign11240_e10582) - (assign11240_e10579 * locals.var_exp_dxth_dn9)) / (assign11240_e10582 * assign11240_e10582)), );
        }

        if (locals.var_guard547 == 0.0) {
            let assign11250_e10591: f64 = (2.0 + locals.var_dxth);
            let assign11250_e10592: f64 = (2.0 * assign11250_e10591);
            (locals.var_diff_min, locals.var_diff_min_dn4, locals.var_diff_min_dn6, locals.var_diff_min_dn7, locals.var_diff_min_dn8, locals.var_diff_min_dn9, ) = (assign11250_e10592, (2.0 * locals.var_dxth_dn4), (2.0 * locals.var_dxth_dn6), (2.0 * locals.var_dxth_dn7), (2.0 * locals.var_dxth_dn8), (2.0 * locals.var_dxth_dn9), );
        }

        let assign11260_e10598: f64 = (locals.var_xg1x - locals.var_xg2x);
        let assign11260_e10599: f64 = (locals.var_keq * assign11260_e10598);
        (locals.var_dx_wi, locals.var_dx_wi_dn4, locals.var_dx_wi_dn6, locals.var_dx_wi_dn7, locals.var_dx_wi_dn8, locals.var_dx_wi_dn9, ) = (assign11260_e10599, ((locals.var_keq_dn4 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn4 - locals.var_xg2x_dn4))), ((locals.var_keq_dn6 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn6 - locals.var_xg2x_dn6))), ((locals.var_keq_dn7 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn7 - locals.var_xg2x_dn7))), ((locals.var_keq_dn8 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn8 - locals.var_xg2x_dn8))), ((locals.var_keq_dn9 * assign11260_e10598) + (locals.var_keq * (locals.var_xg1x_dn9 - locals.var_xg2x_dn9))), );

        let assign11270_e10602: f64 = (locals.var_dx_wi * locals.var_dx_wi);
        (locals.var_dx_wisq, locals.var_dx_wisq_dn4, locals.var_dx_wisq_dn6, locals.var_dx_wisq_dn7, locals.var_dx_wisq_dn8, locals.var_dx_wisq_dn9, ) = (assign11270_e10602, ((locals.var_dx_wi_dn4 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn4)), ((locals.var_dx_wi_dn6 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn6)), ((locals.var_dx_wi_dn7 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn7)), ((locals.var_dx_wi_dn8 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn8)), ((locals.var_dx_wi_dn9 * locals.var_dx_wi) + (locals.var_dx_wi * locals.var_dx_wi_dn9)), );

        let assign11280_e10606: f64 = (locals.var_dx_wi * locals.var_inv_k1);
        let assign11280_e10607: f64 = (locals.var_xg1x - assign11280_e10606);
        (locals.var_x1_wi0, locals.var_x1_wi0_dn4, locals.var_x1_wi0_dn6, locals.var_x1_wi0_dn7, locals.var_x1_wi0_dn8, locals.var_x1_wi0_dn9, ) = (assign11280_e10607, (locals.var_xg1x_dn4 - ((locals.var_dx_wi_dn4 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn4))), (locals.var_xg1x_dn6 - ((locals.var_dx_wi_dn6 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn6))), (locals.var_xg1x_dn7 - ((locals.var_dx_wi_dn7 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn7))), (locals.var_xg1x_dn8 - ((locals.var_dx_wi_dn8 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn8))), (locals.var_xg1x_dn9 - ((locals.var_dx_wi_dn9 * locals.var_inv_k1) + (locals.var_dx_wi * locals.var_inv_k1_dn9))), );

        let assign11290_e10611: f64 = (locals.var_dx_wi * locals.var_inv_k2);
        let assign11290_e10612: f64 = (locals.var_xg2x + assign11290_e10611);
        (locals.var_x2_wi0, locals.var_x2_wi0_dn4, locals.var_x2_wi0_dn6, locals.var_x2_wi0_dn7, locals.var_x2_wi0_dn8, locals.var_x2_wi0_dn9, ) = (assign11290_e10612, (locals.var_xg2x_dn4 + ((locals.var_dx_wi_dn4 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn4))), (locals.var_xg2x_dn6 + ((locals.var_dx_wi_dn6 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn6))), (locals.var_xg2x_dn7 + ((locals.var_dx_wi_dn7 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn7))), (locals.var_xg2x_dn8 + ((locals.var_dx_wi_dn8 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn8))), (locals.var_xg2x_dn9 + ((locals.var_dx_wi_dn9 * locals.var_inv_k2) + (locals.var_dx_wi * locals.var_inv_k2_dn9))), );

        let assign11300_e10616: f64 = (locals.var_k1 + 1.0);
        let assign11300_e10617: f64 = (1.0 / assign11300_e10616);
        (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11300_e10617, (-(locals.var_k1_dn4 / (assign11300_e10616 * assign11300_e10616))), (-(locals.var_k1_dn6 / (assign11300_e10616 * assign11300_e10616))), (-(locals.var_k1_dn7 / (assign11300_e10616 * assign11300_e10616))), (-(locals.var_k1_dn8 / (assign11300_e10616 * assign11300_e10616))), (-(locals.var_k1_dn9 / (assign11300_e10616 * assign11300_e10616))), );

        let assign11310_e10621: f64 = (locals.var_k2 + 1.0);
        let assign11310_e10622: f64 = (1.0 / assign11310_e10621);
        (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11310_e10622, (-(locals.var_k2_dn4 / (assign11310_e10621 * assign11310_e10621))), (-(locals.var_k2_dn6 / (assign11310_e10621 * assign11310_e10621))), (-(locals.var_k2_dn7 / (assign11310_e10621 * assign11310_e10621))), (-(locals.var_k2_dn8 / (assign11310_e10621 * assign11310_e10621))), (-(locals.var_k2_dn9 / (assign11310_e10621 * assign11310_e10621))), );

        let assign11320_e10626: f64 = (locals.var_k2 * locals.var_q_temp2);
        let assign11320_e10627: f64 = (locals.var_k1 + assign11320_e10626);
        let assign11320_e10629: f64 = (assign11320_e10627 * locals.var_diff_min);
        let assign11320_e10631: f64 = (assign11320_e10629 / locals.var_a0);
        let assign11320_e10632: f64 = (assign11320_e10631).ln();
        let assign11320_e10634: f64 = assign11320_e10632;
        let assign11320_e10636: f64 = (assign11320_e10634 + 3.0);
        (locals.var_q_x1sat, locals.var_q_x1sat_dn4, locals.var_q_x1sat_dn6, locals.var_q_x1sat_dn7, locals.var_q_x1sat_dn8, locals.var_q_x1sat_dn9, ) = (assign11320_e10636, (((((((locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn4))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631), (((((((locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn6))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631), (((((((locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn7))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631), (((((((locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn8))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631), (((((((locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_temp2) + (locals.var_k2 * locals.var_q_temp2_dn9))) * locals.var_diff_min) + (assign11320_e10627 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign11320_e10629 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign11320_e10631), );

        let assign11330_e10640: f64 = (locals.var_k1 * locals.var_q_temp1);
        let assign11330_e10641: f64 = (locals.var_k2 + assign11330_e10640);
        let assign11330_e10643: f64 = (assign11330_e10641 * locals.var_diff_min);
        let assign11330_e10645: f64 = (assign11330_e10643 / locals.var_a0);
        let assign11330_e10646: f64 = (assign11330_e10645).ln();
        let assign11330_e10648: f64 = assign11330_e10646;
        let assign11330_e10650: f64 = (assign11330_e10648 + 3.0);
        (locals.var_q_x2sat, locals.var_q_x2sat_dn4, locals.var_q_x2sat_dn6, locals.var_q_x2sat_dn7, locals.var_q_x2sat_dn8, locals.var_q_x2sat_dn9, ) = (assign11330_e10650, (((((((locals.var_k2_dn4 + ((locals.var_k1_dn4 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn4))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn4)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645), (((((((locals.var_k2_dn6 + ((locals.var_k1_dn6 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn6))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn6)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645), (((((((locals.var_k2_dn7 + ((locals.var_k1_dn7 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn7))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn7)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645), (((((((locals.var_k2_dn8 + ((locals.var_k1_dn8 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn8))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn8)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645), (((((((locals.var_k2_dn9 + ((locals.var_k1_dn9 * locals.var_q_temp1) + (locals.var_k1 * locals.var_q_temp1_dn9))) * locals.var_diff_min) + (assign11330_e10641 * locals.var_diff_min_dn9)) * locals.var_a0) - (assign11330_e10643 * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign11330_e10645), );

        let assign11340_e10653: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
        let assign11340_e10655: f64 = (assign11340_e10653 * 0.3333333333333);
        let assign11340_e10657: f64 = if assign11340_e10655 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard548 = assign11340_e10657;

        if (locals.var_guard548 != 0.0) {
            let assign11350_e10662: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
            let assign11350_e10664: f64 = (assign11350_e10662 * 0.3333333333333);
            let assign11350_e10665: f64 = (assign11350_e10664).exp();
            let assign11350_e10666: f64 = (1.0 + assign11350_e10665);
            let assign11350_e10667: f64 = (assign11350_e10666).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11350_e10667, ((assign11350_e10665 * ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333)) / assign11350_e10666), ((assign11350_e10665 * ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333)) / assign11350_e10666), ((assign11350_e10665 * ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333)) / assign11350_e10666), ((assign11350_e10665 * ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333)) / assign11350_e10666), ((assign11350_e10665 * ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333)) / assign11350_e10666), );
        }

        if (locals.var_guard548 == 0.0) {
            let assign11360_e10674: f64 = (locals.var_q_x1sat - locals.var_x1_wi0);
            let assign11360_e10676: f64 = (assign11360_e10674 * 0.3333333333333);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11360_e10676, ((locals.var_q_x1sat_dn4 - locals.var_x1_wi0_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_x1_wi0_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_x1_wi0_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_x1_wi0_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_x1_wi0_dn9) * 0.3333333333333), );
        }

        let assign11370_e10682: f64 = (3.0 * locals.var_q_temp3);
        let assign11370_e10683: f64 = (locals.var_q_x1sat - assign11370_e10682);
        (locals.var_q_x1, locals.var_q_x1_dn4, locals.var_q_x1_dn6, locals.var_q_x1_dn7, locals.var_q_x1_dn8, locals.var_q_x1_dn9, ) = (assign11370_e10683, (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4)), (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6)), (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7)), (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8)), (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9)), );

        let assign11380_e10686: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
        let assign11380_e10688: f64 = (assign11380_e10686 * 0.3333333333333);
        let assign11380_e10690: f64 = if assign11380_e10688 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard549 = assign11380_e10690;

        if (locals.var_guard549 != 0.0) {
            let assign11390_e10695: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
            let assign11390_e10697: f64 = (assign11390_e10695 * 0.3333333333333);
            let assign11390_e10698: f64 = (assign11390_e10697).exp();
            let assign11390_e10699: f64 = (1.0 + assign11390_e10698);
            let assign11390_e10700: f64 = (assign11390_e10699).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11390_e10700, ((assign11390_e10698 * ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333)) / assign11390_e10699), ((assign11390_e10698 * ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333)) / assign11390_e10699), ((assign11390_e10698 * ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333)) / assign11390_e10699), ((assign11390_e10698 * ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333)) / assign11390_e10699), ((assign11390_e10698 * ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333)) / assign11390_e10699), );
        }

        if (locals.var_guard549 == 0.0) {
            let assign11400_e10707: f64 = (locals.var_q_x2sat - locals.var_x2_wi0);
            let assign11400_e10709: f64 = (assign11400_e10707 * 0.3333333333333);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11400_e10709, ((locals.var_q_x2sat_dn4 - locals.var_x2_wi0_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_x2_wi0_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_x2_wi0_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_x2_wi0_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_x2_wi0_dn9) * 0.3333333333333), );
        }

        let assign11410_e10715: f64 = (3.0 * locals.var_q_temp3);
        let assign11410_e10716: f64 = (locals.var_q_x2sat - assign11410_e10715);
        (locals.var_q_x2, locals.var_q_x2_dn4, locals.var_q_x2_dn6, locals.var_q_x2_dn7, locals.var_q_x2_dn8, locals.var_q_x2_dn9, ) = (assign11410_e10716, (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4)), (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6)), (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7)), (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8)), (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9)), );

        let assign11420_e10719: f64 = (locals.var_k1 * locals.var_xg1x);
        let assign11420_e10721: f64 = (assign11420_e10719 + locals.var_q_x2);
        let assign11420_e10723: f64 = (assign11420_e10721 * locals.var_q_temp1);
        (locals.var_q_x1_wi, locals.var_q_x1_wi_dn4, locals.var_q_x1_wi_dn6, locals.var_q_x1_wi_dn7, locals.var_q_x1_wi_dn8, locals.var_q_x1_wi_dn9, ) = (assign11420_e10723, (((((locals.var_k1_dn4 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn4)) + locals.var_q_x2_dn4) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn4)), (((((locals.var_k1_dn6 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn6)) + locals.var_q_x2_dn6) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn6)), (((((locals.var_k1_dn7 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn7)) + locals.var_q_x2_dn7) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn7)), (((((locals.var_k1_dn8 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn8)) + locals.var_q_x2_dn8) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn8)), (((((locals.var_k1_dn9 * locals.var_xg1x) + (locals.var_k1 * locals.var_xg1x_dn9)) + locals.var_q_x2_dn9) * locals.var_q_temp1) + (assign11420_e10721 * locals.var_q_temp1_dn9)), );

        let assign11430_e10726: f64 = (locals.var_k2 * locals.var_xg2x);
        let assign11430_e10728: f64 = (assign11430_e10726 + locals.var_q_x1);
        let assign11430_e10730: f64 = (assign11430_e10728 * locals.var_q_temp2);
        (locals.var_q_x2_wi, locals.var_q_x2_wi_dn4, locals.var_q_x2_wi_dn6, locals.var_q_x2_wi_dn7, locals.var_q_x2_wi_dn8, locals.var_q_x2_wi_dn9, ) = (assign11430_e10730, (((((locals.var_k2_dn4 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn4)) + locals.var_q_x1_dn4) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn4)), (((((locals.var_k2_dn6 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn6)) + locals.var_q_x1_dn6) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn6)), (((((locals.var_k2_dn7 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn7)) + locals.var_q_x1_dn7) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn7)), (((((locals.var_k2_dn8 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn8)) + locals.var_q_x1_dn8) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn8)), (((((locals.var_k2_dn9 * locals.var_xg2x) + (locals.var_k2 * locals.var_xg2x_dn9)) + locals.var_q_x1_dn9) * locals.var_q_temp2) + (assign11430_e10728 * locals.var_q_temp2_dn9)), );

        let assign11440_e10733: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
        let assign11440_e10735: f64 = (assign11440_e10733 * 0.3333333333333);
        let assign11440_e10737: f64 = if assign11440_e10735 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard550 = assign11440_e10737;

        if (locals.var_guard550 != 0.0) {
            let assign11450_e10742: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
            let assign11450_e10744: f64 = (assign11450_e10742 * 0.3333333333333);
            let assign11450_e10745: f64 = (assign11450_e10744).exp();
            let assign11450_e10746: f64 = (1.0 + assign11450_e10745);
            let assign11450_e10747: f64 = (assign11450_e10746).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11450_e10747, ((assign11450_e10745 * ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333)) / assign11450_e10746), ((assign11450_e10745 * ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333)) / assign11450_e10746), ((assign11450_e10745 * ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333)) / assign11450_e10746), ((assign11450_e10745 * ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333)) / assign11450_e10746), ((assign11450_e10745 * ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333)) / assign11450_e10746), );
        }

        if (locals.var_guard550 == 0.0) {
            let assign11460_e10754: f64 = (locals.var_q_x1sat - locals.var_q_x1_wi);
            let assign11460_e10756: f64 = (assign11460_e10754 * 0.3333333333333);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11460_e10756, ((locals.var_q_x1sat_dn4 - locals.var_q_x1_wi_dn4) * 0.3333333333333), ((locals.var_q_x1sat_dn6 - locals.var_q_x1_wi_dn6) * 0.3333333333333), ((locals.var_q_x1sat_dn7 - locals.var_q_x1_wi_dn7) * 0.3333333333333), ((locals.var_q_x1sat_dn8 - locals.var_q_x1_wi_dn8) * 0.3333333333333), ((locals.var_q_x1sat_dn9 - locals.var_q_x1_wi_dn9) * 0.3333333333333), );
        }

        let assign11470_e10762: f64 = (3.0 * locals.var_q_temp3);
        let assign11470_e10763: f64 = (locals.var_q_x1sat - assign11470_e10762);
        (locals.var_q_x1, locals.var_q_x1_dn4, locals.var_q_x1_dn6, locals.var_q_x1_dn7, locals.var_q_x1_dn8, locals.var_q_x1_dn9, ) = (assign11470_e10763, (locals.var_q_x1sat_dn4 - (3.0 * locals.var_q_temp3_dn4)), (locals.var_q_x1sat_dn6 - (3.0 * locals.var_q_temp3_dn6)), (locals.var_q_x1sat_dn7 - (3.0 * locals.var_q_temp3_dn7)), (locals.var_q_x1sat_dn8 - (3.0 * locals.var_q_temp3_dn8)), (locals.var_q_x1sat_dn9 - (3.0 * locals.var_q_temp3_dn9)), );

        let assign11480_e10766: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
        let assign11480_e10768: f64 = (assign11480_e10766 * 0.3333333333333);
        let assign11480_e10770: f64 = if assign11480_e10768 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard551 = assign11480_e10770;

        if (locals.var_guard551 != 0.0) {
            let assign11490_e10775: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
            let assign11490_e10777: f64 = (assign11490_e10775 * 0.3333333333333);
            let assign11490_e10778: f64 = (assign11490_e10777).exp();
            let assign11490_e10779: f64 = (1.0 + assign11490_e10778);
            let assign11490_e10780: f64 = (assign11490_e10779).ln();
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11490_e10780, ((assign11490_e10778 * ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333)) / assign11490_e10779), ((assign11490_e10778 * ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333)) / assign11490_e10779), ((assign11490_e10778 * ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333)) / assign11490_e10779), ((assign11490_e10778 * ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333)) / assign11490_e10779), ((assign11490_e10778 * ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333)) / assign11490_e10779), );
        }

        if (locals.var_guard551 == 0.0) {
            let assign11500_e10787: f64 = (locals.var_q_x2sat - locals.var_q_x2_wi);
            let assign11500_e10789: f64 = (assign11500_e10787 * 0.3333333333333);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11500_e10789, ((locals.var_q_x2sat_dn4 - locals.var_q_x2_wi_dn4) * 0.3333333333333), ((locals.var_q_x2sat_dn6 - locals.var_q_x2_wi_dn6) * 0.3333333333333), ((locals.var_q_x2sat_dn7 - locals.var_q_x2_wi_dn7) * 0.3333333333333), ((locals.var_q_x2sat_dn8 - locals.var_q_x2_wi_dn8) * 0.3333333333333), ((locals.var_q_x2sat_dn9 - locals.var_q_x2_wi_dn9) * 0.3333333333333), );
        }

        let assign11510_e10795: f64 = (3.0 * locals.var_q_temp3);
        let assign11510_e10796: f64 = (locals.var_q_x2sat - assign11510_e10795);
        (locals.var_q_x2, locals.var_q_x2_dn4, locals.var_q_x2_dn6, locals.var_q_x2_dn7, locals.var_q_x2_dn8, locals.var_q_x2_dn9, ) = (assign11510_e10796, (locals.var_q_x2sat_dn4 - (3.0 * locals.var_q_temp3_dn4)), (locals.var_q_x2sat_dn6 - (3.0 * locals.var_q_temp3_dn6)), (locals.var_q_x2sat_dn7 - (3.0 * locals.var_q_temp3_dn7)), (locals.var_q_x2sat_dn8 - (3.0 * locals.var_q_temp3_dn8)), (locals.var_q_x2sat_dn9 - (3.0 * locals.var_q_temp3_dn9)), );

        let assign11520_e10799: f64 = (locals.var_xg1x - locals.var_q_x1);
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign11520_e10799, (locals.var_xg1x_dn4 - locals.var_q_x1_dn4), (locals.var_xg1x_dn6 - locals.var_q_x1_dn6), (locals.var_xg1x_dn7 - locals.var_q_x1_dn7), (locals.var_xg1x_dn8 - locals.var_q_x1_dn8), (locals.var_xg1x_dn9 - locals.var_q_x1_dn9), );

        let assign11530_e10802: f64 = (locals.var_xg2x - locals.var_q_x2);
        (locals.var_q2s, locals.var_q2s_dn4, locals.var_q2s_dn6, locals.var_q2s_dn7, locals.var_q2s_dn8, locals.var_q2s_dn9, ) = (assign11530_e10802, (locals.var_xg2x_dn4 - locals.var_q_x2_dn4), (locals.var_xg2x_dn6 - locals.var_q_x2_dn6), (locals.var_xg2x_dn7 - locals.var_q_x2_dn7), (locals.var_xg2x_dn8 - locals.var_q_x2_dn8), (locals.var_xg2x_dn9 - locals.var_q_x2_dn9), );

        (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );

        let assign11560_e10807: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign11560_e10807, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign11570_e10810: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign11570_e10812: f64 = assign11570_e10810;
        let assign11570_e10814: f64 = if assign11570_e10812 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard552 = assign11570_e10814;

        if (locals.var_guard552 != 0.0) {
            let assign11580_e10818: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign11580_e10820: f64 = assign11580_e10818;
            let assign11580_e10821: f64 = (assign11580_e10820).exp();
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11580_e10821, (assign11580_e10821 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign11580_e10821 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign11580_e10821 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign11580_e10821 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign11580_e10821 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)), );
        }

        if (locals.var_guard552 == 0.0) {
            let assign11590_e10830: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign11590_e10832: f64 = assign11590_e10830;
            let assign11590_e10834: f64 = (assign11590_e10832 - 80.0);
            let assign11590_e10839: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign11590_e10841: f64 = assign11590_e10839;
            let assign11590_e10843: f64 = (assign11590_e10841 - 80.0);
            let assign11590_e10844: f64 = (0.5 * assign11590_e10843);
            let assign11590_e10848: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign11590_e10850: f64 = assign11590_e10848;
            let assign11590_e10852: f64 = (assign11590_e10850 - 80.0);
            let assign11590_e10854: f64 = (assign11590_e10852 * 0.3333333333333);
            let assign11590_e10855: f64 = (1.0 + assign11590_e10854);
            let assign11590_e10856: f64 = (assign11590_e10844 * assign11590_e10855);
            let assign11590_e10857: f64 = (1.0 + assign11590_e10856);
            let assign11590_e10858: f64 = (assign11590_e10834 * assign11590_e10857);
            let assign11590_e10859: f64 = (1.0 + assign11590_e10858);
            let assign11590_e10860: f64 = (5.54062e34 * assign11590_e10859);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11590_e10860, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign11590_e10857) + (assign11590_e10834 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign11590_e10855) + (assign11590_e10844 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))), );
        }

        let assign11600_e10865: f64 = (locals.var_a0 * locals.var_q_temp1);
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9, ) = (assign11600_e10865, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)), );

        let assign11610_e10868: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign11610_e10870: f64 = (assign11610_e10868 - locals.var_q_aexp);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign11610_e10870, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9), );

        let assign11620_e10873: f64 = (2.0 * locals.var_k1);
        let assign11620_e10875: f64 = (assign11620_e10873 * locals.var_q_k1q1);
        let assign11620_e10877: f64 = (assign11620_e10875 + locals.var_q_aexp);
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9, ) = (assign11620_e10877, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign11620_e10873 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9), );

        let assign11630_e10880: f64 = (2.0 * locals.var_k1);
        let assign11630_e10882: f64 = (assign11630_e10880 * locals.var_k1);
        let assign11630_e10884: f64 = (assign11630_e10882 - locals.var_q_aexp);
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9, ) = (assign11630_e10884, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign11630_e10880 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9), );

        let assign11640_e10887: f64 = (-0.005);
        let assign11640_e10888: f64 = if locals.var_q_qsq < assign11640_e10887 { 1.0 } else { 0.0 };
        locals.var_guard553 = assign11640_e10888;

        if (locals.var_guard553 != 0.0) {
            let assign11650_e10891: f64 = (locals.var_q_qsq).abs();
            let assign11650_e10892: f64 = (assign11650_e10891).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign11650_e10892, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign11650_e10892)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign11650_e10892)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign11650_e10892)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign11650_e10892)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign11650_e10892)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11660_e10899: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign11660_e10900: f64 = (assign11660_e10899).tan();
            let assign11660_e10901: f64 = (locals.var_q_rac_qsq / assign11660_e10900);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign11660_e10901, (((locals.var_q_rac_qsq_dn4 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), (((locals.var_q_rac_qsq_dn6 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), (((locals.var_q_rac_qsq_dn7 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), (((locals.var_q_rac_qsq_dn8 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), (((locals.var_q_rac_qsq_dn9 * assign11660_e10900) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign11660_e10899).cos() * (assign11660_e10899).cos())))) / (assign11660_e10900 * assign11660_e10900)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11670_e10907: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign11670_e10909: f64 = (assign11670_e10907 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11670_e10909, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign11670_e10907 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11680_e10917: f64 = (2.0 - locals.var_q_qcoth);
            let assign11680_e10918: f64 = (locals.var_q_qcoth * assign11680_e10917);
            let assign11680_e10919: f64 = (locals.var_q_qsq + assign11680_e10918);
            let assign11680_e10921: f64 = (assign11680_e10919 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign11680_e10921, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign11680_e10917) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign11680_e10919 * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11690_e10928: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign11690_e10931: f64 = (1.0 + locals.var_q_qcoth);
            let assign11690_e10932: f64 = (assign11690_e10928 * assign11690_e10931);
            let assign11690_e10933: f64 = (locals.var_q_d1_qsq - assign11690_e10932);
            let assign11690_e10935: f64 = (assign11690_e10933 * locals.var_q_temp1);
            let assign11690_e10938: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign11690_e10940: f64 = (assign11690_e10938 / locals.var_q_d1_qsq);
            let assign11690_e10941: f64 = (assign11690_e10935 + assign11690_e10940);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign11690_e10941, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign11690_e10931) + (assign11690_e10928 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign11690_e10933 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign11690_e10938 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11700_e10948: f64 = (0.5 * locals.var_q_qcoth);
            let assign11700_e10949: f64 = (1.0 - assign11700_e10948);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11700_e10949, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11710_e10955: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign11710_e10957: f64 = (assign11710_e10955 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign11710_e10957, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11710_e10955 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard553 != 0.0) {
            let assign11720_e10963: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign11720_e10968: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign11720_e10969: f64 = (locals.var_q_d1_ln + assign11720_e10968);
            let assign11720_e10970: f64 = (locals.var_q_d1_qsq * assign11720_e10969);
            let assign11720_e10971: f64 = (assign11720_e10963 - assign11720_e10970);
            let assign11720_e10973: f64 = (assign11720_e10971 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign11720_e10973, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign11720_e10969) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign11720_e10971 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        let assign11730_e10978: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard554 = assign11730_e10978;

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11740_e10984: f64 = (locals.var_q_qsq).abs();
            let assign11740_e10985: f64 = (assign11740_e10984).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign11740_e10985, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign11740_e10985)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign11740_e10985)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign11740_e10985)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign11740_e10985)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign11740_e10985)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11750_e10993: f64 = (-locals.var_q_rac_qsq);
            let assign11750_e10994: f64 = (assign11750_e10993).exp();
            (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (assign11750_e10994, (assign11750_e10994 * (-locals.var_q_rac_qsq_dn4)), (assign11750_e10994 * (-locals.var_q_rac_qsq_dn6)), (assign11750_e10994 * (-locals.var_q_rac_qsq_dn7)), (assign11750_e10994 * (-locals.var_q_rac_qsq_dn8)), (assign11750_e10994 * (-locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11760_e11004: f64 = (1.0 + locals.var_q_invexpq);
            let assign11760_e11005: f64 = (locals.var_q_rac_qsq * assign11760_e11004);
            let assign11760_e11008: f64 = (1.0 - locals.var_q_invexpq);
            let assign11760_e11009: f64 = (assign11760_e11005 / assign11760_e11008);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign11760_e11009, (((((locals.var_q_rac_qsq_dn4 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn4))) / (assign11760_e11008 * assign11760_e11008)), (((((locals.var_q_rac_qsq_dn6 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn6))) / (assign11760_e11008 * assign11760_e11008)), (((((locals.var_q_rac_qsq_dn7 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn7))) / (assign11760_e11008 * assign11760_e11008)), (((((locals.var_q_rac_qsq_dn8 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn8))) / (assign11760_e11008 * assign11760_e11008)), (((((locals.var_q_rac_qsq_dn9 * assign11760_e11004) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign11760_e11008) - (assign11760_e11005 * (-locals.var_q_invexpq_dn9))) / (assign11760_e11008 * assign11760_e11008)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11770_e11018: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign11770_e11020: f64 = (assign11770_e11018 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11770_e11020, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign11770_e11018 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11780_e11031: f64 = (2.0 - locals.var_q_qcoth);
            let assign11780_e11032: f64 = (locals.var_q_qcoth * assign11780_e11031);
            let assign11780_e11033: f64 = (locals.var_q_qsq + assign11780_e11032);
            let assign11780_e11035: f64 = (assign11780_e11033 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign11780_e11035, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign11780_e11031) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign11780_e11033 * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11790_e11045: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign11790_e11048: f64 = (1.0 + locals.var_q_qcoth);
            let assign11790_e11049: f64 = (assign11790_e11045 * assign11790_e11048);
            let assign11790_e11050: f64 = (locals.var_q_d1_qsq - assign11790_e11049);
            let assign11790_e11052: f64 = (assign11790_e11050 * locals.var_q_temp1);
            let assign11790_e11055: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign11790_e11057: f64 = (assign11790_e11055 / locals.var_q_d1_qsq);
            let assign11790_e11058: f64 = (assign11790_e11052 + assign11790_e11057);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign11790_e11058, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign11790_e11048) + (assign11790_e11045 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign11790_e11050 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign11790_e11055 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11800_e11068: f64 = (0.5 * locals.var_q_qcoth);
            let assign11800_e11069: f64 = (1.0 - assign11800_e11068);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11800_e11069, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11810_e11078: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign11810_e11080: f64 = (assign11810_e11078 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign11810_e11080, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign11810_e11078 * locals.var_q_temp2_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 != 0.0)) {
            let assign11820_e11089: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign11820_e11094: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign11820_e11095: f64 = (locals.var_q_d1_ln + assign11820_e11094);
            let assign11820_e11096: f64 = (locals.var_q_d1_qsq * assign11820_e11095);
            let assign11820_e11097: f64 = (assign11820_e11089 - assign11820_e11096);
            let assign11820_e11099: f64 = (assign11820_e11097 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign11820_e11099, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign11820_e11095) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign11820_e11097 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11830_e11111: f64 = (locals.var_q_qsq * 0.0166666666667);
            let assign11830_e11115: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign11830_e11119: f64 = (locals.var_q_qsq * 0.025);
            let assign11830_e11120: f64 = (1.0 - assign11830_e11119);
            let assign11830_e11121: f64 = (assign11830_e11115 * assign11830_e11120);
            let assign11830_e11122: f64 = (1.0 - assign11830_e11121);
            let assign11830_e11123: f64 = (assign11830_e11111 * assign11830_e11122);
            let assign11830_e11124: f64 = (1.0 - assign11830_e11123);
            let assign11830_e11125: f64 = (0.1666666666667 * assign11830_e11124);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign11830_e11125, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign11830_e11122) + (assign11830_e11111 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign11830_e11120) + (assign11830_e11115 * (-(locals.var_q_qsq_dn9 * 0.025))))))))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11840_e11136: f64 = (locals.var_q_qsq * locals.var_q_temp3);
            let assign11840_e11137: f64 = (2.0 + assign11840_e11136);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign11840_e11137, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)), );
        }

    }

    pub(super) fn stamp_transient_block_13(
        locals: &mut StampLocals,
    ) {
        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11850_e11149: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign11850_e11153: f64 = (locals.var_q_qsq * 0.0357142857143);
            let assign11850_e11157: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign11850_e11158: f64 = (1.0 - assign11850_e11157);
            let assign11850_e11159: f64 = (assign11850_e11153 * assign11850_e11158);
            let assign11850_e11160: f64 = (1.0 - assign11850_e11159);
            let assign11850_e11161: f64 = (assign11850_e11149 * assign11850_e11160);
            let assign11850_e11162: f64 = (1.0 - assign11850_e11161);
            let assign11850_e11163: f64 = (0.1666666666667 * assign11850_e11162);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign11850_e11163, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign11850_e11160) + (assign11850_e11149 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign11850_e11158) + (assign11850_e11153 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11860_e11173: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign11860_e11173, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11870_e11185: f64 = (locals.var_q_qsq * 0.0714285714286);
            let assign11870_e11189: f64 = (0.05 * locals.var_q_qsq);
            let assign11870_e11193: f64 = (0.0420875420875421 * locals.var_q_qsq);
            let assign11870_e11194: f64 = (1.0 - assign11870_e11193);
            let assign11870_e11195: f64 = (assign11870_e11189 * assign11870_e11194);
            let assign11870_e11196: f64 = (1.0 - assign11870_e11195);
            let assign11870_e11197: f64 = (assign11870_e11185 * assign11870_e11196);
            let assign11870_e11198: f64 = (1.0 - assign11870_e11197);
            let assign11870_e11199: f64 = (0.0055555555556 * assign11870_e11198);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11870_e11199, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn4) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn6) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn7) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn8) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign11870_e11196) + (assign11870_e11185 * (-(((0.05 * locals.var_q_qsq_dn9) * assign11870_e11194) + (assign11870_e11189 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11880_e11209: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
            let assign11880_e11212: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
            let assign11880_e11214: f64 = (assign11880_e11212 * locals.var_q_temp2);
            let assign11880_e11215: f64 = (assign11880_e11209 - assign11880_e11214);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign11880_e11215, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign11880_e11212 * locals.var_q_temp2_dn9))), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11890_e11224: f64 = (-0.5);
            let assign11890_e11226: f64 = (assign11890_e11224 * locals.var_q_d1_qsq);
            let assign11890_e11228: f64 = (assign11890_e11226 * locals.var_q_temp3);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign11890_e11228, (((assign11890_e11224 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn4)), (((assign11890_e11224 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn6)), (((assign11890_e11224 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn7)), (((assign11890_e11224 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn8)), (((assign11890_e11224 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign11890_e11226 * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard553 == 0.0) && (locals.var_guard554 == 0.0)) {
            let assign11900_e11237: f64 = (-0.5);
            let assign11900_e11239: f64 = (assign11900_e11237 * locals.var_q_d2_qsq);
            let assign11900_e11241: f64 = (assign11900_e11239 * locals.var_q_temp3);
            let assign11900_e11244: f64 = (0.25 * 0.0055555555556);
            let assign11900_e11246: f64 = (assign11900_e11244 * locals.var_q_d1_qsq);
            let assign11900_e11248: f64 = (assign11900_e11246 * locals.var_q_d1_qsq);
            let assign11900_e11252: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign11900_e11256: f64 = (0.075 * locals.var_q_qsq);
            let assign11900_e11257: f64 = (2.0 - assign11900_e11256);
            let assign11900_e11258: f64 = (assign11900_e11252 * assign11900_e11257);
            let assign11900_e11259: f64 = (1.0 - assign11900_e11258);
            let assign11900_e11260: f64 = (assign11900_e11248 * assign11900_e11259);
            let assign11900_e11261: f64 = (assign11900_e11241 + assign11900_e11260);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign11900_e11261, ((((assign11900_e11237 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn4)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn4)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign11900_e11237 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn6)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn6)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign11900_e11237 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn7)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn7)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign11900_e11237 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn8)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn8)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign11900_e11237 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign11900_e11239 * locals.var_q_temp3_dn9)) + (((((assign11900_e11244 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign11900_e11246 * locals.var_q_d1_qsq_dn9)) * assign11900_e11259) + (assign11900_e11248 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign11900_e11257) + (assign11900_e11252 * (-(0.075 * locals.var_q_qsq_dn9)))))))), );
        }

        let assign11910_e11266: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard555 = assign11910_e11266;

        if (locals.var_guard555 != 0.0) {
            let assign11920_e11270: f64 = (4.0 * locals.var_q_qsq);
            let assign11920_e11275: f64 = (2.0 - locals.var_q_invexpq);
            let assign11920_e11276: f64 = (locals.var_q_invexpq * assign11920_e11275);
            let assign11920_e11277: f64 = (1.0 - assign11920_e11276);
            let assign11920_e11278: f64 = (assign11920_e11270 / assign11920_e11277);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11920_e11278, ((((4.0 * locals.var_q_qsq_dn4) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn4 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign11920_e11277 * assign11920_e11277)), ((((4.0 * locals.var_q_qsq_dn6) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn6 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign11920_e11277 * assign11920_e11277)), ((((4.0 * locals.var_q_qsq_dn7) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn7 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign11920_e11277 * assign11920_e11277)), ((((4.0 * locals.var_q_qsq_dn8) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn8 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign11920_e11277 * assign11920_e11277)), ((((4.0 * locals.var_q_qsq_dn9) * assign11920_e11277) - (assign11920_e11270 * (-((locals.var_q_invexpq_dn9 * assign11920_e11275) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign11920_e11277 * assign11920_e11277)), );
        }

        if (locals.var_guard555 != 0.0) {
            let assign11930_e11284: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign11930_e11284, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)), );
        }

        if (locals.var_guard555 != 0.0) {
            let assign11940_e11289: f64 = (locals.var_q_temp2).ln();
            let assign11940_e11291: f64 = (assign11940_e11289 - locals.var_q_rac_qsq);
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign11940_e11291, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9), );
        }

        let assign11950_e11296: f64 = (-0.005);
        let assign11950_e11297: f64 = if locals.var_q_qsq < assign11950_e11296 { 1.0 } else { 0.0 };
        locals.var_guard556 = assign11950_e11297;

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
            let assign11960_e11304: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign11960_e11305: f64 = (assign11960_e11304).sin();
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign11960_e11305, ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign11960_e11304).cos() * (0.5 * locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
            let assign11970_e11313: f64 = (-locals.var_q_qsq);
            let assign11970_e11316: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
            let assign11970_e11317: f64 = (assign11970_e11313 / assign11970_e11316);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign11970_e11317, ((((-locals.var_q_qsq_dn4) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign11970_e11316 * assign11970_e11316)), ((((-locals.var_q_qsq_dn6) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign11970_e11316 * assign11970_e11316)), ((((-locals.var_q_qsq_dn7) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign11970_e11316 * assign11970_e11316)), ((((-locals.var_q_qsq_dn8) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign11970_e11316 * assign11970_e11316)), ((((-locals.var_q_qsq_dn9) * assign11970_e11316) - (assign11970_e11313 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign11970_e11316 * assign11970_e11316)), );
        }

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 != 0.0)) {
            let assign11980_e11325: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign11980_e11325, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 == 0.0)) {
            let assign11990_e11336: f64 = (locals.var_q_qsq * 0.3333333333333);
            let assign11990_e11340: f64 = (0.05 * locals.var_q_qsq);
            let assign11990_e11344: f64 = (0.0396825396825397 * locals.var_q_qsq);
            let assign11990_e11345: f64 = (1.0 - assign11990_e11344);
            let assign11990_e11346: f64 = (assign11990_e11340 * assign11990_e11345);
            let assign11990_e11347: f64 = (1.0 - assign11990_e11346);
            let assign11990_e11348: f64 = (assign11990_e11336 * assign11990_e11347);
            let assign11990_e11349: f64 = (4.0 - assign11990_e11348);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign11990_e11349, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn4) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn6) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn7) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn8) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign11990_e11347) + (assign11990_e11336 * (-(((0.05 * locals.var_q_qsq_dn9) * assign11990_e11345) + (assign11990_e11340 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))), );
        }

        if ((locals.var_guard555 == 0.0) && (locals.var_guard556 == 0.0)) {
            let assign12000_e11358: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign12000_e11358, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        let assign12010_e11363: f64 = (1.01 * locals.var_q_k1q1);
        let assign12010_e11365: f64 = (assign12010_e11363 + locals.var_q_qcoth);
        let assign12010_e11367: f64 = if assign12010_e11365 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard557 = assign12010_e11367;

        if (locals.var_guard557 != 0.0) {
            let assign12020_e11371: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign12020_e11371, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9), );
        }

        if (locals.var_guard557 != 0.0) {
            let assign12030_e11377: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign12030_e11377, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9), );
        }

        if (locals.var_guard557 != 0.0) {
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12050_e11389: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
            let assign12050_e11390: f64 = (1.0 / assign12050_e11389);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign12050_e11390, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign12050_e11389 * assign12050_e11389))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign12050_e11389 * assign12050_e11389))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign12050_e11389 * assign12050_e11389))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign12050_e11389 * assign12050_e11389))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign12050_e11389 * assign12050_e11389))), );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12060_e11397: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign12060_e11397, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9), );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12070_e11404: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
            let assign12070_e11406: f64 = (assign12070_e11404 * locals.var_q_temp2);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign12070_e11406, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign12070_e11404 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12080_e11413: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
            let assign12080_e11415: f64 = (assign12080_e11413 - locals.var_q_aexp);
            let assign12080_e11418: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
            let assign12080_e11419: f64 = (assign12080_e11415 - assign12080_e11418);
            let assign12080_e11421: f64 = (assign12080_e11419 * locals.var_q_temp2);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign12080_e11421, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign12080_e11419 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard557 == 0.0) {
            let assign12090_e11428: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
            let assign12090_e11431: f64 = (2.0 * locals.var_q_temp3);
            let assign12090_e11433: f64 = (assign12090_e11431 * locals.var_q_d1_expnum);
            let assign12090_e11434: f64 = (assign12090_e11428 + assign12090_e11433);
            let assign12090_e11436: f64 = (assign12090_e11434 + locals.var_q_aexp);
            let assign12090_e11440: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
            let assign12090_e11441: f64 = (locals.var_q_d2_ln + assign12090_e11440);
            let assign12090_e11443: f64 = (assign12090_e11441 * locals.var_q_sh_term);
            let assign12090_e11444: f64 = (assign12090_e11436 - assign12090_e11443);
            let assign12090_e11446: f64 = (assign12090_e11444 * locals.var_q_temp2);
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (assign12090_e11446, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign12090_e11431 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign12090_e11441 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign12090_e11444 * locals.var_q_temp2_dn9)), );
        }

        let assign12100_e11451: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard558 = assign12100_e11451;

        if (locals.var_guard558 != 0.0) {
            let assign12110_e11454: f64 = (locals.var_q_expnum).ln();
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign12110_e11454, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum), );
        }

        if (locals.var_guard558 != 0.0) {
            let assign12120_e11460: f64 = (1.0 / locals.var_q_expnum);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12120_e11460, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))), );
        }

        if (locals.var_guard558 != 0.0) {
            let assign12130_e11466: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign12130_e11466, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard558 != 0.0) {
            let assign12140_e11472: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
            let assign12140_e11475: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
            let assign12140_e11476: f64 = (assign12140_e11472 - assign12140_e11475);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign12140_e11476, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))), );
        }

        if (locals.var_guard558 == 0.0) {
            let assign12150_e11483: f64 = (locals.var_q_k1q1 + 0.6931471805599);
            let assign12150_e11485: f64 = (-locals.var_q_k1q1);
            let assign12150_e11486: f64 = (assign12150_e11485).ln();
            let assign12150_e11487: f64 = (assign12150_e11483 + assign12150_e11486);
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign12150_e11487, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign12150_e11485)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign12150_e11485)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign12150_e11485)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign12150_e11485)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign12150_e11485)), );
        }

        if (locals.var_guard558 == 0.0) {
            let assign12160_e11494: f64 = (1.0 / locals.var_q1s);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12160_e11494, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))), );
        }

        if (locals.var_guard558 == 0.0) {
            let assign12170_e11501: f64 = (locals.var_k1 + locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign12170_e11501, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9), );
        }

        if (locals.var_guard558 == 0.0) {
            let assign12180_e11507: f64 = (-locals.var_q_temp1);
            let assign12180_e11509: f64 = (assign12180_e11507 * locals.var_q_temp1);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign12180_e11509, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign12180_e11507 * locals.var_q_temp1_dn9)), );
        }

        let assign12190_e11514: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign12190_e11516: f64 = (assign12190_e11514 + locals.var_q1s);
        let assign12190_e11519: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign12190_e11520: f64 = (assign12190_e11516 + assign12190_e11519);
        let assign12190_e11522: f64 = (assign12190_e11520 - locals.var_q_ln_term);
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9, ) = (assign12190_e11522, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9), );

        let assign12200_e11526: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign12200_e11527: f64 = (1.0 + assign12200_e11526);
        let assign12200_e11529: f64 = (assign12200_e11527 - locals.var_q_d1_ln);
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9, ) = (assign12200_e11529, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9), );

        let assign12210_e11532: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign12210_e11534: f64 = (assign12210_e11532 - locals.var_q_d2_ln);
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9, ) = (assign12210_e11534, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9), );

        let assign12220_e11538: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign12220_e11539: f64 = (locals.var_q_k1q1 + assign12220_e11538);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign12220_e11539, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))), );

        let assign12230_e11543: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign12230_e11544: f64 = (locals.var_k1 + assign12230_e11543);
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9, ) = (assign12230_e11544, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))), );

        let assign12240_e11547: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9, ) = (assign12240_e11547, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)), );

        let assign12250_e11550: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign12250_e11552: f64 = (assign12250_e11550 - locals.var_q_aexp);
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign12250_e11552, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9), );

        let assign12260_e11555: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign12260_e11558: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign12260_e11559: f64 = (assign12260_e11555 + assign12260_e11558);
        let assign12260_e11561: f64 = (assign12260_e11559 + locals.var_q_aexp);
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign12260_e11561, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9), );

        let assign12270_e11564: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign12270_e11567: f64 = (2.0 * locals.var_q_d1_qi);
        let assign12270_e11569: f64 = (assign12270_e11567 * locals.var_q_d1_expnum);
        let assign12270_e11570: f64 = (assign12270_e11564 + assign12270_e11569);
        let assign12270_e11573: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign12270_e11574: f64 = (assign12270_e11570 + assign12270_e11573);
        let assign12270_e11576: f64 = (assign12270_e11574 - locals.var_q_aexp);
        (locals.var_q_d2_zero, locals.var_q_d2_zero_dn4, locals.var_q_d2_zero_dn6, locals.var_q_d2_zero_dn7, locals.var_q_d2_zero_dn8, locals.var_q_d2_zero_dn9, ) = (assign12270_e11576, (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4), (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6), (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7), (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8), (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign12270_e11567 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9), );

        let assign12280_e11579: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign12280_e11582: f64 = (0.5 * locals.var_q_zero);
        let assign12280_e11584: f64 = (assign12280_e11582 * locals.var_q_d2_zero);
        let assign12280_e11585: f64 = (assign12280_e11579 - assign12280_e11584);
        (locals.var_q_temp, locals.var_q_temp_dn4, locals.var_q_temp_dn6, locals.var_q_temp_dn7, locals.var_q_temp_dn8, locals.var_q_temp_dn9, ) = (assign12280_e11585, (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn4))), (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn6))), (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn7))), (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn8))), (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign12280_e11582 * locals.var_q_d2_zero_dn9))), );

        let assign12290_e11587: f64 = (-locals.var_q_zero);
        let assign12290_e11589: f64 = (assign12290_e11587 * locals.var_q_d1_zero);
        let assign12290_e11591: f64 = (assign12290_e11589 * locals.var_q_temp);
        let assign12290_e11594: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign12290_e11596: f64 = (assign12290_e11594 + 1e-200);
        let assign12290_e11597: f64 = (assign12290_e11591 / assign12290_e11596);
        (locals.var_q_eps2, locals.var_q_eps2_dn4, locals.var_q_eps2_dn6, locals.var_q_eps2_dn7, locals.var_q_eps2_dn8, locals.var_q_eps2_dn9, ) = (assign12290_e11597, ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn4)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign12290_e11596 * assign12290_e11596)), ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn6)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign12290_e11596 * assign12290_e11596)), ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn7)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign12290_e11596 * assign12290_e11596)), ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn8)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign12290_e11596 * assign12290_e11596)), ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign12290_e11587 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign12290_e11589 * locals.var_q_temp_dn9)) * assign12290_e11596) - (assign12290_e11591 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign12290_e11596 * assign12290_e11596)), );

        let assign12300_e11600: f64 = (locals.var_q1s + locals.var_q_eps2);
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign12300_e11600, (locals.var_q1s_dn4 + locals.var_q_eps2_dn4), (locals.var_q1s_dn6 + locals.var_q_eps2_dn6), (locals.var_q1s_dn7 + locals.var_q_eps2_dn7), (locals.var_q1s_dn8 + locals.var_q_eps2_dn8), (locals.var_q1s_dn9 + locals.var_q_eps2_dn9), );

        let assign12310_e11603: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign12310_e11603, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign12320_e11606: f64 = (locals.var_k2 * locals.var_q2s);
        (locals.var_q_k2q2, locals.var_q_k2q2_dn4, locals.var_q_k2q2_dn6, locals.var_q_k2q2_dn7, locals.var_q_k2q2_dn8, locals.var_q_k2q2_dn9, ) = (assign12320_e11606, ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4)), ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6)), ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7)), ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8)), ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9)), );

        let assign12330_e11609: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign12330_e11609, (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9), );

        let assign12340_e11613: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign12340_e11614: f64 = (1.0 + assign12340_e11613);
        (locals.var_q_a, locals.var_q_a_dn4, locals.var_q_a_dn6, locals.var_q_a_dn7, locals.var_q_a_dn8, locals.var_q_a_dn9, ) = (assign12340_e11614, (0.065345483024 * locals.var_q_qi_int_dn4), (0.065345483024 * locals.var_q_qi_int_dn6), (0.065345483024 * locals.var_q_qi_int_dn7), (0.065345483024 * locals.var_q_qi_int_dn8), (0.065345483024 * locals.var_q_qi_int_dn9), );

        let assign12350_e11618: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign12350_e11619: f64 = (39.478417604 + assign12350_e11618);
        let assign12350_e11622: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12350_e11623: f64 = (assign12350_e11619 + assign12350_e11622);
        (locals.var_q_b, locals.var_q_b_dn4, locals.var_q_b_dn6, locals.var_q_b_dn7, locals.var_q_b_dn8, locals.var_q_b_dn9, ) = (assign12350_e11623, ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))), ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))), ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))), ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))), ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))), );

        let assign12360_e11627: f64 = (2.0 * locals.var_q_qi_int);
        let assign12360_e11630: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12360_e11631: f64 = (assign12360_e11627 + assign12360_e11630);
        let assign12360_e11632: f64 = (39.478417604 * assign12360_e11631);
        (locals.var_q_c, locals.var_q_c_dn4, locals.var_q_c_dn6, locals.var_q_c_dn7, locals.var_q_c_dn8, locals.var_q_c_dn9, ) = (assign12360_e11632, (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)))), );

        let assign12370_e11635: f64 = (locals.var_q_b * locals.var_q_b);
        let assign12370_e11638: f64 = (4.0 * locals.var_q_a);
        let assign12370_e11640: f64 = (assign12370_e11638 * locals.var_q_c);
        let assign12370_e11641: f64 = (assign12370_e11635 - assign12370_e11640);
        let assign12370_e11642: f64 = (assign12370_e11641).sqrt();
        (locals.var_q_disc, locals.var_q_disc_dn4, locals.var_q_disc_dn6, locals.var_q_disc_dn7, locals.var_q_disc_dn8, locals.var_q_disc_dn9, ) = (assign12370_e11642, ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn4))) / (2.0 * assign12370_e11642)), ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn6))) / (2.0 * assign12370_e11642)), ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn7))) / (2.0 * assign12370_e11642)), ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn8))) / (2.0 * assign12370_e11642)), ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign12370_e11638 * locals.var_q_c_dn9))) / (2.0 * assign12370_e11642)), );

        let assign12380_e11645: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign12380_e11648: f64 = (2.0 * locals.var_q_a);
        let assign12380_e11649: f64 = (assign12380_e11645 / assign12380_e11648);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign12380_e11649, ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn4))) / (assign12380_e11648 * assign12380_e11648)), ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn6))) / (assign12380_e11648 * assign12380_e11648)), ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn7))) / (assign12380_e11648 * assign12380_e11648)), ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn8))) / (assign12380_e11648 * assign12380_e11648)), ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign12380_e11648) - (assign12380_e11645 * (2.0 * locals.var_q_a_dn9))) / (assign12380_e11648 * assign12380_e11648)), );

        let assign12390_e11652: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12390_e11654: f64 = (assign12390_e11652 - locals.var_q_qsq);
        (locals.var_q_delta, locals.var_q_delta_dn4, locals.var_q_delta_dn6, locals.var_q_delta_dn7, locals.var_q_delta_dn8, locals.var_q_delta_dn9, ) = (assign12390_e11654, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9), );

        let assign12400_e11657: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard559 = assign12400_e11657;

        if (locals.var_guard559 != 0.0) {
            let assign12410_e11662: f64 = (locals.var_q_delta / locals.var_a0);
            let assign12410_e11663: f64 = (assign12410_e11662).ln();
            let assign12410_e11665: f64 = assign12410_e11663;
            let assign12410_e11667: f64 = (assign12410_e11665 - locals.var_xg1x);
            let assign12410_e11669: f64 = (assign12410_e11667 + locals.var_q1s);
            let assign12410_e11670: f64 = (locals.var_q_delta * assign12410_e11669);
            (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign12410_e11670, ((locals.var_q_delta_dn4 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn4) + locals.var_q1s_dn4))), ((locals.var_q_delta_dn6 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn6) + locals.var_q1s_dn6))), ((locals.var_q_delta_dn7 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn7) + locals.var_q1s_dn7))), ((locals.var_q_delta_dn8 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn8) + locals.var_q1s_dn8))), ((locals.var_q_delta_dn9 * assign12410_e11669) + (locals.var_q_delta * ((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign12410_e11662) - locals.var_xg1x_dn9) + locals.var_q1s_dn9))), );
        }

        if (locals.var_guard559 != 0.0) {
            let assign12420_e11676: f64 = (2.0 * locals.var_k1);
            let assign12420_e11678: f64 = (assign12420_e11676 * locals.var_q_k1q1);
            let assign12420_e11680: f64 = (assign12420_e11678 + locals.var_q_delta);
            (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign12420_e11680, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12420_e11676 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9), );
        }

        if (locals.var_guard559 != 0.0) {
            let assign12430_e11686: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12430_e11688: f64 = (assign12430_e11686 - locals.var_q_x1sat);
            locals.var_q_dx1 = assign12430_e11688;
        }

        let assign12440_e11700: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign12440_e11702: f64 = (locals.var_k1).ln();
        let assign12440_e11703: f64 = (assign12440_e11700 + assign12440_e11702);
        let assign12440_e11710: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign12440_e11703 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard560 = assign12440_e11710;

        if ((locals.var_guard559 != 0.0) && (locals.var_guard560 != 0.0)) {
            let assign12450_e11717: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
            let assign12450_e11718: f64 = (locals.var_q1s - assign12450_e11717);
            (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign12450_e11718, (locals.var_q1s_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), );
        }

        let assign12460_e11723: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign12460_e11723, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign12470_e11726: f64 = (locals.var_k2 * locals.var_q2s);
        (locals.var_q_k2q2, locals.var_q_k2q2_dn4, locals.var_q_k2q2_dn6, locals.var_q_k2q2_dn7, locals.var_q_k2q2_dn8, locals.var_q_k2q2_dn9, ) = (assign12470_e11726, ((locals.var_k2_dn4 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn4)), ((locals.var_k2_dn6 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn6)), ((locals.var_k2_dn7 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn7)), ((locals.var_k2_dn8 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn8)), ((locals.var_k2_dn9 * locals.var_q2s) + (locals.var_k2 * locals.var_q2s_dn9)), );

        let assign12480_e11729: f64 = (locals.var_q_k1q1 + locals.var_q_k2q2);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign12480_e11729, (locals.var_q_k1q1_dn4 + locals.var_q_k2q2_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_k2q2_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_k2q2_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_k2q2_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_k2q2_dn9), );

        let assign12490_e11733: f64 = (0.065345483024 * locals.var_q_qi_int);
        let assign12490_e11734: f64 = (1.0 + assign12490_e11733);
        (locals.var_q_a, locals.var_q_a_dn4, locals.var_q_a_dn6, locals.var_q_a_dn7, locals.var_q_a_dn8, locals.var_q_a_dn9, ) = (assign12490_e11734, (0.065345483024 * locals.var_q_qi_int_dn4), (0.065345483024 * locals.var_q_qi_int_dn6), (0.065345483024 * locals.var_q_qi_int_dn7), (0.065345483024 * locals.var_q_qi_int_dn8), (0.065345483024 * locals.var_q_qi_int_dn9), );

        let assign12500_e11738: f64 = (8.5797362674 * locals.var_q_qi_int);
        let assign12500_e11739: f64 = (39.478417604 + assign12500_e11738);
        let assign12500_e11742: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12500_e11743: f64 = (assign12500_e11739 + assign12500_e11742);
        (locals.var_q_b, locals.var_q_b_dn4, locals.var_q_b_dn6, locals.var_q_b_dn7, locals.var_q_b_dn8, locals.var_q_b_dn9, ) = (assign12500_e11743, ((8.5797362674 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))), ((8.5797362674 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))), ((8.5797362674 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))), ((8.5797362674 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))), ((8.5797362674 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))), );

        let assign12510_e11747: f64 = (2.0 * locals.var_q_qi_int);
        let assign12510_e11750: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12510_e11751: f64 = (assign12510_e11747 + assign12510_e11750);
        let assign12510_e11752: f64 = (39.478417604 * assign12510_e11751);
        (locals.var_q_c, locals.var_q_c_dn4, locals.var_q_c_dn6, locals.var_q_c_dn7, locals.var_q_c_dn8, locals.var_q_c_dn9, ) = (assign12510_e11752, (39.478417604 * ((2.0 * locals.var_q_qi_int_dn4) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn6) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn7) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn8) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8)))), (39.478417604 * ((2.0 * locals.var_q_qi_int_dn9) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9)))), );

        let assign12520_e11755: f64 = (locals.var_q_b * locals.var_q_b);
        let assign12520_e11758: f64 = (4.0 * locals.var_q_a);
        let assign12520_e11760: f64 = (assign12520_e11758 * locals.var_q_c);
        let assign12520_e11761: f64 = (assign12520_e11755 - assign12520_e11760);
        let assign12520_e11762: f64 = (assign12520_e11761).sqrt();
        (locals.var_q_disc, locals.var_q_disc_dn4, locals.var_q_disc_dn6, locals.var_q_disc_dn7, locals.var_q_disc_dn8, locals.var_q_disc_dn9, ) = (assign12520_e11762, ((((locals.var_q_b_dn4 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn4)) - (((4.0 * locals.var_q_a_dn4) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn4))) / (2.0 * assign12520_e11762)), ((((locals.var_q_b_dn6 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn6)) - (((4.0 * locals.var_q_a_dn6) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn6))) / (2.0 * assign12520_e11762)), ((((locals.var_q_b_dn7 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn7)) - (((4.0 * locals.var_q_a_dn7) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn7))) / (2.0 * assign12520_e11762)), ((((locals.var_q_b_dn8 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn8)) - (((4.0 * locals.var_q_a_dn8) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn8))) / (2.0 * assign12520_e11762)), ((((locals.var_q_b_dn9 * locals.var_q_b) + (locals.var_q_b * locals.var_q_b_dn9)) - (((4.0 * locals.var_q_a_dn9) * locals.var_q_c) + (assign12520_e11758 * locals.var_q_c_dn9))) / (2.0 * assign12520_e11762)), );

        let assign12530_e11765: f64 = (locals.var_q_disc - locals.var_q_b);
        let assign12530_e11768: f64 = (2.0 * locals.var_q_a);
        let assign12530_e11769: f64 = (assign12530_e11765 / assign12530_e11768);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign12530_e11769, ((((locals.var_q_disc_dn4 - locals.var_q_b_dn4) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn4))) / (assign12530_e11768 * assign12530_e11768)), ((((locals.var_q_disc_dn6 - locals.var_q_b_dn6) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn6))) / (assign12530_e11768 * assign12530_e11768)), ((((locals.var_q_disc_dn7 - locals.var_q_b_dn7) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn7))) / (assign12530_e11768 * assign12530_e11768)), ((((locals.var_q_disc_dn8 - locals.var_q_b_dn8) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn8))) / (assign12530_e11768 * assign12530_e11768)), ((((locals.var_q_disc_dn9 - locals.var_q_b_dn9) * assign12530_e11768) - (assign12530_e11765 * (2.0 * locals.var_q_a_dn9))) / (assign12530_e11768 * assign12530_e11768)), );

        let assign12540_e11772: f64 = (-0.005);
        let assign12540_e11773: f64 = if locals.var_q_qsq < assign12540_e11772 { 1.0 } else { 0.0 };
        locals.var_guard561 = assign12540_e11773;

        if (locals.var_guard561 != 0.0) {
            let assign12550_e11776: f64 = (locals.var_q_qsq).abs();
            let assign12550_e11777: f64 = (assign12550_e11776).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign12550_e11777, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12550_e11777)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12550_e11777)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12550_e11777)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12550_e11777)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12550_e11777)), );
        }

        if (locals.var_guard561 != 0.0) {
            let assign12560_e11784: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign12560_e11785: f64 = (assign12560_e11784).tan();
            let assign12560_e11786: f64 = (locals.var_q_rac_qsq / assign12560_e11785);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12560_e11786, (((locals.var_q_rac_qsq_dn4 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), (((locals.var_q_rac_qsq_dn6 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), (((locals.var_q_rac_qsq_dn7 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), (((locals.var_q_rac_qsq_dn8 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), (((locals.var_q_rac_qsq_dn9 * assign12560_e11785) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign12560_e11784).cos() * (assign12560_e11784).cos())))) / (assign12560_e11785 * assign12560_e11785)), );
        }

        if (locals.var_guard561 != 0.0) {
            let assign12570_e11795: f64 = (2.0 - locals.var_q_qcoth);
            let assign12570_e11796: f64 = (locals.var_q_qcoth * assign12570_e11795);
            let assign12570_e11797: f64 = (locals.var_q_qsq + assign12570_e11796);
            let assign12570_e11798: f64 = (0.25 * assign12570_e11797);
            let assign12570_e11800: f64 = (assign12570_e11798 / locals.var_q_qsq);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12570_e11800, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12570_e11795) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign12570_e11798 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        let assign12580_e11805: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard562 = assign12580_e11805;

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
            let assign12590_e11811: f64 = (locals.var_q_qsq).abs();
            let assign12590_e11812: f64 = (assign12590_e11811).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign12590_e11812, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12590_e11812)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12590_e11812)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12590_e11812)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12590_e11812)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12590_e11812)), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
            let assign12600_e11820: f64 = (-locals.var_q_rac_qsq);
            let assign12600_e11821: f64 = (assign12600_e11820).exp();
            (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (assign12600_e11821, (assign12600_e11821 * (-locals.var_q_rac_qsq_dn4)), (assign12600_e11821 * (-locals.var_q_rac_qsq_dn6)), (assign12600_e11821 * (-locals.var_q_rac_qsq_dn7)), (assign12600_e11821 * (-locals.var_q_rac_qsq_dn8)), (assign12600_e11821 * (-locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
            let assign12610_e11831: f64 = (1.0 + locals.var_q_invexpq);
            let assign12610_e11832: f64 = (locals.var_q_rac_qsq * assign12610_e11831);
            let assign12610_e11835: f64 = (1.0 - locals.var_q_invexpq);
            let assign12610_e11836: f64 = (assign12610_e11832 / assign12610_e11835);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12610_e11836, (((((locals.var_q_rac_qsq_dn4 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn4))) / (assign12610_e11835 * assign12610_e11835)), (((((locals.var_q_rac_qsq_dn6 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn6))) / (assign12610_e11835 * assign12610_e11835)), (((((locals.var_q_rac_qsq_dn7 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn7))) / (assign12610_e11835 * assign12610_e11835)), (((((locals.var_q_rac_qsq_dn8 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn8))) / (assign12610_e11835 * assign12610_e11835)), (((((locals.var_q_rac_qsq_dn9 * assign12610_e11831) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign12610_e11835) - (assign12610_e11832 * (-locals.var_q_invexpq_dn9))) / (assign12610_e11835 * assign12610_e11835)), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 != 0.0)) {
            let assign12620_e11848: f64 = (2.0 - locals.var_q_qcoth);
            let assign12620_e11849: f64 = (locals.var_q_qcoth * assign12620_e11848);
            let assign12620_e11850: f64 = (locals.var_q_qsq + assign12620_e11849);
            let assign12620_e11851: f64 = (0.25 * assign12620_e11850);
            let assign12620_e11853: f64 = (assign12620_e11851 / locals.var_q_qsq);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12620_e11853, ((((0.25 * (locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * (locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12620_e11848) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9))))) * locals.var_q_qsq) - (assign12620_e11851 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 == 0.0)) {
            let assign12630_e11864: f64 = (locals.var_q_qsq * 0.1666666666667);
            let assign12630_e11868: f64 = (locals.var_q_qsq * 0.0166666666667);
            let assign12630_e11872: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign12630_e11873: f64 = (1.0 - assign12630_e11872);
            let assign12630_e11874: f64 = (assign12630_e11868 * assign12630_e11873);
            let assign12630_e11875: f64 = (1.0 - assign12630_e11874);
            let assign12630_e11876: f64 = (assign12630_e11864 * assign12630_e11875);
            let assign12630_e11877: f64 = (2.0 + assign12630_e11876);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12630_e11877, (((locals.var_q_qsq_dn4 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn4 * 0.0238095238095))))))), (((locals.var_q_qsq_dn6 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn6 * 0.0238095238095))))))), (((locals.var_q_qsq_dn7 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn7 * 0.0238095238095))))))), (((locals.var_q_qsq_dn8 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn8 * 0.0238095238095))))))), (((locals.var_q_qsq_dn9 * 0.1666666666667) * assign12630_e11875) + (assign12630_e11864 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign12630_e11873) + (assign12630_e11868 * (-(locals.var_q_qsq_dn9 * 0.0238095238095))))))), );
        }

        if ((locals.var_guard561 == 0.0) && (locals.var_guard562 == 0.0)) {
            let assign12640_e11889: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign12640_e11893: f64 = (locals.var_q_qsq * 0.0357142857143);
            let assign12640_e11897: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign12640_e11898: f64 = (1.0 - assign12640_e11897);
            let assign12640_e11899: f64 = (assign12640_e11893 * assign12640_e11898);
            let assign12640_e11900: f64 = (1.0 - assign12640_e11899);
            let assign12640_e11901: f64 = (assign12640_e11889 * assign12640_e11900);
            let assign12640_e11902: f64 = (1.0 - assign12640_e11901);
            let assign12640_e11903: f64 = (0.1666666666667 * assign12640_e11902);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12640_e11903, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign12640_e11900) + (assign12640_e11889 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign12640_e11898) + (assign12640_e11893 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))), );
        }

        let assign12650_e11909: f64 = (locals.var_q_qi_int * locals.var_q_qcoth);
        let assign12650_e11912: f64 = (locals.var_q_k1q1 * locals.var_q_k2q2);
        let assign12650_e11913: f64 = (assign12650_e11909 + assign12650_e11912);
        let assign12650_e11915: f64 = (assign12650_e11913 + locals.var_q_qsq);
        let assign12650_e11918: f64 = (locals.var_q_qi_int * locals.var_q_d1_qcoth);
        let assign12650_e11920: f64 = (assign12650_e11918 + 1.0);
        let assign12650_e11921: f64 = (assign12650_e11915 / assign12650_e11920);
        let assign12650_e11922: f64 = (locals.var_q_qsq - assign12650_e11921);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign12650_e11922, (locals.var_q_qsq_dn4 - (((((((locals.var_q_qi_int_dn4 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn4)) + ((locals.var_q_k1q1_dn4 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn4))) + locals.var_q_qsq_dn4) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn4 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn4)))) / (assign12650_e11920 * assign12650_e11920))), (locals.var_q_qsq_dn6 - (((((((locals.var_q_qi_int_dn6 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn6)) + ((locals.var_q_k1q1_dn6 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn6))) + locals.var_q_qsq_dn6) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn6 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn6)))) / (assign12650_e11920 * assign12650_e11920))), (locals.var_q_qsq_dn7 - (((((((locals.var_q_qi_int_dn7 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn7)) + ((locals.var_q_k1q1_dn7 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn7))) + locals.var_q_qsq_dn7) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn7 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn7)))) / (assign12650_e11920 * assign12650_e11920))), (locals.var_q_qsq_dn8 - (((((((locals.var_q_qi_int_dn8 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn8)) + ((locals.var_q_k1q1_dn8 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn8))) + locals.var_q_qsq_dn8) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn8 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn8)))) / (assign12650_e11920 * assign12650_e11920))), (locals.var_q_qsq_dn9 - (((((((locals.var_q_qi_int_dn9 * locals.var_q_qcoth) + (locals.var_q_qi_int * locals.var_q_qcoth_dn9)) + ((locals.var_q_k1q1_dn9 * locals.var_q_k2q2) + (locals.var_q_k1q1 * locals.var_q_k2q2_dn9))) + locals.var_q_qsq_dn9) * assign12650_e11920) - (assign12650_e11915 * ((locals.var_q_qi_int_dn9 * locals.var_q_d1_qcoth) + (locals.var_q_qi_int * locals.var_q_d1_qcoth_dn9)))) / (assign12650_e11920 * assign12650_e11920))), );

        let assign12660_e11925: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12660_e11927: f64 = (assign12660_e11925 - locals.var_q_qsq);
        (locals.var_q_delta, locals.var_q_delta_dn4, locals.var_q_delta_dn6, locals.var_q_delta_dn7, locals.var_q_delta_dn8, locals.var_q_delta_dn9, ) = (assign12660_e11927, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_qsq_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_qsq_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_qsq_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_qsq_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_qsq_dn9), );

        let assign12670_e11930: f64 = if locals.var_q_delta > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard563 = assign12670_e11930;

    }

    pub(super) fn stamp_transient_block_14(
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard563 != 0.0) {
            let assign12680_e11935: f64 = (locals.var_q_delta / locals.var_a0);
            let assign12680_e11936: f64 = (assign12680_e11935).ln();
            let assign12680_e11938: f64 = assign12680_e11936;
            let assign12680_e11940: f64 = (assign12680_e11938 - locals.var_xg1x);
            let assign12680_e11942: f64 = (assign12680_e11940 + locals.var_q1s);
            let assign12680_e11943: f64 = (locals.var_q_delta * assign12680_e11942);
            (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign12680_e11943, ((locals.var_q_delta_dn4 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn4 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn4)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn4) + locals.var_q1s_dn4))), ((locals.var_q_delta_dn6 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn6 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn6)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn6) + locals.var_q1s_dn6))), ((locals.var_q_delta_dn7 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn7 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn7)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn7) + locals.var_q1s_dn7))), ((locals.var_q_delta_dn8 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn8 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn8)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn8) + locals.var_q1s_dn8))), ((locals.var_q_delta_dn9 * assign12680_e11942) + (locals.var_q_delta * ((((((locals.var_q_delta_dn9 * locals.var_a0) - (locals.var_q_delta * locals.var_a0_dn9)) / (locals.var_a0 * locals.var_a0)) / assign12680_e11935) - locals.var_xg1x_dn9) + locals.var_q1s_dn9))), );
        }

        if (locals.var_guard563 != 0.0) {
            let assign12690_e11949: f64 = (2.0 * locals.var_k1);
            let assign12690_e11951: f64 = (assign12690_e11949 * locals.var_q_k1q1);
            let assign12690_e11953: f64 = (assign12690_e11951 + locals.var_q_delta);
            (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign12690_e11953, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn4)) + locals.var_q_delta_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn6)) + locals.var_q_delta_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn7)) + locals.var_q_delta_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn8)) + locals.var_q_delta_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12690_e11949 * locals.var_q_k1q1_dn9)) + locals.var_q_delta_dn9), );
        }

        if (locals.var_guard563 != 0.0) {
            let assign12700_e11959: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12700_e11961: f64 = (assign12700_e11959 - locals.var_q_x1sat);
            locals.var_q_dx1 = assign12700_e11961;
        }

        let assign12710_e11973: f64 = (locals.var_q_dx1 + 2.3025850929941);
        let assign12710_e11975: f64 = (locals.var_k1).ln();
        let assign12710_e11976: f64 = (assign12710_e11973 + assign12710_e11975);
        let assign12710_e11983: f64 = if ((((locals.var_q_zero < 0.0) && (locals.var_q_d1_zero > 0.0)) && (assign12710_e11976 > 0.0)) || (locals.var_q_dx1 > 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard564 = assign12710_e11983;

        if ((locals.var_guard563 != 0.0) && (locals.var_guard564 != 0.0)) {
            let assign12720_e11990: f64 = (locals.var_q_zero / locals.var_q_d1_zero);
            let assign12720_e11991: f64 = (locals.var_q1s - assign12720_e11990);
            (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign12720_e11991, (locals.var_q1s_dn4 - (((locals.var_q_zero_dn4 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn4)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn6 - (((locals.var_q_zero_dn6 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn6)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn7 - (((locals.var_q_zero_dn7 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn7)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn8 - (((locals.var_q_zero_dn8 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn8)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), (locals.var_q1s_dn9 - (((locals.var_q_zero_dn9 * locals.var_q_d1_zero) - (locals.var_q_zero * locals.var_q_d1_zero_dn9)) / (locals.var_q_d1_zero * locals.var_q_d1_zero))), );
        }

        let assign12730_e11996: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign12730_e11996, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign12740_e11999: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign12740_e12001: f64 = assign12740_e11999;
        let assign12740_e12003: f64 = if assign12740_e12001 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard565 = assign12740_e12003;

        if (locals.var_guard565 != 0.0) {
            let assign12750_e12007: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12750_e12009: f64 = assign12750_e12007;
            let assign12750_e12010: f64 = (assign12750_e12009).exp();
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12750_e12010, (assign12750_e12010 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign12750_e12010 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign12750_e12010 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign12750_e12010 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign12750_e12010 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)), );
        }

        if (locals.var_guard565 == 0.0) {
            let assign12760_e12019: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12760_e12021: f64 = assign12760_e12019;
            let assign12760_e12023: f64 = (assign12760_e12021 - 80.0);
            let assign12760_e12028: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12760_e12030: f64 = assign12760_e12028;
            let assign12760_e12032: f64 = (assign12760_e12030 - 80.0);
            let assign12760_e12033: f64 = (0.5 * assign12760_e12032);
            let assign12760_e12037: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign12760_e12039: f64 = assign12760_e12037;
            let assign12760_e12041: f64 = (assign12760_e12039 - 80.0);
            let assign12760_e12043: f64 = (assign12760_e12041 * 0.3333333333333);
            let assign12760_e12044: f64 = (1.0 + assign12760_e12043);
            let assign12760_e12045: f64 = (assign12760_e12033 * assign12760_e12044);
            let assign12760_e12046: f64 = (1.0 + assign12760_e12045);
            let assign12760_e12047: f64 = (assign12760_e12023 * assign12760_e12046);
            let assign12760_e12048: f64 = (1.0 + assign12760_e12047);
            let assign12760_e12049: f64 = (5.54062e34 * assign12760_e12048);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12760_e12049, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign12760_e12046) + (assign12760_e12023 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign12760_e12044) + (assign12760_e12033 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))), );
        }

        let assign12770_e12054: f64 = (locals.var_a0 * locals.var_q_temp1);
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9, ) = (assign12770_e12054, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)), );

        let assign12780_e12057: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign12780_e12059: f64 = (assign12780_e12057 - locals.var_q_aexp);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign12780_e12059, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9), );

        let assign12790_e12062: f64 = (2.0 * locals.var_k1);
        let assign12790_e12064: f64 = (assign12790_e12062 * locals.var_q_k1q1);
        let assign12790_e12066: f64 = (assign12790_e12064 + locals.var_q_aexp);
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9, ) = (assign12790_e12066, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign12790_e12062 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9), );

        let assign12800_e12069: f64 = (2.0 * locals.var_k1);
        let assign12800_e12071: f64 = (assign12800_e12069 * locals.var_k1);
        let assign12800_e12073: f64 = (assign12800_e12071 - locals.var_q_aexp);
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9, ) = (assign12800_e12073, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign12800_e12069 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9), );

        let assign12810_e12076: f64 = (-0.005);
        let assign12810_e12077: f64 = if locals.var_q_qsq < assign12810_e12076 { 1.0 } else { 0.0 };
        locals.var_guard566 = assign12810_e12077;

        if (locals.var_guard566 != 0.0) {
            let assign12820_e12080: f64 = (locals.var_q_qsq).abs();
            let assign12820_e12081: f64 = (assign12820_e12080).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign12820_e12081, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12820_e12081)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12820_e12081)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12820_e12081)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12820_e12081)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12820_e12081)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12830_e12088: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign12830_e12089: f64 = (assign12830_e12088).tan();
            let assign12830_e12090: f64 = (locals.var_q_rac_qsq / assign12830_e12089);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12830_e12090, (((locals.var_q_rac_qsq_dn4 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), (((locals.var_q_rac_qsq_dn6 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), (((locals.var_q_rac_qsq_dn7 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), (((locals.var_q_rac_qsq_dn8 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), (((locals.var_q_rac_qsq_dn9 * assign12830_e12089) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign12830_e12088).cos() * (assign12830_e12088).cos())))) / (assign12830_e12089 * assign12830_e12089)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12840_e12096: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign12840_e12098: f64 = (assign12840_e12096 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12840_e12098, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign12840_e12096 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12850_e12106: f64 = (2.0 - locals.var_q_qcoth);
            let assign12850_e12107: f64 = (locals.var_q_qcoth * assign12850_e12106);
            let assign12850_e12108: f64 = (locals.var_q_qsq + assign12850_e12107);
            let assign12850_e12110: f64 = (assign12850_e12108 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12850_e12110, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12850_e12106) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign12850_e12108 * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12860_e12117: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign12860_e12120: f64 = (1.0 + locals.var_q_qcoth);
            let assign12860_e12121: f64 = (assign12860_e12117 * assign12860_e12120);
            let assign12860_e12122: f64 = (locals.var_q_d1_qsq - assign12860_e12121);
            let assign12860_e12124: f64 = (assign12860_e12122 * locals.var_q_temp1);
            let assign12860_e12127: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign12860_e12129: f64 = (assign12860_e12127 / locals.var_q_d1_qsq);
            let assign12860_e12130: f64 = (assign12860_e12124 + assign12860_e12129);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign12860_e12130, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign12860_e12120) + (assign12860_e12117 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign12860_e12122 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign12860_e12127 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12870_e12137: f64 = (0.5 * locals.var_q_qcoth);
            let assign12870_e12138: f64 = (1.0 - assign12870_e12137);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign12870_e12138, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12880_e12144: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign12880_e12146: f64 = (assign12880_e12144 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign12880_e12146, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12880_e12144 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard566 != 0.0) {
            let assign12890_e12152: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign12890_e12157: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign12890_e12158: f64 = (locals.var_q_d1_ln + assign12890_e12157);
            let assign12890_e12159: f64 = (locals.var_q_d1_qsq * assign12890_e12158);
            let assign12890_e12160: f64 = (assign12890_e12152 - assign12890_e12159);
            let assign12890_e12162: f64 = (assign12890_e12160 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign12890_e12162, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign12890_e12158) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign12890_e12160 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        let assign12900_e12167: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard567 = assign12900_e12167;

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12910_e12173: f64 = (locals.var_q_qsq).abs();
            let assign12910_e12174: f64 = (assign12910_e12173).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign12910_e12174, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign12910_e12174)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign12910_e12174)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign12910_e12174)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign12910_e12174)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign12910_e12174)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12920_e12182: f64 = (-locals.var_q_rac_qsq);
            let assign12920_e12183: f64 = (assign12920_e12182).exp();
            (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (assign12920_e12183, (assign12920_e12183 * (-locals.var_q_rac_qsq_dn4)), (assign12920_e12183 * (-locals.var_q_rac_qsq_dn6)), (assign12920_e12183 * (-locals.var_q_rac_qsq_dn7)), (assign12920_e12183 * (-locals.var_q_rac_qsq_dn8)), (assign12920_e12183 * (-locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12930_e12193: f64 = (1.0 + locals.var_q_invexpq);
            let assign12930_e12194: f64 = (locals.var_q_rac_qsq * assign12930_e12193);
            let assign12930_e12197: f64 = (1.0 - locals.var_q_invexpq);
            let assign12930_e12198: f64 = (assign12930_e12194 / assign12930_e12197);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign12930_e12198, (((((locals.var_q_rac_qsq_dn4 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn4))) / (assign12930_e12197 * assign12930_e12197)), (((((locals.var_q_rac_qsq_dn6 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn6))) / (assign12930_e12197 * assign12930_e12197)), (((((locals.var_q_rac_qsq_dn7 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn7))) / (assign12930_e12197 * assign12930_e12197)), (((((locals.var_q_rac_qsq_dn8 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn8))) / (assign12930_e12197 * assign12930_e12197)), (((((locals.var_q_rac_qsq_dn9 * assign12930_e12193) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign12930_e12197) - (assign12930_e12194 * (-locals.var_q_invexpq_dn9))) / (assign12930_e12197 * assign12930_e12197)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12940_e12207: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign12940_e12209: f64 = (assign12940_e12207 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign12940_e12209, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign12940_e12207 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12950_e12220: f64 = (2.0 - locals.var_q_qcoth);
            let assign12950_e12221: f64 = (locals.var_q_qcoth * assign12950_e12220);
            let assign12950_e12222: f64 = (locals.var_q_qsq + assign12950_e12221);
            let assign12950_e12224: f64 = (assign12950_e12222 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign12950_e12224, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign12950_e12220) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign12950_e12222 * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12960_e12234: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign12960_e12237: f64 = (1.0 + locals.var_q_qcoth);
            let assign12960_e12238: f64 = (assign12960_e12234 * assign12960_e12237);
            let assign12960_e12239: f64 = (locals.var_q_d1_qsq - assign12960_e12238);
            let assign12960_e12241: f64 = (assign12960_e12239 * locals.var_q_temp1);
            let assign12960_e12244: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign12960_e12246: f64 = (assign12960_e12244 / locals.var_q_d1_qsq);
            let assign12960_e12247: f64 = (assign12960_e12241 + assign12960_e12246);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign12960_e12247, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign12960_e12237) + (assign12960_e12234 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign12960_e12239 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign12960_e12244 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12970_e12257: f64 = (0.5 * locals.var_q_qcoth);
            let assign12970_e12258: f64 = (1.0 - assign12970_e12257);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign12970_e12258, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12980_e12267: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign12980_e12269: f64 = (assign12980_e12267 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign12980_e12269, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign12980_e12267 * locals.var_q_temp2_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 != 0.0)) {
            let assign12990_e12278: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign12990_e12283: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign12990_e12284: f64 = (locals.var_q_d1_ln + assign12990_e12283);
            let assign12990_e12285: f64 = (locals.var_q_d1_qsq * assign12990_e12284);
            let assign12990_e12286: f64 = (assign12990_e12278 - assign12990_e12285);
            let assign12990_e12288: f64 = (assign12990_e12286 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign12990_e12288, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign12990_e12284) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign12990_e12286 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13000_e12300: f64 = (locals.var_q_qsq * 0.0166666666667);
            let assign13000_e12304: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign13000_e12308: f64 = (locals.var_q_qsq * 0.025);
            let assign13000_e12309: f64 = (1.0 - assign13000_e12308);
            let assign13000_e12310: f64 = (assign13000_e12304 * assign13000_e12309);
            let assign13000_e12311: f64 = (1.0 - assign13000_e12310);
            let assign13000_e12312: f64 = (assign13000_e12300 * assign13000_e12311);
            let assign13000_e12313: f64 = (1.0 - assign13000_e12312);
            let assign13000_e12314: f64 = (0.1666666666667 * assign13000_e12313);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign13000_e12314, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign13000_e12311) + (assign13000_e12300 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13000_e12309) + (assign13000_e12304 * (-(locals.var_q_qsq_dn9 * 0.025))))))))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13010_e12325: f64 = (locals.var_q_qsq * locals.var_q_temp3);
            let assign13010_e12326: f64 = (2.0 + assign13010_e12325);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13010_e12326, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13020_e12338: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign13020_e12342: f64 = (locals.var_q_qsq * 0.0357142857143);
            let assign13020_e12346: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign13020_e12347: f64 = (1.0 - assign13020_e12346);
            let assign13020_e12348: f64 = (assign13020_e12342 * assign13020_e12347);
            let assign13020_e12349: f64 = (1.0 - assign13020_e12348);
            let assign13020_e12350: f64 = (assign13020_e12338 * assign13020_e12349);
            let assign13020_e12351: f64 = (1.0 - assign13020_e12350);
            let assign13020_e12352: f64 = (0.1666666666667 * assign13020_e12351);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13020_e12352, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign13020_e12349) + (assign13020_e12338 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign13020_e12347) + (assign13020_e12342 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13030_e12362: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13030_e12362, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13040_e12374: f64 = (locals.var_q_qsq * 0.0714285714286);
            let assign13040_e12378: f64 = (0.05 * locals.var_q_qsq);
            let assign13040_e12382: f64 = (0.0420875420875421 * locals.var_q_qsq);
            let assign13040_e12383: f64 = (1.0 - assign13040_e12382);
            let assign13040_e12384: f64 = (assign13040_e12378 * assign13040_e12383);
            let assign13040_e12385: f64 = (1.0 - assign13040_e12384);
            let assign13040_e12386: f64 = (assign13040_e12374 * assign13040_e12385);
            let assign13040_e12387: f64 = (1.0 - assign13040_e12386);
            let assign13040_e12388: f64 = (0.0055555555556 * assign13040_e12387);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13040_e12388, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign13040_e12385) + (assign13040_e12374 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13040_e12383) + (assign13040_e12378 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13050_e12398: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
            let assign13050_e12401: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
            let assign13050_e12403: f64 = (assign13050_e12401 * locals.var_q_temp2);
            let assign13050_e12404: f64 = (assign13050_e12398 - assign13050_e12403);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13050_e12404, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign13050_e12401 * locals.var_q_temp2_dn9))), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13060_e12413: f64 = (-0.5);
            let assign13060_e12415: f64 = (assign13060_e12413 * locals.var_q_d1_qsq);
            let assign13060_e12417: f64 = (assign13060_e12415 * locals.var_q_temp3);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13060_e12417, (((assign13060_e12413 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn4)), (((assign13060_e12413 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn6)), (((assign13060_e12413 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn7)), (((assign13060_e12413 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn8)), (((assign13060_e12413 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign13060_e12415 * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard566 == 0.0) && (locals.var_guard567 == 0.0)) {
            let assign13070_e12426: f64 = (-0.5);
            let assign13070_e12428: f64 = (assign13070_e12426 * locals.var_q_d2_qsq);
            let assign13070_e12430: f64 = (assign13070_e12428 * locals.var_q_temp3);
            let assign13070_e12433: f64 = (0.25 * 0.0055555555556);
            let assign13070_e12435: f64 = (assign13070_e12433 * locals.var_q_d1_qsq);
            let assign13070_e12437: f64 = (assign13070_e12435 * locals.var_q_d1_qsq);
            let assign13070_e12441: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign13070_e12445: f64 = (0.075 * locals.var_q_qsq);
            let assign13070_e12446: f64 = (2.0 - assign13070_e12445);
            let assign13070_e12447: f64 = (assign13070_e12441 * assign13070_e12446);
            let assign13070_e12448: f64 = (1.0 - assign13070_e12447);
            let assign13070_e12449: f64 = (assign13070_e12437 * assign13070_e12448);
            let assign13070_e12450: f64 = (assign13070_e12430 + assign13070_e12449);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13070_e12450, ((((assign13070_e12426 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn4)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn4)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign13070_e12426 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn6)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn6)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign13070_e12426 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn7)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn7)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign13070_e12426 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn8)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn8)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign13070_e12426 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign13070_e12428 * locals.var_q_temp3_dn9)) + (((((assign13070_e12433 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign13070_e12435 * locals.var_q_d1_qsq_dn9)) * assign13070_e12448) + (assign13070_e12437 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13070_e12446) + (assign13070_e12441 * (-(0.075 * locals.var_q_qsq_dn9)))))))), );
        }

        let assign13080_e12455: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard568 = assign13080_e12455;

        if (locals.var_guard568 != 0.0) {
            let assign13090_e12459: f64 = (4.0 * locals.var_q_qsq);
            let assign13090_e12464: f64 = (2.0 - locals.var_q_invexpq);
            let assign13090_e12465: f64 = (locals.var_q_invexpq * assign13090_e12464);
            let assign13090_e12466: f64 = (1.0 - assign13090_e12465);
            let assign13090_e12467: f64 = (assign13090_e12459 / assign13090_e12466);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13090_e12467, ((((4.0 * locals.var_q_qsq_dn4) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn4 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign13090_e12466 * assign13090_e12466)), ((((4.0 * locals.var_q_qsq_dn6) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn6 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign13090_e12466 * assign13090_e12466)), ((((4.0 * locals.var_q_qsq_dn7) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn7 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign13090_e12466 * assign13090_e12466)), ((((4.0 * locals.var_q_qsq_dn8) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn8 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign13090_e12466 * assign13090_e12466)), ((((4.0 * locals.var_q_qsq_dn9) * assign13090_e12466) - (assign13090_e12459 * (-((locals.var_q_invexpq_dn9 * assign13090_e12464) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign13090_e12466 * assign13090_e12466)), );
        }

        if (locals.var_guard568 != 0.0) {
            let assign13100_e12473: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13100_e12473, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)), );
        }

        if (locals.var_guard568 != 0.0) {
            let assign13110_e12478: f64 = (locals.var_q_temp2).ln();
            let assign13110_e12480: f64 = (assign13110_e12478 - locals.var_q_rac_qsq);
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13110_e12480, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9), );
        }

        let assign13120_e12485: f64 = (-0.005);
        let assign13120_e12486: f64 = if locals.var_q_qsq < assign13120_e12485 { 1.0 } else { 0.0 };
        locals.var_guard569 = assign13120_e12486;

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
            let assign13130_e12493: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign13130_e12494: f64 = (assign13130_e12493).sin();
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13130_e12494, ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign13130_e12493).cos() * (0.5 * locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
            let assign13140_e12502: f64 = (-locals.var_q_qsq);
            let assign13140_e12505: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
            let assign13140_e12506: f64 = (assign13140_e12502 / assign13140_e12505);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13140_e12506, ((((-locals.var_q_qsq_dn4) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign13140_e12505 * assign13140_e12505)), ((((-locals.var_q_qsq_dn6) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign13140_e12505 * assign13140_e12505)), ((((-locals.var_q_qsq_dn7) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign13140_e12505 * assign13140_e12505)), ((((-locals.var_q_qsq_dn8) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign13140_e12505 * assign13140_e12505)), ((((-locals.var_q_qsq_dn9) * assign13140_e12505) - (assign13140_e12502 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign13140_e12505 * assign13140_e12505)), );
        }

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 != 0.0)) {
            let assign13150_e12514: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13150_e12514, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 == 0.0)) {
            let assign13160_e12525: f64 = (locals.var_q_qsq * 0.3333333333333);
            let assign13160_e12529: f64 = (0.05 * locals.var_q_qsq);
            let assign13160_e12533: f64 = (0.0396825396825397 * locals.var_q_qsq);
            let assign13160_e12534: f64 = (1.0 - assign13160_e12533);
            let assign13160_e12535: f64 = (assign13160_e12529 * assign13160_e12534);
            let assign13160_e12536: f64 = (1.0 - assign13160_e12535);
            let assign13160_e12537: f64 = (assign13160_e12525 * assign13160_e12536);
            let assign13160_e12538: f64 = (4.0 - assign13160_e12537);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13160_e12538, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign13160_e12536) + (assign13160_e12525 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13160_e12534) + (assign13160_e12529 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))), );
        }

        if ((locals.var_guard568 == 0.0) && (locals.var_guard569 == 0.0)) {
            let assign13170_e12547: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13170_e12547, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        let assign13180_e12552: f64 = (1.01 * locals.var_q_k1q1);
        let assign13180_e12554: f64 = (assign13180_e12552 + locals.var_q_qcoth);
        let assign13180_e12556: f64 = if assign13180_e12554 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard570 = assign13180_e12556;

        if (locals.var_guard570 != 0.0) {
            let assign13190_e12560: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign13190_e12560, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9), );
        }

        if (locals.var_guard570 != 0.0) {
            let assign13200_e12566: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign13200_e12566, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9), );
        }

        if (locals.var_guard570 != 0.0) {
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13220_e12578: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
            let assign13220_e12579: f64 = (1.0 / assign13220_e12578);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13220_e12579, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign13220_e12578 * assign13220_e12578))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign13220_e12578 * assign13220_e12578))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign13220_e12578 * assign13220_e12578))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign13220_e12578 * assign13220_e12578))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign13220_e12578 * assign13220_e12578))), );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13230_e12586: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign13230_e12586, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9), );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13240_e12593: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
            let assign13240_e12595: f64 = (assign13240_e12593 * locals.var_q_temp2);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign13240_e12595, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign13240_e12593 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13250_e12602: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
            let assign13250_e12604: f64 = (assign13250_e12602 - locals.var_q_aexp);
            let assign13250_e12607: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
            let assign13250_e12608: f64 = (assign13250_e12604 - assign13250_e12607);
            let assign13250_e12610: f64 = (assign13250_e12608 * locals.var_q_temp2);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign13250_e12610, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign13250_e12608 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard570 == 0.0) {
            let assign13260_e12617: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
            let assign13260_e12620: f64 = (2.0 * locals.var_q_temp3);
            let assign13260_e12622: f64 = (assign13260_e12620 * locals.var_q_d1_expnum);
            let assign13260_e12623: f64 = (assign13260_e12617 + assign13260_e12622);
            let assign13260_e12625: f64 = (assign13260_e12623 + locals.var_q_aexp);
            let assign13260_e12629: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
            let assign13260_e12630: f64 = (locals.var_q_d2_ln + assign13260_e12629);
            let assign13260_e12632: f64 = (assign13260_e12630 * locals.var_q_sh_term);
            let assign13260_e12633: f64 = (assign13260_e12625 - assign13260_e12632);
            let assign13260_e12635: f64 = (assign13260_e12633 * locals.var_q_temp2);
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (assign13260_e12635, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign13260_e12620 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign13260_e12630 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign13260_e12633 * locals.var_q_temp2_dn9)), );
        }

        let assign13270_e12640: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard571 = assign13270_e12640;

        if (locals.var_guard571 != 0.0) {
            let assign13280_e12643: f64 = (locals.var_q_expnum).ln();
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign13280_e12643, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum), );
        }

        if (locals.var_guard571 != 0.0) {
            let assign13290_e12649: f64 = (1.0 / locals.var_q_expnum);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13290_e12649, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))), );
        }

        if (locals.var_guard571 != 0.0) {
            let assign13300_e12655: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign13300_e12655, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard571 != 0.0) {
            let assign13310_e12661: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
            let assign13310_e12664: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
            let assign13310_e12665: f64 = (assign13310_e12661 - assign13310_e12664);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign13310_e12665, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))), );
        }

        if (locals.var_guard571 == 0.0) {
            let assign13320_e12672: f64 = (locals.var_q_k1q1 + 0.6931471805599);
            let assign13320_e12674: f64 = (-locals.var_q_k1q1);
            let assign13320_e12675: f64 = (assign13320_e12674).ln();
            let assign13320_e12676: f64 = (assign13320_e12672 + assign13320_e12675);
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign13320_e12676, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign13320_e12674)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign13320_e12674)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign13320_e12674)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign13320_e12674)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign13320_e12674)), );
        }

        if (locals.var_guard571 == 0.0) {
            let assign13330_e12683: f64 = (1.0 / locals.var_q1s);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13330_e12683, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))), );
        }

        if (locals.var_guard571 == 0.0) {
            let assign13340_e12690: f64 = (locals.var_k1 + locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign13340_e12690, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9), );
        }

        if (locals.var_guard571 == 0.0) {
            let assign13350_e12696: f64 = (-locals.var_q_temp1);
            let assign13350_e12698: f64 = (assign13350_e12696 * locals.var_q_temp1);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign13350_e12698, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign13350_e12696 * locals.var_q_temp1_dn9)), );
        }

        let assign13360_e12703: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign13360_e12705: f64 = (assign13360_e12703 + locals.var_q1s);
        let assign13360_e12708: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign13360_e12709: f64 = (assign13360_e12705 + assign13360_e12708);
        let assign13360_e12711: f64 = (assign13360_e12709 - locals.var_q_ln_term);
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9, ) = (assign13360_e12711, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9), );

        let assign13370_e12715: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign13370_e12716: f64 = (1.0 + assign13370_e12715);
        let assign13370_e12718: f64 = (assign13370_e12716 - locals.var_q_d1_ln);
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9, ) = (assign13370_e12718, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9), );

        let assign13380_e12721: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign13380_e12723: f64 = (assign13380_e12721 - locals.var_q_d2_ln);
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9, ) = (assign13380_e12723, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9), );

        let assign13390_e12727: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign13390_e12728: f64 = (locals.var_q_k1q1 + assign13390_e12727);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign13390_e12728, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))), );

        let assign13400_e12732: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign13400_e12733: f64 = (locals.var_k1 + assign13400_e12732);
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9, ) = (assign13400_e12733, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))), );

        let assign13410_e12736: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9, ) = (assign13410_e12736, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)), );

        let assign13420_e12739: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign13420_e12741: f64 = (assign13420_e12739 - locals.var_q_aexp);
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign13420_e12741, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9), );

    }

    pub(super) fn stamp_transient_block_15(
        locals: &mut StampLocals,
    ) {
        let assign13430_e12744: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign13430_e12747: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign13430_e12748: f64 = (assign13430_e12744 + assign13430_e12747);
        let assign13430_e12750: f64 = (assign13430_e12748 + locals.var_q_aexp);
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign13430_e12750, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9), );

        let assign13440_e12753: f64 = (locals.var_q_d2_qi * locals.var_q_expnum);
        let assign13440_e12756: f64 = (2.0 * locals.var_q_d1_qi);
        let assign13440_e12758: f64 = (assign13440_e12756 * locals.var_q_d1_expnum);
        let assign13440_e12759: f64 = (assign13440_e12753 + assign13440_e12758);
        let assign13440_e12762: f64 = (locals.var_q_qi_int * locals.var_q_d2_expnum);
        let assign13440_e12763: f64 = (assign13440_e12759 + assign13440_e12762);
        let assign13440_e12765: f64 = (assign13440_e12763 - locals.var_q_aexp);
        (locals.var_q_d2_zero, locals.var_q_d2_zero_dn4, locals.var_q_d2_zero_dn6, locals.var_q_d2_zero_dn7, locals.var_q_d2_zero_dn8, locals.var_q_d2_zero_dn9, ) = (assign13440_e12765, (((((locals.var_q_d2_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_d1_qi_dn4) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn4))) + ((locals.var_q_qi_int_dn4 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn4))) - locals.var_q_aexp_dn4), (((((locals.var_q_d2_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_d1_qi_dn6) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn6))) + ((locals.var_q_qi_int_dn6 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn6))) - locals.var_q_aexp_dn6), (((((locals.var_q_d2_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_d1_qi_dn7) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn7))) + ((locals.var_q_qi_int_dn7 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn7))) - locals.var_q_aexp_dn7), (((((locals.var_q_d2_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_d1_qi_dn8) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn8))) + ((locals.var_q_qi_int_dn8 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn8))) - locals.var_q_aexp_dn8), (((((locals.var_q_d2_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qi * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_d1_qi_dn9) * locals.var_q_d1_expnum) + (assign13440_e12756 * locals.var_q_d1_expnum_dn9))) + ((locals.var_q_qi_int_dn9 * locals.var_q_d2_expnum) + (locals.var_q_qi_int * locals.var_q_d2_expnum_dn9))) - locals.var_q_aexp_dn9), );

        let assign13450_e12768: f64 = (locals.var_q_d1_zero * locals.var_q_d1_zero);
        let assign13450_e12771: f64 = (0.5 * locals.var_q_zero);
        let assign13450_e12773: f64 = (assign13450_e12771 * locals.var_q_d2_zero);
        let assign13450_e12774: f64 = (assign13450_e12768 - assign13450_e12773);
        (locals.var_q_temp, locals.var_q_temp_dn4, locals.var_q_temp_dn6, locals.var_q_temp_dn7, locals.var_q_temp_dn8, locals.var_q_temp_dn9, ) = (assign13450_e12774, (((locals.var_q_d1_zero_dn4 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn4)) - (((0.5 * locals.var_q_zero_dn4) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn4))), (((locals.var_q_d1_zero_dn6 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn6)) - (((0.5 * locals.var_q_zero_dn6) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn6))), (((locals.var_q_d1_zero_dn7 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn7)) - (((0.5 * locals.var_q_zero_dn7) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn7))), (((locals.var_q_d1_zero_dn8 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn8)) - (((0.5 * locals.var_q_zero_dn8) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn8))), (((locals.var_q_d1_zero_dn9 * locals.var_q_d1_zero) + (locals.var_q_d1_zero * locals.var_q_d1_zero_dn9)) - (((0.5 * locals.var_q_zero_dn9) * locals.var_q_d2_zero) + (assign13450_e12771 * locals.var_q_d2_zero_dn9))), );

        let assign13460_e12776: f64 = (-locals.var_q_zero);
        let assign13460_e12778: f64 = (assign13460_e12776 * locals.var_q_d1_zero);
        let assign13460_e12780: f64 = (assign13460_e12778 * locals.var_q_temp);
        let assign13460_e12783: f64 = (locals.var_q_temp * locals.var_q_temp);
        let assign13460_e12785: f64 = (assign13460_e12783 + 1e-200);
        let assign13460_e12786: f64 = (assign13460_e12780 / assign13460_e12785);
        (locals.var_q_eps2, locals.var_q_eps2_dn4, locals.var_q_eps2_dn6, locals.var_q_eps2_dn7, locals.var_q_eps2_dn8, locals.var_q_eps2_dn9, ) = (assign13460_e12786, ((((((((-locals.var_q_zero_dn4) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn4)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn4)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn4 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn4)))) / (assign13460_e12785 * assign13460_e12785)), ((((((((-locals.var_q_zero_dn6) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn6)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn6)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn6 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn6)))) / (assign13460_e12785 * assign13460_e12785)), ((((((((-locals.var_q_zero_dn7) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn7)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn7)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn7 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn7)))) / (assign13460_e12785 * assign13460_e12785)), ((((((((-locals.var_q_zero_dn8) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn8)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn8)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn8 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn8)))) / (assign13460_e12785 * assign13460_e12785)), ((((((((-locals.var_q_zero_dn9) * locals.var_q_d1_zero) + (assign13460_e12776 * locals.var_q_d1_zero_dn9)) * locals.var_q_temp) + (assign13460_e12778 * locals.var_q_temp_dn9)) * assign13460_e12785) - (assign13460_e12780 * ((locals.var_q_temp_dn9 * locals.var_q_temp) + (locals.var_q_temp * locals.var_q_temp_dn9)))) / (assign13460_e12785 * assign13460_e12785)), );

        let assign13470_e12789: f64 = (locals.var_q1s + locals.var_q_eps2);
        (locals.var_q1s, locals.var_q1s_dn4, locals.var_q1s_dn6, locals.var_q1s_dn7, locals.var_q1s_dn8, locals.var_q1s_dn9, ) = (assign13470_e12789, (locals.var_q1s_dn4 + locals.var_q_eps2_dn4), (locals.var_q1s_dn6 + locals.var_q_eps2_dn6), (locals.var_q1s_dn7 + locals.var_q_eps2_dn7), (locals.var_q1s_dn8 + locals.var_q_eps2_dn8), (locals.var_q1s_dn9 + locals.var_q_eps2_dn9), );

        let assign13480_e12792: f64 = (locals.var_k1 * locals.var_q1s);
        (locals.var_q_k1q1, locals.var_q_k1q1_dn4, locals.var_q_k1q1_dn6, locals.var_q_k1q1_dn7, locals.var_q_k1q1_dn8, locals.var_q_k1q1_dn9, ) = (assign13480_e12792, ((locals.var_k1_dn4 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn4)), ((locals.var_k1_dn6 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn6)), ((locals.var_k1_dn7 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn7)), ((locals.var_k1_dn8 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn8)), ((locals.var_k1_dn9 * locals.var_q1s) + (locals.var_k1 * locals.var_q1s_dn9)), );

        let assign13490_e12795: f64 = (locals.var_xg1x - locals.var_q1s);
        let assign13490_e12797: f64 = assign13490_e12795;
        let assign13490_e12799: f64 = if assign13490_e12797 < 80.0 { 1.0 } else { 0.0 };
        locals.var_guard572 = assign13490_e12799;

        if (locals.var_guard572 != 0.0) {
            let assign13500_e12803: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign13500_e12805: f64 = assign13500_e12803;
            let assign13500_e12806: f64 = (assign13500_e12805).exp();
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13500_e12806, (assign13500_e12806 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)), (assign13500_e12806 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)), (assign13500_e12806 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)), (assign13500_e12806 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)), (assign13500_e12806 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)), );
        }

        if (locals.var_guard572 == 0.0) {
            let assign13510_e12815: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign13510_e12817: f64 = assign13510_e12815;
            let assign13510_e12819: f64 = (assign13510_e12817 - 80.0);
            let assign13510_e12824: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign13510_e12826: f64 = assign13510_e12824;
            let assign13510_e12828: f64 = (assign13510_e12826 - 80.0);
            let assign13510_e12829: f64 = (0.5 * assign13510_e12828);
            let assign13510_e12833: f64 = (locals.var_xg1x - locals.var_q1s);
            let assign13510_e12835: f64 = assign13510_e12833;
            let assign13510_e12837: f64 = (assign13510_e12835 - 80.0);
            let assign13510_e12839: f64 = (assign13510_e12837 * 0.3333333333333);
            let assign13510_e12840: f64 = (1.0 + assign13510_e12839);
            let assign13510_e12841: f64 = (assign13510_e12829 * assign13510_e12840);
            let assign13510_e12842: f64 = (1.0 + assign13510_e12841);
            let assign13510_e12843: f64 = (assign13510_e12819 * assign13510_e12842);
            let assign13510_e12844: f64 = (1.0 + assign13510_e12843);
            let assign13510_e12845: f64 = (5.54062e34 * assign13510_e12844);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13510_e12845, (5.54062e34 * (((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn4 - locals.var_q1s_dn4)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn4 - locals.var_q1s_dn4) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn6 - locals.var_q1s_dn6)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn6 - locals.var_q1s_dn6) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn7 - locals.var_q1s_dn7)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn7 - locals.var_q1s_dn7) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn8 - locals.var_q1s_dn8)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn8 - locals.var_q1s_dn8) * 0.3333333333333)))))), (5.54062e34 * (((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * assign13510_e12842) + (assign13510_e12819 * (((0.5 * (locals.var_xg1x_dn9 - locals.var_q1s_dn9)) * assign13510_e12840) + (assign13510_e12829 * ((locals.var_xg1x_dn9 - locals.var_q1s_dn9) * 0.3333333333333)))))), );
        }

        let assign13520_e12850: f64 = (locals.var_a0 * locals.var_q_temp1);
        (locals.var_q_aexp, locals.var_q_aexp_dn4, locals.var_q_aexp_dn6, locals.var_q_aexp_dn7, locals.var_q_aexp_dn8, locals.var_q_aexp_dn9, ) = (assign13520_e12850, ((locals.var_a0_dn4 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn4)), ((locals.var_a0_dn6 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn6)), ((locals.var_a0_dn7 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn7)), ((locals.var_a0_dn8 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn8)), ((locals.var_a0_dn9 * locals.var_q_temp1) + (locals.var_a0 * locals.var_q_temp1_dn9)), );

        let assign13530_e12853: f64 = (locals.var_q_k1q1 * locals.var_q_k1q1);
        let assign13530_e12855: f64 = (assign13530_e12853 - locals.var_q_aexp);
        (locals.var_q_qsq, locals.var_q_qsq_dn4, locals.var_q_qsq_dn6, locals.var_q_qsq_dn7, locals.var_q_qsq_dn8, locals.var_q_qsq_dn9, ) = (assign13530_e12855, (((locals.var_q_k1q1_dn4 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_k1q1_dn6 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_k1q1_dn7 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_k1q1_dn8 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_k1q1_dn9 * locals.var_q_k1q1) + (locals.var_q_k1q1 * locals.var_q_k1q1_dn9)) - locals.var_q_aexp_dn9), );

        let assign13540_e12858: f64 = (2.0 * locals.var_k1);
        let assign13540_e12860: f64 = (assign13540_e12858 * locals.var_q_k1q1);
        let assign13540_e12862: f64 = (assign13540_e12860 + locals.var_q_aexp);
        (locals.var_q_d1_qsq, locals.var_q_d1_qsq_dn4, locals.var_q_d1_qsq_dn6, locals.var_q_d1_qsq_dn7, locals.var_q_d1_qsq_dn8, locals.var_q_d1_qsq_dn9, ) = (assign13540_e12862, ((((2.0 * locals.var_k1_dn4) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn4)) + locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn6)) + locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn7)) + locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn8)) + locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_q_k1q1) + (assign13540_e12858 * locals.var_q_k1q1_dn9)) + locals.var_q_aexp_dn9), );

        let assign13550_e12865: f64 = (2.0 * locals.var_k1);
        let assign13550_e12867: f64 = (assign13550_e12865 * locals.var_k1);
        let assign13550_e12869: f64 = (assign13550_e12867 - locals.var_q_aexp);
        (locals.var_q_d2_qsq, locals.var_q_d2_qsq_dn4, locals.var_q_d2_qsq_dn6, locals.var_q_d2_qsq_dn7, locals.var_q_d2_qsq_dn8, locals.var_q_d2_qsq_dn9, ) = (assign13550_e12869, ((((2.0 * locals.var_k1_dn4) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn4)) - locals.var_q_aexp_dn4), ((((2.0 * locals.var_k1_dn6) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn6)) - locals.var_q_aexp_dn6), ((((2.0 * locals.var_k1_dn7) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn7)) - locals.var_q_aexp_dn7), ((((2.0 * locals.var_k1_dn8) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn8)) - locals.var_q_aexp_dn8), ((((2.0 * locals.var_k1_dn9) * locals.var_k1) + (assign13550_e12865 * locals.var_k1_dn9)) - locals.var_q_aexp_dn9), );

        let assign13560_e12872: f64 = (-0.005);
        let assign13560_e12873: f64 = if locals.var_q_qsq < assign13560_e12872 { 1.0 } else { 0.0 };
        locals.var_guard573 = assign13560_e12873;

        if (locals.var_guard573 != 0.0) {
            let assign13570_e12876: f64 = (locals.var_q_qsq).abs();
            let assign13570_e12877: f64 = (assign13570_e12876).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign13570_e12877, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13570_e12877)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13570_e12877)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13580_e12884: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign13580_e12885: f64 = (assign13580_e12884).tan();
            let assign13580_e12886: f64 = (locals.var_q_rac_qsq / assign13580_e12885);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13580_e12886, (((locals.var_q_rac_qsq_dn4 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn4) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn6 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn6) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn7 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn7) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn8 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn8) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), (((locals.var_q_rac_qsq_dn9 * assign13580_e12885) - (locals.var_q_rac_qsq * ((0.5 * locals.var_q_rac_qsq_dn9) / ((assign13580_e12884).cos() * (assign13580_e12884).cos())))) / (assign13580_e12885 * assign13580_e12885)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13590_e12892: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign13590_e12894: f64 = (assign13590_e12892 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13590_e12894, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13590_e12892 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13600_e12902: f64 = (2.0 - locals.var_q_qcoth);
            let assign13600_e12903: f64 = (locals.var_q_qcoth * assign13600_e12902);
            let assign13600_e12904: f64 = (locals.var_q_qsq + assign13600_e12903);
            let assign13600_e12906: f64 = (assign13600_e12904 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13600_e12906, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13600_e12902) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13600_e12904 * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13610_e12913: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign13610_e12916: f64 = (1.0 + locals.var_q_qcoth);
            let assign13610_e12917: f64 = (assign13610_e12913 * assign13610_e12916);
            let assign13610_e12918: f64 = (locals.var_q_d1_qsq - assign13610_e12917);
            let assign13610_e12920: f64 = (assign13610_e12918 * locals.var_q_temp1);
            let assign13610_e12923: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign13610_e12925: f64 = (assign13610_e12923 / locals.var_q_d1_qsq);
            let assign13610_e12926: f64 = (assign13610_e12920 + assign13610_e12925);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13610_e12926, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13610_e12916) + (assign13610_e12913 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13610_e12918 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13610_e12923 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13620_e12933: f64 = (0.5 * locals.var_q_qcoth);
            let assign13620_e12934: f64 = (1.0 - assign13620_e12933);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13620_e12934, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13630_e12940: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign13630_e12942: f64 = (assign13630_e12940 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13630_e12942, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13630_e12940 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard573 != 0.0) {
            let assign13640_e12948: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign13640_e12953: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign13640_e12954: f64 = (locals.var_q_d1_ln + assign13640_e12953);
            let assign13640_e12955: f64 = (locals.var_q_d1_qsq * assign13640_e12954);
            let assign13640_e12956: f64 = (assign13640_e12948 - assign13640_e12955);
            let assign13640_e12958: f64 = (assign13640_e12956 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13640_e12958, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13640_e12954) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13640_e12956 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        let assign13650_e12963: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard574 = assign13650_e12963;

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13660_e12969: f64 = (locals.var_q_qsq).abs();
            let assign13660_e12970: f64 = (assign13660_e12969).sqrt();
            (locals.var_q_rac_qsq, locals.var_q_rac_qsq_dn4, locals.var_q_rac_qsq_dn6, locals.var_q_rac_qsq_dn7, locals.var_q_rac_qsq_dn8, locals.var_q_rac_qsq_dn9, ) = (assign13660_e12970, (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn4 } else { (-locals.var_q_qsq_dn4) } / (2.0 * assign13660_e12970)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn6 } else { (-locals.var_q_qsq_dn6) } / (2.0 * assign13660_e12970)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn7 } else { (-locals.var_q_qsq_dn7) } / (2.0 * assign13660_e12970)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn8 } else { (-locals.var_q_qsq_dn8) } / (2.0 * assign13660_e12970)), (if locals.var_q_qsq >= 0.0 { locals.var_q_qsq_dn9 } else { (-locals.var_q_qsq_dn9) } / (2.0 * assign13660_e12970)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13670_e12978: f64 = (-locals.var_q_rac_qsq);
            let assign13670_e12979: f64 = (assign13670_e12978).exp();
            (locals.var_q_invexpq, locals.var_q_invexpq_dn4, locals.var_q_invexpq_dn6, locals.var_q_invexpq_dn7, locals.var_q_invexpq_dn8, locals.var_q_invexpq_dn9, ) = (assign13670_e12979, (assign13670_e12979 * (-locals.var_q_rac_qsq_dn4)), (assign13670_e12979 * (-locals.var_q_rac_qsq_dn6)), (assign13670_e12979 * (-locals.var_q_rac_qsq_dn7)), (assign13670_e12979 * (-locals.var_q_rac_qsq_dn8)), (assign13670_e12979 * (-locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13680_e12989: f64 = (1.0 + locals.var_q_invexpq);
            let assign13680_e12990: f64 = (locals.var_q_rac_qsq * assign13680_e12989);
            let assign13680_e12993: f64 = (1.0 - locals.var_q_invexpq);
            let assign13680_e12994: f64 = (assign13680_e12990 / assign13680_e12993);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13680_e12994, (((((locals.var_q_rac_qsq_dn4 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn4)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn4))) / (assign13680_e12993 * assign13680_e12993)), (((((locals.var_q_rac_qsq_dn6 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn6)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn6))) / (assign13680_e12993 * assign13680_e12993)), (((((locals.var_q_rac_qsq_dn7 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn7)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn7))) / (assign13680_e12993 * assign13680_e12993)), (((((locals.var_q_rac_qsq_dn8 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn8)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn8))) / (assign13680_e12993 * assign13680_e12993)), (((((locals.var_q_rac_qsq_dn9 * assign13680_e12989) + (locals.var_q_rac_qsq * locals.var_q_invexpq_dn9)) * assign13680_e12993) - (assign13680_e12990 * (-locals.var_q_invexpq_dn9))) / (assign13680_e12993 * assign13680_e12993)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13690_e13003: f64 = (0.25 * locals.var_q_d1_qsq);
            let assign13690_e13005: f64 = (assign13690_e13003 / locals.var_q_qsq);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13690_e13005, ((((0.25 * locals.var_q_d1_qsq_dn4) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn6) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn7) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn8) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((0.25 * locals.var_q_d1_qsq_dn9) * locals.var_q_qsq) - (assign13690_e13003 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13700_e13016: f64 = (2.0 - locals.var_q_qcoth);
            let assign13700_e13017: f64 = (locals.var_q_qcoth * assign13700_e13016);
            let assign13700_e13018: f64 = (locals.var_q_qsq + assign13700_e13017);
            let assign13700_e13020: f64 = (assign13700_e13018 * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13700_e13020, (((locals.var_q_qsq_dn4 + ((locals.var_q_qcoth_dn4 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn4)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn4)), (((locals.var_q_qsq_dn6 + ((locals.var_q_qcoth_dn6 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn6)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn6)), (((locals.var_q_qsq_dn7 + ((locals.var_q_qcoth_dn7 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn7)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn7)), (((locals.var_q_qsq_dn8 + ((locals.var_q_qcoth_dn8 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn8)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn8)), (((locals.var_q_qsq_dn9 + ((locals.var_q_qcoth_dn9 * assign13700_e13016) + (locals.var_q_qcoth * (-locals.var_q_qcoth_dn9)))) * locals.var_q_temp1) + (assign13700_e13018 * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13710_e13030: f64 = (2.0 * locals.var_q_d1_qcoth);
            let assign13710_e13033: f64 = (1.0 + locals.var_q_qcoth);
            let assign13710_e13034: f64 = (assign13710_e13030 * assign13710_e13033);
            let assign13710_e13035: f64 = (locals.var_q_d1_qsq - assign13710_e13034);
            let assign13710_e13037: f64 = (assign13710_e13035 * locals.var_q_temp1);
            let assign13710_e13040: f64 = (locals.var_q_d1_qcoth * locals.var_q_d2_qsq);
            let assign13710_e13042: f64 = (assign13710_e13040 / locals.var_q_d1_qsq);
            let assign13710_e13043: f64 = (assign13710_e13037 + assign13710_e13042);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13710_e13043, ((((locals.var_q_d1_qsq_dn4 - (((2.0 * locals.var_q_d1_qcoth_dn4) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn4))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn4)) + (((((locals.var_q_d1_qcoth_dn4 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn4)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn4)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn6 - (((2.0 * locals.var_q_d1_qcoth_dn6) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn6))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn6)) + (((((locals.var_q_d1_qcoth_dn6 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn6)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn6)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn7 - (((2.0 * locals.var_q_d1_qcoth_dn7) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn7))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn7)) + (((((locals.var_q_d1_qcoth_dn7 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn7)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn7)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn8 - (((2.0 * locals.var_q_d1_qcoth_dn8) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn8))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn8)) + (((((locals.var_q_d1_qcoth_dn8 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn8)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn8)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), ((((locals.var_q_d1_qsq_dn9 - (((2.0 * locals.var_q_d1_qcoth_dn9) * assign13710_e13033) + (assign13710_e13030 * locals.var_q_qcoth_dn9))) * locals.var_q_temp1) + (assign13710_e13035 * locals.var_q_temp1_dn9)) + (((((locals.var_q_d1_qcoth_dn9 * locals.var_q_d2_qsq) + (locals.var_q_d1_qcoth * locals.var_q_d2_qsq_dn9)) * locals.var_q_d1_qsq) - (assign13710_e13040 * locals.var_q_d1_qsq_dn9)) / (locals.var_q_d1_qsq * locals.var_q_d1_qsq))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13720_e13053: f64 = (0.5 * locals.var_q_qcoth);
            let assign13720_e13054: f64 = (1.0 - assign13720_e13053);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13720_e13054, (-(0.5 * locals.var_q_qcoth_dn4)), (-(0.5 * locals.var_q_qcoth_dn6)), (-(0.5 * locals.var_q_qcoth_dn7)), (-(0.5 * locals.var_q_qcoth_dn8)), (-(0.5 * locals.var_q_qcoth_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13730_e13063: f64 = (locals.var_q_d1_qsq / locals.var_q_qsq);
            let assign13730_e13065: f64 = (assign13730_e13063 * locals.var_q_temp2);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13730_e13065, (((((locals.var_q_d1_qsq_dn4 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn4)), (((((locals.var_q_d1_qsq_dn6 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn6)), (((((locals.var_q_d1_qsq_dn7 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn7)), (((((locals.var_q_d1_qsq_dn8 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn8)), (((((locals.var_q_d1_qsq_dn9 * locals.var_q_qsq) - (locals.var_q_d1_qsq * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)) * locals.var_q_temp2) + (assign13730_e13063 * locals.var_q_temp2_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 != 0.0)) {
            let assign13740_e13074: f64 = (locals.var_q_d2_qsq * locals.var_q_temp2);
            let assign13740_e13079: f64 = (0.5 * locals.var_q_d1_qcoth);
            let assign13740_e13080: f64 = (locals.var_q_d1_ln + assign13740_e13079);
            let assign13740_e13081: f64 = (locals.var_q_d1_qsq * assign13740_e13080);
            let assign13740_e13082: f64 = (assign13740_e13074 - assign13740_e13081);
            let assign13740_e13084: f64 = (assign13740_e13082 / locals.var_q_qsq);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13740_e13084, ((((((locals.var_q_d2_qsq_dn4 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn4)) - ((locals.var_q_d1_qsq_dn4 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn4 + (0.5 * locals.var_q_d1_qcoth_dn4))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn4)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn6 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn6)) - ((locals.var_q_d1_qsq_dn6 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn6 + (0.5 * locals.var_q_d1_qcoth_dn6))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn6)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn7 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn7)) - ((locals.var_q_d1_qsq_dn7 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn7 + (0.5 * locals.var_q_d1_qcoth_dn7))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn7)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn8 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn8)) - ((locals.var_q_d1_qsq_dn8 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn8 + (0.5 * locals.var_q_d1_qcoth_dn8))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn8)) / (locals.var_q_qsq * locals.var_q_qsq)), ((((((locals.var_q_d2_qsq_dn9 * locals.var_q_temp2) + (locals.var_q_d2_qsq * locals.var_q_temp2_dn9)) - ((locals.var_q_d1_qsq_dn9 * assign13740_e13080) + (locals.var_q_d1_qsq * (locals.var_q_d1_ln_dn9 + (0.5 * locals.var_q_d1_qcoth_dn9))))) * locals.var_q_qsq) - (assign13740_e13082 * locals.var_q_qsq_dn9)) / (locals.var_q_qsq * locals.var_q_qsq)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13750_e13096: f64 = (locals.var_q_qsq * 0.0166666666667);
            let assign13750_e13100: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign13750_e13104: f64 = (locals.var_q_qsq * 0.025);
            let assign13750_e13105: f64 = (1.0 - assign13750_e13104);
            let assign13750_e13106: f64 = (assign13750_e13100 * assign13750_e13105);
            let assign13750_e13107: f64 = (1.0 - assign13750_e13106);
            let assign13750_e13108: f64 = (assign13750_e13096 * assign13750_e13107);
            let assign13750_e13109: f64 = (1.0 - assign13750_e13108);
            let assign13750_e13110: f64 = (0.1666666666667 * assign13750_e13109);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign13750_e13110, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn4 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn6 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn7 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn8 * 0.025))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0166666666667) * assign13750_e13107) + (assign13750_e13096 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13750_e13105) + (assign13750_e13100 * (-(locals.var_q_qsq_dn9 * 0.025))))))))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13760_e13121: f64 = (locals.var_q_qsq * locals.var_q_temp3);
            let assign13760_e13122: f64 = (2.0 + assign13760_e13121);
            (locals.var_q_qcoth, locals.var_q_qcoth_dn4, locals.var_q_qcoth_dn6, locals.var_q_qcoth_dn7, locals.var_q_qcoth_dn8, locals.var_q_qcoth_dn9, ) = (assign13760_e13122, ((locals.var_q_qsq_dn4 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn4)), ((locals.var_q_qsq_dn6 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn6)), ((locals.var_q_qsq_dn7 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn7)), ((locals.var_q_qsq_dn8 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn8)), ((locals.var_q_qsq_dn9 * locals.var_q_temp3) + (locals.var_q_qsq * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13770_e13134: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign13770_e13138: f64 = (locals.var_q_qsq * 0.0357142857143);
            let assign13770_e13142: f64 = (locals.var_q_qsq * 0.0333333333333);
            let assign13770_e13143: f64 = (1.0 - assign13770_e13142);
            let assign13770_e13144: f64 = (assign13770_e13138 * assign13770_e13143);
            let assign13770_e13145: f64 = (1.0 - assign13770_e13144);
            let assign13770_e13146: f64 = (assign13770_e13134 * assign13770_e13145);
            let assign13770_e13147: f64 = (1.0 - assign13770_e13146);
            let assign13770_e13148: f64 = (0.1666666666667 * assign13770_e13147);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign13770_e13148, (0.1666666666667 * (-(((locals.var_q_qsq_dn4 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn4 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn4 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn6 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn6 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn6 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn7 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn7 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn7 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn8 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn8 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn8 * 0.0333333333333))))))))), (0.1666666666667 * (-(((locals.var_q_qsq_dn9 * 0.0333333333333) * assign13770_e13145) + (assign13770_e13134 * (-(((locals.var_q_qsq_dn9 * 0.0357142857143) * assign13770_e13143) + (assign13770_e13138 * (-(locals.var_q_qsq_dn9 * 0.0333333333333))))))))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13780_e13158: f64 = (locals.var_q_d1_qsq * locals.var_q_temp1);
            (locals.var_q_d1_qcoth, locals.var_q_d1_qcoth_dn4, locals.var_q_d1_qcoth_dn6, locals.var_q_d1_qcoth_dn7, locals.var_q_d1_qcoth_dn8, locals.var_q_d1_qcoth_dn9, ) = (assign13780_e13158, ((locals.var_q_d1_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn4)), ((locals.var_q_d1_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn6)), ((locals.var_q_d1_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn7)), ((locals.var_q_d1_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn8)), ((locals.var_q_d1_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d1_qsq * locals.var_q_temp1_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13790_e13170: f64 = (locals.var_q_qsq * 0.0714285714286);
            let assign13790_e13174: f64 = (0.05 * locals.var_q_qsq);
            let assign13790_e13178: f64 = (0.0420875420875421 * locals.var_q_qsq);
            let assign13790_e13179: f64 = (1.0 - assign13790_e13178);
            let assign13790_e13180: f64 = (assign13790_e13174 * assign13790_e13179);
            let assign13790_e13181: f64 = (1.0 - assign13790_e13180);
            let assign13790_e13182: f64 = (assign13790_e13170 * assign13790_e13181);
            let assign13790_e13183: f64 = (1.0 - assign13790_e13182);
            let assign13790_e13184: f64 = (0.0055555555556 * assign13790_e13183);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13790_e13184, (0.0055555555556 * (-(((locals.var_q_qsq_dn4 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn4))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn6 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn6))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn7 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn7))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn8 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn8))))))))), (0.0055555555556 * (-(((locals.var_q_qsq_dn9 * 0.0714285714286) * assign13790_e13181) + (assign13790_e13170 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13790_e13179) + (assign13790_e13174 * (-(0.0420875420875421 * locals.var_q_qsq_dn9))))))))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13800_e13194: f64 = (locals.var_q_d2_qsq * locals.var_q_temp1);
            let assign13800_e13197: f64 = (locals.var_q_d1_qsq * locals.var_q_d1_qsq);
            let assign13800_e13199: f64 = (assign13800_e13197 * locals.var_q_temp2);
            let assign13800_e13200: f64 = (assign13800_e13194 - assign13800_e13199);
            (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, ) = (assign13800_e13200, (((locals.var_q_d2_qsq_dn4 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn4)) - ((((locals.var_q_d1_qsq_dn4 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn4)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn4))), (((locals.var_q_d2_qsq_dn6 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn6)) - ((((locals.var_q_d1_qsq_dn6 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn6)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn6))), (((locals.var_q_d2_qsq_dn7 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn7)) - ((((locals.var_q_d1_qsq_dn7 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn7)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn7))), (((locals.var_q_d2_qsq_dn8 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn8)) - ((((locals.var_q_d1_qsq_dn8 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn8)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn8))), (((locals.var_q_d2_qsq_dn9 * locals.var_q_temp1) + (locals.var_q_d2_qsq * locals.var_q_temp1_dn9)) - ((((locals.var_q_d1_qsq_dn9 * locals.var_q_d1_qsq) + (locals.var_q_d1_qsq * locals.var_q_d1_qsq_dn9)) * locals.var_q_temp2) + (assign13800_e13197 * locals.var_q_temp2_dn9))), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13810_e13209: f64 = (-0.5);
            let assign13810_e13211: f64 = (assign13810_e13209 * locals.var_q_d1_qsq);
            let assign13810_e13213: f64 = (assign13810_e13211 * locals.var_q_temp3);
            (locals.var_q_d1_ln, locals.var_q_d1_ln_dn4, locals.var_q_d1_ln_dn6, locals.var_q_d1_ln_dn7, locals.var_q_d1_ln_dn8, locals.var_q_d1_ln_dn9, ) = (assign13810_e13213, (((assign13810_e13209 * locals.var_q_d1_qsq_dn4) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn4)), (((assign13810_e13209 * locals.var_q_d1_qsq_dn6) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn6)), (((assign13810_e13209 * locals.var_q_d1_qsq_dn7) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn7)), (((assign13810_e13209 * locals.var_q_d1_qsq_dn8) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn8)), (((assign13810_e13209 * locals.var_q_d1_qsq_dn9) * locals.var_q_temp3) + (assign13810_e13211 * locals.var_q_temp3_dn9)), );
        }

        if ((locals.var_guard573 == 0.0) && (locals.var_guard574 == 0.0)) {
            let assign13820_e13222: f64 = (-0.5);
            let assign13820_e13224: f64 = (assign13820_e13222 * locals.var_q_d2_qsq);
            let assign13820_e13226: f64 = (assign13820_e13224 * locals.var_q_temp3);
            let assign13820_e13229: f64 = (0.25 * 0.0055555555556);
            let assign13820_e13231: f64 = (assign13820_e13229 * locals.var_q_d1_qsq);
            let assign13820_e13233: f64 = (assign13820_e13231 * locals.var_q_d1_qsq);
            let assign13820_e13237: f64 = (locals.var_q_qsq * 0.0238095238095);
            let assign13820_e13241: f64 = (0.075 * locals.var_q_qsq);
            let assign13820_e13242: f64 = (2.0 - assign13820_e13241);
            let assign13820_e13243: f64 = (assign13820_e13237 * assign13820_e13242);
            let assign13820_e13244: f64 = (1.0 - assign13820_e13243);
            let assign13820_e13245: f64 = (assign13820_e13233 * assign13820_e13244);
            let assign13820_e13246: f64 = (assign13820_e13226 + assign13820_e13245);
            (locals.var_q_d2_ln, locals.var_q_d2_ln_dn4, locals.var_q_d2_ln_dn6, locals.var_q_d2_ln_dn7, locals.var_q_d2_ln_dn8, locals.var_q_d2_ln_dn9, ) = (assign13820_e13246, ((((assign13820_e13222 * locals.var_q_d2_qsq_dn4) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn4)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn4) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn4)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn4 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn4)))))))), ((((assign13820_e13222 * locals.var_q_d2_qsq_dn6) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn6)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn6) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn6)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn6 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn6)))))))), ((((assign13820_e13222 * locals.var_q_d2_qsq_dn7) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn7)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn7) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn7)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn7 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn7)))))))), ((((assign13820_e13222 * locals.var_q_d2_qsq_dn8) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn8)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn8) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn8)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn8 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn8)))))))), ((((assign13820_e13222 * locals.var_q_d2_qsq_dn9) * locals.var_q_temp3) + (assign13820_e13224 * locals.var_q_temp3_dn9)) + (((((assign13820_e13229 * locals.var_q_d1_qsq_dn9) * locals.var_q_d1_qsq) + (assign13820_e13231 * locals.var_q_d1_qsq_dn9)) * assign13820_e13244) + (assign13820_e13233 * (-(((locals.var_q_qsq_dn9 * 0.0238095238095) * assign13820_e13242) + (assign13820_e13237 * (-(0.075 * locals.var_q_qsq_dn9)))))))), );
        }

        let assign13830_e13251: f64 = if locals.var_q_qsq > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard575 = assign13830_e13251;

        if (locals.var_guard575 != 0.0) {
            let assign13840_e13255: f64 = (4.0 * locals.var_q_qsq);
            let assign13840_e13260: f64 = (2.0 - locals.var_q_invexpq);
            let assign13840_e13261: f64 = (locals.var_q_invexpq * assign13840_e13260);
            let assign13840_e13262: f64 = (1.0 - assign13840_e13261);
            let assign13840_e13263: f64 = (assign13840_e13255 / assign13840_e13262);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13840_e13263, ((((4.0 * locals.var_q_qsq_dn4) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn4 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn4)))))) / (assign13840_e13262 * assign13840_e13262)), ((((4.0 * locals.var_q_qsq_dn6) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn6 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn6)))))) / (assign13840_e13262 * assign13840_e13262)), ((((4.0 * locals.var_q_qsq_dn7) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn7 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn7)))))) / (assign13840_e13262 * assign13840_e13262)), ((((4.0 * locals.var_q_qsq_dn8) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn8 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn8)))))) / (assign13840_e13262 * assign13840_e13262)), ((((4.0 * locals.var_q_qsq_dn9) * assign13840_e13262) - (assign13840_e13255 * (-((locals.var_q_invexpq_dn9 * assign13840_e13260) + (locals.var_q_invexpq * (-locals.var_q_invexpq_dn9)))))) / (assign13840_e13262 * assign13840_e13262)), );
        }

        if (locals.var_guard575 != 0.0) {
            let assign13850_e13269: f64 = (locals.var_q_temp2 * locals.var_q_invexpq);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13850_e13269, ((locals.var_q_temp2_dn4 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn4)), ((locals.var_q_temp2_dn6 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn6)), ((locals.var_q_temp2_dn7 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn7)), ((locals.var_q_temp2_dn8 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn8)), ((locals.var_q_temp2_dn9 * locals.var_q_invexpq) + (locals.var_q_temp2 * locals.var_q_invexpq_dn9)), );
        }

        if (locals.var_guard575 != 0.0) {
            let assign13860_e13274: f64 = (locals.var_q_temp2).ln();
            let assign13860_e13276: f64 = (assign13860_e13274 - locals.var_q_rac_qsq);
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13860_e13276, ((locals.var_q_temp2_dn4 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn4), ((locals.var_q_temp2_dn6 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn6), ((locals.var_q_temp2_dn7 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn7), ((locals.var_q_temp2_dn8 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn8), ((locals.var_q_temp2_dn9 / locals.var_q_temp2) - locals.var_q_rac_qsq_dn9), );
        }

        let assign13870_e13281: f64 = (-0.005);
        let assign13870_e13282: f64 = if locals.var_q_qsq < assign13870_e13281 { 1.0 } else { 0.0 };
        locals.var_guard576 = assign13870_e13282;

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
            let assign13880_e13289: f64 = (0.5 * locals.var_q_rac_qsq);
            let assign13880_e13290: f64 = (assign13880_e13289).sin();
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13880_e13290, ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn4)), ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn6)), ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn7)), ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn8)), ((assign13880_e13289).cos() * (0.5 * locals.var_q_rac_qsq_dn9)), );
        }

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
            let assign13890_e13298: f64 = (-locals.var_q_qsq);
            let assign13890_e13301: f64 = (locals.var_q_temp2 * locals.var_q_temp2);
            let assign13890_e13302: f64 = (assign13890_e13298 / assign13890_e13301);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13890_e13302, ((((-locals.var_q_qsq_dn4) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn4 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn4)))) / (assign13890_e13301 * assign13890_e13301)), ((((-locals.var_q_qsq_dn6) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn6 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn6)))) / (assign13890_e13301 * assign13890_e13301)), ((((-locals.var_q_qsq_dn7) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn7 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn7)))) / (assign13890_e13301 * assign13890_e13301)), ((((-locals.var_q_qsq_dn8) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn8 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn8)))) / (assign13890_e13301 * assign13890_e13301)), ((((-locals.var_q_qsq_dn9) * assign13890_e13301) - (assign13890_e13298 * ((locals.var_q_temp2_dn9 * locals.var_q_temp2) + (locals.var_q_temp2 * locals.var_q_temp2_dn9)))) / (assign13890_e13301 * assign13890_e13301)), );
        }

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 != 0.0)) {
            let assign13900_e13310: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13900_e13310, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 == 0.0)) {
            let assign13910_e13321: f64 = (locals.var_q_qsq * 0.3333333333333);
            let assign13910_e13325: f64 = (0.05 * locals.var_q_qsq);
            let assign13910_e13329: f64 = (0.0396825396825397 * locals.var_q_qsq);
            let assign13910_e13330: f64 = (1.0 - assign13910_e13329);
            let assign13910_e13331: f64 = (assign13910_e13325 * assign13910_e13330);
            let assign13910_e13332: f64 = (1.0 - assign13910_e13331);
            let assign13910_e13333: f64 = (assign13910_e13321 * assign13910_e13332);
            let assign13910_e13334: f64 = (4.0 - assign13910_e13333);
            (locals.var_q_sh_term, locals.var_q_sh_term_dn4, locals.var_q_sh_term_dn6, locals.var_q_sh_term_dn7, locals.var_q_sh_term_dn8, locals.var_q_sh_term_dn9, ) = (assign13910_e13334, (-(((locals.var_q_qsq_dn4 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn4) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn4)))))))), (-(((locals.var_q_qsq_dn6 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn6) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn6)))))))), (-(((locals.var_q_qsq_dn7 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn7) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn7)))))))), (-(((locals.var_q_qsq_dn8 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn8) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn8)))))))), (-(((locals.var_q_qsq_dn9 * 0.3333333333333) * assign13910_e13332) + (assign13910_e13321 * (-(((0.05 * locals.var_q_qsq_dn9) * assign13910_e13330) + (assign13910_e13325 * (-(0.0396825396825397 * locals.var_q_qsq_dn9)))))))), );
        }

        if ((locals.var_guard575 == 0.0) && (locals.var_guard576 == 0.0)) {
            let assign13920_e13343: f64 = (locals.var_q_sh_term).ln();
            (locals.var_q_ln_term, locals.var_q_ln_term_dn4, locals.var_q_ln_term_dn6, locals.var_q_ln_term_dn7, locals.var_q_ln_term_dn8, locals.var_q_ln_term_dn9, ) = (assign13920_e13343, (locals.var_q_sh_term_dn4 / locals.var_q_sh_term), (locals.var_q_sh_term_dn6 / locals.var_q_sh_term), (locals.var_q_sh_term_dn7 / locals.var_q_sh_term), (locals.var_q_sh_term_dn8 / locals.var_q_sh_term), (locals.var_q_sh_term_dn9 / locals.var_q_sh_term), );
        }

        let assign13930_e13348: f64 = (1.01 * locals.var_q_k1q1);
        let assign13930_e13350: f64 = (assign13930_e13348 + locals.var_q_qcoth);
        let assign13930_e13352: f64 = if assign13930_e13350 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign13930_e13352;

        if (locals.var_guard577 != 0.0) {
            let assign13940_e13356: f64 = (locals.var_q_k1q1 + locals.var_q_qcoth);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign13940_e13356, (locals.var_q_k1q1_dn4 + locals.var_q_qcoth_dn4), (locals.var_q_k1q1_dn6 + locals.var_q_qcoth_dn6), (locals.var_q_k1q1_dn7 + locals.var_q_qcoth_dn7), (locals.var_q_k1q1_dn8 + locals.var_q_qcoth_dn8), (locals.var_q_k1q1_dn9 + locals.var_q_qcoth_dn9), );
        }

        if (locals.var_guard577 != 0.0) {
            let assign13950_e13362: f64 = (locals.var_k1 + locals.var_q_d1_qcoth);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign13950_e13362, (locals.var_k1_dn4 + locals.var_q_d1_qcoth_dn4), (locals.var_k1_dn6 + locals.var_q_d1_qcoth_dn6), (locals.var_k1_dn7 + locals.var_q_d1_qcoth_dn7), (locals.var_k1_dn8 + locals.var_q_d1_qcoth_dn8), (locals.var_k1_dn9 + locals.var_q_d1_qcoth_dn9), );
        }

        if (locals.var_guard577 != 0.0) {
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (locals.var_q_d2_qcoth, locals.var_q_d2_qcoth_dn4, locals.var_q_d2_qcoth_dn6, locals.var_q_d2_qcoth_dn7, locals.var_q_d2_qcoth_dn8, locals.var_q_d2_qcoth_dn9, );
        }

        if (locals.var_guard577 == 0.0) {
            let assign13970_e13374: f64 = (locals.var_q_k1q1 - locals.var_q_qcoth);
            let assign13970_e13375: f64 = (1.0 / assign13970_e13374);
            (locals.var_q_temp2, locals.var_q_temp2_dn4, locals.var_q_temp2_dn6, locals.var_q_temp2_dn7, locals.var_q_temp2_dn8, locals.var_q_temp2_dn9, ) = (assign13970_e13375, (-((locals.var_q_k1q1_dn4 - locals.var_q_qcoth_dn4) / (assign13970_e13374 * assign13970_e13374))), (-((locals.var_q_k1q1_dn6 - locals.var_q_qcoth_dn6) / (assign13970_e13374 * assign13970_e13374))), (-((locals.var_q_k1q1_dn7 - locals.var_q_qcoth_dn7) / (assign13970_e13374 * assign13970_e13374))), (-((locals.var_q_k1q1_dn8 - locals.var_q_qcoth_dn8) / (assign13970_e13374 * assign13970_e13374))), (-((locals.var_q_k1q1_dn9 - locals.var_q_qcoth_dn9) / (assign13970_e13374 * assign13970_e13374))), );
        }

        if (locals.var_guard577 == 0.0) {
            let assign13980_e13382: f64 = (locals.var_q_d1_qcoth - locals.var_k1);
            (locals.var_q_temp3, locals.var_q_temp3_dn4, locals.var_q_temp3_dn6, locals.var_q_temp3_dn7, locals.var_q_temp3_dn8, locals.var_q_temp3_dn9, ) = (assign13980_e13382, (locals.var_q_d1_qcoth_dn4 - locals.var_k1_dn4), (locals.var_q_d1_qcoth_dn6 - locals.var_k1_dn6), (locals.var_q_d1_qcoth_dn7 - locals.var_k1_dn7), (locals.var_q_d1_qcoth_dn8 - locals.var_k1_dn8), (locals.var_q_d1_qcoth_dn9 - locals.var_k1_dn9), );
        }

        if (locals.var_guard577 == 0.0) {
            let assign13990_e13389: f64 = (locals.var_q_aexp - locals.var_q_sh_term);
            let assign13990_e13391: f64 = (assign13990_e13389 * locals.var_q_temp2);
            (locals.var_q_expnum, locals.var_q_expnum_dn4, locals.var_q_expnum_dn6, locals.var_q_expnum_dn7, locals.var_q_expnum_dn8, locals.var_q_expnum_dn9, ) = (assign13990_e13391, (((locals.var_q_aexp_dn4 - locals.var_q_sh_term_dn4) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn4)), (((locals.var_q_aexp_dn6 - locals.var_q_sh_term_dn6) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn6)), (((locals.var_q_aexp_dn7 - locals.var_q_sh_term_dn7) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn7)), (((locals.var_q_aexp_dn8 - locals.var_q_sh_term_dn8) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn8)), (((locals.var_q_aexp_dn9 - locals.var_q_sh_term_dn9) * locals.var_q_temp2) + (assign13990_e13389 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard577 == 0.0) {
            let assign14000_e13398: f64 = (locals.var_q_temp3 * locals.var_q_expnum);
            let assign14000_e13400: f64 = (assign14000_e13398 - locals.var_q_aexp);
            let assign14000_e13403: f64 = (locals.var_q_d1_ln * locals.var_q_sh_term);
            let assign14000_e13404: f64 = (assign14000_e13400 - assign14000_e13403);
            let assign14000_e13406: f64 = (assign14000_e13404 * locals.var_q_temp2);
            (locals.var_q_d1_expnum, locals.var_q_d1_expnum_dn4, locals.var_q_d1_expnum_dn6, locals.var_q_d1_expnum_dn7, locals.var_q_d1_expnum_dn8, locals.var_q_d1_expnum_dn9, ) = (assign14000_e13406, ((((((locals.var_q_temp3_dn4 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4) - ((locals.var_q_d1_ln_dn4 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn4)), ((((((locals.var_q_temp3_dn6 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6) - ((locals.var_q_d1_ln_dn6 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn6)), ((((((locals.var_q_temp3_dn7 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7) - ((locals.var_q_d1_ln_dn7 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn7)), ((((((locals.var_q_temp3_dn8 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8) - ((locals.var_q_d1_ln_dn8 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn8)), ((((((locals.var_q_temp3_dn9 * locals.var_q_expnum) + (locals.var_q_temp3 * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9) - ((locals.var_q_d1_ln_dn9 * locals.var_q_sh_term) + (locals.var_q_d1_ln * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14000_e13404 * locals.var_q_temp2_dn9)), );
        }

        if (locals.var_guard577 == 0.0) {
            let assign14010_e13413: f64 = (locals.var_q_d2_qcoth * locals.var_q_expnum);
            let assign14010_e13416: f64 = (2.0 * locals.var_q_temp3);
            let assign14010_e13418: f64 = (assign14010_e13416 * locals.var_q_d1_expnum);
            let assign14010_e13419: f64 = (assign14010_e13413 + assign14010_e13418);
            let assign14010_e13421: f64 = (assign14010_e13419 + locals.var_q_aexp);
            let assign14010_e13425: f64 = (locals.var_q_d1_ln * locals.var_q_d1_ln);
            let assign14010_e13426: f64 = (locals.var_q_d2_ln + assign14010_e13425);
            let assign14010_e13428: f64 = (assign14010_e13426 * locals.var_q_sh_term);
            let assign14010_e13429: f64 = (assign14010_e13421 - assign14010_e13428);
            let assign14010_e13431: f64 = (assign14010_e13429 * locals.var_q_temp2);
            (locals.var_q_d2_expnum, locals.var_q_d2_expnum_dn4, locals.var_q_d2_expnum_dn6, locals.var_q_d2_expnum_dn7, locals.var_q_d2_expnum_dn8, locals.var_q_d2_expnum_dn9, ) = (assign14010_e13431, (((((((locals.var_q_d2_qcoth_dn4 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn4)) + (((2.0 * locals.var_q_temp3_dn4) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4) - (((locals.var_q_d2_ln_dn4 + ((locals.var_q_d1_ln_dn4 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn4))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn4))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn4)), (((((((locals.var_q_d2_qcoth_dn6 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn6)) + (((2.0 * locals.var_q_temp3_dn6) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6) - (((locals.var_q_d2_ln_dn6 + ((locals.var_q_d1_ln_dn6 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn6))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn6))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn6)), (((((((locals.var_q_d2_qcoth_dn7 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn7)) + (((2.0 * locals.var_q_temp3_dn7) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7) - (((locals.var_q_d2_ln_dn7 + ((locals.var_q_d1_ln_dn7 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn7))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn7))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn7)), (((((((locals.var_q_d2_qcoth_dn8 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn8)) + (((2.0 * locals.var_q_temp3_dn8) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8) - (((locals.var_q_d2_ln_dn8 + ((locals.var_q_d1_ln_dn8 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn8))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn8))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn8)), (((((((locals.var_q_d2_qcoth_dn9 * locals.var_q_expnum) + (locals.var_q_d2_qcoth * locals.var_q_expnum_dn9)) + (((2.0 * locals.var_q_temp3_dn9) * locals.var_q_d1_expnum) + (assign14010_e13416 * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9) - (((locals.var_q_d2_ln_dn9 + ((locals.var_q_d1_ln_dn9 * locals.var_q_d1_ln) + (locals.var_q_d1_ln * locals.var_q_d1_ln_dn9))) * locals.var_q_sh_term) + (assign14010_e13426 * locals.var_q_sh_term_dn9))) * locals.var_q_temp2) + (assign14010_e13429 * locals.var_q_temp2_dn9)), );
        }

        let assign14020_e13436: f64 = if locals.var_q_expnum > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard578 = assign14020_e13436;

        if (locals.var_guard578 != 0.0) {
            let assign14030_e13439: f64 = (locals.var_q_expnum).ln();
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign14030_e13439, (locals.var_q_expnum_dn4 / locals.var_q_expnum), (locals.var_q_expnum_dn6 / locals.var_q_expnum), (locals.var_q_expnum_dn7 / locals.var_q_expnum), (locals.var_q_expnum_dn8 / locals.var_q_expnum), (locals.var_q_expnum_dn9 / locals.var_q_expnum), );
        }

        if (locals.var_guard578 != 0.0) {
            let assign14040_e13445: f64 = (1.0 / locals.var_q_expnum);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign14040_e13445, (-(locals.var_q_expnum_dn4 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn6 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn7 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn8 / (locals.var_q_expnum * locals.var_q_expnum))), (-(locals.var_q_expnum_dn9 / (locals.var_q_expnum * locals.var_q_expnum))), );
        }

        if (locals.var_guard578 != 0.0) {
            let assign14050_e13451: f64 = (locals.var_q_d1_expnum * locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign14050_e13451, ((locals.var_q_d1_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn4)), ((locals.var_q_d1_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn6)), ((locals.var_q_d1_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn7)), ((locals.var_q_d1_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn8)), ((locals.var_q_d1_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d1_expnum * locals.var_q_temp1_dn9)), );
        }

        if (locals.var_guard578 != 0.0) {
            let assign14060_e13457: f64 = (locals.var_q_d2_expnum * locals.var_q_temp1);
            let assign14060_e13460: f64 = (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum);
            let assign14060_e13461: f64 = (assign14060_e13457 - assign14060_e13460);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign14060_e13461, (((locals.var_q_d2_expnum_dn4 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn4)) - ((locals.var_q_d1_lnexpnum_dn4 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn4))), (((locals.var_q_d2_expnum_dn6 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn6)) - ((locals.var_q_d1_lnexpnum_dn6 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn6))), (((locals.var_q_d2_expnum_dn7 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn7)) - ((locals.var_q_d1_lnexpnum_dn7 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn7))), (((locals.var_q_d2_expnum_dn8 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn8)) - ((locals.var_q_d1_lnexpnum_dn8 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn8))), (((locals.var_q_d2_expnum_dn9 * locals.var_q_temp1) + (locals.var_q_d2_expnum * locals.var_q_temp1_dn9)) - ((locals.var_q_d1_lnexpnum_dn9 * locals.var_q_d1_lnexpnum) + (locals.var_q_d1_lnexpnum * locals.var_q_d1_lnexpnum_dn9))), );
        }

        if (locals.var_guard578 == 0.0) {
            let assign14070_e13468: f64 = (locals.var_q_k1q1 + 0.6931471805599);
            let assign14070_e13470: f64 = (-locals.var_q_k1q1);
            let assign14070_e13471: f64 = (assign14070_e13470).ln();
            let assign14070_e13472: f64 = (assign14070_e13468 + assign14070_e13471);
            (locals.var_q_lnexpnum, locals.var_q_lnexpnum_dn4, locals.var_q_lnexpnum_dn6, locals.var_q_lnexpnum_dn7, locals.var_q_lnexpnum_dn8, locals.var_q_lnexpnum_dn9, ) = (assign14070_e13472, (locals.var_q_k1q1_dn4 + ((-locals.var_q_k1q1_dn4) / assign14070_e13470)), (locals.var_q_k1q1_dn6 + ((-locals.var_q_k1q1_dn6) / assign14070_e13470)), (locals.var_q_k1q1_dn7 + ((-locals.var_q_k1q1_dn7) / assign14070_e13470)), (locals.var_q_k1q1_dn8 + ((-locals.var_q_k1q1_dn8) / assign14070_e13470)), (locals.var_q_k1q1_dn9 + ((-locals.var_q_k1q1_dn9) / assign14070_e13470)), );
        }

        if (locals.var_guard578 == 0.0) {
            let assign14080_e13479: f64 = (1.0 / locals.var_q1s);
            (locals.var_q_temp1, locals.var_q_temp1_dn4, locals.var_q_temp1_dn6, locals.var_q_temp1_dn7, locals.var_q_temp1_dn8, locals.var_q_temp1_dn9, ) = (assign14080_e13479, (-(locals.var_q1s_dn4 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn6 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn7 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn8 / (locals.var_q1s * locals.var_q1s))), (-(locals.var_q1s_dn9 / (locals.var_q1s * locals.var_q1s))), );
        }

        if (locals.var_guard578 == 0.0) {
            let assign14090_e13486: f64 = (locals.var_k1 + locals.var_q_temp1);
            (locals.var_q_d1_lnexpnum, locals.var_q_d1_lnexpnum_dn4, locals.var_q_d1_lnexpnum_dn6, locals.var_q_d1_lnexpnum_dn7, locals.var_q_d1_lnexpnum_dn8, locals.var_q_d1_lnexpnum_dn9, ) = (assign14090_e13486, (locals.var_k1_dn4 + locals.var_q_temp1_dn4), (locals.var_k1_dn6 + locals.var_q_temp1_dn6), (locals.var_k1_dn7 + locals.var_q_temp1_dn7), (locals.var_k1_dn8 + locals.var_q_temp1_dn8), (locals.var_k1_dn9 + locals.var_q_temp1_dn9), );
        }

        if (locals.var_guard578 == 0.0) {
            let assign14100_e13492: f64 = (-locals.var_q_temp1);
            let assign14100_e13494: f64 = (assign14100_e13492 * locals.var_q_temp1);
            (locals.var_q_d2_lnexpnum, locals.var_q_d2_lnexpnum_dn4, locals.var_q_d2_lnexpnum_dn6, locals.var_q_d2_lnexpnum_dn7, locals.var_q_d2_lnexpnum_dn8, locals.var_q_d2_lnexpnum_dn9, ) = (assign14100_e13494, (((-locals.var_q_temp1_dn4) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn4)), (((-locals.var_q_temp1_dn6) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn6)), (((-locals.var_q_temp1_dn7) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn7)), (((-locals.var_q_temp1_dn8) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn8)), (((-locals.var_q_temp1_dn9) * locals.var_q_temp1) + (assign14100_e13492 * locals.var_q_temp1_dn9)), );
        }

        let assign14110_e13499: f64 = (locals.var_xg2x - locals.var_xg1x);
        let assign14110_e13501: f64 = (assign14110_e13499 + locals.var_q1s);
        let assign14110_e13504: f64 = (2.0 * locals.var_q_lnexpnum);
        let assign14110_e13505: f64 = (assign14110_e13501 + assign14110_e13504);
        let assign14110_e13507: f64 = (assign14110_e13505 - locals.var_q_ln_term);
        (locals.var_q_q2_int, locals.var_q_q2_int_dn4, locals.var_q_q2_int_dn6, locals.var_q_q2_int_dn7, locals.var_q_q2_int_dn8, locals.var_q_q2_int_dn9, ) = (assign14110_e13507, ((((locals.var_xg2x_dn4 - locals.var_xg1x_dn4) + locals.var_q1s_dn4) + (2.0 * locals.var_q_lnexpnum_dn4)) - locals.var_q_ln_term_dn4), ((((locals.var_xg2x_dn6 - locals.var_xg1x_dn6) + locals.var_q1s_dn6) + (2.0 * locals.var_q_lnexpnum_dn6)) - locals.var_q_ln_term_dn6), ((((locals.var_xg2x_dn7 - locals.var_xg1x_dn7) + locals.var_q1s_dn7) + (2.0 * locals.var_q_lnexpnum_dn7)) - locals.var_q_ln_term_dn7), ((((locals.var_xg2x_dn8 - locals.var_xg1x_dn8) + locals.var_q1s_dn8) + (2.0 * locals.var_q_lnexpnum_dn8)) - locals.var_q_ln_term_dn8), ((((locals.var_xg2x_dn9 - locals.var_xg1x_dn9) + locals.var_q1s_dn9) + (2.0 * locals.var_q_lnexpnum_dn9)) - locals.var_q_ln_term_dn9), );

        let assign14120_e13511: f64 = (2.0 * locals.var_q_d1_lnexpnum);
        let assign14120_e13512: f64 = (1.0 + assign14120_e13511);
        let assign14120_e13514: f64 = (assign14120_e13512 - locals.var_q_d1_ln);
        (locals.var_q_d1_q2, locals.var_q_d1_q2_dn4, locals.var_q_d1_q2_dn6, locals.var_q_d1_q2_dn7, locals.var_q_d1_q2_dn8, locals.var_q_d1_q2_dn9, ) = (assign14120_e13514, ((2.0 * locals.var_q_d1_lnexpnum_dn4) - locals.var_q_d1_ln_dn4), ((2.0 * locals.var_q_d1_lnexpnum_dn6) - locals.var_q_d1_ln_dn6), ((2.0 * locals.var_q_d1_lnexpnum_dn7) - locals.var_q_d1_ln_dn7), ((2.0 * locals.var_q_d1_lnexpnum_dn8) - locals.var_q_d1_ln_dn8), ((2.0 * locals.var_q_d1_lnexpnum_dn9) - locals.var_q_d1_ln_dn9), );

        let assign14130_e13517: f64 = (2.0 * locals.var_q_d2_lnexpnum);
        let assign14130_e13519: f64 = (assign14130_e13517 - locals.var_q_d2_ln);
        (locals.var_q_d2_q2, locals.var_q_d2_q2_dn4, locals.var_q_d2_q2_dn6, locals.var_q_d2_q2_dn7, locals.var_q_d2_q2_dn8, locals.var_q_d2_q2_dn9, ) = (assign14130_e13519, ((2.0 * locals.var_q_d2_lnexpnum_dn4) - locals.var_q_d2_ln_dn4), ((2.0 * locals.var_q_d2_lnexpnum_dn6) - locals.var_q_d2_ln_dn6), ((2.0 * locals.var_q_d2_lnexpnum_dn7) - locals.var_q_d2_ln_dn7), ((2.0 * locals.var_q_d2_lnexpnum_dn8) - locals.var_q_d2_ln_dn8), ((2.0 * locals.var_q_d2_lnexpnum_dn9) - locals.var_q_d2_ln_dn9), );

        let assign14140_e13523: f64 = (locals.var_k2 * locals.var_q_q2_int);
        let assign14140_e13524: f64 = (locals.var_q_k1q1 + assign14140_e13523);
        (locals.var_q_qi_int, locals.var_q_qi_int_dn4, locals.var_q_qi_int_dn6, locals.var_q_qi_int_dn7, locals.var_q_qi_int_dn8, locals.var_q_qi_int_dn9, ) = (assign14140_e13524, (locals.var_q_k1q1_dn4 + ((locals.var_k2_dn4 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn4))), (locals.var_q_k1q1_dn6 + ((locals.var_k2_dn6 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn6))), (locals.var_q_k1q1_dn7 + ((locals.var_k2_dn7 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn7))), (locals.var_q_k1q1_dn8 + ((locals.var_k2_dn8 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn8))), (locals.var_q_k1q1_dn9 + ((locals.var_k2_dn9 * locals.var_q_q2_int) + (locals.var_k2 * locals.var_q_q2_int_dn9))), );

        let assign14150_e13528: f64 = (locals.var_k2 * locals.var_q_d1_q2);
        let assign14150_e13529: f64 = (locals.var_k1 + assign14150_e13528);
        (locals.var_q_d1_qi, locals.var_q_d1_qi_dn4, locals.var_q_d1_qi_dn6, locals.var_q_d1_qi_dn7, locals.var_q_d1_qi_dn8, locals.var_q_d1_qi_dn9, ) = (assign14150_e13529, (locals.var_k1_dn4 + ((locals.var_k2_dn4 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn4))), (locals.var_k1_dn6 + ((locals.var_k2_dn6 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn6))), (locals.var_k1_dn7 + ((locals.var_k2_dn7 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn7))), (locals.var_k1_dn8 + ((locals.var_k2_dn8 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn8))), (locals.var_k1_dn9 + ((locals.var_k2_dn9 * locals.var_q_d1_q2) + (locals.var_k2 * locals.var_q_d1_q2_dn9))), );

        let assign14160_e13532: f64 = (locals.var_k2 * locals.var_q_d2_q2);
        (locals.var_q_d2_qi, locals.var_q_d2_qi_dn4, locals.var_q_d2_qi_dn6, locals.var_q_d2_qi_dn7, locals.var_q_d2_qi_dn8, locals.var_q_d2_qi_dn9, ) = (assign14160_e13532, ((locals.var_k2_dn4 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn4)), ((locals.var_k2_dn6 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn6)), ((locals.var_k2_dn7 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn7)), ((locals.var_k2_dn8 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn8)), ((locals.var_k2_dn9 * locals.var_q_d2_q2) + (locals.var_k2 * locals.var_q_d2_q2_dn9)), );

        let assign14170_e13535: f64 = (locals.var_q_qi_int * locals.var_q_expnum);
        let assign14170_e13537: f64 = (assign14170_e13535 - locals.var_q_aexp);
        (locals.var_q_zero, locals.var_q_zero_dn4, locals.var_q_zero_dn6, locals.var_q_zero_dn7, locals.var_q_zero_dn8, locals.var_q_zero_dn9, ) = (assign14170_e13537, (((locals.var_q_qi_int_dn4 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn4)) - locals.var_q_aexp_dn4), (((locals.var_q_qi_int_dn6 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn6)) - locals.var_q_aexp_dn6), (((locals.var_q_qi_int_dn7 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn7)) - locals.var_q_aexp_dn7), (((locals.var_q_qi_int_dn8 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn8)) - locals.var_q_aexp_dn8), (((locals.var_q_qi_int_dn9 * locals.var_q_expnum) + (locals.var_q_qi_int * locals.var_q_expnum_dn9)) - locals.var_q_aexp_dn9), );

        let assign14180_e13540: f64 = (locals.var_q_d1_qi * locals.var_q_expnum);
        let assign14180_e13543: f64 = (locals.var_q_qi_int * locals.var_q_d1_expnum);
        let assign14180_e13544: f64 = (assign14180_e13540 + assign14180_e13543);
        let assign14180_e13546: f64 = (assign14180_e13544 + locals.var_q_aexp);
        (locals.var_q_d1_zero, locals.var_q_d1_zero_dn4, locals.var_q_d1_zero_dn6, locals.var_q_d1_zero_dn7, locals.var_q_d1_zero_dn8, locals.var_q_d1_zero_dn9, ) = (assign14180_e13546, ((((locals.var_q_d1_qi_dn4 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn4)) + ((locals.var_q_qi_int_dn4 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn4))) + locals.var_q_aexp_dn4), ((((locals.var_q_d1_qi_dn6 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn6)) + ((locals.var_q_qi_int_dn6 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn6))) + locals.var_q_aexp_dn6), ((((locals.var_q_d1_qi_dn7 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn7)) + ((locals.var_q_qi_int_dn7 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn7))) + locals.var_q_aexp_dn7), ((((locals.var_q_d1_qi_dn8 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn8)) + ((locals.var_q_qi_int_dn8 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn8))) + locals.var_q_aexp_dn8), ((((locals.var_q_d1_qi_dn9 * locals.var_q_expnum) + (locals.var_q_d1_qi * locals.var_q_expnum_dn9)) + ((locals.var_q_qi_int_dn9 * locals.var_q_d1_expnum) + (locals.var_q_qi_int * locals.var_q_d1_expnum_dn9))) + locals.var_q_aexp_dn9), );

    }
}
