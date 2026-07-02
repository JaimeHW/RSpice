#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign30_e1405: f64 = if param_given[12] { 1.0 } else { 0.0 };
        locals.var_nsubcdfm_given = assign30_e1405;

        let assign40_e1407: f64 = if param_given[268] { 1.0 } else { 0.0 };
        locals.var_cgdo_given = assign40_e1407;

        let assign50_e1409: f64 = if param_given[269] { 1.0 } else { 0.0 };
        locals.var_cgso_given = assign50_e1409;

        locals.var_cgdoe = 0.0;

        locals.var_cgsoe = 0.0;

        locals.var_xd = 0.0;
        locals.var_xd_dn0 = 0.0;
        locals.var_xd_dn2 = 0.0;
        locals.var_xd_dn4 = 0.0;
        locals.var_xd_dn5 = 0.0;
        locals.var_xd_dn6 = 0.0;
        locals.var_xd_dn7 = 0.0;
        locals.var_xd_dn8 = 0.0;
        locals.var_xd_dn9 = 0.0;
        locals.var_xd_dn10 = 0.0;
        locals.var_xd_dn11 = 0.0;
        locals.var_xd_dn14 = 0.0;

        locals.var_rdd = 0.0;
        locals.var_rdd_dn0 = 0.0;
        locals.var_rdd_dn2 = 0.0;
        locals.var_rdd_dn4 = 0.0;
        locals.var_rdd_dn5 = 0.0;
        locals.var_rdd_dn6 = 0.0;
        locals.var_rdd_dn7 = 0.0;
        locals.var_rdd_dn8 = 0.0;
        locals.var_rdd_dn9 = 0.0;
        locals.var_rdd_dn10 = 0.0;
        locals.var_rdd_dn11 = 0.0;
        locals.var_rdd_dn14 = 0.0;

        locals.var_rsd = 0.0;
        locals.var_rsd_dn0 = 0.0;
        locals.var_rsd_dn2 = 0.0;
        locals.var_rsd_dn4 = 0.0;
        locals.var_rsd_dn5 = 0.0;
        locals.var_rsd_dn6 = 0.0;
        locals.var_rsd_dn7 = 0.0;
        locals.var_rsd_dn8 = 0.0;
        locals.var_rsd_dn9 = 0.0;
        locals.var_rsd_dn10 = 0.0;
        locals.var_rsd_dn11 = 0.0;
        locals.var_rsd_dn14 = 0.0;

        locals.var_flg_ign = 0.0;

        locals.var_flg_noqi = 0.0;

        locals.var_flg_rsrd = 0.0;

        locals.var_flg_zone = 0.0;

        locals.var_rd_ps0ld = 0.0;
        locals.var_rd_ps0ld_dn0 = 0.0;
        locals.var_rd_ps0ld_dn2 = 0.0;
        locals.var_rd_ps0ld_dn4 = 0.0;
        locals.var_rd_ps0ld_dn5 = 0.0;
        locals.var_rd_ps0ld_dn6 = 0.0;
        locals.var_rd_ps0ld_dn7 = 0.0;
        locals.var_rd_ps0ld_dn8 = 0.0;
        locals.var_rd_ps0ld_dn9 = 0.0;
        locals.var_rd_ps0ld_dn10 = 0.0;
        locals.var_rd_ps0ld_dn11 = 0.0;
        locals.var_rd_ps0ld_dn14 = 0.0;

        locals.var_rd_qbuld = 0.0;
        locals.var_rd_qbuld_dn0 = 0.0;
        locals.var_rd_qbuld_dn2 = 0.0;
        locals.var_rd_qbuld_dn4 = 0.0;
        locals.var_rd_qbuld_dn5 = 0.0;
        locals.var_rd_qbuld_dn6 = 0.0;
        locals.var_rd_qbuld_dn7 = 0.0;
        locals.var_rd_qbuld_dn8 = 0.0;
        locals.var_rd_qbuld_dn9 = 0.0;
        locals.var_rd_qbuld_dn10 = 0.0;
        locals.var_rd_qbuld_dn11 = 0.0;
        locals.var_rd_qbuld_dn14 = 0.0;

        locals.var_vbs_max = 0.8;
        locals.var_vbs_max_dn0 = 0.0;
        locals.var_vbs_max_dn2 = 0.0;
        locals.var_vbs_max_dn4 = 0.0;
        locals.var_vbs_max_dn5 = 0.0;
        locals.var_vbs_max_dn6 = 0.0;
        locals.var_vbs_max_dn7 = 0.0;
        locals.var_vbs_max_dn8 = 0.0;
        locals.var_vbs_max_dn9 = 0.0;
        locals.var_vbs_max_dn10 = 0.0;
        locals.var_vbs_max_dn11 = 0.0;
        locals.var_vbs_max_dn14 = 0.0;

        locals.var_vbs_bnd = 0.4;
        locals.var_vbs_bnd_dn0 = 0.0;
        locals.var_vbs_bnd_dn2 = 0.0;
        locals.var_vbs_bnd_dn4 = 0.0;
        locals.var_vbs_bnd_dn5 = 0.0;
        locals.var_vbs_bnd_dn6 = 0.0;
        locals.var_vbs_bnd_dn7 = 0.0;
        locals.var_vbs_bnd_dn8 = 0.0;
        locals.var_vbs_bnd_dn9 = 0.0;
        locals.var_vbs_bnd_dn10 = 0.0;
        locals.var_vbs_bnd_dn11 = 0.0;
        locals.var_vbs_bnd_dn14 = 0.0;

        locals.var_flg_pprv = 0.0;

        locals.var_flg_conv = 0.0;

        locals.var_flg_qme = 0.0;

        locals.var_flg_nqs = 0.0;

        locals.var_vbscl = 0.0;
        locals.var_vbscl_dn0 = 0.0;
        locals.var_vbscl_dn2 = 0.0;
        locals.var_vbscl_dn4 = 0.0;
        locals.var_vbscl_dn5 = 0.0;
        locals.var_vbscl_dn6 = 0.0;
        locals.var_vbscl_dn7 = 0.0;
        locals.var_vbscl_dn8 = 0.0;
        locals.var_vbscl_dn9 = 0.0;
        locals.var_vbscl_dn10 = 0.0;
        locals.var_vbscl_dn11 = 0.0;
        locals.var_vbscl_dn14 = 0.0;

        locals.var_vbscldvbs = 0.0;
        locals.var_vbscldvbs_dn0 = 0.0;
        locals.var_vbscldvbs_dn2 = 0.0;
        locals.var_vbscldvbs_dn4 = 0.0;
        locals.var_vbscldvbs_dn5 = 0.0;
        locals.var_vbscldvbs_dn6 = 0.0;
        locals.var_vbscldvbs_dn7 = 0.0;
        locals.var_vbscldvbs_dn8 = 0.0;
        locals.var_vbscldvbs_dn9 = 0.0;
        locals.var_vbscldvbs_dn10 = 0.0;
        locals.var_vbscldvbs_dn11 = 0.0;
        locals.var_vbscldvbs_dn14 = 0.0;

        locals.var_vgp = 0.0;
        locals.var_vgp_dn0 = 0.0;
        locals.var_vgp_dn2 = 0.0;
        locals.var_vgp_dn4 = 0.0;
        locals.var_vgp_dn5 = 0.0;
        locals.var_vgp_dn6 = 0.0;
        locals.var_vgp_dn7 = 0.0;
        locals.var_vgp_dn8 = 0.0;
        locals.var_vgp_dn9 = 0.0;
        locals.var_vgp_dn10 = 0.0;
        locals.var_vgp_dn11 = 0.0;
        locals.var_vgp_dn14 = 0.0;

        locals.var_vgs_fb = 0.0;

        locals.var_ps0 = 0.0;
        locals.var_ps0_dn0 = 0.0;
        locals.var_ps0_dn2 = 0.0;
        locals.var_ps0_dn4 = 0.0;
        locals.var_ps0_dn5 = 0.0;
        locals.var_ps0_dn6 = 0.0;
        locals.var_ps0_dn7 = 0.0;
        locals.var_ps0_dn8 = 0.0;
        locals.var_ps0_dn9 = 0.0;
        locals.var_ps0_dn10 = 0.0;
        locals.var_ps0_dn11 = 0.0;
        locals.var_ps0_dn14 = 0.0;

        locals.var_ps0_ini = 0.0;
        locals.var_ps0_ini_dn0 = 0.0;
        locals.var_ps0_ini_dn2 = 0.0;
        locals.var_ps0_ini_dn4 = 0.0;
        locals.var_ps0_ini_dn5 = 0.0;
        locals.var_ps0_ini_dn6 = 0.0;
        locals.var_ps0_ini_dn7 = 0.0;
        locals.var_ps0_ini_dn8 = 0.0;
        locals.var_ps0_ini_dn9 = 0.0;
        locals.var_ps0_ini_dn10 = 0.0;
        locals.var_ps0_ini_dn11 = 0.0;
        locals.var_ps0_ini_dn14 = 0.0;

        locals.var_ps0_inia = 0.0;
        locals.var_ps0_inia_dn0 = 0.0;
        locals.var_ps0_inia_dn2 = 0.0;
        locals.var_ps0_inia_dn4 = 0.0;
        locals.var_ps0_inia_dn5 = 0.0;
        locals.var_ps0_inia_dn6 = 0.0;
        locals.var_ps0_inia_dn7 = 0.0;
        locals.var_ps0_inia_dn8 = 0.0;
        locals.var_ps0_inia_dn9 = 0.0;
        locals.var_ps0_inia_dn10 = 0.0;
        locals.var_ps0_inia_dn11 = 0.0;
        locals.var_ps0_inia_dn14 = 0.0;

        locals.var_ps0_inib = 0.0;
        locals.var_ps0_inib_dn0 = 0.0;
        locals.var_ps0_inib_dn2 = 0.0;
        locals.var_ps0_inib_dn4 = 0.0;
        locals.var_ps0_inib_dn5 = 0.0;
        locals.var_ps0_inib_dn6 = 0.0;
        locals.var_ps0_inib_dn7 = 0.0;
        locals.var_ps0_inib_dn8 = 0.0;
        locals.var_ps0_inib_dn9 = 0.0;
        locals.var_ps0_inib_dn10 = 0.0;
        locals.var_ps0_inib_dn11 = 0.0;
        locals.var_ps0_inib_dn14 = 0.0;

        locals.var_psl = 0.0;
        locals.var_psl_dn0 = 0.0;
        locals.var_psl_dn2 = 0.0;
        locals.var_psl_dn4 = 0.0;
        locals.var_psl_dn5 = 0.0;
        locals.var_psl_dn6 = 0.0;
        locals.var_psl_dn7 = 0.0;
        locals.var_psl_dn8 = 0.0;
        locals.var_psl_dn9 = 0.0;
        locals.var_psl_dn10 = 0.0;
        locals.var_psl_dn11 = 0.0;
        locals.var_psl_dn14 = 0.0;

        locals.var_psl_lim = 0.0;
        locals.var_psl_lim_dn0 = 0.0;
        locals.var_psl_lim_dn2 = 0.0;
        locals.var_psl_lim_dn4 = 0.0;
        locals.var_psl_lim_dn5 = 0.0;
        locals.var_psl_lim_dn6 = 0.0;
        locals.var_psl_lim_dn7 = 0.0;
        locals.var_psl_lim_dn8 = 0.0;
        locals.var_psl_lim_dn9 = 0.0;
        locals.var_psl_lim_dn10 = 0.0;
        locals.var_psl_lim_dn11 = 0.0;
        locals.var_psl_lim_dn14 = 0.0;

        locals.var_dplim = 0.0;
        locals.var_dplim_dn0 = 0.0;
        locals.var_dplim_dn2 = 0.0;
        locals.var_dplim_dn4 = 0.0;
        locals.var_dplim_dn5 = 0.0;
        locals.var_dplim_dn6 = 0.0;
        locals.var_dplim_dn7 = 0.0;
        locals.var_dplim_dn8 = 0.0;
        locals.var_dplim_dn9 = 0.0;
        locals.var_dplim_dn10 = 0.0;
        locals.var_dplim_dn11 = 0.0;
        locals.var_dplim_dn14 = 0.0;

        locals.var_pds = 0.0;
        locals.var_pds_dn0 = 0.0;
        locals.var_pds_dn2 = 0.0;
        locals.var_pds_dn4 = 0.0;
        locals.var_pds_dn5 = 0.0;
        locals.var_pds_dn6 = 0.0;
        locals.var_pds_dn7 = 0.0;
        locals.var_pds_dn8 = 0.0;
        locals.var_pds_dn9 = 0.0;
        locals.var_pds_dn10 = 0.0;
        locals.var_pds_dn11 = 0.0;
        locals.var_pds_dn14 = 0.0;

        locals.var_pds_ini = 0.0;
        locals.var_pds_ini_dn0 = 0.0;
        locals.var_pds_ini_dn2 = 0.0;
        locals.var_pds_ini_dn4 = 0.0;
        locals.var_pds_ini_dn5 = 0.0;
        locals.var_pds_ini_dn6 = 0.0;
        locals.var_pds_ini_dn7 = 0.0;
        locals.var_pds_ini_dn8 = 0.0;
        locals.var_pds_ini_dn9 = 0.0;
        locals.var_pds_ini_dn10 = 0.0;
        locals.var_pds_ini_dn11 = 0.0;
        locals.var_pds_ini_dn14 = 0.0;

        locals.var_pds_max = 0.0;
        locals.var_pds_max_dn0 = 0.0;
        locals.var_pds_max_dn2 = 0.0;
        locals.var_pds_max_dn4 = 0.0;
        locals.var_pds_max_dn5 = 0.0;
        locals.var_pds_max_dn6 = 0.0;
        locals.var_pds_max_dn7 = 0.0;
        locals.var_pds_max_dn8 = 0.0;
        locals.var_pds_max_dn9 = 0.0;
        locals.var_pds_max_dn10 = 0.0;
        locals.var_pds_max_dn11 = 0.0;
        locals.var_pds_max_dn14 = 0.0;

        locals.var_lp_s0 = 0.0;

        locals.var_lp_sl = 0.0;

        locals.var_xi0 = 0.0;
        locals.var_xi0_dn0 = 0.0;
        locals.var_xi0_dn2 = 0.0;
        locals.var_xi0_dn4 = 0.0;
        locals.var_xi0_dn5 = 0.0;
        locals.var_xi0_dn6 = 0.0;
        locals.var_xi0_dn7 = 0.0;
        locals.var_xi0_dn8 = 0.0;
        locals.var_xi0_dn9 = 0.0;
        locals.var_xi0_dn10 = 0.0;
        locals.var_xi0_dn11 = 0.0;
        locals.var_xi0_dn14 = 0.0;

        locals.var_xi0p12 = 0.0;
        locals.var_xi0p12_dn0 = 0.0;
        locals.var_xi0p12_dn2 = 0.0;
        locals.var_xi0p12_dn4 = 0.0;
        locals.var_xi0p12_dn5 = 0.0;
        locals.var_xi0p12_dn6 = 0.0;
        locals.var_xi0p12_dn7 = 0.0;
        locals.var_xi0p12_dn8 = 0.0;
        locals.var_xi0p12_dn9 = 0.0;
        locals.var_xi0p12_dn10 = 0.0;
        locals.var_xi0p12_dn11 = 0.0;
        locals.var_xi0p12_dn14 = 0.0;

        locals.var_xi0p32 = 0.0;
        locals.var_xi0p32_dn0 = 0.0;
        locals.var_xi0p32_dn2 = 0.0;
        locals.var_xi0p32_dn4 = 0.0;
        locals.var_xi0p32_dn5 = 0.0;
        locals.var_xi0p32_dn6 = 0.0;
        locals.var_xi0p32_dn7 = 0.0;
        locals.var_xi0p32_dn8 = 0.0;
        locals.var_xi0p32_dn9 = 0.0;
        locals.var_xi0p32_dn10 = 0.0;
        locals.var_xi0p32_dn11 = 0.0;
        locals.var_xi0p32_dn14 = 0.0;

        locals.var_xil = 0.0;
        locals.var_xil_dn0 = 0.0;
        locals.var_xil_dn2 = 0.0;
        locals.var_xil_dn4 = 0.0;
        locals.var_xil_dn5 = 0.0;
        locals.var_xil_dn6 = 0.0;
        locals.var_xil_dn7 = 0.0;
        locals.var_xil_dn8 = 0.0;
        locals.var_xil_dn9 = 0.0;
        locals.var_xil_dn10 = 0.0;
        locals.var_xil_dn11 = 0.0;
        locals.var_xil_dn14 = 0.0;

        locals.var_xilp12 = 0.0;
        locals.var_xilp12_dn0 = 0.0;
        locals.var_xilp12_dn2 = 0.0;
        locals.var_xilp12_dn4 = 0.0;
        locals.var_xilp12_dn5 = 0.0;
        locals.var_xilp12_dn6 = 0.0;
        locals.var_xilp12_dn7 = 0.0;
        locals.var_xilp12_dn8 = 0.0;
        locals.var_xilp12_dn9 = 0.0;
        locals.var_xilp12_dn10 = 0.0;
        locals.var_xilp12_dn11 = 0.0;
        locals.var_xilp12_dn14 = 0.0;

        locals.var_xilp32 = 0.0;
        locals.var_xilp32_dn0 = 0.0;
        locals.var_xilp32_dn2 = 0.0;
        locals.var_xilp32_dn4 = 0.0;
        locals.var_xilp32_dn5 = 0.0;
        locals.var_xilp32_dn6 = 0.0;
        locals.var_xilp32_dn7 = 0.0;
        locals.var_xilp32_dn8 = 0.0;
        locals.var_xilp32_dn9 = 0.0;
        locals.var_xilp32_dn10 = 0.0;
        locals.var_xilp32_dn11 = 0.0;
        locals.var_xilp32_dn14 = 0.0;

        locals.var_vbsz = 0.0;
        locals.var_vbsz_dn0 = 0.0;
        locals.var_vbsz_dn2 = 0.0;
        locals.var_vbsz_dn4 = 0.0;
        locals.var_vbsz_dn5 = 0.0;
        locals.var_vbsz_dn6 = 0.0;
        locals.var_vbsz_dn7 = 0.0;
        locals.var_vbsz_dn8 = 0.0;
        locals.var_vbsz_dn9 = 0.0;
        locals.var_vbsz_dn10 = 0.0;
        locals.var_vbsz_dn11 = 0.0;
        locals.var_vbsz_dn14 = 0.0;

        locals.var_vdsz = 0.0;
        locals.var_vdsz_dn0 = 0.0;
        locals.var_vdsz_dn2 = 0.0;
        locals.var_vdsz_dn4 = 0.0;
        locals.var_vdsz_dn5 = 0.0;
        locals.var_vdsz_dn6 = 0.0;
        locals.var_vdsz_dn7 = 0.0;
        locals.var_vdsz_dn8 = 0.0;
        locals.var_vdsz_dn9 = 0.0;
        locals.var_vdsz_dn10 = 0.0;
        locals.var_vdsz_dn11 = 0.0;
        locals.var_vdsz_dn14 = 0.0;

        locals.var_vgsz = 0.0;
        locals.var_vgsz_dn0 = 0.0;
        locals.var_vgsz_dn2 = 0.0;
        locals.var_vgsz_dn4 = 0.0;
        locals.var_vgsz_dn5 = 0.0;
        locals.var_vgsz_dn6 = 0.0;
        locals.var_vgsz_dn7 = 0.0;
        locals.var_vgsz_dn8 = 0.0;
        locals.var_vgsz_dn9 = 0.0;
        locals.var_vgsz_dn10 = 0.0;
        locals.var_vgsz_dn11 = 0.0;
        locals.var_vgsz_dn14 = 0.0;

        locals.var_vzadd = 0.0;
        locals.var_vzadd_dn0 = 0.0;
        locals.var_vzadd_dn2 = 0.0;
        locals.var_vzadd_dn4 = 0.0;
        locals.var_vzadd_dn5 = 0.0;
        locals.var_vzadd_dn6 = 0.0;
        locals.var_vzadd_dn7 = 0.0;
        locals.var_vzadd_dn8 = 0.0;
        locals.var_vzadd_dn9 = 0.0;
        locals.var_vzadd_dn10 = 0.0;
        locals.var_vzadd_dn11 = 0.0;
        locals.var_vzadd_dn14 = 0.0;

        locals.var_ps0z = 0.0;
        locals.var_ps0z_dn0 = 0.0;
        locals.var_ps0z_dn2 = 0.0;
        locals.var_ps0z_dn4 = 0.0;
        locals.var_ps0z_dn5 = 0.0;
        locals.var_ps0z_dn6 = 0.0;
        locals.var_ps0z_dn7 = 0.0;
        locals.var_ps0z_dn8 = 0.0;
        locals.var_ps0z_dn9 = 0.0;
        locals.var_ps0z_dn10 = 0.0;
        locals.var_ps0z_dn11 = 0.0;
        locals.var_ps0z_dn14 = 0.0;

        locals.var_pzadd = 0.0;
        locals.var_pzadd_dn0 = 0.0;
        locals.var_pzadd_dn2 = 0.0;
        locals.var_pzadd_dn4 = 0.0;
        locals.var_pzadd_dn5 = 0.0;
        locals.var_pzadd_dn6 = 0.0;
        locals.var_pzadd_dn7 = 0.0;
        locals.var_pzadd_dn8 = 0.0;
        locals.var_pzadd_dn9 = 0.0;
        locals.var_pzadd_dn10 = 0.0;
        locals.var_pzadd_dn11 = 0.0;
        locals.var_pzadd_dn14 = 0.0;

        locals.var_dvbsibpc = 0.0;
        locals.var_dvbsibpc_dn0 = 0.0;
        locals.var_dvbsibpc_dn2 = 0.0;
        locals.var_dvbsibpc_dn4 = 0.0;
        locals.var_dvbsibpc_dn5 = 0.0;
        locals.var_dvbsibpc_dn6 = 0.0;
        locals.var_dvbsibpc_dn7 = 0.0;
        locals.var_dvbsibpc_dn8 = 0.0;
        locals.var_dvbsibpc_dn9 = 0.0;
        locals.var_dvbsibpc_dn10 = 0.0;
        locals.var_dvbsibpc_dn11 = 0.0;
        locals.var_dvbsibpc_dn14 = 0.0;

        locals.var_dg3 = 0.0;
        locals.var_dg3_dn0 = 0.0;
        locals.var_dg3_dn2 = 0.0;
        locals.var_dg3_dn4 = 0.0;
        locals.var_dg3_dn5 = 0.0;
        locals.var_dg3_dn6 = 0.0;
        locals.var_dg3_dn7 = 0.0;
        locals.var_dg3_dn8 = 0.0;
        locals.var_dg3_dn9 = 0.0;
        locals.var_dg3_dn10 = 0.0;
        locals.var_dg3_dn11 = 0.0;
        locals.var_dg3_dn14 = 0.0;

        locals.var_dg4 = 0.0;
        locals.var_dg4_dn0 = 0.0;
        locals.var_dg4_dn2 = 0.0;
        locals.var_dg4_dn4 = 0.0;
        locals.var_dg4_dn5 = 0.0;
        locals.var_dg4_dn6 = 0.0;
        locals.var_dg4_dn7 = 0.0;
        locals.var_dg4_dn8 = 0.0;
        locals.var_dg4_dn9 = 0.0;
        locals.var_dg4_dn10 = 0.0;
        locals.var_dg4_dn11 = 0.0;
        locals.var_dg4_dn14 = 0.0;

        locals.var_didd = 0.0;
        locals.var_didd_dn0 = 0.0;
        locals.var_didd_dn2 = 0.0;
        locals.var_didd_dn4 = 0.0;
        locals.var_didd_dn5 = 0.0;
        locals.var_didd_dn6 = 0.0;
        locals.var_didd_dn7 = 0.0;
        locals.var_didd_dn8 = 0.0;
        locals.var_didd_dn9 = 0.0;
        locals.var_didd_dn10 = 0.0;
        locals.var_didd_dn11 = 0.0;
        locals.var_didd_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_1(
        locals: &mut StampLocals,
    ) {
        locals.var_betawl = 0.0;
        locals.var_betawl_dn0 = 0.0;
        locals.var_betawl_dn2 = 0.0;
        locals.var_betawl_dn4 = 0.0;
        locals.var_betawl_dn5 = 0.0;
        locals.var_betawl_dn6 = 0.0;
        locals.var_betawl_dn7 = 0.0;
        locals.var_betawl_dn8 = 0.0;
        locals.var_betawl_dn9 = 0.0;
        locals.var_betawl_dn10 = 0.0;
        locals.var_betawl_dn11 = 0.0;
        locals.var_betawl_dn14 = 0.0;

        locals.var_chi = 0.0;
        locals.var_chi_dn0 = 0.0;
        locals.var_chi_dn2 = 0.0;
        locals.var_chi_dn4 = 0.0;
        locals.var_chi_dn5 = 0.0;
        locals.var_chi_dn6 = 0.0;
        locals.var_chi_dn7 = 0.0;
        locals.var_chi_dn8 = 0.0;
        locals.var_chi_dn9 = 0.0;
        locals.var_chi_dn10 = 0.0;
        locals.var_chi_dn11 = 0.0;
        locals.var_chi_dn14 = 0.0;

        locals.var_chib = 0.0;
        locals.var_chib_dn0 = 0.0;
        locals.var_chib_dn2 = 0.0;
        locals.var_chib_dn4 = 0.0;
        locals.var_chib_dn5 = 0.0;
        locals.var_chib_dn6 = 0.0;
        locals.var_chib_dn7 = 0.0;
        locals.var_chib_dn8 = 0.0;
        locals.var_chib_dn9 = 0.0;
        locals.var_chib_dn10 = 0.0;
        locals.var_chib_dn11 = 0.0;
        locals.var_chib_dn14 = 0.0;

        locals.var_rho = 0.0;
        locals.var_rho_dn0 = 0.0;
        locals.var_rho_dn2 = 0.0;
        locals.var_rho_dn4 = 0.0;
        locals.var_rho_dn5 = 0.0;
        locals.var_rho_dn6 = 0.0;
        locals.var_rho_dn7 = 0.0;
        locals.var_rho_dn8 = 0.0;
        locals.var_rho_dn9 = 0.0;
        locals.var_rho_dn10 = 0.0;
        locals.var_rho_dn11 = 0.0;
        locals.var_rho_dn14 = 0.0;

        locals.var_vth = 0.0;

        locals.var_vth0 = 0.0;
        locals.var_vth0_dn0 = 0.0;
        locals.var_vth0_dn2 = 0.0;
        locals.var_vth0_dn4 = 0.0;
        locals.var_vth0_dn5 = 0.0;
        locals.var_vth0_dn6 = 0.0;
        locals.var_vth0_dn7 = 0.0;
        locals.var_vth0_dn8 = 0.0;
        locals.var_vth0_dn9 = 0.0;
        locals.var_vth0_dn10 = 0.0;
        locals.var_vth0_dn11 = 0.0;
        locals.var_vth0_dn14 = 0.0;

        locals.var_dvth = 0.0;
        locals.var_dvth_dn0 = 0.0;
        locals.var_dvth_dn2 = 0.0;
        locals.var_dvth_dn4 = 0.0;
        locals.var_dvth_dn5 = 0.0;
        locals.var_dvth_dn6 = 0.0;
        locals.var_dvth_dn7 = 0.0;
        locals.var_dvth_dn8 = 0.0;
        locals.var_dvth_dn9 = 0.0;
        locals.var_dvth_dn10 = 0.0;
        locals.var_dvth_dn11 = 0.0;
        locals.var_dvth_dn14 = 0.0;

        locals.var_dvth0 = 0.0;
        locals.var_dvth0_dn0 = 0.0;
        locals.var_dvth0_dn2 = 0.0;
        locals.var_dvth0_dn4 = 0.0;
        locals.var_dvth0_dn5 = 0.0;
        locals.var_dvth0_dn6 = 0.0;
        locals.var_dvth0_dn7 = 0.0;
        locals.var_dvth0_dn8 = 0.0;
        locals.var_dvth0_dn9 = 0.0;
        locals.var_dvth0_dn10 = 0.0;
        locals.var_dvth0_dn11 = 0.0;
        locals.var_dvth0_dn14 = 0.0;

        locals.var_dvthsc = 0.0;
        locals.var_dvthsc_dn0 = 0.0;
        locals.var_dvthsc_dn2 = 0.0;
        locals.var_dvthsc_dn4 = 0.0;
        locals.var_dvthsc_dn5 = 0.0;
        locals.var_dvthsc_dn6 = 0.0;
        locals.var_dvthsc_dn7 = 0.0;
        locals.var_dvthsc_dn8 = 0.0;
        locals.var_dvthsc_dn9 = 0.0;
        locals.var_dvthsc_dn10 = 0.0;
        locals.var_dvthsc_dn11 = 0.0;
        locals.var_dvthsc_dn14 = 0.0;

        locals.var_pb20b = 0.0;
        locals.var_pb20b_dn0 = 0.0;
        locals.var_pb20b_dn2 = 0.0;
        locals.var_pb20b_dn4 = 0.0;
        locals.var_pb20b_dn5 = 0.0;
        locals.var_pb20b_dn6 = 0.0;
        locals.var_pb20b_dn7 = 0.0;
        locals.var_pb20b_dn8 = 0.0;
        locals.var_pb20b_dn9 = 0.0;
        locals.var_pb20b_dn10 = 0.0;
        locals.var_pb20b_dn11 = 0.0;
        locals.var_pb20b_dn14 = 0.0;

        locals.var_dvthw = 0.0;
        locals.var_dvthw_dn0 = 0.0;
        locals.var_dvthw_dn2 = 0.0;
        locals.var_dvthw_dn4 = 0.0;
        locals.var_dvthw_dn5 = 0.0;
        locals.var_dvthw_dn6 = 0.0;
        locals.var_dvthw_dn7 = 0.0;
        locals.var_dvthw_dn8 = 0.0;
        locals.var_dvthw_dn9 = 0.0;
        locals.var_dvthw_dn10 = 0.0;
        locals.var_dvthw_dn11 = 0.0;
        locals.var_dvthw_dn14 = 0.0;

        locals.var_alpha = 0.0;
        locals.var_alpha_dn0 = 0.0;
        locals.var_alpha_dn2 = 0.0;
        locals.var_alpha_dn4 = 0.0;
        locals.var_alpha_dn5 = 0.0;
        locals.var_alpha_dn6 = 0.0;
        locals.var_alpha_dn7 = 0.0;
        locals.var_alpha_dn8 = 0.0;
        locals.var_alpha_dn9 = 0.0;
        locals.var_alpha_dn10 = 0.0;
        locals.var_alpha_dn11 = 0.0;
        locals.var_alpha_dn14 = 0.0;

        locals.var_achi = 0.0;
        locals.var_achi_dn0 = 0.0;
        locals.var_achi_dn2 = 0.0;
        locals.var_achi_dn4 = 0.0;
        locals.var_achi_dn5 = 0.0;
        locals.var_achi_dn6 = 0.0;
        locals.var_achi_dn7 = 0.0;
        locals.var_achi_dn8 = 0.0;
        locals.var_achi_dn9 = 0.0;
        locals.var_achi_dn10 = 0.0;
        locals.var_achi_dn11 = 0.0;
        locals.var_achi_dn14 = 0.0;

        locals.var_vgvt = 0.0;
        locals.var_vgvt_dn0 = 0.0;
        locals.var_vgvt_dn2 = 0.0;
        locals.var_vgvt_dn4 = 0.0;
        locals.var_vgvt_dn5 = 0.0;
        locals.var_vgvt_dn6 = 0.0;
        locals.var_vgvt_dn7 = 0.0;
        locals.var_vgvt_dn8 = 0.0;
        locals.var_vgvt_dn9 = 0.0;
        locals.var_vgvt_dn10 = 0.0;
        locals.var_vgvt_dn11 = 0.0;
        locals.var_vgvt_dn14 = 0.0;

        locals.var_pslsat = 0.0;
        locals.var_pslsat_dn0 = 0.0;
        locals.var_pslsat_dn2 = 0.0;
        locals.var_pslsat_dn4 = 0.0;
        locals.var_pslsat_dn5 = 0.0;
        locals.var_pslsat_dn6 = 0.0;
        locals.var_pslsat_dn7 = 0.0;
        locals.var_pslsat_dn8 = 0.0;
        locals.var_pslsat_dn9 = 0.0;
        locals.var_pslsat_dn10 = 0.0;
        locals.var_pslsat_dn11 = 0.0;
        locals.var_pslsat_dn14 = 0.0;

        locals.var_vdsats = 0.0;
        locals.var_vdsats_dn0 = 0.0;
        locals.var_vdsats_dn2 = 0.0;
        locals.var_vdsats_dn4 = 0.0;
        locals.var_vdsats_dn5 = 0.0;
        locals.var_vdsats_dn6 = 0.0;
        locals.var_vdsats_dn7 = 0.0;
        locals.var_vdsats_dn8 = 0.0;
        locals.var_vdsats_dn9 = 0.0;
        locals.var_vdsats_dn10 = 0.0;
        locals.var_vdsats_dn11 = 0.0;
        locals.var_vdsats_dn14 = 0.0;

        locals.var_delta = 0.0;
        locals.var_delta_dn0 = 0.0;
        locals.var_delta_dn2 = 0.0;
        locals.var_delta_dn4 = 0.0;
        locals.var_delta_dn5 = 0.0;
        locals.var_delta_dn6 = 0.0;
        locals.var_delta_dn7 = 0.0;
        locals.var_delta_dn8 = 0.0;
        locals.var_delta_dn9 = 0.0;
        locals.var_delta_dn10 = 0.0;
        locals.var_delta_dn11 = 0.0;
        locals.var_delta_dn14 = 0.0;

        locals.var_qb = 0.0;
        locals.var_qb_dn0 = 0.0;
        locals.var_qb_dn2 = 0.0;
        locals.var_qb_dn4 = 0.0;
        locals.var_qb_dn5 = 0.0;
        locals.var_qb_dn6 = 0.0;
        locals.var_qb_dn7 = 0.0;
        locals.var_qb_dn8 = 0.0;
        locals.var_qb_dn9 = 0.0;
        locals.var_qb_dn10 = 0.0;
        locals.var_qb_dn11 = 0.0;
        locals.var_qb_dn14 = 0.0;

        locals.var_qbu = 0.0;
        locals.var_qbu_dn0 = 0.0;
        locals.var_qbu_dn2 = 0.0;
        locals.var_qbu_dn4 = 0.0;
        locals.var_qbu_dn5 = 0.0;
        locals.var_qbu_dn6 = 0.0;
        locals.var_qbu_dn7 = 0.0;
        locals.var_qbu_dn8 = 0.0;
        locals.var_qbu_dn9 = 0.0;
        locals.var_qbu_dn10 = 0.0;
        locals.var_qbu_dn11 = 0.0;
        locals.var_qbu_dn14 = 0.0;

        locals.var_qi = 0.0;
        locals.var_qi_dn0 = 0.0;
        locals.var_qi_dn2 = 0.0;
        locals.var_qi_dn4 = 0.0;
        locals.var_qi_dn5 = 0.0;
        locals.var_qi_dn6 = 0.0;
        locals.var_qi_dn7 = 0.0;
        locals.var_qi_dn8 = 0.0;
        locals.var_qi_dn9 = 0.0;
        locals.var_qi_dn10 = 0.0;
        locals.var_qi_dn11 = 0.0;
        locals.var_qi_dn14 = 0.0;

        locals.var_qiu = 0.0;
        locals.var_qiu_dn0 = 0.0;
        locals.var_qiu_dn2 = 0.0;
        locals.var_qiu_dn4 = 0.0;
        locals.var_qiu_dn5 = 0.0;
        locals.var_qiu_dn6 = 0.0;
        locals.var_qiu_dn7 = 0.0;
        locals.var_qiu_dn8 = 0.0;
        locals.var_qiu_dn9 = 0.0;
        locals.var_qiu_dn10 = 0.0;
        locals.var_qiu_dn11 = 0.0;
        locals.var_qiu_dn14 = 0.0;

        locals.var_qd = 0.0;
        locals.var_qd_dn0 = 0.0;
        locals.var_qd_dn2 = 0.0;
        locals.var_qd_dn4 = 0.0;
        locals.var_qd_dn5 = 0.0;
        locals.var_qd_dn6 = 0.0;
        locals.var_qd_dn7 = 0.0;
        locals.var_qd_dn8 = 0.0;
        locals.var_qd_dn9 = 0.0;
        locals.var_qd_dn10 = 0.0;
        locals.var_qd_dn11 = 0.0;
        locals.var_qd_dn14 = 0.0;

        locals.var_ids = 0.0;
        locals.var_ids_dn0 = 0.0;
        locals.var_ids_dn2 = 0.0;
        locals.var_ids_dn4 = 0.0;
        locals.var_ids_dn5 = 0.0;
        locals.var_ids_dn6 = 0.0;
        locals.var_ids_dn7 = 0.0;
        locals.var_ids_dn8 = 0.0;
        locals.var_ids_dn9 = 0.0;
        locals.var_ids_dn10 = 0.0;
        locals.var_ids_dn11 = 0.0;
        locals.var_ids_dn14 = 0.0;

        locals.var_ids0 = 0.0;
        locals.var_ids0_dn0 = 0.0;
        locals.var_ids0_dn2 = 0.0;
        locals.var_ids0_dn4 = 0.0;
        locals.var_ids0_dn5 = 0.0;
        locals.var_ids0_dn6 = 0.0;
        locals.var_ids0_dn7 = 0.0;
        locals.var_ids0_dn8 = 0.0;
        locals.var_ids0_dn9 = 0.0;
        locals.var_ids0_dn10 = 0.0;
        locals.var_ids0_dn11 = 0.0;
        locals.var_ids0_dn14 = 0.0;

        locals.var_dvthscsti = 0.0;
        locals.var_dvthscsti_dn0 = 0.0;
        locals.var_dvthscsti_dn2 = 0.0;
        locals.var_dvthscsti_dn4 = 0.0;
        locals.var_dvthscsti_dn5 = 0.0;
        locals.var_dvthscsti_dn6 = 0.0;
        locals.var_dvthscsti_dn7 = 0.0;
        locals.var_dvthscsti_dn8 = 0.0;
        locals.var_dvthscsti_dn9 = 0.0;
        locals.var_dvthscsti_dn10 = 0.0;
        locals.var_dvthscsti_dn11 = 0.0;
        locals.var_dvthscsti_dn14 = 0.0;

        locals.var_vgssti = 0.0;
        locals.var_vgssti_dn0 = 0.0;
        locals.var_vgssti_dn2 = 0.0;
        locals.var_vgssti_dn4 = 0.0;
        locals.var_vgssti_dn5 = 0.0;
        locals.var_vgssti_dn6 = 0.0;
        locals.var_vgssti_dn7 = 0.0;
        locals.var_vgssti_dn8 = 0.0;
        locals.var_vgssti_dn9 = 0.0;
        locals.var_vgssti_dn10 = 0.0;
        locals.var_vgssti_dn11 = 0.0;
        locals.var_vgssti_dn14 = 0.0;

        locals.var_costi0 = 0.0;
        locals.var_costi0_dn0 = 0.0;
        locals.var_costi0_dn2 = 0.0;
        locals.var_costi0_dn4 = 0.0;
        locals.var_costi0_dn5 = 0.0;
        locals.var_costi0_dn6 = 0.0;
        locals.var_costi0_dn7 = 0.0;
        locals.var_costi0_dn8 = 0.0;
        locals.var_costi0_dn9 = 0.0;
        locals.var_costi0_dn10 = 0.0;
        locals.var_costi0_dn11 = 0.0;
        locals.var_costi0_dn14 = 0.0;

        locals.var_costi1 = 0.0;
        locals.var_costi1_dn0 = 0.0;
        locals.var_costi1_dn2 = 0.0;
        locals.var_costi1_dn4 = 0.0;
        locals.var_costi1_dn5 = 0.0;
        locals.var_costi1_dn6 = 0.0;
        locals.var_costi1_dn7 = 0.0;
        locals.var_costi1_dn8 = 0.0;
        locals.var_costi1_dn9 = 0.0;
        locals.var_costi1_dn10 = 0.0;
        locals.var_costi1_dn11 = 0.0;
        locals.var_costi1_dn14 = 0.0;

        locals.var_costi3 = 0.0;
        locals.var_costi3_dn0 = 0.0;
        locals.var_costi3_dn2 = 0.0;
        locals.var_costi3_dn4 = 0.0;
        locals.var_costi3_dn5 = 0.0;
        locals.var_costi3_dn6 = 0.0;
        locals.var_costi3_dn7 = 0.0;
        locals.var_costi3_dn8 = 0.0;
        locals.var_costi3_dn9 = 0.0;
        locals.var_costi3_dn10 = 0.0;
        locals.var_costi3_dn11 = 0.0;
        locals.var_costi3_dn14 = 0.0;

        locals.var_costi4 = 0.0;
        locals.var_costi4_dn0 = 0.0;
        locals.var_costi4_dn2 = 0.0;
        locals.var_costi4_dn4 = 0.0;
        locals.var_costi4_dn5 = 0.0;
        locals.var_costi4_dn6 = 0.0;
        locals.var_costi4_dn7 = 0.0;
        locals.var_costi4_dn8 = 0.0;
        locals.var_costi4_dn9 = 0.0;
        locals.var_costi4_dn10 = 0.0;
        locals.var_costi4_dn11 = 0.0;
        locals.var_costi4_dn14 = 0.0;

        locals.var_costi5 = 0.0;
        locals.var_costi5_dn0 = 0.0;
        locals.var_costi5_dn2 = 0.0;
        locals.var_costi5_dn4 = 0.0;
        locals.var_costi5_dn5 = 0.0;
        locals.var_costi5_dn6 = 0.0;
        locals.var_costi5_dn7 = 0.0;
        locals.var_costi5_dn8 = 0.0;
        locals.var_costi5_dn9 = 0.0;
        locals.var_costi5_dn10 = 0.0;
        locals.var_costi5_dn11 = 0.0;
        locals.var_costi5_dn14 = 0.0;

        locals.var_costi6 = 0.0;
        locals.var_costi6_dn0 = 0.0;
        locals.var_costi6_dn2 = 0.0;
        locals.var_costi6_dn4 = 0.0;
        locals.var_costi6_dn5 = 0.0;
        locals.var_costi6_dn6 = 0.0;
        locals.var_costi6_dn7 = 0.0;
        locals.var_costi6_dn8 = 0.0;
        locals.var_costi6_dn9 = 0.0;
        locals.var_costi6_dn10 = 0.0;
        locals.var_costi6_dn11 = 0.0;
        locals.var_costi6_dn14 = 0.0;

        locals.var_costi7 = 0.0;
        locals.var_costi7_dn0 = 0.0;
        locals.var_costi7_dn2 = 0.0;
        locals.var_costi7_dn4 = 0.0;
        locals.var_costi7_dn5 = 0.0;
        locals.var_costi7_dn6 = 0.0;
        locals.var_costi7_dn7 = 0.0;
        locals.var_costi7_dn8 = 0.0;
        locals.var_costi7_dn9 = 0.0;
        locals.var_costi7_dn10 = 0.0;
        locals.var_costi7_dn11 = 0.0;
        locals.var_costi7_dn14 = 0.0;

        locals.var_psasti = 0.0;
        locals.var_psasti_dn0 = 0.0;
        locals.var_psasti_dn2 = 0.0;
        locals.var_psasti_dn4 = 0.0;
        locals.var_psasti_dn5 = 0.0;
        locals.var_psasti_dn6 = 0.0;
        locals.var_psasti_dn7 = 0.0;
        locals.var_psasti_dn8 = 0.0;
        locals.var_psasti_dn9 = 0.0;
        locals.var_psasti_dn10 = 0.0;
        locals.var_psasti_dn11 = 0.0;
        locals.var_psasti_dn14 = 0.0;

        locals.var_psbsti = 0.0;
        locals.var_psbsti_dn0 = 0.0;
        locals.var_psbsti_dn2 = 0.0;
        locals.var_psbsti_dn4 = 0.0;
        locals.var_psbsti_dn5 = 0.0;
        locals.var_psbsti_dn6 = 0.0;
        locals.var_psbsti_dn7 = 0.0;
        locals.var_psbsti_dn8 = 0.0;
        locals.var_psbsti_dn9 = 0.0;
        locals.var_psbsti_dn10 = 0.0;
        locals.var_psbsti_dn11 = 0.0;
        locals.var_psbsti_dn14 = 0.0;

        locals.var_psab = 0.0;
        locals.var_psab_dn0 = 0.0;
        locals.var_psab_dn2 = 0.0;
        locals.var_psab_dn4 = 0.0;
        locals.var_psab_dn5 = 0.0;
        locals.var_psab_dn6 = 0.0;
        locals.var_psab_dn7 = 0.0;
        locals.var_psab_dn8 = 0.0;
        locals.var_psab_dn9 = 0.0;
        locals.var_psab_dn10 = 0.0;
        locals.var_psab_dn11 = 0.0;
        locals.var_psab_dn14 = 0.0;

        locals.var_psti = 0.0;
        locals.var_psti_dn0 = 0.0;
        locals.var_psti_dn2 = 0.0;
        locals.var_psti_dn4 = 0.0;
        locals.var_psti_dn5 = 0.0;
        locals.var_psti_dn6 = 0.0;
        locals.var_psti_dn7 = 0.0;
        locals.var_psti_dn8 = 0.0;
        locals.var_psti_dn9 = 0.0;
        locals.var_psti_dn10 = 0.0;
        locals.var_psti_dn11 = 0.0;
        locals.var_psti_dn14 = 0.0;

        locals.var_sq1sti = 0.0;
        locals.var_sq1sti_dn0 = 0.0;
        locals.var_sq1sti_dn2 = 0.0;
        locals.var_sq1sti_dn4 = 0.0;
        locals.var_sq1sti_dn5 = 0.0;
        locals.var_sq1sti_dn6 = 0.0;
        locals.var_sq1sti_dn7 = 0.0;
        locals.var_sq1sti_dn8 = 0.0;
        locals.var_sq1sti_dn9 = 0.0;
        locals.var_sq1sti_dn10 = 0.0;
        locals.var_sq1sti_dn11 = 0.0;
        locals.var_sq1sti_dn14 = 0.0;

        locals.var_sq2sti = 0.0;
        locals.var_sq2sti_dn0 = 0.0;
        locals.var_sq2sti_dn2 = 0.0;
        locals.var_sq2sti_dn4 = 0.0;
        locals.var_sq2sti_dn5 = 0.0;
        locals.var_sq2sti_dn6 = 0.0;
        locals.var_sq2sti_dn7 = 0.0;
        locals.var_sq2sti_dn8 = 0.0;
        locals.var_sq2sti_dn9 = 0.0;
        locals.var_sq2sti_dn10 = 0.0;
        locals.var_sq2sti_dn11 = 0.0;
        locals.var_sq2sti_dn14 = 0.0;

        locals.var_qn0sti = 0.0;
        locals.var_qn0sti_dn0 = 0.0;
        locals.var_qn0sti_dn2 = 0.0;
        locals.var_qn0sti_dn4 = 0.0;
        locals.var_qn0sti_dn5 = 0.0;
        locals.var_qn0sti_dn6 = 0.0;
        locals.var_qn0sti_dn7 = 0.0;
        locals.var_qn0sti_dn8 = 0.0;
        locals.var_qn0sti_dn9 = 0.0;
        locals.var_qn0sti_dn10 = 0.0;
        locals.var_qn0sti_dn11 = 0.0;
        locals.var_qn0sti_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_2(
        locals: &mut StampLocals,
    ) {
        locals.var_idssti = 0.0;
        locals.var_idssti_dn0 = 0.0;
        locals.var_idssti_dn2 = 0.0;
        locals.var_idssti_dn4 = 0.0;
        locals.var_idssti_dn5 = 0.0;
        locals.var_idssti_dn6 = 0.0;
        locals.var_idssti_dn7 = 0.0;
        locals.var_idssti_dn8 = 0.0;
        locals.var_idssti_dn9 = 0.0;
        locals.var_idssti_dn10 = 0.0;
        locals.var_idssti_dn11 = 0.0;
        locals.var_idssti_dn14 = 0.0;

        locals.var_beta = 0.0;
        locals.var_beta_dn0 = 0.0;
        locals.var_beta_dn2 = 0.0;
        locals.var_beta_dn4 = 0.0;
        locals.var_beta_dn5 = 0.0;
        locals.var_beta_dn6 = 0.0;
        locals.var_beta_dn7 = 0.0;
        locals.var_beta_dn8 = 0.0;
        locals.var_beta_dn9 = 0.0;
        locals.var_beta_dn10 = 0.0;
        locals.var_beta_dn11 = 0.0;
        locals.var_beta_dn14 = 0.0;

        locals.var_beta_inv = 0.0;
        locals.var_beta_inv_dn0 = 0.0;
        locals.var_beta_inv_dn2 = 0.0;
        locals.var_beta_inv_dn4 = 0.0;
        locals.var_beta_inv_dn5 = 0.0;
        locals.var_beta_inv_dn6 = 0.0;
        locals.var_beta_inv_dn7 = 0.0;
        locals.var_beta_inv_dn8 = 0.0;
        locals.var_beta_inv_dn9 = 0.0;
        locals.var_beta_inv_dn10 = 0.0;
        locals.var_beta_inv_dn11 = 0.0;
        locals.var_beta_inv_dn14 = 0.0;

        locals.var_beta2 = 0.0;
        locals.var_beta2_dn0 = 0.0;
        locals.var_beta2_dn2 = 0.0;
        locals.var_beta2_dn4 = 0.0;
        locals.var_beta2_dn5 = 0.0;
        locals.var_beta2_dn6 = 0.0;
        locals.var_beta2_dn7 = 0.0;
        locals.var_beta2_dn8 = 0.0;
        locals.var_beta2_dn9 = 0.0;
        locals.var_beta2_dn10 = 0.0;
        locals.var_beta2_dn11 = 0.0;
        locals.var_beta2_dn14 = 0.0;

        locals.var_pb2 = 0.0;
        locals.var_pb2_dn0 = 0.0;
        locals.var_pb2_dn2 = 0.0;
        locals.var_pb2_dn4 = 0.0;
        locals.var_pb2_dn5 = 0.0;
        locals.var_pb2_dn6 = 0.0;
        locals.var_pb2_dn7 = 0.0;
        locals.var_pb2_dn8 = 0.0;
        locals.var_pb2_dn9 = 0.0;
        locals.var_pb2_dn10 = 0.0;
        locals.var_pb2_dn11 = 0.0;
        locals.var_pb2_dn14 = 0.0;

        locals.var_pb20 = 0.0;
        locals.var_pb20_dn0 = 0.0;
        locals.var_pb20_dn2 = 0.0;
        locals.var_pb20_dn4 = 0.0;
        locals.var_pb20_dn5 = 0.0;
        locals.var_pb20_dn6 = 0.0;
        locals.var_pb20_dn7 = 0.0;
        locals.var_pb20_dn8 = 0.0;
        locals.var_pb20_dn9 = 0.0;
        locals.var_pb20_dn10 = 0.0;
        locals.var_pb20_dn11 = 0.0;
        locals.var_pb20_dn14 = 0.0;

        locals.var_pb2c = 0.0;
        locals.var_pb2c_dn0 = 0.0;
        locals.var_pb2c_dn2 = 0.0;
        locals.var_pb2c_dn4 = 0.0;
        locals.var_pb2c_dn5 = 0.0;
        locals.var_pb2c_dn6 = 0.0;
        locals.var_pb2c_dn7 = 0.0;
        locals.var_pb2c_dn8 = 0.0;
        locals.var_pb2c_dn9 = 0.0;
        locals.var_pb2c_dn10 = 0.0;
        locals.var_pb2c_dn11 = 0.0;
        locals.var_pb2c_dn14 = 0.0;

        locals.var_vfb = 0.0;

        locals.var_c_eox = 0.0;

        locals.var_leff = 0.0;

        locals.var_weff = 0.0;

        locals.var_weffld_nf = 0.0;

        locals.var_ldrift0 = 0.0;

        locals.var_q_nsub = 0.0;
        locals.var_q_nsub_dn0 = 0.0;
        locals.var_q_nsub_dn2 = 0.0;
        locals.var_q_nsub_dn4 = 0.0;
        locals.var_q_nsub_dn5 = 0.0;
        locals.var_q_nsub_dn6 = 0.0;
        locals.var_q_nsub_dn7 = 0.0;
        locals.var_q_nsub_dn8 = 0.0;
        locals.var_q_nsub_dn9 = 0.0;
        locals.var_q_nsub_dn10 = 0.0;
        locals.var_q_nsub_dn11 = 0.0;
        locals.var_q_nsub_dn14 = 0.0;

        locals.var_psa = 0.0;
        locals.var_psa_dn0 = 0.0;
        locals.var_psa_dn2 = 0.0;
        locals.var_psa_dn4 = 0.0;
        locals.var_psa_dn5 = 0.0;
        locals.var_psa_dn6 = 0.0;
        locals.var_psa_dn7 = 0.0;
        locals.var_psa_dn8 = 0.0;
        locals.var_psa_dn9 = 0.0;
        locals.var_psa_dn10 = 0.0;
        locals.var_psa_dn11 = 0.0;
        locals.var_psa_dn14 = 0.0;

        locals.var_psdl = 0.0;
        locals.var_psdl_dn0 = 0.0;
        locals.var_psdl_dn2 = 0.0;
        locals.var_psdl_dn4 = 0.0;
        locals.var_psdl_dn5 = 0.0;
        locals.var_psdl_dn6 = 0.0;
        locals.var_psdl_dn7 = 0.0;
        locals.var_psdl_dn8 = 0.0;
        locals.var_psdl_dn9 = 0.0;
        locals.var_psdl_dn10 = 0.0;
        locals.var_psdl_dn11 = 0.0;
        locals.var_psdl_dn14 = 0.0;

        locals.var_lred = 0.0;
        locals.var_lred_dn0 = 0.0;
        locals.var_lred_dn2 = 0.0;
        locals.var_lred_dn4 = 0.0;
        locals.var_lred_dn5 = 0.0;
        locals.var_lred_dn6 = 0.0;
        locals.var_lred_dn7 = 0.0;
        locals.var_lred_dn8 = 0.0;
        locals.var_lred_dn9 = 0.0;
        locals.var_lred_dn10 = 0.0;
        locals.var_lred_dn11 = 0.0;
        locals.var_lred_dn14 = 0.0;

        locals.var_lch = 0.0;
        locals.var_lch_dn0 = 0.0;
        locals.var_lch_dn2 = 0.0;
        locals.var_lch_dn4 = 0.0;
        locals.var_lch_dn5 = 0.0;
        locals.var_lch_dn6 = 0.0;
        locals.var_lch_dn7 = 0.0;
        locals.var_lch_dn8 = 0.0;
        locals.var_lch_dn9 = 0.0;
        locals.var_lch_dn10 = 0.0;
        locals.var_lch_dn11 = 0.0;
        locals.var_lch_dn14 = 0.0;

        locals.var_wd = 0.0;
        locals.var_wd_dn0 = 0.0;
        locals.var_wd_dn2 = 0.0;
        locals.var_wd_dn4 = 0.0;
        locals.var_wd_dn5 = 0.0;
        locals.var_wd_dn6 = 0.0;
        locals.var_wd_dn7 = 0.0;
        locals.var_wd_dn8 = 0.0;
        locals.var_wd_dn9 = 0.0;
        locals.var_wd_dn10 = 0.0;
        locals.var_wd_dn11 = 0.0;
        locals.var_wd_dn14 = 0.0;

        locals.var_aclm = 0.0;

        locals.var_vthp = 0.0;
        locals.var_vthp_dn0 = 0.0;
        locals.var_vthp_dn2 = 0.0;
        locals.var_vthp_dn4 = 0.0;
        locals.var_vthp_dn5 = 0.0;
        locals.var_vthp_dn6 = 0.0;
        locals.var_vthp_dn7 = 0.0;
        locals.var_vthp_dn8 = 0.0;
        locals.var_vthp_dn9 = 0.0;
        locals.var_vthp_dn10 = 0.0;
        locals.var_vthp_dn11 = 0.0;
        locals.var_vthp_dn14 = 0.0;

        locals.var_dvthlp = 0.0;
        locals.var_dvthlp_dn0 = 0.0;
        locals.var_dvthlp_dn2 = 0.0;
        locals.var_dvthlp_dn4 = 0.0;
        locals.var_dvthlp_dn5 = 0.0;
        locals.var_dvthlp_dn6 = 0.0;
        locals.var_dvthlp_dn7 = 0.0;
        locals.var_dvthlp_dn8 = 0.0;
        locals.var_dvthlp_dn9 = 0.0;
        locals.var_dvthlp_dn10 = 0.0;
        locals.var_dvthlp_dn11 = 0.0;
        locals.var_dvthlp_dn14 = 0.0;

        locals.var_bs12 = 0.0;
        locals.var_bs12_dn0 = 0.0;
        locals.var_bs12_dn2 = 0.0;
        locals.var_bs12_dn4 = 0.0;
        locals.var_bs12_dn5 = 0.0;
        locals.var_bs12_dn6 = 0.0;
        locals.var_bs12_dn7 = 0.0;
        locals.var_bs12_dn8 = 0.0;
        locals.var_bs12_dn9 = 0.0;
        locals.var_bs12_dn10 = 0.0;
        locals.var_bs12_dn11 = 0.0;
        locals.var_bs12_dn14 = 0.0;

        locals.var_qbmm = 0.0;
        locals.var_qbmm_dn0 = 0.0;
        locals.var_qbmm_dn2 = 0.0;
        locals.var_qbmm_dn4 = 0.0;
        locals.var_qbmm_dn5 = 0.0;
        locals.var_qbmm_dn6 = 0.0;
        locals.var_qbmm_dn7 = 0.0;
        locals.var_qbmm_dn8 = 0.0;
        locals.var_qbmm_dn9 = 0.0;
        locals.var_qbmm_dn10 = 0.0;
        locals.var_qbmm_dn11 = 0.0;
        locals.var_qbmm_dn14 = 0.0;

        locals.var_dqb = 0.0;
        locals.var_dqb_dn0 = 0.0;
        locals.var_dqb_dn2 = 0.0;
        locals.var_dqb_dn4 = 0.0;
        locals.var_dqb_dn5 = 0.0;
        locals.var_dqb_dn6 = 0.0;
        locals.var_dqb_dn7 = 0.0;
        locals.var_dqb_dn8 = 0.0;
        locals.var_dqb_dn9 = 0.0;
        locals.var_dqb_dn10 = 0.0;
        locals.var_dqb_dn11 = 0.0;
        locals.var_dqb_dn14 = 0.0;

        locals.var_vdx = 0.0;
        locals.var_vdx_dn0 = 0.0;
        locals.var_vdx_dn2 = 0.0;
        locals.var_vdx_dn4 = 0.0;
        locals.var_vdx_dn5 = 0.0;
        locals.var_vdx_dn6 = 0.0;
        locals.var_vdx_dn7 = 0.0;
        locals.var_vdx_dn8 = 0.0;
        locals.var_vdx_dn9 = 0.0;
        locals.var_vdx_dn10 = 0.0;
        locals.var_vdx_dn11 = 0.0;
        locals.var_vdx_dn14 = 0.0;

        locals.var_vdx2 = 0.0;
        locals.var_vdx2_dn0 = 0.0;
        locals.var_vdx2_dn2 = 0.0;
        locals.var_vdx2_dn4 = 0.0;
        locals.var_vdx2_dn5 = 0.0;
        locals.var_vdx2_dn6 = 0.0;
        locals.var_vdx2_dn7 = 0.0;
        locals.var_vdx2_dn8 = 0.0;
        locals.var_vdx2_dn9 = 0.0;
        locals.var_vdx2_dn10 = 0.0;
        locals.var_vdx2_dn11 = 0.0;
        locals.var_vdx2_dn14 = 0.0;

        locals.var_pbsum = 0.0;
        locals.var_pbsum_dn0 = 0.0;
        locals.var_pbsum_dn2 = 0.0;
        locals.var_pbsum_dn4 = 0.0;
        locals.var_pbsum_dn5 = 0.0;
        locals.var_pbsum_dn6 = 0.0;
        locals.var_pbsum_dn7 = 0.0;
        locals.var_pbsum_dn8 = 0.0;
        locals.var_pbsum_dn9 = 0.0;
        locals.var_pbsum_dn10 = 0.0;
        locals.var_pbsum_dn11 = 0.0;
        locals.var_pbsum_dn14 = 0.0;

        locals.var_sqrt_pbsum = 0.0;
        locals.var_sqrt_pbsum_dn0 = 0.0;
        locals.var_sqrt_pbsum_dn2 = 0.0;
        locals.var_sqrt_pbsum_dn4 = 0.0;
        locals.var_sqrt_pbsum_dn5 = 0.0;
        locals.var_sqrt_pbsum_dn6 = 0.0;
        locals.var_sqrt_pbsum_dn7 = 0.0;
        locals.var_sqrt_pbsum_dn8 = 0.0;
        locals.var_sqrt_pbsum_dn9 = 0.0;
        locals.var_sqrt_pbsum_dn10 = 0.0;
        locals.var_sqrt_pbsum_dn11 = 0.0;
        locals.var_sqrt_pbsum_dn14 = 0.0;

        locals.var_dppg = 0.0;
        locals.var_dppg_dn0 = 0.0;
        locals.var_dppg_dn2 = 0.0;
        locals.var_dppg_dn4 = 0.0;
        locals.var_dppg_dn5 = 0.0;
        locals.var_dppg_dn6 = 0.0;
        locals.var_dppg_dn7 = 0.0;
        locals.var_dppg_dn8 = 0.0;
        locals.var_dppg_dn9 = 0.0;
        locals.var_dppg_dn10 = 0.0;
        locals.var_dppg_dn11 = 0.0;
        locals.var_dppg_dn14 = 0.0;

        locals.var_dtox = 0.0;
        locals.var_dtox_dn0 = 0.0;
        locals.var_dtox_dn2 = 0.0;
        locals.var_dtox_dn4 = 0.0;
        locals.var_dtox_dn5 = 0.0;
        locals.var_dtox_dn6 = 0.0;
        locals.var_dtox_dn7 = 0.0;
        locals.var_dtox_dn8 = 0.0;
        locals.var_dtox_dn9 = 0.0;
        locals.var_dtox_dn10 = 0.0;
        locals.var_dtox_dn11 = 0.0;
        locals.var_dtox_dn14 = 0.0;

        locals.var_cox = 0.0;
        locals.var_cox_dn0 = 0.0;
        locals.var_cox_dn2 = 0.0;
        locals.var_cox_dn4 = 0.0;
        locals.var_cox_dn5 = 0.0;
        locals.var_cox_dn6 = 0.0;
        locals.var_cox_dn7 = 0.0;
        locals.var_cox_dn8 = 0.0;
        locals.var_cox_dn9 = 0.0;
        locals.var_cox_dn10 = 0.0;
        locals.var_cox_dn11 = 0.0;
        locals.var_cox_dn14 = 0.0;

        locals.var_cox_inv = 0.0;
        locals.var_cox_inv_dn0 = 0.0;
        locals.var_cox_inv_dn2 = 0.0;
        locals.var_cox_inv_dn4 = 0.0;
        locals.var_cox_inv_dn5 = 0.0;
        locals.var_cox_inv_dn6 = 0.0;
        locals.var_cox_inv_dn7 = 0.0;
        locals.var_cox_inv_dn8 = 0.0;
        locals.var_cox_inv_dn9 = 0.0;
        locals.var_cox_inv_dn10 = 0.0;
        locals.var_cox_inv_dn11 = 0.0;
        locals.var_cox_inv_dn14 = 0.0;

        locals.var_tox0 = 0.0;

        locals.var_cox0 = 0.0;

        locals.var_coxb0 = 0.0;

        locals.var_cox0_inv = 0.0;

        locals.var_vthq = 0.0;
        locals.var_vthq_dn0 = 0.0;
        locals.var_vthq_dn2 = 0.0;
        locals.var_vthq_dn4 = 0.0;
        locals.var_vthq_dn5 = 0.0;
        locals.var_vthq_dn6 = 0.0;
        locals.var_vthq_dn7 = 0.0;
        locals.var_vthq_dn8 = 0.0;
        locals.var_vthq_dn9 = 0.0;
        locals.var_vthq_dn10 = 0.0;
        locals.var_vthq_dn11 = 0.0;
        locals.var_vthq_dn14 = 0.0;

        locals.var_psdlz = 0.0;
        locals.var_psdlz_dn0 = 0.0;
        locals.var_psdlz_dn2 = 0.0;
        locals.var_psdlz_dn4 = 0.0;
        locals.var_psdlz_dn5 = 0.0;
        locals.var_psdlz_dn6 = 0.0;
        locals.var_psdlz_dn7 = 0.0;
        locals.var_psdlz_dn8 = 0.0;
        locals.var_psdlz_dn9 = 0.0;
        locals.var_psdlz_dn10 = 0.0;
        locals.var_psdlz_dn11 = 0.0;
        locals.var_psdlz_dn14 = 0.0;

        locals.var_egp12 = 0.0;
        locals.var_egp12_dn0 = 0.0;
        locals.var_egp12_dn2 = 0.0;
        locals.var_egp12_dn4 = 0.0;
        locals.var_egp12_dn5 = 0.0;
        locals.var_egp12_dn6 = 0.0;
        locals.var_egp12_dn7 = 0.0;
        locals.var_egp12_dn8 = 0.0;
        locals.var_egp12_dn9 = 0.0;
        locals.var_egp12_dn10 = 0.0;
        locals.var_egp12_dn11 = 0.0;
        locals.var_egp12_dn14 = 0.0;

        locals.var_egp32 = 0.0;
        locals.var_egp32_dn0 = 0.0;
        locals.var_egp32_dn2 = 0.0;
        locals.var_egp32_dn4 = 0.0;
        locals.var_egp32_dn5 = 0.0;
        locals.var_egp32_dn6 = 0.0;
        locals.var_egp32_dn7 = 0.0;
        locals.var_egp32_dn8 = 0.0;
        locals.var_egp32_dn9 = 0.0;
        locals.var_egp32_dn10 = 0.0;
        locals.var_egp32_dn11 = 0.0;
        locals.var_egp32_dn14 = 0.0;

        locals.var_e1 = 0.0;
        locals.var_e1_dn0 = 0.0;
        locals.var_e1_dn2 = 0.0;
        locals.var_e1_dn4 = 0.0;
        locals.var_e1_dn5 = 0.0;
        locals.var_e1_dn6 = 0.0;
        locals.var_e1_dn7 = 0.0;
        locals.var_e1_dn8 = 0.0;
        locals.var_e1_dn9 = 0.0;
        locals.var_e1_dn10 = 0.0;
        locals.var_e1_dn11 = 0.0;
        locals.var_e1_dn14 = 0.0;

        locals.var_etun = 0.0;
        locals.var_etun_dn0 = 0.0;
        locals.var_etun_dn2 = 0.0;
        locals.var_etun_dn4 = 0.0;
        locals.var_etun_dn5 = 0.0;
        locals.var_etun_dn6 = 0.0;
        locals.var_etun_dn7 = 0.0;
        locals.var_etun_dn8 = 0.0;
        locals.var_etun_dn9 = 0.0;
        locals.var_etun_dn10 = 0.0;
        locals.var_etun_dn11 = 0.0;
        locals.var_etun_dn14 = 0.0;

        locals.var_vdsp = 0.0;
        locals.var_vdsp_dn0 = 0.0;
        locals.var_vdsp_dn2 = 0.0;
        locals.var_vdsp_dn4 = 0.0;
        locals.var_vdsp_dn5 = 0.0;
        locals.var_vdsp_dn6 = 0.0;
        locals.var_vdsp_dn7 = 0.0;
        locals.var_vdsp_dn8 = 0.0;
        locals.var_vdsp_dn9 = 0.0;
        locals.var_vdsp_dn10 = 0.0;
        locals.var_vdsp_dn11 = 0.0;
        locals.var_vdsp_dn14 = 0.0;

        locals.var_egidl = 0.0;
        locals.var_egidl_dn0 = 0.0;
        locals.var_egidl_dn2 = 0.0;
        locals.var_egidl_dn4 = 0.0;
        locals.var_egidl_dn5 = 0.0;
        locals.var_egidl_dn6 = 0.0;
        locals.var_egidl_dn7 = 0.0;
        locals.var_egidl_dn8 = 0.0;
        locals.var_egidl_dn9 = 0.0;
        locals.var_egidl_dn10 = 0.0;
        locals.var_egidl_dn11 = 0.0;
        locals.var_egidl_dn14 = 0.0;

        locals.var_egisl = 0.0;
        locals.var_egisl_dn0 = 0.0;
        locals.var_egisl_dn2 = 0.0;
        locals.var_egisl_dn4 = 0.0;
        locals.var_egisl_dn5 = 0.0;
        locals.var_egisl_dn6 = 0.0;
        locals.var_egisl_dn7 = 0.0;
        locals.var_egisl_dn8 = 0.0;
        locals.var_egisl_dn9 = 0.0;
        locals.var_egisl_dn10 = 0.0;
        locals.var_egisl_dn11 = 0.0;
        locals.var_egisl_dn14 = 0.0;

        locals.var_igate = 0.0;
        locals.var_igate_dn0 = 0.0;
        locals.var_igate_dn2 = 0.0;
        locals.var_igate_dn4 = 0.0;
        locals.var_igate_dn5 = 0.0;
        locals.var_igate_dn6 = 0.0;
        locals.var_igate_dn7 = 0.0;
        locals.var_igate_dn8 = 0.0;
        locals.var_igate_dn9 = 0.0;
        locals.var_igate_dn10 = 0.0;
        locals.var_igate_dn11 = 0.0;
        locals.var_igate_dn14 = 0.0;

        locals.var_igs = 0.0;
        locals.var_igs_dn0 = 0.0;
        locals.var_igs_dn2 = 0.0;
        locals.var_igs_dn4 = 0.0;
        locals.var_igs_dn5 = 0.0;
        locals.var_igs_dn6 = 0.0;
        locals.var_igs_dn7 = 0.0;
        locals.var_igs_dn8 = 0.0;
        locals.var_igs_dn9 = 0.0;
        locals.var_igs_dn10 = 0.0;
        locals.var_igs_dn11 = 0.0;
        locals.var_igs_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_3(
        locals: &mut StampLocals,
    ) {
        locals.var_igd = 0.0;
        locals.var_igd_dn0 = 0.0;
        locals.var_igd_dn2 = 0.0;
        locals.var_igd_dn4 = 0.0;
        locals.var_igd_dn5 = 0.0;
        locals.var_igd_dn6 = 0.0;
        locals.var_igd_dn7 = 0.0;
        locals.var_igd_dn8 = 0.0;
        locals.var_igd_dn9 = 0.0;
        locals.var_igd_dn10 = 0.0;
        locals.var_igd_dn11 = 0.0;
        locals.var_igd_dn14 = 0.0;

        locals.var_igb = 0.0;
        locals.var_igb_dn0 = 0.0;
        locals.var_igb_dn2 = 0.0;
        locals.var_igb_dn4 = 0.0;
        locals.var_igb_dn5 = 0.0;
        locals.var_igb_dn6 = 0.0;
        locals.var_igb_dn7 = 0.0;
        locals.var_igb_dn8 = 0.0;
        locals.var_igb_dn9 = 0.0;
        locals.var_igb_dn10 = 0.0;
        locals.var_igb_dn11 = 0.0;
        locals.var_igb_dn14 = 0.0;

        locals.var_igidl = 0.0;
        locals.var_igidl_dn0 = 0.0;
        locals.var_igidl_dn2 = 0.0;
        locals.var_igidl_dn4 = 0.0;
        locals.var_igidl_dn5 = 0.0;
        locals.var_igidl_dn6 = 0.0;
        locals.var_igidl_dn7 = 0.0;
        locals.var_igidl_dn8 = 0.0;
        locals.var_igidl_dn9 = 0.0;
        locals.var_igidl_dn10 = 0.0;
        locals.var_igidl_dn11 = 0.0;
        locals.var_igidl_dn14 = 0.0;

        locals.var_igisl = 0.0;
        locals.var_igisl_dn0 = 0.0;
        locals.var_igisl_dn2 = 0.0;
        locals.var_igisl_dn4 = 0.0;
        locals.var_igisl_dn5 = 0.0;
        locals.var_igisl_dn6 = 0.0;
        locals.var_igisl_dn7 = 0.0;
        locals.var_igisl_dn8 = 0.0;
        locals.var_igisl_dn9 = 0.0;
        locals.var_igisl_dn10 = 0.0;
        locals.var_igisl_dn11 = 0.0;
        locals.var_igisl_dn14 = 0.0;

        locals.var_vdb = 0.0;
        locals.var_vdb_dn0 = 0.0;
        locals.var_vdb_dn2 = 0.0;
        locals.var_vdb_dn4 = 0.0;
        locals.var_vdb_dn5 = 0.0;
        locals.var_vdb_dn6 = 0.0;
        locals.var_vdb_dn7 = 0.0;
        locals.var_vdb_dn8 = 0.0;
        locals.var_vdb_dn9 = 0.0;
        locals.var_vdb_dn10 = 0.0;
        locals.var_vdb_dn11 = 0.0;
        locals.var_vdb_dn14 = 0.0;

        locals.var_vsb = 0.0;
        locals.var_vsb_dn6 = 0.0;
        locals.var_vsb_dn8 = 0.0;
        locals.var_vsb_dn9 = 0.0;

        locals.var_fd2 = 0.0;
        locals.var_fd2_dn0 = 0.0;
        locals.var_fd2_dn2 = 0.0;
        locals.var_fd2_dn4 = 0.0;
        locals.var_fd2_dn5 = 0.0;
        locals.var_fd2_dn6 = 0.0;
        locals.var_fd2_dn7 = 0.0;
        locals.var_fd2_dn8 = 0.0;
        locals.var_fd2_dn9 = 0.0;
        locals.var_fd2_dn10 = 0.0;
        locals.var_fd2_dn11 = 0.0;
        locals.var_fd2_dn14 = 0.0;

        locals.var_fmdvds = 0.0;
        locals.var_fmdvds_dn0 = 0.0;
        locals.var_fmdvds_dn2 = 0.0;
        locals.var_fmdvds_dn4 = 0.0;
        locals.var_fmdvds_dn5 = 0.0;
        locals.var_fmdvds_dn6 = 0.0;
        locals.var_fmdvds_dn7 = 0.0;
        locals.var_fmdvds_dn8 = 0.0;
        locals.var_fmdvds_dn9 = 0.0;
        locals.var_fmdvds_dn10 = 0.0;
        locals.var_fmdvds_dn11 = 0.0;
        locals.var_fmdvds_dn14 = 0.0;

        locals.var_cnst0 = 0.0;
        locals.var_cnst0_dn0 = 0.0;
        locals.var_cnst0_dn2 = 0.0;
        locals.var_cnst0_dn4 = 0.0;
        locals.var_cnst0_dn5 = 0.0;
        locals.var_cnst0_dn6 = 0.0;
        locals.var_cnst0_dn7 = 0.0;
        locals.var_cnst0_dn8 = 0.0;
        locals.var_cnst0_dn9 = 0.0;
        locals.var_cnst0_dn10 = 0.0;
        locals.var_cnst0_dn11 = 0.0;
        locals.var_cnst0_dn14 = 0.0;

        locals.var_cnst1 = 0.0;
        locals.var_cnst1_dn0 = 0.0;
        locals.var_cnst1_dn2 = 0.0;
        locals.var_cnst1_dn4 = 0.0;
        locals.var_cnst1_dn5 = 0.0;
        locals.var_cnst1_dn6 = 0.0;
        locals.var_cnst1_dn7 = 0.0;
        locals.var_cnst1_dn8 = 0.0;
        locals.var_cnst1_dn9 = 0.0;
        locals.var_cnst1_dn10 = 0.0;
        locals.var_cnst1_dn11 = 0.0;
        locals.var_cnst1_dn14 = 0.0;

        locals.var_cnstcoxi = 0.0;
        locals.var_cnstcoxi_dn0 = 0.0;
        locals.var_cnstcoxi_dn2 = 0.0;
        locals.var_cnstcoxi_dn4 = 0.0;
        locals.var_cnstcoxi_dn5 = 0.0;
        locals.var_cnstcoxi_dn6 = 0.0;
        locals.var_cnstcoxi_dn7 = 0.0;
        locals.var_cnstcoxi_dn8 = 0.0;
        locals.var_cnstcoxi_dn9 = 0.0;
        locals.var_cnstcoxi_dn10 = 0.0;
        locals.var_cnstcoxi_dn11 = 0.0;
        locals.var_cnstcoxi_dn14 = 0.0;

        locals.var_fac1 = 0.0;
        locals.var_fac1_dn0 = 0.0;
        locals.var_fac1_dn2 = 0.0;
        locals.var_fac1_dn4 = 0.0;
        locals.var_fac1_dn5 = 0.0;
        locals.var_fac1_dn6 = 0.0;
        locals.var_fac1_dn7 = 0.0;
        locals.var_fac1_dn8 = 0.0;
        locals.var_fac1_dn9 = 0.0;
        locals.var_fac1_dn10 = 0.0;
        locals.var_fac1_dn11 = 0.0;
        locals.var_fac1_dn14 = 0.0;

        locals.var_fac1p2 = 0.0;
        locals.var_fac1p2_dn0 = 0.0;
        locals.var_fac1p2_dn2 = 0.0;
        locals.var_fac1p2_dn4 = 0.0;
        locals.var_fac1p2_dn5 = 0.0;
        locals.var_fac1p2_dn6 = 0.0;
        locals.var_fac1p2_dn7 = 0.0;
        locals.var_fac1p2_dn8 = 0.0;
        locals.var_fac1p2_dn9 = 0.0;
        locals.var_fac1p2_dn10 = 0.0;
        locals.var_fac1p2_dn11 = 0.0;
        locals.var_fac1p2_dn14 = 0.0;

        locals.var_fs01 = 0.0;
        locals.var_fs01_dn0 = 0.0;
        locals.var_fs01_dn2 = 0.0;
        locals.var_fs01_dn4 = 0.0;
        locals.var_fs01_dn5 = 0.0;
        locals.var_fs01_dn6 = 0.0;
        locals.var_fs01_dn7 = 0.0;
        locals.var_fs01_dn8 = 0.0;
        locals.var_fs01_dn9 = 0.0;
        locals.var_fs01_dn10 = 0.0;
        locals.var_fs01_dn11 = 0.0;
        locals.var_fs01_dn14 = 0.0;

        locals.var_fs01_dps0 = 0.0;
        locals.var_fs01_dps0_dn0 = 0.0;
        locals.var_fs01_dps0_dn2 = 0.0;
        locals.var_fs01_dps0_dn4 = 0.0;
        locals.var_fs01_dps0_dn5 = 0.0;
        locals.var_fs01_dps0_dn6 = 0.0;
        locals.var_fs01_dps0_dn7 = 0.0;
        locals.var_fs01_dps0_dn8 = 0.0;
        locals.var_fs01_dps0_dn9 = 0.0;
        locals.var_fs01_dps0_dn10 = 0.0;
        locals.var_fs01_dps0_dn11 = 0.0;
        locals.var_fs01_dps0_dn14 = 0.0;

        locals.var_fs02 = 0.0;
        locals.var_fs02_dn0 = 0.0;
        locals.var_fs02_dn2 = 0.0;
        locals.var_fs02_dn4 = 0.0;
        locals.var_fs02_dn5 = 0.0;
        locals.var_fs02_dn6 = 0.0;
        locals.var_fs02_dn7 = 0.0;
        locals.var_fs02_dn8 = 0.0;
        locals.var_fs02_dn9 = 0.0;
        locals.var_fs02_dn10 = 0.0;
        locals.var_fs02_dn11 = 0.0;
        locals.var_fs02_dn14 = 0.0;

        locals.var_fs02_dps0 = 0.0;
        locals.var_fs02_dps0_dn0 = 0.0;
        locals.var_fs02_dps0_dn2 = 0.0;
        locals.var_fs02_dps0_dn4 = 0.0;
        locals.var_fs02_dps0_dn5 = 0.0;
        locals.var_fs02_dps0_dn6 = 0.0;
        locals.var_fs02_dps0_dn7 = 0.0;
        locals.var_fs02_dps0_dn8 = 0.0;
        locals.var_fs02_dps0_dn9 = 0.0;
        locals.var_fs02_dps0_dn10 = 0.0;
        locals.var_fs02_dps0_dn11 = 0.0;
        locals.var_fs02_dps0_dn14 = 0.0;

        locals.var_fsl1 = 0.0;
        locals.var_fsl1_dn0 = 0.0;
        locals.var_fsl1_dn2 = 0.0;
        locals.var_fsl1_dn4 = 0.0;
        locals.var_fsl1_dn5 = 0.0;
        locals.var_fsl1_dn6 = 0.0;
        locals.var_fsl1_dn7 = 0.0;
        locals.var_fsl1_dn8 = 0.0;
        locals.var_fsl1_dn9 = 0.0;
        locals.var_fsl1_dn10 = 0.0;
        locals.var_fsl1_dn11 = 0.0;
        locals.var_fsl1_dn14 = 0.0;

        locals.var_fsl1_dpsl = 0.0;
        locals.var_fsl1_dpsl_dn0 = 0.0;
        locals.var_fsl1_dpsl_dn2 = 0.0;
        locals.var_fsl1_dpsl_dn4 = 0.0;
        locals.var_fsl1_dpsl_dn5 = 0.0;
        locals.var_fsl1_dpsl_dn6 = 0.0;
        locals.var_fsl1_dpsl_dn7 = 0.0;
        locals.var_fsl1_dpsl_dn8 = 0.0;
        locals.var_fsl1_dpsl_dn9 = 0.0;
        locals.var_fsl1_dpsl_dn10 = 0.0;
        locals.var_fsl1_dpsl_dn11 = 0.0;
        locals.var_fsl1_dpsl_dn14 = 0.0;

        locals.var_fsl2 = 0.0;
        locals.var_fsl2_dn0 = 0.0;
        locals.var_fsl2_dn2 = 0.0;
        locals.var_fsl2_dn4 = 0.0;
        locals.var_fsl2_dn5 = 0.0;
        locals.var_fsl2_dn6 = 0.0;
        locals.var_fsl2_dn7 = 0.0;
        locals.var_fsl2_dn8 = 0.0;
        locals.var_fsl2_dn9 = 0.0;
        locals.var_fsl2_dn10 = 0.0;
        locals.var_fsl2_dn11 = 0.0;
        locals.var_fsl2_dn14 = 0.0;

        locals.var_fsl2_dpsl = 0.0;
        locals.var_fsl2_dpsl_dn0 = 0.0;
        locals.var_fsl2_dpsl_dn2 = 0.0;
        locals.var_fsl2_dpsl_dn4 = 0.0;
        locals.var_fsl2_dpsl_dn5 = 0.0;
        locals.var_fsl2_dpsl_dn6 = 0.0;
        locals.var_fsl2_dpsl_dn7 = 0.0;
        locals.var_fsl2_dpsl_dn8 = 0.0;
        locals.var_fsl2_dpsl_dn9 = 0.0;
        locals.var_fsl2_dpsl_dn10 = 0.0;
        locals.var_fsl2_dpsl_dn11 = 0.0;
        locals.var_fsl2_dpsl_dn14 = 0.0;

        locals.var_cfs1 = 0.0;
        locals.var_cfs1_dn0 = 0.0;
        locals.var_cfs1_dn2 = 0.0;
        locals.var_cfs1_dn4 = 0.0;
        locals.var_cfs1_dn5 = 0.0;
        locals.var_cfs1_dn6 = 0.0;
        locals.var_cfs1_dn7 = 0.0;
        locals.var_cfs1_dn8 = 0.0;
        locals.var_cfs1_dn9 = 0.0;
        locals.var_cfs1_dn10 = 0.0;
        locals.var_cfs1_dn11 = 0.0;
        locals.var_cfs1_dn14 = 0.0;

        locals.var_fb = 0.0;
        locals.var_fb_dn0 = 0.0;
        locals.var_fb_dn2 = 0.0;
        locals.var_fb_dn4 = 0.0;
        locals.var_fb_dn5 = 0.0;
        locals.var_fb_dn6 = 0.0;
        locals.var_fb_dn7 = 0.0;
        locals.var_fb_dn8 = 0.0;
        locals.var_fb_dn9 = 0.0;
        locals.var_fb_dn10 = 0.0;
        locals.var_fb_dn11 = 0.0;
        locals.var_fb_dn14 = 0.0;

        locals.var_fb_dchi = 0.0;
        locals.var_fb_dchi_dn0 = 0.0;
        locals.var_fb_dchi_dn2 = 0.0;
        locals.var_fb_dchi_dn4 = 0.0;
        locals.var_fb_dchi_dn5 = 0.0;
        locals.var_fb_dchi_dn6 = 0.0;
        locals.var_fb_dchi_dn7 = 0.0;
        locals.var_fb_dchi_dn8 = 0.0;
        locals.var_fb_dchi_dn9 = 0.0;
        locals.var_fb_dchi_dn10 = 0.0;
        locals.var_fb_dchi_dn11 = 0.0;
        locals.var_fb_dchi_dn14 = 0.0;

        locals.var_fi = 0.0;
        locals.var_fi_dn0 = 0.0;
        locals.var_fi_dn2 = 0.0;
        locals.var_fi_dn4 = 0.0;
        locals.var_fi_dn5 = 0.0;
        locals.var_fi_dn6 = 0.0;
        locals.var_fi_dn7 = 0.0;
        locals.var_fi_dn8 = 0.0;
        locals.var_fi_dn9 = 0.0;
        locals.var_fi_dn10 = 0.0;
        locals.var_fi_dn11 = 0.0;
        locals.var_fi_dn14 = 0.0;

        locals.var_fi_dchi = 0.0;
        locals.var_fi_dchi_dn0 = 0.0;
        locals.var_fi_dchi_dn2 = 0.0;
        locals.var_fi_dchi_dn4 = 0.0;
        locals.var_fi_dchi_dn5 = 0.0;
        locals.var_fi_dchi_dn6 = 0.0;
        locals.var_fi_dchi_dn7 = 0.0;
        locals.var_fi_dchi_dn8 = 0.0;
        locals.var_fi_dchi_dn9 = 0.0;
        locals.var_fi_dchi_dn10 = 0.0;
        locals.var_fi_dchi_dn11 = 0.0;
        locals.var_fi_dchi_dn14 = 0.0;

        locals.var_exp_chi = 0.0;
        locals.var_exp_chi_dn0 = 0.0;
        locals.var_exp_chi_dn2 = 0.0;
        locals.var_exp_chi_dn4 = 0.0;
        locals.var_exp_chi_dn5 = 0.0;
        locals.var_exp_chi_dn6 = 0.0;
        locals.var_exp_chi_dn7 = 0.0;
        locals.var_exp_chi_dn8 = 0.0;
        locals.var_exp_chi_dn9 = 0.0;
        locals.var_exp_chi_dn10 = 0.0;
        locals.var_exp_chi_dn11 = 0.0;
        locals.var_exp_chi_dn14 = 0.0;

        locals.var_exp_rho = 0.0;
        locals.var_exp_rho_dn0 = 0.0;
        locals.var_exp_rho_dn2 = 0.0;
        locals.var_exp_rho_dn4 = 0.0;
        locals.var_exp_rho_dn5 = 0.0;
        locals.var_exp_rho_dn6 = 0.0;
        locals.var_exp_rho_dn7 = 0.0;
        locals.var_exp_rho_dn8 = 0.0;
        locals.var_exp_rho_dn9 = 0.0;
        locals.var_exp_rho_dn10 = 0.0;
        locals.var_exp_rho_dn11 = 0.0;
        locals.var_exp_rho_dn14 = 0.0;

        locals.var_exp_bvbs = 0.0;
        locals.var_exp_bvbs_dn0 = 0.0;
        locals.var_exp_bvbs_dn2 = 0.0;
        locals.var_exp_bvbs_dn4 = 0.0;
        locals.var_exp_bvbs_dn5 = 0.0;
        locals.var_exp_bvbs_dn6 = 0.0;
        locals.var_exp_bvbs_dn7 = 0.0;
        locals.var_exp_bvbs_dn8 = 0.0;
        locals.var_exp_bvbs_dn9 = 0.0;
        locals.var_exp_bvbs_dn10 = 0.0;
        locals.var_exp_bvbs_dn11 = 0.0;
        locals.var_exp_bvbs_dn14 = 0.0;

        locals.var_exp_bvbsvds = 0.0;
        locals.var_exp_bvbsvds_dn0 = 0.0;
        locals.var_exp_bvbsvds_dn2 = 0.0;
        locals.var_exp_bvbsvds_dn4 = 0.0;
        locals.var_exp_bvbsvds_dn5 = 0.0;
        locals.var_exp_bvbsvds_dn6 = 0.0;
        locals.var_exp_bvbsvds_dn7 = 0.0;
        locals.var_exp_bvbsvds_dn8 = 0.0;
        locals.var_exp_bvbsvds_dn9 = 0.0;
        locals.var_exp_bvbsvds_dn10 = 0.0;
        locals.var_exp_bvbsvds_dn11 = 0.0;
        locals.var_exp_bvbsvds_dn14 = 0.0;

        locals.var_exp_bps0 = 0.0;
        locals.var_exp_bps0_dn0 = 0.0;
        locals.var_exp_bps0_dn2 = 0.0;
        locals.var_exp_bps0_dn4 = 0.0;
        locals.var_exp_bps0_dn5 = 0.0;
        locals.var_exp_bps0_dn6 = 0.0;
        locals.var_exp_bps0_dn7 = 0.0;
        locals.var_exp_bps0_dn8 = 0.0;
        locals.var_exp_bps0_dn9 = 0.0;
        locals.var_exp_bps0_dn10 = 0.0;
        locals.var_exp_bps0_dn11 = 0.0;
        locals.var_exp_bps0_dn14 = 0.0;

        locals.var_fs0 = 0.0;
        locals.var_fs0_dn0 = 0.0;
        locals.var_fs0_dn2 = 0.0;
        locals.var_fs0_dn4 = 0.0;
        locals.var_fs0_dn5 = 0.0;
        locals.var_fs0_dn6 = 0.0;
        locals.var_fs0_dn7 = 0.0;
        locals.var_fs0_dn8 = 0.0;
        locals.var_fs0_dn9 = 0.0;
        locals.var_fs0_dn10 = 0.0;
        locals.var_fs0_dn11 = 0.0;
        locals.var_fs0_dn14 = 0.0;

        locals.var_fs0_dps0 = 0.0;
        locals.var_fs0_dps0_dn0 = 0.0;
        locals.var_fs0_dps0_dn2 = 0.0;
        locals.var_fs0_dps0_dn4 = 0.0;
        locals.var_fs0_dps0_dn5 = 0.0;
        locals.var_fs0_dps0_dn6 = 0.0;
        locals.var_fs0_dps0_dn7 = 0.0;
        locals.var_fs0_dps0_dn8 = 0.0;
        locals.var_fs0_dps0_dn9 = 0.0;
        locals.var_fs0_dps0_dn10 = 0.0;
        locals.var_fs0_dps0_dn11 = 0.0;
        locals.var_fs0_dps0_dn14 = 0.0;

        locals.var_fsl = 0.0;
        locals.var_fsl_dn0 = 0.0;
        locals.var_fsl_dn2 = 0.0;
        locals.var_fsl_dn4 = 0.0;
        locals.var_fsl_dn5 = 0.0;
        locals.var_fsl_dn6 = 0.0;
        locals.var_fsl_dn7 = 0.0;
        locals.var_fsl_dn8 = 0.0;
        locals.var_fsl_dn9 = 0.0;
        locals.var_fsl_dn10 = 0.0;
        locals.var_fsl_dn11 = 0.0;
        locals.var_fsl_dn14 = 0.0;

        locals.var_fsl_dpsl = 0.0;
        locals.var_fsl_dpsl_dn0 = 0.0;
        locals.var_fsl_dpsl_dn2 = 0.0;
        locals.var_fsl_dpsl_dn4 = 0.0;
        locals.var_fsl_dpsl_dn5 = 0.0;
        locals.var_fsl_dpsl_dn6 = 0.0;
        locals.var_fsl_dpsl_dn7 = 0.0;
        locals.var_fsl_dpsl_dn8 = 0.0;
        locals.var_fsl_dpsl_dn9 = 0.0;
        locals.var_fsl_dpsl_dn10 = 0.0;
        locals.var_fsl_dpsl_dn11 = 0.0;
        locals.var_fsl_dpsl_dn14 = 0.0;

        locals.var_dps0 = 0.0;
        locals.var_dps0_dn0 = 0.0;
        locals.var_dps0_dn2 = 0.0;
        locals.var_dps0_dn4 = 0.0;
        locals.var_dps0_dn5 = 0.0;
        locals.var_dps0_dn6 = 0.0;
        locals.var_dps0_dn7 = 0.0;
        locals.var_dps0_dn8 = 0.0;
        locals.var_dps0_dn9 = 0.0;
        locals.var_dps0_dn10 = 0.0;
        locals.var_dps0_dn11 = 0.0;
        locals.var_dps0_dn14 = 0.0;

        locals.var_dpsl = 0.0;
        locals.var_dpsl_dn0 = 0.0;
        locals.var_dpsl_dn2 = 0.0;
        locals.var_dpsl_dn4 = 0.0;
        locals.var_dpsl_dn5 = 0.0;
        locals.var_dpsl_dn6 = 0.0;
        locals.var_dpsl_dn7 = 0.0;
        locals.var_dpsl_dn8 = 0.0;
        locals.var_dpsl_dn9 = 0.0;
        locals.var_dpsl_dn10 = 0.0;
        locals.var_dpsl_dn11 = 0.0;
        locals.var_dpsl_dn14 = 0.0;

        locals.var_qn0 = 0.0;
        locals.var_qn0_dn0 = 0.0;
        locals.var_qn0_dn2 = 0.0;
        locals.var_qn0_dn4 = 0.0;
        locals.var_qn0_dn5 = 0.0;
        locals.var_qn0_dn6 = 0.0;
        locals.var_qn0_dn7 = 0.0;
        locals.var_qn0_dn8 = 0.0;
        locals.var_qn0_dn9 = 0.0;
        locals.var_qn0_dn10 = 0.0;
        locals.var_qn0_dn11 = 0.0;
        locals.var_qn0_dn14 = 0.0;

        locals.var_qb0 = 0.0;
        locals.var_qb0_dn0 = 0.0;
        locals.var_qb0_dn2 = 0.0;
        locals.var_qb0_dn4 = 0.0;
        locals.var_qb0_dn5 = 0.0;
        locals.var_qb0_dn6 = 0.0;
        locals.var_qb0_dn7 = 0.0;
        locals.var_qb0_dn8 = 0.0;
        locals.var_qb0_dn9 = 0.0;
        locals.var_qb0_dn10 = 0.0;
        locals.var_qb0_dn11 = 0.0;
        locals.var_qb0_dn14 = 0.0;

        locals.var_qbnm = 0.0;
        locals.var_qbnm_dn0 = 0.0;
        locals.var_qbnm_dn2 = 0.0;
        locals.var_qbnm_dn4 = 0.0;
        locals.var_qbnm_dn5 = 0.0;
        locals.var_qbnm_dn6 = 0.0;
        locals.var_qbnm_dn7 = 0.0;
        locals.var_qbnm_dn8 = 0.0;
        locals.var_qbnm_dn9 = 0.0;
        locals.var_qbnm_dn10 = 0.0;
        locals.var_qbnm_dn11 = 0.0;
        locals.var_qbnm_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_4(
        locals: &mut StampLocals,
    ) {
        locals.var_dtpds = 0.0;
        locals.var_dtpds_dn0 = 0.0;
        locals.var_dtpds_dn2 = 0.0;
        locals.var_dtpds_dn4 = 0.0;
        locals.var_dtpds_dn5 = 0.0;
        locals.var_dtpds_dn6 = 0.0;
        locals.var_dtpds_dn7 = 0.0;
        locals.var_dtpds_dn8 = 0.0;
        locals.var_dtpds_dn9 = 0.0;
        locals.var_dtpds_dn10 = 0.0;
        locals.var_dtpds_dn11 = 0.0;
        locals.var_dtpds_dn14 = 0.0;

        locals.var_qinm = 0.0;
        locals.var_qinm_dn0 = 0.0;
        locals.var_qinm_dn2 = 0.0;
        locals.var_qinm_dn4 = 0.0;
        locals.var_qinm_dn5 = 0.0;
        locals.var_qinm_dn6 = 0.0;
        locals.var_qinm_dn7 = 0.0;
        locals.var_qinm_dn8 = 0.0;
        locals.var_qinm_dn9 = 0.0;
        locals.var_qinm_dn10 = 0.0;
        locals.var_qinm_dn11 = 0.0;
        locals.var_qinm_dn14 = 0.0;

        locals.var_qidn = 0.0;
        locals.var_qidn_dn0 = 0.0;
        locals.var_qidn_dn2 = 0.0;
        locals.var_qidn_dn4 = 0.0;
        locals.var_qidn_dn5 = 0.0;
        locals.var_qidn_dn6 = 0.0;
        locals.var_qidn_dn7 = 0.0;
        locals.var_qidn_dn8 = 0.0;
        locals.var_qidn_dn9 = 0.0;
        locals.var_qidn_dn10 = 0.0;
        locals.var_qidn_dn11 = 0.0;
        locals.var_qidn_dn14 = 0.0;

        locals.var_qdnm = 0.0;
        locals.var_qdnm_dn0 = 0.0;
        locals.var_qdnm_dn2 = 0.0;
        locals.var_qdnm_dn4 = 0.0;
        locals.var_qdnm_dn5 = 0.0;
        locals.var_qdnm_dn6 = 0.0;
        locals.var_qdnm_dn7 = 0.0;
        locals.var_qdnm_dn8 = 0.0;
        locals.var_qdnm_dn9 = 0.0;
        locals.var_qdnm_dn10 = 0.0;
        locals.var_qdnm_dn11 = 0.0;
        locals.var_qdnm_dn14 = 0.0;

        locals.var_qddn = 0.0;
        locals.var_qddn_dn0 = 0.0;
        locals.var_qddn_dn2 = 0.0;
        locals.var_qddn_dn4 = 0.0;
        locals.var_qddn_dn5 = 0.0;
        locals.var_qddn_dn6 = 0.0;
        locals.var_qddn_dn7 = 0.0;
        locals.var_qddn_dn8 = 0.0;
        locals.var_qddn_dn9 = 0.0;
        locals.var_qddn_dn10 = 0.0;
        locals.var_qddn_dn11 = 0.0;
        locals.var_qddn_dn14 = 0.0;

        locals.var_quot = 0.0;
        locals.var_quot_dn0 = 0.0;
        locals.var_quot_dn2 = 0.0;
        locals.var_quot_dn4 = 0.0;
        locals.var_quot_dn5 = 0.0;
        locals.var_quot_dn6 = 0.0;
        locals.var_quot_dn7 = 0.0;
        locals.var_quot_dn8 = 0.0;
        locals.var_quot_dn9 = 0.0;
        locals.var_quot_dn10 = 0.0;
        locals.var_quot_dn11 = 0.0;
        locals.var_quot_dn14 = 0.0;

        locals.var_qdrat = 0.5;
        locals.var_qdrat_dn0 = 0.0;
        locals.var_qdrat_dn2 = 0.0;
        locals.var_qdrat_dn4 = 0.0;
        locals.var_qdrat_dn5 = 0.0;
        locals.var_qdrat_dn6 = 0.0;
        locals.var_qdrat_dn7 = 0.0;
        locals.var_qdrat_dn8 = 0.0;
        locals.var_qdrat_dn9 = 0.0;
        locals.var_qdrat_dn10 = 0.0;
        locals.var_qdrat_dn11 = 0.0;
        locals.var_qdrat_dn14 = 0.0;

        locals.var_idd = 0.0;
        locals.var_idd_dn0 = 0.0;
        locals.var_idd_dn2 = 0.0;
        locals.var_idd_dn4 = 0.0;
        locals.var_idd_dn5 = 0.0;
        locals.var_idd_dn6 = 0.0;
        locals.var_idd_dn7 = 0.0;
        locals.var_idd_dn8 = 0.0;
        locals.var_idd_dn9 = 0.0;
        locals.var_idd_dn10 = 0.0;
        locals.var_idd_dn11 = 0.0;
        locals.var_idd_dn14 = 0.0;

        locals.var_idd1 = 0.0;
        locals.var_idd1_dn0 = 0.0;
        locals.var_idd1_dn2 = 0.0;
        locals.var_idd1_dn4 = 0.0;
        locals.var_idd1_dn5 = 0.0;
        locals.var_idd1_dn6 = 0.0;
        locals.var_idd1_dn7 = 0.0;
        locals.var_idd1_dn8 = 0.0;
        locals.var_idd1_dn9 = 0.0;
        locals.var_idd1_dn10 = 0.0;
        locals.var_idd1_dn11 = 0.0;
        locals.var_idd1_dn14 = 0.0;

        locals.var_fdd = 0.0;
        locals.var_fdd_dn0 = 0.0;
        locals.var_fdd_dn2 = 0.0;
        locals.var_fdd_dn4 = 0.0;
        locals.var_fdd_dn5 = 0.0;
        locals.var_fdd_dn6 = 0.0;
        locals.var_fdd_dn7 = 0.0;
        locals.var_fdd_dn8 = 0.0;
        locals.var_fdd_dn9 = 0.0;
        locals.var_fdd_dn10 = 0.0;
        locals.var_fdd_dn11 = 0.0;
        locals.var_fdd_dn14 = 0.0;

        locals.var_eeff = 0.0;
        locals.var_eeff_dn0 = 0.0;
        locals.var_eeff_dn2 = 0.0;
        locals.var_eeff_dn4 = 0.0;
        locals.var_eeff_dn5 = 0.0;
        locals.var_eeff_dn6 = 0.0;
        locals.var_eeff_dn7 = 0.0;
        locals.var_eeff_dn8 = 0.0;
        locals.var_eeff_dn9 = 0.0;
        locals.var_eeff_dn10 = 0.0;
        locals.var_eeff_dn11 = 0.0;
        locals.var_eeff_dn14 = 0.0;

        locals.var_rns = 0.0;
        locals.var_rns_dn0 = 0.0;
        locals.var_rns_dn2 = 0.0;
        locals.var_rns_dn4 = 0.0;
        locals.var_rns_dn5 = 0.0;
        locals.var_rns_dn6 = 0.0;
        locals.var_rns_dn7 = 0.0;
        locals.var_rns_dn8 = 0.0;
        locals.var_rns_dn9 = 0.0;
        locals.var_rns_dn10 = 0.0;
        locals.var_rns_dn11 = 0.0;
        locals.var_rns_dn14 = 0.0;

        locals.var_mu = 0.0;
        locals.var_mu_dn0 = 0.0;
        locals.var_mu_dn2 = 0.0;
        locals.var_mu_dn4 = 0.0;
        locals.var_mu_dn5 = 0.0;
        locals.var_mu_dn6 = 0.0;
        locals.var_mu_dn7 = 0.0;
        locals.var_mu_dn8 = 0.0;
        locals.var_mu_dn9 = 0.0;
        locals.var_mu_dn10 = 0.0;
        locals.var_mu_dn11 = 0.0;
        locals.var_mu_dn14 = 0.0;

        locals.var_muun = 0.0;
        locals.var_muun_dn0 = 0.0;
        locals.var_muun_dn2 = 0.0;
        locals.var_muun_dn4 = 0.0;
        locals.var_muun_dn5 = 0.0;
        locals.var_muun_dn6 = 0.0;
        locals.var_muun_dn7 = 0.0;
        locals.var_muun_dn8 = 0.0;
        locals.var_muun_dn9 = 0.0;
        locals.var_muun_dn10 = 0.0;
        locals.var_muun_dn11 = 0.0;
        locals.var_muun_dn14 = 0.0;

        locals.var_ey = 0.0;
        locals.var_ey_dn0 = 0.0;
        locals.var_ey_dn2 = 0.0;
        locals.var_ey_dn4 = 0.0;
        locals.var_ey_dn5 = 0.0;
        locals.var_ey_dn6 = 0.0;
        locals.var_ey_dn7 = 0.0;
        locals.var_ey_dn8 = 0.0;
        locals.var_ey_dn9 = 0.0;
        locals.var_ey_dn10 = 0.0;
        locals.var_ey_dn11 = 0.0;
        locals.var_ey_dn14 = 0.0;

        locals.var_em = 0.0;
        locals.var_em_dn0 = 0.0;
        locals.var_em_dn2 = 0.0;
        locals.var_em_dn4 = 0.0;
        locals.var_em_dn5 = 0.0;
        locals.var_em_dn6 = 0.0;
        locals.var_em_dn7 = 0.0;
        locals.var_em_dn8 = 0.0;
        locals.var_em_dn9 = 0.0;
        locals.var_em_dn10 = 0.0;
        locals.var_em_dn11 = 0.0;
        locals.var_em_dn14 = 0.0;

        locals.var_eta = 0.0;
        locals.var_eta_dn0 = 0.0;
        locals.var_eta_dn2 = 0.0;
        locals.var_eta_dn4 = 0.0;
        locals.var_eta_dn5 = 0.0;
        locals.var_eta_dn6 = 0.0;
        locals.var_eta_dn7 = 0.0;
        locals.var_eta_dn8 = 0.0;
        locals.var_eta_dn9 = 0.0;
        locals.var_eta_dn10 = 0.0;
        locals.var_eta_dn11 = 0.0;
        locals.var_eta_dn14 = 0.0;

        locals.var_eta1 = 0.0;
        locals.var_eta1_dn0 = 0.0;
        locals.var_eta1_dn2 = 0.0;
        locals.var_eta1_dn4 = 0.0;
        locals.var_eta1_dn5 = 0.0;
        locals.var_eta1_dn6 = 0.0;
        locals.var_eta1_dn7 = 0.0;
        locals.var_eta1_dn8 = 0.0;
        locals.var_eta1_dn9 = 0.0;
        locals.var_eta1_dn10 = 0.0;
        locals.var_eta1_dn11 = 0.0;
        locals.var_eta1_dn14 = 0.0;

        locals.var_eta1p12 = 0.0;
        locals.var_eta1p12_dn0 = 0.0;
        locals.var_eta1p12_dn2 = 0.0;
        locals.var_eta1p12_dn4 = 0.0;
        locals.var_eta1p12_dn5 = 0.0;
        locals.var_eta1p12_dn6 = 0.0;
        locals.var_eta1p12_dn7 = 0.0;
        locals.var_eta1p12_dn8 = 0.0;
        locals.var_eta1p12_dn9 = 0.0;
        locals.var_eta1p12_dn10 = 0.0;
        locals.var_eta1p12_dn11 = 0.0;
        locals.var_eta1p12_dn14 = 0.0;

        locals.var_eta1p32 = 0.0;
        locals.var_eta1p32_dn0 = 0.0;
        locals.var_eta1p32_dn2 = 0.0;
        locals.var_eta1p32_dn4 = 0.0;
        locals.var_eta1p32_dn5 = 0.0;
        locals.var_eta1p32_dn6 = 0.0;
        locals.var_eta1p32_dn7 = 0.0;
        locals.var_eta1p32_dn8 = 0.0;
        locals.var_eta1p32_dn9 = 0.0;
        locals.var_eta1p32_dn10 = 0.0;
        locals.var_eta1p32_dn11 = 0.0;
        locals.var_eta1p32_dn14 = 0.0;

        locals.var_eta1p52 = 0.0;
        locals.var_eta1p52_dn0 = 0.0;
        locals.var_eta1p52_dn2 = 0.0;
        locals.var_eta1p52_dn4 = 0.0;
        locals.var_eta1p52_dn5 = 0.0;
        locals.var_eta1p52_dn6 = 0.0;
        locals.var_eta1p52_dn7 = 0.0;
        locals.var_eta1p52_dn8 = 0.0;
        locals.var_eta1p52_dn9 = 0.0;
        locals.var_eta1p52_dn10 = 0.0;
        locals.var_eta1p52_dn11 = 0.0;
        locals.var_eta1p52_dn14 = 0.0;

        locals.var_zeta12 = 0.0;
        locals.var_zeta12_dn0 = 0.0;
        locals.var_zeta12_dn2 = 0.0;
        locals.var_zeta12_dn4 = 0.0;
        locals.var_zeta12_dn5 = 0.0;
        locals.var_zeta12_dn6 = 0.0;
        locals.var_zeta12_dn7 = 0.0;
        locals.var_zeta12_dn8 = 0.0;
        locals.var_zeta12_dn9 = 0.0;
        locals.var_zeta12_dn10 = 0.0;
        locals.var_zeta12_dn11 = 0.0;
        locals.var_zeta12_dn14 = 0.0;

        locals.var_zeta32 = 0.0;
        locals.var_zeta32_dn0 = 0.0;
        locals.var_zeta32_dn2 = 0.0;
        locals.var_zeta32_dn4 = 0.0;
        locals.var_zeta32_dn5 = 0.0;
        locals.var_zeta32_dn6 = 0.0;
        locals.var_zeta32_dn7 = 0.0;
        locals.var_zeta32_dn8 = 0.0;
        locals.var_zeta32_dn9 = 0.0;
        locals.var_zeta32_dn10 = 0.0;
        locals.var_zeta32_dn11 = 0.0;
        locals.var_zeta32_dn14 = 0.0;

        locals.var_zeta52 = 0.0;
        locals.var_zeta52_dn0 = 0.0;
        locals.var_zeta52_dn2 = 0.0;
        locals.var_zeta52_dn4 = 0.0;
        locals.var_zeta52_dn5 = 0.0;
        locals.var_zeta52_dn6 = 0.0;
        locals.var_zeta52_dn7 = 0.0;
        locals.var_zeta52_dn8 = 0.0;
        locals.var_zeta52_dn9 = 0.0;
        locals.var_zeta52_dn10 = 0.0;
        locals.var_zeta52_dn11 = 0.0;
        locals.var_zeta52_dn14 = 0.0;

        locals.var_f00 = 0.0;
        locals.var_f00_dn0 = 0.0;
        locals.var_f00_dn2 = 0.0;
        locals.var_f00_dn4 = 0.0;
        locals.var_f00_dn5 = 0.0;
        locals.var_f00_dn6 = 0.0;
        locals.var_f00_dn7 = 0.0;
        locals.var_f00_dn8 = 0.0;
        locals.var_f00_dn9 = 0.0;
        locals.var_f00_dn10 = 0.0;
        locals.var_f00_dn11 = 0.0;
        locals.var_f00_dn14 = 0.0;

        locals.var_f10 = 0.0;
        locals.var_f10_dn0 = 0.0;
        locals.var_f10_dn2 = 0.0;
        locals.var_f10_dn4 = 0.0;
        locals.var_f10_dn5 = 0.0;
        locals.var_f10_dn6 = 0.0;
        locals.var_f10_dn7 = 0.0;
        locals.var_f10_dn8 = 0.0;
        locals.var_f10_dn9 = 0.0;
        locals.var_f10_dn10 = 0.0;
        locals.var_f10_dn11 = 0.0;
        locals.var_f10_dn14 = 0.0;

        locals.var_f30 = 0.0;
        locals.var_f30_dn0 = 0.0;
        locals.var_f30_dn2 = 0.0;
        locals.var_f30_dn4 = 0.0;
        locals.var_f30_dn5 = 0.0;
        locals.var_f30_dn6 = 0.0;
        locals.var_f30_dn7 = 0.0;
        locals.var_f30_dn8 = 0.0;
        locals.var_f30_dn9 = 0.0;
        locals.var_f30_dn10 = 0.0;
        locals.var_f30_dn11 = 0.0;
        locals.var_f30_dn14 = 0.0;

        locals.var_f11 = 0.0;
        locals.var_f11_dn0 = 0.0;
        locals.var_f11_dn2 = 0.0;
        locals.var_f11_dn4 = 0.0;
        locals.var_f11_dn5 = 0.0;
        locals.var_f11_dn6 = 0.0;
        locals.var_f11_dn7 = 0.0;
        locals.var_f11_dn8 = 0.0;
        locals.var_f11_dn9 = 0.0;
        locals.var_f11_dn10 = 0.0;
        locals.var_f11_dn11 = 0.0;
        locals.var_f11_dn14 = 0.0;

        locals.var_vgs_min = 0.0;

        locals.var_ps0_min = 0.0;
        locals.var_ps0_min_dn0 = 0.0;
        locals.var_ps0_min_dn2 = 0.0;
        locals.var_ps0_min_dn4 = 0.0;
        locals.var_ps0_min_dn5 = 0.0;
        locals.var_ps0_min_dn6 = 0.0;
        locals.var_ps0_min_dn7 = 0.0;
        locals.var_ps0_min_dn8 = 0.0;
        locals.var_ps0_min_dn9 = 0.0;
        locals.var_ps0_min_dn10 = 0.0;
        locals.var_ps0_min_dn11 = 0.0;
        locals.var_ps0_min_dn14 = 0.0;

        locals.var_acn = 0.0;
        locals.var_acn_dn0 = 0.0;
        locals.var_acn_dn2 = 0.0;
        locals.var_acn_dn4 = 0.0;
        locals.var_acn_dn5 = 0.0;
        locals.var_acn_dn6 = 0.0;
        locals.var_acn_dn7 = 0.0;
        locals.var_acn_dn8 = 0.0;
        locals.var_acn_dn9 = 0.0;
        locals.var_acn_dn10 = 0.0;
        locals.var_acn_dn11 = 0.0;
        locals.var_acn_dn14 = 0.0;

        locals.var_acd = 0.0;
        locals.var_acd_dn0 = 0.0;
        locals.var_acd_dn2 = 0.0;
        locals.var_acd_dn4 = 0.0;
        locals.var_acd_dn5 = 0.0;
        locals.var_acd_dn6 = 0.0;
        locals.var_acd_dn7 = 0.0;
        locals.var_acd_dn8 = 0.0;
        locals.var_acd_dn9 = 0.0;
        locals.var_acd_dn10 = 0.0;
        locals.var_acd_dn11 = 0.0;
        locals.var_acd_dn14 = 0.0;

        locals.var_ac1 = 0.0;
        locals.var_ac1_dn0 = 0.0;
        locals.var_ac1_dn2 = 0.0;
        locals.var_ac1_dn4 = 0.0;
        locals.var_ac1_dn5 = 0.0;
        locals.var_ac1_dn6 = 0.0;
        locals.var_ac1_dn7 = 0.0;
        locals.var_ac1_dn8 = 0.0;
        locals.var_ac1_dn9 = 0.0;
        locals.var_ac1_dn10 = 0.0;
        locals.var_ac1_dn11 = 0.0;
        locals.var_ac1_dn14 = 0.0;

        locals.var_ac2 = 0.0;
        locals.var_ac2_dn0 = 0.0;
        locals.var_ac2_dn2 = 0.0;
        locals.var_ac2_dn4 = 0.0;
        locals.var_ac2_dn5 = 0.0;
        locals.var_ac2_dn6 = 0.0;
        locals.var_ac2_dn7 = 0.0;
        locals.var_ac2_dn8 = 0.0;
        locals.var_ac2_dn9 = 0.0;
        locals.var_ac2_dn10 = 0.0;
        locals.var_ac2_dn11 = 0.0;
        locals.var_ac2_dn14 = 0.0;

        locals.var_ac3 = 0.0;
        locals.var_ac3_dn0 = 0.0;
        locals.var_ac3_dn2 = 0.0;
        locals.var_ac3_dn4 = 0.0;
        locals.var_ac3_dn5 = 0.0;
        locals.var_ac3_dn6 = 0.0;
        locals.var_ac3_dn7 = 0.0;
        locals.var_ac3_dn8 = 0.0;
        locals.var_ac3_dn9 = 0.0;
        locals.var_ac3_dn10 = 0.0;
        locals.var_ac3_dn11 = 0.0;
        locals.var_ac3_dn14 = 0.0;

        locals.var_ac4 = 0.0;
        locals.var_ac4_dn0 = 0.0;
        locals.var_ac4_dn2 = 0.0;
        locals.var_ac4_dn4 = 0.0;
        locals.var_ac4_dn5 = 0.0;
        locals.var_ac4_dn6 = 0.0;
        locals.var_ac4_dn7 = 0.0;
        locals.var_ac4_dn8 = 0.0;
        locals.var_ac4_dn9 = 0.0;
        locals.var_ac4_dn10 = 0.0;
        locals.var_ac4_dn11 = 0.0;
        locals.var_ac4_dn14 = 0.0;

        locals.var_ac31 = 0.0;
        locals.var_ac31_dn0 = 0.0;
        locals.var_ac31_dn2 = 0.0;
        locals.var_ac31_dn4 = 0.0;
        locals.var_ac31_dn5 = 0.0;
        locals.var_ac31_dn6 = 0.0;
        locals.var_ac31_dn7 = 0.0;
        locals.var_ac31_dn8 = 0.0;
        locals.var_ac31_dn9 = 0.0;
        locals.var_ac31_dn10 = 0.0;
        locals.var_ac31_dn11 = 0.0;
        locals.var_ac31_dn14 = 0.0;

        locals.var_ac41 = 0.0;
        locals.var_ac41_dn0 = 0.0;
        locals.var_ac41_dn2 = 0.0;
        locals.var_ac41_dn4 = 0.0;
        locals.var_ac41_dn5 = 0.0;
        locals.var_ac41_dn6 = 0.0;
        locals.var_ac41_dn7 = 0.0;
        locals.var_ac41_dn8 = 0.0;
        locals.var_ac41_dn9 = 0.0;
        locals.var_ac41_dn10 = 0.0;
        locals.var_ac41_dn11 = 0.0;
        locals.var_ac41_dn14 = 0.0;

        locals.var_isub = 0.0;
        locals.var_isub_dn0 = 0.0;
        locals.var_isub_dn2 = 0.0;
        locals.var_isub_dn4 = 0.0;
        locals.var_isub_dn5 = 0.0;
        locals.var_isub_dn6 = 0.0;
        locals.var_isub_dn7 = 0.0;
        locals.var_isub_dn8 = 0.0;
        locals.var_isub_dn9 = 0.0;
        locals.var_isub_dn10 = 0.0;
        locals.var_isub_dn11 = 0.0;
        locals.var_isub_dn14 = 0.0;

        locals.var_isubld = 0.0;
        locals.var_isubld_dn0 = 0.0;
        locals.var_isubld_dn2 = 0.0;
        locals.var_isubld_dn4 = 0.0;
        locals.var_isubld_dn5 = 0.0;
        locals.var_isubld_dn6 = 0.0;
        locals.var_isubld_dn7 = 0.0;
        locals.var_isubld_dn8 = 0.0;
        locals.var_isubld_dn9 = 0.0;
        locals.var_isubld_dn10 = 0.0;
        locals.var_isubld_dn11 = 0.0;
        locals.var_isubld_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_5(
        locals: &mut StampLocals,
    ) {
        locals.var_psislsat = 0.0;
        locals.var_psislsat_dn0 = 0.0;
        locals.var_psislsat_dn2 = 0.0;
        locals.var_psislsat_dn4 = 0.0;
        locals.var_psislsat_dn5 = 0.0;
        locals.var_psislsat_dn6 = 0.0;
        locals.var_psislsat_dn7 = 0.0;
        locals.var_psislsat_dn8 = 0.0;
        locals.var_psislsat_dn9 = 0.0;
        locals.var_psislsat_dn10 = 0.0;
        locals.var_psislsat_dn11 = 0.0;
        locals.var_psislsat_dn14 = 0.0;

        locals.var_psisubsat = 0.0;
        locals.var_psisubsat_dn0 = 0.0;
        locals.var_psisubsat_dn2 = 0.0;
        locals.var_psisubsat_dn4 = 0.0;
        locals.var_psisubsat_dn5 = 0.0;
        locals.var_psisubsat_dn6 = 0.0;
        locals.var_psisubsat_dn7 = 0.0;
        locals.var_psisubsat_dn8 = 0.0;
        locals.var_psisubsat_dn9 = 0.0;
        locals.var_psisubsat_dn10 = 0.0;
        locals.var_psisubsat_dn11 = 0.0;
        locals.var_psisubsat_dn14 = 0.0;

        locals.var_ifn = 0.0;
        locals.var_ifn_dn0 = 0.0;
        locals.var_ifn_dn2 = 0.0;
        locals.var_ifn_dn4 = 0.0;
        locals.var_ifn_dn5 = 0.0;
        locals.var_ifn_dn6 = 0.0;
        locals.var_ifn_dn7 = 0.0;
        locals.var_ifn_dn8 = 0.0;
        locals.var_ifn_dn9 = 0.0;
        locals.var_ifn_dn10 = 0.0;
        locals.var_ifn_dn11 = 0.0;
        locals.var_ifn_dn14 = 0.0;

        locals.var_eg12 = 0.0;
        locals.var_eg12_dn0 = 0.0;
        locals.var_eg12_dn2 = 0.0;
        locals.var_eg12_dn4 = 0.0;
        locals.var_eg12_dn5 = 0.0;
        locals.var_eg12_dn6 = 0.0;
        locals.var_eg12_dn7 = 0.0;
        locals.var_eg12_dn8 = 0.0;
        locals.var_eg12_dn9 = 0.0;
        locals.var_eg12_dn10 = 0.0;
        locals.var_eg12_dn11 = 0.0;
        locals.var_eg12_dn14 = 0.0;

        locals.var_eg32 = 0.0;
        locals.var_eg32_dn0 = 0.0;
        locals.var_eg32_dn2 = 0.0;
        locals.var_eg32_dn4 = 0.0;
        locals.var_eg32_dn5 = 0.0;
        locals.var_eg32_dn6 = 0.0;
        locals.var_eg32_dn7 = 0.0;
        locals.var_eg32_dn8 = 0.0;
        locals.var_eg32_dn9 = 0.0;
        locals.var_eg32_dn10 = 0.0;
        locals.var_eg32_dn11 = 0.0;
        locals.var_eg32_dn14 = 0.0;

        locals.var_cov_slp = 0.0;

        locals.var_cov_mag = 0.0;

        locals.var_qgos = 0.0;
        locals.var_qgos_dn0 = 0.0;
        locals.var_qgos_dn2 = 0.0;
        locals.var_qgos_dn4 = 0.0;
        locals.var_qgos_dn5 = 0.0;
        locals.var_qgos_dn6 = 0.0;
        locals.var_qgos_dn7 = 0.0;
        locals.var_qgos_dn8 = 0.0;
        locals.var_qgos_dn9 = 0.0;
        locals.var_qgos_dn10 = 0.0;
        locals.var_qgos_dn11 = 0.0;
        locals.var_qgos_dn14 = 0.0;

        locals.var_qgod = 0.0;
        locals.var_qgod_dn0 = 0.0;
        locals.var_qgod_dn2 = 0.0;
        locals.var_qgod_dn4 = 0.0;
        locals.var_qgod_dn5 = 0.0;
        locals.var_qgod_dn6 = 0.0;
        locals.var_qgod_dn7 = 0.0;
        locals.var_qgod_dn8 = 0.0;
        locals.var_qgod_dn9 = 0.0;
        locals.var_qgod_dn10 = 0.0;
        locals.var_qgod_dn11 = 0.0;
        locals.var_qgod_dn14 = 0.0;

        locals.var_qgbo = 0.0;
        locals.var_qgbo_dn7 = 0.0;
        locals.var_qgbo_dn8 = 0.0;
        locals.var_qgbo_dn9 = 0.0;

        locals.var_cgbo_loc = 0.0;

        locals.var_qgso = 0.0;
        locals.var_qgso_dn2 = 0.0;
        locals.var_qgso_dn7 = 0.0;

        locals.var_qgdo = 0.0;
        locals.var_qgdo_dn0 = 0.0;
        locals.var_qgdo_dn2 = 0.0;
        locals.var_qgdo_dn7 = 0.0;

        locals.var_qfd = 0.0;
        locals.var_qfd_dn0 = 0.0;
        locals.var_qfd_dn2 = 0.0;
        locals.var_qfd_dn7 = 0.0;

        locals.var_cfd = 0.0;

        locals.var_qfs = 0.0;
        locals.var_qfs_dn2 = 0.0;
        locals.var_qfs_dn7 = 0.0;

        locals.var_cfs = 0.0;

        locals.var_ec = 0.0;
        locals.var_ec_dn0 = 0.0;
        locals.var_ec_dn2 = 0.0;
        locals.var_ec_dn4 = 0.0;
        locals.var_ec_dn5 = 0.0;
        locals.var_ec_dn6 = 0.0;
        locals.var_ec_dn7 = 0.0;
        locals.var_ec_dn8 = 0.0;
        locals.var_ec_dn9 = 0.0;
        locals.var_ec_dn10 = 0.0;
        locals.var_ec_dn11 = 0.0;
        locals.var_ec_dn14 = 0.0;

        locals.var_pslk = 0.0;
        locals.var_pslk_dn0 = 0.0;
        locals.var_pslk_dn2 = 0.0;
        locals.var_pslk_dn4 = 0.0;
        locals.var_pslk_dn5 = 0.0;
        locals.var_pslk_dn6 = 0.0;
        locals.var_pslk_dn7 = 0.0;
        locals.var_pslk_dn8 = 0.0;
        locals.var_pslk_dn9 = 0.0;
        locals.var_pslk_dn10 = 0.0;
        locals.var_pslk_dn11 = 0.0;
        locals.var_pslk_dn14 = 0.0;

        locals.var_qy = 0.0;
        locals.var_qy_dn0 = 0.0;
        locals.var_qy_dn2 = 0.0;
        locals.var_qy_dn4 = 0.0;
        locals.var_qy_dn5 = 0.0;
        locals.var_qy_dn6 = 0.0;
        locals.var_qy_dn7 = 0.0;
        locals.var_qy_dn8 = 0.0;
        locals.var_qy_dn9 = 0.0;
        locals.var_qy_dn10 = 0.0;
        locals.var_qy_dn11 = 0.0;
        locals.var_qy_dn14 = 0.0;

        locals.var_tau = 0.0;
        locals.var_tau_dn0 = 0.0;
        locals.var_tau_dn2 = 0.0;
        locals.var_tau_dn4 = 0.0;
        locals.var_tau_dn5 = 0.0;
        locals.var_tau_dn6 = 0.0;
        locals.var_tau_dn7 = 0.0;
        locals.var_tau_dn8 = 0.0;
        locals.var_tau_dn9 = 0.0;
        locals.var_tau_dn10 = 0.0;
        locals.var_tau_dn11 = 0.0;
        locals.var_tau_dn14 = 0.0;

        locals.var_taub = 0.0;
        locals.var_taub_dn0 = 0.0;
        locals.var_taub_dn2 = 0.0;
        locals.var_taub_dn4 = 0.0;
        locals.var_taub_dn5 = 0.0;
        locals.var_taub_dn6 = 0.0;
        locals.var_taub_dn7 = 0.0;
        locals.var_taub_dn8 = 0.0;
        locals.var_taub_dn9 = 0.0;
        locals.var_taub_dn10 = 0.0;
        locals.var_taub_dn11 = 0.0;
        locals.var_taub_dn14 = 0.0;

        locals.var_eyd = 0.0;
        locals.var_eyd_dn0 = 0.0;
        locals.var_eyd_dn2 = 0.0;
        locals.var_eyd_dn4 = 0.0;
        locals.var_eyd_dn5 = 0.0;
        locals.var_eyd_dn6 = 0.0;
        locals.var_eyd_dn7 = 0.0;
        locals.var_eyd_dn8 = 0.0;
        locals.var_eyd_dn9 = 0.0;
        locals.var_eyd_dn10 = 0.0;
        locals.var_eyd_dn11 = 0.0;
        locals.var_eyd_dn14 = 0.0;

        locals.var_mu_ave = 0.0;
        locals.var_mu_ave_dn0 = 0.0;
        locals.var_mu_ave_dn2 = 0.0;
        locals.var_mu_ave_dn4 = 0.0;
        locals.var_mu_ave_dn5 = 0.0;
        locals.var_mu_ave_dn6 = 0.0;
        locals.var_mu_ave_dn7 = 0.0;
        locals.var_mu_ave_dn8 = 0.0;
        locals.var_mu_ave_dn9 = 0.0;
        locals.var_mu_ave_dn10 = 0.0;
        locals.var_mu_ave_dn11 = 0.0;
        locals.var_mu_ave_dn14 = 0.0;

        locals.var_nthrml = 0.0;
        locals.var_nthrml_dn0 = 0.0;
        locals.var_nthrml_dn2 = 0.0;
        locals.var_nthrml_dn4 = 0.0;
        locals.var_nthrml_dn5 = 0.0;
        locals.var_nthrml_dn6 = 0.0;
        locals.var_nthrml_dn7 = 0.0;
        locals.var_nthrml_dn8 = 0.0;
        locals.var_nthrml_dn9 = 0.0;
        locals.var_nthrml_dn10 = 0.0;
        locals.var_nthrml_dn11 = 0.0;
        locals.var_nthrml_dn14 = 0.0;

        locals.var_mud_hoso = 0.0;
        locals.var_mud_hoso_dn0 = 0.0;
        locals.var_mud_hoso_dn2 = 0.0;
        locals.var_mud_hoso_dn4 = 0.0;
        locals.var_mud_hoso_dn5 = 0.0;
        locals.var_mud_hoso_dn6 = 0.0;
        locals.var_mud_hoso_dn7 = 0.0;
        locals.var_mud_hoso_dn8 = 0.0;
        locals.var_mud_hoso_dn9 = 0.0;
        locals.var_mud_hoso_dn10 = 0.0;
        locals.var_mud_hoso_dn11 = 0.0;
        locals.var_mud_hoso_dn14 = 0.0;

        locals.var_kusai00 = 0.0;
        locals.var_kusai00_dn0 = 0.0;
        locals.var_kusai00_dn2 = 0.0;
        locals.var_kusai00_dn4 = 0.0;
        locals.var_kusai00_dn5 = 0.0;
        locals.var_kusai00_dn6 = 0.0;
        locals.var_kusai00_dn7 = 0.0;
        locals.var_kusai00_dn8 = 0.0;
        locals.var_kusai00_dn9 = 0.0;
        locals.var_kusai00_dn10 = 0.0;
        locals.var_kusai00_dn11 = 0.0;
        locals.var_kusai00_dn14 = 0.0;

        locals.var_kusaidd = 0.0;
        locals.var_kusaidd_dn0 = 0.0;
        locals.var_kusaidd_dn2 = 0.0;
        locals.var_kusaidd_dn4 = 0.0;
        locals.var_kusaidd_dn5 = 0.0;
        locals.var_kusaidd_dn6 = 0.0;
        locals.var_kusaidd_dn7 = 0.0;
        locals.var_kusaidd_dn8 = 0.0;
        locals.var_kusaidd_dn9 = 0.0;
        locals.var_kusaidd_dn10 = 0.0;
        locals.var_kusaidd_dn11 = 0.0;
        locals.var_kusaidd_dn14 = 0.0;

        locals.var_kusail = 0.0;
        locals.var_kusail_dn0 = 0.0;
        locals.var_kusail_dn2 = 0.0;
        locals.var_kusail_dn4 = 0.0;
        locals.var_kusail_dn5 = 0.0;
        locals.var_kusail_dn6 = 0.0;
        locals.var_kusail_dn7 = 0.0;
        locals.var_kusail_dn8 = 0.0;
        locals.var_kusail_dn9 = 0.0;
        locals.var_kusail_dn10 = 0.0;
        locals.var_kusail_dn11 = 0.0;
        locals.var_kusail_dn14 = 0.0;

        locals.var_kusai00l = 0.0;
        locals.var_kusai00l_dn0 = 0.0;
        locals.var_kusai00l_dn2 = 0.0;
        locals.var_kusai00l_dn4 = 0.0;
        locals.var_kusai00l_dn5 = 0.0;
        locals.var_kusai00l_dn6 = 0.0;
        locals.var_kusai00l_dn7 = 0.0;
        locals.var_kusai00l_dn8 = 0.0;
        locals.var_kusai00l_dn9 = 0.0;
        locals.var_kusai00l_dn10 = 0.0;
        locals.var_kusai00l_dn11 = 0.0;
        locals.var_kusai00l_dn14 = 0.0;

        locals.var_sqrtkusail = 0.0;
        locals.var_sqrtkusail_dn0 = 0.0;
        locals.var_sqrtkusail_dn2 = 0.0;
        locals.var_sqrtkusail_dn4 = 0.0;
        locals.var_sqrtkusail_dn5 = 0.0;
        locals.var_sqrtkusail_dn6 = 0.0;
        locals.var_sqrtkusail_dn7 = 0.0;
        locals.var_sqrtkusail_dn8 = 0.0;
        locals.var_sqrtkusail_dn9 = 0.0;
        locals.var_sqrtkusail_dn10 = 0.0;
        locals.var_sqrtkusail_dn11 = 0.0;
        locals.var_sqrtkusail_dn14 = 0.0;

        locals.var_kusai_ig = 0.0;
        locals.var_kusai_ig_dn0 = 0.0;
        locals.var_kusai_ig_dn2 = 0.0;
        locals.var_kusai_ig_dn4 = 0.0;
        locals.var_kusai_ig_dn5 = 0.0;
        locals.var_kusai_ig_dn6 = 0.0;
        locals.var_kusai_ig_dn7 = 0.0;
        locals.var_kusai_ig_dn8 = 0.0;
        locals.var_kusai_ig_dn9 = 0.0;
        locals.var_kusai_ig_dn10 = 0.0;
        locals.var_kusai_ig_dn11 = 0.0;
        locals.var_kusai_ig_dn14 = 0.0;

        locals.var_gds0_ign = 0.0;
        locals.var_gds0_ign_dn0 = 0.0;
        locals.var_gds0_ign_dn2 = 0.0;
        locals.var_gds0_ign_dn4 = 0.0;
        locals.var_gds0_ign_dn5 = 0.0;
        locals.var_gds0_ign_dn6 = 0.0;
        locals.var_gds0_ign_dn7 = 0.0;
        locals.var_gds0_ign_dn8 = 0.0;
        locals.var_gds0_ign_dn9 = 0.0;
        locals.var_gds0_ign_dn10 = 0.0;
        locals.var_gds0_ign_dn11 = 0.0;
        locals.var_gds0_ign_dn14 = 0.0;

        locals.var_gds0_h2 = 0.0;
        locals.var_gds0_h2_dn0 = 0.0;
        locals.var_gds0_h2_dn2 = 0.0;
        locals.var_gds0_h2_dn4 = 0.0;
        locals.var_gds0_h2_dn5 = 0.0;
        locals.var_gds0_h2_dn6 = 0.0;
        locals.var_gds0_h2_dn7 = 0.0;
        locals.var_gds0_h2_dn8 = 0.0;
        locals.var_gds0_h2_dn9 = 0.0;
        locals.var_gds0_h2_dn10 = 0.0;
        locals.var_gds0_h2_dn11 = 0.0;
        locals.var_gds0_h2_dn14 = 0.0;

        locals.var_gamma = 0.0;
        locals.var_gamma_dn0 = 0.0;
        locals.var_gamma_dn2 = 0.0;
        locals.var_gamma_dn4 = 0.0;
        locals.var_gamma_dn5 = 0.0;
        locals.var_gamma_dn6 = 0.0;
        locals.var_gamma_dn7 = 0.0;
        locals.var_gamma_dn8 = 0.0;
        locals.var_gamma_dn9 = 0.0;
        locals.var_gamma_dn10 = 0.0;
        locals.var_gamma_dn11 = 0.0;
        locals.var_gamma_dn14 = 0.0;

        locals.var_crl_f = 0.0;
        locals.var_crl_f_dn0 = 0.0;
        locals.var_crl_f_dn2 = 0.0;
        locals.var_crl_f_dn4 = 0.0;
        locals.var_crl_f_dn5 = 0.0;
        locals.var_crl_f_dn6 = 0.0;
        locals.var_crl_f_dn7 = 0.0;
        locals.var_crl_f_dn8 = 0.0;
        locals.var_crl_f_dn9 = 0.0;
        locals.var_crl_f_dn10 = 0.0;
        locals.var_crl_f_dn11 = 0.0;
        locals.var_crl_f_dn14 = 0.0;

        locals.var_nign0 = 0.0;
        locals.var_nign0_dn0 = 0.0;
        locals.var_nign0_dn2 = 0.0;
        locals.var_nign0_dn4 = 0.0;
        locals.var_nign0_dn5 = 0.0;
        locals.var_nign0_dn6 = 0.0;
        locals.var_nign0_dn7 = 0.0;
        locals.var_nign0_dn8 = 0.0;
        locals.var_nign0_dn9 = 0.0;
        locals.var_nign0_dn10 = 0.0;
        locals.var_nign0_dn11 = 0.0;
        locals.var_nign0_dn14 = 0.0;

        locals.var_mumoda = 0.0;
        locals.var_mumoda_dn0 = 0.0;
        locals.var_mumoda_dn2 = 0.0;
        locals.var_mumoda_dn4 = 0.0;
        locals.var_mumoda_dn5 = 0.0;
        locals.var_mumoda_dn6 = 0.0;
        locals.var_mumoda_dn7 = 0.0;
        locals.var_mumoda_dn8 = 0.0;
        locals.var_mumoda_dn9 = 0.0;
        locals.var_mumoda_dn10 = 0.0;
        locals.var_mumoda_dn11 = 0.0;
        locals.var_mumoda_dn14 = 0.0;

        locals.var_mumodb = 0.0;
        locals.var_mumodb_dn0 = 0.0;
        locals.var_mumodb_dn2 = 0.0;
        locals.var_mumodb_dn4 = 0.0;
        locals.var_mumodb_dn5 = 0.0;
        locals.var_mumodb_dn6 = 0.0;
        locals.var_mumodb_dn7 = 0.0;
        locals.var_mumodb_dn8 = 0.0;
        locals.var_mumodb_dn9 = 0.0;
        locals.var_mumodb_dn10 = 0.0;
        locals.var_mumodb_dn11 = 0.0;
        locals.var_mumodb_dn14 = 0.0;

        locals.var_correct_w1 = 0.0;
        locals.var_correct_w1_dn0 = 0.0;
        locals.var_correct_w1_dn2 = 0.0;
        locals.var_correct_w1_dn4 = 0.0;
        locals.var_correct_w1_dn5 = 0.0;
        locals.var_correct_w1_dn6 = 0.0;
        locals.var_correct_w1_dn7 = 0.0;
        locals.var_correct_w1_dn8 = 0.0;
        locals.var_correct_w1_dn9 = 0.0;
        locals.var_correct_w1_dn10 = 0.0;
        locals.var_correct_w1_dn11 = 0.0;
        locals.var_correct_w1_dn14 = 0.0;

        locals.var_tx = 0.0;
        locals.var_tx_dn0 = 0.0;
        locals.var_tx_dn2 = 0.0;
        locals.var_tx_dn4 = 0.0;
        locals.var_tx_dn5 = 0.0;
        locals.var_tx_dn6 = 0.0;
        locals.var_tx_dn7 = 0.0;
        locals.var_tx_dn8 = 0.0;
        locals.var_tx_dn9 = 0.0;
        locals.var_tx_dn10 = 0.0;
        locals.var_tx_dn11 = 0.0;
        locals.var_tx_dn14 = 0.0;

        locals.var_ty = 0.0;
        locals.var_ty_dn0 = 0.0;
        locals.var_ty_dn2 = 0.0;
        locals.var_ty_dn4 = 0.0;
        locals.var_ty_dn5 = 0.0;
        locals.var_ty_dn6 = 0.0;
        locals.var_ty_dn7 = 0.0;
        locals.var_ty_dn8 = 0.0;
        locals.var_ty_dn9 = 0.0;
        locals.var_ty_dn10 = 0.0;
        locals.var_ty_dn11 = 0.0;
        locals.var_ty_dn14 = 0.0;

        locals.var_t0 = 0.0;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn14 = 0.0;

        locals.var_t1 = 0.0;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;

        locals.var_t2 = 0.0;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn14 = 0.0;

        locals.var_t3 = 0.0;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_6(
        locals: &mut StampLocals,
    ) {
        locals.var_t4 = 0.0;
        locals.var_t4_dn0 = 0.0;
        locals.var_t4_dn2 = 0.0;
        locals.var_t4_dn4 = 0.0;
        locals.var_t4_dn5 = 0.0;
        locals.var_t4_dn6 = 0.0;
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn11 = 0.0;
        locals.var_t4_dn14 = 0.0;

        locals.var_t5 = 0.0;
        locals.var_t5_dn0 = 0.0;
        locals.var_t5_dn2 = 0.0;
        locals.var_t5_dn4 = 0.0;
        locals.var_t5_dn5 = 0.0;
        locals.var_t5_dn6 = 0.0;
        locals.var_t5_dn7 = 0.0;
        locals.var_t5_dn8 = 0.0;
        locals.var_t5_dn9 = 0.0;
        locals.var_t5_dn10 = 0.0;
        locals.var_t5_dn11 = 0.0;
        locals.var_t5_dn14 = 0.0;

        locals.var_t6 = 0.0;
        locals.var_t6_dn0 = 0.0;
        locals.var_t6_dn2 = 0.0;
        locals.var_t6_dn4 = 0.0;
        locals.var_t6_dn5 = 0.0;
        locals.var_t6_dn6 = 0.0;
        locals.var_t6_dn7 = 0.0;
        locals.var_t6_dn8 = 0.0;
        locals.var_t6_dn9 = 0.0;
        locals.var_t6_dn10 = 0.0;
        locals.var_t6_dn11 = 0.0;
        locals.var_t6_dn14 = 0.0;

        locals.var_t7 = 0.0;
        locals.var_t7_dn0 = 0.0;
        locals.var_t7_dn2 = 0.0;
        locals.var_t7_dn4 = 0.0;
        locals.var_t7_dn5 = 0.0;
        locals.var_t7_dn6 = 0.0;
        locals.var_t7_dn7 = 0.0;
        locals.var_t7_dn8 = 0.0;
        locals.var_t7_dn9 = 0.0;
        locals.var_t7_dn10 = 0.0;
        locals.var_t7_dn11 = 0.0;
        locals.var_t7_dn14 = 0.0;

        locals.var_t8 = 0.0;
        locals.var_t8_dn0 = 0.0;
        locals.var_t8_dn2 = 0.0;
        locals.var_t8_dn4 = 0.0;
        locals.var_t8_dn5 = 0.0;
        locals.var_t8_dn6 = 0.0;
        locals.var_t8_dn7 = 0.0;
        locals.var_t8_dn8 = 0.0;
        locals.var_t8_dn9 = 0.0;
        locals.var_t8_dn10 = 0.0;
        locals.var_t8_dn11 = 0.0;
        locals.var_t8_dn14 = 0.0;

        locals.var_t9 = 0.0;
        locals.var_t9_dn0 = 0.0;
        locals.var_t9_dn2 = 0.0;
        locals.var_t9_dn4 = 0.0;
        locals.var_t9_dn5 = 0.0;
        locals.var_t9_dn6 = 0.0;
        locals.var_t9_dn7 = 0.0;
        locals.var_t9_dn8 = 0.0;
        locals.var_t9_dn9 = 0.0;
        locals.var_t9_dn10 = 0.0;
        locals.var_t9_dn11 = 0.0;
        locals.var_t9_dn14 = 0.0;

        locals.var_t10 = 0.0;
        locals.var_t10_dn0 = 0.0;
        locals.var_t10_dn2 = 0.0;
        locals.var_t10_dn4 = 0.0;
        locals.var_t10_dn5 = 0.0;
        locals.var_t10_dn6 = 0.0;
        locals.var_t10_dn7 = 0.0;
        locals.var_t10_dn8 = 0.0;
        locals.var_t10_dn9 = 0.0;
        locals.var_t10_dn10 = 0.0;
        locals.var_t10_dn11 = 0.0;
        locals.var_t10_dn14 = 0.0;

        locals.var_t11 = 0.0;
        locals.var_t11_dn0 = 0.0;
        locals.var_t11_dn2 = 0.0;
        locals.var_t11_dn4 = 0.0;
        locals.var_t11_dn5 = 0.0;
        locals.var_t11_dn6 = 0.0;
        locals.var_t11_dn7 = 0.0;
        locals.var_t11_dn8 = 0.0;
        locals.var_t11_dn9 = 0.0;
        locals.var_t11_dn10 = 0.0;
        locals.var_t11_dn11 = 0.0;
        locals.var_t11_dn14 = 0.0;

        locals.var_t12 = 0.0;
        locals.var_t12_dn0 = 0.0;
        locals.var_t12_dn2 = 0.0;
        locals.var_t12_dn4 = 0.0;
        locals.var_t12_dn5 = 0.0;
        locals.var_t12_dn6 = 0.0;
        locals.var_t12_dn7 = 0.0;
        locals.var_t12_dn8 = 0.0;
        locals.var_t12_dn9 = 0.0;
        locals.var_t12_dn10 = 0.0;
        locals.var_t12_dn11 = 0.0;
        locals.var_t12_dn14 = 0.0;

        locals.var_vdseff = 0.0;
        locals.var_vdseff_dn0 = 0.0;
        locals.var_vdseff_dn2 = 0.0;
        locals.var_vdseff_dn4 = 0.0;
        locals.var_vdseff_dn5 = 0.0;
        locals.var_vdseff_dn6 = 0.0;
        locals.var_vdseff_dn7 = 0.0;
        locals.var_vdseff_dn8 = 0.0;
        locals.var_vdseff_dn9 = 0.0;
        locals.var_vdseff_dn10 = 0.0;
        locals.var_vdseff_dn11 = 0.0;
        locals.var_vdseff_dn14 = 0.0;

        locals.var_vdsorg = 0.0;
        locals.var_vdsorg_dn0 = 0.0;
        locals.var_vdsorg_dn2 = 0.0;
        locals.var_vdsorg_dn4 = 0.0;
        locals.var_vdsorg_dn5 = 0.0;
        locals.var_vdsorg_dn6 = 0.0;
        locals.var_vdsorg_dn7 = 0.0;
        locals.var_vdsorg_dn8 = 0.0;
        locals.var_vdsorg_dn9 = 0.0;
        locals.var_vdsorg_dn10 = 0.0;
        locals.var_vdsorg_dn11 = 0.0;
        locals.var_vdsorg_dn14 = 0.0;

        locals.var_qovdext = 0.0;
        locals.var_qovdext_dn0 = 0.0;
        locals.var_qovdext_dn2 = 0.0;
        locals.var_qovdext_dn4 = 0.0;
        locals.var_qovdext_dn5 = 0.0;
        locals.var_qovdext_dn6 = 0.0;
        locals.var_qovdext_dn7 = 0.0;
        locals.var_qovdext_dn8 = 0.0;
        locals.var_qovdext_dn9 = 0.0;
        locals.var_qovdext_dn10 = 0.0;
        locals.var_qovdext_dn11 = 0.0;
        locals.var_qovdext_dn14 = 0.0;

        locals.var_qovsext = 0.0;
        locals.var_qovsext_dn0 = 0.0;
        locals.var_qovsext_dn2 = 0.0;
        locals.var_qovsext_dn4 = 0.0;
        locals.var_qovsext_dn5 = 0.0;
        locals.var_qovsext_dn6 = 0.0;
        locals.var_qovsext_dn7 = 0.0;
        locals.var_qovsext_dn8 = 0.0;
        locals.var_qovsext_dn9 = 0.0;
        locals.var_qovsext_dn10 = 0.0;
        locals.var_qovsext_dn11 = 0.0;
        locals.var_qovsext_dn14 = 0.0;

        locals.var_qovd = 0.0;
        locals.var_qovd_dn0 = 0.0;
        locals.var_qovd_dn2 = 0.0;
        locals.var_qovd_dn4 = 0.0;
        locals.var_qovd_dn5 = 0.0;
        locals.var_qovd_dn6 = 0.0;
        locals.var_qovd_dn7 = 0.0;
        locals.var_qovd_dn8 = 0.0;
        locals.var_qovd_dn9 = 0.0;
        locals.var_qovd_dn10 = 0.0;
        locals.var_qovd_dn11 = 0.0;
        locals.var_qovd_dn14 = 0.0;

        locals.var_qovs = 0.0;
        locals.var_qovs_dn0 = 0.0;
        locals.var_qovs_dn2 = 0.0;
        locals.var_qovs_dn4 = 0.0;
        locals.var_qovs_dn5 = 0.0;
        locals.var_qovs_dn6 = 0.0;
        locals.var_qovs_dn7 = 0.0;
        locals.var_qovs_dn8 = 0.0;
        locals.var_qovs_dn9 = 0.0;
        locals.var_qovs_dn10 = 0.0;
        locals.var_qovs_dn11 = 0.0;
        locals.var_qovs_dn14 = 0.0;

        locals.var_qbuld = 0.0;
        locals.var_qbuld_dn0 = 0.0;
        locals.var_qbuld_dn2 = 0.0;
        locals.var_qbuld_dn4 = 0.0;
        locals.var_qbuld_dn5 = 0.0;
        locals.var_qbuld_dn6 = 0.0;
        locals.var_qbuld_dn7 = 0.0;
        locals.var_qbuld_dn8 = 0.0;
        locals.var_qbuld_dn9 = 0.0;
        locals.var_qbuld_dn10 = 0.0;
        locals.var_qbuld_dn11 = 0.0;
        locals.var_qbuld_dn14 = 0.0;

        locals.var_qbdld = 0.0;
        locals.var_qbdld_dn0 = 0.0;
        locals.var_qbdld_dn2 = 0.0;
        locals.var_qbdld_dn4 = 0.0;
        locals.var_qbdld_dn5 = 0.0;
        locals.var_qbdld_dn6 = 0.0;
        locals.var_qbdld_dn7 = 0.0;
        locals.var_qbdld_dn8 = 0.0;
        locals.var_qbdld_dn9 = 0.0;
        locals.var_qbdld_dn10 = 0.0;
        locals.var_qbdld_dn11 = 0.0;
        locals.var_qbdld_dn14 = 0.0;

        locals.var_qbsld = 0.0;
        locals.var_qbsld_dn0 = 0.0;
        locals.var_qbsld_dn2 = 0.0;
        locals.var_qbsld_dn4 = 0.0;
        locals.var_qbsld_dn5 = 0.0;
        locals.var_qbsld_dn6 = 0.0;
        locals.var_qbsld_dn7 = 0.0;
        locals.var_qbsld_dn8 = 0.0;
        locals.var_qbsld_dn9 = 0.0;
        locals.var_qbsld_dn10 = 0.0;
        locals.var_qbsld_dn11 = 0.0;
        locals.var_qbsld_dn14 = 0.0;

        locals.var_qodad = 0.0;
        locals.var_qodad_dn0 = 0.0;
        locals.var_qodad_dn2 = 0.0;
        locals.var_qodad_dn4 = 0.0;
        locals.var_qodad_dn5 = 0.0;
        locals.var_qodad_dn6 = 0.0;
        locals.var_qodad_dn7 = 0.0;
        locals.var_qodad_dn8 = 0.0;
        locals.var_qodad_dn9 = 0.0;
        locals.var_qodad_dn10 = 0.0;
        locals.var_qodad_dn11 = 0.0;
        locals.var_qodad_dn14 = 0.0;

        locals.var_qbdldext = 0.0;
        locals.var_qbdldext_dn0 = 0.0;
        locals.var_qbdldext_dn2 = 0.0;
        locals.var_qbdldext_dn4 = 0.0;
        locals.var_qbdldext_dn5 = 0.0;
        locals.var_qbdldext_dn6 = 0.0;
        locals.var_qbdldext_dn7 = 0.0;
        locals.var_qbdldext_dn8 = 0.0;
        locals.var_qbdldext_dn9 = 0.0;
        locals.var_qbdldext_dn10 = 0.0;
        locals.var_qbdldext_dn11 = 0.0;
        locals.var_qbdldext_dn14 = 0.0;

        locals.var_qbsldext = 0.0;
        locals.var_qbsldext_dn0 = 0.0;
        locals.var_qbsldext_dn2 = 0.0;
        locals.var_qbsldext_dn4 = 0.0;
        locals.var_qbsldext_dn5 = 0.0;
        locals.var_qbsldext_dn6 = 0.0;
        locals.var_qbsldext_dn7 = 0.0;
        locals.var_qbsldext_dn8 = 0.0;
        locals.var_qbsldext_dn9 = 0.0;
        locals.var_qbsldext_dn10 = 0.0;
        locals.var_qbsldext_dn11 = 0.0;
        locals.var_qbsldext_dn14 = 0.0;

        locals.var_vbsz2 = 0.0;
        locals.var_vbsz2_dn0 = 0.0;
        locals.var_vbsz2_dn2 = 0.0;
        locals.var_vbsz2_dn4 = 0.0;
        locals.var_vbsz2_dn5 = 0.0;
        locals.var_vbsz2_dn6 = 0.0;
        locals.var_vbsz2_dn7 = 0.0;
        locals.var_vbsz2_dn8 = 0.0;
        locals.var_vbsz2_dn9 = 0.0;
        locals.var_vbsz2_dn10 = 0.0;
        locals.var_vbsz2_dn11 = 0.0;
        locals.var_vbsz2_dn14 = 0.0;

        locals.var_rdrift = 0.0;
        locals.var_rdrift_dn0 = 0.0;
        locals.var_rdrift_dn2 = 0.0;
        locals.var_rdrift_dn4 = 0.0;
        locals.var_rdrift_dn5 = 0.0;
        locals.var_rdrift_dn6 = 0.0;
        locals.var_rdrift_dn7 = 0.0;
        locals.var_rdrift_dn8 = 0.0;
        locals.var_rdrift_dn9 = 0.0;
        locals.var_rdrift_dn10 = 0.0;
        locals.var_rdrift_dn11 = 0.0;
        locals.var_rdrift_dn14 = 0.0;

        locals.var_rsdrift = 0.0;
        locals.var_rsdrift_dn0 = 0.0;
        locals.var_rsdrift_dn2 = 0.0;
        locals.var_rsdrift_dn4 = 0.0;
        locals.var_rsdrift_dn5 = 0.0;
        locals.var_rsdrift_dn6 = 0.0;
        locals.var_rsdrift_dn7 = 0.0;
        locals.var_rsdrift_dn8 = 0.0;
        locals.var_rsdrift_dn9 = 0.0;
        locals.var_rsdrift_dn10 = 0.0;
        locals.var_rsdrift_dn11 = 0.0;
        locals.var_rsdrift_dn14 = 0.0;

        locals.var_ra = 0.0;
        locals.var_ra_dn0 = 0.0;
        locals.var_ra_dn2 = 0.0;
        locals.var_ra_dn4 = 0.0;
        locals.var_ra_dn5 = 0.0;
        locals.var_ra_dn6 = 0.0;
        locals.var_ra_dn7 = 0.0;
        locals.var_ra_dn8 = 0.0;
        locals.var_ra_dn9 = 0.0;
        locals.var_ra_dn10 = 0.0;
        locals.var_ra_dn11 = 0.0;
        locals.var_ra_dn14 = 0.0;

        locals.var_vdse_eff = 0.0;
        locals.var_vdse_eff_dn0 = 0.0;
        locals.var_vdse_eff_dn2 = 0.0;

        locals.var_vdsemodenml = 0.0;

        locals.var_vdsemodervs = 0.0;

        locals.var_vbsegmt = 0.0;
        locals.var_vbsegmt_dn2 = 0.0;
        locals.var_vbsegmt_dn9 = 0.0;

        locals.var_vdsegmt = 0.0;
        locals.var_vdsegmt_dn0 = 0.0;
        locals.var_vdsegmt_dn2 = 0.0;

        locals.var_vgsegmt = 0.0;
        locals.var_vgsegmt_dn2 = 0.0;
        locals.var_vgsegmt_dn7 = 0.0;

        locals.var_vbserev = 0.0;
        locals.var_vbserev_dn0 = 0.0;
        locals.var_vbserev_dn2 = 0.0;
        locals.var_vbserev_dn9 = 0.0;

        locals.var_vdserev = 0.0;
        locals.var_vdserev_dn0 = 0.0;
        locals.var_vdserev_dn2 = 0.0;

        locals.var_vgserev = 0.0;
        locals.var_vgserev_dn0 = 0.0;
        locals.var_vgserev_dn2 = 0.0;
        locals.var_vgserev_dn7 = 0.0;

        locals.var_vdserevz = 0.0;
        locals.var_vdserevz_dn0 = 0.0;
        locals.var_vdserevz_dn2 = 0.0;
        locals.var_vdserevz_dn4 = 0.0;
        locals.var_vdserevz_dn5 = 0.0;
        locals.var_vdserevz_dn6 = 0.0;
        locals.var_vdserevz_dn7 = 0.0;
        locals.var_vdserevz_dn8 = 0.0;
        locals.var_vdserevz_dn9 = 0.0;
        locals.var_vdserevz_dn10 = 0.0;
        locals.var_vdserevz_dn11 = 0.0;
        locals.var_vdserevz_dn14 = 0.0;

        locals.var_vgserevz = 0.0;
        locals.var_vgserevz_dn0 = 0.0;
        locals.var_vgserevz_dn2 = 0.0;
        locals.var_vgserevz_dn4 = 0.0;
        locals.var_vgserevz_dn5 = 0.0;
        locals.var_vgserevz_dn6 = 0.0;
        locals.var_vgserevz_dn7 = 0.0;
        locals.var_vgserevz_dn8 = 0.0;
        locals.var_vgserevz_dn9 = 0.0;
        locals.var_vgserevz_dn10 = 0.0;
        locals.var_vgserevz_dn11 = 0.0;
        locals.var_vgserevz_dn14 = 0.0;

        locals.var_vbserevz = 0.0;
        locals.var_vbserevz_dn0 = 0.0;
        locals.var_vbserevz_dn2 = 0.0;
        locals.var_vbserevz_dn4 = 0.0;
        locals.var_vbserevz_dn5 = 0.0;
        locals.var_vbserevz_dn6 = 0.0;
        locals.var_vbserevz_dn7 = 0.0;
        locals.var_vbserevz_dn8 = 0.0;
        locals.var_vbserevz_dn9 = 0.0;
        locals.var_vbserevz_dn10 = 0.0;
        locals.var_vbserevz_dn11 = 0.0;
        locals.var_vbserevz_dn14 = 0.0;

        locals.var_vsubsrev = 0.0;
        locals.var_vsubsrev_dn0 = 0.0;
        locals.var_vsubsrev_dn2 = 0.0;
        locals.var_vsubsrev_dn4 = 0.0;

        locals.var_ttemp = 0.0;
        locals.var_ttemp_dn0 = 0.0;
        locals.var_ttemp_dn2 = 0.0;
        locals.var_ttemp_dn4 = 0.0;
        locals.var_ttemp_dn5 = 0.0;
        locals.var_ttemp_dn6 = 0.0;
        locals.var_ttemp_dn7 = 0.0;
        locals.var_ttemp_dn8 = 0.0;
        locals.var_ttemp_dn9 = 0.0;
        locals.var_ttemp_dn10 = 0.0;
        locals.var_ttemp_dn11 = 0.0;
        locals.var_ttemp_dn14 = 0.0;

        locals.var_ttemp0 = 0.0;
        locals.var_ttemp0_dn0 = 0.0;
        locals.var_ttemp0_dn2 = 0.0;
        locals.var_ttemp0_dn4 = 0.0;
        locals.var_ttemp0_dn5 = 0.0;
        locals.var_ttemp0_dn6 = 0.0;
        locals.var_ttemp0_dn7 = 0.0;
        locals.var_ttemp0_dn8 = 0.0;
        locals.var_ttemp0_dn9 = 0.0;
        locals.var_ttemp0_dn10 = 0.0;
        locals.var_ttemp0_dn11 = 0.0;
        locals.var_ttemp0_dn14 = 0.0;

        locals.var_tdiff0 = 0.0;
        locals.var_tdiff0_dn0 = 0.0;
        locals.var_tdiff0_dn2 = 0.0;
        locals.var_tdiff0_dn4 = 0.0;
        locals.var_tdiff0_dn5 = 0.0;
        locals.var_tdiff0_dn6 = 0.0;
        locals.var_tdiff0_dn7 = 0.0;
        locals.var_tdiff0_dn8 = 0.0;
        locals.var_tdiff0_dn9 = 0.0;
        locals.var_tdiff0_dn10 = 0.0;
        locals.var_tdiff0_dn11 = 0.0;
        locals.var_tdiff0_dn14 = 0.0;

        locals.var_tdiff0_2 = 0.0;
        locals.var_tdiff0_2_dn0 = 0.0;
        locals.var_tdiff0_2_dn2 = 0.0;
        locals.var_tdiff0_2_dn4 = 0.0;
        locals.var_tdiff0_2_dn5 = 0.0;
        locals.var_tdiff0_2_dn6 = 0.0;
        locals.var_tdiff0_2_dn7 = 0.0;
        locals.var_tdiff0_2_dn8 = 0.0;
        locals.var_tdiff0_2_dn9 = 0.0;
        locals.var_tdiff0_2_dn10 = 0.0;
        locals.var_tdiff0_2_dn11 = 0.0;
        locals.var_tdiff0_2_dn14 = 0.0;

        locals.var_tdiff = 0.0;
        locals.var_tdiff_dn0 = 0.0;
        locals.var_tdiff_dn2 = 0.0;
        locals.var_tdiff_dn4 = 0.0;
        locals.var_tdiff_dn5 = 0.0;
        locals.var_tdiff_dn6 = 0.0;
        locals.var_tdiff_dn7 = 0.0;
        locals.var_tdiff_dn8 = 0.0;
        locals.var_tdiff_dn9 = 0.0;
        locals.var_tdiff_dn10 = 0.0;
        locals.var_tdiff_dn11 = 0.0;
        locals.var_tdiff_dn14 = 0.0;

        locals.var_tdiff_2 = 0.0;
        locals.var_tdiff_2_dn0 = 0.0;
        locals.var_tdiff_2_dn2 = 0.0;
        locals.var_tdiff_2_dn4 = 0.0;
        locals.var_tdiff_2_dn5 = 0.0;
        locals.var_tdiff_2_dn6 = 0.0;
        locals.var_tdiff_2_dn7 = 0.0;
        locals.var_tdiff_2_dn8 = 0.0;
        locals.var_tdiff_2_dn9 = 0.0;
        locals.var_tdiff_2_dn10 = 0.0;
        locals.var_tdiff_2_dn11 = 0.0;
        locals.var_tdiff_2_dn14 = 0.0;

        locals.var_eg = 0.0;
        locals.var_eg_dn0 = 0.0;
        locals.var_eg_dn2 = 0.0;
        locals.var_eg_dn4 = 0.0;
        locals.var_eg_dn5 = 0.0;
        locals.var_eg_dn6 = 0.0;
        locals.var_eg_dn7 = 0.0;
        locals.var_eg_dn8 = 0.0;
        locals.var_eg_dn9 = 0.0;
        locals.var_eg_dn10 = 0.0;
        locals.var_eg_dn11 = 0.0;
        locals.var_eg_dn14 = 0.0;

        locals.var_nin = 0.0;
        locals.var_nin_dn0 = 0.0;
        locals.var_nin_dn2 = 0.0;
        locals.var_nin_dn4 = 0.0;
        locals.var_nin_dn5 = 0.0;
        locals.var_nin_dn6 = 0.0;
        locals.var_nin_dn7 = 0.0;
        locals.var_nin_dn8 = 0.0;
        locals.var_nin_dn9 = 0.0;
        locals.var_nin_dn10 = 0.0;
        locals.var_nin_dn11 = 0.0;
        locals.var_nin_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_vgbgmt = 0.0;
        locals.var_vgbgmt_dn2 = 0.0;
        locals.var_vgbgmt_dn7 = 0.0;
        locals.var_vgbgmt_dn8 = 0.0;
        locals.var_vgbgmt_dn9 = 0.0;

        locals.var_vxbgmt = 0.0;
        locals.var_vxbgmt_dn0 = 0.0;
        locals.var_vxbgmt_dn2 = 0.0;
        locals.var_vxbgmt_dn4 = 0.0;
        locals.var_vxbgmt_dn5 = 0.0;
        locals.var_vxbgmt_dn6 = 0.0;
        locals.var_vxbgmt_dn7 = 0.0;
        locals.var_vxbgmt_dn8 = 0.0;
        locals.var_vxbgmt_dn9 = 0.0;
        locals.var_vxbgmt_dn10 = 0.0;
        locals.var_vxbgmt_dn11 = 0.0;
        locals.var_vxbgmt_dn14 = 0.0;

        locals.var_vxbgmtcl = 0.0;
        locals.var_vxbgmtcl_dn0 = 0.0;
        locals.var_vxbgmtcl_dn2 = 0.0;
        locals.var_vxbgmtcl_dn4 = 0.0;
        locals.var_vxbgmtcl_dn5 = 0.0;
        locals.var_vxbgmtcl_dn6 = 0.0;
        locals.var_vxbgmtcl_dn7 = 0.0;
        locals.var_vxbgmtcl_dn8 = 0.0;
        locals.var_vxbgmtcl_dn9 = 0.0;
        locals.var_vxbgmtcl_dn10 = 0.0;
        locals.var_vxbgmtcl_dn11 = 0.0;
        locals.var_vxbgmtcl_dn14 = 0.0;

        locals.var_qsuld = 0.0;
        locals.var_qsuld_dn0 = 0.0;
        locals.var_qsuld_dn2 = 0.0;
        locals.var_qsuld_dn4 = 0.0;
        locals.var_qsuld_dn5 = 0.0;
        locals.var_qsuld_dn6 = 0.0;
        locals.var_qsuld_dn7 = 0.0;
        locals.var_qsuld_dn8 = 0.0;
        locals.var_qsuld_dn9 = 0.0;
        locals.var_qsuld_dn10 = 0.0;
        locals.var_qsuld_dn11 = 0.0;
        locals.var_qsuld_dn14 = 0.0;

        locals.var_qiuld = 0.0;
        locals.var_qiuld_dn0 = 0.0;
        locals.var_qiuld_dn2 = 0.0;
        locals.var_qiuld_dn4 = 0.0;
        locals.var_qiuld_dn5 = 0.0;
        locals.var_qiuld_dn6 = 0.0;
        locals.var_qiuld_dn7 = 0.0;
        locals.var_qiuld_dn8 = 0.0;
        locals.var_qiuld_dn9 = 0.0;
        locals.var_qiuld_dn10 = 0.0;
        locals.var_qiuld_dn11 = 0.0;
        locals.var_qiuld_dn14 = 0.0;

        locals.var_idsibpc = 0.0;
        locals.var_idsibpc_dn0 = 0.0;
        locals.var_idsibpc_dn2 = 0.0;
        locals.var_idsibpc_dn4 = 0.0;
        locals.var_idsibpc_dn5 = 0.0;
        locals.var_idsibpc_dn6 = 0.0;
        locals.var_idsibpc_dn7 = 0.0;
        locals.var_idsibpc_dn8 = 0.0;
        locals.var_idsibpc_dn9 = 0.0;
        locals.var_idsibpc_dn10 = 0.0;
        locals.var_idsibpc_dn11 = 0.0;
        locals.var_idsibpc_dn14 = 0.0;

        locals.var_vgpld = 0.0;
        locals.var_vgpld_dn2 = 0.0;
        locals.var_vgpld_dn7 = 0.0;
        locals.var_vgpld_dn8 = 0.0;
        locals.var_vgpld_dn9 = 0.0;

        locals.var_vgb_fb_ld = 0.0;

        locals.var_ps0ld = 0.0;
        locals.var_ps0ld_dn0 = 0.0;
        locals.var_ps0ld_dn2 = 0.0;
        locals.var_ps0ld_dn4 = 0.0;
        locals.var_ps0ld_dn5 = 0.0;
        locals.var_ps0ld_dn6 = 0.0;
        locals.var_ps0ld_dn7 = 0.0;
        locals.var_ps0ld_dn8 = 0.0;
        locals.var_ps0ld_dn9 = 0.0;
        locals.var_ps0ld_dn10 = 0.0;
        locals.var_ps0ld_dn11 = 0.0;
        locals.var_ps0ld_dn14 = 0.0;

        locals.var_cnst1over = 0.0;
        locals.var_cnst1over_dn0 = 0.0;
        locals.var_cnst1over_dn2 = 0.0;
        locals.var_cnst1over_dn4 = 0.0;
        locals.var_cnst1over_dn5 = 0.0;
        locals.var_cnst1over_dn6 = 0.0;
        locals.var_cnst1over_dn7 = 0.0;
        locals.var_cnst1over_dn8 = 0.0;
        locals.var_cnst1over_dn9 = 0.0;
        locals.var_cnst1over_dn10 = 0.0;
        locals.var_cnst1over_dn11 = 0.0;
        locals.var_cnst1over_dn14 = 0.0;

        locals.var_ddriftld = p.p334;
        locals.var_ddriftld_dn0 = 0.0;
        locals.var_ddriftld_dn2 = 0.0;
        locals.var_ddriftld_dn4 = 0.0;
        locals.var_ddriftld_dn5 = 0.0;
        locals.var_ddriftld_dn6 = 0.0;
        locals.var_ddriftld_dn7 = 0.0;
        locals.var_ddriftld_dn8 = 0.0;
        locals.var_ddriftld_dn9 = 0.0;
        locals.var_ddriftld_dn10 = 0.0;
        locals.var_ddriftld_dn11 = 0.0;
        locals.var_ddriftld_dn14 = 0.0;

        locals.var_ddriftldc = p.p334;
        locals.var_ddriftldc_dn0 = 0.0;
        locals.var_ddriftldc_dn2 = 0.0;
        locals.var_ddriftldc_dn4 = 0.0;
        locals.var_ddriftldc_dn5 = 0.0;
        locals.var_ddriftldc_dn6 = 0.0;
        locals.var_ddriftldc_dn7 = 0.0;
        locals.var_ddriftldc_dn8 = 0.0;
        locals.var_ddriftldc_dn9 = 0.0;
        locals.var_ddriftldc_dn10 = 0.0;
        locals.var_ddriftldc_dn11 = 0.0;
        locals.var_ddriftldc_dn14 = 0.0;

        locals.var_nover_func = 0.0;

        locals.var_cnst0over_func = 0.0;
        locals.var_cnst0over_func_dn0 = 0.0;
        locals.var_cnst0over_func_dn2 = 0.0;
        locals.var_cnst0over_func_dn4 = 0.0;
        locals.var_cnst0over_func_dn5 = 0.0;
        locals.var_cnst0over_func_dn6 = 0.0;
        locals.var_cnst0over_func_dn7 = 0.0;
        locals.var_cnst0over_func_dn8 = 0.0;
        locals.var_cnst0over_func_dn9 = 0.0;
        locals.var_cnst0over_func_dn10 = 0.0;
        locals.var_cnst0over_func_dn11 = 0.0;
        locals.var_cnst0over_func_dn14 = 0.0;

        locals.var_ta = 0.0093868;

        let assign3340_e1739: f64 = (-0.1047839);
        locals.var_tb = assign3340_e1739;

        locals.var_chi_1 = 0.0;
        locals.var_chi_1_dn0 = 0.0;
        locals.var_chi_1_dn2 = 0.0;
        locals.var_chi_1_dn4 = 0.0;
        locals.var_chi_1_dn5 = 0.0;
        locals.var_chi_1_dn6 = 0.0;
        locals.var_chi_1_dn7 = 0.0;
        locals.var_chi_1_dn8 = 0.0;
        locals.var_chi_1_dn9 = 0.0;
        locals.var_chi_1_dn10 = 0.0;
        locals.var_chi_1_dn11 = 0.0;
        locals.var_chi_1_dn14 = 0.0;

        locals.var_mueph = 0.0;
        locals.var_mueph_dn0 = 0.0;
        locals.var_mueph_dn2 = 0.0;
        locals.var_mueph_dn4 = 0.0;
        locals.var_mueph_dn5 = 0.0;
        locals.var_mueph_dn6 = 0.0;
        locals.var_mueph_dn7 = 0.0;
        locals.var_mueph_dn8 = 0.0;
        locals.var_mueph_dn9 = 0.0;
        locals.var_mueph_dn10 = 0.0;
        locals.var_mueph_dn11 = 0.0;
        locals.var_mueph_dn14 = 0.0;

        locals.var_dl = 0.0;

        locals.var_dlld = 0.0;

        locals.var_lg = 0.0;

        locals.var_dw = 0.0;

        locals.var_dwld = 0.0;

        locals.var_dwcv = 0.0;

        locals.var_wg = 0.0;

        locals.var_wlg = 0.0;

        locals.var_lgate = 0.0;

        locals.var_wgate = 0.0;

        locals.var_nsubpp = 0.0;
        locals.var_nsubpp_dn0 = 0.0;
        locals.var_nsubpp_dn2 = 0.0;
        locals.var_nsubpp_dn4 = 0.0;
        locals.var_nsubpp_dn5 = 0.0;
        locals.var_nsubpp_dn6 = 0.0;
        locals.var_nsubpp_dn7 = 0.0;
        locals.var_nsubpp_dn8 = 0.0;
        locals.var_nsubpp_dn9 = 0.0;
        locals.var_nsubpp_dn10 = 0.0;
        locals.var_nsubpp_dn11 = 0.0;
        locals.var_nsubpp_dn14 = 0.0;

        locals.var_nsubps = 0.0;
        locals.var_nsubps_dn0 = 0.0;
        locals.var_nsubps_dn2 = 0.0;
        locals.var_nsubps_dn4 = 0.0;
        locals.var_nsubps_dn5 = 0.0;
        locals.var_nsubps_dn6 = 0.0;
        locals.var_nsubps_dn7 = 0.0;
        locals.var_nsubps_dn8 = 0.0;
        locals.var_nsubps_dn9 = 0.0;
        locals.var_nsubps_dn10 = 0.0;
        locals.var_nsubps_dn11 = 0.0;
        locals.var_nsubps_dn14 = 0.0;

        locals.var_nsub = 0.0;
        locals.var_nsub_dn0 = 0.0;
        locals.var_nsub_dn2 = 0.0;
        locals.var_nsub_dn4 = 0.0;
        locals.var_nsub_dn5 = 0.0;
        locals.var_nsub_dn6 = 0.0;
        locals.var_nsub_dn7 = 0.0;
        locals.var_nsub_dn8 = 0.0;
        locals.var_nsub_dn9 = 0.0;
        locals.var_nsub_dn10 = 0.0;
        locals.var_nsub_dn11 = 0.0;
        locals.var_nsub_dn14 = 0.0;

        locals.var_nsubb = 0.0;
        locals.var_nsubb_dn0 = 0.0;
        locals.var_nsubb_dn2 = 0.0;
        locals.var_nsubb_dn4 = 0.0;
        locals.var_nsubb_dn5 = 0.0;
        locals.var_nsubb_dn6 = 0.0;
        locals.var_nsubb_dn7 = 0.0;
        locals.var_nsubb_dn8 = 0.0;
        locals.var_nsubb_dn9 = 0.0;
        locals.var_nsubb_dn10 = 0.0;
        locals.var_nsubb_dn11 = 0.0;
        locals.var_nsubb_dn14 = 0.0;

        locals.var_lod_half = 0.0;
        locals.var_lod_half_dn0 = 0.0;
        locals.var_lod_half_dn2 = 0.0;
        locals.var_lod_half_dn4 = 0.0;
        locals.var_lod_half_dn5 = 0.0;
        locals.var_lod_half_dn6 = 0.0;
        locals.var_lod_half_dn7 = 0.0;
        locals.var_lod_half_dn8 = 0.0;
        locals.var_lod_half_dn9 = 0.0;
        locals.var_lod_half_dn10 = 0.0;
        locals.var_lod_half_dn11 = 0.0;
        locals.var_lod_half_dn14 = 0.0;

        locals.var_lod_half_ref = 0.0;
        locals.var_lod_half_ref_dn0 = 0.0;
        locals.var_lod_half_ref_dn2 = 0.0;
        locals.var_lod_half_ref_dn4 = 0.0;
        locals.var_lod_half_ref_dn5 = 0.0;
        locals.var_lod_half_ref_dn6 = 0.0;
        locals.var_lod_half_ref_dn7 = 0.0;
        locals.var_lod_half_ref_dn8 = 0.0;
        locals.var_lod_half_ref_dn9 = 0.0;
        locals.var_lod_half_ref_dn10 = 0.0;
        locals.var_lod_half_ref_dn11 = 0.0;
        locals.var_lod_half_ref_dn14 = 0.0;

        locals.var_log_tratio = 0.0;
        locals.var_log_tratio_dn0 = 0.0;
        locals.var_log_tratio_dn2 = 0.0;
        locals.var_log_tratio_dn4 = 0.0;
        locals.var_log_tratio_dn5 = 0.0;
        locals.var_log_tratio_dn6 = 0.0;
        locals.var_log_tratio_dn7 = 0.0;
        locals.var_log_tratio_dn8 = 0.0;
        locals.var_log_tratio_dn9 = 0.0;
        locals.var_log_tratio_dn10 = 0.0;
        locals.var_log_tratio_dn11 = 0.0;
        locals.var_log_tratio_dn14 = 0.0;

        locals.var_edri = 0.0;
        locals.var_edri_dn0 = 0.0;
        locals.var_edri_dn2 = 0.0;
        locals.var_edri_dn4 = 0.0;
        locals.var_edri_dn5 = 0.0;
        locals.var_edri_dn6 = 0.0;
        locals.var_edri_dn7 = 0.0;
        locals.var_edri_dn8 = 0.0;
        locals.var_edri_dn9 = 0.0;
        locals.var_edri_dn10 = 0.0;
        locals.var_edri_dn11 = 0.0;
        locals.var_edri_dn14 = 0.0;

        locals.var_vdri = 0.0;
        locals.var_vdri_dn0 = 0.0;
        locals.var_vdri_dn2 = 0.0;
        locals.var_vdri_dn4 = 0.0;
        locals.var_vdri_dn5 = 0.0;
        locals.var_vdri_dn6 = 0.0;
        locals.var_vdri_dn7 = 0.0;
        locals.var_vdri_dn8 = 0.0;
        locals.var_vdri_dn9 = 0.0;
        locals.var_vdri_dn10 = 0.0;
        locals.var_vdri_dn11 = 0.0;
        locals.var_vdri_dn14 = 0.0;

        locals.var_mu0 = 0.0;
        locals.var_mu0_dn0 = 0.0;
        locals.var_mu0_dn2 = 0.0;
        locals.var_mu0_dn4 = 0.0;
        locals.var_mu0_dn5 = 0.0;
        locals.var_mu0_dn6 = 0.0;
        locals.var_mu0_dn7 = 0.0;
        locals.var_mu0_dn8 = 0.0;
        locals.var_mu0_dn9 = 0.0;
        locals.var_mu0_dn10 = 0.0;
        locals.var_mu0_dn11 = 0.0;
        locals.var_mu0_dn14 = 0.0;

        locals.var_cx = 0.0;

        locals.var_car = 0.0;

        locals.var_xov = 0.0;
        locals.var_xov_dn0 = 0.0;
        locals.var_xov_dn2 = 0.0;
        locals.var_xov_dn4 = 0.0;
        locals.var_xov_dn5 = 0.0;
        locals.var_xov_dn6 = 0.0;
        locals.var_xov_dn7 = 0.0;
        locals.var_xov_dn8 = 0.0;
        locals.var_xov_dn9 = 0.0;
        locals.var_xov_dn10 = 0.0;
        locals.var_xov_dn11 = 0.0;
        locals.var_xov_dn14 = 0.0;

        locals.var_carr = 0.0;
        locals.var_carr_dn0 = 0.0;
        locals.var_carr_dn2 = 0.0;
        locals.var_carr_dn4 = 0.0;
        locals.var_carr_dn5 = 0.0;
        locals.var_carr_dn6 = 0.0;
        locals.var_carr_dn7 = 0.0;
        locals.var_carr_dn8 = 0.0;
        locals.var_carr_dn9 = 0.0;
        locals.var_carr_dn10 = 0.0;
        locals.var_carr_dn11 = 0.0;
        locals.var_carr_dn14 = 0.0;

        locals.var_gd = 0.0;
        locals.var_gd_dn0 = 0.0;
        locals.var_gd_dn2 = 0.0;
        locals.var_gd_dn4 = 0.0;
        locals.var_gd_dn5 = 0.0;
        locals.var_gd_dn6 = 0.0;
        locals.var_gd_dn7 = 0.0;
        locals.var_gd_dn8 = 0.0;
        locals.var_gd_dn9 = 0.0;
        locals.var_gd_dn10 = 0.0;
        locals.var_gd_dn11 = 0.0;
        locals.var_gd_dn14 = 0.0;

        locals.var_vddpz = 0.0;
        locals.var_vddpz_dn0 = 0.0;
        locals.var_vddpz_dn2 = 0.0;
        locals.var_vddpz_dn4 = 0.0;
        locals.var_vddpz_dn5 = 0.0;
        locals.var_vddpz_dn6 = 0.0;
        locals.var_vddpz_dn7 = 0.0;
        locals.var_vddpz_dn8 = 0.0;
        locals.var_vddpz_dn9 = 0.0;
        locals.var_vddpz_dn10 = 0.0;
        locals.var_vddpz_dn11 = 0.0;
        locals.var_vddpz_dn14 = 0.0;

        locals.var_arg = 0.0;
        locals.var_arg_dn0 = 0.0;
        locals.var_arg_dn2 = 0.0;
        locals.var_arg_dn4 = 0.0;
        locals.var_arg_dn5 = 0.0;
        locals.var_arg_dn6 = 0.0;
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = 0.0;
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;
        locals.var_arg_dn14 = 0.0;

        locals.var_vbd = 0.0;
        locals.var_vbd_dn6 = 0.0;
        locals.var_vbd_dn8 = 0.0;
        locals.var_vbd_dn9 = 0.0;

        locals.var_vbsi = 0.0;
        locals.var_vbsi_dn8 = 0.0;
        locals.var_vbsi_dn9 = 0.0;

        locals.var_vdsi = 0.0;
        locals.var_vdsi_dn6 = 0.0;
        locals.var_vdsi_dn8 = 0.0;

        locals.var_vgd = 0.0;
        locals.var_vgd_dn6 = 0.0;
        locals.var_vgd_dn7 = 0.0;
        locals.var_vgd_dn8 = 0.0;

        locals.var_vgsi = 0.0;
        locals.var_vgsi_dn7 = 0.0;
        locals.var_vgsi_dn8 = 0.0;

        locals.var_deltemp = 0.0;
        locals.var_deltemp_dn0 = 0.0;
        locals.var_deltemp_dn2 = 0.0;
        locals.var_deltemp_dn4 = 0.0;
        locals.var_deltemp_dn5 = 0.0;
        locals.var_deltemp_dn6 = 0.0;
        locals.var_deltemp_dn7 = 0.0;
        locals.var_deltemp_dn8 = 0.0;
        locals.var_deltemp_dn9 = 0.0;
        locals.var_deltemp_dn10 = 0.0;
        locals.var_deltemp_dn11 = 0.0;
        locals.var_deltemp_dn14 = 0.0;

        locals.var_vdsei = 0.0;
        locals.var_vdsei_dn0 = 0.0;
        locals.var_vdsei_dn2 = 0.0;

        locals.var_vgsei = 0.0;
        locals.var_vgsei_dn2 = 0.0;
        locals.var_vgsei_dn7 = 0.0;

        locals.var_vbsei = 0.0;
        locals.var_vbsei_dn2 = 0.0;
        locals.var_vbsei_dn9 = 0.0;

        locals.var_isubs = 0.0;
        locals.var_isubs_dn0 = 0.0;
        locals.var_isubs_dn2 = 0.0;
        locals.var_isubs_dn4 = 0.0;
        locals.var_isubs_dn5 = 0.0;
        locals.var_isubs_dn6 = 0.0;
        locals.var_isubs_dn7 = 0.0;
        locals.var_isubs_dn8 = 0.0;
        locals.var_isubs_dn9 = 0.0;
        locals.var_isubs_dn10 = 0.0;
        locals.var_isubs_dn11 = 0.0;
        locals.var_isubs_dn14 = 0.0;

        locals.var_isublds = 0.0;
        locals.var_isublds_dn0 = 0.0;
        locals.var_isublds_dn2 = 0.0;
        locals.var_isublds_dn4 = 0.0;
        locals.var_isublds_dn5 = 0.0;
        locals.var_isublds_dn6 = 0.0;
        locals.var_isublds_dn7 = 0.0;
        locals.var_isublds_dn8 = 0.0;
        locals.var_isublds_dn9 = 0.0;
        locals.var_isublds_dn10 = 0.0;
        locals.var_isublds_dn11 = 0.0;
        locals.var_isublds_dn14 = 0.0;

        locals.var_idsibpcs = 0.0;
        locals.var_idsibpcs_dn0 = 0.0;
        locals.var_idsibpcs_dn2 = 0.0;
        locals.var_idsibpcs_dn4 = 0.0;
        locals.var_idsibpcs_dn5 = 0.0;
        locals.var_idsibpcs_dn6 = 0.0;
        locals.var_idsibpcs_dn7 = 0.0;
        locals.var_idsibpcs_dn8 = 0.0;
        locals.var_idsibpcs_dn9 = 0.0;
        locals.var_idsibpcs_dn10 = 0.0;
        locals.var_idsibpcs_dn11 = 0.0;
        locals.var_idsibpcs_dn14 = 0.0;

        locals.var_gth = 0.0;
        locals.var_gth_dn0 = 0.0;
        locals.var_gth_dn2 = 0.0;
        locals.var_gth_dn4 = 0.0;
        locals.var_gth_dn5 = 0.0;
        locals.var_gth_dn6 = 0.0;
        locals.var_gth_dn7 = 0.0;
        locals.var_gth_dn8 = 0.0;
        locals.var_gth_dn9 = 0.0;
        locals.var_gth_dn10 = 0.0;
        locals.var_gth_dn11 = 0.0;
        locals.var_gth_dn14 = 0.0;

        locals.var_qg = 0.0;
        locals.var_qg_dn0 = 0.0;
        locals.var_qg_dn2 = 0.0;
        locals.var_qg_dn4 = 0.0;
        locals.var_qg_dn5 = 0.0;
        locals.var_qg_dn6 = 0.0;
        locals.var_qg_dn7 = 0.0;
        locals.var_qg_dn8 = 0.0;
        locals.var_qg_dn9 = 0.0;
        locals.var_qg_dn10 = 0.0;
        locals.var_qg_dn11 = 0.0;
        locals.var_qg_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_qs = 0.0;
        locals.var_qs_dn0 = 0.0;
        locals.var_qs_dn2 = 0.0;
        locals.var_qs_dn4 = 0.0;
        locals.var_qs_dn5 = 0.0;
        locals.var_qs_dn6 = 0.0;
        locals.var_qs_dn7 = 0.0;
        locals.var_qs_dn8 = 0.0;
        locals.var_qs_dn9 = 0.0;
        locals.var_qs_dn10 = 0.0;
        locals.var_qs_dn11 = 0.0;
        locals.var_qs_dn14 = 0.0;

        locals.var_veffpower = 0.0;
        locals.var_veffpower_dn0 = 0.0;
        locals.var_veffpower_dn2 = 0.0;
        locals.var_veffpower_dn4 = 0.0;
        locals.var_veffpower_dn5 = 0.0;
        locals.var_veffpower_dn6 = 0.0;
        locals.var_veffpower_dn7 = 0.0;
        locals.var_veffpower_dn8 = 0.0;
        locals.var_veffpower_dn9 = 0.0;
        locals.var_veffpower_dn10 = 0.0;
        locals.var_veffpower_dn11 = 0.0;
        locals.var_veffpower_dn14 = 0.0;

        locals.var_p = 0.0;
        locals.var_p_dn0 = 0.0;
        locals.var_p_dn2 = 0.0;
        locals.var_p_dn4 = 0.0;
        locals.var_p_dn5 = 0.0;
        locals.var_p_dn6 = 0.0;
        locals.var_p_dn7 = 0.0;
        locals.var_p_dn8 = 0.0;
        locals.var_p_dn9 = 0.0;
        locals.var_p_dn10 = 0.0;
        locals.var_p_dn11 = 0.0;
        locals.var_p_dn14 = 0.0;

        locals.var_qi_nqs = 0.0;
        locals.var_qi_nqs_dn12 = 0.0;

        locals.var_qb_nqs = 0.0;
        locals.var_qb_nqs_dn13 = 0.0;

        locals.var_iqi_nqs = 0.0;
        locals.var_iqi_nqs_dn0 = 0.0;
        locals.var_iqi_nqs_dn2 = 0.0;
        locals.var_iqi_nqs_dn4 = 0.0;
        locals.var_iqi_nqs_dn5 = 0.0;
        locals.var_iqi_nqs_dn6 = 0.0;
        locals.var_iqi_nqs_dn7 = 0.0;
        locals.var_iqi_nqs_dn8 = 0.0;
        locals.var_iqi_nqs_dn9 = 0.0;
        locals.var_iqi_nqs_dn10 = 0.0;
        locals.var_iqi_nqs_dn11 = 0.0;
        locals.var_iqi_nqs_dn12 = 0.0;
        locals.var_iqi_nqs_dn14 = 0.0;

        locals.var_iqb_nqs = 0.0;
        locals.var_iqb_nqs_dn0 = 0.0;
        locals.var_iqb_nqs_dn2 = 0.0;
        locals.var_iqb_nqs_dn4 = 0.0;
        locals.var_iqb_nqs_dn5 = 0.0;
        locals.var_iqb_nqs_dn6 = 0.0;
        locals.var_iqb_nqs_dn7 = 0.0;
        locals.var_iqb_nqs_dn8 = 0.0;
        locals.var_iqb_nqs_dn9 = 0.0;
        locals.var_iqb_nqs_dn10 = 0.0;
        locals.var_iqb_nqs_dn11 = 0.0;
        locals.var_iqb_nqs_dn13 = 0.0;
        locals.var_iqb_nqs_dn14 = 0.0;

        locals.var_qd_nqs = 0.0;
        locals.var_qd_nqs_dn0 = 0.0;
        locals.var_qd_nqs_dn2 = 0.0;
        locals.var_qd_nqs_dn4 = 0.0;
        locals.var_qd_nqs_dn5 = 0.0;
        locals.var_qd_nqs_dn6 = 0.0;
        locals.var_qd_nqs_dn7 = 0.0;
        locals.var_qd_nqs_dn8 = 0.0;
        locals.var_qd_nqs_dn9 = 0.0;
        locals.var_qd_nqs_dn10 = 0.0;
        locals.var_qd_nqs_dn11 = 0.0;
        locals.var_qd_nqs_dn12 = 0.0;
        locals.var_qd_nqs_dn14 = 0.0;

        locals.var_qs_nqs = 0.0;
        locals.var_qs_nqs_dn0 = 0.0;
        locals.var_qs_nqs_dn2 = 0.0;
        locals.var_qs_nqs_dn4 = 0.0;
        locals.var_qs_nqs_dn5 = 0.0;
        locals.var_qs_nqs_dn6 = 0.0;
        locals.var_qs_nqs_dn7 = 0.0;
        locals.var_qs_nqs_dn8 = 0.0;
        locals.var_qs_nqs_dn9 = 0.0;
        locals.var_qs_nqs_dn10 = 0.0;
        locals.var_qs_nqs_dn11 = 0.0;
        locals.var_qs_nqs_dn12 = 0.0;
        locals.var_qs_nqs_dn14 = 0.0;

        locals.var_qg_nqs = 0.0;
        locals.var_qg_nqs_dn12 = 0.0;
        locals.var_qg_nqs_dn13 = 0.0;

        locals.var_qbulk = 0.0;
        locals.var_qbulk_dn0 = 0.0;
        locals.var_qbulk_dn2 = 0.0;
        locals.var_qbulk_dn4 = 0.0;
        locals.var_qbulk_dn5 = 0.0;
        locals.var_qbulk_dn6 = 0.0;
        locals.var_qbulk_dn7 = 0.0;
        locals.var_qbulk_dn8 = 0.0;
        locals.var_qbulk_dn9 = 0.0;
        locals.var_qbulk_dn10 = 0.0;
        locals.var_qbulk_dn11 = 0.0;
        locals.var_qbulk_dn14 = 0.0;

        locals.var_cgsb = 0.0;
        locals.var_cgsb_dn0 = 0.0;
        locals.var_cgsb_dn2 = 0.0;
        locals.var_cgsb_dn4 = 0.0;
        locals.var_cgsb_dn5 = 0.0;
        locals.var_cgsb_dn6 = 0.0;
        locals.var_cgsb_dn7 = 0.0;
        locals.var_cgsb_dn8 = 0.0;
        locals.var_cgsb_dn9 = 0.0;
        locals.var_cgsb_dn10 = 0.0;
        locals.var_cgsb_dn11 = 0.0;
        locals.var_cgsb_dn14 = 0.0;

        locals.var_ninvde = 0.0;
        locals.var_ninvde_dn0 = 0.0;
        locals.var_ninvde_dn2 = 0.0;
        locals.var_ninvde_dn4 = 0.0;
        locals.var_ninvde_dn5 = 0.0;
        locals.var_ninvde_dn6 = 0.0;
        locals.var_ninvde_dn7 = 0.0;
        locals.var_ninvde_dn8 = 0.0;
        locals.var_ninvde_dn9 = 0.0;
        locals.var_ninvde_dn10 = 0.0;
        locals.var_ninvde_dn11 = 0.0;
        locals.var_ninvde_dn14 = 0.0;

        locals.var_ninvdecres = 0.0;
        locals.var_ninvdecres_dn0 = 0.0;
        locals.var_ninvdecres_dn2 = 0.0;
        locals.var_ninvdecres_dn4 = 0.0;
        locals.var_ninvdecres_dn5 = 0.0;
        locals.var_ninvdecres_dn6 = 0.0;
        locals.var_ninvdecres_dn7 = 0.0;
        locals.var_ninvdecres_dn8 = 0.0;
        locals.var_ninvdecres_dn9 = 0.0;
        locals.var_ninvdecres_dn10 = 0.0;
        locals.var_ninvdecres_dn11 = 0.0;
        locals.var_ninvdecres_dn14 = 0.0;

        locals.var_ninvdehres = 0.0;
        locals.var_ninvdehres_dn0 = 0.0;
        locals.var_ninvdehres_dn2 = 0.0;
        locals.var_ninvdehres_dn4 = 0.0;
        locals.var_ninvdehres_dn5 = 0.0;
        locals.var_ninvdehres_dn6 = 0.0;
        locals.var_ninvdehres_dn7 = 0.0;
        locals.var_ninvdehres_dn8 = 0.0;
        locals.var_ninvdehres_dn9 = 0.0;
        locals.var_ninvdehres_dn10 = 0.0;
        locals.var_ninvdehres_dn11 = 0.0;
        locals.var_ninvdehres_dn14 = 0.0;

        locals.var_rrdrmue = 0.0;
        locals.var_rrdrmue_dn0 = 0.0;
        locals.var_rrdrmue_dn2 = 0.0;
        locals.var_rrdrmue_dn4 = 0.0;
        locals.var_rrdrmue_dn5 = 0.0;
        locals.var_rrdrmue_dn6 = 0.0;
        locals.var_rrdrmue_dn7 = 0.0;
        locals.var_rrdrmue_dn8 = 0.0;
        locals.var_rrdrmue_dn9 = 0.0;
        locals.var_rrdrmue_dn10 = 0.0;
        locals.var_rrdrmue_dn11 = 0.0;
        locals.var_rrdrmue_dn14 = 0.0;

        locals.var_rrdrmues = 0.0;
        locals.var_rrdrmues_dn0 = 0.0;
        locals.var_rrdrmues_dn2 = 0.0;
        locals.var_rrdrmues_dn4 = 0.0;
        locals.var_rrdrmues_dn5 = 0.0;
        locals.var_rrdrmues_dn6 = 0.0;
        locals.var_rrdrmues_dn7 = 0.0;
        locals.var_rrdrmues_dn8 = 0.0;
        locals.var_rrdrmues_dn9 = 0.0;
        locals.var_rrdrmues_dn10 = 0.0;
        locals.var_rrdrmues_dn11 = 0.0;
        locals.var_rrdrmues_dn14 = 0.0;

        locals.var_rrdrvmax = 0.0;
        locals.var_rrdrvmax_dn0 = 0.0;
        locals.var_rrdrvmax_dn2 = 0.0;
        locals.var_rrdrvmax_dn4 = 0.0;
        locals.var_rrdrvmax_dn5 = 0.0;
        locals.var_rrdrvmax_dn6 = 0.0;
        locals.var_rrdrvmax_dn7 = 0.0;
        locals.var_rrdrvmax_dn8 = 0.0;
        locals.var_rrdrvmax_dn9 = 0.0;
        locals.var_rrdrvmax_dn10 = 0.0;
        locals.var_rrdrvmax_dn11 = 0.0;
        locals.var_rrdrvmax_dn14 = 0.0;

        locals.var_rde = 0.0;
        locals.var_rde_dn0 = 0.0;
        locals.var_rde_dn2 = 0.0;
        locals.var_rde_dn4 = 0.0;
        locals.var_rde_dn5 = 0.0;
        locals.var_rde_dn6 = 0.0;
        locals.var_rde_dn7 = 0.0;
        locals.var_rde_dn8 = 0.0;
        locals.var_rde_dn9 = 0.0;
        locals.var_rde_dn10 = 0.0;
        locals.var_rde_dn11 = 0.0;
        locals.var_rde_dn14 = 0.0;

        locals.var_rdvde = 0.0;
        locals.var_rdvde_dn0 = 0.0;
        locals.var_rdvde_dn2 = 0.0;
        locals.var_rdvde_dn4 = 0.0;
        locals.var_rdvde_dn5 = 0.0;
        locals.var_rdvde_dn6 = 0.0;
        locals.var_rdvde_dn7 = 0.0;
        locals.var_rdvde_dn8 = 0.0;
        locals.var_rdvde_dn9 = 0.0;
        locals.var_rdvde_dn10 = 0.0;
        locals.var_rdvde_dn11 = 0.0;
        locals.var_rdvde_dn14 = 0.0;

        locals.var_rse = 0.0;
        locals.var_rse_dn0 = 0.0;
        locals.var_rse_dn2 = 0.0;
        locals.var_rse_dn4 = 0.0;
        locals.var_rse_dn5 = 0.0;
        locals.var_rse_dn6 = 0.0;
        locals.var_rse_dn7 = 0.0;
        locals.var_rse_dn8 = 0.0;
        locals.var_rse_dn9 = 0.0;
        locals.var_rse_dn10 = 0.0;
        locals.var_rse_dn11 = 0.0;
        locals.var_rse_dn14 = 0.0;

        locals.var_rsvde = 0.0;
        locals.var_rsvde_dn0 = 0.0;
        locals.var_rsvde_dn2 = 0.0;
        locals.var_rsvde_dn4 = 0.0;
        locals.var_rsvde_dn5 = 0.0;
        locals.var_rsvde_dn6 = 0.0;
        locals.var_rsvde_dn7 = 0.0;
        locals.var_rsvde_dn8 = 0.0;
        locals.var_rsvde_dn9 = 0.0;
        locals.var_rsvde_dn10 = 0.0;
        locals.var_rsvde_dn11 = 0.0;
        locals.var_rsvde_dn14 = 0.0;

        locals.var_rrdrvmaxs = 0.0;
        locals.var_rrdrvmaxs_dn0 = 0.0;
        locals.var_rrdrvmaxs_dn2 = 0.0;
        locals.var_rrdrvmaxs_dn4 = 0.0;
        locals.var_rrdrvmaxs_dn5 = 0.0;
        locals.var_rrdrvmaxs_dn6 = 0.0;
        locals.var_rrdrvmaxs_dn7 = 0.0;
        locals.var_rrdrvmaxs_dn8 = 0.0;
        locals.var_rrdrvmaxs_dn9 = 0.0;
        locals.var_rrdrvmaxs_dn10 = 0.0;
        locals.var_rrdrvmaxs_dn11 = 0.0;
        locals.var_rrdrvmaxs_dn14 = 0.0;

        locals.var_tratio = 0.0;
        locals.var_tratio_dn0 = 0.0;
        locals.var_tratio_dn2 = 0.0;
        locals.var_tratio_dn4 = 0.0;
        locals.var_tratio_dn5 = 0.0;
        locals.var_tratio_dn6 = 0.0;
        locals.var_tratio_dn7 = 0.0;
        locals.var_tratio_dn8 = 0.0;
        locals.var_tratio_dn9 = 0.0;
        locals.var_tratio_dn10 = 0.0;
        locals.var_tratio_dn11 = 0.0;
        locals.var_tratio_dn14 = 0.0;

        locals.var_vmaxeff = 0.0;
        locals.var_vmaxeff_dn0 = 0.0;
        locals.var_vmaxeff_dn2 = 0.0;
        locals.var_vmaxeff_dn4 = 0.0;
        locals.var_vmaxeff_dn5 = 0.0;
        locals.var_vmaxeff_dn6 = 0.0;
        locals.var_vmaxeff_dn7 = 0.0;
        locals.var_vmaxeff_dn8 = 0.0;
        locals.var_vmaxeff_dn9 = 0.0;
        locals.var_vmaxeff_dn10 = 0.0;
        locals.var_vmaxeff_dn11 = 0.0;
        locals.var_vmaxeff_dn14 = 0.0;

        locals.var_betatnom = 0.0;

        locals.var_cnst0over = 0.0;
        locals.var_cnst0over_dn0 = 0.0;
        locals.var_cnst0over_dn2 = 0.0;
        locals.var_cnst0over_dn4 = 0.0;
        locals.var_cnst0over_dn5 = 0.0;
        locals.var_cnst0over_dn6 = 0.0;
        locals.var_cnst0over_dn7 = 0.0;
        locals.var_cnst0over_dn8 = 0.0;
        locals.var_cnst0over_dn9 = 0.0;
        locals.var_cnst0over_dn10 = 0.0;
        locals.var_cnst0over_dn11 = 0.0;
        locals.var_cnst0over_dn14 = 0.0;

        locals.var_cnst0overs = 0.0;
        locals.var_cnst0overs_dn0 = 0.0;
        locals.var_cnst0overs_dn2 = 0.0;
        locals.var_cnst0overs_dn4 = 0.0;
        locals.var_cnst0overs_dn5 = 0.0;
        locals.var_cnst0overs_dn6 = 0.0;
        locals.var_cnst0overs_dn7 = 0.0;
        locals.var_cnst0overs_dn8 = 0.0;
        locals.var_cnst0overs_dn9 = 0.0;
        locals.var_cnst0overs_dn10 = 0.0;
        locals.var_cnst0overs_dn11 = 0.0;
        locals.var_cnst0overs_dn14 = 0.0;

        locals.var_costi0_p2 = 0.0;
        locals.var_costi0_p2_dn0 = 0.0;
        locals.var_costi0_p2_dn2 = 0.0;
        locals.var_costi0_p2_dn4 = 0.0;
        locals.var_costi0_p2_dn5 = 0.0;
        locals.var_costi0_p2_dn6 = 0.0;
        locals.var_costi0_p2_dn7 = 0.0;
        locals.var_costi0_p2_dn8 = 0.0;
        locals.var_costi0_p2_dn9 = 0.0;
        locals.var_costi0_p2_dn10 = 0.0;
        locals.var_costi0_p2_dn11 = 0.0;
        locals.var_costi0_p2_dn14 = 0.0;

        locals.var_mphn0 = 0.0;
        locals.var_mphn0_dn0 = 0.0;
        locals.var_mphn0_dn2 = 0.0;
        locals.var_mphn0_dn4 = 0.0;
        locals.var_mphn0_dn5 = 0.0;
        locals.var_mphn0_dn6 = 0.0;
        locals.var_mphn0_dn7 = 0.0;
        locals.var_mphn0_dn8 = 0.0;
        locals.var_mphn0_dn9 = 0.0;
        locals.var_mphn0_dn10 = 0.0;
        locals.var_mphn0_dn11 = 0.0;
        locals.var_mphn0_dn14 = 0.0;

        locals.var_powratio = 0.0;
        locals.var_powratio_dn0 = 0.0;
        locals.var_powratio_dn2 = 0.0;
        locals.var_powratio_dn4 = 0.0;
        locals.var_powratio_dn5 = 0.0;
        locals.var_powratio_dn6 = 0.0;
        locals.var_powratio_dn7 = 0.0;
        locals.var_powratio_dn8 = 0.0;
        locals.var_powratio_dn9 = 0.0;
        locals.var_powratio_dn10 = 0.0;
        locals.var_powratio_dn11 = 0.0;
        locals.var_powratio_dn14 = 0.0;

        locals.var_ptovr = 0.0;
        locals.var_ptovr_dn0 = 0.0;
        locals.var_ptovr_dn2 = 0.0;
        locals.var_ptovr_dn4 = 0.0;
        locals.var_ptovr_dn5 = 0.0;
        locals.var_ptovr_dn6 = 0.0;
        locals.var_ptovr_dn7 = 0.0;
        locals.var_ptovr_dn8 = 0.0;
        locals.var_ptovr_dn9 = 0.0;
        locals.var_ptovr_dn10 = 0.0;
        locals.var_ptovr_dn11 = 0.0;
        locals.var_ptovr_dn14 = 0.0;

        locals.var_sqrt_eg = 0.0;
        locals.var_sqrt_eg_dn0 = 0.0;
        locals.var_sqrt_eg_dn2 = 0.0;
        locals.var_sqrt_eg_dn4 = 0.0;
        locals.var_sqrt_eg_dn5 = 0.0;
        locals.var_sqrt_eg_dn6 = 0.0;
        locals.var_sqrt_eg_dn7 = 0.0;
        locals.var_sqrt_eg_dn8 = 0.0;
        locals.var_sqrt_eg_dn9 = 0.0;
        locals.var_sqrt_eg_dn10 = 0.0;
        locals.var_sqrt_eg_dn11 = 0.0;
        locals.var_sqrt_eg_dn14 = 0.0;

        locals.var_wdpl = 0.0;
        locals.var_wdpl_dn0 = 0.0;
        locals.var_wdpl_dn2 = 0.0;
        locals.var_wdpl_dn4 = 0.0;
        locals.var_wdpl_dn5 = 0.0;
        locals.var_wdpl_dn6 = 0.0;
        locals.var_wdpl_dn7 = 0.0;
        locals.var_wdpl_dn8 = 0.0;
        locals.var_wdpl_dn9 = 0.0;
        locals.var_wdpl_dn10 = 0.0;
        locals.var_wdpl_dn11 = 0.0;
        locals.var_wdpl_dn14 = 0.0;

        locals.var_wdplp = 0.0;
        locals.var_wdplp_dn0 = 0.0;
        locals.var_wdplp_dn2 = 0.0;
        locals.var_wdplp_dn4 = 0.0;
        locals.var_wdplp_dn5 = 0.0;
        locals.var_wdplp_dn6 = 0.0;
        locals.var_wdplp_dn7 = 0.0;
        locals.var_wdplp_dn8 = 0.0;
        locals.var_wdplp_dn9 = 0.0;
        locals.var_wdplp_dn10 = 0.0;
        locals.var_wdplp_dn11 = 0.0;
        locals.var_wdplp_dn14 = 0.0;

        locals.var_uc_rdrbb = p.p436;
        locals.var_uc_rdrbb_dn0 = 0.0;
        locals.var_uc_rdrbb_dn2 = 0.0;
        locals.var_uc_rdrbb_dn4 = 0.0;
        locals.var_uc_rdrbb_dn5 = 0.0;
        locals.var_uc_rdrbb_dn6 = 0.0;
        locals.var_uc_rdrbb_dn7 = 0.0;
        locals.var_uc_rdrbb_dn8 = 0.0;
        locals.var_uc_rdrbb_dn9 = 0.0;
        locals.var_uc_rdrbb_dn10 = 0.0;
        locals.var_uc_rdrbb_dn11 = 0.0;
        locals.var_uc_rdrbb_dn14 = 0.0;

        locals.var_uc_rdrbb_s = p.p437;
        locals.var_uc_rdrbb_s_dn0 = 0.0;
        locals.var_uc_rdrbb_s_dn2 = 0.0;
        locals.var_uc_rdrbb_s_dn4 = 0.0;
        locals.var_uc_rdrbb_s_dn5 = 0.0;
        locals.var_uc_rdrbb_s_dn6 = 0.0;
        locals.var_uc_rdrbb_s_dn7 = 0.0;
        locals.var_uc_rdrbb_s_dn8 = 0.0;
        locals.var_uc_rdrbb_s_dn9 = 0.0;
        locals.var_uc_rdrbb_s_dn10 = 0.0;
        locals.var_uc_rdrbb_s_dn11 = 0.0;
        locals.var_uc_rdrbb_s_dn14 = 0.0;

        locals.var_ids_acc = 0.0;
        locals.var_ids_acc_dn0 = 0.0;
        locals.var_ids_acc_dn2 = 0.0;
        locals.var_ids_acc_dn4 = 0.0;
        locals.var_ids_acc_dn5 = 0.0;
        locals.var_ids_acc_dn6 = 0.0;
        locals.var_ids_acc_dn7 = 0.0;
        locals.var_ids_acc_dn8 = 0.0;
        locals.var_ids_acc_dn9 = 0.0;
        locals.var_ids_acc_dn10 = 0.0;
        locals.var_ids_acc_dn11 = 0.0;
        locals.var_ids_acc_dn14 = 0.0;

        locals.var_ids_res = 0.0;
        locals.var_ids_res_dn0 = 0.0;
        locals.var_ids_res_dn2 = 0.0;
        locals.var_ids_res_dn4 = 0.0;
        locals.var_ids_res_dn5 = 0.0;
        locals.var_ids_res_dn6 = 0.0;
        locals.var_ids_res_dn7 = 0.0;
        locals.var_ids_res_dn8 = 0.0;
        locals.var_ids_res_dn9 = 0.0;
        locals.var_ids_res_dn10 = 0.0;
        locals.var_ids_res_dn11 = 0.0;
        locals.var_ids_res_dn14 = 0.0;

        locals.var_ires_leak = 0.0;
        locals.var_ires_leak_dn0 = 0.0;
        locals.var_ires_leak_dn2 = 0.0;
        locals.var_ires_leak_dn4 = 0.0;
        locals.var_ires_leak_dn5 = 0.0;
        locals.var_ires_leak_dn6 = 0.0;
        locals.var_ires_leak_dn7 = 0.0;
        locals.var_ires_leak_dn8 = 0.0;
        locals.var_ires_leak_dn9 = 0.0;
        locals.var_ires_leak_dn10 = 0.0;
        locals.var_ires_leak_dn11 = 0.0;
        locals.var_ires_leak_dn14 = 0.0;

        locals.var_pb2n = 0.0;
        locals.var_pb2n_dn0 = 0.0;
        locals.var_pb2n_dn2 = 0.0;
        locals.var_pb2n_dn4 = 0.0;
        locals.var_pb2n_dn5 = 0.0;
        locals.var_pb2n_dn6 = 0.0;
        locals.var_pb2n_dn7 = 0.0;
        locals.var_pb2n_dn8 = 0.0;
        locals.var_pb2n_dn9 = 0.0;
        locals.var_pb2n_dn10 = 0.0;
        locals.var_pb2n_dn11 = 0.0;
        locals.var_pb2n_dn14 = 0.0;

        locals.var_vbipn = 0.0;
        locals.var_vbipn_dn0 = 0.0;
        locals.var_vbipn_dn2 = 0.0;
        locals.var_vbipn_dn4 = 0.0;
        locals.var_vbipn_dn5 = 0.0;
        locals.var_vbipn_dn6 = 0.0;
        locals.var_vbipn_dn7 = 0.0;
        locals.var_vbipn_dn8 = 0.0;
        locals.var_vbipn_dn9 = 0.0;
        locals.var_vbipn_dn10 = 0.0;
        locals.var_vbipn_dn11 = 0.0;
        locals.var_vbipn_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_hbdceff = p.p447;
        locals.var_hbdceff_dn0 = 0.0;
        locals.var_hbdceff_dn2 = 0.0;
        locals.var_hbdceff_dn4 = 0.0;
        locals.var_hbdceff_dn5 = 0.0;
        locals.var_hbdceff_dn6 = 0.0;
        locals.var_hbdceff_dn7 = 0.0;
        locals.var_hbdceff_dn8 = 0.0;
        locals.var_hbdceff_dn9 = 0.0;
        locals.var_hbdceff_dn10 = 0.0;
        locals.var_hbdceff_dn11 = 0.0;
        locals.var_hbdceff_dn14 = 0.0;

        locals.var_uc_subtmp = p.p193;

        locals.var_depmphn0 = 0.0;
        locals.var_depmphn0_dn0 = 0.0;
        locals.var_depmphn0_dn2 = 0.0;
        locals.var_depmphn0_dn4 = 0.0;
        locals.var_depmphn0_dn5 = 0.0;
        locals.var_depmphn0_dn6 = 0.0;
        locals.var_depmphn0_dn7 = 0.0;
        locals.var_depmphn0_dn8 = 0.0;
        locals.var_depmphn0_dn9 = 0.0;
        locals.var_depmphn0_dn10 = 0.0;
        locals.var_depmphn0_dn11 = 0.0;
        locals.var_depmphn0_dn14 = 0.0;

        locals.var_qiu_noi = 0.0;
        locals.var_qiu_noi_dn0 = 0.0;
        locals.var_qiu_noi_dn2 = 0.0;
        locals.var_qiu_noi_dn4 = 0.0;
        locals.var_qiu_noi_dn5 = 0.0;
        locals.var_qiu_noi_dn6 = 0.0;
        locals.var_qiu_noi_dn7 = 0.0;
        locals.var_qiu_noi_dn8 = 0.0;
        locals.var_qiu_noi_dn9 = 0.0;
        locals.var_qiu_noi_dn10 = 0.0;
        locals.var_qiu_noi_dn11 = 0.0;
        locals.var_qiu_noi_dn14 = 0.0;

        locals.var_lp_s0_max = 40.0;

        locals.var_js = 0.0;
        locals.var_js_dn0 = 0.0;
        locals.var_js_dn2 = 0.0;
        locals.var_js_dn4 = 0.0;
        locals.var_js_dn5 = 0.0;
        locals.var_js_dn6 = 0.0;
        locals.var_js_dn7 = 0.0;
        locals.var_js_dn8 = 0.0;
        locals.var_js_dn9 = 0.0;
        locals.var_js_dn10 = 0.0;
        locals.var_js_dn11 = 0.0;
        locals.var_js_dn14 = 0.0;

        locals.var_jssw = 0.0;
        locals.var_jssw_dn0 = 0.0;
        locals.var_jssw_dn2 = 0.0;
        locals.var_jssw_dn4 = 0.0;
        locals.var_jssw_dn5 = 0.0;
        locals.var_jssw_dn6 = 0.0;
        locals.var_jssw_dn7 = 0.0;
        locals.var_jssw_dn8 = 0.0;
        locals.var_jssw_dn9 = 0.0;
        locals.var_jssw_dn10 = 0.0;
        locals.var_jssw_dn11 = 0.0;
        locals.var_jssw_dn14 = 0.0;

        locals.var_js2 = 0.0;
        locals.var_js2_dn0 = 0.0;
        locals.var_js2_dn2 = 0.0;
        locals.var_js2_dn4 = 0.0;
        locals.var_js2_dn5 = 0.0;
        locals.var_js2_dn6 = 0.0;
        locals.var_js2_dn7 = 0.0;
        locals.var_js2_dn8 = 0.0;
        locals.var_js2_dn9 = 0.0;
        locals.var_js2_dn10 = 0.0;
        locals.var_js2_dn11 = 0.0;
        locals.var_js2_dn14 = 0.0;

        locals.var_jssw2 = 0.0;
        locals.var_jssw2_dn0 = 0.0;
        locals.var_jssw2_dn2 = 0.0;
        locals.var_jssw2_dn4 = 0.0;
        locals.var_jssw2_dn5 = 0.0;
        locals.var_jssw2_dn6 = 0.0;
        locals.var_jssw2_dn7 = 0.0;
        locals.var_jssw2_dn8 = 0.0;
        locals.var_jssw2_dn9 = 0.0;
        locals.var_jssw2_dn10 = 0.0;
        locals.var_jssw2_dn11 = 0.0;
        locals.var_jssw2_dn14 = 0.0;

        locals.var_ibs = 0.0;
        locals.var_ibs_dn0 = 0.0;
        locals.var_ibs_dn2 = 0.0;
        locals.var_ibs_dn4 = 0.0;
        locals.var_ibs_dn5 = 0.0;
        locals.var_ibs_dn6 = 0.0;
        locals.var_ibs_dn7 = 0.0;
        locals.var_ibs_dn8 = 0.0;
        locals.var_ibs_dn9 = 0.0;
        locals.var_ibs_dn10 = 0.0;
        locals.var_ibs_dn11 = 0.0;
        locals.var_ibs_dn14 = 0.0;

        locals.var_ibd = 0.0;
        locals.var_ibd_dn0 = 0.0;
        locals.var_ibd_dn2 = 0.0;
        locals.var_ibd_dn4 = 0.0;
        locals.var_ibd_dn5 = 0.0;
        locals.var_ibd_dn6 = 0.0;
        locals.var_ibd_dn7 = 0.0;
        locals.var_ibd_dn8 = 0.0;
        locals.var_ibd_dn9 = 0.0;
        locals.var_ibd_dn10 = 0.0;
        locals.var_ibd_dn11 = 0.0;
        locals.var_ibd_dn14 = 0.0;

        locals.var_ibsi = 0.0;
        locals.var_ibsi_dn0 = 0.0;
        locals.var_ibsi_dn2 = 0.0;
        locals.var_ibsi_dn4 = 0.0;
        locals.var_ibsi_dn5 = 0.0;
        locals.var_ibsi_dn6 = 0.0;
        locals.var_ibsi_dn7 = 0.0;
        locals.var_ibsi_dn8 = 0.0;
        locals.var_ibsi_dn9 = 0.0;
        locals.var_ibsi_dn10 = 0.0;
        locals.var_ibsi_dn11 = 0.0;
        locals.var_ibsi_dn14 = 0.0;

        locals.var_ibdi = 0.0;
        locals.var_ibdi_dn0 = 0.0;
        locals.var_ibdi_dn2 = 0.0;
        locals.var_ibdi_dn4 = 0.0;
        locals.var_ibdi_dn5 = 0.0;
        locals.var_ibdi_dn6 = 0.0;
        locals.var_ibdi_dn7 = 0.0;
        locals.var_ibdi_dn8 = 0.0;
        locals.var_ibdi_dn9 = 0.0;
        locals.var_ibdi_dn10 = 0.0;
        locals.var_ibdi_dn11 = 0.0;
        locals.var_ibdi_dn14 = 0.0;

        locals.var_qbs = 0.0;
        locals.var_qbs_dn0 = 0.0;
        locals.var_qbs_dn2 = 0.0;
        locals.var_qbs_dn4 = 0.0;
        locals.var_qbs_dn5 = 0.0;
        locals.var_qbs_dn6 = 0.0;
        locals.var_qbs_dn7 = 0.0;
        locals.var_qbs_dn8 = 0.0;
        locals.var_qbs_dn9 = 0.0;
        locals.var_qbs_dn10 = 0.0;
        locals.var_qbs_dn11 = 0.0;
        locals.var_qbs_dn14 = 0.0;

        locals.var_qbd = 0.0;
        locals.var_qbd_dn0 = 0.0;
        locals.var_qbd_dn2 = 0.0;
        locals.var_qbd_dn4 = 0.0;
        locals.var_qbd_dn5 = 0.0;
        locals.var_qbd_dn6 = 0.0;
        locals.var_qbd_dn7 = 0.0;
        locals.var_qbd_dn8 = 0.0;
        locals.var_qbd_dn9 = 0.0;
        locals.var_qbd_dn10 = 0.0;
        locals.var_qbd_dn11 = 0.0;
        locals.var_qbd_dn14 = 0.0;
        locals.var_qbd_dn16 = 0.0;
        locals.var_qbd_dn17 = 0.0;
        locals.var_qbd_dn18 = 0.0;

        locals.var_qbsi = 0.0;
        locals.var_qbsi_dn0 = 0.0;
        locals.var_qbsi_dn2 = 0.0;
        locals.var_qbsi_dn4 = 0.0;
        locals.var_qbsi_dn5 = 0.0;
        locals.var_qbsi_dn6 = 0.0;
        locals.var_qbsi_dn7 = 0.0;
        locals.var_qbsi_dn8 = 0.0;
        locals.var_qbsi_dn9 = 0.0;
        locals.var_qbsi_dn10 = 0.0;
        locals.var_qbsi_dn11 = 0.0;
        locals.var_qbsi_dn14 = 0.0;

        locals.var_qbdi = 0.0;
        locals.var_qbdi_dn0 = 0.0;
        locals.var_qbdi_dn2 = 0.0;
        locals.var_qbdi_dn4 = 0.0;
        locals.var_qbdi_dn5 = 0.0;
        locals.var_qbdi_dn6 = 0.0;
        locals.var_qbdi_dn7 = 0.0;
        locals.var_qbdi_dn8 = 0.0;
        locals.var_qbdi_dn9 = 0.0;
        locals.var_qbdi_dn10 = 0.0;
        locals.var_qbdi_dn11 = 0.0;
        locals.var_qbdi_dn14 = 0.0;

        locals.var_czbd = 0.0;
        locals.var_czbd_dn0 = 0.0;
        locals.var_czbd_dn2 = 0.0;
        locals.var_czbd_dn4 = 0.0;
        locals.var_czbd_dn5 = 0.0;
        locals.var_czbd_dn6 = 0.0;
        locals.var_czbd_dn7 = 0.0;
        locals.var_czbd_dn8 = 0.0;
        locals.var_czbd_dn9 = 0.0;
        locals.var_czbd_dn10 = 0.0;
        locals.var_czbd_dn11 = 0.0;
        locals.var_czbd_dn14 = 0.0;

        locals.var_czbdsw = 0.0;
        locals.var_czbdsw_dn0 = 0.0;
        locals.var_czbdsw_dn2 = 0.0;
        locals.var_czbdsw_dn4 = 0.0;
        locals.var_czbdsw_dn5 = 0.0;
        locals.var_czbdsw_dn6 = 0.0;
        locals.var_czbdsw_dn7 = 0.0;
        locals.var_czbdsw_dn8 = 0.0;
        locals.var_czbdsw_dn9 = 0.0;
        locals.var_czbdsw_dn10 = 0.0;
        locals.var_czbdsw_dn11 = 0.0;
        locals.var_czbdsw_dn14 = 0.0;

        locals.var_czbdswg = 0.0;
        locals.var_czbdswg_dn0 = 0.0;
        locals.var_czbdswg_dn2 = 0.0;
        locals.var_czbdswg_dn4 = 0.0;
        locals.var_czbdswg_dn5 = 0.0;
        locals.var_czbdswg_dn6 = 0.0;
        locals.var_czbdswg_dn7 = 0.0;
        locals.var_czbdswg_dn8 = 0.0;
        locals.var_czbdswg_dn9 = 0.0;
        locals.var_czbdswg_dn10 = 0.0;
        locals.var_czbdswg_dn11 = 0.0;
        locals.var_czbdswg_dn14 = 0.0;

        locals.var_czbs = 0.0;
        locals.var_czbs_dn0 = 0.0;
        locals.var_czbs_dn2 = 0.0;
        locals.var_czbs_dn4 = 0.0;
        locals.var_czbs_dn5 = 0.0;
        locals.var_czbs_dn6 = 0.0;
        locals.var_czbs_dn7 = 0.0;
        locals.var_czbs_dn8 = 0.0;
        locals.var_czbs_dn9 = 0.0;
        locals.var_czbs_dn10 = 0.0;
        locals.var_czbs_dn11 = 0.0;
        locals.var_czbs_dn14 = 0.0;

        locals.var_czbssw = 0.0;
        locals.var_czbssw_dn0 = 0.0;
        locals.var_czbssw_dn2 = 0.0;
        locals.var_czbssw_dn4 = 0.0;
        locals.var_czbssw_dn5 = 0.0;
        locals.var_czbssw_dn6 = 0.0;
        locals.var_czbssw_dn7 = 0.0;
        locals.var_czbssw_dn8 = 0.0;
        locals.var_czbssw_dn9 = 0.0;
        locals.var_czbssw_dn10 = 0.0;
        locals.var_czbssw_dn11 = 0.0;
        locals.var_czbssw_dn14 = 0.0;

        locals.var_czbsswg = 0.0;
        locals.var_czbsswg_dn0 = 0.0;
        locals.var_czbsswg_dn2 = 0.0;
        locals.var_czbsswg_dn4 = 0.0;
        locals.var_czbsswg_dn5 = 0.0;
        locals.var_czbsswg_dn6 = 0.0;
        locals.var_czbsswg_dn7 = 0.0;
        locals.var_czbsswg_dn8 = 0.0;
        locals.var_czbsswg_dn9 = 0.0;
        locals.var_czbsswg_dn10 = 0.0;
        locals.var_czbsswg_dn11 = 0.0;
        locals.var_czbsswg_dn14 = 0.0;

        locals.var_pzbd = 0.0;
        locals.var_pzbd_dn0 = 0.0;
        locals.var_pzbd_dn2 = 0.0;
        locals.var_pzbd_dn4 = 0.0;
        locals.var_pzbd_dn5 = 0.0;
        locals.var_pzbd_dn6 = 0.0;
        locals.var_pzbd_dn7 = 0.0;
        locals.var_pzbd_dn8 = 0.0;
        locals.var_pzbd_dn9 = 0.0;
        locals.var_pzbd_dn10 = 0.0;
        locals.var_pzbd_dn11 = 0.0;
        locals.var_pzbd_dn14 = 0.0;

        locals.var_pzbdsw = 0.0;
        locals.var_pzbdsw_dn0 = 0.0;
        locals.var_pzbdsw_dn2 = 0.0;
        locals.var_pzbdsw_dn4 = 0.0;
        locals.var_pzbdsw_dn5 = 0.0;
        locals.var_pzbdsw_dn6 = 0.0;
        locals.var_pzbdsw_dn7 = 0.0;
        locals.var_pzbdsw_dn8 = 0.0;
        locals.var_pzbdsw_dn9 = 0.0;
        locals.var_pzbdsw_dn10 = 0.0;
        locals.var_pzbdsw_dn11 = 0.0;
        locals.var_pzbdsw_dn14 = 0.0;

        locals.var_pzbdswg = 0.0;
        locals.var_pzbdswg_dn0 = 0.0;
        locals.var_pzbdswg_dn2 = 0.0;
        locals.var_pzbdswg_dn4 = 0.0;
        locals.var_pzbdswg_dn5 = 0.0;
        locals.var_pzbdswg_dn6 = 0.0;
        locals.var_pzbdswg_dn7 = 0.0;
        locals.var_pzbdswg_dn8 = 0.0;
        locals.var_pzbdswg_dn9 = 0.0;
        locals.var_pzbdswg_dn10 = 0.0;
        locals.var_pzbdswg_dn11 = 0.0;
        locals.var_pzbdswg_dn14 = 0.0;

        locals.var_pzbs = 0.0;
        locals.var_pzbs_dn0 = 0.0;
        locals.var_pzbs_dn2 = 0.0;
        locals.var_pzbs_dn4 = 0.0;
        locals.var_pzbs_dn5 = 0.0;
        locals.var_pzbs_dn6 = 0.0;
        locals.var_pzbs_dn7 = 0.0;
        locals.var_pzbs_dn8 = 0.0;
        locals.var_pzbs_dn9 = 0.0;
        locals.var_pzbs_dn10 = 0.0;
        locals.var_pzbs_dn11 = 0.0;
        locals.var_pzbs_dn14 = 0.0;

        locals.var_pzbssw = 0.0;
        locals.var_pzbssw_dn0 = 0.0;
        locals.var_pzbssw_dn2 = 0.0;
        locals.var_pzbssw_dn4 = 0.0;
        locals.var_pzbssw_dn5 = 0.0;
        locals.var_pzbssw_dn6 = 0.0;
        locals.var_pzbssw_dn7 = 0.0;
        locals.var_pzbssw_dn8 = 0.0;
        locals.var_pzbssw_dn9 = 0.0;
        locals.var_pzbssw_dn10 = 0.0;
        locals.var_pzbssw_dn11 = 0.0;
        locals.var_pzbssw_dn14 = 0.0;

        locals.var_pzbsswg = 0.0;
        locals.var_pzbsswg_dn0 = 0.0;
        locals.var_pzbsswg_dn2 = 0.0;
        locals.var_pzbsswg_dn4 = 0.0;
        locals.var_pzbsswg_dn5 = 0.0;
        locals.var_pzbsswg_dn6 = 0.0;
        locals.var_pzbsswg_dn7 = 0.0;
        locals.var_pzbsswg_dn8 = 0.0;
        locals.var_pzbsswg_dn9 = 0.0;
        locals.var_pzbsswg_dn10 = 0.0;
        locals.var_pzbsswg_dn11 = 0.0;
        locals.var_pzbsswg_dn14 = 0.0;

        locals.var_sarg = 0.0;
        locals.var_sarg_dn0 = 0.0;
        locals.var_sarg_dn2 = 0.0;
        locals.var_sarg_dn4 = 0.0;
        locals.var_sarg_dn5 = 0.0;
        locals.var_sarg_dn6 = 0.0;
        locals.var_sarg_dn7 = 0.0;
        locals.var_sarg_dn8 = 0.0;
        locals.var_sarg_dn9 = 0.0;
        locals.var_sarg_dn10 = 0.0;
        locals.var_sarg_dn11 = 0.0;
        locals.var_sarg_dn14 = 0.0;

        locals.var_vsbs = 0.0;
        locals.var_vsbs_dn2 = 0.0;
        locals.var_vsbs_dn11 = 0.0;

        locals.var_vdbd = 0.0;
        locals.var_vdbd_dn0 = 0.0;
        locals.var_vdbd_dn10 = 0.0;

        locals.var_vbs_jct = 0.0;
        locals.var_vbs_jct_dn2 = 0.0;
        locals.var_vbs_jct_dn11 = 0.0;

        locals.var_vbd_jct = 0.0;
        locals.var_vbd_jct_dn0 = 0.0;
        locals.var_vbd_jct_dn10 = 0.0;

        locals.var_vbpsp = 0.0;
        locals.var_vbpsp_dn8 = 0.0;
        locals.var_vbpsp_dn9 = 0.0;

        locals.var_vbpdp = 0.0;
        locals.var_vbpdp_dn6 = 0.0;
        locals.var_vbpdp_dn9 = 0.0;

        locals.var_vbsi_jct = 0.0;
        locals.var_vbsi_jct_dn8 = 0.0;
        locals.var_vbsi_jct_dn9 = 0.0;

        locals.var_vbdi_jct = 0.0;
        locals.var_vbdi_jct_dn6 = 0.0;
        locals.var_vbdi_jct_dn9 = 0.0;

        locals.var_exptempd = 0.0;
        locals.var_exptempd_dn0 = 0.0;
        locals.var_exptempd_dn2 = 0.0;
        locals.var_exptempd_dn4 = 0.0;
        locals.var_exptempd_dn5 = 0.0;
        locals.var_exptempd_dn6 = 0.0;
        locals.var_exptempd_dn7 = 0.0;
        locals.var_exptempd_dn8 = 0.0;
        locals.var_exptempd_dn9 = 0.0;
        locals.var_exptempd_dn10 = 0.0;
        locals.var_exptempd_dn11 = 0.0;
        locals.var_exptempd_dn14 = 0.0;

        locals.var_exptemps = 0.0;
        locals.var_exptemps_dn0 = 0.0;
        locals.var_exptemps_dn2 = 0.0;
        locals.var_exptemps_dn4 = 0.0;
        locals.var_exptemps_dn5 = 0.0;
        locals.var_exptemps_dn6 = 0.0;
        locals.var_exptemps_dn7 = 0.0;
        locals.var_exptemps_dn8 = 0.0;
        locals.var_exptemps_dn9 = 0.0;
        locals.var_exptemps_dn10 = 0.0;
        locals.var_exptemps_dn11 = 0.0;
        locals.var_exptemps_dn14 = 0.0;

        locals.var_isbd = 0.0;
        locals.var_isbd_dn0 = 0.0;
        locals.var_isbd_dn2 = 0.0;
        locals.var_isbd_dn4 = 0.0;
        locals.var_isbd_dn5 = 0.0;
        locals.var_isbd_dn6 = 0.0;
        locals.var_isbd_dn7 = 0.0;
        locals.var_isbd_dn8 = 0.0;
        locals.var_isbd_dn9 = 0.0;
        locals.var_isbd_dn10 = 0.0;
        locals.var_isbd_dn11 = 0.0;
        locals.var_isbd_dn14 = 0.0;

        locals.var_isbs = 0.0;
        locals.var_isbs_dn0 = 0.0;
        locals.var_isbs_dn2 = 0.0;
        locals.var_isbs_dn4 = 0.0;
        locals.var_isbs_dn5 = 0.0;
        locals.var_isbs_dn6 = 0.0;
        locals.var_isbs_dn7 = 0.0;
        locals.var_isbs_dn8 = 0.0;
        locals.var_isbs_dn9 = 0.0;
        locals.var_isbs_dn10 = 0.0;
        locals.var_isbs_dn11 = 0.0;
        locals.var_isbs_dn14 = 0.0;

        locals.var_jd_expcd = 0.0;
        locals.var_jd_expcd_dn0 = 0.0;
        locals.var_jd_expcd_dn2 = 0.0;
        locals.var_jd_expcd_dn4 = 0.0;
        locals.var_jd_expcd_dn5 = 0.0;
        locals.var_jd_expcd_dn6 = 0.0;
        locals.var_jd_expcd_dn7 = 0.0;
        locals.var_jd_expcd_dn8 = 0.0;
        locals.var_jd_expcd_dn9 = 0.0;
        locals.var_jd_expcd_dn10 = 0.0;
        locals.var_jd_expcd_dn11 = 0.0;
        locals.var_jd_expcd_dn14 = 0.0;

        locals.var_jd_expcs = 0.0;
        locals.var_jd_expcs_dn0 = 0.0;
        locals.var_jd_expcs_dn2 = 0.0;
        locals.var_jd_expcs_dn4 = 0.0;
        locals.var_jd_expcs_dn5 = 0.0;
        locals.var_jd_expcs_dn6 = 0.0;
        locals.var_jd_expcs_dn7 = 0.0;
        locals.var_jd_expcs_dn8 = 0.0;
        locals.var_jd_expcs_dn9 = 0.0;
        locals.var_jd_expcs_dn10 = 0.0;
        locals.var_jd_expcs_dn11 = 0.0;
        locals.var_jd_expcs_dn14 = 0.0;

        locals.var_vbdt = 0.0;
        locals.var_vbdt_dn0 = 0.0;
        locals.var_vbdt_dn2 = 0.0;
        locals.var_vbdt_dn4 = 0.0;
        locals.var_vbdt_dn5 = 0.0;
        locals.var_vbdt_dn6 = 0.0;
        locals.var_vbdt_dn7 = 0.0;
        locals.var_vbdt_dn8 = 0.0;
        locals.var_vbdt_dn9 = 0.0;
        locals.var_vbdt_dn10 = 0.0;
        locals.var_vbdt_dn11 = 0.0;
        locals.var_vbdt_dn14 = 0.0;

        locals.var_vbst = 0.0;
        locals.var_vbst_dn0 = 0.0;
        locals.var_vbst_dn2 = 0.0;
        locals.var_vbst_dn4 = 0.0;
        locals.var_vbst_dn5 = 0.0;
        locals.var_vbst_dn6 = 0.0;
        locals.var_vbst_dn7 = 0.0;
        locals.var_vbst_dn8 = 0.0;
        locals.var_vbst_dn9 = 0.0;
        locals.var_vbst_dn10 = 0.0;
        locals.var_vbst_dn11 = 0.0;
        locals.var_vbst_dn14 = 0.0;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_jd_nvtm_invd = 0.0;
        locals.var_jd_nvtm_invd_dn0 = 0.0;
        locals.var_jd_nvtm_invd_dn2 = 0.0;
        locals.var_jd_nvtm_invd_dn4 = 0.0;
        locals.var_jd_nvtm_invd_dn5 = 0.0;
        locals.var_jd_nvtm_invd_dn6 = 0.0;
        locals.var_jd_nvtm_invd_dn7 = 0.0;
        locals.var_jd_nvtm_invd_dn8 = 0.0;
        locals.var_jd_nvtm_invd_dn9 = 0.0;
        locals.var_jd_nvtm_invd_dn10 = 0.0;
        locals.var_jd_nvtm_invd_dn11 = 0.0;
        locals.var_jd_nvtm_invd_dn14 = 0.0;

        locals.var_jd_nvtm_invs = 0.0;
        locals.var_jd_nvtm_invs_dn0 = 0.0;
        locals.var_jd_nvtm_invs_dn2 = 0.0;
        locals.var_jd_nvtm_invs_dn4 = 0.0;
        locals.var_jd_nvtm_invs_dn5 = 0.0;
        locals.var_jd_nvtm_invs_dn6 = 0.0;
        locals.var_jd_nvtm_invs_dn7 = 0.0;
        locals.var_jd_nvtm_invs_dn8 = 0.0;
        locals.var_jd_nvtm_invs_dn9 = 0.0;
        locals.var_jd_nvtm_invs_dn10 = 0.0;
        locals.var_jd_nvtm_invs_dn11 = 0.0;
        locals.var_jd_nvtm_invs_dn14 = 0.0;

        locals.var_end_of_part_1 = 0.0;

        locals.var_flg_brk1 = 0.0;

        locals.var_start_of_loopl = 0.0;

        locals.var_flg_brk2 = 0.0;

        locals.var_start_of_mobility = 0.0;

        locals.var_qbd_qs = 0.0;
        locals.var_qbd_qs_dn0 = 0.0;
        locals.var_qbd_qs_dn2 = 0.0;
        locals.var_qbd_qs_dn4 = 0.0;
        locals.var_qbd_qs_dn5 = 0.0;
        locals.var_qbd_qs_dn6 = 0.0;
        locals.var_qbd_qs_dn7 = 0.0;
        locals.var_qbd_qs_dn8 = 0.0;
        locals.var_qbd_qs_dn9 = 0.0;
        locals.var_qbd_qs_dn10 = 0.0;
        locals.var_qbd_qs_dn11 = 0.0;
        locals.var_qbd_qs_dn14 = 0.0;

        locals.var_ibd_nqs = 0.0;
        locals.var_ibd_nqs_dn0 = 0.0;
        locals.var_ibd_nqs_dn2 = 0.0;
        locals.var_ibd_nqs_dn4 = 0.0;
        locals.var_ibd_nqs_dn5 = 0.0;
        locals.var_ibd_nqs_dn6 = 0.0;
        locals.var_ibd_nqs_dn7 = 0.0;
        locals.var_ibd_nqs_dn8 = 0.0;
        locals.var_ibd_nqs_dn9 = 0.0;
        locals.var_ibd_nqs_dn10 = 0.0;
        locals.var_ibd_nqs_dn11 = 0.0;
        locals.var_ibd_nqs_dn14 = 0.0;

        locals.var_isbd_btm = 0.0;
        locals.var_isbd_btm_dn0 = 0.0;
        locals.var_isbd_btm_dn2 = 0.0;
        locals.var_isbd_btm_dn4 = 0.0;
        locals.var_isbd_btm_dn5 = 0.0;
        locals.var_isbd_btm_dn6 = 0.0;
        locals.var_isbd_btm_dn7 = 0.0;
        locals.var_isbd_btm_dn8 = 0.0;
        locals.var_isbd_btm_dn9 = 0.0;
        locals.var_isbd_btm_dn10 = 0.0;
        locals.var_isbd_btm_dn11 = 0.0;
        locals.var_isbd_btm_dn14 = 0.0;

        locals.var_isbd2_btm = 0.0;
        locals.var_isbd2_btm_dn0 = 0.0;
        locals.var_isbd2_btm_dn2 = 0.0;
        locals.var_isbd2_btm_dn4 = 0.0;
        locals.var_isbd2_btm_dn5 = 0.0;
        locals.var_isbd2_btm_dn6 = 0.0;
        locals.var_isbd2_btm_dn7 = 0.0;
        locals.var_isbd2_btm_dn8 = 0.0;
        locals.var_isbd2_btm_dn9 = 0.0;
        locals.var_isbd2_btm_dn10 = 0.0;
        locals.var_isbd2_btm_dn11 = 0.0;
        locals.var_isbd2_btm_dn14 = 0.0;

        locals.var_isbd_sws = 0.0;
        locals.var_isbd_sws_dn0 = 0.0;
        locals.var_isbd_sws_dn2 = 0.0;
        locals.var_isbd_sws_dn4 = 0.0;
        locals.var_isbd_sws_dn5 = 0.0;
        locals.var_isbd_sws_dn6 = 0.0;
        locals.var_isbd_sws_dn7 = 0.0;
        locals.var_isbd_sws_dn8 = 0.0;
        locals.var_isbd_sws_dn9 = 0.0;
        locals.var_isbd_sws_dn10 = 0.0;
        locals.var_isbd_sws_dn11 = 0.0;
        locals.var_isbd_sws_dn14 = 0.0;

        locals.var_isbd2_sws = 0.0;
        locals.var_isbd2_sws_dn0 = 0.0;
        locals.var_isbd2_sws_dn2 = 0.0;
        locals.var_isbd2_sws_dn4 = 0.0;
        locals.var_isbd2_sws_dn5 = 0.0;
        locals.var_isbd2_sws_dn6 = 0.0;
        locals.var_isbd2_sws_dn7 = 0.0;
        locals.var_isbd2_sws_dn8 = 0.0;
        locals.var_isbd2_sws_dn9 = 0.0;
        locals.var_isbd2_sws_dn10 = 0.0;
        locals.var_isbd2_sws_dn11 = 0.0;
        locals.var_isbd2_sws_dn14 = 0.0;

        locals.var_isbd_swg = 0.0;
        locals.var_isbd_swg_dn0 = 0.0;
        locals.var_isbd_swg_dn2 = 0.0;
        locals.var_isbd_swg_dn4 = 0.0;
        locals.var_isbd_swg_dn5 = 0.0;
        locals.var_isbd_swg_dn6 = 0.0;
        locals.var_isbd_swg_dn7 = 0.0;
        locals.var_isbd_swg_dn8 = 0.0;
        locals.var_isbd_swg_dn9 = 0.0;
        locals.var_isbd_swg_dn10 = 0.0;
        locals.var_isbd_swg_dn11 = 0.0;
        locals.var_isbd_swg_dn14 = 0.0;

        locals.var_isbd2_swg = 0.0;
        locals.var_isbd2_swg_dn0 = 0.0;
        locals.var_isbd2_swg_dn2 = 0.0;
        locals.var_isbd2_swg_dn4 = 0.0;
        locals.var_isbd2_swg_dn5 = 0.0;
        locals.var_isbd2_swg_dn6 = 0.0;
        locals.var_isbd2_swg_dn7 = 0.0;
        locals.var_isbd2_swg_dn8 = 0.0;
        locals.var_isbd2_swg_dn9 = 0.0;
        locals.var_isbd2_swg_dn10 = 0.0;
        locals.var_isbd2_swg_dn11 = 0.0;
        locals.var_isbd2_swg_dn14 = 0.0;

        locals.var_isbs_btm = 0.0;
        locals.var_isbs_btm_dn0 = 0.0;
        locals.var_isbs_btm_dn2 = 0.0;
        locals.var_isbs_btm_dn4 = 0.0;
        locals.var_isbs_btm_dn5 = 0.0;
        locals.var_isbs_btm_dn6 = 0.0;
        locals.var_isbs_btm_dn7 = 0.0;
        locals.var_isbs_btm_dn8 = 0.0;
        locals.var_isbs_btm_dn9 = 0.0;
        locals.var_isbs_btm_dn10 = 0.0;
        locals.var_isbs_btm_dn11 = 0.0;
        locals.var_isbs_btm_dn14 = 0.0;

        locals.var_isbs2_btm = 0.0;
        locals.var_isbs2_btm_dn0 = 0.0;
        locals.var_isbs2_btm_dn2 = 0.0;
        locals.var_isbs2_btm_dn4 = 0.0;
        locals.var_isbs2_btm_dn5 = 0.0;
        locals.var_isbs2_btm_dn6 = 0.0;
        locals.var_isbs2_btm_dn7 = 0.0;
        locals.var_isbs2_btm_dn8 = 0.0;
        locals.var_isbs2_btm_dn9 = 0.0;
        locals.var_isbs2_btm_dn10 = 0.0;
        locals.var_isbs2_btm_dn11 = 0.0;
        locals.var_isbs2_btm_dn14 = 0.0;

        locals.var_isbs_sws = 0.0;
        locals.var_isbs_sws_dn0 = 0.0;
        locals.var_isbs_sws_dn2 = 0.0;
        locals.var_isbs_sws_dn4 = 0.0;
        locals.var_isbs_sws_dn5 = 0.0;
        locals.var_isbs_sws_dn6 = 0.0;
        locals.var_isbs_sws_dn7 = 0.0;
        locals.var_isbs_sws_dn8 = 0.0;
        locals.var_isbs_sws_dn9 = 0.0;
        locals.var_isbs_sws_dn10 = 0.0;
        locals.var_isbs_sws_dn11 = 0.0;
        locals.var_isbs_sws_dn14 = 0.0;

        locals.var_isbs2_sws = 0.0;
        locals.var_isbs2_sws_dn0 = 0.0;
        locals.var_isbs2_sws_dn2 = 0.0;
        locals.var_isbs2_sws_dn4 = 0.0;
        locals.var_isbs2_sws_dn5 = 0.0;
        locals.var_isbs2_sws_dn6 = 0.0;
        locals.var_isbs2_sws_dn7 = 0.0;
        locals.var_isbs2_sws_dn8 = 0.0;
        locals.var_isbs2_sws_dn9 = 0.0;
        locals.var_isbs2_sws_dn10 = 0.0;
        locals.var_isbs2_sws_dn11 = 0.0;
        locals.var_isbs2_sws_dn14 = 0.0;

        locals.var_isbs_swg = 0.0;
        locals.var_isbs_swg_dn0 = 0.0;
        locals.var_isbs_swg_dn2 = 0.0;
        locals.var_isbs_swg_dn4 = 0.0;
        locals.var_isbs_swg_dn5 = 0.0;
        locals.var_isbs_swg_dn6 = 0.0;
        locals.var_isbs_swg_dn7 = 0.0;
        locals.var_isbs_swg_dn8 = 0.0;
        locals.var_isbs_swg_dn9 = 0.0;
        locals.var_isbs_swg_dn10 = 0.0;
        locals.var_isbs_swg_dn11 = 0.0;
        locals.var_isbs_swg_dn14 = 0.0;

        locals.var_isbs2_swg = 0.0;
        locals.var_isbs2_swg_dn0 = 0.0;
        locals.var_isbs2_swg_dn2 = 0.0;
        locals.var_isbs2_swg_dn4 = 0.0;
        locals.var_isbs2_swg_dn5 = 0.0;
        locals.var_isbs2_swg_dn6 = 0.0;
        locals.var_isbs2_swg_dn7 = 0.0;
        locals.var_isbs2_swg_dn8 = 0.0;
        locals.var_isbs2_swg_dn9 = 0.0;
        locals.var_isbs2_swg_dn10 = 0.0;
        locals.var_isbs2_swg_dn11 = 0.0;
        locals.var_isbs2_swg_dn14 = 0.0;

        locals.var_qovd_add = 0.0;
        locals.var_qovd_add_dn0 = 0.0;
        locals.var_qovd_add_dn2 = 0.0;
        locals.var_qovd_add_dn4 = 0.0;
        locals.var_qovd_add_dn5 = 0.0;
        locals.var_qovd_add_dn6 = 0.0;
        locals.var_qovd_add_dn7 = 0.0;
        locals.var_qovd_add_dn8 = 0.0;
        locals.var_qovd_add_dn9 = 0.0;
        locals.var_qovd_add_dn10 = 0.0;
        locals.var_qovd_add_dn11 = 0.0;
        locals.var_qovd_add_dn14 = 0.0;

        locals.var_qovs_add = 0.0;
        locals.var_qovs_add_dn0 = 0.0;
        locals.var_qovs_add_dn2 = 0.0;
        locals.var_qovs_add_dn4 = 0.0;
        locals.var_qovs_add_dn5 = 0.0;
        locals.var_qovs_add_dn6 = 0.0;
        locals.var_qovs_add_dn7 = 0.0;
        locals.var_qovs_add_dn8 = 0.0;
        locals.var_qovs_add_dn9 = 0.0;
        locals.var_qovs_add_dn10 = 0.0;
        locals.var_qovs_add_dn11 = 0.0;
        locals.var_qovs_add_dn14 = 0.0;

        locals.var_qbdld_add = 0.0;
        locals.var_qbdld_add_dn0 = 0.0;
        locals.var_qbdld_add_dn2 = 0.0;
        locals.var_qbdld_add_dn4 = 0.0;
        locals.var_qbdld_add_dn5 = 0.0;
        locals.var_qbdld_add_dn6 = 0.0;
        locals.var_qbdld_add_dn7 = 0.0;
        locals.var_qbdld_add_dn8 = 0.0;
        locals.var_qbdld_add_dn9 = 0.0;
        locals.var_qbdld_add_dn10 = 0.0;
        locals.var_qbdld_add_dn11 = 0.0;
        locals.var_qbdld_add_dn14 = 0.0;

        locals.var_qbsld_add = 0.0;
        locals.var_qbsld_add_dn0 = 0.0;
        locals.var_qbsld_add_dn2 = 0.0;
        locals.var_qbsld_add_dn4 = 0.0;
        locals.var_qbsld_add_dn5 = 0.0;
        locals.var_qbsld_add_dn6 = 0.0;
        locals.var_qbsld_add_dn7 = 0.0;
        locals.var_qbsld_add_dn8 = 0.0;
        locals.var_qbsld_add_dn9 = 0.0;
        locals.var_qbsld_add_dn10 = 0.0;
        locals.var_qbsld_add_dn11 = 0.0;
        locals.var_qbsld_add_dn14 = 0.0;

        locals.var_wjuncld = 0.0;
        locals.var_wjuncld_dn0 = 0.0;
        locals.var_wjuncld_dn2 = 0.0;
        locals.var_wjuncld_dn4 = 0.0;
        locals.var_wjuncld_dn5 = 0.0;
        locals.var_wjuncld_dn6 = 0.0;
        locals.var_wjuncld_dn7 = 0.0;
        locals.var_wjuncld_dn8 = 0.0;
        locals.var_wjuncld_dn9 = 0.0;
        locals.var_wjuncld_dn10 = 0.0;
        locals.var_wjuncld_dn11 = 0.0;
        locals.var_wjuncld_dn14 = 0.0;

        locals.var_idspt0 = 0.0;
        locals.var_idspt0_dn0 = 0.0;
        locals.var_idspt0_dn2 = 0.0;
        locals.var_idspt0_dn4 = 0.0;
        locals.var_idspt0_dn5 = 0.0;
        locals.var_idspt0_dn6 = 0.0;
        locals.var_idspt0_dn7 = 0.0;
        locals.var_idspt0_dn8 = 0.0;
        locals.var_idspt0_dn9 = 0.0;
        locals.var_idspt0_dn10 = 0.0;
        locals.var_idspt0_dn11 = 0.0;
        locals.var_idspt0_dn14 = 0.0;

        locals.var_idspt1 = 0.0;
        locals.var_idspt1_dn0 = 0.0;
        locals.var_idspt1_dn2 = 0.0;
        locals.var_idspt1_dn4 = 0.0;
        locals.var_idspt1_dn5 = 0.0;
        locals.var_idspt1_dn6 = 0.0;
        locals.var_idspt1_dn7 = 0.0;
        locals.var_idspt1_dn8 = 0.0;
        locals.var_idspt1_dn9 = 0.0;
        locals.var_idspt1_dn10 = 0.0;
        locals.var_idspt1_dn11 = 0.0;
        locals.var_idspt1_dn14 = 0.0;

        locals.var_cox0_func = 0.0;

        locals.var_iwnqs0_a = 0.0;
        locals.var_iwnqs0_a_dn0 = 0.0;
        locals.var_iwnqs0_a_dn2 = 0.0;
        locals.var_iwnqs0_a_dn4 = 0.0;
        locals.var_iwnqs0_a_dn5 = 0.0;
        locals.var_iwnqs0_a_dn6 = 0.0;
        locals.var_iwnqs0_a_dn7 = 0.0;
        locals.var_iwnqs0_a_dn8 = 0.0;
        locals.var_iwnqs0_a_dn9 = 0.0;
        locals.var_iwnqs0_a_dn10 = 0.0;
        locals.var_iwnqs0_a_dn11 = 0.0;
        locals.var_iwnqs0_a_dn14 = 0.0;
        locals.var_iwnqs0_a_dn18 = 0.0;

        locals.var_inqs0_a = 0.0;
        locals.var_inqs0_a_dn0 = 0.0;
        locals.var_inqs0_a_dn2 = 0.0;
        locals.var_inqs0_a_dn4 = 0.0;
        locals.var_inqs0_a_dn5 = 0.0;
        locals.var_inqs0_a_dn6 = 0.0;
        locals.var_inqs0_a_dn7 = 0.0;
        locals.var_inqs0_a_dn8 = 0.0;
        locals.var_inqs0_a_dn9 = 0.0;
        locals.var_inqs0_a_dn10 = 0.0;
        locals.var_inqs0_a_dn11 = 0.0;
        locals.var_inqs0_a_dn14 = 0.0;
        locals.var_inqs0_a_dn16 = 0.0;

        locals.var_inqs0_k = 0.0;
        locals.var_inqs0_k_dn0 = 0.0;
        locals.var_inqs0_k_dn2 = 0.0;
        locals.var_inqs0_k_dn4 = 0.0;
        locals.var_inqs0_k_dn5 = 0.0;
        locals.var_inqs0_k_dn6 = 0.0;
        locals.var_inqs0_k_dn7 = 0.0;
        locals.var_inqs0_k_dn8 = 0.0;
        locals.var_inqs0_k_dn9 = 0.0;
        locals.var_inqs0_k_dn10 = 0.0;
        locals.var_inqs0_k_dn11 = 0.0;
        locals.var_inqs0_k_dn14 = 0.0;
        locals.var_inqs0_k_dn17 = 0.0;

        locals.var_isubibpc = 0.0;
        locals.var_isubibpc_dn0 = 0.0;
        locals.var_isubibpc_dn2 = 0.0;
        locals.var_isubibpc_dn4 = 0.0;
        locals.var_isubibpc_dn5 = 0.0;
        locals.var_isubibpc_dn6 = 0.0;
        locals.var_isubibpc_dn7 = 0.0;
        locals.var_isubibpc_dn8 = 0.0;
        locals.var_isubibpc_dn9 = 0.0;
        locals.var_isubibpc_dn10 = 0.0;
        locals.var_isubibpc_dn11 = 0.0;
        locals.var_isubibpc_dn14 = 0.0;

        locals.var_lover_func = 0.0;
        locals.var_lover_func_dn0 = 0.0;
        locals.var_lover_func_dn2 = 0.0;
        locals.var_lover_func_dn4 = 0.0;
        locals.var_lover_func_dn5 = 0.0;
        locals.var_lover_func_dn6 = 0.0;
        locals.var_lover_func_dn7 = 0.0;
        locals.var_lover_func_dn8 = 0.0;
        locals.var_lover_func_dn9 = 0.0;
        locals.var_lover_func_dn10 = 0.0;
        locals.var_lover_func_dn11 = 0.0;
        locals.var_lover_func_dn14 = 0.0;

        locals.var_q_nqs_a = 0.0;
        locals.var_q_nqs_a_dn16 = 0.0;

        locals.var_q_nqs_k = 0.0;
        locals.var_q_nqs_k_dn17 = 0.0;

        locals.var_w_nqs_a = 0.0;
        locals.var_w_nqs_a_dn18 = 0.0;

        locals.var_w_res = 0.0;
        locals.var_w_res_dn0 = 0.0;
        locals.var_w_res_dn2 = 0.0;
        locals.var_w_res_dn4 = 0.0;
        locals.var_w_res_dn5 = 0.0;
        locals.var_w_res_dn6 = 0.0;
        locals.var_w_res_dn7 = 0.0;
        locals.var_w_res_dn8 = 0.0;
        locals.var_w_res_dn9 = 0.0;
        locals.var_w_res_dn10 = 0.0;
        locals.var_w_res_dn11 = 0.0;
        locals.var_w_res_dn14 = 0.0;

        locals.var_wdep_func = 0.0;
        locals.var_wdep_func_dn0 = 0.0;
        locals.var_wdep_func_dn2 = 0.0;
        locals.var_wdep_func_dn4 = 0.0;
        locals.var_wdep_func_dn5 = 0.0;
        locals.var_wdep_func_dn6 = 0.0;
        locals.var_wdep_func_dn7 = 0.0;
        locals.var_wdep_func_dn8 = 0.0;
        locals.var_wdep_func_dn9 = 0.0;
        locals.var_wdep_func_dn10 = 0.0;
        locals.var_wdep_func_dn11 = 0.0;
        locals.var_wdep_func_dn14 = 0.0;

        locals.var_wibjt = 0.0;
        locals.var_wibjt_dn0 = 0.0;
        locals.var_wibjt_dn2 = 0.0;
        locals.var_wibjt_dn4 = 0.0;
        locals.var_wibjt_dn5 = 0.0;
        locals.var_wibjt_dn6 = 0.0;
        locals.var_wibjt_dn7 = 0.0;
        locals.var_wibjt_dn8 = 0.0;
        locals.var_wibjt_dn9 = 0.0;
        locals.var_wibjt_dn10 = 0.0;
        locals.var_wibjt_dn11 = 0.0;
        locals.var_wibjt_dn14 = 0.0;

        locals.var_wk_ii = 0.0;
        locals.var_wk_ii_dn0 = 0.0;
        locals.var_wk_ii_dn2 = 0.0;
        locals.var_wk_ii_dn4 = 0.0;
        locals.var_wk_ii_dn5 = 0.0;
        locals.var_wk_ii_dn6 = 0.0;
        locals.var_wk_ii_dn7 = 0.0;
        locals.var_wk_ii_dn8 = 0.0;
        locals.var_wk_ii_dn9 = 0.0;
        locals.var_wk_ii_dn10 = 0.0;
        locals.var_wk_ii_dn11 = 0.0;
        locals.var_wk_ii_dn14 = 0.0;

        locals.var_tauov = 0.0;
        locals.var_tauov_dn0 = 0.0;
        locals.var_tauov_dn2 = 0.0;
        locals.var_tauov_dn4 = 0.0;
        locals.var_tauov_dn5 = 0.0;
        locals.var_tauov_dn6 = 0.0;
        locals.var_tauov_dn7 = 0.0;
        locals.var_tauov_dn8 = 0.0;
        locals.var_tauov_dn9 = 0.0;
        locals.var_tauov_dn10 = 0.0;
        locals.var_tauov_dn11 = 0.0;
        locals.var_tauov_dn14 = 0.0;

        let (assign5340_e1947,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (p.p17,)
    }
};
        locals.var_uc_corsrd = assign5340_e1947;

        locals.var_uc_xpdv = p.p104;

        locals.var_uc_xldld = p.p294;

        locals.var_uc_scp22 = p.p222;

        locals.var_uc_rdrcx = p.p420;

        locals.var_mfactor = 1.0;

        let assign5500_e1990: f64 = if locals.var_uc_scp22 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard10 = assign5500_e1990;

        let (assign5510_e1994,) = {
    if (locals.var_guard10 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5510_e1994;

        let assign5520_e1997: f64 = if locals.var_uc_scp22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign5520_e1997;

        let (assign5530_e2001,) = {
    if (locals.var_guard11 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5530_e2001;

        let assign5550_e2009: f64 = if locals.var_uc_xldld < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign5550_e2009;

        let (assign5560_e2013,) = {
    if (locals.var_guard13 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_xldld,)
    }
};
        locals.var_uc_xldld = assign5560_e2013;

        let assign5590_e2026: f64 = if locals.var_uc_rdrcx < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign5590_e2026;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5600_e2030,) = {
    if (locals.var_guard16 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5600_e2030;

        let assign5610_e2033: f64 = if locals.var_uc_rdrcx > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard17 = assign5610_e2033;

        let (assign5620_e2037,) = {
    if (locals.var_guard17 != 0.0) {
        (1.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5620_e2037;

        locals.var_uc_ndepm = p.p340;
        locals.var_uc_ndepm_dn0 = 0.0;
        locals.var_uc_ndepm_dn2 = 0.0;
        locals.var_uc_ndepm_dn4 = 0.0;
        locals.var_uc_ndepm_dn5 = 0.0;
        locals.var_uc_ndepm_dn6 = 0.0;
        locals.var_uc_ndepm_dn7 = 0.0;
        locals.var_uc_ndepm_dn8 = 0.0;
        locals.var_uc_ndepm_dn9 = 0.0;
        locals.var_uc_ndepm_dn10 = 0.0;
        locals.var_uc_ndepm_dn11 = 0.0;
        locals.var_uc_ndepm_dn14 = 0.0;

        locals.var_uc_depthn = p.p343;
        locals.var_uc_depthn_dn0 = 0.0;
        locals.var_uc_depthn_dn2 = 0.0;
        locals.var_uc_depthn_dn4 = 0.0;
        locals.var_uc_depthn_dn5 = 0.0;
        locals.var_uc_depthn_dn6 = 0.0;
        locals.var_uc_depthn_dn7 = 0.0;
        locals.var_uc_depthn_dn8 = 0.0;
        locals.var_uc_depthn_dn9 = 0.0;
        locals.var_uc_depthn_dn10 = 0.0;
        locals.var_uc_depthn_dn11 = 0.0;
        locals.var_uc_depthn_dn14 = 0.0;

        locals.var_uc_codep = p.p42;

        locals.var_uc_depmueback0 = p.p354;
        locals.var_uc_depmueback0_dn0 = 0.0;
        locals.var_uc_depmueback0_dn2 = 0.0;
        locals.var_uc_depmueback0_dn4 = 0.0;
        locals.var_uc_depmueback0_dn5 = 0.0;
        locals.var_uc_depmueback0_dn6 = 0.0;
        locals.var_uc_depmueback0_dn7 = 0.0;
        locals.var_uc_depmueback0_dn8 = 0.0;
        locals.var_uc_depmueback0_dn9 = 0.0;
        locals.var_uc_depmueback0_dn10 = 0.0;
        locals.var_uc_depmueback0_dn11 = 0.0;
        locals.var_uc_depmueback0_dn14 = 0.0;

        locals.var_uc_depmueback1 = p.p355;
        locals.var_uc_depmueback1_dn0 = 0.0;
        locals.var_uc_depmueback1_dn2 = 0.0;
        locals.var_uc_depmueback1_dn4 = 0.0;
        locals.var_uc_depmueback1_dn5 = 0.0;
        locals.var_uc_depmueback1_dn6 = 0.0;
        locals.var_uc_depmueback1_dn7 = 0.0;
        locals.var_uc_depmueback1_dn8 = 0.0;
        locals.var_uc_depmueback1_dn9 = 0.0;
        locals.var_uc_depmueback1_dn10 = 0.0;
        locals.var_uc_depmueback1_dn11 = 0.0;
        locals.var_uc_depmueback1_dn14 = 0.0;

        locals.var_uc_depmue0 = p.p346;
        locals.var_uc_depmue0_dn0 = 0.0;
        locals.var_uc_depmue0_dn2 = 0.0;
        locals.var_uc_depmue0_dn4 = 0.0;
        locals.var_uc_depmue0_dn5 = 0.0;
        locals.var_uc_depmue0_dn6 = 0.0;
        locals.var_uc_depmue0_dn7 = 0.0;
        locals.var_uc_depmue0_dn8 = 0.0;
        locals.var_uc_depmue0_dn9 = 0.0;
        locals.var_uc_depmue0_dn10 = 0.0;
        locals.var_uc_depmue0_dn11 = 0.0;
        locals.var_uc_depmue0_dn14 = 0.0;

        locals.var_uc_depmue1 = p.p349;
        locals.var_uc_depmue1_dn0 = 0.0;
        locals.var_uc_depmue1_dn2 = 0.0;
        locals.var_uc_depmue1_dn4 = 0.0;
        locals.var_uc_depmue1_dn5 = 0.0;
        locals.var_uc_depmue1_dn6 = 0.0;
        locals.var_uc_depmue1_dn7 = 0.0;
        locals.var_uc_depmue1_dn8 = 0.0;
        locals.var_uc_depmue1_dn9 = 0.0;
        locals.var_uc_depmue1_dn10 = 0.0;
        locals.var_uc_depmue1_dn11 = 0.0;
        locals.var_uc_depmue1_dn14 = 0.0;

        locals.var_uc_depmue2 = p.p352;
        locals.var_uc_depmue2_dn0 = 0.0;
        locals.var_uc_depmue2_dn2 = 0.0;
        locals.var_uc_depmue2_dn4 = 0.0;
        locals.var_uc_depmue2_dn5 = 0.0;
        locals.var_uc_depmue2_dn6 = 0.0;
        locals.var_uc_depmue2_dn7 = 0.0;
        locals.var_uc_depmue2_dn8 = 0.0;
        locals.var_uc_depmue2_dn9 = 0.0;
        locals.var_uc_depmue2_dn10 = 0.0;
        locals.var_uc_depmue2_dn11 = 0.0;
        locals.var_uc_depmue2_dn14 = 0.0;

        locals.var_uc_depleak = p.p360;
        locals.var_uc_depleak_dn0 = 0.0;
        locals.var_uc_depleak_dn2 = 0.0;
        locals.var_uc_depleak_dn4 = 0.0;
        locals.var_uc_depleak_dn5 = 0.0;
        locals.var_uc_depleak_dn6 = 0.0;
        locals.var_uc_depleak_dn7 = 0.0;
        locals.var_uc_depleak_dn8 = 0.0;
        locals.var_uc_depleak_dn9 = 0.0;
        locals.var_uc_depleak_dn10 = 0.0;
        locals.var_uc_depleak_dn11 = 0.0;
        locals.var_uc_depleak_dn14 = 0.0;

        locals.var_uc_depvmax = p.p367;
        locals.var_uc_depvmax_dn0 = 0.0;
        locals.var_uc_depvmax_dn2 = 0.0;
        locals.var_uc_depvmax_dn4 = 0.0;
        locals.var_uc_depvmax_dn5 = 0.0;
        locals.var_uc_depvmax_dn6 = 0.0;
        locals.var_uc_depvmax_dn7 = 0.0;
        locals.var_uc_depvmax_dn8 = 0.0;
        locals.var_uc_depvmax_dn9 = 0.0;
        locals.var_uc_depvmax_dn10 = 0.0;
        locals.var_uc_depvmax_dn11 = 0.0;
        locals.var_uc_depvmax_dn14 = 0.0;

        locals.var_uc_depwlp = p.p364;
        locals.var_uc_depwlp_dn0 = 0.0;
        locals.var_uc_depwlp_dn2 = 0.0;
        locals.var_uc_depwlp_dn4 = 0.0;
        locals.var_uc_depwlp_dn5 = 0.0;
        locals.var_uc_depwlp_dn6 = 0.0;
        locals.var_uc_depwlp_dn7 = 0.0;
        locals.var_uc_depwlp_dn8 = 0.0;
        locals.var_uc_depwlp_dn9 = 0.0;
        locals.var_uc_depwlp_dn10 = 0.0;
        locals.var_uc_depwlp_dn11 = 0.0;
        locals.var_uc_depwlp_dn14 = 0.0;

        locals.var_uc_depmueph1 = p.p377;

        locals.var_uc_depvdsef1 = p.p370;
        locals.var_uc_depvdsef1_dn0 = 0.0;
        locals.var_uc_depvdsef1_dn2 = 0.0;
        locals.var_uc_depvdsef1_dn4 = 0.0;
        locals.var_uc_depvdsef1_dn5 = 0.0;
        locals.var_uc_depvdsef1_dn6 = 0.0;
        locals.var_uc_depvdsef1_dn7 = 0.0;
        locals.var_uc_depvdsef1_dn8 = 0.0;
        locals.var_uc_depvdsef1_dn9 = 0.0;
        locals.var_uc_depvdsef1_dn10 = 0.0;
        locals.var_uc_depvdsef1_dn11 = 0.0;
        locals.var_uc_depvdsef1_dn14 = 0.0;

        locals.var_uc_depvdsef2 = p.p371;
        locals.var_uc_depvdsef2_dn0 = 0.0;
        locals.var_uc_depvdsef2_dn2 = 0.0;
        locals.var_uc_depvdsef2_dn4 = 0.0;
        locals.var_uc_depvdsef2_dn5 = 0.0;
        locals.var_uc_depvdsef2_dn6 = 0.0;
        locals.var_uc_depvdsef2_dn7 = 0.0;
        locals.var_uc_depvdsef2_dn8 = 0.0;
        locals.var_uc_depvdsef2_dn9 = 0.0;
        locals.var_uc_depvdsef2_dn10 = 0.0;
        locals.var_uc_depvdsef2_dn11 = 0.0;
        locals.var_uc_depvdsef2_dn14 = 0.0;

        let assign6710_e2710: f64 = if ((locals.var_uc_codep < 3.0) && (locals.var_uc_codep > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard112 = assign6710_e2710;

        let assign6740_e2723: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard115 = assign6740_e2723;

        let (assign6750_e2729, assign6750_e2729_d_n0, assign6750_e2729_d_n2, assign6750_e2729_d_n4, assign6750_e2729_d_n5, assign6750_e2729_d_n6, assign6750_e2729_d_n7, assign6750_e2729_d_n8, assign6750_e2729_d_n9, assign6750_e2729_d_n10, assign6750_e2729_d_n11, assign6750_e2729_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard115 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign6750_e2729;
        locals.var_uc_ndepm_dn0 = assign6750_e2729_d_n0;
        locals.var_uc_ndepm_dn2 = assign6750_e2729_d_n2;
        locals.var_uc_ndepm_dn4 = assign6750_e2729_d_n4;
        locals.var_uc_ndepm_dn5 = assign6750_e2729_d_n5;
        locals.var_uc_ndepm_dn6 = assign6750_e2729_d_n6;
        locals.var_uc_ndepm_dn7 = assign6750_e2729_d_n7;
        locals.var_uc_ndepm_dn8 = assign6750_e2729_d_n8;
        locals.var_uc_ndepm_dn9 = assign6750_e2729_d_n9;
        locals.var_uc_ndepm_dn10 = assign6750_e2729_d_n10;
        locals.var_uc_ndepm_dn11 = assign6750_e2729_d_n11;
        locals.var_uc_ndepm_dn14 = assign6750_e2729_d_n14;

        let assign6760_e2732: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard116 = assign6760_e2732;

        let (assign6770_e2738, assign6770_e2738_d_n0, assign6770_e2738_d_n2, assign6770_e2738_d_n4, assign6770_e2738_d_n5, assign6770_e2738_d_n6, assign6770_e2738_d_n7, assign6770_e2738_d_n8, assign6770_e2738_d_n9, assign6770_e2738_d_n10, assign6770_e2738_d_n11, assign6770_e2738_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard116 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign6770_e2738;
        locals.var_uc_ndepm_dn0 = assign6770_e2738_d_n0;
        locals.var_uc_ndepm_dn2 = assign6770_e2738_d_n2;
        locals.var_uc_ndepm_dn4 = assign6770_e2738_d_n4;
        locals.var_uc_ndepm_dn5 = assign6770_e2738_d_n5;
        locals.var_uc_ndepm_dn6 = assign6770_e2738_d_n6;
        locals.var_uc_ndepm_dn7 = assign6770_e2738_d_n7;
        locals.var_uc_ndepm_dn8 = assign6770_e2738_d_n8;
        locals.var_uc_ndepm_dn9 = assign6770_e2738_d_n9;
        locals.var_uc_ndepm_dn10 = assign6770_e2738_d_n10;
        locals.var_uc_ndepm_dn11 = assign6770_e2738_d_n11;
        locals.var_uc_ndepm_dn14 = assign6770_e2738_d_n14;

        let assign6800_e2751: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard119 = assign6800_e2751;

        let (assign6810_e2757, assign6810_e2757_d_n0, assign6810_e2757_d_n2, assign6810_e2757_d_n4, assign6810_e2757_d_n5, assign6810_e2757_d_n6, assign6810_e2757_d_n7, assign6810_e2757_d_n8, assign6810_e2757_d_n9, assign6810_e2757_d_n10, assign6810_e2757_d_n11, assign6810_e2757_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard119 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign6810_e2757;
        locals.var_uc_depthn_dn0 = assign6810_e2757_d_n0;
        locals.var_uc_depthn_dn2 = assign6810_e2757_d_n2;
        locals.var_uc_depthn_dn4 = assign6810_e2757_d_n4;
        locals.var_uc_depthn_dn5 = assign6810_e2757_d_n5;
        locals.var_uc_depthn_dn6 = assign6810_e2757_d_n6;
        locals.var_uc_depthn_dn7 = assign6810_e2757_d_n7;
        locals.var_uc_depthn_dn8 = assign6810_e2757_d_n8;
        locals.var_uc_depthn_dn9 = assign6810_e2757_d_n9;
        locals.var_uc_depthn_dn10 = assign6810_e2757_d_n10;
        locals.var_uc_depthn_dn11 = assign6810_e2757_d_n11;
        locals.var_uc_depthn_dn14 = assign6810_e2757_d_n14;

        let assign6820_e2760: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard120 = assign6820_e2760;

        let (assign6830_e2766, assign6830_e2766_d_n0, assign6830_e2766_d_n2, assign6830_e2766_d_n4, assign6830_e2766_d_n5, assign6830_e2766_d_n6, assign6830_e2766_d_n7, assign6830_e2766_d_n8, assign6830_e2766_d_n9, assign6830_e2766_d_n10, assign6830_e2766_d_n11, assign6830_e2766_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard120 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign6830_e2766;
        locals.var_uc_depthn_dn0 = assign6830_e2766_d_n0;
        locals.var_uc_depthn_dn2 = assign6830_e2766_d_n2;
        locals.var_uc_depthn_dn4 = assign6830_e2766_d_n4;
        locals.var_uc_depthn_dn5 = assign6830_e2766_d_n5;
        locals.var_uc_depthn_dn6 = assign6830_e2766_d_n6;
        locals.var_uc_depthn_dn7 = assign6830_e2766_d_n7;
        locals.var_uc_depthn_dn8 = assign6830_e2766_d_n8;
        locals.var_uc_depthn_dn9 = assign6830_e2766_d_n9;
        locals.var_uc_depthn_dn10 = assign6830_e2766_d_n10;
        locals.var_uc_depthn_dn11 = assign6830_e2766_d_n11;
        locals.var_uc_depthn_dn14 = assign6830_e2766_d_n14;

        let assign6860_e2779: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard123 = assign6860_e2779;

        let (assign6870_e2785, assign6870_e2785_d_n0, assign6870_e2785_d_n2, assign6870_e2785_d_n4, assign6870_e2785_d_n5, assign6870_e2785_d_n6, assign6870_e2785_d_n7, assign6870_e2785_d_n8, assign6870_e2785_d_n9, assign6870_e2785_d_n10, assign6870_e2785_d_n11, assign6870_e2785_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard123 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign6870_e2785;
        locals.var_uc_depmue0_dn0 = assign6870_e2785_d_n0;
        locals.var_uc_depmue0_dn2 = assign6870_e2785_d_n2;
        locals.var_uc_depmue0_dn4 = assign6870_e2785_d_n4;
        locals.var_uc_depmue0_dn5 = assign6870_e2785_d_n5;
        locals.var_uc_depmue0_dn6 = assign6870_e2785_d_n6;
        locals.var_uc_depmue0_dn7 = assign6870_e2785_d_n7;
        locals.var_uc_depmue0_dn8 = assign6870_e2785_d_n8;
        locals.var_uc_depmue0_dn9 = assign6870_e2785_d_n9;
        locals.var_uc_depmue0_dn10 = assign6870_e2785_d_n10;
        locals.var_uc_depmue0_dn11 = assign6870_e2785_d_n11;
        locals.var_uc_depmue0_dn14 = assign6870_e2785_d_n14;

        let assign6880_e2788: f64 = if locals.var_uc_depmue0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard124 = assign6880_e2788;

        let (assign6890_e2794, assign6890_e2794_d_n0, assign6890_e2794_d_n2, assign6890_e2794_d_n4, assign6890_e2794_d_n5, assign6890_e2794_d_n6, assign6890_e2794_d_n7, assign6890_e2794_d_n8, assign6890_e2794_d_n9, assign6890_e2794_d_n10, assign6890_e2794_d_n11, assign6890_e2794_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard124 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign6890_e2794;
        locals.var_uc_depmue0_dn0 = assign6890_e2794_d_n0;
        locals.var_uc_depmue0_dn2 = assign6890_e2794_d_n2;
        locals.var_uc_depmue0_dn4 = assign6890_e2794_d_n4;
        locals.var_uc_depmue0_dn5 = assign6890_e2794_d_n5;
        locals.var_uc_depmue0_dn6 = assign6890_e2794_d_n6;
        locals.var_uc_depmue0_dn7 = assign6890_e2794_d_n7;
        locals.var_uc_depmue0_dn8 = assign6890_e2794_d_n8;
        locals.var_uc_depmue0_dn9 = assign6890_e2794_d_n9;
        locals.var_uc_depmue0_dn10 = assign6890_e2794_d_n10;
        locals.var_uc_depmue0_dn11 = assign6890_e2794_d_n11;
        locals.var_uc_depmue0_dn14 = assign6890_e2794_d_n14;

        let assign6920_e2807: f64 = if locals.var_uc_depmueback0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard127 = assign6920_e2807;

        let (assign6930_e2813, assign6930_e2813_d_n0, assign6930_e2813_d_n2, assign6930_e2813_d_n4, assign6930_e2813_d_n5, assign6930_e2813_d_n6, assign6930_e2813_d_n7, assign6930_e2813_d_n8, assign6930_e2813_d_n9, assign6930_e2813_d_n10, assign6930_e2813_d_n11, assign6930_e2813_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard127 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign6930_e2813;
        locals.var_uc_depmueback0_dn0 = assign6930_e2813_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6930_e2813_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6930_e2813_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6930_e2813_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6930_e2813_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6930_e2813_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6930_e2813_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6930_e2813_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6930_e2813_d_n10;
        locals.var_uc_depmueback0_dn11 = assign6930_e2813_d_n11;
        locals.var_uc_depmueback0_dn14 = assign6930_e2813_d_n14;

        let assign6940_e2816: f64 = if locals.var_uc_depmueback0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard128 = assign6940_e2816;

        let (assign6950_e2822, assign6950_e2822_d_n0, assign6950_e2822_d_n2, assign6950_e2822_d_n4, assign6950_e2822_d_n5, assign6950_e2822_d_n6, assign6950_e2822_d_n7, assign6950_e2822_d_n8, assign6950_e2822_d_n9, assign6950_e2822_d_n10, assign6950_e2822_d_n11, assign6950_e2822_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard128 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign6950_e2822;
        locals.var_uc_depmueback0_dn0 = assign6950_e2822_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6950_e2822_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6950_e2822_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6950_e2822_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6950_e2822_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6950_e2822_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6950_e2822_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6950_e2822_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6950_e2822_d_n10;
        locals.var_uc_depmueback0_dn11 = assign6950_e2822_d_n11;
        locals.var_uc_depmueback0_dn14 = assign6950_e2822_d_n14;

        let assign6980_e2835: f64 = if locals.var_uc_depmueph1 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard131 = assign6980_e2835;

        let (assign6990_e2841,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard131 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign6990_e2841;

        let assign7000_e2844: f64 = if locals.var_uc_depmueph1 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard132 = assign7000_e2844;

        let (assign7010_e2850,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard132 != 0.0)) {
        (100000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7010_e2850;

        let assign7040_e2863: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard135 = assign7040_e2863;

        let (assign7050_e2869, assign7050_e2869_d_n0, assign7050_e2869_d_n2, assign7050_e2869_d_n4, assign7050_e2869_d_n5, assign7050_e2869_d_n6, assign7050_e2869_d_n7, assign7050_e2869_d_n8, assign7050_e2869_d_n9, assign7050_e2869_d_n10, assign7050_e2869_d_n11, assign7050_e2869_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard135 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign7050_e2869;
        locals.var_uc_depvdsef2_dn0 = assign7050_e2869_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7050_e2869_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7050_e2869_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7050_e2869_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7050_e2869_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7050_e2869_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7050_e2869_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7050_e2869_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7050_e2869_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign7050_e2869_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign7050_e2869_d_n14;

        let assign7060_e2872: f64 = if locals.var_uc_depvdsef2 > 4.0 { 1.0 } else { 0.0 };
        locals.var_guard136 = assign7060_e2872;

        let (assign7070_e2878, assign7070_e2878_d_n0, assign7070_e2878_d_n2, assign7070_e2878_d_n4, assign7070_e2878_d_n5, assign7070_e2878_d_n6, assign7070_e2878_d_n7, assign7070_e2878_d_n8, assign7070_e2878_d_n9, assign7070_e2878_d_n10, assign7070_e2878_d_n11, assign7070_e2878_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard136 != 0.0)) {
        (4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign7070_e2878;
        locals.var_uc_depvdsef2_dn0 = assign7070_e2878_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7070_e2878_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7070_e2878_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7070_e2878_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7070_e2878_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7070_e2878_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7070_e2878_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7070_e2878_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7070_e2878_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign7070_e2878_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign7070_e2878_d_n14;

        let assign7100_e2891: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign7100_e2891;

        let (assign7110_e2897, assign7110_e2897_d_n0, assign7110_e2897_d_n2, assign7110_e2897_d_n4, assign7110_e2897_d_n5, assign7110_e2897_d_n6, assign7110_e2897_d_n7, assign7110_e2897_d_n8, assign7110_e2897_d_n9, assign7110_e2897_d_n10, assign7110_e2897_d_n11, assign7110_e2897_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard139 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7110_e2897;
        locals.var_uc_depleak_dn0 = assign7110_e2897_d_n0;
        locals.var_uc_depleak_dn2 = assign7110_e2897_d_n2;
        locals.var_uc_depleak_dn4 = assign7110_e2897_d_n4;
        locals.var_uc_depleak_dn5 = assign7110_e2897_d_n5;
        locals.var_uc_depleak_dn6 = assign7110_e2897_d_n6;
        locals.var_uc_depleak_dn7 = assign7110_e2897_d_n7;
        locals.var_uc_depleak_dn8 = assign7110_e2897_d_n8;
        locals.var_uc_depleak_dn9 = assign7110_e2897_d_n9;
        locals.var_uc_depleak_dn10 = assign7110_e2897_d_n10;
        locals.var_uc_depleak_dn11 = assign7110_e2897_d_n11;
        locals.var_uc_depleak_dn14 = assign7110_e2897_d_n14;

        let assign7120_e2900: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard140 = assign7120_e2900;

        let (assign7130_e2906, assign7130_e2906_d_n0, assign7130_e2906_d_n2, assign7130_e2906_d_n4, assign7130_e2906_d_n5, assign7130_e2906_d_n6, assign7130_e2906_d_n7, assign7130_e2906_d_n8, assign7130_e2906_d_n9, assign7130_e2906_d_n10, assign7130_e2906_d_n11, assign7130_e2906_d_n14,) = {
    if ((locals.var_guard112 != 0.0) && (locals.var_guard140 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7130_e2906;
        locals.var_uc_depleak_dn0 = assign7130_e2906_d_n0;
        locals.var_uc_depleak_dn2 = assign7130_e2906_d_n2;
        locals.var_uc_depleak_dn4 = assign7130_e2906_d_n4;
        locals.var_uc_depleak_dn5 = assign7130_e2906_d_n5;
        locals.var_uc_depleak_dn6 = assign7130_e2906_d_n6;
        locals.var_uc_depleak_dn7 = assign7130_e2906_d_n7;
        locals.var_uc_depleak_dn8 = assign7130_e2906_d_n8;
        locals.var_uc_depleak_dn9 = assign7130_e2906_d_n9;
        locals.var_uc_depleak_dn10 = assign7130_e2906_d_n10;
        locals.var_uc_depleak_dn11 = assign7130_e2906_d_n11;
        locals.var_uc_depleak_dn14 = assign7130_e2906_d_n14;

        let assign7140_e2909: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign7140_e2909;

        let assign7170_e2922: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard144 = assign7170_e2922;

        let (assign7180_e2931, assign7180_e2931_d_n0, assign7180_e2931_d_n2, assign7180_e2931_d_n4, assign7180_e2931_d_n5, assign7180_e2931_d_n6, assign7180_e2931_d_n7, assign7180_e2931_d_n8, assign7180_e2931_d_n9, assign7180_e2931_d_n10, assign7180_e2931_d_n11, assign7180_e2931_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard144 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign7180_e2931;
        locals.var_uc_ndepm_dn0 = assign7180_e2931_d_n0;
        locals.var_uc_ndepm_dn2 = assign7180_e2931_d_n2;
        locals.var_uc_ndepm_dn4 = assign7180_e2931_d_n4;
        locals.var_uc_ndepm_dn5 = assign7180_e2931_d_n5;
        locals.var_uc_ndepm_dn6 = assign7180_e2931_d_n6;
        locals.var_uc_ndepm_dn7 = assign7180_e2931_d_n7;
        locals.var_uc_ndepm_dn8 = assign7180_e2931_d_n8;
        locals.var_uc_ndepm_dn9 = assign7180_e2931_d_n9;
        locals.var_uc_ndepm_dn10 = assign7180_e2931_d_n10;
        locals.var_uc_ndepm_dn11 = assign7180_e2931_d_n11;
        locals.var_uc_ndepm_dn14 = assign7180_e2931_d_n14;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7190_e2934: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard145 = assign7190_e2934;

        let (assign7200_e2943, assign7200_e2943_d_n0, assign7200_e2943_d_n2, assign7200_e2943_d_n4, assign7200_e2943_d_n5, assign7200_e2943_d_n6, assign7200_e2943_d_n7, assign7200_e2943_d_n8, assign7200_e2943_d_n9, assign7200_e2943_d_n10, assign7200_e2943_d_n11, assign7200_e2943_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard145 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign7200_e2943;
        locals.var_uc_ndepm_dn0 = assign7200_e2943_d_n0;
        locals.var_uc_ndepm_dn2 = assign7200_e2943_d_n2;
        locals.var_uc_ndepm_dn4 = assign7200_e2943_d_n4;
        locals.var_uc_ndepm_dn5 = assign7200_e2943_d_n5;
        locals.var_uc_ndepm_dn6 = assign7200_e2943_d_n6;
        locals.var_uc_ndepm_dn7 = assign7200_e2943_d_n7;
        locals.var_uc_ndepm_dn8 = assign7200_e2943_d_n8;
        locals.var_uc_ndepm_dn9 = assign7200_e2943_d_n9;
        locals.var_uc_ndepm_dn10 = assign7200_e2943_d_n10;
        locals.var_uc_ndepm_dn11 = assign7200_e2943_d_n11;
        locals.var_uc_ndepm_dn14 = assign7200_e2943_d_n14;

        let assign7230_e2956: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard148 = assign7230_e2956;

        let (assign7240_e2965, assign7240_e2965_d_n0, assign7240_e2965_d_n2, assign7240_e2965_d_n4, assign7240_e2965_d_n5, assign7240_e2965_d_n6, assign7240_e2965_d_n7, assign7240_e2965_d_n8, assign7240_e2965_d_n9, assign7240_e2965_d_n10, assign7240_e2965_d_n11, assign7240_e2965_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard148 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign7240_e2965;
        locals.var_uc_depthn_dn0 = assign7240_e2965_d_n0;
        locals.var_uc_depthn_dn2 = assign7240_e2965_d_n2;
        locals.var_uc_depthn_dn4 = assign7240_e2965_d_n4;
        locals.var_uc_depthn_dn5 = assign7240_e2965_d_n5;
        locals.var_uc_depthn_dn6 = assign7240_e2965_d_n6;
        locals.var_uc_depthn_dn7 = assign7240_e2965_d_n7;
        locals.var_uc_depthn_dn8 = assign7240_e2965_d_n8;
        locals.var_uc_depthn_dn9 = assign7240_e2965_d_n9;
        locals.var_uc_depthn_dn10 = assign7240_e2965_d_n10;
        locals.var_uc_depthn_dn11 = assign7240_e2965_d_n11;
        locals.var_uc_depthn_dn14 = assign7240_e2965_d_n14;

        let assign7250_e2968: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard149 = assign7250_e2968;

        let (assign7260_e2977, assign7260_e2977_d_n0, assign7260_e2977_d_n2, assign7260_e2977_d_n4, assign7260_e2977_d_n5, assign7260_e2977_d_n6, assign7260_e2977_d_n7, assign7260_e2977_d_n8, assign7260_e2977_d_n9, assign7260_e2977_d_n10, assign7260_e2977_d_n11, assign7260_e2977_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard149 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign7260_e2977;
        locals.var_uc_depthn_dn0 = assign7260_e2977_d_n0;
        locals.var_uc_depthn_dn2 = assign7260_e2977_d_n2;
        locals.var_uc_depthn_dn4 = assign7260_e2977_d_n4;
        locals.var_uc_depthn_dn5 = assign7260_e2977_d_n5;
        locals.var_uc_depthn_dn6 = assign7260_e2977_d_n6;
        locals.var_uc_depthn_dn7 = assign7260_e2977_d_n7;
        locals.var_uc_depthn_dn8 = assign7260_e2977_d_n8;
        locals.var_uc_depthn_dn9 = assign7260_e2977_d_n9;
        locals.var_uc_depthn_dn10 = assign7260_e2977_d_n10;
        locals.var_uc_depthn_dn11 = assign7260_e2977_d_n11;
        locals.var_uc_depthn_dn14 = assign7260_e2977_d_n14;

        let assign7290_e2990: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard152 = assign7290_e2990;

        let (assign7300_e2999, assign7300_e2999_d_n0, assign7300_e2999_d_n2, assign7300_e2999_d_n4, assign7300_e2999_d_n5, assign7300_e2999_d_n6, assign7300_e2999_d_n7, assign7300_e2999_d_n8, assign7300_e2999_d_n9, assign7300_e2999_d_n10, assign7300_e2999_d_n11, assign7300_e2999_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard152 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign7300_e2999;
        locals.var_uc_depmue0_dn0 = assign7300_e2999_d_n0;
        locals.var_uc_depmue0_dn2 = assign7300_e2999_d_n2;
        locals.var_uc_depmue0_dn4 = assign7300_e2999_d_n4;
        locals.var_uc_depmue0_dn5 = assign7300_e2999_d_n5;
        locals.var_uc_depmue0_dn6 = assign7300_e2999_d_n6;
        locals.var_uc_depmue0_dn7 = assign7300_e2999_d_n7;
        locals.var_uc_depmue0_dn8 = assign7300_e2999_d_n8;
        locals.var_uc_depmue0_dn9 = assign7300_e2999_d_n9;
        locals.var_uc_depmue0_dn10 = assign7300_e2999_d_n10;
        locals.var_uc_depmue0_dn11 = assign7300_e2999_d_n11;
        locals.var_uc_depmue0_dn14 = assign7300_e2999_d_n14;

        let assign7310_e3002: f64 = if locals.var_uc_depmue0 > 10000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard153 = assign7310_e3002;

        let (assign7320_e3011, assign7320_e3011_d_n0, assign7320_e3011_d_n2, assign7320_e3011_d_n4, assign7320_e3011_d_n5, assign7320_e3011_d_n6, assign7320_e3011_d_n7, assign7320_e3011_d_n8, assign7320_e3011_d_n9, assign7320_e3011_d_n10, assign7320_e3011_d_n11, assign7320_e3011_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard153 != 0.0)) {
        (10000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign7320_e3011;
        locals.var_uc_depmue0_dn0 = assign7320_e3011_d_n0;
        locals.var_uc_depmue0_dn2 = assign7320_e3011_d_n2;
        locals.var_uc_depmue0_dn4 = assign7320_e3011_d_n4;
        locals.var_uc_depmue0_dn5 = assign7320_e3011_d_n5;
        locals.var_uc_depmue0_dn6 = assign7320_e3011_d_n6;
        locals.var_uc_depmue0_dn7 = assign7320_e3011_d_n7;
        locals.var_uc_depmue0_dn8 = assign7320_e3011_d_n8;
        locals.var_uc_depmue0_dn9 = assign7320_e3011_d_n9;
        locals.var_uc_depmue0_dn10 = assign7320_e3011_d_n10;
        locals.var_uc_depmue0_dn11 = assign7320_e3011_d_n11;
        locals.var_uc_depmue0_dn14 = assign7320_e3011_d_n14;

        let assign7350_e3024: f64 = if locals.var_uc_depmueph1 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard156 = assign7350_e3024;

        let (assign7360_e3033,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard156 != 0.0)) {
        (100.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7360_e3033;

        let assign7370_e3036: f64 = if locals.var_uc_depmueph1 > 2000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard157 = assign7370_e3036;

        let (assign7380_e3045,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard157 != 0.0)) {
        (2000000000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7380_e3045;

        let assign7410_e3058: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard160 = assign7410_e3058;

        let (assign7420_e3067, assign7420_e3067_d_n0, assign7420_e3067_d_n2, assign7420_e3067_d_n4, assign7420_e3067_d_n5, assign7420_e3067_d_n6, assign7420_e3067_d_n7, assign7420_e3067_d_n8, assign7420_e3067_d_n9, assign7420_e3067_d_n10, assign7420_e3067_d_n11, assign7420_e3067_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard160 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7420_e3067;
        locals.var_uc_depleak_dn0 = assign7420_e3067_d_n0;
        locals.var_uc_depleak_dn2 = assign7420_e3067_d_n2;
        locals.var_uc_depleak_dn4 = assign7420_e3067_d_n4;
        locals.var_uc_depleak_dn5 = assign7420_e3067_d_n5;
        locals.var_uc_depleak_dn6 = assign7420_e3067_d_n6;
        locals.var_uc_depleak_dn7 = assign7420_e3067_d_n7;
        locals.var_uc_depleak_dn8 = assign7420_e3067_d_n8;
        locals.var_uc_depleak_dn9 = assign7420_e3067_d_n9;
        locals.var_uc_depleak_dn10 = assign7420_e3067_d_n10;
        locals.var_uc_depleak_dn11 = assign7420_e3067_d_n11;
        locals.var_uc_depleak_dn14 = assign7420_e3067_d_n14;

        let assign7430_e3070: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard161 = assign7430_e3070;

        let (assign7440_e3079, assign7440_e3079_d_n0, assign7440_e3079_d_n2, assign7440_e3079_d_n4, assign7440_e3079_d_n5, assign7440_e3079_d_n6, assign7440_e3079_d_n7, assign7440_e3079_d_n8, assign7440_e3079_d_n9, assign7440_e3079_d_n10, assign7440_e3079_d_n11, assign7440_e3079_d_n14,) = {
    if (((locals.var_guard112 == 0.0) && (locals.var_guard141 != 0.0)) && (locals.var_guard161 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7440_e3079;
        locals.var_uc_depleak_dn0 = assign7440_e3079_d_n0;
        locals.var_uc_depleak_dn2 = assign7440_e3079_d_n2;
        locals.var_uc_depleak_dn4 = assign7440_e3079_d_n4;
        locals.var_uc_depleak_dn5 = assign7440_e3079_d_n5;
        locals.var_uc_depleak_dn6 = assign7440_e3079_d_n6;
        locals.var_uc_depleak_dn7 = assign7440_e3079_d_n7;
        locals.var_uc_depleak_dn8 = assign7440_e3079_d_n8;
        locals.var_uc_depleak_dn9 = assign7440_e3079_d_n9;
        locals.var_uc_depleak_dn10 = assign7440_e3079_d_n10;
        locals.var_uc_depleak_dn11 = assign7440_e3079_d_n11;
        locals.var_uc_depleak_dn14 = assign7440_e3079_d_n14;

        locals.var_uc_toxb = p.p96;

        let assign7540_e3117: f64 = if locals.var_uc_toxb < p.p95 { 1.0 } else { 0.0 };
        locals.var_guard170 = assign7540_e3117;

        let (assign7550_e3121,) = {
    if (locals.var_guard170 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7550_e3121;

        let assign7560_e3124: f64 = if locals.var_uc_toxb > 5e-7 { 1.0 } else { 0.0 };
        locals.var_guard171 = assign7560_e3124;

        let (assign7570_e3128,) = {
    if (locals.var_guard171 != 0.0) {
        (5e-7,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7570_e3128;

        let assign7580_e3132: f64 = (100.0_f64).powf(p.p122);
        let assign7580_e3133: f64 = (p.p120 / assign7580_e3132);
        locals.var_mks_ll = assign7580_e3133;

        let assign7590_e3137: f64 = (100.0_f64).powf(p.p129);
        let assign7590_e3138: f64 = (p.p123 / assign7590_e3137);
        locals.var_mks_wl = assign7590_e3138;

        let assign7600_e3142: f64 = (100.0_f64).powf(p.p199);
        let assign7600_e3143: f64 = (p.p198 / assign7600_e3142);
        locals.var_mks_svgsl = assign7600_e3143;

        let assign7610_e3147: f64 = (100.0_f64).powf(p.p201);
        let assign7610_e3148: f64 = (p.p200 / assign7610_e3147);
        locals.var_mks_svgsw = assign7610_e3148;

        let assign7620_e3152: f64 = (100.0_f64).powf(p.p184);
        let assign7620_e3153: f64 = (p.p183 / assign7620_e3152);
        locals.var_mks_svbsl = assign7620_e3153;

        let assign7630_e3157: f64 = (100.0_f64).powf(p.p203);
        let assign7630_e3158: f64 = (p.p202 / assign7630_e3157);
        locals.var_mks_slgl = assign7630_e3158;

        let assign7640_e3162: f64 = (100.0_f64).powf(p.p191);
        let assign7640_e3163: f64 = (p.p190 / assign7640_e3162);
        locals.var_mks_sub1l = assign7640_e3163;

        let assign7650_e3166: f64 = (p.p186 / 100.0);
        locals.var_mks_slg = assign7650_e3166;

        let assign7660_e3169: f64 = (p.p192 / 100.0);
        locals.var_mks_sub2l = assign7660_e3169;

        let assign7670_e3172: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign7670_e3172;

        let assign7680_e3175: f64 = (p.p311 / 100.0);
        locals.var_mks_rdtemp1 = assign7680_e3175;

        let assign7690_e3178: f64 = (p.p312 / 100.0);
        locals.var_mks_rdtemp2 = assign7690_e3178;

        let assign7700_e3181: f64 = (p.p313 / 100.0);
        locals.var_mks_rdvdtemp1 = assign7700_e3181;

        let assign7710_e3184: f64 = (p.p314 / 100.0);
        locals.var_mks_rdvdtemp2 = assign7710_e3184;

        let assign7720_e3187: f64 = (p.p336 / 1e-6);
        locals.var_mks_nsubsub = assign7720_e3187;

        let assign7730_e3190: f64 = (p.p255 * 100.0);
        locals.var_mks_glksd3 = assign7730_e3190;

        let assign7740_e3193: f64 = (p.p248 * 100.0);
        locals.var_mks_gleak4 = assign7740_e3193;

        let assign7750_e3196: f64 = (p.p249 * 100.0);
        locals.var_mks_gleak5 = assign7750_e3196;

        let assign7760_e3199: f64 = (p.p251 / 10000.0);
        locals.var_mks_gleak7 = assign7760_e3199;

        let assign7770_e3202: f64 = (p.p266 * 10000.0);
        locals.var_mks_cit = assign7770_e3202;

        let assign7780_e3205: f64 = (p.p275 / 100.0);
        locals.var_mks_ovslp = assign7780_e3205;

        let assign7790_e3208: f64 = (p.p272 / 10000.0);
        locals.var_mks_dly3 = assign7790_e3208;

        let assign7800_e3211: f64 = (p.p273 / 10000.0);
        locals.var_mks_dlyov = assign7800_e3211;
        locals.var_mks_dlyov_dn0 = 0.0;
        locals.var_mks_dlyov_dn2 = 0.0;
        locals.var_mks_dlyov_dn4 = 0.0;
        locals.var_mks_dlyov_dn5 = 0.0;
        locals.var_mks_dlyov_dn6 = 0.0;
        locals.var_mks_dlyov_dn7 = 0.0;
        locals.var_mks_dlyov_dn8 = 0.0;
        locals.var_mks_dlyov_dn9 = 0.0;
        locals.var_mks_dlyov_dn10 = 0.0;
        locals.var_mks_dlyov_dn11 = 0.0;
        locals.var_mks_dlyov_dn14 = 0.0;

        let assign7820_e3217: f64 = (p.p409 / 10000.0);
        locals.var_mks_rdrmue = assign7820_e3217;

        let assign7830_e3220: f64 = (p.p412 / 100.0);
        locals.var_mks_rdrvmax = assign7830_e3220;

        let assign7840_e3223: f64 = (p.p413 / 10000.0);
        locals.var_mks_rdrmues = assign7840_e3223;

        let assign7850_e3226: f64 = (p.p414 / 100.0);
        locals.var_mks_rdrvmaxs = assign7850_e3226;

        let assign7860_e3229: f64 = (locals.var_uc_ndepm / 1e-6);
        locals.var_uc_ndepm = assign7860_e3229;
        locals.var_uc_ndepm_dn0 = (locals.var_uc_ndepm_dn0 / 1e-6);
        locals.var_uc_ndepm_dn2 = (locals.var_uc_ndepm_dn2 / 1e-6);
        locals.var_uc_ndepm_dn4 = (locals.var_uc_ndepm_dn4 / 1e-6);
        locals.var_uc_ndepm_dn5 = (locals.var_uc_ndepm_dn5 / 1e-6);
        locals.var_uc_ndepm_dn6 = (locals.var_uc_ndepm_dn6 / 1e-6);
        locals.var_uc_ndepm_dn7 = (locals.var_uc_ndepm_dn7 / 1e-6);
        locals.var_uc_ndepm_dn8 = (locals.var_uc_ndepm_dn8 / 1e-6);
        locals.var_uc_ndepm_dn9 = (locals.var_uc_ndepm_dn9 / 1e-6);
        locals.var_uc_ndepm_dn10 = (locals.var_uc_ndepm_dn10 / 1e-6);
        locals.var_uc_ndepm_dn11 = (locals.var_uc_ndepm_dn11 / 1e-6);
        locals.var_uc_ndepm_dn14 = (locals.var_uc_ndepm_dn14 / 1e-6);

        let assign7870_e3232: f64 = (p.p453 / 1e-6);
        locals.var_uc_njunc = assign7870_e3232;

        let assign7880_e3235: f64 = (p.p274 + 273.15);
        locals.var_ktnom = assign7880_e3235;

        let assign7930_e3258: f64 = (p.p0 + p.p116);
        locals.var_lgate = assign7930_e3258;

        let assign7940_e3261: f64 = (p.p1 / p.p7);
        let assign7940_e3263: f64 = (assign7940_e3261 + p.p117);
        locals.var_wgate = assign7940_e3263;

        let assign8090_e3363: f64 = (locals.var_lgate * 1000000.0);
        locals.var_lg = assign8090_e3363;

        let assign8100_e3366: f64 = (locals.var_wgate * 1000000.0);
        locals.var_wg = assign8100_e3366;

        let assign8110_e3369: f64 = (locals.var_lg).powf(p.p553);
        locals.var_lbin = assign8110_e3369;

        let assign8120_e3372: f64 = (locals.var_wg).powf(p.p554);
        locals.var_wbin = assign8120_e3372;

        let assign8130_e3375: f64 = (locals.var_lbin * locals.var_wbin);
        locals.var_lwbin = assign8130_e3375;

        let assign8140_e3379: f64 = (p.p555 / locals.var_lbin);
        let assign8140_e3380: f64 = (p.p89 + assign8140_e3379);
        let assign8140_e3383: f64 = (p.p643 / locals.var_wbin);
        let assign8140_e3384: f64 = (assign8140_e3380 + assign8140_e3383);
        let assign8140_e3387: f64 = (p.p731 / locals.var_lwbin);
        let assign8140_e3388: f64 = (assign8140_e3384 + assign8140_e3387);
        locals.var_uc_vmax = assign8140_e3388;

        let assign8150_e3392: f64 = (p.p556 / locals.var_lbin);
        let assign8150_e3393: f64 = (p.p92 + assign8150_e3392);
        let assign8150_e3396: f64 = (p.p644 / locals.var_wbin);
        let assign8150_e3397: f64 = (assign8150_e3393 + assign8150_e3396);
        let assign8150_e3400: f64 = (p.p732 / locals.var_lwbin);
        let assign8150_e3401: f64 = (assign8150_e3397 + assign8150_e3400);
        locals.var_uc_bgtmp1 = assign8150_e3401;

        let assign8160_e3405: f64 = (p.p557 / locals.var_lbin);
        let assign8160_e3406: f64 = (p.p93 + assign8160_e3405);
        let assign8160_e3409: f64 = (p.p645 / locals.var_wbin);
        let assign8160_e3410: f64 = (assign8160_e3406 + assign8160_e3409);
        let assign8160_e3413: f64 = (p.p733 / locals.var_lwbin);
        let assign8160_e3414: f64 = (assign8160_e3410 + assign8160_e3413);
        locals.var_uc_bgtmp2 = assign8160_e3414;

        let assign8170_e3418: f64 = (p.p558 / locals.var_lbin);
        let assign8170_e3419: f64 = (p.p94 + assign8170_e3418);
        let assign8170_e3422: f64 = (p.p646 / locals.var_wbin);
        let assign8170_e3423: f64 = (assign8170_e3419 + assign8170_e3422);
        let assign8170_e3426: f64 = (p.p734 / locals.var_lwbin);
        let assign8170_e3427: f64 = (assign8170_e3423 + assign8170_e3426);
        locals.var_uc_eg0 = assign8170_e3427;

        let assign8180_e3431: f64 = (p.p559 / locals.var_lbin);
        let assign8180_e3432: f64 = (p.p110 + assign8180_e3431);
        let assign8180_e3435: f64 = (p.p647 / locals.var_wbin);
        let assign8180_e3436: f64 = (assign8180_e3432 + assign8180_e3435);
        let assign8180_e3439: f64 = (p.p735 / locals.var_lwbin);
        let assign8180_e3440: f64 = (assign8180_e3436 + assign8180_e3439);
        locals.var_uc_vfbover = assign8180_e3440;

        let assign8190_e3444: f64 = (p.p560 / locals.var_lbin);
        let assign8190_e3445: f64 = (p.p111 + assign8190_e3444);
        let assign8190_e3448: f64 = (p.p648 / locals.var_wbin);
        let assign8190_e3449: f64 = (assign8190_e3445 + assign8190_e3448);
        let assign8190_e3452: f64 = (p.p736 / locals.var_lwbin);
        let assign8190_e3453: f64 = (assign8190_e3449 + assign8190_e3452);
        locals.var_uc_nover = assign8190_e3453;

        let assign8200_e3457: f64 = (p.p561 / locals.var_lbin);
        let assign8200_e3458: f64 = (p.p112 + assign8200_e3457);
        let assign8200_e3461: f64 = (p.p649 / locals.var_wbin);
        let assign8200_e3462: f64 = (assign8200_e3458 + assign8200_e3461);
        let assign8200_e3465: f64 = (p.p737 / locals.var_lwbin);
        let assign8200_e3466: f64 = (assign8200_e3462 + assign8200_e3465);
        locals.var_uc_novers = assign8200_e3466;

        let assign8210_e3470: f64 = (p.p562 / locals.var_lbin);
        let assign8210_e3471: f64 = (p.p126 + assign8210_e3470);
        let assign8210_e3474: f64 = (p.p650 / locals.var_wbin);
        let assign8210_e3475: f64 = (assign8210_e3471 + assign8210_e3474);
        let assign8210_e3478: f64 = (p.p738 / locals.var_lwbin);
        let assign8210_e3479: f64 = (assign8210_e3475 + assign8210_e3478);
        locals.var_uc_wl2 = assign8210_e3479;

        let assign8220_e3483: f64 = (p.p563 / locals.var_lbin);
        let assign8220_e3484: f64 = (p.p136 + assign8220_e3483);
        let assign8220_e3487: f64 = (p.p651 / locals.var_wbin);
        let assign8220_e3488: f64 = (assign8220_e3484 + assign8220_e3487);
        let assign8220_e3491: f64 = (p.p739 / locals.var_lwbin);
        let assign8220_e3492: f64 = (assign8220_e3488 + assign8220_e3491);
        locals.var_uc_vfbc = assign8220_e3492;

        let assign8230_e3496: f64 = (p.p564 / locals.var_lbin);
        let assign8230_e3497: f64 = (p.p138 + assign8230_e3496);
        let assign8230_e3500: f64 = (p.p652 / locals.var_wbin);
        let assign8230_e3501: f64 = (assign8230_e3497 + assign8230_e3500);
        let assign8230_e3504: f64 = (p.p740 / locals.var_lwbin);
        let assign8230_e3505: f64 = (assign8230_e3501 + assign8230_e3504);
        locals.var_uc_nsubc = assign8230_e3505;

        let assign8240_e3509: f64 = (p.p565 / locals.var_lbin);
        let assign8240_e3510: f64 = (p.p141 + assign8240_e3509);
        let assign8240_e3513: f64 = (p.p653 / locals.var_wbin);
        let assign8240_e3514: f64 = (assign8240_e3510 + assign8240_e3513);
        let assign8240_e3517: f64 = (p.p741 / locals.var_lwbin);
        let assign8240_e3518: f64 = (assign8240_e3514 + assign8240_e3517);
        locals.var_uc_nsubp = assign8240_e3518;

        let assign8250_e3522: f64 = (p.p566 / locals.var_lbin);
        let assign8250_e3523: f64 = (p.p144 + assign8250_e3522);
        let assign8250_e3526: f64 = (p.p654 / locals.var_wbin);
        let assign8250_e3527: f64 = (assign8250_e3523 + assign8250_e3526);
        let assign8250_e3530: f64 = (p.p742 / locals.var_lwbin);
        let assign8250_e3531: f64 = (assign8250_e3527 + assign8250_e3530);
        locals.var_uc_scp1 = assign8250_e3531;

        let assign8260_e3535: f64 = (p.p567 / locals.var_lbin);
        let assign8260_e3536: f64 = (p.p145 + assign8260_e3535);
        let assign8260_e3539: f64 = (p.p655 / locals.var_wbin);
        let assign8260_e3540: f64 = (assign8260_e3536 + assign8260_e3539);
        let assign8260_e3543: f64 = (p.p743 / locals.var_lwbin);
        let assign8260_e3544: f64 = (assign8260_e3540 + assign8260_e3543);
        locals.var_uc_scp2 = assign8260_e3544;

        let assign8270_e3548: f64 = (p.p568 / locals.var_lbin);
        let assign8270_e3549: f64 = (p.p146 + assign8270_e3548);
        let assign8270_e3552: f64 = (p.p656 / locals.var_wbin);
        let assign8270_e3553: f64 = (assign8270_e3549 + assign8270_e3552);
        let assign8270_e3556: f64 = (p.p744 / locals.var_lwbin);
        let assign8270_e3557: f64 = (assign8270_e3553 + assign8270_e3556);
        locals.var_uc_scp3 = assign8270_e3557;

        let assign8280_e3561: f64 = (p.p569 / locals.var_lbin);
        let assign8280_e3562: f64 = (p.p147 + assign8280_e3561);
        let assign8280_e3565: f64 = (p.p657 / locals.var_wbin);
        let assign8280_e3566: f64 = (assign8280_e3562 + assign8280_e3565);
        let assign8280_e3569: f64 = (p.p745 / locals.var_lwbin);
        let assign8280_e3570: f64 = (assign8280_e3566 + assign8280_e3569);
        locals.var_uc_sc1 = assign8280_e3570;

        let assign8290_e3574: f64 = (p.p570 / locals.var_lbin);
        let assign8290_e3575: f64 = (p.p148 + assign8290_e3574);
        let assign8290_e3578: f64 = (p.p658 / locals.var_wbin);
        let assign8290_e3579: f64 = (assign8290_e3575 + assign8290_e3578);
        let assign8290_e3582: f64 = (p.p746 / locals.var_lwbin);
        let assign8290_e3583: f64 = (assign8290_e3579 + assign8290_e3582);
        locals.var_uc_sc2 = assign8290_e3583;

        let assign8300_e3587: f64 = (p.p571 / locals.var_lbin);
        let assign8300_e3588: f64 = (p.p149 + assign8300_e3587);
        let assign8300_e3591: f64 = (p.p659 / locals.var_wbin);
        let assign8300_e3592: f64 = (assign8300_e3588 + assign8300_e3591);
        let assign8300_e3595: f64 = (p.p747 / locals.var_lwbin);
        let assign8300_e3596: f64 = (assign8300_e3592 + assign8300_e3595);
        locals.var_uc_sc3 = assign8300_e3596;

        let assign8310_e3600: f64 = (p.p572 / locals.var_lbin);
        let assign8310_e3601: f64 = (p.p151 + assign8310_e3600);
        let assign8310_e3604: f64 = (p.p660 / locals.var_wbin);
        let assign8310_e3605: f64 = (assign8310_e3601 + assign8310_e3604);
        let assign8310_e3608: f64 = (p.p748 / locals.var_lwbin);
        let assign8310_e3609: f64 = (assign8310_e3605 + assign8310_e3608);
        locals.var_uc_pgd1 = assign8310_e3609;

        let assign8320_e3613: f64 = (p.p573 / locals.var_lbin);
        let assign8320_e3614: f64 = (p.p154 + assign8320_e3613);
        let assign8320_e3617: f64 = (p.p661 / locals.var_wbin);
        let assign8320_e3618: f64 = (assign8320_e3614 + assign8320_e3617);
        let assign8320_e3621: f64 = (p.p749 / locals.var_lwbin);
        let assign8320_e3622: f64 = (assign8320_e3618 + assign8320_e3621);
        locals.var_uc_ndep = assign8320_e3622;

        let assign8330_e3626: f64 = (p.p574 / locals.var_lbin);
        let assign8330_e3627: f64 = (p.p157 + assign8330_e3626);
        let assign8330_e3630: f64 = (p.p662 / locals.var_wbin);
        let assign8330_e3631: f64 = (assign8330_e3627 + assign8330_e3630);
        let assign8330_e3634: f64 = (p.p750 / locals.var_lwbin);
        let assign8330_e3635: f64 = (assign8330_e3631 + assign8330_e3634);
        locals.var_uc_ninv = assign8330_e3635;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8340_e3639: f64 = (p.p575 / locals.var_lbin);
        let assign8340_e3640: f64 = (p.p158 + assign8340_e3639);
        let assign8340_e3643: f64 = (p.p663 / locals.var_wbin);
        let assign8340_e3644: f64 = (assign8340_e3640 + assign8340_e3643);
        let assign8340_e3647: f64 = (p.p751 / locals.var_lwbin);
        let assign8340_e3648: f64 = (assign8340_e3644 + assign8340_e3647);
        locals.var_uc_muecb0 = assign8340_e3648;

        let assign8350_e3652: f64 = (p.p576 / locals.var_lbin);
        let assign8350_e3653: f64 = (p.p159 + assign8350_e3652);
        let assign8350_e3656: f64 = (p.p664 / locals.var_wbin);
        let assign8350_e3657: f64 = (assign8350_e3653 + assign8350_e3656);
        let assign8350_e3660: f64 = (p.p752 / locals.var_lwbin);
        let assign8350_e3661: f64 = (assign8350_e3657 + assign8350_e3660);
        locals.var_uc_muecb1 = assign8350_e3661;

        let assign8360_e3665: f64 = (p.p577 / locals.var_lbin);
        let assign8360_e3666: f64 = (p.p161 + assign8360_e3665);
        let assign8360_e3669: f64 = (p.p665 / locals.var_wbin);
        let assign8360_e3670: f64 = (assign8360_e3666 + assign8360_e3669);
        let assign8360_e3673: f64 = (p.p753 / locals.var_lwbin);
        let assign8360_e3674: f64 = (assign8360_e3670 + assign8360_e3673);
        locals.var_uc_mueph1 = assign8360_e3674;

        let assign8370_e3678: f64 = (p.p578 / locals.var_lbin);
        let assign8370_e3679: f64 = (p.p169 + assign8370_e3678);
        let assign8370_e3682: f64 = (p.p666 / locals.var_wbin);
        let assign8370_e3683: f64 = (assign8370_e3679 + assign8370_e3682);
        let assign8370_e3686: f64 = (p.p754 / locals.var_lwbin);
        let assign8370_e3687: f64 = (assign8370_e3683 + assign8370_e3686);
        locals.var_uc_vtmp = assign8370_e3687;

        let assign8380_e3691: f64 = (p.p579 / locals.var_lbin);
        let assign8380_e3692: f64 = (p.p170 + assign8380_e3691);
        let assign8380_e3695: f64 = (p.p667 / locals.var_wbin);
        let assign8380_e3696: f64 = (assign8380_e3692 + assign8380_e3695);
        let assign8380_e3699: f64 = (p.p755 / locals.var_lwbin);
        let assign8380_e3700: f64 = (assign8380_e3696 + assign8380_e3699);
        locals.var_uc_wvth0 = assign8380_e3700;

        let assign8390_e3704: f64 = (p.p580 / locals.var_lbin);
        let assign8390_e3705: f64 = (p.p172 + assign8390_e3704);
        let assign8390_e3708: f64 = (p.p668 / locals.var_wbin);
        let assign8390_e3709: f64 = (assign8390_e3705 + assign8390_e3708);
        let assign8390_e3712: f64 = (p.p756 / locals.var_lwbin);
        let assign8390_e3713: f64 = (assign8390_e3709 + assign8390_e3712);
        locals.var_uc_muesr1 = assign8390_e3713;

        let assign8400_e3717: f64 = (p.p581 / locals.var_lbin);
        let assign8400_e3718: f64 = (p.p177 + assign8400_e3717);
        let assign8400_e3721: f64 = (p.p669 / locals.var_wbin);
        let assign8400_e3722: f64 = (assign8400_e3718 + assign8400_e3721);
        let assign8400_e3725: f64 = (p.p757 / locals.var_lwbin);
        let assign8400_e3726: f64 = (assign8400_e3722 + assign8400_e3725);
        locals.var_uc_muetmp = assign8400_e3726;

        let assign8410_e3730: f64 = (p.p582 / locals.var_lbin);
        let assign8410_e3731: f64 = (p.p179 + assign8410_e3730);
        let assign8410_e3734: f64 = (p.p670 / locals.var_wbin);
        let assign8410_e3735: f64 = (assign8410_e3731 + assign8410_e3734);
        let assign8410_e3738: f64 = (p.p758 / locals.var_lwbin);
        let assign8410_e3739: f64 = (assign8410_e3735 + assign8410_e3738);
        locals.var_uc_sub1 = assign8410_e3739;

        let assign8420_e3743: f64 = (p.p583 / locals.var_lbin);
        let assign8420_e3744: f64 = (p.p180 + assign8420_e3743);
        let assign8420_e3747: f64 = (p.p671 / locals.var_wbin);
        let assign8420_e3748: f64 = (assign8420_e3744 + assign8420_e3747);
        let assign8420_e3751: f64 = (p.p759 / locals.var_lwbin);
        let assign8420_e3752: f64 = (assign8420_e3748 + assign8420_e3751);
        locals.var_uc_sub2 = assign8420_e3752;

        let assign8430_e3756: f64 = (p.p584 / locals.var_lbin);
        let assign8430_e3757: f64 = (p.p185 + assign8430_e3756);
        let assign8430_e3760: f64 = (p.p672 / locals.var_wbin);
        let assign8430_e3761: f64 = (assign8430_e3757 + assign8430_e3760);
        let assign8430_e3764: f64 = (p.p760 / locals.var_lwbin);
        let assign8430_e3765: f64 = (assign8430_e3761 + assign8430_e3764);
        locals.var_uc_svds = assign8430_e3765;

        let assign8440_e3769: f64 = (p.p585 / locals.var_lbin);
        let assign8440_e3770: f64 = (p.p182 + assign8440_e3769);
        let assign8440_e3773: f64 = (p.p673 / locals.var_wbin);
        let assign8440_e3774: f64 = (assign8440_e3770 + assign8440_e3773);
        let assign8440_e3777: f64 = (p.p761 / locals.var_lwbin);
        let assign8440_e3778: f64 = (assign8440_e3774 + assign8440_e3777);
        locals.var_uc_svbs = assign8440_e3778;

        let assign8450_e3782: f64 = (p.p586 / locals.var_lbin);
        let assign8450_e3783: f64 = (p.p181 + assign8450_e3782);
        let assign8450_e3786: f64 = (p.p674 / locals.var_wbin);
        let assign8450_e3787: f64 = (assign8450_e3783 + assign8450_e3786);
        let assign8450_e3790: f64 = (p.p762 / locals.var_lwbin);
        let assign8450_e3791: f64 = (assign8450_e3787 + assign8450_e3790);
        locals.var_uc_svgs = assign8450_e3791;

        let assign8460_e3795: f64 = (p.p587 / locals.var_lbin);
        let assign8460_e3796: f64 = (p.p187 + assign8460_e3795);
        let assign8460_e3799: f64 = (p.p675 / locals.var_wbin);
        let assign8460_e3800: f64 = (assign8460_e3796 + assign8460_e3799);
        let assign8460_e3803: f64 = (p.p763 / locals.var_lwbin);
        let assign8460_e3804: f64 = (assign8460_e3800 + assign8460_e3803);
        locals.var_uc_sub1snp = assign8460_e3804;

        let assign8470_e3808: f64 = (p.p588 / locals.var_lbin);
        let assign8470_e3809: f64 = (p.p188 + assign8470_e3808);
        let assign8470_e3812: f64 = (p.p676 / locals.var_wbin);
        let assign8470_e3813: f64 = (assign8470_e3809 + assign8470_e3812);
        let assign8470_e3816: f64 = (p.p764 / locals.var_lwbin);
        let assign8470_e3817: f64 = (assign8470_e3813 + assign8470_e3816);
        locals.var_uc_sub2snp = assign8470_e3817;

        let assign8480_e3821: f64 = (p.p589 / locals.var_lbin);
        let assign8480_e3822: f64 = (p.p189 + assign8480_e3821);
        let assign8480_e3825: f64 = (p.p677 / locals.var_wbin);
        let assign8480_e3826: f64 = (assign8480_e3822 + assign8480_e3825);
        let assign8480_e3829: f64 = (p.p765 / locals.var_lwbin);
        let assign8480_e3830: f64 = (assign8480_e3826 + assign8480_e3829);
        locals.var_uc_svdssnp = assign8480_e3830;

        let assign8490_e3834: f64 = (p.p590 / locals.var_lbin);
        let assign8490_e3835: f64 = (p.p194 + assign8490_e3834);
        let assign8490_e3838: f64 = (p.p678 / locals.var_wbin);
        let assign8490_e3839: f64 = (assign8490_e3835 + assign8490_e3838);
        let assign8490_e3842: f64 = (p.p766 / locals.var_lwbin);
        let assign8490_e3843: f64 = (assign8490_e3839 + assign8490_e3842);
        locals.var_uc_fn1 = assign8490_e3843;

        let assign8500_e3847: f64 = (p.p591 / locals.var_lbin);
        let assign8500_e3848: f64 = (p.p195 + assign8500_e3847);
        let assign8500_e3851: f64 = (p.p679 / locals.var_wbin);
        let assign8500_e3852: f64 = (assign8500_e3848 + assign8500_e3851);
        let assign8500_e3855: f64 = (p.p767 / locals.var_lwbin);
        let assign8500_e3856: f64 = (assign8500_e3852 + assign8500_e3855);
        locals.var_uc_fn2 = assign8500_e3856;

        let assign8510_e3860: f64 = (p.p592 / locals.var_lbin);
        let assign8510_e3861: f64 = (p.p196 + assign8510_e3860);
        let assign8510_e3864: f64 = (p.p680 / locals.var_wbin);
        let assign8510_e3865: f64 = (assign8510_e3861 + assign8510_e3864);
        let assign8510_e3868: f64 = (p.p768 / locals.var_lwbin);
        let assign8510_e3869: f64 = (assign8510_e3865 + assign8510_e3868);
        locals.var_uc_fn3 = assign8510_e3869;

        let assign8520_e3873: f64 = (p.p593 / locals.var_lbin);
        let assign8520_e3874: f64 = (p.p197 + assign8520_e3873);
        let assign8520_e3877: f64 = (p.p681 / locals.var_wbin);
        let assign8520_e3878: f64 = (assign8520_e3874 + assign8520_e3877);
        let assign8520_e3881: f64 = (p.p769 / locals.var_lwbin);
        let assign8520_e3882: f64 = (assign8520_e3878 + assign8520_e3881);
        locals.var_uc_fvbs = assign8520_e3882;

        let assign8530_e3886: f64 = (p.p594 / locals.var_lbin);
        let assign8530_e3887: f64 = (p.p204 + assign8530_e3886);
        let assign8530_e3890: f64 = (p.p682 / locals.var_wbin);
        let assign8530_e3891: f64 = (assign8530_e3887 + assign8530_e3890);
        let assign8530_e3894: f64 = (p.p770 / locals.var_lwbin);
        let assign8530_e3895: f64 = (assign8530_e3891 + assign8530_e3894);
        locals.var_uc_nsti = assign8530_e3895;

        let assign8540_e3899: f64 = (p.p595 / locals.var_lbin);
        let assign8540_e3900: f64 = (p.p205 + assign8540_e3899);
        let assign8540_e3903: f64 = (p.p683 / locals.var_wbin);
        let assign8540_e3904: f64 = (assign8540_e3900 + assign8540_e3903);
        let assign8540_e3907: f64 = (p.p771 / locals.var_lwbin);
        let assign8540_e3908: f64 = (assign8540_e3904 + assign8540_e3907);
        locals.var_uc_wsti = assign8540_e3908;
        locals.var_uc_wsti_dn0 = 0.0;
        locals.var_uc_wsti_dn2 = 0.0;
        locals.var_uc_wsti_dn4 = 0.0;
        locals.var_uc_wsti_dn5 = 0.0;
        locals.var_uc_wsti_dn6 = 0.0;
        locals.var_uc_wsti_dn7 = 0.0;
        locals.var_uc_wsti_dn8 = 0.0;
        locals.var_uc_wsti_dn9 = 0.0;
        locals.var_uc_wsti_dn10 = 0.0;
        locals.var_uc_wsti_dn11 = 0.0;
        locals.var_uc_wsti_dn14 = 0.0;

        let assign8550_e3912: f64 = (p.p596 / locals.var_lbin);
        let assign8550_e3913: f64 = (p.p210 + assign8550_e3912);
        let assign8550_e3916: f64 = (p.p684 / locals.var_wbin);
        let assign8550_e3917: f64 = (assign8550_e3913 + assign8550_e3916);
        let assign8550_e3920: f64 = (p.p772 / locals.var_lwbin);
        let assign8550_e3921: f64 = (assign8550_e3917 + assign8550_e3920);
        locals.var_uc_scsti1 = assign8550_e3921;

        let assign8560_e3925: f64 = (p.p597 / locals.var_lbin);
        let assign8560_e3926: f64 = (p.p211 + assign8560_e3925);
        let assign8560_e3929: f64 = (p.p685 / locals.var_wbin);
        let assign8560_e3930: f64 = (assign8560_e3926 + assign8560_e3929);
        let assign8560_e3933: f64 = (p.p773 / locals.var_lwbin);
        let assign8560_e3934: f64 = (assign8560_e3930 + assign8560_e3933);
        locals.var_uc_scsti2 = assign8560_e3934;

        let assign8570_e3938: f64 = (p.p598 / locals.var_lbin);
        let assign8570_e3939: f64 = (p.p212 + assign8570_e3938);
        let assign8570_e3942: f64 = (p.p686 / locals.var_wbin);
        let assign8570_e3943: f64 = (assign8570_e3939 + assign8570_e3942);
        let assign8570_e3946: f64 = (p.p774 / locals.var_lwbin);
        let assign8570_e3947: f64 = (assign8570_e3943 + assign8570_e3946);
        locals.var_uc_vthsti = assign8570_e3947;

        let assign8580_e3951: f64 = (p.p599 / locals.var_lbin);
        let assign8580_e3952: f64 = (p.p214 + assign8580_e3951);
        let assign8580_e3955: f64 = (p.p687 / locals.var_wbin);
        let assign8580_e3956: f64 = (assign8580_e3952 + assign8580_e3955);
        let assign8580_e3959: f64 = (p.p775 / locals.var_lwbin);
        let assign8580_e3960: f64 = (assign8580_e3956 + assign8580_e3959);
        locals.var_uc_muesti1 = assign8580_e3960;

        let assign8590_e3964: f64 = (p.p600 / locals.var_lbin);
        let assign8590_e3965: f64 = (p.p215 + assign8590_e3964);
        let assign8590_e3968: f64 = (p.p688 / locals.var_wbin);
        let assign8590_e3969: f64 = (assign8590_e3965 + assign8590_e3968);
        let assign8590_e3972: f64 = (p.p776 / locals.var_lwbin);
        let assign8590_e3973: f64 = (assign8590_e3969 + assign8590_e3972);
        locals.var_uc_muesti2 = assign8590_e3973;

        let assign8600_e3977: f64 = (p.p601 / locals.var_lbin);
        let assign8600_e3978: f64 = (p.p216 + assign8600_e3977);
        let assign8600_e3981: f64 = (p.p689 / locals.var_wbin);
        let assign8600_e3982: f64 = (assign8600_e3978 + assign8600_e3981);
        let assign8600_e3985: f64 = (p.p777 / locals.var_lwbin);
        let assign8600_e3986: f64 = (assign8600_e3982 + assign8600_e3985);
        locals.var_uc_muesti3 = assign8600_e3986;

        let assign8610_e3990: f64 = (p.p602 / locals.var_lbin);
        let assign8610_e3991: f64 = (p.p217 + assign8610_e3990);
        let assign8610_e3994: f64 = (p.p690 / locals.var_wbin);
        let assign8610_e3995: f64 = (assign8610_e3991 + assign8610_e3994);
        let assign8610_e3998: f64 = (p.p778 / locals.var_lwbin);
        let assign8610_e3999: f64 = (assign8610_e3995 + assign8610_e3998);
        locals.var_uc_nsubpsti1 = assign8610_e3999;

        let assign8620_e4003: f64 = (p.p603 / locals.var_lbin);
        let assign8620_e4004: f64 = (p.p218 + assign8620_e4003);
        let assign8620_e4007: f64 = (p.p691 / locals.var_wbin);
        let assign8620_e4008: f64 = (assign8620_e4004 + assign8620_e4007);
        let assign8620_e4011: f64 = (p.p779 / locals.var_lwbin);
        let assign8620_e4012: f64 = (assign8620_e4008 + assign8620_e4011);
        locals.var_uc_nsubpsti2 = assign8620_e4012;

        let assign8630_e4016: f64 = (p.p604 / locals.var_lbin);
        let assign8630_e4017: f64 = (p.p219 + assign8630_e4016);
        let assign8630_e4020: f64 = (p.p692 / locals.var_wbin);
        let assign8630_e4021: f64 = (assign8630_e4017 + assign8630_e4020);
        let assign8630_e4024: f64 = (p.p780 / locals.var_lwbin);
        let assign8630_e4025: f64 = (assign8630_e4021 + assign8630_e4024);
        locals.var_uc_nsubpsti3 = assign8630_e4025;

        let assign8640_e4029: f64 = (p.p605 / locals.var_lbin);
        let assign8640_e4030: f64 = (p.p269 + assign8640_e4029);
        let assign8640_e4033: f64 = (p.p693 / locals.var_wbin);
        let assign8640_e4034: f64 = (assign8640_e4030 + assign8640_e4033);
        let assign8640_e4037: f64 = (p.p781 / locals.var_lwbin);
        let assign8640_e4038: f64 = (assign8640_e4034 + assign8640_e4037);
        locals.var_uc_cgso = assign8640_e4038;

        let assign8650_e4042: f64 = (p.p606 / locals.var_lbin);
        let assign8650_e4043: f64 = (p.p268 + assign8650_e4042);
        let assign8650_e4046: f64 = (p.p694 / locals.var_wbin);
        let assign8650_e4047: f64 = (assign8650_e4043 + assign8650_e4046);
        let assign8650_e4050: f64 = (p.p782 / locals.var_lwbin);
        let assign8650_e4051: f64 = (assign8650_e4047 + assign8650_e4050);
        locals.var_uc_cgdo = assign8650_e4051;

        let assign8660_e4055: f64 = (p.p607 / locals.var_lbin);
        let assign8660_e4056: f64 = (p.p226 + assign8660_e4055);
        let assign8660_e4059: f64 = (p.p695 / locals.var_wbin);
        let assign8660_e4060: f64 = (assign8660_e4056 + assign8660_e4059);
        let assign8660_e4063: f64 = (p.p783 / locals.var_lwbin);
        let assign8660_e4064: f64 = (assign8660_e4060 + assign8660_e4063);
        locals.var_uc_clm1 = assign8660_e4064;

        let assign8670_e4068: f64 = (p.p608 / locals.var_lbin);
        let assign8670_e4069: f64 = (p.p227 + assign8670_e4068);
        let assign8670_e4072: f64 = (p.p696 / locals.var_wbin);
        let assign8670_e4073: f64 = (assign8670_e4069 + assign8670_e4072);
        let assign8670_e4076: f64 = (p.p784 / locals.var_lwbin);
        let assign8670_e4077: f64 = (assign8670_e4073 + assign8670_e4076);
        locals.var_uc_clm2 = assign8670_e4077;
        locals.var_uc_clm2_dn0 = 0.0;
        locals.var_uc_clm2_dn2 = 0.0;
        locals.var_uc_clm2_dn4 = 0.0;
        locals.var_uc_clm2_dn5 = 0.0;
        locals.var_uc_clm2_dn6 = 0.0;
        locals.var_uc_clm2_dn7 = 0.0;
        locals.var_uc_clm2_dn8 = 0.0;
        locals.var_uc_clm2_dn9 = 0.0;
        locals.var_uc_clm2_dn10 = 0.0;
        locals.var_uc_clm2_dn11 = 0.0;
        locals.var_uc_clm2_dn14 = 0.0;

        let assign8680_e4081: f64 = (p.p609 / locals.var_lbin);
        let assign8680_e4082: f64 = (p.p228 + assign8680_e4081);
        let assign8680_e4085: f64 = (p.p697 / locals.var_wbin);
        let assign8680_e4086: f64 = (assign8680_e4082 + assign8680_e4085);
        let assign8680_e4089: f64 = (p.p785 / locals.var_lwbin);
        let assign8680_e4090: f64 = (assign8680_e4086 + assign8680_e4089);
        locals.var_uc_clm3 = assign8680_e4090;

        let assign8690_e4094: f64 = (p.p610 / locals.var_lbin);
        let assign8690_e4095: f64 = (p.p232 + assign8690_e4094);
        let assign8690_e4098: f64 = (p.p698 / locals.var_wbin);
        let assign8690_e4099: f64 = (assign8690_e4095 + assign8690_e4098);
        let assign8690_e4102: f64 = (p.p786 / locals.var_lwbin);
        let assign8690_e4103: f64 = (assign8690_e4099 + assign8690_e4102);
        locals.var_uc_wfc = assign8690_e4103;

        let assign8700_e4107: f64 = (p.p611 / locals.var_lbin);
        let assign8700_e4108: f64 = (p.p240 + assign8700_e4107);
        let assign8700_e4111: f64 = (p.p699 / locals.var_wbin);
        let assign8700_e4112: f64 = (assign8700_e4108 + assign8700_e4111);
        let assign8700_e4115: f64 = (p.p787 / locals.var_lwbin);
        let assign8700_e4116: f64 = (assign8700_e4112 + assign8700_e4115);
        locals.var_uc_gidl1 = assign8700_e4116;

        let assign8710_e4120: f64 = (p.p612 / locals.var_lbin);
        let assign8710_e4121: f64 = (p.p241 + assign8710_e4120);
        let assign8710_e4124: f64 = (p.p700 / locals.var_wbin);
        let assign8710_e4125: f64 = (assign8710_e4121 + assign8710_e4124);
        let assign8710_e4128: f64 = (p.p788 / locals.var_lwbin);
        let assign8710_e4129: f64 = (assign8710_e4125 + assign8710_e4128);
        locals.var_uc_gidl2 = assign8710_e4129;

        let assign8720_e4133: f64 = (p.p613 / locals.var_lbin);
        let assign8720_e4134: f64 = (p.p245 + assign8720_e4133);
        let assign8720_e4137: f64 = (p.p701 / locals.var_wbin);
        let assign8720_e4138: f64 = (assign8720_e4134 + assign8720_e4137);
        let assign8720_e4141: f64 = (p.p789 / locals.var_lwbin);
        let assign8720_e4142: f64 = (assign8720_e4138 + assign8720_e4141);
        locals.var_uc_gleak1 = assign8720_e4142;

        let assign8730_e4146: f64 = (p.p614 / locals.var_lbin);
        let assign8730_e4147: f64 = (p.p246 + assign8730_e4146);
        let assign8730_e4150: f64 = (p.p702 / locals.var_wbin);
        let assign8730_e4151: f64 = (assign8730_e4147 + assign8730_e4150);
        let assign8730_e4154: f64 = (p.p790 / locals.var_lwbin);
        let assign8730_e4155: f64 = (assign8730_e4151 + assign8730_e4154);
        locals.var_uc_gleak2 = assign8730_e4155;

        let assign8740_e4159: f64 = (p.p615 / locals.var_lbin);
        let assign8740_e4160: f64 = (p.p247 + assign8740_e4159);
        let assign8740_e4163: f64 = (p.p703 / locals.var_wbin);
        let assign8740_e4164: f64 = (assign8740_e4160 + assign8740_e4163);
        let assign8740_e4167: f64 = (p.p791 / locals.var_lwbin);
        let assign8740_e4168: f64 = (assign8740_e4164 + assign8740_e4167);
        locals.var_uc_gleak3 = assign8740_e4168;

        let assign8750_e4172: f64 = (p.p616 / locals.var_lbin);
        let assign8750_e4173: f64 = (p.p250 + assign8750_e4172);
        let assign8750_e4176: f64 = (p.p704 / locals.var_wbin);
        let assign8750_e4177: f64 = (assign8750_e4173 + assign8750_e4176);
        let assign8750_e4180: f64 = (p.p792 / locals.var_lwbin);
        let assign8750_e4181: f64 = (assign8750_e4177 + assign8750_e4180);
        locals.var_uc_gleak6 = assign8750_e4181;

        let assign8760_e4185: f64 = (p.p617 / locals.var_lbin);
        let assign8760_e4186: f64 = (p.p253 + assign8760_e4185);
        let assign8760_e4189: f64 = (p.p705 / locals.var_wbin);
        let assign8760_e4190: f64 = (assign8760_e4186 + assign8760_e4189);
        let assign8760_e4193: f64 = (p.p793 / locals.var_lwbin);
        let assign8760_e4194: f64 = (assign8760_e4190 + assign8760_e4193);
        locals.var_uc_glksd1 = assign8760_e4194;

        let assign8770_e4198: f64 = (p.p618 / locals.var_lbin);
        let assign8770_e4199: f64 = (p.p254 + assign8770_e4198);
        let assign8770_e4202: f64 = (p.p706 / locals.var_wbin);
        let assign8770_e4203: f64 = (assign8770_e4199 + assign8770_e4202);
        let assign8770_e4206: f64 = (p.p794 / locals.var_lwbin);
        let assign8770_e4207: f64 = (assign8770_e4203 + assign8770_e4206);
        locals.var_uc_glksd2 = assign8770_e4207;

        let assign8780_e4211: f64 = (p.p619 / locals.var_lbin);
        let assign8780_e4212: f64 = (p.p256 + assign8780_e4211);
        let assign8780_e4215: f64 = (p.p707 / locals.var_wbin);
        let assign8780_e4216: f64 = (assign8780_e4212 + assign8780_e4215);
        let assign8780_e4219: f64 = (p.p795 / locals.var_lwbin);
        let assign8780_e4220: f64 = (assign8780_e4216 + assign8780_e4219);
        locals.var_uc_glkb1 = assign8780_e4220;

        let assign8790_e4224: f64 = (p.p620 / locals.var_lbin);
        let assign8790_e4225: f64 = (p.p257 + assign8790_e4224);
        let assign8790_e4228: f64 = (p.p708 / locals.var_wbin);
        let assign8790_e4229: f64 = (assign8790_e4225 + assign8790_e4228);
        let assign8790_e4232: f64 = (p.p796 / locals.var_lwbin);
        let assign8790_e4233: f64 = (assign8790_e4229 + assign8790_e4232);
        locals.var_uc_glkb2 = assign8790_e4233;

        let assign8810_e4250: f64 = (p.p622 / locals.var_lbin);
        let assign8810_e4251: f64 = (p.p265 + assign8810_e4250);
        let assign8810_e4254: f64 = (p.p710 / locals.var_wbin);
        let assign8810_e4255: f64 = (assign8810_e4251 + assign8810_e4254);
        let assign8810_e4258: f64 = (p.p798 / locals.var_lwbin);
        let assign8810_e4259: f64 = (assign8810_e4255 + assign8810_e4258);
        locals.var_uc_nfalp = assign8810_e4259;

        let assign8820_e4263: f64 = (p.p623 / locals.var_lbin);
        let assign8820_e4264: f64 = (p.p278 + assign8820_e4263);
        let assign8820_e4267: f64 = (p.p711 / locals.var_wbin);
        let assign8820_e4268: f64 = (assign8820_e4264 + assign8820_e4267);
        let assign8820_e4271: f64 = (p.p799 / locals.var_lwbin);
        let assign8820_e4272: f64 = (assign8820_e4268 + assign8820_e4271);
        locals.var_uc_ibpc1 = assign8820_e4272;

        let assign8830_e4276: f64 = (p.p624 / locals.var_lbin);
        let assign8830_e4277: f64 = (p.p281 + assign8830_e4276);
        let assign8830_e4280: f64 = (p.p712 / locals.var_wbin);
        let assign8830_e4281: f64 = (assign8830_e4277 + assign8830_e4280);
        let assign8830_e4284: f64 = (p.p800 / locals.var_lwbin);
        let assign8830_e4285: f64 = (assign8830_e4281 + assign8830_e4284);
        locals.var_uc_ibpc2 = assign8830_e4285;

        let assign8840_e4289: f64 = (p.p625 / locals.var_lbin);
        let assign8840_e4290: f64 = (p.p79 + assign8840_e4289);
        let assign8840_e4293: f64 = (p.p713 / locals.var_wbin);
        let assign8840_e4294: f64 = (assign8840_e4290 + assign8840_e4293);
        let assign8840_e4297: f64 = (p.p801 / locals.var_lwbin);
        let assign8840_e4298: f64 = (assign8840_e4294 + assign8840_e4297);
        locals.var_uc_cgbo = assign8840_e4298;

        let assign8850_e4302: f64 = (p.p626 / locals.var_lbin);
        let assign8850_e4303: f64 = (p.p86 + assign8850_e4302);
        let assign8850_e4306: f64 = (p.p714 / locals.var_wbin);
        let assign8850_e4307: f64 = (assign8850_e4303 + assign8850_e4306);
        let assign8850_e4310: f64 = (p.p802 / locals.var_lwbin);
        let assign8850_e4311: f64 = (assign8850_e4307 + assign8850_e4310);
        locals.var_uc_cvdsover = assign8850_e4311;

        let assign8870_e4328: f64 = (p.p628 / locals.var_lbin);
        let assign8870_e4329: f64 = (p.p76 + assign8870_e4328);
        let assign8870_e4332: f64 = (p.p716 / locals.var_wbin);
        let assign8870_e4333: f64 = (assign8870_e4329 + assign8870_e4332);
        let assign8870_e4336: f64 = (p.p804 / locals.var_lwbin);
        let assign8870_e4337: f64 = (assign8870_e4333 + assign8870_e4336);
        locals.var_uc_npext = assign8870_e4337;

        let assign8880_e4341: f64 = (p.p629 / locals.var_lbin);
        let assign8880_e4342: f64 = (p.p81 + assign8880_e4341);
        let assign8880_e4345: f64 = (p.p717 / locals.var_wbin);
        let assign8880_e4346: f64 = (assign8880_e4342 + assign8880_e4345);
        let assign8880_e4349: f64 = (p.p805 / locals.var_lwbin);
        let assign8880_e4350: f64 = (assign8880_e4346 + assign8880_e4349);
        locals.var_uc_powrat = assign8880_e4350;

        let assign8890_e4354: f64 = (p.p630 / locals.var_lbin);
        let assign8890_e4355: f64 = (p.p74 + assign8890_e4354);
        let assign8890_e4358: f64 = (p.p718 / locals.var_wbin);
        let assign8890_e4359: f64 = (assign8890_e4355 + assign8890_e4358);
        let assign8890_e4362: f64 = (p.p806 / locals.var_lwbin);
        let assign8890_e4363: f64 = (assign8890_e4359 + assign8890_e4362);
        locals.var_uc_rd = assign8890_e4363;

        let assign8900_e4367: f64 = (p.p631 / locals.var_lbin);
        let assign8900_e4368: f64 = (p.p298 + assign8900_e4367);
        let assign8900_e4371: f64 = (p.p719 / locals.var_wbin);
        let assign8900_e4372: f64 = (assign8900_e4368 + assign8900_e4371);
        let assign8900_e4375: f64 = (p.p807 / locals.var_lwbin);
        let assign8900_e4376: f64 = (assign8900_e4372 + assign8900_e4375);
        locals.var_uc_rd22 = assign8900_e4376;

        let assign8910_e4380: f64 = (p.p632 / locals.var_lbin);
        let assign8910_e4381: f64 = (p.p83 + assign8910_e4380);
        let assign8910_e4384: f64 = (p.p720 / locals.var_wbin);
        let assign8910_e4385: f64 = (assign8910_e4381 + assign8910_e4384);
        let assign8910_e4388: f64 = (p.p808 / locals.var_lwbin);
        let assign8910_e4389: f64 = (assign8910_e4385 + assign8910_e4388);
        locals.var_uc_rd23 = assign8910_e4389;

        let assign8920_e4393: f64 = (p.p633 / locals.var_lbin);
        let assign8920_e4394: f64 = (p.p84 + assign8920_e4393);
        let assign8920_e4397: f64 = (p.p721 / locals.var_wbin);
        let assign8920_e4398: f64 = (assign8920_e4394 + assign8920_e4397);
        let assign8920_e4401: f64 = (p.p809 / locals.var_lwbin);
        let assign8920_e4402: f64 = (assign8920_e4398 + assign8920_e4401);
        locals.var_uc_rd24 = assign8920_e4402;

        let assign8930_e4406: f64 = (p.p634 / locals.var_lbin);
        let assign8930_e4407: f64 = (p.p62 + assign8930_e4406);
        let assign8930_e4410: f64 = (p.p722 / locals.var_wbin);
        let assign8930_e4411: f64 = (assign8930_e4407 + assign8930_e4410);
        let assign8930_e4414: f64 = (p.p810 / locals.var_lwbin);
        let assign8930_e4415: f64 = (assign8930_e4411 + assign8930_e4414);
        locals.var_uc_rdict1 = assign8930_e4415;

        let assign8940_e4419: f64 = (p.p635 / locals.var_lbin);
        let assign8940_e4420: f64 = (p.p59 + assign8940_e4419);
        let assign8940_e4423: f64 = (p.p723 / locals.var_wbin);
        let assign8940_e4424: f64 = (assign8940_e4420 + assign8940_e4423);
        let assign8940_e4427: f64 = (p.p811 / locals.var_lwbin);
        let assign8940_e4428: f64 = (assign8940_e4424 + assign8940_e4427);
        locals.var_uc_rdov13 = assign8940_e4428;

        let assign8950_e4432: f64 = (p.p636 / locals.var_lbin);
        let assign8950_e4433: f64 = (p.p60 + assign8950_e4432);
        let assign8950_e4436: f64 = (p.p724 / locals.var_wbin);
        let assign8950_e4437: f64 = (assign8950_e4433 + assign8950_e4436);
        let assign8950_e4440: f64 = (p.p812 / locals.var_lwbin);
        let assign8950_e4441: f64 = (assign8950_e4437 + assign8950_e4440);
        locals.var_uc_rdslp1 = assign8950_e4441;

        let assign8960_e4445: f64 = (p.p637 / locals.var_lbin);
        let assign8960_e4446: f64 = (p.p85 + assign8960_e4445);
        let assign8960_e4449: f64 = (p.p725 / locals.var_wbin);
        let assign8960_e4450: f64 = (assign8960_e4446 + assign8960_e4449);
        let assign8960_e4453: f64 = (p.p813 / locals.var_lwbin);
        let assign8960_e4454: f64 = (assign8960_e4450 + assign8960_e4453);
        locals.var_uc_rdvb = assign8960_e4454;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8970_e4458: f64 = (p.p638 / locals.var_lbin);
        let assign8970_e4459: f64 = (p.p82 + assign8970_e4458);
        let assign8970_e4462: f64 = (p.p726 / locals.var_wbin);
        let assign8970_e4463: f64 = (assign8970_e4459 + assign8970_e4462);
        let assign8970_e4466: f64 = (p.p814 / locals.var_lwbin);
        let assign8970_e4467: f64 = (assign8970_e4463 + assign8970_e4466);
        locals.var_uc_rdvd = assign8970_e4467;

        let assign8980_e4471: f64 = (p.p639 / locals.var_lbin);
        let assign8980_e4472: f64 = (p.p61 + assign8980_e4471);
        let assign8980_e4475: f64 = (p.p727 / locals.var_wbin);
        let assign8980_e4476: f64 = (assign8980_e4472 + assign8980_e4475);
        let assign8980_e4479: f64 = (p.p815 / locals.var_lwbin);
        let assign8980_e4480: f64 = (assign8980_e4476 + assign8980_e4479);
        locals.var_uc_rdvg11 = assign8980_e4480;

        let assign8990_e4484: f64 = (p.p640 / locals.var_lbin);
        let assign8990_e4485: f64 = (p.p75 + assign8990_e4484);
        let assign8990_e4488: f64 = (p.p728 / locals.var_wbin);
        let assign8990_e4489: f64 = (assign8990_e4485 + assign8990_e4488);
        let assign8990_e4492: f64 = (p.p816 / locals.var_lwbin);
        let assign8990_e4493: f64 = (assign8990_e4489 + assign8990_e4492);
        locals.var_uc_rs = assign8990_e4493;

        let assign9000_e4497: f64 = (p.p641 / locals.var_lbin);
        let assign9000_e4498: f64 = (p.p80 + assign9000_e4497);
        let assign9000_e4501: f64 = (p.p729 / locals.var_wbin);
        let assign9000_e4502: f64 = (assign9000_e4498 + assign9000_e4501);
        let assign9000_e4505: f64 = (p.p817 / locals.var_lwbin);
        let assign9000_e4506: f64 = (assign9000_e4502 + assign9000_e4505);
        locals.var_uc_rth0 = assign9000_e4506;

        let assign9010_e4510: f64 = (p.p642 / locals.var_lbin);
        let assign9010_e4511: f64 = (p.p77 + assign9010_e4510);
        let assign9010_e4514: f64 = (p.p730 / locals.var_wbin);
        let assign9010_e4515: f64 = (assign9010_e4511 + assign9010_e4514);
        let assign9010_e4518: f64 = (p.p818 / locals.var_lwbin);
        let assign9010_e4519: f64 = (assign9010_e4515 + assign9010_e4518);
        locals.var_uc_vover = assign9010_e4519;

        let assign9020_e4523: f64 = (p.p824 / locals.var_lbin);
        let assign9020_e4524: f64 = (p.p493 + assign9020_e4523);
        let assign9020_e4527: f64 = (p.p839 / locals.var_wbin);
        let assign9020_e4528: f64 = (assign9020_e4524 + assign9020_e4527);
        let assign9020_e4531: f64 = (p.p854 / locals.var_lwbin);
        let assign9020_e4532: f64 = (assign9020_e4528 + assign9020_e4531);
        locals.var_uc_js0d = assign9020_e4532;

        let assign9030_e4536: f64 = (p.p825 / locals.var_lbin);
        let assign9030_e4537: f64 = (p.p494 + assign9030_e4536);
        let assign9030_e4540: f64 = (p.p840 / locals.var_wbin);
        let assign9030_e4541: f64 = (assign9030_e4537 + assign9030_e4540);
        let assign9030_e4544: f64 = (p.p855 / locals.var_lwbin);
        let assign9030_e4545: f64 = (assign9030_e4541 + assign9030_e4544);
        locals.var_uc_js0swd = assign9030_e4545;

        let assign9040_e4549: f64 = (p.p826 / locals.var_lbin);
        let assign9040_e4550: f64 = (p.p496 + assign9040_e4549);
        let assign9040_e4553: f64 = (p.p841 / locals.var_wbin);
        let assign9040_e4554: f64 = (assign9040_e4550 + assign9040_e4553);
        let assign9040_e4557: f64 = (p.p856 / locals.var_lwbin);
        let assign9040_e4558: f64 = (assign9040_e4554 + assign9040_e4557);
        locals.var_uc_njd = assign9040_e4558;

        let assign9050_e4562: f64 = (p.p827 / locals.var_lbin);
        let assign9050_e4563: f64 = (p.p513 + assign9050_e4562);
        let assign9050_e4566: f64 = (p.p842 / locals.var_wbin);
        let assign9050_e4567: f64 = (assign9050_e4563 + assign9050_e4566);
        let assign9050_e4570: f64 = (p.p857 / locals.var_lwbin);
        let assign9050_e4571: f64 = (assign9050_e4567 + assign9050_e4570);
        locals.var_uc_cisbkd = assign9050_e4571;

        let assign9060_e4575: f64 = (p.p828 / locals.var_lbin);
        let assign9060_e4576: f64 = (p.p515 + assign9060_e4575);
        let assign9060_e4579: f64 = (p.p843 / locals.var_wbin);
        let assign9060_e4580: f64 = (assign9060_e4576 + assign9060_e4579);
        let assign9060_e4583: f64 = (p.p858 / locals.var_lwbin);
        let assign9060_e4584: f64 = (assign9060_e4580 + assign9060_e4583);
        locals.var_uc_vdiffjd = assign9060_e4584;

        let assign9070_e4588: f64 = (p.p829 / locals.var_lbin);
        let assign9070_e4589: f64 = (p.p516 + assign9070_e4588);
        let assign9070_e4592: f64 = (p.p844 / locals.var_wbin);
        let assign9070_e4593: f64 = (assign9070_e4589 + assign9070_e4592);
        let assign9070_e4596: f64 = (p.p859 / locals.var_lwbin);
        let assign9070_e4597: f64 = (assign9070_e4593 + assign9070_e4596);
        locals.var_uc_js0s = assign9070_e4597;

        let assign9080_e4601: f64 = (p.p830 / locals.var_lbin);
        let assign9080_e4602: f64 = (p.p517 + assign9080_e4601);
        let assign9080_e4605: f64 = (p.p845 / locals.var_wbin);
        let assign9080_e4606: f64 = (assign9080_e4602 + assign9080_e4605);
        let assign9080_e4609: f64 = (p.p860 / locals.var_lwbin);
        let assign9080_e4610: f64 = (assign9080_e4606 + assign9080_e4609);
        locals.var_uc_js0sws = assign9080_e4610;

        let assign9090_e4614: f64 = (p.p831 / locals.var_lbin);
        let assign9090_e4615: f64 = (p.p519 + assign9090_e4614);
        let assign9090_e4618: f64 = (p.p846 / locals.var_wbin);
        let assign9090_e4619: f64 = (assign9090_e4615 + assign9090_e4618);
        let assign9090_e4622: f64 = (p.p861 / locals.var_lwbin);
        let assign9090_e4623: f64 = (assign9090_e4619 + assign9090_e4622);
        locals.var_uc_njs = assign9090_e4623;

        let assign9100_e4627: f64 = (p.p832 / locals.var_lbin);
        let assign9100_e4628: f64 = (p.p536 + assign9100_e4627);
        let assign9100_e4631: f64 = (p.p847 / locals.var_wbin);
        let assign9100_e4632: f64 = (assign9100_e4628 + assign9100_e4631);
        let assign9100_e4635: f64 = (p.p862 / locals.var_lwbin);
        let assign9100_e4636: f64 = (assign9100_e4632 + assign9100_e4635);
        locals.var_uc_cisbks = assign9100_e4636;

        let assign9110_e4640: f64 = (p.p833 / locals.var_lbin);
        let assign9110_e4641: f64 = (p.p538 + assign9110_e4640);
        let assign9110_e4644: f64 = (p.p848 / locals.var_wbin);
        let assign9110_e4645: f64 = (assign9110_e4641 + assign9110_e4644);
        let assign9110_e4648: f64 = (p.p863 / locals.var_lwbin);
        let assign9110_e4649: f64 = (assign9110_e4645 + assign9110_e4648);
        locals.var_uc_vdiffjs = assign9110_e4649;

        let assign9210_e4700: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard187 = assign9210_e4700;

        let (assign9220_e4706, assign9220_e4706_d_n0, assign9220_e4706_d_n2, assign9220_e4706_d_n4, assign9220_e4706_d_n5, assign9220_e4706_d_n6, assign9220_e4706_d_n7, assign9220_e4706_d_n8, assign9220_e4706_d_n9, assign9220_e4706_d_n10, assign9220_e4706_d_n11, assign9220_e4706_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9220_e4704: f64 = (locals.var_lg).powf(p.p342);
        (assign9220_e4704, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9220_e4706;
        locals.var_t3_dn0 = assign9220_e4706_d_n0;
        locals.var_t3_dn2 = assign9220_e4706_d_n2;
        locals.var_t3_dn4 = assign9220_e4706_d_n4;
        locals.var_t3_dn5 = assign9220_e4706_d_n5;
        locals.var_t3_dn6 = assign9220_e4706_d_n6;
        locals.var_t3_dn7 = assign9220_e4706_d_n7;
        locals.var_t3_dn8 = assign9220_e4706_d_n8;
        locals.var_t3_dn9 = assign9220_e4706_d_n9;
        locals.var_t3_dn10 = assign9220_e4706_d_n10;
        locals.var_t3_dn11 = assign9220_e4706_d_n11;
        locals.var_t3_dn14 = assign9220_e4706_d_n14;

        let (assign9230_e4716, assign9230_e4716_d_n0, assign9230_e4716_d_n2, assign9230_e4716_d_n4, assign9230_e4716_d_n5, assign9230_e4716_d_n6, assign9230_e4716_d_n7, assign9230_e4716_d_n8, assign9230_e4716_d_n9, assign9230_e4716_d_n10, assign9230_e4716_d_n11, assign9230_e4716_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9230_e4712: f64 = (p.p341 / locals.var_t3);
        let assign9230_e4713: f64 = (1.0 + assign9230_e4712);
        let assign9230_e4714: f64 = (locals.var_uc_ndepm * assign9230_e4713);
        (assign9230_e4714, ((locals.var_uc_ndepm_dn0 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn2 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn4 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn5 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn6 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn7 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn8 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn9 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn10 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn11 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn14 * assign9230_e4713) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9230_e4716;
        locals.var_uc_ndepm_dn0 = assign9230_e4716_d_n0;
        locals.var_uc_ndepm_dn2 = assign9230_e4716_d_n2;
        locals.var_uc_ndepm_dn4 = assign9230_e4716_d_n4;
        locals.var_uc_ndepm_dn5 = assign9230_e4716_d_n5;
        locals.var_uc_ndepm_dn6 = assign9230_e4716_d_n6;
        locals.var_uc_ndepm_dn7 = assign9230_e4716_d_n7;
        locals.var_uc_ndepm_dn8 = assign9230_e4716_d_n8;
        locals.var_uc_ndepm_dn9 = assign9230_e4716_d_n9;
        locals.var_uc_ndepm_dn10 = assign9230_e4716_d_n10;
        locals.var_uc_ndepm_dn11 = assign9230_e4716_d_n11;
        locals.var_uc_ndepm_dn14 = assign9230_e4716_d_n14;

        let assign9240_e4719: f64 = if locals.var_uc_ndepm < 1e21 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign9240_e4719;

        let (assign9250_e4725, assign9250_e4725_d_n0, assign9250_e4725_d_n2, assign9250_e4725_d_n4, assign9250_e4725_d_n5, assign9250_e4725_d_n6, assign9250_e4725_d_n7, assign9250_e4725_d_n8, assign9250_e4725_d_n9, assign9250_e4725_d_n10, assign9250_e4725_d_n11, assign9250_e4725_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard188 != 0.0)) {
        (1e21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9250_e4725;
        locals.var_uc_ndepm_dn0 = assign9250_e4725_d_n0;
        locals.var_uc_ndepm_dn2 = assign9250_e4725_d_n2;
        locals.var_uc_ndepm_dn4 = assign9250_e4725_d_n4;
        locals.var_uc_ndepm_dn5 = assign9250_e4725_d_n5;
        locals.var_uc_ndepm_dn6 = assign9250_e4725_d_n6;
        locals.var_uc_ndepm_dn7 = assign9250_e4725_d_n7;
        locals.var_uc_ndepm_dn8 = assign9250_e4725_d_n8;
        locals.var_uc_ndepm_dn9 = assign9250_e4725_d_n9;
        locals.var_uc_ndepm_dn10 = assign9250_e4725_d_n10;
        locals.var_uc_ndepm_dn11 = assign9250_e4725_d_n11;
        locals.var_uc_ndepm_dn14 = assign9250_e4725_d_n14;

        let (assign9260_e4731, assign9260_e4731_d_n0, assign9260_e4731_d_n2, assign9260_e4731_d_n4, assign9260_e4731_d_n5, assign9260_e4731_d_n6, assign9260_e4731_d_n7, assign9260_e4731_d_n8, assign9260_e4731_d_n9, assign9260_e4731_d_n10, assign9260_e4731_d_n11, assign9260_e4731_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9260_e4729: f64 = (locals.var_lg).powf(p.p369);
        (assign9260_e4729, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9260_e4731;
        locals.var_t3_dn0 = assign9260_e4731_d_n0;
        locals.var_t3_dn2 = assign9260_e4731_d_n2;
        locals.var_t3_dn4 = assign9260_e4731_d_n4;
        locals.var_t3_dn5 = assign9260_e4731_d_n5;
        locals.var_t3_dn6 = assign9260_e4731_d_n6;
        locals.var_t3_dn7 = assign9260_e4731_d_n7;
        locals.var_t3_dn8 = assign9260_e4731_d_n8;
        locals.var_t3_dn9 = assign9260_e4731_d_n9;
        locals.var_t3_dn10 = assign9260_e4731_d_n10;
        locals.var_t3_dn11 = assign9260_e4731_d_n11;
        locals.var_t3_dn14 = assign9260_e4731_d_n14;

        let (assign9270_e4741, assign9270_e4741_d_n0, assign9270_e4741_d_n2, assign9270_e4741_d_n4, assign9270_e4741_d_n5, assign9270_e4741_d_n6, assign9270_e4741_d_n7, assign9270_e4741_d_n8, assign9270_e4741_d_n9, assign9270_e4741_d_n10, assign9270_e4741_d_n11, assign9270_e4741_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9270_e4737: f64 = (p.p368 / locals.var_t3);
        let assign9270_e4738: f64 = (1.0 + assign9270_e4737);
        let assign9270_e4739: f64 = (locals.var_uc_depvmax * assign9270_e4738);
        (assign9270_e4739, ((locals.var_uc_depvmax_dn0 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn2 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn4 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn5 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn6 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn7 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn8 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn9 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn10 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn11 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn14 * assign9270_e4738) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign9270_e4741;
        locals.var_uc_depvmax_dn0 = assign9270_e4741_d_n0;
        locals.var_uc_depvmax_dn2 = assign9270_e4741_d_n2;
        locals.var_uc_depvmax_dn4 = assign9270_e4741_d_n4;
        locals.var_uc_depvmax_dn5 = assign9270_e4741_d_n5;
        locals.var_uc_depvmax_dn6 = assign9270_e4741_d_n6;
        locals.var_uc_depvmax_dn7 = assign9270_e4741_d_n7;
        locals.var_uc_depvmax_dn8 = assign9270_e4741_d_n8;
        locals.var_uc_depvmax_dn9 = assign9270_e4741_d_n9;
        locals.var_uc_depvmax_dn10 = assign9270_e4741_d_n10;
        locals.var_uc_depvmax_dn11 = assign9270_e4741_d_n11;
        locals.var_uc_depvmax_dn14 = assign9270_e4741_d_n14;

        let (assign9280_e4747, assign9280_e4747_d_n0, assign9280_e4747_d_n2, assign9280_e4747_d_n4, assign9280_e4747_d_n5, assign9280_e4747_d_n6, assign9280_e4747_d_n7, assign9280_e4747_d_n8, assign9280_e4747_d_n9, assign9280_e4747_d_n10, assign9280_e4747_d_n11, assign9280_e4747_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9280_e4745: f64 = (locals.var_lg).powf(p.p362);
        (assign9280_e4745, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9280_e4747;
        locals.var_t3_dn0 = assign9280_e4747_d_n0;
        locals.var_t3_dn2 = assign9280_e4747_d_n2;
        locals.var_t3_dn4 = assign9280_e4747_d_n4;
        locals.var_t3_dn5 = assign9280_e4747_d_n5;
        locals.var_t3_dn6 = assign9280_e4747_d_n6;
        locals.var_t3_dn7 = assign9280_e4747_d_n7;
        locals.var_t3_dn8 = assign9280_e4747_d_n8;
        locals.var_t3_dn9 = assign9280_e4747_d_n9;
        locals.var_t3_dn10 = assign9280_e4747_d_n10;
        locals.var_t3_dn11 = assign9280_e4747_d_n11;
        locals.var_t3_dn14 = assign9280_e4747_d_n14;

        let (assign9290_e4757, assign9290_e4757_d_n0, assign9290_e4757_d_n2, assign9290_e4757_d_n4, assign9290_e4757_d_n5, assign9290_e4757_d_n6, assign9290_e4757_d_n7, assign9290_e4757_d_n8, assign9290_e4757_d_n9, assign9290_e4757_d_n10, assign9290_e4757_d_n11, assign9290_e4757_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9290_e4753: f64 = (p.p361 / locals.var_t3);
        let assign9290_e4754: f64 = (1.0 + assign9290_e4753);
        let assign9290_e4755: f64 = (p.p360 * assign9290_e4754);
        (assign9290_e4755, (p.p360 * (-((p.p361 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9290_e4757;
        locals.var_uc_depleak_dn0 = assign9290_e4757_d_n0;
        locals.var_uc_depleak_dn2 = assign9290_e4757_d_n2;
        locals.var_uc_depleak_dn4 = assign9290_e4757_d_n4;
        locals.var_uc_depleak_dn5 = assign9290_e4757_d_n5;
        locals.var_uc_depleak_dn6 = assign9290_e4757_d_n6;
        locals.var_uc_depleak_dn7 = assign9290_e4757_d_n7;
        locals.var_uc_depleak_dn8 = assign9290_e4757_d_n8;
        locals.var_uc_depleak_dn9 = assign9290_e4757_d_n9;
        locals.var_uc_depleak_dn10 = assign9290_e4757_d_n10;
        locals.var_uc_depleak_dn11 = assign9290_e4757_d_n11;
        locals.var_uc_depleak_dn14 = assign9290_e4757_d_n14;

        let assign9300_e4760: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign9300_e4760;

        let (assign9310_e4766, assign9310_e4766_d_n0, assign9310_e4766_d_n2, assign9310_e4766_d_n4, assign9310_e4766_d_n5, assign9310_e4766_d_n6, assign9310_e4766_d_n7, assign9310_e4766_d_n8, assign9310_e4766_d_n9, assign9310_e4766_d_n10, assign9310_e4766_d_n11, assign9310_e4766_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard189 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9310_e4766;
        locals.var_uc_depleak_dn0 = assign9310_e4766_d_n0;
        locals.var_uc_depleak_dn2 = assign9310_e4766_d_n2;
        locals.var_uc_depleak_dn4 = assign9310_e4766_d_n4;
        locals.var_uc_depleak_dn5 = assign9310_e4766_d_n5;
        locals.var_uc_depleak_dn6 = assign9310_e4766_d_n6;
        locals.var_uc_depleak_dn7 = assign9310_e4766_d_n7;
        locals.var_uc_depleak_dn8 = assign9310_e4766_d_n8;
        locals.var_uc_depleak_dn9 = assign9310_e4766_d_n9;
        locals.var_uc_depleak_dn10 = assign9310_e4766_d_n10;
        locals.var_uc_depleak_dn11 = assign9310_e4766_d_n11;
        locals.var_uc_depleak_dn14 = assign9310_e4766_d_n14;

        let (assign9320_e4772, assign9320_e4772_d_n0, assign9320_e4772_d_n2, assign9320_e4772_d_n4, assign9320_e4772_d_n5, assign9320_e4772_d_n6, assign9320_e4772_d_n7, assign9320_e4772_d_n8, assign9320_e4772_d_n9, assign9320_e4772_d_n10, assign9320_e4772_d_n11, assign9320_e4772_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9320_e4770: f64 = (locals.var_lg).powf(p.p348);
        (assign9320_e4770, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9320_e4772;
        locals.var_t3_dn0 = assign9320_e4772_d_n0;
        locals.var_t3_dn2 = assign9320_e4772_d_n2;
        locals.var_t3_dn4 = assign9320_e4772_d_n4;
        locals.var_t3_dn5 = assign9320_e4772_d_n5;
        locals.var_t3_dn6 = assign9320_e4772_d_n6;
        locals.var_t3_dn7 = assign9320_e4772_d_n7;
        locals.var_t3_dn8 = assign9320_e4772_d_n8;
        locals.var_t3_dn9 = assign9320_e4772_d_n9;
        locals.var_t3_dn10 = assign9320_e4772_d_n10;
        locals.var_t3_dn11 = assign9320_e4772_d_n11;
        locals.var_t3_dn14 = assign9320_e4772_d_n14;

        let (assign9330_e4782, assign9330_e4782_d_n0, assign9330_e4782_d_n2, assign9330_e4782_d_n4, assign9330_e4782_d_n5, assign9330_e4782_d_n6, assign9330_e4782_d_n7, assign9330_e4782_d_n8, assign9330_e4782_d_n9, assign9330_e4782_d_n10, assign9330_e4782_d_n11, assign9330_e4782_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9330_e4778: f64 = (p.p347 / locals.var_t3);
        let assign9330_e4779: f64 = (1.0 + assign9330_e4778);
        let assign9330_e4780: f64 = (p.p346 * assign9330_e4779);
        (assign9330_e4780, (p.p346 * (-((p.p347 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9330_e4782;
        locals.var_uc_depmue0_dn0 = assign9330_e4782_d_n0;
        locals.var_uc_depmue0_dn2 = assign9330_e4782_d_n2;
        locals.var_uc_depmue0_dn4 = assign9330_e4782_d_n4;
        locals.var_uc_depmue0_dn5 = assign9330_e4782_d_n5;
        locals.var_uc_depmue0_dn6 = assign9330_e4782_d_n6;
        locals.var_uc_depmue0_dn7 = assign9330_e4782_d_n7;
        locals.var_uc_depmue0_dn8 = assign9330_e4782_d_n8;
        locals.var_uc_depmue0_dn9 = assign9330_e4782_d_n9;
        locals.var_uc_depmue0_dn10 = assign9330_e4782_d_n10;
        locals.var_uc_depmue0_dn11 = assign9330_e4782_d_n11;
        locals.var_uc_depmue0_dn14 = assign9330_e4782_d_n14;

        let assign9340_e4785: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign9340_e4785;

        let (assign9350_e4791, assign9350_e4791_d_n0, assign9350_e4791_d_n2, assign9350_e4791_d_n4, assign9350_e4791_d_n5, assign9350_e4791_d_n6, assign9350_e4791_d_n7, assign9350_e4791_d_n8, assign9350_e4791_d_n9, assign9350_e4791_d_n10, assign9350_e4791_d_n11, assign9350_e4791_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard190 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9350_e4791;
        locals.var_uc_depmue0_dn0 = assign9350_e4791_d_n0;
        locals.var_uc_depmue0_dn2 = assign9350_e4791_d_n2;
        locals.var_uc_depmue0_dn4 = assign9350_e4791_d_n4;
        locals.var_uc_depmue0_dn5 = assign9350_e4791_d_n5;
        locals.var_uc_depmue0_dn6 = assign9350_e4791_d_n6;
        locals.var_uc_depmue0_dn7 = assign9350_e4791_d_n7;
        locals.var_uc_depmue0_dn8 = assign9350_e4791_d_n8;
        locals.var_uc_depmue0_dn9 = assign9350_e4791_d_n9;
        locals.var_uc_depmue0_dn10 = assign9350_e4791_d_n10;
        locals.var_uc_depmue0_dn11 = assign9350_e4791_d_n11;
        locals.var_uc_depmue0_dn14 = assign9350_e4791_d_n14;

        let (assign9360_e4797, assign9360_e4797_d_n0, assign9360_e4797_d_n2, assign9360_e4797_d_n4, assign9360_e4797_d_n5, assign9360_e4797_d_n6, assign9360_e4797_d_n7, assign9360_e4797_d_n8, assign9360_e4797_d_n9, assign9360_e4797_d_n10, assign9360_e4797_d_n11, assign9360_e4797_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9360_e4795: f64 = (locals.var_lg).powf(p.p351);
        (assign9360_e4795, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9360_e4797;
        locals.var_t3_dn0 = assign9360_e4797_d_n0;
        locals.var_t3_dn2 = assign9360_e4797_d_n2;
        locals.var_t3_dn4 = assign9360_e4797_d_n4;
        locals.var_t3_dn5 = assign9360_e4797_d_n5;
        locals.var_t3_dn6 = assign9360_e4797_d_n6;
        locals.var_t3_dn7 = assign9360_e4797_d_n7;
        locals.var_t3_dn8 = assign9360_e4797_d_n8;
        locals.var_t3_dn9 = assign9360_e4797_d_n9;
        locals.var_t3_dn10 = assign9360_e4797_d_n10;
        locals.var_t3_dn11 = assign9360_e4797_d_n11;
        locals.var_t3_dn14 = assign9360_e4797_d_n14;

        let (assign9370_e4807, assign9370_e4807_d_n0, assign9370_e4807_d_n2, assign9370_e4807_d_n4, assign9370_e4807_d_n5, assign9370_e4807_d_n6, assign9370_e4807_d_n7, assign9370_e4807_d_n8, assign9370_e4807_d_n9, assign9370_e4807_d_n10, assign9370_e4807_d_n11, assign9370_e4807_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9370_e4803: f64 = (p.p350 / locals.var_t3);
        let assign9370_e4804: f64 = (1.0 + assign9370_e4803);
        let assign9370_e4805: f64 = (p.p349 * assign9370_e4804);
        (assign9370_e4805, (p.p349 * (-((p.p350 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9370_e4807;
        locals.var_uc_depmue1_dn0 = assign9370_e4807_d_n0;
        locals.var_uc_depmue1_dn2 = assign9370_e4807_d_n2;
        locals.var_uc_depmue1_dn4 = assign9370_e4807_d_n4;
        locals.var_uc_depmue1_dn5 = assign9370_e4807_d_n5;
        locals.var_uc_depmue1_dn6 = assign9370_e4807_d_n6;
        locals.var_uc_depmue1_dn7 = assign9370_e4807_d_n7;
        locals.var_uc_depmue1_dn8 = assign9370_e4807_d_n8;
        locals.var_uc_depmue1_dn9 = assign9370_e4807_d_n9;
        locals.var_uc_depmue1_dn10 = assign9370_e4807_d_n10;
        locals.var_uc_depmue1_dn11 = assign9370_e4807_d_n11;
        locals.var_uc_depmue1_dn14 = assign9370_e4807_d_n14;

        let assign9380_e4810: f64 = if locals.var_uc_depmue1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign9380_e4810;

        let (assign9390_e4816, assign9390_e4816_d_n0, assign9390_e4816_d_n2, assign9390_e4816_d_n4, assign9390_e4816_d_n5, assign9390_e4816_d_n6, assign9390_e4816_d_n7, assign9390_e4816_d_n8, assign9390_e4816_d_n9, assign9390_e4816_d_n10, assign9390_e4816_d_n11, assign9390_e4816_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9390_e4816;
        locals.var_uc_depmue1_dn0 = assign9390_e4816_d_n0;
        locals.var_uc_depmue1_dn2 = assign9390_e4816_d_n2;
        locals.var_uc_depmue1_dn4 = assign9390_e4816_d_n4;
        locals.var_uc_depmue1_dn5 = assign9390_e4816_d_n5;
        locals.var_uc_depmue1_dn6 = assign9390_e4816_d_n6;
        locals.var_uc_depmue1_dn7 = assign9390_e4816_d_n7;
        locals.var_uc_depmue1_dn8 = assign9390_e4816_d_n8;
        locals.var_uc_depmue1_dn9 = assign9390_e4816_d_n9;
        locals.var_uc_depmue1_dn10 = assign9390_e4816_d_n10;
        locals.var_uc_depmue1_dn11 = assign9390_e4816_d_n11;
        locals.var_uc_depmue1_dn14 = assign9390_e4816_d_n14;

        let (assign9400_e4822, assign9400_e4822_d_n0, assign9400_e4822_d_n2, assign9400_e4822_d_n4, assign9400_e4822_d_n5, assign9400_e4822_d_n6, assign9400_e4822_d_n7, assign9400_e4822_d_n8, assign9400_e4822_d_n9, assign9400_e4822_d_n10, assign9400_e4822_d_n11, assign9400_e4822_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9400_e4820: f64 = (locals.var_lg).powf(p.p357);
        (assign9400_e4820, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9400_e4822;
        locals.var_t3_dn0 = assign9400_e4822_d_n0;
        locals.var_t3_dn2 = assign9400_e4822_d_n2;
        locals.var_t3_dn4 = assign9400_e4822_d_n4;
        locals.var_t3_dn5 = assign9400_e4822_d_n5;
        locals.var_t3_dn6 = assign9400_e4822_d_n6;
        locals.var_t3_dn7 = assign9400_e4822_d_n7;
        locals.var_t3_dn8 = assign9400_e4822_d_n8;
        locals.var_t3_dn9 = assign9400_e4822_d_n9;
        locals.var_t3_dn10 = assign9400_e4822_d_n10;
        locals.var_t3_dn11 = assign9400_e4822_d_n11;
        locals.var_t3_dn14 = assign9400_e4822_d_n14;

        let (assign9410_e4832, assign9410_e4832_d_n0, assign9410_e4832_d_n2, assign9410_e4832_d_n4, assign9410_e4832_d_n5, assign9410_e4832_d_n6, assign9410_e4832_d_n7, assign9410_e4832_d_n8, assign9410_e4832_d_n9, assign9410_e4832_d_n10, assign9410_e4832_d_n11, assign9410_e4832_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9410_e4828: f64 = (p.p356 / locals.var_t3);
        let assign9410_e4829: f64 = (1.0 + assign9410_e4828);
        let assign9410_e4830: f64 = (p.p354 * assign9410_e4829);
        (assign9410_e4830, (p.p354 * (-((p.p356 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9410_e4832;
        locals.var_uc_depmueback0_dn0 = assign9410_e4832_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9410_e4832_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9410_e4832_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9410_e4832_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9410_e4832_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9410_e4832_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9410_e4832_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9410_e4832_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9410_e4832_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9410_e4832_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9410_e4832_d_n14;

        let assign9420_e4835: f64 = if locals.var_uc_depmueback0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign9420_e4835;

        let (assign9430_e4841, assign9430_e4841_d_n0, assign9430_e4841_d_n2, assign9430_e4841_d_n4, assign9430_e4841_d_n5, assign9430_e4841_d_n6, assign9430_e4841_d_n7, assign9430_e4841_d_n8, assign9430_e4841_d_n9, assign9430_e4841_d_n10, assign9430_e4841_d_n11, assign9430_e4841_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9430_e4841;
        locals.var_uc_depmueback0_dn0 = assign9430_e4841_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9430_e4841_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9430_e4841_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9430_e4841_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9430_e4841_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9430_e4841_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9430_e4841_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9430_e4841_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9430_e4841_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9430_e4841_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9430_e4841_d_n14;

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9440_e4847, assign9440_e4847_d_n0, assign9440_e4847_d_n2, assign9440_e4847_d_n4, assign9440_e4847_d_n5, assign9440_e4847_d_n6, assign9440_e4847_d_n7, assign9440_e4847_d_n8, assign9440_e4847_d_n9, assign9440_e4847_d_n10, assign9440_e4847_d_n11, assign9440_e4847_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9440_e4845: f64 = (locals.var_lg).powf(p.p359);
        (assign9440_e4845, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9440_e4847;
        locals.var_t3_dn0 = assign9440_e4847_d_n0;
        locals.var_t3_dn2 = assign9440_e4847_d_n2;
        locals.var_t3_dn4 = assign9440_e4847_d_n4;
        locals.var_t3_dn5 = assign9440_e4847_d_n5;
        locals.var_t3_dn6 = assign9440_e4847_d_n6;
        locals.var_t3_dn7 = assign9440_e4847_d_n7;
        locals.var_t3_dn8 = assign9440_e4847_d_n8;
        locals.var_t3_dn9 = assign9440_e4847_d_n9;
        locals.var_t3_dn10 = assign9440_e4847_d_n10;
        locals.var_t3_dn11 = assign9440_e4847_d_n11;
        locals.var_t3_dn14 = assign9440_e4847_d_n14;

        let (assign9450_e4857, assign9450_e4857_d_n0, assign9450_e4857_d_n2, assign9450_e4857_d_n4, assign9450_e4857_d_n5, assign9450_e4857_d_n6, assign9450_e4857_d_n7, assign9450_e4857_d_n8, assign9450_e4857_d_n9, assign9450_e4857_d_n10, assign9450_e4857_d_n11, assign9450_e4857_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9450_e4853: f64 = (p.p358 / locals.var_t3);
        let assign9450_e4854: f64 = (1.0 + assign9450_e4853);
        let assign9450_e4855: f64 = (p.p355 * assign9450_e4854);
        (assign9450_e4855, (p.p355 * (-((p.p358 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9450_e4857;
        locals.var_uc_depmueback1_dn0 = assign9450_e4857_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9450_e4857_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9450_e4857_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9450_e4857_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9450_e4857_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9450_e4857_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9450_e4857_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9450_e4857_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9450_e4857_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9450_e4857_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9450_e4857_d_n14;

        let assign9460_e4860: f64 = if locals.var_uc_depmueback1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign9460_e4860;

        let (assign9470_e4866, assign9470_e4866_d_n0, assign9470_e4866_d_n2, assign9470_e4866_d_n4, assign9470_e4866_d_n5, assign9470_e4866_d_n6, assign9470_e4866_d_n7, assign9470_e4866_d_n8, assign9470_e4866_d_n9, assign9470_e4866_d_n10, assign9470_e4866_d_n11, assign9470_e4866_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard193 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9470_e4866;
        locals.var_uc_depmueback1_dn0 = assign9470_e4866_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9470_e4866_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9470_e4866_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9470_e4866_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9470_e4866_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9470_e4866_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9470_e4866_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9470_e4866_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9470_e4866_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9470_e4866_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9470_e4866_d_n14;

        let (assign9480_e4872, assign9480_e4872_d_n0, assign9480_e4872_d_n2, assign9480_e4872_d_n4, assign9480_e4872_d_n5, assign9480_e4872_d_n6, assign9480_e4872_d_n7, assign9480_e4872_d_n8, assign9480_e4872_d_n9, assign9480_e4872_d_n10, assign9480_e4872_d_n11, assign9480_e4872_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9480_e4870: f64 = (locals.var_lg).powf(p.p373);
        (assign9480_e4870, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9480_e4872;
        locals.var_t3_dn0 = assign9480_e4872_d_n0;
        locals.var_t3_dn2 = assign9480_e4872_d_n2;
        locals.var_t3_dn4 = assign9480_e4872_d_n4;
        locals.var_t3_dn5 = assign9480_e4872_d_n5;
        locals.var_t3_dn6 = assign9480_e4872_d_n6;
        locals.var_t3_dn7 = assign9480_e4872_d_n7;
        locals.var_t3_dn8 = assign9480_e4872_d_n8;
        locals.var_t3_dn9 = assign9480_e4872_d_n9;
        locals.var_t3_dn10 = assign9480_e4872_d_n10;
        locals.var_t3_dn11 = assign9480_e4872_d_n11;
        locals.var_t3_dn14 = assign9480_e4872_d_n14;

        let (assign9490_e4882, assign9490_e4882_d_n0, assign9490_e4882_d_n2, assign9490_e4882_d_n4, assign9490_e4882_d_n5, assign9490_e4882_d_n6, assign9490_e4882_d_n7, assign9490_e4882_d_n8, assign9490_e4882_d_n9, assign9490_e4882_d_n10, assign9490_e4882_d_n11, assign9490_e4882_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9490_e4878: f64 = (p.p372 / locals.var_t3);
        let assign9490_e4879: f64 = (1.0 + assign9490_e4878);
        let assign9490_e4880: f64 = (locals.var_uc_depvdsef1 * assign9490_e4879);
        (assign9490_e4880, ((locals.var_uc_depvdsef1_dn0 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn2 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn4 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn5 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn6 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn7 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn8 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn9 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn10 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn11 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn14 * assign9490_e4879) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn11, locals.var_uc_depvdsef1_dn14,)
    }
};
        locals.var_uc_depvdsef1 = assign9490_e4882;
        locals.var_uc_depvdsef1_dn0 = assign9490_e4882_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9490_e4882_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9490_e4882_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9490_e4882_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9490_e4882_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9490_e4882_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9490_e4882_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9490_e4882_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9490_e4882_d_n10;
        locals.var_uc_depvdsef1_dn11 = assign9490_e4882_d_n11;
        locals.var_uc_depvdsef1_dn14 = assign9490_e4882_d_n14;

        let (assign9500_e4888, assign9500_e4888_d_n0, assign9500_e4888_d_n2, assign9500_e4888_d_n4, assign9500_e4888_d_n5, assign9500_e4888_d_n6, assign9500_e4888_d_n7, assign9500_e4888_d_n8, assign9500_e4888_d_n9, assign9500_e4888_d_n10, assign9500_e4888_d_n11, assign9500_e4888_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9500_e4886: f64 = (locals.var_lg).powf(p.p375);
        (assign9500_e4886, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9500_e4888;
        locals.var_t3_dn0 = assign9500_e4888_d_n0;
        locals.var_t3_dn2 = assign9500_e4888_d_n2;
        locals.var_t3_dn4 = assign9500_e4888_d_n4;
        locals.var_t3_dn5 = assign9500_e4888_d_n5;
        locals.var_t3_dn6 = assign9500_e4888_d_n6;
        locals.var_t3_dn7 = assign9500_e4888_d_n7;
        locals.var_t3_dn8 = assign9500_e4888_d_n8;
        locals.var_t3_dn9 = assign9500_e4888_d_n9;
        locals.var_t3_dn10 = assign9500_e4888_d_n10;
        locals.var_t3_dn11 = assign9500_e4888_d_n11;
        locals.var_t3_dn14 = assign9500_e4888_d_n14;

        let (assign9510_e4898, assign9510_e4898_d_n0, assign9510_e4898_d_n2, assign9510_e4898_d_n4, assign9510_e4898_d_n5, assign9510_e4898_d_n6, assign9510_e4898_d_n7, assign9510_e4898_d_n8, assign9510_e4898_d_n9, assign9510_e4898_d_n10, assign9510_e4898_d_n11, assign9510_e4898_d_n14,) = {
    if (locals.var_guard187 != 0.0) {
        let assign9510_e4894: f64 = (p.p374 / locals.var_t3);
        let assign9510_e4895: f64 = (1.0 + assign9510_e4894);
        let assign9510_e4896: f64 = (locals.var_uc_depvdsef2 * assign9510_e4895);
        (assign9510_e4896, ((locals.var_uc_depvdsef2_dn0 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn2 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn4 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn5 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn6 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn7 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn8 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn9 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn10 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn11 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn14 * assign9510_e4895) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9510_e4898;
        locals.var_uc_depvdsef2_dn0 = assign9510_e4898_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9510_e4898_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9510_e4898_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9510_e4898_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9510_e4898_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9510_e4898_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9510_e4898_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9510_e4898_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9510_e4898_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9510_e4898_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9510_e4898_d_n14;

        let assign9520_e4901: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign9520_e4901;

        let (assign9530_e4907, assign9530_e4907_d_n0, assign9530_e4907_d_n2, assign9530_e4907_d_n4, assign9530_e4907_d_n5, assign9530_e4907_d_n6, assign9530_e4907_d_n7, assign9530_e4907_d_n8, assign9530_e4907_d_n9, assign9530_e4907_d_n10, assign9530_e4907_d_n11, assign9530_e4907_d_n14,) = {
    if ((locals.var_guard187 != 0.0) && (locals.var_guard194 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9530_e4907;
        locals.var_uc_depvdsef2_dn0 = assign9530_e4907_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9530_e4907_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9530_e4907_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9530_e4907_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9530_e4907_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9530_e4907_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9530_e4907_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9530_e4907_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9530_e4907_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9530_e4907_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9530_e4907_d_n14;

        let (assign9540_e4912, assign9540_e4912_d_n0, assign9540_e4912_d_n2, assign9540_e4912_d_n4, assign9540_e4912_d_n5, assign9540_e4912_d_n6, assign9540_e4912_d_n7, assign9540_e4912_d_n8, assign9540_e4912_d_n9, assign9540_e4912_d_n10, assign9540_e4912_d_n11, assign9540_e4912_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9540_e4912;
        locals.var_uc_ndepm_dn0 = assign9540_e4912_d_n0;
        locals.var_uc_ndepm_dn2 = assign9540_e4912_d_n2;
        locals.var_uc_ndepm_dn4 = assign9540_e4912_d_n4;
        locals.var_uc_ndepm_dn5 = assign9540_e4912_d_n5;
        locals.var_uc_ndepm_dn6 = assign9540_e4912_d_n6;
        locals.var_uc_ndepm_dn7 = assign9540_e4912_d_n7;
        locals.var_uc_ndepm_dn8 = assign9540_e4912_d_n8;
        locals.var_uc_ndepm_dn9 = assign9540_e4912_d_n9;
        locals.var_uc_ndepm_dn10 = assign9540_e4912_d_n10;
        locals.var_uc_ndepm_dn11 = assign9540_e4912_d_n11;
        locals.var_uc_ndepm_dn14 = assign9540_e4912_d_n14;

        let (assign9550_e4917, assign9550_e4917_d_n0, assign9550_e4917_d_n2, assign9550_e4917_d_n4, assign9550_e4917_d_n5, assign9550_e4917_d_n6, assign9550_e4917_d_n7, assign9550_e4917_d_n8, assign9550_e4917_d_n9, assign9550_e4917_d_n10, assign9550_e4917_d_n11, assign9550_e4917_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign9550_e4917;
        locals.var_uc_depvmax_dn0 = assign9550_e4917_d_n0;
        locals.var_uc_depvmax_dn2 = assign9550_e4917_d_n2;
        locals.var_uc_depvmax_dn4 = assign9550_e4917_d_n4;
        locals.var_uc_depvmax_dn5 = assign9550_e4917_d_n5;
        locals.var_uc_depvmax_dn6 = assign9550_e4917_d_n6;
        locals.var_uc_depvmax_dn7 = assign9550_e4917_d_n7;
        locals.var_uc_depvmax_dn8 = assign9550_e4917_d_n8;
        locals.var_uc_depvmax_dn9 = assign9550_e4917_d_n9;
        locals.var_uc_depvmax_dn10 = assign9550_e4917_d_n10;
        locals.var_uc_depvmax_dn11 = assign9550_e4917_d_n11;
        locals.var_uc_depvmax_dn14 = assign9550_e4917_d_n14;

        let (assign9560_e4922, assign9560_e4922_d_n0, assign9560_e4922_d_n2, assign9560_e4922_d_n4, assign9560_e4922_d_n5, assign9560_e4922_d_n6, assign9560_e4922_d_n7, assign9560_e4922_d_n8, assign9560_e4922_d_n9, assign9560_e4922_d_n10, assign9560_e4922_d_n11, assign9560_e4922_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9560_e4922;
        locals.var_uc_depleak_dn0 = assign9560_e4922_d_n0;
        locals.var_uc_depleak_dn2 = assign9560_e4922_d_n2;
        locals.var_uc_depleak_dn4 = assign9560_e4922_d_n4;
        locals.var_uc_depleak_dn5 = assign9560_e4922_d_n5;
        locals.var_uc_depleak_dn6 = assign9560_e4922_d_n6;
        locals.var_uc_depleak_dn7 = assign9560_e4922_d_n7;
        locals.var_uc_depleak_dn8 = assign9560_e4922_d_n8;
        locals.var_uc_depleak_dn9 = assign9560_e4922_d_n9;
        locals.var_uc_depleak_dn10 = assign9560_e4922_d_n10;
        locals.var_uc_depleak_dn11 = assign9560_e4922_d_n11;
        locals.var_uc_depleak_dn14 = assign9560_e4922_d_n14;

        let (assign9570_e4927, assign9570_e4927_d_n0, assign9570_e4927_d_n2, assign9570_e4927_d_n4, assign9570_e4927_d_n5, assign9570_e4927_d_n6, assign9570_e4927_d_n7, assign9570_e4927_d_n8, assign9570_e4927_d_n9, assign9570_e4927_d_n10, assign9570_e4927_d_n11, assign9570_e4927_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9570_e4927;
        locals.var_uc_depmue0_dn0 = assign9570_e4927_d_n0;
        locals.var_uc_depmue0_dn2 = assign9570_e4927_d_n2;
        locals.var_uc_depmue0_dn4 = assign9570_e4927_d_n4;
        locals.var_uc_depmue0_dn5 = assign9570_e4927_d_n5;
        locals.var_uc_depmue0_dn6 = assign9570_e4927_d_n6;
        locals.var_uc_depmue0_dn7 = assign9570_e4927_d_n7;
        locals.var_uc_depmue0_dn8 = assign9570_e4927_d_n8;
        locals.var_uc_depmue0_dn9 = assign9570_e4927_d_n9;
        locals.var_uc_depmue0_dn10 = assign9570_e4927_d_n10;
        locals.var_uc_depmue0_dn11 = assign9570_e4927_d_n11;
        locals.var_uc_depmue0_dn14 = assign9570_e4927_d_n14;

        let (assign9580_e4932, assign9580_e4932_d_n0, assign9580_e4932_d_n2, assign9580_e4932_d_n4, assign9580_e4932_d_n5, assign9580_e4932_d_n6, assign9580_e4932_d_n7, assign9580_e4932_d_n8, assign9580_e4932_d_n9, assign9580_e4932_d_n10, assign9580_e4932_d_n11, assign9580_e4932_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9580_e4932;
        locals.var_uc_depmue1_dn0 = assign9580_e4932_d_n0;
        locals.var_uc_depmue1_dn2 = assign9580_e4932_d_n2;
        locals.var_uc_depmue1_dn4 = assign9580_e4932_d_n4;
        locals.var_uc_depmue1_dn5 = assign9580_e4932_d_n5;
        locals.var_uc_depmue1_dn6 = assign9580_e4932_d_n6;
        locals.var_uc_depmue1_dn7 = assign9580_e4932_d_n7;
        locals.var_uc_depmue1_dn8 = assign9580_e4932_d_n8;
        locals.var_uc_depmue1_dn9 = assign9580_e4932_d_n9;
        locals.var_uc_depmue1_dn10 = assign9580_e4932_d_n10;
        locals.var_uc_depmue1_dn11 = assign9580_e4932_d_n11;
        locals.var_uc_depmue1_dn14 = assign9580_e4932_d_n14;

        let (assign9590_e4937, assign9590_e4937_d_n0, assign9590_e4937_d_n2, assign9590_e4937_d_n4, assign9590_e4937_d_n5, assign9590_e4937_d_n6, assign9590_e4937_d_n7, assign9590_e4937_d_n8, assign9590_e4937_d_n9, assign9590_e4937_d_n10, assign9590_e4937_d_n11, assign9590_e4937_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9590_e4937;
        locals.var_uc_depmueback0_dn0 = assign9590_e4937_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9590_e4937_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9590_e4937_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9590_e4937_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9590_e4937_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9590_e4937_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9590_e4937_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9590_e4937_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9590_e4937_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9590_e4937_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9590_e4937_d_n14;

        let (assign9600_e4942, assign9600_e4942_d_n0, assign9600_e4942_d_n2, assign9600_e4942_d_n4, assign9600_e4942_d_n5, assign9600_e4942_d_n6, assign9600_e4942_d_n7, assign9600_e4942_d_n8, assign9600_e4942_d_n9, assign9600_e4942_d_n10, assign9600_e4942_d_n11, assign9600_e4942_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9600_e4942;
        locals.var_uc_depmueback1_dn0 = assign9600_e4942_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9600_e4942_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9600_e4942_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9600_e4942_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9600_e4942_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9600_e4942_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9600_e4942_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9600_e4942_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9600_e4942_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9600_e4942_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9600_e4942_d_n14;

        let (assign9610_e4947, assign9610_e4947_d_n0, assign9610_e4947_d_n2, assign9610_e4947_d_n4, assign9610_e4947_d_n5, assign9610_e4947_d_n6, assign9610_e4947_d_n7, assign9610_e4947_d_n8, assign9610_e4947_d_n9, assign9610_e4947_d_n10, assign9610_e4947_d_n11, assign9610_e4947_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn11, locals.var_uc_depvdsef1_dn14,)
    }
};
        locals.var_uc_depvdsef1 = assign9610_e4947;
        locals.var_uc_depvdsef1_dn0 = assign9610_e4947_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9610_e4947_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9610_e4947_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9610_e4947_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9610_e4947_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9610_e4947_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9610_e4947_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9610_e4947_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9610_e4947_d_n10;
        locals.var_uc_depvdsef1_dn11 = assign9610_e4947_d_n11;
        locals.var_uc_depvdsef1_dn14 = assign9610_e4947_d_n14;

        let (assign9620_e4952, assign9620_e4952_d_n0, assign9620_e4952_d_n2, assign9620_e4952_d_n4, assign9620_e4952_d_n5, assign9620_e4952_d_n6, assign9620_e4952_d_n7, assign9620_e4952_d_n8, assign9620_e4952_d_n9, assign9620_e4952_d_n10, assign9620_e4952_d_n11, assign9620_e4952_d_n14,) = {
    if (locals.var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9620_e4952;
        locals.var_uc_depvdsef2_dn0 = assign9620_e4952_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9620_e4952_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9620_e4952_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9620_e4952_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9620_e4952_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9620_e4952_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9620_e4952_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9620_e4952_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9620_e4952_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9620_e4952_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9620_e4952_d_n14;

        let assign10140_e5325: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign10140_e5327: f64 = if assign10140_e5325 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard246 = assign10140_e5327;

        let (assign10150_e5333,) = {
    if (locals.var_guard246 != 0.0) {
        let assign10150_e5331: f64 = (1.0 / locals.var_uc_xldld);
        (assign10150_e5331,)
    } else {
        (locals.var_uc_xpdv,)
    }
};
        locals.var_uc_xpdv = assign10150_e5333;

        let assign10170_e5361: f64 = if ((p.p40 == 1.0) && (((p.p19 > 0.0) && (locals.var_uc_nover == 0.0)) || ((p.p18 > 0.0) && (locals.var_uc_novers == 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard248 = assign10170_e5361;

        let (assign10180_e5365,) = {
    if (locals.var_guard248 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10180_e5365;

        let (assign10190_e5370,) = {
    if (locals.var_guard248 == 0.0) {
        (p.p40,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10190_e5370;

        let assign10200_e5373: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign10200_e5373;

        let (assign10210_e5382,) = {
    if (locals.var_guard249 != 0.0) {
        let (assign10210_e5380,) = {
            if (p.p19 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10210_e5380,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10210_e5382;

        let (assign10220_e5391,) = {
    if (locals.var_guard249 != 0.0) {
        let (assign10220_e5389,) = {
            if (p.p18 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10220_e5389,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10220_e5391;

        let assign10230_e5398: f64 = if ((p.p17 == 0.0) || (p.p17 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard250 = assign10230_e5398;

        let (assign10240_e5405,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10240_e5405;

        let (assign10250_e5412,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10250_e5412;

        let (assign10260_e5444, assign10260_e5444_d_n0, assign10260_e5444_d_n2, assign10260_e5444_d_n4, assign10260_e5444_d_n5, assign10260_e5444_d_n6, assign10260_e5444_d_n7, assign10260_e5444_d_n8, assign10260_e5444_d_n9, assign10260_e5444_d_n10, assign10260_e5444_d_n11, assign10260_e5444_d_n14,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 == 0.0)) {
        let assign10260_e5420: f64 = (p.p130 * p.p2);
        let assign10260_e5422: f64 = (assign10260_e5420 * p.p7);
        let assign10260_e5425: f64 = (locals.var_uc_rd + locals.var_uc_rdvd);
        let assign10260_e5428: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign10260_e5430: f64 = (assign10260_e5428 * 1000000.0);
        let assign10260_e5432: f64 = (assign10260_e5430 + locals.var_uc_rdict1);
        let assign10260_e5433: f64 = (assign10260_e5425 * assign10260_e5432);
        let assign10260_e5436: f64 = (p.p68 * p.p100);
        let assign10260_e5438: f64 = (assign10260_e5436 * 1000000.0);
        let assign10260_e5440: f64 = (assign10260_e5438 + p.p101);
        let assign10260_e5441: f64 = (assign10260_e5433 * assign10260_e5440);
        let assign10260_e5442: f64 = (assign10260_e5422 + assign10260_e5441);
        (assign10260_e5442, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10260_e5444;
        locals.var_t1_dn0 = assign10260_e5444_d_n0;
        locals.var_t1_dn2 = assign10260_e5444_d_n2;
        locals.var_t1_dn4 = assign10260_e5444_d_n4;
        locals.var_t1_dn5 = assign10260_e5444_d_n5;
        locals.var_t1_dn6 = assign10260_e5444_d_n6;
        locals.var_t1_dn7 = assign10260_e5444_d_n7;
        locals.var_t1_dn8 = assign10260_e5444_d_n8;
        locals.var_t1_dn9 = assign10260_e5444_d_n9;
        locals.var_t1_dn10 = assign10260_e5444_d_n10;
        locals.var_t1_dn11 = assign10260_e5444_d_n11;
        locals.var_t1_dn14 = assign10260_e5444_d_n14;

        let (assign10270_e5457,) = {
    if ((locals.var_guard249 == 0.0) && (locals.var_guard250 == 0.0)) {
        let (assign10270_e5455,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10270_e5455,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10270_e5457;

    }
}
