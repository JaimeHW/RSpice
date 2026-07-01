#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign10_e1394: f64 = if param_given[12] { 1.0 } else { 0.0 };
        locals.var_nsubcdfm_given = assign10_e1394;

        let assign20_e1396: f64 = if param_given[268] { 1.0 } else { 0.0 };
        locals.var_cgdo_given = assign20_e1396;

        let assign30_e1398: f64 = if param_given[269] { 1.0 } else { 0.0 };
        locals.var_cgso_given = assign30_e1398;

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
        locals.var_xd_dn13 = 0.0;

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
        locals.var_rdd_dn13 = 0.0;

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
        locals.var_rsd_dn13 = 0.0;

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
        locals.var_rd_ps0ld_dn13 = 0.0;

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
        locals.var_rd_qbuld_dn13 = 0.0;

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
        locals.var_vbs_max_dn13 = 0.0;

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
        locals.var_vbs_bnd_dn13 = 0.0;

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
        locals.var_vbscl_dn13 = 0.0;

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
        locals.var_vbscldvbs_dn13 = 0.0;

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
        locals.var_vgp_dn13 = 0.0;

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
        locals.var_ps0_dn13 = 0.0;

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
        locals.var_ps0_ini_dn13 = 0.0;

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
        locals.var_ps0_inia_dn13 = 0.0;

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
        locals.var_ps0_inib_dn13 = 0.0;

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
        locals.var_psl_dn13 = 0.0;

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
        locals.var_psl_lim_dn13 = 0.0;

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
        locals.var_dplim_dn13 = 0.0;

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
        locals.var_pds_dn13 = 0.0;

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
        locals.var_pds_ini_dn13 = 0.0;

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
        locals.var_pds_max_dn13 = 0.0;

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
        locals.var_xi0_dn13 = 0.0;

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
        locals.var_xi0p12_dn13 = 0.0;

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
        locals.var_xi0p32_dn13 = 0.0;

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
        locals.var_xil_dn13 = 0.0;

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
        locals.var_xilp12_dn13 = 0.0;

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
        locals.var_xilp32_dn13 = 0.0;

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
        locals.var_vbsz_dn13 = 0.0;

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
        locals.var_vdsz_dn13 = 0.0;

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
        locals.var_vgsz_dn13 = 0.0;

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
        locals.var_vzadd_dn13 = 0.0;

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
        locals.var_ps0z_dn13 = 0.0;

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
        locals.var_pzadd_dn13 = 0.0;

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
        locals.var_dvbsibpc_dn13 = 0.0;

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
        locals.var_dg3_dn13 = 0.0;

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
        locals.var_dg4_dn13 = 0.0;

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
        locals.var_didd_dn13 = 0.0;

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
        locals.var_betawl_dn13 = 0.0;

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
        locals.var_chi_dn13 = 0.0;

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
        locals.var_chib_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_1(
        locals: &mut StampLocals,
    ) {
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
        locals.var_rho_dn13 = 0.0;

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
        locals.var_vth0_dn13 = 0.0;

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
        locals.var_dvth_dn13 = 0.0;

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
        locals.var_dvth0_dn13 = 0.0;

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
        locals.var_dvthsc_dn13 = 0.0;

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
        locals.var_pb20b_dn13 = 0.0;

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
        locals.var_dvthw_dn13 = 0.0;

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
        locals.var_alpha_dn13 = 0.0;

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
        locals.var_achi_dn13 = 0.0;

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
        locals.var_vgvt_dn13 = 0.0;

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
        locals.var_pslsat_dn13 = 0.0;

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
        locals.var_vdsats_dn13 = 0.0;

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
        locals.var_delta_dn13 = 0.0;

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
        locals.var_qb_dn13 = 0.0;

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
        locals.var_qbu_dn13 = 0.0;

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
        locals.var_qi_dn13 = 0.0;

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
        locals.var_qiu_dn13 = 0.0;

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
        locals.var_qd_dn13 = 0.0;

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
        locals.var_ids_dn13 = 0.0;

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
        locals.var_ids0_dn13 = 0.0;

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
        locals.var_dvthscsti_dn13 = 0.0;

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
        locals.var_vgssti_dn13 = 0.0;

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
        locals.var_costi0_dn13 = 0.0;

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
        locals.var_costi1_dn13 = 0.0;

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
        locals.var_costi3_dn13 = 0.0;

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
        locals.var_costi4_dn13 = 0.0;

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
        locals.var_costi5_dn13 = 0.0;

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
        locals.var_costi6_dn13 = 0.0;

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
        locals.var_costi7_dn13 = 0.0;

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
        locals.var_psasti_dn13 = 0.0;

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
        locals.var_psbsti_dn13 = 0.0;

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
        locals.var_psab_dn13 = 0.0;

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
        locals.var_psti_dn13 = 0.0;

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
        locals.var_sq1sti_dn13 = 0.0;

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
        locals.var_sq2sti_dn13 = 0.0;

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
        locals.var_qn0sti_dn13 = 0.0;

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
        locals.var_idssti_dn13 = 0.0;

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
        locals.var_beta_dn13 = 0.0;

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
        locals.var_beta_inv_dn13 = 0.0;

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
        locals.var_beta2_dn13 = 0.0;

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
        locals.var_pb2_dn13 = 0.0;

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
        locals.var_pb20_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_2(
        locals: &mut StampLocals,
    ) {
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
        locals.var_pb2c_dn13 = 0.0;

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
        locals.var_q_nsub_dn13 = 0.0;

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
        locals.var_psa_dn13 = 0.0;

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
        locals.var_psdl_dn13 = 0.0;

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
        locals.var_lred_dn13 = 0.0;

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
        locals.var_lch_dn13 = 0.0;

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
        locals.var_wd_dn13 = 0.0;

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
        locals.var_vthp_dn13 = 0.0;

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
        locals.var_dvthlp_dn13 = 0.0;

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
        locals.var_bs12_dn13 = 0.0;

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
        locals.var_qbmm_dn13 = 0.0;

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
        locals.var_dqb_dn13 = 0.0;

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
        locals.var_vdx_dn13 = 0.0;

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
        locals.var_vdx2_dn13 = 0.0;

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
        locals.var_pbsum_dn13 = 0.0;

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
        locals.var_sqrt_pbsum_dn13 = 0.0;

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
        locals.var_dppg_dn13 = 0.0;

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
        locals.var_dtox_dn13 = 0.0;

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
        locals.var_cox_dn13 = 0.0;

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
        locals.var_cox_inv_dn13 = 0.0;

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
        locals.var_vthq_dn13 = 0.0;

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
        locals.var_psdlz_dn13 = 0.0;

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
        locals.var_egp12_dn13 = 0.0;

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
        locals.var_egp32_dn13 = 0.0;

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
        locals.var_e1_dn13 = 0.0;

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
        locals.var_etun_dn13 = 0.0;

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
        locals.var_vdsp_dn13 = 0.0;

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
        locals.var_egidl_dn13 = 0.0;

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
        locals.var_egisl_dn13 = 0.0;

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
        locals.var_vdb_dn13 = 0.0;

        locals.var_vsb = 0.0;
        locals.var_vsb_dn5 = 0.0;
        locals.var_vsb_dn7 = 0.0;
        locals.var_vsb_dn8 = 0.0;

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
        locals.var_fd2_dn13 = 0.0;

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
        locals.var_fmdvds_dn13 = 0.0;

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
        locals.var_cnst0_dn13 = 0.0;

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
        locals.var_cnst1_dn13 = 0.0;

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
        locals.var_cnstcoxi_dn13 = 0.0;

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
        locals.var_fac1_dn13 = 0.0;

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
        locals.var_fac1p2_dn13 = 0.0;

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
        locals.var_fs01_dn13 = 0.0;

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
        locals.var_fs01_dps0_dn13 = 0.0;

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
        locals.var_fs02_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_3(
        locals: &mut StampLocals,
    ) {
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
        locals.var_fs02_dps0_dn13 = 0.0;

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
        locals.var_fsl1_dn13 = 0.0;

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
        locals.var_fsl1_dpsl_dn13 = 0.0;

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
        locals.var_fsl2_dn13 = 0.0;

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
        locals.var_fsl2_dpsl_dn13 = 0.0;

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
        locals.var_cfs1_dn13 = 0.0;

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
        locals.var_fb_dn13 = 0.0;

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
        locals.var_fb_dchi_dn13 = 0.0;

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
        locals.var_fi_dn13 = 0.0;

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
        locals.var_fi_dchi_dn13 = 0.0;

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
        locals.var_exp_chi_dn13 = 0.0;

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
        locals.var_exp_rho_dn13 = 0.0;

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
        locals.var_exp_bvbs_dn13 = 0.0;

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
        locals.var_exp_bvbsvds_dn13 = 0.0;

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
        locals.var_exp_bps0_dn13 = 0.0;

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
        locals.var_fs0_dn13 = 0.0;

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
        locals.var_fs0_dps0_dn13 = 0.0;

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
        locals.var_fsl_dn13 = 0.0;

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
        locals.var_fsl_dpsl_dn13 = 0.0;

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
        locals.var_dps0_dn13 = 0.0;

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
        locals.var_dpsl_dn13 = 0.0;

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
        locals.var_qn0_dn13 = 0.0;

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
        locals.var_qb0_dn13 = 0.0;

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
        locals.var_qbnm_dn13 = 0.0;

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
        locals.var_dtpds_dn13 = 0.0;

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
        locals.var_qinm_dn13 = 0.0;

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
        locals.var_qidn_dn13 = 0.0;

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
        locals.var_qdnm_dn13 = 0.0;

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
        locals.var_qddn_dn13 = 0.0;

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
        locals.var_quot_dn13 = 0.0;

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
        locals.var_qdrat_dn13 = 0.0;

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
        locals.var_idd_dn13 = 0.0;

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
        locals.var_idd1_dn13 = 0.0;

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
        locals.var_fdd_dn13 = 0.0;

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
        locals.var_eeff_dn13 = 0.0;

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
        locals.var_rns_dn13 = 0.0;

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
        locals.var_mu_dn13 = 0.0;

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
        locals.var_muun_dn13 = 0.0;

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
        locals.var_ey_dn13 = 0.0;

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
        locals.var_em_dn13 = 0.0;

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
        locals.var_eta_dn13 = 0.0;

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
        locals.var_eta1_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_4(
        locals: &mut StampLocals,
    ) {
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
        locals.var_eta1p12_dn13 = 0.0;

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
        locals.var_eta1p32_dn13 = 0.0;

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
        locals.var_eta1p52_dn13 = 0.0;

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
        locals.var_zeta12_dn13 = 0.0;

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
        locals.var_zeta32_dn13 = 0.0;

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
        locals.var_zeta52_dn13 = 0.0;

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
        locals.var_f00_dn13 = 0.0;

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
        locals.var_f10_dn13 = 0.0;

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
        locals.var_f30_dn13 = 0.0;

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
        locals.var_f11_dn13 = 0.0;

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
        locals.var_ps0_min_dn13 = 0.0;

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
        locals.var_acn_dn13 = 0.0;

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
        locals.var_acd_dn13 = 0.0;

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
        locals.var_ac1_dn13 = 0.0;

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
        locals.var_ac2_dn13 = 0.0;

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
        locals.var_ac3_dn13 = 0.0;

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
        locals.var_ac4_dn13 = 0.0;

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
        locals.var_ac31_dn13 = 0.0;

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
        locals.var_ac41_dn13 = 0.0;

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
        locals.var_isub_dn13 = 0.0;

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
        locals.var_isubld_dn13 = 0.0;

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
        locals.var_psislsat_dn13 = 0.0;

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
        locals.var_psisubsat_dn13 = 0.0;

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
        locals.var_eg12_dn13 = 0.0;

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
        locals.var_eg32_dn13 = 0.0;

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
        locals.var_qgos_dn13 = 0.0;

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
        locals.var_qgod_dn13 = 0.0;

        locals.var_qgbo = 0.0;
        locals.var_qgbo_dn6 = 0.0;
        locals.var_qgbo_dn7 = 0.0;
        locals.var_qgbo_dn8 = 0.0;

        locals.var_cgbo_loc = 0.0;

        locals.var_qgso = 0.0;
        locals.var_qgso_dn2 = 0.0;
        locals.var_qgso_dn6 = 0.0;

        locals.var_qgdo = 0.0;
        locals.var_qgdo_dn0 = 0.0;
        locals.var_qgdo_dn2 = 0.0;
        locals.var_qgdo_dn6 = 0.0;

        locals.var_qfd = 0.0;
        locals.var_qfd_dn0 = 0.0;
        locals.var_qfd_dn2 = 0.0;
        locals.var_qfd_dn6 = 0.0;

        locals.var_cfd = 0.0;

        locals.var_qfs = 0.0;
        locals.var_qfs_dn2 = 0.0;
        locals.var_qfs_dn6 = 0.0;

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
        locals.var_ec_dn13 = 0.0;

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
        locals.var_pslk_dn13 = 0.0;

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
        locals.var_qy_dn13 = 0.0;

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
        locals.var_eyd_dn13 = 0.0;

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
        locals.var_mu_ave_dn13 = 0.0;

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
        locals.var_nthrml_dn13 = 0.0;

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
        locals.var_mud_hoso_dn13 = 0.0;

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
        locals.var_kusai00_dn13 = 0.0;

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
        locals.var_kusaidd_dn13 = 0.0;

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
        locals.var_kusail_dn13 = 0.0;

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
        locals.var_kusai00l_dn13 = 0.0;

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
        locals.var_sqrtkusail_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_5(
        locals: &mut StampLocals,
    ) {
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
        locals.var_kusai_ig_dn13 = 0.0;

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
        locals.var_gds0_ign_dn13 = 0.0;

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
        locals.var_gds0_h2_dn13 = 0.0;

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
        locals.var_gamma_dn13 = 0.0;

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
        locals.var_crl_f_dn13 = 0.0;

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
        locals.var_nign0_dn13 = 0.0;

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
        locals.var_mumoda_dn13 = 0.0;

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
        locals.var_mumodb_dn13 = 0.0;

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
        locals.var_correct_w1_dn13 = 0.0;

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
        locals.var_tx_dn13 = 0.0;

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
        locals.var_ty_dn13 = 0.0;

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
        locals.var_t0_dn13 = 0.0;

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
        locals.var_t1_dn13 = 0.0;

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
        locals.var_t2_dn13 = 0.0;

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
        locals.var_t3_dn13 = 0.0;

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
        locals.var_t4_dn13 = 0.0;

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
        locals.var_t5_dn13 = 0.0;

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
        locals.var_t6_dn13 = 0.0;

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
        locals.var_t7_dn13 = 0.0;

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
        locals.var_t8_dn13 = 0.0;

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
        locals.var_t9_dn13 = 0.0;

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
        locals.var_t10_dn13 = 0.0;

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
        locals.var_t11_dn13 = 0.0;

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
        locals.var_t12_dn13 = 0.0;

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
        locals.var_vdseff_dn13 = 0.0;

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
        locals.var_vdsorg_dn13 = 0.0;

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
        locals.var_qovdext_dn13 = 0.0;

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
        locals.var_qovsext_dn13 = 0.0;

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
        locals.var_qovd_dn13 = 0.0;

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
        locals.var_qovs_dn13 = 0.0;

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
        locals.var_qbuld_dn13 = 0.0;

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
        locals.var_qbdld_dn13 = 0.0;

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
        locals.var_qbsld_dn13 = 0.0;

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
        locals.var_qodad_dn13 = 0.0;

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
        locals.var_qbdldext_dn13 = 0.0;

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
        locals.var_qbsldext_dn13 = 0.0;

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
        locals.var_vbsz2_dn13 = 0.0;

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
        locals.var_rdrift_dn13 = 0.0;

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
        locals.var_rsdrift_dn13 = 0.0;

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
        locals.var_ra_dn13 = 0.0;

        locals.var_vdse_eff = 0.0;
        locals.var_vdse_eff_dn0 = 0.0;
        locals.var_vdse_eff_dn2 = 0.0;

        locals.var_vdsemodenml = 0.0;

        locals.var_vdsemodervs = 0.0;

        locals.var_vbsegmt = 0.0;
        locals.var_vbsegmt_dn2 = 0.0;
        locals.var_vbsegmt_dn8 = 0.0;

        locals.var_vdsegmt = 0.0;
        locals.var_vdsegmt_dn0 = 0.0;
        locals.var_vdsegmt_dn2 = 0.0;

        locals.var_vgsegmt = 0.0;
        locals.var_vgsegmt_dn2 = 0.0;
        locals.var_vgsegmt_dn6 = 0.0;

        locals.var_vbserev = 0.0;
        locals.var_vbserev_dn0 = 0.0;
        locals.var_vbserev_dn2 = 0.0;
        locals.var_vbserev_dn8 = 0.0;

        locals.var_vdserev = 0.0;
        locals.var_vdserev_dn0 = 0.0;
        locals.var_vdserev_dn2 = 0.0;

    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_vgserev = 0.0;
        locals.var_vgserev_dn0 = 0.0;
        locals.var_vgserev_dn2 = 0.0;
        locals.var_vgserev_dn6 = 0.0;

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
        locals.var_vdserevz_dn13 = 0.0;

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
        locals.var_vgserevz_dn13 = 0.0;

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
        locals.var_vbserevz_dn13 = 0.0;

        locals.var_vsubsrev = 0.0;
        locals.var_vsubsrev_dn0 = 0.0;
        locals.var_vsubsrev_dn2 = 0.0;

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
        locals.var_ttemp_dn13 = 0.0;

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
        locals.var_ttemp0_dn13 = 0.0;

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
        locals.var_tdiff0_dn13 = 0.0;

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
        locals.var_tdiff0_2_dn13 = 0.0;

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
        locals.var_tdiff_dn13 = 0.0;

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
        locals.var_tdiff_2_dn13 = 0.0;

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
        locals.var_eg_dn13 = 0.0;

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
        locals.var_nin_dn13 = 0.0;

        locals.var_vgbgmt = 0.0;
        locals.var_vgbgmt_dn2 = 0.0;
        locals.var_vgbgmt_dn6 = 0.0;
        locals.var_vgbgmt_dn7 = 0.0;
        locals.var_vgbgmt_dn8 = 0.0;

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
        locals.var_vxbgmt_dn13 = 0.0;

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
        locals.var_vxbgmtcl_dn13 = 0.0;

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
        locals.var_qsuld_dn13 = 0.0;

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
        locals.var_qiuld_dn13 = 0.0;

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
        locals.var_idsibpc_dn13 = 0.0;

        locals.var_vgpld = 0.0;
        locals.var_vgpld_dn2 = 0.0;
        locals.var_vgpld_dn6 = 0.0;
        locals.var_vgpld_dn7 = 0.0;
        locals.var_vgpld_dn8 = 0.0;

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
        locals.var_ps0ld_dn13 = 0.0;

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
        locals.var_cnst1over_dn13 = 0.0;

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
        locals.var_ddriftld_dn13 = 0.0;

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
        locals.var_ddriftldc_dn13 = 0.0;

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
        locals.var_cnst0over_func_dn13 = 0.0;

        locals.var_ta = 0.0093868;

        let assign3320_e1728: f64 = (-0.1047839);
        locals.var_tb = assign3320_e1728;

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
        locals.var_chi_1_dn13 = 0.0;

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
        locals.var_mueph_dn13 = 0.0;

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
        locals.var_nsubpp_dn13 = 0.0;

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
        locals.var_nsubps_dn13 = 0.0;

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
        locals.var_nsub_dn13 = 0.0;

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
        locals.var_nsubb_dn13 = 0.0;

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
        locals.var_lod_half_dn13 = 0.0;

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
        locals.var_lod_half_ref_dn13 = 0.0;

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
        locals.var_log_tratio_dn13 = 0.0;

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
        locals.var_edri_dn13 = 0.0;

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
        locals.var_vdri_dn13 = 0.0;

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
        locals.var_mu0_dn13 = 0.0;

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
        locals.var_xov_dn13 = 0.0;

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
        locals.var_carr_dn13 = 0.0;

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
        locals.var_gd_dn13 = 0.0;

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
        locals.var_vddpz_dn13 = 0.0;

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
        locals.var_arg_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_vbd = 0.0;
        locals.var_vbd_dn5 = 0.0;
        locals.var_vbd_dn7 = 0.0;
        locals.var_vbd_dn8 = 0.0;

        locals.var_vbsi = 0.0;
        locals.var_vbsi_dn7 = 0.0;
        locals.var_vbsi_dn8 = 0.0;

        locals.var_vdsi = 0.0;
        locals.var_vdsi_dn5 = 0.0;
        locals.var_vdsi_dn7 = 0.0;

        locals.var_vgd = 0.0;
        locals.var_vgd_dn5 = 0.0;
        locals.var_vgd_dn6 = 0.0;
        locals.var_vgd_dn7 = 0.0;

        locals.var_vgsi = 0.0;
        locals.var_vgsi_dn6 = 0.0;
        locals.var_vgsi_dn7 = 0.0;

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
        locals.var_deltemp_dn13 = 0.0;

        locals.var_vdsei = 0.0;
        locals.var_vdsei_dn0 = 0.0;
        locals.var_vdsei_dn2 = 0.0;

        locals.var_vgsei = 0.0;
        locals.var_vgsei_dn2 = 0.0;
        locals.var_vgsei_dn6 = 0.0;

        locals.var_vbsei = 0.0;
        locals.var_vbsei_dn2 = 0.0;
        locals.var_vbsei_dn8 = 0.0;

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
        locals.var_gth_dn13 = 0.0;

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
        locals.var_qg_dn13 = 0.0;

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
        locals.var_qs_dn13 = 0.0;

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
        locals.var_veffpower_dn13 = 0.0;

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
        locals.var_p_dn13 = 0.0;

        locals.var_qi_nqs = 0.0;
        locals.var_qi_nqs_dn11 = 0.0;

        locals.var_qb_nqs = 0.0;
        locals.var_qb_nqs_dn12 = 0.0;

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
        locals.var_qd_nqs_dn13 = 0.0;

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
        locals.var_qs_nqs_dn13 = 0.0;

        locals.var_qg_nqs = 0.0;
        locals.var_qg_nqs_dn11 = 0.0;
        locals.var_qg_nqs_dn12 = 0.0;

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
        locals.var_cgsb_dn13 = 0.0;

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
        locals.var_ninvde_dn13 = 0.0;

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
        locals.var_ninvdecres_dn13 = 0.0;

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
        locals.var_ninvdehres_dn13 = 0.0;

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
        locals.var_rrdrmue_dn13 = 0.0;

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
        locals.var_rrdrmues_dn13 = 0.0;

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
        locals.var_rrdrvmax_dn13 = 0.0;

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
        locals.var_rde_dn13 = 0.0;

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
        locals.var_rdvde_dn13 = 0.0;

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
        locals.var_rse_dn13 = 0.0;

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
        locals.var_rsvde_dn13 = 0.0;

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
        locals.var_rrdrvmaxs_dn13 = 0.0;

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
        locals.var_tratio_dn13 = 0.0;

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
        locals.var_vmaxeff_dn13 = 0.0;

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
        locals.var_cnst0over_dn13 = 0.0;

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
        locals.var_cnst0overs_dn13 = 0.0;

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
        locals.var_costi0_p2_dn13 = 0.0;

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
        locals.var_mphn0_dn13 = 0.0;

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
        locals.var_powratio_dn13 = 0.0;

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
        locals.var_ptovr_dn13 = 0.0;

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
        locals.var_sqrt_eg_dn13 = 0.0;

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
        locals.var_wdpl_dn13 = 0.0;

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
        locals.var_wdplp_dn13 = 0.0;

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
        locals.var_uc_rdrbb_dn13 = 0.0;

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
        locals.var_uc_rdrbb_s_dn13 = 0.0;

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
        locals.var_ids_acc_dn13 = 0.0;

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
        locals.var_ids_res_dn13 = 0.0;

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
        locals.var_ires_leak_dn13 = 0.0;

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
        locals.var_pb2n_dn13 = 0.0;

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
        locals.var_vbipn_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_8(
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
        locals.var_hbdceff_dn13 = 0.0;

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
        locals.var_depmphn0_dn13 = 0.0;

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
        locals.var_qiu_noi_dn13 = 0.0;

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
        locals.var_js_dn13 = 0.0;

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
        locals.var_jssw_dn13 = 0.0;

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
        locals.var_js2_dn13 = 0.0;

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
        locals.var_jssw2_dn13 = 0.0;

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
        locals.var_ibs_dn13 = 0.0;

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
        locals.var_ibd_dn13 = 0.0;

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
        locals.var_qbs_dn13 = 0.0;

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
        locals.var_qbd_dn13 = 0.0;
        locals.var_qbd_dn15 = 0.0;
        locals.var_qbd_dn16 = 0.0;
        locals.var_qbd_dn17 = 0.0;

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
        locals.var_qbsi_dn13 = 0.0;

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
        locals.var_qbdi_dn13 = 0.0;

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
        locals.var_czbd_dn13 = 0.0;

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
        locals.var_czbdsw_dn13 = 0.0;

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
        locals.var_czbdswg_dn13 = 0.0;

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
        locals.var_czbs_dn13 = 0.0;

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
        locals.var_czbssw_dn13 = 0.0;

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
        locals.var_czbsswg_dn13 = 0.0;

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
        locals.var_pzbd_dn13 = 0.0;

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
        locals.var_pzbdsw_dn13 = 0.0;

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
        locals.var_pzbdswg_dn13 = 0.0;

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
        locals.var_pzbs_dn13 = 0.0;

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
        locals.var_pzbssw_dn13 = 0.0;

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
        locals.var_pzbsswg_dn13 = 0.0;

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
        locals.var_sarg_dn13 = 0.0;

        locals.var_vsbs = 0.0;
        locals.var_vsbs_dn2 = 0.0;
        locals.var_vsbs_dn10 = 0.0;

        locals.var_vdbd = 0.0;
        locals.var_vdbd_dn0 = 0.0;
        locals.var_vdbd_dn9 = 0.0;

        locals.var_vbs_jct = 0.0;
        locals.var_vbs_jct_dn2 = 0.0;
        locals.var_vbs_jct_dn10 = 0.0;

        locals.var_vbd_jct = 0.0;
        locals.var_vbd_jct_dn0 = 0.0;
        locals.var_vbd_jct_dn9 = 0.0;

        locals.var_vbpsp = 0.0;
        locals.var_vbpsp_dn7 = 0.0;
        locals.var_vbpsp_dn8 = 0.0;

        locals.var_vbpdp = 0.0;
        locals.var_vbpdp_dn5 = 0.0;
        locals.var_vbpdp_dn8 = 0.0;

        locals.var_vbsi_jct = 0.0;
        locals.var_vbsi_jct_dn7 = 0.0;
        locals.var_vbsi_jct_dn8 = 0.0;

        locals.var_vbdi_jct = 0.0;
        locals.var_vbdi_jct_dn5 = 0.0;
        locals.var_vbdi_jct_dn8 = 0.0;

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
        locals.var_exptempd_dn13 = 0.0;

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
        locals.var_exptemps_dn13 = 0.0;

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
        locals.var_isbd_dn13 = 0.0;

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
        locals.var_isbs_dn13 = 0.0;

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
        locals.var_jd_expcd_dn13 = 0.0;

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
        locals.var_jd_expcs_dn13 = 0.0;

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
        locals.var_vbdt_dn13 = 0.0;

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
        locals.var_vbst_dn13 = 0.0;

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
        locals.var_jd_nvtm_invd_dn13 = 0.0;

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
        locals.var_jd_nvtm_invs_dn13 = 0.0;

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
        locals.var_qbd_qs_dn13 = 0.0;

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
        locals.var_isbd_btm_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_isbd2_btm_dn13 = 0.0;

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
        locals.var_isbd_sws_dn13 = 0.0;

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
        locals.var_isbd2_sws_dn13 = 0.0;

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
        locals.var_isbd_swg_dn13 = 0.0;

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
        locals.var_isbd2_swg_dn13 = 0.0;

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
        locals.var_isbs_btm_dn13 = 0.0;

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
        locals.var_isbs2_btm_dn13 = 0.0;

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
        locals.var_isbs_sws_dn13 = 0.0;

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
        locals.var_isbs2_sws_dn13 = 0.0;

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
        locals.var_isbs_swg_dn13 = 0.0;

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
        locals.var_isbs2_swg_dn13 = 0.0;

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
        locals.var_qovd_add_dn13 = 0.0;

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
        locals.var_qovs_add_dn13 = 0.0;

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
        locals.var_qbdld_add_dn13 = 0.0;

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
        locals.var_qbsld_add_dn13 = 0.0;

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
        locals.var_wjuncld_dn13 = 0.0;

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
        locals.var_idspt0_dn13 = 0.0;

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
        locals.var_idspt1_dn13 = 0.0;

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
        locals.var_iwnqs0_a_dn13 = 0.0;
        locals.var_iwnqs0_a_dn17 = 0.0;

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
        locals.var_inqs0_a_dn13 = 0.0;
        locals.var_inqs0_a_dn15 = 0.0;

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
        locals.var_inqs0_k_dn13 = 0.0;
        locals.var_inqs0_k_dn16 = 0.0;

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
        locals.var_isubibpc_dn13 = 0.0;

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
        locals.var_lover_func_dn13 = 0.0;

        locals.var_q_nqs_a = 0.0;
        locals.var_q_nqs_a_dn15 = 0.0;

        locals.var_q_nqs_k = 0.0;
        locals.var_q_nqs_k_dn16 = 0.0;

        locals.var_w_nqs_a = 0.0;
        locals.var_w_nqs_a_dn17 = 0.0;

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
        locals.var_w_res_dn13 = 0.0;

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
        locals.var_wdep_func_dn13 = 0.0;

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
        locals.var_wk_ii_dn13 = 0.0;

        let (assign5320_e1936,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (p.p17,)
    }
};
        locals.var_uc_corsrd = assign5320_e1936;

        locals.var_uc_xpdv = p.p104;

        locals.var_uc_xldld = p.p294;

        locals.var_uc_scp22 = p.p222;

        locals.var_uc_rdrcx = p.p420;

        locals.var_mfactor = 1.0;

        let assign5480_e1979: f64 = if locals.var_uc_scp22 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard8 = assign5480_e1979;

        let (assign5490_e1983,) = {
    if (locals.var_guard8 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5490_e1983;

        let assign5500_e1986: f64 = if locals.var_uc_scp22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard9 = assign5500_e1986;

        let (assign5510_e1990,) = {
    if (locals.var_guard9 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5510_e1990;

        let assign5530_e1998: f64 = if locals.var_uc_xldld < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard11 = assign5530_e1998;

        let (assign5540_e2002,) = {
    if (locals.var_guard11 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_xldld,)
    }
};
        locals.var_uc_xldld = assign5540_e2002;

        let assign5570_e2015: f64 = if locals.var_uc_rdrcx < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard14 = assign5570_e2015;

        let (assign5580_e2019,) = {
    if (locals.var_guard14 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5580_e2019;

        let assign5590_e2022: f64 = if locals.var_uc_rdrcx > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign5590_e2022;

        let (assign5600_e2026,) = {
    if (locals.var_guard15 != 0.0) {
        (1.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5600_e2026;

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
        locals.var_uc_ndepm_dn13 = 0.0;

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
        locals.var_uc_depthn_dn13 = 0.0;

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
        locals.var_uc_depmueback0_dn13 = 0.0;

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
        locals.var_uc_depmueback1_dn13 = 0.0;

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
        locals.var_uc_depmue0_dn13 = 0.0;

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
        locals.var_uc_depmue1_dn13 = 0.0;

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
        locals.var_uc_depmue2_dn13 = 0.0;

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
        locals.var_uc_depleak_dn13 = 0.0;

    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_uc_depvmax_dn13 = 0.0;

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
        locals.var_uc_depwlp_dn13 = 0.0;

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
        locals.var_uc_depvdsef1_dn13 = 0.0;

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
        locals.var_uc_depvdsef2_dn13 = 0.0;

        let assign6690_e2699: f64 = if ((locals.var_uc_codep < 3.0) && (locals.var_uc_codep > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard110 = assign6690_e2699;

        let assign6720_e2712: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard113 = assign6720_e2712;

        let (assign6730_e2718, assign6730_e2718_d_n0, assign6730_e2718_d_n2, assign6730_e2718_d_n4, assign6730_e2718_d_n5, assign6730_e2718_d_n6, assign6730_e2718_d_n7, assign6730_e2718_d_n8, assign6730_e2718_d_n9, assign6730_e2718_d_n10, assign6730_e2718_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard113 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign6730_e2718;
        locals.var_uc_ndepm_dn0 = assign6730_e2718_d_n0;
        locals.var_uc_ndepm_dn2 = assign6730_e2718_d_n2;
        locals.var_uc_ndepm_dn4 = assign6730_e2718_d_n4;
        locals.var_uc_ndepm_dn5 = assign6730_e2718_d_n5;
        locals.var_uc_ndepm_dn6 = assign6730_e2718_d_n6;
        locals.var_uc_ndepm_dn7 = assign6730_e2718_d_n7;
        locals.var_uc_ndepm_dn8 = assign6730_e2718_d_n8;
        locals.var_uc_ndepm_dn9 = assign6730_e2718_d_n9;
        locals.var_uc_ndepm_dn10 = assign6730_e2718_d_n10;
        locals.var_uc_ndepm_dn13 = assign6730_e2718_d_n13;

        let assign6740_e2721: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard114 = assign6740_e2721;

        let (assign6750_e2727, assign6750_e2727_d_n0, assign6750_e2727_d_n2, assign6750_e2727_d_n4, assign6750_e2727_d_n5, assign6750_e2727_d_n6, assign6750_e2727_d_n7, assign6750_e2727_d_n8, assign6750_e2727_d_n9, assign6750_e2727_d_n10, assign6750_e2727_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard114 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign6750_e2727;
        locals.var_uc_ndepm_dn0 = assign6750_e2727_d_n0;
        locals.var_uc_ndepm_dn2 = assign6750_e2727_d_n2;
        locals.var_uc_ndepm_dn4 = assign6750_e2727_d_n4;
        locals.var_uc_ndepm_dn5 = assign6750_e2727_d_n5;
        locals.var_uc_ndepm_dn6 = assign6750_e2727_d_n6;
        locals.var_uc_ndepm_dn7 = assign6750_e2727_d_n7;
        locals.var_uc_ndepm_dn8 = assign6750_e2727_d_n8;
        locals.var_uc_ndepm_dn9 = assign6750_e2727_d_n9;
        locals.var_uc_ndepm_dn10 = assign6750_e2727_d_n10;
        locals.var_uc_ndepm_dn13 = assign6750_e2727_d_n13;

        let assign6780_e2740: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign6780_e2740;

        let (assign6790_e2746, assign6790_e2746_d_n0, assign6790_e2746_d_n2, assign6790_e2746_d_n4, assign6790_e2746_d_n5, assign6790_e2746_d_n6, assign6790_e2746_d_n7, assign6790_e2746_d_n8, assign6790_e2746_d_n9, assign6790_e2746_d_n10, assign6790_e2746_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard117 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    }
};
        locals.var_uc_depthn = assign6790_e2746;
        locals.var_uc_depthn_dn0 = assign6790_e2746_d_n0;
        locals.var_uc_depthn_dn2 = assign6790_e2746_d_n2;
        locals.var_uc_depthn_dn4 = assign6790_e2746_d_n4;
        locals.var_uc_depthn_dn5 = assign6790_e2746_d_n5;
        locals.var_uc_depthn_dn6 = assign6790_e2746_d_n6;
        locals.var_uc_depthn_dn7 = assign6790_e2746_d_n7;
        locals.var_uc_depthn_dn8 = assign6790_e2746_d_n8;
        locals.var_uc_depthn_dn9 = assign6790_e2746_d_n9;
        locals.var_uc_depthn_dn10 = assign6790_e2746_d_n10;
        locals.var_uc_depthn_dn13 = assign6790_e2746_d_n13;

        let assign6800_e2749: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign6800_e2749;

        let (assign6810_e2755, assign6810_e2755_d_n0, assign6810_e2755_d_n2, assign6810_e2755_d_n4, assign6810_e2755_d_n5, assign6810_e2755_d_n6, assign6810_e2755_d_n7, assign6810_e2755_d_n8, assign6810_e2755_d_n9, assign6810_e2755_d_n10, assign6810_e2755_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard118 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    }
};
        locals.var_uc_depthn = assign6810_e2755;
        locals.var_uc_depthn_dn0 = assign6810_e2755_d_n0;
        locals.var_uc_depthn_dn2 = assign6810_e2755_d_n2;
        locals.var_uc_depthn_dn4 = assign6810_e2755_d_n4;
        locals.var_uc_depthn_dn5 = assign6810_e2755_d_n5;
        locals.var_uc_depthn_dn6 = assign6810_e2755_d_n6;
        locals.var_uc_depthn_dn7 = assign6810_e2755_d_n7;
        locals.var_uc_depthn_dn8 = assign6810_e2755_d_n8;
        locals.var_uc_depthn_dn9 = assign6810_e2755_d_n9;
        locals.var_uc_depthn_dn10 = assign6810_e2755_d_n10;
        locals.var_uc_depthn_dn13 = assign6810_e2755_d_n13;

        let assign6840_e2768: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign6840_e2768;

        let (assign6850_e2774, assign6850_e2774_d_n0, assign6850_e2774_d_n2, assign6850_e2774_d_n4, assign6850_e2774_d_n5, assign6850_e2774_d_n6, assign6850_e2774_d_n7, assign6850_e2774_d_n8, assign6850_e2774_d_n9, assign6850_e2774_d_n10, assign6850_e2774_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard121 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign6850_e2774;
        locals.var_uc_depmue0_dn0 = assign6850_e2774_d_n0;
        locals.var_uc_depmue0_dn2 = assign6850_e2774_d_n2;
        locals.var_uc_depmue0_dn4 = assign6850_e2774_d_n4;
        locals.var_uc_depmue0_dn5 = assign6850_e2774_d_n5;
        locals.var_uc_depmue0_dn6 = assign6850_e2774_d_n6;
        locals.var_uc_depmue0_dn7 = assign6850_e2774_d_n7;
        locals.var_uc_depmue0_dn8 = assign6850_e2774_d_n8;
        locals.var_uc_depmue0_dn9 = assign6850_e2774_d_n9;
        locals.var_uc_depmue0_dn10 = assign6850_e2774_d_n10;
        locals.var_uc_depmue0_dn13 = assign6850_e2774_d_n13;

        let assign6860_e2777: f64 = if locals.var_uc_depmue0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign6860_e2777;

        let (assign6870_e2783, assign6870_e2783_d_n0, assign6870_e2783_d_n2, assign6870_e2783_d_n4, assign6870_e2783_d_n5, assign6870_e2783_d_n6, assign6870_e2783_d_n7, assign6870_e2783_d_n8, assign6870_e2783_d_n9, assign6870_e2783_d_n10, assign6870_e2783_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard122 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign6870_e2783;
        locals.var_uc_depmue0_dn0 = assign6870_e2783_d_n0;
        locals.var_uc_depmue0_dn2 = assign6870_e2783_d_n2;
        locals.var_uc_depmue0_dn4 = assign6870_e2783_d_n4;
        locals.var_uc_depmue0_dn5 = assign6870_e2783_d_n5;
        locals.var_uc_depmue0_dn6 = assign6870_e2783_d_n6;
        locals.var_uc_depmue0_dn7 = assign6870_e2783_d_n7;
        locals.var_uc_depmue0_dn8 = assign6870_e2783_d_n8;
        locals.var_uc_depmue0_dn9 = assign6870_e2783_d_n9;
        locals.var_uc_depmue0_dn10 = assign6870_e2783_d_n10;
        locals.var_uc_depmue0_dn13 = assign6870_e2783_d_n13;

        let assign6900_e2796: f64 = if locals.var_uc_depmueback0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign6900_e2796;

        let (assign6910_e2802, assign6910_e2802_d_n0, assign6910_e2802_d_n2, assign6910_e2802_d_n4, assign6910_e2802_d_n5, assign6910_e2802_d_n6, assign6910_e2802_d_n7, assign6910_e2802_d_n8, assign6910_e2802_d_n9, assign6910_e2802_d_n10, assign6910_e2802_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard125 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign6910_e2802;
        locals.var_uc_depmueback0_dn0 = assign6910_e2802_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6910_e2802_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6910_e2802_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6910_e2802_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6910_e2802_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6910_e2802_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6910_e2802_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6910_e2802_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6910_e2802_d_n10;
        locals.var_uc_depmueback0_dn13 = assign6910_e2802_d_n13;

        let assign6920_e2805: f64 = if locals.var_uc_depmueback0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6920_e2805;

        let (assign6930_e2811, assign6930_e2811_d_n0, assign6930_e2811_d_n2, assign6930_e2811_d_n4, assign6930_e2811_d_n5, assign6930_e2811_d_n6, assign6930_e2811_d_n7, assign6930_e2811_d_n8, assign6930_e2811_d_n9, assign6930_e2811_d_n10, assign6930_e2811_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard126 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign6930_e2811;
        locals.var_uc_depmueback0_dn0 = assign6930_e2811_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6930_e2811_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6930_e2811_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6930_e2811_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6930_e2811_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6930_e2811_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6930_e2811_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6930_e2811_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6930_e2811_d_n10;
        locals.var_uc_depmueback0_dn13 = assign6930_e2811_d_n13;

        let assign6960_e2824: f64 = if locals.var_uc_depmueph1 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign6960_e2824;

        let (assign6970_e2830,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard129 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign6970_e2830;

        let assign6980_e2833: f64 = if locals.var_uc_depmueph1 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign6980_e2833;

        let (assign6990_e2839,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard130 != 0.0)) {
        (100000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign6990_e2839;

        let assign7020_e2852: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign7020_e2852;

        let (assign7030_e2858, assign7030_e2858_d_n0, assign7030_e2858_d_n2, assign7030_e2858_d_n4, assign7030_e2858_d_n5, assign7030_e2858_d_n6, assign7030_e2858_d_n7, assign7030_e2858_d_n8, assign7030_e2858_d_n9, assign7030_e2858_d_n10, assign7030_e2858_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard133 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign7030_e2858;
        locals.var_uc_depvdsef2_dn0 = assign7030_e2858_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7030_e2858_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7030_e2858_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7030_e2858_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7030_e2858_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7030_e2858_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7030_e2858_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7030_e2858_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7030_e2858_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign7030_e2858_d_n13;

        let assign7040_e2861: f64 = if locals.var_uc_depvdsef2 > 4.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign7040_e2861;

        let (assign7050_e2867, assign7050_e2867_d_n0, assign7050_e2867_d_n2, assign7050_e2867_d_n4, assign7050_e2867_d_n5, assign7050_e2867_d_n6, assign7050_e2867_d_n7, assign7050_e2867_d_n8, assign7050_e2867_d_n9, assign7050_e2867_d_n10, assign7050_e2867_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard134 != 0.0)) {
        (4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign7050_e2867;
        locals.var_uc_depvdsef2_dn0 = assign7050_e2867_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7050_e2867_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7050_e2867_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7050_e2867_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7050_e2867_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7050_e2867_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7050_e2867_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7050_e2867_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7050_e2867_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign7050_e2867_d_n13;

        let assign7080_e2880: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign7080_e2880;

        let (assign7090_e2886, assign7090_e2886_d_n0, assign7090_e2886_d_n2, assign7090_e2886_d_n4, assign7090_e2886_d_n5, assign7090_e2886_d_n6, assign7090_e2886_d_n7, assign7090_e2886_d_n8, assign7090_e2886_d_n9, assign7090_e2886_d_n10, assign7090_e2886_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard137 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign7090_e2886;
        locals.var_uc_depleak_dn0 = assign7090_e2886_d_n0;
        locals.var_uc_depleak_dn2 = assign7090_e2886_d_n2;
        locals.var_uc_depleak_dn4 = assign7090_e2886_d_n4;
        locals.var_uc_depleak_dn5 = assign7090_e2886_d_n5;
        locals.var_uc_depleak_dn6 = assign7090_e2886_d_n6;
        locals.var_uc_depleak_dn7 = assign7090_e2886_d_n7;
        locals.var_uc_depleak_dn8 = assign7090_e2886_d_n8;
        locals.var_uc_depleak_dn9 = assign7090_e2886_d_n9;
        locals.var_uc_depleak_dn10 = assign7090_e2886_d_n10;
        locals.var_uc_depleak_dn13 = assign7090_e2886_d_n13;

        let assign7100_e2889: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign7100_e2889;

        let (assign7110_e2895, assign7110_e2895_d_n0, assign7110_e2895_d_n2, assign7110_e2895_d_n4, assign7110_e2895_d_n5, assign7110_e2895_d_n6, assign7110_e2895_d_n7, assign7110_e2895_d_n8, assign7110_e2895_d_n9, assign7110_e2895_d_n10, assign7110_e2895_d_n13,) = {
    if ((locals.var_guard110 != 0.0) && (locals.var_guard138 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign7110_e2895;
        locals.var_uc_depleak_dn0 = assign7110_e2895_d_n0;
        locals.var_uc_depleak_dn2 = assign7110_e2895_d_n2;
        locals.var_uc_depleak_dn4 = assign7110_e2895_d_n4;
        locals.var_uc_depleak_dn5 = assign7110_e2895_d_n5;
        locals.var_uc_depleak_dn6 = assign7110_e2895_d_n6;
        locals.var_uc_depleak_dn7 = assign7110_e2895_d_n7;
        locals.var_uc_depleak_dn8 = assign7110_e2895_d_n8;
        locals.var_uc_depleak_dn9 = assign7110_e2895_d_n9;
        locals.var_uc_depleak_dn10 = assign7110_e2895_d_n10;
        locals.var_uc_depleak_dn13 = assign7110_e2895_d_n13;

        let assign7120_e2898: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard139 = assign7120_e2898;

        let assign7150_e2911: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7150_e2911;

        let (assign7160_e2920, assign7160_e2920_d_n0, assign7160_e2920_d_n2, assign7160_e2920_d_n4, assign7160_e2920_d_n5, assign7160_e2920_d_n6, assign7160_e2920_d_n7, assign7160_e2920_d_n8, assign7160_e2920_d_n9, assign7160_e2920_d_n10, assign7160_e2920_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard142 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign7160_e2920;
        locals.var_uc_ndepm_dn0 = assign7160_e2920_d_n0;
        locals.var_uc_ndepm_dn2 = assign7160_e2920_d_n2;
        locals.var_uc_ndepm_dn4 = assign7160_e2920_d_n4;
        locals.var_uc_ndepm_dn5 = assign7160_e2920_d_n5;
        locals.var_uc_ndepm_dn6 = assign7160_e2920_d_n6;
        locals.var_uc_ndepm_dn7 = assign7160_e2920_d_n7;
        locals.var_uc_ndepm_dn8 = assign7160_e2920_d_n8;
        locals.var_uc_ndepm_dn9 = assign7160_e2920_d_n9;
        locals.var_uc_ndepm_dn10 = assign7160_e2920_d_n10;
        locals.var_uc_ndepm_dn13 = assign7160_e2920_d_n13;

        let assign7170_e2923: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7170_e2923;

        let (assign7180_e2932, assign7180_e2932_d_n0, assign7180_e2932_d_n2, assign7180_e2932_d_n4, assign7180_e2932_d_n5, assign7180_e2932_d_n6, assign7180_e2932_d_n7, assign7180_e2932_d_n8, assign7180_e2932_d_n9, assign7180_e2932_d_n10, assign7180_e2932_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard143 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign7180_e2932;
        locals.var_uc_ndepm_dn0 = assign7180_e2932_d_n0;
        locals.var_uc_ndepm_dn2 = assign7180_e2932_d_n2;
        locals.var_uc_ndepm_dn4 = assign7180_e2932_d_n4;
        locals.var_uc_ndepm_dn5 = assign7180_e2932_d_n5;
        locals.var_uc_ndepm_dn6 = assign7180_e2932_d_n6;
        locals.var_uc_ndepm_dn7 = assign7180_e2932_d_n7;
        locals.var_uc_ndepm_dn8 = assign7180_e2932_d_n8;
        locals.var_uc_ndepm_dn9 = assign7180_e2932_d_n9;
        locals.var_uc_ndepm_dn10 = assign7180_e2932_d_n10;
        locals.var_uc_ndepm_dn13 = assign7180_e2932_d_n13;

        let assign7210_e2945: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7210_e2945;

        let (assign7220_e2954, assign7220_e2954_d_n0, assign7220_e2954_d_n2, assign7220_e2954_d_n4, assign7220_e2954_d_n5, assign7220_e2954_d_n6, assign7220_e2954_d_n7, assign7220_e2954_d_n8, assign7220_e2954_d_n9, assign7220_e2954_d_n10, assign7220_e2954_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard146 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    }
};
        locals.var_uc_depthn = assign7220_e2954;
        locals.var_uc_depthn_dn0 = assign7220_e2954_d_n0;
        locals.var_uc_depthn_dn2 = assign7220_e2954_d_n2;
        locals.var_uc_depthn_dn4 = assign7220_e2954_d_n4;
        locals.var_uc_depthn_dn5 = assign7220_e2954_d_n5;
        locals.var_uc_depthn_dn6 = assign7220_e2954_d_n6;
        locals.var_uc_depthn_dn7 = assign7220_e2954_d_n7;
        locals.var_uc_depthn_dn8 = assign7220_e2954_d_n8;
        locals.var_uc_depthn_dn9 = assign7220_e2954_d_n9;
        locals.var_uc_depthn_dn10 = assign7220_e2954_d_n10;
        locals.var_uc_depthn_dn13 = assign7220_e2954_d_n13;

        let assign7230_e2957: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7230_e2957;

        let (assign7240_e2966, assign7240_e2966_d_n0, assign7240_e2966_d_n2, assign7240_e2966_d_n4, assign7240_e2966_d_n5, assign7240_e2966_d_n6, assign7240_e2966_d_n7, assign7240_e2966_d_n8, assign7240_e2966_d_n9, assign7240_e2966_d_n10, assign7240_e2966_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard147 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn13,)
    }
};
        locals.var_uc_depthn = assign7240_e2966;
        locals.var_uc_depthn_dn0 = assign7240_e2966_d_n0;
        locals.var_uc_depthn_dn2 = assign7240_e2966_d_n2;
        locals.var_uc_depthn_dn4 = assign7240_e2966_d_n4;
        locals.var_uc_depthn_dn5 = assign7240_e2966_d_n5;
        locals.var_uc_depthn_dn6 = assign7240_e2966_d_n6;
        locals.var_uc_depthn_dn7 = assign7240_e2966_d_n7;
        locals.var_uc_depthn_dn8 = assign7240_e2966_d_n8;
        locals.var_uc_depthn_dn9 = assign7240_e2966_d_n9;
        locals.var_uc_depthn_dn10 = assign7240_e2966_d_n10;
        locals.var_uc_depthn_dn13 = assign7240_e2966_d_n13;

        let assign7270_e2979: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7270_e2979;

        let (assign7280_e2988, assign7280_e2988_d_n0, assign7280_e2988_d_n2, assign7280_e2988_d_n4, assign7280_e2988_d_n5, assign7280_e2988_d_n6, assign7280_e2988_d_n7, assign7280_e2988_d_n8, assign7280_e2988_d_n9, assign7280_e2988_d_n10, assign7280_e2988_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard150 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign7280_e2988;
        locals.var_uc_depmue0_dn0 = assign7280_e2988_d_n0;
        locals.var_uc_depmue0_dn2 = assign7280_e2988_d_n2;
        locals.var_uc_depmue0_dn4 = assign7280_e2988_d_n4;
        locals.var_uc_depmue0_dn5 = assign7280_e2988_d_n5;
        locals.var_uc_depmue0_dn6 = assign7280_e2988_d_n6;
        locals.var_uc_depmue0_dn7 = assign7280_e2988_d_n7;
        locals.var_uc_depmue0_dn8 = assign7280_e2988_d_n8;
        locals.var_uc_depmue0_dn9 = assign7280_e2988_d_n9;
        locals.var_uc_depmue0_dn10 = assign7280_e2988_d_n10;
        locals.var_uc_depmue0_dn13 = assign7280_e2988_d_n13;

        let assign7290_e2991: f64 = if locals.var_uc_depmue0 > 10000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign7290_e2991;

        let (assign7300_e3000, assign7300_e3000_d_n0, assign7300_e3000_d_n2, assign7300_e3000_d_n4, assign7300_e3000_d_n5, assign7300_e3000_d_n6, assign7300_e3000_d_n7, assign7300_e3000_d_n8, assign7300_e3000_d_n9, assign7300_e3000_d_n10, assign7300_e3000_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard151 != 0.0)) {
        (10000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign7300_e3000;
        locals.var_uc_depmue0_dn0 = assign7300_e3000_d_n0;
        locals.var_uc_depmue0_dn2 = assign7300_e3000_d_n2;
        locals.var_uc_depmue0_dn4 = assign7300_e3000_d_n4;
        locals.var_uc_depmue0_dn5 = assign7300_e3000_d_n5;
        locals.var_uc_depmue0_dn6 = assign7300_e3000_d_n6;
        locals.var_uc_depmue0_dn7 = assign7300_e3000_d_n7;
        locals.var_uc_depmue0_dn8 = assign7300_e3000_d_n8;
        locals.var_uc_depmue0_dn9 = assign7300_e3000_d_n9;
        locals.var_uc_depmue0_dn10 = assign7300_e3000_d_n10;
        locals.var_uc_depmue0_dn13 = assign7300_e3000_d_n13;

        let assign7330_e3013: f64 = if locals.var_uc_depmueph1 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign7330_e3013;

        let (assign7340_e3022,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard154 != 0.0)) {
        (100.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7340_e3022;

        let assign7350_e3025: f64 = if locals.var_uc_depmueph1 > 2000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign7350_e3025;

        let (assign7360_e3034,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard155 != 0.0)) {
        (2000000000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7360_e3034;

        let assign7390_e3047: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign7390_e3047;

    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7400_e3056, assign7400_e3056_d_n0, assign7400_e3056_d_n2, assign7400_e3056_d_n4, assign7400_e3056_d_n5, assign7400_e3056_d_n6, assign7400_e3056_d_n7, assign7400_e3056_d_n8, assign7400_e3056_d_n9, assign7400_e3056_d_n10, assign7400_e3056_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard158 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign7400_e3056;
        locals.var_uc_depleak_dn0 = assign7400_e3056_d_n0;
        locals.var_uc_depleak_dn2 = assign7400_e3056_d_n2;
        locals.var_uc_depleak_dn4 = assign7400_e3056_d_n4;
        locals.var_uc_depleak_dn5 = assign7400_e3056_d_n5;
        locals.var_uc_depleak_dn6 = assign7400_e3056_d_n6;
        locals.var_uc_depleak_dn7 = assign7400_e3056_d_n7;
        locals.var_uc_depleak_dn8 = assign7400_e3056_d_n8;
        locals.var_uc_depleak_dn9 = assign7400_e3056_d_n9;
        locals.var_uc_depleak_dn10 = assign7400_e3056_d_n10;
        locals.var_uc_depleak_dn13 = assign7400_e3056_d_n13;

        let assign7410_e3059: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign7410_e3059;

        let (assign7420_e3068, assign7420_e3068_d_n0, assign7420_e3068_d_n2, assign7420_e3068_d_n4, assign7420_e3068_d_n5, assign7420_e3068_d_n6, assign7420_e3068_d_n7, assign7420_e3068_d_n8, assign7420_e3068_d_n9, assign7420_e3068_d_n10, assign7420_e3068_d_n13,) = {
    if (((locals.var_guard110 == 0.0) && (locals.var_guard139 != 0.0)) && (locals.var_guard159 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign7420_e3068;
        locals.var_uc_depleak_dn0 = assign7420_e3068_d_n0;
        locals.var_uc_depleak_dn2 = assign7420_e3068_d_n2;
        locals.var_uc_depleak_dn4 = assign7420_e3068_d_n4;
        locals.var_uc_depleak_dn5 = assign7420_e3068_d_n5;
        locals.var_uc_depleak_dn6 = assign7420_e3068_d_n6;
        locals.var_uc_depleak_dn7 = assign7420_e3068_d_n7;
        locals.var_uc_depleak_dn8 = assign7420_e3068_d_n8;
        locals.var_uc_depleak_dn9 = assign7420_e3068_d_n9;
        locals.var_uc_depleak_dn10 = assign7420_e3068_d_n10;
        locals.var_uc_depleak_dn13 = assign7420_e3068_d_n13;

        locals.var_uc_toxb = p.p96;

        let assign7520_e3106: f64 = if locals.var_uc_toxb < p.p95 { 1.0 } else { 0.0 };
        locals.var_guard168 = assign7520_e3106;

        let (assign7530_e3110,) = {
    if (locals.var_guard168 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7530_e3110;

        let assign7540_e3113: f64 = if locals.var_uc_toxb > 5e-7 { 1.0 } else { 0.0 };
        locals.var_guard169 = assign7540_e3113;

        let (assign7550_e3117,) = {
    if (locals.var_guard169 != 0.0) {
        (5e-7,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7550_e3117;

        let assign7560_e3121: f64 = (100.0_f64).powf(p.p122);
        let assign7560_e3122: f64 = (p.p120 / assign7560_e3121);
        locals.var_mks_ll = assign7560_e3122;

        let assign7570_e3126: f64 = (100.0_f64).powf(p.p129);
        let assign7570_e3127: f64 = (p.p123 / assign7570_e3126);
        locals.var_mks_wl = assign7570_e3127;

        let assign7580_e3131: f64 = (100.0_f64).powf(p.p199);
        let assign7580_e3132: f64 = (p.p198 / assign7580_e3131);
        locals.var_mks_svgsl = assign7580_e3132;

        let assign7590_e3136: f64 = (100.0_f64).powf(p.p201);
        let assign7590_e3137: f64 = (p.p200 / assign7590_e3136);
        locals.var_mks_svgsw = assign7590_e3137;

        let assign7600_e3141: f64 = (100.0_f64).powf(p.p184);
        let assign7600_e3142: f64 = (p.p183 / assign7600_e3141);
        locals.var_mks_svbsl = assign7600_e3142;

        let assign7610_e3146: f64 = (100.0_f64).powf(p.p203);
        let assign7610_e3147: f64 = (p.p202 / assign7610_e3146);
        locals.var_mks_slgl = assign7610_e3147;

        let assign7620_e3151: f64 = (100.0_f64).powf(p.p191);
        let assign7620_e3152: f64 = (p.p190 / assign7620_e3151);
        locals.var_mks_sub1l = assign7620_e3152;

        let assign7630_e3155: f64 = (p.p186 / 100.0);
        locals.var_mks_slg = assign7630_e3155;

        let assign7640_e3158: f64 = (p.p192 / 100.0);
        locals.var_mks_sub2l = assign7640_e3158;

        let assign7650_e3161: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign7650_e3161;

        let assign7660_e3164: f64 = (p.p311 / 100.0);
        locals.var_mks_rdtemp1 = assign7660_e3164;

        let assign7670_e3167: f64 = (p.p312 / 100.0);
        locals.var_mks_rdtemp2 = assign7670_e3167;

        let assign7680_e3170: f64 = (p.p313 / 100.0);
        locals.var_mks_rdvdtemp1 = assign7680_e3170;

        let assign7690_e3173: f64 = (p.p314 / 100.0);
        locals.var_mks_rdvdtemp2 = assign7690_e3173;

        let assign7700_e3176: f64 = (p.p336 / 1e-6);
        locals.var_mks_nsubsub = assign7700_e3176;

        let assign7710_e3179: f64 = (p.p255 * 100.0);
        locals.var_mks_glksd3 = assign7710_e3179;

        let assign7720_e3182: f64 = (p.p248 * 100.0);
        locals.var_mks_gleak4 = assign7720_e3182;

        let assign7730_e3185: f64 = (p.p249 * 100.0);
        locals.var_mks_gleak5 = assign7730_e3185;

        let assign7740_e3188: f64 = (p.p251 / 10000.0);
        locals.var_mks_gleak7 = assign7740_e3188;

        let assign7750_e3191: f64 = (p.p266 * 10000.0);
        locals.var_mks_cit = assign7750_e3191;

        let assign7760_e3194: f64 = (p.p275 / 100.0);
        locals.var_mks_ovslp = assign7760_e3194;

        let assign7770_e3197: f64 = (p.p272 / 10000.0);
        locals.var_mks_dly3 = assign7770_e3197;

        let assign7780_e3200: f64 = (p.p273 / 10000.0);
        locals.var_mks_dlyov = assign7780_e3200;
        locals.var_mks_dlyov_dn0 = 0.0;
        locals.var_mks_dlyov_dn2 = 0.0;
        locals.var_mks_dlyov_dn4 = 0.0;
        locals.var_mks_dlyov_dn5 = 0.0;
        locals.var_mks_dlyov_dn6 = 0.0;
        locals.var_mks_dlyov_dn7 = 0.0;
        locals.var_mks_dlyov_dn8 = 0.0;
        locals.var_mks_dlyov_dn9 = 0.0;
        locals.var_mks_dlyov_dn10 = 0.0;
        locals.var_mks_dlyov_dn13 = 0.0;

        let assign7800_e3206: f64 = (p.p409 / 10000.0);
        locals.var_mks_rdrmue = assign7800_e3206;

        let assign7810_e3209: f64 = (p.p412 / 100.0);
        locals.var_mks_rdrvmax = assign7810_e3209;

        let assign7820_e3212: f64 = (p.p413 / 10000.0);
        locals.var_mks_rdrmues = assign7820_e3212;

        let assign7830_e3215: f64 = (p.p414 / 100.0);
        locals.var_mks_rdrvmaxs = assign7830_e3215;

        let assign7840_e3218: f64 = (locals.var_uc_ndepm / 1e-6);
        locals.var_uc_ndepm = assign7840_e3218;
        locals.var_uc_ndepm_dn0 = (locals.var_uc_ndepm_dn0 / 1e-6);
        locals.var_uc_ndepm_dn2 = (locals.var_uc_ndepm_dn2 / 1e-6);
        locals.var_uc_ndepm_dn4 = (locals.var_uc_ndepm_dn4 / 1e-6);
        locals.var_uc_ndepm_dn5 = (locals.var_uc_ndepm_dn5 / 1e-6);
        locals.var_uc_ndepm_dn6 = (locals.var_uc_ndepm_dn6 / 1e-6);
        locals.var_uc_ndepm_dn7 = (locals.var_uc_ndepm_dn7 / 1e-6);
        locals.var_uc_ndepm_dn8 = (locals.var_uc_ndepm_dn8 / 1e-6);
        locals.var_uc_ndepm_dn9 = (locals.var_uc_ndepm_dn9 / 1e-6);
        locals.var_uc_ndepm_dn10 = (locals.var_uc_ndepm_dn10 / 1e-6);
        locals.var_uc_ndepm_dn13 = (locals.var_uc_ndepm_dn13 / 1e-6);

        let assign7850_e3221: f64 = (p.p453 / 1e-6);
        locals.var_uc_njunc = assign7850_e3221;

        let assign7860_e3224: f64 = (p.p274 + 273.15);
        locals.var_ktnom = assign7860_e3224;

        let assign7910_e3247: f64 = (p.p0 + p.p116);
        locals.var_lgate = assign7910_e3247;

        let assign7920_e3250: f64 = (p.p1 / p.p7);
        let assign7920_e3252: f64 = (assign7920_e3250 + p.p117);
        locals.var_wgate = assign7920_e3252;

        let assign8070_e3352: f64 = (locals.var_lgate * 1000000.0);
        locals.var_lg = assign8070_e3352;

        let assign8080_e3355: f64 = (locals.var_wgate * 1000000.0);
        locals.var_wg = assign8080_e3355;

        let assign8090_e3358: f64 = (locals.var_lg).powf(p.p553);
        locals.var_lbin = assign8090_e3358;

        let assign8100_e3361: f64 = (locals.var_wg).powf(p.p554);
        locals.var_wbin = assign8100_e3361;

        let assign8110_e3364: f64 = (locals.var_lbin * locals.var_wbin);
        locals.var_lwbin = assign8110_e3364;

        let assign8120_e3368: f64 = (p.p555 / locals.var_lbin);
        let assign8120_e3369: f64 = (p.p89 + assign8120_e3368);
        let assign8120_e3372: f64 = (p.p643 / locals.var_wbin);
        let assign8120_e3373: f64 = (assign8120_e3369 + assign8120_e3372);
        let assign8120_e3376: f64 = (p.p731 / locals.var_lwbin);
        let assign8120_e3377: f64 = (assign8120_e3373 + assign8120_e3376);
        locals.var_uc_vmax = assign8120_e3377;

        let assign8130_e3381: f64 = (p.p556 / locals.var_lbin);
        let assign8130_e3382: f64 = (p.p92 + assign8130_e3381);
        let assign8130_e3385: f64 = (p.p644 / locals.var_wbin);
        let assign8130_e3386: f64 = (assign8130_e3382 + assign8130_e3385);
        let assign8130_e3389: f64 = (p.p732 / locals.var_lwbin);
        let assign8130_e3390: f64 = (assign8130_e3386 + assign8130_e3389);
        locals.var_uc_bgtmp1 = assign8130_e3390;

        let assign8140_e3394: f64 = (p.p557 / locals.var_lbin);
        let assign8140_e3395: f64 = (p.p93 + assign8140_e3394);
        let assign8140_e3398: f64 = (p.p645 / locals.var_wbin);
        let assign8140_e3399: f64 = (assign8140_e3395 + assign8140_e3398);
        let assign8140_e3402: f64 = (p.p733 / locals.var_lwbin);
        let assign8140_e3403: f64 = (assign8140_e3399 + assign8140_e3402);
        locals.var_uc_bgtmp2 = assign8140_e3403;

        let assign8150_e3407: f64 = (p.p558 / locals.var_lbin);
        let assign8150_e3408: f64 = (p.p94 + assign8150_e3407);
        let assign8150_e3411: f64 = (p.p646 / locals.var_wbin);
        let assign8150_e3412: f64 = (assign8150_e3408 + assign8150_e3411);
        let assign8150_e3415: f64 = (p.p734 / locals.var_lwbin);
        let assign8150_e3416: f64 = (assign8150_e3412 + assign8150_e3415);
        locals.var_uc_eg0 = assign8150_e3416;

        let assign8160_e3420: f64 = (p.p559 / locals.var_lbin);
        let assign8160_e3421: f64 = (p.p110 + assign8160_e3420);
        let assign8160_e3424: f64 = (p.p647 / locals.var_wbin);
        let assign8160_e3425: f64 = (assign8160_e3421 + assign8160_e3424);
        let assign8160_e3428: f64 = (p.p735 / locals.var_lwbin);
        let assign8160_e3429: f64 = (assign8160_e3425 + assign8160_e3428);
        locals.var_uc_vfbover = assign8160_e3429;

        let assign8170_e3433: f64 = (p.p560 / locals.var_lbin);
        let assign8170_e3434: f64 = (p.p111 + assign8170_e3433);
        let assign8170_e3437: f64 = (p.p648 / locals.var_wbin);
        let assign8170_e3438: f64 = (assign8170_e3434 + assign8170_e3437);
        let assign8170_e3441: f64 = (p.p736 / locals.var_lwbin);
        let assign8170_e3442: f64 = (assign8170_e3438 + assign8170_e3441);
        locals.var_uc_nover = assign8170_e3442;

        let assign8180_e3446: f64 = (p.p561 / locals.var_lbin);
        let assign8180_e3447: f64 = (p.p112 + assign8180_e3446);
        let assign8180_e3450: f64 = (p.p649 / locals.var_wbin);
        let assign8180_e3451: f64 = (assign8180_e3447 + assign8180_e3450);
        let assign8180_e3454: f64 = (p.p737 / locals.var_lwbin);
        let assign8180_e3455: f64 = (assign8180_e3451 + assign8180_e3454);
        locals.var_uc_novers = assign8180_e3455;

        let assign8190_e3459: f64 = (p.p562 / locals.var_lbin);
        let assign8190_e3460: f64 = (p.p126 + assign8190_e3459);
        let assign8190_e3463: f64 = (p.p650 / locals.var_wbin);
        let assign8190_e3464: f64 = (assign8190_e3460 + assign8190_e3463);
        let assign8190_e3467: f64 = (p.p738 / locals.var_lwbin);
        let assign8190_e3468: f64 = (assign8190_e3464 + assign8190_e3467);
        locals.var_uc_wl2 = assign8190_e3468;

        let assign8200_e3472: f64 = (p.p563 / locals.var_lbin);
        let assign8200_e3473: f64 = (p.p136 + assign8200_e3472);
        let assign8200_e3476: f64 = (p.p651 / locals.var_wbin);
        let assign8200_e3477: f64 = (assign8200_e3473 + assign8200_e3476);
        let assign8200_e3480: f64 = (p.p739 / locals.var_lwbin);
        let assign8200_e3481: f64 = (assign8200_e3477 + assign8200_e3480);
        locals.var_uc_vfbc = assign8200_e3481;

        let assign8210_e3485: f64 = (p.p564 / locals.var_lbin);
        let assign8210_e3486: f64 = (p.p138 + assign8210_e3485);
        let assign8210_e3489: f64 = (p.p652 / locals.var_wbin);
        let assign8210_e3490: f64 = (assign8210_e3486 + assign8210_e3489);
        let assign8210_e3493: f64 = (p.p740 / locals.var_lwbin);
        let assign8210_e3494: f64 = (assign8210_e3490 + assign8210_e3493);
        locals.var_uc_nsubc = assign8210_e3494;

        let assign8220_e3498: f64 = (p.p565 / locals.var_lbin);
        let assign8220_e3499: f64 = (p.p141 + assign8220_e3498);
        let assign8220_e3502: f64 = (p.p653 / locals.var_wbin);
        let assign8220_e3503: f64 = (assign8220_e3499 + assign8220_e3502);
        let assign8220_e3506: f64 = (p.p741 / locals.var_lwbin);
        let assign8220_e3507: f64 = (assign8220_e3503 + assign8220_e3506);
        locals.var_uc_nsubp = assign8220_e3507;

        let assign8230_e3511: f64 = (p.p566 / locals.var_lbin);
        let assign8230_e3512: f64 = (p.p144 + assign8230_e3511);
        let assign8230_e3515: f64 = (p.p654 / locals.var_wbin);
        let assign8230_e3516: f64 = (assign8230_e3512 + assign8230_e3515);
        let assign8230_e3519: f64 = (p.p742 / locals.var_lwbin);
        let assign8230_e3520: f64 = (assign8230_e3516 + assign8230_e3519);
        locals.var_uc_scp1 = assign8230_e3520;

        let assign8240_e3524: f64 = (p.p567 / locals.var_lbin);
        let assign8240_e3525: f64 = (p.p145 + assign8240_e3524);
        let assign8240_e3528: f64 = (p.p655 / locals.var_wbin);
        let assign8240_e3529: f64 = (assign8240_e3525 + assign8240_e3528);
        let assign8240_e3532: f64 = (p.p743 / locals.var_lwbin);
        let assign8240_e3533: f64 = (assign8240_e3529 + assign8240_e3532);
        locals.var_uc_scp2 = assign8240_e3533;

        let assign8250_e3537: f64 = (p.p568 / locals.var_lbin);
        let assign8250_e3538: f64 = (p.p146 + assign8250_e3537);
        let assign8250_e3541: f64 = (p.p656 / locals.var_wbin);
        let assign8250_e3542: f64 = (assign8250_e3538 + assign8250_e3541);
        let assign8250_e3545: f64 = (p.p744 / locals.var_lwbin);
        let assign8250_e3546: f64 = (assign8250_e3542 + assign8250_e3545);
        locals.var_uc_scp3 = assign8250_e3546;

        let assign8260_e3550: f64 = (p.p569 / locals.var_lbin);
        let assign8260_e3551: f64 = (p.p147 + assign8260_e3550);
        let assign8260_e3554: f64 = (p.p657 / locals.var_wbin);
        let assign8260_e3555: f64 = (assign8260_e3551 + assign8260_e3554);
        let assign8260_e3558: f64 = (p.p745 / locals.var_lwbin);
        let assign8260_e3559: f64 = (assign8260_e3555 + assign8260_e3558);
        locals.var_uc_sc1 = assign8260_e3559;

        let assign8270_e3563: f64 = (p.p570 / locals.var_lbin);
        let assign8270_e3564: f64 = (p.p148 + assign8270_e3563);
        let assign8270_e3567: f64 = (p.p658 / locals.var_wbin);
        let assign8270_e3568: f64 = (assign8270_e3564 + assign8270_e3567);
        let assign8270_e3571: f64 = (p.p746 / locals.var_lwbin);
        let assign8270_e3572: f64 = (assign8270_e3568 + assign8270_e3571);
        locals.var_uc_sc2 = assign8270_e3572;

        let assign8280_e3576: f64 = (p.p571 / locals.var_lbin);
        let assign8280_e3577: f64 = (p.p149 + assign8280_e3576);
        let assign8280_e3580: f64 = (p.p659 / locals.var_wbin);
        let assign8280_e3581: f64 = (assign8280_e3577 + assign8280_e3580);
        let assign8280_e3584: f64 = (p.p747 / locals.var_lwbin);
        let assign8280_e3585: f64 = (assign8280_e3581 + assign8280_e3584);
        locals.var_uc_sc3 = assign8280_e3585;

        let assign8290_e3589: f64 = (p.p572 / locals.var_lbin);
        let assign8290_e3590: f64 = (p.p151 + assign8290_e3589);
        let assign8290_e3593: f64 = (p.p660 / locals.var_wbin);
        let assign8290_e3594: f64 = (assign8290_e3590 + assign8290_e3593);
        let assign8290_e3597: f64 = (p.p748 / locals.var_lwbin);
        let assign8290_e3598: f64 = (assign8290_e3594 + assign8290_e3597);
        locals.var_uc_pgd1 = assign8290_e3598;

        let assign8300_e3602: f64 = (p.p573 / locals.var_lbin);
        let assign8300_e3603: f64 = (p.p154 + assign8300_e3602);
        let assign8300_e3606: f64 = (p.p661 / locals.var_wbin);
        let assign8300_e3607: f64 = (assign8300_e3603 + assign8300_e3606);
        let assign8300_e3610: f64 = (p.p749 / locals.var_lwbin);
        let assign8300_e3611: f64 = (assign8300_e3607 + assign8300_e3610);
        locals.var_uc_ndep = assign8300_e3611;

        let assign8310_e3615: f64 = (p.p574 / locals.var_lbin);
        let assign8310_e3616: f64 = (p.p157 + assign8310_e3615);
        let assign8310_e3619: f64 = (p.p662 / locals.var_wbin);
        let assign8310_e3620: f64 = (assign8310_e3616 + assign8310_e3619);
        let assign8310_e3623: f64 = (p.p750 / locals.var_lwbin);
        let assign8310_e3624: f64 = (assign8310_e3620 + assign8310_e3623);
        locals.var_uc_ninv = assign8310_e3624;

        let assign8320_e3628: f64 = (p.p575 / locals.var_lbin);
        let assign8320_e3629: f64 = (p.p158 + assign8320_e3628);
        let assign8320_e3632: f64 = (p.p663 / locals.var_wbin);
        let assign8320_e3633: f64 = (assign8320_e3629 + assign8320_e3632);
        let assign8320_e3636: f64 = (p.p751 / locals.var_lwbin);
        let assign8320_e3637: f64 = (assign8320_e3633 + assign8320_e3636);
        locals.var_uc_muecb0 = assign8320_e3637;

        let assign8330_e3641: f64 = (p.p576 / locals.var_lbin);
        let assign8330_e3642: f64 = (p.p159 + assign8330_e3641);
        let assign8330_e3645: f64 = (p.p664 / locals.var_wbin);
        let assign8330_e3646: f64 = (assign8330_e3642 + assign8330_e3645);
        let assign8330_e3649: f64 = (p.p752 / locals.var_lwbin);
        let assign8330_e3650: f64 = (assign8330_e3646 + assign8330_e3649);
        locals.var_uc_muecb1 = assign8330_e3650;

        let assign8340_e3654: f64 = (p.p577 / locals.var_lbin);
        let assign8340_e3655: f64 = (p.p161 + assign8340_e3654);
        let assign8340_e3658: f64 = (p.p665 / locals.var_wbin);
        let assign8340_e3659: f64 = (assign8340_e3655 + assign8340_e3658);
        let assign8340_e3662: f64 = (p.p753 / locals.var_lwbin);
        let assign8340_e3663: f64 = (assign8340_e3659 + assign8340_e3662);
        locals.var_uc_mueph1 = assign8340_e3663;

        let assign8350_e3667: f64 = (p.p578 / locals.var_lbin);
        let assign8350_e3668: f64 = (p.p169 + assign8350_e3667);
        let assign8350_e3671: f64 = (p.p666 / locals.var_wbin);
        let assign8350_e3672: f64 = (assign8350_e3668 + assign8350_e3671);
        let assign8350_e3675: f64 = (p.p754 / locals.var_lwbin);
        let assign8350_e3676: f64 = (assign8350_e3672 + assign8350_e3675);
        locals.var_uc_vtmp = assign8350_e3676;

        let assign8360_e3680: f64 = (p.p579 / locals.var_lbin);
        let assign8360_e3681: f64 = (p.p170 + assign8360_e3680);
        let assign8360_e3684: f64 = (p.p667 / locals.var_wbin);
        let assign8360_e3685: f64 = (assign8360_e3681 + assign8360_e3684);
        let assign8360_e3688: f64 = (p.p755 / locals.var_lwbin);
        let assign8360_e3689: f64 = (assign8360_e3685 + assign8360_e3688);
        locals.var_uc_wvth0 = assign8360_e3689;

        let assign8370_e3693: f64 = (p.p580 / locals.var_lbin);
        let assign8370_e3694: f64 = (p.p172 + assign8370_e3693);
        let assign8370_e3697: f64 = (p.p668 / locals.var_wbin);
        let assign8370_e3698: f64 = (assign8370_e3694 + assign8370_e3697);
        let assign8370_e3701: f64 = (p.p756 / locals.var_lwbin);
        let assign8370_e3702: f64 = (assign8370_e3698 + assign8370_e3701);
        locals.var_uc_muesr1 = assign8370_e3702;

        let assign8380_e3706: f64 = (p.p581 / locals.var_lbin);
        let assign8380_e3707: f64 = (p.p177 + assign8380_e3706);
        let assign8380_e3710: f64 = (p.p669 / locals.var_wbin);
        let assign8380_e3711: f64 = (assign8380_e3707 + assign8380_e3710);
        let assign8380_e3714: f64 = (p.p757 / locals.var_lwbin);
        let assign8380_e3715: f64 = (assign8380_e3711 + assign8380_e3714);
        locals.var_uc_muetmp = assign8380_e3715;

        let assign8390_e3719: f64 = (p.p582 / locals.var_lbin);
        let assign8390_e3720: f64 = (p.p179 + assign8390_e3719);
        let assign8390_e3723: f64 = (p.p670 / locals.var_wbin);
        let assign8390_e3724: f64 = (assign8390_e3720 + assign8390_e3723);
        let assign8390_e3727: f64 = (p.p758 / locals.var_lwbin);
        let assign8390_e3728: f64 = (assign8390_e3724 + assign8390_e3727);
        locals.var_uc_sub1 = assign8390_e3728;

        let assign8400_e3732: f64 = (p.p583 / locals.var_lbin);
        let assign8400_e3733: f64 = (p.p180 + assign8400_e3732);
        let assign8400_e3736: f64 = (p.p671 / locals.var_wbin);
        let assign8400_e3737: f64 = (assign8400_e3733 + assign8400_e3736);
        let assign8400_e3740: f64 = (p.p759 / locals.var_lwbin);
        let assign8400_e3741: f64 = (assign8400_e3737 + assign8400_e3740);
        locals.var_uc_sub2 = assign8400_e3741;

        let assign8410_e3745: f64 = (p.p584 / locals.var_lbin);
        let assign8410_e3746: f64 = (p.p185 + assign8410_e3745);
        let assign8410_e3749: f64 = (p.p672 / locals.var_wbin);
        let assign8410_e3750: f64 = (assign8410_e3746 + assign8410_e3749);
        let assign8410_e3753: f64 = (p.p760 / locals.var_lwbin);
        let assign8410_e3754: f64 = (assign8410_e3750 + assign8410_e3753);
        locals.var_uc_svds = assign8410_e3754;

        let assign8420_e3758: f64 = (p.p585 / locals.var_lbin);
        let assign8420_e3759: f64 = (p.p182 + assign8420_e3758);
        let assign8420_e3762: f64 = (p.p673 / locals.var_wbin);
        let assign8420_e3763: f64 = (assign8420_e3759 + assign8420_e3762);
        let assign8420_e3766: f64 = (p.p761 / locals.var_lwbin);
        let assign8420_e3767: f64 = (assign8420_e3763 + assign8420_e3766);
        locals.var_uc_svbs = assign8420_e3767;

        let assign8430_e3771: f64 = (p.p586 / locals.var_lbin);
        let assign8430_e3772: f64 = (p.p181 + assign8430_e3771);
        let assign8430_e3775: f64 = (p.p674 / locals.var_wbin);
        let assign8430_e3776: f64 = (assign8430_e3772 + assign8430_e3775);
        let assign8430_e3779: f64 = (p.p762 / locals.var_lwbin);
        let assign8430_e3780: f64 = (assign8430_e3776 + assign8430_e3779);
        locals.var_uc_svgs = assign8430_e3780;

        let assign8440_e3784: f64 = (p.p587 / locals.var_lbin);
        let assign8440_e3785: f64 = (p.p187 + assign8440_e3784);
        let assign8440_e3788: f64 = (p.p675 / locals.var_wbin);
        let assign8440_e3789: f64 = (assign8440_e3785 + assign8440_e3788);
        let assign8440_e3792: f64 = (p.p763 / locals.var_lwbin);
        let assign8440_e3793: f64 = (assign8440_e3789 + assign8440_e3792);
        locals.var_uc_sub1snp = assign8440_e3793;

        let assign8450_e3797: f64 = (p.p588 / locals.var_lbin);
        let assign8450_e3798: f64 = (p.p188 + assign8450_e3797);
        let assign8450_e3801: f64 = (p.p676 / locals.var_wbin);
        let assign8450_e3802: f64 = (assign8450_e3798 + assign8450_e3801);
        let assign8450_e3805: f64 = (p.p764 / locals.var_lwbin);
        let assign8450_e3806: f64 = (assign8450_e3802 + assign8450_e3805);
        locals.var_uc_sub2snp = assign8450_e3806;

        let assign8460_e3810: f64 = (p.p589 / locals.var_lbin);
        let assign8460_e3811: f64 = (p.p189 + assign8460_e3810);
        let assign8460_e3814: f64 = (p.p677 / locals.var_wbin);
        let assign8460_e3815: f64 = (assign8460_e3811 + assign8460_e3814);
        let assign8460_e3818: f64 = (p.p765 / locals.var_lwbin);
        let assign8460_e3819: f64 = (assign8460_e3815 + assign8460_e3818);
        locals.var_uc_svdssnp = assign8460_e3819;

        let assign8470_e3823: f64 = (p.p590 / locals.var_lbin);
        let assign8470_e3824: f64 = (p.p194 + assign8470_e3823);
        let assign8470_e3827: f64 = (p.p678 / locals.var_wbin);
        let assign8470_e3828: f64 = (assign8470_e3824 + assign8470_e3827);
        let assign8470_e3831: f64 = (p.p766 / locals.var_lwbin);
        let assign8470_e3832: f64 = (assign8470_e3828 + assign8470_e3831);
        locals.var_uc_fn1 = assign8470_e3832;

        let assign8480_e3836: f64 = (p.p591 / locals.var_lbin);
        let assign8480_e3837: f64 = (p.p195 + assign8480_e3836);
        let assign8480_e3840: f64 = (p.p679 / locals.var_wbin);
        let assign8480_e3841: f64 = (assign8480_e3837 + assign8480_e3840);
        let assign8480_e3844: f64 = (p.p767 / locals.var_lwbin);
        let assign8480_e3845: f64 = (assign8480_e3841 + assign8480_e3844);
        locals.var_uc_fn2 = assign8480_e3845;

        let assign8490_e3849: f64 = (p.p592 / locals.var_lbin);
        let assign8490_e3850: f64 = (p.p196 + assign8490_e3849);
        let assign8490_e3853: f64 = (p.p680 / locals.var_wbin);
        let assign8490_e3854: f64 = (assign8490_e3850 + assign8490_e3853);
        let assign8490_e3857: f64 = (p.p768 / locals.var_lwbin);
        let assign8490_e3858: f64 = (assign8490_e3854 + assign8490_e3857);
        locals.var_uc_fn3 = assign8490_e3858;

    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8500_e3862: f64 = (p.p593 / locals.var_lbin);
        let assign8500_e3863: f64 = (p.p197 + assign8500_e3862);
        let assign8500_e3866: f64 = (p.p681 / locals.var_wbin);
        let assign8500_e3867: f64 = (assign8500_e3863 + assign8500_e3866);
        let assign8500_e3870: f64 = (p.p769 / locals.var_lwbin);
        let assign8500_e3871: f64 = (assign8500_e3867 + assign8500_e3870);
        locals.var_uc_fvbs = assign8500_e3871;

        let assign8510_e3875: f64 = (p.p594 / locals.var_lbin);
        let assign8510_e3876: f64 = (p.p204 + assign8510_e3875);
        let assign8510_e3879: f64 = (p.p682 / locals.var_wbin);
        let assign8510_e3880: f64 = (assign8510_e3876 + assign8510_e3879);
        let assign8510_e3883: f64 = (p.p770 / locals.var_lwbin);
        let assign8510_e3884: f64 = (assign8510_e3880 + assign8510_e3883);
        locals.var_uc_nsti = assign8510_e3884;

        let assign8520_e3888: f64 = (p.p595 / locals.var_lbin);
        let assign8520_e3889: f64 = (p.p205 + assign8520_e3888);
        let assign8520_e3892: f64 = (p.p683 / locals.var_wbin);
        let assign8520_e3893: f64 = (assign8520_e3889 + assign8520_e3892);
        let assign8520_e3896: f64 = (p.p771 / locals.var_lwbin);
        let assign8520_e3897: f64 = (assign8520_e3893 + assign8520_e3896);
        locals.var_uc_wsti = assign8520_e3897;
        locals.var_uc_wsti_dn0 = 0.0;
        locals.var_uc_wsti_dn2 = 0.0;
        locals.var_uc_wsti_dn4 = 0.0;
        locals.var_uc_wsti_dn5 = 0.0;
        locals.var_uc_wsti_dn6 = 0.0;
        locals.var_uc_wsti_dn7 = 0.0;
        locals.var_uc_wsti_dn8 = 0.0;
        locals.var_uc_wsti_dn9 = 0.0;
        locals.var_uc_wsti_dn10 = 0.0;
        locals.var_uc_wsti_dn13 = 0.0;

        let assign8530_e3901: f64 = (p.p596 / locals.var_lbin);
        let assign8530_e3902: f64 = (p.p210 + assign8530_e3901);
        let assign8530_e3905: f64 = (p.p684 / locals.var_wbin);
        let assign8530_e3906: f64 = (assign8530_e3902 + assign8530_e3905);
        let assign8530_e3909: f64 = (p.p772 / locals.var_lwbin);
        let assign8530_e3910: f64 = (assign8530_e3906 + assign8530_e3909);
        locals.var_uc_scsti1 = assign8530_e3910;

        let assign8540_e3914: f64 = (p.p597 / locals.var_lbin);
        let assign8540_e3915: f64 = (p.p211 + assign8540_e3914);
        let assign8540_e3918: f64 = (p.p685 / locals.var_wbin);
        let assign8540_e3919: f64 = (assign8540_e3915 + assign8540_e3918);
        let assign8540_e3922: f64 = (p.p773 / locals.var_lwbin);
        let assign8540_e3923: f64 = (assign8540_e3919 + assign8540_e3922);
        locals.var_uc_scsti2 = assign8540_e3923;

        let assign8550_e3927: f64 = (p.p598 / locals.var_lbin);
        let assign8550_e3928: f64 = (p.p212 + assign8550_e3927);
        let assign8550_e3931: f64 = (p.p686 / locals.var_wbin);
        let assign8550_e3932: f64 = (assign8550_e3928 + assign8550_e3931);
        let assign8550_e3935: f64 = (p.p774 / locals.var_lwbin);
        let assign8550_e3936: f64 = (assign8550_e3932 + assign8550_e3935);
        locals.var_uc_vthsti = assign8550_e3936;

        let assign8560_e3940: f64 = (p.p599 / locals.var_lbin);
        let assign8560_e3941: f64 = (p.p214 + assign8560_e3940);
        let assign8560_e3944: f64 = (p.p687 / locals.var_wbin);
        let assign8560_e3945: f64 = (assign8560_e3941 + assign8560_e3944);
        let assign8560_e3948: f64 = (p.p775 / locals.var_lwbin);
        let assign8560_e3949: f64 = (assign8560_e3945 + assign8560_e3948);
        locals.var_uc_muesti1 = assign8560_e3949;

        let assign8570_e3953: f64 = (p.p600 / locals.var_lbin);
        let assign8570_e3954: f64 = (p.p215 + assign8570_e3953);
        let assign8570_e3957: f64 = (p.p688 / locals.var_wbin);
        let assign8570_e3958: f64 = (assign8570_e3954 + assign8570_e3957);
        let assign8570_e3961: f64 = (p.p776 / locals.var_lwbin);
        let assign8570_e3962: f64 = (assign8570_e3958 + assign8570_e3961);
        locals.var_uc_muesti2 = assign8570_e3962;

        let assign8580_e3966: f64 = (p.p601 / locals.var_lbin);
        let assign8580_e3967: f64 = (p.p216 + assign8580_e3966);
        let assign8580_e3970: f64 = (p.p689 / locals.var_wbin);
        let assign8580_e3971: f64 = (assign8580_e3967 + assign8580_e3970);
        let assign8580_e3974: f64 = (p.p777 / locals.var_lwbin);
        let assign8580_e3975: f64 = (assign8580_e3971 + assign8580_e3974);
        locals.var_uc_muesti3 = assign8580_e3975;

        let assign8590_e3979: f64 = (p.p602 / locals.var_lbin);
        let assign8590_e3980: f64 = (p.p217 + assign8590_e3979);
        let assign8590_e3983: f64 = (p.p690 / locals.var_wbin);
        let assign8590_e3984: f64 = (assign8590_e3980 + assign8590_e3983);
        let assign8590_e3987: f64 = (p.p778 / locals.var_lwbin);
        let assign8590_e3988: f64 = (assign8590_e3984 + assign8590_e3987);
        locals.var_uc_nsubpsti1 = assign8590_e3988;

        let assign8600_e3992: f64 = (p.p603 / locals.var_lbin);
        let assign8600_e3993: f64 = (p.p218 + assign8600_e3992);
        let assign8600_e3996: f64 = (p.p691 / locals.var_wbin);
        let assign8600_e3997: f64 = (assign8600_e3993 + assign8600_e3996);
        let assign8600_e4000: f64 = (p.p779 / locals.var_lwbin);
        let assign8600_e4001: f64 = (assign8600_e3997 + assign8600_e4000);
        locals.var_uc_nsubpsti2 = assign8600_e4001;

        let assign8610_e4005: f64 = (p.p604 / locals.var_lbin);
        let assign8610_e4006: f64 = (p.p219 + assign8610_e4005);
        let assign8610_e4009: f64 = (p.p692 / locals.var_wbin);
        let assign8610_e4010: f64 = (assign8610_e4006 + assign8610_e4009);
        let assign8610_e4013: f64 = (p.p780 / locals.var_lwbin);
        let assign8610_e4014: f64 = (assign8610_e4010 + assign8610_e4013);
        locals.var_uc_nsubpsti3 = assign8610_e4014;

        let assign8620_e4018: f64 = (p.p605 / locals.var_lbin);
        let assign8620_e4019: f64 = (p.p269 + assign8620_e4018);
        let assign8620_e4022: f64 = (p.p693 / locals.var_wbin);
        let assign8620_e4023: f64 = (assign8620_e4019 + assign8620_e4022);
        let assign8620_e4026: f64 = (p.p781 / locals.var_lwbin);
        let assign8620_e4027: f64 = (assign8620_e4023 + assign8620_e4026);
        locals.var_uc_cgso = assign8620_e4027;

        let assign8630_e4031: f64 = (p.p606 / locals.var_lbin);
        let assign8630_e4032: f64 = (p.p268 + assign8630_e4031);
        let assign8630_e4035: f64 = (p.p694 / locals.var_wbin);
        let assign8630_e4036: f64 = (assign8630_e4032 + assign8630_e4035);
        let assign8630_e4039: f64 = (p.p782 / locals.var_lwbin);
        let assign8630_e4040: f64 = (assign8630_e4036 + assign8630_e4039);
        locals.var_uc_cgdo = assign8630_e4040;

        let assign8640_e4044: f64 = (p.p607 / locals.var_lbin);
        let assign8640_e4045: f64 = (p.p226 + assign8640_e4044);
        let assign8640_e4048: f64 = (p.p695 / locals.var_wbin);
        let assign8640_e4049: f64 = (assign8640_e4045 + assign8640_e4048);
        let assign8640_e4052: f64 = (p.p783 / locals.var_lwbin);
        let assign8640_e4053: f64 = (assign8640_e4049 + assign8640_e4052);
        locals.var_uc_clm1 = assign8640_e4053;

        let assign8650_e4057: f64 = (p.p608 / locals.var_lbin);
        let assign8650_e4058: f64 = (p.p227 + assign8650_e4057);
        let assign8650_e4061: f64 = (p.p696 / locals.var_wbin);
        let assign8650_e4062: f64 = (assign8650_e4058 + assign8650_e4061);
        let assign8650_e4065: f64 = (p.p784 / locals.var_lwbin);
        let assign8650_e4066: f64 = (assign8650_e4062 + assign8650_e4065);
        locals.var_uc_clm2 = assign8650_e4066;
        locals.var_uc_clm2_dn0 = 0.0;
        locals.var_uc_clm2_dn2 = 0.0;
        locals.var_uc_clm2_dn4 = 0.0;
        locals.var_uc_clm2_dn5 = 0.0;
        locals.var_uc_clm2_dn6 = 0.0;
        locals.var_uc_clm2_dn7 = 0.0;
        locals.var_uc_clm2_dn8 = 0.0;
        locals.var_uc_clm2_dn9 = 0.0;
        locals.var_uc_clm2_dn10 = 0.0;
        locals.var_uc_clm2_dn13 = 0.0;

        let assign8660_e4070: f64 = (p.p609 / locals.var_lbin);
        let assign8660_e4071: f64 = (p.p228 + assign8660_e4070);
        let assign8660_e4074: f64 = (p.p697 / locals.var_wbin);
        let assign8660_e4075: f64 = (assign8660_e4071 + assign8660_e4074);
        let assign8660_e4078: f64 = (p.p785 / locals.var_lwbin);
        let assign8660_e4079: f64 = (assign8660_e4075 + assign8660_e4078);
        locals.var_uc_clm3 = assign8660_e4079;

        let assign8670_e4083: f64 = (p.p610 / locals.var_lbin);
        let assign8670_e4084: f64 = (p.p232 + assign8670_e4083);
        let assign8670_e4087: f64 = (p.p698 / locals.var_wbin);
        let assign8670_e4088: f64 = (assign8670_e4084 + assign8670_e4087);
        let assign8670_e4091: f64 = (p.p786 / locals.var_lwbin);
        let assign8670_e4092: f64 = (assign8670_e4088 + assign8670_e4091);
        locals.var_uc_wfc = assign8670_e4092;

        let assign8680_e4096: f64 = (p.p611 / locals.var_lbin);
        let assign8680_e4097: f64 = (p.p240 + assign8680_e4096);
        let assign8680_e4100: f64 = (p.p699 / locals.var_wbin);
        let assign8680_e4101: f64 = (assign8680_e4097 + assign8680_e4100);
        let assign8680_e4104: f64 = (p.p787 / locals.var_lwbin);
        let assign8680_e4105: f64 = (assign8680_e4101 + assign8680_e4104);
        locals.var_uc_gidl1 = assign8680_e4105;

        let assign8690_e4109: f64 = (p.p612 / locals.var_lbin);
        let assign8690_e4110: f64 = (p.p241 + assign8690_e4109);
        let assign8690_e4113: f64 = (p.p700 / locals.var_wbin);
        let assign8690_e4114: f64 = (assign8690_e4110 + assign8690_e4113);
        let assign8690_e4117: f64 = (p.p788 / locals.var_lwbin);
        let assign8690_e4118: f64 = (assign8690_e4114 + assign8690_e4117);
        locals.var_uc_gidl2 = assign8690_e4118;

        let assign8700_e4122: f64 = (p.p613 / locals.var_lbin);
        let assign8700_e4123: f64 = (p.p245 + assign8700_e4122);
        let assign8700_e4126: f64 = (p.p701 / locals.var_wbin);
        let assign8700_e4127: f64 = (assign8700_e4123 + assign8700_e4126);
        let assign8700_e4130: f64 = (p.p789 / locals.var_lwbin);
        let assign8700_e4131: f64 = (assign8700_e4127 + assign8700_e4130);
        locals.var_uc_gleak1 = assign8700_e4131;

        let assign8710_e4135: f64 = (p.p614 / locals.var_lbin);
        let assign8710_e4136: f64 = (p.p246 + assign8710_e4135);
        let assign8710_e4139: f64 = (p.p702 / locals.var_wbin);
        let assign8710_e4140: f64 = (assign8710_e4136 + assign8710_e4139);
        let assign8710_e4143: f64 = (p.p790 / locals.var_lwbin);
        let assign8710_e4144: f64 = (assign8710_e4140 + assign8710_e4143);
        locals.var_uc_gleak2 = assign8710_e4144;

        let assign8720_e4148: f64 = (p.p615 / locals.var_lbin);
        let assign8720_e4149: f64 = (p.p247 + assign8720_e4148);
        let assign8720_e4152: f64 = (p.p703 / locals.var_wbin);
        let assign8720_e4153: f64 = (assign8720_e4149 + assign8720_e4152);
        let assign8720_e4156: f64 = (p.p791 / locals.var_lwbin);
        let assign8720_e4157: f64 = (assign8720_e4153 + assign8720_e4156);
        locals.var_uc_gleak3 = assign8720_e4157;

        let assign8730_e4161: f64 = (p.p616 / locals.var_lbin);
        let assign8730_e4162: f64 = (p.p250 + assign8730_e4161);
        let assign8730_e4165: f64 = (p.p704 / locals.var_wbin);
        let assign8730_e4166: f64 = (assign8730_e4162 + assign8730_e4165);
        let assign8730_e4169: f64 = (p.p792 / locals.var_lwbin);
        let assign8730_e4170: f64 = (assign8730_e4166 + assign8730_e4169);
        locals.var_uc_gleak6 = assign8730_e4170;

        let assign8740_e4174: f64 = (p.p617 / locals.var_lbin);
        let assign8740_e4175: f64 = (p.p253 + assign8740_e4174);
        let assign8740_e4178: f64 = (p.p705 / locals.var_wbin);
        let assign8740_e4179: f64 = (assign8740_e4175 + assign8740_e4178);
        let assign8740_e4182: f64 = (p.p793 / locals.var_lwbin);
        let assign8740_e4183: f64 = (assign8740_e4179 + assign8740_e4182);
        locals.var_uc_glksd1 = assign8740_e4183;

        let assign8750_e4187: f64 = (p.p618 / locals.var_lbin);
        let assign8750_e4188: f64 = (p.p254 + assign8750_e4187);
        let assign8750_e4191: f64 = (p.p706 / locals.var_wbin);
        let assign8750_e4192: f64 = (assign8750_e4188 + assign8750_e4191);
        let assign8750_e4195: f64 = (p.p794 / locals.var_lwbin);
        let assign8750_e4196: f64 = (assign8750_e4192 + assign8750_e4195);
        locals.var_uc_glksd2 = assign8750_e4196;

        let assign8760_e4200: f64 = (p.p619 / locals.var_lbin);
        let assign8760_e4201: f64 = (p.p256 + assign8760_e4200);
        let assign8760_e4204: f64 = (p.p707 / locals.var_wbin);
        let assign8760_e4205: f64 = (assign8760_e4201 + assign8760_e4204);
        let assign8760_e4208: f64 = (p.p795 / locals.var_lwbin);
        let assign8760_e4209: f64 = (assign8760_e4205 + assign8760_e4208);
        locals.var_uc_glkb1 = assign8760_e4209;

        let assign8770_e4213: f64 = (p.p620 / locals.var_lbin);
        let assign8770_e4214: f64 = (p.p257 + assign8770_e4213);
        let assign8770_e4217: f64 = (p.p708 / locals.var_wbin);
        let assign8770_e4218: f64 = (assign8770_e4214 + assign8770_e4217);
        let assign8770_e4221: f64 = (p.p796 / locals.var_lwbin);
        let assign8770_e4222: f64 = (assign8770_e4218 + assign8770_e4221);
        locals.var_uc_glkb2 = assign8770_e4222;

        let assign8790_e4239: f64 = (p.p622 / locals.var_lbin);
        let assign8790_e4240: f64 = (p.p265 + assign8790_e4239);
        let assign8790_e4243: f64 = (p.p710 / locals.var_wbin);
        let assign8790_e4244: f64 = (assign8790_e4240 + assign8790_e4243);
        let assign8790_e4247: f64 = (p.p798 / locals.var_lwbin);
        let assign8790_e4248: f64 = (assign8790_e4244 + assign8790_e4247);
        locals.var_uc_nfalp = assign8790_e4248;

        let assign8800_e4252: f64 = (p.p623 / locals.var_lbin);
        let assign8800_e4253: f64 = (p.p278 + assign8800_e4252);
        let assign8800_e4256: f64 = (p.p711 / locals.var_wbin);
        let assign8800_e4257: f64 = (assign8800_e4253 + assign8800_e4256);
        let assign8800_e4260: f64 = (p.p799 / locals.var_lwbin);
        let assign8800_e4261: f64 = (assign8800_e4257 + assign8800_e4260);
        locals.var_uc_ibpc1 = assign8800_e4261;

        let assign8810_e4265: f64 = (p.p624 / locals.var_lbin);
        let assign8810_e4266: f64 = (p.p281 + assign8810_e4265);
        let assign8810_e4269: f64 = (p.p712 / locals.var_wbin);
        let assign8810_e4270: f64 = (assign8810_e4266 + assign8810_e4269);
        let assign8810_e4273: f64 = (p.p800 / locals.var_lwbin);
        let assign8810_e4274: f64 = (assign8810_e4270 + assign8810_e4273);
        locals.var_uc_ibpc2 = assign8810_e4274;

        let assign8820_e4278: f64 = (p.p625 / locals.var_lbin);
        let assign8820_e4279: f64 = (p.p79 + assign8820_e4278);
        let assign8820_e4282: f64 = (p.p713 / locals.var_wbin);
        let assign8820_e4283: f64 = (assign8820_e4279 + assign8820_e4282);
        let assign8820_e4286: f64 = (p.p801 / locals.var_lwbin);
        let assign8820_e4287: f64 = (assign8820_e4283 + assign8820_e4286);
        locals.var_uc_cgbo = assign8820_e4287;

        let assign8830_e4291: f64 = (p.p626 / locals.var_lbin);
        let assign8830_e4292: f64 = (p.p86 + assign8830_e4291);
        let assign8830_e4295: f64 = (p.p714 / locals.var_wbin);
        let assign8830_e4296: f64 = (assign8830_e4292 + assign8830_e4295);
        let assign8830_e4299: f64 = (p.p802 / locals.var_lwbin);
        let assign8830_e4300: f64 = (assign8830_e4296 + assign8830_e4299);
        locals.var_uc_cvdsover = assign8830_e4300;

        let assign8850_e4317: f64 = (p.p628 / locals.var_lbin);
        let assign8850_e4318: f64 = (p.p76 + assign8850_e4317);
        let assign8850_e4321: f64 = (p.p716 / locals.var_wbin);
        let assign8850_e4322: f64 = (assign8850_e4318 + assign8850_e4321);
        let assign8850_e4325: f64 = (p.p804 / locals.var_lwbin);
        let assign8850_e4326: f64 = (assign8850_e4322 + assign8850_e4325);
        locals.var_uc_npext = assign8850_e4326;

        let assign8860_e4330: f64 = (p.p629 / locals.var_lbin);
        let assign8860_e4331: f64 = (p.p81 + assign8860_e4330);
        let assign8860_e4334: f64 = (p.p717 / locals.var_wbin);
        let assign8860_e4335: f64 = (assign8860_e4331 + assign8860_e4334);
        let assign8860_e4338: f64 = (p.p805 / locals.var_lwbin);
        let assign8860_e4339: f64 = (assign8860_e4335 + assign8860_e4338);
        locals.var_uc_powrat = assign8860_e4339;

        let assign8870_e4343: f64 = (p.p630 / locals.var_lbin);
        let assign8870_e4344: f64 = (p.p74 + assign8870_e4343);
        let assign8870_e4347: f64 = (p.p718 / locals.var_wbin);
        let assign8870_e4348: f64 = (assign8870_e4344 + assign8870_e4347);
        let assign8870_e4351: f64 = (p.p806 / locals.var_lwbin);
        let assign8870_e4352: f64 = (assign8870_e4348 + assign8870_e4351);
        locals.var_uc_rd = assign8870_e4352;

        let assign8880_e4356: f64 = (p.p631 / locals.var_lbin);
        let assign8880_e4357: f64 = (p.p298 + assign8880_e4356);
        let assign8880_e4360: f64 = (p.p719 / locals.var_wbin);
        let assign8880_e4361: f64 = (assign8880_e4357 + assign8880_e4360);
        let assign8880_e4364: f64 = (p.p807 / locals.var_lwbin);
        let assign8880_e4365: f64 = (assign8880_e4361 + assign8880_e4364);
        locals.var_uc_rd22 = assign8880_e4365;

        let assign8890_e4369: f64 = (p.p632 / locals.var_lbin);
        let assign8890_e4370: f64 = (p.p83 + assign8890_e4369);
        let assign8890_e4373: f64 = (p.p720 / locals.var_wbin);
        let assign8890_e4374: f64 = (assign8890_e4370 + assign8890_e4373);
        let assign8890_e4377: f64 = (p.p808 / locals.var_lwbin);
        let assign8890_e4378: f64 = (assign8890_e4374 + assign8890_e4377);
        locals.var_uc_rd23 = assign8890_e4378;

        let assign8900_e4382: f64 = (p.p633 / locals.var_lbin);
        let assign8900_e4383: f64 = (p.p84 + assign8900_e4382);
        let assign8900_e4386: f64 = (p.p721 / locals.var_wbin);
        let assign8900_e4387: f64 = (assign8900_e4383 + assign8900_e4386);
        let assign8900_e4390: f64 = (p.p809 / locals.var_lwbin);
        let assign8900_e4391: f64 = (assign8900_e4387 + assign8900_e4390);
        locals.var_uc_rd24 = assign8900_e4391;

        let assign8910_e4395: f64 = (p.p634 / locals.var_lbin);
        let assign8910_e4396: f64 = (p.p62 + assign8910_e4395);
        let assign8910_e4399: f64 = (p.p722 / locals.var_wbin);
        let assign8910_e4400: f64 = (assign8910_e4396 + assign8910_e4399);
        let assign8910_e4403: f64 = (p.p810 / locals.var_lwbin);
        let assign8910_e4404: f64 = (assign8910_e4400 + assign8910_e4403);
        locals.var_uc_rdict1 = assign8910_e4404;

        let assign8920_e4408: f64 = (p.p635 / locals.var_lbin);
        let assign8920_e4409: f64 = (p.p59 + assign8920_e4408);
        let assign8920_e4412: f64 = (p.p723 / locals.var_wbin);
        let assign8920_e4413: f64 = (assign8920_e4409 + assign8920_e4412);
        let assign8920_e4416: f64 = (p.p811 / locals.var_lwbin);
        let assign8920_e4417: f64 = (assign8920_e4413 + assign8920_e4416);
        locals.var_uc_rdov13 = assign8920_e4417;

        let assign8930_e4421: f64 = (p.p636 / locals.var_lbin);
        let assign8930_e4422: f64 = (p.p60 + assign8930_e4421);
        let assign8930_e4425: f64 = (p.p724 / locals.var_wbin);
        let assign8930_e4426: f64 = (assign8930_e4422 + assign8930_e4425);
        let assign8930_e4429: f64 = (p.p812 / locals.var_lwbin);
        let assign8930_e4430: f64 = (assign8930_e4426 + assign8930_e4429);
        locals.var_uc_rdslp1 = assign8930_e4430;

        let assign8940_e4434: f64 = (p.p637 / locals.var_lbin);
        let assign8940_e4435: f64 = (p.p85 + assign8940_e4434);
        let assign8940_e4438: f64 = (p.p725 / locals.var_wbin);
        let assign8940_e4439: f64 = (assign8940_e4435 + assign8940_e4438);
        let assign8940_e4442: f64 = (p.p813 / locals.var_lwbin);
        let assign8940_e4443: f64 = (assign8940_e4439 + assign8940_e4442);
        locals.var_uc_rdvb = assign8940_e4443;

        let assign8950_e4447: f64 = (p.p638 / locals.var_lbin);
        let assign8950_e4448: f64 = (p.p82 + assign8950_e4447);
        let assign8950_e4451: f64 = (p.p726 / locals.var_wbin);
        let assign8950_e4452: f64 = (assign8950_e4448 + assign8950_e4451);
        let assign8950_e4455: f64 = (p.p814 / locals.var_lwbin);
        let assign8950_e4456: f64 = (assign8950_e4452 + assign8950_e4455);
        locals.var_uc_rdvd = assign8950_e4456;

        let assign8960_e4460: f64 = (p.p639 / locals.var_lbin);
        let assign8960_e4461: f64 = (p.p61 + assign8960_e4460);
        let assign8960_e4464: f64 = (p.p727 / locals.var_wbin);
        let assign8960_e4465: f64 = (assign8960_e4461 + assign8960_e4464);
        let assign8960_e4468: f64 = (p.p815 / locals.var_lwbin);
        let assign8960_e4469: f64 = (assign8960_e4465 + assign8960_e4468);
        locals.var_uc_rdvg11 = assign8960_e4469;

        let assign8970_e4473: f64 = (p.p640 / locals.var_lbin);
        let assign8970_e4474: f64 = (p.p75 + assign8970_e4473);
        let assign8970_e4477: f64 = (p.p728 / locals.var_wbin);
        let assign8970_e4478: f64 = (assign8970_e4474 + assign8970_e4477);
        let assign8970_e4481: f64 = (p.p816 / locals.var_lwbin);
        let assign8970_e4482: f64 = (assign8970_e4478 + assign8970_e4481);
        locals.var_uc_rs = assign8970_e4482;

        let assign8980_e4486: f64 = (p.p641 / locals.var_lbin);
        let assign8980_e4487: f64 = (p.p80 + assign8980_e4486);
        let assign8980_e4490: f64 = (p.p729 / locals.var_wbin);
        let assign8980_e4491: f64 = (assign8980_e4487 + assign8980_e4490);
        let assign8980_e4494: f64 = (p.p817 / locals.var_lwbin);
        let assign8980_e4495: f64 = (assign8980_e4491 + assign8980_e4494);
        locals.var_uc_rth0 = assign8980_e4495;

        let assign8990_e4499: f64 = (p.p642 / locals.var_lbin);
        let assign8990_e4500: f64 = (p.p77 + assign8990_e4499);
        let assign8990_e4503: f64 = (p.p730 / locals.var_wbin);
        let assign8990_e4504: f64 = (assign8990_e4500 + assign8990_e4503);
        let assign8990_e4507: f64 = (p.p818 / locals.var_lwbin);
        let assign8990_e4508: f64 = (assign8990_e4504 + assign8990_e4507);
        locals.var_uc_vover = assign8990_e4508;

        let assign9000_e4512: f64 = (p.p824 / locals.var_lbin);
        let assign9000_e4513: f64 = (p.p493 + assign9000_e4512);
        let assign9000_e4516: f64 = (p.p839 / locals.var_wbin);
        let assign9000_e4517: f64 = (assign9000_e4513 + assign9000_e4516);
        let assign9000_e4520: f64 = (p.p854 / locals.var_lwbin);
        let assign9000_e4521: f64 = (assign9000_e4517 + assign9000_e4520);
        locals.var_uc_js0d = assign9000_e4521;

        let assign9010_e4525: f64 = (p.p825 / locals.var_lbin);
        let assign9010_e4526: f64 = (p.p494 + assign9010_e4525);
        let assign9010_e4529: f64 = (p.p840 / locals.var_wbin);
        let assign9010_e4530: f64 = (assign9010_e4526 + assign9010_e4529);
        let assign9010_e4533: f64 = (p.p855 / locals.var_lwbin);
        let assign9010_e4534: f64 = (assign9010_e4530 + assign9010_e4533);
        locals.var_uc_js0swd = assign9010_e4534;

        let assign9020_e4538: f64 = (p.p826 / locals.var_lbin);
        let assign9020_e4539: f64 = (p.p496 + assign9020_e4538);
        let assign9020_e4542: f64 = (p.p841 / locals.var_wbin);
        let assign9020_e4543: f64 = (assign9020_e4539 + assign9020_e4542);
        let assign9020_e4546: f64 = (p.p856 / locals.var_lwbin);
        let assign9020_e4547: f64 = (assign9020_e4543 + assign9020_e4546);
        locals.var_uc_njd = assign9020_e4547;

        let assign9030_e4551: f64 = (p.p827 / locals.var_lbin);
        let assign9030_e4552: f64 = (p.p513 + assign9030_e4551);
        let assign9030_e4555: f64 = (p.p842 / locals.var_wbin);
        let assign9030_e4556: f64 = (assign9030_e4552 + assign9030_e4555);
        let assign9030_e4559: f64 = (p.p857 / locals.var_lwbin);
        let assign9030_e4560: f64 = (assign9030_e4556 + assign9030_e4559);
        locals.var_uc_cisbkd = assign9030_e4560;

        let assign9040_e4564: f64 = (p.p828 / locals.var_lbin);
        let assign9040_e4565: f64 = (p.p515 + assign9040_e4564);
        let assign9040_e4568: f64 = (p.p843 / locals.var_wbin);
        let assign9040_e4569: f64 = (assign9040_e4565 + assign9040_e4568);
        let assign9040_e4572: f64 = (p.p858 / locals.var_lwbin);
        let assign9040_e4573: f64 = (assign9040_e4569 + assign9040_e4572);
        locals.var_uc_vdiffjd = assign9040_e4573;

        let assign9050_e4577: f64 = (p.p829 / locals.var_lbin);
        let assign9050_e4578: f64 = (p.p516 + assign9050_e4577);
        let assign9050_e4581: f64 = (p.p844 / locals.var_wbin);
        let assign9050_e4582: f64 = (assign9050_e4578 + assign9050_e4581);
        let assign9050_e4585: f64 = (p.p859 / locals.var_lwbin);
        let assign9050_e4586: f64 = (assign9050_e4582 + assign9050_e4585);
        locals.var_uc_js0s = assign9050_e4586;

        let assign9060_e4590: f64 = (p.p830 / locals.var_lbin);
        let assign9060_e4591: f64 = (p.p517 + assign9060_e4590);
        let assign9060_e4594: f64 = (p.p845 / locals.var_wbin);
        let assign9060_e4595: f64 = (assign9060_e4591 + assign9060_e4594);
        let assign9060_e4598: f64 = (p.p860 / locals.var_lwbin);
        let assign9060_e4599: f64 = (assign9060_e4595 + assign9060_e4598);
        locals.var_uc_js0sws = assign9060_e4599;

        let assign9070_e4603: f64 = (p.p831 / locals.var_lbin);
        let assign9070_e4604: f64 = (p.p519 + assign9070_e4603);
        let assign9070_e4607: f64 = (p.p846 / locals.var_wbin);
        let assign9070_e4608: f64 = (assign9070_e4604 + assign9070_e4607);
        let assign9070_e4611: f64 = (p.p861 / locals.var_lwbin);
        let assign9070_e4612: f64 = (assign9070_e4608 + assign9070_e4611);
        locals.var_uc_njs = assign9070_e4612;

        let assign9080_e4616: f64 = (p.p832 / locals.var_lbin);
        let assign9080_e4617: f64 = (p.p536 + assign9080_e4616);
        let assign9080_e4620: f64 = (p.p847 / locals.var_wbin);
        let assign9080_e4621: f64 = (assign9080_e4617 + assign9080_e4620);
        let assign9080_e4624: f64 = (p.p862 / locals.var_lwbin);
        let assign9080_e4625: f64 = (assign9080_e4621 + assign9080_e4624);
        locals.var_uc_cisbks = assign9080_e4625;

        let assign9090_e4629: f64 = (p.p833 / locals.var_lbin);
        let assign9090_e4630: f64 = (p.p538 + assign9090_e4629);
        let assign9090_e4633: f64 = (p.p848 / locals.var_wbin);
        let assign9090_e4634: f64 = (assign9090_e4630 + assign9090_e4633);
        let assign9090_e4637: f64 = (p.p863 / locals.var_lwbin);
        let assign9090_e4638: f64 = (assign9090_e4634 + assign9090_e4637);
        locals.var_uc_vdiffjs = assign9090_e4638;

        let assign9190_e4689: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard185 = assign9190_e4689;

        let (assign9200_e4695, assign9200_e4695_d_n0, assign9200_e4695_d_n2, assign9200_e4695_d_n4, assign9200_e4695_d_n5, assign9200_e4695_d_n6, assign9200_e4695_d_n7, assign9200_e4695_d_n8, assign9200_e4695_d_n9, assign9200_e4695_d_n10, assign9200_e4695_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9200_e4693: f64 = (locals.var_lg).powf(p.p342);
        (assign9200_e4693, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9200_e4695;
        locals.var_t3_dn0 = assign9200_e4695_d_n0;
        locals.var_t3_dn2 = assign9200_e4695_d_n2;
        locals.var_t3_dn4 = assign9200_e4695_d_n4;
        locals.var_t3_dn5 = assign9200_e4695_d_n5;
        locals.var_t3_dn6 = assign9200_e4695_d_n6;
        locals.var_t3_dn7 = assign9200_e4695_d_n7;
        locals.var_t3_dn8 = assign9200_e4695_d_n8;
        locals.var_t3_dn9 = assign9200_e4695_d_n9;
        locals.var_t3_dn10 = assign9200_e4695_d_n10;
        locals.var_t3_dn13 = assign9200_e4695_d_n13;

    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9210_e4705, assign9210_e4705_d_n0, assign9210_e4705_d_n2, assign9210_e4705_d_n4, assign9210_e4705_d_n5, assign9210_e4705_d_n6, assign9210_e4705_d_n7, assign9210_e4705_d_n8, assign9210_e4705_d_n9, assign9210_e4705_d_n10, assign9210_e4705_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9210_e4701: f64 = (p.p341 / locals.var_t3);
        let assign9210_e4702: f64 = (1.0 + assign9210_e4701);
        let assign9210_e4703: f64 = (locals.var_uc_ndepm * assign9210_e4702);
        (assign9210_e4703, ((locals.var_uc_ndepm_dn0 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn2 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn4 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn5 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn6 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn7 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn8 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn9 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn10 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn13 * assign9210_e4702) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign9210_e4705;
        locals.var_uc_ndepm_dn0 = assign9210_e4705_d_n0;
        locals.var_uc_ndepm_dn2 = assign9210_e4705_d_n2;
        locals.var_uc_ndepm_dn4 = assign9210_e4705_d_n4;
        locals.var_uc_ndepm_dn5 = assign9210_e4705_d_n5;
        locals.var_uc_ndepm_dn6 = assign9210_e4705_d_n6;
        locals.var_uc_ndepm_dn7 = assign9210_e4705_d_n7;
        locals.var_uc_ndepm_dn8 = assign9210_e4705_d_n8;
        locals.var_uc_ndepm_dn9 = assign9210_e4705_d_n9;
        locals.var_uc_ndepm_dn10 = assign9210_e4705_d_n10;
        locals.var_uc_ndepm_dn13 = assign9210_e4705_d_n13;

        let assign9220_e4708: f64 = if locals.var_uc_ndepm < 1e21 { 1.0 } else { 0.0 };
        locals.var_guard186 = assign9220_e4708;

        let (assign9230_e4714, assign9230_e4714_d_n0, assign9230_e4714_d_n2, assign9230_e4714_d_n4, assign9230_e4714_d_n5, assign9230_e4714_d_n6, assign9230_e4714_d_n7, assign9230_e4714_d_n8, assign9230_e4714_d_n9, assign9230_e4714_d_n10, assign9230_e4714_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard186 != 0.0)) {
        (1e21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign9230_e4714;
        locals.var_uc_ndepm_dn0 = assign9230_e4714_d_n0;
        locals.var_uc_ndepm_dn2 = assign9230_e4714_d_n2;
        locals.var_uc_ndepm_dn4 = assign9230_e4714_d_n4;
        locals.var_uc_ndepm_dn5 = assign9230_e4714_d_n5;
        locals.var_uc_ndepm_dn6 = assign9230_e4714_d_n6;
        locals.var_uc_ndepm_dn7 = assign9230_e4714_d_n7;
        locals.var_uc_ndepm_dn8 = assign9230_e4714_d_n8;
        locals.var_uc_ndepm_dn9 = assign9230_e4714_d_n9;
        locals.var_uc_ndepm_dn10 = assign9230_e4714_d_n10;
        locals.var_uc_ndepm_dn13 = assign9230_e4714_d_n13;

        let (assign9240_e4720, assign9240_e4720_d_n0, assign9240_e4720_d_n2, assign9240_e4720_d_n4, assign9240_e4720_d_n5, assign9240_e4720_d_n6, assign9240_e4720_d_n7, assign9240_e4720_d_n8, assign9240_e4720_d_n9, assign9240_e4720_d_n10, assign9240_e4720_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9240_e4718: f64 = (locals.var_lg).powf(p.p369);
        (assign9240_e4718, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9240_e4720;
        locals.var_t3_dn0 = assign9240_e4720_d_n0;
        locals.var_t3_dn2 = assign9240_e4720_d_n2;
        locals.var_t3_dn4 = assign9240_e4720_d_n4;
        locals.var_t3_dn5 = assign9240_e4720_d_n5;
        locals.var_t3_dn6 = assign9240_e4720_d_n6;
        locals.var_t3_dn7 = assign9240_e4720_d_n7;
        locals.var_t3_dn8 = assign9240_e4720_d_n8;
        locals.var_t3_dn9 = assign9240_e4720_d_n9;
        locals.var_t3_dn10 = assign9240_e4720_d_n10;
        locals.var_t3_dn13 = assign9240_e4720_d_n13;

        let (assign9250_e4730, assign9250_e4730_d_n0, assign9250_e4730_d_n2, assign9250_e4730_d_n4, assign9250_e4730_d_n5, assign9250_e4730_d_n6, assign9250_e4730_d_n7, assign9250_e4730_d_n8, assign9250_e4730_d_n9, assign9250_e4730_d_n10, assign9250_e4730_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9250_e4726: f64 = (p.p368 / locals.var_t3);
        let assign9250_e4727: f64 = (1.0 + assign9250_e4726);
        let assign9250_e4728: f64 = (locals.var_uc_depvmax * assign9250_e4727);
        (assign9250_e4728, ((locals.var_uc_depvmax_dn0 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn2 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn4 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn5 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn6 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn7 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn8 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn9 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn10 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn13 * assign9250_e4727) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign9250_e4730;
        locals.var_uc_depvmax_dn0 = assign9250_e4730_d_n0;
        locals.var_uc_depvmax_dn2 = assign9250_e4730_d_n2;
        locals.var_uc_depvmax_dn4 = assign9250_e4730_d_n4;
        locals.var_uc_depvmax_dn5 = assign9250_e4730_d_n5;
        locals.var_uc_depvmax_dn6 = assign9250_e4730_d_n6;
        locals.var_uc_depvmax_dn7 = assign9250_e4730_d_n7;
        locals.var_uc_depvmax_dn8 = assign9250_e4730_d_n8;
        locals.var_uc_depvmax_dn9 = assign9250_e4730_d_n9;
        locals.var_uc_depvmax_dn10 = assign9250_e4730_d_n10;
        locals.var_uc_depvmax_dn13 = assign9250_e4730_d_n13;

        let (assign9260_e4736, assign9260_e4736_d_n0, assign9260_e4736_d_n2, assign9260_e4736_d_n4, assign9260_e4736_d_n5, assign9260_e4736_d_n6, assign9260_e4736_d_n7, assign9260_e4736_d_n8, assign9260_e4736_d_n9, assign9260_e4736_d_n10, assign9260_e4736_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9260_e4734: f64 = (locals.var_lg).powf(p.p362);
        (assign9260_e4734, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9260_e4736;
        locals.var_t3_dn0 = assign9260_e4736_d_n0;
        locals.var_t3_dn2 = assign9260_e4736_d_n2;
        locals.var_t3_dn4 = assign9260_e4736_d_n4;
        locals.var_t3_dn5 = assign9260_e4736_d_n5;
        locals.var_t3_dn6 = assign9260_e4736_d_n6;
        locals.var_t3_dn7 = assign9260_e4736_d_n7;
        locals.var_t3_dn8 = assign9260_e4736_d_n8;
        locals.var_t3_dn9 = assign9260_e4736_d_n9;
        locals.var_t3_dn10 = assign9260_e4736_d_n10;
        locals.var_t3_dn13 = assign9260_e4736_d_n13;

        let (assign9270_e4746, assign9270_e4746_d_n0, assign9270_e4746_d_n2, assign9270_e4746_d_n4, assign9270_e4746_d_n5, assign9270_e4746_d_n6, assign9270_e4746_d_n7, assign9270_e4746_d_n8, assign9270_e4746_d_n9, assign9270_e4746_d_n10, assign9270_e4746_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9270_e4742: f64 = (p.p361 / locals.var_t3);
        let assign9270_e4743: f64 = (1.0 + assign9270_e4742);
        let assign9270_e4744: f64 = (p.p360 * assign9270_e4743);
        (assign9270_e4744, (p.p360 * (-((p.p361 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign9270_e4746;
        locals.var_uc_depleak_dn0 = assign9270_e4746_d_n0;
        locals.var_uc_depleak_dn2 = assign9270_e4746_d_n2;
        locals.var_uc_depleak_dn4 = assign9270_e4746_d_n4;
        locals.var_uc_depleak_dn5 = assign9270_e4746_d_n5;
        locals.var_uc_depleak_dn6 = assign9270_e4746_d_n6;
        locals.var_uc_depleak_dn7 = assign9270_e4746_d_n7;
        locals.var_uc_depleak_dn8 = assign9270_e4746_d_n8;
        locals.var_uc_depleak_dn9 = assign9270_e4746_d_n9;
        locals.var_uc_depleak_dn10 = assign9270_e4746_d_n10;
        locals.var_uc_depleak_dn13 = assign9270_e4746_d_n13;

        let assign9280_e4749: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard187 = assign9280_e4749;

        let (assign9290_e4755, assign9290_e4755_d_n0, assign9290_e4755_d_n2, assign9290_e4755_d_n4, assign9290_e4755_d_n5, assign9290_e4755_d_n6, assign9290_e4755_d_n7, assign9290_e4755_d_n8, assign9290_e4755_d_n9, assign9290_e4755_d_n10, assign9290_e4755_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard187 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign9290_e4755;
        locals.var_uc_depleak_dn0 = assign9290_e4755_d_n0;
        locals.var_uc_depleak_dn2 = assign9290_e4755_d_n2;
        locals.var_uc_depleak_dn4 = assign9290_e4755_d_n4;
        locals.var_uc_depleak_dn5 = assign9290_e4755_d_n5;
        locals.var_uc_depleak_dn6 = assign9290_e4755_d_n6;
        locals.var_uc_depleak_dn7 = assign9290_e4755_d_n7;
        locals.var_uc_depleak_dn8 = assign9290_e4755_d_n8;
        locals.var_uc_depleak_dn9 = assign9290_e4755_d_n9;
        locals.var_uc_depleak_dn10 = assign9290_e4755_d_n10;
        locals.var_uc_depleak_dn13 = assign9290_e4755_d_n13;

        let (assign9300_e4761, assign9300_e4761_d_n0, assign9300_e4761_d_n2, assign9300_e4761_d_n4, assign9300_e4761_d_n5, assign9300_e4761_d_n6, assign9300_e4761_d_n7, assign9300_e4761_d_n8, assign9300_e4761_d_n9, assign9300_e4761_d_n10, assign9300_e4761_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9300_e4759: f64 = (locals.var_lg).powf(p.p348);
        (assign9300_e4759, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9300_e4761;
        locals.var_t3_dn0 = assign9300_e4761_d_n0;
        locals.var_t3_dn2 = assign9300_e4761_d_n2;
        locals.var_t3_dn4 = assign9300_e4761_d_n4;
        locals.var_t3_dn5 = assign9300_e4761_d_n5;
        locals.var_t3_dn6 = assign9300_e4761_d_n6;
        locals.var_t3_dn7 = assign9300_e4761_d_n7;
        locals.var_t3_dn8 = assign9300_e4761_d_n8;
        locals.var_t3_dn9 = assign9300_e4761_d_n9;
        locals.var_t3_dn10 = assign9300_e4761_d_n10;
        locals.var_t3_dn13 = assign9300_e4761_d_n13;

        let (assign9310_e4771, assign9310_e4771_d_n0, assign9310_e4771_d_n2, assign9310_e4771_d_n4, assign9310_e4771_d_n5, assign9310_e4771_d_n6, assign9310_e4771_d_n7, assign9310_e4771_d_n8, assign9310_e4771_d_n9, assign9310_e4771_d_n10, assign9310_e4771_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9310_e4767: f64 = (p.p347 / locals.var_t3);
        let assign9310_e4768: f64 = (1.0 + assign9310_e4767);
        let assign9310_e4769: f64 = (p.p346 * assign9310_e4768);
        (assign9310_e4769, (p.p346 * (-((p.p347 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign9310_e4771;
        locals.var_uc_depmue0_dn0 = assign9310_e4771_d_n0;
        locals.var_uc_depmue0_dn2 = assign9310_e4771_d_n2;
        locals.var_uc_depmue0_dn4 = assign9310_e4771_d_n4;
        locals.var_uc_depmue0_dn5 = assign9310_e4771_d_n5;
        locals.var_uc_depmue0_dn6 = assign9310_e4771_d_n6;
        locals.var_uc_depmue0_dn7 = assign9310_e4771_d_n7;
        locals.var_uc_depmue0_dn8 = assign9310_e4771_d_n8;
        locals.var_uc_depmue0_dn9 = assign9310_e4771_d_n9;
        locals.var_uc_depmue0_dn10 = assign9310_e4771_d_n10;
        locals.var_uc_depmue0_dn13 = assign9310_e4771_d_n13;

        let assign9320_e4774: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard188 = assign9320_e4774;

        let (assign9330_e4780, assign9330_e4780_d_n0, assign9330_e4780_d_n2, assign9330_e4780_d_n4, assign9330_e4780_d_n5, assign9330_e4780_d_n6, assign9330_e4780_d_n7, assign9330_e4780_d_n8, assign9330_e4780_d_n9, assign9330_e4780_d_n10, assign9330_e4780_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard188 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign9330_e4780;
        locals.var_uc_depmue0_dn0 = assign9330_e4780_d_n0;
        locals.var_uc_depmue0_dn2 = assign9330_e4780_d_n2;
        locals.var_uc_depmue0_dn4 = assign9330_e4780_d_n4;
        locals.var_uc_depmue0_dn5 = assign9330_e4780_d_n5;
        locals.var_uc_depmue0_dn6 = assign9330_e4780_d_n6;
        locals.var_uc_depmue0_dn7 = assign9330_e4780_d_n7;
        locals.var_uc_depmue0_dn8 = assign9330_e4780_d_n8;
        locals.var_uc_depmue0_dn9 = assign9330_e4780_d_n9;
        locals.var_uc_depmue0_dn10 = assign9330_e4780_d_n10;
        locals.var_uc_depmue0_dn13 = assign9330_e4780_d_n13;

        let (assign9340_e4786, assign9340_e4786_d_n0, assign9340_e4786_d_n2, assign9340_e4786_d_n4, assign9340_e4786_d_n5, assign9340_e4786_d_n6, assign9340_e4786_d_n7, assign9340_e4786_d_n8, assign9340_e4786_d_n9, assign9340_e4786_d_n10, assign9340_e4786_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9340_e4784: f64 = (locals.var_lg).powf(p.p351);
        (assign9340_e4784, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9340_e4786;
        locals.var_t3_dn0 = assign9340_e4786_d_n0;
        locals.var_t3_dn2 = assign9340_e4786_d_n2;
        locals.var_t3_dn4 = assign9340_e4786_d_n4;
        locals.var_t3_dn5 = assign9340_e4786_d_n5;
        locals.var_t3_dn6 = assign9340_e4786_d_n6;
        locals.var_t3_dn7 = assign9340_e4786_d_n7;
        locals.var_t3_dn8 = assign9340_e4786_d_n8;
        locals.var_t3_dn9 = assign9340_e4786_d_n9;
        locals.var_t3_dn10 = assign9340_e4786_d_n10;
        locals.var_t3_dn13 = assign9340_e4786_d_n13;

        let (assign9350_e4796, assign9350_e4796_d_n0, assign9350_e4796_d_n2, assign9350_e4796_d_n4, assign9350_e4796_d_n5, assign9350_e4796_d_n6, assign9350_e4796_d_n7, assign9350_e4796_d_n8, assign9350_e4796_d_n9, assign9350_e4796_d_n10, assign9350_e4796_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9350_e4792: f64 = (p.p350 / locals.var_t3);
        let assign9350_e4793: f64 = (1.0 + assign9350_e4792);
        let assign9350_e4794: f64 = (p.p349 * assign9350_e4793);
        (assign9350_e4794, (p.p349 * (-((p.p350 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn13,)
    }
};
        locals.var_uc_depmue1 = assign9350_e4796;
        locals.var_uc_depmue1_dn0 = assign9350_e4796_d_n0;
        locals.var_uc_depmue1_dn2 = assign9350_e4796_d_n2;
        locals.var_uc_depmue1_dn4 = assign9350_e4796_d_n4;
        locals.var_uc_depmue1_dn5 = assign9350_e4796_d_n5;
        locals.var_uc_depmue1_dn6 = assign9350_e4796_d_n6;
        locals.var_uc_depmue1_dn7 = assign9350_e4796_d_n7;
        locals.var_uc_depmue1_dn8 = assign9350_e4796_d_n8;
        locals.var_uc_depmue1_dn9 = assign9350_e4796_d_n9;
        locals.var_uc_depmue1_dn10 = assign9350_e4796_d_n10;
        locals.var_uc_depmue1_dn13 = assign9350_e4796_d_n13;

        let assign9360_e4799: f64 = if locals.var_uc_depmue1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign9360_e4799;

        let (assign9370_e4805, assign9370_e4805_d_n0, assign9370_e4805_d_n2, assign9370_e4805_d_n4, assign9370_e4805_d_n5, assign9370_e4805_d_n6, assign9370_e4805_d_n7, assign9370_e4805_d_n8, assign9370_e4805_d_n9, assign9370_e4805_d_n10, assign9370_e4805_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard189 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn13,)
    }
};
        locals.var_uc_depmue1 = assign9370_e4805;
        locals.var_uc_depmue1_dn0 = assign9370_e4805_d_n0;
        locals.var_uc_depmue1_dn2 = assign9370_e4805_d_n2;
        locals.var_uc_depmue1_dn4 = assign9370_e4805_d_n4;
        locals.var_uc_depmue1_dn5 = assign9370_e4805_d_n5;
        locals.var_uc_depmue1_dn6 = assign9370_e4805_d_n6;
        locals.var_uc_depmue1_dn7 = assign9370_e4805_d_n7;
        locals.var_uc_depmue1_dn8 = assign9370_e4805_d_n8;
        locals.var_uc_depmue1_dn9 = assign9370_e4805_d_n9;
        locals.var_uc_depmue1_dn10 = assign9370_e4805_d_n10;
        locals.var_uc_depmue1_dn13 = assign9370_e4805_d_n13;

        let (assign9380_e4811, assign9380_e4811_d_n0, assign9380_e4811_d_n2, assign9380_e4811_d_n4, assign9380_e4811_d_n5, assign9380_e4811_d_n6, assign9380_e4811_d_n7, assign9380_e4811_d_n8, assign9380_e4811_d_n9, assign9380_e4811_d_n10, assign9380_e4811_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9380_e4809: f64 = (locals.var_lg).powf(p.p357);
        (assign9380_e4809, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9380_e4811;
        locals.var_t3_dn0 = assign9380_e4811_d_n0;
        locals.var_t3_dn2 = assign9380_e4811_d_n2;
        locals.var_t3_dn4 = assign9380_e4811_d_n4;
        locals.var_t3_dn5 = assign9380_e4811_d_n5;
        locals.var_t3_dn6 = assign9380_e4811_d_n6;
        locals.var_t3_dn7 = assign9380_e4811_d_n7;
        locals.var_t3_dn8 = assign9380_e4811_d_n8;
        locals.var_t3_dn9 = assign9380_e4811_d_n9;
        locals.var_t3_dn10 = assign9380_e4811_d_n10;
        locals.var_t3_dn13 = assign9380_e4811_d_n13;

        let (assign9390_e4821, assign9390_e4821_d_n0, assign9390_e4821_d_n2, assign9390_e4821_d_n4, assign9390_e4821_d_n5, assign9390_e4821_d_n6, assign9390_e4821_d_n7, assign9390_e4821_d_n8, assign9390_e4821_d_n9, assign9390_e4821_d_n10, assign9390_e4821_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9390_e4817: f64 = (p.p356 / locals.var_t3);
        let assign9390_e4818: f64 = (1.0 + assign9390_e4817);
        let assign9390_e4819: f64 = (p.p354 * assign9390_e4818);
        (assign9390_e4819, (p.p354 * (-((p.p356 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign9390_e4821;
        locals.var_uc_depmueback0_dn0 = assign9390_e4821_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9390_e4821_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9390_e4821_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9390_e4821_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9390_e4821_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9390_e4821_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9390_e4821_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9390_e4821_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9390_e4821_d_n10;
        locals.var_uc_depmueback0_dn13 = assign9390_e4821_d_n13;

        let assign9400_e4824: f64 = if locals.var_uc_depmueback0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign9400_e4824;

        let (assign9410_e4830, assign9410_e4830_d_n0, assign9410_e4830_d_n2, assign9410_e4830_d_n4, assign9410_e4830_d_n5, assign9410_e4830_d_n6, assign9410_e4830_d_n7, assign9410_e4830_d_n8, assign9410_e4830_d_n9, assign9410_e4830_d_n10, assign9410_e4830_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard190 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign9410_e4830;
        locals.var_uc_depmueback0_dn0 = assign9410_e4830_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9410_e4830_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9410_e4830_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9410_e4830_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9410_e4830_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9410_e4830_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9410_e4830_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9410_e4830_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9410_e4830_d_n10;
        locals.var_uc_depmueback0_dn13 = assign9410_e4830_d_n13;

        let (assign9420_e4836, assign9420_e4836_d_n0, assign9420_e4836_d_n2, assign9420_e4836_d_n4, assign9420_e4836_d_n5, assign9420_e4836_d_n6, assign9420_e4836_d_n7, assign9420_e4836_d_n8, assign9420_e4836_d_n9, assign9420_e4836_d_n10, assign9420_e4836_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9420_e4834: f64 = (locals.var_lg).powf(p.p359);
        (assign9420_e4834, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9420_e4836;
        locals.var_t3_dn0 = assign9420_e4836_d_n0;
        locals.var_t3_dn2 = assign9420_e4836_d_n2;
        locals.var_t3_dn4 = assign9420_e4836_d_n4;
        locals.var_t3_dn5 = assign9420_e4836_d_n5;
        locals.var_t3_dn6 = assign9420_e4836_d_n6;
        locals.var_t3_dn7 = assign9420_e4836_d_n7;
        locals.var_t3_dn8 = assign9420_e4836_d_n8;
        locals.var_t3_dn9 = assign9420_e4836_d_n9;
        locals.var_t3_dn10 = assign9420_e4836_d_n10;
        locals.var_t3_dn13 = assign9420_e4836_d_n13;

        let (assign9430_e4846, assign9430_e4846_d_n0, assign9430_e4846_d_n2, assign9430_e4846_d_n4, assign9430_e4846_d_n5, assign9430_e4846_d_n6, assign9430_e4846_d_n7, assign9430_e4846_d_n8, assign9430_e4846_d_n9, assign9430_e4846_d_n10, assign9430_e4846_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9430_e4842: f64 = (p.p358 / locals.var_t3);
        let assign9430_e4843: f64 = (1.0 + assign9430_e4842);
        let assign9430_e4844: f64 = (p.p355 * assign9430_e4843);
        (assign9430_e4844, (p.p355 * (-((p.p358 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn13,)
    }
};
        locals.var_uc_depmueback1 = assign9430_e4846;
        locals.var_uc_depmueback1_dn0 = assign9430_e4846_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9430_e4846_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9430_e4846_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9430_e4846_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9430_e4846_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9430_e4846_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9430_e4846_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9430_e4846_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9430_e4846_d_n10;
        locals.var_uc_depmueback1_dn13 = assign9430_e4846_d_n13;

        let assign9440_e4849: f64 = if locals.var_uc_depmueback1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign9440_e4849;

        let (assign9450_e4855, assign9450_e4855_d_n0, assign9450_e4855_d_n2, assign9450_e4855_d_n4, assign9450_e4855_d_n5, assign9450_e4855_d_n6, assign9450_e4855_d_n7, assign9450_e4855_d_n8, assign9450_e4855_d_n9, assign9450_e4855_d_n10, assign9450_e4855_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn13,)
    }
};
        locals.var_uc_depmueback1 = assign9450_e4855;
        locals.var_uc_depmueback1_dn0 = assign9450_e4855_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9450_e4855_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9450_e4855_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9450_e4855_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9450_e4855_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9450_e4855_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9450_e4855_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9450_e4855_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9450_e4855_d_n10;
        locals.var_uc_depmueback1_dn13 = assign9450_e4855_d_n13;

        let (assign9460_e4861, assign9460_e4861_d_n0, assign9460_e4861_d_n2, assign9460_e4861_d_n4, assign9460_e4861_d_n5, assign9460_e4861_d_n6, assign9460_e4861_d_n7, assign9460_e4861_d_n8, assign9460_e4861_d_n9, assign9460_e4861_d_n10, assign9460_e4861_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9460_e4859: f64 = (locals.var_lg).powf(p.p373);
        (assign9460_e4859, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9460_e4861;
        locals.var_t3_dn0 = assign9460_e4861_d_n0;
        locals.var_t3_dn2 = assign9460_e4861_d_n2;
        locals.var_t3_dn4 = assign9460_e4861_d_n4;
        locals.var_t3_dn5 = assign9460_e4861_d_n5;
        locals.var_t3_dn6 = assign9460_e4861_d_n6;
        locals.var_t3_dn7 = assign9460_e4861_d_n7;
        locals.var_t3_dn8 = assign9460_e4861_d_n8;
        locals.var_t3_dn9 = assign9460_e4861_d_n9;
        locals.var_t3_dn10 = assign9460_e4861_d_n10;
        locals.var_t3_dn13 = assign9460_e4861_d_n13;

        let (assign9470_e4871, assign9470_e4871_d_n0, assign9470_e4871_d_n2, assign9470_e4871_d_n4, assign9470_e4871_d_n5, assign9470_e4871_d_n6, assign9470_e4871_d_n7, assign9470_e4871_d_n8, assign9470_e4871_d_n9, assign9470_e4871_d_n10, assign9470_e4871_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9470_e4867: f64 = (p.p372 / locals.var_t3);
        let assign9470_e4868: f64 = (1.0 + assign9470_e4867);
        let assign9470_e4869: f64 = (locals.var_uc_depvdsef1 * assign9470_e4868);
        (assign9470_e4869, ((locals.var_uc_depvdsef1_dn0 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn2 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn4 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn5 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn6 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn7 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn8 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn9 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn10 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn13 * assign9470_e4868) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn13,)
    }
};
        locals.var_uc_depvdsef1 = assign9470_e4871;
        locals.var_uc_depvdsef1_dn0 = assign9470_e4871_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9470_e4871_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9470_e4871_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9470_e4871_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9470_e4871_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9470_e4871_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9470_e4871_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9470_e4871_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9470_e4871_d_n10;
        locals.var_uc_depvdsef1_dn13 = assign9470_e4871_d_n13;

        let (assign9480_e4877, assign9480_e4877_d_n0, assign9480_e4877_d_n2, assign9480_e4877_d_n4, assign9480_e4877_d_n5, assign9480_e4877_d_n6, assign9480_e4877_d_n7, assign9480_e4877_d_n8, assign9480_e4877_d_n9, assign9480_e4877_d_n10, assign9480_e4877_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9480_e4875: f64 = (locals.var_lg).powf(p.p375);
        (assign9480_e4875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign9480_e4877;
        locals.var_t3_dn0 = assign9480_e4877_d_n0;
        locals.var_t3_dn2 = assign9480_e4877_d_n2;
        locals.var_t3_dn4 = assign9480_e4877_d_n4;
        locals.var_t3_dn5 = assign9480_e4877_d_n5;
        locals.var_t3_dn6 = assign9480_e4877_d_n6;
        locals.var_t3_dn7 = assign9480_e4877_d_n7;
        locals.var_t3_dn8 = assign9480_e4877_d_n8;
        locals.var_t3_dn9 = assign9480_e4877_d_n9;
        locals.var_t3_dn10 = assign9480_e4877_d_n10;
        locals.var_t3_dn13 = assign9480_e4877_d_n13;

        let (assign9490_e4887, assign9490_e4887_d_n0, assign9490_e4887_d_n2, assign9490_e4887_d_n4, assign9490_e4887_d_n5, assign9490_e4887_d_n6, assign9490_e4887_d_n7, assign9490_e4887_d_n8, assign9490_e4887_d_n9, assign9490_e4887_d_n10, assign9490_e4887_d_n13,) = {
    if (locals.var_guard185 != 0.0) {
        let assign9490_e4883: f64 = (p.p374 / locals.var_t3);
        let assign9490_e4884: f64 = (1.0 + assign9490_e4883);
        let assign9490_e4885: f64 = (locals.var_uc_depvdsef2 * assign9490_e4884);
        (assign9490_e4885, ((locals.var_uc_depvdsef2_dn0 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn2 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn4 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn5 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn6 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn7 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn8 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn9 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn10 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn13 * assign9490_e4884) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign9490_e4887;
        locals.var_uc_depvdsef2_dn0 = assign9490_e4887_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9490_e4887_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9490_e4887_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9490_e4887_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9490_e4887_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9490_e4887_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9490_e4887_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9490_e4887_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9490_e4887_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign9490_e4887_d_n13;

        let assign9500_e4890: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign9500_e4890;

        let (assign9510_e4896, assign9510_e4896_d_n0, assign9510_e4896_d_n2, assign9510_e4896_d_n4, assign9510_e4896_d_n5, assign9510_e4896_d_n6, assign9510_e4896_d_n7, assign9510_e4896_d_n8, assign9510_e4896_d_n9, assign9510_e4896_d_n10, assign9510_e4896_d_n13,) = {
    if ((locals.var_guard185 != 0.0) && (locals.var_guard192 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign9510_e4896;
        locals.var_uc_depvdsef2_dn0 = assign9510_e4896_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9510_e4896_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9510_e4896_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9510_e4896_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9510_e4896_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9510_e4896_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9510_e4896_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9510_e4896_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9510_e4896_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign9510_e4896_d_n13;

    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9520_e4901, assign9520_e4901_d_n0, assign9520_e4901_d_n2, assign9520_e4901_d_n4, assign9520_e4901_d_n5, assign9520_e4901_d_n6, assign9520_e4901_d_n7, assign9520_e4901_d_n8, assign9520_e4901_d_n9, assign9520_e4901_d_n10, assign9520_e4901_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn13,)
    }
};
        locals.var_uc_ndepm = assign9520_e4901;
        locals.var_uc_ndepm_dn0 = assign9520_e4901_d_n0;
        locals.var_uc_ndepm_dn2 = assign9520_e4901_d_n2;
        locals.var_uc_ndepm_dn4 = assign9520_e4901_d_n4;
        locals.var_uc_ndepm_dn5 = assign9520_e4901_d_n5;
        locals.var_uc_ndepm_dn6 = assign9520_e4901_d_n6;
        locals.var_uc_ndepm_dn7 = assign9520_e4901_d_n7;
        locals.var_uc_ndepm_dn8 = assign9520_e4901_d_n8;
        locals.var_uc_ndepm_dn9 = assign9520_e4901_d_n9;
        locals.var_uc_ndepm_dn10 = assign9520_e4901_d_n10;
        locals.var_uc_ndepm_dn13 = assign9520_e4901_d_n13;

        let (assign9530_e4906, assign9530_e4906_d_n0, assign9530_e4906_d_n2, assign9530_e4906_d_n4, assign9530_e4906_d_n5, assign9530_e4906_d_n6, assign9530_e4906_d_n7, assign9530_e4906_d_n8, assign9530_e4906_d_n9, assign9530_e4906_d_n10, assign9530_e4906_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn13,)
    }
};
        locals.var_uc_depvmax = assign9530_e4906;
        locals.var_uc_depvmax_dn0 = assign9530_e4906_d_n0;
        locals.var_uc_depvmax_dn2 = assign9530_e4906_d_n2;
        locals.var_uc_depvmax_dn4 = assign9530_e4906_d_n4;
        locals.var_uc_depvmax_dn5 = assign9530_e4906_d_n5;
        locals.var_uc_depvmax_dn6 = assign9530_e4906_d_n6;
        locals.var_uc_depvmax_dn7 = assign9530_e4906_d_n7;
        locals.var_uc_depvmax_dn8 = assign9530_e4906_d_n8;
        locals.var_uc_depvmax_dn9 = assign9530_e4906_d_n9;
        locals.var_uc_depvmax_dn10 = assign9530_e4906_d_n10;
        locals.var_uc_depvmax_dn13 = assign9530_e4906_d_n13;

        let (assign9540_e4911, assign9540_e4911_d_n0, assign9540_e4911_d_n2, assign9540_e4911_d_n4, assign9540_e4911_d_n5, assign9540_e4911_d_n6, assign9540_e4911_d_n7, assign9540_e4911_d_n8, assign9540_e4911_d_n9, assign9540_e4911_d_n10, assign9540_e4911_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn13,)
    }
};
        locals.var_uc_depleak = assign9540_e4911;
        locals.var_uc_depleak_dn0 = assign9540_e4911_d_n0;
        locals.var_uc_depleak_dn2 = assign9540_e4911_d_n2;
        locals.var_uc_depleak_dn4 = assign9540_e4911_d_n4;
        locals.var_uc_depleak_dn5 = assign9540_e4911_d_n5;
        locals.var_uc_depleak_dn6 = assign9540_e4911_d_n6;
        locals.var_uc_depleak_dn7 = assign9540_e4911_d_n7;
        locals.var_uc_depleak_dn8 = assign9540_e4911_d_n8;
        locals.var_uc_depleak_dn9 = assign9540_e4911_d_n9;
        locals.var_uc_depleak_dn10 = assign9540_e4911_d_n10;
        locals.var_uc_depleak_dn13 = assign9540_e4911_d_n13;

        let (assign9550_e4916, assign9550_e4916_d_n0, assign9550_e4916_d_n2, assign9550_e4916_d_n4, assign9550_e4916_d_n5, assign9550_e4916_d_n6, assign9550_e4916_d_n7, assign9550_e4916_d_n8, assign9550_e4916_d_n9, assign9550_e4916_d_n10, assign9550_e4916_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn13,)
    }
};
        locals.var_uc_depmue0 = assign9550_e4916;
        locals.var_uc_depmue0_dn0 = assign9550_e4916_d_n0;
        locals.var_uc_depmue0_dn2 = assign9550_e4916_d_n2;
        locals.var_uc_depmue0_dn4 = assign9550_e4916_d_n4;
        locals.var_uc_depmue0_dn5 = assign9550_e4916_d_n5;
        locals.var_uc_depmue0_dn6 = assign9550_e4916_d_n6;
        locals.var_uc_depmue0_dn7 = assign9550_e4916_d_n7;
        locals.var_uc_depmue0_dn8 = assign9550_e4916_d_n8;
        locals.var_uc_depmue0_dn9 = assign9550_e4916_d_n9;
        locals.var_uc_depmue0_dn10 = assign9550_e4916_d_n10;
        locals.var_uc_depmue0_dn13 = assign9550_e4916_d_n13;

        let (assign9560_e4921, assign9560_e4921_d_n0, assign9560_e4921_d_n2, assign9560_e4921_d_n4, assign9560_e4921_d_n5, assign9560_e4921_d_n6, assign9560_e4921_d_n7, assign9560_e4921_d_n8, assign9560_e4921_d_n9, assign9560_e4921_d_n10, assign9560_e4921_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn13,)
    }
};
        locals.var_uc_depmue1 = assign9560_e4921;
        locals.var_uc_depmue1_dn0 = assign9560_e4921_d_n0;
        locals.var_uc_depmue1_dn2 = assign9560_e4921_d_n2;
        locals.var_uc_depmue1_dn4 = assign9560_e4921_d_n4;
        locals.var_uc_depmue1_dn5 = assign9560_e4921_d_n5;
        locals.var_uc_depmue1_dn6 = assign9560_e4921_d_n6;
        locals.var_uc_depmue1_dn7 = assign9560_e4921_d_n7;
        locals.var_uc_depmue1_dn8 = assign9560_e4921_d_n8;
        locals.var_uc_depmue1_dn9 = assign9560_e4921_d_n9;
        locals.var_uc_depmue1_dn10 = assign9560_e4921_d_n10;
        locals.var_uc_depmue1_dn13 = assign9560_e4921_d_n13;

        let (assign9570_e4926, assign9570_e4926_d_n0, assign9570_e4926_d_n2, assign9570_e4926_d_n4, assign9570_e4926_d_n5, assign9570_e4926_d_n6, assign9570_e4926_d_n7, assign9570_e4926_d_n8, assign9570_e4926_d_n9, assign9570_e4926_d_n10, assign9570_e4926_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn13,)
    }
};
        locals.var_uc_depmueback0 = assign9570_e4926;
        locals.var_uc_depmueback0_dn0 = assign9570_e4926_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9570_e4926_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9570_e4926_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9570_e4926_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9570_e4926_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9570_e4926_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9570_e4926_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9570_e4926_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9570_e4926_d_n10;
        locals.var_uc_depmueback0_dn13 = assign9570_e4926_d_n13;

        let (assign9580_e4931, assign9580_e4931_d_n0, assign9580_e4931_d_n2, assign9580_e4931_d_n4, assign9580_e4931_d_n5, assign9580_e4931_d_n6, assign9580_e4931_d_n7, assign9580_e4931_d_n8, assign9580_e4931_d_n9, assign9580_e4931_d_n10, assign9580_e4931_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn13,)
    }
};
        locals.var_uc_depmueback1 = assign9580_e4931;
        locals.var_uc_depmueback1_dn0 = assign9580_e4931_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9580_e4931_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9580_e4931_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9580_e4931_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9580_e4931_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9580_e4931_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9580_e4931_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9580_e4931_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9580_e4931_d_n10;
        locals.var_uc_depmueback1_dn13 = assign9580_e4931_d_n13;

        let (assign9590_e4936, assign9590_e4936_d_n0, assign9590_e4936_d_n2, assign9590_e4936_d_n4, assign9590_e4936_d_n5, assign9590_e4936_d_n6, assign9590_e4936_d_n7, assign9590_e4936_d_n8, assign9590_e4936_d_n9, assign9590_e4936_d_n10, assign9590_e4936_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn13,)
    }
};
        locals.var_uc_depvdsef1 = assign9590_e4936;
        locals.var_uc_depvdsef1_dn0 = assign9590_e4936_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9590_e4936_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9590_e4936_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9590_e4936_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9590_e4936_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9590_e4936_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9590_e4936_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9590_e4936_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9590_e4936_d_n10;
        locals.var_uc_depvdsef1_dn13 = assign9590_e4936_d_n13;

        let (assign9600_e4941, assign9600_e4941_d_n0, assign9600_e4941_d_n2, assign9600_e4941_d_n4, assign9600_e4941_d_n5, assign9600_e4941_d_n6, assign9600_e4941_d_n7, assign9600_e4941_d_n8, assign9600_e4941_d_n9, assign9600_e4941_d_n10, assign9600_e4941_d_n13,) = {
    if (locals.var_guard185 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn13,)
    }
};
        locals.var_uc_depvdsef2 = assign9600_e4941;
        locals.var_uc_depvdsef2_dn0 = assign9600_e4941_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9600_e4941_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9600_e4941_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9600_e4941_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9600_e4941_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9600_e4941_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9600_e4941_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9600_e4941_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9600_e4941_d_n10;
        locals.var_uc_depvdsef2_dn13 = assign9600_e4941_d_n13;

        let assign10120_e5314: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign10120_e5316: f64 = if assign10120_e5314 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign10120_e5316;

        let (assign10130_e5322,) = {
    if (locals.var_guard244 != 0.0) {
        let assign10130_e5320: f64 = (1.0 / locals.var_uc_xldld);
        (assign10130_e5320,)
    } else {
        (locals.var_uc_xpdv,)
    }
};
        locals.var_uc_xpdv = assign10130_e5322;

        let assign10150_e5350: f64 = if ((p.p40 == 1.0) && (((p.p19 > 0.0) && (locals.var_uc_nover == 0.0)) || ((p.p18 > 0.0) && (locals.var_uc_novers == 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard246 = assign10150_e5350;

        let (assign10160_e5354,) = {
    if (locals.var_guard246 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10160_e5354;

        let (assign10170_e5359,) = {
    if (locals.var_guard246 == 0.0) {
        (p.p40,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10170_e5359;

        let assign10180_e5362: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign10180_e5362;

        let (assign10190_e5371,) = {
    if (locals.var_guard247 != 0.0) {
        let (assign10190_e5369,) = {
            if (p.p19 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10190_e5369,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10190_e5371;

        let (assign10200_e5380,) = {
    if (locals.var_guard247 != 0.0) {
        let (assign10200_e5378,) = {
            if (p.p18 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10200_e5378,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10200_e5380;

        let assign10210_e5387: f64 = if ((p.p17 == 0.0) || (p.p17 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard248 = assign10210_e5387;

        let (assign10220_e5394,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10220_e5394;

        let (assign10230_e5401,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10230_e5401;

        let (assign10240_e5433, assign10240_e5433_d_n0, assign10240_e5433_d_n2, assign10240_e5433_d_n4, assign10240_e5433_d_n5, assign10240_e5433_d_n6, assign10240_e5433_d_n7, assign10240_e5433_d_n8, assign10240_e5433_d_n9, assign10240_e5433_d_n10, assign10240_e5433_d_n13,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let assign10240_e5409: f64 = (p.p130 * p.p2);
        let assign10240_e5411: f64 = (assign10240_e5409 * p.p7);
        let assign10240_e5414: f64 = (locals.var_uc_rd + locals.var_uc_rdvd);
        let assign10240_e5417: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign10240_e5419: f64 = (assign10240_e5417 * 1000000.0);
        let assign10240_e5421: f64 = (assign10240_e5419 + locals.var_uc_rdict1);
        let assign10240_e5422: f64 = (assign10240_e5414 * assign10240_e5421);
        let assign10240_e5425: f64 = (p.p68 * p.p100);
        let assign10240_e5427: f64 = (assign10240_e5425 * 1000000.0);
        let assign10240_e5429: f64 = (assign10240_e5427 + p.p101);
        let assign10240_e5430: f64 = (assign10240_e5422 * assign10240_e5429);
        let assign10240_e5431: f64 = (assign10240_e5411 + assign10240_e5430);
        (assign10240_e5431, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign10240_e5433;
        locals.var_t1_dn0 = assign10240_e5433_d_n0;
        locals.var_t1_dn2 = assign10240_e5433_d_n2;
        locals.var_t1_dn4 = assign10240_e5433_d_n4;
        locals.var_t1_dn5 = assign10240_e5433_d_n5;
        locals.var_t1_dn6 = assign10240_e5433_d_n6;
        locals.var_t1_dn7 = assign10240_e5433_d_n7;
        locals.var_t1_dn8 = assign10240_e5433_d_n8;
        locals.var_t1_dn9 = assign10240_e5433_d_n9;
        locals.var_t1_dn10 = assign10240_e5433_d_n10;
        locals.var_t1_dn13 = assign10240_e5433_d_n13;

        let (assign10250_e5446,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let (assign10250_e5444,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10250_e5444,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10250_e5446;

        let (assign10260_e5476, assign10260_e5476_d_n0, assign10260_e5476_d_n2, assign10260_e5476_d_n4, assign10260_e5476_d_n5, assign10260_e5476_d_n6, assign10260_e5476_d_n7, assign10260_e5476_d_n8, assign10260_e5476_d_n9, assign10260_e5476_d_n10, assign10260_e5476_d_n13,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let assign10260_e5454: f64 = (p.p131 * p.p3);
        let assign10260_e5456: f64 = (assign10260_e5454 * p.p7);
        let assign10260_e5460: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign10260_e5462: f64 = (assign10260_e5460 * 1000000.0);
        let assign10260_e5464: f64 = (assign10260_e5462 + locals.var_uc_rdict1);
        let assign10260_e5465: f64 = (locals.var_uc_rs * assign10260_e5464);
        let assign10260_e5468: f64 = (p.p70 * p.p100);
        let assign10260_e5470: f64 = (assign10260_e5468 * 1000000.0);
        let assign10260_e5472: f64 = (assign10260_e5470 + p.p101);
        let assign10260_e5473: f64 = (assign10260_e5465 * assign10260_e5472);
        let assign10260_e5474: f64 = (assign10260_e5456 + assign10260_e5473);
        (assign10260_e5474, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign10260_e5476;
        locals.var_t1_dn0 = assign10260_e5476_d_n0;
        locals.var_t1_dn2 = assign10260_e5476_d_n2;
        locals.var_t1_dn4 = assign10260_e5476_d_n4;
        locals.var_t1_dn5 = assign10260_e5476_d_n5;
        locals.var_t1_dn6 = assign10260_e5476_d_n6;
        locals.var_t1_dn7 = assign10260_e5476_d_n7;
        locals.var_t1_dn8 = assign10260_e5476_d_n8;
        locals.var_t1_dn9 = assign10260_e5476_d_n9;
        locals.var_t1_dn10 = assign10260_e5476_d_n10;
        locals.var_t1_dn13 = assign10260_e5476_d_n13;

        let (assign10270_e5489,) = {
    if ((locals.var_guard247 == 0.0) && (locals.var_guard248 == 0.0)) {
        let (assign10270_e5487,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10270_e5487,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10270_e5489;

        let assign10280_e5492: f64 = (p.p12 / 1e-6);
        locals.var_mks_nsubcdfm = assign10280_e5492;

        let assign10290_e5495: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign10290_e5495;

        let assign10300_e5498: f64 = (locals.var_uc_nsubc / 1e-6);
        locals.var_uc_nsubc = assign10300_e5498;

        let assign10310_e5501: f64 = (locals.var_uc_nsubp / 1e-6);
        locals.var_uc_nsubp = assign10310_e5501;

        let assign10320_e5504: f64 = (locals.var_uc_nsti / 1e-6);
        locals.var_uc_nsti = assign10320_e5504;

        let assign10330_e5507: f64 = (locals.var_uc_nover / 1e-6);
        locals.var_uc_nover = assign10330_e5507;

        let assign10340_e5510: f64 = (locals.var_uc_novers / 1e-6);
        locals.var_uc_novers = assign10340_e5510;

        let assign10350_e5513: f64 = (locals.var_uc_nsubpsti1 / 100.0);
        locals.var_uc_nsubpsti1 = assign10350_e5513;

        let assign10360_e5516: f64 = (locals.var_uc_muesti1 / 100.0);
        locals.var_uc_muesti1 = assign10360_e5516;

        let assign10370_e5519: f64 = (locals.var_uc_vmax / 100.0);
        locals.var_uc_vmax = assign10370_e5519;

        let assign10380_e5522: f64 = (locals.var_uc_wfc * 10000.0);
        locals.var_uc_wfc = assign10380_e5522;

        let assign10390_e5525: f64 = (locals.var_uc_glksd1 / 100.0);
        locals.var_uc_glksd1 = assign10390_e5525;

        let assign10400_e5528: f64 = (locals.var_uc_glksd2 * 100.0);
        locals.var_uc_glksd2 = assign10400_e5528;

        let assign10410_e5531: f64 = (locals.var_uc_gleak2 * 100.0);
        locals.var_uc_gleak2 = assign10410_e5531;

        let assign10420_e5534: f64 = (locals.var_uc_glkb2 * 100.0);
        locals.var_uc_glkb2 = assign10420_e5534;

        let assign10430_e5537: f64 = (locals.var_uc_fn2 * 100.0);
        locals.var_uc_fn2 = assign10430_e5537;

        let assign10440_e5540: f64 = (locals.var_uc_gidl1 / 10.0);
        locals.var_uc_gidl1 = assign10440_e5540;

        let assign10450_e5543: f64 = (locals.var_uc_gidl2 * 100.0);
        locals.var_uc_gidl2 = assign10450_e5543;

        let assign10460_e5546: f64 = (locals.var_uc_nfalp / 100.0);
        locals.var_uc_nfalp = assign10460_e5546;

        let assign10480_e5552: f64 = (locals.var_uc_npext / 1e-6);
        locals.var_uc_npext = assign10480_e5552;

        let assign10490_e5555: f64 = (locals.var_uc_rd22 / 100.0);
        locals.var_uc_rd22 = assign10490_e5555;

        let assign10500_e5558: f64 = (locals.var_uc_rd23 / 100.0);
        locals.var_uc_rd23 = assign10500_e5558;

        let assign10510_e5561: f64 = (locals.var_uc_rd24 / 100.0);
        locals.var_uc_rd24 = assign10510_e5561;

        let assign10520_e5564: f64 = (locals.var_uc_rdvd / 100.0);
        locals.var_uc_rdvd = assign10520_e5564;

        let assign10530_e5567: f64 = (locals.var_uc_rth0 / 100.0);
        locals.var_uc_rth0 = assign10530_e5567;

        let assign10540_e5569: f64 = (-locals.var_uc_vfbover);
        locals.var_uc_vfbover = assign10540_e5569;

        let assign10550_e5572: f64 = (locals.var_uc_depvmax / 100.0);
        locals.var_uc_depvmax = assign10550_e5572;
        locals.var_uc_depvmax_dn0 = (locals.var_uc_depvmax_dn0 / 100.0);
        locals.var_uc_depvmax_dn2 = (locals.var_uc_depvmax_dn2 / 100.0);
        locals.var_uc_depvmax_dn4 = (locals.var_uc_depvmax_dn4 / 100.0);
        locals.var_uc_depvmax_dn5 = (locals.var_uc_depvmax_dn5 / 100.0);
        locals.var_uc_depvmax_dn6 = (locals.var_uc_depvmax_dn6 / 100.0);
        locals.var_uc_depvmax_dn7 = (locals.var_uc_depvmax_dn7 / 100.0);
        locals.var_uc_depvmax_dn8 = (locals.var_uc_depvmax_dn8 / 100.0);
        locals.var_uc_depvmax_dn9 = (locals.var_uc_depvmax_dn9 / 100.0);
        locals.var_uc_depvmax_dn10 = (locals.var_uc_depvmax_dn10 / 100.0);
        locals.var_uc_depvmax_dn13 = (locals.var_uc_depvmax_dn13 / 100.0);

        locals.var_flg_nqs = p.p28;

        let (assign10570_e5583,) = {
    if ((p.p133 != 0.0) || (p.p134 != 0.0)) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        locals.var_flg_qy = assign10570_e5583;

        let assign10590_e5597: f64 = if (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard250 = assign10590_e5597;

        let (assign10600_e5601,) = {
    if (locals.var_guard250 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10600_e5601;

        let (assign10610_e5606,) = {
    if (locals.var_guard250 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10610_e5606;

        let assign10620_e5609: f64 = (locals.var_wg * locals.var_lg);
        locals.var_wlg = assign10620_e5609;

        let assign10630_e5612: f64 = (p.p289 * 1000000.0);
        locals.var_uc_gdld = assign10630_e5612;

        let assign10640_e5618: f64 = (locals.var_ktnom * 1e-7);
        let assign10640_e5619: f64 = (9.025e-5 + assign10640_e5618);
        let assign10640_e5620: f64 = (locals.var_ktnom * assign10640_e5619);
        let assign10640_e5621: f64 = (locals.var_uc_eg0 - assign10640_e5620);
        locals.var_egtnom = assign10640_e5621;

        let assign10650_e5624: f64 = (8.8541878e-12 * p.p267);
        locals.var_cecox = assign10650_e5624;

        locals.var_msc = locals.var_uc_scp22;

        let assign10670_e5628: f64 = if locals.var_uc_pgd1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign10670_e5628;

        let (assign10680_e5632,) = {
    if (locals.var_guard251 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10680_e5632;

    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10690_e5636,) = {
    if (locals.var_guard251 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10690_e5636;

        let (assign10700_e5641,) = {
    if (locals.var_guard251 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10700_e5641;

        let (assign10710_e5654,) = {
    if (locals.var_guard251 == 0.0) {
        let assign10710_e5647: f64 = (1.0 / locals.var_lg);
        let assign10710_e5648: f64 = (1.0 + assign10710_e5647);
        let assign10710_e5650: f64 = (assign10710_e5648).powf(p.p153);
        let assign10710_e5652: f64 = (assign10710_e5650 * locals.var_uc_pgd1);
        (assign10710_e5652,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10710_e5654;

        let assign10720_e5658: f64 = (locals.var_lg).powf(p.p229);
        let assign10720_e5660: f64 = (assign10720_e5658 * p.p230);
        let assign10720_e5661: f64 = (1.0 + assign10720_e5660);
        locals.var_clmmod = assign10720_e5661;

        let assign10730_e5666: f64 = (0.5 * p.p0);
        let assign10730_e5667: f64 = (p.p118 + assign10730_e5666);
        let assign10730_e5668: f64 = (1.0 / assign10730_e5667);
        let assign10730_e5673: f64 = (0.5 * p.p0);
        let assign10730_e5674: f64 = (p.p119 + assign10730_e5673);
        let assign10730_e5675: f64 = (1.0 / assign10730_e5674);
        let assign10730_e5676: f64 = (assign10730_e5668 + assign10730_e5675);
        locals.var_t1 = assign10730_e5676;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;

        let assign10740_e5679: f64 = (2.0 / locals.var_t1);
        locals.var_lod_half_ref = assign10740_e5679;
        locals.var_lod_half_ref_dn0 = (-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn2 = (-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn4 = (-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn5 = (-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn6 = (-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn7 = (-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn8 = (-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn9 = (-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn10 = (-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn13 = (-((2.0 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)));

        let assign10750_e5698: f64 = if (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard252 = assign10750_e5698;

        let (assign10760_e5702, assign10760_e5702_d_n0, assign10760_e5702_d_n2, assign10760_e5702_d_n4, assign10760_e5702_d_n5, assign10760_e5702_d_n6, assign10760_e5702_d_n7, assign10760_e5702_d_n8, assign10760_e5702_d_n9, assign10760_e5702_d_n10, assign10760_e5702_d_n13,) = {
    if (locals.var_guard252 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign10760_e5702;
        locals.var_t1_dn0 = assign10760_e5702_d_n0;
        locals.var_t1_dn2 = assign10760_e5702_d_n2;
        locals.var_t1_dn4 = assign10760_e5702_d_n4;
        locals.var_t1_dn5 = assign10760_e5702_d_n5;
        locals.var_t1_dn6 = assign10760_e5702_d_n6;
        locals.var_t1_dn7 = assign10760_e5702_d_n7;
        locals.var_t1_dn8 = assign10760_e5702_d_n8;
        locals.var_t1_dn9 = assign10760_e5702_d_n9;
        locals.var_t1_dn10 = assign10760_e5702_d_n10;
        locals.var_t1_dn13 = assign10760_e5702_d_n13;

        let (assign10770_e5706,) = {
    if (locals.var_guard252 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign10770_e5706;

        let mut assign10780_loop_guard: usize = 0;
        while {
            let assign10780_cond_e5711: f64 = if ((locals.var_guard252 != 0.0) && (locals.var_i < p.p7)) { 1.0 } else { 0.0 };
            assign10780_cond_e5711 != 0.0
        } {
            assign10780_loop_guard += 1;
            assert!(assign10780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10780_body0_e5743, assign10780_body0_e5743_d_n0, assign10780_body0_e5743_d_n2, assign10780_body0_e5743_d_n4, assign10780_body0_e5743_d_n5, assign10780_body0_e5743_d_n6, assign10780_body0_e5743_d_n7, assign10780_body0_e5743_d_n8, assign10780_body0_e5743_d_n9, assign10780_body0_e5743_d_n10, assign10780_body0_e5743_d_n13,) = {
    if (locals.var_guard252 != 0.0) {
        let assign10780_body0_e5718: f64 = (0.5 * p.p0);
        let assign10780_body0_e5719: f64 = (p.p8 + assign10780_body0_e5718);
        let assign10780_body0_e5723: f64 = (p.p10 + p.p0);
        let assign10780_body0_e5724: f64 = (locals.var_i * assign10780_body0_e5723);
        let assign10780_body0_e5725: f64 = (assign10780_body0_e5719 + assign10780_body0_e5724);
        let assign10780_body0_e5726: f64 = (1.0 / assign10780_body0_e5725);
        let assign10780_body0_e5727: f64 = (locals.var_t1 + assign10780_body0_e5726);
        let assign10780_body0_e5732: f64 = (0.5 * p.p0);
        let assign10780_body0_e5733: f64 = (p.p9 + assign10780_body0_e5732);
        let assign10780_body0_e5737: f64 = (p.p10 + p.p0);
        let assign10780_body0_e5738: f64 = (locals.var_i * assign10780_body0_e5737);
        let assign10780_body0_e5739: f64 = (assign10780_body0_e5733 + assign10780_body0_e5738);
        let assign10780_body0_e5740: f64 = (1.0 / assign10780_body0_e5739);
        let assign10780_body0_e5741: f64 = (assign10780_body0_e5727 + assign10780_body0_e5740);
        (assign10780_body0_e5741, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
            locals.var_t1 = assign10780_body0_e5743;
            locals.var_t1_dn0 = assign10780_body0_e5743_d_n0;
            locals.var_t1_dn2 = assign10780_body0_e5743_d_n2;
            locals.var_t1_dn4 = assign10780_body0_e5743_d_n4;
            locals.var_t1_dn5 = assign10780_body0_e5743_d_n5;
            locals.var_t1_dn6 = assign10780_body0_e5743_d_n6;
            locals.var_t1_dn7 = assign10780_body0_e5743_d_n7;
            locals.var_t1_dn8 = assign10780_body0_e5743_d_n8;
            locals.var_t1_dn9 = assign10780_body0_e5743_d_n9;
            locals.var_t1_dn10 = assign10780_body0_e5743_d_n10;
            locals.var_t1_dn13 = assign10780_body0_e5743_d_n13;
            let (assign10780_body1_e5749,) = {
    if (locals.var_guard252 != 0.0) {
        let assign10780_body1_e5747: f64 = (locals.var_i + 1.0);
        (assign10780_body1_e5747,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10780_body1_e5749;
        }

        let (assign10790_e5757, assign10790_e5757_d_n0, assign10790_e5757_d_n2, assign10790_e5757_d_n4, assign10790_e5757_d_n5, assign10790_e5757_d_n6, assign10790_e5757_d_n7, assign10790_e5757_d_n8, assign10790_e5757_d_n9, assign10790_e5757_d_n10, assign10790_e5757_d_n13,) = {
    if (locals.var_guard252 != 0.0) {
        let assign10790_e5753: f64 = (2.0 * p.p7);
        let assign10790_e5755: f64 = (assign10790_e5753 / locals.var_t1);
        (assign10790_e5755, (-((assign10790_e5753 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign10790_e5753 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn13,)
    }
};
        locals.var_lod_half = assign10790_e5757;
        locals.var_lod_half_dn0 = assign10790_e5757_d_n0;
        locals.var_lod_half_dn2 = assign10790_e5757_d_n2;
        locals.var_lod_half_dn4 = assign10790_e5757_d_n4;
        locals.var_lod_half_dn5 = assign10790_e5757_d_n5;
        locals.var_lod_half_dn6 = assign10790_e5757_d_n6;
        locals.var_lod_half_dn7 = assign10790_e5757_d_n7;
        locals.var_lod_half_dn8 = assign10790_e5757_d_n8;
        locals.var_lod_half_dn9 = assign10790_e5757_d_n9;
        locals.var_lod_half_dn10 = assign10790_e5757_d_n10;
        locals.var_lod_half_dn13 = assign10790_e5757_d_n13;

        let (assign10800_e5762, assign10800_e5762_d_n0, assign10800_e5762_d_n2, assign10800_e5762_d_n4, assign10800_e5762_d_n5, assign10800_e5762_d_n6, assign10800_e5762_d_n7, assign10800_e5762_d_n8, assign10800_e5762_d_n9, assign10800_e5762_d_n10, assign10800_e5762_d_n13,) = {
    if (locals.var_guard252 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn13,)
    }
};
        locals.var_lod_half = assign10800_e5762;
        locals.var_lod_half_dn0 = assign10800_e5762_d_n0;
        locals.var_lod_half_dn2 = assign10800_e5762_d_n2;
        locals.var_lod_half_dn4 = assign10800_e5762_d_n4;
        locals.var_lod_half_dn5 = assign10800_e5762_d_n5;
        locals.var_lod_half_dn6 = assign10800_e5762_d_n6;
        locals.var_lod_half_dn7 = assign10800_e5762_d_n7;
        locals.var_lod_half_dn8 = assign10800_e5762_d_n8;
        locals.var_lod_half_dn9 = assign10800_e5762_d_n9;
        locals.var_lod_half_dn10 = assign10800_e5762_d_n10;
        locals.var_lod_half_dn13 = assign10800_e5762_d_n13;

        locals.var_npexte = locals.var_uc_npext;
        locals.var_npexte_dn0 = 0.0;
        locals.var_npexte_dn2 = 0.0;
        locals.var_npexte_dn4 = 0.0;
        locals.var_npexte_dn5 = 0.0;
        locals.var_npexte_dn6 = 0.0;
        locals.var_npexte_dn7 = 0.0;
        locals.var_npexte_dn8 = 0.0;
        locals.var_npexte_dn9 = 0.0;
        locals.var_npexte_dn10 = 0.0;
        locals.var_npexte_dn13 = 0.0;

        locals.var_ef_mueph1 = locals.var_uc_mueph1;
        locals.var_ef_mueph1_dn0 = 0.0;
        locals.var_ef_mueph1_dn2 = 0.0;
        locals.var_ef_mueph1_dn4 = 0.0;
        locals.var_ef_mueph1_dn5 = 0.0;
        locals.var_ef_mueph1_dn6 = 0.0;
        locals.var_ef_mueph1_dn7 = 0.0;
        locals.var_ef_mueph1_dn8 = 0.0;
        locals.var_ef_mueph1_dn9 = 0.0;
        locals.var_ef_mueph1_dn10 = 0.0;
        locals.var_ef_mueph1_dn13 = 0.0;

        locals.var_ef_nsubp = locals.var_uc_nsubp;
        locals.var_ef_nsubp_dn0 = 0.0;
        locals.var_ef_nsubp_dn2 = 0.0;
        locals.var_ef_nsubp_dn4 = 0.0;
        locals.var_ef_nsubp_dn5 = 0.0;
        locals.var_ef_nsubp_dn6 = 0.0;
        locals.var_ef_nsubp_dn7 = 0.0;
        locals.var_ef_nsubp_dn8 = 0.0;
        locals.var_ef_nsubp_dn9 = 0.0;
        locals.var_ef_nsubp_dn10 = 0.0;
        locals.var_ef_nsubp_dn13 = 0.0;

        locals.var_ef_nsubc = locals.var_uc_nsubc;
        locals.var_ef_nsubc_dn0 = 0.0;
        locals.var_ef_nsubc_dn2 = 0.0;
        locals.var_ef_nsubc_dn4 = 0.0;
        locals.var_ef_nsubc_dn5 = 0.0;
        locals.var_ef_nsubc_dn6 = 0.0;
        locals.var_ef_nsubc_dn7 = 0.0;
        locals.var_ef_nsubc_dn8 = 0.0;
        locals.var_ef_nsubc_dn9 = 0.0;
        locals.var_ef_nsubc_dn10 = 0.0;
        locals.var_ef_nsubc_dn13 = 0.0;

        let assign10850_e5771: f64 = if ((p.p32 == 1.0) && (locals.var_nsubcdfm_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard253 = assign10850_e5771;

        let (assign10870_e5792, assign10870_e5792_d_n0, assign10870_e5792_d_n2, assign10870_e5792_d_n4, assign10870_e5792_d_n5, assign10870_e5792_d_n6, assign10870_e5792_d_n7, assign10870_e5792_d_n8, assign10870_e5792_d_n9, assign10870_e5792_d_n10, assign10870_e5792_d_n13,) = {
    if (locals.var_guard253 != 0.0) {
        let assign10870_e5783: f64 = (locals.var_mks_nsubcdfm).ln();
        let assign10870_e5785: f64 = (locals.var_ef_nsubc).ln();
        let assign10870_e5786: f64 = (assign10870_e5783 - assign10870_e5785);
        let assign10870_e5787: f64 = (p.p282 * assign10870_e5786);
        let assign10870_e5789: f64 = (assign10870_e5787 + 1.0);
        let assign10870_e5790: f64 = (locals.var_ef_mueph1 * assign10870_e5789);
        (assign10870_e5790, ((locals.var_ef_mueph1_dn0 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn0 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn2 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn2 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn4 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn4 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn5 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn5 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn6 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn6 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn7 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn7 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn8 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn8 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn9 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn9 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn10 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn10 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn13 * assign10870_e5789) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn13 / locals.var_ef_nsubc))))),)
    } else {
        (locals.var_ef_mueph1, locals.var_ef_mueph1_dn0, locals.var_ef_mueph1_dn2, locals.var_ef_mueph1_dn4, locals.var_ef_mueph1_dn5, locals.var_ef_mueph1_dn6, locals.var_ef_mueph1_dn7, locals.var_ef_mueph1_dn8, locals.var_ef_mueph1_dn9, locals.var_ef_mueph1_dn10, locals.var_ef_mueph1_dn13,)
    }
};
        locals.var_ef_mueph1 = assign10870_e5792;
        locals.var_ef_mueph1_dn0 = assign10870_e5792_d_n0;
        locals.var_ef_mueph1_dn2 = assign10870_e5792_d_n2;
        locals.var_ef_mueph1_dn4 = assign10870_e5792_d_n4;
        locals.var_ef_mueph1_dn5 = assign10870_e5792_d_n5;
        locals.var_ef_mueph1_dn6 = assign10870_e5792_d_n6;
        locals.var_ef_mueph1_dn7 = assign10870_e5792_d_n7;
        locals.var_ef_mueph1_dn8 = assign10870_e5792_d_n8;
        locals.var_ef_mueph1_dn9 = assign10870_e5792_d_n9;
        locals.var_ef_mueph1_dn10 = assign10870_e5792_d_n10;
        locals.var_ef_mueph1_dn13 = assign10870_e5792_d_n13;

        let (assign10880_e5800, assign10880_e5800_d_n0, assign10880_e5800_d_n2, assign10880_e5800_d_n4, assign10880_e5800_d_n5, assign10880_e5800_d_n6, assign10880_e5800_d_n7, assign10880_e5800_d_n8, assign10880_e5800_d_n9, assign10880_e5800_d_n10, assign10880_e5800_d_n13,) = {
    if (locals.var_guard253 != 0.0) {
        let assign10880_e5796: f64 = (locals.var_ef_nsubp + locals.var_mks_nsubcdfm);
        let assign10880_e5798: f64 = (assign10880_e5796 - locals.var_ef_nsubc);
        (assign10880_e5798, (locals.var_ef_nsubp_dn0 - locals.var_ef_nsubc_dn0), (locals.var_ef_nsubp_dn2 - locals.var_ef_nsubc_dn2), (locals.var_ef_nsubp_dn4 - locals.var_ef_nsubc_dn4), (locals.var_ef_nsubp_dn5 - locals.var_ef_nsubc_dn5), (locals.var_ef_nsubp_dn6 - locals.var_ef_nsubc_dn6), (locals.var_ef_nsubp_dn7 - locals.var_ef_nsubc_dn7), (locals.var_ef_nsubp_dn8 - locals.var_ef_nsubc_dn8), (locals.var_ef_nsubp_dn9 - locals.var_ef_nsubc_dn9), (locals.var_ef_nsubp_dn10 - locals.var_ef_nsubc_dn10), (locals.var_ef_nsubp_dn13 - locals.var_ef_nsubc_dn13),)
    } else {
        (locals.var_ef_nsubp, locals.var_ef_nsubp_dn0, locals.var_ef_nsubp_dn2, locals.var_ef_nsubp_dn4, locals.var_ef_nsubp_dn5, locals.var_ef_nsubp_dn6, locals.var_ef_nsubp_dn7, locals.var_ef_nsubp_dn8, locals.var_ef_nsubp_dn9, locals.var_ef_nsubp_dn10, locals.var_ef_nsubp_dn13,)
    }
};
        locals.var_ef_nsubp = assign10880_e5800;
        locals.var_ef_nsubp_dn0 = assign10880_e5800_d_n0;
        locals.var_ef_nsubp_dn2 = assign10880_e5800_d_n2;
        locals.var_ef_nsubp_dn4 = assign10880_e5800_d_n4;
        locals.var_ef_nsubp_dn5 = assign10880_e5800_d_n5;
        locals.var_ef_nsubp_dn6 = assign10880_e5800_d_n6;
        locals.var_ef_nsubp_dn7 = assign10880_e5800_d_n7;
        locals.var_ef_nsubp_dn8 = assign10880_e5800_d_n8;
        locals.var_ef_nsubp_dn9 = assign10880_e5800_d_n9;
        locals.var_ef_nsubp_dn10 = assign10880_e5800_d_n10;
        locals.var_ef_nsubp_dn13 = assign10880_e5800_d_n13;

        let (assign10890_e5808, assign10890_e5808_d_n0, assign10890_e5808_d_n2, assign10890_e5808_d_n4, assign10890_e5808_d_n5, assign10890_e5808_d_n6, assign10890_e5808_d_n7, assign10890_e5808_d_n8, assign10890_e5808_d_n9, assign10890_e5808_d_n10, assign10890_e5808_d_n13,) = {
    if (locals.var_guard253 != 0.0) {
        let assign10890_e5804: f64 = (locals.var_npexte + locals.var_mks_nsubcdfm);
        let assign10890_e5806: f64 = (assign10890_e5804 - locals.var_ef_nsubc);
        (assign10890_e5806, (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0), (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2), (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4), (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5), (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6), (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7), (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8), (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9), (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10), (locals.var_npexte_dn13 - locals.var_ef_nsubc_dn13),)
    } else {
        (locals.var_npexte, locals.var_npexte_dn0, locals.var_npexte_dn2, locals.var_npexte_dn4, locals.var_npexte_dn5, locals.var_npexte_dn6, locals.var_npexte_dn7, locals.var_npexte_dn8, locals.var_npexte_dn9, locals.var_npexte_dn10, locals.var_npexte_dn13,)
    }
};
        locals.var_npexte = assign10890_e5808;
        locals.var_npexte_dn0 = assign10890_e5808_d_n0;
        locals.var_npexte_dn2 = assign10890_e5808_d_n2;
        locals.var_npexte_dn4 = assign10890_e5808_d_n4;
        locals.var_npexte_dn5 = assign10890_e5808_d_n5;
        locals.var_npexte_dn6 = assign10890_e5808_d_n6;
        locals.var_npexte_dn7 = assign10890_e5808_d_n7;
        locals.var_npexte_dn8 = assign10890_e5808_d_n8;
        locals.var_npexte_dn9 = assign10890_e5808_d_n9;
        locals.var_npexte_dn10 = assign10890_e5808_d_n10;
        locals.var_npexte_dn13 = assign10890_e5808_d_n13;

        let (assign10900_e5812, assign10900_e5812_d_n0, assign10900_e5812_d_n2, assign10900_e5812_d_n4, assign10900_e5812_d_n5, assign10900_e5812_d_n6, assign10900_e5812_d_n7, assign10900_e5812_d_n8, assign10900_e5812_d_n9, assign10900_e5812_d_n10, assign10900_e5812_d_n13,) = {
    if (locals.var_guard253 != 0.0) {
        (locals.var_mks_nsubcdfm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ef_nsubc, locals.var_ef_nsubc_dn0, locals.var_ef_nsubc_dn2, locals.var_ef_nsubc_dn4, locals.var_ef_nsubc_dn5, locals.var_ef_nsubc_dn6, locals.var_ef_nsubc_dn7, locals.var_ef_nsubc_dn8, locals.var_ef_nsubc_dn9, locals.var_ef_nsubc_dn10, locals.var_ef_nsubc_dn13,)
    }
};
        locals.var_ef_nsubc = assign10900_e5812;
        locals.var_ef_nsubc_dn0 = assign10900_e5812_d_n0;
        locals.var_ef_nsubc_dn2 = assign10900_e5812_d_n2;
        locals.var_ef_nsubc_dn4 = assign10900_e5812_d_n4;
        locals.var_ef_nsubc_dn5 = assign10900_e5812_d_n5;
        locals.var_ef_nsubc_dn6 = assign10900_e5812_d_n6;
        locals.var_ef_nsubc_dn7 = assign10900_e5812_d_n7;
        locals.var_ef_nsubc_dn8 = assign10900_e5812_d_n8;
        locals.var_ef_nsubc_dn9 = assign10900_e5812_d_n9;
        locals.var_ef_nsubc_dn10 = assign10900_e5812_d_n10;
        locals.var_ef_nsubc_dn13 = assign10900_e5812_d_n13;

        let assign10910_e5818: f64 = (locals.var_wg).powf(p.p163);
        let assign10910_e5819: f64 = (p.p162 / assign10910_e5818);
        let assign10910_e5820: f64 = (1.0 + assign10910_e5819);
        let assign10910_e5821: f64 = (locals.var_ef_mueph1 * assign10910_e5820);
        let assign10910_e5826: f64 = (locals.var_lg).powf(p.p165);
        let assign10910_e5827: f64 = (p.p164 / assign10910_e5826);
        let assign10910_e5828: f64 = (1.0 + assign10910_e5827);
        let assign10910_e5829: f64 = (assign10910_e5821 * assign10910_e5828);
        let assign10910_e5834: f64 = (locals.var_wlg).powf(p.p168);
        let assign10910_e5835: f64 = (p.p167 / assign10910_e5834);
        let assign10910_e5836: f64 = (1.0 + assign10910_e5835);
        let assign10910_e5837: f64 = (assign10910_e5829 * assign10910_e5836);
        locals.var_mueph = assign10910_e5837;
        locals.var_mueph_dn0 = (((locals.var_ef_mueph1_dn0 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn2 = (((locals.var_ef_mueph1_dn2 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn4 = (((locals.var_ef_mueph1_dn4 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn5 = (((locals.var_ef_mueph1_dn5 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn6 = (((locals.var_ef_mueph1_dn6 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn7 = (((locals.var_ef_mueph1_dn7 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn8 = (((locals.var_ef_mueph1_dn8 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn9 = (((locals.var_ef_mueph1_dn9 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn10 = (((locals.var_ef_mueph1_dn10 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);
        locals.var_mueph_dn13 = (((locals.var_ef_mueph1_dn13 * assign10910_e5820) * assign10910_e5828) * assign10910_e5836);

        let assign10920_e5840: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign10920_e5840;

        let (assign10930_e5848, assign10930_e5848_d_n0, assign10930_e5848_d_n2, assign10930_e5848_d_n4, assign10930_e5848_d_n5, assign10930_e5848_d_n6, assign10930_e5848_d_n7, assign10930_e5848_d_n8, assign10930_e5848_d_n9, assign10930_e5848_d_n10, assign10930_e5848_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10930_e5845: f64 = (1.0 + locals.var_uc_muesti2);
        let assign10930_e5846: f64 = (1.0 / assign10930_e5845);
        (assign10930_e5846, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn13,)
    }
};
        locals.var_t1 = assign10930_e5848;
        locals.var_t1_dn0 = assign10930_e5848_d_n0;
        locals.var_t1_dn2 = assign10930_e5848_d_n2;
        locals.var_t1_dn4 = assign10930_e5848_d_n4;
        locals.var_t1_dn5 = assign10930_e5848_d_n5;
        locals.var_t1_dn6 = assign10930_e5848_d_n6;
        locals.var_t1_dn7 = assign10930_e5848_d_n7;
        locals.var_t1_dn8 = assign10930_e5848_d_n8;
        locals.var_t1_dn9 = assign10930_e5848_d_n9;
        locals.var_t1_dn10 = assign10930_e5848_d_n10;
        locals.var_t1_dn13 = assign10930_e5848_d_n13;

        let (assign10940_e5856, assign10940_e5856_d_n0, assign10940_e5856_d_n2, assign10940_e5856_d_n4, assign10940_e5856_d_n5, assign10940_e5856_d_n6, assign10940_e5856_d_n7, assign10940_e5856_d_n8, assign10940_e5856_d_n9, assign10940_e5856_d_n10, assign10940_e5856_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10940_e5852: f64 = (locals.var_uc_muesti1 / locals.var_lod_half);
        let assign10940_e5854: f64 = (assign10940_e5852).powf(locals.var_uc_muesti3);
        (assign10940_e5854, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10940_e5852).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn13) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10940_e5854 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn13) / (locals.var_lod_half * locals.var_lod_half))) / assign10940_e5852))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn13,)
    }
};
        locals.var_t2 = assign10940_e5856;
        locals.var_t2_dn0 = assign10940_e5856_d_n0;
        locals.var_t2_dn2 = assign10940_e5856_d_n2;
        locals.var_t2_dn4 = assign10940_e5856_d_n4;
        locals.var_t2_dn5 = assign10940_e5856_d_n5;
        locals.var_t2_dn6 = assign10940_e5856_d_n6;
        locals.var_t2_dn7 = assign10940_e5856_d_n7;
        locals.var_t2_dn8 = assign10940_e5856_d_n8;
        locals.var_t2_dn9 = assign10940_e5856_d_n9;
        locals.var_t2_dn10 = assign10940_e5856_d_n10;
        locals.var_t2_dn13 = assign10940_e5856_d_n13;

        let (assign10950_e5864, assign10950_e5864_d_n0, assign10950_e5864_d_n2, assign10950_e5864_d_n4, assign10950_e5864_d_n5, assign10950_e5864_d_n6, assign10950_e5864_d_n7, assign10950_e5864_d_n8, assign10950_e5864_d_n9, assign10950_e5864_d_n10, assign10950_e5864_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10950_e5860: f64 = (locals.var_uc_muesti1 / locals.var_lod_half_ref);
        let assign10950_e5862: f64 = (assign10950_e5860).powf(locals.var_uc_muesti3);
        (assign10950_e5862, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10950_e5860).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn13) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10950_e5862 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn13) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10950_e5860))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn13,)
    }
};
        locals.var_t3 = assign10950_e5864;
        locals.var_t3_dn0 = assign10950_e5864_d_n0;
        locals.var_t3_dn2 = assign10950_e5864_d_n2;
        locals.var_t3_dn4 = assign10950_e5864_d_n4;
        locals.var_t3_dn5 = assign10950_e5864_d_n5;
        locals.var_t3_dn6 = assign10950_e5864_d_n6;
        locals.var_t3_dn7 = assign10950_e5864_d_n7;
        locals.var_t3_dn8 = assign10950_e5864_d_n8;
        locals.var_t3_dn9 = assign10950_e5864_d_n9;
        locals.var_t3_dn10 = assign10950_e5864_d_n10;
        locals.var_t3_dn13 = assign10950_e5864_d_n13;

        let (assign10960_e5880, assign10960_e5880_d_n0, assign10960_e5880_d_n2, assign10960_e5880_d_n4, assign10960_e5880_d_n5, assign10960_e5880_d_n6, assign10960_e5880_d_n7, assign10960_e5880_d_n8, assign10960_e5880_d_n9, assign10960_e5880_d_n10, assign10960_e5880_d_n13,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10960_e5870: f64 = (locals.var_t1 * locals.var_t2);
        let assign10960_e5871: f64 = (1.0 + assign10960_e5870);
        let assign10960_e5872: f64 = (locals.var_mueph * assign10960_e5871);
        let assign10960_e5876: f64 = (locals.var_t1 * locals.var_t3);
        let assign10960_e5877: f64 = (1.0 + assign10960_e5876);
        let assign10960_e5878: f64 = (assign10960_e5872 / assign10960_e5877);
        (assign10960_e5878, (((((locals.var_mueph_dn0 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn2 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn4 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn5 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn6 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn7 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn8 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn9 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn10 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign10960_e5877 * assign10960_e5877)), (((((locals.var_mueph_dn13 * assign10960_e5871) + (locals.var_mueph * ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13)))) * assign10960_e5877) - (assign10960_e5872 * ((locals.var_t1_dn13 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn13)))) / (assign10960_e5877 * assign10960_e5877)),)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn13,)
    }
};
        locals.var_mueph = assign10960_e5880;
        locals.var_mueph_dn0 = assign10960_e5880_d_n0;
        locals.var_mueph_dn2 = assign10960_e5880_d_n2;
        locals.var_mueph_dn4 = assign10960_e5880_d_n4;
        locals.var_mueph_dn5 = assign10960_e5880_d_n5;
        locals.var_mueph_dn6 = assign10960_e5880_d_n6;
        locals.var_mueph_dn7 = assign10960_e5880_d_n7;
        locals.var_mueph_dn8 = assign10960_e5880_d_n8;
        locals.var_mueph_dn9 = assign10960_e5880_d_n9;
        locals.var_mueph_dn10 = assign10960_e5880_d_n10;
        locals.var_mueph_dn13 = assign10960_e5880_d_n13;

        let assign10970_e5886: f64 = (locals.var_lg).powf(p.p176);
        let assign10970_e5887: f64 = (p.p173 / assign10970_e5886);
        let assign10970_e5888: f64 = (1.0 + assign10970_e5887);
        let assign10970_e5889: f64 = (p.p171 * assign10970_e5888);
        let assign10970_e5894: f64 = (locals.var_wg).powf(p.p175);
        let assign10970_e5895: f64 = (p.p174 / assign10970_e5894);
        let assign10970_e5896: f64 = (1.0 + assign10970_e5895);
        let assign10970_e5897: f64 = (assign10970_e5889 * assign10970_e5896);
        locals.var_muesr = assign10970_e5897;

        let (assign11000_e5921, assign11000_e5921_d_n0, assign11000_e5921_d_n2, assign11000_e5921_d_n4, assign11000_e5921_d_n5, assign11000_e5921_d_n6, assign11000_e5921_d_n7, assign11000_e5921_d_n8, assign11000_e5921_d_n9, assign11000_e5921_d_n10, assign11000_e5921_d_n13,) = {
    if (locals.var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn13,)
    }
};
        locals.var_mueph = assign11000_e5921;
        locals.var_mueph_dn0 = assign11000_e5921_d_n0;
        locals.var_mueph_dn2 = assign11000_e5921_d_n2;
        locals.var_mueph_dn4 = assign11000_e5921_d_n4;
        locals.var_mueph_dn5 = assign11000_e5921_d_n5;
        locals.var_mueph_dn6 = assign11000_e5921_d_n6;
        locals.var_mueph_dn7 = assign11000_e5921_d_n7;
        locals.var_mueph_dn8 = assign11000_e5921_d_n8;
        locals.var_mueph_dn9 = assign11000_e5921_d_n9;
        locals.var_mueph_dn10 = assign11000_e5921_d_n10;
        locals.var_mueph_dn13 = assign11000_e5921_d_n13;

        let (assign11010_e5927,) = {
    if (locals.var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (locals.var_muesr,)
    }
};
        locals.var_muesr = assign11010_e5927;

        let assign11020_e5930: f64 = (locals.var_lg).powf(p.p156);
        locals.var_t1 = assign11020_e5930;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn13 = 0.0;

        let assign11030_e5933: f64 = (locals.var_uc_ndep * locals.var_t1);
        let assign11030_e5936: f64 = (locals.var_t1 + p.p155);
        let assign11030_e5937: f64 = (assign11030_e5933 / assign11030_e5936);
        let assign11030_e5939: f64 = (assign11030_e5937 / 1.034943e-10);
        locals.var_ndep_o_esi = assign11030_e5939;
        locals.var_ndep_o_esi_dn0 = (((((locals.var_uc_ndep * locals.var_t1_dn0) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn0)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn2 = (((((locals.var_uc_ndep * locals.var_t1_dn2) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn2)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn4 = (((((locals.var_uc_ndep * locals.var_t1_dn4) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn4)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn5 = (((((locals.var_uc_ndep * locals.var_t1_dn5) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn5)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn6 = (((((locals.var_uc_ndep * locals.var_t1_dn6) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn6)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn7 = (((((locals.var_uc_ndep * locals.var_t1_dn7) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn7)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn8 = (((((locals.var_uc_ndep * locals.var_t1_dn8) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn8)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn9 = (((((locals.var_uc_ndep * locals.var_t1_dn9) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn9)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn10 = (((((locals.var_uc_ndep * locals.var_t1_dn10) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn10)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn13 = (((((locals.var_uc_ndep * locals.var_t1_dn13) * assign11030_e5936) - (assign11030_e5933 * locals.var_t1_dn13)) / (assign11030_e5936 * assign11030_e5936)) / 1.034943e-10);

        let assign11040_e5942: f64 = (locals.var_uc_ninv / 1.034943e-10);
        locals.var_ninv_o_esi = assign11040_e5942;

    }
}
