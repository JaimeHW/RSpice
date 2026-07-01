#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_144(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat_d: f64,
        var_btatpartgat_d: f64,
        var_csrhgatd_i: f64,
        var_ctatgatd_i: f64,
        var_fstopsti_d: f64,
        var_ftdgat_d: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1696: f64,
        var_guard1710: f64,
        var_ibbt__blk1584: f64,
        var_ibbt__blk1584_dn11: f64,
        var_ibbt__blk1584_dn12: f64,
        var_ibbt__blk1584_dn6: f64,
        var_ibbt__blk1584_dn7: f64,
        var_ibbt__blk1584_dn8: f64,
        var_ibbt__blk1584_dn9: f64,
        var_idmult__blk1551: f64,
        var_idmult__blk1551_dn11: f64,
        var_idmult__blk1551_dn12: f64,
        var_idmult__blk1551_dn7: f64,
        var_idmult__blk1551_dn8: f64,
        var_idsatgat_d: f64,
        var_lgdrain_i: f64,
        var_one_minus_pgat_d: f64,
        var_one_minus_psti_d: f64,
        var_pbrstid_i: f64,
        var_pgatd_i: f64,
        var_qpref2sti_d: f64,
        var_qprefsti_d: f64,
        var_slopesti_d: f64,
        var_two_psistar__blk1555: f64,
        var_two_psistar__blk1555_dn11: f64,
        var_two_psistar__blk1555_dn12: f64,
        var_two_psistar__blk1555_dn7: f64,
        var_two_psistar__blk1555_dn8: f64,
        var_vav__blk1559: f64,
        var_vav__blk1559_dn11: f64,
        var_vav__blk1559_dn12: f64,
        var_vav__blk1559_dn7: f64,
        var_vav__blk1559_dn8: f64,
        var_vbigat_d: f64,
        var_vbiinvsti_d: f64,
        var_vbirgatinv_d: f64,
        var_vbrinvsti_d: f64,
        var_vbrstid_i: f64,
        var_vj__blk1552: f64,
        var_vj__blk1552_dn11: f64,
        var_vj__blk1552_dn12: f64,
        var_vj__blk1552_dn7: f64,
        var_vj__blk1552_dn8: f64,
        var_vjsrh__blk1557: f64,
        var_vjsrh__blk1557_dn11: f64,
        var_vjsrh__blk1557_dn12: f64,
        var_vjsrh__blk1557_dn7: f64,
        var_vjsrh__blk1557_dn8: f64,
        var_vjun_d: f64,
        var_vjun_d_dn12: f64,
        var_vjun_d_dn8: f64,
        var_wdepnulrgat_d: f64,
        var_zinv__blk1554: f64,
        var_zinv__blk1554_dn11: f64,
        var_zinv__blk1554_dn12: f64,
        var_zinv__blk1554_dn7: f64,
        var_zinv__blk1554_dn8: f64,
        var_asrh__blk1568_slot: &mut f64,
        var_asrh__blk1568_dn11_slot: &mut f64,
        var_asrh__blk1568_dn12_slot: &mut f64,
        var_asrh__blk1568_dn6_slot: &mut f64,
        var_asrh__blk1568_dn7_slot: &mut f64,
        var_asrh__blk1568_dn8_slot: &mut f64,
        var_asrh__blk1568_dn9_slot: &mut f64,
        var_btat__blk1570_slot: &mut f64,
        var_btat__blk1570_dn11_slot: &mut f64,
        var_btat__blk1570_dn12_slot: &mut f64,
        var_btat__blk1570_dn6_slot: &mut f64,
        var_btat__blk1570_dn7_slot: &mut f64,
        var_btat__blk1570_dn8_slot: &mut f64,
        var_btat__blk1570_dn9_slot: &mut f64,
        var_dwsrh__blk1565_slot: &mut f64,
        var_dwsrh__blk1565_dn11_slot: &mut f64,
        var_dwsrh__blk1565_dn12_slot: &mut f64,
        var_dwsrh__blk1565_dn7_slot: &mut f64,
        var_dwsrh__blk1565_dn8_slot: &mut f64,
        var_fbreakdown__blk1586_slot: &mut f64,
        var_fbreakdown__blk1586_dn11_slot: &mut f64,
        var_fbreakdown__blk1586_dn12_slot: &mut f64,
        var_fbreakdown__blk1586_dn6_slot: &mut f64,
        var_fbreakdown__blk1586_dn7_slot: &mut f64,
        var_fbreakdown__blk1586_dn8_slot: &mut f64,
        var_fbreakdown__blk1586_dn9_slot: &mut f64,
        var_guard1711_slot: &mut f64,
        var_guard1712_slot: &mut f64,
        var_guard1713_slot: &mut f64,
        var_guard1714_slot: &mut f64,
        var_guard1715_slot: &mut f64,
        var_guard1716_slot: &mut f64,
        var_guard1717_slot: &mut f64,
        var_guard1718_slot: &mut f64,
        var_id__blk1561_slot: &mut f64,
        var_id__blk1561_dn11_slot: &mut f64,
        var_id__blk1561_dn12_slot: &mut f64,
        var_id__blk1561_dn7_slot: &mut f64,
        var_id__blk1561_dn8_slot: &mut f64,
        var_ijungat_d_slot: &mut f64,
        var_ijungat_d_dn11_slot: &mut f64,
        var_ijungat_d_dn12_slot: &mut f64,
        var_ijungat_d_dn6_slot: &mut f64,
        var_ijungat_d_dn7_slot: &mut f64,
        var_ijungat_d_dn8_slot: &mut f64,
        var_ijungat_d_dn9_slot: &mut f64,
        var_ijunsti_d_slot: &mut f64,
        var_ijunsti_d_dn11_slot: &mut f64,
        var_ijunsti_d_dn12_slot: &mut f64,
        var_ijunsti_d_dn6_slot: &mut f64,
        var_ijunsti_d_dn7_slot: &mut f64,
        var_ijunsti_d_dn8_slot: &mut f64,
        var_ijunsti_d_dn9_slot: &mut f64,
        var_isrh__blk1562_slot: &mut f64,
        var_isrh__blk1562_dn11_slot: &mut f64,
        var_isrh__blk1562_dn12_slot: &mut f64,
        var_isrh__blk1562_dn6_slot: &mut f64,
        var_isrh__blk1562_dn7_slot: &mut f64,
        var_isrh__blk1562_dn8_slot: &mut f64,
        var_isrh__blk1562_dn9_slot: &mut f64,
        var_itat__blk1569_slot: &mut f64,
        var_itat__blk1569_dn11_slot: &mut f64,
        var_itat__blk1569_dn12_slot: &mut f64,
        var_itat__blk1569_dn6_slot: &mut f64,
        var_itat__blk1569_dn7_slot: &mut f64,
        var_itat__blk1569_dn8_slot: &mut f64,
        var_itat__blk1569_dn9_slot: &mut f64,
        var_qjungat_d_slot: &mut f64,
        var_qjungat_d_dn11_slot: &mut f64,
        var_qjungat_d_dn12_slot: &mut f64,
        var_qjungat_d_dn6_slot: &mut f64,
        var_qjungat_d_dn7_slot: &mut f64,
        var_qjungat_d_dn8_slot: &mut f64,
        var_qjungat_d_dn9_slot: &mut f64,
        var_qjunsti_d_slot: &mut f64,
        var_qjunsti_d_dn11_slot: &mut f64,
        var_qjunsti_d_dn12_slot: &mut f64,
        var_qjunsti_d_dn6_slot: &mut f64,
        var_qjunsti_d_dn7_slot: &mut f64,
        var_qjunsti_d_dn8_slot: &mut f64,
        var_qjunsti_d_dn9_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn11_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn12_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn6_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn7_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn8_slot: &mut f64,
        var_twoatatoverthreebtat__blk1571_dn9_slot: &mut f64,
        var_umax__blk1573_slot: &mut f64,
        var_umax__blk1573_dn11_slot: &mut f64,
        var_umax__blk1573_dn12_slot: &mut f64,
        var_umax__blk1573_dn6_slot: &mut f64,
        var_umax__blk1573_dn7_slot: &mut f64,
        var_umax__blk1573_dn8_slot: &mut f64,
        var_umax__blk1573_dn9_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn11_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn12_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn6_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn7_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn8_slot: &mut f64,
        var_umaxbeforelimiting__blk1572_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn11_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn12_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn7_slot: &mut f64,
        var_vbi_minus_vjsrh__blk1563_dn8_slot: &mut f64,
        var_wdep__blk1567_slot: &mut f64,
        var_wdep__blk1567_dn11_slot: &mut f64,
        var_wdep__blk1567_dn12_slot: &mut f64,
        var_wdep__blk1567_dn6_slot: &mut f64,
        var_wdep__blk1567_dn7_slot: &mut f64,
        var_wdep__blk1567_dn8_slot: &mut f64,
        var_wdep__blk1567_dn9_slot: &mut f64,
        var_wsrh__blk1566_slot: &mut f64,
        var_wsrh__blk1566_dn11_slot: &mut f64,
        var_wsrh__blk1566_dn12_slot: &mut f64,
        var_wsrh__blk1566_dn7_slot: &mut f64,
        var_wsrh__blk1566_dn8_slot: &mut f64,
        var_wsrhstep__blk1564_slot: &mut f64,
        var_wsrhstep__blk1564_dn11_slot: &mut f64,
        var_wsrhstep__blk1564_dn12_slot: &mut f64,
        var_wsrhstep__blk1564_dn7_slot: &mut f64,
        var_wsrhstep__blk1564_dn8_slot: &mut f64,
    ) {
        let mut var_asrh__blk1568: f64 = *var_asrh__blk1568_slot;
        let mut var_asrh__blk1568_dn11: f64 = *var_asrh__blk1568_dn11_slot;
        let mut var_asrh__blk1568_dn12: f64 = *var_asrh__blk1568_dn12_slot;
        let mut var_asrh__blk1568_dn6: f64 = *var_asrh__blk1568_dn6_slot;
        let mut var_asrh__blk1568_dn7: f64 = *var_asrh__blk1568_dn7_slot;
        let mut var_asrh__blk1568_dn8: f64 = *var_asrh__blk1568_dn8_slot;
        let mut var_asrh__blk1568_dn9: f64 = *var_asrh__blk1568_dn9_slot;
        let mut var_btat__blk1570: f64 = *var_btat__blk1570_slot;
        let mut var_btat__blk1570_dn11: f64 = *var_btat__blk1570_dn11_slot;
        let mut var_btat__blk1570_dn12: f64 = *var_btat__blk1570_dn12_slot;
        let mut var_btat__blk1570_dn6: f64 = *var_btat__blk1570_dn6_slot;
        let mut var_btat__blk1570_dn7: f64 = *var_btat__blk1570_dn7_slot;
        let mut var_btat__blk1570_dn8: f64 = *var_btat__blk1570_dn8_slot;
        let mut var_btat__blk1570_dn9: f64 = *var_btat__blk1570_dn9_slot;
        let mut var_dwsrh__blk1565: f64 = *var_dwsrh__blk1565_slot;
        let mut var_dwsrh__blk1565_dn11: f64 = *var_dwsrh__blk1565_dn11_slot;
        let mut var_dwsrh__blk1565_dn12: f64 = *var_dwsrh__blk1565_dn12_slot;
        let mut var_dwsrh__blk1565_dn7: f64 = *var_dwsrh__blk1565_dn7_slot;
        let mut var_dwsrh__blk1565_dn8: f64 = *var_dwsrh__blk1565_dn8_slot;
        let mut var_fbreakdown__blk1586: f64 = *var_fbreakdown__blk1586_slot;
        let mut var_fbreakdown__blk1586_dn11: f64 = *var_fbreakdown__blk1586_dn11_slot;
        let mut var_fbreakdown__blk1586_dn12: f64 = *var_fbreakdown__blk1586_dn12_slot;
        let mut var_fbreakdown__blk1586_dn6: f64 = *var_fbreakdown__blk1586_dn6_slot;
        let mut var_fbreakdown__blk1586_dn7: f64 = *var_fbreakdown__blk1586_dn7_slot;
        let mut var_fbreakdown__blk1586_dn8: f64 = *var_fbreakdown__blk1586_dn8_slot;
        let mut var_fbreakdown__blk1586_dn9: f64 = *var_fbreakdown__blk1586_dn9_slot;
        let mut var_guard1711: f64 = *var_guard1711_slot;
        let mut var_guard1712: f64 = *var_guard1712_slot;
        let mut var_guard1713: f64 = *var_guard1713_slot;
        let mut var_guard1714: f64 = *var_guard1714_slot;
        let mut var_guard1715: f64 = *var_guard1715_slot;
        let mut var_guard1716: f64 = *var_guard1716_slot;
        let mut var_guard1717: f64 = *var_guard1717_slot;
        let mut var_guard1718: f64 = *var_guard1718_slot;
        let mut var_id__blk1561: f64 = *var_id__blk1561_slot;
        let mut var_id__blk1561_dn11: f64 = *var_id__blk1561_dn11_slot;
        let mut var_id__blk1561_dn12: f64 = *var_id__blk1561_dn12_slot;
        let mut var_id__blk1561_dn7: f64 = *var_id__blk1561_dn7_slot;
        let mut var_id__blk1561_dn8: f64 = *var_id__blk1561_dn8_slot;
        let mut var_ijungat_d: f64 = *var_ijungat_d_slot;
        let mut var_ijungat_d_dn11: f64 = *var_ijungat_d_dn11_slot;
        let mut var_ijungat_d_dn12: f64 = *var_ijungat_d_dn12_slot;
        let mut var_ijungat_d_dn6: f64 = *var_ijungat_d_dn6_slot;
        let mut var_ijungat_d_dn7: f64 = *var_ijungat_d_dn7_slot;
        let mut var_ijungat_d_dn8: f64 = *var_ijungat_d_dn8_slot;
        let mut var_ijungat_d_dn9: f64 = *var_ijungat_d_dn9_slot;
        let mut var_ijunsti_d: f64 = *var_ijunsti_d_slot;
        let mut var_ijunsti_d_dn11: f64 = *var_ijunsti_d_dn11_slot;
        let mut var_ijunsti_d_dn12: f64 = *var_ijunsti_d_dn12_slot;
        let mut var_ijunsti_d_dn6: f64 = *var_ijunsti_d_dn6_slot;
        let mut var_ijunsti_d_dn7: f64 = *var_ijunsti_d_dn7_slot;
        let mut var_ijunsti_d_dn8: f64 = *var_ijunsti_d_dn8_slot;
        let mut var_ijunsti_d_dn9: f64 = *var_ijunsti_d_dn9_slot;
        let mut var_isrh__blk1562: f64 = *var_isrh__blk1562_slot;
        let mut var_isrh__blk1562_dn11: f64 = *var_isrh__blk1562_dn11_slot;
        let mut var_isrh__blk1562_dn12: f64 = *var_isrh__blk1562_dn12_slot;
        let mut var_isrh__blk1562_dn6: f64 = *var_isrh__blk1562_dn6_slot;
        let mut var_isrh__blk1562_dn7: f64 = *var_isrh__blk1562_dn7_slot;
        let mut var_isrh__blk1562_dn8: f64 = *var_isrh__blk1562_dn8_slot;
        let mut var_isrh__blk1562_dn9: f64 = *var_isrh__blk1562_dn9_slot;
        let mut var_itat__blk1569: f64 = *var_itat__blk1569_slot;
        let mut var_itat__blk1569_dn11: f64 = *var_itat__blk1569_dn11_slot;
        let mut var_itat__blk1569_dn12: f64 = *var_itat__blk1569_dn12_slot;
        let mut var_itat__blk1569_dn6: f64 = *var_itat__blk1569_dn6_slot;
        let mut var_itat__blk1569_dn7: f64 = *var_itat__blk1569_dn7_slot;
        let mut var_itat__blk1569_dn8: f64 = *var_itat__blk1569_dn8_slot;
        let mut var_itat__blk1569_dn9: f64 = *var_itat__blk1569_dn9_slot;
        let mut var_qjungat_d: f64 = *var_qjungat_d_slot;
        let mut var_qjungat_d_dn11: f64 = *var_qjungat_d_dn11_slot;
        let mut var_qjungat_d_dn12: f64 = *var_qjungat_d_dn12_slot;
        let mut var_qjungat_d_dn6: f64 = *var_qjungat_d_dn6_slot;
        let mut var_qjungat_d_dn7: f64 = *var_qjungat_d_dn7_slot;
        let mut var_qjungat_d_dn8: f64 = *var_qjungat_d_dn8_slot;
        let mut var_qjungat_d_dn9: f64 = *var_qjungat_d_dn9_slot;
        let mut var_qjunsti_d: f64 = *var_qjunsti_d_slot;
        let mut var_qjunsti_d_dn11: f64 = *var_qjunsti_d_dn11_slot;
        let mut var_qjunsti_d_dn12: f64 = *var_qjunsti_d_dn12_slot;
        let mut var_qjunsti_d_dn6: f64 = *var_qjunsti_d_dn6_slot;
        let mut var_qjunsti_d_dn7: f64 = *var_qjunsti_d_dn7_slot;
        let mut var_qjunsti_d_dn8: f64 = *var_qjunsti_d_dn8_slot;
        let mut var_qjunsti_d_dn9: f64 = *var_qjunsti_d_dn9_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_twoatatoverthreebtat__blk1571: f64 = *var_twoatatoverthreebtat__blk1571_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn11: f64 = *var_twoatatoverthreebtat__blk1571_dn11_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn12: f64 = *var_twoatatoverthreebtat__blk1571_dn12_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn6: f64 = *var_twoatatoverthreebtat__blk1571_dn6_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn7: f64 = *var_twoatatoverthreebtat__blk1571_dn7_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn8: f64 = *var_twoatatoverthreebtat__blk1571_dn8_slot;
        let mut var_twoatatoverthreebtat__blk1571_dn9: f64 = *var_twoatatoverthreebtat__blk1571_dn9_slot;
        let mut var_umax__blk1573: f64 = *var_umax__blk1573_slot;
        let mut var_umax__blk1573_dn11: f64 = *var_umax__blk1573_dn11_slot;
        let mut var_umax__blk1573_dn12: f64 = *var_umax__blk1573_dn12_slot;
        let mut var_umax__blk1573_dn6: f64 = *var_umax__blk1573_dn6_slot;
        let mut var_umax__blk1573_dn7: f64 = *var_umax__blk1573_dn7_slot;
        let mut var_umax__blk1573_dn8: f64 = *var_umax__blk1573_dn8_slot;
        let mut var_umax__blk1573_dn9: f64 = *var_umax__blk1573_dn9_slot;
        let mut var_umaxbeforelimiting__blk1572: f64 = *var_umaxbeforelimiting__blk1572_slot;
        let mut var_umaxbeforelimiting__blk1572_dn11: f64 = *var_umaxbeforelimiting__blk1572_dn11_slot;
        let mut var_umaxbeforelimiting__blk1572_dn12: f64 = *var_umaxbeforelimiting__blk1572_dn12_slot;
        let mut var_umaxbeforelimiting__blk1572_dn6: f64 = *var_umaxbeforelimiting__blk1572_dn6_slot;
        let mut var_umaxbeforelimiting__blk1572_dn7: f64 = *var_umaxbeforelimiting__blk1572_dn7_slot;
        let mut var_umaxbeforelimiting__blk1572_dn8: f64 = *var_umaxbeforelimiting__blk1572_dn8_slot;
        let mut var_umaxbeforelimiting__blk1572_dn9: f64 = *var_umaxbeforelimiting__blk1572_dn9_slot;
        let mut var_vbi_minus_vjsrh__blk1563: f64 = *var_vbi_minus_vjsrh__blk1563_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn11: f64 = *var_vbi_minus_vjsrh__blk1563_dn11_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn12: f64 = *var_vbi_minus_vjsrh__blk1563_dn12_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn7: f64 = *var_vbi_minus_vjsrh__blk1563_dn7_slot;
        let mut var_vbi_minus_vjsrh__blk1563_dn8: f64 = *var_vbi_minus_vjsrh__blk1563_dn8_slot;
        let mut var_wdep__blk1567: f64 = *var_wdep__blk1567_slot;
        let mut var_wdep__blk1567_dn11: f64 = *var_wdep__blk1567_dn11_slot;
        let mut var_wdep__blk1567_dn12: f64 = *var_wdep__blk1567_dn12_slot;
        let mut var_wdep__blk1567_dn6: f64 = *var_wdep__blk1567_dn6_slot;
        let mut var_wdep__blk1567_dn7: f64 = *var_wdep__blk1567_dn7_slot;
        let mut var_wdep__blk1567_dn8: f64 = *var_wdep__blk1567_dn8_slot;
        let mut var_wdep__blk1567_dn9: f64 = *var_wdep__blk1567_dn9_slot;
        let mut var_wsrh__blk1566: f64 = *var_wsrh__blk1566_slot;
        let mut var_wsrh__blk1566_dn11: f64 = *var_wsrh__blk1566_dn11_slot;
        let mut var_wsrh__blk1566_dn12: f64 = *var_wsrh__blk1566_dn12_slot;
        let mut var_wsrh__blk1566_dn7: f64 = *var_wsrh__blk1566_dn7_slot;
        let mut var_wsrh__blk1566_dn8: f64 = *var_wsrh__blk1566_dn8_slot;
        let mut var_wsrhstep__blk1564: f64 = *var_wsrhstep__blk1564_slot;
        let mut var_wsrhstep__blk1564_dn11: f64 = *var_wsrhstep__blk1564_dn11_slot;
        let mut var_wsrhstep__blk1564_dn12: f64 = *var_wsrhstep__blk1564_dn12_slot;
        let mut var_wsrhstep__blk1564_dn7: f64 = *var_wsrhstep__blk1564_dn7_slot;
        let mut var_wsrhstep__blk1564_dn8: f64 = *var_wsrhstep__blk1564_dn8_slot;

        let (assign60860_e78783, assign60860_e78783_d_n6, assign60860_e78783_d_n7, assign60860_e78783_d_n8, assign60860_e78783_d_n9, assign60860_e78783_d_n11, assign60860_e78783_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1710 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown__blk1586, var_fbreakdown__blk1586_dn6, var_fbreakdown__blk1586_dn7, var_fbreakdown__blk1586_dn8, var_fbreakdown__blk1586_dn9, var_fbreakdown__blk1586_dn11, var_fbreakdown__blk1586_dn12,)
    }
};
        var_fbreakdown__blk1586 = assign60860_e78783;
        var_fbreakdown__blk1586_dn6 = assign60860_e78783_d_n6;
        var_fbreakdown__blk1586_dn7 = assign60860_e78783_d_n7;
        var_fbreakdown__blk1586_dn8 = assign60860_e78783_d_n8;
        var_fbreakdown__blk1586_dn9 = assign60860_e78783_d_n9;
        var_fbreakdown__blk1586_dn11 = assign60860_e78783_d_n11;
        var_fbreakdown__blk1586_dn12 = assign60860_e78783_d_n12;

        let assign60870_e78786: f64 = (-var_alphaav);
        let assign60870_e78788: f64 = (assign60870_e78786 * var_vbrstid_i);
        let assign60870_e78789: f64 = if var_vav__blk1559 > assign60870_e78788 { 1.0 } else { 0.0 };
        var_guard1711 = assign60870_e78789;

        let assign60880_e78792: f64 = if var_pbrstid_i == 4.0 { 1.0 } else { 0.0 };
        var_guard1712 = assign60880_e78792;

        let (assign60890_e78823, assign60890_e78823_d_n6, assign60890_e78823_d_n7, assign60890_e78823_d_n8, assign60890_e78823_d_n9, assign60890_e78823_d_n11, assign60890_e78823_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1710 == 0.0)) && (var_guard1711 != 0.0)) && (var_guard1712 != 0.0)) {
        let assign60890_e78809: f64 = (var_vav__blk1559 * var_vbrinvsti_d);
        let assign60890_e78812: f64 = (var_vav__blk1559 * var_vbrinvsti_d);
        let assign60890_e78813: f64 = (assign60890_e78809 * assign60890_e78812);
        let assign60890_e78816: f64 = (var_vav__blk1559 * var_vbrinvsti_d);
        let assign60890_e78817: f64 = (assign60890_e78813 * assign60890_e78816);
        let assign60890_e78820: f64 = (var_vav__blk1559 * var_vbrinvsti_d);
        let assign60890_e78821: f64 = (assign60890_e78817 * assign60890_e78820);
        (assign60890_e78821, 0.0, (((((((var_vav__blk1559_dn7 * var_vbrinvsti_d) * assign60890_e78812) + (assign60890_e78809 * (var_vav__blk1559_dn7 * var_vbrinvsti_d))) * assign60890_e78816) + (assign60890_e78813 * (var_vav__blk1559_dn7 * var_vbrinvsti_d))) * assign60890_e78820) + (assign60890_e78817 * (var_vav__blk1559_dn7 * var_vbrinvsti_d))), (((((((var_vav__blk1559_dn8 * var_vbrinvsti_d) * assign60890_e78812) + (assign60890_e78809 * (var_vav__blk1559_dn8 * var_vbrinvsti_d))) * assign60890_e78816) + (assign60890_e78813 * (var_vav__blk1559_dn8 * var_vbrinvsti_d))) * assign60890_e78820) + (assign60890_e78817 * (var_vav__blk1559_dn8 * var_vbrinvsti_d))), 0.0, (((((((var_vav__blk1559_dn11 * var_vbrinvsti_d) * assign60890_e78812) + (assign60890_e78809 * (var_vav__blk1559_dn11 * var_vbrinvsti_d))) * assign60890_e78816) + (assign60890_e78813 * (var_vav__blk1559_dn11 * var_vbrinvsti_d))) * assign60890_e78820) + (assign60890_e78817 * (var_vav__blk1559_dn11 * var_vbrinvsti_d))), (((((((var_vav__blk1559_dn12 * var_vbrinvsti_d) * assign60890_e78812) + (assign60890_e78809 * (var_vav__blk1559_dn12 * var_vbrinvsti_d))) * assign60890_e78816) + (assign60890_e78813 * (var_vav__blk1559_dn12 * var_vbrinvsti_d))) * assign60890_e78820) + (assign60890_e78817 * (var_vav__blk1559_dn12 * var_vbrinvsti_d))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60890_e78823;
        var_tmp__blk1560_dn6 = assign60890_e78823_d_n6;
        var_tmp__blk1560_dn7 = assign60890_e78823_d_n7;
        var_tmp__blk1560_dn8 = assign60890_e78823_d_n8;
        var_tmp__blk1560_dn9 = assign60890_e78823_d_n9;
        var_tmp__blk1560_dn11 = assign60890_e78823_d_n11;
        var_tmp__blk1560_dn12 = assign60890_e78823_d_n12;

        let (assign60900_e78846, assign60900_e78846_d_n6, assign60900_e78846_d_n7, assign60900_e78846_d_n8, assign60900_e78846_d_n9, assign60900_e78846_d_n11, assign60900_e78846_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1710 == 0.0)) && (var_guard1711 != 0.0)) && (var_guard1712 == 0.0)) {
        let assign60900_e78841: f64 = (var_vav__blk1559 * var_vbrinvsti_d);
        let assign60900_e78842: f64 = (assign60900_e78841).abs();
        let assign60900_e78844: f64 = (assign60900_e78842).powf(var_pbrstid_i);
        (assign60900_e78844, 0.0, if 0.0 == 0.0 && ((var_pbrstid_i) as f64).is_finite() && ((var_pbrstid_i) as f64).fract() == 0.0 { if var_pbrstid_i == 0.0 { 0.0 } else { (var_pbrstid_i * ((assign60900_e78842).powf(var_pbrstid_i - 1.0) * if assign60900_e78841 >= 0.0 { (var_vav__blk1559_dn7 * var_vbrinvsti_d) } else { (-(var_vav__blk1559_dn7 * var_vbrinvsti_d)) })) } } else { (assign60900_e78844 * (var_pbrstid_i * (if assign60900_e78841 >= 0.0 { (var_vav__blk1559_dn7 * var_vbrinvsti_d) } else { (-(var_vav__blk1559_dn7 * var_vbrinvsti_d)) } / assign60900_e78842))) }, if 0.0 == 0.0 && ((var_pbrstid_i) as f64).is_finite() && ((var_pbrstid_i) as f64).fract() == 0.0 { if var_pbrstid_i == 0.0 { 0.0 } else { (var_pbrstid_i * ((assign60900_e78842).powf(var_pbrstid_i - 1.0) * if assign60900_e78841 >= 0.0 { (var_vav__blk1559_dn8 * var_vbrinvsti_d) } else { (-(var_vav__blk1559_dn8 * var_vbrinvsti_d)) })) } } else { (assign60900_e78844 * (var_pbrstid_i * (if assign60900_e78841 >= 0.0 { (var_vav__blk1559_dn8 * var_vbrinvsti_d) } else { (-(var_vav__blk1559_dn8 * var_vbrinvsti_d)) } / assign60900_e78842))) }, 0.0, if 0.0 == 0.0 && ((var_pbrstid_i) as f64).is_finite() && ((var_pbrstid_i) as f64).fract() == 0.0 { if var_pbrstid_i == 0.0 { 0.0 } else { (var_pbrstid_i * ((assign60900_e78842).powf(var_pbrstid_i - 1.0) * if assign60900_e78841 >= 0.0 { (var_vav__blk1559_dn11 * var_vbrinvsti_d) } else { (-(var_vav__blk1559_dn11 * var_vbrinvsti_d)) })) } } else { (assign60900_e78844 * (var_pbrstid_i * (if assign60900_e78841 >= 0.0 { (var_vav__blk1559_dn11 * var_vbrinvsti_d) } else { (-(var_vav__blk1559_dn11 * var_vbrinvsti_d)) } / assign60900_e78842))) }, if 0.0 == 0.0 && ((var_pbrstid_i) as f64).is_finite() && ((var_pbrstid_i) as f64).fract() == 0.0 { if var_pbrstid_i == 0.0 { 0.0 } else { (var_pbrstid_i * ((assign60900_e78842).powf(var_pbrstid_i - 1.0) * if assign60900_e78841 >= 0.0 { (var_vav__blk1559_dn12 * var_vbrinvsti_d) } else { (-(var_vav__blk1559_dn12 * var_vbrinvsti_d)) })) } } else { (assign60900_e78844 * (var_pbrstid_i * (if assign60900_e78841 >= 0.0 { (var_vav__blk1559_dn12 * var_vbrinvsti_d) } else { (-(var_vav__blk1559_dn12 * var_vbrinvsti_d)) } / assign60900_e78842))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60900_e78846;
        var_tmp__blk1560_dn6 = assign60900_e78846_d_n6;
        var_tmp__blk1560_dn7 = assign60900_e78846_d_n7;
        var_tmp__blk1560_dn8 = assign60900_e78846_d_n8;
        var_tmp__blk1560_dn9 = assign60900_e78846_d_n9;
        var_tmp__blk1560_dn11 = assign60900_e78846_d_n11;
        var_tmp__blk1560_dn12 = assign60900_e78846_d_n12;

        let (assign60910_e78865, assign60910_e78865_d_n6, assign60910_e78865_d_n7, assign60910_e78865_d_n8, assign60910_e78865_d_n9, assign60910_e78865_d_n11, assign60910_e78865_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1710 == 0.0)) && (var_guard1711 != 0.0)) {
        let assign60910_e78862: f64 = (1.0 - var_tmp__blk1560);
        let assign60910_e78863: f64 = (1.0 / assign60910_e78862);
        (assign60910_e78863, (-((-var_tmp__blk1560_dn6) / (assign60910_e78862 * assign60910_e78862))), (-((-var_tmp__blk1560_dn7) / (assign60910_e78862 * assign60910_e78862))), (-((-var_tmp__blk1560_dn8) / (assign60910_e78862 * assign60910_e78862))), (-((-var_tmp__blk1560_dn9) / (assign60910_e78862 * assign60910_e78862))), (-((-var_tmp__blk1560_dn11) / (assign60910_e78862 * assign60910_e78862))), (-((-var_tmp__blk1560_dn12) / (assign60910_e78862 * assign60910_e78862))),)
    } else {
        (var_fbreakdown__blk1586, var_fbreakdown__blk1586_dn6, var_fbreakdown__blk1586_dn7, var_fbreakdown__blk1586_dn8, var_fbreakdown__blk1586_dn9, var_fbreakdown__blk1586_dn11, var_fbreakdown__blk1586_dn12,)
    }
};
        var_fbreakdown__blk1586 = assign60910_e78865;
        var_fbreakdown__blk1586_dn6 = assign60910_e78865_d_n6;
        var_fbreakdown__blk1586_dn7 = assign60910_e78865_d_n7;
        var_fbreakdown__blk1586_dn8 = assign60910_e78865_d_n8;
        var_fbreakdown__blk1586_dn9 = assign60910_e78865_d_n9;
        var_fbreakdown__blk1586_dn11 = assign60910_e78865_d_n11;
        var_fbreakdown__blk1586_dn12 = assign60910_e78865_d_n12;

        let (assign60920_e78889, assign60920_e78889_d_n6, assign60920_e78889_d_n7, assign60920_e78889_d_n8, assign60920_e78889_d_n9, assign60920_e78889_d_n11, assign60920_e78889_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1710 == 0.0)) && (var_guard1711 == 0.0)) {
        let assign60920_e78883: f64 = (var_alphaav * var_vbrstid_i);
        let assign60920_e78884: f64 = (var_vav__blk1559 + assign60920_e78883);
        let assign60920_e78886: f64 = (assign60920_e78884 * var_slopesti_d);
        let assign60920_e78887: f64 = (var_fstopsti_d + assign60920_e78886);
        (assign60920_e78887, 0.0, (var_vav__blk1559_dn7 * var_slopesti_d), (var_vav__blk1559_dn8 * var_slopesti_d), 0.0, (var_vav__blk1559_dn11 * var_slopesti_d), (var_vav__blk1559_dn12 * var_slopesti_d),)
    } else {
        (var_fbreakdown__blk1586, var_fbreakdown__blk1586_dn6, var_fbreakdown__blk1586_dn7, var_fbreakdown__blk1586_dn8, var_fbreakdown__blk1586_dn9, var_fbreakdown__blk1586_dn11, var_fbreakdown__blk1586_dn12,)
    }
};
        var_fbreakdown__blk1586 = assign60920_e78889;
        var_fbreakdown__blk1586_dn6 = assign60920_e78889_d_n6;
        var_fbreakdown__blk1586_dn7 = assign60920_e78889_d_n7;
        var_fbreakdown__blk1586_dn8 = assign60920_e78889_d_n8;
        var_fbreakdown__blk1586_dn9 = assign60920_e78889_d_n9;
        var_fbreakdown__blk1586_dn11 = assign60920_e78889_d_n11;
        var_fbreakdown__blk1586_dn12 = assign60920_e78889_d_n12;

        let (assign60930_e78909, assign60930_e78909_d_n6, assign60930_e78909_d_n7, assign60930_e78909_d_n8, assign60930_e78909_d_n9, assign60930_e78909_d_n11, assign60930_e78909_d_n12,) = {
    if (((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) {
        let assign60930_e78900: f64 = (var_id__blk1561 + var_isrh__blk1562);
        let assign60930_e78902: f64 = (assign60930_e78900 + var_itat__blk1569);
        let assign60930_e78904: f64 = (assign60930_e78902 + var_ibbt__blk1584);
        let assign60930_e78905: f64 = (p.p29 * assign60930_e78904);
        let assign60930_e78907: f64 = (assign60930_e78905 * var_fbreakdown__blk1586);
        (assign60930_e78907, (((p.p29 * ((var_isrh__blk1562_dn6 + var_itat__blk1569_dn6) + var_ibbt__blk1584_dn6)) * var_fbreakdown__blk1586) + (assign60930_e78905 * var_fbreakdown__blk1586_dn6)), (((p.p29 * (((var_id__blk1561_dn7 + var_isrh__blk1562_dn7) + var_itat__blk1569_dn7) + var_ibbt__blk1584_dn7)) * var_fbreakdown__blk1586) + (assign60930_e78905 * var_fbreakdown__blk1586_dn7)), (((p.p29 * (((var_id__blk1561_dn8 + var_isrh__blk1562_dn8) + var_itat__blk1569_dn8) + var_ibbt__blk1584_dn8)) * var_fbreakdown__blk1586) + (assign60930_e78905 * var_fbreakdown__blk1586_dn8)), (((p.p29 * ((var_isrh__blk1562_dn9 + var_itat__blk1569_dn9) + var_ibbt__blk1584_dn9)) * var_fbreakdown__blk1586) + (assign60930_e78905 * var_fbreakdown__blk1586_dn9)), (((p.p29 * (((var_id__blk1561_dn11 + var_isrh__blk1562_dn11) + var_itat__blk1569_dn11) + var_ibbt__blk1584_dn11)) * var_fbreakdown__blk1586) + (assign60930_e78905 * var_fbreakdown__blk1586_dn11)), (((p.p29 * (((var_id__blk1561_dn12 + var_isrh__blk1562_dn12) + var_itat__blk1569_dn12) + var_ibbt__blk1584_dn12)) * var_fbreakdown__blk1586) + (assign60930_e78905 * var_fbreakdown__blk1586_dn12)),)
    } else {
        (var_ijunsti_d, var_ijunsti_d_dn6, var_ijunsti_d_dn7, var_ijunsti_d_dn8, var_ijunsti_d_dn9, var_ijunsti_d_dn11, var_ijunsti_d_dn12,)
    }
};
        var_ijunsti_d = assign60930_e78909;
        var_ijunsti_d_dn6 = assign60930_e78909_d_n6;
        var_ijunsti_d_dn7 = assign60930_e78909_d_n7;
        var_ijunsti_d_dn8 = assign60930_e78909_d_n8;
        var_ijunsti_d_dn9 = assign60930_e78909_d_n9;
        var_ijunsti_d_dn11 = assign60930_e78909_d_n11;
        var_ijunsti_d_dn12 = assign60930_e78909_d_n12;

        let assign60940_e78912: f64 = if var_one_minus_psti_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1713 = assign60940_e78912;

        let (assign60950_e78929, assign60950_e78929_d_n6, assign60950_e78929_d_n7, assign60950_e78929_d_n8, assign60950_e78929_d_n9, assign60950_e78929_d_n11, assign60950_e78929_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1713 != 0.0)) {
        let assign60950_e78925: f64 = (var_vj__blk1552 * var_vbiinvsti_d);
        let assign60950_e78926: f64 = (1.0 - assign60950_e78925);
        let assign60950_e78927: f64 = (assign60950_e78926).sqrt();
        (assign60950_e78927, 0.0, ((-(var_vj__blk1552_dn7 * var_vbiinvsti_d)) / (2.0 * assign60950_e78927)), ((-(var_vj__blk1552_dn8 * var_vbiinvsti_d)) / (2.0 * assign60950_e78927)), 0.0, ((-(var_vj__blk1552_dn11 * var_vbiinvsti_d)) / (2.0 * assign60950_e78927)), ((-(var_vj__blk1552_dn12 * var_vbiinvsti_d)) / (2.0 * assign60950_e78927)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60950_e78929;
        var_tmp__blk1560_dn6 = assign60950_e78929_d_n6;
        var_tmp__blk1560_dn7 = assign60950_e78929_d_n7;
        var_tmp__blk1560_dn8 = assign60950_e78929_d_n8;
        var_tmp__blk1560_dn9 = assign60950_e78929_d_n9;
        var_tmp__blk1560_dn11 = assign60950_e78929_d_n11;
        var_tmp__blk1560_dn12 = assign60950_e78929_d_n12;

        let (assign60960_e78948, assign60960_e78948_d_n6, assign60960_e78948_d_n7, assign60960_e78948_d_n8, assign60960_e78948_d_n9, assign60960_e78948_d_n11, assign60960_e78948_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) && (var_guard1713 == 0.0)) {
        let assign60960_e78943: f64 = (var_vj__blk1552 * var_vbiinvsti_d);
        let assign60960_e78944: f64 = (1.0 - assign60960_e78943);
        let assign60960_e78946: f64 = (assign60960_e78944).powf(var_one_minus_psti_d);
        (assign60960_e78946, 0.0, if 0.0 == 0.0 && ((var_one_minus_psti_d) as f64).is_finite() && ((var_one_minus_psti_d) as f64).fract() == 0.0 { if var_one_minus_psti_d == 0.0 { 0.0 } else { (var_one_minus_psti_d * ((assign60960_e78944).powf(var_one_minus_psti_d - 1.0) * (-(var_vj__blk1552_dn7 * var_vbiinvsti_d)))) } } else { (assign60960_e78946 * (var_one_minus_psti_d * ((-(var_vj__blk1552_dn7 * var_vbiinvsti_d)) / assign60960_e78944))) }, if 0.0 == 0.0 && ((var_one_minus_psti_d) as f64).is_finite() && ((var_one_minus_psti_d) as f64).fract() == 0.0 { if var_one_minus_psti_d == 0.0 { 0.0 } else { (var_one_minus_psti_d * ((assign60960_e78944).powf(var_one_minus_psti_d - 1.0) * (-(var_vj__blk1552_dn8 * var_vbiinvsti_d)))) } } else { (assign60960_e78946 * (var_one_minus_psti_d * ((-(var_vj__blk1552_dn8 * var_vbiinvsti_d)) / assign60960_e78944))) }, 0.0, if 0.0 == 0.0 && ((var_one_minus_psti_d) as f64).is_finite() && ((var_one_minus_psti_d) as f64).fract() == 0.0 { if var_one_minus_psti_d == 0.0 { 0.0 } else { (var_one_minus_psti_d * ((assign60960_e78944).powf(var_one_minus_psti_d - 1.0) * (-(var_vj__blk1552_dn11 * var_vbiinvsti_d)))) } } else { (assign60960_e78946 * (var_one_minus_psti_d * ((-(var_vj__blk1552_dn11 * var_vbiinvsti_d)) / assign60960_e78944))) }, if 0.0 == 0.0 && ((var_one_minus_psti_d) as f64).is_finite() && ((var_one_minus_psti_d) as f64).fract() == 0.0 { if var_one_minus_psti_d == 0.0 { 0.0 } else { (var_one_minus_psti_d * ((assign60960_e78944).powf(var_one_minus_psti_d - 1.0) * (-(var_vj__blk1552_dn12 * var_vbiinvsti_d)))) } } else { (assign60960_e78946 * (var_one_minus_psti_d * ((-(var_vj__blk1552_dn12 * var_vbiinvsti_d)) / assign60960_e78944))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign60960_e78948;
        var_tmp__blk1560_dn6 = assign60960_e78948_d_n6;
        var_tmp__blk1560_dn7 = assign60960_e78948_d_n7;
        var_tmp__blk1560_dn8 = assign60960_e78948_d_n8;
        var_tmp__blk1560_dn9 = assign60960_e78948_d_n9;
        var_tmp__blk1560_dn11 = assign60960_e78948_d_n11;
        var_tmp__blk1560_dn12 = assign60960_e78948_d_n12;

        let (assign60970_e78970, assign60970_e78970_d_n6, assign60970_e78970_d_n7, assign60970_e78970_d_n8, assign60970_e78970_d_n9, assign60970_e78970_d_n11, assign60970_e78970_d_n12,) = {
    if (((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1696 == 0.0)) {
        let assign60970_e78960: f64 = (1.0 - var_tmp__blk1560);
        let assign60970_e78961: f64 = (var_qprefsti_d * assign60970_e78960);
        let assign60970_e78965: f64 = (var_vjun_d - var_vj__blk1552);
        let assign60970_e78966: f64 = (var_qpref2sti_d * assign60970_e78965);
        let assign60970_e78967: f64 = (assign60970_e78961 + assign60970_e78966);
        let assign60970_e78968: f64 = (p.p30 * assign60970_e78967);
        (assign60970_e78968, (p.p30 * (var_qprefsti_d * (-var_tmp__blk1560_dn6))), (p.p30 * ((var_qprefsti_d * (-var_tmp__blk1560_dn7)) + (var_qpref2sti_d * (-var_vj__blk1552_dn7)))), (p.p30 * ((var_qprefsti_d * (-var_tmp__blk1560_dn8)) + (var_qpref2sti_d * (var_vjun_d_dn8 - var_vj__blk1552_dn8)))), (p.p30 * (var_qprefsti_d * (-var_tmp__blk1560_dn9))), (p.p30 * ((var_qprefsti_d * (-var_tmp__blk1560_dn11)) + (var_qpref2sti_d * (-var_vj__blk1552_dn11)))), (p.p30 * ((var_qprefsti_d * (-var_tmp__blk1560_dn12)) + (var_qpref2sti_d * (var_vjun_d_dn12 - var_vj__blk1552_dn12)))),)
    } else {
        (var_qjunsti_d, var_qjunsti_d_dn6, var_qjunsti_d_dn7, var_qjunsti_d_dn8, var_qjunsti_d_dn9, var_qjunsti_d_dn11, var_qjunsti_d_dn12,)
    }
};
        var_qjunsti_d = assign60970_e78970;
        var_qjunsti_d_dn6 = assign60970_e78970_d_n6;
        var_qjunsti_d_dn7 = assign60970_e78970_d_n7;
        var_qjunsti_d_dn8 = assign60970_e78970_d_n8;
        var_qjunsti_d_dn9 = assign60970_e78970_d_n9;
        var_qjunsti_d_dn11 = assign60970_e78970_d_n11;
        var_qjunsti_d_dn12 = assign60970_e78970_d_n12;

        let assign60980_e78973: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1714 = assign60980_e78973;

        let (assign60990_e78982, assign60990_e78982_d_n6, assign60990_e78982_d_n7, assign60990_e78982_d_n8, assign60990_e78982_d_n9, assign60990_e78982_d_n11, assign60990_e78982_d_n12,) = {
    if (((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat_d, var_ijungat_d_dn6, var_ijungat_d_dn7, var_ijungat_d_dn8, var_ijungat_d_dn9, var_ijungat_d_dn11, var_ijungat_d_dn12,)
    }
};
        var_ijungat_d = assign60990_e78982;
        var_ijungat_d_dn6 = assign60990_e78982_d_n6;
        var_ijungat_d_dn7 = assign60990_e78982_d_n7;
        var_ijungat_d_dn8 = assign60990_e78982_d_n8;
        var_ijungat_d_dn9 = assign60990_e78982_d_n9;
        var_ijungat_d_dn11 = assign60990_e78982_d_n11;
        var_ijungat_d_dn12 = assign60990_e78982_d_n12;

        let (assign61000_e78991, assign61000_e78991_d_n6, assign61000_e78991_d_n7, assign61000_e78991_d_n8, assign61000_e78991_d_n9, assign61000_e78991_d_n11, assign61000_e78991_d_n12,) = {
    if (((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qjungat_d, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn9, var_qjungat_d_dn11, var_qjungat_d_dn12,)
    }
};
        var_qjungat_d = assign61000_e78991;
        var_qjungat_d_dn6 = assign61000_e78991_d_n6;
        var_qjungat_d_dn7 = assign61000_e78991_d_n7;
        var_qjungat_d_dn8 = assign61000_e78991_d_n8;
        var_qjungat_d_dn9 = assign61000_e78991_d_n9;
        var_qjungat_d_dn11 = assign61000_e78991_d_n11;
        var_qjungat_d_dn12 = assign61000_e78991_d_n12;

        let (assign61010_e79003, assign61010_e79003_d_n7, assign61010_e79003_d_n8, assign61010_e79003_d_n11, assign61010_e79003_d_n12,) = {
    if (((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) {
        let assign61010_e79001: f64 = (var_idsatgat_d * var_idmult__blk1551);
        (assign61010_e79001, (var_idsatgat_d * var_idmult__blk1551_dn7), (var_idsatgat_d * var_idmult__blk1551_dn8), (var_idsatgat_d * var_idmult__blk1551_dn11), (var_idsatgat_d * var_idmult__blk1551_dn12),)
    } else {
        (var_id__blk1561, var_id__blk1561_dn7, var_id__blk1561_dn8, var_id__blk1561_dn11, var_id__blk1561_dn12,)
    }
};
        var_id__blk1561 = assign61010_e79003;
        var_id__blk1561_dn7 = assign61010_e79003_d_n7;
        var_id__blk1561_dn8 = assign61010_e79003_d_n8;
        var_id__blk1561_dn11 = assign61010_e79003_d_n11;
        var_id__blk1561_dn12 = assign61010_e79003_d_n12;

        let assign61020_e79010: f64 = if ((var_csrhgatd_i == 0.0) && (var_ctatgatd_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard1715 = assign61020_e79010;

        let (assign61030_e79022, assign61030_e79022_d_n6, assign61030_e79022_d_n7, assign61030_e79022_d_n8, assign61030_e79022_d_n9, assign61030_e79022_d_n11, assign61030_e79022_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh__blk1562, var_isrh__blk1562_dn6, var_isrh__blk1562_dn7, var_isrh__blk1562_dn8, var_isrh__blk1562_dn9, var_isrh__blk1562_dn11, var_isrh__blk1562_dn12,)
    }
};
        var_isrh__blk1562 = assign61030_e79022;
        var_isrh__blk1562_dn6 = assign61030_e79022_d_n6;
        var_isrh__blk1562_dn7 = assign61030_e79022_d_n7;
        var_isrh__blk1562_dn8 = assign61030_e79022_d_n8;
        var_isrh__blk1562_dn9 = assign61030_e79022_d_n9;
        var_isrh__blk1562_dn11 = assign61030_e79022_d_n11;
        var_isrh__blk1562_dn12 = assign61030_e79022_d_n12;

        let (assign61040_e79037, assign61040_e79037_d_n7, assign61040_e79037_d_n8, assign61040_e79037_d_n11, assign61040_e79037_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) {
        let assign61040_e79035: f64 = (var_vbigat_d - var_vjsrh__blk1557);
        (assign61040_e79035, (-var_vjsrh__blk1557_dn7), (-var_vjsrh__blk1557_dn8), (-var_vjsrh__blk1557_dn11), (-var_vjsrh__blk1557_dn12),)
    } else {
        (var_vbi_minus_vjsrh__blk1563, var_vbi_minus_vjsrh__blk1563_dn7, var_vbi_minus_vjsrh__blk1563_dn8, var_vbi_minus_vjsrh__blk1563_dn11, var_vbi_minus_vjsrh__blk1563_dn12,)
    }
};
        var_vbi_minus_vjsrh__blk1563 = assign61040_e79037;
        var_vbi_minus_vjsrh__blk1563_dn7 = assign61040_e79037_d_n7;
        var_vbi_minus_vjsrh__blk1563_dn8 = assign61040_e79037_d_n8;
        var_vbi_minus_vjsrh__blk1563_dn11 = assign61040_e79037_d_n11;
        var_vbi_minus_vjsrh__blk1563_dn12 = assign61040_e79037_d_n12;

        let (assign61050_e79057, assign61050_e79057_d_n7, assign61050_e79057_d_n8, assign61050_e79057_d_n11, assign61050_e79057_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) {
        let assign61050_e79052: f64 = (var_two_psistar__blk1555 / var_vbi_minus_vjsrh__blk1563);
        let assign61050_e79053: f64 = (1.0 - assign61050_e79052);
        let assign61050_e79054: f64 = (assign61050_e79053).sqrt();
        let assign61050_e79055: f64 = (1.0 - assign61050_e79054);
        (assign61050_e79055, (-((-(((var_two_psistar__blk1555_dn7 * var_vbi_minus_vjsrh__blk1563) - (var_two_psistar__blk1555 * var_vbi_minus_vjsrh__blk1563_dn7)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))) / (2.0 * assign61050_e79054))), (-((-(((var_two_psistar__blk1555_dn8 * var_vbi_minus_vjsrh__blk1563) - (var_two_psistar__blk1555 * var_vbi_minus_vjsrh__blk1563_dn8)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))) / (2.0 * assign61050_e79054))), (-((-(((var_two_psistar__blk1555_dn11 * var_vbi_minus_vjsrh__blk1563) - (var_two_psistar__blk1555 * var_vbi_minus_vjsrh__blk1563_dn11)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))) / (2.0 * assign61050_e79054))), (-((-(((var_two_psistar__blk1555_dn12 * var_vbi_minus_vjsrh__blk1563) - (var_two_psistar__blk1555 * var_vbi_minus_vjsrh__blk1563_dn12)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))) / (2.0 * assign61050_e79054))),)
    } else {
        (var_wsrhstep__blk1564, var_wsrhstep__blk1564_dn7, var_wsrhstep__blk1564_dn8, var_wsrhstep__blk1564_dn11, var_wsrhstep__blk1564_dn12,)
    }
};
        var_wsrhstep__blk1564 = assign61050_e79057;
        var_wsrhstep__blk1564_dn7 = assign61050_e79057_d_n7;
        var_wsrhstep__blk1564_dn8 = assign61050_e79057_d_n8;
        var_wsrhstep__blk1564_dn11 = assign61050_e79057_d_n11;
        var_wsrhstep__blk1564_dn12 = assign61050_e79057_d_n12;

        let assign61060_e79060: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1716 = assign61060_e79060;

        let (assign61070_e79075, assign61070_e79075_d_n7, assign61070_e79075_d_n8, assign61070_e79075_d_n11, assign61070_e79075_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) && (var_guard1716 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dwsrh__blk1565, var_dwsrh__blk1565_dn7, var_dwsrh__blk1565_dn8, var_dwsrh__blk1565_dn11, var_dwsrh__blk1565_dn12,)
    }
};
        var_dwsrh__blk1565 = assign61070_e79075;
        var_dwsrh__blk1565_dn7 = assign61070_e79075_d_n7;
        var_dwsrh__blk1565_dn8 = assign61070_e79075_d_n8;
        var_dwsrh__blk1565_dn11 = assign61070_e79075_d_n11;
        var_dwsrh__blk1565_dn12 = assign61070_e79075_d_n12;

        let (assign61080_e79108, assign61080_e79108_d_n7, assign61080_e79108_d_n8, assign61080_e79108_d_n11, assign61080_e79108_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) && (var_guard1716 == 0.0)) {
        let assign61080_e79091: f64 = (var_wsrhstep__blk1564 * var_wsrhstep__blk1564);
        let assign61080_e79093: f64 = (var_wsrhstep__blk1564).ln();
        let assign61080_e79094: f64 = (assign61080_e79091 * assign61080_e79093);
        let assign61080_e79097: f64 = (1.0 - var_wsrhstep__blk1564);
        let assign61080_e79098: f64 = (assign61080_e79094 / assign61080_e79097);
        let assign61080_e79100: f64 = (assign61080_e79098 + var_wsrhstep__blk1564);
        let assign61080_e79104: f64 = (2.0 * var_pgatd_i);
        let assign61080_e79105: f64 = (1.0 - assign61080_e79104);
        let assign61080_e79106: f64 = (assign61080_e79100 * assign61080_e79105);
        (assign61080_e79106, (((((((((var_wsrhstep__blk1564_dn7 * var_wsrhstep__blk1564) + (var_wsrhstep__blk1564 * var_wsrhstep__blk1564_dn7)) * assign61080_e79093) + (assign61080_e79091 * (var_wsrhstep__blk1564_dn7 / var_wsrhstep__blk1564))) * assign61080_e79097) - (assign61080_e79094 * (-var_wsrhstep__blk1564_dn7))) / (assign61080_e79097 * assign61080_e79097)) + var_wsrhstep__blk1564_dn7) * assign61080_e79105), (((((((((var_wsrhstep__blk1564_dn8 * var_wsrhstep__blk1564) + (var_wsrhstep__blk1564 * var_wsrhstep__blk1564_dn8)) * assign61080_e79093) + (assign61080_e79091 * (var_wsrhstep__blk1564_dn8 / var_wsrhstep__blk1564))) * assign61080_e79097) - (assign61080_e79094 * (-var_wsrhstep__blk1564_dn8))) / (assign61080_e79097 * assign61080_e79097)) + var_wsrhstep__blk1564_dn8) * assign61080_e79105), (((((((((var_wsrhstep__blk1564_dn11 * var_wsrhstep__blk1564) + (var_wsrhstep__blk1564 * var_wsrhstep__blk1564_dn11)) * assign61080_e79093) + (assign61080_e79091 * (var_wsrhstep__blk1564_dn11 / var_wsrhstep__blk1564))) * assign61080_e79097) - (assign61080_e79094 * (-var_wsrhstep__blk1564_dn11))) / (assign61080_e79097 * assign61080_e79097)) + var_wsrhstep__blk1564_dn11) * assign61080_e79105), (((((((((var_wsrhstep__blk1564_dn12 * var_wsrhstep__blk1564) + (var_wsrhstep__blk1564 * var_wsrhstep__blk1564_dn12)) * assign61080_e79093) + (assign61080_e79091 * (var_wsrhstep__blk1564_dn12 / var_wsrhstep__blk1564))) * assign61080_e79097) - (assign61080_e79094 * (-var_wsrhstep__blk1564_dn12))) / (assign61080_e79097 * assign61080_e79097)) + var_wsrhstep__blk1564_dn12) * assign61080_e79105),)
    } else {
        (var_dwsrh__blk1565, var_dwsrh__blk1565_dn7, var_dwsrh__blk1565_dn8, var_dwsrh__blk1565_dn11, var_dwsrh__blk1565_dn12,)
    }
};
        var_dwsrh__blk1565 = assign61080_e79108;
        var_dwsrh__blk1565_dn7 = assign61080_e79108_d_n7;
        var_dwsrh__blk1565_dn8 = assign61080_e79108_d_n8;
        var_dwsrh__blk1565_dn11 = assign61080_e79108_d_n11;
        var_dwsrh__blk1565_dn12 = assign61080_e79108_d_n12;

        let (assign61090_e79123, assign61090_e79123_d_n7, assign61090_e79123_d_n8, assign61090_e79123_d_n11, assign61090_e79123_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) {
        let assign61090_e79121: f64 = (var_wsrhstep__blk1564 + var_dwsrh__blk1565);
        (assign61090_e79121, (var_wsrhstep__blk1564_dn7 + var_dwsrh__blk1565_dn7), (var_wsrhstep__blk1564_dn8 + var_dwsrh__blk1565_dn8), (var_wsrhstep__blk1564_dn11 + var_dwsrh__blk1565_dn11), (var_wsrhstep__blk1564_dn12 + var_dwsrh__blk1565_dn12),)
    } else {
        (var_wsrh__blk1566, var_wsrh__blk1566_dn7, var_wsrh__blk1566_dn8, var_wsrh__blk1566_dn11, var_wsrh__blk1566_dn12,)
    }
};
        var_wsrh__blk1566 = assign61090_e79123;
        var_wsrh__blk1566_dn7 = assign61090_e79123_d_n7;
        var_wsrh__blk1566_dn8 = assign61090_e79123_d_n8;
        var_wsrh__blk1566_dn11 = assign61090_e79123_d_n11;
        var_wsrh__blk1566_dn12 = assign61090_e79123_d_n12;

        let assign61100_e79126: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1717 = assign61100_e79126;

        let (assign61110_e79144, assign61110_e79144_d_n6, assign61110_e79144_d_n7, assign61110_e79144_d_n8, assign61110_e79144_d_n9, assign61110_e79144_d_n11, assign61110_e79144_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) && (var_guard1717 != 0.0)) {
        let assign61110_e79141: f64 = (var_vbi_minus_vjsrh__blk1563 * var_vbirgatinv_d);
        let assign61110_e79142: f64 = (assign61110_e79141).sqrt();
        (assign61110_e79142, 0.0, ((var_vbi_minus_vjsrh__blk1563_dn7 * var_vbirgatinv_d) / (2.0 * assign61110_e79142)), ((var_vbi_minus_vjsrh__blk1563_dn8 * var_vbirgatinv_d) / (2.0 * assign61110_e79142)), 0.0, ((var_vbi_minus_vjsrh__blk1563_dn11 * var_vbirgatinv_d) / (2.0 * assign61110_e79142)), ((var_vbi_minus_vjsrh__blk1563_dn12 * var_vbirgatinv_d) / (2.0 * assign61110_e79142)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61110_e79144;
        var_tmp__blk1560_dn6 = assign61110_e79144_d_n6;
        var_tmp__blk1560_dn7 = assign61110_e79144_d_n7;
        var_tmp__blk1560_dn8 = assign61110_e79144_d_n8;
        var_tmp__blk1560_dn9 = assign61110_e79144_d_n9;
        var_tmp__blk1560_dn11 = assign61110_e79144_d_n11;
        var_tmp__blk1560_dn12 = assign61110_e79144_d_n12;

        let (assign61120_e79164, assign61120_e79164_d_n6, assign61120_e79164_d_n7, assign61120_e79164_d_n8, assign61120_e79164_d_n9, assign61120_e79164_d_n11, assign61120_e79164_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) && (var_guard1717 == 0.0)) {
        let assign61120_e79160: f64 = (var_vbi_minus_vjsrh__blk1563 * var_vbirgatinv_d);
        let assign61120_e79162: f64 = (assign61120_e79160).powf(var_pgatd_i);
        (assign61120_e79162, 0.0, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61120_e79160).powf(var_pgatd_i - 1.0) * (var_vbi_minus_vjsrh__blk1563_dn7 * var_vbirgatinv_d))) } } else { (assign61120_e79162 * (var_pgatd_i * ((var_vbi_minus_vjsrh__blk1563_dn7 * var_vbirgatinv_d) / assign61120_e79160))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61120_e79160).powf(var_pgatd_i - 1.0) * (var_vbi_minus_vjsrh__blk1563_dn8 * var_vbirgatinv_d))) } } else { (assign61120_e79162 * (var_pgatd_i * ((var_vbi_minus_vjsrh__blk1563_dn8 * var_vbirgatinv_d) / assign61120_e79160))) }, 0.0, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61120_e79160).powf(var_pgatd_i - 1.0) * (var_vbi_minus_vjsrh__blk1563_dn11 * var_vbirgatinv_d))) } } else { (assign61120_e79162 * (var_pgatd_i * ((var_vbi_minus_vjsrh__blk1563_dn11 * var_vbirgatinv_d) / assign61120_e79160))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61120_e79160).powf(var_pgatd_i - 1.0) * (var_vbi_minus_vjsrh__blk1563_dn12 * var_vbirgatinv_d))) } } else { (assign61120_e79162 * (var_pgatd_i * ((var_vbi_minus_vjsrh__blk1563_dn12 * var_vbirgatinv_d) / assign61120_e79160))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61120_e79164;
        var_tmp__blk1560_dn6 = assign61120_e79164_d_n6;
        var_tmp__blk1560_dn7 = assign61120_e79164_d_n7;
        var_tmp__blk1560_dn8 = assign61120_e79164_d_n8;
        var_tmp__blk1560_dn9 = assign61120_e79164_d_n9;
        var_tmp__blk1560_dn11 = assign61120_e79164_d_n11;
        var_tmp__blk1560_dn12 = assign61120_e79164_d_n12;

        let (assign61130_e79179, assign61130_e79179_d_n6, assign61130_e79179_d_n7, assign61130_e79179_d_n8, assign61130_e79179_d_n9, assign61130_e79179_d_n11, assign61130_e79179_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) {
        let assign61130_e79177: f64 = (var_wdepnulrgat_d * var_tmp__blk1560);
        (assign61130_e79177, (var_wdepnulrgat_d * var_tmp__blk1560_dn6), (var_wdepnulrgat_d * var_tmp__blk1560_dn7), (var_wdepnulrgat_d * var_tmp__blk1560_dn8), (var_wdepnulrgat_d * var_tmp__blk1560_dn9), (var_wdepnulrgat_d * var_tmp__blk1560_dn11), (var_wdepnulrgat_d * var_tmp__blk1560_dn12),)
    } else {
        (var_wdep__blk1567, var_wdep__blk1567_dn6, var_wdep__blk1567_dn7, var_wdep__blk1567_dn8, var_wdep__blk1567_dn9, var_wdep__blk1567_dn11, var_wdep__blk1567_dn12,)
    }
};
        var_wdep__blk1567 = assign61130_e79179;
        var_wdep__blk1567_dn6 = assign61130_e79179_d_n6;
        var_wdep__blk1567_dn7 = assign61130_e79179_d_n7;
        var_wdep__blk1567_dn8 = assign61130_e79179_d_n8;
        var_wdep__blk1567_dn9 = assign61130_e79179_d_n9;
        var_wdep__blk1567_dn11 = assign61130_e79179_d_n11;
        var_wdep__blk1567_dn12 = assign61130_e79179_d_n12;

        let (assign61140_e79198, assign61140_e79198_d_n6, assign61140_e79198_d_n7, assign61140_e79198_d_n8, assign61140_e79198_d_n9, assign61140_e79198_d_n11, assign61140_e79198_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) {
        let assign61140_e79193: f64 = (var_zinv__blk1554 - 1.0);
        let assign61140_e79195: f64 = (assign61140_e79193 * var_wdep__blk1567);
        let assign61140_e79196: f64 = (var_ftdgat_d * assign61140_e79195);
        (assign61140_e79196, (var_ftdgat_d * (assign61140_e79193 * var_wdep__blk1567_dn6)), (var_ftdgat_d * ((var_zinv__blk1554_dn7 * var_wdep__blk1567) + (assign61140_e79193 * var_wdep__blk1567_dn7))), (var_ftdgat_d * ((var_zinv__blk1554_dn8 * var_wdep__blk1567) + (assign61140_e79193 * var_wdep__blk1567_dn8))), (var_ftdgat_d * (assign61140_e79193 * var_wdep__blk1567_dn9)), (var_ftdgat_d * ((var_zinv__blk1554_dn11 * var_wdep__blk1567) + (assign61140_e79193 * var_wdep__blk1567_dn11))), (var_ftdgat_d * ((var_zinv__blk1554_dn12 * var_wdep__blk1567) + (assign61140_e79193 * var_wdep__blk1567_dn12))),)
    } else {
        (var_asrh__blk1568, var_asrh__blk1568_dn6, var_asrh__blk1568_dn7, var_asrh__blk1568_dn8, var_asrh__blk1568_dn9, var_asrh__blk1568_dn11, var_asrh__blk1568_dn12,)
    }
};
        var_asrh__blk1568 = assign61140_e79198;
        var_asrh__blk1568_dn6 = assign61140_e79198_d_n6;
        var_asrh__blk1568_dn7 = assign61140_e79198_d_n7;
        var_asrh__blk1568_dn8 = assign61140_e79198_d_n8;
        var_asrh__blk1568_dn9 = assign61140_e79198_d_n9;
        var_asrh__blk1568_dn11 = assign61140_e79198_d_n11;
        var_asrh__blk1568_dn12 = assign61140_e79198_d_n12;

        let (assign61150_e79215, assign61150_e79215_d_n6, assign61150_e79215_d_n7, assign61150_e79215_d_n8, assign61150_e79215_d_n9, assign61150_e79215_d_n11, assign61150_e79215_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1715 == 0.0)) {
        let assign61150_e79212: f64 = (var_asrh__blk1568 * var_wsrh__blk1566);
        let assign61150_e79213: f64 = (var_csrhgatd_i * assign61150_e79212);
        (assign61150_e79213, (var_csrhgatd_i * (var_asrh__blk1568_dn6 * var_wsrh__blk1566)), (var_csrhgatd_i * ((var_asrh__blk1568_dn7 * var_wsrh__blk1566) + (var_asrh__blk1568 * var_wsrh__blk1566_dn7))), (var_csrhgatd_i * ((var_asrh__blk1568_dn8 * var_wsrh__blk1566) + (var_asrh__blk1568 * var_wsrh__blk1566_dn8))), (var_csrhgatd_i * (var_asrh__blk1568_dn9 * var_wsrh__blk1566)), (var_csrhgatd_i * ((var_asrh__blk1568_dn11 * var_wsrh__blk1566) + (var_asrh__blk1568 * var_wsrh__blk1566_dn11))), (var_csrhgatd_i * ((var_asrh__blk1568_dn12 * var_wsrh__blk1566) + (var_asrh__blk1568 * var_wsrh__blk1566_dn12))),)
    } else {
        (var_isrh__blk1562, var_isrh__blk1562_dn6, var_isrh__blk1562_dn7, var_isrh__blk1562_dn8, var_isrh__blk1562_dn9, var_isrh__blk1562_dn11, var_isrh__blk1562_dn12,)
    }
};
        var_isrh__blk1562 = assign61150_e79215;
        var_isrh__blk1562_dn6 = assign61150_e79215_d_n6;
        var_isrh__blk1562_dn7 = assign61150_e79215_d_n7;
        var_isrh__blk1562_dn8 = assign61150_e79215_d_n8;
        var_isrh__blk1562_dn9 = assign61150_e79215_d_n9;
        var_isrh__blk1562_dn11 = assign61150_e79215_d_n11;
        var_isrh__blk1562_dn12 = assign61150_e79215_d_n12;

        let assign61160_e79218: f64 = if var_ctatgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1718 = assign61160_e79218;

        let (assign61170_e79230, assign61170_e79230_d_n6, assign61170_e79230_d_n7, assign61170_e79230_d_n8, assign61170_e79230_d_n9, assign61170_e79230_d_n11, assign61170_e79230_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat__blk1569, var_itat__blk1569_dn6, var_itat__blk1569_dn7, var_itat__blk1569_dn8, var_itat__blk1569_dn9, var_itat__blk1569_dn11, var_itat__blk1569_dn12,)
    }
};
        var_itat__blk1569 = assign61170_e79230;
        var_itat__blk1569_dn6 = assign61170_e79230_d_n6;
        var_itat__blk1569_dn7 = assign61170_e79230_d_n7;
        var_itat__blk1569_dn8 = assign61170_e79230_d_n8;
        var_itat__blk1569_dn9 = assign61170_e79230_d_n9;
        var_itat__blk1569_dn11 = assign61170_e79230_d_n11;
        var_itat__blk1569_dn12 = assign61170_e79230_d_n12;

        let (assign61180_e79249, assign61180_e79249_d_n6, assign61180_e79249_d_n7, assign61180_e79249_d_n8, assign61180_e79249_d_n9, assign61180_e79249_d_n11, assign61180_e79249_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61180_e79244: f64 = (var_wdep__blk1567 * var_one_minus_pgat_d);
        let assign61180_e79246: f64 = (assign61180_e79244 / var_vbi_minus_vjsrh__blk1563);
        let assign61180_e79247: f64 = (var_btatpartgat_d * assign61180_e79246);
        (assign61180_e79247, (var_btatpartgat_d * ((var_wdep__blk1567_dn6 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh__blk1563)), (var_btatpartgat_d * ((((var_wdep__blk1567_dn7 * var_one_minus_pgat_d) * var_vbi_minus_vjsrh__blk1563) - (assign61180_e79244 * var_vbi_minus_vjsrh__blk1563_dn7)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))), (var_btatpartgat_d * ((((var_wdep__blk1567_dn8 * var_one_minus_pgat_d) * var_vbi_minus_vjsrh__blk1563) - (assign61180_e79244 * var_vbi_minus_vjsrh__blk1563_dn8)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))), (var_btatpartgat_d * ((var_wdep__blk1567_dn9 * var_one_minus_pgat_d) / var_vbi_minus_vjsrh__blk1563)), (var_btatpartgat_d * ((((var_wdep__blk1567_dn11 * var_one_minus_pgat_d) * var_vbi_minus_vjsrh__blk1563) - (assign61180_e79244 * var_vbi_minus_vjsrh__blk1563_dn11)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))), (var_btatpartgat_d * ((((var_wdep__blk1567_dn12 * var_one_minus_pgat_d) * var_vbi_minus_vjsrh__blk1563) - (assign61180_e79244 * var_vbi_minus_vjsrh__blk1563_dn12)) / (var_vbi_minus_vjsrh__blk1563 * var_vbi_minus_vjsrh__blk1563))),)
    } else {
        (var_btat__blk1570, var_btat__blk1570_dn6, var_btat__blk1570_dn7, var_btat__blk1570_dn8, var_btat__blk1570_dn9, var_btat__blk1570_dn11, var_btat__blk1570_dn12,)
    }
};
        var_btat__blk1570 = assign61180_e79249;
        var_btat__blk1570_dn6 = assign61180_e79249_d_n6;
        var_btat__blk1570_dn7 = assign61180_e79249_d_n7;
        var_btat__blk1570_dn8 = assign61180_e79249_d_n8;
        var_btat__blk1570_dn9 = assign61180_e79249_d_n9;
        var_btat__blk1570_dn11 = assign61180_e79249_d_n11;
        var_btat__blk1570_dn12 = assign61180_e79249_d_n12;

        let (assign61190_e79266, assign61190_e79266_d_n6, assign61190_e79266_d_n7, assign61190_e79266_d_n8, assign61190_e79266_d_n9, assign61190_e79266_d_n11, assign61190_e79266_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61190_e79262: f64 = (0.666666666666667 * var_atatgat_d);
        let assign61190_e79264: f64 = (assign61190_e79262 / var_btat__blk1570);
        (assign61190_e79264, (-((assign61190_e79262 * var_btat__blk1570_dn6) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign61190_e79262 * var_btat__blk1570_dn7) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign61190_e79262 * var_btat__blk1570_dn8) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign61190_e79262 * var_btat__blk1570_dn9) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign61190_e79262 * var_btat__blk1570_dn11) / (var_btat__blk1570 * var_btat__blk1570))), (-((assign61190_e79262 * var_btat__blk1570_dn12) / (var_btat__blk1570 * var_btat__blk1570))),)
    } else {
        (var_twoatatoverthreebtat__blk1571, var_twoatatoverthreebtat__blk1571_dn6, var_twoatatoverthreebtat__blk1571_dn7, var_twoatatoverthreebtat__blk1571_dn8, var_twoatatoverthreebtat__blk1571_dn9, var_twoatatoverthreebtat__blk1571_dn11, var_twoatatoverthreebtat__blk1571_dn12,)
    }
};
        var_twoatatoverthreebtat__blk1571 = assign61190_e79266;
        var_twoatatoverthreebtat__blk1571_dn6 = assign61190_e79266_d_n6;
        var_twoatatoverthreebtat__blk1571_dn7 = assign61190_e79266_d_n7;
        var_twoatatoverthreebtat__blk1571_dn8 = assign61190_e79266_d_n8;
        var_twoatatoverthreebtat__blk1571_dn9 = assign61190_e79266_d_n9;
        var_twoatatoverthreebtat__blk1571_dn11 = assign61190_e79266_d_n11;
        var_twoatatoverthreebtat__blk1571_dn12 = assign61190_e79266_d_n12;

        let (assign61200_e79281, assign61200_e79281_d_n6, assign61200_e79281_d_n7, assign61200_e79281_d_n8, assign61200_e79281_d_n9, assign61200_e79281_d_n11, assign61200_e79281_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61200_e79279: f64 = (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571);
        (assign61200_e79279, ((var_twoatatoverthreebtat__blk1571_dn6 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn6)), ((var_twoatatoverthreebtat__blk1571_dn7 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn7)), ((var_twoatatoverthreebtat__blk1571_dn8 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn8)), ((var_twoatatoverthreebtat__blk1571_dn9 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn9)), ((var_twoatatoverthreebtat__blk1571_dn11 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn11)), ((var_twoatatoverthreebtat__blk1571_dn12 * var_twoatatoverthreebtat__blk1571) + (var_twoatatoverthreebtat__blk1571 * var_twoatatoverthreebtat__blk1571_dn12)),)
    } else {
        (var_umaxbeforelimiting__blk1572, var_umaxbeforelimiting__blk1572_dn6, var_umaxbeforelimiting__blk1572_dn7, var_umaxbeforelimiting__blk1572_dn8, var_umaxbeforelimiting__blk1572_dn9, var_umaxbeforelimiting__blk1572_dn11, var_umaxbeforelimiting__blk1572_dn12,)
    }
};
        var_umaxbeforelimiting__blk1572 = assign61200_e79281;
        var_umaxbeforelimiting__blk1572_dn6 = assign61200_e79281_d_n6;
        var_umaxbeforelimiting__blk1572_dn7 = assign61200_e79281_d_n7;
        var_umaxbeforelimiting__blk1572_dn8 = assign61200_e79281_d_n8;
        var_umaxbeforelimiting__blk1572_dn9 = assign61200_e79281_d_n9;
        var_umaxbeforelimiting__blk1572_dn11 = assign61200_e79281_d_n11;
        var_umaxbeforelimiting__blk1572_dn12 = assign61200_e79281_d_n12;

        let (assign61210_e79303, assign61210_e79303_d_n6, assign61210_e79303_d_n7, assign61210_e79303_d_n8, assign61210_e79303_d_n9, assign61210_e79303_d_n11, assign61210_e79303_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61210_e79294: f64 = (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572);
        let assign61210_e79297: f64 = (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572);
        let assign61210_e79299: f64 = (assign61210_e79297 + 1.0);
        let assign61210_e79300: f64 = (assign61210_e79294 / assign61210_e79299);
        let assign61210_e79301: f64 = (assign61210_e79300).sqrt();
        (assign61210_e79301, ((((((var_umaxbeforelimiting__blk1572_dn6 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn6)) * assign61210_e79299) - (assign61210_e79294 * ((var_umaxbeforelimiting__blk1572_dn6 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn6)))) / (assign61210_e79299 * assign61210_e79299)) / (2.0 * assign61210_e79301)), ((((((var_umaxbeforelimiting__blk1572_dn7 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn7)) * assign61210_e79299) - (assign61210_e79294 * ((var_umaxbeforelimiting__blk1572_dn7 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn7)))) / (assign61210_e79299 * assign61210_e79299)) / (2.0 * assign61210_e79301)), ((((((var_umaxbeforelimiting__blk1572_dn8 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn8)) * assign61210_e79299) - (assign61210_e79294 * ((var_umaxbeforelimiting__blk1572_dn8 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn8)))) / (assign61210_e79299 * assign61210_e79299)) / (2.0 * assign61210_e79301)), ((((((var_umaxbeforelimiting__blk1572_dn9 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn9)) * assign61210_e79299) - (assign61210_e79294 * ((var_umaxbeforelimiting__blk1572_dn9 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn9)))) / (assign61210_e79299 * assign61210_e79299)) / (2.0 * assign61210_e79301)), ((((((var_umaxbeforelimiting__blk1572_dn11 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn11)) * assign61210_e79299) - (assign61210_e79294 * ((var_umaxbeforelimiting__blk1572_dn11 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn11)))) / (assign61210_e79299 * assign61210_e79299)) / (2.0 * assign61210_e79301)), ((((((var_umaxbeforelimiting__blk1572_dn12 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn12)) * assign61210_e79299) - (assign61210_e79294 * ((var_umaxbeforelimiting__blk1572_dn12 * var_umaxbeforelimiting__blk1572) + (var_umaxbeforelimiting__blk1572 * var_umaxbeforelimiting__blk1572_dn12)))) / (assign61210_e79299 * assign61210_e79299)) / (2.0 * assign61210_e79301)),)
    } else {
        (var_umax__blk1573, var_umax__blk1573_dn6, var_umax__blk1573_dn7, var_umax__blk1573_dn8, var_umax__blk1573_dn9, var_umax__blk1573_dn11, var_umax__blk1573_dn12,)
    }
};
        var_umax__blk1573 = assign61210_e79303;
        var_umax__blk1573_dn6 = assign61210_e79303_d_n6;
        var_umax__blk1573_dn7 = assign61210_e79303_d_n7;
        var_umax__blk1573_dn8 = assign61210_e79303_d_n8;
        var_umax__blk1573_dn9 = assign61210_e79303_d_n9;
        var_umax__blk1573_dn11 = assign61210_e79303_d_n11;
        var_umax__blk1573_dn12 = assign61210_e79303_d_n12;

        *var_asrh__blk1568_slot = var_asrh__blk1568;
        *var_asrh__blk1568_dn11_slot = var_asrh__blk1568_dn11;
        *var_asrh__blk1568_dn12_slot = var_asrh__blk1568_dn12;
        *var_asrh__blk1568_dn6_slot = var_asrh__blk1568_dn6;
        *var_asrh__blk1568_dn7_slot = var_asrh__blk1568_dn7;
        *var_asrh__blk1568_dn8_slot = var_asrh__blk1568_dn8;
        *var_asrh__blk1568_dn9_slot = var_asrh__blk1568_dn9;
        *var_btat__blk1570_slot = var_btat__blk1570;
        *var_btat__blk1570_dn11_slot = var_btat__blk1570_dn11;
        *var_btat__blk1570_dn12_slot = var_btat__blk1570_dn12;
        *var_btat__blk1570_dn6_slot = var_btat__blk1570_dn6;
        *var_btat__blk1570_dn7_slot = var_btat__blk1570_dn7;
        *var_btat__blk1570_dn8_slot = var_btat__blk1570_dn8;
        *var_btat__blk1570_dn9_slot = var_btat__blk1570_dn9;
        *var_dwsrh__blk1565_slot = var_dwsrh__blk1565;
        *var_dwsrh__blk1565_dn11_slot = var_dwsrh__blk1565_dn11;
        *var_dwsrh__blk1565_dn12_slot = var_dwsrh__blk1565_dn12;
        *var_dwsrh__blk1565_dn7_slot = var_dwsrh__blk1565_dn7;
        *var_dwsrh__blk1565_dn8_slot = var_dwsrh__blk1565_dn8;
        *var_fbreakdown__blk1586_slot = var_fbreakdown__blk1586;
        *var_fbreakdown__blk1586_dn11_slot = var_fbreakdown__blk1586_dn11;
        *var_fbreakdown__blk1586_dn12_slot = var_fbreakdown__blk1586_dn12;
        *var_fbreakdown__blk1586_dn6_slot = var_fbreakdown__blk1586_dn6;
        *var_fbreakdown__blk1586_dn7_slot = var_fbreakdown__blk1586_dn7;
        *var_fbreakdown__blk1586_dn8_slot = var_fbreakdown__blk1586_dn8;
        *var_fbreakdown__blk1586_dn9_slot = var_fbreakdown__blk1586_dn9;
        *var_guard1711_slot = var_guard1711;
        *var_guard1712_slot = var_guard1712;
        *var_guard1713_slot = var_guard1713;
        *var_guard1714_slot = var_guard1714;
        *var_guard1715_slot = var_guard1715;
        *var_guard1716_slot = var_guard1716;
        *var_guard1717_slot = var_guard1717;
        *var_guard1718_slot = var_guard1718;
        *var_id__blk1561_slot = var_id__blk1561;
        *var_id__blk1561_dn11_slot = var_id__blk1561_dn11;
        *var_id__blk1561_dn12_slot = var_id__blk1561_dn12;
        *var_id__blk1561_dn7_slot = var_id__blk1561_dn7;
        *var_id__blk1561_dn8_slot = var_id__blk1561_dn8;
        *var_ijungat_d_slot = var_ijungat_d;
        *var_ijungat_d_dn11_slot = var_ijungat_d_dn11;
        *var_ijungat_d_dn12_slot = var_ijungat_d_dn12;
        *var_ijungat_d_dn6_slot = var_ijungat_d_dn6;
        *var_ijungat_d_dn7_slot = var_ijungat_d_dn7;
        *var_ijungat_d_dn8_slot = var_ijungat_d_dn8;
        *var_ijungat_d_dn9_slot = var_ijungat_d_dn9;
        *var_ijunsti_d_slot = var_ijunsti_d;
        *var_ijunsti_d_dn11_slot = var_ijunsti_d_dn11;
        *var_ijunsti_d_dn12_slot = var_ijunsti_d_dn12;
        *var_ijunsti_d_dn6_slot = var_ijunsti_d_dn6;
        *var_ijunsti_d_dn7_slot = var_ijunsti_d_dn7;
        *var_ijunsti_d_dn8_slot = var_ijunsti_d_dn8;
        *var_ijunsti_d_dn9_slot = var_ijunsti_d_dn9;
        *var_isrh__blk1562_slot = var_isrh__blk1562;
        *var_isrh__blk1562_dn11_slot = var_isrh__blk1562_dn11;
        *var_isrh__blk1562_dn12_slot = var_isrh__blk1562_dn12;
        *var_isrh__blk1562_dn6_slot = var_isrh__blk1562_dn6;
        *var_isrh__blk1562_dn7_slot = var_isrh__blk1562_dn7;
        *var_isrh__blk1562_dn8_slot = var_isrh__blk1562_dn8;
        *var_isrh__blk1562_dn9_slot = var_isrh__blk1562_dn9;
        *var_itat__blk1569_slot = var_itat__blk1569;
        *var_itat__blk1569_dn11_slot = var_itat__blk1569_dn11;
        *var_itat__blk1569_dn12_slot = var_itat__blk1569_dn12;
        *var_itat__blk1569_dn6_slot = var_itat__blk1569_dn6;
        *var_itat__blk1569_dn7_slot = var_itat__blk1569_dn7;
        *var_itat__blk1569_dn8_slot = var_itat__blk1569_dn8;
        *var_itat__blk1569_dn9_slot = var_itat__blk1569_dn9;
        *var_qjungat_d_slot = var_qjungat_d;
        *var_qjungat_d_dn11_slot = var_qjungat_d_dn11;
        *var_qjungat_d_dn12_slot = var_qjungat_d_dn12;
        *var_qjungat_d_dn6_slot = var_qjungat_d_dn6;
        *var_qjungat_d_dn7_slot = var_qjungat_d_dn7;
        *var_qjungat_d_dn8_slot = var_qjungat_d_dn8;
        *var_qjungat_d_dn9_slot = var_qjungat_d_dn9;
        *var_qjunsti_d_slot = var_qjunsti_d;
        *var_qjunsti_d_dn11_slot = var_qjunsti_d_dn11;
        *var_qjunsti_d_dn12_slot = var_qjunsti_d_dn12;
        *var_qjunsti_d_dn6_slot = var_qjunsti_d_dn6;
        *var_qjunsti_d_dn7_slot = var_qjunsti_d_dn7;
        *var_qjunsti_d_dn8_slot = var_qjunsti_d_dn8;
        *var_qjunsti_d_dn9_slot = var_qjunsti_d_dn9;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_twoatatoverthreebtat__blk1571_slot = var_twoatatoverthreebtat__blk1571;
        *var_twoatatoverthreebtat__blk1571_dn11_slot = var_twoatatoverthreebtat__blk1571_dn11;
        *var_twoatatoverthreebtat__blk1571_dn12_slot = var_twoatatoverthreebtat__blk1571_dn12;
        *var_twoatatoverthreebtat__blk1571_dn6_slot = var_twoatatoverthreebtat__blk1571_dn6;
        *var_twoatatoverthreebtat__blk1571_dn7_slot = var_twoatatoverthreebtat__blk1571_dn7;
        *var_twoatatoverthreebtat__blk1571_dn8_slot = var_twoatatoverthreebtat__blk1571_dn8;
        *var_twoatatoverthreebtat__blk1571_dn9_slot = var_twoatatoverthreebtat__blk1571_dn9;
        *var_umax__blk1573_slot = var_umax__blk1573;
        *var_umax__blk1573_dn11_slot = var_umax__blk1573_dn11;
        *var_umax__blk1573_dn12_slot = var_umax__blk1573_dn12;
        *var_umax__blk1573_dn6_slot = var_umax__blk1573_dn6;
        *var_umax__blk1573_dn7_slot = var_umax__blk1573_dn7;
        *var_umax__blk1573_dn8_slot = var_umax__blk1573_dn8;
        *var_umax__blk1573_dn9_slot = var_umax__blk1573_dn9;
        *var_umaxbeforelimiting__blk1572_slot = var_umaxbeforelimiting__blk1572;
        *var_umaxbeforelimiting__blk1572_dn11_slot = var_umaxbeforelimiting__blk1572_dn11;
        *var_umaxbeforelimiting__blk1572_dn12_slot = var_umaxbeforelimiting__blk1572_dn12;
        *var_umaxbeforelimiting__blk1572_dn6_slot = var_umaxbeforelimiting__blk1572_dn6;
        *var_umaxbeforelimiting__blk1572_dn7_slot = var_umaxbeforelimiting__blk1572_dn7;
        *var_umaxbeforelimiting__blk1572_dn8_slot = var_umaxbeforelimiting__blk1572_dn8;
        *var_umaxbeforelimiting__blk1572_dn9_slot = var_umaxbeforelimiting__blk1572_dn9;
        *var_vbi_minus_vjsrh__blk1563_slot = var_vbi_minus_vjsrh__blk1563;
        *var_vbi_minus_vjsrh__blk1563_dn11_slot = var_vbi_minus_vjsrh__blk1563_dn11;
        *var_vbi_minus_vjsrh__blk1563_dn12_slot = var_vbi_minus_vjsrh__blk1563_dn12;
        *var_vbi_minus_vjsrh__blk1563_dn7_slot = var_vbi_minus_vjsrh__blk1563_dn7;
        *var_vbi_minus_vjsrh__blk1563_dn8_slot = var_vbi_minus_vjsrh__blk1563_dn8;
        *var_wdep__blk1567_slot = var_wdep__blk1567;
        *var_wdep__blk1567_dn11_slot = var_wdep__blk1567_dn11;
        *var_wdep__blk1567_dn12_slot = var_wdep__blk1567_dn12;
        *var_wdep__blk1567_dn6_slot = var_wdep__blk1567_dn6;
        *var_wdep__blk1567_dn7_slot = var_wdep__blk1567_dn7;
        *var_wdep__blk1567_dn8_slot = var_wdep__blk1567_dn8;
        *var_wdep__blk1567_dn9_slot = var_wdep__blk1567_dn9;
        *var_wsrh__blk1566_slot = var_wsrh__blk1566;
        *var_wsrh__blk1566_dn11_slot = var_wsrh__blk1566_dn11;
        *var_wsrh__blk1566_dn12_slot = var_wsrh__blk1566_dn12;
        *var_wsrh__blk1566_dn7_slot = var_wsrh__blk1566_dn7;
        *var_wsrh__blk1566_dn8_slot = var_wsrh__blk1566_dn8;
        *var_wsrhstep__blk1564_slot = var_wsrhstep__blk1564;
        *var_wsrhstep__blk1564_dn11_slot = var_wsrhstep__blk1564_dn11;
        *var_wsrhstep__blk1564_dn12_slot = var_wsrhstep__blk1564_dn12;
        *var_wsrhstep__blk1564_dn7_slot = var_wsrhstep__blk1564_dn7;
        *var_wsrhstep__blk1564_dn8_slot = var_wsrhstep__blk1564_dn8;
    }

    pub(super) fn stamp_transient_block_145(
        var_asrh__blk1568: f64,
        var_asrh__blk1568_dn11: f64,
        var_asrh__blk1568_dn12: f64,
        var_asrh__blk1568_dn6: f64,
        var_asrh__blk1568_dn7: f64,
        var_asrh__blk1568_dn8: f64,
        var_asrh__blk1568_dn9: f64,
        var_atatgat_d: f64,
        var_berfc: f64,
        var_btat__blk1570: f64,
        var_btat__blk1570_dn11: f64,
        var_btat__blk1570_dn12: f64,
        var_btat__blk1570_dn6: f64,
        var_btat__blk1570_dn7: f64,
        var_btat__blk1570_dn8: f64,
        var_btat__blk1570_dn9: f64,
        var_cbbtgatd_i: f64,
        var_cerfc: f64,
        var_ctatgatd_i: f64,
        var_fbbtgat_d: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1714: f64,
        var_guard1718: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_perfc: f64,
        var_pgatd_i: f64,
        var_twoatatoverthreebtat__blk1571: f64,
        var_twoatatoverthreebtat__blk1571_dn11: f64,
        var_twoatatoverthreebtat__blk1571_dn12: f64,
        var_twoatatoverthreebtat__blk1571_dn6: f64,
        var_twoatatoverthreebtat__blk1571_dn7: f64,
        var_twoatatoverthreebtat__blk1571_dn8: f64,
        var_twoatatoverthreebtat__blk1571_dn9: f64,
        var_umax__blk1573: f64,
        var_umax__blk1573_dn11: f64,
        var_umax__blk1573_dn12: f64,
        var_umax__blk1573_dn6: f64,
        var_umax__blk1573_dn7: f64,
        var_umax__blk1573_dn8: f64,
        var_umax__blk1573_dn9: f64,
        var_vbbt__blk1558: f64,
        var_vbbt__blk1558_dn11: f64,
        var_vbbt__blk1558_dn12: f64,
        var_vbbt__blk1558_dn7: f64,
        var_vbbt__blk1558_dn8: f64,
        var_vbirgatd_i: f64,
        var_vbirgatinv_d: f64,
        var_wdepnulrinvgat_d: f64,
        var_wsrh__blk1566: f64,
        var_wsrh__blk1566_dn11: f64,
        var_wsrh__blk1566_dn12: f64,
        var_wsrh__blk1566_dn7: f64,
        var_wsrh__blk1566_dn8: f64,
        var_erfcpos__blk1544_slot: &mut f64,
        var_erfcpos__blk1544_dn11_slot: &mut f64,
        var_erfcpos__blk1544_dn12_slot: &mut f64,
        var_erfcpos__blk1544_dn6_slot: &mut f64,
        var_erfcpos__blk1544_dn7_slot: &mut f64,
        var_erfcpos__blk1544_dn8_slot: &mut f64,
        var_erfcpos__blk1544_dn9_slot: &mut f64,
        var_erfctimesexpmtat__blk1582_slot: &mut f64,
        var_erfctimesexpmtat__blk1582_dn11_slot: &mut f64,
        var_erfctimesexpmtat__blk1582_dn12_slot: &mut f64,
        var_erfctimesexpmtat__blk1582_dn6_slot: &mut f64,
        var_erfctimesexpmtat__blk1582_dn7_slot: &mut f64,
        var_erfctimesexpmtat__blk1582_dn8_slot: &mut f64,
        var_erfctimesexpmtat__blk1582_dn9_slot: &mut f64,
        var_fmaxr__blk1585_slot: &mut f64,
        var_fmaxr__blk1585_dn11_slot: &mut f64,
        var_fmaxr__blk1585_dn12_slot: &mut f64,
        var_fmaxr__blk1585_dn6_slot: &mut f64,
        var_fmaxr__blk1585_dn7_slot: &mut f64,
        var_fmaxr__blk1585_dn8_slot: &mut f64,
        var_fmaxr__blk1585_dn9_slot: &mut f64,
        var_gammamax__blk1583_slot: &mut f64,
        var_gammamax__blk1583_dn11_slot: &mut f64,
        var_gammamax__blk1583_dn12_slot: &mut f64,
        var_gammamax__blk1583_dn6_slot: &mut f64,
        var_gammamax__blk1583_dn7_slot: &mut f64,
        var_gammamax__blk1583_dn8_slot: &mut f64,
        var_gammamax__blk1583_dn9_slot: &mut f64,
        var_guard1719_slot: &mut f64,
        var_guard1720_slot: &mut f64,
        var_guard1721_slot: &mut f64,
        var_guard1722_slot: &mut f64,
        var_guard1723_slot: &mut f64,
        var_guard1724_slot: &mut f64,
        var_guard1725_slot: &mut f64,
        var_guard1726_slot: &mut f64,
        var_ibbt__blk1584_slot: &mut f64,
        var_ibbt__blk1584_dn11_slot: &mut f64,
        var_ibbt__blk1584_dn12_slot: &mut f64,
        var_ibbt__blk1584_dn6_slot: &mut f64,
        var_ibbt__blk1584_dn7_slot: &mut f64,
        var_ibbt__blk1584_dn8_slot: &mut f64,
        var_ibbt__blk1584_dn9_slot: &mut f64,
        var_itat__blk1569_slot: &mut f64,
        var_itat__blk1569_dn11_slot: &mut f64,
        var_itat__blk1569_dn12_slot: &mut f64,
        var_itat__blk1569_dn6_slot: &mut f64,
        var_itat__blk1569_dn7_slot: &mut f64,
        var_itat__blk1569_dn8_slot: &mut f64,
        var_itat__blk1569_dn9_slot: &mut f64,
        var_ktat__blk1578_slot: &mut f64,
        var_ktat__blk1578_dn11_slot: &mut f64,
        var_ktat__blk1578_dn12_slot: &mut f64,
        var_ktat__blk1578_dn6_slot: &mut f64,
        var_ktat__blk1578_dn7_slot: &mut f64,
        var_ktat__blk1578_dn8_slot: &mut f64,
        var_ktat__blk1578_dn9_slot: &mut f64,
        var_ltat__blk1579_slot: &mut f64,
        var_ltat__blk1579_dn11_slot: &mut f64,
        var_ltat__blk1579_dn12_slot: &mut f64,
        var_ltat__blk1579_dn6_slot: &mut f64,
        var_ltat__blk1579_dn7_slot: &mut f64,
        var_ltat__blk1579_dn8_slot: &mut f64,
        var_ltat__blk1579_dn9_slot: &mut f64,
        var_mtat__blk1580_slot: &mut f64,
        var_mtat__blk1580_dn11_slot: &mut f64,
        var_mtat__blk1580_dn12_slot: &mut f64,
        var_mtat__blk1580_dn6_slot: &mut f64,
        var_mtat__blk1580_dn7_slot: &mut f64,
        var_mtat__blk1580_dn8_slot: &mut f64,
        var_mtat__blk1580_dn9_slot: &mut f64,
        var_sqrtumax__blk1574_slot: &mut f64,
        var_sqrtumax__blk1574_dn11_slot: &mut f64,
        var_sqrtumax__blk1574_dn12_slot: &mut f64,
        var_sqrtumax__blk1574_dn6_slot: &mut f64,
        var_sqrtumax__blk1574_dn7_slot: &mut f64,
        var_sqrtumax__blk1574_dn8_slot: &mut f64,
        var_sqrtumax__blk1574_dn9_slot: &mut f64,
        var_terfc__blk1543_slot: &mut f64,
        var_terfc__blk1543_dn11_slot: &mut f64,
        var_terfc__blk1543_dn12_slot: &mut f64,
        var_terfc__blk1543_dn6_slot: &mut f64,
        var_terfc__blk1543_dn7_slot: &mut f64,
        var_terfc__blk1543_dn8_slot: &mut f64,
        var_terfc__blk1543_dn9_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn11_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn12_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn6_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn7_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn8_slot: &mut f64,
        var_umaxpoweronepointfive__blk1575_dn9_slot: &mut f64,
        var_wgamma__blk1576_slot: &mut f64,
        var_wgamma__blk1576_dn11_slot: &mut f64,
        var_wgamma__blk1576_dn12_slot: &mut f64,
        var_wgamma__blk1576_dn6_slot: &mut f64,
        var_wgamma__blk1576_dn7_slot: &mut f64,
        var_wgamma__blk1576_dn8_slot: &mut f64,
        var_wgamma__blk1576_dn9_slot: &mut f64,
        var_wtat__blk1577_slot: &mut f64,
        var_wtat__blk1577_dn11_slot: &mut f64,
        var_wtat__blk1577_dn12_slot: &mut f64,
        var_wtat__blk1577_dn6_slot: &mut f64,
        var_wtat__blk1577_dn7_slot: &mut f64,
        var_wtat__blk1577_dn8_slot: &mut f64,
        var_wtat__blk1577_dn9_slot: &mut f64,
        var_xerfc__blk1581_slot: &mut f64,
        var_xerfc__blk1581_dn11_slot: &mut f64,
        var_xerfc__blk1581_dn12_slot: &mut f64,
        var_xerfc__blk1581_dn6_slot: &mut f64,
        var_xerfc__blk1581_dn7_slot: &mut f64,
        var_xerfc__blk1581_dn8_slot: &mut f64,
        var_xerfc__blk1581_dn9_slot: &mut f64,
        var_ysq__blk1542_slot: &mut f64,
        var_ysq__blk1542_dn11_slot: &mut f64,
        var_ysq__blk1542_dn12_slot: &mut f64,
        var_ysq__blk1542_dn6_slot: &mut f64,
        var_ysq__blk1542_dn7_slot: &mut f64,
        var_ysq__blk1542_dn8_slot: &mut f64,
        var_ysq__blk1542_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos__blk1544: f64 = *var_erfcpos__blk1544_slot;
        let mut var_erfcpos__blk1544_dn11: f64 = *var_erfcpos__blk1544_dn11_slot;
        let mut var_erfcpos__blk1544_dn12: f64 = *var_erfcpos__blk1544_dn12_slot;
        let mut var_erfcpos__blk1544_dn6: f64 = *var_erfcpos__blk1544_dn6_slot;
        let mut var_erfcpos__blk1544_dn7: f64 = *var_erfcpos__blk1544_dn7_slot;
        let mut var_erfcpos__blk1544_dn8: f64 = *var_erfcpos__blk1544_dn8_slot;
        let mut var_erfcpos__blk1544_dn9: f64 = *var_erfcpos__blk1544_dn9_slot;
        let mut var_erfctimesexpmtat__blk1582: f64 = *var_erfctimesexpmtat__blk1582_slot;
        let mut var_erfctimesexpmtat__blk1582_dn11: f64 = *var_erfctimesexpmtat__blk1582_dn11_slot;
        let mut var_erfctimesexpmtat__blk1582_dn12: f64 = *var_erfctimesexpmtat__blk1582_dn12_slot;
        let mut var_erfctimesexpmtat__blk1582_dn6: f64 = *var_erfctimesexpmtat__blk1582_dn6_slot;
        let mut var_erfctimesexpmtat__blk1582_dn7: f64 = *var_erfctimesexpmtat__blk1582_dn7_slot;
        let mut var_erfctimesexpmtat__blk1582_dn8: f64 = *var_erfctimesexpmtat__blk1582_dn8_slot;
        let mut var_erfctimesexpmtat__blk1582_dn9: f64 = *var_erfctimesexpmtat__blk1582_dn9_slot;
        let mut var_fmaxr__blk1585: f64 = *var_fmaxr__blk1585_slot;
        let mut var_fmaxr__blk1585_dn11: f64 = *var_fmaxr__blk1585_dn11_slot;
        let mut var_fmaxr__blk1585_dn12: f64 = *var_fmaxr__blk1585_dn12_slot;
        let mut var_fmaxr__blk1585_dn6: f64 = *var_fmaxr__blk1585_dn6_slot;
        let mut var_fmaxr__blk1585_dn7: f64 = *var_fmaxr__blk1585_dn7_slot;
        let mut var_fmaxr__blk1585_dn8: f64 = *var_fmaxr__blk1585_dn8_slot;
        let mut var_fmaxr__blk1585_dn9: f64 = *var_fmaxr__blk1585_dn9_slot;
        let mut var_gammamax__blk1583: f64 = *var_gammamax__blk1583_slot;
        let mut var_gammamax__blk1583_dn11: f64 = *var_gammamax__blk1583_dn11_slot;
        let mut var_gammamax__blk1583_dn12: f64 = *var_gammamax__blk1583_dn12_slot;
        let mut var_gammamax__blk1583_dn6: f64 = *var_gammamax__blk1583_dn6_slot;
        let mut var_gammamax__blk1583_dn7: f64 = *var_gammamax__blk1583_dn7_slot;
        let mut var_gammamax__blk1583_dn8: f64 = *var_gammamax__blk1583_dn8_slot;
        let mut var_gammamax__blk1583_dn9: f64 = *var_gammamax__blk1583_dn9_slot;
        let mut var_guard1719: f64 = *var_guard1719_slot;
        let mut var_guard1720: f64 = *var_guard1720_slot;
        let mut var_guard1721: f64 = *var_guard1721_slot;
        let mut var_guard1722: f64 = *var_guard1722_slot;
        let mut var_guard1723: f64 = *var_guard1723_slot;
        let mut var_guard1724: f64 = *var_guard1724_slot;
        let mut var_guard1725: f64 = *var_guard1725_slot;
        let mut var_guard1726: f64 = *var_guard1726_slot;
        let mut var_ibbt__blk1584: f64 = *var_ibbt__blk1584_slot;
        let mut var_ibbt__blk1584_dn11: f64 = *var_ibbt__blk1584_dn11_slot;
        let mut var_ibbt__blk1584_dn12: f64 = *var_ibbt__blk1584_dn12_slot;
        let mut var_ibbt__blk1584_dn6: f64 = *var_ibbt__blk1584_dn6_slot;
        let mut var_ibbt__blk1584_dn7: f64 = *var_ibbt__blk1584_dn7_slot;
        let mut var_ibbt__blk1584_dn8: f64 = *var_ibbt__blk1584_dn8_slot;
        let mut var_ibbt__blk1584_dn9: f64 = *var_ibbt__blk1584_dn9_slot;
        let mut var_itat__blk1569: f64 = *var_itat__blk1569_slot;
        let mut var_itat__blk1569_dn11: f64 = *var_itat__blk1569_dn11_slot;
        let mut var_itat__blk1569_dn12: f64 = *var_itat__blk1569_dn12_slot;
        let mut var_itat__blk1569_dn6: f64 = *var_itat__blk1569_dn6_slot;
        let mut var_itat__blk1569_dn7: f64 = *var_itat__blk1569_dn7_slot;
        let mut var_itat__blk1569_dn8: f64 = *var_itat__blk1569_dn8_slot;
        let mut var_itat__blk1569_dn9: f64 = *var_itat__blk1569_dn9_slot;
        let mut var_ktat__blk1578: f64 = *var_ktat__blk1578_slot;
        let mut var_ktat__blk1578_dn11: f64 = *var_ktat__blk1578_dn11_slot;
        let mut var_ktat__blk1578_dn12: f64 = *var_ktat__blk1578_dn12_slot;
        let mut var_ktat__blk1578_dn6: f64 = *var_ktat__blk1578_dn6_slot;
        let mut var_ktat__blk1578_dn7: f64 = *var_ktat__blk1578_dn7_slot;
        let mut var_ktat__blk1578_dn8: f64 = *var_ktat__blk1578_dn8_slot;
        let mut var_ktat__blk1578_dn9: f64 = *var_ktat__blk1578_dn9_slot;
        let mut var_ltat__blk1579: f64 = *var_ltat__blk1579_slot;
        let mut var_ltat__blk1579_dn11: f64 = *var_ltat__blk1579_dn11_slot;
        let mut var_ltat__blk1579_dn12: f64 = *var_ltat__blk1579_dn12_slot;
        let mut var_ltat__blk1579_dn6: f64 = *var_ltat__blk1579_dn6_slot;
        let mut var_ltat__blk1579_dn7: f64 = *var_ltat__blk1579_dn7_slot;
        let mut var_ltat__blk1579_dn8: f64 = *var_ltat__blk1579_dn8_slot;
        let mut var_ltat__blk1579_dn9: f64 = *var_ltat__blk1579_dn9_slot;
        let mut var_mtat__blk1580: f64 = *var_mtat__blk1580_slot;
        let mut var_mtat__blk1580_dn11: f64 = *var_mtat__blk1580_dn11_slot;
        let mut var_mtat__blk1580_dn12: f64 = *var_mtat__blk1580_dn12_slot;
        let mut var_mtat__blk1580_dn6: f64 = *var_mtat__blk1580_dn6_slot;
        let mut var_mtat__blk1580_dn7: f64 = *var_mtat__blk1580_dn7_slot;
        let mut var_mtat__blk1580_dn8: f64 = *var_mtat__blk1580_dn8_slot;
        let mut var_mtat__blk1580_dn9: f64 = *var_mtat__blk1580_dn9_slot;
        let mut var_sqrtumax__blk1574: f64 = *var_sqrtumax__blk1574_slot;
        let mut var_sqrtumax__blk1574_dn11: f64 = *var_sqrtumax__blk1574_dn11_slot;
        let mut var_sqrtumax__blk1574_dn12: f64 = *var_sqrtumax__blk1574_dn12_slot;
        let mut var_sqrtumax__blk1574_dn6: f64 = *var_sqrtumax__blk1574_dn6_slot;
        let mut var_sqrtumax__blk1574_dn7: f64 = *var_sqrtumax__blk1574_dn7_slot;
        let mut var_sqrtumax__blk1574_dn8: f64 = *var_sqrtumax__blk1574_dn8_slot;
        let mut var_sqrtumax__blk1574_dn9: f64 = *var_sqrtumax__blk1574_dn9_slot;
        let mut var_terfc__blk1543: f64 = *var_terfc__blk1543_slot;
        let mut var_terfc__blk1543_dn11: f64 = *var_terfc__blk1543_dn11_slot;
        let mut var_terfc__blk1543_dn12: f64 = *var_terfc__blk1543_dn12_slot;
        let mut var_terfc__blk1543_dn6: f64 = *var_terfc__blk1543_dn6_slot;
        let mut var_terfc__blk1543_dn7: f64 = *var_terfc__blk1543_dn7_slot;
        let mut var_terfc__blk1543_dn8: f64 = *var_terfc__blk1543_dn8_slot;
        let mut var_terfc__blk1543_dn9: f64 = *var_terfc__blk1543_dn9_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_umaxpoweronepointfive__blk1575: f64 = *var_umaxpoweronepointfive__blk1575_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn11: f64 = *var_umaxpoweronepointfive__blk1575_dn11_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn12: f64 = *var_umaxpoweronepointfive__blk1575_dn12_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn6: f64 = *var_umaxpoweronepointfive__blk1575_dn6_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn7: f64 = *var_umaxpoweronepointfive__blk1575_dn7_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn8: f64 = *var_umaxpoweronepointfive__blk1575_dn8_slot;
        let mut var_umaxpoweronepointfive__blk1575_dn9: f64 = *var_umaxpoweronepointfive__blk1575_dn9_slot;
        let mut var_wgamma__blk1576: f64 = *var_wgamma__blk1576_slot;
        let mut var_wgamma__blk1576_dn11: f64 = *var_wgamma__blk1576_dn11_slot;
        let mut var_wgamma__blk1576_dn12: f64 = *var_wgamma__blk1576_dn12_slot;
        let mut var_wgamma__blk1576_dn6: f64 = *var_wgamma__blk1576_dn6_slot;
        let mut var_wgamma__blk1576_dn7: f64 = *var_wgamma__blk1576_dn7_slot;
        let mut var_wgamma__blk1576_dn8: f64 = *var_wgamma__blk1576_dn8_slot;
        let mut var_wgamma__blk1576_dn9: f64 = *var_wgamma__blk1576_dn9_slot;
        let mut var_wtat__blk1577: f64 = *var_wtat__blk1577_slot;
        let mut var_wtat__blk1577_dn11: f64 = *var_wtat__blk1577_dn11_slot;
        let mut var_wtat__blk1577_dn12: f64 = *var_wtat__blk1577_dn12_slot;
        let mut var_wtat__blk1577_dn6: f64 = *var_wtat__blk1577_dn6_slot;
        let mut var_wtat__blk1577_dn7: f64 = *var_wtat__blk1577_dn7_slot;
        let mut var_wtat__blk1577_dn8: f64 = *var_wtat__blk1577_dn8_slot;
        let mut var_wtat__blk1577_dn9: f64 = *var_wtat__blk1577_dn9_slot;
        let mut var_xerfc__blk1581: f64 = *var_xerfc__blk1581_slot;
        let mut var_xerfc__blk1581_dn11: f64 = *var_xerfc__blk1581_dn11_slot;
        let mut var_xerfc__blk1581_dn12: f64 = *var_xerfc__blk1581_dn12_slot;
        let mut var_xerfc__blk1581_dn6: f64 = *var_xerfc__blk1581_dn6_slot;
        let mut var_xerfc__blk1581_dn7: f64 = *var_xerfc__blk1581_dn7_slot;
        let mut var_xerfc__blk1581_dn8: f64 = *var_xerfc__blk1581_dn8_slot;
        let mut var_xerfc__blk1581_dn9: f64 = *var_xerfc__blk1581_dn9_slot;
        let mut var_ysq__blk1542: f64 = *var_ysq__blk1542_slot;
        let mut var_ysq__blk1542_dn11: f64 = *var_ysq__blk1542_dn11_slot;
        let mut var_ysq__blk1542_dn12: f64 = *var_ysq__blk1542_dn12_slot;
        let mut var_ysq__blk1542_dn6: f64 = *var_ysq__blk1542_dn6_slot;
        let mut var_ysq__blk1542_dn7: f64 = *var_ysq__blk1542_dn7_slot;
        let mut var_ysq__blk1542_dn8: f64 = *var_ysq__blk1542_dn8_slot;
        let mut var_ysq__blk1542_dn9: f64 = *var_ysq__blk1542_dn9_slot;

        let (assign61220_e79317, assign61220_e79317_d_n6, assign61220_e79317_d_n7, assign61220_e79317_d_n8, assign61220_e79317_d_n9, assign61220_e79317_d_n11, assign61220_e79317_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61220_e79315: f64 = (var_umax__blk1573).sqrt();
        (assign61220_e79315, (var_umax__blk1573_dn6 / (2.0 * assign61220_e79315)), (var_umax__blk1573_dn7 / (2.0 * assign61220_e79315)), (var_umax__blk1573_dn8 / (2.0 * assign61220_e79315)), (var_umax__blk1573_dn9 / (2.0 * assign61220_e79315)), (var_umax__blk1573_dn11 / (2.0 * assign61220_e79315)), (var_umax__blk1573_dn12 / (2.0 * assign61220_e79315)),)
    } else {
        (var_sqrtumax__blk1574, var_sqrtumax__blk1574_dn6, var_sqrtumax__blk1574_dn7, var_sqrtumax__blk1574_dn8, var_sqrtumax__blk1574_dn9, var_sqrtumax__blk1574_dn11, var_sqrtumax__blk1574_dn12,)
    }
};
        var_sqrtumax__blk1574 = assign61220_e79317;
        var_sqrtumax__blk1574_dn6 = assign61220_e79317_d_n6;
        var_sqrtumax__blk1574_dn7 = assign61220_e79317_d_n7;
        var_sqrtumax__blk1574_dn8 = assign61220_e79317_d_n8;
        var_sqrtumax__blk1574_dn9 = assign61220_e79317_d_n9;
        var_sqrtumax__blk1574_dn11 = assign61220_e79317_d_n11;
        var_sqrtumax__blk1574_dn12 = assign61220_e79317_d_n12;

        let (assign61230_e79332, assign61230_e79332_d_n6, assign61230_e79332_d_n7, assign61230_e79332_d_n8, assign61230_e79332_d_n9, assign61230_e79332_d_n11, assign61230_e79332_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61230_e79330: f64 = (var_umax__blk1573 * var_sqrtumax__blk1574);
        (assign61230_e79330, ((var_umax__blk1573_dn6 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn6)), ((var_umax__blk1573_dn7 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn7)), ((var_umax__blk1573_dn8 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn8)), ((var_umax__blk1573_dn9 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn9)), ((var_umax__blk1573_dn11 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn11)), ((var_umax__blk1573_dn12 * var_sqrtumax__blk1574) + (var_umax__blk1573 * var_sqrtumax__blk1574_dn12)),)
    } else {
        (var_umaxpoweronepointfive__blk1575, var_umaxpoweronepointfive__blk1575_dn6, var_umaxpoweronepointfive__blk1575_dn7, var_umaxpoweronepointfive__blk1575_dn8, var_umaxpoweronepointfive__blk1575_dn9, var_umaxpoweronepointfive__blk1575_dn11, var_umaxpoweronepointfive__blk1575_dn12,)
    }
};
        var_umaxpoweronepointfive__blk1575 = assign61230_e79332;
        var_umaxpoweronepointfive__blk1575_dn6 = assign61230_e79332_d_n6;
        var_umaxpoweronepointfive__blk1575_dn7 = assign61230_e79332_d_n7;
        var_umaxpoweronepointfive__blk1575_dn8 = assign61230_e79332_d_n8;
        var_umaxpoweronepointfive__blk1575_dn9 = assign61230_e79332_d_n9;
        var_umaxpoweronepointfive__blk1575_dn11 = assign61230_e79332_d_n11;
        var_umaxpoweronepointfive__blk1575_dn12 = assign61230_e79332_d_n12;

        let assign61240_e79334: f64 = (-var_pgatd_i);
        let assign61240_e79336: f64 = (assign61240_e79334 * var_one_over_one_minus_pgat_d);
        let assign61240_e79338: f64 = (-1.0);
        let assign61240_e79339: f64 = if assign61240_e79336 == assign61240_e79338 { 1.0 } else { 0.0 };
        var_guard1719 = assign61240_e79339;

        let (assign61250_e79360, assign61250_e79360_d_n6, assign61250_e79360_d_n7, assign61250_e79360_d_n8, assign61250_e79360_d_n9, assign61250_e79360_d_n11, assign61250_e79360_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1719 != 0.0)) {
        let assign61250_e79356: f64 = (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575);
        let assign61250_e79357: f64 = (1.0 + assign61250_e79356);
        let assign61250_e79358: f64 = (1.0 / assign61250_e79357);
        (assign61250_e79358, (-(((var_btat__blk1570_dn6 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn6)) / (assign61250_e79357 * assign61250_e79357))), (-(((var_btat__blk1570_dn7 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn7)) / (assign61250_e79357 * assign61250_e79357))), (-(((var_btat__blk1570_dn8 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn8)) / (assign61250_e79357 * assign61250_e79357))), (-(((var_btat__blk1570_dn9 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn9)) / (assign61250_e79357 * assign61250_e79357))), (-(((var_btat__blk1570_dn11 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn11)) / (assign61250_e79357 * assign61250_e79357))), (-(((var_btat__blk1570_dn12 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn12)) / (assign61250_e79357 * assign61250_e79357))),)
    } else {
        (var_wgamma__blk1576, var_wgamma__blk1576_dn6, var_wgamma__blk1576_dn7, var_wgamma__blk1576_dn8, var_wgamma__blk1576_dn9, var_wgamma__blk1576_dn11, var_wgamma__blk1576_dn12,)
    }
};
        var_wgamma__blk1576 = assign61250_e79360;
        var_wgamma__blk1576_dn6 = assign61250_e79360_d_n6;
        var_wgamma__blk1576_dn7 = assign61250_e79360_d_n7;
        var_wgamma__blk1576_dn8 = assign61250_e79360_d_n8;
        var_wgamma__blk1576_dn9 = assign61250_e79360_d_n9;
        var_wgamma__blk1576_dn11 = assign61250_e79360_d_n11;
        var_wgamma__blk1576_dn12 = assign61250_e79360_d_n12;

        let (assign61260_e79385, assign61260_e79385_d_n6, assign61260_e79385_d_n7, assign61260_e79385_d_n8, assign61260_e79385_d_n9, assign61260_e79385_d_n11, assign61260_e79385_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1719 == 0.0)) {
        let assign61260_e79377: f64 = (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575);
        let assign61260_e79378: f64 = (1.0 + assign61260_e79377);
        let assign61260_e79380: f64 = (-var_pgatd_i);
        let assign61260_e79382: f64 = (assign61260_e79380 * var_one_over_one_minus_pgat_d);
        let assign61260_e79383: f64 = (assign61260_e79378).powf(assign61260_e79382);
        (assign61260_e79383, if 0.0 == 0.0 && ((assign61260_e79382) as f64).is_finite() && ((assign61260_e79382) as f64).fract() == 0.0 { if assign61260_e79382 == 0.0 { 0.0 } else { (assign61260_e79382 * ((assign61260_e79378).powf(assign61260_e79382 - 1.0) * ((var_btat__blk1570_dn6 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn6)))) } } else { (assign61260_e79383 * (assign61260_e79382 * (((var_btat__blk1570_dn6 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn6)) / assign61260_e79378))) }, if 0.0 == 0.0 && ((assign61260_e79382) as f64).is_finite() && ((assign61260_e79382) as f64).fract() == 0.0 { if assign61260_e79382 == 0.0 { 0.0 } else { (assign61260_e79382 * ((assign61260_e79378).powf(assign61260_e79382 - 1.0) * ((var_btat__blk1570_dn7 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn7)))) } } else { (assign61260_e79383 * (assign61260_e79382 * (((var_btat__blk1570_dn7 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn7)) / assign61260_e79378))) }, if 0.0 == 0.0 && ((assign61260_e79382) as f64).is_finite() && ((assign61260_e79382) as f64).fract() == 0.0 { if assign61260_e79382 == 0.0 { 0.0 } else { (assign61260_e79382 * ((assign61260_e79378).powf(assign61260_e79382 - 1.0) * ((var_btat__blk1570_dn8 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn8)))) } } else { (assign61260_e79383 * (assign61260_e79382 * (((var_btat__blk1570_dn8 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn8)) / assign61260_e79378))) }, if 0.0 == 0.0 && ((assign61260_e79382) as f64).is_finite() && ((assign61260_e79382) as f64).fract() == 0.0 { if assign61260_e79382 == 0.0 { 0.0 } else { (assign61260_e79382 * ((assign61260_e79378).powf(assign61260_e79382 - 1.0) * ((var_btat__blk1570_dn9 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn9)))) } } else { (assign61260_e79383 * (assign61260_e79382 * (((var_btat__blk1570_dn9 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn9)) / assign61260_e79378))) }, if 0.0 == 0.0 && ((assign61260_e79382) as f64).is_finite() && ((assign61260_e79382) as f64).fract() == 0.0 { if assign61260_e79382 == 0.0 { 0.0 } else { (assign61260_e79382 * ((assign61260_e79378).powf(assign61260_e79382 - 1.0) * ((var_btat__blk1570_dn11 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn11)))) } } else { (assign61260_e79383 * (assign61260_e79382 * (((var_btat__blk1570_dn11 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn11)) / assign61260_e79378))) }, if 0.0 == 0.0 && ((assign61260_e79382) as f64).is_finite() && ((assign61260_e79382) as f64).fract() == 0.0 { if assign61260_e79382 == 0.0 { 0.0 } else { (assign61260_e79382 * ((assign61260_e79378).powf(assign61260_e79382 - 1.0) * ((var_btat__blk1570_dn12 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn12)))) } } else { (assign61260_e79383 * (assign61260_e79382 * (((var_btat__blk1570_dn12 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn12)) / assign61260_e79378))) },)
    } else {
        (var_wgamma__blk1576, var_wgamma__blk1576_dn6, var_wgamma__blk1576_dn7, var_wgamma__blk1576_dn8, var_wgamma__blk1576_dn9, var_wgamma__blk1576_dn11, var_wgamma__blk1576_dn12,)
    }
};
        var_wgamma__blk1576 = assign61260_e79385;
        var_wgamma__blk1576_dn6 = assign61260_e79385_d_n6;
        var_wgamma__blk1576_dn7 = assign61260_e79385_d_n7;
        var_wgamma__blk1576_dn8 = assign61260_e79385_d_n8;
        var_wgamma__blk1576_dn9 = assign61260_e79385_d_n9;
        var_wgamma__blk1576_dn11 = assign61260_e79385_d_n11;
        var_wgamma__blk1576_dn12 = assign61260_e79385_d_n12;

        let (assign61270_e79404, assign61270_e79404_d_n6, assign61270_e79404_d_n7, assign61270_e79404_d_n8, assign61270_e79404_d_n9, assign61270_e79404_d_n11, assign61270_e79404_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61270_e79398: f64 = (var_wsrh__blk1566 * var_wgamma__blk1576);
        let assign61270_e79401: f64 = (var_wsrh__blk1566 + var_wgamma__blk1576);
        let assign61270_e79402: f64 = (assign61270_e79398 / assign61270_e79401);
        (assign61270_e79402, ((((var_wsrh__blk1566 * var_wgamma__blk1576_dn6) * assign61270_e79401) - (assign61270_e79398 * var_wgamma__blk1576_dn6)) / (assign61270_e79401 * assign61270_e79401)), (((((var_wsrh__blk1566_dn7 * var_wgamma__blk1576) + (var_wsrh__blk1566 * var_wgamma__blk1576_dn7)) * assign61270_e79401) - (assign61270_e79398 * (var_wsrh__blk1566_dn7 + var_wgamma__blk1576_dn7))) / (assign61270_e79401 * assign61270_e79401)), (((((var_wsrh__blk1566_dn8 * var_wgamma__blk1576) + (var_wsrh__blk1566 * var_wgamma__blk1576_dn8)) * assign61270_e79401) - (assign61270_e79398 * (var_wsrh__blk1566_dn8 + var_wgamma__blk1576_dn8))) / (assign61270_e79401 * assign61270_e79401)), ((((var_wsrh__blk1566 * var_wgamma__blk1576_dn9) * assign61270_e79401) - (assign61270_e79398 * var_wgamma__blk1576_dn9)) / (assign61270_e79401 * assign61270_e79401)), (((((var_wsrh__blk1566_dn11 * var_wgamma__blk1576) + (var_wsrh__blk1566 * var_wgamma__blk1576_dn11)) * assign61270_e79401) - (assign61270_e79398 * (var_wsrh__blk1566_dn11 + var_wgamma__blk1576_dn11))) / (assign61270_e79401 * assign61270_e79401)), (((((var_wsrh__blk1566_dn12 * var_wgamma__blk1576) + (var_wsrh__blk1566 * var_wgamma__blk1576_dn12)) * assign61270_e79401) - (assign61270_e79398 * (var_wsrh__blk1566_dn12 + var_wgamma__blk1576_dn12))) / (assign61270_e79401 * assign61270_e79401)),)
    } else {
        (var_wtat__blk1577, var_wtat__blk1577_dn6, var_wtat__blk1577_dn7, var_wtat__blk1577_dn8, var_wtat__blk1577_dn9, var_wtat__blk1577_dn11, var_wtat__blk1577_dn12,)
    }
};
        var_wtat__blk1577 = assign61270_e79404;
        var_wtat__blk1577_dn6 = assign61270_e79404_d_n6;
        var_wtat__blk1577_dn7 = assign61270_e79404_d_n7;
        var_wtat__blk1577_dn8 = assign61270_e79404_d_n8;
        var_wtat__blk1577_dn9 = assign61270_e79404_d_n9;
        var_wtat__blk1577_dn11 = assign61270_e79404_d_n11;
        var_wtat__blk1577_dn12 = assign61270_e79404_d_n12;

        let (assign61280_e79422, assign61280_e79422_d_n6, assign61280_e79422_d_n7, assign61280_e79422_d_n8, assign61280_e79422_d_n9, assign61280_e79422_d_n11, assign61280_e79422_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61280_e79418: f64 = (var_btat__blk1570 / var_sqrtumax__blk1574);
        let assign61280_e79419: f64 = (0.375 * assign61280_e79418);
        let assign61280_e79420: f64 = (assign61280_e79419).sqrt();
        (assign61280_e79420, ((0.375 * (((var_btat__blk1570_dn6 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn6)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign61280_e79420)), ((0.375 * (((var_btat__blk1570_dn7 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn7)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign61280_e79420)), ((0.375 * (((var_btat__blk1570_dn8 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn8)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign61280_e79420)), ((0.375 * (((var_btat__blk1570_dn9 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn9)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign61280_e79420)), ((0.375 * (((var_btat__blk1570_dn11 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn11)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign61280_e79420)), ((0.375 * (((var_btat__blk1570_dn12 * var_sqrtumax__blk1574) - (var_btat__blk1570 * var_sqrtumax__blk1574_dn12)) / (var_sqrtumax__blk1574 * var_sqrtumax__blk1574))) / (2.0 * assign61280_e79420)),)
    } else {
        (var_ktat__blk1578, var_ktat__blk1578_dn6, var_ktat__blk1578_dn7, var_ktat__blk1578_dn8, var_ktat__blk1578_dn9, var_ktat__blk1578_dn11, var_ktat__blk1578_dn12,)
    }
};
        var_ktat__blk1578 = assign61280_e79422;
        var_ktat__blk1578_dn6 = assign61280_e79422_d_n6;
        var_ktat__blk1578_dn7 = assign61280_e79422_d_n7;
        var_ktat__blk1578_dn8 = assign61280_e79422_d_n8;
        var_ktat__blk1578_dn9 = assign61280_e79422_d_n9;
        var_ktat__blk1578_dn11 = assign61280_e79422_d_n11;
        var_ktat__blk1578_dn12 = assign61280_e79422_d_n12;

        let (assign61290_e79441, assign61290_e79441_d_n6, assign61290_e79441_d_n7, assign61290_e79441_d_n8, assign61290_e79441_d_n9, assign61290_e79441_d_n11, assign61290_e79441_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61290_e79436: f64 = (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574);
        let assign61290_e79437: f64 = (2.0 * assign61290_e79436);
        let assign61290_e79439: f64 = (assign61290_e79437 - var_umax__blk1573);
        (assign61290_e79439, ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn6 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn6))) - var_umax__blk1573_dn6), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn7 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn7))) - var_umax__blk1573_dn7), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn8 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn8))) - var_umax__blk1573_dn8), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn9 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn9))) - var_umax__blk1573_dn9), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn11 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn11))) - var_umax__blk1573_dn11), ((2.0 * ((var_twoatatoverthreebtat__blk1571_dn12 * var_sqrtumax__blk1574) + (var_twoatatoverthreebtat__blk1571 * var_sqrtumax__blk1574_dn12))) - var_umax__blk1573_dn12),)
    } else {
        (var_ltat__blk1579, var_ltat__blk1579_dn6, var_ltat__blk1579_dn7, var_ltat__blk1579_dn8, var_ltat__blk1579_dn9, var_ltat__blk1579_dn11, var_ltat__blk1579_dn12,)
    }
};
        var_ltat__blk1579 = assign61290_e79441;
        var_ltat__blk1579_dn6 = assign61290_e79441_d_n6;
        var_ltat__blk1579_dn7 = assign61290_e79441_d_n7;
        var_ltat__blk1579_dn8 = assign61290_e79441_d_n8;
        var_ltat__blk1579_dn9 = assign61290_e79441_d_n9;
        var_ltat__blk1579_dn11 = assign61290_e79441_d_n11;
        var_ltat__blk1579_dn12 = assign61290_e79441_d_n12;

        let (assign61300_e79468, assign61300_e79468_d_n6, assign61300_e79468_d_n7, assign61300_e79468_d_n8, assign61300_e79468_d_n9, assign61300_e79468_d_n11, assign61300_e79468_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61300_e79454: f64 = (var_atatgat_d * var_twoatatoverthreebtat__blk1571);
        let assign61300_e79456: f64 = (assign61300_e79454 * var_sqrtumax__blk1574);
        let assign61300_e79459: f64 = (var_atatgat_d * var_umax__blk1573);
        let assign61300_e79460: f64 = (assign61300_e79456 - assign61300_e79459);
        let assign61300_e79464: f64 = (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575);
        let assign61300_e79465: f64 = (0.5 * assign61300_e79464);
        let assign61300_e79466: f64 = (assign61300_e79460 + assign61300_e79465);
        (assign61300_e79466, (((((var_atatgat_d * var_twoatatoverthreebtat__blk1571_dn6) * var_sqrtumax__blk1574) + (assign61300_e79454 * var_sqrtumax__blk1574_dn6)) - (var_atatgat_d * var_umax__blk1573_dn6)) + (0.5 * ((var_btat__blk1570_dn6 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn6)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1571_dn7) * var_sqrtumax__blk1574) + (assign61300_e79454 * var_sqrtumax__blk1574_dn7)) - (var_atatgat_d * var_umax__blk1573_dn7)) + (0.5 * ((var_btat__blk1570_dn7 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn7)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1571_dn8) * var_sqrtumax__blk1574) + (assign61300_e79454 * var_sqrtumax__blk1574_dn8)) - (var_atatgat_d * var_umax__blk1573_dn8)) + (0.5 * ((var_btat__blk1570_dn8 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn8)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1571_dn9) * var_sqrtumax__blk1574) + (assign61300_e79454 * var_sqrtumax__blk1574_dn9)) - (var_atatgat_d * var_umax__blk1573_dn9)) + (0.5 * ((var_btat__blk1570_dn9 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn9)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1571_dn11) * var_sqrtumax__blk1574) + (assign61300_e79454 * var_sqrtumax__blk1574_dn11)) - (var_atatgat_d * var_umax__blk1573_dn11)) + (0.5 * ((var_btat__blk1570_dn11 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn11)))), (((((var_atatgat_d * var_twoatatoverthreebtat__blk1571_dn12) * var_sqrtumax__blk1574) + (assign61300_e79454 * var_sqrtumax__blk1574_dn12)) - (var_atatgat_d * var_umax__blk1573_dn12)) + (0.5 * ((var_btat__blk1570_dn12 * var_umaxpoweronepointfive__blk1575) + (var_btat__blk1570 * var_umaxpoweronepointfive__blk1575_dn12)))),)
    } else {
        (var_mtat__blk1580, var_mtat__blk1580_dn6, var_mtat__blk1580_dn7, var_mtat__blk1580_dn8, var_mtat__blk1580_dn9, var_mtat__blk1580_dn11, var_mtat__blk1580_dn12,)
    }
};
        var_mtat__blk1580 = assign61300_e79468;
        var_mtat__blk1580_dn6 = assign61300_e79468_d_n6;
        var_mtat__blk1580_dn7 = assign61300_e79468_d_n7;
        var_mtat__blk1580_dn8 = assign61300_e79468_d_n8;
        var_mtat__blk1580_dn9 = assign61300_e79468_d_n9;
        var_mtat__blk1580_dn11 = assign61300_e79468_d_n11;
        var_mtat__blk1580_dn12 = assign61300_e79468_d_n12;

        let (assign61310_e79485, assign61310_e79485_d_n6, assign61310_e79485_d_n7, assign61310_e79485_d_n8, assign61310_e79485_d_n9, assign61310_e79485_d_n11, assign61310_e79485_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61310_e79481: f64 = (var_ltat__blk1579 - 1.0);
        let assign61310_e79483: f64 = (assign61310_e79481 * var_ktat__blk1578);
        (assign61310_e79483, ((var_ltat__blk1579_dn6 * var_ktat__blk1578) + (assign61310_e79481 * var_ktat__blk1578_dn6)), ((var_ltat__blk1579_dn7 * var_ktat__blk1578) + (assign61310_e79481 * var_ktat__blk1578_dn7)), ((var_ltat__blk1579_dn8 * var_ktat__blk1578) + (assign61310_e79481 * var_ktat__blk1578_dn8)), ((var_ltat__blk1579_dn9 * var_ktat__blk1578) + (assign61310_e79481 * var_ktat__blk1578_dn9)), ((var_ltat__blk1579_dn11 * var_ktat__blk1578) + (assign61310_e79481 * var_ktat__blk1578_dn11)), ((var_ltat__blk1579_dn12 * var_ktat__blk1578) + (assign61310_e79481 * var_ktat__blk1578_dn12)),)
    } else {
        (var_xerfc__blk1581, var_xerfc__blk1581_dn6, var_xerfc__blk1581_dn7, var_xerfc__blk1581_dn8, var_xerfc__blk1581_dn9, var_xerfc__blk1581_dn11, var_xerfc__blk1581_dn12,)
    }
};
        var_xerfc__blk1581 = assign61310_e79485;
        var_xerfc__blk1581_dn6 = assign61310_e79485_d_n6;
        var_xerfc__blk1581_dn7 = assign61310_e79485_d_n7;
        var_xerfc__blk1581_dn8 = assign61310_e79485_d_n8;
        var_xerfc__blk1581_dn9 = assign61310_e79485_d_n9;
        var_xerfc__blk1581_dn11 = assign61310_e79485_d_n11;
        var_xerfc__blk1581_dn12 = assign61310_e79485_d_n12;

        let (assign61320_e79500, assign61320_e79500_d_n6, assign61320_e79500_d_n7, assign61320_e79500_d_n8, assign61320_e79500_d_n9, assign61320_e79500_d_n11, assign61320_e79500_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61320_e79498: f64 = (var_xerfc__blk1581 * var_xerfc__blk1581);
        (assign61320_e79498, ((var_xerfc__blk1581_dn6 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn6)), ((var_xerfc__blk1581_dn7 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn7)), ((var_xerfc__blk1581_dn8 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn8)), ((var_xerfc__blk1581_dn9 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn9)), ((var_xerfc__blk1581_dn11 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn11)), ((var_xerfc__blk1581_dn12 * var_xerfc__blk1581) + (var_xerfc__blk1581 * var_xerfc__blk1581_dn12)),)
    } else {
        (var_ysq__blk1542, var_ysq__blk1542_dn6, var_ysq__blk1542_dn7, var_ysq__blk1542_dn8, var_ysq__blk1542_dn9, var_ysq__blk1542_dn11, var_ysq__blk1542_dn12,)
    }
};
        var_ysq__blk1542 = assign61320_e79500;
        var_ysq__blk1542_dn6 = assign61320_e79500_d_n6;
        var_ysq__blk1542_dn7 = assign61320_e79500_d_n7;
        var_ysq__blk1542_dn8 = assign61320_e79500_d_n8;
        var_ysq__blk1542_dn9 = assign61320_e79500_d_n9;
        var_ysq__blk1542_dn11 = assign61320_e79500_d_n11;
        var_ysq__blk1542_dn12 = assign61320_e79500_d_n12;

        let assign61330_e79503: f64 = if var_xerfc__blk1581 > 0.0 { 1.0 } else { 0.0 };
        var_guard1720 = assign61330_e79503;

        let (assign61340_e79524, assign61340_e79524_d_n6, assign61340_e79524_d_n7, assign61340_e79524_d_n8, assign61340_e79524_d_n9, assign61340_e79524_d_n11, assign61340_e79524_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1720 != 0.0)) {
        let assign61340_e79520: f64 = (var_perfc * var_xerfc__blk1581);
        let assign61340_e79521: f64 = (1.0 + assign61340_e79520);
        let assign61340_e79522: f64 = (1.0 / assign61340_e79521);
        (assign61340_e79522, (-((var_perfc * var_xerfc__blk1581_dn6) / (assign61340_e79521 * assign61340_e79521))), (-((var_perfc * var_xerfc__blk1581_dn7) / (assign61340_e79521 * assign61340_e79521))), (-((var_perfc * var_xerfc__blk1581_dn8) / (assign61340_e79521 * assign61340_e79521))), (-((var_perfc * var_xerfc__blk1581_dn9) / (assign61340_e79521 * assign61340_e79521))), (-((var_perfc * var_xerfc__blk1581_dn11) / (assign61340_e79521 * assign61340_e79521))), (-((var_perfc * var_xerfc__blk1581_dn12) / (assign61340_e79521 * assign61340_e79521))),)
    } else {
        (var_terfc__blk1543, var_terfc__blk1543_dn6, var_terfc__blk1543_dn7, var_terfc__blk1543_dn8, var_terfc__blk1543_dn9, var_terfc__blk1543_dn11, var_terfc__blk1543_dn12,)
    }
};
        var_terfc__blk1543 = assign61340_e79524;
        var_terfc__blk1543_dn6 = assign61340_e79524_d_n6;
        var_terfc__blk1543_dn7 = assign61340_e79524_d_n7;
        var_terfc__blk1543_dn8 = assign61340_e79524_d_n8;
        var_terfc__blk1543_dn9 = assign61340_e79524_d_n9;
        var_terfc__blk1543_dn11 = assign61340_e79524_d_n11;
        var_terfc__blk1543_dn12 = assign61340_e79524_d_n12;

        let (assign61350_e79546, assign61350_e79546_d_n6, assign61350_e79546_d_n7, assign61350_e79546_d_n8, assign61350_e79546_d_n9, assign61350_e79546_d_n11, assign61350_e79546_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1720 == 0.0)) {
        let assign61350_e79542: f64 = (var_perfc * var_xerfc__blk1581);
        let assign61350_e79543: f64 = (1.0 - assign61350_e79542);
        let assign61350_e79544: f64 = (1.0 / assign61350_e79543);
        (assign61350_e79544, (-((-(var_perfc * var_xerfc__blk1581_dn6)) / (assign61350_e79543 * assign61350_e79543))), (-((-(var_perfc * var_xerfc__blk1581_dn7)) / (assign61350_e79543 * assign61350_e79543))), (-((-(var_perfc * var_xerfc__blk1581_dn8)) / (assign61350_e79543 * assign61350_e79543))), (-((-(var_perfc * var_xerfc__blk1581_dn9)) / (assign61350_e79543 * assign61350_e79543))), (-((-(var_perfc * var_xerfc__blk1581_dn11)) / (assign61350_e79543 * assign61350_e79543))), (-((-(var_perfc * var_xerfc__blk1581_dn12)) / (assign61350_e79543 * assign61350_e79543))),)
    } else {
        (var_terfc__blk1543, var_terfc__blk1543_dn6, var_terfc__blk1543_dn7, var_terfc__blk1543_dn8, var_terfc__blk1543_dn9, var_terfc__blk1543_dn11, var_terfc__blk1543_dn12,)
    }
};
        var_terfc__blk1543 = assign61350_e79546;
        var_terfc__blk1543_dn6 = assign61350_e79546_d_n6;
        var_terfc__blk1543_dn7 = assign61350_e79546_d_n7;
        var_terfc__blk1543_dn8 = assign61350_e79546_d_n8;
        var_terfc__blk1543_dn9 = assign61350_e79546_d_n9;
        var_terfc__blk1543_dn11 = assign61350_e79546_d_n11;
        var_terfc__blk1543_dn12 = assign61350_e79546_d_n12;

        let assign61360_e79548: f64 = (-var_ysq__blk1542);
        let assign61360_e79550: f64 = (assign61360_e79548 + var_mtat__blk1580);
        let assign61360_e79552: f64 = (-230.25850929940458);
        let assign61360_e79553: f64 = if assign61360_e79550 > assign61360_e79552 { 1.0 } else { 0.0 };
        var_guard1721 = assign61360_e79553;

        let (assign61370_e79572, assign61370_e79572_d_n6, assign61370_e79572_d_n7, assign61370_e79572_d_n8, assign61370_e79572_d_n9, assign61370_e79572_d_n11, assign61370_e79572_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1721 != 0.0)) {
        let assign61370_e79567: f64 = (-var_ysq__blk1542);
        let assign61370_e79569: f64 = (assign61370_e79567 + var_mtat__blk1580);
        let assign61370_e79570: f64 = (assign61370_e79569).exp();
        (assign61370_e79570, (assign61370_e79570 * ((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)), (assign61370_e79570 * ((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)), (assign61370_e79570 * ((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)), (assign61370_e79570 * ((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)), (assign61370_e79570 * ((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)), (assign61370_e79570 * ((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61370_e79572;
        var_tmp__blk1560_dn6 = assign61370_e79572_d_n6;
        var_tmp__blk1560_dn7 = assign61370_e79572_d_n7;
        var_tmp__blk1560_dn8 = assign61370_e79572_d_n8;
        var_tmp__blk1560_dn9 = assign61370_e79572_d_n9;
        var_tmp__blk1560_dn11 = assign61370_e79572_d_n11;
        var_tmp__blk1560_dn12 = assign61370_e79572_d_n12;

        let (assign61380_e79622, assign61380_e79622_d_n6, assign61380_e79622_d_n7, assign61380_e79622_d_n8, assign61380_e79622_d_n9, assign61380_e79622_d_n11, assign61380_e79622_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1721 == 0.0)) {
        let assign61380_e79589: f64 = (-230.25850929940458);
        let assign61380_e79591: f64 = (-var_ysq__blk1542);
        let assign61380_e79593: f64 = (assign61380_e79591 + var_mtat__blk1580);
        let assign61380_e79594: f64 = (assign61380_e79589 - assign61380_e79593);
        let assign61380_e79598: f64 = (-230.25850929940458);
        let assign61380_e79600: f64 = (-var_ysq__blk1542);
        let assign61380_e79602: f64 = (assign61380_e79600 + var_mtat__blk1580);
        let assign61380_e79603: f64 = (assign61380_e79598 - assign61380_e79602);
        let assign61380_e79606: f64 = (-230.25850929940458);
        let assign61380_e79608: f64 = (-var_ysq__blk1542);
        let assign61380_e79610: f64 = (assign61380_e79608 + var_mtat__blk1580);
        let assign61380_e79611: f64 = (assign61380_e79606 - assign61380_e79610);
        let assign61380_e79613: f64 = (assign61380_e79611 * 0.3333333333333333);
        let assign61380_e79614: f64 = (1.0 + assign61380_e79613);
        let assign61380_e79615: f64 = (assign61380_e79603 * assign61380_e79614);
        let assign61380_e79616: f64 = (0.5 * assign61380_e79615);
        let assign61380_e79617: f64 = (1.0 + assign61380_e79616);
        let assign61380_e79618: f64 = (assign61380_e79594 * assign61380_e79617);
        let assign61380_e79619: f64 = (1.0 + assign61380_e79618);
        let assign61380_e79620: f64 = (1e-100 / assign61380_e79619);
        (assign61380_e79620, (-((1e-100 * (((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * assign61380_e79617) + (assign61380_e79594 * (0.5 * (((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * assign61380_e79614) + (assign61380_e79603 * ((-((-var_ysq__blk1542_dn6) + var_mtat__blk1580_dn6)) * 0.3333333333333333))))))) / (assign61380_e79619 * assign61380_e79619))), (-((1e-100 * (((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * assign61380_e79617) + (assign61380_e79594 * (0.5 * (((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * assign61380_e79614) + (assign61380_e79603 * ((-((-var_ysq__blk1542_dn7) + var_mtat__blk1580_dn7)) * 0.3333333333333333))))))) / (assign61380_e79619 * assign61380_e79619))), (-((1e-100 * (((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * assign61380_e79617) + (assign61380_e79594 * (0.5 * (((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * assign61380_e79614) + (assign61380_e79603 * ((-((-var_ysq__blk1542_dn8) + var_mtat__blk1580_dn8)) * 0.3333333333333333))))))) / (assign61380_e79619 * assign61380_e79619))), (-((1e-100 * (((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * assign61380_e79617) + (assign61380_e79594 * (0.5 * (((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * assign61380_e79614) + (assign61380_e79603 * ((-((-var_ysq__blk1542_dn9) + var_mtat__blk1580_dn9)) * 0.3333333333333333))))))) / (assign61380_e79619 * assign61380_e79619))), (-((1e-100 * (((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * assign61380_e79617) + (assign61380_e79594 * (0.5 * (((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * assign61380_e79614) + (assign61380_e79603 * ((-((-var_ysq__blk1542_dn11) + var_mtat__blk1580_dn11)) * 0.3333333333333333))))))) / (assign61380_e79619 * assign61380_e79619))), (-((1e-100 * (((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * assign61380_e79617) + (assign61380_e79594 * (0.5 * (((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * assign61380_e79614) + (assign61380_e79603 * ((-((-var_ysq__blk1542_dn12) + var_mtat__blk1580_dn12)) * 0.3333333333333333))))))) / (assign61380_e79619 * assign61380_e79619))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61380_e79622;
        var_tmp__blk1560_dn6 = assign61380_e79622_d_n6;
        var_tmp__blk1560_dn7 = assign61380_e79622_d_n7;
        var_tmp__blk1560_dn8 = assign61380_e79622_d_n8;
        var_tmp__blk1560_dn9 = assign61380_e79622_d_n9;
        var_tmp__blk1560_dn11 = assign61380_e79622_d_n11;
        var_tmp__blk1560_dn12 = assign61380_e79622_d_n12;

        let (assign61390_e79653, assign61390_e79653_d_n6, assign61390_e79653_d_n7, assign61390_e79653_d_n8, assign61390_e79653_d_n9, assign61390_e79653_d_n11, assign61390_e79653_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61390_e79635: f64 = (0.29214664 * var_terfc__blk1543);
        let assign61390_e79639: f64 = (var_terfc__blk1543 * var_terfc__blk1543);
        let assign61390_e79640: f64 = (var_berfc * assign61390_e79639);
        let assign61390_e79641: f64 = (assign61390_e79635 + assign61390_e79640);
        let assign61390_e79645: f64 = (var_terfc__blk1543 * var_terfc__blk1543);
        let assign61390_e79647: f64 = (assign61390_e79645 * var_terfc__blk1543);
        let assign61390_e79648: f64 = (var_cerfc * assign61390_e79647);
        let assign61390_e79649: f64 = (assign61390_e79641 + assign61390_e79648);
        let assign61390_e79651: f64 = (assign61390_e79649 * var_tmp__blk1560);
        (assign61390_e79651, (((((0.29214664 * var_terfc__blk1543_dn6) + (var_berfc * ((var_terfc__blk1543_dn6 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn6)))) + (var_cerfc * ((((var_terfc__blk1543_dn6 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn6)) * var_terfc__blk1543) + (assign61390_e79645 * var_terfc__blk1543_dn6)))) * var_tmp__blk1560) + (assign61390_e79649 * var_tmp__blk1560_dn6)), (((((0.29214664 * var_terfc__blk1543_dn7) + (var_berfc * ((var_terfc__blk1543_dn7 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn7)))) + (var_cerfc * ((((var_terfc__blk1543_dn7 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn7)) * var_terfc__blk1543) + (assign61390_e79645 * var_terfc__blk1543_dn7)))) * var_tmp__blk1560) + (assign61390_e79649 * var_tmp__blk1560_dn7)), (((((0.29214664 * var_terfc__blk1543_dn8) + (var_berfc * ((var_terfc__blk1543_dn8 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn8)))) + (var_cerfc * ((((var_terfc__blk1543_dn8 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn8)) * var_terfc__blk1543) + (assign61390_e79645 * var_terfc__blk1543_dn8)))) * var_tmp__blk1560) + (assign61390_e79649 * var_tmp__blk1560_dn8)), (((((0.29214664 * var_terfc__blk1543_dn9) + (var_berfc * ((var_terfc__blk1543_dn9 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn9)))) + (var_cerfc * ((((var_terfc__blk1543_dn9 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn9)) * var_terfc__blk1543) + (assign61390_e79645 * var_terfc__blk1543_dn9)))) * var_tmp__blk1560) + (assign61390_e79649 * var_tmp__blk1560_dn9)), (((((0.29214664 * var_terfc__blk1543_dn11) + (var_berfc * ((var_terfc__blk1543_dn11 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn11)))) + (var_cerfc * ((((var_terfc__blk1543_dn11 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn11)) * var_terfc__blk1543) + (assign61390_e79645 * var_terfc__blk1543_dn11)))) * var_tmp__blk1560) + (assign61390_e79649 * var_tmp__blk1560_dn11)), (((((0.29214664 * var_terfc__blk1543_dn12) + (var_berfc * ((var_terfc__blk1543_dn12 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn12)))) + (var_cerfc * ((((var_terfc__blk1543_dn12 * var_terfc__blk1543) + (var_terfc__blk1543 * var_terfc__blk1543_dn12)) * var_terfc__blk1543) + (assign61390_e79645 * var_terfc__blk1543_dn12)))) * var_tmp__blk1560) + (assign61390_e79649 * var_tmp__blk1560_dn12)),)
    } else {
        (var_erfcpos__blk1544, var_erfcpos__blk1544_dn6, var_erfcpos__blk1544_dn7, var_erfcpos__blk1544_dn8, var_erfcpos__blk1544_dn9, var_erfcpos__blk1544_dn11, var_erfcpos__blk1544_dn12,)
    }
};
        var_erfcpos__blk1544 = assign61390_e79653;
        var_erfcpos__blk1544_dn6 = assign61390_e79653_d_n6;
        var_erfcpos__blk1544_dn7 = assign61390_e79653_d_n7;
        var_erfcpos__blk1544_dn8 = assign61390_e79653_d_n8;
        var_erfcpos__blk1544_dn9 = assign61390_e79653_d_n9;
        var_erfcpos__blk1544_dn11 = assign61390_e79653_d_n11;
        var_erfcpos__blk1544_dn12 = assign61390_e79653_d_n12;

        let assign61400_e79656: f64 = if var_xerfc__blk1581 > 0.0 { 1.0 } else { 0.0 };
        var_guard1722 = assign61400_e79656;

        let (assign61410_e79671, assign61410_e79671_d_n6, assign61410_e79671_d_n7, assign61410_e79671_d_n8, assign61410_e79671_d_n9, assign61410_e79671_d_n11, assign61410_e79671_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1722 != 0.0)) {
        (var_erfcpos__blk1544, var_erfcpos__blk1544_dn6, var_erfcpos__blk1544_dn7, var_erfcpos__blk1544_dn8, var_erfcpos__blk1544_dn9, var_erfcpos__blk1544_dn11, var_erfcpos__blk1544_dn12,)
    } else {
        (var_erfctimesexpmtat__blk1582, var_erfctimesexpmtat__blk1582_dn6, var_erfctimesexpmtat__blk1582_dn7, var_erfctimesexpmtat__blk1582_dn8, var_erfctimesexpmtat__blk1582_dn9, var_erfctimesexpmtat__blk1582_dn11, var_erfctimesexpmtat__blk1582_dn12,)
    }
};
        var_erfctimesexpmtat__blk1582 = assign61410_e79671;
        var_erfctimesexpmtat__blk1582_dn6 = assign61410_e79671_d_n6;
        var_erfctimesexpmtat__blk1582_dn7 = assign61410_e79671_d_n7;
        var_erfctimesexpmtat__blk1582_dn8 = assign61410_e79671_d_n8;
        var_erfctimesexpmtat__blk1582_dn9 = assign61410_e79671_d_n9;
        var_erfctimesexpmtat__blk1582_dn11 = assign61410_e79671_d_n11;
        var_erfctimesexpmtat__blk1582_dn12 = assign61410_e79671_d_n12;

        let assign61420_e79674: f64 = (-230.25850929940458);
        let assign61420_e79675: f64 = if var_mtat__blk1580 > assign61420_e79674 { 1.0 } else { 0.0 };
        var_guard1723 = assign61420_e79675;

        let (assign61430_e79694, assign61430_e79694_d_n6, assign61430_e79694_d_n7, assign61430_e79694_d_n8, assign61430_e79694_d_n9, assign61430_e79694_d_n11, assign61430_e79694_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1722 == 0.0)) && (var_guard1723 != 0.0)) {
        let assign61430_e79692: f64 = (var_mtat__blk1580).exp();
        (assign61430_e79692, (assign61430_e79692 * var_mtat__blk1580_dn6), (assign61430_e79692 * var_mtat__blk1580_dn7), (assign61430_e79692 * var_mtat__blk1580_dn8), (assign61430_e79692 * var_mtat__blk1580_dn9), (assign61430_e79692 * var_mtat__blk1580_dn11), (assign61430_e79692 * var_mtat__blk1580_dn12),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61430_e79694;
        var_tmp__blk1560_dn6 = assign61430_e79694_d_n6;
        var_tmp__blk1560_dn7 = assign61430_e79694_d_n7;
        var_tmp__blk1560_dn8 = assign61430_e79694_d_n8;
        var_tmp__blk1560_dn9 = assign61430_e79694_d_n9;
        var_tmp__blk1560_dn11 = assign61430_e79694_d_n11;
        var_tmp__blk1560_dn12 = assign61430_e79694_d_n12;

        let (assign61440_e79738, assign61440_e79738_d_n6, assign61440_e79738_d_n7, assign61440_e79738_d_n8, assign61440_e79738_d_n9, assign61440_e79738_d_n11, assign61440_e79738_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1722 == 0.0)) && (var_guard1723 == 0.0)) {
        let assign61440_e79714: f64 = (-230.25850929940458);
        let assign61440_e79716: f64 = (assign61440_e79714 - var_mtat__blk1580);
        let assign61440_e79720: f64 = (-230.25850929940458);
        let assign61440_e79722: f64 = (assign61440_e79720 - var_mtat__blk1580);
        let assign61440_e79725: f64 = (-230.25850929940458);
        let assign61440_e79727: f64 = (assign61440_e79725 - var_mtat__blk1580);
        let assign61440_e79729: f64 = (assign61440_e79727 * 0.3333333333333333);
        let assign61440_e79730: f64 = (1.0 + assign61440_e79729);
        let assign61440_e79731: f64 = (assign61440_e79722 * assign61440_e79730);
        let assign61440_e79732: f64 = (0.5 * assign61440_e79731);
        let assign61440_e79733: f64 = (1.0 + assign61440_e79732);
        let assign61440_e79734: f64 = (assign61440_e79716 * assign61440_e79733);
        let assign61440_e79735: f64 = (1.0 + assign61440_e79734);
        let assign61440_e79736: f64 = (1e-100 / assign61440_e79735);
        (assign61440_e79736, (-((1e-100 * (((-var_mtat__blk1580_dn6) * assign61440_e79733) + (assign61440_e79716 * (0.5 * (((-var_mtat__blk1580_dn6) * assign61440_e79730) + (assign61440_e79722 * ((-var_mtat__blk1580_dn6) * 0.3333333333333333))))))) / (assign61440_e79735 * assign61440_e79735))), (-((1e-100 * (((-var_mtat__blk1580_dn7) * assign61440_e79733) + (assign61440_e79716 * (0.5 * (((-var_mtat__blk1580_dn7) * assign61440_e79730) + (assign61440_e79722 * ((-var_mtat__blk1580_dn7) * 0.3333333333333333))))))) / (assign61440_e79735 * assign61440_e79735))), (-((1e-100 * (((-var_mtat__blk1580_dn8) * assign61440_e79733) + (assign61440_e79716 * (0.5 * (((-var_mtat__blk1580_dn8) * assign61440_e79730) + (assign61440_e79722 * ((-var_mtat__blk1580_dn8) * 0.3333333333333333))))))) / (assign61440_e79735 * assign61440_e79735))), (-((1e-100 * (((-var_mtat__blk1580_dn9) * assign61440_e79733) + (assign61440_e79716 * (0.5 * (((-var_mtat__blk1580_dn9) * assign61440_e79730) + (assign61440_e79722 * ((-var_mtat__blk1580_dn9) * 0.3333333333333333))))))) / (assign61440_e79735 * assign61440_e79735))), (-((1e-100 * (((-var_mtat__blk1580_dn11) * assign61440_e79733) + (assign61440_e79716 * (0.5 * (((-var_mtat__blk1580_dn11) * assign61440_e79730) + (assign61440_e79722 * ((-var_mtat__blk1580_dn11) * 0.3333333333333333))))))) / (assign61440_e79735 * assign61440_e79735))), (-((1e-100 * (((-var_mtat__blk1580_dn12) * assign61440_e79733) + (assign61440_e79716 * (0.5 * (((-var_mtat__blk1580_dn12) * assign61440_e79730) + (assign61440_e79722 * ((-var_mtat__blk1580_dn12) * 0.3333333333333333))))))) / (assign61440_e79735 * assign61440_e79735))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61440_e79738;
        var_tmp__blk1560_dn6 = assign61440_e79738_d_n6;
        var_tmp__blk1560_dn7 = assign61440_e79738_d_n7;
        var_tmp__blk1560_dn8 = assign61440_e79738_d_n8;
        var_tmp__blk1560_dn9 = assign61440_e79738_d_n9;
        var_tmp__blk1560_dn11 = assign61440_e79738_d_n11;
        var_tmp__blk1560_dn12 = assign61440_e79738_d_n12;

        let (assign61450_e79758, assign61450_e79758_d_n6, assign61450_e79758_d_n7, assign61450_e79758_d_n8, assign61450_e79758_d_n9, assign61450_e79758_d_n11, assign61450_e79758_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) && (var_guard1722 == 0.0)) {
        let assign61450_e79754: f64 = (2.0 * var_tmp__blk1560);
        let assign61450_e79756: f64 = (assign61450_e79754 - var_erfcpos__blk1544);
        (assign61450_e79756, ((2.0 * var_tmp__blk1560_dn6) - var_erfcpos__blk1544_dn6), ((2.0 * var_tmp__blk1560_dn7) - var_erfcpos__blk1544_dn7), ((2.0 * var_tmp__blk1560_dn8) - var_erfcpos__blk1544_dn8), ((2.0 * var_tmp__blk1560_dn9) - var_erfcpos__blk1544_dn9), ((2.0 * var_tmp__blk1560_dn11) - var_erfcpos__blk1544_dn11), ((2.0 * var_tmp__blk1560_dn12) - var_erfcpos__blk1544_dn12),)
    } else {
        (var_erfctimesexpmtat__blk1582, var_erfctimesexpmtat__blk1582_dn6, var_erfctimesexpmtat__blk1582_dn7, var_erfctimesexpmtat__blk1582_dn8, var_erfctimesexpmtat__blk1582_dn9, var_erfctimesexpmtat__blk1582_dn11, var_erfctimesexpmtat__blk1582_dn12,)
    }
};
        var_erfctimesexpmtat__blk1582 = assign61450_e79758;
        var_erfctimesexpmtat__blk1582_dn6 = assign61450_e79758_d_n6;
        var_erfctimesexpmtat__blk1582_dn7 = assign61450_e79758_d_n7;
        var_erfctimesexpmtat__blk1582_dn8 = assign61450_e79758_d_n8;
        var_erfctimesexpmtat__blk1582_dn9 = assign61450_e79758_d_n9;
        var_erfctimesexpmtat__blk1582_dn11 = assign61450_e79758_d_n11;
        var_erfctimesexpmtat__blk1582_dn12 = assign61450_e79758_d_n12;

        let (assign61460_e79779, assign61460_e79779_d_n6, assign61460_e79779_d_n7, assign61460_e79779_d_n8, assign61460_e79779_d_n9, assign61460_e79779_d_n11, assign61460_e79779_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61460_e79771: f64 = (1.772453850905516 * 0.5);
        let assign61460_e79774: f64 = (var_atatgat_d * var_erfctimesexpmtat__blk1582);
        let assign61460_e79776: f64 = (assign61460_e79774 / var_ktat__blk1578);
        let assign61460_e79777: f64 = (assign61460_e79771 * assign61460_e79776);
        (assign61460_e79777, (assign61460_e79771 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1582_dn6) * var_ktat__blk1578) - (assign61460_e79774 * var_ktat__blk1578_dn6)) / (var_ktat__blk1578 * var_ktat__blk1578))), (assign61460_e79771 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1582_dn7) * var_ktat__blk1578) - (assign61460_e79774 * var_ktat__blk1578_dn7)) / (var_ktat__blk1578 * var_ktat__blk1578))), (assign61460_e79771 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1582_dn8) * var_ktat__blk1578) - (assign61460_e79774 * var_ktat__blk1578_dn8)) / (var_ktat__blk1578 * var_ktat__blk1578))), (assign61460_e79771 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1582_dn9) * var_ktat__blk1578) - (assign61460_e79774 * var_ktat__blk1578_dn9)) / (var_ktat__blk1578 * var_ktat__blk1578))), (assign61460_e79771 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1582_dn11) * var_ktat__blk1578) - (assign61460_e79774 * var_ktat__blk1578_dn11)) / (var_ktat__blk1578 * var_ktat__blk1578))), (assign61460_e79771 * ((((var_atatgat_d * var_erfctimesexpmtat__blk1582_dn12) * var_ktat__blk1578) - (assign61460_e79774 * var_ktat__blk1578_dn12)) / (var_ktat__blk1578 * var_ktat__blk1578))),)
    } else {
        (var_gammamax__blk1583, var_gammamax__blk1583_dn6, var_gammamax__blk1583_dn7, var_gammamax__blk1583_dn8, var_gammamax__blk1583_dn9, var_gammamax__blk1583_dn11, var_gammamax__blk1583_dn12,)
    }
};
        var_gammamax__blk1583 = assign61460_e79779;
        var_gammamax__blk1583_dn6 = assign61460_e79779_d_n6;
        var_gammamax__blk1583_dn7 = assign61460_e79779_d_n7;
        var_gammamax__blk1583_dn8 = assign61460_e79779_d_n8;
        var_gammamax__blk1583_dn9 = assign61460_e79779_d_n9;
        var_gammamax__blk1583_dn11 = assign61460_e79779_d_n11;
        var_gammamax__blk1583_dn12 = assign61460_e79779_d_n12;

        let (assign61470_e79798, assign61470_e79798_d_n6, assign61470_e79798_d_n7, assign61470_e79798_d_n8, assign61470_e79798_d_n9, assign61470_e79798_d_n11, assign61470_e79798_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1718 == 0.0)) {
        let assign61470_e79793: f64 = (var_asrh__blk1568 * var_gammamax__blk1583);
        let assign61470_e79795: f64 = (assign61470_e79793 * var_wtat__blk1577);
        let assign61470_e79796: f64 = (var_ctatgatd_i * assign61470_e79795);
        (assign61470_e79796, (var_ctatgatd_i * ((((var_asrh__blk1568_dn6 * var_gammamax__blk1583) + (var_asrh__blk1568 * var_gammamax__blk1583_dn6)) * var_wtat__blk1577) + (assign61470_e79793 * var_wtat__blk1577_dn6))), (var_ctatgatd_i * ((((var_asrh__blk1568_dn7 * var_gammamax__blk1583) + (var_asrh__blk1568 * var_gammamax__blk1583_dn7)) * var_wtat__blk1577) + (assign61470_e79793 * var_wtat__blk1577_dn7))), (var_ctatgatd_i * ((((var_asrh__blk1568_dn8 * var_gammamax__blk1583) + (var_asrh__blk1568 * var_gammamax__blk1583_dn8)) * var_wtat__blk1577) + (assign61470_e79793 * var_wtat__blk1577_dn8))), (var_ctatgatd_i * ((((var_asrh__blk1568_dn9 * var_gammamax__blk1583) + (var_asrh__blk1568 * var_gammamax__blk1583_dn9)) * var_wtat__blk1577) + (assign61470_e79793 * var_wtat__blk1577_dn9))), (var_ctatgatd_i * ((((var_asrh__blk1568_dn11 * var_gammamax__blk1583) + (var_asrh__blk1568 * var_gammamax__blk1583_dn11)) * var_wtat__blk1577) + (assign61470_e79793 * var_wtat__blk1577_dn11))), (var_ctatgatd_i * ((((var_asrh__blk1568_dn12 * var_gammamax__blk1583) + (var_asrh__blk1568 * var_gammamax__blk1583_dn12)) * var_wtat__blk1577) + (assign61470_e79793 * var_wtat__blk1577_dn12))),)
    } else {
        (var_itat__blk1569, var_itat__blk1569_dn6, var_itat__blk1569_dn7, var_itat__blk1569_dn8, var_itat__blk1569_dn9, var_itat__blk1569_dn11, var_itat__blk1569_dn12,)
    }
};
        var_itat__blk1569 = assign61470_e79798;
        var_itat__blk1569_dn6 = assign61470_e79798_d_n6;
        var_itat__blk1569_dn7 = assign61470_e79798_d_n7;
        var_itat__blk1569_dn8 = assign61470_e79798_d_n8;
        var_itat__blk1569_dn9 = assign61470_e79798_d_n9;
        var_itat__blk1569_dn11 = assign61470_e79798_d_n11;
        var_itat__blk1569_dn12 = assign61470_e79798_d_n12;

        let assign61480_e79801: f64 = if var_cbbtgatd_i == 0.0 { 1.0 } else { 0.0 };
        var_guard1724 = assign61480_e79801;

        let (assign61490_e79813, assign61490_e79813_d_n6, assign61490_e79813_d_n7, assign61490_e79813_d_n8, assign61490_e79813_d_n9, assign61490_e79813_d_n11, assign61490_e79813_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1724 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt__blk1584, var_ibbt__blk1584_dn6, var_ibbt__blk1584_dn7, var_ibbt__blk1584_dn8, var_ibbt__blk1584_dn9, var_ibbt__blk1584_dn11, var_ibbt__blk1584_dn12,)
    }
};
        var_ibbt__blk1584 = assign61490_e79813;
        var_ibbt__blk1584_dn6 = assign61490_e79813_d_n6;
        var_ibbt__blk1584_dn7 = assign61490_e79813_d_n7;
        var_ibbt__blk1584_dn8 = assign61490_e79813_d_n8;
        var_ibbt__blk1584_dn9 = assign61490_e79813_d_n9;
        var_ibbt__blk1584_dn11 = assign61490_e79813_d_n11;
        var_ibbt__blk1584_dn12 = assign61490_e79813_d_n12;

        let assign61500_e79816: f64 = if var_pgatd_i == 0.5 { 1.0 } else { 0.0 };
        var_guard1725 = assign61500_e79816;

        let (assign61510_e79836, assign61510_e79836_d_n6, assign61510_e79836_d_n7, assign61510_e79836_d_n8, assign61510_e79836_d_n9, assign61510_e79836_d_n11, assign61510_e79836_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1724 == 0.0)) && (var_guard1725 != 0.0)) {
        let assign61510_e79831: f64 = (var_vbirgatd_i - var_vbbt__blk1558);
        let assign61510_e79833: f64 = (assign61510_e79831 * var_vbirgatinv_d);
        let assign61510_e79834: f64 = (assign61510_e79833).sqrt();
        (assign61510_e79834, 0.0, (((-var_vbbt__blk1558_dn7) * var_vbirgatinv_d) / (2.0 * assign61510_e79834)), (((-var_vbbt__blk1558_dn8) * var_vbirgatinv_d) / (2.0 * assign61510_e79834)), 0.0, (((-var_vbbt__blk1558_dn11) * var_vbirgatinv_d) / (2.0 * assign61510_e79834)), (((-var_vbbt__blk1558_dn12) * var_vbirgatinv_d) / (2.0 * assign61510_e79834)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61510_e79836;
        var_tmp__blk1560_dn6 = assign61510_e79836_d_n6;
        var_tmp__blk1560_dn7 = assign61510_e79836_d_n7;
        var_tmp__blk1560_dn8 = assign61510_e79836_d_n8;
        var_tmp__blk1560_dn9 = assign61510_e79836_d_n9;
        var_tmp__blk1560_dn11 = assign61510_e79836_d_n11;
        var_tmp__blk1560_dn12 = assign61510_e79836_d_n12;

        let (assign61520_e79858, assign61520_e79858_d_n6, assign61520_e79858_d_n7, assign61520_e79858_d_n8, assign61520_e79858_d_n9, assign61520_e79858_d_n11, assign61520_e79858_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1724 == 0.0)) && (var_guard1725 == 0.0)) {
        let assign61520_e79852: f64 = (var_vbirgatd_i - var_vbbt__blk1558);
        let assign61520_e79854: f64 = (assign61520_e79852 * var_vbirgatinv_d);
        let assign61520_e79856: f64 = (assign61520_e79854).powf(var_pgatd_i);
        (assign61520_e79856, 0.0, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61520_e79854).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1558_dn7) * var_vbirgatinv_d))) } } else { (assign61520_e79856 * (var_pgatd_i * (((-var_vbbt__blk1558_dn7) * var_vbirgatinv_d) / assign61520_e79854))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61520_e79854).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1558_dn8) * var_vbirgatinv_d))) } } else { (assign61520_e79856 * (var_pgatd_i * (((-var_vbbt__blk1558_dn8) * var_vbirgatinv_d) / assign61520_e79854))) }, 0.0, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61520_e79854).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1558_dn11) * var_vbirgatinv_d))) } } else { (assign61520_e79856 * (var_pgatd_i * (((-var_vbbt__blk1558_dn11) * var_vbirgatinv_d) / assign61520_e79854))) }, if 0.0 == 0.0 && ((var_pgatd_i) as f64).is_finite() && ((var_pgatd_i) as f64).fract() == 0.0 { if var_pgatd_i == 0.0 { 0.0 } else { (var_pgatd_i * ((assign61520_e79854).powf(var_pgatd_i - 1.0) * ((-var_vbbt__blk1558_dn12) * var_vbirgatinv_d))) } } else { (assign61520_e79856 * (var_pgatd_i * (((-var_vbbt__blk1558_dn12) * var_vbirgatinv_d) / assign61520_e79854))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61520_e79858;
        var_tmp__blk1560_dn6 = assign61520_e79858_d_n6;
        var_tmp__blk1560_dn7 = assign61520_e79858_d_n7;
        var_tmp__blk1560_dn8 = assign61520_e79858_d_n8;
        var_tmp__blk1560_dn9 = assign61520_e79858_d_n9;
        var_tmp__blk1560_dn11 = assign61520_e79858_d_n11;
        var_tmp__blk1560_dn12 = assign61520_e79858_d_n12;

        let (assign61530_e79879, assign61530_e79879_d_n6, assign61530_e79879_d_n7, assign61530_e79879_d_n8, assign61530_e79879_d_n9, assign61530_e79879_d_n11, assign61530_e79879_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1724 == 0.0)) {
        let assign61530_e79872: f64 = (var_vbirgatd_i - var_vbbt__blk1558);
        let assign61530_e79874: f64 = (assign61530_e79872 * var_wdepnulrinvgat_d);
        let assign61530_e79876: f64 = (assign61530_e79874 / var_tmp__blk1560);
        let assign61530_e79877: f64 = (var_one_over_one_minus_pgat_d * assign61530_e79876);
        (assign61530_e79877, (var_one_over_one_minus_pgat_d * (-((assign61530_e79874 * var_tmp__blk1560_dn6) / (var_tmp__blk1560 * var_tmp__blk1560)))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1558_dn7) * var_wdepnulrinvgat_d) * var_tmp__blk1560) - (assign61530_e79874 * var_tmp__blk1560_dn7)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1558_dn8) * var_wdepnulrinvgat_d) * var_tmp__blk1560) - (assign61530_e79874 * var_tmp__blk1560_dn8)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_pgat_d * (-((assign61530_e79874 * var_tmp__blk1560_dn9) / (var_tmp__blk1560 * var_tmp__blk1560)))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1558_dn11) * var_wdepnulrinvgat_d) * var_tmp__blk1560) - (assign61530_e79874 * var_tmp__blk1560_dn11)) / (var_tmp__blk1560 * var_tmp__blk1560))), (var_one_over_one_minus_pgat_d * (((((-var_vbbt__blk1558_dn12) * var_wdepnulrinvgat_d) * var_tmp__blk1560) - (assign61530_e79874 * var_tmp__blk1560_dn12)) / (var_tmp__blk1560 * var_tmp__blk1560))),)
    } else {
        (var_fmaxr__blk1585, var_fmaxr__blk1585_dn6, var_fmaxr__blk1585_dn7, var_fmaxr__blk1585_dn8, var_fmaxr__blk1585_dn9, var_fmaxr__blk1585_dn11, var_fmaxr__blk1585_dn12,)
    }
};
        var_fmaxr__blk1585 = assign61530_e79879;
        var_fmaxr__blk1585_dn6 = assign61530_e79879_d_n6;
        var_fmaxr__blk1585_dn7 = assign61530_e79879_d_n7;
        var_fmaxr__blk1585_dn8 = assign61530_e79879_d_n8;
        var_fmaxr__blk1585_dn9 = assign61530_e79879_d_n9;
        var_fmaxr__blk1585_dn11 = assign61530_e79879_d_n11;
        var_fmaxr__blk1585_dn12 = assign61530_e79879_d_n12;

        let assign61540_e79881: f64 = (-var_fbbtgat_d);
        let assign61540_e79883: f64 = (assign61540_e79881 / var_fmaxr__blk1585);
        let assign61540_e79884: f64 = (assign61540_e79883).abs();
        let assign61540_e79886: f64 = if assign61540_e79884 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1726 = assign61540_e79886;

        *var_erfcpos__blk1544_slot = var_erfcpos__blk1544;
        *var_erfcpos__blk1544_dn11_slot = var_erfcpos__blk1544_dn11;
        *var_erfcpos__blk1544_dn12_slot = var_erfcpos__blk1544_dn12;
        *var_erfcpos__blk1544_dn6_slot = var_erfcpos__blk1544_dn6;
        *var_erfcpos__blk1544_dn7_slot = var_erfcpos__blk1544_dn7;
        *var_erfcpos__blk1544_dn8_slot = var_erfcpos__blk1544_dn8;
        *var_erfcpos__blk1544_dn9_slot = var_erfcpos__blk1544_dn9;
        *var_erfctimesexpmtat__blk1582_slot = var_erfctimesexpmtat__blk1582;
        *var_erfctimesexpmtat__blk1582_dn11_slot = var_erfctimesexpmtat__blk1582_dn11;
        *var_erfctimesexpmtat__blk1582_dn12_slot = var_erfctimesexpmtat__blk1582_dn12;
        *var_erfctimesexpmtat__blk1582_dn6_slot = var_erfctimesexpmtat__blk1582_dn6;
        *var_erfctimesexpmtat__blk1582_dn7_slot = var_erfctimesexpmtat__blk1582_dn7;
        *var_erfctimesexpmtat__blk1582_dn8_slot = var_erfctimesexpmtat__blk1582_dn8;
        *var_erfctimesexpmtat__blk1582_dn9_slot = var_erfctimesexpmtat__blk1582_dn9;
        *var_fmaxr__blk1585_slot = var_fmaxr__blk1585;
        *var_fmaxr__blk1585_dn11_slot = var_fmaxr__blk1585_dn11;
        *var_fmaxr__blk1585_dn12_slot = var_fmaxr__blk1585_dn12;
        *var_fmaxr__blk1585_dn6_slot = var_fmaxr__blk1585_dn6;
        *var_fmaxr__blk1585_dn7_slot = var_fmaxr__blk1585_dn7;
        *var_fmaxr__blk1585_dn8_slot = var_fmaxr__blk1585_dn8;
        *var_fmaxr__blk1585_dn9_slot = var_fmaxr__blk1585_dn9;
        *var_gammamax__blk1583_slot = var_gammamax__blk1583;
        *var_gammamax__blk1583_dn11_slot = var_gammamax__blk1583_dn11;
        *var_gammamax__blk1583_dn12_slot = var_gammamax__blk1583_dn12;
        *var_gammamax__blk1583_dn6_slot = var_gammamax__blk1583_dn6;
        *var_gammamax__blk1583_dn7_slot = var_gammamax__blk1583_dn7;
        *var_gammamax__blk1583_dn8_slot = var_gammamax__blk1583_dn8;
        *var_gammamax__blk1583_dn9_slot = var_gammamax__blk1583_dn9;
        *var_guard1719_slot = var_guard1719;
        *var_guard1720_slot = var_guard1720;
        *var_guard1721_slot = var_guard1721;
        *var_guard1722_slot = var_guard1722;
        *var_guard1723_slot = var_guard1723;
        *var_guard1724_slot = var_guard1724;
        *var_guard1725_slot = var_guard1725;
        *var_guard1726_slot = var_guard1726;
        *var_ibbt__blk1584_slot = var_ibbt__blk1584;
        *var_ibbt__blk1584_dn11_slot = var_ibbt__blk1584_dn11;
        *var_ibbt__blk1584_dn12_slot = var_ibbt__blk1584_dn12;
        *var_ibbt__blk1584_dn6_slot = var_ibbt__blk1584_dn6;
        *var_ibbt__blk1584_dn7_slot = var_ibbt__blk1584_dn7;
        *var_ibbt__blk1584_dn8_slot = var_ibbt__blk1584_dn8;
        *var_ibbt__blk1584_dn9_slot = var_ibbt__blk1584_dn9;
        *var_itat__blk1569_slot = var_itat__blk1569;
        *var_itat__blk1569_dn11_slot = var_itat__blk1569_dn11;
        *var_itat__blk1569_dn12_slot = var_itat__blk1569_dn12;
        *var_itat__blk1569_dn6_slot = var_itat__blk1569_dn6;
        *var_itat__blk1569_dn7_slot = var_itat__blk1569_dn7;
        *var_itat__blk1569_dn8_slot = var_itat__blk1569_dn8;
        *var_itat__blk1569_dn9_slot = var_itat__blk1569_dn9;
        *var_ktat__blk1578_slot = var_ktat__blk1578;
        *var_ktat__blk1578_dn11_slot = var_ktat__blk1578_dn11;
        *var_ktat__blk1578_dn12_slot = var_ktat__blk1578_dn12;
        *var_ktat__blk1578_dn6_slot = var_ktat__blk1578_dn6;
        *var_ktat__blk1578_dn7_slot = var_ktat__blk1578_dn7;
        *var_ktat__blk1578_dn8_slot = var_ktat__blk1578_dn8;
        *var_ktat__blk1578_dn9_slot = var_ktat__blk1578_dn9;
        *var_ltat__blk1579_slot = var_ltat__blk1579;
        *var_ltat__blk1579_dn11_slot = var_ltat__blk1579_dn11;
        *var_ltat__blk1579_dn12_slot = var_ltat__blk1579_dn12;
        *var_ltat__blk1579_dn6_slot = var_ltat__blk1579_dn6;
        *var_ltat__blk1579_dn7_slot = var_ltat__blk1579_dn7;
        *var_ltat__blk1579_dn8_slot = var_ltat__blk1579_dn8;
        *var_ltat__blk1579_dn9_slot = var_ltat__blk1579_dn9;
        *var_mtat__blk1580_slot = var_mtat__blk1580;
        *var_mtat__blk1580_dn11_slot = var_mtat__blk1580_dn11;
        *var_mtat__blk1580_dn12_slot = var_mtat__blk1580_dn12;
        *var_mtat__blk1580_dn6_slot = var_mtat__blk1580_dn6;
        *var_mtat__blk1580_dn7_slot = var_mtat__blk1580_dn7;
        *var_mtat__blk1580_dn8_slot = var_mtat__blk1580_dn8;
        *var_mtat__blk1580_dn9_slot = var_mtat__blk1580_dn9;
        *var_sqrtumax__blk1574_slot = var_sqrtumax__blk1574;
        *var_sqrtumax__blk1574_dn11_slot = var_sqrtumax__blk1574_dn11;
        *var_sqrtumax__blk1574_dn12_slot = var_sqrtumax__blk1574_dn12;
        *var_sqrtumax__blk1574_dn6_slot = var_sqrtumax__blk1574_dn6;
        *var_sqrtumax__blk1574_dn7_slot = var_sqrtumax__blk1574_dn7;
        *var_sqrtumax__blk1574_dn8_slot = var_sqrtumax__blk1574_dn8;
        *var_sqrtumax__blk1574_dn9_slot = var_sqrtumax__blk1574_dn9;
        *var_terfc__blk1543_slot = var_terfc__blk1543;
        *var_terfc__blk1543_dn11_slot = var_terfc__blk1543_dn11;
        *var_terfc__blk1543_dn12_slot = var_terfc__blk1543_dn12;
        *var_terfc__blk1543_dn6_slot = var_terfc__blk1543_dn6;
        *var_terfc__blk1543_dn7_slot = var_terfc__blk1543_dn7;
        *var_terfc__blk1543_dn8_slot = var_terfc__blk1543_dn8;
        *var_terfc__blk1543_dn9_slot = var_terfc__blk1543_dn9;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_umaxpoweronepointfive__blk1575_slot = var_umaxpoweronepointfive__blk1575;
        *var_umaxpoweronepointfive__blk1575_dn11_slot = var_umaxpoweronepointfive__blk1575_dn11;
        *var_umaxpoweronepointfive__blk1575_dn12_slot = var_umaxpoweronepointfive__blk1575_dn12;
        *var_umaxpoweronepointfive__blk1575_dn6_slot = var_umaxpoweronepointfive__blk1575_dn6;
        *var_umaxpoweronepointfive__blk1575_dn7_slot = var_umaxpoweronepointfive__blk1575_dn7;
        *var_umaxpoweronepointfive__blk1575_dn8_slot = var_umaxpoweronepointfive__blk1575_dn8;
        *var_umaxpoweronepointfive__blk1575_dn9_slot = var_umaxpoweronepointfive__blk1575_dn9;
        *var_wgamma__blk1576_slot = var_wgamma__blk1576;
        *var_wgamma__blk1576_dn11_slot = var_wgamma__blk1576_dn11;
        *var_wgamma__blk1576_dn12_slot = var_wgamma__blk1576_dn12;
        *var_wgamma__blk1576_dn6_slot = var_wgamma__blk1576_dn6;
        *var_wgamma__blk1576_dn7_slot = var_wgamma__blk1576_dn7;
        *var_wgamma__blk1576_dn8_slot = var_wgamma__blk1576_dn8;
        *var_wgamma__blk1576_dn9_slot = var_wgamma__blk1576_dn9;
        *var_wtat__blk1577_slot = var_wtat__blk1577;
        *var_wtat__blk1577_dn11_slot = var_wtat__blk1577_dn11;
        *var_wtat__blk1577_dn12_slot = var_wtat__blk1577_dn12;
        *var_wtat__blk1577_dn6_slot = var_wtat__blk1577_dn6;
        *var_wtat__blk1577_dn7_slot = var_wtat__blk1577_dn7;
        *var_wtat__blk1577_dn8_slot = var_wtat__blk1577_dn8;
        *var_wtat__blk1577_dn9_slot = var_wtat__blk1577_dn9;
        *var_xerfc__blk1581_slot = var_xerfc__blk1581;
        *var_xerfc__blk1581_dn11_slot = var_xerfc__blk1581_dn11;
        *var_xerfc__blk1581_dn12_slot = var_xerfc__blk1581_dn12;
        *var_xerfc__blk1581_dn6_slot = var_xerfc__blk1581_dn6;
        *var_xerfc__blk1581_dn7_slot = var_xerfc__blk1581_dn7;
        *var_xerfc__blk1581_dn8_slot = var_xerfc__blk1581_dn8;
        *var_xerfc__blk1581_dn9_slot = var_xerfc__blk1581_dn9;
        *var_ysq__blk1542_slot = var_ysq__blk1542;
        *var_ysq__blk1542_dn11_slot = var_ysq__blk1542_dn11;
        *var_ysq__blk1542_dn12_slot = var_ysq__blk1542_dn12;
        *var_ysq__blk1542_dn6_slot = var_ysq__blk1542_dn6;
        *var_ysq__blk1542_dn7_slot = var_ysq__blk1542_dn7;
        *var_ysq__blk1542_dn8_slot = var_ysq__blk1542_dn8;
        *var_ysq__blk1542_dn9_slot = var_ysq__blk1542_dn9;
    }

    pub(super) fn stamp_transient_block_146(
        p: &Parameters,
        var_alphaav: f64,
        var_anugatd_i: f64,
        var_cbbtgatd_i: f64,
        var_fbbtgat_d: f64,
        var_fbbtgat_d_dn6: f64,
        var_fbbtgat_d_dn7: f64,
        var_fbbtgat_d_dn8: f64,
        var_fbbtgat_d_dn9: f64,
        var_fmaxr__blk1585: f64,
        var_fmaxr__blk1585_dn11: f64,
        var_fmaxr__blk1585_dn12: f64,
        var_fmaxr__blk1585_dn6: f64,
        var_fmaxr__blk1585_dn7: f64,
        var_fmaxr__blk1585_dn8: f64,
        var_fmaxr__blk1585_dn9: f64,
        var_fstopgat_d: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1714: f64,
        var_guard1724: f64,
        var_guard1726: f64,
        var_id__blk1561: f64,
        var_id__blk1561_dn11: f64,
        var_id__blk1561_dn12: f64,
        var_id__blk1561_dn7: f64,
        var_id__blk1561_dn8: f64,
        var_isrh__blk1562: f64,
        var_isrh__blk1562_dn11: f64,
        var_isrh__blk1562_dn12: f64,
        var_isrh__blk1562_dn6: f64,
        var_isrh__blk1562_dn7: f64,
        var_isrh__blk1562_dn8: f64,
        var_isrh__blk1562_dn9: f64,
        var_itat__blk1569: f64,
        var_itat__blk1569_dn11: f64,
        var_itat__blk1569_dn12: f64,
        var_itat__blk1569_dn6: f64,
        var_itat__blk1569_dn7: f64,
        var_itat__blk1569_dn8: f64,
        var_itat__blk1569_dn9: f64,
        var_one_minus_pgat_d: f64,
        var_pbrgatd_i: f64,
        var_qpref2gat_d: f64,
        var_qprefgat_d: f64,
        var_slopegat_d: f64,
        var_slopegat_d_dn6: f64,
        var_slopegat_d_dn7: f64,
        var_slopegat_d_dn8: f64,
        var_slopegat_d_dn9: f64,
        var_swgat2nd_d: f64,
        var_vav__blk1559: f64,
        var_vav__blk1559_dn11: f64,
        var_vav__blk1559_dn12: f64,
        var_vav__blk1559_dn7: f64,
        var_vav__blk1559_dn8: f64,
        var_vbiinvgat_d: f64,
        var_vbrgat_var_d: f64,
        var_vbrgat_var_d_dn6: f64,
        var_vbrgat_var_d_dn7: f64,
        var_vbrgat_var_d_dn8: f64,
        var_vbrgat_var_d_dn9: f64,
        var_vbrinvgat_d: f64,
        var_vbrinvgat_d_dn6: f64,
        var_vbrinvgat_d_dn7: f64,
        var_vbrinvgat_d_dn8: f64,
        var_vbrinvgat_d_dn9: f64,
        var_vch_d: f64,
        var_vfmin_d: f64,
        var_vjun_d: f64,
        var_vjun_d_dn12: f64,
        var_vjun_d_dn8: f64,
        var_vtrgatd_i: f64,
        var_fbreakdown__blk1586_slot: &mut f64,
        var_fbreakdown__blk1586_dn11_slot: &mut f64,
        var_fbreakdown__blk1586_dn12_slot: &mut f64,
        var_fbreakdown__blk1586_dn6_slot: &mut f64,
        var_fbreakdown__blk1586_dn7_slot: &mut f64,
        var_fbreakdown__blk1586_dn8_slot: &mut f64,
        var_fbreakdown__blk1586_dn9_slot: &mut f64,
        var_guard1727_slot: &mut f64,
        var_guard1728_slot: &mut f64,
        var_guard1729_slot: &mut f64,
        var_guard1730_slot: &mut f64,
        var_guard1731_slot: &mut f64,
        var_guard1732_slot: &mut f64,
        var_h1__blk1545_slot: &mut f64,
        var_h2__blk1546_slot: &mut f64,
        var_h2d__blk1547_slot: &mut f64,
        var_h2d__blk1547_dn11_slot: &mut f64,
        var_h2d__blk1547_dn12_slot: &mut f64,
        var_h2d__blk1547_dn7_slot: &mut f64,
        var_h2d__blk1547_dn8_slot: &mut f64,
        var_h3__blk1548_slot: &mut f64,
        var_h3__blk1548_dn11_slot: &mut f64,
        var_h3__blk1548_dn12_slot: &mut f64,
        var_h3__blk1548_dn7_slot: &mut f64,
        var_h3__blk1548_dn8_slot: &mut f64,
        var_h4__blk1549_slot: &mut f64,
        var_h4__blk1549_dn11_slot: &mut f64,
        var_h4__blk1549_dn12_slot: &mut f64,
        var_h4__blk1549_dn7_slot: &mut f64,
        var_h4__blk1549_dn8_slot: &mut f64,
        var_h5__blk1550_slot: &mut f64,
        var_h5__blk1550_dn11_slot: &mut f64,
        var_h5__blk1550_dn12_slot: &mut f64,
        var_h5__blk1550_dn7_slot: &mut f64,
        var_h5__blk1550_dn8_slot: &mut f64,
        var_ibbt__blk1584_slot: &mut f64,
        var_ibbt__blk1584_dn11_slot: &mut f64,
        var_ibbt__blk1584_dn12_slot: &mut f64,
        var_ibbt__blk1584_dn6_slot: &mut f64,
        var_ibbt__blk1584_dn7_slot: &mut f64,
        var_ibbt__blk1584_dn8_slot: &mut f64,
        var_ibbt__blk1584_dn9_slot: &mut f64,
        var_ijungat_d_slot: &mut f64,
        var_ijungat_d_dn11_slot: &mut f64,
        var_ijungat_d_dn12_slot: &mut f64,
        var_ijungat_d_dn6_slot: &mut f64,
        var_ijungat_d_dn7_slot: &mut f64,
        var_ijungat_d_dn8_slot: &mut f64,
        var_ijungat_d_dn9_slot: &mut f64,
        var_nu__blk1587_slot: &mut f64,
        var_nu__blk1587_dn11_slot: &mut f64,
        var_nu__blk1587_dn12_slot: &mut f64,
        var_nu__blk1587_dn7_slot: &mut f64,
        var_nu__blk1587_dn8_slot: &mut f64,
        var_qjungat_d_slot: &mut f64,
        var_qjungat_d_dn11_slot: &mut f64,
        var_qjungat_d_dn12_slot: &mut f64,
        var_qjungat_d_dn6_slot: &mut f64,
        var_qjungat_d_dn7_slot: &mut f64,
        var_qjungat_d_dn8_slot: &mut f64,
        var_qjungat_d_dn9_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_vjtmp_slot: &mut f64,
        var_vjtmp_dn11_slot: &mut f64,
        var_vjtmp_dn12_slot: &mut f64,
        var_vjtmp_dn7_slot: &mut f64,
        var_vjtmp_dn8_slot: &mut f64,
    ) {
        let mut var_fbreakdown__blk1586: f64 = *var_fbreakdown__blk1586_slot;
        let mut var_fbreakdown__blk1586_dn11: f64 = *var_fbreakdown__blk1586_dn11_slot;
        let mut var_fbreakdown__blk1586_dn12: f64 = *var_fbreakdown__blk1586_dn12_slot;
        let mut var_fbreakdown__blk1586_dn6: f64 = *var_fbreakdown__blk1586_dn6_slot;
        let mut var_fbreakdown__blk1586_dn7: f64 = *var_fbreakdown__blk1586_dn7_slot;
        let mut var_fbreakdown__blk1586_dn8: f64 = *var_fbreakdown__blk1586_dn8_slot;
        let mut var_fbreakdown__blk1586_dn9: f64 = *var_fbreakdown__blk1586_dn9_slot;
        let mut var_guard1727: f64 = *var_guard1727_slot;
        let mut var_guard1728: f64 = *var_guard1728_slot;
        let mut var_guard1729: f64 = *var_guard1729_slot;
        let mut var_guard1730: f64 = *var_guard1730_slot;
        let mut var_guard1731: f64 = *var_guard1731_slot;
        let mut var_guard1732: f64 = *var_guard1732_slot;
        let mut var_h1__blk1545: f64 = *var_h1__blk1545_slot;
        let mut var_h2__blk1546: f64 = *var_h2__blk1546_slot;
        let mut var_h2d__blk1547: f64 = *var_h2d__blk1547_slot;
        let mut var_h2d__blk1547_dn11: f64 = *var_h2d__blk1547_dn11_slot;
        let mut var_h2d__blk1547_dn12: f64 = *var_h2d__blk1547_dn12_slot;
        let mut var_h2d__blk1547_dn7: f64 = *var_h2d__blk1547_dn7_slot;
        let mut var_h2d__blk1547_dn8: f64 = *var_h2d__blk1547_dn8_slot;
        let mut var_h3__blk1548: f64 = *var_h3__blk1548_slot;
        let mut var_h3__blk1548_dn11: f64 = *var_h3__blk1548_dn11_slot;
        let mut var_h3__blk1548_dn12: f64 = *var_h3__blk1548_dn12_slot;
        let mut var_h3__blk1548_dn7: f64 = *var_h3__blk1548_dn7_slot;
        let mut var_h3__blk1548_dn8: f64 = *var_h3__blk1548_dn8_slot;
        let mut var_h4__blk1549: f64 = *var_h4__blk1549_slot;
        let mut var_h4__blk1549_dn11: f64 = *var_h4__blk1549_dn11_slot;
        let mut var_h4__blk1549_dn12: f64 = *var_h4__blk1549_dn12_slot;
        let mut var_h4__blk1549_dn7: f64 = *var_h4__blk1549_dn7_slot;
        let mut var_h4__blk1549_dn8: f64 = *var_h4__blk1549_dn8_slot;
        let mut var_h5__blk1550: f64 = *var_h5__blk1550_slot;
        let mut var_h5__blk1550_dn11: f64 = *var_h5__blk1550_dn11_slot;
        let mut var_h5__blk1550_dn12: f64 = *var_h5__blk1550_dn12_slot;
        let mut var_h5__blk1550_dn7: f64 = *var_h5__blk1550_dn7_slot;
        let mut var_h5__blk1550_dn8: f64 = *var_h5__blk1550_dn8_slot;
        let mut var_ibbt__blk1584: f64 = *var_ibbt__blk1584_slot;
        let mut var_ibbt__blk1584_dn11: f64 = *var_ibbt__blk1584_dn11_slot;
        let mut var_ibbt__blk1584_dn12: f64 = *var_ibbt__blk1584_dn12_slot;
        let mut var_ibbt__blk1584_dn6: f64 = *var_ibbt__blk1584_dn6_slot;
        let mut var_ibbt__blk1584_dn7: f64 = *var_ibbt__blk1584_dn7_slot;
        let mut var_ibbt__blk1584_dn8: f64 = *var_ibbt__blk1584_dn8_slot;
        let mut var_ibbt__blk1584_dn9: f64 = *var_ibbt__blk1584_dn9_slot;
        let mut var_ijungat_d: f64 = *var_ijungat_d_slot;
        let mut var_ijungat_d_dn11: f64 = *var_ijungat_d_dn11_slot;
        let mut var_ijungat_d_dn12: f64 = *var_ijungat_d_dn12_slot;
        let mut var_ijungat_d_dn6: f64 = *var_ijungat_d_dn6_slot;
        let mut var_ijungat_d_dn7: f64 = *var_ijungat_d_dn7_slot;
        let mut var_ijungat_d_dn8: f64 = *var_ijungat_d_dn8_slot;
        let mut var_ijungat_d_dn9: f64 = *var_ijungat_d_dn9_slot;
        let mut var_nu__blk1587: f64 = *var_nu__blk1587_slot;
        let mut var_nu__blk1587_dn11: f64 = *var_nu__blk1587_dn11_slot;
        let mut var_nu__blk1587_dn12: f64 = *var_nu__blk1587_dn12_slot;
        let mut var_nu__blk1587_dn7: f64 = *var_nu__blk1587_dn7_slot;
        let mut var_nu__blk1587_dn8: f64 = *var_nu__blk1587_dn8_slot;
        let mut var_qjungat_d: f64 = *var_qjungat_d_slot;
        let mut var_qjungat_d_dn11: f64 = *var_qjungat_d_dn11_slot;
        let mut var_qjungat_d_dn12: f64 = *var_qjungat_d_dn12_slot;
        let mut var_qjungat_d_dn6: f64 = *var_qjungat_d_dn6_slot;
        let mut var_qjungat_d_dn7: f64 = *var_qjungat_d_dn7_slot;
        let mut var_qjungat_d_dn8: f64 = *var_qjungat_d_dn8_slot;
        let mut var_qjungat_d_dn9: f64 = *var_qjungat_d_dn9_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_vjtmp: f64 = *var_vjtmp_slot;
        let mut var_vjtmp_dn11: f64 = *var_vjtmp_dn11_slot;
        let mut var_vjtmp_dn12: f64 = *var_vjtmp_dn12_slot;
        let mut var_vjtmp_dn7: f64 = *var_vjtmp_dn7_slot;
        let mut var_vjtmp_dn8: f64 = *var_vjtmp_dn8_slot;

        let (assign61550_e79905, assign61550_e79905_d_n6, assign61550_e79905_d_n7, assign61550_e79905_d_n8, assign61550_e79905_d_n9, assign61550_e79905_d_n11, assign61550_e79905_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1724 == 0.0)) && (var_guard1726 != 0.0)) {
        let assign61550_e79900: f64 = (-var_fbbtgat_d);
        let assign61550_e79902: f64 = (assign61550_e79900 / var_fmaxr__blk1585);
        let assign61550_e79903: f64 = (assign61550_e79902).exp();
        (assign61550_e79903, (assign61550_e79903 * ((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1585) - (assign61550_e79900 * var_fmaxr__blk1585_dn6)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))), (assign61550_e79903 * ((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1585) - (assign61550_e79900 * var_fmaxr__blk1585_dn7)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))), (assign61550_e79903 * ((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1585) - (assign61550_e79900 * var_fmaxr__blk1585_dn8)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))), (assign61550_e79903 * ((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1585) - (assign61550_e79900 * var_fmaxr__blk1585_dn9)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))), (assign61550_e79903 * (-((assign61550_e79900 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))), (assign61550_e79903 * (-((assign61550_e79900 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61550_e79905;
        var_tmp__blk1560_dn6 = assign61550_e79905_d_n6;
        var_tmp__blk1560_dn7 = assign61550_e79905_d_n7;
        var_tmp__blk1560_dn8 = assign61550_e79905_d_n8;
        var_tmp__blk1560_dn9 = assign61550_e79905_d_n9;
        var_tmp__blk1560_dn11 = assign61550_e79905_d_n11;
        var_tmp__blk1560_dn12 = assign61550_e79905_d_n12;

        let assign61560_e79907: f64 = (-var_fbbtgat_d);
        let assign61560_e79909: f64 = (assign61560_e79907 / var_fmaxr__blk1585);
        let assign61560_e79911: f64 = if assign61560_e79909 < 0.0 { 1.0 } else { 0.0 };
        var_guard1727 = assign61560_e79911;

        let (assign61570_e79963, assign61570_e79963_d_n6, assign61570_e79963_d_n7, assign61570_e79963_d_n8, assign61570_e79963_d_n9, assign61570_e79963_d_n11, assign61570_e79963_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1724 == 0.0)) && (var_guard1726 == 0.0)) && (var_guard1727 != 0.0)) {
        let assign61570_e79930: f64 = (-230.25850929940458);
        let assign61570_e79932: f64 = (-var_fbbtgat_d);
        let assign61570_e79934: f64 = (assign61570_e79932 / var_fmaxr__blk1585);
        let assign61570_e79935: f64 = (assign61570_e79930 - assign61570_e79934);
        let assign61570_e79939: f64 = (-230.25850929940458);
        let assign61570_e79941: f64 = (-var_fbbtgat_d);
        let assign61570_e79943: f64 = (assign61570_e79941 / var_fmaxr__blk1585);
        let assign61570_e79944: f64 = (assign61570_e79939 - assign61570_e79943);
        let assign61570_e79947: f64 = (-230.25850929940458);
        let assign61570_e79949: f64 = (-var_fbbtgat_d);
        let assign61570_e79951: f64 = (assign61570_e79949 / var_fmaxr__blk1585);
        let assign61570_e79952: f64 = (assign61570_e79947 - assign61570_e79951);
        let assign61570_e79954: f64 = (assign61570_e79952 * 0.3333333333333333);
        let assign61570_e79955: f64 = (1.0 + assign61570_e79954);
        let assign61570_e79956: f64 = (assign61570_e79944 * assign61570_e79955);
        let assign61570_e79957: f64 = (0.5 * assign61570_e79956);
        let assign61570_e79958: f64 = (1.0 + assign61570_e79957);
        let assign61570_e79959: f64 = (assign61570_e79935 * assign61570_e79958);
        let assign61570_e79960: f64 = (1.0 + assign61570_e79959);
        let assign61570_e79961: f64 = (1e-100 / assign61570_e79960);
        (assign61570_e79961, (-((1e-100 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1585) - (assign61570_e79932 * var_fmaxr__blk1585_dn6)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61570_e79958) + (assign61570_e79935 * (0.5 * (((-((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1585) - (assign61570_e79941 * var_fmaxr__blk1585_dn6)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61570_e79955) + (assign61570_e79944 * ((-((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1585) - (assign61570_e79949 * var_fmaxr__blk1585_dn6)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))) / (assign61570_e79960 * assign61570_e79960))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1585) - (assign61570_e79932 * var_fmaxr__blk1585_dn7)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61570_e79958) + (assign61570_e79935 * (0.5 * (((-((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1585) - (assign61570_e79941 * var_fmaxr__blk1585_dn7)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61570_e79955) + (assign61570_e79944 * ((-((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1585) - (assign61570_e79949 * var_fmaxr__blk1585_dn7)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))) / (assign61570_e79960 * assign61570_e79960))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1585) - (assign61570_e79932 * var_fmaxr__blk1585_dn8)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61570_e79958) + (assign61570_e79935 * (0.5 * (((-((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1585) - (assign61570_e79941 * var_fmaxr__blk1585_dn8)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61570_e79955) + (assign61570_e79944 * ((-((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1585) - (assign61570_e79949 * var_fmaxr__blk1585_dn8)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))) / (assign61570_e79960 * assign61570_e79960))), (-((1e-100 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1585) - (assign61570_e79932 * var_fmaxr__blk1585_dn9)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61570_e79958) + (assign61570_e79935 * (0.5 * (((-((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1585) - (assign61570_e79941 * var_fmaxr__blk1585_dn9)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61570_e79955) + (assign61570_e79944 * ((-((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1585) - (assign61570_e79949 * var_fmaxr__blk1585_dn9)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))) / (assign61570_e79960 * assign61570_e79960))), (-((1e-100 * (((-(-((assign61570_e79932 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign61570_e79958) + (assign61570_e79935 * (0.5 * (((-(-((assign61570_e79941 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign61570_e79955) + (assign61570_e79944 * ((-(-((assign61570_e79949 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign61570_e79960 * assign61570_e79960))), (-((1e-100 * (((-(-((assign61570_e79932 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign61570_e79958) + (assign61570_e79935 * (0.5 * (((-(-((assign61570_e79941 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * assign61570_e79955) + (assign61570_e79944 * ((-(-((assign61570_e79949 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)))) * 0.3333333333333333))))))) / (assign61570_e79960 * assign61570_e79960))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61570_e79963;
        var_tmp__blk1560_dn6 = assign61570_e79963_d_n6;
        var_tmp__blk1560_dn7 = assign61570_e79963_d_n7;
        var_tmp__blk1560_dn8 = assign61570_e79963_d_n8;
        var_tmp__blk1560_dn9 = assign61570_e79963_d_n9;
        var_tmp__blk1560_dn11 = assign61570_e79963_d_n11;
        var_tmp__blk1560_dn12 = assign61570_e79963_d_n12;

        let (assign61580_e80013, assign61580_e80013_d_n6, assign61580_e80013_d_n7, assign61580_e80013_d_n8, assign61580_e80013_d_n9, assign61580_e80013_d_n11, assign61580_e80013_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1724 == 0.0)) && (var_guard1726 == 0.0)) && (var_guard1727 == 0.0)) {
        let assign61580_e79983: f64 = (-var_fbbtgat_d);
        let assign61580_e79985: f64 = (assign61580_e79983 / var_fmaxr__blk1585);
        let assign61580_e79987: f64 = (assign61580_e79985 - 230.25850929940458);
        let assign61580_e79991: f64 = (-var_fbbtgat_d);
        let assign61580_e79993: f64 = (assign61580_e79991 / var_fmaxr__blk1585);
        let assign61580_e79995: f64 = (assign61580_e79993 - 230.25850929940458);
        let assign61580_e79998: f64 = (-var_fbbtgat_d);
        let assign61580_e80000: f64 = (assign61580_e79998 / var_fmaxr__blk1585);
        let assign61580_e80002: f64 = (assign61580_e80000 - 230.25850929940458);
        let assign61580_e80004: f64 = (assign61580_e80002 * 0.3333333333333333);
        let assign61580_e80005: f64 = (1.0 + assign61580_e80004);
        let assign61580_e80006: f64 = (assign61580_e79995 * assign61580_e80005);
        let assign61580_e80007: f64 = (0.5 * assign61580_e80006);
        let assign61580_e80008: f64 = (1.0 + assign61580_e80007);
        let assign61580_e80009: f64 = (assign61580_e79987 * assign61580_e80008);
        let assign61580_e80010: f64 = (1.0 + assign61580_e80009);
        let assign61580_e80011: f64 = (1e100 * assign61580_e80010);
        (assign61580_e80011, (1e100 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1585) - (assign61580_e79983 * var_fmaxr__blk1585_dn6)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * assign61580_e80008) + (assign61580_e79987 * (0.5 * ((((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1585) - (assign61580_e79991 * var_fmaxr__blk1585_dn6)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * assign61580_e80005) + (assign61580_e79995 * (((((-var_fbbtgat_d_dn6) * var_fmaxr__blk1585) - (assign61580_e79998 * var_fmaxr__blk1585_dn6)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1585) - (assign61580_e79983 * var_fmaxr__blk1585_dn7)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * assign61580_e80008) + (assign61580_e79987 * (0.5 * ((((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1585) - (assign61580_e79991 * var_fmaxr__blk1585_dn7)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * assign61580_e80005) + (assign61580_e79995 * (((((-var_fbbtgat_d_dn7) * var_fmaxr__blk1585) - (assign61580_e79998 * var_fmaxr__blk1585_dn7)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1585) - (assign61580_e79983 * var_fmaxr__blk1585_dn8)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * assign61580_e80008) + (assign61580_e79987 * (0.5 * ((((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1585) - (assign61580_e79991 * var_fmaxr__blk1585_dn8)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * assign61580_e80005) + (assign61580_e79995 * (((((-var_fbbtgat_d_dn8) * var_fmaxr__blk1585) - (assign61580_e79998 * var_fmaxr__blk1585_dn8)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1585) - (assign61580_e79983 * var_fmaxr__blk1585_dn9)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * assign61580_e80008) + (assign61580_e79987 * (0.5 * ((((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1585) - (assign61580_e79991 * var_fmaxr__blk1585_dn9)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * assign61580_e80005) + (assign61580_e79995 * (((((-var_fbbtgat_d_dn9) * var_fmaxr__blk1585) - (assign61580_e79998 * var_fmaxr__blk1585_dn9)) / (var_fmaxr__blk1585 * var_fmaxr__blk1585)) * 0.3333333333333333))))))), (1e100 * (((-((assign61580_e79983 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61580_e80008) + (assign61580_e79987 * (0.5 * (((-((assign61580_e79991 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61580_e80005) + (assign61580_e79995 * ((-((assign61580_e79998 * var_fmaxr__blk1585_dn11) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))), (1e100 * (((-((assign61580_e79983 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61580_e80008) + (assign61580_e79987 * (0.5 * (((-((assign61580_e79991 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * assign61580_e80005) + (assign61580_e79995 * ((-((assign61580_e79998 * var_fmaxr__blk1585_dn12) / (var_fmaxr__blk1585 * var_fmaxr__blk1585))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61580_e80013;
        var_tmp__blk1560_dn6 = assign61580_e80013_d_n6;
        var_tmp__blk1560_dn7 = assign61580_e80013_d_n7;
        var_tmp__blk1560_dn8 = assign61580_e80013_d_n8;
        var_tmp__blk1560_dn9 = assign61580_e80013_d_n9;
        var_tmp__blk1560_dn11 = assign61580_e80013_d_n11;
        var_tmp__blk1560_dn12 = assign61580_e80013_d_n12;

        let (assign61590_e80034, assign61590_e80034_d_n6, assign61590_e80034_d_n7, assign61590_e80034_d_n8, assign61590_e80034_d_n9, assign61590_e80034_d_n11, assign61590_e80034_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1724 == 0.0)) {
        let assign61590_e80027: f64 = (var_vjun_d * var_fmaxr__blk1585);
        let assign61590_e80029: f64 = (assign61590_e80027 * var_fmaxr__blk1585);
        let assign61590_e80031: f64 = (assign61590_e80029 * var_tmp__blk1560);
        let assign61590_e80032: f64 = (var_cbbtgatd_i * assign61590_e80031);
        (assign61590_e80032, (var_cbbtgatd_i * (((((var_vjun_d * var_fmaxr__blk1585_dn6) * var_fmaxr__blk1585) + (assign61590_e80027 * var_fmaxr__blk1585_dn6)) * var_tmp__blk1560) + (assign61590_e80029 * var_tmp__blk1560_dn6))), (var_cbbtgatd_i * (((((var_vjun_d * var_fmaxr__blk1585_dn7) * var_fmaxr__blk1585) + (assign61590_e80027 * var_fmaxr__blk1585_dn7)) * var_tmp__blk1560) + (assign61590_e80029 * var_tmp__blk1560_dn7))), (var_cbbtgatd_i * ((((((var_vjun_d_dn8 * var_fmaxr__blk1585) + (var_vjun_d * var_fmaxr__blk1585_dn8)) * var_fmaxr__blk1585) + (assign61590_e80027 * var_fmaxr__blk1585_dn8)) * var_tmp__blk1560) + (assign61590_e80029 * var_tmp__blk1560_dn8))), (var_cbbtgatd_i * (((((var_vjun_d * var_fmaxr__blk1585_dn9) * var_fmaxr__blk1585) + (assign61590_e80027 * var_fmaxr__blk1585_dn9)) * var_tmp__blk1560) + (assign61590_e80029 * var_tmp__blk1560_dn9))), (var_cbbtgatd_i * (((((var_vjun_d * var_fmaxr__blk1585_dn11) * var_fmaxr__blk1585) + (assign61590_e80027 * var_fmaxr__blk1585_dn11)) * var_tmp__blk1560) + (assign61590_e80029 * var_tmp__blk1560_dn11))), (var_cbbtgatd_i * ((((((var_vjun_d_dn12 * var_fmaxr__blk1585) + (var_vjun_d * var_fmaxr__blk1585_dn12)) * var_fmaxr__blk1585) + (assign61590_e80027 * var_fmaxr__blk1585_dn12)) * var_tmp__blk1560) + (assign61590_e80029 * var_tmp__blk1560_dn12))),)
    } else {
        (var_ibbt__blk1584, var_ibbt__blk1584_dn6, var_ibbt__blk1584_dn7, var_ibbt__blk1584_dn8, var_ibbt__blk1584_dn9, var_ibbt__blk1584_dn11, var_ibbt__blk1584_dn12,)
    }
};
        var_ibbt__blk1584 = assign61590_e80034;
        var_ibbt__blk1584_dn6 = assign61590_e80034_d_n6;
        var_ibbt__blk1584_dn7 = assign61590_e80034_d_n7;
        var_ibbt__blk1584_dn8 = assign61590_e80034_d_n8;
        var_ibbt__blk1584_dn9 = assign61590_e80034_d_n9;
        var_ibbt__blk1584_dn11 = assign61590_e80034_d_n11;
        var_ibbt__blk1584_dn12 = assign61590_e80034_d_n12;

        let assign61600_e80037: f64 = if var_vbrgat_var_d > 1000.0 { 1.0 } else { 0.0 };
        var_guard1728 = assign61600_e80037;

        let (assign61610_e80049, assign61610_e80049_d_n6, assign61610_e80049_d_n7, assign61610_e80049_d_n8, assign61610_e80049_d_n9, assign61610_e80049_d_n11, assign61610_e80049_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1728 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown__blk1586, var_fbreakdown__blk1586_dn6, var_fbreakdown__blk1586_dn7, var_fbreakdown__blk1586_dn8, var_fbreakdown__blk1586_dn9, var_fbreakdown__blk1586_dn11, var_fbreakdown__blk1586_dn12,)
    }
};
        var_fbreakdown__blk1586 = assign61610_e80049;
        var_fbreakdown__blk1586_dn6 = assign61610_e80049_d_n6;
        var_fbreakdown__blk1586_dn7 = assign61610_e80049_d_n7;
        var_fbreakdown__blk1586_dn8 = assign61610_e80049_d_n8;
        var_fbreakdown__blk1586_dn9 = assign61610_e80049_d_n9;
        var_fbreakdown__blk1586_dn11 = assign61610_e80049_d_n11;
        var_fbreakdown__blk1586_dn12 = assign61610_e80049_d_n12;

        let assign61620_e80052: f64 = (-var_alphaav);
        let assign61620_e80054: f64 = (assign61620_e80052 * var_vbrgat_var_d);
        let assign61620_e80055: f64 = if var_vav__blk1559 > assign61620_e80054 { 1.0 } else { 0.0 };
        var_guard1729 = assign61620_e80055;

        let assign61630_e80058: f64 = if var_pbrgatd_i == 4.0 { 1.0 } else { 0.0 };
        var_guard1730 = assign61630_e80058;

        let (assign61640_e80089, assign61640_e80089_d_n6, assign61640_e80089_d_n7, assign61640_e80089_d_n8, assign61640_e80089_d_n9, assign61640_e80089_d_n11, assign61640_e80089_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1728 == 0.0)) && (var_guard1729 != 0.0)) && (var_guard1730 != 0.0)) {
        let assign61640_e80075: f64 = (var_vav__blk1559 * var_vbrinvgat_d);
        let assign61640_e80078: f64 = (var_vav__blk1559 * var_vbrinvgat_d);
        let assign61640_e80079: f64 = (assign61640_e80075 * assign61640_e80078);
        let assign61640_e80082: f64 = (var_vav__blk1559 * var_vbrinvgat_d);
        let assign61640_e80083: f64 = (assign61640_e80079 * assign61640_e80082);
        let assign61640_e80086: f64 = (var_vav__blk1559 * var_vbrinvgat_d);
        let assign61640_e80087: f64 = (assign61640_e80083 * assign61640_e80086);
        (assign61640_e80087, (((((((var_vav__blk1559 * var_vbrinvgat_d_dn6) * assign61640_e80078) + (assign61640_e80075 * (var_vav__blk1559 * var_vbrinvgat_d_dn6))) * assign61640_e80082) + (assign61640_e80079 * (var_vav__blk1559 * var_vbrinvgat_d_dn6))) * assign61640_e80086) + (assign61640_e80083 * (var_vav__blk1559 * var_vbrinvgat_d_dn6))), ((((((((var_vav__blk1559_dn7 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn7)) * assign61640_e80078) + (assign61640_e80075 * ((var_vav__blk1559_dn7 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn7)))) * assign61640_e80082) + (assign61640_e80079 * ((var_vav__blk1559_dn7 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn7)))) * assign61640_e80086) + (assign61640_e80083 * ((var_vav__blk1559_dn7 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn7)))), ((((((((var_vav__blk1559_dn8 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn8)) * assign61640_e80078) + (assign61640_e80075 * ((var_vav__blk1559_dn8 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn8)))) * assign61640_e80082) + (assign61640_e80079 * ((var_vav__blk1559_dn8 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn8)))) * assign61640_e80086) + (assign61640_e80083 * ((var_vav__blk1559_dn8 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn8)))), (((((((var_vav__blk1559 * var_vbrinvgat_d_dn9) * assign61640_e80078) + (assign61640_e80075 * (var_vav__blk1559 * var_vbrinvgat_d_dn9))) * assign61640_e80082) + (assign61640_e80079 * (var_vav__blk1559 * var_vbrinvgat_d_dn9))) * assign61640_e80086) + (assign61640_e80083 * (var_vav__blk1559 * var_vbrinvgat_d_dn9))), (((((((var_vav__blk1559_dn11 * var_vbrinvgat_d) * assign61640_e80078) + (assign61640_e80075 * (var_vav__blk1559_dn11 * var_vbrinvgat_d))) * assign61640_e80082) + (assign61640_e80079 * (var_vav__blk1559_dn11 * var_vbrinvgat_d))) * assign61640_e80086) + (assign61640_e80083 * (var_vav__blk1559_dn11 * var_vbrinvgat_d))), (((((((var_vav__blk1559_dn12 * var_vbrinvgat_d) * assign61640_e80078) + (assign61640_e80075 * (var_vav__blk1559_dn12 * var_vbrinvgat_d))) * assign61640_e80082) + (assign61640_e80079 * (var_vav__blk1559_dn12 * var_vbrinvgat_d))) * assign61640_e80086) + (assign61640_e80083 * (var_vav__blk1559_dn12 * var_vbrinvgat_d))),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61640_e80089;
        var_tmp__blk1560_dn6 = assign61640_e80089_d_n6;
        var_tmp__blk1560_dn7 = assign61640_e80089_d_n7;
        var_tmp__blk1560_dn8 = assign61640_e80089_d_n8;
        var_tmp__blk1560_dn9 = assign61640_e80089_d_n9;
        var_tmp__blk1560_dn11 = assign61640_e80089_d_n11;
        var_tmp__blk1560_dn12 = assign61640_e80089_d_n12;

        let (assign61650_e80112, assign61650_e80112_d_n6, assign61650_e80112_d_n7, assign61650_e80112_d_n8, assign61650_e80112_d_n9, assign61650_e80112_d_n11, assign61650_e80112_d_n12,) = {
    if ((((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1728 == 0.0)) && (var_guard1729 != 0.0)) && (var_guard1730 == 0.0)) {
        let assign61650_e80107: f64 = (var_vav__blk1559 * var_vbrinvgat_d);
        let assign61650_e80108: f64 = (assign61650_e80107).abs();
        let assign61650_e80110: f64 = (assign61650_e80108).powf(var_pbrgatd_i);
        (assign61650_e80110, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61650_e80108).powf(var_pbrgatd_i - 1.0) * if assign61650_e80107 >= 0.0 { (var_vav__blk1559 * var_vbrinvgat_d_dn6) } else { (-(var_vav__blk1559 * var_vbrinvgat_d_dn6)) })) } } else { (assign61650_e80110 * (var_pbrgatd_i * (if assign61650_e80107 >= 0.0 { (var_vav__blk1559 * var_vbrinvgat_d_dn6) } else { (-(var_vav__blk1559 * var_vbrinvgat_d_dn6)) } / assign61650_e80108))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61650_e80108).powf(var_pbrgatd_i - 1.0) * if assign61650_e80107 >= 0.0 { ((var_vav__blk1559_dn7 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn7)) } else { (-((var_vav__blk1559_dn7 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn7))) })) } } else { (assign61650_e80110 * (var_pbrgatd_i * (if assign61650_e80107 >= 0.0 { ((var_vav__blk1559_dn7 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn7)) } else { (-((var_vav__blk1559_dn7 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn7))) } / assign61650_e80108))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61650_e80108).powf(var_pbrgatd_i - 1.0) * if assign61650_e80107 >= 0.0 { ((var_vav__blk1559_dn8 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn8)) } else { (-((var_vav__blk1559_dn8 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn8))) })) } } else { (assign61650_e80110 * (var_pbrgatd_i * (if assign61650_e80107 >= 0.0 { ((var_vav__blk1559_dn8 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn8)) } else { (-((var_vav__blk1559_dn8 * var_vbrinvgat_d) + (var_vav__blk1559 * var_vbrinvgat_d_dn8))) } / assign61650_e80108))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61650_e80108).powf(var_pbrgatd_i - 1.0) * if assign61650_e80107 >= 0.0 { (var_vav__blk1559 * var_vbrinvgat_d_dn9) } else { (-(var_vav__blk1559 * var_vbrinvgat_d_dn9)) })) } } else { (assign61650_e80110 * (var_pbrgatd_i * (if assign61650_e80107 >= 0.0 { (var_vav__blk1559 * var_vbrinvgat_d_dn9) } else { (-(var_vav__blk1559 * var_vbrinvgat_d_dn9)) } / assign61650_e80108))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61650_e80108).powf(var_pbrgatd_i - 1.0) * if assign61650_e80107 >= 0.0 { (var_vav__blk1559_dn11 * var_vbrinvgat_d) } else { (-(var_vav__blk1559_dn11 * var_vbrinvgat_d)) })) } } else { (assign61650_e80110 * (var_pbrgatd_i * (if assign61650_e80107 >= 0.0 { (var_vav__blk1559_dn11 * var_vbrinvgat_d) } else { (-(var_vav__blk1559_dn11 * var_vbrinvgat_d)) } / assign61650_e80108))) }, if 0.0 == 0.0 && ((var_pbrgatd_i) as f64).is_finite() && ((var_pbrgatd_i) as f64).fract() == 0.0 { if var_pbrgatd_i == 0.0 { 0.0 } else { (var_pbrgatd_i * ((assign61650_e80108).powf(var_pbrgatd_i - 1.0) * if assign61650_e80107 >= 0.0 { (var_vav__blk1559_dn12 * var_vbrinvgat_d) } else { (-(var_vav__blk1559_dn12 * var_vbrinvgat_d)) })) } } else { (assign61650_e80110 * (var_pbrgatd_i * (if assign61650_e80107 >= 0.0 { (var_vav__blk1559_dn12 * var_vbrinvgat_d) } else { (-(var_vav__blk1559_dn12 * var_vbrinvgat_d)) } / assign61650_e80108))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61650_e80112;
        var_tmp__blk1560_dn6 = assign61650_e80112_d_n6;
        var_tmp__blk1560_dn7 = assign61650_e80112_d_n7;
        var_tmp__blk1560_dn8 = assign61650_e80112_d_n8;
        var_tmp__blk1560_dn9 = assign61650_e80112_d_n9;
        var_tmp__blk1560_dn11 = assign61650_e80112_d_n11;
        var_tmp__blk1560_dn12 = assign61650_e80112_d_n12;

        let (assign61660_e80131, assign61660_e80131_d_n6, assign61660_e80131_d_n7, assign61660_e80131_d_n8, assign61660_e80131_d_n9, assign61660_e80131_d_n11, assign61660_e80131_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1728 == 0.0)) && (var_guard1729 != 0.0)) {
        let assign61660_e80128: f64 = (1.0 - var_tmp__blk1560);
        let assign61660_e80129: f64 = (1.0 / assign61660_e80128);
        (assign61660_e80129, (-((-var_tmp__blk1560_dn6) / (assign61660_e80128 * assign61660_e80128))), (-((-var_tmp__blk1560_dn7) / (assign61660_e80128 * assign61660_e80128))), (-((-var_tmp__blk1560_dn8) / (assign61660_e80128 * assign61660_e80128))), (-((-var_tmp__blk1560_dn9) / (assign61660_e80128 * assign61660_e80128))), (-((-var_tmp__blk1560_dn11) / (assign61660_e80128 * assign61660_e80128))), (-((-var_tmp__blk1560_dn12) / (assign61660_e80128 * assign61660_e80128))),)
    } else {
        (var_fbreakdown__blk1586, var_fbreakdown__blk1586_dn6, var_fbreakdown__blk1586_dn7, var_fbreakdown__blk1586_dn8, var_fbreakdown__blk1586_dn9, var_fbreakdown__blk1586_dn11, var_fbreakdown__blk1586_dn12,)
    }
};
        var_fbreakdown__blk1586 = assign61660_e80131;
        var_fbreakdown__blk1586_dn6 = assign61660_e80131_d_n6;
        var_fbreakdown__blk1586_dn7 = assign61660_e80131_d_n7;
        var_fbreakdown__blk1586_dn8 = assign61660_e80131_d_n8;
        var_fbreakdown__blk1586_dn9 = assign61660_e80131_d_n9;
        var_fbreakdown__blk1586_dn11 = assign61660_e80131_d_n11;
        var_fbreakdown__blk1586_dn12 = assign61660_e80131_d_n12;

        let (assign61670_e80155, assign61670_e80155_d_n6, assign61670_e80155_d_n7, assign61670_e80155_d_n8, assign61670_e80155_d_n9, assign61670_e80155_d_n11, assign61670_e80155_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1728 == 0.0)) && (var_guard1729 == 0.0)) {
        let assign61670_e80149: f64 = (var_alphaav * var_vbrgat_var_d);
        let assign61670_e80150: f64 = (var_vav__blk1559 + assign61670_e80149);
        let assign61670_e80152: f64 = (assign61670_e80150 * var_slopegat_d);
        let assign61670_e80153: f64 = (var_fstopgat_d + assign61670_e80152);
        (assign61670_e80153, (((var_alphaav * var_vbrgat_var_d_dn6) * var_slopegat_d) + (assign61670_e80150 * var_slopegat_d_dn6)), (((var_vav__blk1559_dn7 + (var_alphaav * var_vbrgat_var_d_dn7)) * var_slopegat_d) + (assign61670_e80150 * var_slopegat_d_dn7)), (((var_vav__blk1559_dn8 + (var_alphaav * var_vbrgat_var_d_dn8)) * var_slopegat_d) + (assign61670_e80150 * var_slopegat_d_dn8)), (((var_alphaav * var_vbrgat_var_d_dn9) * var_slopegat_d) + (assign61670_e80150 * var_slopegat_d_dn9)), (var_vav__blk1559_dn11 * var_slopegat_d), (var_vav__blk1559_dn12 * var_slopegat_d),)
    } else {
        (var_fbreakdown__blk1586, var_fbreakdown__blk1586_dn6, var_fbreakdown__blk1586_dn7, var_fbreakdown__blk1586_dn8, var_fbreakdown__blk1586_dn9, var_fbreakdown__blk1586_dn11, var_fbreakdown__blk1586_dn12,)
    }
};
        var_fbreakdown__blk1586 = assign61670_e80155;
        var_fbreakdown__blk1586_dn6 = assign61670_e80155_d_n6;
        var_fbreakdown__blk1586_dn7 = assign61670_e80155_d_n7;
        var_fbreakdown__blk1586_dn8 = assign61670_e80155_d_n8;
        var_fbreakdown__blk1586_dn9 = assign61670_e80155_d_n9;
        var_fbreakdown__blk1586_dn11 = assign61670_e80155_d_n11;
        var_fbreakdown__blk1586_dn12 = assign61670_e80155_d_n12;

        let (assign61680_e80175, assign61680_e80175_d_n6, assign61680_e80175_d_n7, assign61680_e80175_d_n8, assign61680_e80175_d_n9, assign61680_e80175_d_n11, assign61680_e80175_d_n12,) = {
    if (((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) {
        let assign61680_e80166: f64 = (var_id__blk1561 + var_isrh__blk1562);
        let assign61680_e80168: f64 = (assign61680_e80166 + var_itat__blk1569);
        let assign61680_e80170: f64 = (assign61680_e80168 + var_ibbt__blk1584);
        let assign61680_e80171: f64 = (p.p29 * assign61680_e80170);
        let assign61680_e80173: f64 = (assign61680_e80171 * var_fbreakdown__blk1586);
        (assign61680_e80173, (((p.p29 * ((var_isrh__blk1562_dn6 + var_itat__blk1569_dn6) + var_ibbt__blk1584_dn6)) * var_fbreakdown__blk1586) + (assign61680_e80171 * var_fbreakdown__blk1586_dn6)), (((p.p29 * (((var_id__blk1561_dn7 + var_isrh__blk1562_dn7) + var_itat__blk1569_dn7) + var_ibbt__blk1584_dn7)) * var_fbreakdown__blk1586) + (assign61680_e80171 * var_fbreakdown__blk1586_dn7)), (((p.p29 * (((var_id__blk1561_dn8 + var_isrh__blk1562_dn8) + var_itat__blk1569_dn8) + var_ibbt__blk1584_dn8)) * var_fbreakdown__blk1586) + (assign61680_e80171 * var_fbreakdown__blk1586_dn8)), (((p.p29 * ((var_isrh__blk1562_dn9 + var_itat__blk1569_dn9) + var_ibbt__blk1584_dn9)) * var_fbreakdown__blk1586) + (assign61680_e80171 * var_fbreakdown__blk1586_dn9)), (((p.p29 * (((var_id__blk1561_dn11 + var_isrh__blk1562_dn11) + var_itat__blk1569_dn11) + var_ibbt__blk1584_dn11)) * var_fbreakdown__blk1586) + (assign61680_e80171 * var_fbreakdown__blk1586_dn11)), (((p.p29 * (((var_id__blk1561_dn12 + var_isrh__blk1562_dn12) + var_itat__blk1569_dn12) + var_ibbt__blk1584_dn12)) * var_fbreakdown__blk1586) + (assign61680_e80171 * var_fbreakdown__blk1586_dn12)),)
    } else {
        (var_ijungat_d, var_ijungat_d_dn6, var_ijungat_d_dn7, var_ijungat_d_dn8, var_ijungat_d_dn9, var_ijungat_d_dn11, var_ijungat_d_dn12,)
    }
};
        var_ijungat_d = assign61680_e80175;
        var_ijungat_d_dn6 = assign61680_e80175_d_n6;
        var_ijungat_d_dn7 = assign61680_e80175_d_n7;
        var_ijungat_d_dn8 = assign61680_e80175_d_n8;
        var_ijungat_d_dn9 = assign61680_e80175_d_n9;
        var_ijungat_d_dn11 = assign61680_e80175_d_n11;
        var_ijungat_d_dn12 = assign61680_e80175_d_n12;

        let assign61690_e80178: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard1731 = assign61690_e80178;

        let (assign61700_e80238, assign61700_e80238_d_n7, assign61700_e80238_d_n8, assign61700_e80238_d_n11, assign61700_e80238_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let (assign61700_e80236, assign61700_e80236_d_n8, assign61700_e80236_d_n12,) = {
            if (var_vjun_d < var_vtrgatd_i) {
                let assign61700_e80193: f64 = (var_vjun_d - var_vtrgatd_i);
                let assign61700_e80195: f64 = (assign61700_e80193 / var_anugatd_i);
                let assign61700_e80197: f64 = (-37.0);
                let (assign61700_e80213, assign61700_e80213_d_n8, assign61700_e80213_d_n12,) = {
                    if (assign61700_e80195 < assign61700_e80197) {
                        (var_vtrgatd_i, 0.0, 0.0,)
                    } else {
                        let assign61700_e80204: f64 = (var_vjun_d - var_vtrgatd_i);
                        let assign61700_e80206: f64 = (assign61700_e80204 / var_anugatd_i);
                        let assign61700_e80207: f64 = (assign61700_e80206).exp();
                        let assign61700_e80208: f64 = (1.0 + assign61700_e80207);
                        let assign61700_e80209: f64 = (assign61700_e80208).ln();
                        let assign61700_e80211: f64 = (assign61700_e80209 * var_anugatd_i);
                        let assign61700_e80212: f64 = (var_vtrgatd_i + assign61700_e80211);
                        (assign61700_e80212, (((assign61700_e80207 * (var_vjun_d_dn8 / var_anugatd_i)) / assign61700_e80208) * var_anugatd_i), (((assign61700_e80207 * (var_vjun_d_dn12 / var_anugatd_i)) / assign61700_e80208) * var_anugatd_i),)
                    }
                };
                (assign61700_e80213, assign61700_e80213_d_n8, assign61700_e80213_d_n12,)
            } else {
                let assign61700_e80216: f64 = (var_vjun_d - var_vtrgatd_i);
                let assign61700_e80218: f64 = (assign61700_e80216 / var_anugatd_i);
                let (assign61700_e80235, assign61700_e80235_d_n8, assign61700_e80235_d_n12,) = {
                    if (assign61700_e80218 > 37.0) {
                        (var_vjun_d, var_vjun_d_dn8, var_vjun_d_dn12,)
                    } else {
                        let assign61700_e80226: f64 = (var_vtrgatd_i - var_vjun_d);
                        let assign61700_e80228: f64 = (assign61700_e80226 / var_anugatd_i);
                        let assign61700_e80229: f64 = (assign61700_e80228).exp();
                        let assign61700_e80230: f64 = (1.0 + assign61700_e80229);
                        let assign61700_e80231: f64 = (assign61700_e80230).ln();
                        let assign61700_e80233: f64 = (assign61700_e80231 * var_anugatd_i);
                        let assign61700_e80234: f64 = (var_vjun_d + assign61700_e80233);
                        (assign61700_e80234, (var_vjun_d_dn8 + (((assign61700_e80229 * ((-var_vjun_d_dn8) / var_anugatd_i)) / assign61700_e80230) * var_anugatd_i)), (var_vjun_d_dn12 + (((assign61700_e80229 * ((-var_vjun_d_dn12) / var_anugatd_i)) / assign61700_e80230) * var_anugatd_i)),)
                    }
                };
                (assign61700_e80235, assign61700_e80235_d_n8, assign61700_e80235_d_n12,)
            }
        };
        (assign61700_e80236, 0.0, assign61700_e80236_d_n8, 0.0, assign61700_e80236_d_n12,)
    } else {
        (var_nu__blk1587, var_nu__blk1587_dn7, var_nu__blk1587_dn8, var_nu__blk1587_dn11, var_nu__blk1587_dn12,)
    }
};
        var_nu__blk1587 = assign61700_e80238;
        var_nu__blk1587_dn7 = assign61700_e80238_d_n7;
        var_nu__blk1587_dn8 = assign61700_e80238_d_n8;
        var_nu__blk1587_dn11 = assign61700_e80238_d_n11;
        var_nu__blk1587_dn12 = assign61700_e80238_d_n12;

        let (assign61710_e80254,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61710_e80250: f64 = (4.0 * var_vch_d);
        let assign61710_e80252: f64 = (assign61710_e80250 * var_vch_d);
        (assign61710_e80252,)
    } else {
        (var_h1__blk1545,)
    }
};
        var_h1__blk1545 = assign61710_e80254;

        let (assign61720_e80268,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61720_e80266: f64 = (var_vch_d / var_vfmin_d);
        (assign61720_e80266,)
    } else {
        (var_h2__blk1546,)
    }
};
        var_h2__blk1546 = assign61720_e80268;

        let (assign61730_e80284, assign61730_e80284_d_n7, assign61730_e80284_d_n8, assign61730_e80284_d_n11, assign61730_e80284_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61730_e80281: f64 = (var_vch_d * var_h2__blk1546);
        let assign61730_e80282: f64 = (var_nu__blk1587 + assign61730_e80281);
        (assign61730_e80282, var_nu__blk1587_dn7, var_nu__blk1587_dn8, var_nu__blk1587_dn11, var_nu__blk1587_dn12,)
    } else {
        (var_h2d__blk1547, var_h2d__blk1547_dn7, var_h2d__blk1547_dn8, var_h2d__blk1547_dn11, var_h2d__blk1547_dn12,)
    }
};
        var_h2d__blk1547 = assign61730_e80284;
        var_h2d__blk1547_dn7 = assign61730_e80284_d_n7;
        var_h2d__blk1547_dn8 = assign61730_e80284_d_n8;
        var_h2d__blk1547_dn11 = assign61730_e80284_d_n11;
        var_h2d__blk1547_dn12 = assign61730_e80284_d_n12;

        let (assign61740_e80298, assign61740_e80298_d_n7, assign61740_e80298_d_n8, assign61740_e80298_d_n11, assign61740_e80298_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61740_e80296: f64 = (var_vfmin_d + var_h2d__blk1547);
        (assign61740_e80296, var_h2d__blk1547_dn7, var_h2d__blk1547_dn8, var_h2d__blk1547_dn11, var_h2d__blk1547_dn12,)
    } else {
        (var_h3__blk1548, var_h3__blk1548_dn7, var_h3__blk1548_dn8, var_h3__blk1548_dn11, var_h3__blk1548_dn12,)
    }
};
        var_h3__blk1548 = assign61740_e80298;
        var_h3__blk1548_dn7 = assign61740_e80298_d_n7;
        var_h3__blk1548_dn8 = assign61740_e80298_d_n8;
        var_h3__blk1548_dn11 = assign61740_e80298_d_n11;
        var_h3__blk1548_dn12 = assign61740_e80298_d_n12;

        let (assign61750_e80312, assign61750_e80312_d_n7, assign61750_e80312_d_n8, assign61750_e80312_d_n11, assign61750_e80312_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61750_e80310: f64 = (var_vfmin_d - var_h2d__blk1547);
        (assign61750_e80310, (-var_h2d__blk1547_dn7), (-var_h2d__blk1547_dn8), (-var_h2d__blk1547_dn11), (-var_h2d__blk1547_dn12),)
    } else {
        (var_h4__blk1549, var_h4__blk1549_dn7, var_h4__blk1549_dn8, var_h4__blk1549_dn11, var_h4__blk1549_dn12,)
    }
};
        var_h4__blk1549 = assign61750_e80312;
        var_h4__blk1549_dn7 = assign61750_e80312_d_n7;
        var_h4__blk1549_dn8 = assign61750_e80312_d_n8;
        var_h4__blk1549_dn11 = assign61750_e80312_d_n11;
        var_h4__blk1549_dn12 = assign61750_e80312_d_n12;

        let (assign61760_e80329, assign61760_e80329_d_n7, assign61760_e80329_d_n8, assign61760_e80329_d_n11, assign61760_e80329_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61760_e80324: f64 = (var_h4__blk1549 * var_h4__blk1549);
        let assign61760_e80326: f64 = (assign61760_e80324 + var_h1__blk1545);
        let assign61760_e80327: f64 = (assign61760_e80326).sqrt();
        (assign61760_e80327, (((var_h4__blk1549_dn7 * var_h4__blk1549) + (var_h4__blk1549 * var_h4__blk1549_dn7)) / (2.0 * assign61760_e80327)), (((var_h4__blk1549_dn8 * var_h4__blk1549) + (var_h4__blk1549 * var_h4__blk1549_dn8)) / (2.0 * assign61760_e80327)), (((var_h4__blk1549_dn11 * var_h4__blk1549) + (var_h4__blk1549 * var_h4__blk1549_dn11)) / (2.0 * assign61760_e80327)), (((var_h4__blk1549_dn12 * var_h4__blk1549) + (var_h4__blk1549 * var_h4__blk1549_dn12)) / (2.0 * assign61760_e80327)),)
    } else {
        (var_h5__blk1550, var_h5__blk1550_dn7, var_h5__blk1550_dn8, var_h5__blk1550_dn11, var_h5__blk1550_dn12,)
    }
};
        var_h5__blk1550 = assign61760_e80329;
        var_h5__blk1550_dn7 = assign61760_e80329_d_n7;
        var_h5__blk1550_dn8 = assign61760_e80329_d_n8;
        var_h5__blk1550_dn11 = assign61760_e80329_d_n11;
        var_h5__blk1550_dn12 = assign61760_e80329_d_n12;

        let (assign61770_e80349, assign61770_e80349_d_n7, assign61770_e80349_d_n8, assign61770_e80349_d_n11, assign61770_e80349_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61770_e80342: f64 = (var_nu__blk1587 * var_vfmin_d);
        let assign61770_e80345: f64 = (var_h3__blk1548 + var_h5__blk1550);
        let assign61770_e80346: f64 = (assign61770_e80342 / assign61770_e80345);
        let assign61770_e80347: f64 = (2.0 * assign61770_e80346);
        (assign61770_e80347, (2.0 * ((((var_nu__blk1587_dn7 * var_vfmin_d) * assign61770_e80345) - (assign61770_e80342 * (var_h3__blk1548_dn7 + var_h5__blk1550_dn7))) / (assign61770_e80345 * assign61770_e80345))), (2.0 * ((((var_nu__blk1587_dn8 * var_vfmin_d) * assign61770_e80345) - (assign61770_e80342 * (var_h3__blk1548_dn8 + var_h5__blk1550_dn8))) / (assign61770_e80345 * assign61770_e80345))), (2.0 * ((((var_nu__blk1587_dn11 * var_vfmin_d) * assign61770_e80345) - (assign61770_e80342 * (var_h3__blk1548_dn11 + var_h5__blk1550_dn11))) / (assign61770_e80345 * assign61770_e80345))), (2.0 * ((((var_nu__blk1587_dn12 * var_vfmin_d) * assign61770_e80345) - (assign61770_e80342 * (var_h3__blk1548_dn12 + var_h5__blk1550_dn12))) / (assign61770_e80345 * assign61770_e80345))),)
    } else {
        (var_vjtmp, var_vjtmp_dn7, var_vjtmp_dn8, var_vjtmp_dn11, var_vjtmp_dn12,)
    }
};
        var_vjtmp = assign61770_e80349;
        var_vjtmp_dn7 = assign61770_e80349_d_n7;
        var_vjtmp_dn8 = assign61770_e80349_d_n8;
        var_vjtmp_dn11 = assign61770_e80349_d_n11;
        var_vjtmp_dn12 = assign61770_e80349_d_n12;

        let assign61780_e80352: f64 = if var_one_minus_pgat_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1732 = assign61780_e80352;

        let (assign61790_e80371, assign61790_e80371_d_n6, assign61790_e80371_d_n7, assign61790_e80371_d_n8, assign61790_e80371_d_n9, assign61790_e80371_d_n11, assign61790_e80371_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) && (var_guard1732 != 0.0)) {
        let assign61790_e80367: f64 = (var_vjtmp * var_vbiinvgat_d);
        let assign61790_e80368: f64 = (1.0 - assign61790_e80367);
        let assign61790_e80369: f64 = (assign61790_e80368).sqrt();
        (assign61790_e80369, 0.0, ((-(var_vjtmp_dn7 * var_vbiinvgat_d)) / (2.0 * assign61790_e80369)), ((-(var_vjtmp_dn8 * var_vbiinvgat_d)) / (2.0 * assign61790_e80369)), 0.0, ((-(var_vjtmp_dn11 * var_vbiinvgat_d)) / (2.0 * assign61790_e80369)), ((-(var_vjtmp_dn12 * var_vbiinvgat_d)) / (2.0 * assign61790_e80369)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61790_e80371;
        var_tmp__blk1560_dn6 = assign61790_e80371_d_n6;
        var_tmp__blk1560_dn7 = assign61790_e80371_d_n7;
        var_tmp__blk1560_dn8 = assign61790_e80371_d_n8;
        var_tmp__blk1560_dn9 = assign61790_e80371_d_n9;
        var_tmp__blk1560_dn11 = assign61790_e80371_d_n11;
        var_tmp__blk1560_dn12 = assign61790_e80371_d_n12;

        let (assign61800_e80392, assign61800_e80392_d_n6, assign61800_e80392_d_n7, assign61800_e80392_d_n8, assign61800_e80392_d_n9, assign61800_e80392_d_n11, assign61800_e80392_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) && (var_guard1732 == 0.0)) {
        let assign61800_e80387: f64 = (var_vjtmp * var_vbiinvgat_d);
        let assign61800_e80388: f64 = (1.0 - assign61800_e80387);
        let assign61800_e80390: f64 = (assign61800_e80388).powf(var_one_minus_pgat_d);
        (assign61800_e80390, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61800_e80388).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn7 * var_vbiinvgat_d)))) } } else { (assign61800_e80390 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn7 * var_vbiinvgat_d)) / assign61800_e80388))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61800_e80388).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn8 * var_vbiinvgat_d)))) } } else { (assign61800_e80390 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn8 * var_vbiinvgat_d)) / assign61800_e80388))) }, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61800_e80388).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn11 * var_vbiinvgat_d)))) } } else { (assign61800_e80390 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn11 * var_vbiinvgat_d)) / assign61800_e80388))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61800_e80388).powf(var_one_minus_pgat_d - 1.0) * (-(var_vjtmp_dn12 * var_vbiinvgat_d)))) } } else { (assign61800_e80390 * (var_one_minus_pgat_d * ((-(var_vjtmp_dn12 * var_vbiinvgat_d)) / assign61800_e80388))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61800_e80392;
        var_tmp__blk1560_dn6 = assign61800_e80392_d_n6;
        var_tmp__blk1560_dn7 = assign61800_e80392_d_n7;
        var_tmp__blk1560_dn8 = assign61800_e80392_d_n8;
        var_tmp__blk1560_dn9 = assign61800_e80392_d_n9;
        var_tmp__blk1560_dn11 = assign61800_e80392_d_n11;
        var_tmp__blk1560_dn12 = assign61800_e80392_d_n12;

        let (assign61810_e80416, assign61810_e80416_d_n6, assign61810_e80416_d_n7, assign61810_e80416_d_n8, assign61810_e80416_d_n9, assign61810_e80416_d_n11, assign61810_e80416_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61810_e80406: f64 = (1.0 - var_tmp__blk1560);
        let assign61810_e80407: f64 = (var_qprefgat_d * assign61810_e80406);
        let assign61810_e80411: f64 = (var_nu__blk1587 - var_vjtmp);
        let assign61810_e80412: f64 = (var_qpref2gat_d * assign61810_e80411);
        let assign61810_e80413: f64 = (assign61810_e80407 + assign61810_e80412);
        let assign61810_e80414: f64 = (p.p30 * assign61810_e80413);
        (assign61810_e80414, (p.p30 * (var_qprefgat_d * (-var_tmp__blk1560_dn6))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1560_dn7)) + (var_qpref2gat_d * (var_nu__blk1587_dn7 - var_vjtmp_dn7)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1560_dn8)) + (var_qpref2gat_d * (var_nu__blk1587_dn8 - var_vjtmp_dn8)))), (p.p30 * (var_qprefgat_d * (-var_tmp__blk1560_dn9))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1560_dn11)) + (var_qpref2gat_d * (var_nu__blk1587_dn11 - var_vjtmp_dn11)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1560_dn12)) + (var_qpref2gat_d * (var_nu__blk1587_dn12 - var_vjtmp_dn12)))),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn9, var_qjungat_d_dn11, var_qjungat_d_dn12,)
    }
};
        var_qjungat_d = assign61810_e80416;
        var_qjungat_d_dn6 = assign61810_e80416_d_n6;
        var_qjungat_d_dn7 = assign61810_e80416_d_n7;
        var_qjungat_d_dn8 = assign61810_e80416_d_n8;
        var_qjungat_d_dn9 = assign61810_e80416_d_n9;
        var_qjungat_d_dn11 = assign61810_e80416_d_n11;
        var_qjungat_d_dn12 = assign61810_e80416_d_n12;

        let (assign61820_e80432, assign61820_e80432_d_n7, assign61820_e80432_d_n8, assign61820_e80432_d_n11, assign61820_e80432_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61820_e80428: f64 = (var_vjun_d + var_vtrgatd_i);
        let assign61820_e80430: f64 = (assign61820_e80428 - var_nu__blk1587);
        (assign61820_e80430, (-var_nu__blk1587_dn7), (var_vjun_d_dn8 - var_nu__blk1587_dn8), (-var_nu__blk1587_dn11), (var_vjun_d_dn12 - var_nu__blk1587_dn12),)
    } else {
        (var_nu__blk1587, var_nu__blk1587_dn7, var_nu__blk1587_dn8, var_nu__blk1587_dn11, var_nu__blk1587_dn12,)
    }
};
        var_nu__blk1587 = assign61820_e80432;
        var_nu__blk1587_dn7 = assign61820_e80432_d_n7;
        var_nu__blk1587_dn8 = assign61820_e80432_d_n8;
        var_nu__blk1587_dn11 = assign61820_e80432_d_n11;
        var_nu__blk1587_dn12 = assign61820_e80432_d_n12;

        let (assign61830_e80448,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61830_e80444: f64 = (4.0 * var_vch_d);
        let assign61830_e80446: f64 = (assign61830_e80444 * var_vch_d);
        (assign61830_e80446,)
    } else {
        (var_h1__blk1545,)
    }
};
        var_h1__blk1545 = assign61830_e80448;

        let (assign61840_e80462,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61840_e80460: f64 = (var_vch_d / var_vfmin_d);
        (assign61840_e80460,)
    } else {
        (var_h2__blk1546,)
    }
};
        var_h2__blk1546 = assign61840_e80462;

        let (assign61850_e80478, assign61850_e80478_d_n7, assign61850_e80478_d_n8, assign61850_e80478_d_n11, assign61850_e80478_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61850_e80475: f64 = (var_vch_d * var_h2__blk1546);
        let assign61850_e80476: f64 = (var_nu__blk1587 + assign61850_e80475);
        (assign61850_e80476, var_nu__blk1587_dn7, var_nu__blk1587_dn8, var_nu__blk1587_dn11, var_nu__blk1587_dn12,)
    } else {
        (var_h2d__blk1547, var_h2d__blk1547_dn7, var_h2d__blk1547_dn8, var_h2d__blk1547_dn11, var_h2d__blk1547_dn12,)
    }
};
        var_h2d__blk1547 = assign61850_e80478;
        var_h2d__blk1547_dn7 = assign61850_e80478_d_n7;
        var_h2d__blk1547_dn8 = assign61850_e80478_d_n8;
        var_h2d__blk1547_dn11 = assign61850_e80478_d_n11;
        var_h2d__blk1547_dn12 = assign61850_e80478_d_n12;

        let (assign61860_e80492, assign61860_e80492_d_n7, assign61860_e80492_d_n8, assign61860_e80492_d_n11, assign61860_e80492_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61860_e80490: f64 = (var_vfmin_d + var_h2d__blk1547);
        (assign61860_e80490, var_h2d__blk1547_dn7, var_h2d__blk1547_dn8, var_h2d__blk1547_dn11, var_h2d__blk1547_dn12,)
    } else {
        (var_h3__blk1548, var_h3__blk1548_dn7, var_h3__blk1548_dn8, var_h3__blk1548_dn11, var_h3__blk1548_dn12,)
    }
};
        var_h3__blk1548 = assign61860_e80492;
        var_h3__blk1548_dn7 = assign61860_e80492_d_n7;
        var_h3__blk1548_dn8 = assign61860_e80492_d_n8;
        var_h3__blk1548_dn11 = assign61860_e80492_d_n11;
        var_h3__blk1548_dn12 = assign61860_e80492_d_n12;

        *var_fbreakdown__blk1586_slot = var_fbreakdown__blk1586;
        *var_fbreakdown__blk1586_dn11_slot = var_fbreakdown__blk1586_dn11;
        *var_fbreakdown__blk1586_dn12_slot = var_fbreakdown__blk1586_dn12;
        *var_fbreakdown__blk1586_dn6_slot = var_fbreakdown__blk1586_dn6;
        *var_fbreakdown__blk1586_dn7_slot = var_fbreakdown__blk1586_dn7;
        *var_fbreakdown__blk1586_dn8_slot = var_fbreakdown__blk1586_dn8;
        *var_fbreakdown__blk1586_dn9_slot = var_fbreakdown__blk1586_dn9;
        *var_guard1727_slot = var_guard1727;
        *var_guard1728_slot = var_guard1728;
        *var_guard1729_slot = var_guard1729;
        *var_guard1730_slot = var_guard1730;
        *var_guard1731_slot = var_guard1731;
        *var_guard1732_slot = var_guard1732;
        *var_h1__blk1545_slot = var_h1__blk1545;
        *var_h2__blk1546_slot = var_h2__blk1546;
        *var_h2d__blk1547_slot = var_h2d__blk1547;
        *var_h2d__blk1547_dn11_slot = var_h2d__blk1547_dn11;
        *var_h2d__blk1547_dn12_slot = var_h2d__blk1547_dn12;
        *var_h2d__blk1547_dn7_slot = var_h2d__blk1547_dn7;
        *var_h2d__blk1547_dn8_slot = var_h2d__blk1547_dn8;
        *var_h3__blk1548_slot = var_h3__blk1548;
        *var_h3__blk1548_dn11_slot = var_h3__blk1548_dn11;
        *var_h3__blk1548_dn12_slot = var_h3__blk1548_dn12;
        *var_h3__blk1548_dn7_slot = var_h3__blk1548_dn7;
        *var_h3__blk1548_dn8_slot = var_h3__blk1548_dn8;
        *var_h4__blk1549_slot = var_h4__blk1549;
        *var_h4__blk1549_dn11_slot = var_h4__blk1549_dn11;
        *var_h4__blk1549_dn12_slot = var_h4__blk1549_dn12;
        *var_h4__blk1549_dn7_slot = var_h4__blk1549_dn7;
        *var_h4__blk1549_dn8_slot = var_h4__blk1549_dn8;
        *var_h5__blk1550_slot = var_h5__blk1550;
        *var_h5__blk1550_dn11_slot = var_h5__blk1550_dn11;
        *var_h5__blk1550_dn12_slot = var_h5__blk1550_dn12;
        *var_h5__blk1550_dn7_slot = var_h5__blk1550_dn7;
        *var_h5__blk1550_dn8_slot = var_h5__blk1550_dn8;
        *var_ibbt__blk1584_slot = var_ibbt__blk1584;
        *var_ibbt__blk1584_dn11_slot = var_ibbt__blk1584_dn11;
        *var_ibbt__blk1584_dn12_slot = var_ibbt__blk1584_dn12;
        *var_ibbt__blk1584_dn6_slot = var_ibbt__blk1584_dn6;
        *var_ibbt__blk1584_dn7_slot = var_ibbt__blk1584_dn7;
        *var_ibbt__blk1584_dn8_slot = var_ibbt__blk1584_dn8;
        *var_ibbt__blk1584_dn9_slot = var_ibbt__blk1584_dn9;
        *var_ijungat_d_slot = var_ijungat_d;
        *var_ijungat_d_dn11_slot = var_ijungat_d_dn11;
        *var_ijungat_d_dn12_slot = var_ijungat_d_dn12;
        *var_ijungat_d_dn6_slot = var_ijungat_d_dn6;
        *var_ijungat_d_dn7_slot = var_ijungat_d_dn7;
        *var_ijungat_d_dn8_slot = var_ijungat_d_dn8;
        *var_ijungat_d_dn9_slot = var_ijungat_d_dn9;
        *var_nu__blk1587_slot = var_nu__blk1587;
        *var_nu__blk1587_dn11_slot = var_nu__blk1587_dn11;
        *var_nu__blk1587_dn12_slot = var_nu__blk1587_dn12;
        *var_nu__blk1587_dn7_slot = var_nu__blk1587_dn7;
        *var_nu__blk1587_dn8_slot = var_nu__blk1587_dn8;
        *var_qjungat_d_slot = var_qjungat_d;
        *var_qjungat_d_dn11_slot = var_qjungat_d_dn11;
        *var_qjungat_d_dn12_slot = var_qjungat_d_dn12;
        *var_qjungat_d_dn6_slot = var_qjungat_d_dn6;
        *var_qjungat_d_dn7_slot = var_qjungat_d_dn7;
        *var_qjungat_d_dn8_slot = var_qjungat_d_dn8;
        *var_qjungat_d_dn9_slot = var_qjungat_d_dn9;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_vjtmp_slot = var_vjtmp;
        *var_vjtmp_dn11_slot = var_vjtmp_dn11;
        *var_vjtmp_dn12_slot = var_vjtmp_dn12;
        *var_vjtmp_dn7_slot = var_vjtmp_dn7;
        *var_vjtmp_dn8_slot = var_vjtmp_dn8;
    }

    pub(super) fn stamp_transient_block_147(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_alpha_dc: f64,
        var_alpha_dc_dn4: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_alpha_dc_dn9: f64,
        var_bet_i: f64,
        var_cox_qm: f64,
        var_cox_qm_dn4: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_cox_qm_dn9: f64,
        var_dps_dc: f64,
        var_dps_dc_dn4: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_dps_dc_dn9: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_dn4: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_eta_p_ac_dn9: f64,
        var_gdrain: f64,
        var_gsource: f64,
        var_guard1589: f64,
        var_guard1590: f64,
        var_guard1714: f64,
        var_guard1731: f64,
        var_h1__blk1545: f64,
        var_h2d__blk1547: f64,
        var_h2d__blk1547_dn11: f64,
        var_h2d__blk1547_dn12: f64,
        var_h2d__blk1547_dn7: f64,
        var_h2d__blk1547_dn8: f64,
        var_h3__blk1548: f64,
        var_h3__blk1548_dn11: f64,
        var_h3__blk1548_dn12: f64,
        var_h3__blk1548_dn7: f64,
        var_h3__blk1548_dn8: f64,
        var_i_ds: f64,
        var_i_ds_dn4: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_i_ds_dn9: f64,
        var_i_dsedge: f64,
        var_i_dsedge_dn4: f64,
        var_i_dsedge_dn6: f64,
        var_i_dsedge_dn7: f64,
        var_i_dsedge_dn8: f64,
        var_i_dsedge_dn9: f64,
        var_iimpact: f64,
        var_iimpact_dn4: f64,
        var_iimpact_dn6: f64,
        var_iimpact_dn7: f64,
        var_iimpact_dn8: f64,
        var_iimpact_dn9: f64,
        var_ijunbot_d: f64,
        var_ijunbot_d_dn11: f64,
        var_ijunbot_d_dn12: f64,
        var_ijunbot_d_dn6: f64,
        var_ijunbot_d_dn7: f64,
        var_ijunbot_d_dn8: f64,
        var_ijunbot_d_dn9: f64,
        var_ijungat_d: f64,
        var_ijungat_d_dn11: f64,
        var_ijungat_d_dn12: f64,
        var_ijungat_d_dn6: f64,
        var_ijungat_d_dn7: f64,
        var_ijungat_d_dn8: f64,
        var_ijungat_d_dn9: f64,
        var_ijunsti_d: f64,
        var_ijunsti_d_dn11: f64,
        var_ijunsti_d_dn12: f64,
        var_ijunsti_d_dn6: f64,
        var_ijunsti_d_dn7: f64,
        var_ijunsti_d_dn8: f64,
        var_ijunsti_d_dn9: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_nu__blk1587: f64,
        var_nu__blk1587_dn11: f64,
        var_nu__blk1587_dn12: f64,
        var_nu__blk1587_dn7: f64,
        var_nu__blk1587_dn8: f64,
        var_one_minus_pgat2nd_d: f64,
        var_one_minus_pgat_d: f64,
        var_qb: f64,
        var_qb_dn4: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qg: f64,
        var_qg_dn4: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
        var_qgd_ov: f64,
        var_qgd_ov_dn6: f64,
        var_qgd_ov_dn7: f64,
        var_qgd_ov_dn8: f64,
        var_qgs_ov: f64,
        var_qgs_ov_dn6: f64,
        var_qgs_ov_dn7: f64,
        var_qgs_ov_dn8: f64,
        var_qim1_dc: f64,
        var_qim1_dc_dn4: f64,
        var_qim1_dc_dn6: f64,
        var_qim1_dc_dn7: f64,
        var_qim1_dc_dn8: f64,
        var_qim1_dc_dn9: f64,
        var_qim_dc: f64,
        var_qim_dc_dn4: f64,
        var_qim_dc_dn6: f64,
        var_qim_dc_dn7: f64,
        var_qim_dc_dn8: f64,
        var_qim_dc_dn9: f64,
        var_qjunbot_d: f64,
        var_qjunbot_d_dn11: f64,
        var_qjunbot_d_dn12: f64,
        var_qjunbot_d_dn6: f64,
        var_qjunbot_d_dn7: f64,
        var_qjunbot_d_dn8: f64,
        var_qjunbot_d_dn9: f64,
        var_qjunbot_s: f64,
        var_qjunbot_s_dn11: f64,
        var_qjunbot_s_dn12: f64,
        var_qjunbot_s_dn6: f64,
        var_qjunbot_s_dn7: f64,
        var_qjunbot_s_dn8: f64,
        var_qjunbot_s_dn9: f64,
        var_qjungat_s: f64,
        var_qjungat_s_dn11: f64,
        var_qjungat_s_dn12: f64,
        var_qjungat_s_dn6: f64,
        var_qjungat_s_dn7: f64,
        var_qjungat_s_dn8: f64,
        var_qjungat_s_dn9: f64,
        var_qjunsti_d: f64,
        var_qjunsti_d_dn11: f64,
        var_qjunsti_d_dn12: f64,
        var_qjunsti_d_dn6: f64,
        var_qjunsti_d_dn7: f64,
        var_qjunsti_d_dn8: f64,
        var_qjunsti_d_dn9: f64,
        var_qjunsti_s: f64,
        var_qjunsti_s_dn11: f64,
        var_qjunsti_s_dn12: f64,
        var_qjunsti_s_dn6: f64,
        var_qjunsti_s_dn7: f64,
        var_qjunsti_s_dn8: f64,
        var_qjunsti_s_dn9: f64,
        var_qpref2gat2nd_d: f64,
        var_qpref2gat_d: f64,
        var_qprefgat2nd_d: f64,
        var_qprefgat_d: f64,
        var_rbulk_i: f64,
        var_rde_i: f64,
        var_rg_i: f64,
        var_rjund_i: f64,
        var_rjuns_i: f64,
        var_rse_i: f64,
        var_rth_p: f64,
        var_rwell_i: f64,
        var_sigvds: f64,
        var_v_ds: f64,
        var_v_ds_dn7: f64,
        var_v_ds_dn8: f64,
        var_v_sb: f64,
        var_v_sb_dn7: f64,
        var_v_sb_dn8: f64,
        var_v_sb_dn9: f64,
        var_vbiinvgat2nd_d: f64,
        var_vbiinvgat_d: f64,
        var_vfmin_d: f64,
        var_vj__blk1552: f64,
        var_vj__blk1552_dn11: f64,
        var_vj__blk1552_dn12: f64,
        var_vj__blk1552_dn7: f64,
        var_vj__blk1552_dn8: f64,
        var_vjun_d: f64,
        var_vjun_d_dn12: f64,
        var_vjun_d_dn8: f64,
        var_xg_dc: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_dn4_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_c_igid_dn9_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn4_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_cgeff_dn9_slot: &mut f64,
        var_guard1733_slot: &mut f64,
        var_guard1734_slot: &mut f64,
        var_guard1735_slot: &mut f64,
        var_guard1736_slot: &mut f64,
        var_guard1737_slot: &mut f64,
        var_guard1738_slot: &mut f64,
        var_guard1739_slot: &mut f64,
        var_guard1740_slot: &mut f64,
        var_guard1741_slot: &mut f64,
        var_guard1742_slot: &mut f64,
        var_guard1745_slot: &mut f64,
        var_guard1746_slot: &mut f64,
        var_guard1747_slot: &mut f64,
        var_guard1749_slot: &mut f64,
        var_guard1782_slot: &mut f64,
        var_guard1784_slot: &mut f64,
        var_h0_slot: &mut f64,
        var_h0_dn4_slot: &mut f64,
        var_h0_dn6_slot: &mut f64,
        var_h0_dn7_slot: &mut f64,
        var_h0_dn8_slot: &mut f64,
        var_h0_dn9_slot: &mut f64,
        var_h4__blk1549_slot: &mut f64,
        var_h4__blk1549_dn11_slot: &mut f64,
        var_h4__blk1549_dn12_slot: &mut f64,
        var_h4__blk1549_dn7_slot: &mut f64,
        var_h4__blk1549_dn8_slot: &mut f64,
        var_h5__blk1550_slot: &mut f64,
        var_h5__blk1550_dn11_slot: &mut f64,
        var_h5__blk1550_dn12_slot: &mut f64,
        var_h5__blk1550_dn7_slot: &mut f64,
        var_h5__blk1550_dn8_slot: &mut f64,
        var_ijun_d_slot: &mut f64,
        var_ijun_d_dn11_slot: &mut f64,
        var_ijun_d_dn12_slot: &mut f64,
        var_ijun_d_dn6_slot: &mut f64,
        var_ijun_d_dn7_slot: &mut f64,
        var_ijun_d_dn8_slot: &mut f64,
        var_ijun_d_dn9_slot: &mut f64,
        var_mid_slot: &mut f64,
        var_mid_dn4_slot: &mut f64,
        var_mid_dn6_slot: &mut f64,
        var_mid_dn7_slot: &mut f64,
        var_mid_dn8_slot: &mut f64,
        var_mid_dn9_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_dn4_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_mig_dn9_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid_dn4_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_migid_dn9_slot: &mut f64,
        var_pdiss_1_slot: &mut f64,
        var_pdiss_1_dn0_slot: &mut f64,
        var_pdiss_1_dn2_slot: &mut f64,
        var_pdiss_1_dn4_slot: &mut f64,
        var_pdiss_1_dn6_slot: &mut f64,
        var_pdiss_1_dn7_slot: &mut f64,
        var_pdiss_1_dn8_slot: &mut f64,
        var_pdiss_1_dn9_slot: &mut f64,
        var_pdiss_d_slot: &mut f64,
        var_pdiss_d_dn0_slot: &mut f64,
        var_pdiss_d_dn8_slot: &mut f64,
        var_pdiss_s_slot: &mut f64,
        var_pdiss_s_dn2_slot: &mut f64,
        var_pdiss_s_dn7_slot: &mut f64,
        var_qd_slot: &mut f64,
        var_qd_dn4_slot: &mut f64,
        var_qd_dn6_slot: &mut f64,
        var_qd_dn7_slot: &mut f64,
        var_qd_dn8_slot: &mut f64,
        var_qd_dn9_slot: &mut f64,
        var_qfgd_slot: &mut f64,
        var_qfgd_dn6_slot: &mut f64,
        var_qfgd_dn7_slot: &mut f64,
        var_qfgd_dn8_slot: &mut f64,
        var_qfgs_slot: &mut f64,
        var_qfgs_dn6_slot: &mut f64,
        var_qfgs_dn7_slot: &mut f64,
        var_qfgs_dn8_slot: &mut f64,
        var_qjun_d_slot: &mut f64,
        var_qjun_d_dn11_slot: &mut f64,
        var_qjun_d_dn12_slot: &mut f64,
        var_qjun_d_dn6_slot: &mut f64,
        var_qjun_d_dn7_slot: &mut f64,
        var_qjun_d_dn8_slot: &mut f64,
        var_qjun_d_dn9_slot: &mut f64,
        var_qjun_s_slot: &mut f64,
        var_qjun_s_dn11_slot: &mut f64,
        var_qjun_s_dn12_slot: &mut f64,
        var_qjun_s_dn6_slot: &mut f64,
        var_qjun_s_dn7_slot: &mut f64,
        var_qjun_s_dn8_slot: &mut f64,
        var_qjun_s_dn9_slot: &mut f64,
        var_qjungat2nd_slot: &mut f64,
        var_qjungat2nd_dn11_slot: &mut f64,
        var_qjungat2nd_dn12_slot: &mut f64,
        var_qjungat2nd_dn6_slot: &mut f64,
        var_qjungat2nd_dn7_slot: &mut f64,
        var_qjungat2nd_dn8_slot: &mut f64,
        var_qjungat2nd_dn9_slot: &mut f64,
        var_qjungat_d_slot: &mut f64,
        var_qjungat_d_dn11_slot: &mut f64,
        var_qjungat_d_dn12_slot: &mut f64,
        var_qjungat_d_dn6_slot: &mut f64,
        var_qjungat_d_dn7_slot: &mut f64,
        var_qjungat_d_dn8_slot: &mut f64,
        var_qjungat_d_dn9_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_dn9_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sidexc_dn4_slot: &mut f64,
        var_sidexc_dn6_slot: &mut f64,
        var_sidexc_dn7_slot: &mut f64,
        var_sidexc_dn8_slot: &mut f64,
        var_sidexc_dn9_slot: &mut f64,
        var_sqid_slot: &mut f64,
        var_sqid_dn4_slot: &mut f64,
        var_sqid_dn6_slot: &mut f64,
        var_sqid_dn7_slot: &mut f64,
        var_sqid_dn8_slot: &mut f64,
        var_sqid_dn9_slot: &mut f64,
        var_sqig_slot: &mut f64,
        var_sqig_dn4_slot: &mut f64,
        var_sqig_dn6_slot: &mut f64,
        var_sqig_dn7_slot: &mut f64,
        var_sqig_dn8_slot: &mut f64,
        var_sqig_dn9_slot: &mut f64,
        var_sqt2_slot: &mut f64,
        var_sqt2_dn4_slot: &mut f64,
        var_sqt2_dn6_slot: &mut f64,
        var_sqt2_dn7_slot: &mut f64,
        var_sqt2_dn8_slot: &mut f64,
        var_sqt2_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_temp__blk1748_slot: &mut f64,
        var_temp__blk1748_dn4_slot: &mut f64,
        var_temp__blk1748_dn6_slot: &mut f64,
        var_temp__blk1748_dn7_slot: &mut f64,
        var_temp__blk1748_dn8_slot: &mut f64,
        var_temp__blk1748_dn9_slot: &mut f64,
        var_tmp__blk1560_slot: &mut f64,
        var_tmp__blk1560_dn11_slot: &mut f64,
        var_tmp__blk1560_dn12_slot: &mut f64,
        var_tmp__blk1560_dn6_slot: &mut f64,
        var_tmp__blk1560_dn7_slot: &mut f64,
        var_tmp__blk1560_dn8_slot: &mut f64,
        var_tmp__blk1560_dn9_slot: &mut f64,
        var_vjtmp_slot: &mut f64,
        var_vjtmp_dn11_slot: &mut f64,
        var_vjtmp_dn12_slot: &mut f64,
        var_vjtmp_dn7_slot: &mut f64,
        var_vjtmp_dn8_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_dn4: f64 = *var_c_igid_dn4_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_c_igid_dn9: f64 = *var_c_igid_dn9_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn4: f64 = *var_cgeff_dn4_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_cgeff_dn9: f64 = *var_cgeff_dn9_slot;
        let mut var_guard1733: f64 = *var_guard1733_slot;
        let mut var_guard1734: f64 = *var_guard1734_slot;
        let mut var_guard1735: f64 = *var_guard1735_slot;
        let mut var_guard1736: f64 = *var_guard1736_slot;
        let mut var_guard1737: f64 = *var_guard1737_slot;
        let mut var_guard1738: f64 = *var_guard1738_slot;
        let mut var_guard1739: f64 = *var_guard1739_slot;
        let mut var_guard1740: f64 = *var_guard1740_slot;
        let mut var_guard1741: f64 = *var_guard1741_slot;
        let mut var_guard1742: f64 = *var_guard1742_slot;
        let mut var_guard1745: f64 = *var_guard1745_slot;
        let mut var_guard1746: f64 = *var_guard1746_slot;
        let mut var_guard1747: f64 = *var_guard1747_slot;
        let mut var_guard1749: f64 = *var_guard1749_slot;
        let mut var_guard1782: f64 = *var_guard1782_slot;
        let mut var_guard1784: f64 = *var_guard1784_slot;
        let mut var_h0: f64 = *var_h0_slot;
        let mut var_h0_dn4: f64 = *var_h0_dn4_slot;
        let mut var_h0_dn6: f64 = *var_h0_dn6_slot;
        let mut var_h0_dn7: f64 = *var_h0_dn7_slot;
        let mut var_h0_dn8: f64 = *var_h0_dn8_slot;
        let mut var_h0_dn9: f64 = *var_h0_dn9_slot;
        let mut var_h4__blk1549: f64 = *var_h4__blk1549_slot;
        let mut var_h4__blk1549_dn11: f64 = *var_h4__blk1549_dn11_slot;
        let mut var_h4__blk1549_dn12: f64 = *var_h4__blk1549_dn12_slot;
        let mut var_h4__blk1549_dn7: f64 = *var_h4__blk1549_dn7_slot;
        let mut var_h4__blk1549_dn8: f64 = *var_h4__blk1549_dn8_slot;
        let mut var_h5__blk1550: f64 = *var_h5__blk1550_slot;
        let mut var_h5__blk1550_dn11: f64 = *var_h5__blk1550_dn11_slot;
        let mut var_h5__blk1550_dn12: f64 = *var_h5__blk1550_dn12_slot;
        let mut var_h5__blk1550_dn7: f64 = *var_h5__blk1550_dn7_slot;
        let mut var_h5__blk1550_dn8: f64 = *var_h5__blk1550_dn8_slot;
        let mut var_ijun_d: f64 = *var_ijun_d_slot;
        let mut var_ijun_d_dn11: f64 = *var_ijun_d_dn11_slot;
        let mut var_ijun_d_dn12: f64 = *var_ijun_d_dn12_slot;
        let mut var_ijun_d_dn6: f64 = *var_ijun_d_dn6_slot;
        let mut var_ijun_d_dn7: f64 = *var_ijun_d_dn7_slot;
        let mut var_ijun_d_dn8: f64 = *var_ijun_d_dn8_slot;
        let mut var_ijun_d_dn9: f64 = *var_ijun_d_dn9_slot;
        let mut var_mid: f64 = *var_mid_slot;
        let mut var_mid_dn4: f64 = *var_mid_dn4_slot;
        let mut var_mid_dn6: f64 = *var_mid_dn6_slot;
        let mut var_mid_dn7: f64 = *var_mid_dn7_slot;
        let mut var_mid_dn8: f64 = *var_mid_dn8_slot;
        let mut var_mid_dn9: f64 = *var_mid_dn9_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_dn4: f64 = *var_mig_dn4_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_mig_dn9: f64 = *var_mig_dn9_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid_dn4: f64 = *var_migid_dn4_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_migid_dn9: f64 = *var_migid_dn9_slot;
        let mut var_pdiss_1: f64 = *var_pdiss_1_slot;
        let mut var_pdiss_1_dn0: f64 = *var_pdiss_1_dn0_slot;
        let mut var_pdiss_1_dn2: f64 = *var_pdiss_1_dn2_slot;
        let mut var_pdiss_1_dn4: f64 = *var_pdiss_1_dn4_slot;
        let mut var_pdiss_1_dn6: f64 = *var_pdiss_1_dn6_slot;
        let mut var_pdiss_1_dn7: f64 = *var_pdiss_1_dn7_slot;
        let mut var_pdiss_1_dn8: f64 = *var_pdiss_1_dn8_slot;
        let mut var_pdiss_1_dn9: f64 = *var_pdiss_1_dn9_slot;
        let mut var_pdiss_d: f64 = *var_pdiss_d_slot;
        let mut var_pdiss_d_dn0: f64 = *var_pdiss_d_dn0_slot;
        let mut var_pdiss_d_dn8: f64 = *var_pdiss_d_dn8_slot;
        let mut var_pdiss_s: f64 = *var_pdiss_s_slot;
        let mut var_pdiss_s_dn2: f64 = *var_pdiss_s_dn2_slot;
        let mut var_pdiss_s_dn7: f64 = *var_pdiss_s_dn7_slot;
        let mut var_qd: f64 = *var_qd_slot;
        let mut var_qd_dn4: f64 = *var_qd_dn4_slot;
        let mut var_qd_dn6: f64 = *var_qd_dn6_slot;
        let mut var_qd_dn7: f64 = *var_qd_dn7_slot;
        let mut var_qd_dn8: f64 = *var_qd_dn8_slot;
        let mut var_qd_dn9: f64 = *var_qd_dn9_slot;
        let mut var_qfgd: f64 = *var_qfgd_slot;
        let mut var_qfgd_dn6: f64 = *var_qfgd_dn6_slot;
        let mut var_qfgd_dn7: f64 = *var_qfgd_dn7_slot;
        let mut var_qfgd_dn8: f64 = *var_qfgd_dn8_slot;
        let mut var_qfgs: f64 = *var_qfgs_slot;
        let mut var_qfgs_dn6: f64 = *var_qfgs_dn6_slot;
        let mut var_qfgs_dn7: f64 = *var_qfgs_dn7_slot;
        let mut var_qfgs_dn8: f64 = *var_qfgs_dn8_slot;
        let mut var_qjun_d: f64 = *var_qjun_d_slot;
        let mut var_qjun_d_dn11: f64 = *var_qjun_d_dn11_slot;
        let mut var_qjun_d_dn12: f64 = *var_qjun_d_dn12_slot;
        let mut var_qjun_d_dn6: f64 = *var_qjun_d_dn6_slot;
        let mut var_qjun_d_dn7: f64 = *var_qjun_d_dn7_slot;
        let mut var_qjun_d_dn8: f64 = *var_qjun_d_dn8_slot;
        let mut var_qjun_d_dn9: f64 = *var_qjun_d_dn9_slot;
        let mut var_qjun_s: f64 = *var_qjun_s_slot;
        let mut var_qjun_s_dn11: f64 = *var_qjun_s_dn11_slot;
        let mut var_qjun_s_dn12: f64 = *var_qjun_s_dn12_slot;
        let mut var_qjun_s_dn6: f64 = *var_qjun_s_dn6_slot;
        let mut var_qjun_s_dn7: f64 = *var_qjun_s_dn7_slot;
        let mut var_qjun_s_dn8: f64 = *var_qjun_s_dn8_slot;
        let mut var_qjun_s_dn9: f64 = *var_qjun_s_dn9_slot;
        let mut var_qjungat2nd: f64 = *var_qjungat2nd_slot;
        let mut var_qjungat2nd_dn11: f64 = *var_qjungat2nd_dn11_slot;
        let mut var_qjungat2nd_dn12: f64 = *var_qjungat2nd_dn12_slot;
        let mut var_qjungat2nd_dn6: f64 = *var_qjungat2nd_dn6_slot;
        let mut var_qjungat2nd_dn7: f64 = *var_qjungat2nd_dn7_slot;
        let mut var_qjungat2nd_dn8: f64 = *var_qjungat2nd_dn8_slot;
        let mut var_qjungat2nd_dn9: f64 = *var_qjungat2nd_dn9_slot;
        let mut var_qjungat_d: f64 = *var_qjungat_d_slot;
        let mut var_qjungat_d_dn11: f64 = *var_qjungat_d_dn11_slot;
        let mut var_qjungat_d_dn12: f64 = *var_qjungat_d_dn12_slot;
        let mut var_qjungat_d_dn6: f64 = *var_qjungat_d_dn6_slot;
        let mut var_qjungat_d_dn7: f64 = *var_qjungat_d_dn7_slot;
        let mut var_qjungat_d_dn8: f64 = *var_qjungat_d_dn8_slot;
        let mut var_qjungat_d_dn9: f64 = *var_qjungat_d_dn9_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_dn9: f64 = *var_qs_dn9_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sidexc_dn4: f64 = *var_sidexc_dn4_slot;
        let mut var_sidexc_dn6: f64 = *var_sidexc_dn6_slot;
        let mut var_sidexc_dn7: f64 = *var_sidexc_dn7_slot;
        let mut var_sidexc_dn8: f64 = *var_sidexc_dn8_slot;
        let mut var_sidexc_dn9: f64 = *var_sidexc_dn9_slot;
        let mut var_sqid: f64 = *var_sqid_slot;
        let mut var_sqid_dn4: f64 = *var_sqid_dn4_slot;
        let mut var_sqid_dn6: f64 = *var_sqid_dn6_slot;
        let mut var_sqid_dn7: f64 = *var_sqid_dn7_slot;
        let mut var_sqid_dn8: f64 = *var_sqid_dn8_slot;
        let mut var_sqid_dn9: f64 = *var_sqid_dn9_slot;
        let mut var_sqig: f64 = *var_sqig_slot;
        let mut var_sqig_dn4: f64 = *var_sqig_dn4_slot;
        let mut var_sqig_dn6: f64 = *var_sqig_dn6_slot;
        let mut var_sqig_dn7: f64 = *var_sqig_dn7_slot;
        let mut var_sqig_dn8: f64 = *var_sqig_dn8_slot;
        let mut var_sqig_dn9: f64 = *var_sqig_dn9_slot;
        let mut var_sqt2: f64 = *var_sqt2_slot;
        let mut var_sqt2_dn4: f64 = *var_sqt2_dn4_slot;
        let mut var_sqt2_dn6: f64 = *var_sqt2_dn6_slot;
        let mut var_sqt2_dn7: f64 = *var_sqt2_dn7_slot;
        let mut var_sqt2_dn8: f64 = *var_sqt2_dn8_slot;
        let mut var_sqt2_dn9: f64 = *var_sqt2_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_temp__blk1748: f64 = *var_temp__blk1748_slot;
        let mut var_temp__blk1748_dn4: f64 = *var_temp__blk1748_dn4_slot;
        let mut var_temp__blk1748_dn6: f64 = *var_temp__blk1748_dn6_slot;
        let mut var_temp__blk1748_dn7: f64 = *var_temp__blk1748_dn7_slot;
        let mut var_temp__blk1748_dn8: f64 = *var_temp__blk1748_dn8_slot;
        let mut var_temp__blk1748_dn9: f64 = *var_temp__blk1748_dn9_slot;
        let mut var_tmp__blk1560: f64 = *var_tmp__blk1560_slot;
        let mut var_tmp__blk1560_dn11: f64 = *var_tmp__blk1560_dn11_slot;
        let mut var_tmp__blk1560_dn12: f64 = *var_tmp__blk1560_dn12_slot;
        let mut var_tmp__blk1560_dn6: f64 = *var_tmp__blk1560_dn6_slot;
        let mut var_tmp__blk1560_dn7: f64 = *var_tmp__blk1560_dn7_slot;
        let mut var_tmp__blk1560_dn8: f64 = *var_tmp__blk1560_dn8_slot;
        let mut var_tmp__blk1560_dn9: f64 = *var_tmp__blk1560_dn9_slot;
        let mut var_vjtmp: f64 = *var_vjtmp_slot;
        let mut var_vjtmp_dn11: f64 = *var_vjtmp_dn11_slot;
        let mut var_vjtmp_dn12: f64 = *var_vjtmp_dn12_slot;
        let mut var_vjtmp_dn7: f64 = *var_vjtmp_dn7_slot;
        let mut var_vjtmp_dn8: f64 = *var_vjtmp_dn8_slot;

        let (assign61870_e80506, assign61870_e80506_d_n7, assign61870_e80506_d_n8, assign61870_e80506_d_n11, assign61870_e80506_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61870_e80504: f64 = (var_vfmin_d - var_h2d__blk1547);
        (assign61870_e80504, (-var_h2d__blk1547_dn7), (-var_h2d__blk1547_dn8), (-var_h2d__blk1547_dn11), (-var_h2d__blk1547_dn12),)
    } else {
        (var_h4__blk1549, var_h4__blk1549_dn7, var_h4__blk1549_dn8, var_h4__blk1549_dn11, var_h4__blk1549_dn12,)
    }
};
        var_h4__blk1549 = assign61870_e80506;
        var_h4__blk1549_dn7 = assign61870_e80506_d_n7;
        var_h4__blk1549_dn8 = assign61870_e80506_d_n8;
        var_h4__blk1549_dn11 = assign61870_e80506_d_n11;
        var_h4__blk1549_dn12 = assign61870_e80506_d_n12;

        let (assign61880_e80523, assign61880_e80523_d_n7, assign61880_e80523_d_n8, assign61880_e80523_d_n11, assign61880_e80523_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61880_e80518: f64 = (var_h4__blk1549 * var_h4__blk1549);
        let assign61880_e80520: f64 = (assign61880_e80518 + var_h1__blk1545);
        let assign61880_e80521: f64 = (assign61880_e80520).sqrt();
        (assign61880_e80521, (((var_h4__blk1549_dn7 * var_h4__blk1549) + (var_h4__blk1549 * var_h4__blk1549_dn7)) / (2.0 * assign61880_e80521)), (((var_h4__blk1549_dn8 * var_h4__blk1549) + (var_h4__blk1549 * var_h4__blk1549_dn8)) / (2.0 * assign61880_e80521)), (((var_h4__blk1549_dn11 * var_h4__blk1549) + (var_h4__blk1549 * var_h4__blk1549_dn11)) / (2.0 * assign61880_e80521)), (((var_h4__blk1549_dn12 * var_h4__blk1549) + (var_h4__blk1549 * var_h4__blk1549_dn12)) / (2.0 * assign61880_e80521)),)
    } else {
        (var_h5__blk1550, var_h5__blk1550_dn7, var_h5__blk1550_dn8, var_h5__blk1550_dn11, var_h5__blk1550_dn12,)
    }
};
        var_h5__blk1550 = assign61880_e80523;
        var_h5__blk1550_dn7 = assign61880_e80523_d_n7;
        var_h5__blk1550_dn8 = assign61880_e80523_d_n8;
        var_h5__blk1550_dn11 = assign61880_e80523_d_n11;
        var_h5__blk1550_dn12 = assign61880_e80523_d_n12;

        let (assign61890_e80543, assign61890_e80543_d_n7, assign61890_e80543_d_n8, assign61890_e80543_d_n11, assign61890_e80543_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61890_e80536: f64 = (var_nu__blk1587 * var_vfmin_d);
        let assign61890_e80539: f64 = (var_h3__blk1548 + var_h5__blk1550);
        let assign61890_e80540: f64 = (assign61890_e80536 / assign61890_e80539);
        let assign61890_e80541: f64 = (2.0 * assign61890_e80540);
        (assign61890_e80541, (2.0 * ((((var_nu__blk1587_dn7 * var_vfmin_d) * assign61890_e80539) - (assign61890_e80536 * (var_h3__blk1548_dn7 + var_h5__blk1550_dn7))) / (assign61890_e80539 * assign61890_e80539))), (2.0 * ((((var_nu__blk1587_dn8 * var_vfmin_d) * assign61890_e80539) - (assign61890_e80536 * (var_h3__blk1548_dn8 + var_h5__blk1550_dn8))) / (assign61890_e80539 * assign61890_e80539))), (2.0 * ((((var_nu__blk1587_dn11 * var_vfmin_d) * assign61890_e80539) - (assign61890_e80536 * (var_h3__blk1548_dn11 + var_h5__blk1550_dn11))) / (assign61890_e80539 * assign61890_e80539))), (2.0 * ((((var_nu__blk1587_dn12 * var_vfmin_d) * assign61890_e80539) - (assign61890_e80536 * (var_h3__blk1548_dn12 + var_h5__blk1550_dn12))) / (assign61890_e80539 * assign61890_e80539))),)
    } else {
        (var_vjtmp, var_vjtmp_dn7, var_vjtmp_dn8, var_vjtmp_dn11, var_vjtmp_dn12,)
    }
};
        var_vjtmp = assign61890_e80543;
        var_vjtmp_dn7 = assign61890_e80543_d_n7;
        var_vjtmp_dn8 = assign61890_e80543_d_n8;
        var_vjtmp_dn11 = assign61890_e80543_d_n11;
        var_vjtmp_dn12 = assign61890_e80543_d_n12;

        let assign61900_e80546: f64 = if var_one_minus_pgat2nd_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1733 = assign61900_e80546;

        let (assign61910_e80565, assign61910_e80565_d_n6, assign61910_e80565_d_n7, assign61910_e80565_d_n8, assign61910_e80565_d_n9, assign61910_e80565_d_n11, assign61910_e80565_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) && (var_guard1733 != 0.0)) {
        let assign61910_e80561: f64 = (var_vjtmp * var_vbiinvgat2nd_d);
        let assign61910_e80562: f64 = (1.0 - assign61910_e80561);
        let assign61910_e80563: f64 = (assign61910_e80562).sqrt();
        (assign61910_e80563, 0.0, ((-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)) / (2.0 * assign61910_e80563)), ((-(var_vjtmp_dn8 * var_vbiinvgat2nd_d)) / (2.0 * assign61910_e80563)), 0.0, ((-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)) / (2.0 * assign61910_e80563)), ((-(var_vjtmp_dn12 * var_vbiinvgat2nd_d)) / (2.0 * assign61910_e80563)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61910_e80565;
        var_tmp__blk1560_dn6 = assign61910_e80565_d_n6;
        var_tmp__blk1560_dn7 = assign61910_e80565_d_n7;
        var_tmp__blk1560_dn8 = assign61910_e80565_d_n8;
        var_tmp__blk1560_dn9 = assign61910_e80565_d_n9;
        var_tmp__blk1560_dn11 = assign61910_e80565_d_n11;
        var_tmp__blk1560_dn12 = assign61910_e80565_d_n12;

        let (assign61920_e80586, assign61920_e80586_d_n6, assign61920_e80586_d_n7, assign61920_e80586_d_n8, assign61920_e80586_d_n9, assign61920_e80586_d_n11, assign61920_e80586_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) && (var_guard1733 == 0.0)) {
        let assign61920_e80581: f64 = (var_vjtmp * var_vbiinvgat2nd_d);
        let assign61920_e80582: f64 = (1.0 - assign61920_e80581);
        let assign61920_e80584: f64 = (assign61920_e80582).powf(var_one_minus_pgat2nd_d);
        (assign61920_e80584, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61920_e80582).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)))) } } else { (assign61920_e80584 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn7 * var_vbiinvgat2nd_d)) / assign61920_e80582))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61920_e80582).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn8 * var_vbiinvgat2nd_d)))) } } else { (assign61920_e80584 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn8 * var_vbiinvgat2nd_d)) / assign61920_e80582))) }, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61920_e80582).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)))) } } else { (assign61920_e80584 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn11 * var_vbiinvgat2nd_d)) / assign61920_e80582))) }, if 0.0 == 0.0 && ((var_one_minus_pgat2nd_d) as f64).is_finite() && ((var_one_minus_pgat2nd_d) as f64).fract() == 0.0 { if var_one_minus_pgat2nd_d == 0.0 { 0.0 } else { (var_one_minus_pgat2nd_d * ((assign61920_e80582).powf(var_one_minus_pgat2nd_d - 1.0) * (-(var_vjtmp_dn12 * var_vbiinvgat2nd_d)))) } } else { (assign61920_e80584 * (var_one_minus_pgat2nd_d * ((-(var_vjtmp_dn12 * var_vbiinvgat2nd_d)) / assign61920_e80582))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61920_e80586;
        var_tmp__blk1560_dn6 = assign61920_e80586_d_n6;
        var_tmp__blk1560_dn7 = assign61920_e80586_d_n7;
        var_tmp__blk1560_dn8 = assign61920_e80586_d_n8;
        var_tmp__blk1560_dn9 = assign61920_e80586_d_n9;
        var_tmp__blk1560_dn11 = assign61920_e80586_d_n11;
        var_tmp__blk1560_dn12 = assign61920_e80586_d_n12;

        let (assign61930_e80610, assign61930_e80610_d_n6, assign61930_e80610_d_n7, assign61930_e80610_d_n8, assign61930_e80610_d_n9, assign61930_e80610_d_n11, assign61930_e80610_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61930_e80600: f64 = (1.0 - var_tmp__blk1560);
        let assign61930_e80601: f64 = (var_qprefgat2nd_d * assign61930_e80600);
        let assign61930_e80605: f64 = (var_nu__blk1587 - var_vjtmp);
        let assign61930_e80606: f64 = (var_qpref2gat2nd_d * assign61930_e80605);
        let assign61930_e80607: f64 = (assign61930_e80601 + assign61930_e80606);
        let assign61930_e80608: f64 = (p.p30 * assign61930_e80607);
        (assign61930_e80608, (p.p30 * (var_qprefgat2nd_d * (-var_tmp__blk1560_dn6))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1560_dn7)) + (var_qpref2gat2nd_d * (var_nu__blk1587_dn7 - var_vjtmp_dn7)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1560_dn8)) + (var_qpref2gat2nd_d * (var_nu__blk1587_dn8 - var_vjtmp_dn8)))), (p.p30 * (var_qprefgat2nd_d * (-var_tmp__blk1560_dn9))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1560_dn11)) + (var_qpref2gat2nd_d * (var_nu__blk1587_dn11 - var_vjtmp_dn11)))), (p.p30 * ((var_qprefgat2nd_d * (-var_tmp__blk1560_dn12)) + (var_qpref2gat2nd_d * (var_nu__blk1587_dn12 - var_vjtmp_dn12)))),)
    } else {
        (var_qjungat2nd, var_qjungat2nd_dn6, var_qjungat2nd_dn7, var_qjungat2nd_dn8, var_qjungat2nd_dn9, var_qjungat2nd_dn11, var_qjungat2nd_dn12,)
    }
};
        var_qjungat2nd = assign61930_e80610;
        var_qjungat2nd_dn6 = assign61930_e80610_d_n6;
        var_qjungat2nd_dn7 = assign61930_e80610_d_n7;
        var_qjungat2nd_dn8 = assign61930_e80610_d_n8;
        var_qjungat2nd_dn9 = assign61930_e80610_d_n9;
        var_qjungat2nd_dn11 = assign61930_e80610_d_n11;
        var_qjungat2nd_dn12 = assign61930_e80610_d_n12;

        let (assign61940_e80624, assign61940_e80624_d_n6, assign61940_e80624_d_n7, assign61940_e80624_d_n8, assign61940_e80624_d_n9, assign61940_e80624_d_n11, assign61940_e80624_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 != 0.0)) {
        let assign61940_e80622: f64 = (var_qjungat_d + var_qjungat2nd);
        (assign61940_e80622, (var_qjungat_d_dn6 + var_qjungat2nd_dn6), (var_qjungat_d_dn7 + var_qjungat2nd_dn7), (var_qjungat_d_dn8 + var_qjungat2nd_dn8), (var_qjungat_d_dn9 + var_qjungat2nd_dn9), (var_qjungat_d_dn11 + var_qjungat2nd_dn11), (var_qjungat_d_dn12 + var_qjungat2nd_dn12),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn9, var_qjungat_d_dn11, var_qjungat_d_dn12,)
    }
};
        var_qjungat_d = assign61940_e80624;
        var_qjungat_d_dn6 = assign61940_e80624_d_n6;
        var_qjungat_d_dn7 = assign61940_e80624_d_n7;
        var_qjungat_d_dn8 = assign61940_e80624_d_n8;
        var_qjungat_d_dn9 = assign61940_e80624_d_n9;
        var_qjungat_d_dn11 = assign61940_e80624_d_n11;
        var_qjungat_d_dn12 = assign61940_e80624_d_n12;

        let assign61950_e80627: f64 = if var_one_minus_pgat_d == 0.5 { 1.0 } else { 0.0 };
        var_guard1734 = assign61950_e80627;

        let (assign61960_e80647, assign61960_e80647_d_n6, assign61960_e80647_d_n7, assign61960_e80647_d_n8, assign61960_e80647_d_n9, assign61960_e80647_d_n11, assign61960_e80647_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 == 0.0)) && (var_guard1734 != 0.0)) {
        let assign61960_e80643: f64 = (var_vj__blk1552 * var_vbiinvgat_d);
        let assign61960_e80644: f64 = (1.0 - assign61960_e80643);
        let assign61960_e80645: f64 = (assign61960_e80644).sqrt();
        (assign61960_e80645, 0.0, ((-(var_vj__blk1552_dn7 * var_vbiinvgat_d)) / (2.0 * assign61960_e80645)), ((-(var_vj__blk1552_dn8 * var_vbiinvgat_d)) / (2.0 * assign61960_e80645)), 0.0, ((-(var_vj__blk1552_dn11 * var_vbiinvgat_d)) / (2.0 * assign61960_e80645)), ((-(var_vj__blk1552_dn12 * var_vbiinvgat_d)) / (2.0 * assign61960_e80645)),)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61960_e80647;
        var_tmp__blk1560_dn6 = assign61960_e80647_d_n6;
        var_tmp__blk1560_dn7 = assign61960_e80647_d_n7;
        var_tmp__blk1560_dn8 = assign61960_e80647_d_n8;
        var_tmp__blk1560_dn9 = assign61960_e80647_d_n9;
        var_tmp__blk1560_dn11 = assign61960_e80647_d_n11;
        var_tmp__blk1560_dn12 = assign61960_e80647_d_n12;

        let (assign61970_e80669, assign61970_e80669_d_n6, assign61970_e80669_d_n7, assign61970_e80669_d_n8, assign61970_e80669_d_n9, assign61970_e80669_d_n11, assign61970_e80669_d_n12,) = {
    if (((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 == 0.0)) && (var_guard1734 == 0.0)) {
        let assign61970_e80664: f64 = (var_vj__blk1552 * var_vbiinvgat_d);
        let assign61970_e80665: f64 = (1.0 - assign61970_e80664);
        let assign61970_e80667: f64 = (assign61970_e80665).powf(var_one_minus_pgat_d);
        (assign61970_e80667, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61970_e80665).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1552_dn7 * var_vbiinvgat_d)))) } } else { (assign61970_e80667 * (var_one_minus_pgat_d * ((-(var_vj__blk1552_dn7 * var_vbiinvgat_d)) / assign61970_e80665))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61970_e80665).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1552_dn8 * var_vbiinvgat_d)))) } } else { (assign61970_e80667 * (var_one_minus_pgat_d * ((-(var_vj__blk1552_dn8 * var_vbiinvgat_d)) / assign61970_e80665))) }, 0.0, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61970_e80665).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1552_dn11 * var_vbiinvgat_d)))) } } else { (assign61970_e80667 * (var_one_minus_pgat_d * ((-(var_vj__blk1552_dn11 * var_vbiinvgat_d)) / assign61970_e80665))) }, if 0.0 == 0.0 && ((var_one_minus_pgat_d) as f64).is_finite() && ((var_one_minus_pgat_d) as f64).fract() == 0.0 { if var_one_minus_pgat_d == 0.0 { 0.0 } else { (var_one_minus_pgat_d * ((assign61970_e80665).powf(var_one_minus_pgat_d - 1.0) * (-(var_vj__blk1552_dn12 * var_vbiinvgat_d)))) } } else { (assign61970_e80667 * (var_one_minus_pgat_d * ((-(var_vj__blk1552_dn12 * var_vbiinvgat_d)) / assign61970_e80665))) },)
    } else {
        (var_tmp__blk1560, var_tmp__blk1560_dn6, var_tmp__blk1560_dn7, var_tmp__blk1560_dn8, var_tmp__blk1560_dn9, var_tmp__blk1560_dn11, var_tmp__blk1560_dn12,)
    }
};
        var_tmp__blk1560 = assign61970_e80669;
        var_tmp__blk1560_dn6 = assign61970_e80669_d_n6;
        var_tmp__blk1560_dn7 = assign61970_e80669_d_n7;
        var_tmp__blk1560_dn8 = assign61970_e80669_d_n8;
        var_tmp__blk1560_dn9 = assign61970_e80669_d_n9;
        var_tmp__blk1560_dn11 = assign61970_e80669_d_n11;
        var_tmp__blk1560_dn12 = assign61970_e80669_d_n12;

        let (assign61980_e80694, assign61980_e80694_d_n6, assign61980_e80694_d_n7, assign61980_e80694_d_n8, assign61980_e80694_d_n9, assign61980_e80694_d_n11, assign61980_e80694_d_n12,) = {
    if ((((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) && (var_guard1714 == 0.0)) && (var_guard1731 == 0.0)) {
        let assign61980_e80684: f64 = (1.0 - var_tmp__blk1560);
        let assign61980_e80685: f64 = (var_qprefgat_d * assign61980_e80684);
        let assign61980_e80689: f64 = (var_vjun_d - var_vj__blk1552);
        let assign61980_e80690: f64 = (var_qpref2gat_d * assign61980_e80689);
        let assign61980_e80691: f64 = (assign61980_e80685 + assign61980_e80690);
        let assign61980_e80692: f64 = (p.p30 * assign61980_e80691);
        (assign61980_e80692, (p.p30 * (var_qprefgat_d * (-var_tmp__blk1560_dn6))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1560_dn7)) + (var_qpref2gat_d * (-var_vj__blk1552_dn7)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1560_dn8)) + (var_qpref2gat_d * (var_vjun_d_dn8 - var_vj__blk1552_dn8)))), (p.p30 * (var_qprefgat_d * (-var_tmp__blk1560_dn9))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1560_dn11)) + (var_qpref2gat_d * (-var_vj__blk1552_dn11)))), (p.p30 * ((var_qprefgat_d * (-var_tmp__blk1560_dn12)) + (var_qpref2gat_d * (var_vjun_d_dn12 - var_vj__blk1552_dn12)))),)
    } else {
        (var_qjungat_d, var_qjungat_d_dn6, var_qjungat_d_dn7, var_qjungat_d_dn8, var_qjungat_d_dn9, var_qjungat_d_dn11, var_qjungat_d_dn12,)
    }
};
        var_qjungat_d = assign61980_e80694;
        var_qjungat_d_dn6 = assign61980_e80694_d_n6;
        var_qjungat_d_dn7 = assign61980_e80694_d_n7;
        var_qjungat_d_dn8 = assign61980_e80694_d_n8;
        var_qjungat_d_dn9 = assign61980_e80694_d_n9;
        var_qjungat_d_dn11 = assign61980_e80694_d_n11;
        var_qjungat_d_dn12 = assign61980_e80694_d_n12;

        let (assign61990_e80711, assign61990_e80711_d_n6, assign61990_e80711_d_n7, assign61990_e80711_d_n8, assign61990_e80711_d_n9, assign61990_e80711_d_n11, assign61990_e80711_d_n12,) = {
    if ((var_guard1589 != 0.0) && (var_guard1590 == 0.0)) {
        let assign61990_e80701: f64 = (var_abdrain_i * var_ijunbot_d);
        let assign61990_e80704: f64 = (var_lsdrain_i * var_ijunsti_d);
        let assign61990_e80705: f64 = (assign61990_e80701 + assign61990_e80704);
        let assign61990_e80708: f64 = (var_lgdrain_i * var_ijungat_d);
        let assign61990_e80709: f64 = (assign61990_e80705 + assign61990_e80708);
        (assign61990_e80709, (((var_abdrain_i * var_ijunbot_d_dn6) + (var_lsdrain_i * var_ijunsti_d_dn6)) + (var_lgdrain_i * var_ijungat_d_dn6)), (((var_abdrain_i * var_ijunbot_d_dn7) + (var_lsdrain_i * var_ijunsti_d_dn7)) + (var_lgdrain_i * var_ijungat_d_dn7)), (((var_abdrain_i * var_ijunbot_d_dn8) + (var_lsdrain_i * var_ijunsti_d_dn8)) + (var_lgdrain_i * var_ijungat_d_dn8)), (((var_abdrain_i * var_ijunbot_d_dn9) + (var_lsdrain_i * var_ijunsti_d_dn9)) + (var_lgdrain_i * var_ijungat_d_dn9)), (((var_abdrain_i * var_ijunbot_d_dn11) + (var_lsdrain_i * var_ijunsti_d_dn11)) + (var_lgdrain_i * var_ijungat_d_dn11)), (((var_abdrain_i * var_ijunbot_d_dn12) + (var_lsdrain_i * var_ijunsti_d_dn12)) + (var_lgdrain_i * var_ijungat_d_dn12)),)
    } else {
        (var_ijun_d, var_ijun_d_dn6, var_ijun_d_dn7, var_ijun_d_dn8, var_ijun_d_dn9, var_ijun_d_dn11, var_ijun_d_dn12,)
    }
};
        var_ijun_d = assign61990_e80711;
        var_ijun_d_dn6 = assign61990_e80711_d_n6;
        var_ijun_d_dn7 = assign61990_e80711_d_n7;
        var_ijun_d_dn8 = assign61990_e80711_d_n8;
        var_ijun_d_dn9 = assign61990_e80711_d_n9;
        var_ijun_d_dn11 = assign61990_e80711_d_n11;
        var_ijun_d_dn12 = assign61990_e80711_d_n12;

        let assign62070_e80735: f64 = if var_sigvds > 0.0 { 1.0 } else { 0.0 };
        var_guard1735 = assign62070_e80735;

        let assign62080_e80738: f64 = if var_rg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1736 = assign62080_e80738;

        let assign62090_e80741: f64 = if var_rse_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1737 = assign62090_e80741;

        let assign62100_e80744: f64 = if var_rde_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1738 = assign62100_e80744;

        let assign62110_e80747: f64 = if var_rbulk_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1739 = assign62110_e80747;

        let assign62120_e80750: f64 = if var_rjuns_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1740 = assign62120_e80750;

        let assign62130_e80753: f64 = if var_rjund_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1741 = assign62130_e80753;

        let assign62140_e80756: f64 = if var_rwell_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1742 = assign62140_e80756;

        var_pdiss_1 = 0.0;
        var_pdiss_1_dn0 = 0.0;
        var_pdiss_1_dn2 = 0.0;
        var_pdiss_1_dn4 = 0.0;
        var_pdiss_1_dn6 = 0.0;
        var_pdiss_1_dn7 = 0.0;
        var_pdiss_1_dn8 = 0.0;
        var_pdiss_1_dn9 = 0.0;

        var_pdiss_s = 0.0;
        var_pdiss_s_dn2 = 0.0;
        var_pdiss_s_dn7 = 0.0;

        var_pdiss_d = 0.0;
        var_pdiss_d_dn0 = 0.0;
        var_pdiss_d_dn8 = 0.0;

        let assign62180_e80762: f64 = if var_rse_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1745 = assign62180_e80762;

        let (assign62190_e80770, assign62190_e80770_d_n2, assign62190_e80770_d_n7,) = {
    if (var_guard1745 != 0.0) {
        let assign62190_e80766: f64 = (var_gsource * (nv2 - nv7));
        let assign62190_e80768: f64 = (assign62190_e80766 * (nv2 - nv7));
        (assign62190_e80768, ((var_gsource * (nv2 - nv7)) + assign62190_e80766), (((-var_gsource) * (nv2 - nv7)) + (-assign62190_e80766)),)
    } else {
        (var_pdiss_s, var_pdiss_s_dn2, var_pdiss_s_dn7,)
    }
};
        var_pdiss_s = assign62190_e80770;
        var_pdiss_s_dn2 = assign62190_e80770_d_n2;
        var_pdiss_s_dn7 = assign62190_e80770_d_n7;

        let assign62200_e80773: f64 = if var_rde_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1746 = assign62200_e80773;

        let (assign62210_e80781, assign62210_e80781_d_n0, assign62210_e80781_d_n8,) = {
    if (var_guard1746 != 0.0) {
        let assign62210_e80777: f64 = (var_gdrain * (nv0 - nv8));
        let assign62210_e80779: f64 = (assign62210_e80777 * (nv0 - nv8));
        (assign62210_e80779, ((var_gdrain * (nv0 - nv8)) + assign62210_e80777), (((-var_gdrain) * (nv0 - nv8)) + (-assign62210_e80777)),)
    } else {
        (var_pdiss_d, var_pdiss_d_dn0, var_pdiss_d_dn8,)
    }
};
        var_pdiss_d = assign62210_e80781;
        var_pdiss_d_dn0 = assign62210_e80781_d_n0;
        var_pdiss_d_dn8 = assign62210_e80781_d_n8;

        let assign62220_e80784: f64 = if var_rth_p > 0.001 { 1.0 } else { 0.0 };
        var_guard1747 = assign62220_e80784;

        let (assign62230_e80802, assign62230_e80802_d_n0, assign62230_e80802_d_n2, assign62230_e80802_d_n4, assign62230_e80802_d_n6, assign62230_e80802_d_n7, assign62230_e80802_d_n8, assign62230_e80802_d_n9,) = {
    if (var_guard1747 != 0.0) {
        let assign62230_e80788: f64 = (var_i_ds + var_i_dsedge);
        let assign62230_e80790: f64 = (assign62230_e80788 * var_v_ds);
        let assign62230_e80794: f64 = (var_v_ds + var_v_sb);
        let assign62230_e80795: f64 = (var_iimpact * assign62230_e80794);
        let assign62230_e80796: f64 = (assign62230_e80790 + assign62230_e80795);
        let assign62230_e80798: f64 = (assign62230_e80796 + var_pdiss_s);
        let assign62230_e80800: f64 = (assign62230_e80798 + var_pdiss_d);
        (assign62230_e80800, var_pdiss_d_dn0, var_pdiss_s_dn2, (((var_i_ds_dn4 + var_i_dsedge_dn4) * var_v_ds) + (var_iimpact_dn4 * assign62230_e80794)), (((var_i_ds_dn6 + var_i_dsedge_dn6) * var_v_ds) + (var_iimpact_dn6 * assign62230_e80794)), (((((var_i_ds_dn7 + var_i_dsedge_dn7) * var_v_ds) + (assign62230_e80788 * var_v_ds_dn7)) + ((var_iimpact_dn7 * assign62230_e80794) + (var_iimpact * (var_v_ds_dn7 + var_v_sb_dn7)))) + var_pdiss_s_dn7), (((((var_i_ds_dn8 + var_i_dsedge_dn8) * var_v_ds) + (assign62230_e80788 * var_v_ds_dn8)) + ((var_iimpact_dn8 * assign62230_e80794) + (var_iimpact * (var_v_ds_dn8 + var_v_sb_dn8)))) + var_pdiss_d_dn8), (((var_i_ds_dn9 + var_i_dsedge_dn9) * var_v_ds) + ((var_iimpact_dn9 * assign62230_e80794) + (var_iimpact * var_v_sb_dn9))),)
    } else {
        (var_pdiss_1, var_pdiss_1_dn0, var_pdiss_1_dn2, var_pdiss_1_dn4, var_pdiss_1_dn6, var_pdiss_1_dn7, var_pdiss_1_dn8, var_pdiss_1_dn9,)
    }
};
        var_pdiss_1 = assign62230_e80802;
        var_pdiss_1_dn0 = assign62230_e80802_d_n0;
        var_pdiss_1_dn2 = assign62230_e80802_d_n2;
        var_pdiss_1_dn4 = assign62230_e80802_d_n4;
        var_pdiss_1_dn6 = assign62230_e80802_d_n6;
        var_pdiss_1_dn7 = assign62230_e80802_d_n7;
        var_pdiss_1_dn8 = assign62230_e80802_d_n8;
        var_pdiss_1_dn9 = assign62230_e80802_d_n9;

        let assign62240_e80805: f64 = (var_qg + var_qb);
        let assign62240_e80807: f64 = (assign62240_e80805 + var_qd);
        let assign62240_e80808: f64 = (-assign62240_e80807);
        var_qs = assign62240_e80808;
        var_qs_dn4 = (-((var_qg_dn4 + var_qb_dn4) + var_qd_dn4));
        var_qs_dn6 = (-((var_qg_dn6 + var_qb_dn6) + var_qd_dn6));
        var_qs_dn7 = (-((var_qg_dn7 + var_qb_dn7) + var_qd_dn7));
        var_qs_dn8 = (-((var_qg_dn8 + var_qb_dn8) + var_qd_dn8));
        var_qs_dn9 = (-((var_qg_dn9 + var_qb_dn9) + var_qd_dn9));

        let assign62250_e80811: f64 = (var_qfgs + var_qgs_ov);
        var_qfgs = assign62250_e80811;
        var_qfgs_dn6 = (var_qfgs_dn6 + var_qgs_ov_dn6);
        var_qfgs_dn7 = (var_qfgs_dn7 + var_qgs_ov_dn7);
        var_qfgs_dn8 = (var_qfgs_dn8 + var_qgs_ov_dn8);

        let assign62260_e80814: f64 = (var_qfgd + var_qgd_ov);
        var_qfgd = assign62260_e80814;
        var_qfgd_dn6 = (var_qfgd_dn6 + var_qgd_ov_dn6);
        var_qfgd_dn7 = (var_qfgd_dn7 + var_qgd_ov_dn7);
        var_qfgd_dn8 = (var_qfgd_dn8 + var_qgd_ov_dn8);

        let assign62270_e80817: f64 = (var_absource_i * var_qjunbot_s);
        let assign62270_e80820: f64 = (var_lssource_i * var_qjunsti_s);
        let assign62270_e80821: f64 = (assign62270_e80817 + assign62270_e80820);
        let assign62270_e80824: f64 = (var_lgsource_i * var_qjungat_s);
        let assign62270_e80825: f64 = (assign62270_e80821 + assign62270_e80824);
        var_qjun_s = assign62270_e80825;
        var_qjun_s_dn6 = (((var_absource_i * var_qjunbot_s_dn6) + (var_lssource_i * var_qjunsti_s_dn6)) + (var_lgsource_i * var_qjungat_s_dn6));
        var_qjun_s_dn7 = (((var_absource_i * var_qjunbot_s_dn7) + (var_lssource_i * var_qjunsti_s_dn7)) + (var_lgsource_i * var_qjungat_s_dn7));
        var_qjun_s_dn8 = (((var_absource_i * var_qjunbot_s_dn8) + (var_lssource_i * var_qjunsti_s_dn8)) + (var_lgsource_i * var_qjungat_s_dn8));
        var_qjun_s_dn9 = (((var_absource_i * var_qjunbot_s_dn9) + (var_lssource_i * var_qjunsti_s_dn9)) + (var_lgsource_i * var_qjungat_s_dn9));
        var_qjun_s_dn11 = (((var_absource_i * var_qjunbot_s_dn11) + (var_lssource_i * var_qjunsti_s_dn11)) + (var_lgsource_i * var_qjungat_s_dn11));
        var_qjun_s_dn12 = (((var_absource_i * var_qjunbot_s_dn12) + (var_lssource_i * var_qjunsti_s_dn12)) + (var_lgsource_i * var_qjungat_s_dn12));

        let assign62280_e80828: f64 = (var_abdrain_i * var_qjunbot_d);
        let assign62280_e80831: f64 = (var_lsdrain_i * var_qjunsti_d);
        let assign62280_e80832: f64 = (assign62280_e80828 + assign62280_e80831);
        let assign62280_e80835: f64 = (var_lgdrain_i * var_qjungat_d);
        let assign62280_e80836: f64 = (assign62280_e80832 + assign62280_e80835);
        var_qjun_d = assign62280_e80836;
        var_qjun_d_dn6 = (((var_abdrain_i * var_qjunbot_d_dn6) + (var_lsdrain_i * var_qjunsti_d_dn6)) + (var_lgdrain_i * var_qjungat_d_dn6));
        var_qjun_d_dn7 = (((var_abdrain_i * var_qjunbot_d_dn7) + (var_lsdrain_i * var_qjunsti_d_dn7)) + (var_lgdrain_i * var_qjungat_d_dn7));
        var_qjun_d_dn8 = (((var_abdrain_i * var_qjunbot_d_dn8) + (var_lsdrain_i * var_qjunsti_d_dn8)) + (var_lgdrain_i * var_qjungat_d_dn8));
        var_qjun_d_dn9 = (((var_abdrain_i * var_qjunbot_d_dn9) + (var_lsdrain_i * var_qjunsti_d_dn9)) + (var_lgdrain_i * var_qjungat_d_dn9));
        var_qjun_d_dn11 = (((var_abdrain_i * var_qjunbot_d_dn11) + (var_lsdrain_i * var_qjunsti_d_dn11)) + (var_lgdrain_i * var_qjungat_d_dn11));
        var_qjun_d_dn12 = (((var_abdrain_i * var_qjunbot_d_dn12) + (var_lsdrain_i * var_qjunsti_d_dn12)) + (var_lgdrain_i * var_qjungat_d_dn12));

        let assign62290_e80839: f64 = if var_sigvds < 0.0 { 1.0 } else { 0.0 };
        var_guard1749 = assign62290_e80839;

        let (assign62300_e80843, assign62300_e80843_d_n4, assign62300_e80843_d_n6, assign62300_e80843_d_n7, assign62300_e80843_d_n8, assign62300_e80843_d_n9,) = {
    if (var_guard1749 != 0.0) {
        (var_qd, var_qd_dn4, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9,)
    } else {
        (var_temp__blk1748, var_temp__blk1748_dn4, var_temp__blk1748_dn6, var_temp__blk1748_dn7, var_temp__blk1748_dn8, var_temp__blk1748_dn9,)
    }
};
        var_temp__blk1748 = assign62300_e80843;
        var_temp__blk1748_dn4 = assign62300_e80843_d_n4;
        var_temp__blk1748_dn6 = assign62300_e80843_d_n6;
        var_temp__blk1748_dn7 = assign62300_e80843_d_n7;
        var_temp__blk1748_dn8 = assign62300_e80843_d_n8;
        var_temp__blk1748_dn9 = assign62300_e80843_d_n9;

        let (assign62310_e80847, assign62310_e80847_d_n4, assign62310_e80847_d_n6, assign62310_e80847_d_n7, assign62310_e80847_d_n8, assign62310_e80847_d_n9,) = {
    if (var_guard1749 != 0.0) {
        (var_qs, var_qs_dn4, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9,)
    } else {
        (var_qd, var_qd_dn4, var_qd_dn6, var_qd_dn7, var_qd_dn8, var_qd_dn9,)
    }
};
        var_qd = assign62310_e80847;
        var_qd_dn4 = assign62310_e80847_d_n4;
        var_qd_dn6 = assign62310_e80847_d_n6;
        var_qd_dn7 = assign62310_e80847_d_n7;
        var_qd_dn8 = assign62310_e80847_d_n8;
        var_qd_dn9 = assign62310_e80847_d_n9;

        let (assign62320_e80851, assign62320_e80851_d_n4, assign62320_e80851_d_n6, assign62320_e80851_d_n7, assign62320_e80851_d_n8, assign62320_e80851_d_n9,) = {
    if (var_guard1749 != 0.0) {
        (var_temp__blk1748, var_temp__blk1748_dn4, var_temp__blk1748_dn6, var_temp__blk1748_dn7, var_temp__blk1748_dn8, var_temp__blk1748_dn9,)
    } else {
        (var_qs, var_qs_dn4, var_qs_dn6, var_qs_dn7, var_qs_dn8, var_qs_dn9,)
    }
};
        var_qs = assign62320_e80851;
        var_qs_dn4 = assign62320_e80851_d_n4;
        var_qs_dn6 = assign62320_e80851_d_n6;
        var_qs_dn7 = assign62320_e80851_d_n7;
        var_qs_dn8 = assign62320_e80851_d_n8;
        var_qs_dn9 = assign62320_e80851_d_n9;

        var_sidexc = 0.0;
        var_sidexc_dn4 = 0.0;
        var_sidexc_dn6 = 0.0;
        var_sidexc_dn7 = 0.0;
        var_sidexc_dn8 = 0.0;
        var_sidexc_dn9 = 0.0;

        var_mid = 0.0;
        var_mid_dn4 = 0.0;
        var_mid_dn6 = 0.0;
        var_mid_dn7 = 0.0;
        var_mid_dn8 = 0.0;
        var_mid_dn9 = 0.0;

        var_mig = 1e-40;
        var_mig_dn4 = 0.0;
        var_mig_dn6 = 0.0;
        var_mig_dn7 = 0.0;
        var_mig_dn8 = 0.0;
        var_mig_dn9 = 0.0;

        var_migid = 0.0;
        var_migid_dn4 = 0.0;
        var_migid_dn6 = 0.0;
        var_migid_dn7 = 0.0;
        var_migid_dn8 = 0.0;
        var_migid_dn9 = 0.0;

        var_c_igid = 0.0;
        var_c_igid_dn4 = 0.0;
        var_c_igid_dn6 = 0.0;
        var_c_igid_dn7 = 0.0;
        var_c_igid_dn8 = 0.0;
        var_c_igid_dn9 = 0.0;

        let assign62390_e80860: f64 = (var_cox_qm * var_eta_p_ac);
        var_cgeff = assign62390_e80860;
        var_cgeff_dn4 = ((var_cox_qm_dn4 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn4));
        var_cgeff_dn6 = ((var_cox_qm_dn6 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn6));
        var_cgeff_dn7 = ((var_cox_qm_dn7 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn7));
        var_cgeff_dn8 = ((var_cox_qm_dn8 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn8));
        var_cgeff_dn9 = ((var_cox_qm_dn9 * var_eta_p_ac) + (var_cox_qm * var_eta_p_ac_dn9));

        var_sqid = 0.0;
        var_sqid_dn4 = 0.0;
        var_sqid_dn6 = 0.0;
        var_sqid_dn7 = 0.0;
        var_sqid_dn8 = 0.0;
        var_sqid_dn9 = 0.0;

        var_sqig = 0.0;
        var_sqig_dn4 = 0.0;
        var_sqig_dn6 = 0.0;
        var_sqig_dn7 = 0.0;
        var_sqig_dn8 = 0.0;
        var_sqig_dn9 = 0.0;

        let assign62450_e80872: f64 = if ((var_xg_dc > 0.0) && (var_bet_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard1782 = assign62450_e80872;

        let assign62540_e80978: f64 = if p.p32 > 0.0 { 1.0 } else { 0.0 };
        var_guard1784 = assign62540_e80978;

        let (assign62550_e80986, assign62550_e80986_d_n4, assign62550_e80986_d_n6, assign62550_e80986_d_n7, assign62550_e80986_d_n8, assign62550_e80986_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62550_e80984: f64 = (var_qim1_dc / var_alpha_dc);
        (assign62550_e80984, (((var_qim1_dc_dn4 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn4)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn6 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn6)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn7 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn7)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn8 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn8)) / (var_alpha_dc * var_alpha_dc)), (((var_qim1_dc_dn9 * var_alpha_dc) - (var_qim1_dc * var_alpha_dc_dn9)) / (var_alpha_dc * var_alpha_dc)),)
    } else {
        (var_h0, var_h0_dn4, var_h0_dn6, var_h0_dn7, var_h0_dn8, var_h0_dn9,)
    }
};
        var_h0 = assign62550_e80986;
        var_h0_dn4 = assign62550_e80986_d_n4;
        var_h0_dn6 = assign62550_e80986_d_n6;
        var_h0_dn7 = assign62550_e80986_d_n7;
        var_h0_dn8 = assign62550_e80986_d_n8;
        var_h0_dn9 = assign62550_e80986_d_n9;

        let (assign62560_e80994, assign62560_e80994_d_n4, assign62560_e80994_d_n6, assign62560_e80994_d_n7, assign62560_e80994_d_n8, assign62560_e80994_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62560_e80992: f64 = (var_qim_dc / var_qim1_dc);
        (assign62560_e80992, (((var_qim_dc_dn4 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn4)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn6 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn6)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn7 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn7)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn8 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn8)) / (var_qim1_dc * var_qim1_dc)), (((var_qim_dc_dn9 * var_qim1_dc) - (var_qim_dc * var_qim1_dc_dn9)) / (var_qim1_dc * var_qim1_dc)),)
    } else {
        (var_t1, var_t1_dn4, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9,)
    }
};
        var_t1 = assign62560_e80994;
        var_t1_dn4 = assign62560_e80994_d_n4;
        var_t1_dn6 = assign62560_e80994_d_n6;
        var_t1_dn7 = assign62560_e80994_d_n7;
        var_t1_dn8 = assign62560_e80994_d_n8;
        var_t1_dn9 = assign62560_e80994_d_n9;

        let (assign62570_e81006, assign62570_e81006_d_n4, assign62570_e81006_d_n6, assign62570_e81006_d_n7, assign62570_e81006_d_n8, assign62570_e81006_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62570_e81000: f64 = (0.5 * 0.16666666666666666);
        let assign62570_e81003: f64 = (var_dps_dc / var_h0);
        let assign62570_e81004: f64 = (assign62570_e81000 * assign62570_e81003);
        (assign62570_e81004, (assign62570_e81000 * (((var_dps_dc_dn4 * var_h0) - (var_dps_dc * var_h0_dn4)) / (var_h0 * var_h0))), (assign62570_e81000 * (((var_dps_dc_dn6 * var_h0) - (var_dps_dc * var_h0_dn6)) / (var_h0 * var_h0))), (assign62570_e81000 * (((var_dps_dc_dn7 * var_h0) - (var_dps_dc * var_h0_dn7)) / (var_h0 * var_h0))), (assign62570_e81000 * (((var_dps_dc_dn8 * var_h0) - (var_dps_dc * var_h0_dn8)) / (var_h0 * var_h0))), (assign62570_e81000 * (((var_dps_dc_dn9 * var_h0) - (var_dps_dc * var_h0_dn9)) / (var_h0 * var_h0))),)
    } else {
        (var_sqt2, var_sqt2_dn4, var_sqt2_dn6, var_sqt2_dn7, var_sqt2_dn8, var_sqt2_dn9,)
    }
};
        var_sqt2 = assign62570_e81006;
        var_sqt2_dn4 = assign62570_e81006_d_n4;
        var_sqt2_dn6 = assign62570_e81006_d_n6;
        var_sqt2_dn7 = assign62570_e81006_d_n7;
        var_sqt2_dn8 = assign62570_e81006_d_n8;
        var_sqt2_dn9 = assign62570_e81006_d_n9;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_dn4_slot = var_c_igid_dn4;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_c_igid_dn9_slot = var_c_igid_dn9;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn4_slot = var_cgeff_dn4;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_cgeff_dn9_slot = var_cgeff_dn9;
        *var_guard1733_slot = var_guard1733;
        *var_guard1734_slot = var_guard1734;
        *var_guard1735_slot = var_guard1735;
        *var_guard1736_slot = var_guard1736;
        *var_guard1737_slot = var_guard1737;
        *var_guard1738_slot = var_guard1738;
        *var_guard1739_slot = var_guard1739;
        *var_guard1740_slot = var_guard1740;
        *var_guard1741_slot = var_guard1741;
        *var_guard1742_slot = var_guard1742;
        *var_guard1745_slot = var_guard1745;
        *var_guard1746_slot = var_guard1746;
        *var_guard1747_slot = var_guard1747;
        *var_guard1749_slot = var_guard1749;
        *var_guard1782_slot = var_guard1782;
        *var_guard1784_slot = var_guard1784;
        *var_h0_slot = var_h0;
        *var_h0_dn4_slot = var_h0_dn4;
        *var_h0_dn6_slot = var_h0_dn6;
        *var_h0_dn7_slot = var_h0_dn7;
        *var_h0_dn8_slot = var_h0_dn8;
        *var_h0_dn9_slot = var_h0_dn9;
        *var_h4__blk1549_slot = var_h4__blk1549;
        *var_h4__blk1549_dn11_slot = var_h4__blk1549_dn11;
        *var_h4__blk1549_dn12_slot = var_h4__blk1549_dn12;
        *var_h4__blk1549_dn7_slot = var_h4__blk1549_dn7;
        *var_h4__blk1549_dn8_slot = var_h4__blk1549_dn8;
        *var_h5__blk1550_slot = var_h5__blk1550;
        *var_h5__blk1550_dn11_slot = var_h5__blk1550_dn11;
        *var_h5__blk1550_dn12_slot = var_h5__blk1550_dn12;
        *var_h5__blk1550_dn7_slot = var_h5__blk1550_dn7;
        *var_h5__blk1550_dn8_slot = var_h5__blk1550_dn8;
        *var_ijun_d_slot = var_ijun_d;
        *var_ijun_d_dn11_slot = var_ijun_d_dn11;
        *var_ijun_d_dn12_slot = var_ijun_d_dn12;
        *var_ijun_d_dn6_slot = var_ijun_d_dn6;
        *var_ijun_d_dn7_slot = var_ijun_d_dn7;
        *var_ijun_d_dn8_slot = var_ijun_d_dn8;
        *var_ijun_d_dn9_slot = var_ijun_d_dn9;
        *var_mid_slot = var_mid;
        *var_mid_dn4_slot = var_mid_dn4;
        *var_mid_dn6_slot = var_mid_dn6;
        *var_mid_dn7_slot = var_mid_dn7;
        *var_mid_dn8_slot = var_mid_dn8;
        *var_mid_dn9_slot = var_mid_dn9;
        *var_mig_slot = var_mig;
        *var_mig_dn4_slot = var_mig_dn4;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_mig_dn9_slot = var_mig_dn9;
        *var_migid_slot = var_migid;
        *var_migid_dn4_slot = var_migid_dn4;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_migid_dn9_slot = var_migid_dn9;
        *var_pdiss_1_slot = var_pdiss_1;
        *var_pdiss_1_dn0_slot = var_pdiss_1_dn0;
        *var_pdiss_1_dn2_slot = var_pdiss_1_dn2;
        *var_pdiss_1_dn4_slot = var_pdiss_1_dn4;
        *var_pdiss_1_dn6_slot = var_pdiss_1_dn6;
        *var_pdiss_1_dn7_slot = var_pdiss_1_dn7;
        *var_pdiss_1_dn8_slot = var_pdiss_1_dn8;
        *var_pdiss_1_dn9_slot = var_pdiss_1_dn9;
        *var_pdiss_d_slot = var_pdiss_d;
        *var_pdiss_d_dn0_slot = var_pdiss_d_dn0;
        *var_pdiss_d_dn8_slot = var_pdiss_d_dn8;
        *var_pdiss_s_slot = var_pdiss_s;
        *var_pdiss_s_dn2_slot = var_pdiss_s_dn2;
        *var_pdiss_s_dn7_slot = var_pdiss_s_dn7;
        *var_qd_slot = var_qd;
        *var_qd_dn4_slot = var_qd_dn4;
        *var_qd_dn6_slot = var_qd_dn6;
        *var_qd_dn7_slot = var_qd_dn7;
        *var_qd_dn8_slot = var_qd_dn8;
        *var_qd_dn9_slot = var_qd_dn9;
        *var_qfgd_slot = var_qfgd;
        *var_qfgd_dn6_slot = var_qfgd_dn6;
        *var_qfgd_dn7_slot = var_qfgd_dn7;
        *var_qfgd_dn8_slot = var_qfgd_dn8;
        *var_qfgs_slot = var_qfgs;
        *var_qfgs_dn6_slot = var_qfgs_dn6;
        *var_qfgs_dn7_slot = var_qfgs_dn7;
        *var_qfgs_dn8_slot = var_qfgs_dn8;
        *var_qjun_d_slot = var_qjun_d;
        *var_qjun_d_dn11_slot = var_qjun_d_dn11;
        *var_qjun_d_dn12_slot = var_qjun_d_dn12;
        *var_qjun_d_dn6_slot = var_qjun_d_dn6;
        *var_qjun_d_dn7_slot = var_qjun_d_dn7;
        *var_qjun_d_dn8_slot = var_qjun_d_dn8;
        *var_qjun_d_dn9_slot = var_qjun_d_dn9;
        *var_qjun_s_slot = var_qjun_s;
        *var_qjun_s_dn11_slot = var_qjun_s_dn11;
        *var_qjun_s_dn12_slot = var_qjun_s_dn12;
        *var_qjun_s_dn6_slot = var_qjun_s_dn6;
        *var_qjun_s_dn7_slot = var_qjun_s_dn7;
        *var_qjun_s_dn8_slot = var_qjun_s_dn8;
        *var_qjun_s_dn9_slot = var_qjun_s_dn9;
        *var_qjungat2nd_slot = var_qjungat2nd;
        *var_qjungat2nd_dn11_slot = var_qjungat2nd_dn11;
        *var_qjungat2nd_dn12_slot = var_qjungat2nd_dn12;
        *var_qjungat2nd_dn6_slot = var_qjungat2nd_dn6;
        *var_qjungat2nd_dn7_slot = var_qjungat2nd_dn7;
        *var_qjungat2nd_dn8_slot = var_qjungat2nd_dn8;
        *var_qjungat2nd_dn9_slot = var_qjungat2nd_dn9;
        *var_qjungat_d_slot = var_qjungat_d;
        *var_qjungat_d_dn11_slot = var_qjungat_d_dn11;
        *var_qjungat_d_dn12_slot = var_qjungat_d_dn12;
        *var_qjungat_d_dn6_slot = var_qjungat_d_dn6;
        *var_qjungat_d_dn7_slot = var_qjungat_d_dn7;
        *var_qjungat_d_dn8_slot = var_qjungat_d_dn8;
        *var_qjungat_d_dn9_slot = var_qjungat_d_dn9;
        *var_qs_slot = var_qs;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_dn9_slot = var_qs_dn9;
        *var_sidexc_slot = var_sidexc;
        *var_sidexc_dn4_slot = var_sidexc_dn4;
        *var_sidexc_dn6_slot = var_sidexc_dn6;
        *var_sidexc_dn7_slot = var_sidexc_dn7;
        *var_sidexc_dn8_slot = var_sidexc_dn8;
        *var_sidexc_dn9_slot = var_sidexc_dn9;
        *var_sqid_slot = var_sqid;
        *var_sqid_dn4_slot = var_sqid_dn4;
        *var_sqid_dn6_slot = var_sqid_dn6;
        *var_sqid_dn7_slot = var_sqid_dn7;
        *var_sqid_dn8_slot = var_sqid_dn8;
        *var_sqid_dn9_slot = var_sqid_dn9;
        *var_sqig_slot = var_sqig;
        *var_sqig_dn4_slot = var_sqig_dn4;
        *var_sqig_dn6_slot = var_sqig_dn6;
        *var_sqig_dn7_slot = var_sqig_dn7;
        *var_sqig_dn8_slot = var_sqig_dn8;
        *var_sqig_dn9_slot = var_sqig_dn9;
        *var_sqt2_slot = var_sqt2;
        *var_sqt2_dn4_slot = var_sqt2_dn4;
        *var_sqt2_dn6_slot = var_sqt2_dn6;
        *var_sqt2_dn7_slot = var_sqt2_dn7;
        *var_sqt2_dn8_slot = var_sqt2_dn8;
        *var_sqt2_dn9_slot = var_sqt2_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_temp__blk1748_slot = var_temp__blk1748;
        *var_temp__blk1748_dn4_slot = var_temp__blk1748_dn4;
        *var_temp__blk1748_dn6_slot = var_temp__blk1748_dn6;
        *var_temp__blk1748_dn7_slot = var_temp__blk1748_dn7;
        *var_temp__blk1748_dn8_slot = var_temp__blk1748_dn8;
        *var_temp__blk1748_dn9_slot = var_temp__blk1748_dn9;
        *var_tmp__blk1560_slot = var_tmp__blk1560;
        *var_tmp__blk1560_dn11_slot = var_tmp__blk1560_dn11;
        *var_tmp__blk1560_dn12_slot = var_tmp__blk1560_dn12;
        *var_tmp__blk1560_dn6_slot = var_tmp__blk1560_dn6;
        *var_tmp__blk1560_dn7_slot = var_tmp__blk1560_dn7;
        *var_tmp__blk1560_dn8_slot = var_tmp__blk1560_dn8;
        *var_tmp__blk1560_dn9_slot = var_tmp__blk1560_dn9;
        *var_vjtmp_slot = var_vjtmp;
        *var_vjtmp_dn11_slot = var_vjtmp_dn11;
        *var_vjtmp_dn12_slot = var_vjtmp_dn12;
        *var_vjtmp_dn7_slot = var_vjtmp_dn7;
        *var_vjtmp_dn8_slot = var_vjtmp_dn8;
    }

    pub(super) fn stamp_transient_block_148(
        p: &Parameters,
        var_bet_i: f64,
        var_bet_i_dn4: f64,
        var_chnl_type: f64,
        var_cox_qm: f64,
        var_cox_qm_dn4: f64,
        var_cox_qm_dn6: f64,
        var_cox_qm_dn7: f64,
        var_cox_qm_dn8: f64,
        var_cox_qm_dn9: f64,
        var_dps_dc: f64,
        var_dps_dc_dn4: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_dps_dc_dn9: f64,
        var_eta_p_ac: f64,
        var_eta_p_ac_dn4: f64,
        var_eta_p_ac_dn6: f64,
        var_eta_p_ac_dn7: f64,
        var_eta_p_ac_dn8: f64,
        var_eta_p_ac_dn9: f64,
        var_fac_exc: f64,
        var_fntexc_i: f64,
        var_gmob_dc: f64,
        var_gmob_dc_dn4: f64,
        var_gmob_dc_dn6: f64,
        var_gmob_dc_dn7: f64,
        var_gmob_dc_dn8: f64,
        var_gmob_dc_dn9: f64,
        var_gmob_dl_ac: f64,
        var_gmob_dl_ac_dn4: f64,
        var_gmob_dl_ac_dn6: f64,
        var_gmob_dl_ac_dn7: f64,
        var_gmob_dl_ac_dn8: f64,
        var_gmob_dl_ac_dn9: f64,
        var_guard1782: f64,
        var_guard1784: f64,
        var_gvsat_ac: f64,
        var_gvsat_ac_dn4: f64,
        var_gvsat_ac_dn6: f64,
        var_gvsat_ac_dn7: f64,
        var_gvsat_ac_dn8: f64,
        var_gvsat_ac_dn9: f64,
        var_gvsatinv_dc: f64,
        var_gvsatinv_dc_dn4: f64,
        var_gvsatinv_dc_dn6: f64,
        var_gvsatinv_dc_dn7: f64,
        var_gvsatinv_dc_dn8: f64,
        var_gvsatinv_dc_dn9: f64,
        var_h0: f64,
        var_h0_dn4: f64,
        var_h0_dn6: f64,
        var_h0_dn7: f64,
        var_h0_dn8: f64,
        var_h0_dn9: f64,
        var_h_dc: f64,
        var_h_dc_dn4: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_h_dc_dn9: f64,
        var_i_ds: f64,
        var_i_ds_dn4: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_i_ds_dn9: f64,
        var_nt: f64,
        var_nt0: f64,
        var_nt0_dn4: f64,
        var_nt_dn4: f64,
        var_qim1_dc: f64,
        var_qim1_dc_dn4: f64,
        var_qim1_dc_dn6: f64,
        var_qim1_dc_dn7: f64,
        var_qim1_dc_dn8: f64,
        var_qim1_dc_dn9: f64,
        var_sqt2: f64,
        var_sqt2_dn4: f64,
        var_sqt2_dn6: f64,
        var_sqt2_dn7: f64,
        var_sqt2_dn8: f64,
        var_sqt2_dn9: f64,
        var_t1: f64,
        var_t1_dn4: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_thesateff_dc: f64,
        var_thesateff_dc_dn4: f64,
        var_thesateff_dc_dn6: f64,
        var_thesateff_dc_dn7: f64,
        var_thesateff_dc_dn8: f64,
        var_thesateff_dc_dn9: f64,
        var_vdse_dc: f64,
        var_vdse_dc_dn4: f64,
        var_vdse_dc_dn6: f64,
        var_vdse_dc_dn7: f64,
        var_vdse_dc_dn8: f64,
        var_vdse_dc_dn9: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_dn4_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_c_igid_dn9_slot: &mut f64,
        var_cgeff_slot: &mut f64,
        var_cgeff_dn4_slot: &mut f64,
        var_cgeff_dn6_slot: &mut f64,
        var_cgeff_dn7_slot: &mut f64,
        var_cgeff_dn8_slot: &mut f64,
        var_cgeff_dn9_slot: &mut f64,
        var_g_ideal_slot: &mut f64,
        var_g_ideal_dn4_slot: &mut f64,
        var_g_ideal_dn6_slot: &mut f64,
        var_g_ideal_dn7_slot: &mut f64,
        var_g_ideal_dn8_slot: &mut f64,
        var_g_ideal_dn9_slot: &mut f64,
        var_gfac_slot: &mut f64,
        var_gfac_dn4_slot: &mut f64,
        var_gfac_dn6_slot: &mut f64,
        var_gfac_dn7_slot: &mut f64,
        var_gfac_dn8_slot: &mut f64,
        var_gfac_dn9_slot: &mut f64,
        var_guard1785_slot: &mut f64,
        var_guard1786_slot: &mut f64,
        var_guard1787_slot: &mut f64,
        var_guard1788_slot: &mut f64,
        var_guard1789_slot: &mut f64,
        var_gvsat_exc_slot: &mut f64,
        var_gvsat_exc_dn4_slot: &mut f64,
        var_gvsat_exc_dn6_slot: &mut f64,
        var_gvsat_exc_dn7_slot: &mut f64,
        var_gvsat_exc_dn8_slot: &mut f64,
        var_gvsat_exc_dn9_slot: &mut f64,
        var_lc_slot: &mut f64,
        var_lc_dn4_slot: &mut f64,
        var_lc_dn6_slot: &mut f64,
        var_lc_dn7_slot: &mut f64,
        var_lc_dn8_slot: &mut f64,
        var_lc_dn9_slot: &mut f64,
        var_lcinv2_slot: &mut f64,
        var_lcinv2_dn4_slot: &mut f64,
        var_lcinv2_dn6_slot: &mut f64,
        var_lcinv2_dn7_slot: &mut f64,
        var_lcinv2_dn8_slot: &mut f64,
        var_lcinv2_dn9_slot: &mut f64,
        var_mid_slot: &mut f64,
        var_mid_dn4_slot: &mut f64,
        var_mid_dn6_slot: &mut f64,
        var_mid_dn7_slot: &mut f64,
        var_mid_dn8_slot: &mut f64,
        var_mid_dn9_slot: &mut f64,
        var_mig_slot: &mut f64,
        var_mig_dn4_slot: &mut f64,
        var_mig_dn6_slot: &mut f64,
        var_mig_dn7_slot: &mut f64,
        var_mig_dn8_slot: &mut f64,
        var_mig_dn9_slot: &mut f64,
        var_migid0_slot: &mut f64,
        var_migid0_dn4_slot: &mut f64,
        var_migid0_dn6_slot: &mut f64,
        var_migid0_dn7_slot: &mut f64,
        var_migid0_dn8_slot: &mut f64,
        var_migid0_dn9_slot: &mut f64,
        var_r_slot: &mut f64,
        var_r_dn4_slot: &mut f64,
        var_r_dn6_slot: &mut f64,
        var_r_dn7_slot: &mut f64,
        var_r_dn8_slot: &mut f64,
        var_r_dn9_slot: &mut f64,
        var_sidexc_slot: &mut f64,
        var_sidexc_dn4_slot: &mut f64,
        var_sidexc_dn6_slot: &mut f64,
        var_sidexc_dn7_slot: &mut f64,
        var_sidexc_dn8_slot: &mut f64,
        var_sidexc_dn9_slot: &mut f64,
        var_sqid_slot: &mut f64,
        var_sqid_dn4_slot: &mut f64,
        var_sqid_dn6_slot: &mut f64,
        var_sqid_dn7_slot: &mut f64,
        var_sqid_dn8_slot: &mut f64,
        var_sqid_dn9_slot: &mut f64,
        var_sqig_slot: &mut f64,
        var_sqig_dn4_slot: &mut f64,
        var_sqig_dn6_slot: &mut f64,
        var_sqig_dn7_slot: &mut f64,
        var_sqig_dn8_slot: &mut f64,
        var_sqig_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_thesat1_exc_slot: &mut f64,
        var_thesat1_exc_dn4_slot: &mut f64,
        var_thesat1_exc_dn6_slot: &mut f64,
        var_thesat1_exc_dn7_slot: &mut f64,
        var_thesat1_exc_dn8_slot: &mut f64,
        var_thesat1_exc_dn9_slot: &mut f64,
        var_zsat_exc_slot: &mut f64,
        var_zsat_exc_dn4_slot: &mut f64,
        var_zsat_exc_dn6_slot: &mut f64,
        var_zsat_exc_dn7_slot: &mut f64,
        var_zsat_exc_dn8_slot: &mut f64,
        var_zsat_exc_dn9_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_dn4: f64 = *var_c_igid_dn4_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_c_igid_dn9: f64 = *var_c_igid_dn9_slot;
        let mut var_cgeff: f64 = *var_cgeff_slot;
        let mut var_cgeff_dn4: f64 = *var_cgeff_dn4_slot;
        let mut var_cgeff_dn6: f64 = *var_cgeff_dn6_slot;
        let mut var_cgeff_dn7: f64 = *var_cgeff_dn7_slot;
        let mut var_cgeff_dn8: f64 = *var_cgeff_dn8_slot;
        let mut var_cgeff_dn9: f64 = *var_cgeff_dn9_slot;
        let mut var_g_ideal: f64 = *var_g_ideal_slot;
        let mut var_g_ideal_dn4: f64 = *var_g_ideal_dn4_slot;
        let mut var_g_ideal_dn6: f64 = *var_g_ideal_dn6_slot;
        let mut var_g_ideal_dn7: f64 = *var_g_ideal_dn7_slot;
        let mut var_g_ideal_dn8: f64 = *var_g_ideal_dn8_slot;
        let mut var_g_ideal_dn9: f64 = *var_g_ideal_dn9_slot;
        let mut var_gfac: f64 = *var_gfac_slot;
        let mut var_gfac_dn4: f64 = *var_gfac_dn4_slot;
        let mut var_gfac_dn6: f64 = *var_gfac_dn6_slot;
        let mut var_gfac_dn7: f64 = *var_gfac_dn7_slot;
        let mut var_gfac_dn8: f64 = *var_gfac_dn8_slot;
        let mut var_gfac_dn9: f64 = *var_gfac_dn9_slot;
        let mut var_guard1785: f64 = *var_guard1785_slot;
        let mut var_guard1786: f64 = *var_guard1786_slot;
        let mut var_guard1787: f64 = *var_guard1787_slot;
        let mut var_guard1788: f64 = *var_guard1788_slot;
        let mut var_guard1789: f64 = *var_guard1789_slot;
        let mut var_gvsat_exc: f64 = *var_gvsat_exc_slot;
        let mut var_gvsat_exc_dn4: f64 = *var_gvsat_exc_dn4_slot;
        let mut var_gvsat_exc_dn6: f64 = *var_gvsat_exc_dn6_slot;
        let mut var_gvsat_exc_dn7: f64 = *var_gvsat_exc_dn7_slot;
        let mut var_gvsat_exc_dn8: f64 = *var_gvsat_exc_dn8_slot;
        let mut var_gvsat_exc_dn9: f64 = *var_gvsat_exc_dn9_slot;
        let mut var_lc: f64 = *var_lc_slot;
        let mut var_lc_dn4: f64 = *var_lc_dn4_slot;
        let mut var_lc_dn6: f64 = *var_lc_dn6_slot;
        let mut var_lc_dn7: f64 = *var_lc_dn7_slot;
        let mut var_lc_dn8: f64 = *var_lc_dn8_slot;
        let mut var_lc_dn9: f64 = *var_lc_dn9_slot;
        let mut var_lcinv2: f64 = *var_lcinv2_slot;
        let mut var_lcinv2_dn4: f64 = *var_lcinv2_dn4_slot;
        let mut var_lcinv2_dn6: f64 = *var_lcinv2_dn6_slot;
        let mut var_lcinv2_dn7: f64 = *var_lcinv2_dn7_slot;
        let mut var_lcinv2_dn8: f64 = *var_lcinv2_dn8_slot;
        let mut var_lcinv2_dn9: f64 = *var_lcinv2_dn9_slot;
        let mut var_mid: f64 = *var_mid_slot;
        let mut var_mid_dn4: f64 = *var_mid_dn4_slot;
        let mut var_mid_dn6: f64 = *var_mid_dn6_slot;
        let mut var_mid_dn7: f64 = *var_mid_dn7_slot;
        let mut var_mid_dn8: f64 = *var_mid_dn8_slot;
        let mut var_mid_dn9: f64 = *var_mid_dn9_slot;
        let mut var_mig: f64 = *var_mig_slot;
        let mut var_mig_dn4: f64 = *var_mig_dn4_slot;
        let mut var_mig_dn6: f64 = *var_mig_dn6_slot;
        let mut var_mig_dn7: f64 = *var_mig_dn7_slot;
        let mut var_mig_dn8: f64 = *var_mig_dn8_slot;
        let mut var_mig_dn9: f64 = *var_mig_dn9_slot;
        let mut var_migid0: f64 = *var_migid0_slot;
        let mut var_migid0_dn4: f64 = *var_migid0_dn4_slot;
        let mut var_migid0_dn6: f64 = *var_migid0_dn6_slot;
        let mut var_migid0_dn7: f64 = *var_migid0_dn7_slot;
        let mut var_migid0_dn8: f64 = *var_migid0_dn8_slot;
        let mut var_migid0_dn9: f64 = *var_migid0_dn9_slot;
        let mut var_r: f64 = *var_r_slot;
        let mut var_r_dn4: f64 = *var_r_dn4_slot;
        let mut var_r_dn6: f64 = *var_r_dn6_slot;
        let mut var_r_dn7: f64 = *var_r_dn7_slot;
        let mut var_r_dn8: f64 = *var_r_dn8_slot;
        let mut var_r_dn9: f64 = *var_r_dn9_slot;
        let mut var_sidexc: f64 = *var_sidexc_slot;
        let mut var_sidexc_dn4: f64 = *var_sidexc_dn4_slot;
        let mut var_sidexc_dn6: f64 = *var_sidexc_dn6_slot;
        let mut var_sidexc_dn7: f64 = *var_sidexc_dn7_slot;
        let mut var_sidexc_dn8: f64 = *var_sidexc_dn8_slot;
        let mut var_sidexc_dn9: f64 = *var_sidexc_dn9_slot;
        let mut var_sqid: f64 = *var_sqid_slot;
        let mut var_sqid_dn4: f64 = *var_sqid_dn4_slot;
        let mut var_sqid_dn6: f64 = *var_sqid_dn6_slot;
        let mut var_sqid_dn7: f64 = *var_sqid_dn7_slot;
        let mut var_sqid_dn8: f64 = *var_sqid_dn8_slot;
        let mut var_sqid_dn9: f64 = *var_sqid_dn9_slot;
        let mut var_sqig: f64 = *var_sqig_slot;
        let mut var_sqig_dn4: f64 = *var_sqig_dn4_slot;
        let mut var_sqig_dn6: f64 = *var_sqig_dn6_slot;
        let mut var_sqig_dn7: f64 = *var_sqig_dn7_slot;
        let mut var_sqig_dn8: f64 = *var_sqig_dn8_slot;
        let mut var_sqig_dn9: f64 = *var_sqig_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_thesat1_exc: f64 = *var_thesat1_exc_slot;
        let mut var_thesat1_exc_dn4: f64 = *var_thesat1_exc_dn4_slot;
        let mut var_thesat1_exc_dn6: f64 = *var_thesat1_exc_dn6_slot;
        let mut var_thesat1_exc_dn7: f64 = *var_thesat1_exc_dn7_slot;
        let mut var_thesat1_exc_dn8: f64 = *var_thesat1_exc_dn8_slot;
        let mut var_thesat1_exc_dn9: f64 = *var_thesat1_exc_dn9_slot;
        let mut var_zsat_exc: f64 = *var_zsat_exc_slot;
        let mut var_zsat_exc_dn4: f64 = *var_zsat_exc_dn4_slot;
        let mut var_zsat_exc_dn6: f64 = *var_zsat_exc_dn6_slot;
        let mut var_zsat_exc_dn7: f64 = *var_zsat_exc_dn7_slot;
        let mut var_zsat_exc_dn8: f64 = *var_zsat_exc_dn8_slot;
        let mut var_zsat_exc_dn9: f64 = *var_zsat_exc_dn9_slot;

        let (assign62580_e81014, assign62580_e81014_d_n4, assign62580_e81014_d_n6, assign62580_e81014_d_n7, assign62580_e81014_d_n8, assign62580_e81014_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62580_e81012: f64 = (var_sqt2 * var_sqt2);
        (assign62580_e81012, ((var_sqt2_dn4 * var_sqt2) + (var_sqt2 * var_sqt2_dn4)), ((var_sqt2_dn6 * var_sqt2) + (var_sqt2 * var_sqt2_dn6)), ((var_sqt2_dn7 * var_sqt2) + (var_sqt2 * var_sqt2_dn7)), ((var_sqt2_dn8 * var_sqt2) + (var_sqt2 * var_sqt2_dn8)), ((var_sqt2_dn9 * var_sqt2) + (var_sqt2 * var_sqt2_dn9)),)
    } else {
        (var_t2, var_t2_dn4, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9,)
    }
};
        var_t2 = assign62580_e81014;
        var_t2_dn4 = assign62580_e81014_d_n4;
        var_t2_dn6 = assign62580_e81014_d_n6;
        var_t2_dn7 = assign62580_e81014_d_n7;
        var_t2_dn8 = assign62580_e81014_d_n8;
        var_t2_dn9 = assign62580_e81014_d_n9;

        let (assign62590_e81024, assign62590_e81024_d_n4, assign62590_e81024_d_n6, assign62590_e81024_d_n7, assign62590_e81024_d_n8, assign62590_e81024_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62590_e81020: f64 = (var_h0 / var_h_dc);
        let assign62590_e81022: f64 = (assign62590_e81020 - 1.0);
        (assign62590_e81022, (((var_h0_dn4 * var_h_dc) - (var_h0 * var_h_dc_dn4)) / (var_h_dc * var_h_dc)), (((var_h0_dn6 * var_h_dc) - (var_h0 * var_h_dc_dn6)) / (var_h_dc * var_h_dc)), (((var_h0_dn7 * var_h_dc) - (var_h0 * var_h_dc_dn7)) / (var_h_dc * var_h_dc)), (((var_h0_dn8 * var_h_dc) - (var_h0 * var_h_dc_dn8)) / (var_h_dc * var_h_dc)), (((var_h0_dn9 * var_h_dc) - (var_h0 * var_h_dc_dn9)) / (var_h_dc * var_h_dc)),)
    } else {
        (var_r, var_r_dn4, var_r_dn6, var_r_dn7, var_r_dn8, var_r_dn9,)
    }
};
        var_r = assign62590_e81024;
        var_r_dn4 = assign62590_e81024_d_n4;
        var_r_dn6 = assign62590_e81024_d_n6;
        var_r_dn7 = assign62590_e81024_d_n7;
        var_r_dn8 = assign62590_e81024_d_n8;
        var_r_dn9 = assign62590_e81024_d_n9;

        let (assign62600_e81047, assign62600_e81047_d_n4, assign62600_e81047_d_n6, assign62600_e81047_d_n7, assign62600_e81047_d_n8, assign62600_e81047_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62600_e81032: f64 = (var_r * var_t2);
        let assign62600_e81033: f64 = (12.0 * assign62600_e81032);
        let assign62600_e81034: f64 = (1.0 - assign62600_e81033);
        let (assign62600_e81045, assign62600_e81045_d_n4, assign62600_e81045_d_n6, assign62600_e81045_d_n7, assign62600_e81045_d_n8, assign62600_e81045_d_n9,) = {
            if (assign62600_e81034 > 1e-20) {
                let assign62600_e81041: f64 = (var_r * var_t2);
                let assign62600_e81042: f64 = (12.0 * assign62600_e81041);
                let assign62600_e81043: f64 = (1.0 - assign62600_e81042);
                (assign62600_e81043, (-(12.0 * ((var_r_dn4 * var_t2) + (var_r * var_t2_dn4)))), (-(12.0 * ((var_r_dn6 * var_t2) + (var_r * var_t2_dn6)))), (-(12.0 * ((var_r_dn7 * var_t2) + (var_r * var_t2_dn7)))), (-(12.0 * ((var_r_dn8 * var_t2) + (var_r * var_t2_dn8)))), (-(12.0 * ((var_r_dn9 * var_t2) + (var_r * var_t2_dn9)))),)
            } else {
                (1e-20, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62600_e81045, assign62600_e81045_d_n4, assign62600_e81045_d_n6, assign62600_e81045_d_n7, assign62600_e81045_d_n8, assign62600_e81045_d_n9,)
    } else {
        (var_lc, var_lc_dn4, var_lc_dn6, var_lc_dn7, var_lc_dn8, var_lc_dn9,)
    }
};
        var_lc = assign62600_e81047;
        var_lc_dn4 = assign62600_e81047_d_n4;
        var_lc_dn6 = assign62600_e81047_d_n6;
        var_lc_dn7 = assign62600_e81047_d_n7;
        var_lc_dn8 = assign62600_e81047_d_n8;
        var_lc_dn9 = assign62600_e81047_d_n9;

        let (assign62610_e81057, assign62610_e81057_d_n4, assign62610_e81057_d_n6, assign62610_e81057_d_n7, assign62610_e81057_d_n8, assign62610_e81057_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62610_e81054: f64 = (var_lc * var_lc);
        let assign62610_e81055: f64 = (1.0 / assign62610_e81054);
        (assign62610_e81055, (-(((var_lc_dn4 * var_lc) + (var_lc * var_lc_dn4)) / (assign62610_e81054 * assign62610_e81054))), (-(((var_lc_dn6 * var_lc) + (var_lc * var_lc_dn6)) / (assign62610_e81054 * assign62610_e81054))), (-(((var_lc_dn7 * var_lc) + (var_lc * var_lc_dn7)) / (assign62610_e81054 * assign62610_e81054))), (-(((var_lc_dn8 * var_lc) + (var_lc * var_lc_dn8)) / (assign62610_e81054 * assign62610_e81054))), (-(((var_lc_dn9 * var_lc) + (var_lc * var_lc_dn9)) / (assign62610_e81054 * assign62610_e81054))),)
    } else {
        (var_lcinv2, var_lcinv2_dn4, var_lcinv2_dn6, var_lcinv2_dn7, var_lcinv2_dn8, var_lcinv2_dn9,)
    }
};
        var_lcinv2 = assign62610_e81057;
        var_lcinv2_dn4 = assign62610_e81057_d_n4;
        var_lcinv2_dn6 = assign62610_e81057_d_n6;
        var_lcinv2_dn7 = assign62610_e81057_d_n7;
        var_lcinv2_dn8 = assign62610_e81057_d_n8;
        var_lcinv2_dn9 = assign62610_e81057_d_n9;

        let (assign62620_e81067, assign62620_e81067_d_n4, assign62620_e81067_d_n6, assign62620_e81067_d_n7, assign62620_e81067_d_n8, assign62620_e81067_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62620_e81063: f64 = (var_bet_i * var_qim1_dc);
        let assign62620_e81065: f64 = (assign62620_e81063 * var_gvsatinv_dc);
        (assign62620_e81065, ((((var_bet_i_dn4 * var_qim1_dc) + (var_bet_i * var_qim1_dc_dn4)) * var_gvsatinv_dc) + (assign62620_e81063 * var_gvsatinv_dc_dn4)), (((var_bet_i * var_qim1_dc_dn6) * var_gvsatinv_dc) + (assign62620_e81063 * var_gvsatinv_dc_dn6)), (((var_bet_i * var_qim1_dc_dn7) * var_gvsatinv_dc) + (assign62620_e81063 * var_gvsatinv_dc_dn7)), (((var_bet_i * var_qim1_dc_dn8) * var_gvsatinv_dc) + (assign62620_e81063 * var_gvsatinv_dc_dn8)), (((var_bet_i * var_qim1_dc_dn9) * var_gvsatinv_dc) + (assign62620_e81063 * var_gvsatinv_dc_dn9)),)
    } else {
        (var_g_ideal, var_g_ideal_dn4, var_g_ideal_dn6, var_g_ideal_dn7, var_g_ideal_dn8, var_g_ideal_dn9,)
    }
};
        var_g_ideal = assign62620_e81067;
        var_g_ideal_dn4 = assign62620_e81067_d_n4;
        var_g_ideal_dn6 = assign62620_e81067_d_n6;
        var_g_ideal_dn7 = assign62620_e81067_d_n7;
        var_g_ideal_dn8 = assign62620_e81067_d_n8;
        var_g_ideal_dn9 = assign62620_e81067_d_n9;

        let (assign62630_e81087, assign62630_e81087_d_n4, assign62630_e81087_d_n6, assign62630_e81087_d_n7, assign62630_e81087_d_n8, assign62630_e81087_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62630_e81074: f64 = (12.0 * var_t2);
        let assign62630_e81075: f64 = (var_t1 + assign62630_e81074);
        let assign62630_e81079: f64 = (1.0 + var_t1);
        let assign62630_e81081: f64 = (assign62630_e81079 * var_t2);
        let assign62630_e81083: f64 = (assign62630_e81081 * var_r);
        let assign62630_e81084: f64 = (24.0 * assign62630_e81083);
        let assign62630_e81085: f64 = (assign62630_e81075 - assign62630_e81084);
        (assign62630_e81085, ((var_t1_dn4 + (12.0 * var_t2_dn4)) - (24.0 * ((((var_t1_dn4 * var_t2) + (assign62630_e81079 * var_t2_dn4)) * var_r) + (assign62630_e81081 * var_r_dn4)))), ((var_t1_dn6 + (12.0 * var_t2_dn6)) - (24.0 * ((((var_t1_dn6 * var_t2) + (assign62630_e81079 * var_t2_dn6)) * var_r) + (assign62630_e81081 * var_r_dn6)))), ((var_t1_dn7 + (12.0 * var_t2_dn7)) - (24.0 * ((((var_t1_dn7 * var_t2) + (assign62630_e81079 * var_t2_dn7)) * var_r) + (assign62630_e81081 * var_r_dn7)))), ((var_t1_dn8 + (12.0 * var_t2_dn8)) - (24.0 * ((((var_t1_dn8 * var_t2) + (assign62630_e81079 * var_t2_dn8)) * var_r) + (assign62630_e81081 * var_r_dn8)))), ((var_t1_dn9 + (12.0 * var_t2_dn9)) - (24.0 * ((((var_t1_dn9 * var_t2) + (assign62630_e81079 * var_t2_dn9)) * var_r) + (assign62630_e81081 * var_r_dn9)))),)
    } else {
        (var_mid, var_mid_dn4, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9,)
    }
};
        var_mid = assign62630_e81087;
        var_mid_dn4 = assign62630_e81087_d_n4;
        var_mid_dn6 = assign62630_e81087_d_n6;
        var_mid_dn7 = assign62630_e81087_d_n7;
        var_mid_dn8 = assign62630_e81087_d_n8;
        var_mid_dn9 = assign62630_e81087_d_n9;

        let (assign62640_e81098, assign62640_e81098_d_n4, assign62640_e81098_d_n6, assign62640_e81098_d_n7, assign62640_e81098_d_n8, assign62640_e81098_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let (assign62640_e81096, assign62640_e81096_d_n4, assign62640_e81096_d_n6, assign62640_e81096_d_n7, assign62640_e81096_d_n8, assign62640_e81096_d_n9,) = {
            if (var_mid > 1e-40) {
                (var_mid, var_mid_dn4, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62640_e81096, assign62640_e81096_d_n4, assign62640_e81096_d_n6, assign62640_e81096_d_n7, assign62640_e81096_d_n8, assign62640_e81096_d_n9,)
    } else {
        (var_mid, var_mid_dn4, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9,)
    }
};
        var_mid = assign62640_e81098;
        var_mid_dn4 = assign62640_e81098_d_n4;
        var_mid_dn6 = assign62640_e81098_d_n6;
        var_mid_dn7 = assign62640_e81098_d_n7;
        var_mid_dn8 = assign62640_e81098_d_n8;
        var_mid_dn9 = assign62640_e81098_d_n9;

        let (assign62650_e81108, assign62650_e81108_d_n4, assign62650_e81108_d_n6, assign62650_e81108_d_n7, assign62650_e81108_d_n8, assign62650_e81108_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62650_e81104: f64 = (var_g_ideal * var_lcinv2);
        let assign62650_e81106: f64 = (assign62650_e81104 * var_mid);
        (assign62650_e81106, ((((var_g_ideal_dn4 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn4)) * var_mid) + (assign62650_e81104 * var_mid_dn4)), ((((var_g_ideal_dn6 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn6)) * var_mid) + (assign62650_e81104 * var_mid_dn6)), ((((var_g_ideal_dn7 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn7)) * var_mid) + (assign62650_e81104 * var_mid_dn7)), ((((var_g_ideal_dn8 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn8)) * var_mid) + (assign62650_e81104 * var_mid_dn8)), ((((var_g_ideal_dn9 * var_lcinv2) + (var_g_ideal * var_lcinv2_dn9)) * var_mid) + (assign62650_e81104 * var_mid_dn9)),)
    } else {
        (var_mid, var_mid_dn4, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9,)
    }
};
        var_mid = assign62650_e81108;
        var_mid_dn4 = assign62650_e81108_d_n4;
        var_mid_dn6 = assign62650_e81108_d_n6;
        var_mid_dn7 = assign62650_e81108_d_n7;
        var_mid_dn8 = assign62650_e81108_d_n8;
        var_mid_dn9 = assign62650_e81108_d_n9;

        let assign62660_e81111: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1785 = assign62660_e81111;

        let (assign62670_e81121, assign62670_e81121_d_n4, assign62670_e81121_d_n6, assign62670_e81121_d_n7, assign62670_e81121_d_n8, assign62670_e81121_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) && (var_guard1785 != 0.0)) {
        let assign62670_e81119: f64 = (var_thesateff_dc / var_gmob_dc);
        (assign62670_e81119, (((var_thesateff_dc_dn4 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn4)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn6 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn6)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn7 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn7)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn8 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn8)) / (var_gmob_dc * var_gmob_dc)), (((var_thesateff_dc_dn9 * var_gmob_dc) - (var_thesateff_dc * var_gmob_dc_dn9)) / (var_gmob_dc * var_gmob_dc)),)
    } else {
        (var_thesat1_exc, var_thesat1_exc_dn4, var_thesat1_exc_dn6, var_thesat1_exc_dn7, var_thesat1_exc_dn8, var_thesat1_exc_dn9,)
    }
};
        var_thesat1_exc = assign62670_e81121;
        var_thesat1_exc_dn4 = assign62670_e81121_d_n4;
        var_thesat1_exc_dn6 = assign62670_e81121_d_n6;
        var_thesat1_exc_dn7 = assign62670_e81121_d_n7;
        var_thesat1_exc_dn8 = assign62670_e81121_d_n8;
        var_thesat1_exc_dn9 = assign62670_e81121_d_n9;

        let (assign62680_e81135, assign62680_e81135_d_n4, assign62680_e81135_d_n6, assign62680_e81135_d_n7, assign62680_e81135_d_n8, assign62680_e81135_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) && (var_guard1785 != 0.0)) {
        let assign62680_e81129: f64 = (var_thesat1_exc * var_thesat1_exc);
        let assign62680_e81131: f64 = (assign62680_e81129 * var_dps_dc);
        let assign62680_e81133: f64 = (assign62680_e81131 * var_dps_dc);
        (assign62680_e81133, ((((((var_thesat1_exc_dn4 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn4)) * var_dps_dc) + (assign62680_e81129 * var_dps_dc_dn4)) * var_dps_dc) + (assign62680_e81131 * var_dps_dc_dn4)), ((((((var_thesat1_exc_dn6 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn6)) * var_dps_dc) + (assign62680_e81129 * var_dps_dc_dn6)) * var_dps_dc) + (assign62680_e81131 * var_dps_dc_dn6)), ((((((var_thesat1_exc_dn7 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn7)) * var_dps_dc) + (assign62680_e81129 * var_dps_dc_dn7)) * var_dps_dc) + (assign62680_e81131 * var_dps_dc_dn7)), ((((((var_thesat1_exc_dn8 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn8)) * var_dps_dc) + (assign62680_e81129 * var_dps_dc_dn8)) * var_dps_dc) + (assign62680_e81131 * var_dps_dc_dn8)), ((((((var_thesat1_exc_dn9 * var_thesat1_exc) + (var_thesat1_exc * var_thesat1_exc_dn9)) * var_dps_dc) + (assign62680_e81129 * var_dps_dc_dn9)) * var_dps_dc) + (assign62680_e81131 * var_dps_dc_dn9)),)
    } else {
        (var_zsat_exc, var_zsat_exc_dn4, var_zsat_exc_dn6, var_zsat_exc_dn7, var_zsat_exc_dn8, var_zsat_exc_dn9,)
    }
};
        var_zsat_exc = assign62680_e81135;
        var_zsat_exc_dn4 = assign62680_e81135_d_n4;
        var_zsat_exc_dn6 = assign62680_e81135_d_n6;
        var_zsat_exc_dn7 = assign62680_e81135_d_n7;
        var_zsat_exc_dn8 = assign62680_e81135_d_n8;
        var_zsat_exc_dn9 = assign62680_e81135_d_n9;

        let assign62690_e81138: f64 = (-1.0);
        let assign62690_e81139: f64 = if var_chnl_type == assign62690_e81138 { 1.0 } else { 0.0 };
        var_guard1786 = assign62690_e81139;

        let (assign62700_e81155, assign62700_e81155_d_n4, assign62700_e81155_d_n6, assign62700_e81155_d_n7, assign62700_e81155_d_n8, assign62700_e81155_d_n9,) = {
    if ((((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) && (var_guard1785 != 0.0)) && (var_guard1786 != 0.0)) {
        let assign62700_e81151: f64 = (var_thesat1_exc * var_dps_dc);
        let assign62700_e81152: f64 = (1.0 + assign62700_e81151);
        let assign62700_e81153: f64 = (var_zsat_exc / assign62700_e81152);
        (assign62700_e81153, (((var_zsat_exc_dn4 * assign62700_e81152) - (var_zsat_exc * ((var_thesat1_exc_dn4 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn4)))) / (assign62700_e81152 * assign62700_e81152)), (((var_zsat_exc_dn6 * assign62700_e81152) - (var_zsat_exc * ((var_thesat1_exc_dn6 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn6)))) / (assign62700_e81152 * assign62700_e81152)), (((var_zsat_exc_dn7 * assign62700_e81152) - (var_zsat_exc * ((var_thesat1_exc_dn7 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn7)))) / (assign62700_e81152 * assign62700_e81152)), (((var_zsat_exc_dn8 * assign62700_e81152) - (var_zsat_exc * ((var_thesat1_exc_dn8 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn8)))) / (assign62700_e81152 * assign62700_e81152)), (((var_zsat_exc_dn9 * assign62700_e81152) - (var_zsat_exc * ((var_thesat1_exc_dn9 * var_dps_dc) + (var_thesat1_exc * var_dps_dc_dn9)))) / (assign62700_e81152 * assign62700_e81152)),)
    } else {
        (var_zsat_exc, var_zsat_exc_dn4, var_zsat_exc_dn6, var_zsat_exc_dn7, var_zsat_exc_dn8, var_zsat_exc_dn9,)
    }
};
        var_zsat_exc = assign62700_e81155;
        var_zsat_exc_dn4 = assign62700_e81155_d_n4;
        var_zsat_exc_dn6 = assign62700_e81155_d_n6;
        var_zsat_exc_dn7 = assign62700_e81155_d_n7;
        var_zsat_exc_dn8 = assign62700_e81155_d_n8;
        var_zsat_exc_dn9 = assign62700_e81155_d_n9;

        let (assign62710_e81174, assign62710_e81174_d_n4, assign62710_e81174_d_n6, assign62710_e81174_d_n7, assign62710_e81174_d_n8, assign62710_e81174_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) && (var_guard1785 != 0.0)) {
        let assign62710_e81167: f64 = (2.0 * var_zsat_exc);
        let assign62710_e81168: f64 = (1.0 + assign62710_e81167);
        let assign62710_e81169: f64 = (assign62710_e81168).sqrt();
        let assign62710_e81170: f64 = (1.0 + assign62710_e81169);
        let assign62710_e81171: f64 = (var_gmob_dc * assign62710_e81170);
        let assign62710_e81172: f64 = (0.5 * assign62710_e81171);
        (assign62710_e81172, (0.5 * ((var_gmob_dc_dn4 * assign62710_e81170) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn4) / (2.0 * assign62710_e81169))))), (0.5 * ((var_gmob_dc_dn6 * assign62710_e81170) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn6) / (2.0 * assign62710_e81169))))), (0.5 * ((var_gmob_dc_dn7 * assign62710_e81170) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn7) / (2.0 * assign62710_e81169))))), (0.5 * ((var_gmob_dc_dn8 * assign62710_e81170) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn8) / (2.0 * assign62710_e81169))))), (0.5 * ((var_gmob_dc_dn9 * assign62710_e81170) + (var_gmob_dc * ((2.0 * var_zsat_exc_dn9) / (2.0 * assign62710_e81169))))),)
    } else {
        (var_gvsat_exc, var_gvsat_exc_dn4, var_gvsat_exc_dn6, var_gvsat_exc_dn7, var_gvsat_exc_dn8, var_gvsat_exc_dn9,)
    }
};
        var_gvsat_exc = assign62710_e81174;
        var_gvsat_exc_dn4 = assign62710_e81174_d_n4;
        var_gvsat_exc_dn6 = assign62710_e81174_d_n6;
        var_gvsat_exc_dn7 = assign62710_e81174_d_n7;
        var_gvsat_exc_dn8 = assign62710_e81174_d_n8;
        var_gvsat_exc_dn9 = assign62710_e81174_d_n9;

        let (assign62720_e81186, assign62720_e81186_d_n4, assign62720_e81186_d_n6, assign62720_e81186_d_n7, assign62720_e81186_d_n8, assign62720_e81186_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) && (var_guard1785 != 0.0)) {
        let assign62720_e81183: f64 = (var_gvsat_exc * var_lc);
        let assign62720_e81184: f64 = (var_gmob_dc / assign62720_e81183);
        (assign62720_e81184, (((var_gmob_dc_dn4 * assign62720_e81183) - (var_gmob_dc * ((var_gvsat_exc_dn4 * var_lc) + (var_gvsat_exc * var_lc_dn4)))) / (assign62720_e81183 * assign62720_e81183)), (((var_gmob_dc_dn6 * assign62720_e81183) - (var_gmob_dc * ((var_gvsat_exc_dn6 * var_lc) + (var_gvsat_exc * var_lc_dn6)))) / (assign62720_e81183 * assign62720_e81183)), (((var_gmob_dc_dn7 * assign62720_e81183) - (var_gmob_dc * ((var_gvsat_exc_dn7 * var_lc) + (var_gvsat_exc * var_lc_dn7)))) / (assign62720_e81183 * assign62720_e81183)), (((var_gmob_dc_dn8 * assign62720_e81183) - (var_gmob_dc * ((var_gvsat_exc_dn8 * var_lc) + (var_gvsat_exc * var_lc_dn8)))) / (assign62720_e81183 * assign62720_e81183)), (((var_gmob_dc_dn9 * assign62720_e81183) - (var_gmob_dc * ((var_gvsat_exc_dn9 * var_lc) + (var_gvsat_exc * var_lc_dn9)))) / (assign62720_e81183 * assign62720_e81183)),)
    } else {
        (var_gfac, var_gfac_dn4, var_gfac_dn6, var_gfac_dn7, var_gfac_dn8, var_gfac_dn9,)
    }
};
        var_gfac = assign62720_e81186;
        var_gfac_dn4 = assign62720_e81186_d_n4;
        var_gfac_dn6 = assign62720_e81186_d_n6;
        var_gfac_dn7 = assign62720_e81186_d_n7;
        var_gfac_dn8 = assign62720_e81186_d_n8;
        var_gfac_dn9 = assign62720_e81186_d_n9;

        let (assign62730_e81202, assign62730_e81202_d_n4, assign62730_e81202_d_n6, assign62730_e81202_d_n7, assign62730_e81202_d_n8, assign62730_e81202_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) && (var_guard1785 != 0.0)) {
        let assign62730_e81194: f64 = (var_fac_exc * var_i_ds);
        let assign62730_e81196: f64 = (assign62730_e81194 * var_vdse_dc);
        let assign62730_e81198: f64 = (assign62730_e81196 * var_gfac);
        let assign62730_e81200: f64 = (assign62730_e81198 * var_gfac);
        (assign62730_e81200, (((((((var_fac_exc * var_i_ds_dn4) * var_vdse_dc) + (assign62730_e81194 * var_vdse_dc_dn4)) * var_gfac) + (assign62730_e81196 * var_gfac_dn4)) * var_gfac) + (assign62730_e81198 * var_gfac_dn4)), (((((((var_fac_exc * var_i_ds_dn6) * var_vdse_dc) + (assign62730_e81194 * var_vdse_dc_dn6)) * var_gfac) + (assign62730_e81196 * var_gfac_dn6)) * var_gfac) + (assign62730_e81198 * var_gfac_dn6)), (((((((var_fac_exc * var_i_ds_dn7) * var_vdse_dc) + (assign62730_e81194 * var_vdse_dc_dn7)) * var_gfac) + (assign62730_e81196 * var_gfac_dn7)) * var_gfac) + (assign62730_e81198 * var_gfac_dn7)), (((((((var_fac_exc * var_i_ds_dn8) * var_vdse_dc) + (assign62730_e81194 * var_vdse_dc_dn8)) * var_gfac) + (assign62730_e81196 * var_gfac_dn8)) * var_gfac) + (assign62730_e81198 * var_gfac_dn8)), (((((((var_fac_exc * var_i_ds_dn9) * var_vdse_dc) + (assign62730_e81194 * var_vdse_dc_dn9)) * var_gfac) + (assign62730_e81196 * var_gfac_dn9)) * var_gfac) + (assign62730_e81198 * var_gfac_dn9)),)
    } else {
        (var_sidexc, var_sidexc_dn4, var_sidexc_dn6, var_sidexc_dn7, var_sidexc_dn8, var_sidexc_dn9,)
    }
};
        var_sidexc = assign62730_e81202;
        var_sidexc_dn4 = assign62730_e81202_d_n4;
        var_sidexc_dn6 = assign62730_e81202_d_n6;
        var_sidexc_dn7 = assign62730_e81202_d_n7;
        var_sidexc_dn8 = assign62730_e81202_d_n8;
        var_sidexc_dn9 = assign62730_e81202_d_n9;

        let (assign62740_e81214, assign62740_e81214_d_n4, assign62740_e81214_d_n6, assign62740_e81214_d_n7, assign62740_e81214_d_n8, assign62740_e81214_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) && (var_guard1785 != 0.0)) {
        let assign62740_e81211: f64 = (var_sidexc / var_nt0);
        let assign62740_e81212: f64 = (var_mid + assign62740_e81211);
        (assign62740_e81212, (var_mid_dn4 + (((var_sidexc_dn4 * var_nt0) - (var_sidexc * var_nt0_dn4)) / (var_nt0 * var_nt0))), (var_mid_dn6 + (var_sidexc_dn6 / var_nt0)), (var_mid_dn7 + (var_sidexc_dn7 / var_nt0)), (var_mid_dn8 + (var_sidexc_dn8 / var_nt0)), (var_mid_dn9 + (var_sidexc_dn9 / var_nt0)),)
    } else {
        (var_mid, var_mid_dn4, var_mid_dn6, var_mid_dn7, var_mid_dn8, var_mid_dn9,)
    }
};
        var_mid = assign62740_e81214;
        var_mid_dn4 = assign62740_e81214_d_n4;
        var_mid_dn6 = assign62740_e81214_d_n6;
        var_mid_dn7 = assign62740_e81214_d_n7;
        var_mid_dn8 = assign62740_e81214_d_n8;
        var_mid_dn9 = assign62740_e81214_d_n9;

        let (assign62750_e81223, assign62750_e81223_d_n4, assign62750_e81223_d_n6, assign62750_e81223_d_n7, assign62750_e81223_d_n8, assign62750_e81223_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1784 != 0.0)) {
        let assign62750_e81220: f64 = (var_nt * var_mid);
        let assign62750_e81221: f64 = (assign62750_e81220).sqrt();
        (assign62750_e81221, (((var_nt_dn4 * var_mid) + (var_nt * var_mid_dn4)) / (2.0 * assign62750_e81221)), ((var_nt * var_mid_dn6) / (2.0 * assign62750_e81221)), ((var_nt * var_mid_dn7) / (2.0 * assign62750_e81221)), ((var_nt * var_mid_dn8) / (2.0 * assign62750_e81221)), ((var_nt * var_mid_dn9) / (2.0 * assign62750_e81221)),)
    } else {
        (var_sqid, var_sqid_dn4, var_sqid_dn6, var_sqid_dn7, var_sqid_dn8, var_sqid_dn9,)
    }
};
        var_sqid = assign62750_e81223;
        var_sqid_dn4 = assign62750_e81223_d_n4;
        var_sqid_dn6 = assign62750_e81223_d_n6;
        var_sqid_dn7 = assign62750_e81223_d_n7;
        var_sqid_dn8 = assign62750_e81223_d_n8;
        var_sqid_dn9 = assign62750_e81223_d_n9;

        let assign62760_e81238: f64 = if ((((p.p50 == 1.0) && (var_nt > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1787 = assign62760_e81238;

        let (assign62770_e81270, assign62770_e81270_d_n4, assign62770_e81270_d_n6, assign62770_e81270_d_n7, assign62770_e81270_d_n8, assign62770_e81270_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) {
        let assign62770_e81244: f64 = (var_t1 / 12.0);
        let assign62770_e81248: f64 = (var_t1 + 0.2);
        let assign62770_e81251: f64 = (12.0 * var_t2);
        let assign62770_e81252: f64 = (assign62770_e81248 - assign62770_e81251);
        let assign62770_e81253: f64 = (var_t2 * assign62770_e81252);
        let assign62770_e81254: f64 = (assign62770_e81244 - assign62770_e81253);
        let assign62770_e81259: f64 = (var_t1 + 1.0);
        let assign62770_e81262: f64 = (12.0 * var_t2);
        let assign62770_e81263: f64 = (assign62770_e81259 - assign62770_e81262);
        let assign62770_e81264: f64 = (var_t2 * assign62770_e81263);
        let assign62770_e81266: f64 = (assign62770_e81264 * var_r);
        let assign62770_e81267: f64 = (1.6 * assign62770_e81266);
        let assign62770_e81268: f64 = (assign62770_e81254 - assign62770_e81267);
        (assign62770_e81268, (((var_t1_dn4 / 12.0) - ((var_t2_dn4 * assign62770_e81252) + (var_t2 * (var_t1_dn4 - (12.0 * var_t2_dn4))))) - (1.6 * ((((var_t2_dn4 * assign62770_e81263) + (var_t2 * (var_t1_dn4 - (12.0 * var_t2_dn4)))) * var_r) + (assign62770_e81264 * var_r_dn4)))), (((var_t1_dn6 / 12.0) - ((var_t2_dn6 * assign62770_e81252) + (var_t2 * (var_t1_dn6 - (12.0 * var_t2_dn6))))) - (1.6 * ((((var_t2_dn6 * assign62770_e81263) + (var_t2 * (var_t1_dn6 - (12.0 * var_t2_dn6)))) * var_r) + (assign62770_e81264 * var_r_dn6)))), (((var_t1_dn7 / 12.0) - ((var_t2_dn7 * assign62770_e81252) + (var_t2 * (var_t1_dn7 - (12.0 * var_t2_dn7))))) - (1.6 * ((((var_t2_dn7 * assign62770_e81263) + (var_t2 * (var_t1_dn7 - (12.0 * var_t2_dn7)))) * var_r) + (assign62770_e81264 * var_r_dn7)))), (((var_t1_dn8 / 12.0) - ((var_t2_dn8 * assign62770_e81252) + (var_t2 * (var_t1_dn8 - (12.0 * var_t2_dn8))))) - (1.6 * ((((var_t2_dn8 * assign62770_e81263) + (var_t2 * (var_t1_dn8 - (12.0 * var_t2_dn8)))) * var_r) + (assign62770_e81264 * var_r_dn8)))), (((var_t1_dn9 / 12.0) - ((var_t2_dn9 * assign62770_e81252) + (var_t2 * (var_t1_dn9 - (12.0 * var_t2_dn9))))) - (1.6 * ((((var_t2_dn9 * assign62770_e81263) + (var_t2 * (var_t1_dn9 - (12.0 * var_t2_dn9)))) * var_r) + (assign62770_e81264 * var_r_dn9)))),)
    } else {
        (var_mig, var_mig_dn4, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9,)
    }
};
        var_mig = assign62770_e81270;
        var_mig_dn4 = assign62770_e81270_d_n4;
        var_mig_dn6 = assign62770_e81270_d_n6;
        var_mig_dn7 = assign62770_e81270_d_n7;
        var_mig_dn8 = assign62770_e81270_d_n8;
        var_mig_dn9 = assign62770_e81270_d_n9;

        let (assign62780_e81281, assign62780_e81281_d_n4, assign62780_e81281_d_n6, assign62780_e81281_d_n7, assign62780_e81281_d_n8, assign62780_e81281_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) {
        let (assign62780_e81279, assign62780_e81279_d_n4, assign62780_e81279_d_n6, assign62780_e81279_d_n7, assign62780_e81279_d_n8, assign62780_e81279_d_n9,) = {
            if (var_mig > 1e-40) {
                (var_mig, var_mig_dn4, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9,)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62780_e81279, assign62780_e81279_d_n4, assign62780_e81279_d_n6, assign62780_e81279_d_n7, assign62780_e81279_d_n8, assign62780_e81279_d_n9,)
    } else {
        (var_mig, var_mig_dn4, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9,)
    }
};
        var_mig = assign62780_e81281;
        var_mig_dn4 = assign62780_e81281_d_n4;
        var_mig_dn6 = assign62780_e81281_d_n6;
        var_mig_dn7 = assign62780_e81281_d_n7;
        var_mig_dn8 = assign62780_e81281_d_n8;
        var_mig_dn9 = assign62780_e81281_d_n9;

        let (assign62790_e81291, assign62790_e81291_d_n4, assign62790_e81291_d_n6, assign62790_e81291_d_n7, assign62790_e81291_d_n8, assign62790_e81291_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) {
        let assign62790_e81287: f64 = (var_lcinv2 / var_g_ideal);
        let assign62790_e81289: f64 = (assign62790_e81287 * var_mig);
        (assign62790_e81289, (((((var_lcinv2_dn4 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn4)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62790_e81287 * var_mig_dn4)), (((((var_lcinv2_dn6 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn6)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62790_e81287 * var_mig_dn6)), (((((var_lcinv2_dn7 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn7)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62790_e81287 * var_mig_dn7)), (((((var_lcinv2_dn8 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn8)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62790_e81287 * var_mig_dn8)), (((((var_lcinv2_dn9 * var_g_ideal) - (var_lcinv2 * var_g_ideal_dn9)) / (var_g_ideal * var_g_ideal)) * var_mig) + (assign62790_e81287 * var_mig_dn9)),)
    } else {
        (var_mig, var_mig_dn4, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9,)
    }
};
        var_mig = assign62790_e81291;
        var_mig_dn4 = assign62790_e81291_d_n4;
        var_mig_dn6 = assign62790_e81291_d_n6;
        var_mig_dn7 = assign62790_e81291_d_n7;
        var_mig_dn8 = assign62790_e81291_d_n8;
        var_mig_dn9 = assign62790_e81291_d_n9;

        let (assign62800_e81319, assign62800_e81319_d_n4, assign62800_e81319_d_n6, assign62800_e81319_d_n7, assign62800_e81319_d_n8, assign62800_e81319_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) {
        let assign62800_e81297: f64 = (var_lcinv2 * var_sqt2);
        let assign62800_e81301: f64 = (12.0 * var_t2);
        let assign62800_e81302: f64 = (1.0 - assign62800_e81301);
        let assign62800_e81306: f64 = (19.2 * var_t2);
        let assign62800_e81307: f64 = (var_t1 + assign62800_e81306);
        let assign62800_e81311: f64 = (var_t1 * var_t2);
        let assign62800_e81312: f64 = (12.0 * assign62800_e81311);
        let assign62800_e81313: f64 = (assign62800_e81307 - assign62800_e81312);
        let assign62800_e81315: f64 = (assign62800_e81313 * var_r);
        let assign62800_e81316: f64 = (assign62800_e81302 - assign62800_e81315);
        let assign62800_e81317: f64 = (assign62800_e81297 * assign62800_e81316);
        (assign62800_e81317, ((((var_lcinv2_dn4 * var_sqt2) + (var_lcinv2 * var_sqt2_dn4)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * var_t2_dn4)) - ((((var_t1_dn4 + (19.2 * var_t2_dn4)) - (12.0 * ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)))) * var_r) + (assign62800_e81313 * var_r_dn4))))), ((((var_lcinv2_dn6 * var_sqt2) + (var_lcinv2 * var_sqt2_dn6)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * var_t2_dn6)) - ((((var_t1_dn6 + (19.2 * var_t2_dn6)) - (12.0 * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * var_r) + (assign62800_e81313 * var_r_dn6))))), ((((var_lcinv2_dn7 * var_sqt2) + (var_lcinv2 * var_sqt2_dn7)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * var_t2_dn7)) - ((((var_t1_dn7 + (19.2 * var_t2_dn7)) - (12.0 * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * var_r) + (assign62800_e81313 * var_r_dn7))))), ((((var_lcinv2_dn8 * var_sqt2) + (var_lcinv2 * var_sqt2_dn8)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * var_t2_dn8)) - ((((var_t1_dn8 + (19.2 * var_t2_dn8)) - (12.0 * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * var_r) + (assign62800_e81313 * var_r_dn8))))), ((((var_lcinv2_dn9 * var_sqt2) + (var_lcinv2 * var_sqt2_dn9)) * assign62800_e81316) + (assign62800_e81297 * ((-(12.0 * var_t2_dn9)) - ((((var_t1_dn9 + (19.2 * var_t2_dn9)) - (12.0 * ((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)))) * var_r) + (assign62800_e81313 * var_r_dn9))))),)
    } else {
        (var_migid0, var_migid0_dn4, var_migid0_dn6, var_migid0_dn7, var_migid0_dn8, var_migid0_dn9,)
    }
};
        var_migid0 = assign62800_e81319;
        var_migid0_dn4 = assign62800_e81319_d_n4;
        var_migid0_dn6 = assign62800_e81319_d_n6;
        var_migid0_dn7 = assign62800_e81319_d_n7;
        var_migid0_dn8 = assign62800_e81319_d_n8;
        var_migid0_dn9 = assign62800_e81319_d_n9;

        let (assign62810_e81335, assign62810_e81335_d_n4, assign62810_e81335_d_n6, assign62810_e81335_d_n7, assign62810_e81335_d_n8, assign62810_e81335_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) {
        let assign62810_e81325: f64 = (var_gvsat_ac * var_gvsat_ac);
        let assign62810_e81327: f64 = (assign62810_e81325 * var_cox_qm);
        let assign62810_e81329: f64 = (assign62810_e81327 * var_eta_p_ac);
        let assign62810_e81332: f64 = (var_gmob_dl_ac * var_gmob_dl_ac);
        let assign62810_e81333: f64 = (assign62810_e81329 / assign62810_e81332);
        (assign62810_e81333, (((((((((var_gvsat_ac_dn4 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn4)) * var_cox_qm) + (assign62810_e81325 * var_cox_qm_dn4)) * var_eta_p_ac) + (assign62810_e81327 * var_eta_p_ac_dn4)) * assign62810_e81332) - (assign62810_e81329 * ((var_gmob_dl_ac_dn4 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn4)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((var_gvsat_ac_dn6 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn6)) * var_cox_qm) + (assign62810_e81325 * var_cox_qm_dn6)) * var_eta_p_ac) + (assign62810_e81327 * var_eta_p_ac_dn6)) * assign62810_e81332) - (assign62810_e81329 * ((var_gmob_dl_ac_dn6 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn6)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((var_gvsat_ac_dn7 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn7)) * var_cox_qm) + (assign62810_e81325 * var_cox_qm_dn7)) * var_eta_p_ac) + (assign62810_e81327 * var_eta_p_ac_dn7)) * assign62810_e81332) - (assign62810_e81329 * ((var_gmob_dl_ac_dn7 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn7)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((var_gvsat_ac_dn8 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn8)) * var_cox_qm) + (assign62810_e81325 * var_cox_qm_dn8)) * var_eta_p_ac) + (assign62810_e81327 * var_eta_p_ac_dn8)) * assign62810_e81332) - (assign62810_e81329 * ((var_gmob_dl_ac_dn8 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn8)))) / (assign62810_e81332 * assign62810_e81332)), (((((((((var_gvsat_ac_dn9 * var_gvsat_ac) + (var_gvsat_ac * var_gvsat_ac_dn9)) * var_cox_qm) + (assign62810_e81325 * var_cox_qm_dn9)) * var_eta_p_ac) + (assign62810_e81327 * var_eta_p_ac_dn9)) * assign62810_e81332) - (assign62810_e81329 * ((var_gmob_dl_ac_dn9 * var_gmob_dl_ac) + (var_gmob_dl_ac * var_gmob_dl_ac_dn9)))) / (assign62810_e81332 * assign62810_e81332)),)
    } else {
        (var_cgeff, var_cgeff_dn4, var_cgeff_dn6, var_cgeff_dn7, var_cgeff_dn8, var_cgeff_dn9,)
    }
};
        var_cgeff = assign62810_e81335;
        var_cgeff_dn4 = assign62810_e81335_d_n4;
        var_cgeff_dn6 = assign62810_e81335_d_n6;
        var_cgeff_dn7 = assign62810_e81335_d_n7;
        var_cgeff_dn8 = assign62810_e81335_d_n8;
        var_cgeff_dn9 = assign62810_e81335_d_n9;

        let assign62820_e81338: f64 = if var_fntexc_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1788 = assign62820_e81338;

        let (assign62830_e81362, assign62830_e81362_d_n4, assign62830_e81362_d_n6, assign62830_e81362_d_n7, assign62830_e81362_d_n8, assign62830_e81362_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) && (var_guard1788 != 0.0)) {
        let assign62830_e81349: f64 = (12.0 * var_t2);
        let assign62830_e81350: f64 = (1.0 + assign62830_e81349);
        let assign62830_e81351: f64 = (var_sidexc * assign62830_e81350);
        let assign62830_e81354: f64 = (12.0 * var_g_ideal);
        let assign62830_e81356: f64 = (assign62830_e81354 * var_g_ideal);
        let assign62830_e81358: f64 = (assign62830_e81356 * var_nt0);
        let assign62830_e81359: f64 = (assign62830_e81351 / assign62830_e81358);
        let assign62830_e81360: f64 = (var_mig + assign62830_e81359);
        (assign62830_e81360, (var_mig_dn4 + (((((var_sidexc_dn4 * assign62830_e81350) + (var_sidexc * (12.0 * var_t2_dn4))) * assign62830_e81358) - (assign62830_e81351 * (((((12.0 * var_g_ideal_dn4) * var_g_ideal) + (assign62830_e81354 * var_g_ideal_dn4)) * var_nt0) + (assign62830_e81356 * var_nt0_dn4)))) / (assign62830_e81358 * assign62830_e81358))), (var_mig_dn6 + (((((var_sidexc_dn6 * assign62830_e81350) + (var_sidexc * (12.0 * var_t2_dn6))) * assign62830_e81358) - (assign62830_e81351 * ((((12.0 * var_g_ideal_dn6) * var_g_ideal) + (assign62830_e81354 * var_g_ideal_dn6)) * var_nt0))) / (assign62830_e81358 * assign62830_e81358))), (var_mig_dn7 + (((((var_sidexc_dn7 * assign62830_e81350) + (var_sidexc * (12.0 * var_t2_dn7))) * assign62830_e81358) - (assign62830_e81351 * ((((12.0 * var_g_ideal_dn7) * var_g_ideal) + (assign62830_e81354 * var_g_ideal_dn7)) * var_nt0))) / (assign62830_e81358 * assign62830_e81358))), (var_mig_dn8 + (((((var_sidexc_dn8 * assign62830_e81350) + (var_sidexc * (12.0 * var_t2_dn8))) * assign62830_e81358) - (assign62830_e81351 * ((((12.0 * var_g_ideal_dn8) * var_g_ideal) + (assign62830_e81354 * var_g_ideal_dn8)) * var_nt0))) / (assign62830_e81358 * assign62830_e81358))), (var_mig_dn9 + (((((var_sidexc_dn9 * assign62830_e81350) + (var_sidexc * (12.0 * var_t2_dn9))) * assign62830_e81358) - (assign62830_e81351 * ((((12.0 * var_g_ideal_dn9) * var_g_ideal) + (assign62830_e81354 * var_g_ideal_dn9)) * var_nt0))) / (assign62830_e81358 * assign62830_e81358))),)
    } else {
        (var_mig, var_mig_dn4, var_mig_dn6, var_mig_dn7, var_mig_dn8, var_mig_dn9,)
    }
};
        var_mig = assign62830_e81362;
        var_mig_dn4 = assign62830_e81362_d_n4;
        var_mig_dn6 = assign62830_e81362_d_n6;
        var_mig_dn7 = assign62830_e81362_d_n7;
        var_mig_dn8 = assign62830_e81362_d_n8;
        var_mig_dn9 = assign62830_e81362_d_n9;

        let (assign62840_e81382, assign62840_e81382_d_n4, assign62840_e81382_d_n6, assign62840_e81382_d_n7, assign62840_e81382_d_n8, assign62840_e81382_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) && (var_guard1788 != 0.0)) {
        let assign62840_e81371: f64 = (var_sidexc * var_sqt2);
        let assign62840_e81374: f64 = (1.0 + var_r);
        let assign62840_e81375: f64 = (assign62840_e81371 * assign62840_e81374);
        let assign62840_e81378: f64 = (var_g_ideal * var_nt0);
        let assign62840_e81379: f64 = (assign62840_e81375 / assign62840_e81378);
        let assign62840_e81380: f64 = (var_migid0 - assign62840_e81379);
        (assign62840_e81380, (var_migid0_dn4 - (((((((var_sidexc_dn4 * var_sqt2) + (var_sidexc * var_sqt2_dn4)) * assign62840_e81374) + (assign62840_e81371 * var_r_dn4)) * assign62840_e81378) - (assign62840_e81375 * ((var_g_ideal_dn4 * var_nt0) + (var_g_ideal * var_nt0_dn4)))) / (assign62840_e81378 * assign62840_e81378))), (var_migid0_dn6 - (((((((var_sidexc_dn6 * var_sqt2) + (var_sidexc * var_sqt2_dn6)) * assign62840_e81374) + (assign62840_e81371 * var_r_dn6)) * assign62840_e81378) - (assign62840_e81375 * (var_g_ideal_dn6 * var_nt0))) / (assign62840_e81378 * assign62840_e81378))), (var_migid0_dn7 - (((((((var_sidexc_dn7 * var_sqt2) + (var_sidexc * var_sqt2_dn7)) * assign62840_e81374) + (assign62840_e81371 * var_r_dn7)) * assign62840_e81378) - (assign62840_e81375 * (var_g_ideal_dn7 * var_nt0))) / (assign62840_e81378 * assign62840_e81378))), (var_migid0_dn8 - (((((((var_sidexc_dn8 * var_sqt2) + (var_sidexc * var_sqt2_dn8)) * assign62840_e81374) + (assign62840_e81371 * var_r_dn8)) * assign62840_e81378) - (assign62840_e81375 * (var_g_ideal_dn8 * var_nt0))) / (assign62840_e81378 * assign62840_e81378))), (var_migid0_dn9 - (((((((var_sidexc_dn9 * var_sqt2) + (var_sidexc * var_sqt2_dn9)) * assign62840_e81374) + (assign62840_e81371 * var_r_dn9)) * assign62840_e81378) - (assign62840_e81375 * (var_g_ideal_dn9 * var_nt0))) / (assign62840_e81378 * assign62840_e81378))),)
    } else {
        (var_migid0, var_migid0_dn4, var_migid0_dn6, var_migid0_dn7, var_migid0_dn8, var_migid0_dn9,)
    }
};
        var_migid0 = assign62840_e81382;
        var_migid0_dn4 = assign62840_e81382_d_n4;
        var_migid0_dn6 = assign62840_e81382_d_n6;
        var_migid0_dn7 = assign62840_e81382_d_n7;
        var_migid0_dn8 = assign62840_e81382_d_n8;
        var_migid0_dn9 = assign62840_e81382_d_n9;

        let (assign62850_e81391, assign62850_e81391_d_n4, assign62850_e81391_d_n6, assign62850_e81391_d_n7, assign62850_e81391_d_n8, assign62850_e81391_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) {
        let assign62850_e81388: f64 = (var_nt / var_mig);
        let assign62850_e81389: f64 = (assign62850_e81388).sqrt();
        (assign62850_e81389, ((((var_nt_dn4 * var_mig) - (var_nt * var_mig_dn4)) / (var_mig * var_mig)) / (2.0 * assign62850_e81389)), ((-((var_nt * var_mig_dn6) / (var_mig * var_mig))) / (2.0 * assign62850_e81389)), ((-((var_nt * var_mig_dn7) / (var_mig * var_mig))) / (2.0 * assign62850_e81389)), ((-((var_nt * var_mig_dn8) / (var_mig * var_mig))) / (2.0 * assign62850_e81389)), ((-((var_nt * var_mig_dn9) / (var_mig * var_mig))) / (2.0 * assign62850_e81389)),)
    } else {
        (var_sqig, var_sqig_dn4, var_sqig_dn6, var_sqig_dn7, var_sqig_dn8, var_sqig_dn9,)
    }
};
        var_sqig = assign62850_e81391;
        var_sqig_dn4 = assign62850_e81391_d_n4;
        var_sqig_dn6 = assign62850_e81391_d_n6;
        var_sqig_dn7 = assign62850_e81391_d_n7;
        var_sqig_dn8 = assign62850_e81391_d_n8;
        var_sqig_dn9 = assign62850_e81391_d_n9;

        let assign62860_e81394: f64 = if var_sqid <= 0.0 { 1.0 } else { 0.0 };
        var_guard1789 = assign62860_e81394;

        let (assign62870_e81402, assign62870_e81402_d_n4, assign62870_e81402_d_n6, assign62870_e81402_d_n7, assign62870_e81402_d_n8, assign62870_e81402_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) && (var_guard1789 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_c_igid, var_c_igid_dn4, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn9,)
    }
};
        var_c_igid = assign62870_e81402;
        var_c_igid_dn4 = assign62870_e81402_d_n4;
        var_c_igid_dn6 = assign62870_e81402_d_n6;
        var_c_igid_dn7 = assign62870_e81402_d_n7;
        var_c_igid_dn8 = assign62870_e81402_d_n8;
        var_c_igid_dn9 = assign62870_e81402_d_n9;

        let (assign62880_e81415, assign62880_e81415_d_n4, assign62880_e81415_d_n6, assign62880_e81415_d_n7, assign62880_e81415_d_n8, assign62880_e81415_d_n9,) = {
    if (((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) && (var_guard1789 == 0.0)) {
        let assign62880_e81411: f64 = (var_migid0 * var_sqig);
        let assign62880_e81413: f64 = (assign62880_e81411 / var_sqid);
        (assign62880_e81413, (((((var_migid0_dn4 * var_sqig) + (var_migid0 * var_sqig_dn4)) * var_sqid) - (assign62880_e81411 * var_sqid_dn4)) / (var_sqid * var_sqid)), (((((var_migid0_dn6 * var_sqig) + (var_migid0 * var_sqig_dn6)) * var_sqid) - (assign62880_e81411 * var_sqid_dn6)) / (var_sqid * var_sqid)), (((((var_migid0_dn7 * var_sqig) + (var_migid0 * var_sqig_dn7)) * var_sqid) - (assign62880_e81411 * var_sqid_dn7)) / (var_sqid * var_sqid)), (((((var_migid0_dn8 * var_sqig) + (var_migid0 * var_sqig_dn8)) * var_sqid) - (assign62880_e81411 * var_sqid_dn8)) / (var_sqid * var_sqid)), (((((var_migid0_dn9 * var_sqig) + (var_migid0 * var_sqig_dn9)) * var_sqid) - (assign62880_e81411 * var_sqid_dn9)) / (var_sqid * var_sqid)),)
    } else {
        (var_c_igid, var_c_igid_dn4, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn9,)
    }
};
        var_c_igid = assign62880_e81415;
        var_c_igid_dn4 = assign62880_e81415_d_n4;
        var_c_igid_dn6 = assign62880_e81415_d_n6;
        var_c_igid_dn7 = assign62880_e81415_d_n7;
        var_c_igid_dn8 = assign62880_e81415_d_n8;
        var_c_igid_dn9 = assign62880_e81415_d_n9;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_dn4_slot = var_c_igid_dn4;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_c_igid_dn9_slot = var_c_igid_dn9;
        *var_cgeff_slot = var_cgeff;
        *var_cgeff_dn4_slot = var_cgeff_dn4;
        *var_cgeff_dn6_slot = var_cgeff_dn6;
        *var_cgeff_dn7_slot = var_cgeff_dn7;
        *var_cgeff_dn8_slot = var_cgeff_dn8;
        *var_cgeff_dn9_slot = var_cgeff_dn9;
        *var_g_ideal_slot = var_g_ideal;
        *var_g_ideal_dn4_slot = var_g_ideal_dn4;
        *var_g_ideal_dn6_slot = var_g_ideal_dn6;
        *var_g_ideal_dn7_slot = var_g_ideal_dn7;
        *var_g_ideal_dn8_slot = var_g_ideal_dn8;
        *var_g_ideal_dn9_slot = var_g_ideal_dn9;
        *var_gfac_slot = var_gfac;
        *var_gfac_dn4_slot = var_gfac_dn4;
        *var_gfac_dn6_slot = var_gfac_dn6;
        *var_gfac_dn7_slot = var_gfac_dn7;
        *var_gfac_dn8_slot = var_gfac_dn8;
        *var_gfac_dn9_slot = var_gfac_dn9;
        *var_guard1785_slot = var_guard1785;
        *var_guard1786_slot = var_guard1786;
        *var_guard1787_slot = var_guard1787;
        *var_guard1788_slot = var_guard1788;
        *var_guard1789_slot = var_guard1789;
        *var_gvsat_exc_slot = var_gvsat_exc;
        *var_gvsat_exc_dn4_slot = var_gvsat_exc_dn4;
        *var_gvsat_exc_dn6_slot = var_gvsat_exc_dn6;
        *var_gvsat_exc_dn7_slot = var_gvsat_exc_dn7;
        *var_gvsat_exc_dn8_slot = var_gvsat_exc_dn8;
        *var_gvsat_exc_dn9_slot = var_gvsat_exc_dn9;
        *var_lc_slot = var_lc;
        *var_lc_dn4_slot = var_lc_dn4;
        *var_lc_dn6_slot = var_lc_dn6;
        *var_lc_dn7_slot = var_lc_dn7;
        *var_lc_dn8_slot = var_lc_dn8;
        *var_lc_dn9_slot = var_lc_dn9;
        *var_lcinv2_slot = var_lcinv2;
        *var_lcinv2_dn4_slot = var_lcinv2_dn4;
        *var_lcinv2_dn6_slot = var_lcinv2_dn6;
        *var_lcinv2_dn7_slot = var_lcinv2_dn7;
        *var_lcinv2_dn8_slot = var_lcinv2_dn8;
        *var_lcinv2_dn9_slot = var_lcinv2_dn9;
        *var_mid_slot = var_mid;
        *var_mid_dn4_slot = var_mid_dn4;
        *var_mid_dn6_slot = var_mid_dn6;
        *var_mid_dn7_slot = var_mid_dn7;
        *var_mid_dn8_slot = var_mid_dn8;
        *var_mid_dn9_slot = var_mid_dn9;
        *var_mig_slot = var_mig;
        *var_mig_dn4_slot = var_mig_dn4;
        *var_mig_dn6_slot = var_mig_dn6;
        *var_mig_dn7_slot = var_mig_dn7;
        *var_mig_dn8_slot = var_mig_dn8;
        *var_mig_dn9_slot = var_mig_dn9;
        *var_migid0_slot = var_migid0;
        *var_migid0_dn4_slot = var_migid0_dn4;
        *var_migid0_dn6_slot = var_migid0_dn6;
        *var_migid0_dn7_slot = var_migid0_dn7;
        *var_migid0_dn8_slot = var_migid0_dn8;
        *var_migid0_dn9_slot = var_migid0_dn9;
        *var_r_slot = var_r;
        *var_r_dn4_slot = var_r_dn4;
        *var_r_dn6_slot = var_r_dn6;
        *var_r_dn7_slot = var_r_dn7;
        *var_r_dn8_slot = var_r_dn8;
        *var_r_dn9_slot = var_r_dn9;
        *var_sidexc_slot = var_sidexc;
        *var_sidexc_dn4_slot = var_sidexc_dn4;
        *var_sidexc_dn6_slot = var_sidexc_dn6;
        *var_sidexc_dn7_slot = var_sidexc_dn7;
        *var_sidexc_dn8_slot = var_sidexc_dn8;
        *var_sidexc_dn9_slot = var_sidexc_dn9;
        *var_sqid_slot = var_sqid;
        *var_sqid_dn4_slot = var_sqid_dn4;
        *var_sqid_dn6_slot = var_sqid_dn6;
        *var_sqid_dn7_slot = var_sqid_dn7;
        *var_sqid_dn8_slot = var_sqid_dn8;
        *var_sqid_dn9_slot = var_sqid_dn9;
        *var_sqig_slot = var_sqig;
        *var_sqig_dn4_slot = var_sqig_dn4;
        *var_sqig_dn6_slot = var_sqig_dn6;
        *var_sqig_dn7_slot = var_sqig_dn7;
        *var_sqig_dn8_slot = var_sqig_dn8;
        *var_sqig_dn9_slot = var_sqig_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_thesat1_exc_slot = var_thesat1_exc;
        *var_thesat1_exc_dn4_slot = var_thesat1_exc_dn4;
        *var_thesat1_exc_dn6_slot = var_thesat1_exc_dn6;
        *var_thesat1_exc_dn7_slot = var_thesat1_exc_dn7;
        *var_thesat1_exc_dn8_slot = var_thesat1_exc_dn8;
        *var_thesat1_exc_dn9_slot = var_thesat1_exc_dn9;
        *var_zsat_exc_slot = var_zsat_exc;
        *var_zsat_exc_dn4_slot = var_zsat_exc_dn4;
        *var_zsat_exc_dn6_slot = var_zsat_exc_dn6;
        *var_zsat_exc_dn7_slot = var_zsat_exc_dn7;
        *var_zsat_exc_dn8_slot = var_zsat_exc_dn8;
        *var_zsat_exc_dn9_slot = var_zsat_exc_dn9;
    }

    pub(super) fn stamp_transient_block_149(
        p: &Parameters,
        var_alpha_dc: f64,
        var_alpha_dc_dn4: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_alpha_dc_dn9: f64,
        var_betnedge_i: f64,
        var_cox_over_q: f64,
        var_dsqredge: f64,
        var_dsqredge_dn4: f64,
        var_dsqredge_dn6: f64,
        var_dsqredge_dn7: f64,
        var_dsqredge_dn8: f64,
        var_dsqredge_dn9: f64,
        var_gfedge2: f64,
        var_gfedge2_dn4: f64,
        var_guard1782: f64,
        var_guard1787: f64,
        var_h_dc: f64,
        var_h_dc_dn4: f64,
        var_h_dc_dn6: f64,
        var_h_dc_dn7: f64,
        var_h_dc_dn8: f64,
        var_h_dc_dn9: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_sqid: f64,
        var_sqid_dn4: f64,
        var_sqid_dn6: f64,
        var_sqid_dn7: f64,
        var_sqid_dn8: f64,
        var_sqid_dn9: f64,
        var_sqig: f64,
        var_sqig_dn4: f64,
        var_sqig_dn6: f64,
        var_sqig_dn7: f64,
        var_sqig_dn8: f64,
        var_sqig_dn9: f64,
        var_xgedge: f64,
        var_c_igid_slot: &mut f64,
        var_c_igid_dn4_slot: &mut f64,
        var_c_igid_dn6_slot: &mut f64,
        var_c_igid_dn7_slot: &mut f64,
        var_c_igid_dn8_slot: &mut f64,
        var_c_igid_dn9_slot: &mut f64,
        var_guard1791_slot: &mut f64,
        var_migid_slot: &mut f64,
        var_migid_dn4_slot: &mut f64,
        var_migid_dn6_slot: &mut f64,
        var_migid_dn7_slot: &mut f64,
        var_migid_dn8_slot: &mut f64,
        var_migid_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
    ) {
        let mut var_c_igid: f64 = *var_c_igid_slot;
        let mut var_c_igid_dn4: f64 = *var_c_igid_dn4_slot;
        let mut var_c_igid_dn6: f64 = *var_c_igid_dn6_slot;
        let mut var_c_igid_dn7: f64 = *var_c_igid_dn7_slot;
        let mut var_c_igid_dn8: f64 = *var_c_igid_dn8_slot;
        let mut var_c_igid_dn9: f64 = *var_c_igid_dn9_slot;
        let mut var_guard1791: f64 = *var_guard1791_slot;
        let mut var_migid: f64 = *var_migid_slot;
        let mut var_migid_dn4: f64 = *var_migid_dn4_slot;
        let mut var_migid_dn6: f64 = *var_migid_dn6_slot;
        let mut var_migid_dn7: f64 = *var_migid_dn7_slot;
        let mut var_migid_dn8: f64 = *var_migid_dn8_slot;
        let mut var_migid_dn9: f64 = *var_migid_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;

        let (assign62890_e81431, assign62890_e81431_d_n4, assign62890_e81431_d_n6, assign62890_e81431_d_n7, assign62890_e81431_d_n8, assign62890_e81431_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) {
        let (assign62890_e81429, assign62890_e81429_d_n4, assign62890_e81429_d_n6, assign62890_e81429_d_n7, assign62890_e81429_d_n8, assign62890_e81429_d_n9,) = {
            if (var_c_igid > 0.0) {
                let (assign62890_e81427, assign62890_e81427_d_n4, assign62890_e81427_d_n6, assign62890_e81427_d_n7, assign62890_e81427_d_n8, assign62890_e81427_d_n9,) = {
                    if (var_c_igid < 1.0) {
                        (var_c_igid, var_c_igid_dn4, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn9,)
                    } else {
                        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign62890_e81427, assign62890_e81427_d_n4, assign62890_e81427_d_n6, assign62890_e81427_d_n7, assign62890_e81427_d_n8, assign62890_e81427_d_n9,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign62890_e81429, assign62890_e81429_d_n4, assign62890_e81429_d_n6, assign62890_e81429_d_n7, assign62890_e81429_d_n8, assign62890_e81429_d_n9,)
    } else {
        (var_c_igid, var_c_igid_dn4, var_c_igid_dn6, var_c_igid_dn7, var_c_igid_dn8, var_c_igid_dn9,)
    }
};
        var_c_igid = assign62890_e81431;
        var_c_igid_dn4 = assign62890_e81431_d_n4;
        var_c_igid_dn6 = assign62890_e81431_d_n6;
        var_c_igid_dn7 = assign62890_e81431_d_n7;
        var_c_igid_dn8 = assign62890_e81431_d_n8;
        var_c_igid_dn9 = assign62890_e81431_d_n9;

        let (assign62900_e81441, assign62900_e81441_d_n4, assign62900_e81441_d_n6, assign62900_e81441_d_n7, assign62900_e81441_d_n8, assign62900_e81441_d_n9,) = {
    if ((var_guard1782 != 0.0) && (var_guard1787 != 0.0)) {
        let assign62900_e81437: f64 = (var_c_igid * var_sqid);
        let assign62900_e81439: f64 = (assign62900_e81437 / var_sqig);
        (assign62900_e81439, (((((var_c_igid_dn4 * var_sqid) + (var_c_igid * var_sqid_dn4)) * var_sqig) - (assign62900_e81437 * var_sqig_dn4)) / (var_sqig * var_sqig)), (((((var_c_igid_dn6 * var_sqid) + (var_c_igid * var_sqid_dn6)) * var_sqig) - (assign62900_e81437 * var_sqig_dn6)) / (var_sqig * var_sqig)), (((((var_c_igid_dn7 * var_sqid) + (var_c_igid * var_sqid_dn7)) * var_sqig) - (assign62900_e81437 * var_sqig_dn7)) / (var_sqig * var_sqig)), (((((var_c_igid_dn8 * var_sqid) + (var_c_igid * var_sqid_dn8)) * var_sqig) - (assign62900_e81437 * var_sqig_dn8)) / (var_sqig * var_sqig)), (((((var_c_igid_dn9 * var_sqid) + (var_c_igid * var_sqid_dn9)) * var_sqig) - (assign62900_e81437 * var_sqig_dn9)) / (var_sqig * var_sqig)),)
    } else {
        (var_migid, var_migid_dn4, var_migid_dn6, var_migid_dn7, var_migid_dn8, var_migid_dn9,)
    }
};
        var_migid = assign62900_e81441;
        var_migid_dn4 = assign62900_e81441_d_n4;
        var_migid_dn6 = assign62900_e81441_d_n6;
        var_migid_dn7 = assign62900_e81441_d_n7;
        var_migid_dn8 = assign62900_e81441_d_n8;
        var_migid_dn9 = assign62900_e81441_d_n9;

        let assign63070_e81549: f64 = if (((p.p46 != 0.0) && (var_betnedge_i > 0.0)) && (var_xgedge > 0.0)) { 1.0 } else { 0.0 };
        var_guard1791 = assign63070_e81549;

        let (assign63080_e81557, assign63080_e81557_d_n4, assign63080_e81557_d_n6, assign63080_e81557_d_n7, assign63080_e81557_d_n8, assign63080_e81557_d_n9,) = {
    if (var_guard1791 != 0.0) {
        let assign63080_e81553: f64 = (4.0 * var_dsqredge);
        let assign63080_e81555: f64 = (assign63080_e81553 / var_gfedge2);
        (assign63080_e81555, ((((4.0 * var_dsqredge_dn4) * var_gfedge2) - (assign63080_e81553 * var_gfedge2_dn4)) / (var_gfedge2 * var_gfedge2)), ((4.0 * var_dsqredge_dn6) / var_gfedge2), ((4.0 * var_dsqredge_dn7) / var_gfedge2), ((4.0 * var_dsqredge_dn8) / var_gfedge2), ((4.0 * var_dsqredge_dn9) / var_gfedge2),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign63080_e81557;
        var_temp1_dn4 = assign63080_e81557_d_n4;
        var_temp1_dn6 = assign63080_e81557_d_n6;
        var_temp1_dn7 = assign63080_e81557_d_n7;
        var_temp1_dn8 = assign63080_e81557_d_n8;
        var_temp1_dn9 = assign63080_e81557_d_n9;

        let (assign63100_e81577, assign63100_e81577_d_n4, assign63100_e81577_d_n6, assign63100_e81577_d_n7, assign63100_e81577_d_n8, assign63100_e81577_d_n9,) = {
    if (var_guard1791 != 0.0) {
        let assign63100_e81575: f64 = (var_cox_over_q * var_phit);
        (assign63100_e81575, (var_cox_over_q * var_phit_dn4), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign63100_e81577;
        var_temp1_dn4 = assign63100_e81577_d_n4;
        var_temp1_dn6 = assign63100_e81577_d_n6;
        var_temp1_dn7 = assign63100_e81577_d_n7;
        var_temp1_dn8 = assign63100_e81577_d_n8;
        var_temp1_dn9 = assign63100_e81577_d_n9;

        let (assign63230_e81717, assign63230_e81717_d_n4, assign63230_e81717_d_n6, assign63230_e81717_d_n7, assign63230_e81717_d_n8, assign63230_e81717_d_n9,) = {
    if (var_guard1791 != 0.0) {
        let assign63230_e81715: f64 = (var_alpha_dc * var_h_dc);
        (assign63230_e81715, ((var_alpha_dc_dn4 * var_h_dc) + (var_alpha_dc * var_h_dc_dn4)), ((var_alpha_dc_dn6 * var_h_dc) + (var_alpha_dc * var_h_dc_dn6)), ((var_alpha_dc_dn7 * var_h_dc) + (var_alpha_dc * var_h_dc_dn7)), ((var_alpha_dc_dn8 * var_h_dc) + (var_alpha_dc * var_h_dc_dn8)), ((var_alpha_dc_dn9 * var_h_dc) + (var_alpha_dc * var_h_dc_dn9)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign63230_e81717;
        var_temp1_dn4 = assign63230_e81717_d_n4;
        var_temp1_dn6 = assign63230_e81717_d_n6;
        var_temp1_dn7 = assign63230_e81717_d_n7;
        var_temp1_dn8 = assign63230_e81717_d_n8;
        var_temp1_dn9 = assign63230_e81717_d_n9;

        *var_c_igid_slot = var_c_igid;
        *var_c_igid_dn4_slot = var_c_igid_dn4;
        *var_c_igid_dn6_slot = var_c_igid_dn6;
        *var_c_igid_dn7_slot = var_c_igid_dn7;
        *var_c_igid_dn8_slot = var_c_igid_dn8;
        *var_c_igid_dn9_slot = var_c_igid_dn9;
        *var_guard1791_slot = var_guard1791;
        *var_migid_slot = var_migid;
        *var_migid_dn4_slot = var_migid_dn4;
        *var_migid_dn6_slot = var_migid_dn6;
        *var_migid_dn7_slot = var_migid_dn7;
        *var_migid_dn8_slot = var_migid_dn8;
        *var_migid_dn9_slot = var_migid_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[981] = (p.p37 >= 0.0);
        s.store_scalar(981, if s.b[981] { 1.0 } else { 0.0 });

        if s.b[981] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[981]) {
            s.store_scalar(0, (-1.0));
        }

        s.store_scalar(756, (8.8541878176e-12 * 11.8));

        s.store_scalar(351, (273.15 + p.p38));

        s.store_scalar(475, 0.0);

        s.b[982] = (p.p944 > 0.5);
        s.store_scalar(982, if s.b[982] { 1.0 } else { 0.0 });

        if s.b[982] {
            s.store_scalar(475, 1.0);
        }

        if (!s.b[982]) {
            s.store_scalar(475, 0.0);
        }

        s.store_scalar(365, (273.15 + p.p840));

        s.store_scalar(368, (1.3806505e-23 / 1.6021918e-19));

        s.store_scalar(369, (s.v[368] * s.v[365]));

        s.store_scalar(370, (1.0 / s.v[369]));

        s.store_scalar(376, ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365])));

        s.store_scalar(379, (p.p851 + s.v[376]));

        s.store_scalar(380, (p.p852 + s.v[376]));

        s.store_scalar(381, (p.p853 + s.v[376]));

        s.store_scalar(409, (1.0 - p.p848));

        s.store_scalar(410, (1.0 - p.p849));

        s.store_scalar(411, (1.0 - p.p850));

        s.store_scalar(412, (1.0 / s.v[409]));

        s.store_scalar(413, (1.0 / s.v[410]));

        s.store_scalar(414, (1.0 / s.v[411]));

        s.store_scalar(424, (s.v[756] / p.p842));

        s.store_scalar(425, ((p.p860 * s.v[756]) / p.p843));

        s.store_scalar(426, ((p.p861 * s.v[756]) / p.p844));

        s.store_scalar(427, (1.0 / s.v[424]));

        s.store_scalar(428, (1.0 / s.v[425]));

        s.store_scalar(429, (1.0 / s.v[426]));

        s.store_scalar(430, (1.0 / p.p845));

        s.store_scalar(431, (1.0 / p.p846));

        s.store_scalar(432, (1.0 / p.p847));

        s.store_scalar(445, (1.0 - (1.0 / p.p841)));

        s.store_scalar(449, (1.0 / p.p877));

        s.store_scalar(450, (1.0 / p.p878));

        s.store_scalar(451, (1.0 / p.p879));

        s.b[983] = ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0));
        s.store_scalar(983, if s.b[983] { 1.0 } else { 0.0 });

        if s.b[983] {
            s.store_scalar(474, 1.0);
        }

        if (!s.b[983]) {
            s.store_scalar(474, 0.0);
        }

        s.b[984] = (s.v[474] == 1.0);
        s.store_scalar(984, if s.b[984] { 1.0 } else { 0.0 });

        if s.b[984] {
            s.store_scalar(458, (if ((p.p844 * p.p883) > 1e-18) { (p.p844 * p.p883) } else { 1e-18 }));
        }

        if s.b[984] {
            s.store_scalar(459, (if ((p.p847 * p.p884) > 0.05) { (p.p847 * p.p884) } else { 0.05 }));
        }

        if s.b[984] {
            s.store_scalar(460, (if ((if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) < 0.95) { (if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[984] {
            s.store_scalar(461, (p.p853 * p.p886));
            s.store_offset(463, 461, s.v[376]);
            s.store_sub_from_scalar(468, 1.0, 460);
            s.store_div_from_scalar(469, 1.0, 468);
        }

        s.b[985] = (p.p44 == 0.0);
        s.store_scalar(985, if s.b[985] { 1.0 } else { 0.0 });

        if s.b[985] {
            s.store_scalar(506, p.p842);
            s.store_scalar(507, p.p843);
            s.store_scalar(508, p.p844);
            s.store_scalar(509, p.p845);
            s.store_scalar(510, p.p846);
            s.store_scalar(511, p.p847);
            s.store_scalar(512, p.p848);
            s.store_scalar(513, p.p849);
            s.store_scalar(514, p.p850);
            s.store_scalar(515, p.p851);
            s.store_scalar(516, p.p852);
            s.store_scalar(517, p.p853);
            s.store_scalar(518, p.p854);
            s.store_scalar(519, p.p855);
            s.store_scalar(520, p.p856);
            s.store_scalar(523, p.p857);
            s.store_scalar(524, p.p858);
            s.store_scalar(525, p.p859);
            s.store_scalar(521, p.p860);
            s.store_scalar(522, p.p861);
            s.store_scalar(526, p.p862);
            s.store_scalar(527, p.p863);
            s.store_scalar(528, p.p864);
            s.store_scalar(529, p.p865);
            s.store_scalar(530, p.p866);
            s.store_scalar(531, p.p867);
            s.store_scalar(532, p.p868);
            s.store_scalar(533, p.p869);
            s.store_scalar(534, p.p870);
            s.store_scalar(535, p.p871);
            s.store_scalar(536, p.p872);
            s.store_scalar(537, p.p873);
            s.store_scalar(538, p.p874);
            s.store_scalar(539, p.p875);
            s.store_scalar(540, p.p876);
            s.store_scalar(541, p.p877);
            s.store_scalar(542, p.p878);
            s.store_scalar(543, p.p879);
            s.store_scalar(544, p.p880);
            s.store_scalar(545, p.p881);
            s.store_scalar(546, p.p882);
            s.store_scalar(554, p.p946);
            s.store_scalar(637, p.p889);
            s.store_scalar(638, p.p890);
            s.store_scalar(639, p.p891);
            s.store_scalar(640, p.p892);
            s.store_scalar(547, p.p883);
            s.store_scalar(548, p.p884);
            s.store_scalar(549, p.p885);
            s.store_scalar(550, p.p886);
            s.store_scalar(551, p.p887);
            s.store_scalar(552, p.p888);
        }

        if (!s.b[985]) {
            s.store_scalar(506, p.p893);
            s.store_scalar(507, p.p894);
            s.store_scalar(508, p.p895);
            s.store_scalar(509, p.p896);
            s.store_scalar(510, p.p897);
            s.store_scalar(511, p.p898);
            s.store_scalar(512, p.p899);
            s.store_scalar(513, p.p900);
            s.store_scalar(514, p.p901);
            s.store_scalar(515, p.p902);
            s.store_scalar(516, p.p903);
            s.store_scalar(517, p.p904);
            s.store_scalar(518, p.p905);
            s.store_scalar(519, p.p906);
            s.store_scalar(520, p.p907);
            s.store_scalar(523, p.p908);
            s.store_scalar(524, p.p909);
            s.store_scalar(525, p.p910);
            s.store_scalar(521, p.p911);
            s.store_scalar(522, p.p912);
            s.store_scalar(526, p.p913);
            s.store_scalar(527, p.p914);
            s.store_scalar(528, p.p915);
            s.store_scalar(529, p.p916);
            s.store_scalar(530, p.p917);
            s.store_scalar(531, p.p918);
            s.store_scalar(532, p.p919);
            s.store_scalar(533, p.p920);
            s.store_scalar(534, p.p921);
            s.store_scalar(535, p.p922);
            s.store_scalar(536, p.p923);
            s.store_scalar(537, p.p924);
            s.store_scalar(538, p.p925);
            s.store_scalar(539, p.p926);
            s.store_scalar(540, p.p927);
            s.store_scalar(541, p.p928);
            s.store_scalar(542, p.p929);
            s.store_scalar(543, p.p930);
            s.store_scalar(544, p.p931);
            s.store_scalar(545, p.p932);
            s.store_scalar(546, p.p933);
            s.store_scalar(554, p.p948);
            s.store_scalar(637, p.p940);
            s.store_scalar(638, p.p941);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[985]) {
            s.store_scalar(639, p.p942);
            s.store_scalar(640, p.p943);
            s.store_scalar(547, p.p934);
            s.store_scalar(548, p.p935);
            s.store_scalar(549, p.p936);
            s.store_scalar(550, p.p937);
            s.store_scalar(551, p.p938);
            s.store_scalar(552, p.p939);
        }

        s.store_offset(555, 515, s.v[376]);

        s.store_offset(556, 516, s.v[376]);

        s.store_offset(557, 517, s.v[376]);

        s.store_sub_from_scalar(576, 1.0, 512);

        s.store_sub_from_scalar(577, 1.0, 513);

        s.store_sub_from_scalar(578, 1.0, 514);

        s.store_div_from_scalar(579, 1.0, 576);

        s.store_div_from_scalar(580, 1.0, 577);

        s.store_div_from_scalar(581, 1.0, 578);

        s.store_div_from_scalar(591, s.v[756], 506);

        s.store_div_scaled_inputs_indices(592, 521, s.v[756], 507, 1.0);

        s.store_div_scaled_inputs_indices(593, 522, s.v[756], 508, 1.0);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 593);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar(599, 1.0, 511);

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.store_div_from_scalar(617, 1.0, 543);

        s.b[986] = ((((s.v[547] != 1.0) || (s.v[548] != 1.0)) || (s.v[549] != 1.0)) || (s.v[550] != 1.0));
        s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });

        if s.b[986] {
            s.store_scalar(636, 1.0);
        }

        if (!s.b[986]) {
            s.store_scalar(636, 0.0);
        }

        s.b[987] = (s.v[636] == 1.0);
        s.store_scalar(987, if s.b[987] { 1.0 } else { 0.0 });

        if s.b[987] {
            if ((s.v[508] * s.v[547]) > 1e-18) {
                s.store_mul(621, 508, 547);
            } else {
                s.store_scalar(621, 1e-18);
            }
        }

        if s.b[987] {
            if ((s.v[511] * s.v[548]) > 0.05) {
                s.store_mul(622, 511, 548);
            } else {
                s.store_scalar(622, 0.05);
            }
        }

        if s.b[987] {
            if ((if ((s.v[514] * s.v[549]) > 0.05) { (s.v[514] * s.v[549]) } else { 0.05 }) < 0.95) {
                if ((s.v[514] * s.v[549]) > 0.05) {
                    s.store_mul(623, 514, 549);
                } else {
                    s.store_scalar(623, 0.05);
                }
            } else {
                s.store_scalar(623, 0.95);
            }
        }

        if s.b[987] {
            s.store_mul(624, 517, 550);
            s.store_offset(626, 624, s.v[376]);
            s.store_sub_from_scalar(631, 1.0, 623);
            s.store_div_from_scalar(632, 1.0, 631);
        }

        s.store_scalar(352, ((ctx_temp + p.p55) + p.p35));

        s.store_scalar(353, (s.v[352] / s.v[351]));

        s.store_scalar(354, (s.v[352] - s.v[351]));

        s.store_scalar(355, ((s.v[352] * 1.3806505e-23) / 1.6021918e-19));

        s.store_scalar(356, (1.0 / s.v[355]));

        s.store_scalar(366, (((ctx_temp + p.p55) + p.p35)).max((273.15 + (-250.0))));

        s.store_scalar(367, (s.v[366] / s.v[365]));

        s.store_scalar(371, (s.v[368] * s.v[366]));

        s.store_scalar(372, (1.0 / s.v[371]));

        s.store_scalar(377, ((-((0.000702 * s.v[366]) * s.v[366])) / (1108.0 + s.v[366])));

        s.store_scalar(382, (p.p851 + s.v[377]));

        s.store_scalar(383, (p.p852 + s.v[377]));

        s.store_scalar(384, (p.p853 + s.v[377]));

        s.store_scalar(385, (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[370]) - (s.v[382] * s.v[372])))) as f64).exp()));

        s.store_scalar(386, (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[370]) - (s.v[383] * s.v[372])))) as f64).exp()));

        s.store_scalar(387, (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[381] * s.v[370]) - (s.v[384] * s.v[372])))) as f64).exp()));

        s.store_scalar(388, ((p.p854 * s.v[385]) * s.v[385]));

        s.store_scalar(389, ((p.p855 * s.v[386]) * s.v[386]));

        s.store_scalar(390, ((p.p856 * s.v[387]) * s.v[387]));

        s.store_scalar(391, ((p.p845 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[385]) as f64).ln())));

        s.store_scalar(392, ((p.p846 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[386]) as f64).ln())));

        s.store_scalar(393, ((p.p847 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[387]) as f64).ln())));

        s.store_scalar(394, (s.v[391] + (s.v[371] * (((1.0 + ((((0.05 - s.v[391]) * s.v[372])) as f64).exp())) as f64).ln())));

        s.store_scalar(395, (s.v[392] + (s.v[371] * (((1.0 + ((((0.05 - s.v[392]) * s.v[372])) as f64).exp())) as f64).ln())));

        s.store_scalar(396, (s.v[393] + (s.v[371] * (((1.0 + ((((0.05 - s.v[393]) * s.v[372])) as f64).exp())) as f64).ln())));

        s.store_scalar(406, (1.0 / s.v[394]));

        s.store_scalar(407, (1.0 / s.v[395]));

        s.store_scalar(408, (1.0 / s.v[396]));

        s.store_scalar(415, (p.p842 * (((p.p845 * s.v[406])) as f64).powf(p.p848)));

        s.store_scalar(416, (p.p843 * (((p.p846 * s.v[407])) as f64).powf(p.p849)));

        s.store_scalar(417, (p.p844 * (((p.p847 * s.v[408])) as f64).powf(p.p850)));

        s.store_scalar(418, ((s.v[415] * s.v[394]) * s.v[412]));

        s.store_scalar(419, ((s.v[416] * s.v[395]) * s.v[413]));

        s.store_scalar(420, ((s.v[417] * s.v[396]) * s.v[414]));

        s.store_scalar(421, (2.0 * s.v[415]));

        s.store_scalar(422, (2.0 * s.v[416]));

        s.store_scalar(423, (2.0 * s.v[417]));

        s.store_scalar(433, ((0.5 * s.v[382])).max(s.v[371]));

        s.store_scalar(434, ((0.5 * s.v[383])).max(s.v[371]));

        s.store_scalar(435, ((0.5 * s.v[384])).max(s.v[371]));

        s.store_scalar(436, (s.v[433] * s.v[372]));

        s.store_scalar(437, (s.v[434] * s.v[372]));

        s.store_scalar(438, (s.v[435] * s.v[372]));

        s.store_scalar(439, (((((((32.0 * p.p865) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(440, (((((((32.0 * p.p866) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(441, (((((((32.0 * p.p867) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[435] * s.v[435]) * s.v[435]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(442, (p.p871 * (1.0 + (p.p874 * (s.v[366] - s.v[365])))));

        s.store_scalar(443, (p.p872 * (1.0 + (p.p875 * (s.v[366] - s.v[365])))));

        s.store_scalar(444, (p.p873 * (1.0 + (p.p876 * (s.v[366] - s.v[365])))));

        if (!(s.v[442] > 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (!(s.v[443] > 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (!(s.v[444] > 0.0)) {
            s.store_scalar(444, 0.0);
        }

        s.b[1007] = (s.v[474] == 1.0);
        s.store_scalar(1007, if s.b[1007] { 1.0 } else { 0.0 });

        if s.b[1007] {
            s.store_offset(462, 461, s.v[377]);
            s.store_scale_ad(464, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(463), s.v[370], s.ad_value(462), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(465, 459, s.v[367], A::ln(s.ad_value(464)), (2.0 * s.v[371]));
            s.store_add_scaled_inputs_ad_rhs(466, 465, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(465), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);
            s.store_div_from_scalar(467, 1.0, 466);
            s.store_mul_pow_ad_rhs(470, 458, A::mul(s.ad_value(459), s.ad_value(467)), s.ad_value(460));
            s.store_mul3_lhs(471, 470, 466, 469);
            s.store_scale(472, 470, 2.0);
        }

        s.store_offset(558, 515, s.v[377]);

        s.store_offset(559, 516, s.v[377]);

        s.store_offset(560, 517, s.v[377]);

        s.store_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[370], s.ad_value(558), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[370], s.ad_value(559), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(563, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(557), s.v[370], s.ad_value(560), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_mul3_lhs(564, 518, 561, 561);

        s.store_mul3_lhs(565, 519, 562, 562);

        s.store_mul3_lhs(566, 520, 563, 563);

        s.store_sub_scaled_inputs_ad_rhs(567, 509, s.v[367], A::ln(s.ad_value(561)), (2.0 * s.v[371]));

        s.store_sub_scaled_inputs_ad_rhs(568, 510, s.v[367], A::ln(s.ad_value(562)), (2.0 * s.v[371]));

        s.store_sub_scaled_inputs_ad_rhs(569, 511, s.v[367], A::ln(s.ad_value(563)), (2.0 * s.v[371]));

        s.store_add_scaled_inputs_ad_rhs(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);

        s.store_add_scaled_inputs_ad_rhs(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);

        s.store_add_scaled_inputs_ad_rhs(572, 569, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(569), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(575, 1.0, 572);

        s.store_mul_pow_ad_rhs(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512));

        s.store_mul_pow_ad_rhs(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513));

        s.store_mul_pow_ad_rhs(584, 508, A::mul(s.ad_value(511), s.ad_value(575)), s.ad_value(514));

        s.store_mul3_lhs(585, 582, 570, 579);

        s.store_mul3_lhs(586, 583, 571, 580);

        s.store_mul3_lhs(587, 584, 572, 581);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_scale(590, 584, 2.0);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(602, A::scale(s.ad_value(560), 0.5), s.v[371]);

        s.store_scale(603, 600, s.v[372]);

        s.store_scale(604, 601, s.v[372]);

        s.store_scale(605, 602, s.v[372]);

        s.store_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(608, A::mul3_scaled_output(s.ad_value(531), A::square(s.ad_value(602)), s.ad_value(602), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_scale_offset_rhs(609, 535, 538, (s.v[366] - s.v[365]), 1.0);

        s.store_mul_scale_offset_rhs(610, 536, 539, (s.v[366] - s.v[365]), 1.0);

        s.store_mul_scale_offset_rhs(611, 537, 540, (s.v[366] - s.v[365]), 1.0);

        if (!(s.v[609] > 0.0)) {
            s.store_scalar(609, 0.0);
        }

        if (!(s.v[610] > 0.0)) {
            s.store_scalar(610, 0.0);
        }

        if (!(s.v[611] > 0.0)) {
            s.store_scalar(611, 0.0);
        }

        s.b[1008] = (s.v[636] == 1.0);
        s.store_scalar(1008, if s.b[1008] { 1.0 } else { 0.0 });

        if s.b[1008] {
            s.store_offset(625, 624, s.v[377]);
            s.store_scale_ad(627, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(626), s.v[370], s.ad_value(625), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(628, 622, s.v[367], A::ln(s.ad_value(627)), (2.0 * s.v[371]));
            s.store_add_scaled_inputs_ad_rhs(629, 628, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(628), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);
            s.store_div_from_scalar(630, 1.0, 629);
            s.store_mul_pow_ad_rhs(633, 621, A::mul(s.ad_value(622), s.ad_value(630)), s.ad_value(623));
            s.store_mul3_lhs(634, 633, 629, 632);
            s.store_scale(635, 633, 2.0);
        }

        s.store_scalar(1, 1.0);

        s.store_scalar(2, 1.0);

        s.store_scalar(312, 0.0);

        s.store_scalar(313, 0.0);

        s.store_scalar(3, p.p0);

        s.store_scalar(4, p.p1);

        s.store_scalar(5, p.p2);

        s.store_scalar(6, p.p3);

        s.store_scalar(7, p.p4);

        s.store_scalar(8, p.p8);

        s.store_scalar(647, p.p19);

        s.store_scalar(648, p.p20);

        s.store_scalar(649, p.p21);

        s.store_scalar(674, p.p22);

        s.store_scalar(675, p.p23);

        s.store_scalar(676, p.p24);

        s.store_scalar(650, p.p25);

        s.store_scalar(651, p.p26);

        s.store_scalar(677, p.p27);

        s.store_scalar(678, p.p28);

        s.store_scalar(10, p.p14);

        s.b[1009] = (p.p39 > 0.0);
        s.store_scalar(1009, if s.b[1009] { 1.0 } else { 0.0 });

        if s.b[1009] {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1009] {
            s.store_floor_ad(1, A::offset(s.ad_value(1), 0.5));
            s.store_div_from_scalar(2, 1.0, 1);
        }

        if ((s.v[4] * s.v[2]) > 1e-9) {
            s.store_scale(4, 2, s.v[4]);
        } else {
            s.store_scalar(4, 1e-9);
        }

        s.store_scalar(11, p.p5);

        s.store_scalar(12, p.p6);

        s.store_scalar(13, p.p7);

        s.store_scalar(308, (1e-6 / s.v[3]));

        s.store_div_from_scalar(309, 1e-6, 4);

        s.store_offset_scaled(310, 309, ((p.p191) * ((p.p189 * (1.0 + (p.p190 * s.v[308]))))), (p.p189 * (1.0 + (p.p190 * s.v[308]))));

        s.store_offset_scaled(311, 309, ((p.p195) * ((p.p193 * (1.0 + (p.p194 * s.v[308]))))), (p.p193 * (1.0 + (p.p194 * s.v[308]))));

        if (((s.v[3] + s.v[310]) - (2.0 * p.p192)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[3]) + ((-(2.0 * p.p192)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[4] + s.v[311]) - (2.0 * p.p196)) > 1e-9) {
            s.store_offset_add(313, 4, 311, (-(2.0 * p.p196)));
        } else {
            s.store_scalar(313, 1e-9);
        }

        s.store_div_from_scalar(314, 1e-6, 312);

        s.store_square(315, 314);

        s.store_div_from_scalar(316, 1e-6, 313);

        s.store_div_from_scalar(317, 1.0, 316);

        s.store_mul(318, 314, 316);

        s.store_div_from_scalar(319, 1.0, 318);

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((((s.v[3] + s.v[310]) - (2.0 * p.p192)) + p.p197) > 1e-9) {
            s.store_offset(320, 310, ((((s.v[3]) + ((-(2.0 * p.p192))))) + (p.p197)));
        } else {
            s.store_scalar(320, 1e-9);
        }

        if ((((s.v[4] + s.v[311]) - (2.0 * p.p196)) + p.p198) > 1e-9) {
            s.store_offset_add(321, 4, 311, (((-(2.0 * p.p196))) + (p.p198)));
        } else {
            s.store_scalar(321, 1e-9);
        }

        s.store_scale(322, 321, 1000000.0);

        if (((s.v[3] + s.v[310]) + p.p197) > 1e-9) {
            s.store_offset(323, 310, ((s.v[3]) + (p.p197)));
        } else {
            s.store_scalar(323, 1e-9);
        }

        if (((s.v[4] + s.v[311]) + p.p198) > 1e-9) {
            s.store_offset_add(324, 4, 311, p.p198);
        } else {
            s.store_scalar(324, 1e-9);
        }

        s.store_scale(325, 323, 1000000.0);

        s.store_scale(326, 324, 1000000.0);

        s.store_scalar(40, p.p56);

        s.store_scalar(41, p.p57);

        s.store_scalar(42, p.p58);

        s.store_scalar(43, p.p59);

        s.store_scalar(44, p.p60);

        s.store_scalar(45, p.p61);

        s.store_scalar(46, p.p62);

        s.store_scalar(47, p.p63);

        s.store_scalar(48, p.p64);

        s.store_scalar(49, p.p65);

        s.store_scalar(50, p.p66);

        s.store_scalar(55, p.p67);

        s.store_scalar(56, p.p68);

        s.store_scalar(57, p.p69);

        s.store_scalar(58, p.p70);

        s.store_scalar(51, p.p71);

        s.store_scalar(52, p.p73);

        s.store_scalar(53, p.p72);

        s.store_scalar(54, p.p74);

        s.store_scalar(59, p.p78);

        s.store_scalar(60, p.p80);

        s.store_scalar(61, p.p79);

        s.store_scalar(62, p.p75);

        s.store_scalar(63, p.p77);

        s.store_scalar(64, p.p76);

        s.store_scalar(65, p.p81);

        s.store_scalar(66, p.p82);

        s.store_scalar(67, p.p83);

        s.store_scalar(68, p.p84);

        s.store_scalar(69, p.p85);

        s.store_scalar(70, p.p86);

        s.store_scalar(71, p.p87);

        s.store_scalar(72, p.p88);

        s.store_scalar(73, p.p89);

        s.store_scalar(74, p.p90);

        s.store_scalar(75, p.p91);

        s.store_scalar(76, p.p92);

        s.store_scalar(77, p.p93);

        s.store_scalar(78, p.p94);

        s.store_scalar(79, p.p95);

        s.store_scalar(80, p.p96);

        s.store_scalar(81, p.p97);

        s.store_scalar(82, p.p98);

        s.store_scalar(83, p.p99);

        s.store_scalar(84, p.p100);

        s.store_scalar(85, p.p101);

        s.store_scalar(86, p.p102);

        s.store_scalar(87, p.p103);

        s.store_scalar(88, p.p104);

        s.store_scalar(89, p.p105);

        s.store_scalar(90, p.p106);

        s.store_scalar(91, p.p107);

        s.store_scalar(92, p.p108);

        s.store_scalar(93, p.p109);

        s.store_scalar(94, p.p110);

        s.store_scalar(95, p.p111);

        s.store_scalar(96, p.p112);

        s.store_scalar(97, p.p113);

        s.store_scalar(98, p.p114);

        s.store_scalar(99, p.p115);

        s.store_scalar(100, p.p116);

        s.store_scalar(101, p.p117);

        s.store_scalar(102, p.p118);

        s.store_scalar(103, p.p119);

        s.store_scalar(104, p.p120);

        s.store_scalar(105, p.p119);

        s.b[1010] = param_given[121];
        s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });

        if s.b[1010] {
            s.store_scalar(105, p.p121);
        }

        s.store_scalar(106, p.p120);

        s.b[1011] = param_given[122];
        s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });

        if s.b[1011] {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.b[1012] = param_given[123];
        s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });

        if s.b[1012] {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.b[1013] = param_given[124];
        s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });

        if s.b[1013] {
            s.store_scalar(108, p.p124);
        }

        s.store_scalar(109, p.p125);

        s.store_scalar(110, p.p126);

        s.store_scalar(111, p.p127);

        s.store_scalar(112, p.p128);

        s.store_scalar(113, p.p129);

        s.store_scalar(114, p.p130);

        s.store_scalar(115, p.p131);

        s.store_scalar(116, p.p132);

        s.store_scalar(117, p.p133);

        s.store_scalar(118, p.p134);

        s.store_scalar(119, p.p135);

        s.store_scalar(120, p.p136);

        s.store_scalar(121, p.p98);

        s.b[1014] = param_given[137];
        s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });

        if s.b[1014] {
            s.store_scalar(121, p.p137);
        }

        s.store_scalar(122, p.p103);

        s.b[1015] = param_given[138];
        s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });

        if s.b[1015] {
            s.store_scalar(122, p.p138);
        }

        s.store_scalar(123, p.p139);

        s.store_scalar(124, p.p140);

        s.store_scalar(125, p.p141);

        s.store_scalar(126, p.p142);

        s.store_scalar(127, p.p143);

        s.store_scalar(128, p.p144);

        s.store_scalar(129, p.p145);

        s.store_scalar(130, p.p146);

        s.store_scalar(131, p.p147);

        s.store_scalar(132, p.p148);

        s.store_scalar(133, p.p149);

        s.store_scalar(134, p.p150);

        s.store_scalar(135, p.p151);

        s.store_scalar(136, p.p152);

        s.store_scalar(137, p.p153);

        s.store_scalar(138, p.p154);

        s.store_scalar(139, p.p155);

        s.store_scalar(145, p.p161);

        s.store_scalar(146, p.p162);

        s.store_scalar(147, p.p163);

        s.store_scalar(148, p.p164);

        s.store_scalar(149, p.p165);

        s.store_scalar(150, p.p166);

        s.store_scalar(151, p.p167);

        s.store_scalar(152, p.p168);

        s.store_scalar(153, p.p169);

        s.store_scalar(154, p.p170);

        s.store_scalar(155, p.p171);

        s.store_scalar(156, p.p173);

        s.store_scalar(157, p.p172);

        s.store_scalar(173, p.p187);

        s.b[1016] = (p.p39 > 0.0);
        s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });

        if s.b[1016] {
            s.store_add_scaled_inputs3_offset_mixed_aii(40, A::powf(s.ad_value(314), p.p201), p.p200, 316, p.p202, 318, p.p203, p.p199);
            s.store_add_scaled_inputs3_offset_indices(41, 314, p.p205, 316, p.p206, 318, p.p207, p.p204);
            s.store_scalar(42, p.p208);
            s.store_scalar(43, p.p209);
            s.store_scalar(44, p.p210);
        }

        if s.b[1016] {
            s.store_scale_ad(331, {
                if ((1.0 + ((p.p212 * s.v[316]) * (((1.0 + (s.v[313] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p212, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211);
        }

        if s.b[1016] {
            s.store_scale_ad(332, {
                if ((1.0 + ((p.p215 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p215, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p216), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214);
        }

        if s.b[1016] {
            s.store_scale_ad(333, {
                if ((1.0 + ((p.p218 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p218, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p216), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p217);
        }

        s.b[1017] = (s.v[312] > (2.0 * s.v[333]));
        s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1017]) {
            s.store_scalar(334, 75000000000.0);
            s.store_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));
            s.store_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);
            s.store_square(336, 336);
        }

        s.b[1018] = (s.v[312] >= s.v[333]);
        s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });

        if ((s.b[1016] && (!s.b[1017])) && s.b[1018]) {
            s.store_add_ad_rhs(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));
        }

        if ((s.b[1016] && (!s.b[1017])) && (!s.b[1018])) {
            s.store_add_ad_rhs(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));
        }

        if s.b[1016] {
            s.store_mul_sub_scaled_inputs_rhs(45, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p219)), 1.0, s.ad_value(315), p.p220);
            s.store_add_scaled_inputs3_offset_mixed_aii(46, A::powf(s.ad_value(314), p.p223), p.p222, 316, p.p224, 318, p.p225, p.p221);
            s.store_scalar(47, p.p226);
            s.store_scalar(48, p.p227);
            s.store_add_scaled_inputs3_offset_mixed_aii(49, A::powf(s.ad_value(314), p.p230), p.p229, 316, p.p231, 318, p.p232, p.p228);
        }

        if s.b[1016] {
            s.store_scale_ad(50, {
                if (1e-6 > (1.0 + (p.p234 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p.p234, 1.0)
                }
            }, p.p233);
        }

        if s.b[1016] {
            s.store_scalar(55, p.p235);
            s.store_scalar(56, p.p236);
            s.store_scalar(57, p.p239);
            s.store_scalar(58, p.p240);
            s.store_mul3_ad(51, A::scale_offset(A::powf(s.ad_value(314), p.p243), p.p242, p.p241), A::scale_offset(s.ad_value(316), p.p244, 1.0), A::scale_offset(s.ad_value(318), p.p245, 1.0));
            s.store_scalar(52, p.p247);
            s.store_scalar(53, p.p246);
            s.store_scalar(54, p.p248);
            s.store_scaled_mul_scale_offset_rhs_ad(62, A::powf(s.ad_value(314), p.p250), 316, p.p251, 1.0, p.p249);
            s.store_scalar(63, p.p253);
            s.store_scalar(64, p.p252);
            s.store_scaled_mul_scale_offset_rhs_ad(59, A::powf(s.ad_value(314), p.p255), 316, p.p256, 1.0, p.p254);
            s.store_scalar(60, p.p258);
            s.store_scalar(61, p.p257);
            s.store_offset_scaled(337, 316, ((p.p261) * (p.p260)), p.p260);
        }

        if s.b[1016] {
            s.store_scale_ad(338, {
                if ((1.0 + (p.p263 * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p.p263, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p262);
        }

        if s.b[1016] {
            s.store_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p264 * p.p265), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p265)))));
        }

        if s.b[1016] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }

        if s.b[1016] {
            s.store_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p.p266, 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p268), 1.0)), p.p267);
            s.store_mul_div_scaled_inputs_mixed_iia(65, 340, 313, p.p259, A::mul(s.ad_value(339), s.ad_value(312)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(66, 314, p.p270, 316, p.p271, 318, p.p272, p.p269);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1016] {
            s.store_offset_scaled(67, 316, ((p.p274) * (p.p273)), p.p273);
            s.store_scalar(68, p.p275);
            s.store_scalar(69, p.p276);
            s.store_scalar(70, p.p277);
            s.store_mul3_ad(71, A::scale_offset(A::powf(s.ad_value(314), p.p280), p.p279, p.p278), A::scale_offset(s.ad_value(316), p.p281, 1.0), A::scale_offset(s.ad_value(318), p.p282, 1.0));
            s.store_scalar(72, p.p283);
            s.store_scalar(73, p.p284);
            s.store_scalar(74, p.p285);
            s.store_mul3_ad_scaled_output(75, A::scale_offset(s.ad_value(314), p.p287, 1.0), A::scale_offset(s.ad_value(316), p.p288, 1.0), A::scale_offset(s.ad_value(318), p.p289, 1.0), p.p286);
            s.store_scalar(76, p.p290);
            s.store_scalar(77, p.p291);
            s.store_mul_scale_offset_rhs(78, 316, 316, ((p.p293) * (p.p292)), p.p292);
            s.store_scalar(79, p.p294);
            s.store_scalar(80, p.p295);
            s.store_scalar(81, p.p296);
            s.store_mul3_ad(82, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p.p298, s.ad_value(339), 1.0), A::powf(s.ad_value(314), p.p299)), p.p297), A::scale_offset(s.ad_value(316), p.p300, 1.0), A::scale_offset(s.ad_value(318), p.p301, 1.0));
            s.store_add_scaled_inputs3_offset_indices(83, 314, p.p303, 316, p.p304, 318, p.p305, p.p302);
            s.store_scalar(84, p.p306);
            s.store_scalar(85, p.p307);
            s.store_scalar(86, p.p308);
            s.store_div_from_scalar_offset_scaled_input(87, p.p309, 314, p.p310, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(88, A::powf(s.ad_value(314), p.p312), 316, p.p313, 1.0, p.p311);
            s.store_powf(341, 314, p.p315);
            s.store_div_scaled_product_offset_denominator(89, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p317, 1.0), p.p314, A::mul_scaled_lhs(s.ad_value(314), p.p316, s.ad_value(341)), 1.0, 1.0);
            s.store_powf(341, 314, p.p319);
            s.store_div_scaled_product_offset_denominator(90, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p321, 1.0), p.p318, A::mul_scaled_lhs(s.ad_value(314), p.p320, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(91, p.p322);
            s.store_scaled_mul_scale_offset_inputs(92, 314, p.p324, 1.0, 316, p.p325, 1.0, p.p323);
            s.store_scalar(93, p.p326);
            s.store_scalar(94, p.p327);
            s.store_scaled_mul_scale_offset_inputs(95, 314, p.p329, 1.0, 316, p.p330, 1.0, p.p328);
            s.store_scaled_mul_scale_offset_inputs(96, 314, p.p332, 1.0, 316, p.p333, 1.0, p.p331);
            s.store_scalar(97, p.p334);
            s.store_scalar(98, p.p335);
            s.store_div_from_scalar(99, p.p336, 318);
            s.store_div_from_scalar_scaled_input(100, (p.p337 * p.p237), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(101, (p.p338 * p.p238), 316, 1e-6);
            s.store_scalar(102, p.p339);
            s.store_scalar(103, p.p340);
            s.store_scalar(104, p.p341);
            s.store_scalar(105, p.p340);
        }

        s.b[1019] = param_given[342];
        s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1019]) {
            s.store_scalar(105, p.p342);
        }

        if s.b[1016] {
            s.store_scalar(106, p.p341);
        }

        s.b[1020] = param_given[343];
        s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1020]) {
            s.store_scalar(106, p.p343);
        }

        if s.b[1016] {
            s.copy_ad(107, 105);
        }

        s.b[1021] = param_given[344];
        s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1021]) {
            s.store_scalar(107, p.p344);
        }

        if s.b[1016] {
            s.copy_ad(108, 106);
        }

        s.b[1022] = param_given[345];
        s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1022]) {
            s.store_scalar(108, p.p345);
        }

        if s.b[1016] {
            s.store_scalar(109, p.p346);
            s.store_div_from_scalar_scaled_input(110, (p.p347 * p.p237), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(111, (p.p348 * p.p238), 316, 1e-6);
            s.store_scalar(112, p.p349);
            s.store_scalar(113, p.p350);
            s.store_scalar(114, p.p351);
            s.store_scalar(115, p.p352);
            s.store_scalar(116, p.p353);
            s.store_scalar(117, p.p354);
            s.store_scaled_mul(118, 321, 320, ((8.8541878176e-12 * p.p210) * 1.0 / (p.p209)));
            s.store_scale(125, 321, ((8.8541878176e-12 * p.p210) * (p.p237 * 1.0 / (p.p235))));
            s.store_scale(126, 321, ((8.8541878176e-12 * p.p210) * (p.p238 * 1.0 / (p.p236))));
            s.store_add_scaled_inputs3_offset_mixed_aii(119, A::powf(s.ad_value(314), p.p357), p.p356, 316, p.p358, 318, p.p359, p.p355);
            s.store_add_scaled_inputs3_offset_indices(120, 314, p.p361, 316, p.p362, 318, p.p363, p.p360);
            s.store_scalar(32, p.p297);
        }

        s.b[1023] = param_given[364];
        s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1023]) {
            s.store_scalar(32, p.p364);
        }

        if s.b[1016] {
            s.store_scalar(33, p.p298);
        }

        s.b[1024] = param_given[365];
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1024]) {
            s.store_scalar(33, p.p365);
        }

        if s.b[1016] {
            s.store_scalar(34, p.p299);
        }

        s.b[1025] = param_given[366];
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1025]) {
            s.store_scalar(34, p.p366);
        }

        if s.b[1016] {
            s.store_scalar(35, p.p300);
        }

        s.b[1026] = param_given[367];
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1026]) {
            s.store_scalar(35, p.p367);
        }

        if s.b[1016] {
            s.store_scalar(36, p.p301);
        }

        s.b[1027] = param_given[368];
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1027]) {
            s.store_scalar(36, p.p368);
        }

        if s.b[1016] {
            s.store_mul3_ad(121, A::add_scaled_product(s.ad_value(32), 1.0, A::div_scaled_product(s.ad_value(33), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(34)), 1.0), A::offset(A::mul(s.ad_value(35), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(36), s.ad_value(318)), 1.0));
            s.store_scalar(37, p.p309);
        }

        s.b[1028] = param_given[369];
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1028]) {
            s.store_scalar(37, p.p369);
        }

        if s.b[1016] {
            s.store_scalar(38, p.p310);
        }

        s.b[1029] = param_given[370];
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1029]) {
            s.store_scalar(38, p.p370);
        }

        if s.b[1016] {
            s.store_div_scaled_value_offset_denominator(122, s.ad_value(37), 1.0, A::mul(s.ad_value(38), s.ad_value(314)), 1.0, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(123, A::powf(s.ad_value(314), p.p372), 316, p.p373, 1.0, p.p371);
            s.store_powf(341, 314, p.p375);
            s.store_div_scaled_product_offset_denominator(124, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p377, 1.0), p.p374, A::mul_scaled_lhs(s.ad_value(314), p.p376, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(127, p.p378);
            s.store_scalar(128, p.p379);
            s.store_scalar(129, p.p380);
            s.store_scale(130, 325, p.p381);
            s.store_scale(131, 322, p.p382);
            s.store_scale(132, 322, p.p383);
            s.store_scalar(133, p.p384);
            s.store_scalar(134, p.p385);
            s.store_scalar(135, p.p386);
            s.store_scalar(136, p.p387);
            s.store_scale(137, 326, p.p388);
            s.store_scale(138, 326, p.p389);
            s.store_sub_from_scalar_ad(998, 1.0, A::div_from_scalar((2.0 * p.p396), s.ad_value(312)));
            s.store_scalar(139, p.p390);
            s.store_offset_scaled(344, 313, p.p399, (2.0 * p.p398));
            s.store_scalar(145, p.p400);
            s.store_add_scaled_inputs3_offset_indices(146, 314, p.p402, 316, p.p403, 318, p.p404, p.p401);
            s.store_add_scaled_inputs3_offset_mixed_aii(147, A::powf(s.ad_value(314), p.p407), p.p406, 316, p.p408, 318, p.p409, p.p405);
            s.store_mul3_ad_scaled_output(148, A::scale_offset(A::powf(s.ad_value(314), p.p412), p.p411, 1.0), A::scale_offset(s.ad_value(316), p.p413, 1.0), A::scale_offset(s.ad_value(318), p.p414, 1.0), p.p410);
            s.store_offset_scaled_ad(149, A::powf(s.ad_value(314), p.p417), p.p416, p.p415);
            s.store_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p418 * p.p419), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p419)))), 1.0);
        }

        if s.b[1016] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }

        if s.b[1016] {
            s.store_mul_div_scaled_inputs_mixed_aia(150, A::scale_offset(s.ad_value(316), p.p420, 1.0), 344, p.p259, A::mul(s.ad_value(347), s.ad_value(312)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(151, 314, p.p422, 316, p.p423, 318, p.p424, p.p421);
            s.store_scaled_mul_scale_offset_rhs_ad(152, A::powf(s.ad_value(314), p.p426), 316, p.p427, 1.0, p.p425);
            s.store_scalar(153, p.p428);
            s.store_scalar(154, p.p429);
            s.store_scaled_mul_scale_offset_rhs_ad(155, A::powf(s.ad_value(314), p.p431), 316, p.p432, 1.0, p.p430);
            s.store_scalar(156, p.p434);
            s.store_scalar(157, p.p433);
            s.store_add_scaled_inputs3_offset_indices(348, 314, p.p832, 316, p.p833, 318, p.p834, p.p831);
            s.store_add_scaled_inputs3_offset_indices(349, 314, p.p836, 316, p.p837, 318, p.p838, p.p835);
            s.store_offset_div_scaled_offset_numerator(173, A::div_from_scalar(p.p458, s.ad_value(314)), p.p456, (((1.0) + (p.p457)) * p.p456), s.ad_value(316), 1.0, p.p455);
        }

        s.b[1031] = (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]);
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1031]) {
            s.store_add_scaled_inputs3_offset_indices(40, 314, p.p461, 316, p.p462, 318, p.p463, p.p460);
        }

        s.b[1032] = (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]);
        s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1032]) {
            s.store_add_scaled_inputs3_offset_indices(41, 314, p.p465, 316, p.p466, 318, p.p467, p.p464);
        }

        s.b[1033] = (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]);
        s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1016] && s.b[1033]) {
            s.store_add_scaled_inputs3_offset_indices(45, 314, p.p469, 316, p.p470, 318, p.p471, p.p468);
        }

        s.b[1034] = (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]);
        s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1034]) {
            s.store_add_scaled_inputs3_offset_indices(46, 314, p.p473, 316, p.p474, 318, p.p475, p.p472);
        }

        s.b[1035] = (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]);
        s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1035]) {
            s.store_add_scaled_inputs3_offset_indices(47, 314, p.p477, 316, p.p478, 318, p.p479, p.p476);
        }

        s.b[1036] = (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]);
        s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1036]) {
            s.store_add_scaled_inputs3_offset_indices(49, 314, p.p481, 316, p.p482, 318, p.p483, p.p480);
        }

        s.b[1037] = (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]);
        s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1037]) {
            s.store_add_scaled_inputs3_offset_indices(50, 314, p.p485, 316, p.p486, 318, p.p487, p.p484);
        }

        s.b[1038] = (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]);
        s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1038]) {
            s.store_add_scaled_inputs3_offset_indices(57, 314, p.p489, 316, p.p490, 318, p.p491, p.p488);
        }

        s.b[1039] = (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]);
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1039]) {
            s.store_add_scaled_inputs3_offset_indices(58, 314, p.p493, 316, p.p494, 318, p.p495, p.p492);
        }

        s.b[1040] = (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]);
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1040]) {
            s.store_add_scaled_inputs3_offset_indices(51, 314, p.p497, 316, p.p498, 318, p.p499, p.p496);
        }

        s.b[1041] = (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]);
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1041]) {
            s.store_add_scaled_inputs3_offset_indices(52, 314, p.p505, 316, p.p506, 318, p.p507, p.p504);
        }

        s.b[1042] = (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]);
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1042]) {
            s.store_add_scaled_inputs3_offset_indices(53, 314, p.p501, 316, p.p502, 318, p.p503, p.p500);
        }

        s.b[1043] = (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]);
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1043]) {
            s.store_add_scaled_inputs3_offset_indices(54, 314, p.p509, 316, p.p510, 318, p.p511, p.p508);
        }

        s.b[1044] = (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]);
        s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1044]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(62, 315, s.ad_value(314), p.p513, s.ad_value(316), p.p514, s.ad_value(318), p.p515, p.p512);
        }

        s.b[1045] = (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]);
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset_indices(63, 314, p.p521, 316, p.p522, 318, p.p523, p.p520);
        }

        s.b[1046] = (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset_indices(64, 314, p.p517, 316, p.p518, 318, p.p519, p.p516);
        }

        s.b[1047] = (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1047]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(59, 315, s.ad_value(314), p.p525, s.ad_value(316), p.p526, s.ad_value(318), p.p527, p.p524);
        }

        s.b[1048] = (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]);
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset_indices(60, 314, p.p533, 316, p.p534, 318, p.p535, p.p532);
        }

        s.b[1049] = (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]);
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset_indices(61, 314, p.p529, 316, p.p530, 318, p.p531, p.p528);
        }

        s.b[1050] = (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]);
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1050]) {
            s.store_mul_div_scaled_inputs_mixed_aii(65, A::add_scaled_inputs3_offset(s.ad_value(314), p.p537, s.ad_value(316), p.p538, s.ad_value(318), p.p539, p.p536), 313, 1.0, 312, 1.0);
        }

        s.b[1051] = (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset_indices(66, 314, p.p541, 316, p.p542, 318, p.p543, p.p540);
        }

        s.b[1052] = (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]);
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset_indices(67, 314, p.p545, 316, p.p546, 318, p.p547, p.p544);
        }

        s.b[1053] = (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]);
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1053]) {
            s.store_add_scaled_inputs3_offset_indices(69, 314, p.p549, 316, p.p550, 318, p.p551, p.p548);
        }

        s.b[1054] = (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset_indices(71, 314, p.p553, 316, p.p554, 318, p.p555, p.p552);
        }

        s.b[1055] = (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]);
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset_indices(73, 314, p.p557, 316, p.p558, 318, p.p559, p.p556);
        }

        s.b[1056] = (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]);
        s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset_indices(75, 314, p.p561, 316, p.p562, 318, p.p563, p.p560);
        }

        s.b[1057] = (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]);
        s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1057]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(78, 316, s.ad_value(314), p.p565, s.ad_value(316), p.p566, s.ad_value(318), p.p567, p.p564);
        }

        s.b[1058] = (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]);
        s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1058]) {
            s.store_add_scaled_inputs3_offset_indices(79, 314, p.p569, 316, p.p570, 318, p.p571, p.p568);
        }

        s.b[1059] = (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]);
        s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset_indices(80, 314, p.p573, 316, p.p574, 318, p.p575, p.p572);
        }

        s.b[1060] = (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]);
        s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1060]) {
            s.store_add_scaled_inputs3_offset_indices(81, 314, p.p577, 316, p.p578, 318, p.p579, p.p576);
        }

        s.b[1061] = (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]);
        s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1061]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(82, 314, s.ad_value(314), p.p581, s.ad_value(316), p.p582, s.ad_value(318), p.p583, p.p580);
        }

        s.b[1062] = (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]);
        s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset_indices(83, 314, p.p585, 316, p.p586, 318, p.p587, p.p584);
        }

        s.b[1063] = (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]);
        s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset_indices(84, 314, p.p589, 316, p.p590, 318, p.p591, p.p588);
        }

        s.b[1064] = (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]);
        s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1064]) {
            s.store_add_scaled_inputs3_offset_indices(85, 314, p.p593, 316, p.p594, 318, p.p595, p.p592);
        }

        s.b[1065] = (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]);
        s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset_indices(87, 314, p.p597, 316, p.p598, 318, p.p599, p.p596);
        }

        s.b[1066] = (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]);
        s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1066]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(88, 314, s.ad_value(314), p.p601, s.ad_value(316), p.p602, s.ad_value(318), p.p603, p.p600);
        }

        s.b[1067] = (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]);
        s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset_indices(89, 314, p.p605, 316, p.p606, 318, p.p607, p.p604);
        }

        s.b[1068] = (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]);
        s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset_indices(90, 314, p.p609, 316, p.p610, 318, p.p611, p.p608);
        }

        s.b[1069] = (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]);
        s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1069]) {
            s.store_add_scaled_inputs3_offset_indices(92, 314, p.p613, 316, p.p614, 318, p.p615, p.p612);
        }

        s.b[1070] = (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]);
        s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset_indices(94, 314, p.p617, 316, p.p618, 318, p.p619, p.p616);
        }

        s.b[1071] = (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]);
        s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1071]) {
            s.store_add_scaled_inputs3_offset_indices(95, 314, p.p621, 316, p.p622, 318, p.p623, p.p620);
        }

        s.b[1072] = (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]);
        s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset_indices(96, 314, p.p625, 316, p.p626, 318, p.p627, p.p624);
        }

        s.b[1073] = (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]);
        s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1073]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(99, 319, s.ad_value(314), p.p629, s.ad_value(316), p.p630, s.ad_value(318), p.p631, p.p628);
        }

        s.b[1074] = (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]);
        s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1074]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(100, 317, s.ad_value(314), p.p633, s.ad_value(316), p.p634, s.ad_value(318), p.p635, p.p632);
        }

        s.b[1075] = (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]);
        s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1075]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(101, 317, s.ad_value(314), p.p637, s.ad_value(316), p.p638, s.ad_value(318), p.p639, p.p636);
        }

        s.b[1076] = (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]);
        s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1076]) {
            s.store_add_scaled_inputs3_offset_indices(102, 314, p.p641, 316, p.p642, 318, p.p643, p.p640);
        }

        s.b[1077] = (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]);
        s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1077]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(110, 317, s.ad_value(314), p.p645, s.ad_value(316), p.p646, s.ad_value(318), p.p647, p.p644);
        }

        s.b[1078] = (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]);
        s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1078]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(111, 317, s.ad_value(314), p.p649, s.ad_value(316), p.p650, s.ad_value(318), p.p651, p.p648);
        }

        s.b[1079] = (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]);
        s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset_indices(114, 314, p.p653, 316, p.p654, 318, p.p655, p.p652);
        }

        s.b[1080] = (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]);
        s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1080]) {
            s.store_add_scaled_inputs3_offset_indices(115, 314, p.p657, 316, p.p658, 318, p.p659, p.p656);
        }

        s.b[1081] = (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]);
        s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1081]) {
            s.store_mul_ad_affine_product_rhs(118, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p661, s.ad_value(316), p.p662, s.ad_value(318), p.p663, p.p660), 1.0 / (1e-6), 0.0);
        }

        s.b[1082] = (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]);
        s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset_indices(119, 314, p.p665, 316, p.p666, 318, p.p667, p.p664);
        }

        s.b[1083] = (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]);
        s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset_indices(120, 314, p.p669, 316, p.p670, 318, p.p671, p.p668);
        }

        s.b[1084] = (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]);
        s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(28, p.p580);
        }

        s.b[1085] = param_given[672];
        s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1084]) && s.b[1085]) {
            s.store_scalar(28, p.p672);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(29, p.p581);
        }

        s.b[1086] = param_given[673];
        s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1084]) && s.b[1086]) {
            s.store_scalar(29, p.p673);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(30, p.p582);
        }

        s.b[1087] = param_given[674];
        s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1084]) && s.b[1087]) {
            s.store_scalar(30, p.p674);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(31, p.p583);
        }

        s.b[1088] = param_given[675];
        s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1084]) && s.b[1088]) {
            s.store_scalar(31, p.p675);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_mul_ad_rhs(121, 314, A::add_scaled_value_products3(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(314), 1.0, s.ad_value(30), s.ad_value(316), 1.0, s.ad_value(31), s.ad_value(318), 1.0));
        }

        s.b[1089] = (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]);
        s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(28, p.p596);
        }

        s.b[1090] = param_given[676];
        s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1089]) && s.b[1090]) {
            s.store_scalar(28, p.p676);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(29, p.p597);
        }

        s.b[1091] = param_given[677];
        s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1089]) && s.b[1091]) {
            s.store_scalar(29, p.p677);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(30, p.p598);
        }

        s.b[1092] = param_given[678];
        s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1089]) && s.b[1092]) {
            s.store_scalar(30, p.p678);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(31, p.p599);
        }

        s.b[1093] = param_given[679];
        s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1089]) && s.b[1093]) {
            s.store_scalar(31, p.p679);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_add_scaled_value_products3_indices(122, 28, 1.0, 29, 314, 1.0, 30, 316, 1.0, 31, 318, 1.0);
        }

        s.b[1094] = (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]);
        s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1094]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(123, 314, s.ad_value(314), p.p681, s.ad_value(316), p.p682, s.ad_value(318), p.p683, p.p680);
        }

        s.b[1095] = (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]);
        s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1095]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(124, 314, s.ad_value(314), p.p685, s.ad_value(316), p.p686, s.ad_value(318), p.p687, p.p684);
        }

        s.b[1096] = (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]);
        s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1096]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(125, 322, s.ad_value(314), p.p689, s.ad_value(316), p.p690, s.ad_value(318), p.p691, p.p688);
        }

        s.b[1097] = (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]);
        s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1097]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(126, 322, s.ad_value(314), p.p693, s.ad_value(316), p.p694, s.ad_value(318), p.p695, p.p692);
        }

        s.b[1098] = (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]);
        s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1098]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(130, 325, s.ad_value(314), p.p697, s.ad_value(316), p.p698, s.ad_value(318), p.p699, p.p696);
        }

        s.b[1099] = (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]);
        s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1099]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(131, 322, s.ad_value(314), p.p701, s.ad_value(316), p.p702, s.ad_value(318), p.p703, p.p700);
        }

        s.b[1100] = (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]);
        s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1100]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(132, 322, s.ad_value(314), p.p705, s.ad_value(316), p.p706, s.ad_value(318), p.p707, p.p704);
        }

        s.b[1101] = (((param_given[708] || param_given[709]) || param_given[710]) || param_given[711]);
        s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1101]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 326, s.ad_value(314), p.p709, s.ad_value(316), p.p710, s.ad_value(318), p.p711, p.p708);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1102] = (((param_given[712] || param_given[713]) || param_given[714]) || param_given[715]);
        s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1102]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(138, 326, s.ad_value(314), p.p713, s.ad_value(316), p.p714, s.ad_value(318), p.p715, p.p712);
        }

        s.b[1107] = (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]);
        s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1107]) {
            s.store_add_scaled_inputs3_offset_indices(145, 314, p.p733, 316, p.p734, 318, p.p735, p.p732);
        }

        s.b[1108] = (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]);
        s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1108]) {
            s.store_add_scaled_inputs3_offset_indices(146, 314, p.p737, 316, p.p738, 318, p.p739, p.p736);
        }

        s.b[1109] = (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]);
        s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1109]) {
            s.store_add_scaled_inputs3_offset_indices(147, 314, p.p741, 316, p.p742, 318, p.p743, p.p740);
        }

        s.b[1110] = (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]);
        s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1110]) {
            s.store_add_scaled_inputs3_offset_indices(148, 314, p.p745, 316, p.p746, 318, p.p747, p.p744);
        }

        s.b[1111] = (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]);
        s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1111]) {
            s.store_add_scaled_inputs3_offset_indices(149, 314, p.p749, 316, p.p750, 318, p.p751, p.p748);
        }

        s.b[1112] = (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]);
        s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1112]) {
            s.store_mul_div_scaled_inputs_mixed_aii(150, A::add_scaled_inputs3_offset(s.ad_value(314), p.p753, s.ad_value(316), p.p754, s.ad_value(318), p.p755, p.p752), 344, 1.0, 312, 1.0);
        }

        s.b[1113] = (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1113]) {
            s.store_add_scaled_inputs3_offset_indices(151, 314, p.p757, 316, p.p758, 318, p.p759, p.p756);
        }

        s.b[1114] = (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1114]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(152, 315, s.ad_value(314), p.p761, s.ad_value(316), p.p762, s.ad_value(318), p.p763, p.p760);
        }

        s.b[1115] = (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]);
        s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1115]) {
            s.store_add_scaled_inputs3_offset_indices(153, 314, p.p765, 316, p.p766, 318, p.p767, p.p764);
        }

        s.b[1116] = (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]);
        s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1116]) {
            s.store_add_scaled_inputs3_offset_indices(154, 314, p.p769, 316, p.p770, 318, p.p771, p.p768);
        }

        s.b[1117] = (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]);
        s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1117]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(155, 315, s.ad_value(314), p.p773, s.ad_value(316), p.p774, s.ad_value(318), p.p775, p.p772);
        }

        s.b[1118] = (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]);
        s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1118]) {
            s.store_add_scaled_inputs3_offset_indices(156, 314, p.p781, 316, p.p782, 318, p.p783, p.p780);
        }

        s.b[1119] = (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]);
        s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1119]) {
            s.store_add_scaled_inputs3_offset_indices(157, 314, p.p777, 316, p.p778, 318, p.p779, p.p776);
        }

        s.b[1124] = (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]);
        s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1124]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(173, 319, s.ad_value(314), p.p801, s.ad_value(316), p.p802, s.ad_value(318), p.p803, p.p800);
        }

        if s.b[1016] {
            s.store_scalar(1005, 0.0);
            s.store_scalar(1006, 0.0);
            s.store_scalar(1004, 0.0);
            s.store_scalar(39, p.p812);
        }

        s.b[1126] = param_given[813];
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if (s.b[1016] && s.b[1126]) {
            s.store_scalar(39, p.p813);
        }

        s.b[1127] = (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0))));
        s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });

        let mut assign9190_loop_guard: usize = 0;
        while {
            let assign9190_cond_e9116: f64 = (s.v[1] - 0.5);
            let assign9190_cond_e9118: f64 = if ((s.b[1016] && s.b[1127]) && (s.v[1004] < assign9190_cond_e9116)) { 1.0 } else { 0.0 };
            assign9190_cond_e9118 != 0.0
        } {
            assign9190_loop_guard += 1;
            assert!(assign9190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1016] && s.b[1127]) {
                s.store_add_ad_rhs(1005, 1005, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1004), (s.v[7] + s.v[3]), (s.v[5] + (0.5 * s.v[3])))));
                s.store_add_ad_rhs(1006, 1006, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1004), (s.v[7] + s.v[3]), (s.v[6] + (0.5 * s.v[3])))));
                s.store_offset(1004, 1004, 1.0);
            }
        }

        if (s.b[1016] && s.b[1127]) {
            s.store_mul(989, 1005, 2);
            s.store_mul(990, 1006, 2);
            s.store_scalar(991, (1.0 / (p.p808 + (0.5 * s.v[3]))));
            s.store_scalar(992, (1.0 / (p.p809 + (0.5 * s.v[3]))));
        }

        if (s.b[1016] && s.b[1127]) {
            if ((s.v[3] + s.v[310]) > 1e-9) {
                s.store_offset(1002, 310, s.v[3]);
            } else {
                s.store_scalar(1002, 1e-9);
            }
        }

        if (s.b[1016] && s.b[1127]) {
            if (((s.v[4] + s.v[311]) + p.p810) > 1e-9) {
                s.store_offset_add(1003, 4, 311, p.p810);
            } else {
                s.store_scalar(1003, 1e-9);
            }
        }

        if (s.b[1016] && s.b[1127]) {
            s.store_div_from_scalar_powf_ad(1000, 1.0, s.ad_value(1002), p.p818);
            s.store_div_from_scalar_powf_ad(1001, 1.0, s.ad_value(1003), p.p819);
            s.store_add_scaled_inputs_product_first_ad(993, A::scale_offset(s.ad_value(1000), p.p815, 1.0), (1.0 + (p.p814 * (s.v[353] - 1.0))), 1001, (p.p816 * (1.0 + (p.p814 * (s.v[353] - 1.0)))), 1000, 1001, (p.p817 * (1.0 + (p.p814 * (s.v[353] - 1.0)))));
            s.store_div_scaled_inputs2_indices(994, 989, p.p811, 990, p.p811, 993, 1.0);
            s.store_div_scaled_inputs2_indices(995, 991, p.p811, 992, p.p811, 993, 1.0);
            s.store_div_from_scalar_powf_ad(1000, 1.0, s.ad_value(1002), p.p824);
            s.store_div_from_scalar_powf_ad(1001, 1.0, s.ad_value(1003), p.p825);
            s.store_add_scaled_inputs_product_first_ad(996, A::scale_offset(s.ad_value(1000), p.p821, 1.0), 1.0, 1001, p.p822, 1000, 1001, p.p823);
            s.store_add_scaled_inputs4_indices(998, 989, 1.0, 990, 1.0, 991, -1.0, 992, -1.0);
            s.store_div_scaled_offset_numerator(999, s.ad_value(994), 1.0, 1.0, A::offset(s.ad_value(995), 1.0), 1.0);
            s.store_mul(65, 65, 999);
            s.store_div_scaled_product3_mixed_iiaa(82, 82, 999, A::scale_offset(s.ad_value(995), p.p812, 1.0), 1.0, A::scale_offset(s.ad_value(994), p.p812, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(121, 121, 999, A::offset(A::mul(s.ad_value(39), s.ad_value(995)), 1.0), 1.0, A::offset(A::mul(s.ad_value(39), s.ad_value(994)), 1.0), 1.0);
            s.store_mul(150, 150, 999);
            s.store_div_scaled_inputs_indices(999, 998, p.p820, 996, 1.0);
            s.store_add(40, 40, 999);
            s.store_add(145, 145, 999);
            s.store_div_scaled_inputs_mixed_ia(999, 998, p.p826, A::powf(s.ad_value(996), p.p827), 1.0);
            s.store_add(62, 62, 999);
            s.store_add(155, 155, 999);
        }

        s.b[1128] = ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0));
        s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });

        s.b[1129] = (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0));
        s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });

        if ((s.b[1016] && s.b[1128]) && s.b[1129]) {
            s.store_offset(998, 4, s.v[8]);
            s.store_scalar(999, (1.0 / p.p828));
            s.store_div_from_scalar_scaled_input(11, (p.p828 * p.p828), 998, s.v[8]);
            s.store_div_scaled_add_product(12, A::exp_scaled_input(s.ad_value(999), ((-10.0) * s.v[8])), ((0.1 * s.v[8]) + (0.01 * p.p828)), A::scale_offset(s.ad_value(998), 0.1, (0.01 * p.p828)), A::exp(A::mul_scaled_lhs(s.ad_value(998), (-10.0), s.ad_value(999))), (-1.0), s.ad_value(4), 1.0);
            s.store_div_scaled_add_product(13, A::exp_scaled_input(s.ad_value(999), ((-20.0) * s.v[8])), ((0.05 * s.v[8]) + (0.0025 * p.p828)), A::scale_offset(s.ad_value(998), 0.05, (0.0025 * p.p828)), A::exp(A::mul_scaled_lhs(s.ad_value(998), (-20.0), s.ad_value(999))), (-1.0), s.ad_value(4), 1.0);
        }

        if (s.b[1016] && s.b[1128]) {
            s.store_add_scaled_inputs3_indices(998, 11, 1.0, 12, p.p829, 13, p.p830);
            s.store_add_scaled_product_indices(40, 40, 1.0, 348, 998, 1.0);
            s.store_mul_offset_ad_rhs(65, 65, A::mul(s.ad_value(349), s.ad_value(998)), 1.0);
            s.store_add_scaled_product_indices(145, 145, 1.0, 348, 998, 1.0);
            s.store_mul_offset_ad_rhs(150, 150, A::mul(s.ad_value(349), s.ad_value(998)), 1.0);
        }

        s.copy_ad(175, 40);

        s.copy_ad(176, 41);

        s.copy_ad(177, 42);

        s.copy_ad(179, 43);

        s.copy_ad(180, 44);

        if (s.v[45] > 1e20) {
            if (s.v[45] < 1e26) {
                s.copy_ad(181, 45);
            } else {
                s.store_scalar(181, 1e26);
            }
        } else {
            s.store_scalar(181, 1e20);
        }

        if (s.v[46] > 0.01) {
            s.copy_ad(182, 46);
        } else {
            s.store_scalar(182, 0.01);
        }

        if (s.v[47] > 0.0) {
            s.copy_ad(183, 47);
        } else {
            s.store_scalar(183, 0.0);
        }

        s.copy_ad(184, 48);

        s.copy_ad(185, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(186, 50);
        } else {
            s.store_scalar(186, 0.0);
        }

        s.copy_ad(190, 55);

        s.copy_ad(191, 56);

        if (s.v[57] > 1e23) {
            if (s.v[57] < 1e27) {
                s.copy_ad(192, 57);
            } else {
                s.store_scalar(192, 1e27);
            }
        } else {
            s.store_scalar(192, 1e23);
        }

        if (s.v[58] > 1e23) {
            if (s.v[58] < 1e27) {
                s.copy_ad(193, 58);
            } else {
                s.store_scalar(193, 1e27);
            }
        } else {
            s.store_scalar(193, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(187, 51);
        } else {
            s.store_scalar(187, 0.0);
        }

        if (s.v[53] > 0.0) {
            if (s.v[53] < 0.5) {
                s.copy_ad(189, 53);
            } else {
                s.store_scalar(189, 0.5);
            }
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[52] > 0.0) {
            if (s.v[52] < 1.0) {
                s.copy_ad(188, 52);
            } else {
                s.store_scalar(188, 1.0);
            }
        } else {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(178, 54);

        if (s.v[62] > 0.0) {
            s.copy_ad(194, 62);
        } else {
            s.store_scalar(194, 0.0);
        }

        if (s.v[64] > 0.0) {
            if (s.v[64] < 1.0) {
                s.copy_ad(196, 64);
            } else {
                s.store_scalar(196, 1.0);
            }
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(195, 63);
        } else {
            s.store_scalar(195, 0.0);
        }

        if (s.v[59] > 0.0) {
            s.copy_ad(197, 59);
        } else {
            s.store_scalar(197, 0.0);
        }

        if (s.v[61] > 0.0) {
            if (s.v[61] < 1.0) {
                s.copy_ad(198, 61);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }

        if (s.v[60] > 0.0) {
            s.copy_ad(199, 60);
        } else {
            s.store_scalar(199, 0.0);
        }

        if (s.v[65] > 0.0) {
            s.copy_ad(200, 65);
        } else {
            s.store_scalar(200, 0.0);
        }

        s.copy_ad(201, 66);

        if (s.v[67] > 0.0) {
            s.copy_ad(202, 67);
        } else {
            s.store_scalar(202, 0.0);
        }

        s.copy_ad(203, 68);

        if (s.v[69] > 0.0) {
            s.copy_ad(204, 69);
        } else {
            s.store_scalar(204, 0.0);
        }

        s.copy_ad(205, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(206, 71);
        } else {
            s.store_scalar(206, 0.0);
        }

        s.copy_ad(207, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(208, 73);
        } else {
            s.store_scalar(208, 0.0);
        }

        s.copy_ad(209, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(210, 75);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 76);

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.copy_ad(212, 77);

        if (s.v[78] > 0.0) {
            s.copy_ad(213, 78);
        } else {
            s.store_scalar(213, 0.0);
        }

        s.copy_ad(214, 79);

        if (s.v[80] > (-0.5)) {
            if (s.v[80] < 1.0) {
                s.copy_ad(215, 80);
            } else {
                s.store_scalar(215, 1.0);
            }
        } else {
            s.store_scalar(215, (-0.5));
        }

        if (s.v[81] > (-0.5)) {
            s.copy_ad(216, 81);
        } else {
            s.store_scalar(216, (-0.5));
        }

        if (s.v[82] > 0.0) {
            s.copy_ad(217, 82);
        } else {
            s.store_scalar(217, 0.0);
        }

        s.copy_ad(218, 83);

        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(219, 84);
            } else {
                s.store_scalar(219, 1.0);
            }
        } else {
            s.store_scalar(219, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(220, 85);
        } else {
            s.store_scalar(220, (-0.5));
        }

        if (s.v[86] > 0.01) {
            s.copy_ad(221, 86);
        } else {
            s.store_scalar(221, 0.01);
        }

        if (s.v[87] > 2.0) {
            s.copy_ad(222, 87);
        } else {
            s.store_scalar(222, 2.0);
        }

        if (s.v[88] > 0.0) {
            s.copy_ad(223, 88);
        } else {
            s.store_scalar(223, 0.0);
        }

        if (s.v[89] > 0.0) {
            s.copy_ad(224, 89);
        } else {
            s.store_scalar(224, 0.0);
        }

        if (s.v[90] > 0.0) {
            s.copy_ad(225, 90);
        } else {
            s.store_scalar(225, 0.0);
        }

        s.copy_ad(226, 91);

        if (s.v[92] > 0.0) {
            s.copy_ad(227, 92);
        } else {
            s.store_scalar(227, 0.0);
        }

        s.copy_ad(228, 93);

        s.copy_ad(229, 94);

        if (s.v[95] > 0.0) {
            s.copy_ad(230, 95);
        } else {
            s.store_scalar(230, 0.0);
        }

        if (s.v[96] > 0.0) {
            s.copy_ad(231, 96);
        } else {
            s.store_scalar(231, 0.0);
        }

        if (s.v[97] > 1e-12) {
            s.copy_ad(232, 97);
        } else {
            s.store_scalar(232, 1e-12);
        }

        s.copy_ad(233, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(234, 99);
        } else {
            s.store_scalar(234, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(235, 100);
        } else {
            s.store_scalar(235, 0.0);
        }

        if (s.v[101] > 0.0) {
            s.copy_ad(236, 101);
        } else {
            s.store_scalar(236, 0.0);
        }

        s.copy_ad(237, 102);

        s.copy_ad(238, 103);

        s.copy_ad(239, 104);

        s.copy_ad(240, 105);

        s.copy_ad(241, 106);

        s.copy_ad(242, 107);

        s.copy_ad(243, 108);

        s.copy_ad(244, 109);

        if (s.v[110] > 0.0) {
            s.copy_ad(245, 110);
        } else {
            s.store_scalar(245, 0.0);
        }

        if (s.v[111] > 0.0) {
            s.copy_ad(246, 111);
        } else {
            s.store_scalar(246, 0.0);
        }

        s.copy_ad(247, 112);

        s.copy_ad(248, 113);

        s.copy_ad(249, 114);

        s.copy_ad(250, 115);

        s.copy_ad(251, 116);

        s.copy_ad(252, 117);

        if (s.v[118] > 0.0) {
            s.copy_ad(253, 118);
        } else {
            s.store_scalar(253, 0.0);
        }

        s.copy_ad(254, 119);

        if (s.v[120] > 0.0) {
            s.copy_ad(255, 120);
        } else {
            s.store_scalar(255, 0.0);
        }

        if (s.v[121] > 0.0) {
            s.copy_ad(256, 121);
        } else {
            s.store_scalar(256, 0.0);
        }

        if (s.v[122] > 2.0) {
            s.copy_ad(257, 122);
        } else {
            s.store_scalar(257, 2.0);
        }

        s.copy_ad(258, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(259, 124);
        } else {
            s.store_scalar(259, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(260, 125);
        } else {
            s.store_scalar(260, 0.0);
        }

        if (s.v[126] > 0.0) {
            s.copy_ad(261, 126);
        } else {
            s.store_scalar(261, 0.0);
        }

        s.copy_ad(262, 127);

        s.copy_ad(263, 128);

        s.copy_ad(264, 129);

        if (s.v[130] > 0.0) {
            s.copy_ad(265, 130);
        } else {
            s.store_scalar(265, 0.0);
        }

        if (s.v[131] > 0.0) {
            s.copy_ad(266, 131);
        } else {
            s.store_scalar(266, 0.0);
        }

        if (s.v[132] > 0.0) {
            s.copy_ad(267, 132);
        } else {
            s.store_scalar(267, 0.0);
        }

        s.copy_ad(268, 133);

        s.copy_ad(269, 134);

        s.copy_ad(270, 135);

        s.copy_ad(271, 136);

        if (s.v[137] > 0.0) {
            s.copy_ad(272, 137);
        } else {
            s.store_scalar(272, 0.0);
        }

        if (s.v[138] > 0.0) {
            s.copy_ad(273, 138);
        } else {
            s.store_scalar(273, 0.0);
        }

        s.copy_ad(274, 139);

        s.copy_ad(280, 145);

        s.copy_ad(281, 146);

        s.copy_ad(282, 147);

        if (s.v[148] > 1e20) {
            if (s.v[148] < 1e26) {
                s.copy_ad(283, 148);
            } else {
                s.store_scalar(283, 1e26);
            }
        } else {
            s.store_scalar(283, 1e20);
        }

        if (s.v[149] > 0.0) {
            s.copy_ad(284, 149);
        } else {
            s.store_scalar(284, 0.0);
        }

        if (s.v[150] > 0.0) {
            s.copy_ad(285, 150);
        } else {
            s.store_scalar(285, 0.0);
        }

        s.copy_ad(286, 151);

        if (s.v[152] > 0.0) {
            s.copy_ad(287, 152);
        } else {
            s.store_scalar(287, 0.0);
        }

        if (s.v[153] > 0.0) {
            if (s.v[153] < 1.0) {
                s.copy_ad(288, 153);
            } else {
                s.store_scalar(288, 1.0);
            }
        } else {
            s.store_scalar(288, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(289, 154);
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[155] > 0.0) {
            s.copy_ad(290, 155);
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(292, 157);
            } else {
                s.store_scalar(292, 1.0);
            }
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(291, 156);
        } else {
            s.store_scalar(291, 0.0);
        }

        if (s.v[173] > 0.0) {
            s.copy_ad(306, 173);
        } else {
            s.store_scalar(306, 0.0);
        }

        if ((p.p31 * s.v[1]) > 0.0) {
            s.store_scale(15, 1, p.p31);
        } else {
            s.store_scalar(15, 0.0);
        }

        s.store_scalar(16, p.p16);

        s.store_scalar(17, p.p15);

        s.store_scalar(18, p.p18);

        s.store_scalar(19, p.p17);

        s.b[1130] = (p.p44 == 0.0);
        s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });

        if s.b[1130] {
            s.copy_ad(191, 190);
            s.copy_ad(193, 192);
            s.copy_ad(246, 245);
            s.copy_ad(248, 247);
            s.copy_ad(250, 249);
            s.copy_ad(252, 251);
            s.copy_ad(236, 235);
            s.copy_ad(242, 240);
            s.copy_ad(243, 241);
            s.copy_ad(261, 260);
            s.copy_ad(263, 262);
            s.copy_ad(267, 266);
            s.copy_ad(273, 272);
        }

        s.store_scale(757, 180, 8.8541878176e-12);

        s.store_div(758, 757, 179);

        s.store_square(759, 179);

        s.store_scale(760, 758, 6.241449993689894e18);

        s.store_mul(761, 255, 181);

        if (s.v[761] > 1e20) {
            if (s.v[761] < 1e26) {
            } else {
                s.store_scalar(761, 1e26);
            }
        } else {
            s.store_scalar(761, 1e20);
        }

        s.store_scalar(762, 0.0);

        s.b[1131] = (p.p51 > 0.0);
        s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });

        if s.b[1131] {
            s.store_scale_ad(762, A::powf(s.ad_value(758), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.b[1132] = (s.v[0] == (-1.0));
        s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });

        if (s.b[1131] && s.b[1132]) {
            s.store_scale(762, 762, (7.448711 / 5.951993));
        }

        s.store_scale(763, 758, (1e-8 * 1.0 / (s.v[756])));

        s.store_scale(764, 212, 0.5);

        s.store_scalar(765, 0.5);

        s.b[1133] = (s.v[0] == (-1.0));
        s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });

        if s.b[1133] {
            s.store_scale(764, 212, 0.3333333333333333);
            s.store_scalar(765, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(222)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(766, s.ad_value(997), (-1.0), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(257)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(767, s.ad_value(997), (-1.0), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(768, 1.0, 226);

        s.store_div(769, 757, 190);

        s.store_div(770, 757, 191);

        s.store_div_ad_lhs(771, A::sqrt_scaled_input(s.ad_value(192), (((2.0 * 1.6021918e-19) * s.v[756]) * s.v[356])), 769);

        s.store_div_ad_lhs(772, A::sqrt_scaled_input(s.ad_value(193), (((2.0 * 1.6021918e-19) * s.v[756]) * s.v[356])), 770);

        s.store_square(773, 771);

        s.store_square(774, 772);

        s.store_offset_div_ad(775, A::ln(A::offset(A::exp_scaled_input(s.ad_value(264), (0.005 * s.v[356])), (-1.0))), s.ad_value(264), (-((((((0.005 * s.v[356])) as f64).exp() - 1.0)) as f64).ln()));

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_ad_lhs(776, A::ln_scaled_input(s.ad_value(771), 0.5), 775);

        s.store_add_ad_lhs(777, A::ln_scaled_input(s.ad_value(772), 0.5), 775);

        s.store_div_from_scalar(809, 1.0, 771);

        s.store_offset_scaled(810, 771, 3.1, 8.5);

        s.store_square(778, 810);

        s.store_scale(811, 810, 0.5);

        s.b[1134] = (s.v[809] < 0.06);
        s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });

        if s.b[1134] {
            s.store_scale(779, 809, 64.0);
        }

        s.b[1135] = (s.v[809] <= 0.45);
        s.store_scalar(1135, if s.b[1135] { 1.0 } else { 0.0 });

        if ((!s.b[1134]) && s.b[1135]) {
            s.store_offset_scaled(779, 809, 22.0, 3.0);
        }

        s.b[1136] = (s.v[809] <= 1.6);
        s.store_scalar(1136, if s.b[1136] { 1.0 } else { 0.0 });

        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {
            s.store_offset_scaled(779, 809, (-7.2), 15.5);
        }

        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {
            s.copy_ad(779, 771);
        }

        s.store_add_scaled_inputs_product_right_ad(780, 811, 1.0, 773, 0.5, 771, A::sqrt(A::add_scaled_inputs3(s.ad_value(811), 1.0, s.ad_value(773), 0.25, s.ad_value(779), 1.0)), (-1.0));

        s.store_div_from_scalar(809, 1.0, 772);

        s.store_offset_scaled(810, 772, 3.1, 8.5);

        s.store_square(781, 810);

        s.store_scale(811, 810, 0.5);

        s.b[1137] = (s.v[809] < 0.06);
        s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });

        if s.b[1137] {
            s.store_scale(782, 809, 64.0);
        }

        s.b[1138] = (s.v[809] <= 0.45);
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if ((!s.b[1137]) && s.b[1138]) {
            s.store_offset_scaled(782, 809, 22.0, 3.0);
        }

        s.b[1139] = (s.v[809] <= 1.6);
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {
            s.store_offset_scaled(782, 809, (-7.2), 15.5);
        }

        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {
            s.copy_ad(782, 772);
        }

        s.store_add_scaled_inputs_product_right_ad(783, 811, 1.0, 774, 0.5, 772, A::sqrt(A::add_scaled_inputs3(s.ad_value(811), 1.0, s.ad_value(774), 0.25, s.ad_value(782), 1.0)), (-1.0));

        s.store_div_from_scalar(784, 1.0, 244);

        s.store_scaled_sqrt_scaled_input(785, 244, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(786, 785, 179);

        s.store_mul(787, 785, 190);

        s.store_mul(788, 785, 191);

        s.store_scalar(789, 0.0);

        s.b[1140] = (s.v[239] < 0.0);
        s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });

        if s.b[1140] {
            s.store_div_scaled_inputs_indices(789, 238, (-0.495), 239, 1.0);
        }

        s.store_scalar(790, 0.0);

        s.b[1141] = (s.v[241] < 0.0);
        s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });

        if s.b[1141] {
            s.store_div_scaled_inputs_indices(790, 240, (-0.495), 241, 1.0);
        }

        s.b[1142] = (s.v[243] < 0.0);
        s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });

        if s.b[1142] {
            s.store_div_scaled_inputs_indices(791, 242, (-0.495), 243, 1.0);
        }

        s.store_pow_from_scalar_ad(792, s.v[353], s.ad_value(237));

        s.store_mul(234, 234, 792);

        s.store_mul(235, 235, 792);

        s.store_mul(236, 236, 792);

        if ((1.0 + (s.v[249] * s.v[354])) > 0.0) {
            s.store_offset_scaled(785, 249, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }

        s.store_mul(711, 247, 785);

        s.store_scaled_mul(795, 711, 190, 500000000.0);

        if ((1.0 + (s.v[250] * s.v[354])) > 0.0) {
            s.store_offset_scaled(785, 250, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }

        s.store_mul(712, 248, 785);

        s.store_scaled_mul(796, 712, 191, 500000000.0);

        s.store_scalar(797, 0.0);

        s.b[1143] = (s.v[270] > 1e-10);
        s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });

        if s.b[1143] {
            s.store_div_from_scalar(797, 0.75, 270);
        }

        s.store_square(798, 271);

        s.store_scale(20, 2, s.v[647]);

        s.store_scale(21, 2, s.v[648]);

        s.store_scale(22, 2, s.v[649]);

        s.store_scale(23, 2, s.v[674]);

        s.store_scale(24, 2, s.v[675]);

        s.store_scale(25, 2, s.v[676]);

        s.store_scalar(26, 0.0);

        s.b[1151] = (p.p43 == 3.0);
        s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });

        if s.b[1151] {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 313);

        s.b[1152] = (p.p39 == 0.0);
        s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });

        if s.b[1152] {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.b[1153] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });

        if s.b[1153] {
            s.store_scale(20, 2, s.v[650]);
            s.store_add_scaled_product_indices(21, 2, s.v[651], 26, 27, (-1.0));
            s.copy_ad(22, 27);
            s.store_scale(23, 2, s.v[677]);
            s.store_add_scaled_product_indices(24, 2, s.v[678], 26, 27, (-1.0));
            s.copy_ad(25, 27);
        }

        s.b[1154] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });

        if s.b[1154] {
            if (s.v[20] > 0.0) {
                s.copy_ad(647, 20);
            } else {
                s.store_scalar(647, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[21] > 0.0) {
                s.copy_ad(648, 21);
            } else {
                s.store_scalar(648, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[22] > 0.0) {
                s.copy_ad(649, 22);
            } else {
                s.store_scalar(649, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[23] > 0.0) {
                s.copy_ad(674, 23);
            } else {
                s.store_scalar(674, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[24] > 0.0) {
                s.copy_ad(675, 24);
            } else {
                s.store_scalar(675, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[25] > 0.0) {
                s.copy_ad(676, 25);
            } else {
                s.store_scalar(676, 0.0);
            }
        }

        if (!s.b[1154]) {
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(649, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
            s.store_scalar(676, 0.0);
        }

        s.store_scalar(657, 0.0);

        s.store_scalar(684, 0.0);

        s.store_scalar(659, 0.0);

        s.store_scalar(686, 0.0);

        s.store_scalar(658, 0.0);

        s.store_scalar(685, 0.0);

        s.store_scalar(660, 0.0);

        s.store_scalar(687, 0.0);

        s.store_scalar(655, 0.0);

        s.store_scalar(682, 0.0);

        s.store_scalar(656, 0.0);

        s.store_scalar(683, 0.0);

        s.store_scalar(652, 1.0);

        s.store_scalar(679, 1.0);

        s.store_scalar(653, 1.0);

        s.store_scalar(680, 1.0);

        s.store_scalar(654, 1.0);

        s.store_scalar(681, 1.0);

        s.store_scalar(502, 0.0);

        s.b[1155] = (p.p43 > 0.0);
        s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });

        s.b[1156] = ((s.v[388] * s.v[647]) > 0.0);
        s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1156]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1156])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1157] = ((s.v[389] * s.v[648]) > 0.0);
        s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1157]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1157])) {
            s.store_scalar(456, 100000000.0);
        }

        s.b[1158] = ((s.v[390] * s.v[649]) > 0.0);
        s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1158]) {
            s.store_scaled_ln_ad(457, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(649), s.v[390])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1158])) {
            s.store_scalar(457, 100000000.0);
        }

        if s.b[1155] {
            s.store_min3(655, 455, 456, 457);
        }

        s.b[1159] = ((((s.v[655] * s.v[372])) as f64).abs() < 230.25850929940458);
        s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1159]) {
            s.store_exp_scaled_input(656, 655, s.v[372]);
        }

        s.b[1160] = ((s.v[655] * s.v[372]) < 0.0);
        s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });

        if ((s.b[1155] && (!s.b[1159])) && s.b[1160]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(656, 1e-100, (-230.25850929940458), A::scale(s.ad_value(655), s.v[372]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1155] && (!s.b[1159])) && (!s.b[1160])) {
            s.store_scaled_offset_ad(656, A::mul_offset_rhs(A::scale_offset(s.ad_value(655), s.v[372], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(655), s.v[372], (-230.25850929940458)), A::scale_offset(s.ad_value(655), ((s.v[372]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1155] {
            s.store_scalar(397, s.v[394]);
            s.store_scalar(398, s.v[395]);
            s.store_scalar(399, s.v[396]);
            s.store_scalar(400, p.p848);
            s.store_scalar(401, p.p849);
            s.store_scalar(402, p.p850);
            s.store_scalar(403, p.p845);
            s.store_scalar(404, p.p846);
            s.store_scalar(405, p.p847);
        }

        s.b[1161] = (s.v[647] == 0.0);
        s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1161]) {
            s.store_scalar(397, (s.v[395] + s.v[396]));
            s.store_scalar(400, (0.9 * (p.p849).min(p.p850)));
            s.store_scalar(403, (p.p846 + p.p847));
        }

        s.b[1162] = (s.v[648] == 0.0);
        s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1162]) {
            s.store_scalar(398, (s.v[394] + s.v[396]));
            s.store_scalar(401, (0.9 * (p.p848).min(p.p850)));
            s.store_scalar(404, (p.p845 + p.p847));
        }

        s.b[1163] = (s.v[649] == 0.0);
        s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1163]) {
            s.store_scalar(399, (s.v[394] + s.v[395]));
            s.store_scalar(402, (0.9 * (p.p848).min(p.p849)));
            s.store_scalar(405, (p.p845 + p.p846));
        }

        if s.b[1155] {
            s.store_min3(657, 397, 398, 399);
            s.store_scale(658, 657, 0.1);
            s.store_max3(378, 400, 401, 402);
            s.store_mul_sub_from_scalar_ad_rhs(659, 657, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378))));
            s.store_offset_min_ad(660, A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405), (-0.05));
        }

        s.b[1164] = ((s.v[564] * s.v[674]) > 0.0);
        s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1164]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1164])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1165] = ((s.v[565] * s.v[675]) > 0.0);
        s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1165]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1165])) {
            s.store_scalar(456, 100000000.0);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1166] = ((s.v[566] * s.v[676]) > 0.0);
        s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1166]) {
            s.store_scaled_ln_ad(457, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(566), s.ad_value(676))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1166])) {
            s.store_scalar(457, 100000000.0);
        }

        if s.b[1155] {
            s.store_min3(682, 455, 456, 457);
        }

        s.b[1167] = ((((s.v[682] * s.v[372])) as f64).abs() < 230.25850929940458);
        s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1167]) {
            s.store_exp_scaled_input(683, 682, s.v[372]);
        }

        s.b[1168] = ((s.v[682] * s.v[372]) < 0.0);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if ((s.b[1155] && (!s.b[1167])) && s.b[1168]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(683, 1e-100, (-230.25850929940458), A::scale(s.ad_value(682), s.v[372]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((s.b[1155] && (!s.b[1167])) && (!s.b[1168])) {
            s.store_scaled_offset_ad(683, A::mul_offset_rhs(A::scale_offset(s.ad_value(682), s.v[372], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(682), s.v[372], (-230.25850929940458)), A::scale_offset(s.ad_value(682), ((s.v[372]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1155] {
            s.copy_ad(397, 570);
            s.copy_ad(398, 571);
            s.copy_ad(399, 572);
            s.copy_ad(400, 512);
            s.copy_ad(401, 513);
            s.copy_ad(402, 514);
            s.copy_ad(403, 509);
            s.copy_ad(404, 510);
            s.copy_ad(405, 511);
        }

        s.b[1169] = (s.v[674] == 0.0);
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1169]) {
            s.store_add(397, 571, 572);
            s.store_scale_ad(400, A::min(s.ad_value(513), s.ad_value(514)), 0.9);
            s.store_add(403, 510, 511);
        }

        s.b[1170] = (s.v[675] == 0.0);
        s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1170]) {
            s.store_add(398, 570, 572);
            s.store_scale_ad(401, A::min(s.ad_value(512), s.ad_value(514)), 0.9);
            s.store_add(404, 509, 511);
        }

        s.b[1171] = (s.v[676] == 0.0);
        s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1171]) {
            s.store_add(399, 570, 571);
            s.store_scale_ad(402, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
            s.store_add(405, 509, 510);
        }

        if s.b[1155] {
            s.store_min3(684, 397, 398, 399);
            s.store_scale(685, 684, 0.1);
            s.store_max3(378, 400, 401, 402);
            s.store_mul_sub_from_scalar_ad_rhs(686, 684, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378))));
            s.store_offset_min_ad(687, A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405), (-0.05));
        }

        s.b[1172] = (s.v[475] == 1.0);
        s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });

        if (s.b[1155] && s.b[1172]) {
            s.store_add_scaled_inputs3_indices(502, 647, (s.v[415] * p.p946), 648, (s.v[416] * p.p946), 649, (s.v[417] * p.p946));
        }

        s.b[1507] = ((s.v[647] * s.v[415]) <= s.v[502]);
        s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });

        if ((s.b[1155] && s.b[1172]) && s.b[1507]) {
            s.store_scalar(652, 0.0);
        }

        s.b[1508] = ((s.v[648] * s.v[416]) <= s.v[502]);
        s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });

        if ((s.b[1155] && s.b[1172]) && s.b[1508]) {
            s.store_scalar(653, 0.0);
        }

        s.b[1509] = ((s.v[649] * s.v[417]) <= s.v[502]);
        s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });

        if ((s.b[1155] && s.b[1172]) && s.b[1509]) {
            s.store_scalar(654, 0.0);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_mul_ad_rhs(502, 554, A::add_scaled_products3(s.ad_value(674), s.ad_value(582), 1.0, s.ad_value(675), s.ad_value(583), 1.0, s.ad_value(676), s.ad_value(584), 1.0));
        }

        s.b[1797] = ((s.v[674] * s.v[582]) <= s.v[502]);
        s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });

        if ((s.b[1155] && s.b[1172]) && s.b[1797]) {
            s.store_scalar(679, 0.0);
        }

        s.b[1798] = ((s.v[675] * s.v[583]) <= s.v[502]);
        s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });

        if ((s.b[1155] && s.b[1172]) && s.b[1798]) {
            s.store_scalar(680, 0.0);
        }

        s.b[1799] = ((s.v[676] * s.v[584]) <= s.v[502]);
        s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });

        if ((s.b[1155] && s.b[1172]) && s.b[1799]) {
            s.store_scalar(681, 0.0);
        }

        s.store_scalar(1929, 0.0);

        s.store_scalar(1930, 0.0);

        s.store_scalar(1931, 0.0);

        s.store_offset_voltage(357, ctx, nodes, Some(4), None, s.v[352]);

        s.store_square(358, 357);

        s.store_offset(359, 357, (-s.v[351]));

        s.store_div_from_scalar(360, s.v[351], 357);

        s.store_ln(361, 360);

        s.store_scale(1916, 357, (1.3806505e-23 * 6.241449993689894e18));

        s.store_div_from_scalar(362, 1.0, 1916);

        s.store_sub_scaled_ad_lhs(363, A::sub_from_scalar(1.179, A::scale(s.ad_value(357), 9.025e-5)), 358, 3.05e-7);

        s.store_mul_ad_affine_product_lhs(364, A::scale_offset(s.ad_value(357), 0.00045, 1.045), A::sub_scaled_inputs(A::scale_offset(s.ad_value(357), 0.0014, 0.523), 1.0, s.ad_value(358), 1.48e-6), 1.1111111111111112e-5, 0.0, 358);

        if (!(s.v[364] > 0.001)) {
            s.store_scalar(364, 0.001);
        }

        s.store_add_scaled_inputs_product_right_ad(717, 363, 1.0, 185, 1.0, 1916, A::ln_scaled_input(A::mul(s.ad_value(181), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);

        if (!(s.v[717] > 0.05)) {
            s.store_scalar(717, 0.05);
        }

        s.store_div_ad_lhs(718, A::sqrt(A::mul_scaled_lhs(s.ad_value(181), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);

        s.store_scalar(719, 0.0);

        s.store_scalar(720, 0.0);

        s.b[2004] = (s.v[186] > 0.0);
        s.store_scalar(2004, if s.b[2004] { 1.0 } else { 0.0 });

        if s.b[2004] {
            s.store_div_from_scalar(721, 80000000.0, 759);
        }

        if s.b[2004] {
            if (s.v[186] > s.v[721]) {
                s.copy_ad(720, 186);
            } else {
                s.copy_ad(720, 721);
            }
        }

        if s.b[2004] {
            if (5e24 > s.v[720]) {
                s.store_scalar(720, 5e24);
            } else {
            }
        }

        if s.b[2004] {
            s.store_div_scaled_product3_indices(719, 758, 758, 1916, 2.0, 720, (1.6021918e-19 * s.v[756]));
        }

        s.store_scaled_mul(722, 1916, 1916, 100.0);

        s.b[2005] = (p.p51 > 0.0);
        s.store_scalar(2005, if s.b[2005] { 1.0 } else { 0.0 });

        if s.b[2005] {
            s.store_sqrt_mul_ad(723, A::mul3(s.ad_value(1916), s.ad_value(718), s.ad_value(718)), s.ad_value(717));
            s.store_mul_scaled_powf_rhs(724, 762, 0.75, 723, 0.6666666666666666);
            s.store_add(717, 717, 724);
            s.store_mul_offset_ad_rhs(718, 718, A::div_scaled_inputs(s.ad_value(724), (2.0 * 0.6666666666666666), s.ad_value(723), 1.0), 1.0);
        }

        s.store_sqrt(725, 717);

        s.store_scale(726, 717, 0.95);

        s.store_scaled_mul(727, 717, 717, 0.0025);

        s.copy_ad(728, 727);

        s.store_scaled_sqrt(729, 728, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(730, 726, 0.5, 729, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(726), s.ad_value(729))), s.ad_value(727)), (-0.5));

        s.store_scaled_add(731, 717, 363, 0.5);

        s.store_sub_ad_lhs(732, A::sqrt(A::add(s.ad_value(183), s.ad_value(717))), 725);

        s.store_add_scaled_inputs3_sqrt_first_mixed_aii(733, A::add_scaled_inputs3(s.ad_value(183), 1.0, s.ad_value(184), 1.0, s.ad_value(717), 1.0), 1.0, 725, (-1.0), 732, -1.0);

        s.store_add_scaled_product_mixed_aia(734, A::add_scaled_inputs3(s.ad_value(363), 1.0, s.ad_value(185), 1.0, s.ad_value(254), 1.0), 1.0, 1916, A::ln_scaled_input(A::mul(s.ad_value(761), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);

        if (!(s.v[734] > 0.05)) {
            s.store_scalar(734, 0.05);
        }

        s.store_div_ad_lhs(735, A::sqrt(A::mul_scaled_lhs(s.ad_value(761), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);

        s.b[2006] = (p.p51 > 0.0);
        s.store_scalar(2006, if s.b[2006] { 1.0 } else { 0.0 });

        if s.b[2006] {
            s.store_sqrt_mul_ad(723, A::mul3(s.ad_value(1916), s.ad_value(735), s.ad_value(735)), s.ad_value(734));
            s.store_mul_scaled_powf_rhs(724, 762, 0.75, 723, 0.6666666666666666);
            s.store_add(734, 734, 724);
            s.store_mul_offset_ad_rhs(735, 735, A::div_scaled_inputs(s.ad_value(724), (2.0 * 0.6666666666666666), s.ad_value(723), 1.0), 1.0);
        }

        s.store_scale(736, 734, 0.95);

        s.store_scaled_mul(737, 734, 734, 0.0025);

        s.copy_ad(738, 737);

        s.store_scaled_sqrt(729, 738, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(739, 736, 0.5, 729, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(736), s.ad_value(729))), s.ad_value(737)), (-0.5));

        s.store_offset_add_ad(701, s.ad_value(175), A::mul3(s.ad_value(176), s.ad_value(359), A::offset(A::mul(s.ad_value(177), s.ad_value(359)), 1.0)), s.v[17]);

        s.store_exp_mul(740, 178, 361);

        s.store_mul(702, 187, 740);

        s.store_div(703, 188, 360);

        s.store_exp_mul(741, 201, 361);

        s.store_mul(704, 200, 741);

        s.store_scaled_mul(1917, 704, 758, s.v[16]);

        s.store_mul_exp_ad_rhs(706, 204, A::mul(s.ad_value(205), s.ad_value(361)));

        s.store_exp_mul(742, 203, 361);

        s.store_mul(705, 202, 742);

        s.store_mul_exp_ad_rhs(708, 208, A::mul(s.ad_value(209), s.ad_value(361)));

        s.store_exp_mul(743, 207, 361);

        s.store_mul(707, 206, 743);

        s.store_exp_mul(744, 211, 361);

        s.store_mul(709, 210, 744);

        s.store_exp_mul(745, 214, 361);

        s.store_mul(710, 213, 745);

        s.store_scaled_mul(746, 1917, 710, 2.0);

        s.store_exp_mul(747, 218, 361);

        s.store_mul(1921, 217, 747);

        s.store_mul(1922, 256, 747);

        s.store_mul_exp_ad_rhs(713, 228, A::mul_scaled_lhs(s.ad_value(229), -1.0, s.ad_value(361)));

        s.store_scaled_mul(1920, 274, 357, (4.0 * 1.3806505e-23));

        s.b[2007] = ((p.p46 != 0.0) && (s.v[285] > 0.0));
        s.store_scalar(2007, if s.b[2007] { 1.0 } else { 0.0 });

        if s.b[2007] {
            s.store_offset_add_scaled_product(714, s.ad_value(280), 1.0, s.ad_value(281), s.ad_value(359), 1.0, s.v[19]);
            s.store_exp_mul(748, 286, 361);
            s.store_mul(715, 285, 748);
            s.store_scaled_mul(1918, 715, 758, s.v[18]);
            s.store_mul_offset_ad_rhs(1924, 1916, A::mul(s.ad_value(284), s.ad_value(360)), 1.0);
            s.store_add_scaled_inputs_product_right_ad(749, 363, 1.0, 282, 1.0, 1924, A::ln_scaled_input(A::mul(s.ad_value(283), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);
        }

        if s.b[2007] {
            if (s.v[749] > 0.05) {
            } else {
                s.store_scalar(749, 0.05);
            }
        }

        if s.b[2007] {
            s.store_div_ad_lhs(750, A::sqrt(A::mul_scaled_lhs(s.ad_value(283), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);
            s.store_square(1925, 750);
            s.store_ln(1926, 1925);
            s.store_scale(751, 749, 0.95);
            s.store_scaled_mul(752, 749, 749, 0.0025);
            s.copy_ad(753, 752);
            s.store_scaled_sqrt(754, 753, 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(755, 751, 0.5, 754, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(751), s.ad_value(754))), s.ad_value(752)), (-0.5));
        }

        if (!s.b[2007]) {
            s.store_scalar(714, 0.0);
            s.store_scalar(748, 1.0);
            s.store_scalar(715, 0.0);
            s.store_scalar(1918, 0.0);
            s.copy_ad(1924, 1916);
            s.store_scalar(749, 0.0);
            s.store_scalar(750, 1.0);
            s.store_scalar(1925, 1.0);
            s.store_scalar(1926, 0.0);
            s.store_scalar(751, 0.0);
            s.store_scalar(752, 0.0);
            s.store_scalar(753, 0.0);
            s.store_scalar(754, 0.0);
            s.store_scalar(755, 0.0);
        }

        s.b[2008] = (s.v[0] == 1.0);
        s.store_scalar(2008, if s.b[2008] { 1.0 } else { 0.0 });

        if s.b[2008] {
            s.store_voltage(814, ctx, nodes, Some(6), Some(7));
        }

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[2008] {
            s.store_voltage(815, ctx, nodes, Some(8), Some(7));
            s.store_voltage(816, ctx, nodes, Some(7), Some(9));
            s.store_scaled_voltage(821, ctx, nodes, Some(7), Some(11), -1.0);
            s.store_scaled_voltage(822, ctx, nodes, Some(8), Some(12), -1.0);
        }

        if (!s.b[2008]) {
            s.store_scaled_voltage(814, ctx, nodes, Some(6), Some(7), -1.0);
            s.store_scaled_voltage(815, ctx, nodes, Some(8), Some(7), -1.0);
            s.store_scaled_voltage(816, ctx, nodes, Some(7), Some(9), -1.0);
            s.store_voltage(821, ctx, nodes, Some(7), Some(11));
            s.store_voltage(822, ctx, nodes, Some(8), Some(12));
        }

        s.store_add(818, 814, 816);

        s.copy_ad(823, 814);

        s.copy_ad(824, 816);

        s.store_add(825, 815, 816);

        s.store_sub(826, 814, 815);

        s.store_scale(1801, 823, (-s.v[356]));

        s.store_scale(1802, 826, (-s.v[356]));

        s.store_scaled_sub(1803, 818, 701, (-s.v[356]));

        s.store_scalar(820, 1.0);

        s.b[2009] = (s.v[815] < 0.0);
        s.store_scalar(2009, if s.b[2009] { 1.0 } else { 0.0 });

        if s.b[2009] {
            s.store_scalar(820, (-1.0));
            s.store_sub(814, 814, 815);
            s.store_add(816, 816, 815);
            s.store_neg(815, 815);
        }

        s.store_add(817, 815, 816);

        s.store_div_scaled_product_offset_denominator(819, s.ad_value(815), s.ad_value(815), 1.0, A::sqrt_square_offset(s.ad_value(815), 0.01), 0.1, 1.0);

        s.store_add_scaled_inputs4_mixed_iiai(2013, 817, 0.5, 816, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(728))), (-0.5), 726, 1.0);

        s.copy_ad(1804, 2013);

        s.store_add_scaled_inputs4_mixed_iiai(1932, 816, 1.0, 2013, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2013)), s.ad_value(727))), (-(-0.5)), 730, 1.0);

        s.copy_ad(1805, 1932);

        s.store_scalar(1933, 0.0);

        s.b[2169] = ((p.p45 != 0.0) && (s.v[182] != 1.0));
        s.store_scalar(2169, if s.b[2169] { 1.0 } else { 0.0 });

        if s.b[2169] {
            s.store_add_scaled_inputs3_indices(1934, 1932, 1.0, 815, 0.5, 819, (-0.5));
            s.store_sub_ad_lhs(1935, A::sqrt(A::add(s.ad_value(1934), s.ad_value(717))), 725);
            s.store_offset_div_scaled_inputs2_indices(1929, 1935, 2.0, 732, (-2.0), 733, 1.0, (-1.0));
            s.store_add_scaled_product_mixed_iaa(1936, 1935, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(182), s.ad_value(733), 0.25), A::add(s.ad_value(1929), A::sqrt_square_offset(s.ad_value(1929), 0.4804530139182)), (-1.0));
            s.store_add_scaled_square_product_indices(1937, 1936, 1.0, 725, 1936, 2.0);
            s.store_add_scaled_inputs3_indices(1932, 1937, 1.0, 815, (-0.5), 819, (-(-0.5)));
            s.store_sub(1933, 1805, 1932);
        }

        s.copy_ad(2010, 717);

        s.copy_ad(2011, 727);

        s.copy_ad(2012, 718);

        s.copy_ad(2014, 1932);

        s.copy_ad(2018, 1933);

        s.copy_ad(2015, 1921);

        s.copy_ad(2016, 766);

        s.store_add_scaled_inputs3_indices(2017, 818, 1.0, 2018, (-1.0), 701, -1.0);

        s.store_add_scaled_inputs3_indices(2019, 2014, 1.0, 815, 0.5, 819, (-0.5));

        s.store_scalar(2031, 1.0);

        s.b[2170] = (s.v[188] > 0.0);
        s.store_scalar(2170, if s.b[2170] { 1.0 } else { 0.0 });

        if s.b[2170] {
            s.store_mul(2022, 2010, 362);
            s.store_mul(2023, 2019, 362);
            s.store_mul(2024, 2017, 362);
            s.store_offset_div_scaled_inputs_mixed_ia(1930, 2012, 0.5, A::sqrt(s.ad_value(2022)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(1931, 2022, 1.0, 2012, A::sqrt(s.ad_value(2022)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2025, A::div_scaled_inputs2(s.ad_value(2024), 1.0, s.ad_value(1931), (-1.0), s.ad_value(1930), 1.0), 1.0, 2022, 0.5, A::offset(s.ad_value(189), 1.0), 2023, (-1.0));
            s.store_offset_scaled(2026, 2022, 0.5, 2.0);
            s.store_add(2027, 2022, 2023);
            s.store_sub_scaled_inputs_ad(1930, A::add_scaled_inputs_product(s.ad_value(2024), 1.0, s.ad_value(2027), (-1.0), s.ad_value(2012), A::sqrt(s.ad_value(2027)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2022), s.ad_value(2012)), A::sqrt(s.ad_value(2022)))), 2.0);
            s.store_add_scaled_inputs(2028, 1930, 2.0, 2026, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1930, 2025, 0.5, 2028, 0.5, 2025, 2028, 20.0, 0.5);
            s.store_add_scaled_inputs3_indices(1931, 2024, 2.0, 2023, (-2.0), 2026, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2029, 1930, 0.5, 1931, 0.5, 1930, 1931, 20.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1930, 2029, 0.5, 2026, 0.5, 2029, 2026, 5.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2030, 1930, 0.5, 2026, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2026), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(1931, 703, A::div(s.ad_value(2030), s.ad_value(2026)), 1.0);
        }

        s.b[2171] = (s.v[1931] > (-230.25850929940458));
        s.store_scalar(2171, if s.b[2171] { 1.0 } else { 0.0 });

        if (s.b[2170] && s.b[2171]) {
            s.store_exp(2031, 1931);
        }

        if (s.b[2170] && (!s.b[2171])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2031, 1e-100, (-230.25850929940458), 1931, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_offset_mul(2032, 702, 2031, 1.0);

        s.store_mul(2033, 1916, 2032);

        s.store_mul_ad_product_rhs(2034, 197, A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(198), s.ad_value(2019)), 1.0));

        s.store_mul_offset_rhs(2035, 2033, 2034, 1.0);

        s.store_div_from_scalar(2036, 1.0, 2035);

        s.store_mul_sqrt_ad_rhs(2020, 2012, A::mul(s.ad_value(1916), s.ad_value(2036)));

        s.store_square(2021, 2020);

        s.store_div_from_scalar(2037, 1.0, 2021);

        s.store_mul(2038, 2014, 2036);

        s.store_mul(2039, 2017, 2036);

        s.store_div_scaled_value_offset_denominator(2040, s.ad_value(819), 2.0, A::sqrt_product_offset(s.ad_value(195), s.ad_value(819), 1.0), 1.0, 1.0);

        s.store_mul_ad_product_rhs_mixed_ia(2041, 194, 2040, A::offset(A::mul(s.ad_value(196), s.ad_value(2019)), 1.0));

        s.store_mul(2042, 2010, 2036);

        s.store_sqrt_square_add(1930, 2013, 2011);

        s.store_sqrt_add_ad(1931, A::square(A::sub(s.ad_value(2013), s.ad_value(2041))), s.ad_value(2011));

        s.store_mul_add_scaled_inputs3_offset_rhs(2043, 2036, s.ad_value(2041), 0.5, s.ad_value(1930), 0.5, s.ad_value(1931), ((-1.0) * (0.5)), 0.0);

        s.store_add(2044, 2042, 2038);

        s.store_sub(2045, 2044, 2043);

        s.b[2172] = (p.p45 > 0.0);
        s.store_scalar(2172, if s.b[2172] { 1.0 } else { 0.0 });

        s.b[2173] = (((s.v[2045]) as f64).abs() < 1e-5);
        s.store_scalar(2173, if s.b[2173] { 1.0 } else { 0.0 });

        if (s.b[2172] && s.b[2173]) {
            s.store_offset_ad(2046, A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.3125), 0.5)), 1.0);
        }

        s.b[2174] = (s.v[2045] < 460.51701859880916);
        s.store_scalar(2174, if s.b[2174] { 1.0 } else { 0.0 });

        if ((s.b[2172] && (!s.b[2173])) && s.b[2174]) {
            s.store_exp_neg_input(2060, 2045);
        }

        if ((s.b[2172] && (!s.b[2173])) && (!s.b[2174])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2060, 1e-200, 2045, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2172] && (!s.b[2173])) {
            s.store_scalar(1929, (if (s.v[2045] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2172] && (!s.b[2173])) {
            s.store_offset_ad(2046, A::div_scaled_product3(s.ad_value(1929), s.ad_value(2020), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2060), 1.0, s.ad_value(2045))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2045), 1.0, s.ad_value(2060))), 2.0), 1.0);
        }

        if (!s.b[2172]) {
            s.store_offset_div_scaled_inputs_mixed_ia(2046, 2020, 0.5, A::sqrt(s.ad_value(2045)), 1.0, 1.0);
        }

        s.store_add_scaled_value_products(2047, s.ad_value(2045), 1.0, s.ad_value(2020), A::sqrt(s.ad_value(2045)), 1.0, s.ad_value(2046), A::ln(A::offset(s.ad_value(2046), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2_indices(2048, 2039, 1.0, 2047, (-1.0), 2046, 1.0);

        s.store_mul_scaled_offset_ad_rhs(2054, 2021, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2021)), 1.0)), (-1.0));

        s.store_scalar(2053, 0.0);

        s.store_scalar(2055, 1.0);

        s.b[2175] = (s.v[2048] > (-30.0));
        s.store_scalar(2175, if s.b[2175] { 1.0 } else { 0.0 });

        if s.b[2175] {
            s.store_offset_mul(2049, 2046, 2048, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1929, 2049, 2049, 10.0, 0.5);
            s.store_sub_ad_rhs(2050, 2048, A::ln(s.ad_value(1929)));
            s.store_scaled_add_sqrt_square_offset_rhs(2051, 2050, 2050, 2.0, 0.5);
        }

        s.b[2176] = ((s.v[2048] - s.v[2051]) < 230.25850929940458);
        s.store_scalar(2176, if s.b[2176] { 1.0 } else { 0.0 });

        if (s.b[2175] && s.b[2176]) {
            s.store_exp_sub(1929, 2048, 2051);
        }

        if (s.b[2175] && (!s.b[2176])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1929, A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if s.b[2175] {
            s.store_div(2052, 1929, 2046);
            s.store_sub_ad_lhs(1929, A::scaled_offset(s.ad_value(2051), 1.0, 2.0), 2052);
        }

        s.b[2177] = (s.v[2052] > 1e-6);
        s.store_scalar(2177, if s.b[2177] { 1.0 } else { 0.0 });

        if (s.b[2175] && s.b[2177]) {
            s.store_mul_offset_ad_rhs(2053, 2046, A::sub(s.ad_value(2051), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2052), s.ad_value(1929), 1.0), 1.0, (-1.0), s.ad_value(2052), 1.0)), 1.0);
        }

        if (s.b[2175] && (!s.b[2177])) {
            s.store_mul_ad_affine_product_rhs(2053, 2046, s.ad_value(2052), A::offset(A::mul_scaled_lhs(s.ad_value(1929), 0.25, s.ad_value(1929)), 1.0), 0.5, 0.0);
        }

        if s.b[2175] {
            s.store_add_scaled_inputs3_offset_mixed_iia(1929, 2039, 0.5, 2053, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2039), s.ad_value(2053)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2054, 2021, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2021)), s.ad_value(1929), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2055, 2054, 2054, 1.0, 2053, 1.0);
            s.store_add_scaled_product_indices(2045, 2044, 1.0, 2055, 2043, (-1.0));
        }

        s.store_offset_scaled(2056, 2020, 0.7071067811865475, 1.0);

        s.store_scale(2057, 2056, 1e-5);

        s.store_div_from_scalar(2058, 1.0, 2056);

        s.store_scalar(2165, 0.0);

        s.store_scalar(2059, 0.0);

        s.b[2178] = (s.v[2045] < 460.51701859880916);
        s.store_scalar(2178, if s.b[2178] { 1.0 } else { 0.0 });

        if s.b[2178] {
            s.store_exp_neg_input(2060, 2045);
        }

        if (!s.b[2178]) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2060, 1e-200, 2045, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2179] = (((s.v[2039]) as f64).abs() <= s.v[2057]);
        s.store_scalar(2179, if s.b[2179] { 1.0 } else { 0.0 });

        if s.b[2179] {
            s.store_scaled_square(2145, 2058, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2059, 2039, 2058, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2039), 1.0, s.ad_value(2060)), s.ad_value(2020), s.ad_value(2145)), 1.0));
        }

        s.b[2180] = (s.v[2039] < (-s.v[2057]));
        s.store_scalar(2180, if s.b[2180] { 1.0 } else { 0.0 });

        if ((!s.b[2179]) && s.b[2180]) {
            s.store_neg(2147, 2039);
            s.store_scaled_mul(2148, 2147, 2058, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2149, 2148, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2144, 2147, 2149);
            s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::offset(s.ad_value(2149), 1.0), 1.0);
            s.store_sub_scaled_inputs(2151, 2144, 2.0, 2021, 1.0);
            s.store_sub_ln_mul_lhs(2152, 2150, 2037, 2149);
            s.store_add(813, 2150, 2151);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::sub_scaled_inputs(A::square(s.ad_value(2151)), 0.5, s.ad_value(2150), 1.0), 1.0);
            s.store_add_ad_rhs(2153, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::sub_scaled_inputs(A::square(s.ad_value(2151)), 0.3333333333333333, s.ad_value(2150), 1.0))), 1.0));
        }

        s.b[2181] = (s.v[2153] < 230.25850929940458);
        s.store_scalar(2181, if s.b[2181] { 1.0 } else { 0.0 });

        if (((!s.b[2179]) && s.b[2180]) && s.b[2181]) {
            s.store_exp(2154, 2153);
        }

        if (((!s.b[2179]) && s.b[2180]) && (!s.b[2181])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2154, 2153, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((!s.b[2179]) && s.b[2180]) {
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_div_from_scalar_offset_square(2144, 1.0, 2153, 2.0);
            s.store_mul_square_lhs(2156, 2153, 2144);
            s.store_mul3_affine_lhs(2157, 2153, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), 2144, 2144);
            s.store_sub(2144, 2147, 2153);
            s.store_mul(2145, 2060, 2155);
            s.store_add_scaled_product_right_ad(2159, 2144, 2.0, 2021, A::add_scaled_inputs3_offset(s.ad_value(2154), 1.0, s.ad_value(2145), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2060), 1.0, s.ad_value(2157)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2154), 1.0, s.ad_value(2153), (-1.0), s.ad_value(2145), 1.0, (-1.0)), 1.0, s.ad_value(2060), A::sub(A::offset(s.ad_value(2153), (-1.0)), s.ad_value(2156)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2144, 2.0, 2021, A::add_scaled_inputs_product(s.ad_value(2154), 1.0, s.ad_value(2145), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2059, 2153, -1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_div_from_scalar_offset_scaled_input(2161, 1.0, 2020, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2162, A::mul_scaled_lhs(s.ad_value(2056), 1.25, s.ad_value(2161)), (-1.0), 2161);
            s.store_mul_ad_product_rhs_mixed_ia(2163, 2039, 2058, A::offset(A::mul(s.ad_value(2162), s.ad_value(2039)), 1.0));
        }

        s.b[2182] = ((-s.v[2163]) > (-230.25850929940458));
        s.store_scalar(2182, if s.b[2182] { 1.0 } else { 0.0 });

    }
}
