#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
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
        p: &Parameters,
        var_alphaav_slot: &mut f64,
        var_alphaav_rv_slot: &mut f64,
        var_chnl_type_slot: &mut f64,
        var_chnl_type_rv_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorbotd_i_rv_slot: &mut f64,
        var_cjorgat2nd_slot: &mut f64,
        var_cjorgat2nd_rv_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorgatd_i_rv_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_cjorstid_i_rv_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhbotd_i_rv_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhgatd_i_rv_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_csrhstid_i_rv_slot: &mut f64,
        var_deltaphigr_slot: &mut f64,
        var_deltaphigr_rv_slot: &mut f64,
        var_epssi_slot: &mut f64,
        var_epssi_rv_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard1_rv_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard2_rv_slot: &mut f64,
        var_guard3_slot: &mut f64,
        var_guard3_rv_slot: &mut f64,
        var_guard4_slot: &mut f64,
        var_guard4_rv_slot: &mut f64,
        var_guard5_slot: &mut f64,
        var_guard5_rv_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrbotd_i_rv_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrgatd_i_rv_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_idsatrstid_i_rv_slot: &mut f64,
        var_kbol_over_qele_slot: &mut f64,
        var_kbol_over_qele_rv_slot: &mut f64,
        var_one_minus_pbot_slot: &mut f64,
        var_one_minus_pbot_rv_slot: &mut f64,
        var_one_minus_pgat_slot: &mut f64,
        var_one_minus_pgat2nd_slot: &mut f64,
        var_one_minus_pgat2nd_rv_slot: &mut f64,
        var_one_minus_pgat_rv_slot: &mut f64,
        var_one_minus_psti_slot: &mut f64,
        var_one_minus_psti_rv_slot: &mut f64,
        var_one_over_one_minus_pbot_slot: &mut f64,
        var_one_over_one_minus_pbot_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_rv_slot: &mut f64,
        var_one_over_one_minus_psti_slot: &mut f64,
        var_one_over_one_minus_psti_rv_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbotd_i_rv_slot: &mut f64,
        var_pgat2nd_slot: &mut f64,
        var_pgat2nd_rv_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_pgatd_i_rv_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phigbotd_i_rv_slot: &mut f64,
        var_phiggat2nd_slot: &mut f64,
        var_phiggat2nd_rv_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phiggatd_i_rv_slot: &mut f64,
        var_phigrbot_slot: &mut f64,
        var_phigrbot_rv_slot: &mut f64,
        var_phigrgat_slot: &mut f64,
        var_phigrgat2nd_slot: &mut f64,
        var_phigrgat2nd_rv_slot: &mut f64,
        var_phigrgat_rv_slot: &mut f64,
        var_phigrsti_slot: &mut f64,
        var_phigrsti_rv_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_phigstid_i_rv_slot: &mut f64,
        var_phitr_slot: &mut f64,
        var_phitr_rv_slot: &mut f64,
        var_phitrinv_slot: &mut f64,
        var_phitrinv_rv_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_pstid_i_rv_slot: &mut f64,
        var_swgat2nd_slot: &mut f64,
        var_swgat2nd_rv_slot: &mut f64,
        var_swjunexp_i_slot: &mut f64,
        var_swjunexp_i_rv_slot: &mut f64,
        var_tkr_slot: &mut f64,
        var_tkr_1_slot: &mut f64,
        var_tkr_1_rv_slot: &mut f64,
        var_tkr_rv_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirbotd_i_rv_slot: &mut f64,
        var_vbirbotinv_slot: &mut f64,
        var_vbirbotinv_rv_slot: &mut f64,
        var_vbirgat2nd_slot: &mut f64,
        var_vbirgat2nd_rv_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirgatd_i_rv_slot: &mut f64,
        var_vbirgatinv_slot: &mut f64,
        var_vbirgatinv_rv_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbirstid_i_rv_slot: &mut f64,
        var_vbirstiinv_slot: &mut f64,
        var_vbirstiinv_rv_slot: &mut f64,
        var_vbrinvbot_slot: &mut f64,
        var_vbrinvbot_rv_slot: &mut f64,
        var_vbrinvgat_slot: &mut f64,
        var_vbrinvgat_dn6_slot: &mut f64,
        var_vbrinvgat_dn7_slot: &mut f64,
        var_vbrinvgat_dn8_slot: &mut f64,
        var_vbrinvgat_dn9_slot: &mut f64,
        var_vbrinvgat_rv_slot: &mut f64,
        var_vbrinvsti_slot: &mut f64,
        var_vbrinvsti_rv_slot: &mut f64,
        var_wdepnulrbot_slot: &mut f64,
        var_wdepnulrbot_rv_slot: &mut f64,
        var_wdepnulrgat_slot: &mut f64,
        var_wdepnulrgat_rv_slot: &mut f64,
        var_wdepnulrinvbot_slot: &mut f64,
        var_wdepnulrinvbot_rv_slot: &mut f64,
        var_wdepnulrinvgat_slot: &mut f64,
        var_wdepnulrinvgat_rv_slot: &mut f64,
        var_wdepnulrinvsti_slot: &mut f64,
        var_wdepnulrinvsti_rv_slot: &mut f64,
        var_wdepnulrsti_slot: &mut f64,
        var_wdepnulrsti_rv_slot: &mut f64,
    ) {
        let mut var_alphaav: f64 = *var_alphaav_slot;
        let mut var_alphaav_rv: f64 = *var_alphaav_rv_slot;
        let mut var_chnl_type: f64 = *var_chnl_type_slot;
        let mut var_chnl_type_rv: f64 = *var_chnl_type_rv_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorbotd_i_rv: f64 = *var_cjorbotd_i_rv_slot;
        let mut var_cjorgat2nd: f64 = *var_cjorgat2nd_slot;
        let mut var_cjorgat2nd_rv: f64 = *var_cjorgat2nd_rv_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorgatd_i_rv: f64 = *var_cjorgatd_i_rv_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_cjorstid_i_rv: f64 = *var_cjorstid_i_rv_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhbotd_i_rv: f64 = *var_csrhbotd_i_rv_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhgatd_i_rv: f64 = *var_csrhgatd_i_rv_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_csrhstid_i_rv: f64 = *var_csrhstid_i_rv_slot;
        let mut var_deltaphigr: f64 = *var_deltaphigr_slot;
        let mut var_deltaphigr_rv: f64 = *var_deltaphigr_rv_slot;
        let mut var_epssi: f64 = *var_epssi_slot;
        let mut var_epssi_rv: f64 = *var_epssi_rv_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard1_rv: f64 = *var_guard1_rv_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard2_rv: f64 = *var_guard2_rv_slot;
        let mut var_guard3: f64 = *var_guard3_slot;
        let mut var_guard3_rv: f64 = *var_guard3_rv_slot;
        let mut var_guard4: f64 = *var_guard4_slot;
        let mut var_guard4_rv: f64 = *var_guard4_rv_slot;
        let mut var_guard5: f64 = *var_guard5_slot;
        let mut var_guard5_rv: f64 = *var_guard5_rv_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrbotd_i_rv: f64 = *var_idsatrbotd_i_rv_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrgatd_i_rv: f64 = *var_idsatrgatd_i_rv_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_idsatrstid_i_rv: f64 = *var_idsatrstid_i_rv_slot;
        let mut var_kbol_over_qele: f64 = *var_kbol_over_qele_slot;
        let mut var_kbol_over_qele_rv: f64 = *var_kbol_over_qele_rv_slot;
        let mut var_one_minus_pbot: f64 = *var_one_minus_pbot_slot;
        let mut var_one_minus_pbot_rv: f64 = *var_one_minus_pbot_rv_slot;
        let mut var_one_minus_pgat: f64 = *var_one_minus_pgat_slot;
        let mut var_one_minus_pgat2nd: f64 = *var_one_minus_pgat2nd_slot;
        let mut var_one_minus_pgat2nd_rv: f64 = *var_one_minus_pgat2nd_rv_slot;
        let mut var_one_minus_pgat_rv: f64 = *var_one_minus_pgat_rv_slot;
        let mut var_one_minus_psti: f64 = *var_one_minus_psti_slot;
        let mut var_one_minus_psti_rv: f64 = *var_one_minus_psti_rv_slot;
        let mut var_one_over_one_minus_pbot: f64 = *var_one_over_one_minus_pbot_slot;
        let mut var_one_over_one_minus_pbot_rv: f64 = *var_one_over_one_minus_pbot_rv_slot;
        let mut var_one_over_one_minus_pgat: f64 = *var_one_over_one_minus_pgat_slot;
        let mut var_one_over_one_minus_pgat2nd: f64 = *var_one_over_one_minus_pgat2nd_slot;
        let mut var_one_over_one_minus_pgat2nd_rv: f64 = *var_one_over_one_minus_pgat2nd_rv_slot;
        let mut var_one_over_one_minus_pgat_rv: f64 = *var_one_over_one_minus_pgat_rv_slot;
        let mut var_one_over_one_minus_psti: f64 = *var_one_over_one_minus_psti_slot;
        let mut var_one_over_one_minus_psti_rv: f64 = *var_one_over_one_minus_psti_rv_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbotd_i_rv: f64 = *var_pbotd_i_rv_slot;
        let mut var_pgat2nd: f64 = *var_pgat2nd_slot;
        let mut var_pgat2nd_rv: f64 = *var_pgat2nd_rv_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_pgatd_i_rv: f64 = *var_pgatd_i_rv_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phigbotd_i_rv: f64 = *var_phigbotd_i_rv_slot;
        let mut var_phiggat2nd: f64 = *var_phiggat2nd_slot;
        let mut var_phiggat2nd_rv: f64 = *var_phiggat2nd_rv_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phiggatd_i_rv: f64 = *var_phiggatd_i_rv_slot;
        let mut var_phigrbot: f64 = *var_phigrbot_slot;
        let mut var_phigrbot_rv: f64 = *var_phigrbot_rv_slot;
        let mut var_phigrgat: f64 = *var_phigrgat_slot;
        let mut var_phigrgat2nd: f64 = *var_phigrgat2nd_slot;
        let mut var_phigrgat2nd_rv: f64 = *var_phigrgat2nd_rv_slot;
        let mut var_phigrgat_rv: f64 = *var_phigrgat_rv_slot;
        let mut var_phigrsti: f64 = *var_phigrsti_slot;
        let mut var_phigrsti_rv: f64 = *var_phigrsti_rv_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_phigstid_i_rv: f64 = *var_phigstid_i_rv_slot;
        let mut var_phitr: f64 = *var_phitr_slot;
        let mut var_phitr_rv: f64 = *var_phitr_rv_slot;
        let mut var_phitrinv: f64 = *var_phitrinv_slot;
        let mut var_phitrinv_rv: f64 = *var_phitrinv_rv_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_pstid_i_rv: f64 = *var_pstid_i_rv_slot;
        let mut var_swgat2nd: f64 = *var_swgat2nd_slot;
        let mut var_swgat2nd_rv: f64 = *var_swgat2nd_rv_slot;
        let mut var_swjunexp_i: f64 = *var_swjunexp_i_slot;
        let mut var_swjunexp_i_rv: f64 = *var_swjunexp_i_rv_slot;
        let mut var_tkr: f64 = *var_tkr_slot;
        let mut var_tkr_1: f64 = *var_tkr_1_slot;
        let mut var_tkr_1_rv: f64 = *var_tkr_1_rv_slot;
        let mut var_tkr_rv: f64 = *var_tkr_rv_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirbotd_i_rv: f64 = *var_vbirbotd_i_rv_slot;
        let mut var_vbirbotinv: f64 = *var_vbirbotinv_slot;
        let mut var_vbirbotinv_rv: f64 = *var_vbirbotinv_rv_slot;
        let mut var_vbirgat2nd: f64 = *var_vbirgat2nd_slot;
        let mut var_vbirgat2nd_rv: f64 = *var_vbirgat2nd_rv_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirgatd_i_rv: f64 = *var_vbirgatd_i_rv_slot;
        let mut var_vbirgatinv: f64 = *var_vbirgatinv_slot;
        let mut var_vbirgatinv_rv: f64 = *var_vbirgatinv_rv_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbirstid_i_rv: f64 = *var_vbirstid_i_rv_slot;
        let mut var_vbirstiinv: f64 = *var_vbirstiinv_slot;
        let mut var_vbirstiinv_rv: f64 = *var_vbirstiinv_rv_slot;
        let mut var_vbrinvbot: f64 = *var_vbrinvbot_slot;
        let mut var_vbrinvbot_rv: f64 = *var_vbrinvbot_rv_slot;
        let mut var_vbrinvgat: f64 = *var_vbrinvgat_slot;
        let mut var_vbrinvgat_dn6: f64 = *var_vbrinvgat_dn6_slot;
        let mut var_vbrinvgat_dn7: f64 = *var_vbrinvgat_dn7_slot;
        let mut var_vbrinvgat_dn8: f64 = *var_vbrinvgat_dn8_slot;
        let mut var_vbrinvgat_dn9: f64 = *var_vbrinvgat_dn9_slot;
        let mut var_vbrinvgat_rv: f64 = *var_vbrinvgat_rv_slot;
        let mut var_vbrinvsti: f64 = *var_vbrinvsti_slot;
        let mut var_vbrinvsti_rv: f64 = *var_vbrinvsti_rv_slot;
        let mut var_wdepnulrbot: f64 = *var_wdepnulrbot_slot;
        let mut var_wdepnulrbot_rv: f64 = *var_wdepnulrbot_rv_slot;
        let mut var_wdepnulrgat: f64 = *var_wdepnulrgat_slot;
        let mut var_wdepnulrgat_rv: f64 = *var_wdepnulrgat_rv_slot;
        let mut var_wdepnulrinvbot: f64 = *var_wdepnulrinvbot_slot;
        let mut var_wdepnulrinvbot_rv: f64 = *var_wdepnulrinvbot_rv_slot;
        let mut var_wdepnulrinvgat: f64 = *var_wdepnulrinvgat_slot;
        let mut var_wdepnulrinvgat_rv: f64 = *var_wdepnulrinvgat_rv_slot;
        let mut var_wdepnulrinvsti: f64 = *var_wdepnulrinvsti_slot;
        let mut var_wdepnulrinvsti_rv: f64 = *var_wdepnulrinvsti_rv_slot;
        let mut var_wdepnulrsti: f64 = *var_wdepnulrsti_slot;
        let mut var_wdepnulrsti_rv: f64 = *var_wdepnulrsti_rv_slot;

        let assign00_e1484: f64 = if p.p37 >= 0.0 { 1.0 } else { 0.0 };
        var_guard1 = assign00_e1484;
        var_guard1_rv = 0.0;

        let (assign10_e1489,) = {
    if (var_guard1 != 0.0) {
        let assign10_e1487: f64 = 1.0;
        (assign10_e1487,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign10_e1489;
        var_chnl_type_rv = 0.0;

        let (assign20_e1495,) = {
    if (var_guard1 == 0.0) {
        let assign20_e1493: f64 = (-1.0);
        (assign20_e1493,)
    } else {
        (var_chnl_type,)
    }
};
        var_chnl_type = assign20_e1495;
        var_chnl_type_rv = 0.0;

        let assign30_e1498: f64 = (8.8541878176e-12 * 11.8);
        var_epssi = assign30_e1498;
        var_epssi_rv = 0.0;

        let assign40_e1501: f64 = (273.15 + p.p38);
        var_tkr = assign40_e1501;
        var_tkr_rv = 0.0;

        var_swjunexp_i = 0.0;
        var_swjunexp_i_rv = 0.0;

        let assign60_e1505: f64 = if p.p944 > 0.5 { 1.0 } else { 0.0 };
        var_guard2 = assign60_e1505;
        var_guard2_rv = 0.0;

        let (assign70_e1509,) = {
    if (var_guard2 != 0.0) {
        (1.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign70_e1509;
        var_swjunexp_i_rv = 0.0;

        let (assign80_e1514,) = {
    if (var_guard2 == 0.0) {
        (0.0,)
    } else {
        (var_swjunexp_i,)
    }
};
        var_swjunexp_i = assign80_e1514;
        var_swjunexp_i_rv = 0.0;

        let assign90_e1517: f64 = (273.15 + p.p840);
        var_tkr_1 = assign90_e1517;
        var_tkr_1_rv = 0.0;

        let assign100_e1520: f64 = (1.3806505e-23 / 1.6021918e-19);
        var_kbol_over_qele = assign100_e1520;
        var_kbol_over_qele_rv = 0.0;

        let assign110_e1523: f64 = (var_kbol_over_qele * var_tkr_1);
        var_phitr = assign110_e1523;
        var_phitr_rv = 0.0;

        let assign120_e1526: f64 = (1.0 / var_phitr);
        var_phitrinv = assign120_e1526;
        var_phitrinv_rv = 0.0;

        let assign130_e1529: f64 = (0.000702 * var_tkr_1);
        let assign130_e1531: f64 = (assign130_e1529 * var_tkr_1);
        let assign130_e1532: f64 = (-assign130_e1531);
        let assign130_e1535: f64 = (1108.0 + var_tkr_1);
        let assign130_e1536: f64 = (assign130_e1532 / assign130_e1535);
        var_deltaphigr = assign130_e1536;
        var_deltaphigr_rv = 0.0;

        let assign140_e1539: f64 = (p.p851 + var_deltaphigr);
        var_phigrbot = assign140_e1539;
        var_phigrbot_rv = 0.0;

        let assign150_e1542: f64 = (p.p852 + var_deltaphigr);
        var_phigrsti = assign150_e1542;
        var_phigrsti_rv = 0.0;

        let assign160_e1545: f64 = (p.p853 + var_deltaphigr);
        var_phigrgat = assign160_e1545;
        var_phigrgat_rv = 0.0;

        let assign170_e1548: f64 = (1.0 - p.p848);
        var_one_minus_pbot = assign170_e1548;
        var_one_minus_pbot_rv = 0.0;

        let assign180_e1551: f64 = (1.0 - p.p849);
        var_one_minus_psti = assign180_e1551;
        var_one_minus_psti_rv = 0.0;

        let assign190_e1554: f64 = (1.0 - p.p850);
        var_one_minus_pgat = assign190_e1554;
        var_one_minus_pgat_rv = 0.0;

        let assign200_e1557: f64 = (1.0 / var_one_minus_pbot);
        var_one_over_one_minus_pbot = assign200_e1557;
        var_one_over_one_minus_pbot_rv = 0.0;

        let assign210_e1560: f64 = (1.0 / var_one_minus_psti);
        var_one_over_one_minus_psti = assign210_e1560;
        var_one_over_one_minus_psti_rv = 0.0;

        let assign220_e1563: f64 = (1.0 / var_one_minus_pgat);
        var_one_over_one_minus_pgat = assign220_e1563;
        var_one_over_one_minus_pgat_rv = 0.0;

        let assign230_e1566: f64 = (var_epssi / p.p842);
        var_wdepnulrbot = assign230_e1566;
        var_wdepnulrbot_rv = 0.0;

        let assign240_e1569: f64 = (p.p860 * var_epssi);
        let assign240_e1571: f64 = (assign240_e1569 / p.p843);
        var_wdepnulrsti = assign240_e1571;
        var_wdepnulrsti_rv = 0.0;

        let assign250_e1574: f64 = (p.p861 * var_epssi);
        let assign250_e1576: f64 = (assign250_e1574 / p.p844);
        var_wdepnulrgat = assign250_e1576;
        var_wdepnulrgat_rv = 0.0;

        let assign260_e1579: f64 = (1.0 / var_wdepnulrbot);
        var_wdepnulrinvbot = assign260_e1579;
        var_wdepnulrinvbot_rv = 0.0;

        let assign270_e1582: f64 = (1.0 / var_wdepnulrsti);
        var_wdepnulrinvsti = assign270_e1582;
        var_wdepnulrinvsti_rv = 0.0;

        let assign280_e1585: f64 = (1.0 / var_wdepnulrgat);
        var_wdepnulrinvgat = assign280_e1585;
        var_wdepnulrinvgat_rv = 0.0;

        let assign290_e1588: f64 = (1.0 / p.p845);
        var_vbirbotinv = assign290_e1588;
        var_vbirbotinv_rv = 0.0;

        let assign300_e1591: f64 = (1.0 / p.p846);
        var_vbirstiinv = assign300_e1591;
        var_vbirstiinv_rv = 0.0;

        let assign310_e1594: f64 = (1.0 / p.p847);
        var_vbirgatinv = assign310_e1594;
        var_vbirgatinv_rv = 0.0;

        let assign350_e1619: f64 = (1.0 / p.p841);
        let assign350_e1620: f64 = (1.0 - assign350_e1619);
        var_alphaav = assign350_e1620;
        var_alphaav_rv = 0.0;

        let assign390_e1644: f64 = (1.0 / p.p877);
        var_vbrinvbot = assign390_e1644;
        var_vbrinvbot_rv = 0.0;

        let assign400_e1647: f64 = (1.0 / p.p878);
        var_vbrinvsti = assign400_e1647;
        var_vbrinvsti_rv = 0.0;

        let assign410_e1650: f64 = (1.0 / p.p879);
        var_vbrinvgat = assign410_e1650;
        var_vbrinvgat_dn6 = 0.0;
        var_vbrinvgat_dn7 = 0.0;
        var_vbrinvgat_dn8 = 0.0;
        var_vbrinvgat_dn9 = 0.0;
        var_vbrinvgat_rv = 0.0;

        let assign450_e1707: f64 = if ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0)) { 1.0 } else { 0.0 };
        var_guard3 = assign450_e1707;
        var_guard3_rv = 0.0;

        let (assign460_e1711,) = {
    if (var_guard3 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign460_e1711;
        var_swgat2nd_rv = 0.0;

        let (assign470_e1716,) = {
    if (var_guard3 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd,)
    }
};
        var_swgat2nd = assign470_e1716;
        var_swgat2nd_rv = 0.0;

        let assign480_e1719: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard4 = assign480_e1719;
        var_guard4_rv = 0.0;

        let (assign490_e1732,) = {
    if (var_guard4 != 0.0) {
        let assign490_e1723: f64 = (p.p844 * p.p883);
        let (assign490_e1730,) = {
            if (assign490_e1723 > 1e-18) {
                let assign490_e1728: f64 = (p.p844 * p.p883);
                (assign490_e1728,)
            } else {
                (1e-18,)
            }
        };
        (assign490_e1730,)
    } else {
        (var_cjorgat2nd,)
    }
};
        var_cjorgat2nd = assign490_e1732;
        var_cjorgat2nd_rv = 0.0;

        let (assign500_e1745,) = {
    if (var_guard4 != 0.0) {
        let assign500_e1736: f64 = (p.p847 * p.p884);
        let (assign500_e1743,) = {
            if (assign500_e1736 > 0.05) {
                let assign500_e1741: f64 = (p.p847 * p.p884);
                (assign500_e1741,)
            } else {
                (0.05,)
            }
        };
        (assign500_e1743,)
    } else {
        (var_vbirgat2nd,)
    }
};
        var_vbirgat2nd = assign500_e1745;
        var_vbirgat2nd_rv = 0.0;

        let (assign510_e1772,) = {
    if (var_guard4 != 0.0) {
        let assign510_e1749: f64 = (p.p850 * p.p885);
        let (assign510_e1756,) = {
            if (assign510_e1749 > 0.05) {
                let assign510_e1754: f64 = (p.p850 * p.p885);
                (assign510_e1754,)
            } else {
                (0.05,)
            }
        };
        let (assign510_e1770,) = {
            if (assign510_e1756 < 0.95) {
                let assign510_e1761: f64 = (p.p850 * p.p885);
                let (assign510_e1768,) = {
                    if (assign510_e1761 > 0.05) {
                        let assign510_e1766: f64 = (p.p850 * p.p885);
                        (assign510_e1766,)
                    } else {
                        (0.05,)
                    }
                };
                (assign510_e1768,)
            } else {
                (0.95,)
            }
        };
        (assign510_e1770,)
    } else {
        (var_pgat2nd,)
    }
};
        var_pgat2nd = assign510_e1772;
        var_pgat2nd_rv = 0.0;

        let (assign520_e1778,) = {
    if (var_guard4 != 0.0) {
        let assign520_e1776: f64 = (p.p853 * p.p886);
        (assign520_e1776,)
    } else {
        (var_phiggat2nd,)
    }
};
        var_phiggat2nd = assign520_e1778;
        var_phiggat2nd_rv = 0.0;

        let (assign530_e1784,) = {
    if (var_guard4 != 0.0) {
        let assign530_e1782: f64 = (var_phiggat2nd + var_deltaphigr);
        (assign530_e1782,)
    } else {
        (var_phigrgat2nd,)
    }
};
        var_phigrgat2nd = assign530_e1784;
        var_phigrgat2nd_rv = 0.0;

        let (assign540_e1790,) = {
    if (var_guard4 != 0.0) {
        let assign540_e1788: f64 = (1.0 - var_pgat2nd);
        (assign540_e1788,)
    } else {
        (var_one_minus_pgat2nd,)
    }
};
        var_one_minus_pgat2nd = assign540_e1790;
        var_one_minus_pgat2nd_rv = 0.0;

        let (assign550_e1796,) = {
    if (var_guard4 != 0.0) {
        let assign550_e1794: f64 = (1.0 / var_one_minus_pgat2nd);
        (assign550_e1794,)
    } else {
        (var_one_over_one_minus_pgat2nd,)
    }
};
        var_one_over_one_minus_pgat2nd = assign550_e1796;
        var_one_over_one_minus_pgat2nd_rv = 0.0;

        let assign560_e1799: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard5 = assign560_e1799;
        var_guard5_rv = 0.0;

        let (assign570_e1803,) = {
    if (var_guard5 != 0.0) {
        (p.p842,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign570_e1803;
        var_cjorbotd_i_rv = 0.0;

        let (assign580_e1807,) = {
    if (var_guard5 != 0.0) {
        (p.p843,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign580_e1807;
        var_cjorstid_i_rv = 0.0;

        let (assign590_e1811,) = {
    if (var_guard5 != 0.0) {
        (p.p844,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign590_e1811;
        var_cjorgatd_i_rv = 0.0;

        let (assign600_e1815,) = {
    if (var_guard5 != 0.0) {
        (p.p845,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign600_e1815;
        var_vbirbotd_i_rv = 0.0;

        let (assign610_e1819,) = {
    if (var_guard5 != 0.0) {
        (p.p846,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign610_e1819;
        var_vbirstid_i_rv = 0.0;

        let (assign620_e1823,) = {
    if (var_guard5 != 0.0) {
        (p.p847,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign620_e1823;
        var_vbirgatd_i_rv = 0.0;

        let (assign630_e1827,) = {
    if (var_guard5 != 0.0) {
        (p.p848,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign630_e1827;
        var_pbotd_i_rv = 0.0;

        let (assign640_e1831,) = {
    if (var_guard5 != 0.0) {
        (p.p849,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign640_e1831;
        var_pstid_i_rv = 0.0;

        let (assign650_e1835,) = {
    if (var_guard5 != 0.0) {
        (p.p850,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign650_e1835;
        var_pgatd_i_rv = 0.0;

        let (assign660_e1839,) = {
    if (var_guard5 != 0.0) {
        (p.p851,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign660_e1839;
        var_phigbotd_i_rv = 0.0;

        let (assign670_e1843,) = {
    if (var_guard5 != 0.0) {
        (p.p852,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign670_e1843;
        var_phigstid_i_rv = 0.0;

        let (assign680_e1847,) = {
    if (var_guard5 != 0.0) {
        (p.p853,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign680_e1847;
        var_phiggatd_i_rv = 0.0;

        let (assign690_e1851,) = {
    if (var_guard5 != 0.0) {
        (p.p854,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign690_e1851;
        var_idsatrbotd_i_rv = 0.0;

        let (assign700_e1855,) = {
    if (var_guard5 != 0.0) {
        (p.p855,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign700_e1855;
        var_idsatrstid_i_rv = 0.0;

        let (assign710_e1859,) = {
    if (var_guard5 != 0.0) {
        (p.p856,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign710_e1859;
        var_idsatrgatd_i_rv = 0.0;

        let (assign720_e1863,) = {
    if (var_guard5 != 0.0) {
        (p.p857,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign720_e1863;
        var_csrhbotd_i_rv = 0.0;

        let (assign730_e1867,) = {
    if (var_guard5 != 0.0) {
        (p.p858,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign730_e1867;
        var_csrhstid_i_rv = 0.0;

        let (assign740_e1871,) = {
    if (var_guard5 != 0.0) {
        (p.p859,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign740_e1871;
        var_csrhgatd_i_rv = 0.0;

        *var_alphaav_slot = var_alphaav;
        *var_alphaav_rv_slot = var_alphaav_rv;
        *var_chnl_type_slot = var_chnl_type;
        *var_chnl_type_rv_slot = var_chnl_type_rv;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorbotd_i_rv_slot = var_cjorbotd_i_rv;
        *var_cjorgat2nd_slot = var_cjorgat2nd;
        *var_cjorgat2nd_rv_slot = var_cjorgat2nd_rv;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorgatd_i_rv_slot = var_cjorgatd_i_rv;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_cjorstid_i_rv_slot = var_cjorstid_i_rv;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhbotd_i_rv_slot = var_csrhbotd_i_rv;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhgatd_i_rv_slot = var_csrhgatd_i_rv;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_csrhstid_i_rv_slot = var_csrhstid_i_rv;
        *var_deltaphigr_slot = var_deltaphigr;
        *var_deltaphigr_rv_slot = var_deltaphigr_rv;
        *var_epssi_slot = var_epssi;
        *var_epssi_rv_slot = var_epssi_rv;
        *var_guard1_slot = var_guard1;
        *var_guard1_rv_slot = var_guard1_rv;
        *var_guard2_slot = var_guard2;
        *var_guard2_rv_slot = var_guard2_rv;
        *var_guard3_slot = var_guard3;
        *var_guard3_rv_slot = var_guard3_rv;
        *var_guard4_slot = var_guard4;
        *var_guard4_rv_slot = var_guard4_rv;
        *var_guard5_slot = var_guard5;
        *var_guard5_rv_slot = var_guard5_rv;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrbotd_i_rv_slot = var_idsatrbotd_i_rv;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrgatd_i_rv_slot = var_idsatrgatd_i_rv;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_idsatrstid_i_rv_slot = var_idsatrstid_i_rv;
        *var_kbol_over_qele_slot = var_kbol_over_qele;
        *var_kbol_over_qele_rv_slot = var_kbol_over_qele_rv;
        *var_one_minus_pbot_slot = var_one_minus_pbot;
        *var_one_minus_pbot_rv_slot = var_one_minus_pbot_rv;
        *var_one_minus_pgat_slot = var_one_minus_pgat;
        *var_one_minus_pgat2nd_slot = var_one_minus_pgat2nd;
        *var_one_minus_pgat2nd_rv_slot = var_one_minus_pgat2nd_rv;
        *var_one_minus_pgat_rv_slot = var_one_minus_pgat_rv;
        *var_one_minus_psti_slot = var_one_minus_psti;
        *var_one_minus_psti_rv_slot = var_one_minus_psti_rv;
        *var_one_over_one_minus_pbot_slot = var_one_over_one_minus_pbot;
        *var_one_over_one_minus_pbot_rv_slot = var_one_over_one_minus_pbot_rv;
        *var_one_over_one_minus_pgat_slot = var_one_over_one_minus_pgat;
        *var_one_over_one_minus_pgat2nd_slot = var_one_over_one_minus_pgat2nd;
        *var_one_over_one_minus_pgat2nd_rv_slot = var_one_over_one_minus_pgat2nd_rv;
        *var_one_over_one_minus_pgat_rv_slot = var_one_over_one_minus_pgat_rv;
        *var_one_over_one_minus_psti_slot = var_one_over_one_minus_psti;
        *var_one_over_one_minus_psti_rv_slot = var_one_over_one_minus_psti_rv;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbotd_i_rv_slot = var_pbotd_i_rv;
        *var_pgat2nd_slot = var_pgat2nd;
        *var_pgat2nd_rv_slot = var_pgat2nd_rv;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_pgatd_i_rv_slot = var_pgatd_i_rv;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phigbotd_i_rv_slot = var_phigbotd_i_rv;
        *var_phiggat2nd_slot = var_phiggat2nd;
        *var_phiggat2nd_rv_slot = var_phiggat2nd_rv;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phiggatd_i_rv_slot = var_phiggatd_i_rv;
        *var_phigrbot_slot = var_phigrbot;
        *var_phigrbot_rv_slot = var_phigrbot_rv;
        *var_phigrgat_slot = var_phigrgat;
        *var_phigrgat2nd_slot = var_phigrgat2nd;
        *var_phigrgat2nd_rv_slot = var_phigrgat2nd_rv;
        *var_phigrgat_rv_slot = var_phigrgat_rv;
        *var_phigrsti_slot = var_phigrsti;
        *var_phigrsti_rv_slot = var_phigrsti_rv;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_phigstid_i_rv_slot = var_phigstid_i_rv;
        *var_phitr_slot = var_phitr;
        *var_phitr_rv_slot = var_phitr_rv;
        *var_phitrinv_slot = var_phitrinv;
        *var_phitrinv_rv_slot = var_phitrinv_rv;
        *var_pstid_i_slot = var_pstid_i;
        *var_pstid_i_rv_slot = var_pstid_i_rv;
        *var_swgat2nd_slot = var_swgat2nd;
        *var_swgat2nd_rv_slot = var_swgat2nd_rv;
        *var_swjunexp_i_slot = var_swjunexp_i;
        *var_swjunexp_i_rv_slot = var_swjunexp_i_rv;
        *var_tkr_slot = var_tkr;
        *var_tkr_1_slot = var_tkr_1;
        *var_tkr_1_rv_slot = var_tkr_1_rv;
        *var_tkr_rv_slot = var_tkr_rv;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirbotd_i_rv_slot = var_vbirbotd_i_rv;
        *var_vbirbotinv_slot = var_vbirbotinv;
        *var_vbirbotinv_rv_slot = var_vbirbotinv_rv;
        *var_vbirgat2nd_slot = var_vbirgat2nd;
        *var_vbirgat2nd_rv_slot = var_vbirgat2nd_rv;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirgatd_i_rv_slot = var_vbirgatd_i_rv;
        *var_vbirgatinv_slot = var_vbirgatinv;
        *var_vbirgatinv_rv_slot = var_vbirgatinv_rv;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbirstid_i_rv_slot = var_vbirstid_i_rv;
        *var_vbirstiinv_slot = var_vbirstiinv;
        *var_vbirstiinv_rv_slot = var_vbirstiinv_rv;
        *var_vbrinvbot_slot = var_vbrinvbot;
        *var_vbrinvbot_rv_slot = var_vbrinvbot_rv;
        *var_vbrinvgat_slot = var_vbrinvgat;
        *var_vbrinvgat_dn6_slot = var_vbrinvgat_dn6;
        *var_vbrinvgat_dn7_slot = var_vbrinvgat_dn7;
        *var_vbrinvgat_dn8_slot = var_vbrinvgat_dn8;
        *var_vbrinvgat_dn9_slot = var_vbrinvgat_dn9;
        *var_vbrinvgat_rv_slot = var_vbrinvgat_rv;
        *var_vbrinvsti_slot = var_vbrinvsti;
        *var_vbrinvsti_rv_slot = var_vbrinvsti_rv;
        *var_wdepnulrbot_slot = var_wdepnulrbot;
        *var_wdepnulrbot_rv_slot = var_wdepnulrbot_rv;
        *var_wdepnulrgat_slot = var_wdepnulrgat;
        *var_wdepnulrgat_rv_slot = var_wdepnulrgat_rv;
        *var_wdepnulrinvbot_slot = var_wdepnulrinvbot;
        *var_wdepnulrinvbot_rv_slot = var_wdepnulrinvbot_rv;
        *var_wdepnulrinvgat_slot = var_wdepnulrinvgat;
        *var_wdepnulrinvgat_rv_slot = var_wdepnulrinvgat_rv;
        *var_wdepnulrinvsti_slot = var_wdepnulrinvsti;
        *var_wdepnulrinvsti_rv_slot = var_wdepnulrinvsti_rv;
        *var_wdepnulrsti_slot = var_wdepnulrsti;
        *var_wdepnulrsti_rv_slot = var_wdepnulrsti_rv;
    }

    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        var_guard5: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_adbbtgatd_i_rv_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_advbrgatd_i_rv_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_anugatd_i_rv_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdbbtgatd_i_rv_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_rv_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtbotd_i_rv_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtgatd_i_rv_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cbbtstid_i_rv_slot: &mut f64,
        var_cjorbotd_i_slot: &mut f64,
        var_cjorbotd_i_rv_slot: &mut f64,
        var_cjorgatd_i_slot: &mut f64,
        var_cjorgatd_i_rv_slot: &mut f64,
        var_cjorstid_i_slot: &mut f64,
        var_cjorstid_i_rv_slot: &mut f64,
        var_csrhbotd_i_slot: &mut f64,
        var_csrhbotd_i_rv_slot: &mut f64,
        var_csrhstid_i_slot: &mut f64,
        var_csrhstid_i_rv_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatbotd_i_rv_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatgatd_i_rv_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_ctatstid_i_rv_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrbotd_i_rv_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrgatd_i_rv_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fbbtrstid_i_rv_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fcjorgat2d_i_rv_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
        var_fjunqd_i_rv_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fpgat2d_i_rv_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fphiggat2d_i_rv_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_fvbirgat2d_i_rv_slot: &mut f64,
        var_idsatrbotd_i_slot: &mut f64,
        var_idsatrbotd_i_rv_slot: &mut f64,
        var_idsatrgatd_i_slot: &mut f64,
        var_idsatrgatd_i_rv_slot: &mut f64,
        var_idsatrstid_i_slot: &mut f64,
        var_idsatrstid_i_rv_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatbotd_i_rv_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatgatd_i_rv_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_mefftatstid_i_rv_slot: &mut f64,
        var_pbotd_i_slot: &mut f64,
        var_pbotd_i_rv_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrbotd_i_rv_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrgatd_i_rv_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pbrstid_i_rv_slot: &mut f64,
        var_pgatd_i_slot: &mut f64,
        var_pgatd_i_rv_slot: &mut f64,
        var_phigbotd_i_slot: &mut f64,
        var_phigbotd_i_rv_slot: &mut f64,
        var_phiggatd_i_slot: &mut f64,
        var_phiggatd_i_rv_slot: &mut f64,
        var_phigstid_i_slot: &mut f64,
        var_phigstid_i_rv_slot: &mut f64,
        var_pstid_i_slot: &mut f64,
        var_pstid_i_rv_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtbotd_i_rv_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtgatd_i_rv_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_stfbbtstid_i_rv_slot: &mut f64,
        var_vbirbotd_i_slot: &mut f64,
        var_vbirbotd_i_rv_slot: &mut f64,
        var_vbirgatd_i_slot: &mut f64,
        var_vbirgatd_i_rv_slot: &mut f64,
        var_vbirstid_i_slot: &mut f64,
        var_vbirstid_i_rv_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrbotd_i_rv_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrgatd_i_rv_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vbrstid_i_rv_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_vtrgatd_i_rv_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjungatd_i_rv_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
        var_xjunstid_i_rv_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_adbbtgatd_i_rv: f64 = *var_adbbtgatd_i_rv_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_advbrgatd_i_rv: f64 = *var_advbrgatd_i_rv_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_anugatd_i_rv: f64 = *var_anugatd_i_rv_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdbbtgatd_i_rv: f64 = *var_bdbbtgatd_i_rv_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_bdvbrgatd_i_rv: f64 = *var_bdvbrgatd_i_rv_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtbotd_i_rv: f64 = *var_cbbtbotd_i_rv_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtgatd_i_rv: f64 = *var_cbbtgatd_i_rv_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_cbbtstid_i_rv: f64 = *var_cbbtstid_i_rv_slot;
        let mut var_cjorbotd_i: f64 = *var_cjorbotd_i_slot;
        let mut var_cjorbotd_i_rv: f64 = *var_cjorbotd_i_rv_slot;
        let mut var_cjorgatd_i: f64 = *var_cjorgatd_i_slot;
        let mut var_cjorgatd_i_rv: f64 = *var_cjorgatd_i_rv_slot;
        let mut var_cjorstid_i: f64 = *var_cjorstid_i_slot;
        let mut var_cjorstid_i_rv: f64 = *var_cjorstid_i_rv_slot;
        let mut var_csrhbotd_i: f64 = *var_csrhbotd_i_slot;
        let mut var_csrhbotd_i_rv: f64 = *var_csrhbotd_i_rv_slot;
        let mut var_csrhstid_i: f64 = *var_csrhstid_i_slot;
        let mut var_csrhstid_i_rv: f64 = *var_csrhstid_i_rv_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatbotd_i_rv: f64 = *var_ctatbotd_i_rv_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatgatd_i_rv: f64 = *var_ctatgatd_i_rv_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_ctatstid_i_rv: f64 = *var_ctatstid_i_rv_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrbotd_i_rv: f64 = *var_fbbtrbotd_i_rv_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrgatd_i_rv: f64 = *var_fbbtrgatd_i_rv_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fbbtrstid_i_rv: f64 = *var_fbbtrstid_i_rv_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fcjorgat2d_i_rv: f64 = *var_fcjorgat2d_i_rv_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
        let mut var_fjunqd_i_rv: f64 = *var_fjunqd_i_rv_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fpgat2d_i_rv: f64 = *var_fpgat2d_i_rv_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fphiggat2d_i_rv: f64 = *var_fphiggat2d_i_rv_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_fvbirgat2d_i_rv: f64 = *var_fvbirgat2d_i_rv_slot;
        let mut var_idsatrbotd_i: f64 = *var_idsatrbotd_i_slot;
        let mut var_idsatrbotd_i_rv: f64 = *var_idsatrbotd_i_rv_slot;
        let mut var_idsatrgatd_i: f64 = *var_idsatrgatd_i_slot;
        let mut var_idsatrgatd_i_rv: f64 = *var_idsatrgatd_i_rv_slot;
        let mut var_idsatrstid_i: f64 = *var_idsatrstid_i_slot;
        let mut var_idsatrstid_i_rv: f64 = *var_idsatrstid_i_rv_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatbotd_i_rv: f64 = *var_mefftatbotd_i_rv_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatgatd_i_rv: f64 = *var_mefftatgatd_i_rv_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_mefftatstid_i_rv: f64 = *var_mefftatstid_i_rv_slot;
        let mut var_pbotd_i: f64 = *var_pbotd_i_slot;
        let mut var_pbotd_i_rv: f64 = *var_pbotd_i_rv_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrbotd_i_rv: f64 = *var_pbrbotd_i_rv_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrgatd_i_rv: f64 = *var_pbrgatd_i_rv_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pbrstid_i_rv: f64 = *var_pbrstid_i_rv_slot;
        let mut var_pgatd_i: f64 = *var_pgatd_i_slot;
        let mut var_pgatd_i_rv: f64 = *var_pgatd_i_rv_slot;
        let mut var_phigbotd_i: f64 = *var_phigbotd_i_slot;
        let mut var_phigbotd_i_rv: f64 = *var_phigbotd_i_rv_slot;
        let mut var_phiggatd_i: f64 = *var_phiggatd_i_slot;
        let mut var_phiggatd_i_rv: f64 = *var_phiggatd_i_rv_slot;
        let mut var_phigstid_i: f64 = *var_phigstid_i_slot;
        let mut var_phigstid_i_rv: f64 = *var_phigstid_i_rv_slot;
        let mut var_pstid_i: f64 = *var_pstid_i_slot;
        let mut var_pstid_i_rv: f64 = *var_pstid_i_rv_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtbotd_i_rv: f64 = *var_stfbbtbotd_i_rv_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtgatd_i_rv: f64 = *var_stfbbtgatd_i_rv_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_stfbbtstid_i_rv: f64 = *var_stfbbtstid_i_rv_slot;
        let mut var_vbirbotd_i: f64 = *var_vbirbotd_i_slot;
        let mut var_vbirbotd_i_rv: f64 = *var_vbirbotd_i_rv_slot;
        let mut var_vbirgatd_i: f64 = *var_vbirgatd_i_slot;
        let mut var_vbirgatd_i_rv: f64 = *var_vbirgatd_i_rv_slot;
        let mut var_vbirstid_i: f64 = *var_vbirstid_i_slot;
        let mut var_vbirstid_i_rv: f64 = *var_vbirstid_i_rv_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrbotd_i_rv: f64 = *var_vbrbotd_i_rv_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrgatd_i_rv: f64 = *var_vbrgatd_i_rv_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vbrstid_i_rv: f64 = *var_vbrstid_i_rv_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_vtrgatd_i_rv: f64 = *var_vtrgatd_i_rv_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjungatd_i_rv: f64 = *var_xjungatd_i_rv_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;
        let mut var_xjunstid_i_rv: f64 = *var_xjunstid_i_rv_slot;

        let (assign750_e1875,) = {
    if (var_guard5 != 0.0) {
        (p.p860,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign750_e1875;
        var_xjunstid_i_rv = 0.0;

        let (assign760_e1879,) = {
    if (var_guard5 != 0.0) {
        (p.p861,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign760_e1879;
        var_xjungatd_i_rv = 0.0;

        let (assign770_e1883,) = {
    if (var_guard5 != 0.0) {
        (p.p862,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign770_e1883;
        var_ctatbotd_i_rv = 0.0;

        let (assign780_e1887,) = {
    if (var_guard5 != 0.0) {
        (p.p863,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign780_e1887;
        var_ctatstid_i_rv = 0.0;

        let (assign790_e1891,) = {
    if (var_guard5 != 0.0) {
        (p.p864,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign790_e1891;
        var_ctatgatd_i_rv = 0.0;

        let (assign800_e1895,) = {
    if (var_guard5 != 0.0) {
        (p.p865,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign800_e1895;
        var_mefftatbotd_i_rv = 0.0;

        let (assign810_e1899,) = {
    if (var_guard5 != 0.0) {
        (p.p866,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign810_e1899;
        var_mefftatstid_i_rv = 0.0;

        let (assign820_e1903,) = {
    if (var_guard5 != 0.0) {
        (p.p867,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign820_e1903;
        var_mefftatgatd_i_rv = 0.0;

        let (assign830_e1907,) = {
    if (var_guard5 != 0.0) {
        (p.p868,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign830_e1907;
        var_cbbtbotd_i_rv = 0.0;

        let (assign840_e1911,) = {
    if (var_guard5 != 0.0) {
        (p.p869,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign840_e1911;
        var_cbbtstid_i_rv = 0.0;

        let (assign850_e1915,) = {
    if (var_guard5 != 0.0) {
        (p.p870,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign850_e1915;
        var_cbbtgatd_i_rv = 0.0;

        let (assign860_e1919,) = {
    if (var_guard5 != 0.0) {
        (p.p871,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign860_e1919;
        var_fbbtrbotd_i_rv = 0.0;

        let (assign870_e1923,) = {
    if (var_guard5 != 0.0) {
        (p.p872,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign870_e1923;
        var_fbbtrstid_i_rv = 0.0;

        let (assign880_e1927,) = {
    if (var_guard5 != 0.0) {
        (p.p873,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign880_e1927;
        var_fbbtrgatd_i_rv = 0.0;

        let (assign890_e1931,) = {
    if (var_guard5 != 0.0) {
        (p.p874,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign890_e1931;
        var_stfbbtbotd_i_rv = 0.0;

        let (assign900_e1935,) = {
    if (var_guard5 != 0.0) {
        (p.p875,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign900_e1935;
        var_stfbbtstid_i_rv = 0.0;

        let (assign910_e1939,) = {
    if (var_guard5 != 0.0) {
        (p.p876,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign910_e1939;
        var_stfbbtgatd_i_rv = 0.0;

        let (assign920_e1943,) = {
    if (var_guard5 != 0.0) {
        (p.p877,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign920_e1943;
        var_vbrbotd_i_rv = 0.0;

        let (assign930_e1947,) = {
    if (var_guard5 != 0.0) {
        (p.p878,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign930_e1947;
        var_vbrstid_i_rv = 0.0;

        let (assign940_e1951,) = {
    if (var_guard5 != 0.0) {
        (p.p879,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign940_e1951;
        var_vbrgatd_i_rv = 0.0;

        let (assign950_e1955,) = {
    if (var_guard5 != 0.0) {
        (p.p880,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign950_e1955;
        var_pbrbotd_i_rv = 0.0;

        let (assign960_e1959,) = {
    if (var_guard5 != 0.0) {
        (p.p881,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign960_e1959;
        var_pbrstid_i_rv = 0.0;

        let (assign970_e1963,) = {
    if (var_guard5 != 0.0) {
        (p.p882,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign970_e1963;
        var_pbrgatd_i_rv = 0.0;

        let (assign990_e1971,) = {
    if (var_guard5 != 0.0) {
        (p.p946,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign990_e1971;
        var_fjunqd_i_rv = 0.0;

        let (assign1000_e1975,) = {
    if (var_guard5 != 0.0) {
        (p.p889,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1000_e1975;
        var_advbrgatd_i_rv = 0.0;

        let (assign1010_e1979,) = {
    if (var_guard5 != 0.0) {
        (p.p890,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1010_e1979;
        var_bdvbrgatd_i_rv = 0.0;

        let (assign1020_e1983,) = {
    if (var_guard5 != 0.0) {
        (p.p891,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1020_e1983;
        var_adbbtgatd_i_rv = 0.0;

        let (assign1030_e1987,) = {
    if (var_guard5 != 0.0) {
        (p.p892,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1030_e1987;
        var_bdbbtgatd_i_rv = 0.0;

        let (assign1040_e1991,) = {
    if (var_guard5 != 0.0) {
        (p.p883,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1040_e1991;
        var_fcjorgat2d_i_rv = 0.0;

        let (assign1050_e1995,) = {
    if (var_guard5 != 0.0) {
        (p.p884,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1050_e1995;
        var_fvbirgat2d_i_rv = 0.0;

        let (assign1060_e1999,) = {
    if (var_guard5 != 0.0) {
        (p.p885,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1060_e1999;
        var_fpgat2d_i_rv = 0.0;

        let (assign1070_e2003,) = {
    if (var_guard5 != 0.0) {
        (p.p886,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1070_e2003;
        var_fphiggat2d_i_rv = 0.0;

        let (assign1080_e2007,) = {
    if (var_guard5 != 0.0) {
        (p.p887,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1080_e2007;
        var_vtrgatd_i_rv = 0.0;

        let (assign1090_e2011,) = {
    if (var_guard5 != 0.0) {
        (p.p888,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1090_e2011;
        var_anugatd_i_rv = 0.0;

        let (assign1100_e2016,) = {
    if (var_guard5 == 0.0) {
        (p.p893,)
    } else {
        (var_cjorbotd_i,)
    }
};
        var_cjorbotd_i = assign1100_e2016;
        var_cjorbotd_i_rv = 0.0;

        let (assign1110_e2021,) = {
    if (var_guard5 == 0.0) {
        (p.p894,)
    } else {
        (var_cjorstid_i,)
    }
};
        var_cjorstid_i = assign1110_e2021;
        var_cjorstid_i_rv = 0.0;

        let (assign1120_e2026,) = {
    if (var_guard5 == 0.0) {
        (p.p895,)
    } else {
        (var_cjorgatd_i,)
    }
};
        var_cjorgatd_i = assign1120_e2026;
        var_cjorgatd_i_rv = 0.0;

        let (assign1130_e2031,) = {
    if (var_guard5 == 0.0) {
        (p.p896,)
    } else {
        (var_vbirbotd_i,)
    }
};
        var_vbirbotd_i = assign1130_e2031;
        var_vbirbotd_i_rv = 0.0;

        let (assign1140_e2036,) = {
    if (var_guard5 == 0.0) {
        (p.p897,)
    } else {
        (var_vbirstid_i,)
    }
};
        var_vbirstid_i = assign1140_e2036;
        var_vbirstid_i_rv = 0.0;

        let (assign1150_e2041,) = {
    if (var_guard5 == 0.0) {
        (p.p898,)
    } else {
        (var_vbirgatd_i,)
    }
};
        var_vbirgatd_i = assign1150_e2041;
        var_vbirgatd_i_rv = 0.0;

        let (assign1160_e2046,) = {
    if (var_guard5 == 0.0) {
        (p.p899,)
    } else {
        (var_pbotd_i,)
    }
};
        var_pbotd_i = assign1160_e2046;
        var_pbotd_i_rv = 0.0;

        let (assign1170_e2051,) = {
    if (var_guard5 == 0.0) {
        (p.p900,)
    } else {
        (var_pstid_i,)
    }
};
        var_pstid_i = assign1170_e2051;
        var_pstid_i_rv = 0.0;

        let (assign1180_e2056,) = {
    if (var_guard5 == 0.0) {
        (p.p901,)
    } else {
        (var_pgatd_i,)
    }
};
        var_pgatd_i = assign1180_e2056;
        var_pgatd_i_rv = 0.0;

        let (assign1190_e2061,) = {
    if (var_guard5 == 0.0) {
        (p.p902,)
    } else {
        (var_phigbotd_i,)
    }
};
        var_phigbotd_i = assign1190_e2061;
        var_phigbotd_i_rv = 0.0;

        let (assign1200_e2066,) = {
    if (var_guard5 == 0.0) {
        (p.p903,)
    } else {
        (var_phigstid_i,)
    }
};
        var_phigstid_i = assign1200_e2066;
        var_phigstid_i_rv = 0.0;

        let (assign1210_e2071,) = {
    if (var_guard5 == 0.0) {
        (p.p904,)
    } else {
        (var_phiggatd_i,)
    }
};
        var_phiggatd_i = assign1210_e2071;
        var_phiggatd_i_rv = 0.0;

        let (assign1220_e2076,) = {
    if (var_guard5 == 0.0) {
        (p.p905,)
    } else {
        (var_idsatrbotd_i,)
    }
};
        var_idsatrbotd_i = assign1220_e2076;
        var_idsatrbotd_i_rv = 0.0;

        let (assign1230_e2081,) = {
    if (var_guard5 == 0.0) {
        (p.p906,)
    } else {
        (var_idsatrstid_i,)
    }
};
        var_idsatrstid_i = assign1230_e2081;
        var_idsatrstid_i_rv = 0.0;

        let (assign1240_e2086,) = {
    if (var_guard5 == 0.0) {
        (p.p907,)
    } else {
        (var_idsatrgatd_i,)
    }
};
        var_idsatrgatd_i = assign1240_e2086;
        var_idsatrgatd_i_rv = 0.0;

        let (assign1250_e2091,) = {
    if (var_guard5 == 0.0) {
        (p.p908,)
    } else {
        (var_csrhbotd_i,)
    }
};
        var_csrhbotd_i = assign1250_e2091;
        var_csrhbotd_i_rv = 0.0;

        let (assign1260_e2096,) = {
    if (var_guard5 == 0.0) {
        (p.p909,)
    } else {
        (var_csrhstid_i,)
    }
};
        var_csrhstid_i = assign1260_e2096;
        var_csrhstid_i_rv = 0.0;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_adbbtgatd_i_rv_slot = var_adbbtgatd_i_rv;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_advbrgatd_i_rv_slot = var_advbrgatd_i_rv;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_anugatd_i_rv_slot = var_anugatd_i_rv;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdbbtgatd_i_rv_slot = var_bdbbtgatd_i_rv;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_bdvbrgatd_i_rv_slot = var_bdvbrgatd_i_rv;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtbotd_i_rv_slot = var_cbbtbotd_i_rv;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtgatd_i_rv_slot = var_cbbtgatd_i_rv;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cbbtstid_i_rv_slot = var_cbbtstid_i_rv;
        *var_cjorbotd_i_slot = var_cjorbotd_i;
        *var_cjorbotd_i_rv_slot = var_cjorbotd_i_rv;
        *var_cjorgatd_i_slot = var_cjorgatd_i;
        *var_cjorgatd_i_rv_slot = var_cjorgatd_i_rv;
        *var_cjorstid_i_slot = var_cjorstid_i;
        *var_cjorstid_i_rv_slot = var_cjorstid_i_rv;
        *var_csrhbotd_i_slot = var_csrhbotd_i;
        *var_csrhbotd_i_rv_slot = var_csrhbotd_i_rv;
        *var_csrhstid_i_slot = var_csrhstid_i;
        *var_csrhstid_i_rv_slot = var_csrhstid_i_rv;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatbotd_i_rv_slot = var_ctatbotd_i_rv;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatgatd_i_rv_slot = var_ctatgatd_i_rv;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_ctatstid_i_rv_slot = var_ctatstid_i_rv;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrbotd_i_rv_slot = var_fbbtrbotd_i_rv;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrgatd_i_rv_slot = var_fbbtrgatd_i_rv;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fbbtrstid_i_rv_slot = var_fbbtrstid_i_rv;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fcjorgat2d_i_rv_slot = var_fcjorgat2d_i_rv;
        *var_fjunqd_i_slot = var_fjunqd_i;
        *var_fjunqd_i_rv_slot = var_fjunqd_i_rv;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fpgat2d_i_rv_slot = var_fpgat2d_i_rv;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fphiggat2d_i_rv_slot = var_fphiggat2d_i_rv;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_fvbirgat2d_i_rv_slot = var_fvbirgat2d_i_rv;
        *var_idsatrbotd_i_slot = var_idsatrbotd_i;
        *var_idsatrbotd_i_rv_slot = var_idsatrbotd_i_rv;
        *var_idsatrgatd_i_slot = var_idsatrgatd_i;
        *var_idsatrgatd_i_rv_slot = var_idsatrgatd_i_rv;
        *var_idsatrstid_i_slot = var_idsatrstid_i;
        *var_idsatrstid_i_rv_slot = var_idsatrstid_i_rv;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatbotd_i_rv_slot = var_mefftatbotd_i_rv;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatgatd_i_rv_slot = var_mefftatgatd_i_rv;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_mefftatstid_i_rv_slot = var_mefftatstid_i_rv;
        *var_pbotd_i_slot = var_pbotd_i;
        *var_pbotd_i_rv_slot = var_pbotd_i_rv;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrbotd_i_rv_slot = var_pbrbotd_i_rv;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrgatd_i_rv_slot = var_pbrgatd_i_rv;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pbrstid_i_rv_slot = var_pbrstid_i_rv;
        *var_pgatd_i_slot = var_pgatd_i;
        *var_pgatd_i_rv_slot = var_pgatd_i_rv;
        *var_phigbotd_i_slot = var_phigbotd_i;
        *var_phigbotd_i_rv_slot = var_phigbotd_i_rv;
        *var_phiggatd_i_slot = var_phiggatd_i;
        *var_phiggatd_i_rv_slot = var_phiggatd_i_rv;
        *var_phigstid_i_slot = var_phigstid_i;
        *var_phigstid_i_rv_slot = var_phigstid_i_rv;
        *var_pstid_i_slot = var_pstid_i;
        *var_pstid_i_rv_slot = var_pstid_i_rv;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtbotd_i_rv_slot = var_stfbbtbotd_i_rv;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtgatd_i_rv_slot = var_stfbbtgatd_i_rv;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_stfbbtstid_i_rv_slot = var_stfbbtstid_i_rv;
        *var_vbirbotd_i_slot = var_vbirbotd_i;
        *var_vbirbotd_i_rv_slot = var_vbirbotd_i_rv;
        *var_vbirgatd_i_slot = var_vbirgatd_i;
        *var_vbirgatd_i_rv_slot = var_vbirgatd_i_rv;
        *var_vbirstid_i_slot = var_vbirstid_i;
        *var_vbirstid_i_rv_slot = var_vbirstid_i_rv;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrbotd_i_rv_slot = var_vbrbotd_i_rv;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrgatd_i_rv_slot = var_vbrgatd_i_rv;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vbrstid_i_rv_slot = var_vbrstid_i_rv;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_vtrgatd_i_rv_slot = var_vtrgatd_i_rv;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjungatd_i_rv_slot = var_xjungatd_i_rv;
        *var_xjunstid_i_slot = var_xjunstid_i;
        *var_xjunstid_i_rv_slot = var_xjunstid_i_rv;
    }

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        var_cjorbotd_i: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigr: f64,
        var_epssi: f64,
        var_guard5: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggatd_i: f64,
        var_phigstid_i: f64,
        var_pstid_i: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_adbbtgatd_i_slot: &mut f64,
        var_adbbtgatd_i_rv_slot: &mut f64,
        var_advbrgatd_i_slot: &mut f64,
        var_advbrgatd_i_rv_slot: &mut f64,
        var_anugatd_i_slot: &mut f64,
        var_anugatd_i_rv_slot: &mut f64,
        var_bdbbtgatd_i_slot: &mut f64,
        var_bdbbtgatd_i_rv_slot: &mut f64,
        var_bdvbrgatd_i_slot: &mut f64,
        var_bdvbrgatd_i_rv_slot: &mut f64,
        var_cbbtbotd_i_slot: &mut f64,
        var_cbbtbotd_i_rv_slot: &mut f64,
        var_cbbtgatd_i_slot: &mut f64,
        var_cbbtgatd_i_rv_slot: &mut f64,
        var_cbbtstid_i_slot: &mut f64,
        var_cbbtstid_i_rv_slot: &mut f64,
        var_cjorgat2nd_d_slot: &mut f64,
        var_cjorgat2nd_d_rv_slot: &mut f64,
        var_csrhgatd_i_slot: &mut f64,
        var_csrhgatd_i_rv_slot: &mut f64,
        var_ctatbotd_i_slot: &mut f64,
        var_ctatbotd_i_rv_slot: &mut f64,
        var_ctatgatd_i_slot: &mut f64,
        var_ctatgatd_i_rv_slot: &mut f64,
        var_ctatstid_i_slot: &mut f64,
        var_ctatstid_i_rv_slot: &mut f64,
        var_fbbtrbotd_i_slot: &mut f64,
        var_fbbtrbotd_i_rv_slot: &mut f64,
        var_fbbtrgatd_i_slot: &mut f64,
        var_fbbtrgatd_i_rv_slot: &mut f64,
        var_fbbtrstid_i_slot: &mut f64,
        var_fbbtrstid_i_rv_slot: &mut f64,
        var_fcjorgat2d_i_slot: &mut f64,
        var_fcjorgat2d_i_rv_slot: &mut f64,
        var_fjunqd_i_slot: &mut f64,
        var_fjunqd_i_rv_slot: &mut f64,
        var_fpgat2d_i_slot: &mut f64,
        var_fpgat2d_i_rv_slot: &mut f64,
        var_fphiggat2d_i_slot: &mut f64,
        var_fphiggat2d_i_rv_slot: &mut f64,
        var_fvbirgat2d_i_slot: &mut f64,
        var_fvbirgat2d_i_rv_slot: &mut f64,
        var_guard6_slot: &mut f64,
        var_guard6_rv_slot: &mut f64,
        var_guard7_slot: &mut f64,
        var_guard7_rv_slot: &mut f64,
        var_mefftatbotd_i_slot: &mut f64,
        var_mefftatbotd_i_rv_slot: &mut f64,
        var_mefftatgatd_i_slot: &mut f64,
        var_mefftatgatd_i_rv_slot: &mut f64,
        var_mefftatstid_i_slot: &mut f64,
        var_mefftatstid_i_rv_slot: &mut f64,
        var_one_minus_pbot_d_slot: &mut f64,
        var_one_minus_pbot_d_rv_slot: &mut f64,
        var_one_minus_pgat_d_slot: &mut f64,
        var_one_minus_pgat_d_rv_slot: &mut f64,
        var_one_minus_psti_d_slot: &mut f64,
        var_one_minus_psti_d_rv_slot: &mut f64,
        var_one_over_one_minus_pbot_d_slot: &mut f64,
        var_one_over_one_minus_pbot_d_rv_slot: &mut f64,
        var_one_over_one_minus_pgat_d_slot: &mut f64,
        var_one_over_one_minus_pgat_d_rv_slot: &mut f64,
        var_one_over_one_minus_psti_d_slot: &mut f64,
        var_one_over_one_minus_psti_d_rv_slot: &mut f64,
        var_pbrbotd_i_slot: &mut f64,
        var_pbrbotd_i_rv_slot: &mut f64,
        var_pbrgatd_i_slot: &mut f64,
        var_pbrgatd_i_rv_slot: &mut f64,
        var_pbrstid_i_slot: &mut f64,
        var_pbrstid_i_rv_slot: &mut f64,
        var_phigrbot_d_slot: &mut f64,
        var_phigrbot_d_rv_slot: &mut f64,
        var_phigrgat_d_slot: &mut f64,
        var_phigrgat_d_rv_slot: &mut f64,
        var_phigrsti_d_slot: &mut f64,
        var_phigrsti_d_rv_slot: &mut f64,
        var_stfbbtbotd_i_slot: &mut f64,
        var_stfbbtbotd_i_rv_slot: &mut f64,
        var_stfbbtgatd_i_slot: &mut f64,
        var_stfbbtgatd_i_rv_slot: &mut f64,
        var_stfbbtstid_i_slot: &mut f64,
        var_stfbbtstid_i_rv_slot: &mut f64,
        var_swgat2nd_d_slot: &mut f64,
        var_swgat2nd_d_rv_slot: &mut f64,
        var_vbirbotinv_d_slot: &mut f64,
        var_vbirbotinv_d_rv_slot: &mut f64,
        var_vbirgat2nd_d_slot: &mut f64,
        var_vbirgat2nd_d_rv_slot: &mut f64,
        var_vbirgatinv_d_slot: &mut f64,
        var_vbirgatinv_d_rv_slot: &mut f64,
        var_vbirstiinv_d_slot: &mut f64,
        var_vbirstiinv_d_rv_slot: &mut f64,
        var_vbrbotd_i_slot: &mut f64,
        var_vbrbotd_i_rv_slot: &mut f64,
        var_vbrgatd_i_slot: &mut f64,
        var_vbrgatd_i_rv_slot: &mut f64,
        var_vbrinvbot_d_slot: &mut f64,
        var_vbrinvbot_d_rv_slot: &mut f64,
        var_vbrinvgat_d_slot: &mut f64,
        var_vbrinvgat_d_dn6_slot: &mut f64,
        var_vbrinvgat_d_dn7_slot: &mut f64,
        var_vbrinvgat_d_dn8_slot: &mut f64,
        var_vbrinvgat_d_dn9_slot: &mut f64,
        var_vbrinvgat_d_rv_slot: &mut f64,
        var_vbrinvsti_d_slot: &mut f64,
        var_vbrinvsti_d_rv_slot: &mut f64,
        var_vbrstid_i_slot: &mut f64,
        var_vbrstid_i_rv_slot: &mut f64,
        var_vtrgatd_i_slot: &mut f64,
        var_vtrgatd_i_rv_slot: &mut f64,
        var_wdepnulrbot_d_slot: &mut f64,
        var_wdepnulrbot_d_rv_slot: &mut f64,
        var_wdepnulrgat_d_slot: &mut f64,
        var_wdepnulrgat_d_rv_slot: &mut f64,
        var_wdepnulrinvbot_d_slot: &mut f64,
        var_wdepnulrinvbot_d_rv_slot: &mut f64,
        var_wdepnulrinvgat_d_slot: &mut f64,
        var_wdepnulrinvgat_d_rv_slot: &mut f64,
        var_wdepnulrinvsti_d_slot: &mut f64,
        var_wdepnulrinvsti_d_rv_slot: &mut f64,
        var_wdepnulrsti_d_slot: &mut f64,
        var_wdepnulrsti_d_rv_slot: &mut f64,
        var_xjungatd_i_slot: &mut f64,
        var_xjungatd_i_rv_slot: &mut f64,
        var_xjunstid_i_slot: &mut f64,
        var_xjunstid_i_rv_slot: &mut f64,
    ) {
        let mut var_adbbtgatd_i: f64 = *var_adbbtgatd_i_slot;
        let mut var_adbbtgatd_i_rv: f64 = *var_adbbtgatd_i_rv_slot;
        let mut var_advbrgatd_i: f64 = *var_advbrgatd_i_slot;
        let mut var_advbrgatd_i_rv: f64 = *var_advbrgatd_i_rv_slot;
        let mut var_anugatd_i: f64 = *var_anugatd_i_slot;
        let mut var_anugatd_i_rv: f64 = *var_anugatd_i_rv_slot;
        let mut var_bdbbtgatd_i: f64 = *var_bdbbtgatd_i_slot;
        let mut var_bdbbtgatd_i_rv: f64 = *var_bdbbtgatd_i_rv_slot;
        let mut var_bdvbrgatd_i: f64 = *var_bdvbrgatd_i_slot;
        let mut var_bdvbrgatd_i_rv: f64 = *var_bdvbrgatd_i_rv_slot;
        let mut var_cbbtbotd_i: f64 = *var_cbbtbotd_i_slot;
        let mut var_cbbtbotd_i_rv: f64 = *var_cbbtbotd_i_rv_slot;
        let mut var_cbbtgatd_i: f64 = *var_cbbtgatd_i_slot;
        let mut var_cbbtgatd_i_rv: f64 = *var_cbbtgatd_i_rv_slot;
        let mut var_cbbtstid_i: f64 = *var_cbbtstid_i_slot;
        let mut var_cbbtstid_i_rv: f64 = *var_cbbtstid_i_rv_slot;
        let mut var_cjorgat2nd_d: f64 = *var_cjorgat2nd_d_slot;
        let mut var_cjorgat2nd_d_rv: f64 = *var_cjorgat2nd_d_rv_slot;
        let mut var_csrhgatd_i: f64 = *var_csrhgatd_i_slot;
        let mut var_csrhgatd_i_rv: f64 = *var_csrhgatd_i_rv_slot;
        let mut var_ctatbotd_i: f64 = *var_ctatbotd_i_slot;
        let mut var_ctatbotd_i_rv: f64 = *var_ctatbotd_i_rv_slot;
        let mut var_ctatgatd_i: f64 = *var_ctatgatd_i_slot;
        let mut var_ctatgatd_i_rv: f64 = *var_ctatgatd_i_rv_slot;
        let mut var_ctatstid_i: f64 = *var_ctatstid_i_slot;
        let mut var_ctatstid_i_rv: f64 = *var_ctatstid_i_rv_slot;
        let mut var_fbbtrbotd_i: f64 = *var_fbbtrbotd_i_slot;
        let mut var_fbbtrbotd_i_rv: f64 = *var_fbbtrbotd_i_rv_slot;
        let mut var_fbbtrgatd_i: f64 = *var_fbbtrgatd_i_slot;
        let mut var_fbbtrgatd_i_rv: f64 = *var_fbbtrgatd_i_rv_slot;
        let mut var_fbbtrstid_i: f64 = *var_fbbtrstid_i_slot;
        let mut var_fbbtrstid_i_rv: f64 = *var_fbbtrstid_i_rv_slot;
        let mut var_fcjorgat2d_i: f64 = *var_fcjorgat2d_i_slot;
        let mut var_fcjorgat2d_i_rv: f64 = *var_fcjorgat2d_i_rv_slot;
        let mut var_fjunqd_i: f64 = *var_fjunqd_i_slot;
        let mut var_fjunqd_i_rv: f64 = *var_fjunqd_i_rv_slot;
        let mut var_fpgat2d_i: f64 = *var_fpgat2d_i_slot;
        let mut var_fpgat2d_i_rv: f64 = *var_fpgat2d_i_rv_slot;
        let mut var_fphiggat2d_i: f64 = *var_fphiggat2d_i_slot;
        let mut var_fphiggat2d_i_rv: f64 = *var_fphiggat2d_i_rv_slot;
        let mut var_fvbirgat2d_i: f64 = *var_fvbirgat2d_i_slot;
        let mut var_fvbirgat2d_i_rv: f64 = *var_fvbirgat2d_i_rv_slot;
        let mut var_guard6: f64 = *var_guard6_slot;
        let mut var_guard6_rv: f64 = *var_guard6_rv_slot;
        let mut var_guard7: f64 = *var_guard7_slot;
        let mut var_guard7_rv: f64 = *var_guard7_rv_slot;
        let mut var_mefftatbotd_i: f64 = *var_mefftatbotd_i_slot;
        let mut var_mefftatbotd_i_rv: f64 = *var_mefftatbotd_i_rv_slot;
        let mut var_mefftatgatd_i: f64 = *var_mefftatgatd_i_slot;
        let mut var_mefftatgatd_i_rv: f64 = *var_mefftatgatd_i_rv_slot;
        let mut var_mefftatstid_i: f64 = *var_mefftatstid_i_slot;
        let mut var_mefftatstid_i_rv: f64 = *var_mefftatstid_i_rv_slot;
        let mut var_one_minus_pbot_d: f64 = *var_one_minus_pbot_d_slot;
        let mut var_one_minus_pbot_d_rv: f64 = *var_one_minus_pbot_d_rv_slot;
        let mut var_one_minus_pgat_d: f64 = *var_one_minus_pgat_d_slot;
        let mut var_one_minus_pgat_d_rv: f64 = *var_one_minus_pgat_d_rv_slot;
        let mut var_one_minus_psti_d: f64 = *var_one_minus_psti_d_slot;
        let mut var_one_minus_psti_d_rv: f64 = *var_one_minus_psti_d_rv_slot;
        let mut var_one_over_one_minus_pbot_d: f64 = *var_one_over_one_minus_pbot_d_slot;
        let mut var_one_over_one_minus_pbot_d_rv: f64 = *var_one_over_one_minus_pbot_d_rv_slot;
        let mut var_one_over_one_minus_pgat_d: f64 = *var_one_over_one_minus_pgat_d_slot;
        let mut var_one_over_one_minus_pgat_d_rv: f64 = *var_one_over_one_minus_pgat_d_rv_slot;
        let mut var_one_over_one_minus_psti_d: f64 = *var_one_over_one_minus_psti_d_slot;
        let mut var_one_over_one_minus_psti_d_rv: f64 = *var_one_over_one_minus_psti_d_rv_slot;
        let mut var_pbrbotd_i: f64 = *var_pbrbotd_i_slot;
        let mut var_pbrbotd_i_rv: f64 = *var_pbrbotd_i_rv_slot;
        let mut var_pbrgatd_i: f64 = *var_pbrgatd_i_slot;
        let mut var_pbrgatd_i_rv: f64 = *var_pbrgatd_i_rv_slot;
        let mut var_pbrstid_i: f64 = *var_pbrstid_i_slot;
        let mut var_pbrstid_i_rv: f64 = *var_pbrstid_i_rv_slot;
        let mut var_phigrbot_d: f64 = *var_phigrbot_d_slot;
        let mut var_phigrbot_d_rv: f64 = *var_phigrbot_d_rv_slot;
        let mut var_phigrgat_d: f64 = *var_phigrgat_d_slot;
        let mut var_phigrgat_d_rv: f64 = *var_phigrgat_d_rv_slot;
        let mut var_phigrsti_d: f64 = *var_phigrsti_d_slot;
        let mut var_phigrsti_d_rv: f64 = *var_phigrsti_d_rv_slot;
        let mut var_stfbbtbotd_i: f64 = *var_stfbbtbotd_i_slot;
        let mut var_stfbbtbotd_i_rv: f64 = *var_stfbbtbotd_i_rv_slot;
        let mut var_stfbbtgatd_i: f64 = *var_stfbbtgatd_i_slot;
        let mut var_stfbbtgatd_i_rv: f64 = *var_stfbbtgatd_i_rv_slot;
        let mut var_stfbbtstid_i: f64 = *var_stfbbtstid_i_slot;
        let mut var_stfbbtstid_i_rv: f64 = *var_stfbbtstid_i_rv_slot;
        let mut var_swgat2nd_d: f64 = *var_swgat2nd_d_slot;
        let mut var_swgat2nd_d_rv: f64 = *var_swgat2nd_d_rv_slot;
        let mut var_vbirbotinv_d: f64 = *var_vbirbotinv_d_slot;
        let mut var_vbirbotinv_d_rv: f64 = *var_vbirbotinv_d_rv_slot;
        let mut var_vbirgat2nd_d: f64 = *var_vbirgat2nd_d_slot;
        let mut var_vbirgat2nd_d_rv: f64 = *var_vbirgat2nd_d_rv_slot;
        let mut var_vbirgatinv_d: f64 = *var_vbirgatinv_d_slot;
        let mut var_vbirgatinv_d_rv: f64 = *var_vbirgatinv_d_rv_slot;
        let mut var_vbirstiinv_d: f64 = *var_vbirstiinv_d_slot;
        let mut var_vbirstiinv_d_rv: f64 = *var_vbirstiinv_d_rv_slot;
        let mut var_vbrbotd_i: f64 = *var_vbrbotd_i_slot;
        let mut var_vbrbotd_i_rv: f64 = *var_vbrbotd_i_rv_slot;
        let mut var_vbrgatd_i: f64 = *var_vbrgatd_i_slot;
        let mut var_vbrgatd_i_rv: f64 = *var_vbrgatd_i_rv_slot;
        let mut var_vbrinvbot_d: f64 = *var_vbrinvbot_d_slot;
        let mut var_vbrinvbot_d_rv: f64 = *var_vbrinvbot_d_rv_slot;
        let mut var_vbrinvgat_d: f64 = *var_vbrinvgat_d_slot;
        let mut var_vbrinvgat_d_dn6: f64 = *var_vbrinvgat_d_dn6_slot;
        let mut var_vbrinvgat_d_dn7: f64 = *var_vbrinvgat_d_dn7_slot;
        let mut var_vbrinvgat_d_dn8: f64 = *var_vbrinvgat_d_dn8_slot;
        let mut var_vbrinvgat_d_dn9: f64 = *var_vbrinvgat_d_dn9_slot;
        let mut var_vbrinvgat_d_rv: f64 = *var_vbrinvgat_d_rv_slot;
        let mut var_vbrinvsti_d: f64 = *var_vbrinvsti_d_slot;
        let mut var_vbrinvsti_d_rv: f64 = *var_vbrinvsti_d_rv_slot;
        let mut var_vbrstid_i: f64 = *var_vbrstid_i_slot;
        let mut var_vbrstid_i_rv: f64 = *var_vbrstid_i_rv_slot;
        let mut var_vtrgatd_i: f64 = *var_vtrgatd_i_slot;
        let mut var_vtrgatd_i_rv: f64 = *var_vtrgatd_i_rv_slot;
        let mut var_wdepnulrbot_d: f64 = *var_wdepnulrbot_d_slot;
        let mut var_wdepnulrbot_d_rv: f64 = *var_wdepnulrbot_d_rv_slot;
        let mut var_wdepnulrgat_d: f64 = *var_wdepnulrgat_d_slot;
        let mut var_wdepnulrgat_d_rv: f64 = *var_wdepnulrgat_d_rv_slot;
        let mut var_wdepnulrinvbot_d: f64 = *var_wdepnulrinvbot_d_slot;
        let mut var_wdepnulrinvbot_d_rv: f64 = *var_wdepnulrinvbot_d_rv_slot;
        let mut var_wdepnulrinvgat_d: f64 = *var_wdepnulrinvgat_d_slot;
        let mut var_wdepnulrinvgat_d_rv: f64 = *var_wdepnulrinvgat_d_rv_slot;
        let mut var_wdepnulrinvsti_d: f64 = *var_wdepnulrinvsti_d_slot;
        let mut var_wdepnulrinvsti_d_rv: f64 = *var_wdepnulrinvsti_d_rv_slot;
        let mut var_wdepnulrsti_d: f64 = *var_wdepnulrsti_d_slot;
        let mut var_wdepnulrsti_d_rv: f64 = *var_wdepnulrsti_d_rv_slot;
        let mut var_xjungatd_i: f64 = *var_xjungatd_i_slot;
        let mut var_xjungatd_i_rv: f64 = *var_xjungatd_i_rv_slot;
        let mut var_xjunstid_i: f64 = *var_xjunstid_i_slot;
        let mut var_xjunstid_i_rv: f64 = *var_xjunstid_i_rv_slot;

        let (assign1270_e2101,) = {
    if (var_guard5 == 0.0) {
        (p.p910,)
    } else {
        (var_csrhgatd_i,)
    }
};
        var_csrhgatd_i = assign1270_e2101;
        var_csrhgatd_i_rv = 0.0;

        let (assign1280_e2106,) = {
    if (var_guard5 == 0.0) {
        (p.p911,)
    } else {
        (var_xjunstid_i,)
    }
};
        var_xjunstid_i = assign1280_e2106;
        var_xjunstid_i_rv = 0.0;

        let (assign1290_e2111,) = {
    if (var_guard5 == 0.0) {
        (p.p912,)
    } else {
        (var_xjungatd_i,)
    }
};
        var_xjungatd_i = assign1290_e2111;
        var_xjungatd_i_rv = 0.0;

        let (assign1300_e2116,) = {
    if (var_guard5 == 0.0) {
        (p.p913,)
    } else {
        (var_ctatbotd_i,)
    }
};
        var_ctatbotd_i = assign1300_e2116;
        var_ctatbotd_i_rv = 0.0;

        let (assign1310_e2121,) = {
    if (var_guard5 == 0.0) {
        (p.p914,)
    } else {
        (var_ctatstid_i,)
    }
};
        var_ctatstid_i = assign1310_e2121;
        var_ctatstid_i_rv = 0.0;

        let (assign1320_e2126,) = {
    if (var_guard5 == 0.0) {
        (p.p915,)
    } else {
        (var_ctatgatd_i,)
    }
};
        var_ctatgatd_i = assign1320_e2126;
        var_ctatgatd_i_rv = 0.0;

        let (assign1330_e2131,) = {
    if (var_guard5 == 0.0) {
        (p.p916,)
    } else {
        (var_mefftatbotd_i,)
    }
};
        var_mefftatbotd_i = assign1330_e2131;
        var_mefftatbotd_i_rv = 0.0;

        let (assign1340_e2136,) = {
    if (var_guard5 == 0.0) {
        (p.p917,)
    } else {
        (var_mefftatstid_i,)
    }
};
        var_mefftatstid_i = assign1340_e2136;
        var_mefftatstid_i_rv = 0.0;

        let (assign1350_e2141,) = {
    if (var_guard5 == 0.0) {
        (p.p918,)
    } else {
        (var_mefftatgatd_i,)
    }
};
        var_mefftatgatd_i = assign1350_e2141;
        var_mefftatgatd_i_rv = 0.0;

        let (assign1360_e2146,) = {
    if (var_guard5 == 0.0) {
        (p.p919,)
    } else {
        (var_cbbtbotd_i,)
    }
};
        var_cbbtbotd_i = assign1360_e2146;
        var_cbbtbotd_i_rv = 0.0;

        let (assign1370_e2151,) = {
    if (var_guard5 == 0.0) {
        (p.p920,)
    } else {
        (var_cbbtstid_i,)
    }
};
        var_cbbtstid_i = assign1370_e2151;
        var_cbbtstid_i_rv = 0.0;

        let (assign1380_e2156,) = {
    if (var_guard5 == 0.0) {
        (p.p921,)
    } else {
        (var_cbbtgatd_i,)
    }
};
        var_cbbtgatd_i = assign1380_e2156;
        var_cbbtgatd_i_rv = 0.0;

        let (assign1390_e2161,) = {
    if (var_guard5 == 0.0) {
        (p.p922,)
    } else {
        (var_fbbtrbotd_i,)
    }
};
        var_fbbtrbotd_i = assign1390_e2161;
        var_fbbtrbotd_i_rv = 0.0;

        let (assign1400_e2166,) = {
    if (var_guard5 == 0.0) {
        (p.p923,)
    } else {
        (var_fbbtrstid_i,)
    }
};
        var_fbbtrstid_i = assign1400_e2166;
        var_fbbtrstid_i_rv = 0.0;

        let (assign1410_e2171,) = {
    if (var_guard5 == 0.0) {
        (p.p924,)
    } else {
        (var_fbbtrgatd_i,)
    }
};
        var_fbbtrgatd_i = assign1410_e2171;
        var_fbbtrgatd_i_rv = 0.0;

        let (assign1420_e2176,) = {
    if (var_guard5 == 0.0) {
        (p.p925,)
    } else {
        (var_stfbbtbotd_i,)
    }
};
        var_stfbbtbotd_i = assign1420_e2176;
        var_stfbbtbotd_i_rv = 0.0;

        let (assign1430_e2181,) = {
    if (var_guard5 == 0.0) {
        (p.p926,)
    } else {
        (var_stfbbtstid_i,)
    }
};
        var_stfbbtstid_i = assign1430_e2181;
        var_stfbbtstid_i_rv = 0.0;

        let (assign1440_e2186,) = {
    if (var_guard5 == 0.0) {
        (p.p927,)
    } else {
        (var_stfbbtgatd_i,)
    }
};
        var_stfbbtgatd_i = assign1440_e2186;
        var_stfbbtgatd_i_rv = 0.0;

        let (assign1450_e2191,) = {
    if (var_guard5 == 0.0) {
        (p.p928,)
    } else {
        (var_vbrbotd_i,)
    }
};
        var_vbrbotd_i = assign1450_e2191;
        var_vbrbotd_i_rv = 0.0;

        let (assign1460_e2196,) = {
    if (var_guard5 == 0.0) {
        (p.p929,)
    } else {
        (var_vbrstid_i,)
    }
};
        var_vbrstid_i = assign1460_e2196;
        var_vbrstid_i_rv = 0.0;

        let (assign1470_e2201,) = {
    if (var_guard5 == 0.0) {
        (p.p930,)
    } else {
        (var_vbrgatd_i,)
    }
};
        var_vbrgatd_i = assign1470_e2201;
        var_vbrgatd_i_rv = 0.0;

        let (assign1480_e2206,) = {
    if (var_guard5 == 0.0) {
        (p.p931,)
    } else {
        (var_pbrbotd_i,)
    }
};
        var_pbrbotd_i = assign1480_e2206;
        var_pbrbotd_i_rv = 0.0;

        let (assign1490_e2211,) = {
    if (var_guard5 == 0.0) {
        (p.p932,)
    } else {
        (var_pbrstid_i,)
    }
};
        var_pbrstid_i = assign1490_e2211;
        var_pbrstid_i_rv = 0.0;

        let (assign1500_e2216,) = {
    if (var_guard5 == 0.0) {
        (p.p933,)
    } else {
        (var_pbrgatd_i,)
    }
};
        var_pbrgatd_i = assign1500_e2216;
        var_pbrgatd_i_rv = 0.0;

        let (assign1520_e2226,) = {
    if (var_guard5 == 0.0) {
        (p.p948,)
    } else {
        (var_fjunqd_i,)
    }
};
        var_fjunqd_i = assign1520_e2226;
        var_fjunqd_i_rv = 0.0;

        let (assign1530_e2231,) = {
    if (var_guard5 == 0.0) {
        (p.p940,)
    } else {
        (var_advbrgatd_i,)
    }
};
        var_advbrgatd_i = assign1530_e2231;
        var_advbrgatd_i_rv = 0.0;

        let (assign1540_e2236,) = {
    if (var_guard5 == 0.0) {
        (p.p941,)
    } else {
        (var_bdvbrgatd_i,)
    }
};
        var_bdvbrgatd_i = assign1540_e2236;
        var_bdvbrgatd_i_rv = 0.0;

        let (assign1550_e2241,) = {
    if (var_guard5 == 0.0) {
        (p.p942,)
    } else {
        (var_adbbtgatd_i,)
    }
};
        var_adbbtgatd_i = assign1550_e2241;
        var_adbbtgatd_i_rv = 0.0;

        let (assign1560_e2246,) = {
    if (var_guard5 == 0.0) {
        (p.p943,)
    } else {
        (var_bdbbtgatd_i,)
    }
};
        var_bdbbtgatd_i = assign1560_e2246;
        var_bdbbtgatd_i_rv = 0.0;

        let (assign1570_e2251,) = {
    if (var_guard5 == 0.0) {
        (p.p934,)
    } else {
        (var_fcjorgat2d_i,)
    }
};
        var_fcjorgat2d_i = assign1570_e2251;
        var_fcjorgat2d_i_rv = 0.0;

        let (assign1580_e2256,) = {
    if (var_guard5 == 0.0) {
        (p.p935,)
    } else {
        (var_fvbirgat2d_i,)
    }
};
        var_fvbirgat2d_i = assign1580_e2256;
        var_fvbirgat2d_i_rv = 0.0;

        let (assign1590_e2261,) = {
    if (var_guard5 == 0.0) {
        (p.p936,)
    } else {
        (var_fpgat2d_i,)
    }
};
        var_fpgat2d_i = assign1590_e2261;
        var_fpgat2d_i_rv = 0.0;

        let (assign1600_e2266,) = {
    if (var_guard5 == 0.0) {
        (p.p937,)
    } else {
        (var_fphiggat2d_i,)
    }
};
        var_fphiggat2d_i = assign1600_e2266;
        var_fphiggat2d_i_rv = 0.0;

        let (assign1610_e2271,) = {
    if (var_guard5 == 0.0) {
        (p.p938,)
    } else {
        (var_vtrgatd_i,)
    }
};
        var_vtrgatd_i = assign1610_e2271;
        var_vtrgatd_i_rv = 0.0;

        let (assign1620_e2276,) = {
    if (var_guard5 == 0.0) {
        (p.p939,)
    } else {
        (var_anugatd_i,)
    }
};
        var_anugatd_i = assign1620_e2276;
        var_anugatd_i_rv = 0.0;

        let assign1630_e2279: f64 = (var_phigbotd_i + var_deltaphigr);
        var_phigrbot_d = assign1630_e2279;
        var_phigrbot_d_rv = 0.0;

        let assign1640_e2282: f64 = (var_phigstid_i + var_deltaphigr);
        var_phigrsti_d = assign1640_e2282;
        var_phigrsti_d_rv = 0.0;

        let assign1650_e2285: f64 = (var_phiggatd_i + var_deltaphigr);
        var_phigrgat_d = assign1650_e2285;
        var_phigrgat_d_rv = 0.0;

        let assign1660_e2288: f64 = (1.0 - var_pbotd_i);
        var_one_minus_pbot_d = assign1660_e2288;
        var_one_minus_pbot_d_rv = 0.0;

        let assign1670_e2291: f64 = (1.0 - var_pstid_i);
        var_one_minus_psti_d = assign1670_e2291;
        var_one_minus_psti_d_rv = 0.0;

        let assign1680_e2294: f64 = (1.0 - var_pgatd_i);
        var_one_minus_pgat_d = assign1680_e2294;
        var_one_minus_pgat_d_rv = 0.0;

        let assign1690_e2297: f64 = (1.0 / var_one_minus_pbot_d);
        var_one_over_one_minus_pbot_d = assign1690_e2297;
        var_one_over_one_minus_pbot_d_rv = 0.0;

        let assign1700_e2300: f64 = (1.0 / var_one_minus_psti_d);
        var_one_over_one_minus_psti_d = assign1700_e2300;
        var_one_over_one_minus_psti_d_rv = 0.0;

        let assign1710_e2303: f64 = (1.0 / var_one_minus_pgat_d);
        var_one_over_one_minus_pgat_d = assign1710_e2303;
        var_one_over_one_minus_pgat_d_rv = 0.0;

        let assign1720_e2306: f64 = (var_epssi / var_cjorbotd_i);
        var_wdepnulrbot_d = assign1720_e2306;
        var_wdepnulrbot_d_rv = 0.0;

        let assign1730_e2309: f64 = (var_xjunstid_i * var_epssi);
        let assign1730_e2311: f64 = (assign1730_e2309 / var_cjorstid_i);
        var_wdepnulrsti_d = assign1730_e2311;
        var_wdepnulrsti_d_rv = 0.0;

        let assign1740_e2314: f64 = (var_xjungatd_i * var_epssi);
        let assign1740_e2316: f64 = (assign1740_e2314 / var_cjorgatd_i);
        var_wdepnulrgat_d = assign1740_e2316;
        var_wdepnulrgat_d_rv = 0.0;

        let assign1750_e2319: f64 = (1.0 / var_wdepnulrbot_d);
        var_wdepnulrinvbot_d = assign1750_e2319;
        var_wdepnulrinvbot_d_rv = 0.0;

        let assign1760_e2322: f64 = (1.0 / var_wdepnulrsti_d);
        var_wdepnulrinvsti_d = assign1760_e2322;
        var_wdepnulrinvsti_d_rv = 0.0;

        let assign1770_e2325: f64 = (1.0 / var_wdepnulrgat_d);
        var_wdepnulrinvgat_d = assign1770_e2325;
        var_wdepnulrinvgat_d_rv = 0.0;

        let assign1780_e2328: f64 = (1.0 / var_vbirbotd_i);
        var_vbirbotinv_d = assign1780_e2328;
        var_vbirbotinv_d_rv = 0.0;

        let assign1790_e2331: f64 = (1.0 / var_vbirstid_i);
        var_vbirstiinv_d = assign1790_e2331;
        var_vbirstiinv_d_rv = 0.0;

        let assign1800_e2334: f64 = (1.0 / var_vbirgatd_i);
        var_vbirgatinv_d = assign1800_e2334;
        var_vbirgatinv_d_rv = 0.0;

        let assign1840_e2358: f64 = (1.0 / var_vbrbotd_i);
        var_vbrinvbot_d = assign1840_e2358;
        var_vbrinvbot_d_rv = 0.0;

        let assign1850_e2361: f64 = (1.0 / var_vbrstid_i);
        var_vbrinvsti_d = assign1850_e2361;
        var_vbrinvsti_d_rv = 0.0;

        let assign1860_e2364: f64 = (1.0 / var_vbrgatd_i);
        var_vbrinvgat_d = assign1860_e2364;
        var_vbrinvgat_d_dn6 = 0.0;
        var_vbrinvgat_d_dn7 = 0.0;
        var_vbrinvgat_d_dn8 = 0.0;
        var_vbrinvgat_d_dn9 = 0.0;
        var_vbrinvgat_d_rv = 0.0;

        let assign1900_e2421: f64 = if ((((var_fcjorgat2d_i != 1.0) || (var_fvbirgat2d_i != 1.0)) || (var_fpgat2d_i != 1.0)) || (var_fphiggat2d_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard6 = assign1900_e2421;
        var_guard6_rv = 0.0;

        let (assign1910_e2425,) = {
    if (var_guard6 != 0.0) {
        (1.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign1910_e2425;
        var_swgat2nd_d_rv = 0.0;

        let (assign1920_e2430,) = {
    if (var_guard6 == 0.0) {
        (0.0,)
    } else {
        (var_swgat2nd_d,)
    }
};
        var_swgat2nd_d = assign1920_e2430;
        var_swgat2nd_d_rv = 0.0;

        let assign1930_e2433: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard7 = assign1930_e2433;
        var_guard7_rv = 0.0;

        let (assign1940_e2446,) = {
    if (var_guard7 != 0.0) {
        let assign1940_e2437: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
        let (assign1940_e2444,) = {
            if (assign1940_e2437 > 1e-18) {
                let assign1940_e2442: f64 = (var_cjorgatd_i * var_fcjorgat2d_i);
                (assign1940_e2442,)
            } else {
                (1e-18,)
            }
        };
        (assign1940_e2444,)
    } else {
        (var_cjorgat2nd_d,)
    }
};
        var_cjorgat2nd_d = assign1940_e2446;
        var_cjorgat2nd_d_rv = 0.0;

        let (assign1950_e2459,) = {
    if (var_guard7 != 0.0) {
        let assign1950_e2450: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
        let (assign1950_e2457,) = {
            if (assign1950_e2450 > 0.05) {
                let assign1950_e2455: f64 = (var_vbirgatd_i * var_fvbirgat2d_i);
                (assign1950_e2455,)
            } else {
                (0.05,)
            }
        };
        (assign1950_e2457,)
    } else {
        (var_vbirgat2nd_d,)
    }
};
        var_vbirgat2nd_d = assign1950_e2459;
        var_vbirgat2nd_d_rv = 0.0;

        *var_adbbtgatd_i_slot = var_adbbtgatd_i;
        *var_adbbtgatd_i_rv_slot = var_adbbtgatd_i_rv;
        *var_advbrgatd_i_slot = var_advbrgatd_i;
        *var_advbrgatd_i_rv_slot = var_advbrgatd_i_rv;
        *var_anugatd_i_slot = var_anugatd_i;
        *var_anugatd_i_rv_slot = var_anugatd_i_rv;
        *var_bdbbtgatd_i_slot = var_bdbbtgatd_i;
        *var_bdbbtgatd_i_rv_slot = var_bdbbtgatd_i_rv;
        *var_bdvbrgatd_i_slot = var_bdvbrgatd_i;
        *var_bdvbrgatd_i_rv_slot = var_bdvbrgatd_i_rv;
        *var_cbbtbotd_i_slot = var_cbbtbotd_i;
        *var_cbbtbotd_i_rv_slot = var_cbbtbotd_i_rv;
        *var_cbbtgatd_i_slot = var_cbbtgatd_i;
        *var_cbbtgatd_i_rv_slot = var_cbbtgatd_i_rv;
        *var_cbbtstid_i_slot = var_cbbtstid_i;
        *var_cbbtstid_i_rv_slot = var_cbbtstid_i_rv;
        *var_cjorgat2nd_d_slot = var_cjorgat2nd_d;
        *var_cjorgat2nd_d_rv_slot = var_cjorgat2nd_d_rv;
        *var_csrhgatd_i_slot = var_csrhgatd_i;
        *var_csrhgatd_i_rv_slot = var_csrhgatd_i_rv;
        *var_ctatbotd_i_slot = var_ctatbotd_i;
        *var_ctatbotd_i_rv_slot = var_ctatbotd_i_rv;
        *var_ctatgatd_i_slot = var_ctatgatd_i;
        *var_ctatgatd_i_rv_slot = var_ctatgatd_i_rv;
        *var_ctatstid_i_slot = var_ctatstid_i;
        *var_ctatstid_i_rv_slot = var_ctatstid_i_rv;
        *var_fbbtrbotd_i_slot = var_fbbtrbotd_i;
        *var_fbbtrbotd_i_rv_slot = var_fbbtrbotd_i_rv;
        *var_fbbtrgatd_i_slot = var_fbbtrgatd_i;
        *var_fbbtrgatd_i_rv_slot = var_fbbtrgatd_i_rv;
        *var_fbbtrstid_i_slot = var_fbbtrstid_i;
        *var_fbbtrstid_i_rv_slot = var_fbbtrstid_i_rv;
        *var_fcjorgat2d_i_slot = var_fcjorgat2d_i;
        *var_fcjorgat2d_i_rv_slot = var_fcjorgat2d_i_rv;
        *var_fjunqd_i_slot = var_fjunqd_i;
        *var_fjunqd_i_rv_slot = var_fjunqd_i_rv;
        *var_fpgat2d_i_slot = var_fpgat2d_i;
        *var_fpgat2d_i_rv_slot = var_fpgat2d_i_rv;
        *var_fphiggat2d_i_slot = var_fphiggat2d_i;
        *var_fphiggat2d_i_rv_slot = var_fphiggat2d_i_rv;
        *var_fvbirgat2d_i_slot = var_fvbirgat2d_i;
        *var_fvbirgat2d_i_rv_slot = var_fvbirgat2d_i_rv;
        *var_guard6_slot = var_guard6;
        *var_guard6_rv_slot = var_guard6_rv;
        *var_guard7_slot = var_guard7;
        *var_guard7_rv_slot = var_guard7_rv;
        *var_mefftatbotd_i_slot = var_mefftatbotd_i;
        *var_mefftatbotd_i_rv_slot = var_mefftatbotd_i_rv;
        *var_mefftatgatd_i_slot = var_mefftatgatd_i;
        *var_mefftatgatd_i_rv_slot = var_mefftatgatd_i_rv;
        *var_mefftatstid_i_slot = var_mefftatstid_i;
        *var_mefftatstid_i_rv_slot = var_mefftatstid_i_rv;
        *var_one_minus_pbot_d_slot = var_one_minus_pbot_d;
        *var_one_minus_pbot_d_rv_slot = var_one_minus_pbot_d_rv;
        *var_one_minus_pgat_d_slot = var_one_minus_pgat_d;
        *var_one_minus_pgat_d_rv_slot = var_one_minus_pgat_d_rv;
        *var_one_minus_psti_d_slot = var_one_minus_psti_d;
        *var_one_minus_psti_d_rv_slot = var_one_minus_psti_d_rv;
        *var_one_over_one_minus_pbot_d_slot = var_one_over_one_minus_pbot_d;
        *var_one_over_one_minus_pbot_d_rv_slot = var_one_over_one_minus_pbot_d_rv;
        *var_one_over_one_minus_pgat_d_slot = var_one_over_one_minus_pgat_d;
        *var_one_over_one_minus_pgat_d_rv_slot = var_one_over_one_minus_pgat_d_rv;
        *var_one_over_one_minus_psti_d_slot = var_one_over_one_minus_psti_d;
        *var_one_over_one_minus_psti_d_rv_slot = var_one_over_one_minus_psti_d_rv;
        *var_pbrbotd_i_slot = var_pbrbotd_i;
        *var_pbrbotd_i_rv_slot = var_pbrbotd_i_rv;
        *var_pbrgatd_i_slot = var_pbrgatd_i;
        *var_pbrgatd_i_rv_slot = var_pbrgatd_i_rv;
        *var_pbrstid_i_slot = var_pbrstid_i;
        *var_pbrstid_i_rv_slot = var_pbrstid_i_rv;
        *var_phigrbot_d_slot = var_phigrbot_d;
        *var_phigrbot_d_rv_slot = var_phigrbot_d_rv;
        *var_phigrgat_d_slot = var_phigrgat_d;
        *var_phigrgat_d_rv_slot = var_phigrgat_d_rv;
        *var_phigrsti_d_slot = var_phigrsti_d;
        *var_phigrsti_d_rv_slot = var_phigrsti_d_rv;
        *var_stfbbtbotd_i_slot = var_stfbbtbotd_i;
        *var_stfbbtbotd_i_rv_slot = var_stfbbtbotd_i_rv;
        *var_stfbbtgatd_i_slot = var_stfbbtgatd_i;
        *var_stfbbtgatd_i_rv_slot = var_stfbbtgatd_i_rv;
        *var_stfbbtstid_i_slot = var_stfbbtstid_i;
        *var_stfbbtstid_i_rv_slot = var_stfbbtstid_i_rv;
        *var_swgat2nd_d_slot = var_swgat2nd_d;
        *var_swgat2nd_d_rv_slot = var_swgat2nd_d_rv;
        *var_vbirbotinv_d_slot = var_vbirbotinv_d;
        *var_vbirbotinv_d_rv_slot = var_vbirbotinv_d_rv;
        *var_vbirgat2nd_d_slot = var_vbirgat2nd_d;
        *var_vbirgat2nd_d_rv_slot = var_vbirgat2nd_d_rv;
        *var_vbirgatinv_d_slot = var_vbirgatinv_d;
        *var_vbirgatinv_d_rv_slot = var_vbirgatinv_d_rv;
        *var_vbirstiinv_d_slot = var_vbirstiinv_d;
        *var_vbirstiinv_d_rv_slot = var_vbirstiinv_d_rv;
        *var_vbrbotd_i_slot = var_vbrbotd_i;
        *var_vbrbotd_i_rv_slot = var_vbrbotd_i_rv;
        *var_vbrgatd_i_slot = var_vbrgatd_i;
        *var_vbrgatd_i_rv_slot = var_vbrgatd_i_rv;
        *var_vbrinvbot_d_slot = var_vbrinvbot_d;
        *var_vbrinvbot_d_rv_slot = var_vbrinvbot_d_rv;
        *var_vbrinvgat_d_slot = var_vbrinvgat_d;
        *var_vbrinvgat_d_dn6_slot = var_vbrinvgat_d_dn6;
        *var_vbrinvgat_d_dn7_slot = var_vbrinvgat_d_dn7;
        *var_vbrinvgat_d_dn8_slot = var_vbrinvgat_d_dn8;
        *var_vbrinvgat_d_dn9_slot = var_vbrinvgat_d_dn9;
        *var_vbrinvgat_d_rv_slot = var_vbrinvgat_d_rv;
        *var_vbrinvsti_d_slot = var_vbrinvsti_d;
        *var_vbrinvsti_d_rv_slot = var_vbrinvsti_d_rv;
        *var_vbrstid_i_slot = var_vbrstid_i;
        *var_vbrstid_i_rv_slot = var_vbrstid_i_rv;
        *var_vtrgatd_i_slot = var_vtrgatd_i;
        *var_vtrgatd_i_rv_slot = var_vtrgatd_i_rv;
        *var_wdepnulrbot_d_slot = var_wdepnulrbot_d;
        *var_wdepnulrbot_d_rv_slot = var_wdepnulrbot_d_rv;
        *var_wdepnulrgat_d_slot = var_wdepnulrgat_d;
        *var_wdepnulrgat_d_rv_slot = var_wdepnulrgat_d_rv;
        *var_wdepnulrinvbot_d_slot = var_wdepnulrinvbot_d;
        *var_wdepnulrinvbot_d_rv_slot = var_wdepnulrinvbot_d_rv;
        *var_wdepnulrinvgat_d_slot = var_wdepnulrinvgat_d;
        *var_wdepnulrinvgat_d_rv_slot = var_wdepnulrinvgat_d_rv;
        *var_wdepnulrinvsti_d_slot = var_wdepnulrinvsti_d;
        *var_wdepnulrinvsti_d_rv_slot = var_wdepnulrinvsti_d_rv;
        *var_wdepnulrsti_d_slot = var_wdepnulrsti_d;
        *var_wdepnulrsti_d_rv_slot = var_wdepnulrsti_d_rv;
        *var_xjungatd_i_slot = var_xjungatd_i;
        *var_xjungatd_i_rv_slot = var_xjungatd_i_rv;
        *var_xjunstid_i_slot = var_xjunstid_i;
        *var_xjunstid_i_rv_slot = var_xjunstid_i_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_cjorgat2nd: f64,
        var_deltaphigr: f64,
        var_fpgat2d_i: f64,
        var_fphiggat2d_i: f64,
        var_guard7: f64,
        var_kbol_over_qele: f64,
        var_one_over_one_minus_pbot: f64,
        var_one_over_one_minus_pgat: f64,
        var_one_over_one_minus_psti: f64,
        var_pgat2nd: f64,
        var_pgatd_i: f64,
        var_phiggat2nd: f64,
        var_phiggatd_i: f64,
        var_phigrbot: f64,
        var_phigrgat: f64,
        var_phigrgat2nd: f64,
        var_phigrsti: f64,
        var_phitrinv: f64,
        var_swgat2nd: f64,
        var_tkr: f64,
        var_tkr_1: f64,
        var_vbirgat2nd: f64,
        var_atatbot_slot: &mut f64,
        var_atatbot_rv_slot: &mut f64,
        var_atatgat_slot: &mut f64,
        var_atatgat_rv_slot: &mut f64,
        var_atatsti_slot: &mut f64,
        var_atatsti_rv_slot: &mut f64,
        var_auxt_slot: &mut f64,
        var_auxt_rv_slot: &mut f64,
        var_btatpartbot_slot: &mut f64,
        var_btatpartbot_rv_slot: &mut f64,
        var_btatpartgat_slot: &mut f64,
        var_btatpartgat_rv_slot: &mut f64,
        var_btatpartsti_slot: &mut f64,
        var_btatpartsti_rv_slot: &mut f64,
        var_cjobot_slot: &mut f64,
        var_cjobot_rv_slot: &mut f64,
        var_cjogat_slot: &mut f64,
        var_cjogat2nd_slot: &mut f64,
        var_cjogat2nd_rv_slot: &mut f64,
        var_cjogat_rv_slot: &mut f64,
        var_cjosti_slot: &mut f64,
        var_cjosti_rv_slot: &mut f64,
        var_delta_slot: &mut f64,
        var_delta_rv_slot: &mut f64,
        var_deltaebot_slot: &mut f64,
        var_deltaebot_rv_slot: &mut f64,
        var_deltaegat_slot: &mut f64,
        var_deltaegat_rv_slot: &mut f64,
        var_deltaesti_slot: &mut f64,
        var_deltaesti_rv_slot: &mut f64,
        var_deltaphigd_slot: &mut f64,
        var_deltaphigd_rv_slot: &mut f64,
        var_fbbtbot_slot: &mut f64,
        var_fbbtbot_rv_slot: &mut f64,
        var_fbbtgat_slot: &mut f64,
        var_fbbtgat_dn6_slot: &mut f64,
        var_fbbtgat_dn7_slot: &mut f64,
        var_fbbtgat_dn8_slot: &mut f64,
        var_fbbtgat_dn9_slot: &mut f64,
        var_fbbtgat_rv_slot: &mut f64,
        var_fbbtsti_slot: &mut f64,
        var_fbbtsti_rv_slot: &mut f64,
        var_ftdbot_slot: &mut f64,
        var_ftdbot_rv_slot: &mut f64,
        var_ftdgat_slot: &mut f64,
        var_ftdgat2nd_slot: &mut f64,
        var_ftdgat2nd_rv_slot: &mut f64,
        var_ftdgat_rv_slot: &mut f64,
        var_ftdsti_slot: &mut f64,
        var_ftdsti_rv_slot: &mut f64,
        var_guard27_slot: &mut f64,
        var_guard27_rv_slot: &mut f64,
        var_idsatbot_slot: &mut f64,
        var_idsatbot_rv_slot: &mut f64,
        var_idsatgat_slot: &mut f64,
        var_idsatgat_rv_slot: &mut f64,
        var_idsatsti_slot: &mut f64,
        var_idsatsti_rv_slot: &mut f64,
        var_inv_phita_slot: &mut f64,
        var_inv_phita_rv_slot: &mut f64,
        var_one_minus_pgat2nd_d_slot: &mut f64,
        var_one_minus_pgat2nd_d_rv_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_d_slot: &mut f64,
        var_one_over_one_minus_pgat2nd_d_rv_slot: &mut f64,
        var_pgat2nd_d_slot: &mut f64,
        var_pgat2nd_d_rv_slot: &mut f64,
        var_phigdbot_slot: &mut f64,
        var_phigdbot_rv_slot: &mut f64,
        var_phigdgat_slot: &mut f64,
        var_phigdgat2nd_slot: &mut f64,
        var_phigdgat2nd_rv_slot: &mut f64,
        var_phigdgat_rv_slot: &mut f64,
        var_phigdsti_slot: &mut f64,
        var_phigdsti_rv_slot: &mut f64,
        var_phiggat2nd_d_slot: &mut f64,
        var_phiggat2nd_d_rv_slot: &mut f64,
        var_phigrgat2nd_d_slot: &mut f64,
        var_phigrgat2nd_d_rv_slot: &mut f64,
        var_phita_slot: &mut f64,
        var_phita_rv_slot: &mut f64,
        var_phitd_slot: &mut f64,
        var_phitd_rv_slot: &mut f64,
        var_phitdinv_slot: &mut f64,
        var_phitdinv_rv_slot: &mut f64,
        var_qpref2bot_slot: &mut f64,
        var_qpref2bot_rv_slot: &mut f64,
        var_qpref2gat_slot: &mut f64,
        var_qpref2gat_rv_slot: &mut f64,
        var_qpref2sti_slot: &mut f64,
        var_qpref2sti_rv_slot: &mut f64,
        var_qprefbot_slot: &mut f64,
        var_qprefbot_rv_slot: &mut f64,
        var_qprefgat_slot: &mut f64,
        var_qprefgat_rv_slot: &mut f64,
        var_qprefsti_slot: &mut f64,
        var_qprefsti_rv_slot: &mut f64,
        var_rta_slot: &mut f64,
        var_rta_rv_slot: &mut f64,
        var_tka_slot: &mut f64,
        var_tka_rv_slot: &mut f64,
        var_tkd_1_slot: &mut f64,
        var_tkd_1_rv_slot: &mut f64,
        var_ubibot_slot: &mut f64,
        var_ubibot_rv_slot: &mut f64,
        var_ubigat_slot: &mut f64,
        var_ubigat2nd_slot: &mut f64,
        var_ubigat2nd_rv_slot: &mut f64,
        var_ubigat_rv_slot: &mut f64,
        var_ubisti_slot: &mut f64,
        var_ubisti_rv_slot: &mut f64,
        var_vbibot_slot: &mut f64,
        var_vbibot_rv_slot: &mut f64,
        var_vbigat_slot: &mut f64,
        var_vbigat2nd_slot: &mut f64,
        var_vbigat2nd_rv_slot: &mut f64,
        var_vbigat_rv_slot: &mut f64,
        var_vbiinvbot_slot: &mut f64,
        var_vbiinvbot_rv_slot: &mut f64,
        var_vbiinvgat_slot: &mut f64,
        var_vbiinvgat2nd_slot: &mut f64,
        var_vbiinvgat2nd_rv_slot: &mut f64,
        var_vbiinvgat_rv_slot: &mut f64,
        var_vbiinvsti_slot: &mut f64,
        var_vbiinvsti_rv_slot: &mut f64,
        var_vbisti_slot: &mut f64,
        var_vbisti_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_atatbot: f64 = *var_atatbot_slot;
        let mut var_atatbot_rv: f64 = *var_atatbot_rv_slot;
        let mut var_atatgat: f64 = *var_atatgat_slot;
        let mut var_atatgat_rv: f64 = *var_atatgat_rv_slot;
        let mut var_atatsti: f64 = *var_atatsti_slot;
        let mut var_atatsti_rv: f64 = *var_atatsti_rv_slot;
        let mut var_auxt: f64 = *var_auxt_slot;
        let mut var_auxt_rv: f64 = *var_auxt_rv_slot;
        let mut var_btatpartbot: f64 = *var_btatpartbot_slot;
        let mut var_btatpartbot_rv: f64 = *var_btatpartbot_rv_slot;
        let mut var_btatpartgat: f64 = *var_btatpartgat_slot;
        let mut var_btatpartgat_rv: f64 = *var_btatpartgat_rv_slot;
        let mut var_btatpartsti: f64 = *var_btatpartsti_slot;
        let mut var_btatpartsti_rv: f64 = *var_btatpartsti_rv_slot;
        let mut var_cjobot: f64 = *var_cjobot_slot;
        let mut var_cjobot_rv: f64 = *var_cjobot_rv_slot;
        let mut var_cjogat: f64 = *var_cjogat_slot;
        let mut var_cjogat2nd: f64 = *var_cjogat2nd_slot;
        let mut var_cjogat2nd_rv: f64 = *var_cjogat2nd_rv_slot;
        let mut var_cjogat_rv: f64 = *var_cjogat_rv_slot;
        let mut var_cjosti: f64 = *var_cjosti_slot;
        let mut var_cjosti_rv: f64 = *var_cjosti_rv_slot;
        let mut var_delta: f64 = *var_delta_slot;
        let mut var_delta_rv: f64 = *var_delta_rv_slot;
        let mut var_deltaebot: f64 = *var_deltaebot_slot;
        let mut var_deltaebot_rv: f64 = *var_deltaebot_rv_slot;
        let mut var_deltaegat: f64 = *var_deltaegat_slot;
        let mut var_deltaegat_rv: f64 = *var_deltaegat_rv_slot;
        let mut var_deltaesti: f64 = *var_deltaesti_slot;
        let mut var_deltaesti_rv: f64 = *var_deltaesti_rv_slot;
        let mut var_deltaphigd: f64 = *var_deltaphigd_slot;
        let mut var_deltaphigd_rv: f64 = *var_deltaphigd_rv_slot;
        let mut var_fbbtbot: f64 = *var_fbbtbot_slot;
        let mut var_fbbtbot_rv: f64 = *var_fbbtbot_rv_slot;
        let mut var_fbbtgat: f64 = *var_fbbtgat_slot;
        let mut var_fbbtgat_dn6: f64 = *var_fbbtgat_dn6_slot;
        let mut var_fbbtgat_dn7: f64 = *var_fbbtgat_dn7_slot;
        let mut var_fbbtgat_dn8: f64 = *var_fbbtgat_dn8_slot;
        let mut var_fbbtgat_dn9: f64 = *var_fbbtgat_dn9_slot;
        let mut var_fbbtgat_rv: f64 = *var_fbbtgat_rv_slot;
        let mut var_fbbtsti: f64 = *var_fbbtsti_slot;
        let mut var_fbbtsti_rv: f64 = *var_fbbtsti_rv_slot;
        let mut var_ftdbot: f64 = *var_ftdbot_slot;
        let mut var_ftdbot_rv: f64 = *var_ftdbot_rv_slot;
        let mut var_ftdgat: f64 = *var_ftdgat_slot;
        let mut var_ftdgat2nd: f64 = *var_ftdgat2nd_slot;
        let mut var_ftdgat2nd_rv: f64 = *var_ftdgat2nd_rv_slot;
        let mut var_ftdgat_rv: f64 = *var_ftdgat_rv_slot;
        let mut var_ftdsti: f64 = *var_ftdsti_slot;
        let mut var_ftdsti_rv: f64 = *var_ftdsti_rv_slot;
        let mut var_guard27: f64 = *var_guard27_slot;
        let mut var_guard27_rv: f64 = *var_guard27_rv_slot;
        let mut var_idsatbot: f64 = *var_idsatbot_slot;
        let mut var_idsatbot_rv: f64 = *var_idsatbot_rv_slot;
        let mut var_idsatgat: f64 = *var_idsatgat_slot;
        let mut var_idsatgat_rv: f64 = *var_idsatgat_rv_slot;
        let mut var_idsatsti: f64 = *var_idsatsti_slot;
        let mut var_idsatsti_rv: f64 = *var_idsatsti_rv_slot;
        let mut var_inv_phita: f64 = *var_inv_phita_slot;
        let mut var_inv_phita_rv: f64 = *var_inv_phita_rv_slot;
        let mut var_one_minus_pgat2nd_d: f64 = *var_one_minus_pgat2nd_d_slot;
        let mut var_one_minus_pgat2nd_d_rv: f64 = *var_one_minus_pgat2nd_d_rv_slot;
        let mut var_one_over_one_minus_pgat2nd_d: f64 = *var_one_over_one_minus_pgat2nd_d_slot;
        let mut var_one_over_one_minus_pgat2nd_d_rv: f64 = *var_one_over_one_minus_pgat2nd_d_rv_slot;
        let mut var_pgat2nd_d: f64 = *var_pgat2nd_d_slot;
        let mut var_pgat2nd_d_rv: f64 = *var_pgat2nd_d_rv_slot;
        let mut var_phigdbot: f64 = *var_phigdbot_slot;
        let mut var_phigdbot_rv: f64 = *var_phigdbot_rv_slot;
        let mut var_phigdgat: f64 = *var_phigdgat_slot;
        let mut var_phigdgat2nd: f64 = *var_phigdgat2nd_slot;
        let mut var_phigdgat2nd_rv: f64 = *var_phigdgat2nd_rv_slot;
        let mut var_phigdgat_rv: f64 = *var_phigdgat_rv_slot;
        let mut var_phigdsti: f64 = *var_phigdsti_slot;
        let mut var_phigdsti_rv: f64 = *var_phigdsti_rv_slot;
        let mut var_phiggat2nd_d: f64 = *var_phiggat2nd_d_slot;
        let mut var_phiggat2nd_d_rv: f64 = *var_phiggat2nd_d_rv_slot;
        let mut var_phigrgat2nd_d: f64 = *var_phigrgat2nd_d_slot;
        let mut var_phigrgat2nd_d_rv: f64 = *var_phigrgat2nd_d_rv_slot;
        let mut var_phita: f64 = *var_phita_slot;
        let mut var_phita_rv: f64 = *var_phita_rv_slot;
        let mut var_phitd: f64 = *var_phitd_slot;
        let mut var_phitd_rv: f64 = *var_phitd_rv_slot;
        let mut var_phitdinv: f64 = *var_phitdinv_slot;
        let mut var_phitdinv_rv: f64 = *var_phitdinv_rv_slot;
        let mut var_qpref2bot: f64 = *var_qpref2bot_slot;
        let mut var_qpref2bot_rv: f64 = *var_qpref2bot_rv_slot;
        let mut var_qpref2gat: f64 = *var_qpref2gat_slot;
        let mut var_qpref2gat_rv: f64 = *var_qpref2gat_rv_slot;
        let mut var_qpref2sti: f64 = *var_qpref2sti_slot;
        let mut var_qpref2sti_rv: f64 = *var_qpref2sti_rv_slot;
        let mut var_qprefbot: f64 = *var_qprefbot_slot;
        let mut var_qprefbot_rv: f64 = *var_qprefbot_rv_slot;
        let mut var_qprefgat: f64 = *var_qprefgat_slot;
        let mut var_qprefgat_rv: f64 = *var_qprefgat_rv_slot;
        let mut var_qprefsti: f64 = *var_qprefsti_slot;
        let mut var_qprefsti_rv: f64 = *var_qprefsti_rv_slot;
        let mut var_rta: f64 = *var_rta_slot;
        let mut var_rta_rv: f64 = *var_rta_rv_slot;
        let mut var_tka: f64 = *var_tka_slot;
        let mut var_tka_rv: f64 = *var_tka_rv_slot;
        let mut var_tkd_1: f64 = *var_tkd_1_slot;
        let mut var_tkd_1_rv: f64 = *var_tkd_1_rv_slot;
        let mut var_ubibot: f64 = *var_ubibot_slot;
        let mut var_ubibot_rv: f64 = *var_ubibot_rv_slot;
        let mut var_ubigat: f64 = *var_ubigat_slot;
        let mut var_ubigat2nd: f64 = *var_ubigat2nd_slot;
        let mut var_ubigat2nd_rv: f64 = *var_ubigat2nd_rv_slot;
        let mut var_ubigat_rv: f64 = *var_ubigat_rv_slot;
        let mut var_ubisti: f64 = *var_ubisti_slot;
        let mut var_ubisti_rv: f64 = *var_ubisti_rv_slot;
        let mut var_vbibot: f64 = *var_vbibot_slot;
        let mut var_vbibot_rv: f64 = *var_vbibot_rv_slot;
        let mut var_vbigat: f64 = *var_vbigat_slot;
        let mut var_vbigat2nd: f64 = *var_vbigat2nd_slot;
        let mut var_vbigat2nd_rv: f64 = *var_vbigat2nd_rv_slot;
        let mut var_vbigat_rv: f64 = *var_vbigat_rv_slot;
        let mut var_vbiinvbot: f64 = *var_vbiinvbot_slot;
        let mut var_vbiinvbot_rv: f64 = *var_vbiinvbot_rv_slot;
        let mut var_vbiinvgat: f64 = *var_vbiinvgat_slot;
        let mut var_vbiinvgat2nd: f64 = *var_vbiinvgat2nd_slot;
        let mut var_vbiinvgat2nd_rv: f64 = *var_vbiinvgat2nd_rv_slot;
        let mut var_vbiinvgat_rv: f64 = *var_vbiinvgat_rv_slot;
        let mut var_vbiinvsti: f64 = *var_vbiinvsti_slot;
        let mut var_vbiinvsti_rv: f64 = *var_vbiinvsti_rv_slot;
        let mut var_vbisti: f64 = *var_vbisti_slot;
        let mut var_vbisti_rv: f64 = *var_vbisti_rv_slot;

        let (assign1960_e2486,) = {
    if (var_guard7 != 0.0) {
        let assign1960_e2463: f64 = (var_pgatd_i * var_fpgat2d_i);
        let (assign1960_e2470,) = {
            if (assign1960_e2463 > 0.05) {
                let assign1960_e2468: f64 = (var_pgatd_i * var_fpgat2d_i);
                (assign1960_e2468,)
            } else {
                (0.05,)
            }
        };
        let (assign1960_e2484,) = {
            if (assign1960_e2470 < 0.95) {
                let assign1960_e2475: f64 = (var_pgatd_i * var_fpgat2d_i);
                let (assign1960_e2482,) = {
                    if (assign1960_e2475 > 0.05) {
                        let assign1960_e2480: f64 = (var_pgatd_i * var_fpgat2d_i);
                        (assign1960_e2480,)
                    } else {
                        (0.05,)
                    }
                };
                (assign1960_e2482,)
            } else {
                (0.95,)
            }
        };
        (assign1960_e2484,)
    } else {
        (var_pgat2nd_d,)
    }
};
        var_pgat2nd_d = assign1960_e2486;
        var_pgat2nd_d_rv = 0.0;

        let (assign1970_e2492,) = {
    if (var_guard7 != 0.0) {
        let assign1970_e2490: f64 = (var_phiggatd_i * var_fphiggat2d_i);
        (assign1970_e2490,)
    } else {
        (var_phiggat2nd_d,)
    }
};
        var_phiggat2nd_d = assign1970_e2492;
        var_phiggat2nd_d_rv = 0.0;

        let (assign1980_e2498,) = {
    if (var_guard7 != 0.0) {
        let assign1980_e2496: f64 = (var_phiggat2nd_d + var_deltaphigr);
        (assign1980_e2496,)
    } else {
        (var_phigrgat2nd_d,)
    }
};
        var_phigrgat2nd_d = assign1980_e2498;
        var_phigrgat2nd_d_rv = 0.0;

        let (assign1990_e2504,) = {
    if (var_guard7 != 0.0) {
        let assign1990_e2502: f64 = (1.0 - var_pgat2nd_d);
        (assign1990_e2502,)
    } else {
        (var_one_minus_pgat2nd_d,)
    }
};
        var_one_minus_pgat2nd_d = assign1990_e2504;
        var_one_minus_pgat2nd_d_rv = 0.0;

        let (assign2000_e2510,) = {
    if (var_guard7 != 0.0) {
        let assign2000_e2508: f64 = (1.0 / var_one_minus_pgat2nd_d);
        (assign2000_e2508,)
    } else {
        (var_one_over_one_minus_pgat2nd_d,)
    }
};
        var_one_over_one_minus_pgat2nd_d = assign2000_e2510;
        var_one_over_one_minus_pgat2nd_d_rv = 0.0;

        let assign2050_e2532: f64 = ctx_temp;
        let assign2050_e2534: f64 = (assign2050_e2532 + p.p55);
        let assign2050_e2536: f64 = (assign2050_e2534 + p.p35);
        var_tka = assign2050_e2536;
        var_tka_rv = 0.0;

        let assign2060_e2539: f64 = (var_tka / var_tkr);
        var_rta = assign2060_e2539;
        var_rta_rv = 0.0;

        let assign2070_e2542: f64 = (var_tka - var_tkr);
        var_delta = assign2070_e2542;
        var_delta_rv = 0.0;

        let assign2080_e2545: f64 = (var_tka * 1.3806505e-23);
        let assign2080_e2547: f64 = (assign2080_e2545 / 1.6021918e-19);
        var_phita = assign2080_e2547;
        var_phita_rv = 0.0;

        let assign2090_e2550: f64 = (1.0 / var_phita);
        var_inv_phita = assign2090_e2550;
        var_inv_phita_rv = 0.0;

        let assign2100_e2551: f64 = ctx_temp;
        let assign2100_e2553: f64 = (assign2100_e2551 + p.p55);
        let assign2100_e2555: f64 = (assign2100_e2553 + p.p35);
        let assign2100_e2558: f64 = (-250.0);
        let assign2100_e2559: f64 = (273.15 + assign2100_e2558);
        let assign2100_e2560: f64 = (assign2100_e2555).max(assign2100_e2559);
        var_tkd_1 = assign2100_e2560;
        var_tkd_1_rv = 0.0;

        let assign2110_e2563: f64 = (var_tkd_1 / var_tkr_1);
        var_auxt = assign2110_e2563;
        var_auxt_rv = 0.0;

        let assign2120_e2566: f64 = (var_kbol_over_qele * var_tkd_1);
        var_phitd = assign2120_e2566;
        var_phitd_rv = 0.0;

        let assign2130_e2569: f64 = (1.0 / var_phitd);
        var_phitdinv = assign2130_e2569;
        var_phitdinv_rv = 0.0;

        let assign2140_e2572: f64 = (0.000702 * var_tkd_1);
        let assign2140_e2574: f64 = (assign2140_e2572 * var_tkd_1);
        let assign2140_e2575: f64 = (-assign2140_e2574);
        let assign2140_e2578: f64 = (1108.0 + var_tkd_1);
        let assign2140_e2579: f64 = (assign2140_e2575 / assign2140_e2578);
        var_deltaphigd = assign2140_e2579;
        var_deltaphigd_rv = 0.0;

        let assign2150_e2582: f64 = (p.p851 + var_deltaphigd);
        var_phigdbot = assign2150_e2582;
        var_phigdbot_rv = 0.0;

        let assign2160_e2585: f64 = (p.p852 + var_deltaphigd);
        var_phigdsti = assign2160_e2585;
        var_phigdsti_rv = 0.0;

        let assign2170_e2588: f64 = (p.p853 + var_deltaphigd);
        var_phigdgat = assign2170_e2588;
        var_phigdgat_rv = 0.0;

        let assign2180_e2591: f64 = (var_auxt).powf(1.5);
        let assign2180_e2595: f64 = (var_phigrbot * var_phitrinv);
        let assign2180_e2598: f64 = (var_phigdbot * var_phitdinv);
        let assign2180_e2599: f64 = (assign2180_e2595 - assign2180_e2598);
        let assign2180_e2600: f64 = (0.5 * assign2180_e2599);
        let assign2180_e2601: f64 = (assign2180_e2600).exp();
        let assign2180_e2602: f64 = (assign2180_e2591 * assign2180_e2601);
        var_ftdbot = assign2180_e2602;
        var_ftdbot_rv = 0.0;

        let assign2190_e2605: f64 = (var_auxt).powf(1.5);
        let assign2190_e2609: f64 = (var_phigrsti * var_phitrinv);
        let assign2190_e2612: f64 = (var_phigdsti * var_phitdinv);
        let assign2190_e2613: f64 = (assign2190_e2609 - assign2190_e2612);
        let assign2190_e2614: f64 = (0.5 * assign2190_e2613);
        let assign2190_e2615: f64 = (assign2190_e2614).exp();
        let assign2190_e2616: f64 = (assign2190_e2605 * assign2190_e2615);
        var_ftdsti = assign2190_e2616;
        var_ftdsti_rv = 0.0;

        let assign2200_e2619: f64 = (var_auxt).powf(1.5);
        let assign2200_e2623: f64 = (var_phigrgat * var_phitrinv);
        let assign2200_e2626: f64 = (var_phigdgat * var_phitdinv);
        let assign2200_e2627: f64 = (assign2200_e2623 - assign2200_e2626);
        let assign2200_e2628: f64 = (0.5 * assign2200_e2627);
        let assign2200_e2629: f64 = (assign2200_e2628).exp();
        let assign2200_e2630: f64 = (assign2200_e2619 * assign2200_e2629);
        var_ftdgat = assign2200_e2630;
        var_ftdgat_rv = 0.0;

        let assign2210_e2633: f64 = (p.p854 * var_ftdbot);
        let assign2210_e2635: f64 = (assign2210_e2633 * var_ftdbot);
        var_idsatbot = assign2210_e2635;
        var_idsatbot_rv = 0.0;

        let assign2220_e2638: f64 = (p.p855 * var_ftdsti);
        let assign2220_e2640: f64 = (assign2220_e2638 * var_ftdsti);
        var_idsatsti = assign2220_e2640;
        var_idsatsti_rv = 0.0;

        let assign2230_e2643: f64 = (p.p856 * var_ftdgat);
        let assign2230_e2645: f64 = (assign2230_e2643 * var_ftdgat);
        var_idsatgat = assign2230_e2645;
        var_idsatgat_rv = 0.0;

        let assign2240_e2648: f64 = (p.p845 * var_auxt);
        let assign2240_e2651: f64 = (2.0 * var_phitd);
        let assign2240_e2653: f64 = (var_ftdbot).ln();
        let assign2240_e2654: f64 = (assign2240_e2651 * assign2240_e2653);
        let assign2240_e2655: f64 = (assign2240_e2648 - assign2240_e2654);
        var_ubibot = assign2240_e2655;
        var_ubibot_rv = 0.0;

        let assign2250_e2658: f64 = (p.p846 * var_auxt);
        let assign2250_e2661: f64 = (2.0 * var_phitd);
        let assign2250_e2663: f64 = (var_ftdsti).ln();
        let assign2250_e2664: f64 = (assign2250_e2661 * assign2250_e2663);
        let assign2250_e2665: f64 = (assign2250_e2658 - assign2250_e2664);
        var_ubisti = assign2250_e2665;
        var_ubisti_rv = 0.0;

        let assign2260_e2668: f64 = (p.p847 * var_auxt);
        let assign2260_e2671: f64 = (2.0 * var_phitd);
        let assign2260_e2673: f64 = (var_ftdgat).ln();
        let assign2260_e2674: f64 = (assign2260_e2671 * assign2260_e2673);
        let assign2260_e2675: f64 = (assign2260_e2668 - assign2260_e2674);
        var_ubigat = assign2260_e2675;
        var_ubigat_rv = 0.0;

        let assign2270_e2681: f64 = (0.05 - var_ubibot);
        let assign2270_e2683: f64 = (assign2270_e2681 * var_phitdinv);
        let assign2270_e2684: f64 = (assign2270_e2683).exp();
        let assign2270_e2685: f64 = (1.0 + assign2270_e2684);
        let assign2270_e2686: f64 = (assign2270_e2685).ln();
        let assign2270_e2687: f64 = (var_phitd * assign2270_e2686);
        let assign2270_e2688: f64 = (var_ubibot + assign2270_e2687);
        var_vbibot = assign2270_e2688;
        var_vbibot_rv = 0.0;

        let assign2280_e2694: f64 = (0.05 - var_ubisti);
        let assign2280_e2696: f64 = (assign2280_e2694 * var_phitdinv);
        let assign2280_e2697: f64 = (assign2280_e2696).exp();
        let assign2280_e2698: f64 = (1.0 + assign2280_e2697);
        let assign2280_e2699: f64 = (assign2280_e2698).ln();
        let assign2280_e2700: f64 = (var_phitd * assign2280_e2699);
        let assign2280_e2701: f64 = (var_ubisti + assign2280_e2700);
        var_vbisti = assign2280_e2701;
        var_vbisti_rv = 0.0;

        let assign2290_e2707: f64 = (0.05 - var_ubigat);
        let assign2290_e2709: f64 = (assign2290_e2707 * var_phitdinv);
        let assign2290_e2710: f64 = (assign2290_e2709).exp();
        let assign2290_e2711: f64 = (1.0 + assign2290_e2710);
        let assign2290_e2712: f64 = (assign2290_e2711).ln();
        let assign2290_e2713: f64 = (var_phitd * assign2290_e2712);
        let assign2290_e2714: f64 = (var_ubigat + assign2290_e2713);
        var_vbigat = assign2290_e2714;
        var_vbigat_rv = 0.0;

        let assign2300_e2717: f64 = (1.0 / var_vbibot);
        var_vbiinvbot = assign2300_e2717;
        var_vbiinvbot_rv = 0.0;

        let assign2310_e2720: f64 = (1.0 / var_vbisti);
        var_vbiinvsti = assign2310_e2720;
        var_vbiinvsti_rv = 0.0;

        let assign2320_e2723: f64 = (1.0 / var_vbigat);
        var_vbiinvgat = assign2320_e2723;
        var_vbiinvgat_rv = 0.0;

        let assign2330_e2727: f64 = (p.p845 * var_vbiinvbot);
        let assign2330_e2729: f64 = (assign2330_e2727).powf(p.p848);
        let assign2330_e2730: f64 = (p.p842 * assign2330_e2729);
        var_cjobot = assign2330_e2730;
        var_cjobot_rv = 0.0;

        let assign2340_e2734: f64 = (p.p846 * var_vbiinvsti);
        let assign2340_e2736: f64 = (assign2340_e2734).powf(p.p849);
        let assign2340_e2737: f64 = (p.p843 * assign2340_e2736);
        var_cjosti = assign2340_e2737;
        var_cjosti_rv = 0.0;

        let assign2350_e2741: f64 = (p.p847 * var_vbiinvgat);
        let assign2350_e2743: f64 = (assign2350_e2741).powf(p.p850);
        let assign2350_e2744: f64 = (p.p844 * assign2350_e2743);
        var_cjogat = assign2350_e2744;
        var_cjogat_rv = 0.0;

        let assign2360_e2747: f64 = (var_cjobot * var_vbibot);
        let assign2360_e2749: f64 = (assign2360_e2747 * var_one_over_one_minus_pbot);
        var_qprefbot = assign2360_e2749;
        var_qprefbot_rv = 0.0;

        let assign2370_e2752: f64 = (var_cjosti * var_vbisti);
        let assign2370_e2754: f64 = (assign2370_e2752 * var_one_over_one_minus_psti);
        var_qprefsti = assign2370_e2754;
        var_qprefsti_rv = 0.0;

        let assign2380_e2757: f64 = (var_cjogat * var_vbigat);
        let assign2380_e2759: f64 = (assign2380_e2757 * var_one_over_one_minus_pgat);
        var_qprefgat = assign2380_e2759;
        var_qprefgat_rv = 0.0;

        let assign2390_e2762: f64 = (2.0 * var_cjobot);
        var_qpref2bot = assign2390_e2762;
        var_qpref2bot_rv = 0.0;

        let assign2400_e2765: f64 = (2.0 * var_cjosti);
        var_qpref2sti = assign2400_e2765;
        var_qpref2sti_rv = 0.0;

        let assign2410_e2768: f64 = (2.0 * var_cjogat);
        var_qpref2gat = assign2410_e2768;
        var_qpref2gat_rv = 0.0;

        let assign2420_e2771: f64 = (0.5 * var_phigdbot);
        let assign2420_e2773: f64 = (assign2420_e2771).max(var_phitd);
        var_deltaebot = assign2420_e2773;
        var_deltaebot_rv = 0.0;

        let assign2430_e2776: f64 = (0.5 * var_phigdsti);
        let assign2430_e2778: f64 = (assign2430_e2776).max(var_phitd);
        var_deltaesti = assign2430_e2778;
        var_deltaesti_rv = 0.0;

        let assign2440_e2781: f64 = (0.5 * var_phigdgat);
        let assign2440_e2783: f64 = (assign2440_e2781).max(var_phitd);
        var_deltaegat = assign2440_e2783;
        var_deltaegat_rv = 0.0;

        let assign2450_e2786: f64 = (var_deltaebot * var_phitdinv);
        var_atatbot = assign2450_e2786;
        var_atatbot_rv = 0.0;

        let assign2460_e2789: f64 = (var_deltaesti * var_phitdinv);
        var_atatsti = assign2460_e2789;
        var_atatsti_rv = 0.0;

        let assign2470_e2792: f64 = (var_deltaegat * var_phitdinv);
        var_atatgat = assign2470_e2792;
        var_atatgat_rv = 0.0;

        let assign2480_e2795: f64 = (32.0 * p.p865);
        let assign2480_e2797: f64 = (assign2480_e2795 * 9.1093826e-31);
        let assign2480_e2799: f64 = (assign2480_e2797 * 1.6021918e-19);
        let assign2480_e2802: f64 = (var_deltaebot * var_deltaebot);
        let assign2480_e2804: f64 = (assign2480_e2802 * var_deltaebot);
        let assign2480_e2805: f64 = (assign2480_e2799 * assign2480_e2804);
        let assign2480_e2806: f64 = (assign2480_e2805).sqrt();
        let assign2480_e2809: f64 = (3.0 * 1.05457168e-34);
        let assign2480_e2810: f64 = (assign2480_e2806 / assign2480_e2809);
        var_btatpartbot = assign2480_e2810;
        var_btatpartbot_rv = 0.0;

        let assign2490_e2813: f64 = (32.0 * p.p866);
        let assign2490_e2815: f64 = (assign2490_e2813 * 9.1093826e-31);
        let assign2490_e2817: f64 = (assign2490_e2815 * 1.6021918e-19);
        let assign2490_e2820: f64 = (var_deltaesti * var_deltaesti);
        let assign2490_e2822: f64 = (assign2490_e2820 * var_deltaesti);
        let assign2490_e2823: f64 = (assign2490_e2817 * assign2490_e2822);
        let assign2490_e2824: f64 = (assign2490_e2823).sqrt();
        let assign2490_e2827: f64 = (3.0 * 1.05457168e-34);
        let assign2490_e2828: f64 = (assign2490_e2824 / assign2490_e2827);
        var_btatpartsti = assign2490_e2828;
        var_btatpartsti_rv = 0.0;

        let assign2500_e2831: f64 = (32.0 * p.p867);
        let assign2500_e2833: f64 = (assign2500_e2831 * 9.1093826e-31);
        let assign2500_e2835: f64 = (assign2500_e2833 * 1.6021918e-19);
        let assign2500_e2838: f64 = (var_deltaegat * var_deltaegat);
        let assign2500_e2840: f64 = (assign2500_e2838 * var_deltaegat);
        let assign2500_e2841: f64 = (assign2500_e2835 * assign2500_e2840);
        let assign2500_e2842: f64 = (assign2500_e2841).sqrt();
        let assign2500_e2845: f64 = (3.0 * 1.05457168e-34);
        let assign2500_e2846: f64 = (assign2500_e2842 / assign2500_e2845);
        var_btatpartgat = assign2500_e2846;
        var_btatpartgat_rv = 0.0;

        let assign2510_e2852: f64 = (var_tkd_1 - var_tkr_1);
        let assign2510_e2853: f64 = (p.p874 * assign2510_e2852);
        let assign2510_e2854: f64 = (1.0 + assign2510_e2853);
        let assign2510_e2855: f64 = (p.p871 * assign2510_e2854);
        var_fbbtbot = assign2510_e2855;
        var_fbbtbot_rv = 0.0;

        let assign2520_e2861: f64 = (var_tkd_1 - var_tkr_1);
        let assign2520_e2862: f64 = (p.p875 * assign2520_e2861);
        let assign2520_e2863: f64 = (1.0 + assign2520_e2862);
        let assign2520_e2864: f64 = (p.p872 * assign2520_e2863);
        var_fbbtsti = assign2520_e2864;
        var_fbbtsti_rv = 0.0;

        let assign2530_e2870: f64 = (var_tkd_1 - var_tkr_1);
        let assign2530_e2871: f64 = (p.p876 * assign2530_e2870);
        let assign2530_e2872: f64 = (1.0 + assign2530_e2871);
        let assign2530_e2873: f64 = (p.p873 * assign2530_e2872);
        var_fbbtgat = assign2530_e2873;
        var_fbbtgat_dn6 = 0.0;
        var_fbbtgat_dn7 = 0.0;
        var_fbbtgat_dn8 = 0.0;
        var_fbbtgat_dn9 = 0.0;
        var_fbbtgat_rv = 0.0;

        let (assign2540_e2879,) = {
    if (var_fbbtbot > 0.0) {
        (var_fbbtbot,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot = assign2540_e2879;
        var_fbbtbot_rv = 0.0;

        let (assign2550_e2885,) = {
    if (var_fbbtsti > 0.0) {
        (var_fbbtsti,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti = assign2550_e2885;
        var_fbbtsti_rv = 0.0;

        let (assign2560_e2891, assign2560_e2891_d_n6, assign2560_e2891_d_n7, assign2560_e2891_d_n8, assign2560_e2891_d_n9,) = {
    if (var_fbbtgat > 0.0) {
        (var_fbbtgat, var_fbbtgat_dn6, var_fbbtgat_dn7, var_fbbtgat_dn8, var_fbbtgat_dn9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat = assign2560_e2891;
        var_fbbtgat_dn6 = assign2560_e2891_d_n6;
        var_fbbtgat_dn7 = assign2560_e2891_d_n7;
        var_fbbtgat_dn8 = assign2560_e2891_d_n8;
        var_fbbtgat_dn9 = assign2560_e2891_d_n9;
        var_fbbtgat_rv = 0.0;

        let assign2570_e2894: f64 = if var_swgat2nd == 1.0 { 1.0 } else { 0.0 };
        var_guard27 = assign2570_e2894;
        var_guard27_rv = 0.0;

        let (assign2580_e2900,) = {
    if (var_guard27 != 0.0) {
        let assign2580_e2898: f64 = (var_phiggat2nd + var_deltaphigd);
        (assign2580_e2898,)
    } else {
        (var_phigdgat2nd,)
    }
};
        var_phigdgat2nd = assign2580_e2900;
        var_phigdgat2nd_rv = 0.0;

        let (assign2590_e2917,) = {
    if (var_guard27 != 0.0) {
        let assign2590_e2904: f64 = (var_auxt).powf(1.5);
        let assign2590_e2908: f64 = (var_phigrgat2nd * var_phitrinv);
        let assign2590_e2911: f64 = (var_phigdgat2nd * var_phitdinv);
        let assign2590_e2912: f64 = (assign2590_e2908 - assign2590_e2911);
        let assign2590_e2913: f64 = (0.5 * assign2590_e2912);
        let assign2590_e2914: f64 = (assign2590_e2913).exp();
        let assign2590_e2915: f64 = (assign2590_e2904 * assign2590_e2914);
        (assign2590_e2915,)
    } else {
        (var_ftdgat2nd,)
    }
};
        var_ftdgat2nd = assign2590_e2917;
        var_ftdgat2nd_rv = 0.0;

        let (assign2600_e2930,) = {
    if (var_guard27 != 0.0) {
        let assign2600_e2921: f64 = (var_vbirgat2nd * var_auxt);
        let assign2600_e2924: f64 = (2.0 * var_phitd);
        let assign2600_e2926: f64 = (var_ftdgat2nd).ln();
        let assign2600_e2927: f64 = (assign2600_e2924 * assign2600_e2926);
        let assign2600_e2928: f64 = (assign2600_e2921 - assign2600_e2927);
        (assign2600_e2928,)
    } else {
        (var_ubigat2nd,)
    }
};
        var_ubigat2nd = assign2600_e2930;
        var_ubigat2nd_rv = 0.0;

        let (assign2610_e2946,) = {
    if (var_guard27 != 0.0) {
        let assign2610_e2937: f64 = (0.05 - var_ubigat2nd);
        let assign2610_e2939: f64 = (assign2610_e2937 * var_phitdinv);
        let assign2610_e2940: f64 = (assign2610_e2939).exp();
        let assign2610_e2941: f64 = (1.0 + assign2610_e2940);
        let assign2610_e2942: f64 = (assign2610_e2941).ln();
        let assign2610_e2943: f64 = (var_phitd * assign2610_e2942);
        let assign2610_e2944: f64 = (var_ubigat2nd + assign2610_e2943);
        (assign2610_e2944,)
    } else {
        (var_vbigat2nd,)
    }
};
        var_vbigat2nd = assign2610_e2946;
        var_vbigat2nd_rv = 0.0;

        let (assign2620_e2952,) = {
    if (var_guard27 != 0.0) {
        let assign2620_e2950: f64 = (1.0 / var_vbigat2nd);
        (assign2620_e2950,)
    } else {
        (var_vbiinvgat2nd,)
    }
};
        var_vbiinvgat2nd = assign2620_e2952;
        var_vbiinvgat2nd_rv = 0.0;

        let (assign2630_e2962,) = {
    if (var_guard27 != 0.0) {
        let assign2630_e2957: f64 = (var_vbirgat2nd * var_vbiinvgat2nd);
        let assign2630_e2959: f64 = (assign2630_e2957).powf(var_pgat2nd);
        let assign2630_e2960: f64 = (var_cjorgat2nd * assign2630_e2959);
        (assign2630_e2960,)
    } else {
        (var_cjogat2nd,)
    }
};
        var_cjogat2nd = assign2630_e2962;
        var_cjogat2nd_rv = 0.0;

        *var_atatbot_slot = var_atatbot;
        *var_atatbot_rv_slot = var_atatbot_rv;
        *var_atatgat_slot = var_atatgat;
        *var_atatgat_rv_slot = var_atatgat_rv;
        *var_atatsti_slot = var_atatsti;
        *var_atatsti_rv_slot = var_atatsti_rv;
        *var_auxt_slot = var_auxt;
        *var_auxt_rv_slot = var_auxt_rv;
        *var_btatpartbot_slot = var_btatpartbot;
        *var_btatpartbot_rv_slot = var_btatpartbot_rv;
        *var_btatpartgat_slot = var_btatpartgat;
        *var_btatpartgat_rv_slot = var_btatpartgat_rv;
        *var_btatpartsti_slot = var_btatpartsti;
        *var_btatpartsti_rv_slot = var_btatpartsti_rv;
        *var_cjobot_slot = var_cjobot;
        *var_cjobot_rv_slot = var_cjobot_rv;
        *var_cjogat_slot = var_cjogat;
        *var_cjogat2nd_slot = var_cjogat2nd;
        *var_cjogat2nd_rv_slot = var_cjogat2nd_rv;
        *var_cjogat_rv_slot = var_cjogat_rv;
        *var_cjosti_slot = var_cjosti;
        *var_cjosti_rv_slot = var_cjosti_rv;
        *var_delta_slot = var_delta;
        *var_delta_rv_slot = var_delta_rv;
        *var_deltaebot_slot = var_deltaebot;
        *var_deltaebot_rv_slot = var_deltaebot_rv;
        *var_deltaegat_slot = var_deltaegat;
        *var_deltaegat_rv_slot = var_deltaegat_rv;
        *var_deltaesti_slot = var_deltaesti;
        *var_deltaesti_rv_slot = var_deltaesti_rv;
        *var_deltaphigd_slot = var_deltaphigd;
        *var_deltaphigd_rv_slot = var_deltaphigd_rv;
        *var_fbbtbot_slot = var_fbbtbot;
        *var_fbbtbot_rv_slot = var_fbbtbot_rv;
        *var_fbbtgat_slot = var_fbbtgat;
        *var_fbbtgat_dn6_slot = var_fbbtgat_dn6;
        *var_fbbtgat_dn7_slot = var_fbbtgat_dn7;
        *var_fbbtgat_dn8_slot = var_fbbtgat_dn8;
        *var_fbbtgat_dn9_slot = var_fbbtgat_dn9;
        *var_fbbtgat_rv_slot = var_fbbtgat_rv;
        *var_fbbtsti_slot = var_fbbtsti;
        *var_fbbtsti_rv_slot = var_fbbtsti_rv;
        *var_ftdbot_slot = var_ftdbot;
        *var_ftdbot_rv_slot = var_ftdbot_rv;
        *var_ftdgat_slot = var_ftdgat;
        *var_ftdgat2nd_slot = var_ftdgat2nd;
        *var_ftdgat2nd_rv_slot = var_ftdgat2nd_rv;
        *var_ftdgat_rv_slot = var_ftdgat_rv;
        *var_ftdsti_slot = var_ftdsti;
        *var_ftdsti_rv_slot = var_ftdsti_rv;
        *var_guard27_slot = var_guard27;
        *var_guard27_rv_slot = var_guard27_rv;
        *var_idsatbot_slot = var_idsatbot;
        *var_idsatbot_rv_slot = var_idsatbot_rv;
        *var_idsatgat_slot = var_idsatgat;
        *var_idsatgat_rv_slot = var_idsatgat_rv;
        *var_idsatsti_slot = var_idsatsti;
        *var_idsatsti_rv_slot = var_idsatsti_rv;
        *var_inv_phita_slot = var_inv_phita;
        *var_inv_phita_rv_slot = var_inv_phita_rv;
        *var_one_minus_pgat2nd_d_slot = var_one_minus_pgat2nd_d;
        *var_one_minus_pgat2nd_d_rv_slot = var_one_minus_pgat2nd_d_rv;
        *var_one_over_one_minus_pgat2nd_d_slot = var_one_over_one_minus_pgat2nd_d;
        *var_one_over_one_minus_pgat2nd_d_rv_slot = var_one_over_one_minus_pgat2nd_d_rv;
        *var_pgat2nd_d_slot = var_pgat2nd_d;
        *var_pgat2nd_d_rv_slot = var_pgat2nd_d_rv;
        *var_phigdbot_slot = var_phigdbot;
        *var_phigdbot_rv_slot = var_phigdbot_rv;
        *var_phigdgat_slot = var_phigdgat;
        *var_phigdgat2nd_slot = var_phigdgat2nd;
        *var_phigdgat2nd_rv_slot = var_phigdgat2nd_rv;
        *var_phigdgat_rv_slot = var_phigdgat_rv;
        *var_phigdsti_slot = var_phigdsti;
        *var_phigdsti_rv_slot = var_phigdsti_rv;
        *var_phiggat2nd_d_slot = var_phiggat2nd_d;
        *var_phiggat2nd_d_rv_slot = var_phiggat2nd_d_rv;
        *var_phigrgat2nd_d_slot = var_phigrgat2nd_d;
        *var_phigrgat2nd_d_rv_slot = var_phigrgat2nd_d_rv;
        *var_phita_slot = var_phita;
        *var_phita_rv_slot = var_phita_rv;
        *var_phitd_slot = var_phitd;
        *var_phitd_rv_slot = var_phitd_rv;
        *var_phitdinv_slot = var_phitdinv;
        *var_phitdinv_rv_slot = var_phitdinv_rv;
        *var_qpref2bot_slot = var_qpref2bot;
        *var_qpref2bot_rv_slot = var_qpref2bot_rv;
        *var_qpref2gat_slot = var_qpref2gat;
        *var_qpref2gat_rv_slot = var_qpref2gat_rv;
        *var_qpref2sti_slot = var_qpref2sti;
        *var_qpref2sti_rv_slot = var_qpref2sti_rv;
        *var_qprefbot_slot = var_qprefbot;
        *var_qprefbot_rv_slot = var_qprefbot_rv;
        *var_qprefgat_slot = var_qprefgat;
        *var_qprefgat_rv_slot = var_qprefgat_rv;
        *var_qprefsti_slot = var_qprefsti;
        *var_qprefsti_rv_slot = var_qprefsti_rv;
        *var_rta_slot = var_rta;
        *var_rta_rv_slot = var_rta_rv;
        *var_tka_slot = var_tka;
        *var_tka_rv_slot = var_tka_rv;
        *var_tkd_1_slot = var_tkd_1;
        *var_tkd_1_rv_slot = var_tkd_1_rv;
        *var_ubibot_slot = var_ubibot;
        *var_ubibot_rv_slot = var_ubibot_rv;
        *var_ubigat_slot = var_ubigat;
        *var_ubigat2nd_slot = var_ubigat2nd;
        *var_ubigat2nd_rv_slot = var_ubigat2nd_rv;
        *var_ubigat_rv_slot = var_ubigat_rv;
        *var_ubisti_slot = var_ubisti;
        *var_ubisti_rv_slot = var_ubisti_rv;
        *var_vbibot_slot = var_vbibot;
        *var_vbibot_rv_slot = var_vbibot_rv;
        *var_vbigat_slot = var_vbigat;
        *var_vbigat2nd_slot = var_vbigat2nd;
        *var_vbigat2nd_rv_slot = var_vbigat2nd_rv;
        *var_vbigat_rv_slot = var_vbigat_rv;
        *var_vbiinvbot_slot = var_vbiinvbot;
        *var_vbiinvbot_rv_slot = var_vbiinvbot_rv;
        *var_vbiinvgat_slot = var_vbiinvgat;
        *var_vbiinvgat2nd_slot = var_vbiinvgat2nd;
        *var_vbiinvgat2nd_rv_slot = var_vbiinvgat2nd_rv;
        *var_vbiinvgat_rv_slot = var_vbiinvgat_rv;
        *var_vbiinvsti_slot = var_vbiinvsti;
        *var_vbiinvsti_rv_slot = var_vbiinvsti_rv;
        *var_vbisti_slot = var_vbisti;
        *var_vbisti_rv_slot = var_vbisti_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_auxt: f64,
        var_cjogat2nd: f64,
        var_cjorbotd_i: f64,
        var_cjorgat2nd_d: f64,
        var_cjorgatd_i: f64,
        var_cjorstid_i: f64,
        var_deltaphigd: f64,
        var_fbbtrbotd_i: f64,
        var_fbbtrgatd_i: f64,
        var_fbbtrstid_i: f64,
        var_guard27: f64,
        var_idsatrbotd_i: f64,
        var_idsatrgatd_i: f64,
        var_idsatrstid_i: f64,
        var_mefftatbotd_i: f64,
        var_mefftatgatd_i: f64,
        var_mefftatstid_i: f64,
        var_one_over_one_minus_pbot_d: f64,
        var_one_over_one_minus_pgat2nd: f64,
        var_one_over_one_minus_pgat2nd_d: f64,
        var_one_over_one_minus_pgat_d: f64,
        var_one_over_one_minus_psti_d: f64,
        var_pbotd_i: f64,
        var_pgat2nd_d: f64,
        var_pgatd_i: f64,
        var_phigbotd_i: f64,
        var_phiggat2nd_d: f64,
        var_phiggatd_i: f64,
        var_phigrbot_d: f64,
        var_phigrgat2nd_d: f64,
        var_phigrgat_d: f64,
        var_phigrsti_d: f64,
        var_phigstid_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitrinv: f64,
        var_pstid_i: f64,
        var_stfbbtbotd_i: f64,
        var_stfbbtgatd_i: f64,
        var_stfbbtstid_i: f64,
        var_swgat2nd_d: f64,
        var_tkd_1: f64,
        var_tkr_1: f64,
        var_vbigat2nd: f64,
        var_vbirbotd_i: f64,
        var_vbirgat2nd_d: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_abdrain_i_slot: &mut f64,
        var_abdrain_i_rv_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_absource_i_rv_slot: &mut f64,
        var_ad_i_slot: &mut f64,
        var_ad_i_rv_slot: &mut f64,
        var_as_i_slot: &mut f64,
        var_as_i_rv_slot: &mut f64,
        var_atatbot_d_slot: &mut f64,
        var_atatbot_d_rv_slot: &mut f64,
        var_atatgat_d_slot: &mut f64,
        var_atatgat_d_rv_slot: &mut f64,
        var_atatsti_d_slot: &mut f64,
        var_atatsti_d_rv_slot: &mut f64,
        var_btatpartbot_d_slot: &mut f64,
        var_btatpartbot_d_rv_slot: &mut f64,
        var_btatpartgat_d_slot: &mut f64,
        var_btatpartgat_d_rv_slot: &mut f64,
        var_btatpartsti_d_slot: &mut f64,
        var_btatpartsti_d_rv_slot: &mut f64,
        var_cjobot_d_slot: &mut f64,
        var_cjobot_d_rv_slot: &mut f64,
        var_cjogat2nd_d_slot: &mut f64,
        var_cjogat2nd_d_rv_slot: &mut f64,
        var_cjogat_d_slot: &mut f64,
        var_cjogat_d_rv_slot: &mut f64,
        var_cjosti_d_slot: &mut f64,
        var_cjosti_d_rv_slot: &mut f64,
        var_deltaebot_d_slot: &mut f64,
        var_deltaebot_d_rv_slot: &mut f64,
        var_deltaegat_d_slot: &mut f64,
        var_deltaegat_d_rv_slot: &mut f64,
        var_deltaesti_d_slot: &mut f64,
        var_deltaesti_d_rv_slot: &mut f64,
        var_fbbtbot_d_slot: &mut f64,
        var_fbbtbot_d_rv_slot: &mut f64,
        var_fbbtgat_d_slot: &mut f64,
        var_fbbtgat_d_dn6_slot: &mut f64,
        var_fbbtgat_d_dn7_slot: &mut f64,
        var_fbbtgat_d_dn8_slot: &mut f64,
        var_fbbtgat_d_dn9_slot: &mut f64,
        var_fbbtgat_d_rv_slot: &mut f64,
        var_fbbtsti_d_slot: &mut f64,
        var_fbbtsti_d_rv_slot: &mut f64,
        var_ftdbot_d_slot: &mut f64,
        var_ftdbot_d_rv_slot: &mut f64,
        var_ftdgat2nd_d_slot: &mut f64,
        var_ftdgat2nd_d_rv_slot: &mut f64,
        var_ftdgat_d_slot: &mut f64,
        var_ftdgat_d_rv_slot: &mut f64,
        var_ftdsti_d_slot: &mut f64,
        var_ftdsti_d_rv_slot: &mut f64,
        var_guard28_slot: &mut f64,
        var_guard28_rv_slot: &mut f64,
        var_guard29_slot: &mut f64,
        var_guard29_rv_slot: &mut f64,
        var_idsatbot_d_slot: &mut f64,
        var_idsatbot_d_rv_slot: &mut f64,
        var_idsatgat_d_slot: &mut f64,
        var_idsatgat_d_rv_slot: &mut f64,
        var_idsatsti_d_slot: &mut f64,
        var_idsatsti_d_rv_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_invnf_rv_slot: &mut f64,
        var_jw_i_slot: &mut f64,
        var_jw_i_rv_slot: &mut f64,
        var_l_i_slot: &mut f64,
        var_l_i_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgdrain_i_rv_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lgsource_i_rv_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lsdrain_i_rv_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_lssource_i_rv_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_nf_i_rv_slot: &mut f64,
        var_pd_i_slot: &mut f64,
        var_pd_i_rv_slot: &mut f64,
        var_phigdbot_d_slot: &mut f64,
        var_phigdbot_d_rv_slot: &mut f64,
        var_phigdgat2nd_d_slot: &mut f64,
        var_phigdgat2nd_d_rv_slot: &mut f64,
        var_phigdgat_d_slot: &mut f64,
        var_phigdgat_d_rv_slot: &mut f64,
        var_phigdsti_d_slot: &mut f64,
        var_phigdsti_d_rv_slot: &mut f64,
        var_ps_i_slot: &mut f64,
        var_ps_i_rv_slot: &mut f64,
        var_qpref2bot_d_slot: &mut f64,
        var_qpref2bot_d_rv_slot: &mut f64,
        var_qpref2gat2nd_slot: &mut f64,
        var_qpref2gat2nd_d_slot: &mut f64,
        var_qpref2gat2nd_d_rv_slot: &mut f64,
        var_qpref2gat2nd_rv_slot: &mut f64,
        var_qpref2gat_d_slot: &mut f64,
        var_qpref2gat_d_rv_slot: &mut f64,
        var_qpref2sti_d_slot: &mut f64,
        var_qpref2sti_d_rv_slot: &mut f64,
        var_qprefbot_d_slot: &mut f64,
        var_qprefbot_d_rv_slot: &mut f64,
        var_qprefgat2nd_slot: &mut f64,
        var_qprefgat2nd_d_slot: &mut f64,
        var_qprefgat2nd_d_rv_slot: &mut f64,
        var_qprefgat2nd_rv_slot: &mut f64,
        var_qprefgat_d_slot: &mut f64,
        var_qprefgat_d_rv_slot: &mut f64,
        var_qprefsti_d_slot: &mut f64,
        var_qprefsti_d_rv_slot: &mut f64,
        var_sa_i_slot: &mut f64,
        var_sa_i_rv_slot: &mut f64,
        var_sb_i_slot: &mut f64,
        var_sb_i_rv_slot: &mut f64,
        var_sc_i_slot: &mut f64,
        var_sc_i_rv_slot: &mut f64,
        var_sd_i_slot: &mut f64,
        var_sd_i_rv_slot: &mut f64,
        var_ubibot_d_slot: &mut f64,
        var_ubibot_d_rv_slot: &mut f64,
        var_ubigat2nd_d_slot: &mut f64,
        var_ubigat2nd_d_rv_slot: &mut f64,
        var_ubigat_d_slot: &mut f64,
        var_ubigat_d_rv_slot: &mut f64,
        var_ubisti_d_slot: &mut f64,
        var_ubisti_d_rv_slot: &mut f64,
        var_vbibot_d_slot: &mut f64,
        var_vbibot_d_rv_slot: &mut f64,
        var_vbigat2nd_d_slot: &mut f64,
        var_vbigat2nd_d_rv_slot: &mut f64,
        var_vbigat_d_slot: &mut f64,
        var_vbigat_d_rv_slot: &mut f64,
        var_vbiinvbot_d_slot: &mut f64,
        var_vbiinvbot_d_rv_slot: &mut f64,
        var_vbiinvgat2nd_d_slot: &mut f64,
        var_vbiinvgat2nd_d_rv_slot: &mut f64,
        var_vbiinvgat_d_slot: &mut f64,
        var_vbiinvgat_d_rv_slot: &mut f64,
        var_vbiinvsti_d_slot: &mut f64,
        var_vbiinvsti_d_rv_slot: &mut f64,
        var_vbisti_d_slot: &mut f64,
        var_vbisti_d_rv_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_w_i_rv_slot: &mut f64,
        var_we_slot: &mut f64,
        var_we_rv_slot: &mut f64,
    ) {
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abdrain_i_rv: f64 = *var_abdrain_i_rv_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_absource_i_rv: f64 = *var_absource_i_rv_slot;
        let mut var_ad_i: f64 = *var_ad_i_slot;
        let mut var_ad_i_rv: f64 = *var_ad_i_rv_slot;
        let mut var_as_i: f64 = *var_as_i_slot;
        let mut var_as_i_rv: f64 = *var_as_i_rv_slot;
        let mut var_atatbot_d: f64 = *var_atatbot_d_slot;
        let mut var_atatbot_d_rv: f64 = *var_atatbot_d_rv_slot;
        let mut var_atatgat_d: f64 = *var_atatgat_d_slot;
        let mut var_atatgat_d_rv: f64 = *var_atatgat_d_rv_slot;
        let mut var_atatsti_d: f64 = *var_atatsti_d_slot;
        let mut var_atatsti_d_rv: f64 = *var_atatsti_d_rv_slot;
        let mut var_btatpartbot_d: f64 = *var_btatpartbot_d_slot;
        let mut var_btatpartbot_d_rv: f64 = *var_btatpartbot_d_rv_slot;
        let mut var_btatpartgat_d: f64 = *var_btatpartgat_d_slot;
        let mut var_btatpartgat_d_rv: f64 = *var_btatpartgat_d_rv_slot;
        let mut var_btatpartsti_d: f64 = *var_btatpartsti_d_slot;
        let mut var_btatpartsti_d_rv: f64 = *var_btatpartsti_d_rv_slot;
        let mut var_cjobot_d: f64 = *var_cjobot_d_slot;
        let mut var_cjobot_d_rv: f64 = *var_cjobot_d_rv_slot;
        let mut var_cjogat2nd_d: f64 = *var_cjogat2nd_d_slot;
        let mut var_cjogat2nd_d_rv: f64 = *var_cjogat2nd_d_rv_slot;
        let mut var_cjogat_d: f64 = *var_cjogat_d_slot;
        let mut var_cjogat_d_rv: f64 = *var_cjogat_d_rv_slot;
        let mut var_cjosti_d: f64 = *var_cjosti_d_slot;
        let mut var_cjosti_d_rv: f64 = *var_cjosti_d_rv_slot;
        let mut var_deltaebot_d: f64 = *var_deltaebot_d_slot;
        let mut var_deltaebot_d_rv: f64 = *var_deltaebot_d_rv_slot;
        let mut var_deltaegat_d: f64 = *var_deltaegat_d_slot;
        let mut var_deltaegat_d_rv: f64 = *var_deltaegat_d_rv_slot;
        let mut var_deltaesti_d: f64 = *var_deltaesti_d_slot;
        let mut var_deltaesti_d_rv: f64 = *var_deltaesti_d_rv_slot;
        let mut var_fbbtbot_d: f64 = *var_fbbtbot_d_slot;
        let mut var_fbbtbot_d_rv: f64 = *var_fbbtbot_d_rv_slot;
        let mut var_fbbtgat_d: f64 = *var_fbbtgat_d_slot;
        let mut var_fbbtgat_d_dn6: f64 = *var_fbbtgat_d_dn6_slot;
        let mut var_fbbtgat_d_dn7: f64 = *var_fbbtgat_d_dn7_slot;
        let mut var_fbbtgat_d_dn8: f64 = *var_fbbtgat_d_dn8_slot;
        let mut var_fbbtgat_d_dn9: f64 = *var_fbbtgat_d_dn9_slot;
        let mut var_fbbtgat_d_rv: f64 = *var_fbbtgat_d_rv_slot;
        let mut var_fbbtsti_d: f64 = *var_fbbtsti_d_slot;
        let mut var_fbbtsti_d_rv: f64 = *var_fbbtsti_d_rv_slot;
        let mut var_ftdbot_d: f64 = *var_ftdbot_d_slot;
        let mut var_ftdbot_d_rv: f64 = *var_ftdbot_d_rv_slot;
        let mut var_ftdgat2nd_d: f64 = *var_ftdgat2nd_d_slot;
        let mut var_ftdgat2nd_d_rv: f64 = *var_ftdgat2nd_d_rv_slot;
        let mut var_ftdgat_d: f64 = *var_ftdgat_d_slot;
        let mut var_ftdgat_d_rv: f64 = *var_ftdgat_d_rv_slot;
        let mut var_ftdsti_d: f64 = *var_ftdsti_d_slot;
        let mut var_ftdsti_d_rv: f64 = *var_ftdsti_d_rv_slot;
        let mut var_guard28: f64 = *var_guard28_slot;
        let mut var_guard28_rv: f64 = *var_guard28_rv_slot;
        let mut var_guard29: f64 = *var_guard29_slot;
        let mut var_guard29_rv: f64 = *var_guard29_rv_slot;
        let mut var_idsatbot_d: f64 = *var_idsatbot_d_slot;
        let mut var_idsatbot_d_rv: f64 = *var_idsatbot_d_rv_slot;
        let mut var_idsatgat_d: f64 = *var_idsatgat_d_slot;
        let mut var_idsatgat_d_rv: f64 = *var_idsatgat_d_rv_slot;
        let mut var_idsatsti_d: f64 = *var_idsatsti_d_slot;
        let mut var_idsatsti_d_rv: f64 = *var_idsatsti_d_rv_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_invnf_rv: f64 = *var_invnf_rv_slot;
        let mut var_jw_i: f64 = *var_jw_i_slot;
        let mut var_jw_i_rv: f64 = *var_jw_i_rv_slot;
        let mut var_l_i: f64 = *var_l_i_slot;
        let mut var_l_i_rv: f64 = *var_l_i_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgdrain_i_rv: f64 = *var_lgdrain_i_rv_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lgsource_i_rv: f64 = *var_lgsource_i_rv_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lsdrain_i_rv: f64 = *var_lsdrain_i_rv_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_lssource_i_rv: f64 = *var_lssource_i_rv_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_nf_i_rv: f64 = *var_nf_i_rv_slot;
        let mut var_pd_i: f64 = *var_pd_i_slot;
        let mut var_pd_i_rv: f64 = *var_pd_i_rv_slot;
        let mut var_phigdbot_d: f64 = *var_phigdbot_d_slot;
        let mut var_phigdbot_d_rv: f64 = *var_phigdbot_d_rv_slot;
        let mut var_phigdgat2nd_d: f64 = *var_phigdgat2nd_d_slot;
        let mut var_phigdgat2nd_d_rv: f64 = *var_phigdgat2nd_d_rv_slot;
        let mut var_phigdgat_d: f64 = *var_phigdgat_d_slot;
        let mut var_phigdgat_d_rv: f64 = *var_phigdgat_d_rv_slot;
        let mut var_phigdsti_d: f64 = *var_phigdsti_d_slot;
        let mut var_phigdsti_d_rv: f64 = *var_phigdsti_d_rv_slot;
        let mut var_ps_i: f64 = *var_ps_i_slot;
        let mut var_ps_i_rv: f64 = *var_ps_i_rv_slot;
        let mut var_qpref2bot_d: f64 = *var_qpref2bot_d_slot;
        let mut var_qpref2bot_d_rv: f64 = *var_qpref2bot_d_rv_slot;
        let mut var_qpref2gat2nd: f64 = *var_qpref2gat2nd_slot;
        let mut var_qpref2gat2nd_d: f64 = *var_qpref2gat2nd_d_slot;
        let mut var_qpref2gat2nd_d_rv: f64 = *var_qpref2gat2nd_d_rv_slot;
        let mut var_qpref2gat2nd_rv: f64 = *var_qpref2gat2nd_rv_slot;
        let mut var_qpref2gat_d: f64 = *var_qpref2gat_d_slot;
        let mut var_qpref2gat_d_rv: f64 = *var_qpref2gat_d_rv_slot;
        let mut var_qpref2sti_d: f64 = *var_qpref2sti_d_slot;
        let mut var_qpref2sti_d_rv: f64 = *var_qpref2sti_d_rv_slot;
        let mut var_qprefbot_d: f64 = *var_qprefbot_d_slot;
        let mut var_qprefbot_d_rv: f64 = *var_qprefbot_d_rv_slot;
        let mut var_qprefgat2nd: f64 = *var_qprefgat2nd_slot;
        let mut var_qprefgat2nd_d: f64 = *var_qprefgat2nd_d_slot;
        let mut var_qprefgat2nd_d_rv: f64 = *var_qprefgat2nd_d_rv_slot;
        let mut var_qprefgat2nd_rv: f64 = *var_qprefgat2nd_rv_slot;
        let mut var_qprefgat_d: f64 = *var_qprefgat_d_slot;
        let mut var_qprefgat_d_rv: f64 = *var_qprefgat_d_rv_slot;
        let mut var_qprefsti_d: f64 = *var_qprefsti_d_slot;
        let mut var_qprefsti_d_rv: f64 = *var_qprefsti_d_rv_slot;
        let mut var_sa_i: f64 = *var_sa_i_slot;
        let mut var_sa_i_rv: f64 = *var_sa_i_rv_slot;
        let mut var_sb_i: f64 = *var_sb_i_slot;
        let mut var_sb_i_rv: f64 = *var_sb_i_rv_slot;
        let mut var_sc_i: f64 = *var_sc_i_slot;
        let mut var_sc_i_rv: f64 = *var_sc_i_rv_slot;
        let mut var_sd_i: f64 = *var_sd_i_slot;
        let mut var_sd_i_rv: f64 = *var_sd_i_rv_slot;
        let mut var_ubibot_d: f64 = *var_ubibot_d_slot;
        let mut var_ubibot_d_rv: f64 = *var_ubibot_d_rv_slot;
        let mut var_ubigat2nd_d: f64 = *var_ubigat2nd_d_slot;
        let mut var_ubigat2nd_d_rv: f64 = *var_ubigat2nd_d_rv_slot;
        let mut var_ubigat_d: f64 = *var_ubigat_d_slot;
        let mut var_ubigat_d_rv: f64 = *var_ubigat_d_rv_slot;
        let mut var_ubisti_d: f64 = *var_ubisti_d_slot;
        let mut var_ubisti_d_rv: f64 = *var_ubisti_d_rv_slot;
        let mut var_vbibot_d: f64 = *var_vbibot_d_slot;
        let mut var_vbibot_d_rv: f64 = *var_vbibot_d_rv_slot;
        let mut var_vbigat2nd_d: f64 = *var_vbigat2nd_d_slot;
        let mut var_vbigat2nd_d_rv: f64 = *var_vbigat2nd_d_rv_slot;
        let mut var_vbigat_d: f64 = *var_vbigat_d_slot;
        let mut var_vbigat_d_rv: f64 = *var_vbigat_d_rv_slot;
        let mut var_vbiinvbot_d: f64 = *var_vbiinvbot_d_slot;
        let mut var_vbiinvbot_d_rv: f64 = *var_vbiinvbot_d_rv_slot;
        let mut var_vbiinvgat2nd_d: f64 = *var_vbiinvgat2nd_d_slot;
        let mut var_vbiinvgat2nd_d_rv: f64 = *var_vbiinvgat2nd_d_rv_slot;
        let mut var_vbiinvgat_d: f64 = *var_vbiinvgat_d_slot;
        let mut var_vbiinvgat_d_rv: f64 = *var_vbiinvgat_d_rv_slot;
        let mut var_vbiinvsti_d: f64 = *var_vbiinvsti_d_slot;
        let mut var_vbiinvsti_d_rv: f64 = *var_vbiinvsti_d_rv_slot;
        let mut var_vbisti_d: f64 = *var_vbisti_d_slot;
        let mut var_vbisti_d_rv: f64 = *var_vbisti_d_rv_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_w_i_rv: f64 = *var_w_i_rv_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_we_rv: f64 = *var_we_rv_slot;

        let (assign2640_e2970,) = {
    if (var_guard27 != 0.0) {
        let assign2640_e2966: f64 = (var_cjogat2nd * var_vbigat2nd);
        let assign2640_e2968: f64 = (assign2640_e2966 * var_one_over_one_minus_pgat2nd);
        (assign2640_e2968,)
    } else {
        (var_qprefgat2nd,)
    }
};
        var_qprefgat2nd = assign2640_e2970;
        var_qprefgat2nd_rv = 0.0;

        let (assign2650_e2976,) = {
    if (var_guard27 != 0.0) {
        let assign2650_e2974: f64 = (2.0 * var_cjogat2nd);
        (assign2650_e2974,)
    } else {
        (var_qpref2gat2nd,)
    }
};
        var_qpref2gat2nd = assign2650_e2976;
        var_qpref2gat2nd_rv = 0.0;

        let assign2660_e2979: f64 = (var_phigbotd_i + var_deltaphigd);
        var_phigdbot_d = assign2660_e2979;
        var_phigdbot_d_rv = 0.0;

        let assign2670_e2982: f64 = (var_phigstid_i + var_deltaphigd);
        var_phigdsti_d = assign2670_e2982;
        var_phigdsti_d_rv = 0.0;

        let assign2680_e2985: f64 = (var_phiggatd_i + var_deltaphigd);
        var_phigdgat_d = assign2680_e2985;
        var_phigdgat_d_rv = 0.0;

        let assign2690_e2988: f64 = (var_auxt).powf(1.5);
        let assign2690_e2992: f64 = (var_phigrbot_d * var_phitrinv);
        let assign2690_e2995: f64 = (var_phigdbot_d * var_phitdinv);
        let assign2690_e2996: f64 = (assign2690_e2992 - assign2690_e2995);
        let assign2690_e2997: f64 = (0.5 * assign2690_e2996);
        let assign2690_e2998: f64 = (assign2690_e2997).exp();
        let assign2690_e2999: f64 = (assign2690_e2988 * assign2690_e2998);
        var_ftdbot_d = assign2690_e2999;
        var_ftdbot_d_rv = 0.0;

        let assign2700_e3002: f64 = (var_auxt).powf(1.5);
        let assign2700_e3006: f64 = (var_phigrsti_d * var_phitrinv);
        let assign2700_e3009: f64 = (var_phigdsti_d * var_phitdinv);
        let assign2700_e3010: f64 = (assign2700_e3006 - assign2700_e3009);
        let assign2700_e3011: f64 = (0.5 * assign2700_e3010);
        let assign2700_e3012: f64 = (assign2700_e3011).exp();
        let assign2700_e3013: f64 = (assign2700_e3002 * assign2700_e3012);
        var_ftdsti_d = assign2700_e3013;
        var_ftdsti_d_rv = 0.0;

        let assign2710_e3016: f64 = (var_auxt).powf(1.5);
        let assign2710_e3020: f64 = (var_phigrgat_d * var_phitrinv);
        let assign2710_e3023: f64 = (var_phigdgat_d * var_phitdinv);
        let assign2710_e3024: f64 = (assign2710_e3020 - assign2710_e3023);
        let assign2710_e3025: f64 = (0.5 * assign2710_e3024);
        let assign2710_e3026: f64 = (assign2710_e3025).exp();
        let assign2710_e3027: f64 = (assign2710_e3016 * assign2710_e3026);
        var_ftdgat_d = assign2710_e3027;
        var_ftdgat_d_rv = 0.0;

        let assign2720_e3030: f64 = (var_idsatrbotd_i * var_ftdbot_d);
        let assign2720_e3032: f64 = (assign2720_e3030 * var_ftdbot_d);
        var_idsatbot_d = assign2720_e3032;
        var_idsatbot_d_rv = 0.0;

        let assign2730_e3035: f64 = (var_idsatrstid_i * var_ftdsti_d);
        let assign2730_e3037: f64 = (assign2730_e3035 * var_ftdsti_d);
        var_idsatsti_d = assign2730_e3037;
        var_idsatsti_d_rv = 0.0;

        let assign2740_e3040: f64 = (var_idsatrgatd_i * var_ftdgat_d);
        let assign2740_e3042: f64 = (assign2740_e3040 * var_ftdgat_d);
        var_idsatgat_d = assign2740_e3042;
        var_idsatgat_d_rv = 0.0;

        let assign2750_e3045: f64 = (var_vbirbotd_i * var_auxt);
        let assign2750_e3048: f64 = (2.0 * var_phitd);
        let assign2750_e3050: f64 = (var_ftdbot_d).ln();
        let assign2750_e3051: f64 = (assign2750_e3048 * assign2750_e3050);
        let assign2750_e3052: f64 = (assign2750_e3045 - assign2750_e3051);
        var_ubibot_d = assign2750_e3052;
        var_ubibot_d_rv = 0.0;

        let assign2760_e3055: f64 = (var_vbirstid_i * var_auxt);
        let assign2760_e3058: f64 = (2.0 * var_phitd);
        let assign2760_e3060: f64 = (var_ftdsti_d).ln();
        let assign2760_e3061: f64 = (assign2760_e3058 * assign2760_e3060);
        let assign2760_e3062: f64 = (assign2760_e3055 - assign2760_e3061);
        var_ubisti_d = assign2760_e3062;
        var_ubisti_d_rv = 0.0;

        let assign2770_e3065: f64 = (var_vbirgatd_i * var_auxt);
        let assign2770_e3068: f64 = (2.0 * var_phitd);
        let assign2770_e3070: f64 = (var_ftdgat_d).ln();
        let assign2770_e3071: f64 = (assign2770_e3068 * assign2770_e3070);
        let assign2770_e3072: f64 = (assign2770_e3065 - assign2770_e3071);
        var_ubigat_d = assign2770_e3072;
        var_ubigat_d_rv = 0.0;

        let assign2780_e3078: f64 = (0.05 - var_ubibot_d);
        let assign2780_e3080: f64 = (assign2780_e3078 * var_phitdinv);
        let assign2780_e3081: f64 = (assign2780_e3080).exp();
        let assign2780_e3082: f64 = (1.0 + assign2780_e3081);
        let assign2780_e3083: f64 = (assign2780_e3082).ln();
        let assign2780_e3084: f64 = (var_phitd * assign2780_e3083);
        let assign2780_e3085: f64 = (var_ubibot_d + assign2780_e3084);
        var_vbibot_d = assign2780_e3085;
        var_vbibot_d_rv = 0.0;

        let assign2790_e3091: f64 = (0.05 - var_ubisti_d);
        let assign2790_e3093: f64 = (assign2790_e3091 * var_phitdinv);
        let assign2790_e3094: f64 = (assign2790_e3093).exp();
        let assign2790_e3095: f64 = (1.0 + assign2790_e3094);
        let assign2790_e3096: f64 = (assign2790_e3095).ln();
        let assign2790_e3097: f64 = (var_phitd * assign2790_e3096);
        let assign2790_e3098: f64 = (var_ubisti_d + assign2790_e3097);
        var_vbisti_d = assign2790_e3098;
        var_vbisti_d_rv = 0.0;

        let assign2800_e3104: f64 = (0.05 - var_ubigat_d);
        let assign2800_e3106: f64 = (assign2800_e3104 * var_phitdinv);
        let assign2800_e3107: f64 = (assign2800_e3106).exp();
        let assign2800_e3108: f64 = (1.0 + assign2800_e3107);
        let assign2800_e3109: f64 = (assign2800_e3108).ln();
        let assign2800_e3110: f64 = (var_phitd * assign2800_e3109);
        let assign2800_e3111: f64 = (var_ubigat_d + assign2800_e3110);
        var_vbigat_d = assign2800_e3111;
        var_vbigat_d_rv = 0.0;

        let assign2810_e3114: f64 = (1.0 / var_vbibot_d);
        var_vbiinvbot_d = assign2810_e3114;
        var_vbiinvbot_d_rv = 0.0;

        let assign2820_e3117: f64 = (1.0 / var_vbisti_d);
        var_vbiinvsti_d = assign2820_e3117;
        var_vbiinvsti_d_rv = 0.0;

        let assign2830_e3120: f64 = (1.0 / var_vbigat_d);
        var_vbiinvgat_d = assign2830_e3120;
        var_vbiinvgat_d_rv = 0.0;

        let assign2840_e3124: f64 = (var_vbirbotd_i * var_vbiinvbot_d);
        let assign2840_e3126: f64 = (assign2840_e3124).powf(var_pbotd_i);
        let assign2840_e3127: f64 = (var_cjorbotd_i * assign2840_e3126);
        var_cjobot_d = assign2840_e3127;
        var_cjobot_d_rv = 0.0;

        let assign2850_e3131: f64 = (var_vbirstid_i * var_vbiinvsti_d);
        let assign2850_e3133: f64 = (assign2850_e3131).powf(var_pstid_i);
        let assign2850_e3134: f64 = (var_cjorstid_i * assign2850_e3133);
        var_cjosti_d = assign2850_e3134;
        var_cjosti_d_rv = 0.0;

        let assign2860_e3138: f64 = (var_vbirgatd_i * var_vbiinvgat_d);
        let assign2860_e3140: f64 = (assign2860_e3138).powf(var_pgatd_i);
        let assign2860_e3141: f64 = (var_cjorgatd_i * assign2860_e3140);
        var_cjogat_d = assign2860_e3141;
        var_cjogat_d_rv = 0.0;

        let assign2870_e3144: f64 = (var_cjobot_d * var_vbibot_d);
        let assign2870_e3146: f64 = (assign2870_e3144 * var_one_over_one_minus_pbot_d);
        var_qprefbot_d = assign2870_e3146;
        var_qprefbot_d_rv = 0.0;

        let assign2880_e3149: f64 = (var_cjosti_d * var_vbisti_d);
        let assign2880_e3151: f64 = (assign2880_e3149 * var_one_over_one_minus_psti_d);
        var_qprefsti_d = assign2880_e3151;
        var_qprefsti_d_rv = 0.0;

        let assign2890_e3154: f64 = (var_cjogat_d * var_vbigat_d);
        let assign2890_e3156: f64 = (assign2890_e3154 * var_one_over_one_minus_pgat_d);
        var_qprefgat_d = assign2890_e3156;
        var_qprefgat_d_rv = 0.0;

        let assign2900_e3159: f64 = (2.0 * var_cjobot_d);
        var_qpref2bot_d = assign2900_e3159;
        var_qpref2bot_d_rv = 0.0;

        let assign2910_e3162: f64 = (2.0 * var_cjosti_d);
        var_qpref2sti_d = assign2910_e3162;
        var_qpref2sti_d_rv = 0.0;

        let assign2920_e3165: f64 = (2.0 * var_cjogat_d);
        var_qpref2gat_d = assign2920_e3165;
        var_qpref2gat_d_rv = 0.0;

        let assign2930_e3168: f64 = (0.5 * var_phigdbot_d);
        let assign2930_e3170: f64 = (assign2930_e3168).max(var_phitd);
        var_deltaebot_d = assign2930_e3170;
        var_deltaebot_d_rv = 0.0;

        let assign2940_e3173: f64 = (0.5 * var_phigdsti_d);
        let assign2940_e3175: f64 = (assign2940_e3173).max(var_phitd);
        var_deltaesti_d = assign2940_e3175;
        var_deltaesti_d_rv = 0.0;

        let assign2950_e3178: f64 = (0.5 * var_phigdgat_d);
        let assign2950_e3180: f64 = (assign2950_e3178).max(var_phitd);
        var_deltaegat_d = assign2950_e3180;
        var_deltaegat_d_rv = 0.0;

        let assign2960_e3183: f64 = (var_deltaebot_d * var_phitdinv);
        var_atatbot_d = assign2960_e3183;
        var_atatbot_d_rv = 0.0;

        let assign2970_e3186: f64 = (var_deltaesti_d * var_phitdinv);
        var_atatsti_d = assign2970_e3186;
        var_atatsti_d_rv = 0.0;

        let assign2980_e3189: f64 = (var_deltaegat_d * var_phitdinv);
        var_atatgat_d = assign2980_e3189;
        var_atatgat_d_rv = 0.0;

        let assign2990_e3192: f64 = (32.0 * var_mefftatbotd_i);
        let assign2990_e3194: f64 = (assign2990_e3192 * 9.1093826e-31);
        let assign2990_e3196: f64 = (assign2990_e3194 * 1.6021918e-19);
        let assign2990_e3199: f64 = (var_deltaebot_d * var_deltaebot_d);
        let assign2990_e3201: f64 = (assign2990_e3199 * var_deltaebot_d);
        let assign2990_e3202: f64 = (assign2990_e3196 * assign2990_e3201);
        let assign2990_e3203: f64 = (assign2990_e3202).sqrt();
        let assign2990_e3206: f64 = (3.0 * 1.05457168e-34);
        let assign2990_e3207: f64 = (assign2990_e3203 / assign2990_e3206);
        var_btatpartbot_d = assign2990_e3207;
        var_btatpartbot_d_rv = 0.0;

        let assign3000_e3210: f64 = (32.0 * var_mefftatstid_i);
        let assign3000_e3212: f64 = (assign3000_e3210 * 9.1093826e-31);
        let assign3000_e3214: f64 = (assign3000_e3212 * 1.6021918e-19);
        let assign3000_e3217: f64 = (var_deltaesti_d * var_deltaesti_d);
        let assign3000_e3219: f64 = (assign3000_e3217 * var_deltaesti_d);
        let assign3000_e3220: f64 = (assign3000_e3214 * assign3000_e3219);
        let assign3000_e3221: f64 = (assign3000_e3220).sqrt();
        let assign3000_e3224: f64 = (3.0 * 1.05457168e-34);
        let assign3000_e3225: f64 = (assign3000_e3221 / assign3000_e3224);
        var_btatpartsti_d = assign3000_e3225;
        var_btatpartsti_d_rv = 0.0;

        let assign3010_e3228: f64 = (32.0 * var_mefftatgatd_i);
        let assign3010_e3230: f64 = (assign3010_e3228 * 9.1093826e-31);
        let assign3010_e3232: f64 = (assign3010_e3230 * 1.6021918e-19);
        let assign3010_e3235: f64 = (var_deltaegat_d * var_deltaegat_d);
        let assign3010_e3237: f64 = (assign3010_e3235 * var_deltaegat_d);
        let assign3010_e3238: f64 = (assign3010_e3232 * assign3010_e3237);
        let assign3010_e3239: f64 = (assign3010_e3238).sqrt();
        let assign3010_e3242: f64 = (3.0 * 1.05457168e-34);
        let assign3010_e3243: f64 = (assign3010_e3239 / assign3010_e3242);
        var_btatpartgat_d = assign3010_e3243;
        var_btatpartgat_d_rv = 0.0;

        let assign3020_e3249: f64 = (var_tkd_1 - var_tkr_1);
        let assign3020_e3250: f64 = (var_stfbbtbotd_i * assign3020_e3249);
        let assign3020_e3251: f64 = (1.0 + assign3020_e3250);
        let assign3020_e3252: f64 = (var_fbbtrbotd_i * assign3020_e3251);
        var_fbbtbot_d = assign3020_e3252;
        var_fbbtbot_d_rv = 0.0;

        let assign3030_e3258: f64 = (var_tkd_1 - var_tkr_1);
        let assign3030_e3259: f64 = (var_stfbbtstid_i * assign3030_e3258);
        let assign3030_e3260: f64 = (1.0 + assign3030_e3259);
        let assign3030_e3261: f64 = (var_fbbtrstid_i * assign3030_e3260);
        var_fbbtsti_d = assign3030_e3261;
        var_fbbtsti_d_rv = 0.0;

        let assign3040_e3267: f64 = (var_tkd_1 - var_tkr_1);
        let assign3040_e3268: f64 = (var_stfbbtgatd_i * assign3040_e3267);
        let assign3040_e3269: f64 = (1.0 + assign3040_e3268);
        let assign3040_e3270: f64 = (var_fbbtrgatd_i * assign3040_e3269);
        var_fbbtgat_d = assign3040_e3270;
        var_fbbtgat_d_dn6 = 0.0;
        var_fbbtgat_d_dn7 = 0.0;
        var_fbbtgat_d_dn8 = 0.0;
        var_fbbtgat_d_dn9 = 0.0;
        var_fbbtgat_d_rv = 0.0;

        let (assign3050_e3276,) = {
    if (var_fbbtbot_d > 0.0) {
        (var_fbbtbot_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtbot_d = assign3050_e3276;
        var_fbbtbot_d_rv = 0.0;

        let (assign3060_e3282,) = {
    if (var_fbbtsti_d > 0.0) {
        (var_fbbtsti_d,)
    } else {
        (0.0,)
    }
};
        var_fbbtsti_d = assign3060_e3282;
        var_fbbtsti_d_rv = 0.0;

        let (assign3070_e3288, assign3070_e3288_d_n6, assign3070_e3288_d_n7, assign3070_e3288_d_n8, assign3070_e3288_d_n9,) = {
    if (var_fbbtgat_d > 0.0) {
        (var_fbbtgat_d, var_fbbtgat_d_dn6, var_fbbtgat_d_dn7, var_fbbtgat_d_dn8, var_fbbtgat_d_dn9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_fbbtgat_d = assign3070_e3288;
        var_fbbtgat_d_dn6 = assign3070_e3288_d_n6;
        var_fbbtgat_d_dn7 = assign3070_e3288_d_n7;
        var_fbbtgat_d_dn8 = assign3070_e3288_d_n8;
        var_fbbtgat_d_dn9 = assign3070_e3288_d_n9;
        var_fbbtgat_d_rv = 0.0;

        let assign3080_e3291: f64 = if var_swgat2nd_d == 1.0 { 1.0 } else { 0.0 };
        var_guard28 = assign3080_e3291;
        var_guard28_rv = 0.0;

        let (assign3090_e3297,) = {
    if (var_guard28 != 0.0) {
        let assign3090_e3295: f64 = (var_phiggat2nd_d + var_deltaphigd);
        (assign3090_e3295,)
    } else {
        (var_phigdgat2nd_d,)
    }
};
        var_phigdgat2nd_d = assign3090_e3297;
        var_phigdgat2nd_d_rv = 0.0;

        let (assign3100_e3314,) = {
    if (var_guard28 != 0.0) {
        let assign3100_e3301: f64 = (var_auxt).powf(1.5);
        let assign3100_e3305: f64 = (var_phigrgat2nd_d * var_phitrinv);
        let assign3100_e3308: f64 = (var_phigdgat2nd_d * var_phitdinv);
        let assign3100_e3309: f64 = (assign3100_e3305 - assign3100_e3308);
        let assign3100_e3310: f64 = (0.5 * assign3100_e3309);
        let assign3100_e3311: f64 = (assign3100_e3310).exp();
        let assign3100_e3312: f64 = (assign3100_e3301 * assign3100_e3311);
        (assign3100_e3312,)
    } else {
        (var_ftdgat2nd_d,)
    }
};
        var_ftdgat2nd_d = assign3100_e3314;
        var_ftdgat2nd_d_rv = 0.0;

        let (assign3110_e3327,) = {
    if (var_guard28 != 0.0) {
        let assign3110_e3318: f64 = (var_vbirgat2nd_d * var_auxt);
        let assign3110_e3321: f64 = (2.0 * var_phitd);
        let assign3110_e3323: f64 = (var_ftdgat2nd_d).ln();
        let assign3110_e3324: f64 = (assign3110_e3321 * assign3110_e3323);
        let assign3110_e3325: f64 = (assign3110_e3318 - assign3110_e3324);
        (assign3110_e3325,)
    } else {
        (var_ubigat2nd_d,)
    }
};
        var_ubigat2nd_d = assign3110_e3327;
        var_ubigat2nd_d_rv = 0.0;

        let (assign3120_e3343,) = {
    if (var_guard28 != 0.0) {
        let assign3120_e3334: f64 = (0.05 - var_ubigat2nd_d);
        let assign3120_e3336: f64 = (assign3120_e3334 * var_phitdinv);
        let assign3120_e3337: f64 = (assign3120_e3336).exp();
        let assign3120_e3338: f64 = (1.0 + assign3120_e3337);
        let assign3120_e3339: f64 = (assign3120_e3338).ln();
        let assign3120_e3340: f64 = (var_phitd * assign3120_e3339);
        let assign3120_e3341: f64 = (var_ubigat2nd_d + assign3120_e3340);
        (assign3120_e3341,)
    } else {
        (var_vbigat2nd_d,)
    }
};
        var_vbigat2nd_d = assign3120_e3343;
        var_vbigat2nd_d_rv = 0.0;

        let (assign3130_e3349,) = {
    if (var_guard28 != 0.0) {
        let assign3130_e3347: f64 = (1.0 / var_vbigat2nd_d);
        (assign3130_e3347,)
    } else {
        (var_vbiinvgat2nd_d,)
    }
};
        var_vbiinvgat2nd_d = assign3130_e3349;
        var_vbiinvgat2nd_d_rv = 0.0;

        let (assign3140_e3359,) = {
    if (var_guard28 != 0.0) {
        let assign3140_e3354: f64 = (var_vbirgat2nd_d * var_vbiinvgat2nd_d);
        let assign3140_e3356: f64 = (assign3140_e3354).powf(var_pgat2nd_d);
        let assign3140_e3357: f64 = (var_cjorgat2nd_d * assign3140_e3356);
        (assign3140_e3357,)
    } else {
        (var_cjogat2nd_d,)
    }
};
        var_cjogat2nd_d = assign3140_e3359;
        var_cjogat2nd_d_rv = 0.0;

        let (assign3150_e3367,) = {
    if (var_guard28 != 0.0) {
        let assign3150_e3363: f64 = (var_cjogat2nd_d * var_vbigat2nd_d);
        let assign3150_e3365: f64 = (assign3150_e3363 * var_one_over_one_minus_pgat2nd_d);
        (assign3150_e3365,)
    } else {
        (var_qprefgat2nd_d,)
    }
};
        var_qprefgat2nd_d = assign3150_e3367;
        var_qprefgat2nd_d_rv = 0.0;

        let (assign3160_e3373,) = {
    if (var_guard28 != 0.0) {
        let assign3160_e3371: f64 = (2.0 * var_cjogat2nd_d);
        (assign3160_e3371,)
    } else {
        (var_qpref2gat2nd_d,)
    }
};
        var_qpref2gat2nd_d = assign3160_e3373;
        var_qpref2gat2nd_d_rv = 0.0;

        var_nf_i = 1.0;
        var_nf_i_rv = 0.0;

        var_invnf = 1.0;
        var_invnf_rv = 0.0;

        var_le = 0.0;
        var_le_rv = 0.0;

        var_we = 0.0;
        var_we_rv = 0.0;

        var_l_i = p.p0;
        var_l_i_rv = 0.0;

        var_w_i = p.p1;
        var_w_i_rv = 0.0;

        var_sa_i = p.p2;
        var_sa_i_rv = 0.0;

        var_sb_i = p.p3;
        var_sb_i_rv = 0.0;

        var_sd_i = p.p4;
        var_sd_i_rv = 0.0;

        var_sc_i = p.p8;
        var_sc_i_rv = 0.0;

        var_absource_i = p.p19;
        var_absource_i_rv = 0.0;

        var_lssource_i = p.p20;
        var_lssource_i_rv = 0.0;

        var_lgsource_i = p.p21;
        var_lgsource_i_rv = 0.0;

        var_abdrain_i = p.p22;
        var_abdrain_i_rv = 0.0;

        var_lsdrain_i = p.p23;
        var_lsdrain_i_rv = 0.0;

        var_lgdrain_i = p.p24;
        var_lgdrain_i_rv = 0.0;

        var_as_i = p.p25;
        var_as_i_rv = 0.0;

        var_ps_i = p.p26;
        var_ps_i_rv = 0.0;

        var_ad_i = p.p27;
        var_ad_i_rv = 0.0;

        var_pd_i = p.p28;
        var_pd_i_rv = 0.0;

        var_jw_i = p.p14;
        var_jw_i_rv = 0.0;

        let assign3390_e3398: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard29 = assign3390_e3398;
        var_guard29_rv = 0.0;

        let (assign3400_e3407,) = {
    if (var_guard29 != 0.0) {
        let (assign3400_e3405,) = {
            if (p.p9 > 1.0) {
                (p.p9,)
            } else {
                (1.0,)
            }
        };
        (assign3400_e3405,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3400_e3407;
        var_nf_i_rv = 0.0;

        *var_abdrain_i_slot = var_abdrain_i;
        *var_abdrain_i_rv_slot = var_abdrain_i_rv;
        *var_absource_i_slot = var_absource_i;
        *var_absource_i_rv_slot = var_absource_i_rv;
        *var_ad_i_slot = var_ad_i;
        *var_ad_i_rv_slot = var_ad_i_rv;
        *var_as_i_slot = var_as_i;
        *var_as_i_rv_slot = var_as_i_rv;
        *var_atatbot_d_slot = var_atatbot_d;
        *var_atatbot_d_rv_slot = var_atatbot_d_rv;
        *var_atatgat_d_slot = var_atatgat_d;
        *var_atatgat_d_rv_slot = var_atatgat_d_rv;
        *var_atatsti_d_slot = var_atatsti_d;
        *var_atatsti_d_rv_slot = var_atatsti_d_rv;
        *var_btatpartbot_d_slot = var_btatpartbot_d;
        *var_btatpartbot_d_rv_slot = var_btatpartbot_d_rv;
        *var_btatpartgat_d_slot = var_btatpartgat_d;
        *var_btatpartgat_d_rv_slot = var_btatpartgat_d_rv;
        *var_btatpartsti_d_slot = var_btatpartsti_d;
        *var_btatpartsti_d_rv_slot = var_btatpartsti_d_rv;
        *var_cjobot_d_slot = var_cjobot_d;
        *var_cjobot_d_rv_slot = var_cjobot_d_rv;
        *var_cjogat2nd_d_slot = var_cjogat2nd_d;
        *var_cjogat2nd_d_rv_slot = var_cjogat2nd_d_rv;
        *var_cjogat_d_slot = var_cjogat_d;
        *var_cjogat_d_rv_slot = var_cjogat_d_rv;
        *var_cjosti_d_slot = var_cjosti_d;
        *var_cjosti_d_rv_slot = var_cjosti_d_rv;
        *var_deltaebot_d_slot = var_deltaebot_d;
        *var_deltaebot_d_rv_slot = var_deltaebot_d_rv;
        *var_deltaegat_d_slot = var_deltaegat_d;
        *var_deltaegat_d_rv_slot = var_deltaegat_d_rv;
        *var_deltaesti_d_slot = var_deltaesti_d;
        *var_deltaesti_d_rv_slot = var_deltaesti_d_rv;
        *var_fbbtbot_d_slot = var_fbbtbot_d;
        *var_fbbtbot_d_rv_slot = var_fbbtbot_d_rv;
        *var_fbbtgat_d_slot = var_fbbtgat_d;
        *var_fbbtgat_d_dn6_slot = var_fbbtgat_d_dn6;
        *var_fbbtgat_d_dn7_slot = var_fbbtgat_d_dn7;
        *var_fbbtgat_d_dn8_slot = var_fbbtgat_d_dn8;
        *var_fbbtgat_d_dn9_slot = var_fbbtgat_d_dn9;
        *var_fbbtgat_d_rv_slot = var_fbbtgat_d_rv;
        *var_fbbtsti_d_slot = var_fbbtsti_d;
        *var_fbbtsti_d_rv_slot = var_fbbtsti_d_rv;
        *var_ftdbot_d_slot = var_ftdbot_d;
        *var_ftdbot_d_rv_slot = var_ftdbot_d_rv;
        *var_ftdgat2nd_d_slot = var_ftdgat2nd_d;
        *var_ftdgat2nd_d_rv_slot = var_ftdgat2nd_d_rv;
        *var_ftdgat_d_slot = var_ftdgat_d;
        *var_ftdgat_d_rv_slot = var_ftdgat_d_rv;
        *var_ftdsti_d_slot = var_ftdsti_d;
        *var_ftdsti_d_rv_slot = var_ftdsti_d_rv;
        *var_guard28_slot = var_guard28;
        *var_guard28_rv_slot = var_guard28_rv;
        *var_guard29_slot = var_guard29;
        *var_guard29_rv_slot = var_guard29_rv;
        *var_idsatbot_d_slot = var_idsatbot_d;
        *var_idsatbot_d_rv_slot = var_idsatbot_d_rv;
        *var_idsatgat_d_slot = var_idsatgat_d;
        *var_idsatgat_d_rv_slot = var_idsatgat_d_rv;
        *var_idsatsti_d_slot = var_idsatsti_d;
        *var_idsatsti_d_rv_slot = var_idsatsti_d_rv;
        *var_invnf_slot = var_invnf;
        *var_invnf_rv_slot = var_invnf_rv;
        *var_jw_i_slot = var_jw_i;
        *var_jw_i_rv_slot = var_jw_i_rv;
        *var_l_i_slot = var_l_i;
        *var_l_i_rv_slot = var_l_i_rv;
        *var_le_slot = var_le;
        *var_le_rv_slot = var_le_rv;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgdrain_i_rv_slot = var_lgdrain_i_rv;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lgsource_i_rv_slot = var_lgsource_i_rv;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lsdrain_i_rv_slot = var_lsdrain_i_rv;
        *var_lssource_i_slot = var_lssource_i;
        *var_lssource_i_rv_slot = var_lssource_i_rv;
        *var_nf_i_slot = var_nf_i;
        *var_nf_i_rv_slot = var_nf_i_rv;
        *var_pd_i_slot = var_pd_i;
        *var_pd_i_rv_slot = var_pd_i_rv;
        *var_phigdbot_d_slot = var_phigdbot_d;
        *var_phigdbot_d_rv_slot = var_phigdbot_d_rv;
        *var_phigdgat2nd_d_slot = var_phigdgat2nd_d;
        *var_phigdgat2nd_d_rv_slot = var_phigdgat2nd_d_rv;
        *var_phigdgat_d_slot = var_phigdgat_d;
        *var_phigdgat_d_rv_slot = var_phigdgat_d_rv;
        *var_phigdsti_d_slot = var_phigdsti_d;
        *var_phigdsti_d_rv_slot = var_phigdsti_d_rv;
        *var_ps_i_slot = var_ps_i;
        *var_ps_i_rv_slot = var_ps_i_rv;
        *var_qpref2bot_d_slot = var_qpref2bot_d;
        *var_qpref2bot_d_rv_slot = var_qpref2bot_d_rv;
        *var_qpref2gat2nd_slot = var_qpref2gat2nd;
        *var_qpref2gat2nd_d_slot = var_qpref2gat2nd_d;
        *var_qpref2gat2nd_d_rv_slot = var_qpref2gat2nd_d_rv;
        *var_qpref2gat2nd_rv_slot = var_qpref2gat2nd_rv;
        *var_qpref2gat_d_slot = var_qpref2gat_d;
        *var_qpref2gat_d_rv_slot = var_qpref2gat_d_rv;
        *var_qpref2sti_d_slot = var_qpref2sti_d;
        *var_qpref2sti_d_rv_slot = var_qpref2sti_d_rv;
        *var_qprefbot_d_slot = var_qprefbot_d;
        *var_qprefbot_d_rv_slot = var_qprefbot_d_rv;
        *var_qprefgat2nd_slot = var_qprefgat2nd;
        *var_qprefgat2nd_d_slot = var_qprefgat2nd_d;
        *var_qprefgat2nd_d_rv_slot = var_qprefgat2nd_d_rv;
        *var_qprefgat2nd_rv_slot = var_qprefgat2nd_rv;
        *var_qprefgat_d_slot = var_qprefgat_d;
        *var_qprefgat_d_rv_slot = var_qprefgat_d_rv;
        *var_qprefsti_d_slot = var_qprefsti_d;
        *var_qprefsti_d_rv_slot = var_qprefsti_d_rv;
        *var_sa_i_slot = var_sa_i;
        *var_sa_i_rv_slot = var_sa_i_rv;
        *var_sb_i_slot = var_sb_i;
        *var_sb_i_rv_slot = var_sb_i_rv;
        *var_sc_i_slot = var_sc_i;
        *var_sc_i_rv_slot = var_sc_i_rv;
        *var_sd_i_slot = var_sd_i;
        *var_sd_i_rv_slot = var_sd_i_rv;
        *var_ubibot_d_slot = var_ubibot_d;
        *var_ubibot_d_rv_slot = var_ubibot_d_rv;
        *var_ubigat2nd_d_slot = var_ubigat2nd_d;
        *var_ubigat2nd_d_rv_slot = var_ubigat2nd_d_rv;
        *var_ubigat_d_slot = var_ubigat_d;
        *var_ubigat_d_rv_slot = var_ubigat_d_rv;
        *var_ubisti_d_slot = var_ubisti_d;
        *var_ubisti_d_rv_slot = var_ubisti_d_rv;
        *var_vbibot_d_slot = var_vbibot_d;
        *var_vbibot_d_rv_slot = var_vbibot_d_rv;
        *var_vbigat2nd_d_slot = var_vbigat2nd_d;
        *var_vbigat2nd_d_rv_slot = var_vbigat2nd_d_rv;
        *var_vbigat_d_slot = var_vbigat_d;
        *var_vbigat_d_rv_slot = var_vbigat_d_rv;
        *var_vbiinvbot_d_slot = var_vbiinvbot_d;
        *var_vbiinvbot_d_rv_slot = var_vbiinvbot_d_rv;
        *var_vbiinvgat2nd_d_slot = var_vbiinvgat2nd_d;
        *var_vbiinvgat2nd_d_rv_slot = var_vbiinvgat2nd_d_rv;
        *var_vbiinvgat_d_slot = var_vbiinvgat_d;
        *var_vbiinvgat_d_rv_slot = var_vbiinvgat_d_rv;
        *var_vbiinvsti_d_slot = var_vbiinvsti_d;
        *var_vbiinvsti_d_rv_slot = var_vbiinvsti_d_rv;
        *var_vbisti_d_slot = var_vbisti_d;
        *var_vbisti_d_rv_slot = var_vbisti_d_rv;
        *var_w_i_slot = var_w_i;
        *var_w_i_rv_slot = var_w_i_rv;
        *var_we_slot = var_we;
        *var_we_rv_slot = var_we_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard29: f64,
        var_l_i: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a2_p_rv_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_a4_p_rv_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp2_p_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidl_p_rv_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_bgidld_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfb_p_rv_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfd_p_rv_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidl_p_rv_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgidld_p_rv_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_chib_p_rv_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_dellps_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_delwod_slot: &mut f64,
        var_delwod_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_dvsbnud_p_rv_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_epsrox_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_feta_p_rv_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2_p_rv_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ov_p_rv_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc2ovd_p_rv_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3_p_rv_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ov_p_rv_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gc3ovd_p_rv_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gco_p_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_guard30_slot: &mut f64,
        var_guard30_rv_slot: &mut f64,
        var_guard31_slot: &mut f64,
        var_guard31_rv_slot: &mut f64,
        var_guard32_slot: &mut f64,
        var_guard32_rv_slot: &mut f64,
        var_guard33_slot: &mut f64,
        var_guard33_rv_slot: &mut f64,
        var_iae_slot: &mut f64,
        var_iae_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_iiae_slot: &mut f64,
        var_iiae_rv_slot: &mut f64,
        var_iilcv_slot: &mut f64,
        var_iilcv_rv_slot: &mut f64,
        var_iiwcv_slot: &mut f64,
        var_iiwcv_rv_slot: &mut f64,
        var_iiwe_slot: &mut f64,
        var_iiwe_rv_slot: &mut f64,
        var_iiwecv_slot: &mut f64,
        var_iiwecv_rv_slot: &mut f64,
        var_il_slot: &mut f64,
        var_il_rv_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_ile2_slot: &mut f64,
        var_ile2_rv_slot: &mut f64,
        var_ile_rv_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_imaxii_p_rv_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_invnf_rv_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_iw_rv_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_iwe_rv_slot: &mut f64,
        var_lcv_slot: &mut f64,
        var_lcv_rv_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_rv_slot: &mut f64,
        var_lecv_slot: &mut f64,
        var_lecv_rv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_mue_p_rv_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neff_p_rv_slot: &mut f64,
        var_nf_i_slot: &mut f64,
        var_nf_i_rv_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_nov_p_rv_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_novd_p_rv_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_np_p_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psceb_p_rv_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_psced_p_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsb_p_rv_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rsg_p_rv_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_sca_i_rv_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scb_i_rv_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_scc_i_rv_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_st2vfb_p_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbet_p_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stcs_p_rv_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_stmue_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthecs_p_rv_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthemu_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_stxcor_p_rv_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thecs_p_rv_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_themu_p_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatb_p_rv_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatg_p_rv_slot: &mut f64,
        var_thesatt_p_slot: &mut f64,
        var_thesatt_p_rv_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_tox_p_rv_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxov_p_rv_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_toxovd_p_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vp_p_slot: &mut f64,
        var_vp_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
        var_w_i_slot: &mut f64,
        var_w_i_rv_slot: &mut f64,
        var_wcv_slot: &mut f64,
        var_wcv_rv_slot: &mut f64,
        var_we_slot: &mut f64,
        var_we_rv_slot: &mut f64,
        var_wecv_slot: &mut f64,
        var_wecv_rv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xcor_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
        let mut var_a2_p: f64 = *var_a2_p_slot;
        let mut var_a2_p_rv: f64 = *var_a2_p_rv_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_a4_p_rv: f64 = *var_a4_p_rv_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp2_p_rv: f64 = *var_alp2_p_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidl_p_rv: f64 = *var_bgidl_p_rv_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_bgidld_p_rv: f64 = *var_bgidld_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfb_p_rv: f64 = *var_cfb_p_rv_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfd_p_rv: f64 = *var_cfd_p_rv_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidl_p_rv: f64 = *var_cgidl_p_rv_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgidld_p_rv: f64 = *var_cgidld_p_rv_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_chib_p_rv: f64 = *var_chib_p_rv_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_dellps_rv: f64 = *var_dellps_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
        let mut var_delwod_rv: f64 = *var_delwod_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_dvsbnud_p_rv: f64 = *var_dvsbnud_p_rv_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_epsrox_p_rv: f64 = *var_epsrox_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_feta_p_rv: f64 = *var_feta_p_rv_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2_p_rv: f64 = *var_gc2_p_rv_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ov_p_rv: f64 = *var_gc2ov_p_rv_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc2ovd_p_rv: f64 = *var_gc2ovd_p_rv_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3_p_rv: f64 = *var_gc3_p_rv_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ov_p_rv: f64 = *var_gc3ov_p_rv_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gc3ovd_p_rv: f64 = *var_gc3ovd_p_rv_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gco_p_rv: f64 = *var_gco_p_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_guard30: f64 = *var_guard30_slot;
        let mut var_guard30_rv: f64 = *var_guard30_rv_slot;
        let mut var_guard31: f64 = *var_guard31_slot;
        let mut var_guard31_rv: f64 = *var_guard31_rv_slot;
        let mut var_guard32: f64 = *var_guard32_slot;
        let mut var_guard32_rv: f64 = *var_guard32_rv_slot;
        let mut var_guard33: f64 = *var_guard33_slot;
        let mut var_guard33_rv: f64 = *var_guard33_rv_slot;
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_iae_rv: f64 = *var_iae_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_iiae: f64 = *var_iiae_slot;
        let mut var_iiae_rv: f64 = *var_iiae_rv_slot;
        let mut var_iilcv: f64 = *var_iilcv_slot;
        let mut var_iilcv_rv: f64 = *var_iilcv_rv_slot;
        let mut var_iiwcv: f64 = *var_iiwcv_slot;
        let mut var_iiwcv_rv: f64 = *var_iiwcv_rv_slot;
        let mut var_iiwe: f64 = *var_iiwe_slot;
        let mut var_iiwe_rv: f64 = *var_iiwe_rv_slot;
        let mut var_iiwecv: f64 = *var_iiwecv_slot;
        let mut var_iiwecv_rv: f64 = *var_iiwecv_rv_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_il_rv: f64 = *var_il_rv_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_ile2: f64 = *var_ile2_slot;
        let mut var_ile2_rv: f64 = *var_ile2_rv_slot;
        let mut var_ile_rv: f64 = *var_ile_rv_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_imaxii_p_rv: f64 = *var_imaxii_p_rv_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_invnf_rv: f64 = *var_invnf_rv_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_iw_rv: f64 = *var_iw_rv_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_iwe_rv: f64 = *var_iwe_rv_slot;
        let mut var_lcv: f64 = *var_lcv_slot;
        let mut var_lcv_rv: f64 = *var_lcv_rv_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_rv: f64 = *var_le_rv_slot;
        let mut var_lecv: f64 = *var_lecv_slot;
        let mut var_lecv_rv: f64 = *var_lecv_rv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_mue_p_rv: f64 = *var_mue_p_rv_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neff_p_rv: f64 = *var_neff_p_rv_slot;
        let mut var_nf_i: f64 = *var_nf_i_slot;
        let mut var_nf_i_rv: f64 = *var_nf_i_rv_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_nov_p_rv: f64 = *var_nov_p_rv_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_novd_p_rv: f64 = *var_novd_p_rv_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_np_p_rv: f64 = *var_np_p_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psceb_p_rv: f64 = *var_psceb_p_rv_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_psced_p_rv: f64 = *var_psced_p_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsb_p_rv: f64 = *var_rsb_p_rv_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rsg_p_rv: f64 = *var_rsg_p_rv_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_sca_i_rv: f64 = *var_sca_i_rv_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scb_i_rv: f64 = *var_scb_i_rv_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_scc_i_rv: f64 = *var_scc_i_rv_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_st2vfb_p_rv: f64 = *var_st2vfb_p_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbet_p_rv: f64 = *var_stbet_p_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stcs_p_rv: f64 = *var_stcs_p_rv_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_stmue_p_rv: f64 = *var_stmue_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthecs_p_rv: f64 = *var_stthecs_p_rv_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthemu_p_rv: f64 = *var_stthemu_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_stxcor_p_rv: f64 = *var_stxcor_p_rv_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thecs_p_rv: f64 = *var_thecs_p_rv_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_themu_p_rv: f64 = *var_themu_p_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatb_p_rv: f64 = *var_thesatb_p_rv_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatg_p_rv: f64 = *var_thesatg_p_rv_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_thesatt_p_rv: f64 = *var_thesatt_p_rv_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_tox_p_rv: f64 = *var_tox_p_rv_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxov_p_rv: f64 = *var_toxov_p_rv_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_toxovd_p_rv: f64 = *var_toxovd_p_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_vp_p_rv: f64 = *var_vp_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;
        let mut var_w_i: f64 = *var_w_i_slot;
        let mut var_w_i_rv: f64 = *var_w_i_rv_slot;
        let mut var_wcv: f64 = *var_wcv_slot;
        let mut var_wcv_rv: f64 = *var_wcv_rv_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_we_rv: f64 = *var_we_rv_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_wecv_rv: f64 = *var_wecv_rv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        let (assign3410_e3414,) = {
    if (var_guard29 != 0.0) {
        let assign3410_e3411: f64 = (var_nf_i + 0.5);
        let assign3410_e3412: f64 = (assign3410_e3411).floor();
        (assign3410_e3412,)
    } else {
        (var_nf_i,)
    }
};
        var_nf_i = assign3410_e3414;
        var_nf_i_rv = 0.0;

        let (assign3420_e3420,) = {
    if (var_guard29 != 0.0) {
        let assign3420_e3418: f64 = (1.0 / var_nf_i);
        (assign3420_e3418,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign3420_e3420;
        var_invnf_rv = 0.0;

        let assign3430_e3423: f64 = (var_w_i * var_invnf);
        let (assign3430_e3430,) = {
    if (assign3430_e3423 > 1e-9) {
        let assign3430_e3428: f64 = (var_w_i * var_invnf);
        (assign3430_e3428,)
    } else {
        (1e-9,)
    }
};
        var_w_i = assign3430_e3430;
        var_w_i_rv = 0.0;

        var_sca_i = p.p5;
        var_sca_i_rv = 0.0;

        var_scb_i = p.p6;
        var_scb_i_rv = 0.0;

        var_scc_i = p.p7;
        var_scc_i_rv = 0.0;

        let assign3480_e3442: f64 = (1e-6 / var_l_i);
        var_il = assign3480_e3442;
        var_il_rv = 0.0;

        let assign3490_e3445: f64 = (1e-6 / var_w_i);
        var_iw = assign3490_e3445;
        var_iw_rv = 0.0;

        let assign3500_e3450: f64 = (p.p190 * var_il);
        let assign3500_e3451: f64 = (1.0 + assign3500_e3450);
        let assign3500_e3452: f64 = (p.p189 * assign3500_e3451);
        let assign3500_e3456: f64 = (p.p191 * var_iw);
        let assign3500_e3457: f64 = (1.0 + assign3500_e3456);
        let assign3500_e3458: f64 = (assign3500_e3452 * assign3500_e3457);
        var_dellps = assign3500_e3458;
        var_dellps_rv = 0.0;

        let assign3510_e3463: f64 = (p.p194 * var_il);
        let assign3510_e3464: f64 = (1.0 + assign3510_e3463);
        let assign3510_e3465: f64 = (p.p193 * assign3510_e3464);
        let assign3510_e3469: f64 = (p.p195 * var_iw);
        let assign3510_e3470: f64 = (1.0 + assign3510_e3469);
        let assign3510_e3471: f64 = (assign3510_e3465 * assign3510_e3470);
        var_delwod = assign3510_e3471;
        var_delwod_rv = 0.0;

        let assign3520_e3474: f64 = (var_l_i + var_dellps);
        let assign3520_e3477: f64 = (2.0 * p.p192);
        let assign3520_e3478: f64 = (assign3520_e3474 - assign3520_e3477);
        let (assign3520_e3489,) = {
    if (assign3520_e3478 > 1e-9) {
        let assign3520_e3483: f64 = (var_l_i + var_dellps);
        let assign3520_e3486: f64 = (2.0 * p.p192);
        let assign3520_e3487: f64 = (assign3520_e3483 - assign3520_e3486);
        (assign3520_e3487,)
    } else {
        (1e-9,)
    }
};
        var_le = assign3520_e3489;
        var_le_rv = 0.0;

        let assign3530_e3492: f64 = (var_w_i + var_delwod);
        let assign3530_e3495: f64 = (2.0 * p.p196);
        let assign3530_e3496: f64 = (assign3530_e3492 - assign3530_e3495);
        let (assign3530_e3507,) = {
    if (assign3530_e3496 > 1e-9) {
        let assign3530_e3501: f64 = (var_w_i + var_delwod);
        let assign3530_e3504: f64 = (2.0 * p.p196);
        let assign3530_e3505: f64 = (assign3530_e3501 - assign3530_e3504);
        (assign3530_e3505,)
    } else {
        (1e-9,)
    }
};
        var_we = assign3530_e3507;
        var_we_rv = 0.0;

        let assign3540_e3510: f64 = (1e-6 / var_le);
        var_ile = assign3540_e3510;
        var_ile_rv = 0.0;

        let assign3550_e3513: f64 = (var_ile * var_ile);
        var_ile2 = assign3550_e3513;
        var_ile2_rv = 0.0;

        let assign3560_e3516: f64 = (1e-6 / var_we);
        var_iwe = assign3560_e3516;
        var_iwe_rv = 0.0;

        let assign3570_e3519: f64 = (1.0 / var_iwe);
        var_iiwe = assign3570_e3519;
        var_iiwe_rv = 0.0;

        let assign3580_e3522: f64 = (var_ile * var_iwe);
        var_iae = assign3580_e3522;
        var_iae_rv = 0.0;

        let assign3590_e3525: f64 = (1.0 / var_iae);
        var_iiae = assign3590_e3525;
        var_iiae_rv = 0.0;

        let assign3600_e3528: f64 = (var_l_i + var_dellps);
        let assign3600_e3531: f64 = (2.0 * p.p192);
        let assign3600_e3532: f64 = (assign3600_e3528 - assign3600_e3531);
        let assign3600_e3534: f64 = (assign3600_e3532 + p.p197);
        let (assign3600_e3547,) = {
    if (assign3600_e3534 > 1e-9) {
        let assign3600_e3539: f64 = (var_l_i + var_dellps);
        let assign3600_e3542: f64 = (2.0 * p.p192);
        let assign3600_e3543: f64 = (assign3600_e3539 - assign3600_e3542);
        let assign3600_e3545: f64 = (assign3600_e3543 + p.p197);
        (assign3600_e3545,)
    } else {
        (1e-9,)
    }
};
        var_lecv = assign3600_e3547;
        var_lecv_rv = 0.0;

        let assign3610_e3550: f64 = (var_w_i + var_delwod);
        let assign3610_e3553: f64 = (2.0 * p.p196);
        let assign3610_e3554: f64 = (assign3610_e3550 - assign3610_e3553);
        let assign3610_e3556: f64 = (assign3610_e3554 + p.p198);
        let (assign3610_e3569,) = {
    if (assign3610_e3556 > 1e-9) {
        let assign3610_e3561: f64 = (var_w_i + var_delwod);
        let assign3610_e3564: f64 = (2.0 * p.p196);
        let assign3610_e3565: f64 = (assign3610_e3561 - assign3610_e3564);
        let assign3610_e3567: f64 = (assign3610_e3565 + p.p198);
        (assign3610_e3567,)
    } else {
        (1e-9,)
    }
};
        var_wecv = assign3610_e3569;
        var_wecv_rv = 0.0;

        let assign3620_e3572: f64 = (var_wecv / 1e-6);
        var_iiwecv = assign3620_e3572;
        var_iiwecv_rv = 0.0;

        let assign3630_e3575: f64 = (var_l_i + var_dellps);
        let assign3630_e3577: f64 = (assign3630_e3575 + p.p197);
        let (assign3630_e3586,) = {
    if (assign3630_e3577 > 1e-9) {
        let assign3630_e3582: f64 = (var_l_i + var_dellps);
        let assign3630_e3584: f64 = (assign3630_e3582 + p.p197);
        (assign3630_e3584,)
    } else {
        (1e-9,)
    }
};
        var_lcv = assign3630_e3586;
        var_lcv_rv = 0.0;

        let assign3640_e3589: f64 = (var_w_i + var_delwod);
        let assign3640_e3591: f64 = (assign3640_e3589 + p.p198);
        let (assign3640_e3600,) = {
    if (assign3640_e3591 > 1e-9) {
        let assign3640_e3596: f64 = (var_w_i + var_delwod);
        let assign3640_e3598: f64 = (assign3640_e3596 + p.p198);
        (assign3640_e3598,)
    } else {
        (1e-9,)
    }
};
        var_wcv = assign3640_e3600;
        var_wcv_rv = 0.0;

        let assign3650_e3603: f64 = (var_lcv / 1e-6);
        var_iilcv = assign3650_e3603;
        var_iilcv_rv = 0.0;

        let assign3660_e3606: f64 = (var_wcv / 1e-6);
        var_iiwcv = assign3660_e3606;
        var_iiwcv_rv = 0.0;

        var_vfb_p = p.p56;
        var_vfb_p_rv = 0.0;

        var_stvfb_p = p.p57;
        var_stvfb_p_rv = 0.0;

        var_st2vfb_p = p.p58;
        var_st2vfb_p_rv = 0.0;

        var_tox_p = p.p59;
        var_tox_p_rv = 0.0;

        var_epsrox_p = p.p60;
        var_epsrox_p_rv = 0.0;

        var_neff_p = p.p61;
        var_neff_p_rv = 0.0;

        var_gfacnud_p = p.p62;
        var_gfacnud_p_rv = 0.0;

        var_vsbnud_p = p.p63;
        var_vsbnud_p_rv = 0.0;

        var_dvsbnud_p = p.p64;
        var_dvsbnud_p_rv = 0.0;

        var_dphib_p = p.p65;
        var_dphib_p_rv = 0.0;

        var_np_p = p.p66;
        var_np_p_rv = 0.0;

        var_toxov_p = p.p67;
        var_toxov_p_rv = 0.0;

        var_toxovd_p = p.p68;
        var_toxovd_p_rv = 0.0;

        var_nov_p = p.p69;
        var_nov_p_rv = 0.0;

        var_novd_p = p.p70;
        var_novd_p_rv = 0.0;

        var_ct_p = p.p71;
        var_ct_p_rv = 0.0;

        var_ctg_p = p.p73;
        var_ctg_p_rv = 0.0;

        var_ctb_p = p.p72;
        var_ctb_p_rv = 0.0;

        var_stct_p = p.p74;
        var_stct_p_rv = 0.0;

        var_psce_p = p.p78;
        var_psce_p_rv = 0.0;

        var_psced_p = p.p80;
        var_psced_p_rv = 0.0;

        var_psceb_p = p.p79;
        var_psceb_p_rv = 0.0;

        var_cf_p = p.p75;
        var_cf_p_rv = 0.0;

        var_cfd_p = p.p77;
        var_cfd_p_rv = 0.0;

        var_cfb_p = p.p76;
        var_cfb_p_rv = 0.0;

        var_betn_p = p.p81;
        var_betn_p_rv = 0.0;

        var_stbet_p = p.p82;
        var_stbet_p_rv = 0.0;

        var_mue_p = p.p83;
        var_mue_p_rv = 0.0;

        var_stmue_p = p.p84;
        var_stmue_p_rv = 0.0;

        var_themu_p = p.p85;
        var_themu_p_rv = 0.0;

        var_stthemu_p = p.p86;
        var_stthemu_p_rv = 0.0;

        var_cs_p = p.p87;
        var_cs_p_rv = 0.0;

        var_stcs_p = p.p88;
        var_stcs_p_rv = 0.0;

        var_thecs_p = p.p89;
        var_thecs_p_rv = 0.0;

        var_stthecs_p = p.p90;
        var_stthecs_p_rv = 0.0;

        var_xcor_p = p.p91;
        var_xcor_p_rv = 0.0;

        var_stxcor_p = p.p92;
        var_stxcor_p_rv = 0.0;

        var_feta_p = p.p93;
        var_feta_p_rv = 0.0;

        var_rs_p = p.p94;
        var_rs_p_rv = 0.0;

        var_strs_p = p.p95;
        var_strs_p_rv = 0.0;

        var_rsb_p = p.p96;
        var_rsb_p_rv = 0.0;

        var_rsg_p = p.p97;
        var_rsg_p_rv = 0.0;

        var_thesat_p = p.p98;
        var_thesat_p_rv = 0.0;

        var_stthesat_p = p.p99;
        var_stthesat_p_rv = 0.0;

        var_thesatb_p = p.p100;
        var_thesatb_p_rv = 0.0;

        var_thesatg_p = p.p101;
        var_thesatg_p_rv = 0.0;

        var_thesatt_p = p.p102;
        var_thesatt_p_rv = 0.0;

        var_ax_p = p.p103;
        var_ax_p_rv = 0.0;

        var_alp_p = p.p104;
        var_alp_p_rv = 0.0;

        var_alp1_p = p.p105;
        var_alp1_p_rv = 0.0;

        var_alp2_p = p.p106;
        var_alp2_p_rv = 0.0;

        var_vp_p = p.p107;
        var_vp_p_rv = 0.0;

        var_a1_p = p.p108;
        var_a1_p_rv = 0.0;

        var_a2_p = p.p109;
        var_a2_p_rv = 0.0;

        var_sta2_p = p.p110;
        var_sta2_p_rv = 0.0;

        var_a3_p = p.p111;
        var_a3_p_rv = 0.0;

        var_a4_p = p.p112;
        var_a4_p_rv = 0.0;

        var_imaxii_p = p.p113;
        var_imaxii_p_rv = 0.0;

        var_gco_p = p.p114;
        var_gco_p_rv = 0.0;

        var_iginv_p = p.p115;
        var_iginv_p_rv = 0.0;

        var_igov_p = p.p116;
        var_igov_p_rv = 0.0;

        var_igovd_p = p.p117;
        var_igovd_p_rv = 0.0;

        var_stig_p = p.p118;
        var_stig_p_rv = 0.0;

        var_gc2_p = p.p119;
        var_gc2_p_rv = 0.0;

        var_gc3_p = p.p120;
        var_gc3_p_rv = 0.0;

        var_gc2ov_p = p.p119;
        var_gc2ov_p_rv = 0.0;

        let assign4370_e3718: f64 = if param_given[121] { 1.0 } else { 0.0 };
        let assign4370_e3720: f64 = if assign4370_e3718 == 1.0 { 1.0 } else { 0.0 };
        var_guard30 = assign4370_e3720;
        var_guard30_rv = 0.0;

        let (assign4380_e3724,) = {
    if (var_guard30 != 0.0) {
        (p.p121,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign4380_e3724;
        var_gc2ov_p_rv = 0.0;

        var_gc3ov_p = p.p120;
        var_gc3ov_p_rv = 0.0;

        let assign4400_e3727: f64 = if param_given[122] { 1.0 } else { 0.0 };
        let assign4400_e3729: f64 = if assign4400_e3727 == 1.0 { 1.0 } else { 0.0 };
        var_guard31 = assign4400_e3729;
        var_guard31_rv = 0.0;

        let (assign4410_e3733,) = {
    if (var_guard31 != 0.0) {
        (p.p122,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign4410_e3733;
        var_gc3ov_p_rv = 0.0;

        var_gc2ovd_p = var_gc2ov_p;
        var_gc2ovd_p_rv = 0.0;

        let assign4430_e3736: f64 = if param_given[123] { 1.0 } else { 0.0 };
        let assign4430_e3738: f64 = if assign4430_e3736 == 1.0 { 1.0 } else { 0.0 };
        var_guard32 = assign4430_e3738;
        var_guard32_rv = 0.0;

        let (assign4440_e3742,) = {
    if (var_guard32 != 0.0) {
        (p.p123,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign4440_e3742;
        var_gc2ovd_p_rv = 0.0;

        var_gc3ovd_p = var_gc3ov_p;
        var_gc3ovd_p_rv = 0.0;

        let assign4460_e3745: f64 = if param_given[124] { 1.0 } else { 0.0 };
        let assign4460_e3747: f64 = if assign4460_e3745 == 1.0 { 1.0 } else { 0.0 };
        var_guard33 = assign4460_e3747;
        var_guard33_rv = 0.0;

        let (assign4470_e3751,) = {
    if (var_guard33 != 0.0) {
        (p.p124,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign4470_e3751;
        var_gc3ovd_p_rv = 0.0;

        var_chib_p = p.p125;
        var_chib_p_rv = 0.0;

        var_agidl_p = p.p126;
        var_agidl_p_rv = 0.0;

        var_agidld_p = p.p127;
        var_agidld_p_rv = 0.0;

        var_bgidl_p = p.p128;
        var_bgidl_p_rv = 0.0;

        var_bgidld_p = p.p129;
        var_bgidld_p_rv = 0.0;

        var_stbgidl_p = p.p130;
        var_stbgidl_p_rv = 0.0;

        var_stbgidld_p = p.p131;
        var_stbgidld_p_rv = 0.0;

        var_cgidl_p = p.p132;
        var_cgidl_p_rv = 0.0;

        var_cgidld_p = p.p133;
        var_cgidld_p_rv = 0.0;

        var_cox_p = p.p134;
        var_cox_p_rv = 0.0;

        var_delvtac_p = p.p135;
        var_delvtac_p_rv = 0.0;

        var_facneffac_p = p.p136;
        var_facneffac_p_rv = 0.0;

        var_thesatac_p = p.p98;
        var_thesatac_p_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
        *var_a2_p_slot = var_a2_p;
        *var_a2_p_rv_slot = var_a2_p_rv;
        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_a4_p_slot = var_a4_p;
        *var_a4_p_rv_slot = var_a4_p_rv;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp2_p_rv_slot = var_alp2_p_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidl_p_rv_slot = var_bgidl_p_rv;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_bgidld_p_rv_slot = var_bgidld_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfb_p_rv_slot = var_cfb_p_rv;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfd_p_rv_slot = var_cfd_p_rv;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidl_p_rv_slot = var_cgidl_p_rv;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgidld_p_rv_slot = var_cgidld_p_rv;
        *var_chib_p_slot = var_chib_p;
        *var_chib_p_rv_slot = var_chib_p_rv;
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_dellps_slot = var_dellps;
        *var_dellps_rv_slot = var_dellps_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_delwod_slot = var_delwod;
        *var_delwod_rv_slot = var_delwod_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_dvsbnud_p_rv_slot = var_dvsbnud_p_rv;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_epsrox_p_rv_slot = var_epsrox_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_feta_p_slot = var_feta_p;
        *var_feta_p_rv_slot = var_feta_p_rv;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2_p_rv_slot = var_gc2_p_rv;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ov_p_rv_slot = var_gc2ov_p_rv;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc2ovd_p_rv_slot = var_gc2ovd_p_rv;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3_p_rv_slot = var_gc3_p_rv;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ov_p_rv_slot = var_gc3ov_p_rv;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gc3ovd_p_rv_slot = var_gc3ovd_p_rv;
        *var_gco_p_slot = var_gco_p;
        *var_gco_p_rv_slot = var_gco_p_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_guard30_slot = var_guard30;
        *var_guard30_rv_slot = var_guard30_rv;
        *var_guard31_slot = var_guard31;
        *var_guard31_rv_slot = var_guard31_rv;
        *var_guard32_slot = var_guard32;
        *var_guard32_rv_slot = var_guard32_rv;
        *var_guard33_slot = var_guard33;
        *var_guard33_rv_slot = var_guard33_rv;
        *var_iae_slot = var_iae;
        *var_iae_rv_slot = var_iae_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_iiae_slot = var_iiae;
        *var_iiae_rv_slot = var_iiae_rv;
        *var_iilcv_slot = var_iilcv;
        *var_iilcv_rv_slot = var_iilcv_rv;
        *var_iiwcv_slot = var_iiwcv;
        *var_iiwcv_rv_slot = var_iiwcv_rv;
        *var_iiwe_slot = var_iiwe;
        *var_iiwe_rv_slot = var_iiwe_rv;
        *var_iiwecv_slot = var_iiwecv;
        *var_iiwecv_rv_slot = var_iiwecv_rv;
        *var_il_slot = var_il;
        *var_il_rv_slot = var_il_rv;
        *var_ile_slot = var_ile;
        *var_ile2_slot = var_ile2;
        *var_ile2_rv_slot = var_ile2_rv;
        *var_ile_rv_slot = var_ile_rv;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_imaxii_p_rv_slot = var_imaxii_p_rv;
        *var_invnf_slot = var_invnf;
        *var_invnf_rv_slot = var_invnf_rv;
        *var_iw_slot = var_iw;
        *var_iw_rv_slot = var_iw_rv;
        *var_iwe_slot = var_iwe;
        *var_iwe_rv_slot = var_iwe_rv;
        *var_lcv_slot = var_lcv;
        *var_lcv_rv_slot = var_lcv_rv;
        *var_le_slot = var_le;
        *var_le_rv_slot = var_le_rv;
        *var_lecv_slot = var_lecv;
        *var_lecv_rv_slot = var_lecv_rv;
        *var_mue_p_slot = var_mue_p;
        *var_mue_p_rv_slot = var_mue_p_rv;
        *var_neff_p_slot = var_neff_p;
        *var_neff_p_rv_slot = var_neff_p_rv;
        *var_nf_i_slot = var_nf_i;
        *var_nf_i_rv_slot = var_nf_i_rv;
        *var_nov_p_slot = var_nov_p;
        *var_nov_p_rv_slot = var_nov_p_rv;
        *var_novd_p_slot = var_novd_p;
        *var_novd_p_rv_slot = var_novd_p_rv;
        *var_np_p_slot = var_np_p;
        *var_np_p_rv_slot = var_np_p_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_psceb_p_slot = var_psceb_p;
        *var_psceb_p_rv_slot = var_psceb_p_rv;
        *var_psced_p_slot = var_psced_p;
        *var_psced_p_rv_slot = var_psced_p_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsb_p_rv_slot = var_rsb_p_rv;
        *var_rsg_p_slot = var_rsg_p;
        *var_rsg_p_rv_slot = var_rsg_p_rv;
        *var_sca_i_slot = var_sca_i;
        *var_sca_i_rv_slot = var_sca_i_rv;
        *var_scb_i_slot = var_scb_i;
        *var_scb_i_rv_slot = var_scb_i_rv;
        *var_scc_i_slot = var_scc_i;
        *var_scc_i_rv_slot = var_scc_i_rv;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_st2vfb_p_rv_slot = var_st2vfb_p_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbet_p_rv_slot = var_stbet_p_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_stcs_p_slot = var_stcs_p;
        *var_stcs_p_rv_slot = var_stcs_p_rv;
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_stmue_p_slot = var_stmue_p;
        *var_stmue_p_rv_slot = var_stmue_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthecs_p_rv_slot = var_stthecs_p_rv;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthemu_p_rv_slot = var_stthemu_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_stxcor_p_rv_slot = var_stxcor_p_rv;
        *var_thecs_p_slot = var_thecs_p;
        *var_thecs_p_rv_slot = var_thecs_p_rv;
        *var_themu_p_slot = var_themu_p;
        *var_themu_p_rv_slot = var_themu_p_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatb_p_rv_slot = var_thesatb_p_rv;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatg_p_rv_slot = var_thesatg_p_rv;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_thesatt_p_rv_slot = var_thesatt_p_rv;
        *var_tox_p_slot = var_tox_p;
        *var_tox_p_rv_slot = var_tox_p_rv;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxov_p_rv_slot = var_toxov_p_rv;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_toxovd_p_rv_slot = var_toxovd_p_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vp_p_slot = var_vp_p;
        *var_vp_p_rv_slot = var_vp_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
        *var_w_i_slot = var_w_i;
        *var_w_i_rv_slot = var_w_i_rv;
        *var_wcv_slot = var_wcv;
        *var_wcv_rv_slot = var_wcv_rv;
        *var_we_slot = var_we;
        *var_we_rv_slot = var_we_rv;
        *var_wecv_slot = var_wecv;
        *var_wecv_rv_slot = var_wecv_rv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_aa_slot: &mut f64,
        var_aa_rv_slot: &mut f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_axinr_p_rv_slot: &mut f64,
        var_bb_slot: &mut f64,
        var_bb_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cgovaccg_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_cth_p_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_dvfbinr_p_rv_slot: &mut f64,
        var_dvsbnud_p_slot: &mut f64,
        var_dvsbnud_p_rv_slot: &mut f64,
        var_epsrox_p_slot: &mut f64,
        var_epsrox_p_rv_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovacc_p_rv_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcgovaccd_p_rv_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinracc_p_rv_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_fcinrdep_p_rv_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fnt_p_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_guard34_slot: &mut f64,
        var_guard34_rv_slot: &mut f64,
        var_guard35_slot: &mut f64,
        var_guard35_rv_slot: &mut f64,
        var_guard36_slot: &mut f64,
        var_guard36_rv_slot: &mut f64,
        var_guard37_slot: &mut f64,
        var_guard37_rv_slot: &mut f64,
        var_guard38_slot: &mut f64,
        var_guard38_rv_slot: &mut f64,
        var_lpcke_slot: &mut f64,
        var_lpcke_rv_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neff_p_rv_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_neffedge_p_rv_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_nov_p_rv_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_novd_p_rv_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_np_p_rv_slot: &mut f64,
        var_npcke_slot: &mut f64,
        var_npcke_rv_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub0e_slot: &mut f64,
        var_nsub0e_rv_slot: &mut f64,
        var_nsub_rv_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_st2vfb_p_slot: &mut f64,
        var_st2vfb_p_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_tox_p_slot: &mut f64,
        var_tox_p_rv_slot: &mut f64,
        var_toxov_p_slot: &mut f64,
        var_toxov_p_rv_slot: &mut f64,
        var_toxovd_p_slot: &mut f64,
        var_toxovd_p_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
    ) {
        let mut var_aa: f64 = *var_aa_slot;
        let mut var_aa_rv: f64 = *var_aa_rv_slot;
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_axinr_p_rv: f64 = *var_axinr_p_rv_slot;
        let mut var_bb: f64 = *var_bb_slot;
        let mut var_bb_rv: f64 = *var_bb_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cgovaccg_p_rv: f64 = *var_cgovaccg_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_cth_p_rv: f64 = *var_cth_p_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_dvfbinr_p_rv: f64 = *var_dvfbinr_p_rv_slot;
        let mut var_dvsbnud_p: f64 = *var_dvsbnud_p_slot;
        let mut var_dvsbnud_p_rv: f64 = *var_dvsbnud_p_rv_slot;
        let mut var_epsrox_p: f64 = *var_epsrox_p_slot;
        let mut var_epsrox_p_rv: f64 = *var_epsrox_p_rv_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovacc_p_rv: f64 = *var_fcgovacc_p_rv_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcgovaccd_p_rv: f64 = *var_fcgovaccd_p_rv_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinracc_p_rv: f64 = *var_fcinracc_p_rv_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_fcinrdep_p_rv: f64 = *var_fcinrdep_p_rv_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fnt_p_rv: f64 = *var_fnt_p_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_guard34: f64 = *var_guard34_slot;
        let mut var_guard34_rv: f64 = *var_guard34_rv_slot;
        let mut var_guard35: f64 = *var_guard35_slot;
        let mut var_guard35_rv: f64 = *var_guard35_rv_slot;
        let mut var_guard36: f64 = *var_guard36_slot;
        let mut var_guard36_rv: f64 = *var_guard36_rv_slot;
        let mut var_guard37: f64 = *var_guard37_slot;
        let mut var_guard37_rv: f64 = *var_guard37_rv_slot;
        let mut var_guard38: f64 = *var_guard38_slot;
        let mut var_guard38_rv: f64 = *var_guard38_rv_slot;
        let mut var_lpcke: f64 = *var_lpcke_slot;
        let mut var_lpcke_rv: f64 = *var_lpcke_rv_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neff_p_rv: f64 = *var_neff_p_rv_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_neffedge_p_rv: f64 = *var_neffedge_p_rv_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_nov_p_rv: f64 = *var_nov_p_rv_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_novd_p_rv: f64 = *var_novd_p_rv_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_np_p_rv: f64 = *var_np_p_rv_slot;
        let mut var_npcke: f64 = *var_npcke_slot;
        let mut var_npcke_rv: f64 = *var_npcke_rv_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub0e: f64 = *var_nsub0e_slot;
        let mut var_nsub0e_rv: f64 = *var_nsub0e_rv_slot;
        let mut var_nsub_rv: f64 = *var_nsub_rv_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_st2vfb_p: f64 = *var_st2vfb_p_slot;
        let mut var_st2vfb_p_rv: f64 = *var_st2vfb_p_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_tox_p: f64 = *var_tox_p_slot;
        let mut var_tox_p_rv: f64 = *var_tox_p_rv_slot;
        let mut var_toxov_p: f64 = *var_toxov_p_slot;
        let mut var_toxov_p_rv: f64 = *var_toxov_p_rv_slot;
        let mut var_toxovd_p: f64 = *var_toxovd_p_slot;
        let mut var_toxovd_p_rv: f64 = *var_toxovd_p_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;

        let assign4610_e3766: f64 = if param_given[137] { 1.0 } else { 0.0 };
        let assign4610_e3768: f64 = if assign4610_e3766 == 1.0 { 1.0 } else { 0.0 };
        var_guard34 = assign4610_e3768;
        var_guard34_rv = 0.0;

        let (assign4620_e3772,) = {
    if (var_guard34 != 0.0) {
        (p.p137,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign4620_e3772;
        var_thesatac_p_rv = 0.0;

        var_axac_p = p.p103;
        var_axac_p_rv = 0.0;

        let assign4640_e3775: f64 = if param_given[138] { 1.0 } else { 0.0 };
        let assign4640_e3777: f64 = if assign4640_e3775 == 1.0 { 1.0 } else { 0.0 };
        var_guard35 = assign4640_e3777;
        var_guard35_rv = 0.0;

        let (assign4650_e3781,) = {
    if (var_guard35 != 0.0) {
        (p.p138,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4650_e3781;
        var_axac_p_rv = 0.0;

        var_alpac_p = p.p139;
        var_alpac_p_rv = 0.0;

        var_alp1ac_p = p.p140;
        var_alp1ac_p_rv = 0.0;

        var_cgov_p = p.p141;
        var_cgov_p_rv = 0.0;

        var_cgovd_p = p.p142;
        var_cgovd_p_rv = 0.0;

        var_fcgovacc_p = p.p143;
        var_fcgovacc_p_rv = 0.0;

        var_fcgovaccd_p = p.p144;
        var_fcgovaccd_p_rv = 0.0;

        var_cgovaccg_p = p.p145;
        var_cgovaccg_p_rv = 0.0;

        var_cgbov_p = p.p146;
        var_cgbov_p_rv = 0.0;

        var_cinr_p = p.p147;
        var_cinr_p_rv = 0.0;

        var_cinrd_p = p.p148;
        var_cinrd_p_rv = 0.0;

        var_dvfbinr_p = p.p149;
        var_dvfbinr_p_rv = 0.0;

        var_fcinrdep_p = p.p150;
        var_fcinrdep_p_rv = 0.0;

        var_fcinracc_p = p.p151;
        var_fcinracc_p_rv = 0.0;

        var_axinr_p = p.p152;
        var_axinr_p_rv = 0.0;

        var_cfr_p = p.p153;
        var_cfr_p_rv = 0.0;

        var_cfrd_p = p.p154;
        var_cfrd_p_rv = 0.0;

        var_fnt_p = p.p155;
        var_fnt_p_rv = 0.0;

        var_vfbedge_p = p.p161;
        var_vfbedge_p_rv = 0.0;

        var_stvfbedge_p = p.p162;
        var_stvfbedge_p_rv = 0.0;

        var_dphibedge_p = p.p163;
        var_dphibedge_p_rv = 0.0;

        var_neffedge_p = p.p164;
        var_neffedge_p_rv = 0.0;

        var_ctedge_p = p.p165;
        var_ctedge_p_rv = 0.0;

        var_betnedge_p = p.p166;
        var_betnedge_p_rv = 0.0;

        var_stbetedge_p = p.p167;
        var_stbetedge_p_rv = 0.0;

        var_psceedge_p = p.p168;
        var_psceedge_p_rv = 0.0;

        var_pscebedge_p = p.p169;
        var_pscebedge_p_rv = 0.0;

        var_pscededge_p = p.p170;
        var_pscededge_p_rv = 0.0;

        var_cfedge_p = p.p171;
        var_cfedge_p_rv = 0.0;

        var_cfdedge_p = p.p173;
        var_cfdedge_p_rv = 0.0;

        var_cfbedge_p = p.p172;
        var_cfbedge_p_rv = 0.0;

        var_cth_p = p.p187;
        var_cth_p_rv = 0.0;

        let assign5160_e3834: f64 = if p.p39 > 0.0 { 1.0 } else { 0.0 };
        var_guard36 = assign5160_e3834;
        var_guard36_rv = 0.0;

        let (assign5170_e3852,) = {
    if (var_guard36 != 0.0) {
        let assign5170_e3840: f64 = (var_ile).powf(p.p201);
        let assign5170_e3841: f64 = (p.p200 * assign5170_e3840);
        let assign5170_e3842: f64 = (p.p199 + assign5170_e3841);
        let assign5170_e3845: f64 = (p.p202 * var_iwe);
        let assign5170_e3846: f64 = (assign5170_e3842 + assign5170_e3845);
        let assign5170_e3849: f64 = (p.p203 * var_iae);
        let assign5170_e3850: f64 = (assign5170_e3846 + assign5170_e3849);
        (assign5170_e3850,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign5170_e3852;
        var_vfb_p_rv = 0.0;

        let (assign5180_e3868,) = {
    if (var_guard36 != 0.0) {
        let assign5180_e3857: f64 = (p.p205 * var_ile);
        let assign5180_e3858: f64 = (p.p204 + assign5180_e3857);
        let assign5180_e3861: f64 = (p.p206 * var_iwe);
        let assign5180_e3862: f64 = (assign5180_e3858 + assign5180_e3861);
        let assign5180_e3865: f64 = (p.p207 * var_iae);
        let assign5180_e3866: f64 = (assign5180_e3862 + assign5180_e3865);
        (assign5180_e3866,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign5180_e3868;
        var_stvfb_p_rv = 0.0;

        let (assign5190_e3872,) = {
    if (var_guard36 != 0.0) {
        (p.p208,)
    } else {
        (var_st2vfb_p,)
    }
};
        var_st2vfb_p = assign5190_e3872;
        var_st2vfb_p_rv = 0.0;

        let (assign5200_e3876,) = {
    if (var_guard36 != 0.0) {
        (p.p209,)
    } else {
        (var_tox_p,)
    }
};
        var_tox_p = assign5200_e3876;
        var_tox_p_rv = 0.0;

        let (assign5210_e3880,) = {
    if (var_guard36 != 0.0) {
        (p.p210,)
    } else {
        (var_epsrox_p,)
    }
};
        var_epsrox_p = assign5210_e3880;
        var_epsrox_p_rv = 0.0;

        let (assign5220_e3913,) = {
    if (var_guard36 != 0.0) {
        let assign5220_e3886: f64 = (p.p212 * var_iwe);
        let assign5220_e3890: f64 = (var_we / p.p213);
        let assign5220_e3891: f64 = (1.0 + assign5220_e3890);
        let assign5220_e3892: f64 = (assign5220_e3891).ln();
        let assign5220_e3893: f64 = (assign5220_e3886 * assign5220_e3892);
        let assign5220_e3894: f64 = (1.0 + assign5220_e3893);
        let (assign5220_e3910,) = {
            if (assign5220_e3894 > 0.001) {
                let assign5220_e3900: f64 = (p.p212 * var_iwe);
                let assign5220_e3904: f64 = (var_we / p.p213);
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
        (var_nsub0e,)
    }
};
        var_nsub0e = assign5220_e3913;
        var_nsub0e_rv = 0.0;

        let (assign5230_e3946,) = {
    if (var_guard36 != 0.0) {
        let assign5230_e3919: f64 = (p.p215 * var_iwe);
        let assign5230_e3923: f64 = (var_we / p.p216);
        let assign5230_e3924: f64 = (1.0 + assign5230_e3923);
        let assign5230_e3925: f64 = (assign5230_e3924).ln();
        let assign5230_e3926: f64 = (assign5230_e3919 * assign5230_e3925);
        let assign5230_e3927: f64 = (1.0 + assign5230_e3926);
        let (assign5230_e3943,) = {
            if (assign5230_e3927 > 0.001) {
                let assign5230_e3933: f64 = (p.p215 * var_iwe);
                let assign5230_e3937: f64 = (var_we / p.p216);
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
        (var_npcke,)
    }
};
        var_npcke = assign5230_e3946;
        var_npcke_rv = 0.0;

        let (assign5240_e3979,) = {
    if (var_guard36 != 0.0) {
        let assign5240_e3952: f64 = (p.p218 * var_iwe);
        let assign5240_e3956: f64 = (var_we / p.p216);
        let assign5240_e3957: f64 = (1.0 + assign5240_e3956);
        let assign5240_e3958: f64 = (assign5240_e3957).ln();
        let assign5240_e3959: f64 = (assign5240_e3952 * assign5240_e3958);
        let assign5240_e3960: f64 = (1.0 + assign5240_e3959);
        let (assign5240_e3976,) = {
            if (assign5240_e3960 > 0.001) {
                let assign5240_e3966: f64 = (p.p218 * var_iwe);
                let assign5240_e3970: f64 = (var_we / p.p216);
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
        (var_lpcke,)
    }
};
        var_lpcke = assign5240_e3979;
        var_lpcke_rv = 0.0;

        let assign5250_e3983: f64 = (2.0 * var_lpcke);
        let assign5250_e3984: f64 = if var_le > assign5250_e3983 { 1.0 } else { 0.0 };
        var_guard37 = assign5250_e3984;
        var_guard37_rv = 0.0;

        let (assign5260_e3990,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        (75000000000.0,)
    } else {
        (var_aa,)
    }
};
        var_aa = assign5260_e3990;
        var_aa_rv = 0.0;

        let (assign5270_e4004,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5270_e3997: f64 = (0.5 * var_npcke);
        let assign5270_e3998: f64 = (var_nsub0e + assign5270_e3997);
        let assign5270_e3999: f64 = (assign5270_e3998).sqrt();
        let assign5270_e4001: f64 = (var_nsub0e).sqrt();
        let assign5270_e4002: f64 = (assign5270_e3999 - assign5270_e4001);
        (assign5270_e4002,)
    } else {
        (var_bb,)
    }
};
        var_bb = assign5270_e4004;
        var_bb_rv = 0.0;

        let (assign5280_e4029,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5280_e4009: f64 = (var_nsub0e).sqrt();
        let assign5280_e4014: f64 = (2.0 * var_lpcke);
        let assign5280_e4016: f64 = (assign5280_e4014 / var_le);
        let assign5280_e4019: f64 = (var_bb / var_aa);
        let assign5280_e4020: f64 = (assign5280_e4019).exp();
        let assign5280_e4022: f64 = (assign5280_e4020 - 1.0);
        let assign5280_e4023: f64 = (assign5280_e4016 * assign5280_e4022);
        let assign5280_e4024: f64 = (1.0 + assign5280_e4023);
        let assign5280_e4025: f64 = (assign5280_e4024).ln();
        let assign5280_e4026: f64 = (var_aa * assign5280_e4025);
        let assign5280_e4027: f64 = (assign5280_e4009 + assign5280_e4026);
        (assign5280_e4027,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5280_e4029;
        var_nsub_rv = 0.0;

        let (assign5290_e4037,) = {
    if ((var_guard36 != 0.0) && (var_guard37 != 0.0)) {
        let assign5290_e4035: f64 = (var_nsub * var_nsub);
        (assign5290_e4035,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5290_e4037;
        var_nsub_rv = 0.0;

        let assign5300_e4040: f64 = if var_le >= var_lpcke { 1.0 } else { 0.0 };
        var_guard38 = assign5300_e4040;
        var_guard38_rv = 0.0;

        let (assign5310_e4055,) = {
    if (((var_guard36 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 != 0.0)) {
        let assign5310_e4050: f64 = (var_npcke * var_lpcke);
        let assign5310_e4052: f64 = (assign5310_e4050 / var_le);
        let assign5310_e4053: f64 = (var_nsub0e + assign5310_e4052);
        (assign5310_e4053,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5310_e4055;
        var_nsub_rv = 0.0;

        let (assign5320_e4073,) = {
    if (((var_guard36 != 0.0) && (var_guard37 == 0.0)) && (var_guard38 == 0.0)) {
        let assign5320_e4068: f64 = (var_le / var_lpcke);
        let assign5320_e4069: f64 = (2.0 - assign5320_e4068);
        let assign5320_e4070: f64 = (var_npcke * assign5320_e4069);
        let assign5320_e4071: f64 = (var_nsub0e + assign5320_e4070);
        (assign5320_e4071,)
    } else {
        (var_nsub,)
    }
};
        var_nsub = assign5320_e4073;
        var_nsub_rv = 0.0;

        let (assign5330_e4087,) = {
    if (var_guard36 != 0.0) {
        let assign5330_e4079: f64 = (p.p219 * var_ile);
        let assign5330_e4080: f64 = (1.0 - assign5330_e4079);
        let assign5330_e4083: f64 = (p.p220 * var_ile2);
        let assign5330_e4084: f64 = (assign5330_e4080 - assign5330_e4083);
        let assign5330_e4085: f64 = (var_nsub * assign5330_e4084);
        (assign5330_e4085,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign5330_e4087;
        var_neff_p_rv = 0.0;

        let (assign5340_e4105,) = {
    if (var_guard36 != 0.0) {
        let assign5340_e4093: f64 = (var_ile).powf(p.p223);
        let assign5340_e4094: f64 = (p.p222 * assign5340_e4093);
        let assign5340_e4095: f64 = (p.p221 + assign5340_e4094);
        let assign5340_e4098: f64 = (p.p224 * var_iwe);
        let assign5340_e4099: f64 = (assign5340_e4095 + assign5340_e4098);
        let assign5340_e4102: f64 = (p.p225 * var_iae);
        let assign5340_e4103: f64 = (assign5340_e4099 + assign5340_e4102);
        (assign5340_e4103,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign5340_e4105;
        var_gfacnud_p_rv = 0.0;

        let (assign5350_e4109,) = {
    if (var_guard36 != 0.0) {
        (p.p226,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign5350_e4109;
        var_vsbnud_p_rv = 0.0;

        let (assign5360_e4113,) = {
    if (var_guard36 != 0.0) {
        (p.p227,)
    } else {
        (var_dvsbnud_p,)
    }
};
        var_dvsbnud_p = assign5360_e4113;
        var_dvsbnud_p_rv = 0.0;

        let (assign5370_e4131,) = {
    if (var_guard36 != 0.0) {
        let assign5370_e4119: f64 = (var_ile).powf(p.p230);
        let assign5370_e4120: f64 = (p.p229 * assign5370_e4119);
        let assign5370_e4121: f64 = (p.p228 + assign5370_e4120);
        let assign5370_e4124: f64 = (p.p231 * var_iwe);
        let assign5370_e4125: f64 = (assign5370_e4121 + assign5370_e4124);
        let assign5370_e4128: f64 = (p.p232 * var_iae);
        let assign5370_e4129: f64 = (assign5370_e4125 + assign5370_e4128);
        (assign5370_e4129,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign5370_e4131;
        var_dphib_p_rv = 0.0;

        let (assign5380_e4150,) = {
    if (var_guard36 != 0.0) {
        let assign5380_e4138: f64 = (p.p234 * var_ile);
        let assign5380_e4139: f64 = (1.0 + assign5380_e4138);
        let (assign5380_e4147,) = {
            if (1e-6 > assign5380_e4139) {
                (1e-6,)
            } else {
                let assign5380_e4145: f64 = (p.p234 * var_ile);
                let assign5380_e4146: f64 = (1.0 + assign5380_e4145);
                (assign5380_e4146,)
            }
        };
        let assign5380_e4148: f64 = (p.p233 * assign5380_e4147);
        (assign5380_e4148,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign5380_e4150;
        var_np_p_rv = 0.0;

        let (assign5390_e4154,) = {
    if (var_guard36 != 0.0) {
        (p.p235,)
    } else {
        (var_toxov_p,)
    }
};
        var_toxov_p = assign5390_e4154;
        var_toxov_p_rv = 0.0;

        let (assign5400_e4158,) = {
    if (var_guard36 != 0.0) {
        (p.p236,)
    } else {
        (var_toxovd_p,)
    }
};
        var_toxovd_p = assign5400_e4158;
        var_toxovd_p_rv = 0.0;

        let (assign5410_e4162,) = {
    if (var_guard36 != 0.0) {
        (p.p239,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign5410_e4162;
        var_nov_p_rv = 0.0;

        let (assign5420_e4166,) = {
    if (var_guard36 != 0.0) {
        (p.p240,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign5420_e4166;
        var_novd_p_rv = 0.0;

        *var_aa_slot = var_aa;
        *var_aa_rv_slot = var_aa_rv;
        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_axinr_p_slot = var_axinr_p;
        *var_axinr_p_rv_slot = var_axinr_p_rv;
        *var_bb_slot = var_bb;
        *var_bb_rv_slot = var_bb_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cgovaccg_p_rv_slot = var_cgovaccg_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_cth_p_slot = var_cth_p;
        *var_cth_p_rv_slot = var_cth_p_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_dvfbinr_p_rv_slot = var_dvfbinr_p_rv;
        *var_dvsbnud_p_slot = var_dvsbnud_p;
        *var_dvsbnud_p_rv_slot = var_dvsbnud_p_rv;
        *var_epsrox_p_slot = var_epsrox_p;
        *var_epsrox_p_rv_slot = var_epsrox_p_rv;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovacc_p_rv_slot = var_fcgovacc_p_rv;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcgovaccd_p_rv_slot = var_fcgovaccd_p_rv;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinracc_p_rv_slot = var_fcinracc_p_rv;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_fcinrdep_p_rv_slot = var_fcinrdep_p_rv;
        *var_fnt_p_slot = var_fnt_p;
        *var_fnt_p_rv_slot = var_fnt_p_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_guard34_slot = var_guard34;
        *var_guard34_rv_slot = var_guard34_rv;
        *var_guard35_slot = var_guard35;
        *var_guard35_rv_slot = var_guard35_rv;
        *var_guard36_slot = var_guard36;
        *var_guard36_rv_slot = var_guard36_rv;
        *var_guard37_slot = var_guard37;
        *var_guard37_rv_slot = var_guard37_rv;
        *var_guard38_slot = var_guard38;
        *var_guard38_rv_slot = var_guard38_rv;
        *var_lpcke_slot = var_lpcke;
        *var_lpcke_rv_slot = var_lpcke_rv;
        *var_neff_p_slot = var_neff_p;
        *var_neff_p_rv_slot = var_neff_p_rv;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_neffedge_p_rv_slot = var_neffedge_p_rv;
        *var_nov_p_slot = var_nov_p;
        *var_nov_p_rv_slot = var_nov_p_rv;
        *var_novd_p_slot = var_novd_p;
        *var_novd_p_rv_slot = var_novd_p_rv;
        *var_np_p_slot = var_np_p;
        *var_np_p_rv_slot = var_np_p_rv;
        *var_npcke_slot = var_npcke;
        *var_npcke_rv_slot = var_npcke_rv;
        *var_nsub_slot = var_nsub;
        *var_nsub0e_slot = var_nsub0e;
        *var_nsub0e_rv_slot = var_nsub0e_rv;
        *var_nsub_rv_slot = var_nsub_rv;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_st2vfb_p_slot = var_st2vfb_p;
        *var_st2vfb_p_rv_slot = var_st2vfb_p_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_tox_p_slot = var_tox_p;
        *var_tox_p_rv_slot = var_tox_p_rv;
        *var_toxov_p_slot = var_toxov_p;
        *var_toxov_p_rv_slot = var_toxov_p_rv;
        *var_toxovd_p_slot = var_toxovd_p;
        *var_toxovd_p_rv_slot = var_toxovd_p_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfb_p_rv_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfd_p_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_fbet1e_slot: &mut f64,
        var_fbet1e_rv_slot: &mut f64,
        var_feta_p_slot: &mut f64,
        var_feta_p_rv_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_gpe_rv_slot: &mut f64,
        var_gwe_slot: &mut f64,
        var_gwe_rv_slot: &mut f64,
        var_lp1e_slot: &mut f64,
        var_lp1e_rv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_mue_p_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psceb_p_rv_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_psced_p_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsb_p_rv_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rsg_p_rv_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbet_p_rv_slot: &mut f64,
        var_stcs_p_slot: &mut f64,
        var_stcs_p_rv_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_stmue_p_slot: &mut f64,
        var_stmue_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthecs_p_slot: &mut f64,
        var_stthecs_p_rv_slot: &mut f64,
        var_stthemu_p_slot: &mut f64,
        var_stthemu_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
        var_stxcor_p_slot: &mut f64,
        var_stxcor_p_rv_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thecs_p_rv_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_themu_p_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatb_p_rv_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatg_p_rv_slot: &mut f64,
        var_thesatt_p_slot: &mut f64,
        var_thesatt_p_rv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xcor_p_rv_slot: &mut f64,
    ) {
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfb_p_rv: f64 = *var_cfb_p_rv_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfd_p_rv: f64 = *var_cfd_p_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_fbet1e: f64 = *var_fbet1e_slot;
        let mut var_fbet1e_rv: f64 = *var_fbet1e_rv_slot;
        let mut var_feta_p: f64 = *var_feta_p_slot;
        let mut var_feta_p_rv: f64 = *var_feta_p_rv_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_gpe_rv: f64 = *var_gpe_rv_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
        let mut var_gwe_rv: f64 = *var_gwe_rv_slot;
        let mut var_lp1e: f64 = *var_lp1e_slot;
        let mut var_lp1e_rv: f64 = *var_lp1e_rv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_mue_p_rv: f64 = *var_mue_p_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psceb_p_rv: f64 = *var_psceb_p_rv_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_psced_p_rv: f64 = *var_psced_p_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsb_p_rv: f64 = *var_rsb_p_rv_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rsg_p_rv: f64 = *var_rsg_p_rv_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbet_p_rv: f64 = *var_stbet_p_rv_slot;
        let mut var_stcs_p: f64 = *var_stcs_p_slot;
        let mut var_stcs_p_rv: f64 = *var_stcs_p_rv_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_stmue_p: f64 = *var_stmue_p_slot;
        let mut var_stmue_p_rv: f64 = *var_stmue_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthecs_p: f64 = *var_stthecs_p_slot;
        let mut var_stthecs_p_rv: f64 = *var_stthecs_p_rv_slot;
        let mut var_stthemu_p: f64 = *var_stthemu_p_slot;
        let mut var_stthemu_p_rv: f64 = *var_stthemu_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
        let mut var_stxcor_p: f64 = *var_stxcor_p_slot;
        let mut var_stxcor_p_rv: f64 = *var_stxcor_p_rv_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thecs_p_rv: f64 = *var_thecs_p_rv_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_themu_p_rv: f64 = *var_themu_p_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatb_p_rv: f64 = *var_thesatb_p_rv_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatg_p_rv: f64 = *var_thesatg_p_rv_slot;
        let mut var_thesatt_p: f64 = *var_thesatt_p_slot;
        let mut var_thesatt_p_rv: f64 = *var_thesatt_p_rv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        let (assign5430_e4188,) = {
    if (var_guard36 != 0.0) {
        let assign5430_e4172: f64 = (var_ile).powf(p.p243);
        let assign5430_e4173: f64 = (p.p242 * assign5430_e4172);
        let assign5430_e4174: f64 = (p.p241 + assign5430_e4173);
        let assign5430_e4178: f64 = (p.p244 * var_iwe);
        let assign5430_e4179: f64 = (1.0 + assign5430_e4178);
        let assign5430_e4180: f64 = (assign5430_e4174 * assign5430_e4179);
        let assign5430_e4184: f64 = (p.p245 * var_iae);
        let assign5430_e4185: f64 = (1.0 + assign5430_e4184);
        let assign5430_e4186: f64 = (assign5430_e4180 * assign5430_e4185);
        (assign5430_e4186,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign5430_e4188;
        var_ct_p_rv = 0.0;

        let (assign5440_e4192,) = {
    if (var_guard36 != 0.0) {
        (p.p247,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign5440_e4192;
        var_ctg_p_rv = 0.0;

        let (assign5450_e4196,) = {
    if (var_guard36 != 0.0) {
        (p.p246,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign5450_e4196;
        var_ctb_p_rv = 0.0;

        let (assign5460_e4200,) = {
    if (var_guard36 != 0.0) {
        (p.p248,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign5460_e4200;
        var_stct_p_rv = 0.0;

        let (assign5470_e4214,) = {
    if (var_guard36 != 0.0) {
        let assign5470_e4205: f64 = (var_ile).powf(p.p250);
        let assign5470_e4206: f64 = (p.p249 * assign5470_e4205);
        let assign5470_e4210: f64 = (p.p251 * var_iwe);
        let assign5470_e4211: f64 = (1.0 + assign5470_e4210);
        let assign5470_e4212: f64 = (assign5470_e4206 * assign5470_e4211);
        (assign5470_e4212,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign5470_e4214;
        var_cf_p_rv = 0.0;

        let (assign5480_e4218,) = {
    if (var_guard36 != 0.0) {
        (p.p253,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign5480_e4218;
        var_cfd_p_rv = 0.0;

        let (assign5490_e4222,) = {
    if (var_guard36 != 0.0) {
        (p.p252,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign5490_e4222;
        var_cfb_p_rv = 0.0;

        let (assign5500_e4236,) = {
    if (var_guard36 != 0.0) {
        let assign5500_e4227: f64 = (var_ile).powf(p.p255);
        let assign5500_e4228: f64 = (p.p254 * assign5500_e4227);
        let assign5500_e4232: f64 = (p.p256 * var_iwe);
        let assign5500_e4233: f64 = (1.0 + assign5500_e4232);
        let assign5500_e4234: f64 = (assign5500_e4228 * assign5500_e4233);
        (assign5500_e4234,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign5500_e4236;
        var_psce_p_rv = 0.0;

        let (assign5510_e4240,) = {
    if (var_guard36 != 0.0) {
        (p.p258,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign5510_e4240;
        var_psced_p_rv = 0.0;

        let (assign5520_e4244,) = {
    if (var_guard36 != 0.0) {
        (p.p257,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign5520_e4244;
        var_psceb_p_rv = 0.0;

        let (assign5530_e4254,) = {
    if (var_guard36 != 0.0) {
        let assign5530_e4250: f64 = (p.p261 * var_iwe);
        let assign5530_e4251: f64 = (1.0 + assign5530_e4250);
        let assign5530_e4252: f64 = (p.p260 * assign5530_e4251);
        (assign5530_e4252,)
    } else {
        (var_fbet1e,)
    }
};
        var_fbet1e = assign5530_e4254;
        var_fbet1e_rv = 0.0;

        let (assign5540_e4273,) = {
    if (var_guard36 != 0.0) {
        let assign5540_e4260: f64 = (p.p263 * var_iwe);
        let assign5540_e4261: f64 = (1.0 + assign5540_e4260);
        let (assign5540_e4270,) = {
            if (assign5540_e4261 > 0.001) {
                let assign5540_e4267: f64 = (p.p263 * var_iwe);
                let assign5540_e4268: f64 = (1.0 + assign5540_e4267);
                (assign5540_e4268,)
            } else {
                (0.001,)
            }
        };
        let assign5540_e4271: f64 = (p.p262 * assign5540_e4270);
        (assign5540_e4271,)
    } else {
        (var_lp1e,)
    }
};
        var_lp1e = assign5540_e4273;
        var_lp1e_rv = 0.0;

        let (assign5550_e4305,) = {
    if (var_guard36 != 0.0) {
        let assign5550_e4278: f64 = (var_fbet1e * var_lp1e);
        let assign5550_e4280: f64 = (assign5550_e4278 / var_le);
        let assign5550_e4283: f64 = (-var_le);
        let assign5550_e4285: f64 = (assign5550_e4283 / var_lp1e);
        let assign5550_e4286: f64 = (assign5550_e4285).exp();
        let assign5550_e4287: f64 = (1.0 - assign5550_e4286);
        let assign5550_e4288: f64 = (assign5550_e4280 * assign5550_e4287);
        let assign5550_e4289: f64 = (1.0 + assign5550_e4288);
        let assign5550_e4292: f64 = (p.p264 * p.p265);
        let assign5550_e4294: f64 = (assign5550_e4292 / var_le);
        let assign5550_e4297: f64 = (-var_le);
        let assign5550_e4299: f64 = (assign5550_e4297 / p.p265);
        let assign5550_e4300: f64 = (assign5550_e4299).exp();
        let assign5550_e4301: f64 = (1.0 - assign5550_e4300);
        let assign5550_e4302: f64 = (assign5550_e4294 * assign5550_e4301);
        let assign5550_e4303: f64 = (assign5550_e4289 + assign5550_e4302);
        (assign5550_e4303,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5550_e4305;
        var_gpe_rv = 0.0;

        let (assign5560_e4314,) = {
    if (var_guard36 != 0.0) {
        let (assign5560_e4312,) = {
            if (var_gpe > 1e-15) {
                (var_gpe,)
            } else {
                (1e-15,)
            }
        };
        (assign5560_e4312,)
    } else {
        (var_gpe,)
    }
};
        var_gpe = assign5560_e4314;
        var_gpe_rv = 0.0;

        let (assign5570_e4333,) = {
    if (var_guard36 != 0.0) {
        let assign5570_e4319: f64 = (p.p266 * var_iwe);
        let assign5570_e4320: f64 = (1.0 + assign5570_e4319);
        let assign5570_e4323: f64 = (p.p267 * var_iwe);
        let assign5570_e4327: f64 = (var_we / p.p268);
        let assign5570_e4328: f64 = (1.0 + assign5570_e4327);
        let assign5570_e4329: f64 = (assign5570_e4328).ln();
        let assign5570_e4330: f64 = (assign5570_e4323 * assign5570_e4329);
        let assign5570_e4331: f64 = (assign5570_e4320 + assign5570_e4330);
        (assign5570_e4331,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign5570_e4333;
        var_gwe_rv = 0.0;

        let (assign5580_e4345,) = {
    if (var_guard36 != 0.0) {
        let assign5580_e4337: f64 = (p.p259 * var_we);
        let assign5580_e4340: f64 = (var_gpe * var_le);
        let assign5580_e4341: f64 = (assign5580_e4337 / assign5580_e4340);
        let assign5580_e4343: f64 = (assign5580_e4341 * var_gwe);
        (assign5580_e4343,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign5580_e4345;
        var_betn_p_rv = 0.0;

        let (assign5590_e4361,) = {
    if (var_guard36 != 0.0) {
        let assign5590_e4350: f64 = (p.p270 * var_ile);
        let assign5590_e4351: f64 = (p.p269 + assign5590_e4350);
        let assign5590_e4354: f64 = (p.p271 * var_iwe);
        let assign5590_e4355: f64 = (assign5590_e4351 + assign5590_e4354);
        let assign5590_e4358: f64 = (p.p272 * var_iae);
        let assign5590_e4359: f64 = (assign5590_e4355 + assign5590_e4358);
        (assign5590_e4359,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign5590_e4361;
        var_stbet_p_rv = 0.0;

        let (assign5600_e4371,) = {
    if (var_guard36 != 0.0) {
        let assign5600_e4367: f64 = (p.p274 * var_iwe);
        let assign5600_e4368: f64 = (1.0 + assign5600_e4367);
        let assign5600_e4369: f64 = (p.p273 * assign5600_e4368);
        (assign5600_e4369,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign5600_e4371;
        var_mue_p_rv = 0.0;

        let (assign5610_e4375,) = {
    if (var_guard36 != 0.0) {
        (p.p275,)
    } else {
        (var_stmue_p,)
    }
};
        var_stmue_p = assign5610_e4375;
        var_stmue_p_rv = 0.0;

        let (assign5620_e4379,) = {
    if (var_guard36 != 0.0) {
        (p.p276,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign5620_e4379;
        var_themu_p_rv = 0.0;

        let (assign5630_e4383,) = {
    if (var_guard36 != 0.0) {
        (p.p277,)
    } else {
        (var_stthemu_p,)
    }
};
        var_stthemu_p = assign5630_e4383;
        var_stthemu_p_rv = 0.0;

        let (assign5640_e4405,) = {
    if (var_guard36 != 0.0) {
        let assign5640_e4389: f64 = (var_ile).powf(p.p280);
        let assign5640_e4390: f64 = (p.p279 * assign5640_e4389);
        let assign5640_e4391: f64 = (p.p278 + assign5640_e4390);
        let assign5640_e4395: f64 = (p.p281 * var_iwe);
        let assign5640_e4396: f64 = (1.0 + assign5640_e4395);
        let assign5640_e4397: f64 = (assign5640_e4391 * assign5640_e4396);
        let assign5640_e4401: f64 = (p.p282 * var_iae);
        let assign5640_e4402: f64 = (1.0 + assign5640_e4401);
        let assign5640_e4403: f64 = (assign5640_e4397 * assign5640_e4402);
        (assign5640_e4403,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign5640_e4405;
        var_cs_p_rv = 0.0;

        let (assign5650_e4409,) = {
    if (var_guard36 != 0.0) {
        (p.p283,)
    } else {
        (var_stcs_p,)
    }
};
        var_stcs_p = assign5650_e4409;
        var_stcs_p_rv = 0.0;

        let (assign5660_e4413,) = {
    if (var_guard36 != 0.0) {
        (p.p284,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign5660_e4413;
        var_thecs_p_rv = 0.0;

        let (assign5670_e4417,) = {
    if (var_guard36 != 0.0) {
        (p.p285,)
    } else {
        (var_stthecs_p,)
    }
};
        var_stthecs_p = assign5670_e4417;
        var_stthecs_p_rv = 0.0;

        let (assign5680_e4439,) = {
    if (var_guard36 != 0.0) {
        let assign5680_e4423: f64 = (p.p287 * var_ile);
        let assign5680_e4424: f64 = (1.0 + assign5680_e4423);
        let assign5680_e4425: f64 = (p.p286 * assign5680_e4424);
        let assign5680_e4429: f64 = (p.p288 * var_iwe);
        let assign5680_e4430: f64 = (1.0 + assign5680_e4429);
        let assign5680_e4431: f64 = (assign5680_e4425 * assign5680_e4430);
        let assign5680_e4435: f64 = (p.p289 * var_iae);
        let assign5680_e4436: f64 = (1.0 + assign5680_e4435);
        let assign5680_e4437: f64 = (assign5680_e4431 * assign5680_e4436);
        (assign5680_e4437,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign5680_e4439;
        var_xcor_p_rv = 0.0;

        let (assign5690_e4443,) = {
    if (var_guard36 != 0.0) {
        (p.p290,)
    } else {
        (var_stxcor_p,)
    }
};
        var_stxcor_p = assign5690_e4443;
        var_stxcor_p_rv = 0.0;

        let (assign5700_e4447,) = {
    if (var_guard36 != 0.0) {
        (p.p291,)
    } else {
        (var_feta_p,)
    }
};
        var_feta_p = assign5700_e4447;
        var_feta_p_rv = 0.0;

        let (assign5710_e4459,) = {
    if (var_guard36 != 0.0) {
        let assign5710_e4451: f64 = (p.p292 * var_iwe);
        let assign5710_e4455: f64 = (p.p293 * var_iwe);
        let assign5710_e4456: f64 = (1.0 + assign5710_e4455);
        let assign5710_e4457: f64 = (assign5710_e4451 * assign5710_e4456);
        (assign5710_e4457,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign5710_e4459;
        var_rs_p_rv = 0.0;

        let (assign5720_e4463,) = {
    if (var_guard36 != 0.0) {
        (p.p294,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign5720_e4463;
        var_strs_p_rv = 0.0;

        let (assign5730_e4467,) = {
    if (var_guard36 != 0.0) {
        (p.p295,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign5730_e4467;
        var_rsb_p_rv = 0.0;

        let (assign5740_e4471,) = {
    if (var_guard36 != 0.0) {
        (p.p296,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign5740_e4471;
        var_rsg_p_rv = 0.0;

        let (assign5750_e4497,) = {
    if (var_guard36 != 0.0) {
        let assign5750_e4476: f64 = (p.p298 * var_gwe);
        let assign5750_e4478: f64 = (assign5750_e4476 / var_gpe);
        let assign5750_e4481: f64 = (var_ile).powf(p.p299);
        let assign5750_e4482: f64 = (assign5750_e4478 * assign5750_e4481);
        let assign5750_e4483: f64 = (p.p297 + assign5750_e4482);
        let assign5750_e4487: f64 = (p.p300 * var_iwe);
        let assign5750_e4488: f64 = (1.0 + assign5750_e4487);
        let assign5750_e4489: f64 = (assign5750_e4483 * assign5750_e4488);
        let assign5750_e4493: f64 = (p.p301 * var_iae);
        let assign5750_e4494: f64 = (1.0 + assign5750_e4493);
        let assign5750_e4495: f64 = (assign5750_e4489 * assign5750_e4494);
        (assign5750_e4495,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign5750_e4497;
        var_thesat_p_rv = 0.0;

        let (assign5760_e4513,) = {
    if (var_guard36 != 0.0) {
        let assign5760_e4502: f64 = (p.p303 * var_ile);
        let assign5760_e4503: f64 = (p.p302 + assign5760_e4502);
        let assign5760_e4506: f64 = (p.p304 * var_iwe);
        let assign5760_e4507: f64 = (assign5760_e4503 + assign5760_e4506);
        let assign5760_e4510: f64 = (p.p305 * var_iae);
        let assign5760_e4511: f64 = (assign5760_e4507 + assign5760_e4510);
        (assign5760_e4511,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign5760_e4513;
        var_stthesat_p_rv = 0.0;

        let (assign5770_e4517,) = {
    if (var_guard36 != 0.0) {
        (p.p306,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign5770_e4517;
        var_thesatb_p_rv = 0.0;

        let (assign5780_e4521,) = {
    if (var_guard36 != 0.0) {
        (p.p307,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign5780_e4521;
        var_thesatg_p_rv = 0.0;

        let (assign5790_e4525,) = {
    if (var_guard36 != 0.0) {
        (p.p308,)
    } else {
        (var_thesatt_p,)
    }
};
        var_thesatt_p = assign5790_e4525;
        var_thesatt_p_rv = 0.0;

        let (assign5800_e4535,) = {
    if (var_guard36 != 0.0) {
        let assign5800_e4531: f64 = (p.p310 * var_ile);
        let assign5800_e4532: f64 = (1.0 + assign5800_e4531);
        let assign5800_e4533: f64 = (p.p309 / assign5800_e4532);
        (assign5800_e4533,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign5800_e4535;
        var_ax_p_rv = 0.0;

        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfb_p_rv_slot = var_cfb_p_rv;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfd_p_rv_slot = var_cfd_p_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_fbet1e_slot = var_fbet1e;
        *var_fbet1e_rv_slot = var_fbet1e_rv;
        *var_feta_p_slot = var_feta_p;
        *var_feta_p_rv_slot = var_feta_p_rv;
        *var_gpe_slot = var_gpe;
        *var_gpe_rv_slot = var_gpe_rv;
        *var_gwe_slot = var_gwe;
        *var_gwe_rv_slot = var_gwe_rv;
        *var_lp1e_slot = var_lp1e;
        *var_lp1e_rv_slot = var_lp1e_rv;
        *var_mue_p_slot = var_mue_p;
        *var_mue_p_rv_slot = var_mue_p_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_psceb_p_slot = var_psceb_p;
        *var_psceb_p_rv_slot = var_psceb_p_rv;
        *var_psced_p_slot = var_psced_p;
        *var_psced_p_rv_slot = var_psced_p_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsb_p_rv_slot = var_rsb_p_rv;
        *var_rsg_p_slot = var_rsg_p;
        *var_rsg_p_rv_slot = var_rsg_p_rv;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbet_p_rv_slot = var_stbet_p_rv;
        *var_stcs_p_slot = var_stcs_p;
        *var_stcs_p_rv_slot = var_stcs_p_rv;
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_stmue_p_slot = var_stmue_p;
        *var_stmue_p_rv_slot = var_stmue_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthecs_p_slot = var_stthecs_p;
        *var_stthecs_p_rv_slot = var_stthecs_p_rv;
        *var_stthemu_p_slot = var_stthemu_p;
        *var_stthemu_p_rv_slot = var_stthemu_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
        *var_stxcor_p_slot = var_stxcor_p;
        *var_stxcor_p_rv_slot = var_stxcor_p_rv;
        *var_thecs_p_slot = var_thecs_p;
        *var_thecs_p_rv_slot = var_thecs_p_rv;
        *var_themu_p_slot = var_themu_p;
        *var_themu_p_rv_slot = var_themu_p_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatb_p_rv_slot = var_thesatb_p_rv;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatg_p_rv_slot = var_thesatg_p_rv;
        *var_thesatt_p_slot = var_thesatt_p;
        *var_thesatt_p_rv_slot = var_thesatt_p_rv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_wecv: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
        var_a2_p_slot: &mut f64,
        var_a2_p_rv_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_a4_p_rv_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp2_p_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_bgidl_p_slot: &mut f64,
        var_bgidl_p_rv_slot: &mut f64,
        var_bgidld_p_slot: &mut f64,
        var_bgidld_p_rv_slot: &mut f64,
        var_cgidl_p_slot: &mut f64,
        var_cgidl_p_rv_slot: &mut f64,
        var_cgidld_p_slot: &mut f64,
        var_cgidld_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_chib_p_slot: &mut f64,
        var_chib_p_rv_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_gc2_p_slot: &mut f64,
        var_gc2_p_rv_slot: &mut f64,
        var_gc2ov_p_slot: &mut f64,
        var_gc2ov_p_rv_slot: &mut f64,
        var_gc2ovd_p_slot: &mut f64,
        var_gc2ovd_p_rv_slot: &mut f64,
        var_gc3_p_slot: &mut f64,
        var_gc3_p_rv_slot: &mut f64,
        var_gc3ov_p_slot: &mut f64,
        var_gc3ov_p_rv_slot: &mut f64,
        var_gc3ovd_p_slot: &mut f64,
        var_gc3ovd_p_rv_slot: &mut f64,
        var_gco_p_slot: &mut f64,
        var_gco_p_rv_slot: &mut f64,
        var_guard39_slot: &mut f64,
        var_guard39_rv_slot: &mut f64,
        var_guard40_slot: &mut f64,
        var_guard40_rv_slot: &mut f64,
        var_guard41_slot: &mut f64,
        var_guard41_rv_slot: &mut f64,
        var_guard42_slot: &mut f64,
        var_guard42_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_imaxii_p_slot: &mut f64,
        var_imaxii_p_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_tmpx_rv_slot: &mut f64,
        var_vp_p_slot: &mut f64,
        var_vp_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
        let mut var_a2_p: f64 = *var_a2_p_slot;
        let mut var_a2_p_rv: f64 = *var_a2_p_rv_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_a4_p_rv: f64 = *var_a4_p_rv_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp2_p_rv: f64 = *var_alp2_p_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_bgidl_p: f64 = *var_bgidl_p_slot;
        let mut var_bgidl_p_rv: f64 = *var_bgidl_p_rv_slot;
        let mut var_bgidld_p: f64 = *var_bgidld_p_slot;
        let mut var_bgidld_p_rv: f64 = *var_bgidld_p_rv_slot;
        let mut var_cgidl_p: f64 = *var_cgidl_p_slot;
        let mut var_cgidl_p_rv: f64 = *var_cgidl_p_rv_slot;
        let mut var_cgidld_p: f64 = *var_cgidld_p_slot;
        let mut var_cgidld_p_rv: f64 = *var_cgidld_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_chib_p: f64 = *var_chib_p_slot;
        let mut var_chib_p_rv: f64 = *var_chib_p_rv_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_gc2_p: f64 = *var_gc2_p_slot;
        let mut var_gc2_p_rv: f64 = *var_gc2_p_rv_slot;
        let mut var_gc2ov_p: f64 = *var_gc2ov_p_slot;
        let mut var_gc2ov_p_rv: f64 = *var_gc2ov_p_rv_slot;
        let mut var_gc2ovd_p: f64 = *var_gc2ovd_p_slot;
        let mut var_gc2ovd_p_rv: f64 = *var_gc2ovd_p_rv_slot;
        let mut var_gc3_p: f64 = *var_gc3_p_slot;
        let mut var_gc3_p_rv: f64 = *var_gc3_p_rv_slot;
        let mut var_gc3ov_p: f64 = *var_gc3ov_p_slot;
        let mut var_gc3ov_p_rv: f64 = *var_gc3ov_p_rv_slot;
        let mut var_gc3ovd_p: f64 = *var_gc3ovd_p_slot;
        let mut var_gc3ovd_p_rv: f64 = *var_gc3ovd_p_rv_slot;
        let mut var_gco_p: f64 = *var_gco_p_slot;
        let mut var_gco_p_rv: f64 = *var_gco_p_rv_slot;
        let mut var_guard39: f64 = *var_guard39_slot;
        let mut var_guard39_rv: f64 = *var_guard39_rv_slot;
        let mut var_guard40: f64 = *var_guard40_slot;
        let mut var_guard40_rv: f64 = *var_guard40_rv_slot;
        let mut var_guard41: f64 = *var_guard41_slot;
        let mut var_guard41_rv: f64 = *var_guard41_rv_slot;
        let mut var_guard42: f64 = *var_guard42_slot;
        let mut var_guard42_rv: f64 = *var_guard42_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_imaxii_p: f64 = *var_imaxii_p_slot;
        let mut var_imaxii_p_rv: f64 = *var_imaxii_p_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_tmpx_rv: f64 = *var_tmpx_rv_slot;
        let mut var_vp_p: f64 = *var_vp_p_slot;
        let mut var_vp_p_rv: f64 = *var_vp_p_rv_slot;

        let (assign5810_e4549,) = {
    if (var_guard36 != 0.0) {
        let assign5810_e4540: f64 = (var_ile).powf(p.p312);
        let assign5810_e4541: f64 = (p.p311 * assign5810_e4540);
        let assign5810_e4545: f64 = (p.p313 * var_iwe);
        let assign5810_e4546: f64 = (1.0 + assign5810_e4545);
        let assign5810_e4547: f64 = (assign5810_e4541 * assign5810_e4546);
        (assign5810_e4547,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign5810_e4549;
        var_alp_p_rv = 0.0;

        let (assign5820_e4555,) = {
    if (var_guard36 != 0.0) {
        let assign5820_e4553: f64 = (var_ile).powf(p.p315);
        (assign5820_e4553,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign5820_e4555;
        var_tmpx_rv = 0.0;

        let (assign5830_e4575,) = {
    if (var_guard36 != 0.0) {
        let assign5830_e4559: f64 = (p.p314 * var_tmpx);
        let assign5830_e4563: f64 = (p.p317 * var_iwe);
        let assign5830_e4564: f64 = (1.0 + assign5830_e4563);
        let assign5830_e4565: f64 = (assign5830_e4559 * assign5830_e4564);
        let assign5830_e4569: f64 = (p.p316 * var_ile);
        let assign5830_e4571: f64 = (assign5830_e4569 * var_tmpx);
        let assign5830_e4572: f64 = (1.0 + assign5830_e4571);
        let assign5830_e4573: f64 = (assign5830_e4565 / assign5830_e4572);
        (assign5830_e4573,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign5830_e4575;
        var_alp1_p_rv = 0.0;

        let (assign5840_e4581,) = {
    if (var_guard36 != 0.0) {
        let assign5840_e4579: f64 = (var_ile).powf(p.p319);
        (assign5840_e4579,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign5840_e4581;
        var_tmpx_rv = 0.0;

        let (assign5850_e4601,) = {
    if (var_guard36 != 0.0) {
        let assign5850_e4585: f64 = (p.p318 * var_tmpx);
        let assign5850_e4589: f64 = (p.p321 * var_iwe);
        let assign5850_e4590: f64 = (1.0 + assign5850_e4589);
        let assign5850_e4591: f64 = (assign5850_e4585 * assign5850_e4590);
        let assign5850_e4595: f64 = (p.p320 * var_ile);
        let assign5850_e4597: f64 = (assign5850_e4595 * var_tmpx);
        let assign5850_e4598: f64 = (1.0 + assign5850_e4597);
        let assign5850_e4599: f64 = (assign5850_e4591 / assign5850_e4598);
        (assign5850_e4599,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign5850_e4601;
        var_alp2_p_rv = 0.0;

        let (assign5860_e4605,) = {
    if (var_guard36 != 0.0) {
        (p.p322,)
    } else {
        (var_vp_p,)
    }
};
        var_vp_p = assign5860_e4605;
        var_vp_p_rv = 0.0;

        let (assign5870_e4621,) = {
    if (var_guard36 != 0.0) {
        let assign5870_e4611: f64 = (p.p324 * var_ile);
        let assign5870_e4612: f64 = (1.0 + assign5870_e4611);
        let assign5870_e4613: f64 = (p.p323 * assign5870_e4612);
        let assign5870_e4617: f64 = (p.p325 * var_iwe);
        let assign5870_e4618: f64 = (1.0 + assign5870_e4617);
        let assign5870_e4619: f64 = (assign5870_e4613 * assign5870_e4618);
        (assign5870_e4619,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign5870_e4621;
        var_a1_p_rv = 0.0;

        let (assign5880_e4625,) = {
    if (var_guard36 != 0.0) {
        (p.p326,)
    } else {
        (var_a2_p,)
    }
};
        var_a2_p = assign5880_e4625;
        var_a2_p_rv = 0.0;

        let (assign5890_e4629,) = {
    if (var_guard36 != 0.0) {
        (p.p327,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign5890_e4629;
        var_sta2_p_rv = 0.0;

        let (assign5900_e4645,) = {
    if (var_guard36 != 0.0) {
        let assign5900_e4635: f64 = (p.p329 * var_ile);
        let assign5900_e4636: f64 = (1.0 + assign5900_e4635);
        let assign5900_e4637: f64 = (p.p328 * assign5900_e4636);
        let assign5900_e4641: f64 = (p.p330 * var_iwe);
        let assign5900_e4642: f64 = (1.0 + assign5900_e4641);
        let assign5900_e4643: f64 = (assign5900_e4637 * assign5900_e4642);
        (assign5900_e4643,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign5900_e4645;
        var_a3_p_rv = 0.0;

        let (assign5910_e4661,) = {
    if (var_guard36 != 0.0) {
        let assign5910_e4651: f64 = (p.p332 * var_ile);
        let assign5910_e4652: f64 = (1.0 + assign5910_e4651);
        let assign5910_e4653: f64 = (p.p331 * assign5910_e4652);
        let assign5910_e4657: f64 = (p.p333 * var_iwe);
        let assign5910_e4658: f64 = (1.0 + assign5910_e4657);
        let assign5910_e4659: f64 = (assign5910_e4653 * assign5910_e4658);
        (assign5910_e4659,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign5910_e4661;
        var_a4_p_rv = 0.0;

        let (assign5920_e4665,) = {
    if (var_guard36 != 0.0) {
        (p.p334,)
    } else {
        (var_imaxii_p,)
    }
};
        var_imaxii_p = assign5920_e4665;
        var_imaxii_p_rv = 0.0;

        let (assign5930_e4669,) = {
    if (var_guard36 != 0.0) {
        (p.p335,)
    } else {
        (var_gco_p,)
    }
};
        var_gco_p = assign5930_e4669;
        var_gco_p_rv = 0.0;

        let (assign5940_e4675,) = {
    if (var_guard36 != 0.0) {
        let assign5940_e4673: f64 = (p.p336 / var_iae);
        (assign5940_e4673,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign5940_e4675;
        var_iginv_p_rv = 0.0;

        let (assign5950_e4685,) = {
    if (var_guard36 != 0.0) {
        let assign5950_e4679: f64 = (p.p337 * p.p237);
        let assign5950_e4682: f64 = (1e-6 * var_iwe);
        let assign5950_e4683: f64 = (assign5950_e4679 / assign5950_e4682);
        (assign5950_e4683,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign5950_e4685;
        var_igov_p_rv = 0.0;

        let (assign5960_e4695,) = {
    if (var_guard36 != 0.0) {
        let assign5960_e4689: f64 = (p.p338 * p.p238);
        let assign5960_e4692: f64 = (1e-6 * var_iwe);
        let assign5960_e4693: f64 = (assign5960_e4689 / assign5960_e4692);
        (assign5960_e4693,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign5960_e4695;
        var_igovd_p_rv = 0.0;

        let (assign5970_e4699,) = {
    if (var_guard36 != 0.0) {
        (p.p339,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign5970_e4699;
        var_stig_p_rv = 0.0;

        let (assign5980_e4703,) = {
    if (var_guard36 != 0.0) {
        (p.p340,)
    } else {
        (var_gc2_p,)
    }
};
        var_gc2_p = assign5980_e4703;
        var_gc2_p_rv = 0.0;

        let (assign5990_e4707,) = {
    if (var_guard36 != 0.0) {
        (p.p341,)
    } else {
        (var_gc3_p,)
    }
};
        var_gc3_p = assign5990_e4707;
        var_gc3_p_rv = 0.0;

        let (assign6000_e4711,) = {
    if (var_guard36 != 0.0) {
        (p.p340,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6000_e4711;
        var_gc2ov_p_rv = 0.0;

        let assign6010_e4713: f64 = if param_given[342] { 1.0 } else { 0.0 };
        let assign6010_e4715: f64 = if assign6010_e4713 == 1.0 { 1.0 } else { 0.0 };
        var_guard39 = assign6010_e4715;
        var_guard39_rv = 0.0;

        let (assign6020_e4721,) = {
    if ((var_guard36 != 0.0) && (var_guard39 != 0.0)) {
        (p.p342,)
    } else {
        (var_gc2ov_p,)
    }
};
        var_gc2ov_p = assign6020_e4721;
        var_gc2ov_p_rv = 0.0;

        let (assign6030_e4725,) = {
    if (var_guard36 != 0.0) {
        (p.p341,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6030_e4725;
        var_gc3ov_p_rv = 0.0;

        let assign6040_e4727: f64 = if param_given[343] { 1.0 } else { 0.0 };
        let assign6040_e4729: f64 = if assign6040_e4727 == 1.0 { 1.0 } else { 0.0 };
        var_guard40 = assign6040_e4729;
        var_guard40_rv = 0.0;

        let (assign6050_e4735,) = {
    if ((var_guard36 != 0.0) && (var_guard40 != 0.0)) {
        (p.p343,)
    } else {
        (var_gc3ov_p,)
    }
};
        var_gc3ov_p = assign6050_e4735;
        var_gc3ov_p_rv = 0.0;

        let (assign6060_e4739,) = {
    if (var_guard36 != 0.0) {
        (var_gc2ov_p,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6060_e4739;
        var_gc2ovd_p_rv = 0.0;

        let assign6070_e4741: f64 = if param_given[344] { 1.0 } else { 0.0 };
        let assign6070_e4743: f64 = if assign6070_e4741 == 1.0 { 1.0 } else { 0.0 };
        var_guard41 = assign6070_e4743;
        var_guard41_rv = 0.0;

        let (assign6080_e4749,) = {
    if ((var_guard36 != 0.0) && (var_guard41 != 0.0)) {
        (p.p344,)
    } else {
        (var_gc2ovd_p,)
    }
};
        var_gc2ovd_p = assign6080_e4749;
        var_gc2ovd_p_rv = 0.0;

        let (assign6090_e4753,) = {
    if (var_guard36 != 0.0) {
        (var_gc3ov_p,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6090_e4753;
        var_gc3ovd_p_rv = 0.0;

        let assign6100_e4755: f64 = if param_given[345] { 1.0 } else { 0.0 };
        let assign6100_e4757: f64 = if assign6100_e4755 == 1.0 { 1.0 } else { 0.0 };
        var_guard42 = assign6100_e4757;
        var_guard42_rv = 0.0;

        let (assign6110_e4763,) = {
    if ((var_guard36 != 0.0) && (var_guard42 != 0.0)) {
        (p.p345,)
    } else {
        (var_gc3ovd_p,)
    }
};
        var_gc3ovd_p = assign6110_e4763;
        var_gc3ovd_p_rv = 0.0;

        let (assign6120_e4767,) = {
    if (var_guard36 != 0.0) {
        (p.p346,)
    } else {
        (var_chib_p,)
    }
};
        var_chib_p = assign6120_e4767;
        var_chib_p_rv = 0.0;

        let (assign6130_e4777,) = {
    if (var_guard36 != 0.0) {
        let assign6130_e4771: f64 = (p.p347 * p.p237);
        let assign6130_e4774: f64 = (1e-6 * var_iwe);
        let assign6130_e4775: f64 = (assign6130_e4771 / assign6130_e4774);
        (assign6130_e4775,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign6130_e4777;
        var_agidl_p_rv = 0.0;

        let (assign6140_e4787,) = {
    if (var_guard36 != 0.0) {
        let assign6140_e4781: f64 = (p.p348 * p.p238);
        let assign6140_e4784: f64 = (1e-6 * var_iwe);
        let assign6140_e4785: f64 = (assign6140_e4781 / assign6140_e4784);
        (assign6140_e4785,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign6140_e4787;
        var_agidld_p_rv = 0.0;

        let (assign6150_e4791,) = {
    if (var_guard36 != 0.0) {
        (p.p349,)
    } else {
        (var_bgidl_p,)
    }
};
        var_bgidl_p = assign6150_e4791;
        var_bgidl_p_rv = 0.0;

        let (assign6160_e4795,) = {
    if (var_guard36 != 0.0) {
        (p.p350,)
    } else {
        (var_bgidld_p,)
    }
};
        var_bgidld_p = assign6160_e4795;
        var_bgidld_p_rv = 0.0;

        let (assign6170_e4799,) = {
    if (var_guard36 != 0.0) {
        (p.p351,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign6170_e4799;
        var_stbgidl_p_rv = 0.0;

        let (assign6180_e4803,) = {
    if (var_guard36 != 0.0) {
        (p.p352,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign6180_e4803;
        var_stbgidld_p_rv = 0.0;

        let (assign6190_e4807,) = {
    if (var_guard36 != 0.0) {
        (p.p353,)
    } else {
        (var_cgidl_p,)
    }
};
        var_cgidl_p = assign6190_e4807;
        var_cgidl_p_rv = 0.0;

        let (assign6200_e4811,) = {
    if (var_guard36 != 0.0) {
        (p.p354,)
    } else {
        (var_cgidld_p,)
    }
};
        var_cgidld_p = assign6200_e4811;
        var_cgidld_p_rv = 0.0;

        let (assign6210_e4823,) = {
    if (var_guard36 != 0.0) {
        let assign6210_e4815: f64 = (8.8541878176e-12 * p.p210);
        let assign6210_e4817: f64 = (assign6210_e4815 * var_wecv);
        let assign6210_e4819: f64 = (assign6210_e4817 * var_lecv);
        let assign6210_e4821: f64 = (assign6210_e4819 / p.p209);
        (assign6210_e4821,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign6210_e4823;
        var_cox_p_rv = 0.0;

        let (assign6220_e4835,) = {
    if (var_guard36 != 0.0) {
        let assign6220_e4827: f64 = (8.8541878176e-12 * p.p210);
        let assign6220_e4829: f64 = (assign6220_e4827 * var_wecv);
        let assign6220_e4831: f64 = (assign6220_e4829 * p.p237);
        let assign6220_e4833: f64 = (assign6220_e4831 / p.p235);
        (assign6220_e4833,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign6220_e4835;
        var_cgov_p_rv = 0.0;

        let (assign6230_e4847,) = {
    if (var_guard36 != 0.0) {
        let assign6230_e4839: f64 = (8.8541878176e-12 * p.p210);
        let assign6230_e4841: f64 = (assign6230_e4839 * var_wecv);
        let assign6230_e4843: f64 = (assign6230_e4841 * p.p238);
        let assign6230_e4845: f64 = (assign6230_e4843 / p.p236);
        (assign6230_e4845,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign6230_e4847;
        var_cgovd_p_rv = 0.0;

        let (assign6240_e4865,) = {
    if (var_guard36 != 0.0) {
        let assign6240_e4853: f64 = (var_ile).powf(p.p357);
        let assign6240_e4854: f64 = (p.p356 * assign6240_e4853);
        let assign6240_e4855: f64 = (p.p355 + assign6240_e4854);
        let assign6240_e4858: f64 = (p.p358 * var_iwe);
        let assign6240_e4859: f64 = (assign6240_e4855 + assign6240_e4858);
        let assign6240_e4862: f64 = (p.p359 * var_iae);
        let assign6240_e4863: f64 = (assign6240_e4859 + assign6240_e4862);
        (assign6240_e4863,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign6240_e4865;
        var_delvtac_p_rv = 0.0;

        let (assign6250_e4881,) = {
    if (var_guard36 != 0.0) {
        let assign6250_e4870: f64 = (p.p361 * var_ile);
        let assign6250_e4871: f64 = (p.p360 + assign6250_e4870);
        let assign6250_e4874: f64 = (p.p362 * var_iwe);
        let assign6250_e4875: f64 = (assign6250_e4871 + assign6250_e4874);
        let assign6250_e4878: f64 = (p.p363 * var_iae);
        let assign6250_e4879: f64 = (assign6250_e4875 + assign6250_e4878);
        (assign6250_e4879,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign6250_e4881;
        var_facneffac_p_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
        *var_a2_p_slot = var_a2_p;
        *var_a2_p_rv_slot = var_a2_p_rv;
        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_a4_p_slot = var_a4_p;
        *var_a4_p_rv_slot = var_a4_p_rv;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp2_p_rv_slot = var_alp2_p_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_bgidl_p_slot = var_bgidl_p;
        *var_bgidl_p_rv_slot = var_bgidl_p_rv;
        *var_bgidld_p_slot = var_bgidld_p;
        *var_bgidld_p_rv_slot = var_bgidld_p_rv;
        *var_cgidl_p_slot = var_cgidl_p;
        *var_cgidl_p_rv_slot = var_cgidl_p_rv;
        *var_cgidld_p_slot = var_cgidld_p;
        *var_cgidld_p_rv_slot = var_cgidld_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_chib_p_slot = var_chib_p;
        *var_chib_p_rv_slot = var_chib_p_rv;
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_gc2_p_slot = var_gc2_p;
        *var_gc2_p_rv_slot = var_gc2_p_rv;
        *var_gc2ov_p_slot = var_gc2ov_p;
        *var_gc2ov_p_rv_slot = var_gc2ov_p_rv;
        *var_gc2ovd_p_slot = var_gc2ovd_p;
        *var_gc2ovd_p_rv_slot = var_gc2ovd_p_rv;
        *var_gc3_p_slot = var_gc3_p;
        *var_gc3_p_rv_slot = var_gc3_p_rv;
        *var_gc3ov_p_slot = var_gc3ov_p;
        *var_gc3ov_p_rv_slot = var_gc3ov_p_rv;
        *var_gc3ovd_p_slot = var_gc3ovd_p;
        *var_gc3ovd_p_rv_slot = var_gc3ovd_p_rv;
        *var_gco_p_slot = var_gco_p;
        *var_gco_p_rv_slot = var_gco_p_rv;
        *var_guard39_slot = var_guard39;
        *var_guard39_rv_slot = var_guard39_rv;
        *var_guard40_slot = var_guard40;
        *var_guard40_rv_slot = var_guard40_rv;
        *var_guard41_slot = var_guard41;
        *var_guard41_rv_slot = var_guard41_rv;
        *var_guard42_slot = var_guard42;
        *var_guard42_rv_slot = var_guard42_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_imaxii_p_slot = var_imaxii_p;
        *var_imaxii_p_rv_slot = var_imaxii_p_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_tmpx_slot = var_tmpx;
        *var_tmpx_rv_slot = var_tmpx_rv;
        *var_vp_p_slot = var_vp_p;
        *var_vp_p_rv_slot = var_vp_p_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_gpe: f64,
        var_guard36: f64,
        var_gwe: f64,
        var_iae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_axacl_i_slot: &mut f64,
        var_axacl_i_rv_slot: &mut f64,
        var_axaco_i_slot: &mut f64,
        var_axaco_i_rv_slot: &mut f64,
        var_axinr_p_slot: &mut f64,
        var_axinr_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgovaccg_p_slot: &mut f64,
        var_cgovaccg_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_dvfbinr_p_slot: &mut f64,
        var_dvfbinr_p_rv_slot: &mut f64,
        var_fcgovacc_p_slot: &mut f64,
        var_fcgovacc_p_rv_slot: &mut f64,
        var_fcgovaccd_p_slot: &mut f64,
        var_fcgovaccd_p_rv_slot: &mut f64,
        var_fcinracc_p_slot: &mut f64,
        var_fcinracc_p_rv_slot: &mut f64,
        var_fcinrdep_p_slot: &mut f64,
        var_fcinrdep_p_rv_slot: &mut f64,
        var_fnt_p_slot: &mut f64,
        var_fnt_p_rv_slot: &mut f64,
        var_gpe_edge_slot: &mut f64,
        var_gpe_edge_rv_slot: &mut f64,
        var_guard43_slot: &mut f64,
        var_guard43_rv_slot: &mut f64,
        var_guard44_slot: &mut f64,
        var_guard44_rv_slot: &mut f64,
        var_guard45_slot: &mut f64,
        var_guard45_rv_slot: &mut f64,
        var_guard46_slot: &mut f64,
        var_guard46_rv_slot: &mut f64,
        var_guard47_slot: &mut f64,
        var_guard47_rv_slot: &mut f64,
        var_guard48_slot: &mut f64,
        var_guard48_rv_slot: &mut f64,
        var_guard49_slot: &mut f64,
        var_guard49_rv_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_neffedge_p_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesatacl_i_rv_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclexp_i_rv_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataclw_i_rv_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesataco_i_rv_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
        var_thesatacw_i_rv_slot: &mut f64,
        var_tmpx_slot: &mut f64,
        var_tmpx_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_we_edge_slot: &mut f64,
        var_we_edge_rv_slot: &mut f64,
    ) {
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_axacl_i: f64 = *var_axacl_i_slot;
        let mut var_axacl_i_rv: f64 = *var_axacl_i_rv_slot;
        let mut var_axaco_i: f64 = *var_axaco_i_slot;
        let mut var_axaco_i_rv: f64 = *var_axaco_i_rv_slot;
        let mut var_axinr_p: f64 = *var_axinr_p_slot;
        let mut var_axinr_p_rv: f64 = *var_axinr_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgovaccg_p: f64 = *var_cgovaccg_p_slot;
        let mut var_cgovaccg_p_rv: f64 = *var_cgovaccg_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_dvfbinr_p: f64 = *var_dvfbinr_p_slot;
        let mut var_dvfbinr_p_rv: f64 = *var_dvfbinr_p_rv_slot;
        let mut var_fcgovacc_p: f64 = *var_fcgovacc_p_slot;
        let mut var_fcgovacc_p_rv: f64 = *var_fcgovacc_p_rv_slot;
        let mut var_fcgovaccd_p: f64 = *var_fcgovaccd_p_slot;
        let mut var_fcgovaccd_p_rv: f64 = *var_fcgovaccd_p_rv_slot;
        let mut var_fcinracc_p: f64 = *var_fcinracc_p_slot;
        let mut var_fcinracc_p_rv: f64 = *var_fcinracc_p_rv_slot;
        let mut var_fcinrdep_p: f64 = *var_fcinrdep_p_slot;
        let mut var_fcinrdep_p_rv: f64 = *var_fcinrdep_p_rv_slot;
        let mut var_fnt_p: f64 = *var_fnt_p_slot;
        let mut var_fnt_p_rv: f64 = *var_fnt_p_rv_slot;
        let mut var_gpe_edge: f64 = *var_gpe_edge_slot;
        let mut var_gpe_edge_rv: f64 = *var_gpe_edge_rv_slot;
        let mut var_guard43: f64 = *var_guard43_slot;
        let mut var_guard43_rv: f64 = *var_guard43_rv_slot;
        let mut var_guard44: f64 = *var_guard44_slot;
        let mut var_guard44_rv: f64 = *var_guard44_rv_slot;
        let mut var_guard45: f64 = *var_guard45_slot;
        let mut var_guard45_rv: f64 = *var_guard45_rv_slot;
        let mut var_guard46: f64 = *var_guard46_slot;
        let mut var_guard46_rv: f64 = *var_guard46_rv_slot;
        let mut var_guard47: f64 = *var_guard47_slot;
        let mut var_guard47_rv: f64 = *var_guard47_rv_slot;
        let mut var_guard48: f64 = *var_guard48_slot;
        let mut var_guard48_rv: f64 = *var_guard48_rv_slot;
        let mut var_guard49: f64 = *var_guard49_slot;
        let mut var_guard49_rv: f64 = *var_guard49_rv_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_neffedge_p_rv: f64 = *var_neffedge_p_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesatacl_i_rv: f64 = *var_thesatacl_i_rv_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclexp_i_rv: f64 = *var_thesataclexp_i_rv_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataclw_i_rv: f64 = *var_thesataclw_i_rv_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesataco_i_rv: f64 = *var_thesataco_i_rv_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;
        let mut var_thesatacw_i_rv: f64 = *var_thesatacw_i_rv_slot;
        let mut var_tmpx: f64 = *var_tmpx_slot;
        let mut var_tmpx_rv: f64 = *var_tmpx_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;
        let mut var_we_edge_rv: f64 = *var_we_edge_rv_slot;

        let (assign6260_e4885,) = {
    if (var_guard36 != 0.0) {
        (p.p297,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6260_e4885;
        var_thesataco_i_rv = 0.0;

        let assign6270_e4887: f64 = if param_given[364] { 1.0 } else { 0.0 };
        let assign6270_e4889: f64 = if assign6270_e4887 == 1.0 { 1.0 } else { 0.0 };
        var_guard43 = assign6270_e4889;
        var_guard43_rv = 0.0;

        let (assign6280_e4895,) = {
    if ((var_guard36 != 0.0) && (var_guard43 != 0.0)) {
        (p.p364,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign6280_e4895;
        var_thesataco_i_rv = 0.0;

        let (assign6290_e4899,) = {
    if (var_guard36 != 0.0) {
        (p.p298,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6290_e4899;
        var_thesatacl_i_rv = 0.0;

        let assign6300_e4901: f64 = if param_given[365] { 1.0 } else { 0.0 };
        let assign6300_e4903: f64 = if assign6300_e4901 == 1.0 { 1.0 } else { 0.0 };
        var_guard44 = assign6300_e4903;
        var_guard44_rv = 0.0;

        let (assign6310_e4909,) = {
    if ((var_guard36 != 0.0) && (var_guard44 != 0.0)) {
        (p.p365,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign6310_e4909;
        var_thesatacl_i_rv = 0.0;

        let (assign6320_e4913,) = {
    if (var_guard36 != 0.0) {
        (p.p299,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6320_e4913;
        var_thesataclexp_i_rv = 0.0;

        let assign6330_e4915: f64 = if param_given[366] { 1.0 } else { 0.0 };
        let assign6330_e4917: f64 = if assign6330_e4915 == 1.0 { 1.0 } else { 0.0 };
        var_guard45 = assign6330_e4917;
        var_guard45_rv = 0.0;

        let (assign6340_e4923,) = {
    if ((var_guard36 != 0.0) && (var_guard45 != 0.0)) {
        (p.p366,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign6340_e4923;
        var_thesataclexp_i_rv = 0.0;

        let (assign6350_e4927,) = {
    if (var_guard36 != 0.0) {
        (p.p300,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6350_e4927;
        var_thesatacw_i_rv = 0.0;

        let assign6360_e4929: f64 = if param_given[367] { 1.0 } else { 0.0 };
        let assign6360_e4931: f64 = if assign6360_e4929 == 1.0 { 1.0 } else { 0.0 };
        var_guard46 = assign6360_e4931;
        var_guard46_rv = 0.0;

        let (assign6370_e4937,) = {
    if ((var_guard36 != 0.0) && (var_guard46 != 0.0)) {
        (p.p367,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign6370_e4937;
        var_thesatacw_i_rv = 0.0;

        let (assign6380_e4941,) = {
    if (var_guard36 != 0.0) {
        (p.p301,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6380_e4941;
        var_thesataclw_i_rv = 0.0;

        let assign6390_e4943: f64 = if param_given[368] { 1.0 } else { 0.0 };
        let assign6390_e4945: f64 = if assign6390_e4943 == 1.0 { 1.0 } else { 0.0 };
        var_guard47 = assign6390_e4945;
        var_guard47_rv = 0.0;

        let (assign6400_e4951,) = {
    if ((var_guard36 != 0.0) && (var_guard47 != 0.0)) {
        (p.p368,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign6400_e4951;
        var_thesataclw_i_rv = 0.0;

        let (assign6410_e4977,) = {
    if (var_guard36 != 0.0) {
        let assign6410_e4956: f64 = (var_thesatacl_i * var_gwe);
        let assign6410_e4958: f64 = (assign6410_e4956 / var_gpe);
        let assign6410_e4961: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign6410_e4962: f64 = (assign6410_e4958 * assign6410_e4961);
        let assign6410_e4963: f64 = (var_thesataco_i + assign6410_e4962);
        let assign6410_e4967: f64 = (var_thesatacw_i * var_iwe);
        let assign6410_e4968: f64 = (1.0 + assign6410_e4967);
        let assign6410_e4969: f64 = (assign6410_e4963 * assign6410_e4968);
        let assign6410_e4973: f64 = (var_thesataclw_i * var_iae);
        let assign6410_e4974: f64 = (1.0 + assign6410_e4973);
        let assign6410_e4975: f64 = (assign6410_e4969 * assign6410_e4974);
        (assign6410_e4975,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign6410_e4977;
        var_thesatac_p_rv = 0.0;

        let (assign6420_e4981,) = {
    if (var_guard36 != 0.0) {
        (p.p309,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6420_e4981;
        var_axaco_i_rv = 0.0;

        let assign6430_e4983: f64 = if param_given[369] { 1.0 } else { 0.0 };
        let assign6430_e4985: f64 = if assign6430_e4983 == 1.0 { 1.0 } else { 0.0 };
        var_guard48 = assign6430_e4985;
        var_guard48_rv = 0.0;

        let (assign6440_e4991,) = {
    if ((var_guard36 != 0.0) && (var_guard48 != 0.0)) {
        (p.p369,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign6440_e4991;
        var_axaco_i_rv = 0.0;

        let (assign6450_e4995,) = {
    if (var_guard36 != 0.0) {
        (p.p310,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6450_e4995;
        var_axacl_i_rv = 0.0;

        let assign6460_e4997: f64 = if param_given[370] { 1.0 } else { 0.0 };
        let assign6460_e4999: f64 = if assign6460_e4997 == 1.0 { 1.0 } else { 0.0 };
        var_guard49 = assign6460_e4999;
        var_guard49_rv = 0.0;

        let (assign6470_e5005,) = {
    if ((var_guard36 != 0.0) && (var_guard49 != 0.0)) {
        (p.p370,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign6470_e5005;
        var_axacl_i_rv = 0.0;

        let (assign6480_e5015,) = {
    if (var_guard36 != 0.0) {
        let assign6480_e5011: f64 = (var_axacl_i * var_ile);
        let assign6480_e5012: f64 = (1.0 + assign6480_e5011);
        let assign6480_e5013: f64 = (var_axaco_i / assign6480_e5012);
        (assign6480_e5013,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign6480_e5015;
        var_axac_p_rv = 0.0;

        let (assign6490_e5029,) = {
    if (var_guard36 != 0.0) {
        let assign6490_e5020: f64 = (var_ile).powf(p.p372);
        let assign6490_e5021: f64 = (p.p371 * assign6490_e5020);
        let assign6490_e5025: f64 = (p.p373 * var_iwe);
        let assign6490_e5026: f64 = (1.0 + assign6490_e5025);
        let assign6490_e5027: f64 = (assign6490_e5021 * assign6490_e5026);
        (assign6490_e5027,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign6490_e5029;
        var_alpac_p_rv = 0.0;

        let (assign6500_e5035,) = {
    if (var_guard36 != 0.0) {
        let assign6500_e5033: f64 = (var_ile).powf(p.p375);
        (assign6500_e5033,)
    } else {
        (var_tmpx,)
    }
};
        var_tmpx = assign6500_e5035;
        var_tmpx_rv = 0.0;

        let (assign6510_e5055,) = {
    if (var_guard36 != 0.0) {
        let assign6510_e5039: f64 = (p.p374 * var_tmpx);
        let assign6510_e5043: f64 = (p.p377 * var_iwe);
        let assign6510_e5044: f64 = (1.0 + assign6510_e5043);
        let assign6510_e5045: f64 = (assign6510_e5039 * assign6510_e5044);
        let assign6510_e5049: f64 = (p.p376 * var_ile);
        let assign6510_e5051: f64 = (assign6510_e5049 * var_tmpx);
        let assign6510_e5052: f64 = (1.0 + assign6510_e5051);
        let assign6510_e5053: f64 = (assign6510_e5045 / assign6510_e5052);
        (assign6510_e5053,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign6510_e5055;
        var_alp1ac_p_rv = 0.0;

        let (assign6520_e5059,) = {
    if (var_guard36 != 0.0) {
        (p.p378,)
    } else {
        (var_fcgovacc_p,)
    }
};
        var_fcgovacc_p = assign6520_e5059;
        var_fcgovacc_p_rv = 0.0;

        let (assign6530_e5063,) = {
    if (var_guard36 != 0.0) {
        (p.p379,)
    } else {
        (var_fcgovaccd_p,)
    }
};
        var_fcgovaccd_p = assign6530_e5063;
        var_fcgovaccd_p_rv = 0.0;

        let (assign6540_e5067,) = {
    if (var_guard36 != 0.0) {
        (p.p380,)
    } else {
        (var_cgovaccg_p,)
    }
};
        var_cgovaccg_p = assign6540_e5067;
        var_cgovaccg_p_rv = 0.0;

        let (assign6550_e5073,) = {
    if (var_guard36 != 0.0) {
        let assign6550_e5071: f64 = (p.p381 * var_iilcv);
        (assign6550_e5071,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign6550_e5073;
        var_cgbov_p_rv = 0.0;

        let (assign6560_e5079,) = {
    if (var_guard36 != 0.0) {
        let assign6560_e5077: f64 = (p.p382 * var_iiwecv);
        (assign6560_e5077,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign6560_e5079;
        var_cinr_p_rv = 0.0;

        let (assign6570_e5085,) = {
    if (var_guard36 != 0.0) {
        let assign6570_e5083: f64 = (p.p383 * var_iiwecv);
        (assign6570_e5083,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign6570_e5085;
        var_cinrd_p_rv = 0.0;

        let (assign6580_e5089,) = {
    if (var_guard36 != 0.0) {
        (p.p384,)
    } else {
        (var_dvfbinr_p,)
    }
};
        var_dvfbinr_p = assign6580_e5089;
        var_dvfbinr_p_rv = 0.0;

        let (assign6590_e5093,) = {
    if (var_guard36 != 0.0) {
        (p.p385,)
    } else {
        (var_fcinrdep_p,)
    }
};
        var_fcinrdep_p = assign6590_e5093;
        var_fcinrdep_p_rv = 0.0;

        let (assign6600_e5097,) = {
    if (var_guard36 != 0.0) {
        (p.p386,)
    } else {
        (var_fcinracc_p,)
    }
};
        var_fcinracc_p = assign6600_e5097;
        var_fcinracc_p_rv = 0.0;

        let (assign6610_e5101,) = {
    if (var_guard36 != 0.0) {
        (p.p387,)
    } else {
        (var_axinr_p,)
    }
};
        var_axinr_p = assign6610_e5101;
        var_axinr_p_rv = 0.0;

        let (assign6620_e5107,) = {
    if (var_guard36 != 0.0) {
        let assign6620_e5105: f64 = (p.p388 * var_iiwcv);
        (assign6620_e5105,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign6620_e5107;
        var_cfr_p_rv = 0.0;

        let (assign6630_e5113,) = {
    if (var_guard36 != 0.0) {
        let assign6630_e5111: f64 = (p.p389 * var_iiwcv);
        (assign6630_e5111,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign6630_e5113;
        var_cfrd_p_rv = 0.0;

        let (assign6640_e5123,) = {
    if (var_guard36 != 0.0) {
        let assign6640_e5118: f64 = (2.0 * p.p396);
        let assign6640_e5120: f64 = (assign6640_e5118 / var_le);
        let assign6640_e5121: f64 = (1.0 - assign6640_e5120);
        (assign6640_e5121,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign6640_e5123;
        var_temp0_rv = 0.0;

        let (assign6670_e5144,) = {
    if (var_guard36 != 0.0) {
        (p.p390,)
    } else {
        (var_fnt_p,)
    }
};
        var_fnt_p = assign6670_e5144;
        var_fnt_p_rv = 0.0;

        let (assign6730_e5194,) = {
    if (var_guard36 != 0.0) {
        let assign6730_e5188: f64 = (2.0 * p.p398);
        let assign6730_e5191: f64 = (p.p399 * var_we);
        let assign6730_e5192: f64 = (assign6730_e5188 + assign6730_e5191);
        (assign6730_e5192,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign6730_e5194;
        var_we_edge_rv = 0.0;

        let (assign6760_e5210,) = {
    if (var_guard36 != 0.0) {
        (p.p400,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign6760_e5210;
        var_vfbedge_p_rv = 0.0;

        let (assign6770_e5226,) = {
    if (var_guard36 != 0.0) {
        let assign6770_e5215: f64 = (p.p402 * var_ile);
        let assign6770_e5216: f64 = (p.p401 + assign6770_e5215);
        let assign6770_e5219: f64 = (p.p403 * var_iwe);
        let assign6770_e5220: f64 = (assign6770_e5216 + assign6770_e5219);
        let assign6770_e5223: f64 = (p.p404 * var_iae);
        let assign6770_e5224: f64 = (assign6770_e5220 + assign6770_e5223);
        (assign6770_e5224,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign6770_e5226;
        var_stvfbedge_p_rv = 0.0;

        let (assign6780_e5244,) = {
    if (var_guard36 != 0.0) {
        let assign6780_e5232: f64 = (var_ile).powf(p.p407);
        let assign6780_e5233: f64 = (p.p406 * assign6780_e5232);
        let assign6780_e5234: f64 = (p.p405 + assign6780_e5233);
        let assign6780_e5237: f64 = (p.p408 * var_iwe);
        let assign6780_e5238: f64 = (assign6780_e5234 + assign6780_e5237);
        let assign6780_e5241: f64 = (p.p409 * var_iae);
        let assign6780_e5242: f64 = (assign6780_e5238 + assign6780_e5241);
        (assign6780_e5242,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign6780_e5244;
        var_dphibedge_p_rv = 0.0;

        let (assign6790_e5268,) = {
    if (var_guard36 != 0.0) {
        let assign6790_e5251: f64 = (var_ile).powf(p.p412);
        let assign6790_e5252: f64 = (p.p411 * assign6790_e5251);
        let assign6790_e5253: f64 = (1.0 + assign6790_e5252);
        let assign6790_e5254: f64 = (p.p410 * assign6790_e5253);
        let assign6790_e5258: f64 = (p.p413 * var_iwe);
        let assign6790_e5259: f64 = (1.0 + assign6790_e5258);
        let assign6790_e5260: f64 = (assign6790_e5254 * assign6790_e5259);
        let assign6790_e5264: f64 = (p.p414 * var_iae);
        let assign6790_e5265: f64 = (1.0 + assign6790_e5264);
        let assign6790_e5266: f64 = (assign6790_e5260 * assign6790_e5265);
        (assign6790_e5266,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign6790_e5268;
        var_neffedge_p_rv = 0.0;

        let (assign6800_e5278,) = {
    if (var_guard36 != 0.0) {
        let assign6800_e5274: f64 = (var_ile).powf(p.p417);
        let assign6800_e5275: f64 = (p.p416 * assign6800_e5274);
        let assign6800_e5276: f64 = (p.p415 + assign6800_e5275);
        (assign6800_e5276,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign6800_e5278;
        var_ctedge_p_rv = 0.0;

        let (assign6810_e5296,) = {
    if (var_guard36 != 0.0) {
        let assign6810_e5283: f64 = (p.p418 * p.p419);
        let assign6810_e5285: f64 = (assign6810_e5283 / var_le);
        let assign6810_e5288: f64 = (-var_le);
        let assign6810_e5290: f64 = (assign6810_e5288 / p.p419);
        let assign6810_e5291: f64 = (assign6810_e5290).exp();
        let assign6810_e5292: f64 = (1.0 - assign6810_e5291);
        let assign6810_e5293: f64 = (assign6810_e5285 * assign6810_e5292);
        let assign6810_e5294: f64 = (1.0 + assign6810_e5293);
        (assign6810_e5294,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign6810_e5296;
        var_gpe_edge_rv = 0.0;

        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_axacl_i_slot = var_axacl_i;
        *var_axacl_i_rv_slot = var_axacl_i_rv;
        *var_axaco_i_slot = var_axaco_i;
        *var_axaco_i_rv_slot = var_axaco_i_rv;
        *var_axinr_p_slot = var_axinr_p;
        *var_axinr_p_rv_slot = var_axinr_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgovaccg_p_slot = var_cgovaccg_p;
        *var_cgovaccg_p_rv_slot = var_cgovaccg_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_dvfbinr_p_slot = var_dvfbinr_p;
        *var_dvfbinr_p_rv_slot = var_dvfbinr_p_rv;
        *var_fcgovacc_p_slot = var_fcgovacc_p;
        *var_fcgovacc_p_rv_slot = var_fcgovacc_p_rv;
        *var_fcgovaccd_p_slot = var_fcgovaccd_p;
        *var_fcgovaccd_p_rv_slot = var_fcgovaccd_p_rv;
        *var_fcinracc_p_slot = var_fcinracc_p;
        *var_fcinracc_p_rv_slot = var_fcinracc_p_rv;
        *var_fcinrdep_p_slot = var_fcinrdep_p;
        *var_fcinrdep_p_rv_slot = var_fcinrdep_p_rv;
        *var_fnt_p_slot = var_fnt_p;
        *var_fnt_p_rv_slot = var_fnt_p_rv;
        *var_gpe_edge_slot = var_gpe_edge;
        *var_gpe_edge_rv_slot = var_gpe_edge_rv;
        *var_guard43_slot = var_guard43;
        *var_guard43_rv_slot = var_guard43_rv;
        *var_guard44_slot = var_guard44;
        *var_guard44_rv_slot = var_guard44_rv;
        *var_guard45_slot = var_guard45;
        *var_guard45_rv_slot = var_guard45_rv;
        *var_guard46_slot = var_guard46;
        *var_guard46_rv_slot = var_guard46_rv;
        *var_guard47_slot = var_guard47;
        *var_guard47_rv_slot = var_guard47_rv;
        *var_guard48_slot = var_guard48;
        *var_guard48_rv_slot = var_guard48_rv;
        *var_guard49_slot = var_guard49;
        *var_guard49_rv_slot = var_guard49_rv;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_neffedge_p_rv_slot = var_neffedge_p_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_temp0_slot = var_temp0;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesatacl_i_rv_slot = var_thesatacl_i_rv;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclexp_i_rv_slot = var_thesataclexp_i_rv;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataclw_i_rv_slot = var_thesataclw_i_rv;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesataco_i_rv_slot = var_thesataco_i_rv;
        *var_thesatacw_i_slot = var_thesatacw_i;
        *var_thesatacw_i_rv_slot = var_thesatacw_i_rv;
        *var_tmpx_slot = var_tmpx;
        *var_tmpx_rv_slot = var_tmpx_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_we_edge_slot = var_we_edge;
        *var_we_edge_rv_slot = var_we_edge_rv;
    }
}
