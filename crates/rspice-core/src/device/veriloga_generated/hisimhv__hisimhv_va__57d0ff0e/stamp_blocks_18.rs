#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    pub(super) fn stamp_reactive_block_156(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        if (locals.var_guard2403 == 0.0) {(locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14, ) = (locals.var_qde, locals.var_qde_dn0, locals.var_qde_dn2, locals.var_qde_dn4, locals.var_qde_dn5, locals.var_qde_dn6, locals.var_qde_dn7, locals.var_qde_dn8, locals.var_qde_dn9, locals.var_qde_dn10, locals.var_qde_dn11, locals.var_qde_dn14, );locals.var_qs_rv = 0.0;let assign106190_e158458: f64 = (locals.var_qge + locals.var_qde);let assign106190_e158460: f64 = (assign106190_e158458 + locals.var_qse);let assign106190_e158461: f64 = (-assign106190_e158460);(locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14, ) = (assign106190_e158461, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn4 + locals.var_qde_dn4) + locals.var_qse_dn4)), (-((locals.var_qge_dn5 + locals.var_qde_dn5) + locals.var_qse_dn5)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn8 + locals.var_qde_dn8) + locals.var_qse_dn8)), (-((locals.var_qge_dn9 + locals.var_qde_dn9) + locals.var_qse_dn9)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn14 + locals.var_qde_dn14) + locals.var_qse_dn14)), );locals.var_qb_rv = 0.0;(locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn4, locals.var_isub_dn5, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn8, locals.var_isub_dn9, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_isub_rv = 0.0;(locals.var_isubld, locals.var_isubld_dn0, locals.var_isubld_dn2, locals.var_isubld_dn4, locals.var_isubld_dn5, locals.var_isubld_dn6, locals.var_isubld_dn7, locals.var_isubld_dn8, locals.var_isubld_dn9, locals.var_isubld_dn10, locals.var_isubld_dn11, locals.var_isubld_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_isubld_rv = 0.0;(locals.var_idsibpc, locals.var_idsibpc_dn0, locals.var_idsibpc_dn2, locals.var_idsibpc_dn4, locals.var_idsibpc_dn5, locals.var_idsibpc_dn6, locals.var_idsibpc_dn7, locals.var_idsibpc_dn8, locals.var_idsibpc_dn9, locals.var_idsibpc_dn10, locals.var_idsibpc_dn11, locals.var_idsibpc_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_idsibpc_rv = 0.0;}
        if ((locals.var_guard2403 == 0.0) && (locals.var_flg_nqs != 0.0)) {let assign106320_e158530: f64 = (1.0 - locals.var_xd);(locals.var_qdrat, locals.var_qdrat_dn0, locals.var_qdrat_dn2, locals.var_qdrat_dn4, locals.var_qdrat_dn5, locals.var_qdrat_dn6, locals.var_qdrat_dn7, locals.var_qdrat_dn8, locals.var_qdrat_dn9, locals.var_qdrat_dn10, locals.var_qdrat_dn11, locals.var_qdrat_dn14, ) = (assign106320_e158530, (-locals.var_xd_dn0), (-locals.var_xd_dn2), (-locals.var_xd_dn4), (-locals.var_xd_dn5), (-locals.var_xd_dn6), (-locals.var_xd_dn7), (-locals.var_xd_dn8), (-locals.var_xd_dn9), (-locals.var_xd_dn10), (-locals.var_xd_dn11), (-locals.var_xd_dn14), );locals.var_qdrat_rv = 0.0;}
        let assign106330_e158535: f64 = (locals.var_qg + locals.var_qgov);(locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn4, locals.var_qg_dn5, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn8, locals.var_qg_dn9, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn14, ) = (assign106330_e158535, (locals.var_qg_dn0 + locals.var_qgov_dn0), (locals.var_qg_dn2 + locals.var_qgov_dn2), (locals.var_qg_dn4 + locals.var_qgov_dn4), (locals.var_qg_dn5 + locals.var_qgov_dn5), (locals.var_qg_dn6 + locals.var_qgov_dn6), (locals.var_qg_dn7 + locals.var_qgov_dn7), (locals.var_qg_dn8 + locals.var_qgov_dn8), (locals.var_qg_dn9 + locals.var_qgov_dn9), (locals.var_qg_dn10 + locals.var_qgov_dn10), (locals.var_qg_dn11 + locals.var_qgov_dn11), (locals.var_qg_dn14 + locals.var_qgov_dn14), );locals.var_qg_rv = 0.0;let assign106340_e158538: f64 = (locals.var_qd + locals.var_qdov);(locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn4, locals.var_qd_dn5, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn8, locals.var_qd_dn9, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn14, ) = (assign106340_e158538, (locals.var_qd_dn0 + locals.var_qdov_dn0), (locals.var_qd_dn2 + locals.var_qdov_dn2), (locals.var_qd_dn4 + locals.var_qdov_dn4), (locals.var_qd_dn5 + locals.var_qdov_dn5), (locals.var_qd_dn6 + locals.var_qdov_dn6), (locals.var_qd_dn7 + locals.var_qdov_dn7), (locals.var_qd_dn8 + locals.var_qdov_dn8), (locals.var_qd_dn9 + locals.var_qdov_dn9), (locals.var_qd_dn10 + locals.var_qdov_dn10), (locals.var_qd_dn11 + locals.var_qdov_dn11), (locals.var_qd_dn14 + locals.var_qdov_dn14), );locals.var_qd_rv = 0.0;let assign106350_e158541: f64 = (locals.var_qs + locals.var_qsov);(locals.var_qs, locals.var_qs_dn0, locals.var_qs_dn2, locals.var_qs_dn4, locals.var_qs_dn5, locals.var_qs_dn6, locals.var_qs_dn7, locals.var_qs_dn8, locals.var_qs_dn9, locals.var_qs_dn10, locals.var_qs_dn11, locals.var_qs_dn14, ) = (assign106350_e158541, (locals.var_qs_dn0 + locals.var_qsov_dn0), (locals.var_qs_dn2 + locals.var_qsov_dn2), (locals.var_qs_dn4 + locals.var_qsov_dn4), (locals.var_qs_dn5 + locals.var_qsov_dn5), (locals.var_qs_dn6 + locals.var_qsov_dn6), (locals.var_qs_dn7 + locals.var_qsov_dn7), (locals.var_qs_dn8 + locals.var_qsov_dn8), (locals.var_qs_dn9 + locals.var_qsov_dn9), (locals.var_qs_dn10 + locals.var_qsov_dn10), (locals.var_qs_dn11 + locals.var_qsov_dn11), (locals.var_qs_dn14 + locals.var_qsov_dn14), );locals.var_qs_rv = 0.0;let assign106360_e158544: f64 = (locals.var_qg + locals.var_qd);let assign106360_e158546: f64 = (assign106360_e158544 + locals.var_qs);let assign106360_e158547: f64 = (-assign106360_e158546);(locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn4, locals.var_qb_dn5, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn8, locals.var_qb_dn9, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn14, ) = (assign106360_e158547, (-((locals.var_qg_dn0 + locals.var_qd_dn0) + locals.var_qs_dn0)), (-((locals.var_qg_dn2 + locals.var_qd_dn2) + locals.var_qs_dn2)), (-((locals.var_qg_dn4 + locals.var_qd_dn4) + locals.var_qs_dn4)), (-((locals.var_qg_dn5 + locals.var_qd_dn5) + locals.var_qs_dn5)), (-((locals.var_qg_dn6 + locals.var_qd_dn6) + locals.var_qs_dn6)), (-((locals.var_qg_dn7 + locals.var_qd_dn7) + locals.var_qs_dn7)), (-((locals.var_qg_dn8 + locals.var_qd_dn8) + locals.var_qs_dn8)), (-((locals.var_qg_dn9 + locals.var_qd_dn9) + locals.var_qs_dn9)), (-((locals.var_qg_dn10 + locals.var_qd_dn10) + locals.var_qs_dn10)), (-((locals.var_qg_dn11 + locals.var_qd_dn11) + locals.var_qs_dn11)), (-((locals.var_qg_dn14 + locals.var_qd_dn14) + locals.var_qs_dn14)), );locals.var_qb_rv = 0.0;(locals.var_qfd, locals.var_qfd_dn0, locals.var_qfd_dn2, locals.var_qfd_dn7, ) = (locals.var_qdp, locals.var_qdp_dn0, locals.var_qdp_dn2, locals.var_qdp_dn7, );locals.var_qfd_rv = 0.0;(locals.var_qfs, locals.var_qfs_dn2, locals.var_qfs_dn7, ) = (locals.var_qsp, locals.var_qsp_dn2, locals.var_qsp_dn7, );locals.var_qfs_rv = 0.0;
        (locals.var_qdext, locals.var_qdext_dn0, locals.var_qdext_dn2, locals.var_qdext_dn4, locals.var_qdext_dn5, locals.var_qdext_dn6, locals.var_qdext_dn7, locals.var_qdext_dn8, locals.var_qdext_dn9, locals.var_qdext_dn10, locals.var_qdext_dn11, locals.var_qdext_dn14, ) = (locals.var_qdexte, locals.var_qdexte_dn0, locals.var_qdexte_dn2, locals.var_qdexte_dn4, locals.var_qdexte_dn5, locals.var_qdexte_dn6, locals.var_qdexte_dn7, locals.var_qdexte_dn8, locals.var_qdexte_dn9, locals.var_qdexte_dn10, locals.var_qdexte_dn11, locals.var_qdexte_dn14, );locals.var_qdext_rv = 0.0;(locals.var_qgext, locals.var_qgext_dn0, locals.var_qgext_dn2, locals.var_qgext_dn4, locals.var_qgext_dn5, locals.var_qgext_dn6, locals.var_qgext_dn7, locals.var_qgext_dn8, locals.var_qgext_dn9, locals.var_qgext_dn10, locals.var_qgext_dn11, locals.var_qgext_dn14, ) = (locals.var_qgexte, locals.var_qgexte_dn0, locals.var_qgexte_dn2, locals.var_qgexte_dn4, locals.var_qgexte_dn5, locals.var_qgexte_dn6, locals.var_qgexte_dn7, locals.var_qgexte_dn8, locals.var_qgexte_dn9, locals.var_qgexte_dn10, locals.var_qgexte_dn11, locals.var_qgexte_dn14, );locals.var_qgext_rv = 0.0;let assign106410_e158554: f64 = (locals.var_qgexte + locals.var_qdexte);let assign106410_e158556: f64 = (assign106410_e158554 + locals.var_qsexte);let assign106410_e158557: f64 = (-assign106410_e158556);(locals.var_qbext, locals.var_qbext_dn0, locals.var_qbext_dn2, locals.var_qbext_dn4, locals.var_qbext_dn5, locals.var_qbext_dn6, locals.var_qbext_dn7, locals.var_qbext_dn8, locals.var_qbext_dn9, locals.var_qbext_dn10, locals.var_qbext_dn11, locals.var_qbext_dn14, ) = (assign106410_e158557, (-((locals.var_qgexte_dn0 + locals.var_qdexte_dn0) + locals.var_qsexte_dn0)), (-((locals.var_qgexte_dn2 + locals.var_qdexte_dn2) + locals.var_qsexte_dn2)), (-((locals.var_qgexte_dn4 + locals.var_qdexte_dn4) + locals.var_qsexte_dn4)), (-((locals.var_qgexte_dn5 + locals.var_qdexte_dn5) + locals.var_qsexte_dn5)), (-((locals.var_qgexte_dn6 + locals.var_qdexte_dn6) + locals.var_qsexte_dn6)), (-((locals.var_qgexte_dn7 + locals.var_qdexte_dn7) + locals.var_qsexte_dn7)), (-((locals.var_qgexte_dn8 + locals.var_qdexte_dn8) + locals.var_qsexte_dn8)), (-((locals.var_qgexte_dn9 + locals.var_qdexte_dn9) + locals.var_qsexte_dn9)), (-((locals.var_qgexte_dn10 + locals.var_qdexte_dn10) + locals.var_qsexte_dn10)), (-((locals.var_qgexte_dn11 + locals.var_qdexte_dn11) + locals.var_qsexte_dn11)), (-((locals.var_qgexte_dn14 + locals.var_qdexte_dn14) + locals.var_qsexte_dn14)), );locals.var_qbext_rv = 0.0;let assign106420_e158560: f64 = if p.p53 > 0.0 { 1.0 } else { 0.0 };locals.var_guard2404 = assign106420_e158560;locals.var_guard2404_rv = 0.0;let assign106430_e158563: f64 = if locals.var_rth > 0.0001 { 1.0 } else { 0.0 };locals.var_guard2405 = assign106430_e158563;locals.var_guard2405_rv = 0.0;
        if ((locals.var_guard2404 != 0.0) && (locals.var_guard2405 != 0.0)) {let assign106440_e158569: f64 = (1.0 / locals.var_rth);(locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14, ) = (assign106440_e158569, (-(locals.var_rth_dn0 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn2 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn4 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn5 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn6 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn7 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn8 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn9 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn10 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn11 / (locals.var_rth * locals.var_rth))), (-(locals.var_rth_dn14 / (locals.var_rth * locals.var_rth))), );locals.var_gth_rv = 0.0;}
        if ((locals.var_guard2404 != 0.0) && (locals.var_guard2405 == 0.0)) {let assign106450_e158578: f64 = (1.0 / 0.0001);(locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14, ) = (assign106450_e158578, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_gth_rv = 0.0;}
        let assign106460_e158584: f64 = (locals.var_vdsei - locals.var_vdsi);let assign106460_e158585: f64 = (locals.var_vdsi * assign106460_e158584);let assign106460_e158587: f64 = if assign106460_e158585 >= 0.0 { 1.0 } else { 0.0 };locals.var_guard2406 = assign106460_e158587;locals.var_guard2406_rv = 0.0;let assign106470_e158590: f64 = if locals.var_uc_powrat == 1.0 { 1.0 } else { 0.0 };locals.var_guard2407 = assign106470_e158590;locals.var_guard2407_rv = 0.0;
        if (((locals.var_guard2404 != 0.0) && (locals.var_guard2406 != 0.0)) && (locals.var_guard2407 != 0.0)) {(locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14, ) = (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_veffpower_rv = 0.0;}
        if (((locals.var_guard2404 != 0.0) && (locals.var_guard2406 != 0.0)) && (locals.var_guard2407 == 0.0)) {let assign106490_e158609: f64 = (locals.var_vdsei - locals.var_vdsi);let assign106490_e158610: f64 = (locals.var_powratio * assign106490_e158609);let assign106490_e158611: f64 = (locals.var_vdsi + assign106490_e158610);(locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14, ) = (assign106490_e158611, ((locals.var_powratio_dn0 * assign106490_e158609) + (locals.var_powratio * locals.var_vdsei_dn0)), ((locals.var_powratio_dn2 * assign106490_e158609) + (locals.var_powratio * locals.var_vdsei_dn2)), (locals.var_powratio_dn4 * assign106490_e158609), (locals.var_powratio_dn5 * assign106490_e158609), (locals.var_vdsi_dn6 + ((locals.var_powratio_dn6 * assign106490_e158609) + (locals.var_powratio * (-locals.var_vdsi_dn6)))), (locals.var_powratio_dn7 * assign106490_e158609), (locals.var_vdsi_dn8 + ((locals.var_powratio_dn8 * assign106490_e158609) + (locals.var_powratio * (-locals.var_vdsi_dn8)))), (locals.var_powratio_dn9 * assign106490_e158609), (locals.var_powratio_dn10 * assign106490_e158609), (locals.var_powratio_dn11 * assign106490_e158609), (locals.var_powratio_dn14 * assign106490_e158609), );locals.var_veffpower_rv = 0.0;}
        if ((locals.var_guard2404 != 0.0) && (locals.var_guard2406 == 0.0)) {(locals.var_veffpower, locals.var_veffpower_dn0, locals.var_veffpower_dn2, locals.var_veffpower_dn4, locals.var_veffpower_dn5, locals.var_veffpower_dn6, locals.var_veffpower_dn7, locals.var_veffpower_dn8, locals.var_veffpower_dn9, locals.var_veffpower_dn10, locals.var_veffpower_dn11, locals.var_veffpower_dn14, ) = (locals.var_vdsi, 0.0, 0.0, 0.0, 0.0, locals.var_vdsi_dn6, 0.0, locals.var_vdsi_dn8, 0.0, 0.0, 0.0, 0.0, );locals.var_veffpower_rv = 0.0;}
        if (locals.var_guard2404 != 0.0) {let assign106510_e158624: f64 = (locals.var_ids * locals.var_veffpower);(locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14, ) = (assign106510_e158624, ((locals.var_ids_dn0 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn0)), ((locals.var_ids_dn2 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn2)), ((locals.var_ids_dn4 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn4)), ((locals.var_ids_dn5 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn5)), ((locals.var_ids_dn6 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn6)), ((locals.var_ids_dn7 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn7)), ((locals.var_ids_dn8 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn8)), ((locals.var_ids_dn9 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn9)), ((locals.var_ids_dn10 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn10)), ((locals.var_ids_dn11 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn11)), ((locals.var_ids_dn14 * locals.var_veffpower) + (locals.var_ids * locals.var_veffpower_dn14)), );locals.var_p_rv = 0.0;}
        let assign106520_e158629: f64 = if p.p53 == 1.0 { 1.0 } else { 0.0 };locals.var_guard2408 = assign106520_e158629;locals.var_guard2408_rv = 0.0;
        if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {let assign106530_e158635: f64 = (p.p433 * locals.var_gth);(locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14, ) = (assign106530_e158635, (p.p433 * locals.var_gth_dn0), (p.p433 * locals.var_gth_dn2), (p.p433 * locals.var_gth_dn4), (p.p433 * locals.var_gth_dn5), (p.p433 * locals.var_gth_dn6), (p.p433 * locals.var_gth_dn7), (p.p433 * locals.var_gth_dn8), (p.p433 * locals.var_gth_dn9), (p.p433 * locals.var_gth_dn10), (p.p433 * locals.var_gth_dn11), (p.p433 * locals.var_gth_dn14), );locals.var_t1_rv = 0.0;let assign106540_e158643: f64 = (locals.var_t1 - locals.var_p);let assign106540_e158646: f64 = (p.p337 * locals.var_gth);let assign106540_e158647: f64 = (assign106540_e158643 - assign106540_e158646);(locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14, ) = (assign106540_e158647, ((locals.var_t1_dn0 - locals.var_p_dn0) - (p.p337 * locals.var_gth_dn0)), ((locals.var_t1_dn2 - locals.var_p_dn2) - (p.p337 * locals.var_gth_dn2)), ((locals.var_t1_dn4 - locals.var_p_dn4) - (p.p337 * locals.var_gth_dn4)), ((locals.var_t1_dn5 - locals.var_p_dn5) - (p.p337 * locals.var_gth_dn5)), ((locals.var_t1_dn6 - locals.var_p_dn6) - (p.p337 * locals.var_gth_dn6)), ((locals.var_t1_dn7 - locals.var_p_dn7) - (p.p337 * locals.var_gth_dn7)), ((locals.var_t1_dn8 - locals.var_p_dn8) - (p.p337 * locals.var_gth_dn8)), ((locals.var_t1_dn9 - locals.var_p_dn9) - (p.p337 * locals.var_gth_dn9)), ((locals.var_t1_dn10 - locals.var_p_dn10) - (p.p337 * locals.var_gth_dn10)), ((locals.var_t1_dn11 - locals.var_p_dn11) - (p.p337 * locals.var_gth_dn11)), ((locals.var_t1_dn14 - locals.var_p_dn14) - (p.p337 * locals.var_gth_dn14)), );locals.var_tmf1_rv = 0.0;let assign106550_e158655: f64 = (4.0 * locals.var_t1);let assign106550_e158658: f64 = (p.p337 * locals.var_gth);let assign106550_e158659: f64 = (assign106550_e158655 * assign106550_e158658);(locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14, ) = (assign106550_e158659, (((4.0 * locals.var_t1_dn0) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn0))), (((4.0 * locals.var_t1_dn2) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn2))), (((4.0 * locals.var_t1_dn4) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn4))), (((4.0 * locals.var_t1_dn5) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn5))), (((4.0 * locals.var_t1_dn6) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn6))), (((4.0 * locals.var_t1_dn7) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn7))), (((4.0 * locals.var_t1_dn8) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn8))), (((4.0 * locals.var_t1_dn9) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn9))), (((4.0 * locals.var_t1_dn10) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn10))), (((4.0 * locals.var_t1_dn11) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn11))), (((4.0 * locals.var_t1_dn14) * assign106550_e158658) + (assign106550_e158655 * (p.p337 * locals.var_gth_dn14))), );locals.var_tmf2_rv = 0.0;}
        if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {
            let (assign106560_e158671, assign106560_e158671_d_n0, assign106560_e158671_d_n2, assign106560_e158671_d_n4, assign106560_e158671_d_n5, assign106560_e158671_d_n6, assign106560_e158671_d_n7, assign106560_e158671_d_n8, assign106560_e158671_d_n9, assign106560_e158671_d_n10, assign106560_e158671_d_n11, assign106560_e158671_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign106560_e158670: f64 = (-locals.var_tmf2);
        (assign106560_e158670, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
            (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14, ) = (assign106560_e158671, assign106560_e158671_d_n0, assign106560_e158671_d_n2, assign106560_e158671_d_n4, assign106560_e158671_d_n5, assign106560_e158671_d_n6, assign106560_e158671_d_n7, assign106560_e158671_d_n8, assign106560_e158671_d_n9, assign106560_e158671_d_n10, assign106560_e158671_d_n11, assign106560_e158671_d_n14, );locals.var_tmf2_rv = 0.0;
        }
        if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {let assign106570_e158679: f64 = (locals.var_tmf1 * locals.var_tmf1);let assign106570_e158681: f64 = (assign106570_e158679 + locals.var_tmf2);let assign106570_e158682: f64 = (assign106570_e158681).sqrt();(locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14, ) = (assign106570_e158682, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign106570_e158682)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign106570_e158682)), );locals.var_tmf2_rv = 0.0;}
        if ((locals.var_guard2404 != 0.0) && (locals.var_guard2408 != 0.0)) {let assign106580_e158692: f64 = (locals.var_tmf1 / locals.var_tmf2);let assign106580_e158693: f64 = (1.0 + assign106580_e158692);let assign106580_e158694: f64 = (0.5 * assign106580_e158693);(locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14, ) = (assign106580_e158694, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))), );locals.var_t0_rv = 0.0;let assign106590_e158704: f64 = (locals.var_tmf1 + locals.var_tmf2);let assign106590_e158705: f64 = (0.5 * assign106590_e158704);let assign106590_e158706: f64 = (locals.var_t1 - assign106590_e158705);(locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14, ) = (assign106590_e158706, (locals.var_t1_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t1_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t1_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t1_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t1_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t1_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t1_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t1_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t1_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t1_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t1_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))), );locals.var_t2_rv = 0.0;(locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14, ) = (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14, );locals.var_p_rv = 0.0;}
        if (locals.var_guard2404 == 0.0) {(locals.var_gth, locals.var_gth_dn0, locals.var_gth_dn2, locals.var_gth_dn4, locals.var_gth_dn5, locals.var_gth_dn6, locals.var_gth_dn7, locals.var_gth_dn8, locals.var_gth_dn9, locals.var_gth_dn10, locals.var_gth_dn11, locals.var_gth_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_gth_rv = 0.0;(locals.var_p, locals.var_p_dn0, locals.var_p_dn2, locals.var_p_dn4, locals.var_p_dn5, locals.var_p_dn6, locals.var_p_dn7, locals.var_p_dn8, locals.var_p_dn9, locals.var_p_dn10, locals.var_p_dn11, locals.var_p_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_p_rv = 0.0;}
        if (locals.var_flg_nqs != 0.0) {let assign106690_e158762: f64 = (locals.var_qi_nqs * locals.var_qdrat);(locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn14, ) = (assign106690_e158762, (locals.var_qi_nqs * locals.var_qdrat_dn0), (locals.var_qi_nqs * locals.var_qdrat_dn2), (locals.var_qi_nqs * locals.var_qdrat_dn4), (locals.var_qi_nqs * locals.var_qdrat_dn5), (locals.var_qi_nqs * locals.var_qdrat_dn6), (locals.var_qi_nqs * locals.var_qdrat_dn7), (locals.var_qi_nqs * locals.var_qdrat_dn8), (locals.var_qi_nqs * locals.var_qdrat_dn9), (locals.var_qi_nqs * locals.var_qdrat_dn10), (locals.var_qi_nqs * locals.var_qdrat_dn11), (locals.var_qi_nqs_dn12 * locals.var_qdrat), (locals.var_qi_nqs * locals.var_qdrat_dn14), );locals.var_qd_nqs_rv = 0.0;let assign106700_e158767: f64 = (-locals.var_qi_nqs);let assign106700_e158769: f64 = (assign106700_e158767 - locals.var_qb_nqs);(locals.var_qg_nqs, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, ) = (assign106700_e158769, (-locals.var_qi_nqs_dn12), (-locals.var_qb_nqs_dn13), );locals.var_qg_nqs_rv = 0.0;let assign106710_e158776: f64 = (1.0 - locals.var_qdrat);let assign106710_e158777: f64 = (locals.var_qi_nqs * assign106710_e158776);(locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn14, ) = (assign106710_e158777, (locals.var_qi_nqs * (-locals.var_qdrat_dn0)), (locals.var_qi_nqs * (-locals.var_qdrat_dn2)), (locals.var_qi_nqs * (-locals.var_qdrat_dn4)), (locals.var_qi_nqs * (-locals.var_qdrat_dn5)), (locals.var_qi_nqs * (-locals.var_qdrat_dn6)), (locals.var_qi_nqs * (-locals.var_qdrat_dn7)), (locals.var_qi_nqs * (-locals.var_qdrat_dn8)), (locals.var_qi_nqs * (-locals.var_qdrat_dn9)), (locals.var_qi_nqs * (-locals.var_qdrat_dn10)), (locals.var_qi_nqs * (-locals.var_qdrat_dn11)), (locals.var_qi_nqs_dn12 * assign106710_e158776), (locals.var_qi_nqs * (-locals.var_qdrat_dn14)), );locals.var_qs_nqs_rv = 0.0;}
        if (locals.var_flg_nqs == 0.0) {(locals.var_qd_nqs, locals.var_qd_nqs_dn0, locals.var_qd_nqs_dn2, locals.var_qd_nqs_dn4, locals.var_qd_nqs_dn5, locals.var_qd_nqs_dn6, locals.var_qd_nqs_dn7, locals.var_qd_nqs_dn8, locals.var_qd_nqs_dn9, locals.var_qd_nqs_dn10, locals.var_qd_nqs_dn11, locals.var_qd_nqs_dn12, locals.var_qd_nqs_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_qd_nqs_rv = 0.0;(locals.var_qg_nqs, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, ) = (0.0, 0.0, 0.0, );locals.var_qg_nqs_rv = 0.0;(locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn4, locals.var_qs_nqs_dn5, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn8, locals.var_qs_nqs_dn9, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn14, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_qs_nqs_rv = 0.0;}
        let assign106770_e158807: f64 = (p.p87 * locals.var_mode);let assign106770_e158809: f64 = (assign106770_e158807 * locals.var_ids);(locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn4, locals.var_idse_dn5, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn8, locals.var_idse_dn9, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn14, ) = (assign106770_e158809, (assign106770_e158807 * locals.var_ids_dn0), (assign106770_e158807 * locals.var_ids_dn2), (assign106770_e158807 * locals.var_ids_dn4), (assign106770_e158807 * locals.var_ids_dn5), (assign106770_e158807 * locals.var_ids_dn6), (assign106770_e158807 * locals.var_ids_dn7), (assign106770_e158807 * locals.var_ids_dn8), (assign106770_e158807 * locals.var_ids_dn9), (assign106770_e158807 * locals.var_ids_dn10), (assign106770_e158807 * locals.var_ids_dn11), (assign106770_e158807 * locals.var_ids_dn14), );locals.var_idse_rv = 0.0;let assign106930_e158857: f64 = locals.var_qg_dn6;(locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14, ) = (assign106930_e158857, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_cgdbd_rv = 0.0;let assign106940_e158860: f64 = (p.p87 * locals.var_cgdbd);(locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14, ) = (assign106940_e158860, (p.p87 * locals.var_cgdbd_dn0), (p.p87 * locals.var_cgdbd_dn2), (p.p87 * locals.var_cgdbd_dn4), (p.p87 * locals.var_cgdbd_dn5), (p.p87 * locals.var_cgdbd_dn6), (p.p87 * locals.var_cgdbd_dn7), (p.p87 * locals.var_cgdbd_dn8), (p.p87 * locals.var_cgdbd_dn9), (p.p87 * locals.var_cgdbd_dn10), (p.p87 * locals.var_cgdbd_dn11), (p.p87 * locals.var_cgdbd_dn14), );locals.var_cgdbd_rv = 0.0;let assign106950_e158863: f64 = locals.var_qg_dn8;(locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14, ) = (assign106950_e158863, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );locals.var_cgsbd_rv = 0.0;let assign106960_e158866: f64 = (p.p87 * locals.var_cgsbd);(locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14, ) = (assign106960_e158866, (p.p87 * locals.var_cgsbd_dn0), (p.p87 * locals.var_cgsbd_dn2), (p.p87 * locals.var_cgsbd_dn4), (p.p87 * locals.var_cgsbd_dn5), (p.p87 * locals.var_cgsbd_dn6), (p.p87 * locals.var_cgsbd_dn7), (p.p87 * locals.var_cgsbd_dn8), (p.p87 * locals.var_cgsbd_dn9), (p.p87 * locals.var_cgsbd_dn10), (p.p87 * locals.var_cgsbd_dn11), (p.p87 * locals.var_cgsbd_dn14), );locals.var_cgsbd_rv = 0.0;let assign107330_e158981: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };locals.var_guard2411 = assign107330_e158981;locals.var_guard2411_rv = 0.0;
        if (locals.var_guard2411 != 0.0) {(locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn14, ) = (locals.var_cgsbd, locals.var_cgsbd_dn0, locals.var_cgsbd_dn2, locals.var_cgsbd_dn4, locals.var_cgsbd_dn5, locals.var_cgsbd_dn6, locals.var_cgsbd_dn7, locals.var_cgsbd_dn8, locals.var_cgsbd_dn9, locals.var_cgsbd_dn10, locals.var_cgsbd_dn11, locals.var_cgsbd_dn14, );locals.var_cgsb_rv = 0.0;}
        if (locals.var_guard2411 == 0.0) {(locals.var_cgsb, locals.var_cgsb_dn0, locals.var_cgsb_dn2, locals.var_cgsb_dn4, locals.var_cgsb_dn5, locals.var_cgsb_dn6, locals.var_cgsb_dn7, locals.var_cgsb_dn8, locals.var_cgsb_dn9, locals.var_cgsb_dn10, locals.var_cgsb_dn11, locals.var_cgsb_dn14, ) = (locals.var_cgdbd, locals.var_cgdbd_dn0, locals.var_cgdbd_dn2, locals.var_cgdbd_dn4, locals.var_cgdbd_dn5, locals.var_cgdbd_dn6, locals.var_cgdbd_dn7, locals.var_cgdbd_dn8, locals.var_cgdbd_dn9, locals.var_cgdbd_dn10, locals.var_cgdbd_dn11, locals.var_cgdbd_dn14, );locals.var_cgsb_rv = 0.0;}
        let assign107690_e159100: f64 = if p.p48 > 0.0 { 1.0 } else { 0.0 };locals.var_guard2413 = assign107690_e159100;locals.var_guard2413_rv = 0.0;
        if (p.p28 != 0.0) {locals.var_cqi = 1.0;locals.var_cqi_rv = 0.0;locals.var_cqb = 1.0;locals.var_cqb_rv = 0.0;}
    }
    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        let (eq1_e1022, eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16,) = {
    if (locals.var_guard2313 != 0.0) {
        let eq1_e1019: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_q_nqs_a);let eq1_e1020: f64 = (locals.var_inqs0_a + eq1_e1019);let eq1_e1020_d_n16: f64 = (locals.var_inqs0_a_dn16 + (locals.var_q_nqs_a_dn16 * ddt_scale));
        (eq1_e1020, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn11, locals.var_inqs0_a_dn14, eq1_e1020_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1022;let eq1_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 16];let eq1_node_derivatives: [f64; 12] = [eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16];let eq1_branch_derivative_indices: [usize; 0] = [];let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            None,
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1029, eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17,) = {
    if (locals.var_guard2313 != 0.0) {
        let eq2_e1026: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_q_nqs_k);let eq2_e1027: f64 = (locals.var_inqs0_k + eq2_e1026);let eq2_e1027_d_n17: f64 = (locals.var_inqs0_k_dn17 + (locals.var_q_nqs_k_dn17 * ddt_scale));
        (eq2_e1027, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn11, locals.var_inqs0_k_dn14, eq2_e1027_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1029;let eq2_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 17];let eq2_node_derivatives: [f64; 12] = [eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17];let eq2_branch_derivative_indices: [usize; 0] = [];let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(17),
            None,
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1046, eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18,) = {
    if (locals.var_guard2314 != 0.0) {
        let eq5_e1043: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_w_nqs_a);let eq5_e1044: f64 = (locals.var_iwnqs0_a + eq5_e1043);let eq5_e1044_d_n18: f64 = (locals.var_iwnqs0_a_dn18 + (locals.var_w_nqs_a_dn18 * ddt_scale));
        (eq5_e1044, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn11, locals.var_iwnqs0_a_dn14, eq5_e1044_d_n18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1046;let eq5_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 18];let eq5_node_derivatives: [f64; 12] = [eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18];let eq5_branch_derivative_indices: [usize; 0] = [];let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(18),
            None,
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );let eq8_e1062: f64 = (locals.var_ibreak - locals.var_ibreaks);let eq8_e1062_d_n0: f64 = (locals.var_ibreak_dn0 - locals.var_ibreaks_dn0);let eq8_e1062_d_n2: f64 = (locals.var_ibreak_dn2 - locals.var_ibreaks_dn2);let eq8_e1062_d_n4: f64 = (locals.var_ibreak_dn4 - locals.var_ibreaks_dn4);let eq8_e1062_d_n5: f64 = (locals.var_ibreak_dn5 - locals.var_ibreaks_dn5);let eq8_e1062_d_n6: f64 = (locals.var_ibreak_dn6 - locals.var_ibreaks_dn6);let eq8_e1062_d_n7: f64 = (locals.var_ibreak_dn7 - locals.var_ibreaks_dn7);let eq8_e1062_d_n8: f64 = (locals.var_ibreak_dn8 - locals.var_ibreaks_dn8);let eq8_e1062_d_n9: f64 = (locals.var_ibreak_dn9 - locals.var_ibreaks_dn9);let eq8_e1062_d_n10: f64 = (locals.var_ibreak_dn10 - locals.var_ibreaks_dn10);let eq8_e1062_d_n11: f64 = (locals.var_ibreak_dn11 - locals.var_ibreaks_dn11);let eq8_e1062_d_n14: f64 = (locals.var_ibreak_dn14 - locals.var_ibreaks_dn14);let eq8_e1063: f64 = (p.p87 * eq8_e1062);let eq8_e1063_d_n0: f64 = (p.p87 * eq8_e1062_d_n0);let eq8_e1063_d_n2: f64 = (p.p87 * eq8_e1062_d_n2);let eq8_e1063_d_n4: f64 = (p.p87 * eq8_e1062_d_n4);let eq8_e1063_d_n5: f64 = (p.p87 * eq8_e1062_d_n5);let eq8_e1063_d_n6: f64 = (p.p87 * eq8_e1062_d_n6);let eq8_e1063_d_n7: f64 = (p.p87 * eq8_e1062_d_n7);let eq8_e1063_d_n8: f64 = (p.p87 * eq8_e1062_d_n8);let eq8_e1063_d_n9: f64 = (p.p87 * eq8_e1062_d_n9);let eq8_e1063_d_n10: f64 = (p.p87 * eq8_e1062_d_n10);let eq8_e1063_d_n11: f64 = (p.p87 * eq8_e1062_d_n11);let eq8_e1063_d_n14: f64 = (p.p87 * eq8_e1062_d_n14);let eq8_value: f64 = eq8_e1063;let eq8_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq8_node_derivatives: [f64; 11] = [eq8_e1063_d_n0, eq8_e1063_d_n2, eq8_e1063_d_n4, eq8_e1063_d_n5, eq8_e1063_d_n6, eq8_e1063_d_n7, eq8_e1063_d_n8, eq8_e1063_d_n9, eq8_e1063_d_n10, eq8_e1063_d_n11, eq8_e1063_d_n14];let eq8_branch_derivative_indices: [usize; 0] = [];let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );let eq13_e1086: f64 = (p.p87 * locals.var_ibs);let eq13_e1086_d_n0: f64 = (p.p87 * locals.var_ibs_dn0);let eq13_e1086_d_n2: f64 = (p.p87 * locals.var_ibs_dn2);let eq13_e1086_d_n4: f64 = (p.p87 * locals.var_ibs_dn4);let eq13_e1086_d_n5: f64 = (p.p87 * locals.var_ibs_dn5);let eq13_e1086_d_n6: f64 = (p.p87 * locals.var_ibs_dn6);let eq13_e1086_d_n7: f64 = (p.p87 * locals.var_ibs_dn7);let eq13_e1086_d_n8: f64 = (p.p87 * locals.var_ibs_dn8);let eq13_e1086_d_n9: f64 = (p.p87 * locals.var_ibs_dn9);let eq13_e1086_d_n10: f64 = (p.p87 * locals.var_ibs_dn10);let eq13_e1086_d_n11: f64 = (p.p87 * locals.var_ibs_dn11);let eq13_e1086_d_n14: f64 = (p.p87 * locals.var_ibs_dn14);let eq13_value: f64 = eq13_e1086;let eq13_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq13_node_derivatives: [f64; 11] = [eq13_e1086_d_n0, eq13_e1086_d_n2, eq13_e1086_d_n4, eq13_e1086_d_n5, eq13_e1086_d_n6, eq13_e1086_d_n7, eq13_e1086_d_n8, eq13_e1086_d_n9, eq13_e1086_d_n10, eq13_e1086_d_n11, eq13_e1086_d_n14];let eq13_branch_derivative_indices: [usize; 0] = [];let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );let eq14_e1089: f64 = (p.p87 * locals.var_ibd);let eq14_e1089_d_n0: f64 = (p.p87 * locals.var_ibd_dn0);let eq14_e1089_d_n2: f64 = (p.p87 * locals.var_ibd_dn2);let eq14_e1089_d_n4: f64 = (p.p87 * locals.var_ibd_dn4);let eq14_e1089_d_n5: f64 = (p.p87 * locals.var_ibd_dn5);let eq14_e1089_d_n6: f64 = (p.p87 * locals.var_ibd_dn6);let eq14_e1089_d_n7: f64 = (p.p87 * locals.var_ibd_dn7);let eq14_e1089_d_n8: f64 = (p.p87 * locals.var_ibd_dn8);let eq14_e1089_d_n9: f64 = (p.p87 * locals.var_ibd_dn9);let eq14_e1089_d_n10: f64 = (p.p87 * locals.var_ibd_dn10);let eq14_e1089_d_n11: f64 = (p.p87 * locals.var_ibd_dn11);let eq14_e1089_d_n14: f64 = (p.p87 * locals.var_ibd_dn14);let eq14_value: f64 = eq14_e1089;let eq14_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq14_node_derivatives: [f64; 11] = [eq14_e1089_d_n0, eq14_e1089_d_n2, eq14_e1089_d_n4, eq14_e1089_d_n5, eq14_e1089_d_n6, eq14_e1089_d_n7, eq14_e1089_d_n8, eq14_e1089_d_n9, eq14_e1089_d_n10, eq14_e1089_d_n11, eq14_e1089_d_n14];let eq14_branch_derivative_indices: [usize; 0] = [];let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq14_value),
            &eq14_node_derivative_indices,
            &eq14_node_derivatives,
            &eq14_branch_derivative_indices,
            &eq14_branch_derivatives,
            multiplicity,
        );let eq15_e1092: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qbs);let eq15_e1093: f64 = (p.p87 * eq15_e1092);let eq15_e1093_d_n0: f64 = (p.p87 * (locals.var_qbs_dn0 * ddt_scale));let eq15_e1093_d_n2: f64 = (p.p87 * (locals.var_qbs_dn2 * ddt_scale));let eq15_e1093_d_n4: f64 = (p.p87 * (locals.var_qbs_dn4 * ddt_scale));let eq15_e1093_d_n5: f64 = (p.p87 * (locals.var_qbs_dn5 * ddt_scale));let eq15_e1093_d_n6: f64 = (p.p87 * (locals.var_qbs_dn6 * ddt_scale));let eq15_e1093_d_n7: f64 = (p.p87 * (locals.var_qbs_dn7 * ddt_scale));let eq15_e1093_d_n8: f64 = (p.p87 * (locals.var_qbs_dn8 * ddt_scale));let eq15_e1093_d_n9: f64 = (p.p87 * (locals.var_qbs_dn9 * ddt_scale));let eq15_e1093_d_n10: f64 = (p.p87 * (locals.var_qbs_dn10 * ddt_scale));let eq15_e1093_d_n11: f64 = (p.p87 * (locals.var_qbs_dn11 * ddt_scale));let eq15_e1093_d_n14: f64 = (p.p87 * (locals.var_qbs_dn14 * ddt_scale));let eq15_value: f64 = eq15_e1093;let eq15_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq15_node_derivatives: [f64; 11] = [eq15_e1093_d_n0, eq15_e1093_d_n2, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, eq15_e1093_d_n14];let eq15_branch_derivative_indices: [usize; 0] = [];let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq15_value),
            &eq15_node_derivative_indices,
            &eq15_node_derivatives,
            &eq15_branch_derivative_indices,
            &eq15_branch_derivatives,
            multiplicity,
        );let eq16_e1096: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qbd);let eq16_e1097: f64 = (p.p87 * eq16_e1096);let eq16_e1097_d_n0: f64 = (p.p87 * (locals.var_qbd_dn0 * ddt_scale));let eq16_e1097_d_n2: f64 = (p.p87 * (locals.var_qbd_dn2 * ddt_scale));let eq16_e1097_d_n4: f64 = (p.p87 * (locals.var_qbd_dn4 * ddt_scale));let eq16_e1097_d_n5: f64 = (p.p87 * (locals.var_qbd_dn5 * ddt_scale));let eq16_e1097_d_n6: f64 = (p.p87 * (locals.var_qbd_dn6 * ddt_scale));let eq16_e1097_d_n7: f64 = (p.p87 * (locals.var_qbd_dn7 * ddt_scale));let eq16_e1097_d_n8: f64 = (p.p87 * (locals.var_qbd_dn8 * ddt_scale));let eq16_e1097_d_n9: f64 = (p.p87 * (locals.var_qbd_dn9 * ddt_scale));let eq16_e1097_d_n10: f64 = (p.p87 * (locals.var_qbd_dn10 * ddt_scale));let eq16_e1097_d_n11: f64 = (p.p87 * (locals.var_qbd_dn11 * ddt_scale));let eq16_e1097_d_n14: f64 = (p.p87 * (locals.var_qbd_dn14 * ddt_scale));let eq16_e1097_d_n16: f64 = (p.p87 * (locals.var_qbd_dn16 * ddt_scale));let eq16_e1097_d_n17: f64 = (p.p87 * (locals.var_qbd_dn17 * ddt_scale));let eq16_e1097_d_n18: f64 = (p.p87 * (locals.var_qbd_dn18 * ddt_scale));let eq16_value: f64 = eq16_e1097;let eq16_node_derivative_indices: [usize; 14] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 16, 17, 18];let eq16_node_derivatives: [f64; 14] = [eq16_e1097_d_n0, eq16_e1097_d_n2, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, eq16_e1097_d_n14, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_n18];let eq16_branch_derivative_indices: [usize; 0] = [];let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq16_value),
            &eq16_node_derivative_indices,
            &eq16_node_derivatives,
            &eq16_branch_derivative_indices,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1116, eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14,) = {
    if (locals.var_guard2413 != 0.0) {
        let eq19_e1113: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, locals.var_qbsi);let eq19_e1114: f64 = (p.p87 * eq19_e1113);let eq19_e1114_d_n0: f64 = (p.p87 * (locals.var_qbsi_dn0 * ddt_scale));let eq19_e1114_d_n2: f64 = (p.p87 * (locals.var_qbsi_dn2 * ddt_scale));let eq19_e1114_d_n4: f64 = (p.p87 * (locals.var_qbsi_dn4 * ddt_scale));let eq19_e1114_d_n5: f64 = (p.p87 * (locals.var_qbsi_dn5 * ddt_scale));let eq19_e1114_d_n6: f64 = (p.p87 * (locals.var_qbsi_dn6 * ddt_scale));let eq19_e1114_d_n7: f64 = (p.p87 * (locals.var_qbsi_dn7 * ddt_scale));let eq19_e1114_d_n8: f64 = (p.p87 * (locals.var_qbsi_dn8 * ddt_scale));let eq19_e1114_d_n9: f64 = (p.p87 * (locals.var_qbsi_dn9 * ddt_scale));let eq19_e1114_d_n10: f64 = (p.p87 * (locals.var_qbsi_dn10 * ddt_scale));let eq19_e1114_d_n11: f64 = (p.p87 * (locals.var_qbsi_dn11 * ddt_scale));let eq19_e1114_d_n14: f64 = (p.p87 * (locals.var_qbsi_dn14 * ddt_scale));
        (eq19_e1114, eq19_e1114_d_n0, eq19_e1114_d_n2, eq19_e1114_d_n4, eq19_e1114_d_n5, eq19_e1114_d_n6, eq19_e1114_d_n7, eq19_e1114_d_n8, eq19_e1114_d_n9, eq19_e1114_d_n10, eq19_e1114_d_n11, eq19_e1114_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1116;let eq19_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq19_node_derivatives: [f64; 11] = [eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14];let eq19_branch_derivative_indices: [usize; 0] = [];let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14,) = {
    if (locals.var_guard2413 != 0.0) {
        let eq20_e1120: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbdi);let eq20_e1121: f64 = (p.p87 * eq20_e1120);let eq20_e1121_d_n0: f64 = (p.p87 * (locals.var_qbdi_dn0 * ddt_scale));let eq20_e1121_d_n2: f64 = (p.p87 * (locals.var_qbdi_dn2 * ddt_scale));let eq20_e1121_d_n4: f64 = (p.p87 * (locals.var_qbdi_dn4 * ddt_scale));let eq20_e1121_d_n5: f64 = (p.p87 * (locals.var_qbdi_dn5 * ddt_scale));let eq20_e1121_d_n6: f64 = (p.p87 * (locals.var_qbdi_dn6 * ddt_scale));let eq20_e1121_d_n7: f64 = (p.p87 * (locals.var_qbdi_dn7 * ddt_scale));let eq20_e1121_d_n8: f64 = (p.p87 * (locals.var_qbdi_dn8 * ddt_scale));let eq20_e1121_d_n9: f64 = (p.p87 * (locals.var_qbdi_dn9 * ddt_scale));let eq20_e1121_d_n10: f64 = (p.p87 * (locals.var_qbdi_dn10 * ddt_scale));let eq20_e1121_d_n11: f64 = (p.p87 * (locals.var_qbdi_dn11 * ddt_scale));let eq20_e1121_d_n14: f64 = (p.p87 * (locals.var_qbdi_dn14 * ddt_scale));
        (eq20_e1121, eq20_e1121_d_n0, eq20_e1121_d_n2, eq20_e1121_d_n4, eq20_e1121_d_n5, eq20_e1121_d_n6, eq20_e1121_d_n7, eq20_e1121_d_n8, eq20_e1121_d_n9, eq20_e1121_d_n10, eq20_e1121_d_n11, eq20_e1121_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1123;let eq20_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq20_node_derivatives: [f64; 11] = [eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14];let eq20_branch_derivative_indices: [usize; 0] = [];let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );let eq28_e1167: f64 = (locals.var_qg + locals.var_qg_nqs);let eq28_e1168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq28_e1167);let eq28_e1169: f64 = (p.p87 * eq28_e1168);let eq28_e1169_d_n0: f64 = (p.p87 * (locals.var_qg_dn0 * ddt_scale));let eq28_e1169_d_n2: f64 = (p.p87 * (locals.var_qg_dn2 * ddt_scale));let eq28_e1169_d_n4: f64 = (p.p87 * (locals.var_qg_dn4 * ddt_scale));let eq28_e1169_d_n5: f64 = (p.p87 * (locals.var_qg_dn5 * ddt_scale));let eq28_e1169_d_n6: f64 = (p.p87 * (locals.var_qg_dn6 * ddt_scale));let eq28_e1169_d_n7: f64 = (p.p87 * (locals.var_qg_dn7 * ddt_scale));let eq28_e1169_d_n8: f64 = (p.p87 * (locals.var_qg_dn8 * ddt_scale));let eq28_e1169_d_n9: f64 = (p.p87 * (locals.var_qg_dn9 * ddt_scale));let eq28_e1169_d_n10: f64 = (p.p87 * (locals.var_qg_dn10 * ddt_scale));let eq28_e1169_d_n11: f64 = (p.p87 * (locals.var_qg_dn11 * ddt_scale));let eq28_e1169_d_n12: f64 = (p.p87 * (locals.var_qg_nqs_dn12 * ddt_scale));let eq28_e1169_d_n13: f64 = (p.p87 * (locals.var_qg_nqs_dn13 * ddt_scale));let eq28_e1169_d_n14: f64 = (p.p87 * (locals.var_qg_dn14 * ddt_scale));let eq28_value: f64 = eq28_e1169;let eq28_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];let eq28_node_derivatives: [f64; 13] = [eq28_e1169_d_n0, eq28_e1169_d_n2, eq28_e1169_d_n4, eq28_e1169_d_n5, eq28_e1169_d_n6, eq28_e1169_d_n7, eq28_e1169_d_n8, eq28_e1169_d_n9, eq28_e1169_d_n10, eq28_e1169_d_n11, eq28_e1169_d_n12, eq28_e1169_d_n13, eq28_e1169_d_n14];let eq28_branch_derivative_indices: [usize; 0] = [];let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );let eq29_e1173: f64 = (locals.var_qd + locals.var_qd_nqs);let eq29_e1173_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);let eq29_e1173_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);let eq29_e1173_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);let eq29_e1173_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);let eq29_e1173_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);let eq29_e1173_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qd_nqs_dn7);let eq29_e1173_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);let eq29_e1173_d_n9: f64 = (locals.var_qd_dn9 + locals.var_qd_nqs_dn9);let eq29_e1173_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);let eq29_e1173_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);let eq29_e1173_d_n14: f64 = (locals.var_qd_dn14 + locals.var_qd_nqs_dn14);let eq29_e1174: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq29_e1173);let eq29_e1175: f64 = (p.p87 * eq29_e1174);let eq29_e1175_d_n0: f64 = (p.p87 * (eq29_e1173_d_n0 * ddt_scale));let eq29_e1175_d_n2: f64 = (p.p87 * (eq29_e1173_d_n2 * ddt_scale));let eq29_e1175_d_n4: f64 = (p.p87 * (eq29_e1173_d_n4 * ddt_scale));let eq29_e1175_d_n5: f64 = (p.p87 * (eq29_e1173_d_n5 * ddt_scale));let eq29_e1175_d_n6: f64 = (p.p87 * (eq29_e1173_d_n6 * ddt_scale));let eq29_e1175_d_n7: f64 = (p.p87 * (eq29_e1173_d_n7 * ddt_scale));let eq29_e1175_d_n8: f64 = (p.p87 * (eq29_e1173_d_n8 * ddt_scale));let eq29_e1175_d_n9: f64 = (p.p87 * (eq29_e1173_d_n9 * ddt_scale));let eq29_e1175_d_n10: f64 = (p.p87 * (eq29_e1173_d_n10 * ddt_scale));let eq29_e1175_d_n11: f64 = (p.p87 * (eq29_e1173_d_n11 * ddt_scale));let eq29_e1175_d_n12: f64 = (p.p87 * (locals.var_qd_nqs_dn12 * ddt_scale));let eq29_e1175_d_n14: f64 = (p.p87 * (eq29_e1173_d_n14 * ddt_scale));let eq29_value: f64 = eq29_e1175;let eq29_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14];let eq29_node_derivatives: [f64; 12] = [eq29_e1175_d_n0, eq29_e1175_d_n2, eq29_e1175_d_n4, eq29_e1175_d_n5, eq29_e1175_d_n6, eq29_e1175_d_n7, eq29_e1175_d_n8, eq29_e1175_d_n9, eq29_e1175_d_n10, eq29_e1175_d_n11, eq29_e1175_d_n12, eq29_e1175_d_n14];let eq29_branch_derivative_indices: [usize; 0] = [];let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );let eq30_e1180: f64 = (locals.var_qg_nqs + locals.var_qd_nqs);let eq30_e1180_d_n12: f64 = (locals.var_qg_nqs_dn12 + locals.var_qd_nqs_dn12);let eq30_e1182: f64 = (eq30_e1180 + locals.var_qs_nqs);let eq30_e1182_d_n0: f64 = (locals.var_qd_nqs_dn0 + locals.var_qs_nqs_dn0);let eq30_e1182_d_n2: f64 = (locals.var_qd_nqs_dn2 + locals.var_qs_nqs_dn2);let eq30_e1182_d_n4: f64 = (locals.var_qd_nqs_dn4 + locals.var_qs_nqs_dn4);let eq30_e1182_d_n5: f64 = (locals.var_qd_nqs_dn5 + locals.var_qs_nqs_dn5);let eq30_e1182_d_n6: f64 = (locals.var_qd_nqs_dn6 + locals.var_qs_nqs_dn6);let eq30_e1182_d_n7: f64 = (locals.var_qd_nqs_dn7 + locals.var_qs_nqs_dn7);let eq30_e1182_d_n8: f64 = (locals.var_qd_nqs_dn8 + locals.var_qs_nqs_dn8);let eq30_e1182_d_n9: f64 = (locals.var_qd_nqs_dn9 + locals.var_qs_nqs_dn9);let eq30_e1182_d_n10: f64 = (locals.var_qd_nqs_dn10 + locals.var_qs_nqs_dn10);let eq30_e1182_d_n11: f64 = (locals.var_qd_nqs_dn11 + locals.var_qs_nqs_dn11);let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + locals.var_qs_nqs_dn12);let eq30_e1182_d_n14: f64 = (locals.var_qd_nqs_dn14 + locals.var_qs_nqs_dn14);let eq30_e1183: f64 = (locals.var_qb - eq30_e1182);let eq30_e1183_d_n0: f64 = (locals.var_qb_dn0 - eq30_e1182_d_n0);let eq30_e1183_d_n2: f64 = (locals.var_qb_dn2 - eq30_e1182_d_n2);let eq30_e1183_d_n4: f64 = (locals.var_qb_dn4 - eq30_e1182_d_n4);let eq30_e1183_d_n5: f64 = (locals.var_qb_dn5 - eq30_e1182_d_n5);let eq30_e1183_d_n6: f64 = (locals.var_qb_dn6 - eq30_e1182_d_n6);let eq30_e1183_d_n7: f64 = (locals.var_qb_dn7 - eq30_e1182_d_n7);let eq30_e1183_d_n8: f64 = (locals.var_qb_dn8 - eq30_e1182_d_n8);let eq30_e1183_d_n9: f64 = (locals.var_qb_dn9 - eq30_e1182_d_n9);let eq30_e1183_d_n10: f64 = (locals.var_qb_dn10 - eq30_e1182_d_n10);let eq30_e1183_d_n11: f64 = (locals.var_qb_dn11 - eq30_e1182_d_n11);let eq30_e1183_d_n14: f64 = (locals.var_qb_dn14 - eq30_e1182_d_n14);let eq30_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq30_e1183);let eq30_e1185: f64 = (p.p87 * eq30_e1184);let eq30_e1185_d_n0: f64 = (p.p87 * (eq30_e1183_d_n0 * ddt_scale));let eq30_e1185_d_n2: f64 = (p.p87 * (eq30_e1183_d_n2 * ddt_scale));let eq30_e1185_d_n4: f64 = (p.p87 * (eq30_e1183_d_n4 * ddt_scale));let eq30_e1185_d_n5: f64 = (p.p87 * (eq30_e1183_d_n5 * ddt_scale));let eq30_e1185_d_n6: f64 = (p.p87 * (eq30_e1183_d_n6 * ddt_scale));let eq30_e1185_d_n7: f64 = (p.p87 * (eq30_e1183_d_n7 * ddt_scale));let eq30_e1185_d_n8: f64 = (p.p87 * (eq30_e1183_d_n8 * ddt_scale));let eq30_e1185_d_n9: f64 = (p.p87 * (eq30_e1183_d_n9 * ddt_scale));let eq30_e1185_d_n10: f64 = (p.p87 * (eq30_e1183_d_n10 * ddt_scale));let eq30_e1185_d_n11: f64 = (p.p87 * (eq30_e1183_d_n11 * ddt_scale));let eq30_e1185_d_n12: f64 = (p.p87 * ((-eq30_e1182_d_n12) * ddt_scale));let eq30_e1185_d_n13: f64 = (p.p87 * ((-locals.var_qg_nqs_dn13) * ddt_scale));let eq30_e1185_d_n14: f64 = (p.p87 * (eq30_e1183_d_n14 * ddt_scale));let eq30_value: f64 = eq30_e1185;let eq30_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];let eq30_node_derivatives: [f64; 13] = [eq30_e1185_d_n0, eq30_e1185_d_n2, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14];let eq30_branch_derivative_indices: [usize; 0] = [];let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, locals.var_qgext);let eq31_e1189: f64 = (p.p87 * eq31_e1188);let eq31_e1189_d_n0: f64 = (p.p87 * (locals.var_qgext_dn0 * ddt_scale));let eq31_e1189_d_n2: f64 = (p.p87 * (locals.var_qgext_dn2 * ddt_scale));let eq31_e1189_d_n4: f64 = (p.p87 * (locals.var_qgext_dn4 * ddt_scale));let eq31_e1189_d_n5: f64 = (p.p87 * (locals.var_qgext_dn5 * ddt_scale));let eq31_e1189_d_n6: f64 = (p.p87 * (locals.var_qgext_dn6 * ddt_scale));let eq31_e1189_d_n7: f64 = (p.p87 * (locals.var_qgext_dn7 * ddt_scale));let eq31_e1189_d_n8: f64 = (p.p87 * (locals.var_qgext_dn8 * ddt_scale));let eq31_e1189_d_n9: f64 = (p.p87 * (locals.var_qgext_dn9 * ddt_scale));let eq31_e1189_d_n10: f64 = (p.p87 * (locals.var_qgext_dn10 * ddt_scale));let eq31_e1189_d_n11: f64 = (p.p87 * (locals.var_qgext_dn11 * ddt_scale));let eq31_e1189_d_n14: f64 = (p.p87 * (locals.var_qgext_dn14 * ddt_scale));let eq31_value: f64 = eq31_e1189;let eq31_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq31_node_derivatives: [f64; 11] = [eq31_e1189_d_n0, eq31_e1189_d_n2, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n14];let eq31_branch_derivative_indices: [usize; 0] = [];let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, locals.var_qdext);let eq32_e1193: f64 = (p.p87 * eq32_e1192);let eq32_e1193_d_n0: f64 = (p.p87 * (locals.var_qdext_dn0 * ddt_scale));let eq32_e1193_d_n2: f64 = (p.p87 * (locals.var_qdext_dn2 * ddt_scale));let eq32_e1193_d_n4: f64 = (p.p87 * (locals.var_qdext_dn4 * ddt_scale));let eq32_e1193_d_n5: f64 = (p.p87 * (locals.var_qdext_dn5 * ddt_scale));let eq32_e1193_d_n6: f64 = (p.p87 * (locals.var_qdext_dn6 * ddt_scale));let eq32_e1193_d_n7: f64 = (p.p87 * (locals.var_qdext_dn7 * ddt_scale));let eq32_e1193_d_n8: f64 = (p.p87 * (locals.var_qdext_dn8 * ddt_scale));let eq32_e1193_d_n9: f64 = (p.p87 * (locals.var_qdext_dn9 * ddt_scale));let eq32_e1193_d_n10: f64 = (p.p87 * (locals.var_qdext_dn10 * ddt_scale));let eq32_e1193_d_n11: f64 = (p.p87 * (locals.var_qdext_dn11 * ddt_scale));let eq32_e1193_d_n14: f64 = (p.p87 * (locals.var_qdext_dn14 * ddt_scale));let eq32_value: f64 = eq32_e1193;let eq32_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq32_node_derivatives: [f64; 11] = [eq32_e1193_d_n0, eq32_e1193_d_n2, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n14];let eq32_branch_derivative_indices: [usize; 0] = [];let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
            multiplicity,
        );let eq33_e1196: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, locals.var_qbext);let eq33_e1197: f64 = (p.p87 * eq33_e1196);let eq33_e1197_d_n0: f64 = (p.p87 * (locals.var_qbext_dn0 * ddt_scale));let eq33_e1197_d_n2: f64 = (p.p87 * (locals.var_qbext_dn2 * ddt_scale));let eq33_e1197_d_n4: f64 = (p.p87 * (locals.var_qbext_dn4 * ddt_scale));let eq33_e1197_d_n5: f64 = (p.p87 * (locals.var_qbext_dn5 * ddt_scale));let eq33_e1197_d_n6: f64 = (p.p87 * (locals.var_qbext_dn6 * ddt_scale));let eq33_e1197_d_n7: f64 = (p.p87 * (locals.var_qbext_dn7 * ddt_scale));let eq33_e1197_d_n8: f64 = (p.p87 * (locals.var_qbext_dn8 * ddt_scale));let eq33_e1197_d_n9: f64 = (p.p87 * (locals.var_qbext_dn9 * ddt_scale));let eq33_e1197_d_n10: f64 = (p.p87 * (locals.var_qbext_dn10 * ddt_scale));let eq33_e1197_d_n11: f64 = (p.p87 * (locals.var_qbext_dn11 * ddt_scale));let eq33_e1197_d_n14: f64 = (p.p87 * (locals.var_qbext_dn14 * ddt_scale));let eq33_value: f64 = eq33_e1197;let eq33_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];let eq33_node_derivatives: [f64; 11] = [eq33_e1197_d_n0, eq33_e1197_d_n2, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, eq33_e1197_d_n14];let eq33_branch_derivative_indices: [usize; 0] = [];let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq33_value),
            &eq33_node_derivative_indices,
            &eq33_node_derivatives,
            &eq33_branch_derivative_indices,
            &eq33_branch_derivatives,
            multiplicity,
        );
    }
    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
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
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let nv14 = ctx.node_voltage(nodes[14]);let nv15 = ctx.node_voltage(nodes[15]);let eq34_e1199: f64 = (-p.p87);let eq34_e1201: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, locals.var_qfd);let eq34_e1202: f64 = (eq34_e1199 * eq34_e1201);let eq34_e1202_d_n0: f64 = (eq34_e1199 * (locals.var_qfd_dn0 * ddt_scale));let eq34_e1202_d_n2: f64 = (eq34_e1199 * (locals.var_qfd_dn2 * ddt_scale));let eq34_e1202_d_n7: f64 = (eq34_e1199 * (locals.var_qfd_dn7 * ddt_scale));let eq34_value: f64 = eq34_e1202;
        stamper.stamp_current_node3_local(
            Some(7),
            Some(0),
            multiplicity * (eq34_value),
            0,
            multiplicity * (eq34_e1202_d_n0),
            2,
            multiplicity * (eq34_e1202_d_n2),
            7,
            multiplicity * (eq34_e1202_d_n7),
        );let eq35_e1204: f64 = (-p.p87);let eq35_e1206: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, locals.var_qfs);let eq35_e1207: f64 = (eq35_e1204 * eq35_e1206);let eq35_e1207_d_n2: f64 = (eq35_e1204 * (locals.var_qfs_dn2 * ddt_scale));let eq35_e1207_d_n7: f64 = (eq35_e1204 * (locals.var_qfs_dn7 * ddt_scale));let eq35_value: f64 = eq35_e1207;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(2),
            multiplicity * (eq35_value),
            2,
            multiplicity * (eq35_e1207_d_n2),
            7,
            multiplicity * (eq35_e1207_d_n7),
        );let eq40_e1233: f64 = (locals.var_ci * (nv15 - 0.0));let eq40_e1233_d_n0: f64 = (locals.var_ci_dn0 * (nv15 - 0.0));let eq40_e1233_d_n2: f64 = (locals.var_ci_dn2 * (nv15 - 0.0));let eq40_e1233_d_n4: f64 = (locals.var_ci_dn4 * (nv15 - 0.0));let eq40_e1233_d_n5: f64 = (locals.var_ci_dn5 * (nv15 - 0.0));let eq40_e1233_d_n6: f64 = (locals.var_ci_dn6 * (nv15 - 0.0));let eq40_e1233_d_n7: f64 = (locals.var_ci_dn7 * (nv15 - 0.0));let eq40_e1233_d_n8: f64 = (locals.var_ci_dn8 * (nv15 - 0.0));let eq40_e1233_d_n9: f64 = (locals.var_ci_dn9 * (nv15 - 0.0));let eq40_e1233_d_n10: f64 = (locals.var_ci_dn10 * (nv15 - 0.0));let eq40_e1233_d_n11: f64 = (locals.var_ci_dn11 * (nv15 - 0.0));let eq40_e1233_d_n14: f64 = (locals.var_ci_dn14 * (nv15 - 0.0));let eq40_value: f64 = eq40_e1233;let eq40_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];let eq40_node_derivatives: [f64; 12] = [eq40_e1233_d_n0, eq40_e1233_d_n2, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n14, locals.var_ci];let eq40_branch_derivative_indices: [usize; 0] = [];let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivative_indices,
            &eq40_node_derivatives,
            &eq40_branch_derivative_indices,
            &eq40_branch_derivatives,
            multiplicity,
        );let eq41_e1236: f64 = ((nv15 - 0.0) * locals.var_sigrat_s);let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn0);let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn2);let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn4);let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn5);let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn6);let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn7);let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn8);let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn9);let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn10);let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn11);let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn14);let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq41_e1236);let eq41_value: f64 = eq41_e1237;let eq41_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];let eq41_node_derivatives: [f64; 12] = [(eq41_e1236_d_n0 * ddt_scale), (eq41_e1236_d_n2 * ddt_scale), (eq41_e1236_d_n4 * ddt_scale), (eq41_e1236_d_n5 * ddt_scale), (eq41_e1236_d_n6 * ddt_scale), (eq41_e1236_d_n7 * ddt_scale), (eq41_e1236_d_n8 * ddt_scale), (eq41_e1236_d_n9 * ddt_scale), (eq41_e1236_d_n10 * ddt_scale), (eq41_e1236_d_n11 * ddt_scale), (eq41_e1236_d_n14 * ddt_scale), (locals.var_sigrat_s * ddt_scale)];let eq41_branch_derivative_indices: [usize; 0] = [];let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );let eq42_e1240: f64 = ((nv15 - 0.0) * locals.var_sigrat_d);let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn0);let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn2);let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn4);let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn5);let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn6);let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn7);let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn8);let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn9);let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn10);let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn11);let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn14);let eq42_e1241: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq42_e1240);let eq42_value: f64 = eq42_e1241;let eq42_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];let eq42_node_derivatives: [f64; 12] = [(eq42_e1240_d_n0 * ddt_scale), (eq42_e1240_d_n2 * ddt_scale), (eq42_e1240_d_n4 * ddt_scale), (eq42_e1240_d_n5 * ddt_scale), (eq42_e1240_d_n6 * ddt_scale), (eq42_e1240_d_n7 * ddt_scale), (eq42_e1240_d_n8 * ddt_scale), (eq42_e1240_d_n9 * ddt_scale), (eq42_e1240_d_n10 * ddt_scale), (eq42_e1240_d_n11 * ddt_scale), (eq42_e1240_d_n14 * ddt_scale), (locals.var_sigrat_d * ddt_scale)];let eq42_branch_derivative_indices: [usize; 0] = [];let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n12,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (locals.var_cqi * (nv12 - 0.0));let eq62_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq62_e1359);
        (eq62_e1360, (locals.var_cqi * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1362;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            12,
            multiplicity * (eq62_e1362_d_n12),
        );
        let (eq63_e1369, eq63_e1369_d_n13,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (locals.var_cqb * (nv13 - 0.0));let eq63_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq63_e1366);
        (eq63_e1367, (locals.var_cqb * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1369;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            13,
            multiplicity * (eq63_e1369_d_n13),
        );
        let (eq67_e1388, eq67_e1388_d_n14,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, (nv14 - 0.0));
        (eq67_e1386, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e1388;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq67_value),
            14,
            multiplicity * (eq67_e1388_d_n14),
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
        let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let nv14 = ctx.node_voltage(nodes[14]);let nv15 = ctx.node_voltage(nodes[15]);
        let (eq1_e1022, eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16, eq1_e1022_q, eq1_e1022_q_d_n16,) = {
    if (locals.var_guard2313 != 0.0) {
        let eq1_e1019_q: f64 = locals.var_q_nqs_a;let eq1_e1020: f64 = (locals.var_inqs0_a + locals.var_q_nqs_a);let eq1_e1020_d_n16: f64 = (locals.var_inqs0_a_dn16 + locals.var_q_nqs_a_dn16);let eq1_e1020_q: f64 = eq1_e1019_q;
        (eq1_e1020, locals.var_inqs0_a_dn0, locals.var_inqs0_a_dn2, locals.var_inqs0_a_dn4, locals.var_inqs0_a_dn5, locals.var_inqs0_a_dn6, locals.var_inqs0_a_dn7, locals.var_inqs0_a_dn8, locals.var_inqs0_a_dn9, locals.var_inqs0_a_dn10, locals.var_inqs0_a_dn11, locals.var_inqs0_a_dn14, eq1_e1020_d_n16, eq1_e1020_q, locals.var_q_nqs_a_dn16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq1_e1022_q_d_n16),
        );
        let (eq2_e1029, eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17, eq2_e1029_q, eq2_e1029_q_d_n17,) = {
    if (locals.var_guard2313 != 0.0) {
        let eq2_e1026_q: f64 = locals.var_q_nqs_k;let eq2_e1027: f64 = (locals.var_inqs0_k + locals.var_q_nqs_k);let eq2_e1027_d_n17: f64 = (locals.var_inqs0_k_dn17 + locals.var_q_nqs_k_dn17);let eq2_e1027_q: f64 = eq2_e1026_q;
        (eq2_e1027, locals.var_inqs0_k_dn0, locals.var_inqs0_k_dn2, locals.var_inqs0_k_dn4, locals.var_inqs0_k_dn5, locals.var_inqs0_k_dn6, locals.var_inqs0_k_dn7, locals.var_inqs0_k_dn8, locals.var_inqs0_k_dn9, locals.var_inqs0_k_dn10, locals.var_inqs0_k_dn11, locals.var_inqs0_k_dn14, eq2_e1027_d_n17, eq2_e1027_q, locals.var_q_nqs_k_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq2_e1029_q_d_n17),
        );
        let (eq5_e1046, eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18, eq5_e1046_q, eq5_e1046_q_d_n18,) = {
    if (locals.var_guard2314 != 0.0) {
        let eq5_e1043_q: f64 = locals.var_w_nqs_a;let eq5_e1044: f64 = (locals.var_iwnqs0_a + locals.var_w_nqs_a);let eq5_e1044_d_n18: f64 = (locals.var_iwnqs0_a_dn18 + locals.var_w_nqs_a_dn18);let eq5_e1044_q: f64 = eq5_e1043_q;
        (eq5_e1044, locals.var_iwnqs0_a_dn0, locals.var_iwnqs0_a_dn2, locals.var_iwnqs0_a_dn4, locals.var_iwnqs0_a_dn5, locals.var_iwnqs0_a_dn6, locals.var_iwnqs0_a_dn7, locals.var_iwnqs0_a_dn8, locals.var_iwnqs0_a_dn9, locals.var_iwnqs0_a_dn10, locals.var_iwnqs0_a_dn11, locals.var_iwnqs0_a_dn14, eq5_e1044_d_n18, eq5_e1044_q, locals.var_w_nqs_a_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq5_e1046_q_d_n18),
        );let eq15_e1092_q: f64 = locals.var_qbs;let eq15_e1093: f64 = (p.p87 * locals.var_qbs);let eq15_e1093_d_n0: f64 = (p.p87 * locals.var_qbs_dn0);let eq15_e1093_d_n2: f64 = (p.p87 * locals.var_qbs_dn2);let eq15_e1093_d_n4: f64 = (p.p87 * locals.var_qbs_dn4);let eq15_e1093_d_n5: f64 = (p.p87 * locals.var_qbs_dn5);let eq15_e1093_d_n6: f64 = (p.p87 * locals.var_qbs_dn6);let eq15_e1093_d_n7: f64 = (p.p87 * locals.var_qbs_dn7);let eq15_e1093_d_n8: f64 = (p.p87 * locals.var_qbs_dn8);let eq15_e1093_d_n9: f64 = (p.p87 * locals.var_qbs_dn9);let eq15_e1093_d_n10: f64 = (p.p87 * locals.var_qbs_dn10);let eq15_e1093_d_n11: f64 = (p.p87 * locals.var_qbs_dn11);let eq15_e1093_d_n14: f64 = (p.p87 * locals.var_qbs_dn14);let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);let eq15_reactive_node_derivatives: [f64; 19] = [eq15_e1093_d_n0, 0.0, eq15_e1093_d_n2, 0.0, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, 0.0, 0.0, eq15_e1093_d_n14, 0.0, 0.0, 0.0, 0.0];let eq15_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[2]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );let eq16_e1096_q: f64 = locals.var_qbd;let eq16_e1097: f64 = (p.p87 * locals.var_qbd);let eq16_e1097_d_n0: f64 = (p.p87 * locals.var_qbd_dn0);let eq16_e1097_d_n2: f64 = (p.p87 * locals.var_qbd_dn2);let eq16_e1097_d_n4: f64 = (p.p87 * locals.var_qbd_dn4);let eq16_e1097_d_n5: f64 = (p.p87 * locals.var_qbd_dn5);let eq16_e1097_d_n6: f64 = (p.p87 * locals.var_qbd_dn6);let eq16_e1097_d_n7: f64 = (p.p87 * locals.var_qbd_dn7);let eq16_e1097_d_n8: f64 = (p.p87 * locals.var_qbd_dn8);let eq16_e1097_d_n9: f64 = (p.p87 * locals.var_qbd_dn9);let eq16_e1097_d_n10: f64 = (p.p87 * locals.var_qbd_dn10);let eq16_e1097_d_n11: f64 = (p.p87 * locals.var_qbd_dn11);let eq16_e1097_d_n14: f64 = (p.p87 * locals.var_qbd_dn14);let eq16_e1097_d_n16: f64 = (p.p87 * locals.var_qbd_dn16);let eq16_e1097_d_n17: f64 = (p.p87 * locals.var_qbd_dn17);let eq16_e1097_d_n18: f64 = (p.p87 * locals.var_qbd_dn18);let eq16_e1097_q: f64 = (p.p87 * eq16_e1096_q);let eq16_reactive_node_derivatives: [f64; 19] = [eq16_e1097_d_n0, 0.0, eq16_e1097_d_n2, 0.0, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, 0.0, 0.0, eq16_e1097_d_n14, 0.0, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_n18];let eq16_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1116, eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14, eq19_e1116_q,) = {
    if (locals.var_guard2413 != 0.0) {
        let eq19_e1113_q: f64 = locals.var_qbsi;let eq19_e1114: f64 = (p.p87 * locals.var_qbsi);let eq19_e1114_d_n0: f64 = (p.p87 * locals.var_qbsi_dn0);let eq19_e1114_d_n2: f64 = (p.p87 * locals.var_qbsi_dn2);let eq19_e1114_d_n4: f64 = (p.p87 * locals.var_qbsi_dn4);let eq19_e1114_d_n5: f64 = (p.p87 * locals.var_qbsi_dn5);let eq19_e1114_d_n6: f64 = (p.p87 * locals.var_qbsi_dn6);let eq19_e1114_d_n7: f64 = (p.p87 * locals.var_qbsi_dn7);let eq19_e1114_d_n8: f64 = (p.p87 * locals.var_qbsi_dn8);let eq19_e1114_d_n9: f64 = (p.p87 * locals.var_qbsi_dn9);let eq19_e1114_d_n10: f64 = (p.p87 * locals.var_qbsi_dn10);let eq19_e1114_d_n11: f64 = (p.p87 * locals.var_qbsi_dn11);let eq19_e1114_d_n14: f64 = (p.p87 * locals.var_qbsi_dn14);let eq19_e1114_q: f64 = (p.p87 * eq19_e1113_q);
        (eq19_e1114, eq19_e1114_d_n0, eq19_e1114_d_n2, eq19_e1114_d_n4, eq19_e1114_d_n5, eq19_e1114_d_n6, eq19_e1114_d_n7, eq19_e1114_d_n8, eq19_e1114_d_n9, eq19_e1114_d_n10, eq19_e1114_d_n11, eq19_e1114_d_n14, eq19_e1114_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e1116_d_n0, 0.0, eq19_e1116_d_n2, 0.0, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, 0.0, 0.0, eq19_e1116_d_n14, 0.0, 0.0, 0.0, 0.0];let eq19_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14, eq20_e1123_q,) = {
    if (locals.var_guard2413 != 0.0) {
        let eq20_e1120_q: f64 = locals.var_qbdi;let eq20_e1121: f64 = (p.p87 * locals.var_qbdi);let eq20_e1121_d_n0: f64 = (p.p87 * locals.var_qbdi_dn0);let eq20_e1121_d_n2: f64 = (p.p87 * locals.var_qbdi_dn2);let eq20_e1121_d_n4: f64 = (p.p87 * locals.var_qbdi_dn4);let eq20_e1121_d_n5: f64 = (p.p87 * locals.var_qbdi_dn5);let eq20_e1121_d_n6: f64 = (p.p87 * locals.var_qbdi_dn6);let eq20_e1121_d_n7: f64 = (p.p87 * locals.var_qbdi_dn7);let eq20_e1121_d_n8: f64 = (p.p87 * locals.var_qbdi_dn8);let eq20_e1121_d_n9: f64 = (p.p87 * locals.var_qbdi_dn9);let eq20_e1121_d_n10: f64 = (p.p87 * locals.var_qbdi_dn10);let eq20_e1121_d_n11: f64 = (p.p87 * locals.var_qbdi_dn11);let eq20_e1121_d_n14: f64 = (p.p87 * locals.var_qbdi_dn14);let eq20_e1121_q: f64 = (p.p87 * eq20_e1120_q);
        (eq20_e1121, eq20_e1121_d_n0, eq20_e1121_d_n2, eq20_e1121_d_n4, eq20_e1121_d_n5, eq20_e1121_d_n6, eq20_e1121_d_n7, eq20_e1121_d_n8, eq20_e1121_d_n9, eq20_e1121_d_n10, eq20_e1121_d_n11, eq20_e1121_d_n14, eq20_e1121_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e1123_d_n0, 0.0, eq20_e1123_d_n2, 0.0, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, 0.0, 0.0, eq20_e1123_d_n14, 0.0, 0.0, 0.0, 0.0];let eq20_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );let eq28_e1167: f64 = (locals.var_qg + locals.var_qg_nqs);let eq28_e1168_q: f64 = eq28_e1167;let eq28_e1169: f64 = (p.p87 * eq28_e1167);let eq28_e1169_d_n0: f64 = (p.p87 * locals.var_qg_dn0);let eq28_e1169_d_n2: f64 = (p.p87 * locals.var_qg_dn2);let eq28_e1169_d_n4: f64 = (p.p87 * locals.var_qg_dn4);let eq28_e1169_d_n5: f64 = (p.p87 * locals.var_qg_dn5);let eq28_e1169_d_n6: f64 = (p.p87 * locals.var_qg_dn6);let eq28_e1169_d_n7: f64 = (p.p87 * locals.var_qg_dn7);let eq28_e1169_d_n8: f64 = (p.p87 * locals.var_qg_dn8);let eq28_e1169_d_n9: f64 = (p.p87 * locals.var_qg_dn9);let eq28_e1169_d_n10: f64 = (p.p87 * locals.var_qg_dn10);let eq28_e1169_d_n11: f64 = (p.p87 * locals.var_qg_dn11);let eq28_e1169_d_n12: f64 = (p.p87 * locals.var_qg_nqs_dn12);let eq28_e1169_d_n13: f64 = (p.p87 * locals.var_qg_nqs_dn13);let eq28_e1169_d_n14: f64 = (p.p87 * locals.var_qg_dn14);let eq28_e1169_q: f64 = (p.p87 * eq28_e1168_q);let eq28_reactive_node_derivatives: [f64; 19] = [eq28_e1169_d_n0, 0.0, eq28_e1169_d_n2, 0.0, eq28_e1169_d_n4, eq28_e1169_d_n5, eq28_e1169_d_n6, eq28_e1169_d_n7, eq28_e1169_d_n8, eq28_e1169_d_n9, eq28_e1169_d_n10, eq28_e1169_d_n11, eq28_e1169_d_n12, eq28_e1169_d_n13, eq28_e1169_d_n14, 0.0, 0.0, 0.0, 0.0];let eq28_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );let eq29_e1173: f64 = (locals.var_qd + locals.var_qd_nqs);let eq29_e1173_d_n0: f64 = (locals.var_qd_dn0 + locals.var_qd_nqs_dn0);let eq29_e1173_d_n2: f64 = (locals.var_qd_dn2 + locals.var_qd_nqs_dn2);let eq29_e1173_d_n4: f64 = (locals.var_qd_dn4 + locals.var_qd_nqs_dn4);let eq29_e1173_d_n5: f64 = (locals.var_qd_dn5 + locals.var_qd_nqs_dn5);let eq29_e1173_d_n6: f64 = (locals.var_qd_dn6 + locals.var_qd_nqs_dn6);let eq29_e1173_d_n7: f64 = (locals.var_qd_dn7 + locals.var_qd_nqs_dn7);let eq29_e1173_d_n8: f64 = (locals.var_qd_dn8 + locals.var_qd_nqs_dn8);let eq29_e1173_d_n9: f64 = (locals.var_qd_dn9 + locals.var_qd_nqs_dn9);let eq29_e1173_d_n10: f64 = (locals.var_qd_dn10 + locals.var_qd_nqs_dn10);let eq29_e1173_d_n11: f64 = (locals.var_qd_dn11 + locals.var_qd_nqs_dn11);let eq29_e1173_d_n14: f64 = (locals.var_qd_dn14 + locals.var_qd_nqs_dn14);let eq29_e1174_q: f64 = eq29_e1173;let eq29_e1175: f64 = (p.p87 * eq29_e1173);let eq29_e1175_d_n0: f64 = (p.p87 * eq29_e1173_d_n0);let eq29_e1175_d_n2: f64 = (p.p87 * eq29_e1173_d_n2);let eq29_e1175_d_n4: f64 = (p.p87 * eq29_e1173_d_n4);let eq29_e1175_d_n5: f64 = (p.p87 * eq29_e1173_d_n5);let eq29_e1175_d_n6: f64 = (p.p87 * eq29_e1173_d_n6);let eq29_e1175_d_n7: f64 = (p.p87 * eq29_e1173_d_n7);let eq29_e1175_d_n8: f64 = (p.p87 * eq29_e1173_d_n8);let eq29_e1175_d_n9: f64 = (p.p87 * eq29_e1173_d_n9);let eq29_e1175_d_n10: f64 = (p.p87 * eq29_e1173_d_n10);let eq29_e1175_d_n11: f64 = (p.p87 * eq29_e1173_d_n11);let eq29_e1175_d_n12: f64 = (p.p87 * locals.var_qd_nqs_dn12);let eq29_e1175_d_n14: f64 = (p.p87 * eq29_e1173_d_n14);let eq29_e1175_q: f64 = (p.p87 * eq29_e1174_q);let eq29_reactive_node_derivatives: [f64; 19] = [eq29_e1175_d_n0, 0.0, eq29_e1175_d_n2, 0.0, eq29_e1175_d_n4, eq29_e1175_d_n5, eq29_e1175_d_n6, eq29_e1175_d_n7, eq29_e1175_d_n8, eq29_e1175_d_n9, eq29_e1175_d_n10, eq29_e1175_d_n11, eq29_e1175_d_n12, 0.0, eq29_e1175_d_n14, 0.0, 0.0, 0.0, 0.0];let eq29_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );let eq30_e1180: f64 = (locals.var_qg_nqs + locals.var_qd_nqs);let eq30_e1180_d_n12: f64 = (locals.var_qg_nqs_dn12 + locals.var_qd_nqs_dn12);let eq30_e1182: f64 = (eq30_e1180 + locals.var_qs_nqs);let eq30_e1182_d_n0: f64 = (locals.var_qd_nqs_dn0 + locals.var_qs_nqs_dn0);let eq30_e1182_d_n2: f64 = (locals.var_qd_nqs_dn2 + locals.var_qs_nqs_dn2);let eq30_e1182_d_n4: f64 = (locals.var_qd_nqs_dn4 + locals.var_qs_nqs_dn4);let eq30_e1182_d_n5: f64 = (locals.var_qd_nqs_dn5 + locals.var_qs_nqs_dn5);let eq30_e1182_d_n6: f64 = (locals.var_qd_nqs_dn6 + locals.var_qs_nqs_dn6);let eq30_e1182_d_n7: f64 = (locals.var_qd_nqs_dn7 + locals.var_qs_nqs_dn7);let eq30_e1182_d_n8: f64 = (locals.var_qd_nqs_dn8 + locals.var_qs_nqs_dn8);let eq30_e1182_d_n9: f64 = (locals.var_qd_nqs_dn9 + locals.var_qs_nqs_dn9);let eq30_e1182_d_n10: f64 = (locals.var_qd_nqs_dn10 + locals.var_qs_nqs_dn10);let eq30_e1182_d_n11: f64 = (locals.var_qd_nqs_dn11 + locals.var_qs_nqs_dn11);let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + locals.var_qs_nqs_dn12);let eq30_e1182_d_n14: f64 = (locals.var_qd_nqs_dn14 + locals.var_qs_nqs_dn14);let eq30_e1183: f64 = (locals.var_qb - eq30_e1182);let eq30_e1183_d_n0: f64 = (locals.var_qb_dn0 - eq30_e1182_d_n0);let eq30_e1183_d_n2: f64 = (locals.var_qb_dn2 - eq30_e1182_d_n2);let eq30_e1183_d_n4: f64 = (locals.var_qb_dn4 - eq30_e1182_d_n4);let eq30_e1183_d_n5: f64 = (locals.var_qb_dn5 - eq30_e1182_d_n5);let eq30_e1183_d_n6: f64 = (locals.var_qb_dn6 - eq30_e1182_d_n6);let eq30_e1183_d_n7: f64 = (locals.var_qb_dn7 - eq30_e1182_d_n7);let eq30_e1183_d_n8: f64 = (locals.var_qb_dn8 - eq30_e1182_d_n8);let eq30_e1183_d_n9: f64 = (locals.var_qb_dn9 - eq30_e1182_d_n9);let eq30_e1183_d_n10: f64 = (locals.var_qb_dn10 - eq30_e1182_d_n10);let eq30_e1183_d_n11: f64 = (locals.var_qb_dn11 - eq30_e1182_d_n11);let eq30_e1183_d_n14: f64 = (locals.var_qb_dn14 - eq30_e1182_d_n14);let eq30_e1184_q: f64 = eq30_e1183;let eq30_e1185: f64 = (p.p87 * eq30_e1183);let eq30_e1185_d_n0: f64 = (p.p87 * eq30_e1183_d_n0);let eq30_e1185_d_n2: f64 = (p.p87 * eq30_e1183_d_n2);let eq30_e1185_d_n4: f64 = (p.p87 * eq30_e1183_d_n4);let eq30_e1185_d_n5: f64 = (p.p87 * eq30_e1183_d_n5);let eq30_e1185_d_n6: f64 = (p.p87 * eq30_e1183_d_n6);let eq30_e1185_d_n7: f64 = (p.p87 * eq30_e1183_d_n7);let eq30_e1185_d_n8: f64 = (p.p87 * eq30_e1183_d_n8);let eq30_e1185_d_n9: f64 = (p.p87 * eq30_e1183_d_n9);let eq30_e1185_d_n10: f64 = (p.p87 * eq30_e1183_d_n10);let eq30_e1185_d_n11: f64 = (p.p87 * eq30_e1183_d_n11);let eq30_e1185_d_n12: f64 = (p.p87 * (-eq30_e1182_d_n12));let eq30_e1185_d_n13: f64 = (p.p87 * (-locals.var_qg_nqs_dn13));let eq30_e1185_d_n14: f64 = (p.p87 * eq30_e1183_d_n14);let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e1185_d_n0, 0.0, eq30_e1185_d_n2, 0.0, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, 0.0, 0.0, 0.0, 0.0];let eq30_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );let eq31_e1188_q: f64 = locals.var_qgext;let eq31_e1189: f64 = (p.p87 * locals.var_qgext);let eq31_e1189_d_n0: f64 = (p.p87 * locals.var_qgext_dn0);let eq31_e1189_d_n2: f64 = (p.p87 * locals.var_qgext_dn2);let eq31_e1189_d_n4: f64 = (p.p87 * locals.var_qgext_dn4);let eq31_e1189_d_n5: f64 = (p.p87 * locals.var_qgext_dn5);let eq31_e1189_d_n6: f64 = (p.p87 * locals.var_qgext_dn6);let eq31_e1189_d_n7: f64 = (p.p87 * locals.var_qgext_dn7);let eq31_e1189_d_n8: f64 = (p.p87 * locals.var_qgext_dn8);let eq31_e1189_d_n9: f64 = (p.p87 * locals.var_qgext_dn9);let eq31_e1189_d_n10: f64 = (p.p87 * locals.var_qgext_dn10);let eq31_e1189_d_n11: f64 = (p.p87 * locals.var_qgext_dn11);let eq31_e1189_d_n14: f64 = (p.p87 * locals.var_qgext_dn14);let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e1189_d_n0, 0.0, eq31_e1189_d_n2, 0.0, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, 0.0, 0.0, eq31_e1189_d_n14, 0.0, 0.0, 0.0, 0.0];let eq31_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );let eq32_e1192_q: f64 = locals.var_qdext;let eq32_e1193: f64 = (p.p87 * locals.var_qdext);let eq32_e1193_d_n0: f64 = (p.p87 * locals.var_qdext_dn0);let eq32_e1193_d_n2: f64 = (p.p87 * locals.var_qdext_dn2);let eq32_e1193_d_n4: f64 = (p.p87 * locals.var_qdext_dn4);let eq32_e1193_d_n5: f64 = (p.p87 * locals.var_qdext_dn5);let eq32_e1193_d_n6: f64 = (p.p87 * locals.var_qdext_dn6);let eq32_e1193_d_n7: f64 = (p.p87 * locals.var_qdext_dn7);let eq32_e1193_d_n8: f64 = (p.p87 * locals.var_qdext_dn8);let eq32_e1193_d_n9: f64 = (p.p87 * locals.var_qdext_dn9);let eq32_e1193_d_n10: f64 = (p.p87 * locals.var_qdext_dn10);let eq32_e1193_d_n11: f64 = (p.p87 * locals.var_qdext_dn11);let eq32_e1193_d_n14: f64 = (p.p87 * locals.var_qdext_dn14);let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);let eq32_reactive_node_derivatives: [f64; 19] = [eq32_e1193_d_n0, 0.0, eq32_e1193_d_n2, 0.0, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, 0.0, 0.0, eq32_e1193_d_n14, 0.0, 0.0, 0.0, 0.0];let eq32_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );let eq33_e1196_q: f64 = locals.var_qbext;let eq33_e1197: f64 = (p.p87 * locals.var_qbext);let eq33_e1197_d_n0: f64 = (p.p87 * locals.var_qbext_dn0);let eq33_e1197_d_n2: f64 = (p.p87 * locals.var_qbext_dn2);let eq33_e1197_d_n4: f64 = (p.p87 * locals.var_qbext_dn4);let eq33_e1197_d_n5: f64 = (p.p87 * locals.var_qbext_dn5);let eq33_e1197_d_n6: f64 = (p.p87 * locals.var_qbext_dn6);let eq33_e1197_d_n7: f64 = (p.p87 * locals.var_qbext_dn7);let eq33_e1197_d_n8: f64 = (p.p87 * locals.var_qbext_dn8);let eq33_e1197_d_n9: f64 = (p.p87 * locals.var_qbext_dn9);let eq33_e1197_d_n10: f64 = (p.p87 * locals.var_qbext_dn10);let eq33_e1197_d_n11: f64 = (p.p87 * locals.var_qbext_dn11);let eq33_e1197_d_n14: f64 = (p.p87 * locals.var_qbext_dn14);let eq33_e1197_q: f64 = (p.p87 * eq33_e1196_q);let eq33_reactive_node_derivatives: [f64; 19] = [eq33_e1197_d_n0, 0.0, eq33_e1197_d_n2, 0.0, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, 0.0, 0.0, eq33_e1197_d_n14, 0.0, 0.0, 0.0, 0.0];let eq33_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );let eq34_e1199: f64 = (-p.p87);let eq34_e1201_q: f64 = locals.var_qfd;let eq34_e1202: f64 = (eq34_e1199 * locals.var_qfd);let eq34_e1202_d_n0: f64 = (eq34_e1199 * locals.var_qfd_dn0);let eq34_e1202_d_n2: f64 = (eq34_e1199 * locals.var_qfd_dn2);let eq34_e1202_d_n7: f64 = (eq34_e1199 * locals.var_qfd_dn7);let eq34_e1202_q: f64 = (eq34_e1199 * eq34_e1201_q);
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq34_e1202_d_n0),
            nodes[2],
            multiplicity * (eq34_e1202_d_n2),
            nodes[7],
            multiplicity * (eq34_e1202_d_n7),
        );let eq35_e1204: f64 = (-p.p87);let eq35_e1206_q: f64 = locals.var_qfs;let eq35_e1207: f64 = (eq35_e1204 * locals.var_qfs);let eq35_e1207_d_n2: f64 = (eq35_e1204 * locals.var_qfs_dn2);let eq35_e1207_d_n7: f64 = (eq35_e1204 * locals.var_qfs_dn7);let eq35_e1207_q: f64 = (eq35_e1204 * eq35_e1206_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes[2],
            multiplicity * (eq35_e1207_d_n2),
            nodes[7],
            multiplicity * (eq35_e1207_d_n7),
        );let eq41_e1236: f64 = ((nv15 - 0.0) * locals.var_sigrat_s);let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn0);let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn2);let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn4);let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn5);let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn6);let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn7);let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn8);let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn9);let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn10);let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn11);let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_s_dn14);let eq41_e1237_q: f64 = eq41_e1236;let eq41_reactive_node_derivatives: [f64; 19] = [eq41_e1236_d_n0, 0.0, eq41_e1236_d_n2, 0.0, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, 0.0, 0.0, eq41_e1236_d_n14, locals.var_sigrat_s, 0.0, 0.0, 0.0];let eq41_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );let eq42_e1240: f64 = ((nv15 - 0.0) * locals.var_sigrat_d);let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn0);let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn2);let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn4);let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn5);let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn6);let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn7);let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn8);let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn9);let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn10);let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn11);let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * locals.var_sigrat_d_dn14);let eq42_e1241_q: f64 = eq42_e1240;let eq42_reactive_node_derivatives: [f64; 19] = [eq42_e1240_d_n0, 0.0, eq42_e1240_d_n2, 0.0, eq42_e1240_d_n4, eq42_e1240_d_n5, eq42_e1240_d_n6, eq42_e1240_d_n7, eq42_e1240_d_n8, eq42_e1240_d_n9, eq42_e1240_d_n10, eq42_e1240_d_n11, 0.0, 0.0, eq42_e1240_d_n14, locals.var_sigrat_d, 0.0, 0.0, 0.0];let eq42_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n12, eq62_e1362_q,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (locals.var_cqi * (nv12 - 0.0));let eq62_e1360_q: f64 = eq62_e1359;
        (eq62_e1359, locals.var_cqi, eq62_e1360_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq62_e1362_d_n12),
        );
        let (eq63_e1369, eq63_e1369_d_n13, eq63_e1369_q,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (locals.var_cqb * (nv13 - 0.0));let eq63_e1367_q: f64 = eq63_e1366;
        (eq63_e1366, locals.var_cqb, eq63_e1367_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq63_e1369_d_n13),
        );
        let (eq67_e1388, eq67_e1388_d_n14, eq67_e1388_q,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386_q: f64 = (nv14 - 0.0);
        ((nv14 - 0.0), 1.0, eq67_e1386_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[14]),
            None,
            nodes[14],
            multiplicity * (eq67_e1388_d_n14),
        );
    }
}
